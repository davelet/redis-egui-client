use crate::ui::window::RedisApp;
use crate::ui::window::shortcut_manager::get_shortcut_display;
use e_client_basics::constants::{LOAD_MORE_BATCH_SIZE, MAX_LOADED_KEYS};
use e_client_basics::emoji;
use e_client_config::config::shortcuts::ShortcutAction;
use e_client_config::constants::WILD_KEY_FILTER;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};
use std::collections::BTreeMap;

/// Represents a node in the key tree structure
#[derive(Debug, Default)]
struct KeyTreeNode {
    /// Full key name if this node represents a complete key
    full_key: Option<String>,
    /// Child nodes keyed by their segment name
    children: BTreeMap<String, KeyTreeNode>,
}

impl KeyTreeNode {
    fn new() -> Self {
        Self::default()
    }

    /// Insert a key into the tree
    fn insert(&mut self, key: &str) {
        let parts: Vec<&str> = key.split(':').collect();
        self.insert_parts(&parts, key);
    }

    fn insert_parts(&mut self, parts: &[&str], full_key: &str) {
        if parts.is_empty() {
            self.full_key = Some(full_key.to_string());
            return;
        }

        let first = parts[0].to_string();
        let child = self.children.entry(first).or_default();
        child.insert_parts(&parts[1..], full_key);
    }
}

