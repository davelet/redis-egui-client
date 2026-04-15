//! Display settings UI components

use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

use super::super::super::RedisApp;

/// Render display settings section
pub fn render_display_settings(app: &mut RedisApp, ui: &mut egui::Ui, current_lang: Language) {
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
