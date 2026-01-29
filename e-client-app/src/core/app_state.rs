use crate::core::redis_client::{RedisClient, ValueData};
use e_client_basics::constants::{LOAD_MORE_BATCH_SIZE, MAX_INITIAL_KEYS, SCAN_COUNT};
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::{tr, tr_fmt};
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
    pub command_input: Arc<RwLock<String>>,
    pub command_output: Arc<RwLock<String>>,
    pub key_filter: Arc<RwLock<String>>,
    pub loading: Arc<RwLock<bool>>,
    pub language: Arc<RwLock<Language>>,
    pub scan_cursor: Arc<RwLock<u64>>,
    pub scan_has_more: Arc<RwLock<bool>>,
    pub total_keys: Arc<RwLock<usize>>,
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
            command_input: Arc::new(RwLock::new(String::new())),
            command_output: Arc::new(RwLock::new(String::new())),
            key_filter: Arc::new(RwLock::new("".to_string())),
            loading: Arc::new(RwLock::new(false)),
            language: Arc::new(RwLock::new(Language::English)),
            scan_cursor: Arc::new(RwLock::new(0)),
            scan_has_more: Arc::new(RwLock::new(true)),
            total_keys: Arc::new(RwLock::new(0)),
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
            let lang = *state.language.read().await;

            if let Some(connection_config) = param {
                match state.redis_client.connect(connection_config).await {
                    Ok(_) => {
                        *state.connected.write().await = true;
                        *state.scan_cursor.write().await = 0;
                        *state.scan_has_more.write().await = true;

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
                        *state.command_output.write().await =
                            tr_fmt(keys::CONNECTION_FAILED, lang, &[&e.to_string()]);
                        *state.connected.write().await = false;
                    }
                }
            } else {
                *state.command_output.write().await =
                    "No connection configuration found".to_string();
                *state.connected.write().await = false;
            }
            *state.loading.write().await = false;
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
            *state.scan_cursor.write().await = 0;
            *state.scan_has_more.write().await = true;
        });
    }

    pub fn spawn_select_db(&self, db: u32) {
        let state = self.clone();
        tokio::spawn(async move {
            if state.redis_client.select_db(db).await.is_ok() {
                *state.current_db.write().await = db;
                *state.scan_cursor.write().await = 0;
                *state.scan_has_more.write().await = true;
                
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

            // Get total key count with current pattern
            if pattern == "*" {
                if let Ok(total) = state.redis_client.get_db_size().await {
                    *state.total_keys.write().await = total;
                }
            }

            // Reset scan state
            *state.scan_cursor.write().await = 0;
            *state.scan_has_more.write().await = true;

            // Use HashSet for deduplication
            let mut all_keys_set = std::collections::HashSet::new();
            let cursor = *state.scan_cursor.read().await;
            let mut current_count = 0;
            let mut current_cursor = cursor;

            loop {
                match state.redis_client.scan_keys(current_cursor, &pattern, SCAN_COUNT).await {
                    Ok((new_cursor, keys)) => {
                        // Deduplicate keys
                        for key in keys {
                            if all_keys_set.insert(key) {
                                current_count += 1;
                            }
                        }

                        current_cursor = new_cursor;

                        // Stop if we reached max initial keys
                        if current_count >= MAX_INITIAL_KEYS {
                            *state.scan_cursor.write().await = current_cursor;
                            *state.scan_has_more.write().await = current_cursor != 0;
                            break;
                        }

                        // Stop if scan is complete
                        if current_cursor == 0 {
                            *state.scan_cursor.write().await = current_cursor;
                            *state.scan_has_more.write().await = false;
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

            // Convert to sorted vector
            let mut all_keys: Vec<String> = all_keys_set.into_iter().collect();
            all_keys.sort();
            *state.keys.write().await = all_keys;
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

            let cursor = *state.scan_cursor.read().await;
            if cursor == 0 {
                *state.loading.write().await = false;
                return;
            }

            let pattern = state.key_filter.read().await.clone();
            let mut current_cursor = cursor;
            let mut loaded_count = 0;

            loop {
                match state.redis_client.scan_keys(current_cursor, &pattern, SCAN_COUNT).await {
                    Ok((new_cursor, keys)) => {
                        for key in keys {
                            if existing_keys.insert(key) {
                                loaded_count += 1;
                            }
                        }

                        current_cursor = new_cursor;

                        // Stop if we loaded the batch size requested or scan is complete
                        if !load_all && (current_cursor == 0 || loaded_count >= LOAD_MORE_BATCH_SIZE) {
                            break;
                        }

                        // Stop if scan is complete when loading all
                        if load_all && current_cursor == 0 {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            *state.scan_cursor.write().await = current_cursor;
            *state.scan_has_more.write().await = current_cursor != 0;

            // Convert to sorted vector
            let mut all_keys: Vec<String> = existing_keys.into_iter().collect();
            all_keys.sort();
            
            // Update total_keys to match actual loaded keys when scan is complete
            if current_cursor == 0 {
                *state.total_keys.write().await = all_keys.len();
            }
            
            *state.keys.write().await = all_keys;
            *state.loading.write().await = false;
        });
    }

    pub fn spawn_load_value(&self, key: String) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let lang = *state.language.read().await;

            match state.redis_client.get_value(&key).await {
                Ok(value) => {
                    *state.selected_key.write().await = Some(key);
                    *state.key_value.write().await = Some(value);
                }
                Err(e) => {
                    *state.command_output.write().await =
                        tr_fmt(keys::GET_VALUE_FAILED, lang, &[&e.to_string()]);
                }
            }

            *state.loading.write().await = false;
        });
    }

    pub fn spawn_execute_command(&self, cmd: String) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let lang = *state.language.read().await;

            if cmd.trim().is_empty() {
                *state.command_output.write().await = tr(keys::EMPTY_COMMAND, lang).to_string();
                *state.loading.write().await = false;
                return;
            }

            match state.redis_client.execute_command(&cmd).await {
                Ok(result) => {
                    *state.command_output.write().await = result;
                }
                Err(e) => {
                    state.show_err(tr_fmt(keys::GENERIC_ERROR, lang, &[&e.to_string()]));
                }
            }

            *state.loading.write().await = false;
        });
    }

    pub(crate) fn show_err(&self, err: String) {
        let state = self.clone();
        *state.command_output.blocking_write() = err;
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
