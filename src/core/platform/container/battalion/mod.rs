//! Battalion Orchestration Base Types
//!
//! This module provides the core domain types for multi-Paladin orchestration.
//! Battalions coordinate multiple Paladins using various execution patterns.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

// Note: PaladinResult is defined in application layer, but we reference it here
// for the domain result type. This is acceptable as Battalion results contain
// references to execution outcomes.
use crate::application::ports::output::paladin_port::PaladinResult;

/// Configuration for Battalion operations
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy};
///
/// let config = BattalionConfig::new("research_battalion")
///     .with_timeout(300)
///     .with_error_strategy(ErrorStrategy::FailFast);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionConfig {
    /// Name of the Battalion
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Maximum execution time in seconds
    pub timeout_seconds: u64,

    /// Retry policy for failed operations
    pub retry_policy: RetryPolicy,

    /// Strategy for handling errors
    pub error_strategy: ErrorStrategy,

    /// Directory for saving metadata output
    pub metadata_output_dir: Option<PathBuf>,
}

impl BattalionConfig {
    /// Create a new BattalionConfig with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            timeout_seconds: 300,
            retry_policy: RetryPolicy::default(),
            error_strategy: ErrorStrategy::default(),
            metadata_output_dir: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the timeout in seconds
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Set the retry policy
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Set the error strategy
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Set the metadata output directory
    pub fn with_metadata_dir(mut self, dir: PathBuf) -> Self {
        self.metadata_output_dir = Some(dir);
        self
    }
}

impl Default for BattalionConfig {
    fn default() -> Self {
        Self::new("default_battalion")
    }
}

/// Retry policy configuration
///
/// Defines how failed operations should be retried with exponential backoff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,

    /// Base delay between retries
    pub base_delay: Duration,

    /// Maximum delay between retries
    pub max_delay: Duration,

    /// Whether to use exponential backoff
    pub exponential_backoff: bool,

    /// Whether to add jitter to prevent thundering herd
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            exponential_backoff: true,
            jitter: true,
        }
    }
}

/// Strategy for handling errors during Battalion execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ErrorStrategy {
    /// Stop immediately on first error
    #[default]
    FailFast,

    /// Continue execution despite errors, collect all at end
    ContinueOnError,

    /// Retry failed operations, then continue
    RetryThenContinue,
}

/// Current status of a Battalion execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattalionStatus {
    /// Battalion is idle, not yet started
    #[default]
    Idle,

    /// Battalion is currently executing
    Running,

    /// Battalion execution is paused
    Paused,

    /// Battalion completed successfully
    Completed,

    /// Battalion failed with errors
    Failed,

    /// Battalion was cancelled
    Cancelled,
}

/// Result of a Battalion execution
///
/// Contains the final output, individual Paladin results, and execution metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionResult {
    /// Unique identifier for this execution
    pub battalion_id: Uuid,

    /// Name of the Battalion
    pub battalion_name: String,

    /// When execution started
    pub started_at: DateTime<Utc>,

    /// When execution completed
    pub completed_at: DateTime<Utc>,

    /// Final aggregated output
    pub final_output: String,

    /// Individual Paladin results
    pub paladin_results: Vec<PaladinResult>,

    /// Final status
    pub status: BattalionStatus,
}

impl BattalionResult {
    /// Create a new BattalionResult
    pub fn new(
        battalion_id: Uuid,
        battalion_name: String,
        started_at: DateTime<Utc>,
        final_output: String,
        paladin_results: Vec<PaladinResult>,
    ) -> Self {
        Self {
            battalion_id,
            battalion_name,
            started_at,
            completed_at: Utc::now(),
            final_output,
            paladin_results,
            status: BattalionStatus::Completed,
        }
    }

    /// Create from a list of Paladin results (sequential execution)
    pub fn from_paladin_results(
        battalion_id: Uuid,
        battalion_name: String,
        started_at: DateTime<Utc>,
        results: Vec<PaladinResult>,
    ) -> Self {
        let final_output = results.last().map(|r| r.output.clone()).unwrap_or_default();

        Self::new(
            battalion_id,
            battalion_name,
            started_at,
            final_output,
            results,
        )
    }

    /// Get execution duration
    pub fn duration(&self) -> Duration {
        (self.completed_at - self.started_at)
            .to_std()
            .unwrap_or_default()
    }
}

