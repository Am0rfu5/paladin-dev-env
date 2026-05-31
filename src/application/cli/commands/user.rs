/*
CLI Commands for User Management

Command-line interface for user operations, useful for administrative tasks
and testing.
*/

use crate::core::platform::container::user::{User, UserProfile};
use crate::core::platform::manager::user_service::{
    UserLoginRequest, UserProfileUpdateRequest, UserRegistrationRequest, UserService,
    UserServiceTrait,
};
use clap::{Args, Subcommand};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    /// Register a new user
    Register(RegisterArgs),
    /// Login user
    Login(LoginArgs),
    /// Get user information
    Get(GetUserArgs),
    /// Update user profile
    Update(UpdateUserArgs),
    /// List users
    List(ListUsersArgs),
    /// Activate user
    Activate(ActivateUserArgs),
    /// Deactivate user
    Deactivate(DeactivateUserArgs),
    /// Verify user
    Verify(VerifyUserArgs),
}

#[derive(Debug, Args)]
pub struct RegisterArgs {
    /// Username
    #[arg(short, long)]
    pub username: String,

    /// Email address
    #[arg(short, long)]
    pub email: String,

    /// Password
    #[arg(short, long)]
    pub password: String,

    /// First name
    #[arg(long)]
    pub first_name: Option<String>,

    /// Last name
    #[arg(long)]
    pub last_name: Option<String>,

    /// Bio
    #[arg(long)]
    pub bio: Option<String>,

    /// Timezone
    #[arg(long, default_value = "UTC")]
    pub timezone: String,

    /// Locale
    #[arg(long, default_value = "en-US")]
    pub locale: String,
}

#[derive(Debug, Args)]
pub struct LoginArgs {
    /// Email address
    #[arg(short, long)]
    pub email: String,

    /// Password
    #[arg(short, long)]
    pub password: String,
}

#[derive(Debug, Args)]
pub struct GetUserArgs {
    /// User ID or email
    #[arg(short, long)]
    pub identifier: String,
}

#[derive(Debug, Args)]
pub struct UpdateUserArgs {
    /// User ID
    #[arg(short, long)]
    pub user_id: String,

    /// New username
    #[arg(long)]
    pub username: Option<String>,

    /// New email
    #[arg(long)]
    pub email: Option<String>,

    /// First name
    #[arg(long)]
    pub first_name: Option<String>,

    /// Last name
    #[arg(long)]
    pub last_name: Option<String>,

    /// Bio
    #[arg(long)]
    pub bio: Option<String>,

    /// Avatar URL
    #[arg(long)]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Args)]
pub struct ListUsersArgs {
    /// Filter by active status
    #[arg(long)]
    pub active: Option<bool>,

    /// Filter by verification status
    #[arg(long)]
    pub verified: Option<bool>,

    /// Limit number of results
    #[arg(short, long, default_value = "10")]
    pub limit: u32,
}

#[derive(Debug, Args)]
pub struct ActivateUserArgs {
    /// User ID
    #[arg(short, long)]
    pub user_id: String,
}

#[derive(Debug, Args)]
pub struct DeactivateUserArgs {
    /// User ID
    #[arg(short, long)]
    pub user_id: String,
}

#[derive(Debug, Args)]
pub struct VerifyUserArgs {
    /// User ID
    #[arg(short, long)]
    pub user_id: String,
}

/// User command handler
pub struct UserCommandHandler<T: UserServiceTrait = UserService> {
    user_service: Arc<T>,
}

impl<T: UserServiceTrait> UserCommandHandler<T> {
    pub fn new(user_service: Arc<T>) -> Self {
        Self { user_service }
    }

    pub async fn handle_command(
        &self,
        command: UserCommands,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            UserCommands::Register(args) => self.register_user(args).await,
            UserCommands::Login(args) => self.login_user(args).await,
            UserCommands::Get(args) => self.get_user(args).await,
            UserCommands::Update(args) => self.update_user(args).await,
            UserCommands::List(args) => self.list_users(args).await,
            UserCommands::Activate(args) => self.activate_user(args).await,
            UserCommands::Deactivate(args) => self.deactivate_user(args).await,
            UserCommands::Verify(args) => self.verify_user(args).await,
        }
    }

