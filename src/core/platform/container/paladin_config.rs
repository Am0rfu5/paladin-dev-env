//! Paladin Configuration Types
//!
//! This module defines runtime configuration for Paladin execution, including
//! retry behavior, timeouts, planning capabilities, and output formatting.

use serde::{Deserialize, Serialize};

/// Output format for Paladin responses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON-formatted output
    Json,
    /// Structured output with metadata
    Structured,
}

/// Runtime configuration for Paladin execution
///
/// Controls how a Paladin executes tasks, including retry logic, timeouts,
/// and planning capabilities.
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::paladin_config::{PaladinConfig, OutputFormat};
///
/// let config = PaladinConfig::builder()
///     .retry_attempts(5)
///     .timeout_seconds(600)
///     .enable_planning(true)
///     .output_format(OutputFormat::Json)
///     .build()
///     .unwrap();
///
/// assert_eq!(config.retry_attempts, 5);
/// assert_eq!(config.timeout_seconds, 600);
/// assert!(config.enable_planning);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinConfig {
    /// Number of times to retry on transient failures
    pub retry_attempts: u32,

    /// Maximum execution time in seconds before timeout
    pub timeout_seconds: u64,

    /// Whether to enable planning phase before execution
    pub enable_planning: bool,

    /// Optional custom prompt for the planning phase
    pub planning_prompt: Option<String>,

    /// Format for output responses
    pub output_format: OutputFormat,
}

impl Default for PaladinConfig {
    fn default() -> Self {
        Self {
            retry_attempts: 3,
            timeout_seconds: 300,
            enable_planning: false,
            planning_prompt: None,
            output_format: OutputFormat::Text,
        }
    }
}

impl PaladinConfig {
    /// Create a new builder for PaladinConfig
    pub fn builder() -> PaladinConfigBuilder {
        PaladinConfigBuilder::default()
    }
}

/// Builder for PaladinConfig
///
/// Provides a fluent interface for constructing PaladinConfig instances.
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::paladin_config::{PaladinConfig, OutputFormat};
///
/// let config = PaladinConfig::builder()
///     .retry_attempts(3)
///     .timeout_seconds(300)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct PaladinConfigBuilder {
    retry_attempts: Option<u32>,
    timeout_seconds: Option<u64>,
    enable_planning: Option<bool>,
    planning_prompt: Option<String>,
    output_format: Option<OutputFormat>,
}

impl PaladinConfigBuilder {
    /// Set the number of retry attempts
    pub fn retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = Some(attempts);
        self
    }

    /// Set the timeout in seconds
    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Enable or disable planning phase
    pub fn enable_planning(mut self, enable: bool) -> Self {
        self.enable_planning = Some(enable);
        self
    }

    /// Set a custom planning prompt
    pub fn planning_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.planning_prompt = Some(prompt.into());
        self
    }

    /// Set the output format
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Build the PaladinConfig
    pub fn build(self) -> Result<PaladinConfig, String> {
        Ok(PaladinConfig {
            retry_attempts: self.retry_attempts.unwrap_or(3),
            timeout_seconds: self.timeout_seconds.unwrap_or(300),
            enable_planning: self.enable_planning.unwrap_or(false),
            planning_prompt: self.planning_prompt,
            output_format: self.output_format.unwrap_or(OutputFormat::Text),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paladin_config_defaults() {
        let config = PaladinConfig::default();
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.timeout_seconds, 300);
        assert!(!config.enable_planning);
        assert_eq!(config.planning_prompt, None);
        assert_eq!(config.output_format, OutputFormat::Text);
    }

    #[test]
    fn test_paladin_config_builder() {
        let config = PaladinConfig::builder()
            .retry_attempts(5)
            .timeout_seconds(600)
            .enable_planning(true)
            .planning_prompt("Think step by step")
            .output_format(OutputFormat::Json)
            .build()
            .unwrap();

        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.timeout_seconds, 600);
        assert!(config.enable_planning);
        assert_eq!(
            config.planning_prompt,
            Some("Think step by step".to_string())
        );
        assert_eq!(config.output_format, OutputFormat::Json);
    }

    #[test]
    fn test_paladin_config_builder_with_defaults() {
        let config = PaladinConfig::builder().retry_attempts(5).build().unwrap();

        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.timeout_seconds, 300); // default
        assert!(!config.enable_planning); // default
    }

    #[test]
    fn test_output_format_variants() {
        let text = OutputFormat::Text;
        let json = OutputFormat::Json;
        let structured = OutputFormat::Structured;

        assert_eq!(text, OutputFormat::Text);
        assert_eq!(json, OutputFormat::Json);
        assert_eq!(structured, OutputFormat::Structured);
        assert_ne!(text, json);
    }
}
