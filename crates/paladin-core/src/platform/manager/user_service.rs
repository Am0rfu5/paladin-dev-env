//! User service trait and request/response DTOs for the Paladin framework.
//!
//! This module contains the abstract service interface for user management
//! operations. The concrete implementation lives in the facade's
//! `core::platform::manager::user_service`.

use crate::platform::container::user::{User, UserError, UserProfile};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User registration request DTO.
#[derive(Debug, Clone)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile: Option<UserProfile>,
}

/// User login request DTO.
#[derive(Debug, Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

/// User profile update request DTO.
#[derive(Debug, Clone)]
pub struct UserProfileUpdateRequest {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub profile: Option<UserProfile>,
}

/// User authentication result DTO.
#[derive(Debug, Clone)]
pub struct UserAuthenticationResult {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub success: bool,
    /// Opaque bearer token issued on successful login, when an auth provider is
    /// configured. `None` if no token provider is wired into the service.
    pub token: Option<String>,
    /// Expiry instant of the issued token, paired with [`Self::token`].
    pub token_expires_at: Option<DateTime<Utc>>,
}

/// Abstract user service port: business operations on users.
#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    /// Register a new user.
    async fn register_user(&self, request: UserRegistrationRequest) -> Result<User, UserError>;

    /// Authenticate a user login.
    async fn login_user(
        &self,
        request: UserLoginRequest,
    ) -> Result<UserAuthenticationResult, UserError>;

    /// Update user profile.
    async fn update_user_profile(
        &self,
        request: UserProfileUpdateRequest,
    ) -> Result<User, UserError>;

    /// Get user by ID.
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError>;

    /// Get user by email.
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError>;

    /// Delete a user account by ID.
    async fn delete_user(&self, user_id: Uuid) -> Result<(), UserError>;

    /// List all users (administrative operation).
    async fn list_users(&self) -> Result<Vec<User>, UserError>;

    /// Activate user account.
    async fn activate_user(&self, user_id: Uuid) -> Result<(), UserError>;

    /// Deactivate user account.
    async fn deactivate_user(&self, user_id: Uuid) -> Result<(), UserError>;

    /// Verify user email.
    async fn verify_user(&self, user_id: Uuid) -> Result<(), UserError>;

    /// Find users by active status.
    async fn find_by_active_status(&self, is_active: bool) -> Result<Vec<User>, UserError>;

    /// Find users by verification status.
    async fn find_by_verification_status(&self, is_verified: bool) -> Result<Vec<User>, UserError>;

    /// Count total users.
    async fn count_users(&self) -> Result<u64, UserError>;
}
