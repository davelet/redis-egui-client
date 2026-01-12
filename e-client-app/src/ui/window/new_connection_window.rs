use e_client_config::config::Config;
use e_client_config::constants::DEFAULT_REDIS_PORT;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};
use e_client_config::RedisConnection;
use egui::Context;

pub(crate) struct NewConnectionWindowWindow {
    pub show: bool,
    pub new_connection_name: String,
    pub new_connection_url: String,
    pub new_connection_port: String,
    pub new_connection_username: String,
    pub new_connection_password: String,
    new_connection_color: [f32; 3],
    pub new_connection_color_hex: Option<String>,
    pub error_message: Option<String>,
}

impl NewConnectionWindowWindow {
    pub(crate) fn new() -> NewConnectionWindowWindow {
        NewConnectionWindowWindow {
            show: false,
            new_connection_name: "".to_string(),
            new_connection_url: "".to_string(),
            new_connection_port: DEFAULT_REDIS_PORT.to_string(),
            new_connection_username: "".to_string(),
            new_connection_password: "".to_string(),
            new_connection_color: [0f32, 0f32, 0f32],
            new_connection_color_hex: None,
            error_message: None,
        }
    }

    pub(crate) fn render_new_connection_dialog(
        &mut self,
        app: &mut Config,
        ctx: &Context,
        current_lang: Language,
    ) {
        egui::Window::new(tr(keys::NEW_CONNECTION_DIALOG, current_lang))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_NAME, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_ADDRESS, current_lang));
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
                        ui.label(tr(keys::CONNECTION_PORT, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_port);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_USERNAME, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_PASSWORD, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_password);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_COLOR, current_lang));
                        if ui
                            .color_edit_button_rgb(&mut self.new_connection_color)
                            .changed()
                        {
                            let c = self.new_connection_color;
                            self.new_connection_color_hex = Some(format!(
                                "#{:02X}{:02X}{:02X}",
                                c[0] as u8, c[1] as u8, c[2] as u8
                            ));
                        };
                    });

                    if let Some(err) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, err);
                    }

                    ui.horizontal(|ui| {
                        let save_btn = tr(keys::SAVE, current_lang);
                        if ui.button(save_btn).clicked() {
                            if self.new_connection_name.trim().is_empty() {
                                self.error_message = Some(
                                    tr(keys::PLEASE_ENTER_CONNECTION_NAME, current_lang)
                                        .to_string(),
                                );
                            } else if self.new_connection_url.trim().is_empty() {
                                self.error_message = Some(
                                    tr(keys::PLEASE_ENTER_CONNECTION_ADDRESS, current_lang)
                                        .to_string(),
                                );
                            } else {
                                match app.add_connection(RedisConnection::new(
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
                                )) {
                                    Ok(_) => {
                                        self.clear_err();
                                    }
                                    Err(e) => {
                                        self.error_message = Some(e.to_message(current_lang));
                                    }
                                }
                            }
                        }

                        if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                            self.clear();
                        }
                    });
                });
            });
    }

    fn clear_err(&mut self) {
        self.show = false;
        self.error_message = None;
    }

    fn clear(&mut self) {
        self.new_connection_name = "".to_string();
        self.new_connection_url = "".to_string();
        self.new_connection_port = DEFAULT_REDIS_PORT.to_string();
        self.new_connection_username = "".to_string();
        self.new_connection_password = "".to_string();
        self.new_connection_color = [0f32, 0f32, 0f32];
        self.new_connection_color_hex = None;
        self.clear_err();
    }
}
