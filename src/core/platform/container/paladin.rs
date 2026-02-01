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
//!     vision_enabled: false,
//! };
//!
//! let paladin = Node::new(data, Some("MyPaladin".to_string()));
//! ```

use crate::core::base::entity::node::Node;
use serde::{Deserialize, Deserializer, Serialize};

/// Maximum iteration control for Paladin execution
///
/// Controls how many reasoning loops a Paladin can perform before stopping.
/// This can be either a fixed number or an automatic planning mode that
/// decomposes tasks into subtasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaxLoops {
    /// Fixed number of reasoning iterations
    Fixed(u32),
    /// Automatic planning mode with LLM-driven task decomposition
    ///
    /// The Paladin will use an LLM to analyze the task and create a plan
    /// with subtasks, then execute them sequentially up to `max_subtasks`.
    Auto {
        /// Maximum number of subtasks the planner can create
        max_subtasks: u32,
    },
}

// Custom deserializer to handle both old integer format and new enum format
impl<'de> Deserialize<'de> for MaxLoops {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MaxLoopsHelper {
            Integer(u32),
            Enum(MaxLoopsEnum),
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        enum MaxLoopsEnum {
            Fixed(u32),
            Auto { max_subtasks: u32 },
        }

        match MaxLoopsHelper::deserialize(deserializer)? {
            MaxLoopsHelper::Integer(n) => Ok(MaxLoops::Fixed(n)),
            MaxLoopsHelper::Enum(MaxLoopsEnum::Fixed(n)) => Ok(MaxLoops::Fixed(n)),
            MaxLoopsHelper::Enum(MaxLoopsEnum::Auto { max_subtasks }) => {
                Ok(MaxLoops::Auto { max_subtasks })
            }
        }
    }
}

impl Default for MaxLoops {
    fn default() -> Self {
        MaxLoops::Fixed(3)
    }
}

impl MaxLoops {
    /// Gets the loop count for Fixed mode, or max_subtasks for Auto mode
    ///
    /// This provides backward compatibility with code that expects a u32.
    pub fn as_u32(&self) -> u32 {
        match self {
            MaxLoops::Fixed(count) => *count,
            MaxLoops::Auto { max_subtasks } => *max_subtasks,
        }
    }

    /// Returns true if this is Auto planning mode
    pub fn is_auto(&self) -> bool {
        matches!(self, MaxLoops::Auto { .. })
    }

    /// Returns true if this is Fixed loop mode
    pub fn is_fixed(&self) -> bool {
        matches!(self, MaxLoops::Fixed(_))
    }
}

impl std::fmt::Display for MaxLoops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxLoops::Fixed(count) => write!(f, "{}", count),
            MaxLoops::Auto { max_subtasks } => write!(f, "Auto({})", max_subtasks),
        }
    }
}

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

    /// Maximum iteration control (Fixed or Auto planning mode)
    pub max_loops: MaxLoops,

    /// Tokens that signal the Paladin should stop processing
    pub stop_words: Vec<String>,

    /// Current execution status
    pub status: PaladinStatus,

    /// Whether vision capabilities are enabled for this Paladin
    pub vision_enabled: bool,
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
///     vision_enabled: false,
/// };
///
/// let paladin: Paladin = Node::new(data, Some("CodeReviewer".to_string()));
/// assert_eq!(paladin.node.name, "CodeReviewer");
/// ```
pub type Paladin = Node<PaladinData>;

impl Default for PaladinData {
    /// Creates a PaladinData instance with sensible defaults.
    ///
    /// # Default Values
    /// - `system_prompt`: Empty string (must be set before use)
    /// - `name`: "Paladin"
    /// - `user_name`: "User"
    /// - `model`: "gpt-4"
    /// - `temperature`: 0.7
    /// - `max_loops`: MaxLoops::Fixed(3)
    /// - `stop_words`: Empty vector
    /// - `status`: Idle
    /// - `vision_enabled`: false
    fn default() -> Self {
        Self {
            system_prompt: String::new(),
            name: "Paladin".to_string(),
            user_name: "User".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::default(),
            stop_words: Vec::new(),
            status: PaladinStatus::Idle,
            vision_enabled: false,
        }
    }
}

impl PaladinStatus {
    /// Returns `true` if the status represents a terminal state.
    ///
    /// Terminal states are [`Completed`](PaladinStatus::Completed) and
    /// [`Failed`](PaladinStatus::Failed), indicating the Paladin's work is done.
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin::core::platform::container::paladin::PaladinStatus;
    ///
    /// assert!(PaladinStatus::Completed.is_terminal());
    /// assert!(PaladinStatus::Failed("error".to_string()).is_terminal());
    /// assert!(!PaladinStatus::Idle.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(self, PaladinStatus::Completed | PaladinStatus::Failed(_))
    }

    /// Returns `true` if the status represents an active state.
    ///
    /// Active states are [`Reasoning`](PaladinStatus::Reasoning) and
    /// [`Executing`](PaladinStatus::Executing), indicating the Paladin is currently working.
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin::core::platform::container::paladin::PaladinStatus;
    ///
    /// assert!(PaladinStatus::Reasoning.is_active());
    /// assert!(PaladinStatus::Executing.is_active());
    /// assert!(!PaladinStatus::Idle.is_active());
    /// ```
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
    fn test_max_loops_fixed() {
        let loops = MaxLoops::Fixed(5);
        assert_eq!(loops, MaxLoops::Fixed(5));
    }

    #[test]
    fn test_max_loops_auto() {
        let loops = MaxLoops::Auto { max_subtasks: 10 };
        if let MaxLoops::Auto { max_subtasks } = loops {
            assert_eq!(max_subtasks, 10);
        } else {
            panic!("Expected MaxLoops::Auto variant");
        }
    }

    #[test]
    fn test_max_loops_default() {
        let loops = MaxLoops::default();
        assert_eq!(loops, MaxLoops::Fixed(3));
    }

    #[test]
    fn test_paladin_data_default() {
        let data = PaladinData::default();
        assert_eq!(data.name, "Paladin");
        assert_eq!(data.user_name, "User");
        assert_eq!(data.model, "gpt-4");
        assert_eq!(data.temperature, 0.7);
        assert_eq!(data.max_loops, MaxLoops::Fixed(3));
        assert_eq!(data.status, PaladinStatus::Idle);
        assert!(!data.vision_enabled);
    }

    #[test]
    fn test_paladin_data_with_auto_planning() {
        let mut data = PaladinData::default();
        data.max_loops = MaxLoops::Auto { max_subtasks: 8 };

        if let MaxLoops::Auto { max_subtasks } = data.max_loops {
            assert_eq!(max_subtasks, 8);
        } else {
            panic!("Expected MaxLoops::Auto variant");
        }
    }

    #[test]
    fn test_vision_enabled_field() {
        let mut data = PaladinData::default();
        assert!(!data.vision_enabled);

        data.vision_enabled = true;
        assert!(data.vision_enabled);
    }
}
