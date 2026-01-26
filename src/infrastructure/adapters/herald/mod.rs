//! Herald output formatter adapters
//!
//! This module contains concrete implementations of the Herald trait for
//! various output formats (JSON, Markdown, Table).

pub mod json_herald;
pub mod markdown_herald;

pub use json_herald::JsonHerald;
pub use markdown_herald::MarkdownHerald;
