use crate::ui::window::RedisApp;
use e_client_config::constants::WILD_KEY_FILTER;
use e_client_config::translations::{TranslationKey, tr};

pub fn render_status_bar(app: &mut RedisApp, ctx: &egui::Context) {
    // Early return if no active tab
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let current_lang = app.poll_language(app.tabs[active_tab_idx].state.language.clone());
    let connected = app.poll_bool(app.tabs[active_tab_idx].state.connected.clone());
    let loading = app.poll_bool(app.tabs[active_tab_idx].state.loading.clone());
    let total_keys = app.poll_usize(app.tabs[active_tab_idx].state.total_keys.clone());
    let loaded_keys = app.tabs[active_tab_idx]
        .state
        .keys
        .try_read()
        .map(|k| k.len())
        .unwrap_or(0);
    let scan_has_more = app.poll_bool(app.tabs[active_tab_idx].state.scan_has_more.clone());
    let error_message = app.poll_string(app.tabs[active_tab_idx].state.error_message.clone());

    // Detect loading transitions for toast logic:
    //   false → true: new connection attempt → clear error dedup, cancel pending
    //   true  → false: loading just finished → set pending_error_check
    // pending_error_check persists across frames until we see the result (error or success),
    // because loading and error_message may update in different frames.
    let loading_just_started = loading && !app.tabs[active_tab_idx].was_loading;
    let loading_just_finished = !loading && app.tabs[active_tab_idx].was_loading;

    if loading_just_started {
        app.tabs[active_tab_idx].last_error_shown = None;
        app.tabs[active_tab_idx].pending_error_check = false;
    }
    if loading_just_finished {
        app.tabs[active_tab_idx].pending_error_check = true;
    }
    app.tabs[active_tab_idx].was_loading = loading;

    // Show toast when: pending_error_check is set (loading recently finished)
    // AND error_message is present. This handles the case where the error
    // arrives one frame after loading drops to false.
    if !error_message.is_empty() && app.tabs[active_tab_idx].pending_error_check {
        // Get owner before borrowing tab mutably
        let selected_connection = app.tabs[active_tab_idx].selected_connection;
        let tab_name = app.tabs[active_tab_idx].name.clone();
        let owner = if let Some(conn_idx) = selected_connection {
            app.config()
                .connections
                .get(conn_idx)
                .map(|c| c.name.clone())
                .unwrap_or(tab_name)
        } else {
            tab_name
        };

        // Dedup key includes owner + message, so different connections
        // with the same error (e.g. "Connection refused") each get a toast.
        let dedup_key = format!("{}|{}", owner, error_message);

        let tab = &mut app.tabs[active_tab_idx];
        let should_show = match &tab.last_error_shown {
            None => true,
            Some(last) => last != &dedup_key,
        };

        if should_show {
            let toast_message = if error_message.len() > 200 {
                format!("{}...", &error_message[..200])
            } else {
                error_message.clone()
            };

            app.toasts.warning(owner, toast_message);
            tab.last_error_shown = Some(dedup_key);
        }
        // Connection attempt resolved with error — clear pending
        tab.pending_error_check = false;
    } else if error_message.is_empty() && app.tabs[active_tab_idx].pending_error_check {
        // No error but pending check — connection succeeded, clear pending
        app.tabs[active_tab_idx].pending_error_check = false;
    }

    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Connection status
            ui.label(tr(TranslationKey::StatusBar, current_lang));
            ui.separator();

            if !connected {
                ui.label(tr(TranslationKey::Disconnected, current_lang));
            } else if loading {
                ui.label(tr(TranslationKey::Loading, current_lang));
                ui.spinner();
            } else {
                ui.label(tr(TranslationKey::Ready, current_lang));
            }

            ui.separator();

            if connected {
                // Total keys - only show exact count if it's a full scan OR scan is complete
                ui.label(tr(TranslationKey::TotalKeys, current_lang));
                let key_filter = app.poll_string(app.tabs[active_tab_idx].state.key_filter.clone());
                let key_filter = key_filter.trim();
                let is_full_scan =
                    key_filter == String::new() || key_filter == WILD_KEY_FILTER.to_string();
                if is_full_scan || !scan_has_more {
                    ui.label(format!("{}", total_keys));
                } else {
                    ui.label(tr(TranslationKey::Unknown, current_lang));
                }
                ui.separator();

                // Loaded keys
                ui.label(tr(TranslationKey::LoadedKeys, current_lang));
                if scan_has_more {
                    ui.label(
                        egui::RichText::new(format!("{}", loaded_keys)).color(egui::Color32::RED),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(format!("{}", loaded_keys)).color(egui::Color32::GREEN),
                    );
                }
            }

            // Error message
            if !error_message.is_empty() && error_message.lines().count() <= 2 {
                ui.separator();
                ui.label(egui::RichText::new(&error_message).color(egui::Color32::RED));
            }

            // Push remaining space to the left and command line button to the right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Command line toggle button - only show when connected
                if connected {
                    let show = app.tabs[active_tab_idx].command_line_panel.show;
                    let button_text = "CLI";
                    let button_color = if show {
                        egui::Color32::from_rgb(50, 200, 50) // Green when opened
                    } else {
                        egui::Color32::BLACK // Black when closed
                    };
                    let button =
                        egui::Button::new(egui::RichText::new(button_text).color(button_color));
                    if ui.add(button).clicked() {
                        app.tabs[active_tab_idx].command_line_panel.show = !show;
                        if app.tabs[active_tab_idx].command_line_panel.show {
                            app.tabs[active_tab_idx].command_line_panel.scroll_to_bottom = true;
                        }
                    }
                }
            });
        });
    });
}
