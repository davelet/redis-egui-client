use super::RedisApp;
use crate::core::redis_client::ValueData;
use e_client_config::config::Config;
use e_client_config::constants::{APP_NAME, CHINESE, ENGLISH, LOAD_ERROR_TITLE};
use e_client_config::error::ConfigError;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::{tr, tr_fmt};

pub fn render_tab_bar(app: &mut RedisApp, ctx: &egui::Context) {
    let mut tab_to_close: Option<usize> = None;
    let mut new_tab_requested = false;
    let mut switch_to_tab: Option<usize> = None;

    egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            // Collect tab info first to avoid borrow issues
            let tab_infos: Vec<_> = app
                .tabs
                .iter()
                .enumerate()
                .map(|(idx, tab)| {
                    let is_active = idx == app.active_tab;

                    // Get connection color from tab (only set after successful connection)
                    let color = tab
                        .connected_color
                        .as_ref()
                        .and_then(|hex| parse_color_hex(hex));

                    (idx, is_active, tab.name.clone(), color)
                })
                .collect();

            // Render tabs
            for (idx, is_active, tab_text, color) in tab_infos {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Show color indicator before the button
                        if let Some(color) = color {
                            ui.colored_label(color, "●");
                        }

                        let button = if is_active {
                            egui::Button::new(&tab_text)
                                .fill(egui::Color32::from_rgb(200, 220, 240))
                        } else {
                            egui::Button::new(&tab_text)
                        };

                        if ui.add(button).clicked() {
                            switch_to_tab = Some(idx);
                        }

                        // Close button
                        if app.tabs.len() > 1 {
                            if ui.small_button("×").clicked() {
                                tab_to_close = Some(idx);
                            }
                        }
                    });
                });
            }

            // New tab button
            if ui.button("+").clicked() {
                new_tab_requested = true;
            }
        });
    });

    // Handle tab operations after the UI
    if let Some(idx) = switch_to_tab {
        app.switch_to_tab(idx);
    }
    if let Some(idx) = tab_to_close {
        app.close_tab(idx);
    }
    if new_tab_requested {
        app.create_new_tab();
    }
}

pub fn render_top_panel(app: &mut RedisApp, ctx: &egui::Context) {
    // Early return if no active tab
    if app.get_active_tab().is_none() {
        return;
    }

    // Get all needed data before entering the UI closure
    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let selected_connection = tab.selected_connection;
    let connected = app.poll_bool(tab.state.connected.clone());
    let loading = app.poll_bool(tab.state.loading.clone());
    let current_db = app.poll_u32(tab.state.current_db.clone());
    let databases = app.poll_vec_u32(tab.state.databases.clone());

    let mut create_new_tab_with: Option<(
        usize,
        e_client_config::connection::RedisConnectionConfig,
    )> = None;

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Show connection color indicator for current connection
            if let Some(idx) = selected_connection {
                if let Some(conn) = app.config.connections.get(idx) {
                    if let Some(color_hex) = &conn.color {
                        if let Some(color) = parse_color_hex(color_hex) {
                            ui.colored_label(color, "●");
                        }
                    }
                }
            }

            ui.label(tr(keys::CONNECTION_URL, current_lang));

            // Connection dropdown - disabled when connected
            let selected_name = selected_connection
                .and_then(|idx| app.config.connections.get(idx))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| tr(keys::SELECT_CONNECTION, current_lang).to_string());

            ui.add_enabled_ui(!connected, |ui| {
                egui::ComboBox::from_id_salt("connection_select")
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (idx, conn) in app.config.connections.connections.iter().enumerate() {
                            let tab = &mut app.tabs[active_tab_idx];
                            let is_selected = tab.selected_connection == Some(idx);
                            ui.horizontal(|ui| {
                                // Show color indicator for each connection in dropdown
                                if let Some(color_hex) = &conn.color {
                                    if let Some(color) = parse_color_hex(color_hex) {
                                        ui.colored_label(color, "●");
                                    }
                                } else {
                                    ui.label("  "); // Placeholder for alignment
                                }
                                if ui.selectable_label(is_selected, &conn.name).clicked() {
                                    tab.selected_connection = Some(idx);
                                }
                            });
                        }
                    });
            });

            if connected {
                if ui.button(tr(keys::DISCONNECT, current_lang)).clicked() {
                    app.tabs[active_tab_idx].state.spawn_disconnect();
                }

                ui.separator();
                ui.label(tr(keys::DATABASE, current_lang));

                egui::ComboBox::from_id_salt("db_select")
                    .selected_text(format!("DB {}", current_db))
                    .show_ui(ui, |ui| {
                        for db in databases {
                            if ui
                                .selectable_label(current_db == db, format!("DB {}", db))
                                .clicked()
                            {
                                app.tabs[active_tab_idx].state.spawn_select_db(db);
                            }
                        }
                    });
            } else {
                if ui.button(tr(keys::CONNECT, current_lang)).clicked() {
                    let tab = &mut app.tabs[active_tab_idx];
                    if let Some(idx) = tab.selected_connection {
                        if let Some(conn) = app.config.connections.get(idx) {
                            *tab.state.connection_param.blocking_write() = Some(conn.clone());
                            // Update tab name and color to connection name and color
                            tab.name = conn.name.clone();
                            tab.connected_color = conn.color.clone();
                            tab.state.spawn_connect();
                        }
                    } else {
                        app.tabs[active_tab_idx]
                            .state
                            .show_err(tr(keys::PLEASE_SELECT_CONNECTION, current_lang).to_string());
                    }
                }

                // "Open in New Tab" button
                if let Some(idx) = selected_connection {
                    if ui
                        .button(format!("📑 {}", tr(keys::OPEN_IN_NEW_TAB, current_lang)))
                        .clicked()
                    {
                        if let Some(conn) = app.config.connections.get(idx) {
                            create_new_tab_with = Some((idx, conn.clone()));
                        }
                    }
                }
            }

            if loading {
                ui.spinner();
            }
            ui.separator();

            // Connection management buttons - disabled when connected
            ui.add_enabled_ui(!connected, |ui| {
                // Add edit connection button
                if let Some(selected_idx) = selected_connection {
                    if ui
                        .button(format!("✏ {}", tr(keys::EDIT_CONNECTION, current_lang)))
                        .clicked()
                    {
                        if let Some(conn) = app.config.connections.get(selected_idx) {
                            app.new_connection.open_for_edit(conn);
                        }
                    }
                }
                // Add new connection button
                if ui
                    .button(format!("+ {}", tr(keys::NEW_CONNECTION, current_lang)))
                    .clicked()
                {
                    app.new_connection.show = true;
                }
            });

            // Right side - Language selector
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let lang = current_lang;
                egui::ComboBox::from_id_salt("lang_select")
                    .selected_text(match lang {
                        Language::English => ENGLISH,
                        Language::Chinese => CHINESE,
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(matches!(lang, Language::English), ENGLISH)
                            .clicked()
                        {
                            app.update_language(Language::English);
                        }
                        if ui
                            .selectable_label(matches!(lang, Language::Chinese), CHINESE)
                            .clicked()
                        {
                            app.update_language(Language::Chinese);
                        }
                    });
                ui.label(tr(keys::LANGUAGE, lang));
            });
        });
    });

    // Handle deferred operations
    if let Some((idx, conn)) = create_new_tab_with {
        app.create_tab_with_connection(idx, conn);
    }

    // New connection dialog
    if app.new_connection.show {
        app.new_connection
            .render_new_connection_dialog(&mut app.config, ctx, current_lang);
    }
}

