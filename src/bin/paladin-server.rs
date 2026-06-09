//! `paladin-server` — run configured Paladin agents behind an HTTP API.
//!
//! This is the runnable entry point for the **HTTP service-host** deployment topology
//! (Milestone 12). It loads a `config.yml`, builds the configured agents into a
//! `paladin-web` agent registry, and serves the agent-execution API
//! (`/agents/*`) over HTTP with graceful shutdown.
//!
//! ```bash
//! OPENAI_API_KEY=sk-... cargo run --bin paladin-server --features web-server
//! # or point at a specific config:
//! PALADIN_CONFIG=./config.yml paladin-server
//! ```
//!
//! Requires the `web-server` feature (enforced via `required-features` in `Cargo.toml`).

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use log::{error, info, warn};
use paladin::config::agents::AuthConfig;
use paladin::config::settings::Settings;
use paladin::infrastructure::adapters::auth::InMemoryTokenAuthAdapter;
use paladin::infrastructure::web::agent_host::{bind_address, build_agent_registry};
use paladin::infrastructure::web::facade_provisioner::FacadeProvisioner;
use paladin::infrastructure::web::{
    AgentApiState, AgentAuthConfig, HttpLayersConfig, Principal, RateLimitConfig, TimeoutPolicy,
    agent_router, with_http_layers,
};
use paladin_ports::output::auth_port::AuthPort;
use tokio::signal;

#[tokio::main]
async fn main() {
    // Load .env in debug builds; production uses real secrets management.
    #[cfg(debug_assertions)]
    {
        let _ = dotenv::dotenv();
    }
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if let Err(e) = run().await {
        error!("paladin-server failed to start: {e}");
        std::process::exit(1);
    }
}

/// Load config, build the agent host, and serve until a shutdown signal.
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config_path();
    info!("Loading configuration from '{config_path}'");
    let settings = Settings::load_from_file(&config_path)?;

    // Build the resident agents and the runtime provisioner from the same config.
    // `build_agent_registry` validates the config first, so misconfiguration fails here
    // with a specific message rather than mid-serve.
    let registry = build_agent_registry(&settings).await?;
    let mut agent_ids: Vec<String> = registry.list().into_iter().map(|(id, _)| id).collect();
    agent_ids.sort();
    let provisioner = FacadeProvisioner::from_settings(&settings);
    let timeouts = settings.timeouts.clone().unwrap_or_default();
    // Cross-cutting HTTP layers (health routes are merged inside `agent_router`).
    let http = settings.http.clone().unwrap_or_default();

    // Resolve authentication (fail-closed: enabled + no credentials ⇒ refuse to start).
    let auth = build_auth_config(&http.auth)?;
    let state = AgentApiState::new(Arc::new(registry))
        .with_provisioner(Arc::new(provisioner))
        .with_timeouts(TimeoutPolicy {
            default_secs: timeouts.default_seconds,
            max_secs: timeouts.max_seconds,
        })
        .with_auth(auth);
    let layers = HttpLayersConfig {
        cors_allow_origins: http.cors_allow_origins.clone(),
        body_limit_bytes: http.body_limit_bytes,
        global_timeout_secs: http.global_timeout_seconds,
        rate_limit: RateLimitConfig {
            enabled: http.rate_limit.enabled,
            per_second: http.rate_limit.per_second,
            burst: http.rate_limit.burst,
        },
    };
    let app = with_http_layers(agent_router(state), &layers);

    let listener = tokio::net::TcpListener::bind(bind_address(&settings)).await?;
    let bound = listener.local_addr()?;
    info!(
        "paladin-server listening on http://{bound} — serving {} agent(s): {:?}",
        agent_ids.len(),
        agent_ids
    );
    info!(
        "routes: GET /health, GET /ready, GET/POST /v1/agents, GET/DELETE /v1/agents/{{id}}, POST /v1/agents/{{id}}/execute[/stream], POST /v1/agents/{{id}}/jobs, GET /v1/agents/{{id}}/jobs/{{job_id}}"
    );
    info!(
        "layers: request-log + CORS + body-limit({}B){}{}",
        layers.body_limit_bytes,
        if layers.global_timeout_secs > 0 {
            format!(" + global-timeout({}s)", layers.global_timeout_secs)
        } else {
            String::new()
        },
        if layers.rate_limit.enabled {
            format!(
                " + rate-limit({}/s, burst {})",
                layers.rate_limit.per_second, layers.rate_limit.burst
            )
        } else {
            String::new()
        }
    );

    // `ConnectInfo` lets the rate limiter key on the peer IP for direct connections.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    info!("paladin-server shut down cleanly");
    Ok(())
}

/// Translate the config `auth` section into the web layer's [`AgentAuthConfig`].
///
/// **Fail-closed:** when auth is enabled but no credential source (API keys or JWT) is
/// configured, this returns an error so the server refuses to start rather than silently
/// serving an open API. When auth is disabled, a warning is logged and the API is open.
fn build_auth_config(cfg: &AuthConfig) -> Result<AgentAuthConfig, Box<dyn std::error::Error>> {
    if !cfg.enabled {
        warn!(
            "agent API authentication is DISABLED (http.auth.enabled = false) — all agent routes are open"
        );
        return Ok(AgentAuthConfig {
            enabled: false,
            api_keys: HashMap::new(),
            jwt: None,
        });
    }

    let api_keys: HashMap<String, Principal> = cfg
        .api_keys
        .iter()
        .map(|k| {
            (
                k.key.clone(),
                Principal {
                    id: k.name.clone(),
                    role: k.role,
                },
            )
        })
        .collect();

    // The JWT path reuses the existing AuthPort. The in-memory adapter verifies tokens it
    // issued in-process, so JWT is primarily useful when token issuance is co-located;
    // API keys are the standalone service-to-service mechanism.
    let jwt: Option<Arc<dyn AuthPort>> = if cfg.jwt.enabled {
        Some(Arc::new(InMemoryTokenAuthAdapter::new()))
    } else {
        None
    };

    let auth = AgentAuthConfig {
        enabled: true,
        api_keys,
        jwt,
    };

    if !auth.has_credentials() {
        return Err(
            "authentication is enabled but no credentials are configured: set \
             http.auth.api_keys and/or http.auth.jwt.enabled, or set http.auth.enabled = false"
                .into(),
        );
    }

    info!(
        "agent API authentication ENABLED ({} API key(s){})",
        auth.api_keys.len(),
        if cfg.jwt.enabled { " + JWT" } else { "" }
    );
    Ok(auth)
}

/// Resolve the config file path: `PALADIN_CONFIG`, else the first CLI argument, else
/// `config.yml`.
fn config_path() -> String {
    std::env::var("PALADIN_CONFIG")
        .ok()
        .or_else(|| std::env::args().nth(1))
        .unwrap_or_else(|| "config.yml".to_string())
}

/// Resolve when the process receives `Ctrl-C` or (on Unix) `SIGTERM`.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("received Ctrl-C; shutting down"),
        _ = terminate => info!("received SIGTERM; shutting down"),
    }
}
