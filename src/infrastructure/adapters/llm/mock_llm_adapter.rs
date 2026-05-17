// src/infrastructure/adapters/llm/mock_llm_adapter.rs
//
// Mock LLM Adapter for testing
//
// Provides a configurable mock implementation of LlmPort for integration testing
// without requiring real LLM API calls. Supports:
// - Configurable responses (single or queue)
// - Error simulation
// - Delays for timing tests
// - Call tracking
// - Token usage configuration
// - Streaming support

use async_trait::async_trait;
use chrono::Utc;
use futures::stream;
use paladin_ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities,
    StreamingResponse, TokenUsage,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

/// Response or error to be returned by the mock
#[derive(Debug, Clone)]
enum MockResponse {
    Success(String),
    Error(LlmError),
}

/// Mock LLM Adapter configuration
#[derive(Debug)]
struct MockConfig {
    /// Queue of responses/errors to return
    responses: Vec<MockResponse>,
    /// Current response index
    response_index: usize,
    /// Delay to apply before each response
    delay: Option<Duration>,
    /// Token usage configuration
    token_usage: TokenUsage,
    /// Finish reason to return
    finish_reason: FinishReason,
    /// Available models
    available_models: Vec<String>,
    /// Call count
    call_count: usize,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            responses: vec![MockResponse::Success("Mock LLM response".to_string())],
            response_index: 0,
            delay: None,
            token_usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
            finish_reason: FinishReason::Stop,
            available_models: vec!["mock-model".to_string()],
            call_count: 0,
        }
    }
}

/// Mock LLM Adapter for testing
///
/// Provides a configurable mock implementation of the LlmPort trait.
/// Use the builder pattern to configure responses, errors, delays, etc.
///
/// # Example
///
/// ```rust,no_run
/// use paladin::infrastructure::adapters::llm::mock_llm_adapter::MockLlmAdapter;
/// use paladin_ports::output::llm_port::LlmError;
/// use std::time::Duration;
///
/// let adapter = MockLlmAdapter::new()
///     .with_responses(vec![
///         "First response".to_string(),
///         "Second response".to_string(),
///     ])
///     .with_delay(Duration::from_millis(100));
/// ```
#[doc(hidden)]
#[derive(Debug)]
pub struct MockLlmAdapter {
    config: Arc<Mutex<MockConfig>>,
}

