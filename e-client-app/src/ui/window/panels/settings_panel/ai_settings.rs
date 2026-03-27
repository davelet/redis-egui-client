//! AI settings UI components

use e_client_basics::constants::{
    AI_API_KEY_LIMIT, AI_MODEL_ID_LIMIT, AI_MODEL_NAME_LIMIT, AI_MODEL_URL_LIMIT,
};
use e_client_config::config::ai_config::{AiModel, AiProviderType, DEFAULT_SYSTEM_PROMPT};
use e_client_config::language::Language;
use e_client_config::translations::{emoji, keys, tr, tr_fmt};
use e_client_core::{detect_api_provider, AiClient};

use super::super::super::RedisApp;

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
                        tr(keys::AI_ENABLE, current_lang),
                    );
                });

                ui.add_space(8.0);

                // Active model selector and add button in one row
                ui.horizontal(|ui| {
                    ui.label(tr(keys::AI_ACTIVE_MODEL, current_lang));
                    ui.add_space(5.0);
                    let active_model_name = app
                        .config
                        .ai_config
                        .get_active_model()
                        .map(|m| m.name.as_str())
                        .unwrap_or(tr(keys::AI_SELECT_MODEL, current_lang));
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
                        .button(format!("+ {}", tr(keys::AI_ADD_MODEL, current_lang)))
                        .clicked()
                    {
                        app.ai_model_editor.open_for_new();
                    }
                });

                ui.add_space(8.0);

                // Confirm before execute checkbox
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut app.config.ai_config.confirm_before_execute,
                            tr(keys::AI_CONFIRM_BEFORE_EXECUTE, current_lang),
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
                            tr(keys::AI_SHOW_THINKING, current_lang),
                        )
                        .changed()
                    {
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }
                });

                ui.add_space(8.0);

                // System prompt label and hint
                ui.label(tr(keys::AI_SYSTEM_PROMPT, current_lang));
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(tr(keys::AI_SYSTEM_PROMPT_HINT, current_lang))
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );

                // System prompt text area
                let system_prompt = &mut app.config.ai_config.system_prompt;
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().code_bg_color)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(system_prompt)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace)
                                .hint_text(tr(keys::AI_SYSTEM_PROMPT_PLACEHOLDER, current_lang)),
                        );
                    });

                // Save and restore default buttons
                ui.horizontal(|ui| {
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::action::SAVE,
                            tr(keys::SAVE, current_lang)
                        ))
                        .clicked()
                    {
                        if let Err(e) = app.config.save_ai_config() {
                            eprintln!("Failed to save AI config: {}", e.to_message(current_lang));
                        }
                    }

                    // Show "Restore Default" button only when prompt differs from default
                    if app.config.ai_config.system_prompt != DEFAULT_SYSTEM_PROMPT {
                        if ui
                            .button(tr(keys::AI_RESTORE_DEFAULT_PROMPT, current_lang))
                            .clicked()
                        {
                            app.config.ai_config.system_prompt = DEFAULT_SYSTEM_PROMPT.to_string();
                            if let Err(e) = app.config.save_ai_config() {
                                eprintln!(
                                    "Failed to save AI config: {}",
                                    e.to_message(current_lang)
                                );
                            }
                        }
                    }
                });

                // Models list - collapsible with background
                if !app.config.ai_config.models.is_empty() {
                    ui.add_space(12.0);

                    egui::CollapsingHeader::new(tr(keys::AI_MODELS, current_lang))
                        .id_salt("ai_models_collapsible")
                        .default_open(!app.ai_model_editor.models_collapsed)
                        .show(ui, |ui| {
                            // Add background color using a frame
                            let bg_color = ui.visuals().code_bg_color;
                            egui::Frame::group(ui.style())
                                .fill(bg_color)
                                .show(ui, |ui| {
                                    egui::ScrollArea::horizontal().max_height(120.0).show(
                                        ui,
                                        |ui| {
                                            egui::Grid::new("ai_models_grid")
                                                .num_columns(5)
                                                .spacing([8.0, 4.0])
                                                .striped(true)
                                                .show(ui, |ui| {
                                                    ui.label(tr(keys::EDIT, current_lang));
                                                    ui.label(tr(keys::DELETE, current_lang));
                                                    ui.label(tr(keys::AI_MODEL_NAME, current_lang));
                                                    ui.label(tr(keys::AI_URL, current_lang));
                                                    ui.label(tr(keys::AI_MODEL_ID, current_lang));
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
                        });

                    // Update collapsed state
                    app.ai_model_editor.models_collapsed = false;
                }
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
        tr(keys::AI_EDIT_MODEL, current_lang)
    } else {
        tr(keys::AI_ADD_MODEL, current_lang)
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
            egui::Grid::new("ai_model_editor_grid")
                .num_columns(3)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label(tr(keys::AI_MODEL_NAME, current_lang));
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

                    ui.label(tr(keys::AI_PROVIDER, current_lang));
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
                            tr(keys::AI_PROVIDER_CUSTOM, current_lang).to_string()
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
                                            tr(keys::AI_PROVIDER_CUSTOM, current_lang),
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
                                .hint_text(tr(keys::AI_PROVIDER_SEARCH_PLACEHOLDER, current_lang)),
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
                            .on_hover_text(tr(keys::AI_PROVIDER_SEARCH_HINT, current_lang));
                        ui.label(tr_fmt(
                            keys::AI_PROVIDERS_COUNT,
                            current_lang,
                            &[&matching_count.to_string()],
                        ));
                    });

                    // Auto-fill or clear URL when provider changes (via ComboBox selection)
                    if old_provider != app.ai_model_editor.provider {
                        let provider_type = match app.ai_model_editor.provider.as_str() {
                            "OpenAi" => AiProviderType::OpenAi,
                            "Anthropic" => AiProviderType::Anthropic,
                            "Meta" => AiProviderType::Meta,
                            "Mistral" => AiProviderType::Mistral,
                            "Cohere" => AiProviderType::Cohere,
                            "Ollama" => AiProviderType::Ollama,
                            "LMStudio" => AiProviderType::LMStudio,
                            "LocalAI" => AiProviderType::LocalAI,
                            "Vllm" => AiProviderType::Vllm,
                            "OpenRouter" => AiProviderType::OpenRouter,
                            "Together" => AiProviderType::Together,
                            "Replicate" => AiProviderType::Replicate,
                            "Huggingface" => AiProviderType::Huggingface,
                            "Groq" => AiProviderType::Groq,
                            "Perplexity" => AiProviderType::Perplexity,
                            "Gemini" => AiProviderType::Gemini,
                            "Grok" => AiProviderType::Grok,
                            "Qwen" => AiProviderType::Qwen,
                            "Baichuan" => AiProviderType::Baichuan,
                            "Doubao" => AiProviderType::Doubao,
                            "Moonshot" => AiProviderType::Moonshot,
                            "Zhipu" => AiProviderType::Zhipu,
                            "Minimax" => AiProviderType::Minimax,
                            _ => AiProviderType::Custom,
                        };
                        if let Some(default_url) = provider_type.default_url() {
                            app.ai_model_editor.base_url = default_url;
                        } else {
                            // Custom provider - clear the URL
                            app.ai_model_editor.base_url.clear();
                        }
                        // Note: search box is NOT cleared automatically, user can manually clear it
                    }

                    ui.end_row();

                    ui.label(tr(keys::AI_URL, current_lang));
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

                    ui.label(tr(keys::AI_MODEL_ID, current_lang));
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

                    ui.label(tr(keys::AI_API_KEY, current_lang));
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

                    ui.label(tr(keys::AI_TEMPERATURE, current_lang));
                    ui.add(egui::Slider::new(
                        &mut app.ai_model_editor.temperature,
                        0.0..=1.0,
                    ));
                    ui.label("");
                    ui.end_row();
                });

            // Validation message
            ui.add_space(8.0);
            let is_valid = !app.ai_model_editor.name.is_empty()
                && !app.ai_model_editor.base_url.is_empty()
                && !app.ai_model_editor.model_id.is_empty();

            if !is_valid {
                let error_msg = tr(keys::AI_MODEL_REQUIRED_FIELDS, current_lang).to_string();
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
                        tr(keys::AI_TESTING_CONNECTION, current_lang)
                    } else {
                        tr(keys::AI_TEST_CONNECTION, current_lang)
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
                                egui::RichText::new(tr(keys::AI_TEST_SUCCESS, current_lang))
                                    .color(egui::Color32::from_rgb(50, 200, 50)),
                            );
                        }
                        Err(e) => {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{}: {}",
                                    tr(keys::AI_TEST_FAILED, current_lang),
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
                let save_btn =
                    ui.add_enabled(is_valid, egui::Button::new(tr(keys::SAVE, current_lang)));
                if save_btn.clicked() && is_valid {
                    let api_key = if app.ai_model_editor.api_key.is_empty() {
                        None
                    } else {
                        Some(app.ai_model_editor.api_key.clone())
                    };

                    let provider_type = match app.ai_model_editor.provider.as_str() {
                        "OpenAi" => AiProviderType::OpenAi,
                        "Anthropic" => AiProviderType::Anthropic,
                        "Meta" => AiProviderType::Meta,
                        "Mistral" => AiProviderType::Mistral,
                        "Cohere" => AiProviderType::Cohere,
                        "Ollama" => AiProviderType::Ollama,
                        "LMStudio" => AiProviderType::LMStudio,
                        "LocalAI" => AiProviderType::LocalAI,
                        "Vllm" => AiProviderType::Vllm,
                        "OpenRouter" => AiProviderType::OpenRouter,
                        "Together" => AiProviderType::Together,
                        "Replicate" => AiProviderType::Replicate,
                        "Huggingface" => AiProviderType::Huggingface,
                        "Groq" => AiProviderType::Groq,
                        "Perplexity" => AiProviderType::Perplexity,
                        "Gemini" => AiProviderType::Gemini,
                        "Grok" => AiProviderType::Grok,
                        "Qwen" => AiProviderType::Qwen,
                        "Baichuan" => AiProviderType::Baichuan,
                        "Doubao" => AiProviderType::Doubao,
                        "Moonshot" => AiProviderType::Moonshot,
                        "Zhipu" => AiProviderType::Zhipu,
                        "Minimax" => AiProviderType::Minimax,
                        _ => AiProviderType::Custom,
                    };

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

                if ui.button(tr(keys::CANCEL, current_lang)).clicked() {
                    app.ai_model_editor.close();
                }
            });
        });
}
