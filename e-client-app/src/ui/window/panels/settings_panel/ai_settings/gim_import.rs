//! GIM configuration import dialog

use e_client_basics::emoji;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};

use crate::ui::window::RedisApp;

/// Render GIM configuration import dialog
pub fn render_gim_import_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    if !app.gim_import_dialog.show {
        return;
    }

    let screen_rect = ctx.input(|i| i.viewport().outer_rect).unwrap_or(egui::Rect::ZERO);
    let top_right = egui::pos2(screen_rect.max.x - 150.0, screen_rect.min.y + 40.0);

    egui::Window::new(tr(TranslationKey::AiImportingTitle, current_lang))
        .id(egui::Id::new("gim_import_window"))
        .resizable(false)
        .collapsible(false)
        .interactable(true)
        .default_pos(top_right)
        .movable(true)
        .show(ctx, |ui| {

            // Check if config was not found
            if let Some(ref error) = app.gim_import_dialog.error_message {
                if error == "NOT_FOUND" {
                    // Show error message for not found
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        tr(TranslationKey::AiImportNotFound, current_lang),
                    );
                    ui.add_space(10.0);
                    if ui.button(tr(TranslationKey::Cancel, current_lang)).clicked() {
                        app.gim_import_dialog.close();
                    }
                    return;
                } else {
                    // Show generic error
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        format!("{}: {}", tr(TranslationKey::AiImportFailed, current_lang), error),
                    );
                    ui.add_space(10.0);
                    if ui.button(tr(TranslationKey::Cancel, current_lang)).clicked() {
                        app.gim_import_dialog.close();
                    }
                    return;
                }
            }

            // Show config preview - use nested if-let to avoid borrow conflicts
            if let Some(gim_config) = &app.gim_import_dialog.gim_config {
                if let Some(converted_model) = &app.gim_import_dialog.converted_model {
                    // Model info section with subtle background
                    ui.vertical(|ui| {
                        ui.set_width(280.0);

                        // Model name (prominent)
                        ui.label(
                            egui::RichText::new(&converted_model.name)
                                .strong()
                                .size(16.0)
                        );
                        ui.add_space(8.0);

                        // Provider
                        ui.horizontal(|ui| {
                            ui.label(tr(TranslationKey::AiImportProvider, current_lang));
                            ui.label(egui::RichText::new(converted_model.provider.to_string())
                                .color(ui.visuals().hyperlink_color));
                        });

                        // URL
                        ui.horizontal(|ui| {
                            ui.label(tr(TranslationKey::AiImportUrl, current_lang));
                            ui.label(egui::RichText::new(converted_model.get_base_url())
                                .small()
                                .weak());
                        });

                        // API Key indicator (if present)
                        if gim_config.api_key.is_some() {
                            ui.horizontal(|ui| {
                                ui.label(tr(TranslationKey::AiImportApiKey, current_lang));
                                ui.label(egui::RichText::new("••••••••")
                                    .weak());
                                ui.label(egui::RichText::new(tr(TranslationKey::AiImportKeychainNote, current_lang))
                                    .small()
                                    .weak()
                                    .italics());
                            });
                        }
                    }).response.on_hover_text("GIM model configuration");

                    ui.add_space(10.0);
                }

                // Check if model already exists - clone needed values first
                let model_name = gim_config.model.clone();
                let model_exists = app
                    .config
                    .ai_config
                    .models
                    .iter()
                    .any(|m| m.name == model_name);

                if model_exists {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(tr_fmt(TranslationKey::AiImportAlreadyExists, current_lang, &[emoji::action::WARNING]))
                            .color(ui.visuals().warn_fg_color));
                        ui.label(egui::RichText::new(format!("({})", tr(TranslationKey::AiImportOverride, current_lang)))
                            .color(ui.visuals().warn_fg_color));
                    });
                    ui.add_space(5.0);
                }

                // Buttons
                let mut import_clicked = false;
                let mut cancel_clicked = false;
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        cancel_clicked = ui.button(tr(TranslationKey::Cancel, current_lang)).clicked();
                        ui.add_space(5.0);
                        import_clicked = ui.button(
                            egui::RichText::new(tr(TranslationKey::AiImportConfirm, current_lang))
                                .strong()
                        ).clicked();
                    });
                });

                // Handle button clicks outside the closure to avoid borrow conflicts
                if cancel_clicked {
                    app.gim_import_dialog.close();
                } else if import_clicked {
                    // Convert model and import
                    let model = gim_config.to_ai_model();

                    // Remove existing model with same name if exists
                    if model_exists {
                        if let Some(id) = app.config.ai_config.models.iter()
                            .find(|m| m.name == model_name)
                            .map(|m| m.id.clone())
                        {
                            app.config.remove_ai_model(&id);
                        }
                    }

                    // Add the new model
                    app.config.add_ai_model(model.clone());
                    app.config.set_active_ai_model(&model.id);

                    // Save immediately
                    if let Err(e) = app.config.save_ai_config() {
                        eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                    }

                    // Show success toast
                    app.toasts.success(
                        tr(TranslationKey::AiImportingTitle, current_lang).to_string(),
                        tr(TranslationKey::AiImportSuccess, current_lang).to_string(),
                    );

                    app.gim_import_dialog.close();
                }
            }
        });
}