pub fn render_side_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let (
        current_lang,
        connected,
        keys_arc,
        selected_key,
        loading,
        scan_has_more,
        total_keys,
        loading_progress_text,
        key_filter,
        side_panel_width,
    ) = {
        let tab = &app.tabs[active_tab_idx];
        (
            app.poll_language(tab.state.language.clone()),
            app.poll_bool(tab.state.connected.clone()),
            tab.state.keys.clone(),
            app.poll_option_string(tab.state.selected_key.clone()),
            app.poll_bool(tab.state.loading.clone()),
            app.poll_bool(tab.state.scan_has_more.clone()),
            app.poll_usize(tab.state.total_keys.clone()),
            app.poll_string(tab.state.loading_progress_text.clone()),
            app.poll_string(tab.state.key_filter.clone()),
            tab.side_panel_width,
        )
    };

    if !connected {
        return;
    }

    let keys_guard = keys_arc.try_read();
    let keys = match &keys_guard {
        Ok(guard) => &**guard,
        Err(_) => &[] as &[String],
    };

    // Use a unique panel ID for each tab to avoid sharing state
    let panel_id = egui::Id::new(("side_panel", active_tab_idx));

    egui::SidePanel::left(panel_id)
        .min_width(250.0)
        .max_width(800.0)
        .default_width(side_panel_width)
        .resizable(true)
        .show(ctx, |ui| {
            // Save the current width if it changed
            let current_width = ui.available_width();
            if (current_width - side_panel_width).abs() > 1.0 {
                app.update_tab_side_panel_width(active_tab_idx, current_width);
            }

            // Heading with loaded/total key count
            let is_full_scan = key_filter.is_empty() || key_filter == WILD_KEY_FILTER.to_string();
            let total_display = if is_full_scan || !scan_has_more {
                total_keys.to_string()
            } else {
                tr(TranslationKey::Unknown, current_lang).to_string()
            };
            let heading_text = format!(
                "{} ({}/{})",
                tr(TranslationKey::Keys, current_lang),
                keys.len(),
                total_display
            );
            ui.horizontal(|ui| {
                ui.heading(heading_text);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let new_key_binding = app
                        .config
                        .settings
                        .shortcuts
                        .get_binding(&ShortcutAction::NewKey);
                    if ui
                        .button(format!(
                            "{}{}",
                            emoji::action::ADD,
                            get_shortcut_display(&new_key_binding)
                        ))
                        .clicked()
                    {
                        app.new_key_dialog.reset();
                        app.new_key_dialog.show = true;
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label(tr(TranslationKey::Filter, current_lang));
                let refresh_keys_binding = app
                    .config
                    .settings
                    .shortcuts
                    .get_binding(&ShortcutAction::RefreshKeys);
                let refresh_btn = ui.button(format!(
                    "{}{}",
                    emoji::action::REFRESH,
                    get_shortcut_display(&refresh_keys_binding)
                ));
                // Use remaining width for filter input
                let tab = &mut app.tabs[active_tab_idx];
                let changed = ui
                    .add_sized(
                        egui::vec2(ui.available_width(), 20.0),
                        egui::TextEdit::singleline(&mut tab.key_filter_input)
                            .id(egui::Id::new("key_filter_input")),
                    )
                    .changed();
                if changed {
                    // Don't trigger the SCAN immediately - just record the time
                    // of the latest edit. The trailing edge fires below once
                    // the user pauses typing. The visible input still updates
                    // every frame so the UI feels real-time.
                    tab.key_filter_pending_since = Some(std::time::Instant::now());
                }

                // Drive the trailing edge of the debounce. We need to schedule
                // a repaint at the deadline because egui will otherwise sleep
                // and never re-enter this branch after the user stops typing.
                // Pull the delay straight from user settings (0..=2000 ms,
                // matches the slider in Display Settings). A value of 0 fires
                // the SCAN on the same frame the user stops typing.
                let debounce_ms = app.config.settings.key_filter_debounce_ms;
                if let Some(since) = tab.key_filter_pending_since {
                    let debounce = std::time::Duration::from_millis(debounce_ms);
                    let elapsed = since.elapsed();
                    if elapsed >= debounce {
                        tab.key_filter_pending_since = None;

                        let key_filter = tab.state.key_filter.clone();
                        let input = tab.key_filter_input.clone();
                        let input = input.trim();

                        // Process filter: empty -> "*", contains '*' -> as-is,
                        // otherwise wrap with '*' on both sides.
                        let processed_filter = if input.is_empty() {
                            WILD_KEY_FILTER.to_string()
                        } else if input.contains('*') {
                            input.to_string()
                        } else {
                            format!("{}{}{}", WILD_KEY_FILTER, input, WILD_KEY_FILTER)
                        };

                        app.update_string(key_filter, processed_filter);
                        app.tabs[active_tab_idx].state.spawn_load_keys();
                    } else {
                        ui.ctx().request_repaint_after(debounce - elapsed);
                    }
                }

                if refresh_btn.clicked() {
                    app.tabs[active_tab_idx].state.spawn_load_keys();
                }
            });

            // Show loading progress. We treat the debounce-pending window as
            // "loading" too so the user gets immediate visual feedback the
            // moment they stop typing, even before the SCAN actually starts.
            let debounce_pending = app.tabs[active_tab_idx].key_filter_pending_since.is_some();
            let show_loading = loading || debounce_pending;
            if show_loading {
                if !loading_progress_text.is_empty() {
                    ui.label(egui::RichText::new(&loading_progress_text).weak());
                } else {
                    // Debounce window: nothing meaningful to show yet, but make
                    // it clear work is queued.
                    ui.label(egui::RichText::new(tr(TranslationKey::Loading, current_lang)).weak());
                }
            }

            // Show "load more" button below filter (only if connected and not
            // actively loading). Suppress during the debounce window too to
            // avoid stale "Load More" clicks firing against the soon-to-be
            // replaced key set.
            if connected && !show_loading {
                let remaining_keys = if is_full_scan {
                    total_keys.saturating_sub(keys.len())
                } else {
                    // Cannot determine remaining keys when using filter
                    usize::MAX
                };

                if keys.len() >= MAX_LOADED_KEYS {
                    ui.label(
                        egui::RichText::new(tr(TranslationKey::TooManyKeys, current_lang))
                            .color(egui::Color32::PURPLE),
                    );
                } else if scan_has_more {
                    ui.horizontal(|ui| {
                        if remaining_keys > LOAD_MORE_BATCH_SIZE
                            && ui
                                .button(tr(TranslationKey::LoadMoreKeys, current_lang))
                                .clicked()
                            {
                                app.tabs[active_tab_idx].state.spawn_load_more_keys(false);
                            }
                        if ui
                            .button(tr(TranslationKey::LoadAllKeys, current_lang))
                            .clicked()
                        {
                            app.tabs[active_tab_idx].state.spawn_load_more_keys(true);
                        }
                    });
                }
            }

            ui.separator();

            // Check if we should group keys by colon
            let group_by_colon = app.config.settings.group_keys_by_colon;

            // Fill remaining space with scroll area - optimized with limit for large key lists
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .id_salt("keys_scroll")
                .show(ui, |ui| {
                    // Limit rendering to improve performance with large datasets
                    // Only show first MAX_LOADED_KEYS keys or all keys if less than that
                    let keys_to_show = if keys.len() > MAX_LOADED_KEYS {
                        &keys[0..MAX_LOADED_KEYS]
                    } else {
                        keys
                    };

                    if group_by_colon {
                        // Build and render key tree
                        let mut root = KeyTreeNode::new();
                        for key in keys_to_show {
                            root.insert(key);
                        }
                        render_key_tree(ui, &root, &selected_key, active_tab_idx, app, "root");
                    } else {
                        // Render flat list
                        for key in keys_to_show {
                            render_key_item(ui, key, key, &selected_key, active_tab_idx, app);
                        }
                    }
                });
        });

    // New key dialog
    if app.new_key_dialog.show {
        let current_lang = {
            let tab = &app.tabs[app.active_tab];
            app.poll_language(tab.state.language.clone())
        };
        render_new_key_dialog(app, ctx, current_lang);
    }
}

