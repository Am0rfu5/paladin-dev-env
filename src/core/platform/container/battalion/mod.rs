//! Battalion Orchestration Base Types
//!
//! This module provides the core domain types for multi-Paladin orchestration.
//! Battalions coordinate multiple Paladins using various execution patterns.

pub mod campaign;
pub mod chain_of_command;
pub mod conclave;
pub mod council;
pub mod formation;
pub mod grove;
pub mod maneuver;
pub mod parser;
pub mod phalanx;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

// Note: PaladinResult is defined in application layer, but we reference it here
// for the domain result type. This is acceptable as Battalion results contain
// references to execution outcomes.
use crate::application::ports::output::paladin_port::PaladinResult;

/// Configuration for Battalion operations
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy};
///
/// let config = BattalionConfig::new("research_battalion")
///     .with_timeout(300)
///     .with_error_strategy(ErrorStrategy::FailFast);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionConfig {
    /// Name of the Battalion
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Maximum execution time in seconds
    pub timeout_seconds: u64,

    /// Retry policy for failed operations
    pub retry_policy: RetryPolicy,

    /// Strategy for handling errors
    pub error_strategy: ErrorStrategy,

    /// Directory for saving metadata output
    pub metadata_output_dir: Option<PathBuf>,
}

impl BattalionConfig {
    /// Create a new BattalionConfig with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name identifier for this Battalion
    ///
    /// Uses default values for all other configuration options.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            timeout_seconds: 300,
            retry_policy: RetryPolicy::default(),
            error_strategy: ErrorStrategy::default(),
            metadata_output_dir: None,
        }
    }

    /// Set the description (builder pattern).
    ///
    /// # Arguments
    ///
    /// * `description` - Human-readable description of the Battalion's purpose
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the timeout in seconds (builder pattern).
    ///
    /// # Arguments
    ///
    /// * `seconds` - Maximum execution time before timing out
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Set the retry policy
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Set the error strategy
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Set the metadata output directory
    pub fn with_metadata_dir(mut self, dir: PathBuf) -> Self {
        self.metadata_output_dir = Some(dir);
        self
    }
}

impl Default for BattalionConfig {
    /// Creates a BattalionConfig with default values.
    ///
    /// # Default Values
    /// - `name`: "default_battalion"
    /// - `timeout_seconds`: 300 (5 minutes)
    /// - `retry_policy`: RetryPolicy::default()
    /// - `error_strategy`: ErrorStrategy::FailFast
    fn default() -> Self {
        Self::new("default_battalion")
    }
}

/// Retry policy configuration for Battalion operations.
///
/// Defines how failed operations should be retried with exponential backoff
/// and jitter to prevent thundering herd problems.
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::RetryPolicy;
/// use std::time::Duration;
///
/// let policy = RetryPolicy {
///     max_attempts: 5,
///     base_delay: Duration::from_millis(200),
///     max_delay: Duration::from_secs(30),
///     exponential_backoff: true,
///     jitter: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,

    /// Base delay between retries
    pub base_delay: Duration,

    /// Maximum delay between retries
    pub max_delay: Duration,

    /// Whether to use exponential backoff
    pub exponential_backoff: bool,

    /// Whether to add jitter to prevent thundering herd
    pub jitter: bool,
}

impl Default for RetryPolicy {
    /// Creates a RetryPolicy with sensible defaults.
    ///
    /// # Default Values
    /// - `max_attempts`: 3
    /// - `base_delay`: 100ms
    /// - `max_delay`: 10s
    /// - `exponential_backoff`: true
    /// - `jitter`: true
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            exponential_backoff: true,
            jitter: true,
        }
    }
}

/// Strategy for handling errors during Battalion execution.
///
/// Determines how the Battalion should respond when individual Paladin executions fail.
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::ErrorStrategy;
///
/// let fail_fast = ErrorStrategy::FailFast; // Stop on first error
/// let continue_on_error = ErrorStrategy::ContinueOnError; // Collect all errors
/// let retry = ErrorStrategy::RetryThenContinue; // Retry then proceed
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ErrorStrategy {
    /// Stop immediately on first error
    #[default]
    FailFast,

    /// Continue execution despite errors, collect all at end
    ContinueOnError,

    /// Retry failed operations, then continue
    RetryThenContinue,
}

