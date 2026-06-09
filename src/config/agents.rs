//! Configuration for agents served by the HTTP service host (Milestone 12, Epic 2).
//!
//! The top-level `agents:` key in `config.yml` is a list of [`AgentDefinition`]s. The
//! `paladin-server` binary turns each definition into a resident agent in the
//! `paladin_web::AgentRegistry` (see the facade `infrastructure::web` builder).
//!
//! Secrets (API keys) are **never** read from these definitions — they come from the
//! `llm:` provider configuration and the corresponding environment variables.

use paladin_core::platform::container::user::UserRole;
use serde::{Deserialize, Serialize};

/// Server-wide execution timeout configuration for the HTTP service host.
///
/// Maps onto `paladin_web::TimeoutPolicy`. Absent fields fall back to the defaults
/// (300s default, 600s max).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTimeoutsConfig {
    /// Default execution timeout (seconds) when neither request nor agent specifies one.
    #[serde(default = "default_timeout_seconds")]
    pub default_seconds: u64,
    /// Maximum execution timeout (seconds); per-request/agent values are clamped to it.
    #[serde(default = "default_max_timeout_seconds")]
    pub max_seconds: u64,
}

fn default_timeout_seconds() -> u64 {
    300
}

fn default_max_timeout_seconds() -> u64 {
    600
}

impl Default for AgentTimeoutsConfig {
    fn default() -> Self {
        Self {
            default_seconds: default_timeout_seconds(),
            max_seconds: default_max_timeout_seconds(),
        }
    }
}

/// Per-client rate-limit settings for the HTTP service host (off by default).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Whether the rate limiter is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Sustained requests per second allowed per client IP.
    #[serde(default = "default_rate_per_second")]
    pub per_second: u64,
    /// Burst capacity per client IP.
    #[serde(default = "default_rate_burst")]
    pub burst: u32,
}

fn default_rate_per_second() -> u64 {
    10
}

fn default_rate_burst() -> u32 {
    20
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            per_second: default_rate_per_second(),
            burst: default_rate_burst(),
        }
    }
}

/// One static API key mapped to a principal (`name` + `role`).
///
/// The `key` should come from an environment variable / secret in practice, not be
/// committed in plaintext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// The secret key value presented in the `X-API-Key` header.
    pub key: String,
    /// A stable identifier for the caller (used as the principal id; appears in logs).
    pub name: String,
    /// The role granted to requests authenticated with this key.
    pub role: UserRole,
}

/// JWT authentication settings for the agent API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtAuthConfig {
    /// Whether to accept `Authorization: Bearer` tokens via the wired `AuthPort`.
    #[serde(default)]
    pub enabled: bool,
}

/// Authentication configuration for the agent API (maps onto `paladin_web::AgentAuthConfig`).
///
/// `enabled` defaults to **true** (secure by default); the server fails closed when auth is
/// enabled but no credential source (API keys or JWT) is configured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Whether authentication is enforced on the agent routes.
    #[serde(default = "default_auth_enabled")]
    pub enabled: bool,
    /// Static API keys accepted via the `X-API-Key` header.
    #[serde(default)]
    pub api_keys: Vec<ApiKeyConfig>,
    /// JWT bearer-token settings.
    #[serde(default)]
    pub jwt: JwtAuthConfig,
}

fn default_auth_enabled() -> bool {
    true
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: default_auth_enabled(),
            api_keys: Vec::new(),
            jwt: JwtAuthConfig::default(),
        }
    }
}

/// Cross-cutting HTTP layer configuration (CORS, body limit, global timeout, rate limit, auth).
///
/// Maps onto `paladin_web::HttpLayersConfig` (+ `AgentAuthConfig`); absent fields use safe
/// defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebHttpConfig {
    /// Allowed CORS origins; empty ⇒ permissive (suitable for local dev).
    #[serde(default)]
    pub cors_allow_origins: Vec<String>,
    /// Maximum request body size in bytes.
    #[serde(default = "default_body_limit_bytes")]
    pub body_limit_bytes: usize,
    /// Global request timeout (seconds) for non-streaming routes; `0` disables it.
    #[serde(default)]
    pub global_timeout_seconds: u64,
    /// Rate-limit settings.
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    /// Authentication settings (enabled by default).
    #[serde(default)]
    pub auth: AuthConfig,
}

fn default_body_limit_bytes() -> usize {
    1024 * 1024
}

