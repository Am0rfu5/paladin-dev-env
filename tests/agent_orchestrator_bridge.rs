//! Integration tests for the agent → orchestrator bridge (Milestone 9, Epic 4).
//!
//! These tests build a real [`Orchestrator`], wrap it in an
//! [`OrchestratorBridgeAdapter`], and attach the resulting
//! [`OrchestratorPort`] to a [`PaladinExecutionService`]. They then drive a
//! `schedule_job` action through the bridge and assert that the job is
//! observable in the orchestrator's scheduler statistics, proving the bridge
//! reaches real orchestration state end-to-end.
//!
//! The suite is fully deterministic and offline: it uses
//! [`MockLlmAdapter`](paladin::MockLlmAdapter) to construct the execution
//! service and an in-process [`Orchestrator`], so no network access or real LLM
//! credentials are required.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::orchestration::{Orchestrator, OrchestratorBridgeAdapter};
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::schedule::Schedule;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin_ports::output::llm_port::LlmPort;
use paladin_ports::output::orchestrator_port::{
    BridgeAction, BridgePolicy, OrchestratorBridgeError, OrchestratorPort, ScheduleJobRequest,
};

/// Builds a [`PaladinExecutionService`] backed by a mock LLM, with the given
/// orchestrator bridge port attached.
fn build_service_with_bridge(bridge: Arc<dyn OrchestratorPort>) -> PaladinExecutionService {
    let llm_port: Arc<dyn LlmPort> =
        Arc::new(MockLlmAdapter::new().with_response("ok".to_string()));
    let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
    PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
        .with_orchestrator_port(bridge)
}

#[tokio::test]
async fn agent_schedule_job_reaches_orchestrator_scheduler() {
    // Arrange: real orchestrator + bridge with the default (permissive) policy.
    let orchestrator = Arc::new(Orchestrator::new());
    let bridge = Arc::new(OrchestratorBridgeAdapter::new(
        orchestrator.clone(),
        BridgePolicy::default(),
    ));

    // The bridge composes cleanly into a PaladinExecutionService.
    let service = build_service_with_bridge(bridge.clone());
    assert!(
        service.orchestrator_port().is_some(),
        "bridge should be attached to the execution service"
    );

    // Sanity: no jobs scheduled yet.
    let before = orchestrator.get_stats().await;
    assert_eq!(before.scheduler_stats.total_jobs, 0);

    // Act: agent schedules a job through the bridge port.
    let job_id = service
        .orchestrator_port()
        .expect("bridge attached")
        .schedule_job(ScheduleJobRequest {
            name: "nightly-report".to_string(),
            description: "Generate the nightly report".to_string(),
            schedule: Schedule::Interval(Duration::from_secs(3600)),
        })
        .await
        .expect("schedule_job succeeds");

    // Assert: the scheduled job is observable in orchestrator scheduler state.
    let after = orchestrator.get_stats().await;
    assert_eq!(
        after.scheduler_stats.total_jobs, 1,
        "scheduled job should be visible in the orchestrator"
    );
    assert!(!job_id.is_nil(), "a valid job id should be returned");
}

#[tokio::test]
async fn agent_schedule_job_rejected_when_action_disallowed() {
    // Arrange: a policy that allows nothing.
    let orchestrator = Arc::new(Orchestrator::new());
    let policy = BridgePolicy::new(HashSet::new(), 0, 0, 0, 0);
    let bridge = Arc::new(OrchestratorBridgeAdapter::new(orchestrator.clone(), policy));

    // Act: attempt a disallowed schedule_job.
    let result = bridge
        .schedule_job(ScheduleJobRequest {
            name: "forbidden".to_string(),
            description: "should not run".to_string(),
            schedule: Schedule::Interval(Duration::from_secs(60)),
        })
        .await;

    // Assert: typed policy error and no job scheduled.
    match result {
        Err(OrchestratorBridgeError::ActionNotAllowed(action)) => {
            assert_eq!(action, BridgeAction::ScheduleJob.as_str());
        }
        other => panic!("expected ActionNotAllowed, got {other:?}"),
    }

    let stats = orchestrator.get_stats().await;
    assert_eq!(
        stats.scheduler_stats.total_jobs, 0,
        "no job should be scheduled when the action is disallowed"
    );
}