/// Battalion orchestration strategy
///
/// Defines the pattern used to coordinate multiple Paladins.
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::BattalionStrategy;
///
/// let strategy = BattalionStrategy::Formation;
/// assert_eq!(strategy, BattalionStrategy::Formation);
///
/// let auto = BattalionStrategy::Auto;
/// // Auto will be resolved to a specific strategy at runtime
///
/// // Explicit strategy selection
/// let strategy = BattalionStrategy::Formation;
///
/// // Auto mode for intelligent selection
/// let auto_strategy = BattalionStrategy::Auto;
///
/// // Explicit strategy for known patterns
/// let formation = BattalionStrategy::Formation;
/// let phalanx = BattalionStrategy::Phalanx;
///
/// // Pattern matching
/// match formation {
///     BattalionStrategy::Formation => println!("Sequential pipeline"),
///     BattalionStrategy::Phalanx => println!("Parallel execution"),
///     BattalionStrategy::Campaign => println!("Graph workflow"),
///     BattalionStrategy::ChainOfCommand => println!("Hierarchical delegation"),
///     BattalionStrategy::Conclave => println!("Multi-expert synthesis"),
///     BattalionStrategy::Auto => println!("Automatic selection"),
/// }
/// ```
///
/// # Strategy Descriptions
///
/// ## Formation
///
/// Sequential execution where each Paladin output becomes input for the next Paladin.
/// Ideal for pipelines and multi-stage transformations.
///
/// Use When:
/// - Tasks must be performed in a specific order
/// - Each step depends on the previous step output
/// - Data flows through a linear transformation pipeline
///
/// Example Use Cases:
/// - Research -> Analysis -> Summary workflow
/// - Data extraction -> Transformation -> Loading (ETL)
/// - Draft -> Edit -> Review document workflow
///
/// ## Phalanx
///
/// Concurrent parallel execution where all Paladins receive the same input and execute
/// simultaneously. Results are aggregated at the end.
/// Ideal for independent parallel tasks.
///
/// Use When:
/// - Tasks can run independently without dependencies
/// - All tasks need the same input data
/// - Want to maximize throughput via parallelism
///
/// Example Use Cases:
/// - Analyzing same data with different models
/// - Batch processing independent items
/// - Multi-perspective analysis (technical, business, legal review in parallel)
///
/// ## Campaign
///
/// Graph/DAG-based execution with conditional branching and complex dependencies.
/// Paladins are organized in a directed graph with conditional edges.
/// Ideal for complex workflows with branching logic.
///
/// Use When:
/// - Workflow has conditional branching (if-then-else)
/// - Dependencies form a complex graph (not just linear)
/// - Need dynamic routing based on intermediate results
///
/// Example Use Cases:
/// - Approval workflows with escalation paths
/// - Multi-stage decision trees
/// - Error handling with fallback paths
///
/// ## ChainOfCommand
///
/// Hierarchical delegation where a commander Paladin analyzes the task and delegates
/// to specialist Paladins based on expertise matching.
/// Ideal for dynamic task routing to specialists.
///
/// Use When:
/// - Have specialized Paladins with different expertise
/// - Task requires intelligent routing to the right specialist
/// - Need hierarchical decision-making
///
/// Example Use Cases:
/// - Customer support routing to specialized agents
/// - Code review routing to domain experts
/// - Medical triage routing to specialists
///
/// ## Auto
///
/// Automatic strategy selection based on intelligent heuristics analyzing:
/// - Input keywords such as parallel, sequential, workflow, delegate
/// - Number of Paladins (1-3 uses Formation, 4+ considers parallelism)
/// - Task characteristics
///
/// Use When:
/// - Want the framework to select the optimal pattern
/// - Building general-purpose orchestration APIs
/// - Prototyping or exploring different patterns
///
/// Selection Rules:
/// - Formation: Keywords like sequential, step-by-step, pipeline; or 1-3 Paladins
/// - Phalanx: Keywords like parallel, concurrent, simultaneously; or 4+ similar tasks
/// - Campaign: Keywords like workflow, conditional, if-then, depends-on
/// - ChainOfCommand: Keywords like delegate, specialist, expert, route-to
/// - Conclave: Keywords like synthesize, compare, expert panel, consensus; or 3+ diverse experts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattalionStrategy {
    /// Sequential execution with output chaining (Paladin N output -> Paladin N+1 input)
    ///
    /// Best for linear pipelines where each stage depends on the previous stage's output.
    Formation,

    /// Concurrent parallel execution (all Paladins receive same input, results aggregated)
    ///
    /// Best for independent tasks that can run simultaneously to maximize throughput.
    Phalanx,

    /// Graph/DAG-based orchestration (conditional branching and complex workflows)
    ///
    /// Best for complex workflows with conditional logic and multi-path dependencies.
    Campaign,

    /// Hierarchical delegation pattern (commander delegates to specialist Paladins)
    ///
    /// Best for dynamic routing to specialists based on task characteristics.
    ChainOfCommand,

    /// Mixture of Agents pattern (multiple expert Paladins analyze in parallel, aggregator synthesizes)
    ///
    /// Best for complex analytical tasks requiring diverse expert perspectives with synthesis.
    /// All expert Paladins process the input independently in parallel with retry logic,
    /// then an aggregator Paladin synthesizes their outputs into a comprehensive response.
    ///
    /// Particularly effective for:
    /// - Multi-perspective analysis (legal + technical + business review)
    /// - Comparative evaluations (pros/cons from different viewpoints)
    /// - Expert consensus building
    /// - Complex decision-making requiring diverse expertise
    Conclave,

    /// Conversational multi-agent pattern (Paladins engage in turn-based discussion)
    ///
    /// Best for collaborative problem-solving through structured dialogue with flexible turn-taking.
    /// Paladins take turns contributing to a shared conversation using configurable turn strategies
    /// (RoundRobin, ModeratorDirected, Random, VoluntaryWithTimeout) until a termination condition
    /// is met (MaxRounds, Consensus, ModeratorDecision, Keyword detection).
    ///
    /// Particularly effective for:
    /// - Collaborative brainstorming and idea refinement
    /// - Debate and deliberation with opposing viewpoints
    /// - Consensus-building discussions
    /// - Iterative problem-solving with feedback
    Council,

    /// Tree-based intelligent routing pattern (routes tasks to specialized agents by expertise)
    ///
    /// Best for dynamically routing tasks to the most qualified agent based on expertise matching.
    /// Uses configurable routing strategies (KeywordMatch, SemanticSimilarity, LlmRouting) to
    /// analyze task requirements and select the optimal agent from a hierarchical tree structure.
    ///
    /// Particularly effective for:
    /// - Customer support routing to specialized departments
    /// - Task assignment based on domain expertise
    /// - Dynamic workload distribution by capability
    /// - Intelligent delegation in complex organizations
    Grove,

    /// Automatic strategy selection based on heuristics
    ///
    /// Analyzes input and Paladin characteristics to intelligently select Formation,
    /// Phalanx, Campaign, ChainOfCommand, Conclave, Council, or Grove. Provides reasoning for transparency.
    Auto,
}

