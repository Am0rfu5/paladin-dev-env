//! # LLM Provider Factory
//!
//! Creates [`paladin_ports::output::llm_port::LlmPort`] adapter instances by
//! provider name.
//!
//! Backed by a single `cfg`-gated, table-driven registry (D-10):
//! [`create`](LlmProviderFactory::create),
//! [`get_default_provider`](LlmProviderFactory::get_default_provider),
//! [`list_available_providers`](LlmProviderFactory::list_available_providers)
//! and [`ProviderFactoryError::UnknownProvider`]'s message all derive from
//! the same table. Adding a provider is one row in [`provider_registry`];
//! a provider whose feature is compiled out is structurally absent from
//! every one of those surfaces — there is no second lookup path to forget
//! to `cfg`-gate (the defect this replaces:
//! `provider_factory.rs:123-149` in the pre-D-10 shape reported a provider
//! as available whenever its env var was set, even when its feature had
//! been compiled out).

use std::sync::{Arc, OnceLock};

use paladin_ports::output::llm_port::LlmPort;
use thiserror::Error;

/// Errors that can be returned by [`LlmProviderFactory`].
#[derive(Debug, Error)]
pub enum ProviderFactoryError {
    /// The requested provider name is not recognised.
    #[error(
        "Unknown provider: {0}. Supported providers: {supported}",
        supported = provider_names().join(", ")
    )]
    UnknownProvider(String),

    /// Required environment variables or configuration are missing.
    #[error("Provider configuration missing: {0}")]
    ConfigurationMissing(String),

    /// The provider adapter could not be constructed.
    #[error("Failed to create provider adapter: {0}")]
    AdapterCreationFailed(String),
}

/// One row of the provider registry: a name, the env var whose presence
/// signals a credential is configured (`None` for a provider that needs no
/// credential, e.g. a future local-endpoint preset), and the constructor
/// function.
struct ProviderRegistration {
    name: &'static str,
    env_var: Option<&'static str>,
    construct: fn() -> Result<Arc<dyn LlmPort>, ProviderFactoryError>,
}

#[cfg(feature = "openai")]
fn construct_openai() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::openai::{OpenAIAdapter, OpenAIConfig};
    let config = OpenAIConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "OpenAI configuration error: {}. Ensure OPENAI_API_KEY is set.",
            e
        ))
    })?;
    let adapter = OpenAIAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create OpenAI adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "deepseek")]
fn construct_deepseek() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::deepseek::{DeepSeekAdapter, DeepSeekConfig};
    let config = DeepSeekConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "DeepSeek configuration error: {}. Ensure DEEPSEEK_API_KEY is set.",
            e
        ))
    })?;
    let adapter = DeepSeekAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create DeepSeek adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "anthropic")]
fn construct_anthropic() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::anthropic::{AnthropicAdapter, AnthropicConfig};
    let config = AnthropicConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "Anthropic configuration error: {}. Ensure ANTHROPIC_API_KEY is set.",
            e
        ))
    })?;
    let adapter = AnthropicAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create Anthropic adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "kimi")]
fn construct_kimi() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::kimi::{KimiAdapter, KimiConfig};
    let config = KimiConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "Kimi configuration error: {}. Ensure MOONSHOT_API_KEY is set.",
            e
        ))
    })?;
    let adapter = KimiAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!("Failed to create Kimi adapter: {}", e))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "qwen")]
fn construct_qwen() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::qwen::{QwenAdapter, QwenConfig};
    let config = QwenConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "Qwen configuration error: {}. Ensure DASHSCOPE_API_KEY is set.",
            e
        ))
    })?;
    let adapter = QwenAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!("Failed to create Qwen adapter: {}", e))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "grok")]
fn construct_grok() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::grok::{GrokAdapter, GrokConfig};
    let config = GrokConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "Grok configuration error: {}. Ensure XAI_API_KEY is set.",
            e
        ))
    })?;
    let adapter = GrokAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!("Failed to create Grok adapter: {}", e))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "gemini")]
fn construct_gemini() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::gemini::{GeminiAdapter, GeminiConfig};
    let config = GeminiConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "Gemini configuration error: {}. Ensure GEMINI_API_KEY is set.",
            e
        ))
    })?;
    let adapter = GeminiAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create Gemini adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "openai-compatible")]
