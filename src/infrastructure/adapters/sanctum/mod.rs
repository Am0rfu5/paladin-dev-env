//! Sanctum Adapters — re-exported from `paladin-memory`.
//!
//! All sanctum implementations have been extracted into the `paladin-memory`
//! crate. This module provides backward-compatible re-exports.

pub use paladin_memory::sanctum::InMemorySanctum;

#[cfg(feature = "qdrant")]
pub use paladin_memory::sanctum::QdrantSanctumAdapter;