    async fn register_user(&self, args: RegisterArgs) -> Result<(), Box<dyn std::error::Error>> {
        let profile = Some(UserProfile {
            first_name: args.first_name,
            last_name: args.last_name,
            bio: args.bio,
            avatar_url: None,
            timezone: Some(args.timezone),
            locale: Some(args.locale),
        });

        let request = UserRegistrationRequest {
            username: args.username,
            email: args.email,
            password: args.password,
            profile,
        };

        let user = self.user_service.register_user(request).await?;

        println!("✅ User registered successfully!");
        println!("   ID: {}", user.uuid);
        println!("   Username: {}", user.username());
        println!("   Email: {}", user.email());
        println!("   Active: {}", user.is_active());
        println!("   Verified: {}", user.is_verified());

        Ok(())
    }

    async fn login_user(&self, args: LoginArgs) -> Result<(), Box<dyn std::error::Error>> {
        let request = UserLoginRequest {
            email: args.email,
            password: args.password,
        };

        let result = self.user_service.login_user(request).await?;

        if result.success {
            println!("✅ Login successful!");
            println!("   User ID: {}", result.user_id);
            println!("   Username: {}", result.username);
            println!("   Email: {}", result.email);
            println!("   Verified: {}", result.is_verified);
        } else {
            println!("❌ Login failed!");
        }

        Ok(())
    }

    async fn get_user(&self, args: GetUserArgs) -> Result<(), Box<dyn std::error::Error>> {
        let user = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
            self.user_service.get_user_by_id(uuid).await?
        } else {
            self.user_service
                .get_user_by_email(&args.identifier)
                .await?
        };

        match user {
            Some(user) => {
                println!("👤 User Information:");
                println!("   ID: {}", user.uuid);
                println!("   Username: {}", user.username());
                println!("   Email: {}", user.email());
                println!("   Active: {}", user.is_active());
                println!("   Verified: {}", user.is_verified());
                println!(
                    "   Created: {}",
                    user.created.format("%Y-%m-%d %H:%M:%S UTC")
                );
                println!(
                    "   Modified: {}",
                    user.modified.format("%Y-%m-%d %H:%M:%S UTC")
                );

                let profile = user.profile();
                if profile.first_name.is_some() || profile.last_name.is_some() {
                    println!(
                        "   Name: {} {}",
                        profile.first_name.as_deref().unwrap_or(""),
                        profile.last_name.as_deref().unwrap_or("")
                    );
                }
                if let Some(bio) = &profile.bio {
                    println!("   Bio: {}", bio);
                }
                if let Some(timezone) = &profile.timezone {
                    println!("   Timezone: {}", timezone);
                }
                if let Some(locale) = &profile.locale {
                    println!("   Locale: {}", locale);
                }
            }
            None => println!("❌ User not found: {}", args.identifier),
        }

