//! Planning Error Types
//!
//! This module defines error types for autonomous planning operations.
//! All errors follow the fail-fast principle with descriptive messages for debugging.

use thiserror::Error;

/// Errors that can occur during autonomous planning operations
///
/// These errors cover plan creation, validation, subtask execution,
/// and result synthesis during autonomous planning mode.
///
/// # Examples
///
/// ```
/// use paladin::application::errors::planning_error::PlanningError;
///
/// let error = PlanningError::MaxSubtasksExceeded { max: 10, attempted: 15 };
/// assert!(error.to_string().contains("Maximum subtasks"));
/// ```
#[derive(Debug, Error)]
pub enum PlanningError {
    /// Plan contains invalid or inconsistent structure
    #[error("Invalid plan: {0}")]
    InvalidPlan(String),

    /// Attempted to create more subtasks than allowed
    #[error("Maximum subtasks exceeded: attempted {attempted}, max allowed {max}")]
    MaxSubtasksExceeded {
        /// Maximum number of subtasks allowed
        max: u32,
        /// Number of subtasks attempted
        attempted: u32,
    },

    /// Subtask execution failed
    #[error("Subtask execution failed: {subtask_id} - {reason}")]
    ExecutionFailed {
        /// ID of the failed subtask
        subtask_id: String,
        /// Reason for failure
        reason: String,
    },

    /// LLM failed to generate a valid plan
    #[error("Plan generation failed: {0}")]
    GenerationFailed(String),

    /// Circular dependency detected in task plan
    #[error("Circular dependency detected in plan: {0}")]
    CircularDependency(String),

    /// Dependency references a non-existent subtask
    #[error("Invalid dependency: subtask {dependency} not found in plan")]
    InvalidDependency {
        /// ID of the invalid dependency
        dependency: String,
    },

    /// Plan synthesis failed
    #[error("Failed to synthesize subtask results: {0}")]
    SynthesisFailed(String),

    /// LLM error during planning operations
    #[error("LLM error: {0}")]
    LlmError(String),

    /// Empty plan generated (no subtasks)
    #[error("Empty plan generated: must have at least one subtask")]
    EmptyPlan,

    /// Timeout during plan execution
    #[error("Planning timeout after {0} seconds")]
    Timeout(u64),

    /// Configuration error for planning mode
    #[error("Planning configuration error: {0}")]
    ConfigurationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning_error_invalid_plan() {
        let error = PlanningError::InvalidPlan("Missing task description".to_string());
        assert!(error.to_string().contains("Invalid plan"));
        assert!(error.to_string().contains("Missing task description"));
    }

    #[test]
    fn test_planning_error_max_subtasks_exceeded() {
        let error = PlanningError::MaxSubtasksExceeded {
            max: 10,
            attempted: 15,
        };
        let msg = error.to_string();
        assert!(msg.contains("Maximum subtasks exceeded"));
        assert!(msg.contains("attempted 15"));
        assert!(msg.contains("max allowed 10"));
    }

    #[test]
    fn test_planning_error_execution_failed() {
        let error = PlanningError::ExecutionFailed {
            subtask_id: "st-3".to_string(),
            reason: "LLM timeout".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Subtask execution failed"));
        assert!(msg.contains("st-3"));
        assert!(msg.contains("LLM timeout"));
    }

    #[test]
    fn test_planning_error_generation_failed() {
        let error = PlanningError::GenerationFailed("Invalid JSON response".to_string());
        assert!(error.to_string().contains("Plan generation failed"));
    }

    #[test]
    fn test_planning_error_circular_dependency() {
        let error = PlanningError::CircularDependency("st-1 -> st-2 -> st-1".to_string());
        assert!(error.to_string().contains("Circular dependency"));
    }

    #[test]
    fn test_planning_error_invalid_dependency() {
        let error = PlanningError::InvalidDependency {
            dependency: "st-999".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Invalid dependency"));
        assert!(msg.contains("st-999"));
    }

    #[test]
    fn test_planning_error_synthesis_failed() {
        let error = PlanningError::SynthesisFailed("Conflicting results".to_string());
        assert!(error.to_string().contains("Failed to synthesize"));
    }

    #[test]
    fn test_planning_error_llm_error() {
        let error = PlanningError::LlmError("API key invalid".to_string());
        assert!(error.to_string().contains("LLM error"));
    }

    #[test]
    fn test_planning_error_empty_plan() {
        let error = PlanningError::EmptyPlan;
        assert!(error.to_string().contains("Empty plan generated"));
    }

    #[test]
    fn test_planning_error_timeout() {
        let error = PlanningError::Timeout(300);
        let msg = error.to_string();
        assert!(msg.contains("Planning timeout"));
        assert!(msg.contains("300"));
    }

    #[test]
    fn test_planning_error_configuration() {
        let error = PlanningError::ConfigurationError("max_subtasks must be > 0".to_string());
        assert!(error.to_string().contains("Planning configuration error"));
    }
}
