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

    #[test]
    fn test_sanctum_entry_dimension_validation() {
        // Test: Entry with valid embedding
        let memory = create_test_memory("paladin-1", "Valid entry", 0.8);
        let embedding = vec![0.1, 0.2, 0.3];
        let entry = SanctumEntry::new(memory.clone(), embedding);
        assert!(entry.is_ok());

        // Test: Entry with empty embedding should fail
        let empty_embedding: Vec<f32> = vec![];
        let entry = SanctumEntry::new(memory.clone(), empty_embedding);
        assert!(entry.is_err());
    }

    #[test]
    fn test_sanctum_entry_normalized_embedding() {
        let memory = create_test_memory("paladin-1", "Test", 0.5);
        let embedding = vec![1.0, 1.0, 1.0, 1.0]; // Not normalized
        
        let entry = SanctumEntry::new(memory, embedding.clone()).unwrap();
        
        // Verify embedding is stored (normalization is optional)
        assert_eq!(entry.embedding.len(), 4);
        assert_eq!(entry.dimension, 4);
    }

    #[test]
    fn test_memory_type_variants() {
        // Test: All MemoryType variants can be used
        let types = vec![
            MemoryType::Episodic,
            MemoryType::Semantic,
            MemoryType::Procedural,
            MemoryType::Working,
        ];

        for memory_type in types {
            let memory = MemoryBuilder::new("paladin-1".to_string(), "Test".to_string())
                .memory_type(memory_type)
                .build()
                .unwrap();
            
            assert_eq!(memory.memory_type, memory_type);
        }
    }

    #[test]
    fn test_memory_importance_bounds() {
        // Test: Valid importance values (0.0 to 1.0)
        let valid_importances = vec![0.0, 0.5, 1.0];
        
        for importance in valid_importances {
            let memory = MemoryBuilder::new("paladin-1".to_string(), "Test".to_string())
                .importance(importance)
                .build();
            
            assert!(memory.is_ok());
            assert_eq!(memory.unwrap().importance, importance);
        }

        // Test: Out-of-bounds importance (should be clamped or validated)
        let memory = MemoryBuilder::new("paladin-1".to_string(), "Test".to_string())
            .importance(1.5) // Over 1.0
            .build();
        
        // Depending on implementation, this might fail or clamp
        match memory {
            Ok(m) => assert!(m.importance <= 1.0, "Importance should be clamped to 1.0"),
            Err(_) => (), // Validation error is also acceptable
        }
    }

    #[test]
    fn test_sanctum_filter_builder() {
        // Test: Filter with paladin_id
        let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
        assert!(filter.paladin_id.is_some());
        assert_eq!(filter.paladin_id.unwrap(), "paladin-1");

        // Test: Filter with memory_type
        let filter = SanctumFilter::new().memory_type(MemoryType::Semantic);
        assert!(filter.memory_type.is_some());
        assert_eq!(filter.memory_type.unwrap(), MemoryType::Semantic);

        // Test: Filter with min_importance
        let filter = SanctumFilter::new().min_importance(0.7);
        assert!(filter.min_importance.is_some());
        assert_eq!(filter.min_importance.unwrap(), 0.7);

        // Test: Chained filter building
        let filter = SanctumFilter::new()
            .paladin_id("paladin-1".to_string())
            .memory_type(MemoryType::Episodic)
            .min_importance(0.8);
        
        assert_eq!(filter.paladin_id.unwrap(), "paladin-1");
        assert_eq!(filter.memory_type.unwrap(), MemoryType::Episodic);
        assert_eq!(filter.min_importance.unwrap(), 0.8);
    }

    #[test]
    fn test_sanctum_query_builder() {
        let embedding = vec![0.1, 0.2, 0.3];
        
        // Test: Basic query
        let query = SanctumQuery::new(embedding.clone(), 5);
        assert_eq!(query.embedding.len(), 3);
        assert_eq!(query.top_k, 5);
        assert!(query.filter.is_none());
        assert!(query.min_score.is_none());

        // Test: Query with filter
        let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
        let query = SanctumQuery::new(embedding.clone(), 10).with_filter(filter);
        assert!(query.filter.is_some());
        assert_eq!(query.filter.unwrap().paladin_id.unwrap(), "paladin-1");

        // Test: Query with min_score
        let query = SanctumQuery::new(embedding.clone(), 5).with_min_score(0.7);
        assert_eq!(query.min_score.unwrap(), 0.7);

        // Test: Full query builder chain
        let filter = SanctumFilter::new().memory_type(MemoryType::Semantic);
        let query = SanctumQuery::new(embedding, 3)
            .with_filter(filter)
            .with_min_score(0.85);
        
        assert_eq!(query.top_k, 3);
        assert_eq!(query.min_score.unwrap(), 0.85);
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_memory_content_validation() {
        // Test: Valid content
        let memory = MemoryBuilder::new("paladin-1".to_string(), "Valid content".to_string())
            .build();
        assert!(memory.is_ok());

        // Test: Empty content (should be allowed or validated)
        let memory = MemoryBuilder::new("paladin-1".to_string(), "".to_string()).build();
        match memory {
            Ok(m) => assert!(m.content.is_empty()),
            Err(_) => (), // Validation error is acceptable
        }
    }

    #[test]
    fn test_sanctum_entry_id_uniqueness() {
        // Test: Each entry gets a unique ID
        let entry1 = create_test_entry("paladin-1", "First entry", 0.8);
        let entry2 = create_test_entry("paladin-1", "Second entry", 0.8);
        
        assert_ne!(entry1.id(), entry2.id(), "Each entry should have a unique ID");
    }

    #[test]
    fn test_memory_metadata_operations() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), json!("value1"));
        metadata.insert("key2".to_string(), json!(42));
        metadata.insert("key3".to_string(), json!(true));
        
        let memory = MemoryBuilder::new("paladin-1".to_string(), "Test".to_string())
            .metadata(metadata.clone())
            .build()
            .unwrap();
        
        // Verify all metadata is preserved
        assert_eq!(memory.metadata.len(), 3);
        assert_eq!(memory.metadata.get("key1"), Some(&json!("value1")));
        assert_eq!(memory.metadata.get("key2"), Some(&json!(42)));
        assert_eq!(memory.metadata.get("key3"), Some(&json!(true)));
    }

    #[test]
    fn test_memory_timestamps() {
        let memory = create_test_memory("paladin-1", "Test", 0.5);
        
        // Verify timestamps are set
        assert!(memory.created_at <= Utc::now());
        assert!(memory.updated_at <= Utc::now());
        assert_eq!(memory.created_at, memory.updated_at); // Should be equal for new memories
    }
}

```
