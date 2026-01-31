//! DeepSeek Provider Integration Tests
//!
//! Tests for DeepSeek adapter integration with real API calls (requires API key).
//! These tests are gated behind the integration-tests feature flag.

#[cfg(all(test, feature = "integration-tests"))]
mod deepseek_integration_tests {
    use paladin::application::ports::output::llm_port::{FinishReason, LlmPort, LlmRequest};
    use paladin::core::platform::container::prompt::PromptItem;
    use paladin::infrastructure::adapters::llm::deepseek_adapter::DeepSeekAdapter;
    use std::collections::HashMap;
    use std::env;
    use uuid::Uuid;

    /// Helper to create DeepSeek adapter from environment
    fn create_deepseek_adapter() -> DeepSeekAdapter {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .expect("DEEPSEEK_API_KEY must be set for DeepSeek integration tests");
        let base_url = env::var("DEEPSEEK_BASE_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com/v1".to_string());

        DeepSeekAdapter::new(api_key, base_url)
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_deepseek_simple_completion() {
        let adapter = create_deepseek_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Say 'Hello from DeepSeek!' and nothing else.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "deepseek-chat".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let result = adapter.generate(&request).await;
        assert!(
            result.is_ok(),
            "DeepSeek API call failed: {:?}",
            result.err()
        );

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
    async fn test_deepseek_reasoning_task() {
        let adapter = create_deepseek_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Explain why Rust's ownership system prevents data races.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "deepseek-chat".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        // Check for keywords related to Rust's ownership
        let content_lower = response.content.to_lowercase();
        assert!(
            content_lower.contains("ownership") || content_lower.contains("borrow"),
            "Response should discuss ownership or borrowing"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_deepseek_token_usage() {
        let adapter = create_deepseek_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "List three programming languages.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "deepseek-chat".to_string(),
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
            "Total tokens should equal sum"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_deepseek_with_temperature() {
        let adapter = create_deepseek_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Generate a creative name for a software project.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let mut metadata = HashMap::new();
        metadata.insert("temperature".to_string(), "0.9".to_string());
        metadata.insert("max_tokens".to_string(), "50".to_string());

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "deepseek-chat".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata,
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        // With high temperature, responses should be creative/varied
        // Just verify we got a valid response
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_deepseek_cost_efficiency() {
        // DeepSeek is known for cost efficiency - test basic functionality
        let adapter = create_deepseek_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "deepseek-chat".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(response.content.contains("4"));

        // Verify reasonable token usage for simple query
        assert!(
            response.usage.total_tokens < 100,
            "Simple query should not use excessive tokens"
        );
    }
}
