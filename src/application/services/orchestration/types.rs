/*
Orchestration Coordination Types

Application-layer coordination types for the orchestration subsystem.
These types live in the application layer because they reference multiple domain
entities, hold application-layer concerns, or implement async coordination logic.

Types moved here from:
- `core/platform/manager/orchestrator.rs`: OrchestratorError, OrchestratorStats,
  ContentAnalysisType, ContentProcessingResult, ContentProcessor, DefaultContentProcessor
- `core/platform/manager/listener_service.rs`: ListenerError
- `core/platform/manager/scheduler.rs`: SchedulerError
*/

use crate::application::services::queue_orchestrator::QueueError;
use crate::core::platform::container::content::ContentItem;
use crate::core::platform::container::job::JobError;
use crate::core::platform::container::orchestration_context::OrchestrationContext;
use crate::core::platform::container::queue_config::QueueStats;
use crate::core::platform::container::schedule::SchedulerStats;
use crate::core::platform::container::task::TaskError;
use crate::core::platform::container::trigger::ListenerStats;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Listener service errors.
#[derive(Debug, Error)]
pub enum ListenerError {
    #[error("Listener not found: {0}")]
    ListenerNotFound(String),
    #[error("Trigger not found: {0}")]
    TriggerNotFound(Uuid),
    #[error("Invalid listener configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Event processing failed: {0}")]
    EventProcessingFailed(String),
    #[error("Trigger creation failed: {0}")]
    TriggerCreationFailed(String),
    #[error("Listener operation failed: {0}")]
    OperationFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Scheduler-specific errors.
#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Job not found: {0}")]
    JobNotFound(Uuid),
    #[error("Job error: {0}")]
    JobError(#[from] JobError),
    #[error("Invalid schedule: {0}")]
    InvalidSchedule(String),
}

/// Orchestrator errors.
#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("Scheduler error: {0}")]
    SchedulerError(#[from] SchedulerError),
    #[error("Queue error: {0}")]
    QueueError(#[from] QueueError),
    #[error("Listener error: {0}")]
    ListenerError(#[from] ListenerError),
    #[error("Job error: {0}")]
    JobError(#[from] JobError),
    #[error("Task error: {0}")]
    TaskError(#[from] TaskError),
    #[error("Processor not found: {0}")]
    ProcessorNotFound(String),
    #[error("Workflow not found: {0}")]
    WorkflowNotFound(Uuid),
    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Service error: {0}")]
    ServiceError(String),
}

/// Orchestrator statistics.
#[derive(Debug, Clone)]
pub struct OrchestratorStats {
    pub active_sessions: usize,
    pub total_workflows: usize,
    pub total_services: usize,
    pub total_processors: usize,
    pub scheduler_stats: SchedulerStats,
    pub queue_stats: HashMap<String, QueueStats>,
    pub listener_stats: HashMap<String, ListenerStats>,
}

/// Content analysis types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentAnalysisType {
    Summarization,
    SentimentAnalysis,
    KeywordExtraction,
    TopicModeling,
    LanguageDetection,
    Custom(String),
}

impl ContentAnalysisType {
    pub fn name(&self) -> &str {
        match self {
            Self::Summarization => "Summarization",
            Self::SentimentAnalysis => "Sentiment Analysis",
            Self::KeywordExtraction => "Keyword Extraction",
            Self::TopicModeling => "Topic Modeling",
            Self::LanguageDetection => "Language Detection",
            Self::Custom(name) => name,
        }
    }
}

/// Content processing result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProcessingResult {
    pub content_id: Uuid,
    pub processor_name: String,
    pub processing_time_ms: u64,
    pub success: bool,
    pub result_data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Trait for content processors.
#[async_trait]
pub trait ContentProcessor: Send + Sync {
    fn name(&self) -> &str;
    async fn process_content(
        &self,
        content: ContentItem,
        context: OrchestrationContext,
    ) -> Result<ContentProcessingResult, OrchestratorError>;
    fn clone_box(&self) -> Result<Box<dyn ContentProcessor>, OrchestratorError>;
}

/// Default content processor implementation.
#[derive(Debug, Clone)]
pub struct DefaultContentProcessor;

#[async_trait]
impl ContentProcessor for DefaultContentProcessor {
    fn name(&self) -> &str {
        "DefaultContentProcessor"
    }

    async fn process_content(
        &self,
        content: ContentItem,
        context: OrchestrationContext,
    ) -> Result<ContentProcessingResult, OrchestratorError> {
        let start_time = std::time::Instant::now();

        // Simulate content processing
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(ContentProcessingResult {
            content_id: content.uuid(),
            processor_name: self.name().to_string(),
            processing_time_ms,
            success: true,
            result_data: Some(serde_json::json!({
                "processed": true,
                "content_type": format!("{:?}", content.content()),
                "session_id": context.session_id
            })),
            error: None,
            metadata: HashMap::new(),
        })
    }

    fn clone_box(&self) -> Result<Box<dyn ContentProcessor>, OrchestratorError> {
        Ok(Box::new(self.clone()))
    }
}

/// Internal, crate-private terminal state of a single job within a workflow run.
///
/// Not part of the public API: callers observe results via
/// [`WorkflowExecutionResult`] and [`JobExecutionOutcome`] instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JobRunState {
    /// The job finished successfully.
    Completed,
    /// The job finished with a failure.
    Failed,
}

/// Internal, crate-private lifecycle state of a whole workflow run.
///
/// Not part of the public API: callers observe results via
/// [`WorkflowExecutionResult`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkflowRunState {
    /// The workflow run has been created but not started.
    Pending,
    /// The workflow run is in progress.
    Running,
    /// All jobs completed successfully.
    Completed,
    /// At least one job failed (and the workflow stopped per its error strategy).
    Failed,
}

/// Result data for a single job executed as part of a workflow.
///
/// The job's success/failure is derived from the crate-private lifecycle state
/// ([`JobRunState`]); no public state enum is exposed.
#[derive(Debug, Clone)]
pub struct JobExecutionOutcome {
    /// Identifier of the job.
    pub job_id: Uuid,
    /// Human-readable job name.
    pub job_name: String,
    /// Summary output produced by the job, if any.
    pub output: Option<serde_json::Value>,
    /// Error description when the job failed.
    pub error: Option<String>,
    /// Internal lifecycle state (crate-private, source of truth).
    state: JobRunState,
}

impl JobExecutionOutcome {
    /// Build an outcome for a successful job.
    pub(crate) fn success(
        job_id: Uuid,
        job_name: String,
        output: Option<serde_json::Value>,
    ) -> Self {
        Self {
            job_id,
            job_name,
            output,
            error: None,
            state: JobRunState::Completed,
        }
    }

    /// Build an outcome for a failed job.
    pub(crate) fn failure(job_id: Uuid, job_name: String, error: String) -> Self {
        Self {
            job_id,
            job_name,
            output: None,
            error: Some(error),
            state: JobRunState::Failed,
        }
    }

    /// Whether the job completed successfully.
    pub fn succeeded(&self) -> bool {
        matches!(self.state, JobRunState::Completed)
    }
}

/// Aggregated result of executing a workflow end-to-end.
///
/// Exposes result/reporting data only; the terminal status is derived from the
/// crate-private lifecycle state ([`WorkflowRunState`]) so no new public state
/// enum is added.
#[derive(Debug, Clone)]
pub struct WorkflowExecutionResult {
    /// Identifier of the workflow that was executed.
    pub workflow_id: Uuid,
    /// Per-job outcomes, ordered by execution order.
    pub job_outcomes: Vec<JobExecutionOutcome>,
    /// Internal lifecycle state (crate-private, source of truth).
    state: WorkflowRunState,
}

impl WorkflowExecutionResult {
    /// Create a new result in the pending state for `workflow_id`.
    pub(crate) fn new(workflow_id: Uuid) -> Self {
        Self {
            workflow_id,
            job_outcomes: Vec::new(),
            state: WorkflowRunState::Pending,
        }
    }

    /// Transition the workflow into the running state.
    pub(crate) fn start(&mut self) {
        self.state = WorkflowRunState::Running;
    }

    /// Record a job outcome onto the result.
    pub(crate) fn record_outcome(&mut self, outcome: JobExecutionOutcome) {
        self.job_outcomes.push(outcome);
    }

    /// Transition the workflow to the completed state.
    pub(crate) fn mark_completed(&mut self) {
        self.state = WorkflowRunState::Completed;
    }

    /// Transition the workflow to the failed state.
    pub(crate) fn mark_failed(&mut self) {
        self.state = WorkflowRunState::Failed;
    }

    /// Whether the workflow reached a successful terminal state.
    pub fn completed(&self) -> bool {
        matches!(self.state, WorkflowRunState::Completed)
    }

    /// Whether the workflow terminated due to a job failure.
    pub fn failed(&self) -> bool {
        matches!(self.state, WorkflowRunState::Failed)
    }
}
