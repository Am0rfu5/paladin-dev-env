/*
Notification Orchestrator

Application-layer orchestrator for notification operations.  Relocated from
`core::platform::manager::notification_service` as part of Milestone 6 Epic 2 to
enforce hexagonal architecture boundaries: services that depend on ports or coordinate
cross-cutting infrastructure belong in the application layer, not the core domain.

This module:
- Coordinates notification lifecycle (create / send / schedule / cancel)
- Routes notifications to the appropriate channel handler
- Processes templates via a pluggable `NotificationTemplateProcessor`
- Publishes lifecycle events via unbounded channels
- Delegates low-level messaging to `MessageService` (core base service)
*/

pub mod types;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::core::base::entity::message::{Location, Message};
use crate::core::base::service::message_service::{MessageHandler, MessageResult, MessageService};
use crate::core::platform::container::notification::{
    Notification, NotificationChannel, NotificationContent, NotificationEvent,
    NotificationPriority, NotificationRecipient, NotificationStatus, NotificationTemplate,
};
// Domain types now live in paladin-core
use paladin_core::platform::container::notification::{
    NotificationServiceConfig, NotificationServiceStats,
};

pub use types::{
    NotificationChannelHandler, NotificationDeliveryResult, NotificationOrchestratorError,
    NotificationOrchestratorResult, NotificationTemplateProcessor,
};

// ---------------------------------------------------------------------------
// Backwards-compatibility type aliases
// ---------------------------------------------------------------------------

/// Alias kept for call-sites that still use the old `NotificationService` name.
pub type NotificationService = NotificationOrchestrator;

/// Alias kept for call-sites that still use the old error type name.
pub type NotificationServiceError = NotificationOrchestratorError;

/// Alias kept for call-sites that still use the old result type name.
pub type NotificationServiceResult<T> = NotificationOrchestratorResult<T>;

// ---------------------------------------------------------------------------
// Orchestrator
// ---------------------------------------------------------------------------

/// Application-layer notification orchestrator.
///
/// Coordinates notification routing, template processing, and lifecycle events.
/// Previously named `NotificationService`; renamed to emphasise that this is an
/// orchestration concern rather than a domain entity.
pub struct NotificationOrchestrator {
    config: NotificationServiceConfig,
    message_service: Arc<MessageService>,
    channel_handlers:
        Arc<RwLock<HashMap<NotificationChannel, Arc<dyn NotificationChannelHandler>>>>,
    template_processor: Arc<RwLock<Option<Arc<dyn NotificationTemplateProcessor>>>>,
    template_cache: Arc<RwLock<HashMap<String, NotificationTemplate>>>,
    active_notifications: Arc<RwLock<HashMap<Uuid, Notification>>>,
    stats: Arc<RwLock<NotificationServiceStats>>,
    event_senders: Arc<RwLock<Vec<mpsc::UnboundedSender<NotificationEvent>>>>,
    workers: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    is_running: Arc<RwLock<bool>>,
}