/// Current status of a Battalion execution.
///
/// Tracks the lifecycle state of a Battalion from creation through completion.
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::BattalionStatus;
///
/// let mut status = BattalionStatus::Idle;
/// status = BattalionStatus::Running;
/// // ... execution happens ...
/// status = BattalionStatus::Completed;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattalionStatus {
    /// Battalion is idle, not yet started
    #[default]
    Idle,

    /// Battalion is currently executing
    Running,

    /// Battalion execution is paused
    Paused,

    /// Battalion completed successfully
    Completed,

    /// Battalion failed with errors
    Failed,

    /// Battalion was cancelled
    Cancelled,
}

/// Result of a Battalion execution
///
/// Contains the final output, individual Paladin results, and execution metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionResult {
    /// Unique identifier for this execution
    pub battalion_id: Uuid,

    /// Name of the Battalion
    pub battalion_name: String,

    /// When execution started
    pub started_at: DateTime<Utc>,

    /// When execution completed
    pub completed_at: DateTime<Utc>,

    /// Final aggregated output
    pub final_output: String,

    /// Individual Paladin results
    pub paladin_results: Vec<PaladinResult>,

    /// Final status
    pub status: BattalionStatus,

    /// The orchestration strategy that was used for execution
    /// For Auto mode, this contains the resolved strategy, not Auto
    pub strategy_used: BattalionStrategy,

    /// Reasoning for strategy selection (only present for Auto mode)
    pub strategy_selection_reasoning: Option<String>,

    /// Time spent on strategy selection in milliseconds
    pub strategy_selection_time_ms: u64,

    /// Execution time for each Paladin in milliseconds
    pub per_paladin_times: Vec<u64>,

    /// Count of Paladins that completed successfully
    pub paladin_success_count: usize,

    /// Count of Paladins that failed
    pub paladin_failure_count: usize,
}

