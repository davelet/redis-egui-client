mod redis_client;
mod app_state;

use app_state::AppState;
use eframe::egui;
use redis_client::ValueData;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    connection_url_input: String,
    command_input_buffer: String,
    key_filter_input: String,
}

impl Default for RedisApp {
    fn default() -> Self {
        Self {
            state: AppState::new(),
            connection_url_input: "redis://127.0.0.1:6379".to_string(),
            command_input_buffer: String::new(),
            key_filter_input: "*".to_string(),
        }
    }
}

impl eframe::App for RedisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.render_top_panel(ctx);
        self.render_side_panel(ctx);
        self.render_central_panel(ctx);
    }
}

impl RedisApp {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("连接:");
                ui.text_edit_singleline(&mut self.connection_url_input);

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
                        self.update_string(self.state.connection_url.clone(), self.connection_url_input.clone());
                        self.state.spawn_connect();
                    }
                }
                
                let loading = self.poll_bool(self.state.loading.clone());
                if loading {
                    ui.spinner();
                }
            });
        });
    }

    fn render_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel").min_width(250.0).show(ctx, |ui| {
            ui.heading("Keys");
            
            ui.horizontal(|ui| {
                ui.label("过滤:");
                let mut changed = false;
                if ui.text_edit_singleline(&mut self.key_filter_input).changed() {
                    changed = true;
                }
                if ui.button("刷新").clicked() || changed {
                    self.update_string(self.state.key_filter.clone(), self.key_filter_input.clone());
                    self.state.spawn_load_keys();
                }
            });

            ui.separator();

            let keys = self.poll_vec_string(self.state.keys.clone());
            let selected_key = self.poll_option_string(self.state.selected_key.clone());

            egui::ScrollArea::vertical().show(ui, |ui| {
                for key in keys {
                    let is_selected = selected_key.as_ref() == Some(&key);
                    if ui.selectable_label(is_selected, &key).clicked() {
                        self.state.spawn_load_value(key.clone());
                    }
                }
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
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
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
        lock.try_read().ok().and_then(|v| v.clone())
    }

    fn update_string(&self, lock: Arc<RwLock<String>>, value: String) {
        if let Ok(mut v) = lock.try_write() {
            *v = value;
        }
    }
}
