//! Queue configuration types.
use crate::core::platform::container::queue_item::QueueItemConfig;
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
