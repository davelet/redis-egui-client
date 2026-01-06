mod app_state;
mod config;
mod constants;
mod icon;
mod redis_client;
mod translations;
mod ui;

use eframe::egui;
use tokio::runtime::Runtime;
use constants::APP_NAME;
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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([crate::constants::WINDOW_WIDTH, crate::constants::WINDOW_HEIGHT])
            .with_icon(icon::load_icon())
            .with_title(APP_NAME),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(RedisApp::default()))),
    )
}