use crate::redis_client::ValueData;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{compact_json_if_single_line, format_json_for_edit, operations, EditState};

/// Main application state for a Redis connection tab
#[derive(Clone)]
pub struct AppState {
    pub redis_client: crate::redis_client::RedisClient,
    pub connection_param: Arc<RwLock<Option<e_client_config::connection::RedisConnectionConfig>>>,
    pub connected: Arc<RwLock<bool>>,
    pub current_db: Arc<RwLock<u32>>,
    pub databases: Arc<RwLock<Vec<u32>>>,
    pub keys: Arc<RwLock<Vec<String>>>,
    pub selected_key: Arc<RwLock<Option<String>>>,
    pub key_value: Arc<RwLock<Option<ValueData>>>,
    pub key_ttl: Arc<RwLock<i64>>,
    pub edit_state: Arc<RwLock<EditState>>,
    pub key_filter: Arc<RwLock<String>>,
    pub hash_field_filter: Arc<RwLock<String>>,
    pub loading: Arc<RwLock<bool>>,
    pub error_message: Arc<RwLock<String>>,
    pub language: Arc<RwLock<Language>>,
    pub scan_cursor: Arc<RwLock<u64>>,
    pub scan_has_more: Arc<RwLock<bool>>,
    pub total_keys: Arc<RwLock<usize>>,
    pub loaded_keys_count: Arc<RwLock<usize>>,
    pub loading_progress_text: Arc<RwLock<String>>,
    pub ttl_edit_mode: Arc<RwLock<bool>>,
    pub ttl_edit_value: Arc<RwLock<String>>,
    pub ttl_edit_key: Arc<RwLock<Option<String>>>,
    /// Last time TTL was auto-refreshed (for timer-based TTL refresh)
    pub ttl_last_refresh: Arc<RwLock<Option<std::time::Instant>>>,
    /// Flag indicating we're loading hash fields to prepare for editing
    pub loading_fields_for_edit: Arc<RwLock<bool>>,
    /// Flag indicating we should enter edit mode after loading fields
    pub pending_edit_after_load: Arc<RwLock<bool>>,
    /// Stored TTL for pending edit after load
    pub pending_edit_ttl: Arc<RwLock<i64>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            redis_client: crate::redis_client::RedisClient::new(),
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
            hash_field_filter: Arc::new(RwLock::new(String::new())),
            loading: Arc::new(RwLock::new(false)),
            error_message: Arc::new(RwLock::new(String::new())),
            language: Arc::new(RwLock::new(Language::English)),
            scan_cursor: Arc::new(RwLock::new(0)),
            scan_has_more: Arc::new(RwLock::new(true)),
            total_keys: Arc::new(RwLock::new(0)),
            loaded_keys_count: Arc::new(RwLock::new(0)),
            loading_progress_text: Arc::new(RwLock::new(String::new())),
            ttl_edit_mode: Arc::new(RwLock::new(false)),
            ttl_edit_value: Arc::new(RwLock::new(String::new())),
            ttl_edit_key: Arc::new(RwLock::new(None)),
            ttl_last_refresh: Arc::new(RwLock::new(None)),
            loading_fields_for_edit: Arc::new(RwLock::new(false)),
            pending_edit_after_load: Arc::new(RwLock::new(false)),
            pending_edit_ttl: Arc::new(RwLock::new(-1)),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    // Delegate connection operations
    pub fn spawn_connect(&self) {
        self.spawn_connect_with_db(None);
    }

    pub fn spawn_connect_with_db(&self, initial_db: Option<i64>) {
        super::operations::connection::spawn_connect_with_db(self, initial_db);
    }

    pub fn spawn_disconnect(&self) {
        super::operations::connection::spawn_disconnect(self);
    }

    // Delegate database operations
    pub fn spawn_select_db(&self, db: u32) {
        super::operations::database::spawn_select_db(self, db);
    }

    // Delegate key operations
    pub fn spawn_load_keys(&self) {
        super::operations::keys::spawn_load_keys(self);
    }

    pub fn spawn_load_more_keys(&self, load_all: bool) {
        super::operations::keys::spawn_load_more_keys(self, load_all);
    }

    pub fn spawn_load_value(&self, key: String, reload_hash: bool) {
        super::operations::keys::spawn_load_value(self, key, reload_hash);
    }

    pub fn spawn_refresh_ttl_only(&self, key: String) {
        super::operations::keys::spawn_refresh_ttl_only(self, key);
    }

    pub fn spawn_create_new_key(&self, key: String, key_type: String, value: String, ttl: i64) {
        super::operations::keys::spawn_create_new_key(self, key, key_type, value, ttl);
    }

    pub fn spawn_delete_key(&self, key: String) {
        super::operations::keys::spawn_delete_key(self, key);
    }

    // Delegate value operations
    pub fn spawn_save_element(
        &self,
        key: String,
        key_type: String,
        field: String,
        value: String,
        original_value: String,
    ) {
        super::operations::values::spawn_save_element(
            self,
            key,
            key_type,
            field,
            value,
            original_value,
        );
    }

    pub fn spawn_update_ttl(&self, key: String, ttl: i64) {
        super::operations::values::spawn_update_ttl(self, key, ttl);
    }

    pub fn spawn_save_edits(&self, original_key: String) {
        super::operations::values::spawn_save_edits(self, original_key);
    }

    // Delegate lazy loading operations
    pub fn spawn_load_list_range(&self, key: String, start: isize, stop: isize) {
        operations::lazy_load::spawn_load_list_range(self, key, start, stop);
    }

    pub fn spawn_load_hash_fields(&self, key: String) {
        operations::lazy_load::spawn_load_hash_fields(self, key);
    }

    pub fn spawn_load_hash_fields_preserve_values(&self, key: String) {
        operations::lazy_load::spawn_load_hash_fields_preserve_values(self, key);
    }

    pub fn spawn_load_hash_field_value(&self, key: String, field: String) {
        operations::lazy_load::spawn_load_hash_field_value(self, key, field);
    }

    pub fn spawn_load_all_hash_field_values(&self, key: String) {
        operations::lazy_load::spawn_load_all_hash_field_values(self, key);
    }

    pub fn spawn_load_set_members(&self, key: String) {
        operations::lazy_load::spawn_load_set_members(self, key);
    }

    pub fn spawn_load_zset_range(&self, key: String, start: isize, stop: isize) {
        operations::lazy_load::spawn_load_zset_range(self, key, start, stop);
    }
}
