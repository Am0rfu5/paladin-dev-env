use paladin::core::platform::container::sanctum::MemoryType;
use paladin_ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumQuery, SanctumSearchResult,
};

#[cfg(test)]
mod sanctum_error_tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        let error = SanctumError::StorageError("Failed to write".to_string());
        assert_eq!(error.to_string(), "Storage error: Failed to write");
    }

    #[test]
    fn test_search_error_display() {
        let error = SanctumError::SearchError("Query failed".to_string());
        assert_eq!(error.to_string(), "Search error: Query failed");
    }

    #[test]
    fn test_invalid_dimension_error() {
        let error = SanctumError::InvalidDimension("Expected 1536, got 512".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid dimension: Expected 1536, got 512"
        );
    }

    #[test]
    fn test_not_found_error() {
        let error = SanctumError::NotFound("Entry abc123 not found".to_string());
        assert_eq!(error.to_string(), "Not found: Entry abc123 not found");
    }

    #[test]
    fn test_config_error() {
        let error = SanctumError::ConfigError("Invalid connection string".to_string());
        assert_eq!(
            error.to_string(),
            "Configuration error: Invalid connection string"
        );
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SanctumError>();
    }
}

#[cfg(test)]
mod sanctum_filter_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_filter_new() {
        let filter = SanctumFilter::new();
        assert!(filter.paladin_id.is_none());
        assert!(filter.memory_type.is_none());
        assert!(filter.min_importance.is_none());
    }

    #[test]
    fn test_filter_paladin_id() {
        let filter = SanctumFilter::new().paladin_id("test-123".to_string());
        assert_eq!(filter.paladin_id, Some("test-123".to_string()));
    }

    #[test]
    fn test_filter_memory_type() {
        let filter = SanctumFilter::new().memory_type(MemoryType::Semantic);
        assert_eq!(filter.memory_type, Some(MemoryType::Semantic));
    }

    #[test]
    fn test_filter_min_importance() {
        let filter = SanctumFilter::new().min_importance(0.7);
        assert_eq!(filter.min_importance, Some(0.7));
    }

    #[test]
    fn test_filter_chaining() {
        let filter = SanctumFilter::new()
            .paladin_id("p1".to_string())
            .memory_type(MemoryType::Episodic)
            .min_importance(0.5);

        assert_eq!(filter.paladin_id, Some("p1".to_string()));
        assert_eq!(filter.memory_type, Some(MemoryType::Episodic));
        assert_eq!(filter.min_importance, Some(0.5));
    }

    #[test]
    fn test_filter_metadata_filters() {
        let filter = SanctumFilter::new()
            .add_metadata_filter("topic".to_string(), json!("rust"))
            .add_metadata_filter("priority".to_string(), json!("high"));

        assert_eq!(filter.metadata_filters.len(), 2);
        assert_eq!(
            filter.metadata_filters.get("topic").unwrap(),
            &json!("rust")
        );
    }

    #[test]
    fn test_filter_serialization() {
        let filter = SanctumFilter::new()
            .paladin_id("test".to_string())
            .memory_type(MemoryType::Procedural);

        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: SanctumFilter = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.paladin_id, filter.paladin_id);
        assert_eq!(deserialized.memory_type, filter.memory_type);
    }
}

#[cfg(test)]
mod sanctum_query_tests {
    use super::*;

    #[test]
    fn test_query_new() {
        let embedding = vec![0.1, 0.2, 0.3];
        let query = SanctumQuery::new(embedding.clone(), 10);

        assert_eq!(query.embedding, embedding);
        assert_eq!(query.top_k, 10);
        assert!(query.filter.is_none());
        assert!(query.min_score.is_none());
    }

    #[test]
    fn test_query_with_filter() {
        let filter = SanctumFilter::new().paladin_id("test".to_string());
        let query = SanctumQuery::new(vec![0.1], 5).with_filter(filter.clone());

        assert!(query.filter.is_some());
        assert_eq!(query.filter.unwrap().paladin_id, Some("test".to_string()));
    }

    #[test]
    fn test_query_with_min_score() {
        let query = SanctumQuery::new(vec![0.1], 5).with_min_score(0.8);

        assert_eq!(query.min_score, Some(0.8));
    }

    #[test]
    fn test_query_chaining() {
        let filter = SanctumFilter::new().paladin_id("p1".to_string());
        let query = SanctumQuery::new(vec![0.1, 0.2], 15)
            .with_filter(filter)
            .with_min_score(0.75);

        assert_eq!(query.top_k, 15);
        assert!(query.filter.is_some());
        assert_eq!(query.min_score, Some(0.75));
    }

    #[test]
    fn test_query_embedding_dimension() {
        let embedding = vec![0.1; 1536];
        let query = SanctumQuery::new(embedding.clone(), 5);

        assert_eq!(query.embedding.len(), 1536);
    }
}

#[cfg(test)]
mod sanctum_search_result_tests {
    use super::*;
    use paladin::core::platform::container::sanctum::{MemoryBuilder, SanctumEntry};

    #[test]
    fn test_search_result_new() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test".to_string())
            .build()
            .unwrap();
        let entry = SanctumEntry::new(memory, vec![0.1, 0.2]).unwrap();
        let result = SanctumSearchResult::new(entry.clone(), 0.95);

        assert_eq!(result.score, 0.95);
        assert_eq!(result.entry.memory.content, "Test");
    }

    #[test]
    fn test_search_result_score_range() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test".to_string())
            .build()
            .unwrap();
        let entry = SanctumEntry::new(memory, vec![0.1]).unwrap();

        // Test various score values
        let result_low = SanctumSearchResult::new(entry.clone(), 0.1);
        assert_eq!(result_low.score, 0.1);

        let result_high = SanctumSearchResult::new(entry.clone(), 1.0);
        assert_eq!(result_high.score, 1.0);

        let result_zero = SanctumSearchResult::new(entry, 0.0);
        assert_eq!(result_zero.score, 0.0);
    }

    #[test]
    fn test_search_result_entry_access() {
        let memory = MemoryBuilder::new("paladin-123".to_string(), "Content".to_string())
            .build()
            .unwrap();
        let memory_id = memory.id;
        let entry = SanctumEntry::new(memory, vec![0.5]).unwrap();
        let result = SanctumSearchResult::new(entry, 0.85);

        assert_eq!(result.entry.paladin_id(), "paladin-123");
        assert_eq!(result.entry.id(), memory_id);
    }
}
