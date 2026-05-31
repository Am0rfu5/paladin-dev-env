use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::base::service::message_service::MessageError;
use crate::core::platform::container::notification::{
    Notification, NotificationChannel, NotificationContent, NotificationDomainError,
    NotificationStatus, NotificationTemplate,
};

/// Result type for notification orchestrator operations
pub type NotificationOrchestratorResult<T> = Result<T, NotificationOrchestratorError>;

/// Errors that can occur in notification orchestrator operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum NotificationOrchestratorError {
    #[error("Domain error: {0}")]
    DomainError(#[from] NotificationDomainError),

    #[error("Message service error: {0}")]
    MessageError(#[from] MessageError),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Channel not available: {0}")]
    ChannelNotAvailable(String),

    #[error("Delivery failed: {0}")]
    DeliveryFailed(String),

    #[error("Service not initialized")]
    ServiceNotInitialized,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Trait for notification channel handlers
#[async_trait]
pub trait NotificationChannelHandler: Send + Sync {
    /// Get the channel this handler supports
    fn channel(&self) -> NotificationChannel;

    /// Check if this handler can process the notification
    fn can_handle(&self, notification: &Notification) -> bool;

    /// Process a notification for delivery
    async fn handle_notification(
        &self,
        notification: Notification,
    ) -> NotificationOrchestratorResult<NotificationDeliveryResult>;

    /// Check handler health
    async fn health_check(&self) -> bool;
}

/// Result of notification delivery attempt
#[derive(Debug, Clone)]
pub struct NotificationDeliveryResult {
    /// Notification ID
    pub notification_id: Uuid,
    /// Delivery status
    pub status: NotificationStatus,
    /// External service ID if available
    pub external_id: Option<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Error message if delivery failed
    pub error_message: Option<String>,
    /// Delivery timestamp
    pub timestamp: DateTime<Utc>,
}

/// Trait for template processing
#[async_trait]
pub trait NotificationTemplateProcessor: Send + Sync {
    /// Render template with variables
    async fn render_template(
        &self,
        template: &NotificationTemplate,
        variables: &HashMap<String, serde_json::Value>,
    ) -> NotificationOrchestratorResult<NotificationContent>;

    /// Validate template syntax
    async fn validate_template(
        &self,
        template: &NotificationTemplate,
    ) -> NotificationOrchestratorResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants_display() {
        let template_err =
            NotificationOrchestratorError::TemplateNotFound("test_template".to_string());
        assert_eq!(
            template_err.to_string(),
            "Template not found: test_template"
        );

        let channel_err = NotificationOrchestratorError::ChannelNotAvailable("sms".to_string());
        assert_eq!(channel_err.to_string(), "Channel not available: sms");

        let delivery_err = NotificationOrchestratorError::DeliveryFailed("timeout".to_string());
        assert_eq!(delivery_err.to_string(), "Delivery failed: timeout");

        let init_err = NotificationOrchestratorError::ServiceNotInitialized;
        assert_eq!(init_err.to_string(), "Service not initialized");

        let config_err = NotificationOrchestratorError::ConfigurationError("invalid".to_string());
        assert_eq!(config_err.to_string(), "Configuration error: invalid");

        let validation_err =
            NotificationOrchestratorError::ValidationError("missing field".to_string());
        assert_eq!(
            validation_err.to_string(),
            "Validation error: missing field"
        );

        let storage_err = NotificationOrchestratorError::StorageError("db down".to_string());
        assert_eq!(storage_err.to_string(), "Storage error: db down");

        let unknown_err = NotificationOrchestratorError::Unknown("mystery".to_string());
        assert_eq!(unknown_err.to_string(), "Unknown error: mystery");
    }
}
