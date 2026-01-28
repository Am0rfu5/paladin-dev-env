/*
Queue Manager Service

This is the Queue Manager Service, it is responsible for managing queue. This module
contains the Queue Manager Service, its related traits and implementations.

Within our Hexagonal Architecture, the Queue Manager Service is at the Platform Layer,
above the Domain Layer and below the Application layer. It places Queue Item Containers
into a Queue and retrieves them from a Queue.

The Queue is a FIFO (First In First Out) data structure that is used to store Queue Items.
The Queue Service is abstracted in the application layer with a Hexagonal Architecture "Port" and Adapter that multiple Queues may exist.

Ports for the External Queue Adapters are also on the Infrastructure Layer.

An example of an external queue would be a message broker like RabbitMQ, Kafka, or AWS SQS.
*/

use crate::core::platform::container::queue_item::{QueueItem, QueueItemConfig};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

/// Queue service errors
#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Queue not found: {0}")]
    QueueNotFound(String),
    #[error("Queue item not found: {0}")]
    ItemNotFound(Uuid),
    #[error("Queue is full: {queue_name} (capacity: {capacity})")]
    QueueFull { queue_name: String, capacity: usize },
    #[error("Queue is empty: {0}")]
    QueueEmpty(String),
    #[error("Invalid queue configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Queue operation failed: {0}")]
    OperationFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Queue configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Maximum number of items in the queue (0 = unlimited)
    pub max_capacity: usize,
    /// Default configuration for items in this queue
    pub default_item_config: QueueItemConfig,
    /// Whether to preserve completed items
    pub preserve_completed: bool,
    /// Whether to preserve failed items
    pub preserve_failed: bool,
    /// Auto-cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
    /// Priority-based processing
    pub priority_based: bool,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10000,
            default_item_config: QueueItemConfig::default(),
            preserve_completed: false,
            preserve_failed: true,
            cleanup_interval_seconds: 300, // 5 minutes
            priority_based: true,
        }
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub name: String,
    pub total_items: usize,
    pub pending_items: usize,
    pub processing_items: usize,
    pub completed_items: usize,
    pub failed_items: usize,
    pub abandoned_items: usize,
    pub oldest_item_age_seconds: Option<i64>,
    pub average_processing_time_ms: Option<u64>,
    pub throughput_per_minute: f64,
}

/// Internal queue structure
#[derive(Debug)]
struct Queue {
    name: String,
    config: QueueConfig,
    items: VecDeque<QueueItem<serde_json::Value>>,
    processing_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    completed_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    failed_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    stats: QueueStats,
    // TODO this should be set so it can be read
    // ignored for now dead code
    #[allow(dead_code)]
    created_at: DateTime<Utc>,
    last_cleanup: DateTime<Utc>,
}

impl Queue {
    fn new(name: String, config: QueueConfig) -> Self {
        let now = Utc::now();
        Self {
            stats: QueueStats {
                name: name.clone(),
                total_items: 0,
                pending_items: 0,
                processing_items: 0,
                completed_items: 0,
                failed_items: 0,
                abandoned_items: 0,
                oldest_item_age_seconds: None,
                average_processing_time_ms: None,
                throughput_per_minute: 0.0,
            },
            name,
            config,
            items: VecDeque::new(),
            processing_items: HashMap::new(),
            completed_items: HashMap::new(),
            failed_items: HashMap::new(),
            created_at: now,
            last_cleanup: now,
        }
    }

    fn is_full(&self) -> bool {
        if self.config.max_capacity == 0 {
            return false; // Unlimited capacity
        }
        self.items.len() + self.processing_items.len() >= self.config.max_capacity
    }

