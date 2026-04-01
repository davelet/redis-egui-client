use crate::ui::window::RedisApp;
use crate::ui::window::components::markdown::render_markdown;
use crate::ui::window::types::HistoryEntry;
use e_client_basics::constants::REDIS_COMMANDS;
use e_client_config::config::ai_config::AiMode;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};
use e_client_core::{AiChatResult, AiResponseError, OpenAiRigAgent};

/// Helper to clear rig_agent when mode/model changes
fn clear_agent(app: &mut RedisApp, tab_idx: usize) {
    let rig_agent = app.tabs[tab_idx].command_line_panel.rig_agent.clone();
    tokio::task::spawn(async move {
        let mut agent = rig_agent.lock().await;
        *agent = None;
    });
}

/// Check if the input is a Redis command
fn is_redis_command(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Get the first word (command)
    let first_word = trimmed
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_uppercase();

    REDIS_COMMANDS.contains(&first_word.as_str())
}

pub fn render_command_line_panel(app: &mut RedisApp, ctx: &egui::Context) {
    let active_tab_idx = app.active_tab;

    // Early return if panel is hidden for current tab
    if !app.tabs[active_tab_idx].command_line_panel.show {
        return;
    }

    // Process any pending Redis command results
    process_redis_command_results(app, active_tab_idx);

    // Process any pending AI chat results
    process_ai_chat_results(app, active_tab_idx);

    let current_lang = app.poll_language(app.tabs[active_tab_idx].state.language.clone());

    // Handle pending AI command confirmation dialog
    if let Some((ref user_input, ref redis_cmd)) = app.tabs[active_tab_idx]
        .command_line_panel
        .pending_ai_command
        .clone()
    {
        render_ai_confirm_dialog(
            app,
            active_tab_idx,
            ctx,
            current_lang,
            user_input,
            redis_cmd,
        );
    }

    egui::TopBottomPanel::bottom("command_line_panel")
        .exact_height(ctx.content_rect().height() / 2.0)
        .show(ctx, |ui| {
            // Mode toggle and model selector row
            ui.horizontal(|ui| {
                // Mode toggle
                ui.label(
                    egui::RichText::new(tr(TranslationKey::ChatMode, current_lang))
                        .small()
                        .color(egui::Color32::GRAY),
                );
                let current_mode = app.tabs[active_tab_idx].command_line_panel.current_mode;
                if ui
                    .selectable_label(current_mode == AiMode::Chat, "Chat")
                    .on_hover_text("Stateless - translates natural language to Redis commands (no context, no tools)")
                    .clicked()
                    && current_mode != AiMode::Chat
                {
                    app.tabs[active_tab_idx].command_line_panel.current_mode = AiMode::Chat;
                    clear_agent(app, active_tab_idx);
                }
                if ui
                    .selectable_label(current_mode == AiMode::Agent, "Agent")
                    .on_hover_text("Stateful - has access to Redis tools for direct operations (requires tool-calling capable models)")
                    .clicked()
                    && current_mode != AiMode::Agent
                {
                    app.tabs[active_tab_idx].command_line_panel.current_mode = AiMode::Agent;
                    clear_agent(app, active_tab_idx);
                }

                ui.separator();

                // Model selector (per-tab, not persisted)
                ui.label(
                    egui::RichText::new(tr(TranslationKey::AiSelectModel, current_lang))
                        .small()
                        .color(egui::Color32::GRAY),
                );
                let current_model_id = app.tabs[active_tab_idx].command_line_panel.current_model_id.clone();
                let active_model_name = current_model_id
                    .as_ref()
                    .and_then(|id| app.config.ai_config.models.iter().find(|m| &m.id == id))
                    .map(|m| m.name.as_str())
                    .or_else(|| app.config.ai_config.get_active_model().map(|m| m.name.as_str()))
                    .unwrap_or("--");
                egui::ComboBox::from_id_salt("cli_ai_model_selector")
                    .selected_text(active_model_name)
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        let mut new_model_id: Option<String> = None;
                        for model in &app.config.ai_config.models {
                            let is_selected = current_model_id.as_deref() == Some(&model.id)
                                || (current_model_id.is_none()
                                    && app.config.ai_config.active_model_id.as_deref() == Some(&model.id));
                            ui.selectable_label(is_selected, &model.name)
                                .clicked()
                                .then(|| {
                                    new_model_id = Some(model.id.clone());
                                });
                        }
                        if let Some(id) = new_model_id {
                            app.tabs[active_tab_idx].command_line_panel.current_model_id = Some(id);
                            clear_agent(app, active_tab_idx);
                        }
                    });

                // Turn counter (Agent mode only, right-aligned)
                let current_mode = app.tabs[active_tab_idx].command_line_panel.current_mode;
                if current_mode == AiMode::Agent {
                    let rig_agent = app.tabs[active_tab_idx].command_line_panel.rig_agent.clone();
                    if let Ok(agent_guard) = rig_agent.try_lock() {
                        if let Some(ref agent) = *agent_guard {
                            let turns = agent.turn_count();
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} turns", turns))
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                            });
                        }
                    }
                }
            });

            ui.separator();

            // Command history output area
            let available_height = ui.available_height() - 40.0; // Reserve space for input
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(available_height)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for entry in &app.tabs[active_tab_idx].command_line_panel.history {
                            let cmd = &entry.command;
                            let result = &entry.result;
                            // Command line
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("> ")
                                        .color(egui::Color32::from_rgb(100, 200, 100))
                                        .monospace(),
                                );
                                ui.label(
                                    egui::RichText::new(cmd)
                                        .color(egui::Color32::BLUE)
                                        .monospace(),
                                );
                            });
                            // Result line
                            ui.vertical(|ui| {
                                ui.add_space(2.0);
                                let render_as_md = app.config.ai_config.render_markdown;
                                if render_as_md {
                                    render_markdown(result, ui);
                                } else {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.add_space(16.0);
                                        ui.label(
                                            egui::RichText::new(result)
                                                .color(if result.starts_with("ERR:") || result.starts_with("Error:") {
                                                    egui::Color32::from_rgb(255, 100, 100)
                                                } else if result == "Thinking..." {
                                                    egui::Color32::from_rgb(150, 150, 150)
                                                } else {
                                                    egui::Color32::BLACK
                                                })
                                                .monospace(),
                                        ).on_hover_text(result.clone());
                                    });
                                }
                            });
                            ui.add_space(4.0);
                        }
                    });

                    // Auto-scroll to bottom when new content is added
                    if app.tabs[active_tab_idx].command_line_panel.scroll_to_bottom {
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                        app.tabs[active_tab_idx].command_line_panel.scroll_to_bottom = false;
                    }
                });

            ui.separator();

            // Command input area
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("> ")
                        .color(egui::Color32::from_rgb(100, 200, 100))
                        .monospace()
                        .size(14.0),
                );

                let response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(
                        &mut app.tabs[active_tab_idx].command_line_panel.input,
                    )
                    .font(egui::TextStyle::Monospace)
                    .hint_text(tr(TranslationKey::CommandLineHint, current_lang))
                    .id(egui::Id::new("command_line_input")),
                );

                // Set focus to input when panel is shown
                if app.tabs[active_tab_idx].command_line_panel.scroll_to_bottom {
                    response.request_focus();
                }

                // Handle Up/Down keys for history
                if response.has_focus() {
                    let mut history_changed = false;
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        let panel = &mut app.tabs[active_tab_idx].command_line_panel;
                        if !panel.history.is_empty() {
                            if panel.history_index.is_none() {
                                panel.saved_input = panel.input.clone();
                                panel.history_index = Some(panel.history.len() - 1);
                                history_changed = true;
                            } else if let Some(idx) = panel.history_index {
                                if idx > 0 {
                                    panel.history_index = Some(idx - 1);
                                    history_changed = true;
                                }
                            }
                            if history_changed {
                                if let Some(idx) = panel.history_index {
                                    panel.input = panel.history[idx].command.clone();
                                }
                            }
                        }
                    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        let panel = &mut app.tabs[active_tab_idx].command_line_panel;
                        if let Some(idx) = panel.history_index {
                            if idx + 1 < panel.history.len() {
                                panel.history_index = Some(idx + 1);
                                panel.input = panel.history[idx + 1].command.clone();
                                history_changed = true;
                            } else {
                                panel.history_index = None;
                                panel.input = panel.saved_input.clone();
                                history_changed = true;
                            }
                        }
                    }

                    if history_changed {
                        // Move cursor to end
                        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                            let ccursor = egui::text::CCursor::new(
                                app.tabs[active_tab_idx]
                                    .command_line_panel
                                    .input
                                    .chars()
                                    .count(),
                            );
                            state.cursor.set_char_range(Some(
                                egui::text_selection::CCursorRange::one(ccursor),
                            ));
                            state.store(ui.ctx(), response.id);
                        }
                    }
                }

                // Handle Enter key to execute command
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let command = app.tabs[active_tab_idx]
                        .command_line_panel
                        .input
                        .trim()
                        .to_string();
                    if !command.is_empty() {
                        execute_command(app, active_tab_idx, command);
                        app.tabs[active_tab_idx].command_line_panel.input.clear();
                        app.tabs[active_tab_idx].command_line_panel.scroll_to_bottom = true;
                        app.tabs[active_tab_idx].command_line_panel.history_index = None;
                        app.tabs[active_tab_idx]
                            .command_line_panel
                            .saved_input
                            .clear();
                    }
                    response.request_focus();
                }

                // Handle Esc key to close panel (but not when AI confirm dialog is open)
                if !app.show_settings
                    && app.tabs[active_tab_idx]
                        .command_line_panel
                        .pending_ai_command
                        .is_none()
                    && ui.input(|i| i.key_pressed(egui::Key::Escape))
                {
                    app.tabs[active_tab_idx].command_line_panel.show = false;
                }
            });
        });
}

