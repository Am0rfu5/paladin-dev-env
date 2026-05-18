//! Chain of Command Execution Service
//!
//! This service implements the Chain of Command Battalion orchestration pattern.
//! A Chain of Command consists of a commander Paladin and specialist Paladins,
//! where the commander coordinates and delegates tasks to specialists based on
//! the configured delegation strategy.
//!
//! # Delegation Strategies
//!
//! - **Automatic**: Commander analyzes input and selects appropriate specialist(s)
//! - **Broadcast**: All specialists execute concurrently
//! - **RoundRobin**: Rotate through specialists on consecutive calls
//! - **Custom**: User-defined delegation logic
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::application::use_cases::battalion::chain_of_command_service::ChainOfCommandExecutionService;
//! use paladin::core::platform::container::battalion::{ChainOfCommand, DelegationStrategy};
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let paladin_port = unimplemented!();
//! # let commander = unimplemented!();
//! # let specialists = vec![];
//! # let config = paladin::core::platform::container::battalion::BattalionConfig::default();
//! let service = ChainOfCommandExecutionService::new(paladin_port);
//! let chain = ChainOfCommand::new(commander, specialists, config)?
//!     .with_strategy(DelegationStrategy::Automatic);
//!
//! let result = service.execute(&chain, "Analyze this data").await?;
//! println!("Selected specialists: {:?}", result.selected_specialists);
//! # Ok(())
//! # }
//! ```

use crate::application::use_cases::paladin::error::PaladinError;
use crate::core::platform::container::battalion::BattalionError;
use crate::core::platform::container::battalion::chain_of_command::{
    ChainOfCommand, DelegationStrategy,
};
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Result of executing a Chain of Command delegation
#[derive(Debug, Clone)]
pub struct DelegationResult {
    /// Names of specialists that were selected for execution
    pub selected_specialists: Vec<String>,
    /// Commander's reasoning for specialist selection (if Automatic strategy)
    pub reasoning: String,
    /// Outputs from the executed specialists
    pub outputs: Vec<String>,
}

/// Service for executing Chain of Command Battalion patterns
///
/// This service coordinates the execution of a commander Paladin and specialist
/// Paladins according to the configured delegation strategy.
pub struct ChainOfCommandExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
    /// Round-robin state: maps chain ID to current index
    round_robin_state: Arc<Mutex<HashMap<String, usize>>>,
}

