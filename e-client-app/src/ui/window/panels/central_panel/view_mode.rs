use crate::core::ValueData;
use crate::ui::window::RedisApp;
use e_client_basics::constants::{MAX_CENTRAL_PANEL_HEIGHT, MIN_CENTRAL_PANEL_HEIGHT};
use e_client_config::language::Language;
use e_client_config::translations::emoji;
use e_client_config::translations::{keys, tr, tr_fmt};

use super::utils::{format_ttl, truncate_key, value_to_copy_text};

/// Render key in view mode
pub fn render_view_mode(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    value: &Option<ValueData>,
    ttl: i64,
    current_lang: Language,
) {
    render_header(app, ui, ctx, active_tab_idx, key, value, ttl, current_lang);

    // Value display
    if let Some(val) = value {
        super::value_renderer::render_value_view(
            app,
            ui,
            ctx,
            active_tab_idx,
            key,
            val,
            current_lang,
        );
    }
}

/// Render the header row with key actions
fn render_header(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    value: &Option<ValueData>,
    ttl: i64,
    current_lang: Language,
) {
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
        render_ttl_controls(app, ui, ctx, active_tab_idx, key, ttl, current_lang);
    });
}

/// Render TTL display and edit controls
fn render_ttl_controls(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    active_tab_idx: usize,
    key: &str,
    ttl: i64,
    current_lang: Language,
) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if ui.button(emoji::action::REFRESH).clicked() {
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
        let mut ttl_edit_key = app.tabs[active_tab_idx].state.ttl_edit_key.blocking_write();

        // Check if key changed while in edit mode - if so, update the value
        if *ttl_edit_mode {
            let key_changed = ttl_edit_key.as_ref().map_or(true, |k| k != key);
            if key_changed {
                *ttl_edit_value = ttl.to_string();
                *ttl_edit_key = Some(key.to_string());
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
                *ttl_edit_key = None;
            }
            if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                *ttl_edit_value = String::new();
                *ttl_edit_mode = false;
                *ttl_edit_key = None;
            }
        } else {
            if ui.button(tr(keys::EDIT_TTL, current_lang)).clicked() {
                *ttl_edit_value = ttl.to_string();
                *ttl_edit_mode = true;
                *ttl_edit_key = Some(key.to_string());
            }
        }
        drop(ttl_edit_key);
        drop(ttl_edit_value);
        drop(ttl_edit_mode);
    });
}
