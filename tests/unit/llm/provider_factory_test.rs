// tests/unit/llm/provider_factory_test.rs
//
// Unit tests for LLM provider factory

use paladin::infrastructure::adapters::llm::provider_factory::{
    LlmProviderFactory, ProviderFactoryError,
};
use std::env;

#[test]
fn test_factory_provider_selection() {
    let factory = LlmProviderFactory::new();

    // Test unknown provider error
    let result = factory.create("unknown_provider");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ProviderFactoryError::UnknownProvider(_)
    ));

    // Test that provider names are recognized (even if config is missing)
    let result_openai = factory.create("openai");
    let result_deepseek = factory.create("deepseek");
    let result_anthropic = factory.create("anthropic");

    // All should fail with ConfigurationMissing (not UnknownProvider)
    assert!(matches!(
        result_openai.unwrap_err(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
    assert!(matches!(
        result_deepseek.unwrap_err(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
    assert!(matches!(
        result_anthropic.unwrap_err(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
}

#[test]
fn test_factory_config_validation() {
    // Test that factory properly validates configurations
    let factory = LlmProviderFactory::new();

    // Test with missing environment variables
    let result = factory.create("deepseek");
    assert!(result.is_err());

    let error = result.unwrap_err();
    match error {
        ProviderFactoryError::ConfigurationMissing(msg) => {
            assert!(msg.contains("DEEPSEEK_API_KEY"));
        }
        _ => panic!("Expected ConfigurationMissing error"),
    }
}

#[test]
fn test_factory_case_insensitive() {
    let factory = LlmProviderFactory::new();

    // All case variations should be recognized
    let result1 = factory.create("OpenAI");
    let result2 = factory.create("OPENAI");
    let result3 = factory.create("openai");

    // All should fail with ConfigurationMissing (not UnknownProvider)
    for result in [result1, result2, result3] {
        assert!(matches!(
            result.unwrap_err(),
            ProviderFactoryError::ConfigurationMissing(_)
        ));
    }
}

#[test]
fn test_factory_error_messages() {
    let factory = LlmProviderFactory::new();

    // Test unknown provider error message
    let result = factory.create("invalid_provider");
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unknown provider"));
    assert!(error_msg.contains("invalid_provider"));
    assert!(error_msg.contains("Supported providers"));
}

#[test]
fn test_get_default_provider() {
    // Save current environment state
    let openai_key = env::var("OPENAI_API_KEY").ok();
    let deepseek_key = env::var("DEEPSEEK_API_KEY").ok();
    let anthropic_key = env::var("ANTHROPIC_API_KEY").ok();

    // Clean environment
    env::remove_var("OPENAI_API_KEY");
    env::remove_var("DEEPSEEK_API_KEY");
    env::remove_var("ANTHROPIC_API_KEY");

    // No providers configured
    assert_eq!(LlmProviderFactory::get_default_provider(), None);

    // Only Anthropic configured - should be selected
    env::set_var("ANTHROPIC_API_KEY", "test-key");
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("anthropic".to_string())
    );

    // Add DeepSeek - should be selected over Anthropic
    env::set_var("DEEPSEEK_API_KEY", "test-key");
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("deepseek".to_string())
    );

    // Add OpenAI - should be selected as highest priority
    env::set_var("OPENAI_API_KEY", "test-key");
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("openai".to_string())
    );

    // Restore environment
    env::remove_var("OPENAI_API_KEY");
    env::remove_var("DEEPSEEK_API_KEY");
    env::remove_var("ANTHROPIC_API_KEY");

    if let Some(key) = openai_key {
        env::set_var("OPENAI_API_KEY", key);
    }
    if let Some(key) = deepseek_key {
        env::set_var("DEEPSEEK_API_KEY", key);
    }
    if let Some(key) = anthropic_key {
        env::set_var("ANTHROPIC_API_KEY", key);
    }
}

#[test]
fn test_list_available_providers() {
    // Save current environment state
    let openai_key = env::var("OPENAI_API_KEY").ok();
    let deepseek_key = env::var("DEEPSEEK_API_KEY").ok();
    let anthropic_key = env::var("ANTHROPIC_API_KEY").ok();

    // Clean environment
    env::remove_var("OPENAI_API_KEY");
    env::remove_var("DEEPSEEK_API_KEY");
    env::remove_var("ANTHROPIC_API_KEY");

    // No providers configured
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 0);

    // Add one provider
    env::set_var("DEEPSEEK_API_KEY", "test-key");
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 1);
    assert!(providers.contains(&"deepseek".to_string()));

    // Add all providers
    env::set_var("OPENAI_API_KEY", "test-key");
    env::set_var("ANTHROPIC_API_KEY", "test-key");
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 3);
    assert!(providers.contains(&"openai".to_string()));
    assert!(providers.contains(&"deepseek".to_string()));
    assert!(providers.contains(&"anthropic".to_string()));

    // Restore environment
    env::remove_var("OPENAI_API_KEY");
    env::remove_var("DEEPSEEK_API_KEY");
    env::remove_var("ANTHROPIC_API_KEY");

    if let Some(key) = openai_key {
        env::set_var("OPENAI_API_KEY", key);
    }
    if let Some(key) = deepseek_key {
        env::set_var("DEEPSEEK_API_KEY", key);
    }
    if let Some(key) = anthropic_key {
        env::set_var("ANTHROPIC_API_KEY", key);
    }
}

#[test]
fn test_factory_zero_sized() {
    let factory = LlmProviderFactory::new();
    // Factory should be zero-sized (no runtime cost)
    assert_eq!(std::mem::size_of_val(&factory), 0);
}

#[test]
fn test_factory_default() {
    let factory = LlmProviderFactory::default();
    assert_eq!(std::mem::size_of_val(&factory), 0);
}
