//! Council command - quick group discussions
//!
//! This command facilitates structured multi-agent discussions with
//! role-based participation, visual formatting, and transcript saving.

use crate::application::cli::error::CliError;
use crate::application::cli::formatters::output::OutputFormatter;
use crate::application::cli::interactive::prompts::PromptBuilder;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Default participant roles based on count
const DEFAULT_ROLES_2: &[&str] = &["Advocate", "Critic"];
const DEFAULT_ROLES_3: &[&str] = &["Advocate", "Critic", "Moderator"];
const DEFAULT_ROLES_4: &[&str] = &["Advocate", "Critic", "Moderator", "Synthesizer"];
const DEFAULT_ROLES_5: &[&str] = &[
    "Advocate",
    "Critic",
    "Moderator",
    "Synthesizer",
    "Subject Matter Expert",
];

/// Council participant with role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub name: String,
    pub role: String,
}

/// Discussion turn in the council
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionTurn {
    pub round: usize,
    pub participant: String,
    pub role: String,
    pub content: String,
}

/// Council discussion transcript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilTranscript {
    pub topic: String,
    pub participants: Vec<Participant>,
    pub turns: Vec<DiscussionTurn>,
    pub summary: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
}

/// Run a council discussion
pub async fn run_council(
    topic: Option<String>,
    participants: usize,
    roles: Option<Vec<String>>,
    max_rounds: usize,
    save: Option<String>,
    model: Option<String>,
    temperature: Option<f32>,
) -> Result<(), CliError> {
    let formatter = OutputFormatter::new();
    formatter.header("🏛️ Council - Group Discussion");

    // Step 1: Validate and get inputs
    let participant_count = validate_participant_count(participants)?;
    let topic_text = get_topic(topic, &formatter).await?;
    let participant_roles = determine_roles(participant_count, roles)?;

    // Step 2: Display configuration
    display_council_setup(&formatter, &topic_text, &participant_roles, max_rounds);

    // Step 3: Run discussion (simulated for now)
    formatter.section("💬 Discussion");
    let transcript = run_discussion(
        &topic_text,
        &participant_roles,
        max_rounds,
        model.as_deref(),
        temperature,
        &formatter,
    )
    .await?;

    // Step 4: Generate summary
    formatter.section("📊 Summary");
    let summary = generate_summary(&transcript, &formatter)?;

    // Step 5: Save transcript if requested
    if let Some(path) = save {
        save_transcript(&transcript, &summary, &path).await?;
        formatter.success(&format!("Transcript saved to: {}", path));
    }

    formatter.success("Council discussion completed!");
    Ok(())
}

/// Validate participant count
fn validate_participant_count(count: usize) -> Result<usize, CliError> {
    if count < 2 {
        return Err(CliError::validation(
            "Council requires at least 2 participants",
        ));
    }
    if count > 10 {
        return Err(CliError::validation(
            "Council supports maximum 10 participants",
        ));
    }
    Ok(count)
}

/// Get discussion topic
async fn get_topic(topic: Option<String>, formatter: &OutputFormatter) -> Result<String, CliError> {
    if let Some(topic) = topic {
        Ok(topic)
    } else {
        formatter.section("📋 Discussion Topic");
        PromptBuilder::input("What topic should the council discuss?")
            .allow_empty(false)
            .prompt()
    }
}

/// Determine participant roles
fn determine_roles(
    participant_count: usize,
    custom_roles: Option<Vec<String>>,
) -> Result<Vec<String>, CliError> {
    if let Some(roles) = custom_roles {
        if roles.len() != participant_count {
            return Err(CliError::validation(format!(
                "Number of roles ({}) must match participant count ({})",
                roles.len(),
                participant_count
            )));
        }
        Ok(roles)
    } else {
        Ok(get_default_roles(participant_count))
    }
}

