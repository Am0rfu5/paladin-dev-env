//! Resource controls for Arsenal tool execution
//!
//! This module provides timeout and concurrency limiting capabilities for tool execution.
//! It ensures that tools don't run indefinitely and that the system doesn't become overloaded
//! with too many concurrent tool executions.
//!
//! # Examples
//!
//! ```rust,no_run
//! use paladin::infrastructure::adapters::arsenal::resource_controls::{
//!     TimeoutWrapper, ConcurrencyLimiter
//! };
//! use std::time::Duration;
//!
//! # async fn example() {
//! let timeout_wrapper = TimeoutWrapper::new(Duration::from_secs(30));
//! let limiter = ConcurrencyLimiter::new(5); // Max 5 concurrent executions
//! # }
//! ```

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::{Instant, timeout};

use crate::core::platform::container::arsenal::ArsenalError;

/// Wraps tool execution with a timeout
///
/// This wrapper ensures that tool executions don't run indefinitely.
/// If a tool exceeds the specified duration, execution is cancelled
/// and a timeout error is returned.
///
/// # Examples
///
/// ```rust,no_run
/// use paladin::infrastructure::adapters::arsenal::resource_controls::TimeoutWrapper;
/// use std::time::Duration;
///
/// # async fn example() {
/// let wrapper = TimeoutWrapper::new(Duration::from_secs(30));
///
/// let result = wrapper.execute(async {
///     // Your tool execution logic here
///     Ok::<_, String>("result".to_string())
/// }).await;
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TimeoutWrapper {
    /// Maximum duration for tool execution
    duration: Duration,
}

impl TimeoutWrapper {
    /// Creates a new TimeoutWrapper with the specified duration
    ///
    /// # Arguments
    ///
    /// * `duration` - Maximum time allowed for tool execution
    ///
    /// # Example
    ///
    /// ```rust
    /// use paladin::infrastructure::adapters::arsenal::resource_controls::TimeoutWrapper;
    /// use std::time::Duration;
    ///
    /// let wrapper = TimeoutWrapper::new(Duration::from_secs(60));
    /// ```
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    /// Executes a future with a timeout
    ///
    /// Returns the result if execution completes within the timeout,
    /// otherwise returns a Timeout error.
    ///
    /// # Arguments
    ///
    /// * `future` - The async operation to execute
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::Timeout` if execution exceeds the configured duration
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::infrastructure::adapters::arsenal::resource_controls::TimeoutWrapper;
    /// # use paladin::core::platform::container::arsenal::ArsenalError;
    /// # use std::time::Duration;
    /// # async fn example() -> Result<String, ArsenalError> {
    /// let wrapper = TimeoutWrapper::new(Duration::from_secs(30));
    ///
    /// wrapper.execute(async {
    ///     // Tool execution logic
    ///     Ok("success".to_string())
    /// }).await
    /// # }
    /// ```
    pub async fn execute<F, T, E>(&self, future: F) -> Result<T, ArsenalError>
    where
        F: Future<Output = Result<T, E>>,
        E: Into<ArsenalError>,
    {
        match timeout(self.duration, future).await {
            Ok(result) => result.map_err(|e| e.into()),
            Err(_) => Err(ArsenalError::Timeout(self.duration.as_secs())),
        }
    }

    /// Executes a future with a timeout and returns execution time
    ///
    /// Returns a tuple of (result, execution_time_ms)
    ///
    /// # Arguments
    ///
    /// * `future` - The async operation to execute
    ///
    /// # Returns
    ///
    /// A tuple containing the result and execution time in milliseconds
    pub async fn execute_with_timing<F, T, E>(&self, future: F) -> (Result<T, ArsenalError>, u64)
    where
        F: Future<Output = Result<T, E>>,
        E: Into<ArsenalError>,
    {
        let start = Instant::now();
        let result = self.execute(future).await;
        let elapsed_ms = start.elapsed().as_millis() as u64;
        (result, elapsed_ms)
    }
}

/// Limits the number of concurrent tool executions
///
/// This limiter uses a semaphore to ensure that only a specified number
/// of tools can execute concurrently. Additional requests will wait for
/// a permit to become available.
///
/// # Examples
///
/// ```rust,no_run
/// use paladin::infrastructure::adapters::arsenal::resource_controls::ConcurrencyLimiter;
///
/// # async fn example() {
/// let limiter = ConcurrencyLimiter::new(5); // Max 5 concurrent executions
///
/// let _permit = limiter.acquire().await;
/// // Tool execution happens here
/// // Permit is automatically released when dropped
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ConcurrencyLimiter {
    /// Semaphore for controlling concurrent access
    semaphore: Arc<Semaphore>,
}

