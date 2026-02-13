//! Mock Paladin Port for testing Battalion patterns
//!
//! Provides a mock implementation of PaladinPort that wraps MockLlmAdapter
//! to enable testing of Formation, Phalanx, and other Battalion patterns.

use async_trait::async_trait;
use std::sync::Arc;

use paladin::application::ports::output::llm_port::LlmPort;
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStreamChunk,
};
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::paladin::Paladin;

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