/// Get default roles based on participant count
fn get_default_roles(count: usize) -> Vec<String> {
    let base_roles = match count {
        2 => DEFAULT_ROLES_2,
        3 => DEFAULT_ROLES_3,
        4 => DEFAULT_ROLES_4,
        5 => DEFAULT_ROLES_5,
        _ => {
            // For 6+ participants, use base 5 and add numbered experts
            let mut roles: Vec<String> = DEFAULT_ROLES_5.iter().map(|s| s.to_string()).collect();
            for i in 6..=count {
                roles.push(format!("Expert {}", i - 4));
            }
            return roles;
        }
    };
    base_roles.iter().map(|s| s.to_string()).collect()
}

/// Display council setup
fn display_council_setup(
    formatter: &OutputFormatter,
    topic: &str,
    roles: &[String],
    max_rounds: usize,
) {
    formatter.section("⚙️ Configuration");
    formatter.info(&format!("Topic: {}", topic));
    formatter.info(&format!("Participants: {}", roles.len()));
    formatter.info(&format!("Max Rounds: {}", max_rounds));
    formatter.blank_line();
    formatter.info("Roles:");
    for (i, role) in roles.iter().enumerate() {
        formatter.info(&format!("  {}. {}", i + 1, role));
    }
    formatter.blank_line();
}

/// Run the council discussion (simplified simulation)
async fn run_discussion(
    topic: &str,
    roles: &[String],
    max_rounds: usize,
    _model: Option<&str>,
    _temperature: Option<f32>,
    formatter: &OutputFormatter,
) -> Result<CouncilTranscript, CliError> {
    let participants: Vec<Participant> = roles
        .iter()
        .enumerate()
        .map(|(i, role)| Participant {
            name: format!("Participant{}", i + 1),
            role: role.clone(),
        })
        .collect();

    let mut turns = Vec::new();

    // Simulate discussion rounds
    for round in 1..=max_rounds {
        formatter.info(&format!("\n═══ Round {} ═══", round));

        for participant in &participants {
            // Simulate a turn
            let content = format!(
                "As the {}, I believe this topic requires careful consideration of multiple perspectives.",
                participant.role
            );

            // Display turn with formatting
            display_turn(
                round,
                &participant.name,
                &participant.role,
                &content,
                formatter,
            );

            turns.push(DiscussionTurn {
                round,
                participant: participant.name.clone(),
                role: participant.role.clone(),
                content,
            });
        }
    }

    Ok(CouncilTranscript {
        topic: topic.to_string(),
        participants,
        turns,
        summary: None,
        started_at: Utc::now().to_rfc3339(),
        ended_at: Some(Utc::now().to_rfc3339()),
    })
}

/// Display a discussion turn with formatting
fn display_turn(
    _round: usize,
    participant: &str,
    role: &str,
    content: &str,
    formatter: &OutputFormatter,
) {
    formatter.blank_line();
    formatter.info(&format!("🗣️  {} [{}]", participant, role));
    formatter.info(&format!("   {}", content));
}

/// Generate discussion summary
fn generate_summary(
    transcript: &CouncilTranscript,
    formatter: &OutputFormatter,
) -> Result<String, CliError> {
    let total_turns = transcript.turns.len();
    let rounds_completed = transcript.turns.iter().map(|t| t.round).max().unwrap_or(0);

    let summary = format!(
        r#"Discussion Topic: {}
Participants: {}
Total Rounds: {}
Total Contributions: {}

Key Points:
- Multiple perspectives were considered
- All participants contributed to the discussion
- The council reached a thorough examination of the topic

Consensus:
- The topic requires further consideration
- All viewpoints were heard and acknowledged

Conclusion:
The council successfully completed {} rounds of discussion with {} participants contributing {} total statements."#,
        transcript.topic,
        transcript.participants.len(),
        rounds_completed,
        total_turns,
        rounds_completed,
        transcript.participants.len(),
        total_turns
    );

    formatter.info(&summary);
    Ok(summary)
}

