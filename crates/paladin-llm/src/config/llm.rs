//! LLM provider configuration types for the paladin-llm crate.

use serde::{Deserialize, Serialize};

use crate::provider_factory::provider_names;

/// Configuration for an individual LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    /// API key for the provider (can use `${ENV_VAR}` syntax).
    pub api_key: String,
    /// Base URL for the API endpoint.
    pub base_url: Option<String>,
    /// Default model to use.
    pub default_model: Option<String>,
    /// Default temperature (0.0–2.0).
    pub default_temperature: Option<f32>,
    /// Default timeout in seconds.
    pub timeout_seconds: Option<u64>,
    /// Maximum retries for failed requests.
    pub max_retries: Option<u32>,
}

/// Configuration for all LLM providers.
///
/// One `Option<LlmProviderConfig>` field per registered provider (Claude's
/// Discretion, `17-CONTEXT.md`: six additional named fields rather than a
/// `HashMap<String, LlmProviderConfig>`, so an existing `config.yml` naming
/// only `openai`/`deepseek`/`anthropic` keeps deserializing byte-for-byte —
/// every field defaults to `None` when its key is absent, per serde's own
/// `Option<T>`-is-absent-tolerant behaviour, with no `#[serde(default)]`
/// needed except where `openai_compatible`'s hyphenated rename requires one).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Default provider to use if not specified.
    ///
    /// Accepts any name this build's provider registry knows
    /// ([`provider_names`]) — `"openai"`, `"deepseek"`, `"anthropic"`, and
    /// (when compiled in) `"kimi"`, `"qwen"`, `"grok"`, `"ollama"`,
    /// `"gemini"`, `"openai-compatible"`.
    pub default_provider: Option<String>,
    /// OpenAI configuration.
    pub openai: Option<LlmProviderConfig>,
    /// DeepSeek configuration.
    pub deepseek: Option<LlmProviderConfig>,
    /// Anthropic configuration.
    pub anthropic: Option<LlmProviderConfig>,
    /// Kimi (Moonshot AI) configuration.
    pub kimi: Option<LlmProviderConfig>,
    /// Qwen (DashScope) configuration.
    pub qwen: Option<LlmProviderConfig>,
    /// Grok (xAI) configuration.
    pub grok: Option<LlmProviderConfig>,
    /// Ollama (self-hosted, credential-free — D-12) configuration.
    pub ollama: Option<LlmProviderConfig>,
    /// Gemini configuration.
    pub gemini: Option<LlmProviderConfig>,
    /// Generic operator-configured OpenAI-compatible provider configuration
    /// (D-03). Accepts the hyphenated key `openai-compatible` (matching the
    /// provider's own registered name) and the snake_case alias
    /// `openai_compatible`, so both an operator typing the provider literal
    /// verbatim and a `#[derive(Deserialize)]` consumer keying off the Rust
    /// field name land on the same field.
    #[serde(default, rename = "openai-compatible", alias = "openai_compatible")]
    pub openai_compatible: Option<LlmProviderConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
            kimi: None,
            qwen: None,
            grok: None,
            ollama: None,
            gemini: None,
            openai_compatible: None,
        }
    }
}

impl LlmConfig {
    /// Validate LLM configuration.
    ///
    /// Returns `Err(String)` describing the first validation failure found.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(default) = &self.default_provider {
            match self.get_provider_config(default) {
                Some(_) => {}
                None => {
                    // `get_provider_config` recognises all nine field names
                    // unconditionally (it is not gated on which paladin-llm
                    // adapter features happen to be compiled), so reaching
                    // this branch means either the name is one of the nine
                    // but its block is absent, or the name is not one of
                    // the nine at all. `provider_names()` (the D-10
                    // registry, compiled-features-aware) is consulted only
                    // to phrase the second case's message with the names
                    // this build actually registers — never to decide which
                    // branch we are in, since that decision must stay
                    // correct under every feature combination this crate
                    // builds with (including combinations that compile out
                    // some of the nine config fields' matching adapter).
                    if is_recognised_provider_field_name(default) {
                        return Err(format!(
                            "Default provider is '{default}' but {default} config is not present"
                        ));
                    }
                    return Err(format!(
                        "Invalid default provider: {default}. Must be one of: {}",
                        provider_names().join(", ")
                    ));
                }
            }
        }

        // Every provider block requires a non-empty api_key EXCEPT ollama,
        // which legitimately has none (D-12: Ollama requires no
        // credential) — do not "fix" the missing ollama entry below, its
        // absence from this list is deliberate.
        let keyed_blocks: [(&str, &Option<LlmProviderConfig>); 8] = [
            ("OpenAI", &self.openai),
            ("DeepSeek", &self.deepseek),
            ("Anthropic", &self.anthropic),
            ("Kimi", &self.kimi),
            ("Qwen", &self.qwen),
            ("Grok", &self.grok),
            ("Gemini", &self.gemini),
            ("openai-compatible", &self.openai_compatible),
        ];

        for (display_name, block) in keyed_blocks {
            if let Some(cfg) = block
                && cfg.api_key.is_empty()
            {
                return Err(format!("{display_name} API key cannot be empty"));
            }
        }

        Ok(())
    }

    /// Get the provider config for a specific provider name (case-insensitive).
    ///
    /// Recognises all nine field names this struct carries, regardless of
    /// which paladin-llm adapter features are compiled into this build —
    /// the config surface is decoupled from the (feature-gated) registry
    /// [`provider_names`] enumerates, matching PROV-03's requirement that
    /// existing config files keep loading unchanged.
    pub fn get_provider_config(&self, provider_name: &str) -> Option<&LlmProviderConfig> {
        match provider_name.to_lowercase().as_str() {
            "openai" => self.openai.as_ref(),
            "deepseek" => self.deepseek.as_ref(),
            "anthropic" => self.anthropic.as_ref(),
            "kimi" => self.kimi.as_ref(),
            "qwen" => self.qwen.as_ref(),
            "grok" => self.grok.as_ref(),
            "ollama" => self.ollama.as_ref(),
            "gemini" => self.gemini.as_ref(),
            "openai-compatible" | "openai_compatible" => self.openai_compatible.as_ref(),
            _ => None,
        }
    }

    /// Get the default provider name.
    pub fn get_default_provider_name(&self) -> Option<String> {
        self.default_provider.clone()
    }
}

