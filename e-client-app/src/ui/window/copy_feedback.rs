use e_client_config::language::Language;
use std::time::Instant;

pub struct CopyFeedbackManager {
    copy_feedback: (Option<Instant>, Option<Instant>),
    last_copy_button_id: Option<egui::Id>,
}

impl Default for CopyFeedbackManager {
    fn default() -> Self {
        Self {
            copy_feedback: (None, None),
            last_copy_button_id: None,
        }
    }
}

impl CopyFeedbackManager {
    pub fn update(&mut self) {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;
        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let now = std::time::Instant::now();

        if let Some(time) = self.copy_feedback.0 {
            if now.duration_since(time) >= duration {
                self.copy_feedback.0 = None;
            }
        }

        if let Some(time) = self.copy_feedback.1 {
            if now.duration_since(time) >= duration {
                self.copy_feedback.1 = None;
            }
        }
    }

    pub fn record_copy_success(&mut self) {
        self.copy_feedback.0 = Some(Instant::now());
    }

    pub fn record_copy_failure(&mut self) {
        self.copy_feedback.1 = Some(Instant::now());
    }

    pub fn record_copy_failure_with_id(&mut self, button_id: egui::Id) {
        self.copy_feedback.1 = Some(Instant::now());
        self.last_copy_button_id = Some(button_id);
    }

    pub fn copy_button_text_color(&self, button_id: egui::Id) -> egui::Color32 {
        use e_client_basics::constants::COPY_FEEDBACK_DURATION_MS;
        use std::time::Duration;

        let duration = Duration::from_millis(COPY_FEEDBACK_DURATION_MS);
        let (success_time, failure_time) = self.copy_feedback;
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
        let (success_time, failure_time) = self.copy_feedback;
        let now = Instant::now();

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

        original_text.to_string()
    }
}
