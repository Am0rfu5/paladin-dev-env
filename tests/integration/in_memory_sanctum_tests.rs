//! Integration tests for InMemorySanctum adapter
//!
//! Tests the in-memory implementation of SanctumPort trait including:
//! - Storage and retrieval operations
//! - Semantic search with cosine similarity
//! - Filtering by metadata
//! - LRU eviction when capacity is reached
//! - Thread-safety with concurrent operations

use paladin::application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery,
};
use paladin::core::platform::container::sanctum::{
    Memory, MemoryBuilder, MemoryType, SanctumEntry,
};
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;
use std::sync::Arc;
use tokio::task;

/// Helper function to create a test memory
fn create_test_memory(
    paladin_id: &str,
    content: &str,
    memory_type: MemoryType,
    importance: f32,
) -> Memory {
    MemoryBuilder::new(paladin_id.to_string(), content.to_string())
        .memory_type(memory_type)
        .importance(importance)
        .build()
        .expect("Failed to build test memory")
}

/// Helper function to create a test entry with a simple embedding
fn create_test_entry(memory: Memory, embedding: Vec<f32>) -> SanctumEntry {
    SanctumEntry::new(memory, embedding).expect("Failed to create test entry")
}

#[tokio::test]
async fn test_store_and_retrieve() {
    let sanctum = InMemorySanctum::new(100);
    let memory = create_test_memory("paladin-1", "Test content", MemoryType::Episodic, 0.8);
    let entry = create_test_entry(memory.clone(), vec![1.0, 0.0, 0.0]);

    // Store the entry
    sanctum
        .store(entry.clone())
        .await
        .expect("Failed to store entry");

    // Count should be 1
    let count = sanctum.count(None).await.expect("Failed to count entries");
    assert_eq!(count, 1);

    // Search with exact embedding should return the stored entry
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 1);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.id, memory.id);
    assert_eq!(results[0].score, 1.0); // Exact match
}

#[tokio::test]
async fn test_store_batch() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "Content 1", MemoryType::Episodic, 0.8),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Content 2", MemoryType::Semantic, 0.7),
            vec![0.0, 1.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-2", "Content 3", MemoryType::Procedural, 0.9),
            vec![0.0, 0.0, 1.0],
        ),
    ];

    // Store batch
    sanctum
        .store_batch(entries.clone())
        .await
        .expect("Failed to store batch");

    // Count should be 3
    let count = sanctum.count(None).await.expect("Failed to count entries");
    assert_eq!(count, 3);
}

#[tokio::test]
async fn test_cosine_similarity_search() {
    let sanctum = InMemorySanctum::new(100);

    // Store entries with different embeddings
    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "Very similar", MemoryType::Episodic, 0.8),
            vec![1.0, 0.1, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Somewhat similar", MemoryType::Episodic, 0.7),
            vec![0.7, 0.5, 0.2],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Not similar", MemoryType::Episodic, 0.6),
            vec![0.0, 0.0, 1.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with query embedding similar to the first entry
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 3);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 3);

    // Results should be sorted by similarity (descending)
    assert!(results[0].score >= results[1].score);
    assert!(results[1].score >= results[2].score);

    // First result should be "Very similar"
    assert_eq!(results[0].entry.memory.content, "Very similar");
    assert!(results[0].score > 0.9); // High similarity
}

#[tokio::test]
async fn test_search_with_top_k() {
    let sanctum = InMemorySanctum::new(100);

    // Store 5 entries
    let entries: Vec<_> = (0..5)
        .map(|i| {
            create_test_entry(
                create_test_memory(
                    "paladin-1",
                    &format!("Content {}", i),
                    MemoryType::Episodic,
                    0.5,
                ),
                vec![i as f32 / 10.0, 0.5, 0.5],
            )
        })
        .collect();

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with top_k=2
    let query = SanctumQuery::new(vec![0.0, 0.5, 0.5], 2);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_search_with_min_score() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "High similarity", MemoryType::Episodic, 0.8),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Low similarity", MemoryType::Episodic, 0.7),
            vec![0.0, 1.0, 0.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with high min_score to filter out low similarity results
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).min_score(0.9);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.content, "High similarity");
}

#[tokio::test]
async fn test_search_with_paladin_filter() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "Content 1", MemoryType::Episodic, 0.8),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-2", "Content 2", MemoryType::Episodic, 0.7),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Content 3", MemoryType::Episodic, 0.9),
            vec![0.9, 0.1, 0.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with paladin_id filter
    let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert_eq!(result.entry.memory.paladin_id, "paladin-1");
    }
}