/// Save transcript to file
async fn save_transcript(
    transcript: &CouncilTranscript,
    summary: &str,
    path: &str,
) -> Result<(), CliError> {
    let mut content = String::new();

    // Header
    content.push_str(&format!(
        "# Council Discussion Transcript\n\nTopic: {}\n",
        transcript.topic
    ));
    content.push_str(&format!("Started: {}\n", transcript.started_at));
    if let Some(ended) = &transcript.ended_at {
        content.push_str(&format!("Ended: {}\n", ended));
    }
    content.push_str("\n## Participants\n\n");

    for participant in &transcript.participants {
        content.push_str(&format!("- {}: {}\n", participant.name, participant.role));
    }

    // Discussion turns
    content.push_str("\n## Discussion\n\n");
    for turn in &transcript.turns {
        content.push_str(&format!(
            "### Round {} - {} [{}]\n\n{}\n\n",
            turn.round, turn.participant, turn.role, turn.content
        ));
    }

    // Summary
    content.push_str(&format!("\n## Summary\n\n{}\n", summary));

    fs::write(path, content)
        .await
        .map_err(|e| CliError::IoError {
            message: format!("Failed to write transcript to {}", path),
            source: e,
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_participant_count_valid() {
        assert!(validate_participant_count(2).is_ok());
        assert!(validate_participant_count(5).is_ok());
        assert!(validate_participant_count(10).is_ok());
    }

    #[test]
    fn test_validate_participant_count_too_few() {
        assert!(validate_participant_count(0).is_err());
        assert!(validate_participant_count(1).is_err());
    }

    #[test]
    fn test_validate_participant_count_too_many() {
        assert!(validate_participant_count(11).is_err());
        assert!(validate_participant_count(100).is_err());
    }

    #[test]
    fn test_get_default_roles_2() {
        let roles = get_default_roles(2);
        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0], "Advocate");
        assert_eq!(roles[1], "Critic");
    }

    #[test]
    fn test_get_default_roles_3() {
        let roles = get_default_roles(3);
        assert_eq!(roles.len(), 3);
        assert_eq!(roles[0], "Advocate");
        assert_eq!(roles[1], "Critic");
        assert_eq!(roles[2], "Moderator");
    }

    #[test]
    fn test_get_default_roles_4() {
        let roles = get_default_roles(4);
        assert_eq!(roles.len(), 4);
        assert_eq!(roles[3], "Synthesizer");
    }

    #[test]
    fn test_get_default_roles_5() {
        let roles = get_default_roles(5);
        assert_eq!(roles.len(), 5);
        assert_eq!(roles[4], "Subject Matter Expert");
    }

    #[test]
    fn test_get_default_roles_6_plus() {
        let roles = get_default_roles(7);
        assert_eq!(roles.len(), 7);
        assert_eq!(roles[5], "Expert 2");
        assert_eq!(roles[6], "Expert 3");
    }

    #[test]
    fn test_determine_roles_default() {
        let roles = determine_roles(3, None).unwrap();
        assert_eq!(roles.len(), 3);
        assert_eq!(roles[0], "Advocate");
    }

    #[test]
    fn test_determine_roles_custom() {
        let custom = vec![
            "Role1".to_string(),
            "Role2".to_string(),
            "Role3".to_string(),
        ];
        let roles = determine_roles(3, Some(custom)).unwrap();
        assert_eq!(roles.len(), 3);
        assert_eq!(roles[0], "Role1");
    }

    #[test]
    fn test_determine_roles_custom_mismatch() {
        let custom = vec!["Role1".to_string(), "Role2".to_string()];
        let result = determine_roles(3, Some(custom));
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_summary() {
        let transcript = CouncilTranscript {
            topic: "Test Topic".to_string(),
            participants: vec![
                Participant {
                    name: "P1".to_string(),
                    role: "Advocate".to_string(),
                },
                Participant {
                    name: "P2".to_string(),
                    role: "Critic".to_string(),
                },
            ],
            turns: vec![DiscussionTurn {
                round: 1,
                participant: "P1".to_string(),
                role: "Advocate".to_string(),
                content: "Test content".to_string(),
            }],
            summary: None,
            started_at: Utc::now().to_rfc3339(),
            ended_at: Some(Utc::now().to_rfc3339()),
        };

        let formatter = OutputFormatter::new();
        let summary = generate_summary(&transcript, &formatter).unwrap();
        assert!(summary.contains("Test Topic"));
        assert!(summary.contains("2"));
    }
}
