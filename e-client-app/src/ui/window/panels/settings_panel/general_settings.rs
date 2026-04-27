//! General settings UI components

use crate::ui::theme_to_visuals;
use crate::ui::window::{UpdateAction, handle_update_action};
use e_client_config::config::Theme;
use e_client_config::constants::{CHINESE, ENGLISH};
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

use super::super::super::RedisApp;

/// Render general settings section (theme, font, and language)
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

    // Global monospace font setting
    egui::Grid::new("global_monospace_grid")
        .num_columns(2)
        .spacing([40.0, 12.0])
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label(tr(TranslationKey::GlobalMonospace, current_lang));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut global_monospace = app.config.settings.global_monospace;
                if ui.checkbox(&mut global_monospace, "").changed() {
                    app.config.settings.global_monospace = global_monospace;
                    app.config.mark_settings_dirty();
                    // Apply font change immediately
                    crate::ui::font::setup_fonts(ctx, global_monospace);
                }
            });
            ui.end_row();
        });

    ui.separator();

    // Language setting
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

    // Menu language restart hint
    ui.add_space(8.0);
    ui.weak(tr(TranslationKey::MenuLanguageRestartHint, current_lang));
}

/// Render update checker settings
pub fn render_update_settings(app: &mut RedisApp, ui: &mut egui::Ui, current_lang: Language) {
    // Checkbox: Check on startup
    let mut enabled = app.config.settings.update_config.enabled;
    if ui
        .checkbox(
            &mut enabled,
            tr(TranslationKey::UpdateCheckOnStartup, current_lang),
        )
        .changed()
    {
        app.config.update_check_enabled(enabled);
    }

    // Check interval dropdown
    if enabled {
        ui.horizontal(|ui| {
            ui.label(tr(TranslationKey::UpdateCheckInterval, current_lang));

            let current_hours = app.config.settings.update_config.check_interval_hours;
            let intervals = [
                (
                    24,
                    tr(TranslationKey::UpdateCheckIntervalDaily, current_lang),
                ),
                (
                    168,
                    tr(TranslationKey::UpdateCheckIntervalWeekly, current_lang),
                ),
                (
                    720,
                    tr(TranslationKey::UpdateCheckIntervalMonthly, current_lang),
                ),
            ];

            let selected_idx = intervals
                .iter()
                .position(|(h, _)| *h == current_hours)
                .unwrap_or(0);

            egui::ComboBox::from_id_salt("update_check_interval")
                .selected_text(intervals[selected_idx].1)
                .show_ui(ui, |ui| {
                    for (idx, (hours, label)) in intervals.iter().enumerate() {
                        if ui.selectable_label(selected_idx == idx, *label).clicked() {
                            app.config.update_check_interval(*hours);
                        }
                    }
                });
        });
    }

    // Skip version display (when user already skipped a version)
    if let Some(skip_ver) = app.config.settings.update_config.skip_version.clone() {
        ui.horizontal(|ui| {
            ui.label(tr(TranslationKey::UpdateSkipVersion, current_lang));
            ui.label(format!(" ({})", skip_ver));
            if ui.small_button("✖").clicked() {
                app.config.update_skip_version(None);
            }
        });
    }

    // Skip this version button (when an update is available)
    if let Some(e_client_core::updater::UpdateCheckResult::UpdateAvailable { latest_version }) =
        &app.pending_update_result
    {
        let latest_version = latest_version.clone();
        // Don't show if this version is already skipped
        let already_skipped = app.config.settings.update_config.skip_version.as_deref()
            == Some(latest_version.as_str());
        if !already_skipped {
            ui.horizontal(|ui| {
                ui.label(tr(TranslationKey::UpdateAvailable, current_lang));
                if ui
                    .button(tr(TranslationKey::UpdateSkipVersion, current_lang))
                    .clicked()
                {
                    handle_update_action(app, UpdateAction::SkipVersion(latest_version.clone()));
                }
            });
        }
    }

    // Manual check button
    ui.horizontal(|ui| {
        if ui
            .button(tr(TranslationKey::UpdateCheckManually, current_lang))
            .clicked()
        {
            handle_update_action(app, UpdateAction::CheckManually);
        }

        // Show progress if checking
        if app.update_check_in_progress {
            ui.spinner();
        }
    });
}

/// Get display name for theme in current language
fn theme_display_name(theme: Theme, lang: Language) -> String {
    match theme {
        Theme::System => tr(TranslationKey::ThemeSystem, lang).to_string(),
        Theme::Light => tr(TranslationKey::ThemeLight, lang).to_string(),
        Theme::Dark => tr(TranslationKey::ThemeDark, lang).to_string(),
        Theme::Dracula => tr(TranslationKey::ThemeDracula, lang).to_string(),
        Theme::Nord => tr(TranslationKey::ThemeNord, lang).to_string(),
        Theme::Gruvbox => tr(TranslationKey::ThemeGruvbox, lang).to_string(),
        Theme::Monokai => tr(TranslationKey::ThemeMonokai, lang).to_string(),
        Theme::OneDark => tr(TranslationKey::ThemeOneDark, lang).to_string(),
        Theme::TokyoNight => tr(TranslationKey::ThemeTokyoNight, lang).to_string(),
        Theme::SolarizedDark => tr(TranslationKey::ThemeSolarizedDark, lang).to_string(),
        Theme::SolarizedLight => tr(TranslationKey::ThemeSolarizedLight, lang).to_string(),
    }
}
