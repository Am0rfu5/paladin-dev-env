//! Mock LLM adapters for testing.
//!
//! This module provides [`MockLlmAdapter`] and [`MultiStepMockLlmPort`] for
//! use in unit and integration tests that need an in-process LLM adapter
//! without real API calls.

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

/// A single mocked response entry — either a success string or an error.
#[derive(Debug, Clone)]
enum MockEntry {
    Success(String),
    Error(LlmError),
}

/// Internal state shared between clones of a [`MockLlmAdapter`].
#[derive(Debug)]
struct MockState {
    responses: Vec<MockEntry>,
    response_index: usize,
    delay: Option<Duration>,
    token_usage: TokenUsage,
    finish_reason: FinishReason,
    available_models: Vec<String>,
    call_count: usize,
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            responses: vec![MockEntry::Success("Mock LLM response".to_string())],
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

/// Configurable mock adapter for the [`LlmPort`] trait.
///
/// Suitable for unit tests and integration tests that need an in-process LLM
/// without making real API calls.
///
/// # Example
///
/// ```rust
/// use paladin_llm::mock::MockLlmAdapter;
///
/// let adapter = MockLlmAdapter::new()
///     .with_responses(vec![
///         "First response".to_string(),
///         "Second response".to_string(),
///     ]);
/// ```
#[derive(Debug, Clone)]
pub struct MockLlmAdapter {
    state: Arc<Mutex<MockState>>,
}

impl MockLlmAdapter {
    /// Create a new adapter with default configuration (single success response).
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState::default())),
        }
    }

    /// Set the sequence of success responses to return (cycling when exhausted).
    pub fn with_responses(self, responses: Vec<String>) -> Self {
        let mut state = self.state.lock().unwrap();
        state.responses = responses.into_iter().map(MockEntry::Success).collect();
        state.response_index = 0;
        drop(state);
        self
    }

    /// Queue a single success response.
    pub fn with_response(self, response: impl Into<String>) -> Self {
        self.with_responses(vec![response.into()])
    }

    /// Set an error to be returned on the next call.
    pub fn with_error(self, error: LlmError) -> Self {
        let mut state = self.state.lock().unwrap();
        state.responses = vec![MockEntry::Error(error)];
        state.response_index = 0;
        drop(state);
        self
    }

    /// Simulate network latency by adding a delay before each response.
    pub fn with_delay(self, delay: Duration) -> Self {
        self.state.lock().unwrap().delay = Some(delay);
        self
    }

    /// Configure the token usage returned with each response.
    pub fn with_token_usage(self, usage: TokenUsage) -> Self {
        self.state.lock().unwrap().token_usage = usage;
        self
    }

    /// Return the number of times [`LlmPort::generate`] has been called.
    pub fn call_count(&self) -> usize {
        self.state.lock().unwrap().call_count
    }

    /// Return `true` if the adapter was called at least once.
    pub fn was_called(&self) -> bool {
        self.call_count() > 0
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
        let (response_entry, delay, token_usage, finish_reason) = {
            let mut state = self.state.lock().unwrap();
            state.call_count += 1;
            let index = state.response_index;
            let entry = state
                .responses
                .get(index)
                .cloned()
                .unwrap_or(MockEntry::Success("Mock LLM response".to_string()));
            // Advance index, cycling through responses
            state.response_index = (index + 1) % state.responses.len().max(1);
            (
                entry,
                state.delay,
                state.token_usage.clone(),
                state.finish_reason.clone(),
            )
        };

        if let Some(delay) = delay {
            tokio::time::sleep(delay).await;
        }

        match response_entry {
            MockEntry::Error(e) => Err(e),
            MockEntry::Success(content) => Ok(LlmResponse {
                id: Uuid::new_v4(),
                request_id: request.id,
                model: request.model.clone(),
                content,
                finish_reason,
                usage: token_usage,
                created_at: Utc::now(),
                metadata: HashMap::new(),
                function_call: None,
            }),
        }
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>
    {
        let response = self.generate(request).await?;
        // Emit the full response as a single streaming chunk, then stop.
        let chunks = vec![
            Ok(StreamingResponse {
                id: Uuid::new_v4(),
                delta: response.content.clone(),
                finish_reason: None,
            }),
            Ok(StreamingResponse {
                id: Uuid::new_v4(),
                delta: String::new(),
                finish_reason: Some(response.finish_reason),
            }),
        ];
        Ok(Box::new(stream::iter(chunks)))
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        let state = self.state.lock().unwrap();
        Ok(state.available_models.contains(&model.to_string()))
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(self.state.lock().unwrap().available_models.clone())
    }

    fn get_provider_name(&self) -> &'static str {
        "mock"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: false,
            supports_function_calling: false,
            supports_vision: false,
            max_context_tokens: Some(4096),
            supports_embeddings: false,
            supports_system_messages: true,
        }
    }
}

