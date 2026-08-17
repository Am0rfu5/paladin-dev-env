// tests/unit/llm/provider_factory_test.rs
//
// Unit tests for LLM provider factory

use paladin_llm::provider_factory::{LlmProviderFactory, ProviderFactoryError, provider_names};
use std::env;
use std::sync::Mutex;

/// Serializes access to the three provider API-key environment variables
/// (`OPENAI_API_KEY` / `DEEPSEEK_API_KEY` / `ANTHROPIC_API_KEY`) across every
/// test in this file that reads or mutates them. `cargo test` runs tests in
/// parallel threads within one process by default, and every test below
/// observes or mutates this same process-wide state (T-02-23).
static PROVIDER_ENV_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard that clears the three provider API-key vars for its lifetime
/// and restores each to its prior value (present-with-value or absent) on
/// drop — including on panic/unwind, so a failing assertion never leaks
/// state into a sibling test. Holds `PROVIDER_ENV_LOCK` for its entire
/// lifetime so no other test in this file can observe or mutate these vars
/// concurrently.
///
/// This also neutralises ambient values some sandboxes/CI harnesses predefine
/// (even an empty string counts as "set" to `std::env::var`, which would
/// otherwise make `ConfigurationMissing` assertions below fail against a
/// polluted environment rather than the clean one the tests intend).
struct CleanProviderEnv<'a> {
    _lock: std::sync::MutexGuard<'a, ()>,
    openai_key: Option<String>,
    deepseek_key: Option<String>,
    anthropic_key: Option<String>,
}

impl CleanProviderEnv<'_> {
    fn acquire() -> Self {
        let lock = PROVIDER_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let openai_key = env::var("OPENAI_API_KEY").ok();
        let deepseek_key = env::var("DEEPSEEK_API_KEY").ok();
        let anthropic_key = env::var("ANTHROPIC_API_KEY").ok();

        // SAFETY: PROVIDER_ENV_LOCK is held for this guard's lifetime (via
        // `_lock`), so no other test in this binary can observe or mutate
        // these process-wide vars while we hold it (T-02-23).
        unsafe {
            env::remove_var("OPENAI_API_KEY");
        }
        // SAFETY: see above.
        unsafe {
            env::remove_var("DEEPSEEK_API_KEY");
        }
        // SAFETY: see above.
        unsafe {
            env::remove_var("ANTHROPIC_API_KEY");
        }

        Self {
            _lock: lock,
            openai_key,
            deepseek_key,
            anthropic_key,
        }
    }
}

impl Drop for CleanProviderEnv<'_> {
    fn drop(&mut self) {
        // SAFETY: `_lock` (a sibling field) is still held for the duration
        // of this `Drop::drop` call — Rust drops struct fields after the
        // `Drop` impl runs — so this restore cannot race a sibling test.
        unsafe {
            match self.openai_key.as_deref() {
                Some(v) => env::set_var("OPENAI_API_KEY", v),
                None => env::remove_var("OPENAI_API_KEY"),
            }
        }
        // SAFETY: see above.
        unsafe {
            match self.deepseek_key.as_deref() {
                Some(v) => env::set_var("DEEPSEEK_API_KEY", v),
                None => env::remove_var("DEEPSEEK_API_KEY"),
            }
        }
        // SAFETY: see above.
        unsafe {
            match self.anthropic_key.as_deref() {
                Some(v) => env::set_var("ANTHROPIC_API_KEY", v),
                None => env::remove_var("ANTHROPIC_API_KEY"),
            }
        }
    }
}

