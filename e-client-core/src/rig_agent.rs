use crate::ai_tools::{
    DeleteKeysTool, ExecuteCommandTool, FilterKeysTool, GetDbStatsTool, GetKeyInfoTool, HdelTool,
    HsetTool, KeyExistsTool, LsetTool, RenameKeyTool, RpushTool, SaddTool, SelectDbTool,
    SetStringTool, SetTtlTool, SremTool, ZaddTool, ZremTool,
};
use crate::redis_client::RedisClient;
use e_client_config::config::ai_config::{AiConfig, AiModel, AiMode};
use rig::{agent::Agent, client::CompletionClient, completion::{Message, Prompt}, providers::openai};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

/// Built-in system prompt for the Redis GUI Client AI assistant (Agent mode - full tools).
pub const SYSTEM_PROMPT: &str = r#"You are an AI assistant for Redis GUI Client. You can help users:
1. Browse and search Redis keys
2. View and edit key values
3. Execute Redis commands
4. Filter and organize keys
5. Manage database and data structures

You have access to tools that can:
- filter_keys: List Redis keys matching a pattern (pattern supports * wildcard)
- get_key_info: Get detailed information about a specific key (type, TTL, value preview)
- delete_keys: Delete one or more keys
- execute_redis_command: Execute any Redis command
- get_db_stats: Get database statistics
- set_string: Set a string value for a key
- set_ttl: Set expiration time for a key (-1 to remove expiration)
- rename_key: Rename a key (fails if new key already exists)
- key_exists: Check if a key exists
- hset: Set a field-value pair in a Hash
- hdel: Delete a field from a Hash
- lset: Set an element in a List by index
- sadd: Add a member to a Set
- srem: Remove a member from a Set
- zadd: Add a member with score to a Sorted Set
- zrem: Remove a member from a Sorted Set
- rpush: Push a value to the right end of a List
- select_db: Switch to a different Redis database

When responding:
- Use tools to fetch actual data from Redis
- Format your responses clearly
- Show Redis output in a readable format
- For key lists, show count and samples
- For errors, explain what went wrong and suggest fixes

Example interactions:
- User: "Show me all user keys"
  You: Use filter_keys with pattern "user:*"

- User: "What's in key 'session:123'?"
  You: Use get_key_info with key "session:123"

- User: "Delete old cache keys"
  You: First find them with filter_keys, then delete with delete_keys

- User: "Set session:abc to expire in 1 hour"
  You: Use set_ttl with key "session:abc" and ttl 3600

- User: "Add user:100 to my users set"
  You: Use sadd with key "my_users" and member "user:100""#;

/// System prompt for Chat mode - stateless, tool-less command translation.
pub const CHAT_SYSTEM_PROMPT: &str = r#"You are helping with Redis database operations.
Your task is to translate natural language queries into ONE precise Redis command.
Always respond with valid Redis command that can be executed directly.
Only output the Redis command without any explanation or markdown formatting.

Examples:
- User: "how many keys do I have?" → DBSIZE
- User: "show me all keys" → KEYS *
- User: "what's the value of mykey" → GET mykey
- User: "set mykey to hello" → SET mykey hello
- User: "delete the key test" → DEL test
- User: "list all string keys" → KEYS *"#;

pub type OpenAiAgent = Agent<openai::responses_api::ResponsesCompletionModel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiResponseError {
    NoModelConfigured,
    MissingApiKey,
    AuthFailed,
    RateLimitExceeded,
    NetworkError,
    InvalidUrl,
    InvalidModel(String),
    ServerError(String),
    ModelNotFound,
    Other(String),
}

impl AiResponseError {
    pub fn from_error_string(error: &str) -> Self {
        let error_lower = error.to_lowercase();

        if error_lower.contains("401")
            || error_lower.contains("unauthorized")
            || error_lower.contains("incorrect api key")
            || error_lower.contains("invalid_api_key")
        {
            AiResponseError::AuthFailed
        } else if error_lower.contains("429") || error_lower.contains("rate limit") {
            AiResponseError::RateLimitExceeded
        } else if error_lower.contains("connection")
            || error_lower.contains("timeout")
            || error_lower.contains("network")
            || error_lower.contains("refused")
        {
            AiResponseError::NetworkError
        } else if error_lower.contains("reasoning is not supported for model") {
            // "CompletionError: HttpError: Invalid status code 400 Bad Request with message: {"error":{"code":"InvalidParameter","message":"The parameter `input[2].reasoning` specified in the request are not valid: Item reasoning is not supported for model: doubao-seed-1-6-lite, version: 251015, please use another latest model. Request id: 0217749362096289a0d83abff91b62c26f47d22f8f16225aca1e2","param":"input[2].reasoning","type":"BadRequest"}}"
            let mut m = String::new();
            if let Some(model_start) = error_lower.find("model: ") {
                    let after_model = &error_lower[model_start + "model: ".len()..];

                    if let Some(comma_pos) = after_model.find(", please") {
                        let model_part = &after_model[..comma_pos].trim();

                        m = model_part.to_string();
                    }
                }
            AiResponseError::InvalidModel(m)
        } else if error_lower.contains("url") || error_lower.contains("invalid") {
            AiResponseError::InvalidUrl
        } else if error_lower.contains("500")
            || error_lower.contains("502")
            || error_lower.contains("503")
            || error_lower.contains("server error")
        {
            AiResponseError::ServerError(error.to_string())
        } else if error_lower.contains("model") && error_lower.contains("not found") {
            AiResponseError::ModelNotFound
        } else if error_lower.contains("no active ai model")
            || error_lower.contains("no model configured")
        {
            AiResponseError::NoModelConfigured
        } else if error_lower.contains("api key") && error_lower.contains("required") {
            AiResponseError::MissingApiKey
        } else {
            AiResponseError::Other(error.to_string())
        }
    }

