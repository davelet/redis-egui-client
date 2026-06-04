use super::super::AppState;
use crate::redis_client::ValueData;
use e_client_basics::constants::ITEMS_PER_LOAD;

/// Spawn load list range operation
pub fn spawn_load_list_range(state: &AppState, key: String, start: isize, stop: isize) {
    let state = state.clone();
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
                // Explicitly release the write lock before issuing a repaint.
                // The `needs_repaint` flag lives on the same shared state, and
                // the spawned task here is the *only* writer for this branch,
                // so dropping the guard up front keeps the critical section
                // as short as possible and lets any UI-side `blocking_read`
                // proceed while the repaint callback runs. The same pattern
                // is used in every other spawn_* helper in this module.
                drop(value);
                state.request_repaint().await;
            }
            Err(_) => {}
        }
    });
}

/// Spawn load hash fields operation
/// This will clear all loaded values when reloading fields.
pub fn spawn_load_hash_fields(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        let filter = state.hash_field_filter.read().await.clone();
        let match_pattern = if filter.is_empty() {
            "*".to_string()
        } else if filter.contains('*') {
            filter
        } else {
            format!("*{}*", filter)
        };

        match state
            .redis_client
            .get_hash_fields(&key, &match_pattern)
            .await
        {
            Ok(fields) => {
                let mut value = state.key_value.write().await;
                if let Some(ValueData::Hash {
                    fields: existing_fields,
                    loaded_values,
                    ..
                }) = value.as_mut()
                {
                    *existing_fields = fields;
                    loaded_values.clear();
                }
                drop(value);
                state.request_repaint().await;
            }
            Err(_) => {}
        }
    });
}

/// Spawn load hash fields operation while preserving existing loaded values
/// Use this when you want to refresh the field list but keep already loaded values.
pub fn spawn_load_hash_fields_preserve_values(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        let filter = state.hash_field_filter.read().await.clone();
        let match_pattern = if filter.is_empty() {
            "*".to_string()
        } else if filter.contains('*') {
            filter
        } else {
            format!("*{}*", filter)
        };

        match state
            .redis_client
            .get_hash_fields(&key, &match_pattern)
            .await
        {
            Ok(fields) => {
                let mut value = state.key_value.write().await;
                if let Some(ValueData::Hash {
                    fields: existing_fields,
                    loaded_values,
                    ..
                }) = value.as_mut()
                {
                    // Remove values for fields that no longer exist
                    let fields_set: std::collections::HashSet<&String> = fields.iter().collect();
                    loaded_values.retain(|field, _| fields_set.contains(field));
                    // Update fields list after building the set
                    *existing_fields = fields;
                }
                drop(value);
                state.request_repaint().await;
            }
            Err(_) => {}
        }
    });
}

/// Spawn load hash field value operation.
/// While the request is in flight the `(key, field)` pair is added to
/// `loading_hash_fields` so the UI can show a "Loading..." state and reject
/// duplicate clicks. The entry is always removed at the end, even on error,
/// to avoid leaving the button stuck.
pub fn spawn_load_hash_field_value(state: &AppState, key: String, field: String) {
    let state = state.clone();
    tokio::spawn(async move {
        // Mark as loading. If another task is already loading this field, skip
        // to avoid issuing a duplicate HGET.
        {
            let mut loading = state.loading_hash_fields.write().await;
            if !loading.insert((key.clone(), field.clone())) {
                return;
            }
        }
        state.request_repaint().await;

        let result = state.redis_client.get_hash_field_value(&key, &field).await;

        if let Ok(Some(value_str)) = result {
            let mut value = state.key_value.write().await;
            if let Some(ValueData::Hash { loaded_values, .. }) = value.as_mut() {
                loaded_values.insert(field.clone(), value_str);
            }
        }

        state
            .loading_hash_fields
            .write()
            .await
            .remove(&(key, field));
        state.request_repaint().await;
    });
}

/// Spawn load all unloaded hash field values for editing
/// This loads all field values that haven't been loaded yet, preparing for edit mode.
pub fn spawn_load_all_hash_field_values(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        // Set flag indicating we're loading fields for edit
        *state.loading_fields_for_edit.write().await = true;

        // Get the list of fields that need to be loaded
        let fields_to_load: Vec<String> = {
            let value = state.key_value.read().await;
            if let Some(ValueData::Hash {
                fields,
                loaded_values,
                ..
            }) = value.as_ref()
            {
                fields
                    .iter()
                    .filter(|f| !loaded_values.contains_key(*f))
                    .cloned()
                    .collect()
            } else {
                vec![]
            }
        };

        // Load each field value
        for field in fields_to_load {
            if let Ok(Some(value_str)) = state.redis_client.get_hash_field_value(&key, &field).await
            {
                let mut value = state.key_value.write().await;
                if let Some(ValueData::Hash { loaded_values, .. }) = value.as_mut() {
                    loaded_values.insert(field, value_str);
                }
                drop(value);
                state.request_repaint().await;
            }
        }

        // Clear the flag when done
        *state.loading_fields_for_edit.write().await = false;

        // Check if we should auto-enter edit mode after loading
        let should_enter_edit = *state.pending_edit_after_load.read().await;
        if should_enter_edit {
            // Get current value and TTL for edit
            let value = state.key_value.read().await.clone();
            let ttl = *state.pending_edit_ttl.read().await;

            // Enter edit mode
            state.edit_state.write().await.enter_edit(&key, ttl, &value);

            // Clear the pending flag
            *state.pending_edit_after_load.write().await = false;
        }
        // Final repaint: flushes `loading_fields_for_edit = false` and, when
        // the caller requested it, the freshly-entered edit-mode state, so
        // the next frame shows the editor instead of a stale loading UI.
        state.request_repaint().await;
    });
}

/// Spawn load set members operation
pub fn spawn_load_set_members(state: &AppState, key: String) {
    let state = state.clone();
    tokio::spawn(async move {
        match state
            .redis_client
            .get_set_members(&key, 0, ITEMS_PER_LOAD)
            .await
        {
            Ok((_, members)) => {
                let mut value = state.key_value.write().await;
                if let Some(ValueData::Set {
                    items: existing, ..
                }) = value.as_mut()
                {
                    *existing = members;
                }
                drop(value);
                state.request_repaint().await;
            }
            Err(_) => {}
        }
    });
}

/// Spawn load sorted set range operation
pub fn spawn_load_zset_range(state: &AppState, key: String, start: isize, stop: isize) {
    let state = state.clone();
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
                drop(value);
                state.request_repaint().await;
            }
            Err(_) => {}
        }
    });
}
