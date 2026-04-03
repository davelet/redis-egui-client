//! Settings panel module - UI components for settings window

pub mod ai_settings;
pub mod general_settings;
pub mod shortcut_settings;

pub use ai_settings::render_ai_settings_section;
pub use general_settings::render_general_settings;
pub use shortcut_settings::{parse_key_from_str, render_shortcut_settings, SUPPORTED_KEYS};

use e_client_config::config::shortcuts::ShortcutAction;
use e_client_config::language::Language;
use e_client_config::translations::{tr, TranslationKey};

use super::super::RedisApp;

// Re-export SettingsSection for use in other modules
pub use super::super::SettingsSection;

/// Settings window dimensions
pub const SETTINGS_WINDOW_WIDTH: f32 = 450.0;
pub const SETTINGS_WINDOW_HEIGHT: f32 = 400.0;

/// Render the settings window
pub fn render_settings_window(app: &mut RedisApp, ctx: &egui::Context, current_lang: Language) {
    // Check if we're currently editing a shortcut
    let is_editing_shortcut = app.shortcut_state.editing_shortcut.is_some();

    // Handle Esc key - cancel editing if in edit mode, otherwise close settings
    // But if AI model editor is open, let it handle ESC (don't close settings)
    let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    if esc_pressed && !app.ai_model_editor.show {
        if is_editing_shortcut {
            // Cancel editing
            app.shortcut_state.editing_shortcut = None;
            app.shortcut_state.shortcut_input_buffer.clear();
            app.shortcut_state.shortcut_conflict_warning = None;
        } else {
            // Close settings window
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
                    // General settings
                    render_general_settings(app, ui, current_lang, ctx);

                    ui.separator();

                    // AI settings (collapsible) - mutually exclusive with shortcuts
                    let ai_header_text = tr(TranslationKey::AiSettings, current_lang);
                    let ai_open = app.settings_expanded_section == Some(SettingsSection::Ai);
                    let ai_response = egui::CollapsingHeader::new(ai_header_text)
                        .id_salt("settings_ai_collapsible")
                        .open(Some(ai_open))
                        .show(ui, |ui| {
                            ui.add_space(8.0);
                            render_ai_settings_section(app, ui, ctx, current_lang);
                        });

                    // Update expanded state based on user interaction
                    if ai_response.header_response.clicked() {
                        if app.settings_expanded_section == Some(SettingsSection::Ai) {
                            // Close AI section
                            app.settings_expanded_section = None;
                        } else {
                            // Open AI section, close shortcuts
                            app.settings_expanded_section = Some(SettingsSection::Ai);
                        }
                    }

                    ui.separator();

                    // Keyboard shortcuts (collapsible) - mutually exclusive with AI
                    let shortcuts_header_text = tr(TranslationKey::KeyboardShortcuts, current_lang);
                    let shortcuts_open =
                        app.settings_expanded_section == Some(SettingsSection::Shortcuts);
                    let shortcuts_response = egui::CollapsingHeader::new(shortcuts_header_text)
                        .id_salt("settings_shortcuts_collapsible")
                        .open(Some(shortcuts_open))
                        .show(ui, |ui| {
                            ui.add_space(8.0);
                            render_shortcut_settings(app, ui, ctx, current_lang);
                        });

                    // Update expanded state based on user interaction
                    if shortcuts_response.header_response.clicked() {
                        if app.settings_expanded_section == Some(SettingsSection::Shortcuts) {
                            // Close shortcuts section
                            app.settings_expanded_section = None;
                        } else {
                            // Open shortcuts section, close AI
                            app.settings_expanded_section = Some(SettingsSection::Shortcuts);
                        }
                    }

                    ui.separator();
                });

            egui::Frame::NONE
                .fill(ui.visuals().faint_bg_color)
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(tr(TranslationKey::Close, current_lang)).clicked() {
                            app.show_settings = false;
                            app.shortcut_state.editing_shortcut = None;
                            app.shortcut_state.shortcut_input_buffer.clear();
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(tr(TranslationKey::Help, current_lang)).clicked() {
                                app.show_help = true;
                            }
                        });
                    });
                });
        });
}
