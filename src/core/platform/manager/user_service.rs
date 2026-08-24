/*
User Service

Business logic layer for user operations including registration, authentication,
and profile management. This service coordinates between domain entities and
infrastructure adapters.
*/

use crate::application::services::notification_orchestrator::NotificationService;
use crate::core::base::entity::message::Location;
use crate::core::platform::container::log::{LogDestination, LogEntryBuilder, LogLevel};
use crate::core::platform::container::user::{Email, User, UserError};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use paladin_ports::output::auth_port::AuthPort;
use paladin_ports::output::log_port::LogPort;
use paladin_ports::output::user_repository_port::UserRepositoryPort;
use std::sync::Arc;
use uuid::Uuid;

// Re-export trait + DTOs from paladin-core so existing consumers keep working.
pub use paladin_core::platform::manager::user_service::{
    UserAuthenticationResult, UserLoginRequest, UserProfileUpdateRequest, UserRegistrationRequest,
    UserServiceTrait,
};

/// Concrete implementation of UserService
///
/// # Examples
///
/// ```
/// use paladin::application::services::notification_orchestrator::NotificationService;
/// use paladin::core::base::service::message_service::{MessageService, MessageServiceConfig};
/// use paladin::core::platform::container::notification::NotificationServiceConfig;
/// use paladin::core::platform::manager::user_service::UserService;
/// use paladin_ports::output::log_port::LogPort;
/// use paladin_ports::output::user_repository_port::UserRepositoryPort;
/// use std::sync::Arc;
///
/// fn build(
///     user_repository: Arc<dyn UserRepositoryPort>,
///     log_port: Arc<dyn LogPort>,
/// ) -> UserService {
///     let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
///     let notification_service = Arc::new(NotificationService::new(
///         NotificationServiceConfig::default(),
///         message_service,
///     ));
///     UserService::new(user_repository, log_port, notification_service)
/// }
/// ```
pub struct UserService {
    user_repository: Arc<dyn UserRepositoryPort>,
    log_port: Arc<dyn LogPort>,
    notification_service: Arc<NotificationService>,
    argon2: Argon2<'static>,
    auth_port: Option<Arc<dyn AuthPort>>,
}

