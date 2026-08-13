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
    // ============================================================================
    // DEFER-03 entry record (plan 15-08, Task 1) — re-measurement before re-scoping
    // ============================================================================
    //
    // DEFER-03's own "Done when" clause requires a *current* `cargo llvm-cov` figure before the
    // remaining scope is stated — not the inherited register figure. This block is that
    // re-measurement, and the honest limitation encountered while producing it.
    //
    // **Command that would produce the entry figure** (per
    // `docs/src/contributing/testing-guide.md` § Test Coverage and ADR-0006's tool-of-record):
    //   rustup component add llvm-tools-preview
    //   cargo install cargo-llvm-cov --locked
    //   cargo llvm-cov --workspace --lib --json --output-path /tmp/cov-listener-entry.json
    //
    // **Scope:** default features, `--lib` only (not `--features integration-tests`, the
    // ADR-0006 gate's scope) — Docker is absent from this authoring environment, and per
    // ADR-0006 the two scopes are not comparable regardless (ignore regex, doctest decision and
    // feature set all have to match for two coverage runs to agree).
    //
    // **Result: NOT MEASURED.** `cargo-llvm-cov` is not installed in this execution environment,
    // and this plan's own harness instructions explicitly forbid installing it here (no Docker,
    // and installing the tool was called out as not worth the time in this session) rather than
    // fabricating a number. Verified absent by direct check: `command -v cargo-llvm-cov` returns
    // nothing.
    //   - Commit SHA at time of this entry: bd1924e3c17de458c3e9f5b457874040d7f51d82
    //   - Date: 2026-08-13
    //   - Prior figure being re-measured: 57.83% (602 LOC), dated 2026-02-14, recorded against
    //     the path `src/core/platform/manager/listener_service.rs` (relocated by Milestone 6
    //     Epic 2 to this file, 538 lines, `ListenerOrchestrator`).
    //   - **Delta: cannot be computed from a real number in this session.** This is not framed
    //     as a regression or an improvement — it is an unmeasured entry, explicitly recorded as
    //     such so a later session (or CI's `coverage` job, once it exists per PIPE-02) can fill
    //     in the real figure without this gap being silently carried forward as "57.83% still
    //     holds". The module has gained Milestone 9 Epic 2's match/no-match/fan-out/rate-limit/
    //     dispatch tests and moved path since the register's figure was struck, so 57.83% is
    //     known-stale in both directions (more code covered by more tests, but also more lines
    //     added by the relocation) — no directional claim is made without the number.
    //   - **Not comparable with the ADR-0006 CI gate figure** even once measured, per the scope
    //     note above (`--lib` default-feature scope here vs. `--features integration-tests`
    //     there).
    //
    // This plan (15-08) still proceeds to close the sequential half of DEFER-03's named scope
    // against a *qualitative* re-read of the existing test module plus Milestone 9 Epic 2's
    // history, per the plan's own instruction to state remaining scope "against the measured
    // figure" where a figure exists, and against direct code inspection where it does not. Plan
    // 15-09 takes the concurrency/stress half and is the natural place for a session with
    // `cargo-llvm-cov` available to fill in this gap.
    //
    // ## Remaining scope per DEFER-03's five named areas (stated against direct inspection,
    // since no current coverage figure exists to state it against)
    //
    // 1. **Registration and lifecycle** — genuinely under-covered before this plan. The three
    //    pre-existing tests below never registered a duplicate name, never unregistered anything
    //    (known or unknown), and never called `set_listener_enabled`. This plan closes all
    //    three.
    // 2. **Delivery and filtering, with ordering guarantees** — the pre-existing
    //    `test_event_processing` covers exactly one matching-event path with one listener. No
    //    non-matching event, no fan-out, and no ordering assertion existed. This plan adds all
    //    three, including a discovered fact about the real ordering guarantee (see
    //    `trigger_queue_is_fifo_across_sequential_process_event_calls` below) and an explicit
    //    non-guarantee for intra-call ordering across multiple matching listeners (`HashMap`
    //    iteration order is unspecified — the fan-out test asserts the *set* of producing
    //    listeners, not an order).
    // 3. **Trigger status tracking and retry** — entirely uncovered before this plan. No test
    //    called `get_trigger`, `update_trigger_status`, or `get_trigger_summaries`. This plan
    //    adds coverage for every `TriggerStatus` variant round-tripping through
    //    `update_trigger_status`/`get_trigger`, the `preserve_after_completion: false` +
    //    `Completed` non-preservation path, and a retry transition built on
    //    `Trigger::start_processing`/`fail_processing`.
    // 4. **Statistics and health-check status** — partially covered. `test_listener_stats`
    //    covers `get_listener_stats` for a known listener after one event. Unknown-listener
    //    lookups, `get_all_stats` after N events across two listeners, `trigger_queue_length`
    //    before/after, and `health_check` (all-healthy / one-unhealthy / none-registered) were
    //    all uncovered. This plan closes all of them.
    // 5. **Concurrency and stress** — genuinely uncovered, and explicitly **out of scope for
    //    this plan** per its own boundary. Milestone 9 Epic 2 added sequential match/no-match/
    //    fan-out/rate-limit/dispatch tests (visible in the pre-existing three-test module this
    //    plan extends), not concurrent producer/consumer or deadlock-detection coverage. Plan
    //    15-09 owns this half.
    //
    // Rate-limit and trigger-expiry boundaries (named directly in this plan's must_haves, not
    // one of DEFER-03's five register areas but part of the "delivery and filtering" /
    // lifecycle scope) are closed by this plan — see the discovered-behavior note on
    // `tokio::time::pause`/`advance` at
    // `rate_limit_boundary_exercised_at_below_at_and_above_the_limit` below.

    use super::*;
    use crate::core::base::component::action::Action;
    use crate::core::base::component::event::Event;
    use crate::test_support::event_factory::{
        build_event, build_event_batch, build_non_matching_event,
    };
    use serde_json::json;
    use std::collections::HashSet;
    use std::time::Duration;

    /// Configurable `should_process` behaviour for [`MockEventListener`] — added by plan 15-08 to
    /// serve cases the three pre-existing tests never needed (fixed match/no-match verdicts
    /// independent of the event's `event_type`).
    #[derive(Clone, Copy)]
    enum ShouldProcessBehavior {
        /// The original hardcoded rule every pre-existing construction site relied on:
        /// `event.event_type.starts_with("test_")`.
        Default,
        /// Always returns this fixed verdict, ignoring the event entirely.
        Fixed(bool),
    }

    /// Configurable `health_check` behaviour for [`MockEventListener`] — added by plan 15-08.
    #[derive(Clone)]
    enum HealthCheckBehavior {
        /// The original hardcoded result every pre-existing construction site relied on.
        Healthy,
        /// Reports unhealthy without erroring.
        Unhealthy,
        /// The health check itself fails.
        Err(String),
    }

    // Mock listener for testing
    struct MockEventListener {
        name: String,
        config: ListenerConfig,
        conditions: Vec<TriggerCondition>,
        should_process_behavior: ShouldProcessBehavior,
        health_check_behavior: HealthCheckBehavior,
    }

    impl MockEventListener {
        /// Builds a listener with the original defaults every pre-existing test relied on: the
        /// `"test_"`-prefix `should_process` rule and an `Ok(true)` health check. Existing
        /// construction sites are preserved behaviourally by routing through this constructor.
        fn new(name: &str, config: ListenerConfig) -> Self {
            Self {
                name: name.to_string(),
                config,
                conditions: vec![],
                should_process_behavior: ShouldProcessBehavior::Default,
                health_check_behavior: HealthCheckBehavior::Healthy,
            }
        }

        /// Overrides `should_process` to a fixed verdict, independent of the event.
        fn with_should_process(mut self, behavior: ShouldProcessBehavior) -> Self {
            self.should_process_behavior = behavior;
            self
        }

        /// Overrides `health_check`'s result.
        fn with_health_check(mut self, behavior: HealthCheckBehavior) -> Self {
            self.health_check_behavior = behavior;
            self
        }
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
            match self.should_process_behavior {
                ShouldProcessBehavior::Default => event.event_type.starts_with("test_"),
                ShouldProcessBehavior::Fixed(verdict) => verdict,
            }
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
            match &self.health_check_behavior {
                HealthCheckBehavior::Healthy => Ok(true),
                HealthCheckBehavior::Unhealthy => Ok(false),
                HealthCheckBehavior::Err(message) => {
                    Err(ListenerError::OperationFailed(message.clone()))
                }
            }
        }
    }

    /// Builds a `TriggerCondition` matching the `"test_*"` prefix `MockEventListener`'s default
    /// `should_process` rule uses, for use with `event_factory::build_non_matching_event`.
    fn test_prefix_condition() -> TriggerCondition {
        TriggerCondition {
            event_type_pattern: "test_*".to_string(),
            source_pattern: None,
            payload_conditions: vec![],
            min_priority: None,
            time_conditions: None,
        }
    }

    #[tokio::test]
    async fn test_listener_registration() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener::new(
            "test_listener",
            ListenerConfig::default(),
        ));

        let result = service.register_listener(listener).await;
        assert!(result.is_ok());

        let listeners = service.list_listeners().await;
        assert!(listeners.contains(&"test_listener".to_string()));
    }

    #[tokio::test]
    async fn test_event_processing() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener::new(
            "test_listener",
            ListenerConfig::default(),
        ));

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

        let listener = Box::new(MockEventListener::new(
            "test_listener",
            ListenerConfig::default(),
        ));

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

    // ========================================================================
    // Registration and lifecycle (DEFER-03 area 1)
    // ========================================================================

    #[tokio::test]
    async fn registering_a_duplicate_name_replaces_rather_than_rejects() {
        // Observed verdict, not presumed: `register_listener` performs an unconditional
        // `HashMap::insert`, so registering a second listener under a name already in use
        // silently *replaces* the first — it does not return an error and does not merge state.
        let service = ListenerOrchestrator::new();

        let first = Box::new(
            MockEventListener::new("dup", ListenerConfig::default())
                .with_should_process(ShouldProcessBehavior::Fixed(true)),
        );
        let second = Box::new(
            MockEventListener::new("dup", ListenerConfig::default())
                .with_should_process(ShouldProcessBehavior::Fixed(false)),
        );

        assert!(service.register_listener(first).await.is_ok());
        assert!(service.register_listener(second).await.is_ok());

        // Exactly one "dup" entry survives -- not two, not an error.
        let listeners = service.list_listeners().await;
        assert_eq!(listeners.iter().filter(|n| *n == "dup").count(), 1);

        // Behavioral proof the *second* registration is the one active: since it always returns
        // `false` from `should_process`, no trigger is produced for an event the first listener
        // would have matched.
        let event = build_event("test_probe", json!({}));
        let triggers = service.process_event(event).await.unwrap();
        assert!(triggers.is_empty());
    }

    #[tokio::test]
    async fn unregistering_an_unknown_listener_returns_a_defined_error_not_a_panic() {
        let service = ListenerOrchestrator::new();

        let result = service.unregister_listener("never-registered").await;
        assert!(matches!(result, Err(ListenerError::ListenerNotFound(_))));
    }

    #[tokio::test]
    async fn unregistering_a_registered_listener_removes_it_from_listing_and_stats() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(MockEventListener::new("temp", ListenerConfig::default()));
        service.register_listener(listener).await.unwrap();

        assert!(service.list_listeners().await.contains(&"temp".to_string()));
        assert!(service.get_all_stats().await.contains_key("temp"));

        service.unregister_listener("temp").await.unwrap();

        assert!(!service.list_listeners().await.contains(&"temp".to_string()));
        assert!(!service.get_all_stats().await.contains_key("temp"));
    }

    #[tokio::test]
    async fn set_listener_enabled_effect_is_asserted_behaviorally_in_both_directions() {
        let service = ListenerOrchestrator::new();

        let listener = Box::new(
            MockEventListener::new("toggle", ListenerConfig::default())
                .with_should_process(ShouldProcessBehavior::Fixed(true)),
        );
        service.register_listener(listener).await.unwrap();

        // Enabled by default -- a matching event produces a trigger.
        let triggers = service
            .process_event(build_event("test_a", json!({})))
            .await
            .unwrap();
        assert_eq!(triggers.len(), 1);

        // Disable, then prove the effect by processing another event and observing no trigger --
        // not by reading the config flag back.
        service.set_listener_enabled("toggle", false).await.unwrap();
        let triggers = service
            .process_event(build_event("test_b", json!({})))
            .await
            .unwrap();
        assert!(triggers.is_empty());

        // Re-enable, then prove the effect the same way.
        service.set_listener_enabled("toggle", true).await.unwrap();
        let triggers = service
            .process_event(build_event("test_c", json!({})))
            .await
            .unwrap();
        assert_eq!(triggers.len(), 1);
    }

    // ========================================================================
    // Delivery and filtering, with ordering (DEFER-03 area 2)
    // ========================================================================

    #[tokio::test]
    async fn a_matching_event_produces_exactly_one_trigger() {
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(MockEventListener::new(
                "matcher",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();

        let triggers = service
            .process_event(build_event("test_match", json!({"k": "v"})))
            .await
            .unwrap();
        assert_eq!(triggers.len(), 1);
    }

    #[tokio::test]
    async fn a_non_matching_event_produces_no_trigger() {
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(MockEventListener::new(
                "matcher",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();

        let condition = test_prefix_condition();
        let event = build_non_matching_event(&condition, json!({}))
            .expect("\"test_*\" is satisfiable, so a deliberately non-matching event exists");

        let triggers = service.process_event(event).await.unwrap();
        assert!(triggers.is_empty());
    }

    #[tokio::test]
    async fn fan_out_to_three_matching_listeners_produces_one_trigger_per_listener() {
        let service = ListenerOrchestrator::new();
        let names = ["fan_a", "fan_b", "fan_c"];
        for name in names {
            service
                .register_listener(Box::new(MockEventListener::new(
                    name,
                    ListenerConfig::default(),
                )))
                .await
                .unwrap();
        }

        let triggers = service
            .process_event(build_event("test_fanout", json!({})))
            .await
            .unwrap();
        assert_eq!(triggers.len(), 3);

        // Assert the *set* of producing listeners, not merely the count.
        let mut producing_sources = HashSet::new();
        for trigger_id in &triggers {
            let trigger = service.get_trigger(*trigger_id).await.unwrap();
            producing_sources.insert(trigger.source);
        }
        let expected: HashSet<String> = names.iter().map(|n| n.to_string()).collect();
        assert_eq!(producing_sources, expected);
    }

    #[tokio::test]
    async fn trigger_queue_is_fifo_across_sequential_process_event_calls() {
        // Discovered, not presumed: `process_event` pushes every trigger it creates onto the
        // back of a `VecDeque` before returning, and `get_next_trigger` pops from the front. So
        // across *separate* `process_event` calls (each of which fully completes before the
        // next begins), dequeue order matches call order. This is the real ordering guarantee
        // the implementation provides.
        //
        // No such guarantee exists *within* one `process_event` call when more than one listener
        // matches: iteration is over `HashMap::values()`, whose order is unspecified. The
        // fan-out test above deliberately asserts set membership, not order, for that case.
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(MockEventListener::new(
                "sequencer",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();

        let first_ids = service
            .process_event(build_event("test_first", json!({"order": 1})))
            .await
            .unwrap();
        let second_ids = service
            .process_event(build_event("test_second", json!({"order": 2})))
            .await
            .unwrap();
        assert_eq!(first_ids.len(), 1);
        assert_eq!(second_ids.len(), 1);

        let first_dequeued = service.get_next_trigger().await.unwrap();
        let second_dequeued = service.get_next_trigger().await.unwrap();
        assert_eq!(first_dequeued.id, first_ids[0]);
        assert_eq!(second_dequeued.id, second_ids[0]);
    }

    // ========================================================================
    // Rate-limit and trigger-expiry boundaries
    // ========================================================================
    //
    // **Discovered constraint, recorded honestly per this plan's own values prohibition against
    // coverage theater:** `ListenerWrapper::can_create_trigger`, `record_trigger_created` and
    // `Trigger::is_expired` all read `chrono::Utc::now()` -- the real wall clock -- not
    // `tokio::time::Instant`. `tokio::time::pause()`/`tokio::time::advance()` control *tokio's*
    // virtual clock (`sleep`, `timeout`, `interval`) and have **no effect** on `chrono::Utc::now()`
    // reads. Calling them here would not make these tests' verdicts independent of real time; it
    // would be theater. CONTEXT.md's own "Claude's Discretion" section confirms this is
    // discretionary: "these are std tokio features needing no wrapper unless the listener tests
    // want a shared helper" -- it does not assert they control this module's clock, because they
    // do not.
    //
    // Genuine determinism is achieved differently, and without any real-time wait:
    //   - The rate-limit test drives an exact, small event count synchronously inside one test
    //     body, under a `time_window_seconds` large enough that the window cannot roll over
    //     during test execution. No sleep occurs, so no flakiness is possible.
    //   - The trigger-expiry test backdates `Trigger::created_at` directly (a public field) and
    //     re-stores the trigger through the public `update_trigger_status` API, then calls
    //     `cleanup_expired_triggers()` once. `Utc::now()` is read exactly once per constructed
    //     trigger to compute the offset -- never awaited or slept on.
    //
    // `tokio::time::pause()` is still called below, once, as defensive hygiene against any
    // incidental `tokio::time::sleep`/`timeout` a future edit to this test module might
    // introduce -- not because it gates any assertion here. This keeps the module honest about
    // what actually provides determinism while satisfying the spirit of "no new test reaches its
    // assertion by waiting on wall-clock elapsed time".

    #[tokio::test]
    async fn rate_limit_boundary_exercised_at_below_at_and_above_the_limit() {
        tokio::time::pause(); // see the module-level note above: inert against chrono::Utc::now()

        let config = ListenerConfig {
            max_triggers_per_window: 3,
            // Large enough that the window cannot naturally roll over during this test's
            // synchronous execution -- avoids any real-time dependency.
            time_window_seconds: 300,
            ..ListenerConfig::default()
        };
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(
                MockEventListener::new("rate_limited", config)
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();

        // Window starts empty (0 < 3): accepted. Window becomes 1 -- one below the limit.
        let t1 = service
            .process_event(build_event("evt", json!({"i": 1})))
            .await
            .unwrap();
        assert_eq!(t1.len(), 1, "event 1: window 0 -> 1, below the limit");

        // Window is 1 (1 < 3): accepted. Window becomes 2 -- one below the limit.
        let t2 = service
            .process_event(build_event("evt", json!({"i": 2})))
            .await
            .unwrap();
        assert_eq!(t2.len(), 1, "event 2: window 1 -> 2, one below the limit");

        // Window is 2 (2 < 3): accepted. Window becomes 3 -- exactly at the configured limit.
        let t3 = service
            .process_event(build_event("evt", json!({"i": 3})))
            .await
            .unwrap();
        assert_eq!(t3.len(), 1, "event 3: window 2 -> 3, exactly at the limit");

        // Window is 3 (3 < 3 is false): rejected -- one event above the limit.
        let t4 = service
            .process_event(build_event("evt", json!({"i": 4})))
            .await
            .unwrap();
        assert!(
            t4.is_empty(),
            "event 4: window at 3, one above the limit -- rejected"
        );
    }

    #[tokio::test]
    async fn trigger_expiry_exercised_at_the_boundary_and_either_side() {
        tokio::time::pause(); // see the module-level note above: inert against chrono::Utc::now()

        let service = ListenerOrchestrator::new();
        let ttl_seconds: i64 = 60;

        // Builds a trigger backdated by `age_seconds`, stored directly via the public
        // `update_trigger_status` API (no listener needs to be registered -- the stats-update
        // half of that method silently no-ops when `trigger.source` matches no listener).
        async fn store_backdated_trigger(
            service: &ListenerOrchestrator,
            source: &str,
            ttl_seconds: i64,
            age_seconds: i64,
        ) -> Uuid {
            let event = Event::new(
                "test_expiry".to_string(),
                json!({}),
                "expiry_test".to_string(),
            );
            let action = Action::new(
                "Expiry Action".to_string(),
                "Backdated for expiry boundary testing".to_string(),
                source.to_string(),
                "mock_service".to_string(),
            );
            let condition = TriggerCondition {
                event_type_pattern: "test_*".to_string(),
                source_pattern: None,
                payload_conditions: vec![],
                min_priority: None,
                time_conditions: None,
            };
            let mut trigger = Trigger::new(
                "Expiry Trigger".to_string(),
                "Backdated trigger".to_string(),
                source.to_string(),
                "mock_service".to_string(),
                event,
                action,
                condition,
            );
            trigger.config.ttl_seconds = ttl_seconds as u64;
            trigger.created_at = Utc::now() - chrono::Duration::seconds(age_seconds);
            let id = trigger.id;
            service.update_trigger_status(id, trigger).await.unwrap();
            id
        }

        // is_expired() checks `age_seconds > ttl_seconds` -- strictly greater, so exactly-at-TTL
        // is NOT expired.
        let one_below_id =
            store_backdated_trigger(&service, "below", ttl_seconds, ttl_seconds - 1).await;
        let exactly_at_id = store_backdated_trigger(&service, "at", ttl_seconds, ttl_seconds).await;
        let one_above_id =
            store_backdated_trigger(&service, "above", ttl_seconds, ttl_seconds + 1).await;

        let population_before = service.get_trigger_summaries().await.len();
        assert_eq!(population_before, 3);
        // Discovered, not presumed: `cleanup_expired_triggers` only touches the `triggers` map,
        // never `trigger_queue` -- these backdated triggers were stored via
        // `update_trigger_status`, which never pushes onto the processing queue, so the queue
        // length is unaffected by either storing them or cleaning them up.
        let queue_length_before = service.trigger_queue_length().await;

        service.cleanup_expired_triggers().await;

        let queue_length_after = service.trigger_queue_length().await;
        assert_eq!(
            queue_length_before, queue_length_after,
            "cleanup_expired_triggers does not touch the processing queue"
        );

        // One below the boundary: survives.
        assert!(service.get_trigger(one_below_id).await.is_ok());
        // Exactly at the boundary: survives (`>`, not `>=`).
        assert!(service.get_trigger(exactly_at_id).await.is_ok());
        // One above the boundary: removed.
        assert!(matches!(
            service.get_trigger(one_above_id).await,
            Err(ListenerError::TriggerNotFound(_))
        ));

        let population_after = service.get_trigger_summaries().await.len();
        assert_eq!(population_after, 2, "exactly one of the three was expired");
    }

    // ========================================================================
    // Trigger status tracking and retry (DEFER-03 area 3)
    // ========================================================================

    #[tokio::test]
    async fn get_trigger_distinguishes_known_from_unknown_ids() {
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(
                MockEventListener::new("status_source", ListenerConfig::default())
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();

        let ids = service
            .process_event(build_event("evt", json!({})))
            .await
            .unwrap();
        let known_id = ids[0];

        assert!(service.get_trigger(known_id).await.is_ok());
        assert!(matches!(
            service.get_trigger(Uuid::new_v4()).await,
            Err(ListenerError::TriggerNotFound(_))
        ));
    }

    #[tokio::test]
    async fn every_supported_trigger_status_round_trips_through_update_and_get() {
        let service = ListenerOrchestrator::new();

        let statuses = [
            TriggerStatus::Pending,
            TriggerStatus::Processing,
            TriggerStatus::Completed,
            TriggerStatus::Failed,
            TriggerStatus::Cancelled,
            TriggerStatus::Skipped,
            TriggerStatus::Expired,
        ];

        for status in statuses {
            let event = Event::new(
                "test_status".to_string(),
                json!({}),
                "status_src".to_string(),
            );
            let action = Action::new(
                "Status Action".to_string(),
                "Status round-trip".to_string(),
                "status_src".to_string(),
                "mock_service".to_string(),
            );
            let condition = test_prefix_condition();
            let mut trigger = Trigger::new(
                "Status Trigger".to_string(),
                "Status round-trip".to_string(),
                "status_src".to_string(),
                "mock_service".to_string(),
                event,
                action,
                condition,
            );
            trigger.status = status.clone();
            let id = trigger.id;

            service.update_trigger_status(id, trigger).await.unwrap();

            // preserve_after_completion defaults to true, so every variant here round-trips.
            let stored = service.get_trigger(id).await.unwrap();
            assert_eq!(stored.status, status);
        }
    }

    #[tokio::test]
    async fn completed_trigger_with_preservation_disabled_is_not_retrievable() {
        // Observed verdict: `update_trigger_status` only re-inserts when
        // `preserve_after_completion || status != Completed`. A trigger that is both `Completed`
        // and configured not to preserve itself is therefore intentionally dropped, not stored.
        let service = ListenerOrchestrator::new();

        let event = Event::new("test_drop".to_string(), json!({}), "drop_src".to_string());
        let action = Action::new(
            "Drop Action".to_string(),
            "Non-preserved completion".to_string(),
            "drop_src".to_string(),
            "mock_service".to_string(),
        );
        let condition = test_prefix_condition();
        let mut trigger = Trigger::new(
            "Drop Trigger".to_string(),
            "Non-preserved completion".to_string(),
            "drop_src".to_string(),
            "mock_service".to_string(),
            event,
            action,
            condition,
        );
        trigger.status = TriggerStatus::Completed;
        trigger.config.preserve_after_completion = false;
        let id = trigger.id;

        service.update_trigger_status(id, trigger).await.unwrap();

        assert!(matches!(
            service.get_trigger(id).await,
            Err(ListenerError::TriggerNotFound(_))
        ));
    }

    #[tokio::test]
    async fn a_retry_transition_is_observed_through_attempt_count_and_status() {
        // "Retry" at the orchestrator boundary is whatever `update_trigger_status` is handed:
        // `Trigger::start_processing` increments `attempt_count`; `Trigger::fail_processing`
        // decides, via the underlying `Action`'s retry policy, whether to reset status to
        // `Pending` (retryable) or move to `Failed` (exhausted). This test observes that real
        // behavior rather than presuming a specific retry-count contract at the orchestrator
        // level, which owns none of this logic itself -- it only stores whatever `Trigger` state
        // it is given.
        let service = ListenerOrchestrator::new();

        let event = Event::new("test_retry".to_string(), json!({}), "retry_src".to_string());
        let action = Action::new(
            "Retry Action".to_string(),
            "Retry transition".to_string(),
            "retry_src".to_string(),
            "mock_service".to_string(),
        );
        let condition = test_prefix_condition();
        let mut trigger = Trigger::new(
            "Retry Trigger".to_string(),
            "Retry transition".to_string(),
            "retry_src".to_string(),
            "mock_service".to_string(),
            event,
            action,
            condition,
        );
        let id = trigger.id;

        trigger.start_processing("worker-1".to_string()).unwrap();
        assert_eq!(trigger.attempt_count, 1);
        service
            .update_trigger_status(id, trigger.clone())
            .await
            .unwrap();
        assert_eq!(
            service.get_trigger(id).await.unwrap().status,
            TriggerStatus::Processing
        );

        let can_retry = trigger.fail_processing("transient failure".to_string());
        assert!(
            can_retry,
            "a fresh Action defaults to retryable with room under max_retries"
        );
        assert_eq!(trigger.status, TriggerStatus::Pending);
        // fail_processing does not itself bump Trigger::attempt_count -- only
        // start_processing does. This is the retry-count contract as implemented, not assumed.
        assert_eq!(trigger.attempt_count, 1);

        service
            .update_trigger_status(id, trigger.clone())
            .await
            .unwrap();
        let stored = service.get_trigger(id).await.unwrap();
        assert_eq!(stored.status, TriggerStatus::Pending);
        assert_eq!(stored.attempt_count, 1);
    }

    // ========================================================================
    // Idempotency
    // ========================================================================

    #[tokio::test]
    async fn processing_the_same_event_twice_is_not_deduplicated() {
        // Observed, not presumed: process_event carries no dedup guard keyed on event identity.
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(
                MockEventListener::new("no_dedup", ListenerConfig::default())
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();

        let event = build_event("evt", json!({"payload": "shared"}));

        let first = service.process_event(event.clone()).await.unwrap();
        let second = service.process_event(event).await.unwrap();

        assert_eq!(first.len(), 1);
        assert_eq!(
            second.len(),
            1,
            "the same event processed again produces another trigger"
        );
        assert_ne!(
            first[0], second[0],
            "each processing pass creates a distinct trigger id, even for an identical event"
        );
    }

    // ========================================================================
    // Statistics and health (DEFER-03 area 4)
    // ========================================================================

    #[tokio::test]
    async fn get_listener_stats_distinguishes_known_from_unknown_listeners() {
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(MockEventListener::new(
                "known",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();

        assert!(service.get_listener_stats("known").await.is_ok());
        assert!(matches!(
            service.get_listener_stats("unknown").await,
            Err(ListenerError::ListenerNotFound(_))
        ));
    }

    #[tokio::test]
    async fn get_all_stats_reflects_events_processed_across_all_listeners_after_n_events() {
        // Discovered, not presumed: every registered listener's `events_processed` increments
        // for *every* event handed to `process_event`, whether or not that listener matched it.
        // `triggers_created` only increments on an actual match + successful creation.
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(
                MockEventListener::new("matches_all", ListenerConfig::default())
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();
        service
            .register_listener(Box::new(
                MockEventListener::new("matches_none", ListenerConfig::default())
                    .with_should_process(ShouldProcessBehavior::Fixed(false)),
            ))
            .await
            .unwrap();

        for i in 0..3 {
            service
                .process_event(build_event("evt", json!({"i": i})))
                .await
                .unwrap();
        }

        let stats = service.get_all_stats().await;
        assert_eq!(stats["matches_all"].events_processed, 3);
        assert_eq!(stats["matches_all"].triggers_created, 3);
        assert_eq!(stats["matches_none"].events_processed, 3);
        assert_eq!(stats["matches_none"].triggers_created, 0);
    }

    #[tokio::test]
    async fn trigger_queue_length_reflects_processing_before_and_after_draining() {
        let service = ListenerOrchestrator::new();
        service
            .register_listener(Box::new(
                MockEventListener::new("queued", ListenerConfig::default())
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();

        assert_eq!(service.trigger_queue_length().await, 0);

        service
            .process_event(build_event("evt", json!({"i": 1})))
            .await
            .unwrap();
        service
            .process_event(build_event("evt", json!({"i": 2})))
            .await
            .unwrap();
        assert_eq!(service.trigger_queue_length().await, 2);

        service.get_next_trigger().await;
        assert_eq!(service.trigger_queue_length().await, 1);

        service.get_next_trigger().await;
        assert_eq!(service.trigger_queue_length().await, 0);
    }

    #[tokio::test]
    async fn health_check_reports_map_contents_for_all_healthy_one_unhealthy_and_none_registered() {
        // None registered: the map itself is empty, not a size-1 default.
        let service = ListenerOrchestrator::new();
        let health = service.health_check().await.unwrap();
        assert!(health.is_empty());

        // All healthy.
        service
            .register_listener(Box::new(MockEventListener::new(
                "healthy_one",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();
        service
            .register_listener(Box::new(MockEventListener::new(
                "healthy_two",
                ListenerConfig::default(),
            )))
            .await
            .unwrap();
        let health = service.health_check().await.unwrap();
        assert_eq!(health.len(), 2);
        assert_eq!(health.get("healthy_one"), Some(&true));
        assert_eq!(health.get("healthy_two"), Some(&true));

        // One unhealthy: assert map contents distinguish it, not merely a count.
        service
            .register_listener(Box::new(
                MockEventListener::new("unhealthy_one", ListenerConfig::default())
                    .with_health_check(HealthCheckBehavior::Unhealthy),
            ))
            .await
            .unwrap();
        let health = service.health_check().await.unwrap();
        assert_eq!(health.len(), 3);
        assert_eq!(health.get("healthy_one"), Some(&true));
        assert_eq!(health.get("healthy_two"), Some(&true));
        assert_eq!(health.get("unhealthy_one"), Some(&false));

        // A health check that itself errors is also reported `false`, distinct from a healthy
        // report but not distinguishable from a plain "unhealthy" verdict in the returned map --
        // an observed API-shape fact, not a presumed richer error channel.
        service
            .register_listener(Box::new(
                MockEventListener::new("erroring_one", ListenerConfig::default())
                    .with_health_check(HealthCheckBehavior::Err("boom".to_string())),
            ))
            .await
            .unwrap();
        let health = service.health_check().await.unwrap();
        assert_eq!(health.get("erroring_one"), Some(&false));
    }

    // ========================================================================
    // Concurrency and stress (DEFER-03 area 5, plan 15-09)
    // ========================================================================
    //
    // Every test below runs under `#[tokio::test(flavor = "multi_thread")]` -- a single-threaded
    // runtime cannot surface a real lock-ordering problem between `listeners` (`RwLock`),
    // `triggers` (`RwLock`) and `trigger_queue` (`Mutex`) -- and every concurrent section is
    // wrapped in an explicit `tokio::time::timeout` with a generous but finite bound, asserted
    // not to have elapsed. A lock-ordering deadlock therefore fails the test with a named
    // panic/timeout rather than hanging the runner until a workflow-level timeout kills the
    // whole CI job with no useful signal (T-15-22).
    //
    // Every listener registered below is given a `max_triggers_per_window` comfortably above
    // the number of events it will actually see and a `time_window_seconds` large enough not to
    // roll over during the test, so the rate limiter documented and exercised above cannot
    // silently absorb part of the exact totals these tests assert (T-15-25's exactness
    // requirement would otherwise be defeated by an unrelated boundary this suite already
    // covers elsewhere).

    /// A rate-limit configuration wide enough that it never interferes with the exact-count
    /// assertions the concurrency tests below make -- `max_triggers_per_window` comfortably
    /// exceeds any event count used here, and `time_window_seconds` is large enough that the
    /// window cannot naturally roll over during a test's execution.
    fn non_interfering_config() -> ListenerConfig {
        ListenerConfig {
            max_triggers_per_window: 100_000,
            time_window_seconds: 3600,
            ..ListenerConfig::default()
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn concurrent_emission_from_multiple_producers_yields_the_exact_expected_trigger_total() {
        const PRODUCERS: usize = 8;
        const EVENTS_PER_PRODUCER: usize = 50;
        const LISTENERS: usize = 3;

        let service = Arc::new(ListenerOrchestrator::new());
        for i in 0..LISTENERS {
            service
                .register_listener(Box::new(
                    MockEventListener::new(
                        &format!("multi_producer_listener_{i}"),
                        non_interfering_config(),
                    )
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
                ))
                .await
                .unwrap();
        }

        let mut handles = Vec::with_capacity(PRODUCERS);
        for producer_id in 0..PRODUCERS {
            let service_clone = Arc::clone(&service);
            handles.push(tokio::spawn(async move {
                let mut produced = 0usize;
                for event_index in 0..EVENTS_PER_PRODUCER {
                    let event = build_event(
                        "test_concurrent_multi_producer",
                        json!({"producer": producer_id, "event": event_index}),
                    );
                    produced += service_clone.process_event(event).await.unwrap().len();
                }
                produced
            }));
        }

        let total = tokio::time::timeout(Duration::from_secs(30), async move {
            let mut sum = 0usize;
            for handle in handles {
                sum += handle.await.expect("producer task must not panic");
            }
            sum
        })
        .await
        .expect(
            "multi-producer emission must complete inside the timeout -- a lock-ordering \
             deadlock between `listeners` and `trigger_queue` would hang here instead of failing",
        );

        // Exact equality, not a lower bound: a lost update under `RwLock`/`Mutex` contention
        // would show up as a count below this product, which a `>=` assertion would silently
        // tolerate.
        assert_eq!(
            total,
            PRODUCERS * EVENTS_PER_PRODUCER * LISTENERS,
            "trigger total must equal the exact arithmetic product of producers, events and \
             matching listeners"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn concurrent_registration_and_unregistration_during_active_processing_stays_consistent()
    {
        let service = Arc::new(ListenerOrchestrator::new());

        // Two listeners the churn task never touches, so the processing task always has
        // something stable to hit regardless of how the churn task interleaves.
        for name in ["steady_a", "steady_b"] {
            service
                .register_listener(Box::new(
                    MockEventListener::new(name, non_interfering_config())
                        .with_should_process(ShouldProcessBehavior::Fixed(true)),
                ))
                .await
                .unwrap();
        }

        let processor_handle = Arc::clone(&service);
        let processor = tokio::spawn(async move {
            for i in 0..300 {
                processor_handle
                    .process_event(build_event("test_registration_churn", json!({"i": i})))
                    .await
                    .unwrap();
                tokio::task::yield_now().await;
            }
        });

        const CHURN_LISTENERS: usize = 50;
        let churn_handle = Arc::clone(&service);
        let churner = tokio::spawn(async move {
            for i in 0..CHURN_LISTENERS {
                let name = format!("churn_{i}");
                churn_handle
                    .register_listener(Box::new(MockEventListener::new(
                        &name,
                        non_interfering_config(),
                    )))
                    .await
                    .unwrap();
                tokio::task::yield_now().await;
                // Unregister every even-indexed churn listener, deliberately leaving the
                // odd-indexed half registered -- this exercises both register and unregister
                // concurrently with active processing rather than only one direction.
                if i % 2 == 0 {
                    churn_handle.unregister_listener(&name).await.unwrap();
                }
                tokio::task::yield_now().await;
            }
        });

        tokio::time::timeout(Duration::from_secs(30), async {
            let (processor_result, churner_result) = tokio::join!(processor, churner);
            processor_result.expect("processing task must not panic");
            churner_result.expect("registration/unregistration churn task must not panic");
        })
        .await
        .expect(
            "registration/unregistration churn concurrent with active processing must complete \
             inside the timeout -- a lock-ordering problem between the listener map and the \
             trigger queue would hang here instead of failing",
        );

        // Consistency check: `list_listeners` and `get_all_stats` must agree exactly on the
        // surviving set -- no listener present in one and absent from the other, and no call
        // above returned a poisoned-state error (every `.unwrap()`/`.expect()` above already
        // asserts that).
        let listed: HashSet<String> = service.list_listeners().await.into_iter().collect();
        let stat_keys: HashSet<String> = service.get_all_stats().await.into_keys().collect();
        assert_eq!(
            listed, stat_keys,
            "list_listeners and get_all_stats must agree on the surviving listener set"
        );

        assert!(listed.contains("steady_a"));
        assert!(listed.contains("steady_b"));
        let surviving_churn = listed.iter().filter(|n| n.starts_with("churn_")).count();
        assert_eq!(
            surviving_churn,
            CHURN_LISTENERS / 2,
            "exactly the odd-indexed (never-unregistered) half of the churn listeners should \
             remain"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_1000_plus_event_burst_across_several_producers_yields_exact_aggregate_counts() {
        const BURST_SIZE: usize = 1200;
        const LISTENERS: usize = 2;
        const PRODUCER_TASKS: usize = 4;

        let service = Arc::new(ListenerOrchestrator::new());
        for i in 0..LISTENERS {
            service
                .register_listener(Box::new(
                    MockEventListener::new(
                        &format!("burst_listener_{i}"),
                        non_interfering_config(),
                    )
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
                ))
                .await
                .unwrap();
        }

        // Built in one call via the shared bulk constructor -- what makes a 1000-plus-event
        // burst expressible without a hand-copied loop.
        let events = build_event_batch("test_burst", BURST_SIZE);
        let chunk_size = BURST_SIZE.div_ceil(PRODUCER_TASKS);

        let mut handles = Vec::with_capacity(PRODUCER_TASKS);
        for chunk in events.chunks(chunk_size) {
            let chunk_owned: Vec<Event> = chunk.to_vec();
            let service_clone = Arc::clone(&service);
            handles.push(tokio::spawn(async move {
                for event in chunk_owned {
                    service_clone.process_event(event).await.unwrap();
                }
            }));
        }

        tokio::time::timeout(Duration::from_secs(60), async move {
            for handle in handles {
                handle.await.expect("burst producer task must not panic");
            }
        })
        .await
        .expect(
            "1000-plus-event burst across several concurrent producers must complete inside \
             the timeout -- a lock-ordering deadlock would hang here instead of failing",
        );

        // Exact equality on both the shared queue and every per-listener counter, so a
        // discrepancy between the two would surface rather than cancel out -- a lost increment
        // under `Mutex`/`RwLock` contention shows up as a count low by a small amount, which
        // only an exact assertion (not a range or a lower bound) would ever notice.
        let queue_length = service.trigger_queue_length().await;
        assert_eq!(
            queue_length,
            BURST_SIZE * LISTENERS,
            "trigger_queue_length must equal the exact arithmetic total across all listeners"
        );

        let stats = service.get_all_stats().await;
        for i in 0..LISTENERS {
            let name = format!("burst_listener_{i}");
            let listener_stats = stats
                .get(&name)
                .unwrap_or_else(|| panic!("listener {name} must still be registered"));
            assert_eq!(
                listener_stats.events_processed as usize, BURST_SIZE,
                "listener {name}: exact events_processed"
            );
            assert_eq!(
                listener_stats.triggers_created as usize, BURST_SIZE,
                "listener {name}: exact triggers_created"
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn dropping_the_orchestrator_during_active_processing_completes_without_panicking_or_leaking_a_lock()
     {
        // `ListenerOrchestrator` exposes no `shutdown()` method and defines no custom `Drop` --
        // confirmed by direct inspection of the struct and its `impl` block above, per this
        // task's own instruction to read the type before presuming an affordance exists. Every
        // field (`listeners`, `triggers`, `trigger_queue`) is `Arc`-wrapped, so the type's actual
        // "shutdown" behaviour is exactly ordinary `Arc` reference-counting: dropping one handle
        // while another clone is still live only decrements a refcount, and the underlying state
        // (and every lock inside it) is released only once the last strong reference is dropped.
        // This test exercises and asserts that real behaviour rather than an invented
        // `shutdown()` call, and records the absent affordance here for Task 2's justification
        // block per the plan's explicit instruction not to add one to production code.
        let primary = Arc::new(ListenerOrchestrator::new());
        primary
            .register_listener(Box::new(
                MockEventListener::new("shutdown_probe", non_interfering_config())
                    .with_should_process(ShouldProcessBehavior::Fixed(true)),
            ))
            .await
            .unwrap();

        let weak = Arc::downgrade(&primary);

        let worker_handle = Arc::clone(&primary);
        const IN_FLIGHT_EVENTS: usize = 200;
        let events = build_event_batch("test_shutdown_burst", IN_FLIGHT_EVENTS);
        let worker = tokio::spawn(async move {
            let mut total = 0usize;
            for event in events {
                total += worker_handle.process_event(event).await.unwrap().len();
            }
            total
        });

        // Drop this test's own handle immediately -- the spawned worker task holds its own
        // clone, so the orchestrator's state stays alive and reachable through it. This is the
        // "graceful shutdown during active processing" scenario as the type actually supports
        // it: dropping handles while a task is mid-flight, since no explicit shutdown affordance
        // exists.
        drop(primary);

        let total = tokio::time::timeout(Duration::from_secs(15), worker)
            .await
            .expect(
                "the worker task must complete inside the timeout -- a lock held across the \
                 drop above would hang this instead of failing",
            )
            .expect("worker task must not panic despite the handle being dropped mid-flight");
        assert_eq!(
            total, IN_FLIGHT_EVENTS,
            "every in-flight event must still have produced exactly one trigger despite the drop"
        );

        // The worker's own `Arc` clone is dropped when its spawned task completes and its stack
        // frame unwinds. With every strong reference now gone, the `Weak` upgrade must fail --
        // proof the orchestrator's state, and every lock inside it, was fully released rather
        // than left held by a dangling guard.
        assert!(
            weak.upgrade().is_none(),
            "all strong references must be gone once the worker task has completed, proving no \
             lock guard or clone was leaked across the drop"
        );
    }
}
