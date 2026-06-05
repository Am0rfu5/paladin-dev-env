/*
Listener Orchestrator

Application-layer orchestrator for event listener management. Relocated from
`core/platform/manager/listener_service.rs`.

Renamed `ListenerService` → `ListenerOrchestrator`. A backwards-compatible type
alias `ListenerService = ListenerOrchestrator` is provided.

`ListenerConfig` and `ListenerStats` are re-exported from their canonical location
in `paladin-core::platform::container::trigger`.
*/

use super::types::ListenerError;
use crate::core::base::component::event::Event;
use crate::core::platform::container::trigger::{
    Trigger, TriggerCondition, TriggerStatus, TriggerSummary,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub use crate::core::platform::container::trigger::{ListenerConfig, ListenerStats};

/// Event listener trait.
#[async_trait]
pub trait EventListener: Send + Sync {
    /// Get the listener name.
    fn name(&self) -> &str;

    /// Get the listener description.
    fn description(&self) -> &str;

    /// Get the trigger conditions this listener matches.
    fn conditions(&self) -> &[TriggerCondition];

    /// Check if this listener should process the given event.
    async fn should_process(&self, event: &Event) -> bool;

    /// Create a trigger for the given event.
    async fn create_trigger(&self, event: Event) -> Result<Trigger, ListenerError>;

    /// Get listener configuration.
    fn config(&self) -> &ListenerConfig;

    /// Update listener configuration.
    fn update_config(&mut self, config: ListenerConfig);

    /// Health check for the listener.
    async fn health_check(&self) -> Result<bool, ListenerError>;
}

/// Internal listener wrapper.
struct ListenerWrapper {
    listener: Box<dyn EventListener>,
    stats: ListenerStats,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
    trigger_count_window: VecDeque<DateTime<Utc>>,
}

impl std::fmt::Debug for ListenerWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListenerWrapper")
            .field("listener_name", &self.listener.name())
            .field("listener_description", &self.listener.description())
            .field("listener_conditions", &self.listener.conditions())
            .field("listener_enabled", &self.listener.config().enabled)
            .field("stats", &self.stats)
            .field("created_at", &self.created_at)
            .field("last_updated", &self.last_updated)
            .field("trigger_count_window_len", &self.trigger_count_window.len())
            .finish()
    }
}

impl ListenerWrapper {
    fn new(listener: Box<dyn EventListener>) -> Self {
        let now = Utc::now();
        Self {
            stats: ListenerStats {
                name: listener.name().to_string(),
                enabled: listener.config().enabled,
                events_processed: 0,
                triggers_created: 0,
                triggers_completed: 0,
                triggers_failed: 0,
                average_processing_time_ms: None,
                last_event_processed: None,
                last_trigger_created: None,
            },
            listener,
            created_at: now,
            last_updated: now,
            trigger_count_window: VecDeque::new(),
        }
    }

    fn can_create_trigger(&mut self) -> bool {
        let config = self.listener.config();
        if !config.enabled {
            return false;
        }

        // Clean up old entries from the time window
        let window_start =
            Utc::now() - chrono::Duration::seconds(config.time_window_seconds as i64);
        while let Some(&front_time) = self.trigger_count_window.front() {
            if front_time < window_start {
                self.trigger_count_window.pop_front();
            } else {
                break;
            }
        }

        // Check if we're under the limit
        self.trigger_count_window.len() < config.max_triggers_per_window
    }

    fn record_trigger_created(&mut self) {
        self.trigger_count_window.push_back(Utc::now());
        self.stats.triggers_created += 1;
        self.stats.last_trigger_created = Some(Utc::now());
        self.last_updated = Utc::now();
    }

    fn record_event_processed(&mut self) {
        self.stats.events_processed += 1;
        self.stats.last_event_processed = Some(Utc::now());
        self.last_updated = Utc::now();
    }
}

/// Application-layer orchestrator for event listeners.
///
/// Renamed from `ListenerService`. A backwards-compatible type alias is provided.
#[derive(Debug)]
pub struct ListenerOrchestrator {
    listeners: Arc<RwLock<HashMap<String, Arc<Mutex<ListenerWrapper>>>>>,
    triggers: Arc<RwLock<HashMap<Uuid, Trigger>>>,
    trigger_queue: Arc<Mutex<VecDeque<Uuid>>>,
}

/// Backwards-compatible alias for `ListenerOrchestrator`.
pub type ListenerService = ListenerOrchestrator;

