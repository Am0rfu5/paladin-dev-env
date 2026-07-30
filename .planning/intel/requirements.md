# Requirements (from PRD-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP`. 11 PRDs consumed.

IDs marked `-v1` / `-v2` are competing variants preserved verbatim from
different PRDs on the same scope. They are NOT merged. See
`.planning/INGEST-CONFLICTS.md` WARNINGS for the resolution each needs.

---

## REQ-paladin-entity
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-1)
- description: Core Paladin domain entity representing an autonomous AI agent.
- acceptance:
  - MUST use the existing `Node<T>` pattern for consistency with other domain entities
  - PaladinData MUST include: system_prompt, name, user_name, model, temperature, max_loops, stop_words, status
  - PaladinStatus MUST support states: Idle, Reasoning, Executing, Completed, Failed(String)
  - MUST be serializable/deserializable (Serde support)
  - MUST be cloneable for distributed execution scenarios
- scope: Paladin entity, PaladinData, PaladinStatus, core domain layer

## REQ-paladin-builder
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-2, US-1, US-2)
- description: Fluent PaladinBuilder for declarative agent configuration with fail-fast validation.
- acceptance:
  - MUST require LlmPort at construction time
  - MUST validate system_prompt is non-empty
  - MUST validate temperature is in range [0.0, 1.0] (see REQ-temperature-range-v1 / -v2)
  - MUST validate max_loops is in range [1, 100], default 3
  - MUST provide default values for optional fields
  - MUST support method chaining
  - Build method MUST return `Result<Paladin, PaladinError>` with detailed validation errors
  - stop_words accepts list of strings, can be empty
- scope: PaladinBuilder, builder validation, application use_cases layer

## REQ-paladin-config
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-3)
- description: Runtime configuration type for Paladin execution.
- acceptance:
  - MUST support retry_attempts (u32, default: 3)
  - MUST support timeout_seconds (u64, default: 300)
  - MUST support enable_planning (bool, default: false)
  - MUST support optional planning_prompt (String)
  - MUST support output_format (enum: Text, Json, Structured)
  - MUST use Builder pattern for construction
- scope: PaladinConfig, configuration types

## REQ-paladin-port
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-4)
- description: Hexagonal port abstraction for Paladin execution.
- acceptance:
  - MUST define async `execute()` returning PaladinResult
  - MUST define async `execute_stream()` for streaming responses
  - MUST define sync `validate()` for configuration checks
  - MUST be Send + Sync for async compatibility
  - MUST use trait objects (dyn PaladinPort) for runtime polymorphism
  - Note: streaming interface is defined here but implementation is deferred to Epic 6 (PRD Non-Goal 10)
- scope: PaladinPort, application ports/output layer

## REQ-paladin-execution-service
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-5, US-3, US-4, US-5)
- description: Service coordinating the Paladin reasoning loop, retries, and circuit breaking.
- acceptance:
  - MUST coordinate LLM calls via LlmPort
  - MUST implement reasoning loop respecting max_loops
  - MUST check stop words after each LLM response; case-insensitive matching; partial matches do not trigger stop
  - MUST enforce timeout using tokio::time::timeout
  - MUST implement retry logic with exponential backoff (100ms, 200ms, 400ms; default 3 attempts)
  - MUST implement circuit breaker pattern (open/half-open/closed); opens after 5 consecutive failures by default; half-open allows test requests; closes after successful recovery
  - MUST build prompts from Paladin config + user input
  - MUST track execution metadata (duration, tokens, retries)
  - PaladinResult contains output text, token usage, execution metadata
- scope: PaladinExecutionService, reasoning loop, retry, circuit breaker, stop words, timeout

## REQ-paladin-error-handling
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-6)
- description: Layer-specific error type for the Paladin domain.
- acceptance:
  - PaladinError MUST use thiserror
  - MUST include variants: ConfigurationError, ExecutionError, LlmError, Timeout, StopWordDetected
  - All errors MUST include descriptive messages
  - Errors MUST be propagated using Result<T, E>
  - Circuit breaker state changes MUST be logged
- scope: PaladinError, error handling

## REQ-paladin-observability
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-7)
- description: Structured logging for Paladin execution.
- acceptance:
  - MUST use the `tracing` crate for structured logging
  - MUST log at appropriate levels (trace, debug, info, warn, error)
  - MUST include execution_id in all log entries
  - MUST log: execution start, each loop iteration, stop word detection, completion, errors
  - Enhanced monitoring features are explicitly OUT OF SCOPE (deferred to Epic 10)
- scope: logging, tracing, observability

## REQ-paladin-testing-infra
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-8)
- description: Mock LLM test double enabling Paladin testing without a live provider.
- acceptance:
  - MUST provide MockLlmPort for testing
  - MUST support configurable responses
  - MUST support failure simulation
  - MUST track call history for assertions
  - All public APIs MUST have rustdoc examples that compile
  - Mock must additionally support delay simulation and token counting
- scope: MockLlmPort, MockLlmAdapter, test doubles

---

## REQ-garrison-entry
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR1)
- description: GarrisonEntry domain type representing a single memory record.
- acceptance:
  - MUST contain unique identifier (UUID), conversation role (System, User, Assistant, Tool), content, UTC timestamp, extensible metadata key-value map, optional token count
  - MUST validate all required fields are populated before storing
  - MUST support serialization/deserialization for persistence
- scope: GarrisonEntry, ConversationRole, core domain layer

## REQ-garrison-windowing
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR2)
- description: ConversationHistory with entry/token limits and importance-based eviction.
- acceptance:
  - MUST store entries in chronological order, enforce max entry count, enforce max token count when configured, provide efficient recent-N retrieval
  - Importance-based eviction MUST always preserve System-role entries and the most recent N messages (configurable, default 10)
  - MUST evict oldest user/assistant messages from the middle of history; never evict if within limits
  - If still exceeding limits after removing all candidates: log warning and proceed anyway
  - MUST calculate token counts using LLM-specific tokenizers; integrate tiktoken for OpenAI; support pluggable tokenizers; cache token counts
- scope: ConversationHistory, memory windowing, token limits, eviction

## REQ-garrison-port
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR3)
- description: GarrisonPort trait defining memory operations.
- acceptance:
  - MUST define async methods: `remember(entry)`, `recall_recent(limit)`, `search(query, limit)`, `forget_all()`, `stats()`
  - All trait methods MUST be Send + Sync
  - All trait methods MUST return `Result<T, GarrisonError>`
- scope: GarrisonPort, application ports/output layer

## REQ-garrison-longterm-port
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR4)
- description: LongTermGarrisonPort extension adding vector/semantic retrieval.
- acceptance:
  - MUST extend GarrisonPort with `remember_with_embedding(entry, embedding)` and `search_similar(embedding, limit)`
  - MUST support embeddings as `Vec<f32>`
  - Semantic search MUST return results ranked by cosine similarity
  - Search results MUST be limitable to top-K most similar entries
- scope: LongTermGarrisonPort, semantic search, embeddings

## REQ-garrison-in-memory
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR5)
- description: InMemoryGarrison adapter for short-term, non-durable memory.
- acceptance:
  - MUST store entries in a thread-safe `RwLock<VecDeque<GarrisonEntry>>`
  - MUST support all GarrisonPort operations
  - MUST provide O(1) append and O(N) search performance
  - Loses all data on application shutdown
  - MUST be the default implementation for quick prototyping
- scope: InMemoryGarrison, infrastructure adapters

## REQ-garrison-sqlite
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR6)
- description: SqliteGarrison adapter for durable memory across restarts.
- acceptance:
  - MUST use SQLite for persistent storage in a `garrison_entries` table, support all GarrisonPort operations, persist across restarts
  - Schema MUST include: id TEXT PRIMARY KEY, paladin_id TEXT NOT NULL, role TEXT NOT NULL, content TEXT NOT NULL, timestamp INTEGER NOT NULL, metadata TEXT, token_count INTEGER, embedding BLOB
  - MUST create index `idx_paladin_timestamp` on (paladin_id, timestamp)
  - MUST implement connection pooling using sqlx
  - MUST support vector search using the SQLite-vss extension for embeddings
- scope: SqliteGarrison, schema, sqlx, sqlite-vss

## REQ-garrison-paladin-integration
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR7)
- description: Optional attachment of Garrison to a Paladin, with automatic context handling.
- acceptance:
  - PaladinBuilder MUST provide `with_garrison(port)`
  - Paladins without Garrison MUST execute successfully for single-turn requests
  - Paladins without Garrison MUST return `PaladinError::GarrisonRequired` when attempting multi-turn conversations
  - PaladinExecutionService MUST automatically store user input as an entry, retrieve recent history before LLM calls, inject history into prompt construction, and store LLM responses as entries
- scope: PaladinBuilder.with_garrison, PaladinExecutionService, multi-turn conversations

## REQ-garrison-config
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR8)
- description: Garrison configuration via config.yml and ApplicationSettings.
- acceptance:
  - MUST support config.yml keys: garrison.type ("sqlite" | "in_memory"), path, max_entries, max_tokens, tokenizer, eviction_strategy
  - MUST be loaded via ApplicationSettings
  - Invalid configuration MUST result in `GarrisonError::Configuration`
- scope: garrison configuration, ApplicationSettings, config.yml

## REQ-garrison-errors
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR9)
- description: GarrisonError enum for memory subsystem failures.
- acceptance:
  - MUST define variants: StorageError(String), SerializationError(String), TokenizationError(String), NotFound(String), ConfigurationError(String)
  - All errors MUST implement `std::error::Error` and `Display` via thiserror
  - Errors propagate to `PaladinError::GarrisonError(GarrisonError)` at the service layer
- scope: GarrisonError, error handling

## REQ-garrison-testing
- source: /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md (FR10)
- description: Test and quality gates for the Garrison subsystem.
- acceptance:
  - Unit test coverage MUST be >= 80% (see REQ-test-coverage-target-v1 / -v2)
  - MUST include integration tests: test_sqlite_garrison_persistence, test_paladin_with_garrison_context, test_garrison_recovery_after_restart, test_token_limit_enforcement, test_semantic_search_accuracy
  - All public APIs MUST have rustdoc documentation
  - Code MUST pass `cargo clippy` with no warnings
- scope: garrison testing, coverage, clippy

---

## REQ-arsenal-domain-types
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-1)
- description: Arsenal domain types for tool definition, invocation, and result.
- acceptance:
  - `Armament` MUST have name (unique identifier), description, JSON Schema for parameters, list of required parameter names
  - `ArmamentCall` MUST have tool name, arguments (HashMap of parameter name to JSON value), unique call ID (UUID)
  - `ArmamentResult` MUST have call ID matching the request, success boolean, optional output (JSON value), optional error message, execution time in milliseconds
- scope: Armament, ArmamentCall, ArmamentResult, core domain layer

## REQ-arsenal-port
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-2)
- description: ArsenalPort and ArsenalRegistry traits for tool operations.
- acceptance:
  - ArsenalPort MUST provide async `list_armaments()`, `invoke(call: ArmamentCall)`, `validate_call(call: &ArmamentCall)`
  - ArsenalRegistry MUST provide `register()`, `unregister()`, `get()`
  - All port traits MUST be Send + Sync
- scope: ArsenalPort, ArsenalRegistry, application ports/output layer

## REQ-mcp-protocol
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-3)
- description: MCP (Model Context Protocol) client and transport abstraction.
- acceptance:
  - MCPClient MUST manage communication with MCP servers, handle protocol-level message serialization/deserialization, report server capabilities
  - MUST define an `MCPTransport` trait with `send(message: MCPMessage)` and `receive()`
  - MUST comply with the Model Context Protocol specification for tool discovery and invocation
  - Implementation must follow JSON-RPC 2.0 message format, standard tool discovery and invocation methods, and proper error codes
- scope: MCPClient, MCPTransport, MCP protocol compliance, JSON-RPC 2.0

## REQ-mcp-stdio-transport
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-4)
- description: STDIO transport adapter for command-line MCP servers.
- acceptance:
  - MCPStdioAdapter MUST spawn external processes with configurable command and arguments, communicate via stdin/stdout, manage process lifecycle (start, stop, cleanup)
  - MUST handle process initialization and connect to the MCP server on startup
  - MUST properly clean up child processes on shutdown or failure
- scope: MCPStdioAdapter, tokio::process, infrastructure adapters

## REQ-mcp-sse-transport
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-5)
- description: SSE transport adapter for web-service MCP servers.
- acceptance:
  - MCPSseAdapter MUST connect to HTTP/HTTPS endpoints, use Server-Sent Events for receiving messages, use HTTP POST for sending messages
  - MUST handle connection retry logic with exponential backoff
  - MUST include proper connection timeout handling
- scope: MCPSseAdapter, SSE, reqwest, infrastructure adapters

## REQ-arsenal-builder-integration
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-6)
- description: PaladinBuilder methods for attaching MCP tool servers.
- acceptance:
  - MUST provide `add_mcp_stdio(command, args)`
  - MUST provide `add_mcp_sse(name, endpoint)`
  - Builder MUST validate MCP server connections during the `build()` phase
- scope: PaladinBuilder, MCP server registration

## REQ-arsenal-resource-controls
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-7)
- description: Timeout and concurrency limits on tool execution.
- acceptance:
  - MUST support configurable timeout for each tool invocation (default: 30 seconds)
  - MUST support configurable maximum concurrent tool executions (default: 5)
  - Invocations exceeding the timeout MUST return a timeout error in the ArmamentResult
  - Invocations exceeding concurrency limits MUST queue and wait for available slots
  - Implementation uses `tokio::sync::Semaphore` for concurrency and `tokio::time::timeout` for timeouts
- scope: tool timeout, concurrency limiting, resource safety

## REQ-arsenal-resilience
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-8)
- description: Tool failures must degrade gracefully without halting the Paladin.
- acceptance:
  - On failure the system MUST create an ArmamentResult with `success: false`, include the error message, record execution time, and return the result to the Paladin
  - Paladin execution MUST continue after tool failures, with the failure result injected into context
  - MUST log all tool failures with appropriate severity levels
  - Connection failures to MCP servers during builder initialization MUST return a clear error
- scope: tool failure handling, resilience

## REQ-arsenal-context-injection
- source: /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md (FR-9)
- description: Formatting and injection of tool results into Paladin conversation context.
- acceptance:
  - Results MUST be formatted as structured text and injected into the Paladin's conversation context
  - Injected context MUST include tool name, call arguments, execution outcome (success/failure), output data or error message, execution time
  - Format MUST be readable by the LLM for reasoning about tool outcomes
- scope: tool result formatting, context injection

---

## REQ-battalion-config-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.1)
- description: BattalionConfig as defined by the Battalion Orchestration PRD. COMPETING VARIANT — see REQ-battalion-config-v2.
- acceptance:
  - MUST support: name, description, timeout_seconds, retry_policy, error_strategy, metadata_output_dir
  - MUST validate configuration before execution
  - MUST serialize/deserialize Battalion configurations
- scope: BattalionConfig, Battalion configuration

## REQ-battalion-config-v2
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-7)
- description: BattalionConfig as defined by the Commander Strategy Router PRD. COMPETING VARIANT — see REQ-battalion-config-v1.
- acceptance:
  - MUST accept and forward: name (String), timeout_seconds (u64), retry_attempts (u32), error_strategy (ErrorStrategy), enable_checkpointing (bool), metadata_output_dir (Option<PathBuf>)
  - MUST apply config consistently across all underlying Battalion services
  - MUST provide sensible defaults for all config fields if not specified
- scope: BattalionConfig, Commander configuration passthrough

## REQ-battalion-result-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.2)
- description: BattalionResult as defined by the Battalion Orchestration PRD. COMPETING VARIANT — see REQ-battalion-result-v2.
- acceptance:
  - MUST contain: battalion_id, battalion_name, timestamps, final_output, individual paladin_results, status
  - MUST capture all intermediate Paladin results
  - MUST include execution timing for each Paladin and overall Battalion
- scope: BattalionResult, Battalion output structure

## REQ-battalion-result-v2
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-5)
- description: BattalionResult as defined by the Commander Strategy Router PRD. COMPETING VARIANT — see REQ-battalion-result-v1.
- acceptance:
  - MUST contain: battalion_id (Uuid), strategy_used (BattalionStrategy), paladin_results (Vec<PaladinResult>), final_output (String), execution_time_ms (u64), status (BattalionStatus), metadata (BattalionMetadata)
  - metadata MUST include strategy_selection_reasoning (Option<String>), strategy_selection_time_ms (u64), per_paladin_times (Vec<u64>), paladin_success_count (usize), paladin_failure_count (usize), timestamp (DateTime<Utc>)
  - MUST preserve all error details in `BattalionResult::errors: Vec<PaladinError>`
- scope: BattalionResult, Commander telemetry, result normalization

## REQ-battalion-error-strategy
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.3, US-4.3)
- description: Configurable error handling strategies for Battalion execution.
- acceptance:
  - FailFast: MUST stop immediately on first error, return error result
  - ContinueOnError: MUST continue execution, collect all errors, report at end
  - RetryThenContinue: MUST retry failed operations up to configured attempts with exponential backoff
- scope: ErrorStrategy, Battalion error handling

## REQ-battalion-retry-policy
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.4)
- description: Retry policy for Battalion operations.
- acceptance:
  - MUST support configurable retry attempts (default: 3)
  - MUST support configurable retry delays with exponential backoff
  - MUST support jitter to prevent thundering herd
  - MUST log all retry attempts
  - Reference policy: max_attempts 3, base_delay 100ms, max_delay 10s, exponential_backoff true, jitter true
- scope: RetryPolicy, exponential backoff, jitter

## REQ-formation-min-paladins-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.5, FR-4.8)
- description: Minimum Paladin count for Formation and Phalanx. COMPETING VARIANT — see REQ-formation-min-paladins-v2.
- acceptance:
  - Formation MUST validate at least 2 Paladins are provided
  - Phalanx MUST accept a list of Paladin instances (>= 2)
  - Majority aggregation requires >= 3 Paladins
- scope: Formation construction, Phalanx construction, Paladin count validation

## REQ-formation-min-paladins-v2
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-1, FR-3)
- description: Minimum Paladin count as implied by Commander construction and Auto routing. COMPETING VARIANT — see REQ-formation-min-paladins-v1.
- acceptance:
  - Commander MUST validate at construction that at least one Paladin is provided
  - Auto rule 1: "Single Paladin: Select `Formation` (trivial case)"
- scope: Commander construction validation, Auto strategy selection, Paladin count

## REQ-formation-construction
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.5)
- description: Formation construction from an ordered Paladin list.
- acceptance:
  - MUST accept ordered list of Paladin instances
  - MUST validate minimum Paladin count (see REQ-formation-min-paladins-v1 / -v2)
  - MUST support optional shared context injected into all Paladin prompts
- scope: Formation, sequential Battalion construction

## REQ-formation-execution
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.6, US-4.1)
- description: Sequential Formation execution with output chaining.
- acceptance:
  - MUST execute Paladins sequentially in order
  - MUST pass output of Paladin N as input to Paladin N+1
  - MUST respect timeout_seconds for total execution time
  - MUST respect error_strategy configuration
- scope: FormationExecutionService, sequential execution

## REQ-formation-output
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.7)
- description: Formation result content.
- acceptance:
  - MUST return final output from last Paladin
  - MUST include all intermediate Paladin outputs in result
  - MUST preserve execution order in results
- scope: Formation output, intermediate results

## REQ-phalanx-construction
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.8)
- description: Phalanx construction with an aggregation strategy.
- acceptance:
  - MUST accept a list of Paladin instances (minimum count per REQ-formation-min-paladins-v1 / -v2)
  - MUST accept AggregationStrategy: CollectAll, FirstSuccess, Majority, or Custom
  - MUST support Custom aggregation via user-provided function
- scope: Phalanx, AggregationStrategy, concurrent Battalion construction

