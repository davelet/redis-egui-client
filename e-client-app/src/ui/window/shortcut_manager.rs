use crate::ui::window::RedisApp;
use e_client_config::config::shortcuts::ShortcutAction;

pub struct ShortcutManagerState {
    pub editing_shortcut: Option<String>,
    pub shortcut_input_buffer: String,
    pub shortcut_conflict_warning: Option<String>,
}

impl Default for ShortcutManagerState {
    fn default() -> Self {
        Self {
            editing_shortcut: None,
            shortcut_input_buffer: String::new(),
            shortcut_conflict_warning: None,
        }
    }
}

pub fn handle_shortcuts(app: &mut RedisApp, ctx: &egui::Context) {
    use e_client_config::config::shortcuts::ParsedShortcut;

    // F1 toggles help (works even when settings is open)
    let f1_pressed = ctx.input(|i| i.key_pressed(egui::Key::F1));
    if f1_pressed {
        app.set_show_help(!app.show_help());
        ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F1));
    }

    // Handle new connection window shortcuts (work even when new connection window is open)
    if app.new_connection.show {
        use crate::ui::window::panels::settings_panel::SUPPORTED_KEYS;
        use e_client_config::config::shortcuts::ParsedShortcut;

        let (modifiers, pressed_keys): (egui::Modifiers, Vec<egui::Key>) = ctx.input(|i| {
            let keys: Vec<egui::Key> = SUPPORTED_KEYS
                .into_iter()
                .filter(|k| i.key_pressed(*k))
                .collect();
            (i.modifiers, keys)
        });

        let is_macos = cfg!(target_os = "macos");

        // Check for Confirm New Connection shortcut
        let confirm_binding = app.config().settings.shortcuts.get_binding(&ShortcutAction::ConfirmNewConnection);
        if let Some(parsed) = ParsedShortcut::parse(&confirm_binding) {
            for key in &pressed_keys {
                let key_str = format!("{:?}", key);
                let mod_pressed = parsed.is_mod_pressed(is_macos, modifiers.ctrl, modifiers.command);
                let alt_match = parsed.alt == modifiers.alt;
                let shift_match = parsed.shift == modifiers.shift;
                let key_match = parsed.key_matches(&key_str);

                let meets_mod_requirement = if parsed.command || parsed.ctrl {
                    mod_pressed
                } else {
                    !modifiers.command && !modifiers.ctrl
                };

                if meets_mod_requirement && alt_match && shift_match && key_match {
                    let current_lang = if !app.config.settings.language.is_empty() {
                        e_client_config::language::Language::file_name_to_lang(&app.config.settings.language)
                    } else {
                        e_client_config::language::Language::English
                    };
                    app.new_connection.try_save(&mut app.config, current_lang);
                    ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, *key));
                    return;
                }
            }
        }

        // Check for Cancel New Connection shortcut (default: Esc)
        let cancel_binding = app.config().settings.shortcuts.get_binding(&ShortcutAction::CancelNewConnection);
        if let Some(parsed) = ParsedShortcut::parse(&cancel_binding) {
            for key in &pressed_keys {
                let key_str = format!("{:?}", key);
                let mod_pressed = parsed.is_mod_pressed(is_macos, modifiers.ctrl, modifiers.command);
                let alt_match = parsed.alt == modifiers.alt;
                let shift_match = parsed.shift == modifiers.shift;
                let key_match = parsed.key_matches(&key_str);

                let meets_mod_requirement = if parsed.command || parsed.ctrl {
                    mod_pressed
                } else {
                    !modifiers.command && !modifiers.ctrl
                };

                if meets_mod_requirement && alt_match && shift_match && key_match {
                    app.new_connection.clear();
                    ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, *key));
                    return;
                }
            }
        }
    }

    if !app.show_settings() && !app.new_connection.show {
        use crate::ui::window::panels::settings_panel::SUPPORTED_KEYS;

        let (modifiers, pressed_keys): (egui::Modifiers, Vec<egui::Key>) = ctx.input(|i| {
            let keys: Vec<egui::Key> = SUPPORTED_KEYS
                .into_iter()
                .filter(|k| i.key_pressed(*k))
                .collect();
            (i.modifiers, keys)
        });

        let text_input_focused = ctx.wants_keyboard_input();
        let has_function_key = pressed_keys.iter().any(|k| {
            matches!(
                k,
                egui::Key::F1
                    | egui::Key::F2
                    | egui::Key::F3
                    | egui::Key::F4
                    | egui::Key::F5
                    | egui::Key::F6
                    | egui::Key::F7
                    | egui::Key::F8
                    | egui::Key::F9
                    | egui::Key::F10
                    | egui::Key::F11
                    | egui::Key::F12
            )
        });
        let esc_pressed = pressed_keys.contains(&egui::Key::Escape);

        let should_process_shortcuts = !text_input_focused || esc_pressed || has_function_key;

        if should_process_shortcuts {
            let is_macos = cfg!(target_os = "macos");
            let mut action_to_execute = None;

            for (action, _) in ShortcutAction::all_actions() {
                let binding = app.config().settings.shortcuts.get_binding(&action);
                if let Some(parsed) = ParsedShortcut::parse(&binding) {
                    for key in &pressed_keys {
                        let key_str = format!("{:?}", key);
                        let mod_pressed: bool =
                            parsed.is_mod_pressed(is_macos, modifiers.ctrl, modifiers.command);
                        let alt_match = parsed.alt == modifiers.alt;
                        let shift_match = parsed.shift == modifiers.shift;
                        let key_match = parsed.key_matches(&key_str);

                        let meets_mod_requirement = if parsed.command || parsed.ctrl {
                            mod_pressed
                        } else {
                            !modifiers.command && !modifiers.ctrl
                        };

                        if meets_mod_requirement && alt_match && shift_match && key_match {
                            action_to_execute = Some((action.clone(), *key));
                        }
                    }
                }
            }

            if let Some((act, key)) = action_to_execute {
                handle_shortcut_action(app, act, ctx);
                ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, key));
            }
        }
    }
}

