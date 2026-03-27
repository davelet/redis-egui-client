use serde::{Deserialize, Serialize};

/// Service name for keyring entries
const KEYRING_SERVICE: &str = "com.e-client.ai";

/// Default system prompt for AI Redis assistant
pub const DEFAULT_SYSTEM_PROMPT: &str = "You are helping with Redis database operations. \
Always respond with valid Redis commands that can be executed directly. \
Only output the Redis command without any explanation or markdown formatting.";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiProviderType {
    // Major providers
    OpenAi,
    Anthropic,
    Meta,
    Mistral,
    Cohere,

    // Open source / Self-hosted
    Ollama,
    LMStudio,
    LocalAI,
    Vllm,

    // API aggregators
    OpenRouter,
    Together,
    Replicate,
    Huggingface,

    // Specialized
    Groq,
    Perplexity,
    Gemini,
    Grok,

    // Chinese providers
    Qwen,
    Baichuan,
    Doubao,
    Moonshot,
    Zhipu,
    Minimax,

    // Custom
    Custom,
}

impl Default for AiProviderType {
    fn default() -> Self {
        AiProviderType::OpenAi
    }
}

impl std::fmt::Display for AiProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiProviderType::OpenAi => write!(f, "OpenAI"),
            AiProviderType::Anthropic => write!(f, "Anthropic"),
            AiProviderType::Meta => write!(f, "Meta (Llama)"),
            AiProviderType::Mistral => write!(f, "Mistral"),
            AiProviderType::Cohere => write!(f, "Cohere"),
            AiProviderType::Ollama => write!(f, "Ollama"),
            AiProviderType::LMStudio => write!(f, "LM Studio"),
            AiProviderType::LocalAI => write!(f, "LocalAI"),
            AiProviderType::Vllm => write!(f, "vLLM"),
            AiProviderType::OpenRouter => write!(f, "OpenRouter"),
            AiProviderType::Together => write!(f, "Together"),
            AiProviderType::Replicate => write!(f, "Replicate"),
            AiProviderType::Huggingface => write!(f, "Hugging Face"),
            AiProviderType::Groq => write!(f, "Groq"),
            AiProviderType::Perplexity => write!(f, "Perplexity"),
            AiProviderType::Gemini => write!(f, "Gemini"),
            AiProviderType::Grok => write!(f, "Grok"),
            AiProviderType::Qwen => write!(f, "Qwen (阿里)"),
            AiProviderType::Baichuan => write!(f, "Baichuan (百川)"),
            AiProviderType::Doubao => write!(f, "Doubao (豆包)"),
            AiProviderType::Moonshot => write!(f, "Moonshot (月之暗面)"),
            AiProviderType::Zhipu => write!(f, "Zhipu (智谱)"),
            AiProviderType::Minimax => write!(f, "Minimax"),
            AiProviderType::Custom => write!(f, "Custom"),
        }
    }
}

impl AiProviderType {
    pub fn default_url(&self) -> Option<String> {
        match self {
            AiProviderType::OpenAi => Some("https://api.openai.com/v1".to_string()),
            AiProviderType::Anthropic => Some("https://api.anthropic.com".to_string()),
            AiProviderType::Mistral => Some("https://api.mistral.ai/v1".to_string()),
            AiProviderType::Cohere => Some("https://api.cohere.ai/v1".to_string()),
            AiProviderType::Ollama => Some("http://localhost:11434/v1".to_string()),
            AiProviderType::LMStudio => Some("http://localhost:1234/v1".to_string()),
            AiProviderType::LocalAI => Some("http://localhost:8080/v1".to_string()),
            AiProviderType::Vllm => Some("http://localhost:8000/v1".to_string()),
            AiProviderType::OpenRouter => Some("https://openrouter.ai/api/v1".to_string()),
            AiProviderType::Together => Some("https://api.together.xyz/v1".to_string()),
            AiProviderType::Replicate => Some("https://api.replicate.com/v1".to_string()),
            AiProviderType::Huggingface => {
                Some("https://api-inference.huggingface.co/v1".to_string())
            }
            AiProviderType::Groq => Some("https://api.groq.com/openai/v1".to_string()),
            AiProviderType::Perplexity => Some("https://api.perplexity.ai".to_string()),
            AiProviderType::Gemini => {
                Some("https://generativelanguage.googleapis.com/v1beta/openai/".to_string())
            }
            AiProviderType::Grok => Some("https://api.x.ai/v1".to_string()),
            AiProviderType::Qwen => Some("https://dashscope.aliyuncs.com/api/v1".to_string()),
            AiProviderType::Baichuan => Some("https://api.baichuan-ai.com/v1".to_string()),
            AiProviderType::Doubao => Some("https://ark.cn-beijing.volces.com/api/v3".to_string()),
            AiProviderType::Moonshot => Some("https://api.moonshot.cn/v1".to_string()),
            AiProviderType::Zhipu => Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
            AiProviderType::Minimax => Some("https://api.minimax.chat/v1".to_string()),
            AiProviderType::Meta => None,
            AiProviderType::Custom => None,
        }
    }

