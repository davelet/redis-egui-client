use crate::core::EditedValue;
use crate::ui::window::RedisApp;
use e_client_basics::constants::{MAX_CENTRAL_PANEL_HEIGHT, MIN_CENTRAL_PANEL_HEIGHT};
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr, tr_fmt};
use Language::*;

/// Render key in edit mode
pub fn render_edit_mode(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    active_tab_idx: usize,
    original_key: &str,
    _value: &Option<crate::core::ValueData>,
    _ttl: i64,
    current_lang: Language,
) {
    let mut edit = app.tabs[active_tab_idx]
        .state
        .edit_state
        .blocking_write()
        .clone();

    let key_changed = render_header(
        app,
        ui,
        active_tab_idx,
        original_key,
        &mut edit,
        current_lang,
    );

    ui.separator();

    let editable = !edit.saving;
    let value_changed = render_value_edit(ui, &mut edit.edited_value, editable, current_lang);

    // Write back edit state if anything changed
    if key_changed || value_changed {
        *app.tabs[active_tab_idx].state.edit_state.blocking_write() = edit;
    }
}

/// Render edit header with key editing
/// Returns true if the key was modified
fn render_header(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    active_tab_idx: usize,
    original_key: &str,
    edit: &mut crate::core::EditState,
    current_lang: Language,
) -> bool {
    let mut key_changed = false;

    // Header row with key editing
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(tr(keys::KEY_LABEL, current_lang)).strong());
        let response = ui.add_enabled(
            !edit.saving,
            egui::TextEdit::singleline(&mut edit.edited_key).desired_width(300.0),
        );
        if response.changed() {
            key_changed = true;
        }

        // Save button - disabled when saving
        ui.add_enabled_ui(!edit.saving, |ui| {
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
            }
        });

        // Cancel button - disabled when saving
        ui.add_enabled_ui(!edit.saving, |ui| {
            if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                app.tabs[active_tab_idx]
                    .state
                    .edit_state
                    .blocking_write()
                    .cancel_edit();
            }
        });

        // Delete button - disabled when saving
        ui.add_enabled_ui(!edit.saving, |ui| {
            if ui
                .button(
                    egui::RichText::new(tr(keys::DELETE, current_lang)).color(egui::Color32::RED),
                )
                .clicked()
            {
                app.tabs[active_tab_idx]
                    .state
                    .spawn_delete_key(original_key.to_string());
            }
        });

        // Show saving indicator
        if edit.saving {
            ui.spinner();
            ui.label(egui::RichText::new("Saving...").color(egui::Color32::YELLOW));
        }
    });

    // Save message
    if !edit.save_message.is_empty() {
        ui.colored_label(egui::Color32::RED, &edit.save_message);
    }

    key_changed
}

/// Render value editing based on type
fn render_value_edit(
    ui: &mut egui::Ui,
    edited_value: &mut EditedValue,
    editable: bool,
    current_lang: Language,
) -> bool {
    let mut changed = false;

    match edited_value {
        EditedValue::String(s) => {
            changed |= render_string_edit(ui, s, editable, current_lang);
        }
        EditedValue::Hash(fields) => {
            changed |= render_hash_edit(ui, fields, editable, current_lang);
        }
        EditedValue::List(items) => {
            changed |= render_list_edit(ui, items, editable, current_lang);
        }
        EditedValue::Set(items) => {
            changed |= render_set_edit(ui, items, editable, current_lang);
        }
        EditedValue::ZSet(items) => {
            changed |= render_zset_edit(ui, items, editable, current_lang);
        }
        EditedValue::None => {
            ui.label(tr(keys::NO_VALUE_TO_EDIT, current_lang));
        }
    }

    changed
}

/// Render string value edit
fn render_string_edit(
    ui: &mut egui::Ui,
    s: &mut crate::core::JsonValue,
    editable: bool,
    current_lang: Language,
) -> bool {
    ui.label(tr(keys::TYPE_STRING, current_lang));
    let mut changed = false;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let available = ui.available_size();
            let desired_height = available
                .y
                .max(MIN_CENTRAL_PANEL_HEIGHT)
                .min(MAX_CENTRAL_PANEL_HEIGHT);
            let mut text_edit = egui::TextEdit::multiline(&mut s.value);
            if !editable {
                text_edit = text_edit.interactive(false);
            }
            if ui
                .add_sized([available.x, desired_height], text_edit)
                .changed()
            {
                changed = true;
            }
        });
    changed
}

/// Render hash value edit
fn render_hash_edit(
    ui: &mut egui::Ui,
    fields: &mut Vec<(String, crate::core::JsonValue)>,
    editable: bool,
    current_lang: Language,
) -> bool {
    ui.label(tr_fmt(
        keys::TYPE_HASH,
        current_lang,
        &[&fields.len().to_string()],
    ));

    let mut delete_idx: Option<usize> = None;
    let mut changed = false;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, (field, value)) in fields.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    // Add delete button with fixed width at the beginning
                    let delete_button_width = 60.0;
                    ui.add_enabled_ui(editable, |ui| {
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
                    });

                    ui.label(format!("{}.", idx));
                    let mut field_edit = egui::TextEdit::singleline(field)
                        .desired_width(150.0)
                        .hint_text("field");
                    if !editable {
                        field_edit = field_edit.interactive(false);
                    }
                    if ui.add(field_edit).changed() {
                        changed = true;
                    }
                    ui.label(tr(keys::COLON_SEPARATOR, current_lang));

                    let available_for_value =
                        ui.ctx().available_rect().width() - delete_button_width;

                    let mut value_edit = egui::TextEdit::singleline(&mut value.value)
                        .desired_width(available_for_value)
                        .hint_text(tr(keys::VALUE_PLACEHOLDER, current_lang));
                    if !editable {
                        value_edit = value_edit.interactive(false);
                    }
                    if ui.add(value_edit).changed() {
                        changed = true;
                    }
                });
            }

            ui.add_enabled_ui(editable, |ui| {
                if ui.button(tr(keys::ADD_FIELD, current_lang)).clicked() {
                    fields.push((String::new(), crate::core::JsonValue::new("")));
                    changed = true;
                }
            });
        });

    if editable {
        if let Some(idx) = delete_idx {
            fields.remove(idx);
            changed = true;
        }
    }

    changed
}

