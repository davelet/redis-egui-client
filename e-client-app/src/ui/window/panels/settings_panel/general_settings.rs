//! General settings UI components

use crate::ui::theme_to_visuals;
use e_client_config::config::Theme;
use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

use super::super::super::RedisApp;

/// Render general settings section (theme and language only)
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
}

/// Get display name for theme in current language
fn theme_display_name(theme: Theme, lang: Language) -> String {
    match theme {
        Theme::System => tr(TranslationKey::ThemeSystem, lang).to_string(),
        Theme::Light => tr(TranslationKey::ThemeLight, lang).to_string(),
        Theme::Dark => tr(TranslationKey::ThemeDark, lang).to_string(),
    }
}