        Ok(())
    }

    async fn update_user(&self, args: UpdateUserArgs) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = Uuid::parse_str(&args.user_id)?;

        let profile = if args.first_name.is_some()
            || args.last_name.is_some()
            || args.bio.is_some()
            || args.avatar_url.is_some()
        {
            // Get current user to preserve existing values
            let current_user = self
                .user_service
                .get_user_by_id(user_id)
                .await?
                .ok_or("User not found")?;

            Some(UserProfile {
                first_name: args
                    .first_name
                    .or(current_user.profile().first_name.clone()),
                last_name: args.last_name.or(current_user.profile().last_name.clone()),
                bio: args.bio.or(current_user.profile().bio.clone()),
                avatar_url: args
                    .avatar_url
                    .or(current_user.profile().avatar_url.clone()),
                timezone: current_user.profile().timezone.clone(),
                locale: current_user.profile().locale.clone(),
            })
        } else {
            None
        };

        let request = UserProfileUpdateRequest {
            user_id,
            username: args.username,
            email: args.email,
            profile,
        };

        let user = self.user_service.update_user_profile(request).await?;

        println!("✅ User updated successfully!");
        println!("   ID: {}", user.uuid);
        println!("   Username: {}", user.username());
        println!("   Email: {}", user.email());

        Ok(())
    }

    async fn list_users(&self, args: ListUsersArgs) -> Result<(), Box<dyn std::error::Error>> {
        // This would require additional methods in UserService and UserRepository
        // For now, we'll implement a basic version

        println!("📋 User List:");

        if let Some(active) = args.active {
            let users = self.user_service.find_by_active_status(active).await?;
            self.print_user_list(&users, args.limit);
        } else if let Some(verified) = args.verified {
            let users = self
                .user_service
                .find_by_verification_status(verified)
                .await?;
            self.print_user_list(&users, args.limit);
        } else {
            println!("   Use --active or --verified filters to list users");
        }

        Ok(())
    }

    fn print_user_list(&self, users: &[User], limit: u32) {
        let displayed_users = users.iter().take(limit as usize);

        for user in displayed_users {
            println!(
                "   • {} ({}) - {} - Active: {} - Verified: {}",
                user.username(),
                user.email(),
                user.uuid,
                user.is_active(),
                user.is_verified()
            );
        }

        if users.len() > limit as usize {
            println!("   ... and {} more users", users.len() - limit as usize);
        }

        println!("   Total: {} users", users.len());
    }

    async fn activate_user(
        &self,
        args: ActivateUserArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = Uuid::parse_str(&args.user_id)?;
        self.user_service.activate_user(user_id).await?;
        println!("✅ User activated successfully: {}", user_id);
        Ok(())
    }

    async fn deactivate_user(
        &self,
        args: DeactivateUserArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = Uuid::parse_str(&args.user_id)?;
        self.user_service.deactivate_user(user_id).await?;
        println!("✅ User deactivated successfully: {}", user_id);
        Ok(())
    }

    async fn verify_user(&self, args: VerifyUserArgs) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = Uuid::parse_str(&args.user_id)?;
        self.user_service.verify_user(user_id).await?;
        println!("✅ User verified successfully: {}", user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::user::{Email, User, UserError};
    use crate::core::platform::manager::user_service::UserAuthenticationResult;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock UserService for testing
    struct MockUserService {
        users: Mutex<HashMap<Uuid, User>>,
        login_results: Mutex<HashMap<String, Result<UserAuthenticationResult, UserError>>>,
        should_fail: Mutex<bool>,
    }

    impl MockUserService {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
                login_results: Mutex::new(HashMap::new()),
                should_fail: Mutex::new(false),
            }
        }

        fn add_user(&self, user: User) {
            self.users.lock().unwrap().insert(user.uuid, user);
        }

        fn set_should_fail(&self, fail: bool) {
            *self.should_fail.lock().unwrap() = fail;
        }

        #[allow(dead_code)]
        fn set_login_result(
            &self,
            email: &str,
            result: Result<UserAuthenticationResult, UserError>,
        ) {
            self.login_results
                .lock()
                .unwrap()
                .insert(email.to_string(), result);
        }
    }

    #[async_trait]
    impl UserServiceTrait for MockUserService {
        async fn register_user(&self, request: UserRegistrationRequest) -> Result<User, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }

            let user = User::new_user(
                request.username,
                Email::new(request.email).unwrap(),
                "mock_hash".to_string(),
                request.profile,
            );
            self.add_user(user.clone());
            Ok(user)
        }

        async fn login_user(
            &self,
            request: UserLoginRequest,
        ) -> Result<UserAuthenticationResult, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::AuthenticationFailed);
            }

            if let Some(result) = self.login_results.lock().unwrap().remove(&request.email) {
                result
            } else {
                Ok(UserAuthenticationResult {
                    user_id: Uuid::new_v4(),
                    username: "testuser".to_string(),
                    email: request.email,
                    is_verified: true,
                    success: true,
                    token: None,
                    token_expires_at: None,
                })
            }
        }

        async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self.users.lock().unwrap().get(&user_id).cloned())
        }

        async fn delete_user(&self, user_id: Uuid) -> Result<(), UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            self.users.lock().unwrap().remove(&user_id);
            Ok(())
        }

        async fn list_users(&self) -> Result<Vec<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self.users.lock().unwrap().values().cloned().collect())
        }

        async fn update_user_profile(
            &self,
            request: UserProfileUpdateRequest,
        ) -> Result<User, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }

            let existing_user = self
                .users
                .lock()
                .unwrap()
                .get(&request.user_id)
                .cloned()
                .ok_or(UserError::UserNotFound(request.user_id))?;

            let updated_username = request
                .username
                .unwrap_or_else(|| existing_user.username().to_string());
            let updated_email = request
                .email
                .unwrap_or_else(|| existing_user.email().value().to_string());
            let updated_profile = request
                .profile
                .unwrap_or_else(|| existing_user.profile().clone());

            let updated_user = User::new_user(
                updated_username,
                Email::new(updated_email).unwrap(),
                "mock_hash".to_string(),
                Some(updated_profile),
            );

            self.add_user(updated_user.clone());
            Ok(updated_user)
        }

        async fn activate_user(&self, _user_id: Uuid) -> Result<(), UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(())
        }

        async fn deactivate_user(&self, _user_id: Uuid) -> Result<(), UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(())
        }

        async fn verify_user(&self, _user_id: Uuid) -> Result<(), UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(())
        }

        async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self
                .users
                .lock()
                .unwrap()
                .values()
                .find(|u| u.email().value() == email)
                .cloned())
        }

        async fn find_by_active_status(&self, is_active: bool) -> Result<Vec<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self
                .users
                .lock()
                .unwrap()
                .values()
                .filter(|u| u.is_active() == is_active)
                .cloned()
                .collect())
        }

        async fn find_by_verification_status(
            &self,
            is_verified: bool,
        ) -> Result<Vec<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self
                .users
                .lock()
                .unwrap()
                .values()
                .filter(|u| u.is_verified() == is_verified)
                .cloned()
                .collect())
        }

        async fn count_users(&self) -> Result<u64, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }
            Ok(self.users.lock().unwrap().len() as u64)
        }
    }

    #[tokio::test]
    async fn test_register_user_command() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = RegisterArgs {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            bio: Some("Test bio".to_string()),
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        };

        let result = handler.register_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_user_command_failure() {
        let mock_service = Arc::new(MockUserService::new());
        mock_service.set_should_fail(true);
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = RegisterArgs {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: None,
            last_name: None,
            bio: None,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        };

        let result = handler.register_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_user_command_success() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = LoginArgs {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = handler.login_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_user_command_failure() {
        let mock_service = Arc::new(MockUserService::new());
        mock_service.set_should_fail(true);
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = LoginArgs {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = handler.login_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_by_uuid() {
        let mock_service = Arc::new(MockUserService::new());
        let user = User::new_user(
            "testuser".to_string(),
            Email::new("test@example.com".to_string()).unwrap(),
            "mock_hash".to_string(),
            None,
        );
        let user_id = user.uuid;
        mock_service.add_user(user);

        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = GetUserArgs {
            identifier: user_id.to_string(),
        };

        let result = handler.get_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        let mock_service = Arc::new(MockUserService::new());
        let user = User::new_user(
            "testuser".to_string(),
            Email::new("test@example.com".to_string()).unwrap(),
            "mock_hash".to_string(),
            None,
        );
        mock_service.add_user(user);

        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = GetUserArgs {
            identifier: "test@example.com".to_string(),
        };

        let result = handler.get_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = GetUserArgs {
            identifier: Uuid::new_v4().to_string(),
        };

        let result = handler.get_user(args).await;
        assert!(result.is_ok()); // Should not error, just print "not found"
    }

    #[tokio::test]
    async fn test_update_user_command() {
        let mock_service = Arc::new(MockUserService::new());
        let user = User::new_user(
            "testuser".to_string(),
            Email::new("test@example.com".to_string()).unwrap(),
            "mock_hash".to_string(),
            None,
        );
        let user_id = user.uuid;
        mock_service.add_user(user);

        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = UpdateUserArgs {
            user_id: user_id.to_string(),
            username: Some("newusername".to_string()),
            email: Some("newemail@example.com".to_string()),
            first_name: Some("New".to_string()),
            last_name: Some("Name".to_string()),
            bio: Some("New bio".to_string()),
            avatar_url: None,
        };

        let result = handler.update_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_invalid_uuid() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = UpdateUserArgs {
            user_id: "invalid-uuid".to_string(),
            username: None,
            email: None,
            first_name: None,
            last_name: None,
            bio: None,
            avatar_url: None,
        };

        let result = handler.update_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_users_by_active() {
        let mock_service = Arc::new(MockUserService::new());
        let user1 = User::new_user(
            "user1".to_string(),
            Email::new("user1@example.com".to_string()).unwrap(),
            "hash".to_string(),
            None,
        );
        let user2 = User::new_user(
            "user2".to_string(),
            Email::new("user2@example.com".to_string()).unwrap(),
            "hash".to_string(),
            None,
        );
        mock_service.add_user(user1);
        mock_service.add_user(user2);

        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = ListUsersArgs {
            active: Some(true),
            verified: None,
            limit: 10,
        };

        let result = handler.list_users(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_users_by_verified() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = ListUsersArgs {
            active: None,
            verified: Some(true),
            limit: 10,
        };

        let result = handler.list_users(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_users_no_filter() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = ListUsersArgs {
            active: None,
            verified: None,
            limit: 10,
        };

        let result = handler.list_users(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_activate_user_command() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let user_id = Uuid::new_v4();
        let args = ActivateUserArgs {
            user_id: user_id.to_string(),
        };

        let result = handler.activate_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_activate_user_invalid_uuid() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = ActivateUserArgs {
            user_id: "invalid-uuid".to_string(),
        };

        let result = handler.activate_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_deactivate_user_command() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let user_id = Uuid::new_v4();
        let args = DeactivateUserArgs {
            user_id: user_id.to_string(),
        };

        let result = handler.deactivate_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deactivate_user_invalid_uuid() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = DeactivateUserArgs {
            user_id: "invalid-uuid".to_string(),
        };

        let result = handler.deactivate_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_user_command() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let user_id = Uuid::new_v4();
        let args = VerifyUserArgs {
            user_id: user_id.to_string(),
        };

        let result = handler.verify_user(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_user_invalid_uuid() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let args = VerifyUserArgs {
            user_id: "invalid-uuid".to_string(),
        };

        let result = handler.verify_user(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_command_register() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let command = UserCommands::Register(RegisterArgs {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: None,
            last_name: None,
            bio: None,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        });

        let result = handler.handle_command(command).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_command_login() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let command = UserCommands::Login(LoginArgs {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });

        let result = handler.handle_command(command).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_user_list() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let users = vec![
            User::new_user(
                "user1".to_string(),
                Email::new("user1@example.com".to_string()).unwrap(),
                "hash".to_string(),
                None,
            ),
            User::new_user(
                "user2".to_string(),
                Email::new("user2@example.com".to_string()).unwrap(),
                "hash".to_string(),
                None,
            ),
        ];

        // This just tests that the function doesn't panic
        handler.print_user_list(&users, 10);
    }

    #[test]
    fn test_print_user_list_with_limit() {
        let mock_service = Arc::new(MockUserService::new());
        let handler = UserCommandHandler {
            user_service: mock_service.clone(),
        };

        let users = vec![
            User::new_user(
                "user1".to_string(),
                Email::new("user1@example.com".to_string()).unwrap(),
                "hash".to_string(),
                None,
            ),
            User::new_user(
                "user2".to_string(),
                Email::new("user2@example.com".to_string()).unwrap(),
                "hash".to_string(),
                None,
            ),
            User::new_user(
                "user3".to_string(),
                Email::new("user3@example.com".to_string()).unwrap(),
                "hash".to_string(),
                None,
            ),
        ];

        // Test with limit smaller than list size
        handler.print_user_list(&users, 2);
    }
}
