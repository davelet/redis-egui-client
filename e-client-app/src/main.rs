use crate::ui::start_app;
use tokio::runtime::Runtime;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod core;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let runtime = Runtime::new().unwrap();
    let _guard = runtime.enter();
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    });

    start_app()
}
