use crate::redis_client::ValueData;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{EditState, operations};

/// Type alias for the common Arc<RwLock<T>> pattern
pub type Shared<T> = Arc<RwLock<T>>;

/// Repaint signal invoked by background tasks to wake the UI thread.
/// Defaults to a no-op so non-UI consumers (tests, CLI tools) work unchanged.
/// The UI layer installs a closure that calls `egui::Context::request_repaint`.
pub type RepaintFn = Arc<dyn Fn() + Send + Sync>;

/// Helper function to create a new Shared<T>
fn shared<T>(value: T) -> Shared<T> {
    Arc::new(RwLock::new(value))
}

/// Main application state for a Redis connection tab
#[derive(Clone)]
pub struct AppState {
    pub redis_client: crate::redis_client::RedisClient,
    // Connection state
    pub connection_param: Shared<Option<e_client_config::connection::RedisConnectionConfig>>,
    pub connected: Shared<bool>,
    pub current_db: Shared<u32>,
    pub databases: Shared<Vec<u32>>,
    // Key state
    pub keys: Shared<Vec<String>>,
    pub selected_key: Shared<Option<String>>,
    pub key_value: Shared<Option<ValueData>>,
    pub key_ttl: Shared<i64>,
    pub key_filter: Shared<String>,
    pub hash_field_filter: Shared<String>,
    // Scan/pagination state
    pub scan_cursor: Shared<u64>,
    pub scan_has_more: Shared<bool>,
    pub total_keys: Shared<usize>,
    pub loaded_keys_count: Shared<usize>,
    pub loading_progress_text: Shared<String>,
    // Edit state
    pub edit_state: Shared<EditState>,
    // TTL edit state
    pub ttl_edit_mode: Shared<bool>,
    pub ttl_edit_value: Shared<String>,
    pub ttl_edit_key: Shared<Option<String>>,
    pub ttl_last_refresh: Shared<Option<std::time::Instant>>,
    // Pending edit state
    pub loading_fields_for_edit: Shared<bool>,
    pub pending_edit_after_load: Shared<bool>,
    pub pending_edit_ttl: Shared<i64>,
    /// (key, field) pairs whose value is currently being fetched. Used by the
    /// UI to swap the per-field "Load" button to "Loading..." and disable it
    /// to prevent duplicate requests. Keyed on the tuple so that switching
    /// to a different hash mid-load does not collide.
    pub loading_hash_fields: Shared<std::collections::HashSet<(String, String)>>,
    // Global state
    pub loading: Shared<bool>,
    pub needs_repaint: Shared<bool>, // Flag to request UI repaint (e.g., when keys change)
    pub error_message: Shared<String>,
    pub language: Shared<Language>,
    /// Callback used by background tasks to wake the UI thread immediately
    /// after mutating shared state. The `needs_repaint` flag only helps when
    /// `update()` happens to be running; if egui has gone idle (no input
    /// events) the flag is never read and the UI freezes on stale data until
    /// the user moves the mouse. Invoking this callback issues an
    /// `egui::Context::request_repaint` from any thread, which is the only
    /// thread-safe way to force a repaint when egui is asleep.
    pub repaint: Shared<Option<RepaintFn>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            redis_client: crate::redis_client::RedisClient::new(),
            connection_param: shared(None),
            connected: shared(false),
            current_db: shared(0),
            databases: shared(vec![]),
            keys: shared(vec![]),
            selected_key: shared(None),
            key_value: shared(None),
            key_ttl: shared(-2),
            edit_state: shared(EditState::default()),
            key_filter: shared(String::new()),
            hash_field_filter: shared(String::new()),
            loading: shared(false),
            needs_repaint: shared(false),
            error_message: shared(String::new()),
            language: shared(Language::English),
            scan_cursor: shared(0),
            scan_has_more: shared(true),
            total_keys: shared(0),
            loaded_keys_count: shared(0),
            loading_progress_text: shared(String::new()),
            ttl_edit_mode: shared(false),
            ttl_edit_value: shared(String::new()),
            ttl_edit_key: shared(None),
            ttl_last_refresh: shared(None),
            loading_fields_for_edit: shared(false),
            pending_edit_after_load: shared(false),
            pending_edit_ttl: shared(-1),
            loading_hash_fields: shared(std::collections::HashSet::new()),
            repaint: shared(None),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Install the closure that wakes the UI thread. Call once per tab from
    /// the UI layer with `Arc::new(move || ctx.request_repaint())`.
    pub fn set_repaint_callback(&self, cb: RepaintFn) {
        if let Ok(mut guard) = self.repaint.try_write() {
            *guard = Some(cb);
        }
    }

    /// Wake the UI thread from a background task. Combined with setting
    /// `needs_repaint = true` this guarantees the next frame observes the
    /// state change even when egui was idle.
    pub async fn request_repaint(&self) {
        *self.needs_repaint.write().await = true;
        if let Some(cb) = self.repaint.read().await.as_ref() {
            cb();
        }
    }

    // === Connection Operations ===
    pub fn spawn_connect(&self) {
        self.spawn_connect_with_db(None);
    }

    pub fn spawn_connect_with_db(&self, initial_db: Option<i64>) {
        super::operations::connection::spawn_connect_with_db(self, initial_db);
    }

    pub fn spawn_disconnect(&self) {
        super::operations::connection::spawn_disconnect(self);
    }

    // === Database Operations ===
    pub fn spawn_select_db(&self, db: u32) {
        super::operations::database::spawn_select_db(self, db);
    }

    // === Key Operations ===
    pub fn spawn_load_keys(&self) {
        super::operations::keys::spawn_load_keys(self);
    }

    pub fn spawn_load_more_keys(&self, load_all: bool) {
        super::operations::keys::spawn_load_more_keys(self, load_all);
    }

    pub fn spawn_load_value(
        &self,
        key: String,
        hash_load_mode: super::operations::keys::HashLoadMode,
    ) {
        super::operations::keys::spawn_load_value(self, key, hash_load_mode);
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

    // === Value Operations ===
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

    // === Lazy Loading Operations ===
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
