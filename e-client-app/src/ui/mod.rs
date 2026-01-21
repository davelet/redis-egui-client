use crate::ui::icon::load_icon;
use crate::ui::window::panels::render_error_panel;
use crate::ui::window::RedisApp;
use e_client_config::config::Config;
use e_client_config::constants::APP_NAME;

pub mod font;
pub mod icon;
pub mod window;

pub(crate) fn start_app() -> eframe::Result {
    // Load configuration
    let config = Config::load();
    if config.is_err() {
        let err = config.err().unwrap();
        return render_error_panel(err);
    }

    let config = config.unwrap();
    let width = config.window.width.max(100.0); // Minimum width 100
    let height = config.window.height.max(100.0); // Minimum height 100
    let mut viewport = egui::ViewportBuilder::default()
        .with_icon(load_icon())
        .with_title(APP_NAME);

    // Set window size and position
    if !config.window.maximized {
        viewport = viewport
            .with_inner_size([width, height])
            .with_position(egui::Pos2::new(
                config.window.x.max(0.0), // Ensure position is not negative
                config.window.y.max(0.0),
            ));
    } else {
        viewport = viewport.with_maximized(true);
    }

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,                      // Enable vertical sync
        multisampling: 0,                 // Disable multisampling
        renderer: eframe::Renderer::Wgpu, // Explicitly use wgpu renderer
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            // set chinese font todo - support more chinese fonts
            font::configure_fonts(&cc.egui_ctx);
            // Set language
            if !config.settings.language.is_empty() {
                let mut style = (*cc.egui_ctx.style()).clone();
                style.visuals = egui::style::Visuals::light(); // todo - support dark style
                cc.egui_ctx.set_style(style);
            }

            Ok(Box::new(RedisApp::with_config(config)))
        }),
    )
}
