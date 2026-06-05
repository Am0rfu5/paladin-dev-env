//! Repository adapters — re-exported from the non-optional `paladin-storage` crate.

/// MySQL content repository: available only when the `storage-mysql` feature is active.
#[cfg(feature = "storage-mysql")]
pub use paladin_storage::mysql_content_repository;
/// SQLite content/user repositories (always available; `paladin-storage` is non-optional
/// with its `sqlite` feature enabled).
pub use paladin_storage::{sqlite_content_repository, sqlite_user_repository};
