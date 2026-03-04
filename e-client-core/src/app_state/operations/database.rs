use super::super::AppState;

/// Spawn select database
pub fn spawn_select_db(state: &AppState, db: u32) {
    let state = state.clone();
    tokio::spawn(async move {
        if state.redis_client.select_db(db).await.is_ok() {
            *state.current_db.write().await = db;
            *state.scan_cursor.write().await = 0;
            *state.scan_has_more.write().await = true;
            *state.loaded_keys_count.write().await = 0;
            *state.loading_progress_text.write().await = String::new();

            // Get total key count in new database
            if let Ok(total) = state.redis_client.get_db_size().await {
                *state.total_keys.write().await = total;
            }

            state.spawn_load_keys();
        }
    });
}