#[tokio::test]
async fn test_search_with_memory_type_filter() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "Episodic 1", MemoryType::Episodic, 0.8),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Semantic 1", MemoryType::Semantic, 0.7),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Episodic 2", MemoryType::Episodic, 0.9),
            vec![0.9, 0.1, 0.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with memory_type filter
    let filter = SanctumFilter::new().memory_type(MemoryType::Episodic);
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert_eq!(result.entry.memory.memory_type, MemoryType::Episodic);
    }
}

#[tokio::test]
async fn test_search_with_importance_filter() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "High importance", MemoryType::Episodic, 0.9),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Low importance", MemoryType::Episodic, 0.3),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Medium importance", MemoryType::Episodic, 0.6),
            vec![0.9, 0.1, 0.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Search with min_importance filter
    let filter = SanctumFilter::new().min_importance(0.5);
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert!(result.entry.memory.importance >= 0.5);
    }
}

#[tokio::test]
async fn test_delete() {
    let sanctum = InMemorySanctum::new(100);

    let memory = create_test_memory("paladin-1", "Test content", MemoryType::Episodic, 0.8);
    let entry = create_test_entry(memory.clone(), vec![1.0, 0.0, 0.0]);

    // Store the entry
    sanctum.store(entry).await.expect("Failed to store entry");

    // Verify it's stored
    assert_eq!(sanctum.count(None).await.unwrap(), 1);

    // Delete the entry
    let deleted = sanctum
        .delete(&memory.id.to_string())
        .await
        .expect("Delete failed");
    assert!(deleted);

    // Verify it's deleted
    assert_eq!(sanctum.count(None).await.unwrap(), 0);

    // Deleting again should return false
    let deleted_again = sanctum
        .delete(&memory.id.to_string())
        .await
        .expect("Delete failed");
    assert!(!deleted_again);
}

#[tokio::test]
async fn test_update() {
    let sanctum = InMemorySanctum::new(100);

    let memory = create_test_memory("paladin-1", "Original content", MemoryType::Episodic, 0.8);
    let entry = create_test_entry(memory.clone(), vec![1.0, 0.0, 0.0]);

    // Store the entry
    sanctum.store(entry).await.expect("Failed to store entry");

    // Update the memory
    let updated_memory =
        create_test_memory("paladin-1", "Updated content", MemoryType::Episodic, 0.9);
    let updated_entry = SanctumEntry::new(
        Memory {
            id: memory.id,
            ..updated_memory
        },
        vec![0.0, 1.0, 0.0],
    )
    .expect("Failed to create updated entry");

    sanctum
        .update(updated_entry.clone())
        .await
        .expect("Update failed");

    // Search to verify update
    let query = SanctumQuery::new(vec![0.0, 1.0, 0.0], 1);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.content, "Updated content");
    assert_eq!(results[0].entry.memory.importance, 0.9);
}

