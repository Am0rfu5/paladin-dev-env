//! SQLite Garrison Integration Tests
//!
//! These tests verify the SQLite Garrison adapter works correctly with:
//! - Persistence across connection cycles
//! - Full CRUD operations
//! - Migration execution
//! - Connection pooling
//! - Full-text search

use paladin::application::ports::output::garrison_port::GarrisonPort;
use paladin::core::platform::container::garrison::{
    ConversationRole, GarrisonConfig, GarrisonEntry,
};
use paladin::infrastructure::adapters::garrison::sqlite_garrison::SqliteGarrison;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::task::JoinSet;

#[tokio::test]
async fn test_sqlite_garrison_persistence() {
    // Create a temporary database file
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();
    let config = GarrisonConfig::default();

    // First connection - add some entries
    {
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "test-paladin")
            .await
            .unwrap();

        for i in 0..5 {
            let entry =
                GarrisonEntry::new(ConversationRole::User, format!("Persistent message {}", i));
            garrison.remember(entry).await.unwrap();
        }

        let stats = garrison.stats().await.unwrap();
        assert_eq!(stats.entry_count, 5);
    }

    // Second connection - verify data persisted
    {
        let garrison = SqliteGarrison::connect(&db_path, config, "test-paladin")
            .await
            .unwrap();

        let entries = garrison.recall_recent(10).await.unwrap();
        assert_eq!(entries.len(), 5);

        // Verify content
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.content, format!("Persistent message {}", i));
            assert_eq!(entry.role, ConversationRole::User);
        }
    }
}

#[tokio::test]
async fn test_sqlite_garrison_crud_operations() {
    let temp_file = NamedTempFile::new().unwrap();
    let config = GarrisonConfig::default();
    let garrison = SqliteGarrison::connect(temp_file.path(), config, "test-paladin")
        .await
        .unwrap();

    // CREATE
    let entry1 = GarrisonEntry::new(ConversationRole::System, "System prompt".to_string());
    let entry2 = GarrisonEntry::new(ConversationRole::User, "User message".to_string());
    let entry3 = GarrisonEntry::new(
        ConversationRole::Assistant,
        "Assistant response".to_string(),
    );

    garrison.remember(entry1).await.unwrap();
    garrison.remember(entry2).await.unwrap();
    garrison.remember(entry3).await.unwrap();

    // READ
    let entries = garrison.recall_recent(10).await.unwrap();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].role, ConversationRole::System);
    assert_eq!(entries[1].role, ConversationRole::User);
    assert_eq!(entries[2].role, ConversationRole::Assistant);

    // SEARCH
    let search_results = garrison.search("User", 10).await.unwrap();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].content, "User message");

    // STATS
    let stats = garrison.stats().await.unwrap();
    assert_eq!(stats.entry_count, 3);
    assert!(stats.size_bytes.is_some());
    assert!(stats.size_bytes.unwrap() > 0);

    // DELETE
    garrison.forget_all().await.unwrap();
    let entries_after_delete = garrison.recall_recent(10).await.unwrap();
    assert_eq!(entries_after_delete.len(), 0);

    let stats_after_delete = garrison.stats().await.unwrap();
    assert_eq!(stats_after_delete.entry_count, 0);
}

#[tokio::test]
async fn test_sqlite_migration_execution() {
    let temp_file = NamedTempFile::new().unwrap();
    let config = GarrisonConfig::default();

    // First connection should run migrations
    let garrison = SqliteGarrison::connect(temp_file.path(), config.clone(), "test-paladin")
        .await
        .unwrap();

    // Verify we can perform operations (migrations succeeded)
    let entry = GarrisonEntry::new(ConversationRole::User, "Test after migration".to_string());
    garrison.remember(entry).await.unwrap();

    let entries = garrison.recall_recent(1).await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].content, "Test after migration");

    // Second connection should handle existing schema gracefully
    let garrison2 = SqliteGarrison::connect(temp_file.path(), config, "test-paladin")
        .await
        .unwrap();

    let entries2 = garrison2.recall_recent(10).await.unwrap();
    assert_eq!(entries2.len(), 1);
}

