use super::super::AppState;
use super::super::{EditedValue, compact_json_if_single_line};
use crate::redis_client::ValueData;
use e_client_bilingual::translations::{keys, tr};

/// Spawn save element operation
pub fn spawn_save_element(
    state: &AppState,
    key: String,
    key_type: String,
    field: String,
    value: String,
    original_value: String,
) {
    let state = state.clone();
    tokio::spawn(async move {
        // Compact JSON if original was single-line
        let value_to_save = compact_json_if_single_line(&value, &original_value);

        let result = match key_type.as_str() {
            "hash" => state.redis_client.hset(&key, &field, &value_to_save).await,
            "list" => {
                if let Ok(index) = field.parse::<i64>() {
                    state.redis_client.lset(&key, index, &value_to_save).await
                } else {
                    Err(redis::RedisError::from((
                        redis::ErrorKind::InvalidClientConfig,
                        "Invalid list index",
                        format!("Field '{}' is not a valid index", field),
                    )))
                }
            }
            "set" => {
                // For set, we need to remove old member and add new one
                let _ = state.redis_client.srem(&key, &field).await;
                state.redis_client.sadd(&key, &value_to_save).await
            }
            "zset" => {
                // For zset, remove old member and add with score
                let _ = state.redis_client.zrem(&key, &field).await;
                let score: f64 = value_to_save.parse().unwrap_or(0.0);
                state.redis_client.zadd(&key, score, &field).await
            }
            _ => Ok(()),
        };

        match result {
            Ok(_) => {
                // Update the saved field's value in loaded_values for Hash type
                if key_type == "hash" {
                    let mut key_value = state.key_value.write().await;
                    if let Some(ValueData::Hash {
                        fields,
                        loaded_values,
                        ..
                    }) = key_value.as_mut()
                    {
                        loaded_values.insert(field.clone(), value_to_save);
                        // Ensure field is in fields list (for newly added fields)
                        if !fields.contains(&field) {
                            fields.push(field);
                        }
                    }
                }
                // Refresh TTL only, preserve all loaded values
                if let Ok(ttl) = state.redis_client.get_ttl(&key).await {
                    *state.key_ttl.write().await = ttl;
                }
            }
            Err(e) => {
                *state.error_message.write().await = format!("Save element failed: {}", e);
            }
        }
    });
}

/// Spawn update TTL operation
pub fn spawn_update_ttl(state: &AppState, key: String, ttl: i64) {
    let state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = state.redis_client.set_ttl(&key, ttl).await {
            state.edit_state.write().await.save_message = format!("TTL update failed: {}", e);
        } else {
            state.edit_state.write().await.save_message = "TTL updated successfully".to_string();
            // Refresh TTL display without clearing hash fields
            state.spawn_load_value(
                key,
                crate::app_state::operations::keys::HashLoadMode::PreserveFields,
            );
        }
    });
}

