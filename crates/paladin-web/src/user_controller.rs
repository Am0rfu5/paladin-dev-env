/*
User Controller

REST API endpoints for user operations. This handles HTTP requests and responses,
delegating business logic to the UserService.
*/

use paladin_core::platform::container::user::{UserError, UserProfile};
use paladin_core::platform::manager::user_service::{
    UserLoginRequest, UserProfileUpdateRequest, UserRegistrationRequest, UserServiceTrait,
};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// User registration request DTO
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

/// User login request DTO
#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

/// User profile update request DTO
#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

/// User response DTO
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub profile: UserProfileResponse,
    pub created_at: String,
    pub modified_at: String,
}

/// User profile response DTO
#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

/// Login response DTO
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub success: bool,
}

/// Error response DTO
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

/// API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String, code: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorResponse { error, code }),
        }
    }
}

/// Convert UserError to HTTP status code and error response
fn user_error_to_response(error: UserError) -> (StatusCode, Json<ApiResponse<()>>) {
    let (status, code, message) = match error {
        UserError::InvalidEmail(_) => (StatusCode::BAD_REQUEST, "INVALID_EMAIL", error.to_string()),
        UserError::InvalidUsername(_) => (
            StatusCode::BAD_REQUEST,
            "INVALID_USERNAME",
            error.to_string(),
        ),
        UserError::InvalidPassword(_) => (
            StatusCode::BAD_REQUEST,
            "INVALID_PASSWORD",
            error.to_string(),
        ),
        UserError::EmailAlreadyExists(_) => {
            (StatusCode::CONFLICT, "EMAIL_EXISTS", error.to_string())
        }
        UserError::UsernameAlreadyExists(_) => {
            (StatusCode::CONFLICT, "USERNAME_EXISTS", error.to_string())
        }
        UserError::UserNotFound(_) => (StatusCode::NOT_FOUND, "USER_NOT_FOUND", error.to_string()),
        UserError::UserNotFoundByEmail(_) => {
            (StatusCode::NOT_FOUND, "USER_NOT_FOUND", error.to_string())
        }
        UserError::AuthenticationFailed => (
            StatusCode::UNAUTHORIZED,
            "AUTH_FAILED",
            "Invalid credentials".to_string(),
        ),
        UserError::UserNotActive => (
            StatusCode::FORBIDDEN,
            "USER_INACTIVE",
            "User account is not active".to_string(),
        ),
        UserError::UserNotVerified => (
            StatusCode::FORBIDDEN,
            "USER_NOT_VERIFIED",
            "User email is not verified".to_string(),
        ),
        UserError::RepositoryError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Internal server error".to_string(),
        ),
        UserError::HashError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Internal server error".to_string(),
        ),
    };

    (status, Json(ApiResponse::error(message, code.to_string())))
}

/// Convert User to UserResponse
fn user_to_response(user: &paladin_core::platform::container::user::User) -> UserResponse {
    UserResponse {
        id: user.uuid,
        username: user.username().to_string(),
        email: user.email().value().to_string(),
        is_active: user.is_active(),
        is_verified: user.is_verified(),
        profile: UserProfileResponse {
            first_name: user.profile().first_name.clone(),
            last_name: user.profile().last_name.clone(),
            bio: user.profile().bio.clone(),
            avatar_url: user.profile().avatar_url.clone(),
            timezone: user.profile().timezone.clone(),
            locale: user.profile().locale.clone(),
        },
        created_at: user.created.to_rfc3339(),
        modified_at: user.modified.to_rfc3339(),
    }
}

