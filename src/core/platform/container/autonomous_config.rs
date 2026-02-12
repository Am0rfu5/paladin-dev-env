//! Autonomous Feature Configuration
//!
//! This module defines configuration structures for all autonomous agent features.
//! These configurations control planning mode, prompt generation, dynamic temperature,
//! and agent handoffs.

use serde::{Deserialize, Serialize};

use super::handoff::HandoffStrategy;

/// Retry configuration for handoff execution
///
/// Controls how handoffs are retried when transient errors occur.
/// Uses exponential backoff: `delay = initial_backoff_ms * (multiplier ^ attempt)`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffRetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,

    /// Initial backoff delay in milliseconds
    pub initial_backoff_ms: u64,

    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
}

impl Default for HandoffRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
            backoff_multiplier: 2.0,
        }
    }
}

impl HandoffRetryConfig {
    /// Creates a new retry configuration
    pub fn new(max_retries: u32, initial_backoff_ms: u64, backoff_multiplier: f64) -> Self {
        Self {
            max_retries,
            initial_backoff_ms,
            backoff_multiplier,
        }
    }

    /// Calculates the backoff delay for a given attempt
    ///
    /// Formula: `initial_backoff_ms * (backoff_multiplier ^ attempt)`
    pub fn calculate_backoff(&self, attempt: u32) -> u64 {
        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        (self.initial_backoff_ms as f64 * multiplier) as u64
    }
}

/// Complete configuration for all autonomous features
///
/// All features are opt-in (disabled by default) to maintain backward compatibility.
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::autonomous_config::AutonomousConfig;
///
/// let config = AutonomousConfig::default();
/// assert!(!config.planning.enabled);
/// assert!(!config.prompt_generation.enabled);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AutonomousConfig {
    /// Planning mode configuration
    pub planning: PlanningConfig,

    /// Prompt generation configuration
    pub prompt_generation: PromptConfig,

    /// Dynamic temperature configuration
    pub dynamic_temperature: TemperatureConfig,

    /// Agent handoff configuration
    pub handoffs: HandoffConfig,
}

impl AutonomousConfig {
    /// Creates a new configuration with all features disabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates the configuration
    ///
    /// Checks for:
    /// - Valid temperature bounds
    /// - Positive max_subtasks
    /// - Valid handoff depth
    pub fn validate(&self) -> Result<(), String> {
        if self.planning.enabled {
            if self.planning.max_subtasks == 0 {
                return Err("max_subtasks must be greater than 0".to_string());
            }
            if self.planning.max_subtasks > 100 {
                return Err("max_subtasks should not exceed 100 (performance concern)".to_string());
            }
        }

        if self.dynamic_temperature.enabled {
            if self.dynamic_temperature.min < 0.0 || self.dynamic_temperature.min > 1.0 {
                return Err("temperature min must be between 0.0 and 1.0".to_string());
            }
            if self.dynamic_temperature.max < 0.0 || self.dynamic_temperature.max > 1.0 {
                return Err("temperature max must be between 0.0 and 1.0".to_string());
            }
            if self.dynamic_temperature.min >= self.dynamic_temperature.max {
                return Err("temperature min must be less than max".to_string());
            }
        }

        if self.handoffs.enabled {
            if self.handoffs.max_depth == 0 {
                return Err("handoff max_depth must be greater than 0".to_string());
            }
            if self.handoffs.max_depth > 20 {
                return Err(
                    "handoff max_depth should not exceed 20 (performance concern)".to_string(),
                );
            }
        }

        Ok(())
    }
}

/// Configuration for autonomous planning mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanningConfig {
    /// Whether autonomous planning is enabled
    pub enabled: bool,

    /// Maximum number of subtasks the planner can create
    pub max_subtasks: u32,
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_subtasks: 10,
        }
    }
}

impl PlanningConfig {
    /// Creates a new planning configuration with custom max_subtasks
    pub fn new(max_subtasks: u32) -> Self {
        Self {
            enabled: true,
            max_subtasks,
        }
    }

    /// Enables planning with default settings
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }
}

/// Configuration for automatic prompt generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PromptConfig {
    /// Whether automatic prompt generation is enabled
    #[serde(default)]
    pub enabled: bool,

    /// Description of the agent's role and capabilities (required if enabled)
    #[serde(default)]
    pub description: Option<String>,
}

impl PromptConfig {
    /// Creates a new prompt configuration with description
    pub fn new(description: String) -> Self {
        Self {
            enabled: true,
            description: Some(description),
        }
    }

    /// Enables prompt generation (description must be set separately)
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            description: None,
        }
    }

    /// Sets the agent description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// Configuration for dynamic temperature adjustment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemperatureConfig {
    /// Whether dynamic temperature is enabled
    pub enabled: bool,

    /// Minimum temperature value (0.0-1.0)
    pub min: f32,

    /// Maximum temperature value (0.0-1.0)
    pub max: f32,
}

impl Default for TemperatureConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min: 0.1,
            max: 0.9,
        }
    }
}

impl TemperatureConfig {
    /// Creates a new temperature configuration with custom bounds
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum temperature (0.0-1.0)
    /// * `max` - Maximum temperature (0.0-1.0)
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            enabled: true,
            min,
            max,
        }
    }

    /// Enables dynamic temperature with default bounds
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Sets custom temperature bounds
    pub fn with_bounds(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

/// Configuration for agent handoffs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffConfig {
    /// Whether agent handoffs are enabled
    pub enabled: bool,

    /// Strategy for handoff decisions
    pub strategy: HandoffStrategy,

    /// Maximum depth of handoff chain
    pub max_depth: u32,

    /// Retry configuration for handoff execution
    pub retry: HandoffRetryConfig,
}

