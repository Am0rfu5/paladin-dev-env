//! PaladinBuilder - Fluent builder for creating Paladin instances with validation
//!
//! This module provides a builder pattern implementation for constructing Paladin entities
//! with compile-time safety and runtime validation of configuration parameters.
//!
//! # Examples
//!
//! ```rust,no_run
//! use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
//! use paladin::application::ports::output::llm_port::LlmPort;
//! use paladin::core::platform::container::paladin_config::OutputFormat;
//! use std::sync::Arc;
//!
//! # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
//! let paladin = PaladinBuilder::new(llm_port)
//!     .system_prompt("You are a helpful coding assistant")
//!     .name("CodePaladin")
//!     .user_name("Developer")
//!     .model("gpt-4")
//!     .temperature(0.7)
//!     .max_loops(5)
//!     .add_stop_word("STOP")
//!     .retry_attempts(3)
//!     .timeout_seconds(300)
//!     .enable_planning(true)
//!     .output_format(OutputFormat::Json)
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use crate::application::ports::output::llm_port::LlmPort;
use crate::application::use_cases::paladin::error::PaladinError;
use crate::core::base::entity::node::Node;
use crate::core::platform::container::paladin::{Paladin, PaladinData};
use crate::core::platform::container::paladin_config::{OutputFormat, PaladinConfig};
use std::sync::Arc;

/// Builder for creating Paladin instances with validation
///
/// The builder provides a fluent interface for constructing Paladins with comprehensive
/// validation of all configuration parameters. It enforces constraints like:
/// - Non-empty system prompts
/// - Temperature in range [0.0, 1.0]
/// - Max loops in range [1, 100]
///
/// # Example
///
/// ```rust,no_run
/// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
/// # use paladin::application::ports::output::llm_port::LlmPort;
/// # use std::sync::Arc;
/// # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
/// let paladin = PaladinBuilder::new(llm_port)
///     .system_prompt("You are an AI assistant")
///     .name("Assistant")
///     .model("gpt-4")
///     .temperature(0.8)
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct PaladinBuilder {
    _llm_port: Arc<dyn LlmPort>, // Stored for future use, not currently used in build()
    data: PaladinData,
    config: PaladinConfig,
}

