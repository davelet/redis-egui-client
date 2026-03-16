use serde::{Deserialize, Serialize};

/// Custom AI model configuration
/// All providers are treated the same - user provides the API endpoint URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModel {
    /// Unique identifier for this model config
    pub id: String,
    /// Display name for this model config (e.g., "My GPT-4")
    pub name: String,
    /// The base URL for the AI API (e.g., "https://api.openai.com/v1" or "http://localhost:11434")
    pub url: String,
    /// API Key for authentication
    pub api_key: Option<String>,
    /// The model ID to use (e.g., "gpt-4o", "claude-3-sonnet-20240229", "llama2")
    pub model_id: String,
    /// Sampling temperature (0.0 - 2.0)
    pub temperature: f32,
}

impl AiModel {
    /// Generate a simple unique ID based on timestamp
    pub fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{:x}-{:x}", duration.as_secs(), duration.subsec_nanos())
    }
}

/// AI configuration for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// List of configured AI models
    pub models: Vec<AiModel>,
    /// ID of the currently active model
    pub active_model_id: Option<String>,
    /// Whether AI features are enabled globally
    pub enabled: bool,
    /// Whether to show confirmation dialog before AI executes commands
    pub confirm_before_execute: bool,
    /// Whether to show AI thinking process
    pub show_ai_thinking: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            active_model_id: None,
            enabled: true,
            confirm_before_execute: true,
            show_ai_thinking: true,
        }
    }
}

impl AiConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_model(&mut self, model: AiModel) {
        self.models.push(model);
    }

    pub fn remove_model(&mut self, id: &str) {
        self.models.retain(|m| m.id != id);
        if self.active_model_id.as_deref() == Some(id) {
            self.active_model_id = self.models.first().map(|m| m.id.clone());
        }
    }

    pub fn get_active_model(&self) -> Option<&AiModel> {
        self.active_model_id
            .as_ref()
            .and_then(|id| self.models.iter().find(|m| &m.id == id))
    }

    pub fn get_active_model_mut(&mut self) -> Option<&mut AiModel> {
        if let Some(ref id) = self.active_model_id {
            self.models.iter_mut().find(|m| &m.id == id)
        } else {
            None
        }
    }

    pub fn set_active_model(&mut self, id: &str) {
        if self.models.iter().any(|m| m.id == id) {
            self.active_model_id = Some(id.to_string());
        }
    }

    pub fn update_model(&mut self, model: AiModel) {
        if let Some(existing) = self.models.iter_mut().find(|m| m.id == model.id) {
            *existing = model;
        }
    }

    /// Get a model by ID
    pub fn get_model(&self, id: &str) -> Option<&AiModel> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Get a mutable model by ID
    pub fn get_model_mut(&mut self, id: &str) -> Option<&mut AiModel> {
        self.models.iter_mut().find(|m| m.id == id)
    }
}
