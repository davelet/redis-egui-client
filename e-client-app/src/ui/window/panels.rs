use super::RedisApp;
use crate::core::redis_client::ValueData;
use e_client_config::constants::DEFAULT_REDIS_URL;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::{tr, tr_fmt};

pub fn render_top_panel(app: &mut RedisApp, ctx: &egui::Context) {
    let current_lang = app.poll_language(app.state.language.clone());

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr(keys::CONNECTION_URL, current_lang));

            // Connection dropdown
            let selected_name = app
                .selected_connection
                .and_then(|idx| app.config.connections.get(idx))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| tr(keys::SELECT_CONNECTION, current_lang).to_string());

            egui::ComboBox::from_id_salt("connection_select")
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    for (idx, conn) in app.config.connections.iter().enumerate() {
                        let is_selected = app.selected_connection == Some(idx);
                        if ui.selectable_label(is_selected, &conn.name).clicked() {
                            app.selected_connection = Some(idx);
                        }
                    }
                });

            // Add new connection button
            if ui
                .button(format!("+ {}", tr(keys::NEW_CONNECTION, current_lang)))
                .clicked()
            {
                app.show_new_connection_dialog = true;
                app.new_connection_name.clear();
                app.new_connection_url = DEFAULT_REDIS_URL.to_string();
                app.error_message.clear();
            }
            ui.separator();

            let connected = app.poll_bool(app.state.connected.clone());

            if connected {
                if ui.button(tr(keys::DISCONNECT, current_lang)).clicked() {
                    app.state.spawn_disconnect();
                }

                ui.separator();
                ui.label(tr(keys::DATABASE, current_lang));

                let current_db = app.poll_u32(app.state.current_db.clone());
                let databases = app.poll_vec_u32(app.state.databases.clone());

                egui::ComboBox::from_id_salt("db_select")
                    .selected_text(format!("DB {}", current_db))
                    .show_ui(ui, |ui| {
                        for db in databases {
                            if ui
                                .selectable_label(current_db == db, format!("DB {}", db))
                                .clicked()
                            {
                                app.state.spawn_select_db(db);
                            }
                        }
                    });
            } else {
                if ui.button(tr(keys::CONNECT, current_lang)).clicked() {
                    if let Some(idx) = app.selected_connection {
                        if let Some(conn) = app.config.connections.get(idx) {
                            app.update_string(app.state.connection_url.clone(), conn.url.clone());
                            app.state.spawn_connect();
                        }
                    } else {
                        app.error_message =
                            tr(keys::PLEASE_SELECT_CONNECTION, current_lang).to_string();
                    }
                }
            }

            let loading = app.poll_bool(app.state.loading.clone());
            if loading {
                ui.spinner();
            }
        });

        // Right side - Language selector
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let lang = app.poll_language(app.state.language.clone());
            egui::ComboBox::from_id_salt("lang_select")
                .selected_text(match lang {
                    Language::English => tr(keys::ENGLISH, lang),
                    Language::Chinese => tr(keys::CHINESE, lang),
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(
                            matches!(lang, Language::English),
                            tr(keys::ENGLISH, lang),
                        )
                        .clicked()
                    {
                        app.update_language(Language::English);
                    }
                    if ui
                        .selectable_label(
                            matches!(lang, Language::Chinese),
                            tr(keys::CHINESE, lang),
                        )
                        .clicked()
                    {
                        app.update_language(Language::Chinese);
                    }
                });
            ui.label(tr(keys::LANGUAGE, lang));
        });
    });

    // New connection dialog
    if app.show_new_connection_dialog {
        render_new_connection_dialog(app, ctx, current_lang);
    }
}

fn render_new_connection_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    egui::Window::new(tr(keys::NEW_CONNECTION_DIALOG, current_lang))
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(tr(keys::CONNECTION_NAME, current_lang));
                    ui.text_edit_singleline(&mut app.new_connection_name);
                });

                ui.horizontal(|ui| {
                    ui.label(tr(keys::CONNECTION_ADDRESS, current_lang));
                    ui.text_edit_singleline(&mut app.new_connection_url);
                });

                if !app.error_message.is_empty() {
                    ui.colored_label(egui::Color32::RED, &app.error_message);
                }

                ui.horizontal(|ui| {
                    if ui.button(tr(keys::SAVE, current_lang)).clicked() {
                        if app.new_connection_name.trim().is_empty() {
                            app.error_message =
                                tr(keys::PLEASE_ENTER_CONNECTION_NAME, current_lang).to_string();
                        } else if app.new_connection_url.trim().is_empty() {
                            app.error_message =
                                tr(keys::PLEASE_ENTER_CONNECTION_ADDRESS, current_lang).to_string();
                        } else {
                            match app.config.add_connection(
                                app.new_connection_name.clone(),
                                app.new_connection_url.clone(),
                            ) {
                                Ok(_) => {
                                    app.show_new_connection_dialog = false;
                                    app.error_message.clear();
                                }
                                Err(e) => {
                                    app.error_message = e.to_message(current_lang);
                                }
                            }
                        }
                    }

                    if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                        app.show_new_connection_dialog = false;
                        app.error_message.clear();
                    }
                });
            });
        });
}

