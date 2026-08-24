//! Authentication port for issuing and verifying bearer tokens.
//!
//! This port defines the contract used by the web layer to authenticate
//! requests without performing any cryptography itself. Concrete adapters
//! (for example an in-memory opaque-token store) live in the root crate where
//! cryptographic dependencies are available, preserving the hexagonal boundary:
//! `paladin-web` depends only on this trait via `Arc<dyn AuthPort>`.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use paladin_core::platform::container::user::UserRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An issued authentication token together with its expiry instant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthToken {
    /// The opaque bearer token string presented by clients.
    pub token: String,
    /// The instant at which the token expires and is no longer valid.
    pub expires_at: DateTime<Utc>,
}

/// The verified claims associated with a valid authentication token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthClaims {
    /// The unique identifier of the authenticated user.
    pub user_id: Uuid,
    /// The role granted to the authenticated user.
    pub role: UserRole,
    /// The instant at which the underlying token expires.
    pub expires_at: DateTime<Utc>,
}

/// Errors that can occur while issuing or verifying authentication tokens.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// No token was supplied where one was required.
    #[error("Missing authentication token")]
    MissingToken,
    /// The supplied token is malformed or unknown.
    #[error("Invalid authentication token")]
    InvalidToken,
    /// The supplied token has expired.
    #[error("Authentication token has expired")]
    Expired,
    /// An internal error occurred while processing the token.
    #[error("Authentication internal error: {0}")]
    Internal(String),
}

/// Port abstracting token issuance, verification, and revocation.
///
/// Implementations must be safe to share across threads (`Send + Sync`) so that
/// they can be held behind an `Arc` in asynchronous request handlers.
///
/// # Examples
///
/// ```rust
/// use paladin_ports::output::auth_port::AuthPort;
/// use paladin_core::platform::container::user::UserRole;
/// use uuid::Uuid;
///
/// async fn login_and_verify(
///     auth: &dyn AuthPort,
///     user_id: Uuid,
/// ) -> Result<bool, paladin_ports::output::auth_port::AuthError> {
///     let issued = auth.issue_token(user_id, UserRole::User).await?;
///     let claims = auth.verify_token(&issued.token).await?;
///     let is_same_user = claims.user_id == user_id;
///
///     // Revoking makes the token immediately invalid.
///     auth.revoke_token(&issued.token).await?;
///
///     Ok(is_same_user)
/// }
/// ```
#[async_trait]
pub trait AuthPort: Send + Sync {
    /// Issues a new token for the given user and role.
    ///
    /// # Errors
    /// Returns [`AuthError::Internal`] if the token could not be generated or
    /// stored.
    async fn issue_token(&self, user_id: Uuid, role: UserRole) -> Result<AuthToken, AuthError>;

    /// Verifies a presented token and returns its claims.
    ///
    /// # Errors
    /// Returns [`AuthError::InvalidToken`] if the token is unknown,
    /// [`AuthError::Expired`] if it has expired, or [`AuthError::Internal`] on
    /// an unexpected failure.
    async fn verify_token(&self, token: &str) -> Result<AuthClaims, AuthError>;

    /// Revokes a previously issued token, making it immediately invalid.
    ///
    /// Revoking an unknown or already-expired token is a no-op and succeeds.
    ///
    /// # Errors
    /// Returns [`AuthError::Internal`] on an unexpected failure.
    async fn revoke_token(&self, token: &str) -> Result<(), AuthError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_error_messages_are_stable() {
        assert_eq!(
            AuthError::MissingToken.to_string(),
            "Missing authentication token"
        );
        assert_eq!(
            AuthError::InvalidToken.to_string(),
            "Invalid authentication token"
        );
        assert_eq!(
            AuthError::Expired.to_string(),
            "Authentication token has expired"
        );
        assert_eq!(
            AuthError::Internal("boom".to_string()).to_string(),
            "Authentication internal error: boom"
        );
    }

    #[test]
    fn auth_claims_round_trip_through_json() {
        let claims = AuthClaims {
            user_id: Uuid::nil(),
            role: UserRole::Admin,
            expires_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
        };
        let json = serde_json::to_string(&claims).unwrap();
        let decoded: AuthClaims = serde_json::from_str(&json).unwrap();
        assert_eq!(claims, decoded);
    }
}
