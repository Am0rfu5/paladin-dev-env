#![allow(missing_docs)]
//! Content Port
//!
//! A port that defines how the application fetches content from external sources or
//! ingests content from the user. This could be an HTTP API, a database, a file system,
//! or any other source of content.

use paladin_core::platform::container::content::ContentItem;

/// Port abstracting how content is fetched from and ingested into external sources.
///
/// # Examples
///
/// ```rust
/// use paladin_ports::input::content_input_port::ContentIngestionPort;
/// use paladin_core::platform::container::content::ContentItem;
///
/// fn round_trip(source: &dyn ContentIngestionPort, item: ContentItem) -> Result<(), String> {
///     let fetched = source.fetch_content(item)?;
///     source.ingest_content(fetched)
/// }
/// ```
pub trait ContentIngestionPort {
    /// Fetches (and optionally transforms) a content item from its source.
    fn fetch_content(&self, content: ContentItem) -> Result<ContentItem, String>;
    /// Ingests a content item into the application's storage/processing pipeline.
    fn ingest_content(&self, content: ContentItem) -> Result<(), String>;
}
