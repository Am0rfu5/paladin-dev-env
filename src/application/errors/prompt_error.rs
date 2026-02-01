//! Prompt Generation Error Types
//!
//! This module defines error types for autonomous prompt generation operations.
//! All errors follow the fail-fast principle with descriptive messages for debugging.

use thiserror::Error;

/// Errors that can occur during autonomous prompt generation operations
///
/// These errors cover prompt generation, caching, validation, and regeneration
/// during autonomous prompt generation mode.
///
/// # Examples
///
/// ```
/// use paladin::application::errors::prompt_error::PromptError;
///
/// let error = PromptError::GenerationFailed("LLM returned empty response".to_string());
/// assert!(error.to_string().contains("Prompt generation failed"));
/// ```
#[derive(Debug, Error)]
pub enum PromptError {
    /// LLM failed to generate a valid system prompt
    #[error("Prompt generation failed: {0}")]
    GenerationFailed(String),

    /// Agent description is invalid or empty
    #[error("Invalid agent description: {0}")]
    InvalidDescription(String),

    /// Requested cached prompt not found
    #[error("Cache miss for agent: {agent_name}")]
    CacheMiss {
        /// Name of the agent whose prompt was not cached
        agent_name: String,
    },

    /// Generated prompt is empty or invalid
    #[error("Invalid prompt generated: {0}")]
    InvalidPrompt(String),

    /// LLM error during prompt generation
    #[error("LLM error: {0}")]
    LlmError(String),

    /// Prompt regeneration failed
    #[error("Regeneration failed: {0}")]
    RegenerationFailed(String),

    /// Configuration error for prompt generation
    #[error("Prompt generation configuration error: {0}")]
    ConfigurationError(String),

    /// Cache operation failed (read/write)
    #[error("Cache operation failed: {0}")]
    CacheError(String),

    /// Timeout during prompt generation
    #[error("Prompt generation timeout after {0} seconds")]
    Timeout(u64),

    /// Prompt too long for model context window
    #[error("Generated prompt exceeds maximum length: {length} > {max}")]
    PromptTooLong {
        /// Generated prompt length in tokens
        length: usize,
        /// Maximum allowed length
        max: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_error_generation_failed() {
        let error = PromptError::GenerationFailed("Empty LLM response".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Prompt generation failed"));
        assert!(msg.contains("Empty LLM response"));
    }

    #[test]
    fn test_prompt_error_invalid_description() {
        let error = PromptError::InvalidDescription("Description is empty".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Invalid agent description"));
        assert!(msg.contains("Description is empty"));
    }

    #[test]
    fn test_prompt_error_cache_miss() {
        let error = PromptError::CacheMiss {
            agent_name: "DataAnalyst".to_string(),
        };
        let msg = error.to_string();
        assert!(msg.contains("Cache miss"));
        assert!(msg.contains("DataAnalyst"));
    }

    #[test]
    fn test_prompt_error_invalid_prompt() {
        let error = PromptError::InvalidPrompt("Prompt contains forbidden keywords".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Invalid prompt generated"));
        assert!(msg.contains("forbidden keywords"));
    }

    #[test]
    fn test_prompt_error_llm_error() {
        let error = PromptError::LlmError("API rate limit exceeded".to_string());
        let msg = error.to_string();
        assert!(msg.contains("LLM error"));
        assert!(msg.contains("rate limit"));
    }

    #[test]
    fn test_prompt_error_regeneration_failed() {
        let error = PromptError::RegenerationFailed("Cache invalidation failed".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Regeneration failed"));
        assert!(msg.contains("Cache invalidation"));
    }

    #[test]
    fn test_prompt_error_configuration() {
        let error = PromptError::ConfigurationError("Missing agent name".to_string());
        let msg = error.to_string();
        assert!(msg.contains("configuration error"));
        assert!(msg.contains("Missing agent name"));
    }

    #[test]
    fn test_prompt_error_cache_error() {
        let error = PromptError::CacheError("Disk write failed".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Cache operation failed"));
        assert!(msg.contains("Disk write"));
    }

    #[test]
    fn test_prompt_error_timeout() {
        let error = PromptError::Timeout(30);
        let msg = error.to_string();
        assert!(msg.contains("timeout"));
        assert!(msg.contains("30"));
    }

    #[test]
    fn test_prompt_error_prompt_too_long() {
        let error = PromptError::PromptTooLong {
            length: 5000,
            max: 4096,
        };
        let msg = error.to_string();
        assert!(msg.contains("exceeds maximum length"));
        assert!(msg.contains("5000"));
        assert!(msg.contains("4096"));
    }
}
