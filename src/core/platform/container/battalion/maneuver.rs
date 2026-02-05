//! Maneuver Domain Model
//!
//! The Maneuver pattern provides string-based workflow orchestration for Battalion agents.
//! It uses a simple DSL with `->` for sequential and `,` for parallel execution.

use crate::core::platform::container::battalion::parser::FlowExpression;
use crate::core::platform::container::paladin::Paladin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Error strategy for handling agent failures during execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ErrorStrategy {
    /// Stop execution immediately on first error
    #[default]
    FailFast,
    /// Continue parallel branches but fail sequential chains
    ContinueParallel,
    /// Log errors but continue execution
    IgnoreErrors,
}

/// Output format for aggregating results from multiple agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    /// Concatenate outputs as plain text
    #[default]
    Concatenate,
    /// Aggregate outputs as JSON array
    JsonArray,
}

/// Configuration for Maneuver execution behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverConfig {
    /// How to handle errors during execution
    pub error_strategy: ErrorStrategy,

    /// How to format aggregated outputs
    pub output_format: OutputFormat,

    /// Whether to pass output from one agent as input to the next in sequences
    pub pass_output_as_input: bool,

    /// Maximum execution time for the entire workflow
    pub timeout: Option<Duration>,

    /// Whether to collect detailed timing metrics per agent
    pub collect_timing_metrics: bool,

    /// Whether to collect detailed observability data
    pub detailed_observability: bool,
}

impl Default for ManeuverConfig {
    fn default() -> Self {
        ManeuverConfig {
            error_strategy: ErrorStrategy::FailFast,
            output_format: OutputFormat::Concatenate,
            pass_output_as_input: true,
            timeout: Some(Duration::from_secs(300)), // 5 minutes
            collect_timing_metrics: true,
            detailed_observability: false,
        }
    }
}

impl ManeuverConfig {
    /// Create a new ManeuverConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the error strategy
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Set the output format
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Set whether to pass output as input in sequences
    pub fn with_pass_output_as_input(mut self, pass: bool) -> Self {
        self.pass_output_as_input = pass;
        self
    }

    /// Set the timeout duration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Disable timeout
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Enable timing metrics collection
    pub fn with_timing_metrics(mut self, enabled: bool) -> Self {
        self.collect_timing_metrics = enabled;
        self
    }

    /// Enable detailed observability
    pub fn with_detailed_observability(mut self, enabled: bool) -> Self {
        self.detailed_observability = enabled;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if matches!(self.timeout, Some(timeout) if timeout.as_secs() == 0) {
            return Err("Timeout must be greater than zero".to_string());
        }
        Ok(())
    }
}

/// Maneuver domain model - orchestrates multiple Paladin agents via flow DSL
#[derive(Debug, Clone)]
pub struct Maneuver {
    /// Unique name for this maneuver
    pub name: String,

    /// Map of agent names to Paladin instances
    pub agents: HashMap<String, Paladin>,

    /// Flow expression defining the execution pattern
    pub flow: FlowExpression,

    /// Configuration for execution behavior
    pub config: ManeuverConfig,
}

impl Maneuver {
    /// Create a new Maneuver
    pub fn new(
        name: impl Into<String>,
        agents: HashMap<String, Paladin>,
        flow: FlowExpression,
        config: ManeuverConfig,
    ) -> Result<Self, ManeuverError> {
        let name = name.into();

        // Validate configuration
        config
            .validate()
            .map_err(ManeuverError::ValidationError)?;

        let maneuver = Maneuver {
            name,
            agents,
            flow,
            config,
        };

        // Validate that all agents in flow exist
        maneuver.validate()?;

        Ok(maneuver)
    }

    /// Validate that all agents referenced in the flow exist in the agents map
    pub fn validate(&self) -> Result<(), ManeuverError> {
        let flow_agents = self.flow.agent_names();

        for agent_name in &flow_agents {
            if !self.agents.contains_key(agent_name) {
                return Err(ManeuverError::AgentNotFound {
                    agent_name: agent_name.clone(),
                    available_agents: self.agents.keys().cloned().collect(),
                });
            }
        }

        // Validate max depth (5 levels)
        let depth = self.flow.depth();
        if depth > 5 {
            return Err(ManeuverError::ValidationError(format!(
                "Flow depth {} exceeds maximum of 5 levels",
                depth
            )));
        }

        // Validate max agent count (30 agents)
        let agent_count = self.flow.agent_count();
        if agent_count > 30 {
            return Err(ManeuverError::ValidationError(format!(
                "Flow contains {} agents, exceeds maximum of 30",
                agent_count
            )));
        }

        Ok(())
    }

    /// Get the depth of the flow expression tree
    pub fn depth(&self) -> usize {
        self.flow.depth()
    }

    /// Get the total number of agent nodes in the flow
    pub fn agent_count(&self) -> usize {
        self.flow.agent_count()
    }

    /// Get the maximum width (parallel branches) in the flow
    pub fn width(&self) -> usize {
        self.flow.width()
    }
}

/// Result of Maneuver execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverResult {
    /// Final aggregated output from the workflow
    pub final_output: String,

    /// Individual outputs from each step/agent
    pub step_outputs: HashMap<String, String>,

    /// Order in which agents were executed
    pub execution_order: Vec<String>,

    /// Optional timing metrics per agent (if enabled)
    pub timing_metrics: Option<HashMap<String, Duration>>,

    /// Execution status
    pub status: ExecutionStatus,
}

