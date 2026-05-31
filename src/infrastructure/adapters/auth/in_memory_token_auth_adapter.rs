//! In-memory opaque-token authentication adapter.
//!
//! Tokens are opaque random strings (not JWTs). Only a SHA-256 hash of each
//! token is stored alongside its claims, so a leak of the in-memory store does
//! not reveal usable tokens. Tokens carry a configurable time-to-live and can
//! be explicitly revoked.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration as StdDuration;

use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{Duration, Utc};
use paladin_ports::output::auth_port::{AuthClaims, AuthError, AuthPort, AuthToken};
use rand::RngCore;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::core::platform::container::user::UserRole;

/// Default token lifetime (24 hours).
const DEFAULT_TTL: StdDuration = StdDuration::from_secs(24 * 60 * 60);

/// In-memory [`AuthPort`] adapter using opaque random bearer tokens.
pub struct InMemoryTokenAuthAdapter {
    /// Maps the SHA-256 hash of a token to its claims.
    store: RwLock<HashMap<String, AuthClaims>>,
    /// How long newly issued tokens remain valid.
    ttl: Duration,
}

impl InMemoryTokenAuthAdapter {
    /// Creates a new adapter with the default 24-hour token lifetime.
    pub fn new() -> Self {
        Self::with_ttl(DEFAULT_TTL)
    }

    /// Creates a new adapter with a custom token lifetime.
    pub fn with_ttl(ttl: StdDuration) -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            ttl: Duration::from_std(ttl).unwrap_or_else(|_| Duration::hours(24)),
        }
    }

    /// Computes the SHA-256 hash (hex string) used as the storage key for a token.
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let digest = hasher.finalize();
        digest.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Generates a fresh opaque token from 32 cryptographically random bytes.
    fn generate_token() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }
}

impl Default for InMemoryTokenAuthAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthPort for InMemoryTokenAuthAdapter {
    async fn issue_token(&self, user_id: Uuid, role: UserRole) -> Result<AuthToken, AuthError> {
        let token = Self::generate_token();
        let expires_at = Utc::now() + self.ttl;
        let claims = AuthClaims {
            user_id,
            role,
            expires_at,
        };

        let hash = Self::hash_token(&token);
        let mut store = self
            .store
            .write()
            .map_err(|e| AuthError::Internal(format!("token store poisoned: {e}")))?;
        store.insert(hash, claims);

        Ok(AuthToken { token, expires_at })
    }

    async fn verify_token(&self, token: &str) -> Result<AuthClaims, AuthError> {
        if token.is_empty() {
            return Err(AuthError::MissingToken);
        }

        let hash = Self::hash_token(token);
        let claims = {
            let store = self
                .store
                .read()
                .map_err(|e| AuthError::Internal(format!("token store poisoned: {e}")))?;
            store.get(&hash).cloned()
        };

        match claims {
            None => Err(AuthError::InvalidToken),
            Some(claims) if claims.expires_at <= Utc::now() => {
                // Proactively evict the expired entry.
                if let Ok(mut store) = self.store.write() {
                    store.remove(&hash);
                }
                Err(AuthError::Expired)
            }
            Some(claims) => Ok(claims),
        }
    }

    async fn revoke_token(&self, token: &str) -> Result<(), AuthError> {
        let hash = Self::hash_token(token);
        let mut store = self
            .store
            .write()
            .map_err(|e| AuthError::Internal(format!("token store poisoned: {e}")))?;
        store.remove(&hash);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn issue_then_verify_round_trip() {
        let adapter = InMemoryTokenAuthAdapter::new();
        let user_id = Uuid::new_v4();

        let issued = adapter.issue_token(user_id, UserRole::Admin).await.unwrap();
        assert!(!issued.token.is_empty());
        assert!(issued.expires_at > Utc::now());

        let claims = adapter.verify_token(&issued.token).await.unwrap();
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.role, UserRole::Admin);
    }

    #[tokio::test]
    async fn unknown_token_is_invalid() {
        let adapter = InMemoryTokenAuthAdapter::new();
        let err = adapter.verify_token("not-a-real-token").await.unwrap_err();
        assert!(matches!(err, AuthError::InvalidToken));
    }

    #[tokio::test]
    async fn empty_token_is_missing() {
        let adapter = InMemoryTokenAuthAdapter::new();
        let err = adapter.verify_token("").await.unwrap_err();
        assert!(matches!(err, AuthError::MissingToken));
    }

    #[tokio::test]
    async fn revoked_token_is_invalid() {
        let adapter = InMemoryTokenAuthAdapter::new();
        let issued = adapter
            .issue_token(Uuid::new_v4(), UserRole::User)
            .await
            .unwrap();

        adapter.revoke_token(&issued.token).await.unwrap();
        let err = adapter.verify_token(&issued.token).await.unwrap_err();
        assert!(matches!(err, AuthError::InvalidToken));
    }

    #[tokio::test]
    async fn expired_token_is_rejected() {
        // Zero TTL means tokens are already expired at verification time.
        let adapter = InMemoryTokenAuthAdapter::with_ttl(StdDuration::from_secs(0));
        let issued = adapter
            .issue_token(Uuid::new_v4(), UserRole::User)
            .await
            .unwrap();

        let err = adapter.verify_token(&issued.token).await.unwrap_err();
        assert!(matches!(err, AuthError::Expired));
    }
}
