//! A [`NotificationChannelHandler`] double whose `handle_notification` always errors.
//!
//! `UserService.notification_service` holds a *concrete* `Arc<NotificationService>`, so no
//! trait double can be substituted for it without changing a public constructor. But
//! `NotificationService::register_channel_handler` is public, so registering a
//! [`FailingChannelHandler`] on a real `NotificationService` before injecting it forces the
//! same delivery-failure path with no production signature change. The behaviour that path
//! guards — `register_user` handling the send result with `if let Err(..)` rather than `?` — is
//! already implemented correctly; this double exists to let a test prove that, not to drive new
//! production code.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::application::services::notification_orchestrator::{
    NotificationChannelHandler, NotificationDeliveryResult, NotificationOrchestratorError,
    NotificationOrchestratorResult,
};
use crate::core::platform::container::notification::{Notification, NotificationChannel};

/// A single recorded dispatch to a [`FailingChannelHandler`].
///
/// Recorded by domain identity — the notification's id and channel — rather than by a rendered
/// display string, so a multi-byte Unicode payload in a notification cannot change an
/// assertion's verdict.
///
/// # Examples
///
/// ```ignore
/// use paladin::test_support::FailingChannelInvocation;
///
/// let invocation = FailingChannelInvocation {
///     notification_id: uuid::Uuid::new_v4(),
///     channel: paladin::core::platform::container::notification::NotificationChannel::Email,
/// };
/// assert_eq!(invocation.channel, paladin::core::platform::container::notification::NotificationChannel::Email);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailingChannelInvocation {
    /// The id of the notification that was dispatched to the handler.
    pub notification_id: Uuid,
    /// The channel the dispatched notification was addressed to.
    pub channel: NotificationChannel,
}

/// A [`NotificationChannelHandler`] whose `handle_notification` always returns an error.
///
/// Follows the workspace's established `Arc<Mutex<..>>` recording shape (see the `tests`
/// crate's `MockLlmAdapter` helper): clones share the same underlying recording state, so
/// the handler is cheaply cloneable into an `Arc<dyn NotificationChannelHandler>` without a
/// wrapper, and remains `Send + Sync` for use from a multi-threaded `#[tokio::test]`.
///
/// A poisoned internal lock is recovered rather than panicked on: every access uses
/// `.lock().unwrap_or_else(|poisoned| poisoned.into_inner())`, so one failing test cannot take
/// down an otherwise-passing run.
///
/// # Examples
///
/// ```ignore
/// use paladin::test_support::FailingChannelHandler;
/// use std::sync::Arc;
///
/// let handler = Arc::new(FailingChannelHandler::new());
/// assert_eq!(handler.call_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct FailingChannelHandler {
    channel: NotificationChannel,
    error: NotificationOrchestratorError,
    invocations: Arc<Mutex<Vec<FailingChannelInvocation>>>,
}

impl FailingChannelHandler {
    /// Creates a handler for [`NotificationChannel::Email`] — the channel
    /// `UserService::send_welcome_notification` dispatches through — that fails every
    /// notification with a sensible default delivery error.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use paladin::test_support::FailingChannelHandler;
    ///
    /// let handler = FailingChannelHandler::new();
    /// assert_eq!(handler.call_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            channel: NotificationChannel::Email,
            error: NotificationOrchestratorError::DeliveryFailed(
                "FailingChannelHandler: delivery deliberately fails for this test".to_string(),
            ),
            invocations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a handler for [`NotificationChannel::Email`] that fails every notification with
    /// the given error variant, letting a test choose exactly which failure path it exercises.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use paladin::test_support::FailingChannelHandler;
    /// use paladin::application::services::notification_orchestrator::NotificationOrchestratorError;
    ///
    /// let handler = FailingChannelHandler::with_error(
    ///     NotificationOrchestratorError::ChannelNotAvailable("email".to_string()),
    /// );
    /// ```
    pub fn with_error(error: NotificationOrchestratorError) -> Self {
        Self {
            error,
            ..Self::new()
        }
    }