    fn enqueue(&mut self, mut item: QueueItem<serde_json::Value>) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::QueueFull {
                queue_name: self.name.clone(),
                capacity: self.config.max_capacity,
            });
        }

        // Apply default config if item doesn't have custom config
        if item.attempt_count == 0 {
            item.config = self.config.default_item_config.clone();
        }

        if self.config.priority_based {
            // Insert based on priority
            let priority = item.message.priority;
            let mut inserted = false;

            for (index, existing_item) in self.items.iter().enumerate() {
                if priority > existing_item.message.priority {
                    self.items.insert(index, item.clone());
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                self.items.push_back(item);
            }
        } else {
            // FIFO
            self.items.push_back(item);
        }

        self.update_stats();
        Ok(())
    }

    fn dequeue(&mut self) -> Option<QueueItem<serde_json::Value>> {
        // Find first processable item
        let mut item_index = None;
        for (index, item) in self.items.iter().enumerate() {
            if item.can_process() && !item.is_expired() {
                item_index = Some(index);
                break;
            }
        }

        if let Some(index) = item_index {
            let item = self.items.remove(index).unwrap();
            self.update_stats();
            Some(item)
        } else {
            None
        }
    }

    fn start_processing(&mut self, item_id: Uuid, worker_id: String) -> Result<(), QueueError> {
        if let Some(item) = self.processing_items.get_mut(&item_id) {
            item.start_processing(worker_id)
                .map_err(QueueError::OperationFailed)?;
            self.update_stats();
            Ok(())
        } else {
            Err(QueueError::ItemNotFound(item_id))
        }
    }

    fn complete_processing(
        &mut self,
        item_id: Uuid,
        result_data: Option<serde_json::Value>,
    ) -> Result<(), QueueError> {
        if let Some(mut item) = self.processing_items.remove(&item_id) {
            item.complete_processing(result_data);

            if self.config.preserve_completed {
                self.completed_items.insert(item_id, item);
            }

            self.update_stats();
            Ok(())
        } else {
            Err(QueueError::ItemNotFound(item_id))
        }
    }

    fn fail_processing(&mut self, item_id: Uuid, error: String) -> Result<bool, QueueError> {
        if let Some(mut item) = self.processing_items.remove(&item_id) {
            let can_retry = item.fail_processing(error);

            if can_retry {
                // Re-queue for retry
                self.items.push_back(item);
            } else {
                // Move to failed items
                if self.config.preserve_failed {
                    self.failed_items.insert(item_id, item);
                }
            }

            self.update_stats();
            Ok(can_retry)
        } else {
            Err(QueueError::ItemNotFound(item_id))
        }
    }

    fn update_stats(&mut self) {
        self.stats.total_items = self.items.len()
            + self.processing_items.len()
            + self.completed_items.len()
            + self.failed_items.len();
        self.stats.pending_items = self.items.len();
        self.stats.processing_items = self.processing_items.len();
        self.stats.completed_items = self.completed_items.len();
        self.stats.failed_items = self.failed_items.len();

        // Calculate oldest item age
        self.stats.oldest_item_age_seconds = self
            .items
            .iter()
            .map(|item| item.message.age_seconds())
            .max();
    }

    fn cleanup_expired(&mut self) {
        let now = Utc::now();

        // Remove expired items from pending queue
        self.items.retain(|item| !item.is_expired());

        // Handle processing timeouts
        let timed_out_items: Vec<_> = self
            .processing_items
            .iter()
            .filter(|(_, item)| item.is_processing_timeout())
            .map(|(id, _)| *id)
            .collect();

        for item_id in timed_out_items {
            if let Some(mut item) = self.processing_items.remove(&item_id) {
                item.fail_processing("Processing timeout".to_string());
                if self.config.preserve_failed {
                    self.failed_items.insert(item_id, item);
                }
            }
        }

        self.last_cleanup = now;
        self.update_stats();
    }
}

/// Main Queue Service
#[derive(Debug)]
pub struct QueueService {
    queues: Arc<RwLock<HashMap<String, Arc<Mutex<Queue>>>>>,
    default_config: QueueConfig,
}

impl QueueService {
    /// Create a new queue service
    pub fn new() -> Self {
        Self {
            queues: Arc::new(RwLock::new(HashMap::new())),
            default_config: QueueConfig::default(),
        }
    }

    /// Create a new queue service with custom default configuration
    pub fn with_default_config(config: QueueConfig) -> Self {
        Self {
            queues: Arc::new(RwLock::new(HashMap::new())),
            default_config: config,
        }
    }

    /// Create a new queue
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

    /// Delete a queue
    pub async fn delete_queue(&self, name: &str) -> Result<(), QueueError> {
        let mut queues = self.queues.write().await;
        queues
            .remove(name)
            .ok_or_else(|| QueueError::QueueNotFound(name.to_string()))?;
        Ok(())
    }

    /// Enqueue an item
    pub async fn enqueue<T>(&self, queue_name: &str, item: QueueItem<T>) -> Result<Uuid, QueueError>
    where
        T: serde::Serialize + Clone + for<'de> serde::Deserialize<'de>,
    {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        // Convert to JSON value for storage
        let json_item = item.map_payload(|payload| {
            serde_json::to_value(payload).unwrap_or(serde_json::Value::Null)
        });

        let item_id = json_item.id();
        let mut queue_guard = queue.lock().await;
        queue_guard.enqueue(json_item)?;

        Ok(item_id)
    }

    /// Dequeue an item
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

        // Move to processing if item was found
        if let Some(item) = item {
            let item_id = item.id();
            queue_guard.processing_items.insert(item_id, item.clone());
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Start processing an item
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

    /// Complete processing an item
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

    /// Fail processing an item
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

    /// Get queue statistics
    pub async fn get_queue_stats(&self, queue_name: &str) -> Result<QueueStats, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let queue_guard = queue.lock().await;
        Ok(queue_guard.stats.clone())
    }