fn render_new_key_dialog(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    let mut open = true;
    egui::Window::new(tr(TranslationKey::NewKey, current_lang))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_pos(ctx.content_rect().center())
        .show(ctx, |ui| {
            egui::Grid::new("new_key_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .min_col_width(80.0)
                .show(ui, |ui| {
                    // Key name
                    ui.label(tr(TranslationKey::KeyNameLabel, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.new_key_dialog.key_name)
                            .desired_width(250.0)
                            .hint_text("my_key"),
                    );
                    ui.end_row();

                    // Type selector
                    ui.label(tr(TranslationKey::TypeLabel, current_lang));
                    egui::ComboBox::from_id_salt("new_key_type")
                        .selected_text(&app.new_key_dialog.key_type)
                        .width(250.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "string".to_string(),
                                "string",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "hash".to_string(),
                                "hash",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "list".to_string(),
                                "list",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "set".to_string(),
                                "set",
                            );
                            ui.selectable_value(
                                &mut app.new_key_dialog.key_type,
                                "zset".to_string(),
                                "zset",
                            );
                        });
                    ui.end_row();

                    // TTL
                    ui.label(tr(TranslationKey::TtlLabel, current_lang));
                    ui.add(
                        egui::TextEdit::singleline(&mut app.new_key_dialog.ttl)
                            .desired_width(250.0)
                            .hint_text(tr(TranslationKey::TtlNoExpirationHint, current_lang)),
                    );
                    ui.end_row();
                });

            ui.separator();

            // Value input with hint based on type
            let hint = match app.new_key_dialog.key_type.as_str() {
                "string" => tr(TranslationKey::ValueHintString, current_lang),
                "list" => tr(TranslationKey::ValueHintList, current_lang),
                "set" => tr(TranslationKey::ValueHintSet, current_lang),
                "hash" => tr(TranslationKey::ValueHintHash, current_lang),
                "zset" => tr(TranslationKey::ValueHintZset, current_lang),
                _ => tr(TranslationKey::ValueHintString, current_lang),
            };
            ui.label(format!(
                "{} ({})",
                tr(TranslationKey::ValueLabel, current_lang),
                hint
            ));
            ui.add_sized(
                [ui.available_width(), 120.0],
                egui::TextEdit::multiline(&mut app.new_key_dialog.value).hint_text(hint),
            );

            // Error message
            if !app.new_key_dialog.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &app.new_key_dialog.error_message);
            }

            ui.separator();

            // Buttons
            ui.horizontal(|ui| {
                if ui
                    .button(
                        egui::RichText::new(tr(TranslationKey::Save, current_lang))
                            .color(egui::Color32::from_rgb(50, 180, 50)),
                    )
                    .clicked()
                {
                    let key_name = app.new_key_dialog.key_name.trim().to_string();
                    if key_name.is_empty() {
                        app.new_key_dialog.error_message =
                            tr(TranslationKey::KeyNameEmptyError, current_lang).to_string();
                    } else {
                        let key_type = app.new_key_dialog.key_type.clone();
                        let value = app.new_key_dialog.value.clone();
                        let ttl: i64 = app.new_key_dialog.ttl.trim().parse().unwrap_or(-1);
                        let active_tab_idx = app.active_tab;
                        app.tabs[active_tab_idx]
                            .state
                            .spawn_create_new_key(key_name, key_type, value, ttl);
                        app.new_key_dialog.show = false;
                    }
                }

                if ui
                    .button(tr(TranslationKey::Cancel, current_lang))
                    .clicked()
                {
                    app.new_key_dialog.show = false;
                }
            });
        });

    if !open {
        app.new_key_dialog.show = false;
    }
}

