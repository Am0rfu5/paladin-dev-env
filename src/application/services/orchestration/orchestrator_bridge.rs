//! Concrete agent → orchestrator bridge adapter.
//!
//! This module implements [`OrchestratorPort`] over the root-crate
//! [`Orchestrator`]. It lives in the **root crate** (not `paladin-ports`)
//! because it depends on the concrete `Orchestrator`: a lower crate cannot
//! depend on the root crate without introducing a circular dependency. This is
//! the same placement constraint documented for the Epic 3 content processors.
//!
//! The adapter enforces a [`BridgePolicy`] *before* performing any underlying
//! orchestrator call: a disallowed action yields
//! [`OrchestratorBridgeError::ActionNotAllowed`] and a cap-exceeding action
//! yields [`OrchestratorBridgeError::QuotaExceeded`]. Caps are tracked
//! per-adapter-instance; construct one adapter per agent execution so caps are
//! scoped to that execution.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use async_trait::async_trait;
use uuid::Uuid;

use paladin_ports::output::notification_port::NotificationDeliveryPort;
use paladin_ports::output::orchestrator_port::{
    BridgeAction, BridgePolicy, EventDispatchResult, FireEventRequest, OrchestratorBridgeError,
    OrchestratorPort, QueueItemRequest, ScheduleJobRequest, SendNotificationRequest,
};

use crate::core::base::component::event::Event;
use crate::core::platform::container::job::Job;
use crate::core::platform::container::notification::{
    Notification, NotificationChannel, NotificationContent, NotificationPriority,
    NotificationRecipient,
};
use crate::core::platform::container::orchestration_context::OrchestrationContext;

use super::Orchestrator;

/// Per-execution counters for the four bridge actions.
#[derive(Debug, Default)]
struct ActionCounters {
    jobs_scheduled: AtomicU32,
    items_queued: AtomicU32,
    events_fired: AtomicU32,
    notifications_sent: AtomicU32,
}

impl ActionCounters {
    fn counter(&self, action: BridgeAction) -> &AtomicU32 {
        match action {
            BridgeAction::ScheduleJob => &self.jobs_scheduled,
            BridgeAction::QueueItem => &self.items_queued,
            BridgeAction::FireEvent => &self.events_fired,
            BridgeAction::SendNotification => &self.notifications_sent,
        }
    }
}

/// Concrete [`OrchestratorPort`] backed by an [`Orchestrator`].
pub struct OrchestratorBridgeAdapter {
    orchestrator: Arc<Orchestrator>,
    policy: BridgePolicy,
    notification: Option<Arc<dyn NotificationDeliveryPort>>,
    counters: ActionCounters,
}

impl OrchestratorBridgeAdapter {
    /// Create a new bridge adapter with the given policy and no notification
    /// delivery port. `send_notification` will fail until a delivery port is
    /// attached via [`OrchestratorBridgeAdapter::with_notification_port`].
    pub fn new(orchestrator: Arc<Orchestrator>, policy: BridgePolicy) -> Self {
        Self {
            orchestrator,
            policy,
            notification: None,
            counters: ActionCounters::default(),
        }
    }

    /// Attach a notification delivery port enabling `send_notification`.
    pub fn with_notification_port(
        mut self,
        notification: Arc<dyn NotificationDeliveryPort>,
    ) -> Self {
        self.notification = Some(notification);
        self
    }

    /// Enforce the policy for an action: reject disallowed actions and count the
    /// invocation against the per-execution cap.
    fn enforce(&self, action: BridgeAction) -> Result<(), OrchestratorBridgeError> {
        if !self.policy.is_allowed(action) {
            return Err(OrchestratorBridgeError::ActionNotAllowed(
                action.as_str().to_string(),
            ));
        }
        let cap = self.policy.cap_for(action);
        // fetch_add returns the previous value; the new count is previous + 1.
        let new_count = self.counters.counter(action).fetch_add(1, Ordering::SeqCst) + 1;
        if new_count > cap {
            return Err(OrchestratorBridgeError::QuotaExceeded {
                action: action.as_str().to_string(),
                limit: cap,
            });
        }
        Ok(())
    }

    fn context() -> OrchestrationContext {
        OrchestrationContext::new("agent_orchestrator_bridge".to_string(), "agent".to_string())
    }
}

#[async_trait]
impl OrchestratorPort for OrchestratorBridgeAdapter {
    async fn schedule_job(
        &self,
        request: ScheduleJobRequest,
    ) -> Result<Uuid, OrchestratorBridgeError> {
        self.enforce(BridgeAction::ScheduleJob)?;
        if request.name.trim().is_empty() {
            return Err(OrchestratorBridgeError::InvalidRequest(
                "job name cannot be empty".to_string(),
            ));
        }
        let job = Job::new(request.name, request.description, Vec::new());
        self.orchestrator
            .schedule_job(job, request.schedule, Self::context())
            .await
            .map_err(|e| OrchestratorBridgeError::OrchestratorError(e.to_string()))
    }

