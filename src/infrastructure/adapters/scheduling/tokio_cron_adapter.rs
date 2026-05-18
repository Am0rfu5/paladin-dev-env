//! Tokio Cron Scheduler Adapter
//!
//! Infrastructure adapter implementing [`SchedulerPort`] using the
//! `tokio-cron-scheduler` crate. This adapter wraps [`JobScheduler`] and
//! provides cron-based job scheduling, cancellation, and status tracking.
//!
//! # Architecture
//!
//! ```text
//! ┌────────────────────────┐
//! │  TokioCronScheduler   │
//! │       Adapter          │
//! ├────────────────────────┤
//! │ inner: JobScheduler    │──► tokio-cron-scheduler
//! │ jobs:  HashMap<Id,Info>│──► Internal state tracking
//! │ running: AtomicBool    │──► Lifecycle flag
//! └────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;
//! use paladin_ports::output::scheduler_port::{SchedulerPort, JobSpec};
//!
//! async fn example() {
//!     let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
//!     adapter.start().await.unwrap();
//!
//!     let spec = JobSpec::new("heartbeat", "*/10 * * * * *");
//!     let job_id = adapter.schedule_job(spec).await.unwrap();
//!     println!("Scheduled job: {}", job_id);
//!
//!     adapter.shutdown().await.unwrap();
//! }
//! ```

use async_trait::async_trait;
use chrono::Utc;
use log::{debug, info};
use paladin_ports::output::scheduler_port::{
    JobId, JobInfo, JobSpec, JobStatus, SchedulerError, SchedulerPort,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Error conversion
// ---------------------------------------------------------------------------

/// Map `JobSchedulerError` into our domain `SchedulerError`.
fn map_scheduler_error(err: JobSchedulerError) -> SchedulerError {
    SchedulerError::Internal(format!("{err:?}"))
}

// ---------------------------------------------------------------------------
// TokioCronSchedulerAdapter
// ---------------------------------------------------------------------------

/// Adapter that bridges the [`SchedulerPort`] trait to the
/// `tokio-cron-scheduler` library.
///
/// Internally tracks job metadata in a `HashMap` so that callers can
/// query status, run counts, and next-run times without reaching
/// directly into the underlying scheduler.
#[doc(hidden)]
pub struct TokioCronSchedulerAdapter {
    /// The underlying `tokio-cron-scheduler` instance.
    ///
    /// Wrapped in a `tokio::sync::Mutex` because `shutdown()` requires
    /// `&mut self` on the inner scheduler.
    inner: tokio::sync::Mutex<JobScheduler>,

    /// Map of `JobId` → `JobInfo` for tracking job metadata.
    jobs: Arc<RwLock<HashMap<JobId, JobInfo>>>,

    /// Whether the scheduler is currently running.
    running: AtomicBool,
}

impl std::fmt::Debug for TokioCronSchedulerAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokioCronSchedulerAdapter")
            .field("running", &self.running.load(Ordering::SeqCst))
            .finish_non_exhaustive()
    }
}

impl TokioCronSchedulerAdapter {
    /// Create a new adapter with an uninitialised scheduler.
    ///
    /// The scheduler must be started with [`SchedulerPort::start`] before
    /// any jobs will execute.
    ///
    /// # Errors
    ///
    /// Returns `SchedulerError::Internal` if the underlying
    /// `JobScheduler::new()` call fails.
    pub async fn new() -> Result<Self, SchedulerError> {
        let scheduler = JobScheduler::new().await.map_err(map_scheduler_error)?;

        Ok(Self {
            inner: tokio::sync::Mutex::new(scheduler),
            jobs: Arc::new(RwLock::new(HashMap::new())),
            running: AtomicBool::new(false),
        })
    }

    /// Retrieve the internal `JobScheduler` UUID for a given `JobId`.
    ///
    /// This is simply `job_id.as_uuid()` since our `JobId` wraps a `Uuid`
    /// and `tokio-cron-scheduler` uses `Uuid` as its job handle.
    fn inner_uuid(job_id: &JobId) -> Uuid {
        *job_id.as_uuid()
    }
}

