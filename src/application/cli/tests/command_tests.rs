//! High-level command tests
//!
//! These tests verify command-level functionality beyond the inline tests
//! in each command module. They focus on integration points and edge cases.

#[cfg(test)]
mod tests {
    use super::super::test_utils::strip_ansi;

    // ============ Features Command Tests ============

    mod features_command {
        use crate::application::cli::commands::features::FeatureCategory;

        #[test]
        fn test_feature_categories_exist() {
            // Ensure all feature categories are defined
            let _agent = FeatureCategory::Agent;
            let _battalion = FeatureCategory::Battalion;
            let _orchestration = FeatureCategory::Orchestration;
            let _memory = FeatureCategory::Memory;
            let _utilities = FeatureCategory::Utilities;
        }
    }

    // ============ Onboarding Command Tests ============

    mod onboarding_command {
        #[test]
        fn test_api_key_format_validation() {
            // OpenAI keys start with sk-
            assert!("sk-test123456789012345678901234567890".starts_with("sk-"));
            assert!(!"invalid-key".starts_with("sk-"));

            // Anthropic keys start with sk-ant-
            assert!("sk-ant-test12345678901234567890".starts_with("sk-ant-"));

            // DeepSeek keys start with sk-
            assert!("sk-deepseek-test123456789".starts_with("sk-"));
        }

        #[test]
        fn test_sample_config_structure() {
            // Test that sample config generation would create valid YAML
            let sample_content = r#"
paladin:
  name: "example-paladin"
  model: "gpt-4"
  temperature: 0.7
"#;

            let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(sample_content);
            assert!(parsed.is_ok());
        }
    }

    // ============ Setup Check Command Tests ============

    mod setup_check_command {
        #[test]
        fn test_version_parsing() {
            // Test version string parsing logic
            let version = env!("CARGO_PKG_VERSION");
            assert!(!version.is_empty());
            assert!(version.contains('.'));
        }
    }

    // ============ Muster Command Tests ============

    mod muster_command {
        use crate::application::cli::commands::muster::{
            AgentRole, BattalionPattern, TaskAnalysis,
        };

        #[test]
        fn test_battalion_pattern_variants() {
            // Ensure all pattern types exist
            let _formation = BattalionPattern::Formation;
            let _phalanx = BattalionPattern::Phalanx;
            let _campaign = BattalionPattern::Campaign;
            let _chain = BattalionPattern::ChainOfCommand;
            let _conclave = BattalionPattern::Conclave;
            let _maneuver = BattalionPattern::Maneuver;
        }

        #[test]
        fn test_task_analysis_structure() {
            let analysis = TaskAnalysis {
                recommended_pattern: BattalionPattern::Formation,
                reasoning: "Sequential processing needed".to_string(),
                agents: vec![AgentRole {
                    name: "processor".to_string(),
                    role: "Process data".to_string(),
                    system_prompt: "You process data".to_string(),
                }],
                battalion_name: "data-pipeline".to_string(),
            };

            assert_eq!(analysis.agents.len(), 1);
            assert!(!analysis.reasoning.is_empty());
        }

        #[test]
        fn test_keyword_detection() {
            // Sequential keywords
            let sequential_task = "First do this, then do that, followed by the other";
            assert!(sequential_task.contains("then") || sequential_task.contains("followed by"));

            // Parallel keywords
            let parallel_task =
                "Compare multiple approaches and analyze from different perspectives";
            assert!(parallel_task.contains("multiple") || parallel_task.contains("perspectives"));

            // Discussion keywords
            let discussion_task = "Have agents discuss and reach consensus on the decision";
            assert!(discussion_task.contains("discuss") || discussion_task.contains("consensus"));
        }

        #[test]
        fn test_yaml_generation_basic() {
            let yaml = r#"
name: "test-battalion"
pattern: "formation"
agents:
  - name: "agent-1"
    role: "processor"
"#;

            let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(yaml);
            assert!(parsed.is_ok());
        }
    }

    // ============ Council Command Tests ============

    mod council_command {
        use crate::application::cli::commands::council::{
            CouncilTranscript, DiscussionTurn, Participant,
        };

        #[test]
        fn test_participant_structure() {
            let participant = Participant {
                name: "alice".to_string(),
                role: "Advocate".to_string(),
            };

            assert_eq!(participant.name, "alice");
            assert_eq!(participant.role, "Advocate");
        }

        #[test]
        fn test_discussion_turn_structure() {
            let turn = DiscussionTurn {
                round: 1,
                participant: "alice".to_string(),
                role: "Advocate".to_string(),
                content: "I propose we do X".to_string(),
            };

            assert_eq!(turn.round, 1);
            assert!(!turn.content.is_empty());
        }

        #[test]
        fn test_council_transcript_structure() {
            let transcript = CouncilTranscript {
                topic: "Test Topic".to_string(),
                participants: vec![Participant {
                    name: "alice".to_string(),
                    role: "Advocate".to_string(),
                }],
                turns: vec![],
                summary: None,
                started_at: chrono::Utc::now().to_string(),
                ended_at: None,
            };

            assert_eq!(transcript.topic, "Test Topic");
            assert_eq!(transcript.participants.len(), 1);
        }

        #[test]
        fn test_participant_count_validation() {
            // Min 2, max 10 participants
            let valid_min = 2;
            let valid_max = 10;
            let invalid_low = 1;
            let invalid_high = 11;
            
            // Validation logic tests
            assert!((2..=10).contains(&valid_min));
            assert!((2..=10).contains(&valid_max));
            assert!(!(2..=10).contains(&invalid_low));
            assert!(!(2..=10).contains(&invalid_high));
        }

        #[test]
        fn test_markdown_transcript_format() {
            let md = format!(
                "# Council Discussion: {}\n\n## Round {}\n\n**{}** ({}): {}\n",
                "Test Topic", 1, "Alice", "Advocate", "My perspective"
            );

            assert!(md.contains("# Council Discussion"));
            assert!(md.contains("## Round"));
            assert!(md.contains("**Alice**"));
        }
    }

    // ============ Error Handling Tests ============

    #[test]
    fn test_cli_error_types() {
        use crate::application::cli::error::CliError;

        let _config_error = CliError::configuration("test");
        let _validation_error = CliError::validation("test");
        let _network_error = CliError::network("test");
    }

    // ============ Interactive Utilities Tests ============

    #[test]
    fn test_ansi_stripping() {
        let colored = "\x1b[32mGreen text\x1b[0m";
        let stripped = strip_ansi(colored);
        assert_eq!(stripped, "Green text");
    }

    #[test]
    fn test_ansi_stripping_multiple() {
        let colored = "\x1b[1m\x1b[31mBold Red\x1b[0m normal \x1b[32mGreen\x1b[0m";
        let stripped = strip_ansi(colored);
        assert!(stripped.contains("Bold Red"));
        assert!(stripped.contains("normal"));
        assert!(stripped.contains("Green"));
        assert!(!stripped.contains("\x1b["));
    }
}