    async fn queue_item(&self, request: QueueItemRequest) -> Result<Uuid, OrchestratorBridgeError> {
        self.enforce(BridgeAction::QueueItem)?;
        if request.queue_name.trim().is_empty() {
            return Err(OrchestratorBridgeError::InvalidRequest(
                "queue name cannot be empty".to_string(),
            ));
        }
        let description = request.payload.to_string();
        let job = Job::new(
            format!("queued-item-{}", request.queue_name),
            description,
            Vec::new(),
        );
        self.orchestrator
            .ensure_queue(&request.queue_name)
            .await
            .map_err(|e| OrchestratorBridgeError::OrchestratorError(e.to_string()))?;
        self.orchestrator
            .queue_job(job, &request.queue_name, Self::context())
            .await
            .map_err(|e| OrchestratorBridgeError::OrchestratorError(e.to_string()))
    }

    async fn fire_event(
        &self,
        request: FireEventRequest,
    ) -> Result<EventDispatchResult, OrchestratorBridgeError> {
        self.enforce(BridgeAction::FireEvent)?;
        if request.event_type.trim().is_empty() {
            return Err(OrchestratorBridgeError::InvalidRequest(
                "event type cannot be empty".to_string(),
            ));
        }
        let event = Event::new(request.event_type, request.payload, request.source);
        let trigger_ids = self
            .orchestrator
            .process_event(event)
            .await
            .map_err(|e| OrchestratorBridgeError::OrchestratorError(e.to_string()))?;
        Ok(EventDispatchResult {
            triggered_count: trigger_ids.len(),
            trigger_ids,
        })
    }

    async fn send_notification(
        &self,
        request: SendNotificationRequest,
    ) -> Result<Uuid, OrchestratorBridgeError> {
        self.enforce(BridgeAction::SendNotification)?;
        let delivery = self.notification.as_ref().ok_or_else(|| {
            OrchestratorBridgeError::InvalidRequest(
                "no notification delivery port is configured".to_string(),
            )
        })?;

        let (channel, recipient) = map_channel_and_recipient(&request.channel, &request.recipient);
        let content = NotificationContent::new(
            request.subject,
            request.body,
            "agent_notification".to_string(),
        );
        let notification =
            Notification::new(recipient, content, channel, NotificationPriority::Normal)
                .map_err(|e| OrchestratorBridgeError::InvalidRequest(e.to_string()))?;
        let id = notification.id;

        delivery
            .deliver_notification(notification)
            .await
            .map_err(|e| OrchestratorBridgeError::OrchestratorError(e.to_string()))?;
        Ok(id)
    }
}

