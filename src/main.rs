mod app_state;
mod redis_client;
mod config;
mod translations;

use app_state::AppState;
use config::Config;
use eframe::egui;
use redis_client::ValueData;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{app_state::Language, translations::tr};

fn main() -> Result<(), eframe::Error> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = runtime.enter();

    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Redis Client"),
        ..Default::default()
    };

    eframe::run_native(
        "Redis Client",
        options,
        Box::new(|_cc| Ok(Box::new(RedisApp::default()))),
    )
}

struct RedisApp {
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

impl Default for RedisApp {
    fn default() -> Self {
        let config = Config::load().unwrap_or_default();
        Self {
            state: AppState::new(),
            config,
            selected_connection: None,
            command_input_buffer: String::new(),
            key_filter_input: "*".to_string(),
            show_new_connection_dialog: false,
            new_connection_name: String::new(),
            new_connection_url: "redis://127.0.0.1:6379".to_string(),
            error_message: String::new(),
        }
    }
}

impl eframe::App for RedisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        configure_fonts(ctx);
        ctx.request_repaint();

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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("连接:");
                
                // 连接下拉框
                let selected_name = self.selected_connection
                    .and_then(|idx| self.config.connections.get(idx))
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "选择连接".to_string());
                
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
                if ui.button("+ 新建").clicked() {
                    self.show_new_connection_dialog = true;
                    self.new_connection_name.clear();
                    self.new_connection_url = "redis://127.0.0.1:6379".to_string();
                    self.error_message.clear();
                }
                
                ui.separator();
                
                let connected = self.poll_bool(self.state.connected.clone());
                
                if connected {
                    if ui.button("断开").clicked() {
                        self.state.spawn_disconnect();
                    }
                    
                    ui.separator();
                    ui.label("数据库:");
                    
                    let current_db = self.poll_u32(self.state.current_db.clone());
                    let databases = self.poll_vec_u32(self.state.databases.clone());
                    
                    egui::ComboBox::from_id_salt("db_select")
                        .selected_text(format!("DB {}", current_db))
                        .show_ui(ui, |ui| {
                            for db in databases {
                                if ui.selectable_label(current_db == db, format!("DB {}", db)).clicked() {
                                    self.state.spawn_select_db(db);
                                }
                            }
                        });
                } else {
                    if ui.button("连接").clicked() {
                        if let Some(idx) = self.selected_connection {
                            if let Some(conn) = self.config.connections.get(idx) {
                                self.update_string(
                                    self.state.connection_url.clone(),
                                    conn.url.clone(),
                                );
                                self.state.spawn_connect();
                            }
                        } else {
                            self.error_message = "请先选择一个连接".to_string();
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
                let current_lang = self.poll_language(self.state.language.clone());
                egui::ComboBox::from_id_salt("lang_select")
                    .selected_text(match current_lang {
                        Language::English => tr("english", current_lang),
                        Language::Chinese => tr("chinese", current_lang),
                    })
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(
                            matches!(current_lang, Language::English),
                            tr("english", current_lang),
                        ).clicked() {
                            self.update_language(Language::English);
                        }
                        if ui.selectable_label(
                            matches!(current_lang, Language::Chinese),
                            tr("chinese", current_lang),
                        ).clicked() {
                            self.update_language(Language::Chinese);
                        }
                    });
                ui.label(tr("language", current_lang));
            });
        });
        
        // 新建连接对话框
        if self.show_new_connection_dialog {
            self.render_new_connection_dialog(ctx);
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
    
    fn render_new_connection_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("新建 Redis 连接")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("连接名称:");
                        ui.text_edit_singleline(&mut self.new_connection_name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("连接地址:");
                        ui.text_edit_singleline(&mut self.new_connection_url);
                    });
                    
                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.error_message);
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("保存").clicked() {
                            if self.new_connection_name.trim().is_empty() {
                                self.error_message = "请输入连接名称".to_string();
                            } else if self.new_connection_url.trim().is_empty() {
                                self.error_message = "请输入连接地址".to_string();
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
                                        self.error_message = e;
                                    }
                                }
                            }
                        }
                        
                        if ui.button("取消").clicked() {
                            self.show_new_connection_dialog = false;
                            self.error_message.clear();
                        }
                    });
                });
            });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("命令:");
                let response = ui.text_edit_singleline(&mut self.command_input_buffer);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let cmd = self.command_input_buffer.clone();
                    self.state.spawn_execute_command(cmd);
                    self.command_input_buffer.clear();
                }

                if ui.button("执行").clicked() {
                    let cmd = self.command_input_buffer.clone();
                    self.state.spawn_execute_command(cmd);
                    self.command_input_buffer.clear();
                }
            });

            ui.separator();

            let command_output = self.poll_string(self.state.command_output.clone());
            if !command_output.is_empty() {
                ui.group(|ui| {
                    ui.label("输出:");
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
                ui.heading(format!("Key: {}", key));

                let value = self.poll_option_value(self.state.key_value.clone());

                if let Some(val) = value {
                    match val {
                        ValueData::String(s) => {
                            ui.label("类型: String");
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.text_edit_multiline(&mut s.as_str());
                            });
                        }
                        ValueData::List { len, items } => {
                            ui.label(format!("类型: List (长度: {})", len));

                            if items.is_empty() && len > 0 {
                                if ui.button("加载前100项").clicked() {
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
                            ui.label(format!("类型: Hash (字段数: {})", len));

                            if fields.is_empty() && len > 0 {
                                if ui.button("加载字段").clicked() {
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
                            ui.label(format!("类型: Set (成员数: {})", len));
                        }
                        ValueData::ZSet { len, .. } => {
                            ui.label(format!("类型: ZSet (成员数: {})", len));
                        }
                        ValueData::None => {
                            ui.label("Key 不存在");
                        }
                    }
                }
            } else {
                ui.label("选择一个 key 查看详情");
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
