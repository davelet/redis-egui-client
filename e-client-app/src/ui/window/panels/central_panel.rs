use crate::core::app_state::EditedValue;
use crate::core::redis_client::ValueData;
use crate::ui::window::RedisApp;
use e_client_basics::constants::{MAX_CENTRAL_PANEL_HEIGHT, MIN_CENTRAL_PANEL_HEIGHT};
use e_client_config::config::Config;
use e_client_config::constants::APP_NAME;
use e_client_config::constants::LOAD_ERROR_TITLE;
use e_client_config::error::ConfigError;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;
use e_client_config::translations::tr_fmt;

fn format_ttl(ttl: i64) -> String {
    match ttl {
        -1 => "ttl: -1".to_string(),
        -2 => String::new(),
        t if t >= 100 => {
            let days = t / 86400;
            let hours = (t % 86400) / 3600;
            let minutes = (t % 3600) / 60;
            let seconds = t % 60;
            if days > 0 {
                format!("ttl: {}d {}h {}m {}s", days, hours, minutes, seconds)
            } else if hours > 0 {
                format!("ttl: {}h {}m {}s", hours, minutes, seconds)
            } else {
                format!("ttl: {}m {}s", minutes, seconds)
            }
        }
        t if t >= 0 => format!("ttl: {}s", t),
        t => format!("ttl: {}", t),
    }
}

fn value_to_copy_text(val: &ValueData) -> String {
    match val {
        ValueData::String(s) => s.clone(),
        ValueData::List { items, .. } => items.join("\n"),
        ValueData::Hash {
            fields,
            loaded_values,
            ..
        } => fields
            .iter()
            .map(|k| {
                format!(
                    "{}: {}",
                    k,
                    loaded_values.get(k).cloned().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        ValueData::Set { items, .. } => items.join("\n"),
        ValueData::ZSet { items, .. } => items
            .iter()
            .map(|(m, s)| format!("{} ({})", m, s))
            .collect::<Vec<_>>()
            .join("\n"),
        ValueData::None => String::new(),
    }
}

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
    let ttl = app.poll_i64(tab.state.key_ttl.clone());
    let edit_state = tab.state.edit_state.blocking_read().clone();

    egui::CentralPanel::default().show(ctx, |ui| {
        if !connected {
            return;
        }

        if let Some(key) = selected_key {
            if edit_state.editing {
                render_edit_mode(
                    app,
                    ui,
                    ctx,
                    active_tab_idx,
                    &key,
                    &value,
                    ttl,
                    current_lang,
                );
            } else {
                render_view_mode(
                    app,
                    ui,
                    ctx,
                    active_tab_idx,
                    &key,
                    &value,
                    ttl,
                    current_lang,
                );
            }
        } else {
            ui.label(tr(keys::SELECT_KEY_PROMPT, current_lang));
        }
    });

    // Element edit dialog
    render_element_edit_dialog(app, ctx, current_lang);
}

fn render_view_mode(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    value: &Option<ValueData>,
    ttl: i64,
    current_lang: Language,
) {
    // Header row
    ui.horizontal(|ui| {
        ui.heading(tr_fmt(keys::KEY_HEADING, current_lang, &[key]));

        // Copy key
        if ui.button(tr(keys::COPY_KEY, current_lang)).clicked() {
            ctx.copy_text(key.to_string());
        }

        // Copy value
        if let Some(val) = value.as_ref() {
            if ui.button(tr(keys::COPY_VALUE, current_lang)).clicked() {
                ctx.copy_text(value_to_copy_text(val));
            }
        }

        // Edit button
        if ui.button(tr(keys::EDIT, current_lang)).clicked() {
            app.tabs[active_tab_idx]
                .state
                .edit_state
                .blocking_write()
                .enter_edit(key, ttl, value);
        }

        // Delete button (always visible)
        if ui
            .button(egui::RichText::new(tr(keys::DELETE, current_lang)).color(egui::Color32::RED))
            .clicked()
        {
            app.tabs[active_tab_idx]
                .state
                .spawn_delete_key(key.to_string());
        }

        // Right side: TTL + refresh
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🔄").clicked() {
                app.tabs[active_tab_idx]
                    .state
                    .spawn_load_value(key.to_string());
            }

            let ttl_text = format_ttl(ttl);
            if !ttl_text.is_empty() {
                ui.label(ttl_text);
            }
        });
    });

    // Value display
    if let Some(val) = value {
        render_value_view(app, ui, active_tab_idx, key, val, current_lang);
    }
}

