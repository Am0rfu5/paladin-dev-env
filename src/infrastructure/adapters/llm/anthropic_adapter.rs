// src/infrastructure/adapters/llm/anthropic_adapter.rs
//
// Anthropic Claude LLM Adapter
//
// Provides integration with Anthropic's Claude API.
// Supports standard completions, streaming, and all core LlmPort functionality.
// Claude has unique requirements: system messages separate from messages array,
// max_tokens required, and different message structure.

use async_trait::async_trait;
use chrono::Utc;
use futures::{Stream, StreamExt};
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use uuid::Uuid;

use crate::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities,
    StreamingResponse, TokenUsage,
};
use crate::core::platform::container::prompt::PromptType;

/// Configuration for Anthropic Claude LLM adapter
#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    /// API key for Anthropic authentication
    pub api_key: String,
    /// Base URL for Anthropic API
    pub base_url: String,
    /// Default model to use (e.g., claude-3-5-sonnet-20241022)
    pub model: String,
    /// Default max tokens for responses (required by Claude API)
    pub max_tokens: u32,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl AnthropicConfig {
    /// Load configuration from environment variables
    ///
    /// # Environment Variables
    /// - `ANTHROPIC_API_KEY` (required): Anthropic API key
    /// - `ANTHROPIC_BASE_URL` (optional): API base URL, defaults to https://api.anthropic.com/v1
    /// - `ANTHROPIC_MODEL` (optional): Default model, defaults to claude-3-5-sonnet-20241022
    /// - `ANTHROPIC_MAX_TOKENS` (optional): Default max tokens, defaults to 4096
    /// - `ANTHROPIC_TIMEOUT_SECONDS` (optional): Request timeout, defaults to 60
    ///
    /// # Errors
    /// Returns error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY environment variable not set")?;

        let base_url = env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string());

        let model = env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

        let max_tokens = env::var("ANTHROPIC_MAX_TOKENS")
            .unwrap_or_else(|_| "4096".to_string())
            .parse()
            .map_err(|_| "Invalid ANTHROPIC_MAX_TOKENS value")?;

        let timeout_seconds = env::var("ANTHROPIC_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map_err(|_| "Invalid ANTHROPIC_TIMEOUT_SECONDS value")?;

        let config = Self {
            api_key,
            base_url,
            model,
            max_tokens,
            timeout_seconds,
        };

        config.validate()?;
        Ok(config)
    }

    /// Create configuration with custom values
    pub fn new(api_key: String, base_url: String, model: String, max_tokens: u32) -> Self {
        Self {
            api_key,
            base_url,
            model,
            max_tokens,
            timeout_seconds: 60,
        }
    }

    /// Validate configuration
    fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }

        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err("Base URL must start with http:// or https://".to_string());
        }

        if self.model.is_empty() {
            return Err("Model cannot be empty".to_string());
        }

        if self.max_tokens == 0 {
            return Err("Max tokens must be greater than 0".to_string());
        }

        Ok(())
    }
}

/// Anthropic Claude LLM Adapter
pub struct AnthropicAdapter {
    pub(crate) client: Client,
    pub(crate) config: AnthropicConfig,
}

impl AnthropicAdapter {
    /// Create a new Anthropic adapter with the given configuration
    ///
    /// # Arguments
    /// * `config` - Anthropic configuration
    ///
    /// # Errors
    /// Returns error if configuration is invalid or HTTP client cannot be created
    pub fn new(config: AnthropicConfig) -> Result<Self, LlmError> {
        config
            .validate()
            .map_err(|e| LlmError::AuthenticationError(format!("Invalid configuration: {}", e)))?;

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| LlmError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Build HTTP headers for Anthropic API requests
    fn build_headers(&self) -> Result<HeaderMap, LlmError> {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Anthropic uses x-api-key header
        let api_key_header = HeaderValue::from_str(&self.config.api_key)
            .map_err(|e| LlmError::AuthenticationError(format!("Invalid API key format: {}", e)))?;
        headers.insert("x-api-key", api_key_header);

        // Anthropic API version header (required)
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));

