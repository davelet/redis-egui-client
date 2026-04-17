//! Settings panel module - UI components for settings window

pub mod ai_settings;
pub mod connection_settings;
pub mod display_settings;
pub mod general_settings;
pub mod shortcut_settings;

pub use ai_settings::{
    render_ai_settings_section, render_gim_import_dialog, render_json_import_preview,
};
pub use connection_settings::render_connection_settings;
pub use display_settings::render_display_settings;
pub use general_settings::{render_general_settings, render_update_settings};
pub use shortcut_settings::{SUPPORTED_KEYS, parse_key_from_str, render_shortcut_settings};

use crate::ui::window::shortcut_manager::get_shortcut_display;
use e_client_config::config::shortcuts::ShortcutAction;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};

use super::super::RedisApp;

// Re-export SettingsSection for use in other modules
pub use super::super::SettingsSection;

/// Settings window dimensions
pub const SETTINGS_WINDOW_WIDTH: f32 = 450.0;
pub const SETTINGS_WINDOW_HEIGHT: f32 = 400.0;

/// Render the settings window
pub fn render_settings_window(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    // Render GIM import dialog first (it's a separate window)
    render_gim_import_dialog(app, ctx, current_lang);
    // Render JSON import preview dialog
    render_json_import_preview(app, ctx, current_lang);

    // Check if we're currently editing a shortcut
    let is_editing_shortcut = app.shortcut_state.editing_shortcut.is_some();

    // Handle Esc key - cancel editing if in edit mode, otherwise close settings
    // Priority: help window > GIM dialog > import preview > shortcut editing > settings
    let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    if esc_pressed && !app.ai_model_editor.show && !app.show_help {
        if app.gim_import_dialog.show {
            // Close GIM import dialog only (don't close settings)
            app.gim_import_dialog.close();
        } else if app.json_import_preview.is_some() {
            // Close import preview
            app.json_import_preview = None;
        } else if is_editing_shortcut {
            // Cancel editing
            app.shortcut_state.editing_shortcut = None;
            app.shortcut_state.shortcut_input_buffer.clear();
            app.shortcut_state.shortcut_conflict_warning = None;
        } else {
            // Close settings window (only if help is not open)
            app.show_settings = false;
        }
    }

    // Handle close settings shortcut (only when not editing)
    if !is_editing_shortcut {
        let close_binding = app
            .config
            .settings
            .shortcuts
            .get_binding(&ShortcutAction::CloseSettings);
        let should_close = if close_binding == "Esc" {
            // Already handled above
            false
        } else if close_binding.starts_with("Ctrl+") || close_binding.starts_with("Cmd+") {
            let key_part = close_binding.split('+').nth(1);
            if let Some(key_str) = key_part {
                let key = parse_key_from_str(key_str);
                ctx.input(|i| {
                    let modifiers_match = if cfg!(target_os = "macos") {
                        i.modifiers.command
                    } else {
                        i.modifiers.ctrl
                    };
                    modifiers_match && key.map_or(false, |k| i.key_pressed(k))
                })
            } else {
                false
            }
        } else {
            false
        };

        if should_close {
            app.show_settings = false;
            app.shortcut_state.editing_shortcut = None;
            app.shortcut_state.shortcut_input_buffer.clear();
            app.shortcut_state.shortcut_conflict_warning = None;
        }
    }

    // Position window at top-right corner with some margin
    let screen_rect = ctx
        .input(|i| i.viewport().outer_rect)
        .unwrap_or(egui::Rect::ZERO);
    let top_right = egui::pos2(
        screen_rect.max.x - SETTINGS_WINDOW_WIDTH - 20.0,
        screen_rect.min.y + 40.0,
    );

    egui::Window::new(tr(TranslationKey::Settings, current_lang))
        .collapsible(false)
        .resizable(true)
        .default_pos(top_right)
        .min_size([SETTINGS_WINDOW_WIDTH, 200.0])
        .default_size([SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT])
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("settings_scroll")
                .max_height(600.0)
                .show(ui, |ui| {
                    // General settings (Theme, Language) - always visible, not collapsible
                    render_general_settings(app, ui, current_lang, ctx);
                    ui.separator();

                    // Connection settings
                    {
                        let header_text = tr(TranslationKey::SavedConnections, current_lang);
                        let is_open =
                            app.settings_expanded_section == Some(SettingsSection::Connection);
                        let response = egui::CollapsingHeader::new(header_text)
                            .id_salt("settings_connection_collapsible")
                            .open(Some(is_open))
                            .show(ui, |ui| {
                                ui.add_space(8.0);
                                render_connection_settings(app, ui, current_lang);
                            });

                        if response.header_response.clicked() {
                            if app.settings_expanded_section == Some(SettingsSection::Connection) {
                                app.settings_expanded_section = None;
                            } else {
                                app.settings_expanded_section = Some(SettingsSection::Connection);
                            }
                        }
                        ui.separator();
                    }

                    // Display settings
                    {
                        let header_text = tr(TranslationKey::DisplaySettings, current_lang);
                        let is_open =
                            app.settings_expanded_section == Some(SettingsSection::Display);
                        let response = egui::CollapsingHeader::new(header_text)
                            .id_salt("settings_display_collapsible")
                            .open(Some(is_open))
                            .show(ui, |ui| {
                                ui.add_space(8.0);
                                render_display_settings(app, ui, current_lang);
                            });

                        if response.header_response.clicked() {
                            if app.settings_expanded_section == Some(SettingsSection::Display) {
                                app.settings_expanded_section = None;
                            } else {
                                app.settings_expanded_section = Some(SettingsSection::Display);
                            }
                        }
                        ui.separator();
                    }

                    // AI settings
                    {
                        let header_text = tr(TranslationKey::AiSettings, current_lang);
                        let is_open = app.settings_expanded_section == Some(SettingsSection::Ai);
                        let response = egui::CollapsingHeader::new(header_text)
                            .id_salt("settings_ai_collapsible")
                            .open(Some(is_open))
                            .show(ui, |ui| {
                                ui.add_space(8.0);
                                render_ai_settings_section(app, ui, ctx, current_lang);
                            });

                        if response.header_response.clicked() {
                            if app.settings_expanded_section == Some(SettingsSection::Ai) {
                                app.settings_expanded_section = None;
                            } else {
                                app.settings_expanded_section = Some(SettingsSection::Ai);
                            }
                        }
                        ui.separator();
                    }

                    // Keyboard shortcuts
                    {
                        let header_text = tr(TranslationKey::KeyboardShortcuts, current_lang);
                        let is_open =
                            app.settings_expanded_section == Some(SettingsSection::Shortcuts);
                        let response = egui::CollapsingHeader::new(header_text)
                            .id_salt("settings_shortcuts_collapsible")
                            .open(Some(is_open))
                            .show(ui, |ui| {
                                ui.add_space(8.0);
                                render_shortcut_settings(app, ui, ctx, current_lang);
                            });

                        if response.header_response.clicked() {
                            if app.settings_expanded_section == Some(SettingsSection::Shortcuts) {
                                app.settings_expanded_section = None;
                            } else {
                                app.settings_expanded_section = Some(SettingsSection::Shortcuts);
                            }
                        }
                        ui.separator();
                    }

                    // Update settings
                    {
                        let header_text = tr(TranslationKey::UpdateSettings, current_lang);
                        let is_open =
                            app.settings_expanded_section == Some(SettingsSection::Update);
                        let response = egui::CollapsingHeader::new(header_text)
                            .id_salt("settings_update_collapsible")
                            .open(Some(is_open))
                            .show(ui, |ui| {
                                ui.add_space(8.0);
                                render_update_settings(app, ui, current_lang);
                            });

                        if response.header_response.clicked() {
                            if app.settings_expanded_section == Some(SettingsSection::Update) {
                                app.settings_expanded_section = None;
                            } else {
                                app.settings_expanded_section = Some(SettingsSection::Update);
                            }
                        }
                        ui.separator();
                    }
                });

            egui::Frame::NONE
                .fill(ui.visuals().faint_bg_color)
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let close_binding = app
                            .config
                            .settings
                            .shortcuts
                            .get_binding(&ShortcutAction::CloseSettings);
                        let help_binding = app
                            .config
                            .settings
                            .shortcuts
                            .get_binding(&ShortcutAction::ToggleHelp);

                        if ui
                            .button(format!(
                                "{}{}",
                                tr(TranslationKey::Close, current_lang),
                                get_shortcut_display(&close_binding)
                            ))
                            .clicked()
                        {
                            app.show_settings = false;
                            app.shortcut_state.editing_shortcut = None;
                            app.shortcut_state.shortcut_input_buffer.clear();
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(format!(
                                    "{}{}",
                                    tr(TranslationKey::Help, current_lang),
                                    get_shortcut_display(&help_binding)
                                ))
                                .clicked()
                            {
                                app.show_help = true;
                            }
                        });
                    });
                });
        });
}
