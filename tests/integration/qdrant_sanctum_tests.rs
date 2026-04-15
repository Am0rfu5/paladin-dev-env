//! Integration tests for QdrantSanctumAdapter
//!
//! Tests the Qdrant vector database implementation of SanctumPort trait.
//! Requires Qdrant running via Docker Compose on localhost:6334
//!
//! Start Qdrant: docker-compose -f docker/docker-compose.yml up -d qdrant --profile test

use paladin::application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery,
};
use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin::infrastructure::adapters::sanctum::QdrantSanctumAdapter;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

/// Helper to create test collection name
fn test_collection_name() -> String {
    format!("test_collection_{}", Uuid::new_v4().as_simple())
}

/// Helper to create a test memory entry
fn create_test_entry(
    paladin_id: &str,
    content: &str,
    memory_type: MemoryType,
    importance: f32,
    embedding: Vec<f32>,
) -> SanctumEntry {
    let memory = MemoryBuilder::new(paladin_id.to_string(), content.to_string())
        .memory_type(memory_type)
        .importance(importance)
        .build()
        .expect("Failed to build test memory");

    SanctumEntry::new(memory, embedding).expect("Failed to create test entry")
}

/// Create a normalized random embedding vector
fn random_embedding(dimensions: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let vec: Vec<f32> = (0..dimensions).map(|_| rng.gen_range(-1.0..1.0)).collect();

    // Normalize the vector
    let magnitude: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    vec.iter().map(|x| x / magnitude).collect()
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_qdrant_connection() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 1536)
        .await
        .expect("Failed to create Qdrant adapter");

    // Test connection by counting (should be 0 for new collection)
    let count = adapter.count(None).await.expect("Failed to count");
    assert_eq!(count, 0);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_store_and_retrieve() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entry = create_test_entry(
        "paladin-1",
        "Test content",
        MemoryType::Episodic,
        0.8,
        vec![1.0, 0.0, 0.0],
    );

    // Store the entry
    adapter
        .store(entry.clone())
        .await
        .expect("Failed to store entry");

    // Give Qdrant time to index
    sleep(Duration::from_millis(100)).await;

    // Search with exact embedding should return the stored entry
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 1);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.content, "Test content");
    assert!(results[0].score > 0.99); // Should be near-perfect match
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_store_batch() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "Content 1",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Content 2",
            MemoryType::Semantic,
            0.7,
            vec![0.0, 1.0, 0.0],
        ),
        create_test_entry(
            "paladin-2",
            "Content 3",
            MemoryType::Procedural,
            0.9,
            vec![0.0, 0.0, 1.0],
        ),
    ];

    // Store batch
    adapter
        .store_batch(entries.clone())
        .await
        .expect("Failed to store batch");

    // Give Qdrant time to index
    sleep(Duration::from_millis(200)).await;

    // Count should be 3
    let count = adapter.count(None).await.expect("Failed to count");
    assert_eq!(count, 3);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_semantic_search() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    // Store entries with different embeddings
    let entries = vec![
        create_test_entry(
            "paladin-1",
            "Very similar",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.1, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Somewhat similar",
            MemoryType::Episodic,
            0.7,
            vec![0.7, 0.5, 0.2],
        ),
        create_test_entry(
            "paladin-1",
            "Not similar",
            MemoryType::Episodic,
            0.6,
            vec![0.0, 0.0, 1.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    // Give Qdrant time to index
    sleep(Duration::from_millis(200)).await;

    // Search with query similar to first entry
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 3);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 3);

    // Results should be sorted by similarity
    assert!(results[0].score >= results[1].score);
    assert!(results[1].score >= results[2].score);

    // First result should be "Very similar"
    assert_eq!(results[0].entry.memory.content, "Very similar");
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_search_with_top_k() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    // Store 5 entries
    let entries: Vec<_> = (0..5)
        .map(|i| {
            create_test_entry(
                "paladin-1",
                &format!("Content {}", i),
                MemoryType::Episodic,
                0.5,
                vec![i as f32 / 10.0, 0.5, 0.5],
            )
        })
        .collect();

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Search with top_k=2
    let query = SanctumQuery::new(vec![0.0, 0.5, 0.5], 2);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_search_with_min_score() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "High similarity",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Low similarity",
            MemoryType::Episodic,
            0.7,
            vec![0.0, 1.0, 0.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Search with high min_score to filter out low similarity results
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).min_score(0.9);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.content, "High similarity");
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_search_with_paladin_filter() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "Content 1",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-2",
            "Content 2",
            MemoryType::Episodic,
            0.7,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Content 3",
            MemoryType::Episodic,
            0.9,
            vec![0.9, 0.1, 0.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Search with paladin_id filter
    let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert_eq!(result.entry.memory.paladin_id, "paladin-1");
    }
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_search_with_memory_type_filter() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "Episodic 1",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Semantic 1",
            MemoryType::Semantic,
            0.7,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Episodic 2",
            MemoryType::Episodic,
            0.9,
            vec![0.9, 0.1, 0.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Search with memory_type filter
    let filter = SanctumFilter::new().memory_type(MemoryType::Episodic);
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert_eq!(result.entry.memory.memory_type, MemoryType::Episodic);
    }
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_search_with_importance_filter() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "High importance",
            MemoryType::Episodic,
            0.9,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Low importance",
            MemoryType::Episodic,
            0.3,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Medium importance",
            MemoryType::Episodic,
            0.6,
            vec![0.9, 0.1, 0.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Search with min_importance filter
    let filter = SanctumFilter::new().min_importance(0.5);
    let query = SanctumQuery::new(vec![1.0, 0.0, 0.0], 10).filter(filter);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 2);
    for result in results {
        assert!(result.entry.memory.importance >= 0.5);
    }
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_delete() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entry = create_test_entry(
        "paladin-1",
        "Test content",
        MemoryType::Episodic,
        0.8,
        vec![1.0, 0.0, 0.0],
    );

    let memory_id = entry.memory.id;

    // Store the entry
    adapter.store(entry).await.expect("Failed to store entry");

    sleep(Duration::from_millis(100)).await;

    // Verify it's stored
    assert_eq!(adapter.count(None).await.unwrap(), 1);

    // Delete the entry
    let deleted = adapter
        .delete(&memory_id.to_string())
        .await
        .expect("Delete failed");
    assert!(deleted);

    sleep(Duration::from_millis(100)).await;

    // Verify it's deleted
    assert_eq!(adapter.count(None).await.unwrap(), 0);

    // Deleting again should return false
    let deleted_again = adapter
        .delete(&memory_id.to_string())
        .await
        .expect("Delete failed");
    assert!(!deleted_again);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_update() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entry = create_test_entry(
        "paladin-1",
        "Original content",
        MemoryType::Episodic,
        0.8,
        vec![1.0, 0.0, 0.0],
    );

    let memory_id = entry.memory.id;

    // Store the entry
    adapter.store(entry).await.expect("Failed to store entry");

    sleep(Duration::from_millis(100)).await;

    // Update the entry
    let updated_memory = MemoryBuilder::new("paladin-1".to_string(), "Updated content".to_string())
        .memory_type(MemoryType::Episodic)
        .importance(0.9)
        .build()
        .expect("Failed to build memory");

    let updated_entry = SanctumEntry::new(
        paladin::core::platform::container::sanctum::Memory {
            id: memory_id,
            ..updated_memory
        },
        vec![0.0, 1.0, 0.0],
    )
    .expect("Failed to create updated entry");

    adapter
        .update(updated_entry.clone())
        .await
        .expect("Update failed");

    sleep(Duration::from_millis(100)).await;

    // Search to verify update
    let query = SanctumQuery::new(vec![0.0, 1.0, 0.0], 1);
    let results = adapter.search(query).await.expect("Search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory.content, "Updated content");
    assert_eq!(results[0].entry.memory.importance, 0.9);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_count_with_filter() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    let entries = vec![
        create_test_entry(
            "paladin-1",
            "Content 1",
            MemoryType::Episodic,
            0.8,
            vec![1.0, 0.0, 0.0],
        ),
        create_test_entry(
            "paladin-2",
            "Content 2",
            MemoryType::Episodic,
            0.7,
            vec![0.0, 1.0, 0.0],
        ),
        create_test_entry(
            "paladin-1",
            "Content 3",
            MemoryType::Semantic,
            0.9,
            vec![0.0, 0.0, 1.0],
        ),
    ];

    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");

    sleep(Duration::from_millis(200)).await;

    // Count all
    assert_eq!(adapter.count(None).await.unwrap(), 3);

    // Count with paladin_id filter
    let filter = SanctumFilter::new().paladin_id("paladin-1".to_string());
    assert_eq!(adapter.count(Some(filter)).await.unwrap(), 2);

    // Count with memory_type filter
    let filter = SanctumFilter::new().memory_type(MemoryType::Episodic);
    assert_eq!(adapter.count(Some(filter)).await.unwrap(), 2);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_large_batch_performance() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 1536)
        .await
        .expect("Failed to create Qdrant adapter");

    // Create 1000 entries with realistic embeddings
    let entries: Vec<_> = (0..1000)
        .map(|i| {
            create_test_entry(
                &format!("paladin-{}", i % 10),
                &format!("Content {}", i),
                MemoryType::Episodic,
                0.5 + (i % 50) as f32 / 100.0,
                random_embedding(1536),
            )
        })
        .collect();

    let start = std::time::Instant::now();
    adapter
        .store_batch(entries)
        .await
        .expect("Failed to store batch");
    let duration = start.elapsed();

    println!("Stored 1000 entries in {:?}", duration);

    sleep(Duration::from_millis(500)).await;

    // Verify count
    let count = adapter.count(None).await.expect("Failed to count");
    assert_eq!(count, 1000);

    // Test search performance
    let query_vector = random_embedding(1536);
    let start = std::time::Instant::now();
    let results = adapter
        .search(SanctumQuery::new(query_vector, 10))
        .await
        .expect("Search failed");
    let search_duration = start.elapsed();

    println!("Searched 1000 entries in {:?}", search_duration);
    assert_eq!(results.len(), 10);

    // Verify performance target: < 500ms for search
    assert!(
        search_duration.as_millis() < 500,
        "Search took {}ms, expected < 500ms",
        search_duration.as_millis()
    );
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_collection_auto_creation() {
    let collection = test_collection_name();

    // Create adapter - should auto-create collection
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 128)
        .await
        .expect("Failed to create Qdrant adapter");

    // Should be able to store immediately
    let entry = create_test_entry(
        "paladin-1",
        "Test content",
        MemoryType::Episodic,
        0.8,
        random_embedding(128),
    );

    adapter.store(entry).await.expect("Failed to store entry");

    sleep(Duration::from_millis(100)).await;

    assert_eq!(adapter.count(None).await.unwrap(), 1);
}

#[tokio::test]
#[ignore = "Requires Qdrant running on localhost:6334"]
async fn test_dimension_mismatch() {
    let collection = test_collection_name();
    let adapter = QdrantSanctumAdapter::new("http://localhost:6334", &collection, 3)
        .await
        .expect("Failed to create Qdrant adapter");

    // Store entry with 3-dimensional embedding
    let entry = create_test_entry(
        "paladin-1",
        "Content",
        MemoryType::Episodic,
        0.8,
        vec![1.0, 0.0, 0.0],
    );

    adapter.store(entry).await.expect("Failed to store");

    sleep(Duration::from_millis(100)).await;

    // Search with different dimension should return error
    let query = SanctumQuery::new(vec![1.0, 0.0], 10); // 2D instead of 3D
    let result = adapter.search(query).await;

    assert!(result.is_err());
    match result {
        Err(SanctumError::InvalidDimension(_)) => {}
        _ => panic!("Expected InvalidDimension error"),
    }
}
