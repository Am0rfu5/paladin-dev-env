//! RAG and memory-extraction configuration types for the paladin-memory crate.
//!
//! This module provides:
//! - [`RetrievalTrigger`] — when to trigger RAG retrieval
//! - [`RagConfig`] — unified RAG configuration (merged from `application_settings` and `rag_retrieval_service`)
//! - [`MemoryExtractionStrategy`] — when to extract memories from conversations
//! - [`MemoryExtractionConfig`] — configuration for the memory-extraction pipeline

use serde::{Deserialize, Serialize};

// ── RetrievalTrigger ─────────────────────────────────────────────────────────

/// When to trigger memory retrieval during Paladin execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RetrievalTrigger {
    /// Always retrieve memories for every query.
    Always,
    /// Retrieve only when specific keywords are detected.
    KeywordBased,
    /// Retrieve when semantic similarity exceeds a threshold.
    SemanticThreshold,
}

// ── RagConfig ────────────────────────────────────────────────────────────────

/// Unified configuration for RAG (Retrieval-Augmented Generation).
///
/// Combines the application-level settings (formerly in `application_settings.rs`)
/// and the service-level configuration (formerly in `rag_retrieval_service.rs`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    /// Number of top results to retrieve from Sanctum.
    pub top_k: usize,
    /// Minimum similarity score threshold (0.0–1.0).
    pub min_similarity: f32,
    /// Maximum tokens to include in the RAG context.
    pub max_tokens: usize,
    /// Timeout for RAG retrieval in seconds.
    pub timeout_seconds: u64,
    /// When to trigger memory retrieval.
    pub retrieval_trigger: RetrievalTrigger,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            min_similarity: 0.7,
            max_tokens: 2000,
            timeout_seconds: 5,
            retrieval_trigger: RetrievalTrigger::Always,
        }
    }
}

impl RagConfig {
    /// Validates RAG configuration.
    ///
    /// Returns `Err(String)` describing the first validation failure found.
    pub fn validate(&self) -> Result<(), String> {
        if self.top_k == 0 {
            return Err("RAG top_k must be greater than 0".to_string());
        }

        if self.top_k > 100 {
            return Err(format!(
                "RAG top_k {} seems unusually large (max 100)",
                self.top_k
            ));
        }

        if !(0.0..=1.0).contains(&self.min_similarity) {
            return Err(format!(
                "RAG min_similarity {} must be between 0.0 and 1.0",
                self.min_similarity
            ));
        }

        if self.max_tokens == 0 {
            return Err("RAG max_tokens must be greater than 0".to_string());
        }

        if self.timeout_seconds == 0 {
            return Err("RAG timeout_seconds must be greater than 0".to_string());
        }

        Ok(())
    }
}

// ── MemoryExtractionStrategy ─────────────────────────────────────────────────

/// Strategy for when to extract memories from conversations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MemoryExtractionStrategy {
    /// Extract after every conversation turn.
    EveryTurn,
    /// Extract only when the conversation completes (recommended).
    #[default]
    OnCompletion,
    /// Manual extraction only (user-triggered).
    Manual,
    /// Extract when importance threshold is exceeded.
    Threshold { importance: u8 },
}

// ── MemoryExtractionConfig ────────────────────────────────────────────────────

/// Configuration for the memory-extraction pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryExtractionConfig {
    /// Memory extraction strategy.
    pub strategy: MemoryExtractionStrategy,
    /// Enable automatic extraction.
    pub enabled: bool,
}

impl Default for MemoryExtractionConfig {
    fn default() -> Self {
        Self {
            strategy: MemoryExtractionStrategy::OnCompletion,
            enabled: true,
        }
    }
}

impl MemoryExtractionConfig {
    /// Validates memory extraction configuration.
    ///
    /// Returns `Err(String)` if the configuration is invalid.
    pub fn validate(&self) -> Result<(), String> {
        if let MemoryExtractionStrategy::Threshold { importance } = self.strategy
            && importance == 0
        {
            return Err(
                "Memory extraction threshold importance must be greater than 0".to_string(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_config_defaults() {
        let cfg = RagConfig::default();
        assert_eq!(cfg.top_k, 5);
        assert!((cfg.min_similarity - 0.7).abs() < f32::EPSILON);
        assert_eq!(cfg.max_tokens, 2000);
        assert_eq!(cfg.timeout_seconds, 5);
        assert_eq!(cfg.retrieval_trigger, RetrievalTrigger::Always);
    }

    #[test]
    fn test_rag_config_validate_ok() {
        assert!(RagConfig::default().validate().is_ok());
    }

    #[test]
    fn test_rag_config_validate_zero_top_k() {
        let cfg = RagConfig {
            top_k: 0,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_rag_config_validate_bad_similarity() {
        let cfg = RagConfig {
            min_similarity: 1.5,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_rag_config_validate_zero_timeout() {
        let cfg = RagConfig {
            timeout_seconds: 0,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_memory_extraction_config_defaults() {
        let cfg = MemoryExtractionConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.strategy, MemoryExtractionStrategy::OnCompletion);
    }

    #[test]
    fn test_memory_extraction_threshold_zero_invalid() {
        let cfg = MemoryExtractionConfig {
            strategy: MemoryExtractionStrategy::Threshold { importance: 0 },
            enabled: true,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_memory_extraction_threshold_nonzero_valid() {
        let cfg = MemoryExtractionConfig {
            strategy: MemoryExtractionStrategy::Threshold { importance: 5 },
            enabled: true,
        };
        assert!(cfg.validate().is_ok());
    }
}
