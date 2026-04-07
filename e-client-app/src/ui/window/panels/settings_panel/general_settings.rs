//! General settings UI components

use crate::ui::theme_to_visuals;
use e_client_config::config::Theme;
use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::{tr, TranslationKey};

use super::super::super::RedisApp;

/// Render general settings section (language, theme, auto-connect, etc.)
pub fn render_general_settings(
    app: &mut RedisApp,
    ui: &mut egui::Ui,
    current_lang: Language,
    ctx: &egui::Context,
) {
    // Theme setting
    egui::Grid::new("theme_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::Theme, current_lang));
            let current_theme = app.config.settings.theme;
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::ComboBox::from_id_salt("settings_theme_select")
                    .selected_text(theme_display_name(current_theme, current_lang))
                    .show_ui(ui, |ui| {
                        for theme in Theme::all() {
                            let display = theme_display_name(*theme, current_lang);
                            if ui
                                .selectable_label(current_theme == *theme, display)
                                .clicked()
                            {
                                app.config.settings.theme = *theme;
                                app.config.mark_settings_dirty();
                                // Apply theme immediately
                                let mut style = (*ctx.style()).clone();
                                style.visuals = theme_to_visuals(*theme);
                                ctx.set_style(style);
                            }
                        }
                    });
            });
            ui.end_row();
        });

    ui.separator();

    // Language setting - separate grid
    egui::Grid::new("language_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::Language, current_lang));
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
        });

    ui.separator();

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

    ui.separator();

    // Group keys by colon setting
    egui::Grid::new("group_keys_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::GroupKeysByColon, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut group_keys = app.config.settings.group_keys_by_colon;
                if ui.checkbox(&mut group_keys, "").changed() {
                    app.config.settings.group_keys_by_colon = group_keys;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Auto refresh TTL setting
    egui::Grid::new("auto_refresh_ttl_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::AutoRefreshTtl, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut auto_refresh_ttl = app.config.settings.auto_refresh_ttl;
                if ui.checkbox(&mut auto_refresh_ttl, "").changed() {
                    app.config.settings.auto_refresh_ttl = auto_refresh_ttl;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Auto expand composite types setting
    egui::Grid::new("auto_expand_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::AutoExpand, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut auto_expand = app.config.settings.auto_expand;
                if ui.checkbox(&mut auto_expand, "").changed() {
                    app.config.settings.auto_expand = auto_expand;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();

            // Threshold slider (only effective when auto_expand is enabled)
            let threshold_enabled = app.config.settings.auto_expand;
            let mut threshold = app.config.settings.auto_expand_threshold as f32;
            ui.label(tr(TranslationKey::AutoExpandThreshold, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_enabled(
                    threshold_enabled,
                    egui::Slider::new(&mut threshold, 1.0..=100.0).show_value(true),
                );
                // Sync back every frame so text-input also takes effect.
                // clamp avoids stale values from outside the range.
                let threshold = threshold.clamp(1.0, 100.0);
                if threshold_enabled
                    && (threshold as usize) != app.config.settings.auto_expand_threshold
                {
                    app.config.settings.auto_expand_threshold = threshold as usize;
                    app.config.mark_settings_dirty();
                }
            });
            ui.end_row();
        });
}

/// Get display name for theme in current language
fn theme_display_name(theme: Theme, lang: Language) -> String {
    match theme {
        Theme::System => tr(TranslationKey::ThemeSystem, lang).to_string(),
        Theme::Light => tr(TranslationKey::ThemeLight, lang).to_string(),
        Theme::Dark => tr(TranslationKey::ThemeDark, lang).to_string(),
    }
}
