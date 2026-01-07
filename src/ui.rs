use crate::app_state::{AppState, Language};
use crate::config::Config;
use crate::redis_client::ValueData;
use crate::translations::{tr, tr_fmt};
use eframe::egui;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RedisApp {
    state: AppState,
    config: Config,
    selected_connection: Option<usize>,
    command_input_buffer: String,
    key_filter_input: String,
    show_new_connection_dialog: bool,
    new_connection_name: String,
    new_connection_url: String,
    error_message: String,
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
            selected_connection: None,
            command_input_buffer: String::new(),
            key_filter_input: crate::constants::DEFAULT_KEY_FILTER.to_string(),
            show_new_connection_dialog: false,
            new_connection_name: String::new(),
            new_connection_url: crate::constants::DEFAULT_REDIS_URL.to_string(),
            error_message: String::new(),
        }
    }
}

impl Default for RedisApp {
    fn default() -> Self {
        let config = Config::load().unwrap_or_default();
        Self::with_config(config)
    }
}

impl eframe::App for RedisApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        configure_fonts(ctx);
        ctx.request_repaint();

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

        // Update UI
        self.render_top_panel(ctx);
        self.render_side_panel(ctx);
        self.render_central_panel(ctx);
    }
}

// Function to configure fonts for Chinese characters
fn configure_fonts(ctx: &egui::Context) {
    use egui::FontFamily;

    let mut fonts = egui::FontDefinitions::default();

    // Add the Songti.ttc font for Chinese characters
    fonts.font_data.insert("songti".to_owned(), {
        let font_data = std::fs::read("/System/Library/Fonts/Supplemental/Songti.ttc")
            .expect("Failed to read Songti font file");
        egui::FontData::from_owned(font_data).into()
    });

    // Use the Songti font for proportional text
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "songti".to_owned());

    // Use the Songti font for monospace text
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "songti".to_owned());

    ctx.set_fonts(fonts);
}