/// Render AI command confirmation dialog
fn render_ai_confirm_dialog(
    app: &mut RedisApp,
    tab_idx: usize,
    ctx: &egui::Context,
    current_lang: Language,
    user_input: &str,
    redis_cmd: &str,
) {
    // Read config outside the closure to avoid borrow conflict
    let confirm_before_execute = app.config.ai_config.confirm_before_execute;

    let screen = ctx.viewport_rect();
    let dialog_width = 520.0_f32.min(screen.width() - 40.0);

    let mut dialog_action: Option<DialogAction> = None;

    // Handle keyboard shortcuts before rendering (consumes the events)
    // Right arrow to execute, Left arrow to cancel
    ctx.input_mut(|i| {
        if i.key_pressed(egui::Key::ArrowRight) {
            dialog_action = Some(DialogAction::Execute);
            i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight);
        } else if i.key_pressed(egui::Key::ArrowLeft) {
            dialog_action = Some(DialogAction::Cancel);
            i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft);
        }
    });

    // If keyboard action was taken, handle it directly without showing the dialog
    if dialog_action.is_some() {
        match dialog_action {
            Some(DialogAction::Execute) => {
                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
                execute_redis_command(app, tab_idx, redis_cmd.to_string());
            }
            Some(DialogAction::Cancel) => {
                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
            }
            None => {}
        }
        return;
    }

    egui::Window::new(tr(TranslationKey::AiConfirmDialogTitle, current_lang))
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .default_width(dialog_width)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            egui::Frame::group(ui.style())
                .fill(ui.visuals().panel_fill)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Title
                        ui.label(
                            egui::RichText::new(tr(
                                TranslationKey::AiConfirmDialogTitle,
                                current_lang,
                            ))
                            .size(16.0)
                            .strong(),
                        );

                        ui.add_space(4.0);

                        // User's question
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Q: ")
                                    .color(egui::Color32::BLUE)
                                    .monospace(),
                            );
                            ui.label(egui::RichText::new(user_input).monospace());
                        });

                        // Suggested command
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("→ ")
                                    .color(egui::Color32::GREEN)
                                    .monospace(),
                            );
                            ui.label(
                                egui::RichText::new(redis_cmd)
                                    .color(egui::Color32::BLUE)
                                    .monospace(),
                            );
                        });

                        ui.add_space(8.0);

                        // Buttons
                        ui.horizontal(|ui| {
                            let execute_btn = ui.button(
                                egui::RichText::new(tr(TranslationKey::AiExecute, current_lang))
                                    .color(ui.visuals().text_color()),
                            );

                            if execute_btn.clicked() {
                                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
                                execute_redis_command(app, tab_idx, redis_cmd.to_string());
                            }

                            execute_btn.on_hover_text("Execute the Redis command");

                            ui.add_space(8.0);

                            let cancel_btn = ui.button(tr(TranslationKey::Cancel, current_lang));
                            if cancel_btn.clicked() {
                                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
                            }

                            // Show skip info if not configured to confirm
                            if !confirm_before_execute {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new("(Auto-execute enabled)")
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                            }
                        });
                    });
                });
        });
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DialogAction {
    Execute,
    Cancel,
}

