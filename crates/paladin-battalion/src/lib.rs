//! # paladin-battalion
//!
//! Multi-agent orchestration runtime for the Paladin framework.
//!
//! This crate provides all eight Battalion execution patterns and the Commander
//! strategy router. It depends only on `paladin-core` (domain types) and
//! `paladin-ports` (port trait contracts) — never on infrastructure SDKs,
//! database drivers, or LLM provider libraries.
//!
//! ## Patterns
//!
//! - [`formation_service`] — Sequential execution: output of agent N feeds input of agent N+1
//! - [`phalanx_service`] — Concurrent execution: all agents run in parallel
//! - [`campaign_service`] — DAG/graph execution: topologically sorted dependency graph
//! - [`chain_of_command_service`] — Hierarchical delegation: commander delegates to sub-agents
//! - [`conclave_execution_service`] — Mixture-of-experts synthesis
//! - [`council_service`] — Multi-agent discussion and consensus
//! - [`grove_service`] — Intelligent semantic routing
//! - [`maneuver_service`] — Flow DSL execution
//! - [`commander`] — Strategy auto-detection router
//!
//! ## Utilities
//!
//! - [`error_aggregation`] — Collect and summarise errors across parallel agent runs
//! - [`flow_visualizer`] — Generate visual representations of battalion execution flows
//! - [`retry`] — Exponential back-off retry helper

#![warn(missing_docs)]

// Execution services
pub mod campaign_service;
pub mod chain_of_command_service;
pub mod commander;
pub mod conclave_execution_service;
pub mod council_service;
pub mod formation_service;
pub mod grove_service;
pub mod maneuver_service;
pub mod phalanx_service;

// Utility / support
pub mod error_aggregation;
pub mod flow_visualizer;
pub mod retry;

// Internal helpers
pub(crate) mod in_memory_registry;
