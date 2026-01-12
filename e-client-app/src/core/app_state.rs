use crate::core::redis_client::{RedisClient, ValueData};
use std::sync::Arc;
use tokio::sync::RwLock;

use e_client_config::constants::DEFAULT_KEY_FILTER;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::{tr, tr_fmt};

#[derive(Clone)]
pub struct AppState {
    pub redis_client: RedisClient,
    pub connection_url: Arc<RwLock<String>>,
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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            redis_client: RedisClient::new(),
            connection_url: Arc::new(RwLock::new("".to_string())),
            connected: Arc::new(RwLock::new(false)),
            current_db: Arc::new(RwLock::new(0)),
            databases: Arc::new(RwLock::new(vec![])),
            keys: Arc::new(RwLock::new(vec![])),
            selected_key: Arc::new(RwLock::new(None)),
            key_value: Arc::new(RwLock::new(None)),
            command_input: Arc::new(RwLock::new(String::new())),
            command_output: Arc::new(RwLock::new(String::new())),
            key_filter: Arc::new(RwLock::new(DEFAULT_KEY_FILTER.to_string())),
            loading: Arc::new(RwLock::new(false)),
            language: Arc::new(RwLock::new(Language::English)),
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
            let url = state.connection_url.read().await.clone();
            let lang = *state.language.read().await;

            match state.redis_client.connect(&url).await {
                Ok(_) => {
                    *state.connected.write().await = true;

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
        });
    }

    pub fn spawn_select_db(&self, db: u32) {
        let state = self.clone();
        tokio::spawn(async move {
            if state.redis_client.select_db(db).await.is_ok() {
                *state.current_db.write().await = db;
                state.spawn_load_keys();
            }
        });
    }

    pub fn spawn_load_keys(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let pattern = state.key_filter.read().await.clone();
            let mut all_keys = vec![];
            let mut cursor = 0;

            loop {
                match state.redis_client.scan_keys(cursor, &pattern, 100).await {
                    Ok((new_cursor, keys)) => {
                        all_keys.extend(keys);
                        cursor = new_cursor;
                        if cursor == 0 {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            all_keys.sort();
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
}