/// Register a new user
async fn register_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let profile = if request.first_name.is_some()
        || request.last_name.is_some()
        || request.bio.is_some()
        || request.timezone.is_some()
        || request.locale.is_some()
    {
        Some(UserProfile {
            first_name: request.first_name,
            last_name: request.last_name,
            bio: request.bio,
            avatar_url: None,
            timezone: request.timezone.or_else(|| Some("UTC".to_string())),
            locale: request.locale.or_else(|| Some("en-US".to_string())),
        })
    } else {
        None
    };

    let registration_request = UserRegistrationRequest {
        username: request.username,
        email: request.email,
        password: request.password,
        profile,
    };

    match user_service.register_user(registration_request).await {
        Ok(user) => Ok(Json(ApiResponse::success(user_to_response(&user)))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// User login
async fn login_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Json(request): Json<LoginUserRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let login_request = UserLoginRequest {
        email: request.email,
        password: request.password,
    };

    match user_service.login_user(login_request).await {
        Ok(result) => Ok(Json(ApiResponse::success(LoginResponse {
            user_id: result.user_id,
            username: result.username,
            email: result.email,
            is_verified: result.is_verified,
            success: result.success,
        }))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Get user by ID
async fn get_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(Json(ApiResponse::success(user_to_response(&user)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(
                "User not found".to_string(),
                "USER_NOT_FOUND".to_string(),
            )),
        )),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Update user profile
async fn update_user_profile(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserProfileRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let profile = if request.first_name.is_some()
        || request.last_name.is_some()
        || request.bio.is_some()
        || request.avatar_url.is_some()
        || request.timezone.is_some()
        || request.locale.is_some()
    {
        // First, get current user to preserve existing profile values
        let current_user = match user_service.get_user_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::error(
                        "User not found".to_string(),
                        "USER_NOT_FOUND".to_string(),
                    )),
                ));
            }
            Err(error) => return Err(user_error_to_response(error)),
        };

        Some(UserProfile {
            first_name: request
                .first_name
                .or(current_user.profile().first_name.clone()),
            last_name: request
                .last_name
                .or(current_user.profile().last_name.clone()),
            bio: request.bio.or(current_user.profile().bio.clone()),
            avatar_url: request
                .avatar_url
                .or(current_user.profile().avatar_url.clone()),
            timezone: request.timezone.or(current_user.profile().timezone.clone()),
            locale: request.locale.or(current_user.profile().locale.clone()),
        })
    } else {
        None
    };

    let update_request = UserProfileUpdateRequest {
        user_id,
        username: request.username,
        email: request.email,
        profile,
    };

    match user_service.update_user_profile(update_request).await {
        Ok(user) => Ok(Json(ApiResponse::success(user_to_response(&user)))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Activate user account
async fn activate_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    match user_service.activate_user(user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Deactivate user account
async fn deactivate_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    match user_service.deactivate_user(user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Verify user email
async fn verify_user(
    State(user_service): State<Arc<dyn UserServiceTrait>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    match user_service.verify_user(user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(error) => Err(user_error_to_response(error)),
    }
}

/// Create the user routes
pub fn create_user_routes(user_service: Arc<dyn UserServiceTrait>) -> Router {
    Router::new()
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user_profile))
        .route("/users/:id/activate", post(activate_user))
        .route("/users/:id/deactivate", post(deactivate_user))
        .route("/users/:id/verify", post(verify_user))
        .with_state(user_service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono;
    use paladin_core::platform::container::user::{User, UserError, UserProfile};
    use paladin_core::platform::manager::user_service::UserAuthenticationResult;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock types for testing
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct MockUser {
        pub uuid: Uuid,
        pub username: String,
        pub email: String,
        pub is_active: bool,
        pub is_verified: bool,
        pub profile: UserProfile,
        pub created: chrono::DateTime<chrono::Utc>,
        pub modified: chrono::DateTime<chrono::Utc>,
    }

    impl MockUser {
        fn new() -> Self {
            Self {
                uuid: Uuid::new_v4(),
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                is_active: true,
                is_verified: true,
                profile: UserProfile {
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    bio: Some("Test bio".to_string()),
                    avatar_url: None,
                    timezone: Some("UTC".to_string()),
                    locale: Some("en-US".to_string()),
                },
                created: chrono::Utc::now(),
                modified: chrono::Utc::now(),
            }
        }
    }

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

        fn set_should_fail(&self, fail: bool) {
            *self.should_fail.lock().unwrap() = fail;
        }

        fn add_user(&self, user: User) {
            self.users.lock().unwrap().insert(user.uuid, user);
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
                paladin_core::platform::container::user::Email::new(request.email).unwrap(),
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
                // Default successful login
                Ok(UserAuthenticationResult {
                    user_id: Uuid::new_v4(),
                    username: "testuser".to_string(),
                    email: request.email,
                    is_verified: true,
                    success: true,
                })
            }
        }

        async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }

            if let Some(user) = self.users.lock().unwrap().get(&user_id) {
                Ok(Some(user.clone()))
            } else {
                Ok(None)
            }
        }

        async fn update_user_profile(
            &self,
            request: UserProfileUpdateRequest,
        ) -> Result<User, UserError> {
            if *self.should_fail.lock().unwrap() {
                return Err(UserError::RepositoryError("Mock error".to_string()));
            }

            // Get existing user or create mock
            let existing_user =
                if let Some(existing) = self.users.lock().unwrap().get(&request.user_id) {
                    existing.clone()
                } else {
                    User::new_user(
                        "testuser".to_string(),
                        paladin_core::platform::container::user::Email::new(
                            "test@example.com".to_string(),
                        )
                        .unwrap(),
                        "mock_hash".to_string(),
                        None,
                    )
                };

            // Apply updates
            let updated_profile = request
                .profile
                .unwrap_or_else(|| existing_user.profile().clone());
            let updated_username = request
                .username
                .unwrap_or_else(|| existing_user.username().to_string());
            let updated_email = request
                .email
                .unwrap_or_else(|| existing_user.email().value().to_string());

            let updated_user = User::new_user(
                updated_username,
                paladin_core::platform::container::user::Email::new(updated_email).unwrap(),
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

    #[test]
    fn test_user_error_to_response_invalid_email() {
        let error = UserError::InvalidEmail("Invalid email".to_string());
        let (status, response) = user_error_to_response(error);

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(!response.0.success);
        assert_eq!(response.0.error.as_ref().unwrap().code, "INVALID_EMAIL");
    }

    #[test]
    fn test_user_error_to_response_user_not_found() {
        let error = UserError::UserNotFound(Uuid::new_v4());
        let (status, response) = user_error_to_response(error);

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(!response.0.success);
        assert_eq!(response.0.error.as_ref().unwrap().code, "USER_NOT_FOUND");
    }

    #[test]
    fn test_user_error_to_response_authentication_failed() {
        let error = UserError::AuthenticationFailed;
        let (status, response) = user_error_to_response(error);

        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(!response.0.success);
        assert_eq!(response.0.error.as_ref().unwrap().code, "AUTH_FAILED");
    }

    #[test]
    fn test_user_to_response() {
        let mock_user = MockUser::new();
        let user = User::new_user(
            mock_user.username.clone(),
            paladin_core::platform::container::user::Email::new(mock_user.email.clone()).unwrap(),
            "mock_hash".to_string(),
            Some(mock_user.profile.clone()),
        );

        let response = user_to_response(&user);

        assert_eq!(response.username, user.username());
        assert_eq!(response.email, user.email().value());
        assert_eq!(response.is_active, user.is_active());
        assert_eq!(response.is_verified, user.is_verified());
        assert_eq!(response.profile.first_name, user.profile().first_name);
        assert_eq!(response.profile.last_name, user.profile().last_name);
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let request = RegisterUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            bio: Some("Test bio".to_string()),
            timezone: Some("UTC".to_string()),
            locale: Some("en-US".to_string()),
        };

        let result = register_user(State(service_ref), Json(request)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
        assert!(response.0.data.is_some());
    }

    #[tokio::test]
    async fn test_register_user_service_error() {
        let user_service = Arc::new(MockUserService::new());
        user_service.set_should_fail(true);
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();

        let request = RegisterUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: None,
            last_name: None,
            bio: None,
            timezone: None,
            locale: None,
        };

        let result = register_user(State(service_ref), Json(request)).await;

        assert!(result.is_err());
        let (status, response) = result.err().unwrap();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!response.0.success);
    }

    #[tokio::test]
    async fn test_login_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let request = LoginUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login_user(State(service_ref), Json(request)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
        assert!(response.0.data.is_some());
        assert_eq!(response.0.data.as_ref().unwrap().email, "test@example.com");
        assert!(response.0.data.as_ref().unwrap().success);
    }

    #[tokio::test]
    async fn test_login_user_auth_failed() {
        let user_service = Arc::new(MockUserService::new());
        user_service.set_should_fail(true);
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();

        let request = LoginUserRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = login_user(State(service_ref), Json(request)).await;

        assert!(result.is_err());
        let (status, response) = result.err().unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(!response.0.success);
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let mock_user = User::new_user(
            "testuser".to_string(),
            paladin_core::platform::container::user::Email::new("test@example.com".to_string())
                .unwrap(),
            "mock_hash".to_string(),
            None,
        );
        user_service.add_user(mock_user.clone());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();

        let result = get_user(State(service_ref), Path(mock_user.uuid)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
        assert!(response.0.data.is_some());
        assert_eq!(response.0.data.as_ref().unwrap().id, mock_user.uuid);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let user_id = Uuid::new_v4();

        let result = get_user(State(service_ref), Path(user_id)).await;

        assert!(result.is_err());
        let (status, response) = result.err().unwrap();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(!response.0.success);
    }

    #[tokio::test]
    async fn test_activate_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let user_id = Uuid::new_v4();

        let result = activate_user(State(service_ref), Path(user_id)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_deactivate_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let user_id = Uuid::new_v4();

        let result = deactivate_user(State(service_ref), Path(user_id)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_verify_user_success() {
        let user_service = Arc::new(MockUserService::new());
        let service_ref: Arc<dyn UserServiceTrait> = user_service.clone();
        let user_id = Uuid::new_v4();

        let result = verify_user(State(service_ref), Path(user_id)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.success);
    }

    #[test]
    fn test_api_response_success() {
        let data = "test data".to_string();
        let response: ApiResponse<String> = ApiResponse::success(data.clone());

        assert!(response.success);
        assert_eq!(response.data, Some(data));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let error_msg = "Test error".to_string();
        let error_code = "TEST_ERROR".to_string();
        let response: ApiResponse<()> = ApiResponse::error(error_msg.clone(), error_code.clone());

        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error.as_ref().unwrap().error, error_msg);
        assert_eq!(response.error.as_ref().unwrap().code, error_code);
    }
}
