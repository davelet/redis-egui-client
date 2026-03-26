pub mod error_panel;
pub mod render_edit_mode;
pub mod render_element_edit;
pub mod utils;
pub mod value_renderer;
pub mod view_mode;
pub mod welcome_page;

// Re-export main functions
pub use error_panel::render_error_panel;
pub use render_edit_mode::render_edit_mode;
pub use render_element_edit::render_element_edit_dialog;
pub use view_mode::render_view_mode;
pub use welcome_page::render_welcome_page;

use crate::ui::window::RedisApp;

/// Main entry point for central panel rendering
pub fn render_central_panel(app: &mut RedisApp, ctx: &egui::Context) {
    if app.get_active_tab().is_none() {
        return;
    }

    let active_tab_idx = app.active_tab;
    let tab = &app.tabs[active_tab_idx];
    let current_lang = app.poll_language(tab.state.language.clone());
    let connected = app.poll_bool(tab.state.connected.clone());
    let selected_key = app.poll_option_string(tab.state.selected_key.clone());
    let value = app.poll_option_value(tab.state.key_value.clone());
    let ttl = app.poll_i64(tab.state.key_ttl.clone());
    let edit_state = tab.state.edit_state.blocking_read().clone();
    let show_open_connections = app.show_open_connections_prompt;

    egui::CentralPanel::default().show(ctx, |ui| {
        if !connected {
            render_welcome_page(app, ui, ctx, current_lang, show_open_connections);
            return;
        }

        if let Some(key) = selected_key {
            if edit_state.editing {
                render_edit_mode(app, ui, active_tab_idx, &key, &value, ttl, current_lang);
            } else {
                render_view_mode(
                    app,
                    ui,
                    ctx,
                    active_tab_idx,
                    &key,
                    &value,
                    ttl,
                    current_lang,
                );
            }
        } else {
            use e_client_config::translations::{keys, tr};
            ui.label(tr(keys::SELECT_KEY_PROMPT, current_lang));
        }
    });

    // Element edit dialog
    render_element_edit_dialog(app, ctx, current_lang);
}
