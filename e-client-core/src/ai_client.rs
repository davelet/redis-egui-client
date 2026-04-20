use e_client_config::config::ai_config::AiModel;
use serde::{Deserialize, Serialize};
use tracing::{warn, info};

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

// ==================== Anthropic API Structures ====================

/// Anthropic API request structure
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

/// Anthropic API response structure
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    #[serde(default)]
    #[allow(dead_code)]
    stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AnthropicContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Other,
}

/// Detect the API provider type based on URL
pub fn detect_api_provider(url: &str) -> ApiProvider {
    let url_lower = url.to_lowercase();
    if url_lower.contains("anthropic.com") || url_lower.contains("api.anthropic.com") {
        ApiProvider::Anthropic
    } else if url_lower.contains("openrouter.ai") {
        ApiProvider::OpenRouter
    } else if url_lower.contains("ollama") || url_lower.contains("localhost:11434") || url_lower.contains("127.0.0.1:11434") {
        ApiProvider::Ollama
    } else {
        ApiProvider::OpenAI
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProvider {
    OpenAI,
    Anthropic,
    OpenRouter,
    Ollama,
}

impl ApiProvider {
    /// Get the expected header name for API key authentication
    pub fn api_key_header(&self) -> &'static str {
        match self {
            ApiProvider::Anthropic => "x-api-key",
            _ => "Authorization",
        }
    }

    /// Get the API endpoint path for chat completions
    pub fn chat_endpoint(&self) -> &'static str {
        match self {
            ApiProvider::Anthropic => "/messages",
            _ => "/chat/completions",
        }
    }

    /// Check if this provider uses OpenAI-compatible response format
    pub fn uses_openai_format(&self) -> bool {
        !matches!(self, ApiProvider::Anthropic)
    }
}

