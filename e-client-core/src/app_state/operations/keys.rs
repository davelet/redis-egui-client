use super::super::AppState;
use crate::redis_client::ValueData;
use e_client_basics::constants::{
    LOAD_MORE_BATCH_SIZE, MAX_INITIAL_KEYS, MAX_LOADED_KEYS, SCAN_COUNT, SCAN_SLEEP_INTERVAL_MS,
    SORT_INTERVAL_KEYS, UI_UPDATE_INTERVAL_BATCHES, WILD_KEY_FILTER,
};

/// Spawn load keys operation
pub fn spawn_load_keys(state: &AppState) {
    let state = state.clone();
    tokio::spawn(async move {
        *state.loading.write().await = true;
        let pattern = state.key_filter.read().await.clone();

        // Get total key count with current pattern (this is just initial estimate)
        // It may change if keys expire during scanning
        if pattern == WILD_KEY_FILTER.to_string() {
            if let Ok(total) = state.redis_client.get_db_size().await {
                *state.total_keys.write().await = total;
            }
        }

        // Reset scan state
        *state.scan_cursor.write().await = 0;
        *state.scan_has_more.write().await = true;
        *state.loaded_keys_count.write().await = 0;
        *state.keys.write().await = vec![];

        // Use HashSet for deduplication
        let mut all_keys_set = std::collections::HashSet::new();
        let mut current_cursor = 0;
        let mut loaded_count = 0;
        let mut last_sort_count = 0;

        loop {
            match state
                .redis_client
                .scan_keys(current_cursor, &pattern, SCAN_COUNT)
                .await
            {
                Ok((new_cursor, keys)) => {
                    // Deduplicate keys
                    for key in keys {
                        if all_keys_set.insert(key) {
                            loaded_count += 1;
                        }
                    }

                    current_cursor = new_cursor;

                    // Update UI every SCAN batch with periodic sorting for display
                    // Sort and display every SORT_INTERVAL_KEYS keys
                    if loaded_count - last_sort_count >= SORT_INTERVAL_KEYS || current_cursor == 0 {
                        let mut all_keys: Vec<String> = all_keys_set.iter().cloned().collect();
                        all_keys.sort();
                        *state.keys.write().await = all_keys;
                        last_sort_count = loaded_count;
                    }

                    *state.loaded_keys_count.write().await = loaded_count;
                    *state.scan_cursor.write().await = current_cursor;
                    *state.scan_has_more.write().await = current_cursor != 0;

                    // Update progress text
                    let total = *state.total_keys.read().await;
                    let progress_text = if current_cursor != 0 {
                        format!("Loading keys... {}/{}", loaded_count, total)
                    } else {
                        format!("Loaded {} keys", loaded_count)
                    };
                    *state.loading_progress_text.write().await = progress_text;

                    // Small delay to allow UI to refresh and prevent tight loop
                    tokio::time::sleep(tokio::time::Duration::from_millis(SCAN_SLEEP_INTERVAL_MS))
                        .await;

                    // Stop if we reached max initial keys
                    if loaded_count >= MAX_INITIAL_KEYS {
                        break;
                    }

                    // Stop if scan is complete
                    if current_cursor == 0 {
                        break;
                    }
                }
                Err(_) => {
                    *state.scan_cursor.write().await = 0;
                    *state.scan_has_more.write().await = false;
                    break;
                }
            }
        }

        // Final sort at the end to ensure everything is sorted
        let mut all_keys: Vec<String> = all_keys_set.into_iter().collect();
        all_keys.sort();
        *state.keys.write().await = all_keys.clone();

        // Update final state - ensure scan_has_more and loading are set correctly
        // If we exited due to MAX_INITIAL_KEYS, current_cursor should still be valid
        *state.scan_has_more.write().await = current_cursor != 0;
        *state.scan_cursor.write().await = current_cursor;
        *state.loaded_keys_count.write().await = all_keys.len();

        // When scan is complete (current_cursor == 0), update total_keys to actual count
        // This handles the case where keys expired during scanning
        if current_cursor == 0 {
            *state.total_keys.write().await = all_keys.len();
        }

        // Update final progress text
        if current_cursor != 0 {
            let total = *state.total_keys.read().await;
            *state.loading_progress_text.write().await = format!(
                "Loaded {} of ~{} keys. Click 'Load More' to load additional keys",
                all_keys.len(),
                total
            );
        } else {
            *state.loading_progress_text.write().await =
                format!("All {} keys loaded", all_keys.len());
        }

        *state.loading.write().await = false;
    });
}

