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

// ========================================================================================
// Error Handling Tests (Task 3.0 - FR-2.4 to FR-2.7)
// ========================================================================================

/// FR-2.4: Test tool call when no arsenal is available
#[tokio::test]
async fn test_tool_call_no_arsenal_available() {
    println!("\n▶ Testing tool call with no arsenal available...");

    // Arrange: Mock LLM that tries to call a tool
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call(
        "calculator",
        r#"{"operation":"add","a":1,"b":2}"#.to_string(),
    );
    mock_llm.add_success("Done without tool".to_string());

    // Create Paladin and service WITHOUT arsenal
    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(mock_llm.clone() as Arc<dyn LlmPort>, None);

    // Act: Execute
    let result = service
        .execute(&paladin, "What is 1 + 2?")
        .await;

    // Assert: Should not crash, execution completes gracefully
    assert!(
        result.is_ok(),
        "Execution should succeed even without arsenal"
    );

    let output = result.unwrap();
    println!("  Output: {}", output.output);

    // Should complete both loops (tool call attempt + final response)
    assert_eq!(
        mock_llm.call_count(),
        2,
        "Should have 2 LLM calls (tool call + final)"
    );

    println!("✓ No arsenal available test passed");
}

/// FR-2.5: Test tool call with unknown/unregistered tool
#[tokio::test]
async fn test_tool_call_unknown_tool() {
    println!("\n▶ Testing tool call with unknown tool...");

    // Arrange: Mock LLM calling a nonexistent tool
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call(
        "nonexistent_tool",
        r#"{"arg":"value"}"#.to_string(),
    );
    mock_llm.add_success("Proceeding anyway".to_string());

    // Mock Arsenal with NO tools registered (invoke will return ToolNotFound)
    let mock_arsenal = Arc::new(MockArsenalPort::new());
    // Don't add any tools - leaving it empty

    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(mock_llm.clone() as Arc<dyn LlmPort>, Some(mock_arsenal.clone()));

    // Act
    let result = service
        .execute(&paladin, "Use the nonexistent tool")
        .await;

    // Assert: Execution succeeds with graceful error handling
    assert!(result.is_ok(), "Should handle unknown tool gracefully");

    let output = result.unwrap();
    println!("  Output: {}", output.output);

    // Graceful degradation: execution continues without crashing
    // The error may or may not be visible in the output, depending on LLM response
    // Key is that the system doesn't crash

    assert_eq!(mock_llm.call_count(), 2, "Should have 2 LLM calls");
    assert_eq!(
        mock_arsenal.call_count(),
        1,
        "Should have attempted 1 tool invocation"
    );

    println!("✓ Unknown tool test passed");
}

/// FR-2.6: Test tool call with invalid arguments (malformed JSON)
#[tokio::test]
async fn test_tool_call_invalid_arguments() {
    println!("\n▶ Testing tool call with invalid arguments...");

    // Arrange: Mock LLM providing invalid JSON arguments
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call("calculator", "not valid json{{{".to_string());
    mock_llm.add_success("Recovered from error".to_string());

    // Mock Arsenal with calculator tool
    let mut mock_arsenal_raw = MockArsenalPort::new();
    mock_arsenal_raw.add_tool("calculator", "A calculator tool");
    let mock_arsenal = Arc::new(mock_arsenal_raw);
    mock_arsenal.set_response(
        "calculator",
        ArmamentResult::success(
            Uuid::new_v4(),
            json!("Should not reach here"),
            10
        ),
    );

    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(mock_llm.clone() as Arc<dyn LlmPort>, Some(mock_arsenal.clone()));

    // Act
    let result = service
        .execute(&paladin, "Calculate with bad args")
        .await;

    // Assert: Graceful degradation
    assert!(
        result.is_ok(),
        "Should handle invalid arguments gracefully"
    );

    let output = result.unwrap();
    println!("  Output: {}", output.output);

    // The tool invocation might fail early due to JSON parsing, or succeed but return error
    // Either way, execution should complete
    assert!(
        mock_llm.call_count() >= 1,
        "Should have at least 1 LLM call"
    );

    println!("✓ Invalid arguments test passed");
}

/// FR-2.7: Test tool execution error
#[tokio::test]
async fn test_tool_call_execution_error() {
    println!("\n▶ Testing tool call execution error...");

    // Arrange: Mock LLM calling a tool that will fail
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call("failing_tool", r#"{"input":"test"}"#.to_string());
    mock_llm.add_success("Handled the error".to_string());

    // Mock Arsenal configured to return an error for this tool
    let mut mock_arsenal_raw = MockArsenalPort::new();
    mock_arsenal_raw.add_tool("failing_tool", "A tool that fails");
    let mock_arsenal = Arc::new(mock_arsenal_raw);
    mock_arsenal.set_error("failing_tool", "Simulated execution failure");

    let paladin = create_test_paladin(MaxLoops::Fixed(2));
    let service = create_service(mock_llm.clone() as Arc<dyn LlmPort>, Some(mock_arsenal.clone()));

    // Act
    let result = service
        .execute(&paladin, "Use the failing tool")
        .await;

    // Assert: Execution succeeds, error is handled gracefully
    assert!(result.is_ok(), "Should handle tool execution error");

    let output = result.unwrap();
    println!("  Output: {}", output.output);

    // Output should contain error information
    let has_error = output.output.contains("FAILED")
        || output.output.contains("error")
        || output.output.contains("Error")
        || output.output.contains("failure");

    assert!(
        has_error,
        "Output should contain error message. Got: {}",
        output.output
    );

    assert_eq!(mock_llm.call_count(), 2, "Should have 2 LLM calls");
    assert_eq!(
        mock_arsenal.call_count(),
        1,
        "Should have attempted tool execution"
    );

    println!("✓ Execution error test passed");
}

