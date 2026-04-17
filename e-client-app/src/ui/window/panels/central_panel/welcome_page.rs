use crate::ui::window::RedisApp;
use crate::ui::window::shortcut_manager::get_shortcut_display;
use e_client_basics::emoji;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr, tr_fmt};

use super::utils::parse_color_hex;

/// Render welcome page
pub fn render_welcome_page(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    _ctx: &egui::Context,
    current_lang: Language,
    show_open_connections: bool,
    new_conn_shortcut: &str,
) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);

        // Welcome title
        ui.heading(
            egui::RichText::new(tr_fmt(
                TranslationKey::WelcomeTitle,
                current_lang,
                &[env!("CARGO_PKG_NAME")],
            ))
            .size(32.0),
        );
        ui.add_space(20.0);

        // Welcome message
        ui.label(egui::RichText::new(tr(TranslationKey::WelcomeMessage, current_lang)).size(16.0));
        ui.add_space(30.0);

        // Get started button
        render_new_connection_button(app, ui, current_lang, new_conn_shortcut);

        ui.add_space(10.0);

        // Instructions
        ui.label(
            egui::RichText::new(tr(TranslationKey::WelcomeInstruction, current_lang))
                .weak()
                .size(14.0),
        );

        // Connection list section
        render_saved_connections(app, ui, current_lang, show_open_connections);
    });
}

/// Render new connection button
fn render_new_connection_button(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    new_conn_shortcut: &str,
) {
    if ui
        .button(
            egui::RichText::new(format!(
                "{}{}",
                tr(TranslationKey::NewConnection, current_lang),
                get_shortcut_display(new_conn_shortcut)
            ))
            .size(18.0)
            .color(egui::Color32::WHITE),
        )
        .clicked()
    {
        app.new_connection.show = true;
    }
}

/// Render saved connections list
fn render_saved_connections(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    show_open_connections: bool,
) {
    let connections: Vec<_> = app.config.connections.connections.iter().cloned().collect();
    let open_conn_names: Vec<_> = app.config.window.open_connections.connection_names.clone();

    if connections.is_empty() {
        return;
    }

    ui.add_space(40.0);
    ui.separator();
    ui.add_space(20.0);

    // Section title
    ui.heading(egui::RichText::new(tr(TranslationKey::SavedConnections, current_lang)).size(20.0));
    ui.add_space(10.0);

    // Show open connections prompt and Connect All button if there are any
    if show_open_connections && !open_conn_names.is_empty() {
        render_open_connections_prompt(app, ui, current_lang, &open_conn_names, &connections);
    }

    // Connection table
    // Only mark connections as "open" (blue) if show_unclosed_connections setting is enabled
    let should_highlight_open = app.config.settings.show_unclosed_connections;
    render_connection_table(
        app,
        ui,
        current_lang,
        &connections,
        &open_conn_names,
        should_highlight_open,
    );
}

/// Render open connections prompt
fn render_open_connections_prompt(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    open_conn_names: &[String],
    connections: &Vec<e_client_config::connection::RedisConnectionConfig>,
) {
    ui.label(
        egui::RichText::new(tr(
            TranslationKey::OpenConnectionsPromptMessage,
            current_lang,
        ))
        .size(12.0)
        .color(egui::Color32::BLUE),
    );
    ui.add_space(10.0);

    // Connect All button - connect first connection in current tab, others in new tabs
    if ui
        .button(tr(TranslationKey::ConnectAll, current_lang))
        .clicked()
    {
        for (i, conn_name) in open_conn_names.iter().enumerate() {
            if let Some(conn_idx) = connections.iter().position(|c| &c.name == conn_name) {
                let conn = connections[conn_idx].clone();
                if i == 0 && !app.config.settings.open_connections_in_new_tab {
                    // Connect first connection in current tab
                    app.connect_in_current_tab(conn_idx, &conn);
                } else {
                    // Create new tabs for remaining connections
                    app.create_tab_with_connection(conn_idx, conn);
                }
            }
        }
        app.show_open_connections_prompt = false;
    }
    ui.add_space(10.0);
}

