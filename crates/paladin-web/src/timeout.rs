//! Execution timeout policy and resolution (Milestone 12, Epic 3).
//!
//! Every execution path (buffered, streaming, job) is bounded by an effective timeout
//! resolved from three sources, in precedence order:
//!
//! 1. the per-request `timeout_seconds` (if supplied),
//! 2. the per-agent override (from config / `AgentSpec`),
//! 3. the server-wide [`TimeoutPolicy::default_secs`].
//!
//! The result is clamped to [`TimeoutPolicy::max_secs`]. A per-request value of `0` is
//! rejected (the handler maps it to `400`).

use std::time::Duration;

/// Server-wide timeout policy.
#[derive(Debug, Clone, Copy)]
pub struct TimeoutPolicy {
    /// Default timeout (seconds) when neither request nor agent specifies one.
    pub default_secs: u64,
    /// Maximum timeout (seconds); a per-request/agent value is clamped to this.
    pub max_secs: u64,
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        Self {
            default_secs: 300,
            max_secs: 600,
        }
    }
}

/// A per-request timeout override was invalid (non-positive).
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidTimeout;

/// Resolve the effective execution timeout (see the module docs for precedence).
///
/// # Errors
///
/// Returns [`InvalidTimeout`] if `request_secs` is `Some(0)`.
pub fn resolve_timeout(
    request_secs: Option<u64>,
    agent_secs: Option<u64>,
    policy: &TimeoutPolicy,
) -> Result<Duration, InvalidTimeout> {
    if request_secs == Some(0) {
        return Err(InvalidTimeout);
    }
    let secs = request_secs.or(agent_secs).unwrap_or(policy.default_secs);
    // Clamp to [1, max]: never zero (a 0 agent override shouldn't time out instantly).
    let clamped = secs.clamp(1, policy.max_secs.max(1));
    Ok(Duration::from_secs(clamped))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> TimeoutPolicy {
        TimeoutPolicy {
            default_secs: 30,
            max_secs: 60,
        }
    }

    #[test]
    fn request_override_takes_precedence() {
        assert_eq!(
            resolve_timeout(Some(10), Some(20), &policy()),
            Ok(Duration::from_secs(10))
        );
    }

    #[test]
    fn falls_back_to_agent_then_default() {
        assert_eq!(
            resolve_timeout(None, Some(20), &policy()),
            Ok(Duration::from_secs(20))
        );
        assert_eq!(
            resolve_timeout(None, None, &policy()),
            Ok(Duration::from_secs(30))
        );
    }

    #[test]
    fn clamps_to_max() {
        assert_eq!(
            resolve_timeout(Some(1000), None, &policy()),
            Ok(Duration::from_secs(60))
        );
        assert_eq!(
            resolve_timeout(None, Some(1000), &policy()),
            Ok(Duration::from_secs(60))
        );
    }

    #[test]
    fn zero_request_is_invalid() {
        assert_eq!(
            resolve_timeout(Some(0), None, &policy()),
            Err(InvalidTimeout)
        );
    }

    #[test]
    fn zero_agent_override_is_coerced_to_at_least_one_second() {
        assert_eq!(
            resolve_timeout(None, Some(0), &policy()),
            Ok(Duration::from_secs(1))
        );
    }
}