/// Status of Maneuver execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// All agents completed successfully
    Success,
    /// Some agents failed but execution continued (ContinueParallel or IgnoreErrors)
    PartialSuccess,
    /// Execution failed
    Failed,
}

impl ManeuverResult {
    /// Create a new successful result
    pub fn new(
        final_output: String,
        step_outputs: HashMap<String, String>,
        execution_order: Vec<String>,
    ) -> Self {
        ManeuverResult {
            final_output,
            step_outputs,
            execution_order,
            timing_metrics: None,
            status: ExecutionStatus::Success,
        }
    }

    /// Create a result with timing metrics
    pub fn with_timing(
        final_output: String,
        step_outputs: HashMap<String, String>,
        execution_order: Vec<String>,
        timing_metrics: HashMap<String, Duration>,
    ) -> Self {
        ManeuverResult {
            final_output,
            step_outputs,
            execution_order,
            timing_metrics: Some(timing_metrics),
            status: ExecutionStatus::Success,
        }
    }

    /// Set the execution status
    pub fn with_status(mut self, status: ExecutionStatus) -> Self {
        self.status = status;
        self
    }

    /// Get the output from a specific agent
    pub fn get_agent_output(&self, agent_name: &str) -> Option<&String> {
        self.step_outputs.get(agent_name)
    }

    /// Calculate total execution duration if timing metrics are available
    pub fn total_duration(&self) -> Option<Duration> {
        self.timing_metrics
            .as_ref()
            .map(|metrics| metrics.values().sum())
    }
}

/// Errors that can occur during Maneuver operations
#[derive(Debug, thiserror::Error)]
pub enum ManeuverError {
    #[error("Parse error: {0}")]
    ParseError(#[from] crate::core::platform::container::battalion::parser::FlowParseError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Agent '{agent_name}' not found. Available agents: {}", available_agents.join(", "))]
    AgentNotFound {
        agent_name: String,
        available_agents: Vec<String>,
    },

    #[error("Timeout after {duration:?}")]
    TimeoutError { duration: Duration },

    #[error("Paladin error: {0}")]
    PaladinError(String),
}

// Implement From for common error conversions
// Note: PaladinError is in application layer, so we use string conversion
// to maintain hexagonal architecture boundaries (core should not depend on application)
impl From<String> for ManeuverError {
    fn from(err: String) -> Self {
        ManeuverError::PaladinError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_strategy_default() {
        assert_eq!(ErrorStrategy::default(), ErrorStrategy::FailFast);
    }

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Concatenate);
    }

    #[test]
    fn test_maneuver_config_default() {
        let config = ManeuverConfig::default();
        assert_eq!(config.error_strategy, ErrorStrategy::FailFast);
        assert_eq!(config.output_format, OutputFormat::Concatenate);
        assert!(config.pass_output_as_input);
        assert!(config.timeout.is_some());
        assert!(config.collect_timing_metrics);
        assert!(!config.detailed_observability);
    }

    #[test]
    fn test_maneuver_config_builder() {
        let config = ManeuverConfig::new()
            .with_error_strategy(ErrorStrategy::ContinueParallel)
            .with_output_format(OutputFormat::JsonArray)
            .with_pass_output_as_input(false)
            .with_timeout(Duration::from_secs(60))
            .with_timing_metrics(false)
            .with_detailed_observability(true);

        assert_eq!(config.error_strategy, ErrorStrategy::ContinueParallel);
        assert_eq!(config.output_format, OutputFormat::JsonArray);
        assert!(!config.pass_output_as_input);
        assert_eq!(config.timeout, Some(Duration::from_secs(60)));
        assert!(!config.collect_timing_metrics);
        assert!(config.detailed_observability);
    }

    #[test]
    fn test_maneuver_config_validation() {
        let config = ManeuverConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = ManeuverConfig::default().with_timeout(Duration::from_secs(0));
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_execution_status() {
        let status = ExecutionStatus::Success;
        assert_eq!(status, ExecutionStatus::Success);
    }

    #[test]
    fn test_maneuver_result_new() {
        let mut step_outputs = HashMap::new();
        step_outputs.insert("agent1".to_string(), "output1".to_string());

        let result = ManeuverResult::new(
            "final output".to_string(),
            step_outputs.clone(),
            vec!["agent1".to_string()],
        );

        assert_eq!(result.final_output, "final output");
        assert_eq!(result.status, ExecutionStatus::Success);
        assert!(result.timing_metrics.is_none());
    }

    #[test]
    fn test_maneuver_result_with_timing() {
        let mut step_outputs = HashMap::new();
        step_outputs.insert("agent1".to_string(), "output1".to_string());

        let mut timing = HashMap::new();
        timing.insert("agent1".to_string(), Duration::from_millis(100));

        let result = ManeuverResult::with_timing(
            "final output".to_string(),
            step_outputs.clone(),
            vec!["agent1".to_string()],
            timing,
        );

        assert!(result.timing_metrics.is_some());
        assert_eq!(result.total_duration(), Some(Duration::from_millis(100)));
    }

    #[test]
    fn test_maneuver_result_get_agent_output() {
        let mut step_outputs = HashMap::new();
        step_outputs.insert("agent1".to_string(), "output1".to_string());

        let result = ManeuverResult::new(
            "final".to_string(),
            step_outputs,
            vec!["agent1".to_string()],
        );

        assert_eq!(
            result.get_agent_output("agent1"),
            Some(&"output1".to_string())
        );
        assert_eq!(result.get_agent_output("agent2"), None);
    }
}
