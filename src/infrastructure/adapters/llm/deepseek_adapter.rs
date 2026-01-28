// src/infrastructure/adapters/llm/deepseek_adapter.rs
//
// DeepSeek LLM Adapter
//
// Provides integration with DeepSeek's API, which is OpenAI-compatible.
// Supports standard completions, streaming, and all core LlmPort functionality.

use async_trait::async_trait;
use chrono::Utc;
use futures::{Stream, StreamExt};
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
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
use crate::core::platform::container::prompt::{PromptItem, PromptType};

/// Configuration for DeepSeek LLM adapter
#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    /// API key for DeepSeek authentication
    pub api_key: String,
    /// Base URL for DeepSeek API
    pub base_url: String,
    /// Default model to use
    pub model: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl DeepSeekConfig {
    /// Load configuration from environment variables
    ///
    /// # Environment Variables
    /// - `DEEPSEEK_API_KEY` (required): DeepSeek API key
    /// - `DEEPSEEK_BASE_URL` (optional): API base URL, defaults to https://api.deepseek.com/v1
    /// - `DEEPSEEK_MODEL` (optional): Default model, defaults to deepseek-chat
    /// - `DEEPSEEK_TIMEOUT_SECONDS` (optional): Request timeout, defaults to 60
    ///
    /// # Errors
    /// Returns error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| "DEEPSEEK_API_KEY environment variable not set")?;

        let base_url = env::var("DEEPSEEK_BASE_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com/v1".to_string());

        let model = env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());

        let timeout_seconds = env::var("DEEPSEEK_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map_err(|_| "Invalid DEEPSEEK_TIMEOUT_SECONDS value")?;

        let config = Self {
            api_key,
            base_url,
            model,
            timeout_seconds,
        };

        config.validate()?;
        Ok(config)
    }

    /// Create configuration with custom values
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            timeout_seconds: 60,
        }
    }

    /// Validate the configuration
    fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }

        if !self.base_url.starts_with("http") {
            return Err("Base URL must start with http or https".to_string());
        }

        if self.model.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        Ok(())
    }
}

/// DeepSeek API request structures (OpenAI-compatible format)
#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

/// DeepSeek API response structures
#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    _id: String,
    model: String,
    choices: Vec<DeepSeekChoice>,
    usage: DeepSeekUsage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    _index: u32,
    message: DeepSeekMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// DeepSeek streaming response structures