#[test]
fn test_factory_provider_selection() {
    let _env = CleanProviderEnv::acquire();
    let factory = LlmProviderFactory::new();

    // Test unknown provider error
    let result = factory.create("unknown_provider");
    assert!(result.is_err());
    // `Arc<dyn LlmPort>` (the Ok type) is not `Debug`, so `Result::unwrap_err`
    // (which requires `T: Debug`) cannot be used here — go via `err()`
    // (`Option<E>`) then `Option::unwrap`, which has no such bound.
    assert!(matches!(
        result.err().unwrap(),
        ProviderFactoryError::UnknownProvider(_)
    ));

    // Test that provider names are recognized (even if config is missing)
    let result_openai = factory.create("openai");
    let result_deepseek = factory.create("deepseek");
    let result_anthropic = factory.create("anthropic");

    // All should fail with ConfigurationMissing (not UnknownProvider)
    assert!(matches!(
        result_openai.err().unwrap(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
    assert!(matches!(
        result_deepseek.err().unwrap(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
    assert!(matches!(
        result_anthropic.err().unwrap(),
        ProviderFactoryError::ConfigurationMissing(_)
    ));
}

#[test]
fn test_factory_config_validation() {
    let _env = CleanProviderEnv::acquire();
    // Test that factory properly validates configurations
    let factory = LlmProviderFactory::new();

    // Test with missing environment variables
    let result = factory.create("deepseek");
    assert!(result.is_err());

    let error = result.err().unwrap();
    match error {
        ProviderFactoryError::ConfigurationMissing(msg) => {
            assert!(msg.contains("DEEPSEEK_API_KEY"));
        }
        _ => panic!("Expected ConfigurationMissing error"),
    }
}

#[test]
fn test_factory_case_insensitive() {
    let _env = CleanProviderEnv::acquire();
    let factory = LlmProviderFactory::new();

    // All case variations should be recognized
    let result1 = factory.create("OpenAI");
    let result2 = factory.create("OPENAI");
    let result3 = factory.create("openai");

    // All should fail with ConfigurationMissing (not UnknownProvider)
    for result in [result1, result2, result3] {
        assert!(matches!(
            result.err().unwrap(),
            ProviderFactoryError::ConfigurationMissing(_)
        ));
    }
}

#[test]
fn test_factory_error_messages() {
    let factory = LlmProviderFactory::new();

    // Test unknown provider error message
    let result = factory.create("invalid_provider");
    assert!(result.is_err());

    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("Unknown provider"));
    assert!(error_msg.contains("invalid_provider"));
    assert!(error_msg.contains("Supported providers"));
}

#[test]
fn test_get_default_provider() {
    // CleanProviderEnv::acquire() takes PROVIDER_ENV_LOCK, saves the current
    // value of all three provider-key vars, clears them, and restores the
    // saved values on drop (including on panic) — see its doc comment.
    let _env = CleanProviderEnv::acquire();

    // No providers configured
    assert_eq!(LlmProviderFactory::get_default_provider(), None);

    // Only Anthropic configured - should be selected
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("ANTHROPIC_API_KEY", "test-key");
    }
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("anthropic".to_string())
    );

    // Add DeepSeek - should be selected over Anthropic
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("DEEPSEEK_API_KEY", "test-key");
    }
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("deepseek".to_string())
    );

    // Add OpenAI - should be selected as highest priority
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }
    assert_eq!(
        LlmProviderFactory::get_default_provider(),
        Some("openai".to_string())
    );

    // `_env`'s Drop impl restores the pre-test environment when it goes out
    // of scope at the end of this function.
}

#[test]
fn test_list_available_providers() {
    // See test_get_default_provider's comment: this guard clears the three
    // provider-key vars for the duration of this test and restores their
    // pre-test values (including on panic) when it is dropped.
    let _env = CleanProviderEnv::acquire();

    // No providers configured
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 0);

    // Add one provider
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("DEEPSEEK_API_KEY", "test-key");
    }
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 1);
    assert!(providers.contains(&"deepseek".to_string()));

    // Add all providers
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }
    // SAFETY: test-owned provider-key var, restored at the end of this test.
    unsafe {
        env::set_var("ANTHROPIC_API_KEY", "test-key");
    }
    let providers = LlmProviderFactory::list_available_providers();
    assert_eq!(providers.len(), 3);
    assert!(providers.contains(&"openai".to_string()));
    assert!(providers.contains(&"deepseek".to_string()));
    assert!(providers.contains(&"anthropic".to_string()));

    // `_env`'s Drop impl restores the pre-test environment when it goes out
    // of scope at the end of this function.
}

