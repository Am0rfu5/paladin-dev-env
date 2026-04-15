//! Garrison Adapters - Storage Implementations
//!
//! This module contains concrete implementations of the GarrisonPort trait
//! for different storage backends.

pub mod in_memory_garrison;
pub mod sqlite_garrison;
#[cfg(feature = "content-processing")]
pub mod token_counter;

pub use in_memory_garrison::InMemoryGarrison;
pub use sqlite_garrison::SqliteGarrison;
#[cfg(feature = "content-processing")]
pub use token_counter::{TiktokenCounter, TokenCounter, TokenCounterFactory};
