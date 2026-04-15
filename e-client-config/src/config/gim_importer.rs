//! GIM (Git Intelligence Message) configuration importer
//!
//! This module provides functionality to import AI model configurations from
//! GIM's configuration file (~/.gim.toml).

use std::fs;
use std::path::PathBuf;

use crate::config::ai_config::{AiModel, AiProviderType};
use crate::error::ConfigError;

/// GIM configuration directory name
const GIM_CONFIG_DIR: &str = "gim";
/// GIM configuration file name
const GIM_CONFIG_FILENAME: &str = "config.toml";

/// GIM AI configuration extracted from TOML
#[derive(Debug, Clone)]
pub struct GimAiConfig {
    /// Model identifier (e.g., "gpt-4o", "claude-3-sonnet")
    pub model: String,
    /// API key (optional)
    pub api_key: Option<String>,
    /// Base URL for the API (optional)
    pub url: Option<String>,
}

impl GimAiConfig {
    /// Load GIM config from the default location (~/.gim.toml)
    ///
    /// Returns `None` if the config file doesn't exist.
    pub fn load_from_default() -> Result<Option<Self>, ConfigError> {
        let path = Self::default_path()?;
        Self::load_from_path(&path)
    }

    /// Get the default GIM config file path
    pub fn default_path() -> Result<PathBuf, ConfigError> {
        let home = dirs::home_dir().ok_or(ConfigError::HomeDirMissing)?;
        Ok(home
            .join(".config")
            .join(GIM_CONFIG_DIR)
            .join(GIM_CONFIG_FILENAME))
    }

    /// Load GIM config from a specific path
    ///
    /// Returns `Ok(None)` if the file doesn't exist.
    pub fn load_from_path(path: &PathBuf) -> Result<Option<Self>, ConfigError> {
        if !path.exists() {
            return Ok(None);
        }

        let content =
            fs::read_to_string(path).map_err(|e| ConfigError::ReadFailed(e.to_string()))?;

        Self::parse(&content)
    }

    /// Parse GIM AI config from TOML content
    pub fn parse(content: &str) -> Result<Option<Self>, ConfigError> {
        let value: toml::Value = toml::from_str(content)
            .map_err(|e| ConfigError::ParseFailed(format!("Invalid TOML: {}", e)))?;

        let ai_table = match value.get("ai") {
            Some(v) => v.as_table(),
            None => return Ok(None),
        };

        let ai_table = match ai_table {
            Some(t) => t,
            None => return Ok(None),
        };

        // model is required
        let model = match ai_table.get("model").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => return Ok(None),
        };

        Ok(Some(Self {
            model,
            api_key: ai_table
                .get("apikey")
                .and_then(|v| v.as_str())
                .map(String::from),
            url: ai_table
                .get("url")
                .and_then(|v| v.as_str())
                .map(String::from),
        }))
    }

    /// Convert GIM config to an AiModel for Rudist
    ///
    /// Provider is auto-detected from URL first, then from model name if URL is empty.
    /// API key is included (will be stored in keyring by caller).
    pub fn to_ai_model(&self) -> AiModel {
        let url = self.url.as_deref().unwrap_or("");

        // Detect provider: first try URL, then model name
        let provider = if !url.is_empty() {
            AiProviderType::from_url(url)
        } else {
            AiProviderType::from_model_name(&self.model)
        };

        // Determine base_url: use existing URL if present, or use provider's default
        let base_url = if !url.is_empty() {
            self.url.clone()
        } else {
            // Use provider's default API endpoint
            provider.default_url()
        };

        let mut model = AiModel::new_model(
            provider,
            self.model.clone(), // name = model ID
            base_url,
            self.model.clone(), // model_id
        );

        // Copy API key if present
        model.api_key = self.api_key.clone();

        model
    }
}

