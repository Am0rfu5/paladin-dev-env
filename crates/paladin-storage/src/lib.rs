//! # paladin-storage
//!
//! SQL-backed repository adapters for the Paladin multi-agent framework.
//!
//! ## Feature flags
//!
//! | Flag | Enables |
//! |------|---------|
//! | `sqlite` | [`sqlite_content_repository`], [`sqlite_user_repository`] |
//! | `mysql`  | [`mysql_content_repository`] |
//!
//! Enable only the backends your deployment actually uses.

#![warn(missing_docs)]
#![allow(rustdoc::broken_intra_doc_links)]

/// SQLite implementation of `ContentRepository`, `ContentListRepository`,
/// `MigrationManager`, and `SqlStore`.
#[cfg(feature = "sqlite")]
pub mod sqlite_content_repository;

/// SQLite implementation of `UserRepositoryPort`.
#[cfg(feature = "sqlite")]
pub mod sqlite_user_repository;

/// SQLite implementation of `WorkflowRepositoryPort`.
#[cfg(feature = "sqlite")]
pub mod sqlite_workflow_repository;

/// MySQL implementation of `ContentRepository`, `ContentListRepository`,
/// `MigrationManager`, and `SqlStore`.
#[cfg(feature = "mysql")]
pub mod mysql_content_repository;