fn construct_openai_compatible() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::openai_compatible::{OpenAiCompatibleAdapter, OpenAiCompatibleConfig};
    let config = OpenAiCompatibleConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!(
            "openai-compatible configuration error: {}. Ensure OPENAI_COMPATIBLE_API_KEY, \
             OPENAI_COMPATIBLE_BASE_URL and OPENAI_COMPATIBLE_MODEL are all set.",
            e
        ))
    })?;
    let adapter = OpenAiCompatibleAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create openai-compatible adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

#[cfg(feature = "ollama")]
fn construct_ollama() -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    use crate::ollama::{OllamaAdapter, OllamaConfig};
    // No credential env var to read — Ollama requires none (D-12). `from_env`
    // only fails if `OLLAMA_TIMEOUT_SECONDS` is set to an unparseable value.
    let config = OllamaConfig::from_env().map_err(|e| {
        ProviderFactoryError::ConfigurationMissing(format!("Ollama configuration error: {}", e))
    })?;
    let adapter = OllamaAdapter::new(config).map_err(|e| {
        ProviderFactoryError::AdapterCreationFailed(format!(
            "Failed to create Ollama adapter: {}",
            e
        ))
    })?;
    Ok(Arc::new(adapter))
}

/// Build the `cfg`-gated provider registry table. Exactly one row per
/// enabled provider feature; a provider whose feature is compiled out has
/// no row at all, so it cannot be reported as available regardless of
/// whether its env var happens to be set.
///
/// `vec![]` cannot express this construction: each row is individually
/// `#[cfg]`-gated on its own provider feature, so the number of pushes
/// varies per build — clippy's `vec_init_then_push` lint cannot see that
/// from a single call site, hence the explicit `allow`.
#[allow(unused_mut, clippy::vec_init_then_push)]
fn build_provider_registry() -> Vec<ProviderRegistration> {
    let mut rows: Vec<ProviderRegistration> = Vec::new();

    #[cfg(feature = "openai")]
    rows.push(ProviderRegistration {
        name: "openai",
        env_var: Some("OPENAI_API_KEY"),
        construct: construct_openai,
    });

    #[cfg(feature = "deepseek")]
    rows.push(ProviderRegistration {
        name: "deepseek",
        env_var: Some("DEEPSEEK_API_KEY"),
        construct: construct_deepseek,
    });

    #[cfg(feature = "anthropic")]
    rows.push(ProviderRegistration {
        name: "anthropic",
        env_var: Some("ANTHROPIC_API_KEY"),
        construct: construct_anthropic,
    });

    #[cfg(feature = "kimi")]
    rows.push(ProviderRegistration {
        name: "kimi",
        env_var: Some("MOONSHOT_API_KEY"),
        construct: construct_kimi,
    });

    #[cfg(feature = "qwen")]
    rows.push(ProviderRegistration {
        name: "qwen",
        env_var: Some("DASHSCOPE_API_KEY"),
        construct: construct_qwen,
    });

    #[cfg(feature = "grok")]
    rows.push(ProviderRegistration {
        name: "grok",
        env_var: Some("XAI_API_KEY"),
        construct: construct_grok,
    });

    // Gemini is a curated (named-vendor) preset like Kimi/Qwen/Grok — it
    // just happens to be bespoke-protocol rather than compat-engine-backed
    // (D-08). Declared alongside the other curated presets, before the
    // generic openai-compatible row and Ollama's credential-free row, for
    // the same reason those two are ordered last: neither should ever
    // pre-empt an explicitly-configured named provider in
    // `get_default_provider()`'s declared-table-order scan.
    #[cfg(feature = "gemini")]
    rows.push(ProviderRegistration {
        name: "gemini",
        env_var: Some("GEMINI_API_KEY"),
        construct: construct_gemini,
    });

    // Placed after every curated (named-vendor) preset row so it never
    // pre-empts an explicitly-configured named provider in
    // `get_default_provider()`'s declared-table-order scan, but BEFORE
    // Ollama: Ollama's `env_var: None` row unconditionally "matches" in
    // that scan, so if it were declared first it would always win and the
    // generic provider's own credential would never be reachable through
    // `get_default_provider()` at all.
    #[cfg(feature = "openai-compatible")]
    rows.push(ProviderRegistration {
        name: "openai-compatible",
        env_var: Some("OPENAI_COMPATIBLE_API_KEY"),
        construct: construct_openai_compatible,
    });

    // Placed after every credentialed row: a compiled-in, credential-free
    // Ollama must never pre-empt an explicitly-configured hosted provider
    // in `get_default_provider()`'s declared-table-order scan.
    #[cfg(feature = "ollama")]
    rows.push(ProviderRegistration {
        name: "ollama",
        env_var: None,
        construct: construct_ollama,
    });

    rows
}

