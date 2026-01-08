use eframe::egui;
use egui::FontFamily;

// Function to configure fonts for Chinese characters
pub(crate) fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Add the Songti.ttc font for Chinese characters
    fonts.font_data.insert("songti".to_owned(), {
        let font_data = std::fs::read("/System/Library/Fonts/Supplemental/Songti.ttc")
            .expect("Failed to read Songti font file");
        egui::FontData::from_owned(font_data).into()
    });

    // Use the Songti font for proportional text
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "songti".to_owned());

    // Use the Songti font for monospace text
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "songti".to_owned());

    ctx.set_fonts(fonts);
}
