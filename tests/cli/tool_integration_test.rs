//! Tool integration tests for Arsenal tool call flows
//!
//! Tests the end-to-end tool call loop: LLM requests tool → Arsenal invokes tool →
//! result is formatted and fed back to LLM for next iteration. Uses in-process
//! mocks (MockLlmAdapter + MockArsenalPort) for CI-friendly testing without
//! external dependencies.

use paladin::application::ports::output::arsenal_port::ArsenalPort;
use paladin::application::ports::output::llm_port::LlmPort;
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::arsenal::ArmamentResult;
use paladin::core::platform::container::paladin::{MaxLoops, PaladinData, PaladinStatus};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

// Test helper imports
use crate::helpers::{MockArsenalPort, MockLlmAdapter};

/// Create a test Paladin with minimal configuration for tool integration tests
fn create_test_paladin(max_loops: MaxLoops) -> Node<PaladinData> {
    let paladin_data = PaladinData {
        system_prompt: "You are a helpful AI assistant with access to tools.".to_string(),
        name: "ToolTestPaladin".to_string(),
        user_name: "TestUser".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_loops,
        stop_words: vec![],
        status: PaladinStatus::Idle,
        vision_enabled: false,
        ..Default::default()
    };
    Node::new(paladin_data, None)
}

/// Create PaladinExecutionService with CircuitBreaker wired up
fn create_service(
    llm: Arc<dyn LlmPort>,
    arsenal: Option<Arc<dyn ArsenalPort>>,
) -> PaladinExecutionService {
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    PaladinExecutionService::new(
        llm,
        circuit_breaker,
        None, // No garrison for these basic tests
        arsenal,
    )
}

#[tokio::test]
async fn test_tool_call_basic_flow() {
    // Arrange: Set up MockLlmAdapter with tool call + final response
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call("calculator", r#"{"operation":"add","a":1,"b":2}"#);
    mock_llm.add_success("The result is 3");

    // Arrange: Set up MockArsenalPort with calculator tool
    let mut mock_arsenal = MockArsenalPort::new();
    mock_arsenal.add_tool("calculator", "A basic calculator");
    let mock_arsenal = Arc::new(mock_arsenal);

    let call_id = Uuid::new_v4();
    mock_arsenal.set_response(
        "calculator",
        ArmamentResult::success(call_id, json!("3"), 100),
    );

    // Arrange: Create Paladin and execution service
    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(
        mock_llm.clone() as Arc<dyn LlmPort>,
        Some(mock_arsenal.clone()),
    );

    // Act: Execute Paladin with tool-requiring query
    let result = service.execute(&paladin, "What is 1 + 2?").await;

    // Assert: Execution succeeds
    assert!(
        result.is_ok(),
        "Execution should succeed: {:?}",
        result.err()
    );

    let output = result.unwrap();

    // Assert: Output contains tool result
    assert!(
        output.output.contains("3") || output.output.contains("calculator"),
        "Output should contain tool result or tool name. Got: {}",
        output.output
    );

    // Assert: LLM called twice (tool call + final)
    assert_eq!(
        mock_llm.call_count(),
        2,
        "LLM should be called twice: tool call then final response"
    );

    // Assert: Arsenal invoked once
    assert_eq!(
        mock_arsenal.call_count(),
        1,
        "Arsenal should be invoked once for calculator"
    );

    println!("✓ Basic tool call flow test passed");
    println!("  Output: {}", output.output);
}

#[tokio::test]
async fn test_tool_call_result_fed_back_to_llm() {
    // Arrange: Same setup as basic flow
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call("calculator", r#"{"operation":"add","a":5,"b":7}"#);
    mock_llm.add_success("The calculation shows the answer is 12");

    let mut mock_arsenal = MockArsenalPort::new();
    mock_arsenal.add_tool("calculator", "A calculator");
    let mock_arsenal = Arc::new(mock_arsenal);

    let call_id = Uuid::new_v4();
    mock_arsenal.set_response(
        "calculator",
        ArmamentResult::success(call_id, json!("12"), 50),
    );

    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(
        mock_llm.clone() as Arc<dyn LlmPort>,
        Some(mock_arsenal.clone()),
    );

    // Act: Execute
    let result = service.execute(&paladin, "What is 5 + 7?").await;

    // Assert: Execution succeeds
    assert!(result.is_ok(), "Execution should succeed");

    // Assert: Inspect invocations to verify tool result was fed back
    let invocations = mock_llm.invocations();
    assert_eq!(
        invocations.len(),
        2,
        "Should have exactly 2 LLM invocations"
    );

    // The second invocation's prompt should contain the formatted tool result
    let second_prompt = &invocations[1].prompt;

    // Check for formatted tool result markers (from ToolResultFormatter)
    let has_tool_execution = second_prompt.contains("Tool Execution")
        || second_prompt.contains("calculator")
        || second_prompt.contains("🔧");
    let has_result_marker = second_prompt.contains("Result:") || second_prompt.contains("SUCCESS");
    let has_output = second_prompt.contains("12") || second_prompt.contains("Output:");

    assert!(
        has_tool_execution,
        "Second LLM call should contain tool execution marker. Prompt: {}",
        second_prompt
    );
    assert!(
        has_result_marker || has_output,
        "Second LLM call should contain result marker or output. Prompt: {}",
        second_prompt
    );

    println!("✓ Tool result feedback test passed");
    println!(
        "  Second invocation prompt length: {} chars",
        second_prompt.len()
    );
    println!(
        "  Contains 'calculator': {}",
        second_prompt.contains("calculator")
    );
    println!("  Contains '12': {}", second_prompt.contains("12"));
}
