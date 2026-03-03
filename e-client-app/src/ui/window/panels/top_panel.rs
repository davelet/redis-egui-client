use crate::ui::window::RedisApp;
use e_client_basics::constants::WILD_KEY_FILTER;
use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;

pub fn render_top_panel(app: &mut RedisApp, ctx: &egui::Context) {
    // Early return if no active tab
    if app.get_active_tab().is_none() {
        return;
    }

    // Get all needed data before entering UI closure
    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let selected_connection = tab.selected_connection;
    let connected = app.poll_bool(tab.state.connected.clone());
    let _loading = app.poll_bool(tab.state.loading.clone());
    let current_db = app.poll_u32(tab.state.current_db.clone());
    let databases = app.poll_vec_u32(tab.state.databases.clone());

    let mut create_new_tab_with: Option<(
        usize,
        e_client_config::connection::RedisConnectionConfig,
    )> = None;
    let mut auto_connect_idx: Option<usize> = None;

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

                                    // Auto connect if enabled
                                    if app.config.settings.auto_connect {
                                        auto_connect_idx = Some(idx);
                                    }
                                }
                            });
                        }
                    });
            });

            if connected {
                if ui.button(tr(keys::DISCONNECT, current_lang)).clicked() {
                    app.tabs[active_tab_idx].state.spawn_disconnect();
                    // Clear tab name, color, and filter on disconnect, but keep selected_connection
                    let tab = &mut app.tabs[active_tab_idx];
                    tab.name = format!("{} {}", tr(keys::TAB, current_lang), tab.id);
                    tab.connected_color = None;
                    tab.key_filter_input.clear();

                    let key_filter = tab.state.key_filter.clone();
                    app.update_string(key_filter, WILD_KEY_FILTER.to_string());
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
                                // Save DB preference
                                let conn_idx = app.tabs[active_tab_idx].selected_connection;
                                if let Some(idx) = conn_idx {
                                    let conn_name =
                                        app.config.connections.get(idx).map(|c| c.name.clone());
                                    if let Some(name) = conn_name {
                                        let _ = app.update_db_for_connection(&name, db);
                                    }
                                }
                            }
                        }
                    });
            } else {
                if ui.button(tr(keys::CONNECT, current_lang)).clicked() {
                    let tab = &mut app.tabs[active_tab_idx];
                    if let Some(idx) = tab.selected_connection {
                        if let Some(conn) = app.config.connections.get(idx) {
                            let conn_clone = conn.clone();
                            *tab.state.connection_param.blocking_write() = Some(conn_clone.clone());
                            // Update tab name and color to connection name and color
                            tab.name = conn.name.clone();
                            tab.connected_color = conn.color.clone();

                            // Load preferences for this connection
                            app.load_connection_preferences(active_tab_idx);

                            // Connect with preferred DB
                            app.spawn_connect_with_initial_db(active_tab_idx);
                        }
                    } else {
                        // No connection selected - show error
                        *tab.state.error_message.blocking_write() =
                            tr(keys::PLEASE_SELECT_CONNECTION, current_lang).to_string();
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

            // Right side - Settings button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙").clicked() {
                    app.show_settings = true;
                }
            });
        });
    });

    // Settings window
    if app.show_settings {
        egui::Window::new(tr(keys::SETTINGS, current_lang))
            .collapsible(false)
            .resizable(false)
            .default_pos(ctx.screen_rect().center())
            .show(ctx, |ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .spacing([40.0, 8.0])
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        // Language setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::LANGUAGE, current_lang));
                        });
                        let lang = current_lang;
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_salt("settings_lang_select")
                                .selected_text(match lang {
                                    Language::English => ENGLISH,
                                    Language::Chinese => CHINESE,
                                })
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_label(
                                            matches!(lang, Language::English),
                                            ENGLISH,
                                        )
                                        .clicked()
                                    {
                                        app.update_language(Language::English);
                                    }
                                    if ui
                                        .selectable_label(
                                            matches!(lang, Language::Chinese),
                                            CHINESE,
                                        )
                                        .clicked()
                                    {
                                        app.update_language(Language::Chinese);
                                    }
                                });
                        });
                        ui.end_row();

                        // Auto connect setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::AUTO_CONNECT, current_lang));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut auto_connect = app.config.settings.auto_connect;
                            if ui.checkbox(&mut auto_connect, "").changed() {
                                app.config.update_auto_connect(auto_connect);
                            }
                        });
                        ui.end_row();
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button(tr(keys::CLOSE, current_lang)).clicked() {
                        app.show_settings = false;
                    }
                });
            });
    }

    // Handle deferred operations
    if let Some((idx, conn)) = create_new_tab_with {
        app.create_tab_with_connection(idx, conn);
    }

    // Handle auto connect
    if let Some(idx) = auto_connect_idx {
        let active_tab_idx = app.active_tab;
        if let Some(conn) = app.config.connections.get(idx).cloned() {
            let tab = &mut app.tabs[active_tab_idx];
            *tab.state.connection_param.blocking_write() = Some(conn.clone());
            tab.name = conn.name.clone();
            tab.connected_color = conn.color.clone();
            app.load_connection_preferences(active_tab_idx);
            app.spawn_connect_with_initial_db(active_tab_idx);
        }
    }

    // New connection dialog
    if app.new_connection.show {
        app.new_connection
            .render_new_connection_dialog(&mut app.config, ctx, current_lang);
    }
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
