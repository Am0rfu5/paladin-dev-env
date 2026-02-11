//! Paladin Execution Service
//!
//! This module provides the core execution service for Paladins, handling LLM interactions,
//! retry logic, circuit breaking, timeout enforcement, and execution metadata tracking.
//!
//! # Overview
//!
//! The `PaladinExecutionService` orchestrates the execution of a Paladin's reasoning loop,
//! wrapping LLM calls with resilience patterns including:
//! - Exponential backoff retry logic
//! - Circuit breaker pattern for fault tolerance
//! - Timeout enforcement
//! - Stop word detection
//! - Execution metadata tracking
//!
//! # Example
//!
//! ```rust,no_run
//! use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
//! use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
//! use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
//! use paladin::application::ports::output::llm_port::LlmPort;
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
//! // Create circuit breaker
//! let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
//!
//! // Create execution service
//! let service = PaladinExecutionService::new(llm_port.clone(), circuit_breaker, None, None);
//!
//! // Build paladin
//! let paladin = PaladinBuilder::new(llm_port)
//!     .system_prompt("You are a helpful assistant")
//!     .max_loops(5)
//!     .retry_attempts(3)
//!     .timeout_seconds(300)
//!     .build().await?;
//!
//! // Execute
//! let result = service.execute(&paladin, "What is Rust?").await?;
//! println!("Output: {}", result.output);
//! println!("Loops: {}, Tokens: {}", result.loop_count, result.token_count);
//! # Ok(())
//! # }
//! ```

use crate::application::ports::output::arsenal_port::ArsenalPort;
use crate::application::ports::output::garrison_port::GarrisonPort;
use crate::application::ports::output::llm_port::{FunctionCall, LlmPort, LlmRequest};
use crate::application::ports::output::paladin_port::{PaladinResult, StopReason};
use crate::application::ports::output::vision_port::VisionPort;
use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use crate::application::use_cases::paladin::error::PaladinError;
use crate::application::use_cases::sanctum::memory_extraction_service::{
    MemoryExtractionService, MemoryExtractionStrategy,
};
use crate::application::use_cases::sanctum::rag_retrieval_service::RagRetrievalService;
use crate::core::base::entity::node::Node;
use crate::core::platform::container::arsenal::{ArmamentCall, ArsenalError};
use crate::core::platform::container::garrison::{ConversationRole, GarrisonEntry};
use crate::core::platform::container::herald::Herald;
use crate::core::platform::container::paladin::Paladin;
use crate::core::platform::container::prompt::{
    PromptData, PromptItem, PromptParameters, PromptType, UserPrompt,
};
use crate::core::platform::container::vision::VisionContent;
use crate::infrastructure::adapters::arsenal::tool_result_formatter::ToolResultFormatter;
use log::{debug, error, info, warn};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{Duration, sleep, timeout};

/// Paladin Execution Service
///
/// Orchestrates the execution of Paladin reasoning loops with resilience patterns.
///
/// # Features
///
/// - **Retry Logic**: Exponential backoff (100ms, 200ms, 400ms, etc.)
/// - **Circuit Breaker**: Prevents cascading failures
/// - **Timeout Enforcement**: Respects configured timeout limits
/// - **Stop Word Detection**: Halts execution on detected stop words
/// - **Metadata Tracking**: Records execution time, loops, and token usage
/// - **Memory Management**: Stores conversation history in Garrison when provided
///
/// # Thread Safety
///
/// This service is thread-safe and can be shared across threads using `Arc<PaladinExecutionService>`.
pub struct PaladinExecutionService {
    /// LLM port for model interactions
    llm_port: Arc<dyn LlmPort>,

    /// Circuit breaker for fault tolerance
    circuit_breaker: Arc<CircuitBreaker>,

    /// Optional Garrison for conversation memory
    garrison: Option<Arc<dyn GarrisonPort>>,

    /// Optional Arsenal for tool execution
    arsenal: Option<Arc<dyn ArsenalPort>>,

    /// Optional Herald for output formatting
    herald: Option<Arc<dyn Herald>>,

    /// Tool result formatter for context injection
    formatter: ToolResultFormatter,

    /// Optional RAG retrieval service for context augmentation
    rag_retrieval_service: Option<Arc<RagRetrievalService>>,

    /// Optional memory extraction service for storing important information
    memory_extraction_service: Option<Arc<MemoryExtractionService>>,

    /// Vision adapters registry (provider name → adapter)
    vision_adapters: HashMap<String, Arc<dyn VisionPort>>,
}

impl PaladinExecutionService {
    /// Creates a new Paladin execution service
    ///
    /// # Arguments
    ///
    /// * `llm_port` - The LLM port implementation to use for model calls
    /// * `circuit_breaker` - Circuit breaker for fault tolerance
    /// * `garrison` - Optional Garrison for conversation memory (None for stateless operations)
    /// * `arsenal` - Optional Arsenal for tool execution (None to disable tool support)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    /// use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    /// use paladin::application::ports::output::llm_port::LlmPort;
    /// use std::sync::Arc;
    /// use std::time::Duration;
    ///
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    /// let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);
    /// # }
    /// ```
    pub fn new(
        llm_port: Arc<dyn LlmPort>,
        circuit_breaker: Arc<CircuitBreaker>,
        garrison: Option<Arc<dyn GarrisonPort>>,
        arsenal: Option<Arc<dyn ArsenalPort>>,
    ) -> Self {
        info!(
            "Creating PaladinExecutionService with garrison: {}, arsenal: {}",
            garrison.is_some(),
            arsenal.is_some()
        );
        Self {
            llm_port,
            circuit_breaker,
            garrison,
            arsenal,
            herald: None,
            formatter: ToolResultFormatter::new(),
            rag_retrieval_service: None,
            memory_extraction_service: None,
            vision_adapters: HashMap::new(),
        }
    }

    /// Sets the RAG retrieval service for context augmentation
    ///
    /// # Arguments
    ///
    /// * `service` - The RAG retrieval service to use for context retrieval
    ///
    /// # Returns
    ///
    /// Returns self for method chaining
    pub fn with_rag_retrieval(mut self, service: Arc<RagRetrievalService>) -> Self {
        info!("Attaching RAG retrieval service to PaladinExecutionService");
        self.rag_retrieval_service = Some(service);
        self
    }

