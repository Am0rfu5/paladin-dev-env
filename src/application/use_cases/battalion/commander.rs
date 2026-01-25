//! Commander Strategy Router
//!
//! Provides unified interface for selecting and executing Battalion orchestration patterns.
//! Supports both manual strategy selection and Auto mode with rule-based heuristics.

use log::{debug, info};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::ports::output::paladin_port::PaladinPort;
use crate::core::platform::container::battalion::{
    BattalionConfig, BattalionError, BattalionResult, BattalionStrategy,
};
use crate::core::platform::container::paladin::Paladin;

/// Commander for routing Battalion execution to appropriate strategies
///
/// The Commander provides a unified interface for executing groups of Paladins,
/// automatically selecting the optimal orchestration pattern based on configured
/// strategy or intelligent heuristics.
///
/// # Example
///
/// ```ignore
/// use paladin::application::use_cases::battalion::commander::{Commander, CommanderBuilder};
/// use std::sync::Arc;
///
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Auto)
///     .paladins(vec![paladin1, paladin2])
///     .config(config)
///     .build()?;
///
/// let result = commander.execute("Initial input").await?;
/// ```
pub struct Commander {
    /// Unique identifier for this Commander instance
    pub id: Uuid,

    /// Selected orchestration strategy
    pub strategy: BattalionStrategy,

    /// Paladins to orchestrate
    pub paladins: Vec<Paladin>,

    /// Battalion configuration
    pub config: BattalionConfig,

    /// Paladin execution port
    #[allow(dead_code)] // Will be used in Task 4.0 for execute implementation
    paladin_port: Arc<dyn PaladinPort>,
}

impl std::fmt::Debug for Commander {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commander")
            .field("id", &self.id)
            .field("strategy", &self.strategy)
            .field("paladins", &self.paladins)
            .field("config", &self.config)
            .field("paladin_port", &"<dyn PaladinPort>")
            .finish()
    }
}

impl Commander {
    /// Create a new Commander instance
    ///
    /// # Arguments
    ///
    /// * `strategy` - Orchestration strategy to use
    /// * `paladins` - Vector of Paladins to orchestrate
    /// * `config` - Battalion configuration
    /// * `paladin_port` - Port for executing Paladins
    ///
    /// # Returns
    ///
    /// A new Commander instance with generated UUID and creation timestamp
    pub fn new(
        strategy: BattalionStrategy,
        paladins: Vec<Paladin>,
        config: BattalionConfig,
        paladin_port: Arc<dyn PaladinPort>,
    ) -> Self {
        let id = Uuid::new_v4();
        info!(
            "Creating Commander {} with strategy {:?} and {} Paladins",
            id,
            strategy,
            paladins.len()
        );

        Self {
            id,
            strategy,
            paladins,
            config,
            paladin_port,
        }
    }

    /// Execute the Commander's Battalion with the given input
    ///
    /// Routes execution to the appropriate strategy service based on the
    /// configured strategy. For Auto mode, applies heuristics to select
    /// the optimal strategy.
    ///
    /// # Arguments
    ///
    /// * `input` - Initial input for the Battalion
    ///
    /// # Returns
    ///
    /// * `Ok(BattalionResult)` - Result of Battalion execution
    /// * `Err(BattalionError)` - If execution fails
    pub async fn execute(&self, _input: &str) -> Result<BattalionResult, BattalionError> {
        debug!(
            "Commander {} executing with strategy {:?}",
            self.id, self.strategy
        );

        // TODO: Implement strategy routing in Task 4.0
        todo!("Implement execute method in Task 4.0")
    }
}

/// Builder for creating Commander instances with validation
///
/// Provides a fluent interface for constructing Commanders with proper
/// validation of required fields.
///
/// # Example
///
/// ```ignore
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Formation)
///     .paladins(vec![paladin1, paladin2])
///     .config(config)
///     .build()?;
/// ```
pub struct CommanderBuilder {
    strategy: Option<BattalionStrategy>,
    paladins: Option<Vec<Paladin>>,
    config: Option<BattalionConfig>,
    paladin_port: Arc<dyn PaladinPort>,
}

impl CommanderBuilder {
    /// Create a new CommanderBuilder
    ///
    /// # Arguments
    ///
    /// * `paladin_port` - Port for executing Paladins (required)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = CommanderBuilder::new(paladin_port);
    /// ```
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self {
        Self {
            strategy: None,
            paladins: None,
            config: None,
            paladin_port,
        }
    }