impl RedisApp {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        let current_lang = self.poll_language(self.state.language.clone());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("connection_url", current_lang));

                // 连接下拉框
                let selected_name = self
                    .selected_connection
                    .and_then(|idx| self.config.connections.get(idx))
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| tr("select_connection", current_lang).to_string());

                egui::ComboBox::from_id_salt("connection_select")
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (idx, conn) in self.config.connections.iter().enumerate() {
                            let is_selected = self.selected_connection == Some(idx);
                            if ui.selectable_label(is_selected, &conn.name).clicked() {
                                self.selected_connection = Some(idx);
                            }
                        }
                    });

                // 新增连接按钮
                if ui
                    .button(format!("+ {}", tr("new_connection", current_lang)))
                    .clicked()
                {
                    self.show_new_connection_dialog = true;
                    self.new_connection_name.clear();
                    self.new_connection_url = crate::constants::DEFAULT_REDIS_URL.to_string();
                    self.error_message.clear();
                }
                ui.separator();

                let connected = self.poll_bool(self.state.connected.clone());

                if connected {
                    if ui.button(tr("disconnect", current_lang)).clicked() {
                        self.state.spawn_disconnect();
                    }

                    ui.separator();
                    ui.label(tr("database", current_lang));

                    let current_db = self.poll_u32(self.state.current_db.clone());
                    let databases = self.poll_vec_u32(self.state.databases.clone());

                    egui::ComboBox::from_id_salt("db_select")
                        .selected_text(format!("DB {}", current_db))
                        .show_ui(ui, |ui| {
                            for db in databases {
                                if ui
                                    .selectable_label(current_db == db, format!("DB {}", db))
                                    .clicked()
                                {
                                    self.state.spawn_select_db(db);
                                }
                            }
                        });
                } else {
                    if ui.button(tr("connect", current_lang)).clicked() {
                        if let Some(idx) = self.selected_connection {
                            if let Some(conn) = self.config.connections.get(idx) {
                                self.update_string(
                                    self.state.connection_url.clone(),
                                    conn.url.clone(),
                                );
                                self.state.spawn_connect();
                            }
                        } else {
                            self.error_message =
                                tr("please_select_connection", current_lang).to_string();
                        }
                    }
                }

                let loading = self.poll_bool(self.state.loading.clone());
                if loading {
                    ui.spinner();
                }
            });

            // Right side - Language selector
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let lang = self.poll_language(self.state.language.clone());
                egui::ComboBox::from_id_salt("lang_select")
                    .selected_text(match lang {
                        Language::English => tr("english", lang),
                        Language::Chinese => tr("chinese", lang),
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                matches!(lang, Language::English),
                                tr("english", lang),
                            )
                            .clicked()
                        {
                            self.update_language(Language::English);
                        }
                        if ui
                            .selectable_label(
                                matches!(lang, Language::Chinese),
                                tr("chinese", lang),
                            )
                            .clicked()
                        {
                            self.update_language(Language::Chinese);
                        }
                    });
                ui.label(tr("language", lang));
            });
        });

        // 新建连接对话框
        if self.show_new_connection_dialog {
            self.render_new_connection_dialog(ctx, current_lang);
        }
    }

    fn render_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .min_width(250.0)
            .show(ctx, |ui| {
                let current_lang = self.poll_language(self.state.language.clone());

                ui.vertical(|ui| {
                    ui.heading(tr("keys", current_lang));

                    ui.horizontal(|ui| {
                        ui.label(tr("filter", current_lang));
                        if ui
                            .text_edit_singleline(&mut self.key_filter_input)
                            .changed()
                        {
                            self.update_string(
                                self.state.key_filter.clone(),
                                self.key_filter_input.clone(),
                            );
                            self.state.spawn_load_keys();
                        }
                    });

                    let keys = self.poll_vec_string(self.state.keys.clone());
                    let selected_key = self.poll_option_string(self.state.selected_key.clone());

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for key in keys {
                            let is_selected = selected_key.as_ref() == Some(&key);
                            if ui.selectable_label(is_selected, &key).clicked() {
                                *self.state.selected_key.blocking_write() = Some(key.clone());
                                self.state.spawn_load_value(key);
                            }
                        }
                    });
                });
            });
    }

    fn render_new_connection_dialog(&mut self, ctx: &egui::Context, current_lang: Language) {
        egui::Window::new(tr("new_connection_dialog", current_lang))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(tr("connection_name", current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label(tr("connection_address", current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_url);
                    });

                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.error_message);
                    }

                    ui.horizontal(|ui| {
                        if ui.button(tr("save", current_lang)).clicked() {
                            if self.new_connection_name.trim().is_empty() {
                                self.error_message =
                                    tr("please_enter_connection_name", current_lang).to_string();
                            } else if self.new_connection_url.trim().is_empty() {
                                self.error_message =
                                    tr("please_enter_connection_address", current_lang).to_string();
                            } else {
                                match self.config.add_connection(
                                    self.new_connection_name.clone(),
                                    self.new_connection_url.clone(),
                                ) {
                                    Ok(_) => {
                                        self.show_new_connection_dialog = false;
                                        self.error_message.clear();
                                    }
                                    Err(e) => {
                                        self.error_message = e.to_message(current_lang);
                                    }
                                }
                            }
                        }

                        if ui.button(tr("cancel", current_lang)).clicked() {
                            self.show_new_connection_dialog = false;
                            self.error_message.clear();
                        }
                    });
                });
            });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        let current_lang = self.poll_language(self.state.language.clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("command_label", current_lang));
                let response = ui.text_edit_singleline(&mut self.command_input_buffer);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let cmd = self.command_input_buffer.clone();
                    self.state.spawn_execute_command(cmd);
                    self.command_input_buffer.clear();
                }

                if ui.button(tr("execute", current_lang)).clicked() {
                    let cmd = self.command_input_buffer.clone();
                    self.state.spawn_execute_command(cmd);
                    self.command_input_buffer.clear();
                }
            });

            ui.separator();

            let command_output = self.poll_string(self.state.command_output.clone());
            if !command_output.is_empty() {
                ui.group(|ui| {
                    ui.label(tr("output", current_lang));
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.label(&command_output);
                        });
                });
                ui.separator();
            }

            let selected_key = self.poll_option_string(self.state.selected_key.clone());
            if let Some(key) = selected_key {
                ui.heading(tr_fmt("key_heading", current_lang, &[&key]));

                let value = self.poll_option_value(self.state.key_value.clone());

                if let Some(val) = value {
                    match val {
                        ValueData::String(s) => {
                            ui.label(tr("type_string", current_lang));
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.text_edit_multiline(&mut s.as_str());
                            });
                        }
                        ValueData::List { len, items } => {
                            ui.label(tr_fmt("type_list", current_lang, &[&len.to_string()]));

                            if items.is_empty() && len > 0 {
                                if ui.button(tr("load_first_100", current_lang)).clicked() {
                                    self.state.spawn_load_list_range(key.clone(), 0, 99);
                                }
                            } else {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for (idx, item) in items.iter().enumerate() {
                                        ui.label(format!("[{}] {}", idx, item));
                                    }
                                });
                            }
                        }
                        ValueData::Hash { len, fields } => {
                            ui.label(tr_fmt("type_hash", current_lang, &[&len.to_string()]));

                            if fields.is_empty() && len > 0 {
                                if ui.button(tr("load_fields", current_lang)).clicked() {
                                    self.state.spawn_load_hash_fields(key.clone());
                                }
                            } else {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for (field, value) in fields.iter() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{}: {}", field, value));
                                        });
                                    }
                                });
                            }
                        }
                        ValueData::Set { len, .. } => {
                            ui.label(tr_fmt("type_set", current_lang, &[&len.to_string()]));
                        }
                        ValueData::ZSet { len, .. } => {
                            ui.label(tr_fmt("type_zset", current_lang, &[&len.to_string()]));
                        }
                        ValueData::None => {
                            ui.label(tr("key_not_exist", current_lang));
                        }
                    }
                }
            } else {
                ui.label(tr("select_key_prompt", current_lang));
            }
        });
    }

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
