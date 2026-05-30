/*
Task Container

A Task is a platform-level container that extends the base Action component
to provide specific execution capabilities. Tasks represent individual units
of work that can be executed independently or as part of a larger Job.

Tasks add:
- Service-based execution through TaskService trait
- Task-specific error handling and states
- Integration with the Action lifecycle
- Platform-level orchestration capabilities

Tasks are the building blocks for Jobs and can be scheduled, queued,
and executed through various platform services.
*/

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

// Import the base Action component
use crate::base::component::action::{
    Action, ActionError, ActionPriority, ActionResult, ActionStatus,
};

/// Task-specific execution modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskExecutionMode {
    /// Execute once and complete
    OneTime,
    /// Can be retried on failure
    Retryable,
    /// Idempotent - safe to run multiple times
    Idempotent,
}

/// Task container that extends the base Action component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Base action functionality
    pub action: Action,
    /// Task-specific execution mode
    pub execution_mode: TaskExecutionMode,
    /// Service name that will execute this task
    pub service_name: String,
    /// Task-specific metadata
    pub task_metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl Task {
    /// Creates a new Task with the specified service
    pub fn new(name: String, description: String, service_name: String) -> Self {
        let action = Action::new(
            name.clone(),
            description,
            "task_manager".to_string(),
            service_name.clone(),
        );

        Self {
            action,
            execution_mode: TaskExecutionMode::Retryable,
            service_name,
            task_metadata: std::collections::HashMap::new(),
        }
    }

    /// Creates a new Task with full configuration
    pub fn new_with_config(
        name: String,
        description: String,
        service_name: String,
        execution_mode: TaskExecutionMode,
        priority: ActionPriority,
    ) -> Self {
        let action = Action::new(
            name.clone(),
            description,
            "task_manager".to_string(),
            service_name.clone(),
        )
        .with_priority(priority);

        Self {
            action,
            execution_mode,
            service_name,
            task_metadata: std::collections::HashMap::new(),
        }
    }

    /// Sets the execution mode
    pub fn with_execution_mode(mut self, mode: TaskExecutionMode) -> Self {
        self.execution_mode = mode;
        self
    }

    /// Sets task priority
    pub fn with_priority(mut self, priority: ActionPriority) -> Self {
        self.action = self.action.with_priority(priority);
        self
    }

    /// Sets timeout for task execution
    pub fn with_timeout(mut self, timeout_seconds: u32) -> Self {
        self.action = self.action.with_timeout(timeout_seconds);
        self
    }

    /// Adds task-specific metadata
    pub fn add_metadata<T: Serialize>(&mut self, key: String, value: T) -> Result<(), TaskError> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| TaskError::SerializationError(e.to_string()))?;
        self.task_metadata.insert(key, json_value);
        Ok(())
    }

    /// Gets task metadata
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.task_metadata.get(key)
    }

    /// Executes the task using the provided service
    pub async fn execute<T: TaskService + ?Sized>(&mut self, service: &T) -> Result<(), TaskError> {
        // Check if task can be executed
        if !self.action.can_execute() {
            return Err(TaskError::InvalidState(self.action.status.clone()));
        }

        // Start execution
        self.action.start_execution();

        // Execute the task
        let start_time = std::time::Instant::now();
        let execution_result = service.execute(&self.action).await;
        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Handle result
        match execution_result {
            Ok(result_data) => {
                let action_result = ActionResult {
                    success: true,
                    duration_ms,
                    data: result_data,
                    error: None,
                    metadata: std::collections::HashMap::new(),
                };
                self.action.complete_execution(action_result);
                Ok(())
            }
            Err(task_error) => {
                let error_message = task_error.to_string();
                let can_retry = self.action.fail_execution(error_message, duration_ms);

                if can_retry && matches!(self.execution_mode, TaskExecutionMode::Retryable) {
                    Err(TaskError::RetryableFailure(task_error.to_string()))
                } else {
                    Err(task_error)
                }
            }
        }
    }

    /// Executes the task without a service (for testing or simple tasks)
    pub async fn execute_simple(&mut self) -> Result<(), TaskError> {
        if !self.action.can_execute() {
            return Err(TaskError::InvalidState(self.action.status.clone()));
        }

        self.action.start_execution();

        // Simulate simple execution
        let start_time = std::time::Instant::now();
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let duration_ms = start_time.elapsed().as_millis() as u64;

        let action_result = ActionResult {
            success: true,
            duration_ms,
            data: Some(serde_json::json!({"message": "Task completed successfully"})),
            error: None,
            metadata: std::collections::HashMap::new(),
        };

        self.action.complete_execution(action_result);
        Ok(())
    }

    /// Cancels the task
    pub fn cancel(&mut self) {
        self.action.cancel();
    }

    /// Resets the task for re-execution
    pub fn reset(&mut self) {
        self.action.reset();
    }

    /// Checks if the task can be executed
    pub fn can_execute(&self) -> bool {
        self.action.can_execute()
    }

    /// Checks if the task is in a terminal state
    pub fn is_complete(&self) -> bool {
        self.action.is_terminal()
    }

    /// Gets the task's current status
    pub fn status(&self) -> &ActionStatus {
        &self.action.status
    }

    /// Gets the task ID
    pub fn id(&self) -> Uuid {
        self.action.id
    }

    /// Gets the task name
    pub fn name(&self) -> &str {
        &self.action.name
    }

    /// Gets execution statistics
    pub fn execution_stats(&self) -> TaskStats {
        TaskStats {
            execution_count: self.action.execution_count,
            success_rate: self.action.success_rate(),
            average_duration_ms: self.action.average_duration_ms(),
            last_execution: self.action.last_execution,
        }
    }

    /// Creates a new instance for re-execution (useful for recurring tasks)
    pub fn clone_for_new_execution(&self) -> Self {
        let mut cloned = self.clone();
        cloned.action = self.action.clone_for_new_execution();
        cloned
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.action.id == other.action.id
    }
}

