use crate::ui::window::RedisApp;
use e_client_config::translations::{TranslationKey, tr};

/// Render the log viewer panel inside the command line panel.
/// Shows live logs while AI chat is active.
pub fn render_log_viewer(ui: &mut egui::Ui, app: &mut RedisApp, tab_idx: usize) {
    // Get language before mutable borrow
    let current_lang = app.poll_language(app.tabs[tab_idx].state.language.clone());

    let panel = &mut app.tabs[tab_idx].command_line_panel;
    let log_viewer = &mut panel.log_viewer;

    // Drain newly arrived log lines from the background task
    log_viewer.drain_received();

    // ── Top control bar ───────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(tr(TranslationKey::LiveLogs, current_lang))
                .small()
                .color(egui::Color32::GRAY),
        );

        // Clear button
        if ui
            .button(tr(TranslationKey::LiveLogsClear, current_lang))
            .on_hover_text(tr(TranslationKey::LiveLogsClearHint, current_lang))
            .clicked()
        {
            log_viewer.clear();
        }

        // Follow tail toggle
        let follow_label = if log_viewer.follow_tail {
            tr(TranslationKey::LiveLogsFollowActive, current_lang)
        } else {
            tr(TranslationKey::LiveLogsFollow, current_lang)
        };
        ui.toggle_value(&mut log_viewer.follow_tail, follow_label);

        // Status indicator — right-aligned so it never overflows the panel.
        // Use right_to_left instead of allocate_space + label, which used to
        // push the label off-screen by consuming all remaining width first.
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (text, color) = if log_viewer.is_capturing() {
                (
                    tr(TranslationKey::LiveLogsRecording, current_lang),
                    egui::Color32::from_rgb(80, 200, 80),
                )
            } else {
                (
                    tr(TranslationKey::LiveLogsIdle, current_lang),
                    egui::Color32::GRAY,
                )
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
                egui::RichText::new(tr(TranslationKey::LiveLogsEmpty, current_lang))
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
        .max_height(200.0)
        .show(ui, |ui| {
            for line in &log_viewer.log_lines {
                let color = color_by_level(line);
                ui.label(egui::RichText::new(line).monospace().small().color(color));
            }
        });
}

/// Choose display color based on actual log level (matches level position in tracing format).
fn color_by_level(line: &str) -> egui::Color32 {
    // Match exact level markers with spaces around to avoid matching keywords in log content
    if line.contains(" ERROR ") {
        egui::Color32::from_rgb(230, 50, 50)
    } else if line.contains(" WARN ") {
        egui::Color32::from_rgb(220, 160, 30)
    } else if line.contains(" DEBUG ") || line.contains(" TRACE ") {
        egui::Color32::from_rgb(140, 140, 140)
    } else if line.contains(" INFO ") {
        egui::Color32::from_rgb(60, 130, 220)
    } else {
        egui::Color32::from_rgb(80, 80, 80)
    }
}
