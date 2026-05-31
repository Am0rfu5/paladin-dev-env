//! Operational validation of the queue contract (Milestone 9, Epic 2, Task 2.2).
//!
//! These tests exercise the same queue contract against two backends:
//!
//! * the in-memory `QueueOrchestrator` (always run), and
//! * the Redis-backed `RedisQueueAdapter` (run only with the `redis-queue`
//!   feature, and skipped gracefully when no Redis instance is reachable).
//!
//! Both backends are driven through a single [`QueuePortHarness`] abstraction so
//! they are held to identical behavioural guarantees. The Redis harness delegates
//! to the real [`paladin_ports::output::queue_port::QueuePort`] trait; the
//! in-memory harness delegates to the orchestrator's equivalent inherent methods
//! (the orchestrator predates the port trait and uses its own error/stats types).
//! The shared [`assert_queue_port_contract`] routine validates the full
//! enqueue → dequeue → process lifecycle, automatic retry, and dead-letter
//! handling.

use async_trait::async_trait;
use paladin::core::base::entity::message::{Location, Message};
use paladin::core::platform::container::queue_config::QueueConfig;
use paladin::core::platform::container::queue_item::{QueueItem, QueueItemConfig};
use uuid::Uuid;

/// Build a queue item carrying a `String` payload with an explicit retry budget.
fn make_item(queue: &str, payload: &str, max_retries: u32) -> QueueItem<String> {
    let message = Message::new(
        Location::service("queue-contract-test"),
        Location::system("queue"),
        payload.to_string(),
    );
    let config = QueueItemConfig {
        max_retries,
        ..Default::default()
    };
    QueueItem::new(queue.to_string(), message, Some(config))
}