impl Eq for Task {}

/// Task execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub execution_count: u32,
    pub success_rate: f64,
    pub average_duration_ms: Option<u64>,
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

/// Task-specific errors
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Task execution timed out")]
    Timeout,
    #[error("Task failed but can be retried: {0}")]
    RetryableFailure(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Task is not in a valid state for execution: {0:?}")]
    InvalidState(ActionStatus),
    #[error("Action error: {0}")]
    ActionError(#[from] ActionError),
}

/// Trait for services that can execute tasks
#[async_trait]
pub trait TaskService: Debug + Send + Sync {
    /// Returns the service name
    fn name(&self) -> &str;

    /// Executes the task and returns optional result data
    async fn execute(&self, action: &Action) -> Result<Option<serde_json::Value>, TaskError>;

    /// Checks if the service can handle the given task
    fn can_handle(&self, task: &Task) -> bool {
        task.service_name == self.name()
    }

    /// Clones the service for re-use
    fn clone_service(&self) -> Box<dyn TaskService>;
}

// ---------------------------------------------------------------------------
// Default task services
// ---------------------------------------------------------------------------

/// Validates a caller-supplied relative artifact name and guards against
/// path-traversal (OWASP A01). Absolute paths and any `..`/root components are
/// rejected so writes can never escape their configured base directory.
fn safe_relative_name(name: &str) -> Result<PathBuf, TaskError> {
    let candidate = Path::new(name);
    if candidate.as_os_str().is_empty() {
        return Err(TaskError::ExecutionFailed(
            "artifact name must not be empty".to_string(),
        ));
    }
    if candidate.is_absolute() {
        return Err(TaskError::ExecutionFailed(
            "artifact name must be relative".to_string(),
        ));
    }
    for component in candidate.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(TaskError::ExecutionFailed(
                "artifact name must not traverse directories".to_string(),
            ));
        }
    }
    Ok(candidate.to_path_buf())
}

