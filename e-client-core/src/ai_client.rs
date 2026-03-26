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
pub fn detect_api_provider(url: &str) -> ApiProvider {
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
        let response = request_builder.send().await.map_err(|e| {
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
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
                _ => {
                    request_builder =
                        request_builder.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        }

        // Send with a short timeout for testing
        let response = request_builder.send().await.map_err(|e| {
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
        ];
        for url in urls {
            assert_eq!(
                detect_api_provider(url),
                ApiProvider::Ollama,
                "Failed for URL: {}",
                url
            );
        }

        // 127.0.0.1 doesn't contain "localhost" so it's not detected as Ollama
        assert_eq!(
            detect_api_provider("http://127.0.0.1:11434"),
            ApiProvider::OpenAI,
            "127.0.0.1 should be detected as OpenAI (no localhost keyword)"
        );
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
