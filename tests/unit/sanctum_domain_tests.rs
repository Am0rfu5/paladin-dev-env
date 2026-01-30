use chrono::Utc;
use paladin::core::platform::container::sanctum::{
    Memory, MemoryBuilder, MemoryDecayStrategy, MemoryType, SanctumEntry,
};
use serde_json::json;
use std::collections::HashMap;

#[cfg(test)]
mod memory_type_tests {
    use super::*;

    #[test]
    fn test_memory_type_variants() {
        let episodic = MemoryType::Episodic;
        let semantic = MemoryType::Semantic;
        let procedural = MemoryType::Procedural;

        assert_eq!(format!("{:?}", episodic), "Episodic");
        assert_eq!(format!("{:?}", semantic), "Semantic");
        assert_eq!(format!("{:?}", procedural), "Procedural");
    }

    #[test]
    fn test_memory_type_equality() {
        assert_eq!(MemoryType::Episodic, MemoryType::Episodic);
        assert_ne!(MemoryType::Episodic, MemoryType::Semantic);
    }

    #[test]
    fn test_memory_type_serialization() {
        let memory_type = MemoryType::Semantic;
        let json = serde_json::to_string(&memory_type).unwrap();
        let deserialized: MemoryType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, memory_type);
    }

    #[test]
    fn test_memory_type_is_copy() {
        let mt1 = MemoryType::Episodic;
        let mt2 = mt1; // Copy, not move
        assert_eq!(mt1, mt2);
    }
}

#[cfg(test)]
mod memory_decay_strategy_tests {
    use super::*;

    #[test]
    fn test_decay_strategy_variants() {
        let no_decay = MemoryDecayStrategy::NoDecay;
        let linear = MemoryDecayStrategy::LinearDecay;
        let access_based = MemoryDecayStrategy::AccessBasedDecay;
        let custom = MemoryDecayStrategy::CustomDecay;

        assert_eq!(format!("{:?}", no_decay), "NoDecay");
        assert_eq!(format!("{:?}", linear), "LinearDecay");
        assert_eq!(format!("{:?}", access_based), "AccessBasedDecay");
        assert_eq!(format!("{:?}", custom), "CustomDecay");
    }

    #[test]
    fn test_decay_strategy_serialization() {
        let strategy = MemoryDecayStrategy::AccessBasedDecay;
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: MemoryDecayStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, strategy);
    }
}

#[cfg(test)]
mod memory_tests {
    use super::*;

    #[test]
    fn test_memory_builder_basic() {
        let memory = MemoryBuilder::new("paladin-123".to_string(), "Test content".to_string())
            .memory_type(MemoryType::Semantic)
            .importance(0.8)
            .build()
            .unwrap();

        assert_eq!(memory.paladin_id, "paladin-123");
        assert_eq!(memory.content, "Test content");
        assert_eq!(memory.memory_type, MemoryType::Semantic);
        assert_eq!(memory.importance, 0.8);
        assert_eq!(memory.access_count, 0);
    }