/// The `cfg`-gated provider registry table, memoized for the process
/// lifetime.
fn provider_registry() -> &'static [ProviderRegistration] {
    static REGISTRY: OnceLock<Vec<ProviderRegistration>> = OnceLock::new();
    REGISTRY.get_or_init(build_provider_registry)
}

/// The names of every provider compiled into this build, in registry
/// declaration order. Consumed by the config layer and by documentation
/// tooling so both read one list rather than each hand-maintaining a copy.
pub fn provider_names() -> Vec<&'static str> {
    provider_registry().iter().map(|row| row.name).collect()
}

/// Factory for creating LLM provider adapters by name.
///
/// Each provider is only available when the corresponding feature flag is
/// enabled. See [`provider_names`] for the list compiled into this build.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "openai")]
/// # {
/// use paladin_llm::provider_factory::LlmProviderFactory;
///
/// let factory = LlmProviderFactory::new();
/// let provider = factory.create("openai").expect("OPENAI_API_KEY must be set");
/// # }
/// ```
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    /// Create a new provider factory.
    pub fn new() -> Self {
        Self
    }

    /// Create an [`LlmPort`] adapter by provider name.
    ///
    /// Configuration is loaded from environment variables (see each
    /// adapter's `*Config::from_env()` for the expected variable names).
    /// The name is matched case-insensitively against
    /// [`provider_names`].
    ///
    /// # Errors
    ///
    /// Returns [`ProviderFactoryError`] if the provider name is unknown, a
    /// required environment variable is absent, or the adapter fails to
    /// initialise.
    pub fn create(&self, provider_name: &str) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
        let lower = provider_name.to_lowercase();
        provider_registry()
            .iter()
            .find(|row| row.name == lower)
            .map(|row| (row.construct)())
            .unwrap_or_else(|| {
                Err(ProviderFactoryError::UnknownProvider(
                    provider_name.to_string(),
                ))
            })
    }

    /// Return the name of the first available provider — the first table
    /// row (in registry declaration order) whose credential env var holds a
    /// non-blank value, or whose row requires no credential at all.
    ///
    /// A variable set to an empty or whitespace-only string is treated as
    /// unset: every preset's own `validate()` rejects such a value, so
    /// reporting it as available here would name a provider `create()`
    /// refuses to construct.
    ///
    /// Returns `None` when the table is empty (no provider feature
    /// compiled in) or no row's credential is present and non-blank.
    pub fn get_default_provider() -> Option<String> {
        provider_registry()
            .iter()
            .find(|row| match row.env_var {
                Some(var) => std::env::var(var).is_ok_and(|v| !v.trim().is_empty()),
                None => true,
            })
            .map(|row| row.name.to_string())
    }

    /// Return the names of all providers that are both compiled in and
    /// have their credential configured (or need none).
    ///
    /// A provider whose feature was not compiled in is structurally absent
    /// from this list regardless of whether its env var happens to be set.
    /// A variable set to an empty or whitespace-only string is likewise
    /// treated as not configured, so this list stays in agreement with
    /// [`create`](Self::create): a name reported here is always one
    /// `create()` can actually construct.
    pub fn list_available_providers() -> Vec<String> {
        provider_registry()
            .iter()
            .filter(|row| match row.env_var {
                Some(var) => std::env::var(var).is_ok_and(|v| !v.trim().is_empty()),
                None => true,
            })
            .map(|row| row.name.to_string())
            .collect()
    }
}

