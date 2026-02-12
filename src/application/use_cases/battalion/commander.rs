//! Commander Strategy Router
//!
//! Provides unified interface for selecting and executing Battalion orchestration patterns.
//! Supports both manual strategy selection and Auto mode with rule-based heuristics.

use log::{debug, info};
use std::sync::Arc;
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use crate::application::ports::output::paladin_port::PaladinPort;
use crate::application::ports::output::paladin_registry::PaladinRegistry;
use crate::application::use_cases::battalion::campaign_service::CampaignExecutionService;
use crate::application::use_cases::battalion::chain_of_command_service::ChainOfCommandExecutionService;
use crate::application::use_cases::battalion::conclave_execution_service::ConclaveExecutionService;
use crate::application::use_cases::battalion::council_service::CouncilExecutionService;
use crate::application::use_cases::battalion::formation_service::FormationExecutionService;
use crate::application::use_cases::battalion::grove_service::GroveExecutionService;
use crate::application::use_cases::battalion::maneuver_service::ManeuverExecutionService;
use crate::application::use_cases::battalion::phalanx_service::PhalanxExecutionService;
use crate::core::platform::container::battalion::{
    BattalionConfig, BattalionError, BattalionResult, BattalionStrategy, ErrorStrategy,
};
use crate::core::platform::container::paladin::Paladin;
use crate::infrastructure::adapters::paladin_registry::HashMapPaladinRegistry;

/// Commander for routing Battalion execution to appropriate strategies.
///
/// The Commander is the primary interface for orchestrating multiple Paladins in coordinated
/// workflows. It provides intelligent strategy selection and unified execution across all
/// Battalion patterns: Formation, Phalanx, Campaign, and ChainOfCommand.
///
/// # Features
///
/// - **Auto Mode**: Automatically selects optimal strategy based on input analysis and Paladin count
/// - **Explicit Strategy**: Manually select Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, or Grove
/// - **Timeout Enforcement**: Configurable execution timeouts with automatic cancellation
/// - **Error Handling**: Supports FailFast, ContinueOnError, and RetryThenContinue strategies
/// - **Telemetry**: Comprehensive execution metadata including timing and success/failure counts
/// - **Retry Logic**: Configurable retry policies with exponential backoff
///
/// # Auto Mode Heuristics
///
/// When using `BattalionStrategy::Auto`, the Commander applies the following rules:
///
/// 1. **Conclave** (Expert Synthesis)
///    - 3+ Paladins with diverse expertise
///    - Keywords: "synthesize", "compare", "expert panel", "perspectives", "consensus"
///
/// 2. **Council** (Collaborative Discussion)
///    - 2+ Paladins for turn-based dialogue
///    - Keywords: "discuss", "debate", "collaborate", "consensus", "brainstorm"
///
/// 3. **Grove** (Intelligent Routing)
///    - 2+ Paladins with specialized capabilities
///    - Keywords: "route", "best agent", "expertise", "most qualified"
///
/// 4. **Formation** (Sequential)
///    - 1-3 Paladins by default
///    - Keywords: "sequential", "pipeline", "step by step", "one after", "first then"
///
/// 5. **Phalanx** (Parallel)
///    - 4+ Paladins with independent tasks
///    - Keywords: "parallel", "concurrent", "all at once", "simultaneously"
///
/// 6. **Campaign** (Graph/Workflow)
///    - Complex multi-stage workflows
///    - Keywords: "workflow", "graph", "conditional", "if-then", "depends on"
///
/// 7. **ChainOfCommand** (Hierarchical)
///    - Specialist delegation patterns
///    - Keywords: "delegate", "hierarchy", "specialist", "expert", "assign to"
///
/// # Examples
///
/// ## Basic Usage with Explicit Strategy
///
/// ```ignore
/// use paladin::application::use_cases::battalion::commander::CommanderBuilder;
/// use paladin::core::platform::container::battalion::{BattalionConfig, BattalionStrategy};
/// use std::sync::Arc;
///
/// // Create Commander with Formation strategy
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Formation)
///     .paladins(vec![analyzer, enhancer, reviewer])
///     .config(BattalionConfig::new("sequential_pipeline").with_timeout(60))
///     .build()?;
///
/// // Execute with input
/// let result = commander.execute("Process this data").await?;
/// println!("Final output: {}", result.final_output);
/// ```
///
/// ## Auto Mode with Telemetry
///
/// ```ignore
/// use paladin::core::platform::container::battalion::BattalionStrategy;
///
/// // Auto mode will select the best strategy
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Auto)
///     .paladins(vec![worker1, worker2, worker3, worker4, worker5])
///     .build()?; // Uses default config
///
/// let result = commander.execute("Process batch in parallel").await?;
///
/// // Check what strategy was selected
/// println!("Selected: {:?}", result.strategy_used);
/// if let Some(reasoning) = &result.strategy_selection_reasoning {
///     println!("Because: {}", reasoning);
/// }
/// println!("Selection took: {}ms", result.strategy_selection_time_ms);
/// ```
///
/// ## Production Configuration
///
/// ```ignore
/// use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy, RetryPolicy};
///
/// let config = BattalionConfig::new("production_battalion")
///     .with_timeout(300)
///     .with_error_strategy(ErrorStrategy::RetryThenContinue)
///     .with_retry_policy(RetryPolicy {
///         max_attempts: 3,
///         ..Default::default()
///     });
///
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Formation)
///     .paladins(paladins)
///     .config(config)
///     .build()?;
///
/// match commander.execute("Critical task").await {
///     Ok(result) => {
///         println!("Success: {} succeeded, {} failed",
///             result.paladin_success_count,
///             result.paladin_failure_count);
///     }
///     Err(e) => eprintln!("Battalion failed: {}", e),
/// }
/// ```
///
/// # See Also
///
/// - [`CommanderBuilder`] - Fluent builder for creating Commander instances
/// - [`BattalionStrategy`] - Available orchestration patterns
/// - [`BattalionConfig`] - Configuration options for execution
/// - [`BattalionResult`] - Result type with execution metadata
pub struct Commander {
    /// Unique identifier for this Commander instance
    pub id: Uuid,

    /// Selected orchestration strategy
    pub strategy: BattalionStrategy,

    /// Paladins to orchestrate
    pub paladins: Vec<Paladin>,

    /// Battalion configuration
    pub config: BattalionConfig,

    /// Optional aggregator Paladin for Conclave strategy
    pub aggregator: Option<Paladin>,

    /// Optional flow expression for Maneuver strategy
    pub flow_expression: Option<String>,

