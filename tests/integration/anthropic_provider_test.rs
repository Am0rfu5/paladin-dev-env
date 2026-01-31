//! Anthropic Provider Integration Tests
//!
//! Tests for Anthropic (Claude) adapter integration with real API calls (requires API key).
//! These tests are gated behind the integration-tests feature flag.

#[cfg(all(test, feature = "integration-tests"))]
mod anthropic_integration_tests {
    use paladin::application::ports::output::llm_port::{FinishReason, LlmPort, LlmRequest};
    use paladin::core::platform::container::prompt::PromptItem;
    use paladin::infrastructure::adapters::llm::anthropic_adapter::AnthropicAdapter;
    use std::collections::HashMap;
    use std::env;
    use uuid::Uuid;

    /// Helper to create Anthropic adapter from environment
    fn create_anthropic_adapter() -> AnthropicAdapter {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set for Anthropic integration tests");
        let base_url = env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string());

        AnthropicAdapter::new(api_key, base_url)
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_anthropic_simple_completion() {
        let adapter = create_anthropic_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Say 'Hello from Claude!' and nothing else.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-sonnet-20240229".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let result = adapter.generate(&request).await;
        assert!(
            result.is_ok(),
            "Anthropic API call failed: {:?}",
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
    async fn test_anthropic_long_context() {
        let adapter = create_anthropic_adapter();

        // Claude models support long context windows
        let long_content = "Context: ".to_string() + &"word ".repeat(1000);
        let question = format!(
            "{}\n\nHow many times does 'word' appear in the context?",
            long_content
        );

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: question,
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-sonnet-20240229".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        // Claude should be able to handle and count the words
        assert!(
            response.content.contains("1000") || response.content.contains("thousand"),
            "Should correctly count or approximate word count"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_anthropic_reasoning_quality() {
        let adapter = create_anthropic_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content:
                "Explain the benefits of hexagonal architecture in software design. Be concise."
                    .to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-sonnet-20240229".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());

        let content_lower = response.content.to_lowercase();
        // Check for architecture-related concepts
        let has_relevant_content = content_lower.contains("architecture")
            || content_lower.contains("decoupl")
            || content_lower.contains("port")
            || content_lower.contains("adapter")
            || content_lower.contains("testab");

        assert!(
            has_relevant_content,
            "Response should discuss architecture concepts"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_anthropic_token_usage() {
        let adapter = create_anthropic_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Write the word 'test' three times.".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-sonnet-20240229".to_string(),
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
    async fn test_anthropic_with_system_message() {
        let adapter = create_anthropic_adapter();

        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What should I know about your capabilities?".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let mut metadata = HashMap::new();
        metadata.insert(
            "system_message".to_string(),
            "You are Claude, an AI assistant created by Anthropic. You should be helpful, harmless, and honest.".to_string(),
        );

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-sonnet-20240229".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata,
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        // Response should acknowledge being Claude/AI assistant
        let content_lower = response.content.to_lowercase();
        assert!(
            content_lower.contains("claude") || content_lower.contains("assistant"),
            "Should reference being Claude or an assistant"
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key and makes real API calls
    async fn test_anthropic_model_variants() {
        let adapter = create_anthropic_adapter();

        // Test with Claude Haiku (faster, cheaper model)
        let prompt = PromptItem {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What is Rust?".to_string(),
            template_name: None,
            template_vars: HashMap::new(),
        };

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "claude-3-haiku-20240307".to_string(),
            prompt,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = adapter.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        assert!(
            response.content.to_lowercase().contains("rust")
                || response.content.to_lowercase().contains("programming"),
            "Should explain Rust programming language"
        );
    }
}
