use crate::core::app_state::AppState;
use crate::core::redis_client::ValueData;
use crate::ui::window::new_connection_window::NewConnectionWindowWindow;
use e_client_config::config::Config;
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::constants::DEFAULT_KEY_FILTER;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

mod new_connection_window;
pub mod panels;

/// Represents a single Redis connection tab
pub struct RedisTab {
    pub id: usize,
    pub name: String,
    pub state: AppState,
    pub selected_connection: Option<usize>,
    pub connected_color: Option<String>, // Connection color (hex) after successful connection
    pub command_input_buffer: String,
    pub key_filter_input: String,
}

impl RedisTab {
    pub fn new(id: usize, language: Language) -> Self {
        let mut state = AppState::new();
        state.language = Arc::new(RwLock::new(language));

        Self {
            id,
            name: format!("Tab {}", id),
            state,
            selected_connection: None,
            connected_color: None,
            command_input_buffer: String::new(),
            key_filter_input: DEFAULT_KEY_FILTER.to_string(),
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

        // Render tab bar
        panels::render_tab_bar(self, ctx);

        // Update UI (delegated to panels module)
        panels::render_top_panel(self, ctx);
        panels::render_side_panel(self, ctx);
        panels::render_central_panel(self, ctx);
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
    }

    pub fn create_tab_with_connection(&mut self, conn_idx: usize, conn: RedisConnectionConfig) {
        let new_tab =
            RedisTab::with_connection(self.next_tab_id, conn_idx, conn, self.global_language);
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;

        // Auto-connect
        if let Some(tab) = self.get_active_tab() {
            tab.state.spawn_connect();
        }
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

            // Adjust active tab index
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            } else if self.active_tab > index {
                self.active_tab -= 1;
            }
        }
    }

    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    // Helper methods for compatibility with panels
    fn poll_bool(&self, lock: Arc<RwLock<bool>>) -> bool {
        lock.try_read().map(|v| *v).unwrap_or(false)
    }

    fn poll_u32(&self, lock: Arc<RwLock<u32>>) -> u32 {
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
        if let Some(tab) = self.get_active_tab() {
            *tab.state.language.blocking_write() = lang;
        }
    }
}