impl BattalionResult {
    /// Create a new BattalionResult
    pub fn new(
        battalion_id: Uuid,
        battalion_name: String,
        started_at: DateTime<Utc>,
        final_output: String,
        paladin_results: Vec<PaladinResult>,
    ) -> Self {
        // Count successes and failures based on stop_reason
        let paladin_success_count = paladin_results
            .iter()
            .filter(|r| {
                matches!(
                    r.stop_reason,
                    crate::application::ports::output::paladin_port::StopReason::Completed
                )
            })
            .count();
        let paladin_failure_count = paladin_results.len() - paladin_success_count;

        Self {
            battalion_id,
            battalion_name,
            started_at,
            completed_at: Utc::now(),
            final_output,
            paladin_results,
            status: BattalionStatus::Completed,
            strategy_used: BattalionStrategy::Formation, // Default to Formation
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times: Vec::new(),
            paladin_success_count,
            paladin_failure_count,
        }
    }

    /// Create from a list of Paladin results (sequential execution)
    pub fn from_paladin_results(
        battalion_id: Uuid,
        battalion_name: String,
        started_at: DateTime<Utc>,
        results: Vec<PaladinResult>,
    ) -> Self {
        let final_output = results.last().map(|r| r.output.clone()).unwrap_or_default();

        Self::new(
            battalion_id,
            battalion_name,
            started_at,
            final_output,
            results,
        )
    }

    /// Get execution duration
    pub fn duration(&self) -> Duration {
        (self.completed_at - self.started_at)
            .to_std()
            .unwrap_or_default()
    }

    /// Set the strategy used for this execution
    pub fn with_strategy(mut self, strategy: BattalionStrategy) -> Self {
        self.strategy_used = strategy;
        self
    }

    /// Set the strategy selection reasoning (for Auto mode)
    pub fn with_selection_reasoning(mut self, reasoning: String) -> Self {
        self.strategy_selection_reasoning = Some(reasoning);
        self
    }

    /// Set the strategy selection time in milliseconds
    pub fn with_selection_time_ms(mut self, time_ms: u64) -> Self {
        self.strategy_selection_time_ms = time_ms;
        self
    }

    /// Set the per-Paladin execution times
    pub fn with_paladin_times(mut self, times: Vec<u64>) -> Self {
        self.per_paladin_times = times;
        self
    }
}

/// Errors specific to Council pattern operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum CouncilError {
    /// No participants configured in the Council
    #[error("No participants configured")]
    NoParticipants,

    /// Moderator required for ModeratorDirected strategy
    #[error("Moderator required for ModeratorDirected strategy")]
    ModeratorRequired,

    /// Participant execution failed
    #[error("Participant execution failed: {0}")]
    ParticipantError(String),

    /// Invalid turn strategy configuration
    #[error("Invalid turn strategy configuration: {0}")]
    InvalidStrategy(String),

    /// Maximum rounds must be greater than zero
    #[error("Maximum rounds must be greater than zero")]
    InvalidMaxRounds,
}

/// Errors specific to Grove pattern operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum GroveError {
    /// No trees configured in the Grove
    #[error("No trees configured")]
    NoTrees,

    /// No agents available in the Grove
    #[error("No agents in grove")]
    NoAgents,

    /// Routing operation failed
    #[error("Routing failed: {0}")]
    RoutingFailed(String),

    /// No agent meets the similarity threshold
    #[error("No agent meets similarity threshold {0}")]
    NoMatchingAgent(f32),

    /// Embeddings required for SemanticSimilarity strategy
    #[error("Embeddings required for SemanticSimilarity strategy")]
    EmbeddingsRequired,

    /// Invalid similarity threshold (must be 0.0-1.0)
    #[error("Invalid similarity threshold: {0} (must be between 0.0 and 1.0)")]
    InvalidSimilarityThreshold(f32),
}