/// Render a single key item
fn render_key_item(
    ui: &mut egui::Ui,
    full_key: &str,
    display_name: &str,
    selected_key: &Option<String>,
    active_tab_idx: usize,
    app: &mut RedisApp,
) {
    let is_selected = selected_key.as_ref().map(|s| s.as_str()) == Some(full_key);

    // Use allocate_ui_with_layout to make the entire row clickable
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ui.spacing().interact_size.y),
        egui::Sense::click(),
    );

    if response.clicked() {
        let tab = &mut app.tabs[active_tab_idx];
        *tab.state.selected_key.blocking_write() = Some(full_key.to_string());
        tab.state.spawn_load_value(
            full_key.to_string(),
            e_client_core::app_state::operations::keys::HashLoadMode::ReloadFields,
        );
    }

    // Draw the background for selected item
    if is_selected {
        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::same(2),
            ui.visuals().selection.bg_fill,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::same(2),
            ui.visuals().widgets.hovered.bg_fill,
        );
    }

    // Draw the text
    let text_color = if is_selected {
        ui.visuals().selection.stroke.color
    } else {
        ui.visuals().text_color()
    };

    ui.painter().text(
        rect.left_center() + egui::vec2(ui.spacing().item_spacing.x, 0.0),
        egui::Align2::LEFT_CENTER,
        display_name,
        egui::FontId::default(),
        text_color,
    );
}

/// Recursively render the key tree
fn render_key_tree(
    ui: &mut egui::Ui,
    node: &KeyTreeNode,
    selected_key: &Option<String>,
    active_tab_idx: usize,
    app: &mut RedisApp,
    path: &str,
) {
    // Then render children as collapsible sections
    for (name, child) in &node.children {
        let has_children = !child.children.is_empty();
        let has_key = child.full_key.is_some();

        if has_children {
            // This is a folder/group
            let header_id = ui.make_persistent_id(format!("key_group_{}_{}", path, name));
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                header_id,
                false, // Default to closed to avoid lag with many keys
            )
            .show_header(ui, |ui| {
                if has_key {
                    let full_key = child.full_key.as_ref().unwrap();
                    let is_selected =
                        selected_key.as_ref().map(|s| s.as_str()) == Some(full_key.as_str());

                    if ui.selectable_label(is_selected, name).clicked() {
                        let tab = &mut app.tabs[active_tab_idx];
                        *tab.state.selected_key.blocking_write() = Some(full_key.clone());
                        tab.state.spawn_load_value(
                            full_key.clone(),
                            e_client_core::app_state::operations::keys::HashLoadMode::ReloadFields,
                        );
                    }
                } else {
                    ui.label(name);
                }
            })
            .body(|ui| {
                let new_path = format!("{}:{}", path, name);
                render_key_tree(ui, child, selected_key, active_tab_idx, app, &new_path);
            });
        } else {
            // This is a leaf node
            if let Some(ref full_key) = child.full_key {
                render_key_item(ui, full_key, name, selected_key, active_tab_idx, app);
            }
        }
    }
}
