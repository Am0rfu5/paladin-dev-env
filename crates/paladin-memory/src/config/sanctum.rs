//! Sanctum configuration types for the paladin-memory crate.
//!
//! Defines [`SanctumAdapterType`], [`QdrantSanctumConfig`], and [`SanctumConfig`]
//! — the application-level configuration structs for the Sanctum long-term
//! vector-memory subsystem.

use serde::{Deserialize, Serialize};

/// Type of Sanctum adapter to use.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SanctumAdapterType {
    /// In-memory storage (ephemeral, fast — for development/testing).
    #[default]
    InMemory,
    /// Qdrant vector database (persistent, production-grade).
    Qdrant,
}

/// Configuration for the Qdrant vector database adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantSanctumConfig {
    /// Qdrant server URL (e.g., `"http://localhost:6334"`).
    pub url: String,
    /// Collection name to use for storing memories.
    pub collection_name: String,
    /// Vector dimension (must match the embedding model).
    ///
    /// Common values: 1536 (OpenAI `text-embedding-3-small`), 3072 (`text-embedding-3-large`).
    pub vector_dimension: usize,
}

impl Default for QdrantSanctumConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6334".to_string(),
            collection_name: "paladin_memories".to_string(),
            vector_dimension: 1536,
        }
    }
}

/// Configuration for the Sanctum long-term memory system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctumConfig {
    /// Enable or disable the Sanctum system.
    pub enabled: bool,
    /// Type of adapter to use.
    pub adapter_type: SanctumAdapterType,
    /// Qdrant-specific configuration (required when `adapter_type` is [`SanctumAdapterType::Qdrant`]).
    pub qdrant: Option<QdrantSanctumConfig>,
}

impl Default for SanctumConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            adapter_type: SanctumAdapterType::InMemory,
            qdrant: None,
        }
    }
}

impl SanctumConfig {
    /// Validates sanctum configuration.
    ///
    /// Returns `Err(String)` describing the first validation failure found.
    /// No validation is performed when `enabled` is `false`.
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.adapter_type == SanctumAdapterType::Qdrant {
            let qdrant = self.qdrant.as_ref().ok_or_else(|| {
                "Qdrant adapter requires 'qdrant' configuration section".to_string()
            })?;

            if qdrant.url.trim().is_empty() {
                return Err("Qdrant URL cannot be empty".to_string());
            }

            if qdrant.collection_name.trim().is_empty() {
                return Err("Qdrant collection_name cannot be empty".to_string());
            }

            if qdrant.vector_dimension == 0 {
                return Err("Qdrant vector_dimension must be greater than 0".to_string());
            }

            if qdrant.vector_dimension > 10000 {
                return Err(format!(
                    "Qdrant vector_dimension {} seems unusually large (max 10000)",
                    qdrant.vector_dimension
                ));
            }
        }

        Ok(())
    }

    /// Returns the adapter type as a display string.
    pub fn adapter_type_str(&self) -> &str {
        match self.adapter_type {
            SanctumAdapterType::InMemory => "in_memory",
            SanctumAdapterType::Qdrant => "qdrant",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_sanctum_config() {
        let config = SanctumConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.adapter_type, SanctumAdapterType::InMemory);
        assert!(config.qdrant.is_none());
    }

    #[test]
    fn test_sanctum_validation_disabled() {
        // Disabled config with Qdrant + no qdrant section should still pass
        let config = SanctumConfig {
            enabled: false,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: None,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_in_memory() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::InMemory,
            qdrant: None,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_qdrant_missing_config() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: None,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("qdrant' configuration section")
        );
    }

    #[test]
    fn test_sanctum_validation_qdrant_valid() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "http://localhost:6334".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 1536,
            }),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_empty_url() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "   ".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 1536,
            }),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("URL cannot be empty"));
    }

    #[test]
    fn test_sanctum_validation_zero_dimension() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "http://localhost:6334".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 0,
            }),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be greater than 0"));
    }
}
