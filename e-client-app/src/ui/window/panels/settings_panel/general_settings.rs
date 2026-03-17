//! General settings UI components

use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};

use super::super::super::RedisApp;

/// Render general settings section (language, auto-connect, etc.)
pub fn render_general_settings(app: &mut RedisApp, ui: &mut egui::Ui, current_lang: Language) {
    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            // Language setting
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(tr(keys::LANGUAGE, current_lang));
            });
            let lang = current_lang;
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::ComboBox::from_id_salt("settings_lang_select")
                    .selected_text(match lang {
                        Language::English => ENGLISH,
                        Language::Chinese => CHINESE,
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(matches!(lang, Language::English), ENGLISH)
                            .clicked()
                        {
                            app.update_language(Language::English);
                        }
                        if ui
                            .selectable_label(matches!(lang, Language::Chinese), CHINESE)
                            .clicked()
                        {
                            app.update_language(Language::Chinese);
                        }
                    });
            });
            ui.end_row();

            // Auto connect setting
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(tr(keys::AUTO_CONNECT, current_lang));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut auto_connect = app.config.settings.auto_connect;
                if ui.checkbox(&mut auto_connect, "").changed() {
                    app.config.update_auto_connect(auto_connect);
                }
            });
            ui.end_row();

            // Show unclosed connections setting
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(tr(keys::SHOW_UNCLOSED_CONNECTIONS, current_lang));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut show_unclosed = app.config.settings.show_unclosed_connections;
                if ui.checkbox(&mut show_unclosed, "").changed() {
                    app.config.settings.show_unclosed_connections = show_unclosed;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();

            // Allow duplicate connections setting
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(tr(keys::ALLOW_DUPLICATE_CONNECTIONS, current_lang));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut allow_duplicate = app.config.settings.allow_duplicate_connections;
                if ui.checkbox(&mut allow_duplicate, "").changed() {
                    app.config.settings.allow_duplicate_connections = allow_duplicate;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();

            // Group keys by colon setting
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(tr(keys::GROUP_KEYS_BY_COLON, current_lang));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut group_keys = app.config.settings.group_keys_by_colon;
                if ui.checkbox(&mut group_keys, "").changed() {
                    app.config.settings.group_keys_by_colon = group_keys;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });
}
