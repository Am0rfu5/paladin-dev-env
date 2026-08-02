//! Mock Paladin Port for testing Battalion patterns
//!
//! Provides a mock implementation of PaladinPort that wraps MockLlmAdapter
//! to enable testing of Formation, Phalanx, and other Battalion patterns.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use paladin::application::services::paladin::error::PaladinError;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::paladin::Paladin;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin_ports::output::llm_port::LlmPort;
use paladin_ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStream, PaladinStreamChunk, StopReason,
};

use super::MockLlmAdapter;

/// Mock implementation of PaladinPort for testing
///
/// This mock wraps a PaladinExecutionService with MockLlmAdapter to enable
/// testing of Battalion patterns (Formation, Phalanx, etc.) without real LLM calls.
pub struct MockPaladinPort {
    execution_service: Arc<PaladinExecutionService>,
}

impl MockPaladinPort {
    /// Create a new MockPaladinPort with the given MockLlmAdapter
    pub fn new(mock_llm: Arc<MockLlmAdapter>, circuit_breaker: Arc<CircuitBreaker>) -> Self {
        let execution_service = Arc::new(PaladinExecutionService::new(
            mock_llm as Arc<dyn LlmPort>,
            circuit_breaker,
            None, // No garrison
            None, // No arsenal
        ));

        Self { execution_service }
    }
}

#[async_trait]
impl PaladinPort for MockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        self.execution_service.execute(paladin, input).await
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<PaladinStreamChunk, PaladinError>>, PaladinError>
    {
        // For testing, we don't need streaming support
        Err(PaladinError::ExecutionError(
            "Streaming not supported in MockPaladinPort".to_string(),
        ))
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        // Always validate successfully for testing
        Ok(())
    }
}

/// Configurable failing [`PaladinPort`] mock for exercising Commander error paths.
///
/// Unlike [`MockPaladinPort`], which always succeeds, `FaultyPaladinPort` supports four
/// independently-configurable fault modes — fail always, fail a named Paladin, fail until
/// the Nth attempt (an invocation counter), and a controllable per-execution delay — so
/// tests can exercise `ErrorStrategy::FailFast`, `ErrorStrategy::ContinueOnError` and
/// `ErrorStrategy::RetryThenContinue` against a Commander without a real LLM.
///
/// This is the mock D-09/D-10 asked for: a single shared home for Commander error-path
/// testing, built by combining the retry-counter idiom from
/// [`crate::helpers::mock_llm_adapter`]-style interior mutability with the
/// `fail_until_attempt` pattern in `FormationExecutionService`'s in-crate test mock and the
/// `fail_paladin_names` + `delay_ms` pattern in `PhalanxExecutionService`'s.
///
/// All interior state uses `Arc<Mutex<_>>`, never `Rc`/`RefCell`, so the type is
/// `Send + Sync` and safe to share across concurrent Paladin executions (Phalanx, Campaign).
#[derive(Clone)]
pub struct FaultyPaladinPort {
    /// Total number of `execute` calls made across every Paladin, in invocation order.
    call_count: Arc<Mutex<usize>>,
    /// One entry per `execute` call, naming the Paladin and the input it received.
    execution_log: Arc<Mutex<Vec<String>>>,
    /// When `true`, every `execute` call fails regardless of which Paladin ran.
    fail_always: bool,
    /// Paladin names that always fail when executed (checked by `paladin.node.name`).
    fail_paladin_names: Arc<Mutex<Vec<String>>>,
    /// When `Some(n)`, `execute` fails while the invocation counter is at or below `n`,
    /// then succeeds on every call after that — the retry-count pattern.
    fail_until_attempt: Option<usize>,
    /// Milliseconds `execute` sleeps before deciding success or failure.
    delay_ms: u64,
}

impl FaultyPaladinPort {
    /// Creates a `FaultyPaladinPort` with no configured failures: every `execute` call
    /// succeeds and is recorded in the execution log.
    pub fn new() -> Self {
        Self {
            call_count: Arc::new(Mutex::new(0)),
            execution_log: Arc::new(Mutex::new(Vec::new())),
            fail_always: false,
            fail_paladin_names: Arc::new(Mutex::new(Vec::new())),
            fail_until_attempt: None,
            delay_ms: 0,
        }
    }

    /// Makes every `execute` call fail, regardless of which Paladin ran.
    pub fn fail_always(mut self) -> Self {
        self.fail_always = true;
        self
    }

    /// Adds a Paladin name that should fail whenever it is executed. Chainable — call
    /// multiple times to fail more than one Paladin.
    pub fn fail_paladin(self, name: impl Into<String>) -> Self {
        self.fail_paladin_names.lock().unwrap().push(name.into());
        self
    }

    /// Fails every `execute` call while the invocation counter is at or below `n`, then
    /// succeeds from the `n + 1`th call onward. The counter is shared across every
    /// Paladin executed through this port, not scoped per Paladin.
    pub fn fail_until_attempt(mut self, n: usize) -> Self {
        self.fail_until_attempt = Some(n);
        self
    }

    /// Sets a delay, in milliseconds, that `execute` sleeps before deciding success or
    /// failure — used to prove timeout enforcement stops sibling agents.
    pub fn with_delay_ms(mut self, ms: u64) -> Self {
        self.delay_ms = ms;
        self
    }

    /// Returns the exact number of `execute` calls made so far, across every Paladin.
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Returns a clone of the execution log, in invocation order — one entry per
    /// `execute` call, naming the Paladin executed.
    pub fn execution_log(&self) -> Vec<String> {
        self.execution_log.lock().unwrap().clone()
    }
}

impl Default for FaultyPaladinPort {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PaladinPort for FaultyPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        // Record the invocation before any await, in invocation order.
        {
            let mut log = self.execution_log.lock().unwrap();
            log.push(format!("{}: {}", paladin.node.name, input));
        }

