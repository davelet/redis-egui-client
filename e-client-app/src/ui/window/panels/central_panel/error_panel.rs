use e_client_config::config::Config;
use e_client_config::constants::{APP_NAME, LOAD_ERROR_TITLE};
use e_client_config::error::ConfigError;
use e_client_config::language::Language;

/// Render error panel when configuration fails to load
pub fn render_error_panel(err: ConfigError) -> Result<(), eframe::Error> {
    let err = err.to_message(Language::default());
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native(APP_NAME, options, move |ctx, _frame| {
        use std::cell::Cell;
        thread_local! {
            static SHOW_POPUP: Cell<bool> = Cell::new(false);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new(LOAD_ERROR_TITLE).color(egui::Color32::RED));
            ui.horizontal(|ui| {
                ui.label(err.clone());
            });
            ui.separator();

            if ui.button("click to reset problematic file").clicked() {
                reset_config_files();
                SHOW_POPUP.set(true);
            }
        });
        SHOW_POPUP.with(|popup| {
            if popup.get() {
                egui::Window::new("Well Done!")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Now restart your app.");
                        if ui.button("OK").clicked() {
                            std::process::exit(0);
                        }
                    });
            }
        })
    })
}

/// Reset problematic configuration files
fn reset_config_files() {
    if let Err(_) = Config::load_user_settings() {
        let _ = Config::reset_user_settings();
    }
    if let Err(_) = Config::load_window_params() {
        let _ = Config::reset_window_params();
    }
    if let Err(_) = Config::load_connections() {
        let _ = Config::clear_connections();
    }
}
