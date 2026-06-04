//! # Paladin Herald
//!
//! Concrete [`Herald`](paladin_core::platform::container::herald::Herald) output-formatter
//! adapters for the Paladin framework. The `Herald` trait (the output-formatting port) lives in
//! `paladin-core`; this crate provides the rendering implementations and their presentation
//! dependencies (`comfy-table`, `colored`) so the pure domain crate stays dependency-light.
//!
//! ## Formatters
//!
//! - [`JsonHerald`] — structured JSON output
//! - [`MarkdownHerald`] — Markdown / ANSI-colored terminal output
//! - [`TableHerald`] — compact table output
//!
//! Each implements `paladin_core::platform::container::herald::Herald` and can be used
//! anywhere a `dyn Herald` is expected.

#![allow(missing_docs)]

pub mod json_herald;
pub mod markdown_herald;
pub mod table_herald;

pub use json_herald::JsonHerald;
pub use markdown_herald::MarkdownHerald;
pub use table_herald::TableHerald;
