//! Council Execution Service
//!
//! Provides orchestration logic for executing multi-Paladin discussions in Council pattern.

use log::{debug, info, warn};
use std::sync::Arc;
use tokio::time::{Duration, timeout};

use crate::application::ports::output::garrison_port::GarrisonPort;
use crate::application::ports::output::paladin_port::PaladinPort;
use crate::application::use_cases::paladin::error::PaladinError;
use crate::core::platform::container::battalion::BattalionError;
use crate::core::platform::container::battalion::council::{
    Council, CouncilMessage, TerminationCondition, TurnStrategy,
};
use crate::core::platform::container::garrison::{ConversationRole, GarrisonEntry};

/// Result of a Council discussion
///
/// Contains the complete conversation transcript, final conclusion, and metadata
/// about the discussion execution.
#[derive(Debug, Clone)]
pub struct CouncilResult {
    /// Complete conversation transcript
    pub transcript: Vec<CouncilMessage>,

    /// Final conclusion or summary (if available)
    pub conclusion: Option<String>,

    /// Number of rounds completed
    pub rounds_completed: u32,

    /// Reason for termination
    pub termination_reason: TerminationCondition,
}

/// Service for executing Council patterns
///
/// Orchestrates turn-based discussions between multiple Paladins, managing
/// conversation flow, turn-taking logic, and termination conditions.
///
/// # Example
///
/// ```ignore
/// use paladin::application::use_cases::battalion::council_service::CouncilExecutionService;
/// use std::sync::Arc;
///
/// let service = CouncilExecutionService::new(paladin_port, Some(garrison_port));
/// let result = service.convene(&council, "Should we implement feature X?").await?;
///
/// for message in result.transcript {
///     println!("{}", message.format());
/// }
/// ```
pub struct CouncilExecutionService {
    /// Paladin execution port (reserved for future use)
    #[allow(dead_code)]
    paladin_port: Arc<dyn PaladinPort>,

    /// Optional Garrison for storing conversation history
    garrison_port: Option<Arc<dyn GarrisonPort>>,
}

impl CouncilExecutionService {
    /// Create a new CouncilExecutionService
    ///
    /// # Arguments
    ///
    /// * `paladin_port` - Port for executing individual Paladins
    /// * `garrison_port` - Optional port for storing conversation history
    ///
    /// # Example
    ///
    /// ```ignore
    /// let service = CouncilExecutionService::new(paladin_port, Some(garrison_port));
    /// ```
    pub fn new(
        paladin_port: Arc<dyn PaladinPort>,
        garrison_port: Option<Arc<dyn GarrisonPort>>,
    ) -> Self {
        info!("Creating CouncilExecutionService");
        Self {
            paladin_port,
            garrison_port,
        }
    }

