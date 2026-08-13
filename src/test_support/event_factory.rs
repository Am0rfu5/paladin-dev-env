//! Deterministic [`Event`] construction for the listener trigger-condition coverage work in
//! plans 15-08 and 15-09.
//!
//! Determinism is the point: given the same arguments, [`build_event`] and [`build_event_batch`]
//! produce events with the same identity and timestamp fields, every time. Nothing here reads
//! the clock or a random source — identities are derived from the caller's arguments via a fixed
//! (non-cryptographic, deterministic-within-a-process) hash, and timestamps are offset from a
//! fixed base rather than sampled from the wall clock. A test that fails only on a slow runner
//! is worse than no test.
//!
//! No wrapper is provided around `tokio::time::pause()`/`advance()` — those are standard tokio
//! and a wrapper would add indirection for nothing.

use crate::core::base::component::event::Event;
use crate::core::platform::container::trigger::TriggerCondition;
use chrono::{DateTime, Duration, Utc};
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use thiserror::Error;
use uuid::Uuid;

/// The fixed source recorded on every event this factory constructs.
const FACTORY_SOURCE: &str = "test_support::event_factory";

/// A sentinel event type designed not to satisfy an ordinary domain
/// [`TriggerCondition::event_type_pattern`]. Wrapped in Unicode Private Use Area characters
/// (`U+E000`), which legitimate event types are not expected to contain.
const NON_MATCHING_EVENT_TYPE: &str = "\u{E000}gsd-test-support-non-matching-event\u{E000}";

/// Errors that can occur while deterministically constructing test [`Event`]s.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EventFactoryError {
    /// The condition's `event_type_pattern` matches every possible event type (for example
    /// `"*"`), so no event type exists that would deliberately fail to match it.
    #[error(
        "condition's event_type_pattern `{0}` matches every possible event type; no \
         non-matching event type can be constructed"
    )]
    UnavoidableMatch(String),
}

/// Builds a fully populated, deterministic [`Event`] for the given type and payload.
///
/// The identity ([`Event::id`]) and [`Event::timestamp`] are derived deterministically from
/// `event_type` and `payload` rather than from the clock or a random source, so calling this
/// twice with the same arguments returns events equal on their identity fields.
///
/// # Examples
///
/// ```ignore
/// use paladin::test_support::event_factory::build_event;
/// use serde_json::json;
///
/// let event = build_event("content_ingested", json!({ "id": "abc" }));
/// assert_eq!(event.event_type, "content_ingested");
/// ```
pub fn build_event(event_type: &str, payload: Value) -> Event {
    build_indexed_event(event_type, payload, 0)
}

/// Builds an [`Event`] whose `event_type` is guaranteed not to satisfy `condition`'s
/// [`TriggerCondition::event_type_pattern`] — usable to exercise a listener's no-match path
/// without hand-assembling a near-miss event.
///
/// Returns [`EventFactoryError::UnavoidableMatch`] if `condition`'s pattern matches every
/// possible event type (for example `"*"`), since in that case no non-matching event type
/// exists to construct.
///
/// # Examples
///
/// ```ignore
/// use paladin::core::platform::container::trigger::TriggerCondition;
/// use paladin::test_support::event_factory::build_non_matching_event;
/// use serde_json::json;
///
/// let condition = TriggerCondition {
///     event_type_pattern: "content_*".to_string(),
///     source_pattern: None,
///     payload_conditions: vec![],
///     min_priority: None,
///     time_conditions: None,
/// };
/// let event = build_non_matching_event(&condition, json!({}));
/// assert!(event.is_ok());
/// ```
pub fn build_non_matching_event(
    condition: &TriggerCondition,
    payload: Value,
) -> Result<Event, EventFactoryError> {
    if matches_wildcard(&condition.event_type_pattern, NON_MATCHING_EVENT_TYPE) {
        return Err(EventFactoryError::UnavoidableMatch(
            condition.event_type_pattern.clone(),
        ));
    }
    Ok(build_event(NON_MATCHING_EVENT_TYPE, payload))
}

/// Builds `count` distinct, deterministic [`Event`]s of the given type in one call — what makes
/// DEFER-03's 1000-plus-event burst expressible without a loop copied into several tests.
///
/// Each event has a pairwise-distinct identity and an index-derived payload, and calling this
/// twice with the same `event_type` and `count` returns vectors equal element-for-element on
/// their identity fields.
///
/// # Examples
///
/// ```ignore
/// use paladin::test_support::event_factory::build_event_batch;
///
/// let events = build_event_batch("burst_event", 1000);
/// assert_eq!(events.len(), 1000);
/// ```
pub fn build_event_batch(event_type: &str, count: usize) -> Vec<Event> {
    (0..count)
        .map(|index| build_indexed_event(event_type, serde_json::json!({ "index": index }), index))
        .collect()
}

/// Shared construction path for [`build_event`] and [`build_event_batch`]: derives a
/// deterministic identity and timestamp from `event_type`, `payload` and `index`.
fn build_indexed_event(event_type: &str, payload: Value, index: usize) -> Event {
    let id = deterministic_uuid(event_type, &payload, index);
    let timestamp = DateTime::<Utc>::default() + Duration::seconds(index as i64);
    Event {
        id,
        event_type: event_type.to_string(),
        payload,
        timestamp,
        source: FACTORY_SOURCE.to_string(),
        correlation_id: None,
        version: "1.0".to_string(),
    }
}

