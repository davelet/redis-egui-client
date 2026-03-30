use crate::ui::start_app;
use e_client_logging::{LoggingConfig, init_logging, log_app_shutdown, log_app_start};
use tokio::runtime::Runtime;

mod core;
mod ui;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging with default configuration
    let logging_config = LoggingConfig::default();
    init_logging(&logging_config).expect("Failed to initialize logging");

    log_app_start();

    let runtime = Runtime::new().unwrap();
    let _guard = runtime.enter();

    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    });

    let result = start_app();

    log_app_shutdown();
    result
}
