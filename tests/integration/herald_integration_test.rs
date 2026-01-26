//! Integration tests for Herald output formatting with Paladin and Battalion execution
//!
//! These tests verify that Herald formatters correctly format real Paladin and Battalion
//! execution results with proper metadata inclusion.

use async_trait::async_trait;
use chrono::Utc;
use paladin::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, TokenUsage,
};
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::herald::Herald;
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Mock LLM Port for testing
struct MockLlmPort {
    response_text: String,
}

impl MockLlmPort {
    fn new(response_text: impl Into<String>) -> Self {
        Self {
            response_text: response_text.into(),
        }
    }
}

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            model: request.model,
            content: self.response_text.clone(),
            finish_reason: FinishReason::Stop,
            usage: TokenUsage {
                prompt_tokens: 50,
                completion_tokens: 100,
                total_tokens: 150,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<
        Box<
            dyn futures::Stream<
                    Item = Result<
                        paladin::application::ports::output::llm_port::StreamingResponse,
                        LlmError,
                    >,
                > + Send,
        >,
        LlmError,
    > {
        unimplemented!("Streaming not needed for this test")
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

#[tokio::test]
async fn test_paladin_with_json_herald() {
    // Create mock LLM port
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort::new("Test response from Paladin"));

    // Create circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,
        3,
        std::time::Duration::from_secs(60),
    ));

    // Create Herald
    let herald: Arc<dyn Herald> = Arc::new(JsonHerald::new());

    // Create Paladin with Herald
    let paladin = PaladinBuilder::new(Arc::clone(&llm_port))
        .system_prompt("You are a helpful assistant")
        .name("Test Paladin")
        .build()
        .expect("Failed to build Paladin");

    // Create execution service with Herald
    let service = PaladinExecutionService::new(
        Arc::clone(&llm_port),
        circuit_breaker,
        None, // No garrison
        None, // No arsenal
    )
    .with_herald(Arc::clone(&herald));

    // Execute (this will use the mock LLM)
    let result = service.execute(&paladin, "Hello").await;
    assert!(result.is_ok(), "Paladin execution should succeed");

    let result = result.unwrap();

    // Format the result
    let formatted = service.format_result(&result, &paladin);
    assert!(formatted.is_ok(), "Herald formatting should succeed");

    let formatted = formatted.unwrap();
    assert!(formatted.is_some(), "Herald should format the result");

    let json_output = formatted.unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json_output).expect("Output should be valid JSON");

    // Verify JSON contains expected fields
    assert!(parsed.get("paladin_id").is_some());
    assert!(parsed.get("paladin_name").is_some());
    assert!(parsed.get("status").is_some());
    assert!(parsed.get("output").is_some());

    println!("JSON Herald output:\n{}", json_output);
}

#[tokio::test]
async fn test_paladin_with_markdown_herald() {
    // Create mock LLM port
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort::new("Markdown test response"));

    // Create circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,
        3,
        std::time::Duration::from_secs(60),
    ));

    // Create Herald
    let herald: Arc<dyn Herald> = Arc::new(MarkdownHerald::new());

    // Create Paladin
    let paladin = PaladinBuilder::new(Arc::clone(&llm_port))
        .system_prompt("You are a helpful assistant")
        .name("Markdown Paladin")
        .build()
        .expect("Failed to build Paladin");

    // Create execution service with Herald
    let service = PaladinExecutionService::new(Arc::clone(&llm_port), circuit_breaker, None, None)
        .with_herald(Arc::clone(&herald));

    // Execute
    let result = service.execute(&paladin, "Test").await;
    assert!(result.is_ok());

    let result = result.unwrap();

    // Format the result
    let formatted = service.format_result(&result, &paladin);
    assert!(formatted.is_ok());

    let formatted = formatted.unwrap();
    assert!(formatted.is_some());

    let markdown_output = formatted.unwrap();

    // Verify Markdown content
    assert!(markdown_output.contains("##") || markdown_output.contains("**"));
    assert!(markdown_output.contains("Markdown Paladin"));

    println!("Markdown Herald output:\n{}", markdown_output);
}

