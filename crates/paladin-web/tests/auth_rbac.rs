//! Integration tests for authentication and role-based access control on the
//! user management REST API.
//!
//! These tests build the application router via
//! [`paladin_web::app::create_app_router`] and drive it with `tower`'s
//! `oneshot`, using an in-test [`AuthPort`] and mock [`UserServiceTrait`]. They
//! are fully offline and deterministic.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::{Duration, Utc};
use paladin_core::platform::container::user::UserRole;
use paladin_core::platform::container::user::{Email, User, UserError};
use paladin_core::platform::manager::user_service::{
    UserAuthenticationResult, UserLoginRequest, UserProfileUpdateRequest, UserRegistrationRequest,
    UserServiceTrait,
};
use paladin_ports::output::auth_port::{AuthClaims, AuthError, AuthPort, AuthToken};
use paladin_web::adapters::api_content_deliverer::ApiContentDeliverer;
use paladin_web::app::create_app_router;
use tower::ServiceExt;
use uuid::Uuid;

/// In-test auth port backed by a fixed token-to-claims map.
struct TestAuthPort {
    tokens: HashMap<String, AuthClaims>,
}

impl TestAuthPort {
    fn new(tokens: HashMap<String, AuthClaims>) -> Self {
        Self { tokens }
    }
}

#[async_trait]
impl AuthPort for TestAuthPort {
    async fn issue_token(&self, _user_id: Uuid, _role: UserRole) -> Result<AuthToken, AuthError> {
        Err(AuthError::Internal("issuing not used in tests".to_string()))
    }

    async fn verify_token(&self, token: &str) -> Result<AuthClaims, AuthError> {
        if token.is_empty() {
            return Err(AuthError::MissingToken);
        }
        self.tokens
            .get(token)
            .cloned()
            .ok_or(AuthError::InvalidToken)
    }

    async fn revoke_token(&self, _token: &str) -> Result<(), AuthError> {
        Ok(())
    }
}

/// Minimal mock user service returning deterministic data.
struct MockUserService;

fn sample_user() -> User {
    User::new_user(
        "testuser".to_string(),
        Email::new("test@example.com".to_string()).unwrap(),
        "mock_hash".to_string(),
        None,
    )
}

#[async_trait]
impl UserServiceTrait for MockUserService {
    async fn register_user(&self, _request: UserRegistrationRequest) -> Result<User, UserError> {
        Ok(sample_user())
    }

    async fn login_user(
        &self,
        _request: UserLoginRequest,
    ) -> Result<UserAuthenticationResult, UserError> {
        Ok(UserAuthenticationResult {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            is_verified: true,
            success: true,
            token: None,
            token_expires_at: None,
        })
    }

    async fn update_user_profile(
        &self,
        _request: UserProfileUpdateRequest,
    ) -> Result<User, UserError> {
        Ok(sample_user())
    }

    async fn get_user_by_id(&self, _user_id: Uuid) -> Result<Option<User>, UserError> {
        Ok(Some(sample_user()))
    }

    async fn get_user_by_email(&self, _email: &str) -> Result<Option<User>, UserError> {
        Ok(Some(sample_user()))
    }

    async fn delete_user(&self, _user_id: Uuid) -> Result<(), UserError> {
        Ok(())
    }

    async fn list_users(&self) -> Result<Vec<User>, UserError> {
        Ok(vec![sample_user()])
    }

    async fn activate_user(&self, _user_id: Uuid) -> Result<(), UserError> {
        Ok(())
    }

    async fn deactivate_user(&self, _user_id: Uuid) -> Result<(), UserError> {
        Ok(())
    }

    async fn verify_user(&self, _user_id: Uuid) -> Result<(), UserError> {
        Ok(())
    }

    async fn find_by_active_status(&self, _is_active: bool) -> Result<Vec<User>, UserError> {
        Ok(vec![sample_user()])
    }

    async fn find_by_verification_status(
        &self,
        _is_verified: bool,
    ) -> Result<Vec<User>, UserError> {
        Ok(vec![sample_user()])
    }

    async fn count_users(&self) -> Result<u64, UserError> {
        Ok(1)
    }
}

fn claims(role: UserRole) -> AuthClaims {
    AuthClaims {
        user_id: Uuid::new_v4(),
        role,
        expires_at: Utc::now() + Duration::hours(1),
    }
}

fn build_app() -> axum::Router {
    let mut tokens = HashMap::new();
    tokens.insert("admin-token".to_string(), claims(UserRole::Admin));
    tokens.insert("user-token".to_string(), claims(UserRole::User));

    let user_service: Arc<dyn UserServiceTrait> = Arc::new(MockUserService);
    let auth_port: Arc<dyn AuthPort> = Arc::new(TestAuthPort::new(tokens));
    let deliverer = Arc::new(ApiContentDeliverer::new());
    create_app_router(user_service, auth_port, deliverer)
}

#[tokio::test]
async fn protected_route_without_token_is_unauthorized() {
    let app = build_app();
    let request = Request::builder()
        .method("GET")
        .uri(format!("/users/{}", Uuid::new_v4()))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_route_with_valid_admin_token_succeeds() {
    let app = build_app();
    let request = Request::builder()
        .method("GET")
        .uri(format!("/users/{}", Uuid::new_v4()))
        .header("authorization", "Bearer admin-token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn admin_route_with_user_token_is_forbidden() {
    let app = build_app();
    let request = Request::builder()
        .method("GET")
        .uri("/users")
        .header("authorization", "Bearer user-token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn admin_route_with_admin_token_succeeds() {
    let app = build_app();
    let request = Request::builder()
        .method("GET")
        .uri("/users")
        .header("authorization", "Bearer admin-token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn admin_route_with_invalid_token_is_unauthorized() {
    let app = build_app();
    let request = Request::builder()
        .method("GET")
        .uri("/users")
        .header("authorization", "Bearer bogus")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
