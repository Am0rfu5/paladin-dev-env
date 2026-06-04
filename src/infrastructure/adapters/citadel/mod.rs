//! Citadel state-persistence adapters — re-exported from `paladin-memory`.
//!
//! The `FileCitadel` implementation lives in `paladin_memory::citadel::file_citadel`;
//! the facade re-exports it so the `crate::infrastructure::adapters::citadel::file_citadel`
//! path remains stable for the composition root and examples.
pub use paladin_memory::citadel::file_citadel;