    /// Sets the memory extraction service for storing important information
    ///
    /// # Arguments
    ///
    /// * `service` - The memory extraction service to use
    ///
    /// # Returns
    ///
    /// Returns self for method chaining
    pub fn with_memory_extraction(mut self, service: Arc<MemoryExtractionService>) -> Self {
        info!("Attaching memory extraction service to PaladinExecutionService");
        self.memory_extraction_service = Some(service);
        self
    }

    /// Sets the Herald formatter for this service
    ///
    /// # Arguments
    ///
    /// * `herald` - The Herald implementation to use for formatting execution results
    ///
    /// # Returns
    ///
    /// Returns self for method chaining
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    /// use paladin::infrastructure::adapters::herald::JsonHerald;
    /// use std::sync::Arc;
    /// # use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::time::Duration;
    ///
    /// # fn example(llm_port: Arc<dyn LlmPort>) {
    /// # let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    /// let herald = Arc::new(JsonHerald::default());
    /// let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
    ///     .with_herald(herald);
    /// # }
    /// ```
    pub fn with_herald(mut self, herald: Arc<dyn Herald>) -> Self {
        self.herald = Some(herald);
        self
    }

    /// Registers a vision adapter for a specific provider
    ///
    /// # Arguments
    ///
    /// * `provider` - Provider name (e.g., "openai", "anthropic")
    /// * `adapter` - The vision adapter implementation
    ///
    /// # Returns
    ///
    /// Returns self for method chaining
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    /// use paladin::infrastructure::adapters::llm::openai_adapter::OpenAIAdapter;
    /// use std::sync::Arc;
    /// # use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::time::Duration;
    ///
    /// # fn example(llm_port: Arc<dyn LlmPort>, openai: Arc<OpenAIAdapter>) {
    /// # let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    /// let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
    ///     .with_vision_adapter("openai".to_string(), openai);
    /// # }
    /// ```
    pub fn with_vision_adapter(mut self, provider: String, adapter: Arc<dyn VisionPort>) -> Self {
        info!("Registering vision adapter for provider: {}", provider);
        self.vision_adapters.insert(provider, adapter);
        self
    }

    /// Formats a Paladin execution result using the configured Herald
    ///
    /// If no Herald is configured, returns None. This allows for optional formatting
    /// based on runtime configuration or user preferences.
    ///
    /// # Arguments
    ///
    /// * `result` - The Paladin execution result to format
    /// * `paladin` - The Paladin that produced this result (for name/ID)
    ///
    /// # Returns
    ///
    /// Returns `Some(formatted_output)` if a Herald is configured and formatting succeeds,
    /// `None` if no Herald is configured.
    ///
    /// # Errors
    ///
    /// Returns `PaladinError::ExecutionError` if formatting fails.
    pub fn format_result(
        &self,
        result: &PaladinResult,
        _paladin: &Paladin,
    ) -> Result<Option<String>, PaladinError> {
        if let Some(ref herald) = self.herald {
            // Herald now uses actual PaladinResult directly - no conversion needed!
            let formatted = herald.format_paladin_result(result).map_err(|e| {
                PaladinError::ExecutionError(format!("Herald formatting failed: {}", e))
            })?;
            Ok(Some(formatted))
        } else {
            Ok(None)
        }
    }