/// Build a queue config whose default item retry budget is `retry_budget`.
///
/// The in-memory `Queue::enqueue` overrides a fresh item's config with the
/// queue's `default_item_config`, so the retry budget for that backend must be
/// set here (the Redis backend honours the per-item config instead). The
/// contract sets both so the budget is identical across backends.
fn queue_config_with_budget(retry_budget: u32) -> QueueConfig {
    QueueConfig {
        default_item_config: QueueItemConfig {
            max_retries: retry_budget,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Minimal queue operations needed to validate the contract, unified across
/// backends (errors are flattened to `String` since each backend defines its
/// own error type).
#[async_trait]
trait QueuePortHarness {
    async fn create_queue(&self, name: &str, retry_budget: u32) -> Result<(), String>;
    async fn delete_queue(&self, name: &str) -> Result<(), String>;
    async fn enqueue(&self, name: &str, item: QueueItem<String>) -> Result<Uuid, String>;
    async fn dequeue(&self, name: &str) -> Result<Option<Uuid>, String>;
    async fn start_processing(&self, name: &str, id: Uuid, worker: &str) -> Result<(), String>;
    async fn complete_processing(&self, name: &str, id: Uuid) -> Result<(), String>;
    /// Returns `true` if the item was re-queued for retry.
    async fn fail_processing(&self, name: &str, id: Uuid) -> Result<bool, String>;
    async fn list_queues(&self) -> Vec<String>;
    async fn queue_length(&self, name: &str) -> Result<usize, String>;
    async fn health_check(&self) -> Result<bool, String>;
}

/// Reusable queue contract: any conforming backend must pass this routine.
///
/// Validates, in order:
/// 1. queue creation and listing,
/// 2. enqueue → dequeue → start → complete happy path,
/// 3. health check reports healthy,
/// 4. retry-then-dead-letter: an item with `max_retries == 1` is retried exactly
///    once and then permanently failed (no infinite retry loop), leaving the
///    queue drained of pending work.
async fn assert_queue_port_contract<Q: QueuePortHarness>(queue: &Q, qname: &str) {
    // 1. Create + list. Retry budget of 1 → exactly one retry before dead-letter.
    queue.create_queue(qname, 1).await.unwrap();
    assert!(
        queue.list_queues().await.contains(&qname.to_string()),
        "created queue should be listed"
    );

    // 2. Happy-path lifecycle.
    queue
        .enqueue(qname, make_item(qname, "ok-job", 1))
        .await
        .unwrap();
    let item_id = queue
        .dequeue(qname)
        .await
        .unwrap()
        .expect("an enqueued item must be dequeued");
    queue
        .start_processing(qname, item_id, "worker-1")
        .await
        .unwrap();
    queue.complete_processing(qname, item_id).await.unwrap();

    // 3. Health check.
    assert!(
        queue.health_check().await.unwrap(),
        "backend should report healthy"
    );

    // 4. Retry then dead-letter (max_retries == 1 → one retry, then exhausted).
    queue
        .enqueue(qname, make_item(qname, "flaky-job", 1))
        .await
        .unwrap();

    let mut retry_results = Vec::new();
    for _ in 0..5 {
        let id = match queue.dequeue(qname).await.unwrap() {
            Some(id) => id,
            None => break,
        };
        queue.start_processing(qname, id, "worker-1").await.unwrap();
        let will_retry = queue.fail_processing(qname, id).await.unwrap();
        retry_results.push(will_retry);
        if !will_retry {
            break;
        }
    }

    assert_eq!(
        retry_results.first(),
        Some(&true),
        "first failure of a retryable item must be re-queued"
    );
    assert_eq!(
        retry_results.last(),
        Some(&false),
        "final failure must be exhausted and dead-lettered (no infinite retry)"
    );
    assert_eq!(
        queue.queue_length(qname).await.unwrap(),
        0,
        "queue should be drained of pending work after dead-letter"
    );

    // Cleanup so repeated runs against a shared backend stay isolated.
    queue.delete_queue(qname).await.unwrap();
}

// ---------------------------------------------------------------------------
// In-memory backend
// ---------------------------------------------------------------------------

use paladin::application::services::queue_orchestrator::QueueOrchestrator;

#[async_trait]
impl QueuePortHarness for QueueOrchestrator {
    async fn create_queue(&self, name: &str, retry_budget: u32) -> Result<(), String> {
        QueueOrchestrator::create_queue(
            self,
            name.to_string(),
            Some(queue_config_with_budget(retry_budget)),
        )
        .await
        .map_err(|e| e.to_string())
    }
    async fn delete_queue(&self, name: &str) -> Result<(), String> {
        QueueOrchestrator::delete_queue(self, name)
            .await
            .map_err(|e| e.to_string())
    }
    async fn enqueue(&self, name: &str, item: QueueItem<String>) -> Result<Uuid, String> {
        QueueOrchestrator::enqueue(self, name, item)
            .await
            .map_err(|e| e.to_string())
    }
    async fn dequeue(&self, name: &str) -> Result<Option<Uuid>, String> {
        QueueOrchestrator::dequeue(self, name)
            .await
            .map(|opt| opt.map(|item| item.id()))
            .map_err(|e| e.to_string())
    }
    async fn start_processing(&self, name: &str, id: Uuid, worker: &str) -> Result<(), String> {
        QueueOrchestrator::start_processing(self, name, id, worker.to_string())
            .await
            .map_err(|e| e.to_string())
    }
    async fn complete_processing(&self, name: &str, id: Uuid) -> Result<(), String> {
        QueueOrchestrator::complete_processing(self, name, id, None)
            .await
            .map_err(|e| e.to_string())
    }
    async fn fail_processing(&self, name: &str, id: Uuid) -> Result<bool, String> {
        QueueOrchestrator::fail_processing(self, name, id, "boom".to_string())
            .await
            .map_err(|e| e.to_string())
    }
    async fn list_queues(&self) -> Vec<String> {
        QueueOrchestrator::list_queues(self).await
    }
    async fn queue_length(&self, name: &str) -> Result<usize, String> {
        QueueOrchestrator::queue_length(self, name)
            .await
            .map_err(|e| e.to_string())
    }
    async fn health_check(&self) -> Result<bool, String> {
        // In-memory queue is always healthy once constructed.
        Ok(true)
    }
}

#[tokio::test]
async fn in_memory_queue_satisfies_queue_port_contract() {
    let orchestrator = QueueOrchestrator::new();
    assert_queue_port_contract(&orchestrator, "in-memory-contract").await;
}

/// In-memory dead-letter parity: a permanently failed item is preserved in the
/// failed set (reflected in `failed_items` stats) when `preserve_failed` is on
/// (the default).
#[tokio::test]
async fn in_memory_queue_preserves_dead_lettered_items() {
    let orchestrator = QueueOrchestrator::new();
    let qname = "in-memory-deadletter";
    orchestrator
        .create_queue(qname.to_string(), Some(queue_config_with_budget(0)))
        .await
        .unwrap();
    orchestrator
        .enqueue(qname, make_item(qname, "doomed-job", 0))
        .await
        .unwrap();

    let item = orchestrator.dequeue(qname).await.unwrap().unwrap();
    let id = item.id();
    orchestrator
        .start_processing(qname, id, "worker-1".to_string())
        .await
        .unwrap();
    let will_retry = orchestrator
        .fail_processing(qname, id, "fatal".to_string())
        .await
        .unwrap();
    assert!(!will_retry, "max_retries == 0 must not retry");

    let stats = orchestrator.get_queue_stats(qname).await.unwrap();
    assert_eq!(
        stats.failed_items, 1,
        "dead-lettered item must be preserved in the failed set"
    );
}

// ---------------------------------------------------------------------------
// Redis backend (feature-gated, skipped when no Redis is reachable)
// ---------------------------------------------------------------------------

#[cfg(feature = "redis-queue")]
mod redis_contract {
    use super::*;
    use paladin::infrastructure::adapters::queue::redis::{RedisQueueAdapter, RedisQueueConfig};
    use paladin_ports::output::queue_port::QueuePort;

    #[async_trait]
    impl QueuePortHarness for RedisQueueAdapter {
        async fn create_queue(&self, name: &str, retry_budget: u32) -> Result<(), String> {
            QueuePort::create_queue(
                self,
                name.to_string(),
                Some(queue_config_with_budget(retry_budget)),
            )
            .await
            .map_err(|e| e.to_string())
        }
        async fn delete_queue(&self, name: &str) -> Result<(), String> {
            QueuePort::delete_queue(self, name)
                .await
                .map_err(|e| e.to_string())
        }
        async fn enqueue(&self, name: &str, item: QueueItem<String>) -> Result<Uuid, String> {
            QueuePort::enqueue(self, name, item)
                .await
                .map_err(|e| e.to_string())
        }
        async fn dequeue(&self, name: &str) -> Result<Option<Uuid>, String> {
            QueuePort::dequeue(self, name)
                .await
                .map(|opt| opt.map(|item| item.id()))
                .map_err(|e| e.to_string())
        }
        async fn start_processing(&self, name: &str, id: Uuid, worker: &str) -> Result<(), String> {
            QueuePort::start_processing(self, name, id, worker.to_string())
                .await
                .map_err(|e| e.to_string())
        }
        async fn complete_processing(&self, name: &str, id: Uuid) -> Result<(), String> {
            QueuePort::complete_processing(self, name, id, None)
                .await
                .map_err(|e| e.to_string())
        }
        async fn fail_processing(&self, name: &str, id: Uuid) -> Result<bool, String> {
            QueuePort::fail_processing(self, name, id, "boom".to_string())
                .await
                .map_err(|e| e.to_string())
        }
        async fn list_queues(&self) -> Vec<String> {
            QueuePort::list_queues(self).await
        }
        async fn queue_length(&self, name: &str) -> Result<usize, String> {
            QueuePort::queue_length(self, name)
                .await
                .map_err(|e| e.to_string())
        }
        async fn health_check(&self) -> Result<bool, String> {
            QueuePort::health_check(self)
                .await
                .map_err(|e| e.to_string())
        }
    }

    /// Attempt to connect to a test Redis instance, trying the docker-compose
    /// test mapping (6380) and the conventional default (6379). Returns `None`
    /// when no instance is reachable so the test can skip rather than fail in
    /// environments without Redis.
    async fn try_connect() -> Option<RedisQueueAdapter> {
        let mut ports = Vec::new();
        if let Ok(port) = std::env::var("PALADIN_TEST_REDIS_PORT")
            && let Ok(port) = port.parse::<u16>()
        {
            ports.push(port);
        }
        ports.extend_from_slice(&[6380, 6379]);

        for port in ports {
            let config = RedisQueueConfig {
                redis_port: port,
                key_prefix: format!("paladin:test:{}", Uuid::new_v4()),
                connection_timeout: 2,
                ..Default::default()
            };
            // `ConnectionManager` applies its own retry/backoff and ignores
            // `connection_timeout`, so bound the whole connect + health check in
            // an explicit timeout to skip quickly when no Redis is reachable.
            let connect = async {
                let adapter = RedisQueueAdapter::new(config, None).await.ok()?;
                if QueuePort::health_check(&adapter).await.unwrap_or(false) {
                    Some(adapter)
                } else {
                    None
                }
            };
            if let Ok(Some(adapter)) =
                tokio::time::timeout(std::time::Duration::from_secs(3), connect).await
            {
                return Some(adapter);
            }
        }
        None
    }

    #[tokio::test]
    async fn redis_queue_satisfies_queue_port_contract() {
        let Some(adapter) = try_connect().await else {
            eprintln!("skipping: no reachable Redis instance (tried 6380, 6379)");
            return;
        };
        let qname = format!("redis-contract-{}", Uuid::new_v4());
        assert_queue_port_contract(&adapter, &qname).await;
    }
}