pub fn handle_shortcut_action(app: &mut RedisApp, action: ShortcutAction, ctx: &egui::Context) {
    match action {
        ShortcutAction::NewConnection => {
            if let Some(tab) = app.get_active_tab() {
                let connected = app.poll_bool(tab.state.connected.clone());
                if !connected {
                    app.new_connection.show = true;
                }
            } else {
                app.new_connection.show = true;
            }
        }
        ShortcutAction::ConnectAllUnclosed => {
            let connections: Vec<_> = app
                .config()
                .connections
                .connections
                .iter()
                .cloned()
                .collect();
            let open_conn_names: Vec<_> = app
                .config()
                .window
                .open_connections
                .connection_names
                .clone();

            if !open_conn_names.is_empty() {
                for (i, conn_name) in open_conn_names.iter().enumerate() {
                    if let Some(conn_idx) = connections.iter().position(|c| &c.name == conn_name) {
                        let conn = connections[conn_idx].clone();
                        if i == 0 && !app.config().settings.open_connections_in_new_tab {
                            app.connect_in_current_tab(conn_idx, &conn);
                        } else {
                            app.create_tab_with_connection(conn_idx, conn);
                        }
                    }
                }
                app.config_mut().clear_open_connections();
                app.show_open_connections_prompt = false;
            }
        }
        ShortcutAction::NewTab => app.create_new_tab(),
        ShortcutAction::CloseTab => {
            let idx = app.active_tab();
            app.close_tab(idx, ctx);
        }
        ShortcutAction::RefreshKey => {
            if let Some(tab) = app.tabs().get(app.active_tab()) {
                if let Some(key) = tab.state.selected_key.blocking_read().clone() {
                    tab.state.spawn_load_value(
                        key,
                        e_client_core::app_state::operations::keys::HashLoadMode::ReloadFields,
                    );
                }
            }
        }
        ShortcutAction::FocusFilter => {
            ctx.memory_mut(|mem| {
                mem.request_focus(egui::Id::new("key_filter_input"));
            });
        }
        ShortcutAction::CloseSettings => {}
        ShortcutAction::OpenSettings => {
            app.set_show_settings(true);
        }
        ShortcutAction::ToggleCommandLine => {
            let active_idx = app.active_tab();
            if let Some(tab) = app.tabs_mut().get_mut(active_idx) {
                if tab.command_line_panel.show {
                    ctx.memory_mut(|mem| {
                        mem.request_focus(egui::Id::new("command_line_input"));
                    });
                } else {
                    tab.command_line_panel.show = true;
                    tab.command_line_panel.scroll_to_bottom = true;
                }
            }
        }
        ShortcutAction::CloseCommandLine => {
            let active_idx = app.active_tab();
            if let Some(tab) = app.tabs_mut().get_mut(active_idx) {
                if tab.command_line_panel.pending_ai_command.is_none() {
                    tab.command_line_panel.show = false;
                }
            }
        }
        ShortcutAction::SwitchToTab1
        | ShortcutAction::SwitchToTab2
        | ShortcutAction::SwitchToTab3
        | ShortcutAction::SwitchToTab4
        | ShortcutAction::SwitchToTab5
        | ShortcutAction::SwitchToTab6
        | ShortcutAction::SwitchToTab7
        | ShortcutAction::SwitchToTab8
        | ShortcutAction::SwitchToTab9 => {
            if let Some(tab_idx) = action.tab_index() {
                app.switch_to_tab(tab_idx);
            }
        }
        ShortcutAction::ConnectConnection1
        | ShortcutAction::ConnectConnection2
        | ShortcutAction::ConnectConnection3
        | ShortcutAction::ConnectConnection4
        | ShortcutAction::ConnectConnection5
        | ShortcutAction::ConnectConnection6
        | ShortcutAction::ConnectConnection7
        | ShortcutAction::ConnectConnection8
        | ShortcutAction::ConnectConnection9 => {
            if let Some(tab) = app.get_active_tab() {
                let connected = app.poll_bool(tab.state.connected.clone());
                if !connected {
                    if let Some(conn_idx) = action.connection_index() {
                        let config = app.config();
                        if conn_idx < config.connections.connections.len() {
                            let conn = config.connections.connections[conn_idx].clone();
                            let open_in_new_tab = config.settings.open_connections_in_new_tab;
                            if open_in_new_tab {
                                app.create_tab_with_connection(conn_idx, conn);
                            } else {
                                app.connect_in_current_tab(conn_idx, &conn);
                            }
                        }
                    }
                }
            }
        }
        ShortcutAction::SwitchToLastTab => {
            let len = app.tabs().len();
            if len > 0 {
                app.switch_to_tab(len - 1);
            }
        }
        ShortcutAction::RemoveDuplicateAndInvalidTabs => {
            app.remove_duplicate_and_invalid_tabs(ctx);
        }
        ShortcutAction::RefreshKeys => {
            let active_idx = app.active_tab();
            if let Some(tab) = app.tabs_mut().get_mut(active_idx) {
                tab.state.spawn_load_keys();
            }
        }
        ShortcutAction::ExecuteAiCommand
        | ShortcutAction::CancelAiCommand
        | ShortcutAction::CloseAiModelEditor => {}
        ShortcutAction::ToggleHelp => {
            // Handled at the top level to work even when settings is open
        }
        ShortcutAction::ToggleLiveLogs => {
            let active_idx = app.active_tab();
            if let Some(tab) = app.tabs_mut().get_mut(active_idx) {
                tab.command_line_panel.log_viewer.enabled =
                    !tab.command_line_panel.log_viewer.enabled;
            }
        }
        ShortcutAction::ConfirmNewConnection => {
            // Handled at the top level when new connection window is open
        }
        ShortcutAction::CancelNewConnection => {
            // Handled at the top level when new connection window is open
        }
    }
}