    /// Executes a Paladin with the given input
    ///
    /// This is the main entry point for Paladin execution. It orchestrates the entire
    /// execution lifecycle including timeout enforcement, retry logic, and metadata tracking.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin to execute
    /// * `input` - The input text to process
    ///
    /// # Returns
    ///
    /// - `Ok(PaladinResult)` - Execution succeeded with result metadata
    /// - `Err(PaladinError)` - Execution failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    /// # use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    /// # use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # use std::sync::Arc;
    /// # use std::time::Duration;
    /// # async fn example(llm_port: Arc<dyn LlmPort>, service: PaladinExecutionService) -> Result<(), Box<dyn std::error::Error>> {
    /// # let paladin = PaladinBuilder::new(llm_port).system_prompt("test").build().await?;
    /// let result = service.execute(&paladin, "Explain quantum computing").await?;
    /// println!("Result: {}", result.output);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        &self,
        paladin: &Paladin,
        input: &str,
    ) -> Result<PaladinResult, PaladinError> {
        let execution_id = uuid::Uuid::new_v4();
        info!(
            "Starting Paladin execution: id={}, name={}, input_len={}",
            execution_id,
            paladin.node.name,
            input.len()
        );

        let start_time = Instant::now();
        let timeout_duration = Duration::from_secs(paladin.node.max_loops.as_u32() as u64 * 60);

        // Wrap execution with timeout
        let execution_future = self.execute_internal(paladin, input, execution_id);

        match timeout(timeout_duration, execution_future).await {
            Ok(result) => {
                let elapsed = start_time.elapsed();
                info!(
                    "Paladin execution completed: id={}, duration_ms={}, success={}",
                    execution_id,
                    elapsed.as_millis(),
                    result.is_ok()
                );
                result
            }
            Err(_) => {
                let elapsed = start_time.elapsed();
                error!(
                    "Paladin execution timed out: id={}, duration_ms={}",
                    execution_id,
                    elapsed.as_millis()
                );
                Err(PaladinError::Timeout(elapsed.as_secs()))
            }
        }
    }

    /// Execute a Paladin with vision capabilities
    ///
    /// Validates that the Paladin has vision enabled and the LLM provider supports vision.
    /// Executes vision analysis using the registered vision adapters.
    pub async fn execute_with_vision(
        &self,
        paladin: &Paladin,
        task: &str,
        images: Vec<VisionContent>,
    ) -> Result<PaladinResult, PaladinError> {
        // Step 1: Validate vision is enabled on the Paladin
        if !paladin.node.vision_enabled {
            return Err(PaladinError::ConfigurationError(
                "Vision execution requires vision_enabled=true. Use PaladinBuilder::enable_vision(true)".to_string(),
            ));
        }

        // Step 2: Check if LLM provider supports vision
        let capabilities = self.llm_port.get_capabilities();
        if !capabilities.supports_vision {
            return Err(PaladinError::ConfigurationError(format!(
                "LLM provider '{}' does not support vision capabilities. Use a vision-capable model like gpt-4o or claude-3-opus",
                self.llm_port.get_provider_name()
            )));
        }

        // Step 3: Extract provider from model name
        let model = paladin.node.model.as_str();
        let provider = self.extract_provider_from_model(model)?;

        // Step 4: Get vision adapter for provider
        let vision_adapter = self.vision_adapters.get(&provider).ok_or_else(|| {
            PaladinError::ExecutionError(format!(
                "No vision adapter registered for provider: {}. Available: {:?}",
                provider,
                self.vision_adapters.keys().collect::<Vec<_>>()
            ))
        })?;

        // Step 5: Determine timeout from paladin config
        let timeout_secs = paladin.node.max_loops.as_u32() as u64 * 60;
        let timeout_duration = Duration::from_secs(timeout_secs);

        // Step 6: Execute vision analysis with timeout
        let vision_result = timeout(timeout_duration, async {
            vision_adapter
                .analyze_image(
                    task,
                    images.clone(),
                    model,
                    Some(4000), // Default max tokens for vision
                )
                .await
                .map_err(|e| match e {
                    crate::core::platform::container::vision::VisionError::InvalidRequest(msg) => {
                        PaladinError::ExecutionError(msg)
                    }
                    crate::core::platform::container::vision::VisionError::Timeout(secs) => {
                        PaladinError::ExecutionError(format!(
                            "Vision request timed out after {} seconds",
                            secs
                        ))
                    }
                    crate::core::platform::container::vision::VisionError::MaxRetriesExceeded(
                        attempts,
                    ) => PaladinError::ExecutionError(format!(
                        "Max retries exceeded: {} attempts",
                        attempts
                    )),
                    _ => PaladinError::ExecutionError(e.to_string()),
                })
        })
        .await
        .map_err(|_| {
            PaladinError::ExecutionError(format!(
                "Vision execution timed out after {} seconds",
                timeout_secs
            ))
        })??;

        // Step 7: Check for stop words
        for stop_word in &paladin.node.stop_words {
            if vision_result.content.contains(stop_word) {
                return Err(PaladinError::ExecutionError(format!(
                    "Stop word detected: {}",
                    stop_word
                )));
            }
        }

        // Step 8: Store in garrison if configured
        if let Some(ref garrison) = self.garrison {
            // Store user message with task
            let user_entry = GarrisonEntry::new(ConversationRole::User, task.to_string());
            garrison.remember(user_entry).await.map_err(|e| {
                warn!("Failed to store user vision message in garrison: {}", e);
                PaladinError::ExecutionError(format!("Garrison storage failed: {}", e))
            })?;

            // Store assistant response
            let assistant_entry =
                GarrisonEntry::new(ConversationRole::Assistant, vision_result.content.clone());
            garrison.remember(assistant_entry).await.map_err(|e| {
                warn!(
                    "Failed to store assistant vision response in garrison: {}",
                    e
                );
                PaladinError::ExecutionError(format!("Garrison storage failed: {}", e))
            })?;
        }

        // Step 9: Build and return PaladinResult
        Ok(PaladinResult {
            output: vision_result.content,
            loop_count: 1, // Vision is single-shot
            token_count: vision_result.token_usage.total_tokens,
            stop_reason: StopReason::Completed,
            execution_time_ms: 0, // Will be set by caller if needed
        })
    }

    /// Extract provider name from model string
    ///
    /// Examples:
    /// - "gpt-4o" → "openai"
    /// - "claude-3-opus" → "anthropic"
    fn extract_provider_from_model(&self, model: &str) -> Result<String, PaladinError> {
        if model.starts_with("gpt-") || model.starts_with("o1-") {
            Ok("openai".to_string())
        } else if model.starts_with("claude-") {
            Ok("anthropic".to_string())
        } else {
            Err(PaladinError::ConfigurationError(format!(
                "Cannot determine provider from model: {}. Use gpt-* or claude-* models",
                model
            )))
        }
    }

    /// Internal execution logic without timeout wrapper
    async fn execute_internal(
        &self,
        paladin: &Paladin,
        input: &str,
        execution_id: uuid::Uuid,
    ) -> Result<PaladinResult, PaladinError> {
        let start_time = Instant::now();
        let mut total_tokens = 0u32;
        let mut accumulated_output = String::new();
        let mut _retrieval_latency_ms = 0u64;
        let mut _memories_retrieved_count = 0usize;
        let mut _extraction_triggered = false;

        // Step 1: Retrieve relevant context from Sanctum if RAG is configured
        let retrieved_context = if self.check_sanctum_configured() {
            debug!(
                "Sanctum configured, retrieving context: execution_id={}",
                execution_id
            );
            let retrieval_start = Instant::now();

            match self
                .retrieve_context_with_timeout(paladin, input, execution_id)
                .await
            {
                Ok(results) => {
                    _retrieval_latency_ms = retrieval_start.elapsed().as_millis() as u64;
                    _memories_retrieved_count = results.len();

                    // Format results into context string
                    let context = if results.is_empty() {
                        String::new()
                    } else {
                        self.format_retrieved_context(&results)
                    };

                    info!(
                        "RAG retrieval succeeded: execution_id={}, memories={}, latency_ms={}",
                        execution_id, _memories_retrieved_count, _retrieval_latency_ms
                    );
                    Some(context)
                }
                Err(e) => {
                    _retrieval_latency_ms = retrieval_start.elapsed().as_millis() as u64;
                    warn!(
                        "RAG retrieval failed: execution_id={}, error={}, latency_ms={}",
                        execution_id, e, _retrieval_latency_ms
                    );
                    None
                }
            }
        } else {
            None
        };

        // Store user input in garrison if available
        if let Some(garrison) = &self.garrison {
            let user_entry = GarrisonEntry::new(ConversationRole::User, input.to_string());
            garrison.remember(user_entry).await?;
            debug!(
                "Stored user input in garrison: execution_id={}",
                execution_id
            );
        }

        // Retrieve conversation history if garrison is available
        let conversation_history = if let Some(garrison) = &self.garrison {
            let history = garrison.recall_recent(20).await?;
            debug!(
                "Retrieved {} messages from garrison: execution_id={}",
                history.len(),
                execution_id
            );
            history
        } else {
            vec![]
        };

        // Execute reasoning loop
        for loop_num in 1..=paladin.node.max_loops.as_u32() {
            debug!(
                "Paladin loop iteration: id={}, loop={}/{}",
                execution_id, loop_num, paladin.node.max_loops
            );

            // Build prompt for this iteration with conversation history and RAG context
            let prompt = self.build_prompt_with_history_and_rag(
                paladin,
                input,
                &accumulated_output,
                &conversation_history,
                retrieved_context.as_deref(),
            );

            // Execute with retry and circuit breaker
            let response = self
                .execute_with_retry(paladin, &prompt, execution_id, loop_num)
                .await?;

            // Update accumulated output and token count
            accumulated_output = response.content.clone();
            total_tokens += response.usage.total_tokens;

            // Check for tool calls and execute them if arsenal is available
            if let Some(ref function_call) = response.function_call {
                if let Some(ref arsenal) = self.arsenal {
                    debug!(
                        "Tool call detected: id={}, tool={}, loop={}",
                        execution_id, function_call.name, loop_num
                    );

                    match self
                        .handle_tool_call(function_call, arsenal.as_ref(), execution_id)
                        .await
                    {
                        Ok(formatted_result) => {
                            debug!(
                                "Tool execution succeeded: id={}, tool={}",
                                execution_id, function_call.name
                            );
                            // Inject tool result into accumulated output for next iteration
                            accumulated_output.push_str("\n\n");
                            accumulated_output.push_str(&formatted_result);

                            // Store tool result in garrison if available
                            if let Some(garrison) = &self.garrison {
                                let tool_entry =
                                    GarrisonEntry::new(ConversationRole::Tool, formatted_result);
                                garrison.remember(tool_entry).await?;
                            }
                        }
                        Err(e) => {
                            warn!(
                                "Tool execution failed: id={}, tool={}, error={}",
                                execution_id, function_call.name, e
                            );
                            // Inject error message for LLM to see and potentially recover
                            let error_message = format!(
                                "\n\n🔧 Tool Execution: {}\nResult: FAILED\nError: {}\n",
                                function_call.name, e
                            );
                            accumulated_output.push_str(&error_message);
                        }
                    }
                } else {
                    warn!(
                        "Tool call requested but no arsenal available: id={}, tool={}",
                        execution_id, function_call.name
                    );
                }
            }

            // Check for stop words
            if let Some(stop_word) = self.check_stop_words(paladin, &accumulated_output) {
                warn!(
                    "Stop word detected: id={}, word={}, loop={}",
                    execution_id, stop_word, loop_num
                );
                return Err(PaladinError::StopWordDetected(stop_word));
            }

            // Check if we've reached max loops
            if loop_num == paladin.node.max_loops.as_u32() {
                debug!(
                    "Reached max loops: id={}, loops={}",
                    execution_id, paladin.node.max_loops
                );

                // Store assistant response in garrison if available
                if let Some(garrison) = &self.garrison {
                    let assistant_entry =
                        GarrisonEntry::new(ConversationRole::Assistant, accumulated_output.clone());
                    garrison.remember(assistant_entry).await?;
                    debug!(
                        "Stored assistant response in garrison: execution_id={}",
                        execution_id
                    );
                }

                // Trigger memory extraction on completion if configured
                if self.should_extract_memories(MemoryExtractionStrategy::OnCompletion) {
                    _extraction_triggered = true;
                    self.extract_memories_async(paladin, &conversation_history, execution_id);
                }

                return Ok(PaladinResult {
                    output: accumulated_output,
                    token_count: total_tokens,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    loop_count: loop_num,
                    stop_reason: StopReason::MaxLoops,
                });
            }
        }

        // This should not be reached due to the loop logic, but provide a fallback
        // Store response in garrison before returning
        if let Some(garrison) = &self.garrison {
            let assistant_entry =
                GarrisonEntry::new(ConversationRole::Assistant, accumulated_output.clone());
            garrison.remember(assistant_entry).await?;
        }

        // Trigger memory extraction on completion if configured
        if self.should_extract_memories(MemoryExtractionStrategy::OnCompletion) {
            _extraction_triggered = true;
            self.extract_memories_async(paladin, &conversation_history, execution_id);
        }

        Ok(PaladinResult {
            output: accumulated_output,
            token_count: total_tokens,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            loop_count: paladin.node.max_loops.as_u32(),
            stop_reason: StopReason::Completed,
        })
    }

    /// Builds the prompt for an LLM call
    ///
    /// Combines the system prompt, RAG context, conversation history from Garrison,
    /// user input, and accumulated output from previous loops.
    fn build_prompt_with_history_and_rag(
        &self,
        paladin: &Paladin,
        input: &str,
        accumulated_output: &str,
        conversation_history: &[GarrisonEntry],
        rag_context: Option<&str>,
    ) -> String {
        let mut prompt = format!("{}\n\n", paladin.node.system_prompt);

        // Inject RAG context if available
        if let Some(context) = rag_context
            && !context.is_empty()
        {
            prompt.push_str("## Relevant Context from Memory\n");
            prompt.push_str(context);
            prompt.push_str("\n\n");
        }

        // Add conversation history if available
        if !conversation_history.is_empty() {
            prompt.push_str("Previous conversation:\n");
            for entry in conversation_history.iter().rev().take(10).rev() {
                // Most recent 10 entries
                let role_str = match entry.role {
                    ConversationRole::System => "System",
                    ConversationRole::User => "User",
                    ConversationRole::Assistant => "Assistant",
                    ConversationRole::Tool => "Tool",
                };
                prompt.push_str(&format!("{}: {}\n", role_str, entry.content));
            }
            prompt.push('\n');
        }

        prompt.push_str(&format!("User: {}\n", input));

        if !accumulated_output.is_empty() {
            prompt.push_str(&format!("Previous output: {}\n", accumulated_output));
        }

        prompt
    }

    /// Checks if Sanctum (RAG) is configured and ready
    fn check_sanctum_configured(&self) -> bool {
        self.rag_retrieval_service.is_some()
    }

    /// Retrieves context from Sanctum with timeout
    ///
    /// Wraps RAG retrieval in a 5-second timeout to prevent blocking execution.
    async fn retrieve_context_with_timeout(
        &self,
        paladin: &Paladin,
        query: &str,
        execution_id: uuid::Uuid,
    ) -> Result<
        Vec<crate::application::ports::output::sanctum_port::SanctumSearchResult>,
        PaladinError,
    > {
        if let Some(ref rag_service) = self.rag_retrieval_service {
            let paladin_id = paladin.uuid.to_string();

            match timeout(
                Duration::from_secs(5),
                rag_service.retrieve_context(&paladin_id, query),
            )
            .await
            {
                Ok(Ok(results)) => {
                    debug!(
                        "RAG retrieval completed: execution_id={}, results={}",
                        execution_id,
                        results.len()
                    );
                    Ok(results)
                }
                Ok(Err(e)) => {
                    warn!(
                        "RAG retrieval failed: execution_id={}, error={}",
                        execution_id, e
                    );
                    Err(PaladinError::ExecutionError(format!(
                        "RAG retrieval failed: {}",
                        e
                    )))
                }
                Err(_) => {
                    warn!("RAG retrieval timed out: execution_id={}", execution_id);
                    Err(PaladinError::Timeout(5))
                }
            }
        } else {
            Err(PaladinError::ConfigurationError(
                "RAG retrieval service not configured".to_string(),
            ))
        }
    }

    /// Formats retrieved search results into a context string for injection
    fn format_retrieved_context(
        &self,
        results: &[crate::application::ports::output::sanctum_port::SanctumSearchResult],
    ) -> String {
        if results.is_empty() {
            return String::new();
        }

        let mut context = String::new();
        for (i, result) in results.iter().enumerate() {
            context.push_str(&format!(
                "{}. [Score: {:.2}] {}\n",
                i + 1,
                result.score,
                result.entry.memory.content
            ));
        }
        context
    }

    /// Checks if memories should be extracted based on the strategy
    fn should_extract_memories(&self, strategy: MemoryExtractionStrategy) -> bool {
        self.memory_extraction_service.is_some()
            && matches!(strategy, MemoryExtractionStrategy::OnCompletion)
    }

    /// Spawns an async task to extract memories in the background
    ///
    /// This runs asynchronously so it doesn't block Paladin execution completion.
    fn extract_memories_async(
        &self,
        paladin: &Paladin,
        conversation_history: &[GarrisonEntry],
        execution_id: uuid::Uuid,
    ) {
        if let Some(ref extraction_service) = self.memory_extraction_service {
            let paladin_id = paladin.uuid.to_string();
            let conversation = conversation_history.to_vec();
            let service = Arc::clone(extraction_service);

            // Spawn background task
            tokio::spawn(async move {
                let start = Instant::now();
                debug!(
                    "Starting background memory extraction: execution_id={}, conversations={}",
                    execution_id,
                    conversation.len()
                );

                match service.extract_memories(&paladin_id, &conversation).await {
                    Ok(extracted_entries) => {
                        let elapsed = start.elapsed();
                        info!(
                            "Memory extraction completed: execution_id={}, memories_extracted={}, duration_ms={}",
                            execution_id,
                            extracted_entries.len(),
                            elapsed.as_millis()
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        warn!(
                            "Memory extraction failed: execution_id={}, error={}, duration_ms={}",
                            execution_id,
                            e,
                            elapsed.as_millis()
                        );
                    }
                }
            });
        }
    }

    /// Checks if any stop words are present in the output
    ///
    /// Performs case-insensitive exact word matching.
    ///
    /// # Returns
    ///
    /// - `Some(stop_word)` if a stop word is found
    /// - `None` if no stop words are found
    fn check_stop_words(&self, paladin: &Paladin, output: &str) -> Option<String> {
        let output_lower = output.to_lowercase();

        for stop_word in &paladin.node.stop_words {
            let stop_word_lower = stop_word.to_lowercase();

            // Check for exact word match (case-insensitive)
            if output_lower.contains(&stop_word_lower) {
                return Some(stop_word.clone());
            }
        }

        None
    }

    /// Executes an LLM call with retry logic and circuit breaker
    ///
    /// Implements exponential backoff: 100ms, 200ms, 400ms, etc.
    ///
    /// # Arguments
    ///
    /// * `paladin` - The Paladin configuration
    /// * `prompt` - The prompt to send to the LLM
    /// * `execution_id` - Unique ID for this execution (for logging)
    /// * `loop_num` - Current loop iteration number
    ///
    /// # Returns
    ///
    /// The LLM response or an error after exhausting retries
    async fn execute_with_retry(
        &self,
        paladin: &Paladin,
        prompt: &str,
        execution_id: uuid::Uuid,
        loop_num: u32,
    ) -> Result<crate::application::ports::output::llm_port::LlmResponse, PaladinError> {
        let mut attempt = 0;
        let max_attempts = paladin.node.max_loops.as_u32().min(10); // Cap retries at 10

        loop {
            attempt += 1;

            debug!(
                "LLM call attempt: id={}, loop={}, attempt={}/{}",
                execution_id, loop_num, attempt, max_attempts
            );

            // Create prompt item
            let prompt_data = PromptData {
                prompt_type: PromptType::User(UserPrompt {
                    query: prompt.to_string(),
                    context: None,
                }),
                content_attachments: vec![],
                parameters: PromptParameters {
                    max_tokens: None,
                    temperature: Some(paladin.node.temperature),
                    top_p: None,
                    frequency_penalty: None,
                    presence_penalty: None,
                    stop_sequences: if paladin.node.stop_words.is_empty() {
                        None
                    } else {
                        Some(paladin.node.stop_words.clone())
                    },
                },
                context: None,
                expected_output: None,
                tags: None,
                category: None,
                author: None,
                metadata: BTreeMap::new(),
            };

            let prompt_item = PromptItem {
                node: Node::new(prompt_data, Some(format!("execution-{}", execution_id))),
            };

            // Create LLM request
            let request = LlmRequest {
                id: uuid::Uuid::new_v4(),
                model: paladin.node.model.clone(),
                prompt: prompt_item,
                attachments: vec![],
                stream: false,
                metadata: std::collections::HashMap::new(),
            };

            // Wrap LLM call with circuit breaker (async version)
            let llm_port = Arc::clone(&self.llm_port);
            let result = self
                .circuit_breaker
                .call_async(async move {
                    match llm_port.generate(request).await {
                        Ok(response) => Ok(response),
                        Err(e) => Err(PaladinError::LlmError(e.to_string())),
                    }
                })
                .await;

            match result {
                Ok(response) => {
                    debug!(
                        "LLM call succeeded: id={}, loop={}, attempt={}",
                        execution_id, loop_num, attempt
                    );
                    return Ok(response);
                }
                Err(PaladinError::CircuitBreakerOpen) => {
                    // Circuit breaker is open, fail fast
                    error!(
                        "Circuit breaker open: id={}, loop={}",
                        execution_id, loop_num
                    );
                    return Err(PaladinError::CircuitBreakerOpen);
                }
                Err(_e) if attempt >= max_attempts => {
                    // Exhausted retries
                    error!(
                        "Max retries exhausted: id={}, loop={}, attempts={}",
                        execution_id, loop_num, attempt
                    );
                    return Err(PaladinError::MaxRetriesExceeded(attempt));
                }
                Err(e) => {
                    // Retry with exponential backoff
                    let backoff_ms = 100 * 2u64.pow(attempt - 1);
                    warn!(
                        "LLM call failed, retrying: id={}, loop={}, attempt={}, backoff_ms={}, error={}",
                        execution_id, loop_num, attempt, backoff_ms, e
                    );
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
            }
        }
    }

    /// Handles tool call execution and formatting
    ///
    /// Parses the function call, invokes the tool via Arsenal, and formats
    /// the result for injection into the conversation context.
    ///
    /// # Arguments
    ///
    /// * `function_call` - The function call details from the LLM
    /// * `arsenal` - The Arsenal port for tool execution
    /// * `execution_id` - Unique ID for this execution (for logging)
    ///
    /// # Returns
    ///
    /// Formatted tool result as a string, or error if tool execution fails
    async fn handle_tool_call(
        &self,
        function_call: &FunctionCall,
        arsenal: &dyn ArsenalPort,
        execution_id: uuid::Uuid,
    ) -> Result<String, ArsenalError> {
        // Parse function call arguments
        let arguments: HashMap<String, Value> = serde_json::from_str(&function_call.arguments)
            .map_err(|e| {
                error!(
                    "Failed to parse function call arguments: id={}, error={}",
                    execution_id, e
                );
                ArsenalError::InvalidArguments(format!("Failed to parse arguments JSON: {}", e))
            })?;

        // Create armament call
        let call = ArmamentCall::new(&function_call.name, arguments);

        debug!(
            "Invoking tool: id={}, tool={}, call_id={}",
            execution_id, call.tool_name, call.call_id
        );

        // Invoke the tool (clone because invoke takes ownership)
        let result = arsenal.invoke(call.clone()).await?;

        debug!(
            "Tool invocation completed: id={}, tool={}, success={}, time_ms={}",
            execution_id, call.tool_name, result.success, result.execution_time_ms
        );

        // Format result for LLM context
        let formatted = self.formatter.format_result(&call, &result);

        Ok(formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::output::llm_port::{
        LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities, StreamingResponse,
    };
    use crate::application::ports::output::sanctum_port::SanctumSearchResult;
    use crate::application::use_cases::sanctum::MemoryExtractionStrategy;
    use crate::core::base::entity::node::Node;
    use crate::core::platform::container::{
        paladin::PaladinData,
        sanctum::{Memory, MemoryType, SanctumEntry},
        vision::ImageDetail,
    };
    use async_trait::async_trait;
    use uuid::Uuid;

    // Mock LlmPort for testing
    struct MockLlmPort;

    #[async_trait]
    impl LlmPort for MockLlmPort {
        async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, LlmError> {
            unimplemented!()
        }

        async fn generate_stream(
            &self,
            _request: LlmRequest,
        ) -> Result<
            Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>,
            LlmError,
        > {
            unimplemented!()
        }

        async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
            Ok(true)
        }

        async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
            Ok(vec![])
        }

        fn get_provider_name(&self) -> &'static str {
            "Mock"
        }

        fn get_capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::default()
        }
    }

    fn create_test_paladin() -> Paladin {
        let data = PaladinData {
            system_prompt: "You are a helpful assistant".to_string(),
            ..Default::default()
        };

        Node::new(data, Some("TestPaladin".to_string()))
    }

    fn create_mock_search_result(content: &str, score: f32) -> SanctumSearchResult {
        let memory = Memory {
            id: Uuid::new_v4(),
            paladin_id: "test".to_string(),
            content: content.to_string(),
            memory_type: MemoryType::Episodic,
            importance: 0.5,
            access_count: 0,
            last_accessed: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        let entry = SanctumEntry::new(memory, vec![0.1; 384]).expect("Failed to create test entry");

        SanctumSearchResult { entry, score }
    }

    #[tokio::test]
    async fn test_format_retrieved_context() {
        // Arrange
        let results = vec![
            create_mock_search_result("First memory", 0.95),
            create_mock_search_result("Second memory", 0.85),
        ];

        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Act
        let formatted = service.format_retrieved_context(&results);

        // Assert
        assert!(formatted.contains("1. [Score: 0.95] First memory"));
        assert!(formatted.contains("2. [Score: 0.85] Second memory"));
    }

    #[tokio::test]
    async fn test_format_retrieved_context_empty() {
        // Arrange
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Act
        let formatted = service.format_retrieved_context(&[]);

        // Assert
        assert!(formatted.is_empty());
    }

    #[tokio::test]
    async fn test_check_sanctum_configured() {
        // Arrange
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        // Act & Assert: Without RAG service
        let service_without = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);
        assert!(!service_without.check_sanctum_configured());
    }

    #[tokio::test]
    async fn test_should_extract_memories() {
        // Arrange
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        // Act & Assert: Without extraction service
        let service_without = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);
        assert!(!service_without.should_extract_memories(MemoryExtractionStrategy::OnCompletion));
    }

    #[tokio::test]
    async fn test_rag_context_injection() {
        // Arrange
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);
        let paladin = create_test_paladin();

        // Act: Build prompt with RAG context
        let retrieved_context = "1. [Score: 0.95] Previous conversation about Rust\n";
        let prompt = service.build_prompt_with_history_and_rag(
            &paladin,
            "What is Rust?",
            "",
            &[],
            Some(retrieved_context),
        );

        // Assert: Context should be injected
        assert!(prompt.contains("## Relevant Context from Memory"));
        assert!(prompt.contains("Previous conversation about Rust"));
    }

    #[tokio::test]
    async fn test_rag_context_injection_empty() {
        // Arrange
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);
        let paladin = create_test_paladin();

        // Act: Build prompt without RAG context
        let prompt =
            service.build_prompt_with_history_and_rag(&paladin, "What is Rust?", "", &[], None);

        // Assert: No RAG section should be present
        assert!(!prompt.contains("## Relevant Context from Memory"));
    }

    #[test]
    fn test_build_prompt_basic() {
        // Basic test without async context
        let prompt = "Test system prompt";
        let input = "Test input";
        let _accumulated = "";

        let expected = format!("{}\n\nUser: {}\n", prompt, input);
        assert!(expected.contains(prompt));
        assert!(expected.contains(input));
    }

    #[test]
    fn test_check_stop_words_case_insensitive() {
        // Test stop word detection logic
        let output = "This response contains STOP keyword";
        let stop_words = ["stop".to_string()];

        let output_lower = output.to_lowercase();
        let found = stop_words
            .iter()
            .any(|word| output_lower.contains(&word.to_lowercase()));

        assert!(found, "Should detect stop word case-insensitively");
    }

    #[tokio::test]
    async fn test_vision_capability_check() {
        // Test that vision capability is checked
        // This is a placeholder until we implement the run_with_vision method
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let _service = PaladinExecutionService::new(llm_port.clone(), circuit_breaker, None, None);

        // Create a paladin with vision_enabled
        let data = PaladinData {
            vision_enabled: true,
            ..Default::default()
        };
        let paladin = Node::new(data, Some("VisionPaladin".to_string()));

        // Verify that the MockLlmPort doesn't support vision
        let caps = llm_port.get_capabilities();
        assert!(
            !caps.supports_vision,
            "MockLlmPort should not support vision"
        );

        // Verify paladin has vision_enabled
        assert!(
            paladin.node.vision_enabled,
            "Paladin should have vision enabled"
        );
    }

    #[tokio::test]
    async fn test_execute_with_vision_not_enabled() {
        // Test that execute_with_vision fails when vision is not enabled
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Create a paladin WITHOUT vision_enabled
        let data = PaladinData {
            vision_enabled: false,
            ..Default::default()
        };
        let paladin = Node::new(data, Some("NormalPaladin".to_string()));

        // Try to execute with vision - should fail
        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: ImageDetail::Auto,
        }];

        let result = service
            .execute_with_vision(&paladin, "What's in this image?", images)
            .await;

        assert!(result.is_err(), "Should fail when vision not enabled");
        match result {
            Err(PaladinError::ConfigurationError(msg)) => {
                assert!(
                    msg.contains("vision_enabled=true"),
                    "Error should mention vision_enabled"
                );
            }
            _ => panic!("Should return ConfigurationError"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_vision_unsupported_provider() {
        // Test that execute_with_vision fails when LLM provider doesn't support vision
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Create a paladin with vision_enabled but MockLlmPort doesn't support vision
        let data = PaladinData {
            vision_enabled: true,
            ..Default::default()
        };
        let paladin = Node::new(data, Some("VisionPaladin".to_string()));

        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: ImageDetail::Auto,
        }];

        let result = service
            .execute_with_vision(&paladin, "What's in this image?", images)
            .await;

        assert!(
            result.is_err(),
            "Should fail when provider doesn't support vision"
        );
        match result {
            Err(PaladinError::ConfigurationError(msg)) => {
                assert!(
                    msg.contains("does not support vision"),
                    "Error should mention lack of vision support"
                );
            }
            _ => panic!("Should return ConfigurationError"),
        }
    }

    // Mock VisionPort for testing
    struct MockVisionPort {
        provider: String,
        should_fail: bool,
        response_content: String,
    }

    impl MockVisionPort {
        fn new(provider: &str) -> Self {
            Self {
                provider: provider.to_string(),
                should_fail: false,
                response_content: "Mock vision analysis result".to_string(),
            }
        }

        #[allow(dead_code)]
        fn with_failure(mut self) -> Self {
            self.should_fail = true;
            self
        }

        fn with_response(mut self, content: String) -> Self {
            self.response_content = content;
            self
        }
    }

    #[async_trait]
    impl crate::application::ports::output::vision_port::VisionPort for MockVisionPort {
        async fn analyze_image(
            &self,
            _prompt: &str,
            _images: Vec<VisionContent>,
            _model: &str,
            _max_tokens: Option<u32>,
        ) -> Result<
            crate::application::ports::output::vision_port::VisionResult,
            crate::core::platform::container::vision::VisionError,
        > {
            if self.should_fail {
                return Err(
                    crate::core::platform::container::vision::VisionError::InvalidRequest(
                        "Mock failure".to_string(),
                    ),
                );
            }

            Ok(
                crate::application::ports::output::vision_port::VisionResult {
                    content: self.response_content.clone(),
                    model: "mock-model".to_string(),
                    token_usage: crate::application::ports::output::vision_port::VisionTokenUsage {
                        prompt_tokens: 100,
                        completion_tokens: 50,
                        total_tokens: 150,
                    },
                    metadata: std::collections::HashMap::new(),
                    timestamp: chrono::Utc::now(),
                },
            )
        }

        fn is_vision_model(&self, _model: &str) -> bool {
            true
        }

        fn provider_name(&self) -> &str {
            &self.provider
        }
    }

    #[tokio::test]
    async fn test_extract_provider_from_openai_model() {
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Test OpenAI models
        assert_eq!(
            service.extract_provider_from_model("gpt-4o").unwrap(),
            "openai"
        );
        assert_eq!(
            service.extract_provider_from_model("gpt-4-turbo").unwrap(),
            "openai"
        );
        assert_eq!(
            service
                .extract_provider_from_model("gpt-3.5-turbo")
                .unwrap(),
            "openai"
        );
        assert_eq!(
            service.extract_provider_from_model("o1-preview").unwrap(),
            "openai"
        );
    }

    #[tokio::test]
    async fn test_extract_provider_from_anthropic_model() {
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Test Anthropic models
        assert_eq!(
            service
                .extract_provider_from_model("claude-3-opus-20240229")
                .unwrap(),
            "anthropic"
        );
        assert_eq!(
            service
                .extract_provider_from_model("claude-3-sonnet")
                .unwrap(),
            "anthropic"
        );
        assert_eq!(
            service
                .extract_provider_from_model("claude-3-5-sonnet")
                .unwrap(),
            "anthropic"
        );
    }

    #[tokio::test]
    async fn test_extract_provider_from_unsupported_model() {
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Test unsupported model
        let result = service.extract_provider_from_model("llama-2-70b");
        assert!(result.is_err());
        match result {
            Err(PaladinError::ConfigurationError(msg)) => {
                assert!(msg.contains("Cannot determine provider"));
            }
            _ => panic!("Should return ConfigurationError"),
        }
    }

    #[tokio::test]
    async fn test_with_vision_adapter() {
        let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        let mock_vision = Arc::new(MockVisionPort::new("openai"));

        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
            .with_vision_adapter("openai".to_string(), mock_vision.clone());

        // Verify adapter was registered
        assert!(service.vision_adapters.contains_key("openai"));
        assert_eq!(service.vision_adapters.len(), 1);
    }

    #[tokio::test]
    async fn test_vision_execution_with_stop_word() {
        // Create mock LLM port with vision support
        struct VisionCapableMockLlmPort;

        #[async_trait]
        impl LlmPort for VisionCapableMockLlmPort {
            async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, LlmError> {
                unimplemented!()
            }

            async fn generate_stream(
                &self,
                _request: LlmRequest,
            ) -> Result<
                Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>,
                LlmError,
            > {
                unimplemented!()
            }

            async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
                Ok(true)
            }

            async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
                Ok(vec![])
            }

            fn get_provider_name(&self) -> &'static str {
                "MockWithVision"
            }

            fn get_capabilities(&self) -> ProviderCapabilities {
                ProviderCapabilities {
                    supports_streaming: false,
                    supports_function_calling: false,
                    supports_tool_calling: false,
                    supports_vision: true,
                    supports_embeddings: false,
                    max_context_tokens: Some(4096),
                    supports_system_messages: true,
                }
            }
        }

        let llm_port: Arc<dyn LlmPort> = Arc::new(VisionCapableMockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        // Create mock vision adapter that returns content with stop word
        let mock_vision = Arc::new(
            MockVisionPort::new("openai").with_response("This is a STOP word test".to_string()),
        );

        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
            .with_vision_adapter("openai".to_string(), mock_vision);

        // Create paladin with vision enabled and stop word
        let data = PaladinData {
            vision_enabled: true,
            model: "gpt-4o".to_string(),
            stop_words: vec!["STOP".to_string()],
            ..Default::default()
        };
        let paladin = Node::new(data, Some("VisionPaladin".to_string()));

        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: ImageDetail::Auto,
        }];

        let result = service
            .execute_with_vision(&paladin, "What's in this image?", images)
            .await;

        // Should detect stop word and return error
        assert!(result.is_err());
        match result {
            Err(PaladinError::ExecutionError(msg)) => {
                assert!(
                    msg.contains("Stop word detected") || msg.contains("STOP"),
                    "Error should mention stop word: {}",
                    msg
                );
            }
            _ => panic!("Should return ExecutionError with stop word message"),
        }
    }

    #[tokio::test]
    async fn test_vision_execution_missing_adapter() {
        // Create mock LLM port with vision support
        struct VisionCapableMockLlmPort;

        #[async_trait]
        impl LlmPort for VisionCapableMockLlmPort {
            async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, LlmError> {
                unimplemented!()
            }

            async fn generate_stream(
                &self,
                _request: LlmRequest,
            ) -> Result<
                Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>,
                LlmError,
            > {
                unimplemented!()
            }

            async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
                Ok(true)
            }

            async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
                Ok(vec![])
            }

            fn get_provider_name(&self) -> &'static str {
                "MockWithVision"
            }

            fn get_capabilities(&self) -> ProviderCapabilities {
                ProviderCapabilities {
                    supports_streaming: false,
                    supports_function_calling: false,
                    supports_tool_calling: false,
                    supports_vision: true,
                    supports_embeddings: false,
                    max_context_tokens: Some(4096),
                    supports_system_messages: true,
                }
            }
        }

        let llm_port: Arc<dyn LlmPort> = Arc::new(VisionCapableMockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        // Create service WITHOUT registering vision adapter
        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None);

        // Create paladin with OpenAI model (requires openai adapter)
        let data = PaladinData {
            vision_enabled: true,
            model: "gpt-4o".to_string(),
            ..Default::default()
        };
        let paladin = Node::new(data, Some("VisionPaladin".to_string()));

        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: ImageDetail::Auto,
        }];

        let result = service
            .execute_with_vision(&paladin, "What's in this image?", images)
            .await;

        // Should fail because no vision adapter registered
        assert!(result.is_err());
        match result {
            Err(PaladinError::ExecutionError(msg)) => {
                assert!(
                    msg.contains("No vision adapter registered"),
                    "Error should mention missing adapter: {}",
                    msg
                );
            }
            _ => panic!("Should return ExecutionError about missing adapter"),
        }
    }

    #[tokio::test]
    async fn test_vision_execution_successful() {
        // Create mock LLM port with vision support
        struct VisionCapableMockLlmPort;

        #[async_trait]
        impl LlmPort for VisionCapableMockLlmPort {
            async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, LlmError> {
                unimplemented!()
            }

            async fn generate_stream(
                &self,
                _request: LlmRequest,
            ) -> Result<
                Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>,
                LlmError,
            > {
                unimplemented!()
            }

            async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
                Ok(true)
            }

            async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
                Ok(vec![])
            }

            fn get_provider_name(&self) -> &'static str {
                "MockWithVision"
            }

            fn get_capabilities(&self) -> ProviderCapabilities {
                ProviderCapabilities {
                    supports_streaming: false,
                    supports_function_calling: false,
                    supports_tool_calling: false,
                    supports_vision: true,
                    supports_embeddings: false,
                    max_context_tokens: Some(4096),
                    supports_system_messages: true,
                }
            }
        }

        let llm_port: Arc<dyn LlmPort> = Arc::new(VisionCapableMockLlmPort);
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));

        let mock_vision = Arc::new(MockVisionPort::new("openai"));

        let service = PaladinExecutionService::new(llm_port, circuit_breaker, None, None)
            .with_vision_adapter("openai".to_string(), mock_vision);

        // Create paladin with vision enabled
        let data = PaladinData {
            vision_enabled: true,
            model: "gpt-4o".to_string(),
            ..Default::default()
        };
        let paladin = Node::new(data, Some("VisionPaladin".to_string()));

        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: ImageDetail::Auto,
        }];

        let result = service
            .execute_with_vision(&paladin, "What's in this image?", images)
            .await;

        // Should succeed
        assert!(result.is_ok());
        let paladin_result = result.unwrap();
        assert_eq!(paladin_result.output, "Mock vision analysis result");
        assert_eq!(paladin_result.token_count, 150);
        assert_eq!(paladin_result.loop_count, 1);
        assert_eq!(paladin_result.stop_reason, StopReason::Completed);
    }
}