impl Default for HandoffConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: HandoffStrategy::default(),
            max_depth: 5,
            retry: HandoffRetryConfig::default(),
        }
    }
}

impl HandoffConfig {
    /// Creates a new handoff configuration with custom settings
    ///
    /// # Arguments
    ///
    /// * `strategy` - Handoff strategy to use
    /// * `max_depth` - Maximum handoff chain depth
    pub fn new(strategy: HandoffStrategy, max_depth: u32) -> Self {
        Self {
            enabled: true,
            strategy,
            max_depth,
            retry: HandoffRetryConfig::default(),
        }
    }

    /// Enables handoffs with default settings
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Sets the handoff strategy
    pub fn with_strategy(mut self, strategy: HandoffStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the maximum handoff depth
    pub fn with_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Sets the retry configuration
    pub fn with_retry(mut self, retry: HandoffRetryConfig) -> Self {
        self.retry = retry;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomous_config_default() {
        let config = AutonomousConfig::default();
        assert!(!config.planning.enabled);
        assert!(!config.prompt_generation.enabled);
        assert!(!config.dynamic_temperature.enabled);
        assert!(!config.handoffs.enabled);
    }

    #[test]
    fn test_autonomous_config_validate_valid() {
        let config = AutonomousConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_autonomous_config_validate_planning_zero_subtasks() {
        let mut config = AutonomousConfig::default();
        config.planning.enabled = true;
        config.planning.max_subtasks = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("greater than 0"));
    }

    #[test]
    fn test_autonomous_config_validate_planning_too_many_subtasks() {
        let mut config = AutonomousConfig::default();
        config.planning.enabled = true;
        config.planning.max_subtasks = 150;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("should not exceed 100"));
    }

    #[test]
    fn test_autonomous_config_validate_temperature_bounds() {
        let mut config = AutonomousConfig::default();
        config.dynamic_temperature.enabled = true;
        config.dynamic_temperature.min = 0.8;
        config.dynamic_temperature.max = 0.5;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("min must be less than max"));
    }

    #[test]
    fn test_autonomous_config_validate_temperature_invalid_range() {
        let mut config = AutonomousConfig::default();
        config.dynamic_temperature.enabled = true;
        config.dynamic_temperature.min = -0.1;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 0.0 and 1.0"));
    }

    #[test]
    fn test_autonomous_config_validate_handoff_zero_depth() {
        let mut config = AutonomousConfig::default();
        config.handoffs.enabled = true;
        config.handoffs.max_depth = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("greater than 0"));
    }

    #[test]
    fn test_planning_config_default() {
        let config = PlanningConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.max_subtasks, 10);
    }

    #[test]
    fn test_planning_config_new() {
        let config = PlanningConfig::new(15);
        assert!(config.enabled);
        assert_eq!(config.max_subtasks, 15);
    }

    #[test]
    fn test_planning_config_enabled() {
        let config = PlanningConfig::enabled();
        assert!(config.enabled);
        assert_eq!(config.max_subtasks, 10);
    }

    #[test]
    fn test_prompt_config_default() {
        let config = PromptConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.description, None);
    }

    #[test]
    fn test_prompt_config_new() {
        let config = PromptConfig::new("Data analyst".to_string());
        assert!(config.enabled);
        assert_eq!(config.description, Some("Data analyst".to_string()));
    }

    #[test]
    fn test_prompt_config_with_description() {
        let config = PromptConfig::enabled().with_description("Expert coder".to_string());
        assert!(config.enabled);
        assert_eq!(config.description, Some("Expert coder".to_string()));
    }

    #[test]
    fn test_temperature_config_default() {
        let config = TemperatureConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.min, 0.1);
        assert_eq!(config.max, 0.9);
    }

    #[test]
    fn test_temperature_config_new() {
        let config = TemperatureConfig::new(0.2, 0.8);
        assert!(config.enabled);
        assert_eq!(config.min, 0.2);
        assert_eq!(config.max, 0.8);
    }

    #[test]
    fn test_temperature_config_with_bounds() {
        let config = TemperatureConfig::enabled().with_bounds(0.3, 0.7);
        assert!(config.enabled);
        assert_eq!(config.min, 0.3);
        assert_eq!(config.max, 0.7);
    }

    #[test]
    fn test_handoff_config_default() {
        let config = HandoffConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.strategy, HandoffStrategy::Automatic);
        assert_eq!(config.max_depth, 5);
    }

    #[test]
    fn test_handoff_config_new() {
        let config = HandoffConfig::new(HandoffStrategy::Explicit, 10);
        assert!(config.enabled);
        assert_eq!(config.strategy, HandoffStrategy::Explicit);
        assert_eq!(config.max_depth, 10);
    }

    #[test]
    fn test_handoff_config_with_strategy() {
        let config = HandoffConfig::enabled()
            .with_strategy(HandoffStrategy::threshold(0.7))
            .with_max_depth(8);
        assert!(config.enabled);
        assert_eq!(config.max_depth, 8);
    }

    #[test]
    fn test_autonomous_config_serialization() {
        let config = AutonomousConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AutonomousConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
