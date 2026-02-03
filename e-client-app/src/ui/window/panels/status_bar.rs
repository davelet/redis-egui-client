use crate::ui::window::RedisApp;
use e_client_config::constants::WILD_KEY_FILTER;
use e_client_config::translations::keys;
use e_client_config::translations::tr;

pub fn render_status_bar(app: &mut RedisApp, ctx: &egui::Context) {
    // Early return if no active tab
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let connected = app.poll_bool(tab.state.connected.clone());
    let loading = app.poll_bool(tab.state.loading.clone());
    let total_keys = app.poll_usize(tab.state.total_keys.clone());
    let loaded_keys = app.poll_vec_string(tab.state.keys.clone()).len();
    let scan_has_more = app.poll_bool(tab.state.scan_has_more.clone());
    let error_message = app.poll_string(tab.state.error_message.clone());

    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Connection status
            ui.label(tr(keys::STATUS_BAR, current_lang));
            ui.separator();

            if !connected {
                ui.label(tr(keys::DISCONNECTED, current_lang));
            } else if loading {
                ui.label(tr(keys::LOADING, current_lang));
                ui.spinner();
            } else {
                ui.label(tr(keys::READY, current_lang));
            }

            ui.separator();

            if connected {
                // Total keys - only show exact count if it's a full scan OR scan is complete
                ui.label(tr(keys::TOTAL_KEYS, current_lang));
                let key_filter = app.poll_string(tab.state.key_filter.clone());
                let key_filter = key_filter.trim();
                let is_full_scan =
                    key_filter == String::new() || key_filter == WILD_KEY_FILTER.to_string();
                if is_full_scan || !scan_has_more {
                    ui.label(format!("{}", total_keys));
                } else {
                    ui.label(tr(keys::UNKNOWN, current_lang));
                }
                ui.separator();

                // Loaded keys
                ui.label(tr(keys::LOADED_KEYS, current_lang));
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
        });
    });
}
