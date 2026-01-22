//! Paladin Port Abstraction
//!
//! This module defines the port (interface) for Paladin execution following
//! the hexagonal architecture pattern. Implementations of this port handle
//! the actual execution logic while the domain remains independent.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::application::use_cases::paladin::error::PaladinError;
use crate::core::platform::container::paladin::Paladin;

/// Result of a Paladin execution
///
/// Contains the output, metadata about execution, and the reason for completion.
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
        };

        assert_eq!(result.output, "Test output");
        assert_eq!(result.token_count, 100);
        assert_eq!(result.loop_count, 2);
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
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let deserialized: PaladinResult =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(result.output, deserialized.output);
        assert_eq!(result.token_count, deserialized.token_count);
        assert_eq!(result.stop_reason, deserialized.stop_reason);
    }
}
