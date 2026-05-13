//! Handoff Domain Types
//!
//! This module defines domain types for agent handoff and delegation.
//! These types support the autonomous handoff feature where a Paladin
//! can delegate tasks to specialist agents.

use serde::{Deserialize, Serialize};

/// Strategy for determining when to handoff tasks to specialist agents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HandoffStrategy {
    /// Automatically decide based on task analysis and confidence
    #[default]
    Automatic,

    /// Only handoff when explicitly requested via tool call
    Explicit,

    /// Handoff when confidence is below the threshold
    Threshold {
        /// Confidence level below which handoff occurs (0.0-1.0)
        confidence: u8, // Stored as 0-100 for serde compatibility
    },
}

impl HandoffStrategy {
    /// Creates a threshold strategy with the given confidence level
    ///
    /// # Arguments
    ///
    /// * `confidence` - Confidence threshold (0.0-1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::handoff::HandoffStrategy;
    ///
    /// let strategy = HandoffStrategy::threshold(0.7);
    /// ```
    pub fn threshold(confidence: f32) -> Self {
        let confidence_u8 = (confidence * 100.0).clamp(0.0, 100.0) as u8;
        HandoffStrategy::Threshold {
            confidence: confidence_u8,
        }
    }

    /// Gets the confidence threshold as f32 (0.0-1.0)
    pub fn get_threshold(&self) -> Option<f32> {
        match self {
            HandoffStrategy::Threshold { confidence } => Some(*confidence as f32 / 100.0),
            _ => None,
        }
    }
}

/// Context information for a handoff between agents
///
/// Contains all the information needed to transfer a task from one
/// agent to another, including conversation history and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffContext {
    /// The task being delegated
    pub task: String,

    /// Chain of agent names that have handled this task
    ///
    /// Used to prevent circular handoffs where an agent delegates
    /// back to an agent earlier in the chain.
    pub chain: Vec<String>,

    /// Conversation history relevant to this task
    pub history: Vec<String>,

    /// Additional metadata (key-value pairs)
    pub metadata: std::collections::HashMap<String, String>,

    /// Current depth of the handoff chain
    pub depth: u32,
}

impl HandoffContext {
    /// Creates a new handoff context
    ///
    /// # Arguments
    ///
    /// * `task` - The task description
    /// * `origin_agent` - Name of the agent initiating the handoff
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::handoff::HandoffContext;
    ///
    /// let context = HandoffContext::new(
    ///     "Debug this code".to_string(),
    ///     "Coordinator".to_string()
    /// );
    /// assert_eq!(context.depth, 1);
    /// ```
    pub fn new(task: String, origin_agent: String) -> Self {
        Self {
            task,
            chain: vec![origin_agent],
            history: Vec::new(),
            metadata: std::collections::HashMap::new(),
            depth: 1,
        }
    }

    /// Adds an agent to the handoff chain
    ///
    /// Returns an error if the agent is already in the chain (circular handoff)
    pub fn add_to_chain(&mut self, agent_name: String) -> Result<(), String> {
        if self.chain.contains(&agent_name) {
            return Err(format!(
                "Circular handoff detected: {} is already in the chain",
                agent_name
            ));
        }
        self.chain.push(agent_name);
        self.depth += 1;
        Ok(())
    }

    /// Checks if an agent is in the handoff chain
    pub fn is_in_chain(&self, agent_name: &str) -> bool {
        self.chain.iter().any(|name| name == agent_name)
    }

    /// Adds a message to the history
    pub fn add_history(&mut self, message: String) {
        self.history.push(message);
    }

    /// Adds metadata key-value pair
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Decision about whether and how to handoff a task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffDecision {
    /// Whether to proceed with the handoff
    pub should_handoff: bool,

    /// Name of the target agent (if handoff should occur)
    pub target_agent: Option<String>,

    /// Reasoning for the decision
    pub reasoning: String,

    /// Confidence in this decision (0.0-1.0)
    pub confidence: f32,
}

impl HandoffDecision {
    /// Creates a decision to proceed with handoff
    pub fn handoff(target_agent: String, reasoning: String, confidence: f32) -> Self {
        Self {
            should_handoff: true,
            target_agent: Some(target_agent),
            reasoning,
            confidence,
        }
    }

    /// Creates a decision to NOT handoff (handle locally)
    pub fn no_handoff(reasoning: String, confidence: f32) -> Self {
        Self {
            should_handoff: false,
            target_agent: None,
            reasoning,
            confidence,
        }
    }
}

/// Record of a completed handoff for tracking and transparency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffRecord {
    /// Agent that initiated the handoff
    pub from_agent: String,

    /// Agent that received the handoff
    pub to_agent: String,

    /// Task that was handed off
    pub task: String,

    /// Result from the target agent
    pub result: Option<String>,

    /// Depth in the handoff chain
    pub depth: u32,
}

