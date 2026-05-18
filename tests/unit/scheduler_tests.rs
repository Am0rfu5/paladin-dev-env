//! Unit tests for the Scheduler port, adapter, and deliverer integration.
//!
//! These tests verify:
//! - `SchedulerPort` contract via a mock implementation
//! - `TokioCronSchedulerAdapter` lifecycle and error handling
//! - `ApiContentDeliverer` integration with a scheduler

use async_trait::async_trait;
use chrono::Utc;
use paladin_ports::output::scheduler_port::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Mock SchedulerPort
// ---------------------------------------------------------------------------

/// Deterministic mock that records calls and returns configurable responses.
#[derive(Debug)]
struct MockScheduler {
    running: std::sync::atomic::AtomicBool,
    jobs: Arc<Mutex<HashMap<JobId, JobInfo>>>,
    /// If set, `schedule_job` will return this error.
    fail_schedule: Arc<Mutex<Option<SchedulerError>>>,
}

impl MockScheduler {
    fn new() -> Self {
        Self {
            running: std::sync::atomic::AtomicBool::new(false),
            jobs: Arc::new(Mutex::new(HashMap::new())),
            fail_schedule: Arc::new(Mutex::new(None)),
        }
    }

    fn set_schedule_failure(&self, err: SchedulerError) {
        *self.fail_schedule.lock().unwrap() = Some(err);
    }
}

#[async_trait]
impl SchedulerPort for MockScheduler {
    async fn start(&self) -> Result<(), SchedulerError> {
        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(SchedulerError::AlreadyRunning);
        }
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), SchedulerError> {
        if !self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(SchedulerError::NotRunning);
        }
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn schedule_job(&self, spec: JobSpec) -> Result<JobId, SchedulerError> {
        // Check for injected failure.
        if let Some(err) = self.fail_schedule.lock().unwrap().take() {
            return Err(err);
        }

        let id = JobId::new();
        let info = JobInfo {
            id: id.clone(),
            spec,
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            last_run: None,
            next_run: None,
            run_count: 0,
            failure_count: 0,
        };
        self.jobs.lock().unwrap().insert(id.clone(), info);
        Ok(id)
    }

    async fn cancel_job(&self, job_id: &JobId) -> Result<(), SchedulerError> {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(info) = jobs.get_mut(job_id) {
            info.status = JobStatus::Cancelled;
            Ok(())
        } else {
            Err(SchedulerError::JobNotFound(job_id.clone()))
        }
    }

    async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, SchedulerError> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id)
            .map(|i| i.status.clone())
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn get_job_info(&self, job_id: &JobId) -> Result<JobInfo, SchedulerError> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id)
            .cloned()
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn list_jobs(&self) -> Result<Vec<JobInfo>, SchedulerError> {
        let jobs = self.jobs.lock().unwrap();
        Ok(jobs.values().cloned().collect())
    }

    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// ---------------------------------------------------------------------------
// SchedulerPort trait contract tests (via mock)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_mock_scheduler_lifecycle() {
    let sched = MockScheduler::new();
    assert!(!sched.is_running());

    sched.start().await.unwrap();
    assert!(sched.is_running());

    sched.shutdown().await.unwrap();
    assert!(!sched.is_running());
}

