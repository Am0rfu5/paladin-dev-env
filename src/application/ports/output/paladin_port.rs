//! Paladin Port Abstraction
//!
//! This module defines the port (interface) for Paladin execution following
//! the hexagonal architecture pattern. Implementations of this port handle
//! the actual execution logic while the domain remains independent.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::application::use_cases::paladin::error::PaladinError;
use crate::core::platform::container::handoff::HandoffRecord;
use crate::core::platform::container::paladin::Paladin;
use crate::core::platform::container::planning::TaskPlan;

/// Result of a Paladin execution
///
/// Contains the output, metadata about execution, and the reason for completion.
/// Optionally includes autonomous execution metadata like task plans and handoff history.
///
/// # Example
///
/// ```
/// use paladin::application::ports::output::paladin_port::{PaladinResult, StopReason};
///
/// let result = PaladinResult {
///     output: "The answer is 42".to_string(),
///     token_count: 150,
///     execution_time_ms: 1250,
///     loop_count: 1,
///     stop_reason: StopReason::Completed,
///     plan: None,
///     handoff_history: Vec::new(),
/// };
///
/// assert_eq!(result.loop_count, 1);
/// assert!(result.stop_reason.is_successful());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinResult {
    /// The generated output text
    pub output: String,

    /// Total number of tokens used (prompt + completion)
    pub token_count: u32,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Number of reasoning loops executed
    pub loop_count: u32,

    /// Reason why execution stopped
    pub stop_reason: StopReason,

    /// Task plan generated during autonomous planning mode
    ///
    /// When a Paladin runs in autonomous planning mode (MaxLoops::Auto),
    /// this field contains the decomposed task plan with subtasks and their results.
    /// This provides transparency into how the Paladin broke down the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TaskPlan>,

    /// History of agent handoffs during execution
    ///
    /// When a Paladin delegates tasks to specialist agents, each handoff
    /// is recorded here for transparency and debugging. The records include
    /// which agent handled what task and at what depth in the delegation chain.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handoff_history: Vec<HandoffRecord>,
}

/// Reason why Paladin execution stopped
///
/// Indicates whether the Paladin completed successfully or was terminated
/// by a limit or external factor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopReason {
    /// Maximum loop iterations reached
    MaxLoops,

    /// A configured stop word was detected
    StopWord(String),

    /// Execution completed naturally
    Completed,

    /// Execution exceeded timeout
    Timeout,
}

impl StopReason {
    /// Check if this represents successful completion
    pub fn is_successful(&self) -> bool {
        matches!(self, StopReason::Completed | StopReason::StopWord(_))
    }

    /// Check if this represents a limit being reached
    pub fn is_limit(&self) -> bool {
        matches!(self, StopReason::MaxLoops | StopReason::Timeout)
    }
}

impl Default for PaladinResult {
    /// Creates a PaladinResult with default values
    ///
    /// Useful for testing and as a base for builder patterns.
    fn default() -> Self {
        Self {
            output: String::new(),
            token_count: 0,
            execution_time_ms: 0,
            loop_count: 0,
            stop_reason: StopReason::Completed,
            plan: None,
            handoff_history: Vec::new(),
        }
    }
}

impl PaladinResult {
    /// Creates a new PaladinResult with required fields
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::application::ports::output::paladin_port::{PaladinResult, StopReason};
    ///
    /// let result = PaladinResult::new(
    ///     "Response text".to_string(),
    ///     100,
    ///     500,
    ///     1,
    ///     StopReason::Completed
    /// );
    /// ```
    pub fn new(
        output: String,
        token_count: u32,
        execution_time_ms: u64,
        loop_count: u32,
        stop_reason: StopReason,
    ) -> Self {
        Self {
            output,
            token_count,
            execution_time_ms,
            loop_count,
            stop_reason,
            plan: None,
            handoff_history: Vec::new(),
        }
    }

    /// Checks if this result includes autonomous planning metadata
    pub fn has_plan(&self) -> bool {
        self.plan.is_some()
    }