        Ok(headers)
    }

    /// Convert LlmRequest to Claude API format
    ///
    /// Claude has specific requirements:
    /// - System message goes in separate `system` parameter
    /// - Messages array contains only user/assistant messages
    /// - Messages must alternate between user and assistant
    fn build_request(&self, request: &LlmRequest) -> Result<ClaudeRequest, LlmError> {
        let mut system_message = None;
        let mut messages = Vec::new();

        // Extract prompt type from the prompt item
        match &request.prompt.node.node.prompt_type {
            PromptType::System(system_prompt) => {
                system_message = Some(system_prompt.instructions.clone());
            }
            PromptType::User(user_prompt) => {
                messages.push(ClaudeMessage {
                    role: "user".to_string(),
                    content: user_prompt.query.clone(),
                });
            }
            PromptType::Text(text_prompt) => {
                // For text prompts, extract role and content
                let role = match &text_prompt.role {
                    crate::core::platform::container::prompt::PromptRole::System => {
                        // Store system message separately
                        system_message = Some(text_prompt.content.clone());
                        return Ok(ClaudeRequest {
                            model: request.model.clone(),
                            messages: vec![],
                            system: system_message,
                            max_tokens: request
                                .prompt
                                .node
                                .node
                                .parameters
                                .max_tokens
                                .unwrap_or(self.config.max_tokens),
                            temperature: request.prompt.node.node.parameters.temperature,
                            top_p: request.prompt.node.node.parameters.top_p,
                            stream: false,
                        });
                    }
                    crate::core::platform::container::prompt::PromptRole::User => "user",
                    crate::core::platform::container::prompt::PromptRole::Assistant => "assistant",
                    crate::core::platform::container::prompt::PromptRole::Function => "user", // Map function to user
                };
                messages.push(ClaudeMessage {
                    role: role.to_string(),
                    content: text_prompt.content.clone(),
                });
            }
            PromptType::Assistant(assistant_prompt) => {
                messages.push(ClaudeMessage {
                    role: "assistant".to_string(),
                    content: assistant_prompt.response.clone(),
                });
            }
            PromptType::Function(_) => {
                return Err(LlmError::InvalidPrompt(
                    "Function prompts not yet supported for Claude".to_string(),
                ));
            }
        }

        // Determine max_tokens from prompt parameters or use config default
        let max_tokens = request
            .prompt
            .node
            .node
            .parameters
            .max_tokens
            .unwrap_or(self.config.max_tokens);

        Ok(ClaudeRequest {
            model: request.model.clone(),
            messages,
            system: system_message,
            max_tokens,
            temperature: request.prompt.node.node.parameters.temperature,
            top_p: request.prompt.node.node.parameters.top_p,
            stream: false,
        })
    }

    /// Parse Claude API response into LlmResponse
    fn parse_response(&self, request_id: Uuid, response: ClaudeResponse) -> LlmResponse {
        let content = response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("stop_sequence") => FinishReason::Stop,
            _ => FinishReason::Error("unknown".to_string()),
        };

        LlmResponse {
            id: Uuid::new_v4(),
            request_id,
            model: response.model,
            content,
            finish_reason,
            usage: TokenUsage {
                prompt_tokens: response.usage.input_tokens,
                completion_tokens: response.usage.output_tokens,
                total_tokens: response.usage.input_tokens + response.usage.output_tokens,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        }
    }

    /// Map Anthropic API errors to LlmError
    fn map_error(&self, status: u16, body: &str) -> LlmError {
        match status {
            401 => LlmError::AuthenticationError(
                "Invalid API key. Check your ANTHROPIC_API_KEY environment variable.".to_string(),
            ),
            403 => LlmError::AuthenticationError(
                "API key does not have permission for this resource.".to_string(),
            ),
            429 => LlmError::RateLimitExceeded,
            400 => {
                if body.contains("max_tokens") {
                    LlmError::InvalidPrompt(
                        "Invalid max_tokens value. Claude requires max_tokens to be set."
                            .to_string(),
                    )
                } else {
                    LlmError::InvalidPrompt(format!("Bad request: {}", body))
                }
            }
            500..=599 => LlmError::ProcessingError(format!(
                "Anthropic server error ({}). Please retry.",
                status
            )),
            _ => LlmError::ProcessingError(format!(
                "Request failed with status {}: {}",
                status, body
            )),
        }
    }

    /// Execute request with retry logic
    async fn execute_with_retry<F, Fut, T>(
        &self,
        operation: F,
        max_retries: u32,
    ) -> Result<T, LlmError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, LlmError>>,
    {
        let mut attempt = 0;
        let mut delay_ms = 1000; // Start with 1 second

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;

                    // Don't retry authentication or invalid prompt errors
                    if matches!(
                        e,
                        LlmError::AuthenticationError(_) | LlmError::InvalidPrompt(_)
                    ) {
                        return Err(e);
                    }

                    // Check if we've exhausted retries
                    if attempt >= max_retries {
                        return Err(e);
                    }

                    // Exponential backoff with jitter
                    let jitter = (rand::random::<f64>() * 200.0) as u64;
                    tokio::time::sleep(Duration::from_millis(delay_ms + jitter)).await;
                    delay_ms = (delay_ms * 2).min(10000); // Cap at 10 seconds
                }
            }
        }
    }
}

