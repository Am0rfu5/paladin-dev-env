//! Configuration for the Citadel state persistence system.

use crate::config::env_utils::EnvOverridable;
use serde::{Deserialize, Serialize};

/// Configuration for Citadel state persistence system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitadelConfig {
    /// Enable or disable state persistence
    pub enabled: bool,
    /// Directory where state files will be saved
    pub state_dir: String,
    /// Enable automatic saving after Paladin execution
    pub autosave_enabled: bool,
    /// Enable automatic state cleanup (remove old states)
    pub cleanup_enabled: bool,
    /// Maximum age of state files in days before cleanup (if cleanup_enabled is true)
    pub max_state_age_days: Option<u32>,
}

impl Default for CitadelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            state_dir: "./paladin-states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: false,
            max_state_age_days: Some(30),
        }
    }
}

impl CitadelConfig {
    /// Validates citadel configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled && self.state_dir.trim().is_empty() {
            return Err("state_dir cannot be empty when citadel is enabled".to_string());
        }

        if self.cleanup_enabled {
            if let Some(max_age) = self.max_state_age_days {
                if max_age == 0 {
                    return Err(
                        "max_state_age_days must be greater than 0 when cleanup is enabled"
                            .to_string(),
                    );
                }
            } else {
                return Err(
                    "max_state_age_days must be specified when cleanup_enabled is true".to_string(),
                );
            }
        }

        Ok(())
    }
}

impl EnvOverridable for CitadelConfig {
    fn apply_env_overrides(&mut self) {
        if let Ok(v) = std::env::var("APP_CITADEL_ENABLED")
            && let Ok(b) = v.parse::<bool>()
        {
            self.enabled = b;
        }
        if let Ok(v) = std::env::var("APP_CITADEL_STATE_DIR") {
            self.state_dir = v;
        }
        if let Ok(v) = std::env::var("APP_CITADEL_AUTOSAVE_ENABLED")
            && let Ok(b) = v.parse::<bool>()
        {
            self.autosave_enabled = b;
        }
        if let Ok(v) = std::env::var("APP_CITADEL_CLEANUP_ENABLED")
            && let Ok(b) = v.parse::<bool>()
        {
            self.cleanup_enabled = b;
        }
        if let Ok(v) = std::env::var("APP_CITADEL_MAX_STATE_AGE_DAYS")
            && let Ok(n) = v.parse::<u32>()
        {
            self.max_state_age_days = Some(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_citadel_config_default() {
        let config = CitadelConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.state_dir, "./paladin-states");
        assert!(!config.autosave_enabled);
        assert!(!config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(30));
    }

    #[test]
    fn test_citadel_config_validate_valid() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: true,
            cleanup_enabled: true,
            max_state_age_days: Some(7),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_citadel_config_validate_empty_state_dir_when_enabled() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "   ".to_string(),
            autosave_enabled: false,
            cleanup_enabled: false,
            max_state_age_days: Some(30),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("state_dir cannot be empty when citadel is enabled")
        );
    }

    #[test]
    fn test_citadel_config_validate_cleanup_no_max_age() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: true,
            max_state_age_days: None,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("max_state_age_days must be specified when cleanup_enabled is true")
        );
    }

    #[test]
    fn test_citadel_config_validate_cleanup_zero_max_age() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: true,
            max_state_age_days: Some(0),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("max_state_age_days must be greater than 0 when cleanup is enabled")
        );
    }

    #[test]
    #[serial]
    fn test_citadel_config_env_override() {
        unsafe {
            env::set_var("APP_CITADEL_ENABLED", "true");
            env::set_var("APP_CITADEL_STATE_DIR", "/custom/states");
            env::set_var("APP_CITADEL_AUTOSAVE_ENABLED", "true");
            env::set_var("APP_CITADEL_CLEANUP_ENABLED", "true");
            env::set_var("APP_CITADEL_MAX_STATE_AGE_DAYS", "60");
        }

        let mut config = CitadelConfig::default();
        config.apply_env_overrides();

        assert!(config.enabled);
        assert_eq!(config.state_dir, "/custom/states");
        assert!(config.autosave_enabled);
        assert!(config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(60));

        unsafe {
            env::remove_var("APP_CITADEL_ENABLED");
            env::remove_var("APP_CITADEL_STATE_DIR");
            env::remove_var("APP_CITADEL_AUTOSAVE_ENABLED");
            env::remove_var("APP_CITADEL_CLEANUP_ENABLED");
            env::remove_var("APP_CITADEL_MAX_STATE_AGE_DAYS");
        }
    }

    #[test]
    fn test_citadel_config_deserialization_from_yml() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./test-states".to_string(),
            autosave_enabled: true,
            cleanup_enabled: false,
            max_state_age_days: Some(14),
        };

        assert!(config.enabled);
        assert_eq!(config.state_dir, "./test-states");
        assert!(config.autosave_enabled);
        assert!(!config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(14));
    }
}