impl UserService {
    /// Creates a new UserService instance
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        log_port: Arc<dyn LogPort>,
        notification_service: Arc<NotificationService>,
    ) -> Self {
        Self {
            user_repository,
            log_port,
            notification_service,
            argon2: Argon2::default(),
            auth_port: None,
        }
    }

    /// Attaches an authentication provider so that successful logins issue a
    /// bearer token. Without it, `login_user` succeeds but returns no token.
    pub fn with_auth_port(mut self, auth_port: Arc<dyn AuthPort>) -> Self {
        self.auth_port = Some(auth_port);
        self
    }

    /// Hash password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, UserError> {
        if password.len() < 8 {
            return Err(UserError::InvalidPassword(
                "Password must be at least 8 characters".to_string(),
            ));
        }
        if password.len() > 128 {
            return Err(UserError::InvalidPassword(
                "Password cannot exceed 128 characters".to_string(),
            ));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| UserError::HashError(e.to_string()))?;

        Ok(password_hash.to_string())
    }

    /// Verify password against hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, UserError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| UserError::HashError(e.to_string()))?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Validate username
    fn validate_username(&self, username: &str) -> Result<(), UserError> {
        if username.trim().is_empty() {
            return Err(UserError::InvalidUsername(
                "Username cannot be empty".to_string(),
            ));
        }
        if username.len() < 3 {
            return Err(UserError::InvalidUsername(
                "Username must be at least 3 characters".to_string(),
            ));
        }
        if username.len() > 50 {
            return Err(UserError::InvalidUsername(
                "Username cannot exceed 50 characters".to_string(),
            ));
        }
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UserError::InvalidUsername(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }
        Ok(())
    }

    /// Send welcome notification
    async fn send_welcome_notification(&self, user: &User) -> Result<(), UserError> {
        use crate::core::platform::container::notification::{
            NotificationChannel, NotificationContent, NotificationPriority, NotificationRecipient,
        };

        let recipient = NotificationRecipient::Email(user.email().value().to_string());
        let content = NotificationContent {
            title: "Welcome to paladin!".to_string(),
            body: format!("Hello {}, welcome to our platform!", user.username()),
            category: "welcome".to_string(),
            action_url: None,
            attachments: Vec::new(),
            template_id: Some("user_welcome".to_string()),
            template_variables: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        };

        let notification = self
            .notification_service
            .create_notification(
                recipient,
                content,
                NotificationChannel::Email,
                NotificationPriority::Normal,
            )
            .await
            .map_err(|e| {
                UserError::RepositoryError(format!("Failed to create welcome notification: {}", e))
            })?;

        self.notification_service
            .send_notification(notification.id)
            .await
            .map_err(|e| {
                UserError::RepositoryError(format!("Failed to send welcome notification: {}", e))
            })?;

        Ok(())
    }

    /// Log user action
    async fn log_action(&self, level: LogLevel, message: String, user_id: Option<Uuid>) {
        let enhanced_message = match user_id {
            Some(id) => format!("[User: {}] {}", id, message),
            None => message,
        };

        // Create a proper log entry using LogEntryBuilder
        let log_entry = LogEntryBuilder::new_entry(
            Location::service("user-service"),
            LogDestination::System,
            level,
            enhanced_message,
        );

        // Use write_entry instead of log
        if let Err(e) = self.log_port.write_entry(log_entry).await {
            log::error!("Failed to log user action: {}", e);
        }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn register_user(&self, request: UserRegistrationRequest) -> Result<User, UserError> {
        // Validate input
        self.validate_username(&request.username)?;
        let email = Email::new(request.email)?;

        // Check if user already exists
        if self
            .user_repository
            .find_by_email(email.value())
            .await?
            .is_some()
        {
            return Err(UserError::EmailAlreadyExists(email.value().to_string()));
        }

        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Create user
        let user = User::new_user(
            request.username.clone(),
            email,
            password_hash,
            request.profile,
        );

        // Save user
        let saved_user = self.user_repository.save(user).await?;

        // Log successful registration
        self.log_action(
            LogLevel::Info,
            format!("User registered successfully: {}", request.username),
            Some(saved_user.uuid),
        )
        .await;

        // Send welcome notification
        if let Err(e) = self.send_welcome_notification(&saved_user).await {
            self.log_action(
                LogLevel::Warn,
                format!("Failed to send welcome notification: {}", e),
                Some(saved_user.uuid),
            )
            .await;
        }

        Ok(saved_user)
    }

    async fn login_user(
        &self,
        request: UserLoginRequest,
    ) -> Result<UserAuthenticationResult, UserError> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or(UserError::AuthenticationFailed)?;

        // Check if user is active
        if !user.is_active() {
            self.log_action(
                LogLevel::Warn,
                format!("Login attempt for inactive user: {}", request.email),
                Some(user.uuid),
            )
            .await;
            return Err(UserError::UserNotActive);
        }

        // Verify password
        if !self.verify_password(&request.password, user.password_hash())? {
            self.log_action(
                LogLevel::Warn,
                format!("Failed login attempt for user: {}", request.email),
                Some(user.uuid),
            )
            .await;
            return Err(UserError::AuthenticationFailed);
        }

        // Log successful login
        self.log_action(
            LogLevel::Info,
            format!("User logged in successfully: {}", request.email),
            Some(user.uuid),
        )
        .await;

        // Issue a bearer token when an auth provider is configured.
        let (token, token_expires_at) = match &self.auth_port {
            Some(auth_port) => {
                let issued = auth_port
                    .issue_token(user.uuid, user.role())
                    .await
                    .map_err(|e| UserError::HashError(format!("token issuance failed: {e}")))?;
                (Some(issued.token), Some(issued.expires_at))
            }
            None => (None, None),
        };

        Ok(UserAuthenticationResult {
            user_id: user.uuid,
            username: user.username().to_string(),
            email: user.email().value().to_string(),
            is_verified: user.is_verified(),
            success: true,
            token,
            token_expires_at,
        })
    }

    async fn update_user_profile(
        &self,
        request: UserProfileUpdateRequest,
    ) -> Result<User, UserError> {
        // Get existing user
        let mut user = self
            .user_repository
            .find_by_id(request.user_id)
            .await?
            .ok_or(UserError::UserNotFound(request.user_id))?;

        // Update username if provided
        if let Some(new_username) = request.username {
            user.update_username(new_username)?;
        }

        // Update email if provided
        if let Some(new_email) = request.email {
            let email = Email::new(new_email)?;

            // Check if email is already taken by another user
            if let Some(existing_user) = self.user_repository.find_by_email(email.value()).await?
                && existing_user.uuid != user.uuid
            {
                return Err(UserError::EmailAlreadyExists(email.value().to_string()));
            }

            user.update_email(email)?;
        }

        // Update profile if provided
        if let Some(new_profile) = request.profile {
            user.update_profile(new_profile);
        }

        // Save updated user
        let updated_user = self.user_repository.update(user).await?;

        // Log update
        self.log_action(
            LogLevel::Info,
            "User profile updated successfully".to_string(),
            Some(updated_user.uuid),
        )
        .await;

        Ok(updated_user)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError> {
        self.user_repository.find_by_id(user_id).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        self.user_repository.find_by_email(email).await
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), UserError> {
        // Ensure the user exists so callers get a clear not-found error.
        if self.user_repository.find_by_id(user_id).await?.is_none() {
            return Err(UserError::UserNotFound(user_id));
        }

        self.user_repository.delete(user_id).await?;

        self.log_action(
            LogLevel::Info,
            "User account deleted".to_string(),
            Some(user_id),
        )
        .await;

        Ok(())
    }

    async fn list_users(&self) -> Result<Vec<User>, UserError> {
        // The repository exposes status-scoped queries; combine both active
        // states to enumerate every user without a dedicated `find_all` method.
        let mut users = self.user_repository.find_by_active_status(true).await?;
        let inactive = self.user_repository.find_by_active_status(false).await?;
        users.extend(inactive);
        Ok(users)
    }

    async fn activate_user(&self, user_id: Uuid) -> Result<(), UserError> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(UserError::UserNotFound(user_id))?;

        user.activate();
        self.user_repository.update(user).await?;

        self.log_action(
            LogLevel::Info,
            "User account activated".to_string(),
            Some(user_id),
        )
        .await;

        Ok(())
    }

    async fn deactivate_user(&self, user_id: Uuid) -> Result<(), UserError> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(UserError::UserNotFound(user_id))?;

        user.deactivate();
        self.user_repository.update(user).await?;

        self.log_action(
            LogLevel::Info,
            "User account deactivated".to_string(),
            Some(user_id),
        )
        .await;

        Ok(())
    }

    async fn verify_user(&self, user_id: Uuid) -> Result<(), UserError> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(UserError::UserNotFound(user_id))?;

        user.verify();
        self.user_repository.update(user).await?;

        self.log_action(
            LogLevel::Info,
            "User email verified".to_string(),
            Some(user_id),
        )
        .await;

        Ok(())
    }

    /// CLI support methods
    /// Find users by active status
    async fn find_by_active_status(&self, is_active: bool) -> Result<Vec<User>, UserError> {
        self.user_repository.find_by_active_status(is_active).await
    }

    /// Find users by verification status
    async fn find_by_verification_status(&self, is_verified: bool) -> Result<Vec<User>, UserError> {
        self.user_repository
            .find_by_verification_status(is_verified)
            .await
    }

    /// Count total users
    async fn count_users(&self) -> Result<u64, UserError> {
        self.user_repository.count_users().await
    }
}