    pub fn all_providers() -> Vec<AiProviderType> {
        vec![
            AiProviderType::Anthropic,
            AiProviderType::Baichuan,
            AiProviderType::Cohere,
            AiProviderType::Custom,
            AiProviderType::Doubao,
            AiProviderType::Gemini,
            AiProviderType::Grok,
            AiProviderType::Groq,
            AiProviderType::Huggingface,
            AiProviderType::LMStudio,
            AiProviderType::LocalAI,
            AiProviderType::Meta,
            AiProviderType::Minimax,
            AiProviderType::Mistral,
            AiProviderType::Moonshot,
            AiProviderType::Ollama,
            AiProviderType::OpenAi,
            AiProviderType::OpenRouter,
            AiProviderType::Perplexity,
            AiProviderType::Qwen,
            AiProviderType::Replicate,
            AiProviderType::Together,
            AiProviderType::Vllm,
            AiProviderType::Zhipu,
        ]
    }

    pub fn is_placeholder(&self) -> bool {
        false
    }
}

/// Unified AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AiProvider {
    OpenAi {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Ollama {
        endpoint: String,
        model: String,
    },
    Anthropic {
        api_key: String,
        model: String,
    },
    OpenRouter {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Custom {
        api_key: Option<String>,
        base_url: String,
        model: String,
    },
}

impl AiProvider {
    pub fn provider_type(&self) -> AiProviderType {
        match self {
            AiProvider::OpenAi { .. } => AiProviderType::OpenAi,
            AiProvider::Ollama { .. } => AiProviderType::Ollama,
            AiProvider::Anthropic { .. } => AiProviderType::Anthropic,
            AiProvider::OpenRouter { .. } => AiProviderType::OpenRouter,
            AiProvider::Custom { .. } => AiProviderType::Custom,
        }
    }

    pub fn get_base_url(&self) -> String {
        match self {
            AiProvider::OpenAi { base_url, .. } => base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            AiProvider::Ollama { endpoint, .. } => endpoint.clone(),
            AiProvider::Anthropic { .. } => "https://api.anthropic.com/v1".to_string(),
            AiProvider::OpenRouter { base_url, .. } => base_url
                .clone()
                .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string()),
            AiProvider::Custom { base_url, .. } => base_url.clone(),
        }
    }

    pub fn get_model(&self) -> String {
        match self {
            AiProvider::OpenAi { model, .. } => model.clone(),
            AiProvider::Ollama { model, .. } => model.clone(),
            AiProvider::Anthropic { model, .. } => model.clone(),
            AiProvider::OpenRouter { model, .. } => model.clone(),
            AiProvider::Custom { model, .. } => model.clone(),
        }
    }

    pub fn get_api_key(&self) -> Option<String> {
        match self {
            AiProvider::OpenAi { api_key, .. } => Some(api_key.clone()),
            AiProvider::Anthropic { api_key, .. } => Some(api_key.clone()),
            AiProvider::OpenRouter { api_key, .. } => Some(api_key.clone()),
            AiProvider::Custom { api_key, .. } => api_key.clone(),
            AiProvider::Ollama { .. } => None,
        }
    }

    pub fn requires_api_key(&self) -> bool {
        !matches!(self, AiProvider::Ollama { .. })
    }
}

