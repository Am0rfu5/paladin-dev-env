//! Garrison Adapters - Storage Implementations
//!
//! This module contains concrete implementations of the GarrisonPort trait
//! for different storage backends.

pub mod in_memory_garrison;

pub use in_memory_garrison::InMemoryGarrison;
