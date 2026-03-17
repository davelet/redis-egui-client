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

/// AI Client for making requests to AI models
pub struct AiClient;

impl AiClient {
    /// Send a message to the AI model and get a response
    pub async fn chat(model: &AiModel, message: &str) -> Result<String, String> {
        let client = reqwest::Client::new();

        let chat_request = ChatCompletionRequest {
            model: model.model_id.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            temperature: Some(model.temperature),
        };

        // Build the request URL
        let url = format!("{}/chat/completions", model.url.trim_end_matches('/'));

        // Build the request
        let mut request_builder = client.post(&url).json(&chat_request);

        if let Some(ref api_key) = model.api_key {
            if model.url.contains("anthropic.com") {
                request_builder = request_builder.header("x-api-key", api_key);
                request_builder = request_builder.header("anthropic-version", "2023-06-01");
            } else {
                request_builder =
                    request_builder.header("Authorization", format!("Bearer {}", api_key));
            }
        }

        // Send the request
        let response = request_builder
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("HTTP {}: {}", status, text));
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
    pub fn chat_sync(model: &AiModel, message: &str) -> Result<String, String> {
        let rt = tokio::runtime::Handle::try_current();
        match rt {
            Ok(handle) => {
                // We're in an async context, use block_in_place
                tokio::task::block_in_place(|| {
                    handle.block_on(async { Self::chat(model, message).await })
                })
            }
            Err(_) => {
                // No runtime available, create one
                let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
                rt.block_on(async { Self::chat(model, message).await })
            }
        }
    }
}