fn execute_command(app: &mut RedisApp, tab_idx: usize, input: String) {
    let trimmed_input = input.trim();

    // Check if input is a Redis command
    if is_redis_command(trimmed_input) {
        // Execute as Redis command directly
        execute_redis_command(app, tab_idx, input);
    } else {
        // Send to AI for inference
        execute_ai_command(app, tab_idx, trimmed_input.to_string());
    }
}

/// Execute a Redis command directly
fn execute_redis_command(app: &mut RedisApp, tab_idx: usize, command: String) {
    if app.tabs[tab_idx]
        .command_line_panel
        .redis_command_pending
        .is_some()
    {
        return;
    }

    let client = app.tabs[tab_idx].state.redis_client.clone();
    let (tx, rx) = std::sync::mpsc::channel();

    app.tabs[tab_idx]
        .command_line_panel
        .history
        .push(HistoryEntry::plain(command.clone(), "Executing..."));
    app.tabs[tab_idx].command_line_panel.redis_command_pending = Some(rx);

    tokio::task::spawn(async move {
        let result = client.execute_command(&command).await;
        let out = match result {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };
        let _ = tx.send(out);
    });
}

/// Process pending Redis command results
pub fn process_redis_command_results(app: &mut RedisApp, tab_idx: usize) {
    let pending = match app.tabs[tab_idx]
        .command_line_panel
        .redis_command_pending
        .take()
    {
        Some(p) => p,
        None => return,
    };

    match pending.try_recv() {
        Ok(result) => {
            let last_idx = app.tabs[tab_idx].command_line_panel.history.len() - 1;
            let final_output = match result {
                Ok(output) => output,
                Err(e) => format!("ERR: {}", e),
            };
            app.tabs[tab_idx].command_line_panel.history[last_idx].result = final_output;
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            app.tabs[tab_idx].command_line_panel.redis_command_pending = Some(pending);
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            let last_idx = app.tabs[tab_idx].command_line_panel.history.len() - 1;
            app.tabs[tab_idx].command_line_panel.history[last_idx].result =
                "ERR: Command execution failed - channel disconnected".to_string();
        }
    }
}