/// Persists a backup artifact into a configured, path-constrained directory.
#[derive(Debug, Clone)]
pub struct DataBackupService {
    /// Base directory that all backups are written beneath.
    pub backup_path: String,
}

impl DataBackupService {
    /// Creates a new backup service rooted at `backup_path`.
    pub fn new(backup_path: impl Into<String>) -> Self {
        Self {
            backup_path: backup_path.into(),
        }
    }
}

#[async_trait]
impl TaskService for DataBackupService {
    fn name(&self) -> &str {
        "DataBackupService"
    }

    async fn execute(&self, action: &Action) -> Result<Option<serde_json::Value>, TaskError> {
        let base = Path::new(&self.backup_path);
        tokio::fs::create_dir_all(base).await.map_err(|e| {
            TaskError::ExecutionFailed(format!("failed to prepare backup dir: {e}"))
        })?;

        // Resolve the target file name, guarding against path traversal.
        let file_name = match action.get_argument("backup_name").and_then(|v| v.as_str()) {
            Some(name) => safe_relative_name(name)?,
            None => PathBuf::from(format!("{}.backup.json", action.id)),
        };
        let target = base.join(&file_name);

        // Use an explicit payload when provided, otherwise persist a manifest
        // derived from the task itself.
        let payload = match action.get_argument("payload") {
            Some(value) => serde_json::to_vec_pretty(value),
            None => serde_json::to_vec_pretty(&serde_json::json!({
                "task_id": action.id,
                "task_name": action.name,
                "description": action.description,
                "backed_up_at": chrono::Utc::now(),
            })),
        }
        .map_err(|e| TaskError::SerializationError(e.to_string()))?;

        let bytes_written = payload.len();
        tokio::fs::write(&target, &payload)
            .await
            .map_err(|e| TaskError::ExecutionFailed(format!("backup write failed: {e}")))?;

        Ok(Some(serde_json::json!({
            "backup_path": target.to_string_lossy(),
            "bytes_written": bytes_written,
            "status": "completed",
            "timestamp": chrono::Utc::now(),
        })))
    }

    fn clone_service(&self) -> Box<dyn TaskService> {
        Box::new(self.clone())
    }
}

/// Builds and persists a simple term-frequency index artifact.
#[derive(Debug, Clone)]
pub struct ContentIndexingService {
    /// Logical name of the index this service maintains.
    pub index_name: String,
}

impl ContentIndexingService {
    /// Creates a new indexing service for the given `index_name`.
    pub fn new(index_name: impl Into<String>) -> Self {
        Self {
            index_name: index_name.into(),
        }
    }

    /// Base directory where index artifacts are persisted.
    fn index_dir() -> PathBuf {
        std::env::temp_dir().join("paladin_index")
    }
}

#[async_trait]
impl TaskService for ContentIndexingService {
    fn name(&self) -> &str {
        "ContentIndexingService"
    }

    async fn execute(&self, action: &Action) -> Result<Option<serde_json::Value>, TaskError> {
        // The index name doubles as part of the artifact file name, so guard it.
        let safe_name = safe_relative_name(&self.index_name)?;
        let dir = Self::index_dir();
        tokio::fs::create_dir_all(&dir)
            .await
            .map_err(|e| TaskError::ExecutionFailed(format!("failed to prepare index dir: {e}")))?;

        // Index either explicit content or the task description as a fallback.
        let content = action
            .get_argument("content")
            .and_then(|v| v.as_str())
            .unwrap_or(action.description.as_str());

        let mut term_counts: BTreeMap<String, usize> = BTreeMap::new();
        for term in content.split_whitespace() {
            *term_counts.entry(term.to_lowercase()).or_insert(0) += 1;
        }

        let artifact = serde_json::json!({
            "index_name": self.index_name,
            "task_id": action.id,
            "terms": term_counts,
            "indexed_at": chrono::Utc::now(),
        });
        let bytes = serde_json::to_vec_pretty(&artifact)
            .map_err(|e| TaskError::SerializationError(e.to_string()))?;
        let target = dir.join(format!(
            "{}-{}.index.json",
            safe_name.to_string_lossy(),
            action.id
        ));
        tokio::fs::write(&target, &bytes)
            .await
            .map_err(|e| TaskError::ExecutionFailed(format!("index write failed: {e}")))?;

        Ok(Some(serde_json::json!({
            "index_name": self.index_name,
            "index_path": target.to_string_lossy(),
            "terms_indexed": term_counts.len(),
            "status": "indexed",
            "timestamp": chrono::Utc::now(),
        })))
    }

