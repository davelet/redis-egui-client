use e_client_config::config::ai_config::AiModel;
use serde::{Deserialize, Serialize};

/// AI chat message
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI-compatible chat completion request
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

/// OpenAI-compatible chat completion response
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

/// Detect the API provider type based on URL
fn detect_api_provider(url: &str) -> ApiProvider {
    let url_lower = url.to_lowercase();
    if url_lower.contains("anthropic.com") || url_lower.contains("api.anthropic.com") {
        ApiProvider::Anthropic
    } else if url_lower.contains("openrouter.ai") {
        ApiProvider::OpenRouter
    } else if url_lower.contains("ollama") || url_lower.contains("localhost:11434") {
        ApiProvider::Ollama
    } else {
        ApiProvider::OpenAI
    }
}

#[derive(Debug, Clone, Copy)]
enum ApiProvider {
    OpenAI,
    Anthropic,
    OpenRouter,
    Ollama,
}

/// User-friendly error messages for common API errors
fn parse_api_error(status: reqwest::StatusCode, body: &str) -> String {
    let body_lower = body.to_lowercase();

    // Rate limit errors
    if status.as_u16() == 429 {
        if body_lower.contains("rate limit") || body_lower.contains("too many requests") {
            return "Rate limit exceeded. Please wait a moment and try again.".to_string();
        }
        return "Rate limit exceeded.".to_string();
    }

    // Authentication errors
    if status.as_u16() == 401 || status.as_u16() == 403 {
        if body_lower.contains("invalid") || body_lower.contains("unauthorized")
            || body_lower.contains("authentication") {
            return "Authentication failed. Please check your API key.".to_string();
        }
        return "Access denied. Please check your API key and permissions.".to_string();
    }

    // Bad request
    if status.as_u16() == 400 {
        if body_lower.contains("model") && body_lower.contains("not found") {
            return format!(
                "Model not found. Please check the model ID in your settings."
            );
        }
        return format!("Invalid request: {}", body);
    }

    // Server errors
    if status.is_server_error() {
        return "Server error. Please try again later.".to_string();
    }

    format!("HTTP {}: {}", status, body)
}

/// AI Client for making requests to AI models
pub struct AiClient;

impl AiClient {
    /// Send a message to the AI model with optional context
    pub async fn chat(model: &AiModel, message: &str, context: Option<&str>) -> Result<String, String> {
        let client = reqwest::Client::new();

        // Build messages with optional system prompt
        let mut messages = Vec::new();

        // Add system prompt with Redis context if provided
        if let Some(ctx) = context {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: ctx.to_string(),
            });
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        });

        let chat_request = ChatCompletionRequest {
            model: model.model_id.clone(),
            messages,
            temperature: Some(model.temperature),
        };

        // Build the request URL
        let url = format!("{}/chat/completions", model.url.trim_end_matches('/'));

        // Build the request with appropriate headers based on provider
        let provider = detect_api_provider(&model.url);
        let mut request_builder = client.post(&url).json(&chat_request);

        if let Some(ref api_key) = model.api_key {
            match provider {
                ApiProvider::Anthropic => {
                    request_builder = request_builder.header("x-api-key", api_key);
                    request_builder = request_builder.header("anthropic-version", "2023-06-01");
                }
                ApiProvider::OpenRouter => {
                    request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
                    request_builder = request_builder.header("HTTP-Referer", "https://github.com/e-client/redis-egui");
                }
                _ => {
                    request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        }

        // Send the request with timeout
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("timeout") {
                    "Request timeout. Please check your network connection.".to_string()
                } else if err_str.contains("connection") {
                    "Network error. Please check your internet connection.".to_string()
                } else {
                    format!("Request failed: {}", err_str)
                }
            })?;

        // Check for HTTP errors with friendly messages
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(parse_api_error(status, &text));
        }

        // Parse the response
        let chat_response: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Extract the content from the first choice
        chat_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| "No response from AI".to_string())
    }

    /// Send a message synchronously (blocking)
    pub fn chat_sync(model: &AiModel, message: &str, context: Option<&str>) -> Result<String, String> {
        let rt = tokio::runtime::Handle::try_current();
        match rt {
            Ok(handle) => {
                // We're in an async context, use block_in_place
                tokio::task::block_in_place(|| {
                    handle.block_on(async { Self::chat(model, message, context).await })
                })
            }
            Err(_) => {
                // No runtime available, create one
                let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
                rt.block_on(async { Self::chat(model, message, context).await })
            }
        }
    }

    /// Test if the API connection is working
    pub async fn test_connection(model: &AiModel) -> Result<(), String> {
        let client = reqwest::Client::new();
        let provider = detect_api_provider(&model.url);

        // Build a simple test request
        let test_request = ChatCompletionRequest {
            model: model.model_id.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Hi".to_string(),
            }],
            temperature: Some(0.7),
        };

        let url = format!("{}/chat/completions", model.url.trim_end_matches('/'));
        let mut request_builder = client.post(&url).json(&test_request);

        if let Some(ref api_key) = model.api_key {
            match provider {
                ApiProvider::Anthropic => {
                    request_builder = request_builder.header("x-api-key", api_key);
                    request_builder = request_builder.header("anthropic-version", "2023-06-01");
                }
                ApiProvider::OpenRouter => {
                    request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
                _ => {
                    request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        }

        // Send with a short timeout for testing
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("timeout") {
                    "Connection timeout. Please check the URL.".to_string()
                } else if err_str.contains("connection refused") {
                    "Connection refused. Please check the URL.".to_string()
                } else {
                    format!("Connection failed: {}", err_str)
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(parse_api_error(status, &text))
        } else {
            Ok(())
        }
    }

    /// Test connection synchronously
    pub fn test_connection_sync(model: &AiModel) -> Result<(), String> {
        let rt = tokio::runtime::Handle::try_current();
        match rt {
            Ok(handle) => {
                tokio::task::block_in_place(|| {
                    handle.block_on(async { Self::test_connection(model).await })
                })
            }
            Err(_) => {
                let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
                rt.block_on(async { Self::test_connection(model).await })
            }
        }
    }
}
