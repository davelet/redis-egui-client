//! Connection settings UI components

use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

use super::super::super::RedisApp;

/// Render connection settings section
pub fn render_connection_settings(app: &mut RedisApp, ui: &mut egui::Ui, current_lang: Language) {
    // Open connections in new tab setting
    egui::Grid::new("open_in_new_tab_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::OpenConnectionsInNewTab, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut open_new = app.config.settings.open_connections_in_new_tab;
                if ui.checkbox(&mut open_new, "").changed() {
                    app.config.settings.open_connections_in_new_tab = open_new;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Auto connect setting
    egui::Grid::new("auto_connect_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::AutoConnect, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut auto_connect = app.config.settings.auto_connect;
                if ui.checkbox(&mut auto_connect, "").changed() {
                    app.config.update_auto_connect(auto_connect);
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Show unclosed connections setting
    egui::Grid::new("show_unclosed_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::ShowUnclosedConnections, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut show_unclosed = app.config.settings.show_unclosed_connections;
                if ui.checkbox(&mut show_unclosed, "").changed() {
                    app.config.settings.show_unclosed_connections = show_unclosed;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Allow duplicate connections setting
    egui::Grid::new("allow_duplicate_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::AllowDuplicateConnections, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut allow_duplicate = app.config.settings.allow_duplicate_connections;
                if ui.checkbox(&mut allow_duplicate, "").changed() {
                    app.config.settings.allow_duplicate_connections = allow_duplicate;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });
}
