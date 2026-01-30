//! Paladin with Sanctum Long-term Memory Example
//!
//! Demonstrates integrating Sanctum long-term memory with Paladin agents:
//! - Storing conversation history as memories
//! - Retrieving relevant context for agent execution
//! - Building agent knowledge base over time
//! - Using memories to improve responses
//!
//! This combines Garrison (short-term) with Sanctum (long-term) memory:
//! - Garrison: Recent conversation context (sliding window)
//! - Sanctum: Persistent memories with semantic search
//!
//! Run with: cargo run --example paladin_with_sanctum

use paladin::application::ports::output::sanctum_port::{SanctumFilter, SanctumPort, SanctumQuery};
use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Paladin with Sanctum Long-term Memory ===\n");

    // Setup: Create Sanctum adapter
    println!("1. Setting up Sanctum long-term memory...");
    let sanctum: Arc<dyn SanctumPort> = Arc::new(InMemorySanctum::new(10000));
    let paladin_id = "learning-assistant-001";
    println!("   ✓ InMemory Sanctum ready for paladin: {}\n", paladin_id);

    // Scenario: Teaching Assistant Paladin
    println!("2. Scenario: Teaching Assistant Building Knowledge Base");
    println!("   ========================================================\n");

    // Initial interactions - building the knowledge base
    println!("   First Session: Learning about Rust ownership");
    println!("   ---------------------------------------------\n");

    let session1_interactions = vec![
        (
            "What is ownership in Rust?",
            "Ownership is Rust's key feature for memory safety without garbage collection. Each value has a single owner.",
            MemoryType::Episodic,
            0.9,
        ),
        (
            "Rust ownership ensures memory safety",
            "Core fact: Ownership rules are enforced at compile time, preventing data races.",
            MemoryType::Semantic,
            1.0,
        ),
        (
            "How to transfer ownership?",
            "Use move semantics - when assigning, the value moves unless it implements Copy trait.",
            MemoryType::Procedural,
            0.8,
        ),
    ];

    // Store memories from first session
    for (user_query, agent_response, mem_type, importance) in session1_interactions {
        // Store the user query as episodic memory
        let query_memory = MemoryBuilder::new(
            paladin_id.to_string(),
            format!("User asked: {}", user_query),
        )
        .memory_type(MemoryType::Episodic)
        .importance(importance * 0.8) // Queries slightly less important
        .build()?;

        let query_entry = SanctumEntry::new(
            query_memory,
            vec![0.1; 384], // Dummy embedding
        )?;

        sanctum.store(query_entry).await?;

        // Store the agent's knowledge as semantic/procedural memory
        let response_memory =
            MemoryBuilder::new(paladin_id.to_string(), agent_response.to_string())
                .memory_type(mem_type)
                .importance(importance)
                .build()?;

        let response_entry = SanctumEntry::new(
            response_memory,
            vec![0.15; 384], // Dummy embedding
        )?;

        sanctum.store(response_entry).await?;

        println!("   Stored: \"{}\"", user_query);
    }

    let session1_count = sanctum.count(None).await?;
    println!("   ✓ Session 1: {} memories stored\n", session1_count);

    // Second session - different topic
    println!("   Second Session: Learning about async/await");
    println!("   -------------------------------------------\n");

    let session2_interactions = vec![
        (
            "How does async work in Rust?",
            "Async in Rust is zero-cost abstraction for concurrent programming using futures.",
            MemoryType::Episodic,
            0.85,
        ),
        (
            "Tokio is the async runtime for Rust",
            "Tokio provides the scheduler and I/O drivers for async execution.",
            MemoryType::Semantic,
            0.9,
        ),
    ];

    for (user_query, agent_response, mem_type, importance) in session2_interactions {
        let query_memory = MemoryBuilder::new(
            paladin_id.to_string(),
            format!("User asked: {}", user_query),
        )
        .memory_type(MemoryType::Episodic)
        .importance(importance * 0.8)
        .build()?;

        let response_memory =
            MemoryBuilder::new(paladin_id.to_string(), agent_response.to_string())
                .memory_type(mem_type)
                .importance(importance)
                .build()?;

        sanctum
            .store_batch(vec![
                SanctumEntry::new(query_memory, vec![0.5; 384])?,
                SanctumEntry::new(response_memory, vec![0.52; 384])?,
            ])
            .await?;

        println!("   Stored: \"{}\"", user_query);
    }

    let total_count = sanctum.count(None).await?;
    println!("   ✓ Session 2: {} total memories\n", total_count);

    // Agent execution: Using memories to provide context
    println!("3. Agent Execution with Memory Retrieval");
    println!("   ======================================\n");

    println!("   New User Query: \"Tell me about Rust memory management\"\n");

    // Retrieve relevant memories using semantic search
    println!("   Searching long-term memory for relevant context...");
    let query_embedding = vec![0.12; 384]; // Would be real embedding of user query
    let search_query = SanctumQuery::new(query_embedding, 3).min_score(0.5);

    let relevant_memories = sanctum.search(search_query).await?;

    println!("   Found {} relevant memories:\n", relevant_memories.len());

    for (i, result) in relevant_memories.iter().enumerate() {
        println!(
            "   {}. [Relevance: {:.2}] [Type: {:?}]",
            i + 1,
            result.score,
            result.entry.memory.memory_type
        );
        println!("      {}", result.entry.memory.content);
        println!();
    }

    // Build context for agent from memories
    let _memory_context = relevant_memories
        .iter()
        .map(|r| r.entry.memory.content.clone())
        .collect::<Vec<_>>()
        .join("\n");

    println!("   Context built from {} memories", relevant_memories.len());
    println!("   Agent would now use this context to generate informed response\n");

    // Simulate agent using memory context
    let agent_response_with_memory = format!(
        "Based on our previous discussions about ownership and the core principles \
        of Rust's memory management, here's what you need to know:\n\n\
        {} ensures memory safety without garbage collection. This is achieved through \
        the ownership system where each value has a single owner, and {} for safe \
        concurrent programming.",
        relevant_memories
            .get(1)
            .map(|m| &m.entry.memory.content)
            .unwrap_or(&"Rust's ownership system".to_string()),
        relevant_memories
            .get(2)
            .map(|m| &m.entry.memory.content)
            .unwrap_or(&"async/await provides zero-cost abstractions".to_string())
    );

    println!("   Agent Response (enriched with memory):");
    println!("   \"{}\"", agent_response_with_memory);
    println!();

    // 4. Memory Analytics
    println!("4. Memory Analytics");
    println!("   =================\n");

    // Count by memory type
    let episodic = sanctum
        .count(Some(SanctumFilter::new().memory_type(MemoryType::Episodic)))
        .await?;
    let semantic = sanctum
        .count(Some(SanctumFilter::new().memory_type(MemoryType::Semantic)))
        .await?;
    let procedural = sanctum
        .count(Some(
            SanctumFilter::new().memory_type(MemoryType::Procedural),
        ))
        .await?;

    println!("   Memory Distribution:");
    println!("   • Episodic (experiences):  {}", episodic);
    println!("   • Semantic (facts):        {}", semantic);
    println!("   • Procedural (how-tos):    {}", procedural);
    println!("   • Total:                   {}\n", total_count);

    // High-importance memories
    let important = sanctum
        .count(Some(SanctumFilter::new().min_importance(0.9)))
        .await?;
    println!("   High-importance memories (≥0.9): {}\n", important);

    // 5. Memory Evolution Over Time
    println!("5. Memory Evolution");
    println!("   ================\n");

    println!("   As Paladin interacts more:");
    println!("   • Episodic memories accumulate (conversation history)");
    println!("   • Semantic memories grow (knowledge base)");
    println!("   • Procedural memories increase (learned skills)");
    println!("   • Less important memories can be pruned");
    println!("   • Memory importance can be updated based on access\n");

    // Demonstrate importance update
    if let Some(memory_result) = relevant_memories.first() {
        let mut updated_memory = memory_result.entry.memory.clone();
        updated_memory.importance = (updated_memory.importance * 1.1).min(1.0);
        updated_memory.access_count += 1;

        let updated_entry = SanctumEntry::new(
            updated_memory.clone(),
            memory_result.entry.embedding.clone(),
        )?;

        sanctum.update(updated_entry).await?;
        println!(
            "   ✓ Updated frequently accessed memory importance to {:.2}\n",
            updated_memory.importance
        );
    }

    // Summary
    println!("=== Integration Summary ===\n");
    println!("Garrison vs Sanctum:");
    println!("┌──────────────┬─────────────────────────┬──────────────────────┐");
    println!("│ Aspect       │ Garrison (Short-term)   │ Sanctum (Long-term)  │");
    println!("├──────────────┼─────────────────────────┼──────────────────────┤");
    println!("│ Purpose      │ Recent conversation     │ Knowledge base       │");
    println!("│ Duration     │ Session-scoped          │ Persistent           │");
    println!("│ Retrieval    │ Sequential/windowed     │ Semantic search      │");
    println!("│ Size         │ Limited (e.g., 20 msgs) │ Unlimited            │");
    println!("│ Storage      │ In-memory/SQLite        │ Vector database      │");
    println!("└──────────────┴─────────────────────────┴──────────────────────┘\n");

    println!("Use Cases:");
    println!("  • Garrison: \"What did I just say?\" (immediate context)");
    println!("  • Sanctum: \"What do I know about X?\" (knowledge retrieval)\n");

    println!("Best Practices:");
    println!("  1. Store important facts in Sanctum with high importance");
    println!("  2. Use semantic search to retrieve relevant memories");
    println!("  3. Combine Garrison context + Sanctum memories for responses");
    println!("  4. Update memory importance based on access patterns");
    println!("  5. Prune low-importance, rarely accessed memories");
    println!("  6. Use metadata for filtering (topic, date, user, etc.)\n");

    println!("Production Considerations:");
    println!("  • Use Qdrant adapter for persistent Sanctum storage");
    println!("  • Generate real embeddings with embedding service");
    println!("  • Implement memory consolidation (merge similar memories)");
    println!("  • Add memory lifecycle management (TTL, archival)");
    println!("  • Monitor memory growth and search performance");
    println!("  • See docs/SANCTUM.md for detailed guidance");

    Ok(())
}
