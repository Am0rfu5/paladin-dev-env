//! Integration tests for Arsenal registry service.
//!
//! Tests tool registration, retrieval, unregistration, and concurrent access.

use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
use paladin::core::platform::container::arsenal::Armament;
use paladin_ports::output::arsenal_port::ArsenalRegistry;
use serde_json::json;
use std::sync::Arc;

/// Helper to create a test armament (tool)
fn create_armament(name: &str, description: &str) -> Armament {
    Armament {
        name: name.to_string(),
        description: description.to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "input": { "type": "string" }
            }
        }),
        required_params: vec!["input".to_string()],
    }
}

#[tokio::test]
async fn test_registry_creation() {
    let registry = ArsenalRegistryService::new();
    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_registry_default() {
    let registry = ArsenalRegistryService::default();
    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_register_single_tool() {
    let registry = ArsenalRegistryService::new();

    let tool = create_armament("calculator", "Performs calculations");
    registry.register(tool.clone()).await;

    assert_eq!(registry.count().await, 1);

    let retrieved = registry.get("calculator").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "calculator");
}

#[tokio::test]
async fn test_register_multiple_tools() {
    let registry = ArsenalRegistryService::new();

    // Register multiple tools
    for i in 1..=10 {
        let tool = create_armament(&format!("tool_{}", i), &format!("Tool number {}", i));
        registry.register(tool).await;
    }

    assert_eq!(registry.count().await, 10);

    // Verify all tools can be retrieved
    for i in 1..=10 {
        let tool = registry.get(&format!("tool_{}", i)).await;
        assert!(tool.is_some());
    }
}

#[tokio::test]
async fn test_register_overwrites_existing_tool() {
    let registry = ArsenalRegistryService::new();

    // Register initial tool
    let tool1 = create_armament("updatable_tool", "Original description");
    registry.register(tool1).await;

    assert_eq!(registry.count().await, 1);

    // Register tool with same name but different description
    let tool2 = create_armament("updatable_tool", "Updated description");
    registry.register(tool2).await;

    // Count should still be 1 (overwritten, not added)
    assert_eq!(registry.count().await, 1);

    // Retrieved tool should have updated description
    let retrieved = registry.get("updatable_tool").await.unwrap();
    assert_eq!(retrieved.description, "Updated description");
}

#[tokio::test]
async fn test_get_nonexistent_tool() {
    let registry = ArsenalRegistryService::new();

    let tool = registry.get("nonexistent").await;
    assert!(tool.is_none());
}

#[tokio::test]
async fn test_unregister_tool() {
    let registry = ArsenalRegistryService::new();

    // Register a tool
    let tool = create_armament("removable_tool", "Will be removed");
    registry.register(tool).await;

    assert_eq!(registry.count().await, 1);

    // Unregister the tool
    let removed = registry.unregister("removable_tool").await;

    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "removable_tool");
    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_unregister_nonexistent_tool() {
    let registry = ArsenalRegistryService::new();

    let removed = registry.unregister("nonexistent").await;
    assert!(removed.is_none());
}

#[tokio::test]
async fn test_unregister_returns_correct_tool() {
    let registry = ArsenalRegistryService::new();

    // Register multiple tools
    let tool1 = create_armament("tool1", "First tool");
    let tool2 = create_armament("tool2", "Second tool");
    registry.register(tool1).await;
    registry.register(tool2).await;

    // Unregister one tool
    let removed = registry.unregister("tool1").await.unwrap();

    assert_eq!(removed.name, "tool1");
    assert_eq!(removed.description, "First tool");

    // Other tool should still be present
    assert_eq!(registry.count().await, 1);
    assert!(registry.get("tool2").await.is_some());
    assert!(registry.get("tool1").await.is_none());
}

#[tokio::test]
async fn test_clear_registry() {
    let registry = ArsenalRegistryService::new();

    // Register multiple tools
    for i in 1..=5 {
        let tool = create_armament(&format!("tool_{}", i), "Test tool");
        registry.register(tool).await;
    }

    assert_eq!(registry.count().await, 5);

    // Clear all tools
    registry.clear().await;

    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_clear_empty_registry() {
    let registry = ArsenalRegistryService::new();

    // Clear empty registry (should not panic)
    registry.clear().await;

    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_concurrent_registrations() {
    let registry = Arc::new(ArsenalRegistryService::new());

    // Concurrent registrations
    let mut handles = vec![];
    for i in 1..=20 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let tool = create_armament(&format!("concurrent_tool_{}", i), "Concurrent test");
            registry_clone.register(tool).await;
        });
        handles.push(handle);
    }

    // Wait for all registrations
    for handle in handles {
        handle.await.unwrap();
    }

    // All tools should be registered
    assert_eq!(registry.count().await, 20);
}

