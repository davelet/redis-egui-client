use crate::core::ValueData;
use crate::ui::window::RedisApp;
use e_client_basics::constants::{MAX_CENTRAL_PANEL_HEIGHT, MIN_CENTRAL_PANEL_HEIGHT};
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};

use super::utils::truncate_with_ellipsis;

/// Render value display based on data type
pub fn render_value_view(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    val: &ValueData,
    current_lang: Language,
) {
    match val {
        ValueData::String(s) => render_string_value(ui, s, current_lang),
        ValueData::List { len, items } => {
            render_list_value(app, ui, active_tab_idx, key, len, items, current_lang)
        }
        ValueData::Hash {
            len,
            fields,
            loaded_values,
        } => render_hash_value(
            app,
            ui,
            ctx,
            active_tab_idx,
            key,
            len,
            fields,
            loaded_values,
            current_lang,
        ),
        ValueData::Set { len, items } => {
            render_set_value(app, ui, ctx, active_tab_idx, key, len, items, current_lang)
        }
        ValueData::ZSet { len, items } => {
            render_zset_value(app, ui, ctx, active_tab_idx, key, len, items, current_lang)
        }
        ValueData::None => {
            ui.label(tr(TranslationKey::KeyNotExist, current_lang));
        }
    }
}

/// Render string value
fn render_string_value(ui: &mut egui::Ui, s: &str, current_lang: Language) {
    ui.label(tr(TranslationKey::TypeString, current_lang));
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let available = ui.available_size();
            let desired_height = available
                .y
                .max(MIN_CENTRAL_PANEL_HEIGHT)
                .min(MAX_CENTRAL_PANEL_HEIGHT);
            let mut s_copy = s.to_string();
            ui.add_sized(
                [available.x, desired_height],
                egui::TextEdit::multiline(&mut s_copy),
            );
        });
}

/// Render list value
fn render_list_value(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    active_tab_idx: usize,
    key: &str,
    len: &usize,
    items: &[String],
    current_lang: Language,
) {
    ui.label(tr_fmt(
        TranslationKey::TypeList,
        current_lang,
        &[&len.to_string()],
    ));

    if items.is_empty() && *len > 0 {
        let auto_expand = app.config.settings.auto_expand;
        let threshold = app.config.settings.auto_expand_threshold;
        if auto_expand && *len <= threshold {
            // Auto-load all items when auto-expand is enabled and within threshold
            app.tabs[active_tab_idx].state.spawn_load_list_range(
                key.to_string(),
                0,
                ((*len).saturating_sub(1)) as isize,
            );
        } else if ui
            .button(tr(TranslationKey::LoadFirst100, current_lang))
            .clicked()
        {
            app.tabs[active_tab_idx]
                .state
                .spawn_load_list_range(key.to_string(), 0, 99);
        }
    } else {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, item) in items.iter().enumerate() {
                let response = ui.selectable_label(false, format!("[{}] {}", idx, item));
                if response.clicked() {
                    open_element_edit_dialog(app, key, &idx.to_string(), item, "list");
                }
            }
        });
    }
}