#[derive(Debug, Deserialize)]
struct DeepSeekStreamResponse {
    _id: String,
    choices: Vec<DeepSeekStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamChoice {
    delta: DeepSeekStreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamDelta {
    content: Option<String>,
}

/// DeepSeek LLM Adapter
///
/// Implements the LlmPort trait for DeepSeek's API.
/// DeepSeek provides OpenAI-compatible API endpoints.
///
/// # Example
///
/// ```rust,no_run
/// use paladin::infrastructure::adapters::llm::deepseek_adapter::{DeepSeekAdapter, DeepSeekConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = DeepSeekConfig::from_env()?;
/// let adapter = DeepSeekAdapter::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct DeepSeekAdapter {
    client: Client,
    config: DeepSeekConfig,
}

impl DeepSeekAdapter {
    /// Create a new DeepSeek adapter
    ///
    /// # Arguments
    /// * `config` - DeepSeek configuration
    ///
    /// # Errors
    /// Returns error if configuration is invalid or client cannot be created
    pub fn new(config: DeepSeekConfig) -> Result<Self, LlmError> {
        config.validate().map_err(|e| {
            LlmError::AuthenticationError(format!("Invalid DeepSeek configuration: {}", e))
        })?;

        let timeout = Duration::from_secs(config.timeout_seconds);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key)).map_err(|e| {
                LlmError::AuthenticationError(format!("Invalid API key format: {}", e))
            })?,
        );

        let client = Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()
            .map_err(|e| LlmError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Build DeepSeek API request from LlmRequest
    fn build_request(&self, request: &LlmRequest) -> Result<DeepSeekRequest, LlmError> {
        let messages = self.convert_prompt_to_messages(&request.prompt)?;
        let params = &request.prompt.node.node.parameters;

        Ok(DeepSeekRequest {
            model: request.model.clone(),
            messages,
            temperature: params.temperature,
            max_tokens: params.max_tokens,
            top_p: params.top_p,
            frequency_penalty: params.frequency_penalty,
            presence_penalty: params.presence_penalty,
            stream: request.stream,
        })
    }

    /// Convert PromptItem to DeepSeek messages
    fn convert_prompt_to_messages(
        &self,
        prompt: &PromptItem,
    ) -> Result<Vec<DeepSeekMessage>, LlmError> {
        let mut messages = Vec::new();

        match &prompt.node.node.prompt_type {
            PromptType::System(system_prompt) => {
                messages.push(DeepSeekMessage {
                    role: "system".to_string(),
                    content: system_prompt.instructions.clone(),
                });
            }
            PromptType::User(user_prompt) => {
                messages.push(DeepSeekMessage {
                    role: "user".to_string(),
                    content: user_prompt.query.clone(),
                });
            }
            PromptType::Text(text_prompt) => {
                messages.push(DeepSeekMessage {
                    role: match text_prompt.role {
                        crate::core::platform::container::prompt::PromptRole::System => "system",
                        crate::core::platform::container::prompt::PromptRole::User => "user",
                        crate::core::platform::container::prompt::PromptRole::Assistant => {
                            "assistant"
                        }
                        crate::core::platform::container::prompt::PromptRole::Function => {
                            "function"
                        }
                    }
                    .to_string(),
                    content: text_prompt.content.clone(),
                });
            }
            PromptType::Assistant(assistant_prompt) => {
                messages.push(DeepSeekMessage {
                    role: "assistant".to_string(),
                    content: assistant_prompt.response.clone(),
                });
            }
            PromptType::Function(function_prompt) => {
                messages.push(DeepSeekMessage {
                    role: "function".to_string(),
                    content: function_prompt.function_name.clone(),
                });
            }
        }

        if messages.is_empty() {
            return Err(LlmError::InvalidPrompt(
                "Prompt must contain at least one message".to_string(),
            ));
        }

        Ok(messages)
    }

    /// Map DeepSeek finish reason to our FinishReason enum
    fn map_finish_reason(reason: Option<String>) -> FinishReason {
        match reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("function_call") => FinishReason::FunctionCall,
            Some(other) => FinishReason::Error(format!("Unknown finish reason: {}", other)),
            None => FinishReason::Stop,
        }
    }

    /// Map DeepSeek API errors to LlmError
    fn map_error(&self, status: u16, message: &str) -> LlmError {
        match status {
            401 => LlmError::AuthenticationError(format!(
                "Invalid API key for DeepSeek. Check DEEPSEEK_API_KEY environment variable. Error: {}",
                message
            )),
            429 => LlmError::RateLimitExceeded,
            404 => LlmError::ModelNotAvailable(message.to_string()),
            400 => LlmError::InvalidPrompt(message.to_string()),
            _ => LlmError::ProcessingError(format!("DeepSeek API error ({}): {}", status, message)),
        }
    }

    /// Perform API call with retry logic
    async fn call_api_with_retry<F, Fut, T>(
        &self,
        operation: F,
        max_retries: u32,
    ) -> Result<T, LlmError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, LlmError>>,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(LlmError::RateLimitExceeded) if attempt < max_retries => {
                    // Exponential backoff with jitter
                    let backoff = Duration::from_millis(100 * 2_u64.pow(attempt));
                    let jitter = Duration::from_millis(rand::random::<u64>() % 100);
                    tokio::time::sleep(backoff + jitter).await;
                    last_error = Some(LlmError::RateLimitExceeded);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            LlmError::ProcessingError("Retry logic failed unexpectedly".to_string())
        }))
    }
}

#[async_trait]
impl LlmPort for DeepSeekAdapter {
    /// Generate completion using DeepSeek API
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let api_request = self.build_request(&request)?;
        let url = format!("{}/chat/completions", self.config.base_url);

        let operation = || async {
            let response = self
                .client
                .post(&url)
                .json(&api_request)
                .send()
                .await
                .map_err(|e| {
                    if e.is_timeout() {
                        LlmError::Timeout(format!(
                            "DeepSeek API request timed out after {} seconds",
                            self.config.timeout_seconds
                        ))
                    } else {
                        LlmError::NetworkError(format!("Failed to send request to DeepSeek: {}", e))
                    }
                })?;

            let status = response.status();

            if !status.is_success() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(self.map_error(status.as_u16(), &error_text));
            }

            let api_response: DeepSeekResponse = response.json().await.map_err(|e| {
                LlmError::ProcessingError(format!("Failed to parse DeepSeek response: {}", e))
            })?;

            let choice = api_response.choices.first().ok_or_else(|| {
                LlmError::ProcessingError("DeepSeek response contained no choices".to_string())
            })?;

