/// Sanctum Domain Models
///
/// Core domain types for the Sanctum long-term memory system.
/// These models represent the fundamental concepts of vector-based memory storage
/// with no dependencies on external infrastructure.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Type of memory being stored
///
/// Classifies memories based on their nature and use case:
/// - **Episodic**: Specific events, conversations, or experiences
/// - **Semantic**: Facts, knowledge, and general information
/// - **Procedural**: How-to instructions, procedures, and workflows
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    /// Memories of specific events or conversations
    Episodic,

    /// Facts and general knowledge
    Semantic,

    /// Procedural knowledge and instructions
    Procedural,
}

/// Strategy for memory importance decay over time
///
/// Determines how memory importance changes as time passes:
/// - **NoDecay**: Importance never decreases
/// - **LinearDecay**: Importance decreases linearly with time
/// - **AccessBasedDecay**: Importance decreases unless memory is accessed
/// - **CustomDecay**: User-defined decay function
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryDecayStrategy {
    /// Memory importance never decreases
    NoDecay,

    /// Importance decreases linearly over time
    LinearDecay,

    /// Importance decreases unless memory is accessed/reinforced
    AccessBasedDecay,

    /// Custom decay strategy (implementation-specific)
    CustomDecay,
}

/// A memory represents a stored piece of knowledge or experience
///
/// Memories are the core domain entity for long-term storage in Sanctum.
/// They contain the actual content, metadata, and tracking information
/// needed for semantic search and retrieval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier for this memory
    pub id: Uuid,

    /// ID of the Paladin that owns this memory
    pub paladin_id: String,

    /// The actual content of the memory
    pub content: String,

    /// Type of memory (episodic, semantic, procedural)
    pub memory_type: MemoryType,

    /// Importance score (0.0 - 1.0)
    pub importance: f32,

    /// Number of times this memory has been accessed
    pub access_count: u32,

    /// When this memory was last accessed
    pub last_accessed: DateTime<Utc>,

    /// When this memory was created
    pub created_at: DateTime<Utc>,

    /// Additional metadata for filtering and context
    pub metadata: HashMap<String, Value>,
}

impl Memory {
    /// Increment the access count and update last accessed timestamp
    pub fn increment_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Utc::now();
    }

    /// Validate the importance score is within valid range
    pub fn validate_importance(importance: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&importance) {
            return Err(format!(
                "Importance must be between 0.0 and 1.0, got: {}",
                importance
            ));
        }
        Ok(())
    }
}

/// Builder for creating Memory instances with validation
///
/// # Examples
///
/// ```ignore
/// use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType};
///
/// let memory = MemoryBuilder::new("paladin-123".to_string(), "Rust is awesome".to_string())
///     .memory_type(MemoryType::Semantic)
///     .importance(0.9)
///     .build()
///     .unwrap();
/// ```
pub struct MemoryBuilder {
    paladin_id: String,
    content: String,
    memory_type: MemoryType,
    importance: f32,
    metadata: HashMap<String, Value>,
}

impl MemoryBuilder {
    /// Create a new memory builder
    ///
    /// # Arguments
    ///
    /// * `paladin_id` - The ID of the Paladin creating this memory
    /// * `content` - The actual content to store
    pub fn new(paladin_id: String, content: String) -> Self {
        Self {
            paladin_id,
            content,
            memory_type: MemoryType::Episodic,
            importance: 0.5,
            metadata: HashMap::new(),
        }
    }

    /// Set the memory type
    pub fn memory_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_type = memory_type;
        self
    }

    /// Set the importance score (0.0 - 1.0)
    pub fn importance(mut self, importance: f32) -> Self {
        self.importance = importance;
        self
    }

    /// Set the metadata
    pub fn metadata(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Add a single metadata entry
    pub fn add_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the Memory instance with validation
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Importance is not between 0.0 and 1.0
    /// - Content is empty
    pub fn build(self) -> Result<Memory, String> {
        // Validate importance
        Memory::validate_importance(self.importance)?;

        // Validate content
        if self.content.is_empty() {
            return Err("Content cannot be empty".to_string());
        }

        let now = Utc::now();
        Ok(Memory {
            id: Uuid::new_v4(),
            paladin_id: self.paladin_id,
            content: self.content,
            memory_type: self.memory_type,
            importance: self.importance,
            access_count: 0,
            last_accessed: now,
            created_at: now,
            metadata: self.metadata,
        })
    }
}

/// A Sanctum entry combines a memory with its vector embedding
///
/// This is the complete unit that gets stored in the vector database.
/// It contains both the semantic meaning (embedding) and the actual content (memory).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctumEntry {
    /// The memory being stored
    pub memory: Memory,

    /// The vector embedding of the memory content
    pub embedding: Vec<f32>,

    /// Dimension of the embedding vector
    pub dimension: usize,
}

impl SanctumEntry {
    /// Create a new Sanctum entry
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to store
    /// * `embedding` - The vector embedding of the memory content
    ///
    /// # Errors
    ///
    /// Returns an error if the embedding is empty
    pub fn new(memory: Memory, embedding: Vec<f32>) -> Result<Self, String> {
        if embedding.is_empty() {
            return Err("Embedding cannot be empty".to_string());
        }

        let dimension = embedding.len();

        Ok(Self {
            memory,
            embedding,
            dimension,
        })
    }

    /// Get the Paladin ID for this entry
    pub fn paladin_id(&self) -> &str {
        &self.memory.paladin_id
    }

    /// Get the memory ID for this entry
    pub fn id(&self) -> Uuid {
        self.memory.id
    }

    /// Validate that the embedding dimension matches the expected dimension
    pub fn validate_dimension(&self, expected_dimension: usize) -> Result<(), String> {
        if self.dimension != expected_dimension {
            return Err(format!(
                "Embedding dimension {} does not match expected dimension {}",
                self.dimension, expected_dimension
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_type_default() {
        let builder = MemoryBuilder::new("p1".to_string(), "content".to_string());
        assert_eq!(builder.memory_type, MemoryType::Episodic);
    }

    #[test]
    fn test_importance_default() {
        let builder = MemoryBuilder::new("p1".to_string(), "content".to_string());
        assert_eq!(builder.importance, 0.5);
    }

    #[test]
    fn test_empty_content_error() {
        let result = MemoryBuilder::new("p1".to_string(), "".to_string()).build();
        assert!(result.is_err());
    }
}