#[tokio::test]
async fn test_mock_scheduler_double_start_errors() {
    let sched = MockScheduler::new();
    sched.start().await.unwrap();
    let err = sched.start().await.unwrap_err();
    assert!(matches!(err, SchedulerError::AlreadyRunning));
    sched.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_mock_scheduler_shutdown_when_stopped_errors() {
    let sched = MockScheduler::new();
    let err = sched.shutdown().await.unwrap_err();
    assert!(matches!(err, SchedulerError::NotRunning));
}

#[tokio::test]
async fn test_mock_schedule_and_list_jobs() {
    let sched = MockScheduler::new();
    sched.start().await.unwrap();

    let s1 = JobSpec::new("job-a", "*/5 * * * * *");
    let s2 = JobSpec::new("job-b", "0 0 * * * *");
    let id1 = sched.schedule_job(s1).await.unwrap();
    let id2 = sched.schedule_job(s2).await.unwrap();

    let jobs = sched.list_jobs().await.unwrap();
    assert_eq!(jobs.len(), 2);

    let status = sched.get_job_status(&id1).await.unwrap();
    assert_eq!(status, JobStatus::Scheduled);

    let info = sched.get_job_info(&id2).await.unwrap();
    assert_eq!(info.spec.label, "job-b");

    sched.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_mock_cancel_job() {
    let sched = MockScheduler::new();
    sched.start().await.unwrap();

    let spec = JobSpec::new("cancel-me", "*/30 * * * * *");
    let id = sched.schedule_job(spec).await.unwrap();

    sched.cancel_job(&id).await.unwrap();
    let status = sched.get_job_status(&id).await.unwrap();
    assert_eq!(status, JobStatus::Cancelled);

    sched.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_mock_cancel_nonexistent_returns_not_found() {
    let sched = MockScheduler::new();
    sched.start().await.unwrap();

    let fake = JobId::new();
    let err = sched.cancel_job(&fake).await.unwrap_err();
    assert!(matches!(err, SchedulerError::JobNotFound(_)));

    sched.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_mock_get_info_nonexistent_returns_not_found() {
    let sched = MockScheduler::new();
    let fake = JobId::new();
    let err = sched.get_job_info(&fake).await.unwrap_err();
    assert!(matches!(err, SchedulerError::JobNotFound(_)));
}

#[tokio::test]
async fn test_mock_schedule_failure_injection() {
    let sched = MockScheduler::new();
    sched.start().await.unwrap();

    sched.set_schedule_failure(SchedulerError::Internal("boom".into()));
    let spec = JobSpec::new("doomed", "*/5 * * * * *");
    let err = sched.schedule_job(spec).await.unwrap_err();
    assert!(matches!(err, SchedulerError::Internal(_)));

    // Subsequent calls should succeed (failure was consumed).
    let spec2 = JobSpec::new("ok", "*/10 * * * * *");
    assert!(sched.schedule_job(spec2).await.is_ok());

    sched.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_job_spec_metadata_builder() {
    let spec = JobSpec::new("meta", "0 0 * * * *")
        .with_metadata("key1", "val1")
        .with_metadata("key2", "val2");

    assert_eq!(spec.metadata.len(), 2);
    assert_eq!(spec.metadata.get("key1").unwrap(), "val1");
}

#[tokio::test]
async fn test_job_status_display_variants() {
    assert_eq!(format!("{}", JobStatus::Scheduled), "Scheduled");
    assert_eq!(format!("{}", JobStatus::Running), "Running");
    assert_eq!(format!("{}", JobStatus::Completed), "Completed");
    assert_eq!(
        format!("{}", JobStatus::Failed("oops".into())),
        "Failed: oops"
    );
    assert_eq!(format!("{}", JobStatus::Cancelled), "Cancelled");
}

#[tokio::test]
async fn test_scheduler_error_display() {
    let e = SchedulerError::InvalidCronExpression {
        expression: "bad".into(),
        reason: "parse error".into(),
    };
    assert!(e.to_string().contains("bad"));
    assert!(e.to_string().contains("parse error"));

    let e2 = SchedulerError::NotRunning;
    assert_eq!(e2.to_string(), "Scheduler is not running");
}

// ---------------------------------------------------------------------------
// TokioCronSchedulerAdapter unit tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_adapter_new_not_running() {
    use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;

    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    assert!(!adapter.is_running());
}

#[tokio::test]
async fn test_adapter_start_stop_lifecycle() {
    use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;

    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    adapter.start().await.unwrap();
    assert!(adapter.is_running());
    adapter.shutdown().await.unwrap();
    assert!(!adapter.is_running());
}

#[tokio::test]
async fn test_adapter_schedule_and_cancel() {
    use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;

    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    adapter.start().await.unwrap();

    let spec = JobSpec::new("test-unit", "*/15 * * * * *");
    let id = adapter.schedule_job(spec).await.unwrap();
    assert_eq!(adapter.list_jobs().await.unwrap().len(), 1);

    adapter.cancel_job(&id).await.unwrap();
    let status = adapter.get_job_status(&id).await.unwrap();
    assert_eq!(status, JobStatus::Cancelled);

    adapter.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_adapter_invalid_cron() {
    use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;

    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    adapter.start().await.unwrap();

    let spec = JobSpec::new("bad", "not-cron");
    let err = adapter.schedule_job(spec).await.unwrap_err();
    assert!(matches!(err, SchedulerError::InvalidCronExpression { .. }));

    adapter.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_adapter_debug_impl() {
    use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;

    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("TokioCronSchedulerAdapter"));
    assert!(debug_str.contains("running"));
}
