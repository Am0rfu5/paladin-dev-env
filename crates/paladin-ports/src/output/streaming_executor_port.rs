//! Streaming Executor Port
//!
//! Defines the abstraction for executing a Paladin agent with **streamed** output.
//! It is the streaming counterpart to
//! [`PaladinExecutorPort`](crate::output::paladin_executor_port::PaladinExecutorPort)
//! (which is buffered) and is kept as a separate, focused trait so that buffered-only
//! callers and registries are unaffected — an executor may implement one or both.
//!
//! Implementors produce a [`PaladinStream`] (an `mpsc` receiver of
//! [`PaladinStreamChunk`](crate::output::paladin_port::PaladinStreamChunk)s), forwarding
//! incremental output as it is generated and a final chunk when execution completes.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────┐     ┌──────────────────────────────┐
//! │  SSE HTTP handler   │────▶│   StreamingExecutorPort      │
//! │ (paladin-web)       │     │   (trait / abstraction)      │
//! └─────────────────────┘     └──────────────┬───────────────┘
//!                                            │ implements
//!                                            ▼
//!                              ┌──────────────────────────────┐
//!                              │  PaladinExecutionService      │
//!                              │  (drives LlmPort::            │
//!                              │   generate_stream)            │
//!                              └──────────────────────────────┘
//! ```

use async_trait::async_trait;

use crate::output::paladin_port::PaladinStream;
use paladin_core::platform::container::paladin::Paladin;
use paladin_core::platform::container::paladin_error::PaladinError;

/// Port trait for executing a Paladin agent with streamed output.
///
/// Separate from [`PaladinExecutorPort`](crate::output::paladin_executor_port::PaladinExecutorPort)
/// so the buffered execution path and any registry that stores buffered executors stay
/// unchanged; streaming is an *optional* capability layered alongside it.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` to be shared across async tasks.
///
/// # Example
///
/// ```rust,no_run
/// use paladin_ports::output::streaming_executor_port::StreamingExecutorPort;
/// use paladin_core::platform::container::paladin::Paladin;
/// use paladin_core::platform::container::paladin_error::PaladinError;
///
/// async fn first_chunk(
///     executor: &dyn StreamingExecutorPort,
///     agent: &Paladin,
///     input: &str,
/// ) -> Result<Option<String>, PaladinError> {
///     let mut stream = executor.execute_stream(agent, input).await?;
///     match stream.recv().await {
///         Some(Ok(chunk)) => Ok(Some(chunk.text)),
///         Some(Err(e)) => Err(e),
///         None => Ok(None),
///     }
/// }
/// ```
#[async_trait]
pub trait StreamingExecutorPort: Send + Sync {
    /// Execute a Paladin with the given input, streaming output chunks.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin agent to execute
    /// * `input` - The input/task to process
    ///
    /// # Returns
    ///
    /// * `Ok(PaladinStream)` - A receiver yielding `Ok(PaladinStreamChunk)` per chunk
    ///   (the last with `is_final = true`), or `Err(PaladinError)` if a chunk fails.
    /// * `Err(PaladinError)` - If the stream could not be started (e.g. the provider
    ///   does not support streaming).
    async fn execute_stream(
        &self,
        paladin: &Paladin,
        input: &str,
    ) -> Result<PaladinStream, PaladinError>;
}
