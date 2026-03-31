use e_client_config::language::Language;
use std::time::Instant;

/// Feedback tuple: (copy_success_time, copy_failure_time, action_success_time)
type FeedbackTriple = (Option<Instant>, Option<Instant>, Option<Instant>);

pub struct CopyFeedbackManager {
    feedback: FeedbackTriple,
    last_copy_button_id: Option<egui::Id>,
    last_action_button_id: Option<egui::Id>,
}

impl Default for CopyFeedbackManager {
    fn default() -> Self {
        Self {
            feedback: (None, None, None),
            last_copy_button_id: None,
            last_action_button_id: None,
        }
    }
}

impl CopyFeedbackManager {
    pub fn update(&mut self) {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;
        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let now = std::time::Instant::now();

        if let Some(time) = self.feedback.0 {
            if now.duration_since(time) >= duration {
                self.feedback.0 = None;
            }
        }

        if let Some(time) = self.feedback.1 {
            if now.duration_since(time) >= duration {
                self.feedback.1 = None;
            }
        }

        if let Some(time) = self.feedback.2 {
            if now.duration_since(time) >= duration {
                self.feedback.2 = None;
            }
        }
    }

    pub fn record_copy_success(&mut self) {
        self.feedback.0 = Some(Instant::now());
    }

    pub fn record_copy_failure(&mut self) {
        self.feedback.1 = Some(Instant::now());
    }

    pub fn record_copy_success_with_id(&mut self, button_id: egui::Id) {
        self.feedback.0 = Some(Instant::now());
        self.last_copy_button_id = Some(button_id);
    }

    pub fn record_copy_failure_with_id(&mut self, button_id: egui::Id) {
        self.feedback.1 = Some(Instant::now());
        self.last_copy_button_id = Some(button_id);
    }

    /// Record an action success (e.g. shortcut reset) for a given button id.
    pub fn record_action_success(&mut self, button_id: egui::Id) {
        self.feedback.2 = Some(Instant::now());
        self.last_action_button_id = Some(button_id);
    }

    pub fn copy_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;

        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (success_time, failure_time, _) = self.feedback;
        let now = Instant::now();

        if self.last_copy_button_id == Some(button_id) {
            if let Some(time) = success_time {
                if now.duration_since(time) < duration {
                    return egui::Color32::from_rgb(50, 200, 50); // Green
                }
            }

            if let Some(time) = failure_time {
                if now.duration_since(time) < duration {
                    return egui::Color32::from_rgb(220, 50, 50); // Red
                }
            }
        }

        egui::Color32::BLACK
    }

    /// Returns the button text, replacing it with a feedback label if the button
    /// recently received a copy or action success.
    pub fn copy_button_text(
        &self,
        button_id: egui::Id,
        original_text: &str,
        global_language: Language,
    ) -> String {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use e_client_config::translations::{tr, TranslationKey};
        use std::time::Duration;

        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (success_time, failure_time, action_time) = self.feedback;
        let now = Instant::now();

        // Check copy feedback
        if self.last_copy_button_id == Some(button_id) {
            if let Some(time) = success_time {
                if now.duration_since(time) < duration {
                    return tr(TranslationKey::CopySuccess, global_language).to_string();
                }
            }

            if let Some(time) = failure_time {
                if now.duration_since(time) < duration {
                    return tr(TranslationKey::CopyFailed, global_language).to_string();
                }
            }
        }

        // Check action feedback (e.g. "Restored")
        if self.last_action_button_id == Some(button_id) {
            if let Some(time) = action_time {
                if now.duration_since(time) < duration {
                    return tr(TranslationKey::ShortcutRestored, global_language).to_string();
                }
            }
        }

        original_text.to_string()
    }

    /// Returns the button text color, applying green for action success feedback.
    pub fn action_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;

        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (_, _, action_time) = self.feedback;
        let now = Instant::now();

        if self.last_action_button_id == Some(button_id) {
            if let Some(time) = action_time {
                if now.duration_since(time) < duration {
                    return egui::Color32::from_rgb(50, 200, 50); // Green
                }
            }
        }

        egui::Color32::BLACK
    }
}