impl Default for WebHttpConfig {
    fn default() -> Self {
        Self {
            cors_allow_origins: Vec::new(),
            body_limit_bytes: default_body_limit_bytes(),
            global_timeout_seconds: 0,
            rate_limit: RateLimitConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

/// Declarative definition of one agent to load into the HTTP service host.
///
/// `id`, `model`, and `system_prompt` are required; everything else is optional and
/// falls back to a provider/builder default. Optional fields use `#[serde(default)]`
/// so new fields can be added without breaking existing configs.
///
/// # Example (YAML)
///
/// ```yaml
/// agents:
///   - id: "researcher"
///     provider: "openai"      # optional; defaults to llm.default_provider
///     model: "gpt-4"
///     system_prompt: "You research topics thoroughly."
///     temperature: 0.7
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentDefinition {
    /// Unique registry id — the `{id}` path segment in `/agents/{id}/…`.
    pub id: String,

    /// LLM model identifier (e.g. `"gpt-4"`).
    pub model: String,

    /// System prompt defining the agent's behavior.
    pub system_prompt: String,

    /// Provider name (e.g. `"openai"`, `"anthropic"`, `"deepseek"`).
    ///
    /// When absent, the server falls back to `llm.default_provider`.
    #[serde(default)]
    pub provider: Option<String>,

    /// Response randomness (`0.0`–`1.0`). When absent, the builder default applies.
    #[serde(default)]
    pub temperature: Option<f32>,

    /// Maximum reasoning loops. When absent, the builder default applies.
    #[serde(default)]
    pub max_loops: Option<u32>,

    /// Tokens that signal the agent to stop processing.
    #[serde(default)]
    pub stop_words: Vec<String>,

    /// Per-agent execution timeout (seconds). When absent, the server default applies.
    #[serde(default)]
    pub timeout_seconds: Option<u64>,

    /// Roles permitted to invoke this agent; empty/absent ⇒ any authenticated caller.
    #[serde(default)]
    pub allowed_roles: Vec<UserRole>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_full_definition() {
        let json = serde_json::json!({
            "id": "researcher",
            "model": "gpt-4",
            "system_prompt": "You research topics.",
            "provider": "openai",
            "temperature": 0.7,
            "max_loops": 5,
            "stop_words": ["STOP"]
        });
        let def: AgentDefinition = serde_json::from_value(json).expect("parses");
        assert_eq!(def.id, "researcher");
        assert_eq!(def.model, "gpt-4");
        assert_eq!(def.system_prompt, "You research topics.");
        assert_eq!(def.provider.as_deref(), Some("openai"));
        assert_eq!(def.temperature, Some(0.7));
        assert_eq!(def.max_loops, Some(5));
        assert_eq!(def.stop_words, vec!["STOP".to_string()]);
    }

    #[test]
    fn deserializes_minimal_definition_with_defaults() {
        // Only the three required fields are present.
        let json = serde_json::json!({
            "id": "summarizer",
            "model": "gpt-4",
            "system_prompt": "You summarize."
        });
        let def: AgentDefinition = serde_json::from_value(json).expect("parses");
        assert_eq!(def.id, "summarizer");
        assert!(def.provider.is_none());
        assert!(def.temperature.is_none());
        assert!(def.max_loops.is_none());
        assert!(def.stop_words.is_empty());
    }

    #[test]
    fn missing_required_field_fails() {
        // No `system_prompt` → must not deserialize.
        let json = serde_json::json!({ "id": "x", "model": "gpt-4" });
        let result: Result<AgentDefinition, _> = serde_json::from_value(json);
        assert!(result.is_err(), "missing required field must fail");
    }

    #[test]
    fn definition_parses_allowed_roles() {
        let json = serde_json::json!({
            "id": "x", "model": "gpt-4", "system_prompt": "p",
            "allowed_roles": ["admin", "user"]
        });
        let def: AgentDefinition = serde_json::from_value(json).expect("parses");
        assert_eq!(def.allowed_roles, vec![UserRole::Admin, UserRole::User]);
    }

    #[test]
    fn auth_config_defaults_to_enabled_with_no_credentials() {
        // An empty `auth:` section ⇒ enabled, no keys, JWT off (secure default).
        let auth: AuthConfig = serde_json::from_value(serde_json::json!({})).expect("parses");
        assert!(auth.enabled);
        assert!(auth.api_keys.is_empty());
        assert!(!auth.jwt.enabled);
    }

    #[test]
    fn auth_config_parses_api_keys_with_roles() {
        let json = serde_json::json!({
            "enabled": true,
            "api_keys": [
                { "key": "sk-1", "name": "ci", "role": "admin" },
                { "key": "sk-2", "name": "fe", "role": "user" }
            ],
            "jwt": { "enabled": true }
        });
        let auth: AuthConfig = serde_json::from_value(json).expect("parses");
        assert_eq!(auth.api_keys.len(), 2);
        assert_eq!(auth.api_keys[0].role, UserRole::Admin);
        assert_eq!(auth.api_keys[1].role, UserRole::User);
        assert!(auth.jwt.enabled);
    }

    #[test]
    fn auth_config_rejects_unknown_role() {
        let json = serde_json::json!({
            "api_keys": [ { "key": "sk", "name": "x", "role": "superuser" } ]
        });
        let result: Result<AuthConfig, _> = serde_json::from_value(json);
        assert!(result.is_err(), "unknown role must fail to parse");
    }
}
