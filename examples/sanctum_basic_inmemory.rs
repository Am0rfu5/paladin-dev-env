//! Basic Sanctum InMemory Example
//!
//! Demonstrates basic usage of Sanctum with the InMemory adapter:
//! - Creating memories with metadata
//! - Storing single and batch entries
//! - Semantic search with scoring
//! - Filtering by memory type and importance
//! - Updating and deleting memories
//!
//! Run with: cargo run --example sanctum_basic_inmemory

use paladin::application::ports::output::sanctum_port::{SanctumFilter, SanctumPort, SanctumQuery};
use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sanctum InMemory Basic Example ===\n");

    // Step 1: Create InMemory adapter (no configuration needed)
    println!("1. Creating InMemory Sanctum adapter...");
    let sanctum = InMemorySanctum::new(10000); // Max 10,000 entries
    println!("   ✓ Adapter ready (max 10,000 entries)\n");

    // Step 2: Store a single memory
    println!("2. Storing a single memory...");
    let mut metadata1 = std::collections::HashMap::new();
    metadata1.insert("topic".to_string(), json!("programming"));
    metadata1.insert("language".to_string(), json!("rust"));

    let memory1 = MemoryBuilder::new(
        "paladin-123".to_string(),
        "User asked about Rust programming language features".to_string(),
    )
    .memory_type(MemoryType::Episodic)
    .importance(0.8)
    .metadata(metadata1)
    .build()?;

    // Create embedding (dummy 384-dimensional vector for demo)
    let embedding1 = vec![0.1; 384];
    let entry1 = SanctumEntry::new(memory1, embedding1)?;

    sanctum.store(entry1).await?;
    println!("   ✓ Memory stored with ID and embedding\n");

    // Step 3: Store a batch of memories
    println!("3. Storing batch of memories...");
    let batch_entries: Vec<SanctumEntry> = vec![
        {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("topic".to_string(), json!("programming"));
            SanctumEntry::new(
                MemoryBuilder::new(
                    "paladin-123".to_string(),
                    "Rust has ownership system for memory safety".to_string(),
                )
                .memory_type(MemoryType::Semantic)
                .importance(0.9)
                .metadata(metadata)
                .build()?,
                vec![0.2; 384],
            )?
        },
        {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("topic".to_string(), json!("learning"));
            SanctumEntry::new(
                MemoryBuilder::new(
                    "paladin-123".to_string(),
                    "User completed Rust tutorial on borrowing".to_string(),
                )
                .memory_type(MemoryType::Procedural)
                .importance(0.7)
                .metadata(metadata)
                .build()?,
                vec![0.15; 384],
            )?
        },
        {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("topic".to_string(), json!("programming"));
            metadata.insert("language".to_string(), json!("python"));
            SanctumEntry::new(
                MemoryBuilder::new(
                    "paladin-456".to_string(),
                    "Different paladin asked about Python".to_string(),
                )
                .memory_type(MemoryType::Episodic)
                .importance(0.6)
                .metadata(metadata)
                .build()?,
                vec![0.5; 384],
            )?
        },
    ];

    sanctum.store_batch(batch_entries).await?;
    println!("   ✓ Batch of 3 memories stored\n");

    // Step 4: Count total memories
    let total_count = sanctum.count(None).await?;
    println!("4. Total memories stored: {}\n", total_count);

    // Step 5: Semantic search
    println!("5. Performing semantic search...");
    let query_embedding = vec![0.12; 384]; // Similar to first entries
    let query = SanctumQuery::new(query_embedding, 3).min_score(0.5);

    let results = sanctum.search(query).await?;
    println!("   Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   {}. [Score: {:.3}] {}",
            i + 1,
            result.score,
            result.entry.memory.content
        );
        println!(
            "      Type: {:?}, Importance: {:.2}",
            result.entry.memory.memory_type, result.entry.memory.importance
        );
    }
    println!();

    // Step 6: Filtered search (only paladin-123's episodic memories)
    println!("6. Filtered search (paladin-123, episodic only)...");
    let filter = SanctumFilter::new()
        .paladin_id("paladin-123".to_string())
        .memory_type(MemoryType::Episodic);

    let filtered_query = SanctumQuery::new(vec![0.1; 384], 5).filter(filter);
    let filtered_results = sanctum.search(filtered_query).await?;

    println!(
        "   Found {} episodic memories for paladin-123:",
        filtered_results.len()
    );
    for result in &filtered_results {
        println!("   - {}", result.entry.memory.content);
    }
    println!();

    // Step 7: Filter by importance
    println!("7. Searching high-importance memories (>= 0.8)...");
    let importance_filter = SanctumFilter::new().min_importance(0.8);
    let importance_count = sanctum.count(Some(importance_filter)).await?;
    println!("   Found {} high-importance memories\n", importance_count);

    // Step 8: Update a memory
    println!("8. Updating a memory...");
    if let Some(first_result) = results.first() {
        let mut updated_memory = first_result.entry.memory.clone();
        updated_memory.importance = 0.95; // Increase importance
        updated_memory.access_count += 1;

        let updated_entry =
            SanctumEntry::new(updated_memory, first_result.entry.embedding.clone())?;
        sanctum.update(updated_entry).await?;
        println!("   ✓ Memory importance updated to 0.95\n");
    }

    // Step 9: Delete a memory
    println!("9. Deleting a memory...");
    if let Some(last_result) = results.last() {
        let deleted = sanctum
            .delete(&last_result.entry.memory.id.to_string())
            .await?;
        if deleted {
            println!("   ✓ Memory deleted successfully");
        }
    }

    let final_count = sanctum.count(None).await?;
    println!("   Final count: {} memories\n", final_count);

    // Step 10: Count by paladin
    println!("10. Memories per paladin:");
    let paladin_123_count = sanctum
        .count(Some(
            SanctumFilter::new().paladin_id("paladin-123".to_string()),
        ))
        .await?;
    let paladin_456_count = sanctum
        .count(Some(
            SanctumFilter::new().paladin_id("paladin-456".to_string()),
        ))
        .await?;

    println!("    paladin-123: {} memories", paladin_123_count);
    println!("    paladin-456: {} memories", paladin_456_count);

    println!("\n=== Example Complete ===");
    println!("Key Takeaways:");
    println!("  • InMemory adapter requires zero configuration");
    println!("  • Fast operations (<1ms for small datasets)");
    println!("  • Perfect for development and testing");
    println!("  • Data is ephemeral (lost on restart)");
    println!("  • Switch to Qdrant adapter for production");

    Ok(())
}
