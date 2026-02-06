use crate::ui::window::RedisApp;
use e_client_basics::constants::{LOAD_MORE_BATCH_SIZE, MAX_LOADED_KEYS};
use e_client_config::constants::WILD_KEY_FILTER;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;

pub fn render_side_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let keys = app.poll_vec_string(tab.state.keys.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());
    let loading = app.poll_bool(tab.state.loading.clone());
    let scan_has_more = app.poll_bool(tab.state.scan_has_more.clone());
    let total_keys = app.poll_usize(tab.state.total_keys.clone());
    let connected = app.poll_bool(tab.state.connected.clone());
    let loading_progress_text = app.poll_string(tab.state.loading_progress_text.clone());
    let key_filter = app.poll_string(tab.state.key_filter.clone());
    let side_panel_width = tab.side_panel_width;

    egui::SidePanel::left("side_panel")
        .min_width(250.0)
        .max_width(800.0)
        .default_width(side_panel_width)
        .resizable(true)
        .show(ctx, |ui| {
            // Save the current width if it changed
            let current_width = ui.available_width();
            if (current_width - side_panel_width).abs() > 1.0 {
                app.update_tab_side_panel_width(active_tab_idx, current_width);
            }

            if !connected {
                // Show blank when not connected
                return;
            }
            if !connected {
                // Show blank when not connected
                return;
            }

            // Heading with loaded/total key count
            let is_full_scan = key_filter == "" || key_filter == WILD_KEY_FILTER.to_string();
            let total_display = if is_full_scan || !scan_has_more {
                total_keys.to_string()
            } else {
                tr(keys::UNKNOWN, current_lang).to_string()
            };
            let heading_text = format!(
                "{} ({}/{})",
                tr(keys::KEYS, current_lang),
                keys.len(),
                total_display
            );
            ui.horizontal(|ui| {
                ui.heading(heading_text);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("➕").clicked() {
                        app.new_key_dialog.reset();
                        app.new_key_dialog.show = true;
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label(tr(keys::FILTER, current_lang));
                let tab = &mut app.tabs[active_tab_idx];
                let available_width = ui.available_width() - 30.0; // Reserve space for refresh button
                let changed = ui
                    .add_sized(
                        egui::vec2(available_width, 20.0),
                        egui::TextEdit::singleline(&mut tab.key_filter_input),
                    )
                    .changed();
                if changed {
                    let key_filter = tab.state.key_filter.clone();
                    let input = tab.key_filter_input.clone();
                    let input = input.trim();

                    // Process filter: if empty, use "*"; if contains *, use as-is; otherwise add * on both sides
                    let processed_filter = if input.is_empty() {
                        WILD_KEY_FILTER.to_string()
                    } else if input.contains('*') {
                        input.to_string()
                    } else {
                        format!("{}{}{}", WILD_KEY_FILTER, input, WILD_KEY_FILTER)
                    };

                    // Release mutable borrow
                    app.update_string(key_filter, processed_filter);
                    app.tabs[active_tab_idx].state.spawn_load_keys();
                }

                // Refresh keys button
                if ui.button("🔄").clicked() {
                    app.tabs[active_tab_idx].state.spawn_load_keys();
                }
            });

            // Show loading progress
            if loading && !loading_progress_text.is_empty() {
                ui.label(egui::RichText::new(&loading_progress_text).weak());
            }

            // Show "load more" button below filter (only if connected)
            if connected && !loading {
                let remaining_keys = if is_full_scan {
                    total_keys.saturating_sub(keys.len())
                } else {
                    // Cannot determine remaining keys when using filter
                    usize::MAX
                };

                if keys.len() >= MAX_LOADED_KEYS {
                    ui.label(
                        egui::RichText::new(tr(keys::TOO_MANY_KEYS, current_lang))
                            .color(egui::Color32::PURPLE),
                    );
                } else if scan_has_more {
                    ui.horizontal(|ui| {
                        if remaining_keys > LOAD_MORE_BATCH_SIZE {
                            if ui.button(tr(keys::LOAD_MORE_KEYS, current_lang)).clicked() {
                                app.tabs[active_tab_idx].state.spawn_load_more_keys(false);
                            }
                        }
                        if ui.button(tr(keys::LOAD_ALL_KEYS, current_lang)).clicked() {
                            app.tabs[active_tab_idx].state.spawn_load_more_keys(true);
                        }
                    });
                }
            }

            ui.separator();

            // Fill remaining space with scroll area - optimized with limit for large key lists
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .id_salt("keys_scroll")
                .show(ui, |ui| {
                    // Limit rendering to improve performance with large datasets
                    // Only show first MAX_LOADED_KEYS keys or all keys if less than that
                    let keys_to_show = if keys.len() > MAX_LOADED_KEYS {
                        &keys[0..MAX_LOADED_KEYS]
                    } else {
                        &keys[..]
                    };

                    for key in keys_to_show {
                        let is_selected = selected_key.as_ref() == Some(key);
                        // Use allocate_ui_with_layout to make the entire row clickable
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), ui.spacing().interact_size.y),
                            egui::Sense::click(),
                        );

                        if response.clicked() {
                            let tab = &mut app.tabs[active_tab_idx];
                            *tab.state.selected_key.blocking_write() = Some(key.clone());
                            tab.state.spawn_load_value(key.clone());
                        }

                        // Draw the background for selected item
                        if is_selected {
                            ui.painter().rect_filled(
                                rect,
                                egui::CornerRadius::same(2),
                                ui.visuals().selection.bg_fill,
                            );
                        } else if response.hovered() {
                            ui.painter().rect_filled(
                                rect,
                                egui::CornerRadius::same(2),
                                ui.visuals().widgets.hovered.bg_fill,
                            );
                        }

                        // Draw the text
                        let text_color = if is_selected {
                            ui.visuals().selection.stroke.color
                        } else {
                            ui.visuals().text_color()
                        };

                        ui.painter().text(
                            rect.left_center() + egui::vec2(ui.spacing().item_spacing.x, 0.0),
                            egui::Align2::LEFT_CENTER,
                            key,
                            egui::FontId::default(),
                            text_color,
                        );
                    }
                });
        });

    // New key dialog
    if app.new_key_dialog.show {
        let current_lang = {
            let tab = &app.tabs[app.active_tab];
            app.poll_language(tab.state.language.clone())
        };
        render_new_key_dialog(app, ctx, current_lang);
    }
}

