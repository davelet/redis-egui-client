mod app_state;
mod config;
mod constants;
mod icon;
mod redis_client;
mod translations;
mod ui;

use config::Config;
use constants::APP_NAME;
use eframe::egui;
use tokio::runtime::Runtime;
use ui::RedisApp;

fn main() -> Result<(), eframe::Error> {
    let runtime = Runtime::new().unwrap();
    let _guard = runtime.enter();

    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    });

    // 加载配置
    let config = Config::load().unwrap_or_default();
    let width = config.window.width.max(100.0);  // 最小宽度100
    let height = config.window.height.max(100.0); // 最小高度100
    let mut viewport = egui::ViewportBuilder::default()
        .with_icon(icon::load_icon())
        .with_title(APP_NAME);

    // 设置窗口大小和位置
    if !config.window.maximized {
         viewport = viewport
        .with_inner_size([width, height])
        .with_position(egui::Pos2::new(
            config.window.x.max(0.0),  // 确保位置不为负
            config.window.y.max(0.0),
        ));
    } else {
        viewport = viewport.with_maximized(true);
    }

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,  // 启用垂直同步
        multisampling: 0,  // 禁用多重采样
        renderer: eframe::Renderer::Wgpu,  // 明确使用wgpu渲染器
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            // 设置语言
            if !config.language.is_empty() {
                let mut style = (*cc.egui_ctx.style()).clone();
                style.visuals = egui::style::Visuals::light();
                cc.egui_ctx.set_style(style);
            }

            Ok(Box::new(RedisApp::with_config(config)))
        }),
    )
}