/// Custom AI model configuration with provider support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModel {
    pub id: String,
    pub name: String,
    pub provider: AiProviderType,
    #[serde(flatten)]
    pub provider_config: AiProvider,
    /// Optional override for base_url (not serialized, use provider_config.base_url)
    #[serde(skip)]
    pub base_url: Option<String>,
    /// Primary model ID (kept in sync with provider_config.model)
    pub model_id: String,
    pub temperature: f32,
    #[serde(skip)]
    pub api_key: Option<String>,
    #[serde(skip)]
    pub use_env_key: bool,
    #[serde(skip)]
    pub env_key_name: Option<String>,
}

impl Default for AiModel {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            provider: AiProviderType::OpenAi,
            provider_config: AiProvider::OpenAi {
                api_key: String::new(),
                base_url: None,
                model: String::new(),
            },
            base_url: None,
            model_id: String::new(),
            temperature: 0.7,
            api_key: None,
            use_env_key: false,
            env_key_name: None,
        }
    }
}

impl AiModel {
    pub fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{:x}-{:x}", duration.as_secs(), duration.subsec_nanos())
    }

    pub fn keyring_key(&self) -> String {
        format!("api_key_{}", self.id)
    }

    /// Load API key from system keyring or environment variable
    pub fn load_api_key(&mut self) -> Result<(), AiConfigError> {
        if self.use_env_key {
            let env_key = self.env_key_name.as_deref().unwrap_or("REDIS_AI_API_KEY");
            if let Ok(key) = std::env::var(env_key) {
                self.api_key = Some(key);
                return Ok(());
            }
        }

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
            Err(e) => Err(AiConfigError::KeyringError(e.to_string())),
        }
    }

    /// Save API key to system keyring
    pub fn save_api_key(&self) -> Result<(), AiConfigError> {
        if self.use_env_key {
            return Ok(());
        }

        if let Some(ref key) = self.api_key {
            let entry = keyring::Entry::new(KEYRING_SERVICE, &self.keyring_key())?;
            entry
                .set_password(key)
                .map_err(|e| AiConfigError::KeyringError(e.to_string()))?;
        }
        Ok(())
    }

    /// Delete API key from system keyring
    pub fn delete_api_key(&self) -> Result<(), AiConfigError> {
        if self.use_env_key {
            return Ok(());
        }

        let entry = keyring::Entry::new(KEYRING_SERVICE, &self.keyring_key())?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AiConfigError::KeyringError(e.to_string())),
        }
    }

    /// Get the effective base URL for this model
    pub fn get_base_url(&self) -> String {
        if let Some(url) = &self.base_url {
            return url.clone();
        }
        self.provider_config.get_base_url()
    }

    pub fn get_model_id(&self) -> String {
        self.model_id.clone()
    }

    /// Backward compatibility helper - use get_base_url() instead
    pub fn url(&self) -> String {
        self.get_base_url()
    }

    pub fn new_model(
        provider: AiProviderType,
        name: String,
        base_url: Option<String>,
        model_id: String,
    ) -> Self {
        let (default_url, provider_config) = match provider {
            AiProviderType::OpenAi => (
                Some("https://api.openai.com/v1".to_string()),
                AiProvider::OpenAi {
                    api_key: String::new(),
                    base_url: base_url.clone(),
                    model: model_id.clone(),
                },
            ),
            AiProviderType::Anthropic => (
                Some("https://api.anthropic.com/v1".to_string()),
                AiProvider::Anthropic {
                    api_key: String::new(),
                    model: model_id.clone(),
                },
            ),
            AiProviderType::Ollama => (
                Some("http://localhost:11434/v1".to_string()),
                AiProvider::Ollama {
                    endpoint: base_url
                        .clone()
                        .unwrap_or_else(|| "http://localhost:11434/v1".to_string()),
                    model: model_id.clone(),
                },
            ),
            AiProviderType::OpenRouter => (
                Some("https://openrouter.ai/api/v1".to_string()),
                AiProvider::OpenRouter {
                    api_key: String::new(),
                    base_url: base_url.clone(),
                    model: model_id.clone(),
                },
            ),
            // All other providers use Custom config
            _ => (
                None,
                AiProvider::Custom {
                    api_key: None,
                    base_url: base_url.clone().unwrap_or_default(),
                    model: model_id.clone(),
                },
            ),
        };

        let final_base_url = base_url.or(default_url);

        Self {
            id: Self::generate_id(),
            name,
            provider,
            provider_config,
            base_url: final_base_url,
            model_id,
            temperature: 0.7,
            api_key: None,
            use_env_key: false,
            env_key_name: None,
        }
    }
}

