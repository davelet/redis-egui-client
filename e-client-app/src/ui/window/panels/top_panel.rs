use crate::ui::window::RedisApp;
use e_client_basics::constants::WILD_KEY_FILTER;
use e_client_config::config::shortcuts::ShortcutAction;
use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::emoji;
use e_client_config::translations::keys;
use e_client_config::translations::{tr, tr_fmt};

/// All supported keyboard keys for shortcuts
pub const SUPPORTED_KEYS: [egui::Key; 51] = [
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
];

/// Settings window dimensions
const SETTINGS_WINDOW_WIDTH: f32 = 450.0;
const SETTINGS_WINDOW_HEIGHT: f32 = 200.0;

/// Parse action key string to ShortcutAction enum
fn parse_action_from_key(key: &str) -> Option<ShortcutAction> {
    match key {
        "NewTab" => Some(ShortcutAction::NewTab),
        "CloseTab" => Some(ShortcutAction::CloseTab),
        "RefreshKey" => Some(ShortcutAction::RefreshKey),
        "FocusFilter" => Some(ShortcutAction::FocusFilter),
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
        _ => None,
    }
}

/// Parse key string to egui::Key
/// Handles both short format ("4") and Debug format ("Num4")
fn parse_key_from_str(key: &str) -> Option<egui::Key> {
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
        _ => None,
    }
}

