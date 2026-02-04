use crate::core::app_state::AppState;
use crate::core::redis_client::ValueData;
use crate::ui::window::new_connection_window::NewConnectionWindowWindow;
use e_client_basics::constants::{
    DEFAULT_SIDE_PANEL_WIDTH, MAX_SIDE_PANEL_WIDTH, MIN_SIDE_PANEL_WIDTH, UI_REPAINT_INTERVAL_MS,
};
use e_client_config::config::Config;
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export panel functions for convenient access
pub use panels::{
    render_central_panel, render_side_panel, render_status_bar, render_tab_bar, render_top_panel,
};

mod new_connection_window;
pub mod panels;

/// Represents a single Redis connection tab
pub struct RedisTab {
    pub id: usize,
    pub name: String,
    pub state: AppState,
    pub selected_connection: Option<usize>,
    pub connected_color: Option<String>, // Connection color (hex) after successful connection
    pub key_filter_input: String,
    pub side_panel_width: f32, // Current side panel width for this tab
}

impl RedisTab {
    pub fn new(id: usize, language: Language) -> Self {
        let mut state = AppState::new();
        state.language = Arc::new(RwLock::new(language));

        Self {
            id,
            name: format!("{}{}", tr(keys::TAB, language), id),
            state,
            selected_connection: None,
            connected_color: None,
            key_filter_input: String::new(),
            side_panel_width: DEFAULT_SIDE_PANEL_WIDTH,
        }
    }

    pub fn with_connection(
        id: usize,
        conn_idx: usize,
        conn: RedisConnectionConfig,
        language: Language,
    ) -> Self {
        let mut tab = Self::new(id, language);
        tab.selected_connection = Some(conn_idx);
        tab.name = conn.name.clone();
        tab.connected_color = conn.color.clone();
        *tab.state.connection_param.blocking_write() = Some(conn);
        tab
    }
}

pub struct RedisApp {
    tabs: Vec<RedisTab>,
    active_tab: usize,
    next_tab_id: usize,
    config: Config,
    new_connection: NewConnectionWindowWindow,
    global_language: Language,
    // Track previous connection states to detect changes
    prev_connected_states: Vec<bool>,
}

impl eframe::App for RedisApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Get the viewport information before the async block
        let viewport = ctx.input(|i| i.viewport().clone());
        let is_maximized = viewport.maximized.unwrap_or(false);
        let rect = viewport.outer_rect;

        // Clone only what we need for the async block
        let mut config = self.config.clone();

        // Spawn the async task for saving window state
        if let Some(rect) = rect {
            if !config.window.maximized {
                let _ = config.update_window_position(rect.min.x, rect.min.y);
                let _ = config.update_window_size(rect.width(), rect.height());
            }

            if is_maximized != config.window.maximized {
                let _ = config.update_maximized(is_maximized);
            }
        }

        // Handle language updates
        if let Some(active_tab) = self.get_active_tab() {
            let current_lang = active_tab.state.language.blocking_read();
            let current_lang_str = current_lang.to_file_string();
            drop(current_lang); // Drop the lock before borrowing config mutably

            if self.config.settings.language != current_lang_str {
                if let Err(e) = self.config.update_language(&current_lang_str) {
                    eprintln!(
                        "Failed to update language: {}",
                        e.to_message(Language::English)
                    );
                }
            }
        }

        // Detect connection state changes
        for (idx, tab) in self.tabs.iter().enumerate() {
            let current_connected = tab.state.connected.blocking_read().clone();
            if idx < self.prev_connected_states.len() {
                self.prev_connected_states[idx] = current_connected;
            } else {
                self.prev_connected_states.push(current_connected);
            }
        }

        // Render tab bar
        render_tab_bar(self, ctx);

        // Render status bar first to ensure it's on top
        render_status_bar(self, ctx);

        // Update UI (delegated to panels module)
        render_top_panel(self, ctx);
        render_side_panel(self, ctx);
        render_central_panel(self, ctx);

        // Force continuous repaint while loading to ensure smooth UI updates
        // This solves the issue where async updates might not trigger repaints consistently
        let mut any_loading = false;
        for tab in &self.tabs {
            if tab.state.loading.try_read().map(|v| *v).unwrap_or(false) {
                any_loading = true;
                break;
            }
        }
        if any_loading {
            // Request repaint immediately and schedule the next one within a few milliseconds
            ctx.request_repaint();
            ctx.request_repaint_after(std::time::Duration::from_millis(UI_REPAINT_INTERVAL_MS));
        }
    }
}

