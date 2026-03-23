use crate::ui::window::RedisApp;
use e_client_basics::constants::REDIS_COMMANDS;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};
use e_client_core::AiClient;

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

/// Build Redis context for AI system prompt
fn build_redis_context(app: &RedisApp, tab_idx: usize) -> Option<String> {
    let tab = app.tabs.get(tab_idx)?;

    // Check if connected
    let connected = app.poll_bool(tab.state.connected.clone());
    if !connected {
        return None;
    }

    // Get connection info from connection_param
    let conn_param = tab.state.connection_param.blocking_read();

    // Get current database
    let current_db = app.poll_u32(tab.state.current_db.clone());

    // Get selected key if any
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());

    // Get system prompt from config
    let system_prompt = &app.config.ai_config.system_prompt;

    // Build context: system prompt + Redis connection info
    let mut context = system_prompt.clone();

    if let Some(conn) = conn_param.as_ref() {
        context.push_str(&format!("\n\nConnection name: {}", conn.name));
        context.push_str(&format!("\nConnection address: {}:{}", conn.url, conn.port));
        if let Some(db) = conn.database {
            context.push_str(&format!("\nDefault database: {}", db));
        }
    }

    context.push_str(&format!("\nCurrent database: {}", current_db));

    if let Some(key) = selected_key {
        context.push_str(&format!("\nCurrently selected key: {}", key));
    }

    Some(context)
}

pub fn render_command_line_panel(app: &mut RedisApp, ctx: &egui::Context) {
    let active_tab_idx = app.active_tab;

    // Early return if panel is hidden for current tab
    if !app.tabs[active_tab_idx].command_line_panel.show {
        return;
    }

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
        .exact_height(ctx.screen_rect().height() / 4.0)
        .show(ctx, |ui| {
            // Command history output area
            let available_height = ui.available_height() - 40.0; // Reserve space for input
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(available_height)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        let history = app.tabs[active_tab_idx].command_line_panel.history.clone();
                        for (cmd, result) in &history {
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
                            ui.horizontal(|ui| {
                                ui.add_space(16.0);
                                if result.starts_with("ERR:") || result.starts_with("Error:") {
                                    ui.label(
                                        egui::RichText::new(result)
                                            .color(egui::Color32::from_rgb(255, 100, 100))
                                            .monospace(),
                                    );
                                } else if result == "Thinking..." {
                                    ui.label(
                                        egui::RichText::new(result)
                                            .color(egui::Color32::from_rgb(150, 150, 150))
                                            .monospace(),
                                    );
                                } else {
                                    ui.label(
                                        egui::RichText::new(result)
                                            .color(egui::Color32::BLACK)
                                            .monospace(),
                                    );
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
                    .hint_text(tr(keys::COMMAND_LINE_HINT, current_lang))
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
                                    panel.input = panel.history[idx].0.clone();
                                }
                            }
                        }
                    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        let panel = &mut app.tabs[active_tab_idx].command_line_panel;
                        if let Some(idx) = panel.history_index {
                            if idx + 1 < panel.history.len() {
                                panel.history_index = Some(idx + 1);
                                panel.input = panel.history[idx + 1].0.clone();
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

    let screen = ctx.screen_rect();
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

    egui::Window::new(tr(keys::AI_CONFIRM_DIALOG_TITLE, current_lang))
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
                            egui::RichText::new(tr(keys::AI_CONFIRM_DIALOG_TITLE, current_lang))
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
                                egui::RichText::new(tr(keys::AI_EXECUTE, current_lang))
                                    .color(ui.visuals().text_color()),
                            );

                            if execute_btn.clicked() {
                                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
                                execute_redis_command(app, tab_idx, redis_cmd.to_string());
                            }

                            execute_btn.on_hover_text("Execute the Redis command");

                            ui.add_space(8.0);

                            let cancel_btn = ui.button(tr(keys::CANCEL, current_lang));
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
    let client = app.tabs[tab_idx].state.redis_client.clone();
    match client.execute_raw_command_sync(&command) {
        Ok(output) => {
            app.tabs[tab_idx]
                .command_line_panel
                .history
                .push((command, output));
        }
        Err(e) => {
            app.tabs[tab_idx]
                .command_line_panel
                .history
                .push((command, format!("ERR: {}", e)));
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

    if !ai_config.enabled {
        app.tabs[tab_idx].command_line_panel.history.push((
            trimmed_input,
            "ERR: AI is disabled. Enable it in settings.".to_string(),
        ));
        return;
    }

    let active_model = ai_config.get_active_model();
    if active_model.is_none() {
        app.tabs[tab_idx].command_line_panel.history.push((
            trimmed_input,
            "ERR: No AI model configured. Please configure an AI model in settings.".to_string(),
        ));
        return;
    }

    let model = active_model.unwrap();

    // Build Redis context
    let context = build_redis_context(app, tab_idx);

    // Show thinking indicator only if configured
    if ai_config.show_ai_thinking {
        app.tabs[tab_idx]
            .command_line_panel
            .history
            .push((trimmed_input.clone(), "Thinking...".to_string()));
    }

    // Get the history index of the thinking message (if shown)
    let thinking_idx = if ai_config.show_ai_thinking {
        Some(app.tabs[tab_idx].command_line_panel.history.len() - 1)
    } else {
        None
    };

    // Clone model and context for the async operation
    let model = model.clone();
    let context_str = context.clone();
    let user_input = trimmed_input.clone();

    // Create channel for async result
    let (tx, rx) = std::sync::mpsc::channel();

    // Store pending state
    app.tabs[tab_idx].command_line_panel.ai_chat_pending =
        Some(crate::ui::window::types::AiChatPending {
            thinking_idx,
            user_input: trimmed_input,
            context: context_str.clone(),
            confirm_before_execute: ai_config.confirm_before_execute,
            receiver: rx,
        });

    // Execute AI chat asynchronously
    tokio::task::spawn_blocking(move || {
        let result = AiClient::chat_sync(&model, &user_input, context_str.as_deref());
        let _ = tx.send(result);
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
                app.tabs[tab_idx].command_line_panel.history[idx] = (
                    pending.user_input,
                    "ERR: AI request failed - channel disconnected".to_string(),
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
                            (pending.user_input.clone(), "Executing...".to_string());
                    }

                    execute_redis_command(app, tab_idx, trimmed_response.to_string());

                    // Remove the "Executing..." message
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history.remove(idx);
                    }
                }
            } else {
                // Non-Redis command response
                if let Some(idx) = pending.thinking_idx {
                    app.tabs[tab_idx].command_line_panel.history[idx] =
                        (pending.user_input, response);
                } else {
                    app.tabs[tab_idx]
                        .command_line_panel
                        .history
                        .push((pending.user_input, response));
                }
            }
        }
        Err(e) => {
            // Update the thinking message with the error
            if let Some(idx) = pending.thinking_idx {
                app.tabs[tab_idx].command_line_panel.history[idx] = (
                    pending.user_input,
                    format!("ERR: AI request failed - {}", e),
                );
            } else {
                app.tabs[tab_idx].command_line_panel.history.push((
                    pending.user_input,
                    format!("ERR: AI request failed - {}", e),
                ));
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