    /// Set the orchestration strategy
    ///
    /// # Arguments
    ///
    /// * `strategy` - BattalionStrategy to use
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.strategy(BattalionStrategy::Auto)
    /// ```
    pub fn strategy(mut self, strategy: BattalionStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Set the Paladins to orchestrate
    ///
    /// # Arguments
    ///
    /// * `paladins` - Vector of Paladin instances
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.paladins(vec![paladin1, paladin2, paladin3])
    /// ```
    pub fn paladins(mut self, paladins: Vec<Paladin>) -> Self {
        self.paladins = Some(paladins);
        self
    }

    /// Set the Battalion configuration
    ///
    /// # Arguments
    ///
    /// * `config` - BattalionConfig instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.config(config)
    /// ```
    pub fn config(mut self, config: BattalionConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the Commander instance with validation
    ///
    /// Validates that all required fields are present and returns a configured
    /// Commander ready for execution.
    ///
    /// # Returns
    ///
    /// * `Ok(Commander)` - Successfully built Commander
    /// * `Err(BattalionError::CommanderValidation)` - If validation fails
    ///
    /// # Errors
    ///
    /// Returns `CommanderValidation` error if:
    /// - Strategy is not set
    /// - Paladins vector is not set or is empty
    /// - Config is not set
    ///
    /// # Example
    ///
    /// ```ignore
    /// let commander = builder.build()?;
    /// ```
    pub fn build(self) -> Result<Commander, BattalionError> {
        let strategy = self.strategy.ok_or_else(|| {
            BattalionError::CommanderValidation("Strategy is required".to_string())
        })?;

        let paladins = self.paladins.ok_or_else(|| {
            BattalionError::CommanderValidation("Paladins are required".to_string())
        })?;

        if paladins.is_empty() {
            return Err(BattalionError::CommanderValidation(
                "At least one Paladin is required".to_string(),
            ));
        }

        let config = self
            .config
            .ok_or_else(|| BattalionError::CommanderValidation("Config is required".to_string()))?;

        Ok(Commander::new(
            strategy,
            paladins,
            config,
            self.paladin_port,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::output::paladin_port::{
        PaladinResult, PaladinStream, StopReason,
    };
    use crate::application::use_cases::paladin::error::PaladinError;
    use crate::core::base::entity::node::Node;
    use crate::core::platform::container::battalion::{ErrorStrategy, RetryPolicy};
    use crate::core::platform::container::paladin::{PaladinData, PaladinStatus};
    use async_trait::async_trait;

    /// Mock PaladinPort for testing
    struct MockPaladinPort;

    #[async_trait]
    impl PaladinPort for MockPaladinPort {
        async fn execute(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<PaladinResult, PaladinError> {
            Ok(PaladinResult {
                output: "test output".to_string(),
                token_count: 100,
                execution_time_ms: 100,
                loop_count: 1,
                stop_reason: StopReason::Completed,
            })
        }

        async fn execute_stream(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<PaladinStream, PaladinError> {
            let (_tx, rx) = tokio::sync::mpsc::channel(1);
            Ok(rx)
        }

        fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
            Ok(())
        }
    }

    fn create_test_paladin() -> Paladin {
        let data = PaladinData {
            system_prompt: "Test prompt".to_string(),
            name: "TestPaladin".to_string(),
            user_name: "User".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            stop_words: vec![],
            status: PaladinStatus::Idle,
        };
        Node::new(data, Some("TestPaladin".to_string()))
    }

    fn create_test_config() -> BattalionConfig {
        BattalionConfig {
            name: "TestBattalion".to_string(),
            description: None,
            timeout_seconds: 300,
            retry_policy: RetryPolicy::default(),
            error_strategy: ErrorStrategy::FailFast,
            metadata_output_dir: None,
        }
    }

    #[test]
    fn test_commander_builder_success() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(vec![paladin])
            .config(config)
            .build();

        assert!(commander.is_ok());
        let commander = commander.unwrap();
        assert_eq!(commander.strategy, BattalionStrategy::Formation);
        assert_eq!(commander.paladins.len(), 1);
        assert_eq!(commander.config.name, "TestBattalion");
    }

    #[test]
    fn test_commander_builder_missing_strategy() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        let result = CommanderBuilder::new(paladin_port)
            .paladins(vec![paladin])
            .config(config)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert_eq!(msg, "Strategy is required");
            }
            _ => panic!("Expected CommanderValidation error"),
        }
    }

    #[test]
    fn test_commander_builder_missing_paladins() {
        let paladin_port = Arc::new(MockPaladinPort);
        let config = create_test_config();

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Phalanx)
            .config(config)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert_eq!(msg, "Paladins are required");
            }
            _ => panic!("Expected CommanderValidation error"),
        }
    }

    #[test]
    fn test_commander_builder_empty_paladins() {
        let paladin_port = Arc::new(MockPaladinPort);
        let config = create_test_config();

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Campaign)
            .paladins(vec![])
            .config(config)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert_eq!(msg, "At least one Paladin is required");
            }
            _ => panic!("Expected CommanderValidation error"),
        }
    }

    #[test]
    fn test_commander_builder_missing_config() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::ChainOfCommand)
            .paladins(vec![paladin])
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert_eq!(msg, "Config is required");
            }
            _ => panic!("Expected CommanderValidation error"),
        }
    }

    #[test]
    fn test_commander_all_strategies() {
        let strategies = vec![
            BattalionStrategy::Formation,
            BattalionStrategy::Phalanx,
            BattalionStrategy::Campaign,
            BattalionStrategy::ChainOfCommand,
            BattalionStrategy::Auto,
        ];

        for strategy in strategies {
            let paladin_port = Arc::new(MockPaladinPort);
            let paladin = create_test_paladin();
            let config = create_test_config();

            let commander = CommanderBuilder::new(paladin_port)
                .strategy(strategy.clone())
                .paladins(vec![paladin.clone()])
                .config(config.clone())
                .build();

            assert!(commander.is_ok());
            assert_eq!(commander.unwrap().strategy, strategy);
        }
    }
}