fn render_edit_mode(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    original_key: &str,
    _value: &Option<ValueData>,
    _ttl: i64,
    current_lang: Language,
) {
    let mut edit = app.tabs[active_tab_idx]
        .state
        .edit_state
        .blocking_write()
        .clone();

    // Header row with key editing
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Key:").strong());
        ui.add(egui::TextEdit::singleline(&mut edit.edited_key).desired_width(200.0));

        ui.label(egui::RichText::new("TTL:").strong());
        ui.add(
            egui::TextEdit::singleline(&mut edit.edited_ttl)
                .desired_width(80.0)
                .hint_text("-1"),
        );

        // Save button
        if ui
            .button(
                egui::RichText::new(tr(keys::SAVE, current_lang))
                    .color(egui::Color32::from_rgb(50, 180, 50)),
            )
            .clicked()
        {
            // Write back edit state before saving
            *app.tabs[active_tab_idx].state.edit_state.blocking_write() = edit.clone();
            app.tabs[active_tab_idx]
                .state
                .spawn_save_edits(original_key.to_string());
            return;
        }

        // Cancel button
        if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
            app.tabs[active_tab_idx]
                .state
                .edit_state
                .blocking_write()
                .cancel_edit();
            return;
        }

        // Delete button
        if ui
            .button(egui::RichText::new(tr(keys::DELETE, current_lang)).color(egui::Color32::RED))
            .clicked()
        {
            app.tabs[active_tab_idx]
                .state
                .spawn_delete_key(original_key.to_string());
            return;
        }
    });

    // Save message
    if !edit.save_message.is_empty() {
        ui.colored_label(egui::Color32::RED, &edit.save_message);
    }

    ui.separator();

    // Value editing by type
    let mut changed = false;
    match &mut edit.edited_value {
        EditedValue::String(s) => {
            ui.label(tr(keys::TYPE_STRING, current_lang));
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let available = ui.available_size();
                    let desired_height = available
                        .y
                        .max(MIN_CENTRAL_PANEL_HEIGHT)
                        .min(MAX_CENTRAL_PANEL_HEIGHT);
                    if ui
                        .add_sized([available.x, desired_height], egui::TextEdit::multiline(s))
                        .changed()
                    {
                        changed = true;
                    }
                });
        }
        EditedValue::Hash(fields) => {
            ui.label(tr_fmt(
                keys::TYPE_HASH,
                current_lang,
                &[&fields.len().to_string()],
            ));

            let mut delete_idx: Option<usize> = None;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (idx, (field, value)) in fields.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx));
                            if ui
                                .add(
                                    egui::TextEdit::singleline(field)
                                        .desired_width(150.0)
                                        .hint_text("field"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            ui.label(":");
                            if ui
                                .add(
                                    egui::TextEdit::singleline(value)
                                        .desired_width(ui.available_width() - 40.0)
                                        .hint_text("value"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .button(egui::RichText::new("🗑️").color(egui::Color32::RED))
                                .clicked()
                            {
                                delete_idx = Some(idx);
                            }
                        });
                    }

                    if ui.button(tr(keys::ADD_FIELD, current_lang)).clicked() {
                        fields.push((String::new(), String::new()));
                        changed = true;
                    }
                });

            if let Some(idx) = delete_idx {
                fields.remove(idx);
                changed = true;
            }
        }
        EditedValue::List(items) => {
            ui.label(tr_fmt(
                keys::TYPE_LIST,
                current_lang,
                &[&items.len().to_string()],
            ));

            let mut delete_idx: Option<usize> = None;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (idx, item) in items.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("[{}]", idx));
                            if ui
                                .add(
                                    egui::TextEdit::singleline(item)
                                        .desired_width(ui.available_width() - 40.0),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .button(egui::RichText::new("🗑️").color(egui::Color32::RED))
                                .clicked()
                            {
                                delete_idx = Some(idx);
                            }
                        });
                    }

                    if ui.button(tr(keys::ADD_ITEM, current_lang)).clicked() {
                        items.push(String::new());
                        changed = true;
                    }
                });

            if let Some(idx) = delete_idx {
                items.remove(idx);
                changed = true;
            }
        }
        EditedValue::Set(items) => {
            ui.label(tr_fmt(
                keys::TYPE_SET,
                current_lang,
                &[&items.len().to_string()],
            ));

            let mut delete_idx: Option<usize> = None;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (idx, item) in items.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx));
                            if ui
                                .add(
                                    egui::TextEdit::singleline(item)
                                        .desired_width(ui.available_width() - 40.0),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .button(egui::RichText::new("🗑️").color(egui::Color32::RED))
                                .clicked()
                            {
                                delete_idx = Some(idx);
                            }
                        });
                    }

                    if ui.button("+ Add Member").clicked() {
                        items.push(String::new());
                        changed = true;
                    }
                });

            if let Some(idx) = delete_idx {
                items.remove(idx);
                changed = true;
            }
        }
        EditedValue::ZSet(items) => {
            ui.label(tr_fmt(
                keys::TYPE_ZSET,
                current_lang,
                &[&items.len().to_string()],
            ));

            let mut delete_idx: Option<usize> = None;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (idx, (member, score)) in items.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx));
                            ui.label("score:");
                            if ui
                                .add(
                                    egui::TextEdit::singleline(score)
                                        .desired_width(80.0)
                                        .hint_text("0.0"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            ui.label("member:");
                            if ui
                                .add(
                                    egui::TextEdit::singleline(member)
                                        .desired_width(ui.available_width() - 40.0),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .button(egui::RichText::new("🗑️").color(egui::Color32::RED))
                                .clicked()
                            {
                                delete_idx = Some(idx);
                            }
                        });
                    }

                    if ui.button("+ Add Member").clicked() {
                        items.push((String::new(), "0".to_string()));
                        changed = true;
                    }
                });

            if let Some(idx) = delete_idx {
                items.remove(idx);
                changed = true;
            }
        }
        EditedValue::None => {
            ui.label("No value to edit");
        }
    }

    // Write back edit state if anything changed
    if changed {
        *app.tabs[active_tab_idx].state.edit_state.blocking_write() = edit;
    }
}

