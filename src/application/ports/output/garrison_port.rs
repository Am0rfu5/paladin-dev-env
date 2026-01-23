//! Garrison Port - Memory Operations Interface
//!
//! This module defines the port (interface) for Garrison memory operations,
//! following hexagonal architecture principles. Implementations can be in-memory,
//! database-backed, or any other storage mechanism.
//!
//! # Traits
//!
//! - [`GarrisonPort`]: Basic memory operations (CRUD)
//! - [`LongTermGarrisonPort`]: Extended operations with vector embeddings

use crate::core::platform::container::garrison::GarrisonEntry;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Statistics about a Garrison's current state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonStats {
    /// Total number of entries currently stored
    pub entry_count: usize,
    /// Total tokens across all entries (if tracked)
    pub total_tokens: u32,
    /// Approximate size in bytes (if available)
    pub size_bytes: Option<usize>,
}

/// Errors that can occur during Garrison operations
#[derive(Debug, Error)]
pub enum GarrisonError {
    /// Error occurred in underlying storage (database, file system, etc.)
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Failed to serialize or deserialize data
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Failed to calculate token count
    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    /// Requested entry was not found
    #[error("Entry not found: {0}")]
    NotFound(String),

    /// Configuration is invalid
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),
}

/// Port for basic Garrison memory operations
///
/// This trait defines the core interface for storing and retrieving conversation
/// history. All Garrison implementations must implement this trait.
///
/// # Thread Safety
///
/// All implementations must be `Send + Sync` to support async operations across
/// thread boundaries.
///
/// # Examples
///
/// ```no_run
/// use paladin::application::ports::output::garrison_port::GarrisonPort;
/// use paladin::core::platform::container::garrison::{GarrisonEntry, ConversationRole};
///
/// async fn example(garrison: &dyn GarrisonPort) -> Result<(), Box<dyn std::error::Error>> {
///     // Create and store an entry
///     let entry = GarrisonEntry::new(
///         ConversationRole::User,
///         "Hello!".to_string()
///     );
///     garrison.remember(entry).await?;
///
///     // Retrieve recent entries
///     let recent = garrison.recall_recent(10).await?;
///     println!("Found {} recent entries", recent.len());
///
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait GarrisonPort: Send + Sync {
    /// Stores a new entry in the Garrison
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to store
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if the storage operation fails.
    /// Returns [`GarrisonError::SerializationError`] if the entry cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # use paladin::core::platform::container::garrison::{GarrisonEntry, ConversationRole};
    /// # async fn example(garrison: &dyn GarrisonPort) {
    /// let entry = GarrisonEntry::new(
    ///     ConversationRole::User,
    ///     "Store this message".to_string()
    /// );
    /// garrison.remember(entry).await.expect("Failed to store entry");
    /// # }
    /// ```
    async fn remember(&self, entry: GarrisonEntry) -> Result<(), GarrisonError>;

    /// Retrieves the N most recent entries
    ///
    /// Returns entries in chronological order (oldest first).
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of entries to retrieve
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if the retrieval fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # async fn example(garrison: &dyn GarrisonPort) {
    /// let recent = garrison.recall_recent(5).await.expect("Failed to recall");
    /// for entry in recent {
    ///     println!("{:?}: {}", entry.role, entry.content);
    /// }
    /// # }
    /// ```
    async fn recall_recent(&self, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;

    /// Searches for entries matching a text query
    ///
    /// The exact search behavior (substring, full-text, etc.) is implementation-specific.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string
    /// * `limit` - Maximum number of results to return
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if the search fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # async fn example(garrison: &dyn GarrisonPort) {
    /// let results = garrison.search("error", 10).await.expect("Search failed");
    /// println!("Found {} entries containing 'error'", results.len());
    /// # }
    /// ```
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;

    /// Clears all entries from the Garrison
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if the clear operation fails.
    ///
    /// # Warning
    ///
    /// This operation is irreversible. All conversation history will be lost.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # async fn example(garrison: &dyn GarrisonPort) {
    /// garrison.forget_all().await.expect("Failed to clear garrison");
    /// # }
    /// ```
    async fn forget_all(&self) -> Result<(), GarrisonError>;

    /// Returns statistics about the current state of the Garrison
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if statistics cannot be calculated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # async fn example(garrison: &dyn GarrisonPort) {
    /// let stats = garrison.stats().await.expect("Failed to get stats");
    /// println!("Entries: {}, Tokens: {}", stats.entry_count, stats.total_tokens);
    /// # }
    /// ```
    async fn stats(&self) -> Result<GarrisonStats, GarrisonError>;
}

/// Extended port for long-term memory with semantic search capabilities
///
/// This trait extends [`GarrisonPort`] with vector embedding support for semantic
/// similarity search. Not all Garrison implementations need to support this.
///
/// # Use Cases
///
/// - Semantic search across historical conversations
/// - Finding conceptually similar past interactions
/// - Building knowledge bases with similarity-based retrieval
///
/// # Examples
///
/// ```no_run
/// use paladin::application::ports::output::garrison_port::LongTermGarrisonPort;
/// use paladin::core::platform::container::garrison::{GarrisonEntry, ConversationRole};
///
/// async fn example(
///     garrison: &dyn LongTermGarrisonPort,
///     embedding_model: &dyn Fn(&str) -> Vec<f32>
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     // Store with embedding
///     let entry = GarrisonEntry::new(
///         ConversationRole::User,
///         "What is machine learning?".to_string()
///     );
///     let embedding = embedding_model(&entry.content);
///     garrison.remember_with_embedding(entry, embedding).await?;
///
///     // Search by semantic similarity
///     let query_embedding = embedding_model("AI fundamentals");
///     let similar = garrison.search_similar(query_embedding, 5).await?;
///     
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait LongTermGarrisonPort: GarrisonPort {
    /// Stores an entry with its vector embedding for semantic search
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to store
    /// * `embedding` - Vector representation of the entry content
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if storage fails.
    /// Returns [`GarrisonError::SerializationError`] if data cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::LongTermGarrisonPort;
    /// # use paladin::core::platform::container::garrison::{GarrisonEntry, ConversationRole};
    /// # async fn example(garrison: &dyn LongTermGarrisonPort) {
    /// let entry = GarrisonEntry::new(
    ///     ConversationRole::User,
    ///     "Important information".to_string()
    /// );
    /// let embedding = vec![0.1, 0.2, 0.3]; // From embedding model
    /// garrison.remember_with_embedding(entry, embedding).await
    ///     .expect("Failed to store with embedding");
    /// # }
    /// ```
    async fn remember_with_embedding(
        &self,
        entry: GarrisonEntry,
        embedding: Vec<f32>,
    ) -> Result<(), GarrisonError>;

    /// Searches for entries similar to the given embedding
    ///
    /// Returns entries ranked by cosine similarity (most similar first).
    ///
    /// # Arguments
    ///
    /// * `embedding` - Query vector to find similar entries
    /// * `limit` - Maximum number of results to return
    ///
    /// # Errors
    ///
    /// Returns [`GarrisonError::StorageError`] if search fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use paladin::application::ports::output::garrison_port::LongTermGarrisonPort;
    /// # async fn example(garrison: &dyn LongTermGarrisonPort) {
    /// let query_embedding = vec![0.15, 0.25, 0.35]; // From embedding model
    /// let similar = garrison.search_similar(query_embedding, 10).await
    ///     .expect("Failed to search");
    ///
    /// for entry in similar {
    ///     println!("Similar: {}", entry.content);
    /// }
    /// # }
    /// ```
    async fn search_similar(
        &self,
        embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<GarrisonEntry>, GarrisonError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garrison_stats_creation() {
        let stats = GarrisonStats {
            entry_count: 42,
            total_tokens: 1000,
            size_bytes: Some(8192),
        };

        assert_eq!(stats.entry_count, 42);
        assert_eq!(stats.total_tokens, 1000);
        assert_eq!(stats.size_bytes, Some(8192));
    }

    #[test]
    fn test_garrison_error_display() {
        let error = GarrisonError::StorageError("Database connection failed".to_string());
        assert_eq!(
            error.to_string(),
            "Storage error: Database connection failed"
        );

        let error = GarrisonError::NotFound("entry-123".to_string());
        assert_eq!(error.to_string(), "Entry not found: entry-123");
    }

    #[test]
    fn test_garrison_stats_serialization() {
        let stats = GarrisonStats {
            entry_count: 10,
            total_tokens: 500,
            size_bytes: None,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: GarrisonStats = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.entry_count, 10);
        assert_eq!(deserialized.total_tokens, 500);
        assert_eq!(deserialized.size_bytes, None);
    }
}
