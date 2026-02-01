//! PlanningService - LLM-based autonomous task decomposition
//!
//! This service implements US-14.1: Autonomous Planning Mode.
//! When a Paladin is configured with `MaxLoops::Auto`, it uses this service
//! to decompose complex tasks into subtasks, execute them with dependency tracking,
//! and synthesize results into a cohesive response.
//!
//! # Examples
//!
//! ```rust,no_run
//! use paladin::application::use_cases::paladin::planning_service::PlanningService;
//! use paladin::application::ports::output::llm_port::LlmPort;
//! use std::sync::Arc;
//!
//! # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
//! let planning_service = PlanningService::new(llm_port);
//!
//! // Create and execute a plan
//! let plan = planning_service.create_plan(
//!     "Analyze the security vulnerabilities in this codebase",
//!     10, // max_subtasks
//! ).await?;
//!
//! let result = planning_service.execute_plan(&plan, "/* code here */").await?;
//! # Ok(())
//! # }
//! ```

use crate::application::errors::planning_error::PlanningError;
use crate::application::ports::output::llm_port::{LlmPort, LlmRequest};
use crate::core::platform::container::planning::{Subtask, TaskPlan};
use crate::core::platform::container::prompt::{PromptItem, PromptType, UserPrompt};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Service for LLM-based autonomous task planning and execution
///
/// Implements the planning mode where a Paladin decomposes complex tasks
/// into subtasks, manages their execution with dependency tracking, and
/// synthesizes results.
pub struct PlanningService {
    /// LLM port for task decomposition and synthesis
    llm_port: Arc<dyn LlmPort>,
}

/// Internal structure for deserializing LLM plan responses
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LlmPlanResponse {
    task: String,
    subtasks: Vec<LlmSubtask>,
}

/// Internal structure for deserializing subtasks from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LlmSubtask {
    id: String,
    description: String,
    dependencies: Vec<String>,
}

impl PlanningService {
    /// Creates a new PlanningService
    ///
    /// # Arguments
    ///
    /// * `llm_port` - LLM port for generating plans and synthesizing results
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paladin::application::use_cases::paladin::planning_service::PlanningService;
    /// use paladin::application::ports::output::llm_port::LlmPort;
    /// use std::sync::Arc;
    ///
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let service = PlanningService::new(llm_port);
    /// # }
    /// ```
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self {
        info!("Creating PlanningService");
        Self { llm_port }
    }

    /// Creates a task decomposition plan using LLM
    ///
    /// # Arguments
    ///
    /// * `task_description` - Description of the task to decompose
    /// * `max_subtasks` - Maximum number of subtasks allowed
    ///
    /// # Returns
    ///
    /// A `TaskPlan` containing the decomposed subtasks
    ///
    /// # Errors
    ///
    /// Returns `PlanningError` if:
    /// - LLM call fails
    /// - Response cannot be parsed
    /// - Plan exceeds max_subtasks limit
    /// - Plan has invalid dependencies
    pub async fn create_plan(
        &self,
        task_description: &str,
        max_subtasks: u32,
    ) -> Result<TaskPlan, PlanningError> {
        info!(
            "Creating plan for task: '{}' (max {} subtasks)",
            task_description, max_subtasks
        );

        // Build the planning prompt
        let prompt = self.build_planning_prompt(task_description, max_subtasks);

        // Call LLM
        let user_prompt = UserPrompt {
            query: prompt,
            context: None,
        };
        let prompt_item = PromptItem::new(PromptType::User(user_prompt))
            .map_err(|e| PlanningError::GenerationFailed(e.to_string()))?;

        let request = LlmRequest {
            id: Uuid::new_v4(),
            model: "gpt-4".to_string(), // TODO: Make configurable
            prompt: prompt_item,
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        };

        let response = self
            .llm_port
            .generate(request)
            .await
            .map_err(|e| PlanningError::LlmError(e.to_string()))?;

        // Parse the LLM response into a TaskPlan
        let plan = self.parse_plan_from_llm(&response.content, max_subtasks)?;

        info!("Created plan with {} subtasks", plan.subtask_count());
        Ok(plan)
    }

