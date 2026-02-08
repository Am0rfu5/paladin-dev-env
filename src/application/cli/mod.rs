//! CLI module for Paladin
//!
//! This module provides the command-line interface for the Paladin framework,
//! including interactive wizards, setup validation, and rich terminal output.

pub mod commands;
pub mod error;
pub mod formatters;
pub mod interactive;
pub mod templates;

pub use error::{CliError, CliResult};