fn render_value_view(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    active_tab_idx: usize,
    key: &str,
    val: &ValueData,
    current_lang: Language,
) {
    match val {
        ValueData::String(s) => {
            ui.label(tr(keys::TYPE_STRING, current_lang));
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let available = ui.available_size();
                    let desired_height = available
                        .y
                        .max(MIN_CENTRAL_PANEL_HEIGHT)
                        .min(MAX_CENTRAL_PANEL_HEIGHT);
                    ui.add_sized(
                        [available.x, desired_height],
                        egui::TextEdit::multiline(&mut s.as_str()),
                    );
                });
        }
        ValueData::List { len, items } => {
            ui.label(tr_fmt(keys::TYPE_LIST, current_lang, &[&len.to_string()]));

            if items.is_empty() && *len > 0 {
                if ui.button(tr(keys::LOAD_FIRST_100, current_lang)).clicked() {
                    app.tabs[active_tab_idx]
                        .state
                        .spawn_load_list_range(key.to_string(), 0, 99);
                }
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, item) in items.iter().enumerate() {
                        let response = ui.selectable_label(false, format!("[{}] {}", idx, item));
                        if response.clicked() {
                            app.element_edit_dialog.key = key.to_string();
                            app.element_edit_dialog.field = idx.to_string();
                            app.element_edit_dialog.value = item.clone();
                            app.element_edit_dialog.original_value = item.clone();
                            app.element_edit_dialog.key_type = "list".to_string();
                            app.element_edit_dialog.show = true;
                            app.element_edit_dialog.just_opened = true;
                        }
                    }
                });
            }
        }
        ValueData::Hash {
            len,
            fields,
            loaded_values,
        } => {
            ui.label(tr_fmt(keys::TYPE_HASH, current_lang, &[&len.to_string()]));

            // Field filter input
            ui.horizontal(|ui| {
                ui.label(tr(keys::FILTER, current_lang));
                let mut filter_text = app.poll_string_hash_field_filter();
                if ui.text_edit_singleline(&mut filter_text).changed() {
                    app.set_hash_field_filter(filter_text);
                }
            });

            if fields.is_empty() {
                if *len > 0 {
                    if ui.button(tr(keys::LOAD_FIELDS, current_lang)).clicked() {
                        app.tabs[active_tab_idx]
                            .state
                            .spawn_load_hash_fields(key.to_string());
                    }
                }
            } else {
                let filter = app.poll_string_hash_field_filter();

                let filtered_fields: Vec<&String> = fields
                    .iter()
                    .filter(|f| {
                        filter.is_empty() || f.to_lowercase().contains(&filter.to_lowercase())
                    })
                    .collect();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for field in filtered_fields {
                            let value = loaded_values.get(field).cloned();
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{}: ", field)).strong());
                                match value {
                                    Some(v) => {
                                        let v_clone = v.clone();
                                        let response = ui.selectable_label(false, &v_clone);
                                        if response.clicked() {
                                            app.element_edit_dialog.key = key.to_string();
                                            app.element_edit_dialog.field = field.to_string();
                                            app.element_edit_dialog.value = v_clone;
                                            app.element_edit_dialog.original_value = v.clone();
                                            app.element_edit_dialog.key_type = "hash".to_string();
                                            app.element_edit_dialog.show = true;
                                            app.element_edit_dialog.just_opened = true;
                                        }
                                    }
                                    None => {
                                        if ui.button(tr(keys::LOAD_FIELDS, current_lang)).clicked()
                                        {
                                            app.tabs[active_tab_idx]
                                                .state
                                                .spawn_load_hash_field_value(
                                                    key.to_string(),
                                                    field.clone(),
                                                );
                                        }
                                    }
                                }
                            });
                            ui.separator();
                        }
                    });
            }
        }
        ValueData::Set { len, items } => {
            ui.label(tr_fmt(keys::TYPE_SET, current_lang, &[&len.to_string()]));

            if items.is_empty() && *len > 0 {
                if ui.button(tr(keys::LOAD_MEMBERS, current_lang)).clicked() {
                    app.tabs[active_tab_idx]
                        .state
                        .spawn_load_set_members(key.to_string());
                }
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for item in items.iter() {
                            let response = ui.selectable_label(false, item);
                            if response.clicked() {
                                app.element_edit_dialog.key = key.to_string();
                                app.element_edit_dialog.field = item.clone();
                                app.element_edit_dialog.value = item.clone();
                                app.element_edit_dialog.original_value = item.clone();
                                app.element_edit_dialog.key_type = "set".to_string();
                                app.element_edit_dialog.show = true;
                                app.element_edit_dialog.just_opened = true;
                            }
                        }
                    });
            }
        }
        ValueData::ZSet { len, items } => {
            ui.label(tr_fmt(keys::TYPE_ZSET, current_lang, &[&len.to_string()]));

            if items.is_empty() && *len > 0 {
                if ui.button(tr(keys::LOAD_MEMBERS, current_lang)).clicked() {
                    app.tabs[active_tab_idx]
                        .state
                        .spawn_load_zset_range(key.to_string(), 0, 99);
                }
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (member, score) in items.iter() {
                            let response =
                                ui.selectable_label(false, format!("{}: {}", score, member));
                            if response.clicked() {
                                app.element_edit_dialog.key = key.to_string();
                                app.element_edit_dialog.field = member.clone();
                                app.element_edit_dialog.value = score.to_string();
                                app.element_edit_dialog.original_value = score.to_string();
                                app.element_edit_dialog.key_type = "zset".to_string();
                                app.element_edit_dialog.show = true;
                                app.element_edit_dialog.just_opened = true;
                            }
                        }
                    });
            }
        }
        ValueData::None => {
            ui.label(tr(keys::KEY_NOT_EXIST, current_lang));
        }
    }
}

