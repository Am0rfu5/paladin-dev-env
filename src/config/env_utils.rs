//! Shared utilities for environment-variable-based configuration overrides.
//!
//! Provides the [`EnvOverridable`] trait and the [`read_env`] helper so every
//! configuration struct can apply env-var overrides through a single, consistent
//! pattern instead of duplicating the read-parse-assign logic.

/// Implemented by configuration structs that support environment variable overrides.
///
/// Each implementing struct is responsible for reading its own set of
/// environment variables and overriding its fields accordingly.  The
/// `Settings::get_*_config()` methods call this after cloning the base config
/// from the YAML file, so the returned value always reflects both the file and
/// any env overrides.
///
/// # Example
///
/// ```rust
/// use paladin::config::env_utils::{EnvOverridable, read_env};
///
/// #[derive(Default)]
/// struct MyConfig {
///     host: String,
///     port: u16,
/// }
///
/// impl EnvOverridable for MyConfig {
///     fn apply_env_overrides(&mut self) {
///         if let Some(v) = read_env::<String>("MY_HOST") { self.host = v; }
///         if let Some(v) = read_env::<u16>("MY_PORT")   { self.port = v; }
///     }
/// }
/// ```
pub trait EnvOverridable {
    /// Read environment variables and override the corresponding fields in place.
    fn apply_env_overrides(&mut self);
}

/// Read an environment variable and parse it to type `T`.
///
/// Returns `Some(value)` when the variable is set **and** parseable as `T`.
/// Returns `None` when the variable is absent or cannot be parsed (the error is
/// silently swallowed — callers should fall back to the field's existing value).
///
/// # Examples
///
/// ```rust
/// use paladin::config::env_utils::read_env;
///
/// // Will return None if APP_PORT is not set or not a valid u16.
/// let port: Option<u16> = read_env("APP_PORT");
/// ```
pub fn read_env<T: std::str::FromStr>(var_name: &str) -> Option<T> {
    std::env::var(var_name).ok()?.parse::<T>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    // ── read_env::<String> ───────────────────────────────────────────────────

    #[test]
    #[serial]
    fn test_read_env_string_set() {
        unsafe { env::set_var("_TEST_STR", "hello") };
        let result: Option<String> = read_env("_TEST_STR");
        assert_eq!(result, Some("hello".to_string()));
        unsafe { env::remove_var("_TEST_STR") };
    }

    #[test]
    #[serial]
    fn test_read_env_string_unset() {
        unsafe { env::remove_var("_TEST_STR_UNSET") };
        let result: Option<String> = read_env("_TEST_STR_UNSET");
        assert_eq!(result, None);
    }

    // ── read_env::<u16> ──────────────────────────────────────────────────────

    #[test]
    #[serial]
    fn test_read_env_u16_valid() {
        unsafe { env::set_var("_TEST_U16", "8080") };
        let result: Option<u16> = read_env("_TEST_U16");
        assert_eq!(result, Some(8080u16));
        unsafe { env::remove_var("_TEST_U16") };
    }

    #[test]
    #[serial]
    fn test_read_env_u16_invalid_value() {
        unsafe { env::set_var("_TEST_U16_BAD", "not_a_number") };
        let result: Option<u16> = read_env("_TEST_U16_BAD");
        assert_eq!(result, None);
        unsafe { env::remove_var("_TEST_U16_BAD") };
    }

    #[test]
    #[serial]
    fn test_read_env_u16_overflow() {
        unsafe { env::set_var("_TEST_U16_OVF", "99999") }; // > u16::MAX
        let result: Option<u16> = read_env("_TEST_U16_OVF");
        assert_eq!(result, None);
        unsafe { env::remove_var("_TEST_U16_OVF") };
    }

    // ── read_env::<u64> ──────────────────────────────────────────────────────

    #[test]
    #[serial]
    fn test_read_env_u64_valid() {
        unsafe { env::set_var("_TEST_U64", "9999999999") };
        let result: Option<u64> = read_env("_TEST_U64");
        assert_eq!(result, Some(9_999_999_999u64));
        unsafe { env::remove_var("_TEST_U64") };
    }

    // ── read_env::<bool> ─────────────────────────────────────────────────────

    #[test]
    #[serial]
    fn test_read_env_bool_true() {
        unsafe { env::set_var("_TEST_BOOL", "true") };
        let result: Option<bool> = read_env("_TEST_BOOL");
        assert_eq!(result, Some(true));
        unsafe { env::remove_var("_TEST_BOOL") };
    }

    #[test]
    #[serial]
    fn test_read_env_bool_false() {
        unsafe { env::set_var("_TEST_BOOL_F", "false") };
        let result: Option<bool> = read_env("_TEST_BOOL_F");
        assert_eq!(result, Some(false));
        unsafe { env::remove_var("_TEST_BOOL_F") };
    }

    #[test]
    #[serial]
    fn test_read_env_bool_invalid() {
        unsafe { env::set_var("_TEST_BOOL_BAD", "yes") }; // "yes" is not a valid bool
        let result: Option<bool> = read_env("_TEST_BOOL_BAD");
        assert_eq!(result, None);
        unsafe { env::remove_var("_TEST_BOOL_BAD") };
    }

    // ── read_env for Option<String> (simulated via unset var) ────────────────

    #[test]
    #[serial]
    fn test_read_env_unset_returns_none() {
        unsafe { env::remove_var("_TEST_NEVER_SET_XYZ") };
        let result: Option<String> = read_env("_TEST_NEVER_SET_XYZ");
        assert_eq!(result, None);
    }

    // ── EnvOverridable trait integration ─────────────────────────────────────

    #[test]
    #[serial]
    fn test_env_overridable_trait_applies_override() {
        #[derive(Default)]
        struct TestConfig {
            host: String,
            port: u16,
        }

        impl EnvOverridable for TestConfig {
            fn apply_env_overrides(&mut self) {
                if let Some(v) = read_env::<String>("_TENV_HOST") {
                    self.host = v;
                }
                if let Some(v) = read_env::<u16>("_TENV_PORT") {
                    self.port = v;
                }
            }
        }

        unsafe {
            env::set_var("_TENV_HOST", "example.com");
            env::set_var("_TENV_PORT", "9090");
        }

        let mut cfg = TestConfig {
            host: "localhost".to_string(),
            port: 8080,
        };
        cfg.apply_env_overrides();

        assert_eq!(cfg.host, "example.com");
        assert_eq!(cfg.port, 9090);

        unsafe {
            env::remove_var("_TENV_HOST");
            env::remove_var("_TENV_PORT");
        }
    }

    #[test]
    #[serial]
    fn test_env_overridable_trait_no_override_when_unset() {
        #[derive(Default)]
        struct TestConfig {
            value: String,
        }

        impl EnvOverridable for TestConfig {
            fn apply_env_overrides(&mut self) {
                if let Some(v) = read_env::<String>("_TENV_ABSENT_VAR") {
                    self.value = v;
                }
            }
        }

        unsafe { env::remove_var("_TENV_ABSENT_VAR") };

        let mut cfg = TestConfig {
            value: "original".to_string(),
        };
        cfg.apply_env_overrides();

        assert_eq!(cfg.value, "original");
    }
}