/// Error types for Battalion operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum BattalionError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Paladin execution error
    #[error("Paladin error: {0}")]
    PaladinError(String),

    /// Formation-specific error
    #[error("Formation error: {0}")]
    FormationError(String),

    /// Phalanx-specific error
    #[error("Phalanx error: {0}")]
    PhalanxError(String),

    /// Campaign-specific error
    #[error("Campaign error: {0}")]
    CampaignError(String),

    /// Invalid graph structure
    #[error("Invalid graph: {0}")]
    InvalidGraph(String),

    /// Chain of Command error
    #[error("Chain of Command error: {0}")]
    ChainOfCommandError(String),

    /// Commander validation error
    #[error("Commander validation error: {0}")]
    CommanderValidation(String),

    /// Strategy selection error
    #[error("Strategy selection failed: {0}")]
    StrategySelection(String),

    /// Council pattern error
    #[error("Council error: {0}")]
    CouncilError(#[from] CouncilError),

    /// Grove pattern error
    #[error("Grove error: {0}")]
    GroveError(#[from] GroveError),

    /// Routing error (Grove pattern) - kept for backward compatibility
    #[error("Routing error: {0}")]
    RoutingError(String),

    /// Timeout error
    #[error("Battalion execution timed out after {0} seconds")]
    Timeout(u64),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Aggregation error
    #[error("Aggregation error: {0}")]
    AggregationError(String),

    /// Battalion execution cancelled
    #[error("Battalion execution was cancelled")]
    Cancelled,

    /// General execution error
    #[error("Execution error: {0}")]
    ExecutionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battalion_config_default() {
        let config = BattalionConfig::default();
        assert_eq!(config.name, "default_battalion");
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.error_strategy, ErrorStrategy::FailFast);
    }

    #[test]
    fn test_battalion_config_builder() {
        let config = BattalionConfig::new("test_battalion")
            .with_description("Test description")
            .with_timeout(600)
            .with_error_strategy(ErrorStrategy::ContinueOnError);

        assert_eq!(config.name, "test_battalion");
        assert_eq!(config.description, Some("Test description".to_string()));
        assert_eq!(config.timeout_seconds, 600);
        assert_eq!(config.error_strategy, ErrorStrategy::ContinueOnError);
    }

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.base_delay, Duration::from_millis(100));
        assert_eq!(policy.max_delay, Duration::from_secs(10));
        assert!(policy.exponential_backoff);
        assert!(policy.jitter);
    }

    #[test]
    fn test_error_strategy_variants() {
        let fail_fast = ErrorStrategy::FailFast;
        let continue_on_error = ErrorStrategy::ContinueOnError;
        let retry_then_continue = ErrorStrategy::RetryThenContinue;

        assert_eq!(fail_fast, ErrorStrategy::FailFast);
        assert_eq!(continue_on_error, ErrorStrategy::ContinueOnError);
        assert_eq!(retry_then_continue, ErrorStrategy::RetryThenContinue);
        assert_ne!(fail_fast, continue_on_error);
    }

    #[test]
    fn test_error_strategy_default() {
        let strategy = ErrorStrategy::default();
        assert_eq!(strategy, ErrorStrategy::FailFast);
    }

    #[test]
    fn test_battalion_status_variants() {
        assert_eq!(BattalionStatus::default(), BattalionStatus::Idle);

        let statuses = vec![
            BattalionStatus::Idle,
            BattalionStatus::Running,
            BattalionStatus::Paused,
            BattalionStatus::Completed,
            BattalionStatus::Failed,
            BattalionStatus::Cancelled,
        ];

        for status in statuses {
            // Verify each status can be created
            let _ = status;
        }
    }

    #[test]
    fn test_battalion_result_new() {
        let battalion_id = Uuid::new_v4();
        let started_at = Utc::now();

        let result = BattalionResult::new(
            battalion_id,
            "test_battalion".to_string(),
            started_at,
            "final output".to_string(),
            vec![],
        );

        assert_eq!(result.battalion_id, battalion_id);
        assert_eq!(result.battalion_name, "test_battalion");
        assert_eq!(result.final_output, "final output");
        assert_eq!(result.status, BattalionStatus::Completed);
        assert!(result.paladin_results.is_empty());
    }

    #[test]
    fn test_battalion_result_duration() {
        let battalion_id = Uuid::new_v4();
        let started_at = Utc::now();

        let mut result = BattalionResult::new(
            battalion_id,
            "test_battalion".to_string(),
            started_at,
            "output".to_string(),
            vec![],
        );

        // Set completed_at to 2 seconds after started_at
        result.completed_at = started_at + chrono::Duration::seconds(2);

        let duration = result.duration();
        assert_eq!(duration.as_secs(), 2);
    }

    #[test]
    fn test_battalion_config_serialization() {
        let config = BattalionConfig::new("test").with_timeout(120);

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: BattalionConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    }

    #[test]
    fn test_retry_policy_serialization() {
        let policy = RetryPolicy::default();

        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: RetryPolicy = serde_json::from_str(&json).unwrap();

        assert_eq!(policy.max_attempts, deserialized.max_attempts);
        assert_eq!(policy.exponential_backoff, deserialized.exponential_backoff);
    }

    #[test]
    fn test_error_strategy_serialization() {
        let strategy = ErrorStrategy::ContinueOnError;

        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: ErrorStrategy = serde_json::from_str(&json).unwrap();

        assert_eq!(strategy, deserialized);
    }

    #[test]
    fn test_battalion_status_serialization() {
        let status = BattalionStatus::Running;

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: BattalionStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_battalion_result_serialization() {
        let result = BattalionResult::new(
            Uuid::new_v4(),
            "test".to_string(),
            Utc::now(),
            "output".to_string(),
            vec![],
        );

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: BattalionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.battalion_id, deserialized.battalion_id);
        assert_eq!(result.battalion_name, deserialized.battalion_name);
        assert_eq!(result.final_output, deserialized.final_output);
    }

    #[test]
    fn test_battalion_error_variants() {
        let errors = vec![
            BattalionError::ConfigurationError("test".to_string()),
            BattalionError::PaladinError("test".to_string()),
            BattalionError::FormationError("test".to_string()),
            BattalionError::PhalanxError("test".to_string()),
            BattalionError::CampaignError("test".to_string()),
            BattalionError::InvalidGraph("test".to_string()),
            BattalionError::ChainOfCommandError("test".to_string()),
            BattalionError::Timeout(300),
            BattalionError::ValidationError("test".to_string()),
            BattalionError::AggregationError("test".to_string()),
        ];

        // Verify error messages
        for error in errors {
            let msg = error.to_string();
            assert!(!msg.is_empty());
        }
    }
}

#[test]
fn test_battalion_strategy_creation() {
    let formation = BattalionStrategy::Formation;
    let phalanx = BattalionStrategy::Phalanx;
    let campaign = BattalionStrategy::Campaign;
    let chain = BattalionStrategy::ChainOfCommand;
    let auto = BattalionStrategy::Auto;

    assert_eq!(formation, BattalionStrategy::Formation);
    assert_eq!(phalanx, BattalionStrategy::Phalanx);
    assert_eq!(campaign, BattalionStrategy::Campaign);
    assert_eq!(chain, BattalionStrategy::ChainOfCommand);
    assert_eq!(auto, BattalionStrategy::Auto);
    assert_ne!(formation, phalanx);
}

#[test]
fn test_battalion_strategy_serialization() {
    let strategy = BattalionStrategy::Formation;
    let serialized = serde_json::to_string(&strategy).unwrap();
    let deserialized: BattalionStrategy = serde_json::from_str(&serialized).unwrap();
    assert_eq!(strategy, deserialized);

    let auto = BattalionStrategy::Auto;
    let serialized = serde_json::to_string(&auto).unwrap();
    let deserialized: BattalionStrategy = serde_json::from_str(&serialized).unwrap();
    assert_eq!(auto, deserialized);
}