impl ListenerOrchestrator {
    /// Create a new listener orchestrator.
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            triggers: Arc::new(RwLock::new(HashMap::new())),
            trigger_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Create a listener orchestrator with a custom default configuration.
    pub fn with_default_config(_config: ListenerConfig) -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            triggers: Arc::new(RwLock::new(HashMap::new())),
            trigger_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Register a new event listener.
    pub async fn register_listener(
        &self,
        listener: Box<dyn EventListener>,
    ) -> Result<(), ListenerError> {
        let name = listener.name().to_string();
        let wrapper = Arc::new(Mutex::new(ListenerWrapper::new(listener)));

        let mut listeners = self.listeners.write().await;
        listeners.insert(name, wrapper);
        Ok(())
    }

    /// Unregister an event listener.
    pub async fn unregister_listener(&self, name: &str) -> Result<(), ListenerError> {
        let mut listeners = self.listeners.write().await;
        listeners
            .remove(name)
            .ok_or_else(|| ListenerError::ListenerNotFound(name.to_string()))?;
        Ok(())
    }

    /// Process an event through all registered listeners.
    pub async fn process_event(&self, event: Event) -> Result<Vec<Uuid>, ListenerError> {
        let mut created_triggers = Vec::new();
        let listeners = self.listeners.read().await;

        for listener_wrapper in listeners.values() {
            let mut wrapper = listener_wrapper.lock().await;

            // Record that we processed an event
            wrapper.record_event_processed();

            // Check if listener should process this event
            if !wrapper.listener.should_process(&event).await {
                continue;
            }

            // Check rate limiting
            if !wrapper.can_create_trigger() {
                continue;
            }

            // Create trigger
            match wrapper.listener.create_trigger(event.clone()).await {
                Ok(trigger) => {
                    let trigger_id = trigger.id;

                    // Store the trigger
                    {
                        let mut triggers = self.triggers.write().await;
                        triggers.insert(trigger_id, trigger);
                    }

                    // Add to processing queue
                    {
                        let mut queue = self.trigger_queue.lock().await;
                        queue.push_back(trigger_id);
                    }

                    // Record metrics
                    wrapper.record_trigger_created();
                    created_triggers.push(trigger_id);
                }
                Err(e) => {
                    // Log error but continue processing with other listeners
                    log::error!(
                        "Failed to create trigger for listener {}: {}",
                        wrapper.listener.name(),
                        e
                    );
                }
            }
        }

        Ok(created_triggers)
    }

    /// Get the next trigger to process.
    pub async fn get_next_trigger(&self) -> Option<Trigger> {
        let trigger_id = {
            let mut queue = self.trigger_queue.lock().await;
            queue.pop_front()
        }?;

        let mut triggers = self.triggers.write().await;
        triggers.remove(&trigger_id)
    }

    /// Get a specific trigger.
    pub async fn get_trigger(&self, trigger_id: Uuid) -> Result<Trigger, ListenerError> {
        let triggers = self.triggers.read().await;
        triggers
            .get(&trigger_id)
            .cloned()
            .ok_or(ListenerError::TriggerNotFound(trigger_id))
    }

    /// Update trigger status.
    pub async fn update_trigger_status(
        &self,
        trigger_id: Uuid,
        trigger: Trigger,
    ) -> Result<(), ListenerError> {
        // Update listener stats based on trigger status
        {
            let listeners = self.listeners.read().await;
            if let Some(listener_wrapper) = listeners.get(&trigger.source) {
                let mut wrapper = listener_wrapper.lock().await;
                match trigger.status {
                    TriggerStatus::Completed => wrapper.stats.triggers_completed += 1,
                    TriggerStatus::Failed => wrapper.stats.triggers_failed += 1,
                    _ => {}
                }
            }
        }

        // Store updated trigger if it should be preserved
        if trigger.config.preserve_after_completion || trigger.status != TriggerStatus::Completed {
            let mut triggers = self.triggers.write().await;
            triggers.insert(trigger_id, trigger);
        }

        Ok(())
    }

    /// Get listener statistics.
    pub async fn get_listener_stats(&self, name: &str) -> Result<ListenerStats, ListenerError> {
        let listeners = self.listeners.read().await;
        let wrapper = listeners
            .get(name)
            .ok_or_else(|| ListenerError::ListenerNotFound(name.to_string()))?;

        let wrapper_guard = wrapper.lock().await;
        Ok(wrapper_guard.stats.clone())
    }

    /// List all registered listeners.
    pub async fn list_listeners(&self) -> Vec<String> {
        let listeners = self.listeners.read().await;
        listeners.keys().cloned().collect()
    }

    /// Get all listener statistics.
    pub async fn get_all_stats(&self) -> HashMap<String, ListenerStats> {
        let listeners = self.listeners.read().await;
        let mut stats = HashMap::new();

        for (name, wrapper) in listeners.iter() {
            let wrapper_guard = wrapper.lock().await;
            stats.insert(name.clone(), wrapper_guard.stats.clone());
        }

        stats
    }

