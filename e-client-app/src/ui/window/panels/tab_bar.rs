use crate::ui::window::RedisApp;
use e_client_basics::constants::ACTIVE_TAB_BACKGROUND_COLOR;
use e_client_config::language::Language;
use e_client_config::translations::keys;
use e_client_config::translations::tr;

pub fn render_tab_bar(app: &mut RedisApp, ctx: &egui::Context) {
    let mut tab_to_close: Option<usize> = None;
    let mut close_other_tabs: Option<usize> = None;
    let mut switch_to_tab: Option<usize> = None;
    let mut duplicate_tab: Option<usize> = None;

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

            // Scrollable area for tabs
            egui::ScrollArea::horizontal()
                .id_salt("tab_scroll_area")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Render tabs
                        for (idx, is_active, tab_text, color, has_connection) in tab_infos {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    // Show color indicator before the button
                                    if let Some(color) = color {
                                        ui.colored_label(color, "●");
                                    }

                                    let button = if is_active {
                                        egui::Button::new(&tab_text).fill(egui::Color32::from_rgb(
                                            ACTIVE_TAB_BACKGROUND_COLOR[0],
                                            ACTIVE_TAB_BACKGROUND_COLOR[1],
                                            ACTIVE_TAB_BACKGROUND_COLOR[2],
                                        ))
                                    } else {
                                        egui::Button::new(&tab_text)
                                    };

                                    if ui.add(button).clicked() {
                                        switch_to_tab = Some(idx);
                                    }

                                    // Close button - always show
                                    if ui.small_button("×").clicked() {
                                        tab_to_close = Some(idx);
                                    }

                                    // Menu button (three dots) - show if has connection OR more than 2 tabs
                                    if has_connection || app.tabs.len() > 2 {
                                        ui.menu_button("⋮", |ui| {
                                            let current_lang = app
                                                .tabs
                                                .get(app.active_tab)
                                                .map(|tab| *tab.state.language.blocking_read());
                                            let lang = current_lang.unwrap_or(Language::English);

                                            // Copy button - show if has connection
                                            if has_connection {
                                                if ui.button(tr(keys::DUPLICATE, lang)).clicked() {
                                                    duplicate_tab = Some(idx);
                                                }
                                            }

                                            // Close others button - show if more than 2 tabs
                                            if app.tabs.len() > 2 {
                                                ui.separator();
                                                if ui.button(tr(keys::CLOSE_OTHERS, lang)).clicked()
                                                {
                                                    close_other_tabs = Some(idx);
                                                }
                                            }
                                        });
                                    }
                                });
                            });
                        }

                        // New tab button
                        if ui.button("+").clicked() {
                            app.create_new_tab();
                        }
                    });
                });
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