    /// Builds the planning prompt for the LLM
    fn build_planning_prompt(&self, task_description: &str, max_subtasks: u32) -> String {
        format!(
            r#"You are a task planning assistant. Decompose the following task into subtasks.

TASK: {}

INSTRUCTIONS:
- Break down the task into {} or fewer subtasks
- Each subtask should be concrete and actionable
- Identify dependencies between subtasks
- Return your response as JSON in the following format:

{{
  "task": "original task description",
  "subtasks": [
    {{
      "id": "1",
      "description": "description of subtask",
      "dependencies": ["id1", "id2"]
    }}
  ]
}}

Return ONLY the JSON, no additional text."#,
            task_description, max_subtasks
        )
    }

    /// Parses LLM response into a TaskPlan
    ///
    /// # Arguments
    ///
    /// * `llm_response` - The LLM's response content
    /// * `max_subtasks` - Maximum allowed subtasks
    ///
    /// # Returns
    ///
    /// A validated `TaskPlan`
    ///
    /// # Errors
    ///
    /// Returns `PlanningError` if parsing fails or plan is invalid
    fn parse_plan_from_llm(
        &self,
        llm_response: &str,
        max_subtasks: u32,
    ) -> Result<TaskPlan, PlanningError> {
        // Try to extract JSON from the response (LLM might add extra text)
        let json_str = self.extract_json(llm_response)?;

        // Parse JSON
        let llm_plan: LlmPlanResponse = serde_json::from_str(&json_str)
            .map_err(|e| PlanningError::GenerationFailed(format!("JSON parse error: {}", e)))?;

        // Validate subtask count
        if llm_plan.subtasks.len() as u32 > max_subtasks {
            return Err(PlanningError::MaxSubtasksExceeded {
                max: max_subtasks,
                attempted: llm_plan.subtasks.len() as u32,
            });
        }

        // Create TaskPlan
        let mut plan = TaskPlan::new(llm_plan.task, max_subtasks);

        // Add subtasks
        for llm_subtask in llm_plan.subtasks {
            let subtask = Subtask::new(
                llm_subtask.id.clone(),
                llm_subtask.description,
                "Expected output from subtask execution".to_string(), // TODO: Ask LLM for expected output
            );
            plan.add_subtask(subtask)
                .map_err(PlanningError::InvalidPlan)?;

            // Add dependencies if any
            if !llm_subtask.dependencies.is_empty() {
                plan.dependencies
                    .insert(llm_subtask.id, llm_subtask.dependencies);
            }
        }

        // Validate the plan (checks for circular dependencies, etc.)
        plan.validate().map_err(PlanningError::InvalidPlan)?;

        Ok(plan)
    }

    /// Extracts JSON from LLM response (handles markdown code blocks, etc.)
    fn extract_json(&self, response: &str) -> Result<String, PlanningError> {
        let trimmed = response.trim();

        // Check for markdown code block
        if let Some(start) = trimmed.find("```json")
            && let Some(end) = trimmed[start + 7..].find("```")
        {
            return Ok(trimmed[start + 7..start + 7 + end].trim().to_string());
        }

        // Check for plain code block
        if let Some(start) = trimmed.find("```")
            && let Some(end) = trimmed[start + 3..].find("```")
        {
            return Ok(trimmed[start + 3..start + 3 + end].trim().to_string());
        }

        // Assume the whole response is JSON
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::output::llm_port::{
        FinishReason, LlmError, LlmResponse, ProviderCapabilities, TokenUsage,
    };
    use async_trait::async_trait;
    use chrono::Utc;

    /// Mock LLM port for testing
    struct MockLlmPort {
        response: String,
    }

    impl MockLlmPort {
        fn new(response: impl Into<String>) -> Self {
            Self {
                response: response.into(),
            }
        }
    }

    #[async_trait]
    impl LlmPort for MockLlmPort {
        async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, LlmError> {
            Ok(LlmResponse {
                id: Uuid::new_v4(),
                request_id: Uuid::new_v4(),
                model: "test-model".to_string(),
                content: self.response.clone(),
                finish_reason: FinishReason::Stop,
                usage: TokenUsage {
                    prompt_tokens: 10,
                    completion_tokens: 20,
                    total_tokens: 30,
                },
                created_at: Utc::now(),
                metadata: HashMap::new(),
                function_call: None,
            })
        }

        async fn generate_stream(
            &self,
            _request: LlmRequest,
        ) -> Result<
            Box<
                dyn futures::Stream<
                        Item = Result<
                            crate::application::ports::output::llm_port::StreamingResponse,
                            LlmError,
                        >,
                    > + Send,
            >,
            LlmError,
        > {
            unimplemented!("Streaming not needed for tests")
        }

        async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
            Ok(true)
        }

        async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
            Ok(vec!["test-model".to_string()])
        }

