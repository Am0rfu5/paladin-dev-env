/// SQL database repository port traits.
///
/// Defines the abstract contracts (ports) that SQL-backed storage adapters must
/// implement. These traits live here in `paladin-ports` so that any crate
/// (e.g. `paladin-storage`) can implement them without depending on the facade
/// crate, preserving the hexagonal architecture's inward-only dependency rule.
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use paladin_core::platform::container::content::{ContentItem, ContentItemError};
use paladin_core::platform::container::content_list::{ContentList, ContentListError};
use thiserror::Error;
use uuid::Uuid;

/// Errors produced by repository operations.
#[derive(Debug, Clone, Error)]
pub enum RepositoryError {
    /// A database connection could not be established or was lost.
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    /// A query failed to execute.
    #[error("Query execution error: {0}")]
    QueryError(String),
    /// Serialization or deserialization of a stored value failed.
    #[error("Serialization error: {0}")]
    SerializationError(String),
    /// The requested entity does not exist.
    #[error("Entity not found: {0}")]
    NotFound(String),
    /// A uniqueness constraint was violated.
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    /// A transaction could not be committed or rolled back.
    #[error("Transaction error: {0}")]
    TransactionError(String),
    /// A schema migration failed.
    #[error("Migration error: {0}")]
    MigrationError(String),
    /// A [`ContentItem`] domain error was encountered during persistence.
    #[error("Content item error: {0}")]
    ContentItemError(#[from] ContentItemError),
    /// A [`ContentList`] domain error was encountered during persistence.
    #[error("Content list error: {0}")]
    ContentListError(#[from] ContentListError),
}

/// Repository port for [`ContentItem`] persistence.
#[async_trait]
pub trait ContentRepository: Send + Sync {
    /// Get a content item by its deduplication hash.
    async fn get_by_hash(&self, hash: &str) -> Result<Option<ContentItem>, RepositoryError>;

    /// Get a content item by UUID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ContentItem>, RepositoryError>;

    /// Persist a new content item; returns the assigned UUID.
    async fn create(&self, content: ContentItem) -> Result<Uuid, RepositoryError>;

    /// Overwrite an existing content item.
    async fn update(&self, content: &ContentItem) -> Result<(), RepositoryError>;

    /// Remove a content item by UUID.
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    /// Return all content items (no pagination).
    async fn list(&self) -> Result<Vec<ContentItem>, RepositoryError>;

    /// Find content items that carry all of the given tags.
    async fn find_by_tags(&self, tags: &[String]) -> Result<Vec<ContentItem>, RepositoryError>;

    /// Find content items originating from a given source URL or path.
    async fn find_by_source(&self, source: &str) -> Result<Vec<ContentItem>, RepositoryError>;

    /// Return the total number of stored content items.
    async fn count(&self) -> Result<u64, RepositoryError>;

    /// Return `true` if a content item with the given hash already exists.
    async fn exists_by_hash(&self, hash: &str) -> Result<bool, RepositoryError>;
}

/// Repository port for [`ContentList`] persistence.
///
/// NOTE: methods are currently synchronous. An async migration is tracked as a
/// TODO in the original `sql_store.rs`.
pub trait ContentListRepository {
    /// Get a content list by UUID.
    fn get_by_id(&self, id: Uuid) -> Result<Option<ContentList>, RepositoryError>;

    /// Get a content list by name.
    fn get_by_name(&self, name: &str) -> Result<Option<ContentList>, RepositoryError>;

    /// Persist a content list.
    fn save(&self, content_list: &ContentList) -> Result<(), RepositoryError>;

    /// Overwrite an existing content list.
    fn update(&self, content_list: &ContentList) -> Result<(), RepositoryError>;

    /// Remove a content list by UUID.
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    /// Return all content lists, with optional pagination.
    fn find_all(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<ContentList>, RepositoryError>;

    /// Find content lists created within a date range.
    fn find_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<ContentList>, RepositoryError>;

    /// Return the total number of stored content lists.
    fn count(&self) -> Result<u64, RepositoryError>;

    /// Append an item to a content list.
    fn add_item_to_list(&self, list_id: Uuid, item: &ContentItem) -> Result<(), RepositoryError>;

    /// Remove an item from a content list.
    fn remove_item_from_list(&self, list_id: Uuid, item_id: Uuid) -> Result<(), RepositoryError>;
}

/// Port for wrapping multiple repository operations in a single database transaction.
pub trait TransactionManager {
    /// Execute `operation` atomically; roll back on any error.
    fn with_transaction<F, R>(&self, operation: F) -> Result<R, RepositoryError>
    where
        F: FnOnce() -> Result<R, RepositoryError>;
}

/// Port for running database schema migrations.
#[async_trait]
pub trait MigrationManager: Send + Sync {
    /// Apply any pending migrations.
    async fn migrate(&self) -> Result<(), RepositoryError>;

    /// Return `true` if the schema is already at the latest version.
    async fn is_up_to_date(&self) -> Result<bool, RepositoryError>;

    /// Return the identifier of the currently applied migration version.
    async fn current_version(&self) -> Result<Option<String>, RepositoryError>;
}

/// General-purpose SQL store health and maintenance port.
#[async_trait]
pub trait SqlStore: Send + Sync {
    /// Return aggregate statistics for the underlying store.
    async fn get_stats(&self) -> Result<RepositoryStats, RepositoryError>;

    /// Perform a lightweight connectivity / health check.
    async fn health_check(&self) -> Result<bool, RepositoryError>;

    /// Remove all content older than `older_than`; returns the number of rows deleted.
    async fn cleanup(&self, older_than: DateTime<Utc>) -> Result<u64, RepositoryError>;
}

/// Aggregate statistics reported by a [`SqlStore`].
#[derive(Debug, Clone, PartialEq)]
pub struct RepositoryStats {
    /// Total number of stored [`ContentItem`]s.
    pub total_content_items: u64,
    /// Total number of stored [`ContentList`]s.
    pub total_content_lists: u64,
    /// Number of items that still need to be fetched.
    pub total_items_to_fetch: u64,
    /// Physical database file size in bytes, if available.
    pub database_size_bytes: Option<u64>,
    /// Timestamp of the most recent write.
    pub last_updated: DateTime<Utc>,
}
