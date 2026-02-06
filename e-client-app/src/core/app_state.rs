use crate::core::redis_client::{RedisClient, ValueData};
use e_client_basics::constants::{
    ITEMS_PER_LOAD, LOAD_MORE_BATCH_SIZE, MAX_INITIAL_KEYS, MAX_LOADED_KEYS, SCAN_COUNT,
    SCAN_SLEEP_INTERVAL_MS, SORT_INTERVAL_KEYS, UI_UPDATE_INTERVAL_BATCHES, WILD_KEY_FILTER,
};
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum EditedValue {
    String(String),
    List(Vec<String>),
    Hash(Vec<(String, String)>),
    Set(Vec<String>),
    ZSet(Vec<(String, String)>),
    None,
}

#[derive(Debug, Clone)]
pub struct EditState {
    pub editing: bool,
    pub edited_key: String,
    pub edited_ttl: String,
    pub edited_value: EditedValue,
    pub save_message: String,
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            editing: false,
            edited_key: String::new(),
            edited_ttl: String::new(),
            edited_value: EditedValue::None,
            save_message: String::new(),
        }
    }
}

impl EditState {
    pub fn enter_edit(&mut self, key: &str, ttl: i64, value: &Option<ValueData>) {
        self.editing = true;
        self.edited_key = key.to_string();
        self.edited_ttl = if ttl == -1 {
            "-1".to_string()
        } else if ttl >= 0 {
            ttl.to_string()
        } else {
            String::new()
        };
        self.edited_value = match value {
            Some(ValueData::String(s)) => EditedValue::String(s.clone()),
            Some(ValueData::List { items, .. }) => EditedValue::List(items.clone()),
            Some(ValueData::Hash { fields, .. }) => EditedValue::Hash(fields.clone()),
            Some(ValueData::Set { items, .. }) => EditedValue::Set(items.clone()),
            Some(ValueData::ZSet { items, .. }) => EditedValue::ZSet(
                items
                    .iter()
                    .map(|(m, s)| (m.clone(), s.to_string()))
                    .collect(),
            ),
            _ => EditedValue::None,
        };
        self.save_message.clear();
    }

    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.save_message.clear();
    }
}

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
    pub key_ttl: Arc<RwLock<i64>>,
    pub edit_state: Arc<RwLock<EditState>>,
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
            key_ttl: Arc::new(RwLock::new(-2)),
            edit_state: Arc::new(RwLock::new(EditState::default())),
            key_filter: Arc::new(RwLock::new(String::new())),
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
        self.spawn_connect_with_db(None);
    }

    pub fn spawn_connect_with_db(&self, initial_db: Option<i64>) {
        let state = self.clone();
        tokio::spawn(async move {
            *state.loading.write().await = true;
            let param = state.connection_param.read().await.clone();
            if let Some(mut connection_config) = param {
                connection_config.database = initial_db;
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
            if pattern == WILD_KEY_FILTER.to_string() {
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
                        // Sort and display every SORT_INTERVAL_KEYS keys
                        if loaded_count - last_sort_count >= SORT_INTERVAL_KEYS
                            || current_cursor == 0
                        {
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
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            SCAN_SLEEP_INTERVAL_MS,
                        ))
                        .await;

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

                        // Update UI every UI_UPDATE_INTERVAL_BATCHES SCAN batches for display
                        if batch_count - last_update_batch >= UI_UPDATE_INTERVAL_BATCHES
                            || current_cursor == 0
                        {
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
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            SCAN_SLEEP_INTERVAL_MS,
                        ))
                        .await;

                        // Check if we should stop loading more
                        let total_new_keys = existing_keys.len() - keys_at_start;
                        if !load_all
                            && (current_cursor == 0 || total_new_keys >= LOAD_MORE_BATCH_SIZE)
                        {
                            break;
                        }

                        // Stop if we reached MAX_LOADED_KEYS
                        if existing_keys.len() >= MAX_LOADED_KEYS {
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
            // Cancel editing when reloading
            state.edit_state.write().await.cancel_edit();

            match state.redis_client.get_value(&key).await {
                Ok(value) => {
                    *state.selected_key.write().await = Some(key.clone());
                    *state.key_value.write().await = Some(value);

                    // Get TTL for the key
                    match state.redis_client.get_ttl(&key).await {
                        Ok(ttl) => {
                            *state.key_ttl.write().await = ttl;
                        }
                        Err(_) => {
                            *state.key_ttl.write().await = -2;
                        }
                    }
                }
                Err(_) => {
                    // Ignore error - command_output was removed
                }
            }

            *state.loading.write().await = false;
        });
    }

    pub fn spawn_create_new_key(&self, key: String, key_type: String, value: String, ttl: i64) {
        let state = self.clone();
        tokio::spawn(async move {
            let result = match key_type.as_str() {
                "string" => state.redis_client.set_string(&key, &value).await,
                "list" => {
                    let mut r = Ok(());
                    for item in value.lines() {
                        let item = item.trim();
                        if !item.is_empty() {
                            if let Err(e) = state.redis_client.rpush(&key, item).await {
                                r = Err(e);
                                break;
                            }
                        }
                    }
                    r
                }
                "hash" => {
                    let mut r = Ok(());
                    for line in value.lines() {
                        let line = line.trim();
                        if let Some((field, val)) = line.split_once(':') {
                            let field = field.trim();
                            let val = val.trim();
                            if !field.is_empty() {
                                if let Err(e) = state.redis_client.hset(&key, field, val).await {
                                    r = Err(e);
                                    break;
                                }
                            }
                        }
                    }
                    r
                }
                "set" => {
                    let mut r = Ok(());
                    for item in value.lines() {
                        let item = item.trim();
                        if !item.is_empty() {
                            if let Err(e) = state.redis_client.sadd(&key, item).await {
                                r = Err(e);
                                break;
                            }
                        }
                    }
                    r
                }
                "zset" => {
                    let mut r = Ok(());
                    for line in value.lines() {
                        let line = line.trim();
                        if let Some((score_str, member)) = line.split_once(':') {
                            let score: f64 = score_str.trim().parse().unwrap_or(0.0);
                            let member = member.trim();
                            if !member.is_empty() {
                                if let Err(e) = state.redis_client.zadd(&key, score, member).await {
                                    r = Err(e);
                                    break;
                                }
                            }
                        }
                    }
                    r
                }
                _ => Ok(()),
            };

            match result {
                Ok(_) => {
                    // Set TTL if specified
                    if ttl >= 0 {
                        let _ = state.redis_client.set_ttl(&key, ttl).await;
                    }
                    // Select the new key and load its value
                    *state.selected_key.write().await = Some(key.clone());
                    state.edit_state.write().await.cancel_edit();
                    state.spawn_load_value(key);
                    state.spawn_load_keys();
                }
                Err(e) => {
                    *state.error_message.write().await = format!("Create key failed: {}", e);
                }
            }
        });
    }

    pub fn spawn_delete_key(&self, key: String) {
        let state = self.clone();
        tokio::spawn(async move {
            match state.redis_client.del_key(&key).await {
                Ok(_) => {
                    *state.selected_key.write().await = None;
                    *state.key_value.write().await = None;
                    *state.key_ttl.write().await = -2;
                    state.edit_state.write().await.cancel_edit();
                    // Refresh key list
                    state.spawn_load_keys();
                }
                Err(e) => {
                    state.edit_state.write().await.save_message = format!("Delete failed: {}", e);
                }
            }
        });
    }

    pub fn spawn_save_edits(&self, original_key: String) {
        let state = self.clone();
        let edit = state.edit_state.blocking_read().clone();
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
                        state.edit_state.write().await.save_message =
                            format!("Rename failed: {}", e);
                        return;
                    }
                }
            }

            // 2. Update TTL
            if let Ok(ttl) = edit.edited_ttl.trim().parse::<i64>() {
                if let Err(e) = state.redis_client.set_ttl(&current_key, ttl).await {
                    state.edit_state.write().await.save_message =
                        format!("TTL update failed: {}", e);
                    return;
                }
            }

            // 3. Save value by type
            let save_result = match &edit.edited_value {
                EditedValue::String(s) => state.redis_client.set_string(&current_key, s).await,
                EditedValue::Hash(fields) => {
                    // Delete old key and re-create with new fields
                    let _ = state.redis_client.del_key(&current_key).await;
                    let mut result = Ok(());
                    for (field, value) in fields {
                        if !field.is_empty() {
                            if let Err(e) =
                                state.redis_client.hset(&current_key, field, value).await
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
                        if let Err(e) = state.redis_client.rpush(&current_key, item).await {
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
                        if !item.is_empty() {
                            if let Err(e) = state.redis_client.sadd(&current_key, item).await {
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
                        if !member.is_empty() {
                            let score: f64 = score_str.parse().unwrap_or(0.0);
                            if let Err(e) =
                                state.redis_client.zadd(&current_key, score, member).await
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
                    state.edit_state.write().await.save_message = format!("Save failed: {}", e);
                }
            }
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
            match state
                .redis_client
                .get_hash_fields(&key, 0, ITEMS_PER_LOAD)
                .await
            {
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
