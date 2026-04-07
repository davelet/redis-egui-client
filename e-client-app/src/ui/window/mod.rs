use crate::ui::window::new_connection_window::NewConnectionWindowWindow;
use e_client_basics::constants::UI_REPAINT_INTERVAL_MS;
use e_client_config::config::Config;
use e_client_config::language::Language;

// Re-export panel functions for convenient access
pub use panels::{
    render_central_panel, render_command_line_panel, render_side_panel, render_status_bar,
    render_tab_bar, render_top_panel,
};

// Re-export types
pub use types::{AiModelEditor, ElementEditDialog, NewKeyDialog, RedisTab};

pub mod copy_feedback;
pub mod components;
mod new_connection_window;
pub mod panels;
pub mod shortcut_manager;
mod types;

/// Main application state
pub struct RedisApp {
    tabs: Vec<RedisTab>,
    active_tab: usize,
    next_tab_id: usize,
    config: Config,
    new_connection: NewConnectionWindowWindow,
    show_settings: bool,
    new_key_dialog: NewKeyDialog,
    pub element_edit_dialog: ElementEditDialog,
    global_language: Language,
    // Track previous connection states to detect changes
    prev_connected_states: Vec<bool>,
    pub copy_feedback_manager: copy_feedback::CopyFeedbackManager,
    // Track if open connections prompt should be shown
    show_open_connections_prompt: bool,
    // Shortcut editing state
    pub shortcut_state: shortcut_manager::ShortcutManagerState,
    // Track if tab bar should scroll to show active tab
    scroll_to_tab: Option<usize>,
    // Track if all tabs dropdown should be shown (triggered by shortcut)
    pub show_all_tabs_dropdown: bool,
    // AI model editor
    pub ai_model_editor: AiModelEditor,
    // Delete connection confirmation: (connection index, connection name)
    pub delete_connection_confirm: Option<(usize, String)>,
    // Settings panel state - track expanded sections for mutually exclusive behavior
    pub settings_expanded_section: Option<SettingsSection>,
    // Help overlay
    show_help: bool,
    help_selected_section: Option<usize>,
}

/// Settings panel expandable sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    Ai,
    Shortcuts,
}

impl eframe::App for RedisApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Collect all connected connection names (deduplicated)
        let mut connected_names = Vec::new();
        for tab in &self.tabs {
            if let Some(conn_idx) = tab.selected_connection {
                if let Some(conn) = self.config.connections.get(conn_idx) {
                    // Check if the tab is actually connected
                    if *tab.state.connected.blocking_read() {
                        // Only add if not already present
                        if !connected_names.contains(&conn.name) {
                            connected_names.push(conn.name.clone());
                        }
                    }
                }
            }
        }

        // Save open connections to window config
        self.config
            .window
            .open_connections
            .set_connections(connected_names);
        // Mark window as dirty to save (open_connections was modified)
        self.config.mark_window_dirty();

        // Save all dirty configurations on exit
        if let Err(e) = self.config.save_all_if_dirty() {
            eprintln!(
                "Failed to save configuration on exit: {}",
                e.to_message(Language::English)
            );
        }
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Sync connection state and update tab name/color when connected successfully
        for (idx, tab) in self.tabs.iter_mut().enumerate() {
            if idx < self.prev_connected_states.len() {
                let is_connected = tab.state.connected.try_read().map(|v| *v).unwrap_or(false);
                if is_connected && !self.prev_connected_states[idx] {
                    // Just connected -> update name and color
                    if let Some(conn_idx) = tab.selected_connection {
                        if let Some(conn) = self.config.connections.get(conn_idx) {
                            tab.name = conn.name.clone();
                            tab.connected_color = conn.color.clone();
                        }
                    }
                }
                self.prev_connected_states[idx] = is_connected;
            }
        }

        ctx.set_visuals(super::theme_to_visuals(self.config.settings.theme));
        // Handle keyboard shortcuts
        shortcut_manager::handle_shortcuts(self, ctx);

        // Get the viewport information before the async block
        let viewport = ctx.input(|i| i.viewport().clone());
        let is_maximized = viewport.maximized.unwrap_or(false);
        let rect = viewport.outer_rect;

        // Update window state (marks as dirty, doesn't save immediately)
        if let Some(rect) = rect {
            if !self.config.window.maximized {
                self.config.update_window_position(rect.min.x, rect.min.y);
                self.config.update_window_size(rect.width(), rect.height());
            }

            if is_maximized != self.config.window.maximized {
                self.config.update_maximized(is_maximized);
            }
        }

        // Settings are saved on exit, no need to check here

        // Clear expired copy feedback
        self.copy_feedback_manager.update();

        // Render tab bar
        render_tab_bar(self, ctx);

        // Render command line panel (above status bar if shown)
        render_command_line_panel(self, ctx);

        // Render status bar first to ensure it's on top
        render_status_bar(self, ctx);

        // Update UI (delegated to panels module)
        render_top_panel(self, ctx);
        render_side_panel(self, ctx);
        render_central_panel(self, ctx);

        // Force continuous repaint while loading to ensure smooth UI updates
        let mut any_loading = false;
        let mut any_needs_repaint = false;
        for tab in &self.tabs {
            if tab.state.loading.try_read().map(|v| *v).unwrap_or(false) {
                any_loading = true;
            }
            if tab
                .state
                .needs_repaint
                .try_read()
                .map(|v| *v)
                .unwrap_or(false)
            {
                any_needs_repaint = true;
                // Clear the flag to prevent repeated repaints
                if let Ok(mut guard) = tab.state.needs_repaint.try_write() {
                    *guard = false;
                }
            }
        }
        if any_loading || any_needs_repaint {
            ctx.request_repaint();
            ctx.request_repaint_after(std::time::Duration::from_millis(UI_REPAINT_INTERVAL_MS));
        }
    }
}

