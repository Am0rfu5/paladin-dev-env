// tests/unit/llm/deepseek_adapter_test.rs
//
// Unit tests for DeepSeek adapter with mocked HTTP responses

use mockito::{Mock, Server, ServerGuard};
use paladin_ports::output::llm_port::{LlmPort, LlmRequest};
use paladin::core::platform::container::prompt::{
    PromptData, PromptItem, PromptParameters, PromptRole, PromptType, SystemPrompt, TextPrompt,
    UserPrompt,
};
use paladin::infrastructure::adapters::llm::deepseek_adapter::{DeepSeekAdapter, DeepSeekConfig};
use uuid::Uuid;

/// Helper to create a mock server and adapter configured to use it
fn setup_mock_server() -> (ServerGuard, DeepSeekAdapter) {
    let server = Server::new();
    let config = DeepSeekConfig {
        api_key: "test-api-key".to_string(),
        base_url: server.url(),
        timeout_seconds: 30,
        max_retries: 0, // Disable retries for tests
    };
    let adapter = DeepSeekAdapter::new(config).unwrap();
    (server, adapter)
}

/// Helper to create a basic LLM request
fn create_test_request(content: &str) -> LlmRequest {
    let system_prompt = PromptItem::new(PromptData {
        id: Uuid::new_v4(),
        prompt_type: PromptType::System(SystemPrompt {
            instructions: content.to_string(),
            constraints: None,
            examples: None,
        }),
        parameters: PromptParameters {
            max_tokens: Some(100),
            temperature: Some(0.7),
            top_p: Some(1.0),
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: None,
        },
    });

    LlmRequest {
        id: Uuid::new_v4(),
        model: "deepseek-chat".to_string(),
        prompt: system_prompt,
        attachments: vec![],
    }
}

#[tokio::test]
async fn test_deepseek_successful_completion() {
    let (mut server, adapter) = setup_mock_server();

    let mock_response = r#"{
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "deepseek-chat",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! How can I help you today?"
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30
        }
    }"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_ok());
    let response = response.unwrap();
    assert_eq!(response.content, "Hello! How can I help you today?");
    assert_eq!(response.usage.prompt_tokens, 10);
    assert_eq!(response.usage.completion_tokens, 20);
    assert_eq!(response.usage.total_tokens, 30);
}

#[tokio::test]
async fn test_deepseek_streaming_response() {
    let (mut server, adapter) = setup_mock_server();

    // Mock SSE streaming response
    let mock_stream = "data: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"deepseek-chat\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\ndata: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"deepseek-chat\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" world\"},\"finish_reason\":null}]}\n\ndata: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"deepseek-chat\",\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}]}\n\ndata: [DONE]\n\n";

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(mock_stream)
        .create();

    let request = create_test_request("Hello");
    let stream_result = adapter.generate_stream(request).await;

    assert!(stream_result.is_ok());
    // Streaming validation would require consuming the stream
    // For now, we just verify it doesn't error
}

#[tokio::test]
async fn test_deepseek_auth_failure_401() {
    let (mut server, adapter) = setup_mock_server();

    let error_response = r#"{
        "error": {
            "message": "Invalid authentication",
            "type": "invalid_request_error",
            "code": "invalid_api_key"
        }
    }"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(error_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::AuthenticationError(_)
    ));
}

#[tokio::test]
async fn test_deepseek_rate_limit_429() {
    let (mut server, adapter) = setup_mock_server();

    let error_response = r#"{
        "error": {
            "message": "Rate limit exceeded",
            "type": "rate_limit_error"
        }
    }"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(error_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::RateLimitExceeded
    ));
}

#[tokio::test]
async fn test_deepseek_timeout() {
    // Create adapter with very short timeout
    let server = Server::new();
    let config = DeepSeekConfig {
        api_key: "test-api-key".to_string(),
        base_url: server.url(),
        timeout_seconds: 1,
        max_retries: 0,
    };
    let adapter = DeepSeekAdapter::new(config).unwrap();

    // Don't create a mock - server will not respond, causing timeout
    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    // Timeout manifests as NetworkError
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::NetworkError(_)
    ));
}

#[tokio::test]
async fn test_deepseek_invalid_model_error() {
    let (mut server, adapter) = setup_mock_server();

    let error_response = r#"{
        "error": {
            "message": "The model 'invalid-model' does not exist",
            "type": "invalid_request_error",
            "code": "model_not_found"
        }
    }"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(error_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::InvalidPrompt(_)
    ));
}

#[tokio::test]
async fn test_deepseek_server_error_500() {
    let (mut server, adapter) = setup_mock_server();

    let error_response = r#"{
        "error": {
            "message": "Internal server error",
            "type": "server_error"
        }
    }"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(error_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::ProcessingError(_)
    ));
}

#[tokio::test]
async fn test_deepseek_malformed_response() {
    let (mut server, adapter) = setup_mock_server();

    let malformed_response = r#"{"invalid": "json", "missing": "required_fields"}"#;

    let _mock = server
        .mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(malformed_response)
        .create();

    let request = create_test_request("Hello");
    let response = adapter.generate(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(matches!(
        error,
        paladin_ports::output::llm_port::LlmError::ProcessingError(_)
    ));
}
