//! Scheduler Port Module
//!
//! Defines the abstraction for job scheduling operations within the
//! hexagonal architecture. The `SchedulerPort` trait decouples the
//! application layer from specific scheduler implementations
//! (e.g., `tokio-cron-scheduler`), allowing the infrastructure layer
//! to provide concrete adapters.
//!
//! # Architecture
//!
//! ```text
//! Application Layer         Infrastructure Layer
//! ┌──────────────┐         ┌──────────────────────┐
//! │ SchedulerPort│◄────────│ TokioCronScheduler   │
//! │  (trait)     │         │  Adapter              │
//! └──────────────┘         └──────────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! use paladin_ports::output::scheduler_port::*;
//!
//! async fn schedule_example(scheduler: &dyn SchedulerPort) {
//!     let spec = JobSpec::new(
//!         "daily-report",
//!         "0 0 9 * * *", // Every day at 9 AM
//!     );
//!     let job_id = scheduler.schedule_job(spec).await.unwrap();
//!     let status = scheduler.get_job_status(&job_id).await.unwrap();
//!     println!("Job {} status: {:?}", job_id, status);
//! }
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Unique identifier for a scheduled job.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(Uuid);

impl JobId {
    /// Create a new random `JobId`.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a `JobId` from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Return the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for JobId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

// ---------------------------------------------------------------------------
// JobStatus
// ---------------------------------------------------------------------------

/// Represents the lifecycle state of a scheduled job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Job is scheduled but has not yet executed.
    Scheduled,
    /// Job is currently executing.
    Running,
    /// Job completed successfully.
    Completed,
    /// Job failed with an error message.
    Failed(String),
    /// Job was cancelled before execution.
    Cancelled,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scheduled => write!(f, "Scheduled"),
            Self::Running => write!(f, "Running"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed(msg) => write!(f, "Failed: {}", msg),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}

// ---------------------------------------------------------------------------
// JobSpec
// ---------------------------------------------------------------------------

/// Specification for a job to be scheduled.
///
/// Contains the cron schedule expression, an optional human-readable label,
/// and arbitrary metadata that can be used by the job handler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSpec {
    /// Human-readable label for the job.
    pub label: String,

    /// Cron schedule expression (6-field: sec min hour day month weekday).
    ///
    /// Examples:
    /// - `"0 0 9 * * *"` — every day at 09:00:00
    /// - `"0 */5 * * * *"` — every 5 minutes
    /// - `"0 0 0 * * Mon"` — every Monday at midnight
    pub schedule: String,

    /// Arbitrary metadata associated with the job.
    pub metadata: HashMap<String, String>,
}

impl JobSpec {
    /// Create a new `JobSpec` with the given label and cron schedule.
    pub fn new(label: impl Into<String>, schedule: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            schedule: schedule.into(),
            metadata: HashMap::new(),
        }
    }

    /// Add a metadata key-value pair to this spec.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

// ---------------------------------------------------------------------------
// JobInfo
// ---------------------------------------------------------------------------

/// Detailed information about a scheduled job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
    /// Unique job identifier.
    pub id: JobId,

    /// The specification this job was created from.
    pub spec: JobSpec,

    /// Current status of the job.
    pub status: JobStatus,

    /// When the job was created/scheduled.
    pub created_at: DateTime<Utc>,

    /// When the job last ran, if ever.
    pub last_run: Option<DateTime<Utc>>,

    /// Next scheduled execution time, if applicable.
    pub next_run: Option<DateTime<Utc>>,

    /// Number of times this job has executed.
    pub run_count: u32,

    /// Number of consecutive failures.
    pub failure_count: u32,
}

// ---------------------------------------------------------------------------
// SchedulerError
// ---------------------------------------------------------------------------

/// Errors that can occur during scheduler operations.
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    /// The provided cron expression is invalid.
    #[error("Invalid cron expression '{expression}': {reason}")]
    InvalidCronExpression {
        /// The cron expression that failed to parse.
        expression: String,
        /// Reason for the parse failure.
        reason: String,
    },

    /// The specified job was not found.
    #[error("Job not found: {0}")]
    JobNotFound(JobId),

    /// The scheduler has not been started.
    #[error("Scheduler is not running")]
    NotRunning,

    /// The scheduler is already running.
    #[error("Scheduler is already running")]
    AlreadyRunning,

    /// A job execution failed.
    #[error("Job execution failed: {0}")]
    ExecutionFailed(String),

    /// Internal scheduler error.
    #[error("Scheduler internal error: {0}")]
    Internal(String),
}

