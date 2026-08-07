//! Token Usage Tracking
//!
//! This module defines [`TokenUsage`], a pure domain value type for tracking
//! LLM token consumption. The `application` layer re-exports it from here.

use serde::{Deserialize, Serialize};

/// Token usage statistics for an LLM request
///
/// Tracks the number of tokens consumed by prompt and completion so that
/// callers can estimate cost and enforce budget limits.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the input prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion
    pub completion_tokens: u32,
    /// Total tokens (prompt + completion)
    pub total_tokens: u32,
}

impl TokenUsage {
    /// Create a new `TokenUsage` with specified counts
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }

    /// Create a `TokenUsage` from a total count only (no prompt/completion breakdown)
    pub fn from_total(total_tokens: u32) -> Self {
        Self {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_computes_total_from_prompt_and_completion() {
        let usage = TokenUsage::new(10, 5);
        assert_eq!(usage.prompt_tokens, 10);
        assert_eq!(usage.completion_tokens, 5);
        assert_eq!(usage.total_tokens, 15);
    }

    #[test]
    fn from_total_leaves_prompt_and_completion_at_zero() {
        let usage = TokenUsage::from_total(263);
        assert_eq!(usage.prompt_tokens, 0);
        assert_eq!(usage.completion_tokens, 0);
        assert_eq!(usage.total_tokens, 263);
    }

    #[test]
    fn default_is_all_zero() {
        let usage = TokenUsage::default();
        assert_eq!(usage.prompt_tokens, 0);
        assert_eq!(usage.completion_tokens, 0);
        assert_eq!(usage.total_tokens, 0);
    }

    #[test]
    fn partial_eq_compares_all_three_fields_not_only_total() {
        assert_eq!(TokenUsage::new(1, 2), TokenUsage::new(1, 2));
        assert_ne!(TokenUsage::new(1, 2), TokenUsage::from_total(3));
    }
}