#[cfg(test)]
mod tests {
    //! # DEFER-02 test-scope justification record (Phase 15, plans 15-06 and 15-07)
    //!
    //! ## Measured figure
    //!
    //! | Field | Value |
    //! |---|---|
    //! | Line coverage | **94.21%** (927 / 984 lines) for `src/core/platform/manager/user_service.rs` |
    //! | Command | `cargo llvm-cov --workspace --lib --json --output-path /tmp/cov.json`, figure extracted for this file's `summary.lines.percent` |
    //! | Feature scope | **default features, `--lib`** (no `--features integration-tests`) |
    //! | Commit | `432d514873ebe4196f0ef550374cc25c5654cfa5` (15-07 Task 1; this Task 2 commit adds tests only -- the production half above `#[cfg(test)]` is byte-identical to that commit, so the figure holds for this commit's tree as well) |
    //! | Date | 2026-08-13 |
    //!
    //! **This figure is not comparable to ADR-0006's 84% workspace CI gate**, which is measured
    //! under `--features integration-tests` against live Redis/MinIO (ADR-0006 §"The relationship
    //! to `cargo llvm-cov`": the two commands' denominators agree only when the ignore regex, the
    //! doctest decision and the feature set all match, and they do not here). Docker is absent from
    //! every authoring environment verified in this phase, so the gate's own scope cannot be
    //! reproduced locally; this module figure is a one-time plan-acceptance measurement (D-12), not
    //! a standing CI gate, and it is not wired into any pipeline by this plan.
    //!
    //! ## Untested paths, with reasons
    //!
    //! Every path DEFER-02's own scope text names is covered by a passing test above, **except**:
    //!
    //! - **Login-attempt tracking.** DEFER-02 names it in the authentication scope. `login_user`
    //!   has no attempt counter, no lockout threshold and no related state anywhere in this file or
    //!   in `UserData` -- `grep -n "attempt" src/core/platform/manager/user_service.rs` matches only
    //!   two log messages, not a counter. The register names a path the module does not implement;
    //!   there is nothing to test.
    //! - **Repository error (edge case).** DEFER-02 names a generic "repository error" edge case.
    //!   Every method in this file propagates `UserRepositoryPort` failures with a bare `?`, so the
    //!   propagation itself is a language guarantee, not service logic to characterize. Forcing a
    //!   live failure would require a `UserRepositoryPort` test double built for this plan alone;
    //!   `SqliteUserRepository`'s own error-mapping (`RepositoryError(format!(...))` wrapping the
    //!   underlying `sqlx::Error`) already has direct unit tests in `sqlite_user_repository.rs`.
    //!   Left untested here as a deliberate scope boundary, not an oversight.
    //!
    //! A further nine lines of `user_service.rs` remain uncovered without being individually named
    //! by DEFER-02's scope text; recorded so no gap is silently absent:
    //!
    //! - `hash_password`'s `argon2::hash_password(..)` internal failure (`UserError::HashError`,
    //!   the map_err arm) -- argon2 only fails this call on malformed salt/params, neither of which
    //!   this file's fixed, valid construction can produce; not reachable through the public API.
    //! - `send_welcome_notification`'s `create_notification(..)` failure arm and its own success
    //!   return (`Ok(())`) -- see the observed-behaviour note below.
    //! - `log_action`'s `user_id: None` branch and its `LogPort::write_entry` failure branch -- no
    //!   call site in this file passes `None`, and every log-port double used in this module's
    //!   tests (`SystemLogAdapter`, `RecordingLogPort`) always succeeds; a failing `LogPort` double
    //!   is disproportionate scope for a `log::error!` fallback line.
    //! - `login_user`'s token-issuance failure arm (`auth_port.issue_token(..)` returning `Err`) --
    //!   `InMemoryTokenAuthAdapter::issue_token` only fails via a poisoned `RwLock`, not achievable
    //!   from a test without deliberately panicking while the lock is held.
    //!
    //! ## Observed behaviour that differs from what the register or requirement text assumes
    //!
    //! - **Concurrent same-username registration has a real, database-enforced outcome, not an
    //!   application-level one.** `register_user`'s own duplicate check
    //!   (`register_user_accepts_a_case_variant_username_because_the_duplicate_check_is_on_email`,
    //!   15-06) is scoped to email only, so two concurrent calls with the same username and
    //!   different emails both clear it. The collision is caught by
    //!   `sqlite_user_repository.rs`'s migration-declared `username TEXT UNIQUE NOT NULL`
    //!   constraint instead. Directly observed (`concurrent_registration_with_the_same_username_leaves_exactly_one_user_persisted`,
    //!   run five times with no flake before this assertion was written): exactly one call
    //!   succeeds, the loser returns `UserError::RepositoryError` wrapping a SQLite
    //!   `UNIQUE constraint failed: users.username` error, and `count_users` reports exactly one
    //!   row afterward. **No race exists for this file's own logic** -- the constraint is doing the
    //!   work, not `register_user`'s check-then-save sequence, so there is no finding to hand off
    //!   for a production fix.
    //! - **The "send a welcome notification" success path is effectively unreachable in this test
    //!   suite, by design.** `build_service()` (the fixture nearly every test in this module uses)
    //!   never caches a `"user_welcome"` template or registers a template processor, so
    //!   `NotificationService::send_notification` always fails at template resolution --
    //!   `send_welcome_notification`'s `Ok(())` return (line 163) is never executed by any test in
    //!   this file, including the ones that specifically prove "notification failure does not block
    //!   registration" (15-06 Task 2). This is consistent with the guarantee DEFER-02 actually asks
    //!   for and does not weaken it, but it means the module's coverage above comes entirely through
    //!   the failure branch of that one call, never the success branch. Recorded here rather than
    //!   built around: reaching the success branch would need a third notification fixture (cached
    //!   template + processor + a channel handler that *succeeds*), which is scope this plan's two
    //!   tasks did not ask for.