impl ChainOfCommandExecutionService {
    /// Create a new Chain of Command execution service
    ///
    /// # Arguments
    ///
    /// * `paladin_port` - Port for executing individual Paladins
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use paladin::application::use_cases::battalion::chain_of_command_service::ChainOfCommandExecutionService;
    /// use std::sync::Arc;
    ///
    /// # let paladin_port = unimplemented!();
    /// let service = ChainOfCommandExecutionService::new(paladin_port);
    /// ```
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self {
        Self {
            paladin_port,
            round_robin_state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Validate a Chain of Command configuration
    ///
    /// # Arguments
    ///
    /// * `chain` - The Chain of Command to validate
    ///
    /// # Errors
    ///
    /// Returns error if the chain configuration is invalid
    pub async fn validate(&self, chain: &ChainOfCommand) -> Result<(), BattalionError> {
        chain.validate()?;
        Ok(())
    }

    /// Execute a Chain of Command with the given input
    ///
    /// The execution flow depends on the delegation strategy:
    ///
    /// 1. **Automatic**: Commander analyzes input and selects specialist(s)
    /// 2. **Broadcast**: All specialists execute concurrently
    /// 3. **RoundRobin**: Next specialist in rotation executes
    /// 4. **Custom**: User-defined logic determines execution
    ///
    /// # Arguments
    ///
    /// * `chain` - The Chain of Command to execute
    /// * `input` - Input to process
    ///
    /// # Returns
    ///
    /// Returns a `DelegationResult` containing selected specialists, reasoning, and outputs
    ///
    /// # Errors
    ///
    /// Returns error if execution fails or validation fails
    pub async fn execute(
        &self,
        chain: &ChainOfCommand,
        input: &str,
    ) -> Result<DelegationResult, BattalionError> {
        // Validate first
        self.validate(chain).await?;

        // Execute based on delegation strategy
        match chain.delegation_strategy() {
            DelegationStrategy::Automatic => self.execute_automatic(chain, input).await,
            DelegationStrategy::Broadcast => self.execute_broadcast(chain, input).await,
            DelegationStrategy::RoundRobin => self.execute_round_robin(chain, input).await,
            DelegationStrategy::Custom(logic) => self.execute_custom(chain, input, logic).await,
        }
    }

    /// Execute with Automatic delegation strategy
    ///
    /// Commander analyzes the input and selects appropriate specialist(s)
    ///
    /// The commander is provided with:
    /// 1. The original input
    /// 2. Descriptions of all available specialists
    /// 3. Instructions to select specialists and provide reasoning
    ///
    /// The commander's response should follow this format:
    /// ```text
    /// SELECT: specialist_name1, specialist_name2
    /// REASON: Explanation of why these specialists were chosen
    /// ```
    async fn execute_automatic(
        &self,
        chain: &ChainOfCommand,
        input: &str,
    ) -> Result<DelegationResult, BattalionError> {
        // Build specialist descriptions for commander context
        let specialist_descriptions: Vec<String> = chain
            .specialists()
            .iter()
            .map(|p| {
                format!(
                    "- {}: {}",
                    p.node.name,
                    p.node.system_prompt.lines().next().unwrap_or("")
                )
            })
            .collect();

        // Build commander prompt with specialist context
        let commander_prompt = format!(
            r#"You are a commander coordinating a team of specialists. Your task is to analyze the following request and select the appropriate specialist(s) to handle it.

Available Specialists:
{}

User Request:
{}

Instructions:
1. Analyze the request carefully
2. Select one or more specialists best suited for this task
3. Respond EXACTLY in this format:

SELECT: specialist_name1, specialist_name2
REASON: Brief explanation of your selection

Important: Use the exact specialist names shown above. Separate multiple specialists with commas."#,
            specialist_descriptions.join("\n"),
            input
        );

        // Execute commander to get specialist selection
        let commander_result = self
            .paladin_port
            .execute(chain.commander(), &commander_prompt)
            .await?;

        // Parse commander's response to extract specialist selection and reasoning
        let (selected_names, reasoning) =
            self.parse_commander_response(&commander_result.output)?;

        // Validate that selected specialists exist
        let selected_specialists: Vec<&crate::core::platform::container::paladin::Paladin> =
            selected_names
                .iter()
                .map(|name| {
                    chain
                        .specialists()
                        .iter()
                        .find(|s| s.node.name == *name)
                        .ok_or_else(|| {
                            BattalionError::ExecutionError(format!(
                                "Commander selected non-existent specialist: {}",
                                name
                            ))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;

        // Execute selected specialists with the original input
        let mut outputs = Vec::new();
        for specialist in &selected_specialists {
            let result = self.paladin_port.execute(specialist, input).await?;
            outputs.push(result.output);
        }

        Ok(DelegationResult {
            selected_specialists: selected_names,
            reasoning,
            outputs,
        })
    }

    /// Parse commander's response to extract specialist selection and reasoning
    ///
    /// Expected format:
    /// ```text
    /// SELECT: specialist1, specialist2
    /// REASON: explanation text
    /// ```
    fn parse_commander_response(
        &self,
        response: &str,
    ) -> Result<(Vec<String>, String), BattalionError> {
        let mut selected = Vec::new();
        let mut reasoning = String::new();

        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("SELECT:") {
                let selection = line.strip_prefix("SELECT:").unwrap().trim();
                selected = selection
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            } else if line.starts_with("REASON:") {
                reasoning = line.strip_prefix("REASON:").unwrap().trim().to_string();
            }
        }

        if selected.is_empty() {
            return Err(BattalionError::ExecutionError(
                "Commander did not select any specialists".to_string(),
            ));
        }

        if reasoning.is_empty() {
            reasoning = "No reasoning provided".to_string();
        }

        Ok((selected, reasoning))
    }

    /// Execute with Broadcast delegation strategy
    ///
    /// All specialists execute concurrently with the same input
    ///
    /// # Behavior
    ///
    /// - All specialists receive the same input simultaneously
    /// - Execution happens concurrently using tokio::spawn
    /// - All results are collected regardless of individual failures (per error strategy)
    /// - No commander analysis is performed
    async fn execute_broadcast(
        &self,
        chain: &ChainOfCommand,
        input: &str,
    ) -> Result<DelegationResult, BattalionError> {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        // Spawn concurrent execution for all specialists
        for specialist in chain.specialists() {
            let specialist_clone: crate::core::platform::container::paladin::Paladin =
                specialist.clone();
            let input_clone = input.to_string();
            let port_clone = Arc::clone(&self.paladin_port);

            join_set.spawn(async move {
                let result: Result<PaladinResult, PaladinError> =
                    port_clone.execute(&specialist_clone, &input_clone).await;
                (specialist_clone.node.name.clone(), result)
            });
        }

        // Collect all results
        let mut outputs = Vec::new();
        let mut selected_specialists = Vec::new();

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((name, Ok(paladin_result))) => {
                    selected_specialists.push(name);
                    outputs.push(paladin_result.output);
                }
                Ok((name, Err(e))) => {
                    // Handle specialist failure based on error strategy
                    // For now, propagate the error
                    return Err(BattalionError::PaladinError(format!(
                        "Specialist {} failed: {}",
                        name, e
                    )));
                }
                Err(join_error) => {
                    return Err(BattalionError::ExecutionError(format!(
                        "Task join error: {}",
                        join_error
                    )));
                }
            }
        }

        Ok(DelegationResult {
            selected_specialists,
            reasoning: "Broadcast to all specialists concurrently".to_string(),
            outputs,
        })
    }

    /// Execute with RoundRobin delegation strategy
    ///
    /// Rotate through specialists on consecutive calls
    ///
    /// # Behavior
    ///
    /// - Maintains state across calls using a unique chain identifier
    /// - Cycles through specialists in order: 0 -> 1 -> 2 -> ... -> N -> 0
    /// - State is thread-safe using Mutex
    /// - Only one specialist executes per call
    async fn execute_round_robin(
        &self,
        chain: &ChainOfCommand,
        input: &str,
    ) -> Result<DelegationResult, BattalionError> {
        let specialists = chain.specialists();
        if specialists.is_empty() {
            return Err(BattalionError::ValidationError(
                "No specialists available for round-robin delegation".to_string(),
            ));
        }

        // Generate a unique ID for this chain based on commander + specialists
        let chain_id = format!(
            "{}:{}",
            chain.commander().node.name,
            specialists
                .iter()
                .map(|s| s.node.name.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );

        // Get current index and increment for next call
        let current_index = {
            let mut state = self.round_robin_state.lock().unwrap();
            let index = state.entry(chain_id.clone()).or_insert(0);
            let current = *index;
            *index = (current + 1) % specialists.len();
            current
        };

        // Execute the selected specialist
        let selected_specialist = &specialists[current_index];
        let result = self
            .paladin_port
            .execute(selected_specialist, input)
            .await?;

        Ok(DelegationResult {
            selected_specialists: vec![selected_specialist.node.name.clone()],
            reasoning: format!(
                "Round-robin delegation selected specialist {} of {}",
                current_index + 1,
                specialists.len()
            ),
            outputs: vec![result.output],
        })
    }

    /// Execute with Custom delegation strategy
    ///
    /// User-defined logic determines specialist selection
    ///
    /// # Behavior
    ///
    /// - Custom logic string describes the delegation approach
    /// - For now, defaults to selecting the first specialist
    /// - Future enhancements could support more sophisticated custom logic parsing
    ///
    /// # Arguments
    ///
    /// * `chain` - The Chain of Command to execute
    /// * `input` - Input to process
    /// * `logic` - User-defined logic description
    async fn execute_custom(
        &self,
        chain: &ChainOfCommand,
        input: &str,
        logic: &str,
    ) -> Result<DelegationResult, BattalionError> {
        let specialists = chain.specialists();
        if specialists.is_empty() {
            return Err(BattalionError::ValidationError(
                "No specialists available for custom delegation".to_string(),
            ));
        }

        // For now, custom delegation defaults to selecting the first specialist
        // Future enhancement: parse logic string for more sophisticated selection
        let selected_specialist = &specialists[0];

        let result = self
            .paladin_port
            .execute(selected_specialist, input)
            .await?;

        Ok(DelegationResult {
            selected_specialists: vec![selected_specialist.node.name.clone()],
            reasoning: format!("Custom delegation using custom logic: {}", logic),
            outputs: vec![result.output],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::battalion::BattalionConfig;
    use crate::core::platform::container::paladin::{Paladin, PaladinData};
    use paladin_ports::output::paladin_port::PaladinResult;

    fn create_test_paladin(name: &str) -> Paladin {
        let data = PaladinData {
            system_prompt: format!("{} system prompt", name),
            name: name.to_string(),
            user_name: "test_user".to_string(),
            ..Default::default()
        };
        Paladin::new(data, Some(name.to_string()))
    }

    #[test]
    fn test_service_construction() {
        use async_trait::async_trait;
        use paladin_ports::output::paladin_port::StopReason;

        struct MockPort;

        #[async_trait]
        impl PaladinPort for MockPort {
            async fn execute(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<PaladinResult, crate::application::use_cases::paladin::error::PaladinError>
            {
                Ok(PaladinResult {
                    output: String::new(),
                    token_count: 0,
                    execution_time_ms: 0,
                    loop_count: 1,
                    stop_reason: StopReason::Completed,
                    ..Default::default()
                })
            }

            async fn execute_stream(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<
                paladin_ports::output::paladin_port::PaladinStream,
                crate::application::use_cases::paladin::error::PaladinError,
            > {
                unimplemented!()
            }

            fn validate(
                &self,
                _paladin: &Paladin,
            ) -> Result<(), crate::application::use_cases::paladin::error::PaladinError>
            {
                Ok(())
            }
        }

        let port = Arc::new(MockPort);
        let _service = ChainOfCommandExecutionService::new(port);
        // Service should be created successfully
    }

    #[tokio::test]
    async fn test_validate_valid_chain() {
        use async_trait::async_trait;
        use paladin_ports::output::paladin_port::StopReason;

        struct MockPort;

        #[async_trait]
        impl PaladinPort for MockPort {
            async fn execute(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<PaladinResult, crate::application::use_cases::paladin::error::PaladinError>
            {
                Ok(PaladinResult {
                    output: String::new(),
                    token_count: 0,
                    execution_time_ms: 0,
                    loop_count: 1,
                    stop_reason: StopReason::Completed,
                    ..Default::default()
                })
            }

            async fn execute_stream(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<
                paladin_ports::output::paladin_port::PaladinStream,
                crate::application::use_cases::paladin::error::PaladinError,
            > {
                unimplemented!()
            }

            fn validate(
                &self,
                _paladin: &Paladin,
            ) -> Result<(), crate::application::use_cases::paladin::error::PaladinError>
            {
                Ok(())
            }
        }

        let port = Arc::new(MockPort);
        let service = ChainOfCommandExecutionService::new(port);

        let commander = create_test_paladin("commander");
        let specialist = create_test_paladin("specialist");
        let config = BattalionConfig::default();

        let chain = ChainOfCommand::new(commander, vec![specialist], config)
            .expect("Should create valid chain");

        let result = service.validate(&chain).await;
        assert!(result.is_ok());
    }
}