## REQ-phalanx-concurrency
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.9, US-4.2)
- description: Concurrent Phalanx execution characteristics.
- acceptance:
  - MUST execute all Paladins concurrently using the Tokio runtime
  - MUST support >= 10 concurrent Paladin executions
  - MUST complete with < 1 second orchestration overhead for typical workloads
  - MUST handle partial failures based on error_strategy
  - All Paladins receive the same input simultaneously
  - Implementation: tokio::spawn per Paladin, futures::future::join_all for collection, semaphore-based concurrency limiting (default 10)
- scope: PhalanxExecutionService, concurrency, orchestration overhead

## REQ-phalanx-aggregation
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.10)
- description: Aggregation semantics per strategy.
- acceptance:
  - CollectAll: MUST return all Paladin results in array
  - FirstSuccess: MUST return first successfully completed result and cancel remaining
  - Majority: MUST analyze results and return most common output (requires >= 3 Paladins)
  - Custom: MUST invoke user-provided aggregation function with all results
- scope: AggregationStrategy semantics

## REQ-campaign-graph
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.11, US-4.5)
- description: Campaign DAG construction and validation.
- acceptance:
  - MUST support DAG structure with nodes (Paladins) and edges (CampaignEdge)
  - MUST allow adding Paladins and edges programmatically
  - MUST validate graph is acyclic before execution
  - MUST validate all edges have valid source/target nodes
  - Implementation uses petgraph (>= 0.6)
- scope: Campaign, DAG, graph validation, petgraph

## REQ-campaign-edge-conditions
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.12)
- description: Conditional routing on Campaign edges.
- acceptance:
  - MUST support EdgeCondition types: Always, Contains(String), Regex(String), Custom(Fn)
  - MUST evaluate edge conditions based on source Paladin output
  - MUST support optional output transformation functions on edges
- scope: CampaignEdge, EdgeCondition, conditional routing

## REQ-campaign-execution
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.13)
- description: Campaign graph execution ordering and topology handling.
- acceptance:
  - MUST execute Paladins respecting dependency order
  - MUST support multiple entry points
  - MUST handle fan-out (1 -> N) and fan-in (N -> 1) patterns
  - MUST execute independent branches concurrently
  - Implementation uses topological sort; tokio::sync::mpsc for inter-node communication
- scope: CampaignExecutionService, topological sort, fan-out/fan-in

## REQ-chain-of-command-construction
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.14)
- description: Chain of Command construction with commander and specialists.
- acceptance:
  - MUST accept one commander Paladin and >= 1 specialist Paladins
  - MUST accept DelegationStrategy: Automatic, Broadcast, RoundRobin, Custom
- scope: ChainOfCommand, DelegationStrategy, hierarchical construction

## REQ-chain-of-command-execution
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.15, US-4.6)
- description: Delegation semantics per strategy.
- acceptance:
  - Automatic: Commander MUST analyze input and select appropriate specialists
  - Broadcast: MUST delegate to all specialists concurrently
  - RoundRobin: MUST cycle through specialists sequentially
  - Custom: MUST invoke user-provided delegation function
  - MUST handle specialist failures with fallback logic
- scope: Chain of Command delegation, specialist selection

## REQ-chain-of-command-aggregation
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.16)
- description: Commander aggregation of specialist results.
- acceptance:
  - MUST have Commander aggregate specialist results
  - MUST inject specialist outputs into Commander's context
  - MUST return Commander's final synthesized response
- scope: Chain of Command aggregation, result synthesis

## REQ-battalion-status
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.17, US-4.4)
- description: Battalion status queries for progress monitoring.
- acceptance:
  - MUST provide `status()` method returning current Battalion state
  - MUST support states: Idle, Running, Paused, Completed, Failed, Cancelled
  - MUST include progress information (completed/total Paladins) and list of completed/failed/pending Paladins
  - MUST include timing information (elapsed, estimated remaining) and error details
  - MUST support async status polling
- scope: BattalionStatus, status queries, monitoring

## REQ-battalion-logging
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.18)
- description: Execution logging for Battalion runs.
- acceptance:
  - MUST log Battalion start/completion with metadata
  - MUST log each Paladin execution start/completion
  - MUST log all errors and retry attempts
  - MUST support structured logging (JSON) for observability tools
- scope: Battalion logging, structured logging

## REQ-battalion-cancellation
- source: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md (FR-4.19)
- description: Cancellation of in-flight Battalion execution.
- acceptance:
  - MUST provide `cancel()` method to stop ongoing execution
  - MUST gracefully terminate in-progress Paladins
  - MUST return partial results on cancellation
  - MUST mark status as Cancelled
- scope: Battalion cancellation, graceful termination

---

## REQ-commander-construction
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-1, US-1)
- description: Commander construction and construction-time validation.
- acceptance:
  - MUST be constructible with strategy (BattalionStrategy, explicit or Auto), paladins (Vec<Paladin>, ordered), config (BattalionConfig, optional)
  - MUST validate at construction: at least one Paladin provided (see REQ-formation-min-paladins-v2), all Paladins valid (not in failed state), config internally consistent
  - MUST return detailed error if validation fails
  - Commander MUST handle service instantiation internally
- scope: Commander, construction validation, builder pattern

## REQ-commander-strategy-types
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-2)
- description: BattalionStrategy enum variants supported by the Commander.
- acceptance:
  - MUST support: Formation (sequential with output chaining), Phalanx (concurrent parallel), Campaign (graph/DAG), ChainOfCommand (hierarchical delegation), Auto (rule-based automatic selection)
- scope: BattalionStrategy enum

## REQ-commander-auto-selection
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-3, US-2)
- description: Rule-based automatic strategy selection in Auto mode.
- acceptance:
  - Rules applied in order: (1) Single Paladin -> Formation; (2) keyword detection — "sequential"/"pipeline"/"chain"/"step by step" -> Formation, "parallel"/"concurrent"/"all at once"/"simultaneously" -> Phalanx, "workflow"/"graph"/"conditional"/"if-then" -> Campaign, "delegate"/"hierarchy"/"specialist"/"expert" -> ChainOfCommand; (3) Paladin count heuristics — 2-3 -> Formation, 4+ with similar roles -> Phalanx, 4+ with specialized roles -> ChainOfCommand; (4) default fallback -> Formation
  - MUST log selection reasoning including triggering rule, detected keywords, Paladin count and role analysis
  - Manual strategy selection is an acceptable alternative if Auto is uncertain
- scope: Auto strategy selection, heuristics, keyword detection

## REQ-commander-execute
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-4, US-3)
- description: Single unified async execution entry point.
- acceptance:
  - MUST provide `pub async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError>`
  - Implementation steps: resolve strategy (run selection if Auto), build appropriate Battalion structure, delegate to corresponding service, wrap result in telemetry metadata, return normalized BattalionResult
  - Returns a consistent BattalionResult type; unified error type BattalionError
  - Execution behavior matches underlying Battalion type semantics
- scope: Commander.execute, unified interface, delegation

## REQ-commander-result-normalization
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-5)
- description: Normalization of results across all strategies. See REQ-battalion-result-v2 for the field set.
- acceptance:
  - MUST return the consistent BattalionResult defined in REQ-battalion-result-v2
  - MUST populate detailed telemetry metadata fields
- scope: result normalization, BattalionMetadata

## REQ-commander-error-strategy
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-6, US-5)
- description: Commander-level error handling configuration.
- acceptance:
  - MUST support ErrorStrategy::FailFast (stop on first Paladin failure, return error immediately)
  - MUST support ErrorStrategy::ContinueOnError (continue remaining Paladins, collect all errors)
  - MUST support ErrorStrategy::RetryThenContinue (retry failed Paladin up to N times, then continue)
  - MUST respect retry configuration from `BattalionConfig::retry_attempts`
  - MUST preserve all error details in `BattalionResult::errors: Vec<PaladinError>`
  - MUST allow developers to configure strategy per Commander instance via BattalionConfig
- scope: Commander error handling, ErrorStrategy

## REQ-commander-config-passthrough
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-7, US-4)
- description: Full BattalionConfig passthrough. See REQ-battalion-config-v2 for the field set.
- acceptance:
  - MUST accept complete BattalionConfig and forward it to underlying Battalion services
  - Default config MUST be provided if none specified
  - Config validation MUST happen at Commander construction time
- scope: configuration passthrough

## REQ-commander-service-composition
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-8)
- description: Internal composition of the four Battalion execution services.
- acceptance:
  - MUST internally compose Arc<FormationExecutionService>, Arc<PhalanxExecutionService>, Arc<CampaignExecutionService>, Arc<ChainOfCommandService>
  - SHOULD lazy-initialize services (do not instantiate unused services)
- scope: service composition, Arc sharing, lazy initialization

## REQ-commander-telemetry
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-9, US-6)
- description: Structured logging and telemetry export for Commander.
- acceptance:
  - MUST log: Commander construction with strategy and Paladin count, strategy selection (Auto) with reasoning, execution start with resolved strategy, each Paladin execution (delegated), execution completion with summary statistics
  - MUST use structured logging with contextual fields: commander_id, strategy, paladin_count, execution_time_ms, strategy_selection_reasoning
  - MUST export metadata to configured output directory if `metadata_output_dir` is set
- scope: Commander telemetry, structured logging, metadata export

## REQ-commander-validation
- source: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md (FR-10)
- description: Validation gates at construction and before execution.
- acceptance:
  - At construction: Paladins vector is not empty, all Paladins have valid configurations, BattalionConfig internally consistent
  - Before execution: input string is not empty (or allow empty based on Paladin requirements), Commander is not in error state from construction
- scope: Commander validation

---

## REQ-llm-port-interface
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-1, REQ-2)
- description: Provider-agnostic LlmPort trait and capability struct.
- acceptance:
  - All providers MUST implement: `async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>`; `async fn generate_stream(...) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>`; `async fn validate_model(&self, model: &str) -> Result<bool, LlmError>`; `async fn get_available_models(&self) -> Result<Vec<String>, LlmError>`; `fn get_provider_name(&self) -> &'static str`; `fn get_capabilities(&self) -> ProviderCapabilities`
  - ProviderCapabilities MUST expose: supports_streaming, supports_tool_calling, supports_function_calling, supports_vision, supports_embeddings, max_context_tokens (Option<u32>), supports_system_messages
  - All providers MUST use `futures::Stream` for streaming consistency
- scope: LlmPort, ProviderCapabilities, feature detection

## REQ-deepseek-adapter
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-3 through REQ-8, Story 3)
- description: DeepSeek LLM provider adapter.
- acceptance:
  - MUST be implemented at `src/infrastructure/adapters/llm/deepseek_adapter.rs`
  - DeepSeekConfig MUST support: api_key (String), base_url (String), model (String), timeout_seconds (u64)
  - MUST support standard completion requests with temperature control (0.0-2.0 — see REQ-temperature-range-v2), max tokens limit, top-p sampling, frequency/presence penalties
  - MUST support streaming responses via SSE
  - MUST validate API keys before making requests and return clear authentication failure errors
  - MUST map DeepSeek-specific errors to Paladin's LlmError enum
- scope: DeepSeekAdapter, DeepSeekConfig, streaming, error mapping

## REQ-anthropic-adapter
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-9 through REQ-15, Story 4)
- description: Anthropic (Claude) LLM provider adapter.
- acceptance:
  - MUST be implemented at `src/infrastructure/adapters/llm/anthropic_adapter.rs`
  - AnthropicConfig MUST support: api_key (String), base_url (String), model (String), max_tokens (u32)
  - MUST correctly format messages for the Claude API: system messages sent via `system` parameter (not in messages array); user/assistant messages alternate in messages array; multi-turn conversations handled correctly
  - MUST require the `max_tokens` parameter (Claude API requirement)
  - MUST support streaming via SSE
  - SHOULD implement tool use formatting if Claude supports tool/function calling
  - MUST handle Claude-specific rate limits and retry logic
- scope: AnthropicAdapter, AnthropicConfig, Claude message format, tool use

## REQ-provider-configuration
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-16 through REQ-19, Story 1, Story 2)
- description: Per-Paladin provider selection via config file and builder.
- acceptance:
  - config.yml MUST support an `llm` section with per-provider blocks for openai, deepseek, anthropic (api_key, base_url, default_model, timeout_seconds; anthropic additionally max_tokens)
  - PaladinBuilder MUST accept provider specification, e.g. `.provider("deepseek")`
  - Environment variables MUST be supported for API keys: OPENAI_API_KEY, DEEPSEEK_API_KEY, ANTHROPIC_API_KEY
  - Provider selection MUST fail fast with a clear error if the provider is not configured or the API key is missing
  - Provider selection MUST be per-instance, not global; multiple Paladins may run simultaneously with different providers, each maintaining its own client
  - Instantiation via `LlmProviderFactory::create(config, provider)`
- scope: provider configuration, LlmProviderFactory, per-Paladin provider selection

## REQ-provider-backward-compat
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-20 through REQ-22)
- description: Backward compatibility guarantees for existing consumers.
- acceptance:
  - Existing Paladin code without provider specification MUST continue to work using OpenAI as default (if configured)
  - Existing config files without provider sections MUST continue to work
  - Breaking changes MUST NOT be introduced to public APIs from Epic 1
- scope: backward compatibility, public API stability

## REQ-provider-error-mapping
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-23, REQ-24)
- description: Standardized error mapping across providers.
- acceptance:
  - All adapters MUST map provider-specific errors to LlmError variants: AuthenticationError, RateLimitError, ModelNotFoundError, InvalidRequestError, TimeoutError, NetworkError, ProviderError (with context)
  - Error messages MUST include actionable information (example: "Invalid API key for DeepSeek. Check DEEPSEEK_API_KEY environment variable.")
  - API keys MUST NOT be logged and MUST NOT be included in error messages
- scope: LlmError, error mapping, secret hygiene

## REQ-provider-testing
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-25 through REQ-28)
- description: Test strategy for provider adapters.
- acceptance:
  - Unit tests MUST achieve >= 80% code coverage using mocked HTTP responses (see REQ-test-coverage-target-v1 / -v2)
  - Integration tests MUST be available for live API testing but SHOULD be optional (feature-flagged or CI-only)
  - Test suite MUST cover: provider configuration loading, API key validation, request/response serialization, error mapping, streaming functionality, capability detection
  - Mock test servers SHOULD be provided for offline development
- scope: provider testing, mocked HTTP, live API tests

## REQ-provider-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-29 through REQ-31, Story 6, Story 7)
- description: Provider comparison, migration, and extension documentation.
- acceptance:
  - Provider comparison documentation MUST include feature matrix (streaming, tools, context limits), pricing comparison if publicly available, performance characteristics, use case recommendations
  - Migration guide MUST document how to add provider configuration to existing config files, how to update builder code for provider selection, how to implement a new provider adapter
  - API documentation MUST include examples for each provider
- scope: provider docs, migration guide, contribution guide

## REQ-temperature-range-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md (FR-2.3, US-2)
- description: Valid temperature range enforced at build time. COMPETING VARIANT — see REQ-temperature-range-v2.
- acceptance:
  - Builder MUST validate temperature is in range [0.0, 1.0]
  - Builder MUST reject invalid values (e.g. temperature > 1.0)
  - Temperature accepts 0.0-1.0 range, validated at build time
- scope: temperature validation, PaladinBuilder

## REQ-temperature-range-v2
- source: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md (REQ-5)
- description: Valid temperature range for the DeepSeek adapter. COMPETING VARIANT — see REQ-temperature-range-v1.
- acceptance:
  - DeepSeek adapter MUST support standard completion requests with temperature control (0.0-2.0)
- scope: temperature range, DeepSeekAdapter

---

## REQ-citadel-paladin-state-serialization
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR1)
- description: Complete serialization of Paladin state.
- acceptance:
  - MUST serialize Paladin configuration (Node<PaladinData>)
  - MUST serialize all Garrison entries with timestamps, roles, and content
  - MUST serialize execution history with timestamps, inputs, outputs, and status
  - MUST serialize created and updated timestamps
  - State format MUST be valid JSON conforming to the PaladinState schema
- scope: PaladinState, JSON serialization, core domain layer

## REQ-citadel-autosave
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR2, User Story 1)
- description: Automatic state persistence after every Paladin execution.
- acceptance:
  - MUST trigger save after every Paladin execution completion (success or failure)
  - MUST save to the configured directory (default: `./citadel/`)
  - MUST generate filename as `paladin-{uuid}.json`
  - MUST overwrite existing state file for the same Paladin ID
  - MUST log save operations with timestamps and file paths
  - Save operation MUST NOT block Paladin execution
- scope: autosave, Citadel persistence, file naming

## REQ-citadel-paladin-restore
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR3, User Story 2)
- description: Restoration of a Paladin from a saved state file.
- acceptance:
  - `PaladinBuilder.restore_from(state_id: Uuid)` MUST load state from file
  - MUST restore all configuration values from PaladinData
  - MUST restore all Garrison entries maintaining chronological order
  - MUST restore execution history for debugging/audit purposes
  - MUST fail with clear error if state file not found or invalid JSON
  - Restored Paladin has identical configuration, memory, and execution history
- scope: state restoration, PaladinBuilder.restore_from

## REQ-citadel-battalion-state-serialization
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR4)
- description: Complete serialization of Battalion state.
- acceptance:
  - MUST serialize Battalion type (Formation, Phalanx, Campaign, Chain of Command)
  - MUST serialize Battalion configuration and orchestration parameters
  - MUST serialize complete state of all constituent Paladins
  - MUST serialize checkpoint data indicating last completed Paladin/stage
  - State format MUST be valid JSON conforming to the BattalionState schema
- scope: BattalionState, CheckpointData, JSON serialization

## REQ-citadel-battalion-checkpoint-restore
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR5, User Story 3)
- description: Resumption of Battalion workflows from the last checkpoint.
- acceptance:
  - MUST identify last successfully completed Paladin from checkpoint
  - Formation MUST resume from next Paladin in sequence
  - Phalanx MUST re-execute only incomplete/failed Paladins
  - Campaign MUST continue from checkpoint node in graph
  - Chain of Command MUST resume delegation from last level
  - Already-completed Paladins MUST be skipped on restoration
- scope: Battalion checkpoint recovery, workflow resumption

## REQ-citadel-port
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR6)
- description: CitadelPort trait for state storage operations.
- acceptance:
  - MUST define `save_paladin(state: &PaladinState) -> Result<(), CitadelError>`
  - MUST define `load_paladin(id: Uuid) -> Result<Option<PaladinState>, CitadelError>`
  - MUST define `save_battalion(state: &BattalionState) -> Result<(), CitadelError>`
  - MUST define `load_battalion(id: Uuid) -> Result<Option<BattalionState>, CitadelError>`
  - MUST define `list_saved() -> Result<Vec<StateSummary>, CitadelError>`
  - Trait MUST be Send + Sync; FileCitadel implementation uses tokio::fs for async I/O
- scope: CitadelPort, FileCitadel, application ports/output layer

## REQ-citadel-errors
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR7)
- description: Fail-fast error handling for state operations.
- acceptance:
  - Corrupted JSON files MUST fail with `CitadelError::CorruptedState`
  - Missing files MUST fail with `CitadelError::StateNotFound`
  - Incompatible schema versions MUST fail with `CitadelError::IncompatibleVersion`
  - File system permission errors MUST fail with `CitadelError::IoError`
  - All errors MUST include descriptive messages for debugging
  - No partial recovery from corrupted state files (Non-Goal NG7)
- scope: CitadelError, fail-fast error handling

## REQ-citadel-builder-integration
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR8)
- description: PaladinBuilder integration for Citadel.
- acceptance:
  - `enable_autosave()` MUST activate automatic state persistence
  - `save_state_dir(path: &str)` MUST configure the storage directory
  - `restore_from(state_id: Uuid)` MUST load from saved state
  - Builder validation MUST ensure the state directory is writable
  - Configuration conflicts MUST produce clear build-time errors
- scope: PaladinBuilder, autosave configuration

