//! Token Usage Tracking
//!
//! This module defines [`TokenUsage`], a pure domain value type for tracking
//! LLM token consumption. The `application` layer re-exports it from here.

use serde::{Deserialize, Serialize};

/// Token usage statistics for an LLM request
///
/// Tracks the number of tokens consumed by prompt and completion so that
/// callers can estimate cost and enforce budget limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the input prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion
    pub completion_tokens: u32,
    /// Total tokens (prompt + completion)
    pub total_tokens: u32,
}
