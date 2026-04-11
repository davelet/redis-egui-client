use crate::ui::window::RedisApp;

/// Render the log viewer panel inside the command line panel.
/// Shows live logs while AI chat is active.
pub fn render_log_viewer(ui: &mut egui::Ui, app: &mut RedisApp, tab_idx: usize) {
    let panel = &mut app.tabs[tab_idx].command_line_panel;
    let log_viewer = &mut panel.log_viewer;

    // Drain newly arrived log lines from the background task
    log_viewer.drain_received();

    // ── Top control bar ───────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Live Logs")
                .small()
                .color(egui::Color32::GRAY),
        );

        // Clear button
        if ui.button("🗑 Clear").on_hover_text("Clear log buffer").clicked() {
            log_viewer.clear();
        }

        // Follow tail toggle
        let follow_label = if log_viewer.follow_tail { "🔽 Follow" } else { "📜 Follow" };
        ui.toggle_value(&mut log_viewer.follow_tail, follow_label);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Status indicator
            let (text, color) = if log_viewer.is_capturing() {
                ("● Recording", egui::Color32::from_rgb(80, 200, 80))
            } else {
                ("○ Idle", egui::Color32::GRAY)
            };
            ui.label(egui::RichText::new(text).small().color(color));
        });
    });

    ui.add_space(2.0);
    ui.separator();

    let log_count = log_viewer.log_lines.len();
    if log_count == 0 {
        ui.vertical_centered(|ui| {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("No logs captured for this session")
                    .small()
                    .color(egui::Color32::GRAY),
            );
        });
        return;
    }

    // ── Log content scroll area ────────────────────────────────────────────
    egui::ScrollArea::vertical()
        .id_salt(egui::Id::new("cli_log_viewer_scroll"))
        .auto_shrink([false; 2])
        .stick_to_bottom(log_viewer.follow_tail)
        .show(ui, |ui| {
            for line in &log_viewer.log_lines {
                let (color, prefix) = color_by_level(line);
                ui.label(
                    egui::RichText::new(format!("{}{}", prefix, line))
                        .monospace()
                        .small()
                        .color(color),
                );
            }
        });
}

/// Choose display color and prefix based on log level keywords.
fn color_by_level(line: &str) -> (egui::Color32, &'static str) {
    let l = line.to_lowercase();
    if l.contains("error") || l.contains("err:") || l.contains("failed") || l.contains("panic") {
        (egui::Color32::from_rgb(230, 50, 50), "✗ ")
    } else if l.contains("warn") {
        (egui::Color32::from_rgb(220, 160, 30), "⚠ ")
    } else if l.contains("debug") || l.contains("trace") {
        (egui::Color32::from_rgb(140, 140, 140), "▸ ")
    } else if l.contains("info") {
        (egui::Color32::from_rgb(60, 130, 220), "· ")
    } else {
        (egui::Color32::from_rgb(80, 80, 80), "  ")
    }
}
