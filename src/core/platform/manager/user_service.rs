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
            eprintln!("Failed to log user action: {}", e);
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
    use super::*;
    use crate::core::base::service::message_service::{MessageService, MessageServiceConfig};
    use crate::infrastructure::adapters::auth::InMemoryTokenAuthAdapter;
    use crate::infrastructure::adapters::logs::system_log_adapter::{
        SystemLogAdapter, SystemLogAdapterConfig,
    };
    use crate::infrastructure::repositories::sqlite_user_repository::SqliteUserRepository;
    use paladin_core::platform::container::notification::NotificationServiceConfig;

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
}