/// Map an agent-supplied channel string and recipient identifier onto a
/// compatible [`NotificationChannel`]/[`NotificationRecipient`] pair.
fn map_channel_and_recipient(
    channel: &str,
    recipient: &str,
) -> (NotificationChannel, NotificationRecipient) {
    match channel.to_ascii_lowercase().as_str() {
        "email" => (
            NotificationChannel::Email,
            NotificationRecipient::Email(recipient.to_string()),
        ),
        "sms" => (
            NotificationChannel::Sms,
            NotificationRecipient::Phone(recipient.to_string()),
        ),
        "webhook" => (
            NotificationChannel::Webhook,
            NotificationRecipient::WebhookUrl(recipient.to_string()),
        ),
        _ => (
            NotificationChannel::System,
            NotificationRecipient::SystemComponent(recipient.to_string()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_ports::output::notification_port::{
        DeliveryCapabilities, NotificationDeliveryResult, NotificationPortResult,
        NotificationStatus,
    };
    use std::collections::HashSet;
    use std::time::Duration;

    fn interval_schedule() -> paladin_core::platform::container::schedule::Schedule {
        paladin_core::platform::container::schedule::Schedule::Interval(Duration::from_secs(60))
    }

    fn allow_only(action: BridgeAction) -> BridgePolicy {
        let mut allowed = HashSet::new();
        allowed.insert(action);
        BridgePolicy::new(allowed, 1, 1, 1, 1)
    }

    struct MockDeliveryPort;

    #[async_trait]
    impl NotificationDeliveryPort for MockDeliveryPort {
        fn channel(&self) -> NotificationChannel {
            NotificationChannel::System
        }
        fn can_handle(&self, _notification: &Notification) -> bool {
            true
        }
        async fn deliver_notification(
            &self,
            notification: Notification,
        ) -> NotificationPortResult<NotificationDeliveryResult> {
            Ok(NotificationDeliveryResult {
                notification_id: notification.id,
                status: NotificationStatus::Delivered,
                external_id: Some("mock".to_string()),
                processing_time_ms: 1,
                error_message: None,
                delivered_at: chrono::Utc::now(),
                channel: notification.channel,
                metadata: Default::default(),
            })
        }
        async fn health_check(&self) -> bool {
            true
        }
        fn capabilities(&self) -> DeliveryCapabilities {
            DeliveryCapabilities {
                supports_bulk: false,
                supports_receipts: false,
                supports_attachments: false,
                supports_rich_content: false,
                supports_templates: false,
                max_attachment_size: None,
                rate_limit: None,
            }
        }
    }

    #[tokio::test]
    async fn schedule_job_success() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(Arc::clone(&orchestrator), BridgePolicy::default());
        let result = adapter
            .schedule_job(ScheduleJobRequest {
                name: "follow-up".to_string(),
                description: "re-analyze tomorrow".to_string(),
                schedule: interval_schedule(),
            })
            .await;
        assert!(result.is_ok());
        let stats = orchestrator.get_stats().await;
        assert_eq!(stats.scheduler_stats.total_jobs, 1);
    }

    #[tokio::test]
    async fn schedule_job_action_not_allowed() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(orchestrator, allow_only(BridgeAction::FireEvent));
        let err = adapter
            .schedule_job(ScheduleJobRequest {
                name: "x".to_string(),
                description: "y".to_string(),
                schedule: interval_schedule(),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, OrchestratorBridgeError::ActionNotAllowed(_)));
    }

    #[tokio::test]
    async fn schedule_job_quota_exceeded() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(orchestrator, allow_only(BridgeAction::ScheduleJob));
        // First call within cap (1).
        assert!(
            adapter
                .schedule_job(ScheduleJobRequest {
                    name: "a".to_string(),
                    description: "d".to_string(),
                    schedule: interval_schedule(),
                })
                .await
                .is_ok()
        );
        // Second call exceeds cap.
        let err = adapter
            .schedule_job(ScheduleJobRequest {
                name: "b".to_string(),
                description: "d".to_string(),
                schedule: interval_schedule(),
            })
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            OrchestratorBridgeError::QuotaExceeded { limit: 1, .. }
        ));
    }

    #[tokio::test]
    async fn queue_item_success() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter = OrchestratorBridgeAdapter::new(orchestrator, BridgePolicy::default());
        let result = adapter
            .queue_item(QueueItemRequest {
                queue_name: "analysis".to_string(),
                payload: serde_json::json!({"doc": "1"}),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn queue_item_action_not_allowed() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(orchestrator, allow_only(BridgeAction::ScheduleJob));
        let err = adapter
            .queue_item(QueueItemRequest {
                queue_name: "q".to_string(),
                payload: serde_json::json!({}),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, OrchestratorBridgeError::ActionNotAllowed(_)));
    }

    #[tokio::test]
    async fn fire_event_success() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter = OrchestratorBridgeAdapter::new(orchestrator, BridgePolicy::default());
        let result = adapter
            .fire_event(FireEventRequest {
                event_type: "critical_finding".to_string(),
                payload: serde_json::json!({"severity": "high"}),
                source: "analyst_agent".to_string(),
            })
            .await;
        let dispatch = result.expect("event should dispatch");
        // No listeners registered, so no triggers are created.
        assert_eq!(dispatch.triggered_count, 0);
    }

    #[tokio::test]
    async fn fire_event_quota_exceeded() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(orchestrator, allow_only(BridgeAction::FireEvent));
        assert!(
            adapter
                .fire_event(FireEventRequest {
                    event_type: "e".to_string(),
                    payload: serde_json::json!({}),
                    source: "s".to_string(),
                })
                .await
                .is_ok()
        );
        let err = adapter
            .fire_event(FireEventRequest {
                event_type: "e".to_string(),
                payload: serde_json::json!({}),
                source: "s".to_string(),
            })
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            OrchestratorBridgeError::QuotaExceeded { limit: 1, .. }
        ));
    }

    #[tokio::test]
    async fn send_notification_success() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter = OrchestratorBridgeAdapter::new(orchestrator, BridgePolicy::default())
            .with_notification_port(Arc::new(MockDeliveryPort));
        let result = adapter
            .send_notification(SendNotificationRequest {
                channel: "system".to_string(),
                recipient: "ops".to_string(),
                subject: "alert".to_string(),
                body: "critical finding".to_string(),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn send_notification_without_port_fails() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter = OrchestratorBridgeAdapter::new(orchestrator, BridgePolicy::default());
        let err = adapter
            .send_notification(SendNotificationRequest {
                channel: "system".to_string(),
                recipient: "ops".to_string(),
                subject: "alert".to_string(),
                body: "body".to_string(),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, OrchestratorBridgeError::InvalidRequest(_)));
    }

    #[tokio::test]
    async fn send_notification_action_not_allowed() {
        let orchestrator = Arc::new(Orchestrator::new());
        let adapter =
            OrchestratorBridgeAdapter::new(orchestrator, allow_only(BridgeAction::ScheduleJob))
                .with_notification_port(Arc::new(MockDeliveryPort));
        let err = adapter
            .send_notification(SendNotificationRequest {
                channel: "system".to_string(),
                recipient: "ops".to_string(),
                subject: "alert".to_string(),
                body: "body".to_string(),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, OrchestratorBridgeError::ActionNotAllowed(_)));
    }
}
