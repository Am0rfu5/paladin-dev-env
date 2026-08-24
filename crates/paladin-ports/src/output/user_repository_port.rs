/// User persistence repository port.
///
/// Defines the abstract contract that SQL-backed (or any other)
/// user-store adapters must implement. Lives in `paladin-ports` so
/// `paladin-storage` can implement it without depending on the facade.
use async_trait::async_trait;
use paladin_core::platform::container::user::{User, UserError};
use uuid::Uuid;

/// Repository port for [`User`] persistence.
///
/// # Examples
///
/// ```rust
/// use paladin_ports::output::user_repository_port::UserRepositoryPort;
/// use paladin_core::platform::container::user::{User, UserData, UserError, UserProfile, UserRole, Email};
///
/// async fn register_user(
///     repo: &dyn UserRepositoryPort,
///     username: &str,
///     email: &str,
/// ) -> Result<User, UserError> {
///     if repo.email_exists(email).await? {
///         return Err(UserError::InvalidEmail(email.to_string()));
///     }
///
///     let data = UserData {
///         username: username.to_string(),
///         email: Email::new(email.to_string())?,
///         password_hash: "hashed-password".to_string(),
///         is_active: true,
///         is_verified: false,
///         role: UserRole::User,
///         profile: UserProfile::default(),
///     };
///
///     repo.save(User::new(data, None)).await
/// }
/// ```
#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    /// Find a user by their UUID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;

    /// Find a user by email address.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;

    /// Find a user by username.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError>;

    /// Persist a new user; returns the saved entity.
    async fn save(&self, user: User) -> Result<User, UserError>;

    /// Overwrite an existing user; returns the updated entity.
    async fn update(&self, user: User) -> Result<User, UserError>;

    /// Remove a user by UUID.
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;

    /// Return `true` if the given email address is already taken.
    async fn email_exists(&self, email: &str) -> Result<bool, UserError>;

    /// Return `true` if the given username is already taken.
    async fn username_exists(&self, username: &str) -> Result<bool, UserError>;

    /// Return all users with the given active status.
    async fn find_by_active_status(&self, is_active: bool) -> Result<Vec<User>, UserError>;

    /// Return all users with the given verification status.
    async fn find_by_verification_status(&self, is_verified: bool) -> Result<Vec<User>, UserError>;

    /// Return the total number of stored users.
    async fn count_users(&self) -> Result<u64, UserError>;
}
