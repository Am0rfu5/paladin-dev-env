//! Mock LLM adapter for testing Battalion and Commander
//!
//! Provides a configurable mock implementation of LlmPort that returns
//! pre-configured responses in sequence, tracks invocation counts, and
//! supports both success and error scenarios.

use async_trait::async_trait;
use chrono::Utc;
use futures::stream;
use paladin::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, StreamingResponse, TokenUsage,
};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Mock LLM adapter with configurable responses for testing
///
/// Responses are queued and returned in FIFO order. When the queue is empty,
/// returns a default success response.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
/// let mock = Arc::new(MockLlmAdapter::new());
/// mock.add_success("First response");
/// mock.add_success("Second response");
///
/// // First call returns "First response", second returns "Second response"
/// // Third call returns default response
/// ```
#[derive(Clone)]
pub struct MockLlmAdapter {
    responses: Arc<Mutex<VecDeque<Result<String, LlmError>>>>,
    call_count: Arc<Mutex<usize>>,
}

impl MockLlmAdapter {
    /// Create a new MockLlmAdapter with empty response queue
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(VecDeque::new())),
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add a response to the queue (success or error)
    pub fn add_response(&self, response: Result<String, LlmError>) {
        self.responses.lock().unwrap().push_back(response);
    }

    /// Add a success response to the queue
    pub fn add_success(&self, content: impl Into<String>) {
        self.add_response(Ok(content.into()));
    }

    /// Add a failure response to the queue
    pub fn add_failure(&self, error: LlmError) {
        self.add_response(Err(error));
    }

    /// Get the number of times generate() was called
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Reset the mock: clear responses and reset call count
    pub fn reset(&self) {
        self.responses.lock().unwrap().clear();
        *self.call_count.lock().unwrap() = 0;
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
        // Increment call count
        *self.call_count.lock().unwrap() += 1;

        // Pop next response from queue, or return default if empty
        let response_content = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| Ok("Mock LLM response".to_string()))?;

        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            model: request.model,
            content: response_content,
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
        // Increment call count
        *self.call_count.lock().unwrap() += 1;

        // Get response
        let response_content = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| Ok("Mock LLM streaming response".to_string()))?;

        // Create streaming response
        let response = StreamingResponse {
            id: request.id,
            delta: response_content,
            finish_reason: Some(FinishReason::Stop),
        };

        Ok(Box::new(stream::once(async move { Ok(response) })))
    }

    async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
        Ok(true)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec!["mock-model".to_string()])
    }

    fn get_provider_name(&self) -> &'static str {
        "mock"
    }

    fn get_capabilities(
        &self,
    ) -> paladin::application::ports::output::llm_port::ProviderCapabilities {
        paladin::application::ports::output::llm_port::ProviderCapabilities::default()
    }
}

/// Helper function to create a test Paladin with a mock LLM adapter
pub fn create_test_paladin_with_mock(
    name: impl Into<String>,
    _mock: Arc<MockLlmAdapter>,
) -> paladin::core::platform::container::paladin::Paladin {
    use paladin::core::base::entity::node::Node;
    use paladin::core::platform::container::paladin::{MaxLoops, PaladinData, PaladinStatus};

    let data = PaladinData {
        system_prompt: "Test prompt".to_string(),
        name: name.into(),
        user_name: "test".to_string(),
        model: "mock-model".to_string(),
        temperature: 0.7,
        max_loops: MaxLoops::Fixed(3),
        stop_words: vec![],
        status: PaladinStatus::Idle,
        vision_enabled: false,
        ..Default::default()
    };
    Node::new(data, None)
}

/// Helper function to create a mock pre-configured with success responses
pub fn create_mock_with_responses(responses: Vec<&str>) -> Arc<MockLlmAdapter> {
    let mock = Arc::new(MockLlmAdapter::new());
    for response in responses {
        mock.add_success(response);
    }
    mock
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin::core::platform::container::prompt::{PromptItem, PromptType, UserPrompt};

    fn create_test_prompt() -> PromptItem {
        PromptItem::new(PromptType::User(UserPrompt {
            query: "test prompt".to_string(),
            context: None,
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn test_mock_llm_adapter_returns_configured_responses() {
        let mock = MockLlmAdapter::new();
        mock.add_success("First response");
        mock.add_success("Second response");
        mock.add_success("Third response");

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "mock".to_string(),
            prompt: create_test_prompt(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response1 = mock.generate(request.clone()).await.unwrap();
        assert_eq!(response1.content, "First response");

        let response2 = mock.generate(request.clone()).await.unwrap();
        assert_eq!(response2.content, "Second response");

        let response3 = mock.generate(request.clone()).await.unwrap();
        assert_eq!(response3.content, "Third response");

        // Fourth call returns default
        let response4 = mock.generate(request).await.unwrap();
        assert_eq!(response4.content, "Mock LLM response");
    }

    #[tokio::test]
    async fn test_mock_llm_adapter_tracks_call_count() {
        let mock = MockLlmAdapter::new();
        assert_eq!(mock.call_count(), 0);

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "mock".to_string(),
            prompt: create_test_prompt(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        mock.generate(request.clone()).await.unwrap();
        assert_eq!(mock.call_count(), 1);

        mock.generate(request.clone()).await.unwrap();
        assert_eq!(mock.call_count(), 2);

        mock.generate(request).await.unwrap();
        assert_eq!(mock.call_count(), 3);
    }

    #[tokio::test]
    async fn test_mock_llm_adapter_handles_failures() {
        let mock = MockLlmAdapter::new();
        mock.add_success("Success response");
        mock.add_failure(LlmError::ProcessingError("Simulated LLM error".to_string()));
        mock.add_success("Another success");

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "mock".to_string(),
            prompt: create_test_prompt(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response1 = mock.generate(request.clone()).await;
        assert!(response1.is_ok());
        assert_eq!(response1.unwrap().content, "Success response");

        let response2 = mock.generate(request.clone()).await;
        assert!(response2.is_err());
        if let Err(LlmError::ProcessingError(msg)) = response2 {
            assert_eq!(msg, "Simulated LLM error");
        } else {
            panic!("Expected ProcessingError");
        }

        let response3 = mock.generate(request).await;
        assert!(response3.is_ok());
        assert_eq!(response3.unwrap().content, "Another success");
    }

    #[tokio::test]
    async fn test_mock_llm_adapter_reset() {
        let mock = MockLlmAdapter::new();
        mock.add_success("Response 1");
        mock.add_success("Response 2");

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "mock".to_string(),
            prompt: create_test_prompt(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        mock.generate(request.clone()).await.unwrap();
        mock.generate(request.clone()).await.unwrap();
        assert_eq!(mock.call_count(), 2);

        mock.reset();
        assert_eq!(mock.call_count(), 0);

        // After reset, should return default response
        let response = mock.generate(request).await.unwrap();
        assert_eq!(response.content, "Mock LLM response");
    }

    #[test]
    fn test_create_mock_with_responses() {
        let mock = create_mock_with_responses(vec!["First", "Second", "Third"]);
        // Verify responses are queued
        assert_eq!(mock.responses.lock().unwrap().len(), 3);
    }

    #[test]
    fn test_create_test_paladin_with_mock() {
        let mock = Arc::new(MockLlmAdapter::new());
        let paladin = create_test_paladin_with_mock("test_paladin", mock);
        // Access inner data via .node field
        assert_eq!(paladin.node.name, "test_paladin");
        assert_eq!(paladin.node.model, "mock-model");
    }
}