impl NotificationOrchestrator {
    /// Create a new `NotificationOrchestrator`.
    pub fn new(config: NotificationServiceConfig, message_service: Arc<MessageService>) -> Self {
        Self {
            config,
            message_service,
            channel_handlers: Arc::new(RwLock::new(HashMap::new())),
            template_processor: Arc::new(RwLock::new(None)),
            template_cache: Arc::new(RwLock::new(HashMap::new())),
            active_notifications: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(NotificationServiceStats::default())),
            event_senders: Arc::new(RwLock::new(Vec::new())),
            workers: Arc::new(RwLock::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the notification orchestrator.
    pub async fn start(&self) -> NotificationOrchestratorResult<()> {
        println!("Starting notification orchestrator");

        {
            let mut is_running = self.is_running.write().await;
            if *is_running {
                return Ok(());
            }
            *is_running = true;
        }

        self.message_service.start().await?;

        let notification_handler = Arc::new(NotificationMessageHandler::new(
            self.channel_handlers.clone(),
            self.stats.clone(),
        ));

        self.message_service
            .register_handler("notification".to_string(), notification_handler)
            .await?;

        self.start_workers().await?;

        println!("Notification orchestrator started successfully");
        Ok(())
    }

    /// Stop the notification orchestrator.
    pub async fn stop(&self) -> NotificationOrchestratorResult<()> {
        println!("Stopping notification orchestrator");

        {
            let mut is_running = self.is_running.write().await;
            if !*is_running {
                return Ok(());
            }
            *is_running = false;
        }

        let mut workers = self.workers.write().await;
        for worker in workers.drain(..) {
            worker.abort();
        }

        self.message_service.stop().await?;

        println!("Notification orchestrator stopped");
        Ok(())
    }

    /// Create a new notification.
    pub async fn create_notification(
        &self,
        recipient: NotificationRecipient,
        content: NotificationContent,
        channel: NotificationChannel,
        priority: NotificationPriority,
    ) -> NotificationOrchestratorResult<Notification> {
        println!(
            "Creating notification for recipient: {:?}, channel: {:?}",
            recipient, channel
        );

        let mut notification = Notification::new(recipient, content, channel, priority)?;

        if notification.expiry_time.is_none() {
            let expiry = Utc::now() + chrono::Duration::seconds(self.config.default_expiry_seconds);
            notification.set_expiry(expiry)?;
        }

        notification.max_retries = self.config.default_max_retries;

        {
            let mut active = self.active_notifications.write().await;
            active.insert(notification.id, notification.clone());
        }

        {
            let mut stats = self.stats.write().await;
            stats.notifications_created += 1;
            *stats
                .channel_stats
                .entry(notification.channel.clone())
                .or_insert(0) += 1;
            *stats
                .priority_stats
                .entry(notification.priority)
                .or_insert(0) += 1;
            stats.last_activity = Some(Utc::now());
            stats.active_notifications += 1;
        }

        self.publish_event(NotificationEvent::NotificationCreated {
            notification_id: notification.id,
            recipient: notification.recipient.clone(),
            channel: notification.channel.clone(),
            priority: notification.priority,
            created_at: notification.created_at,
        })
        .await;

        println!("Notification created: {}", notification.id);
        Ok(notification)
    }

    /// Send a notification immediately.
    pub async fn send_notification(
        &self,
        notification_id: Uuid,
    ) -> NotificationOrchestratorResult<NotificationDeliveryResult> {
        println!("Sending notification: {}", notification_id);

        let mut notification = {
            let mut active = self.active_notifications.write().await;
            active.remove(&notification_id).ok_or_else(|| {
                NotificationOrchestratorError::ValidationError(format!(
                    "Notification not found: {}",
                    notification_id
                ))
            })?
        };

        if notification.is_expired() {
            notification.update_status(NotificationStatus::Expired)?;
            self.publish_event(NotificationEvent::NotificationExpired {
                notification_id: notification.id,
                expired_at: Utc::now(),
            })
            .await;

            return Ok(NotificationDeliveryResult {
                notification_id: notification.id,
                status: NotificationStatus::Expired,
                external_id: None,
                processing_time_ms: 0,
                error_message: Some("Notification expired".to_string()),
                timestamp: Utc::now(),
            });
        }

        if notification.content.uses_template() {
            notification.content = self.process_template(&notification.content).await?;
        }

        notification.update_status(NotificationStatus::Queued)?;
        self.publish_event(NotificationEvent::NotificationQueued {
            notification_id: notification.id,
            channel: notification.channel.clone(),
            queued_at: Utc::now(),
        })
        .await;

        let handler = {
            let handlers = self.channel_handlers.read().await;
            handlers
                .get(&notification.channel)
                .cloned()
                .ok_or_else(|| {
                    NotificationOrchestratorError::ChannelNotAvailable(format!(
                        "No handler available for channel: {:?}",
                        notification.channel
                    ))
                })?
        };

        let result = handler.handle_notification(notification.clone()).await;

        match result {
            Ok(delivery_result) => {
                notification.update_status(delivery_result.status.clone())?;
                notification.external_id = delivery_result.external_id.clone();

                match delivery_result.status {
                    NotificationStatus::Sent => {
                        self.publish_event(NotificationEvent::NotificationSent {
                            notification_id: notification.id,
                            channel: notification.channel.clone(),
                            external_id: delivery_result.external_id.clone(),
                            sent_at: delivery_result.timestamp,
                        })
                        .await;
                    }
                    NotificationStatus::Failed => {
                        self.publish_event(NotificationEvent::NotificationFailed {
                            notification_id: notification.id,
                            error: delivery_result.error_message.clone().unwrap_or_default(),
                            retry_count: notification.retry_count,
                            failed_at: delivery_result.timestamp,
                        })
                        .await;
                    }
                    _ => {}
                }

                {
                    let mut stats = self.stats.write().await;
                    match delivery_result.status {
                        NotificationStatus::Sent | NotificationStatus::Delivered => {
                            stats.notifications_sent += 1;
                            if delivery_result.status == NotificationStatus::Delivered {
                                stats.notifications_delivered += 1;
                            }
                        }
                        NotificationStatus::Failed => {
                            stats.notifications_failed += 1;
                        }
                        _ => {}
                    }
                    stats.last_activity = Some(Utc::now());
                    stats.active_notifications -= 1;
                }

                println!(
                    "Notification {} processed with status: {:?}",
                    notification_id, delivery_result.status
                );
                Ok(delivery_result)
            }
            Err(error) => {
                eprintln!("Failed to send notification {}: {}", notification_id, error);

                if notification.can_retry()
                    && let Some(retry_status) =
                        notification.status.next_retry(notification.max_retries)
                {
                    notification.update_status(retry_status)?;
                    let retry_count = notification.retry_count;
                    {
                        let mut active = self.active_notifications.write().await;
                        active.insert(notification.id, notification);
                    }
                    return Ok(NotificationDeliveryResult {
                        notification_id,
                        status: NotificationStatus::Retry(retry_count),
                        external_id: None,
                        processing_time_ms: 0,
                        error_message: Some(error.to_string()),
                        timestamp: Utc::now(),
                    });
                }

                notification.update_status(NotificationStatus::Failed)?;
                self.publish_event(NotificationEvent::NotificationFailed {
                    notification_id: notification.id,
                    error: error.to_string(),
                    retry_count: notification.retry_count,
                    failed_at: Utc::now(),
                })
                .await;

                let mut stats = self.stats.write().await;
                stats.notifications_failed += 1;
                stats.active_notifications -= 1;

                Err(error)
            }
        }
    }

    /// Schedule a notification for future delivery.
    pub async fn schedule_notification(
        &self,
        notification_id: Uuid,
        scheduled_time: DateTime<Utc>,
    ) -> NotificationOrchestratorResult<()> {
        println!(
            "Scheduling notification {} for {}",
            notification_id, scheduled_time
        );

        let mut active = self.active_notifications.write().await;
        let notification = active.get_mut(&notification_id).ok_or_else(|| {
            NotificationOrchestratorError::ValidationError(format!(
                "Notification not found: {}",
                notification_id
            ))
        })?;

        notification.schedule(scheduled_time)?;

        self.publish_event(NotificationEvent::NotificationScheduled {
            notification_id: notification.id,
            scheduled_time,
            scheduled_at: Utc::now(),
        })
        .await;

        println!(
            "Notification {} scheduled for {}",
            notification_id, scheduled_time
        );
        Ok(())
    }

    /// Cancel a notification.
    pub async fn cancel_notification(
        &self,
        notification_id: Uuid,
        reason: Option<String>,
    ) -> NotificationOrchestratorResult<()> {
        println!("Cancelling notification: {}", notification_id);

        let mut active = self.active_notifications.write().await;
        if let Some(mut notification) = active.remove(&notification_id) {
            notification.update_status(NotificationStatus::Cancelled)?;

            self.publish_event(NotificationEvent::NotificationCancelled {
                notification_id: notification.id,
                reason,
                cancelled_at: Utc::now(),
            })
            .await;

            let mut stats = self.stats.write().await;
            stats.active_notifications -= 1;

            println!("Notification {} cancelled", notification_id);
        }

        Ok(())
    }

    /// Register a channel handler.
    pub async fn register_channel_handler(&self, handler: Arc<dyn NotificationChannelHandler>) {
        let channel = handler.channel();
        println!("Registering channel handler for: {:?}", channel);
        let mut handlers = self.channel_handlers.write().await;
        handlers.insert(channel, handler);
    }

    /// Set template processor.
    pub async fn set_template_processor(&self, processor: Arc<dyn NotificationTemplateProcessor>) {
        println!("Setting template processor");
        let mut template_processor = self.template_processor.write().await;
        *template_processor = Some(processor);
    }

    /// Add template to cache.
    pub async fn cache_template(
        &self,
        template: NotificationTemplate,
    ) -> NotificationOrchestratorResult<()> {
        template.validate()?;
        let mut cache = self.template_cache.write().await;
        cache.insert(template.id.clone(), template);
        Ok(())
    }

    /// Get service statistics.
    pub async fn get_stats(&self) -> NotificationServiceStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get active notifications count.
    pub async fn get_active_count(&self) -> usize {
        let active = self.active_notifications.read().await;
        active.len()
    }

    /// Subscribe to notification events.
    pub async fn subscribe_to_events(&self) -> mpsc::UnboundedReceiver<NotificationEvent> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut senders = self.event_senders.write().await;
        senders.push(sender);
        receiver
    }

    /// Check orchestrator health.
    pub async fn health_check(&self) -> bool {
        let is_running = *self.is_running.read().await;
        let message_service_healthy = self.message_service.health_check().await.unwrap_or(false);

        let handlers = self.channel_handlers.read().await;
        let mut healthy_handlers = 0;
        for handler in handlers.values() {
            if handler.health_check().await {
                healthy_handlers += 1;
            }
        }

        is_running && message_service_healthy && (handlers.is_empty() || healthy_handlers > 0)
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    async fn process_template(
        &self,
        content: &NotificationContent,
    ) -> NotificationOrchestratorResult<NotificationContent> {
        let template_id = content.template_id.as_ref().ok_or_else(|| {
            NotificationOrchestratorError::ValidationError("No template ID specified".to_string())
        })?;

        let template = {
            let cache = self.template_cache.read().await;
            cache.get(template_id).cloned().ok_or_else(|| {
                NotificationOrchestratorError::TemplateNotFound(template_id.clone())
            })?
        };

        let processor = {
            let processor_guard = self.template_processor.read().await;
            processor_guard
                .as_ref()
                .ok_or_else(|| {
                    NotificationOrchestratorError::ConfigurationError(
                        "No template processor configured".to_string(),
                    )
                })?
                .clone()
        };

        processor
            .render_template(&template, &content.template_variables)
            .await
    }

    async fn start_workers(&self) -> NotificationOrchestratorResult<()> {
        println!("Starting notification processing workers");
        let mut workers = self.workers.write().await;
        workers.push(self.start_scheduled_processor().await);
        workers.push(self.start_retry_processor().await);
        Ok(())
    }

    async fn start_scheduled_processor(&self) -> tokio::task::JoinHandle<()> {
        let active_notifications = self.active_notifications.clone();
        let interval = self.config.processing_interval_ms;

        tokio::spawn(async move {
            let mut interval_timer =
                tokio::time::interval(tokio::time::Duration::from_millis(interval));
            loop {
                interval_timer.tick().await;
                let ready_notifications: Vec<Uuid> = {
                    let active = active_notifications.read().await;
                    active
                        .values()
                        .filter(|n| n.should_send_now())
                        .map(|n| n.id)
                        .collect()
                };
                for notification_id in ready_notifications {
                    println!("Would send scheduled notification: {}", notification_id);
                }
            }
        })
    }

    async fn start_retry_processor(&self) -> tokio::task::JoinHandle<()> {
        let active_notifications = self.active_notifications.clone();
        let interval = self.config.processing_interval_ms * 2;

        tokio::spawn(async move {
            let mut interval_timer =
                tokio::time::interval(tokio::time::Duration::from_millis(interval));
            loop {
                interval_timer.tick().await;
                let retry_notifications: Vec<Uuid> = {
                    let active = active_notifications.read().await;
                    active
                        .values()
                        .filter(|n| {
                            n.can_retry() && matches!(n.status, NotificationStatus::Retry(_))
                        })
                        .map(|n| n.id)
                        .collect()
                };
                for notification_id in retry_notifications {
                    println!("Would retry notification: {}", notification_id);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        })
    }

    async fn publish_event(&self, event: NotificationEvent) {
        let senders = self.event_senders.read().await;
        let mut failed_senders = Vec::new();

        for (index, sender) in senders.iter().enumerate() {
            if sender.send(event.clone()).is_err() {
                failed_senders.push(index);
            }
        }

        if !failed_senders.is_empty() {
            drop(senders);
            let mut senders = self.event_senders.write().await;
            for &index in failed_senders.iter().rev() {
                senders.remove(index);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Internal message handler (not part of the public API)
// ---------------------------------------------------------------------------

struct NotificationMessageHandler {
    #[allow(dead_code)]
    channel_handlers:
        Arc<RwLock<HashMap<NotificationChannel, Arc<dyn NotificationChannelHandler>>>>,
    #[allow(dead_code)]
    stats: Arc<RwLock<NotificationServiceStats>>,
}

impl NotificationMessageHandler {
    fn new(
        channel_handlers: Arc<
            RwLock<HashMap<NotificationChannel, Arc<dyn NotificationChannelHandler>>>,
        >,
        stats: Arc<RwLock<NotificationServiceStats>>,
    ) -> Self {
        Self {
            channel_handlers,
            stats,
        }
    }
}

#[async_trait]
impl MessageHandler<NotificationContent> for NotificationMessageHandler {
    async fn handle_message(&self, message: Message<NotificationContent>) -> MessageResult<()> {
        println!("Handling notification message: {}", message.id);
        Ok(())
    }

    fn supported_destinations(&self) -> Vec<Location> {
        vec![
            Location::service("notification"),
            Location::external("notification"),
        ]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::service::message_service::MessageServiceConfig;

    #[tokio::test]
    async fn test_notification_orchestrator_creation() {
        let config = NotificationServiceConfig::default();
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let orchestrator = NotificationOrchestrator::new(config, message_service);
        assert!(!orchestrator.health_check().await);
    }

    #[tokio::test]
    async fn test_notification_creation() {
        let config = NotificationServiceConfig::default();
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let orchestrator = NotificationOrchestrator::new(config, message_service);

        let recipient = NotificationRecipient::Email("test@example.com".to_string());
        let content = NotificationContent::new(
            "Test".to_string(),
            "Test body".to_string(),
            "test".to_string(),
        );

        let notification = orchestrator
            .create_notification(
                recipient,
                content,
                NotificationChannel::Email,
                NotificationPriority::Normal,
            )
            .await;

        assert!(notification.is_ok());
        let stats = orchestrator.get_stats().await;
        assert_eq!(stats.notifications_created, 1);
    }

    #[tokio::test]
    async fn test_config_default_values() {
        let config = NotificationServiceConfig::default();
        assert_eq!(config.default_max_retries, 3);
        assert_eq!(config.default_expiry_seconds, 86400);
        assert!(config.enable_persistence);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.processing_interval_ms, 1000);
        assert_eq!(config.template_cache_size, 1000);
        assert_eq!(config.max_attachment_size, 25 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_orchestrator_start_and_stop() {
        let config = NotificationServiceConfig::default();
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let orchestrator = NotificationOrchestrator::new(config, message_service);

        assert!(!orchestrator.health_check().await);

        let start_result = orchestrator.start().await;
        assert!(start_result.is_ok());

        let start_again = orchestrator.start().await;
        assert!(start_again.is_ok());

        let stop_result = orchestrator.stop().await;
        assert!(stop_result.is_ok());

        let stop_again = orchestrator.stop().await;
        assert!(stop_again.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_notification_creation() {
        let config = NotificationServiceConfig::default();
        let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
        let orchestrator = NotificationOrchestrator::new(config, message_service);

        for i in 0..5 {
            let recipient = NotificationRecipient::Email(format!("user{}@example.com", i));
            let content = NotificationContent::new(
                format!("Subject {}", i),
                format!("Body {}", i),
                format!("test{}", i),
            );

            let result = orchestrator
                .create_notification(
                    recipient,
                    content,
                    NotificationChannel::Email,
                    NotificationPriority::Normal,
                )
                .await;
            assert!(result.is_ok());
        }

        let stats = orchestrator.get_stats().await;
        assert_eq!(stats.notifications_created, 5);
        assert_eq!(stats.active_notifications, 5);
        assert_eq!(orchestrator.get_active_count().await, 5);
    }
}