impl RedisApp {

    // Getters for private fields (needed by panels)
    pub fn tabs(&self) -> &[RedisTab] {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut Vec<RedisTab> {
        &mut self.tabs
    }

    pub fn active_tab(&self) -> usize {
        self.active_tab
    }

    pub fn set_active_tab(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.active_tab = idx;
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn show_settings(&self) -> bool {
        self.show_settings
    }

    pub fn set_show_settings(&mut self, show: bool) {
        self.show_settings = show;
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn set_show_help(&mut self, show: bool) {
        self.show_help = show;
    }

    pub fn with_config(config: Config) -> Self {
        let global_language = if !config.settings.language.is_empty() {
            Language::file_name_to_lang(&config.settings.language)
        } else {
            Language::English
        };

        // Check if there are open connections before moving config
        let has_open_connections =
            !config.window.open_connections.is_empty() && config.settings.show_unclosed_connections;

        // Create initial tab
        let initial_tab = RedisTab::new(0, global_language);

        Self {
            tabs: vec![initial_tab],
            active_tab: 0,
            next_tab_id: 1,
            config,
            new_connection: NewConnectionWindowWindow::new(),
            show_settings: false,
            new_key_dialog: NewKeyDialog::new(),
            element_edit_dialog: ElementEditDialog::default(),
            global_language,
            prev_connected_states: vec![false],
            copy_feedback_manager: copy_feedback::CopyFeedbackManager::default(),
            show_open_connections_prompt: has_open_connections,
            shortcut_state: shortcut_manager::ShortcutManagerState::default(),
            scroll_to_tab: None,
            show_all_tabs_dropdown: false,
            ai_model_editor: AiModelEditor::default(),
            delete_connection_confirm: None,
            settings_expanded_section: None,
            show_help: false,
            help_selected_section: None,
        }
    }

    // Other public methods
    pub fn create_new_tab(&mut self) {
        let new_tab = RedisTab::new(self.next_tab_id, self.global_language);
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;
        self.prev_connected_states.push(false);
        self.scroll_to_tab = Some(self.active_tab);
    }

    pub fn create_tab_with_connection(
        &mut self,
        conn_idx: usize,
        conn: e_client_config::connection::RedisConnectionConfig,
    ) {
        // Check if duplicate connections are allowed
        if !self.config.settings.allow_duplicate_connections {
            // Check if this connection is already open in another tab
            if let Some(existing_tab_idx) = self.find_tab_with_connection(conn_idx) {
                // Switch to the existing tab instead of creating a new one
                self.switch_to_tab(existing_tab_idx);
                return;
            }
        }

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

        // Scroll to the new tab
        self.scroll_to_tab = Some(self.active_tab);

        // Auto-connect with preferred DB
        self.spawn_connect_with_initial_db(new_tab_idx);
    }

    pub fn connect_in_current_tab(
        &mut self,
        conn_idx: usize,
        conn: &e_client_config::connection::RedisConnectionConfig,
    ) {
        // Check if duplicate connections are allowed
        if !self.config.settings.allow_duplicate_connections {
            // Check if this connection is already open in another tab
            if let Some(existing_tab_idx) = self.find_tab_with_connection(conn_idx) {
                // Switch to the existing tab
                self.switch_to_tab(existing_tab_idx);
                self.show_open_connections_prompt = false;
                return;
            }
        }

        if let Some(tab) = self.get_active_tab_mut() {
            *tab.state.connection_param.blocking_write() = Some(conn.clone());
            tab.selected_connection = Some(conn_idx);

            let active_tab_idx = self.active_tab;
            self.load_connection_preferences(active_tab_idx);
            self.spawn_connect_with_initial_db(active_tab_idx);

            // Clear open connections list when connecting (same as "Connect All")
            self.config.clear_open_connections();
            self.show_open_connections_prompt = false;
        }
    }

    pub fn close_tab(&mut self, index: usize, ctx: &egui::Context) {
        use e_client_config::translations::{TranslationKey, tr};

        if self.tabs.len() <= 1 {
            // Close the last tab and exit the application
            if let Some(tab) = self.tabs.get(index) {
                tab.state.spawn_disconnect();
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
                tab.name = format!("{} {}", tr(TranslationKey::Tab, lang), tab.id);
                tab.connected_color = None;
                tab.selected_connection = None;
            }
        }
    }

    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
            self.scroll_to_tab = Some(index);
        }
    }

    /// Find a tab that is already connected to the given connection index
    pub fn find_tab_with_connection(&self, conn_idx: usize) -> Option<usize> {
        self.tabs.iter().position(|tab| {
            tab.selected_connection == Some(conn_idx) && *tab.state.connected.blocking_read()
        })
    }

    pub fn close_other_tabs(&mut self, keep_index: usize) {
        if self.tabs.len() <= 1 {
            return;
        }

        // Disconnect and remove all tabs except the one to keep
        let indices_to_remove: Vec<usize> = (0..self.tabs.len())
            .filter(|&idx| idx != keep_index)
            .collect();

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

    /// Remove duplicate and invalid tabs, keeping only the first occurrence of each connection.
    /// Also removes tabs with failed connections.
    pub fn remove_duplicate_and_invalid_tabs(&mut self, ctx: &egui::Context) {
        if self.tabs.len() <= 1 {
            return;
        }

        let mut seen_connections: std::collections::HashSet<Option<usize>> =
            std::collections::HashSet::new();
        let indices_to_remove: Vec<usize> = self
            .tabs
            .iter()
            .enumerate()
            .filter_map(|(idx, tab)| {
                // Check if disconnected (not connected)
                let connected = self.poll_bool(tab.state.connected.clone());
                if !connected {
                    return Some(idx); // Disconnected tab
                }
                if tab.selected_connection.is_none() {
                    Some(idx) // Invalid tab (no connection)
                } else if !seen_connections.insert(tab.selected_connection) {
                    Some(idx) // Duplicate connection
                } else {
                    None
                }
            })
            .collect();

        // Remove in reverse order to avoid index shifting
        for idx in indices_to_remove.into_iter().rev() {
            self.close_tab(idx, ctx);
        }
    }

    pub fn record_copy_success_with_id(&mut self, button_id: egui::Id) {
        self.copy_feedback_manager
            .record_copy_success_with_id(button_id);
    }

    pub fn record_action_success(&mut self, button_id: egui::Id) {
        self.copy_feedback_manager.record_action_success(button_id);
    }

    pub fn copy_button_text(&self, button_id: egui::Id, original_text: &str) -> String {
        self.copy_feedback_manager
            .copy_button_text(button_id, original_text, self.global_language)
    }

    pub fn copy_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        self.copy_feedback_manager.copy_button_text_color(button_id)
    }

    pub fn action_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        self.copy_feedback_manager
            .action_button_text_color(button_id)
    }

