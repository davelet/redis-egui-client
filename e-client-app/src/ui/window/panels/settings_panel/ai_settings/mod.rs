//! AI settings UI components

mod gim_import;
mod json_import;
mod model_editor;

pub use gim_import::render_gim_import_dialog;
pub use json_import::render_json_import_preview;

use e_client_basics::emoji;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};
use e_client_core::{CHAT_SYSTEM_PROMPT, SYSTEM_PROMPT};

use crate::ui::window::JsonImportPreview;
use crate::ui::window::RedisApp;

use json_import::{export_ai_config_to_json, import_ai_config_from_json};
use model_editor::render_ai_model_editor;

/// Render AI settings section
pub fn render_ai_settings_section(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    current_lang: Language,
) {
    egui::ScrollArea::vertical()
        .id_salt("ai_settings_scroll")
        .max_height(300.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                // AI enabled checkbox
                ui.horizontal(|ui| {
                    ui.checkbox(
                        &mut app.config.ai_config.enabled,
                        tr(TranslationKey::AiEnable, current_lang),
                    );
                });

                ui.add_space(8.0);

                // Active model selector and add button in one row
                ui.horizontal(|ui| {
                    ui.label(tr(TranslationKey::AiActiveModel, current_lang));
                    ui.add_space(5.0);
                    let active_model_name = app
                        .config
                        .ai_config
                        .get_active_model()
                        .map(|m| m.name.as_str())
                        .unwrap_or(tr(TranslationKey::AiSelectModel, current_lang));
                    egui::ComboBox::from_id_salt("ai_active_model")
                        .selected_text(active_model_name)
                        .show_ui(ui, |ui| {
                            let mut new_active_id: Option<String> = None;
                            for model in &app.config.ai_config.models {
                                let is_selected = app.config.ai_config.active_model_id.as_deref()
                                    == Some(&model.id);
                                ui.selectable_label(is_selected, &model.name)
                                    .clicked()
                                    .then(|| {
                                        new_active_id = Some(model.id.clone());
                                    });
                            }
                            if let Some(id) = new_active_id {
                                app.config.set_active_ai_model(&id);
                                if let Err(e) = app.config.save_ai_config() {
                                    eprintln!(
                                        "Failed to save AI config: {}",
                                        e.to_message(current_lang)
                                    );
                                }
                            }
                        });

                    ui.add_space(10.0);

                    if ui
                        .button(format!(
                            "+ {}",
                            tr(TranslationKey::AiAddModel, current_lang)
                        ))
                        .clicked()
                    {
                        app.ai_model_editor.open_for_new();
                    }
                });

                ui.add_space(8.0);

                // GIM Import + JSON Import/Export in one row
                ui.horizontal(|ui| {
                    // GIM Import button
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::action::IMPORT_FOLDER,
                            tr(TranslationKey::AiImportFromGim, current_lang)
                        ))
                        .on_hover_text(tr(TranslationKey::AiImportFromGimTooltip, current_lang))
                        .clicked()
                    {
                        app.gim_import_dialog.open();
                    }

                    ui.add_space(10.0);

                    // JSON Export button
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::action::EXPORT,
                            tr(TranslationKey::AiExportJson, current_lang)
                        ))
                        .on_hover_text(tr(TranslationKey::AiExportJsonTooltip, current_lang))
                        .clicked()
                    {
                        match export_ai_config_to_json(app) {
                            Some(Ok(())) => {
                                app.toasts.success(
                                    "json_export".to_string(),
                                    tr(TranslationKey::AiExportSuccess, current_lang).to_string(),
                                );
                            }
                            Some(Err(e)) => {
                                app.toasts.error(
                                    "json_export".to_string(),
                                    format!("Export failed: {}", e),
                                );
                            }
                            None => {} // User cancelled, silent
                        }
                    }

                    ui.add_space(5.0);

                    // JSON Import button
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::action::IMPORT,
                            tr(TranslationKey::AiImportJson, current_lang)
                        ))
                        .on_hover_text(tr(TranslationKey::AiImportJsonTooltip, current_lang))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .pick_file()
                        {
                            match import_ai_config_from_json(&path) {
                                Ok(config) => {
                                    let mut selected = vec![false; config.models.len()];
                                    let mut all_conflict = true;
                                    for (i, model) in config.models.iter().enumerate() {
                                        let is_duplicate = app.config.ai_config.models.iter().any(|m| m.name == model.name);
                                        if !is_duplicate {
                                            selected[i] = true;
                                            all_conflict = false;
                                        }
                                    }
                                    if all_conflict {
                                        for s in &mut selected { *s = true; }
                                    }

                                    // Show preview and confirm dialog
                                    app.json_import_preview = Some(JsonImportPreview {
                                        path,
                                        config,
                                        selected,
                                    });
                                }
                                Err(e) => {
                                    app.toasts.error(
                                        "json_import".to_string(),
                                        format!("Import failed: {}", e),
                                    );
                                }
                            }
                        }
                    }
                });

                ui.add_space(8.0);

                // Models list - collapsible with background
                if !app.config.ai_config.models.is_empty() {
                    ui.add_space(4.0);

                    // Label instead of CollapsingHeader
                    ui.label(egui::RichText::new(tr(TranslationKey::AiModels, current_lang)).strong());
                    ui.add_space(4.0);

                    // Add background color using a frame
                    let bg_color = ui.visuals().code_bg_color;
                    egui::Frame::group(ui.style())
                        .fill(bg_color)
                        .show(ui, |ui| {
                            // max_height ~85.0 shows about 2.5 items vertically
                            egui::ScrollArea::both().max_height(85.0).show(
                                ui,
                                |ui| {

                                    egui::Grid::new("ai_models_grid")
                                        .num_columns(5)
                                        .spacing([8.0, 4.0])
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label(tr(
                                                TranslationKey::Edit,
                                                current_lang,
                                            ));
                                            ui.label(tr(
                                                TranslationKey::Delete,
                                                current_lang,
                                            ));
                                            ui.label(tr(
                                                TranslationKey::AiModelName,
                                                current_lang,
                                            ));
                                            ui.label(tr(
                                                TranslationKey::AiUrl,
                                                current_lang,
                                            ));
                                            ui.label(tr(
                                                TranslationKey::AiModelId,
                                                current_lang,
                                            ));
                                            ui.end_row();

                                            let mut model_ids_to_delete: Vec<String> =
                                                Vec::new();
                                            for model in &app.config.ai_config.models {
                                                let model_id = model.id.clone();
                                                if ui.button(emoji::action::EDIT).clicked()
                                                {
                                                    app.ai_model_editor
                                                        .open_for_edit(model);
                                                }
                                                if ui
                                                    .button(emoji::action::DELETE)
                                                    .clicked()
                                                {
                                                    model_ids_to_delete.push(model_id);
                                                }
                                                ui.label(&model.name);
                                                ui.label(&model.get_base_url());
                                                ui.label(&model.get_model_id());
                                                ui.end_row();
                                            }
                                            for id in model_ids_to_delete.iter() {
                                                app.config.remove_ai_model(id);
                                            }
                                            if !model_ids_to_delete.is_empty() {
                                                if let Err(e) = app.config.save_ai_config()
                                                {
                                                    eprintln!(
                                                        "Failed to save AI config: {}",
                                                        e.to_message(current_lang)
                                                    );
                                                }
                                            }
                                        });
                                },
                            );
                        });
                }

                ui.add_space(8.0);

                // Confirm before execute checkbox
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut app.config.ai_config.confirm_before_execute,
                            tr(TranslationKey::AiConfirmBeforeExecute, current_lang),
                        )
                        .changed()
                    {
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }
                });

                // Show AI thinking checkbox
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut app.config.ai_config.show_ai_thinking,
                            tr(TranslationKey::AiShowThinking, current_lang),
                        )
                        .changed()
                    {
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }
                });

                // Render markdown in CLI checkbox
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut app.config.ai_config.render_markdown,
                            tr(TranslationKey::AiRenderMarkdown, current_lang),
                        )
                        .changed()
                    {
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }
                });

                // Max tool-call turns slider
                ui.horizontal(|ui| {
                    ui.label(tr(TranslationKey::AiMaxTurns, current_lang));
                    let mut max_turns = app.config.ai_config.max_turns;
                    ui.add(
                        egui::Slider::new(&mut max_turns, 1..=20)
                            .suffix(tr(TranslationKey::AiMaxTurnsUnit, current_lang)),
                    );
                    if max_turns != app.config.ai_config.max_turns {
                        app.config.ai_config.max_turns = max_turns;
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }
                });

                ui.add_space(8.0);

                // System prompt: Agent (left) and Chat (right) side by side
                ui.horizontal_wrapped(|ui| {
                    // --- Agent mode prompt (left) ---
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() / 2.0 - 4.0);
                        ui.horizontal(|ui| {
                            ui.label(tr(TranslationKey::AiSystemPrompt, current_lang));
                            let copy_prompt_id = egui::Id::new("copy_system_prompt");
                            let copy_prompt_color = app.copy_button_text_color(copy_prompt_id);
                            let copy_prompt_text = app.copy_button_text(
                                copy_prompt_id,
                                tr(TranslationKey::AiCopyPrompt, current_lang),
                            );
                            if ui
                                .button(
                                    egui::RichText::new(copy_prompt_text).color(copy_prompt_color),
                                )
                                .clicked()
                            {
                                ui.ctx().copy_text(SYSTEM_PROMPT.to_string());
                                app.record_copy_success_with_id(copy_prompt_id);
                            }
                        });
                        egui::Frame::group(ui.style())
                            .fill(ui.visuals().code_bg_color)
                            .show(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .id_salt("agent_prompt_scroll")
                                    .max_height(200.0)
                                    .show(ui, |ui| {
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(SYSTEM_PROMPT).monospace(),
                                            )
                                            .wrap(),
                                        );
                                    });
                            });
                    });

                    ui.add_space(8.0);

                    // --- Chat mode prompt (right) ---
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() / 2.0 - 4.0);
                        ui.horizontal(|ui| {
                            ui.label(tr(TranslationKey::AiChatSystemPrompt, current_lang));
                            let copy_chat_id = egui::Id::new("copy_chat_prompt");
                            let copy_chat_color = app.copy_button_text_color(copy_chat_id);
                            let copy_chat_text = app.copy_button_text(
                                copy_chat_id,
                                tr(TranslationKey::AiCopyChatPrompt, current_lang),
                            );
                            if ui
                                .button(egui::RichText::new(copy_chat_text).color(copy_chat_color))
                                .clicked()
                            {
                                ui.ctx().copy_text(CHAT_SYSTEM_PROMPT.to_string());
                                app.record_copy_success_with_id(copy_chat_id);
                            }
                        });
                        egui::Frame::group(ui.style())
                            .fill(ui.visuals().code_bg_color)
                            .show(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .id_salt("chat_prompt_scroll")
                                    .max_height(200.0)
                                    .show(ui, |ui| {
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(CHAT_SYSTEM_PROMPT).monospace(),
                                            )
                                            .wrap(),
                                        );
                                    });
                            });
                    });
                });
            });
        });

    // Render AI model editor window
    if app.ai_model_editor.show {
        render_ai_model_editor(app, ctx, current_lang);
    }
}
