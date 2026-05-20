/// RAG Retrieval Service
///
/// Handles retrieval of relevant memories from long-term storage (Sanctum)
/// for Retrieval-Augmented Generation (RAG).
///
/// This service:
/// - Generates embeddings for queries
/// - Searches Sanctum for similar memories
/// - Filters by similarity threshold
/// - Deduplicates near-identical memories
/// - Ranks by relevance
/// - Truncates to fit token budget
/// - Formats memories for prompt injection
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

use paladin_ports::output::embedding_port::EmbeddingPort;
use paladin_ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult,
};

/// Configuration for RAG retrieval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    /// Maximum number of memories to retrieve.
    pub top_k: usize,

    /// Minimum similarity score threshold (0.0 – 1.0).
    pub min_similarity: f32,

    /// Maximum tokens to include in context.
    pub max_tokens: usize,

    /// When to trigger memory retrieval.
    pub retrieval_trigger: RetrievalTrigger,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            min_similarity: 0.7,
            max_tokens: 2000,
            retrieval_trigger: RetrievalTrigger::Always,
        }
    }
}

/// When to trigger memory retrieval.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RetrievalTrigger {
    /// Always retrieve memories for every query.
    Always,

    /// Retrieve only when specific keywords are detected.
    KeywordBased,

    /// Retrieve when semantic similarity exceeds threshold.
    SemanticThreshold,
}

/// Service for retrieving relevant memories using RAG.
///
/// Depends only on port traits — contains no concrete adapter references.
pub struct RagRetrievalService {
    sanctum: Arc<dyn SanctumPort>,
    embedding: Arc<dyn EmbeddingPort>,
    config: RagConfig,
}

impl RagRetrievalService {
    /// Create a new RAG retrieval service.
    ///
    /// # Arguments
    ///
    /// * `sanctum` - Vector storage port for memory retrieval
    /// * `embedding` - Embedding generation port for query vectorization
    /// * `config` - RAG configuration parameters
    pub fn new(
        sanctum: Arc<dyn SanctumPort>,
        embedding: Arc<dyn EmbeddingPort>,
        config: RagConfig,
    ) -> Self {
        Self {
            sanctum,
            embedding,
            config,
        }
    }

    /// Retrieve relevant memories for a given query.
    ///
    /// # Arguments
    ///
    /// * `paladin_id` - ID of the Paladin requesting memories
    /// * `query` - The query text to find relevant memories for
    ///
    /// # Returns
    ///
    /// A vector of search results sorted by relevance (descending).
    ///
    /// # Errors
    ///
    /// Returns [`SanctumError`] if embedding generation or search fails.
    pub async fn retrieve_context(
        &self,
        paladin_id: &str,
        query: &str,
    ) -> Result<Vec<SanctumSearchResult>, SanctumError> {
        // Generate query embedding
        let embedding_result = self.embedding.embed_text(query).await.map_err(|e| {
            SanctumError::SearchError(format!("Embedding generation failed: {}", e))
        })?;

        // Build filter for this Paladin
        let filter = SanctumFilter::new().paladin_id(paladin_id.to_string());

        // Build search query
        let sanctum_query = SanctumQuery::new(embedding_result.vector, self.config.top_k)
            .with_filter(filter)
            .with_min_score(self.config.min_similarity);

        // Execute search
        let mut results = self.sanctum.search(sanctum_query).await?;

        log::debug!(
            "Retrieved {} memories for paladin {} with query: {}",
            results.len(),
            paladin_id,
            query
        );

        // Apply post-processing
        results = self.filter_by_similarity(results);
        results = self.deduplicate_memories(results);
        results = self.rank_by_relevance(results);
        results = self.truncate_to_token_budget(results);

        Ok(results)
    }

    /// Filter results by minimum similarity threshold.
    fn filter_by_similarity(&self, results: Vec<SanctumSearchResult>) -> Vec<SanctumSearchResult> {
        results
            .into_iter()
            .filter(|r| r.score >= self.config.min_similarity)
            .collect()
    }

