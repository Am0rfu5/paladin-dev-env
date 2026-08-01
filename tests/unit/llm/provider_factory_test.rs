// tests/unit/llm/provider_factory_test.rs
//
// Unit tests for LLM provider factory

use paladin_llm::provider_factory::{LlmProviderFactory, ProviderFactoryError};
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