impl AiProviderType {
    /// Detect provider type from URL
    ///
    /// Falls back to `Custom` if no match is found.
    pub fn from_url(url: &str) -> Self {
        let url_lower = url.to_lowercase();

        if url_lower.contains("openai.com") {
            AiProviderType::OpenAi
        } else if url_lower.contains("anthropic.com") {
            AiProviderType::Anthropic
        } else if url_lower.contains("groq") || url_lower.contains("groq.com") {
            AiProviderType::Groq
        } else if url_lower.contains("openrouter") || url_lower.contains("openrouter.ai") {
            AiProviderType::OpenRouter
        } else if url_lower.contains("mistral") || url_lower.contains("mistral.ai") {
            AiProviderType::Mistral
        } else if url_lower.contains("cohere") || url_lower.contains("cohere.ai") {
            AiProviderType::Cohere
        } else if url_lower.contains("ollama") || url.contains("localhost:11434") {
            AiProviderType::Ollama
        } else if url_lower.contains("lmstudio") || url.contains("localhost:1234") {
            AiProviderType::LMStudio
        } else if url_lower.contains("localai") || url_lower.contains("localhost:8080") {
            AiProviderType::LocalAI
        } else if url_lower.contains("vllm") || url.contains("localhost:8000") {
            AiProviderType::Vllm
        } else if url_lower.contains("together") || url_lower.contains("together.ai") {
            AiProviderType::Together
        } else if url_lower.contains("replicate") || url_lower.contains("replicate.com") {
            AiProviderType::Replicate
        } else if url_lower.contains("huggingface") || url_lower.contains("hf.co") {
            AiProviderType::Huggingface
        } else if url_lower.contains("perplexity") || url_lower.contains("perplexity.ai") {
            AiProviderType::Perplexity
        } else if url_lower.contains("gemini") || url_lower.contains("googleapis") {
            AiProviderType::Gemini
        } else if url_lower.contains("x.ai") || url_lower.contains("grok") {
            AiProviderType::Grok
        } else if url_lower.contains("qwen")
            || url_lower.contains("aliyun")
            || url_lower.contains("dashscope")
        {
            AiProviderType::Qwen
        } else if url_lower.contains("baichuan") || url_lower.contains("baichuan-ai") {
            AiProviderType::Baichuan
        } else if url_lower.contains("doubao")
            || url_lower.contains("volces")
            || url_lower.contains("ark.cn")
        {
            AiProviderType::Doubao
        } else if url_lower.contains("moonshot") || url_lower.contains("moonshot.cn") {
            AiProviderType::Moonshot
        } else if url_lower.contains("zhipu") || url_lower.contains("bigmodel") {
            AiProviderType::Zhipu
        } else if url_lower.contains("minimax") || url_lower.contains("minimax.chat") {
            AiProviderType::Minimax
        } else {
            // Default to Custom for unrecognized providers
            AiProviderType::Custom
        }
    }

