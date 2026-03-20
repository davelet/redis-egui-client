use super::super::AppState;
use e_client_basics::constants::WILD_KEY_FILTER;
use e_client_bilingual::translations::{keys, tr_fmt};

/// Spawn connection with optional initial database
pub fn spawn_connect_with_db(state: &AppState, initial_db: Option<i64>) {
    let state = state.clone();
    tokio::spawn(async move {
        *state.loading.write().await = true;
        let param = state.connection_param.read().await.clone();
        if let Some(mut connection_config) = param {
            connection_config.database = initial_db;
            match state.redis_client.connect(connection_config).await {
                Ok(_) => {
                    *state.connected.write().await = true;
                    *state.loading.write().await = false;
                    *state.error_message.write().await = String::new();
                    *state.scan_cursor.write().await = 0;
                    *state.scan_has_more.write().await = true;
                    *state.loaded_keys_count.write().await = 0;
                    *state.loading_progress_text.write().await = String::new();

                    // Get total key count in database
                    if let Ok(total) = state.redis_client.get_db_size().await {
                        *state.total_keys.write().await = total;
                    }

                    // Set initial key filter to "*"
                    let current_filter = state.key_filter.read().await.clone();
                    if current_filter.is_empty() {
                        *state.key_filter.write().await = WILD_KEY_FILTER.to_string();
                    }

                    if let Ok(dbs) = state.redis_client.get_databases().await {
                        *state.databases.write().await = dbs;
                    }

                    state.spawn_load_keys();
                }
                Err(e) => {
                    *state.connected.write().await = false;
                    *state.loading.write().await = false;
                    let lang = *state.language.read().await;
                    *state.error_message.write().await =
                        tr_fmt(keys::CONNECTION_FAILED_MSG, lang, &[&e.to_string()]);
                }
            }
        } else {
            *state.connected.write().await = false;
            *state.loading.write().await = false;
        }
    });
}

/// Spawn disconnect
pub fn spawn_disconnect(state: &AppState) {
    let state = state.clone();
    tokio::spawn(async move {
        state.redis_client.disconnect().await;
        *state.connected.write().await = false;
        *state.keys.write().await = vec![];
        *state.needs_repaint.write().await = true; // Trigger repaint to clear UI
        *state.selected_key.write().await = None;
        *state.key_value.write().await = None;
        *state.error_message.write().await = String::new();
        *state.scan_cursor.write().await = 0;
        *state.scan_has_more.write().await = true;
        *state.loaded_keys_count.write().await = 0;
        *state.loading_progress_text.write().await = String::new();
    });
}
