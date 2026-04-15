//! Integration test for scheduler adapter and deliverer integration.
//!
//! Verifies end-to-end behaviour of `TokioCronSchedulerAdapter` including:
//! - Full lifecycle (start → schedule → cancel → shutdown)
//! - Integration with `ApiContentDeliverer` via `with_scheduler()`
//! - Multiple concurrent jobs

#![cfg(feature = "web-server")]

use async_trait::async_trait;
use chrono::Utc;
use paladin::application::ports::output::content_delivery_port::{
    ContentDeliveryService, ContentPayload, DeliveryMethod, DeliveryPriority, DeliveryRequest,
    DeliveryStatus,
};
use paladin::application::ports::output::scheduler_port::*;
use paladin::core::platform::container::content::{ContentItem, ContentType, TextContent};
use paladin::infrastructure::adapters::output::api_content_deliverer::ApiContentDeliverer;
use paladin::infrastructure::adapters::scheduling::tokio_cron_adapter::TokioCronSchedulerAdapter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Mock SchedulerPort for deliverer integration tests
// ---------------------------------------------------------------------------

/// A simple mock that records schedule/cancel calls without any tokio
/// runtime dependency, so it won't deadlock when called from the sync
/// `schedule_delivery` / `cancel_delivery` bridge.
#[derive(Debug)]
struct MockScheduler {
    running: std::sync::atomic::AtomicBool,
    jobs: Mutex<HashMap<JobId, JobInfo>>,
}