    #[test]
    fn test_memory_builder_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), json!("conversation"));
        metadata.insert("topic".to_string(), json!("rust programming"));

        let memory = MemoryBuilder::new("paladin-456".to_string(), "Rust is great".to_string())
            .memory_type(MemoryType::Episodic)
            .importance(0.9)
            .metadata(metadata.clone())
            .build()
            .unwrap();

        assert_eq!(memory.metadata.len(), 2);
        assert_eq!(
            memory.metadata.get("source").unwrap(),
            &json!("conversation")
        );
    }

    #[test]
    fn test_memory_importance_validation_valid() {
        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .importance(0.0)
            .build();
        assert!(memory.is_ok());

        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .importance(1.0)
            .build();
        assert!(memory.is_ok());

        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .importance(0.5)
            .build();
        assert!(memory.is_ok());
    }

    #[test]
    fn test_memory_importance_validation_invalid() {
        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .importance(-0.1)
            .build();
        assert!(memory.is_err());

        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .importance(1.1)
            .build();
        assert!(memory.is_err());
    }

    #[test]
    fn test_memory_serialization() {
        let memory = MemoryBuilder::new("paladin-789".to_string(), "Test".to_string())
            .memory_type(MemoryType::Procedural)
            .importance(0.7)
            .build()
            .unwrap();

        let json = serde_json::to_string(&memory).unwrap();
        let deserialized: Memory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.paladin_id, memory.paladin_id);
        assert_eq!(deserialized.content, memory.content);
        assert_eq!(deserialized.memory_type, memory.memory_type);
        assert_eq!(deserialized.importance, memory.importance);
    }

    #[test]
    fn test_memory_default_values() {
        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .build()
            .unwrap();

        assert_eq!(memory.memory_type, MemoryType::Episodic);
        assert_eq!(memory.importance, 0.5);
        assert_eq!(memory.access_count, 0);
        assert!(memory.metadata.is_empty());
    }

    #[test]
    fn test_memory_timestamps() {
        let before = Utc::now();
        let memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .build()
            .unwrap();
        let after = Utc::now();

        assert!(memory.created_at >= before && memory.created_at <= after);
        assert!(memory.last_accessed >= before && memory.last_accessed <= after);
    }

    #[test]
    fn test_memory_increment_access() {
        let mut memory = MemoryBuilder::new("p1".to_string(), "content".to_string())
            .build()
            .unwrap();

        assert_eq!(memory.access_count, 0);

        memory.increment_access();
        assert_eq!(memory.access_count, 1);

        memory.increment_access();
        assert_eq!(memory.access_count, 2);
    }

    #[test]
    fn test_memory_id_is_unique() {
        let memory1 = MemoryBuilder::new("p1".to_string(), "content1".to_string())
            .build()
            .unwrap();
        let memory2 = MemoryBuilder::new("p1".to_string(), "content2".to_string())
            .build()
            .unwrap();

        assert_ne!(memory1.id, memory2.id);
    }
}

#[cfg(test)]
mod sanctum_entry_tests {
    use super::*;

    #[test]
    fn test_sanctum_entry_creation() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test content".to_string())
            .build()
            .unwrap();

        let embedding = vec![0.1, 0.2, 0.3, 0.4];
        let entry = SanctumEntry::new(memory.clone(), embedding.clone()).unwrap();

        assert_eq!(entry.memory.content, "Test content");
        assert_eq!(entry.embedding, embedding);
        assert_eq!(entry.dimension, 4);
    }

    #[test]
    fn test_sanctum_entry_empty_embedding_error() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test".to_string())
            .build()
            .unwrap();

        let result = SanctumEntry::new(memory, vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_sanctum_entry_dimension_validation() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test".to_string())
            .build()
            .unwrap();

        let embedding = vec![0.1; 1536];
        let entry = SanctumEntry::new(memory, embedding).unwrap();

        assert_eq!(entry.dimension, 1536);
    }

    #[test]
    fn test_sanctum_entry_serialization() {
        let memory = MemoryBuilder::new("p1".to_string(), "Test".to_string())
            .build()
            .unwrap();

        let embedding = vec![0.5, 0.6];
        let entry = SanctumEntry::new(memory, embedding.clone()).unwrap();

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: SanctumEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.embedding, embedding);
        assert_eq!(deserialized.dimension, 2);
        assert_eq!(deserialized.memory.content, "Test");
    }

    #[test]
    fn test_sanctum_entry_paladin_id_accessor() {
        let memory = MemoryBuilder::new("test-paladin".to_string(), "Content".to_string())
            .build()
            .unwrap();

        let entry = SanctumEntry::new(memory, vec![0.1, 0.2]).unwrap();
        assert_eq!(entry.paladin_id(), "test-paladin");
    }

    #[test]
    fn test_sanctum_entry_id_accessor() {
        let memory = MemoryBuilder::new("p1".to_string(), "Content".to_string())
            .build()
            .unwrap();

        let memory_id = memory.id;
        let entry = SanctumEntry::new(memory, vec![0.1]).unwrap();

        assert_eq!(entry.id(), memory_id);
    }
}
