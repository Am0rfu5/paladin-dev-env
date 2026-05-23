//! Configuration for the Redis queue system.

use crate::config::env_utils::EnvOverridable;
use serde::{Deserialize, Serialize};

/// Configuration for Redis queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: Option<String>,
    pub redis_db: u8,
    pub connection_timeout: Option<u64>,
    pub key_prefix: Option<String>,
    pub max_retries: Option<u32>,
    pub enable_priority_queues: Option<bool>,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            redis_host: "localhost".to_string(),
            redis_port: 6379,
            redis_password: None,
            redis_db: 0,
            connection_timeout: Some(30),
            key_prefix: Some("paladin:queue".to_string()),
            max_retries: Some(3),
            enable_priority_queues: Some(true),
        }
    }
}

impl EnvOverridable for QueueConfig {
    fn apply_env_overrides(&mut self) {
        if let Ok(v) = std::env::var("APP_REDIS_HOST") {
            self.redis_host = v;
        }
        if let Ok(v) = std::env::var("APP_REDIS_PORT")
            && let Ok(p) = v.parse::<u16>()
        {
            self.redis_port = p;
        }
        if let Ok(v) = std::env::var("APP_REDIS_PASSWORD") {
            self.redis_password = Some(v);
        }
        if let Ok(v) = std::env::var("APP_REDIS_DB")
            && let Ok(d) = v.parse::<u8>()
        {
            self.redis_db = d;
        }
        if let Ok(v) = std::env::var("APP_REDIS_CONNECTION_TIMEOUT")
            && let Ok(t) = v.parse::<u64>()
        {
            self.connection_timeout = Some(t);
        }
        if let Ok(v) = std::env::var("APP_REDIS_KEY_PREFIX") {
            self.key_prefix = Some(v);
        }
        if let Ok(v) = std::env::var("APP_REDIS_MAX_RETRIES")
            && let Ok(r) = v.parse::<u32>()
        {
            self.max_retries = Some(r);
        }
        if let Ok(v) = std::env::var("APP_REDIS_ENABLE_PRIORITY_QUEUES")
            && let Ok(b) = v.parse::<bool>()
        {
            self.enable_priority_queues = Some(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_config_compatibility() {
        let config = QueueConfig::default();
        assert_eq!(config.redis_host, "localhost");
        assert_eq!(config.redis_port, 6379);
        assert_eq!(config.redis_db, 0);
    }
}