#[async_trait]
impl LlmPort for AnthropicAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let claude_request = self.build_request(&request)?;
        let headers = self.build_headers()?;

        let operation = || async {
            let response = self
                .client
                .post(format!("{}/messages", self.config.base_url))
                .headers(headers.clone())
                .json(&claude_request)
                .send()
                .await
                .map_err(|e| LlmError::ProcessingError(format!("Request failed: {}", e)))?;

            let status = response.status().as_u16();

            if !response.status().is_success() {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(self.map_error(status, &body));
            }

            let claude_response: ClaudeResponse = response.json().await.map_err(|e| {
                LlmError::ProcessingError(format!("Failed to parse response: {}", e))
            })?;

            Ok(self.parse_response(request.id, claude_response))
        };

        self.execute_with_retry(operation, 3).await
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError> {
        let mut claude_request = self.build_request(&request)?;
        claude_request.stream = true;

        let headers = self.build_headers()?;

        let response = self
            .client
            .post(format!("{}/messages", self.config.base_url))
            .headers(headers)
            .json(&claude_request)
            .send()
            .await
            .map_err(|e| LlmError::ProcessingError(format!("Stream request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(self.map_error(status, &body));
        }

        let _request_id = request.id;
        let stream = response.bytes_stream().map(move |chunk_result| {
            match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);

                    // Parse SSE format: data: {...}\n\n
                    for line in text.lines() {
                        if let Some(json_str) = line.strip_prefix("data: ") {
                            if json_str.trim() == "[DONE]" {
                                continue;
                            }

                            // Parse Claude streaming event
                            if let Ok(event) = serde_json::from_str::<ClaudeStreamEvent>(json_str) {
                                match event.event_type.as_str() {
                                    "content_block_delta" => {
                                        if let Some(delta) = event.delta
                                            && let Some(text) = delta.text
                                        {
                                            return Ok(StreamingResponse {
                                                id: Uuid::new_v4(),
                                                delta: text,
                                                finish_reason: None,
                                            });
                                        }
                                    }
                                    "message_stop" => {
                                        return Ok(StreamingResponse {
                                            id: Uuid::new_v4(),
                                            delta: String::new(),
                                            finish_reason: Some(FinishReason::Stop),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // If we didn't find valid data, return empty chunk
                    Ok(StreamingResponse {
                        id: Uuid::new_v4(),
                        delta: String::new(),
                        finish_reason: None,
                    })
                }
                Err(e) => Err(LlmError::ProcessingError(format!("Stream error: {}", e))),
            }
        });

        Ok(Box::new(stream))
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        // List of known Claude models
        let valid_models = [
            "claude-3-5-sonnet-20241022",
            "claude-3-5-sonnet-20240620",
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307",
            "claude-2.1",
            "claude-2.0",
            "claude-instant-1.2",
        ];

        Ok(valid_models.contains(&model))
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec![
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-sonnet-20240620".to_string(),
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-2.1".to_string(),
            "claude-2.0".to_string(),
            "claude-instant-1.2".to_string(),
        ])
    }

    fn get_provider_name(&self) -> &'static str {
        "anthropic"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true, // Claude 3 supports tools
            supports_function_calling: false,
            supports_vision: true, // Claude 3 supports vision
            supports_embeddings: false,
            max_context_tokens: Some(200_000), // Claude 3.5 Sonnet has 200K context
            supports_system_messages: true,
        }
    }
}

// Claude API request/response types

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    #[allow(dead_code)]
    id: String,
    model: String,
    content: Vec<ClaudeContent>,
    stop_reason: Option<String>,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ClaudeStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta: Option<ClaudeDelta>,
}

#[derive(Debug, Deserialize)]
struct ClaudeDelta {
    #[serde(rename = "type")]
    _delta_type: Option<String>,
    text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_config_validation() {
        // Valid config
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config.validate().is_ok());

        // Empty API key
        let config = AnthropicConfig::new(
            "".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config.validate().is_err());

        // Invalid URL
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "invalid-url".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config.validate().is_err());

        // Zero max tokens
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            0,
        );
        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn test_anthropic_adapter_creation() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );

        let adapter = AnthropicAdapter::new(config);
        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn test_anthropic_provider_capabilities() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );

        let adapter = AnthropicAdapter::new(config).unwrap();
        let capabilities = adapter.get_capabilities();

        assert!(capabilities.supports_streaming);
        assert!(capabilities.supports_tool_calling);
        assert!(capabilities.supports_vision);
        assert!(capabilities.supports_system_messages);
        assert_eq!(capabilities.max_context_tokens, Some(200_000));
        assert_eq!(adapter.get_provider_name(), "anthropic");
    }

    #[test]
    fn test_anthropic_config_with_custom_timeout() {
        let mut config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        config.timeout_seconds = 120;
        assert_eq!(config.timeout_seconds, 120);
    }

    #[test]
    fn test_anthropic_config_max_tokens_validation() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            1024,
        );
        assert!(config.validate().is_ok());
        assert_eq!(config.max_tokens, 1024);
    }

    #[test]
    fn test_anthropic_config_model_field() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-opus-20240229".to_string(),
            4096,
        );
        assert_eq!(config.model, "claude-3-opus-20240229");
    }

    #[test]
    fn test_anthropic_config_empty_model() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "".to_string(),
            4096,
        );
        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn test_anthropic_adapter_provider_name() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        let adapter = AnthropicAdapter::new(config).unwrap();
        assert_eq!(adapter.get_provider_name(), "anthropic");
    }

    #[test]
    fn test_anthropic_config_clone() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        let cloned = config.clone();
        assert_eq!(config.api_key, cloned.api_key);
        assert_eq!(config.model, cloned.model);
    }

    #[test]
    fn test_anthropic_config_debug_format() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("AnthropicConfig"));
    }
}
