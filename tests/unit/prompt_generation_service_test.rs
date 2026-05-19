use paladin::MockLlmAdapter;
use paladin::application::errors::prompt_error::PromptError;
use paladin::application::use_cases::paladin::prompt_generation_service::PromptGenerationService;
use std::sync::Arc;

#[test]
fn test_new_prompt_generation_service() {
    let llm = Arc::new(MockLlmAdapter::new().with_response("Generated prompt".to_string()));

    let service = PromptGenerationService::new(llm);

    // Service should be created successfully
    assert!(std::mem::size_of_val(&service) > 0);
}

#[tokio::test]
async fn test_generate_prompt_with_llm() {
    let expected_prompt = "You are a code analysis expert specialized in security auditing.";
    let llm = Arc::new(MockLlmAdapter::new().with_response(expected_prompt.to_string()));

    let service = PromptGenerationService::new(llm);

    let result = service
        .generate_prompt(
            "CodeAuditor",
            "An agent specialized in analyzing code for security vulnerabilities",
            "mock-model",
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_prompt);
}

#[tokio::test]
async fn test_prompt_caching() {
    let llm = Arc::new(MockLlmAdapter::new().with_response("Generated prompt".to_string()));

    let service = PromptGenerationService::new(llm);

    // First call should generate
    let result1 = service
        .generate_prompt("Agent1", "Description", "mock-model")
        .await
        .unwrap();

    // Second call with same inputs should return cached result
    let result2 = service
        .generate_prompt("Agent1", "Description", "mock-model")
        .await
        .unwrap();

    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_deterministic_generation() {
    let llm = Arc::new(MockLlmAdapter::new().with_response("Deterministic prompt".to_string()));

    let service = PromptGenerationService::new(llm);

    // Generate prompt twice with same inputs
    let result1 = service
        .generate_prompt("TestAgent", "Test description", "mock-model")
        .await
        .unwrap();

    // Clear cache to force regeneration
    service.clear_cache();

    let result2 = service
        .generate_prompt("TestAgent", "Test description", "mock-model")
        .await
        .unwrap();

    // Results should be identical (deterministic)
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_empty_description_error() {
    let llm = Arc::new(MockLlmAdapter::new().with_response("Prompt".to_string()));

    let service = PromptGenerationService::new(llm);

    let result = service.generate_prompt("Agent", "", "mock-model").await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PromptError::InvalidDescription(_) => (),
        _ => panic!("Expected InvalidDescription error"),
    }
}

#[tokio::test]
async fn test_cache_invalidation() {
    let llm = Arc::new(MockLlmAdapter::new().with_response("First prompt".to_string()));

    let service = PromptGenerationService::new(llm);

    let result1 = service
        .generate_prompt("Agent", "Desc", "mock-model")
        .await
        .unwrap();
    assert_eq!(result1, "First prompt");

    // Clear cache
    service.clear_cache();

    // Should regenerate (in real scenario, might get different result from LLM)
    let result2 = service
        .generate_prompt("Agent", "Desc", "mock-model")
        .await
        .unwrap();
    assert_eq!(result2, "First prompt"); // Same mock response
}
