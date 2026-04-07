use std::time::{Duration, Instant};

use egui::{Align2, Color32, RichText, Vec2};

/// A simple toast notification
#[derive(Clone, Debug)]
pub struct Toast {
    pub owner: String,
    pub message: String,
    pub toast_type: ToastType,
    pub elapsed: Duration,
    pub duration: Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
    Info,
    Warning,
    Error,
    Success,
}

impl ToastType {
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Info => "ℹ",
            ToastType::Warning => "⚠",
            ToastType::Error => "✖",
            ToastType::Success => "✔",
        }
    }

    pub fn accent_color(&self) -> Color32 {
        match self {
            ToastType::Info => Color32::from_rgb(0, 150, 255),
            ToastType::Warning => Color32::from_rgb(255, 165, 0),
            ToastType::Error => Color32::from_rgb(255, 60, 60),
            ToastType::Success => Color32::from_rgb(0, 200, 80),
        }
    }
}

impl Toast {
    pub fn new(owner: String, message: String, toast_type: ToastType) -> Self {
        Self {
            owner,
            message,
            toast_type,
            elapsed: Duration::ZERO,
            duration: Duration::from_secs(10),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.elapsed >= self.duration
    }
}

/// Toast manager for displaying notifications
pub struct ToastManager {
    toasts: Vec<Toast>,
    last_frame_time: Instant,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            last_frame_time: Instant::now(),
        }
    }

    /// Add or replace a toast for the given owner.
    /// If a toast with the same owner already exists, it is replaced (timer resets).
    pub fn push(&mut self, owner: String, message: String, toast_type: ToastType) {
        if let Some(pos) = self.toasts.iter().position(|t| t.owner == owner) {
            self.toasts[pos] = Toast::new(owner, message, toast_type);
        } else {
            self.toasts.push(Toast::new(owner, message, toast_type));
        }
    }

    /// Show a warning toast
    pub fn warning(&mut self, owner: String, message: String) {
        self.push(owner, message, ToastType::Warning);
    }

    /// Show an error toast
    pub fn error(&mut self, owner: String, message: String) {
        self.push(owner, message, ToastType::Error);
    }

    /// Show an info toast
    pub fn info(&mut self, owner: String, message: String) {
        self.push(owner, message, ToastType::Info);
    }

    /// Show a success toast
    pub fn success(&mut self, owner: String, message: String) {
        self.push(owner, message, ToastType::Success);
    }

    /// Remove expired toasts
    pub fn cleanup(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    /// Get all active toasts
    pub fn toasts(&self) -> &[Toast] {
        &self.toasts
    }

    /// Check if there are any toasts
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    /// Count of active toasts
    pub fn len(&self) -> usize {
        self.toasts.len()
    }

    /// Render all toasts in the UI
    pub fn render(&mut self, ctx: &egui::Context) {
        // Clean up expired toasts
        self.cleanup();

        if self.toasts.is_empty() {
            return;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        let mut any_not_hovered = false;

        // Create a panel at the bottom-right corner
        egui::Area::new(egui::Id::new("toast_area"))
            .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-10.0, -10.0))
            .show(ctx, |ui| {
                // Stack toasts from bottom to top
                for toast in self.toasts.iter_mut().rev() {
                    let accent_color = toast.toast_type.accent_color();
                    let icon = toast.toast_type.icon();
                    let progress = (1.0
                        - (toast.elapsed.as_secs_f32() / toast.duration.as_secs_f32()))
                    .clamp(0.0, 1.0);

                    // Frame for each toast
                    let response = egui::Frame::window(ui.style())
                        .fill(ui.visuals().window_fill.gamma_multiply(0.95))
                        .stroke(egui::Stroke::new(1.0, accent_color.linear_multiply(0.5)))
                        .corner_radius(8.0)
                        .show(ui, |ui| {
                            ui.set_max_width(320.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(icon).color(accent_color).strong().size(16.0),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(&toast.owner)
                                        .color(accent_color)
                                        .strong()
                                        .size(12.0),
                                );
                            });
                            ui.label(
                                RichText::new(&toast.message)
                                    .color(ui.visuals().text_color())
                                    .size(13.0),
                            );

                            ui.add_space(8.0);

                            // Progress bar at bottom
                            let bar_rect = ui.available_rect_before_wrap();
                            let mut bar_rect = bar_rect;
                            bar_rect.set_height(2.0);
                            ui.painter().rect_filled(
                                bar_rect,
                                0.0,
                                ui.visuals().widgets.noninteractive.bg_fill,
                            );

                            bar_rect.set_width(bar_rect.width() * progress);
                            ui.painter().rect_filled(bar_rect, 0.0, accent_color);
                        })
                        .response;

                    if !response.hovered() {
                        toast.elapsed += dt;
                        any_not_hovered = true;
                    }
                }
            });

        // Request repaint for toast animation if any toast is not hovered
        if any_not_hovered {
            ctx.request_repaint_after(Duration::from_millis(16));
        }
    }
}
