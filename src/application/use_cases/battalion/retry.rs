//! Retry Logic Utility
//!
//! Provides exponential backoff retry logic with jitter for Battalion operations.

use rand::Rng;
use std::time::Duration;

use crate::core::platform::container::battalion::RetryPolicy;

/// Calculate the next retry delay based on retry policy
///
/// Implements exponential backoff with optional jitter to prevent thundering herd.
///
/// # Arguments
///
/// * `policy` - The retry policy configuration
/// * `attempt` - Current attempt number (0-indexed)
///
/// # Returns
///
/// Duration to wait before next retry
///
/// # Example
///
/// ```
/// use paladin::application::use_cases::battalion::retry::calculate_retry_delay;
/// use paladin::core::platform::container::battalion::RetryPolicy;
/// use std::time::Duration;
///
/// // Test with jitter disabled for deterministic results
/// let mut policy = RetryPolicy::default();
/// policy.jitter = false;
///
/// let delay = calculate_retry_delay(&policy, 0);
/// assert_eq!(delay, Duration::from_millis(100));
///
/// let delay = calculate_retry_delay(&policy, 1);
/// assert_eq!(delay, Duration::from_millis(200));
/// ```
pub fn calculate_retry_delay(policy: &RetryPolicy, attempt: u32) -> Duration {
    let base_delay = policy.base_delay;

    let delay = if policy.exponential_backoff {
        // Exponential backoff: base_delay * 2^attempt
        let multiplier = 2u32.pow(attempt);
        base_delay * multiplier
    } else {
        // Linear backoff
        base_delay
    };

    // Cap at max_delay
    let delay = delay.min(policy.max_delay);

    // Add jitter if enabled
    if policy.jitter {
        add_jitter(delay)
    } else {
        delay
    }
}

/// Add jitter to a duration
///
/// Adds random jitter between 0% and 100% of the duration to prevent
/// synchronized retries (thundering herd).
///
/// # Arguments
///
/// * `duration` - Base duration
///
/// # Returns
///
/// Duration with jitter added (50-100% of original)
fn add_jitter(duration: Duration) -> Duration {
    let mut rng = rand::thread_rng();
    // Jitter factor between 0.5 and 1.0
    let jitter_factor = rng.gen_range(0.5..=1.0);
    duration.mul_f64(jitter_factor)
}

/// Check if retry should be attempted
///
/// # Arguments
///
/// * `policy` - The retry policy
/// * `attempt` - Current attempt number (0-indexed)
///
/// # Returns
///
/// `true` if another retry should be attempted, `false` if max attempts reached
///
/// # Example
///
/// ```
/// use paladin::application::use_cases::battalion::retry::should_retry;
/// use paladin::core::platform::container::battalion::RetryPolicy;
///
/// let policy = RetryPolicy::default(); // max_attempts = 3
/// assert!(should_retry(&policy, 0)); // First retry
/// assert!(should_retry(&policy, 1)); // Second retry
/// assert!(should_retry(&policy, 2)); // Third retry
/// assert!(!should_retry(&policy, 3)); // Max reached
/// ```
pub fn should_retry(policy: &RetryPolicy, attempt: u32) -> bool {
    attempt < policy.max_attempts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_calculate_retry_delay_linear() {
        let policy = RetryPolicy {
            exponential_backoff: false,
            jitter: false,
            base_delay: Duration::from_millis(100),
            ..Default::default()
        };

        // Linear backoff should always return base_delay
        assert_eq!(
            calculate_retry_delay(&policy, 0),
            Duration::from_millis(100)
        );
        assert_eq!(
            calculate_retry_delay(&policy, 1),
            Duration::from_millis(100)
        );
        assert_eq!(
            calculate_retry_delay(&policy, 2),
            Duration::from_millis(100)
        );
    }

    #[test]
    fn test_calculate_retry_delay_exponential() {
        let policy = RetryPolicy {
            exponential_backoff: true,
            jitter: false,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            ..Default::default()
        };

        // Exponential: base * 2^attempt
        assert_eq!(
            calculate_retry_delay(&policy, 0),
            Duration::from_millis(100)
        ); // 100 * 2^0
        assert_eq!(
            calculate_retry_delay(&policy, 1),
            Duration::from_millis(200)
        ); // 100 * 2^1
        assert_eq!(
            calculate_retry_delay(&policy, 2),
            Duration::from_millis(400)
        ); // 100 * 2^2
        assert_eq!(
            calculate_retry_delay(&policy, 3),
            Duration::from_millis(800)
        ); // 100 * 2^3
    }

    #[test]
    fn test_calculate_retry_delay_max_cap() {
        let mut policy = RetryPolicy::default();
        policy.exponential_backoff = true;
        policy.jitter = false;
        policy.base_delay = Duration::from_millis(100);
        policy.max_delay = Duration::from_millis(500);

        // Should be capped at max_delay
        assert_eq!(
            calculate_retry_delay(&policy, 10),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn test_calculate_retry_delay_with_jitter() {
        let mut policy = RetryPolicy::default();
        policy.exponential_backoff = false;
        policy.jitter = true;
        policy.base_delay = Duration::from_millis(100);

        // With jitter, delay should be between 50-100ms (50% to 100% of base)
        let delay = calculate_retry_delay(&policy, 0);
        assert!(delay >= Duration::from_millis(50));
        assert!(delay <= Duration::from_millis(100));
    }

    #[test]
    fn test_should_retry_within_limit() {
        let policy = RetryPolicy::default(); // max_attempts = 3

        assert!(should_retry(&policy, 0)); // First retry
        assert!(should_retry(&policy, 1)); // Second retry
        assert!(should_retry(&policy, 2)); // Third retry
    }

    #[test]
    fn test_should_retry_exceeds_limit() {
        let policy = RetryPolicy::default(); // max_attempts = 3

        assert!(!should_retry(&policy, 3)); // Exceeds max
        assert!(!should_retry(&policy, 4)); // Way over
    }

    #[test]
    fn test_should_retry_custom_limit() {
        let policy = RetryPolicy {
            max_attempts: 5,
            ..Default::default()
        };

        assert!(should_retry(&policy, 4)); // Within limit
        assert!(!should_retry(&policy, 5)); // At limit
    }

    #[test]
    fn test_add_jitter_range() {
        let duration = Duration::from_millis(1000);

        // Test jitter multiple times to ensure it's random but within range
        for _ in 0..10 {
            let jittered = add_jitter(duration);
            assert!(jittered >= Duration::from_millis(500)); // 50% minimum
            assert!(jittered <= Duration::from_millis(1000)); // 100% maximum
        }
    }

    #[test]
    fn test_exponential_backoff_sequence() {
        let policy = RetryPolicy {
            exponential_backoff: true,
            jitter: false,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(60),
            ..Default::default()
        };

        // Verify exponential growth
        let delays: Vec<Duration> = (0..5).map(|i| calculate_retry_delay(&policy, i)).collect();

        assert_eq!(delays[0], Duration::from_millis(10)); // 10 * 2^0
        assert_eq!(delays[1], Duration::from_millis(20)); // 10 * 2^1
        assert_eq!(delays[2], Duration::from_millis(40)); // 10 * 2^2
        assert_eq!(delays[3], Duration::from_millis(80)); // 10 * 2^3
        assert_eq!(delays[4], Duration::from_millis(160)); // 10 * 2^4
    }
}
