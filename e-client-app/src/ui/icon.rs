use eframe::egui::IconData;

pub fn load_icon() -> IconData {
    let bytes = include_bytes!("../../../assets/icon-1024.png");

    let image = image::load_from_memory(bytes).unwrap_or_else(|e| {
        panic!("Failed to load icon {}", e);
    });

    let image = image.into_rgba8();
    let (width, height) = image.dimensions();

    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