#[tokio::test]
async fn test_concurrent_reads() {
    let registry = Arc::new(ArsenalRegistryService::new());

    // Register a tool
    let tool = create_armament("shared_tool", "Shared across reads");
    registry.register(tool).await;

    // Concurrent reads
    let mut handles = vec![];
    for _ in 1..=50 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move { registry_clone.get("shared_tool").await });
        handles.push(handle);
    }

    // Wait for all reads
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // All reads should succeed
    assert_eq!(results.len(), 50);
    for result in results {
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "shared_tool");
    }
}

#[tokio::test]
async fn test_concurrent_mixed_operations() {
    let registry = Arc::new(ArsenalRegistryService::new());

    // Pre-register some tools
    for i in 1..=5 {
        let tool = create_armament(&format!("tool_{}", i), "Initial tool");
        registry.register(tool).await;
    }

    // Concurrent mixed operations
    let mut reg_handles = vec![];
    let mut read_handles = vec![];
    let mut unreg_handles = vec![];

    // Registrations
    for i in 6..=15 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let tool = create_armament(&format!("tool_{}", i), "New tool");
            registry_clone.register(tool).await;
        });
        reg_handles.push(handle);
    }

    // Reads
    for i in 1..=5 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move { registry_clone.get(&format!("tool_{}", i)).await });
        read_handles.push(handle);
    }

    // Unregistrations
    for i in 1..=3 {
        let registry_clone = registry.clone();
        let handle =
            tokio::spawn(async move { registry_clone.unregister(&format!("tool_{}", i)).await });
        unreg_handles.push(handle);
    }

    // Wait for all operations
    for handle in reg_handles {
        handle.await.unwrap();
    }
    for handle in read_handles {
        handle.await.unwrap();
    }
    for handle in unreg_handles {
        handle.await.unwrap();
    }

    // Count should reflect registrations and unregistrations
    // Started with 5, added 10, removed 3 = 12
    assert_eq!(registry.count().await, 12);
}

#[tokio::test]
async fn test_registry_clone() {
    let registry1 = ArsenalRegistryService::new();

    // Register a tool in registry1
    let tool = create_armament("cloned_tool", "Test cloning");
    registry1.register(tool).await;

    // Clone the registry
    let registry2 = registry1.clone();

    // Both should see the same tool (they share the Arc<RwLock>)
    assert_eq!(registry1.count().await, 1);
    assert_eq!(registry2.count().await, 1);

    // Modifications in one should be visible in the other
    registry2
        .register(create_armament("new_tool", "Added via clone"))
        .await;

    assert_eq!(registry1.count().await, 2);
    assert_eq!(registry2.count().await, 2);
}

#[tokio::test]
async fn test_get_retrieves_complete_tool_metadata() {
    let registry = ArsenalRegistryService::new();

    let tool = Armament {
        name: "metadata_test".to_string(),
        description: "Testing complete metadata".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "param1": { "type": "string" },
                "param2": { "type": "number" }
            },
            "required": ["param1"]
        }),
        required_params: vec!["param1".to_string(), "param2".to_string()],
    };

    registry.register(tool.clone()).await;

    let retrieved = registry.get("metadata_test").await.unwrap();

    // Verify all fields are preserved
    assert_eq!(retrieved.name, tool.name);
    assert_eq!(retrieved.description, tool.description);
    assert_eq!(retrieved.parameters, tool.parameters);
    assert_eq!(retrieved.required_params, tool.required_params);
}

#[tokio::test]
async fn test_count_accuracy_after_operations() {
    let registry = ArsenalRegistryService::new();

    // Initial count
    assert_eq!(registry.count().await, 0);

    // Add 10 tools
    for i in 1..=10 {
        registry
            .register(create_armament(&format!("tool_{}", i), "Test"))
            .await;
    }
    assert_eq!(registry.count().await, 10);

    // Remove 3 tools
    for i in 1..=3 {
        registry.unregister(&format!("tool_{}", i)).await;
    }
    assert_eq!(registry.count().await, 7);

    // Add 2 more tools
    for i in 11..=12 {
        registry
            .register(create_armament(&format!("tool_{}", i), "Test"))
            .await;
    }
    assert_eq!(registry.count().await, 9);

    // Clear all
    registry.clear().await;
    assert_eq!(registry.count().await, 0);
}