    /// Deduplicate near-identical memories (>0.95 similarity).
    ///
    /// Removes memories that are very similar to each other, keeping only
    /// the highest-scoring instance.
    fn deduplicate_memories(
        &self,
        mut results: Vec<SanctumSearchResult>,
    ) -> Vec<SanctumSearchResult> {
        if results.len() <= 1 {
            return results;
        }

        // Sort by score descending to keep highest-scoring duplicates
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let original_count = results.len();
        let mut deduplicated = Vec::new();
        let mut seen_contents = HashSet::new();

        for result in results {
            // Use content as deduplication key
            let content_key = result.entry.memory.content.trim().to_lowercase();

            // Check if we've seen very similar content
            let is_duplicate = seen_contents.iter().any(|seen: &String| {
                // Simple similarity: check if content is substring or vice versa
                content_key.contains(seen) || seen.contains(&content_key)
            });

            if !is_duplicate {
                seen_contents.insert(content_key);
                deduplicated.push(result);
            }
        }

        log::debug!(
            "Deduplication: {} -> {} memories",
            original_count,
            deduplicated.len()
        );

        deduplicated
    }

    /// Rank memories by relevance score (descending).
    fn rank_by_relevance(&self, mut results: Vec<SanctumSearchResult>) -> Vec<SanctumSearchResult> {
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Truncate results to fit within token budget.
    ///
    /// Estimates tokens per memory and removes lowest-scoring memories
    /// to stay within the configured `max_tokens` limit.
    fn truncate_to_token_budget(
        &self,
        results: Vec<SanctumSearchResult>,
    ) -> Vec<SanctumSearchResult> {
        let mut total_tokens = 0;
        let mut truncated = Vec::new();

        for result in results {
            // Rough estimation: ~4 characters per token
            let estimated_tokens = result.entry.memory.content.len() / 4;

            if total_tokens + estimated_tokens <= self.config.max_tokens {
                total_tokens += estimated_tokens;
                truncated.push(result);
            } else {
                log::debug!(
                    "Truncating memories at token budget: {} tokens used of {} max",
                    total_tokens,
                    self.config.max_tokens
                );
                break;
            }
        }

        truncated
    }

    /// Format retrieved memories for prompt injection.
    ///
    /// Creates a structured text block suitable for including in the system
    /// prompt or user message.
    ///
    /// # Arguments
    ///
    /// * `memories` - The search results to format
    ///
    /// # Returns
    ///
    /// A formatted string containing the relevant context section.
    pub fn format_for_prompt(&self, memories: &[SanctumSearchResult]) -> String {
        if memories.is_empty() {
            return String::new();
        }

        let mut formatted = String::from("## Relevant Context\n\n");
        formatted.push_str("The following memories may be relevant to your current task:\n\n");

        for (idx, result) in memories.iter().enumerate() {
            let memory = &result.entry.memory;

            formatted.push_str(&format!(
                "**Memory {}** (Similarity: {:.2})\n",
                idx + 1,
                result.score
            ));
            formatted.push_str(&format!("Type: {:?}\n", memory.memory_type));
            formatted.push_str(&format!("Content: {}\n", memory.content));
            formatted.push_str(&format!(
                "Source: Conversation on {}\n\n",
                memory.created_at.format("%Y-%m-%d")
            ));
        }

        formatted.push_str("---\n\n");
        formatted
    }
}

/// Async wrapper for [`RagRetrievalService::retrieve_context`] with a timeout.
///
/// Returns an empty `Vec` on timeout to enable graceful degradation.
pub async fn retrieve_context_with_timeout(
    service: &RagRetrievalService,
    paladin_id: &str,
    query: &str,
    timeout_secs: u64,
) -> Result<Vec<SanctumSearchResult>, SanctumError> {
    match tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        service.retrieve_context(paladin_id, query),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            log::warn!(
                "Memory retrieval timed out after {} seconds, continuing with empty context",
                timeout_secs
            );
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use paladin_core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
    use paladin_ports::output::embedding_port::{Embedding, EmbeddingError, EmbeddingPort};
    use paladin_ports::output::sanctum_port::{
        SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult,
    };
    use std::sync::Arc;

    // ── Mock helpers ──────────────────────────────────────────────────────────

    struct MockEmbeddingPort;

    #[async_trait]
    impl EmbeddingPort for MockEmbeddingPort {
        async fn embed_text(&self, _text: &str) -> Result<Embedding, EmbeddingError> {
            Ok(Embedding {
                vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
                model: "mock-model".to_string(),
                dimension: 5,
                token_count: Some(10),
            })
        }

        async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingError> {
            Ok(texts
                .iter()
                .map(|_| Embedding {
                    vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
                    model: "mock-model".to_string(),
                    dimension: 5,
                    token_count: Some(10),
                })
                .collect())
        }

        fn dimension(&self) -> usize {
            5
        }

        fn model_name(&self) -> &str {
            "mock-model"
        }
    }

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

        async fn search(
            &self,
            _query: SanctumQuery,
        ) -> Result<Vec<SanctumSearchResult>, SanctumError> {
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

    fn create_test_entry(
        paladin_id: &str,
        content: &str,
        importance: f32,
        score: f32,
    ) -> SanctumSearchResult {
        let memory = MemoryBuilder::new(paladin_id.to_string(), content.to_string())
            .importance(importance)
            .memory_type(MemoryType::Semantic)
            .build()
            .unwrap();
        let entry = SanctumEntry::new(memory, vec![0.1, 0.2, 0.3, 0.4, 0.5]).unwrap();
        SanctumSearchResult::new(entry, score)
    }

    // ── Config / trigger tests ────────────────────────────────────────────────

    #[test]
    fn test_rag_config_builder() {
        let config = RagConfig {
            top_k: 10,
            min_similarity: 0.8,
            max_tokens: 3000,
            retrieval_trigger: RetrievalTrigger::KeywordBased,
        };

        assert_eq!(config.top_k, 10);
        assert_eq!(config.min_similarity, 0.8);
        assert_eq!(config.max_tokens, 3000);
    }

    #[test]
    fn test_retrieval_trigger_equality() {
        assert_eq!(RetrievalTrigger::Always, RetrievalTrigger::Always);
        assert_ne!(RetrievalTrigger::Always, RetrievalTrigger::KeywordBased);
    }

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

    // ── Async retrieval tests ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_successful_retrieval_with_multiple_memories() {
        let mock_results = vec![
            create_test_entry("paladin-1", "Memory 1", 0.9, 0.95),
            create_test_entry("paladin-1", "Memory 2", 0.8, 0.85),
            create_test_entry("paladin-1", "Memory 3", 0.7, 0.75),
        ];

        let sanctum = Arc::new(MockSanctumPort {
            results: mock_results,
        });
        let embedding = Arc::new(MockEmbeddingPort);
        let config = RagConfig::default();

        let service = RagRetrievalService::new(sanctum, embedding, config);
        let results = service
            .retrieve_context("paladin-1", "test query")
            .await
            .unwrap();

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_filtering_by_min_similarity() {
        let mock_results = vec![
            create_test_entry("paladin-1", "High score", 0.9, 0.95),
            create_test_entry("paladin-1", "Medium score", 0.8, 0.75),
            create_test_entry("paladin-1", "Low score", 0.7, 0.50),
        ];

        let sanctum = Arc::new(MockSanctumPort {
            results: mock_results,
        });
        let embedding = Arc::new(MockEmbeddingPort);
        let config = RagConfig {
            min_similarity: 0.7,
            ..Default::default()
        };

        let service = RagRetrievalService::new(sanctum, embedding, config);
        let results = service
            .retrieve_context("paladin-1", "test query")
            .await
            .unwrap();

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
        assert!(formatted.contains("0.95"));
        assert!(formatted.contains("0.85"));
    }

    #[tokio::test]
    async fn test_empty_results_graceful_handling() {
        let sanctum = Arc::new(MockSanctumPort { results: vec![] });
        let embedding = Arc::new(MockEmbeddingPort);
        let service = RagRetrievalService::new(sanctum, embedding, RagConfig::default());

        let results = service
            .retrieve_context("paladin-1", "test query")
            .await
            .unwrap();
        assert!(results.is_empty());
    }
}
