//! AI settings UI components

use e_client_basics::constants::{
    AI_API_KEY_LIMIT, AI_MODEL_ID_LIMIT, AI_MODEL_NAME_LIMIT, AI_MODEL_URL_LIMIT,
};
use e_client_basics::emoji;
use e_client_config::config::ai_config::{AiModel, AiProviderType};
use e_client_config::config::json_importer;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};
use e_client_core::{AiClient, CHAT_SYSTEM_PROMPT, SYSTEM_PROMPT, detect_api_provider};
use std::str::FromStr;

use super::super::super::RedisApp;
use super::super::super::JsonImportPreview;

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
                            "📥 {}",
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
                            "📤 {}",
                            tr(TranslationKey::AiExportJson, current_lang)
                        ))
                        .on_hover_text(tr(TranslationKey::AiExportJsonTooltip, current_lang))
                        .clicked()
                    {
                        if let Err(e) = export_ai_config_to_json(app) {
                            app.toasts.error(
                                "json_export".to_string(),
                                format!("Export failed: {}", e),
                            );
                        } else {
                            app.toasts.success(
                                "json_export".to_string(),
                                tr(TranslationKey::AiExportSuccess, current_lang).to_string(),
                            );
                        }
                    }

                    ui.add_space(5.0);

                    // JSON Import button
                    if ui
                        .button(format!(
                            "📂 {}",
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

/// Render AI model editor dialog
pub fn render_ai_model_editor(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    let window_title = if app.ai_model_editor.is_editing() {
        tr(TranslationKey::AiEditModel, current_lang)
    } else {
        tr(TranslationKey::AiAddModel, current_lang)
    };

    let screen_rect = ctx
        .input(|i| i.viewport().outer_rect)
        .unwrap_or(egui::Rect::ZERO);
    let top_right = egui::pos2(screen_rect.max.x - 120.0, screen_rect.min.y + 40.0);
    egui::Window::new(window_title)
        .id(egui::Id::new("ai_model_window"))
        .resizable(false)
        .collapsible(false)
        .default_pos(top_right)
        .movable(true)
        .show(ctx, |ui| {
            // Close on ESC key
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                app.ai_model_editor.close();
                return;
            }
            egui::Grid::new("ai_model_editor_grid")
                .num_columns(3)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label(tr(TranslationKey::AiModelName, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.ai_model_editor.name)
                            .char_limit(AI_MODEL_NAME_LIMIT),
                    );
                    ui.label(format!(
                        "{}/{}",
                        app.ai_model_editor.name.len(),
                        AI_MODEL_NAME_LIMIT
                    ));
                    ui.end_row();

                    ui.label(tr(TranslationKey::AiProvider, current_lang));
                    let old_provider = app.ai_model_editor.provider.clone();
                    let search_trimmed = app.ai_model_editor.provider_search.trim().to_lowercase();

                    ui.horizontal(|ui| {
                        // Count matching providers
                        let matching_count = if search_trimmed.is_empty() {
                            AiProviderType::all_providers().len()
                        } else {
                            AiProviderType::all_providers()
                                .iter()
                                .filter(|p| p.to_string().to_lowercase().contains(&search_trimmed))
                                .count()
                        };

                        // ComboBox with provider
                        let display_text = if app.ai_model_editor.provider == "Custom" {
                            tr(TranslationKey::AiProviderCustom, current_lang).to_string()
                        } else {
                            app.ai_model_editor.provider.clone()
                        };

                        egui::ComboBox::from_id_salt("ai_provider_selector")
                            .selected_text(&display_text)
                            .show_ui(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .max_height(200.0)
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        // Custom option first
                                        ui.selectable_value(
                                            &mut app.ai_model_editor.provider,
                                            "Custom".to_string(),
                                            tr(TranslationKey::AiProviderCustom, current_lang),
                                        );
                                        ui.separator();

                                        // Other providers
                                        for provider_type in AiProviderType::all_providers() {
                                            if matches!(provider_type, AiProviderType::Custom) {
                                                continue;
                                            }
                                            let provider_str = provider_type.to_string();
                                            if search_trimmed.is_empty()
                                                || provider_str
                                                    .to_lowercase()
                                                    .contains(&search_trimmed)
                                            {
                                                let provider_key = format!("{:?}", provider_type);
                                                ui.selectable_value(
                                                    &mut app.ai_model_editor.provider,
                                                    provider_key,
                                                    provider_str,
                                                );
                                            }
                                        }
                                    });
                            });

                        // Search box (second)
                        let search_response = ui.add(
                            egui::TextEdit::singleline(&mut app.ai_model_editor.provider_search)
                                .hint_text(tr(
                                    TranslationKey::AiProviderSearchPlaceholder,
                                    current_lang,
                                )),
                        );

                        // Auto-select first match when search filter changes
                        if search_response.changed() {
                            let current_search =
                                app.ai_model_editor.provider_search.trim().to_lowercase();
                            if !current_search.is_empty() {
                                // Find first matching provider
                                if let Some(first_match) =
                                    AiProviderType::all_providers().iter().find(|p| {
                                        p.to_string().to_lowercase().contains(&current_search)
                                    })
                                {
                                    app.ai_model_editor.provider = format!("{:?}", first_match);
                                    // Auto-fill URL
                                    if let Some(default_url) = first_match.default_url() {
                                        app.ai_model_editor.base_url = default_url;
                                    } else {
                                        app.ai_model_editor.base_url.clear();
                                    }
                                }
                            }
                        }

                        search_response
                            .on_hover_text(tr(TranslationKey::AiProviderSearchHint, current_lang));
                        ui.label(tr_fmt(
                            TranslationKey::AiProvidersCount,
                            current_lang,
                            &[&matching_count.to_string()],
                        ));
                    });

                    // Auto-fill or clear URL when provider changes (via ComboBox selection)
                    if old_provider != app.ai_model_editor.provider {
                        let provider_type = AiProviderType::from_str(&app.ai_model_editor.provider)
                            .unwrap_or(AiProviderType::Custom);
                        if let Some(default_url) = provider_type.default_url() {
                            app.ai_model_editor.base_url = default_url;
                        } else {
                            // Custom provider - clear the URL
                            app.ai_model_editor.base_url.clear();
                        }
                        // Note: search box is NOT cleared automatically, user can manually clear it
                    }

                    ui.end_row();

                    ui.label(tr(TranslationKey::AiUrl, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.ai_model_editor.base_url)
                            .char_limit(AI_MODEL_URL_LIMIT),
                    );
                    ui.label(format!(
                        "{}/{}",
                        app.ai_model_editor.base_url.len(),
                        AI_MODEL_URL_LIMIT
                    ));
                    ui.end_row();

                    // Show full path preview
                    if !app.ai_model_editor.base_url.is_empty() {
                        ui.label("");
                        let provider = detect_api_provider(&app.ai_model_editor.base_url);
                        let endpoint = provider.chat_endpoint();
                        let full_path = format!(
                            "{}{}",
                            app.ai_model_editor.base_url.trim_end_matches('/'),
                            endpoint
                        );
                        ui.label(
                            egui::RichText::new(full_path)
                                .size(11.0)
                                .color(egui::Color32::from_rgb(100, 150, 200)),
                        );
                        ui.label("");
                        ui.end_row();
                    }

                    ui.label(tr(TranslationKey::AiModelId, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.ai_model_editor.model_id)
                            .char_limit(AI_MODEL_ID_LIMIT),
                    );
                    ui.label(format!(
                        "{}/{}",
                        app.ai_model_editor.model_id.len(),
                        AI_MODEL_ID_LIMIT
                    ));
                    ui.end_row();

                    ui.label(tr(TranslationKey::AiApiKey, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.ai_model_editor.api_key)
                            .char_limit(AI_API_KEY_LIMIT)
                            .password(true),
                    );
                    ui.label(format!(
                        "{}/{}",
                        app.ai_model_editor.api_key.len(),
                        AI_API_KEY_LIMIT
                    ));
                    ui.end_row();

                    ui.label(tr(TranslationKey::AiTemperature, current_lang));
                    let mut temperature = app.ai_model_editor.temperature;
                    ui.add(egui::Slider::new(&mut temperature, 0.0..=1.0));
                    app.ai_model_editor.temperature = temperature.clamp(0.0, 1.0);
                    ui.label("");
                    ui.end_row();
                });

            // Validation message
            ui.add_space(8.0);
            let is_valid = !app.ai_model_editor.name.is_empty()
                && !app.ai_model_editor.base_url.is_empty()
                && !app.ai_model_editor.model_id.is_empty();

            if !is_valid {
                let error_msg = tr(TranslationKey::AiModelRequiredFields, current_lang).to_string();
                ui.colored_label(ui.visuals().error_fg_color, error_msg);
            }

            // Test connection button and result
            ui.horizontal(|ui| {
                // Check for async test result
                if let Some(receiver) = &app.ai_model_editor.test_result_receiver {
                    if let Ok(result) = receiver.try_recv() {
                        app.ai_model_editor.test_result = Some(result);
                        app.ai_model_editor.testing_connection = false;
                        app.ai_model_editor.test_result_receiver = None;
                    }
                }

                let test_btn = ui.add_enabled(
                    is_valid && !app.ai_model_editor.testing_connection,
                    egui::Button::new(if app.ai_model_editor.testing_connection {
                        tr(TranslationKey::AiTestingConnection, current_lang)
                    } else {
                        tr(TranslationKey::AiTestConnection, current_lang)
                    }),
                );

                if test_btn.clicked() && is_valid && !app.ai_model_editor.testing_connection {
                    let mut test_model = AiModel::default();
                    test_model.name = app.ai_model_editor.name.clone();
                    test_model.base_url = Some(app.ai_model_editor.base_url.clone());
                    test_model.model_id = app.ai_model_editor.model_id.clone();
                    test_model.api_key = if app.ai_model_editor.api_key.is_empty() {
                        None
                    } else {
                        Some(app.ai_model_editor.api_key.clone())
                    };
                    test_model.temperature = app.ai_model_editor.temperature;

                    // Test connection asynchronously
                    app.ai_model_editor.testing_connection = true;
                    app.ai_model_editor.test_result = None;
                    let (tx, rx) = std::sync::mpsc::channel();
                    app.ai_model_editor.test_result_receiver = Some(rx);

                    tokio::task::spawn_blocking(move || {
                        let result = AiClient::test_connection_sync(&test_model);
                        let _ = tx.send(result);
                    });
                }

                // Show test result
                if let Some(result) = &app.ai_model_editor.test_result {
                    match result {
                        Ok(()) => {
                            ui.label(
                                egui::RichText::new(tr(
                                    TranslationKey::AiTestSuccess,
                                    current_lang,
                                ))
                                .color(egui::Color32::from_rgb(50, 200, 50)),
                            );
                        }
                        Err(e) => {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{}: {}",
                                    tr(TranslationKey::AiTestFailed, current_lang),
                                    e
                                ))
                                .color(egui::Color32::from_rgb(255, 100, 100)),
                            );
                        }
                    }
                }
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                let save_btn = ui.add_enabled(
                    is_valid,
                    egui::Button::new(tr(TranslationKey::Save, current_lang)),
                );
                if save_btn.clicked() && is_valid {
                    let api_key = if app.ai_model_editor.api_key.is_empty() {
                        None
                    } else {
                        Some(app.ai_model_editor.api_key.clone())
                    };

                    let provider_type = AiProviderType::from_str(&app.ai_model_editor.provider)
                        .unwrap_or(AiProviderType::Custom);

                    let mut model = AiModel::new_model(
                        provider_type,
                        app.ai_model_editor.name.clone(),
                        Some(app.ai_model_editor.base_url.clone()),
                        app.ai_model_editor.model_id.clone(),
                    );
                    model.id = app
                        .ai_model_editor
                        .editing_model_id
                        .clone()
                        .unwrap_or_else(AiModel::generate_id);
                    model.api_key = api_key;
                    model.temperature = app.ai_model_editor.temperature;

                    if app.ai_model_editor.is_editing() {
                        app.config.ai_config.update_model(model.clone());
                    } else {
                        app.config.add_ai_model(model.clone());
                    }

                    // Set as active model
                    app.config.set_active_ai_model(&model.id);

                    // Save immediately
                    if let Err(e) = app.config.save_ai_config() {
                        eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                    }

                    app.ai_model_editor.close();
                }

                if ui
                    .button(tr(TranslationKey::Cancel, current_lang))
                    .clicked()
                {
                    app.ai_model_editor.close();
                }
            });
        });
}

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
        .default_pos(top_right)
        .movable(true)
        .show(ctx, |ui| {
            // Close on ESC key
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                app.gim_import_dialog.close();
                return;
            }

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

// =============================================================================
// JSON Import/Export Functions
// =============================================================================

use e_client_config::config::ai_config::AiConfig;
use std::path::Path;

/// Export AI config to JSON file (API keys are excluded)
fn export_ai_config_to_json(app: &mut RedisApp) -> Result<(), String> {
    let path = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_file_name("ai_config.json")
        .save_file()
        .ok_or("No file selected")?;

    json_importer::export_to_json(&app.config.ai_config, &path)
        .map_err(|e| format!("{:?}", e))
}

/// Import AI config from JSON file
fn import_ai_config_from_json(path: &Path) -> Result<AiConfig, String> {
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
                egui::RichText::new(tr_fmt(TranslationKey::AiImportJsonWarning, current_lang, &[emoji::action::WARNING]))
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
                if ui.button(tr(TranslationKey::AiImportSelectAll, current_lang)).clicked() {
                    for s in &mut preview.selected { *s = true; }
                }
                if ui.button(tr(TranslationKey::AiImportDeselectAll, current_lang)).clicked() {
                    for s in &mut preview.selected { *s = false; }
                }
                if ui.button(tr(TranslationKey::AiImportInvertSelection, current_lang)).clicked() {
                    for s in &mut preview.selected { *s = !*s; }
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
                                    let luminance = (bg.r() as u16 + bg.g() as u16 + bg.b() as u16) / 3;
                                    let is_light_bg = luminance > 128;

                                    let (border_color, check_color, text_color, box_fill, box_fill_hover, box_fill_active) = if is_light_bg {
                                        // Light background: use dark colors
                                        (
                                            egui::Color32::from_gray(80),   // border
                                            egui::Color32::from_gray(30),   // checkmark
                                            egui::Color32::from_gray(20),   // label text
                                            egui::Color32::from_gray(230),  // box fill
                                            egui::Color32::from_gray(210),  // box fill hover
                                            egui::Color32::from_gray(190),  // box fill active
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
                                    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.5, check_color);
                                    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border_color);
                                    v.widgets.inactive.bg_fill = box_fill;
                                    // hovered
                                    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, check_color);
                                    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, border_color);
                                    v.widgets.hovered.bg_fill = box_fill_hover;
                                    // active (pressed)
                                    v.widgets.active.fg_stroke = egui::Stroke::new(2.0, check_color);
                                    v.widgets.active.bg_stroke = egui::Stroke::new(1.5, border_color);
                                    v.widgets.active.bg_fill = box_fill_active;
                                    // noninteractive
                                    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, border_color);
                                    v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_color);
                                    v.widgets.noninteractive.bg_fill = box_fill;

                                    for (i, model) in preview.config.models.iter().enumerate() {
                                        let is_duplicate = app.config.ai_config.models.iter().any(|m| m.name == model.name);
                                        ui.horizontal(|ui| {
                                            ui.checkbox(&mut preview.selected[i],
                                                egui::RichText::new(format!("• {}", model.name)).color(text_color));
                                            ui.label(
                                                egui::RichText::new(format!("({})", model.provider))
                                                    .color(text_color.linear_multiply(0.6)),
                                            );
                                            if is_duplicate {
                                                ui.add_space(5.0);
                                                ui.label(
                                                    egui::RichText::new(tr_fmt(TranslationKey::AiImportAlreadyExists, current_lang, &[emoji::action::WARNING]))
                                                        .color(ui.visuals().warn_fg_color)
                                                );
                                                ui.label(
                                                    egui::RichText::new(format!("({})", tr(TranslationKey::AiImportOverride, current_lang)))
                                                        .color(ui.visuals().warn_fg_color)
                                                );
                                            }
                                        });
                                        ui.label(
                                            egui::RichText::new(format!("  Model: {}", model.model_id)).weak(),
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
                cancel_clicked = ui.button(tr(TranslationKey::Cancel, current_lang)).clicked();
                ui.add_space(10.0);
                
                let any_selected = preview.selected.iter().any(|&s| s);
                confirm_clicked = ui.add_enabled(
                    any_selected,
                    egui::Button::new(
                        egui::RichText::new(tr(TranslationKey::AiImportJsonConfirm, current_lang))
                            .strong()
                    )
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
                        if let Some(id) = app.config.ai_config.models.iter().find(|m| m.name == model.name).map(|m| m.id.clone()) {
                            app.config.remove_ai_model(&id);
                        }
                        app.config.add_ai_model(model);
                    }
                }

                // Save config
                if let Err(e) = app.config.save_ai_config() {
                    app.toasts.error(
                        "json_import".to_string(),
                        format!("Save failed: {:?}", e),
                    );
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