/// Render list value edit
fn render_list_edit(
    ui: &mut egui::Ui,
    items: &mut Vec<crate::core::JsonValue>,
    editable: bool,
    current_lang: Language,
) -> bool {
    ui.label(tr_fmt(
        keys::TYPE_LIST,
        current_lang,
        &[&items.len().to_string()],
    ));

    let mut delete_idx: Option<usize> = None;
    let mut changed = false;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, item) in items.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    // Add delete button with fixed width at the beginning
                    let delete_button_width = 60.0;
                    ui.add_enabled_ui(editable, |ui| {
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
                    });

                    ui.label(format!("[{}]", idx));

                    let available_for_value =
                        ui.ctx().available_rect().width() - delete_button_width;

                    let mut item_edit = egui::TextEdit::singleline(&mut item.value)
                        .desired_width(available_for_value);
                    if !editable {
                        item_edit = item_edit.interactive(false);
                    }
                    if ui.add(item_edit).changed() {
                        changed = true;
                    }
                });
            }

            ui.add_enabled_ui(editable, |ui| {
                if ui.button(tr(keys::ADD_ITEM, current_lang)).clicked() {
                    items.push(crate::core::JsonValue::new(""));
                    changed = true;
                }
            });
        });

    if editable {
        if let Some(idx) = delete_idx {
            items.remove(idx);
            changed = true;
        }
    }

    changed
}

/// Render set value edit
fn render_set_edit(
    ui: &mut egui::Ui,
    items: &mut Vec<crate::core::JsonValue>,
    editable: bool,
    current_lang: Language,
) -> bool {
    ui.label(tr_fmt(
        keys::TYPE_SET,
        current_lang,
        &[&items.len().to_string()],
    ));

    let mut delete_idx: Option<usize> = None;
    let mut changed = false;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, item) in items.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    // Add delete button with fixed width at the beginning
                    let delete_button_width = 60.0;
                    ui.add_enabled_ui(editable, |ui| {
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
                    });

                    ui.label(format!("{}.", idx));

                    let available_for_value =
                        ui.ctx().available_rect().width() - delete_button_width;

                    let mut item_edit = egui::TextEdit::singleline(&mut item.value)
                        .desired_width(available_for_value);
                    if !editable {
                        item_edit = item_edit.interactive(false);
                    }
                    if ui.add(item_edit).changed() {
                        changed = true;
                    }
                });
            }

            ui.add_enabled_ui(editable, |ui| {
                if ui.button(tr(keys::ADD_MEMBER, current_lang)).clicked() {
                    items.push(crate::core::JsonValue::new(""));
                    changed = true;
                }
            });
        });

    if editable {
        if let Some(idx) = delete_idx {
            items.remove(idx);
            changed = true;
        }
    }

    changed
}

/// Render sorted set value edit
fn render_zset_edit(
    ui: &mut egui::Ui,
    items: &mut Vec<(crate::core::JsonValue, String)>,
    editable: bool,
    current_lang: Language,
) -> bool {
    ui.label(tr_fmt(
        keys::TYPE_ZSET,
        current_lang,
        &[&items.len().to_string()],
    ));

    let mut delete_idx: Option<usize> = None;
    let mut changed = false;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, (member, score)) in items.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    // Add delete button with fixed width at the beginning
                    let delete_button_width = 60.0;
                    ui.add_enabled_ui(editable, |ui| {
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
                    });

                    ui.label(format!("{}.", idx));
                    ui.label(tr(keys::SCORE_LABEL, current_lang));
                    let mut score_edit = egui::TextEdit::singleline(score)
                        .desired_width(80.0)
                        .hint_text("0.0");
                    if !editable {
                        score_edit = score_edit.interactive(false);
                    }
                    if ui.add(score_edit).changed() {
                        changed = true;
                    }
                    ui.label(tr(keys::MEMBER_LABEL, current_lang));

                    let available_for_value =
                        ui.ctx().available_rect().width() - delete_button_width;

                    let mut member_edit = egui::TextEdit::singleline(&mut member.value)
                        .desired_width(available_for_value);
                    if !editable {
                        member_edit = member_edit.interactive(false);
                    }
                    if ui.add(member_edit).changed() {
                        changed = true;
                    }
                });
            }

            ui.add_enabled_ui(editable, |ui| {
                if ui.button(tr(keys::ADD_MEMBER, current_lang)).clicked() {
                    items.push((crate::core::JsonValue::new(""), "0".to_string()));
                    changed = true;
                }
            });
        });

    if editable {
        if let Some(idx) = delete_idx {
            items.remove(idx);
            changed = true;
        }
    }

    changed
}