        // Increment and read the shared counter, dropping the guard before the sleep.
        let current_attempt = {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            *count
        };

        if self.delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
        }

        // Precedence: fail_until_attempt, then fail_always, then fail_paladin_names.
        if let Some(threshold) = self.fail_until_attempt
            && current_attempt <= threshold
        {
            return Err(PaladinError::ExecutionError(format!(
                "FaultyPaladinPort: {} failed on attempt {} (fail_until_attempt={})",
                paladin.node.name, current_attempt, threshold
            )));
        }

        if self.fail_always {
            return Err(PaladinError::ExecutionError(format!(
                "FaultyPaladinPort: {} failed on attempt {} (fail_always)",
                paladin.node.name, current_attempt
            )));
        }

        if self
            .fail_paladin_names
            .lock()
            .unwrap()
            .contains(&paladin.node.name)
        {
            return Err(PaladinError::ExecutionError(format!(
                "FaultyPaladinPort: {} failed on attempt {} (fail_paladin)",
                paladin.node.name, current_attempt
            )));
        }

        Ok(PaladinResult {
            output: format!(
                "FaultyPaladinPort: {} processed {}",
                paladin.node.name, input
            ),
            token_count: 10,
            execution_time_ms: self.delay_ms,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            ..Default::default()
        })
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<PaladinStream, PaladinError> {
        // Streaming is not supported in this mock, matching every existing mock in this
        // workspace (MockPaladinPort above, and the in-crate mocks in formation_service.rs
        // and phalanx_service.rs).
        Err(PaladinError::ExecutionError(
            "Streaming not supported in FaultyPaladinPort".to_string(),
        ))
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin::core::base::entity::node::Node;
    use paladin::core::platform::container::paladin::{MaxLoops, PaladinData, PaladinStatus};

    fn assert_send_sync<T: Send + Sync>() {}

    fn make_paladin(name: &str) -> Paladin {
        let data = PaladinData {
            system_prompt: format!("{} prompt", name),
            name: name.to_string(),
            user_name: "TestUser".to_string(),
            model: "test-model".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(1),
            stop_words: vec![],
            status: PaladinStatus::Idle,
            vision_enabled: false,
            ..Default::default()
        };
        Node::new(data, Some(name.to_string()))
    }

    #[test]
    fn faulty_paladin_port_is_send_and_sync() {
        assert_send_sync::<FaultyPaladinPort>();
    }

    #[tokio::test]
    async fn faulty_paladin_port_new_succeeds_and_logs_execution() {
        let port = FaultyPaladinPort::new();
        let paladin = make_paladin("Paladin-1");

        let result = port.execute(&paladin, "hello").await;

        assert!(result.is_ok(), "New FaultyPaladinPort should succeed");
        assert_eq!(port.call_count(), 1);
        let log = port.execution_log();
        assert_eq!(log.len(), 1);
        assert!(log[0].contains("Paladin-1"));
        assert!(log[0].contains("hello"));
    }

    #[tokio::test]
    async fn faulty_paladin_port_fail_always_fails_every_execution() {
        let port = FaultyPaladinPort::new().fail_always();
        let paladin = make_paladin("Paladin-1");

        assert!(port.execute(&paladin, "first").await.is_err());
        assert!(port.execute(&paladin, "second").await.is_err());
        assert_eq!(port.call_count(), 2);
    }

    #[tokio::test]
    async fn faulty_paladin_port_fail_paladin_fails_only_named_paladin() {
        let port = FaultyPaladinPort::new().fail_paladin("Paladin-2");
        let paladin1 = make_paladin("Paladin-1");
        let paladin2 = make_paladin("Paladin-2");

        assert!(
            port.execute(&paladin1, "x").await.is_ok(),
            "Paladin-1 was not configured to fail"
        );
        assert!(
            port.execute(&paladin2, "x").await.is_err(),
            "Paladin-2 was configured to fail"
        );
    }

    #[tokio::test]
    async fn faulty_paladin_port_fail_until_attempt_then_succeeds() {
        let port = FaultyPaladinPort::new().fail_until_attempt(2);
        let paladin = make_paladin("Paladin-1");

        assert!(
            port.execute(&paladin, "x").await.is_err(),
            "attempt 1 fails"
        );
        assert!(
            port.execute(&paladin, "x").await.is_err(),
            "attempt 2 fails"
        );
        assert!(
            port.execute(&paladin, "x").await.is_ok(),
            "attempt 3 succeeds"
        );
        assert_eq!(
            port.call_count(),
            3,
            "call_count reads the exact number of attempts, not a range"
        );
    }

    #[tokio::test]
    async fn faulty_paladin_port_with_delay_ms_sleeps_before_deciding() {
        let port = FaultyPaladinPort::new().with_delay_ms(20);
        let paladin = make_paladin("Paladin-1");

        let start = std::time::Instant::now();
        let _ = port.execute(&paladin, "x").await;

        assert!(
            start.elapsed() >= std::time::Duration::from_millis(20),
            "execute should sleep for the configured delay before deciding"
        );
    }

    #[tokio::test]
    async fn faulty_paladin_port_execution_log_records_invocation_order() {
        let port = FaultyPaladinPort::new();
        let paladin1 = make_paladin("Paladin-1");
        let paladin2 = make_paladin("Paladin-2");

        port.execute(&paladin1, "first").await.unwrap();
        port.execute(&paladin2, "second").await.unwrap();

        let log = port.execution_log();
        assert_eq!(log.len(), 2);
        assert!(log[0].contains("Paladin-1"), "first entry: {:?}", log);
        assert!(log[1].contains("Paladin-2"), "second entry: {:?}", log);
    }
}