#[tokio::test]
async fn test_sqlite_connection_pooling() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();
    let config = GarrisonConfig::default();

    let garrison = Arc::new(
        SqliteGarrison::connect(&db_path, config, "test-paladin")
            .await
            .unwrap(),
    );

    // Spawn multiple concurrent tasks
    let mut join_set = JoinSet::new();

    for i in 0..10 {
        let garrison_clone = garrison.clone();
        join_set.spawn(async move {
            let entry =
                GarrisonEntry::new(ConversationRole::User, format!("Concurrent message {}", i));
            garrison_clone.remember(entry).await.unwrap();
        });
    }

    // Wait for all tasks to complete
    while join_set.join_next().await.is_some() {}

    // Verify all entries were stored
    let entries = garrison.recall_recent(20).await.unwrap();
    assert_eq!(entries.len(), 10);

    // Verify concurrent reads work
    let mut read_set = JoinSet::new();
    for _ in 0..5 {
        let garrison_clone = garrison.clone();
        read_set.spawn(async move {
            let entries = garrison_clone.recall_recent(10).await.unwrap();
            entries.len()
        });
    }

    // All reads should succeed
    while let Some(result) = read_set.join_next().await {
        let count = result.unwrap();
        assert_eq!(count, 10);
    }
}

#[tokio::test]
async fn test_sqlite_full_text_search() {
    let temp_file = NamedTempFile::new().unwrap();
    let config = GarrisonConfig::default();
    let garrison = SqliteGarrison::connect(temp_file.path(), config, "test-paladin")
        .await
        .unwrap();

    // Add diverse content for searching
    let test_entries = vec![
        ("Rust programming language", ConversationRole::User),
        ("Python is great for scripting", ConversationRole::User),
        ("JavaScript runs in browsers", ConversationRole::Assistant),
        (
            "Rust has excellent memory safety",
            ConversationRole::Assistant,
        ),
        ("Python numpy for data science", ConversationRole::User),
    ];

    for (content, role) in test_entries {
        let entry = GarrisonEntry::new(role, content.to_string());
        garrison.remember(entry).await.unwrap();
    }

    // Search for "Rust"
    let rust_results = garrison.search("Rust", 10).await.unwrap();
    assert_eq!(rust_results.len(), 2);
    assert!(
        rust_results
            .iter()
            .all(|e| e.content.contains("Rust") || e.content.contains("rust"))
    );

    // Search for "Python"
    let python_results = garrison.search("Python", 10).await.unwrap();
    assert_eq!(python_results.len(), 2);

    // Search for "programming"
    let programming_results = garrison.search("programming", 10).await.unwrap();
    assert_eq!(programming_results.len(), 1);

    // Search with no results
    let no_results = garrison.search("nonexistent", 10).await.unwrap();
    assert_eq!(no_results.len(), 0);

    // Empty search returns no results
    let empty_search = garrison.search("", 10).await.unwrap();
    assert_eq!(empty_search.len(), 0);
}