// ---------------------------------------------------------------------------
// SchedulerPort trait
// ---------------------------------------------------------------------------

/// Port trait for job scheduling operations.
///
/// Implementations provide the mechanism for scheduling, querying, and
/// cancelling cron-based jobs. The trait is `Send + Sync` to support
/// async runtimes and shared state.
///
/// # Examples
///
/// ```rust
/// use paladin_ports::output::scheduler_port::{SchedulerPort, JobSpec, JobStatus, SchedulerError};
///
/// async fn schedule_daily_report(
///     scheduler: &dyn SchedulerPort,
/// ) -> Result<JobStatus, SchedulerError> {
///     let spec = JobSpec::new("daily-report", "0 0 9 * * *")
///         .with_metadata("owner", "reporting-team");
///
///     let job_id = scheduler.schedule_job(spec).await?;
///     scheduler.get_job_status(&job_id).await
/// }
/// ```
#[async_trait]
pub trait SchedulerPort: Send + Sync {
    /// Start the scheduler so that scheduled jobs begin executing.
    async fn start(&self) -> Result<(), SchedulerError>;

    /// Shutdown the scheduler gracefully.
    async fn shutdown(&self) -> Result<(), SchedulerError>;

    /// Schedule a new job according to the given specification.
    ///
    /// Returns the `JobId` assigned to the newly created job.
    async fn schedule_job(&self, spec: JobSpec) -> Result<JobId, SchedulerError>;

    /// Cancel a previously scheduled job.
    ///
    /// Returns `Ok(())` if the job was successfully cancelled, or
    /// `Err(SchedulerError::JobNotFound)` if no such job exists.
    async fn cancel_job(&self, job_id: &JobId) -> Result<(), SchedulerError>;

    /// Retrieve the current status of a job.
    async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, SchedulerError>;

    /// Retrieve detailed information about a job.
    async fn get_job_info(&self, job_id: &JobId) -> Result<JobInfo, SchedulerError>;

    /// List all currently tracked jobs.
    async fn list_jobs(&self) -> Result<Vec<JobInfo>, SchedulerError>;

    /// Check whether the scheduler is currently running.
    fn is_running(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_id_display() {
        let id = JobId::new();
        let display = format!("{}", id);
        assert!(!display.is_empty());
        // UUID v4 format
        assert_eq!(display.len(), 36);
    }

    #[test]
    fn test_job_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = JobId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_job_id_default() {
        let id1 = JobId::default();
        let id2 = JobId::default();
        assert_ne!(id1, id2, "Two defaults should differ");
    }

    #[test]
    fn test_job_status_display() {
        assert_eq!(format!("{}", JobStatus::Scheduled), "Scheduled");
        assert_eq!(format!("{}", JobStatus::Running), "Running");
        assert_eq!(format!("{}", JobStatus::Completed), "Completed");
        assert_eq!(
            format!("{}", JobStatus::Failed("oops".into())),
            "Failed: oops"
        );
        assert_eq!(format!("{}", JobStatus::Cancelled), "Cancelled");
    }

    #[test]
    fn test_job_spec_builder() {
        let spec = JobSpec::new("backup", "0 0 2 * * *")
            .with_metadata("target", "database")
            .with_metadata("retention", "30d");

        assert_eq!(spec.label, "backup");
        assert_eq!(spec.schedule, "0 0 2 * * *");
        assert_eq!(spec.metadata.len(), 2);
        assert_eq!(spec.metadata.get("target").unwrap(), "database");
    }

    #[test]
    fn test_scheduler_error_display() {
        let err = SchedulerError::InvalidCronExpression {
            expression: "bad".into(),
            reason: "parse failure".into(),
        };
        assert!(format!("{}", err).contains("bad"));
        assert!(format!("{}", err).contains("parse failure"));

        let err = SchedulerError::JobNotFound(JobId::new());
        assert!(format!("{}", err).contains("Job not found"));

        let err = SchedulerError::NotRunning;
        assert!(format!("{}", err).contains("not running"));
    }
}
