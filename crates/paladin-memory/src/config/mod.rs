//! Per-domain configuration types for the paladin-memory crate.

pub mod garrison;
pub mod rag;
pub mod sanctum;

pub use garrison::GarrisonSettings;
pub use rag::{MemoryExtractionConfig, MemoryExtractionStrategy, RagConfig, RetrievalTrigger};
pub use sanctum::{QdrantSanctumConfig, SanctumAdapterType, SanctumConfig};
