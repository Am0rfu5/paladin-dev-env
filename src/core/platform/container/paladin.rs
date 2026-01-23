//! Paladin Domain Entity
//!
//! This module defines the core domain entity representing an autonomous AI agent
//! capable of reasoning and executing actions. The Paladin follows the `Node<T>` pattern
//! for consistency with other domain entities in the system.
//!
//! # Example
//!
//! ```
//! use paladin::core::platform::container::paladin::{PaladinData, PaladinStatus};
//! use paladin::core::base::entity::node::Node;
//!
//! let data = PaladinData {
//!     system_prompt: "You are a helpful assistant".to_string(),
//!     name: "MyPaladin".to_string(),
//!     user_name: "User".to_string(),
//!     model: "gpt-4".to_string(),
//!     temperature: 0.7,
//!     max_loops: 3,
//!     stop_words: vec!["STOP".to_string()],
//!     status: PaladinStatus::Idle,
//! };
//!
//! let paladin = Node::new(data, Some("MyPaladin".to_string()));
//! ```

use crate::core::base::entity::node::Node;
use serde::{Deserialize, Serialize};

/// Status of a Paladin during its lifecycle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaladinStatus {
    /// Paladin is idle and ready to accept tasks
    Idle,
    /// Paladin is actively reasoning about the problem
    Reasoning,
    /// Paladin is executing an action or tool
    Executing,
    /// Paladin has completed its task successfully
    Completed,
    /// Paladin failed with an error message
    Failed(String),
}

/// Core data payload for a Paladin entity
///
/// Contains all configuration and state information for an autonomous AI agent.
/// This struct is wrapped in a `Node<T>` to provide UUID, timestamps, and versioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    /// System prompt defining the Paladin's behavior and personality
    pub system_prompt: String,

    /// Display name for the Paladin
    pub name: String,

    /// Name to use for the user in conversations
    pub user_name: String,

    /// LLM model identifier (e.g., "gpt-4", "claude-3")
    pub model: String,

    /// Response randomness (0.0 = deterministic, 1.0 = maximum randomness)
    pub temperature: f32,

    /// Maximum number of reasoning iterations before stopping
    pub max_loops: u32,

    /// Tokens that signal the Paladin should stop processing
    pub stop_words: Vec<String>,

    /// Current execution status
    pub status: PaladinStatus,
}

/// Type alias for a Paladin entity following the `Node<T>` pattern
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
/// use paladin::core::base::entity::node::Node;
///
/// let data = PaladinData {
///     system_prompt: "You are a code reviewer".to_string(),
///     name: "CodeReviewer".to_string(),
///     user_name: "Developer".to_string(),
///     model: "gpt-4".to_string(),
///     temperature: 0.3,
///     max_loops: 5,
///     stop_words: vec!["DONE".to_string()],
///     status: PaladinStatus::Idle,
/// };
///
/// let paladin: Paladin = Node::new(data, Some("CodeReviewer".to_string()));
/// assert_eq!(paladin.node.name, "CodeReviewer");
/// ```
pub type Paladin = Node<PaladinData>;

impl Default for PaladinData {
    fn default() -> Self {
        Self {
            system_prompt: String::new(),
            name: "Paladin".to_string(),
            user_name: "User".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            stop_words: Vec::new(),
            status: PaladinStatus::Idle,
        }
    }
}

impl PaladinStatus {
    /// Check if the status represents a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, PaladinStatus::Completed | PaladinStatus::Failed(_))
    }

    /// Check if the status represents an active state
    pub fn is_active(&self) -> bool {
        matches!(self, PaladinStatus::Reasoning | PaladinStatus::Executing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paladin_status_is_terminal() {
        assert!(PaladinStatus::Completed.is_terminal());
        assert!(PaladinStatus::Failed("error".to_string()).is_terminal());
        assert!(!PaladinStatus::Idle.is_terminal());
        assert!(!PaladinStatus::Reasoning.is_terminal());
        assert!(!PaladinStatus::Executing.is_terminal());
    }

    #[test]
    fn test_paladin_status_is_active() {
        assert!(PaladinStatus::Reasoning.is_active());
        assert!(PaladinStatus::Executing.is_active());
        assert!(!PaladinStatus::Idle.is_active());
        assert!(!PaladinStatus::Completed.is_active());
        assert!(!PaladinStatus::Failed("error".to_string()).is_active());
    }

    #[test]
    fn test_paladin_data_default() {
        let data = PaladinData::default();
        assert_eq!(data.name, "Paladin");
        assert_eq!(data.user_name, "User");
        assert_eq!(data.model, "gpt-4");
        assert_eq!(data.temperature, 0.7);
        assert_eq!(data.max_loops, 3);
        assert_eq!(data.status, PaladinStatus::Idle);
    }
}