impl HandoffRecord {
    /// Creates a new handoff record
    pub fn new(from_agent: String, to_agent: String, task: String, depth: u32) -> Self {
        Self {
            from_agent,
            to_agent,
            task,
            result: None,
            depth,
        }
    }

    /// Sets the result from the target agent
    pub fn set_result(&mut self, result: String) {
        self.result = Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handoff_strategy_automatic() {
        let strategy = HandoffStrategy::Automatic;
        assert_eq!(strategy, HandoffStrategy::Automatic);
    }

    #[test]
    fn test_handoff_strategy_explicit() {
        let strategy = HandoffStrategy::Explicit;
        assert_eq!(strategy, HandoffStrategy::Explicit);
    }

    #[test]
    fn test_handoff_strategy_threshold() {
        let strategy = HandoffStrategy::threshold(0.7);
        assert_eq!(strategy.get_threshold(), Some(0.7));
    }

    #[test]
    fn test_handoff_strategy_threshold_clamping() {
        let strategy1 = HandoffStrategy::threshold(1.5);
        assert_eq!(strategy1.get_threshold(), Some(1.0));

        let strategy2 = HandoffStrategy::threshold(-0.5);
        assert_eq!(strategy2.get_threshold(), Some(0.0));
    }

    #[test]
    fn test_handoff_strategy_default() {
        let strategy = HandoffStrategy::default();
        assert_eq!(strategy, HandoffStrategy::Automatic);
    }

    #[test]
    fn test_handoff_context_new() {
        let context = HandoffContext::new("Test task".to_string(), "Agent1".to_string());

        assert_eq!(context.task, "Test task");
        assert_eq!(context.chain, vec!["Agent1"]);
        assert_eq!(context.depth, 1);
        assert!(context.history.is_empty());
        assert!(context.metadata.is_empty());
    }

    #[test]
    fn test_handoff_context_add_to_chain() {
        let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

        assert!(context.add_to_chain("Agent2".to_string()).is_ok());
        assert_eq!(context.chain, vec!["Agent1", "Agent2"]);
        assert_eq!(context.depth, 2);
    }

    #[test]
    fn test_handoff_context_circular_detection() {
        let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

        context.add_to_chain("Agent2".to_string()).unwrap();

        let result = context.add_to_chain("Agent1".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular handoff"));
    }

    #[test]
    fn test_handoff_context_is_in_chain() {
        let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

        assert!(context.is_in_chain("Agent1"));
        assert!(!context.is_in_chain("Agent2"));

        context.add_to_chain("Agent2".to_string()).unwrap();
        assert!(context.is_in_chain("Agent2"));
    }

    #[test]
    fn test_handoff_context_history() {
        let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

        context.add_history("First message".to_string());
        context.add_history("Second message".to_string());

        assert_eq!(context.history.len(), 2);
        assert_eq!(context.history[0], "First message");
    }

    #[test]
    fn test_handoff_context_metadata() {
        let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

        context.add_metadata("priority".to_string(), "high".to_string());
        context.add_metadata("category".to_string(), "code".to_string());

        assert_eq!(context.metadata.len(), 2);
        assert_eq!(context.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_handoff_decision_handoff() {
        let decision = HandoffDecision::handoff(
            "SpecialistAgent".to_string(),
            "Task requires specialist knowledge".to_string(),
            0.85,
        );

        assert!(decision.should_handoff);
        assert_eq!(decision.target_agent, Some("SpecialistAgent".to_string()));
        assert_eq!(decision.confidence, 0.85);
    }

    #[test]
    fn test_handoff_decision_no_handoff() {
        let decision = HandoffDecision::no_handoff("Can handle locally".to_string(), 0.90);

        assert!(!decision.should_handoff);
        assert_eq!(decision.target_agent, None);
        assert_eq!(decision.confidence, 0.90);
    }

    #[test]
    fn test_handoff_record_new() {
        let record = HandoffRecord::new(
            "Agent1".to_string(),
            "Agent2".to_string(),
            "Task".to_string(),
            2,
        );

        assert_eq!(record.from_agent, "Agent1");
        assert_eq!(record.to_agent, "Agent2");
        assert_eq!(record.task, "Task");
        assert_eq!(record.depth, 2);
        assert!(record.result.is_none());
    }

    #[test]
    fn test_handoff_record_set_result() {
        let mut record = HandoffRecord::new(
            "Agent1".to_string(),
            "Agent2".to_string(),
            "Task".to_string(),
            1,
        );

        record.set_result("Task completed successfully".to_string());
        assert_eq!(
            record.result,
            Some("Task completed successfully".to_string())
        );
    }

    #[test]
    fn test_handoff_types_serialization() {
        let context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
        let json = serde_json::to_string(&context).unwrap();
        let deserialized: HandoffContext = serde_json::from_str(&json).unwrap();
        assert_eq!(context, deserialized);

        let decision = HandoffDecision::handoff("Agent2".to_string(), "Reason".to_string(), 0.8);
        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: HandoffDecision = serde_json::from_str(&json).unwrap();
        assert_eq!(decision, deserialized);
    }
}
