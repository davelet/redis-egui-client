use crate::ui::icon::load_icon;
use crate::ui::window::RedisApp;
use crate::ui::window::panels::render_error_panel;
use e_client_config::config::{Config, Theme};
use e_client_config::constants::APP_NAME;

pub mod font;
pub mod icon;
pub mod window;

/// Convert Theme to egui Visuals
pub fn theme_to_visuals(theme: Theme) -> egui::Visuals {
    match theme {
        Theme::Light => egui::Visuals::light(),
        Theme::Dark => egui::Visuals::dark(),
        Theme::System => detect_system_theme(),
    }
}

/// Detect system theme preference
fn detect_system_theme() -> egui::Visuals {
    #[cfg(target_os = "macos")]
    {
        return detect_macos_theme();
    }
    #[cfg(target_os = "windows")]
    {
        return detect_windows_theme();
    }
    egui::Visuals::light()
}

#[cfg(target_os = "macos")]
fn detect_macos_theme() -> egui::Visuals {
    let output = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output();

    match output {
        Ok(out) if out.stdout.starts_with(b"Dark") => egui::Visuals::dark(),
        _ => egui::Visuals::light(),
    }
}

#[cfg(target_os = "windows")]
fn detect_windows_theme() -> egui::Visuals {
    let output = std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme"
        ])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            // AppsUseLightTheme = 0 means dark, = 1 means light
            if stdout.contains("0x0") {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            }
        }
        _ => egui::Visuals::light(),
    }
}

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
            // Configure fonts for better Chinese and English display
            font::setup_chinese_fonts(&cc.egui_ctx)?;

            Ok(Box::new(RedisApp::with_config(config)))
        }),
    )
}
