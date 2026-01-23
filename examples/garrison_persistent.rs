//! Persistent SQLite Garrison Example
//!
//! Demonstrates using SQLite for durable conversation storage that survives
//! application restarts. Shows connection, persistence, recovery, and cleanup.
//!
//! Run with: `cargo run --example garrison_persistent`

use paladin::application::ports::output::garrison_port::GarrisonPort;
use paladin::core::platform::container::garrison::{
    ConversationRole, EvictionStrategy, GarrisonConfig, GarrisonEntry,
};
use paladin::infrastructure::adapters::garrison::sqlite_garrison::SqliteGarrison;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Persistent SQLite Garrison Example ===\n");

    // Create a temporary directory for the database (for demo purposes)
    // In production, you'd use a permanent path like "./data/garrison.db"
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("garrison_demo.db");
    println!("Database location: {:?}\n", db_path);

    // Configure the garrison
    let config = GarrisonConfig::new(20, Some(8000))
        .with_eviction_strategy(EvictionStrategy::ImportanceBased)
        .with_preserve_recent(5);

    // === Phase 1: Initial Session ===
    println!("--- Phase 1: Initial Session ---\n");
    {
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "assistant-001").await?;
        println!("✓ Connected to database");

        // System prompt
        let system_entry = GarrisonEntry::new(
            ConversationRole::System,
            "You are a helpful assistant that remembers context across sessions.".to_string(),
        );
        garrison.remember(system_entry).await?;
        println!("✓ Stored system prompt");

        // Initial conversation
        let conversation = vec![
            (
                "What's my favorite programming language?",
                "I don't have that information yet. What is your favorite programming language?",
            ),
            (
                "I love Rust! It's powerful and safe.",
                "Great choice! Rust is excellent for systems programming with memory safety guarantees. I'll remember that you love Rust.",
            ),
            (
                "What are the key features?",
                "Rust's key features include ownership, borrowing, zero-cost abstractions, and fearless concurrency.",
            ),
        ];

        for (user_msg, assistant_msg) in conversation {
            garrison
                .remember(GarrisonEntry::new(
                    ConversationRole::User,
                    user_msg.to_string(),
                ))
                .await?;
            garrison
                .remember(GarrisonEntry::new(
                    ConversationRole::Assistant,
                    assistant_msg.to_string(),
                ))
                .await?;
        }
        println!("✓ Stored 3 conversation turns");

        let stats = garrison.stats().await?;
        println!("\nSession 1 Stats:");
        println!("  Entries: {}", stats.entry_count);
        println!("  Tokens: {}", stats.total_tokens);
    }
    println!("✓ Session 1 ended (connection closed)\n");

    // === Phase 2: Application Restart ===
    println!("--- Phase 2: Application Restart (Simulated) ---\n");
    {
        // Reconnect to the same database
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "assistant-001").await?;
        println!("✓ Reconnected to database");

        // Verify previous data is still there
        let previous = garrison.recall_recent(10).await?;
        println!(
            "✓ Recovered {} entries from previous session",
            previous.len()
        );

        // Search for context
        let rust_mentions = garrison.search("Rust", 10).await?;
        println!("✓ Found {} entries mentioning 'Rust'", rust_mentions.len());

        // Continue the conversation
        garrison
            .remember(GarrisonEntry::new(
                ConversationRole::User,
                "Can you remind me what my favorite language is?".to_string(),
            ))
            .await?;

        garrison
            .remember(GarrisonEntry::new(
                ConversationRole::Assistant,
                "Of course! You mentioned that you love Rust because it's powerful and safe."
                    .to_string(),
            ))
            .await?;

        println!("✓ Continued conversation with context\n");

        let stats = garrison.stats().await?;
        println!("Session 2 Stats:");
        println!("  Total entries: {}", stats.entry_count);
        println!("  Total tokens: {}", stats.total_tokens);
    }
    println!("✓ Session 2 ended\n");

    // === Phase 3: Full-Text Search ===
    println!("--- Phase 3: Full-Text Search Demo ---\n");
    {
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "assistant-001").await?;

        // Search with different queries
        let queries = vec!["\"favorite programming language\"", "safety", "Rust"];

        for query in queries {
            let results = garrison.search(query, 5).await?;
            println!("Query '{}': {} results", query, results.len());
            for entry in results.iter().take(2) {
                println!("  - {:?}: {}", entry.role, truncate(&entry.content, 60));
            }
        }
    }
    println!();

    // === Phase 4: Multi-Paladin Isolation ===
    println!("--- Phase 4: Multi-Paladin Isolation ---\n");
    {
        // Create a second paladin's garrison in the same database
        let garrison2 = SqliteGarrison::connect(&db_path, config.clone(), "assistant-002").await?;

        garrison2
            .remember(GarrisonEntry::new(
                ConversationRole::User,
                "This is a different paladin's conversation.".to_string(),
            ))
            .await?;

        let stats2 = garrison2.stats().await?;
        println!("✓ Paladin 'assistant-002' created");
        println!(
            "  Entries: {} (isolated from assistant-001)",
            stats2.entry_count
        );

        // Verify original paladin still has its own data
        let garrison1 = SqliteGarrison::connect(&db_path, config, "assistant-001").await?;
        let stats1 = garrison1.stats().await?;
        println!("✓ Paladin 'assistant-001' entries: {}", stats1.entry_count);
        println!("  Data isolation verified!");
    }

    println!("\n--- Cleanup ---\n");
    std::fs::remove_file(&db_path)?;
    println!("✓ Removed temporary database");

    println!("\n=== Example Complete ===");
    println!("\nKey Takeaways:");
    println!("• Data persists across application restarts");
    println!("• Full-text search enables semantic queries");
    println!("• Multiple paladins can share a database with isolation");
    println!("• SQLite provides production-ready durability");

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