    /// Returns a snapshot of every notification dispatched to this handler, in call order, so a
    /// test can distinguish the first call from the second.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use paladin::test_support::FailingChannelHandler;
    ///
    /// let handler = FailingChannelHandler::new();
    /// assert!(handler.invocations().is_empty());
    /// ```
    pub fn invocations(&self) -> Vec<FailingChannelInvocation> {
        self.invocations
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Returns the number of times [`NotificationChannelHandler::handle_notification`] has been
    /// called on this handler (or a clone sharing its recording state).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use paladin::test_support::FailingChannelHandler;
    ///
    /// let handler = FailingChannelHandler::new();
    /// assert_eq!(handler.call_count(), 0);
    /// ```
    pub fn call_count(&self) -> usize {
        self.invocations
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}

impl Default for FailingChannelHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NotificationChannelHandler for FailingChannelHandler {
    fn channel(&self) -> NotificationChannel {
        self.channel.clone()
    }

    fn can_handle(&self, notification: &Notification) -> bool {
        notification.channel == self.channel
    }

    async fn handle_notification(
        &self,
        notification: Notification,
    ) -> NotificationOrchestratorResult<NotificationDeliveryResult> {
        let invocation = FailingChannelInvocation {
            notification_id: notification.id,
            channel: notification.channel.clone(),
        };
        self.invocations
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(invocation);
        Err(self.error.clone())
    }

    async fn health_check(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::notification::{
        NotificationContent, NotificationPriority, NotificationRecipient,
    };

    /// Builds a well-formed `Notification` for the tests below, or `None` if construction
    /// failed, so a test failure here surfaces as a skipped assertion rather than a panic.
    fn sample_notification() -> Option<Notification> {
        Notification::new(
            NotificationRecipient::Email("user@example.com".to_string()),
            NotificationContent::new(
                "Welcome".to_string(),
                "Welcome to Paladin".to_string(),
                "onboarding".to_string(),
            ),
            NotificationChannel::Email,
            NotificationPriority::Normal,
        )
        .ok()
    }

    #[test]
    fn new_defaults_to_the_email_channel_with_no_invocations() {
        let handler = FailingChannelHandler::new();
        assert_eq!(handler.channel(), NotificationChannel::Email);
        assert_eq!(handler.call_count(), 0);
        assert!(handler.invocations().is_empty());
    }

    #[test]
    fn can_handle_matches_only_the_claimed_channel() {
        let handler = FailingChannelHandler::new();
        if let Some(notification) = sample_notification() {
            assert!(handler.can_handle(&notification));
        }
    }

    #[tokio::test]
    async fn handle_notification_always_errors_and_records_by_domain_identity() {
        let handler = FailingChannelHandler::new();
        let Some(notification) = sample_notification() else {
            return;
        };
        let expected_id = notification.id;
        let expected_channel = notification.channel.clone();

        let result = handler.handle_notification(notification).await;
        assert!(result.is_err());
        assert_eq!(handler.call_count(), 1);

        let invocations = handler.invocations();
        assert_eq!(invocations.len(), 1);
        assert_eq!(invocations[0].notification_id, expected_id);
        assert_eq!(invocations[0].channel, expected_channel);
    }

    #[tokio::test]
    async fn handle_notification_preserves_call_order_across_two_calls() {
        let handler = FailingChannelHandler::new();
        let Some(first) = sample_notification() else {
            return;
        };
        let Some(second) = sample_notification() else {
            return;
        };
        let first_id = first.id;
        let second_id = second.id;

        let _ = handler.handle_notification(first).await;
        let _ = handler.handle_notification(second).await;

        let invocations = handler.invocations();
        assert_eq!(invocations.len(), 2);
        assert_eq!(invocations[0].notification_id, first_id);
        assert_eq!(invocations[1].notification_id, second_id);
    }

    #[tokio::test]
    async fn with_error_lets_a_test_choose_the_error_variant() {
        let handler = FailingChannelHandler::with_error(
            NotificationOrchestratorError::ChannelNotAvailable("email".to_string()),
        );
        let Some(notification) = sample_notification() else {
            return;
        };

        let result = handler.handle_notification(notification).await;
        assert!(matches!(
            result,
            Err(NotificationOrchestratorError::ChannelNotAvailable(_))
        ));
    }

    #[tokio::test]
    async fn health_check_reports_healthy() {
        let handler = FailingChannelHandler::new();
        assert!(handler.health_check().await);
    }

    #[test]
    fn clones_share_recorded_invocations() {
        let handler = FailingChannelHandler::new();
        let clone = handler.clone();
        assert_eq!(clone.call_count(), handler.call_count());
    }
}
