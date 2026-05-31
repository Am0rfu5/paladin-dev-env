//! Content processors that bridge the content pipeline into AI agent execution.
//!
//! This module implements the **content → agent** direction of the
//! Paladin integration: it turns a [`ContentItem`] into a prompt, runs it
//! through an agent (a single Paladin or a Battalion of Paladins), and parses
//! the agent's response back into a
//! [`ContentProcessingResult`](crate::application::services::orchestration::types::ContentProcessingResult)
//! with enrichment metadata.
//!
//! These processors live in the root crate beside the
//! [`ContentProcessor`](crate::application::services::orchestration::types::ContentProcessor)
//! trait (rather than in `paladin-content`) because that trait and
//! [`OrchestratorError`](crate::application::services::orchestration::types::OrchestratorError)
//! are defined here, and the root crate already depends on both
//! `paladin-content` and `paladin-battalion`. Implementing them in
//! `paladin-content` would create a circular crate dependency.
//!
//! [`ContentItem`]: crate::core::platform::container::content::ContentItem

pub mod battalion_processor;
pub mod paladin_processor;

pub use battalion_processor::{BattalionContentProcessor, BattalionPattern};
pub use paladin_processor::PaladinContentProcessor;

use crate::core::platform::container::content::{ContentItem, ContentType};

/// Strategy for parsing an agent's textual response into structured enrichment.
///
/// The default is [`OutputParsing::RawText`], which stores the agent's
/// response verbatim. [`OutputParsing::Json`] attempts to parse the response
/// as JSON; a malformed response degrades gracefully (see the processor docs)
/// rather than panicking or losing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputParsing {
    /// Store the agent's response text verbatim under `result_data`.
    #[default]
    RawText,
    /// Parse the agent's response as JSON into `result_data`.
    Json,
}

impl OutputParsing {
    /// Returns a stable string identifier for this strategy (for metadata).
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputParsing::RawText => "raw_text",
            OutputParsing::Json => "json",
        }
    }
}

/// A configurable prompt template that interpolates a [`ContentItem`] into a
/// prompt string for an agent.
///
/// The template supports two placeholders:
/// - `{title}` — the content item's title (empty string if none).
/// - `{content}` — the extracted textual body of the content item.
///
/// [`ContentItem`]: crate::core::platform::container::content::ContentItem
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    template: String,
}

impl Default for PromptTemplate {
    fn default() -> Self {
        Self {
            template: "Analyze the following content and provide your response.\n\nTitle: {title}\n\nContent:\n{content}".to_string(),
        }
    }
}

impl PromptTemplate {
    /// Creates a new prompt template from a template string.
    ///
    /// The string may contain the `{title}` and `{content}` placeholders, which
    /// are substituted when [`PromptTemplate::render`] is called.
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    /// Renders the template for the given content item.
    pub fn render(&self, content: &ContentItem) -> String {
        let title = content.title().cloned().unwrap_or_default();
        let body = extract_text(content);
        self.template
            .replace("{title}", &title)
            .replace("{content}", &body)
    }
}

/// Extracts a textual representation of a content item for prompting.
///
/// For [`ContentType::Text`] the inline content is used (falling back to the
/// file path when only a path is present). Non-text content types are rendered
/// as a short descriptive placeholder so the agent still receives context.
pub(crate) fn extract_text(content: &ContentItem) -> String {
    match content.content() {
        ContentType::Text(text) => text
            .content
            .clone()
            .or_else(|| text.path.clone())
            .unwrap_or_default(),
        ContentType::Video(_) => "[video content]".to_string(),
        ContentType::Audio(_) => "[audio content]".to_string(),
        ContentType::Image(_) => "[image content]".to_string(),
    }
}