    /// Enable or disable a listener.
    pub async fn set_listener_enabled(
        &self,
        name: &str,
        enabled: bool,
    ) -> Result<(), ListenerError> {
        let listeners = self.listeners.read().await;
        let wrapper = listeners
            .get(name)
            .ok_or_else(|| ListenerError::ListenerNotFound(name.to_string()))?;

        let mut wrapper_guard = wrapper.lock().await;
        let mut config = wrapper_guard.listener.config().clone();
        config.enabled = enabled;
        wrapper_guard.listener.update_config(config);
        wrapper_guard.stats.enabled = enabled;
        wrapper_guard.last_updated = Utc::now();

        Ok(())
    }

    /// Get trigger queue length.
    pub async fn trigger_queue_length(&self) -> usize {
        let queue = self.trigger_queue.lock().await;
        queue.len()
    }

    /// Get trigger summaries for monitoring.
    pub async fn get_trigger_summaries(&self) -> Vec<TriggerSummary> {
        let triggers = self.triggers.read().await;
        triggers.values().map(|t| t.summary()).collect()
    }

    /// Cleanup expired triggers.
    pub async fn cleanup_expired_triggers(&self) {
        let mut triggers = self.triggers.write().await;
        let expired_ids: Vec<_> = triggers
            .iter()
            .filter(|(_, trigger)| trigger.is_expired())
            .map(|(id, _)| *id)
            .collect();

        for id in expired_ids {
            triggers.remove(&id);
        }
    }

    /// Health check for all listeners.
    pub async fn health_check(&self) -> Result<HashMap<String, bool>, ListenerError> {
        let listeners = self.listeners.read().await;
        let mut health_status = HashMap::new();

        for (name, wrapper) in listeners.iter() {
            let wrapper_guard = wrapper.lock().await;
            match wrapper_guard.listener.health_check().await {
                Ok(healthy) => {
                    health_status.insert(name.clone(), healthy);
                }
                Err(_) => {
                    health_status.insert(name.clone(), false);
                }
            }
        }

        Ok(health_status)
    }
}

impl Default for ListenerOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::component::action::Action;
    use crate::core::base::component::event::Event;
    use serde_json::json;

    // Mock listener for testing
    struct MockEventListener {
        name: String,
        config: ListenerConfig,
        conditions: Vec<TriggerCondition>,
    }

    #[async_trait]
    impl EventListener for MockEventListener {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "Mock listener for testing"
        }

        fn conditions(&self) -> &[TriggerCondition] {
            &self.conditions
        }

        async fn should_process(&self, event: &Event) -> bool {
            event.event_type.starts_with("test_")
        }

        async fn create_trigger(&self, event: Event) -> Result<Trigger, ListenerError> {
            let action = Action::new(
                "Test Action".to_string(),
                "Generated by mock listener".to_string(),
                self.name.clone(),
                "mock_service".to_string(),
            );

            let condition = TriggerCondition {
                event_type_pattern: "test_*".to_string(),
                source_pattern: None,
                payload_conditions: vec![],
                min_priority: None,
                time_conditions: None,
            };

            Ok(Trigger::new(
                format!("Trigger for {}", event.event_type),
                "Generated trigger".to_string(),
                self.name.clone(),
                "mock_service".to_string(),
                event,
                action,
                condition,
            ))
        }

        fn config(&self) -> &ListenerConfig {
            &self.config
        }

        fn update_config(&mut self, config: ListenerConfig) {
            self.config = config;
        }

        async fn health_check(&self) -> Result<bool, ListenerError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_listener_registration() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener {
            name: "test_listener".to_string(),
            config: ListenerConfig::default(),
            conditions: vec![],
        });

        let result = service.register_listener(listener).await;
        assert!(result.is_ok());

        let listeners = service.list_listeners().await;
        assert!(listeners.contains(&"test_listener".to_string()));
    }

    #[tokio::test]
    async fn test_event_processing() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener {
            name: "test_listener".to_string(),
            config: ListenerConfig::default(),
            conditions: vec![],
        });

        service.register_listener(listener).await.unwrap();

        let event = Event::new(
            "test_event".to_string(),
            json!({"data": "test"}),
            "test_source".to_string(),
        );

        let triggers = service.process_event(event).await.unwrap();
        assert_eq!(triggers.len(), 1);

        // Check that trigger was created
        let trigger = service.get_next_trigger().await;
        assert!(trigger.is_some());
    }

    #[tokio::test]
    async fn test_listener_stats() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener {
            name: "test_listener".to_string(),
            config: ListenerConfig::default(),
            conditions: vec![],
        });

        service.register_listener(listener).await.unwrap();

        let event = Event::new(
            "test_event".to_string(),
            json!({}),
            "test_source".to_string(),
        );

        service.process_event(event).await.unwrap();

        let stats = service.get_listener_stats("test_listener").await.unwrap();
        assert_eq!(stats.events_processed, 1);
        assert_eq!(stats.triggers_created, 1);
    }
}
