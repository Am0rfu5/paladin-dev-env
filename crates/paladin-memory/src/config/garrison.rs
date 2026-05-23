//! Garrison configuration types for the paladin-memory crate.
//!
//! Defines [`GarrisonSettings`] — the application-level configuration struct
//! for the Garrison conversation-history subsystem.

use paladin_core::platform::container::garrison::EvictionStrategy;
use serde::{Deserialize, Serialize};

/// Configuration for the Garrison memory system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonSettings {
    /// Type of garrison storage: `"in_memory"` or `"sqlite"`.
    pub garrison_type: String,
    /// Path to SQLite database file (only used when `garrison_type` is `"sqlite"`).
    pub path: Option<String>,
    /// Maximum number of entries to keep in memory.
    pub max_entries: usize,
    /// Maximum total tokens across all entries (`None` = no limit).
    pub max_tokens: Option<u32>,
    /// Tokenizer to use for token counting: `"gpt-4"`, `"gpt-3.5-turbo"`, etc.
    pub tokenizer: String,
    /// Eviction strategy: `"importance_based"`, `"fifo"`, or `"sliding_window"`.
    pub eviction_strategy: String,
    /// Number of recent entries to always preserve.
    pub preserve_recent_count: usize,
}

impl Default for GarrisonSettings {
    fn default() -> Self {
        Self {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        }
    }
}

impl GarrisonSettings {
    /// Validates garrison configuration.
    ///
    /// Returns `Err(String)` describing the first validation failure found.
    pub fn validate(&self) -> Result<(), String> {
        if self.garrison_type != "in_memory" && self.garrison_type != "sqlite" {
            return Err(format!(
                "Invalid garrison_type '{}': must be 'in_memory' or 'sqlite'",
                self.garrison_type
            ));
        }

        if self.garrison_type == "sqlite" && self.path.is_none() {
            return Err("SQLite garrison requires a 'path' to be specified".to_string());
        }

        if self.max_entries == 0 {
            return Err("max_entries must be greater than 0".to_string());
        }

        if self.preserve_recent_count > self.max_entries {
            return Err(format!(
                "preserve_recent_count ({}) cannot exceed max_entries ({})",
                self.preserve_recent_count, self.max_entries
            ));
        }

        if self.eviction_strategy != "importance_based"
            && self.eviction_strategy != "fifo"
            && self.eviction_strategy != "sliding_window"
        {
            return Err(format!(
                "Invalid eviction_strategy '{}': must be 'importance_based', 'fifo', or 'sliding_window'",
                self.eviction_strategy
            ));
        }

        Ok(())
    }

    /// Converts the `eviction_strategy` string to an [`EvictionStrategy`] enum.
    ///
    /// Defaults to [`EvictionStrategy::ImportanceBased`] for unknown strings.
    pub fn get_eviction_strategy(&self) -> EvictionStrategy {
        match self.eviction_strategy.as_str() {
            "fifo" => EvictionStrategy::FIFO,
            "sliding_window" => EvictionStrategy::SlidingWindow,
            "importance_based" => EvictionStrategy::ImportanceBased,
            _ => EvictionStrategy::ImportanceBased,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garrison_settings_validation_success() {
        let valid = GarrisonSettings {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        };
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn test_garrison_settings_validation_invalid_type() {
        let invalid = GarrisonSettings {
            garrison_type: "invalid_type".to_string(),
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_sqlite_without_path() {
        let invalid = GarrisonSettings {
            garrison_type: "sqlite".to_string(),
            path: None,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_zero_max_entries() {
        let invalid = GarrisonSettings {
            max_entries: 0,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_preserve_exceeds_max() {
        let invalid = GarrisonSettings {
            max_entries: 10,
            preserve_recent_count: 20,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_invalid_eviction() {
        let invalid = GarrisonSettings {
            eviction_strategy: "invalid_strategy".to_string(),
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_garrison_get_eviction_strategy() {
        let s = |strat: &str| GarrisonSettings {
            eviction_strategy: strat.to_string(),
            ..Default::default()
        };

        assert!(matches!(
            s("importance_based").get_eviction_strategy(),
            EvictionStrategy::ImportanceBased
        ));
        assert!(matches!(
            s("fifo").get_eviction_strategy(),
            EvictionStrategy::FIFO
        ));
        assert!(matches!(
            s("sliding_window").get_eviction_strategy(),
            EvictionStrategy::SlidingWindow
        ));
        assert!(matches!(
            s("invalid").get_eviction_strategy(),
            EvictionStrategy::ImportanceBased
        ));
    }
}
