use crate::core::AppState;
use crate::core::ValueData;
use crate::ui::window::new_connection_window::NewConnectionWindowWindow;
use e_client_basics::constants::{
    DEFAULT_SIDE_PANEL_WIDTH, MAX_SIDE_PANEL_WIDTH, MIN_SIDE_PANEL_WIDTH, UI_REPAINT_INTERVAL_MS,
};
use e_client_config::config::ai_config::AiModel;
use e_client_config::config::Config;
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

// Re-export panel functions for convenient access
pub use panels::{
    render_central_panel, render_command_line_panel, render_side_panel, render_status_bar,
    render_tab_bar, render_top_panel,
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
    pub command_line_panel: CommandLinePanel, // Per-tab command line panel state
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
            command_line_panel: CommandLinePanel::default(),
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

pub struct NewKeyDialog {
    pub show: bool,
    pub key_name: String,
    pub key_type: String,
    pub value: String,
    pub ttl: String,
    pub error_message: String,
}

impl NewKeyDialog {
    pub fn new() -> Self {
        Self {
            show: false,
            key_name: String::new(),
            key_type: "string".to_string(),
            value: String::new(),
            ttl: "-1".to_string(),
            error_message: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.key_name.clear();
        self.key_type = "string".to_string();
        self.value.clear();
        self.ttl = "-1".to_string();
        self.error_message.clear();
    }
}

pub struct ElementEditDialog {
    pub show: bool,
    pub key: String,
    pub field: String, // For hash: field name, for list/set/zset: index
    pub value: String,
    pub original_value: String,
    pub key_type: String,  // hash, list, set, zset
    pub just_opened: bool, // Track if dialog just opened for auto-formatting
}

/// Command line panel state
pub struct CommandLinePanel {
    pub show: bool,
    pub input: String,
    pub history: Vec<(String, String)>, // (command, result)
    pub scroll_to_bottom: bool,
    pub history_index: Option<usize>,
    pub saved_input: String,
}

impl Default for CommandLinePanel {
    fn default() -> Self {
        Self {
            show: false,
            input: String::new(),
            history: Vec::new(),
            scroll_to_bottom: false,
            history_index: None,
            saved_input: String::new(),
        }
    }
}

impl Default for ElementEditDialog {
    fn default() -> Self {
        Self {
            show: false,
            key: String::new(),
            field: String::new(),
            value: String::new(),
            original_value: String::new(),
            key_type: String::new(),
            just_opened: false,
        }
    }
}

/// AI Model Editor state - manages the UI for adding/editing AI models
pub struct AiModelEditor {
    pub show: bool,
    pub editing_model_id: Option<String>,
    // Editable fields
    pub name: String,
    pub api_key: String,
    pub url: String,
    pub model_id: String,
    pub temperature: f32,
    /// Whether the models list section is collapsed
    pub models_collapsed: bool,
}

impl Default for AiModelEditor {
    fn default() -> Self {
        Self {
            show: false,
            editing_model_id: None,
            name: String::new(),
            api_key: String::new(),
            url: String::new(),
            model_id: String::new(),
            temperature: 0.7,
            models_collapsed: true,
        }
    }
}

impl AiModelEditor {
    /// Open editor for creating a new model
    pub fn open_for_new(&mut self) {
        self.show = true;
        self.editing_model_id = None;
        self.name.clear();
        self.api_key.clear();
        self.url.clear();
        self.model_id.clear();
        self.temperature = 0.7;
    }

    /// Open editor for editing an existing model
    pub fn open_for_edit(&mut self, model: &AiModel) {
        self.show = true;
        self.editing_model_id = Some(model.id.clone());
        self.name = model.name.clone();
        self.api_key = model.api_key.clone().unwrap_or_default();
        self.url = model.url.clone();
        self.model_id = model.model_id.clone();
        self.temperature = model.temperature;
    }

    /// Close the editor
    pub fn close(&mut self) {
        self.show = false;
        self.editing_model_id = None;
    }

    /// Check if currently editing an existing model
    pub fn is_editing(&self) -> bool {
        self.editing_model_id.is_some()
    }
}

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
    // Copy button feedback: (success_time, failure_time)
    copy_feedback: (Option<Instant>, Option<Instant>),
    // Track which button was last clicked for per-button feedback
    last_copy_button_id: Option<egui::Id>,
    // Track if open connections prompt should be shown
    show_open_connections_prompt: bool,
    // Shortcut editing state
    editing_shortcut: Option<String>, // action name being edited
    shortcut_input_buffer: String,    // buffer for capturing new shortcut
    shortcut_conflict_warning: Option<String>, // conflict warning message
    // Track if tab bar should scroll to show active tab
    scroll_to_tab: Option<usize>,
    // Track if all tabs dropdown should be shown (triggered by shortcut)
    pub show_all_tabs_dropdown: bool,
    // AI model editor
    pub ai_model_editor: AiModelEditor,
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
        // Open connections prompt is now shown in the welcome page

        // Handle keyboard shortcuts using custom configuration
        // Only process shortcuts when settings window is not open
        if !self.show_settings {
            use crate::ui::window::panels::top_panel::SUPPORTED_KEYS;
            use e_client_config::config::shortcuts::{ParsedShortcut, ShortcutAction};

            // Collect current input state
            let (modifiers, pressed_keys): (egui::Modifiers, Vec<egui::Key>) = ctx.input(|i| {
                let keys: Vec<egui::Key> = SUPPORTED_KEYS
                    .into_iter()
                    .filter(|k| i.key_pressed(*k))
                    .collect();
                (i.modifiers, keys)
            });

            // Check each configured shortcut
            let is_macos = cfg!(target_os = "macos");
            for (action, _) in ShortcutAction::all_actions() {
                let binding = self.config.settings.shortcuts.get_binding(&action);
                if let Some(parsed) = ParsedShortcut::parse(&binding) {
                    for key in &pressed_keys {
                        let key_str = format!("{:?}", key);
                        let mod_pressed: bool =
                            parsed.is_mod_pressed(is_macos, modifiers.ctrl, modifiers.command);
                        let alt_match = parsed.alt == modifiers.alt;
                        let shift_match = parsed.shift == modifiers.shift;
                        let key_match = parsed.key_matches(&key_str);

                        if mod_pressed && alt_match && shift_match && key_match {
                            match action {
                                ShortcutAction::NewTab => self.create_new_tab(),
                                ShortcutAction::CloseTab => {
                                    let idx = self.active_tab;
                                    self.close_tab(idx, ctx);
                                }
                                ShortcutAction::RefreshKey => {
                                    if let Some(tab) = self.tabs.get(self.active_tab) {
                                        if let Some(key) =
                                            tab.state.selected_key.blocking_read().clone()
                                        {
                                            tab.state.spawn_load_value(key);
                                        }
                                    }
                                }
                                ShortcutAction::FocusFilter => {
                                    ctx.memory_mut(|mem| {
                                        mem.request_focus(egui::Id::new("key_filter_input"));
                                    });
                                }
                                ShortcutAction::CloseSettings => {
                                    // CloseSettings is handled in top_panel.rs when settings window is open
                                    // This case should not be reached here since shortcuts are disabled when settings is open
                                }
                                ShortcutAction::OpenSettings => {
                                    self.show_settings = true;
                                }
                                ShortcutAction::ToggleCommandLine => {
                                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                                        if tab.command_line_panel.show {
                                            // Already open, focus the input
                                            ctx.memory_mut(|mem| {
                                                mem.request_focus(egui::Id::new(
                                                    "command_line_input",
                                                ));
                                            });
                                        } else {
                                            // Open the panel
                                            tab.command_line_panel.show = true;
                                            tab.command_line_panel.scroll_to_bottom = true;
                                        }
                                    }
                                }
                                ShortcutAction::CloseCommandLine => {
                                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                                        tab.command_line_panel.show = false;
                                    }
                                }
                                ShortcutAction::SwitchToTab1
                                | ShortcutAction::SwitchToTab2
                                | ShortcutAction::SwitchToTab3
                                | ShortcutAction::SwitchToTab4
                                | ShortcutAction::SwitchToTab5
                                | ShortcutAction::SwitchToTab6
                                | ShortcutAction::SwitchToTab7
                                | ShortcutAction::SwitchToTab8
                                | ShortcutAction::SwitchToTab9 => {
                                    if let Some(tab_idx) = action.tab_index() {
                                        if tab_idx < self.tabs.len() {
                                            self.active_tab = tab_idx;
                                            self.scroll_to_tab = Some(tab_idx);
                                        }
                                    }
                                }
                                ShortcutAction::SwitchToLastTab => {
                                    if !self.tabs.is_empty() {
                                        self.active_tab = self.tabs.len() - 1;
                                        self.scroll_to_tab = Some(self.tabs.len() - 1);
                                    }
                                }
                                ShortcutAction::RemoveDuplicateAndInvalidTabs => {
                                    self.remove_duplicate_and_invalid_tabs(ctx);
                                }
                            }
                        }
                    }
                }
            }
        }

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

        // Check and save hash column widths if debounce time has passed
        self.check_and_save_hash_column_widths();

        // Clear expired copy feedback
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;
        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let now = std::time::Instant::now();

        if let Some(time) = self.copy_feedback.0 {
            if now.duration_since(time) >= duration {
                self.copy_feedback.0 = None;
            }
        }

        if let Some(time) = self.copy_feedback.1 {
            if now.duration_since(time) >= duration {
                self.copy_feedback.1 = None;
            }
        }

        // Handle language updates (marks as dirty, doesn't save immediately)
        if let Some(active_tab) = self.get_active_tab() {
            let current_lang = active_tab.state.language.blocking_read();
            let current_lang_str = current_lang.to_file_string();
            drop(current_lang); // Drop the lock before borrowing config mutably

            if self.config.settings.language != current_lang_str {
                self.config.update_language(&current_lang_str);
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

        // Render command line panel (above status bar if shown)
        render_command_line_panel(self, ctx);

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

        // Check if there are open connections before moving config
        // Only show open connections prompt if the setting is enabled
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
            copy_feedback: (None, None),
            last_copy_button_id: None,
            show_open_connections_prompt: has_open_connections,
            editing_shortcut: None,
            shortcut_input_buffer: String::new(),
            shortcut_conflict_warning: None,
            scroll_to_tab: None,
            show_all_tabs_dropdown: false,
            ai_model_editor: AiModelEditor::default(),
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
        // Scroll to the new tab
        self.scroll_to_tab = Some(self.active_tab);
    }

    // Record copy operation feedback
    pub fn record_copy_success(&mut self) {
        self.copy_feedback.0 = Some(std::time::Instant::now());
        self.copy_feedback.1 = None;
    }

    pub fn record_copy_failure(&mut self) {
        self.copy_feedback.1 = Some(std::time::Instant::now());
        self.copy_feedback.0 = None;
    }

    // Record copy success with button id for per-button feedback
    pub fn record_copy_success_with_id(&mut self, button_id: egui::Id) {
        let now = std::time::Instant::now();
        self.copy_feedback.0 = Some(now);
        self.last_copy_button_id = Some(button_id);
        self.copy_feedback.1 = None;
    }

    // Record copy failure with button id for per-button feedback
    pub fn record_copy_failure_with_id(&mut self, button_id: egui::Id) {
        let now = std::time::Instant::now();
        self.copy_feedback.1 = Some(now);
        self.last_copy_button_id = Some(button_id);
        self.copy_feedback.0 = None;
    }

    // Get feedback color for a specific copy button
    pub fn copy_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;
        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (success_time, failure_time) = self.copy_feedback;
        let now = std::time::Instant::now();

        // Only show feedback if this is the button that was clicked
        if self.last_copy_button_id == Some(button_id) {
            if let Some(time) = success_time {
                if now.duration_since(time) < duration {
                    return egui::Color32::from_rgb(50, 200, 50); // Green
                }
            }

            if let Some(time) = failure_time {
                if now.duration_since(time) < duration {
                    return egui::Color32::from_rgb(220, 50, 50); // Red
                }
            }
        }

        egui::Color32::BLACK
    }

    // Get feedback text for a specific copy button
    pub fn copy_button_text(&self, button_id: egui::Id, original_text: &str) -> String {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;
        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (success_time, failure_time) = self.copy_feedback;
        let now = std::time::Instant::now();

        // Only show feedback if this is the button that was clicked
        if self.last_copy_button_id == Some(button_id) {
            if let Some(time) = success_time {
                if now.duration_since(time) < duration {
                    return tr(keys::COPY_SUCCESS, self.global_language).to_string();
                }
            }

            if let Some(time) = failure_time {
                if now.duration_since(time) < duration {
                    return tr(keys::COPY_FAILED, self.global_language).to_string();
                }
            }
        }

        original_text.to_string()
    }

    pub fn create_tab_with_connection(&mut self, conn_idx: usize, conn: RedisConnectionConfig) {
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

    pub fn close_tab(&mut self, index: usize, ctx: &egui::Context) {
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
                tab.name = format!("{} {}", tr(keys::TAB, lang), tab.id);
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
    /// Returns the tab index if found, None otherwise
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

    /// Remove duplicate and invalid tabs, keeping only the first occurrence of each connection
    pub fn remove_duplicate_and_invalid_tabs(&mut self, ctx: &egui::Context) {
        if self.tabs.len() <= 1 {
            return;
        }

        let mut seen_connections: std::collections::HashSet<Option<usize>> =
            std::collections::HashSet::new();
        let mut indices_to_remove: Vec<usize> = Vec::new();

        // First pass: identify tabs to remove
        for (idx, tab) in self.tabs.iter().enumerate() {
            if tab.selected_connection.is_none() {
                // Invalid tab (no connection)
                indices_to_remove.push(idx);
            } else {
                // Check for duplicates
                let conn_idx = tab.selected_connection;
                if !seen_connections.insert(conn_idx) {
                    // Duplicate connection, mark for removal
                    indices_to_remove.push(idx);
                }
            }
        }

        // Remove in reverse order to avoid index shifting
        for idx in indices_to_remove.into_iter().rev() {
            if idx < self.tabs.len() {
                self.close_tab(idx, ctx);
            }
        }
    }

    // Helper methods for compatibility with panels
    fn poll_bool(&self, lock: Arc<RwLock<bool>>) -> bool {
        lock.try_read().map(|v| *v).unwrap_or(false)
    }

    fn poll_u32(&self, lock: Arc<RwLock<u32>>) -> u32 {
        lock.try_read().map(|v| *v).unwrap_or(0)
    }

    fn poll_i64(&self, lock: Arc<RwLock<i64>>) -> i64 {
        lock.try_read().map(|v| *v).unwrap_or(-2)
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

    fn poll_string_hash_field_filter(&self) -> String {
        if let Some(tab) = self.get_active_tab() {
            self.poll_string(tab.state.hash_field_filter.clone())
        } else {
            String::new()
        }
    }

    fn set_hash_field_filter(&self, value: String) {
        if let Some(tab) = self.get_active_tab() {
            self.update_string(tab.state.hash_field_filter.clone(), value);
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

        let rounded_width = (width.max(MIN_SIDE_PANEL_WIDTH).min(MAX_SIDE_PANEL_WIDTH)).round();

        // Update the current tab
        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            tab.side_panel_width = rounded_width;
        }

        // If connected, sync width to all tabs with the same connection
        if let Some(ref name) = conn_name {
            // Update config (marks as dirty, doesn't save immediately)
            self.config
                .update_side_panel_width_for_connection(name, rounded_width);

            // Sync width to all other tabs with the same connection
            for (idx, tab) in self.tabs.iter_mut().enumerate() {
                if idx != tab_idx {
                    if let Some(conn_idx) = tab.selected_connection {
                        if let Some(conn) = self.config.connections.get(conn_idx) {
                            if conn.name == *name {
                                tab.side_panel_width = rounded_width;
                            }
                        }
                    }
                }
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
}

impl Drop for RedisApp {
    fn drop(&mut self) {
        // Disconnect all tabs when app is closed (immediate, no async wait)
        for tab in &self.tabs {
            tab.state.redis_client.disconnect_sync();
        }
    }
}