    /// Checks if this result includes handoff history
    pub fn has_handoffs(&self) -> bool {
        !self.handoff_history.is_empty()
    }

    /// Returns the number of handoffs in the history
    pub fn handoff_count(&self) -> usize {
        self.handoff_history.len()
    }
}

/// Streaming chunk from Paladin execution
///
/// Represents a single chunk of output during streaming execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinStreamChunk {
    /// The text chunk
    pub text: String,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Optional metadata for this chunk
    pub metadata: Option<ChunkMetadata>,
}

/// Metadata for a streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Tokens in this chunk
    pub tokens: Option<u32>,

    /// Current loop iteration
    pub loop_count: Option<u32>,
}

/// Type alias for Paladin streaming receiver
///
/// Receives chunks of output as they are generated.
pub type PaladinStream = mpsc::Receiver<Result<PaladinStreamChunk, PaladinError>>;

/// Port abstraction for Paladin execution
///
/// This trait defines the interface that any Paladin execution implementation
/// must satisfy. It follows the hexagonal architecture pattern, allowing the
/// core domain to remain independent of execution details.
///
/// # Example Implementation
///
/// ```ignore
/// use async_trait::async_trait;
/// use paladin::application::ports::output::paladin_port::{PaladinPort, PaladinResult, StopReason};
/// use paladin::core::platform::container::paladin::Paladin;
/// use paladin::application::use_cases::paladin::error::PaladinError;
///
/// struct MyPaladinExecutor;
///
/// #[async_trait]
/// impl PaladinPort for MyPaladinExecutor {
///     async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
///         // Implementation here
///         Ok(PaladinResult {
///             output: "Result".to_string(),
///             token_count: 100,
///             execution_time_ms: 500,
///             loop_count: 1,
///             stop_reason: StopReason::Completed,
///         })
///     }
///
///     async fn execute_stream(
///         &self,
///         paladin: &Paladin,
///         input: &str,
///     ) -> Result<PaladinStream, PaladinError> {
///         // Streaming implementation
///         let (_tx, rx) = tokio::sync::mpsc::channel(10);
///         Ok(rx)
///     }
///
///     fn validate(&self, paladin: &Paladin) -> Result<(), PaladinError> {
///         // Validation logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait PaladinPort: Send + Sync {
    /// Execute a Paladin with the given input
    ///
    /// Runs the Paladin's reasoning loop until completion, timeout, or stop word detection.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin entity to execute
    /// * `input` - The user input to process
    ///
    /// # Returns
    ///
    /// Returns a `PaladinResult` on success or a `PaladinError` on failure.
    ///
    /// # Errors
    ///
    /// * `PaladinError::ConfigurationError` - Invalid Paladin configuration
    /// * `PaladinError::ExecutionError` - Error during execution
    /// * `PaladinError::Timeout` - Execution exceeded timeout
    /// * `PaladinError::LlmError` - LLM provider error
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError>;

    /// Execute a Paladin with streaming output
    ///
    /// Similar to `execute()` but returns a stream of chunks as they are generated.
    /// This is useful for real-time user interfaces or long-running operations.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin entity to execute
    /// * `input` - The user input to process
    ///
    /// # Returns
    ///
    /// Returns a receiver that yields `PaladinStreamChunk` results.
    ///
    /// # Errors
    ///
    /// Same errors as `execute()`, delivered through the stream.
    async fn execute_stream(
        &self,
        paladin: &Paladin,
        input: &str,
    ) -> Result<PaladinStream, PaladinError>;

    /// Validate a Paladin's configuration
    ///
    /// Checks that the Paladin is properly configured before execution.
    /// This should be called before `execute()` or `execute_stream()`.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin entity to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if valid, or `PaladinError::ConfigurationError` with details.
    ///
    /// # Errors
    ///
    /// * `PaladinError::ConfigurationError` - Invalid configuration with specific details
    fn validate(&self, paladin: &Paladin) -> Result<(), PaladinError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paladin_result_creation() {
        let result = PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 500,
            loop_count: 2,
            stop_reason: StopReason::Completed,
            plan: None,
            handoff_history: Vec::new(),
        };

        assert_eq!(result.output, "Test output");
        assert_eq!(result.token_count, 100);
        assert_eq!(result.loop_count, 2);
        assert!(!result.has_plan());
        assert!(!result.has_handoffs());
    }

    #[test]
    fn test_stop_reason_is_successful() {
        assert!(StopReason::Completed.is_successful());
        assert!(StopReason::StopWord("DONE".to_string()).is_successful());
        assert!(!StopReason::MaxLoops.is_successful());
        assert!(!StopReason::Timeout.is_successful());
    }

    #[test]
    fn test_stop_reason_is_limit() {
        assert!(StopReason::MaxLoops.is_limit());
        assert!(StopReason::Timeout.is_limit());
        assert!(!StopReason::Completed.is_limit());
        assert!(!StopReason::StopWord("DONE".to_string()).is_limit());
    }

    #[test]
    fn test_stop_reason_equality() {
        assert_eq!(StopReason::Completed, StopReason::Completed);
        assert_eq!(StopReason::MaxLoops, StopReason::MaxLoops);
        assert_eq!(
            StopReason::StopWord("STOP".to_string()),
            StopReason::StopWord("STOP".to_string())
        );
        assert_ne!(StopReason::Completed, StopReason::MaxLoops);
    }

    #[test]
    fn test_paladin_stream_chunk() {
        let chunk = PaladinStreamChunk {
            text: "Hello ".to_string(),
            is_final: false,
            metadata: Some(ChunkMetadata {
                tokens: Some(2),
                loop_count: Some(1),
            }),
        };

        assert_eq!(chunk.text, "Hello ");
        assert!(!chunk.is_final);
        assert!(chunk.metadata.is_some());
    }

    #[test]
    fn test_chunk_metadata() {
        let metadata = ChunkMetadata {
            tokens: Some(10),
            loop_count: Some(3),
        };

        assert_eq!(metadata.tokens, Some(10));
        assert_eq!(metadata.loop_count, Some(3));
    }

    #[test]
    fn test_paladin_result_serialization() {
        let result = PaladinResult {
            output: "Test".to_string(),
            token_count: 50,
            execution_time_ms: 250,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            plan: None,
            handoff_history: Vec::new(),
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let deserialized: PaladinResult =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(result.output, deserialized.output);
        assert_eq!(result.token_count, deserialized.token_count);
        assert_eq!(result.stop_reason, deserialized.stop_reason);
    }

    #[test]
    fn test_paladin_result_default_values() {
        let result = PaladinResult::default();

        assert_eq!(result.output, "");
        assert_eq!(result.token_count, 0);
        assert_eq!(result.execution_time_ms, 0);
        assert_eq!(result.loop_count, 0);
        assert_eq!(result.stop_reason, StopReason::Completed);
        assert!(result.plan.is_none());
        assert!(result.handoff_history.is_empty());
        assert!(!result.has_plan());
        assert!(!result.has_handoffs());
    }

    #[test]
    fn test_paladin_result_with_plan_metadata() {
        use crate::core::platform::container::planning::{Subtask, TaskPlan};

        let mut plan = TaskPlan::new("Test task".to_string(), 5);
        plan.add_subtask(Subtask::new(
            "st-1".to_string(),
            "First subtask".to_string(),
            "Expected output".to_string(),
        ))
        .expect("Failed to add subtask");

        let result = PaladinResult {
            output: "Task completed".to_string(),
            token_count: 200,
            execution_time_ms: 1000,
            loop_count: 3,
            stop_reason: StopReason::Completed,
            plan: Some(plan.clone()),
            handoff_history: Vec::new(),
        };

        assert!(result.has_plan());
        assert_eq!(result.plan.as_ref().unwrap().original_task, "Test task");
        assert_eq!(result.plan.as_ref().unwrap().subtask_count(), 1);
        assert!(!result.has_handoffs());
    }

    #[test]
    fn test_paladin_result_with_handoff_history() {
        use crate::core::platform::container::handoff::HandoffRecord;

        let mut record1 = HandoffRecord::new(
            "Coordinator".to_string(),
            "RustExpert".to_string(),
            "Debug Rust code".to_string(),
            1,
        );
        record1.set_result("Code debugged successfully".to_string());

        let record2 = HandoffRecord::new(
            "RustExpert".to_string(),
            "TestExpert".to_string(),
            "Write unit tests".to_string(),
            2,
        );

        let result = PaladinResult {
            output: "All tasks completed".to_string(),
            token_count: 500,
            execution_time_ms: 3000,
            loop_count: 5,
            stop_reason: StopReason::Completed,
            plan: None,
            handoff_history: vec![record1, record2],
        };

        assert!(!result.has_plan());
        assert!(result.has_handoffs());
        assert_eq!(result.handoff_count(), 2);
        assert_eq!(result.handoff_history[0].from_agent, "Coordinator");
        assert_eq!(result.handoff_history[0].to_agent, "RustExpert");
        assert_eq!(
            result.handoff_history[0].result.as_ref().unwrap(),
            "Code debugged successfully"
        );
        assert_eq!(result.handoff_history[1].depth, 2);
    }

    #[test]
    fn test_paladin_result_serialization_with_new_fields() {
        use crate::core::platform::container::handoff::HandoffRecord;
        use crate::core::platform::container::planning::{Subtask, TaskPlan};

        let mut plan = TaskPlan::new("Complex task".to_string(), 3);
        plan.add_subtask(Subtask::new(
            "st-1".to_string(),
            "Step 1".to_string(),
            "Output 1".to_string(),
        ))
        .expect("Failed to add subtask");

        let record = HandoffRecord::new(
            "Agent1".to_string(),
            "Agent2".to_string(),
            "Subtask".to_string(),
            1,
        );

        let result = PaladinResult {
            output: "Final output".to_string(),
            token_count: 300,
            execution_time_ms: 2000,
            loop_count: 4,
            stop_reason: StopReason::Completed,
            plan: Some(plan),
            handoff_history: vec![record],
        };

        // Serialize
        let json = serde_json::to_string(&result).expect("Failed to serialize");
        assert!(json.contains("\"plan\""));
        assert!(json.contains("\"handoff_history\""));
        assert!(json.contains("Complex task"));
        assert!(json.contains("Agent1"));

        // Deserialize
        let deserialized: PaladinResult =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.output, "Final output");
        assert!(deserialized.has_plan());
        assert!(deserialized.has_handoffs());
        assert_eq!(deserialized.handoff_count(), 1);
    }

    #[test]
    fn test_paladin_result_deserialization_backward_compatibility() {
        // Old JSON format without plan and handoff_history fields
        let old_json = r#"{
            "output": "Old result",
            "token_count": 150,
            "execution_time_ms": 800,
            "loop_count": 2,
            "stop_reason": "Completed"
        }"#;

        // Should deserialize successfully with default values for new fields
        let result: PaladinResult =
            serde_json::from_str(old_json).expect("Failed to deserialize old format");

        assert_eq!(result.output, "Old result");
        assert_eq!(result.token_count, 150);
        assert_eq!(result.execution_time_ms, 800);
        assert_eq!(result.loop_count, 2);
        assert_eq!(result.stop_reason, StopReason::Completed);

        // New fields should have default values
        assert!(result.plan.is_none());
        assert!(result.handoff_history.is_empty());
        assert!(!result.has_plan());
        assert!(!result.has_handoffs());
    }

    #[test]
    fn test_paladin_result_new_constructor() {
        let result = PaladinResult::new(
            "Constructor test".to_string(),
            250,
            1500,
            3,
            StopReason::MaxLoops,
        );

        assert_eq!(result.output, "Constructor test");
        assert_eq!(result.token_count, 250);
        assert_eq!(result.execution_time_ms, 1500);
        assert_eq!(result.loop_count, 3);
        assert_eq!(result.stop_reason, StopReason::MaxLoops);
        assert!(result.plan.is_none());
        assert!(result.handoff_history.is_empty());
    }
}