    /// List all queues
    pub async fn list_queues(&self) -> Vec<String> {
        let queues = self.queues.read().await;
        queues.keys().cloned().collect()
    }

    /// Get queue length
    pub async fn queue_length(&self, queue_name: &str) -> Result<usize, QueueError> {
        let queues = self.queues.read().await;
        let queue = queues
            .get(queue_name)
            .ok_or_else(|| QueueError::QueueNotFound(queue_name.to_string()))?;

        let queue_guard = queue.lock().await;
        Ok(queue_guard.items.len())
    }

    /// Cleanup expired items across all queues
    pub async fn cleanup_expired(&self) {
        let queues = self.queues.read().await;
        for queue in queues.values() {
            let mut queue_guard = queue.lock().await;
            queue_guard.cleanup_expired();
        }
    }

    /// Get all queue statistics
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

impl Default for QueueService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::entity::message::{Location, Message};

    #[tokio::test]
    async fn test_queue_creation() {
        let service = QueueService::new();

        let result = service.create_queue("test-queue".to_string(), None).await;
        assert!(result.is_ok());

        let queues = service.list_queues().await;
        assert!(queues.contains(&"test-queue".to_string()));
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        let item_id = service.enqueue("test-queue", queue_item).await.unwrap();

        let dequeued = service.dequeue("test-queue").await.unwrap();
        assert!(dequeued.is_some());

        let item = dequeued.unwrap();
        assert_eq!(item.id(), item_id);
    }

    #[tokio::test]
    async fn test_processing_lifecycle() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        // ToDo - item_id is currently unread but maybe should be verified with a test
        let _item_id = service.enqueue("test-queue", queue_item).await.unwrap();

        let dequeued = service.dequeue("test-queue").await.unwrap().unwrap();
        let item_id = dequeued.id();

        // Start processing
        let result = service
            .start_processing("test-queue", item_id, "worker-1".to_string())
            .await;
        assert!(result.is_ok());

