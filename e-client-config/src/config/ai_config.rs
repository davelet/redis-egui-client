use serde::{Deserialize, Serialize};

/// AI interaction mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[must_use = "AiMode determines whether the AI has access to tools and maintains conversation state"]
pub enum AiMode {
    /// Stateless chat mode - no conversation context, no Redis tools.
    /// Uses a specialized prompt for direct command translation.
    Chat,
    /// Stateful agent mode - full conversation context, has access to Redis tools.
    /// Requires LLMs with robust tool-calling/reasoning capabilities.
    #[default]
    Agent,
}

impl std::fmt::Display for AiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiMode::Chat => write!(f, "Chat"),
            AiMode::Agent => write!(f, "Agent"),
        }
    }
}

/// Service name for keyring entries
const KEYRING_SERVICE: &str = "com.e-client.ai";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, strum::EnumString, strum::EnumIter)]
#[strum(serialize_all = "PascalCase")]
#[derive(Default)]
pub enum AiProviderType {
    // Major providers
    #[default]
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
        use strum::IntoEnumIterator;
        AiProviderType::iter().collect()
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
    /// Optional override for base_url
    pub base_url: Option<String>,
    /// Primary model ID (kept in sync with provider_config.model)
    pub model_id: String,
    pub temperature: f32,
    #[serde(skip)]
    pub api_key: Option<String>,
}

impl Default for AiModel {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            provider: AiProviderType::OpenAi,
            base_url: None,
            model_id: String::new(),
            temperature: 0.7,
            api_key: None,
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
        String::new()
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
        Self {
            id: Self::generate_id(),
            name,
            provider,
            base_url,
            model_id,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum AiConfigError {
    KeyringError(String),
    EnvVarError(String),
    SerializationError(String),
    DeserializationError(String),
    IoError(String),
}

impl std::fmt::Display for AiConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiConfigError::KeyringError(msg) => write!(f, "Keyring error: {}", msg),
            AiConfigError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
            AiConfigError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AiConfigError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            AiConfigError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for AiConfigError {}

impl From<std::io::Error> for AiConfigError {
    fn from(e: std::io::Error) -> Self {
        AiConfigError::IoError(e.to_string())
    }
}

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
    /// Maximum tool-call turns per request
    #[serde(default = "AiConfig::default_max_turns")]
    pub max_turns: u32,
    /// Whether to render CLI chat output as markdown (default: true)
    #[serde(default = "AiConfig::default_render_markdown")]
    pub render_markdown: bool,
    /// User-defined custom command whitelist for raw command execution
    #[serde(default)]
    pub custom_command_whitelist: Vec<String>,
}

/// Built-in whitelist commands (cannot be deleted by user)
pub const BUILTIN_WHITELIST_COMMANDS: &[&str] = &[
    // General query commands
    "SCAN",
    "TYPE",
    "TTL",
    "PTTL",
    "DBSIZE",
    "EXISTS",
    "INFO",
    // String operations
    "GET",
    "SET",
    "DEL",
    "EXPIRE",
    "PERSIST",
    // Hash operations
    "HGET",
    "HGETALL",
    "HMGET",
    "HLEN",
    "HSET",
    "HDEL",
    // List operations
    "LRANGE",
    "LLEN",
    "LINDEX",
    "LSET",
    "RPUSH",
    // Set operations
    "SMEMBERS",
    "SCARD",
    "SADD",
    "SREM",
    "SISMEMBER",
    // Sorted Set operations
    "ZRANGE",
    "ZREVRANGE",
    "ZCARD",
    "ZSCORE",
    "ZADD",
    "ZREM",
    // Database operations
    "SELECT",
    "RENAMENX",
];

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            active_model_id: None,
            enabled: true,
            confirm_before_execute: true,
            show_ai_thinking: true,
            max_turns: 20,
            render_markdown: true,
            custom_command_whitelist: Vec::new(),
        }
    }
}

impl AiConfig {
    pub fn default_max_turns() -> u32 {
        20
    }

    pub fn default_render_markdown() -> bool {
        true
    }

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

    pub fn save_api_keys(&self) {
        for model in &self.models {
            if let Err(e) = model.save_api_key() {
                eprintln!("Failed to save API key for model {}: {}", model.name, e);
            }
        }
    }
}
