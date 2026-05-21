//! Convenient re-exports of the most commonly used Paladin types.
//!
//! Import everything you need to build and run Paladin agents in one line:
//!
//! ```rust,no_run
//! use paladin::prelude::*;
//! ```
//!
//! This module re-exports the types used in the majority of Paladin programs.
//! For less common types, import them directly from their source modules.
//!
//! ## Example
//!
//! ```rust,no_run
//! use paladin::prelude::*;
//! // Verify core types are in scope
//! let _status = PaladinStatus::Idle;
//! ```

pub use crate::{
    // Tool / arsenal types
    Armament,
    ArsenalPort,
    ArsenalRegistry,
    // Battalion / orchestration types
    BattalionConfig,
    BattalionError,
    BattalionResult,
    Campaign,
    ChainOfCommand,
    CommanderBuilder,
    CouncilBuilder,
    Formation,
    // Memory port types
    GarrisonError,
    GarrisonPort,
    GroveBuilder,
    // Memory adapter types (always available)
    InMemoryGarrison,
    InMemorySanctum,
    // LLM port and request/response types
    LlmError,
    LlmPort,
    LlmRequest,
    LlmResponse,
    // Agent types
    Paladin,
    PaladinBuilder,
    PaladinConfig,
    PaladinData,
    PaladinError,
    // Execution result types
    PaladinResult,
    PaladinStatus,
    Phalanx,
    SanctumError,
    SanctumPort,
    StopReason,
};
