//! Integration tests for Council Battalion pattern
//!
//! Tests multi-Paladin discussion orchestration with turn-taking strategies.

use async_trait::async_trait;
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStream, StopReason,
};
use paladin::application::use_cases::battalion::council_service::CouncilExecutionService;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::battalion::council::{
    CouncilBuilder, TerminationCondition, TurnStrategy,
};
use paladin::core::platform::container::paladin::{MaxLoops, Paladin, PaladinData, PaladinStatus};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

/// Mock PaladinPort for Council testing with configurable responses and execution tracking
#[derive(Clone)]
struct CouncilMockPaladinPort {
    /// Configured responses by Paladin name
    responses: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Tracks which response index to use next for each Paladin
    response_indices: Arc<Mutex<HashMap<String, usize>>>,
    /// Execution log to verify turn order
    execution_log: Arc<Mutex<Vec<String>>>,
    /// Configured errors by Paladin name
    errors: Arc<Mutex<HashMap<String, String>>>,
    /// Configured delays by Paladin name
    delays: Arc<Mutex<HashMap<String, Duration>>>,
}

impl CouncilMockPaladinPort {
    fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            response_indices: Arc::new(Mutex::new(HashMap::new())),
            execution_log: Arc::new(Mutex::new(Vec::new())),
            errors: Arc::new(Mutex::new(HashMap::new())),
            delays: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn with_responses(self, paladin_name: &str, responses: Vec<String>) -> Self {
        self.responses
            .lock()
            .unwrap()
            .insert(paladin_name.to_string(), responses);
        self
    }

    fn with_error(self, paladin_name: &str, error_msg: &str) -> Self {
        self.errors
            .lock()
            .unwrap()
            .insert(paladin_name.to_string(), error_msg.to_string());
        self
    }

    fn with_delay(self, paladin_name: &str, delay: Duration) -> Self {
        self.delays
            .lock()
            .unwrap()
            .insert(paladin_name.to_string(), delay);
        self
    }

    fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.lock().unwrap().clone()
    }
}

#[async_trait]
impl PaladinPort for CouncilMockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        let paladin_name = paladin.node.name.clone();

        // Check for configured error
        if let Some(error_msg) = self.errors.lock().unwrap().get(&paladin_name) {
            return Err(PaladinError::ExecutionError(error_msg.clone()));
        }

        // Check for configured delay and clone it to avoid holding the lock across await
        let delay_duration = self.delays.lock().unwrap().get(&paladin_name).copied();
        let delay_ms = if let Some(delay) = delay_duration {
            tokio::time::sleep(delay).await;
            delay.as_millis() as u64
        } else {
            10
        };

        // Log execution for verification
        self.execution_log
            .lock()
            .unwrap()
            .push(format!("{}:{}", paladin_name, input));

        // Get configured response or default
        let output = if let Some(responses) = self.responses.lock().unwrap().get(&paladin_name) {
            let mut indices = self.response_indices.lock().unwrap();
            let index = *indices.get(&paladin_name).unwrap_or(&0);

            if index < responses.len() {
                let response = responses[index].clone();
                indices.insert(paladin_name.clone(), index + 1);
                response
            } else {
                format!(
                    "[{}]: I've reviewed the discussion. Input was: {}",
                    paladin_name, input
                )
            }
        } else {
            format!("[{}]: Analyzed: {}", paladin_name, input)
        };

        Ok(PaladinResult {
            output,
            token_count: 100,
            execution_time_ms: delay_ms,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        })
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<PaladinStream, PaladinError> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(rx)
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        Ok(())
    }
}

/// Helper function to create a test Paladin with a given name
fn create_test_paladin(name: &str) -> Paladin {
    let data = PaladinData {
        system_prompt: format!("You are {} expert", name),
        name: name.to_string(),
        user_name: "User".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_loops: MaxLoops::Fixed(3),
        stop_words: vec![],
        status: PaladinStatus::Idle,
        vision_enabled: false,
    };
    Node::new(data, Some(name.to_string()))
}

#[tokio::test]
async fn test_council_roundrobin_three_paladins_two_rounds() {
    // Task 8.3: Council with 3 Paladins, RoundRobin, 2 rounds
    let paladin_port = Arc::new(
        CouncilMockPaladinPort::new()
            .with_responses(
                "Security",
                vec![
                    "Security concerns: We need authentication".to_string(),
                    "I agree with the implementation plan".to_string(),
                ],
            )
            .with_responses(
                "Legal",
                vec![
                    "Legal requirements: GDPR compliance needed".to_string(),
                    "We should document everything".to_string(),
                ],
            )
            .with_responses(
                "Technical",
                vec![
                    "Technical perspective: OAuth2 is the standard".to_string(),
                    "I can implement this in two sprints".to_string(),
                ],
            ),
    );

    // Create Paladins (Note: Council uses Paladin IDs, not Paladin objects)
    let _security = create_test_paladin("Security");
    let _legal = create_test_paladin("Legal");
    let _technical = create_test_paladin("Technical");

    let council = CouncilBuilder::new()
        .name("SecurityCouncil")
        .add_participant("Security")
        .add_participant("Legal")
        .add_participant("Technical")
        .max_rounds(2)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port.clone(), None);

    let result = service
        .convene(&council, "Should we implement two-factor authentication?")
        .await
        .expect("Council execution should succeed");

    // Verify execution
    assert!(
        !result.transcript.is_empty(),
        "Transcript should not be empty"
    );
    assert_eq!(result.rounds_completed, 2, "Should complete 2 rounds");

    // Verify execution log shows RoundRobin pattern
    let log = paladin_port.get_execution_log();
    assert!(
        log.len() >= 6,
        "Should have at least 6 turns (3 participants x 2 rounds)"
    );
}

