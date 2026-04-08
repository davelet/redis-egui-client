use crate::core::ValueData;
use crate::ui::window::RedisApp;
use e_client_basics::emoji;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};

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
    // Auto-refresh TTL every second (only when TTL > 0 and auto_refresh is enabled)
    if ttl > 0 && app.config.settings.auto_refresh_ttl {
        let should_refresh = {
            let last_refresh = app.tabs[active_tab_idx]
                .state
                .ttl_last_refresh
                .blocking_read();
            match *last_refresh {
                None => true,
                Some(instant) => instant.elapsed().as_secs() >= 1,
            }
        };

        if should_refresh {
            app.tabs[active_tab_idx]
                .state
                .spawn_refresh_ttl_only(key.to_string());
        }
        // Request repaint every second to keep TTL updated
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }

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
        ui.heading(tr_fmt(
            TranslationKey::KeyHeading,
            current_lang,
            &[&display_key],
        ));

        let copy_key_id = egui::Id::new("copy_key").with(key);
        let copy_key_color = app.copy_button_text_color(copy_key_id);
        let copy_key_text =
            app.copy_button_text(copy_key_id, tr(TranslationKey::CopyKey, current_lang));

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
                let copy_value_text = app
                    .copy_button_text(copy_value_id, tr(TranslationKey::CopyValue, current_lang));
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
        let loading_for_edit = *app.tabs[active_tab_idx]
            .state
            .loading_fields_for_edit
            .blocking_read();

        // Check if hash has unloaded fields
        let has_unloaded_fields = if let Some(ValueData::Hash {
            fields,
            loaded_values,
            ..
        }) = value
        {
            fields.iter().any(|f| !loaded_values.contains_key(f))
        } else {
            false
        };

        let edit_button_text = if loading_for_edit {
            "Loading..."
        } else {
            tr(TranslationKey::Edit, current_lang)
        };

        let edit_button_enabled = !loading_for_edit;

        if ui
            .add_enabled(edit_button_enabled, egui::Button::new(edit_button_text))
            .clicked()
        {
            if has_unloaded_fields && !loading_for_edit {
                // Load all field values first, then auto-enter edit mode when done
                *app.tabs[active_tab_idx]
                    .state
                    .pending_edit_after_load
                    .blocking_write() = true;
                *app.tabs[active_tab_idx]
                    .state
                    .pending_edit_ttl
                    .blocking_write() = ttl;
                app.tabs[active_tab_idx]
                    .state
                    .spawn_load_all_hash_field_values(key.to_string());
            } else {
                // All fields loaded, enter edit mode
                app.tabs[active_tab_idx]
                    .state
                    .edit_state
                    .blocking_write()
                    .enter_edit(key, ttl, value);
            }
        }

        // Delete button (always visible)
        if ui
            .button(
                egui::RichText::new(tr(TranslationKey::Delete, current_lang))
                    .color(egui::Color32::RED),
            )
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
            app.tabs[active_tab_idx].state.spawn_load_value(
                key.to_string(),
                e_client_core::app_state::operations::keys::HashLoadMode::ReloadFields,
            );
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

            if ui.button(tr(TranslationKey::Save, current_lang)).clicked() {
                if let Ok(new_ttl) = ttl_edit_value.parse::<i64>() {
                    app.tabs[active_tab_idx]
                        .state
                        .spawn_update_ttl(key.to_string(), new_ttl);
                }
                *ttl_edit_value = String::new();
                *ttl_edit_mode = false;
                *ttl_edit_key = None;
            }
            if ui
                .button(tr(TranslationKey::Cancel, current_lang))
                .clicked()
            {
                *ttl_edit_value = String::new();
                *ttl_edit_mode = false;
                *ttl_edit_key = None;
            }
        } else {
            if ui
                .button(tr(TranslationKey::EditTtl, current_lang))
                .clicked()
            {
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
