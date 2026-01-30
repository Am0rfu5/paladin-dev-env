//! Sanctum (long-term memory) adapter implementations
//!
//! This module contains different storage backend implementations for the Sanctum system:
//! - InMemorySanctum: Fast, ephemeral storage for development and testing
//! - QdrantSanctum: Production-grade vector database storage (optional feature)

mod in_memory_adapter;

pub use in_memory_adapter::InMemorySanctum;
