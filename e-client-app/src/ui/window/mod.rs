use crate::core::app_state::AppState;
use crate::core::redis_client::ValueData;
use crate::ui::window::new_connection_window::NewConnectionWindowWindow;
use e_client_config::config::Config;
use e_client_config::constants::DEFAULT_KEY_FILTER;
use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

mod new_connection_window;
mod panels;

pub struct RedisApp {
    state: AppState,
    config: Config,
    // error_message: String,
    selected_connection: Option<usize>,
    command_input_buffer: String,
    key_filter_input: String,
    new_connection: NewConnectionWindowWindow,
}

impl eframe::App for RedisApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // ctx.request_repaint(); DO NOT repaint

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
        {
            let current_lang = self.state.language.blocking_read();
            let current_lang_str = current_lang.to_string();
            if self.config.language != current_lang_str {
                if let Err(e) = self.config.update_language(&current_lang_str) {
                    eprintln!(
                        "Failed to update language: {}",
                        e.to_message(Language::English)
                    );
                }
            }
        } // current_lang is dropped here, releasing the read lock

        // Update UI (delegated to panels module)
        panels::render_top_panel(self, ctx);
        panels::render_side_panel(self, ctx);
        panels::render_central_panel(self, ctx);
    }
}

impl RedisApp {
    pub fn with_config(config: Config) -> Self {
        let mut state = AppState::new();

        // Initialize language from config
        if !config.language.is_empty() {
            let lang = if config.language == "zh" {
                Language::Chinese
            } else {
                Language::English
            };
            state.language = Arc::new(RwLock::new(lang));
        }

        Self {
            state,
            config,
            selected_connection: None, // todo - remember the last opened
            command_input_buffer: String::new(),
            key_filter_input: DEFAULT_KEY_FILTER.to_string(),
            new_connection: NewConnectionWindowWindow::new(),
            // error_message: String::new(),
        }
    }
    // render_* methods moved to panels.rs

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

    fn update_language(&self, lang: Language) {
        *self.state.language.blocking_write() = lang;
    }
}
