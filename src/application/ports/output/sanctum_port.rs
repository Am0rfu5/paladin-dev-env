/// Sanctum Port - Vector Storage Operations
///
/// Port trait for vector storage and semantic search operations.
/// This provides a standardized interface for different vector database
/// implementations (Qdrant, in-memory, etc.).
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::core::platform::container::sanctum::{MemoryType, SanctumEntry};

/// Errors that can occur during Sanctum operations
#[derive(Debug, thiserror::Error)]
pub enum SanctumError {
    /// Storage operation failed
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Search operation failed
    #[error("Search error: {0}")]
    SearchError(String),

    /// Invalid embedding dimension
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),

    /// Entry not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Filter criteria for searching memories
///
/// Allows filtering by metadata fields before semantic similarity search.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SanctumFilter {
    /// Filter by Paladin ID
    pub paladin_id: Option<String>,

    /// Filter by memory type
    pub memory_type: Option<MemoryType>,

    /// Filter by creation date range
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,

    /// Filter by importance threshold
    pub min_importance: Option<f32>,

    /// Additional custom metadata filters
    pub metadata_filters: HashMap<String, Value>,
}

impl SanctumFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by Paladin ID
    pub fn paladin_id(mut self, paladin_id: String) -> Self {
        self.paladin_id = Some(paladin_id);
        self
    }

    /// Filter by memory type
    pub fn memory_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_type = Some(memory_type);
        self
    }

    /// Filter by minimum importance
    pub fn min_importance(mut self, min_importance: f32) -> Self {
        self.min_importance = Some(min_importance);
        self
    }

    /// Add a custom metadata filter
    pub fn add_metadata_filter(mut self, key: String, value: Value) -> Self {
        self.metadata_filters.insert(key, value);
        self
    }
}

/// Query for searching the vector store
///
/// Contains the search vector, result limits, and filtering criteria.
#[derive(Debug, Clone)]
pub struct SanctumQuery {
    /// The query embedding vector
    pub embedding: Vec<f32>,

    /// Maximum number of results to return
    pub top_k: usize,

    /// Optional filter criteria
    pub filter: Option<SanctumFilter>,

    /// Minimum similarity score (0.0 - 1.0)
    pub min_score: Option<f32>,
}

impl SanctumQuery {
    /// Create a new query with an embedding vector
    ///
    /// # Arguments
    ///
    /// * `embedding` - The query vector
    /// * `top_k` - Maximum number of results to return
    pub fn new(embedding: Vec<f32>, top_k: usize) -> Self {
        Self {
            embedding,
            top_k,
            filter: None,
            min_score: None,
        }
    }

    /// Add a filter to the query
    pub fn with_filter(mut self, filter: SanctumFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Add a filter to the query (short alias for with_filter)
    pub fn filter(self, filter: SanctumFilter) -> Self {
        self.with_filter(filter)
    }

    /// Set minimum similarity score threshold
    pub fn with_min_score(mut self, min_score: f32) -> Self {
        self.min_score = Some(min_score);
        self
    }

    /// Set minimum similarity score threshold (short alias for with_min_score)
    pub fn min_score(self, min_score: f32) -> Self {
        self.with_min_score(min_score)
    }
}

/// Search result containing an entry and its similarity score
///
/// Results are typically sorted by score in descending order.
#[derive(Debug, Clone)]
pub struct SanctumSearchResult {
    /// The matching entry
    pub entry: SanctumEntry,

    /// Similarity score (0.0 - 1.0, higher is more similar)
    pub score: f32,
}

impl SanctumSearchResult {
    /// Create a new search result
    pub fn new(entry: SanctumEntry, score: f32) -> Self {
        Self { entry, score }
    }
}

/// Port trait for vector storage and semantic search
///
/// This trait provides a standardized interface for storing and retrieving
/// vector embeddings with associated memories. Implementations can use
/// different vector databases (Qdrant, Pinecone, in-memory, etc.).
///
/// # Examples
///
/// ```ignore
/// use paladin::application::ports::output::sanctum_port::{SanctumPort, SanctumQuery};
///
/// async fn search_memories(port: &dyn SanctumPort, query_embedding: Vec<f32>) {
///     let query = SanctumQuery::new(query_embedding, 10);
///     let results = port.search(query).await.unwrap();
///     
///     for result in results {
///         println!("Score: {}, Content: {}", result.score, result.entry.memory.content);
///     }
/// }
/// ```
#[async_trait]
pub trait SanctumPort: Send + Sync {
    /// Store a single entry in the vector database
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to store
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::StorageError` if the operation fails
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError>;

    /// Store multiple entries in a batch operation
    ///
    /// This is more efficient than calling `store()` multiple times.
    ///
    /// # Arguments
    ///
    /// * `entries` - The entries to store
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::StorageError` if the operation fails
    async fn store_batch(&self, entries: Vec<SanctumEntry>) -> Result<(), SanctumError>;

    /// Search for similar entries using semantic similarity
    ///
    /// # Arguments
    ///
    /// * `query` - The search query with embedding and filters
    ///
    /// # Returns
    ///
    /// A vector of search results sorted by similarity score (descending)
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::SearchError` if the operation fails
    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError>;

    /// Delete an entry by its memory ID
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the memory to delete (as string)
    ///
    /// # Returns
    ///
    /// `true` if the entry was found and deleted, `false` if not found
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::StorageError` if the operation fails
    async fn delete(&self, id: &str) -> Result<bool, SanctumError>;

    /// Update an existing entry
    ///
    /// # Arguments
    ///
    /// * `entry` - The updated entry (must have existing ID)
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::StorageError` if the operation fails
    /// Returns `SanctumError::NotFound` if the entry doesn't exist
    async fn update(&self, entry: SanctumEntry) -> Result<(), SanctumError>;

    /// Get the total count of stored entries
    ///
    /// # Returns
    ///
    /// The number of entries in the database
    /// Count total entries, optionally filtered by criteria
    ///
    /// # Arguments
    /// * `filter` - Optional filter to apply
    ///
    /// # Returns
    ///
    /// Total count of entries matching the filter (or all entries if no filter)
    ///
    /// # Errors
    ///
    /// Returns `SanctumError::StorageError` if the operation fails
    async fn count(&self, filter: Option<SanctumFilter>) -> Result<usize, SanctumError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanctum_filter_builder() {
        let filter = SanctumFilter::new()
            .paladin_id("test-123".to_string())
            .memory_type(MemoryType::Semantic)
            .min_importance(0.7);

        assert_eq!(filter.paladin_id, Some("test-123".to_string()));
        assert_eq!(filter.memory_type, Some(MemoryType::Semantic));
        assert_eq!(filter.min_importance, Some(0.7));
    }

    #[test]
    fn test_sanctum_query_builder() {
        let embedding = vec![0.1, 0.2, 0.3];
        let query = SanctumQuery::new(embedding.clone(), 10).with_min_score(0.8);

        assert_eq!(query.embedding, embedding);
        assert_eq!(query.top_k, 10);
        assert_eq!(query.min_score, Some(0.8));
    }

    #[test]
    fn test_sanctum_filter_default() {
        let filter = SanctumFilter::default();
        assert!(filter.paladin_id.is_none());
        assert!(filter.memory_type.is_none());
        assert!(filter.metadata_filters.is_empty());
    }
}