/// D-11 (amended 2026-08-17, option-b) / ADR-0046 regression guard: the
/// facade's `llm-*` feature flags are now real gates on `paladin-llm`'s own
/// features rather than inert stubs, but `default = ["llm-openai",
/// "llm-anthropic", "llm-deepseek"]` must still compile — and therefore
/// still resolve — exactly the same three providers a default build did
/// before this plan. A `cargo tree` assertion alone proves the feature
/// graph is wired; this test proves the runtime consequence (`create(..)`
/// still finding a registered row) actually holds under whatever features
/// this test binary was compiled with.
#[test]
fn default_features_still_resolve_openai_anthropic_and_deepseek() {
    let _env = CleanProviderEnv::acquire();
    let factory = LlmProviderFactory::new();

    for name in ["openai", "anthropic", "deepseek"] {
        // `Arc<dyn LlmPort>` (the `Ok` type) is not `Debug`, so the whole
        // `Result` cannot be formatted with `{:?}` — match on a reference
        // and report only the `Err` variant when the assertion trips.
        let result = factory.create(name);
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "provider {name:?} must still resolve under default features (D-11/ADR-0046); got {:?}",
            result.as_ref().err()
        );
    }
}

/// Serializes access to the seven new-provider credential/configuration
/// environment variables the two D-10 regression tests below read or mutate
/// (`MOONSHOT_API_KEY`/`DASHSCOPE_API_KEY`/`XAI_API_KEY`/`GEMINI_API_KEY`/
/// `OPENAI_COMPATIBLE_API_KEY`/`OPENAI_COMPATIBLE_BASE_URL`/
/// `OPENAI_COMPATIBLE_MODEL`) — a disjoint variable set from
/// [`PROVIDER_ENV_LOCK`], which owns the three shipped providers' vars, so a
/// separate lock avoids serializing tests that touch unrelated variables.
static NEW_PROVIDER_ENV_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard mirroring [`CleanProviderEnv`] for the seven new-provider
/// credential/configuration variables: clears all seven for its lifetime and
/// restores each to its prior value (present-with-value or absent) on drop,
/// including on panic/unwind, so a failing assertion never leaks state into a
/// sibling test.
struct CleanNewProviderEnv<'a> {
    _lock: std::sync::MutexGuard<'a, ()>,
    moonshot_key: Option<String>,
    dashscope_key: Option<String>,
    xai_key: Option<String>,
    gemini_key: Option<String>,
    openai_compatible_key: Option<String>,
    openai_compatible_base_url: Option<String>,
    openai_compatible_model: Option<String>,
}

/// The seven variable names [`CleanNewProviderEnv`] owns, paired with the
/// field that saves each one's pre-test value.
const NEW_PROVIDER_ENV_VARS: [&str; 7] = [
    "MOONSHOT_API_KEY",
    "DASHSCOPE_API_KEY",
    "XAI_API_KEY",
    "GEMINI_API_KEY",
    "OPENAI_COMPATIBLE_API_KEY",
    "OPENAI_COMPATIBLE_BASE_URL",
    "OPENAI_COMPATIBLE_MODEL",
];

impl CleanNewProviderEnv<'_> {
    fn acquire() -> Self {
        let lock = NEW_PROVIDER_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let moonshot_key = env::var("MOONSHOT_API_KEY").ok();
        let dashscope_key = env::var("DASHSCOPE_API_KEY").ok();
        let xai_key = env::var("XAI_API_KEY").ok();
        let gemini_key = env::var("GEMINI_API_KEY").ok();
        let openai_compatible_key = env::var("OPENAI_COMPATIBLE_API_KEY").ok();
        let openai_compatible_base_url = env::var("OPENAI_COMPATIBLE_BASE_URL").ok();
        let openai_compatible_model = env::var("OPENAI_COMPATIBLE_MODEL").ok();

        for var in NEW_PROVIDER_ENV_VARS {
            // SAFETY: NEW_PROVIDER_ENV_LOCK is held for this guard's lifetime
            // (via `_lock`), so no other test in this binary can observe or
            // mutate these process-wide vars while we hold it.
            unsafe {
                env::remove_var(var);
            }
        }

        Self {
            _lock: lock,
            moonshot_key,
            dashscope_key,
            xai_key,
            gemini_key,
            openai_compatible_key,
            openai_compatible_base_url,
            openai_compatible_model,
        }
    }

    /// Set one of the seven owned variables for the remainder of this
    /// guard's lifetime.
    fn set(&self, var: &str, value: &str) {
        debug_assert!(
            NEW_PROVIDER_ENV_VARS.contains(&var),
            "{var} is not one of the variables this guard owns and restores on drop"
        );
        // SAFETY: `_lock` is held for this guard's entire lifetime, so no
        // other test in this binary can observe or mutate `var` concurrently.
        unsafe {
            env::set_var(var, value);
        }
    }
}