/// Derives a deterministic [`Uuid`] from `event_type`, `payload` and `index` using two
/// differently-salted passes of the standard library's fixed-seed [`DefaultHasher`] — no
/// randomness, no clock, and stable for repeated calls within a single test run.
fn deterministic_uuid(event_type: &str, payload: &Value, index: usize) -> Uuid {
    let seed = format!("{event_type}|{index}|{payload}");

    let mut first_hasher = DefaultHasher::new();
    seed.hash(&mut first_hasher);
    let first = first_hasher.finish();

    let mut second_hasher = DefaultHasher::new();
    ("salt-two", &seed, first).hash(&mut second_hasher);
    let second = second_hasher.finish();

    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&first.to_be_bytes());
    bytes[8..].copy_from_slice(&second.to_be_bytes());
    Uuid::from_bytes(bytes)
}

/// Mirrors `Trigger::matches_pattern`'s single-wildcard semantics (exact match, or a `*`
/// splitting the pattern into a prefix and suffix) so [`build_non_matching_event`] can determine
/// whether a candidate type would satisfy a condition's pattern. Kept in sync manually, since
/// the real method is private to `crates/paladin-core/src/platform/container/trigger.rs`.
fn matches_wildcard(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return value.starts_with(parts[0]) && value.ends_with(parts[1]);
        }
    }
    pattern == value
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn build_event_sets_requested_type_and_payload() {
        let payload = serde_json::json!({ "key": "value" });
        let event = build_event("probe_event", payload.clone());
        assert_eq!(event.event_type, "probe_event");
        assert_eq!(event.payload, payload);
        assert_eq!(event.source, FACTORY_SOURCE);
    }

    #[test]
    fn build_event_is_deterministic() {
        let payload = serde_json::json!({ "key": "value" });
        let first = build_event("probe_event", payload.clone());
        let second = build_event("probe_event", payload);
        assert_eq!(first.id, second.id);
        assert_eq!(first.timestamp, second.timestamp);
    }

    #[test]
    fn build_event_batch_is_deterministic() {
        let first = build_event_batch("determinism_probe", 25);
        let second = build_event_batch("determinism_probe", 25);

        let first_ids: Vec<Uuid> = first.iter().map(|event| event.id).collect();
        let second_ids: Vec<Uuid> = second.iter().map(|event| event.id).collect();
        assert_eq!(first_ids, second_ids);

        let first_timestamps: Vec<_> = first.iter().map(|event| event.timestamp).collect();
        let second_timestamps: Vec<_> = second.iter().map(|event| event.timestamp).collect();
        assert_eq!(first_timestamps, second_timestamps);
    }

    #[test]
    fn build_event_batch_produces_distinct_identities_for_a_thousand_events() {
        let events = build_event_batch("distinctness_probe", 1000);
        assert_eq!(events.len(), 1000);

        let unique_ids: HashSet<Uuid> = events.iter().map(|event| event.id).collect();
        assert_eq!(unique_ids.len(), 1000);
    }

    #[test]
    fn build_non_matching_event_avoids_a_satisfiable_prefix_pattern() {
        let condition = TriggerCondition {
            event_type_pattern: "content_*".to_string(),
            source_pattern: None,
            payload_conditions: vec![],
            min_priority: None,
            time_conditions: None,
        };

        let result = build_non_matching_event(&condition, serde_json::json!({}));
        assert!(result.is_ok());
        if let Ok(event) = result {
            assert!(!matches_wildcard(
                &condition.event_type_pattern,
                &event.event_type
            ));
        }
    }

    #[test]
    fn build_non_matching_event_avoids_a_satisfiable_exact_pattern() {
        let condition = TriggerCondition {
            event_type_pattern: "user_created".to_string(),
            source_pattern: None,
            payload_conditions: vec![],
            min_priority: None,
            time_conditions: None,
        };

        let result = build_non_matching_event(&condition, serde_json::json!({}));
        assert!(result.is_ok());
        if let Ok(event) = result {
            assert_ne!(event.event_type, condition.event_type_pattern);
        }
    }

    #[test]
    fn build_non_matching_event_rejects_a_wildcard_that_matches_everything() {
        let condition = TriggerCondition {
            event_type_pattern: "*".to_string(),
            source_pattern: None,
            payload_conditions: vec![],
            min_priority: None,
            time_conditions: None,
        };

        let result = build_non_matching_event(&condition, serde_json::json!({}));
        assert!(matches!(
            result,
            Err(EventFactoryError::UnavoidableMatch(_))
        ));
    }

    #[test]
    fn matches_wildcard_handles_prefix_suffix_and_exact_patterns() {
        assert!(matches_wildcard("*", "anything"));
        assert!(matches_wildcard("test_*", "test_created"));
        assert!(!matches_wildcard("test_*", "other_created"));
        assert!(matches_wildcard("*_created", "user_created"));
        assert!(!matches_wildcard("*_created", "user_deleted"));
        assert!(matches_wildcard("exact_type", "exact_type"));
        assert!(!matches_wildcard("exact_type", "other_type"));
    }
}
