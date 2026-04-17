//! Keyboard shortcut settings UI components

use e_client_basics::emoji;
use e_client_config::config::shortcuts::ShortcutAction;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};

use super::super::super::RedisApp;

/// All supported keyboard keys for shortcuts
pub const SUPPORTED_KEYS: [egui::Key; 56] = [
    egui::Key::A,
    egui::Key::B,
    egui::Key::C,
    egui::Key::D,
    egui::Key::E,
    egui::Key::F,
    egui::Key::G,
    egui::Key::H,
    egui::Key::I,
    egui::Key::J,
    egui::Key::K,
    egui::Key::L,
    egui::Key::M,
    egui::Key::N,
    egui::Key::O,
    egui::Key::P,
    egui::Key::Q,
    egui::Key::R,
    egui::Key::S,
    egui::Key::T,
    egui::Key::U,
    egui::Key::V,
    egui::Key::W,
    egui::Key::X,
    egui::Key::Y,
    egui::Key::Z,
    egui::Key::Num0,
    egui::Key::Num1,
    egui::Key::Num2,
    egui::Key::Num3,
    egui::Key::Num4,
    egui::Key::Num5,
    egui::Key::Num6,
    egui::Key::Num7,
    egui::Key::Num8,
    egui::Key::Num9,
    egui::Key::F1,
    egui::Key::F2,
    egui::Key::F3,
    egui::Key::F4,
    egui::Key::F5,
    egui::Key::F6,
    egui::Key::F7,
    egui::Key::F8,
    egui::Key::F9,
    egui::Key::F10,
    egui::Key::F11,
    egui::Key::F12,
    egui::Key::Comma,
    egui::Key::Period,
    egui::Key::Semicolon,
    egui::Key::Enter,
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
];

/// Parse action key string to ShortcutAction enum
pub fn parse_action_from_key(key: &str) -> Option<ShortcutAction> {
    match key {
        "NewConnection" => Some(ShortcutAction::NewConnection),
        "ConnectAllUnclosed" => Some(ShortcutAction::ConnectAllUnclosed),
        "ConnectConnection1" => Some(ShortcutAction::ConnectConnection1),
        "ConnectConnection2" => Some(ShortcutAction::ConnectConnection2),
        "ConnectConnection3" => Some(ShortcutAction::ConnectConnection3),
        "ConnectConnection4" => Some(ShortcutAction::ConnectConnection4),
        "ConnectConnection5" => Some(ShortcutAction::ConnectConnection5),
        "ConnectConnection6" => Some(ShortcutAction::ConnectConnection6),
        "ConnectConnection7" => Some(ShortcutAction::ConnectConnection7),
        "ConnectConnection8" => Some(ShortcutAction::ConnectConnection8),
        "ConnectConnection9" => Some(ShortcutAction::ConnectConnection9),
        "NewTab" => Some(ShortcutAction::NewTab),
        "CloseTab" => Some(ShortcutAction::CloseTab),
        "RefreshKey" => Some(ShortcutAction::RefreshKey),
        "FocusFilter" => Some(ShortcutAction::FocusFilter),
        "NewKey" => Some(ShortcutAction::NewKey),
        "CloseSettings" => Some(ShortcutAction::CloseSettings),
        "OpenSettings" => Some(ShortcutAction::OpenSettings),
        "ToggleCommandLine" => Some(ShortcutAction::ToggleCommandLine),
        "CloseCommandLine" => Some(ShortcutAction::CloseCommandLine),
        "SwitchToTab1" => Some(ShortcutAction::SwitchToTab1),
        "SwitchToTab2" => Some(ShortcutAction::SwitchToTab2),
        "SwitchToTab3" => Some(ShortcutAction::SwitchToTab3),
        "SwitchToTab4" => Some(ShortcutAction::SwitchToTab4),
        "SwitchToTab5" => Some(ShortcutAction::SwitchToTab5),
        "SwitchToTab6" => Some(ShortcutAction::SwitchToTab6),
        "SwitchToTab7" => Some(ShortcutAction::SwitchToTab7),
        "SwitchToTab8" => Some(ShortcutAction::SwitchToTab8),
        "SwitchToTab9" => Some(ShortcutAction::SwitchToTab9),
        "SwitchToLastTab" => Some(ShortcutAction::SwitchToLastTab),
        "RemoveDuplicateAndInvalidTabs" => Some(ShortcutAction::RemoveDuplicateAndInvalidTabs),
        "RefreshKeys" => Some(ShortcutAction::RefreshKeys),
        "ExecuteAiCommand" => Some(ShortcutAction::ExecuteAiCommand),
        "CancelAiCommand" => Some(ShortcutAction::CancelAiCommand),
        _ => None,
    }
}

