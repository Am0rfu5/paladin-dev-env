//! CLI module for Paladin
//!
//! This module provides the command-line interface for the Paladin framework,
//! including interactive wizards, setup validation, and rich terminal output.
//!
//! # Module Organization
//!
//! The CLI has been consolidated into `src/application/cli/` following hexagonal
//! architecture principles. All CLI functionality is now part of the application
//! layer, with clear separation from domain logic and infrastructure adapters.
//!
//! ## Submodules
//!
//! - `commands`: Command implementations (agent, arsenal, battalion, maneuver, etc.)
//! - `config`: Configuration loading and validation
//! - `error`: Unified error handling with `CliError` and `CliResult`
//! - `formatters`: Output formatting (JSON, Markdown, tables)
//! - `interactive`: Interactive prompts and TTY utilities
//! - `templates`: File templates for scaffolding
//!
//! ## Migration Note
//!
//! Previously, CLI code was split between `src/cli/` (old implementation) and
//! `src/application/cli/` (Epic 18 foundation). As of Epic 17.5, all CLI code
//! has been consolidated into `src/application/cli/` with a unified error type.
//!
//! ## Usage
//!
//! ```rust
//! use paladin::application::cli::{CliError, CliResult};
//! use paladin::application::cli::commands::agent;
//!
//! async fn run_agent_command() -> CliResult<()> {
//!     // Your CLI logic here
//!     Ok(())
//! }
//! ```

pub mod commands;
pub mod config;
pub mod error;
mod error_impl; // Implementation details
pub mod formatters;
pub mod interactive;
pub mod templates;

#[cfg(test)]
mod tests;

pub use error::{CliError, CliResult};