impl Default for LlmProviderFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = LlmProviderFactory::new();
        assert_eq!(std::mem::size_of_val(&factory), 0);
    }

    #[test]
    fn test_unknown_provider_returns_error() {
        let factory = LlmProviderFactory::new();
        let result = factory.create("bogus_provider");
        assert!(result.is_err());
        if let Err(ProviderFactoryError::UnknownProvider(name)) = result {
            assert_eq!(name, "bogus_provider");
        } else {
            panic!("Expected UnknownProvider error");
        }
    }

    #[test]
    fn test_list_available_providers_returns_vec() {
        // Smoke test — just ensure it returns without panicking.
        let _ = LlmProviderFactory::list_available_providers();
    }

    #[test]
    fn provider_names_has_no_duplicate_entries() {
        let names = provider_names();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            names.len(),
            sorted.len(),
            "duplicate provider name in registry: {:?}",
            names
        );
    }

    /// The second of Task 3's two hygiene assertions (17-04): every
    /// registered provider name is an operator-facing configuration value
    /// (typed into an env var or a config file), so it must be lowercase
    /// and free of whitespace regardless of which features are compiled in.
    #[test]
    fn provider_names_are_lowercase_and_whitespace_free() {
        for name in provider_names() {
            assert_eq!(
                name,
                name.to_lowercase(),
                "provider name {name:?} must be lowercase"
            );
            assert!(
                !name.chars().any(|c| c.is_whitespace()),
                "provider name {name:?} must contain no whitespace"
            );
        }
    }

    #[test]
    fn list_available_providers_only_contains_names_from_the_registry() {
        let registry_names: std::collections::HashSet<&str> =
            provider_names().into_iter().collect();
        for name in LlmProviderFactory::list_available_providers() {
            assert!(
                registry_names.contains(name.as_str()),
                "list_available_providers returned {name}, which is not in the compiled-in \
                 registry {registry_names:?} — a compiled-out provider must never be reported \
                 as available"
            );
        }
    }

    // ── D-10 regression coverage: kimi-only build ──
    //
    // Exercised under `cargo test -p paladin-llm --no-default-features
    // --features kimi provider_factory`. Gated so it does not run (and
    // therefore does not assert an exact single-entry list) under the
    // default `openai` + `mock` feature set.
    #[cfg(all(
        feature = "kimi",
        not(feature = "openai"),
        not(feature = "anthropic"),
        not(feature = "deepseek"),
        not(feature = "qwen"),
        not(feature = "grok"),
        not(feature = "ollama")
    ))]
    mod kimi_only_build {
        use super::*;

        #[test]
        fn provider_names_returns_exactly_kimi() {
            assert_eq!(provider_names(), vec!["kimi"]);
        }

        #[test]
        fn create_bogus_provider_error_message_lists_kimi() {
            let factory = LlmProviderFactory::new();
            let result = factory.create("bogus_provider");
            match result {
                Err(ProviderFactoryError::UnknownProvider(name)) => {
                    assert_eq!(name, "bogus_provider");
                }
                Err(other) => panic!("expected UnknownProvider, got Err({other})"),
                Ok(_) => panic!("expected UnknownProvider, got Ok(_)"),
            }

            let message =
                ProviderFactoryError::UnknownProvider("bogus_provider".to_string()).to_string();
            assert!(
                message.contains("kimi"),
                "UnknownProvider message must list the compiled-in providers: {message}"
            );
        }

        #[test]
        fn create_normalizes_case_for_a_registered_provider_name() {
            let factory = LlmProviderFactory::new();
            let upper = factory.create("KIMI");
            let lower = factory.create("kimi");

            // Neither call should fail with UnknownProvider — the row is
            // found regardless of case. (It may still fail with
            // ConfigurationMissing if MOONSHOT_API_KEY is unset in this
            // test process, which is expected and fine here.)
            assert!(!matches!(
                upper,
                Err(ProviderFactoryError::UnknownProvider(_))
            ));
            assert!(!matches!(
                lower,
                Err(ProviderFactoryError::UnknownProvider(_))
            ));
        }

        #[test]
        fn build_with_zero_providers_is_not_exercised_here() {
            // Documents intent: the zero-provider empty-table case
            // (`cargo build -p paladin-llm --no-default-features`, no
            // provider feature at all) is covered by the acceptance
            // criterion's build command, not by an in-crate test — there
            // is no way to compile this test module without at least one
            // provider feature enabled.
        }
    }

    // ── D-12 regression coverage: Ollama resolves with no credential ──
    //
    // The first (and only) registry row whose `env_var` is `None`.
    // `get_default_provider()`/`list_available_providers()` must treat a
    // `None` credential as "available" without any environment variable
    // being set.
    #[cfg(feature = "ollama")]
    mod ollama_requires_no_credential {
        use super::*;

        #[test]
        fn create_resolves_with_no_credential_in_environment() {
            let factory = LlmProviderFactory::new();
            let result = factory.create("ollama");
            assert!(
                result.is_ok(),
                "ollama must resolve with no credential env var set: {:?}",
                result.err()
            );
        }

        #[test]
        fn list_available_providers_includes_ollama_whenever_compiled_in() {
            assert!(
                LlmProviderFactory::list_available_providers()
                    .iter()
                    .any(|name| name == "ollama"),
                "a compiled-in, credential-free ollama row must always report as available"
            );
        }
    }

    // ── D-10 regression coverage: the four-new-preset build (17-03) ──
    //
    // Exercised under `cargo test -p paladin-llm --no-default-features
    // --features kimi,qwen,grok,ollama`. Proves table declaration order is
    // preserved end to end and that the credential-free `ollama` row lands
    // last, never pre-empting a credentialed row.
    //
    // Gate widened in plan 17-04 (Rule 1 auto-fix, mirroring plan 17-03's
    // own precedent for this exact gate): `not(feature = "openai-compatible")`
    // added so this module's exact five-row-free assertion does not silently
    // break under the plan's own combined verification command
    // (`--features kimi,qwen,grok,ollama,openai-compatible`), which now adds
    // a fifth row. See `five_new_preset_build` below for that combined case.
    #[cfg(all(
        feature = "kimi",
        feature = "qwen",
        feature = "grok",
        feature = "ollama",
        not(feature = "openai"),
        not(feature = "anthropic"),
        not(feature = "deepseek"),
        not(feature = "openai-compatible")
    ))]
    mod four_new_preset_build {
        use super::*;

        #[test]
        fn provider_names_returns_exactly_kimi_qwen_grok_ollama_in_table_order() {
            assert_eq!(provider_names(), vec!["kimi", "qwen", "grok", "ollama"]);
        }
    }

    // ── D-10 regression coverage: the five-new-preset build (17-04) ──
    //
    // Exercised under `cargo test -p paladin-llm --no-default-features
    // --features kimi,qwen,grok,ollama,openai-compatible`. Proves the
    // generic `openai-compatible` row lands after every curated preset but
    // BEFORE the credential-free `ollama` row — see the placement comment on
    // `build_provider_registry` for why that specific position is required
    // for `get_default_provider()` to ever be able to select it.
    //
    // Gate widened in plan 17-05 (Rule 1 auto-fix, mirroring plan 17-04's own
    // precedent for this exact gate): `not(feature = "gemini")` added so this
    // module's exact five-row assertion does not silently break under plan
    // 17-05's own combined verification command (`--features
    // kimi,qwen,grok,ollama,openai-compatible,gemini`), which now adds a
    // sixth row. See `six_new_preset_build` below for that combined case.
    #[cfg(all(
        feature = "kimi",
        feature = "qwen",
        feature = "grok",
        feature = "ollama",
        feature = "openai-compatible",
        not(feature = "openai"),
        not(feature = "anthropic"),
        not(feature = "deepseek"),
        not(feature = "gemini")
    ))]
    mod five_new_preset_build {
        use super::*;

        #[test]
        fn provider_names_returns_exactly_kimi_qwen_grok_openai_compatible_ollama_in_table_order() {
            assert_eq!(
                provider_names(),
                vec!["kimi", "qwen", "grok", "openai-compatible", "ollama"]
            );
        }
    }

    // ── D-10 regression coverage: the six-preset build (17-05) ──
    //
    // Exercised under `cargo test -p paladin-llm --no-default-features
    // --features kimi,qwen,grok,ollama,openai-compatible,gemini` — plan
    // 17-05's own combined verification command. Proves Gemini's row lands
    // alongside the other curated (named-vendor) presets, before the
    // generic `openai-compatible` row and Ollama's credential-free row —
    // see the placement comment on `build_provider_registry` for why.
    #[cfg(all(
        feature = "kimi",
        feature = "qwen",
        feature = "grok",
        feature = "ollama",
        feature = "openai-compatible",
        feature = "gemini",
        not(feature = "openai"),
        not(feature = "anthropic"),
        not(feature = "deepseek")
    ))]
    mod six_preset_build {
        use super::*;

        #[test]
        fn provider_names_returns_exactly_kimi_qwen_grok_gemini_openai_compatible_ollama_in_table_order()
         {
            assert_eq!(
                provider_names(),
                vec![
                    "kimi",
                    "qwen",
                    "grok",
                    "gemini",
                    "openai-compatible",
                    "ollama"
                ]
            );
        }
    }

    // ── The registry-wide provider-name round-trip invariant (Task 3, 17-04) ──
    //
    // Companion test to this plan's `<assumption_delta_decision>`: every
    // adapter reachable through the registry must resolve to its own
    // registered name, for every compiled-in provider. It goes red the
    // moment a future phase reintroduces the singular
    // one-adapter-type-one-name assumption — most likely by adding a second
    // generic row that reuses the `"openai-compatible"` literal under a
    // different table name.
    //
    // Gated on `feature = "ollama"` specifically: Ollama's registry row
    // requires no credential (`env_var: None`) and is therefore guaranteed
    // to construct regardless of which secrets happen to be present in the
    // test process. That guarantee is what makes the "at least one row was
    // exercised" assertion below safe to run in an environment with zero
    // configured credentials — without it, a CI run with no API keys set
    // could exercise zero rows and pass vacuously, which is the exact
    // failure mode this test exists to prevent. Exercised in this plan's own
    // verification command: `cargo test -p paladin-llm --no-default-features
    // --features ollama,openai-compatible provider_name_round_trips`.
    #[cfg(feature = "ollama")]
    mod provider_name_round_trip {
        use super::*;

        #[test]
        fn provider_name_round_trips_for_every_registry_row() {
            let mut exercised = 0usize;

            for row in provider_registry() {
                // Uses the same non-blank check `get_default_provider()` and
                // `list_available_providers()` use (CR-01): an env var set
                // to an empty (or whitespace-only) string is *set* but is
                // not a usable credential — every preset's own `validate()`
                // rejects an empty key. Treating it as "present" here would
                // panic on `construct()`'s resulting `AdapterCreationFailed`,
                // which is a false alarm about this test's own invariant
                // (every adapter that *does* construct round-trips its
                // name), not a real violation of it.
                let credential_present = match row.env_var {
                    Some(var) => std::env::var(var).is_ok_and(|v| !v.trim().is_empty()),
                    None => true,
                };

                if !credential_present {
                    eprintln!(
                        "skipping provider_name round-trip for {:?} — its credential env var \
                         is not set (or is empty) in this test process",
                        row.name
                    );
                    continue;
                }

                match (row.construct)() {
                    Ok(adapter) => {
                        exercised += 1;
                        assert_eq!(
                            adapter.get_provider_name(),
                            row.name,
                            "row {:?} constructed an adapter whose get_provider_name() did not \
                             round-trip to its own registered name",
                            row.name
                        );
                    }
                    Err(e) => {
                        panic!(
                            "row {:?} reported its credential as present but construct() failed: {e}",
                            row.name
                        );
                    }
                }
            }

            assert!(
                exercised > 0,
                "no registry row was exercised — this module is gated on `feature = \"ollama\"` \
                 specifically so its credential-free row guarantees at least one construction \
                 regardless of environment; zero exercised rows means that guarantee broke"
            );
        }
    }

    /// `create()` is safe to call concurrently: N concurrent calls each
    /// resolve the registry independently and return distinct results with
    /// no shared mutable state. Exercised against an unknown provider name
    /// rather than a credentialed one — mutating `MOONSHOT_API_KEY` via
    /// `std::env::set_var` is `unsafe` under Rust 2024 and this crate
    /// denies `unsafe_code`; the credentialed 20-concurrent-`create("kimi")`
    /// variant is deferred to the workspace-level test target (plan 17-07
    /// owns env-var-dependent tests). This test still proves the concurrency
    /// property D-10 depends on: the lazily-initialized `OnceLock` registry
    /// resolves safely under concurrent readers.
    #[tokio::test]
    async fn create_is_safe_to_call_concurrently() {
        let factory = Arc::new(LlmProviderFactory::new());
        let mut handles = Vec::new();

        for _ in 0..20 {
            let factory = Arc::clone(&factory);
            handles.push(tokio::spawn(async move {
                factory.create("bogus-concurrent-provider")
            }));
        }

        for handle in handles {
            let result = handle.await.expect("task must not panic");
            assert!(matches!(
                result,
                Err(ProviderFactoryError::UnknownProvider(_))
            ));
        }
    }
}
