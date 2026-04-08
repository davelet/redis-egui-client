use crate::ui::window::RedisApp;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

/// Render element edit dialog
pub fn render_element_edit_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    if !app.element_edit_dialog.show {
        return;
    }

    let mut open = true;
    let key_type = app.element_edit_dialog.key_type.clone();

    // Calculate dialog position only when dialog just opened
    let dialog_size = egui::vec2(800.0, 600.0);
    let window_builder = if app.element_edit_dialog.just_opened {
        auto_format_json_value(app);

        app.element_edit_dialog.just_opened = false;

        // Center on screen
        let content_rect = ctx.content_rect();
        let dialog_pos = content_rect.center() - dialog_size * 0.5;

        egui::Window::new(tr(TranslationKey::EditElement, current_lang))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(dialog_size)
            .default_pos(dialog_pos)
    } else {
        egui::Window::new(tr(TranslationKey::EditElement, current_lang))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(dialog_size)
    };

    window_builder.show(ctx, |ui| {
        render_dialog_content(app, ui, ctx, current_lang, &key_type);
    });

    if !open {
        app.element_edit_dialog.show = false;
    }
}

/// Auto-format JSON value if valid
fn auto_format_json_value(app: &mut RedisApp) {
    if let Ok(json_value) =
        serde_json::from_str::<serde_json::Value>(&app.element_edit_dialog.value)
    {
        if let Ok(formatted) = serde_json::to_string_pretty(&json_value) {
            app.element_edit_dialog.value = formatted;
        }
    }
}

/// Render dialog content
fn render_dialog_content(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    current_lang: Language,
    key_type: &str,
) {
    render_context_info(app, ui, current_lang, key_type);
    ui.separator();
    render_value_editor(app, ui);
    ui.separator();
    render_action_buttons(app, ui, ctx, current_lang);
}

/// Render context information (key, field/index)
fn render_context_info(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    key_type: &str,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(tr(TranslationKey::KeyLabel, current_lang)).strong());
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
}

/// Render value editor
fn render_value_editor(app: &mut RedisApp, ui: &mut egui::Ui) {
    let available_height = ui.available_height() - 15.0; // Reserve space for buttons
    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), available_height],
                egui::TextEdit::multiline(&mut app.element_edit_dialog.value).code_editor(),
            );
        });
}

/// Render action buttons (save, copy, cancel)
fn render_action_buttons(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    current_lang: Language,
) {
    ui.horizontal(|ui| {
        // Save button
        if ui
            .button(
                egui::RichText::new(tr(TranslationKey::Save, current_lang))
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
                app.element_edit_dialog.original_value.clone(),
            );
            app.element_edit_dialog.show = false;
        }

        // Copy value button
        if ui
            .button(
                egui::RichText::new(app.copy_button_text(
                    egui::Id::new("copy_edit_dialog_value"),
                    tr(TranslationKey::CopyValue, current_lang),
                ))
                .color(app.copy_button_text_color(egui::Id::new("copy_edit_dialog_value"))),
            )
            .clicked()
        {
            ctx.copy_text(app.element_edit_dialog.value.clone());
            app.record_copy_success_with_id(egui::Id::new("copy_edit_dialog_value"));
        }

        // Cancel button
        if ui
            .button(tr(TranslationKey::Cancel, current_lang))
            .clicked()
        {
            app.element_edit_dialog.show = false;
        }
    });
}
