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
        match state
            .redis_client
            .get_hash_fields(&key, 0, ITEMS_PER_LOAD)
            .await
        {
            Ok((_, fields)) => {
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
        match state
            .redis_client
            .get_hash_fields(&key, 0, ITEMS_PER_LOAD)
            .await
        {
            Ok((_, fields)) => {
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
            }
            Err(_) => {}
        }
    });
}

/// Spawn load hash field value operation
pub fn spawn_load_hash_field_value(state: &AppState, key: String, field: String) {
    let state = state.clone();
    tokio::spawn(async move {
        match state.redis_client.get_hash_field_value(&key, &field).await {
            Ok(Some(value_str)) => {
                let mut value = state.key_value.write().await;
                if let Some(ValueData::Hash { loaded_values, .. }) = value.as_mut() {
                    loaded_values.insert(field, value_str);
                }
            }
            _ => {}
        }
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
            }
            Err(_) => {}
        }
    });
}