/// Error types for Battalion operations
#[derive(Debug, thiserror::Error)]
pub enum BattalionError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Paladin execution error
    #[error("Paladin error: {0}")]
    PaladinError(String),

    /// Formation-specific error
    #[error("Formation error: {0}")]
    FormationError(String),

    /// Phalanx-specific error
    #[error("Phalanx error: {0}")]
    PhalanxError(String),

    /// Campaign-specific error
    #[error("Campaign error: {0}")]
    CampaignError(String),

    /// Invalid graph structure
    #[error("Invalid graph: {0}")]
    InvalidGraph(String),

    /// Chain of Command error
    #[error("Chain of Command error: {0}")]
    ChainOfCommandError(String),

    /// Timeout error
    #[error("Battalion execution timed out after {0} seconds")]
    Timeout(u64),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Aggregation error
    #[error("Aggregation error: {0}")]
    AggregationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battalion_config_default() {
        let config = BattalionConfig::default();
        assert_eq!(config.name, "default_battalion");
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.error_strategy, ErrorStrategy::FailFast);
    }

    #[test]
    fn test_battalion_config_builder() {
        let config = BattalionConfig::new("test_battalion")
            .with_description("Test description")
            .with_timeout(600)
            .with_error_strategy(ErrorStrategy::ContinueOnError);

        assert_eq!(config.name, "test_battalion");
        assert_eq!(config.description, Some("Test description".to_string()));
        assert_eq!(config.timeout_seconds, 600);
        assert_eq!(config.error_strategy, ErrorStrategy::ContinueOnError);
    }

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.base_delay, Duration::from_millis(100));
        assert_eq!(policy.max_delay, Duration::from_secs(10));
        assert!(policy.exponential_backoff);
        assert!(policy.jitter);
    }

    #[test]
    fn test_error_strategy_variants() {
        let fail_fast = ErrorStrategy::FailFast;
        let continue_on_error = ErrorStrategy::ContinueOnError;
        let retry_then_continue = ErrorStrategy::RetryThenContinue;

        assert_eq!(fail_fast, ErrorStrategy::FailFast);
        assert_eq!(continue_on_error, ErrorStrategy::ContinueOnError);
        assert_eq!(retry_then_continue, ErrorStrategy::RetryThenContinue);
        assert_ne!(fail_fast, continue_on_error);
    }

    #[test]
    fn test_error_strategy_default() {
        let strategy = ErrorStrategy::default();
        assert_eq!(strategy, ErrorStrategy::FailFast);
    }

    #[test]
    fn test_battalion_status_variants() {
        assert_eq!(BattalionStatus::default(), BattalionStatus::Idle);

        let statuses = vec![
            BattalionStatus::Idle,
            BattalionStatus::Running,
            BattalionStatus::Paused,
            BattalionStatus::Completed,
            BattalionStatus::Failed,
            BattalionStatus::Cancelled,
        ];

        for status in statuses {
            // Verify each status can be created
            let _ = status;
        }
    }

    #[test]
    fn test_battalion_result_new() {
        let battalion_id = Uuid::new_v4();
        let started_at = Utc::now();

        let result = BattalionResult::new(
            battalion_id,
            "test_battalion".to_string(),
            started_at,
            "final output".to_string(),
            vec![],
        );

        assert_eq!(result.battalion_id, battalion_id);
        assert_eq!(result.battalion_name, "test_battalion");
        assert_eq!(result.final_output, "final output");
        assert_eq!(result.status, BattalionStatus::Completed);
        assert!(result.paladin_results.is_empty());
    }

    #[test]
    fn test_battalion_result_duration() {
        let battalion_id = Uuid::new_v4();
        let started_at = Utc::now();

        let mut result = BattalionResult::new(
            battalion_id,
            "test_battalion".to_string(),
            started_at,
            "output".to_string(),
            vec![],
        );

        // Set completed_at to 2 seconds after started_at
        result.completed_at = started_at + chrono::Duration::seconds(2);

        let duration = result.duration();
        assert_eq!(duration.as_secs(), 2);
    }

    #[test]
    fn test_battalion_config_serialization() {
        let config = BattalionConfig::new("test").with_timeout(120);

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: BattalionConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    }

    #[test]
    fn test_retry_policy_serialization() {
        let policy = RetryPolicy::default();

        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: RetryPolicy = serde_json::from_str(&json).unwrap();

        assert_eq!(policy.max_attempts, deserialized.max_attempts);
        assert_eq!(policy.exponential_backoff, deserialized.exponential_backoff);
    }

    #[test]
    fn test_error_strategy_serialization() {
        let strategy = ErrorStrategy::ContinueOnError;

        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: ErrorStrategy = serde_json::from_str(&json).unwrap();

        assert_eq!(strategy, deserialized);
    }

    #[test]
    fn test_battalion_status_serialization() {
        let status = BattalionStatus::Running;

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: BattalionStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_battalion_result_serialization() {
        let result = BattalionResult::new(
            Uuid::new_v4(),
            "test".to_string(),
            Utc::now(),
            "output".to_string(),
            vec![],
        );

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: BattalionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.battalion_id, deserialized.battalion_id);
        assert_eq!(result.battalion_name, deserialized.battalion_name);
        assert_eq!(result.final_output, deserialized.final_output);
    }

    #[test]
    fn test_battalion_error_variants() {
        let errors = vec![
            BattalionError::ConfigurationError("test".to_string()),
            BattalionError::PaladinError("test".to_string()),
            BattalionError::FormationError("test".to_string()),
            BattalionError::PhalanxError("test".to_string()),
            BattalionError::CampaignError("test".to_string()),
            BattalionError::InvalidGraph("test".to_string()),
            BattalionError::ChainOfCommandError("test".to_string()),
            BattalionError::Timeout(300),
            BattalionError::ValidationError("test".to_string()),
            BattalionError::AggregationError("test".to_string()),
        ];

        // Verify error messages
        for error in errors {
            let msg = error.to_string();
            assert!(!msg.is_empty());
        }
    }
}