## REQ-citadel-state-directory
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR9, User Story 5)
- description: State directory management.
- acceptance:
  - MUST create state directory automatically if it does not exist
  - MUST verify write permissions at initialization
  - MUST organize files as a flat structure (no subdirectories)
  - File naming convention: `paladin-{uuid}.json`, `battalion-{uuid}.json`
  - MUST support configurable path (relative or absolute); default `./citadel/`
  - Invalid paths MUST produce clear error messages at build time
  - config.yml keys: citadel.state_dir, citadel.autosave_enabled
- scope: state directory, file layout, Citadel configuration

## REQ-citadel-logging-docs
- source: /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md (FR10, User Story 4)
- description: Visibility and documentation for state operations.
- acceptance:
  - MUST log state save operations at INFO level with file path
  - MUST log state load operations at INFO level with state ID
  - MUST log restoration events with Paladin/Battalion identifiers
  - MUST provide rustdoc documentation for all public APIs
  - MUST provide example code demonstrating save/restore workflows
  - State files MUST be valid, formatted, human-readable JSON with readable Garrison role/content fields
- scope: Citadel logging, documentation, human-readable state

---

## REQ-herald-trait-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-1)
- description: Herald trait signature as stated in the Functional Requirements section. COMPETING VARIANT — see REQ-herald-trait-v2.
- acceptance:
  - MUST define `format_paladin_result(&self, result: &PaladinResult) -> String`
  - MUST define `format_battalion_result(&self, result: &BattalionResult) -> String`
  - MUST define `format_paladin_stream(&self, chunk: &StreamChunk) -> Option<String>`
  - MUST define `format_error(&self, error: &PaladinError) -> String`
  - Trait MUST be Send + Sync for async compatibility
- scope: Herald trait, formatter interface

## REQ-herald-trait-v2
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (section 6.2 Trait Design)
- description: Herald trait signature as stated in the Design Considerations section of the same PRD. COMPETING VARIANT — see REQ-herald-trait-v1.
- acceptance:
  - `format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError>`
  - `format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError>`
  - `format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError>`
  - `finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError>`
  - `format_error(&self, error: &PaladinError) -> String`
  - `name(&self) -> &str`
  - `mime_type(&self) -> &str` (e.g. "application/json", "text/markdown")
- scope: Herald trait, formatter interface, HeraldError

## REQ-herald-builtin-formatters
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-2, US-1, US-2)
- description: Three built-in Herald implementations.
- acceptance:
  - JsonHerald: serializes results to JSON using serde_json, supports optional pretty-printing via configuration, includes all metadata fields, schema documented in rustdoc
  - MarkdownHerald: formats results as structured Markdown with headings, code blocks and status badges; preserves whitespace/formatting from Paladin output; suitable for documentation generation; output color-coded when terminal supports ANSI colors
  - TableHerald: formats results as ASCII tables using comfy-table or similar; supports single-result and multi-result (Battalion) tables; includes column headers for metadata fields; configurable column widths
- scope: JsonHerald, MarkdownHerald, TableHerald

## REQ-herald-streaming
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-3, US-4)
- description: Complete and streaming output modes.
- acceptance:
  - Complete mode: formats entire PaladinResult after execution completes with all metadata available
  - Streaming mode: formats partial output as tokens arrive from the LLM; progressive formatting maintains consistency; final metadata appended when stream completes; errors during streaming include partial output
  - Format consistency MUST be maintained between streaming and complete modes
- scope: streaming formatting, progressive output

## REQ-herald-configuration
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-4)
- description: Herald configuration in config.yml.
- acceptance:
  - MUST support `herald.default_formatter` ("json" | "markdown" | "table")
  - MUST support `herald.json.pretty`, `herald.json.include_metadata`
  - MUST support `herald.markdown.include_colors`, `herald.markdown.heading_level`
  - MUST support `herald.table.max_column_width`, `herald.table.border_style`
- scope: Herald configuration, config.yml

## REQ-herald-default-and-override
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-5, US-5)
- description: Global default formatter with per-execution runtime override.
- acceptance:
  - Global default set in config.yml or environment variable, applied to all executions unless overridden, changeable at application startup
  - Execution methods MUST accept `Option<Arc<dyn Herald>>`
  - Override MUST take precedence over global default and MUST NOT mutate global configuration
- scope: formatter selection, runtime override

## REQ-herald-paladin-result-fields
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-6)
- description: Required content of formatted PaladinResult output.
- acceptance:
  - MUST include Paladin name and ID, execution status (Success, Failed, Timeout), output text/content
  - MUST include metadata: loop count, token usage (input, output, total), execution time, tool calls made (if any), stop reason (stop word, max loops, completion)
  - MUST include error details if failed, and timestamp
- scope: PaladinResult formatting, metadata preservation

## REQ-herald-battalion-result-fields
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-7, US-6)
- description: Required content of formatted BattalionResult output.
- acceptance:
  - MUST include Battalion name, ID, and type (Formation, Phalanx, Campaign, Chain of Command)
  - MUST include overall status and individual Paladin results formatted per selected Herald
  - MUST include execution order/graph (for Campaign) and reflect execution order/parallelism
  - MUST include aggregated metadata: total execution time, total token usage across all Paladins, success/failure counts
  - MUST include errors and partial results
  - Note: requires a Battalion type/strategy field and aggregated token usage in BattalionResult; see REQ-battalion-result-v1 / -v2
- scope: BattalionResult formatting, aggregated metadata

## REQ-herald-registry
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-8, US-3)
- description: HeraldRegistry for formatter management.
- acceptance:
  - MUST register built-in formatters by name and register custom formatters
  - MUST retrieve formatters by name and list available formatters
  - MUST provide thread-safe concurrent access
  - Herald trait MUST be public and well-documented so integrators can implement custom formatters
- scope: HeraldRegistry, custom formatters, extensibility

## REQ-herald-builder-integration
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-9)
- description: PaladinBuilder integration for Herald.
- acceptance:
  - MUST support `.with_herald(Arc::new(MarkdownHerald))` on PaladinBuilder
- scope: PaladinBuilder.with_herald

## REQ-herald-error-handling
- source: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md (FR-10)
- description: Graceful degradation on formatting failure.
- acceptance:
  - Formatting errors MUST NOT prevent result retrieval
  - MUST fall back to basic string representation on formatter failure
  - Errors MUST include context (which formatter, which result)
  - Partial formatting results MUST be preserved when possible
  - HeraldError variants: SerializationError(String), TemplateError(String), InvalidResult(String), IoError(#[from] std::io::Error)
- scope: HeraldError, formatter fallback

---

## REQ-cli-structure
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-1, FR-2, FR-3)
- description: Armory CLI naming and subcommand structure.
- acceptance:
  - CLI MUST be named `paladin-cli` and invocable via `paladin` when installed
  - MUST follow subcommand structure: `paladin <resource> <action> [options]`
  - MUST support top-level resource groups: `agent`, `battalion`, `arsenal`
  - Uses clap (v4+) with derive macros; `--help` supported at every level
- scope: paladin-cli, clap, command structure

## REQ-cli-agent-run
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-4, FR-6)
- description: `paladin agent run` execution of a single Paladin from YAML.
- acceptance:
  - Options: `--config`/`-c` (path to YAML config, required), `--input`/`-i` (input text, optional), `--output`/`-o` (path to save output, optional; prints to stdout if missing), `--verbose`/`-v` (optional)
  - When `--input` is not provided, MUST prompt interactively with: "Enter input for Paladin: "
- scope: agent run command, CLI options

## REQ-cli-agent-new
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-5, FR-7)
- description: `paladin agent new` template generation.
- acceptance:
  - Options: `--name`/`-n` (required), `--output`/`-o` (required), `--provider` (openai | deepseek | anthropic; optional, defaults to openai)
  - Generated YAML template MUST include commented examples for all configuration options, system prompt placeholder with guidance, LLM provider configuration section, Garrison (memory) configuration example, Arsenal (tools) configuration example, all available configuration parameters documented
- scope: agent new command, template generation

## REQ-cli-battalion-run
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-8, FR-9)
- description: `paladin battalion run` multi-Paladin execution.
- acceptance:
  - Options: `--config`/`-c` (required), `--type`/`-t` (formation | phalanx | campaign | chain-of-command, required), `--output`/`-o` (optional), `--verbose`/`-v` (optional)
  - MUST validate that the Battalion type matches the structure defined in the configuration file
- scope: battalion run command, Battalion type validation

## REQ-cli-battalion-new
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-10)
- description: `paladin battalion new` Battalion template generation.
- acceptance:
  - Options: `--name`/`-n` (required), `--type`/`-t` (formation | phalanx | campaign | chain-of-command, required), `--output`/`-o` (required)
- scope: battalion new command, template generation

## REQ-cli-arsenal-list
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-11)
- description: `paladin arsenal list` tool discovery.
- acceptance:
  - MUST display for all configured MCP tools: tool name, tool description, tool type (stdio, sse), connection status
- scope: arsenal list command, tool discovery

## REQ-cli-arsenal-test
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-12, FR-13)
- description: `paladin arsenal test` MCP connectivity verification.
- acceptance:
  - Options: `--mcp-stdio <command>` and `--mcp-sse <endpoint>`, mutually exclusive
  - Output MUST include connection status (success/failure), list of available tools from the server, basic tool schema information, connection time/latency
- scope: arsenal test command, MCP connectivity

## REQ-cli-config-format
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-14, FR-15, FR-16)
- description: YAML-only configuration schema for CLI-driven Paladins and Battalions.
- acceptance:
  - All Paladin and Battalion configurations MUST be in YAML format only (no JSON or TOML — NG-4)
  - Paladin YAML MUST support: name (required), system_prompt (required), model (required), temperature (optional, default 0.7), max_loops (optional, default 3), timeout_seconds (optional, default 300), stop_words (optional list), provider.type (openai|deepseek|anthropic, required, API key from environment), garrison.type (in_memory|sqlite, optional) with garrison.config, arsenal.mcp_servers list (name, type stdio|sse, command, args, endpoint)
  - Battalion YAML MUST support type-specific structure (formation, phalanx, campaign, chain-of-command), reference to Paladin config files or inline Paladin definitions, and execution parameters specific to the Battalion type
- scope: CLI YAML schema, Paladin config, Battalion config

## REQ-cli-env-vars
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-17, FR-18)
- description: API key loading from environment.
- acceptance:
  - MUST load provider API keys from OPENAI_API_KEY, DEEPSEEK_API_KEY, ANTHROPIC_API_KEY
  - If a required API key is missing, MUST fail with: "Missing API key: <KEY_NAME>. Please set the environment variable."
  - No encrypted config file support and no keychain/secret manager integration (NG-2, NG-10)
- scope: environment variables, API key handling

## REQ-cli-validation-errors
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-19, FR-20, FR-21)
- description: Configuration validation, error message quality, and exit codes.
- acceptance:
  - MUST validate configuration files before execution and report: invalid YAML syntax with line/column numbers, missing required fields with field names, invalid field values with expected format, file-not-found errors with file paths
  - All error messages MUST state what went wrong, why it is a problem, and how to fix it when possible
  - Exit codes MUST be: 0 success, 1 user errors (invalid config, missing required args), 2 runtime errors (LLM failures, tool failures), 130 SIGINT (Ctrl+C)
  - Configuration loading order: load YAML from `--config`, deserialize into domain config types, load environment variables for API keys, validate all configuration, fail fast with clear errors
- scope: CLI validation, error messages, exit codes

## REQ-cli-output-formatting
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-22, FR-23, FR-24)
- description: CLI output rendering, file output, and verbose mode.
- acceptance:
  - Human-readable output MUST include Paladin name, input provided, final output/response, execution time, token usage (if available)
  - When `--output` is specified, MUST save results to file in structured JSON containing all execution metadata, full conversation history, tool calls and results (if any), timestamps
  - `--verbose` MUST output each reasoning loop iteration, tool calls and results, LLM requests and responses, timing information for each step
  - Output formatting uses the Herald formatter (Epic 8 dependency)
- scope: CLI output, JSON output file, verbose mode

## REQ-cli-interactive-mode
- source: /workspace/.project/Milestone_1-MVP/Epic_9/prd-armory-cli-tools.md (FR-25, FR-26)
- description: Interactive prompting for missing required arguments.
- acceptance:
  - When required arguments are missing, MUST prompt the user interactively, display helpful context about what is being requested, and allow cancellation with Ctrl+C
  - Interactive prompts MUST validate user input and re-prompt on invalid input with guidance
  - No REPL or interactive shell (NG-7)
- scope: interactive prompts, CLI UX

---

## REQ-integration-testing
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-INT-1 through FR-INT-8)
- description: Integration test suite for end-to-end validation.
- acceptance:
  - MUST cover end-to-end Paladin execution with real LLM interactions (test API keys or mocks)
  - MUST include Battalion integration tests validating Formation, Phalanx, Campaign, Chain of Command with multiple Paladins
  - MUST provide MCP server integration tests validating both STDIO and SSE adapter connections
  - MUST include provider integration tests for OpenAI, DeepSeek, Anthropic (configurable via feature flags)
  - MUST implement load testing for concurrent Phalanx execution measuring throughput, latency, resource usage
  - Integration tests MUST be feature-flag gated: `cargo test --features integration-tests`, while unit tests run without external dependencies
  - MUST provide a `make test-integration-docker` command that starts Redis and MinIO and runs all integration tests
  - Integration test coverage MUST reach >= 70% of critical paths (Paladin execution, Battalion orchestration, tool invocation)
- scope: integration testing, feature flags, load testing, coverage

## REQ-performance-benchmarking
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-PERF-1 through FR-PERF-4)
- description: Performance baselines and Criterion benchmarks.
- acceptance:
  - MUST establish baselines for throughput (requests/sec single Paladin and Phalanx), latency (P50, P95, P99 for synchronous operations), resource efficiency (memory and CPU under load)
  - MUST provide Criterion benchmarks for: Paladin execution loop (with mocked LLM calls), Battalion pattern execution (Formation, Phalanx, Campaign), Garrison memory operations (add, retrieve, search), Arsenal tool invocation overhead
  - MUST generate benchmark reports comparing debug vs release optimization levels
  - MUST document acceptable performance thresholds for production workloads
  - Baseline targets: single Paladin throughput >= 10 req/sec, Phalanx parallel speedup >= 2x vs Formation, P95 latency (single agent) < 2 seconds, memory per Paladin < 50 MB, startup time < 500 ms
- scope: Criterion benchmarks, performance baselines, throughput, latency

## REQ-api-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-API-1 through FR-API-4)
- description: rustdoc API documentation completeness.
- acceptance:
  - All public types, traits, functions MUST have rustdoc comments with purpose description, parameter explanations, return value descriptions, example code (doc tests), link references to related types
  - MUST generate and publish rustdoc HTML via `cargo doc --no-deps --document-private-items`
  - MUST provide module-level documentation (`//!`) explaining each layer's purpose (core, application, infrastructure)
  - Doc tests MUST compile and pass as part of `cargo test`
- scope: rustdoc, doc tests, API reference

## REQ-user-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-USER-1 through FR-USER-6)
- description: Progressive user-facing documentation set.
- acceptance:
  - Getting Started tutorial covering installation from source and crates.io, basic Paladin configuration, first agent execution, expected output explanation, troubleshooting common issues; target time to first working agent < 15 minutes
  - Paladin Configuration Guide covering system prompt best practices, model selection, temperature and parameter tuning, stop word configuration, timeout and retry settings
  - Battalion Patterns Cookbook with one example per pattern (Formation, Phalanx, Campaign, Chain of Command) plus a decision matrix for pattern selection
  - Tool Integration Guide covering Arsenal/Armament concepts, MCP STDIO and SSE integration, custom tool development, tool result handling
  - Examples Gallery with runnable code for single Paladin with different providers, each Battalion pattern, Garrison usage (in-memory and persistent), Arsenal tool integration, Herald output formatting, Citadel state persistence and recovery
  - All examples MUST be executable via `cargo run --example <name>` and include README explanations
- scope: quickstart, guides, cookbook, examples gallery

## REQ-architecture-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-ARCH-1 through FR-ARCH-4)
- description: Architecture documentation and diagrams.
- acceptance:
  - MUST provide system overview diagrams for the three-layer hexagonal architecture, domain model relationships (Paladin, Battalion, Garrison, Arsenal), and data flow through layers
  - MUST document the port/adapter mapping showing which adapters implement which ports
  - MUST provide dependency flow diagrams showing allowed and prohibited import directions
  - MUST document all major design patterns used (Builder, Repository, Port/Adapter, Node<T>)
- scope: architecture docs, diagrams, port/adapter mapping

## REQ-deployment-artifacts
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-DEPLOY-1 through FR-DEPLOY-4)
- description: Cloud-native deployment artifacts and guidance.
- acceptance:
  - Docker images MUST have multi-architecture support (amd64, arm64), minimal base images (distroless or alpine), clear semantic versioning strategy, publication to Docker Hub or GitHub Container Registry
  - Kubernetes manifests MUST include Deployment with replica configuration, Service definitions, ConfigMap for configuration, Secret management examples (API keys), resource requests and limits
  - GitHub Actions workflow examples MUST cover building and testing on PR, publishing Docker images on release, running integration tests in CI, automated deployment to staging/production
  - Production deployment best practices MUST cover environment configuration (dev, staging, prod), secret management (HashiCorp Vault, AWS Secrets Manager), horizontal scaling strategies, load balancing configuration, health check implementation, graceful shutdown handling
  - Deployment targets: Docker image size < 500 MB, Docker build time < 5 minutes, Kubernetes pod startup < 30 seconds, CI/CD pipeline success rate > 95%
- scope: Docker, Kubernetes, GitHub Actions, production deployment

## REQ-operations-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-OPS-1 through FR-OPS-4)
- description: Operations and SRE documentation.
- acceptance:
  - Logging documentation MUST cover log level settings (RUST_LOG), structured logging format (JSON for production), log aggregation setup (ELK, Splunk), sensitive data redaction
  - Metrics documentation MUST cover Prometheus-compatible metrics endpoints, key metrics to monitor (request rate, error rate, latency, resource usage), Grafana dashboard examples
  - MUST document common error scenarios with description, likely causes, resolution steps, prevention strategies
  - Performance tuning guide MUST cover optimal Paladin configuration for throughput vs latency, Battalion sizing recommendations, Garrison memory limits, connection pooling settings
- scope: logging docs, metrics, troubleshooting, performance tuning

## REQ-contribution-documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (FR-CONTRIB-1 through FR-CONTRIB-3)
- description: Contributor and extension documentation.
- acceptance:
  - Contributor guide MUST document development environment setup, running tests locally, code style guidelines (rustfmt, clippy), PR submission process, review criteria
  - MUST document how to implement custom adapters: LLM provider adapter tutorial, Arsenal/MCP tool adapter tutorial, Garrison storage adapter tutorial, testing requirements for adapters
  - MUST document extension points and plugin architecture (if applicable)
- scope: contributing docs, adapter development, extension points

## REQ-epic10-quality-gates
- source: /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md (section 7.4 Quality Gates, section 11 Acceptance Criteria)
- description: Quality gates that must pass before Epic 10 is complete.
- acceptance:
  - `cargo fmt --check` passes; `cargo clippy -- -D warnings` passes with zero warnings; `cargo audit` shows no high/critical vulnerabilities
  - `cargo test` passes (all unit tests); `cargo test --features integration-tests` passes; `cargo test --all-features` passes
  - Unit test coverage >= 80% via cargo-llvm-cov (see REQ-test-coverage-target-v1 / -v2); integration test coverage >= 70%
  - `cargo doc --no-deps` generates without warnings; all doc tests pass; all markdown files pass linting; all examples compile and run successfully; no broken links
  - `cargo bench` completes successfully; baseline metrics documented; no performance regressions from previous runs
  - Docker images build successfully for amd64 and arm64; Kubernetes manifests deploy successfully to a test cluster; GitHub Actions workflows execute successfully
  - Code review completed and approved; documentation review completed; integration tests verified in CI
