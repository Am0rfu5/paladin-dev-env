//! Configuration for the Scheduler subsystem.

use serde::{Deserialize, Serialize};

/// Configuration for the job scheduler system.
///
/// Controls whether scheduled job execution is enabled and the
/// cron scheduler's runtime parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Enable or disable the scheduler subsystem.
    pub enabled: bool,
    /// Default cron expression for scheduled deliveries when none is
    /// provided in the request metadata (6-field: sec min hour day month weekday).
    pub default_cron: String,
    /// Internal channel buffer size for the tokio-cron-scheduler.
    /// Higher values reduce back-pressure at the cost of memory.
    pub channel_size: usize,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default_cron: "0 0 * * * *".to_string(),
            channel_size: 200,
        }
    }
}

impl SchedulerConfig {
    /// Validates scheduler configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled && self.default_cron.trim().is_empty() {
            return Err("default_cron must not be empty when scheduler is enabled".to_string());
        }
        if self.channel_size == 0 {
            return Err("channel_size must be greater than 0".to_string());
        }
        Ok(())
    }
}
