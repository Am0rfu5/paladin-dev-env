//! In-Memory Garrison Example
//!
//! Demonstrates basic usage of the in-memory Garrison for maintaining
//! conversation context within a single session.
//!
//! Run with: `cargo run --example garrison_in_memory`

use paladin::application::ports::output::garrison_port::GarrisonPort;
use paladin::core::platform::container::garrison::{
    ConversationRole, EvictionStrategy, GarrisonConfig, GarrisonEntry,
};
use paladin::infrastructure::adapters::garrison::InMemoryGarrison;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== In-Memory Garrison Example ===\n");

    // Configure the garrison with a small window for demonstration
    let config = GarrisonConfig::new(5, Some(1000))
        .with_eviction_strategy(EvictionStrategy::SlidingWindow)
        .with_preserve_recent(3);

    println!("Configuration:");
    println!("  Max entries: {}", config.max_entries);
    println!("  Max tokens: {:?}", config.max_tokens);
    println!("  Eviction: {:?}", config.eviction_strategy);
    println!("  Preserve recent: {}\n", config.preserve_recent_count);

    // Create the garrison
    let garrison = InMemoryGarrison::new(config);

    // Simulate a conversation
    println!("--- Simulating Conversation ---\n");

    // System prompt
    let system_entry = GarrisonEntry::new(
        ConversationRole::System,
        "You are a helpful AI assistant specialized in Rust programming.".to_string(),
    );
    garrison.remember(system_entry.clone()).await?;
    println!("✓ Stored system prompt");

    // User messages and assistant responses
    let conversations = vec![
        (
            "What is the difference between String and &str in Rust?",
            "String is an owned, heap-allocated string type, while &str is a string slice that borrows data.",
        ),
        (
            "When should I use each one?",
            "Use String when you need owned data or to build strings dynamically. Use &str for function parameters when you don't need ownership.",
        ),
        (
            "Can you show me an example?",
            "Sure! fn greet(name: &str) { println!(\"Hello, {}\", name); } - here &str is perfect because we only need to read the name.",
        ),
        (
            "What about lifetime annotations?",
            "Lifetime annotations tell the compiler how long references are valid. They're often inferred, but explicit when needed.",
        ),
        (
            "Thanks for the explanation!",
            "You're welcome! Feel free to ask if you have more Rust questions.",
        ),
    ];

    for (i, (user_msg, assistant_msg)) in conversations.iter().enumerate() {
        let user_entry = GarrisonEntry::new(ConversationRole::User, user_msg.to_string());
        garrison.remember(user_entry).await?;

        let assistant_entry =
            GarrisonEntry::new(ConversationRole::Assistant, assistant_msg.to_string());
        garrison.remember(assistant_entry).await?;

        println!("✓ Turn {} stored", i + 1);
    }

    // Check statistics
    println!("\n--- Garrison Statistics ---\n");
    let stats = garrison.stats().await?;
    println!("  Entries stored: {}", stats.entry_count);
    println!("  Total tokens: {}", stats.total_tokens);
    println!("  Size: {:?} bytes\n", stats.size_bytes.unwrap_or_default());

    // Recall recent conversation
    println!("--- Recalling Recent Context (last 5 entries) ---\n");
    let recent = garrison.recall_recent(5).await?;
    for (i, entry) in recent.iter().enumerate() {
        println!(
            "{}: [{:?}] {}",
            i + 1,
            entry.role,
            truncate(&entry.content, 60)
        );
    }

    // Search functionality
    println!("\n--- Searching for 'String' ---\n");
    let results = garrison.search("String", 10).await?;
    println!("Found {} matching entries:", results.len());
    for (i, entry) in results.iter().enumerate() {
        println!("  {}: {}", i + 1, truncate(&entry.content, 70));
    }

    // Clear all history
    println!("\n--- Clearing Garrison ---\n");
    garrison.forget_all().await?;
    let stats_after = garrison.stats().await?;
    println!("✓ All entries cleared");
    println!("  Entries remaining: {}", stats_after.entry_count);

    println!("\n=== Example Complete ===");
    Ok(())
}

/// Helper function to truncate long strings for display
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