- scope: quality gates, CI checks, Epic 10 acceptance

---

## REQ-test-coverage-target-v1
- source: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md; /workspace/.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md; /workspace/.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md; /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md; /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md; /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md; /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md; /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md; /workspace/.project/Milestone_1-MVP/Epic_10/prd-epic10-validation-documentation.md
- description: Project-wide test coverage target as stated by the nine Epic PRDs. COMPETING VARIANT — see REQ-test-coverage-target-v2.
- acceptance:
  - Unit test coverage >= 80%
  - Integration test coverage >= 70%
  - Measured via cargo-llvm-cov
- scope: test coverage target, quality gate

## REQ-test-coverage-target-v2
- source: /workspace/.project/Milestone_1-MVP/unit-test-improvements/prd-improve-unit-test-coverage.md
- description: Project-wide test coverage target as stated by the coverage-improvement PRD. COMPETING VARIANT — see REQ-test-coverage-target-v1.
- acceptance:
  - Overall code coverage MUST exceed 85% as measured by `cargo llvm-cov`
  - Functions with low coverage (< 50%) MUST reach at least 80% coverage in those areas
  - Stated baseline: 67.79%; target timeline 1-2 weeks
- scope: test coverage target, quality gate

## REQ-unit-test-gap-closure
- source: /workspace/.project/Milestone_1-MVP/unit-test-improvements/prd-improve-unit-test-coverage.md (Functional Requirements 1-8)
- description: Closing unit test coverage gaps across the codebase.
- acceptance:
  - MUST add unit tests for all functions and methods currently at 0% coverage (e.g. main.rs, user-related modules, certain infrastructure adapters)
  - MUST add unit tests for functions with low coverage (< 50%) to reach at least 80% in those areas
  - MUST ensure all critical paths in core business logic (Paladin execution, Battalion orchestration) are covered by unit tests
  - MUST use Rust's built-in testing framework (`#[test]`, `#[cfg(test)]`)
  - MUST run `cargo test` successfully after adding new tests with no failures
  - MUST maintain existing coverage levels in already well-covered modules (e.g. ports at 100%)
  - MUST add tests for error handling paths and edge cases in uncovered functions
  - MUST use appropriate mocking and test doubles for external dependencies to isolate unit tests
  - Out of scope: integration/E2E tests, performance/benchmark tests, refactoring solely for testability, tests for third-party or generated code, property-based testing or fuzzing, build/CI changes beyond coverage reporting
- scope: unit test coverage, cargo llvm-cov, test doubles

---

# Requirements — Ingest run 2 (Milestone 2 + Milestone 3)

Ingest run 2 of 5 — source set: `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`. 15 PRDs consumed (30 DOCs went to `context.md`).

MODE=merge. Run-1 entries above are unchanged. Where a run-2 PRD supersedes or
competes with a run-1 requirement, the run-2 entry says so explicitly and the
run-1 entry is left intact — later positions do NOT overwrite earlier ones.

IDs marked `-v1` / `-v2` / `-v3` are competing variants preserved verbatim from
different PRDs on the same scope. They are NOT merged. See
`.planning/INGEST-CONFLICTS.md` WARNINGS for the resolution each needs.

---

## REQ-embedding-port
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.1, FR 1-6)
- description: Port-based abstraction for pluggable text embedding providers.
- acceptance:
  - `EmbeddingPort` trait MUST be defined in `src/application/ports/output/embedding_port.rs`
  - Trait MUST include async `embed_text()`, `embed_batch()`, and sync `dimension()`, `model_name()`
  - `Embedding` struct MUST contain vector, model metadata, and token count
  - `EmbeddingError` MUST cover NetworkError, RateLimited, InvalidInput, ProviderError
  - MUST support async batch embedding generation for efficiency
  - MUST validate embedding dimensions match the configured model
  - Unit tests for error handling and trait contract; documentation with usage examples
- scope: EmbeddingPort, Embedding, EmbeddingError, application ports/output layer

## REQ-openai-embedding-adapter
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.2, FR-3, FR-5)
- description: OpenAI-backed implementation of EmbeddingPort.
- acceptance:
  - `OpenAIEmbeddingAdapter` MUST implement `EmbeddingPort` in `src/infrastructure/adapters/llm/openai_embedding_adapter.rs`
  - MUST support `text-embedding-3-small` (1536 dims, default), `text-embedding-3-large` (3072), `text-embedding-ada-002` (1536, legacy)
  - MUST be configurable via `OpenAIEmbeddingConfig` (api_key, model, base_url, max_retries, timeout_seconds)
  - MUST implement exponential backoff retry for rate limits
  - Batch processing MUST respect the API limit of max 2048 inputs per request
  - Feature flag `openai-embeddings`, enabled by default
  - Integration test with mocked HTTP responses
- scope: OpenAIEmbeddingAdapter, OpenAIEmbeddingConfig, embedding provider

## REQ-sanctum-port
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.3, FR 7-14)
- description: Port abstraction for vector storage and semantic search.
- acceptance:
  - `SanctumPort` trait MUST be defined in `src/application/ports/output/sanctum_port.rs`
  - MUST support `store()`, `store_batch()`, `search()`, `delete()`, `update()`, `count()`
  - `SanctumQuery` MUST carry filtering, top-k and min_score parameters
  - `SanctumSearchResult` MUST return entries with similarity scores (0.0 - 1.0)
  - `SanctumFilter` MUST support metadata-based filtering (paladin_id, memory_type, date ranges)
  - MUST be thread-safe (`Send + Sync`)
  - `SanctumError` MUST cover StorageError, SearchError, InvalidDimension, NotFound, ConfigError
- scope: SanctumPort, SanctumQuery, SanctumFilter, SanctumSearchResult, SanctumError

## REQ-qdrant-sanctum-adapter-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.4, FR-8, section 7.5)
- description: Production Qdrant vector store adapter as scoped by the Epic 11 PRD. COMPETING VARIANT — see REQ-qdrant-sanctum-adapter-v2.
- acceptance:
  - `QdrantSanctumAdapter` MUST implement `SanctumPort` in `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`
  - MUST use the official Qdrant Rust client
  - Connection configurable by host, port (6334), api_key, collection, `use_grpc: true`
  - Collection auto-creation with configurable indexing parameters
  - MUST support metadata filtering via Qdrant filter syntax; connection pooling and retry logic
  - Feature flag `qdrant` (optional dependency)
  - Performance: < 500ms for top-10 searches on 100K vectors
  - Collection name `paladin_memories_{environment}`, vector dimension 1536 (configurable), Cosine distance, indexed fields paladin_id / memory_type / created_at / importance
  - Integration tests with a Docker Compose Qdrant container
- scope: QdrantSanctumAdapter, Qdrant configuration, Epic 11 scope

## REQ-qdrant-sanctum-adapter-v2
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (US-12.1, FR-1.1 to FR-1.10, section 6.4)
- description: Production Qdrant vector store adapter as scoped by the Epic 12 PRD. COMPETING VARIANT — see REQ-qdrant-sanctum-adapter-v1.
- acceptance:
  - `QdrantSanctum` struct MUST be implemented in `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs`
  - Connection configuration: URL (`http://localhost:6333`), optional API key, collection_name, vector_size 1536, distance Cosine, on_disk true
  - MUST implement store (upsert with metadata), search (cosine), delete by ID, update, count
  - MUST support payload filtering using Qdrant filter syntax
  - MUST implement a health check verifying the collection exists and is accessible
  - MUST map connection errors to `SanctumError` variants
  - Dependency `qdrant-client = "1.7"`
  - Integration test using a Qdrant Docker container
- scope: QdrantSanctum, Qdrant configuration, Epic 12 scope

## REQ-in-memory-sanctum
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.5, FR-9, FR-19, FR-22)
- description: Dependency-free in-memory vector store for development and testing.
- acceptance:
  - `InMemorySanctum` MUST implement `SanctumPort` in `src/infrastructure/adapters/sanctum/in_memory_adapter.rs`
  - MUST use brute-force cosine similarity (acceptable for < 10K vectors)
  - MUST be thread-safe via `Arc<RwLock<HashMap<String, SanctumEntry>>>`
  - MUST support configurable max capacity with LRU eviction
  - MUST support all CRUD operations from `SanctumPort`
  - Performance: < 100ms for searches on 10K vectors
  - No external dependencies (always available); unit tests for all operations
- scope: InMemorySanctum, in-memory vector store, LRU eviction

## REQ-sanctum-domain-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (US-11.6, FR 15-18, FR-30)
- description: Core domain model for long-term memory.
- acceptance:
  - Domain types MUST live in `src/core/platform/container/sanctum.rs`
  - `Memory` MUST carry id, paladin_id, content, memory_type, importance, access_count, timestamps, metadata
  - `MemoryType` MUST be Episodic (conversations), Semantic (facts), Procedural (how-to)
  - `MemoryDecayStrategy` MUST be NoDecay, LinearDecay, AccessBasedDecay, CustomDecay
  - `SanctumEntry` MUST combine memory + embedding + serialization
  - MUST assign importance scores 0.0-1.0 and track access count / last accessed
  - MUST validate embedding dimensions against the configured model; Serde support; builder pattern
  - MUST use the existing `Node<T>` pattern for domain entities
- scope: Memory, MemoryType, MemoryDecayStrategy, SanctumEntry, core domain layer

## REQ-sanctum-configuration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (FR 24-27, section 6.3)
- description: Configuration surface for Sanctum storage, embedding and memory defaults.
- acceptance:
  - MUST allow configuration of embedding provider (model, API keys, base URL)
  - MUST allow selection of vector database (qdrant vs in_memory)
  - MUST support environment variable substitution for sensitive values
  - MUST validate configuration at startup with clear error messages
  - `config.yml` MUST carry `sanctum.storage`, `sanctum.embedding`, `sanctum.memory` sections; memory defaults `default_importance: 0.5`, `decay_strategy: access_based` with per-Paladin override
- scope: Sanctum configuration, application_settings, config.yml

## REQ-sanctum-garrison-coexistence
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md (FR 28-29, FR-31)
- description: Sanctum and Garrison operate as complementary, independently optional memory systems.
- acceptance:
  - Garrison remains short-term memory; Sanctum is long-term memory; both usable simultaneously
  - Paladins MUST be able to opt out of Sanctum entirely (backward compatible)
  - MUST follow hexagonal layering core -> application -> infrastructure
  - Automatic Garrison-to-Sanctum migration is explicitly out of scope
- scope: Garrison/Sanctum boundary, backward compatibility

## REQ-paladin-builder-sanctum-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (US-12.2, FR-3.1 to FR-3.5)
- description: PaladinBuilder extensions for long-term memory configuration.
- acceptance:
  - `PaladinBuilder::with_sanctum(Arc<dyn SanctumPort>)` MUST be added
  - `PaladinBuilder::with_embedding_port(Arc<dyn EmbeddingPort>)` MUST be added
  - `PaladinBuilder::memory_extraction_strategy(MemoryExtractionStrategy)` MUST be added
  - Builder MUST validate that an embedding port is present whenever a sanctum is configured
  - RAG configuration MUST be stored in `PaladinConfig` or `PaladinData`
- scope: PaladinBuilder, Sanctum wiring, RAG configuration

## REQ-memory-extraction-strategy
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (FR-4.1 to FR-4.5)
- description: Configurable trigger policy for writing memories to Sanctum.
- acceptance:
  - `MemoryExtractionStrategy` MUST define `EveryTurn`, `OnCompletion`, `Manual`, `Threshold { importance: f32 }`
  - Default MUST be `OnCompletion`
  - `OnCompletion` MUST trigger extraction when `Paladin::run()` completes successfully
  - `Manual` MUST require an explicit `extract_memories()` call
  - `Threshold` MUST extract only when the importance score exceeds the configured value
- scope: MemoryExtractionStrategy, memory write policy

## REQ-rag-retrieval-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (US-12.3, FR-5.1 to FR-5.11)
- description: Service that retrieves and formats relevant long-term memories before an LLM call.
- acceptance:
  - `RagRetrievalService` MUST live in `src/application/use_cases/sanctum/rag_retrieval_service.rs`
  - MUST expose `retrieve_context(paladin_id: &str, query: &str) -> Result<Vec<Memory>, SanctumError>`
  - MUST generate the query embedding via the configured `EmbeddingPort` and call `SanctumPort::search()` with top_k
  - MUST filter by `min_similarity` (default 0.7) and deduplicate memories with > 0.95 mutual similarity
  - MUST rank by relevance descending and truncate to the `max_tokens` budget by dropping lowest-scoring memories
  - MUST expose `format_for_prompt(&[Memory]) -> String`
  - MUST run asynchronously with a 5-second timeout and return an empty Vec on failure/timeout (graceful degradation)
- scope: RagRetrievalService, retrieval, deduplication, token budget

## REQ-rag-config
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (FR-6.1 to FR-6.5, section 6.4)
- description: Configuration structure governing RAG retrieval behaviour.
- acceptance:
  - `RagConfig` MUST define `top_k`, `min_similarity`, `max_tokens`, `retrieval_trigger`
  - Defaults MUST be `top_k: 5`, `min_similarity: 0.7`, `max_tokens: 2000`
  - `retrieval_trigger` MUST support `Always`, `KeywordBased`, `SemanticThreshold`; default `Always`
  - MUST be configurable via YAML and via builder methods; YAML also carries `timeout_seconds: 5`
- scope: RagConfig, RetrievalTrigger, configuration

## REQ-memory-extraction-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (US-12.4, FR-7.1 to FR-7.9)
- description: LLM-driven extraction of memorable content from a conversation.
- acceptance:
  - `MemoryExtractionService` MUST live in `src/application/use_cases/sanctum/memory_extraction_service.rs`
  - MUST expose `extract_memories(paladin_id: &str, conversation: &[GarrisonEntry]) -> Result<Vec<Memory>, SanctumError>`
  - MUST build an extraction prompt asking the LLM to identify memorable content (facts, preferences, events, instructions)
  - MUST parse the LLM response into structured `Memory` objects with content, type and importance 0.0-1.0
  - MUST generate embeddings for extracted memories and skip duplicates detected at > 0.95 similarity (no merging)
  - MUST store new memories via `SanctumPort::store()`
  - MUST log extraction metrics: count, average importance, duration
- scope: MemoryExtractionService, memory extraction, deduplication

## REQ-execution-service-rag-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (US-12.5, FR-8.1 to FR-8.8, FR-10.1 to FR-10.4)
- description: Wiring of retrieval and extraction into the Paladin execution flow.
- acceptance:
  - `PaladinExecutionService::execute()` MUST check for Sanctum configuration and, when present, call `RagRetrievalService::retrieve_context()` before the LLM call
  - Retrieved memories MUST be injected into the system prompt under a `## Relevant Context` section
  - On retrieval failure or timeout the service MUST log a warning and continue with empty context (non-fatal)
  - After successful execution the configured extraction strategy MUST be evaluated; `OnCompletion` triggers `MemoryExtractionService::extract_memories()` asynchronously without blocking the response
  - MUST collect metrics: retrieval_latency_ms, memories_retrieved_count, extraction_triggered
  - Extraction failures MUST be logged and MUST NOT affect the Paladin response
  - `SanctumError` MUST include ConnectionError, QueryError, StorageError, EmbeddingError
- scope: PaladinExecutionService, RAG integration, graceful degradation, metrics

## REQ-rag-performance-targets
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/prd-sanctum-rag-integration.md (section 7.2, 8.2)
- description: Non-functional targets for the RAG pipeline.
- acceptance:
  - Retrieval latency < 500ms p95 for collections under 100k vectors
  - Extraction latency < 3 seconds p95 for conversations under 10 messages
  - Memory overhead < 100MB for an in-memory store with 10k vectors
  - MUST support 10+ concurrent Paladin executions sharing one Sanctum
  - Memory retrieval hit rate > 80%; zero degradation in execution time when retrieval fails
- scope: RAG performance, scalability

---

## REQ-vision-content-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.1, FR-1, section 6)
- description: Type-safe multi-modal request model for images. SUPERSEDED IN PART by REQ-vision-response-model (Epic 20) — both preserved.
- acceptance:
  - `VisionContent` enum MUST provide `ImageUrl { url, detail }`, `ImageBase64 { data, media_type, detail }`, `ImageFile { path, detail }`
  - `ImageDetail` MUST provide Auto, Low, High for quality control
  - `VisionRequest` MUST carry `text: String` and `images: Vec<VisionContent>`
  - MUST allow multiple images in a single request
  - Image metadata MUST include format, size and dimensions when available
  - Types MUST live in `src/core/platform/container/vision.rs`
- scope: VisionContent, ImageDetail, VisionRequest, core domain layer

## REQ-vision-format-validation-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-1.2, US-13.1, success criteria)
- description: Framework-side image format validation. COMPETING VARIANT — see REQ-vision-format-validation-v2.
- acceptance:
  - System MUST validate image formats and accept only PNG, JPEG, GIF, WebP
  - Validation MUST enforce supported formats before the request is dispatched
  - `VisionError::UnsupportedFormat` and `VisionError::FileTooLarge { size, max }` MUST be returned for rejected inputs
  - CLI MUST provide clear error messages for unsupported formats
- scope: image format validation, VisionError, Epic 13 position

## REQ-vision-format-validation-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (US-20.1, US-20.2, NG-3, NG-5)
- description: Provider-side image format validation. COMPETING VARIANT — see REQ-vision-format-validation-v1.
- acceptance:
  - Adapters MUST delegate image format validation to the OpenAI / Anthropic API and support all formats the provider accepts
  - Adapters MUST NOT convert image formats; conversion is the caller's responsibility
  - Adapters MUST NOT preprocess images (no resizing, cropping, filtering); images are sent as-is
  - Image size validation MUST be delegated to the provider (OpenAI ~20MB per image; Anthropic varies by model), documented in `docs/SENTINEL.md`
- scope: image format validation, adapter responsibility, Epic 20 position

## REQ-openai-vision-adapter-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.2, FR-2)
- description: OpenAI vision support expressed as an extension of the existing LLM adapter. COMPETING VARIANT — see REQ-openai-vision-adapter-v2.
- acceptance:
  - `OpenAILlmAdapter` MUST be extended to support vision requests
  - MUST support `gpt-4-vision-preview`, `gpt-4o`, `gpt-4o-mini`
  - MUST convert `VisionContent` to OpenAI message format, handling both URLs and base64
  - MUST handle image token counting for context limit management
  - MUST use HTTPS for all API communication and retry transient failures with exponential backoff
  - MUST respect token limits for image processing
- scope: OpenAILlmAdapter vision, Epic 13 position

## REQ-openai-vision-adapter-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (US-20.1, FR-1.1 to FR-1.6)
- description: Dedicated OpenAI vision adapter making real HTTP calls. COMPETING VARIANT — see REQ-openai-vision-adapter-v1.
- acceptance:
  - `OpenAIVisionAdapter::analyze_image()` MUST POST to `https://api.openai.com/v1/chat/completions` from `src/infrastructure/adapters/llm/openai_vision.rs`
  - Request MUST include model, messages (system prompt + user text + image content), configurable `max_tokens`, `Authorization: Bearer` and `Content-Type` headers
  - Image content MUST support `{"type":"image_url","image_url":{"url":"https://..."}}` and the `data:image/...;base64,` form
  - MUST parse 200 responses for `choices[0].message.content`, `usage`, and `model`, returning a `VisionResponse`
  - MUST map 400 -> `VisionError::InvalidImage`, 401 -> `AuthenticationError`, 429 and 5xx -> retry with backoff
  - Retry MUST read `max_retries` (default 3), `initial_backoff_ms`, `backoff_multiplier` from configuration, using `initial_backoff_ms * (backoff_multiplier ^ attempt)`, and MUST NOT retry 400/401/403/404
  - Unit tests with mocked HTTP for success and every error case; integration tests gated by `ENABLE_VISION_TESTS=true`
