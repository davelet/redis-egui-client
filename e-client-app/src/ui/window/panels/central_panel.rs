use crate::core::app_state::EditedValue;
use crate::core::redis_client::ValueData;
use crate::ui::window::RedisApp;
use e_client_basics::constants::{
    MAX_CENTRAL_PANEL_HEIGHT, MAX_KEY_DISPLAY_LENGTH, MIN_CENTRAL_PANEL_HEIGHT,
};
use e_client_config::config::Config;
use e_client_config::constants::APP_NAME;
use e_client_config::constants::LOAD_ERROR_TITLE;
use e_client_config::error::ConfigError;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;
use e_client_config::translations::tr_fmt;

fn truncate_key(key: &str) -> String {
    if key.len() > MAX_KEY_DISPLAY_LENGTH {
        format!("{}...", &key[..MAX_KEY_DISPLAY_LENGTH])
    } else {
        key.to_string()
    }
}

fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let char_count = text.chars().count();
    if char_count > max_chars {
        let truncated: String = text.chars().take(max_chars).collect();
        format!("{}...", truncated)
    } else {
        text.to_string()
    }
}

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
    let show_open_connections = app.show_open_connections_prompt;

    egui::CentralPanel::default().show(ctx, |ui| {
        if !connected {
            render_welcome_page(app, ui, ctx, current_lang, show_open_connections);
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
        let display_key = truncate_key(key);
        ui.heading(tr_fmt(keys::KEY_HEADING, current_lang, &[&display_key]));

        let copy_key_id = egui::Id::new("copy_key").with(key);
        let copy_key_color = app.copy_button_text_color(copy_key_id);
        let copy_key_text = app.copy_button_text(copy_key_id, tr(keys::COPY_KEY, current_lang));

        // Copy key
        if ui
            .button(egui::RichText::new(copy_key_text).color(copy_key_color))
            .clicked()
        {
            ctx.copy_text(key.to_string());
            app.record_copy_success_with_id(copy_key_id);
        }

        // Copy value (not for Hash type)
        if let Some(val) = value.as_ref() {
            let is_hash = matches!(val, ValueData::Hash { .. });
            if !is_hash {
                let copy_value_id = egui::Id::new("copy_value").with(key);
                let copy_value_color = app.copy_button_text_color(copy_value_id);
                let copy_value_text =
                    app.copy_button_text(copy_value_id, tr(keys::COPY_VALUE, current_lang));
                if ui
                    .button(egui::RichText::new(copy_value_text).color(copy_value_color))
                    .clicked()
                {
                    ctx.copy_text(value_to_copy_text(val));
                    app.record_copy_success_with_id(copy_value_id);
                }
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

        // Right side: TTL + refresh + TTL edit
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

            // TTL edit button
            let mut ttl_edit_mode = app.tabs[active_tab_idx]
                .state
                .ttl_edit_mode
                .blocking_write();
            let mut ttl_edit_value = app.tabs[active_tab_idx]
                .state
                .ttl_edit_value
                .blocking_write();

            if *ttl_edit_mode {
                // Show TTL input field
                if ttl_edit_value.is_empty() {
                    *ttl_edit_value = ttl.to_string();
                }
                if ui
                    .add(egui::TextEdit::singleline(&mut *ttl_edit_value).desired_width(80.0))
                    .changed()
                {
                    // Input changed, value is stored in ttl_edit_value
                }

                if ui.button(tr(keys::SAVE, current_lang)).clicked() {
                    if let Ok(new_ttl) = ttl_edit_value.parse::<i64>() {
                        app.tabs[active_tab_idx]
                            .state
                            .spawn_update_ttl(key.to_string(), new_ttl);
                    }
                    *ttl_edit_value = String::new();
                    *ttl_edit_mode = false;
                }
                if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                    *ttl_edit_value = String::new();
                    *ttl_edit_mode = false;
                }
            } else {
                if ui.button(tr(keys::EDIT_TTL, current_lang)).clicked() {
                    *ttl_edit_value = ttl.to_string();
                    *ttl_edit_mode = true;
                }
            }
            drop(ttl_edit_value);
            drop(ttl_edit_mode);
        });
    });

    // Value display
    if let Some(val) = value {
        render_value_view(app, ui, ctx, active_tab_idx, key, val, current_lang);
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
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Key:").strong());
        if ui
            .add(egui::TextEdit::singleline(&mut edit.edited_key).desired_width(300.0))
            .changed()
        {
            changed = true;
        }

        // Save button
        if ui
            .button(
                egui::RichText::new(tr(keys::SAVE, current_lang))
                    .color(egui::Color32::from_rgb(50, 180, 50)),
            )
            .clicked()
        {
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
                            // Add delete button with fixed width at the beginning
                            let delete_button_width = 60.0;
                            ui.add_sized(
                                [delete_button_width, 24.0],
                                egui::Button::new(
                                    egui::RichText::new(tr(keys::DELETE, current_lang))
                                        .color(egui::Color32::RED),
                                ),
                            )
                            .clicked()
                            .then(|| {
                                delete_idx = Some(idx);
                            });

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

                            let available_for_value =
                                ui.ctx().available_rect().width() - delete_button_width;

                            if ui
                                .add(
                                    egui::TextEdit::singleline(value)
                                        .desired_width(available_for_value)
                                        .hint_text("value"),
                                )
                                .changed()
                            {
                                changed = true;
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
                            // Add delete button with fixed width at the beginning
                            let delete_button_width = 60.0;
                            ui.add_sized(
                                [delete_button_width, 24.0],
                                egui::Button::new(
                                    egui::RichText::new(tr(keys::DELETE, current_lang))
                                        .color(egui::Color32::RED),
                                ),
                            )
                            .clicked()
                            .then(|| {
                                delete_idx = Some(idx);
                            });

                            ui.label(format!("[{}]", idx));

                            let available_for_value =
                                ui.ctx().available_rect().width() - delete_button_width;

                            if ui
                                .add(
                                    egui::TextEdit::singleline(item)
                                        .desired_width(available_for_value),
                                )
                                .changed()
                            {
                                changed = true;
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
                            // Add delete button with fixed width at the beginning
                            let delete_button_width = 60.0;
                            ui.add_sized(
                                [delete_button_width, 24.0],
                                egui::Button::new(
                                    egui::RichText::new(tr(keys::DELETE, current_lang))
                                        .color(egui::Color32::RED),
                                ),
                            )
                            .clicked()
                            .then(|| {
                                delete_idx = Some(idx);
                            });

                            ui.label(format!("{}.", idx));

                            let available_for_value =
                                ui.ctx().available_rect().width() - delete_button_width;

                            if ui
                                .add(
                                    egui::TextEdit::singleline(item)
                                        .desired_width(available_for_value),
                                )
                                .changed()
                            {
                                changed = true;
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
                            // Add delete button with fixed width at the beginning
                            let delete_button_width = 60.0;
                            ui.add_sized(
                                [delete_button_width, 24.0],
                                egui::Button::new(
                                    egui::RichText::new(tr(keys::DELETE, current_lang))
                                        .color(egui::Color32::RED),
                                ),
                            )
                            .clicked()
                            .then(|| {
                                delete_idx = Some(idx);
                            });

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

                            let available_for_value =
                                ui.ctx().available_rect().width() - delete_button_width;

                            if ui
                                .add(
                                    egui::TextEdit::singleline(member)
                                        .desired_width(available_for_value),
                                )
                                .changed()
                            {
                                changed = true;
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
    ctx: &egui::Context,
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
                        use egui_extras::Column;
                        let (saved_field_width, saved_value_width) = app.get_hash_column_widths();
                        let copy_button_width = 80.0;
                        let field_width = saved_field_width as f32;
                        let value_width = saved_value_width as f32;

                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(
                                Column::initial(field_width)
                                    .at_least(80.0)
                                    .at_most(400.0)
                                    .clip(true),
                            )
                            .column(Column::initial(value_width).at_least(100.0).clip(true))
                            .column(Column::exact(copy_button_width))
                            .resizable(true)
                            .min_scrolled_height(0.0)
                            .header(24.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Field");
                                });
                                header.col(|ui| {
                                    ui.strong("Value");
                                });
                                header.col(|_ui| {});
                            })
                            .body(|mut body| {
                                let widths = body.widths();
                                let field_width = widths.get(0).copied().unwrap_or(field_width);
                                let value_width = widths.get(1).copied().unwrap_or(value_width);

                                app.save_hash_column_widths(field_width as u32, value_width as u32);

                                for field in filtered_fields {
                                    let value = loaded_values.get(field).cloned();
                                    let display_field = truncate_key(field);
                                    body.row(24.0, |mut row| {
                                        // Left: Field name
                                        row.col(|ui| {
                                            ui.label(format!("{}", display_field));
                                        });

                                        // Middle: Value or load button
                                        row.col(|ui| match value {
                                            Some(v) => {
                                                let text = egui::RichText::new(
                                                    truncate_with_ellipsis(&v, 150),
                                                );
                                                let response = ui.selectable_label(false, text);
                                                if response.clicked() {
                                                    app.element_edit_dialog.key = key.to_string();
                                                    app.element_edit_dialog.field =
                                                        field.to_string();
                                                    app.element_edit_dialog.value = v.clone();
                                                    app.element_edit_dialog.original_value =
                                                        v.clone();
                                                    app.element_edit_dialog.key_type =
                                                        "hash".to_string();
                                                    app.element_edit_dialog.show = true;
                                                    app.element_edit_dialog.just_opened = true;
                                                }
                                            }
                                            None => {
                                                if ui
                                                    .button(tr(keys::LOAD_FIELDS, current_lang))
                                                    .clicked()
                                                {
                                                    app.tabs[active_tab_idx]
                                                        .state
                                                        .spawn_load_hash_field_value(
                                                            key.to_string(),
                                                            field.clone(),
                                                        );
                                                }
                                            }
                                        });

                                        // Right: Copy field button
                                        row.col(|ui| {
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    let copy_field_id =
                                                        egui::Id::new("copy_hash_field")
                                                            .with(key)
                                                            .with(field);
                                                    let copy_field_color =
                                                        app.copy_button_text_color(copy_field_id);
                                                    let copy_field_text = app.copy_button_text(
                                                        copy_field_id,
                                                        tr(keys::COPY_KEY, current_lang),
                                                    );
                                                    if ui
                                                        .button(
                                                            egui::RichText::new(copy_field_text)
                                                                .color(copy_field_color),
                                                        )
                                                        .clicked()
                                                    {
                                                        ctx.copy_text(field.to_string());
                                                        app.record_copy_success_with_id(
                                                            copy_field_id,
                                                        );
                                                    }
                                                },
                                            );
                                        });
                                    });
                                }
                            });
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

        let available_height = ui.available_height() - 15.0; // Reserve space for buttons
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

            if ui
                .button(
                    egui::RichText::new(app.copy_button_text(
                        egui::Id::new("copy_edit_dialog_value"),
                        tr(keys::COPY_VALUE, current_lang),
                    ))
                    .color(app.copy_button_text_color(egui::Id::new("copy_edit_dialog_value"))),
                )
                .clicked()
            {
                ctx.copy_text(app.element_edit_dialog.value.clone());
                app.record_copy_success_with_id(egui::Id::new("copy_edit_dialog_value"));
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

fn render_welcome_page(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    _ctx: &egui::Context,
    current_lang: Language,
    show_open_connections: bool,
) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);

        // Welcome title
        ui.heading(egui::RichText::new(tr(keys::WELCOME_TITLE, current_lang)).size(32.0));
        ui.add_space(20.0);

        // Welcome message
        ui.label(egui::RichText::new(tr(keys::WELCOME_MESSAGE, current_lang)).size(16.0));
        ui.add_space(40.0);

        // Get started button
        if ui
            .button(
                egui::RichText::new(tr(keys::NEW_CONNECTION, current_lang))
                    .size(18.0)
                    .color(egui::Color32::WHITE),
            )
            .clicked()
        {
            app.new_connection.show = true;
        }

        ui.add_space(20.0);

        // Instructions
        ui.label(
            egui::RichText::new(tr(keys::WELCOME_INSTRUCTION, current_lang))
                .weak()
                .size(14.0),
        );

        // Show open connections section if there are any
        if show_open_connections {
            ui.add_space(40.0);
            ui.separator();
            ui.add_space(20.0);

            // Open connections prompt
            ui.label(
                egui::RichText::new(tr(keys::OPEN_CONNECTIONS_PROMPT_MESSAGE, current_lang))
                    .size(16.0),
            );
            ui.add_space(10.0);

            // Get open connections
            let open_conn_names: Vec<_> =
                app.config.window.open_connections.connection_names.clone();
            let connections: Vec<_> = app.config.connections.connections.iter().cloned().collect();

            // List open connections
            for conn_name in &open_conn_names {
                ui.horizontal(|ui| {
                    ui.label(conn_name);
                    if ui.button(tr(keys::CONNECT, current_lang)).clicked() {
                        // Find the connection in the connections list
                        if let Some(conn_idx) =
                            connections.iter().position(|c| &c.name == conn_name)
                        {
                            let conn = connections[conn_idx].clone();
                            app.create_tab_with_connection(conn_idx, conn);
                            app.show_open_connections_prompt = false;
                            app.config.clear_open_connections();
                        }
                    }
                });
            }

            ui.add_space(10.0);

            // Connect All button
            if ui.button(tr(keys::CONNECT_ALL, current_lang)).clicked() {
                for conn_name in &open_conn_names {
                    if let Some(conn_idx) = connections.iter().position(|c| &c.name == conn_name) {
                        let conn = connections[conn_idx].clone();
                        app.create_tab_with_connection(conn_idx, conn);
                    }
                }
                app.show_open_connections_prompt = false;
                app.config.clear_open_connections();
            }

            // Close button
            if ui.button(tr(keys::CLOSE, current_lang)).clicked() {
                app.show_open_connections_prompt = false;
                app.config.clear_open_connections();
            }
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
