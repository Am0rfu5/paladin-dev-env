//! OpenAI Provider Integration Tests
//!
//! Tests for OpenAI adapter integration with real API calls (requires API key).
//! These tests are gated behind the integration-tests feature flag.

#[cfg(all(test, feature = "integration-tests"))]
mod openai_integration_tests {
    use paladin::application::ports::output::llm_port::{
        FinishReason, FunctionCall, LlmPort, LlmRequest, LlmResponse, TokenUsage,
    };
    use paladin::core::platform::container::content::ContentItem;
    use paladin::core::platform::container::prompt::PromptItem;
    use paladin::infrastructure::adapters::llm::openai_adapter::OpenAiAdapter;
    use std::collections::HashMap;
    use std::env;
    use uuid::Uuid;

    /// Helper to create OpenAI adapter from environment
    fn create_openai_adapter() -> OpenAiAdapter {
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set for OpenAI integration tests");
        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        OpenAiAdapter::new(api_key, base_url)
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_openai_simple_completion() {
        let adapter = create_openai_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Say 'Hello, Paladin!' and nothing else.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-3.5-turbo".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let result = adapter.generate(&request).await;
        assert!(result.is_ok(), "OpenAI API call failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(
            !response.content.is_empty(),
            "Response content should not be empty"
        );
        assert!(
            response.content.to_lowercase().contains("hello"),
            "Response should contain 'hello'"
        );
        assert_eq!(response.request_id, request.id);
        assert!(matches!(response.finish_reason, FinishReason::Stop));
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_openai_function_calling() {
        let adapter = create_openai_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What's the weather in San Francisco?".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-3.5-turbo".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert(
                    "functions".to_string(),
                    r#"[{"name": "get_weather", "description": "Get weather for a location", "parameters": {"type": "object", "properties": {"location": {"type": "string"}}}}]"#.to_string(),
                );
                meta
            },
        };

        let result = adapter.generate(&request).await;
        assert!(
            result.is_ok(),
            "OpenAI function call failed: {:?}",
            result.err()
        );

        let response = result.unwrap();
        // Function calling should return either a function call or content
        let has_function = response.function_call.is_some();
        let has_content = !response.content.is_empty();
        assert!(
            has_function || has_content,
            "Response should have either function call or content"
        );

        if let Some(func) = response.function_call {
            assert_eq!(func.name, "get_weather");
            assert!(!func.arguments.is_empty());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_openai_token_usage() {
        let adapter = create_openai_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Count to 5.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-3.5-turbo".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();

        // Verify token usage tracking
        assert!(
            response.usage.prompt_tokens > 0,
            "Prompt tokens should be tracked"
        );
        assert!(
            response.usage.completion_tokens > 0,
            "Completion tokens should be tracked"
        );
        assert_eq!(
            response.usage.total_tokens,
            response.usage.prompt_tokens + response.usage.completion_tokens,
            "Total tokens should equal sum of prompt and completion"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_openai_model_validation() {
        let adapter = create_openai_adapter();

        // Test with an invalid/non-existent model
        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Test".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-nonexistent-model".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let result = adapter.generate(&request).await;
        assert!(result.is_err(), "Should fail with non-existent model");
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_openai_with_system_message() {
        let adapter = create_openai_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What is your purpose?".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let mut metadata = HashMap::new();
        metadata.insert(
            "system_message".to_string(),
            "You are a helpful AI assistant specializing in Rust programming.".to_string(),
        );

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-3.5-turbo".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata,
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        // The response might reference being an assistant or helping with Rust
        // but we can't guarantee exact content, so just verify we got a response
    }
}