/// Spawn load more keys operation
pub fn spawn_load_more_keys(state: &AppState, load_all: bool) {
    let state = state.clone();
    tokio::spawn(async move {
        *state.loading.write().await = true;

        // Get existing keys to avoid duplicates
        let mut existing_keys: std::collections::HashSet<String> =
            state.keys.read().await.iter().cloned().collect();
        let keys_at_start = existing_keys.len();

        let cursor = *state.scan_cursor.read().await;
        if cursor == 0 {
            *state.loading.write().await = false;
            return;
        }

        let pattern = state.key_filter.read().await.clone();
        let mut current_cursor = cursor;
        let mut batch_count = 0;
        let mut last_update_batch = 0;

        loop {
            match state
                .redis_client
                .scan_keys(current_cursor, &pattern, SCAN_COUNT)
                .await
            {
                Ok((new_cursor, keys)) => {
                    batch_count += 1;

                    for key in keys {
                        existing_keys.insert(key);
                    }

                    current_cursor = new_cursor;

                    // Update UI every UI_UPDATE_INTERVAL_BATCHES SCAN batches for display
                    if batch_count - last_update_batch >= UI_UPDATE_INTERVAL_BATCHES
                        || current_cursor == 0
                    {
                        let mut all_keys: Vec<String> = existing_keys.iter().cloned().collect();
                        all_keys.sort();
                        *state.keys.write().await = all_keys;
                        last_update_batch = batch_count;
                    }

                    // Update progress text
                    let total = existing_keys.len();
                    let progress_text = if current_cursor != 0 {
                        format!(
                            "Loading keys... {}/{}",
                            total,
                            *state.total_keys.read().await
                        )
                    } else {
                        format!("Loaded {} keys", total)
                    };
                    *state.loading_progress_text.write().await = progress_text;
                    *state.loaded_keys_count.write().await = total;

                    // Small delay to allow UI to refresh and prevent tight loop
                    tokio::time::sleep(tokio::time::Duration::from_millis(SCAN_SLEEP_INTERVAL_MS))
                        .await;

                    // Check if we should stop loading more
                    let total_new_keys = existing_keys.len() - keys_at_start;
                    if !load_all && (current_cursor == 0 || total_new_keys >= LOAD_MORE_BATCH_SIZE)
                    {
                        break;
                    }

                    // Stop if we reached MAX_LOADED_KEYS
                    if existing_keys.len() >= MAX_LOADED_KEYS {
                        break;
                    }

                    // Stop if scan is complete when loading all
                    if load_all && current_cursor == 0 {
                        break;
                    }
                }
                Err(_e) => {
                    break;
                }
            }
        }

        *state.scan_cursor.write().await = current_cursor;
        *state.scan_has_more.write().await = current_cursor != 0;

        // Final sort and update
        let mut all_keys: Vec<String> = existing_keys.into_iter().collect();
        all_keys.sort();

        // Update total_keys based on scan completion status
        if current_cursor == 0 {
            // Scan is complete - use loaded count as it's accurate total
            // (keys may have expired during scanning)
            *state.total_keys.write().await = all_keys.len();
        } else {
            // Scan is not complete - use dbsize to show real-time total
            // This gives user context of how many keys are still in database
            if let Ok(total) = state.redis_client.get_db_size().await {
                *state.total_keys.write().await = total;
            }
        }

        // Update progress text based on scan state
        if current_cursor == 0 {
            *state.loading_progress_text.write().await =
                format!("All {} keys loaded", all_keys.len());
        }

        *state.keys.write().await = all_keys.clone();
        *state.loaded_keys_count.write().await = all_keys.len();

        // Update progress text if still loading more
        if current_cursor != 0 && state.loading_progress_text.read().await.is_empty() {
            *state.loading_progress_text.write().await = format!(
                "Loaded {} keys. Click 'Load More' to continue",
                all_keys.len()
            );
        }

        *state.loading.write().await = false;
    });
}