/// Parse key string to egui::Key
/// Handles both short format ("4") and Debug format ("Num4")
pub fn parse_key_from_str(key: &str) -> Option<egui::Key> {
    match key {
        // Letters
        "A" => Some(egui::Key::A),
        "B" => Some(egui::Key::B),
        "C" => Some(egui::Key::C),
        "D" => Some(egui::Key::D),
        "E" => Some(egui::Key::E),
        "F" => Some(egui::Key::F),
        "G" => Some(egui::Key::G),
        "H" => Some(egui::Key::H),
        "I" => Some(egui::Key::I),
        "J" => Some(egui::Key::J),
        "K" => Some(egui::Key::K),
        "L" => Some(egui::Key::L),
        "M" => Some(egui::Key::M),
        "N" => Some(egui::Key::N),
        "O" => Some(egui::Key::O),
        "P" => Some(egui::Key::P),
        "Q" => Some(egui::Key::Q),
        "R" => Some(egui::Key::R),
        "S" => Some(egui::Key::S),
        "T" => Some(egui::Key::T),
        "U" => Some(egui::Key::U),
        "V" => Some(egui::Key::V),
        "W" => Some(egui::Key::W),
        "X" => Some(egui::Key::X),
        "Y" => Some(egui::Key::Y),
        "Z" => Some(egui::Key::Z),
        // Numbers - handle both "4" and "Num4" formats
        "0" | "Num0" => Some(egui::Key::Num0),
        "1" | "Num1" => Some(egui::Key::Num1),
        "2" | "Num2" => Some(egui::Key::Num2),
        "3" | "Num3" => Some(egui::Key::Num3),
        "4" | "Num4" => Some(egui::Key::Num4),
        "5" | "Num5" => Some(egui::Key::Num5),
        "6" | "Num6" => Some(egui::Key::Num6),
        "7" | "Num7" => Some(egui::Key::Num7),
        "8" | "Num8" => Some(egui::Key::Num8),
        "9" | "Num9" => Some(egui::Key::Num9),
        // Punctuation
        "," | "Comma" => Some(egui::Key::Comma),
        "." | "Period" => Some(egui::Key::Period),
        ";" | "Semicolon" => Some(egui::Key::Semicolon),
        // Special keys
        "Enter" | "Return" => Some(egui::Key::Enter),
        // Arrow keys
        "Up" | "ArrowUp" => Some(egui::Key::ArrowUp),
        "Down" | "ArrowDown" => Some(egui::Key::ArrowDown),
        "Left" | "ArrowLeft" => Some(egui::Key::ArrowLeft),
        "Right" | "ArrowRight" => Some(egui::Key::ArrowRight),
        _ => None,
    }
}

