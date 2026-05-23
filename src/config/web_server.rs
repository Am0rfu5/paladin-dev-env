//! Configuration for web server, sources, and message service.

use serde::{Deserialize, Serialize};

/// Configuration for a content source
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: String,
    pub url: String,
    pub prompt: String,
    pub tags: Vec<String>,
}

/// Configuration for the HTTP server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Configuration for the message service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageServiceSettings {
    pub max_queue_size: Option<usize>,
    pub default_ttl_seconds: Option<i64>,
    pub enable_persistence: Option<bool>,
    pub worker_threads: Option<usize>,
    pub retry_attempts: Option<u32>,
    pub retry_delay_ms: Option<u64>,
}