/// Spawn load value operation
///
/// # Arguments
/// * `state` - The app state
/// * `key` - The key to load
/// * `reload_hash` - If true, reload hash fields and clear loaded values.
///                   If false, preserve existing loaded hash values.
pub fn spawn_load_value(state: &AppState, key: String, reload_hash: bool) {
    let state = state.clone();
    tokio::spawn(async move {
        *state.loading.write().await = true;
        // Cancel editing when reloading
        state.edit_state.write().await.cancel_edit();
        // Clear hash field filter when switching keys
        *state.hash_field_filter.write().await = String::new();

        match state.redis_client.get_value(&key).await {
            Ok(value) => {
                *state.selected_key.write().await = Some(key.clone());

                // For Hash type, preserve fields and loaded_values if not reloading
                // When reload_hash is false, we only update the len and preserve everything else
                // When reload_hash is true, we replace the entire value and reload fields
                let is_hash = matches!(&value, ValueData::Hash { .. });
                if !reload_hash && is_hash {
                    // Get the new len from the fetched value
                    let new_len = if let ValueData::Hash { len, .. } = &value {
                        *len
                    } else {
                        0
                    };

                    // Update only the len in the existing value, preserve fields and loaded_values
                    let mut existing_value = state.key_value.write().await;
                    if let Some(ValueData::Hash { len, .. }) = existing_value.as_mut() {
                        *len = new_len;
                    } else {
                        // If no existing value, use the new one (but fields will be empty)
                        *existing_value = Some(value);
                    }
                } else {
                    *state.key_value.write().await = Some(value);
                }

                // Get TTL for key
                match state.redis_client.get_ttl(&key).await {
                    Ok(ttl) => {
                        *state.key_ttl.write().await = ttl;
                    }
                    Err(_) => {
                        *state.key_ttl.write().await = -2;
                    }
                }

                // For Hash type, load field names
                // If reload_hash is true, clear all loaded values and reload fields
                // If reload_hash is false, fields are already preserved above, just ensure values are fresh
                if is_hash && reload_hash {
                    state.spawn_load_hash_fields(key);
                }
            }
            Err(_) => {
                // Ignore error - command_output was removed
            }
        }

        *state.loading.write().await = false;
    });
}

/// Spawn refresh TTL only (lightweight, for auto-refresh timer)
pub fn spawn_refresh_ttl_only(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        match state.redis_client.get_ttl(&key).await {
            Ok(ttl) => {
                *state.key_ttl.write().await = ttl;
            }
            Err(_) => {
                *state.key_ttl.write().await = -2;
            }
        }
        // Update last refresh time
        *state.ttl_last_refresh.write().await = Some(std::time::Instant::now());
    });
}

/// Spawn create new key operation
pub fn spawn_create_new_key(
    state: &AppState,
    key: String,
    key_type: String,
    value: String,
    ttl: i64,
) {
    let state = state.clone();
    tokio::spawn(async move {
        let result = match key_type.as_str() {
            "string" => state.redis_client.set_string(&key, &value).await,
            "list" => {
                let mut r = Ok(());
                for item in value.lines() {
                    let item = item.trim();
                    if !item.is_empty() {
                        if let Err(e) = state.redis_client.rpush(&key, item).await {
                            r = Err(e);
                            break;
                        }
                    }
                }
                r
            }
            "hash" => {
                let mut r = Ok(());
                for line in value.lines() {
                    let line = line.trim();
                    if let Some((field, val)) = line.split_once(':') {
                        let field = field.trim();
                        let val = val.trim();
                        if !field.is_empty() {
                            if let Err(e) = state.redis_client.hset(&key, field, val).await {
                                r = Err(e);
                                break;
                            }
                        }
                    }
                }
                r
            }
            "set" => {
                let mut r = Ok(());
                for item in value.lines() {
                    let item = item.trim();
                    if !item.is_empty() {
                        if let Err(e) = state.redis_client.sadd(&key, item).await {
                            r = Err(e);
                            break;
                        }
                    }
                }
                r
            }
            "zset" => {
                let mut r = Ok(());
                for line in value.lines() {
                    let line = line.trim();
                    if let Some((score_str, member)) = line.split_once(':') {
                        let score: f64 = score_str.trim().parse().unwrap_or(0.0);
                        let member = member.trim();
                        if !member.is_empty() {
                            if let Err(e) = state.redis_client.zadd(&key, score, member).await {
                                r = Err(e);
                                break;
                            }
                        }
                    }
                }
                r
            }
            _ => Ok(()),
        };

        match result {
            Ok(_) => {
                // Set TTL if specified
                if ttl >= 0 {
                    let _ = state.redis_client.set_ttl(&key, ttl).await;
                }
                // Select new key and load its value
                *state.selected_key.write().await = Some(key.clone());
                state.edit_state.write().await.cancel_edit();
                state.spawn_load_value(key, true);
                state.spawn_load_keys();
            }
            Err(e) => {
                *state.error_message.write().await = format!("Create key failed: {}", e);
            }
        }
    });
}

/// Spawn delete key operation
pub fn spawn_delete_key(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        match state.redis_client.del_key(&key).await {
            Ok(_) => {
                *state.selected_key.write().await = None;
                *state.key_value.write().await = None;
                *state.key_ttl.write().await = -2;
                state.edit_state.write().await.cancel_edit();
                *state.hash_field_filter.write().await = String::new();
                // Refresh key list
                state.spawn_load_keys();
            }
            Err(e) => {
                state.edit_state.write().await.save_message = format!("Delete failed: {}", e);
            }
        }
    });
}
