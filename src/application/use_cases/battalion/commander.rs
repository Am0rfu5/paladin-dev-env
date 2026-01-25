//! Commander Strategy Router
//!
//! Provides unified interface for selecting and executing Battalion orchestration patterns.
//! Supports both manual strategy selection and Auto mode with rule-based heuristics.

use log::{debug, info};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::ports::output::paladin_port::PaladinPort;
use crate::application::use_cases::battalion::campaign_service::CampaignExecutionService;
use crate::application::use_cases::battalion::chain_of_command_service::ChainOfCommandExecutionService;
use crate::application::use_cases::battalion::formation_service::FormationExecutionService;
use crate::application::use_cases::battalion::phalanx_service::PhalanxExecutionService;
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
    pub async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError> {
        let start_time = std::time::Instant::now();
        let started_at = chrono::Utc::now();

        // Resolve strategy (Auto mode uses heuristics)
        let (effective_strategy, selection_reason) = match &self.strategy {
            BattalionStrategy::Auto => {
                let (selected, reason) = self.analyze_and_select(input);
                info!(
                    "Commander {} Auto mode selected {:?}: {}",
                    self.id, selected, reason
                );
                (selected, Some(reason))
            }
            explicit_strategy => {
                debug!(
                    "Commander {} using explicit strategy {:?}",
                    self.id, explicit_strategy
                );
                (explicit_strategy.clone(), None)
            }
        };

        let selection_time_ms = start_time.elapsed().as_millis() as u64;
        debug!(
            "Strategy selection took {}ms for Commander {}",
            selection_time_ms, self.id
        );

        // Log execution details
        info!(
            "Commander {} executing {} Paladins with {:?} strategy",
            self.id,
            self.paladins.len(),
            effective_strategy
        );

        // Delegate to appropriate service
        let result = match effective_strategy {
            BattalionStrategy::Formation => {
                debug!("Delegating to FormationExecutionService");
                let formation =
                    crate::core::platform::container::battalion::formation::Formation::new(
                        self.paladins.clone(),
                        self.config.clone(),
                    )?;
                let service = FormationExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&formation, input).await
            }
            BattalionStrategy::Phalanx => {
                debug!("Delegating to PhalanxExecutionService");
                let phalanx = crate::core::platform::container::battalion::phalanx::Phalanx::new(
                    self.paladins.clone(),
                    self.config.clone(),
                )?;
                let service = PhalanxExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&phalanx, input).await
            }
            BattalionStrategy::Campaign => {
                debug!("Delegating to CampaignExecutionService");
                // For Campaign, we need a DAG. For now, create a simple linear graph
                // This will be enhanced when we have proper Campaign builder support
                let campaign = crate::core::platform::container::battalion::campaign::Campaign::new(
                    self.config.clone(),
                );
                let service = CampaignExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&campaign, input).await
            }
            BattalionStrategy::ChainOfCommand => {
                debug!("Delegating to ChainOfCommandExecutionService");
                // For ChainOfCommand, use first Paladin as commander, rest as specialists
                if self.paladins.is_empty() {
                    return Err(BattalionError::ValidationError(
                        "ChainOfCommand requires at least 1 Paladin".to_string(),
                    ));
                }
                let commander = self.paladins[0].clone();
                let specialists = if self.paladins.len() > 1 {
                    self.paladins[1..].to_vec()
                } else {
                    // If only 1 Paladin, use it as both commander and specialist
                    vec![self.paladins[0].clone()]
                };
                let chain = crate::core::platform::container::battalion::chain_of_command::ChainOfCommand::new(
                    commander,
                    specialists,
                    self.config.clone(),
                )?;
                let service = ChainOfCommandExecutionService::new(Arc::clone(&self.paladin_port));
                let delegation_result = service.execute(&chain, input).await?;

                // Convert DelegationResult to BattalionResult
                let final_output = delegation_result.outputs.join("\n");
                Ok(BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output,
                    paladin_results: vec![], // ChainOfCommand handles this internally
                    status: crate::core::platform::container::battalion::BattalionStatus::Completed,
                })
            }
            BattalionStrategy::Auto => {
                // This should never happen as Auto is resolved above
                return Err(BattalionError::StrategySelection(
                    "Auto strategy was not resolved".to_string(),
                ));
            }
        };

        let total_time_ms = start_time.elapsed().as_millis() as u64;
        info!(
            "Commander {} completed in {}ms (selection: {}ms, execution: {}ms)",
            self.id,
            total_time_ms,
            selection_time_ms,
            total_time_ms - selection_time_ms
        );

        if let Some(reason) = selection_reason {
            debug!("Auto-selection reasoning: {}", reason);
        }

        result
    }

    /// Analyze input and Paladins to select optimal strategy
    ///
    /// Applies rule-based heuristics to determine the best orchestration
    /// pattern based on input keywords and Paladin characteristics.
    ///
    /// # Arguments
    ///
    /// * `input` - The user's input query/task
    ///
    /// # Returns
    ///
    /// A tuple of (selected strategy, reasoning for the selection)
    ///
    /// # Strategy Selection Rules
    ///
    /// 1. **Formation** - Sequential execution
    ///    - Keywords: "sequential", "pipeline", "chain", "step by step", "one after", "in order"
    ///    - 1-3 Paladins (default for small teams)
    ///
    /// 2. **Phalanx** - Parallel execution
    ///    - Keywords: "parallel", "concurrent", "all at once", "simultaneously", "together"
    ///    - 4+ Paladins with similar capabilities
    ///
    /// 3. **Campaign** - Graph/DAG orchestration
    ///    - Keywords: "workflow", "graph", "conditional", "if-then", "depends on", "after"
    ///    - Complex multi-stage tasks
    ///
    /// 4. **ChainOfCommand** - Hierarchical delegation
    ///    - Keywords: "delegate", "hierarchy", "specialist", "expert", "coordinator", "manager"
    ///    - Tasks requiring specialized expertise
    ///
    /// # Default
    ///
    /// Falls back to Formation if no clear indicators are found.
    fn analyze_and_select(&self, input: &str) -> (BattalionStrategy, String) {
        let input_lower = input.to_lowercase();

        // Check for Formation indicators
        let formation_keywords = [
            "sequential",
            "pipeline",
            "chain",
            "step by step",
            "one after",
            "in order",
            "first",
            "then",
            "next",
        ];
        if formation_keywords.iter().any(|kw| input_lower.contains(kw)) {
            return (
                BattalionStrategy::Formation,
                format!(
                    "Input contains sequential keywords, using Formation for {} Paladins",
                    self.paladins.len()
                ),
            );
        }

        // Check for Phalanx indicators
        let phalanx_keywords = [
            "parallel",
            "concurrent",
            "all at once",
            "simultaneously",
            "together",
            "at the same time",
            "in parallel",
        ];
        if phalanx_keywords.iter().any(|kw| input_lower.contains(kw)) {
            return (
                BattalionStrategy::Phalanx,
                format!(
                    "Input contains parallel keywords, using Phalanx for {} Paladins",
                    self.paladins.len()
                ),
            );
        }

        // Check for Campaign indicators
        let campaign_keywords = [
            "workflow",
            "graph",
            "conditional",
            "if-then",
            "depends on",
            "after",
            "before",
            "when",
            "complex",
            "multi-stage",
        ];
        if campaign_keywords.iter().any(|kw| input_lower.contains(kw)) {
            return (
                BattalionStrategy::Campaign,
                format!(
                    "Input contains workflow/conditional keywords, using Campaign for {} Paladins",
                    self.paladins.len()
                ),
            );
        }

        // Check for ChainOfCommand indicators
        let chain_keywords = [
            "delegate",
            "hierarchy",
            "specialist",
            "expert",
            "coordinator",
            "manager",
            "lead",
            "senior",
            "specialized",
        ];
        if chain_keywords.iter().any(|kw| input_lower.contains(kw)) {
            return (
                BattalionStrategy::ChainOfCommand,
                format!(
                    "Input contains delegation/hierarchy keywords, using ChainOfCommand for {} Paladins",
                    self.paladins.len()
                ),
            );
        }

        // Heuristics based on Paladin count
        match self.paladins.len() {
            1 => (
                BattalionStrategy::Formation,
                "Single Paladin detected, using Formation (sequential)".to_string(),
            ),
            2..=3 => (
                BattalionStrategy::Formation,
                format!(
                    "Small team ({} Paladins), using Formation (sequential)",
                    self.paladins.len()
                ),
            ),
            _ => {
                // Default fallback for larger teams
                (
                    BattalionStrategy::Formation,
                    format!(
                        "No clear strategy indicators, defaulting to Formation for {} Paladins",
                        self.paladins.len()
                    ),
                )
            }
        }
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

    #[test]
    fn test_auto_selects_formation_for_sequential_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(), create_test_paladin()];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Process this step by step");
        assert_eq!(strategy, BattalionStrategy::Formation);
        assert!(reason.contains("sequential"));

        let (strategy2, _) = commander.analyze_and_select("Run these in a pipeline");
        assert_eq!(strategy2, BattalionStrategy::Formation);

        let (strategy3, _) = commander.analyze_and_select("Chain these together");
        assert_eq!(strategy3, BattalionStrategy::Formation);
    }

    #[test]
    fn test_auto_selects_phalanx_for_parallel_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 4];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Run these in parallel");
        assert_eq!(strategy, BattalionStrategy::Phalanx);
        assert!(reason.contains("parallel"));

        let (strategy2, _) = commander.analyze_and_select("Execute all at once");
        assert_eq!(strategy2, BattalionStrategy::Phalanx);

        let (strategy3, _) = commander.analyze_and_select("Process simultaneously");
        assert_eq!(strategy3, BattalionStrategy::Phalanx);
    }

    #[test]
    fn test_auto_selects_campaign_for_workflow_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Build a workflow for this task");
        assert_eq!(strategy, BattalionStrategy::Campaign);
        assert!(reason.contains("workflow"));

        let (strategy2, _) = commander.analyze_and_select("If-then conditional logic");
        assert_eq!(strategy2, BattalionStrategy::Campaign);

        let (strategy3, _) = commander.analyze_and_select("This is a complex multi-stage process");
        assert_eq!(strategy3, BattalionStrategy::Campaign);
    }

    #[test]
    fn test_auto_selects_chain_for_delegate_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Delegate to specialist");
        assert_eq!(strategy, BattalionStrategy::ChainOfCommand);
        assert!(reason.contains("delegation"));

        let (strategy2, _) = commander.analyze_and_select("Use a hierarchy of experts");
        assert_eq!(strategy2, BattalionStrategy::ChainOfCommand);

        let (strategy3, _) = commander.analyze_and_select("Coordinator should manage this");
        assert_eq!(strategy3, BattalionStrategy::ChainOfCommand);
    }

    #[test]
    fn test_auto_selects_formation_for_single_paladin() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin()];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Do something");
        assert_eq!(strategy, BattalionStrategy::Formation);
        assert!(reason.contains("Single Paladin"));
    }

    #[test]
    fn test_auto_defaults_to_formation_when_uncertain() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 5];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Analyze this data");
        assert_eq!(strategy, BattalionStrategy::Formation);
        assert!(reason.contains("defaulting"));
    }

    #[test]
    fn test_auto_selection_is_case_insensitive() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 2];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy1, _) = commander.analyze_and_select("Run these in PARALLEL");
        assert_eq!(strategy1, BattalionStrategy::Phalanx);

        let (strategy2, _) = commander.analyze_and_select("Execute STEP BY STEP");
        assert_eq!(strategy2, BattalionStrategy::Formation);

        let (strategy3, _) = commander.analyze_and_select("Create a WORKFLOW");
        assert_eq!(strategy3, BattalionStrategy::Campaign);
    }

    #[test]
    fn test_auto_prioritizes_keywords_over_count() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin()];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        // Even with 1 Paladin, "parallel" keyword should select Phalanx
        let (strategy, _) = commander.analyze_and_select("Run this in parallel");
        assert_eq!(strategy, BattalionStrategy::Phalanx);
    }
}
