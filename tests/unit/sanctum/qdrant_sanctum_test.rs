/// Unit tests for Qdrant Sanctum Adapter
///
/// These tests verify the QdrantSanctum adapter implementation with mocked Qdrant client.
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use crate::application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery,
};
use crate::core::platform::container::sanctum::{Memory, MemoryBuilder, MemoryType, SanctumEntry};
use crate::infrastructure::adapters::sanctum::qdrant_adapter::QdrantSanctumAdapter;

// Helper function to create a test memory
fn create_test_memory(paladin_id: &str, content: &str, importance: f32) -> Memory {
    MemoryBuilder::new(paladin_id.to_string(), content.to_string())
        .importance(importance)
        .memory_type(MemoryType::Semantic)
        .build()
        .unwrap()
}

// Helper function to create a test entry
fn create_test_entry(paladin_id: &str, content: &str, importance: f32) -> SanctumEntry {
    let memory = create_test_memory(paladin_id, content, importance);
    let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5]; // 5-dimensional test embedding
    SanctumEntry::new(memory, embedding).unwrap()
}

#[cfg(test)]
mod qdrant_sanctum_tests {
    use super::*;

    // Note: These are placeholder tests that will be implemented
    // once the actual Qdrant client mocking is set up

    #[test]
    fn test_sanctum_entry_creation() {
        let entry = create_test_entry("paladin-1", "Test memory", 0.8);
        assert_eq!(entry.paladin_id(), "paladin-1");
        assert_eq!(entry.memory.content, "Test memory");
        assert_eq!(entry.memory.importance, 0.8);
        assert_eq!(entry.dimension, 5);
    }

    #[test]
    fn test_memory_builder_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), json!("conversation"));
        
        let memory = MemoryBuilder::new("paladin-1".to_string(), "Test content".to_string())
            .importance(0.9)
            .memory_type(MemoryType::Episodic)
            .metadata(metadata.clone())
            .build()
            .unwrap();
        
        assert_eq!(memory.metadata.get("source"), Some(&json!("conversation")));
    }

    // TODO: Add integration tests with actual Qdrant client
    // These will be added in the integration test file
}