impl MockLlmAdapter {
    /// Creates a new MockLlmAdapter with default configuration
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(MockConfig::default())),
        }
    }

    /// Configure a single response
    pub fn with_response(self, response: String) -> Self {
        let mut config = self.config.lock().unwrap();
        config.responses = vec![MockResponse::Success(response)];
        config.response_index = 0;
        drop(config);
        self
    }

    /// Configure multiple responses (will be returned in sequence)
    pub fn with_responses(self, responses: Vec<String>) -> Self {
        let mut config = self.config.lock().unwrap();
        config.responses = responses.into_iter().map(MockResponse::Success).collect();
        config.response_index = 0;
        drop(config);
        self
    }

    /// Configure an error to be returned
    pub fn with_error(self, error: LlmError) -> Self {
        let mut config = self.config.lock().unwrap();
        config.responses = vec![MockResponse::Error(error)];
        config.response_index = 0;
        drop(config);
        self
    }

    /// Add both error and success responses (will be returned in order)
    pub fn with_error_then_response(self, error: LlmError, response: String) -> Self {
        let mut config = self.config.lock().unwrap();
        config.responses = vec![MockResponse::Error(error), MockResponse::Success(response)];
        config.response_index = 0;
        drop(config);
        self
    }

    /// Configure delay before each response
    pub fn with_delay(self, delay: Duration) -> Self {
        let mut config = self.config.lock().unwrap();
        config.delay = Some(delay);
        drop(config);
        self
    }

    /// Configure custom token usage
    pub fn with_token_usage(
        self,
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
    ) -> Self {
        let mut config = self.config.lock().unwrap();
        config.token_usage = TokenUsage {
            prompt_tokens,
            completion_tokens,
            total_tokens,
        };
        drop(config);
        self
    }

    /// Configure finish reason
    pub fn with_finish_reason(self, finish_reason: FinishReason) -> Self {
        let mut config = self.config.lock().unwrap();
        config.finish_reason = finish_reason;
        drop(config);
        self
    }

    /// Configure available models
    pub fn with_available_models(self, models: Vec<String>) -> Self {
        let mut config = self.config.lock().unwrap();
        config.available_models = models;
        drop(config);
        self
    }

    /// Get the number of times generate() was called
    pub fn get_call_count(&self) -> usize {
        let config = self.config.lock().unwrap();
        config.call_count
    }

    /// Reset call count and response index
    pub fn reset(&self) {
        let mut config = self.config.lock().unwrap();
        config.call_count = 0;
        config.response_index = 0;
    }

    /// Get the next response from the queue
    fn get_next_response(&self) -> Result<(String, TokenUsage, FinishReason), LlmError> {
        let mut config = self.config.lock().unwrap();
        config.call_count += 1;

        if config.responses.is_empty() {
            return Ok((
                "Default mock response".to_string(),
                config.token_usage.clone(),
                config.finish_reason.clone(),
            ));
        }

        let response = config.responses[config.response_index].clone();

        // Move to next response, wrapping around if needed
        config.response_index = (config.response_index + 1) % config.responses.len();

        match response {
            MockResponse::Success(content) => Ok((
                content,
                config.token_usage.clone(),
                config.finish_reason.clone(),
            )),
            MockResponse::Error(error) => Err(error),
        }
    }

    /// Apply configured delay if any
    async fn apply_delay(&self) {
        let delay = {
            let config = self.config.lock().unwrap();
            config.delay
        }; // MutexGuard is dropped here

        if let Some(delay) = delay {
            tokio::time::sleep(delay).await;
        }
    }
}

impl Default for MockLlmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmPort for MockLlmAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        // Apply delay if configured
        self.apply_delay().await;

        // Get next response
        let (content, token_usage, finish_reason) = self.get_next_response()?;

        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            model: request.model,
            content,
            finish_reason,
            usage: token_usage,
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>
    {
        // Apply delay if configured
        self.apply_delay().await;

        // Get the full response
        let (content, _token_usage, finish_reason) = self.get_next_response()?;

        // Split into chunks (simulate streaming)
        let chunk_size = std::cmp::max(1, content.len() / 3);
        let mut chunks = Vec::new();
        let mut remaining = content.as_str();

        while !remaining.is_empty() {
            let split_at = std::cmp::min(chunk_size, remaining.len());
            let (chunk, rest) = remaining.split_at(split_at);
            chunks.push(chunk.to_string());
            remaining = rest;
        }

        // Create stream chunks
        let stream_id = Uuid::new_v4();
        let total_chunks = chunks.len();
        let stream_chunks: Vec<_> = chunks
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                Ok(StreamingResponse {
                    id: stream_id,
                    delta: chunk,
                    finish_reason: if i == total_chunks - 1 {
                        Some(finish_reason.clone())
                    } else {
                        None
                    },
                })
            })
            .collect();

        Ok(Box::new(stream::iter(stream_chunks)))
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        let config = self.config.lock().unwrap();
        Ok(config.available_models.contains(&model.to_string()))
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        let config = self.config.lock().unwrap();
        Ok(config.available_models.clone())
    }

    fn get_provider_name(&self) -> &'static str {
        "MockLLM"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        // Mock adapter supports everything for testing purposes
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true,
            supports_function_calling: true,
            supports_vision: true,
            supports_embeddings: true,
            max_context_tokens: Some(100000),
            supports_system_messages: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_config_default() {
        let config = MockConfig::default();
        assert_eq!(config.token_usage.total_tokens, 30);
        assert!(matches!(config.finish_reason, FinishReason::Stop));
        assert_eq!(config.available_models, vec!["mock-model"]);
    }

    #[test]
    fn test_builder_pattern() {
        let adapter = MockLlmAdapter::new()
            .with_response("test".to_string())
            .with_delay(Duration::from_millis(10));

        assert_eq!(adapter.get_call_count(), 0);
    }
}
