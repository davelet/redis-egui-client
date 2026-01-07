use tokio::runtime::Runtime;
use crate::ui::start_app;

mod ui;
mod core;

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

   start_app()
}