    /// Detect provider type from model name
    ///
    /// Falls back to `Custom` if no match is found.
    pub fn from_model_name(model_name: &str) -> Self {
        let name_lower = model_name.to_lowercase();

        if name_lower.contains("gpt") || name_lower.contains("openai") {
            AiProviderType::OpenAi
        } else if name_lower.contains("claude") || name_lower.contains("anthropic") {
            AiProviderType::Anthropic
        } else if name_lower.contains("groq") {
            AiProviderType::Groq
        } else if name_lower.contains("openrouter") {
            AiProviderType::OpenRouter
        } else if name_lower.contains("mistral") {
            AiProviderType::Mistral
        } else if name_lower.contains("cohere") {
            AiProviderType::Cohere
        } else if name_lower.contains("ollama") {
            AiProviderType::Ollama
        } else if name_lower.contains("lmstudio") {
            AiProviderType::LMStudio
        } else if name_lower.contains("localai") || name_lower.contains("local-ai") {
            AiProviderType::LocalAI
        } else if name_lower.contains("vllm") || name_lower.contains("vllm-") {
            AiProviderType::Vllm
        } else if name_lower.contains("together") {
            AiProviderType::Together
        } else if name_lower.contains("replicate") {
            AiProviderType::Replicate
        } else if name_lower.contains("huggingface")
            || name_lower.contains("hf.co")
            || name_lower.contains("-hf")
        {
            AiProviderType::Huggingface
        } else if name_lower.contains("perplexity") {
            AiProviderType::Perplexity
        } else if name_lower.contains("gemini") {
            AiProviderType::Gemini
        } else if name_lower.contains("xai") || name_lower.contains("grok") {
            AiProviderType::Grok
        } else if name_lower.contains("qwen")
            || name_lower.contains("qwen-")
            || name_lower.contains("dashscope")
        {
            AiProviderType::Qwen
        } else if name_lower.contains("baichuan") || name_lower.contains("baichuan-") {
            AiProviderType::Baichuan
        } else if name_lower.contains("doubao")
            || name_lower.contains("doubao-")
            || name_lower.contains("volc")
            || name_lower.contains("火山")
        {
            AiProviderType::Doubao
        } else if name_lower.contains("moonshot")
            || name_lower.contains("moonshot-")
            || name_lower.contains("moonshotai")
        {
            AiProviderType::Moonshot
        } else if name_lower.contains("zhipu") || name_lower.contains("智谱") {
            AiProviderType::Zhipu
        } else if name_lower.contains("minimax") || name_lower.contains("minimax-") {
            AiProviderType::Minimax
        } else {
            AiProviderType::Custom
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_gim_config() {
        let toml = r#"
[ai]
model = "gpt-4o"
apikey = "sk-test123"
url = "https://api.openai.com/v1"
"#;
        let result = GimAiConfig::parse(toml).unwrap();
        assert!(result.is_some());
        let config = result.unwrap();
        assert_eq!(config.model, "gpt-4o");
        assert_eq!(config.api_key, Some("sk-test123".to_string()));
        assert_eq!(config.url, Some("https://api.openai.com/v1".to_string()));
    }

    #[test]
    fn test_parse_minimal_gim_config() {
        let toml = r#"
[ai]
model = "claude-3"
"#;
        let result = GimAiConfig::parse(toml).unwrap();
        assert!(result.is_some());
        let config = result.unwrap();
        assert_eq!(config.model, "claude-3");
        assert_eq!(config.api_key, None);
        assert_eq!(config.url, None);
    }

    #[test]
    fn test_parse_no_ai_section() {
        let toml = r#"
[other]
key = "value"
"#;
        let result = GimAiConfig::parse(toml).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_no_model() {
        let toml = r#"
[ai]
apikey = "sk-test"
"#;
        let result = GimAiConfig::parse(toml).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_provider_detection() {
        assert!(matches!(
            AiProviderType::from_url("https://api.openai.com/v1"),
            AiProviderType::OpenAi
        ));
        assert!(matches!(
            AiProviderType::from_url("https://api.anthropic.com"),
            AiProviderType::Anthropic
        ));
        assert!(matches!(
            AiProviderType::from_url("https://api.groq.com"),
            AiProviderType::Groq
        ));
        assert!(matches!(
            AiProviderType::from_url("https://openrouter.ai/api/v1"),
            AiProviderType::OpenRouter
        ));
        assert!(matches!(
            AiProviderType::from_url("http://localhost:11434"),
            AiProviderType::Ollama
        ));
        assert!(matches!(
            AiProviderType::from_url("https://api.moonshot.cn/v1"),
            AiProviderType::Moonshot
        ));
        assert!(matches!(
            AiProviderType::from_url("https://unknown-provider.com/api"),
            AiProviderType::Custom
        ));
    }

    #[test]
    fn test_to_ai_model() {
        let config = GimAiConfig {
            model: "gpt-4o".to_string(),
            api_key: Some("sk-test".to_string()),
            url: Some("https://api.openai.com/v1".to_string()),
        };

        let model = config.to_ai_model();
        assert_eq!(model.model_id, "gpt-4o");
        assert_eq!(model.name, "gpt-4o");
        assert_eq!(model.api_key, Some("sk-test".to_string()));
        assert!(matches!(model.provider, AiProviderType::OpenAi));
    }

    #[test]
    fn test_to_ai_model_doubao_empty_url() {
        // Test case: GIM config with empty URL but doubao model name
        let config = GimAiConfig {
            model: "doubao-seed-1-6-lite-251015".to_string(),
            api_key: Some("test-api-key".to_string()),
            url: Some("".to_string()),
        };

        let model = config.to_ai_model();
        assert_eq!(model.model_id, "doubao-seed-1-6-lite-251015");
        assert!(matches!(model.provider, AiProviderType::Doubao));
        // Should have a default base_url from Doubao provider
        assert!(model.get_base_url().contains("ark"));
    }

    #[test]
    fn test_provider_detection_from_model_name() {
        assert!(matches!(
            AiProviderType::from_model_name("doubao-seed-1-6-lite-251015"),
            AiProviderType::Doubao
        ));
        assert!(matches!(
            AiProviderType::from_model_name("gpt-4o"),
            AiProviderType::OpenAi
        ));
        assert!(matches!(
            AiProviderType::from_model_name("claude-3-sonnet"),
            AiProviderType::Anthropic
        ));
        assert!(matches!(
            AiProviderType::from_model_name("qwen-2.5-72b"),
            AiProviderType::Qwen
        ));
        assert!(matches!(
            AiProviderType::from_model_name("moonshot-v1-8k"),
            AiProviderType::Moonshot
        ));
        assert!(matches!(
            AiProviderType::from_model_name("unknown-model"),
            AiProviderType::Custom
        ));
    }
}
