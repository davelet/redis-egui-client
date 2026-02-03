use crate::ui::window::RedisApp;
use e_client_config::constants::WILD_KEY_FILTER;
use e_client_config::translations::keys;
use e_client_config::translations::tr;

pub fn render_side_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let keys = app.poll_vec_string(tab.state.keys.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());
    let loading = app.poll_bool(tab.state.loading.clone());
    let scan_has_more = app.poll_bool(tab.state.scan_has_more.clone());
    let total_keys = app.poll_usize(tab.state.total_keys.clone());
    let connected = app.poll_bool(tab.state.connected.clone());
    let loading_progress_text = app.poll_string(tab.state.loading_progress_text.clone());
    let key_filter = app.poll_string(tab.state.key_filter.clone());

    egui::SidePanel::left("side_panel")
        .min_width(250.0)
        .exact_width(300.0)
        .show(ctx, |ui| {
            // Heading with loaded/total key count
            let is_full_scan = key_filter == "*" || key_filter == WILD_KEY_FILTER.to_string();
            let total_display = if is_full_scan || !scan_has_more {
                total_keys.to_string()
            } else {
                tr(keys::UNKNOWN, current_lang).to_string()
            };
            let heading_text = format!(
                "{} ({}/{})",
                tr(keys::KEYS, current_lang),
                keys.len(),
                total_display
            );
            ui.heading(heading_text);

            ui.horizontal(|ui| {
                ui.label(tr(keys::FILTER, current_lang));
                let tab = &mut app.tabs[active_tab_idx];
                let changed = ui.text_edit_singleline(&mut tab.key_filter_input).changed();
                if changed {
                    let key_filter = tab.state.key_filter.clone();
                    let input = tab.key_filter_input.clone();
                    let input = input.trim();

                    // Process filter: if empty, use "*"; if contains *, use as-is; otherwise add * on both sides
                    let processed_filter = if input.is_empty() {
                        WILD_KEY_FILTER.to_string()
                    } else if input.contains('*') {
                        input.to_string()
                    } else {
                        format!("{}{}{}", WILD_KEY_FILTER, input, WILD_KEY_FILTER)
                    };

                    // Release mutable borrow
                    app.update_string(key_filter, processed_filter);
                    app.tabs[active_tab_idx].state.spawn_load_keys();
                }
            });

            // Show loading progress
            if loading && !loading_progress_text.is_empty() {
                ui.label(egui::RichText::new(&loading_progress_text).weak());
            }

            // Show "load more" button below filter (only if connected)
            if connected && scan_has_more && !loading {
                ui.horizontal(|ui| {
                    if ui.button(tr(keys::LOAD_MORE_KEYS, current_lang)).clicked() {
                        app.tabs[active_tab_idx].state.spawn_load_more_keys(false);
                    }
                    if ui.button(tr(keys::LOAD_ALL_KEYS, current_lang)).clicked() {
                        app.tabs[active_tab_idx].state.spawn_load_more_keys(true);
                    }
                });
            }

            ui.separator();

            // Fill remaining space with scroll area - optimized with limit for large key lists
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .id_salt("keys_scroll")
                .show(ui, |ui| {
                    // Limit rendering to improve performance with large datasets
                    // Only show first 5000 keys or all keys if less than that
                    let keys_to_show = if keys.len() > 5000 {
                        &keys[0..5000]
                    } else {
                        &keys[..]
                    };

                    for key in keys_to_show {
                        let is_selected = selected_key.as_ref() == Some(key);
                        if ui.selectable_label(is_selected, key).clicked() {
                            let tab = &mut app.tabs[active_tab_idx];
                            *tab.state.selected_key.blocking_write() = Some(key.clone());
                            tab.state.spawn_load_value(key.clone());
                        }
                    }

                    // Show message if there are more keys
                    if keys.len() > 5000 {
                        ui.separator();
                        ui.label(
                            egui::RichText::new(format!(
                                "Showing 5000 of {} keys (use filter to narrow down)",
                                keys.len()
                            ))
                            .weak(),
                        );
                    }
                });
        });
}
