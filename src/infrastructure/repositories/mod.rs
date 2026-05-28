pub mod file_content_repository;
/// SQLite content/user repositories: re-exported from `paladin-storage` when the
/// `storage-sqlite` feature is active; falls back to the facade-local copy otherwise.
#[cfg(feature = "storage-sqlite")]
pub use paladin_storage::sqlite_content_repository;
#[cfg(not(feature = "storage-sqlite"))]
pub mod sqlite_content_repository;
#[cfg(feature = "storage-sqlite")]
pub use paladin_storage::sqlite_user_repository;
#[cfg(not(feature = "storage-sqlite"))]
pub mod sqlite_user_repository;
/// MySQL content repository: available only when the `storage-mysql` feature is active.
#[cfg(feature = "storage-mysql")]
pub use paladin_storage::mysql_content_repository;