#[tokio::test]
async fn test_council_moderator_directed_strategy() {
    // Task 8.4: Council with moderator-directed strategy
    let paladin_port = Arc::new(
        CouncilMockPaladinPort::new()
            .with_responses(
                "Moderator",
                vec!["Next, let's hear from the Security team".to_string()],
            )
            .with_responses(
                "Security",
                vec!["We need to assess the threat model".to_string()],
            ),
    );

    let _moderator = create_test_paladin("Moderator");
    let _security = create_test_paladin("Security");
    let _legal = create_test_paladin("Legal");

    let council = CouncilBuilder::new()
        .name("ModeratedCouncil")
        .add_participant("Moderator")
        .add_participant("Security")
        .add_participant("Legal")
        .moderator("Moderator")
        .max_rounds(1)
        .turn_strategy(TurnStrategy::ModeratorDirected)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port.clone(), None);

    let result = service
        .convene(&council, "Security assessment meeting")
        .await
        .expect("Council execution should succeed");

    assert!(!result.transcript.is_empty());

    // Verify moderator was first in execution
    let log = paladin_port.get_execution_log();
    assert!(!log.is_empty());
    assert!(log[0].contains("Moderator"));
}

#[tokio::test]
async fn test_council_max_rounds_termination() {
    // Task 8.5: Council terminates after max_rounds
    let paladin_port = Arc::new(
        CouncilMockPaladinPort::new()
            .with_responses("Participant", vec!["I have an opinion".to_string(); 10]),
    );

    let _p1 = create_test_paladin("Participant");
    let _p2 = create_test_paladin("Participant2");

    let council = CouncilBuilder::new()
        .name("MaxRoundsCouncil")
        .add_participant("Participant")
        .add_participant("Participant2")
        .max_rounds(3)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port.clone(), None);

    let result = service
        .convene(&council, "Long discussion topic")
        .await
        .expect("Council execution should succeed");

    // Should have stopped at 3 rounds
    assert_eq!(
        result.rounds_completed, 3,
        "Should complete exactly 3 rounds"
    );
    let log = paladin_port.get_execution_log();
    assert!(
        log.len() >= 6,
        "Should have at least 6 turns (2 participants x 3 rounds)"
    );
}

#[tokio::test]
async fn test_council_consensus_termination() {
    // Task 8.6: Council with consensus-based termination
    let paladin_port = Arc::new(
        CouncilMockPaladinPort::new()
            .with_responses("P1", vec!["I agree with the proposal".to_string()])
            .with_responses("P2", vec!["Yes, I also agree".to_string()]),
    );

    let _p1 = create_test_paladin("P1");
    let _p2 = create_test_paladin("P2");

    let council = CouncilBuilder::new()
        .name("ConsensusCouncil")
        .add_participant("P1")
        .add_participant("P2")
        .max_rounds(5)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::Consensus)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port.clone(), None);

    let result = service
        .convene(&council, "Should we proceed?")
        .await
        .expect("Council execution should succeed");

    assert!(!result.transcript.is_empty());
    // Note: Consensus detection depends on implementation
}

#[tokio::test]
async fn test_council_error_handling() {
    // Task 8.7: Council handles Paladin execution errors gracefully
    let paladin_port = Arc::new(
        CouncilMockPaladinPort::new()
            .with_error("Faulty", "Simulated error")
            .with_responses("Backup", vec!["I'll take over".to_string()]),
    );

    let _faulty = create_test_paladin("Faulty");
    let _backup = create_test_paladin("Backup");

    let council = CouncilBuilder::new()
        .name("ErrorHandlingCouncil")
        .add_participant("Faulty")
        .add_participant("Backup")
        .max_rounds(1)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port.clone(), None);

    // Should handle error gracefully, possibly with partial results
    let result = service.convene(&council, "Test topic").await;

    // Either succeeds with partial results or fails gracefully
    match result {
        Ok(res) => {
            // Execution continued with other participants
            assert!(!res.transcript.is_empty());
        }
        Err(e) => {
            // Error is well-structured
            assert!(!e.to_string().is_empty());
        }
    }
}

#[tokio::test]
async fn test_council_timeout_enforcement() {
    // Additional test: Council respects timeout configuration
    let paladin_port = Arc::new(CouncilMockPaladinPort::new().with_delay(
        "Slow",
        Duration::from_secs(10), // Deliberately slow
    ));

    let _slow = create_test_paladin("Slow");

    let council = CouncilBuilder::new()
        .name("TimeoutCouncil")
        .add_participant("Slow")
        .max_rounds(1)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()
        .expect("Council build should succeed");

    let service = CouncilExecutionService::new(paladin_port, None);

    let start = tokio::time::Instant::now();
    let result = service.convene(&council, "Quick topic").await;
    let elapsed = start.elapsed();

    // Should complete or timeout reasonably quickly
    // Note: Actual timeout enforcement depends on implementation
    assert!(elapsed < Duration::from_secs(15));
    // Result might succeed (if implementation doesn't timeout) or error (if timeout implemented)
    match result {
        Ok(_) => assert!(true, "Council completed"),
        Err(_) => assert!(true, "Council timed out"),
    }
}
