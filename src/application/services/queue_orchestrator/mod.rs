/*
Queue Orchestrator

Application-layer orchestrator for queue management.  Relocated from
`core::platform::manager::queue_service` as part of Milestone 6 Epic 2 to enforce
hexagonal architecture boundaries.

This module provides FIFO / priority-queue management for the platform, coordinating
queue lifecycle, item processing state, and cleanup workers.
*/

pub mod types;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::core::platform::container::queue_config::QueueConfig;
use crate::core::platform::container::queue_item::QueueItem;

use types::Queue;
pub use types::{QueueError, QueueStats};

// Re-export QueueConfig so existing `pub use queue_service::QueueConfig` call-sites
// can be migrated to this module in one step.
pub use crate::core::platform::container::queue_config::QueueConfig as QueueConfigAlias;

// ---------------------------------------------------------------------------
// Backwards-compatibility type alias
// ---------------------------------------------------------------------------

/// Alias kept so call-sites that still use the old `QueueService` name compile
/// without changes.
pub type QueueService = QueueOrchestrator;

// ---------------------------------------------------------------------------
// Orchestrator
// ---------------------------------------------------------------------------

/// Application-layer queue orchestrator.
///
/// Manages named in-process FIFO/priority queues with processing lifecycle
/// tracking and periodic cleanup.  Previously named `QueueService`.
#[derive(Debug)]
pub struct QueueOrchestrator {
    queues: Arc<RwLock<HashMap<String, Arc<Mutex<Queue>>>>>,
    default_config: QueueConfig,
}

impl QueueOrchestrator {
    /// Create a new `QueueOrchestrator` with default configuration.
    pub fn new() -> Self {
        Self {
            queues: Arc::new(RwLock::new(HashMap::new())),
            default_config: QueueConfig::default(),
        }
    }

    /// Create a new `QueueOrchestrator` with a custom default queue configuration.
    pub fn with_default_config(config: QueueConfig) -> Self {
        Self {
            queues: Arc::new(RwLock::new(HashMap::new())),
            default_config: config,
        }
    }

    /// Create a named queue.
    pub async fn create_queue(
        &self,
        name: String,
        config: Option<QueueConfig>,
    ) -> Result<(), QueueError> {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        let queue = Arc::new(Mutex::new(Queue::new(name.clone(), config)));
        let mut queues = self.queues.write().await;
        queues.insert(name, queue);
        Ok(())
    }

    /// Delete a named queue.
    pub async fn delete_queue(&self, name: &str) -> Result<(), QueueError> {
        let mut queues = self.queues.write().await;
        queues
            .remove(name)
            .ok_or_else(|| QueueError::QueueNotFound(name.to_string()))?;
        Ok(())
    }

    /// Enqueue an item.
    pub async fn enqueue<T>(&self, queue_name: &str, item: QueueItem<T>) -> Result<Uuid, QueueError>
    where
        T: serde::Serialize + Clone + for<'de> serde::Deserialize<'de>,
    {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let json_item = item.map_payload(|payload| {
            serde_json::to_value(payload).unwrap_or(serde_json::Value::Null)
        });

        let item_id = json_item.id();
        let mut queue_guard = queue.lock().await;
        queue_guard.enqueue(json_item)?;

        Ok(item_id)
    }

    /// Dequeue an item (moves it to the processing state).
    pub async fn dequeue(
        &self,
        queue_name: &str,
    ) -> Result<Option<QueueItem<serde_json::Value>>, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let mut queue_guard = queue.lock().await;
        let item = queue_guard.dequeue();

        if let Some(item) = item {
            let item_id = item.id();
            queue_guard.processing_items.insert(item_id, item.clone());
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Mark an item as actively being processed by a worker.
    pub async fn start_processing(
        &self,
        queue_name: &str,
        item_id: Uuid,
        worker_id: String,
    ) -> Result<(), QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let mut queue_guard = queue.lock().await;
        queue_guard.start_processing(item_id, worker_id)
    }

    /// Mark an item as successfully completed.
    pub async fn complete_processing(
        &self,
        queue_name: &str,
        item_id: Uuid,
        result_data: Option<serde_json::Value>,
    ) -> Result<(), QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let mut queue_guard = queue.lock().await;
        queue_guard.complete_processing(item_id, result_data)
    }

    /// Mark an item as failed; returns `true` if the item was re-queued for retry.
    pub async fn fail_processing(
        &self,
        queue_name: &str,
        item_id: Uuid,
        error: String,
    ) -> Result<bool, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let mut queue_guard = queue.lock().await;
        queue_guard.fail_processing(item_id, error)
    }

    /// Get statistics for a named queue.
    pub async fn get_queue_stats(&self, queue_name: &str) -> Result<QueueStats, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let queue_guard = queue.lock().await;
        Ok(queue_guard.stats.clone())
    }

    /// List all queue names.
    pub async fn list_queues(&self) -> Vec<String> {
        let queues = self.queues.read().await;
        queues.keys().cloned().collect()
    }

    /// Return the number of pending items in a queue.
    pub async fn queue_length(&self, queue_name: &str) -> Result<usize, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let queue_guard = queue.lock().await;
        Ok(queue_guard.items.len())
    }

    /// Cleanup expired items across all queues.
    pub async fn cleanup_expired(&self) {
        let queues = self.queues.read().await;
        for queue in queues.values() {
            let mut queue_guard = queue.lock().await;
            queue_guard.cleanup_expired();
        }
    }

    /// Get statistics for every queue.
    pub async fn get_all_stats(&self) -> HashMap<String, QueueStats> {
        let queues = self.queues.read().await;
        let mut stats = HashMap::new();
        for (name, queue) in queues.iter() {
            let queue_guard = queue.lock().await;
            stats.insert(name.clone(), queue_guard.stats.clone());
        }
        stats
    }
}