        fn get_provider_name(&self) -> &'static str {
            "mock"
        }

        fn get_capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                supports_streaming: false,
                supports_function_calling: false,
                supports_tool_calling: false,
                supports_vision: false,
                supports_embeddings: false,
                supports_system_messages: true,
                max_context_tokens: Some(4096),
            }
        }
    }

    #[test]
    fn test_planning_service_new() {
        // Given: A mock LLM port
        let llm_port = Arc::new(MockLlmPort::new("test"));

        // When: Creating a new PlanningService
        let _service = PlanningService::new(llm_port.clone());

        // Then: The service should be created successfully
        // Verify the Arc has been cloned (service holds a reference)
        assert!(Arc::strong_count(&llm_port) >= 2);
    }

    #[tokio::test]
    async fn test_create_plan_basic() {
        // Given: A mock LLM that returns a valid plan
        let plan_json = r#"{
            "task": "Analyze security vulnerabilities",
            "subtasks": [
                {
                    "id": "1",
                    "description": "Scan for SQL injection vulnerabilities",
                    "dependencies": []
                },
                {
                    "id": "2",
                    "description": "Check for XSS vulnerabilities",
                    "dependencies": []
                },
                {
                    "id": "3",
                    "description": "Generate security report",
                    "dependencies": ["1", "2"]
                }
            ]
        }"#;

        let llm_port = Arc::new(MockLlmPort::new(plan_json));
        let service = PlanningService::new(llm_port);

        // When: Creating a plan
        let result = service
            .create_plan("Analyze security vulnerabilities", 10)
            .await;

        // Then: The plan should be created successfully
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.subtask_count(), 3);
    }

    #[tokio::test]
    async fn test_create_plan_enforces_max_subtasks() {
        // Given: A mock LLM that returns a plan with many subtasks
        let plan_json = r#"{
            "task": "Complex task",
            "subtasks": [
                {"id": "1", "description": "Task 1", "dependencies": []},
                {"id": "2", "description": "Task 2", "dependencies": []},
                {"id": "3", "description": "Task 3", "dependencies": []},
                {"id": "4", "description": "Task 4", "dependencies": []},
                {"id": "5", "description": "Task 5", "dependencies": []},
                {"id": "6", "description": "Task 6", "dependencies": []}
            ]
        }"#;

        let llm_port = Arc::new(MockLlmPort::new(plan_json));
        let service = PlanningService::new(llm_port);

        // When: Creating a plan with max_subtasks=3
        let result = service.create_plan("Complex task", 3).await;

        // Then: Should return error for exceeding limit
        assert!(result.is_err());
        if let Err(e) = result {
            // Check it's the right error type
            match e {
                PlanningError::MaxSubtasksExceeded { max, attempted } => {
                    assert_eq!(max, 3);
                    assert_eq!(attempted, 6);
                }
                other => panic!("Expected MaxSubtasksExceeded, got: {:?}", other),
            }
        }
    }
}
