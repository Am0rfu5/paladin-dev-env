//! Orchestration context for workflow sessions.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Orchestrator execution context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationContext {
    /// Unique identifier for the orchestration session
    pub session_id: Uuid,
    /// User or system that initiated the orchestration
    pub initiator: String,
    /// Environment context (dev, staging, prod)
    pub environment: String,
    /// Correlation ID for tracking related operations
    pub correlation_id: Option<Uuid>,
    /// Session metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Session start time
    pub started_at: DateTime<Utc>,
}

impl OrchestrationContext {
    /// Create a new orchestration context.
    pub fn new(initiator: String, environment: String) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            initiator,
            environment,
            correlation_id: None,
            metadata: HashMap::new(),
            started_at: Utc::now(),
        }
    }

    /// Set the correlation ID.
    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Add typed metadata to the context.
    pub fn add_metadata<T: Serialize>(
        &mut self,
        key: String,
        value: T,
    ) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.metadata.insert(key, json_value);
        Ok(())
    }
}