            Ok(LlmResponse {
                id: Uuid::new_v4(),
                request_id: request.id,
                model: api_response.model,
                content: choice.message.content.clone(),
                finish_reason: Self::map_finish_reason(choice.finish_reason.clone()),
                usage: TokenUsage {
                    prompt_tokens: api_response.usage.prompt_tokens,
                    completion_tokens: api_response.usage.completion_tokens,
                    total_tokens: api_response.usage.total_tokens,
                },
                created_at: Utc::now(),
                metadata: HashMap::new(),
                function_call: None,
            })
        };

        self.call_api_with_retry(operation, 3).await
    }

    /// Generate streaming completion using DeepSeek API
    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError> {
        let mut api_request = self.build_request(&request)?;
        api_request.stream = true;

        let url = format!("{}/chat/completions", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .json(&api_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    LlmError::Timeout(format!(
                        "DeepSeek API request timed out after {} seconds",
                        self.config.timeout_seconds
                    ))
                } else {
                    LlmError::NetworkError(format!(
                        "Failed to send streaming request to DeepSeek: {}",
                        e
                    ))
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(self.map_error(status.as_u16(), &error_text));
        }

        let stream = response.bytes_stream();

        let llm_stream = stream.map(|chunk_result| {
            match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);

                    // Parse SSE format: "data: {json}\n\n"
                    for line in text.lines() {
                        if let Some(json_str) = line.strip_prefix("data: ") {
                            if json_str.trim() == "[DONE]" {
                                return Ok(StreamingResponse {
                                    id: Uuid::new_v4(),
                                    delta: String::new(),
                                    finish_reason: Some(FinishReason::Stop),
                                });
                            }

                            match serde_json::from_str::<DeepSeekStreamResponse>(json_str) {
                                Ok(response) => {
                                    if let Some(choice) = response.choices.first() {
                                        let content =
                                            choice.delta.content.clone().unwrap_or_default();
                                        return Ok(StreamingResponse {
                                            id: Uuid::new_v4(),
                                            delta: content,
                                            finish_reason: choice
                                                .finish_reason
                                                .as_ref()
                                                .map(|r| Self::map_finish_reason(Some(r.clone()))),
                                        });
                                    }
                                }
                                Err(e) => {
                                    return Err(LlmError::ProcessingError(format!(
                                        "Failed to parse streaming response: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    }

                    Ok(StreamingResponse {
                        id: Uuid::new_v4(),
                        delta: String::new(),
                        finish_reason: None,
                    })
                }
                Err(e) => Err(LlmError::NetworkError(format!("Stream error: {}", e))),
            }
        });

        Ok(Box::new(llm_stream))
    }

    /// Validate if a model is available
    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        let available_models = self.get_available_models().await?;
        Ok(available_models.contains(&model.to_string()))
    }

    /// Get list of available DeepSeek models
    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        // DeepSeek's primary models
        Ok(vec![
            "deepseek-chat".to_string(),
            "deepseek-coder".to_string(),
        ])
    }

    /// Get provider name
    fn get_provider_name(&self) -> &'static str {
        "deepseek"
    }

    /// Get provider capabilities
    ///
    /// DeepSeek supports:
    /// - Streaming responses via SSE
    /// - System messages
    /// - 64K context window
    ///
    /// Currently does not support:
    /// - Tool/function calling (may be added in future)
    /// - Vision/image inputs
    /// - Embeddings (separate API)
    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: false,
            supports_function_calling: false,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(64000),
            supports_system_messages: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepseek_config_validation() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_deepseek_config_empty_api_key() {
        let config = DeepSeekConfig::new(
            "".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_deepseek_config_invalid_url() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "invalid-url".to_string(),
            "deepseek-chat".to_string(),
        );
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_deepseek_adapter_creation() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        let adapter = DeepSeekAdapter::new(config);
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_deepseek_provider_name() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        let adapter = DeepSeekAdapter::new(config).unwrap();
        assert_eq!(adapter.get_provider_name(), "deepseek");
    }

    #[test]
    fn test_deepseek_capabilities() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        let adapter = DeepSeekAdapter::new(config).unwrap();
        let capabilities = adapter.get_capabilities();

        assert!(capabilities.supports_streaming);
        assert!(!capabilities.supports_function_calling); // DeepSeek doesn't support function calling
        assert!(capabilities.supports_system_messages);
        assert_eq!(capabilities.max_context_tokens, Some(64000));
    }

    #[test]
    fn test_deepseek_config_with_custom_model() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-coder".to_string(),
        );
        assert_eq!(config.model, "deepseek-coder");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_deepseek_config_empty_model() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "".to_string(),
        );
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_deepseek_config_with_custom_timeout() {
        let mut config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        config.timeout_seconds = 120;
        assert_eq!(config.timeout_seconds, 120);
    }

    #[test]
    fn test_deepseek_config_clone() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        let cloned = config.clone();
        assert_eq!(config.api_key, cloned.api_key);
        assert_eq!(config.model, cloned.model);
    }

    #[test]
    fn test_deepseek_config_debug_format() {
        let config = DeepSeekConfig::new(
            "test-key".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "deepseek-chat".to_string(),
        );
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("DeepSeekConfig"));
    }
}
