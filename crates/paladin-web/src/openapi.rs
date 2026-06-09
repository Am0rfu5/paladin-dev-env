//! OpenAPI spec assembly and interactive docs serving (Milestone 12, Epic 6).
//!
//! The spec is **derived from the handlers** (`#[utoipa::path]`) and DTOs (`ToSchema`) via
//! `utoipa-axum`, so the served API and the published contract come from one source.
//! [`build_openapi`] assembles the `/v1` agent API document and decorates it with API info
//! and the two security schemes (API key + bearer JWT); [`docs_router`] serves it at
//! `GET /openapi.json` with a Swagger UI at `/docs`.
//!
//! Exposure is gated by the binary on `http.docs.enabled` — when disabled, the docs router
//! is simply not mounted (both routes `404`). The docs endpoints are unversioned and
//! unauthenticated: a consumer needs the contract before they hold credentials, and the
//! spec describes shapes, never secret values.

use std::sync::Arc;

use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::agent_controller::{AgentApiState, versioned_agent_parts};
use crate::agent_registry::AgentRegistry;

/// Security-scheme name for the `X-API-Key` header credential (matches the handler annotations).
pub const SEC_API_KEY: &str = "api_key";
/// Security-scheme name for the `Authorization: Bearer` (JWT) credential.
pub const SEC_JWT: &str = "jwt";

/// Path at which the raw OpenAPI document is served.
pub const OPENAPI_JSON_PATH: &str = "/openapi.json";
/// Base path at which the Swagger UI is served.
pub const DOCS_PATH: &str = "/docs";

/// Decorate a generated document with API info and the security schemes.
fn decorate(api: &mut OpenApi) {
    api.info.title = "Paladin Agent API".to_string();
    api.info.version = env!("CARGO_PKG_VERSION").to_string();
    api.info.description = Some(
        "HTTP API for executing and managing resident Paladin agents. \
         Agent routes are served under `/v1`; `/health`, `/ready`, and the docs are unversioned."
            .to_string(),
    );

    let components = api.components.get_or_insert_with(Default::default);
    components.add_security_scheme(
        SEC_API_KEY,
        SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
    );
    components.add_security_scheme(
        SEC_JWT,
        SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        ),
    );
}

/// Build the decorated OpenAPI document for the agent API (paths under `/v1`).
///
/// The `state` only shapes the (discarded) router; the document depends solely on the
/// handler annotations, so any state — including an empty one — yields the same spec.
pub fn build_openapi(state: AgentApiState) -> OpenApi {
    let (_router, mut api) = versioned_agent_parts(state);
    decorate(&mut api);
    api
}

/// Build the decorated OpenAPI document using a throwaway empty state.
///
/// Convenience for the drift guard and `/openapi.json` serving where no live state is at
/// hand.
pub fn openapi_spec() -> OpenApi {
    build_openapi(AgentApiState::new(Arc::new(AgentRegistry::new())))
}

/// Router serving the spec at [`OPENAPI_JSON_PATH`] and Swagger UI at [`DOCS_PATH`].
///
/// Merge into the application router only when docs are enabled.
pub fn docs_router(spec: OpenApi) -> Router {
    Router::new().merge(SwaggerUi::new(DOCS_PATH).url(OPENAPI_JSON_PATH, spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_has_info_and_security_schemes() {
        let api = openapi_spec();
        assert_eq!(api.info.title, "Paladin Agent API");
        let schemes = &api
            .components
            .as_ref()
            .expect("components present")
            .security_schemes;
        assert!(schemes.contains_key(SEC_API_KEY), "missing api_key scheme");
        assert!(schemes.contains_key(SEC_JWT), "missing jwt scheme");
    }

    #[test]
    fn spec_paths_are_versioned_under_v1() {
        let api = openapi_spec();
        let paths = &api.paths.paths;
        assert!(
            paths.contains_key("/v1/agents"),
            "paths: {:?}",
            paths.keys().collect::<Vec<_>>()
        );
        assert!(paths.contains_key("/v1/agents/{id}/execute"));
        assert!(!paths.contains_key("/agents"));
    }

    use crate::agent_controller::agent_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn docs_router_serves_spec_and_ui() {
        let app = docs_router(openapi_spec());

        let spec = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(OPENAPI_JSON_PATH)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(spec.status(), StatusCode::OK);

        // Swagger UI index (the bundle redirects `/docs` → `/docs/`).
        let ui = app
            .oneshot(
                Request::builder()
                    .uri("/docs/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(
            ui.status().is_success() || ui.status().is_redirection(),
            "unexpected /docs status: {}",
            ui.status()
        );
    }

    #[tokio::test]
    async fn without_docs_router_spec_is_404_but_api_works() {
        // Mirrors the binary when `http.docs.enabled = false`: no docs routes mounted.
        let state = AgentApiState::new(Arc::new(AgentRegistry::new()));
        let app = agent_router(state);

        let spec = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(OPENAPI_JSON_PATH)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(spec.status(), StatusCode::NOT_FOUND);

        let health = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), StatusCode::OK);
    }
}
