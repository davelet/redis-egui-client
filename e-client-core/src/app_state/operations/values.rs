use super::super::AppState;
use super::super::{EditedValue, JsonValue, compact_json_if_single_line};

/// Spawn save element operation
pub fn spawn_save_element(
    state: &AppState,
    key: String,
    key_type: String,
    field: String,
    value: String,
) {
    let state = state.clone();
    tokio::spawn(async move {
        // Compact JSON if original was single-line
        let value_to_save = compact_json_if_single_line(&value);

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
                // Reload value
                state.spawn_load_value(key);
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
            // Refresh TTL display
            state.spawn_load_value(key);
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

        // 2. Update TTL
        if let Ok(ttl) = edit.edited_ttl.trim().parse::<i64>() {
            if let Err(e) = state.redis_client.set_ttl(&current_key, ttl).await {
                state.edit_state.write().await.save_message = format!("TTL update failed: {}", e);
                return;
            }
        }

        // 3. Save value by type
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

        match save_result {
            Ok(_) => {
                state.edit_state.write().await.cancel_edit();
                // Reload value and keys
                *state.selected_key.write().await = Some(current_key.clone());
                state.spawn_load_value(current_key);
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
