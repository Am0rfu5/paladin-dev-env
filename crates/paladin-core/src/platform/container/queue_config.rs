//! Queue configuration types.
use crate::platform::container::queue_item::QueueItemConfig;
use serde::{Deserialize, Serialize};

/// Configuration for a workflow queue.
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
            cleanup_interval_seconds: 300,
            priority_based: true,
        }
    }
}

/// Statistics snapshot for a single named queue.
///
/// Plain data type with `Serialize`/`Deserialize` — suitable for core container layer.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueueStats {
    /// Queue name
    pub name: String,
    /// Total items ever enqueued
    pub total_items: usize,
    /// Items waiting for processing
    pub pending_items: usize,
    /// Items currently being processed
    pub processing_items: usize,
    /// Items successfully completed
    pub completed_items: usize,
    /// Items that failed processing
    pub failed_items: usize,
    /// Items abandoned (expired / max retries exceeded)
    pub abandoned_items: usize,
    /// Age of the oldest pending item in seconds
    pub oldest_item_age_seconds: Option<i64>,
    /// Average processing time across completed items
    pub average_processing_time_ms: Option<u64>,
    /// Throughput (completed items per minute)
    pub throughput_per_minute: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_stats_default_fields() {
        let stats = QueueStats {
            name: "test".to_string(),
            total_items: 0,
            pending_items: 0,
            processing_items: 0,
            completed_items: 0,
            failed_items: 0,
            abandoned_items: 0,
            oldest_item_age_seconds: None,
            average_processing_time_ms: None,
            throughput_per_minute: 0.0,
        };
        assert_eq!(stats.name, "test");
        assert_eq!(stats.total_items, 0);
    }

    #[test]
    fn test_queue_stats_serde_round_trip() {
        let stats = QueueStats {
            name: "my-queue".to_string(),
            total_items: 100,
            pending_items: 10,
            processing_items: 5,
            completed_items: 80,
            failed_items: 5,
            abandoned_items: 0,
            oldest_item_age_seconds: Some(30),
            average_processing_time_ms: Some(250),
            throughput_per_minute: 4.5,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let restored: QueueStats = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, stats.name);
        assert_eq!(restored.total_items, stats.total_items);
        assert_eq!(restored.throughput_per_minute, stats.throughput_per_minute);
    }

    #[test]
    fn test_queue_config_default() {
        let config = QueueConfig::default();
        assert_eq!(config.max_capacity, 10000);
        assert!(config.priority_based);
        assert!(!config.preserve_completed);
        assert!(config.preserve_failed);
    }
}