/// Render connection table
fn render_connection_table(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    connections: &[e_client_config::connection::RedisConnectionConfig],
    open_conn_names: &[String],
    should_highlight_open: bool,
) {
    let available_width = ui.available_width();
    let table_width = available_width.min(600.0).max(400.0);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_width(table_width);

        // Table header
        ui.horizontal(|ui| {
            ui.set_width(table_width);
            ui.colored_label(
                egui::Color32::GRAY,
                tr(TranslationKey::Number, current_lang),
            );
            ui.colored_label(
                egui::Color32::GRAY,
                tr(TranslationKey::ConnectionName, current_lang),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.colored_label(
                    egui::Color32::GRAY,
                    tr(TranslationKey::Action, current_lang),
                );
            });
        });
        ui.separator();

        // Table rows
        for (idx, conn) in connections.iter().enumerate() {
            // Only mark as "open" if the setting is enabled and connection is in open list
            let is_open = should_highlight_open && open_conn_names.contains(&conn.name);
            render_connection_row(app, ui, current_lang, idx, conn, is_open, table_width);

            if idx < connections.len() - 1 {
                ui.separator();
            }
        }
    });
}

/// Render single connection row
fn render_connection_row(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    idx: usize,
    conn: &e_client_config::connection::RedisConnectionConfig,
    is_open: bool,
    table_width: f32,
) {
    ui.horizontal(|ui| {
        ui.set_width(table_width);

        // Row number (1-based)
        ui.colored_label(egui::Color32::GRAY, format!("{}", idx + 1));

        // Color indicator and name
        ui.horizontal(|ui| {
            render_color_indicator(ui, &conn.color);
            render_connection_name(ui, conn, is_open);
        });

        // Action buttons
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Check if this row is in confirm delete state
            let is_confirming = app
                .delete_connection_confirm
                .as_ref()
                .map(|(i, _)| *i == idx)
                .unwrap_or(false);

            // Delete / Confirm Delete button
            if is_confirming {
                if ui
                    .button(
                        egui::RichText::new(tr(TranslationKey::ConfirmDelete, current_lang))
                            .color(egui::Color32::RED),
                    )
                    .clicked()
                {
                    let _ = app.config.remove_connection(&conn.name);
                    app.delete_connection_confirm = None;
                }
            } else {
                if ui
                    .button(
                        egui::RichText::new(tr(TranslationKey::Delete, current_lang))
                            .color(egui::Color32::RED),
                    )
                    .clicked()
                {
                    app.delete_connection_confirm = Some((idx, conn.name.clone()));
                }
            }

            ui.add_space(10.0);

            // Connect button
            if ui
                .button(tr(TranslationKey::Connect, current_lang))
                .clicked()
            {
                if app.config.settings.open_connections_in_new_tab {
                    app.create_tab_with_connection(idx, conn.clone());
                    // Clear open connections list when connecting
                    app.config.clear_open_connections();
                    app.show_open_connections_prompt = false;
                } else {
                    app.connect_in_current_tab(idx, conn);
                }
            }
        });
    });
}

/// Render color indicator
fn render_color_indicator(ui: &mut egui::Ui, color: &Option<String>) {
    if let Some(color_hex) = color {
        if let Some(color) = parse_color_hex(color_hex) {
            ui.colored_label(color, emoji::status::DOT);
        }
    }
}

/// Render connection name
fn render_connection_name(
    ui: &mut egui::Ui,
    conn: &e_client_config::connection::RedisConnectionConfig,
    is_open: bool,
) {
    let name_text = if is_open {
        egui::RichText::new(&conn.name)
            .color(egui::Color32::BLUE)
            .strong()
    } else {
        egui::RichText::new(&conn.name)
    };
    ui.label(name_text);
}