impl Default for QueueOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::entity::message::{Location, Message};
    use crate::core::platform::container::queue_item::QueueItemConfig;

    #[tokio::test]
    async fn test_queue_creation() {
        let orchestrator = QueueOrchestrator::new();
        let result = orchestrator
            .create_queue("test-queue".to_string(), None)
            .await;
        assert!(result.is_ok());

        let queues = orchestrator.list_queues().await;
        assert!(queues.contains(&"test-queue".to_string()));
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        let item_id = orchestrator
            .enqueue("test-queue", queue_item)
            .await
            .unwrap();

        let dequeued = orchestrator.dequeue("test-queue").await.unwrap();
        assert!(dequeued.is_some());
        assert_eq!(dequeued.unwrap().id(), item_id);
    }

    #[tokio::test]
    async fn test_processing_lifecycle() {
        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        let _item_id = orchestrator
            .enqueue("test-queue", queue_item)
            .await
            .unwrap();

        let dequeued = orchestrator.dequeue("test-queue").await.unwrap().unwrap();
        let item_id = dequeued.id();

        let result = orchestrator
            .start_processing("test-queue", item_id, "worker-1".to_string())
            .await;
        assert!(result.is_ok());

        let result = orchestrator
            .complete_processing(
                "test-queue",
                item_id,
                Some(serde_json::json!({"result": "success"})),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_queue_stats() {
        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        orchestrator
            .enqueue("test-queue", queue_item)
            .await
            .unwrap();

        let stats = orchestrator.get_queue_stats("test-queue").await.unwrap();
        assert_eq!(stats.pending_items, 1);
        assert_eq!(stats.total_items, 1);
    }

    #[tokio::test]
    async fn test_queue_config_default() {
        let config = QueueConfig::default();
        assert_eq!(config.max_capacity, 10000);
        assert!(!config.preserve_completed);
        assert!(config.preserve_failed);
        assert_eq!(config.cleanup_interval_seconds, 300);
        assert!(config.priority_based);
    }

    #[tokio::test]
    async fn test_service_with_custom_config() {
        let custom_config = QueueConfig {
            max_capacity: 5,
            preserve_completed: true,
            preserve_failed: false,
            cleanup_interval_seconds: 600,
            priority_based: false,
            default_item_config: QueueItemConfig::default(),
        };

        let orchestrator = QueueOrchestrator::with_default_config(custom_config);
        orchestrator
            .create_queue("custom-queue".to_string(), None)
            .await
            .unwrap();

        let stats = orchestrator.get_queue_stats("custom-queue").await.unwrap();
        assert_eq!(stats.total_items, 0);
    }

    #[tokio::test]
    async fn test_delete_queue() {
        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("temp-queue".to_string(), None)
            .await
            .unwrap();

        let result = orchestrator.delete_queue("temp-queue").await;
        assert!(result.is_ok());

        let queues = orchestrator.list_queues().await;
        assert!(!queues.contains(&"temp-queue".to_string()));
    }

    #[tokio::test]
    async fn test_delete_nonexistent_queue() {
        let orchestrator = QueueOrchestrator::new();
        let result = orchestrator.delete_queue("nonexistent").await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueNotFound(name)) => assert_eq!(name, "nonexistent"),
            _ => panic!("Expected QueueNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_enqueue_to_nonexistent_queue() {
        let orchestrator = QueueOrchestrator::new();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test".to_string(),
        );
        let queue_item = QueueItem::new("nonexistent".to_string(), message, None);

        let result = orchestrator.enqueue("nonexistent", queue_item).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dequeue_from_empty_queue() {
        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("empty-queue".to_string(), None)
            .await
            .unwrap();

        let result = orchestrator.dequeue("empty-queue").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_dequeue_from_nonexistent_queue() {
        let orchestrator = QueueOrchestrator::new();
        let result = orchestrator.dequeue("nonexistent").await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueNotFound(_)) => {}
            _ => panic!("Expected QueueNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_queue_full_behavior() {
        let config = QueueConfig {
            max_capacity: 2,
            ..Default::default()
        };

        let orchestrator = QueueOrchestrator::new();
        orchestrator
            .create_queue("small-queue".to_string(), Some(config))
            .await
            .unwrap();

        for i in 0..2 {
            let message = Message::new(
                Location::service("test"),
                Location::system("queue"),
                format!("payload {}", i),
            );
            let queue_item = QueueItem::new("small-queue".to_string(), message, None);
            orchestrator
                .enqueue("small-queue", queue_item)
                .await
                .unwrap();
        }

        // Third enqueue should fail
        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "overflow".to_string(),
        );
        let queue_item = QueueItem::new("small-queue".to_string(), message, None);
        let result = orchestrator.enqueue("small-queue", queue_item).await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueFull { .. }) => {}
            _ => panic!("Expected QueueFull error"),
        }
    }
}