pub fn render_side_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let keys = app.poll_vec_string(tab.state.keys.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());

    egui::SidePanel::left("side_panel")
        .min_width(250.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading(tr(keys::KEYS, current_lang));

                ui.horizontal(|ui| {
                    ui.label(tr(keys::FILTER, current_lang));
                    let tab = &mut app.tabs[active_tab_idx];
                    let changed = ui.text_edit_singleline(&mut tab.key_filter_input).changed();
                    if changed {
                        let key_filter = tab.state.key_filter.clone();
                        let key_filter_input = tab.key_filter_input.clone();
                        // Release mutable borrow
                        app.update_string(key_filter, key_filter_input);
                        app.tabs[active_tab_idx].state.spawn_load_keys();
                    }
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for key in keys {
                        let is_selected = selected_key.as_ref() == Some(&key);
                        if ui.selectable_label(is_selected, &key).clicked() {
                            let tab = &mut app.tabs[active_tab_idx];
                            *tab.state.selected_key.blocking_write() = Some(key.clone());
                            tab.state.spawn_load_value(key);
                        }
                    }
                });
            });
        });
}

pub fn render_central_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let command_output = app.poll_string(tab.state.command_output.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());
    let value = app.poll_option_value(tab.state.key_value.clone());

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr(keys::COMMAND_LABEL, current_lang));
            let tab = &mut app.tabs[active_tab_idx];
            let response = ui.text_edit_singleline(&mut tab.command_input_buffer);

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let cmd = tab.command_input_buffer.clone();
                tab.state.spawn_execute_command(cmd);
                tab.command_input_buffer.clear();
            }

            if ui.button(tr(keys::EXECUTE, current_lang)).clicked() {
                let cmd = tab.command_input_buffer.clone();
                tab.state.spawn_execute_command(cmd);
                tab.command_input_buffer.clear();
            }
        });

        ui.separator();

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

        if let Some(key) = selected_key {
            ui.heading(tr_fmt(keys::KEY_HEADING, current_lang, &[&key]));

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
        });
    })
}

/// Parse hex color string (#RRGGBB) to egui Color32
fn parse_color_hex(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(egui::Color32::from_rgb(r, g, b))
}