- scope: OpenAIVisionAdapter, retry policy, Epic 20 position

## REQ-anthropic-vision-adapter-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.3, FR-3)
- description: Anthropic vision support as an LLM adapter extension. COMPETING VARIANT — see REQ-anthropic-vision-adapter-v2.
- acceptance:
  - `AnthropicLlmAdapter` MUST be extended to support vision requests
  - MUST support Claude 3 Opus, Sonnet and Haiku
  - MUST automatically convert image URLs to base64 (Anthropic requirement)
  - MUST handle Anthropic content block format for images and multiple images per request
  - MUST use HTTPS and implement appropriate rate limiting
- scope: AnthropicLlmAdapter vision, Epic 13 position

## REQ-anthropic-vision-adapter-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (US-20.2, FR-2.1 to FR-2.6)
- description: Dedicated Anthropic vision adapter making real HTTP calls. COMPETING VARIANT — see REQ-anthropic-vision-adapter-v1.
- acceptance:
  - `AnthropicVisionAdapter::analyze_image()` MUST POST to `https://api.anthropic.com/v1/messages` from `src/infrastructure/adapters/llm/anthropic_vision.rs`
  - Request MUST include model (e.g. `claude-3-opus-20240229`, `claude-3-sonnet-20240229`), configurable `max_tokens`, user-role messages, and headers `x-api-key`, `anthropic-version: 2023-06-01`, `Content-Type`
  - Image content MUST use `{"type":"image","source":{"type":"base64"|"url","media_type":...,"data"|"url":...}}`
  - MUST parse 200 responses for `content[0].text`, `usage`, `model`
  - MUST map 400 -> InvalidImage, 401 -> AuthenticationError, 429 and 5xx -> retry with backoff, following the same configurable retry contract as the OpenAI adapter
  - Unit tests with mocked HTTP; integration tests gated by `ENABLE_VISION_TESTS=true`
- scope: AnthropicVisionAdapter, retry policy, Epic 20 position

## REQ-vision-capable-llm-trait
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-4)
- description: Trait-level capability detection for vision. COMPETING with the Epic 20 `VisionPort` position — see REQ-vision-port.
- acceptance:
  - System MUST define a `VisionCapableLlm` trait extending `LlmPort`
  - Trait MUST include `generate_with_vision()`
  - Trait MUST include `supports_vision() -> bool`
  - Adapters for non-vision models MUST return `false` from `supports_vision()`
- scope: VisionCapableLlm, LlmPort extension, Epic 13 position

## REQ-vision-port
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (DC-1, DC-2, FR-5.2)
- description: Application-layer vision port implemented by provider adapters. COMPETING with REQ-vision-capable-llm-trait.
- acceptance:
  - OpenAI and Anthropic vision adapters MUST both implement the same `VisionPort` trait
  - `VisionPort` MUST be defined in the application layer; adapters are infrastructure
  - Core domain types (`VisionImage`, `VisionResponse`) MUST live in the core layer
  - All adapter methods MUST return `Result<VisionResponse, VisionError>` and be `async`
  - Error handling and retry logic MUST be consistent across providers (provider parity)
- scope: VisionPort, hexagonal boundaries, Epic 20 position

## REQ-paladin-vision-api-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.4, FR-5)
- description: Paladin-level vision entry point as scoped by Epic 13. COMPETING VARIANT — see REQ-paladin-vision-api-v2.
- acceptance:
  - `Paladin::run_with_vision(task, images)` MUST be available
  - `PaladinBuilder::enable_vision(bool)` MUST be provided
  - System MUST validate the LLM adapter supports vision before execution and return a clear error otherwise
  - MUST support mixing text and image inputs in a single request
- scope: Paladin::run_with_vision, PaladinBuilder::enable_vision, Epic 13 position

## REQ-paladin-vision-api-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (US-20.3, FR-3.1 to FR-3.8)
- description: Execution-service vision entry point as scoped by Epic 20. COMPETING VARIANT — see REQ-paladin-vision-api-v1.
- acceptance:
  - `PaladinExecutionService::execute_with_vision(paladin, prompt, images: Vec<VisionImage>)` MUST build a multimodal prompt from the Paladin system prompt, user text and image references
  - Provider MUST be derived from `paladin.model()` (gpt-* -> OpenAI, claude-* -> Anthropic); same provider for text and vision; `VisionError::UnsupportedProvider` otherwise
  - Vision execution MUST be non-streaming: the complete analysis is parsed before continuing
  - MUST respect `max_loops` (vision analysis counts as one iteration), `stop_words` and `timeout_seconds`
  - MUST store prompt, images, analysis result, timestamp and metadata in Garrison when configured
  - MUST return a `VisionResult` with analysis text, token usage, model used and execution metadata (duration, loops used)
  - `VisionError` MUST convert to `PaladinError` at the service boundary
- scope: PaladinExecutionService::execute_with_vision, provider selection, Epic 20 position

## REQ-vision-error-model-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-12, section 7)
- description: VisionError variant set as defined by Epic 13. COMPETING VARIANT — see REQ-vision-error-model-v2.
- acceptance:
  - `VisionError` MUST provide UnsupportedFormat, FileTooLarge, InvalidImage, ModelNotSupported, NetworkError, EncryptionError, IoError
  - `DocumentError` MUST provide UnsupportedFormat, EncryptedPdf, CorruptedFile, ExtractionFailed, IoError
  - All errors MUST include descriptive messages suitable for end users
- scope: VisionError, DocumentError, Epic 13 position

## REQ-vision-error-model-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (FR-5.1)
- description: VisionError variant set as defined by Epic 20. COMPETING VARIANT — see REQ-vision-error-model-v1.
- acceptance:
  - `VisionError` MUST be defined in `src/core/platform/container/sentinel/vision_types.rs`
  - Variants MUST be InvalidImage, UnsupportedFormat, AuthenticationError, RateLimitExceeded(u64), ProviderError, NetworkError, Timeout(u64), UnsupportedProvider, MaxRetriesExceeded
  - No `EncryptionError` or `FileTooLarge` variant is specified
- scope: VisionError, Epic 20 position

## REQ-vision-security-encryption
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-11, section 7 Security Implementation, success metrics)
- description: Security requirements for handling visual data. Epic 20 is silent on these — see INGEST-CONFLICTS.md.
- acceptance:
  - System MUST encrypt image data at rest whenever it is stored temporarily (aes-gcm or chacha20poly1305, unique key per session, keys held in env vars or a secrets manager)
  - System MUST use HTTPS/TLS 1.3 for all external API communication and validate SSL certificates
  - System MUST clear sensitive image data from memory after processing (Drop impl, `zeroize`)
  - System MUST support configurable data retention policies with automatic deletion
  - System MUST log security-relevant events (file access, API calls) without logging sensitive data
  - Targets: 100% of stored image/document data encrypted; 100% of API calls over TLS
- scope: encryption at rest, TLS, data retention, audit logging

## REQ-pdf-extraction
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.5, FR-6, section 6)
- description: PDF text extraction utility.
- acceptance:
  - `PdfExtractor` MUST support `extract(path)` and `extract_bytes(bytes)`
  - MUST return a `Document { pages: Vec<Page>, metadata: DocumentMetadata, total_chars }`
  - `Page` MUST carry number and content; `DocumentMetadata` MUST carry title, author, page_count, creation_date
  - MUST handle multi-page documents and preserve paragraph/spacing structure reasonably
  - MUST return a descriptive error for encrypted or malformed PDFs
  - Targets: small PDF (<10 pages) < 2s, large PDF (100+ pages) < 10s, >95% text accuracy
- scope: PdfExtractor, Document, Page, DocumentMetadata

## REQ-document-port
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (US-13.6, FR-7)
- description: Port abstraction for document ingestion and chunking.
- acceptance:
  - `DocumentPort` trait MUST be defined in `src/application/ports/input/document_port.rs`
  - MUST include `ingest(source: DocumentSource)`; `DocumentSource` MUST support File(PathBuf), Bytes, Url
  - MUST include `chunk(document, config)`; `ChunkConfig` MUST support chunk_size, chunk_overlap, separator
  - MUST support PDF, TXT and MD (DOCX deferred)
  - MUST extract metadata (title, author, date) when available
  - MUST be thread-safe and async-compatible
- scope: DocumentPort, DocumentSource, ChunkConfig, document ingestion

## REQ-vision-cli-and-yaml
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-8, FR-9)
- description: CLI and YAML surface for multi-modal inputs.
- acceptance:
  - CLI MUST support repeatable `--image <path>` and `--document <path>` flags
  - CLI MUST validate file paths exist before execution and report unsupported formats clearly
  - CLI output MUST indicate when vision/document inputs were processed
  - YAML MUST support `images: [..]`, `documents: [..]`, `vision_enabled: true|false`, plus `security.encrypt_at_rest` and `security.data_retention_hours`
  - All file paths MUST be validated during configuration loading with helpful errors
- scope: CLI vision flags, YAML vision configuration

## REQ-battalion-vision-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-10, goal 4)
- description: Vision support across all Battalion orchestration patterns. Epic 20 NG-6 narrows this — see INGEST-CONFLICTS.md.
- acceptance:
  - Formation, Phalanx, Campaign and Chain of Command MUST all support vision inputs
  - Formation MUST pass vision context sequentially between Paladins
  - Phalanx MUST support parallel processing of multiple images (e.g. `phalanx.run_with_images(vec![...])`)
  - Campaign MUST support conditional branching on vision analysis results
  - Chain of Command MUST support delegating vision tasks to specialised sub-agents
- scope: Battalion vision integration

## REQ-vision-performance-and-config
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md (FR-13, section 8) and /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md (FR-4)
- description: Vision performance targets and retry configuration.
- acceptance:
  - MUST support configurable batch sizes for parallel image processing, lazy loading of large files, optional compression/resizing, async processing and configurable timeouts (Epic 13)
  - `config.yml` MUST carry `vision.retry.{max_retries: 3, initial_backoff_ms: 1000, backoff_multiplier: 2.0}` and per-provider `max_tokens: 4096` plus model allow-lists (Epic 20)
  - Configuration MUST load into a `VisionConfig` struct in `src/config/application_settings.rs` and be injected into adapters via constructor
  - Targets: single image < 5s end to end; batch of 10 < 15s with Phalanx; vision API call < 30s typical; total retry time < 60s
- scope: vision performance, VisionConfig, retry configuration

---

## REQ-max-loops-auto
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.1, FR-1.1)
- description: MaxLoops becomes an enum with an autonomous planning variant. SUPERSEDES the scalar `max_loops` in REQ-paladin-entity and the `[1, 100]` validation in REQ-paladin-builder (run 1, Epic 1) — both preserved.
- acceptance:
  - System MUST support `MaxLoops::Auto { max_subtasks: u32 }` in addition to `MaxLoops::Fixed(u32)`
  - `PaladinBuilder::max_loops(MaxLoops)` MUST accept the enum
  - `MaxLoops::Auto` MUST route execution through `PlanningService`
  - Planning loops MUST NOT exceed `max_subtasks`
- scope: MaxLoops, PaladinData.max_loops, PaladinBuilder, autonomous planning

## REQ-planning-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.1, FR-1.2 to FR-1.8)
- description: LLM-driven task decomposition and sequential subtask execution.
- acceptance:
  - `PlanningService` MUST use the LLM to generate a `TaskPlan` from the task description
  - `TaskPlan` MUST include the original task, a list of subtasks and a dependency graph
  - Each `Subtask` MUST include id, description and expected output
  - System MUST execute subtasks in dependency order and synthesise their results into a final response
  - Planning MUST use a dedicated planning prompt template
  - System MUST log planning decisions and execution progress
  - Planning overhead target: <= 2x total execution time vs non-planning mode; >= 90% of plans decompose tasks appropriately
- scope: PlanningService, TaskPlan, Subtask, dependency ordering

## REQ-prompt-generation-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.2, FR-2.1 to FR-2.8)
- description: Automatic system-prompt generation from agent metadata.
- acceptance:
  - `PaladinBuilder` MUST support `auto_generate_prompt(bool)`, `agent_description(String)` and `regenerate_prompt()`
  - `PromptGenerationService` MUST generate prompts including role, capabilities and constraints
  - Generated prompts MUST be cached after first generation
  - A manual `system_prompt()` call after auto-generation MUST override the generated prompt
  - Generated prompts MUST be logged for review and MUST be deterministic given identical inputs
  - Target: generation <= 3s at build time; >= 85% of generated prompts need no manual override
- scope: PromptGenerationService, PaladinBuilder, prompt caching

## REQ-dynamic-temperature
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.3, FR-3.1 to FR-3.7)
- description: Task-type-driven temperature selection. Interacts with REQ-temperature-range-v1/-v2 (run 1).
- acceptance:
  - `PaladinBuilder` MUST support `dynamic_temperature(bool)` and `temperature_bounds(f32, f32)`
  - `TemperatureService` MUST classify tasks as Factual, Analytical, Conversational or Creative
  - Ranges MUST be Factual 0.1-0.3, Analytical 0.3-0.5, Conversational 0.5-0.7, Creative 0.7-1.0
  - When disabled the value from `PaladinBuilder::temperature()` (default 0.7) MUST be used
  - Selected temperature MUST respect the configured bounds and be logged with the classification reasoning
  - Classification MUST use heuristics (keywords, question type, context signals), not an LLM call, and complete in <= 50ms
- scope: TemperatureService, TaskType, dynamic temperature, temperature bounds

## REQ-handoff-infrastructure
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.4, FR-4.1 to FR-4.9)
- description: Specialist delegation infrastructure for a coordinating Paladin.
- acceptance:
  - `PaladinBuilder` MUST support `with_handoffs(Vec<Arc<Paladin>>)` and `handoff_strategy(HandoffStrategy)`
  - `HandoffStrategy` MUST support `Automatic`, `Explicit`, `Threshold { confidence: f32 }`
  - `HandoffService` MUST analyse the task and decide whether a handoff is needed, based on task complexity, agent capabilities and confidence
  - System MUST track the handoff chain to prevent circular delegation (100% of circular attempts blocked)
  - System MUST enforce a maximum handoff depth (default 5, configurable)
  - Handoff context MUST include task description, conversation history and relevant metadata
  - Handoff history MUST be included in `PaladinResult`; all handoff decisions MUST be logged with reasoning
  - Only sequential delegation is in scope; parallel handoffs are a Battalion concern
- scope: HandoffService, HandoffStrategy, HandoffDecision, handoff depth, circular detection

## REQ-handoff-tool-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (US-14.5, FR-5.1 to FR-5.8)
- description: Handoff exposed to the LLM as a tool, as named by Epic 14. COMPETING VARIANT — see REQ-handoff-tool-v2.
- acceptance:
  - A tool named `handoff_to_agent` MUST be registered automatically when handoffs are configured
  - Tool schema MUST include `agent_name` (enum of available agent names) and `message` (context for the specialist), both required
  - Tool MUST validate `agent_name` against available agents and execute the handoff via `HandoffService`
  - Tool MUST return the specialist result to the original agent for synthesis
  - Tool MUST track the handoff chain across invocations and error on circular handoffs, invalid agent names and exceeded depth
  - Tool calls MUST appear in the execution trace
- scope: handoff_to_agent tool, tool schema, Epic 14 naming

## REQ-handoff-tool-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (US-21.2, FR-3.1 to FR-3.6) and /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md (Non-Goal 5)
- description: Handoff tool contract as named by Epic 21 / Epic 23. COMPETING VARIANT — see REQ-handoff-tool-v1.
- acceptance:
  - `PaladinBuilder::build()` MUST detect a prior `with_handoffs()` call and auto-register the handoff tool in the arsenal
  - Tool schema MUST include `specialist_name` (enum of configured specialists) and `task_description` (string)
  - Tool schema MUST carry specialist names, descriptions and parameter requirements
  - Auto-registration MUST be idempotent (no duplicates, safe to call build repeatedly) and the tool MUST be removed/updated if handoffs are reconfigured
  - The Epic 23 Task 4.6 PRD refers to this tool as `handoff_to_specialist`
- scope: handoff tool auto-registration, tool schema, Epic 21/23 naming

## REQ-autonomous-configuration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (FR-6.1 to FR-6.5, section 6.2)
- description: Configuration surface and opt-in semantics for autonomous features.
- acceptance:
  - All autonomous features MUST be configurable via `PaladinConfig`
  - YAML MUST support every autonomous feature flag under `paladin.autonomous` (planning.enabled/max_subtasks, prompt_generation.enabled/description, dynamic_temperature.enabled/min/max, handoffs.enabled/strategy/max_depth/specialists)
  - CLI MUST support flags for enabling autonomous features
  - All features MUST be opt-in (disabled by default) so existing `PaladinBuilder` code is unchanged
  - Configuration validation MUST occur at build time
- scope: autonomous configuration, YAML schema, backward compatibility

## REQ-autonomous-error-handling
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md (FR-7.1 to FR-7.6)
- description: Error types and degradation policy for autonomous features.
- acceptance:
  - System MUST define `PlanningError`, `PromptError` and `HandoffError` enums
  - All errors MUST include descriptive messages and be logged with full context
  - Errors MUST degrade gracefully — e.g. fall back to non-planning mode when planning fails
  - All autonomous decisions MUST be logged at INFO level; execution traces MUST include planning, temperature and handoff decisions
- scope: PlanningError, PromptError, HandoffError, graceful degradation, observability

---

## REQ-conclave-domain-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md (US-15.1, FR-C1 to FR-C5, section 6)
- description: Domain model for the Conclave (MixtureOfAgents) Battalion pattern.
- acceptance:
  - `Conclave` MUST live in `src/core/platform/container/battalion/conclave.rs` and carry name, `experts: Vec<Paladin>`, `aggregator: Paladin`, `config: ConclaveConfig`
  - `Conclave` MUST contain at least 2 experts and exactly 1 aggregator; validation MUST reject duplicate agent names
  - `ConclaveConfig` MUST carry name, timeout_seconds (10-3600, default 300), retry_attempts (0-5, default 2), synthesis_prompt (Option), include_expert_names (bool), max_expert_output_tokens (Option), observability_level
  - `ConclaveResult` MUST carry expert_outputs (HashMap<String, PaladinResult>), aggregated_output, execution_time_ms, expert_execution_times, retry_counts, status
  - `ConclaveStatus` MUST be Success (all experts succeeded), PartialSuccess, Failed
  - `ConclaveError` MUST provide AllExpertsFailed, AggregatorFailed, ConfigurationError, Timeout and ExpertError(name, detail)
- scope: Conclave, ConclaveConfig, ConclaveResult, ConclaveStatus, ConclaveError

## REQ-conclave-execution-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md (US-15.2, FR-E1 to FR-E7, section 7)
- description: Parallel expert execution with resilient aggregation.
- acceptance:
  - `ConclaveExecutionService` MUST live in `src/application/use_cases/battalion/conclave_execution_service.rs` and execute all experts in parallel with async/await
  - Failed experts MUST be retried up to the configured limit with exponential backoff (1s, 2s, 4s, 8s, 16s) plus +/-20% jitter; only transient errors (network, timeout, rate limit) are retried
  - Execution MUST continue with the available expert outputs when some experts fail after retries; if all fail, `ConclaveError::AllExpertsFailed`
  - Expert outputs MUST be formatted into a structured aggregator prompt, optionally labelled with agent names
  - Timeout MUST apply to the entire Conclave execution (experts + aggregation), not per agent
  - `ConclaveResult` MUST indicate which experts succeeded and which failed
  - Aggregator prompt template MUST be customisable with a sensible default
  - Target: total time <= max expert time + aggregation time + 10% overhead
- scope: ConclaveExecutionService, parallel execution, retry with backoff, partial success

