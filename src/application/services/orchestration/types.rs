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
