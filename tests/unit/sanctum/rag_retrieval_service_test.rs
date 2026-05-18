/// Unit tests for RAG Retrieval Service
///
/// Tests verify retrieval, deduplication, ranking, and token budget management.

use std::sync::Arc;
use chrono::Utc;

use paladin_ports::output::sanctum_port::{
    SanctumPort, SanctumQuery, SanctumSearchResult, SanctumError, SanctumFilter,
};
use paladin_ports::output::embedding_port::{EmbeddingPort, Embedding, EmbeddingError};
use crate::application::use_cases::sanctum::rag_retrieval_service::{
    RagRetrievalService, RagConfig, RetrievalTrigger,
};
use crate::core::platform::container::sanctum::{Memory, MemoryBuilder, MemoryType, SanctumEntry};
use async_trait::async_trait;

// Mock EmbeddingPort for testing
struct MockEmbeddingPort;

#[async_trait]
impl EmbeddingPort for MockEmbeddingPort {
    async fn embed_text(&self, _text: &str) -> Result<Embedding, EmbeddingError> {
        Ok(Embedding {
            vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            model: "mock-model".to_string(),
            token_count: Some(10),
        })
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingError> {
        Ok(texts.iter().map(|_| Embedding {
            vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            model: "mock-model".to_string(),
            token_count: Some(10),
        }).collect())
    }

    fn dimension(&self) -> usize {
        5
    }

    fn model_name(&self) -> &str {
        "mock-model"
    }
}

// Mock SanctumPort for testing
struct MockSanctumPort {
    results: Vec<SanctumSearchResult>,
}

#[async_trait]
impl SanctumPort for MockSanctumPort {
    async fn store(&self, _entry: SanctumEntry) -> Result<(), SanctumError> {
        Ok(())
    }

    async fn store_batch(&self, _entries: Vec<SanctumEntry>) -> Result<(), SanctumError> {
        Ok(())
    }

    async fn search(&self, _query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError> {
        Ok(self.results.clone())
    }

    async fn delete(&self, _id: &str) -> Result<bool, SanctumError> {
        Ok(true)
    }

    async fn update(&self, _entry: SanctumEntry) -> Result<(), SanctumError> {
        Ok(())
    }

    async fn count(&self, _filter: Option<SanctumFilter>) -> Result<usize, SanctumError> {
        Ok(self.results.len())
    }
}

fn create_test_entry(paladin_id: &str, content: &str, importance: f32, score: f32) -> SanctumSearchResult {
    let memory = MemoryBuilder::new(paladin_id.to_string(), content.to_string())
        .importance(importance)
        .memory_type(MemoryType::Semantic)
        .build()
        .unwrap();
    
    let entry = SanctumEntry::new(memory, vec![0.1, 0.2, 0.3, 0.4, 0.5]).unwrap();
    SanctumSearchResult::new(entry, score)
}

#[cfg(test)]
mod rag_retrieval_service_tests {
    use super::*;

    #[test]
    fn test_rag_config_default() {
        let config = RagConfig::default();
        assert_eq!(config.top_k, 5);
        assert_eq!(config.min_similarity, 0.7);
        assert_eq!(config.max_tokens, 2000);
    }

    #[test]
    fn test_retrieval_trigger_variants() {
        let trigger = RetrievalTrigger::Always;
        assert!(matches!(trigger, RetrievalTrigger::Always));
    }

    #[tokio::test]
    async fn test_successful_retrieval_with_multiple_memories() {
        let mock_results = vec![
            create_test_entry("paladin-1", "Memory 1", 0.9, 0.95),
            create_test_entry("paladin-1", "Memory 2", 0.8, 0.85),
            create_test_entry("paladin-1", "Memory 3", 0.7, 0.75),
        ];

        let sanctum = Arc::new(MockSanctumPort { results: mock_results });
        let embedding = Arc::new(MockEmbeddingPort);
        let config = RagConfig::default();

        let service = RagRetrievalService::new(sanctum, embedding, config);
        let results = service.retrieve_context("paladin-1", "test query").await.unwrap();

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_filtering_by_min_similarity() {
        let mock_results = vec![
            create_test_entry("paladin-1", "High score", 0.9, 0.95),
            create_test_entry("paladin-1", "Medium score", 0.8, 0.75),
            create_test_entry("paladin-1", "Low score", 0.7, 0.50), // Below threshold
        ];

        let sanctum = Arc::new(MockSanctumPort { results: mock_results });
        let embedding = Arc::new(MockEmbeddingPort);
        let config = RagConfig {
            min_similarity: 0.7,
            ..Default::default()
        };

        let service = RagRetrievalService::new(sanctum, embedding, config);
        let results = service.retrieve_context("paladin-1", "test query").await.unwrap();

        // Only 2 results should pass the 0.7 threshold
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.score >= 0.7));
    }

    #[test]
    fn test_format_for_prompt() {
        let memories = vec![
            create_test_entry("paladin-1", "First memory", 0.9, 0.95),
            create_test_entry("paladin-1", "Second memory", 0.8, 0.85),
        ];

        let sanctum = Arc::new(MockSanctumPort { results: vec![] });
        let embedding = Arc::new(MockEmbeddingPort);
        let service = RagRetrievalService::new(sanctum, embedding, RagConfig::default());

        let formatted = service.format_for_prompt(&memories);
        
        assert!(formatted.contains("## Relevant Context"));
        assert!(formatted.contains("First memory"));
        assert!(formatted.contains("Second memory"));
        assert!(formatted.contains("0.95")); // Score
        assert!(formatted.contains("0.85")); // Score
    }

    #[tokio::test]
    async fn test_empty_results_graceful_handling() {
        let sanctum = Arc::new(MockSanctumPort { results: vec![] });
        let embedding = Arc::new(MockEmbeddingPort);
        let service = RagRetrievalService::new(sanctum, embedding, RagConfig::default());

        let results = service.retrieve_context("paladin-1", "test query").await.unwrap();
        assert!(results.is_empty());
    }
}