## REQ-conclave-commander-strategy
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md (US-15.3, FR-M1 to FR-M5)
- description: Conclave as a first-class Commander strategy.
- acceptance:
  - `BattalionStrategy::Conclave` variant MUST be added
  - `CommanderBuilder` MUST support `.aggregator(paladin)`; aggregator selection MUST also be possible via `aggregator_index` or `aggregator_name`
  - Default behaviour MUST make the last agent in the roster the aggregator and the rest experts
  - Commander MUST validate at least 2 experts and 1 aggregator before execution
  - Auto strategy MUST consider Conclave when the task contains synthesis keywords ("compare", "synthesize", "combine perspectives", "expert panel") or requires multi-perspective analysis, scored +3 keywords / +2 three-or-more distinct expertises / +1 comprehensive question
- scope: BattalionStrategy::Conclave, CommanderBuilder, auto-strategy scoring

## REQ-conclave-cli-and-yaml
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md (US-15.4, FR-I1 to FR-I6, section 6)
- description: CLI and YAML surface for Conclave.
- acceptance:
  - CLI MUST support `paladin battalion run --type conclave --config <file>`
  - CLI MUST support `paladin battalion new --type conclave --name <name>` and generate a template with 3 example experts with distinct roles plus 1 aggregator
  - YAML MUST support inline and reference-based agent definitions plus `retry_attempts`, `timeout_seconds`, `synthesis_prompt`, `include_expert_names`, `observability_level`
  - CLI MUST output both individual expert outputs and the aggregated result, in JSON, Markdown or plain text
  - YAML schema validation MUST produce helpful error messages; template generation MUST complete in < 1 second
- scope: Conclave CLI, Conclave YAML schema, template generation

## REQ-conclave-observability
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md (FR-O1 to FR-O4)
- description: Configurable observability levels for Conclave execution.
- acceptance:
  - Observability MUST be configurable as minimal, standard or verbose
  - Standard MUST include per-expert execution time, total time, retry counts and success/failure status
  - Verbose MUST additionally include full expert outputs, token usage, LLM provider details and timestamps
  - Minimal MUST include only the final aggregated result and overall status
- scope: ObservabilityLevel, Conclave logging

---

## REQ-council-domain-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (US-16.1, FR-1.1)
- description: Domain model for the Council (conversational collaboration) pattern.
- acceptance:
  - `Council` MUST be defined in `src/core/platform/container/battalion/council.rs` with name, participant Paladins, optional moderator and configuration
  - `CouncilConfig` MUST carry max_rounds, turn_strategy, termination_condition and an include_history flag
  - `CouncilMessage` MUST carry speaker name, content, round number and timestamp
  - Council is an aggregate containing CouncilConfig and CouncilMessages
- scope: Council, CouncilConfig, CouncilMessage

## REQ-council-turn-strategies
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-1.2, NG-6)
- description: Turn-taking strategies for Council discussions.
- acceptance:
  - `TurnStrategy::RoundRobin` MUST be implemented (participants speak in sequence)
  - `TurnStrategy::ModeratorDirected` MUST be implemented (moderator chooses the next speaker)
  - The enum SHOULD be prepared for future `Random` and `VoluntaryWithTimeout` variants, which are explicitly NOT implemented in this epic
  - Edge cases MUST be handled: speaker unavailable, moderator offline
- scope: TurnStrategy, turn-taking, deferred variants

## REQ-council-termination-conditions
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-1.3)
- description: Conditions that end a Council discussion.
- acceptance:
  - `TerminationCondition::MaxRounds` MUST be supported (stop after N rounds)
  - `TerminationCondition::ModeratorDecision` MUST be supported
  - `TerminationCondition::Consensus` SHOULD be supported via agreement-keyword detection
  - `TerminationCondition::Keyword(String)` SHOULD be supported for custom triggers
- scope: TerminationCondition, consensus detection

## REQ-council-execution-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (US-16.2, FR-1.4, FR-4.1)
- description: Service driving Council conversation flow.
- acceptance:
  - `CouncilExecutionService` MUST live in `src/application/use_cases/battalion/council_service.rs`
  - MUST provide `convene(council, topic)` to start a discussion
  - MUST track conversation history as an ordered list of `CouncilMessage`
  - MUST implement turn-taking per the selected strategy and evaluate the termination condition after each turn
  - MUST return `CouncilResult` with transcript, conclusion, rounds_completed and termination_reason
  - MUST handle empty participant lists, a missing moderator when required, and invalid turn-strategy configuration with clear errors
  - Participant execution failures MUST be handled gracefully by skipping to the next speaker; a per-speaker timeout MUST prevent blocking
- scope: CouncilExecutionService, CouncilResult, conversation flow

## REQ-council-garrison-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-1.5, State Persistence)
- description: Persistence of Council conversation state.
- acceptance:
  - Conversation history MUST be stored in Garrison for context continuity and retrievable for follow-up discussions
  - Conversation branching MUST be supported (multiple councils on the same topic)
  - `CouncilResult` SHOULD be stored in Citadel for recovery, with a checkpoint after each round
  - Conversation history access MUST be thread-safe; multiple Councils MUST be able to run concurrently
- scope: Council + Garrison, Council + Citadel, concurrency

## REQ-grove-domain-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (US-16.3, FR-2.1)
- description: Domain model for the Grove (tree-based routing) pattern.
- acceptance:
  - `Grove` MUST be defined in `src/core/platform/container/battalion/grove.rs` with name, `trees: Vec<Tree>` and `GroveConfig`
  - `Tree` MUST carry a name and a list of `TreeAgent`
  - `TreeAgent` MUST carry a Paladin reference, `expertise_keywords` and an optional `expertise_embedding`
  - Grove is an aggregate containing Trees and TreeAgents; ubiquitous language Grove / Tree / Moderator / Routing MUST be used
- scope: Grove, Tree, TreeAgent

## REQ-grove-routing-strategies
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-2.2, FR-2.5, NG-3)
- description: Agent selection strategies for Grove.
- acceptance:
  - `RoutingStrategy::KeywordMatch` MUST be implemented as the default, counting matching keywords between task and agent expertise
  - `RoutingStrategy::SemanticSimilarity` MUST be implemented using cosine similarity between task and agent embeddings, with a configurable similarity threshold
  - `RoutingStrategy::LlmRouting` MUST be implemented, sending task plus agent descriptions to the LLM and expecting a JSON selection
  - The agent with the highest score/confidence MUST be selected; `fallback_tree` MUST be used when no agent meets the threshold
  - Grove learning from past routing decisions is an explicit non-goal (NG-3)
  - Grove routing MUST complete in < 3s including LlmRouting; routing accuracy target >= 85% in test cases
- scope: RoutingStrategy, routing scoring, fallback

## REQ-grove-config-v1
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-2.3)
- description: GroveConfig as defined by Epic 16. COMPETING VARIANT — see REQ-grove-config-v2.
- acceptance:
  - `GroveConfig` MUST carry `routing_strategy`, optional `fallback_tree` name and `similarity_threshold`
  - Configuration MUST be validated on Grove creation
  - Defaults MUST be `KeywordMatch` strategy and threshold `0.7`
- scope: GroveConfig, similarity_threshold, Epic 16 position

## REQ-grove-config-v2
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (FR-6.1 to FR-6.3, US-22.2)
- description: GroveConfig extensions defined by Epic 22. COMPETING VARIANT — see REQ-grove-config-v1.
- acceptance:
  - `GroveConfig` MUST gain `routing_fallback: String` with values "keyword" or "error"
  - `GroveConfig` MUST gain `min_confidence: f32`, default `0.5`, valid range 0.0-1.0
  - Validation MUST reject invalid fallback values and out-of-range confidence
  - Fallback behaviour MUST be configurable rather than always falling back to keyword matching
- scope: GroveConfig, routing_fallback, min_confidence, Epic 22 position

## REQ-grove-execution-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (US-16.4, FR-2.4, FR-4.2)
- description: Service performing Grove routing and execution.
- acceptance:
  - `GroveExecutionService` MUST live in `src/application/use_cases/battalion/grove_service.rs`
  - MUST provide `execute(grove, task)` and an internal `route_task(grove, task)`
  - MUST return `GroveResult` with the selected agent, routing decision and execution result
  - `RoutingDecision` MUST carry selected_tree, selected_agent, confidence score and reasoning
  - MUST handle empty trees, no agents, invalid routing strategy, missing embeddings for SemanticSimilarity, and LLM routing failure with fallback to KeywordMatch
  - MUST return a routing decision even on failure, including reasoning
  - Routing calculation for all agents MAY run in parallel; multiple Grove executions MUST be able to run concurrently
- scope: GroveExecutionService, GroveResult, RoutingDecision, error handling

## REQ-grove-arsenal-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (FR-2.6)
- description: Optional tool-availability awareness in Grove routing.
- acceptance:
  - TreeAgents SHOULD be able to declare required Arsenal tools
  - Routing SHOULD validate the agent has access to required tools before routing
  - Tool availability SHOULD be included in the routing decision
- scope: Grove + Arsenal, routing constraints

## REQ-council-grove-commander-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md (US-16.5, FR-3.1 to FR-3.3)
- description: Commander support for Council and Grove.
- acceptance:
  - `BattalionStrategy::Council` and `BattalionStrategy::Grove` variants MUST be added
  - `BattalionStrategy::Auto` logic MUST be updated to consider Council and Grove
  - Commander MUST route Council requests to `CouncilExecutionService` and Grove requests to `GroveExecutionService`
  - Explicit strategy selection via config MUST be supported
  - Auto-detection MUST map "discuss", "debate", "collaborate" to Council and "expert", "specialist", "route" to Grove
  - CLI MUST support `--strategy council` and `--strategy grove` with example configs in `examples/cli_configs/council_*.yml` and `grove_*.yml`
  - No breaking changes or regressions to existing Battalion patterns
- scope: BattalionStrategy::Council, BattalionStrategy::Grove, Commander routing, CLI

---

## REQ-flow-dsl-syntax
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (US-17.1, FR-1)
- description: String grammar for expressing agent workflows.
- acceptance:
  - Parser MUST support `a -> b` (sequential), `a, b` (parallel), `a -> b, c` (fan-out), `a, b -> c` (fan-in), `(a -> b)` (grouping) and `a -> b -> c` (chain)
  - Nested grouping such as `"planner -> (coder -> tester), docs"` MUST be supported
- scope: Flow DSL grammar, operators

## REQ-flow-parser
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-2.1 to FR-2.4, section 7.1)
- description: Parser producing a validated flow AST.
- acceptance:
  - Parser MUST live in `src/core/platform/container/battalion/parser/` (mod.rs, lexer.rs, ast.rs, error.rs)
  - Parser MUST return `Result<FlowExpression, FlowParseError>`
  - Parser MUST validate balanced parentheses, valid agent identifiers (alphanumeric, underscore, hyphen), no empty groups and no consecutive operators
  - Errors MUST show the position in the expression and suggest corrections; 100% of parse errors MUST include a helpful suggestion
  - Parsing MUST complete in < 1ms for 99% of flows (30+ agents) with zero panics on malformed input
- scope: FlowParser, FlowParseError, lexer, validation

## REQ-flow-expression-ast
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-3.1 to FR-3.3)
- description: AST representation of a parsed flow.
- acceptance:
  - `FlowExpression` MUST provide `Agent(String)`, `Sequential(Vec<FlowExpression>)` and `Parallel(Vec<FlowExpression>)`
  - The AST MUST be serialisable/deserialisable for storage and debugging
  - The AST MUST be extensible without breaking changes
- scope: FlowExpression, AST

## REQ-maneuver-domain-model
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (US-17.2, FR-4.1 to FR-4.4)
- description: Domain model for the Maneuver (AgentRearrange) pattern.
- acceptance:
  - `Maneuver` MUST live in `src/core/platform/container/battalion/maneuver.rs`
  - `Maneuver` MUST carry `name: String`, `agents: HashMap<String, Paladin>`, `flow: FlowExpression` and `config: ManeuverConfig`
  - Construction MUST validate that every agent name referenced in the flow exists in the agents map
  - MUST support 10-30 agents with nesting depth up to 5 levels
- scope: Maneuver, agent registry validation

## REQ-maneuver-config
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-5.1 to FR-5.4, section 7.4)
- description: Execution configuration for Maneuver.
- acceptance:
  - MUST support `timeout_seconds: u64` and `agent_timeout_seconds: Option<u64>`
  - MUST support `pass_output_as_input: bool` and `output_format: OutputFormat` (concatenate | json_array) for fan-in
  - MUST support `collect_timing_metrics: bool` and `capture_intermediate_outputs: bool`
  - Global defaults MUST be configurable in `config.yml` under `maneuver` (default_error_strategy, default_timeout_seconds 300, max_nesting_depth 5, max_parallel_branches 10)
- scope: ManeuverConfig, output aggregation, tracing flags

## REQ-maneuver-error-strategy-v2
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-5.1)
- description: ErrorStrategy variant set for Maneuver. COMPETING VARIANT — see REQ-battalion-error-strategy (run 1, Epic 4), which defines a differently named variant set for the same type.
- acceptance:
  - `ErrorStrategy::FailFast` MUST stop the entire workflow on the first error
  - `ErrorStrategy::ContinueParallel` MUST continue parallel branches but fail the sequence
  - `ErrorStrategy::IgnoreErrors` MUST log errors but continue execution
- scope: ErrorStrategy, Maneuver error handling

## REQ-maneuver-execution-service
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (US-17.3, FR-6.1 to FR-6.6)
- description: Recursive execution of flow expressions.
- acceptance:
  - Service MUST execute flow expressions recursively, evaluating nested sub-expressions in the correct order
  - Sequential execution MUST feed `a`'s output into `b` when `pass_output_as_input` is set, otherwise reuse the original input
  - Parallel execution MUST run agents concurrently with the same input, wait for all within the timeout, and aggregate per configuration
  - Error handling MUST respect the configured `ErrorStrategy` and report which agent failed at which step
  - MUST return `ManeuverResult` with `final_output: String`, `step_outputs: HashMap<String, PaladinResult>`, `execution_order: Vec<String>` and `timing_metrics: Option<HashMap<String, Duration>>`
  - Orchestration overhead MUST be < 10ms (target < 2% of total execution time); memory footprint O(n) in agent count
- scope: ManeuverExecutionService, ManeuverResult, recursion, aggregation

## REQ-maneuver-commander-integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (US-17.4, FR-7.1 to FR-7.4)
- description: Commander support for flow-based workflows.
- acceptance:
  - `BattalionStrategy::Maneuver` variant MUST be added
  - `CommanderBuilder` MUST accept `flow(expression: &str)` and `error_strategy(ErrorStrategy)`
  - Auto strategy MUST NOT select Maneuver — it is explicit only
  - Commander MUST validate that every agent referenced in the flow is registered
- scope: BattalionStrategy::Maneuver, CommanderBuilder, explicit-only selection

## REQ-maneuver-cli
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-8.1 to FR-8.3, FR-9.3)
- description: CLI surface for Maneuver as defined by Epic 17. COMPETING with the `paladin maneuver ...` surface recorded in the Milestone 3 release notes — see INGEST-CONFLICTS.md.
- acceptance:
  - CLI MUST support `paladin battalion run --type maneuver --flow "<expr>" --config maneuver.yaml`
  - YAML MUST support `type: maneuver`, `flow`, `config.{error_strategy, timeout_seconds, pass_output_as_input, collect_timing_metrics}` and an `agents` list
  - CLI MUST support template generation via `paladin battalion new --type maneuver > maneuver.yaml`
  - CLI MUST support `paladin battalion visualize --flow "<expr>"` with an optional `--format mermaid`
- scope: Maneuver CLI, Maneuver YAML schema

## REQ-flow-visualization
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (US-17.5, FR-9.1 to FR-9.4)
- description: Human-readable rendering of flow expressions.
- acceptance:
  - `FlowVisualizer::to_ascii(&FlowExpression) -> String` MUST render parallel branches and sequential chains clearly
  - `FlowVisualizer::to_mermaid(&FlowExpression) -> String` MUST emit valid Mermaid.js syntax
  - Visualisation MUST overlay per-agent execution time and total workflow time when metrics are available, highlighting the slowest agent
  - An `--output` flag SHOULD allow saving diagrams for documentation
- scope: FlowVisualizer, ASCII and Mermaid rendering, timing overlay

## REQ-maneuver-validation
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md (FR-10.1 to FR-10.3)
- description: Construction-time validation and graceful degradation for Maneuver.
- acceptance:
  - Validation at construction MUST confirm all agent names exist, reject self-references and circular dependencies, and enforce configurable max depth/width
  - Errors MUST clearly report parse position, missing agent references, which agent timed out, and agent execution failures
  - Partial results MUST be surfaced where possible, with a clear indication of which steps failed and recovery suggestions
- scope: Maneuver validation, error messages, partial results

---

## REQ-cli-onboarding-wizard
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.1, FR-6 to FR-10)
- description: Interactive first-run setup wizard.
- acceptance:
  - `paladin onboarding` MUST run an interactive wizard guiding API key configuration for at least one LLM provider
  - Wizard MUST create a `.env` file containing OPENAI_API_KEY, ANTHROPIC_API_KEY, DEEPSEEK_API_KEY and optional REDIS_URL, QDRANT_URL, MINIO_ENDPOINT entries
  - Wizard MUST validate API keys by making test API calls to each provider
  - When existing configuration is detected the wizard MUST show the conflicting path and offer Overwrite / Skip / Merge, merging without duplicates
  - Wizard MUST offer to generate sample configs: `examples/basic_paladin.yaml`, `formation.yaml`, `phalanx.yaml`, `paladin_with_rag.yaml`
  - Wizard MUST be resumable after interruption (completed steps tracked) and print a summary of completed setup steps
  - Target: 90% of new users run their first agent within 5 minutes
- scope: paladin onboarding, .env generation, sample configs

## REQ-cli-setup-check
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.2, FR-11 to FR-14)
- description: Environment validation command.
- acceptance:
  - `paladin setup-check` MUST validate Paladin CLI version, Rust toolchain version, each configured LLM provider, and Redis / Qdrant / MinIO connectivity when configured
  - Provider validation MUST make real API calls: OpenAI `/v1/models`, Anthropic minimal message request, DeepSeek models endpoint
  - Status indicators MUST be green check, red cross and yellow warning, with actionable error messages for failures
  - `--verbose` MUST show full version strings, API response times, detailed errors and the configuration file locations in use
  - Exit codes MUST be 0 all pass, 1 critical failure, 2 warnings
  - Target: 95% of runs identify actual configuration issues
- scope: paladin setup-check, environment validation, exit codes

## REQ-cli-features-discovery
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.3, FR-15 to FR-17)
- description: Capability discovery command.
- acceptance:
  - `paladin features` MUST list commands grouped as Agent, Battalion, Orchestration Patterns, Memory Systems and Utility
  - Orchestration patterns listed MUST include Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove and Maneuver
  - Memory systems listed MUST include Garrison (in-memory, SQLite) and Sanctum (Qdrant, in-memory)
  - Each entry MUST show command name, 1-2 sentence description, availability status (available vs requires feature flag) and a documentation link
  - MUST support `--category <name>` filtering and `--format json` machine-readable output with the documented `categories[].commands[]` shape
- scope: paladin features, capability catalogue, JSON output

## REQ-cli-muster-command
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.4, FR-18 to FR-22)
- description: LLM-powered battalion generation from a task description.
- acceptance:
  - `paladin muster --task "description"` MUST accept the task via flag, interactive prompt or stdin
  - The LLM analysis MUST return a recommended orchestration pattern with justification, suggested agents (name, role, system prompt), estimated complexity (simple|medium|complex) and estimated token usage
  - Generated configuration MUST be valid YAML immediately executable with `paladin battalion run`, saved by default as `muster_<timestamp>.yaml`
  - MUST support `--execute`, `--output <path>`, `--provider <name>`, `--model <name>` and `--no-review`
  - The user MUST be able to review and edit before execution; the chosen pattern MUST be explained
  - On LLM failure the command MUST fall back to template selection by keyword matching