#[derive(Debug)]
pub enum AiConfigError {
    KeyringError(String),
    EnvVarError(String),
}

impl std::fmt::Display for AiConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiConfigError::KeyringError(msg) => write!(f, "Keyring error: {}", msg),
            AiConfigError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
        }
    }
}

impl std::error::Error for AiConfigError {}

impl From<keyring::Error> for AiConfigError {
    fn from(e: keyring::Error) -> Self {
        AiConfigError::KeyringError(e.to_string())
    }
}

// Re-export for convenience
pub use self::AiConfigError as Error;

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
        if let Some(model) = self.models.iter().find(|m| m.id == id) {
            let _ = model.delete_api_key();
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

    pub fn get_model(&self, id: &str) -> Option<&AiModel> {
        self.models.iter().find(|m| m.id == id)
    }

    pub fn get_model_mut(&mut self, id: &str) -> Option<&mut AiModel> {
        self.models.iter_mut().find(|m| m.id == id)
    }

    pub fn load_api_keys(&mut self) {
        for model in &mut self.models {
            if let Err(e) = model.load_api_key() {
                eprintln!("Failed to load API key for model {}: {}", model.name, e);
            }
        }
    }

    pub fn save_api_keys(&self) {
        for model in &self.models {
            if let Err(e) = model.save_api_key() {
                eprintln!("Failed to save API key for model {}: {}", model.name, e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AiModel Tests ====================

    #[test]
    fn test_ai_model_generate_id_unique() {
        let id1 = AiModel::generate_id();
        let id2 = AiModel::generate_id();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_ai_model_generate_id_format() {
        let id = AiModel::generate_id();
        // Should be in format: {timestamp}-{nanoseconds}
        assert!(id.contains("-"), "ID should contain hyphen separator");
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 2, "ID should have exactly 2 parts");
    }

    #[test]
    fn test_ai_model_new() {
        let model = AiModel {
            id: "test-id".to_string(),
            name: "Test Model".to_string(),
            provider: AiProviderType::OpenAi,
            provider_config: AiProvider::OpenAi {
                api_key: "sk-test".to_string(),
                base_url: Some("https://api.test.com".to_string()),
                model: "gpt-4".to_string(),
            },
            base_url: Some("https://api.test.com".to_string()),
            model_id: "gpt-4".to_string(),
            temperature: 0.7,
            api_key: Some("sk-test".to_string()),
            use_env_key: false,
            env_key_name: None,
        };

        assert_eq!(model.name, "Test Model");
        assert_eq!(model.get_base_url(), "https://api.test.com");
        assert_eq!(model.model_id, "gpt-4");
        assert_eq!(model.temperature, 0.7);
        assert!(model.api_key.is_some());
    }

    // ==================== AiConfig Tests ====================

    /// Helper function to create a test model
    fn create_test_model(id: &str, name: &str) -> AiModel {
        AiModel {
            id: id.to_string(),
            name: name.to_string(),
            provider: AiProviderType::OpenAi,
            provider_config: AiProvider::OpenAi {
                api_key: String::new(),
                base_url: Some("https://api.test.com".to_string()),
                model: "gpt-4".to_string(),
            },
            base_url: Some("https://api.test.com".to_string()),
            model_id: "gpt-4".to_string(),
            temperature: 0.7,
            api_key: None,
            use_env_key: false,
            env_key_name: None,
        }
    }

    #[test]
    fn test_ai_config_default() {
        let config = AiConfig::default();

        assert!(config.models.is_empty());
        assert!(config.active_model_id.is_none());
        assert!(config.enabled);
        assert!(config.confirm_before_execute);
        assert!(config.show_ai_thinking);
        assert_eq!(config.system_prompt, DEFAULT_SYSTEM_PROMPT);
    }

    #[test]
    fn test_ai_config_new() {
        let config = AiConfig::new();
        // new() should return default config
        assert!(config.models.is_empty());
        assert!(config.enabled);
    }

    #[test]
    fn test_ai_config_add_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model.clone());

        assert_eq!(config.models.len(), 1);
        assert_eq!(config.models[0].name, "Model 1");
    }

    #[test]
    fn test_ai_config_remove_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        assert_eq!(config.models.len(), 1);

        config.remove_model("model-1");
        assert!(config.models.is_empty());
    }

    #[test]
    fn test_ai_config_remove_active_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        config.set_active_model("model-1");
        assert_eq!(config.active_model_id, Some("model-1".to_string()));

        config.remove_model("model-1");
        // Should auto-select first model if available, or None
        assert!(config.active_model_id.is_none());
    }

    #[test]
    fn test_ai_config_get_active_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        assert!(config.get_active_model().is_none());

        config.set_active_model("model-1");
        assert!(config.get_active_model().is_some());
        assert_eq!(config.get_active_model().unwrap().name, "Model 1");
    }

    #[test]
    fn test_ai_config_set_active_model_invalid_id() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        config.set_active_model("non-existent");

        // Should not change active model
        assert!(config.active_model_id.is_none());
    }

    #[test]
    fn test_ai_config_update_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        assert_eq!(config.models[0].temperature, 0.7);

        let mut updated_model = create_test_model("model-1", "Model 1 Updated");
        updated_model.temperature = 0.9;

        config.update_model(updated_model);
        assert_eq!(config.models[0].name, "Model 1 Updated");
        assert_eq!(config.models[0].temperature, 0.9);
    }

    #[test]
    fn test_ai_config_get_model() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);

        let found = config.get_model("model-1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Model 1");

        let not_found = config.get_model("non-existent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_ai_config_get_active_model_mut() {
        let mut config = AiConfig::default();
        let model = create_test_model("model-1", "Model 1");

        config.add_model(model);
        config.set_active_model("model-1");

        let found = config.get_active_model_mut();
        assert!(found.is_some());

        // Modify through mutable reference
        if let Some(m) = found {
            m.temperature = 1.0;
        }

        assert_eq!(config.models[0].temperature, 1.0);
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_ai_config_serialization() {
        let config = AiConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AiConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.enabled, config.enabled);
        assert_eq!(
            deserialized.confirm_before_execute,
            config.confirm_before_execute
        );
        assert_eq!(deserialized.show_ai_thinking, config.show_ai_thinking);
        assert_eq!(deserialized.system_prompt, config.system_prompt);
    }

    #[test]
    fn test_ai_config_serialization_with_models() {
        let mut config = AiConfig::default();
        config.enabled = false;
        config.confirm_before_execute = false;
        config.system_prompt = "Custom prompt".to_string();

        // Use new_model constructor to avoid base_url duplication
        let model = AiModel::new_model(
            AiProviderType::OpenAi,
            "Model 1".to_string(),
            Some("https://api.test.com".to_string()),
            "gpt-4".to_string(),
        );

        let model_id = model.id.clone();
        config.add_model(model);
        config.set_active_model(&model_id);

        let json = serde_json::to_string_pretty(&config).unwrap();

        // Check that test API key is not exposed (provider_config.api_key is empty string)
        assert!(!json.contains("sk-"));

        let deserialized: AiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.enabled, false);
        assert_eq!(deserialized.confirm_before_execute, false);
        assert_eq!(deserialized.system_prompt, "Custom prompt");
        assert_eq!(deserialized.models.len(), 1);
        assert_eq!(deserialized.models[0].name, "Model 1");
    }

    // ==================== Default System Prompt Tests ====================

    #[test]
    fn test_default_system_prompt_not_empty() {
        assert!(!DEFAULT_SYSTEM_PROMPT.is_empty());
    }

    #[test]
    fn test_default_system_prompt_contains_redis_hint() {
        let prompt = DEFAULT_SYSTEM_PROMPT.to_lowercase();
        assert!(
            prompt.contains("redis"),
            "System prompt should mention Redis"
        );
    }

    #[test]
    fn test_default_system_prompt_contains_command_hint() {
        let prompt = DEFAULT_SYSTEM_PROMPT.to_lowercase();
        assert!(
            prompt.contains("command"),
            "System prompt should mention commands"
        );
    }
}
