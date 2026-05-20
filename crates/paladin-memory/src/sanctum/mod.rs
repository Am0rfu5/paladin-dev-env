//! Sanctum adapters — vector / semantic memory storage.
//!
//! Re-exports available adapters based on enabled feature flags.

pub mod in_memory_adapter;
pub use in_memory_adapter::{InMemorySanctum, InMemorySanctumConfig};

#[cfg(feature = "qdrant")]
pub mod qdrant_adapter;
#[cfg(feature = "qdrant")]
pub use qdrant_adapter::QdrantSanctumAdapter;
