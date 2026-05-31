/*
SQLite Workflow Repository Adapter

Concrete implementation of `WorkflowRepositoryPort` using SQLite. Persists
workflow execution progress so the orchestrator can resume incomplete
workflows after a restart or crash. The workflow definition and the set of
completed job ids are stored as JSON; all writes use bound parameters to avoid
SQL injection (OWASP A03).
*/

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use paladin_ports::output::workflow_repository_port::{
    PersistedWorkflow, WorkflowPersistenceStatus, WorkflowRepositoryError, WorkflowRepositoryPort,
};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use uuid::Uuid;

#[doc(hidden)]
pub struct SqliteWorkflowRepository {
    pool: SqlitePool,
}

impl SqliteWorkflowRepository {
    /// Create a new SQLite workflow repository and run migrations.
    pub async fn new(database_url: &str) -> Result<Self, WorkflowRepositoryError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| {
                WorkflowRepositoryError::RepositoryError(format!(
                    "Failed to connect to database: {e}"
                ))
            })?;

        let repository = Self { pool };
        repository.migrate().await?;
        Ok(repository)
    }

    /// Run database migrations.
    async fn migrate(&self) -> Result<(), WorkflowRepositoryError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS workflow_state (
                workflow_id TEXT PRIMARY KEY NOT NULL,
                status TEXT NOT NULL,
                completed_job_ids TEXT NOT NULL,
                definition TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WorkflowRepositoryError::RepositoryError(format!("Migration failed: {e}")))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_workflow_state_status ON workflow_state(status)",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Index creation failed: {e}"))
        })?;

        Ok(())
    }

    /// Convert a database row into a [`PersistedWorkflow`].
    fn row_to_record(
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<PersistedWorkflow, WorkflowRepositoryError> {
        let workflow_id_str: String = row.try_get("workflow_id").map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Failed to get workflow_id: {e}"))
        })?;
        let workflow_id = Uuid::parse_str(&workflow_id_str).map_err(|e| {
            WorkflowRepositoryError::DeserializationError(format!("Invalid UUID: {e}"))
        })?;

        let status_str: String = row.try_get("status").map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Failed to get status: {e}"))
        })?;
        let status = WorkflowPersistenceStatus::from_str_value(&status_str)?;

        let completed_json: String = row.try_get("completed_job_ids").map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!(
                "Failed to get completed_job_ids: {e}"
            ))
        })?;
        let completed_job_ids: Vec<Uuid> = serde_json::from_str(&completed_json)
            .map_err(|e| WorkflowRepositoryError::DeserializationError(e.to_string()))?;

        let definition_json: String = row.try_get("definition").map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Failed to get definition: {e}"))
        })?;
        let definition = serde_json::from_str(&definition_json)
            .map_err(|e| WorkflowRepositoryError::DeserializationError(e.to_string()))?;

        let updated_str: String = row.try_get("updated_at").map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Failed to get updated_at: {e}"))
        })?;
        let updated_at = DateTime::parse_from_rfc3339(&updated_str)
            .map_err(|e| WorkflowRepositoryError::DeserializationError(e.to_string()))?
            .with_timezone(&Utc);

        Ok(PersistedWorkflow {
            workflow_id,
            status,
            completed_job_ids,
            definition,
            updated_at,
        })
    }
}

#[async_trait]
impl WorkflowRepositoryPort for SqliteWorkflowRepository {
    async fn save(&self, record: &PersistedWorkflow) -> Result<(), WorkflowRepositoryError> {
        let completed_json = serde_json::to_string(&record.completed_job_ids)
            .map_err(|e| WorkflowRepositoryError::SerializationError(e.to_string()))?;
        let definition_json = serde_json::to_string(&record.definition)
            .map_err(|e| WorkflowRepositoryError::SerializationError(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO workflow_state (
                workflow_id, status, completed_job_ids, definition, updated_at
            ) VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(workflow_id) DO UPDATE SET
                status = excluded.status,
                completed_job_ids = excluded.completed_job_ids,
                definition = excluded.definition,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(record.workflow_id.to_string())
        .bind(record.status.as_str())
        .bind(completed_json)
        .bind(definition_json)
        .bind(record.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            WorkflowRepositoryError::RepositoryError(format!("Failed to save workflow: {e}"))
        })?;

        Ok(())
    }

    async fn load(
        &self,
        workflow_id: Uuid,
    ) -> Result<Option<PersistedWorkflow>, WorkflowRepositoryError> {
        let row = sqlx::query("SELECT * FROM workflow_state WHERE workflow_id = ?")
            .bind(workflow_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                WorkflowRepositoryError::RepositoryError(format!("Database query failed: {e}"))
            })?;

        match row {
            Some(row) => Ok(Some(Self::row_to_record(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_incomplete(&self) -> Result<Vec<PersistedWorkflow>, WorkflowRepositoryError> {
        let rows =
            sqlx::query("SELECT * FROM workflow_state WHERE status IN ('pending', 'running')")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    WorkflowRepositoryError::RepositoryError(format!("Database query failed: {e}"))
                })?;

        rows.iter().map(Self::row_to_record).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_core::platform::container::orchestration_context::OrchestrationContext;
    use paladin_core::platform::container::workflow::{Workflow, WorkflowExecutionOrder};

    fn sample_workflow() -> Workflow {
        let context = OrchestrationContext::new("tester".to_string(), "test".to_string());
        Workflow {
            id: Uuid::new_v4(),
            name: "persisted".to_string(),
            description: "persistence test".to_string(),
            jobs: Vec::new(),
            listeners: Vec::new(),
            queues: Vec::new(),
            execution_order: WorkflowExecutionOrder::Sequential,
            context,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_save_and_load_roundtrip() {
        let repo = SqliteWorkflowRepository::new("sqlite::memory:")
            .await
            .unwrap();
        let mut record = PersistedWorkflow::pending(sample_workflow());
        let id = record.workflow_id;
        record.status = WorkflowPersistenceStatus::Running;
        record.completed_job_ids.push(Uuid::new_v4());

        repo.save(&record).await.unwrap();
        let loaded = repo.load(id).await.unwrap().unwrap();

        assert_eq!(loaded.workflow_id, id);
        assert_eq!(loaded.status, WorkflowPersistenceStatus::Running);
        assert_eq!(loaded.completed_job_ids.len(), 1);
    }

    #[tokio::test]
    async fn test_upsert_overwrites_existing() {
        let repo = SqliteWorkflowRepository::new("sqlite::memory:")
            .await
            .unwrap();
        let mut record = PersistedWorkflow::pending(sample_workflow());
        let id = record.workflow_id;
        repo.save(&record).await.unwrap();

        record.status = WorkflowPersistenceStatus::Completed;
        repo.save(&record).await.unwrap();

        let loaded = repo.load(id).await.unwrap().unwrap();
        assert_eq!(loaded.status, WorkflowPersistenceStatus::Completed);
    }

    #[tokio::test]
    async fn test_list_incomplete_excludes_terminal() {
        let repo = SqliteWorkflowRepository::new("sqlite::memory:")
            .await
            .unwrap();

        let mut running = PersistedWorkflow::pending(sample_workflow());
        running.status = WorkflowPersistenceStatus::Running;
        repo.save(&running).await.unwrap();

        let mut done = PersistedWorkflow::pending(sample_workflow());
        done.status = WorkflowPersistenceStatus::Completed;
        repo.save(&done).await.unwrap();

        let incomplete = repo.list_incomplete().await.unwrap();
        assert_eq!(incomplete.len(), 1);
        assert_eq!(incomplete[0].workflow_id, running.workflow_id);
    }
}