- scope: paladin muster, battalion generation, YAML output

## REQ-cli-council-command
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.5, FR-23 to FR-26)
- description: Shortcut command for Council discussions.
- acceptance:
  - `paladin council` MUST support `--topic "description"`, `--participants N` (min 2, max 10, default 3), `--roles "r1,r2,..."`, `--max-rounds N` (default 5) and `--save <path>`
  - Default role assignment MUST be 2 -> Advocate, Critic; 3 -> Advocate, Critic, Moderator; 4 -> plus Synthesizer; 5+ -> mix of Experts, Advocates, Critics, Moderator
  - Real-time output MUST show round number, speaker role and name, contribution text and clear visual separation between turns
  - Final summary MUST include key points, areas of consensus, areas of disagreement and a recommended action
  - Full transcript MUST be savable to file
- scope: paladin council, role assignment, transcript

## REQ-cli-rich-output
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (US-18.6, FR-27 to FR-30)
- description: Consistent, accessible terminal presentation.
- acceptance:
  - Progress indicators MUST be used for API calls (spinner with status), file operations, battalion execution (completion percentage) and embedding generation batches
  - Colour scheme MUST be green success, red error, yellow warning, blue informational, cyan links, default standard output, and MUST respect `NO_COLOR`
  - Tables MUST be used for battalion execution summaries (agent, time, tokens, status), setup-check results and feature listings
  - Box drawing MUST be used for section headers, important notices, final summaries and error messages with context
  - Token usage and timing MUST be displayed clearly; agent responses MUST stream in real time
  - `--quiet` and `--verbose` modes MUST be supported
  - Libraries: clap 4.5, indicatif 0.17, console 0.15, colored 2.1, comfy-table 7.1, dialoguer 0.11
- scope: CLI formatters, progress, colour, tables, accessibility

## REQ-cli-core-infrastructure
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md (FR-1 to FR-5, section 7)
- description: Cross-cutting CLI infrastructure requirements.
- acceptance:
  - Every command MUST support both interactive and non-interactive modes
  - Every command MUST support `--help` with comprehensive usage information
  - CLI MUST respect standard environment variables (`NO_COLOR`, `TERM`)
  - Exit codes MUST be 0 success, 1 error, 2 warning
  - Configuration MUST load from `.env`, `config.yml` and CLI flags
  - Command implementations MUST live in `src/application/cli/commands/`, formatters in `src/application/cli/formatters/`, interactive helpers in `src/application/cli/interactive/`, entry point `src/bin/paladin-cli.rs`
  - Test coverage: unit >= 80% line coverage, integration covering all happy paths plus major error cases, snapshot tests for all user-facing output formats
- scope: CLI infrastructure, module layout, exit codes, configuration loading

---

## REQ-herald-type-consolidation
- source: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md (US-19.1, FR-1.1 to FR-1.5)
- description: Herald must consume the real domain types instead of placeholders. LATER POSITION on the run-1 `REQ-herald-*` / `REQ-battalion-result-*` duplication question — run-1 entries preserved.
- acceptance:
  - Placeholder `PaladinResult` (herald.rs line 147), `BattalionResult` (line 158) and `PaladinError` (line 187) MUST be removed
  - Herald MUST import `PaladinResult` and `PaladinError` from `src/core/platform/container/paladin.rs` and `BattalionResult` from `src/core/platform/container/battalion/mod.rs`
  - `HeraldPort` in `src/application/ports/output/herald_port.rs` MUST be updated to the consolidated types
  - `JsonHerald`, `MarkdownHerald` and `TableHerald` MUST be updated to the consolidated types
  - No duplicate type definitions may remain, verified by grep/search
  - All existing Herald tests MUST continue to pass; trait bounds and generic constraints updated as needed; `Send + Sync` preserved
  - This is internal refactoring: breaking changes are acceptable and no compatibility shim is required
- scope: herald.rs, HeraldPort, Herald adapters, single source of truth for domain types

## REQ-stream-chunk-complete
- source: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md (US-19.1, FR-2.1 to FR-2.5)
- description: Complete the StreamChunk streaming metadata structure.
- acceptance:
  - `StreamChunk` MUST carry `chunk_id: Uuid`, `sequence_number: u64` (0-indexed), `timestamp: DateTime<Utc>`, `content: String`, `token_count: Option<u32>`, `is_final: bool`
  - `StreamChunk` MUST carry an extensible `metadata: HashMap<String, serde_json::Value>` using `#[serde(flatten)]`
  - MUST derive `Debug`, `Clone`, `Serialize`, `Deserialize`
  - MUST provide a builder (`StreamChunk::builder()`) and validation for required fields
  - MUST serialise for JSON formatting output, Citadel state persistence and event streaming
- scope: StreamChunk, streaming telemetry, serialization

## REQ-execution-metadata-complete
- source: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md (US-19.1, FR-3.1 to FR-3.6)
- description: Complete the ExecutionMetadata telemetry structure.
- acceptance:
  - `ExecutionMetadata` MUST carry `execution_id: Uuid`, `start_time: DateTime<Utc>`, `end_time: Option<DateTime<Utc>>`, `duration_ms: Option<u64>`, `model_used: String`, `token_usage: TokenUsage`, `cost_estimate: Option<f64>`, `error_count: u32`
  - `TokenUsage` MUST carry `input_tokens`, `output_tokens`, `total_tokens`
  - MUST carry an extensible `metadata: HashMap<String, serde_json::Value>` with `#[serde(flatten)]` for provider-specific data
  - MUST derive `Debug`, `Clone`, `Serialize`, `Deserialize` and provide a builder
  - MUST provide `calculate_duration()` deriving `duration_ms` from start/end times, plus helper methods for token usage analysis
- scope: ExecutionMetadata, TokenUsage, telemetry, cost estimation

## REQ-herald-formatter-autoregistration
- source: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md (US-19.2, FR-4.1 to FR-4.6)
- description: Zero-config Herald usage via default formatter registration.
- acceptance:
  - `HeraldRegistry` MUST implement `Default`, auto-registering `JsonHerald` as "json", `MarkdownHerald` as "markdown" and `TableHerald` as "table"
  - Formatters MUST be retrievable via `registry.get("json")` etc.
  - The existing manual registration API MUST be preserved so custom formatters can still be added
  - Duplicate keys MUST be handled deliberately (error or overwrite) and the behaviour documented
  - Formatter keys MUST be documented in rustdoc, with a zero-config usage example of three lines
- scope: HeraldRegistry::default, built-in formatters, zero-config usage

## REQ-herald-consolidation-quality-gates
- source: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md (FR-5.1 to FR-5.8, Success Metrics)
- description: TDD and quality requirements specific to the Herald consolidation.
- acceptance:
  - Failing tests MUST be written before each change (TDD)
  - Unit tests MUST cover type consolidation, StreamChunk fields, ExecutionMetadata fields and default-registry auto-registration
  - Integration tests MUST cover the full Herald pipeline with consolidated types
  - Serialization round-trip tests MUST prove no data loss for extensible metadata fields; builder patterns and validation MUST be tested
  - Test coverage MUST be >= 95% for modified Herald modules; 100% of public APIs documented
  - `cargo build` with no warnings, `cargo clippy` zero warnings, all `examples/herald_*.rs` run successfully
- scope: Herald test coverage, TDD, quality gates

---

## REQ-autonomous-configurable-model
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (US-21.5, FR-1.1 to FR-1.6)
- description: Remove hardcoded model identifiers from autonomous services.
- acceptance:
  - `PlanningService` MUST read `model` from the Paladin config instead of a hardcoded `"gpt-4"` (planning_service.rs lines 128, 305, 426, 538)
  - `PromptGenerationService` MUST read `model` from the Paladin config instead of a hardcoded `"gpt-4"` (prompt_generation_service.rs line 146)
  - Services MUST pass the configured model to `LlmPort` methods
  - Services MUST validate model compatibility with required features (e.g. vision) and log a warning then fall back to a safe default on invalid configuration
  - Subtask expected output MUST be generated by the LLM instead of a hardcoded placeholder string
- scope: PlanningService, PromptGenerationService, model configuration

## REQ-paladin-result-autonomous-metadata
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (US-21.3, FR-2.1 to FR-2.7)
- description: PaladinResult gains planning and handoff metadata with zero breaking changes.
- acceptance:
  - `PaladinResult` MUST include `plan: Option<TaskPlan>` defaulting to `None`
  - `PaladinResult` MUST include `handoff_history: Vec<HandoffRecord>` defaulting to `Vec::new()`
  - `TaskPlan` MUST carry `goal: String`, `subtasks: Vec<Subtask>`, `created_at: DateTime<Utc>`
  - `HandoffRecord` MUST carry `specialist_name`, `task_description`, `timestamp`, `result: Option<String>`, `depth: usize`
  - Serialization MUST support JSON and MessagePack; deserialization MUST tolerate missing fields for backward compatibility
  - All existing tests MUST pass unmodified; new tests MUST verify metadata capture when autonomous features are enabled
  - Handoff history SHOULD be capped (recommended default 100 records, oldest-first eviction)
- scope: PaladinResult, TaskPlan, HandoffRecord, backward compatibility

## REQ-autonomous-orchestration-layers
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (US-21.4, FR-4.1 to FR-4.8, section 6.5)
- description: Layered orchestration of autonomous features inside the execution service.
- acceptance:
  - `PaladinExecutionService` MUST implement a layered flow: Layer 0 core LLM execution (always), Layer 1 planning then prompt generation (optional), Layer 2 dynamic temperature (optional), Layer 3 handoff handling (optional)
  - Each layer MUST be independently enabled/disabled by configuration: `autonomous_planning`, `autonomous_prompts`, `dynamic_temperature`, `handoffs`
  - Layer failures MUST NOT prevent core execution (graceful degradation); core execution MUST never fail because an optional feature is disabled
  - Feature-interaction edge cases MUST be handled: subtasks triggering handoffs, temperature adjusted per loop, all features together
  - Orchestration MUST populate `PaladinResult` metadata when features are active
  - Integration tests MUST cover each layer independently and in combination; at least one end-to-end test with all features enabled; performance impact measured
- scope: PaladinExecutionService orchestration, layered features, graceful degradation

## REQ-handoff-execution-integration
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (US-21.1, FR-5.1 to FR-5.11)
- description: End-to-end handoff execution with retry and cycle safety.
- acceptance:
  - `HandoffService::execute_handoff()` MUST delegate to `PaladinExecutionService`, passing the specialist Paladin instance
  - The specialist result MUST flow back to the original agent as a tool response so it can continue with context
  - The handoff chain MUST be tracked in `HandoffRecord` with a depth counter and maintained across all tool invocations
  - Circular handoff MUST be detected (same specialist at the same depth) and max depth enforced (configurable, default 5)
  - Handoff calls MUST be visible in the execution trace and logs
  - Handoff errors MUST support configurable retry with exponential backoff: `max_handoff_retries`, `initial_backoff_ms` (default 1000), `backoff_multiplier` (default 2.0), `max_retries` default 3
  - Transient errors (network, timeout, rate limit) MUST retry; permanent errors (invalid specialist, circular reference, config error) MUST fail immediately
  - Circuit breaker MUST integrate with handoff retry logic
  - Recommended defaults: sequential (non-concurrent) handoffs; fail with a clear error if a specialist becomes unavailable mid-execution
  - Error codes: E-HANDOFF-001 CircularHandoff, 002 MaxDepthExceeded, 003 SpecialistNotFound, 004 ExecutionFailed (retryable), 005 InvalidResponse; E-PLAN-001, E-PROMPT-001, E-CONFIG-001, E-CONFIG-002
- scope: HandoffService::execute_handoff, retry policy, cycle detection, error codes

## REQ-autonomous-completion-config-schema
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (FR-6.1, Summary Configuration Schema)
- description: Consolidated YAML schema for autonomous execution. LATER POSITION relative to REQ-autonomous-configuration (Epic 14) — both preserved.
- acceptance:
  - Paladin configuration MUST support top-level `model`, `autonomous_planning`, `autonomous_prompts`, `dynamic_temperature`
  - `handoffs` MUST support `enabled`, `max_depth` (default 5), `concurrent` (default false), `retry.{max_retries, initial_backoff_ms, backoff_multiplier}`, `history.{max_records: 100, eviction: oldest_first}`, `on_specialist_unavailable: fail`, and a `specialists` list of `{name, description, model?}`
  - `planning.validate_at: planning_time` MUST be supported (fail fast on invalid plans)
  - Configuration errors MUST fail fast at build time before execution
  - New settings MUST be opt-in with sensible defaults; no migration scripts required
- scope: autonomous YAML schema, handoff configuration, validation timing

## REQ-autonomous-completion-quality-gates
- source: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md (section 6.4, SM-1 to SM-5)
- description: Test and quality requirements for the autonomous completion epic.
- acceptance:
  - Unit tests MUST reach >= 90% line coverage for autonomous components
  - Integration tests MUST cover every user story; at least 3 end-to-end workflow scenarios
  - All 23 deferred Epic 14 tasks MUST be completed with zero remaining TODO comments in autonomous agent code
  - `cargo clippy -- -D warnings`, `cargo fmt --check` and `cargo build --release` MUST pass
  - No breaking changes to public APIs; existing examples and configuration MUST keep working
  - Core execution performance MUST be unchanged when features are disabled; handoff execution within 2x single-agent time; memory stable across handoff chains
- scope: autonomous test coverage, quality gates, backward compatibility

---

## REQ-paladin-registry-port
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.1, FR-1.1 to FR-1.4, section 6.2)
- description: Trait-based registry resolving Paladin IDs to instances.
- acceptance:
  - `PaladinRegistry` trait MUST be defined in `src/application/ports/output/paladin_registry.rs` and be `Send + Sync`
  - Trait methods MUST be `register(id: String, paladin: Arc<Paladin>) -> Result<(), RegistryError>`, `get(id: &str) -> Option<Arc<Paladin>>`, `contains(id: &str) -> bool`, `list_ids() -> Vec<String>`
  - Return types MUST use `Arc<Paladin>` for shared ownership
  - The registry MUST be passed to services via constructor injection as `Arc<dyn PaladinRegistry>`, never a global singleton
  - `RegistryError` MUST include DuplicateId and InvalidId
  - Multi-tenancy, persistence and distribution are explicit non-goals
- scope: PaladinRegistry port, dependency injection, hexagonal boundary

## REQ-paladin-registry-adapter
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (FR-2.1 to FR-2.3)
- description: Default HashMap-backed registry implementation.
- acceptance:
  - `HashMapPaladinRegistry` MUST be implemented in `src/infrastructure/adapters/paladin_registry.rs` using `HashMap<String, Arc<Paladin>>`
  - Access MUST be thread-safe via `RwLock` or `Mutex`
  - `new()` MUST create an empty registry
  - Lookup MUST be O(1) with < 1ms overhead per operation
- scope: HashMapPaladinRegistry, thread safety

## REQ-council-grove-registry-resolution
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.1, FR-3.1 to FR-4.3)
- description: Council and Grove resolve stored IDs to real Paladin instances before execution.
- acceptance:
  - `CouncilService` MUST accept `Arc<dyn PaladinRegistry>` in its constructor and resolve all participant IDs before discussion rounds begin
  - `GroveService` MUST accept `Arc<dyn PaladinRegistry>` and resolve the routed agent ID to a Paladin instance after the routing decision
  - Unresolvable IDs MUST return `BattalionError::PaladinNotFound(id)`
  - Commander MUST populate the registry when creating Council/Grove battalions from configuration, after configuration validation
  - Unit tests MUST cover registration/lookup, Council resolving 3 participants, Grove resolving the selected agent and the missing-ID error path
  - Integration tests MUST verify the full execution flow with resolved Paladins
  - The registry is required only for Council and Grove; other patterns are unaffected and existing configurations keep working
- scope: CouncilService, GroveService, Commander registry population, BattalionError::PaladinNotFound

## REQ-grove-llm-routing
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.2, FR-5.1 to FR-5.7)
- description: Replace the stubbed Grove LLM routing with a real implementation.
- acceptance:
  - `GroveService::route_with_llm()` MUST send a routing prompt to the configured LLM provider via `LlmPort::generate()`
  - The prompt MUST include the user input and the list of available agents with descriptions and specialisations, and MUST instruct the model to return JSON `{"tree_name","agent_id","confidence","reasoning"}`
  - The response MUST be parsed with `serde_json` and validated: confidence in [0.0, 1.0] and `agent_id` present in the Grove configuration
  - Confidence below `min_confidence` MUST be treated as a routing failure
  - Fallback MUST follow `GroveConfig::routing_fallback`: "keyword" falls back to keyword matching, "error" returns an error
  - Errors MUST be handled for LLM call failure (network, timeout, rate limit), invalid JSON, missing fields and unknown agent_id
  - Routing decisions with reasoning MUST be logged for observability
  - Unit tests MUST cover successful routing, low-confidence fallback, invalid JSON, LLM failure and keyword fallback; an integration test MUST cover the full flow with a mocked HTTP LLM adapter
- scope: GroveService::route_with_llm, JSON contract, fallback strategy

## REQ-phalanx-per-paladin-metrics
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.3, FR-7.1 to FR-7.6)
- description: Per-Paladin timing and token metrics for parallel execution.
- acceptance:
  - `PhalanxService::execute()` MUST record start and end time for each Paladin execution
  - `BattalionMetadata::per_paladin_times` MUST be populated as `HashMap<String, Duration>` keyed by Paladin ID
  - `BattalionMetadata::per_paladin_tokens` MUST be populated as `HashMap<String, TokenUsage>` (prompt_tokens, completion_tokens, total_tokens) extracted from `PaladinResult::metadata`
  - Aggregates MUST be computed: `paladin_success_count`, `paladin_failure_count`, `total_tokens`
  - Metrics MUST survive partial failures and timeouts
  - Unit tests MUST verify timing accuracy within 10ms tolerance, correct token aggregation, accurate success/failure counts and metric capture when some Paladins fail
  - Metrics collection overhead MUST stay under 1% of execution time
  - Memory, GPU and network metrics are explicit non-goals
- scope: PhalanxService metrics, BattalionMetadata, token aggregation

## REQ-battalion-metadata-extension
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (FR-8.1 to FR-8.4)
- description: BattalionMetadata gains per-Paladin and aggregate token fields. LATER POSITION on the run-1 `REQ-battalion-result-v1/-v2` question — run-1 entries preserved. Note the run-1 Epic 5 shape typed `per_paladin_times` as `Vec<u64>`.
- acceptance:
  - `BattalionMetadata` MUST gain `per_paladin_times` and `per_paladin_tokens`
  - `BattalionMetadata` MUST gain `total_tokens` as an aggregate token count
  - All new fields MUST derive `Serialize`/`Deserialize`
  - The type is located in `src/core/platform/container/battalion/battalion_result.rs` per this PRD
- scope: BattalionMetadata, BattalionResult shape, serialization

## REQ-commander-metadata-export
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.4, FR-9.1 to FR-9.6, section 6.4)
- description: Opt-in JSON export of Commander execution metadata.
- acceptance:
  - When `metadata_output_dir` is configured, Commander MUST write a metadata JSON file after each execution
  - File naming MUST follow `<metadata_output_dir>/<strategy>_<timestamp>_<uuid>.json` where timestamp is `YYYYMMDD_HHMMSS` local time and uuid is an 8-character short UUID
  - The JSON MUST include strategy, timestamp, duration_ms, paladin_count, success_count, failure_count, total_tokens, per_paladin_times, per_paladin_tokens and a sanitised config_snapshot containing no secrets
  - The output directory MUST be created if missing; write and permission failures MUST be logged but remain non-fatal
  - Export overhead MUST stay under 50ms per execution
  - JSON is the only supported format; CSV/YAML/Markdown, streaming, compression and encryption are explicit non-goals
  - Unit tests MUST verify naming, JSON content, directory creation and write-failure handling in a temp directory; an integration test MUST cover a full Commander execution with export enabled