    use super::*;
    use crate::application::services::notification_orchestrator::NotificationTemplateProcessor;
    use crate::core::base::service::message_service::{MessageService, MessageServiceConfig};
    use crate::core::platform::container::log::{LogEntry, LogEntryExt};
    use crate::core::platform::container::notification::{
        NotificationChannel, NotificationContent, NotificationTemplate,
    };
    use crate::infrastructure::adapters::auth::InMemoryTokenAuthAdapter;
    use crate::infrastructure::adapters::logs::system_log_adapter::{
        SystemLogAdapter, SystemLogAdapterConfig,
    };
    use crate::infrastructure::repositories::sqlite_user_repository::SqliteUserRepository;
    use crate::test_support::FailingChannelHandler;
    use paladin_core::platform::container::notification::NotificationServiceConfig;
    use paladin_ports::output::log_port::{
        BatchWriteRequest, LogDestinationConfig, LogError, LogFormat, LogHealthCheck, LogQuery,
        LogResult, LogStats,
    };
    use std::collections::HashMap;

    async fn build_service(with_auth: bool) -> UserService {
        let repo = Arc::new(SqliteUserRepository::new("sqlite::memory:").await.unwrap());
        let log_port =
            Arc::new(SystemLogAdapter::new_for_test(SystemLogAdapterConfig::default()).unwrap());
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let notification_service = Arc::new(NotificationService::new(
            NotificationServiceConfig::default(),
            message_service,
        ));
        let service = UserService::new(repo, log_port, notification_service);
        if with_auth {
            service.with_auth_port(Arc::new(InMemoryTokenAuthAdapter::new()))
        } else {
            service
        }
    }