impl Drop for CleanNewProviderEnv<'_> {
    fn drop(&mut self) {
        // SAFETY: `_lock` (a sibling field) is still held for the duration
        // of this `Drop::drop` call — Rust drops struct fields after the
        // `Drop` impl runs — so this restore cannot race a sibling test.
        for (var, value) in [
            ("MOONSHOT_API_KEY", &self.moonshot_key),
            ("DASHSCOPE_API_KEY", &self.dashscope_key),
            ("XAI_API_KEY", &self.xai_key),
            ("GEMINI_API_KEY", &self.gemini_key),
            ("OPENAI_COMPATIBLE_API_KEY", &self.openai_compatible_key),
            (
                "OPENAI_COMPATIBLE_BASE_URL",
                &self.openai_compatible_base_url,
            ),
            ("OPENAI_COMPATIBLE_MODEL", &self.openai_compatible_model),
        ] {
            unsafe {
                match value.as_deref() {
                    Some(v) => env::set_var(var, v),
                    None => env::remove_var(var),
                }
            }
        }
    }
}

/// D-10 regression guard (RESEARCH.md Pitfall 1/4, this plan's own remit): a
/// provider whose feature is not compiled into this test binary must be
/// absent from `list_available_providers()` even when its credential env var
/// is set. `paladin-llm` denies `unsafe_code`, so this env-mutating variant
/// of the regression lives in this workspace-level test crate instead (Task
/// 1's own action text).
///
/// Written against whichever of the four credentialed new-provider
/// candidates this workspace's `unit` test target's *currently-active*
/// feature set genuinely does not compile in — determined at runtime via
/// [`provider_names`], never assumed. Under this root crate's default
/// feature set (`default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`,
/// D-11) all four are compiled out, but the lookup stays correct under any
/// combination this test binary might be built with. If every candidate
/// happens to be compiled in (e.g. a `--features llm-all` run), the plan's
/// own fallback applies: assert the structural invariant that makes the
/// defect impossible — every reported-available name is in the compiled-in
/// registry — instead of the env-mutating single-name form.
#[test]
fn test_compiled_out_provider_absent_from_list_available_providers() {
    let _env = CleanNewProviderEnv::acquire();
    let compiled = provider_names();

    let candidates: &[(&str, &str)] = &[
        ("kimi", "MOONSHOT_API_KEY"),
        ("qwen", "DASHSCOPE_API_KEY"),
        ("grok", "XAI_API_KEY"),
        ("gemini", "GEMINI_API_KEY"),
    ];

    let Some(&(name, env_var)) = candidates.iter().find(|(name, _)| !compiled.contains(name))
    else {
        assert!(
            LlmProviderFactory::list_available_providers()
                .iter()
                .all(|available| compiled.contains(&available.as_str())),
            "every name list_available_providers() reports must be in the compiled-in registry \
             — the env-mutating compiled-out-provider form was not expressible under this \
             build's feature set, which compiles every one of kimi/qwen/grok/gemini"
        );
        return;
    };

    _env.set(env_var, "test-key-set-despite-feature-being-compiled-out");

    assert!(
        !LlmProviderFactory::list_available_providers()
            .iter()
            .any(|available| available == name),
        "{name} must be absent from list_available_providers() even with {env_var} set, because \
         its feature is not compiled into this build (D-10 regression)"
    );
}

