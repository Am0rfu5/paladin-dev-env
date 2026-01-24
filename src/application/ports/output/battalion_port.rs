//! Battalion Port Abstraction
//!
//! This module defines the port (interface) for Battalion execution following
//! the hexagonal architecture pattern. Implementations of this port handle
//! the actual orchestration logic while the domain remains independent.

use async_trait::async_trait;
use uuid::Uuid;

use crate::core::platform::container::battalion::{
    BattalionError, BattalionResult, BattalionStatus,
};

/// Port abstraction for Battalion execution
///
/// This trait defines the interface that any Battalion execution implementation
/// must satisfy. It follows the hexagonal architecture pattern, allowing the
/// core domain to remain independent of orchestration details.
///
/// # Example Implementation
///
/// ```ignore
/// use async_trait::async_trait;
/// use paladin::application::ports::output::battalion_port::BattalionPort;
/// use paladin::core::platform::container::battalion::{BattalionResult, BattalionStatus, BattalionError};
/// use uuid::Uuid;
///
/// struct MyBattalionExecutor;
///
/// #[async_trait]
/// impl BattalionPort for MyBattalionExecutor {
///     async fn execute(&self, battalion_id: Uuid) -> Result<BattalionResult, BattalionError> {
///         // Implementation here
///         unimplemented!()
///     }
///
///     async fn status(&self, battalion_id: Uuid) -> Result<BattalionStatus, BattalionError> {
///         Ok(BattalionStatus::Running)
///     }
///
///     async fn cancel(&self, battalion_id: Uuid) -> Result<(), BattalionError> {
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait BattalionPort: Send + Sync {
    /// Execute a Battalion and return the final result
    ///
    /// This method orchestrates the execution of all Paladins in the Battalion
    /// according to the configured pattern (Formation, Phalanx, Campaign, or
    /// Chain of Command).
    ///
    /// # Arguments
    ///
    /// * `battalion_id` - Unique identifier for this Battalion execution
    ///
    /// # Returns
    ///
    /// * `Ok(BattalionResult)` - The final aggregated result on success
    /// * `Err(BattalionError)` - If execution fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = battalion_port.execute(battalion_id).await?;
    /// println!("Battalion completed: {}", result.final_output);
    /// ```
    async fn execute(&self, battalion_id: Uuid) -> Result<BattalionResult, BattalionError>;

    /// Get the current status of a Battalion execution
    ///
    /// This method returns the current execution status without blocking.
    /// Useful for monitoring long-running Battalion operations.
    ///
    /// # Arguments
    ///
    /// * `battalion_id` - Unique identifier for the Battalion execution
    ///
    /// # Returns
    ///
    /// * `Ok(BattalionStatus)` - Current status
    /// * `Err(BattalionError)` - If Battalion not found or error accessing status
    ///
    /// # Example
    ///
    /// ```ignore
    /// let status = battalion_port.status(battalion_id).await?;
    /// match status {
    ///     BattalionStatus::Running => println!("Still executing..."),
    ///     BattalionStatus::Completed => println!("Finished!"),
    ///     _ => println!("Status: {:?}", status),
    /// }
    /// ```
    async fn status(&self, battalion_id: Uuid) -> Result<BattalionStatus, BattalionError>;

    /// Cancel a running Battalion execution
    ///
    /// This method attempts to gracefully cancel a Battalion that is currently
    /// executing. Running Paladins will be signaled to stop, and the Battalion
    /// status will be set to Cancelled.
    ///
    /// # Arguments
    ///
    /// * `battalion_id` - Unique identifier for the Battalion execution
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If cancellation was successful
    /// * `Err(BattalionError)` - If Battalion not found or already completed
    ///
    /// # Example
    ///
    /// ```ignore
    /// battalion_port.cancel(battalion_id).await?;
    /// println!("Battalion cancelled");
    /// ```
    async fn cancel(&self, battalion_id: Uuid) -> Result<(), BattalionError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::battalion::BattalionStatus;

    // Mock implementation for testing
    struct MockBattalionPort {
        should_fail: bool,
    }

    #[async_trait]
    impl BattalionPort for MockBattalionPort {
        async fn execute(&self, _battalion_id: Uuid) -> Result<BattalionResult, BattalionError> {
            if self.should_fail {
                Err(BattalionError::PaladinError("Mock failure".to_string()))
            } else {
                // This will fail until BattalionResult::new is available
                unimplemented!("BattalionResult construction not yet implemented")
            }
        }

        async fn status(&self, _battalion_id: Uuid) -> Result<BattalionStatus, BattalionError> {
            if self.should_fail {
                Err(BattalionError::ValidationError(
                    "Battalion not found".to_string(),
                ))
            } else {
                Ok(BattalionStatus::Running)
            }
        }

        async fn cancel(&self, _battalion_id: Uuid) -> Result<(), BattalionError> {
            if self.should_fail {
                Err(BattalionError::ValidationError(
                    "Battalion not found".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_battalion_port_status_success() {
        let port = MockBattalionPort { should_fail: false };
        let battalion_id = Uuid::new_v4();

        let result = port.status(battalion_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BattalionStatus::Running);
    }

    #[tokio::test]
    async fn test_battalion_port_status_failure() {
        let port = MockBattalionPort { should_fail: true };
        let battalion_id = Uuid::new_v4();

        let result = port.status(battalion_id).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::ValidationError(msg) => {
                assert_eq!(msg, "Battalion not found");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn test_battalion_port_cancel_success() {
        let port = MockBattalionPort { should_fail: false };
        let battalion_id = Uuid::new_v4();

        let result = port.cancel(battalion_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_battalion_port_cancel_failure() {
        let port = MockBattalionPort { should_fail: true };
        let battalion_id = Uuid::new_v4();

        let result = port.cancel(battalion_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_battalion_port_execute_failure() {
        let port = MockBattalionPort { should_fail: true };
        let battalion_id = Uuid::new_v4();

        let result = port.execute(battalion_id).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            BattalionError::PaladinError(msg) => {
                assert_eq!(msg, "Mock failure");
            }
            _ => panic!("Expected PaladinError"),
        }
    }

    #[test]
    fn test_battalion_port_is_send_sync() {
        // Verify trait is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn BattalionPort>>();
    }
}