    fn load_connection_preferences(&mut self, tab_idx: usize) {
        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            if let Some(conn_idx) = tab.selected_connection {
                if let Some(conn) = self.config.connections.get(conn_idx) {
                    if let Some(pref) = self.config.get_connection_preference(&conn.name) {
                        tab.side_panel_width = pref.side_panel_width;
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

    pub fn get_hash_column_widths(&self) -> (u32, u32) {
        self.config.get_hash_column_widths()
    }

    pub fn save_hash_column_widths(&mut self, field_width: u32, value_width: u32) {
        self.config
            .update_hash_column_widths(field_width, value_width);
    }

    pub fn check_and_save_hash_column_widths(&mut self) {
        // Window config is now only saved on exit, no need to check here
    }

    // Active tab helpers
    pub fn get_active_tab(&self) -> Option<&RedisTab> {
        self.tabs.get(self.active_tab)
    }

    pub fn get_active_tab_mut(&mut self) -> Option<&mut RedisTab> {
        self.tabs.get_mut(self.active_tab)
    }

    // Hash field filter helpers
    fn poll_string(&self, lock: std::sync::Arc<tokio::sync::RwLock<String>>) -> String {
        lock.try_read().map(|v| v.clone()).unwrap_or_default()
    }

    fn update_string(&self, lock: std::sync::Arc<tokio::sync::RwLock<String>>, value: String) {
        if let Ok(mut v) = lock.try_write() {
            *v = value;
        }
    }

    pub fn poll_string_hash_field_filter(&self) -> String {
        if let Some(tab) = self.get_active_tab() {
            self.poll_string(tab.state.hash_field_filter.clone())
        } else {
            String::new()
        }
    }

    pub fn set_hash_field_filter(&self, value: String) {
        if let Some(tab) = self.get_active_tab() {
            self.update_string(tab.state.hash_field_filter.clone(), value);
        }
    }

    // Generic poll helpers for UI
    pub fn poll_language(&self, lock: std::sync::Arc<tokio::sync::RwLock<Language>>) -> Language {
        *lock.blocking_read()
    }

    pub fn poll_bool(&self, lock: std::sync::Arc<tokio::sync::RwLock<bool>>) -> bool {
        *lock.blocking_read()
    }

    pub fn poll_option_string(
        &self,
        lock: std::sync::Arc<tokio::sync::RwLock<Option<String>>>,
    ) -> Option<String> {
        lock.blocking_read().clone()
    }

    pub fn poll_i64(&self, lock: std::sync::Arc<tokio::sync::RwLock<i64>>) -> i64 {
        lock.try_read().map(|v| *v).unwrap_or(-2)
    }

    pub fn poll_option_value(
        &self,
        lock: std::sync::Arc<tokio::sync::RwLock<Option<crate::core::ValueData>>>,
    ) -> Option<crate::core::ValueData> {
        lock.blocking_read().clone()
    }

    pub fn poll_usize(&self, lock: std::sync::Arc<tokio::sync::RwLock<usize>>) -> usize {
        lock.try_read().map(|v| *v).unwrap_or(0)
    }

    pub fn poll_u32(&self, lock: std::sync::Arc<tokio::sync::RwLock<u32>>) -> u32 {
        lock.try_read().map(|v| *v).unwrap_or(0)
    }

    pub fn poll_vec_u32(&self, lock: std::sync::Arc<tokio::sync::RwLock<Vec<u32>>>) -> Vec<u32> {
        lock.try_read().map(|v| v.clone()).unwrap_or_default()
    }

    pub fn update_tab_side_panel_width(&mut self, tab_idx: usize, width: f32) {
        use e_client_basics::constants::MIN_SIDE_PANEL_WIDTH;

        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            // Round to reduce unnecessary updates
            let rounded_width = (width / 5.0).round() * 5.0;
            tab.side_panel_width = rounded_width.max(MIN_SIDE_PANEL_WIDTH);

            // Sync width to all other tabs with the same connection
            if let Some(conn_idx) = tab.selected_connection {
                if let Some(conn) = self.config.connections.get(conn_idx) {
                    let name = &conn.name;
                    for (idx, t) in self.tabs.iter_mut().enumerate() {
                        if idx != tab_idx {
                            if let Some(c_idx) = t.selected_connection {
                                if let Some(c) = self.config.connections.get(c_idx) {
                                    if c.name == *name {
                                        t.side_panel_width = rounded_width;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_language(&mut self, lang: Language) {
        use e_client_config::translations::{TranslationKey, tr};
        self.global_language = lang;
        // Update all tabs' language and names
        for tab in self.tabs.iter_mut() {
            *tab.state.language.blocking_write() = lang;
            // Update tab name if it's the default name
            if tab.name.starts_with("Tab ") || tab.name.starts_with("标签页") {
                tab.name = format!("{}{}", tr(TranslationKey::Tab, lang), tab.id);
            }
        }
    }
}

impl Drop for RedisApp {
    fn drop(&mut self) {
        // Disconnect all tabs when app is closed (immediate, no async wait)
        for tab in &self.tabs {
            tab.state.redis_client.disconnect_sync();
        }
        e_client_logging::log_app_shutdown();
    }
}