impl RedisApp {
    pub fn with_config(config: Config) -> Self {
        let global_language = if !config.settings.language.is_empty() {
            Language::file_name_to_lang(&config.settings.language)
        } else {
            Language::English
        };

        // Create initial tab
        let initial_tab = RedisTab::new(0, global_language);

        Self {
            tabs: vec![initial_tab],
            active_tab: 0,
            next_tab_id: 1,
            config,
            new_connection: NewConnectionWindowWindow::new(),
            global_language,
            prev_connected_states: vec![false],
        }
    }

    pub fn get_active_tab(&self) -> Option<&RedisTab> {
        self.tabs.get(self.active_tab)
    }

    pub fn get_active_tab_mut(&mut self) -> Option<&mut RedisTab> {
        self.tabs.get_mut(self.active_tab)
    }

    pub fn create_new_tab(&mut self) {
        let new_tab = RedisTab::new(self.next_tab_id, self.global_language);
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;
        self.prev_connected_states.push(false);
    }

    pub fn create_tab_with_connection(&mut self, conn_idx: usize, conn: RedisConnectionConfig) {
        let new_tab =
            RedisTab::with_connection(self.next_tab_id, conn_idx, conn, self.global_language);
        let new_tab_idx = self.tabs.len();
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;

        // Load preferences for this connection
        self.load_connection_preferences(new_tab_idx);

        // Add initial connection state (not connected yet)
        self.prev_connected_states.push(false);

        // Auto-connect with preferred DB
        self.spawn_connect_with_initial_db(new_tab_idx);
    }

    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            // Don't close the last tab
            return;
        }

        if index < self.tabs.len() {
            // Disconnect before closing
            let tab = &self.tabs[index];
            tab.state.spawn_disconnect();

            self.tabs.remove(index);
            self.prev_connected_states.remove(index);

            // Adjust active tab index
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            } else if self.active_tab > index {
                self.active_tab -= 1;
            }
        } else {
            // Disconnect from the active tab if no tab is being closed
            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                let lang = *tab.state.language.blocking_read();
                tab.state.spawn_disconnect();
                // Clear tab name and color on disconnect
                tab.name = format!("{} {}", tr(keys::TAB, lang), tab.id);
                tab.connected_color = None;
                tab.selected_connection = None;
            }
        }
    }

    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    pub fn close_other_tabs(&mut self, keep_index: usize) {
        if self.tabs.len() <= 1 {
            return;
        }

        // Disconnect and remove all tabs except the one to keep
        let mut indices_to_remove: Vec<usize> = Vec::new();
        for idx in 0..self.tabs.len() {
            if idx != keep_index {
                indices_to_remove.push(idx);
            }
        }

        // Remove in reverse order to avoid index shifting
        for idx in indices_to_remove.into_iter().rev() {
            if idx < self.tabs.len() {
                let tab = &self.tabs[idx];
                tab.state.spawn_disconnect();
                self.tabs.remove(idx);
                self.prev_connected_states.remove(idx);
            }
        }

        // Adjust active tab index
        self.active_tab = 0;
    }

    // Helper methods for compatibility with panels
    fn poll_bool(&self, lock: Arc<RwLock<bool>>) -> bool {
        lock.try_read().map(|v| *v).unwrap_or(false)
    }

    fn poll_u32(&self, lock: Arc<RwLock<u32>>) -> u32 {
        lock.try_read().map(|v| *v).unwrap_or(0)
    }

    fn poll_usize(&self, lock: Arc<RwLock<usize>>) -> usize {
        lock.try_read().map(|v| *v).unwrap_or(0)
    }

    fn poll_string(&self, lock: Arc<RwLock<String>>) -> String {
        lock.try_read().map(|v| v.clone()).unwrap_or_default()
    }

    fn poll_vec_string(&self, lock: Arc<RwLock<Vec<String>>>) -> Vec<String> {
        lock.try_read().map(|v| v.clone()).unwrap_or_default()
    }

    fn poll_vec_u32(&self, lock: Arc<RwLock<Vec<u32>>>) -> Vec<u32> {
        lock.try_read().map(|v| v.clone()).unwrap_or_default()
    }

    fn poll_option_string(&self, lock: Arc<RwLock<Option<String>>>) -> Option<String> {
        lock.try_read().ok().and_then(|v| v.clone())
    }

    fn poll_option_value(&self, lock: Arc<RwLock<Option<ValueData>>>) -> Option<ValueData> {
        lock.blocking_read().clone()
    }

    fn update_string(&self, lock: Arc<RwLock<String>>, value: String) {
        if let Ok(mut v) = lock.try_write() {
            *v = value;
        }
    }

    fn poll_language(&self, lock: Arc<RwLock<Language>>) -> Language {
        *lock.blocking_read()
    }

    fn update_language(&mut self, lang: Language) {
        self.global_language = lang;
        // Update all tabs' language and names
        for tab in self.tabs.iter_mut() {
            *tab.state.language.blocking_write() = lang;
            // Update tab name if it's the default name (has connection name format)
            // Only update default tab names, not connection names
            if tab.name.starts_with("Tab ") || tab.name.starts_with("标签页") {
                tab.name = format!("{}{}", tr(keys::TAB, lang), tab.id);
            }
        }
    }

    fn update_tab_side_panel_width(&mut self, tab_idx: usize, width: f32) {
        let conn_name = if let Some(tab) = self.tabs.get(tab_idx) {
            tab.selected_connection
                .and_then(|idx| self.config.connections.get(idx).map(|c| c.name.clone()))
        } else {
            None
        };

        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            let rounded_width = (width.max(MIN_SIDE_PANEL_WIDTH).min(MAX_SIDE_PANEL_WIDTH)).round();
            tab.side_panel_width = rounded_width;

            // Save to config if connected
            if let Some(name) = conn_name {
                let _ = self
                    .config
                    .update_side_panel_width_for_connection(&name, rounded_width);
            }
        }
    }

    fn load_connection_preferences(&mut self, tab_idx: usize) {
        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            if let Some(conn_idx) = tab.selected_connection {
                if let Some(conn) = self.config.connections.get(conn_idx) {
                    if let Some(pref) = self.config.get_connection_preference(&conn.name) {
                        tab.side_panel_width = pref.side_panel_width;
                        // Set preferred DB and spawn connect with it
                        // This ensures the connection uses the preferred DB from the start
                        *tab.state.current_db.blocking_write() = pref.db;
                    }
                }
            }
        }
    }

    pub fn spawn_connect_with_initial_db(&self, tab_idx: usize) {
        if let Some(tab) = self.tabs.get(tab_idx) {
            let initial_db = *tab.state.current_db.blocking_read();
            tab.state.spawn_connect_with_db(Some(initial_db as i64));
        }
    }

    fn update_db_for_connection(&mut self, connection_name: &str, db: u32) {
        let _ = self.config.update_db_for_connection(connection_name, db);
    }
}

impl Drop for RedisApp {
    fn drop(&mut self) {
        // Disconnect all tabs when app is closed (immediate, no async wait)
        for tab in &self.tabs {
            tab.state.redis_client.disconnect_sync();
        }
    }
}
