//! Top panel - connection bar and settings button

use crate::help::get_help_sections;
use crate::ui::window::RedisApp;
use crate::ui::window::components::markdown::render_markdown;
use e_client_basics::constants::WILD_KEY_FILTER;
use e_client_basics::constants::{GITHUB_REPO_URL, ONLINE_DOCS_URL};
use e_client_basics::emoji;
use e_client_basics::emoji::web::WEB;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};
use std::sync::OnceLock;

use super::settings_panel::{SettingsSection, render_settings_window};
use crate::ui::window::{UpdateAction, handle_update_action};

/// Cached donate image texture
static DONATE_TEXTURE: OnceLock<std::sync::Arc<egui::TextureHandle>> = OnceLock::new();

/// Load donate image texture (cached)
fn get_donate_texture(ctx: &egui::Context) -> std::sync::Arc<egui::TextureHandle> {
    DONATE_TEXTURE
        .get_or_init(|| {
            let image_bytes = include_bytes!("../../../../../assets/donate.png");
            let image = image::load_from_memory(image_bytes).unwrap_or_else(|_| {
                image::DynamicImage::ImageRgba8(image::RgbaImage::new(1, 1)) // Fallback to empty image
            });
            let rgba = image.to_rgba8();
            let size = [rgba.width() as _, rgba.height() as _];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
            std::sync::Arc::new(ctx.load_texture(
                "donate",
                color_image,
                egui::TextureOptions::default(),
            ))
        })
        .clone()
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
                            ui.colored_label(color, emoji::status::DOT);
                        }
                    }
                }
            }

            ui.label(tr(TranslationKey::ConnectionUrl, current_lang));

            // Connection dropdown - disabled when connected
            let selected_name = selected_connection
                .and_then(|idx| app.config.connections.get(idx))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| tr(TranslationKey::SelectConnection, current_lang).to_string());

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
                                        ui.colored_label(color, emoji::status::DOT);
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
                if ui
                    .button(tr(TranslationKey::Disconnect, current_lang))
                    .clicked()
                {
                    app.tabs[active_tab_idx].state.spawn_disconnect();
                    // Clear tab name, color, and filter on disconnect, but keep selected_connection
                    let tab = &mut app.tabs[active_tab_idx];
                    tab.name = format!("{} {}", tr(TranslationKey::Tab, current_lang), tab.id);
                    tab.connected_color = None;
                    tab.key_filter_input.clear();

                    let key_filter = tab.state.key_filter.clone();
                    app.update_string(key_filter, WILD_KEY_FILTER.to_string());
                }

                ui.separator();
                ui.label(tr(TranslationKey::Database, current_lang));

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
                if ui
                    .button(tr(TranslationKey::Connect, current_lang))
                    .clicked()
                {
                    let tab = &mut app.tabs[active_tab_idx];
                    if let Some(idx) = tab.selected_connection {
                        if let Some(conn) = app.config.connections.get(idx) {
                            let conn_clone = conn.clone();
                            *tab.state.connection_param.blocking_write() = Some(conn_clone.clone());

                            // Load preferences for this connection
                            app.load_connection_preferences(active_tab_idx);

                            // Connect with preferred DB
                            app.spawn_connect_with_initial_db(active_tab_idx);
                        }
                    } else {
                        // No connection selected - show error
                        *tab.state.error_message.blocking_write() =
                            tr(TranslationKey::PleaseSelectConnection, current_lang).to_string();
                    }
                }

                // "Open in New Tab" button
                if let Some(idx) = selected_connection {
                    if ui
                        .button(format!(
                            "{} {}",
                            emoji::navigation::NEW_TAB,
                            tr(TranslationKey::OpenInNewTab, current_lang)
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
                            tr(TranslationKey::EditConnection, current_lang)
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
                    .button(format!(
                        "+ {}",
                        tr(TranslationKey::NewConnection, current_lang)
                    ))
                    .clicked()
                {
                    app.new_connection.show = true;
                }
            });

            // Right side - Update indicator + Settings button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Settings button (rightmost)
                if ui.button(emoji::action::SETTINGS).clicked() {
                    app.show_settings = true;
                }

                // Update available indicator (left of settings)
                if let Some(e_client_core::updater::UpdateCheckResult::UpdateAvailable {
                    latest_version,
                }) = &app.pending_update_result
                {
                    let version_str = latest_version.clone();
                    let update_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new(&version_str).color(egui::Color32::from_rgb(0x4C, 0xAF, 0x50)),
                        ),
                    );
                    if update_btn.clicked() {
                        handle_update_action(app, UpdateAction::OpenReleasePage(version_str));
                    }
                }
            });
        });
    });

    // Settings window
    if app.show_settings {
        render_settings_window(app, ctx, current_lang);
    }

    // Help overlay
    if app.show_help {
        render_help_window(app, ctx, current_lang);
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

/// Render help overlay window
fn render_help_window(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    // Handle Esc key to close
    let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    if esc_pressed {
        app.show_help = false;
        return;
    }

    let sections = get_help_sections(current_lang);
    let selected_idx = app
        .help_selected_section
        .unwrap_or(0)
        .min(sections.len().saturating_sub(1));

    if sections.is_empty() {
        return;
    }

    let selected_section = &sections[selected_idx];

    egui::Window::new(tr(TranslationKey::Help, current_lang))
        .id(egui::Id::new("help_window"))
        .default_size([750.0, 550.0])
        .min_size([600.0, 400.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            // Left sidebar - module list
            egui::SidePanel::left("help_sidebar")
                .width_range(180.0..=200.0)
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(4.0);

                        // Sidebar header - use text instead of emoji
                        ui.label(
                            egui::RichText::new(tr(TranslationKey::HelpContents, current_lang))
                                .strong()
                                .size(14.0),
                        );
                        ui.add_space(8.0);

                        // Module list with unique IDs
                        for (idx, section) in sections.iter().enumerate() {
                            let is_selected = idx == selected_idx;
                            let label = egui::RichText::new(&section.title).size(13.0).strong();

                            let response = ui
                                .push_id(idx, |ui| ui.selectable_label(is_selected, label))
                                .inner;
                            if response.clicked() {
                                app.help_selected_section = Some(idx);
                            }
                        }

                        ui.add_space(8.0);
                        ui.separator();

                        // Spacer to push links to bottom
                        ui.add_space(16.0);

                        // Online docs and GitHub links
                        ui.hyperlink_to(
                            tr_fmt(TranslationKey::HelpOnlineDocs, current_lang, &[WEB]),
                            ONLINE_DOCS_URL,
                        );
                        ui.hyperlink_to(
                            tr_fmt(TranslationKey::HelpGithub, current_lang, &[WEB]),
                            GITHUB_REPO_URL,
                        );

                        // Version info at bottom of sidebar
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{} v{}",
                                env!("CARGO_PKG_NAME"),
                                env!("CARGO_PKG_VERSION")
                            ));
                        });

                        // Promotions: Donation and other tools
                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // WeChat donation
                        ui.label(
                            egui::RichText::new(tr(TranslationKey::SupportUs, current_lang))
                                .size(12.0)
                                .weak(),
                        );
                        ui.add_space(4.0);

                        // Load and display donate image (cached)
                        let donate_texture = get_donate_texture(ui.ctx());
                        ui.add(
                            egui::Image::new(&*donate_texture)
                                .max_width(160.0)
                                .fit_to_original_size(1.0),
                        )
                        .on_hover_text(tr(TranslationKey::SupportUsDesc, current_lang));

                        ui.add_space(12.0);

                        // Recommended tool
                        ui.label(
                            egui::RichText::new(tr(TranslationKey::Recommended, current_lang))
                                .size(12.0)
                                .weak(),
                        );
                        ui.add_space(4.0);
                        ui.hyperlink_to(
                            tr(TranslationKey::GitIntelligenceMessage, current_lang),
                            "https://git-intelligence-message.pages.dev/",
                        );
                        ui.label(
                            egui::RichText::new(tr(
                                TranslationKey::GitIntelligenceMessageDesc,
                                current_lang,
                            ))
                            .size(11.0)
                            .weak(),
                        );
                    });
                });

            // Vertical separator
            ui.add(egui::Separator::default().spacing(8.0));

            // Right content area
            egui::CentralPanel::default().show_inside(ui, |ui| {
                // Section title
                ui.label(
                    egui::RichText::new(&selected_section.title)
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(12.0);

                // Content with unique ID for scroll area
                egui::ScrollArea::vertical()
                    .id_salt(selected_idx)
                    .show(ui, |ui| {
                        // Subsections with unique IDs
                        if !selected_section.subsections.is_empty() {
                            ui.label(
                                egui::RichText::new(tr(
                                    TranslationKey::HelpInThisSection,
                                    current_lang,
                                ))
                                .size(12.0)
                                .weak(),
                            );
                            ui.add_space(8.0);

                            for (sub_idx, sub) in selected_section.subsections.iter().enumerate() {
                                ui.push_id(sub_idx, |ui| ui.label(format!("> {}", sub.title)));
                            }

                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(12.0);
                        }

                        // Section content - check for shortcuts button marker
                        let shortcuts_button_marker_en = "[View All Shortcuts in Settings]";
                        let shortcuts_button_marker_zh = "[在设置中查看所有快捷键]";

                        if selected_section
                            .content
                            .contains(shortcuts_button_marker_en)
                            || selected_section
                                .content
                                .contains(shortcuts_button_marker_zh)
                        {
                            // Split content at the marker
                            let marker = if selected_section
                                .content
                                .contains(shortcuts_button_marker_en)
                            {
                                shortcuts_button_marker_en
                            } else {
                                shortcuts_button_marker_zh
                            };

                            let parts: Vec<&str> = selected_section.content.split(marker).collect();
                            if !parts.is_empty() {
                                // Render content before marker
                                render_markdown(parts[0].trim(), ui);
                                ui.add_space(16.0);

                                // Render button
                                let button_text =
                                    tr(TranslationKey::KeyboardShortcuts, current_lang);
                                if ui.button(button_text).clicked() {
                                    app.show_help = false;
                                    app.show_settings = true;
                                    // Set to keyboard shortcuts section
                                    app.settings_expanded_section =
                                        Some(SettingsSection::Shortcuts);
                                }
                                ui.add_space(8.0);

                                // Render content after marker if any
                                if parts.len() > 1 {
                                    render_markdown(parts[1].trim(), ui);
                                }
                            } else {
                                render_markdown(&selected_section.content, ui);
                            }
                        } else {
                            render_markdown(&selected_section.content, ui);
                        }
                    });
            });
        });
}