fn render_new_key_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    let mut open = true;
    egui::Window::new(tr(keys::NEW_KEY, current_lang))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_pos(ctx.screen_rect().center())
        .show(ctx, |ui| {
            egui::Grid::new("new_key_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .min_col_width(80.0)
                .show(ui, |ui| {
                    // Key name
                    ui.label("Key:");
                    ui.add(
                        egui::TextEdit::singleline(&mut app.new_key_dialog.key_name)
                            .desired_width(250.0)
                            .hint_text("my_key"),
                    );
                    ui.end_row();

                    // Type selector
                    ui.label("Type:");
                    egui::ComboBox::from_id_salt("new_key_type")
                        .selected_text(&app.new_key_dialog.key_type)
                        .width(250.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "string".to_string(),
                                "string",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "hash".to_string(),
                                "hash",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "list".to_string(),
                                "list",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "set".to_string(),
                                "set",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "zset".to_string(),
                                "zset",
                            );
                        });
                    ui.end_row();

                    // TTL
                    ui.label("TTL:");
                    ui.add(
                        egui::TextEdit::singleline(&mut app.new_key_dialog.ttl)
                            .desired_width(250.0)
                            .hint_text("-1 (no expiration)"),
                    );
                    ui.end_row();
                });

            ui.separator();

            // Value input with hint based on type
            let hint = match app.new_key_dialog.key_type.as_str() {
                "string" => "Enter value",
                "list" => "One item per line",
                "set" => "One member per line",
                "hash" => "field:value per line",
                "zset" => "score:member per line",
                _ => "Enter value",
            };
            ui.label(format!("Value ({})", hint));
            ui.add_sized(
                [ui.available_width(), 120.0],
                egui::TextEdit::multiline(&mut app.new_key_dialog.value).hint_text(hint),
            );

            // Error message
            if !app.new_key_dialog.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &app.new_key_dialog.error_message);
            }

            ui.separator();

            // Buttons
            ui.horizontal(|ui| {
                if ui
                    .button(
                        egui::RichText::new(tr(keys::SAVE, current_lang))
                            .color(egui::Color32::from_rgb(50, 180, 50)),
                    )
                    .clicked()
                {
                    let key_name = app.new_key_dialog.key_name.trim().to_string();
                    if key_name.is_empty() {
                        app.new_key_dialog.error_message = "Key name cannot be empty".to_string();
                    } else {
                        let key_type = app.new_key_dialog.key_type.clone();
                        let value = app.new_key_dialog.value.clone();
                        let ttl: i64 = app.new_key_dialog.ttl.trim().parse().unwrap_or(-1);
                        let active_tab_idx = app.active_tab;
                        app.tabs[active_tab_idx]
                            .state
                            .spawn_create_new_key(key_name, key_type, value, ttl);
                        app.new_key_dialog.show = false;
                    }
                }

                if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                    app.new_key_dialog.show = false;
                }
            });
        });

    if !open {
        app.new_key_dialog.show = false;
    }
}