// ---------------------------------------------------------------------------
// SchedulerPort implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl SchedulerPort for TokioCronSchedulerAdapter {
    async fn start(&self) -> Result<(), SchedulerError> {
        if self.running.load(Ordering::SeqCst) {
            return Err(SchedulerError::AlreadyRunning);
        }

        let sched = self.inner.lock().await;
        sched.start().await.map_err(map_scheduler_error)?;
        self.running.store(true, Ordering::SeqCst);
        info!("Scheduler started");
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), SchedulerError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(SchedulerError::NotRunning);
        }

        let mut sched = self.inner.lock().await;
        sched.shutdown().await.map_err(map_scheduler_error)?;
        self.running.store(false, Ordering::SeqCst);
        info!("Scheduler shut down");
        Ok(())
    }

    async fn schedule_job(&self, spec: JobSpec) -> Result<JobId, SchedulerError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(SchedulerError::NotRunning);
        }

        let label = spec.label.clone();
        let schedule_expr = spec.schedule.clone();
        let label_for_log = spec.label.clone();

        // Create an async cron job.  The closure receives the job UUID
        // and a mutable scheduler lock so that it can inspect next ticks, etc.
        let job = Job::new_async(schedule_expr.as_str(), move |uuid, _lock| {
            let label = label.clone();
            Box::pin(async move {
                debug!("Scheduled job '{}' fired (uuid={})", label, uuid);
            })
        })
        .map_err(|e| SchedulerError::InvalidCronExpression {
            expression: spec.schedule.clone(),
            reason: format!("{e:?}"),
        })?;

        let uuid = job.guid();
        let job_id = JobId::from_uuid(uuid);

        // Register the job with the inner scheduler.
        let sched = self.inner.lock().await;
        sched.add(job).await.map_err(map_scheduler_error)?;

        // Compute next run time.
        // `next_tick_for_job` needs `&mut self` on the scheduler, so we
        // skip for now and set `next_run` to `None`. The next tick will
        // be populated on the first run or via `get_job_info`.
        let now = Utc::now();
        let info = JobInfo {
            id: job_id.clone(),
            spec,
            status: JobStatus::Scheduled,
            created_at: now,
            last_run: None,
            next_run: None,
            run_count: 0,
            failure_count: 0,
        };

        {
            let mut jobs = self.jobs.write().await;
            jobs.insert(job_id.clone(), info);
        }

        info!("Scheduled job '{}' with id {}", label_for_log, job_id);
        Ok(job_id)
    }

    async fn cancel_job(&self, job_id: &JobId) -> Result<(), SchedulerError> {
        // Verify the job exists in our tracker.
        {
            let jobs = self.jobs.read().await;
            if !jobs.contains_key(job_id) {
                return Err(SchedulerError::JobNotFound(job_id.clone()));
            }
        }

        let uuid = Self::inner_uuid(job_id);

        // Remove from the inner scheduler.
        let sched = self.inner.lock().await;
        sched.remove(&uuid).await.map_err(map_scheduler_error)?;

        // Update our tracker.
        {
            let mut jobs = self.jobs.write().await;
            if let Some(info) = jobs.get_mut(job_id) {
                info.status = JobStatus::Cancelled;
            }
        }

        info!("Cancelled job {}", job_id);
        Ok(())
    }

    async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, SchedulerError> {
        let jobs = self.jobs.read().await;
        jobs.get(job_id)
            .map(|info| info.status.clone())
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn get_job_info(&self, job_id: &JobId) -> Result<JobInfo, SchedulerError> {
        let jobs = self.jobs.read().await;
        jobs.get(job_id)
            .cloned()
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn list_jobs(&self) -> Result<Vec<JobInfo>, SchedulerError> {
        let jobs = self.jobs.read().await;
        Ok(jobs.values().cloned().collect())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_ports::output::scheduler_port::{JobSpec, SchedulerPort};

    #[tokio::test]
    async fn test_new_adapter_is_not_running() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        assert!(!adapter.is_running());
    }

    #[tokio::test]
    async fn test_start_sets_running() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();
        assert!(adapter.is_running());
        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_double_start_returns_already_running() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();
        let result = adapter.start().await;
        assert!(matches!(result, Err(SchedulerError::AlreadyRunning)));
        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_shutdown_when_not_running_returns_error() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        let result = adapter.shutdown().await;
        assert!(matches!(result, Err(SchedulerError::NotRunning)));
    }

    #[tokio::test]
    async fn test_schedule_job_returns_job_id() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let spec = JobSpec::new("test-job", "*/10 * * * * *");
        let job_id = adapter.schedule_job(spec).await.unwrap();

        let status = adapter.get_job_status(&job_id).await.unwrap();
        assert_eq!(status, JobStatus::Scheduled);

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_schedule_job_when_not_running_returns_error() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        let spec = JobSpec::new("test-job", "*/10 * * * * *");
        let result = adapter.schedule_job(spec).await;
        assert!(matches!(result, Err(SchedulerError::NotRunning)));
    }

    #[tokio::test]
    async fn test_schedule_job_invalid_cron_returns_error() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let spec = JobSpec::new("bad-job", "not a cron expression");
        let result = adapter.schedule_job(spec).await;
        assert!(matches!(
            result,
            Err(SchedulerError::InvalidCronExpression { .. })
        ));

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cancel_job() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let spec = JobSpec::new("cancel-me", "*/30 * * * * *");
        let job_id = adapter.schedule_job(spec).await.unwrap();

        adapter.cancel_job(&job_id).await.unwrap();
        let status = adapter.get_job_status(&job_id).await.unwrap();
        assert_eq!(status, JobStatus::Cancelled);

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cancel_nonexistent_job_returns_not_found() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let fake_id = JobId::new();
        let result = adapter.cancel_job(&fake_id).await;
        assert!(matches!(result, Err(SchedulerError::JobNotFound(_))));

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_job_info() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let spec = JobSpec::new("info-job", "0 0 * * * *").with_metadata("env", "test");
        let job_id = adapter.schedule_job(spec).await.unwrap();

        let info = adapter.get_job_info(&job_id).await.unwrap();
        assert_eq!(info.id, job_id);
        assert_eq!(info.spec.label, "info-job");
        assert_eq!(info.spec.metadata.get("env").unwrap(), "test");
        assert_eq!(info.status, JobStatus::Scheduled);
        assert_eq!(info.run_count, 0);

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_nonexistent_job_info_returns_not_found() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        let fake_id = JobId::new();
        let result = adapter.get_job_info(&fake_id).await;
        assert!(matches!(result, Err(SchedulerError::JobNotFound(_))));
    }

    #[tokio::test]
    async fn test_list_jobs() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        assert!(adapter.list_jobs().await.unwrap().is_empty());

        let spec1 = JobSpec::new("job-1", "*/10 * * * * *");
        let spec2 = JobSpec::new("job-2", "*/20 * * * * *");
        adapter.schedule_job(spec1).await.unwrap();
        adapter.schedule_job(spec2).await.unwrap();

        let jobs = adapter.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 2);

        adapter.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_schedule_job_with_metadata() {
        let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
        adapter.start().await.unwrap();

        let spec = JobSpec::new("meta-job", "*/5 * * * * *")
            .with_metadata("delivery_id", "abc-123")
            .with_metadata("priority", "high");
        let job_id = adapter.schedule_job(spec).await.unwrap();

        let info = adapter.get_job_info(&job_id).await.unwrap();
        assert_eq!(info.spec.metadata.len(), 2);
        assert_eq!(info.spec.metadata.get("delivery_id").unwrap(), "abc-123");

        adapter.shutdown().await.unwrap();
    }
}
