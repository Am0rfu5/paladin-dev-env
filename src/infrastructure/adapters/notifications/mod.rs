pub mod email_notification_adapter;
pub mod push_notification_adapter;
pub mod system_notification_adapter;

// Re-export main adapters for convenience
pub use email_notification_adapter::{EmailAdapterConfig, EmailNotificationAdapter};
pub use system_notification_adapter::{SystemAdapterConfig, SystemNotificationAdapter};