        // Complete processing
        let result = service
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
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test payload".to_string(),
        );

        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        service.enqueue("test-queue", queue_item).await.unwrap();

        let stats = service.get_queue_stats("test-queue").await.unwrap();
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
    async fn test_error_variants_display() {
        let queue_not_found = QueueError::QueueNotFound("my_queue".to_string());
        assert_eq!(queue_not_found.to_string(), "Queue not found: my_queue");

        let item_not_found = QueueError::ItemNotFound(Uuid::new_v4());
        assert!(item_not_found.to_string().contains("Queue item not found"));

        let queue_full = QueueError::QueueFull {
            queue_name: "full_queue".to_string(),
            capacity: 100,
        };
        assert_eq!(
            queue_full.to_string(),
            "Queue is full: full_queue (capacity: 100)"
        );

        let queue_empty = QueueError::QueueEmpty("empty_queue".to_string());
        assert_eq!(queue_empty.to_string(), "Queue is empty: empty_queue");

        let invalid_config = QueueError::InvalidConfiguration("bad config".to_string());
        assert_eq!(
            invalid_config.to_string(),
            "Invalid queue configuration: bad config"
        );

        let operation_failed = QueueError::OperationFailed("timeout".to_string());
        assert_eq!(
            operation_failed.to_string(),
            "Queue operation failed: timeout"
        );

        let serialization_err = QueueError::SerializationError("invalid json".to_string());
        assert_eq!(
            serialization_err.to_string(),
            "Serialization error: invalid json"
        );
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

        let service = QueueService::with_default_config(custom_config.clone());
        service
            .create_queue("custom-queue".to_string(), None)
            .await
            .unwrap();

        let stats = service.get_queue_stats("custom-queue").await.unwrap();
        assert_eq!(stats.total_items, 0);
    }

    #[tokio::test]
    async fn test_delete_queue() {
        let service = QueueService::new();
        service
            .create_queue("temp-queue".to_string(), None)
            .await
            .unwrap();

        let queues = service.list_queues().await;
        assert!(queues.contains(&"temp-queue".to_string()));

        let result = service.delete_queue("temp-queue").await;
        assert!(result.is_ok());

        let queues = service.list_queues().await;
        assert!(!queues.contains(&"temp-queue".to_string()));
    }

    #[tokio::test]
    async fn test_delete_nonexistent_queue() {
        let service = QueueService::new();
        let result = service.delete_queue("nonexistent").await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueNotFound(name)) => {
                assert_eq!(name, "nonexistent");
            }
            _ => panic!("Expected QueueNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_enqueue_to_nonexistent_queue() {
        let service = QueueService::new();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test".to_string(),
        );
        let queue_item = QueueItem::new("nonexistent".to_string(), message, None);

        let result = service.enqueue("nonexistent", queue_item).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dequeue_from_empty_queue() {
        let service = QueueService::new();
        service
            .create_queue("empty-queue".to_string(), None)
            .await
            .unwrap();

        let result = service.dequeue("empty-queue").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_dequeue_from_nonexistent_queue() {
        let service = QueueService::new();
        let result = service.dequeue("nonexistent").await;

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

        let service = QueueService::new();
        service
            .create_queue("small-queue".to_string(), Some(config))
            .await
            .unwrap();

        // Fill the queue
        for i in 0..2 {
            let message = Message::new(
                Location::service("test"),
                Location::system("queue"),
                format!("message {}", i),
            );
            let queue_item = QueueItem::new("small-queue".to_string(), message, None);
            service.enqueue("small-queue", queue_item).await.unwrap();
        }

        // Try to add one more - should fail
        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "overflow".to_string(),
        );
        let queue_item = QueueItem::new("small-queue".to_string(), message, None);
        let result = service.enqueue("small-queue", queue_item).await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueFull {
                queue_name,
                capacity,
            }) => {
                assert_eq!(queue_name, "small-queue");
                assert_eq!(capacity, 2);
            }
            _ => panic!("Expected QueueFull error"),
        }
    }

    #[tokio::test]
    async fn test_multiple_queues() {
        let service = QueueService::new();

        service
            .create_queue("queue1".to_string(), None)
            .await
            .unwrap();
        service
            .create_queue("queue2".to_string(), None)
            .await
            .unwrap();
        service
            .create_queue("queue3".to_string(), None)
            .await
            .unwrap();

        let queues = service.list_queues().await;
        assert_eq!(queues.len(), 3);
        assert!(queues.contains(&"queue1".to_string()));
        assert!(queues.contains(&"queue2".to_string()));
        assert!(queues.contains(&"queue3".to_string()));
    }

    #[tokio::test]
    async fn test_fail_processing() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let message = Message::new(
            Location::service("test"),
            Location::system("queue"),
            "test".to_string(),
        );
        let queue_item = QueueItem::new("test-queue".to_string(), message, None);
        service.enqueue("test-queue", queue_item).await.unwrap();

        let dequeued = service.dequeue("test-queue").await.unwrap().unwrap();
        let item_id = dequeued.id();

        service
            .start_processing("test-queue", item_id, "worker-1".to_string())
            .await
            .unwrap();

        let result = service
            .fail_processing("test-queue", item_id, "Test error".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fail_processing_nonexistent_item() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let fake_id = Uuid::new_v4();
        let result = service
            .fail_processing("test-queue", fake_id, "error".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(QueueError::ItemNotFound(_)) => {}
            _ => panic!("Expected ItemNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_stats_nonexistent_queue() {
        let service = QueueService::new();
        let result = service.get_queue_stats("nonexistent").await;

        assert!(result.is_err());
        match result {
            Err(QueueError::QueueNotFound(_)) => {}
            _ => panic!("Expected QueueNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_all_queues_stats() {
        let service = QueueService::new();
        service
            .create_queue("queue1".to_string(), None)
            .await
            .unwrap();
        service
            .create_queue("queue2".to_string(), None)
            .await
            .unwrap();

        let all_stats = service.get_all_stats().await;
        assert_eq!(all_stats.len(), 2);
        assert!(all_stats.contains_key("queue1"));
        assert!(all_stats.contains_key("queue2"));
    }

    #[tokio::test]
    async fn test_queue_stats_structure() {
        let service = QueueService::new();
        service
            .create_queue("stats-queue".to_string(), None)
            .await
            .unwrap();

        let stats = service.get_queue_stats("stats-queue").await.unwrap();
        assert_eq!(stats.name, "stats-queue");
        assert_eq!(stats.total_items, 0);
        assert_eq!(stats.pending_items, 0);
        assert_eq!(stats.processing_items, 0);
        assert_eq!(stats.completed_items, 0);
        assert_eq!(stats.failed_items, 0);
        assert_eq!(stats.abandoned_items, 0);
        assert!(stats.oldest_item_age_seconds.is_none());
        assert!(stats.average_processing_time_ms.is_none());
        assert_eq!(stats.throughput_per_minute, 0.0);
    }

    #[tokio::test]
    async fn test_complete_processing_nonexistent_item() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let fake_id = Uuid::new_v4();
        let result = service
            .complete_processing("test-queue", fake_id, None)
            .await;

        assert!(result.is_err());
        match result {
            Err(QueueError::ItemNotFound(_)) => {}
            _ => panic!("Expected ItemNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_start_processing_nonexistent_item() {
        let service = QueueService::new();
        service
            .create_queue("test-queue".to_string(), None)
            .await
            .unwrap();

        let fake_id = Uuid::new_v4();
        let result = service
            .start_processing("test-queue", fake_id, "worker-1".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(QueueError::ItemNotFound(_)) => {}
            _ => panic!("Expected ItemNotFound error"),
        }
    }
}
