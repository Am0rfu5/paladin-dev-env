//! Integration tests for Paladin with Garrison memory system
//!
//! These tests verify that Paladins correctly integrate with Garrison for conversation
//! context management across multiple turns.

use async_trait::async_trait;
use chrono::Utc;
use paladin::application::ports::output::garrison_port::{GarrisonPort, GarrisonStats};
use paladin::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, StreamingResponse, TokenUsage,
};
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::garrison::{
    ConversationRole, GarrisonConfig, GarrisonEntry,
};
use paladin::infrastructure::adapters::garrison::in_memory_garrison::InMemoryGarrison;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// Mock LLM Port for testing
struct MockLlmPort {
    response_content: String,
}

impl MockLlmPort {
    fn new(response: impl Into<String>) -> Self {
        Self {
            response_content: response.into(),
        }
    }
}

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            content: self.response_content.clone(),
            model: "gpt-4".to_string(),
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
            finish_reason: FinishReason::Stop,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>
    {
        unimplemented!("Streaming not used in these tests")
    }

    async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
        Ok(true)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec!["gpt-4".to_string()])
    }

    fn get_provider_name(&self) -> &'static str {
        "Mock"
    }
}

#[tokio::test]
async fn test_paladin_with_garrison_stores_conversation() {
    // Create garrison with default config
    let config = GarrisonConfig::default();
    let garrison = Arc::new(InMemoryGarrison::new(config)) as Arc<dyn GarrisonPort>;

    // Create mock LLM port
    let llm_port = Arc::new(MockLlmPort::new("I am a helpful assistant ready to code!"));

    // Build paladin WITH garrison
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a coding assistant")
        .name("TestPaladin")
        .model("gpt-4")
        .with_garrison(garrison.clone())
        .build()
        .expect("Failed to build paladin");

    // Create execution service WITH garrison
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(llm_port, circuit_breaker, Some(garrison.clone()));

    // Execute first turn
    let result = service.execute(&paladin, "Hello, can you help me?").await;
    assert!(result.is_ok(), "First execution should succeed");

    // Verify garrison contains entries
    let stats = garrison.stats().await.expect("Failed to get stats");
    assert!(
        stats.entry_count >= 2,
        "Should have at least user input and assistant response"
    );

    // Verify we can recall recent entries
    let recent = garrison
        .recall_recent(10)
        .await
        .expect("Failed to recall recent");
    assert!(!recent.is_empty(), "Should have stored entries");

    // Check that user message was stored
    let user_entries: Vec<_> = recent
        .iter()
        .filter(|e| matches!(e.role, ConversationRole::User))
        .collect();
    assert!(!user_entries.is_empty(), "Should have user entry");
    assert!(
        user_entries[0].content.contains("Hello"),
        "User entry should contain input"
    );

    // Check that assistant response was stored
    let assistant_entries: Vec<_> = recent
        .iter()
        .filter(|e| matches!(e.role, ConversationRole::Assistant))
        .collect();
    assert!(!assistant_entries.is_empty(), "Should have assistant entry");
}

#[tokio::test]
async fn test_paladin_without_garrison_single_turn() {
    // Create mock LLM port
    let llm_port = Arc::new(MockLlmPort::new("Single turn response"));

    // Build paladin WITHOUT garrison
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a helpful assistant")
        .name("SingleTurnPaladin")
        .model("gpt-4")
        .build()
        .expect("Failed to build paladin");

    // Create execution service WITHOUT garrison (None)
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(llm_port, circuit_breaker, None);

    // Execute single turn - should work without garrison
    let result = service.execute(&paladin, "Quick question").await;
    assert!(
        result.is_ok(),
        "Single turn without garrison should succeed"
    );
}

#[tokio::test]
async fn test_paladin_multi_turn_conversation() {
    // Create garrison with default config
    let config = GarrisonConfig::default();
    let garrison = Arc::new(InMemoryGarrison::new(config)) as Arc<dyn GarrisonPort>;

    // Create mock LLM port that echoes context
    let llm_port = Arc::new(MockLlmPort::new("Response based on context"));

    // Build paladin WITH garrison
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a conversational assistant")
        .name("ConversationalPaladin")
        .model("gpt-4")
        .max_loops(1) // Single loop per turn
        .with_garrison(garrison.clone())
        .build()
        .expect("Failed to build paladin");

    // Create execution service WITH garrison
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(llm_port, circuit_breaker, Some(garrison.clone()));

    // First turn
    let result1 = service.execute(&paladin, "My name is Alice").await;
    assert!(result1.is_ok(), "First turn should succeed");

    // Second turn - garrison should have previous context
    let result2 = service.execute(&paladin, "What is my name?").await;
    assert!(result2.is_ok(), "Second turn should succeed");

    // Verify garrison has all messages
    let stats = garrison.stats().await.expect("Failed to get stats");
    assert!(
        stats.entry_count >= 4,
        "Should have 2 user + 2 assistant messages"
    );

    // Verify conversation order
    let recent = garrison.recall_recent(10).await.expect("Failed to recall");
    assert!(recent.len() >= 4, "Should have at least 4 entries");
}

#[tokio::test]
async fn test_garrison_error_handling() {
    // Create garrison that will fail
    struct FailingGarrison;

    #[async_trait]
    impl GarrisonPort for FailingGarrison {
        async fn remember(
            &self,
            _entry: GarrisonEntry,
        ) -> Result<(), paladin::application::ports::output::garrison_port::GarrisonError> {
            Err(
                paladin::application::ports::output::garrison_port::GarrisonError::StorageError(
                    "Mock storage failure".to_string(),
                ),
            )
        }

        async fn recall_recent(
            &self,
            _limit: usize,
        ) -> Result<
            Vec<GarrisonEntry>,
            paladin::application::ports::output::garrison_port::GarrisonError,
        > {
            Ok(vec![])
        }

        async fn search(
            &self,
            _query: &str,
            _limit: usize,
        ) -> Result<
            Vec<GarrisonEntry>,
            paladin::application::ports::output::garrison_port::GarrisonError,
        > {
            Ok(vec![])
        }

        async fn forget_all(
            &self,
        ) -> Result<(), paladin::application::ports::output::garrison_port::GarrisonError> {
            Ok(())
        }

        async fn stats(
            &self,
        ) -> Result<GarrisonStats, paladin::application::ports::output::garrison_port::GarrisonError>
        {
            Ok(GarrisonStats {
                entry_count: 0,
                total_tokens: 0,
                size_bytes: Some(0),
            })
        }
    }

    let garrison = Arc::new(FailingGarrison) as Arc<dyn GarrisonPort>;
    let llm_port = Arc::new(MockLlmPort::new("Response"));

    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test")
        .with_garrison(garrison.clone())
        .build()
        .expect("Failed to build");

    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(llm_port, circuit_breaker, Some(garrison));

    // Execution should fail due to garrison error
    let result = service.execute(&paladin, "Test input").await;
    assert!(result.is_err(), "Should fail when garrison fails");

    // Verify error type
    if let Err(e) = result {
        assert!(
            matches!(e, PaladinError::GarrisonError(_)),
            "Should be GarrisonError variant"
        );
    }
}
