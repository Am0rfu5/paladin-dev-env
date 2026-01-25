//! Unit tests for PaladinBuilder
//! Following TDD - these tests should fail initially

use paladin::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, StreamingResponse, TokenUsage,
};
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
use paladin::core::platform::container::paladin::PaladinStatus;
use paladin::core::platform::container::paladin_config::OutputFormat;
use std::sync::Arc;

// Mock LLM Port for testing
struct MockLlmPort;

#[async_trait::async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            model: request.model,
            content: "Mock response".to_string(),
            finish_reason: FinishReason::Stop,
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
            created_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>
    {
        Err(LlmError::ProcessingError(
            "Mock does not support streaming".to_string(),
        ))
    }

    async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
        Ok(true)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec!["gpt-4".to_string(), "gpt-4-turbo".to_string()])
    }

    fn get_provider_name(&self) -> &'static str {
        "MockProvider"
    }

    fn get_capabilities(
        &self,
    ) -> paladin::application::ports::output::llm_port::ProviderCapabilities {
        paladin::application::ports::output::llm_port::ProviderCapabilities::default()
    }
}

#[test]
fn test_paladin_builder_creates_valid_paladin() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    let result = PaladinBuilder::new(llm_port)
        .system_prompt("You are a helpful assistant")
        .name("TestPaladin")
        .model("gpt-4")
        .build();

    assert!(result.is_ok(), "Builder should create valid Paladin");
    let paladin = result.unwrap();
    assert_eq!(paladin.node.system_prompt, "You are a helpful assistant");
    assert_eq!(paladin.node.name, "TestPaladin");
    assert_eq!(paladin.node.model, "gpt-4");
    assert_eq!(paladin.node.status, PaladinStatus::Idle);
}

#[test]
fn test_paladin_builder_validates_required_fields() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    // Missing system_prompt (empty default)
    let result = PaladinBuilder::new(llm_port.clone())
        .name("TestPaladin")
        .build();

    assert!(result.is_err(), "Builder should reject empty system_prompt");
    match result.unwrap_err() {
        PaladinError::ConfigurationError(msg) => {
            assert!(
                msg.contains("system prompt"),
                "Error should mention system prompt"
            );
        }
        _ => panic!("Expected ConfigurationError"),
    }

    // Explicitly empty system_prompt
    let result = PaladinBuilder::new(llm_port)
        .system_prompt("")
        .name("TestPaladin")
        .build();

    assert!(
        result.is_err(),
        "Builder should reject explicitly empty system_prompt"
    );
}

#[test]
fn test_paladin_builder_rejects_invalid_temperature() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    // Temperature too low
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .temperature(-0.1)
        .build();

    assert!(result.is_err(), "Builder should reject temperature < 0.0");
    match result.unwrap_err() {
        PaladinError::ConfigurationError(msg) => {
            assert!(
                msg.contains("temperature"),
                "Error should mention temperature"
            );
        }
        _ => panic!("Expected ConfigurationError"),
    }

    // Temperature too high
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .temperature(1.1)
        .build();

    assert!(result.is_err(), "Builder should reject temperature > 1.0");

    // Valid temperature range
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .temperature(0.0)
        .build();
    assert!(result.is_ok(), "Builder should accept temperature = 0.0");

    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .temperature(1.0)
        .build();
    assert!(result.is_ok(), "Builder should accept temperature = 1.0");

    let result = PaladinBuilder::new(llm_port)
        .system_prompt("Test prompt")
        .temperature(0.7)
        .build();
    assert!(result.is_ok(), "Builder should accept temperature = 0.7");
}

#[test]
fn test_paladin_builder_rejects_invalid_max_loops() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    // max_loops too low
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .max_loops(0)
        .build();

    assert!(result.is_err(), "Builder should reject max_loops = 0");
    match result.unwrap_err() {
        PaladinError::ConfigurationError(msg) => {
            assert!(msg.contains("max_loops"), "Error should mention max_loops");
        }
        _ => panic!("Expected ConfigurationError"),
    }

    // max_loops too high
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .max_loops(101)
        .build();

    assert!(result.is_err(), "Builder should reject max_loops > 100");

    // Valid max_loops range
    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .max_loops(1)
        .build();
    assert!(result.is_ok(), "Builder should accept max_loops = 1");

    let result = PaladinBuilder::new(llm_port.clone())
        .system_prompt("Test prompt")
        .max_loops(100)
        .build();
    assert!(result.is_ok(), "Builder should accept max_loops = 100");

    let result = PaladinBuilder::new(llm_port)
        .system_prompt("Test prompt")
        .max_loops(10)
        .build();
    assert!(result.is_ok(), "Builder should accept max_loops = 10");
}

#[test]
fn test_paladin_builder_sets_defaults() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    let paladin = PaladinBuilder::new(llm_port)
        .system_prompt("Test prompt")
        .build()
        .expect("Builder should succeed with minimal required fields");

    // Check PaladinData defaults from Default trait
    assert_eq!(paladin.node.temperature, 0.7);
    assert_eq!(paladin.node.max_loops, 3); // Default is 3, not 1
    assert_eq!(paladin.node.status, PaladinStatus::Idle);
    assert!(!paladin.node.name.is_empty(), "Should have default name");
    assert!(
        !paladin.node.user_name.is_empty(),
        "Should have default user_name"
    );
}

#[test]
fn test_paladin_builder_method_chaining() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    // Test fluent interface with multiple chained methods
    let result = PaladinBuilder::new(llm_port)
        .system_prompt("You are a coding assistant")
        .name("CodePaladin")
        .user_name("Developer")
        .model("gpt-4-turbo")
        .temperature(0.8)
        .max_loops(5)
        .add_stop_word("STOP")
        .add_stop_word("END")
        .retry_attempts(5)
        .timeout_seconds(600)
        .enable_planning(true)
        .output_format(OutputFormat::Json)
        .build();

    assert!(result.is_ok(), "Builder should support method chaining");
    let paladin = result.unwrap();

    assert_eq!(paladin.node.system_prompt, "You are a coding assistant");
    assert_eq!(paladin.node.name, "CodePaladin");
    assert_eq!(paladin.node.user_name, "Developer");
    assert_eq!(paladin.node.model, "gpt-4-turbo");
    assert_eq!(paladin.node.temperature, 0.8);
    assert_eq!(paladin.node.max_loops, 5);
    assert_eq!(paladin.node.stop_words.len(), 2);
    assert!(paladin.node.stop_words.contains(&"STOP".to_string()));
    assert!(paladin.node.stop_words.contains(&"END".to_string()));
}
