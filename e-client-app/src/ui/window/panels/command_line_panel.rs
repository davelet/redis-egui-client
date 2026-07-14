use crate::ui::window::RedisApp;
use crate::ui::window::components::markdown::render_markdown;
use crate::ui::window::panels::render_log_viewer;
use crate::ui::window::shortcut_manager::get_shortcut_display;
use crate::ui::window::types::HistoryEntry;
use e_client_basics::constants::REDIS_COMMANDS;
use e_client_config::config::ai_config::AiMode;
use e_client_config::config::shortcuts::ShortcutAction;
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
        // Still drain log lines so capture thread doesn't fall behind
        app.tabs[active_tab_idx]
            .command_line_panel
            .log_viewer
            .drain_received();
        return;
    }

    // Process any pending Redis command results
    process_redis_command_results(app, active_tab_idx);

    // Process any pending AI chat results
    process_ai_chat_results(app, active_tab_idx);

    // Check if a pending cancel time has elapsed — stop capture if so
    if let Some(cancel_time) = app.tabs[active_tab_idx]
        .command_line_panel
        .log_viewer
        .log_cancel_time
        && cancel_time.elapsed().as_secs() >= 3 {
            app.tabs[active_tab_idx]
                .command_line_panel
                .log_viewer
                .stop_capture();
        }

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
                    .selectable_label(
                        current_mode == AiMode::Chat,
                        tr(TranslationKey::CliModeChat, current_lang),
                    )
                    .on_hover_text(tr(TranslationKey::CliModeChatHint, current_lang))
                    .clicked()
                    && current_mode != AiMode::Chat
                {
                    app.tabs[active_tab_idx].command_line_panel.current_mode = AiMode::Chat;
                    clear_agent(app, active_tab_idx);
                }
                if ui
                    .selectable_label(
                        current_mode == AiMode::Agent,
                        tr(TranslationKey::CliModeAgent, current_lang),
                    )
                    .on_hover_text(tr(TranslationKey::CliModeAgentHint, current_lang))
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
                let current_model_id = app.tabs[active_tab_idx]
                    .command_line_panel
                    .current_model_id
                    .clone();
                let active_model_name = current_model_id
                    .as_ref()
                    .and_then(|id| app.config.ai_config.models.iter().find(|m| &m.id == id))
                    .map(|m| m.name.as_str())
                    .or_else(|| {
                        app.config
                            .ai_config
                            .get_active_model()
                            .map(|m| m.name.as_str())
                    })
                    .unwrap_or("--");
                egui::ComboBox::from_id_salt("cli_ai_model_selector")
                    .selected_text(active_model_name)
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        let mut new_model_id: Option<String> = None;
                        for model in &app.config.ai_config.models {
                            let is_selected = current_model_id.as_deref() == Some(&model.id)
                                || (current_model_id.is_none()
                                    && app.config.ai_config.active_model_id.as_deref()
                                        == Some(&model.id));
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

                // Right-aligned section: Live Logs toggle + optional turn counter.
                // Everything is in ONE right_to_left block so nothing overflows the panel.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Live Logs toggle (rightmost item)
                    let log_viewer = &mut app.tabs[active_tab_idx].command_line_panel.log_viewer;
                    let log_binding = app
                        .config
                        .settings
                        .shortcuts
                        .get_binding(&ShortcutAction::ToggleLiveLogs);
                    ui.toggle_value(
                        &mut log_viewer.enabled,
                        format!("{}{}", tr(TranslationKey::LiveLogsToggle, current_lang), get_shortcut_display(&log_binding)),
                    )
                    .on_hover_text(tr(TranslationKey::LiveLogsShowHint, current_lang));

                    // Turn counter (Agent mode only, shown to the left of the toggle)
                    let current_mode = app.tabs[active_tab_idx].command_line_panel.current_mode;
                    if current_mode == AiMode::Agent {
                        let rig_agent = app.tabs[active_tab_idx]
                            .command_line_panel
                            .rig_agent
                            .clone();
                        if let Ok(agent_guard) = rig_agent.try_lock()
                            && let Some(ref agent) = *agent_guard {
                                let turns = agent.turn_count();
                                ui.separator();
                                ui.label(
                                    egui::RichText::new(tr_fmt(
                                        TranslationKey::CliTurns,
                                        current_lang,
                                        &[&turns.to_string()],
                                    ))
                                    .small()
                                    .color(egui::Color32::GRAY),
                                );
                            }
                    }
                });
            });

            ui.separator();

            // ── History + Log Viewer + Input ────────────────────────────────
            //
            // Layout strategy: pin the input row at the very bottom using a
            // bottom_up sub-UI so its position is stable whether or not the
            // log viewer is open.  History and log fill the remaining space.

            // Always drain log lines so the capture thread doesn't fall behind.
            app.tabs[active_tab_idx]
                .command_line_panel
                .log_viewer
                .drain_received();

            let log_viewer_enabled = app.tabs[active_tab_idx]
                .command_line_panel
                .log_viewer
                .enabled;

            // ── Pinned input row at the bottom ──────────────────────────────
            // We render the input first in a bottom_up layout so it always
            // sits flush against the panel's bottom edge.
            let total_height = ui.available_height();
            let input_row_height = 28.0;
            // Reserve space at the bottom for separator + input row.
            let body_height =
                total_height - input_row_height - ui.spacing().item_spacing.y * 2.0 - 1.0; // 1 px separator

            // ── Scrollable body (history + optional log) ────────────────────
            ui.allocate_ui(egui::vec2(ui.available_width(), body_height), |ui| {
                let log_height_reserve = if log_viewer_enabled { 150.0 } else { 0.0 };
                let history_max = (body_height - log_height_reserve).max(40.0);

                // Command history output area
                egui::ScrollArea::vertical()
                    .id_salt(egui::Id::new("cli_history_scroll"))
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(history_max)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Clone history to avoid borrow issues when we need mutable access for abort
                            let history_snapshot: Vec<_> = app.tabs[active_tab_idx]
                                .command_line_panel
                                .history
                                .iter()
                                .map(|e| (e.command.clone(), e.result.clone(), e.translation_key, e.tool_calls.clone()))
                                .collect();
                            for (cmd, result, translation_key, tool_calls) in &history_snapshot {
                                let entry_translation_key = *translation_key;
                                let cmd = cmd;
                                let result = result;
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

                                // Show tool calls if any
                                if !tool_calls.is_empty() {
                                    ui.add_space(4.0);
                                    for tool_call in tool_calls {
                                        ui.horizontal(|ui| {
                                            ui.add_space(16.0);

                                            // Tool icon based on status
                                            let (icon, color) = match tool_call.status {
                                                crate::ui::window::types::ToolCallStatus::Running => ("⏳", egui::Color32::from_rgb(255, 200, 50)),
                                                crate::ui::window::types::ToolCallStatus::Success => ("✅", egui::Color32::from_rgb(80, 200, 120)),
                                                crate::ui::window::types::ToolCallStatus::Error(_) => ("❌", egui::Color32::from_rgb(255, 100, 100)),
                                            };

                                            ui.label(
                                                egui::RichText::new(icon)
                                                    .size(12.0),
                                            );

                                            // Tool name
                                            ui.label(
                                                egui::RichText::new(&tool_call.name)
                                                    .color(color)
                                                    .monospace()
                                                    .small(),
                                            );

                                            // Args summary
                                            if !tool_call.args_summary.is_empty() {
                                                ui.label(
                                                    egui::RichText::new(format!("({})", tool_call.args_summary))
                                                        .color(egui::Color32::GRAY)
                                                        .monospace()
                                                        .small(),
                                                );
                                            }

                                            // Duration
                                            if let Some(ms) = tool_call.duration_ms {
                                                let duration_text = if ms < 1000 {
                                                    format!("{}ms", ms)
                                                } else {
                                                    format!("{:.1}s", ms as f64 / 1000.0)
                                                };
                                                ui.label(
                                                    egui::RichText::new(duration_text)
                                                        .color(egui::Color32::GRAY)
                                                        .monospace()
                                                        .small(),
                                                );
                                            }
                                        });
                                    }
                                    ui.add_space(4.0);
                                }

                                // Result line
                                ui.vertical(|ui| {
                                    ui.add_space(2.0);
                                    let is_thinking =
                                        entry_translation_key == Some(TranslationKey::AiThinking);

                                    if is_thinking {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.add_space(16.0);
                                            let result_color =
                                                egui::Color32::from_rgb(150, 150, 150); // gray for thinking
                                            let thinking_text =
                                                tr(TranslationKey::AiThinking, current_lang);

                                            // Show thinking indicator with inline Stop button
                                            ui.label(
                                                egui::RichText::new(thinking_text)
                                                    .color(result_color)
                                                    .monospace(),
                                            );

                                            // Inline Stop button next to thinking text
                                            let stop_text =
                                                tr(TranslationKey::AiStop, current_lang);
                                            if ui
                                                .button(
                                                    egui::RichText::new(" 🟥")
                                                        .color(egui::Color32::RED),
                                                )
                                                .on_hover_text(stop_text)
                                                .clicked()
                                            {
                                                // Set a flag to abort after the loop
                                                app.tabs[active_tab_idx]
                                                    .command_line_panel
                                                    .abort_requested = true;
                                            }
                                        });
                                    } else {
                                        let render_as_md = app.config.ai_config.render_markdown;
                                        if render_as_md {
                                            render_markdown(result, ui);
                                        } else {
                                            ui.horizontal_wrapped(|ui| {
                                                ui.add_space(16.0);

                                                // Determine styling based on translation key or content
                                                let result_color = if result.starts_with("ERR:")
                                                    || result.starts_with("Error:")
                                                {
                                                    egui::Color32::from_rgb(255, 100, 100)
                                                } else if result.contains(tr(
                                                    TranslationKey::AiInterrupted,
                                                    current_lang,
                                                )) {
                                                    egui::Color32::from_rgb(200, 150, 150)
                                                } else {
                                                    egui::Color32::BLACK
                                                };

                                                ui.label(
                                                    egui::RichText::new(result)
                                                        .color(result_color)
                                                        .monospace(),
                                                )
                                                .on_hover_text(result.clone());
                                            });
                                        }
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

                // Log viewer section (shown between history and input)
                if log_viewer_enabled {
                    ui.add_space(2.0);
                    ui.separator();
                    render_log_viewer(ui, app, active_tab_idx);
                }
            });

            ui.separator();

            // ── Input row (always at the bottom) ────────────────────────────
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("> ")
                        .color(egui::Color32::from_rgb(100, 200, 100))
                        .monospace()
                        .size(14.0),
                );

                let input_width = ui.available_width();
                let response = ui.add_sized(
                    [input_width, ui.available_height()],
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

                // Handle Up/Down keys for history navigation
                if response.has_focus() {
                    let mut history_changed = false;
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        let panel = &mut app.tabs[active_tab_idx].command_line_panel;
                        if !panel.history.is_empty() {
                            if panel.history_index.is_none() {
                                panel.saved_input = panel.input.clone();
                                panel.history_index = Some(panel.history.len() - 1);
                                history_changed = true;
                            } else if let Some(idx) = panel.history_index
                                && idx > 0 {
                                    panel.history_index = Some(idx - 1);
                                    history_changed = true;
                                }
                            if history_changed
                                && let Some(idx) = panel.history_index {
                                    panel.input = panel.history[idx].command.clone();
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

                    if history_changed
                        && let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
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

                // Handle Esc key to close panel
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
        }); // closes outer TopBottomPanel "command_line_panel"
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
                app.tabs[tab_idx]
                    .command_line_panel
                    .log_viewer
                    .log_cancel_time = Some(std::time::Instant::now());
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

                            execute_btn.on_hover_text(tr(
                                TranslationKey::ExecuteCommandHint,
                                current_lang,
                            ));

                            ui.add_space(8.0);

                            let cancel_btn = ui.button(tr(TranslationKey::Cancel, current_lang));
                            if cancel_btn.clicked() {
                                app.tabs[tab_idx].command_line_panel.pending_ai_command = None;
                                app.tabs[tab_idx]
                                    .command_line_panel
                                    .log_viewer
                                    .log_cancel_time = Some(std::time::Instant::now());
                            }

                            // Show skip info if not configured to confirm
                            if !confirm_before_execute {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(tr(
                                        TranslationKey::AutoExecuteEnabled,
                                        current_lang,
                                    ))
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
pub(crate) fn execute_redis_command(app: &mut RedisApp, tab_idx: usize, command: String) {
    if app.tabs[tab_idx]
        .command_line_panel
        .redis_command_pending
        .is_some()
    {
        return;
    }

    let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());
    let client = app.tabs[tab_idx].state.redis_client.clone();
    let (tx, rx) = std::sync::mpsc::channel();

    app.tabs[tab_idx]
        .command_line_panel
        .history
        .push(HistoryEntry::new(
            command.clone(),
            tr(TranslationKey::Executing, current_lang).to_string(),
        ));
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
            // Redis execution complete — stop log capture
            app.tabs[tab_idx]
                .command_line_panel
                .log_viewer
                .stop_capture();
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            app.tabs[tab_idx].command_line_panel.redis_command_pending = Some(pending);
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            let last_idx = app.tabs[tab_idx].command_line_panel.history.len() - 1;
            app.tabs[tab_idx].command_line_panel.history[last_idx].result =
                "ERR: Command execution failed - channel disconnected".to_string();
            // Redis execution failed — stop log capture
            app.tabs[tab_idx]
                .command_line_panel
                .log_viewer
                .stop_capture();
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
            .push(HistoryEntry::new(trimmed_input, error_msg));
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
            .push(HistoryEntry::new(trimmed_input, error_msg));
        return;
    }

    // Show thinking indicator only if configured
    if ai_config.show_ai_thinking {
        app.tabs[tab_idx].command_line_panel.history.push(
            HistoryEntry::new(
                trimmed_input.clone(),
                tr(TranslationKey::AiThinking, current_lang).to_string(),
            )
            .with_translation_key(TranslationKey::AiThinking),
        );
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

    // Get references needed for async task
    let redis_client = std::sync::Arc::new(app.tabs[tab_idx].state.redis_client.clone());
    let ai_config_clone = ai_config.clone();
    let rig_agent_arc = app.tabs[tab_idx].command_line_panel.rig_agent.clone();
    let agent_cache_arc = app.tabs[tab_idx].command_line_panel.agent_cache.clone();
    let mode_for_task = current_mode;
    let per_tab_model_id = app.tabs[tab_idx]
        .command_line_panel
        .current_model_id
        .clone();

    // Store pending state BEFORE spawning to prevent race condition
    // We'll update the handle after spawn completes
    app.tabs[tab_idx].command_line_panel.ai_chat_pending =
        Some(crate::ui::window::types::AiChatPending {
            thinking_idx,
            user_input: user_input.clone(),
            context: None,
            confirm_before_execute,
            receiver: rx,
            handle: tokio::task::spawn(async {}), // placeholder, will be replaced
            use_rig_agent: true,
        });

    // Start log capture so the user can see live logs during AI chat
    app.tabs[tab_idx]
        .command_line_panel
        .log_viewer
        .start_capture();

    // Execute AI chat with rig agent asynchronously
    let task_user_input = user_input.clone();
    let handle = tokio::task::spawn(async move {
        // Determine which model to use: per-tab > global active
        let model_id = per_tab_model_id.or_else(|| ai_config_clone.active_model_id.clone());
        let cache_key = format!(
            "{}:{:?}",
            model_id.clone().unwrap_or_default(),
            mode_for_task
        );

        // Try to get agent from cache first
        let mut agent_cache = agent_cache_arc.lock().await;
        let mut agent_opt = rig_agent_arc.lock().await;

        // Check if we have a cached agent
        if agent_opt.is_none()
            && let Some(cached_agent) = agent_cache.remove(&cache_key) {
                *agent_opt = Some(cached_agent);
            }

        // If agent still doesn't exist, try to create it first
        if agent_opt.is_none() {
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
            if let Some(m) = config_with_key
                .models
                .iter_mut()
                .find(|m| m.id == model_with_key.id)
            {
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
            agent.chat(&task_user_input).await
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
        } else if let Some(agent) = agent_opt.take() {
            // For Agent mode, put agent back to cache after use
            agent_cache.insert(cache_key, agent);
        }

        let _ = tx.send(chat_response);
    });

    // Update pending with the actual handle
    if let Some(ref mut pending) = app.tabs[tab_idx].command_line_panel.ai_chat_pending {
        pending.handle = handle;
    }
}

/// Abort a pending AI chat operation
pub fn abort_ai_chat(app: &mut RedisApp, tab_idx: usize) {
    if let Some(pending) = app.tabs[tab_idx].command_line_panel.ai_chat_pending.take() {
        // 1. Abort the async task
        pending.handle.abort();

        // 2. Stop log capture
        app.tabs[tab_idx]
            .command_line_panel
            .log_viewer
            .stop_capture();

        // 3. Update history if there was a thinking indicator
        if let Some(history_idx) = pending.thinking_idx
            && history_idx < app.tabs[tab_idx].command_line_panel.history.len() {
                let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());
                let interrupted_text = tr(TranslationKey::AiInterrupted, current_lang);
                let entry = &mut app.tabs[tab_idx].command_line_panel.history[history_idx];
                entry.result = format!("_({})_", interrupted_text);
                entry.translation_key = None;
            }
    }
}

/// Process pending AI chat results (call this each frame)
pub fn process_ai_chat_results(app: &mut RedisApp, tab_idx: usize) {
    // Check if abort was requested
    if app.tabs[tab_idx].command_line_panel.abort_requested {
        app.tabs[tab_idx].command_line_panel.abort_requested = false;
        if app.tabs[tab_idx]
            .command_line_panel
            .ai_chat_pending
            .is_some()
        {
            abort_ai_chat(app, tab_idx);
            return;
        }
    }

    let pending = match app.tabs[tab_idx].command_line_panel.ai_chat_pending.take() {
        Some(p) => p,
        None => return,
    };

    // Check if result is ready
    let result = match pending.receiver.try_recv() {
        Ok(r) => {
            // AI responded — do NOT stop capture here yet.
            // Capture continues until Redis execution completes (or user cancels).
            r
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            // Not ready yet, put it back
            app.tabs[tab_idx].command_line_panel.ai_chat_pending = Some(pending);
            return;
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            // Channel disconnected — stop capture with delay so user sees final logs
            app.tabs[tab_idx]
                .command_line_panel
                .log_viewer
                .log_cancel_time = Some(std::time::Instant::now());
            if let Some(idx) = pending.thinking_idx {
                app.tabs[tab_idx].command_line_panel.history[idx] = HistoryEntry::new(
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
                    let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history[idx] = HistoryEntry::new(
                            pending.user_input.clone(),
                            tr(TranslationKey::Executing, current_lang).to_string(),
                        );
                    }

                    execute_redis_command(app, tab_idx, trimmed_response.to_string());

                    // Remove the "Executing..." message
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history.remove(idx);
                    }
                }
            } else {
                // Non-Redis command response — mark as markdown (AI response)
                // Parse tool calls from logs
                let log_lines: Vec<String> = app.tabs[tab_idx]
                    .command_line_panel
                    .log_viewer
                    .log_lines
                    .iter()
                    .cloned()
                    .collect();
                let tool_calls = crate::ui::window::types::parse_tool_calls_from_logs(&log_lines);

                if let Some(idx) = pending.thinking_idx {
                    app.tabs[tab_idx].command_line_panel.history[idx] =
                        HistoryEntry::new(pending.user_input, response).with_tool_calls(tool_calls);
                } else {
                    app.tabs[tab_idx].command_line_panel.history.push(
                        HistoryEntry::new(pending.user_input, response).with_tool_calls(tool_calls),
                    );
                }
                // No Redis execution needed — stop capture with delay
                app.tabs[tab_idx]
                    .command_line_panel
                    .log_viewer
                    .log_cancel_time = Some(std::time::Instant::now());
            }
        }
        Err(e) => {
            // Get current language for i18n
            let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());

            match e {
                AiResponseError::CommandNotWhitelisted(cmd) => {
                    // Command blocked by whitelist, show confirmation dialog
                    app.pending_unsafe_command =
                        Some((tab_idx, cmd.clone(), pending.user_input.clone()));
                    // Update history with temporary message
                    let msg = format!(
                        "⚠️  Command `{}` is not in whitelist, waiting for confirmation...",
                        cmd
                    );
                    if let Some(idx) = pending.thinking_idx {
                        app.tabs[tab_idx].command_line_panel.history[idx] =
                            HistoryEntry::new(pending.user_input, msg);
                    } else {
                        app.tabs[tab_idx]
                            .command_line_panel
                            .history
                            .push(HistoryEntry::new(pending.user_input, msg));
                    }
                    // Don't stop capture yet, wait for user decision
                }
                _ => {
                    // Regular error
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
                            HistoryEntry::new(pending.user_input, error_msg);
                    } else {
                        app.tabs[tab_idx]
                            .command_line_panel
                            .history
                            .push(HistoryEntry::new(pending.user_input, error_msg));
                    }

                    // Error occurred — stop capture with delay
                    app.tabs[tab_idx]
                        .command_line_panel
                        .log_viewer
                        .log_cancel_time = Some(std::time::Instant::now());
                }
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