    fn clone_service(&self) -> Box<dyn TaskService> {
        Box::new(self.clone())
    }
}

/// A captured outbound email message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailMessage {
    /// Recipient address.
    pub to: String,
    /// Message subject line.
    pub subject: String,
    /// Message body.
    pub body: String,
}

/// Injectable transport seam for delivering email notifications. Production
/// deployments supply a real SMTP transport; tests supply observable or
/// failing sinks without touching the network.
#[async_trait]
pub trait EmailSink: Debug + Send + Sync {
    /// Delivers a single message, returning a `TaskError` on failure.
    async fn deliver(&self, message: &EmailMessage) -> Result<(), TaskError>;

    /// Clones the sink behind a shared pointer for reuse.
    fn clone_sink(&self) -> Arc<dyn EmailSink>;
}

/// Default in-process sink that records delivered messages for inspection.
#[derive(Debug, Clone, Default)]
pub struct InMemoryEmailSink {
    delivered: Arc<Mutex<Vec<EmailMessage>>>,
}

impl InMemoryEmailSink {
    /// Creates an empty in-memory sink.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a snapshot of all messages delivered so far.
    pub fn delivered(&self) -> Vec<EmailMessage> {
        self.delivered
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }
}

#[async_trait]
impl EmailSink for InMemoryEmailSink {
    async fn deliver(&self, message: &EmailMessage) -> Result<(), TaskError> {
        self.delivered
            .lock()
            .map_err(|e| TaskError::ExecutionFailed(format!("sink poisoned: {e}")))?
            .push(message.clone());
        Ok(())
    }

    fn clone_sink(&self) -> Arc<dyn EmailSink> {
        Arc::new(self.clone())
    }
}

/// Dispatches email notifications through an injectable [`EmailSink`].
#[derive(Debug, Clone)]
pub struct EmailNotificationService {
    /// SMTP server identifier recorded with each dispatch.
    pub smtp_server: String,
    /// Transport seam used to actually deliver messages.
    sink: Arc<dyn EmailSink>,
}

impl EmailNotificationService {
    /// Creates a service that records messages in an in-memory sink.
    pub fn new(smtp_server: impl Into<String>) -> Self {
        Self {
            smtp_server: smtp_server.into(),
            sink: Arc::new(InMemoryEmailSink::new()),
        }
    }

    /// Creates a service backed by a caller-supplied transport sink.
    pub fn with_sink(smtp_server: impl Into<String>, sink: Arc<dyn EmailSink>) -> Self {
        Self {
            smtp_server: smtp_server.into(),
            sink,
        }
    }
}

#[async_trait]
impl TaskService for EmailNotificationService {
    fn name(&self) -> &str {
        "EmailNotificationService"
    }

    async fn execute(&self, action: &Action) -> Result<Option<serde_json::Value>, TaskError> {
        let to = action
            .get_argument("to_email")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown@example.com")
            .to_string();
        let subject = action
            .get_argument("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("(no subject)")
            .to_string();
        let body = action
            .get_argument("body")
            .and_then(|v| v.as_str())
            .unwrap_or(action.description.as_str())
            .to_string();

        let message = EmailMessage {
            to: to.clone(),
            subject,
            body,
        };
        self.sink.deliver(&message).await?;

        Ok(Some(serde_json::json!({
            "smtp_server": self.smtp_server,
            "to_email": to,
            "status": "sent",
            "timestamp": chrono::Utc::now(),
        })))
    }

    fn clone_service(&self) -> Box<dyn TaskService> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_task_creation() {
        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "TestService".to_string(),
        );

        assert_eq!(task.name(), "Test Task");
        assert_eq!(task.service_name, "TestService");
        assert_eq!(task.execution_mode, TaskExecutionMode::Retryable);
        assert!(task.can_execute());
    }

