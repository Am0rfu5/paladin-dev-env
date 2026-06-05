//! Notification adapters — re-exported from `paladin-notifications` under the
//! `notifications` feature.
//!
//! When the feature is inactive the facade exposes no notification adapters; every consumer
//! of these types (e.g. `config/notifications.rs`, the notification integration test) is itself
//! gated behind the same feature.
#[cfg(feature = "notifications")]
pub use paladin_notifications::{
    email_notification_adapter, push_notification_adapter, system_notification_adapter,
};

// Convenience re-exports of the most-used adapter types.
#[cfg(feature = "notifications")]
pub use paladin_notifications::email_notification_adapter::{
    EmailAdapterConfig, EmailNotificationAdapter,
};
#[cfg(feature = "notifications")]
pub use paladin_notifications::system_notification_adapter::{
    SystemAdapterConfig, SystemNotificationAdapter,
};