/// Whether `name` (case-insensitive) is one of the nine provider field names
/// [`LlmConfig`] structurally recognises — independent of which paladin-llm
/// adapter features this build compiled in. Used only to phrase
/// [`LlmConfig::validate`]'s error message; never to decide whether a
/// present config block is honoured (that is `get_provider_config`'s job).
fn is_recognised_provider_field_name(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "openai"
            | "deepseek"
            | "anthropic"
            | "kimi"
            | "qwen"
            | "grok"
            | "ollama"
            | "gemini"
            | "openai-compatible"
            | "openai_compatible"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert_eq!(config.default_provider, Some("openai".to_string()));
        assert!(config.openai.is_none());
        assert!(config.deepseek.is_none());
        assert!(config.anthropic.is_none());
    }

    #[test]
    fn test_llm_config_validate_default_provider_must_be_configured() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_invalid_provider_name() {
        let config = LlmConfig {
            default_provider: Some("invalid_provider".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_empty_api_key() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_success() {
        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(LlmProviderConfig {
                api_key: "test-key".to_string(),
                base_url: Some("https://api.deepseek.com/v1".to_string()),
                default_model: Some("deepseek-chat".to_string()),
                default_temperature: Some(0.7),
                timeout_seconds: Some(300),
                max_retries: Some(3),
            }),
            anthropic: None,
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_llm_config_get_provider_config() {
        let openai_config = LlmProviderConfig {
            api_key: "openai-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(openai_config),
            deepseek: None,
            anthropic: None,
            ..Default::default()
        };

        let retrieved = config.get_provider_config("openai");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().api_key, "openai-key");

        let not_found = config.get_provider_config("deepseek");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_llm_config_get_provider_config_case_insensitive() {
        let deepseek_config = LlmProviderConfig {
            api_key: "deepseek-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(deepseek_config),
            anthropic: None,
            ..Default::default()
        };

        assert!(config.get_provider_config("DeepSeek").is_some());
        assert!(config.get_provider_config("DEEPSEEK").is_some());
        assert!(config.get_provider_config("deepseek").is_some());
    }

    #[test]
    fn test_llm_config_get_default_provider_name() {
        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            ..Default::default()
        };

        assert_eq!(
            config.get_default_provider_name(),
            Some("anthropic".to_string())
        );
    }

    // ── New behaviour, Task 3 (17-06): the nine-provider config surface ──

    /// PROV-03's own "done when": a config naming only the three providers
    /// that existed before this phase — no `kimi`/`qwen`/`grok`/`ollama`/
    /// `gemini`/`openai-compatible` key at all — must keep deserializing
    /// and validating exactly as before. Exercised via `serde_json` rather
    /// than YAML: this crate carries no YAML deserializer dependency, and
    /// the behaviour under test (serde's derived `Deserialize` impl
    /// treating a missing `Option<T>` field as `None`, per serde's own
    /// `missing_field` helper) is format-independent — a JSON object and a
    /// YAML mapping exercise the identical generated code path.
    #[test]
    fn test_llm_config_pre_phase_17_fixture_still_deserializes_and_validates() {
        let fixture = r#"{
            "default_provider": "openai",
            "openai": {
                "api_key": "test-key",
                "base_url": null,
                "default_model": null,
                "default_temperature": null,
                "timeout_seconds": null,
                "max_retries": null
            },
            "deepseek": {
                "api_key": "test-key",
                "base_url": null,
                "default_model": null,
                "default_temperature": null,
                "timeout_seconds": null,
                "max_retries": null
            },
            "anthropic": {
                "api_key": "test-key",
                "base_url": null,
                "default_model": null,
                "default_temperature": null,
                "timeout_seconds": null,
                "max_retries": null
            }
        }"#;

        let config: LlmConfig = serde_json::from_str(fixture).expect(
            "a pre-phase-17 config naming only openai/deepseek/anthropic must still deserialize",
        );

        assert!(config.kimi.is_none());
        assert!(config.qwen.is_none());
        assert!(config.grok.is_none());
        assert!(config.ollama.is_none());
        assert!(config.gemini.is_none());
        assert!(config.openai_compatible.is_none());
        assert!(config.validate().is_ok());
    }

    /// New behaviour: a config naming each of the six providers Phase 17
    /// added deserializes into its matching field, including the
    /// hyphenated `openai-compatible` key.
    #[test]
    fn test_llm_config_new_provider_names_deserialize_into_matching_fields() {
        let fixture = r#"{
            "default_provider": "kimi",
            "kimi": {"api_key": "kimi-key", "base_url": null, "default_model": null, "default_temperature": null, "timeout_seconds": null, "max_retries": null},
            "qwen": {"api_key": "qwen-key", "base_url": null, "default_model": null, "default_temperature": null, "timeout_seconds": null, "max_retries": null},
            "grok": {"api_key": "grok-key", "base_url": null, "default_model": null, "default_temperature": null, "timeout_seconds": null, "max_retries": null},
            "ollama": {"api_key": "", "base_url": null, "default_model": null, "default_temperature": null, "timeout_seconds": null, "max_retries": null},
            "gemini": {"api_key": "gemini-key", "base_url": null, "default_model": null, "default_temperature": null, "timeout_seconds": null, "max_retries": null},
            "openai-compatible": {"api_key": "compat-key", "base_url": "https://compat.example.com/v1", "default_model": "compat-model", "default_temperature": null, "timeout_seconds": null, "max_retries": null}
        }"#;

        let config: LlmConfig =
            serde_json::from_str(fixture).expect("the six-new-provider fixture must deserialize");

        assert_eq!(config.kimi.as_ref().unwrap().api_key, "kimi-key");
        assert_eq!(config.qwen.as_ref().unwrap().api_key, "qwen-key");
        assert_eq!(config.grok.as_ref().unwrap().api_key, "grok-key");
        assert!(config.ollama.is_some());
        assert_eq!(config.gemini.as_ref().unwrap().api_key, "gemini-key");
        assert_eq!(
            config.openai_compatible.as_ref().unwrap().api_key,
            "compat-key"
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_llm_config_get_provider_config_openai_compatible_hyphen_and_underscore_case_insensitive()
     {
        let config = LlmConfig {
            openai_compatible: Some(LlmProviderConfig {
                api_key: "compat-key".to_string(),
                base_url: Some("https://compat.example.com/v1".to_string()),
                default_model: Some("compat-model".to_string()),
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            ..Default::default()
        };

        assert!(config.get_provider_config("OPENAI-COMPATIBLE").is_some());
        assert!(config.get_provider_config("openai_compatible").is_some());
        assert_eq!(
            config
                .get_provider_config("openai-compatible")
                .unwrap()
                .api_key,
            "compat-key"
        );
    }

    /// D-12: Ollama requires no credential, so an empty `api_key` on its
    /// block is not a validation error — the one deliberate exception to
    /// every other provider's non-empty-api_key rule.
    #[test]
    fn test_llm_config_validate_ollama_empty_api_key_is_allowed() {
        let config = LlmConfig {
            default_provider: Some("ollama".to_string()),
            ollama: Some(LlmProviderConfig {
                api_key: "".to_string(),
                base_url: Some("http://localhost:11434/v1".to_string()),
                default_model: Some("llama3".to_string()),
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    /// The error message for a totally unrecognised `default_provider`
    /// lists names drawn from [`provider_names`] rather than a hardcoded
    /// three — gated on the six-new-preset feature combination (this
    /// plan's own `<verify>` command) so the "at least three" assertion is
    /// only exercised where this build's registry actually has that many
    /// rows; a default-features build (`openai` + `mock` only) has a
    /// one-row registry and would fail an unconditional version of this
    /// assertion for reasons that have nothing to do with the behaviour
    /// under test.
    #[cfg(all(
        feature = "kimi",
        feature = "qwen",
        feature = "grok",
        feature = "ollama",
        feature = "openai-compatible",
        feature = "gemini"
    ))]
    #[test]
    fn test_llm_config_validate_unknown_provider_name_lists_registered_names() {
        let config = LlmConfig {
            default_provider: Some("nosuchprovider".to_string()),
            ..Default::default()
        };

        let err = config
            .validate()
            .expect_err("an unrecognised default_provider must be rejected");
        let matched = [
            "kimi",
            "qwen",
            "grok",
            "ollama",
            "gemini",
            "openai-compatible",
        ]
        .iter()
        .filter(|name| err.contains(**name))
        .count();
        assert!(
            matched >= 3,
            "error message must contain at least three registered provider names: {err}"
        );
    }
}