    #[tokio::test]
    async fn test_task_execution_with_service() {
        let mut task = Task::new(
            "Backup Task".to_string(),
            "Backup data task".to_string(),
            "DataBackupService".to_string(),
        );

        let temp = std::env::temp_dir().join(format!("paladin_backup_test_{}", Uuid::new_v4()));
        let service = DataBackupService::new(temp.to_string_lossy().to_string());

        let result = task.execute(&service).await;
        assert!(result.is_ok());
        assert_eq!(task.status(), &ActionStatus::Completed);
        assert_eq!(task.action.execution_count, 1);
    }

    #[tokio::test]
    async fn test_task_simple_execution() {
        let mut task = Task::new(
            "Simple Task".to_string(),
            "A simple task".to_string(),
            "SimpleService".to_string(),
        );

        let result = task.execute_simple().await;
        assert!(result.is_ok());
        assert_eq!(task.status(), &ActionStatus::Completed);
    }

    #[tokio::test]
    async fn test_task_with_metadata() {
        let mut task = Task::new(
            "Metadata Task".to_string(),
            "Task with metadata".to_string(),
            "TestService".to_string(),
        );

        task.add_metadata("custom_field".to_string(), "custom_value")
            .unwrap();
        task.add_metadata("priority_level".to_string(), 5).unwrap();

        assert_eq!(
            task.get_metadata("custom_field"),
            Some(&json!("custom_value"))
        );
        assert_eq!(task.get_metadata("priority_level"), Some(&json!(5)));
    }

    #[tokio::test]
    async fn test_task_configuration() {
        let task = Task::new_with_config(
            "Config Task".to_string(),
            "Configured task".to_string(),
            "ConfigService".to_string(),
            TaskExecutionMode::Idempotent,
            ActionPriority::High,
        )
        .with_timeout(30);

        assert_eq!(task.execution_mode, TaskExecutionMode::Idempotent);
        assert_eq!(task.action.priority, ActionPriority::High);
        assert_eq!(task.action.timeout_seconds, Some(30));
    }

    #[tokio::test]
    async fn test_task_statistics() {
        let mut task = Task::new(
            "Stats Task".to_string(),
            "Task for statistics".to_string(),
            "StatsService".to_string(),
        );

        // Execute the task
        task.execute_simple().await.unwrap();

        let stats = task.execution_stats();
        assert_eq!(stats.execution_count, 1);
        assert_eq!(stats.success_rate, 100.0);
        assert!(stats.average_duration_ms.is_some());
        assert!(stats.last_execution.is_some());
    }

    #[tokio::test]
    async fn test_task_clone_for_new_execution() {
        let mut original = Task::new(
            "Original Task".to_string(),
            "Original task".to_string(),
            "TestService".to_string(),
        );

        // Execute the original
        original.execute_simple().await.unwrap();

        // Clone for new execution
        let cloned = original.clone_for_new_execution();

        assert_ne!(original.id(), cloned.id());
        assert_eq!(cloned.status(), &ActionStatus::Pending);
        assert_eq!(cloned.action.execution_count, 0);
    }

    #[tokio::test]
    async fn test_email_notification_service() {
        let mut task = Task::new(
            "Email Task".to_string(),
            "Send email notification".to_string(),
            "EmailNotificationService".to_string(),
        );

        // Add email arguments
        task.action
            .add_argument("to_email".to_string(), "user@example.com")
            .unwrap();
        task.action
            .add_argument("subject".to_string(), "Test Subject")
            .unwrap();

        let service = EmailNotificationService::new("smtp.example.com");

        let result = task.execute(&service).await;
        assert!(result.is_ok());
        assert_eq!(task.status(), &ActionStatus::Completed);
    }

