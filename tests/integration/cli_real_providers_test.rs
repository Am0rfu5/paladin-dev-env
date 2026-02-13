//! CLI Integration Tests - Tier 3: API-Key-Gated Provider Tests
//!
//! These tests require real LLM API keys:
//! - OPENAI_API_KEY for OpenAI tests
//! - ANTHROPIC_API_KEY for Anthropic tests
//! - DEEPSEEK_API_KEY for DeepSeek tests
//!
//! Run with: `cargo test --test lib --features integration-tests integration::cli_real_providers_test -- --ignored`
//!
//! Tests are gated with both `#[cfg(feature = "integration-tests")]` and `#[ignore]`.

#[cfg(all(test, feature = "integration-tests"))]
mod provider_tests {
    use paladin::application::cli::config::loader::load_paladin_config;
    use paladin::application::ports::output::llm_port::LlmPort;
    use paladin::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    /// Helper to create temp config file
    fn write_config(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let path = dir.path().join(format!("{}.yaml", name));
        fs::write(&path, content).expect("Failed to write config");
        path
    }

    // =========================================================================
    // 5.3.1 & 5.3.2: OpenAI provider tests
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires OPENAI_API_KEY environment variable"]
    async fn test_openai_provider_connection() {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set for this test");

        assert!(!api_key.is_empty(), "OPENAI_API_KEY must not be empty");

        // Create provider via factory
        let provider = LlmProviderFactory::create("openai", &api_key, None);
        assert!(
            provider.is_ok(),
            "OpenAI provider should be created successfully"
        );

        // Validate connection with a simple request
        let provider = provider.unwrap();
        let result = provider
            .generate("gpt-4", "Say 'hello' and nothing else.", 0.1.into(), None)
            .await;

        assert!(
            result.is_ok(),
            "OpenAI should respond successfully. Error: {:?}",
            result.err()
        );

        let response = result.unwrap();
        assert!(
            !response.content.is_empty(),
            "OpenAI response should not be empty"
        );
        println!("OpenAI response: {}", response.content);
    }

    #[tokio::test]
    #[ignore = "Requires OPENAI_API_KEY environment variable"]
    async fn test_openai_agent_config_end_to_end() {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set for this test");

        let temp_dir = TempDir::new().unwrap();
        let config_path = write_config(
            &temp_dir,
            "openai_agent",
            r#"
name: "test-openai-agent"
system_prompt: "You are a concise test assistant. Respond in one word."
model: "gpt-4"
temperature: 0.1
max_loops: 1

provider:
  type: "openai"
"#,
        );

        // Load and validate config
        let config = load_paladin_config(&config_path).expect("Config should load successfully");

        assert_eq!(config.name, "test-openai-agent");
        assert_eq!(config.provider.provider_type, "openai");

        // Create provider and verify it can generate
        let provider = LlmProviderFactory::create("openai", &api_key, None)
            .expect("Provider should be created");

        let result = provider
            .generate(
                &config.model,
                &format!("{}\n\nUser: What is 2+2?", config.system_prompt),
                config.temperature.into(),
                None,
            )
            .await;

        assert!(result.is_ok(), "End-to-end agent execution should succeed");
    }

    // =========================================================================
    // 5.3.3 & 5.3.4: Anthropic provider tests
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires ANTHROPIC_API_KEY environment variable"]
    async fn test_anthropic_provider_connection() {
        let api_key =
            env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set for this test");

        assert!(!api_key.is_empty(), "ANTHROPIC_API_KEY must not be empty");

        let provider = LlmProviderFactory::create("anthropic", &api_key, None);
        assert!(
            provider.is_ok(),
            "Anthropic provider should be created successfully"
        );

        let provider = provider.unwrap();
        let result = provider
            .generate(
                "claude-sonnet-4-20250514",
                "Say 'hello' and nothing else.",
                0.1.into(),
                None,
            )
            .await;

        assert!(
            result.is_ok(),
            "Anthropic should respond successfully. Error: {:?}",
            result.err()
        );

        let response = result.unwrap();
        assert!(
            !response.content.is_empty(),
            "Anthropic response should not be empty"
        );
        println!("Anthropic response: {}", response.content);
    }

    // =========================================================================
    // 5.3.5 & 5.3.6: DeepSeek provider tests
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires DEEPSEEK_API_KEY environment variable"]
    async fn test_deepseek_provider_connection() {
        let api_key =
            env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set for this test");

        assert!(!api_key.is_empty(), "DEEPSEEK_API_KEY must not be empty");

        let provider = LlmProviderFactory::create("deepseek", &api_key, None);
        assert!(
            provider.is_ok(),
            "DeepSeek provider should be created successfully"
        );

        let provider = provider.unwrap();
        let result = provider
            .generate(
                "deepseek-chat",
                "Say 'hello' and nothing else.",
                0.1.into(),
                None,
            )
            .await;

        assert!(
            result.is_ok(),
            "DeepSeek should respond successfully. Error: {:?}",
            result.err()
        );

        let response = result.unwrap();
        assert!(
            !response.content.is_empty(),
            "DeepSeek response should not be empty"
        );
        println!("DeepSeek response: {}", response.content);
    }

    // =========================================================================
    // 5.3.8: Test streaming response handling
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires OPENAI_API_KEY environment variable"]
    async fn test_openai_streaming_response() {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set for this test");

        let provider = LlmProviderFactory::create("openai", &api_key, None)
            .expect("Provider should be created");

        let result = provider
            .generate_stream(
                "gpt-4",
                "Count from 1 to 5, separated by commas.",
                0.1.into(),
                None,
            )
            .await;

        assert!(
            result.is_ok(),
            "Streaming should start successfully. Error: {:?}",
            result.err()
        );

        let mut stream = result.unwrap();
        let mut collected = String::new();
        let mut chunk_count = 0;

        while let Some(chunk) = stream.recv().await {
            match chunk {
                Ok(text) => {
                    collected.push_str(&text);
                    chunk_count += 1;
                }
                Err(e) => {
                    panic!("Stream error: {:?}", e);
                }
            }
        }

        assert!(!collected.is_empty(), "Streaming should produce output");
        assert!(chunk_count > 0, "Should receive at least one chunk");
        println!("Streaming result ({} chunks): {}", chunk_count, collected);
    }
}
