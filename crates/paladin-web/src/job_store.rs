//! In-memory async job store for fire-and-poll agent execution (Milestone 12, Epic 3).
//!
//! `POST /agents/{id}/jobs` enqueues a job (a spawned task running the agent) and returns
//! a job id immediately; `GET /agents/{id}/jobs/{job_id}` polls its status/result. Jobs
//! are **ephemeral** (lost on restart) and bounded — the store keeps at most `capacity`
//! records, evicting the oldest (and logging the eviction). Durable / distributed jobs
//! are the [queue/worker topology](../../../docs/src/deployment-topologies/queue-worker.md),
//! out of scope here.

use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

/// Lifecycle status of an async job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// The job is executing.
    Running,
    /// The job finished successfully (see `result`).
    Completed,
    /// The job failed (see `error`).
    Failed,
    /// The job exceeded its timeout and was cancelled.
    TimedOut,
}

/// A stored job: its status plus, when finished, a result or error.
///
/// `result` is the serialized success payload (an `ExecuteResponse` as JSON), kept as a
/// [`Value`] so the store stays decoupled from the controller's wire types.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct JobRecord {
    /// Current status.
    pub status: JobStatus,
    /// Success payload (an `ExecuteResponse` as JSON), present once `status == Completed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object, nullable)]
    pub result: Option<Value>,
    /// Error message, present once `status == Failed`/`TimedOut`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct Inner {
    jobs: HashMap<String, JobRecord>,
    order: VecDeque<String>,
}

/// A thread-safe, bounded, in-memory store of async jobs keyed by job id.
pub struct JobStore {
    inner: RwLock<Inner>,
    capacity: usize,
}

impl JobStore {
    /// Create a store retaining at most `capacity` jobs (minimum 1).
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(Inner {
                jobs: HashMap::new(),
                order: VecDeque::new(),
            }),
            capacity: capacity.max(1),
        }
    }

    /// Create a new `Running` job and return its id. Evicts the oldest job if the store
    /// is at capacity.
    pub fn create(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut inner = self.inner.write().unwrap_or_else(|e| e.into_inner());
        inner.jobs.insert(
            id.clone(),
            JobRecord {
                status: JobStatus::Running,
                result: None,
                error: None,
            },
        );
        inner.order.push_back(id.clone());
        while inner.order.len() > self.capacity {
            if let Some(evicted) = inner.order.pop_front() {
                inner.jobs.remove(&evicted);
                log::debug!("job store at capacity; evicted oldest job {evicted}");
            }
        }
        id
    }

    /// Mark a job completed with its result payload. No-op if the job is gone (evicted).
    pub fn complete(&self, id: &str, result: Value) {
        self.update(id, JobStatus::Completed, Some(result), None);
    }

    /// Mark a job failed with an error message.
    pub fn fail(&self, id: &str, error: impl Into<String>) {
        self.update(id, JobStatus::Failed, None, Some(error.into()));
    }

    /// Mark a job timed out.
    pub fn time_out(&self, id: &str, error: impl Into<String>) {
        self.update(id, JobStatus::TimedOut, None, Some(error.into()));
    }

    fn update(&self, id: &str, status: JobStatus, result: Option<Value>, error: Option<String>) {
        let mut inner = self.inner.write().unwrap_or_else(|e| e.into_inner());
        if let Some(record) = inner.jobs.get_mut(id) {
            record.status = status;
            record.result = result;
            record.error = error;
        }
    }

    /// Fetch a cloned job record by id.
    pub fn get(&self, id: &str) -> Option<JobRecord> {
        let inner = self.inner.read().unwrap_or_else(|e| e.into_inner());
        inner.jobs.get(id).cloned()
    }

    /// Number of retained jobs.
    pub fn len(&self) -> usize {
        let inner = self.inner.read().unwrap_or_else(|e| e.into_inner());
        inner.jobs.len()
    }

    /// Whether the store holds no jobs.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for JobStore {
    /// A store retaining the most recent 1024 jobs.
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn create_yields_running_job() {
        let store = JobStore::new(8);
        let id = store.create();
        let record = store.get(&id).expect("job present");
        assert_eq!(record.status, JobStatus::Running);
        assert!(record.result.is_none());
        assert!(record.error.is_none());
    }

    #[test]
    fn complete_fail_and_timeout_transitions() {
        let store = JobStore::new(8);

        let a = store.create();
        store.complete(&a, json!({ "output": "ok" }));
        let ra = store.get(&a).unwrap();
        assert_eq!(ra.status, JobStatus::Completed);
        assert_eq!(ra.result, Some(json!({ "output": "ok" })));

        let b = store.create();
        store.fail(&b, "boom");
        assert_eq!(store.get(&b).unwrap().status, JobStatus::Failed);
        assert_eq!(store.get(&b).unwrap().error.as_deref(), Some("boom"));

        let c = store.create();
        store.time_out(&c, "deadline");
        assert_eq!(store.get(&c).unwrap().status, JobStatus::TimedOut);
    }

    #[test]
    fn unknown_job_is_none() {
        let store = JobStore::new(8);
        assert!(store.get("nope").is_none());
    }

    #[test]
    fn bounded_retention_evicts_oldest() {
        let store = JobStore::new(2);
        let a = store.create();
        let b = store.create();
        let c = store.create(); // evicts `a`

        assert!(store.get(&a).is_none(), "oldest job should be evicted");
        assert!(store.get(&b).is_some());
        assert!(store.get(&c).is_some());
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn update_after_eviction_is_noop() {
        let store = JobStore::new(1);
        let a = store.create();
        let _b = store.create(); // evicts `a`
        store.complete(&a, json!({})); // must not panic or resurrect
        assert!(store.get(&a).is_none());
    }
}