    #[tokio::test]
    async fn test_data_backup_service_writes_artifact() {
        let dir = std::env::temp_dir().join(format!("paladin_backup_{}", Uuid::new_v4()));
        let service = DataBackupService::new(dir.to_string_lossy().to_string());

        let action = Action::new(
            "backup".to_string(),
            "nightly backup".to_string(),
            "task_manager".to_string(),
            "DataBackupService".to_string(),
        );
        let result = service.execute(&action).await.unwrap().unwrap();

        let path = result["backup_path"].as_str().unwrap();
        assert!(
            std::path::Path::new(path).exists(),
            "backup file must exist"
        );
        assert!(result["bytes_written"].as_u64().unwrap() > 0);
        assert_eq!(result["status"], "completed");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_data_backup_service_rejects_path_traversal() {
        let dir = std::env::temp_dir().join(format!("paladin_backup_{}", Uuid::new_v4()));
        let service = DataBackupService::new(dir.to_string_lossy().to_string());

        let mut action = Action::new(
            "backup".to_string(),
            "malicious backup".to_string(),
            "task_manager".to_string(),
            "DataBackupService".to_string(),
        );
        action
            .add_argument("backup_name".to_string(), "../escape.json")
            .unwrap();

        let result = service.execute(&action).await;
        assert!(matches!(result, Err(TaskError::ExecutionFailed(_))));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_content_indexing_service_builds_index() {
        let service = ContentIndexingService::new(format!("idx_{}", Uuid::new_v4()));
        let mut action = Action::new(
            "index".to_string(),
            "index this".to_string(),
            "task_manager".to_string(),
            "ContentIndexingService".to_string(),
        );
        action
            .add_argument("content".to_string(), "alpha beta beta gamma")
            .unwrap();

        let result = service.execute(&action).await.unwrap().unwrap();
        let path = result["index_path"].as_str().unwrap();
        assert!(std::path::Path::new(path).exists());
        assert_eq!(result["terms_indexed"].as_u64().unwrap(), 3);
        assert_eq!(result["status"], "indexed");
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn test_email_service_delivers_to_sink() {
        let sink = InMemoryEmailSink::new();
        let service = EmailNotificationService::with_sink("smtp.test", sink.clone_sink());

        let mut action = Action::new(
            "email".to_string(),
            "hello body".to_string(),
            "task_manager".to_string(),
            "EmailNotificationService".to_string(),
        );
        action
            .add_argument("to_email".to_string(), "a@b.com")
            .unwrap();
        action.add_argument("subject".to_string(), "Hi").unwrap();

        let result = service.execute(&action).await.unwrap().unwrap();
        assert_eq!(result["status"], "sent");
        let delivered = sink.delivered();
        assert_eq!(delivered.len(), 1);
        assert_eq!(delivered[0].to, "a@b.com");
        assert_eq!(delivered[0].subject, "Hi");
    }

    #[tokio::test]
    async fn test_email_service_propagates_sink_failure() {
        #[derive(Debug, Clone)]
        struct FailingSink;
        #[async_trait]
        impl EmailSink for FailingSink {
            async fn deliver(&self, _message: &EmailMessage) -> Result<(), TaskError> {
                Err(TaskError::ServiceUnavailable("transport down".to_string()))
            }
            fn clone_sink(&self) -> Arc<dyn EmailSink> {
                Arc::new(self.clone())
            }
        }

        let service = EmailNotificationService::with_sink("smtp.test", Arc::new(FailingSink));
        let action = Action::new(
            "email".to_string(),
            "body".to_string(),
            "task_manager".to_string(),
            "EmailNotificationService".to_string(),
        );

        let result = service.execute(&action).await;
        assert!(matches!(result, Err(TaskError::ServiceUnavailable(_))));
    }
}
