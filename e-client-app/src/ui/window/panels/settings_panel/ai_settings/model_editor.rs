//! AI model editor dialog

use e_client_basics::constants::{
    AI_API_KEY_LIMIT, AI_MODEL_ID_LIMIT, AI_MODEL_NAME_LIMIT, AI_MODEL_URL_LIMIT,
};
use e_client_config::config::ai_config::{AiModel, AiProviderType};
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};
use e_client_core::{AiClient, detect_api_provider};
use std::str::FromStr;

use crate::ui::window::RedisApp;

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
                            // Show detailed error via toast, only once
                            app.toasts.error(
                                "ai_model_test_failed".to_string(),
                                format!("{}: {}", tr(TranslationKey::AiTestFailed, current_lang), e),
                            );
                            // Show simple fail message in dialog
                            ui.label(
                                egui::RichText::new(tr(TranslationKey::AiTestFailed, current_lang))
                                    .color(egui::Color32::from_rgb(255, 100, 100)),
                            );
                            // Clear result to avoid showing toast repeatedly
                            app.ai_model_editor.test_result = None;
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
