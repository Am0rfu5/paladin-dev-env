//! # Paladin Herald
//!
//! Concrete [`Herald`](paladin_core::platform::container::herald::Herald) output-formatter
//! adapters for the Paladin framework. The `Herald` trait (the output-formatting port) lives in
//! `paladin-core`; this crate provides the rendering implementations and their presentation
//! dependencies (`comfy-table`, `colored`) so the pure domain crate stays dependency-light.
//!
//! ## Formatters
//!
//! - [`JsonHerald`] — structured JSON output. Always available.
//! - [`MarkdownHerald`] — Markdown terminal output. Always available; its *coloured* rendering
//!   path requires the `color` feature (gates the `colored` dependency) — with the feature off,
//!   it renders the same plain-text path it already takes when `include_colors` is `false`.
//! - `TableHerald` — compact table output. Requires the `table` feature (gates the
//!   `comfy-table` dependency).
//!
//! Each implements `paladin_core::platform::container::herald::Herald` and can be used
//! anywhere a `dyn Herald` is expected.

#![warn(missing_docs)]

pub mod json_herald;
pub mod markdown_herald;
#[cfg(feature = "table")]
pub mod table_herald;

pub use json_herald::JsonHerald;
pub use markdown_herald::MarkdownHerald;
#[cfg(feature = "table")]
pub use table_herald::TableHerald;
