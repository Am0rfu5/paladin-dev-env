//! Cross-cutting HTTP layer configuration and builders (Milestone 12, Epic 4).
//!
//! Provides [`HttpLayersConfig`] and helpers to build the edge middleware — CORS, a
//! request body-size limit, and an optional (off-by-default) per-IP rate limiter — plus,
//! in [`with_http_layers`](crate::http_layers::with_http_layers) (task 6.0), the request
//! logger, a global timeout scoped away from streaming, and the health routes.
//!
//! Layer rejections that reach the client (e.g. rate-limit `429`) render through the
//! unified [`ApiError`] envelope.

use axum::Router;
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use crate::error::ApiError;

/// Per-client rate-limit settings (off by default).
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Whether the rate limiter is enabled.
    pub enabled: bool,
    /// Sustained requests per second allowed per client IP.
    pub per_second: u64,
    /// Burst capacity (requests allowed instantaneously) per client IP.
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            per_second: 10,
            burst: 20,
        }
    }
}

/// Configuration for the cross-cutting HTTP layers.
#[derive(Debug, Clone)]
pub struct HttpLayersConfig {
    /// Allowed CORS origins; empty ⇒ permissive (`Any`), suitable for local dev.
    pub cors_allow_origins: Vec<String>,
    /// Maximum request body size in bytes (oversized requests are rejected with `413`).
    pub body_limit_bytes: usize,
    /// Global request timeout in seconds for non-streaming routes; `0` disables it.
    pub global_timeout_secs: u64,
    /// Rate-limit settings (off by default).
    pub rate_limit: RateLimitConfig,
}

impl Default for HttpLayersConfig {
    fn default() -> Self {
        Self {
            cors_allow_origins: Vec::new(),
            body_limit_bytes: 1024 * 1024, // 1 MiB
            global_timeout_secs: 0,        // disabled (streaming-safe default)
            rate_limit: RateLimitConfig::default(),
        }
    }
}

/// Build the CORS layer. With no configured origins it is permissive (dev-friendly);
/// otherwise it allows exactly the listed origins (with any method/header).
pub fn cors_layer(config: &HttpLayersConfig) -> CorsLayer {
    if config.cors_allow_origins.is_empty() {
        CorsLayer::permissive()
    } else {
        let origins: Vec<HeaderValue> = config
            .cors_allow_origins
            .iter()
            .filter_map(|o| o.parse::<HeaderValue>().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}

/// Build the request body-size limit layer.
pub fn body_limit_layer(config: &HttpLayersConfig) -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(config.body_limit_bytes)
}

/// Apply the per-IP rate limiter to `router` when enabled, rendering `429` as [`ApiError`].
///
/// Disabled (the default) or an invalid config returns the router unchanged (the latter is
/// logged). Uses [`SmartIpKeyExtractor`], which reads forwarded headers and falls back to
/// the connection peer IP (the server binary wires `ConnectInfo` so direct connections key
/// correctly).
pub fn apply_rate_limit(router: Router, config: &RateLimitConfig) -> Router {
    if !config.enabled {
        return router;
    }
    match GovernorConfigBuilder::default()
        .per_second(config.per_second)
        .burst_size(config.burst)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
    {
        Some(governor_conf) => {
            let layer =
                GovernorLayer::new(std::sync::Arc::new(governor_conf)).error_handler(|_err| {
                    ApiError::too_many_requests("rate limit exceeded").into_response()
                });
            router.layer(layer)
        }
        None => {
            log::error!(
                "invalid rate-limit config (per_second={}, burst={}); rate limiting disabled",
                config.per_second,
                config.burst
            );
            router
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::{get, post};
    use tower::ServiceExt; // for `Router::oneshot`

    #[tokio::test]
    async fn cors_preflight_returns_allow_origin() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(cors_layer(&HttpLayersConfig::default()));

        let resp = app
            .oneshot(
                Request::builder()
                    .method("OPTIONS")
                    .uri("/")
                    .header("origin", "http://example.com")
                    .header("access-control-request-method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(
            resp.headers().contains_key("access-control-allow-origin"),
            "preflight should set access-control-allow-origin"
        );
    }

    #[tokio::test]
    async fn oversized_body_is_rejected() {
        let config = HttpLayersConfig {
            body_limit_bytes: 8,
            ..HttpLayersConfig::default()
        };
        // The `Bytes` extractor reads the whole body, which trips the limit.
        let app = Router::new()
            .route("/", post(|_body: axum::body::Bytes| async { "ok" }))
            .layer(body_limit_layer(&config));

        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-length", "48")
                    .body(Body::from(
                        "this body is definitely longer than eight bytes",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn rate_limit_returns_429_when_exceeded() {
        let config = RateLimitConfig {
            enabled: true,
            per_second: 1,
            burst: 1,
        };
        let app = apply_rate_limit(Router::new().route("/", get(|| async { "ok" })), &config);

        let make_req = || {
            Request::builder()
                .uri("/")
                .header("x-real-ip", "1.2.3.4") // stable key without ConnectInfo
                .body(Body::empty())
                .unwrap()
        };

        let first = app.clone().oneshot(make_req()).await.unwrap();
        assert_eq!(first.status(), StatusCode::OK);

        let second = app.oneshot(make_req()).await.unwrap();
        assert_eq!(second.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn rate_limit_disabled_is_passthrough() {
        let app = apply_rate_limit(
            Router::new().route("/", get(|| async { "ok" })),
            &RateLimitConfig::default(), // disabled
        );
        for _ in 0..5 {
            let resp = app
                .clone()
                .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }
    }
}