    /// Convene a Council discussion on a topic
    ///
    /// Orchestrates a multi-round discussion between participant Paladins,
    /// managing turn-taking according to the configured strategy and monitoring
    /// for termination conditions.
    ///
    /// # Arguments
    ///
    /// * `council` - The Council configuration
    /// * `topic` - The discussion topic
    ///
    /// # Returns
    ///
    /// * `Ok(CouncilResult)` - Complete discussion result
    /// * `Err(BattalionError)` - If execution fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = service.convene(&council, "Discuss security implications").await?;
    /// println!("Discussion concluded after {} rounds", result.rounds_completed);
    /// ```
    pub async fn convene(
        &self,
        council: &Council,
        topic: &str,
    ) -> Result<CouncilResult, BattalionError> {
        info!(
            "Convening Council '{}' with {} participants on topic: {}",
            council.node.name,
            council.node.participant_ids.len(),
            topic
        );

        // Initialize conversation state
        let mut transcript: Vec<CouncilMessage> = Vec::new();
        let mut current_round = 1u32;
        let mut speaker_index = 0usize;

        // Main conversation loop
        loop {
            // Check termination conditions before each turn
            if self.should_terminate(
                &council.node.config.termination_condition,
                &transcript,
                current_round,
                &council.node.config.max_rounds,
            ) {
                info!(
                    "Council termination condition met: {:?}",
                    council.node.config.termination_condition
                );
                break;
            }

            // Determine next speaker
            let next_speaker_id = self.determine_next_speaker(
                &council.node.config.turn_strategy,
                &council.node.participant_ids,
                &council.node.moderator_id,
                &mut speaker_index,
                &transcript,
            )?;

            debug!(
                "Round {}: Next speaker is {}",
                current_round, next_speaker_id
            );

            // TODO: Get actual Paladin from registry/repository
            // For now, we'll need to receive Paladins as part of the Council or use a registry
            // This is a placeholder that will be addressed in integration

            // Build context with conversation history
            let context = if council.node.config.include_history {
                self.format_conversation_history(&transcript, topic)
            } else {
                topic.to_string()
            };

            // Execute speaker with timeout
            let timeout_duration = Duration::from_secs(300); // 5 minutes per speaker
            let speaker_output = match timeout(
                timeout_duration,
                self.execute_speaker(&next_speaker_id, &context),
            )
            .await
            {
                Ok(Ok(output)) => output,
                Ok(Err(e)) => {
                    warn!(
                        "Speaker {} failed in round {}: {}",
                        next_speaker_id, current_round, e
                    );
                    // Skip to next speaker on error
                    speaker_index += 1;
                    continue;
                }
                Err(_) => {
                    warn!(
                        "Speaker {} timed out in round {}",
                        next_speaker_id, current_round
                    );
                    // Skip to next speaker on timeout
                    speaker_index += 1;
                    continue;
                }
            };

            // Record the message
            let message =
                CouncilMessage::new(next_speaker_id.clone(), speaker_output, current_round);

            // Store in Garrison if available
            if let Some(garrison) = &self.garrison_port
                && let Err(e) = self
                    .store_in_garrison(garrison.as_ref(), &message, topic)
                    .await
            {
                warn!("Failed to store message in Garrison: {}", e);
                // Continue anyway - Garrison storage is non-critical
            }

            transcript.push(message);

            // Check if we should start a new round
            if speaker_index >= council.node.participant_ids.len() {
                current_round += 1;
                speaker_index = 0;
            }
        }

        // Extract conclusion
        let conclusion = self.extract_conclusion(&transcript, &council.node.moderator_id);

        let result = CouncilResult {
            transcript,
            conclusion,
            rounds_completed: current_round,
            termination_reason: council.node.config.termination_condition.clone(),
        };

        info!(
            "Council '{}' concluded after {} rounds",
            council.node.name, result.rounds_completed
        );

        Ok(result)
    }

    /// Determine the next speaker based on turn strategy
    ///
    /// # Arguments
    ///
    /// * `strategy` - The turn-taking strategy to use
    /// * `participants` - List of participant IDs
    /// * `moderator_id` - Optional moderator ID
    /// * `speaker_index` - Current speaker index (mutable for RoundRobin)
    /// * `transcript` - Current conversation transcript
    ///
    /// # Returns
    ///
    /// The ID of the next speaker, or an error if determination fails
    fn determine_next_speaker(
        &self,
        strategy: &TurnStrategy,
        participants: &[String],
        moderator_id: &Option<String>,
        speaker_index: &mut usize,
        transcript: &[CouncilMessage],
    ) -> Result<String, BattalionError> {
        match strategy {
            TurnStrategy::RoundRobin => {
                // Simple round-robin: cycle through participants
                let speaker_id = participants[*speaker_index % participants.len()].clone();
                *speaker_index += 1;
                Ok(speaker_id)
            }
            TurnStrategy::ModeratorDirected => {
                // Moderator selects next speaker by parsing their response
                if let Some(last_message) = transcript.last()
                    && let Some(mod_id) = moderator_id
                    && &last_message.speaker == mod_id
                    && let Some(next_speaker) =
                        self.parse_next_speaker(&last_message.content, participants)
                {
                    return Ok(next_speaker);
                }

                // Default: If moderator exists, they speak first or next
                if let Some(mod_id) = moderator_id {
                    Ok(mod_id.clone())
                } else {
                    Err(BattalionError::ValidationError(
                        "ModeratorDirected strategy requires a moderator".to_string(),
                    ))
                }
            }
            TurnStrategy::Random => {
                // Random selection from participants
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                participants.choose(&mut rng).cloned().ok_or_else(|| {
                    BattalionError::ValidationError("No participants available".to_string())
                })
            }
            TurnStrategy::VoluntaryWithTimeout { timeout_ms: _ } => {
                // For now, fallback to RoundRobin
                // Full implementation would require async signaling
                warn!("VoluntaryWithTimeout not fully implemented, using RoundRobin");
                let speaker_id = participants[*speaker_index % participants.len()].clone();
                *speaker_index += 1;
                Ok(speaker_id)
            }
        }
    }