/// Render hash value
fn render_hash_value(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    _len: &usize,
    _fields: &[String],
    _loaded_values: &std::collections::HashMap<String, String>,
    current_lang: Language,
) {
    // Get fresh values from app_state to ensure we have the latest data
    let (len, fields, loaded_values) = {
        let value = app.tabs[active_tab_idx].state.key_value.blocking_read();
        if let Some(ValueData::Hash {
            len,
            fields,
            loaded_values,
        }) = value.as_ref()
        {
            (*len, fields.clone(), loaded_values.clone())
        } else {
            (0, vec![], std::collections::HashMap::new())
        }
    };

    // Snapshot of fields currently being fetched, so the per-row button can
    // render its "Loading..." state and reject duplicate clicks.
    let loading_fields: std::collections::HashSet<(String, String)> = app.tabs[active_tab_idx]
        .state
        .loading_hash_fields
        .try_read()
        .map(|g| g.clone())
        .unwrap_or_default();

    ui.label(tr_fmt(
        TranslationKey::TypeHash,
        current_lang,
        &[&len.to_string()],
    ));

    // Field filter input
    ui.horizontal(|ui| {
        ui.label(tr(TranslationKey::Filter, current_lang));
        let mut filter_text = app.poll_string_hash_field_filter();
        if ui.text_edit_singleline(&mut filter_text).changed() {
            app.set_hash_field_filter(filter_text);
        }
    });

    let auto_expand = app.config.settings.auto_expand;
    let threshold = app.config.settings.auto_expand_threshold;

    if fields.is_empty() {
        if len > 0 {
            if auto_expand && len <= threshold {
                // Auto-load fields when auto-expand is enabled and len is within threshold
                app.tabs[active_tab_idx]
                    .state
                    .spawn_load_hash_fields(key.to_string());
            } else if ui
                .button(tr(TranslationKey::LoadFields, current_lang))
                .clicked()
            {
                app.tabs[active_tab_idx]
                    .state
                    .spawn_load_hash_fields(key.to_string());
            }
        }
    } else {
        // Auto-load all hash field values when auto-expand is enabled and within threshold
        if auto_expand && len <= threshold && loaded_values.len() < len {
            app.tabs[active_tab_idx]
                .state
                .spawn_load_all_hash_field_values(key.to_string());
        }

        let filter = app.poll_string_hash_field_filter();

        let filtered_fields: Vec<&String> = fields
            .iter()
            .filter(|f| filter.is_empty() || f.to_lowercase().contains(&filter.to_lowercase()))
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
                            ui.strong(tr(TranslationKey::HashField, current_lang));
                        });
                        header.col(|ui| {
                            ui.strong(tr(TranslationKey::HashValue, current_lang));
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
                            let display_field =
                                crate::ui::window::panels::central_panel::utils::truncate_key(
                                    field,
                                );
                            body.row(24.0, |mut row| {
                                // Left: Field name
                                row.col(|ui| {
                                    ui.label(format!("{}", display_field));
                                });

                                // Middle: Value or load button
                                row.col(|ui| match value {
                                    Some(v) => {
                                        let text =
                                            egui::RichText::new(truncate_with_ellipsis(&v, 150));
                                        let response = ui.selectable_label(false, text);
                                        if response.clicked() {
                                            open_element_edit_dialog(app, key, field, &v, "hash");
                                        }
                                    }
                                    None => {
                                        let is_loading = loading_fields
                                            .contains(&(key.to_string(), field.clone()));
                                        let label = if is_loading {
                                            tr(TranslationKey::Loading, current_lang)
                                        } else {
                                            tr(TranslationKey::LoadFields, current_lang)
                                        };
                                        let btn = egui::Button::new(label);
                                        // `add_enabled(!is_loading, ...)` greys out
                                        // and ignores clicks while a fetch is in
                                        // flight. The redundant `!is_loading` in the
                                        // `clicked()` branch is defense-in-depth: the
                                        // snapshot we read at the top of this frame can
                                        // race with a background task inserting into
                                        // `loading_hash_fields`, so we re-check the
                                        // latest snapshot before spawning another
                                        // `spawn_load_hash_field_value` task.
                                        let response = ui.add_enabled(!is_loading, btn);
                                        if response.clicked() && !is_loading {
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
                                            let copy_field_id = egui::Id::new("copy_hash_field")
                                                .with(key)
                                                .with(field);
                                            let copy_field_color =
                                                app.copy_button_text_color(copy_field_id);
                                            let copy_field_text = app.copy_button_text(
                                                copy_field_id,
                                                tr(TranslationKey::CopyKey, current_lang),
                                            );
                                            if ui
                                                .button(
                                                    egui::RichText::new(copy_field_text)
                                                        .color(copy_field_color),
                                                )
                                                .clicked()
                                            {
                                                ctx.copy_text(field.to_string());
                                                app.record_copy_success_with_id(copy_field_id);
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

/// Render set value
fn render_set_value(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    _ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    len: &usize,
    items: &[String],
    current_lang: Language,
) {
    ui.label(tr_fmt(
        TranslationKey::TypeSet,
        current_lang,
        &[&len.to_string()],
    ));

    if items.is_empty() && *len > 0 {
        let auto_expand = app.config.settings.auto_expand;
        let threshold = app.config.settings.auto_expand_threshold;
        if auto_expand && *len <= threshold {
            // Auto-load all members when auto-expand is enabled and within threshold
            app.tabs[active_tab_idx]
                .state
                .spawn_load_set_members(key.to_string());
        } else if ui
            .button(tr(TranslationKey::LoadMembers, current_lang))
            .clicked()
        {
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
                        open_element_edit_dialog(app, key, item, item, "set");
                    }
                }
            });
    }
}

/// Render sorted set value
fn render_zset_value(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    _ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    len: &usize,
    items: &[(String, f64)],
    current_lang: Language,
) {
    ui.label(tr_fmt(
        TranslationKey::TypeZset,
        current_lang,
        &[&len.to_string()],
    ));

    if items.is_empty() && *len > 0 {
        let auto_expand = app.config.settings.auto_expand;
        let threshold = app.config.settings.auto_expand_threshold;
        if auto_expand && *len <= threshold {
            // Auto-load all items when auto-expand is enabled and within threshold
            app.tabs[active_tab_idx].state.spawn_load_zset_range(
                key.to_string(),
                0,
                ((*len).saturating_sub(1)) as isize,
            );
        } else if ui
            .button(tr(TranslationKey::LoadMembers, current_lang))
            .clicked()
        {
            app.tabs[active_tab_idx]
                .state
                .spawn_load_zset_range(key.to_string(), 0, 99);
        }
    } else {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (member, score) in items.iter() {
                    let response = ui.selectable_label(false, format!("{}: {}", score, member));
                    if response.clicked() {
                        open_element_edit_dialog(app, key, member, &score.to_string(), "zset");
                    }
                }
            });
    }
}

/// Open element edit dialog for the specified element
fn open_element_edit_dialog(
    app: &mut RedisApp,
    key: &str,
    field: &str,
    value: &str,
    key_type: &str,
) {
    app.element_edit_dialog.key = key.to_string();
    app.element_edit_dialog.field = field.to_string();
    app.element_edit_dialog.value = value.to_string();
    app.element_edit_dialog.original_value = value.to_string();
    app.element_edit_dialog.key_type = key_type.to_string();
    app.element_edit_dialog.show = true;
    app.element_edit_dialog.just_opened = true;
}
