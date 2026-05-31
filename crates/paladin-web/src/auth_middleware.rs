//! Authentication and authorization middleware for the user REST API.
//!
//! This module provides Axum middleware that authenticates incoming requests
//! using a bearer token verified through an [`AuthPort`] implementation, plus
//! authorization guards for role-based access control (RBAC). The middleware
//! injects the verified [`AuthClaims`] into the request extensions so that
//! downstream handlers can make self-scope decisions.
//!
//! All failure responses use a non-revealing JSON body so that the API does not
//! leak whether a token was missing, malformed, unknown, or expired.

use std::sync::Arc;

use axum::{
    Json,
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use paladin_core::platform::container::user::UserRole;
use paladin_ports::output::auth_port::{AuthClaims, AuthPort};
use serde_json::json;
use uuid::Uuid;

/// Build a non-revealing `401 Unauthorized` JSON response.
fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized", "code": "UNAUTHORIZED" })),
    )
        .into_response()
}

/// Build a non-revealing `403 Forbidden` JSON response.
fn forbidden() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({ "error": "Forbidden", "code": "FORBIDDEN" })),
    )
        .into_response()
}

/// Extract a non-empty bearer token from the `Authorization` header.
///
/// Returns `None` when the header is absent, not valid UTF-8, does not use the
/// `Bearer` scheme, or carries an empty token.
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

/// Authenticate a request from its headers, returning the verified claims.
///
/// # Errors
/// Returns a `401 Unauthorized` [`Response`] when no usable token is present or
/// when the token fails verification (unknown, malformed, or expired). The error
/// response is intentionally identical for all failure modes.
async fn authenticate(auth: &dyn AuthPort, headers: &HeaderMap) -> Result<AuthClaims, Response> {
    let token = bearer_token(headers).ok_or_else(unauthorized)?;
    auth.verify_token(token).await.map_err(|_| unauthorized())
}

/// Decide whether the supplied claims satisfy the admin-only guard.
///
/// # Errors
/// Returns a `403 Forbidden` response when the claims belong to a non-admin
/// user, or a `401 Unauthorized` response when no claims are present (indicating
/// the request was not authenticated first).
#[allow(clippy::result_large_err)]
fn check_admin(claims: Option<&AuthClaims>) -> Result<(), Response> {
    match claims {
        Some(claims) if claims.role == UserRole::Admin => Ok(()),
        Some(_) => Err(forbidden()),
        None => Err(unauthorized()),
    }
}

/// Authorize access to a user-scoped resource for the given target id.
///
/// Admins may access any resource; non-admin users may only access their own
/// resource (where `claims.user_id == target`).
///
/// # Errors
/// Returns a `403 Forbidden` response when a non-admin user attempts to access a
/// resource that is not their own.
#[allow(clippy::result_large_err)]
pub fn authorize_self_or_admin(claims: &AuthClaims, target: Uuid) -> Result<(), Response> {
    if claims.role == UserRole::Admin || claims.user_id == target {
        Ok(())
    } else {
        Err(forbidden())
    }
}

/// Middleware that requires a valid bearer token on the request.
///
/// On success the verified [`AuthClaims`] are inserted into the request
/// extensions and the request is forwarded to the next handler. On failure a
/// non-revealing `401 Unauthorized` JSON response is returned.
pub async fn require_auth(
    State(auth): State<Arc<dyn AuthPort>>,
    mut request: Request,
    next: Next,
) -> Response {
    match authenticate(auth.as_ref(), request.headers()).await {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            next.run(request).await
        }
        Err(response) => response,
    }
}

/// Middleware that requires the authenticated user to have the `Admin` role.
///
/// This must run after [`require_auth`] so that [`AuthClaims`] are present in the
/// request extensions. Returns `403 Forbidden` for non-admin users and
/// `401 Unauthorized` if the request was not authenticated.
pub async fn require_admin(request: Request, next: Next) -> Response {
    match check_admin(request.extensions().get::<AuthClaims>()) {
        Ok(()) => next.run(request).await,
        Err(response) => response,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use paladin_ports::output::auth_port::{AuthError, AuthToken};

    struct MockAuthPort {
        claims: Option<AuthClaims>,
    }

    #[async_trait]
    impl AuthPort for MockAuthPort {
        async fn issue_token(
            &self,
            _user_id: Uuid,
            _role: UserRole,
        ) -> Result<AuthToken, AuthError> {
            Err(AuthError::Internal("not used".to_string()))
        }

        async fn verify_token(&self, _token: &str) -> Result<AuthClaims, AuthError> {
            self.claims.clone().ok_or(AuthError::InvalidToken)
        }

        async fn revoke_token(&self, _token: &str) -> Result<(), AuthError> {
            Ok(())
        }
    }

    fn headers_with(value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, value.parse().unwrap());
        headers
    }

    fn claims(role: UserRole, user_id: Uuid) -> AuthClaims {
        AuthClaims {
            user_id,
            role,
            expires_at: Utc::now() + Duration::hours(1),
        }
    }

    #[test]
    fn bearer_token_parses_valid_header() {
        let headers = headers_with("Bearer abc123");
        assert_eq!(bearer_token(&headers), Some("abc123"));
    }

    #[test]
    fn bearer_token_rejects_missing_and_empty() {
        assert_eq!(bearer_token(&HeaderMap::new()), None);
        assert_eq!(bearer_token(&headers_with("Bearer ")), None);
        assert_eq!(bearer_token(&headers_with("Basic abc")), None);
    }

    #[tokio::test]
    async fn authenticate_missing_token_is_unauthorized() {
        let auth = MockAuthPort { claims: None };
        let response = authenticate(&auth, &HeaderMap::new()).await.unwrap_err();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn authenticate_invalid_token_is_unauthorized() {
        let auth = MockAuthPort { claims: None };
        let response = authenticate(&auth, &headers_with("Bearer bad"))
            .await
            .unwrap_err();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn authenticate_valid_token_returns_claims() {
        let expected = claims(UserRole::User, Uuid::new_v4());
        let auth = MockAuthPort {
            claims: Some(expected.clone()),
        };
        let result = authenticate(&auth, &headers_with("Bearer good"))
            .await
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn check_admin_allows_admin() {
        let claims = claims(UserRole::Admin, Uuid::new_v4());
        assert!(check_admin(Some(&claims)).is_ok());
    }

    #[test]
    fn check_admin_forbids_non_admin() {
        let claims = claims(UserRole::User, Uuid::new_v4());
        let response = check_admin(Some(&claims)).unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn check_admin_unauthorized_without_claims() {
        let response = check_admin(None).unwrap_err();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn self_scope_allows_admin_for_any_target() {
        let claims = claims(UserRole::Admin, Uuid::new_v4());
        assert!(authorize_self_or_admin(&claims, Uuid::new_v4()).is_ok());
    }

    #[test]
    fn self_scope_allows_user_for_own_target() {
        let id = Uuid::new_v4();
        let claims = claims(UserRole::User, id);
        assert!(authorize_self_or_admin(&claims, id).is_ok());
    }

    #[test]
    fn self_scope_forbids_user_for_other_target() {
        let claims = claims(UserRole::User, Uuid::new_v4());
        let response = authorize_self_or_admin(&claims, Uuid::new_v4()).unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