/// D-10 regression: every new provider name this build's compiled feature
/// set includes resolves through `create()` once its credential is set
/// (Ollama needs none, D-12), and a name that is not compiled in at all
/// always returns `UnknownProvider` whose message lists only the compiled-in
/// names.
#[test]
fn test_new_provider_names_resolve_through_create() {
    let _env = CleanNewProviderEnv::acquire();
    let compiled = provider_names();
    let factory = LlmProviderFactory::new();

    if compiled.contains(&"kimi") {
        _env.set("MOONSHOT_API_KEY", "test-key");
        let result = factory.create("kimi");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "kimi is compiled in and its credential is set; create() must not return \
             UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }
    if compiled.contains(&"qwen") {
        _env.set("DASHSCOPE_API_KEY", "test-key");
        let result = factory.create("qwen");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "qwen is compiled in and its credential is set; create() must not return \
             UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }
    if compiled.contains(&"grok") {
        _env.set("XAI_API_KEY", "test-key");
        let result = factory.create("grok");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "grok is compiled in and its credential is set; create() must not return \
             UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }
    if compiled.contains(&"gemini") {
        _env.set("GEMINI_API_KEY", "test-key");
        let result = factory.create("gemini");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "gemini is compiled in and its credential is set; create() must not return \
             UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }
    if compiled.contains(&"ollama") {
        // No credential env var to set — Ollama requires none (D-12).
        let result = factory.create("ollama");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "ollama is compiled in and requires no credential; create() must not return \
             UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }
    if compiled.contains(&"openai-compatible") {
        _env.set("OPENAI_COMPATIBLE_API_KEY", "test-key");
        _env.set("OPENAI_COMPATIBLE_BASE_URL", "http://localhost:8080");
        _env.set("OPENAI_COMPATIBLE_MODEL", "test-model");
        let result = factory.create("openai-compatible");
        assert!(
            !matches!(result, Err(ProviderFactoryError::UnknownProvider(_))),
            "openai-compatible is compiled in and its three required vars are set; create() \
             must not return UnknownProvider: {:?}",
            result.as_ref().err()
        );
    }

    // A name that is not compiled in — regardless of which feature set this
    // binary was built with — must always return UnknownProvider, with a
    // message that lists every compiled-in name and never the bogus one.
    let bogus_name = "definitely-not-a-registered-provider";
    let result = factory.create(bogus_name);
    match result {
        Err(ProviderFactoryError::UnknownProvider(name)) => {
            assert_eq!(name, bogus_name);
        }
        Err(other) => panic!("expected UnknownProvider, got Err({other})"),
        Ok(_) => panic!("expected UnknownProvider, got Ok(_)"),
    }

    // The message's Display impl necessarily echoes the *requested* name back
    // (`"Unknown provider: {bogus_name}. Supported providers: ..."`), so the
    // "does not list the bogus name" half of this assertion must look only at
    // the supported-providers segment, not the whole message.
    let message = ProviderFactoryError::UnknownProvider(bogus_name.to_string()).to_string();
    let supported_segment = message
        .split_once("Supported providers: ")
        .map(|(_, rest)| rest)
        .unwrap_or_else(|| {
            panic!("UnknownProvider message missing 'Supported providers: ' segment: {message}")
        });
    for name in &compiled {
        assert!(
            supported_segment.contains(*name),
            "UnknownProvider message's supported-providers segment must list every compiled-in \
             provider name; missing {name:?}: {message}"
        );
    }
    assert!(
        !supported_segment.contains(bogus_name),
        "UnknownProvider message's supported-providers segment must not list the bogus name: \
         {message}"
    );
}

#[test]
fn test_factory_zero_sized() {
    let factory = LlmProviderFactory::new();
    // Factory should be zero-sized (no runtime cost)
    assert_eq!(std::mem::size_of_val(&factory), 0);
}

#[test]
// The whole point of this test is exercising the `Default` trait impl itself
// (not just constructing the unit struct), so the "just write the struct
// literal" suggestion would defeat what is being tested.
#[allow(clippy::default_constructed_unit_structs)]
fn test_factory_default() {
    let factory = LlmProviderFactory::default();
    assert_eq!(std::mem::size_of_val(&factory), 0);
}
