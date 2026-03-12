use crate::ui::window::RedisApp;
use e_client_basics::constants::ACTIVE_TAB_BACKGROUND_COLOR;
use e_client_config::language::Language;
use e_client_config::translations::emoji;
use e_client_config::translations::keys;
use e_client_config::translations::tr;
use std::collections::HashSet;

pub fn render_tab_bar(app: &mut RedisApp, ctx: &egui::Context) {
    let mut tab_to_close: Option<usize> = None;
    let mut close_other_tabs: Option<usize> = None;
    let mut switch_to_tab: Option<usize> = None;
    let mut duplicate_tab: Option<usize> = None;
    let mut remove_duplicate_and_invalid: bool = false;

    // Get the scroll_to_tab flag
    let scroll_to_tab = app.scroll_to_tab.take();

    // Get and consume the show_all_tabs_dropdown flag
    let show_dropdown_via_shortcut = app.show_all_tabs_dropdown;
    if show_dropdown_via_shortcut {
        app.show_all_tabs_dropdown = false;
    }

    // Get current language
    let current_lang = app
        .tabs
        .get(app.active_tab)
        .map(|tab| *tab.state.language.blocking_read());
    let lang = current_lang.unwrap_or(Language::English);

    egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            // Collect tab info first to avoid borrow issues
            let tab_infos: Vec<_> = app
                .tabs
                .iter()
                .enumerate()
                .map(|(idx, tab)| {
                    let is_active = idx == app.active_tab;

                    // Get connection color from tab (only set after successful connection)
                    let color = tab
                        .connected_color
                        .as_ref()
                        .and_then(|hex| parse_color_hex(hex));

                    // Check if tab has a connection
                    let has_connection = tab.selected_connection.is_some();

                    (idx, is_active, tab.name.clone(), color, has_connection)
                })
                .collect();

            // Available width for scroll area (reserve space for dropdown button)
            let dropdown_button_width = 30.0;
            let available_width = ui.available_width() - dropdown_button_width;

            // Scrollable area for tabs
            egui::ScrollArea::horizontal()
                .id_salt("tab_scroll_area")
                .max_width(available_width)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Render tabs
                        for (idx, is_active, tab_text, color, has_connection) in &tab_infos {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    // Show color indicator before the button
                                    if let Some(color) = color {
                                        ui.colored_label(*color, "●");
                                    }

                                    let button = if *is_active {
                                        egui::Button::new(tab_text).fill(egui::Color32::from_rgb(
                                            ACTIVE_TAB_BACKGROUND_COLOR[0],
                                            ACTIVE_TAB_BACKGROUND_COLOR[1],
                                            ACTIVE_TAB_BACKGROUND_COLOR[2],
                                        ))
                                    } else {
                                        egui::Button::new(tab_text)
                                    };

                                    if ui.add(button).clicked() {
                                        switch_to_tab = Some(*idx);
                                    }

                                    // Close button - always show
                                    if ui.small_button("×").clicked() {
                                        tab_to_close = Some(*idx);
                                    }

                                    // Menu button (three dots) - show if has connection OR more than 2 tabs
                                    if *has_connection || app.tabs.len() > 2 {
                                        ui.menu_button("⋮", |ui| {
                                            // Copy button - show if has connection
                                            if *has_connection {
                                                if ui.button(tr(keys::DUPLICATE, lang)).clicked() {
                                                    duplicate_tab = Some(*idx);
                                                }
                                            }

                                            // Close others button - show if more than 2 tabs
                                            if app.tabs.len() > 2 {
                                                ui.separator();
                                                if ui.button(tr(keys::CLOSE_OTHERS, lang)).clicked()
                                                {
                                                    close_other_tabs = Some(*idx);
                                                }
                                            }
                                        });
                                    }
                                });
                            });

                            // Scroll to this tab if needed
                            if let Some(target_idx) = scroll_to_tab {
                                if *idx == target_idx {
                                    ui.scroll_to_cursor(Some(egui::Align::Center));
                                }
                            }
                        }

                        // New tab button
                        if ui.button("+").clicked() {
                            app.create_new_tab();
                        }
                    });
                });

            // Dropdown menu for all tabs - show when there are more than 2 tabs
            if tab_infos.len() > 2 {
                // Check for duplicate and invalid tabs
                let mut seen_connections: HashSet<Option<usize>> = HashSet::new();
                let mut has_duplicates = false;
                let mut has_invalid = false;

                for (idx, _, _, _, has_connection) in &tab_infos {
                    if !has_connection {
                        has_invalid = true;
                    } else {
                        // Check for duplicate connections
                        let tab = &app.tabs[*idx];
                        let conn_idx = tab.selected_connection;
                        if !seen_connections.insert(conn_idx) {
                            has_duplicates = true;
                        }
                    }
                }

                // Create a menu button that can be opened via shortcut
                if show_dropdown_via_shortcut {
                    // Force the menu to open when triggered by shortcut
                    let dropdown_id = egui::Id::new("all_tabs_dropdown");
                    ctx.memory_mut(|mem| mem.open_popup(dropdown_id));
                }

                ui.menu_button("▼", |ui| {
                    ui.set_min_width(200.0);

                    // Title and remove button in horizontal layout
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(tr(keys::ALL_TABS, lang)).strong());

                        // Show remove duplicate button if needed
                        if has_duplicates || has_invalid {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .small_button(emoji::action::DELETE)
                                        .on_hover_text(tr(
                                            keys::REMOVE_DUPLICATE_AND_INVALID_TABS,
                                            lang,
                                        ))
                                        .clicked()
                                    {
                                        remove_duplicate_and_invalid = true;
                                        ui.close();
                                    }
                                },
                            );
                        }
                    });

                    ui.separator();
                    for (idx, is_active, tab_text, color, _) in &tab_infos {
                        let mut label_text = tab_text.clone();
                        if *is_active {
                            label_text = format!("● {}", tab_text);
                        }
                        let response = ui.horizontal(|ui| {
                            if let Some(color) = color {
                                ui.colored_label(*color, "●");
                            }
                            if ui.selectable_label(*is_active, &label_text).clicked() {
                                switch_to_tab = Some(*idx);
                                ui.close();
                            }
                        });
                        // Make the entire row clickable
                        if response.response.clicked() {
                            switch_to_tab = Some(*idx);
                            ui.close();
                        }
                    }
                });
            }
        });
    });

    // Handle tab operations after the UI
    if let Some(idx) = switch_to_tab {
        app.switch_to_tab(idx);
    }
    if let Some(idx) = tab_to_close {
        app.close_tab(idx, ctx);
    }
    if let Some(idx) = close_other_tabs {
        app.close_other_tabs(idx);
    }
    if let Some(idx) = duplicate_tab {
        // Duplicate tab by creating a new tab with the same connection
        if let Some(conn_idx) = app.tabs.get(idx).and_then(|t| t.selected_connection) {
            if let Some(conn) = app.config.connections.get(conn_idx).cloned() {
                app.create_tab_with_connection(conn_idx, conn);
            }
        }
    }

    // Handle remove duplicate and invalid tabs
    if remove_duplicate_and_invalid {
        remove_duplicate_and_invalid_tabs(app, ctx);
    }
}

/// Remove duplicate and invalid tabs, keeping only the first occurrence of each connection
fn remove_duplicate_and_invalid_tabs(app: &mut RedisApp, ctx: &egui::Context) {
    if app.tabs.len() <= 1 {
        return;
    }

    let mut seen_connections: HashSet<Option<usize>> = HashSet::new();
    let mut indices_to_remove: Vec<usize> = Vec::new();

    // First pass: identify tabs to remove
    for (idx, tab) in app.tabs.iter().enumerate() {
        if tab.selected_connection.is_none() {
            // Invalid tab (no connection)
            indices_to_remove.push(idx);
        } else {
            // Check for duplicates
            let conn_idx = tab.selected_connection;
            if !seen_connections.insert(conn_idx) {
                // Duplicate connection, mark for removal
                indices_to_remove.push(idx);
            }
        }
    }

    // Remove in reverse order to avoid index shifting
    for idx in indices_to_remove.into_iter().rev() {
        if idx < app.tabs.len() {
            app.close_tab(idx, ctx);
        }
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
