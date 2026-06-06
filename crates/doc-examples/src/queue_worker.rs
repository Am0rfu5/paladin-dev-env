//! Compiled example for `docs/src/deployment-topologies/queue-worker.md`
//! (Epic 6 of Milestone 11).
//!
//! Pulled into the page via mdBook `{{#include}}`, so `cargo check
//! -p paladin-doc-examples` keeps it matching the current `RedisQueueAdapter`
//! and queue-port API. The Redis calls compile but are never executed by the
//! gate, so no live Redis is needed to check the example.
#![allow(unused_variables, unused_imports, dead_code)]

// ANCHOR: queue
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use paladin_core::base::entity::message::{Location, Message};
use paladin_core::platform::container::queue_item::QueueItem;
use paladin_ports::output::queue_port::QueuePort;
use paladin_storage::redis::{RedisQueueAdapter, RedisQueueConfig};

use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::prelude::*; // Paladin, PaladinResult, ...

const QUEUE: &str = "agent-jobs";

/// The unit of work a producer enqueues and a worker executes.
#[derive(Clone, Serialize, Deserialize)]
struct AgentJob {
    agent: String,
    input: String,
}

/// **Producer** — connect to Redis and enqueue an agent job. Many producers can
/// enqueue concurrently; the queue absorbs bursts and applies backpressure.
pub async fn enqueue_job() -> Result<(), Box<dyn std::error::Error>> {
    let queue = RedisQueueAdapter::new(RedisQueueConfig::default(), None).await?;
    queue.create_queue(QUEUE.to_string(), None).await?;

    let job = AgentJob {
        agent: "summarizer".to_string(),
        input: "Summarise the Q3 earnings call.".to_string(),
    };
    let message = Message::new(
        Location::service("producer"),
        Location::service("worker"),
        job,
    );
    let item = QueueItem::new(QUEUE.to_string(), message, None);

    let id = queue.enqueue(QUEUE, item).await?;
    println!("enqueued job {id}");
    Ok(())
}

/// **Worker** — pull jobs off the queue and run them through a
/// `PaladinExecutionService`. Run many of these (in this process or across hosts)
/// to scale out. Each item is marked in-progress, then completed with its result.
pub async fn run_worker(
    queue: &RedisQueueAdapter,
    service: &PaladinExecutionService,
    agent: &Paladin,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(item) = queue.dequeue(QUEUE).await? {
        let item_id = item.action.id;
        queue
            .start_processing(QUEUE, item_id, "worker-1".to_string())
            .await?;

        // The dequeued payload is generic JSON; read the agent input from it.
        let input = item.message.payload()["input"].as_str().unwrap_or_default();
        let result: PaladinResult = service.execute(agent, input).await?;

        queue
            .complete_processing(
                QUEUE,
                item_id,
                Some(serde_json::json!({ "output": result.output })),
            )
            .await?;
    }
    Ok(())
}
// ANCHOR_END: queue