#[tokio::test]
async fn test_update_nonexistent() {
    let sanctum = InMemorySanctum::new(100);

    let memory = create_test_memory("paladin-1", "Test content", MemoryType::Episodic, 0.8);
    let entry = create_test_entry(memory, vec![1.0, 0.0, 0.0]);

    // Try to update a non-existent entry
    let result = sanctum.update(entry).await;

    assert!(result.is_err());
    match result {
        Err(SanctumError::NotFound(_)) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_count_with_filter() {
    let sanctum = InMemorySanctum::new(100);

    let entries = vec![
        create_test_entry(
            create_test_memory("paladin-1", "Content 1", MemoryType::Episodic, 0.8),
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-2", "Content 2", MemoryType::Episodic, 0.7),
            vec![0.0, 1.0, 0.0],
        ),
        create_test_entry(
            create_test_memory("paladin-1", "Content 3", MemoryType::Semantic, 0.9),
            vec![0.0, 0.0, 1.0],
        ),
    ];

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Count all
    assert_eq!(sanctum.count(None).await.unwrap(), 3);

    // Count with paladin_id filter
    let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
    assert_eq!(sanctum.count(Some(filter)).await.unwrap(), 2);

    // Count with memory_type filter
    let filter = SanctumFilter::new().memory_type(MemoryType::Episodic);
    assert_eq!(sanctum.count(Some(filter)).await.unwrap(), 2);
}

#[tokio::test]
async fn test_lru_eviction() {
    // Create sanctum with capacity of 3
    let sanctum = InMemorySanctum::new(3);

    // Store 3 entries (fill capacity)
    let entries: Vec<_> = (0..3)
        .map(|i| {
            create_test_entry(
                create_test_memory(
                    "paladin-1",
                    &format!("Content {}", i),
                    MemoryType::Episodic,
                    0.5,
                ),
                vec![i as f32, 0.0, 0.0],
            )
        })
        .collect();

    let first_id = entries[0].memory.id.clone();

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    assert_eq!(sanctum.count(None).await.unwrap(), 3);

    // Store a 4th entry, should evict the oldest (first) entry
    let new_entry = create_test_entry(
        create_test_memory("paladin-1", "Content 3", MemoryType::Episodic, 0.5),
        vec![3.0, 0.0, 0.0],
    );

    sanctum.store(new_entry).await.expect("Failed to store");

    // Count should still be 3 (capacity)
    assert_eq!(sanctum.count(None).await.unwrap(), 3);

    // First entry should be evicted
    let query = SanctumQuery::new(vec![0.0, 0.0, 0.0], 10);
    let results = sanctum.search(query).await.expect("Search failed");

    let ids: Vec<_> = results.iter().map(|r| &r.entry.memory.id).collect();
    assert!(!ids.contains(&&first_id));
}

#[tokio::test]
async fn test_thread_safety() {
    let sanctum = Arc::new(InMemorySanctum::new(100));

    // Spawn multiple tasks that store entries concurrently
    let mut handles = vec![];
    for i in 0..10 {
        let sanctum = Arc::clone(&sanctum);
        let handle = task::spawn(async move {
            let entry = create_test_entry(
                create_test_memory(
                    &format!("paladin-{}", i),
                    &format!("Content {}", i),
                    MemoryType::Episodic,
                    0.5,
                ),
                vec![i as f32, 0.0, 0.0],
            );

            sanctum.store(entry).await.expect("Failed to store");
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task failed");
    }

    // Verify all entries were stored
    assert_eq!(sanctum.count(None).await.unwrap(), 10);
}

#[tokio::test]
async fn test_concurrent_reads_and_writes() {
    let sanctum = Arc::new(InMemorySanctum::new(100));

    // Pre-populate with some entries
    let entries: Vec<_> = (0..5)
        .map(|i| {
            create_test_entry(
                create_test_memory(
                    "paladin-1",
                    &format!("Content {}", i),
                    MemoryType::Episodic,
                    0.5,
                ),
                vec![i as f32 / 10.0, 0.5, 0.5],
            )
        })
        .collect();

    sanctum
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Spawn tasks that read and write concurrently
    let mut handles = vec![];

    // 5 read tasks
    for _ in 0..5 {
        let sanctum = Arc::clone(&sanctum);
        let handle = task::spawn(async move {
            let query = SanctumQuery::new(vec![0.5, 0.5, 0.5], 10);
            sanctum.search(query).await.expect("Search failed");
        });
        handles.push(handle);
    }

    // 5 write tasks
    for i in 5..10 {
        let sanctum = Arc::clone(&sanctum);
        let handle = task::spawn(async move {
            let entry = create_test_entry(
                create_test_memory(
                    &format!("paladin-{}", i),
                    &format!("Content {}", i),
                    MemoryType::Episodic,
                    0.5,
                ),
                vec![i as f32 / 10.0, 0.5, 0.5],
            );
            sanctum.store(entry).await.expect("Failed to store");
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task failed");
    }

    // Verify final count
    assert_eq!(sanctum.count(None).await.unwrap(), 10);
}

#[tokio::test]
async fn test_empty_search() {
    let sanctum = InMemorySanctum::new(100);

    // Search in empty sanctum
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10);
    let results = sanctum.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_dimension_mismatch() {
    let sanctum = InMemorySanctum::new(100);

    // Store entry with 3-dimensional embedding
    let entry = create_test_entry(
        create_test_memory("paladin-1", "Content", MemoryType::Episodic, 0.8),
        vec![1.0, 0.0, 0.0],
    );

    sanctum.store(entry).await.expect("Failed to store");

    // Search with different dimension should return error
    let query = SanctumQuery::new(vec![1.0, 0.0], 10); // 2D instead of 3D
    let result = sanctum.search(query).await;

    assert!(result.is_err());
    match result {
        Err(SanctumError::InvalidDimension(_)) => {}
        _ => panic!("Expected InvalidDimension error"),
    }
}
