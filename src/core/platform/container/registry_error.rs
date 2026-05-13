//! Paladin Registry Error Types
//!
//! This module defines [`RegistryError`], a pure domain error type for Paladin
//! registry operations. The `application` layer re-exports it from here.

use thiserror::Error;

/// Errors that can occur during Paladin registry operations
#[derive(Debug, Error, Clone)]
pub enum RegistryError {
    /// Attempted to register a Paladin with an ID that already exists
    #[error("Paladin ID already registered: {0}")]
    DuplicateId(String),

    /// Invalid Paladin ID provided (e.g., empty string)
    #[error("Invalid Paladin ID: {0}")]
    InvalidId(String),

    /// Failed to access the registry (internal error)
    #[error("Registry access failed: {0}")]
    AccessFailed(String),
}
