//! AI settings UI components

use e_client_basics::constants::{
    AI_API_KEY_LIMIT, AI_MODEL_ID_LIMIT, AI_MODEL_NAME_LIMIT, AI_MODEL_URL_LIMIT,
};
use e_client_config::config::ai_config::AiModel;
use e_client_config::language::Language;
use e_client_config::translations::{emoji, keys, tr};

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
                                                        ui.label(&model.url);
                                                        ui.label(&model.model_id);
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

                    ui.label(tr(keys::AI_URL, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.ai_model_editor.url)
                            .char_limit(AI_MODEL_URL_LIMIT),
                    );
                    ui.label(format!(
                        "{}/{}",
                        app.ai_model_editor.url.len(),
                        AI_MODEL_URL_LIMIT
                    ));
                    ui.end_row();

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
                            .char_limit(AI_API_KEY_LIMIT),
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
                && !app.ai_model_editor.url.is_empty()
                && !app.ai_model_editor.model_id.is_empty();

            if !is_valid {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    tr(keys::AI_MODEL_REQUIRED_FIELDS, current_lang),
                );
            }

            ui.horizontal(|ui| {
                let save_btn =
                    ui.add_enabled(is_valid, egui::Button::new(tr(keys::SAVE, current_lang)));
                if save_btn.clicked() && is_valid {
                    let api_key = if app.ai_model_editor.api_key.is_empty() {
                        None
                    } else {
                        Some(app.ai_model_editor.api_key.clone())
                    };

                    let model = AiModel {
                        id: app
                            .ai_model_editor
                            .editing_model_id
                            .clone()
                            .unwrap_or_else(AiModel::generate_id),
                        name: app.ai_model_editor.name.clone(),
                        url: app.ai_model_editor.url.clone(),
                        model_id: app.ai_model_editor.model_id.clone(),
                        api_key,
                        temperature: app.ai_model_editor.temperature,
                    };

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
