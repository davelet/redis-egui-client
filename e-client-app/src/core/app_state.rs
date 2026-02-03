use crate::core::redis_client::{RedisClient, ValueData};
use e_client_basics::constants::{LOAD_MORE_BATCH_SIZE, MAX_INITIAL_KEYS, SCAN_COUNT};
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: RedisClient,
    pub connection_param: Arc<RwLock<Option<RedisConnectionConfig>>>,
    pub connected: Arc<RwLock<bool>>,
    pub current_db: Arc<RwLock<u32>>,
    pub databases: Arc<RwLock<Vec<u32>>>,
    pub keys: Arc<RwLock<Vec<String>>>,
    pub selected_key: Arc<RwLock<Option<String>>>,
    pub key_value: Arc<RwLock<Option<ValueData>>>,
    pub key_filter: Arc<RwLock<String>>,
    pub loading: Arc<RwLock<bool>>,
    pub error_message: Arc<RwLock<String>>,
    pub language: Arc<RwLock<Language>>,
    pub scan_cursor: Arc<RwLock<u64>>,
    pub scan_has_more: Arc<RwLock<bool>>,
    pub total_keys: Arc<RwLock<usize>>,
    pub loaded_keys_count: Arc<RwLock<usize>>,
    pub loading_progress_text: Arc<RwLock<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            redis_client: RedisClient::new(),
            connection_param: Arc::new(RwLock::new(None)),
            connected: Arc::new(RwLock::new(false)),
            current_db: Arc::new(RwLock::new(0)),
            databases: Arc::new(RwLock::new(vec![])),
            keys: Arc::new(RwLock::new(vec![])),
            selected_key: Arc::new(RwLock::new(None)),
            key_value: Arc::new(RwLock::new(None)),
            key_filter: Arc::new(RwLock::new("".to_string())),
            loading: Arc::new(RwLock::new(false)),
            error_message: Arc::new(RwLock::new(String::new())),
            language: Arc::new(RwLock::new(Language::English)),
            scan_cursor: Arc::new(RwLock::new(0)),
            scan_has_more: Arc::new(RwLock::new(true)),
            total_keys: Arc::new(RwLock::new(0)),
            loaded_keys_count: Arc::new(RwLock::new(0)),
            loading_progress_text: Arc::new(RwLock::new(String::new())),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn_connect(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let param = state.connection_param.read().await.clone();

            if let Some(connection_config) = param {
                match state.redis_client.connect(connection_config).await {
                    Ok(_) => {
                        *state.connected.write().await = true;
                        *state.loading.write().await = false; // Clear loading after connection succeeds
                        *state.error_message.write().await = String::new();
                        *state.scan_cursor.write().await = 0;
                        *state.scan_has_more.write().await = true;
                        *state.loaded_keys_count.write().await = 0;
                        *state.loading_progress_text.write().await = String::new();

                        // Get total key count in the database
                        if let Ok(total) = state.redis_client.get_db_size().await {
                            *state.total_keys.write().await = total;
                        }

                        // Set initial key filter to "*"
                        let current_filter = state.key_filter.read().await.clone();
                        if current_filter.is_empty() {
                            *state.key_filter.write().await = "*".to_string();
                        }

                        if let Ok(dbs) = state.redis_client.get_databases().await {
                            *state.databases.write().await = dbs;
                        }

                        state.spawn_load_keys();
                    }
                    Err(e) => {
                        *state.connected.write().await = false;
                        *state.loading.write().await = false;
                        *state.error_message.write().await = format!("Connection failed: {}", e);
                    }
                }
            } else {
                *state.connected.write().await = false;
                *state.loading.write().await = false;
            }
        });
    }

    pub fn spawn_disconnect(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            state.redis_client.disconnect().await;
            *state.connected.write().await = false;
            *state.keys.write().await = vec![];
            *state.selected_key.write().await = None;
            *state.key_value.write().await = None;
            *state.error_message.write().await = String::new();
            *state.scan_cursor.write().await = 0;
            *state.scan_has_more.write().await = true;
            *state.loaded_keys_count.write().await = 0;
            *state.loading_progress_text.write().await = String::new();
        });
    }

    pub fn spawn_select_db(&self, db: u32) {
        let state = self.clone();
        tokio::spawn(async move {
            if state.redis_client.select_db(db).await.is_ok() {
                *state.current_db.write().await = db;
                *state.scan_cursor.write().await = 0;
                *state.scan_has_more.write().await = true;
                *state.loaded_keys_count.write().await = 0;
                *state.loading_progress_text.write().await = String::new();

                // Get total key count in the new database
                if let Ok(total) = state.redis_client.get_db_size().await {
                    *state.total_keys.write().await = total;
                }

                state.spawn_load_keys();
            }
        });
    }

    pub fn spawn_load_keys(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let pattern = state.key_filter.read().await.clone();

            // Get total key count with current pattern (this is just initial estimate)
            // It may change if keys expire during scanning
            if pattern == "*" {
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
                        // Sort and display every 200 keys (roughly 2 SCAN batches at SCAN_COUNT=100)
                        if loaded_count - last_sort_count >= 200 || current_cursor == 0 {
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
                        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

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

    pub fn spawn_load_more_keys(&self, load_all: bool) {
        let state = self.clone();
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

                        // Update UI every 2 SCAN batches for display
                        if batch_count - last_update_batch >= 2 || current_cursor == 0 {
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
                        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

                        // Check if we should stop loading more
                        let total_new_keys = existing_keys.len() - keys_at_start;
                        if !load_all
                            && (current_cursor == 0 || total_new_keys >= LOAD_MORE_BATCH_SIZE)
                        {
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
                // Scan is complete - use the loaded count as it's the accurate total
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

    pub fn spawn_load_value(&self, key: String) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;

            match state.redis_client.get_value(&key).await {
                Ok(value) => {
                    *state.selected_key.write().await = Some(key);
                    *state.key_value.write().await = Some(value);
                }
                Err(_) => {
                    // Ignore error - command_output was removed
                }
            }

            *state.loading.write().await = false;
        });
    }

    pub fn spawn_load_list_range(&self, key: String, start: isize, stop: isize) {
        let state = self.clone();
        tokio::spawn(async move {
            match state.redis_client.get_list_range(&key, start, stop).await {
                Ok(items) => {
                    let mut value = state.key_value.write().await;
                    if let Some(ValueData::List {
                        items: existing, ..
                    }) = value.as_mut()
                    {
                        *existing = items;
                    }
                }
                Err(_) => {}
            }
        });
    }

    pub fn spawn_load_hash_fields(&self, key: String) {
        let state = self.clone();
        tokio::spawn(async move {
            match state.redis_client.get_hash_fields(&key, 0, 100).await {
                Ok((_, fields)) => {
                    let mut value = state.key_value.write().await;
                    if let Some(ValueData::Hash {
                        fields: existing, ..
                    }) = value.as_mut()
                    {
                        *existing = fields;
                    }
                }
                Err(_) => {}
            }
        });
    }

    pub fn spawn_load_set_members(&self, key: String) {
        let state = self.clone();
        tokio::spawn(async move {
            match state.redis_client.get_set_members(&key, 0, 100).await {
                Ok((_, members)) => {
                    let mut value = state.key_value.write().await;
                    if let Some(ValueData::Set {
                        items: existing, ..
                    }) = value.as_mut()
                    {
                        *existing = members;
                    }
                }
                Err(_) => {}
            }
        });
    }

    pub fn spawn_load_zset_range(&self, key: String, start: isize, stop: isize) {
        let state = self.clone();
        tokio::spawn(async move {
            match state.redis_client.get_zset_range(&key, start, stop).await {
                Ok(items) => {
                    let mut value = state.key_value.write().await;
                    if let Some(ValueData::ZSet {
                        items: existing, ..
                    }) = value.as_mut()
                    {
                        *existing = items;
                    }
                }
                Err(_) => {}
            }
        });
    }
}
