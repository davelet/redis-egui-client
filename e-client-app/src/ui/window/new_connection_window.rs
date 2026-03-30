use e_client_config::config::Config;
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::constants::DEFAULT_REDIS_PORT;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr, TranslationKey};
use egui::Context;

// Predefined high-contrast colors for connections
const PREDEFINED_COLORS: &[(&str, [f32; 3])] = &[
    ("#E53935", [0.90, 0.22, 0.21]), // Red
    ("#D81B60", [0.85, 0.11, 0.38]), // Pink
    ("#8E24AA", [0.56, 0.14, 0.67]), // Purple
    ("#3949AB", [0.22, 0.29, 0.67]), // Indigo
    ("#1E88E5", [0.12, 0.53, 0.90]), // Blue
    ("#00ACC1", [0.00, 0.67, 0.76]), // Cyan
    ("#43A047", [0.26, 0.63, 0.28]), // Green
    ("#FDD835", [0.99, 0.85, 0.21]), // Yellow
];

pub(crate) struct NewConnectionWindowWindow {
    pub show: bool,
    pub edit_mode: bool,
    pub editing_connection_name: Option<String>,
    pub new_connection_name: String,
    pub new_connection_url: String,
    pub new_connection_port: String,
    pub new_connection_username: String,
    pub new_connection_password: String,
    pub new_connection_color_hex: Option<String>,
    pub error_message: Option<String>,
}

impl NewConnectionWindowWindow {
    pub(crate) fn new() -> NewConnectionWindowWindow {
        NewConnectionWindowWindow {
            show: false,
            edit_mode: false,
            editing_connection_name: None,
            new_connection_name: String::new(),
            new_connection_url: String::new(),
            new_connection_port: DEFAULT_REDIS_PORT.to_string(),
            new_connection_username: String::new(),
            new_connection_password: String::new(),
            new_connection_color_hex: None,
            error_message: None,
        }
    }

    pub(crate) fn open_for_edit(&mut self, conn: &RedisConnectionConfig) {
        self.show = true;
        self.edit_mode = true;
        self.editing_connection_name = Some(conn.name.clone());
        self.new_connection_name = conn.name.clone();
        self.new_connection_url = conn.url.clone();
        self.new_connection_port = conn.port.clone();
        self.new_connection_username = conn.username.clone().unwrap_or_default();
        self.new_connection_password = conn.password.clone().unwrap_or_default();

        // Parse color hex to RGB
        self.new_connection_color_hex = conn.color.clone();

        self.error_message = None;
    }

    pub(crate) fn render_new_connection_dialog(
        &mut self,
        app: &mut Config,
        ctx: &Context,
        current_lang: Language,
    ) {
        let title = if self.edit_mode {
            tr(TranslationKey::EditConnectionDialog, current_lang)
        } else {
            tr(TranslationKey::NewConnectionDialog, current_lang)
        };

        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionName, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionAddress, current_lang));
                        let url_id = ui.make_persistent_id("new_connection_url");
                        let res = ui.add(
                            egui::TextEdit::singleline(&mut self.new_connection_url).id(url_id),
                        );
                        let had_focus = ctx.memory(|m| m.has_focus(url_id));
                        let has_focus_now = res.has_focus();

                        if (!had_focus || !has_focus_now)
                            && self.new_connection_name.trim().is_empty()
                            && !self.new_connection_url.trim().is_empty()
                        {
                            self.new_connection_name = self.new_connection_url.clone();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionPort, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_port);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionUsername, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionPassword, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_password);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(TranslationKey::ConnectionColor, current_lang));
                        ui.horizontal_wrapped(|ui| {
                            for (hex, rgb) in PREDEFINED_COLORS {
                                let color = egui::Color32::from_rgb(
                                    (rgb[0] * 255.0) as u8,
                                    (rgb[1] * 255.0) as u8,
                                    (rgb[2] * 255.0) as u8,
                                );
                                let is_selected = self.new_connection_color_hex.as_ref()
                                    == Some(&hex.to_string());
                                let button = egui::Button::new("")
                                    .fill(color)
                                    .min_size(egui::vec2(24.0, 24.0))
                                    .stroke(if is_selected {
                                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                                    } else {
                                        egui::Stroke::NONE
                                    });
                                if ui.add(button).clicked() {
                                    self.new_connection_color_hex = Some(hex.to_string());
                                }
                            }
                        });
                    });

                    if let Some(err) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, err);
                    }

                    ui.horizontal(|ui| {
                        let save_btn = tr(TranslationKey::Save, current_lang);
                        if ui.button(save_btn).clicked() {
                            if self.new_connection_name.trim().is_empty() {
                                self.error_message = Some(
                                    tr(TranslationKey::PleaseEnterConnectionName, current_lang)
                                        .to_string(),
                                );
                            } else if self.new_connection_url.trim().is_empty() {
                                self.error_message = Some(
                                    tr(TranslationKey::PleaseEnterConnectionAddress, current_lang)
                                        .to_string(),
                                );
                            } else {
                                let result = if self.edit_mode {
                                    // Edit existing connection
                                    if let Some(old_name) = &self.editing_connection_name {
                                        app.update_single_connection(
                                            old_name,
                                            self.build_connection(),
                                        )
                                    } else {
                                        Err(e_client_config::error::ConfigError::ConnectionNotFound)
                                    }
                                } else {
                                    // Add new connection
                                    app.add_connection(self.build_connection())
                                };

                                match result {
                                    Ok(_) => {
                                        self.clear();
                                    }
                                    Err(e) => {
                                        self.error_message = Some(e.to_message(current_lang));
                                    }
                                }
                            }
                        }

                        if ui
                            .button(tr(TranslationKey::Cancel, current_lang))
                            .clicked()
                        {
                            self.clear();
                        }
                    });
                });
            });
    }

    fn build_connection(&self) -> RedisConnectionConfig {
        RedisConnectionConfig::new(
            self.new_connection_name.clone(),
            self.new_connection_url.clone(),
            self.new_connection_port.clone(),
            if self.new_connection_username.is_empty() {
                None
            } else {
                Some(self.new_connection_username.clone())
            },
            if self.new_connection_password.is_empty() {
                None
            } else {
                Some(self.new_connection_password.clone())
            },
            self.new_connection_color_hex.clone(),
        )
    }

    fn clear_err(&mut self) {
        self.show = false;
        self.error_message = None;
    }

    fn clear(&mut self) {
        self.new_connection_name = String::new();
        self.new_connection_url = String::new();
        self.new_connection_port = DEFAULT_REDIS_PORT.to_string();
        self.new_connection_username = String::new();
        self.new_connection_password = String::new();
        self.new_connection_color_hex = None;
        self.edit_mode = false;
        self.editing_connection_name = None;
        self.clear_err();
    }
}