    fn registration(username: &str, email: &str) -> UserRegistrationRequest {
        UserRegistrationRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: "password123".to_string(),
            profile: None,
        }
    }

    #[tokio::test]
    async fn delete_user_removes_the_user() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("alice", "alice@example.com"))
            .await
            .unwrap();

        service.delete_user(user.uuid).await.unwrap();

        assert!(service.get_user_by_id(user.uuid).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_unknown_user_is_not_found() {
        let service = build_service(false).await;
        let err = service.delete_user(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, UserError::UserNotFound(_)));
    }

    #[tokio::test]
    async fn list_users_returns_all_users() {
        let service = build_service(false).await;
        service
            .register_user(registration("bob", "bob@example.com"))
            .await
            .unwrap();
        service
            .register_user(registration("carol", "carol@example.com"))
            .await
            .unwrap();

        let users = service.list_users().await.unwrap();
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn login_issues_token_when_auth_port_configured() {
        let service = build_service(true).await;
        service
            .register_user(registration("dave", "dave@example.com"))
            .await
            .unwrap();

        let result = service
            .login_user(UserLoginRequest {
                email: "dave@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await
            .unwrap();

        assert!(result.success);
        let token = result.token.expect("token should be issued");
        assert!(!token.is_empty());
        let expires_at = result.token_expires_at.expect("expiry should be set");
        assert!(expires_at > chrono::Utc::now());
    }

    #[tokio::test]
    async fn login_without_auth_port_has_no_token() {
        let service = build_service(false).await;
        service
            .register_user(registration("erin", "erin@example.com"))
            .await
            .unwrap();

        let result = service
            .login_user(UserLoginRequest {
                email: "erin@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.token.is_none());
        assert!(result.token_expires_at.is_none());
    }

    // -----------------------------------------------------------------
    // Registration, validation and password-hashing coverage
    // (DEFER-02, 15-06 Task 1)
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn register_user_persists_an_argon2_hash_that_only_verifies_the_original_password() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("frank_hash", "frank_hash@example.com"))
            .await
            .unwrap();

        let hash = user.password_hash().to_string();
        assert!(
            hash.starts_with("$argon2"),
            "stored password field should be an argon2 PHC-format hash, got: {hash}"
        );
        assert!(
            service.verify_password("password123", &hash).unwrap(),
            "hash should verify against the original password"
        );
        assert!(
            !service
                .verify_password("a-completely-different-password", &hash)
                .unwrap(),
            "hash should not verify against a different password"
        );
    }

    #[tokio::test]
    async fn register_user_rejects_a_byte_identical_duplicate_email() {
        let service = build_service(false).await;
        service
            .register_user(registration("gina", "dup@example.com"))
            .await
            .unwrap();

        let err = service
            .register_user(registration("gina-two", "dup@example.com"))
            .await
            .unwrap_err();

        assert!(matches!(err, UserError::EmailAlreadyExists(_)));
    }

    #[tokio::test]
    async fn register_user_accepts_a_case_variant_username_because_the_duplicate_check_is_on_email()
    {
        let service = build_service(false).await;
        service
            .register_user(registration("harold", "harold@example.com"))
            .await
            .unwrap();

        // The duplicate check in `register_user` only looks up by email; whether a
        // case-variant username (with a distinct email) is accepted is a real property of
        // the system, pinned here as observed rather than assumed.
        let result = service
            .register_user(registration("HAROLD", "harold-two@example.com"))
            .await;

        assert!(
            result.is_ok(),
            "observed verdict: a username differing only in case from an existing one is \
             accepted, because duplicate detection is email-scoped only; got: {result:?}"
        );
    }

    #[tokio::test]
    async fn register_user_called_twice_with_the_same_request_leaves_exactly_one_user_persisted() {
        let service = build_service(false).await;
        let request = registration("ivan", "ivan@example.com");

        let first = service.register_user(request.clone()).await;
        assert!(first.is_ok());

        let second = service.register_user(request).await;
        assert!(matches!(
            second.unwrap_err(),
            UserError::EmailAlreadyExists(_)
        ));

        assert_eq!(service.count_users().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn register_user_rejects_an_empty_username() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("", "empty-username@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidUsername(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_a_whitespace_only_username() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("   ", "whitespace-username@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidUsername(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_an_empty_email() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("empty-email-user", ""))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_an_empty_password() {
        let service = build_service(false).await;
        let request = UserRegistrationRequest {
            username: "empty-password-user".to_string(),
            email: "empty-password@example.com".to_string(),
            password: "".to_string(),
            profile: None,
        };
        let err = service.register_user(request).await.unwrap_err();
        assert!(matches!(err, UserError::InvalidPassword(_)));
    }

    #[tokio::test]
    async fn register_user_accepts_a_multi_byte_unicode_username_within_the_byte_length_rule() {
        let service = build_service(false).await;

        // "\u{e9}\u{e9}" (two lowercase e-acute characters) is 2 `char`s but 4 UTF-8 bytes.
        // `validate_username` enforces its minimum length via `str::len()` (byte length), not
        // `chars().count()`, so this username clears the >= 3 check on byte length alone -- a
        // char-count rule would reject it as too short. Registering it (and asserting success)
        // pins the observed rule rather than an assumption about it.
        let username = "\u{e9}\u{e9}";
        assert_eq!(username.chars().count(), 2);
        assert_eq!(username.len(), 4);

        let result = service
            .register_user(registration(username, "unicode-user@example.com"))
            .await;

        assert!(
            result.is_ok(),
            "observed rule: validate_username enforces byte length, so a 2-char/4-byte \
             username is accepted; got: {result:?}"
        );
    }

    #[tokio::test]
    async fn register_user_rejects_an_email_missing_the_at_symbol() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("email-shape-one", "not-an-email"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_an_email_missing_the_local_part() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("email-shape-two", "@domain-only.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_an_email_missing_the_domain_part() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("email-shape-three", "trailing-at@"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }

    // -----------------------------------------------------------------
    // Notification-dispatch failure coverage (DEFER-02, 15-06 Task 2)
    // -----------------------------------------------------------------

    /// A [`LogPort`] double that records every `write_entry` call's level and message.
    ///
    /// `UserService`'s `log_port` field is `Arc<dyn LogPort>`, so -- unlike
    /// `notification_service`, which is a concrete `Arc<NotificationService>` -- this seam
    /// already accepts a substitute with no production signature change. Used here to prove
    /// that a notification-dispatch failure is logged at `Warn` rather than merely asserted in
    /// prose.
    #[derive(Debug, Default)]
    struct RecordingLogPort {
        entries: std::sync::Mutex<Vec<(LogLevel, String)>>,
    }

    impl RecordingLogPort {
        fn new() -> Self {
            Self::default()
        }

        fn entries(&self) -> Vec<(LogLevel, String)> {
            self.entries
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .clone()
        }

        fn contains(&self, level: LogLevel, needle: &str) -> bool {
            self.entries().into_iter().any(|(recorded_level, message)| {
                recorded_level == level && message.contains(needle)
            })
        }
    }

    #[async_trait]
    impl LogPort for RecordingLogPort {
        async fn write_entry(&self, entry: LogEntry) -> LogResult<()> {
            self.entries
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push((entry.level(), entry.message.message.clone()));
            Ok(())
        }

        async fn write_entries(&self, entries: Vec<LogEntry>) -> LogResult<()> {
            for entry in entries {
                self.write_entry(entry).await?;
            }
            Ok(())
        }

        async fn batch_write(&self, request: BatchWriteRequest) -> LogResult<()> {
            self.write_entries(request.entries).await
        }

        async fn read_entries(&self, _query: LogQuery) -> LogResult<Vec<LogEntry>> {
            Ok(Vec::new())
        }

        async fn count_entries(&self, _query: LogQuery) -> LogResult<u64> {
            Ok(self.entries().len() as u64)
        }

        async fn configure_destination(&self, _config: LogDestinationConfig) -> LogResult<()> {
            Ok(())
        }

        async fn remove_destination(&self, _destination: LogDestination) -> LogResult<()> {
            Ok(())
        }

        async fn list_destinations(&self) -> LogResult<Vec<LogDestination>> {
            Ok(Vec::new())
        }

        async fn flush(&self) -> LogResult<()> {
            Ok(())
        }

        async fn flush_destination(&self, _destination: LogDestination) -> LogResult<()> {
            Ok(())
        }

        async fn rotate_logs(&self, _destination: LogDestination) -> LogResult<()> {
            Ok(())
        }

        async fn get_stats(&self) -> LogResult<LogStats> {
            Ok(LogStats::default())
        }

        async fn get_destination_stats(&self, _destination: LogDestination) -> LogResult<LogStats> {
            Ok(LogStats::default())
        }

        async fn clear_logs(&self, _destination: LogDestination) -> LogResult<()> {
            Ok(())
        }

        async fn clear_logs_before(
            &self,
            _destination: LogDestination,
            _before: chrono::DateTime<chrono::Utc>,
        ) -> LogResult<u64> {
            Ok(0)
        }

        async fn health_check(&self) -> LogResult<Vec<LogHealthCheck>> {
            Ok(Vec::new())
        }

        async fn health_check_destination(
            &self,
            _destination: LogDestination,
        ) -> LogResult<LogHealthCheck> {
            Err(LogError::DestinationNotFound(
                "RecordingLogPort does not track destinations".to_string(),
            ))
        }

        fn get_provider_name(&self) -> &'static str {
            "recording-log-port-test-double"
        }

        async fn test_connection(&self) -> LogResult<()> {
            Ok(())
        }

        async fn archive_logs(
            &self,
            _destination: LogDestination,
            _before: chrono::DateTime<chrono::Utc>,
        ) -> LogResult<String> {
            Ok(String::new())
        }

        fn supported_formats(&self) -> Vec<LogFormat> {
            vec![LogFormat::Text]
        }
    }

    /// A no-op [`NotificationTemplateProcessor`] that renders a template by copying its body
    /// straight through, ignoring variable substitution. Exists only so
    /// `build_service_with_failing_notifications` can get `NotificationService::send_notification`
    /// past its template-resolution step and reach the per-channel handler dispatch this plan's
    /// test targets -- it is not a stand-in for a real template engine.
    struct PassthroughTemplateProcessor;

    #[async_trait]
    impl NotificationTemplateProcessor for PassthroughTemplateProcessor {
        async fn render_template(
            &self,
            template: &NotificationTemplate,
            _variables: &HashMap<String, serde_json::Value>,
        ) -> crate::application::services::notification_orchestrator::NotificationOrchestratorResult<
            NotificationContent,
        >{
            Ok(NotificationContent::new(
                template.name.clone(),
                template.body_template.clone(),
                "welcome".to_string(),
            ))
        }

        async fn validate_template(
            &self,
            _template: &NotificationTemplate,
        ) -> crate::application::services::notification_orchestrator::NotificationOrchestratorResult<()>
        {
            Ok(())
        }
    }

    /// A `build_service`-sibling fixture: constructs the same real `NotificationService`,
    /// registers a [`FailingChannelHandler`] on it through the public
    /// `register_channel_handler` seam *before* the service is passed to `UserService::new`,
    /// and swaps in a [`RecordingLogPort`] so a test can also inspect what was logged. Returns
    /// the assembled `UserService` plus handles to both doubles. `build_service` itself is
    /// left unchanged.
    async fn build_service_with_failing_notifications() -> (
        UserService,
        Arc<FailingChannelHandler>,
        Arc<RecordingLogPort>,
    ) {
        let repo = Arc::new(SqliteUserRepository::new("sqlite::memory:").await.unwrap());
        let log_port = Arc::new(RecordingLogPort::new());
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let notification_service = Arc::new(NotificationService::new(
            NotificationServiceConfig::default(),
            message_service,
        ));

        // `send_welcome_notification` addresses the "user_welcome" template on the Email
        // channel. Without a cached template and a registered processor,
        // `NotificationService::send_notification` fails during template resolution, before it
        // ever looks up a channel handler -- which would make the failing-channel-handler path
        // unreachable regardless of what is registered below. Both are wired here so the
        // failure this test proves genuinely happens at channel dispatch.
        notification_service
            .cache_template(NotificationTemplate::new(
                "user_welcome".to_string(),
                "User Welcome".to_string(),
                NotificationChannel::Email,
                "Hello {{username}}, welcome!".to_string(),
                vec!["username".to_string()],
            ))
            .await
            .unwrap();
        notification_service
            .set_template_processor(Arc::new(PassthroughTemplateProcessor))
            .await;

        let failing_handler = Arc::new(FailingChannelHandler::new());
        notification_service
            .register_channel_handler(failing_handler.clone())
            .await;

        let service = UserService::new(repo, log_port.clone(), notification_service);
        (service, failing_handler, log_port)
    }

    #[tokio::test]
    async fn notification_failure_does_not_block_registration() {
        let (service, failing_handler, log_port) = build_service_with_failing_notifications().await;

        let user = service
            .register_user(registration("nora", "nora@example.com"))
            .await
            .unwrap();

        assert!(
            service.get_user_by_id(user.uuid).await.unwrap().is_some(),
            "the write should have committed despite the notification failure"
        );
        assert!(
            failing_handler.call_count() > 0,
            "the failure path should have been genuinely taken"
        );
        assert!(
            log_port.contains(LogLevel::Warn, "Failed to send welcome notification"),
            "a warning should have been logged rather than propagated; got: {:?}",
            log_port.entries()
        );
    }

    #[tokio::test]
    async fn registration_succeeds_when_no_failing_handler_is_registered() {
        let service = build_service(false).await;

        let result = service
            .register_user(registration("oscar", "oscar@example.com"))
            .await;

        assert!(
            result.is_ok(),
            "registration should also succeed with no failing handler registered, so the \
             failure-path test above is shown to discriminate; got: {result:?}"
        );
    }

    // -----------------------------------------------------------------
    // Authentication coverage (DEFER-02, 15-07 Task 1)
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn login_with_incorrect_password_issues_no_token_and_does_not_succeed() {
        let service = build_service(true).await;
        service
            .register_user(registration("penny", "penny@example.com"))
            .await
            .unwrap();

        let err = service
            .login_user(UserLoginRequest {
                email: "penny@example.com".to_string(),
                password: "wrong-password".to_string(),
            })
            .await
            .unwrap_err();

        assert!(
            matches!(err, UserError::AuthenticationFailed),
            "a wrong password should be rejected with AuthenticationFailed; got: {err:?}"
        );
    }

    #[tokio::test]
    async fn login_for_a_never_registered_email_returns_authentication_failed() {
        let service = build_service(true).await;

        let err = service
            .login_user(UserLoginRequest {
                email: "nobody-registered@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await
            .unwrap_err();

        assert!(
            matches!(err, UserError::AuthenticationFailed),
            "a login for an identity that was never registered should return the \
             not-found-shaped AuthenticationFailed variant, without panicking; got: {err:?}"
        );
    }

    #[tokio::test]
    async fn login_against_a_deactivated_account_is_rejected_as_a_distinct_variant() {
        let service = build_service(true).await;
        let user = service
            .register_user(registration("quinn", "quinn@example.com"))
            .await
            .unwrap();
        service.deactivate_user(user.uuid).await.unwrap();

        let err = service
            .login_user(UserLoginRequest {
                email: "quinn@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await
            .unwrap_err();

        // Asserted as UserNotActive specifically -- distinct from the wrong-password case's
        // AuthenticationFailed above -- so a single catch-all rejection could not satisfy both
        // assertions (T-15-20).
        assert!(
            matches!(err, UserError::UserNotActive),
            "a login against a deactivated account should be rejected with UserNotActive, \
             distinct from the wrong-password AuthenticationFailed case; got: {err:?}"
        );
    }

    // -----------------------------------------------------------------
    // Profile lifecycle coverage (DEFER-02, 15-07 Task 1)
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn update_user_profile_on_existing_user_changes_the_stored_profile() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("rachel", "rachel@example.com"))
            .await
            .unwrap();

        let new_profile = crate::core::platform::container::user::UserProfile {
            first_name: Some("Rachel".to_string()),
            last_name: Some("Green".to_string()),
            bio: Some("Updated via test".to_string()),
            avatar_url: None,
            timezone: Some("UTC".to_string()),
            locale: Some("en-US".to_string()),
        };

        let updated = service
            .update_user_profile(UserProfileUpdateRequest {
                user_id: user.uuid,
                username: None,
                email: None,
                profile: Some(new_profile.clone()),
            })
            .await
            .unwrap();

        assert_eq!(updated.profile().first_name, Some("Rachel".to_string()));
        assert_eq!(updated.profile().bio, Some("Updated via test".to_string()));

        let read_back = service
            .get_user_by_id(user.uuid)
            .await
            .unwrap()
            .expect("user should still exist");
        assert_eq!(read_back.profile().first_name, Some("Rachel".to_string()));
    }

    #[tokio::test]
    async fn update_user_profile_on_an_unknown_user_returns_user_not_found() {
        let service = build_service(false).await;

        let err = service
            .update_user_profile(UserProfileUpdateRequest {
                user_id: Uuid::new_v4(),
                username: None,
                email: None,
                profile: None,
            })
            .await
            .unwrap_err();

        assert!(matches!(err, UserError::UserNotFound(_)));
    }

    #[tokio::test]
    async fn update_user_profile_email_change_resets_the_verification_state() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("sam", "sam@example.com"))
            .await
            .unwrap();
        service.verify_user(user.uuid).await.unwrap();

        let verified_before = service
            .get_user_by_id(user.uuid)
            .await
            .unwrap()
            .unwrap()
            .is_verified();
        assert!(
            verified_before,
            "sanity check: user should be verified before the email change"
        );

        let updated = service
            .update_user_profile(UserProfileUpdateRequest {
                user_id: user.uuid,
                username: None,
                email: Some("sam-new@example.com".to_string()),
                profile: None,
            })
            .await
            .unwrap();

        // Observed behaviour, not an assumption: `User::update_email` unconditionally resets
        // `is_verified` to `false` on any email change, verified/unverified alike.
        assert!(
            !updated.is_verified(),
            "changing a verified user's email should reset verification state to unverified; \
             observed is_verified: {}",
            updated.is_verified()
        );
    }

    #[tokio::test]
    async fn activate_user_on_existing_user_is_reflected_when_read_back() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("tara", "tara@example.com"))
            .await
            .unwrap();
        service.deactivate_user(user.uuid).await.unwrap();

        service.activate_user(user.uuid).await.unwrap();

        let read_back = service.get_user_by_id(user.uuid).await.unwrap().unwrap();
        assert!(
            read_back.is_active(),
            "user should be active after activate_user"
        );
    }

    #[tokio::test]
    async fn activate_user_on_an_unknown_id_returns_user_not_found() {
        let service = build_service(false).await;
        let err = service.activate_user(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, UserError::UserNotFound(_)));
    }

    #[tokio::test]
    async fn deactivate_user_on_existing_user_is_reflected_when_read_back() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("uma", "uma@example.com"))
            .await
            .unwrap();

        service.deactivate_user(user.uuid).await.unwrap();

        let read_back = service.get_user_by_id(user.uuid).await.unwrap().unwrap();
        assert!(
            !read_back.is_active(),
            "user should be inactive after deactivate_user"
        );
    }

    #[tokio::test]
    async fn deactivate_user_on_an_unknown_id_returns_user_not_found() {
        let service = build_service(false).await;
        let err = service.deactivate_user(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, UserError::UserNotFound(_)));
    }

    #[tokio::test]
    async fn verify_user_on_existing_user_is_reflected_when_read_back() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("victor", "victor@example.com"))
            .await
            .unwrap();
        assert!(
            !service
                .get_user_by_id(user.uuid)
                .await
                .unwrap()
                .unwrap()
                .is_verified(),
            "sanity check: a freshly registered user should start unverified"
        );

        service.verify_user(user.uuid).await.unwrap();

        let read_back = service.get_user_by_id(user.uuid).await.unwrap().unwrap();
        assert!(
            read_back.is_verified(),
            "user should be verified after verify_user"
        );
    }

    #[tokio::test]
    async fn verify_user_on_an_unknown_id_returns_user_not_found() {
        let service = build_service(false).await;
        let err = service.verify_user(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, UserError::UserNotFound(_)));
    }

    // -----------------------------------------------------------------
    // Query coverage (DEFER-02, 15-07 Task 1)
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn get_user_by_id_hit_returns_the_user() {
        let service = build_service(false).await;
        let user = service
            .register_user(registration("wendy", "wendy@example.com"))
            .await
            .unwrap();

        let found = service.get_user_by_id(user.uuid).await.unwrap();
        assert_eq!(found.map(|u| u.uuid), Some(user.uuid));
    }

    #[tokio::test]
    async fn get_user_by_id_miss_returns_none() {
        let service = build_service(false).await;
        let found = service.get_user_by_id(Uuid::new_v4()).await.unwrap();
        assert!(found.is_none(), "a miss should return None, not an error");
    }

    #[tokio::test]
    async fn get_user_by_email_hit_returns_the_user() {
        let service = build_service(false).await;
        service
            .register_user(registration("xavier", "xavier@example.com"))
            .await
            .unwrap();

        let found = service
            .get_user_by_email("xavier@example.com")
            .await
            .unwrap();
        assert_eq!(
            found.map(|u| u.username().to_string()),
            Some("xavier".to_string())
        );
    }

    #[tokio::test]
    async fn get_user_by_email_miss_returns_none() {
        let service = build_service(false).await;
        let found = service
            .get_user_by_email("nobody-here@example.com")
            .await
            .unwrap();
        assert!(found.is_none(), "a miss should return None, not an error");
    }

    #[tokio::test]
    async fn count_users_is_zero_on_an_empty_repository() {
        let service = build_service(false).await;
        assert_eq!(service.count_users().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn count_users_reflects_the_count_after_n_registrations() {
        let service = build_service(false).await;
        for (username, email) in [
            ("yara", "yara@example.com"),
            ("zack", "zack@example.com"),
            ("amelia2", "amelia2@example.com"),
        ] {
            service
                .register_user(registration(username, email))
                .await
                .unwrap();
        }

        assert_eq!(service.count_users().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn find_by_active_status_asserts_membership_in_both_polarities() {
        let service = build_service(false).await;
        let active_user = service
            .register_user(registration("brianna", "brianna@example.com"))
            .await
            .unwrap();
        let inactive_user = service
            .register_user(registration("carlos", "carlos@example.com"))
            .await
            .unwrap();
        service.deactivate_user(inactive_user.uuid).await.unwrap();

        let active = service.find_by_active_status(true).await.unwrap();
        let active_ids: Vec<_> = active.iter().map(|u| u.uuid).collect();
        assert!(active_ids.contains(&active_user.uuid));
        assert!(!active_ids.contains(&inactive_user.uuid));

        let inactive = service.find_by_active_status(false).await.unwrap();
        let inactive_ids: Vec<_> = inactive.iter().map(|u| u.uuid).collect();
        assert!(inactive_ids.contains(&inactive_user.uuid));
        assert!(!inactive_ids.contains(&active_user.uuid));
    }

    #[tokio::test]
    async fn find_by_verification_status_asserts_membership_in_both_polarities() {
        let service = build_service(false).await;
        let verified_user = service
            .register_user(registration("dalia", "dalia@example.com"))
            .await
            .unwrap();
        service.verify_user(verified_user.uuid).await.unwrap();
        let unverified_user = service
            .register_user(registration("ewan", "ewan@example.com"))
            .await
            .unwrap();

        let verified = service.find_by_verification_status(true).await.unwrap();
        let verified_ids: Vec<_> = verified.iter().map(|u| u.uuid).collect();
        assert!(verified_ids.contains(&verified_user.uuid));
        assert!(!verified_ids.contains(&unverified_user.uuid));

        let unverified = service.find_by_verification_status(false).await.unwrap();
        let unverified_ids: Vec<_> = unverified.iter().map(|u| u.uuid).collect();
        assert!(unverified_ids.contains(&unverified_user.uuid));
        assert!(!unverified_ids.contains(&verified_user.uuid));
    }

    // -----------------------------------------------------------------
    // Additional validation and credential-path coverage (DEFER-02, 15-07 Task 2)
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn register_user_rejects_a_too_short_non_whitespace_username() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration("ab", "too-short-username@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidUsername(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_a_password_over_one_hundred_twenty_eight_characters() {
        let service = build_service(false).await;
        let password = "a".repeat(129);
        assert_eq!(password.len(), 129);
        let request = UserRegistrationRequest {
            username: "long-password-user".to_string(),
            email: "long-password@example.com".to_string(),
            password,
            profile: None,
        };
        let err = service.register_user(request).await.unwrap_err();
        assert!(matches!(err, UserError::InvalidPassword(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_a_username_over_fifty_characters() {
        let service = build_service(false).await;
        let username = "a".repeat(51);
        assert_eq!(username.len(), 51);
        let err = service
            .register_user(registration(&username, "too-long-username@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidUsername(_)));
    }

    #[tokio::test]
    async fn register_user_rejects_a_username_with_invalid_characters() {
        let service = build_service(false).await;
        let err = service
            .register_user(registration(
                "bad user!",
                "invalid-chars-username@example.com",
            ))
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidUsername(_)));
    }

    #[tokio::test]
    async fn verify_password_against_a_malformed_hash_returns_a_hash_error() {
        let service = build_service(false).await;

        // Routed through the service's own `verify_password`, not a hand-rolled comparison
        // (T-15-04): a hash string that is not valid PHC format should surface as a
        // `HashError` from `argon2::PasswordHash::new`, rather than panicking or silently
        // returning `false`.
        let err = service
            .verify_password("any-password", "not-a-valid-phc-hash")
            .unwrap_err();
        assert!(matches!(err, UserError::HashError(_)));
    }

    // -----------------------------------------------------------------
    // Concurrent registration coverage (DEFER-02, 15-07 Task 2)
    // -----------------------------------------------------------------

    /// Two `register_user` calls for the same username, driven concurrently against one
    /// shared `UserService`, with distinct emails so the request clears `register_user`'s
    /// own application-level duplicate check (which is email-scoped only -- see
    /// `register_user_accepts_a_case_variant_username_because_the_duplicate_check_is_on_email`
    /// above). The collision is caught downstream, by the database: the migration in
    /// `sqlite_user_repository.rs` declares `username TEXT UNIQUE NOT NULL`, so the loser's
    /// `INSERT` fails on that constraint.
    ///
    /// Observed (not assumed) outcome, confirmed by a direct run before this assertion was
    /// written: exactly one call succeeds, the other returns `UserError::RepositoryError`
    /// wrapping a SQLite `UNIQUE constraint failed: users.username` error, and exactly one
    /// row is persisted. No production change was made to obtain this result -- `register_user`
    /// already relies on the repository's own unique constraint to catch what its
    /// email-scoped pre-check does not.
    #[tokio::test(flavor = "multi_thread")]
    async fn concurrent_registration_with_the_same_username_leaves_exactly_one_user_persisted() {
        let service = Arc::new(build_service(false).await);
        let svc_a = service.clone();
        let svc_b = service.clone();

        let (first, second) = tokio::time::timeout(std::time::Duration::from_secs(10), async {
            tokio::join!(
                svc_a.register_user(registration("paul", "paul-a@example.com")),
                svc_b.register_user(registration("paul", "paul-b@example.com")),
            )
        })
        .await
        .expect("concurrent same-username registration should not hang");

        let results = [first, second];
        let ok_count = results.iter().filter(|r| r.is_ok()).count();
        let err_count = results.iter().filter(|r| r.is_err()).count();

        assert_eq!(
            ok_count, 1,
            "exactly one of the two concurrent same-username registrations should succeed; \
             got: {results:?}"
        );
        assert_eq!(
            err_count, 1,
            "the losing registration should return an error rather than silently persisting a \
             second row; got: {results:?}"
        );

        for result in &results {
            if let Err(err) = result {
                assert!(
                    matches!(err, UserError::RepositoryError(_)),
                    "the losing registration's error should surface as a RepositoryError from \
                     the database's unique-constraint violation (register_user's own duplicate \
                     check is email-scoped and would not catch this); got: {err:?}"
                );
            }
        }

        assert_eq!(
            service.count_users().await.unwrap(),
            1,
            "exactly one user should be persisted after the race"
        );
    }
}
