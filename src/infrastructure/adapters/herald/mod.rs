//! Herald output formatter adapters — re-exported from the `paladin-herald` crate.
//!
//! The concrete `Herald` implementations (JSON, Markdown, Table) live in `paladin-herald`
//! so their presentation dependencies (`comfy-table`, `colored`) stay out of the pure
//! domain crate. The facade re-exports them here so the
//! `crate::infrastructure::adapters::herald::…` paths remain stable for the composition
//! root, examples, and tests.

pub use paladin_herald::{JsonHerald, MarkdownHerald, TableHerald};
pub use paladin_herald::{json_herald, markdown_herald, table_herald};