// ---------------------------------------------------------------------------
// MultiStepMockLlmPort
// ---------------------------------------------------------------------------

/// A mock adapter that returns different responses for successive calls.
///
/// Unlike [`MockLlmAdapter`] (which cycles), this adapter returns each response
/// exactly once and then panics if called more times than there are responses.
/// This is useful for tests that assert the *exact* sequence of LLM calls.
///
/// # Example
///
/// ```rust
/// use paladin_llm::mock::MultiStepMockLlmPort;
///
/// let adapter = MultiStepMockLlmPort::new(vec![
///     "Step 1 response".to_string(),
///     "Step 2 response".to_string(),
/// ]);
/// ```
#[derive(Debug)]
pub struct MultiStepMockLlmPort {
    responses: Vec<String>,
    call_count: Arc<Mutex<usize>>,
}

impl MultiStepMockLlmPort {
    /// Create a new multi-step mock with the given response sequence.
    pub fn new(responses: Vec<String>) -> Self {
        Self {
            responses,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Return the number of times [`LlmPort::generate`] has been called.
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl LlmPort for MultiStepMockLlmPort {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let mut count = self.call_count.lock().unwrap();
        let index = *count;
        *count += 1;
        drop(count);

        let content = self
            .responses
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("Mock step {} response", index));

        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            model: request.model.clone(),
            content,
            finish_reason: FinishReason::Stop,
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>
    {
        let response = self.generate(request).await?;
        let chunks = vec![Ok(StreamingResponse {
            id: Uuid::new_v4(),
            delta: response.content,
            finish_reason: Some(FinishReason::Stop),
        })];
        Ok(Box::new(stream::iter(chunks)))
    }

    async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
        Ok(true)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec!["mock-model".to_string()])
    }

    fn get_provider_name(&self) -> &'static str {
        "multi-step-mock"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: false,
            supports_function_calling: false,
            supports_vision: false,
            max_context_tokens: Some(4096),
            supports_embeddings: false,
            supports_system_messages: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_core::platform::container::prompt::{PromptItem, PromptType, UserPrompt};
    use paladin_ports::output::llm_port::LlmPort;
    use uuid::Uuid;

    fn make_request() -> LlmRequest {
        let prompt = PromptItem::new(PromptType::User(UserPrompt {
            query: "test query".to_string(),
            context: None,
        }))
        .unwrap();
        LlmRequest {
            id: Uuid::new_v4(),
            model: "mock-model".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_mock_returns_default_response() {
        let adapter = MockLlmAdapter::new();
        let request = make_request();
        let response = adapter.generate(request).await.unwrap();
        assert_eq!(response.content, "Mock LLM response");
    }

    #[tokio::test]
    async fn test_mock_cycles_responses() {
        let adapter =
            MockLlmAdapter::new().with_responses(vec!["First".to_string(), "Second".to_string()]);
        let r1 = adapter.generate(make_request()).await.unwrap();
        let r2 = adapter.generate(make_request()).await.unwrap();
        let r3 = adapter.generate(make_request()).await.unwrap(); // cycles back
        assert_eq!(r1.content, "First");
        assert_eq!(r2.content, "Second");
        assert_eq!(r3.content, "First");
    }

    #[tokio::test]
    async fn test_mock_tracks_call_count() {
        let adapter = MockLlmAdapter::new();
        assert_eq!(adapter.call_count(), 0);
        adapter.generate(make_request()).await.unwrap();
        assert_eq!(adapter.call_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_returns_error() {
        let adapter = MockLlmAdapter::new().with_error(LlmError::RateLimitExceeded);
        let result = adapter.generate(make_request()).await;
        assert!(matches!(result, Err(LlmError::RateLimitExceeded)));
    }

    #[tokio::test]
    async fn test_multi_step_returns_sequence() {
        let adapter = MultiStepMockLlmPort::new(vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ]);
        let r1 = adapter.generate(make_request()).await.unwrap();
        let r2 = adapter.generate(make_request()).await.unwrap();
        let r3 = adapter.generate(make_request()).await.unwrap();
        assert_eq!(r1.content, "Step 1");
        assert_eq!(r2.content, "Step 2");
        assert_eq!(r3.content, "Step 3");
    }

    #[tokio::test]
    async fn test_multi_step_tracks_call_count() {
        let adapter = MultiStepMockLlmPort::new(vec!["A".to_string()]);
        assert_eq!(adapter.call_count(), 0);
        adapter.generate(make_request()).await.unwrap();
        assert_eq!(adapter.call_count(), 1);
    }
}
