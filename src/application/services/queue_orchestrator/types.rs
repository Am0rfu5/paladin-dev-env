use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;
use uuid::Uuid;

use crate::core::platform::container::queue_config::QueueConfig;
use crate::core::platform::container::queue_item::QueueItem;
// QueueStats lives in paladin-core after Task 2.0
pub use paladin_core::platform::container::queue_config::QueueStats;

/// Queue orchestrator errors
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

/// Internal queue structure (application-layer coordination state, not a domain value object)
#[derive(Debug)]
pub(super) struct Queue {
    pub(super) name: String,
    pub(super) config: QueueConfig,
    pub(super) items: VecDeque<QueueItem<serde_json::Value>>,
    pub(super) processing_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    pub(super) completed_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    pub(super) failed_items: HashMap<Uuid, QueueItem<serde_json::Value>>,
    pub(super) stats: QueueStats,
    #[allow(dead_code)]
    pub(super) created_at: DateTime<Utc>,
    pub(super) last_cleanup: DateTime<Utc>,
}

impl Queue {
    pub(super) fn new(name: String, config: QueueConfig) -> Self {
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

    pub(super) fn is_full(&self) -> bool {
        if self.config.max_capacity == 0 {
            return false;
        }
        self.items.len() + self.processing_items.len() >= self.config.max_capacity
    }

    pub(super) fn enqueue(
        &mut self,
        mut item: QueueItem<serde_json::Value>,
    ) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::QueueFull {
                queue_name: self.name.clone(),
                capacity: self.config.max_capacity,
            });
        }

        if item.attempt_count == 0 {
            item.config = self.config.default_item_config.clone();
        }

        if self.config.priority_based {
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
            self.items.push_back(item);
        }

        self.update_stats();
        Ok(())
    }

    pub(super) fn dequeue(&mut self) -> Option<QueueItem<serde_json::Value>> {
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

    pub(super) fn start_processing(
        &mut self,
        item_id: Uuid,
        worker_id: String,
    ) -> Result<(), QueueError> {
        if let Some(item) = self.processing_items.get_mut(&item_id) {
            item.start_processing(worker_id)
                .map_err(QueueError::OperationFailed)?;
            self.update_stats();
            Ok(())
        } else {
            Err(QueueError::ItemNotFound(item_id))
        }
    }

    pub(super) fn complete_processing(
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

    pub(super) fn fail_processing(
        &mut self,
        item_id: Uuid,
        error: String,
    ) -> Result<bool, QueueError> {
        if let Some(mut item) = self.processing_items.remove(&item_id) {
            let can_retry = item.fail_processing(error);
            if can_retry {
                self.items.push_back(item);
            } else if self.config.preserve_failed {
                self.failed_items.insert(item_id, item);
            }
            self.update_stats();
            Ok(can_retry)
        } else {
            Err(QueueError::ItemNotFound(item_id))
        }
    }

    pub(super) fn update_stats(&mut self) {
        self.stats.total_items = self.items.len()
            + self.processing_items.len()
            + self.completed_items.len()
            + self.failed_items.len();
        self.stats.pending_items = self.items.len();
        self.stats.processing_items = self.processing_items.len();
        self.stats.completed_items = self.completed_items.len();
        self.stats.failed_items = self.failed_items.len();

        self.stats.oldest_item_age_seconds = self
            .items
            .iter()
            .map(|item| item.message.age_seconds())
            .max();
    }

    pub(super) fn cleanup_expired(&mut self) {
        let now = Utc::now();

        self.items.retain(|item| !item.is_expired());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants_display() {
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
}