/// User-friendly error messages for common API errors
pub fn parse_api_error(status: reqwest::StatusCode, body: &str) -> String {
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
        if body_lower.contains("invalid")
            || body_lower.contains("unauthorized")
            || body_lower.contains("authentication")
        {
            return "Authentication failed. Please check your API key.".to_string();
        }
        return "Access denied. Please check your API key and permissions.".to_string();
    }

    // Bad request
    if status.as_u16() == 400 {
        if body_lower.contains("model") && body_lower.contains("not found") {
            return "Model not found. Please check the model ID in your settings.".to_string();
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
    pub async fn chat(
        model: &AiModel,
        message: &str,
        context: Option<&str>,
    ) -> Result<String, String> {
        info!(model = %model.model_id, "Sending chat request to AI");
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        // Build the request URL
        let base_url = model.get_base_url();
        let provider = detect_api_provider(&base_url);
        let endpoint = provider.chat_endpoint();
        let url = format!("{}{}", base_url.trim_end_matches('/'), endpoint);

        // Build request based on provider type
        let mut request_builder = match provider {
            ApiProvider::Anthropic => {
                // Anthropic API format
                let mut messages = Vec::new();
                messages.push(AnthropicMessage {
                    role: "user".to_string(),
                    content: message.to_string(),
                });

                let anthropic_request = AnthropicRequest {
                    model: model.model_id.clone(),
                    messages,
                    system: context.map(|s| s.to_string()),
                    max_tokens: Some(4096),
                    temperature: Some(model.temperature),
                };

                client.post(&url).json(&anthropic_request)
            }
            _ => {
                // OpenAI-compatible format for all other providers
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

                client.post(&url).json(&chat_request)
            }
        };

        // Log request URL and body before sending
        info!(url = %url, model = %model.model_id, provider = ?provider, "AI request URL");
        let request_summary = if provider == ApiProvider::Anthropic {
            format!(
                "model={}, messages=[{{role:user, content:\"{}\"}}], temperature={:?}",
                model.model_id,
                message,
                Some(model.temperature),
            )
        } else {
            format!(
                "model={}, messages=[{}{{role:user, content:\"{}\"}}], temperature={:?}",
                model.model_id,
                context
                    .map(|_| "{role:system, content:...}, ")
                    .unwrap_or_default(),
                message,
                Some(model.temperature),
            )
        };
        info!(request = %request_summary, "AI request body");

        // Add appropriate headers based on provider
        if let Some(ref api_key) = model.api_key {
            match provider {
                ApiProvider::Anthropic => {
                    request_builder = request_builder.header("x-api-key", api_key);
                    request_builder = request_builder.header("anthropic-version", "2023-06-01");
                    request_builder = request_builder.header("Content-Type", "application/json");
                }
                ApiProvider::OpenRouter => {
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                    request_builder = request_builder
                        .header("HTTP-Referer", "https://github.com/e-client/redis-egui");
                }
                _ => {
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        }

        // Send the request with timeout
        info!(url = %url, "Sending HTTP request");
        let response = request_builder.send().await.map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("timeout") {
                warn!(error = %err_str, "Request timeout");
                "Request timeout. Please check your network connection.".to_string()
            } else if err_str.contains("connection") {
                warn!(error = %err_str, "Network error");
                "Network error. Please check your internet connection.".to_string()
            } else {
                warn!(error = %err_str, "Request failed");
                format!("Request failed: {}", err_str)
            }
        })?;

        // Check for HTTP errors with friendly messages
        let status = response.status();
        info!(status = %status, "Received HTTP response");
        if !status.is_success() {
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!(status = %status, error = %text, "API request failed");
            return Err(parse_api_error(status, &text));
        }

        // Parse the response based on provider
        match provider {
            ApiProvider::Anthropic => {
                let anthropic_response: AnthropicResponse = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;

                // Extract text content from Anthropic response
                for content in &anthropic_response.content {
                    if let AnthropicContent::Text { text } = content {
                        return Ok(text.clone());
                    }
                }
                Err("No text content in response".to_string())
            }
            _ => {
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
        }
    }

    /// Send a message synchronously (blocking)
    pub fn chat_sync(
        model: &AiModel,
        message: &str,
        context: Option<&str>,
    ) -> Result<String, String> {
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
        info!(
            model_name = %model.name,
            model_id = %model.model_id,
            provider = ?model.provider,
            "Starting AI connection test"
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| {
                warn!(error = %e, "Failed to create HTTP client");
                format!("Failed to create HTTP client: {}", e)
            })?;

        let base_url = model.get_base_url();
        let provider = detect_api_provider(&base_url);
        let endpoint = provider.chat_endpoint();
        let url = format!("{}{}", base_url.trim_end_matches('/'), endpoint);

        info!(
            base_url = %base_url,
            detected_provider = ?provider,
            endpoint = %endpoint,
            full_url = %url,
            "Connection test configuration"
        );

        // Build request based on provider type
        let mut request_builder = match provider {
            ApiProvider::Anthropic => {
                // Anthropic API format
                let anthropic_request = AnthropicRequest {
                    model: model.get_model_id(),
                    messages: vec![AnthropicMessage {
                        role: "user".to_string(),
                        content: "Hi".to_string(),
                    }],
                    system: None,
                    max_tokens: Some(10),
                    temperature: Some(0.7),
                };
                info!("Using Anthropic API format");
                client.post(&url).json(&anthropic_request)
            }
            _ => {
                // OpenAI-compatible format for all other providers
                let test_request = ChatCompletionRequest {
                    model: model.get_model_id(),
                    messages: vec![ChatMessage {
                        role: "user".to_string(),
                        content: "Hi".to_string(),
                    }],
                    temperature: Some(0.7),
                };
                info!("Using OpenAI-compatible API format");
                client.post(&url).json(&test_request)
            }
        };

        // Add appropriate headers based on provider
        let has_api_key = model.api_key.is_some();
        info!(has_api_key = %has_api_key, "Checking API key");
        if let Some(ref api_key) = model.api_key {
            match provider {
                ApiProvider::Anthropic => {
                    info!("Adding Anthropic headers (x-api-key, anthropic-version)");
                    request_builder = request_builder.header("x-api-key", api_key);
                    request_builder = request_builder.header("anthropic-version", "2023-06-01");
                    request_builder = request_builder.header("Content-Type", "application/json");
                }
                ApiProvider::OpenRouter => {
                    info!("Adding OpenRouter headers (Authorization, HTTP-Referer)");
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                    request_builder = request_builder
                        .header("HTTP-Referer", "https://github.com/e-client/redis-egui");
                }
                _ => {
                    info!("Adding standard Authorization header (Bearer token)");
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        } else {
            warn!("No API key provided for connection test");
        }

        // Send with a short timeout for testing
        info!(url = %url, "Sending connection test request");
        let response = request_builder.send().await.map_err(|e| {
            let err_str = e.to_string();
            warn!(error = %err_str, url = %url, "Connection test request failed");
            if err_str.contains("timeout") {
                "Connection timeout. Please check the URL.".to_string()
            } else if err_str.contains("connection refused") {
                "Connection refused. Please check the URL.".to_string()
            } else {
                format!("Connection failed: {}", err_str)
            }
        })?;

        let status = response.status();
        info!(status = %status, "Received response from server");

        if !response.status().is_success() {
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!(
                status = %status,
                error_body = %text,
                "Connection test failed with HTTP error"
            );
            Err(parse_api_error(status, &text))
        } else {
            let response_text = response.text().await.unwrap_or_else(|_| "".to_string());
            info!(
                raw_response = %response_text,
                "Received raw response from model"
            );

            match provider {
                ApiProvider::Anthropic => {
                    if let Ok(anthropic_response) = serde_json::from_str::<AnthropicResponse>(&response_text) {
                        for content in &anthropic_response.content {
                            if let AnthropicContent::Text { text } = content {
                                info!(model_response = %text, "Model response (Anthropic)");
                            }
                        }
                    }
                }
                _ => {
                    if let Ok(chat_response) = serde_json::from_str::<ChatCompletionResponse>(&response_text) {
                        if let Some(choice) = chat_response.choices.first() {
                            info!(model_response = %choice.message.content, "Model response (OpenAI compatible)");
                        }
                    }
                }
            }

            info!("Connection test successful!");
            Ok(())
        }
    }

    /// Test connection synchronously
    pub fn test_connection_sync(model: &AiModel) -> Result<(), String> {
        let rt = tokio::runtime::Handle::try_current();
        match rt {
            Ok(handle) => tokio::task::block_in_place(|| {
                handle.block_on(async { Self::test_connection(model).await })
            }),
            Err(_) => {
                let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
                rt.block_on(async { Self::test_connection(model).await })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== API Provider Detection Tests ====================

    #[test]
    fn test_detect_api_provider_openai() {
        let urls = vec![
            "https://api.openai.com/v1",
            "https://api.openai.com/v1/",
            "https://openai.com/api",
            "https://api.azure.com/openai",
        ];
        for url in urls {
            assert_eq!(
                detect_api_provider(url),
                ApiProvider::OpenAI,
                "Failed for URL: {}",
                url
            );
        }
    }

    #[test]
    fn test_detect_api_provider_anthropic() {
        let urls = vec![
            "https://api.anthropic.com",
            "https://api.anthropic.com/v1",
            "https://anthropic.com/api",
            "https://console.anthropic.com/api",
        ];
        for url in urls {
            assert_eq!(
                detect_api_provider(url),
                ApiProvider::Anthropic,
                "Failed for URL: {}",
                url
            );
        }
    }

    #[test]
    fn test_detect_api_provider_openrouter() {
        let urls = vec!["https://openrouter.ai/api", "https://openrouter.ai/api/v1"];
        for url in urls {
            assert_eq!(
                detect_api_provider(url),
                ApiProvider::OpenRouter,
                "Failed for URL: {}",
                url
            );
        }
    }

    #[test]
    fn test_detect_api_provider_ollama() {
        let urls = vec![
            "http://localhost:11434",
            "http://localhost:11434/api",
            "http://ollama.local:11434",
            "http://127.0.0.1:11434",
            "http://127.0.0.1:11434/api",
        ];
        for url in urls {
            assert_eq!(
                detect_api_provider(url),
                ApiProvider::Ollama,
                "Failed for URL: {}",
                url
            );
        }
    }

    #[test]
    fn test_detect_api_provider_case_insensitive() {
        assert_eq!(
            detect_api_provider("HTTPS://API.OPENAI.COM/V1"),
            ApiProvider::OpenAI
        );
        assert_eq!(
            detect_api_provider("HTTPS://API.ANTHROPIC.COM"),
            ApiProvider::Anthropic
        );
        assert_eq!(
            detect_api_provider("HTTPS://OPENROUTER.AI/API"),
            ApiProvider::OpenRouter
        );
        assert_eq!(
            detect_api_provider("HTTP://LOCALHOST:11434"),
            ApiProvider::Ollama
        );
    }

    // ==================== API Provider Header Tests ====================

    #[test]
    fn test_api_provider_header_openai() {
        assert_eq!(ApiProvider::OpenAI.api_key_header(), "Authorization");
    }

    #[test]
    fn test_api_provider_header_anthropic() {
        assert_eq!(ApiProvider::Anthropic.api_key_header(), "x-api-key");
    }

    #[test]
    fn test_api_provider_header_openrouter() {
        assert_eq!(ApiProvider::OpenRouter.api_key_header(), "Authorization");
    }

    #[test]
    fn test_api_provider_header_ollama() {
        assert_eq!(ApiProvider::Ollama.api_key_header(), "Authorization");
    }

    // ==================== Error Parsing Tests ====================

    #[test]
    fn test_parse_api_error_rate_limit_generic() {
        let result = parse_api_error(
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded",
        );
        assert!(result.contains("Rate limit exceeded"));
    }

    #[test]
    fn test_parse_api_error_rate_limit_specific() {
        let result = parse_api_error(
            reqwest::StatusCode::from_u16(429).unwrap(),
            "Too many requests, please slow down",
        );
        assert!(result.contains("Rate limit exceeded"));
        assert!(result.contains("wait"));
    }

    #[test]
    fn test_parse_api_error_auth_401_invalid() {
        let result = parse_api_error(reqwest::StatusCode::UNAUTHORIZED, "Invalid API key");
        assert!(result.contains("Authentication failed"));
    }

    #[test]
    fn test_parse_api_error_auth_401_unauthorized() {
        let result = parse_api_error(
            reqwest::StatusCode::from_u16(401).unwrap(),
            "Unauthorized access",
        );
        assert!(result.contains("Authentication failed"));
    }

    #[test]
    fn test_parse_api_error_auth_403() {
        let result = parse_api_error(reqwest::StatusCode::FORBIDDEN, "Access denied");
        assert!(result.contains("Access denied"));
    }

    #[test]
    fn test_parse_api_error_auth_403_authentication() {
        let result = parse_api_error(
            reqwest::StatusCode::from_u16(403).unwrap(),
            "Authentication required",
        );
        assert!(result.contains("Authentication failed"));
    }

    #[test]
    fn test_parse_api_error_bad_request_model_not_found() {
        let result = parse_api_error(
            reqwest::StatusCode::BAD_REQUEST,
            "Model not found in the model list",
        );
        assert!(result.contains("Model not found"));
    }

    #[test]
    fn test_parse_api_error_bad_request_other() {
        let result = parse_api_error(
            reqwest::StatusCode::BAD_REQUEST,
            "Invalid parameter: temperature",
        );
        assert!(result.contains("Invalid request"));
        assert!(result.contains("temperature"));
    }

    #[test]
    fn test_parse_api_error_server_error_500() {
        let result = parse_api_error(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
        );
        assert!(result.contains("Server error"));
    }

    #[test]
    fn test_parse_api_error_server_error_502() {
        let result = parse_api_error(reqwest::StatusCode::BAD_GATEWAY, "Bad gateway");
        assert!(result.contains("Server error"));
    }

    #[test]
    fn test_parse_api_error_server_error_503() {
        let result = parse_api_error(
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            "Service unavailable",
        );
        assert!(result.contains("Server error"));
    }

    #[test]
    fn test_parse_api_error_other_status() {
        let result = parse_api_error(reqwest::StatusCode::from_u16(418).unwrap(), "I'm a teapot");
        assert!(result.contains("HTTP"));
        assert!(result.contains("418"));
    }
}