fn execute_ai_command(app: &mut RedisApp, tab_idx: usize, trimmed_input: String) {
    // Check if there's already a pending AI chat
    if app.tabs[tab_idx]
        .command_line_panel
        .ai_chat_pending
        .is_some()
    {
        return;
    }

    // Check if AI is enabled and has an active model
    let ai_config = &app.config.ai_config.clone();
    let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());

    if !ai_config.enabled {
        let error_msg = format!("ERR: {}", tr(TranslationKey::AiDisabled, current_lang));
        app.tabs[tab_idx]
            .command_line_panel
            .history
            .push(HistoryEntry::plain(trimmed_input, error_msg));
        return;
    }

    let active_model = ai_config.get_active_model();
    if active_model.is_none() {
        let error_msg = format!(
            "ERR: {}. {}",
            tr(TranslationKey::AiNotConfigured, current_lang),
            tr(TranslationKey::AiPleaseConfigure, current_lang)
        );
        app.tabs[tab_idx]
            .command_line_panel
            .history
            .push(HistoryEntry::plain(trimmed_input, error_msg));
        return;
    }

    // Show thinking indicator only if configured
    if ai_config.show_ai_thinking {
        app.tabs[tab_idx]
            .command_line_panel
            .history
            .push(HistoryEntry::plain(trimmed_input.clone(), "Thinking..."));
    }

    // Get the history index of the thinking message (if shown)
    let thinking_idx = if ai_config.show_ai_thinking {
        Some(app.tabs[tab_idx].command_line_panel.history.len() - 1)
    } else {
        None
    };

    let user_input = trimmed_input.clone();
    let confirm_before_execute = ai_config.confirm_before_execute;
    let current_mode = app.tabs[tab_idx].command_line_panel.current_mode;

    // Create channel for async result
    let (tx, rx) = std::sync::mpsc::channel();

    // Store pending state
    app.tabs[tab_idx].command_line_panel.ai_chat_pending =
        Some(crate::ui::window::types::AiChatPending {
            thinking_idx,
            user_input: trimmed_input,
            context: None,
            confirm_before_execute,
            receiver: rx,
            use_rig_agent: true,
        });

    // Get references needed for async task
    let redis_client = std::sync::Arc::new(app.tabs[tab_idx].state.redis_client.clone());
    let ai_config_clone = ai_config.clone();
    let rig_agent_arc = app.tabs[tab_idx].command_line_panel.rig_agent.clone();
    let mode_for_task = current_mode;
    let per_tab_model_id = app.tabs[tab_idx].command_line_panel.current_model_id.clone();

    // Execute AI chat with rig agent asynchronously
    tokio::task::spawn(async move {
        // Try to get or create the rig agent
        let mut agent_opt = rig_agent_arc.lock().await;

        // If agent doesn't exist, try to create it first
        if agent_opt.is_none() {
            // Determine which model to use: per-tab > global active
            let model_id = per_tab_model_id
                .or_else(|| ai_config_clone.active_model_id.clone());

            let mut model_with_key = match model_id {
                Some(id) => ai_config_clone
                    .models
                    .iter()
                    .find(|m| m.id == id)
                    .cloned()
                    .unwrap_or_default(),
                None => Default::default(),
            };

            if let Err(e) = model_with_key.load_api_key() {
                let _ = tx.send(Err(AiResponseError::Other(format!(
                    "Failed to load API key from keyring: {}",
                    e
                ))));
                return;
            }

            // Build a config with the selected model's API key
            let mut config_with_key = ai_config_clone.clone();
            config_with_key.active_model_id = Some(model_with_key.id.clone());
            if let Some(m) = config_with_key.models.iter_mut().find(|m| m.id == model_with_key.id) {
                m.api_key = model_with_key.api_key.clone();
            } else {
                config_with_key.models.push(model_with_key.clone());
            }

            match OpenAiRigAgent::new(&config_with_key, redis_client.clone(), mode_for_task).await {
                Ok(new_agent) => {
                    *agent_opt = Some(new_agent);
                }
                Err(e) => {
                    // Agent creation failed, send error immediately
                    let creation_error = AiResponseError::Other(e);
                    let _ = tx.send(Err(creation_error));
                    return;
                }
            }
        }

        // Now we have an agent (or had one), chat with it
        let response = if let Some(ref mut agent) = *agent_opt {
            agent.chat(&user_input).await
        } else {
            // This shouldn't happen, but handle it gracefully
            let _ = tx.send(Err(AiResponseError::NoModelConfigured));
            return;
        };

        let chat_response = match response {
            Ok(AiChatResult::Text(text)) => Ok(text),
            Ok(AiChatResult::ToolCall { name, result }) => {
                Ok(format!("[Tool '{}' executed]\n{}", name, result))
            }
            Ok(AiChatResult::Error(e)) => Err(e),
            Err(e) => Err(e),
        };

        // For Chat mode, clear the agent after each request to enforce statelessness
        if mode_for_task == AiMode::Chat {
            *agent_opt = None;
        }

        let _ = tx.send(chat_response);
    });
}