impl ConcurrencyLimiter {
    /// Creates a new ConcurrencyLimiter
    ///
    /// # Arguments
    ///
    /// * `max_concurrent` - Maximum number of concurrent tool executions allowed
    ///
    /// # Example
    ///
    /// ```rust
    /// use paladin::infrastructure::adapters::arsenal::resource_controls::ConcurrencyLimiter;
    ///
    /// let limiter = ConcurrencyLimiter::new(10);
    /// ```
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Acquires a permit for tool execution
    ///
    /// This method will block (asynchronously) until a permit is available.
    /// The permit is automatically released when dropped.
    ///
    /// # Returns
    ///
    /// A permit guard that releases the permit when dropped
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::infrastructure::adapters::arsenal::resource_controls::ConcurrencyLimiter;
    /// # async fn example() {
    /// let limiter = ConcurrencyLimiter::new(5);
    ///
    /// let _permit = limiter.acquire().await;
    /// // Execute tool - permit is held
    /// // Permit is automatically released when _permit goes out of scope
    /// # }
    /// ```
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore
            .acquire()
            .await
            .expect("Semaphore should not be closed")
    }

    /// Gets the number of available permits
    ///
    /// # Returns
    ///
    /// The number of permits currently available
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_timeout_within_limit() {
        let wrapper = TimeoutWrapper::new(Duration::from_secs(1));

        let result = wrapper
            .execute(async {
                sleep(Duration::from_millis(100)).await;
                Ok::<_, ArsenalError>("success".to_string())
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_timeout_exceeds_limit() {
        let wrapper = TimeoutWrapper::new(Duration::from_millis(100));

        let result = wrapper
            .execute(async {
                sleep(Duration::from_secs(1)).await;
                Ok::<_, ArsenalError>("success".to_string())
            })
            .await;

        assert!(result.is_err());
        match result {
            Err(ArsenalError::Timeout(seconds)) => {
                assert_eq!(seconds, 0); // 100ms rounds to 0 seconds
            }
            _ => panic!("Expected Timeout error"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_timing() {
        let wrapper = TimeoutWrapper::new(Duration::from_secs(1));

        let (result, elapsed_ms) = wrapper
            .execute_with_timing(async {
                sleep(Duration::from_millis(100)).await;
                Ok::<_, ArsenalError>("success".to_string())
            })
            .await;

        assert!(result.is_ok());
        assert!(elapsed_ms >= 100, "Elapsed time: {}ms", elapsed_ms);
        assert!(elapsed_ms < 500, "Elapsed time should be less than 500ms");
    }

    #[tokio::test]
    async fn test_concurrency_limit_enforced() {
        let limiter = ConcurrencyLimiter::new(2);

        // Acquire 2 permits
        let permit1 = limiter.acquire().await;
        let permit2 = limiter.acquire().await;

        assert_eq!(limiter.available_permits(), 0);

        // Try to check if we can acquire (should be 0 available)
        let available = limiter.available_permits();
        assert_eq!(available, 0);

        // Release one permit
        drop(permit1);
        assert_eq!(limiter.available_permits(), 1);

        // Release second permit
        drop(permit2);
        assert_eq!(limiter.available_permits(), 2);
    }

    #[tokio::test]
    async fn test_concurrency_queuing() {
        let limiter = ConcurrencyLimiter::new(1);

        // Acquire the single permit
        let permit = limiter.acquire().await;

        // Spawn a task that tries to acquire a permit
        let limiter_clone = limiter.clone();
        let handle = tokio::spawn(async move {
            let _p = limiter_clone.acquire().await;
            "acquired".to_string()
        });

        // Give the spawned task time to queue
        sleep(Duration::from_millis(50)).await;

        // Spawned task should still be waiting
        assert!(!handle.is_finished());

        // Release the permit
        drop(permit);

        // Now the spawned task should complete
        let result = handle.await.unwrap();
        assert_eq!(result, "acquired");
    }

    #[tokio::test]
    async fn test_timeout_wrapper_clone() {
        let wrapper1 = TimeoutWrapper::new(Duration::from_secs(30));
        let wrapper2 = wrapper1.clone();

        assert_eq!(wrapper1.duration, wrapper2.duration);
    }

    #[tokio::test]
    async fn test_concurrency_limiter_clone() {
        let limiter1 = ConcurrencyLimiter::new(5);
        let limiter2 = limiter1.clone();

        let _permit1 = limiter1.acquire().await;
        // Both limiters share the same semaphore
        assert_eq!(limiter2.available_permits(), 4);
    }
}