pub fn render_top_panel(app: &mut RedisApp, ctx: &egui::Context) {
    // Early return if no active tab
    if app.get_active_tab().is_none() {
        return;
    }

    // Get all needed data before entering UI closure
    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let selected_connection = tab.selected_connection;
    let connected = app.poll_bool(tab.state.connected.clone());
    let _loading = app.poll_bool(tab.state.loading.clone());
    let current_db = app.poll_u32(tab.state.current_db.clone());
    let databases = app.poll_vec_u32(tab.state.databases.clone());

    let mut create_new_tab_with: Option<(
        usize,
        e_client_config::connection::RedisConnectionConfig,
    )> = None;
    let mut auto_connect_idx: Option<usize> = None;

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Show connection color indicator for current connection
            if let Some(idx) = selected_connection {
                if let Some(conn) = app.config.connections.get(idx) {
                    if let Some(color_hex) = &conn.color {
                        if let Some(color) = parse_color_hex(color_hex) {
                            ui.colored_label(color, "●");
                        }
                    }
                }
            }

            ui.label(tr(keys::CONNECTION_URL, current_lang));

            // Connection dropdown - disabled when connected
            let selected_name = selected_connection
                .and_then(|idx| app.config.connections.get(idx))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| tr(keys::SELECT_CONNECTION, current_lang).to_string());

            ui.add_enabled_ui(!connected, |ui| {
                egui::ComboBox::from_id_salt("connection_select")
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (idx, conn) in app.config.connections.connections.iter().enumerate() {
                            let tab = &mut app.tabs[active_tab_idx];
                            let is_selected = tab.selected_connection == Some(idx);
                            ui.horizontal(|ui| {
                                // Show color indicator for each connection in dropdown
                                if let Some(color_hex) = &conn.color {
                                    if let Some(color) = parse_color_hex(color_hex) {
                                        ui.colored_label(color, "●");
                                    }
                                } else {
                                    ui.label("  "); // Placeholder for alignment
                                }
                                if ui.selectable_label(is_selected, &conn.name).clicked() {
                                    tab.selected_connection = Some(idx);

                                    // Auto connect if enabled
                                    if app.config.settings.auto_connect {
                                        auto_connect_idx = Some(idx);
                                    }
                                }
                            });
                        }
                    });
            });

            if connected {
                if ui.button(tr(keys::DISCONNECT, current_lang)).clicked() {
                    app.tabs[active_tab_idx].state.spawn_disconnect();
                    // Clear tab name, color, and filter on disconnect, but keep selected_connection
                    let tab = &mut app.tabs[active_tab_idx];
                    tab.name = format!("{} {}", tr(keys::TAB, current_lang), tab.id);
                    tab.connected_color = None;
                    tab.key_filter_input.clear();

                    let key_filter = tab.state.key_filter.clone();
                    app.update_string(key_filter, WILD_KEY_FILTER.to_string());
                }

                ui.separator();
                ui.label(tr(keys::DATABASE, current_lang));

                egui::ComboBox::from_id_salt("db_select")
                    .selected_text(format!("DB {}", current_db))
                    .show_ui(ui, |ui| {
                        for db in databases {
                            if ui
                                .selectable_label(current_db == db, format!("DB {}", db))
                                .clicked()
                            {
                                app.tabs[active_tab_idx].state.spawn_select_db(db);
                                // Save DB preference
                                let conn_idx = app.tabs[active_tab_idx].selected_connection;
                                if let Some(idx) = conn_idx {
                                    let conn_name =
                                        app.config.connections.get(idx).map(|c| c.name.clone());
                                    if let Some(name) = conn_name {
                                        let _ = app.update_db_for_connection(&name, db);
                                    }
                                }
                            }
                        }
                    });
            } else {
                if ui.button(tr(keys::CONNECT, current_lang)).clicked() {
                    let tab = &mut app.tabs[active_tab_idx];
                    if let Some(idx) = tab.selected_connection {
                        if let Some(conn) = app.config.connections.get(idx) {
                            let conn_clone = conn.clone();
                            *tab.state.connection_param.blocking_write() = Some(conn_clone.clone());
                            // Update tab name and color to connection name and color
                            tab.name = conn.name.clone();
                            tab.connected_color = conn.color.clone();

                            // Load preferences for this connection
                            app.load_connection_preferences(active_tab_idx);

                            // Connect with preferred DB
                            app.spawn_connect_with_initial_db(active_tab_idx);
                        }
                    } else {
                        // No connection selected - show error
                        *tab.state.error_message.blocking_write() =
                            tr(keys::PLEASE_SELECT_CONNECTION, current_lang).to_string();
                    }
                }

                // "Open in New Tab" button
                if let Some(idx) = selected_connection {
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::navigation::NEW_TAB,
                            tr(keys::OPEN_IN_NEW_TAB, current_lang)
                        ))
                        .clicked()
                    {
                        if let Some(conn) = app.config.connections.get(idx) {
                            create_new_tab_with = Some((idx, conn.clone()));
                        }
                    }
                }
            }

            ui.separator();

            // Connection management buttons - disabled when connected
            ui.add_enabled_ui(!connected, |ui| {
                // Add edit connection button
                if let Some(selected_idx) = selected_connection {
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::action::EDIT,
                            tr(keys::EDIT_CONNECTION, current_lang)
                        ))
                        .clicked()
                    {
                        if let Some(conn) = app.config.connections.get(selected_idx) {
                            app.new_connection.open_for_edit(conn);
                        }
                    }
                }
                // Add new connection button
                if ui
                    .button(format!("+ {}", tr(keys::NEW_CONNECTION, current_lang)))
                    .clicked()
                {
                    app.new_connection.show = true;
                }
            });

            // Right side - Settings button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(emoji::action::SETTINGS).clicked() {
                    app.show_settings = true;
                }
            });
        });
    });

    // Settings window
    if app.show_settings {
        // Check if we're currently editing a shortcut
        let is_editing_shortcut = app.editing_shortcut.is_some();

        // Handle Esc key - cancel editing if in edit mode, otherwise close settings
        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if esc_pressed {
            if is_editing_shortcut {
                // Cancel editing
                app.editing_shortcut = None;
                app.shortcut_input_buffer.clear();
                app.shortcut_conflict_warning = None;
            } else {
                // Close settings window
                app.show_settings = false;
            }
        }

        // Handle close settings shortcut (only when not editing)
        if !is_editing_shortcut {
            let close_binding = app
                .config
                .settings
                .shortcuts
                .get_binding(&ShortcutAction::CloseSettings);
            let should_close = if close_binding == "Esc" {
                // Already handled above
                false
            } else if close_binding.starts_with("Ctrl+") || close_binding.starts_with("Cmd+") {
                let key_part = close_binding.split('+').nth(1);
                if let Some(key_str) = key_part {
                    let key = parse_key_from_str(key_str);
                    ctx.input(|i| {
                        let modifiers_match = if cfg!(target_os = "macos") {
                            i.modifiers.command
                        } else {
                            i.modifiers.ctrl
                        };
                        modifiers_match && key.map_or(false, |k| i.key_pressed(k))
                    })
                } else {
                    false
                }
            } else {
                false
            };

            if should_close {
                app.show_settings = false;
                app.editing_shortcut = None;
                app.shortcut_input_buffer.clear();
                app.shortcut_conflict_warning = None;
            }
        }

        // Position window at top-right corner with some margin
        let screen_rect = ctx
            .input(|i| i.viewport().outer_rect)
            .unwrap_or(egui::Rect::ZERO);
        let top_right = egui::pos2(
            screen_rect.max.x - SETTINGS_WINDOW_WIDTH - 20.0,
            screen_rect.min.y + 40.0,
        );

        egui::Window::new(tr(keys::SETTINGS, current_lang))
            .collapsible(false)
            .resizable(true)
            .default_pos(top_right)
            .min_size([SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT])
            .default_size([SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT])
            .show(ctx, |ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .spacing([40.0, 12.0])
                    .min_col_width(120.0)
                    .show(ui, |ui| {
                        // Language setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::LANGUAGE, current_lang));
                        });
                        let lang = current_lang;
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_salt("settings_lang_select")
                                .selected_text(match lang {
                                    Language::English => ENGLISH,
                                    Language::Chinese => CHINESE,
                                })
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_label(
                                            matches!(lang, Language::English),
                                            ENGLISH,
                                        )
                                        .clicked()
                                    {
                                        app.update_language(Language::English);
                                    }
                                    if ui
                                        .selectable_label(
                                            matches!(lang, Language::Chinese),
                                            CHINESE,
                                        )
                                        .clicked()
                                    {
                                        app.update_language(Language::Chinese);
                                    }
                                });
                        });
                        ui.end_row();

                        // Auto connect setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::AUTO_CONNECT, current_lang));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut auto_connect = app.config.settings.auto_connect;
                            if ui.checkbox(&mut auto_connect, "").changed() {
                                app.config.update_auto_connect(auto_connect);
                            }
                        });
                        ui.end_row();

                        // Show unclosed connections setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::SHOW_UNCLOSED_CONNECTIONS, current_lang));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut show_unclosed = app.config.settings.show_unclosed_connections;
                            if ui.checkbox(&mut show_unclosed, "").changed() {
                                app.config.settings.show_unclosed_connections = show_unclosed;
                                app.config.mark_settings_dirty();
                            }
                        });
                        ui.end_row();

                        // Allow duplicate connections setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::ALLOW_DUPLICATE_CONNECTIONS, current_lang));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut allow_duplicate =
                                app.config.settings.allow_duplicate_connections;
                            if ui.checkbox(&mut allow_duplicate, "").changed() {
                                app.config.settings.allow_duplicate_connections = allow_duplicate;
                                app.config.mark_settings_dirty();
                            }
                        });
                        ui.end_row();

                        // Group keys by colon setting
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(tr(keys::GROUP_KEYS_BY_COLON, current_lang));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut group_keys = app.config.settings.group_keys_by_colon;
                            if ui.checkbox(&mut group_keys, "").changed() {
                                app.config.settings.group_keys_by_colon = group_keys;
                                app.config.mark_settings_dirty();
                            }
                        });
                        ui.end_row();
                    });

                ui.separator();

                // Keyboard shortcuts section (collapsible)
                let shortcuts_header_text = tr(keys::KEYBOARD_SHORTCUTS, current_lang);
                egui::CollapsingHeader::new(shortcuts_header_text)
                    .id_salt("settings_shortcuts_collapsible")
                    .show(ui, |ui| {
                        ui.add_space(8.0);

                        // Check if we're currently capturing a shortcut
                        let is_capturing = app.editing_shortcut.is_some();

                        if is_capturing {
                            ui.colored_label(
                                ui.visuals().warn_fg_color,
                                tr(keys::SHORTCUT_PRESS_KEYS, current_lang),
                            );
                            ui.add_space(8.0);
                        }

                        // Show conflict warning if any
                        if let Some(ref warning) = app.shortcut_conflict_warning {
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
                                    let has_conflict = app.shortcut_conflict_warning.is_some();
                                    let can_save =
                                        !app.shortcut_input_buffer.is_empty() && !has_conflict;
                                    if can_save {
                                        if let Some(current_action_str) = &app.editing_shortcut {
                                            if let Some(current_action) =
                                                parse_action_from_key(current_action_str)
                                            {
                                                app.config.settings.shortcuts.set_binding(
                                                    &current_action,
                                                    app.shortcut_input_buffer.clone(),
                                                );
                                                app.config.mark_settings_dirty();
                                                app.editing_shortcut = None;
                                                app.shortcut_input_buffer.clear();
                                                app.shortcut_conflict_warning = None;
                                            }
                                        }
                                    }
                                }

                                // Check for Escape key to cancel
                                if i.key_pressed(egui::Key::Escape) {
                                    app.editing_shortcut = None;
                                    app.shortcut_input_buffer.clear();
                                    app.shortcut_conflict_warning = None;
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
                                        app.shortcut_input_buffer = parts.join("+");

                                        // Check for conflicts
                                        if let Some(current_action_str) = &app.editing_shortcut {
                                            if let Some(current_action) =
                                                parse_action_from_key(current_action_str)
                                            {
                                                if let Some(conflict_action) =
                                                    app.config.settings.shortcuts.check_conflict(
                                                        &app.shortcut_input_buffer,
                                                        &current_action,
                                                    )
                                                {
                                                    let conflict_name = tr(
                                                        conflict_action.translation_key(),
                                                        current_lang,
                                                    );
                                                    app.shortcut_conflict_warning = Some(tr_fmt(
                                                        keys::SHORTCUT_CONFLICTS_WITH,
                                                        current_lang,
                                                        &[conflict_name],
                                                    ));
                                                } else {
                                                    app.shortcut_conflict_warning = None;
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
                                                app.editing_shortcut.as_ref() == Some(&action_key);

                                            // Check if this shortcut is non-editable
                                            let is_non_editable = action.is_non_editable();

                                            // Action name with bold font (translated)
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(tr(
                                                        trans_key,
                                                        current_lang,
                                                    ))
                                                    .strong(),
                                                );
                                            });

                                            // Current shortcut display or input
                                            if is_editing {
                                                let display_text =
                                                    app.shortcut_input_buffer.clone();
                                                let response = ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut display_text.clone(),
                                                    )
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
                                                let binding = app
                                                    .config
                                                    .settings
                                                    .shortcuts
                                                    .get_binding(&action);
                                                ui.monospace(&binding);
                                            }

                                            // Edit/Save/Cancel buttons or "Not customizable" text
                                            ui.horizontal(|ui| {
                                                if is_non_editable {
                                                    // Show "Not customizable" text for non-editable shortcuts
                                                    ui.label(
                                                        egui::RichText::new(tr(
                                                            keys::SHORTCUT_NON_EDITABLE,
                                                            current_lang,
                                                        ))
                                                        .color(ui.visuals().weak_text_color())
                                                        .italics()
                                                        .size(12.0),
                                                    );
                                                } else if is_editing {
                                                    // Check if there's a conflict before allowing save
                                                    let has_conflict =
                                                        app.shortcut_conflict_warning.is_some();
                                                    let can_save =
                                                        !app.shortcut_input_buffer.is_empty()
                                                            && !has_conflict;

                                                    ui.visuals_mut().override_text_color =
                                                        Some(ui.visuals().selection.bg_fill);
                                                    let save_text = format!(
                                                        "{} {}",
                                                        emoji::action::SAVE,
                                                        tr(keys::SAVE, current_lang)
                                                    );
                                                    if ui
                                                        .add_enabled(
                                                            can_save,
                                                            egui::Button::new(save_text),
                                                        )
                                                        .clicked()
                                                    {
                                                        app.config.settings.shortcuts.set_binding(
                                                            &action,
                                                            app.shortcut_input_buffer.clone(),
                                                        );
                                                        app.config.mark_settings_dirty();
                                                        app.editing_shortcut = None;
                                                        app.shortcut_input_buffer.clear();
                                                        app.shortcut_conflict_warning = None;
                                                    }
                                                    ui.visuals_mut().override_text_color = None;
                                                    let cancel_text = format!(
                                                        "{} {}",
                                                        emoji::action::CANCEL,
                                                        tr(keys::CANCEL, current_lang)
                                                    );
                                                    if ui.button(cancel_text).clicked() {
                                                        app.editing_shortcut = None;
                                                        app.shortcut_input_buffer.clear();
                                                        app.shortcut_conflict_warning = None;
                                                    }
                                                } else {
                                                    let edit_text = format!(
                                                        "{} {}",
                                                        emoji::action::EDIT,
                                                        tr(keys::EDIT, current_lang)
                                                    );
                                                    if ui.button(edit_text).clicked() {
                                                        app.editing_shortcut = Some(action_key);
                                                        // Initialize with current binding
                                                        app.shortcut_input_buffer = app
                                                            .config
                                                            .settings
                                                            .shortcuts
                                                            .get_binding(&action);
                                                        app.shortcut_conflict_warning = None;
                                                    }
                                                }
                                            });
                                            ui.end_row();
                                        }
                                    });
                            });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            let reset_text = format!(
                                "{} {}",
                                emoji::action::REFRESH,
                                tr(keys::SHORTCUT_RESET_DEFAULTS, current_lang)
                            );
                            if ui.button(reset_text).clicked() {
                                app.config.settings.shortcuts.reset_to_default();
                                app.config.mark_settings_dirty();
                                // Clear editing state to refresh the display
                                app.editing_shortcut = None;
                                app.shortcut_input_buffer.clear();
                                app.shortcut_conflict_warning = None;
                            }
                        });
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button(tr(keys::CLOSE, current_lang)).clicked() {
                        app.show_settings = false;
                        app.editing_shortcut = None;
                        app.shortcut_input_buffer.clear();
                    }
                });
            });
    }

    // Handle deferred operations
    if let Some((idx, conn)) = create_new_tab_with {
        app.create_tab_with_connection(idx, conn);
    }

    // Handle auto connect
    if let Some(idx) = auto_connect_idx {
        let active_tab_idx = app.active_tab;
        if let Some(conn) = app.config.connections.get(idx).cloned() {
            let tab = &mut app.tabs[active_tab_idx];
            *tab.state.connection_param.blocking_write() = Some(conn.clone());
            tab.name = conn.name.clone();
            tab.connected_color = conn.color.clone();
            app.load_connection_preferences(active_tab_idx);
            app.spawn_connect_with_initial_db(active_tab_idx);
        }
    }

    // New connection dialog
    if app.new_connection.show {
        app.new_connection
            .render_new_connection_dialog(&mut app.config, ctx, current_lang);
    }
}

/// Parse hex color string (#RRGGBB) to egui Color32
fn parse_color_hex(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(egui::Color32::from_rgb(r, g, b))
}
