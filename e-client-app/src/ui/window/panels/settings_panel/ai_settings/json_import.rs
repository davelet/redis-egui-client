//! JSON import/export for AI configuration

use e_client_basics::emoji;
use e_client_config::config::ai_config::AiConfig;
use e_client_config::config::json_importer;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};
use std::path::Path;

use crate::ui::window::RedisApp;

/// Export AI config to JSON file (API keys are excluded)
/// Returns `Some(Ok(()))` on success, `Some(Err(_))` on failure, `None` if user cancelled.
pub(super) fn export_ai_config_to_json(app: &mut RedisApp) -> Option<Result<(), String>> {
    let path = match rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_file_name("ai_config.json")
        .save_file()
    {
        Some(p) => p,
        None => return None, // User cancelled, no notification
    };

    Some(
        json_importer::export_to_json(&app.config.ai_config, &path).map_err(|e| format!("{:?}", e)),
    )
}

/// Import AI config from JSON file
pub(super) fn import_ai_config_from_json(path: &Path) -> Result<AiConfig, String> {
    json_importer::import_from_json(path).map_err(|e| format!("{:?}", e))
}

/// Render JSON import preview dialog
pub fn render_json_import_preview(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    let mut preview = match app.json_import_preview.take() {
        Some(p) => p,
        None => return,
    };

    egui::Window::new(tr(TranslationKey::AiImportJsonTitle, current_lang))
        .id(egui::Id::new("json_import_preview_window"))
        .anchor(egui::Align2::CENTER_TOP, [0.0, 50.0])
        .resizable(false)
        .collapsible(false)
        .fixed_size([450.0, 250.0])
        .show(ctx, |ui| {
            ui.set_width(450.0);

            // Warning message
            ui.add(egui::Label::new(
                egui::RichText::new(tr_fmt(
                    TranslationKey::AiImportJsonWarning,
                    current_lang,
                    &[emoji::action::WARNING],
                ))
                .color(ui.visuals().warn_fg_color)
                .small(),
            ));
            ui.add_space(5.0);

            // Model count
            ui.label(format!(
                "{}: {}",
                tr(TranslationKey::AiImportJsonModelCount, current_lang),
                preview.config.models.len()
            ));
            ui.add_space(10.0);

            // Bulk selection actions
            ui.horizontal(|ui| {
                if ui
                    .button(tr(TranslationKey::AiImportSelectAll, current_lang))
                    .clicked()
                {
                    for s in &mut preview.selected {
                        *s = true;
                    }
                }
                if ui
                    .button(tr(TranslationKey::AiImportDeselectAll, current_lang))
                    .clicked()
                {
                    for s in &mut preview.selected {
                        *s = false;
                    }
                }
                if ui
                    .button(tr(TranslationKey::AiImportInvertSelection, current_lang))
                    .clicked()
                {
                    for s in &mut preview.selected {
                        *s = !*s;
                    }
                }
            });
            ui.add_space(5.0);

            // Model list
            let bg_color = ui.visuals().code_bg_color;
            egui::Frame::group(ui.style())
                .fill(bg_color)
                .show(ui, |ui| {
                    let bg_color = ui.visuals().code_bg_color;
                    egui::Frame::group(ui.style())
                        .fill(bg_color)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("json_import_preview_scroll")
                                .max_height(120.0)
                                .auto_shrink([false; 2])
                                .show(ui, |ui| {
                                    // Adapt checkbox colors based on background luminance
                                    let bg = ui.visuals().code_bg_color;
                                    let luminance =
                                        (bg.r() as u16 + bg.g() as u16 + bg.b() as u16) / 3;
                                    let is_light_bg = luminance > 128;

                                    let (
                                        border_color,
                                        check_color,
                                        text_color,
                                        box_fill,
                                        box_fill_hover,
                                        box_fill_active,
                                    ) = if is_light_bg {
                                        // Light background: use dark colors
                                        (
                                            egui::Color32::from_gray(80),  // border
                                            egui::Color32::from_gray(30),  // checkmark
                                            egui::Color32::from_gray(20),  // label text
                                            egui::Color32::from_gray(230), // box fill
                                            egui::Color32::from_gray(210), // box fill hover
                                            egui::Color32::from_gray(190), // box fill active
                                        )
                                    } else {
                                        // Dark background: use light colors
                                        (
                                            egui::Color32::from_gray(160),
                                            egui::Color32::from_gray(230),
                                            egui::Color32::from_gray(220),
                                            egui::Color32::from_gray(60),
                                            egui::Color32::from_gray(80),
                                            egui::Color32::from_gray(100),
                                        )
                                    };

                                    let v = ui.visuals_mut();
                                    // inactive (default)
                                    v.widgets.inactive.fg_stroke =
                                        egui::Stroke::new(1.5, check_color);
                                    v.widgets.inactive.bg_stroke =
                                        egui::Stroke::new(1.0, border_color);
                                    v.widgets.inactive.bg_fill = box_fill;
                                    // hovered
                                    v.widgets.hovered.fg_stroke =
                                        egui::Stroke::new(1.5, check_color);
                                    v.widgets.hovered.bg_stroke =
                                        egui::Stroke::new(1.5, border_color);
                                    v.widgets.hovered.bg_fill = box_fill_hover;
                                    // active (pressed)
                                    v.widgets.active.fg_stroke =
                                        egui::Stroke::new(2.0, check_color);
                                    v.widgets.active.bg_stroke =
                                        egui::Stroke::new(1.5, border_color);
                                    v.widgets.active.bg_fill = box_fill_active;
                                    // noninteractive
                                    v.widgets.noninteractive.fg_stroke =
                                        egui::Stroke::new(1.0, border_color);
                                    v.widgets.noninteractive.bg_stroke =
                                        egui::Stroke::new(1.0, border_color);
                                    v.widgets.noninteractive.bg_fill = box_fill;

                                    for (i, model) in preview.config.models.iter().enumerate() {
                                        let is_duplicate = app
                                            .config
                                            .ai_config
                                            .models
                                            .iter()
                                            .any(|m| m.name == model.name);
                                        ui.horizontal(|ui| {
                                            ui.checkbox(
                                                &mut preview.selected[i],
                                                egui::RichText::new(format!("• {}", model.name))
                                                    .color(text_color),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "({})",
                                                    model.provider
                                                ))
                                                .color(text_color.linear_multiply(0.6)),
                                            );
                                            if is_duplicate {
                                                ui.add_space(5.0);
                                                ui.label(
                                                    egui::RichText::new(tr_fmt(
                                                        TranslationKey::AiImportAlreadyExists,
                                                        current_lang,
                                                        &[emoji::action::WARNING],
                                                    ))
                                                    .color(ui.visuals().warn_fg_color),
                                                );
                                                ui.label(
                                                    egui::RichText::new(format!(
                                                        "({})",
                                                        tr(
                                                            TranslationKey::AiImportOverride,
                                                            current_lang
                                                        )
                                                    ))
                                                    .color(ui.visuals().warn_fg_color),
                                                );
                                            }
                                        });
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "  Model: {}",
                                                model.model_id
                                            ))
                                            .weak(),
                                        );
                                        ui.add_space(4.0);
                                    }
                                });
                        });
                });

            ui.add_space(15.0);

            // Buttons
            let mut confirm_clicked = false;
            let mut cancel_clicked = false;
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                cancel_clicked = ui
                    .button(tr(TranslationKey::Cancel, current_lang))
                    .clicked();
                ui.add_space(10.0);

                let any_selected = preview.selected.iter().any(|&s| s);
                confirm_clicked = ui
                    .add_enabled(
                        any_selected,
                        egui::Button::new(
                            egui::RichText::new(tr(
                                TranslationKey::AiImportJsonConfirm,
                                current_lang,
                            ))
                            .strong(),
                        ),
                    )
                    .clicked();
            });

            // Handle button clicks outside closure
            if cancel_clicked {
                // Do nothing, just close
            } else if confirm_clicked {
                // Merge models: add new ones, overwrite duplicates by name
                let selected_flags = preview.selected;
                for (i, model) in preview.config.models.into_iter().enumerate() {
                    if selected_flags[i] {
                        if let Some(id) = app
                            .config
                            .ai_config
                            .models
                            .iter()
                            .find(|m| m.name == model.name)
                            .map(|m| m.id.clone())
                        {
                            app.config.remove_ai_model(&id);
                        }
                        app.config.add_ai_model(model);
                    }
                }

                // Save config
                if let Err(e) = app.config.save_ai_config() {
                    app.toasts
                        .error("json_import".to_string(), format!("Save failed: {:?}", e));
                } else {
                    app.toasts.success(
                        "json_import".to_string(),
                        tr(TranslationKey::AiImportJsonSuccess, current_lang).to_string(),
                    );
                }
            } else {
                app.json_import_preview = Some(preview);
            }
        });
}