impl PaladinBuilder {
    /// Creates a new PaladinBuilder with default values
    ///
    /// # Arguments
    ///
    /// * `llm_port` - The LLM port implementation to use for this Paladin
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port);
    /// # }
    /// ```
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self {
        Self {
            _llm_port: llm_port,
            data: PaladinData::default(),
            config: PaladinConfig::default(),
        }
    }

    /// Sets the system prompt that defines the Paladin's behavior and personality
    ///
    /// # Arguments
    ///
    /// * `prompt` - The system prompt (must be non-empty)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a helpful coding assistant");
    /// # }
    /// ```
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.data.system_prompt = prompt.into();
        self
    }

    /// Sets the name of the Paladin
    ///
    /// # Arguments
    ///
    /// * `name` - The Paladin's name
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .name("CodeAssistant");
    /// # }
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.data.name = name.into();
        self
    }

    /// Sets the user name that the Paladin will interact with
    ///
    /// # Arguments
    ///
    /// * `user_name` - The user's name
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .user_name("Developer");
    /// # }
    /// ```
    pub fn user_name(mut self, user_name: impl Into<String>) -> Self {
        self.data.user_name = user_name.into();
        self
    }

    /// Sets the LLM model to use
    ///
    /// # Arguments
    ///
    /// * `model` - The model identifier (e.g., "gpt-4", "gpt-3.5-turbo")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .model("gpt-4");
    /// # }
    /// ```
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.data.model = model.into();
        self
    }

    /// Sets the temperature for LLM generation (controls randomness)
    ///
    /// # Arguments
    ///
    /// * `temperature` - Value between 0.0 (deterministic) and 1.0 (random)
    ///
    /// # Validation
    ///
    /// Temperature must be in range [0.0, 1.0] or validation will fail during build()
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .temperature(0.7);
    /// # }
    /// ```
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.data.temperature = temperature;
        self
    }

    /// Sets the maximum number of reasoning loops
    ///
    /// # Arguments
    ///
    /// * `max_loops` - Maximum iterations (must be between 1 and 100)
    ///
    /// # Validation
    ///
    /// max_loops must be in range [1, 100] or validation will fail during build()
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .max_loops(5);
    /// # }
    /// ```
    pub fn max_loops(mut self, max_loops: u32) -> Self {
        self.data.max_loops = max_loops;
        self
    }

    /// Adds a stop word that will halt execution when detected in LLM output
    ///
    /// # Arguments
    ///
    /// * `word` - The stop word to add
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .add_stop_word("STOP")
    ///     .add_stop_word("END");
    /// # }
    /// ```
    pub fn add_stop_word(mut self, word: impl Into<String>) -> Self {
        self.data.stop_words.push(word.into());
        self
    }

    /// Sets the number of retry attempts for failed LLM calls
    ///
    /// # Arguments
    ///
    /// * `attempts` - Number of retries (default: 3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .retry_attempts(5);
    /// # }
    /// ```
    pub fn retry_attempts(mut self, attempts: u32) -> Self {
        self.config.retry_attempts = attempts;
        self
    }

    /// Sets the execution timeout in seconds
    ///
    /// # Arguments
    ///
    /// * `seconds` - Timeout duration (default: 300)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .timeout_seconds(600);
    /// # }
    /// ```
    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.config.timeout_seconds = seconds;
        self
    }

    /// Enables or disables planning mode
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable planning (default: false)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .enable_planning(true);
    /// # }
    /// ```
    pub fn enable_planning(mut self, enabled: bool) -> Self {
        self.config.enable_planning = enabled;
        self
    }

    /// Sets the output format for responses
    ///
    /// # Arguments
    ///
    /// * `format` - The desired output format
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use paladin::core::platform::container::paladin_config::OutputFormat;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .output_format(OutputFormat::Json);
    /// # }
    /// ```
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.config.output_format = format;
        self
    }

    /// Validates all configuration parameters
    ///
    /// # Validation Rules
    ///
    /// - `system_prompt` must be non-empty
    /// - `temperature` must be in range [0.0, 1.0]
    /// - `max_loops` must be in range [1, 100]
    ///
    /// # Returns
    ///
    /// Ok(()) if all validations pass, Err(PaladinError::ConfigurationError) otherwise
    fn validate(&self) -> Result<(), PaladinError> {
        // Validate system_prompt is non-empty
        if self.data.system_prompt.trim().is_empty() {
            return Err(PaladinError::ConfigurationError(
                "system prompt cannot be empty".to_string(),
            ));
        }

        // Validate temperature is in [0.0, 1.0]
        if !(0.0..=1.0).contains(&self.data.temperature) {
            return Err(PaladinError::ConfigurationError(format!(
                "temperature must be between 0.0 and 1.0, got {}",
                self.data.temperature
            )));
        }

        // Validate max_loops is in [1, 100]
        if self.data.max_loops < 1 || self.data.max_loops > 100 {
            return Err(PaladinError::ConfigurationError(format!(
                "max_loops must be between 1 and 100, got {}",
                self.data.max_loops
            )));
        }

        Ok(())
    }

    /// Builds and returns a validated Paladin instance
    ///
    /// # Returns
    ///
    /// - `Ok(Paladin)` if validation succeeds
    /// - `Err(PaladinError::ConfigurationError)` if validation fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
    /// let paladin = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are an AI assistant")
    ///     .model("gpt-4")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<Paladin, PaladinError> {
        // Validate configuration
        self.validate()?;

        // Create Paladin using Node pattern with name
        let name = if self.data.name.is_empty() {
            None
        } else {
            Some(self.data.name.clone())
        };

        Ok(Node::new(self.data, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_validation_empty_prompt() {
        let builder = PaladinBuilder {
            llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "".to_string(),
                ..Default::default()
            },
            config: PaladinConfig::default(),
        };

        let result = builder.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(PaladinError::ConfigurationError(_))));
    }

    #[test]
    fn test_builder_validation_invalid_temperature() {
        let builder = PaladinBuilder {
            llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                temperature: 1.5,
                ..Default::default()
            },
            config: PaladinConfig::default(),
        };

        let result = builder.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_validation_invalid_max_loops() {
        let builder = PaladinBuilder {
            llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                max_loops: 0,
                ..Default::default()
            },
            config: PaladinConfig::default(),
        };

        let result = builder.validate();
        assert!(result.is_err());
    }

    // Mock for internal tests
    struct MockLlmPort;

    #[async_trait::async_trait]
    impl LlmPort for MockLlmPort {
        async fn generate(
            &self,
            _request: crate::application::ports::output::llm_port::LlmRequest,
        ) -> Result<
            crate::application::ports::output::llm_port::LlmResponse,
            crate::application::ports::output::llm_port::LlmError,
        > {
            unimplemented!()
        }

        async fn generate_stream(
            &self,
            _request: crate::application::ports::output::llm_port::LlmRequest,
        ) -> Result<
            Box<
                dyn futures::Stream<
                        Item = Result<
                            crate::application::ports::output::llm_port::StreamingResponse,
                            crate::application::ports::output::llm_port::LlmError,
                        >,
                    > + Send,
            >,
            crate::application::ports::output::llm_port::LlmError,
        > {
            unimplemented!()
        }

        async fn validate_model(
            &self,
            _model: &str,
        ) -> Result<bool, crate::application::ports::output::llm_port::LlmError> {
            Ok(true)
        }

        async fn get_available_models(
            &self,
        ) -> Result<Vec<String>, crate::application::ports::output::llm_port::LlmError> {
            Ok(vec![])
        }

        fn get_provider_name(&self) -> &'static str {
            "Mock"
        }
    }
}