/// Spawn save edits operation
pub fn spawn_save_edits(state: &AppState, original_key: String) {
    let state = state.clone();
    let edit = state.edit_state.blocking_read().clone();

    // Set saving state to true
    state.edit_state.blocking_write().saving = true;

    tokio::spawn(async move {
        let new_key = edit.edited_key.trim().to_string();
        let mut current_key = original_key.clone();

        // Check if original key still exists (may have expired during editing)
        let key_exists = state
            .redis_client
            .key_exists(&original_key)
            .await
            .unwrap_or(false);
        if !key_exists && new_key == original_key {
            let lang = *state.language.read().await;
            state.edit_state.write().await.save_message = tr(keys::KEY_EXPIRED, lang).to_string();
            state.edit_state.write().await.saving = false;
            return;
        }

        // 1. Rename key if changed
        if new_key != original_key && !new_key.is_empty() {
            match state
                .redis_client
                .rename_key_nx(&original_key, &new_key)
                .await
            {
                Ok(true) => {
                    current_key = new_key.clone();
                }
                Ok(false) => {
                    state.edit_state.write().await.save_message =
                        "Rename failed: target key already exists".to_string();
                    return;
                }
                Err(e) => {
                    state.edit_state.write().await.save_message = format!("Rename failed: {}", e);
                    return;
                }
            }
        }

        // 2. Save value by type (must be done before setting TTL, since we delete and recreate the key)
        let save_result = match &edit.edited_value {
            EditedValue::String(s) => {
                // Use JsonValue.to_save() to handle JSON compression
                state
                    .redis_client
                    .set_string(&current_key, &s.to_save())
                    .await
            }
            EditedValue::Hash(fields) => {
                // Delete old key and re-create with new fields
                let _ = state.redis_client.del_key(&current_key).await;
                let mut result = Ok(());
                for (field, value) in fields {
                    if !field.is_empty() {
                        // Use JsonValue.to_save() to handle JSON compression
                        if let Err(e) = state
                            .redis_client
                            .hset(&current_key, field, &value.to_save())
                            .await
                        {
                            result = Err(e);
                            break;
                        }
                    }
                }
                result
            }
            EditedValue::List(items) => {
                let _ = state.redis_client.del_key(&current_key).await;
                let mut result = Ok(());
                for item in items {
                    // Use JsonValue.to_save() to handle JSON compression
                    if let Err(e) = state
                        .redis_client
                        .rpush(&current_key, &item.to_save())
                        .await
                    {
                        result = Err(e);
                        break;
                    }
                }
                result
            }
            EditedValue::Set(items) => {
                let _ = state.redis_client.del_key(&current_key).await;
                let mut result = Ok(());
                for item in items {
                    if !item.value.is_empty() {
                        // Use JsonValue.to_save() to handle JSON compression
                        if let Err(e) = state.redis_client.sadd(&current_key, &item.to_save()).await
                        {
                            result = Err(e);
                            break;
                        }
                    }
                }
                result
            }
            EditedValue::ZSet(items) => {
                let _ = state.redis_client.del_key(&current_key).await;
                let mut result = Ok(());
                for (member, score_str) in items {
                    if !member.value.is_empty() {
                        let score: f64 = score_str.parse().unwrap_or(0.0);
                        // Use JsonValue.to_save() to handle JSON compression
                        if let Err(e) = state
                            .redis_client
                            .zadd(&current_key, score, &member.to_save())
                            .await
                        {
                            result = Err(e);
                            break;
                        }
                    }
                }
                result
            }
            EditedValue::None => Ok(()),
        };

        // 3. Update TTL after saving value (important: Hash/List/Set/ZSet delete and recreate the key)
        if save_result.is_ok() {
            // Get current actual TTL from Redis (may have decreased during editing)
            let current_ttl = state.redis_client.get_ttl(&current_key).await.unwrap_or(-1);

            // Parse user's edited TTL
            let edited_ttl_str = edit.edited_ttl.trim();

            // Determine which TTL to use
            let ttl_to_set = if edited_ttl_str.is_empty() || edited_ttl_str == "-1" {
                // User wants no expiration (or left empty)
                -1i64
            } else if let Ok(edited_ttl) = edited_ttl_str.parse::<i64>() {
                // Compare with original TTL at edit start
                // If edited_ttl is close to original_ttl, user probably didn't change it
                let ttl_diff_from_original = (edited_ttl - edit.original_ttl).abs();

                // If current TTL is positive and user didn't modify TTL (within 5s tolerance),
                // use actual remaining TTL to preserve relative expiration time
                if current_ttl > 0 && ttl_diff_from_original <= 5 {
                    current_ttl // Use actual remaining time (preserves relative expiration)
                } else {
                    edited_ttl // User explicitly changed TTL, use their value
                }
            } else {
                current_ttl // Fallback to current TTL if parse fails
            };

            if let Err(e) = state.redis_client.set_ttl(&current_key, ttl_to_set).await {
                state.edit_state.write().await.save_message = format!("TTL update failed: {}", e);
                // Continue to reload even if TTL update fails
            }
        }

        match save_result {
            Ok(_) => {
                state.edit_state.write().await.cancel_edit();
                // For Hash type, preserve all edited values in loaded_values to avoid showing "Load" buttons
                if let EditedValue::Hash(fields) = &edit.edited_value {
                    let mut key_value = state.key_value.write().await;
                    if let Some(ValueData::Hash {
                        fields: existing_fields,
                        loaded_values,
                        ..
                    }) = key_value.as_mut()
                    {
                        // Update fields list
                        existing_fields.clear();
                        for (field, _) in fields {
                            if !field.is_empty() {
                                existing_fields.push(field.clone());
                            }
                        }
                        // Update loaded values
                        loaded_values.clear();
                        for (field, value) in fields {
                            if !field.is_empty() {
                                loaded_values.insert(field.clone(), value.to_save());
                            }
                        }
                    }
                }
                // Refresh TTL only, preserve hash values we just set
                if let Ok(ttl) = state.redis_client.get_ttl(&current_key).await {
                    *state.key_ttl.write().await = ttl;
                }
                // Reload keys list
                *state.selected_key.write().await = Some(current_key.clone());
                state.spawn_load_keys();
            }
            Err(e) => {
                let mut edit_state = state.edit_state.write().await;
                edit_state.save_message = format!("Save failed: {}", e);
                edit_state.saving = false;
            }
        }
    });
}
