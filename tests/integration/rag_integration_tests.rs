//! Integration tests for RAG (Retrieval-Augmented Generation) Configuration
//!
//! Tests RAG and Memory Extraction configuration validation.
//! Full end-to-end integration tests with Qdrant will be added in a follow-up task.

use paladin::config::application_settings::{
    MemoryExtractionConfig, MemoryExtractionStrategy, RagConfig,
};

#[tokio::test]
async fn test_rag_config_validation() {
    // Test: RagConfig validates constraints
    let config = RagConfig {
        top_k: 5,
        min_similarity: 0.7,
        max_tokens: 2000,
        timeout_seconds: 5,
    };

    assert!(config.validate().is_ok());

    // Invalid configurations
    let invalid_top_k = RagConfig {
        top_k: 0,
        ..RagConfig::default()
    };
    assert!(invalid_top_k.validate().is_err());

    let invalid_similarity = RagConfig {
        min_similarity: 1.5,
        ..RagConfig::default()
    };
    assert!(invalid_similarity.validate().is_err());

    let invalid_tokens = RagConfig {
        max_tokens: 0,
        ..RagConfig::default()
    };
    assert!(invalid_tokens.validate().is_err());
}

#[tokio::test]
async fn test_memory_extraction_config_validation() {
    // Test: MemoryExtractionConfig validates correctly
    let config = MemoryExtractionConfig {
        enabled: true,
        strategy: MemoryExtractionStrategy::OnCompletion,
    };

    assert!(config.validate().is_ok());

    // Threshold strategy with valid importance
    let threshold_config = MemoryExtractionConfig {
        enabled: true,
        strategy: MemoryExtractionStrategy::Threshold { importance: 5 },
    };
    assert!(threshold_config.validate().is_ok());

    // Invalid threshold importance (0)
    let invalid_config = MemoryExtractionConfig {
        enabled: true,
        strategy: MemoryExtractionStrategy::Threshold { importance: 0 },
    };
    assert!(invalid_config.validate().is_err());
}

#[tokio::test]
async fn test_rag_config_defaults() {
    // Test: RagConfig has sensible defaults
    let config = RagConfig::default();

    assert_eq!(config.top_k, 5);
    assert_eq!(config.min_similarity, 0.7);
    assert_eq!(config.max_tokens, 2000);
    assert_eq!(config.timeout_seconds, 5);

    // Defaults should be valid
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_memory_extraction_config_defaults() {
    // Test: MemoryExtractionConfig has sensible defaults
    let config = MemoryExtractionConfig::default();

    assert!(config.enabled);
    assert_eq!(config.strategy, MemoryExtractionStrategy::OnCompletion);

    // Defaults should be valid
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_memory_extraction_strategy_variants() {
    // Test: All MemoryExtractionStrategy variants can be constructed
    let strategies = vec![
        MemoryExtractionStrategy::EveryTurn,
        MemoryExtractionStrategy::OnCompletion,
        MemoryExtractionStrategy::Manual,
        MemoryExtractionStrategy::Threshold { importance: 5 },
    ];

    for strategy in strategies {
        let config = MemoryExtractionConfig {
            enabled: true,
            strategy,
        };
        assert!(config.validate().is_ok());
    }
}

#[tokio::test]
async fn test_rag_config_boundary_values() {
    // Test: RAG config boundary validation

    // Minimum valid values
    let min_config = RagConfig {
        top_k: 1,
        min_similarity: 0.0,
        max_tokens: 1,
        timeout_seconds: 1,
    };
    assert!(min_config.validate().is_ok());

    // Maximum valid top_k
    let max_top_k = RagConfig {
        top_k: 100,
        ..RagConfig::default()
    };
    assert!(max_top_k.validate().is_ok());

    // Just over limit should fail
    let over_limit = RagConfig {
        top_k: 101,
        ..RagConfig::default()
    };
    assert!(over_limit.validate().is_err());

    // Maximum valid similarity
    let max_similarity = RagConfig {
        min_similarity: 1.0,
        ..RagConfig::default()
    };
    assert!(max_similarity.validate().is_ok());
}

// TODO: Add full integration tests with Qdrant in follow-up task
// These will test:
// - RagRetrievalService.retrieve_context() with real Qdrant
// - MemoryExtractionService.extract_memories() with real storage
// - End-to-end Paladin execution with RAG enabled
// - Token budget limiting
// - Context formatting
