use crate::core::redis_client::ValueData;
use crate::ui::window::RedisApp;
use e_client_config::config::Config;
use e_client_config::constants::APP_NAME;
use e_client_config::constants::LOAD_ERROR_TITLE;
use e_client_config::error::ConfigError;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;
use e_client_config::translations::tr_fmt;

pub fn render_central_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let connected = app.poll_bool(tab.state.connected.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());
    let value = app.poll_option_value(tab.state.key_value.clone());

    egui::CentralPanel::default().show(ctx, |ui| {
        if !connected {
            // Show blank when not connected
            return;
        }

        if let Some(key) = selected_key {
            ui.heading(tr_fmt(keys::KEY_HEADING, current_lang, &[&key]));

            if let Some(val) = value {
                match val {
                    ValueData::String(s) => {
                        ui.label(tr(keys::TYPE_STRING, current_lang));
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let available = ui.available_size();
                                let desired_height = available.y.max(300.0).min(600.0);
                                ui.add_sized(
                                    [available.x, desired_height],
                                    egui::TextEdit::multiline(&mut s.as_str()),
                                );
                            });
                    }
                    ValueData::List { len, items } => {
                        ui.label(tr_fmt(keys::TYPE_LIST, current_lang, &[&len.to_string()]));

                        if items.is_empty() && len > 0 {
                            if ui.button(tr(keys::LOAD_FIRST_100, current_lang)).clicked() {
                                app.tabs[active_tab_idx].state.spawn_load_list_range(
                                    key.clone(),
                                    0,
                                    99,
                                );
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
                                app.tabs[active_tab_idx]
                                    .state
                                    .spawn_load_hash_fields(key.clone());
                            }
                        } else {
                            egui::ScrollArea::both()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for (field, value) in fields.iter() {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(
                                                egui::RichText::new(format!("{}: ", field))
                                                    .strong(),
                                            );
                                            ui.label(value);
                                        });
                                        ui.separator();
                                    }
                                });
                        }
                    }
                    ValueData::Set { len, items } => {
                        ui.label(tr_fmt(keys::TYPE_SET, current_lang, &[&len.to_string()]));

                        if items.is_empty() && len > 0 {
                            if ui.button(tr(keys::LOAD_MEMBERS, current_lang)).clicked() {
                                app.tabs[active_tab_idx]
                                    .state
                                    .spawn_load_set_members(key.clone());
                            }
                        } else {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for item in items.iter() {
                                        ui.label(item);
                                        ui.separator();
                                    }
                                });
                        }
                    }
                    ValueData::ZSet { len, items } => {
                        ui.label(tr_fmt(keys::TYPE_ZSET, current_lang, &[&len.to_string()]));

                        if items.is_empty() && len > 0 {
                            if ui.button(tr(keys::LOAD_MEMBERS, current_lang)).clicked() {
                                app.tabs[active_tab_idx].state.spawn_load_zset_range(
                                    key.clone(),
                                    0,
                                    99,
                                );
                            }
                        } else {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for (member, score) in items.iter() {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(format!("{}: ", score))
                                                    .strong(),
                                            );
                                            ui.label(member);
                                        });
                                        ui.separator();
                                    }
                                });
                        }
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

pub fn render_error_panel(err: ConfigError) -> Result<(), eframe::Error> {
    let err = err.to_message(Language::default());
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native(APP_NAME, options, move |ctx, _frame| {
        use std::cell::Cell;
        thread_local! {
            static SHOW_POPUP: Cell<bool> = Cell::new(false);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new(LOAD_ERROR_TITLE).color(egui::Color32::RED));
            ui.horizontal(|ui| {
                ui.label(err.clone());
            });
            ui.separator();

            if ui.button("click to reset problematic file").clicked() {
                if let Err(_) = Config::load_user_settings() {
                    let _ = Config::reset_user_settings();
                }
                if let Err(_) = Config::load_window_params() {
                    let _ = Config::reset_window_params();
                }
                if let Err(_) = Config::load_connections() {
                    let _ = Config::clear_connections();
                }
                SHOW_POPUP.set(true);
            }
        });
        SHOW_POPUP.with(|popup| {
            if popup.get() {
                egui::Window::new("Well Done!")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Now restart your app.");
                        if ui.button("OK").clicked() {
                            std::process::exit(0);
                        }
                    });
            }
        })
    })
}