impl MockScheduler {
    fn new_started() -> Self {
        Self {
            running: std::sync::atomic::AtomicBool::new(true),
            jobs: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SchedulerPort for MockScheduler {
    async fn start(&self) -> Result<(), SchedulerError> {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), SchedulerError> {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn schedule_job(&self, spec: JobSpec) -> Result<JobId, SchedulerError> {
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
        self.jobs
            .lock()
            .unwrap()
            .get(job_id)
            .map(|i| i.status.clone())
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn get_job_info(&self, job_id: &JobId) -> Result<JobInfo, SchedulerError> {
        self.jobs
            .lock()
            .unwrap()
            .get(job_id)
            .cloned()
            .ok_or_else(|| SchedulerError::JobNotFound(job_id.clone()))
    }

    async fn list_jobs(&self) -> Result<Vec<JobInfo>, SchedulerError> {
        Ok(self.jobs.lock().unwrap().values().cloned().collect())
    }

    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// ---------------------------------------------------------------------------
// Adapter-level integration tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_scheduler_full_lifecycle() {
    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();

    // Start
    adapter.start().await.unwrap();
    assert!(adapter.is_running());

    // Schedule several jobs
    let ids: Vec<_> = futures::future::join_all((0..5).map(|i| {
        let spec = JobSpec::new(format!("job-{i}"), format!("*/{} * * * * *", 10 + i));
        adapter.schedule_job(spec)
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    assert_eq!(ids.len(), 5);

    // List
    let all = adapter.list_jobs().await.unwrap();
    assert_eq!(all.len(), 5);

    // Cancel the first two
    adapter.cancel_job(&ids[0]).await.unwrap();
    adapter.cancel_job(&ids[1]).await.unwrap();

    let s0 = adapter.get_job_status(&ids[0]).await.unwrap();
    let s1 = adapter.get_job_status(&ids[1]).await.unwrap();
    let s2 = adapter.get_job_status(&ids[2]).await.unwrap();
    assert_eq!(s0, JobStatus::Cancelled);
    assert_eq!(s1, JobStatus::Cancelled);
    assert_eq!(s2, JobStatus::Scheduled);

    // Shutdown
    adapter.shutdown().await.unwrap();
    assert!(!adapter.is_running());
}

#[tokio::test]
async fn test_scheduler_job_info_persists_metadata() {
    let adapter = TokioCronSchedulerAdapter::new().await.unwrap();
    adapter.start().await.unwrap();

    let spec = JobSpec::new("meta-test", "0 0 * * * *")
        .with_metadata("delivery_id", "d-123")
        .with_metadata("region", "us-west-2");

    let id = adapter.schedule_job(spec).await.unwrap();
    let info = adapter.get_job_info(&id).await.unwrap();

    assert_eq!(info.spec.label, "meta-test");
    assert_eq!(info.spec.metadata.get("delivery_id").unwrap(), "d-123");
    assert_eq!(info.spec.metadata.get("region").unwrap(), "us-west-2");
    assert_eq!(info.run_count, 0);
    assert!(info.last_run.is_none());

    adapter.shutdown().await.unwrap();
}

// ---------------------------------------------------------------------------
// Deliverer + Scheduler integration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_deliverer_schedule_with_scheduler() {
    let mock = Arc::new(MockScheduler::new_started());

    let deliverer =
        ApiContentDeliverer::new().with_scheduler(mock.clone() as Arc<dyn SchedulerPort>);

    let text = TextContent::new(None, Some("Hello scheduler".to_string())).unwrap();
    let item = ContentItem::new(ContentType::Text(text)).unwrap();

    let request = DeliveryRequest {
        recipient_id: "user-42".to_string(),
        delivery_method: DeliveryMethod::Http {
            endpoint: "https://example.com/hook".to_string(),
            headers: None,
        },
        content_payload: ContentPayload::SingleItem(item),
        priority: DeliveryPriority::Normal,
        scheduled_time: None,
        metadata: Some({
            let mut m = std::collections::HashMap::new();
            m.insert(
                "cron_schedule".to_string(),
                serde_json::Value::String("0 0 9 * * *".to_string()),
            );
            m
        }),
    };

    let response = deliverer.schedule_delivery(request).unwrap();
    assert_eq!(response.status, DeliveryStatus::Scheduled);

    // The mock scheduler should now have one job
    let jobs = mock.list_jobs().await.unwrap();
    assert_eq!(jobs.len(), 1);
    assert!(jobs[0].spec.label.starts_with("delivery-"));
}

#[tokio::test]
async fn test_deliverer_cancel_with_scheduler() {
    let mock = Arc::new(MockScheduler::new_started());

    let deliverer =
        ApiContentDeliverer::new().with_scheduler(mock.clone() as Arc<dyn SchedulerPort>);

    let text = TextContent::new(None, Some("Cancel me".to_string())).unwrap();
    let item = ContentItem::new(ContentType::Text(text)).unwrap();

    let request = DeliveryRequest {
        recipient_id: "user-99".to_string(),
        delivery_method: DeliveryMethod::Http {
            endpoint: "https://example.com/hook".to_string(),
            headers: None,
        },
        content_payload: ContentPayload::SingleItem(item),
        priority: DeliveryPriority::Normal,
        scheduled_time: None,
        metadata: None,
    };

    let response = deliverer.schedule_delivery(request).unwrap();
    let delivery_id = response.delivery_id;

    // Cancel via the deliverer
    deliverer.cancel_delivery(delivery_id).unwrap();

    // The underlying mock scheduler job should be cancelled
    let jobs = mock.list_jobs().await.unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].status, JobStatus::Cancelled);
}

#[tokio::test]
async fn test_deliverer_without_scheduler_still_works() {
    // Ensure backward compatibility: no scheduler = same stub behaviour
    let deliverer = ApiContentDeliverer::new();

    let text = TextContent::new(None, Some("No scheduler".to_string())).unwrap();
    let item = ContentItem::new(ContentType::Text(text)).unwrap();

    let request = DeliveryRequest {
        recipient_id: "user-1".to_string(),
        delivery_method: DeliveryMethod::Http {
            endpoint: "https://example.com/hook".to_string(),
            headers: None,
        },
        content_payload: ContentPayload::SingleItem(item),
        priority: DeliveryPriority::Low,
        scheduled_time: None,
        metadata: None,
    };

    let response = deliverer.schedule_delivery(request).unwrap();
    assert_eq!(response.status, DeliveryStatus::Scheduled);

    // Cancel should work without scheduler too
    deliverer.cancel_delivery(response.delivery_id).unwrap();
}
