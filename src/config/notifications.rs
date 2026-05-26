//! Configuration for the notification system.

#[cfg(feature = "notifications")]
use crate::config::env_utils::EnvOverridable;
#[cfg(feature = "notifications")]
use crate::infrastructure::adapters::notifications::{EmailAdapterConfig, SystemAdapterConfig};
#[cfg(feature = "notifications")]
use serde::{Deserialize, Serialize};

/// Configuration for notification system
#[cfg(feature = "notifications")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enable/disable notification system
    pub enabled: bool,
    /// Email notification configuration
    pub email: Option<EmailAdapterConfig>,
    /// System notification configuration
    pub system: Option<SystemAdapterConfig>,
    /// Global notification settings
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
    pub enable_delivery_tracking: bool,
    /// Rate limiting settings
    pub global_rate_limit_per_minute: Option<u32>,
}

#[cfg(feature = "notifications")]
impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email: Some(EmailAdapterConfig::default()),
            system: Some(SystemAdapterConfig::default()),
            max_retries: 3,
            retry_delay_seconds: 60,
            enable_delivery_tracking: true,
            global_rate_limit_per_minute: Some(100),
        }
    }
}

#[cfg(feature = "notifications")]
impl EnvOverridable for NotificationConfig {
    fn apply_env_overrides(&mut self) {
        if let Some(b) = std::env::var("APP_NOTIFICATIONS_ENABLED")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
        {
            self.enabled = b;
        }
        if let Some(ref mut email_config) = self.email {
            if let Ok(v) = std::env::var("APP_EMAIL_SMTP_HOST") {
                email_config.smtp_host = v;
            }
            if let Some(p) = std::env::var("APP_EMAIL_SMTP_PORT")
                .ok()
                .and_then(|v| v.parse::<u16>().ok())
            {
                email_config.smtp_port = p;
            }
            if let Ok(v) = std::env::var("APP_EMAIL_USERNAME") {
                email_config.username = v;
            }
            if let Ok(v) = std::env::var("APP_EMAIL_PASSWORD") {
                email_config.password = v;
            }
            if let Ok(v) = std::env::var("APP_EMAIL_FROM_ADDRESS") {
                email_config.from_address = v;
            }
            if let Ok(v) = std::env::var("APP_EMAIL_FROM_NAME") {
                email_config.from_name = Some(v);
            }
            if let Some(b) = std::env::var("APP_EMAIL_USE_TLS")
                .ok()
                .and_then(|v| v.parse::<bool>().ok())
            {
                email_config.use_tls = b;
            }
        }
    }
}