pub fn render_side_panel(app: &mut RedisApp, ctx: &egui::Context) {
    egui::SidePanel::left("side_panel")
        .min_width(250.0)
        .show(ctx, |ui| {
            let current_lang = app.poll_language(app.state.language.clone());

            ui.vertical(|ui| {
                ui.heading(tr(keys::KEYS, current_lang));

                ui.horizontal(|ui| {
                    ui.label(tr(keys::FILTER, current_lang));
                    if ui.text_edit_singleline(&mut app.key_filter_input).changed() {
                        app.update_string(
                            app.state.key_filter.clone(),
                            app.key_filter_input.clone(),
                        );
                        app.state.spawn_load_keys();
                    }
                });

                let keys = app.poll_vec_string(app.state.keys.clone());
                let selected_key = app.poll_option_string(app.state.selected_key.clone());

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for key in keys {
                        let is_selected = selected_key.as_ref() == Some(&key);
                        if ui.selectable_label(is_selected, &key).clicked() {
                            *app.state.selected_key.blocking_write() = Some(key.clone());
                            app.state.spawn_load_value(key);
                        }
                    }
                });
            });
        });
}

pub fn render_central_panel(app: &mut RedisApp, ctx: &egui::Context) {
    let current_lang = app.poll_language(app.state.language.clone());

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr(keys::COMMAND_LABEL, current_lang));
            let response = ui.text_edit_singleline(&mut app.command_input_buffer);

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let cmd = app.command_input_buffer.clone();
                app.state.spawn_execute_command(cmd);
                app.command_input_buffer.clear();
            }

            if ui.button(tr(keys::EXECUTE, current_lang)).clicked() {
                let cmd = app.command_input_buffer.clone();
                app.state.spawn_execute_command(cmd);
                app.command_input_buffer.clear();
            }
        });

        ui.separator();

        let command_output = app.poll_string(app.state.command_output.clone());
        if !command_output.is_empty() {
            ui.group(|ui| {
                ui.label(tr(keys::OUTPUT, current_lang));
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        ui.label(&command_output);
                    });
            });
            ui.separator();
        }

        let selected_key = app.poll_option_string(app.state.selected_key.clone());
        if let Some(key) = selected_key {
            ui.heading(tr_fmt(keys::KEY_HEADING, current_lang, &[&key]));

            let value = app.poll_option_value(app.state.key_value.clone());

            if let Some(val) = value {
                match val {
                    ValueData::String(s) => {
                        ui.label(tr(keys::TYPE_STRING, current_lang));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.text_edit_multiline(&mut s.as_str());
                        });
                    }
                    ValueData::List { len, items } => {
                        ui.label(tr_fmt(keys::TYPE_LIST, current_lang, &[&len.to_string()]));

                        if items.is_empty() && len > 0 {
                            if ui.button(tr(keys::LOAD_FIRST_100, current_lang)).clicked() {
                                app.state.spawn_load_list_range(key.clone(), 0, 99);
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
                        ui.label(tr_fmt(keys::TYPE_HASH, current_lang, &[&len.to_string()]));

                        if fields.is_empty() && len > 0 {
                            if ui.button(tr(keys::LOAD_FIELDS, current_lang)).clicked() {
                                app.state.spawn_load_hash_fields(key.clone());
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
                        ui.label(tr_fmt(keys::TYPE_SET, current_lang, &[&len.to_string()]));
                    }
                    ValueData::ZSet { len, .. } => {
                        ui.label(tr_fmt(keys::TYPE_ZSET, current_lang, &[&len.to_string()]));
                    }
                    ValueData::None => {
                        ui.label(tr(keys::KEY_NOT_EXIST, current_lang));
                    }
                }
            }
        } else {
            ui.label(tr(keys::SELECT_KEY_PROMPT, current_lang));
        }
    });
}
