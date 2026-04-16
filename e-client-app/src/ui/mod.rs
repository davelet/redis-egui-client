use crate::ui::icon::load_icon;
use crate::ui::window::RedisApp;
use crate::ui::window::panels::render_error_panel;
use e_client_config::config::{Config, Theme};
use egui_thematic::ThemeConfig;

pub mod font;
pub mod icon;
pub mod window;

pub fn theme_to_visuals(theme: Theme) -> egui::Visuals {
    if theme.is_system() {
        return detect_system_theme();
    }
    get_preset_visuals(theme.to_egui_thematic_name())
}

fn get_preset_visuals(preset: &str) -> egui::Visuals {
    match preset {
        "dark" => ThemeConfig::dark_preset().to_visuals(),
        "light" => ThemeConfig::light_preset().to_visuals(),
        "dracula" => ThemeConfig::dracula_preset().to_visuals(),
        "nord" => ThemeConfig::nord_preset().to_visuals(),
        "gruvbox_dark" => ThemeConfig::gruvbox_dark_preset().to_visuals(),
        "monokai" => ThemeConfig::monokai_preset().to_visuals(),
        "one_dark" => ThemeConfig::one_dark_preset().to_visuals(),
        "tokyo_night" => ThemeConfig::tokyo_night_preset().to_visuals(),
        "solarized_dark" => ThemeConfig::solarized_dark_preset().to_visuals(),
        "solarized_light" => ThemeConfig::solarized_light_preset().to_visuals(),
        _ => ThemeConfig::light_preset().to_visuals(),
    }
}

fn detect_system_theme() -> egui::Visuals {
    #[cfg(target_os = "macos")]
    {
        detect_macos_theme()
    }
    #[cfg(target_os = "windows")]
    {
        detect_windows_theme()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        get_preset_visuals("light")
    }
}

#[cfg(target_os = "macos")]
fn detect_macos_theme() -> egui::Visuals {
    let output = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output();

    match output {
        Ok(out) if out.stdout.starts_with(b"Dark") => ThemeConfig::dark_preset().to_visuals(),
        _ => ThemeConfig::light_preset().to_visuals(),
    }
}

#[cfg(target_os = "windows")]
fn detect_windows_theme() -> egui::Visuals {
    let output = std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("0x0") {
                ThemeConfig::dark_preset().to_visuals()
            } else {
                ThemeConfig::light_preset().to_visuals()
            }
        }
        _ => ThemeConfig::light_preset().to_visuals(),
    }
}

pub(crate) fn start_app(tokio_handle: tokio::runtime::Handle) -> eframe::Result {
    let config = Config::load();
    if config.is_err() {
        let err = config.err().unwrap();
        return render_error_panel(err);
    }

    let config = config.unwrap();
    let width = config.window.width.max(100.0);
    let height = config.window.height.max(100.0);
    let mut viewport = egui::ViewportBuilder::default()
        .with_icon(load_icon())
        .with_title(env!("CARGO_PKG_NAME"));

    if !config.window.maximized {
        viewport = viewport
            .with_inner_size([width, height])
            .with_position(egui::Pos2::new(
                config.window.x.max(0.0),
                config.window.y.max(0.0),
            ));
    } else {
        viewport = viewport.with_maximized(true);
    }

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,
        multisampling: 0,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        options,
        Box::new(|cc| {
            Ok(Box::new(RedisApp::with_config(
                config,
                tokio_handle,
                cc.egui_ctx.clone(),
            )))
        }),
    )
}
