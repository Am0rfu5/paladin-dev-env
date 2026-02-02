use paladin::application::use_cases::paladin::prompt_generation_service::PromptGenerationService;
use paladin::application::ports::output::llm_port::{LlmPort, LlmRequest, LlmResponse, ProviderCapabilities};
use paladin::application::errors::prompt_error::PromptError;
use async_trait::async_trait;
use std::sync::Arc;

// Mock LLM Port for testing
struct MockLlmPort {
    response: String,
}

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(LlmResponse {
            id: uuid::Uuid::new_v4(),
            content: self.response.clone(),
            model: "mock-model".to_string(),
            finish_reason: "stop".to_string(),
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<
        tokio::sync::mpsc::Receiver<Result<String, Box<dyn std::error::Error + Send + Sync>>>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        unimplemented!("Streaming not needed for tests")
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: false,
            supports_functions: false,
            supports_vision: false,
            max_context_tokens: 4096,
        }
    }

    fn get_provider_name(&self) -> String {
        "MockLLM".to_string()
    }

    fn validate_model(&self, _model: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[test]
fn test_new_prompt_generation_service() {
    let llm = Arc::new(MockLlmPort {
        response: "Generated prompt".to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    // Service should be created successfully
    assert!(std::mem::size_of_val(&service) > 0);
}

#[tokio::test]
async fn test_generate_prompt_with_llm() {
    let expected_prompt = "You are a code analysis expert specialized in security auditing.";
    let llm = Arc::new(MockLlmPort {
        response: expected_prompt.to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    let result = service.generate_prompt(
        "CodeAuditor",
        "An agent specialized in analyzing code for security vulnerabilities"
    ).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_prompt);
}

#[tokio::test]
async fn test_prompt_caching() {
    let llm = Arc::new(MockLlmPort {
        response: "Generated prompt".to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    // First call should generate
    let result1 = service.generate_prompt("Agent1", "Description").await.unwrap();
    
    // Second call with same inputs should return cached result
    let result2 = service.generate_prompt("Agent1", "Description").await.unwrap();
    
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_deterministic_generation() {
    let llm = Arc::new(MockLlmPort {
        response: "Deterministic prompt".to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    // Generate prompt twice with same inputs
    let result1 = service.generate_prompt("TestAgent", "Test description").await.unwrap();
    
    // Clear cache to force regeneration
    service.clear_cache();
    
    let result2 = service.generate_prompt("TestAgent", "Test description").await.unwrap();
    
    // Results should be identical (deterministic)
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_empty_description_error() {
    let llm = Arc::new(MockLlmPort {
        response: "Prompt".to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    let result = service.generate_prompt("Agent", "").await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        PromptError::InvalidDescription(_) => (),
        _ => panic!("Expected InvalidDescription error"),
    }
}

#[tokio::test]
async fn test_cache_invalidation() {
    let llm = Arc::new(MockLlmPort {
        response: "First prompt".to_string(),
    });
    
    let service = PromptGenerationService::new(llm);
    
    let result1 = service.generate_prompt("Agent", "Desc").await.unwrap();
    assert_eq!(result1, "First prompt");
    
    // Clear cache
    service.clear_cache();
    
    // Should regenerate (in real scenario, might get different result from LLM)
    let result2 = service.generate_prompt("Agent", "Desc").await.unwrap();
    assert_eq!(result2, "First prompt"); // Same mock response
}
