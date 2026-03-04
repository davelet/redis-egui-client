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