    /// Parse next speaker directive from moderator's message
    ///
    /// Looks for patterns like "Next: [name]", "@[name]", or "[name] please respond"
    fn parse_next_speaker(&self, message: &str, participants: &[String]) -> Option<String> {
        let message_lower = message.to_lowercase();

        // Try common patterns
        for participant in participants {
            let participant_lower = participant.to_lowercase();

            // Check for explicit mentions
            if message_lower.contains(&format!("next: {}", participant_lower))
                || message_lower.contains(&format!("@{}", participant_lower))
                || message_lower.contains(&format!("{} please", participant_lower))
            {
                return Some(participant.clone());
            }
        }

        None
    }

    /// Format conversation history for context
    ///
    /// Converts the transcript into a formatted string suitable for Paladin context
    fn format_conversation_history(&self, transcript: &[CouncilMessage], topic: &str) -> String {
        let mut formatted = format!("Discussion Topic: {}\n\n", topic);
        formatted.push_str("Conversation History:\n");
        formatted.push_str("=".repeat(60).as_str());
        formatted.push('\n');

        for message in transcript {
            formatted.push_str(&message.format());
            formatted.push('\n');
        }

        formatted.push_str("=".repeat(60).as_str());
        formatted.push_str("\n\nPlease provide your response:");

        formatted
    }

    /// Execute a single speaker turn
    ///
    /// This is a placeholder for actual Paladin execution.
    /// In a real implementation, this would look up the Paladin and execute it.
    async fn execute_speaker(
        &self,
        speaker_id: &str,
        _context: &str,
    ) -> Result<String, PaladinError> {
        // TODO: Implement actual Paladin lookup and execution
        // For now, this is a placeholder that will be completed during integration
        debug!("Executing speaker: {}", speaker_id);

        // This would normally:
        // 1. Look up Paladin from a registry/repository
        // 2. Execute paladin_port.execute(paladin, context)
        // 3. Return the result

        Err(PaladinError::ConfigurationError(
            "Paladin lookup not yet implemented".to_string(),
        ))
    }

    /// Store a message in the Garrison
    async fn store_in_garrison(
        &self,
        garrison: &dyn GarrisonPort,
        message: &CouncilMessage,
        topic: &str,
    ) -> Result<(), BattalionError> {
        let entry = GarrisonEntry::new(
            ConversationRole::Assistant,
            format!(
                "[Council: {}] Round {}: {}",
                topic, message.round, message.content
            ),
        );

        garrison
            .remember(entry)
            .await
            .map_err(|e| BattalionError::ExecutionError(format!("Garrison error: {}", e)))
    }

    /// Check if termination condition is met
    fn should_terminate(
        &self,
        condition: &TerminationCondition,
        transcript: &[CouncilMessage],
        current_round: u32,
        max_rounds: &u32,
    ) -> bool {
        match condition {
            TerminationCondition::MaxRounds => current_round > *max_rounds,
            TerminationCondition::Consensus => self.detect_consensus(transcript),
            TerminationCondition::ModeratorDecision => self.detect_moderator_conclusion(transcript),
            TerminationCondition::Keyword(keyword) => self.detect_keyword(transcript, keyword),
        }
    }

    /// Detect consensus keywords in recent messages
    fn detect_consensus(&self, transcript: &[CouncilMessage]) -> bool {
        // Check last few messages for consensus indicators
        let recent_messages: Vec<&CouncilMessage> = transcript.iter().rev().take(3).collect();

        let consensus_patterns = [
            " consensus",
            " agree ",
            " agreed ",
            "we agree",
            "in agreement",
            " unanimous",
            "i agree",
        ];

        recent_messages.iter().any(|msg| {
            let content_lower = format!(" {} ", msg.content.to_lowercase());
            consensus_patterns
                .iter()
                .any(|pattern| content_lower.contains(pattern))
        })
    }

    /// Detect moderator conclusion statement
    fn detect_moderator_conclusion(&self, transcript: &[CouncilMessage]) -> bool {
        if let Some(last_message) = transcript.last() {
            let content_lower = last_message.content.to_lowercase();
            content_lower.contains("discussion concluded")
                || content_lower.contains("meeting adjourned")
                || content_lower.contains("we have reached a conclusion")
        } else {
            false
        }
    }

    /// Detect custom keyword in transcript
    fn detect_keyword(&self, transcript: &[CouncilMessage], keyword: &str) -> bool {
        if let Some(last_message) = transcript.last() {
            last_message
                .content
                .to_lowercase()
                .contains(&keyword.to_lowercase())
        } else {
            false
        }
    }

