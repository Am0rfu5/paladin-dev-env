//! Paladin Executor Port
//!
//! Defines the abstraction for executing a Paladin agent. This port breaks the
//! circular dependency between `HandoffService` and `PaladinExecutionService`:
//!
//! - `HandoffService` depends on `Arc<dyn PaladinExecutorPort>` to execute specialists
//! - `PaladinExecutionService` implements `PaladinExecutorPort`
//!
//! This follows the Dependency Inversion Principle: both high-level (HandoffService)
//! and low-level (PaladinExecutionService) modules depend on the abstraction.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────┐     ┌──────────────────────────┐
//! │   HandoffService    │────▶│  PaladinExecutorPort     │
//! │ (uses trait to      │     │  (trait / abstraction)   │
//! │  execute specialist)│     └──────────┬───────────────┘
//! └─────────────────────┘                │ implements
//!                                        ▼
//!                              ┌──────────────────────────┐
//!                              │ PaladinExecutionService   │
//!                              │ (concrete implementation) │
//!                              └──────────────────────────┘
//! ```

use async_trait::async_trait;

use crate::output::paladin_port::PaladinResult;
use paladin_core::platform::container::paladin::Paladin;
use paladin_core::platform::container::paladin_error::PaladinError;

/// Port trait for executing a Paladin agent
///
/// This abstraction allows services like `HandoffService` to delegate execution
/// to a Paladin without directly depending on `PaladinExecutionService`, thus
/// avoiding circular dependencies in the dependency graph.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` to allow sharing across async tasks.
///
/// # Example
///
/// ```rust,no_run
/// use paladin::application::ports::output::paladin_executor_port::PaladinExecutorPort;
/// use paladin::application::ports::output::paladin_port::PaladinResult;
/// use paladin::core::platform::container::paladin::Paladin;
/// use std::sync::Arc;
///
/// async fn delegate_to_specialist(
///     executor: &dyn PaladinExecutorPort,
///     specialist: &Paladin,
///     task: &str,
/// ) -> Result<PaladinResult, paladin::application::use_cases::paladin::error::PaladinError> {
///     executor.execute(specialist, task).await
/// }
/// ```
#[async_trait]
pub trait PaladinExecutorPort: Send + Sync {
    /// Execute a Paladin with the given input
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin agent to execute
    /// * `input` - The input/task to process
    ///
    /// # Returns
    ///
    /// * `Ok(PaladinResult)` - The execution result including output, tokens, etc.
    /// * `Err(PaladinError)` - If execution fails
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError>;
}
