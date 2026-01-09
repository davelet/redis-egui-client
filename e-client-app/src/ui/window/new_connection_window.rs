use crate::ui::window::RedisApp;
use e_client_config::config::Config;
use e_client_config::constants::{DEFAULT_REDIS_PORT, DEFAULT_REDIS_URL};
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};
use egui::Context;

pub(crate) struct NewConnectionWindowWindow {
    pub show: bool,
    pub new_connection_name: String,
    pub new_connection_url: String,
    pub new_connection_port: String,
    pub new_connection_username: String,
    pub new_connection_password: String,
    pub new_connection_color: String,
    pub error_message: String,
}

impl NewConnectionWindowWindow {
    pub(crate) fn new() -> NewConnectionWindowWindow {
        NewConnectionWindowWindow {
            show: false,
            new_connection_name: "".to_string(),
            new_connection_url: DEFAULT_REDIS_URL.to_string(),
            new_connection_port: DEFAULT_REDIS_PORT.to_string(),
            new_connection_username: "".to_string(),
            new_connection_password: "".to_string(),
            new_connection_color: "".to_string(),
            error_message: "".to_string(),
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
                        ui.text_edit_singleline(&mut self.new_connection_url);
                    });
                    ui.horizontal(|ui| {
                        ui.label(tr(keys::CONNECTION_PORT, current_lang));
                        ui.text_edit_singleline(&mut self.new_connection_port);
                    });

                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.error_message);
                    }

                    ui.horizontal(|ui| {
                        let save_btn = tr(keys::SAVE, current_lang);
                        if ui.button(save_btn).clicked() {
                            if self.new_connection_name.trim().is_empty() {
                                self.error_message =
                                    tr(keys::PLEASE_ENTER_CONNECTION_NAME, current_lang)
                                        .to_string();
                            } else if self.new_connection_url.trim().is_empty() {
                                self.error_message =
                                    tr(keys::PLEASE_ENTER_CONNECTION_ADDRESS, current_lang)
                                        .to_string();
                            } else {
                                match app.add_connection(
                                    self.new_connection_name.clone(),
                                    self.new_connection_url.clone(),
                                ) {
                                    Ok(_) => {
                                        self.clear_err();
                                    }
                                    Err(e) => {
                                        self.error_message = e.to_message(current_lang);
                                    }
                                }
                            }
                        }

                        if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                            self.clear_err();
                        }
                    });
                });
            });
    }

    fn clear_err(&mut self) {
        self.show = false;
        self.error_message.clear();
    }
}
