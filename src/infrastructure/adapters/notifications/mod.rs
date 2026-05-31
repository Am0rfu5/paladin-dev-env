/// When `notifications` feature is active, all adapters come from `paladin-notifications`.
#[cfg(feature = "notifications")]
pub use paladin_notifications::email_notification_adapter;
#[cfg(feature = "notifications")]
pub use paladin_notifications::push_notification_adapter;
#[cfg(feature = "notifications")]
pub use paladin_notifications::system_notification_adapter;

/// Fallback: use facade-local copies when `notifications` feature is inactive.
#[cfg(not(feature = "notifications"))]
pub mod email_notification_adapter;
#[cfg(not(feature = "notifications"))]
pub mod system_notification_adapter;

// Re-export main adapters for convenience
#[cfg(not(feature = "notifications"))]
pub use email_notification_adapter::{EmailAdapterConfig, EmailNotificationAdapter};
#[cfg(feature = "notifications")]
pub use paladin_notifications::email_notification_adapter::{
    EmailAdapterConfig, EmailNotificationAdapter,
};
#[cfg(feature = "notifications")]
pub use paladin_notifications::system_notification_adapter::{
    SystemAdapterConfig, SystemNotificationAdapter,
};
#[cfg(not(feature = "notifications"))]
pub use system_notification_adapter::{SystemAdapterConfig, SystemNotificationAdapter};