    /// Extract conclusion from the conversation
    ///
    /// Attempts to identify a final conclusion or summary from the transcript
    fn extract_conclusion(
        &self,
        transcript: &[CouncilMessage],
        moderator_id: &Option<String>,
    ) -> Option<String> {
        // If there's a moderator, prefer their last message as conclusion
        if let Some(mod_id) = moderator_id {
            for message in transcript.iter().rev() {
                if &message.speaker == mod_id {
                    return Some(message.content.clone());
                }
            }
        }

        // Otherwise, use the last message
        transcript.last().map(|msg| msg.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::battalion::council::CouncilBuilder;
    use crate::core::platform::container::paladin::Paladin;

    #[test]
    fn test_format_conversation_history() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "I think we should proceed", 1),
            CouncilMessage::new("expert_2", "I agree with expert_1", 1),
        ];

        let formatted = service.format_conversation_history(&messages, "Test Topic");

        assert!(formatted.contains("Test Topic"));
        assert!(formatted.contains("expert_1"));
        assert!(formatted.contains("I think we should proceed"));
        assert!(formatted.contains("expert_2"));
    }

    #[test]
    fn test_detect_consensus() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "I think option A is best", 1),
            CouncilMessage::new("expert_2", "I agree with expert_1", 1),
            CouncilMessage::new("expert_3", "We have reached consensus on option A", 1),
        ];

        assert!(service.detect_consensus(&messages));
    }

    #[test]
    fn test_detect_consensus_negative() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "I think option A is best", 1),
            CouncilMessage::new("expert_2", "I disagree strongly", 1),
            CouncilMessage::new("expert_3", "I have concerns about this approach", 1),
        ];

        assert!(!service.detect_consensus(&messages));
    }

    #[test]
    fn test_detect_moderator_conclusion() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "My opinion is X", 1),
            CouncilMessage::new("moderator", "Thank you all. Discussion concluded.", 1),
        ];

        assert!(service.detect_moderator_conclusion(&messages));
    }

    #[test]
    fn test_detect_keyword() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "Let's discuss more", 1),
            CouncilMessage::new("expert_2", "DONE with discussion", 1),
        ];

        assert!(service.detect_keyword(&messages, "DONE"));
        assert!(!service.detect_keyword(&messages, "STOP"));
    }

    #[test]
    fn test_parse_next_speaker() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let participants = vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ];

        // Test various patterns
        assert_eq!(
            service.parse_next_speaker("Next: alice", &participants),
            Some("alice".to_string())
        );

        assert_eq!(
            service.parse_next_speaker("@bob please respond", &participants),
            Some("bob".to_string())
        );

        assert_eq!(
            service.parse_next_speaker("charlie please provide your input", &participants),
            Some("charlie".to_string())
        );

        assert_eq!(
            service.parse_next_speaker("no match here", &participants),
            None
        );
    }

    #[test]
    fn test_extract_conclusion_with_moderator() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "I think A", 1),
            CouncilMessage::new("moderator", "Final decision: We go with A", 1),
            CouncilMessage::new("expert_2", "Sounds good", 1),
        ];

        let conclusion = service.extract_conclusion(&messages, &Some("moderator".to_string()));
        assert_eq!(conclusion, Some("Final decision: We go with A".to_string()));
    }

    #[test]
    fn test_extract_conclusion_without_moderator() {
        let service = CouncilExecutionService::new(Arc::new(MockPaladinPort), None);

        let messages = vec![
            CouncilMessage::new("expert_1", "First message", 1),
            CouncilMessage::new("expert_2", "Last message", 1),
        ];

        let conclusion = service.extract_conclusion(&messages, &None);
        assert_eq!(conclusion, Some("Last message".to_string()));
    }

    // Mock PaladinPort for testing
    struct MockPaladinPort;

    #[async_trait::async_trait]
    impl PaladinPort for MockPaladinPort {
        async fn execute(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<crate::application::ports::output::paladin_port::PaladinResult, PaladinError>
        {
            Ok(
                crate::application::ports::output::paladin_port::PaladinResult {
                    output: "Mock response".to_string(),
                    token_count: 100,
                    execution_time_ms: 1000,
                    loop_count: 1,
                    stop_reason:
                        crate::application::ports::output::paladin_port::StopReason::Completed,
                },
            )
        }

        async fn execute_stream(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<crate::application::ports::output::paladin_port::PaladinStream, PaladinError>
        {
            unimplemented!("Streaming not needed for Council tests")
        }

        fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
            Ok(())
        }
    }
}
