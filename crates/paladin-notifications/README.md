# paladin-notifications

Notification delivery adapters for the Paladin framework.

## Purpose

`paladin-notifications` provides adapter implementations for email, push, and system notification delivery.

## Key Modules

- `email_notification_adapter`: Email notification integration.
- `push_notification_adapter`: Push-style message delivery integration.
- `system_notification_adapter`: System channel notification integration.

## Usage

```rust
use paladin_notifications::email_notification_adapter;
use paladin_notifications::push_notification_adapter;

// Select an adapter module and instantiate via the corresponding constructor.
let _email_module = std::any::type_name::<email_notification_adapter::EmailNotificationAdapter>();
let _push_module = std::any::type_name::<push_notification_adapter::PushNotificationAdapter>();
```

## Feature Flags

- `email`: Enable email adapter dependencies.
- `push`: Enable push adapter dependencies.
- `system`: Enable system notification adapter support.
