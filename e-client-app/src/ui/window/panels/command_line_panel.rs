use crate::ui::window::RedisApp;

pub fn render_command_line_panel(app: &mut RedisApp, ctx: &egui::Context) {
    let active_tab_idx = app.active_tab;

    // Early return if panel is hidden for current tab
    if !app.tabs[active_tab_idx].command_line_panel.show {
        return;
    }

    let _current_lang = app.poll_language(app.tabs[active_tab_idx].state.language.clone());

    egui::TopBottomPanel::bottom("command_line_panel")
        .resizable(true)
        .min_height(150.0)
        .default_height(200.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Command history output area
                let available_height = ui.available_height() - 40.0; // Reserve space for input
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(available_height)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let history =
                                app.tabs[active_tab_idx].command_line_panel.history.clone();
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
                        .hint_text("Enter Redis command...")
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
                            if let Some(mut state) =
                                egui::TextEdit::load_state(ui.ctx(), response.id)
                            {
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

                    // Handle Esc key to close panel
                    if !app.show_settings && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        app.tabs[active_tab_idx].command_line_panel.show = false;
                    }
                });
            });
        });
}

fn execute_command(app: &mut RedisApp, tab_idx: usize, command: String) {
    // Execute the command using the redis client
    let client = app.tabs[tab_idx].state.redis_client.clone();
    let result = client.execute_raw_command_sync(&command);
    match result {
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
