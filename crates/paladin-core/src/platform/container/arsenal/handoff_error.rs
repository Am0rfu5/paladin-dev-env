//! Handoff Error Types
//!
//! This module defines error types for agent handoff and delegation operations.
//! All errors follow the fail-fast principle with descriptive messages for debugging.
//!
//! The `application` layer re-exports this type from
//! `application::errors::handoff_error`.

use thiserror::Error;

/// Errors that can occur during agent handoff operations
///
/// These errors cover handoff decisions, delegation execution, circular handoff
/// prevention, and agent selection during handoff operations.
///
/// # Examples
///
/// ```
/// use paladin_core::platform::container::arsenal::handoff_error::HandoffError;
///
/// let error = HandoffError::InvalidAgent {
///     agent_name: "UnknownAgent".to_string(),
/// };
/// assert!(error.to_string().contains("Invalid agent"));
/// ```
#[derive(Debug, Error)]
pub enum HandoffError {
    /// Attempted to handoff to an agent that doesn't exist
    #[error("Invalid agent: {agent_name} not found in registered specialists")]
    InvalidAgent {
        /// Name of the agent that wasn't found
        agent_name: String,
    },

    /// Circular handoff detected (agent already in chain)
    #[error("Circular handoff detected: {agent_name} is already in the chain {chain}")]
    CircularHandoff {
        /// Name of the agent causing the circular handoff
        agent_name: String,
        /// Current handoff chain
        chain: String,
    },

    /// Maximum handoff depth exceeded
    #[error("Maximum handoff depth exceeded: current {current}, max allowed {max}")]
    MaxDepthExceeded {
        /// Current handoff depth
        current: u32,
        /// Maximum allowed depth
        max: u32,
    },

    /// Handoff execution failed
    #[error("Handoff execution failed: {from_agent} -> {to_agent}: {reason}")]
    ExecutionFailed {
        /// Agent initiating the handoff
        from_agent: String,
        /// Target agent
        to_agent: String,
        /// Reason for failure
        reason: String,
    },

    /// Handoff decision analysis failed
    #[error("Handoff decision failed: {0}")]
    DecisionFailed(String),

    /// No suitable agent found for the task
    #[error("No suitable agent found for task: {task}")]
    NoSuitableAgent {
        /// Task that couldn't be matched to an agent
        task: String,
    },

    /// Agent capabilities don't match task requirements
    #[error("Agent capabilities mismatch: {agent_name} cannot handle task type {task_type}")]
    CapabilitiesMismatch {
        /// Name of the agent
        agent_name: String,
        /// Required task type
        task_type: String,
    },

    /// Context transfer failed
    #[error("Context transfer failed: {0}")]
    ContextTransferFailed(String),

    /// Configuration error for handoffs
    #[error("Handoff configuration error: {0}")]
    ConfigurationError(String),

    /// LLM error during handoff decision
    #[error("LLM error during handoff: {0}")]
    LlmError(String),

    /// Timeout during handoff operation
    #[error("Handoff timeout after {0} seconds")]
    Timeout(u64),

    /// Handoff chain validation failed
    #[error("Invalid handoff chain: {0}")]
    InvalidChain(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handoff_error_invalid_agent() {
        let error = HandoffError::InvalidAgent {
            agent_name: "CodeExpert".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Invalid agent"));
        assert!(msg.contains("CodeExpert"));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn test_handoff_error_circular_handoff() {
        let error = HandoffError::CircularHandoff {
            agent_name: "Agent1".to_string(),
            chain: "Agent1 -> Agent2 -> Agent3".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Circular handoff detected"));
        assert!(msg.contains("Agent1"));
        assert!(msg.contains("Agent1 -> Agent2 -> Agent3"));
    }

    #[test]
    fn test_handoff_error_max_depth_exceeded() {
        let error = HandoffError::MaxDepthExceeded { current: 6, max: 5 };
        let msg = error.to_string();
        assert!(msg.contains("Maximum handoff depth exceeded"));
        assert!(msg.contains("current 6"));
        assert!(msg.contains("max allowed 5"));
    }

    #[test]
    fn test_handoff_error_execution_failed() {
        let error = HandoffError::ExecutionFailed {
            from_agent: "Coordinator".to_string(),
            to_agent: "Specialist".to_string(),
            reason: "Target agent offline".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Handoff execution failed"));
        assert!(msg.contains("Coordinator"));
        assert!(msg.contains("Specialist"));
        assert!(msg.contains("offline"));
    }

    #[test]
    fn test_handoff_error_decision_failed() {
        let error = HandoffError::DecisionFailed("Insufficient context".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Handoff decision failed"));
        assert!(msg.contains("Insufficient context"));
    }

    #[test]
    fn test_handoff_error_no_suitable_agent() {
        let error = HandoffError::NoSuitableAgent {
            task: "Quantum physics calculation".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("No suitable agent"));
        assert!(msg.contains("Quantum physics"));
    }

    #[test]
    fn test_handoff_error_capabilities_mismatch() {
        let error = HandoffError::CapabilitiesMismatch {
            agent_name: "TextAnalyst".to_string(),
            task_type: "image_processing".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("capabilities mismatch"));
        assert!(msg.contains("TextAnalyst"));
        assert!(msg.contains("image_processing"));
    }

    #[test]
    fn test_handoff_error_context_transfer_failed() {
        let error = HandoffError::ContextTransferFailed("Serialization error".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Context transfer failed"));
        assert!(msg.contains("Serialization"));
    }

    #[test]
    fn test_handoff_error_configuration() {
        let error = HandoffError::ConfigurationError("No agents registered".to_string());
        let msg = error.to_string();
        assert!(msg.contains("configuration error"));
        assert!(msg.contains("No agents registered"));
    }

    #[test]
    fn test_handoff_error_llm_error() {
        let error = HandoffError::LlmError("Model not available".to_string());
        let msg = error.to_string();
        assert!(msg.contains("LLM error"));
        assert!(msg.contains("Model not available"));
    }

    #[test]
    fn test_handoff_error_timeout() {
        let error = HandoffError::Timeout(60);
        let msg = error.to_string();
        assert!(msg.contains("Handoff timeout"));
        assert!(msg.contains("60"));
    }

    #[test]
    fn test_handoff_error_invalid_chain() {
        let error = HandoffError::InvalidChain("Empty chain not allowed".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Invalid handoff chain"));
        assert!(msg.contains("Empty chain"));
    }
}
