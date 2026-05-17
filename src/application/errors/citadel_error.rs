//! Citadel Error Types
//!
//! Re-exports [`CitadelError`] from `paladin-core` where it now lives alongside
//! the other domain error types (`garrison_error`, `registry_error`, etc.).

pub use paladin_core::platform::container::citadel_error::CitadelError;
