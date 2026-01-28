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

use crate::application::ports::output::arsenal_port::ArsenalRegistry;
use crate::application::ports::output::citadel_port::CitadelPort;
use crate::application::ports::output::garrison_port::GarrisonPort;
use crate::application::ports::output::llm_port::LlmPort;
use crate::application::use_cases::paladin::error::PaladinError;
use crate::config::application_settings::MCPServerConfig;
use crate::core::base::entity::node::Node;
use crate::core::platform::container::herald::Herald;
use crate::core::platform::container::paladin::{Paladin, PaladinData};
use crate::core::platform::container::paladin_config::{OutputFormat, PaladinConfig};
use crate::infrastructure::adapters::citadel::file_citadel::FileCitadel;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

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
    garrison: Option<Arc<dyn GarrisonPort>>,
    arsenal_registry: Option<Arc<dyn ArsenalRegistry>>,
    mcp_servers: Vec<MCPServerConfig>,
    citadel_port: Option<Arc<dyn CitadelPort>>,
    autosave_enabled: bool,
    state_dir: Option<String>,
    herald: Option<Arc<dyn Herald>>,
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
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: false,
            state_dir: None,
            herald: None,
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

    /// Attaches a Garrison memory system to the Paladin
    ///
    /// The Garrison enables the Paladin to maintain conversation context across
    /// multiple turns. It is optional for single-turn operations but required
    /// for multi-turn conversations.
    ///
    /// # Arguments
    ///
    /// * `garrison` - The Garrison port implementation to use for memory
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use paladin::application::ports::output::garrison_port::GarrisonPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>, garrison: Arc<dyn GarrisonPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a conversational assistant")
    ///     .with_garrison(garrison);
    /// # }
    /// ```
    pub fn with_garrison(mut self, garrison: Arc<dyn GarrisonPort>) -> Self {
        self.garrison = Some(garrison);
        self
    }

    /// Attaches an Arsenal registry to the Paladin for tool execution
    ///
    /// The Arsenal enables the Paladin to discover and invoke external tools
    /// through the Model Context Protocol (MCP). Tools can be STDIO-based
    /// (command-line) or SSE-based (HTTP).
    ///
    /// # Arguments
    ///
    /// * `registry` - The Arsenal registry implementation containing registered tools
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use paladin::application::ports::output::arsenal_port::ArsenalRegistry;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>, registry: Arc<dyn ArsenalRegistry>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a tool-using assistant")
    ///     .with_arsenal_registry(registry);
    /// # }
    /// ```
    pub fn with_arsenal_registry(mut self, registry: Arc<dyn ArsenalRegistry>) -> Self {
        self.arsenal_registry = Some(registry);
        self
    }

    /// Sets a Herald formatter for output formatting
    ///
    /// Herald formatters control how Paladin execution results are formatted for display.
    /// Built-in formatters include JSON, Markdown, and Table.
    ///
    /// # Arguments
    ///
    /// * `herald` - The Herald implementation to use for formatting
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::infrastructure::adapters::herald::JsonHerald;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let herald = Arc::new(JsonHerald::default());
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a helpful assistant")
    ///     .with_herald(herald);
    /// # }
    /// ```
    pub fn with_herald(mut self, herald: Arc<dyn Herald>) -> Self {
        self.herald = Some(herald);
        self
    }

    /// Adds an STDIO-based MCP server configuration
    ///
    /// STDIO servers are command-line tools that communicate via stdin/stdout
    /// using the Model Context Protocol.
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for the server
    /// * `command` - Command to execute
    /// * `args` - Command-line arguments
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are an assistant with web search")
    ///     .add_mcp_stdio("web_search", "uvx", &["mcp-web-search"]);
    /// # }
    /// ```
    pub fn add_mcp_stdio(
        mut self,
        name: impl Into<String>,
        command: impl Into<String>,
        args: &[&str],
    ) -> Self {
        self.mcp_servers.push(MCPServerConfig {
            name: name.into(),
            server_type: "stdio".to_string(),
            command: Some(command.into()),
            args: Some(args.iter().map(|s| s.to_string()).collect()),
            endpoint: None,
        });
        self
    }

    /// Adds an SSE-based MCP server configuration
    ///
    /// SSE servers are HTTP-based tools that communicate using Server-Sent Events
    /// and the Model Context Protocol.
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for the server
    /// * `endpoint` - HTTP endpoint URL
    ///
    /// # Example
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are an assistant with code analysis")
    ///     .add_mcp_sse("code_analyzer", "http://localhost:8080/mcp");
    /// # }
    /// ```
    pub fn add_mcp_sse(mut self, name: impl Into<String>, endpoint: impl Into<String>) -> Self {
        self.mcp_servers.push(MCPServerConfig {
            name: name.into(),
            server_type: "sse".to_string(),
            command: None,
            args: None,
            endpoint: Some(endpoint.into()),
        });
        self
    }

    /// Attaches a Citadel state persistence system to the Paladin
    ///
    /// The Citadel enables automatic saving and restoration of Paladin state,
    /// including configuration, execution history, and Garrison context.
    ///
    /// # Arguments
    ///
    /// * `citadel` - The Citadel port implementation to use for state persistence
    ///
    /// Enables state persistence by attaching a Citadel adapter.
    ///
    /// The Citadel system provides automatic state saving and restoration for
    /// Paladin agents. This enables fault tolerance, debugging, and long-running
    /// workflows that can survive system restarts.
    ///
    /// # Arguments
    ///
    /// * `citadel` - A Citadel adapter implementing the CitadelPort trait
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use paladin::infrastructure::adapters::citadel::file_citadel::FileCitadel;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Create a file-based Citadel adapter
    /// let citadel = Arc::new(FileCitadel::new("./paladin-states")?);
    ///
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a stateful assistant")
    ///     .with_citadel(citadel);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_citadel(mut self, citadel: Arc<dyn CitadelPort>) -> Self {
        self.citadel_port = Some(citadel);
        self
    }

    /// Enables automatic state saving after Paladin execution
    ///
    /// When enabled, the Paladin's state will be automatically saved to the
    /// Citadel after each execution completes successfully. This enables
    /// resumption of work and audit trails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a persistent assistant")
    ///     .enable_autosave();
    /// # }
    /// ```
    pub fn enable_autosave(mut self) -> Self {
        self.autosave_enabled = true;
        self
    }

    /// Sets the directory path for state persistence
    ///
    /// If not set, the default directory from configuration will be used.
    /// The directory will be created automatically if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - Directory path where state files will be saved
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .system_prompt("You are a persistent assistant")
    ///     .save_state_dir("./my_citadel");
    /// # }
    /// ```
    pub fn save_state_dir(mut self, path: impl Into<String>) -> Self {
        self.state_dir = Some(path.into());
        self
    }

    /// Restores a Paladin from a previously saved state
    ///
    /// Loads the Paladin configuration, execution history, and Garrison context
    /// from the Citadel, allowing resumption of previous work.
    ///
    /// # Arguments
    ///
    /// * `state_id` - UUID of the saved state to restore
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if state was successfully restored
    /// - `Err(PaladinError)` if state couldn't be loaded or Citadel is not configured
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use paladin::application::ports::output::citadel_port::CitadelPort;
    /// # use uuid::Uuid;
    /// # use std::sync::Arc;
    /// # async fn example(llm_port: Arc<dyn LlmPort>, citadel: Arc<dyn CitadelPort>) -> Result<(), Box<dyn std::error::Error>> {
    /// let state_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    /// let builder = PaladinBuilder::new(llm_port)
    ///     .with_citadel(citadel)
    ///     .restore_from(state_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_from(mut self, state_id: Uuid) -> Result<Self, PaladinError> {
        let citadel = self.citadel_port.as_ref().ok_or_else(|| {
            PaladinError::ConfigurationError(
                "Citadel port must be configured to restore state".to_string(),
            )
        })?;

        let state = citadel
            .load_paladin(state_id)
            .await
            .map_err(|e| PaladinError::ConfigurationError(format!("Failed to load state: {}", e)))?
            .ok_or_else(|| {
                PaladinError::ConfigurationError(format!("State not found: {}", state_id))
            })?;

        // Restore data from state
        self.data.system_prompt = state.paladin.node.system_prompt.clone();
        self.data.name = state.paladin.node.name.clone();
        self.data.user_name = state.paladin.node.user_name.clone();
        self.data.model = state.paladin.node.model.clone();
        self.data.temperature = state.paladin.node.temperature;
        self.data.max_loops = state.paladin.node.max_loops;
        self.data.stop_words = state.paladin.node.stop_words.clone();

        // Note: Config fields (retry_attempts, timeout_seconds) are not part of PaladinData
        // They would need to be stored separately if we want to restore them

        Ok(self)
    }

    /// Validates all configuration parameters
    ///
    /// # Validation Rules
    ///
    /// - `system_prompt` must be non-empty
    /// - `temperature` must be in range [0.0, 1.0]
    /// - `max_loops` must be in range [1, 100]
    /// - If `autosave_enabled` is true, `state_dir` must be set or Citadel must be configured
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

        // Validate autosave configuration
        if self.autosave_enabled && self.citadel_port.is_none() && self.state_dir.is_none() {
            return Err(PaladinError::ConfigurationError(
                "autosave_enabled requires either citadel_port or state_dir to be set".to_string(),
            ));
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
    pub fn build(mut self) -> Result<Paladin, PaladinError> {
        // Validate configuration
        self.validate()?;

        // Initialize FileCitadel if state_dir is provided but citadel_port is not
        if let Some(state_dir) = &self.state_dir
            && self.citadel_port.is_none()
        {
            let file_citadel = FileCitadel::new(PathBuf::from(state_dir)).map_err(|e| {
                PaladinError::ConfigurationError(format!("Failed to initialize FileCitadel: {}", e))
            })?;
            self.citadel_port = Some(Arc::new(file_citadel));
        }

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
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "".to_string(),
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: false,
            state_dir: None,
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(PaladinError::ConfigurationError(_))));
    }

    #[test]
    fn test_builder_validation_invalid_temperature() {
        let builder = PaladinBuilder {
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                temperature: 1.5,
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: false,
            state_dir: None,
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_validation_invalid_max_loops() {
        let builder = PaladinBuilder {
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                max_loops: 0,
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: false,
            state_dir: None,
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_with_citadel() {
        let citadel = Arc::new(MockCitadelPort);
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort))
            .system_prompt("Test")
            .with_citadel(citadel);

        assert!(builder.citadel_port.is_some());
    }

    #[test]
    fn test_builder_enable_autosave() {
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort))
            .system_prompt("Test")
            .enable_autosave();

        assert!(builder.autosave_enabled);
    }

    #[test]
    fn test_builder_save_state_dir() {
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort))
            .system_prompt("Test")
            .save_state_dir("./test_citadel");

        assert_eq!(builder.state_dir, Some("./test_citadel".to_string()));
    }

    #[test]
    fn test_builder_validation_autosave_without_citadel_or_dir() {
        let builder = PaladinBuilder {
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: true,
            state_dir: None,
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(PaladinError::ConfigurationError(_))));
    }

    #[test]
    fn test_builder_validation_autosave_with_citadel() {
        let builder = PaladinBuilder {
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: Some(Arc::new(MockCitadelPort)),
            autosave_enabled: true,
            state_dir: None,
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_validation_autosave_with_state_dir() {
        let builder = PaladinBuilder {
            _llm_port: Arc::new(MockLlmPort),
            data: PaladinData {
                system_prompt: "Test".to_string(),
                ..Default::default()
            },
            config: PaladinConfig::default(),
            garrison: None,
            arsenal_registry: None,
            mcp_servers: Vec::new(),
            citadel_port: None,
            autosave_enabled: true,
            state_dir: Some("./test".to_string()),
            herald: None,
        };

        let result = builder.validate();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_builder_restore_from_without_citadel() {
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort));
        let state_id = Uuid::new_v4();

        let result = builder.restore_from(state_id).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(PaladinError::ConfigurationError(_))));
    }

    #[tokio::test]
    async fn test_builder_restore_from_state_not_found() {
        let citadel = Arc::new(MockCitadelPort);
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort)).with_citadel(citadel);
        let state_id = Uuid::new_v4();

        let result = builder.restore_from(state_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_builder_restore_from_success() {
        let citadel = Arc::new(MockCitadelPortWithState);
        let builder = PaladinBuilder::new(Arc::new(MockLlmPort)).with_citadel(citadel);
        let state_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        let result = builder.restore_from(state_id).await;
        assert!(result.is_ok());

        let restored_builder = result.unwrap();
        assert_eq!(restored_builder.data.system_prompt, "Restored prompt");
        assert_eq!(restored_builder.data.name, "RestoredPaladin");
        assert_eq!(restored_builder.data.model, "gpt-4");
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

        fn get_capabilities(
            &self,
        ) -> crate::application::ports::output::llm_port::ProviderCapabilities {
            crate::application::ports::output::llm_port::ProviderCapabilities::default()
        }
    }

    // Mock CitadelPort for testing
    struct MockCitadelPort;

    #[async_trait::async_trait]
    impl CitadelPort for MockCitadelPort {
        async fn save_paladin(
            &self,
            _state: &crate::core::platform::container::citadel::PaladinState,
        ) -> Result<(), crate::application::errors::citadel_error::CitadelError> {
            Ok(())
        }

        async fn load_paladin(
            &self,
            _state_id: Uuid,
        ) -> Result<
            Option<crate::core::platform::container::citadel::PaladinState>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            Ok(None)
        }

        async fn save_battalion(
            &self,
            _state: &crate::core::platform::container::citadel::BattalionState,
        ) -> Result<(), crate::application::errors::citadel_error::CitadelError> {
            Ok(())
        }

        async fn load_battalion(
            &self,
            _state_id: Uuid,
        ) -> Result<
            Option<crate::core::platform::container::citadel::BattalionState>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            Ok(None)
        }

        async fn list_saved(
            &self,
        ) -> Result<
            Vec<crate::core::platform::container::citadel::StateSummary>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            Ok(vec![])
        }
    }

    // Mock CitadelPort that returns a state for testing restore
    struct MockCitadelPortWithState;

    #[async_trait::async_trait]
    impl CitadelPort for MockCitadelPortWithState {
        async fn save_paladin(
            &self,
            _state: &crate::core::platform::container::citadel::PaladinState,
        ) -> Result<(), crate::application::errors::citadel_error::CitadelError> {
            Ok(())
        }

        async fn load_paladin(
            &self,
            state_id: Uuid,
        ) -> Result<
            Option<crate::core::platform::container::citadel::PaladinState>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            if state_id == Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap() {
                // Create PaladinData
                let data = crate::core::platform::container::citadel::PaladinData {
                    system_prompt: "Restored prompt".to_string(),
                    name: "RestoredPaladin".to_string(),
                    user_name: "User".to_string(),
                    model: "gpt-4".to_string(),
                    temperature: 0.8,
                    max_loops: 5,
                    stop_words: vec![],
                    status: crate::core::platform::container::citadel::PaladinStatus::Idle,
                };

                // Create Paladin (Node<PaladinData>)
                let paladin = Node::new(data, Some("RestoredPaladin".to_string()));

                Ok(Some(
                    crate::core::platform::container::citadel::PaladinState {
                        paladin,
                        garrison: vec![],
                        execution_history: vec![],
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                        schema_version: "1.0.0".to_string(),
                    },
                ))
            } else {
                Ok(None)
            }
        }

        async fn save_battalion(
            &self,
            _state: &crate::core::platform::container::citadel::BattalionState,
        ) -> Result<(), crate::application::errors::citadel_error::CitadelError> {
            Ok(())
        }

        async fn load_battalion(
            &self,
            _state_id: Uuid,
        ) -> Result<
            Option<crate::core::platform::container::citadel::BattalionState>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            Ok(None)
        }

        async fn list_saved(
            &self,
        ) -> Result<
            Vec<crate::core::platform::container::citadel::StateSummary>,
            crate::application::errors::citadel_error::CitadelError,
        > {
            Ok(vec![])
        }
    }
}