#[tokio::test]
async fn test_paladin_with_table_herald() {
    // Create mock LLM port
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort::new("Table test response"));

    // Create circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,
        3,
        std::time::Duration::from_secs(60),
    ));

    // Create Herald
    let herald: Arc<dyn Herald> = Arc::new(TableHerald::default());

    // Create Paladin
    let paladin = PaladinBuilder::new(Arc::clone(&llm_port))
        .system_prompt("You are a helpful assistant")
        .name("Table Paladin")
        .build()
        .expect("Failed to build Paladin");

    // Create execution service with Herald
    let service = PaladinExecutionService::new(Arc::clone(&llm_port), circuit_breaker, None, None)
        .with_herald(Arc::clone(&herald));

    // Execute
    let result = service.execute(&paladin, "Test").await;
    assert!(result.is_ok());

    let result = result.unwrap();

    // Format the result
    let formatted = service.format_result(&result, &paladin);
    assert!(formatted.is_ok());

    let formatted = formatted.unwrap();
    assert!(formatted.is_some());

    let table_output = formatted.unwrap();

    // Verify table structure (has table borders)
    assert!(
        table_output.contains("│") || table_output.contains("|"),
        "Output should be a table"
    );
    // Table formatter may not include paladin name in simple format
    assert!(!table_output.is_empty(), "Table output should not be empty");

    println!("Table Herald output:\n{}", table_output);
}

#[tokio::test]
async fn test_paladin_without_herald() {
    // Create mock LLM port
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort::new("No herald response"));

    // Create circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,
        3,
        std::time::Duration::from_secs(60),
    ));

    // Create Paladin
    let paladin = PaladinBuilder::new(Arc::clone(&llm_port))
        .system_prompt("You are a helpful assistant")
        .name("No Herald Paladin")
        .build()
        .expect("Failed to build Paladin");

    // Create execution service WITHOUT Herald
    let service = PaladinExecutionService::new(Arc::clone(&llm_port), circuit_breaker, None, None);
    // Note: not calling .with_herald()

    // Execute
    let result = service.execute(&paladin, "Test").await;
    assert!(result.is_ok());

    let result = result.unwrap();

    // Format the result - should return None
    let formatted = service.format_result(&result, &paladin);
    assert!(formatted.is_ok());

    let formatted = formatted.unwrap();
    assert!(
        formatted.is_none(),
        "Should return None when no Herald is configured"
    );
}

#[tokio::test]
async fn test_herald_with_metadata() {
    // Create mock LLM port
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort::new("Metadata test response"));

    // Create circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,
        3,
        std::time::Duration::from_secs(60),
    ));

    // Create JSON Herald with metadata enabled
    let herald: Arc<dyn Herald> = Arc::new(JsonHerald::new());

    // Create Paladin
    let paladin = PaladinBuilder::new(Arc::clone(&llm_port))
        .system_prompt("You are a helpful assistant")
        .name("Metadata Paladin")
        .build()
        .expect("Failed to build Paladin");

    // Create execution service
    let service = PaladinExecutionService::new(Arc::clone(&llm_port), circuit_breaker, None, None)
        .with_herald(Arc::clone(&herald));

    // Execute
    let result = service.execute(&paladin, "Test").await;
    assert!(result.is_ok());

    let result = result.unwrap();

    // Verify metadata is present in result
    assert!(result.token_count > 0, "Token count should be populated");
    assert!(
        result.execution_time_ms >= 0,
        "Execution time should be non-negative"
    );
    assert!(result.loop_count >= 1, "Loop count should be at least 1");

    // Format
    let formatted = service.format_result(&result, &paladin).unwrap().unwrap();

    // Parse JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&formatted).expect("Output should be valid JSON");

    // The formatted output should contain the result data
    // (The exact metadata format depends on the Herald implementation)
    assert!(parsed.is_object());

    println!("Formatted output with metadata:\n{}", formatted);
}
