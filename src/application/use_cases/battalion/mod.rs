//! Battalion Use Cases
//!
//! This module re-exports all battalion orchestration types from the
//! `paladin-battalion` crate.  It is kept as a thin shim so that existing
//! `use paladin::application::use_cases::battalion::*` import paths continue
//! to resolve without modification.

pub use paladin_battalion::campaign_service;
pub use paladin_battalion::chain_of_command_service;
pub use paladin_battalion::commander;
pub use paladin_battalion::conclave_execution_service;
pub use paladin_battalion::council_service;
pub use paladin_battalion::error_aggregation;
pub use paladin_battalion::flow_visualizer;
pub use paladin_battalion::formation_service;
pub use paladin_battalion::grove_service;
pub use paladin_battalion::maneuver_service;
pub use paladin_battalion::phalanx_service;
pub use paladin_battalion::retry;