/// Process pending AI chat results (call this each frame)
pub fn process_ai_chat_results(app: &mut RedisApp, tab_idx: usize) {
    let pending = match app.tabs[tab_idx].command_line_panel.ai_chat_pending.take() {
        Some(p) => p,
        None => return,
    };

    // Check if result is ready
    let result = match pending.receiver.try_recv() {
        Ok(r) => r,
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            // Not ready yet, put it back
            app.tabs[tab_idx].command_line_panel.ai_chat_pending = Some(pending);
            return;
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            // Channel disconnected, show error
            if let Some(idx) = pending.thinking_idx {
                app.tabs[tab_idx].command_line_panel.history[idx] = HistoryEntry::plain(
                    pending.user_input,
                    "ERR: AI request failed - channel disconnected",
                );
            }
            return;
        }
    };

    match result {
        Ok(response) => {
            let trimmed_response = response.trim();
            if is_redis_command(trimmed_response) {
                if pending.confirm_before_execute {
                    // Store pending command and show confirmation dialog
                    app.tabs[tab_idx].command_line_panel.pending_ai_command =
                        Some((pending.user_input, trimmed_response.to_string()));

                    // Remove the "Thinking..." message since we'll show the dialog
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history.remove(idx);
                    }
                } else {
                    // Execute without confirmation
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history[idx] =
                            HistoryEntry::plain(pending.user_input.clone(), "Executing...");
                    }

                    execute_redis_command(app, tab_idx, trimmed_response.to_string());

                    // Remove the "Executing..." message
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history.remove(idx);
                    }
                }
            } else {
                // Non-Redis command response — mark as markdown (AI response)
                if let Some(idx) = pending.thinking_idx {
                    app.tabs[tab_idx].command_line_panel.history[idx] =
                        HistoryEntry::markdown(pending.user_input, response);
                } else {
                    app.tabs[tab_idx]
                        .command_line_panel
                        .history
                        .push(HistoryEntry::markdown(pending.user_input, response));
                }
            }
        }
        Err(e) => {
            // Get current language for i18n
            let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());

            // Format error message using i18n
            let error_msg = if let Some(detail) = e.get_detail() {
                format!(
                    "ERR: {}",
                    tr_fmt(e.get_i18n_key(), current_lang, &[&detail])
                )
            } else {
                format!("ERR: {}", tr(e.get_i18n_key(), current_lang))
            };

            // Update the thinking message with the error
            if let Some(idx) = pending.thinking_idx {
                app.tabs[tab_idx].command_line_panel.history[idx] =
                    HistoryEntry::plain(pending.user_input, error_msg);
            } else {
                app.tabs[tab_idx]
                    .command_line_panel
                    .history
                    .push(HistoryEntry::plain(pending.user_input, error_msg));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== is_redis_command Tests ====================

    #[test]
    fn test_is_redis_command_valid_commands() {
        // Common Redis commands
        let commands = vec![
            "GET mykey",
            "SET mykey value",
            "DEL mykey",
            "EXISTS mykey",
            "KEYS *",
            "INFO",
            "PING",
            "DBSIZE",
            "FLUSHDB",
            "FLUSHALL",
            "HGET myhash field",
            "HSET myhash field value",
            "LPUSH mylist value",
            "RPOP mylist",
            "SADD myset member",
            "ZADD myzset 1.0 member",
        ];

        for cmd in commands {
            assert!(
                is_redis_command(cmd),
                "Should recognize '{}' as Redis command",
                cmd
            );
        }
    }

    #[test]
    fn test_is_redis_command_case_insensitive() {
        // Test case insensitivity
        assert!(is_redis_command("get mykey"));
        assert!(is_redis_command("Get mykey"));
        assert!(is_redis_command("GET mykey"));
        assert!(is_redis_command("GeT mykey"));
    }

    #[test]
    fn test_is_redis_command_with_leading_whitespace() {
        assert!(is_redis_command("  GET mykey"));
        assert!(is_redis_command("\tSET mykey value"));
        assert!(is_redis_command("  PING"));
    }

    #[test]
    fn test_is_redis_command_with_trailing_whitespace() {
        assert!(is_redis_command("GET mykey  "));
        assert!(is_redis_command("SET mykey value\n"));
    }

    #[test]
    fn test_is_redis_command_not_redis() {
        // Natural language queries should not be recognized
        assert!(
            !is_redis_command("show me all keys"),
            "Natural language should not be recognized"
        );
        assert!(
            !is_redis_command("how many keys do I have"),
            "Natural language should not be recognized"
        );
        assert!(
            !is_redis_command("list all string keys"),
            "Natural language should not be recognized"
        );
        assert!(
            !is_redis_command("find keys with pattern"),
            "Natural language should not be recognized"
        );
    }

    #[test]
    fn test_is_redis_command_empty() {
        assert!(!is_redis_command(""));
        assert!(!is_redis_command("   "));
        assert!(!is_redis_command("\t"));
        assert!(!is_redis_command("\n"));
    }

    #[test]
    fn test_is_redis_command_partial_match() {
        // Partial matches should not work
        assert!(
            !is_redis_command("GETKEYS mykey"),
            "GETKEYS should not match GET"
        );
        assert!(
            !is_redis_command("SETTINGS"),
            "SETTINGS should not match SET"
        );
    }

    #[test]
    fn test_is_redis_command_unknown_command() {
        assert!(
            !is_redis_command("UNKNOWN mykey"),
            "Unknown commands should not be recognized"
        );
        assert!(
            !is_redis_command("FOOBAR"),
            "Random text should not be recognized"
        );
    }

    #[test]
    fn test_is_redis_command_with_args() {
        // Commands with various arguments
        assert!(is_redis_command("SET key value EX 100 NX"));
        assert!(is_redis_command("GET key with spaces"));
        assert!(is_redis_command("HGETALL biglongkeyname"));
        assert!(is_redis_command("SCAN 0 MATCH pattern COUNT 100"));
    }

    #[test]
    fn test_is_redis_command_unicode() {
        // Unicode should not affect command detection
        assert!(is_redis_command("GET 中文key"));
        assert!(is_redis_command("SET key 值为空"));
    }
}