    pub fn get_i18n_key(&self) -> e_client_bilingual::translations::TranslationKey {
        use e_client_bilingual::translations::TranslationKey;
        match self {
            AiResponseError::NoModelConfigured => TranslationKey::AiErrorNoModelConfigured,
            AiResponseError::MissingApiKey => TranslationKey::AiErrorMissingApiKey,
            AiResponseError::AuthFailed => TranslationKey::AiErrorAuthFailed,
            AiResponseError::RateLimitExceeded => TranslationKey::AiErrorRateLimit,
            AiResponseError::NetworkError => TranslationKey::AiErrorNetwork,
            AiResponseError::InvalidUrl => TranslationKey::AiErrorInvalidUrl,
            AiResponseError::InvalidModel(_) => TranslationKey::AiErrorInvalidModel,
            AiResponseError::ServerError(_) => TranslationKey::AiErrorServerError,
            AiResponseError::ModelNotFound => TranslationKey::AiErrorModelNotFound,
            AiResponseError::Other(_) => TranslationKey::AiErrorOther,
        }
    }

    pub fn get_detail(&self) -> Option<String> {
        match self {
            AiResponseError::ServerError(msg) => Some(msg.clone()),
            AiResponseError::Other(msg) => Some(msg.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum AiChatResult {
    Text(String),
    ToolCall { name: String, result: String },
    Error(AiResponseError),
}

pub struct OpenAiRigAgent {
    agent: OpenAiAgent,
    llm_model: AiModel,
    mode: AiMode,
    history: Vec<Message>,
    max_turns: u32,
}

impl OpenAiRigAgent {
    pub async fn new(
        config: &AiConfig,
        redis_client: Arc<RedisClient>,
        mode: AiMode,
    ) -> Result<Self, String> {
        let model = config
            .get_active_model()
            .ok_or("No active AI model configured")?;

        if model.api_key.is_none() {
            return Err("API key not loaded. Please configure API key.".to_string());
        }

        let api_key = model.api_key.as_ref().unwrap();
        let base_url = model.get_base_url();

        let client = if base_url == "https://api.openai.com/v1" {
            openai::Client::new(api_key.clone())
                .map_err(|e| format!("Failed to create OpenAI client: {}", e))?
        } else {
            openai::Client::builder()
                .base_url(&base_url)
                .api_key(api_key)
                .build()
                .map_err(|e| format!("Failed to create API client: {}", e))?
        };

        let agent = match mode {
            AiMode::Agent => {
                // Agent mode: full tools, system prompt with tools
                client
                    .agent(&model.get_model_id())
                    .preamble(SYSTEM_PROMPT)
                    .default_max_turns(config.max_turns as usize)
                    .temperature(model.temperature as f64)
                    .tool(FilterKeysTool::new(redis_client.clone()))
                    .tool(GetKeyInfoTool::new(redis_client.clone()))
                    .tool(DeleteKeysTool::new(redis_client.clone()))
                    .tool(ExecuteCommandTool::new(redis_client.clone()))
                    .tool(GetDbStatsTool::new(redis_client.clone()))
                    .tool(SetStringTool::new(redis_client.clone()))
                    .tool(SetTtlTool::new(redis_client.clone()))
                    .tool(RenameKeyTool::new(redis_client.clone()))
                    .tool(KeyExistsTool::new(redis_client.clone()))
                    .tool(HsetTool::new(redis_client.clone()))
                    .tool(HdelTool::new(redis_client.clone()))
                    .tool(LsetTool::new(redis_client.clone()))
                    .tool(SaddTool::new(redis_client.clone()))
                    .tool(SremTool::new(redis_client.clone()))
                    .tool(ZaddTool::new(redis_client.clone()))
                    .tool(ZremTool::new(redis_client.clone()))
                    .tool(RpushTool::new(redis_client.clone()))
                    .tool(SelectDbTool::new(redis_client))
                    .build()
            }
            AiMode::Chat => {
                // Chat mode: no tools, stateless, simple command translation prompt
                client
                    .agent(&model.get_model_id())
                    .preamble(CHAT_SYSTEM_PROMPT)
                    .temperature(model.temperature as f64)
                    .build()
            }
        };

        Ok(Self {
            agent,
            llm_model: model.clone(),
            mode,
            history: Vec::new(),
            max_turns: config.max_turns,
        })
    }

    /// Number of completed conversation turns (user+assistant pairs in history).
    pub fn turn_count(&self) -> usize {
        // history contains alternating User/Assistant messages; each pair = 1 turn
        self.history.len() / 2
    }

    /// Configured max turns for this agent.
    pub fn max_turns(&self) -> u32 {
        self.max_turns
    }

    pub async fn chat(&mut self, message: &str) -> Result<AiChatResult, AiResponseError> {
        info!(
            url = %self.llm_model.base_url.clone().unwrap_or(String::new()),
            model = %self.llm_model.model_id,
            mode = ?self.mode,
            message = %message,
            "Rig agent AI request: "
        );
        self.agent
            .prompt(message)
            .max_turns(self.max_turns as usize)
            .with_history(&mut self.history)
            .await
            .map(AiChatResult::Text)
            .map_err(|e| AiResponseError::from_error_string(&e.to_string()))
    }
}
