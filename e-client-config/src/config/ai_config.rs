use serde::{Deserialize, Serialize};

/// Service name for keyring entries
const KEYRING_SERVICE: &str = "com.e-client.ai";

/// Default system prompt for AI Redis assistant
pub const DEFAULT_SYSTEM_PROMPT: &str = "You are helping with Redis database operations. \
Always respond with valid Redis commands that can be executed directly. \
Only output the Redis command without any explanation or markdown formatting.";

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
    /// API Key for authentication (not serialized - stored in system keyring)
    #[serde(skip)]
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

    /// Get the keyring entry key for this model's API key
    fn keyring_key(&self) -> String {
        format!("api_key_{}", self.id)
    }

    /// Load API key from system keyring
    pub fn load_api_key_from_keyring(&mut self) -> Result<(), keyring::Error> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, &self.keyring_key())?;
        match entry.get_password() {
            Ok(key) => {
                self.api_key = Some(key);
                Ok(())
            }
            Err(keyring::Error::NoEntry) => {
                self.api_key = None;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Save API key to system keyring
    pub fn save_api_key_to_keyring(&self) -> Result<(), keyring::Error> {
        if let Some(ref key) = self.api_key {
            let entry = keyring::Entry::new(KEYRING_SERVICE, &self.keyring_key())?;
            entry.set_password(key)?;
        }
        Ok(())
    }

    /// Delete API key from system keyring
    pub fn delete_api_key_from_keyring(&self) -> Result<(), keyring::Error> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, &self.keyring_key())?;
        entry.delete_credential()
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
    /// Custom system prompt for AI assistant
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
}

fn default_system_prompt() -> String {
    DEFAULT_SYSTEM_PROMPT.to_string()
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            active_model_id: None,
            enabled: true,
            confirm_before_execute: true,
            show_ai_thinking: true,
            system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
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
        // Delete API key from keyring before removing model
        if let Some(model) = self.models.iter().find(|m| m.id == id) {
            let _ = model.delete_api_key_from_keyring();
        }
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

    /// Load API keys from system keyring for all models
    pub fn load_api_keys_from_keyring(&mut self) {
        for model in &mut self.models {
            if let Err(e) = model.load_api_key_from_keyring() {
                eprintln!("Failed to load API key for model {}: {}", model.name, e);
            }
        }
    }

    /// Save API keys to system keyring for all models
    pub fn save_api_keys_to_keyring(&self) {
        for model in &self.models {
            if let Err(e) = model.save_api_key_to_keyring() {
                eprintln!("Failed to save API key for model {}: {}", model.name, e);
            }
        }
    }
}