- scope: Commander metadata export, JSON schema, non-fatal I/O

## REQ-commander-config-metadata-dir-v3
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (FR-10.1 to FR-10.3)
- description: `metadata_output_dir` located on CommanderConfig. COMPETING VARIANT — run 1 placed the same field on `BattalionConfig` in both `REQ-battalion-config-v1` (Epic 4) and `REQ-battalion-config-v2` (Epic 5).
- acceptance:
  - `metadata_output_dir: Option<PathBuf>` MUST be added to `CommanderConfig` in `src/core/platform/container/battalion/commander_config.rs`
  - `None` MUST disable metadata export (default behaviour)
  - When `Some`, the path MUST be validated as writable before the first execution
  - YAML surface: `commander.metadata_output_dir: "./metadata"`
- scope: CommanderConfig, metadata_output_dir ownership, Epic 22 position

## REQ-commander-test-hardening
- source: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md (US-22.5, FR-11.1 to FR-11.5)
- description: Enable the six ignored Commander tests with real Paladins over a mock LLM.
- acceptance:
  - `test_execute_campaign` MUST be enabled and passing with a mocked DAG of 4+ nodes, verifying dependency-respecting execution order and correct result collection
  - `test_execute_chain_of_command` MUST be enabled and passing with a mocked supervisor plus 2 workers, verifying delegation flow and result aggregation
  - `test_error_handling_fail_fast`, `test_error_handling_continue_on_error`, `test_error_handling_retry_then_continue` and `test_partial_failure_handling` MUST be enabled and passing
  - Tests MUST use real `Paladin` instances with a mock LLM adapter; only the LLM layer is mocked
  - `MockLlmAdapter` MUST support configurable responses per call, failure simulation and call-count tracking, with helper functions for building test Paladins
  - All `#[ignore]` attributes MUST be removed from these tests; zero regressions in existing Battalion tests; >= 80% unit coverage for new code
- scope: Commander tests, MockLlmAdapter, error-strategy coverage

---

## REQ-cli-garrison-configuration
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (US-23.1, FR-23.1.1 to FR-23.1.5)
- description: YAML-driven garrison configuration for CLI-launched agents.
- acceptance:
  - YAML MUST support `garrison.{type: in_memory|sqlite, path, max_entries, ttl_seconds}`
  - `in_memory` MUST instantiate `InMemoryGarrison`; `sqlite` MUST instantiate `SqliteGarrison`
  - Validation MUST check type presence/validity, path presence and writability for sqlite, and positive max_entries and ttl_seconds
  - Error messages MUST be actionable and name the exact field, e.g. "garrison.type is required (valid values: in_memory, sqlite)"
  - The configured garrison MUST be passed to `PaladinBuilder` during agent construction (resolving the TODO at `src/application/cli/commands/agent.rs` line 293)
  - Recommended: auto-create parent directories for SQLite paths; garrison initialisation failures are fatal (fail fast)
  - Unit tests from sample YAML plus an integration test proving garrison persistence across agent executions
- scope: CLI garrison configuration, PaladinBuilder wiring, validation errors

## REQ-cli-arsenal-configuration
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (US-23.2, FR-23.2.1 to FR-23.2.6)
- description: YAML-driven arsenal / MCP configuration for CLI-launched agents.
- acceptance:
  - YAML MUST support `arsenal.mcp_servers[]` entries with `name`, `type: stdio|sse`, `command` and `args` for stdio, `url` and optional `auth_token` for sse, with `${VAR}` environment substitution
  - `stdio` MUST instantiate `MCPStdioAdapter`; `sse` MUST instantiate `MCPSseAdapter`
  - Discovered tools MUST be registered in the arsenal registry with name, description, capability mapping (tool parameters to MCP schema) and a server reference
  - Validation MUST check required fields per transport, valid HTTP/HTTPS URLs and resolvable environment variables
  - Error messages MUST name the offending server and field, including connection failures
  - The configured arsenal registry MUST be passed to `PaladinBuilder` (resolving the TODO at `agent.rs` line 296)
  - Recommended MCP connection timeout: 10 seconds default, configurable via `arsenal.connection_timeout_seconds`
  - Unit tests for both transports plus an integration test proving tool discovery and invocation
- scope: CLI arsenal configuration, MCP adapters, tool registration

## REQ-mock-llm-adapter
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (US-23.3, FR-23.3.1 to FR-23.3.6)
- description: Configurable mock LLM provider enabling dependency-free CLI tests.
- acceptance:
  - `MockLlmAdapter` MUST implement the `LlmPort` trait (`generate`, `generate_stream`, `validate_model`)
  - It MUST support response modes: simple text, tool calls, chunked streaming and error simulation (API failure, rate limit, timeout)
  - It MUST be configurable in test setup, e.g. `.with_response(..)`, `.with_tool_call(name, args)`, `.with_streaming(vec![..])`, `.with_error(..)`
  - It MUST record invocations for assertions: call count, prompts received, models requested, tool calls made
  - Integration tests MUST validate single-Paladin execution from YAML, Formation sequential execution, Phalanx parallel execution, error handling/recovery and tool integration through the arsenal
  - Mock-based tests MUST run in CI with no API keys and no external dependencies
  - Recommended placement: test utilities rather than production code
- scope: MockLlmAdapter, CLI integration tests, CI independence

## REQ-cli-tiered-environment-testing
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (US-23.4, FR-23.4.1 to FR-23.4.4)
- description: Three-tier test strategy gated by external dependency.
- acceptance:
  - Tier 1 (always in CI) MUST cover happy paths for every command, error handling for common failures and edge cases (empty input, very large input, malformed YAML, concurrent operations)
  - Tier 2 (Docker-gated) MUST cover `setup-check` against real Redis, Qdrant and MinIO, plus service health validation and connection error handling
  - Tier 3 (API-key-gated) MUST cover real OpenAI / DeepSeek / Anthropic execution, the `council` command end to end and streaming response handling
  - Non-interactive mode MUST be supported: all required arguments available as flags, a `--non-interactive` flag disabling prompts, and clear errors instead of hanging prompts
  - `NO_COLOR` MUST disable ANSI codes; the CLI MUST work in basic terminals and buffer lines properly in CI
  - Gated tests MUST print clear skip messages naming the missing prerequisite
  - Coverage targets: >= 80% unit for new configuration code, >= 70% integration for CLI workflows; CI pipeline under 10 minutes
- scope: CLI test tiers, non-interactive mode, terminal compatibility, skip messages

## REQ-scheduler-port
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (US-23.5, FR-23.5.1 to FR-23.5.3, FR-23.5.7)
- description: Application-layer scheduling port with a production cron adapter.
- acceptance:
  - A `SchedulerPort` trait MUST be defined in the application layer exposing at minimum `schedule_job(JobSpec) -> Result<JobId, SchedulerError>`, `cancel_job(JobId)` and job status retrieval
  - A `SchedulerAdapter` MUST live in `src/infrastructure/adapters/scheduling/` and wrap `tokio-cron-scheduler` (PRD pins `tokio-cron-scheduler = "0.9"`)
  - The adapter MUST support creating jobs from cron expressions or intervals, cancelling pending jobs by ID, tracking job state (scheduled, running, completed, failed), configurable retry logic and async job execution
  - Configuration MUST support `scheduler.{max_concurrent_jobs: 10, retry_failed_jobs: false, max_retries: 3, retry_delay_seconds: 60}`
  - Scheduled jobs are in-memory only in this phase; persistence across restarts is deferred
- scope: SchedulerPort, TokioCronSchedulerAdapter, scheduler configuration

## REQ-content-deliverer-scheduling
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (FR-23.5.4 to FR-23.5.6, Appendix)
- description: Replace the API content deliverer scheduler stub with real scheduling.
- acceptance:
  - `APIContentDeliverer::schedule_delivery()` MUST create real scheduled jobs, replacing the `unimplemented!("Scheduler integration pending")` stub at `src/infrastructure/adapters/output/api_content_deliverer.rs` line 297
  - Jobs MUST execute content delivery at the specified times and return a `JobId` for tracking
  - Cancellation of pending scheduled deliveries MUST be supported
  - Job failures MUST be logged and optionally retried
  - Unit tests with a mock scheduler MUST cover job creation, cancellation, successful execution, failure with retry and state transitions
  - An integration test MUST schedule a job with a short delay (e.g. 2 seconds) and verify execution timing within tolerance and correct state updates
  - Zero `unimplemented!()` macros may remain in production code paths
- scope: APIContentDeliverer, schedule_delivery, cancellation

## REQ-cli-error-types
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md (section 7 Error Handling)
- description: Unified CLI error surface for the new configuration paths.
- acceptance:
  - `CliError` MUST gain `GarrisonError(#[from] GarrisonError)`, `ArsenalError(#[from] ArsenalError)` and `SchedulerError(#[from] SchedulerError)` alongside existing variants
  - Configuration errors MUST remain distinguishable from network, validation and user errors
  - Existing `CliError` variants MUST be preserved
- scope: CliError, error conversion at CLI boundary

## REQ-mock-arsenal-port
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md (FR-1.1 to FR-1.4)
- description: In-process mock Arsenal for tool-call testing.
- acceptance:
  - `MockArsenalPort` MUST implement the `ArsenalPort` trait (`list_armaments`, `invoke`, `validate_call`)
  - It MUST support a configurable armament list, pre-configured `invoke()` responses keyed by tool name, configurable per-tool error responses and invocation recording (call count, arguments received)
  - It MUST be `Send + Sync` and usable as `Arc<dyn ArsenalPort>`
  - It MUST live in `tests/helpers/mock_arsenal_adapter.rs` alongside the existing `MockLlmAdapter`
  - It SHOULD be designed generically for reuse beyond Epic 23
- scope: MockArsenalPort, test helpers

## REQ-tool-call-loop-tests
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md (US-1, US-2, US-3, FR-2.1 to FR-2.9)
- description: End-to-end coverage of the LLM to Arsenal tool-call loop.
- acceptance:
  - Core tests MUST live in `tests/cli/tool_integration_test.rs` and use only in-process mocks (no network, no Python process)
  - `test_tool_call_basic_flow`: mock LLM returns a `calculator` tool call then a text answer; mock Arsenal returns a successful `ArmamentResult`; execution succeeds, output contains the tool result, LLM called twice, Arsenal called once
  - `test_tool_call_result_fed_back_to_llm`: the second LLM call's context MUST include the formatted tool result
  - `test_tool_call_no_arsenal_available`: a tool call with `arsenal: None` MUST complete without error, logging a warning
  - `test_tool_call_unknown_tool`: `ArsenalError::ToolNotFound` MUST inject an error message into context and continue to the next iteration
  - `test_tool_call_invalid_arguments`: malformed JSON arguments MUST yield `ArsenalError::InvalidArguments` handled gracefully
  - `test_tool_call_execution_error`: `ArsenalError::ExecutionError` MUST produce a formatted error in output and continue
  - `test_multiple_sequential_tool_calls`: two tool calls then a final answer MUST invoke both tools, place both results in context and call the LLM three times
  - `test_tool_call_with_garrison`: the tool result MUST be stored in Garrison as a `ConversationRole::Tool` entry
  - All error scenarios MUST degrade gracefully with no panic; target >= 8 core tests green in CI
- scope: PaladinExecutionService tool loop, ToolResultFormatter, error injection path

## REQ-mcp-gated-integration-tests
- source: /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md (US-4, FR-3.1 to FR-3.4)
- description: Optional real-MCP-server tests behind an explicit gate.
- acceptance:
  - Gated tests MUST live in `tests/integration/tool_integration_mcp_test.rs` and be marked `#[ignore]` or feature-gated
  - `test_full_mcp_stdio_tool_call_flow` MUST start `tests/mcp_test_server.py` through a real `MCPStdioAdapter`, discover tools, register them in `ArsenalRegistryService`, wrap in `ArsenalExecutionService`, and prove the `echo` tool result appears in execution output
  - `test_mcp_calculator_tool_invocation` MUST invoke the calculator tool with `{"operation":"add","a":5,"b":3}` and assert the result contains "8"
  - Tests MUST check Python availability at start and skip gracefully when unavailable
  - Target: >= 2 gated tests green when run with `--ignored` and Python available
  - SSE MCP server integration, real LLM providers, new Arsenal features, benchmarks, handoff tool-call testing and streaming-with-tools are explicit non-goals
- scope: MCP STDIO integration tests, gating, Python test server

---

## REQ-battalion-benchmark-repair
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.1, FR-4.1)
- description: Restore compiling Battalion benchmarks. Related to the run-1 finding that benchmarks were disabled at MVP.
- acceptance:
  - `benchmark_campaign` MUST be updated to the current Campaign API (add_node/add_edge) and `benchmark_chain_of_command` to the current constructor signature (`benches/battalion_benchmarks.rs` lines 297, 390, 950)
  - Both benchmarks MUST be re-enabled in criterion group registration
  - All benchmarks in `benches/` MUST compile without errors or warnings and use current API signatures
  - Benchmark results MUST be reproducible and documented in `docs/BATTALION_BENCHMARKS.md`
  - Benchmark documentation MUST explain what each benchmark measures
  - `cargo bench --no-run` MUST compile all benchmarks successfully in CI to catch API drift early
- scope: battalion_benchmarks.rs, criterion registration, benchmark documentation

## REQ-prompt-generation-test-reenable
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.2)
- description: Re-enable the disabled prompt generation service test module.
- acceptance:
  - The mock in `tests/unit/mod.rs` (line 22) MUST be updated to match the current `LlmPort` trait signature
  - The `prompt_generation_service_test` module MUST be uncommented, fixed and passing
  - Prompt generation test coverage MUST reach >= 80%
  - No `#[ignore]` attributes may remain on passing tests
- scope: tests/unit/mod.rs, prompt generation coverage

## REQ-timeout-test-hardening
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.3, FR-4.2)
- description: Verified execution timeout behaviour.
- acceptance:
  - `MockLlmPort` MUST support configurable response delays
  - A timeout test MUST verify 60-second timeout behaviour and have its `#[ignore]` removed (`tests/unit/paladin_execution_service_test.rs` lines 237, 239)
  - The test MUST pass reliably in CI with no flakiness
  - Both hard timeout and graceful shutdown scenarios MUST be covered, plus edge cases (timeout at 0s, timeout greater than max_duration)
- scope: MockLlmPort delays, timeout verification

## REQ-qdrant-integration-tests
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.4, FR-4.4)
- description: Validate the Sanctum RAG pipeline against a real Qdrant instance.
- acceptance:
  - Integration tests MUST be implemented in `tests/integration/rag_integration_tests.rs` (replacing the placeholder at line 147) covering store, search, delete and update against a real Qdrant
  - Tests MUST cover end-to-end RAG-enabled Paladin execution, token budget limiting and context formatting with real vector search results
  - Tests MUST run against a local Qdrant instance at `http://localhost:6333` via Docker and skip when unavailable
  - Unit-level Qdrant tests in `tests/unit/sanctum/qdrant_sanctum_test.rs` (line 62) MUST be expanded
  - Tests MUST create and destroy test collections so no state persists between runs
  - Vector search MUST be validated with different similarity metrics; integration coverage >= 70%
- scope: Qdrant integration tests, RAG end-to-end validation, test isolation

## REQ-deferred-coverage-review
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.5, FR-4.3)
- description: Cost/benefit review of the two remaining low-coverage platform modules.
- acceptance:
  - Coverage MUST be reviewed for `src/core/platform/manager/user_service.rs` (was 4.23%) and `src/core/platform/manager/listener_service.rs` (was 57.83%)
  - A cost/benefit determination MUST be made and tests implemented where ROI justifies the effort
  - Any item re-deferred MUST be documented with explicit rationale
  - Overall project coverage MUST be maintained at >= 80% unit and >= 70% integration
  - A coverage report MUST be generated and reviewed in the PR; recommended tool `cargo llvm-cov`
  - If 80% is unreachable for these modules, the lower figure MUST be documented with a follow-up technical-debt ticket
- scope: user_service coverage, listener_service coverage, deferred-coverage rationale

## REQ-cli-snapshot-testing
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.6, FR-4.5)
- description: Snapshot-based regression testing for CLI output.
- acceptance:
  - The `insta` crate MUST be added (>= 1.34, latest stable) and a `tests/cli/snapshots/` directory created
  - Snapshot tests MUST cover table rendering for all table formats, progress indicators and spinners, formatted and coloured error messages, and command help output for all subcommands
  - Snapshots MUST capture terminal output including ANSI codes and be reviewable with `cargo insta review`
  - CLI tests MUST validate both success and error output formatting; help output MUST be stable and properly formatted
  - At least 10 snapshot tests MUST exist
  - Inline rustdoc MUST be added for all public CLI functions and types
  - `QUICKSTART.md`, `INSTALLATION.md` and `docs/cli/README.md` MUST be updated with CLI usage, installation and comprehensive documentation
  - Recommended: inline snapshots for easier PR review
- scope: insta snapshot tests, CLI output regression, CLI documentation

## REQ-provider-live-api-tests
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.7, FR-4.2 item 9, section 6.2)
- description: Optional live API validation for all three LLM providers. COMPETING with the post-Epic-24 change to fail loudly on missing keys — see INGEST-CONFLICTS.md.
- acceptance:
  - Integration tests MUST exist for OpenAI, DeepSeek and Anthropic live APIs, each requiring its API key from an env var or `.env` file
  - Tests MUST validate completion, streaming, tool calling, error handling and rate limits
  - Tests MUST be gated behind the `live-api-tests` feature flag and excluded from default `cargo test`
  - Tests MUST skip gracefully when API keys are unavailable — no failures, just warnings
  - Tests MUST respect provider rate limits with appropriate delays/retries
  - README MUST document how to run live API tests
  - Recommended: cache responses for determinism and cap at roughly 10 API calls per provider per CI run
- scope: live API tests, feature gating, graceful skip

## REQ-final-documentation-and-demo
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (US-24.8, FR-4.6)
- description: Presentation-ready documentation and demo assets.
- acceptance:
  - `README.md` MUST be updated with comprehensive Council and Grove examples; `docs/QUICKSTART.md` MUST gain Council/Grove quickstart guides
  - A demo video, animated GIF or terminal recording of CLI features MUST be created and stored in an appropriate location (e.g. `docs/assets/`); recommended format asciinema
  - A CI/CD test job specifically for CLI tests MUST be added
  - `cargo doc --open` MUST generate clean documentation with no warnings; all public APIs (functions, types, modules) documented with rustdoc including practical examples
  - `docs/Design/Design_and_Architecture.md` MUST be reviewed and updated; `CONTRIBUTING.md` MUST gain testing guidelines
  - Release notes for Milestone 3 completion MUST be drafted
- scope: README, QUICKSTART, demo assets, rustdoc, CONTRIBUTING, release notes

## REQ-epic24-quality-gates
- source: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md (FR-4.7, section 7)
- description: Final quality gate set for Milestone 3.
- acceptance:
  - `cargo fmt --check` MUST pass; `cargo clippy -- -D warnings` MUST pass
  - `cargo test` MUST pass with no ignored tests remaining; `cargo test --features live-api-tests` MUST pass when API keys are provided
  - `cargo bench --no-run` MUST compile all benchmarks
  - Unit coverage MUST be >= 80% for all modules and integration coverage >= 70% for critical workflows, with coverage reports generated in CI
  - The number of enabled tests MUST increase by at least 50; 100% of benchmarks compile and run; 100% of public APIs have rustdoc
  - Pre-existing TODOs outside the multi-agent scope (content services, `sql_store.rs`, `trigger.rs`, repositories), performance optimisation, load testing, security testing, UI redesign and breaking API changes are explicit non-goals
- scope: Milestone 3 quality gates, coverage targets, scope exclusions