    /// Optional Maneuver configuration
    pub maneuver_config:
        Option<crate::core::platform::container::battalion::maneuver::ManeuverConfig>,

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
            .field("aggregator", &self.aggregator)
            .field("flow_expression", &self.flow_expression)
            .field("maneuver_config", &self.maneuver_config)
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
    /// * `aggregator` - Optional aggregator Paladin for Conclave strategy
    /// * `paladin_port` - Port for executing Paladins
    ///
    /// # Returns
    ///
    /// A new Commander instance with generated UUID and creation timestamp
    pub fn new(
        strategy: BattalionStrategy,
        paladins: Vec<Paladin>,
        config: BattalionConfig,
        aggregator: Option<Paladin>,
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
            aggregator,
            flow_expression: None,
            maneuver_config: None,
            paladin_port,
        }
    }

    /// Execute the Commander's Battalion with the given input.
    ///
    /// This is the primary entry point for Battalion execution. It handles strategy
    /// resolution, timeout enforcement, service delegation, and result enrichment.
    ///
    /// # Execution Flow
    ///
    /// 1. **Strategy Resolution**: If using Auto mode, analyzes input and Paladin characteristics
    ///    to select the optimal strategy. Otherwise, uses the explicitly configured strategy.
    ///
    /// 2. **Timeout Enforcement**: Wraps execution with `tokio::time::timeout` using the
    ///    configured `timeout_seconds`. If execution exceeds this limit, returns a Timeout error.
    ///
    /// 3. **Service Delegation**: Routes to the appropriate Battalion service:
    ///    - Formation -> Sequential pipeline execution
    ///    - Phalanx -> Concurrent parallel execution  
    ///    - Campaign -> Graph-based workflow execution
    ///    - ChainOfCommand -> Hierarchical delegation execution
    ///
    /// 4. **Result Enrichment**: Enhances the result with Commander-specific metadata:
    ///    - `strategy_used`: The actual strategy executed (resolved from Auto if applicable)
    ///    - `strategy_selection_reasoning`: Explanation of why the strategy was chosen
    ///    - `strategy_selection_time_ms`: Time spent selecting strategy
    ///    - Timing and success/failure counts
    ///
    /// # Arguments
    ///
    /// * `input` - The initial input string to provide to the Battalion. For:
    ///   - **Formation**: Provided to the first Paladin; subsequent Paladins receive prior output
    ///   - **Phalanx**: Provided identically to all Paladins in parallel
    ///   - **Campaign**: Provided to entry point Paladin(s) in the graph
    ///   - **ChainOfCommand**: Provided to the commander Paladin for delegation decisions
    ///
    /// # Returns
    ///
    /// * `Ok(BattalionResult)` - Successful execution with:
    ///   - `final_output`: The final result from the Battalion
    ///   - `status`: BattalionStatus::Completed
    ///   - `strategy_used`: The strategy that was actually executed
    ///   - `strategy_selection_reasoning`: Auto mode explanation (if applicable)
    ///   - `paladin_success_count` / `paladin_failure_count`: Execution statistics
    ///   - `started_at` / `completed_at`: Timestamp metadata
    ///   - Additional telemetry fields
    ///
    /// # Errors
    ///
    /// * `BattalionError::Timeout` - Execution exceeded configured timeout_seconds
    /// * `BattalionError::ExecutionError` - One or more Paladins failed (if using FailFast)
    /// * `BattalionError::ValidationError` - Invalid Battalion configuration or state
    /// * `BattalionError::PaladinError` - Underlying Paladin execution failure
    /// * Other strategy-specific errors from delegated services
    ///
    /// # Examples
    ///
    /// ## Basic Execution
    ///
    /// ```ignore
    /// let result = commander.execute("Analyze this customer feedback").await?;
    /// println!("Analysis result: {}", result.final_output);
    /// ```
    ///
    /// ## With Error Handling
    ///
    /// ```ignore
    /// match commander.execute("Process data").await {
    ///     Ok(result) => {
    ///         println!("✅ Success: {}", result.final_output);
    ///         println!("   Strategy: {:?}", result.strategy_used);
    ///         println!("   Duration: {}ms",
    ///             result.completed_at.signed_duration_since(result.started_at).num_milliseconds());
    ///     }
    ///     Err(BattalionError::Timeout(secs)) => {
    ///         eprintln!("❌ Execution timed out after {} seconds", secs);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("❌ Execution failed: {}", e);
    ///     }
    /// }
    /// ```
    ///
    /// ## Analyzing Auto Mode Selection
    ///
    /// ```ignore
    /// let result = commander.execute("Run these tasks in parallel").await?;
    ///
    /// // Auto mode provides reasoning
    /// if let Some(reasoning) = result.strategy_selection_reasoning {
    ///     println!("Auto mode selected {:?}", result.strategy_used);
    ///     println!("Reasoning: {}", reasoning);
    ///     println!("Selection time: {}ms", result.strategy_selection_time_ms);
    /// }
    /// ```
    ///
    /// # Performance Considerations
    ///
    /// - Auto mode adds 0-5ms overhead for strategy analysis
    /// - Timeout is enforced at the Commander level and also passed to services
    /// - Telemetry collection adds minimal overhead (<1ms typically)
    ///
    /// # See Also
    ///
    /// - [`BattalionStrategy`] - Available orchestration patterns
    /// - [`BattalionResult`] - Detailed result structure
    /// - [`BattalionConfig`] - Configuration options affecting execution
    pub async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError> {
        let timeout_duration = Duration::from_secs(self.config.timeout_seconds);

        match timeout(timeout_duration, self.execute_internal(input)).await {
            Ok(result) => result,
            Err(_) => {
                info!(
                    "Commander {} timed out after {} seconds",
                    self.id, self.config.timeout_seconds
                );
                Err(BattalionError::Timeout(self.config.timeout_seconds))
            }
        }
    }

    /// Internal execution logic without timeout wrapper
    async fn execute_internal(&self, input: &str) -> Result<BattalionResult, BattalionError> {
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
        let mut result = match effective_strategy {
            BattalionStrategy::Formation => {
                debug!("Delegating to FormationExecutionService");
                let formation =
                    crate::core::platform::container::battalion::formation::Formation::new(
                        self.paladins.clone(),
                        self.config.clone(),
                    )?;
                let service = FormationExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&formation, input).await?
            }
            BattalionStrategy::Phalanx => {
                debug!("Delegating to PhalanxExecutionService");
                let phalanx = crate::core::platform::container::battalion::phalanx::Phalanx::new(
                    self.paladins.clone(),
                    self.config.clone(),
                )?;
                let service = PhalanxExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&phalanx, input).await?
            }
            BattalionStrategy::Campaign => {
                debug!("Delegating to CampaignExecutionService");
                // For Campaign, create a simple linear graph from paladins
                let mut campaign =
                    crate::core::platform::container::battalion::campaign::Campaign::new(
                        self.config.clone(),
                    );

                // Add all Paladins to the campaign
                let mut paladin_ids: Vec<uuid::Uuid> = Vec::new();
                for paladin in &self.paladins {
                    let paladin_clone: crate::core::platform::container::paladin::Paladin =
                        paladin.clone();
                    let id = campaign.add_paladin(paladin_clone);
                    paladin_ids.push(id);
                }

                // Create linear edges: paladin_0 -> paladin_1 -> paladin_2 -> ...
                for i in 0..paladin_ids.len().saturating_sub(1) {
                    let edge = crate::core::platform::container::battalion::campaign::CampaignEdge::new(
                        paladin_ids[i],
                        paladin_ids[i + 1],
                        crate::core::platform::container::battalion::campaign::EdgeCondition::Always,
                    );
                    campaign.add_edge(edge)?;
                }

                // Set first Paladin as entry point
                if !paladin_ids.is_empty() {
                    campaign.set_entry_point(paladin_ids[0])?;
                }

                let service = CampaignExecutionService::new(Arc::clone(&self.paladin_port));
                service.execute(&campaign, input).await?
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
                BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output,
                    paladin_results: vec![], // ChainOfCommand handles this internally
                    status: crate::core::platform::container::battalion::BattalionStatus::Completed,
                    strategy_used: BattalionStrategy::ChainOfCommand,
                    strategy_selection_reasoning: None,
                    strategy_selection_time_ms: 0,
                    per_paladin_times: std::collections::HashMap::new(),
                    per_paladin_tokens: std::collections::HashMap::new(),
                    total_tokens: 0,
                    paladin_success_count: 0,
                    paladin_failure_count: 0,
                }
            }
            BattalionStrategy::Conclave => {
                debug!("Delegating to ConclaveExecutionService");

                // Determine experts and aggregator
                let aggregator = self.aggregator.as_ref().ok_or_else(|| {
                    BattalionError::ValidationError(
                        "Conclave strategy requires an aggregator Paladin".to_string(),
                    )
                })?;

                // All paladins become experts
                let experts = self.paladins.clone();

                if experts.len() < 2 {
                    return Err(BattalionError::ValidationError(
                        "Conclave requires at least 2 experts".to_string(),
                    ));
                }

                // Create ConclaveConfig from BattalionConfig
                let conclave_config =
                    crate::core::platform::container::battalion::conclave::ConclaveConfig::new(
                        &self.config.name,
                        self.config.clone(),
                    )
                    .with_timeout(self.config.timeout_seconds)
                    .with_retry_attempts(self.config.retry_policy.max_attempts.saturating_sub(1));

                // Create Conclave instance
                let conclave =
                    crate::core::platform::container::battalion::conclave::Conclave::new(
                        experts,
                        aggregator.clone(),
                        conclave_config,
                    )?;

                // Execute Conclave
                let service = ConclaveExecutionService::new(Arc::clone(&self.paladin_port));
                let conclave_result = service.execute(&conclave, input).await?;

                // Convert ConclaveResult to BattalionResult
                let total_experts = conclave.expert_count();
                let successful_experts = conclave_result.successful_expert_count();
                let failed_experts = total_experts.saturating_sub(successful_experts);

                BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output: conclave_result.aggregated_output.output.clone(),
                    paladin_results: vec![], // Conclave handles this internally
                    status: crate::core::platform::container::battalion::BattalionStatus::Completed,
                    strategy_used: BattalionStrategy::Conclave,
                    strategy_selection_reasoning: None,
                    strategy_selection_time_ms: 0,
                    per_paladin_times: std::collections::HashMap::new(),
                    per_paladin_tokens: std::collections::HashMap::new(),
                    total_tokens: 0,
                    paladin_success_count: successful_experts,
                    paladin_failure_count: failed_experts,
                }
            }
            BattalionStrategy::Council => {
                debug!("Delegating to CouncilExecutionService");

                // Validation: Council requires at least 2 Paladins for meaningful discussion
                if self.paladins.len() < 2 {
                    return Err(BattalionError::ValidationError(
                        "Council requires at least 2 Paladins for discussion".to_string(),
                    ));
                }

                // Build Council using builder pattern
                let mut council_builder =
                    crate::core::platform::container::battalion::council::CouncilBuilder::new()
                        .name(self.config.name.clone())
                        .max_rounds(3); // Limit to 3 rounds for reasonable execution time

                // Add all Paladins as participants using their actual names as IDs
                for paladin in &self.paladins {
                    council_builder = council_builder.add_participant(paladin.node.name.clone());
                }

                let council = council_builder.build()?;

                // Create temporary registry from paladins for Council execution
                use crate::application::ports::output::paladin_registry::PaladinRegistry;
                use crate::infrastructure::adapters::paladin_registry::HashMapPaladinRegistry;
                let registry = HashMapPaladinRegistry::new();
                for paladin in &self.paladins {
                    // Use paladin name as ID
                    registry.register(paladin.node.name.clone(), Arc::new(paladin.clone()))?;
                }

                // Execute Council (pass None for garrison_port - Commander doesn't have one)
                let service = CouncilExecutionService::new(
                    Arc::clone(&self.paladin_port),
                    None,
                    Arc::new(registry),
                );
                let council_result = service.convene(&council, input).await?;

                // Convert council result to BattalionResult
                // Final output is the complete conversation history
                let final_output = council_result
                    .transcript
                    .iter()
                    .map(|msg| format!("{}: {}", msg.speaker, msg.content))
                    .collect::<Vec<_>>()
                    .join("\n\n");

                let total_participants = self.paladins.len();

                BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output,
                    paladin_results: vec![], // Council handles this internally
                    status: crate::core::platform::container::battalion::BattalionStatus::Completed,
                    strategy_used: BattalionStrategy::Council,
                    strategy_selection_reasoning: None,
                    strategy_selection_time_ms: 0,
                    per_paladin_times: std::collections::HashMap::new(),
                    per_paladin_tokens: std::collections::HashMap::new(),
                    total_tokens: 0,
                    paladin_success_count: total_participants,
                    paladin_failure_count: 0,
                }
            }
            BattalionStrategy::Grove => {
                debug!("Delegating to GroveExecutionService");

                // Validation: Grove requires at least 2 agents for routing
                if self.paladins.len() < 2 {
                    return Err(BattalionError::ValidationError(
                        "Grove requires at least 2 Paladins for routing".to_string(),
                    ));
                }

                // Create a temporary registry from self.paladins
                let registry = HashMapPaladinRegistry::new();

                // Build Grove instance using builder pattern
                // Create a Tree with all Paladins as agents
                let mut tree =
                    crate::core::platform::container::battalion::grove::Tree::new("main");

                // Convert Paladins to TreeAgents using Paladin names as IDs
                for paladin in &self.paladins {
                    // Register paladin in registry
                    registry
                        .register(paladin.node.name.clone(), Arc::new(paladin.clone()))
                        .map_err(|e| {
                            BattalionError::ExecutionError(format!(
                                "Failed to register paladin '{}': {}",
                                paladin.node.name, e
                            ))
                        })?;

                    // Create TreeAgent with paladin ID matching registry
                    let tree_agent =
                        crate::core::platform::container::battalion::grove::TreeAgent::new(
                            paladin.node.name.clone(),
                        );
                    tree = tree.add_agent(tree_agent);
                }

                let grove = crate::core::platform::container::battalion::grove::GroveBuilder::new()
                    .name(self.config.name.clone())
                    .routing_strategy(
                        crate::core::platform::container::battalion::grove::RoutingStrategy::KeywordMatch,
                    )
                    .add_tree(tree)
                    .build()?;

                // Execute Grove with registry (no longer passes paladins directly)
                let service = GroveExecutionService::new(
                    Arc::clone(&self.paladin_port),
                    None, // embedding_port
                    None, // llm_port
                    Arc::new(registry),
                );
                let grove_result = service.execute(&grove, input).await?;

                // Convert grove result to BattalionResult
                BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output: grove_result.execution_result.clone(),
                    paladin_results: vec![], // Grove handles routing internally
                    status: crate::core::platform::container::battalion::BattalionStatus::Completed,
                    strategy_used: BattalionStrategy::Grove,
                    strategy_selection_reasoning: None,
                    strategy_selection_time_ms: 0,
                    per_paladin_times: std::collections::HashMap::new(),
                    per_paladin_tokens: std::collections::HashMap::new(),
                    total_tokens: 0,
                    paladin_success_count: 1,
                    paladin_failure_count: 0,
                }
            }
            BattalionStrategy::Maneuver => {
                debug!("Delegating to ManeuverExecutionService");

                // Validation: Maneuver requires at least 1 Paladin
                if self.paladins.is_empty() {
                    return Err(BattalionError::ValidationError(
                        "Maneuver requires at least 1 Paladin".to_string(),
                    ));
                }

                // Use flow expression from Commander (set via builder)
                // If not set, default to sequential flow for backwards compatibility
                let flow_expr = self.flow_expression.as_deref().unwrap_or_else(|| {
                    // Generate default sequential flow
                    if self.paladins.len() == 1 {
                        self.paladins[0].name.as_deref().unwrap_or("agent0")
                    } else {
                        // This fallback is not ideal but maintains backwards compatibility
                        // In practice, flow_expression should always be set via builder
                        debug!(
                            "Warning: No flow expression set, generating default sequential flow"
                        );
                        "" // Will be handled below
                    }
                });

                // If empty flow_expr from fallback, generate sequential
                let flow_expr = if flow_expr.is_empty() {
                    self.paladins
                        .iter()
                        .enumerate()
                        .map(|(i, p)| p.name.as_ref().unwrap_or(&format!("agent{}", i)).clone())
                        .collect::<Vec<_>>()
                        .join(" -> ")
                } else {
                    flow_expr.to_string()
                };

                // Parse the flow expression
                let flow = crate::core::platform::container::battalion::parser::FlowParser::parse(
                    &flow_expr,
                )
                .map_err(|e| BattalionError::ValidationError(format!("Flow parse error: {}", e)))?;

                // Build agent name -> Paladin mapping
                let mut agents = std::collections::HashMap::new();
                for (i, paladin) in self.paladins.iter().enumerate() {
                    let agent_name = paladin
                        .name
                        .as_ref()
                        .unwrap_or(&format!("agent{}", i))
                        .clone();
                    agents.insert(agent_name, paladin.clone());
                }

                // Use ManeuverConfig from Commander if set, otherwise create from BattalionConfig
                let maneuver_config = self.maneuver_config.clone().unwrap_or_else(|| {
                    crate::core::platform::container::battalion::maneuver::ManeuverConfig {
                        error_strategy: match self.config.error_strategy {
                            ErrorStrategy::FailFast => crate::core::platform::container::battalion::maneuver::ErrorStrategy::FailFast,
                            ErrorStrategy::ContinueOnError => crate::core::platform::container::battalion::maneuver::ErrorStrategy::ContinueParallel,
                            ErrorStrategy::RetryThenContinue => crate::core::platform::container::battalion::maneuver::ErrorStrategy::ContinueParallel,
                        },
                        output_format: crate::core::platform::container::battalion::maneuver::OutputFormat::Concatenate,
                        pass_output_as_input: true,
                        timeout: Some(Duration::from_secs(self.config.timeout_seconds)),
                        collect_timing_metrics: true,
                        detailed_observability: false,
                    }
                });

                // Create Maneuver instance
                let maneuver =
                    crate::core::platform::container::battalion::maneuver::Maneuver::new(
                        &self.config.name,
                        agents,
                        flow,
                        maneuver_config,
                    )
                    .map_err(|e| {
                        BattalionError::ValidationError(format!("Maneuver creation failed: {}", e))
                    })?;

                // Execute Maneuver
                let service = ManeuverExecutionService::new(Arc::clone(&self.paladin_port));
                let maneuver_result = service.execute(&maneuver, input).await.map_err(|e| {
                    BattalionError::ExecutionError(format!("Maneuver execution failed: {}", e))
                })?;

                // Convert ManeuverResult to BattalionResult
                let successful_agents = maneuver_result.execution_order.len();

                // Convert timing metrics to HashMap<String, u64> keyed by Paladin name
                let per_paladin_times: std::collections::HashMap<String, u64> = maneuver_result
                    .timing_metrics
                    .as_ref()
                    .map(|metrics| {
                        metrics
                            .iter()
                            .map(|(name, d)| (name.clone(), d.as_millis() as u64))
                            .collect()
                    })
                    .unwrap_or_default();

                BattalionResult {
                    battalion_id: Uuid::new_v4(),
                    battalion_name: self.config.name.clone(),
                    started_at,
                    completed_at: chrono::Utc::now(),
                    final_output: maneuver_result.final_output.clone(),
                    paladin_results: vec![], // Maneuver handles this internally
                    status: match maneuver_result.status {
                        crate::core::platform::container::battalion::maneuver::ExecutionStatus::Success =>
                            crate::core::platform::container::battalion::BattalionStatus::Completed,
                        crate::core::platform::container::battalion::maneuver::ExecutionStatus::PartialSuccess =>
                            crate::core::platform::container::battalion::BattalionStatus::Completed,
                        crate::core::platform::container::battalion::maneuver::ExecutionStatus::Failed =>
                            crate::core::platform::container::battalion::BattalionStatus::Failed,
                    },
                    strategy_used: BattalionStrategy::Maneuver,
                    strategy_selection_reasoning: None,
                    strategy_selection_time_ms: 0,
                    per_paladin_times,
                    per_paladin_tokens: std::collections::HashMap::new(),
                    total_tokens: 0,
                    paladin_success_count: successful_agents,
                    paladin_failure_count: 0,
                }
            }
            BattalionStrategy::Auto => {
                // This should never happen as Auto is resolved above
                return Err(BattalionError::StrategySelection(
                    "Auto strategy was not resolved".to_string(),
                ));
            }
        };

        // Enrich result with Commander-specific metadata
        result.strategy_used = effective_strategy.clone();
        result.strategy_selection_reasoning = selection_reason.clone();
        result.strategy_selection_time_ms = selection_time_ms;

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

        Ok(result)
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
    /// 1. **Conclave** - Mixture of Agents synthesis
    ///    - Keywords: "synthesize", "compare", "expert panel", "perspectives", "consensus", "combine"
    ///    - 3+ Paladins with diverse expertise
    ///
    /// # Strategy Selection Rules
    ///
    /// 1. **Conclave** - Mixture of Agents synthesis
    ///    - Keywords: "synthesize", "compare", "expert panel", "perspectives", "consensus", "combine"
    ///    - 3+ Paladins with diverse expertise
    ///
    /// 2. **Council** - Conversational multi-agent collaboration
    ///    - Keywords: "discuss", "debate", "collaborate", "consensus", "brainstorm", "dialogue"
    ///    - 2+ Paladins for turn-based discussion
    ///
    /// 3. **Grove** - Intelligent routing to specialists
    ///    - Keywords: "route", "best agent", "expertise", "most qualified", "match to"
    ///    - 2+ Paladins with specialized capabilities
    ///
    /// 4. **Formation** - Sequential execution
    ///    - Keywords: "sequential", "pipeline", "chain", "step by step", "one after", "in order"
    ///    - 1-3 Paladins (default for small teams)
    ///
    /// 5. **Phalanx** - Parallel execution
    ///    - Keywords: "parallel", "concurrent", "all at once", "simultaneously", "together"
    ///    - 4+ Paladins with similar capabilities
    ///
    /// 6. **Campaign** - Graph/DAG orchestration
    ///    - Keywords: "workflow", "graph", "conditional", "if-then", "depends on", "after"
    ///    - Complex multi-stage tasks
    ///
    /// 7. **ChainOfCommand** - Hierarchical delegation
    ///    - Keywords: "delegate", "hierarchy", "specialist", "expert", "coordinator", "manager"
    ///    - Tasks requiring specialized expertise
    ///
    /// # Default
    ///
    /// Falls back to Formation if no clear indicators are found.
    fn analyze_and_select(&self, input: &str) -> (BattalionStrategy, String) {
        let input_lower = input.to_lowercase();

        // Check for Conclave indicators (synthesis/multi-perspective analysis)
        // Check this FIRST as it's most specific and should take precedence
        let conclave_keywords = [
            "synthesize",
            "synthesis",
            "compare",
            "expert panel",
            "perspectives",
            "consensus",
            "combine",
            "aggregate",
            "merge",
            "integrate views",
            "diverse opinions",
            "multiple experts",
            "comprehensive analysis",
        ];
        if conclave_keywords.iter().any(|kw| input_lower.contains(kw)) && self.paladins.len() >= 3 {
            return (
                BattalionStrategy::Conclave,
                format!(
                    "Input contains synthesis/multi-perspective keywords with {} Paladins, using Conclave for expert synthesis",
                    self.paladins.len()
                ),
            );
        }

        // Check for Council indicators (conversational collaboration)
        // Check this SECOND as it's also very specific
        let council_keywords = [
            "discuss",
            "discussion",
            "debate",
            "deliberate",
            "collaborate",
            "conversation",
            "dialogue",
            "consensus",
            "brainstorm",
            "round table",
            "panel discussion",
            "town hall",
            "collaborate on",
            "talk through",
        ];
        if council_keywords.iter().any(|kw| input_lower.contains(kw)) && self.paladins.len() >= 2 {
            return (
                BattalionStrategy::Council,
                format!(
                    "Input contains discussion/collaboration keywords with {} Paladins, using Council for turn-based dialogue",
                    self.paladins.len()
                ),
            );
        }

        // Check for Grove indicators (intelligent routing to specialists)
        // Check this THIRD before other routing patterns
        let grove_keywords = [
            "route",
            "routing",
            "best agent",
            "expertise",
            "expert for",
            "most qualified",
            "match to",
            "assign based on",
            "specialized in",
            "skilled in",
            "capability match",
            "dynamic routing",
            "intelligent assignment",
        ];
        if grove_keywords.iter().any(|kw| input_lower.contains(kw)) && self.paladins.len() >= 2 {
            return (
                BattalionStrategy::Grove,
                format!(
                    "Input contains routing/expertise keywords with {} Paladins, using Grove for intelligent agent selection",
                    self.paladins.len()
                ),
            );
        }

        // NOTE: Maneuver strategy is EXPLICIT-ONLY and NOT selected by Auto mode.
        // Flow DSL patterns like "->" and "|" are now checked AFTER Campaign to avoid
        // conflicting with natural language usage of arrows and pipes.
        // To use Maneuver, explicitly set BattalionStrategy::Maneuver via CommanderBuilder
        // and provide a flow expression using .flow() method.

        // Check for Campaign indicators (workflow/graph orchestration)
        // Only check if no flow syntax was found (since Campaign is conceptual, not syntax-based)
        let campaign_keywords = [
            "workflow",
            "graph",
            "conditional",
            "if-then", // Multi-word phrase checked as a whole
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

        // NOTE: Maneuver keyword detection removed - Maneuver is explicit-only.
        // Keywords like "flow", "branch", "nested" will now be handled by other strategies
        // (Campaign for workflow/branching, Formation for nested sequences).

        // Check for Formation indicators (sequential execution)
        let formation_keywords = [
            "sequential",
            "pipeline",
            "chain",
            "step by step",
            "one after",
            "in order",
            "first",
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

        // Check for Phalanx indicators (parallel execution)
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

        // Check for ChainOfCommand indicators (hierarchical delegation)
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

        // Heuristics based on Paladin count (only if no keywords matched)
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
///
/// # Builder Pattern
///
/// The CommanderBuilder follows the fluent builder pattern, allowing method chaining
/// for readable and flexible Commander construction.
///
/// ## Required Fields
///
/// - **paladin_port**: Must be provided to `new()` - adapter for executing Paladins
/// - **strategy**: Must be set via `strategy()` - the orchestration pattern to use
/// - **paladins**: Must be set via `paladins()` - the Paladins to orchestrate (cannot be empty)
///
/// ## Optional Fields
///
/// - **config**: Can be set via `config()` - if omitted, uses sensible defaults:
///   - Name: "default_commander_battalion"
///   - Timeout: 300 seconds (5 minutes)
///   - Error Strategy: FailFast
///   - Retry Policy: 3 attempts with exponential backoff
///
/// # Validation
///
/// The `build()` method performs comprehensive validation:
///
/// - Ensures strategy is set
/// - Ensures at least one Paladin is provided
/// - Validates config timeout_seconds > 0
/// - Validates retry_policy max_attempts > 0
///
/// # Examples
///
/// ## Minimal Configuration (with defaults)
///
/// ```ignore
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Formation)
///     .paladins(vec![paladin1, paladin2])
///     .build()?; // Uses default config
/// ```
///
/// ## Full Configuration
///
/// ```ignore
/// use paladin::core::platform::container::battalion::{
///     BattalionConfig, BattalionStrategy, ErrorStrategy, RetryPolicy
/// };
/// use std::path::PathBuf;
///
/// let config = BattalionConfig::new("custom_battalion")
///     .with_description("Customer data processing pipeline")
///     .with_timeout(600) // 10 minutes
///     .with_error_strategy(ErrorStrategy::RetryThenContinue)
///     .with_retry_policy(RetryPolicy {
///         max_attempts: 5,
///         ..Default::default()
///     })
///     .with_metadata_dir(PathBuf::from("./checkpoints"));
///
/// let commander = CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Auto)
///     .paladins(paladins)
///     .config(config)
///     .build()?;
/// ```
///
/// ## Error Handling
///
/// ```ignore
/// use paladin::core::platform::container::battalion::BattalionError;
///
/// match CommanderBuilder::new(paladin_port)
///     .strategy(BattalionStrategy::Formation)
///     .paladins(vec![])
///     .build()
/// {
///     Ok(commander) => { /* use commander */ }
///     Err(BattalionError::CommanderValidation(msg)) => {
///         eprintln!("Validation failed: {}", msg);
///     }
///     Err(e) => eprintln!("Build error: {}", e),
/// }
/// ```
pub struct CommanderBuilder {
    strategy: Option<BattalionStrategy>,
    paladins: Option<Vec<Paladin>>,
    config: Option<BattalionConfig>,
    aggregator: Option<Paladin>,
    flow_expression: Option<String>,
    maneuver_config: Option<crate::core::platform::container::battalion::maneuver::ManeuverConfig>,
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
            aggregator: None,
            flow_expression: None,
            maneuver_config: None,
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

    /// Set the aggregator Paladin for Conclave strategy
    ///
    /// The aggregator is responsible for synthesizing expert outputs in Conclave.
    /// If not set and using Conclave strategy, the last Paladin in the list will
    /// be used as the aggregator.
    ///
    /// # Arguments
    ///
    /// * `paladin` - Paladin to use as aggregator
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.aggregator(synthesis_paladin)
    /// ```
    pub fn aggregator(mut self, paladin: Paladin) -> Self {
        self.aggregator = Some(paladin);
        self
    }

    /// Set the flow expression for Maneuver strategy
    ///
    /// The flow expression defines the execution pattern using Flow DSL syntax:
    /// - `agent1 -> agent2` - Sequential execution
    /// - `agent1, agent2` - Parallel execution
    /// - `(agent1 -> agent2), agent3` - Nested patterns
    ///
    /// Required when using BattalionStrategy::Maneuver.
    ///
    /// # Arguments
    ///
    /// * `expression` - Flow DSL expression string
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.flow("analyzer -> enhancer -> (reviewer, validator)")
    /// ```
    pub fn flow(mut self, expression: impl Into<String>) -> Self {
        self.flow_expression = Some(expression.into());
        self
    }

    /// Set the error strategy for Maneuver execution
    ///
    /// Configures how errors should be handled during Maneuver execution.
    ///
    /// # Arguments
    ///
    /// * `strategy` - ManeuverErrorStrategy to use
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paladin::core::platform::container::battalion::maneuver::ErrorStrategy;
    /// builder.error_strategy(ErrorStrategy::ContinueParallel)
    /// ```
    pub fn error_strategy(
        mut self,
        strategy: crate::core::platform::container::battalion::maneuver::ErrorStrategy,
    ) -> Self {
        let mut config = self.maneuver_config.unwrap_or_default();
        config.error_strategy = strategy;
        self.maneuver_config = Some(config);
        self
    }

    /// Set the complete Maneuver configuration
    ///
    /// Provides fine-grained control over Maneuver execution behavior including
    /// error handling, output formatting, timing metrics, and timeouts.
    ///
    /// # Arguments
    ///
    /// * `config` - ManeuverConfig instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paladin::core::platform::container::battalion::maneuver::ManeuverConfig;
    /// let config = ManeuverConfig::default()
    ///     .with_timeout(Duration::from_secs(60))
    ///     .with_timing_metrics(true);
    /// builder.maneuver_config(config)
    /// ```
    pub fn maneuver_config(
        mut self,
        config: crate::core::platform::container::battalion::maneuver::ManeuverConfig,
    ) -> Self {
        self.maneuver_config = Some(config);
        self
    }

    /// Build the Commander instance with validation
    ///
    /// Validates that all required fields are present and returns a configured
    /// Commander ready for execution.
    ///
    /// If no config is provided, generates a default configuration with:
    /// - Name: "default_commander_battalion"
    /// - Timeout: 300 seconds
    /// - Error strategy: FailFast
    /// - Retry policy: 3 attempts with exponential backoff
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
    /// - Config validation fails (timeout_seconds == 0)
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

        // Validate Conclave strategy requirements and handle aggregator
        let aggregator = if strategy == BattalionStrategy::Conclave {
            if paladins.len() < 2 {
                return Err(BattalionError::CommanderValidation(
                    "Conclave requires at least 2 Paladins (for experts)".to_string(),
                ));
            }
            // If no aggregator specified, use the last Paladin as aggregator
            let agg = self.aggregator.unwrap_or_else(|| {
                debug!("No aggregator specified for Conclave, using last Paladin as aggregator");
                paladins.last().cloned().unwrap()
            });
            Some(agg)
        } else {
            self.aggregator // Keep aggregator if explicitly set for other strategies
        };

        // Validate Maneuver strategy requirements
        if strategy == BattalionStrategy::Maneuver {
            if self.flow_expression.is_none() {
                return Err(BattalionError::CommanderValidation(
                    "Maneuver strategy requires a flow expression. Use .flow() to set it."
                        .to_string(),
                ));
            }

            // Validate flow expression can be parsed
            let flow_expr = self.flow_expression.as_ref().unwrap();
            crate::core::platform::container::battalion::parser::FlowParser::parse(flow_expr)
                .map_err(|e| {
                    BattalionError::CommanderValidation(format!("Invalid flow expression: {}", e))
                })?;

            // Validate all agents referenced in flow exist in paladins
            // This will be done at execution time since we need the parsed expression
        }

        // Generate default config if none provided
        let config = self.config.unwrap_or_else(|| {
            debug!("No config provided, generating default configuration");
            BattalionConfig::new("default_commander_battalion")
                .with_timeout(300)
                .with_error_strategy(ErrorStrategy::FailFast)
        });

        // Validate config
        if config.timeout_seconds == 0 {
            return Err(BattalionError::CommanderValidation(
                "Config timeout_seconds must be greater than 0".to_string(),
            ));
        }

        if config.retry_policy.max_attempts == 0 {
            return Err(BattalionError::CommanderValidation(
                "Config retry_policy.max_attempts must be greater than 0".to_string(),
            ));
        }

        // Validate metadata output directory if configured
        config.validate_metadata_dir().map_err(|e| {
            BattalionError::CommanderValidation(format!("Metadata directory error: {}", e))
        })?;

        let mut commander =
            Commander::new(strategy, paladins, config, aggregator, self.paladin_port);

        // Set optional Maneuver fields
        commander.flow_expression = self.flow_expression;
        commander.maneuver_config = self.maneuver_config;

        Ok(commander)
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
    use crate::core::platform::container::paladin::{MaxLoops, PaladinData, PaladinStatus};
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
                ..Default::default()
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
            max_loops: MaxLoops::Fixed(3),
            stop_words: vec![],
            status: PaladinStatus::Idle,
            vision_enabled: false,
            ..Default::default()
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
    fn test_commander_builder_invalid_config() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();

        // Test with zero timeout (invalid)
        let invalid_config = BattalionConfig::new("test").with_timeout(0);

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(vec![paladin])
            .config(invalid_config)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert!(msg.contains("timeout_seconds must be greater than 0"));
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

    #[tokio::test]
    async fn test_execute_routes_to_phalanx_service() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Phalanx)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("Test input").await;
        assert!(result.is_ok(), "Phalanx execution should succeed");
    }

    #[tokio::test]
    #[ignore] // TODO: Requires proper Campaign DAG setup with mock data - move to integration tests
    async fn test_execute_routes_to_campaign_service() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Campaign)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("Test input").await;
        if let Err(ref e) = result {
            eprintln!("Campaign execution error: {:?}", e);
        }
        assert!(
            result.is_ok(),
            "Campaign execution should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore] // TODO: Requires proper ChainOfCommand setup with mock delegation - move to integration tests  
    async fn test_execute_routes_to_chain_service() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::ChainOfCommand)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("Test input").await;
        if let Err(ref e) = result {
            eprintln!("ChainOfCommand execution error: {:?}", e);
        }
        assert!(
            result.is_ok(),
            "ChainOfCommand execution should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_execute_resolves_auto_strategy() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 4];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        // Test with parallel keyword - should select Phalanx and execute
        let result = commander.execute("Run these in parallel").await;
        assert!(
            result.is_ok(),
            "Auto mode with parallel keyword should succeed"
        );

        // Test with sequential keyword - should select Formation and execute
        let result2 = commander.execute("Run these step by step").await;
        assert!(
            result2.is_ok(),
            "Auto mode with sequential keyword should succeed"
        );
    }

    #[tokio::test]
    async fn test_result_contains_strategy_used() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 2];
        let config = create_test_config();

        // Test explicit strategy
        let commander = CommanderBuilder::new(paladin_port.clone())
            .strategy(BattalionStrategy::Formation)
            .paladins(paladins.clone())
            .config(config.clone())
            .build()
            .unwrap();

        let result = commander.execute("Test input").await.unwrap();
        assert_eq!(result.strategy_used, BattalionStrategy::Formation);

        // Test Auto mode resolves to actual strategy (not Auto)
        let commander_auto = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result_auto = commander_auto.execute("Test input").await.unwrap();
        assert_ne!(result_auto.strategy_used, BattalionStrategy::Auto);
        assert_eq!(result_auto.strategy_used, BattalionStrategy::Formation);
    }

    #[tokio::test]
    async fn test_result_contains_selection_reasoning() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        // Test explicit strategy - should have no reasoning
        let commander = CommanderBuilder::new(paladin_port.clone())
            .strategy(BattalionStrategy::Phalanx)
            .paladins(paladins.clone())
            .config(config.clone())
            .build()
            .unwrap();

        let result = commander.execute("Test input").await.unwrap();
        assert!(result.strategy_selection_reasoning.is_none());

        // Test Auto mode - should have reasoning
        let commander_auto = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result_auto = commander_auto
            .execute("Run these in parallel")
            .await
            .unwrap();
        assert!(result_auto.strategy_selection_reasoning.is_some());
        let reasoning = result_auto.strategy_selection_reasoning.unwrap();
        assert!(reasoning.contains("parallel") || reasoning.contains("Phalanx"));
    }

    #[tokio::test]
    async fn test_result_contains_telemetry_metadata() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 2];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("Test input").await.unwrap();

        // Verify all metadata fields are present
        // strategy_selection_time_ms is u64 so always >= 0
        assert!(!result.battalion_id.is_nil());
        assert!(!result.battalion_name.is_empty());
        assert_eq!(result.strategy_used, BattalionStrategy::Formation);
    }

    // Error handling tests require proper mock setup to simulate Paladin failures
    // These are marked as ignored and should be implemented as integration tests

    #[tokio::test]
    #[ignore] // TODO: Requires mock Paladin that can fail - move to integration tests
    async fn test_fail_fast_stops_on_first_error() {
        // Test that FailFast strategy stops on first Paladin error
        // Verify that subsequent Paladins are not executed
        // Verify that error is propagated immediately
    }

    #[tokio::test]
    #[ignore] // TODO: Requires mock Paladin that can fail - move to integration tests
    async fn test_continue_on_error_collects_all_errors() {
        // Test that ContinueOnError executes all Paladins despite errors
        // Verify that all errors are collected in the result
        // Verify that partial results are still returned
    }

    #[tokio::test]
    #[ignore] // TODO: Requires mock Paladin with retry logic - move to integration tests
    async fn test_retry_then_continue_retries_failed_paladins() {
        // Test that RetryThenContinue retries failed Paladins
        // Verify retry_attempts from config is respected
        // Verify that execution continues after retries exhausted
    }

    #[tokio::test]
    #[ignore] // TODO: Requires mock Paladin that can partially fail - move to integration tests
    async fn test_partial_results_returned_with_errors() {
        // Test that successful Paladin results are preserved when others fail
        // Verify that BattalionResult contains both successes and failures
        // Verify metadata correctly tracks success/failure counts
    }

    #[tokio::test]
    async fn test_config_passthrough_to_services() {
        let paladin_port = Arc::new(MockPaladinPort);

        // Create paladin for testing
        let paladin = create_test_paladin();

        // Create config with specific values to verify passthrough
        let config = BattalionConfig::new("test_battalion")
            .with_timeout(600)
            .with_error_strategy(ErrorStrategy::ContinueOnError);

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(vec![paladin])
            .config(config.clone())
            .build()
            .unwrap();

        // Verify config is properly stored
        assert_eq!(commander.config.name, "test_battalion");
        assert_eq!(commander.config.timeout_seconds, 600);
        assert_eq!(
            commander.config.error_strategy,
            ErrorStrategy::ContinueOnError
        );
    }

    #[tokio::test]
    async fn test_timeout_enforcement() {
        // Create a mock that simulates a long-running operation
        struct SlowMockPaladinPort;

        #[async_trait]
        impl PaladinPort for SlowMockPaladinPort {
            async fn execute(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<PaladinResult, crate::application::use_cases::paladin::error::PaladinError>
            {
                // Sleep for 2 seconds to trigger timeout
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                Ok(PaladinResult {
                    output: "slow output".to_string(),
                    token_count: 100,
                    execution_time_ms: 2000,
                    loop_count: 1,
                    stop_reason: StopReason::Completed,
                    ..Default::default()
                })
            }

            async fn execute_stream(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<PaladinStream, crate::application::use_cases::paladin::error::PaladinError>
            {
                unimplemented!()
            }

            fn validate(
                &self,
                _paladin: &Paladin,
            ) -> Result<(), crate::application::use_cases::paladin::error::PaladinError>
            {
                Ok(())
            }
        }

        let paladin_port = Arc::new(SlowMockPaladinPort);
        let paladin1 = create_test_paladin();
        let paladin2 = create_test_paladin();

        // Create config with 1 second timeout
        let config = BattalionConfig::new("timeout_test").with_timeout(1);

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(vec![paladin1, paladin2])
            .config(config)
            .build()
            .unwrap();

        // Execute should timeout
        let result = commander.execute("Test input").await;

        // Verify timeout error
        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::Timeout(seconds) => {
                assert_eq!(seconds, 1);
            }
            other => panic!("Expected Timeout error, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_default_config_generation() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();

        // Build Commander without providing config
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(vec![paladin])
            // Intentionally NOT calling .config()
            .build()
            .unwrap();

        // Verify default config was generated
        assert_eq!(commander.config.name, "default_commander_battalion");
        assert_eq!(commander.config.timeout_seconds, 300);
        assert_eq!(commander.config.error_strategy, ErrorStrategy::FailFast);
        assert_eq!(commander.config.retry_policy.max_attempts, 3);
    }

    #[test]
    fn test_auto_selects_council_for_discussion_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Let's discuss this problem");
        assert_eq!(strategy, BattalionStrategy::Council);
        assert!(reason.contains("discussion") || reason.contains("Council"));

        let (strategy2, _) = commander.analyze_and_select("Debate the best approach");
        assert_eq!(strategy2, BattalionStrategy::Council);

        let (strategy3, _) = commander.analyze_and_select("Collaborate on a solution");
        assert_eq!(strategy3, BattalionStrategy::Council);

        let (strategy4, _) = commander.analyze_and_select("Have a dialogue about this");
        assert_eq!(strategy4, BattalionStrategy::Council);

        let (strategy5, _) = commander.analyze_and_select("Round table discussion needed");
        assert_eq!(strategy5, BattalionStrategy::Council);
    }

    #[test]
    fn test_auto_selects_grove_for_routing_keywords() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let (strategy, reason) = commander.analyze_and_select("Route this to the best agent");
        assert_eq!(strategy, BattalionStrategy::Grove);
        assert!(reason.contains("routing") || reason.contains("Grove"));

        let (strategy2, _) = commander.analyze_and_select("Find the expert for this task");
        assert_eq!(strategy2, BattalionStrategy::Grove);

        let (strategy3, _) = commander.analyze_and_select("Match to the most qualified agent");
        assert_eq!(strategy3, BattalionStrategy::Grove);

        let (strategy4, _) = commander.analyze_and_select("Who is skilled in this area?");
        assert_eq!(strategy4, BattalionStrategy::Grove);

        let (strategy5, _) = commander.analyze_and_select("Dynamic routing based on expertise");
        assert_eq!(strategy5, BattalionStrategy::Grove);
    }

    #[test]
    fn test_council_requires_multiple_paladins() {
        let paladin_port = Arc::new(MockPaladinPort);
        let single_paladin = vec![create_test_paladin()];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(single_paladin)
            .config(config)
            .build()
            .unwrap();

        // With only 1 Paladin, "discuss" keyword should NOT select Council
        // Should fall back to Formation instead
        let (strategy, _) = commander.analyze_and_select("Let's discuss this");
        assert_ne!(strategy, BattalionStrategy::Council);
        assert_eq!(strategy, BattalionStrategy::Formation);
    }

    #[test]
    fn test_grove_requires_multiple_paladins() {
        let paladin_port = Arc::new(MockPaladinPort);
        let single_paladin = vec![create_test_paladin()];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(single_paladin)
            .config(config)
            .build()
            .unwrap();

        // With only 1 Paladin, "route" keyword should NOT select Grove
        // Should fall back to Formation instead
        let (strategy, _) = commander.analyze_and_select("Route to the best agent");
        assert_ne!(strategy, BattalionStrategy::Grove);
        assert_eq!(strategy, BattalionStrategy::Formation);
    }

    #[test]
    fn test_council_and_grove_keywords_are_case_insensitive() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin(); 3];
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        // Test Council with uppercase
        let (strategy1, _) = commander.analyze_and_select("Let's DISCUSS this");
        assert_eq!(strategy1, BattalionStrategy::Council);

        // Test Grove with uppercase
        let (strategy2, _) = commander.analyze_and_select("ROUTE to the best EXPERT");
        assert_eq!(strategy2, BattalionStrategy::Grove);

        // Test mixed case
        let (strategy3, _) = commander.analyze_and_select("Collaborate ON this problem");
        assert_eq!(strategy3, BattalionStrategy::Council);
    }

    #[tokio::test]
    async fn test_maneuver_strategy_explicit() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..3 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        // Test explicit Maneuver strategy with flow expression
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0 -> agent1 -> agent2")
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("Process this workflow").await.unwrap();

        // Verify strategy was Maneuver
        assert_eq!(result.strategy_used, BattalionStrategy::Maneuver);
        assert_eq!(
            result.status,
            crate::core::platform::container::battalion::BattalionStatus::Completed
        );
        assert!(!result.final_output.is_empty());
    }

    // NOTE: These tests are REMOVED because Maneuver is now explicit-only per Task 4.4
    // Maneuver should NOT be selected by Auto mode. To use Maneuver, explicitly set
    // BattalionStrategy::Maneuver via CommanderBuilder.strategy() and provide flow expression.
    //
    // Previous behavior (now removed):
    // - Auto mode would select Maneuver for "flow", "branch", "nested" keywords
    // - Auto mode would select Maneuver for "->" or "|" patterns in input
    //
    // New behavior:
    // - Auto mode will NEVER select Maneuver
    // - Keywords like "flow" and "branch" now route to Campaign or other strategies
    // - Patterns like "->" in natural language don't trigger Maneuver

    #[test]
    fn test_maneuver_requires_at_least_one_paladin() {
        let paladin_port = Arc::new(MockPaladinPort);
        let config = create_test_config();

        // Without paladins, Maneuver strategy should return error
        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent1")
            .paladins(vec![]) // Empty paladins vector
            .config(config)
            .build();

        // Should fail during build validation
        assert!(result.is_err());
    }

    // Task 4.5: Commander integration tests for Maneuver strategy

    #[test]
    fn test_commander_builder_with_flow_expression() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..3 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        // Test CommanderBuilder with flow expression
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0 -> agent1 -> agent2")
            .paladins(paladins)
            .config(config)
            .build();

        assert!(commander.is_ok());
        let commander = commander.unwrap();
        assert_eq!(commander.strategy, BattalionStrategy::Maneuver);
        assert!(commander.flow_expression.is_some());
        assert_eq!(
            commander.flow_expression.unwrap(),
            "agent0 -> agent1 -> agent2"
        );
    }

    #[test]
    fn test_maneuver_without_flow_expression_fails() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        // Maneuver strategy requires flow expression
        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .paladins(vec![paladin])
            .config(config)
            // Intentionally NOT calling .flow()
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert!(msg.contains("flow expression"));
            }
            _ => panic!("Expected CommanderValidation error for missing flow"),
        }
    }

    #[test]
    fn test_maneuver_with_invalid_flow_expression_fails() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        // Invalid flow expression (empty parentheses)
        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent1 -> ()")
            .paladins(vec![paladin])
            .config(config)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::CommanderValidation(msg) => {
                assert!(msg.contains("Invalid flow expression"));
            }
            _ => panic!("Expected CommanderValidation error for invalid flow"),
        }
    }

    #[test]
    fn test_commander_builder_with_error_strategy() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        // Test setting error strategy via builder
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0")
            .error_strategy(crate::core::platform::container::battalion::maneuver::ErrorStrategy::ContinueParallel)
            .paladins(vec![paladin])
            .config(config)
            .build();

        assert!(commander.is_ok());
        let commander = commander.unwrap();
        assert!(commander.maneuver_config.is_some());
        assert_eq!(
            commander.maneuver_config.unwrap().error_strategy,
            crate::core::platform::container::battalion::maneuver::ErrorStrategy::ContinueParallel
        );
    }

    #[test]
    fn test_commander_builder_with_maneuver_config() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladin = create_test_paladin();
        let config = create_test_config();

        // Test setting complete ManeuverConfig
        let maneuver_config =
            crate::core::platform::container::battalion::maneuver::ManeuverConfig::default()
                .with_timeout(std::time::Duration::from_secs(60))
                .with_timing_metrics(false);

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0")
            .maneuver_config(maneuver_config.clone())
            .paladins(vec![paladin])
            .config(config)
            .build();

        assert!(commander.is_ok());
        let commander = commander.unwrap();
        assert!(commander.maneuver_config.is_some());
        let stored_config = commander.maneuver_config.unwrap();
        assert_eq!(
            stored_config.timeout,
            Some(std::time::Duration::from_secs(60))
        );
        assert!(!stored_config.collect_timing_metrics);
    }

    #[tokio::test]
    async fn test_maneuver_execution_through_commander() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..3 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        // Test execution through Commander with Maneuver
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0 -> agent1 -> agent2")
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("test input").await;
        assert!(result.is_ok(), "Maneuver execution should succeed");

        let result = result.unwrap();
        assert_eq!(result.strategy_used, BattalionStrategy::Maneuver);
        assert!(!result.final_output.is_empty());
    }

    #[test]
    fn test_auto_strategy_does_not_select_maneuver() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..3 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Auto)
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        // Test various inputs that should NOT select Maneuver
        // Maneuver is explicit-only per Task 4.4

        // Input with arrow (could be confused with flow DSL)
        let (strategy1, _) = commander.analyze_and_select("Process step1 -> step2 -> step3");
        assert_ne!(
            strategy1,
            BattalionStrategy::Maneuver,
            "Auto should not select Maneuver even with -> in input"
        );

        // Input with "flow" keyword
        let (strategy2, _) = commander.analyze_and_select("Create a flow for this task");
        assert_ne!(
            strategy2,
            BattalionStrategy::Maneuver,
            "Auto should not select Maneuver for 'flow' keyword"
        );

        // Input with "branch" keyword
        let (strategy3, _) = commander.analyze_and_select("Branch execution based on results");
        assert_ne!(
            strategy3,
            BattalionStrategy::Maneuver,
            "Auto should not select Maneuver for 'branch' keyword"
        );

        // Input with pipe character
        let (strategy4, _) = commander.analyze_and_select("Run agent1 | agent2 | agent3");
        assert_ne!(
            strategy4,
            BattalionStrategy::Maneuver,
            "Auto should not select Maneuver even with | in input"
        );
    }

    #[tokio::test]
    async fn test_maneuver_with_parallel_pattern() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..3 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        // Test with parallel pattern
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0, agent1, agent2")
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("test input").await;
        assert!(
            result.is_ok(),
            "Maneuver with parallel pattern should succeed"
        );
    }

    #[tokio::test]
    async fn test_maneuver_with_nested_pattern() {
        let paladin_port = Arc::new(MockPaladinPort);
        let mut paladins = vec![];
        for i in 0..4 {
            let data = PaladinData {
                system_prompt: format!("Agent {}", i),
                name: format!("agent{}", i),
                user_name: "test".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_loops: MaxLoops::Fixed(3),
                stop_words: vec![],
                status: PaladinStatus::Idle,
                vision_enabled: false,
                ..Default::default()
            };
            paladins.push(Node::new(data, Some(format!("agent{}", i))));
        }
        let config = create_test_config();

        // Test with nested pattern: agent0 -> (agent1 -> agent2)
        // This creates Sequential(agent0, Sequential(agent1, agent2))
        let commander = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Maneuver)
            .flow("agent0 -> (agent1 -> agent2)")
            .paladins(paladins)
            .config(config)
            .build()
            .unwrap();

        let result = commander.execute("test input").await;
        if let Err(ref e) = result {
            eprintln!("Error: {:?}", e);
        }
        assert!(
            result.is_ok(),
            "Maneuver with nested sequential pattern should succeed: {:?}",
            result.err()
        );
    }

    // ── Task 8.0: Commander metadata export configuration tests ──

    #[tokio::test]
    async fn test_commander_build_with_valid_metadata_dir() {
        let dir = std::env::temp_dir().join("paladin_cmd_meta_valid_8_0");
        let _ = std::fs::remove_dir_all(&dir);

        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin()];
        let config = BattalionConfig::new("meta_test")
            .with_timeout(120)
            .with_metadata_dir(dir.clone());

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(paladins)
            .config(config)
            .build();

        assert!(
            result.is_ok(),
            "Build should succeed with valid metadata dir"
        );
        assert!(dir.exists(), "Metadata dir should be auto-created");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_commander_build_without_metadata_dir() {
        let paladin_port = Arc::new(MockPaladinPort);
        let paladins = vec![create_test_paladin()];
        let config = BattalionConfig::new("no_meta_test").with_timeout(120);

        let result = CommanderBuilder::new(paladin_port)
            .strategy(BattalionStrategy::Formation)
            .paladins(paladins)
            .config(config)
            .build();

        assert!(result.is_ok(), "Build should succeed without metadata dir");
    }
}