pub fn render_element_edit_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    if !app.element_edit_dialog.show {
        return;
    }

    let mut open = true;
    let key_type = app.element_edit_dialog.key_type.clone();

    // Calculate dialog position only when dialog just opened
    let dialog_size = egui::vec2(800.0, 600.0);
    let window_builder = if app.element_edit_dialog.just_opened {
        // Auto-format JSON if valid
        if let Ok(json_value) =
            serde_json::from_str::<serde_json::Value>(&app.element_edit_dialog.value)
        {
            if let Ok(formatted) = serde_json::to_string_pretty(&json_value) {
                app.element_edit_dialog.value = formatted;
            }
        }

        app.element_edit_dialog.just_opened = false;

        // Center on screen
        let screen_rect = ctx.screen_rect();
        let dialog_pos = screen_rect.center() - dialog_size * 0.5;

        egui::Window::new(tr(keys::EDIT_ELEMENT, current_lang))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(dialog_size)
            .default_pos(dialog_pos)
    } else {
        egui::Window::new(tr(keys::EDIT_ELEMENT, current_lang))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(dialog_size)
    };

    window_builder.show(ctx, |ui| {
        // Show context info
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Key:").strong());
            egui::ScrollArea::horizontal()
                .max_height(20.0)
                .show(ui, |ui| {
                    ui.label(&app.element_edit_dialog.key);
                });
        });

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Type:").strong());
            ui.label(&key_type);
        });

        if key_type == "hash" || key_type == "list" {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(if key_type == "hash" {
                        "Field:"
                    } else {
                        "Index:"
                    })
                    .strong(),
                );
                ui.label(&app.element_edit_dialog.field);
            });
        }

        ui.separator();

        // Value editor - try to format as JSON for display
        let display_value = if let Ok(json_value) =
            serde_json::from_str::<serde_json::Value>(&app.element_edit_dialog.value)
        {
            if let Ok(formatted) = serde_json::to_string_pretty(&json_value) {
                formatted
            } else {
                app.element_edit_dialog.value.clone()
            }
        } else {
            app.element_edit_dialog.value.clone()
        };

        // Value editor
        ui.label(egui::RichText::new("Value:").strong());
        let available_height = ui.available_height() - 60.0; // Reserve space for buttons
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width(), available_height],
                    egui::TextEdit::multiline(&mut app.element_edit_dialog.value).code_editor(),
                );
            });

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
                let active_tab_idx = app.active_tab;
                app.tabs[active_tab_idx].state.spawn_save_element(
                    app.element_edit_dialog.key.clone(),
                    app.element_edit_dialog.key_type.clone(),
                    app.element_edit_dialog.field.clone(),
                    app.element_edit_dialog.value.clone(),
                );
                app.element_edit_dialog.show = false;
            }

            if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                app.element_edit_dialog.show = false;
            }
        });
    });

    if !open {
        app.element_edit_dialog.show = false;
    }
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