/// Render keyboard shortcut settings section
pub fn render_shortcut_settings(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    current_lang: Language,
) {
    // Check if we're currently capturing a shortcut
    let is_capturing = app.shortcut_state.editing_shortcut.is_some();

    // Show hint about underlined shortcuts only if there are customized shortcuts
    let has_customized_shortcuts = ShortcutAction::all_actions().iter().any(|(action, _)| {
        !action.is_non_editable() && app.config.settings.shortcuts.is_customized(action)
    });
    if has_customized_shortcuts {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(tr(TranslationKey::ShortcutUnderlineHint, current_lang))
                    .color(ui.visuals().weak_text_color())
                    .size(12.0),
            );
        });
        ui.add_space(4.0);
    }

    if is_capturing {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            tr(TranslationKey::ShortcutPressKeys, current_lang),
        );
        ui.add_space(8.0);
    }

    // Show conflict warning if any
    if let Some(ref warning) = app.shortcut_state.shortcut_conflict_warning {
        ui.colored_label(
            ui.visuals().error_fg_color,
            format!("{} {}", emoji::action::WARNING, warning),
        );
        ui.add_space(8.0);
    }

    // Capture shortcut input if in editing mode
    if is_capturing {
        ctx.input(|i| {
            // Check for Enter key to save
            if i.key_pressed(egui::Key::Enter) {
                let has_conflict = app.shortcut_state.shortcut_conflict_warning.is_some();
                let can_save =
                    !app.shortcut_state.shortcut_input_buffer.is_empty() && !has_conflict;
                if can_save {
                    if let Some(current_action_str) = &app.shortcut_state.editing_shortcut {
                        if let Some(current_action) = parse_action_from_key(current_action_str) {
                            app.config.settings.shortcuts.set_binding(
                                &current_action,
                                app.shortcut_state.shortcut_input_buffer.clone(),
                            );
                            app.config.mark_settings_dirty();
                            app.shortcut_state.editing_shortcut = None;
                            app.shortcut_state.shortcut_input_buffer.clear();
                            app.shortcut_state.shortcut_conflict_warning = None;
                        }
                    }
                }
            }

            // Check for Escape key to cancel
            if i.key_pressed(egui::Key::Escape) {
                app.shortcut_state.editing_shortcut = None;
                app.shortcut_state.shortcut_input_buffer.clear();
                app.shortcut_state.shortcut_conflict_warning = None;
            }

            let modifiers = i.modifiers;
            for key in SUPPORTED_KEYS {
                if i.key_pressed(key) {
                    let mut parts = Vec::new();
                    // Platform-specific modifier key handling:
                    // - macOS: "Cmd" for Command key, "Ctrl" for Control key
                    // - Other: "Ctrl" for both Ctrl and Command (they're unified)
                    if cfg!(target_os = "macos") {
                        if modifiers.command {
                            parts.push("Cmd".to_string());
                        }
                        if modifiers.ctrl {
                            parts.push("Ctrl".to_string());
                        }
                    } else {
                        if modifiers.ctrl || modifiers.command {
                            parts.push("Ctrl".to_string());
                        }
                    }
                    if modifiers.alt {
                        parts.push("Alt".to_string());
                    }
                    if modifiers.shift {
                        parts.push("Shift".to_string());
                    }
                    parts.push(format!("{:?}", key));
                    app.shortcut_state.shortcut_input_buffer = parts.join("+");

                    // Check for conflicts
                    if let Some(current_action_str) = &app.shortcut_state.editing_shortcut {
                        if let Some(current_action) = parse_action_from_key(current_action_str) {
                            if let Some(conflict_action) =
                                app.config.settings.shortcuts.check_conflict(
                                    &app.shortcut_state.shortcut_input_buffer,
                                    &current_action,
                                )
                            {
                                let conflict_name =
                                    tr(conflict_action.translation_key(), current_lang);
                                app.shortcut_state.shortcut_conflict_warning = Some(tr_fmt(
                                    TranslationKey::ShortcutConflictsWith,
                                    current_lang,
                                    &[conflict_name],
                                ));
                            } else {
                                app.shortcut_state.shortcut_conflict_warning = None;
                            }
                        }
                    }
                }
            }
        });
    }

    egui::Frame::group(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .show(ui, |ui| {
            ui.set_min_width(400.0);
            egui::Grid::new("shortcuts_grid")
                .num_columns(3)
                .spacing([20.0, 8.0])
                .min_col_width(120.0)
                .show(ui, |ui| {
                    for (action, trans_key) in ShortcutAction::all_actions() {
                        let action_key = format!("{:?}", action);
                        let is_editing =
                            app.shortcut_state.editing_shortcut.as_ref() == Some(&action_key);

                        // Check if this shortcut is non-editable
                        let is_non_editable = action.is_non_editable();

                        // Check if this is a customized shortcut
                        let is_customized = !is_non_editable
                            && app.config.settings.shortcuts.is_customized(&action);

                        // Action name with bold font (translated), underline for customized
                        ui.horizontal(|ui| {
                            let text = egui::RichText::new(tr(trans_key, current_lang));
                            if is_customized {
                                ui.label(text.underline());
                            } else {
                                ui.label(text);
                            }
                        });

                        // Current shortcut display or input
                        if is_editing {
                            let display_text = app.shortcut_state.shortcut_input_buffer.clone();
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut display_text.clone())
                                    .desired_width(120.0)
                                    .interactive(false)
                                    .font(egui::TextStyle::Monospace),
                            );
                            // Highlight the editing field
                            ui.painter().rect_stroke(
                                response.rect.expand(2.0),
                                4.0,
                                ui.visuals().selection.stroke,
                                egui::StrokeKind::Inside,
                            );
                        } else {
                            let binding = app.config.settings.shortcuts.get_binding(&action);
                            ui.monospace(&binding);
                        }

                        // Edit/Save/Cancel buttons or "Not customizable" text
                        ui.horizontal(|ui| {
                            if is_non_editable {
                                // Show "Not customizable" text for non-editable shortcuts
                                ui.label(
                                    egui::RichText::new(tr(
                                        TranslationKey::ShortcutNonEditable,
                                        current_lang,
                                    ))
                                    .color(ui.visuals().weak_text_color())
                                    .italics()
                                    .size(12.0),
                                );
                            } else if is_editing {
                                // Check if there's a conflict before allowing save
                                let has_conflict =
                                    app.shortcut_state.shortcut_conflict_warning.is_some();
                                let can_save = !app.shortcut_state.shortcut_input_buffer.is_empty()
                                    && !has_conflict;

                                ui.visuals_mut().override_text_color =
                                    Some(ui.visuals().selection.bg_fill);
                                let save_text = format!(
                                    "{} {}",
                                    emoji::action::SAVE,
                                    tr(TranslationKey::Save, current_lang)
                                );
                                if ui
                                    .add_enabled(can_save, egui::Button::new(save_text))
                                    .clicked()
                                {
                                    app.config.settings.shortcuts.set_binding(
                                        &action,
                                        app.shortcut_state.shortcut_input_buffer.clone(),
                                    );
                                    app.config.mark_settings_dirty();
                                    app.shortcut_state.editing_shortcut = None;
                                    app.shortcut_state.shortcut_input_buffer.clear();
                                    app.shortcut_state.shortcut_conflict_warning = None;
                                }
                                ui.visuals_mut().override_text_color = None;
                                let cancel_text = format!(
                                    "{} {}",
                                    emoji::action::CANCEL,
                                    tr(TranslationKey::Cancel, current_lang)
                                );
                                if ui.button(cancel_text).clicked() {
                                    app.shortcut_state.editing_shortcut = None;
                                    app.shortcut_state.shortcut_input_buffer.clear();
                                    app.shortcut_state.shortcut_conflict_warning = None;
                                }
                            } else {
                                let edit_text = format!(
                                    "{} {}",
                                    emoji::action::EDIT,
                                    tr(TranslationKey::Edit, current_lang)
                                );
                                if ui.button(edit_text).clicked() {
                                    app.shortcut_state.editing_shortcut = Some(action_key);
                                    // Initialize with current binding
                                    app.shortcut_state.shortcut_input_buffer =
                                        app.config.settings.shortcuts.get_binding(&action);
                                    app.shortcut_state.shortcut_conflict_warning = None;
                                }
                            }
                        });
                        ui.end_row();
                    }
                });
        });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let reset_btn_id = egui::Id::new("shortcut_reset_defaults");
        let reset_btn_color = app.action_button_text_color(reset_btn_id);
        let reset_btn_text = app.copy_button_text(
            reset_btn_id,
            &format!(
                "{} {}",
                emoji::action::REFRESH,
                tr(TranslationKey::ShortcutResetDefaults, current_lang)
            ),
        );
        if ui
            .button(egui::RichText::new(reset_btn_text).color(reset_btn_color))
            .clicked()
        {
            app.config.settings.shortcuts.reset_to_default();
            app.config.mark_settings_dirty();
            app.record_action_success(reset_btn_id);
            // Clear editing state to refresh the display
            app.shortcut_state.editing_shortcut = None;
            app.shortcut_state.shortcut_input_buffer.clear();
            app.shortcut_state.shortcut_conflict_warning = None;
        }
    });
}
