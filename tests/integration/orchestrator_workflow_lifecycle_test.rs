//! Full-lifecycle orchestrator workflow integration test.
//!
//! Exercises the orchestrator end-to-end: register mock `TaskService`
//! implementations with observable, synchronized side effects, build a
//! three-job sequential workflow, start the orchestrator, execute the
//! workflow, and assert ordered execution with a `Completed` terminal state
//! and retrievable per-job results.

use async_trait::async_trait;
use paladin::application::services::orchestration::Orchestrator;
use paladin::core::platform::container::job::Job;
use paladin::core::platform::container::orchestration_context::OrchestrationContext;
use paladin::core::platform::container::task::{Task, TaskError, TaskService};
use paladin::core::platform::container::workflow::{Workflow, WorkflowExecutionOrder};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// A `TaskService` that appends its service name to a shared, synchronized
/// log when executed, giving the test an observable record of execution order.
#[derive(Debug, Clone)]
struct RecordingService {
    service_name: String,
    log: Arc<Mutex<Vec<String>>>,
}

impl RecordingService {
    fn new(service_name: &str, log: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            service_name: service_name.to_string(),
            log,
        }
    }
}

#[async_trait]
impl TaskService for RecordingService {
    fn name(&self) -> &str {
        &self.service_name
    }

    async fn execute(
        &self,
        _action: &paladin::core::base::component::action::Action,
    ) -> Result<Option<serde_json::Value>, TaskError> {
        self.log.lock().unwrap().push(self.service_name.clone());
        Ok(Some(serde_json::json!({ "service": self.service_name })))
    }

    fn clone_service(&self) -> Box<dyn TaskService> {
        Box::new(self.clone())
    }
}

/// Build a single-task job bound to `service_name`.
fn job_for_service(job_name: &str, service_name: &str) -> Job {
    let task = Task::new(
        format!("{job_name} task"),
        "lifecycle integration task".to_string(),
        service_name.to_string(),
    );
    Job::new(
        job_name.to_string(),
        "lifecycle integration job".to_string(),
        vec![task],
    )
}

#[tokio::test]
async fn full_lifecycle_sequential_workflow_executes_in_order_to_completion() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let orchestrator = Orchestrator::new();

    // Register three observable services.
    orchestrator
        .register_task_service(Box::new(RecordingService::new("StepOne", log.clone())))
        .await
        .expect("register StepOne");
    orchestrator
        .register_task_service(Box::new(RecordingService::new("StepTwo", log.clone())))
        .await
        .expect("register StepTwo");
    orchestrator
        .register_task_service(Box::new(RecordingService::new("StepThree", log.clone())))
        .await
        .expect("register StepThree");

    // Start the orchestrator (initializes defaults + scheduler).
    orchestrator.start().await.expect("orchestrator start");

    // Build a three-job sequential workflow.
    let jobs = vec![
        job_for_service("job-one", "StepOne"),
        job_for_service("job-two", "StepTwo"),
        job_for_service("job-three", "StepThree"),
    ];
    let expected_job_order: Vec<Uuid> = jobs.iter().map(|j| j.id()).collect();

    let context = OrchestrationContext::new("integration".to_string(), "test".to_string());
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: "Lifecycle Workflow".to_string(),
        description: "full-lifecycle integration".to_string(),
        jobs,
        listeners: Vec::new(),
        queues: Vec::new(),
        execution_order: WorkflowExecutionOrder::Sequential,
        context,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let workflow_id = orchestrator
        .create_workflow(workflow)
        .await
        .expect("create workflow");

    // Execute the workflow end-to-end.
    let result = orchestrator
        .execute_workflow(workflow_id)
        .await
        .expect("execute workflow");

    // Terminal state is Completed.
    assert!(result.completed(), "workflow should reach Completed");
    assert!(!result.failed());

    // Per-job results are retrievable and ordered.
    assert_eq!(result.job_outcomes.len(), 3);
    let actual_job_order: Vec<Uuid> = result.job_outcomes.iter().map(|o| o.job_id).collect();
    assert_eq!(
        actual_job_order, expected_job_order,
        "jobs must be recorded in workflow order"
    );
    for outcome in &result.job_outcomes {
        assert!(outcome.succeeded(), "every job should succeed");
        assert!(outcome.output.is_some(), "each job should record output");
    }

    // The observable side effects prove deterministic ordered execution.
    let recorded = log.lock().unwrap().clone();
    assert_eq!(
        recorded,
        vec![
            "StepOne".to_string(),
            "StepTwo".to_string(),
            "StepThree".to_string()
        ],
        "services must execute in sequential order"
    );

    // The result remains retrievable from the orchestrator after execution.
    let stored = orchestrator
        .workflow_execution_result(workflow_id)
        .await
        .expect("stored result must exist");
    assert!(stored.completed());
}