#[tokio::test]
async fn test_garrison_recovery_after_restart() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();
    let config = GarrisonConfig::new(10, Some(2000)); // Higher limits to avoid eviction

    // Simulate application lifecycle
    // Phase 1: Initial run - create and populate
    {
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "app-instance-1")
            .await
            .unwrap();

        for i in 0..3 {
            let entry =
                GarrisonEntry::new(ConversationRole::User, format!("Session 1 message {}", i));
            garrison.remember(entry).await.unwrap();
        }

        let stats = garrison.stats().await.unwrap();
        assert_eq!(
            stats.entry_count, 3,
            "Phase 1 should have exactly 3 entries"
        );
    }

    // Phase 2: Application restart - same paladin
    {
        let garrison = SqliteGarrison::connect(&db_path, config.clone(), "app-instance-1")
            .await
            .unwrap();

        // Previous data should still be there
        let entries = garrison.recall_recent(10).await.unwrap();
        assert_eq!(
            entries.len(),
            3,
            "After restart, should still have 3 entries"
        );

        // Add more data
        for i in 0..2 {
            let entry =
                GarrisonEntry::new(ConversationRole::User, format!("Session 2 message {}", i));
            garrison.remember(entry).await.unwrap();
        }

        let stats = garrison.stats().await.unwrap();
        assert_eq!(stats.entry_count, 5, "Phase 2 should have 5 total entries");
    }

    // Phase 3: Another restart - verify complete history
    {
        let garrison = SqliteGarrison::connect(&db_path, config, "app-instance-1")
            .await
            .unwrap();

        let all_entries = garrison.recall_recent(10).await.unwrap();
        assert_eq!(
            all_entries.len(),
            5,
            "Final restart should have all 5 entries"
        );

        // Verify chronological order
        assert!(all_entries[0].content.contains("Session 1 message 0"));
        assert!(all_entries[4].content.contains("Session 2 message 1"));

        // Test search across sessions - use phrase queries to avoid false matches
        let session1_results = garrison.search("\"Session 1 message\"", 10).await.unwrap();
        assert_eq!(
            session1_results.len(),
            3,
            "Should have exactly 3 Session 1 messages"
        );

        let session2_results = garrison.search("\"Session 2 message\"", 10).await.unwrap();
        assert_eq!(
            session2_results.len(),
            2,
            "Should have exactly 2 Session 2 messages"
        );
    }
}

#[tokio::test]
async fn test_sqlite_multi_paladin_isolation() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();
    let config = GarrisonConfig::default();

    // Create two garrisons for different paladins
    let garrison1 = SqliteGarrison::connect(&db_path, config.clone(), "paladin-alpha")
        .await
        .unwrap();
    let garrison2 = SqliteGarrison::connect(&db_path, config, "paladin-beta")
        .await
        .unwrap();

    // Add entries to first paladin
    for i in 0..3 {
        let entry = GarrisonEntry::new(ConversationRole::User, format!("Alpha message {}", i));
        garrison1.remember(entry).await.unwrap();
    }

    // Add entries to second paladin
    for i in 0..2 {
        let entry = GarrisonEntry::new(ConversationRole::User, format!("Beta message {}", i));
        garrison2.remember(entry).await.unwrap();
    }

    // Verify isolation
    let alpha_entries = garrison1.recall_recent(10).await.unwrap();
    assert_eq!(alpha_entries.len(), 3);
    assert!(alpha_entries.iter().all(|e| e.content.contains("Alpha")));

    let beta_entries = garrison2.recall_recent(10).await.unwrap();
    assert_eq!(beta_entries.len(), 2);
    assert!(beta_entries.iter().all(|e| e.content.contains("Beta")));

    // Search should be isolated
    let alpha_search = garrison1.search("Alpha", 10).await.unwrap();
    assert_eq!(alpha_search.len(), 3);

    let beta_search = garrison1.search("Beta", 10).await.unwrap();
    assert_eq!(beta_search.len(), 0); // Should not find Beta entries in Alpha garrison

    // Stats should be independent
    let alpha_stats = garrison1.stats().await.unwrap();
    assert_eq!(alpha_stats.entry_count, 3);

    let beta_stats = garrison2.stats().await.unwrap();
    assert_eq!(beta_stats.entry_count, 2);

    // Forget one shouldn't affect the other
    garrison1.forget_all().await.unwrap();

    let alpha_after = garrison1.recall_recent(10).await.unwrap();
    assert_eq!(alpha_after.len(), 0);

    let beta_after = garrison2.recall_recent(10).await.unwrap();
    assert_eq!(beta_after.len(), 2);
}
