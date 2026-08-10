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

---

# Ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements`

13 PRDs consumed (32 docs total: 13 PRD, 19 DOC, 0 ADR, 0 SPEC).

All file paths below are **as written in the source PRDs** and are largely historical: these three
milestones restructured the tree that runs 1-2 describe, and milestones outside this run moved it
again. Resolve current locations through `.planning/intel/code-verification.md` and
`.planning/codebase/*.md`, never through these paths.

`-v1` / `-v2` IDs are competing variants preserved verbatim from different PRDs on the same scope.
They are NOT merged. See `.planning/INGEST-CONFLICTS.md` WARNINGS. Where shipped code settles a
variant, the entry says so and points at `code-verification.md`.

---

## REQ-feature-flag-matrix
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR1)
- description: Expand Cargo feature flags from 5 thin flags to a comprehensive gating surface so consumers compile only the subsystems they use.
- acceptance:
  - LLM provider flags MUST exist: `llm-openai`, `llm-anthropic`, `llm-deepseek`, plus an `llm-all` convenience flag
  - The `LlmPort` trait MUST always be compiled; only concrete adapter implementations and provider-specific dependencies are gated
  - Subsystem flags MUST exist: `content-processing` (pdf-extract, scraper, tiktoken-rs, rss), `web-server` (both `actix-web` and `axum` and all HTTP/API infrastructure), `notifications` (lettre and notification publisher adapters), `vision`
  - Existing flags MUST be retained unchanged: `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`
  - The originally planned `mcp-arsenal` flag is ELIMINATED from scope (note dated 2026-04-15): Arsenal and its MCP transport adapters remain unconditionally compiled as core framework components
- scope: Cargo feature flags, LLM adapters, content processing, web server, notifications, vision

## REQ-vision-feature-gating
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR1 vision, Design Considerations "Dependency Grouping")
- description: The `vision` flag gates the vision pipeline, its adapters, and the Sentinel Vision encryption dependencies.
- acceptance:
  - `vision` MUST gate the vision pipeline and vision adapters (`openai_vision.rs`, `anthropic_vision.rs`)
  - `vision` MUST gate `chacha20poly1305` and `zeroize` as "Sentinel Vision encryption deps"
  - `vision` MUST gate `VisionPort` / `VisionCapableLlm` trait implementations
- scope: vision feature flag, vision adapters, encryption dependencies
- note: CONTRADICTED by `Epic_1/dependency-matrix.md`, which classifies `chacha20poly1305` and `zeroize` as general-purpose deps of `security/encryption.rs` that must stay unconditional, and by shipped code where `vision = []` gates no dependency at all. See INGEST-CONFLICTS.md WARNINGS and code-verification.md.

## REQ-feature-default-set
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR2, US-3, Non-Goals)
- description: Replace the current default feature set with a minimal orchestration-only default, accepting the breaking change.
- acceptance:
  - Old default `["redis-queue", "s3-storage", "openai-embeddings"]` MUST be replaced with `["llm-openai"]`
  - The breaking change is intentional; no multi-version deprecation cycle is provided
  - A migration guide in `CHANGELOG.md` MUST explain the change and give the exact flags to restore old behaviour
  - Existing examples MUST be updated to use the new defaults or explicitly specify the old features
  - Existing downstream consumers MUST be able to migrate with fewer than 5 lines of `Cargo.toml` change
- scope: default feature set, breaking change, migration guide

## REQ-feature-full-flag
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR1 Convenience Flags, US-2)
- description: A single `full` flag enabling every optional subsystem.
- acceptance:
  - `full` MUST enable all LLM providers, content-processing, web-server, notifications, vision, redis-queue, s3-storage, openai-embeddings and qdrant
  - `paladin = { features = ["full"] }` MUST give the production-ready experience without enumerating flags
  - All integration tests MUST pass with `--all-features`
- scope: full convenience feature flag

## REQ-cfg-guard-discipline
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR3, FR4, Technical Considerations)
- description: Conditional-compilation discipline for all feature-gated code.
- acceptance:
  - `#[cfg(feature = "...")]` guards MUST be applied at module declarations, `use` statements for gated imports, port implementation registrations and adapter instantiations, and test modules depending on gated features
  - Unavailable adapters MUST fail at compile time, not with runtime errors; no dynamic runtime feature detection is required
  - `#[allow(dead_code)]` MUST NOT be used to suppress `cfg`-gated code
  - `src/infrastructure/adapters/llm/provider_factory.rs` MUST gain `#[cfg]` guards on each provider import and on the factory match arm for unavailable providers
- scope: conditional compilation, provider_factory.rs, dead-code warnings

## REQ-feature-flag-docs
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR5)
- description: Documentation and migration deliverables for the feature-flag expansion.
- acceptance:
  - `/docs/CONFIGURATION.md` MUST document all feature flags
  - `/docs/FEATURE_FLAGS.md` MUST be created with per-flag explanation, use cases and examples
  - `/docs/MIGRATION.md` MUST explain the breaking change and how to migrate
  - `README.md` MUST gain a features table; `CHANGELOG.md` MUST carry a breaking-change notice
  - `/examples` MUST gain feature-flag examples demonstrating minimal and full builds (at least one example per subsystem feature)
- scope: CONFIGURATION.md, FEATURE_FLAGS.md, MIGRATION.md, README, CHANGELOG, examples
- note: these root/`docs/` paths are historical — equivalent pages ship as mdbook chapters. See code-verification.md.

## REQ-feature-ci-matrix
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md (FR6, Success Metrics)
- description: Systematic CI testing of feature-flag combinations.
- acceptance:
  - `cargo build --no-default-features`, `cargo build` (default) and `cargo build --all-features` MUST all pass
  - Each LLM provider flag MUST build in isolation; `web-server`, `content-processing` and `notifications` MUST each build alone; `redis-queue` plus compatible combinations MUST build
  - Every combination MUST compile without errors, run `cargo test` successfully or skip tests via `#[cfg]`, produce no `cargo clippy -D warnings` output, and pass `cargo fmt --check`
- scope: CI feature matrix, build combinations

## REQ-curated-lib-exports
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-1)
- description: Replace the glob re-exports in `src/lib.rs` with explicit, curated `pub use` statements.
- acceptance:
  - The four glob re-exports (`pub use application::*; pub use config::*; pub use core::*; pub use infrastructure::*;`) MUST be removed
  - All ~20 port traits from `application::ports::{input, output}` MUST be exported
  - Essential domain entities MUST be exported: `Paladin`, `PaladinData`, `PaladinConfig`, `PaladinResult`, `PaladinStatus`; Battalion types `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`; `Garrison`, `Arsenal`, `Armament`, `Citadel`; base types `Node<T>`, `Collection`, `Field`, `Message`
  - Builders (`PaladinBuilder`, `BattalionBuilder` if applicable), configuration types (`ApplicationSettings` and relevant subsystem configs) and error types (`PaladinError`, `BattalionError`, `GarrisonError`, …) MUST be exported
  - Adapter implementations, repository implementations, CLI modules, manager services and internal infrastructure utilities MUST NOT be exported
- scope: src/lib.rs, public API surface, curated exports

## REQ-visibility-hardening
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-2)
- description: Apply `pub(crate)` / `pub(super)` to internal modules and types.
- acceptance:
  - Adapter module internals MUST be `pub(crate)`: llm (`openai_adapter`), garrison adapters, queue (`redis_adapter`), and all other adapter private types
  - Repository implementations MUST be `pub(crate)`
  - CLI modules MUST be `pub(crate)` (coordinated with Epic 3)
  - Manager services MUST be `pub(crate)` unless explicitly part of the public API
  - Port trait definitions MUST remain `pub` and exported
- scope: visibility modifiers, adapters, repositories, CLI, manager services

## REQ-port-trait-rustdoc
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-3, US-2, §6 style guide)
- description: Reference-grade rustdoc on every port trait.
- acceptance:
  - Each trait MUST document purpose (3-5 sentences), when to implement it, thread-safety guarantees (`Send + Sync` implications) and the async execution model
  - Each method MUST document its behaviour contract, parameter constraints, return semantics, all error variants and any panic conditions
  - Each trait MUST have a `# Examples` section with at least 2 compiling examples — one basic, one advanced/custom-implementation
  - Each trait MUST have `# Implementation Notes` with best practices, common pitfalls, performance considerations and links to reference implementations
  - Minimum trait list: `LlmPort`, `GarrisonPort`, `SanctumPort`, `EmbeddingPort`, `ArsenalPort`/`ArsenalRegistry`, `PaladinPort`, `BattalionPort`, `CitadelPort`, `QueuePort`, `NotificationPort`, `FileStoragePort`, and all input ports
- scope: port trait rustdoc, examples, implementor guidance

## REQ-stable-api-doc
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-4)
- description: A `STABLE_API.md` catalogue of the public API surface with stability guarantees.
- acceptance:
  - MUST contain: introduction, versioning policy, stability tiers (Stable / Unstable-Experimental / Deprecated), the public types catalogue, and the API change process
  - Catalogue sections MUST cover output port traits, input ports, domain entities, builders, configuration types, error types and base types
  - Each entry MUST give fully qualified path, stability tier, one-sentence description, rustdoc link and breaking-change policy
  - The versioning policy MUST define a breaking change per SemVer, a deprecation process of at least 1 minor version notice, feature-flag impact on stability, and crate-split impact for future workspace decomposition
  - Format MUST use tables, a linked table of contents, alphabetical ordering within sections, and a last-updated date
  - Feature-gated adapters MUST be clearly marked
- scope: STABLE_API.md, stability tiers, versioning policy

## REQ-import-path-updates-m4
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-5)
- description: Update examples and integration tests to the new import paths.
- acceptance:
  - All files under `examples/` and `tests/` MUST be audited for direct imports of types that will no longer be re-exported
  - Imports MUST be updated to explicit paths; `#[allow(deprecated)]` MUST be added to any code using deprecated paths during transition
  - All 193+ examples MUST compile and all 1,487+ tests MUST pass after the changes
- scope: examples, integration tests, import paths

## REQ-doc-build-clean
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-6, US-4)
- description: Clean documentation generation with zero warnings.
- acceptance:
  - `cargo doc --no-deps` MUST complete with zero warnings
  - All intra-doc links MUST resolve (no `[broken link]` warnings)
  - All public items MUST have rustdoc
  - Generated HTML MUST place port traits prominently in the sidebar and clearly mark any internal types that remain public
- scope: cargo doc, intra-doc links, rustdoc coverage

## REQ-api-surface-ci
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-7, US-5)
- description: CI tooling that detects public API surface changes.
- acceptance:
  - `cargo-public-api` (or equivalent) MUST be installed and configured in CI
  - A baseline API snapshot MUST be generated on `main` and stored in version control as `.public-api-baseline.txt`
  - The job MUST run on every PR targeting `main`, generate an API diff, post it as a PR comment or artifact, and fail the check on breaking changes unless labelled `breaking-change`
  - The baseline update process MUST be documented in `CONTRIBUTING.md`
  - The tooling MUST detect removal of deprecated items and verify the deprecation was present for the required period
- scope: cargo-public-api, CI, API baseline

## REQ-deprecation-warnings
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (FR-8)
- description: Deprecation warnings for types leaving the public API.
- acceptance:
  - Types currently public but not intended for the stable API MUST be identified from the Task 2.1 audit
  - Each MUST carry `#[deprecated(since = "X.Y.Z", note = "Use ... instead. This will be made private in version X+1.Y.Z")]`
  - The note MUST name a public alternative, or explain the pattern to use instead
  - Deprecated types MUST be marked "Deprecated" in `STABLE_API.md`
  - A removal timeline of at least 1 minor version MUST be planned
- scope: deprecation annotations, migration guidance
- note: VERIFIED OPEN — zero `#[deprecated]` attributes exist in the shipped tree. See code-verification.md.

## REQ-api-surface-reduction-target
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/prd-harden-port-traits-stable-api.md (§8 Success Metrics)
- description: Quantitative targets for the API hardening work.
- acceptance:
  - Public API surface MUST be reduced from ~200+ exported types to <= 50, measured by `cargo public-api --simplified | wc -l`
  - Documentation coverage MUST reach 100% of public items; broken intra-doc links MUST be 0
  - Test pass rate MUST stay at 100% (1,487+ tests); all 193+ examples MUST compile
  - CI build time MUST NOT increase
  - Port trait signatures MUST NOT change — only documentation and visibility (explicit non-goal)
- scope: API surface size, coverage, regression budget

## REQ-cli-feature-gate
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR1, §6.3)
- description: Gate the entire `src/application/cli/` module tree behind a single `cli` feature flag.
- acceptance:
  - `#[cfg(feature = "cli")]` MUST be applied to the `application::cli` module declaration in `src/application/mod.rs` and to all sub-modules
  - All CLI type re-exports in `src/lib.rs` MUST be gated with `#[cfg(feature = "cli")]`
  - No dead-code warnings when the `cli` feature is disabled
  - A single `cli` flag MUST be used rather than granular flags (`cli-agent`, `cli-battalion`, …) — the CLI is a cohesive unit
  - Gating MUST be applied at the highest module boundary possible, not per-type
- scope: cli feature flag, src/application/cli, src/lib.rs

## REQ-cli-dependency-isolation
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR2, §6.5)
- description: CLI-only dependencies must not compile in library builds.
- acceptance:
  - CLI-exclusive dependencies MUST be `optional = true` and enabled by the `cli` feature
  - Every dependency MUST be classified CLI-only, shared, or core; only exclusively-CLI dependencies may be optional
  - Preliminary CLI-only list: `clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`, `console`, and any CLI-specific `serde_yaml` usage
  - Shared dependencies (`serde`, `serde_json`, `tokio`, `anyhow`, `thiserror`, core domain deps) MUST stay unconditional
  - `cargo tree --no-default-features` MUST exclude CLI dependencies; `cargo tree --features cli` MUST include them
- scope: Cargo.toml optional dependencies, dependency classification
- note: PARTIALLY SHIPPED — `structopt`, `colored` and `comfy-table` remain unconditional. See code-verification.md.

## REQ-binary-target-config
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR3, §6.2, Q1)
- description: Both binary targets must enable the `cli` feature and compile.
- acceptance:
  - `src/main.rs` and `src/bin/paladin-cli.rs` MUST enable the `cli` feature
  - `cargo build --bin paladin` and `cargo build --bin paladin-cli` MUST both succeed
  - The relationship and intended use case of each binary target MUST be documented
  - The architecture for `src/main.rs` versus `paladin-cli` requires an architecture review — three options are on the table (keep both with distinct purposes; consolidate to `paladin-cli`; keep `paladin` as a lightweight wrapper). Recorded status: "User selected Option D — requires architecture review"
  - `cli` MUST NOT be in the `default` feature set (recommendation): library use is the primary case and binary builds can add `features = ["cli"]`
- scope: binary targets, paladin, paladin-cli, default features
- note: Q1 was never answered by a decision record; three binaries ship (`paladin`, `paladin-cli`, `paladin-server`). See code-verification.md.

## REQ-cli-test-isolation
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR4, §6.4)
- description: All 193 CLI tests compile only when the `cli` feature is enabled.
- acceptance:
  - All CLI test modules MUST be wrapped with `#[cfg(feature = "cli")]`
  - All 193 CLI tests MUST pass with `cargo test --features cli`
  - `cargo test --lib --no-default-features` MUST produce no CLI test failures or warnings
  - Snapshot test files (`.snap`) MUST remain committed and functional in place
  - CI MUST run CLI tests with `--features cli`
  - CLI tests MUST stay in their current location; no test files are moved
- scope: CLI tests, feature gating, snapshot tests

## REQ-library-only-build
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR5, US-1)
- description: The library must compile and function without CLI code or dependencies.
- acceptance:
  - `cargo build --lib --no-default-features` MUST succeed
  - `cargo build --lib` (default features, no `cli`) MUST succeed
  - All core framework functionality (Paladin, Battalion, Arsenal, Garrison) MUST be available without `cli`
  - `cargo tree --lib --no-default-features` MUST contain zero CLI code or dependencies
  - A downstream project depending on `paladin` MUST NOT compile `clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored` or `console`
- scope: library-only build, dependency tree

## REQ-library-only-integration-tests
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR6, US-4, Q4)
- description: Integration tests that verify library-only usage and catch CLI leakage.
- acceptance:
  - A library-only integration test suite MUST exist; recommended placement is `tests/library_only_*.rs` (naming convention in the existing `tests/` directory)
  - Tests MUST verify core agent orchestration, all four Battalion patterns (Formation, Phalanx, Campaign, Chain of Command), Arsenal tool execution and Garrison memory operations without CLI
  - Tests MUST explicitly fail if CLI dependencies are detected
  - CI MUST run them with `--no-default-features` or a minimal feature set
  - Minimum 5 new tests covering core functionality
- scope: library-only integration tests, CLI leakage detection

## REQ-cli-build-time-measurement
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR7, §8.1, §8.3)
- description: Measure and document build-time and dependency-count impact of CLI isolation.
- acceptance:
  - Baseline clean and incremental library build times MUST be captured before the change
  - Post-isolation measurements MUST be captured for library-only and CLI-enabled builds
  - Dependency counts before and after MUST be documented using `cargo tree`
  - Results MUST be documented in the milestone completion report
  - Any measurable improvement is acceptable — no specific percentage target
- scope: build-time measurement, dependency count, milestone report

## REQ-cli-ci-matrix
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR8, §7.2, §8.4)
- description: CI matrix entries covering library-only and CLI-enabled configurations.
- acceptance:
  - At least 6 new matrix entries MUST be added: `cargo build --lib --no-default-features`, `cargo build --lib`, `cargo build --bin paladin`, `cargo build --bin paladin-cli`, `cargo test --features cli`, and library-only integration tests
  - All matrix entries MUST be green
- scope: CI matrix, library/CLI build combinations

## REQ-cli-docs
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md (FR9, §8.5)
- description: Documentation updates for the CLI isolation change.
- acceptance:
  - `README.md` MUST document the `cli` feature flag; `CONTRIBUTING.md` MUST document CLI testing requirements
  - The binary target architecture decision MUST be documented after the architecture review
  - A `CHANGELOG.md` entry MUST mark this as a breaking change for consumers who import CLI types or rely on CLI being available by default
  - A contribution to the cross-epic migration guide MUST be prepared
  - `cargo clippy -- -D warnings`, `cargo fmt --check` and `cargo doc --no-deps` MUST be clean for all feature configurations
- scope: README, CONTRIBUTING, CHANGELOG, migration guide, quality gates

## REQ-cargo-workspace-root
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-1, FR-2, FR-3)
- description: Convert the repository root into a Cargo workspace manifest with shared dependency versions.
- acceptance:
  - Root `Cargo.toml` MUST contain a `[workspace]` section with `members = ["crates/*"]` plus any other existing crate paths
  - `[workspace.dependencies]` MUST declare shared versions for at minimum `serde` (derive), `uuid` (v4, serde), `chrono` (serde), `thiserror`, `tokio` (full), `async-trait`, `serde_json`, `reqwest` and `log`
  - Member crates MUST reference them with `dep = { workspace = true }` rather than pinning versions
  - The existing `paladin` crate MUST remain buildable: `cargo build` from the workspace root MUST succeed after the workspace manifest is created and before any source files are moved
- scope: workspace root, workspace.dependencies

## REQ-workspace-crate-edition-v1
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-5, §7 "Cargo Edition"); corroborated by Epic_2 FR-2, Epic_3 FR-2, Epic_4 FR-2
- description: All workspace crates use Rust edition 2021.
- acceptance:
  - `crates/paladin-core/Cargo.toml` MUST set `edition = "2021"`
  - "All crates in this workspace must use `edition = \"2021\"`. Do not use an older edition."
  - `paladin-ports`, `paladin-battalion` and `paladin-llm` MUST each set `edition = "2021"`
- scope: Cargo edition, all workspace crates
- note: COMPETING with REQ-workspace-crate-edition-v2. Shipped tree is mixed — see code-verification.md.

## REQ-workspace-crate-edition-v2
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-1.2); corroborated by overview/Milestone_5-Tier_2-Workspace-Decomposition.md Appendix D
- description: All workspace crates use Rust edition 2024, matching the workspace root.
- acceptance:
  - `crates/paladin-memory/Cargo.toml` MUST declare `edition = "2024"` "(matching the workspace root)"
  - The workspace `[workspace.package]` section MUST declare `edition = "2024"` alongside `version = "0.2.0"`
- scope: Cargo edition, workspace.package, paladin-memory
- note: COMPETING with REQ-workspace-crate-edition-v1, which is the later-dated PRD position for four other crates. Later position: v2 for `paladin-memory`, v1 for Epics 1-4 crates. Shipped tree is mixed.

## REQ-paladin-core-scaffold
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-4, FR-7, Story 1, Story 3)
- description: Scaffold `crates/paladin-core/` as an independently compilable domain crate.
- acceptance:
  - `crates/paladin-core/` MUST exist with a valid `Cargo.toml` and `src/lib.rs`
  - `Cargo.toml` MUST set `name = "paladin-core"` and reference workspace dependencies
  - `cargo build -p paladin-core` MUST succeed in isolation without building any other workspace member, in under 30 seconds on a standard developer machine
  - A downstream crate MUST be able to add `paladin-core` and use `Paladin`, `GarrisonEntry`, `Citadel` etc. with no transitive dependency on `reqwest`, `redis`, `sqlx` or any LLM provider SDK
- scope: paladin-core crate scaffold, isolated build

## REQ-paladin-core-dependency-allowlist-v1
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-6, Appendix B, Story 3)
- description: `paladin-core`'s dependency list is a closed set of six crates.
- acceptance:
  - `[dependencies]` MUST contain only `serde`, `uuid`, `chrono`, `thiserror`, `async-trait` and `serde_json`. "No other dependencies are permitted."
  - Appendix B is "the complete and exhaustive list of external crates that `paladin-core` is allowed to depend on"; any other dependency requires explicit approval and a documented justification
  - `async-trait` is permitted because some domain types use `#[async_trait]`; `serde_json` because some domain types use `serde_json::Value`
- scope: paladin-core dependencies, dependency allowlist
- note: COMPETING with REQ-paladin-core-dependency-allowlist-v2. Shipped `paladin-core` carries 14 dependencies — see code-verification.md.

## REQ-paladin-core-dependency-allowlist-v2
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (§9 Open Question 4)
- description: `paladin-core` also depends on `petgraph` for the Campaign DAG domain types.
- acceptance:
  - "`petgraph` is used by both `paladin-core` (Campaign DAG domain types) and `paladin-battalion` (Campaign execution service topological sort)"
  - Both MUST use the same workspace-pinned version to avoid duplicate compilation of `petgraph`
- scope: paladin-core dependencies, petgraph, version alignment
- note: COMPETING with REQ-paladin-core-dependency-allowlist-v1, which declares its 6-crate list exhaustive. This is the later PRD (2026-05-17 versus 2026-04-21). Shipped code confirms `petgraph` in `paladin-core`.

## REQ-core-base-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-8 to FR-12)
- description: Relocate `src/core/base/` into `paladin-core`.
- acceptance:
  - All files MUST move to `crates/paladin-core/src/base/`, preserving the module tree (`mod.rs` declarations, sub-modules)
  - Types MUST include at minimum `Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event` and any internal helper types they reference
  - All `use` statements MUST be updated to crate-local paths (`crate::base::…`) instead of monolith paths (`crate::core::base::…`)
  - No moved file may contain a `use` statement referencing `application::`, `infrastructure::` or any path outside `paladin-core`
  - All previously-embedded `#[cfg(test)]` unit tests MUST compile and pass via `cargo test -p paladin-core`
- scope: src/core/base, paladin-core/src/base, Node/Collection/Field/Message

## REQ-core-container-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-13 to FR-16, FR-18)
- description: Relocate `src/core/platform/container/` into `paladin-core`.
- acceptance:
  - All files MUST move to `crates/paladin-core/src/platform/container/`, preserving the module tree
  - Types MUST include at minimum `Paladin`, `PaladinData`, `PaladinConfig`, `PaladinStatus`, all Battalion domain types (`Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`, `Conclave`, `Council`, `Grove`, `Maneuver` with its lexer/AST/parser), `Garrison`, `GarrisonEntry`, `GarrisonConfig`, `Arsenal`, `Armament`, `Citadel`, `Herald`, `Sanctum`, `SanctumEntry`, `Memory`, `MemoryBuilder` and all supporting types
  - All `use` statements MUST be updated to crate-local paths
  - After the upward-dependency resolution, no moved file may reference `application::` or `infrastructure::`
  - All previously-embedded unit tests MUST compile and pass via `cargo test -p paladin-core`
- scope: src/core/platform/container, paladin-core, domain entities
- note: the Maneuver lexer/AST/parser inclusion here is SUPERSEDED by REQ-maneuver-files-moved-from-core (M6 Epic 3), which moves them out again into `paladin-battalion`.

## REQ-core-upward-dependency-resolution
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-17, §7, OQ-1, OQ-2, SM-5, SM-10, Story 4)
- description: Resolve the upward dependency from `battalion/mod.rs` into the application layer through a structured decision process.
- acceptance:
  - `src/core/platform/container/battalion/mod.rs` currently imports from `application::ports::output::paladin_port` and `application::ports::output::paladin_registry`; these imports MUST NOT exist in the extracted crate
  - The resolution approach is deliberately NOT specified in the PRD — it is the subject of a dedicated architectural decision task producing (a) an options-analysis document, (b) an implementer interview/decision task, (c) implementation sub-tasks generated from the chosen approach
  - The only hard constraint: after resolution `battalion/mod.rs` MUST NOT import from `application::` or any module outside `paladin-core`
  - `grep -r "application::" crates/paladin-core/` MUST return no results
  - A full audit MUST be performed for any other upward dependencies from `src/core/` beyond the known `battalion/mod.rs` coupling
  - The decision artifact MUST be committed under `project/Milestone_5-Workspace-Decomposition/Epic_1/`
- scope: battalion/mod.rs, upward dependency, decision task
- note: SETTLED by the Epic-1 decision record (Option A). See context.md and code-verification.md.

## REQ-port-value-type-ownership-v1
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md (Chosen Option A, Status: Approved, 2026-05-13)
- description: The five pure value/error types live in `paladin-core`; the application ports become thin re-exports.
- acceptance:
  - `PaladinResult` and `StopReason` MUST move to `core/platform/container/execution_result.rs`; `TokenUsage` to `token_usage.rs`; `RegistryError` to `registry_error.rs`; `HandoffError` to `arsenal/handoff_error.rs` (from `src/application/errors/handoff_error.rs`)
  - `paladin_port.rs`, `llm_port.rs`, `paladin_registry.rs` and `application/errors/handoff_error.rs` MUST have their struct/enum bodies removed and replaced with `pub use` re-exports from the core locations
  - Zero breaking changes: all existing `paladin::application::ports::output::…` paths MUST continue to resolve via re-exports
  - `PaladinError` is deliberately EXCLUDED because it depends on `GarrisonError` in `application::`; the convenience `pub use PaladinError` in `herald.rs` MUST be removed and callers must import it from `application::use_cases::paladin::error`
  - `TaskPlan` and `HandoffRecord` are already in core, so no circular dependency is introduced
  - These types travel to `paladin-core` in Task 5.0
- scope: PaladinResult, StopReason, TokenUsage, RegistryError, HandoffError, PaladinError exclusion, port re-exports
- note: COMPETING with REQ-port-value-type-ownership-v2. This document carries `Status: Approved` and reads as an ADR but is manifest-typed DOC, so it creates no locked decision. It settles the LOCATION of these five types only — it never mentions `BattalionResult`. Shipped code implements this variant.

## REQ-port-value-type-ownership-v2
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-7 inventory, FR-10)
- description: Port-associated value types are co-located with their trait inside `paladin-ports`.
- acceptance:
  - `paladin_port.rs` MUST export `PaladinPort`, `PaladinResult`, `StopReason` from `crates/paladin-ports/src/output/`
  - `llm_port.rs` MUST export `LlmPort`, `LlmRequest`, `LlmResponse`, `LlmError`, `TokenUsage`, `FinishReason`, `StreamingResponse`, `ToolCall`, `ToolResult`
  - "All associated types that are defined **within** a port module file (error enums, request/response structs, config structs, supporting enums) must move with their port trait into `paladin-ports`. Types must not be split across crates." (FR-10)
  - `RegistryError` is the single named exception: it MUST remain reachable at `paladin_ports::output::paladin_registry::RegistryError` and, if the underlying type lives in `paladin-core`, the re-export MUST point at `paladin_core::platform::container::registry_error::RegistryError` (FR-11)
- scope: paladin-ports output ports, associated type co-location
- note: COMPETING with REQ-port-value-type-ownership-v1. This is the later document (2026-05-15 versus 2026-05-13) and is PRD precedence, but it applies the FR-11 core-re-export carve-out only to `RegistryError`, leaving `PaladinResult`, `StopReason` and `TokenUsage` contradicting the Approved decision record. Shipped code follows v1.

## REQ-facade-core-reexports
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-19 to FR-23, SM-7, SM-8, §7 Visibility Rules)
- description: Wire the root `paladin` crate to re-export `paladin-core`, preserving all existing import paths.
- acceptance:
  - Root `Cargo.toml` MUST list `paladin-core = { path = "crates/paladin-core" }`
  - `src/lib.rs` MUST re-export `paladin-core` types under the existing module paths so `paladin::core::base::Node`, `paladin::core::platform::container::Paladin` etc. compile unmodified; the exact mechanism is an implementation detail but the result MUST be zero breaking changes
  - Fully-relocated files MUST be removed from `src/core/`, leaving it empty or a thin re-export shim only
  - `cargo build --workspace` MUST succeed and `cargo test --workspace` MUST pass with a test count no lower than the pre-epic baseline
  - Any type that was `pub(crate)` in the monolith and is referenced by the root crate MUST become `pub`; all `pub(crate)` declarations MUST be reviewed during extraction
  - Breaking changes to `paladin-core`'s own public API are permitted; the facade absorbs them
- scope: src/lib.rs re-exports, src/core removal, visibility promotion

## REQ-core-dependency-validation
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md (FR-24 to FR-26, SM-4, SM-6)
- description: Prove `paladin-core`'s dependency layering is clean.
- acceptance:
  - A dependency graph analysis (`cargo tree -p paladin-core` or equivalent) MUST confirm no transitive dependency on application or infrastructure crates
  - The output MUST NOT show any crate suggesting LLM provider SDKs (`openai`, `anthropic`, `deepseek`), database drivers (`sqlx`, `redis`, `mysql`), HTTP frameworks (`axum`, `actix`) or object storage clients (`minio`, `s3`)
  - `cargo doc -p paladin-core --no-deps` MUST produce documentation with zero broken intra-doc links and zero warnings
  - All CI pipeline checks MUST pass on the feature branch
- scope: dependency graph validation, cargo tree, cargo doc

## REQ-paladin-ports-scaffold
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-1 to FR-6, Story 4, Story 5, §7 Tokio)
- description: Scaffold `crates/paladin-ports/` as a minimal contract crate depending only on `paladin-core`.
- acceptance:
  - `crates/paladin-ports/` MUST exist with a valid `Cargo.toml` and `src/lib.rs`
  - `Cargo.toml` MUST set `name = "paladin-ports"` and use `dep = { workspace = true }` syntax
  - `[dependencies]` MUST contain only `paladin-core` (path dependency), `async-trait`, `serde`, `thiserror`, `uuid`, `chrono` and `tokio`; no others without explicit justification and a PRD update
  - `tokio` is required with at minimum the `sync` feature because streaming port methods use `tokio::sync::mpsc`
  - `cargo build -p paladin-ports` and `cargo test -p paladin-ports` MUST succeed without building any other workspace member beyond declared dependencies
  - `src/lib.rs` MUST declare `pub mod input;` and `pub mod output;` with crate-level doc comments describing the crate's role in the hexagonal architecture
  - `paladin-ports` MUST include all port modules unconditionally — including `vision_llm_port` and `vision_port` — and MUST define no feature flags of its own; the existing `#[cfg(feature = "vision")]` attributes MUST NOT be carried into `paladin-ports/src/output/mod.rs`. Feature gating of vision is the root crate's responsibility via conditional re-export
- scope: paladin-ports scaffold, dependency allowlist, vision port handling

## REQ-output-ports-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-7 to FR-11, §7 RegistryError, §7 CitadelError)
- description: Relocate all output port trait files into `crates/paladin-ports/src/output/`.
- acceptance:
  - The full inventory MUST move: `llm_port.rs`, `garrison_port.rs`, `sanctum_port.rs`, `embedding_port.rs`, `arsenal_port.rs`, `citadel_port.rs`, `queue_port.rs`, `notification_port.rs`, `file_storage_port.rs`, `paladin_port.rs`, `paladin_executor_port.rs`, `paladin_registry.rs`, `battalion_port.rs`, `log_port.rs`, `scheduler_port.rs`, `search_engine_port.rs`, `content_delivery_port.rs`, `vision_llm_port.rs`, `vision_port.rs`
  - The module tree structure MUST be preserved identically
  - Domain types imported from `crate::core::platform::container::*` MUST become `paladin_core::platform::container::*`; self-referential imports MUST use `crate::output::*`; no moved file may reference `crate::application::`, `crate::infrastructure::` or `crate::core::`
  - `CitadelError` (in `src/application/errors/citadel_error.rs`) is NOT a port trait and is explicitly out of scope; it stays in the `paladin` crate's `application::errors` module
- scope: output port traits, paladin-ports/src/output, import rewriting

## REQ-input-ports-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-12 to FR-14, §9 resolution 1)
- description: Relocate all input port trait files into `crates/paladin-ports/src/input/`.
- acceptance:
  - The full inventory MUST move: `content_input_port.rs` (`ContentIngestionPort`), `document_port.rs`, `listener_port.rs`, `ml_port.rs`, `nlp_port.rs`, `rpc_port.rs` (`RpcGatewayPort`)
  - Import rules follow FR-9: no references to `crate::application::`, `crate::infrastructure::` or `crate::core::`
  - Domain types from old monolith paths (e.g. `crate::core::platform::container::content::ContentItem`) MUST be updated to their `paladin-core` equivalents
  - The five previously-unspecified files (`content_delivery_port.rs`, `listener_port.rs`, `paladin_executor_port.rs`, `scheduler_port.rs`, `search_engine_port.rs`) are all IN scope — every file currently under `src/application/ports/` is extracted
- scope: input port traits, paladin-ports/src/input

## REQ-ports-facade-wiring
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-15 to FR-18, Story 3, §9 resolution 2)
- description: Wire `paladin-ports` into the root crate and fully delete `src/application/ports/`.
- acceptance:
  - `paladin-ports = { path = "crates/paladin-ports" }` MUST be added to the root crate's `[dependencies]` alongside the existing `paladin-core`
  - `src/application/ports/` MUST be **fully deleted** after extraction and import migration — Option B, full deletion, was selected; no shim files are left behind
  - `src/lib.rs` re-exports MUST be updated to resolve from `paladin_ports::` directly; every type previously reachable at `paladin::application::ports::*` MUST still be re-exported at an equivalent path
  - The `src/application/ports/{mod,input/mod,output/mod}.rs` files and the corresponding `pub mod` declarations MUST be removed so `src/application/ports/` no longer exists as a module path
  - The `vision` feature flag MUST remain in the root crate exactly as before, gating the re-exports with `#[cfg(feature = "vision")]`
- scope: root crate wiring, full deletion of src/application/ports, src/lib.rs re-exports

## REQ-ports-import-migration
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-19, FR-20, §6 migration, §7 incremental strategy)
- description: Migrate every import site off `crate::application::ports::`.
- acceptance:
  - All 314 occurrences of `use crate::application::ports::` across 76 `.rs` files MUST be updated to import from `paladin_ports::` directly or from the updated `src/lib.rs` re-exports, spanning `src/infrastructure/`, `src/application/use_cases/`, `src/application/` and `src/lib.rs`
  - Only `use` statement path strings may change — no function bodies, trait implementations, struct definitions or test assertions
  - A scripted bulk find-and-replace (e.g. `s/crate::application::ports::/paladin_ports::/g`) followed by `cargo build --workspace` is the recommended approach
  - Recommended extraction order: scaffold, extract output ports one file at a time building after each, extract input ports one at a time, update `src/lib.rs`, bulk-migrate the 314 `.rs` occurrences, bulk-update docs, delete `src/application/ports/`, run the full test suite
- scope: 314 import occurrences, 76 files, scripted migration

## REQ-ports-doctest-compilation
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-21, SM-8)
- description: Rustdoc examples embedded in port files must compile.
- acceptance:
  - Rustdoc examples inside `//!` and `///` blocks in the moved port files MUST be updated to import paths that compile under `cargo test --doc`
  - "Broken doc examples are treated as test failures"
  - `cargo doc -p paladin-ports --no-deps` MUST complete with zero broken intra-doc link errors
- scope: paladin-ports doctests, cargo test --doc
- note: VERIFIED OPEN — `paladin-ports/Cargo.toml` sets `[lib] doctest = false` with a comment deferring the fix to "Task 7.0", and CI runs `cargo test --workspace --doc --exclude paladin-ports`. See code-verification.md.

## REQ-ports-docs-markdown-update
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-22)
- description: Update Markdown documentation that names the old port paths.
- acceptance:
  - The 12 occurrences of `application::ports::` paths across 5 files under `docs/` MUST be updated
  - The docs MUST accurately reflect how consumers import port traits after the refactor
- scope: docs/ markdown, port import paths

## REQ-ports-layering-validation
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-23 to FR-26, Story 1, Story 2, SM-4, SM-5, SM-9)
- description: Prove `paladin-ports` has no infrastructure dependencies and no public path regressions.
- acceptance:
  - `cargo tree -p paladin-ports` MUST show no transitive dependency on `redis`, `sqlx`, `aws-sdk-s3`, `minio`, storage/queue-originated `reqwest`, `openai`, `anthropic` or any LLM provider SDK
  - `cargo tree -p paladin-ports` MUST list exactly one workspace-internal dependency: `paladin-core`
  - The confirmed-green `cargo build -p paladin-ports` output MUST be saved to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-isolation-build.txt`
  - The `cargo tree -p paladin-ports` output MUST be saved to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-dependency-tree.txt`
  - At least three existing examples (`basic_paladin.rs`, `formation_sequential.rs`, `garrison_in_memory.rs`) MUST pass `cargo check --example <name>`, confirming no public import path was broken
- scope: dependency isolation, evidence artifacts, example verification

## REQ-ports-tests-and-rustdoc
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md (FR-27 to FR-30, SM-1 to SM-3, SM-6, SM-7, §5 Non-Goals)
- description: Test and documentation preservation for the ports extraction.
- acceptance:
  - All unit tests previously in `src/application/ports/` MUST compile and pass via `cargo test -p paladin-ports`
  - `cargo test --workspace` MUST report the same number of passing tests as the Epic 1 baseline — zero regressions
  - All public items in `paladin-ports` MUST have rustdoc; existing doc comments MUST be preserved with no documentation lost during the move
  - `cargo clippy --workspace -- -D warnings` MUST report zero warnings; `cargo fmt --all --check` MUST pass without changes
  - No behavioural changes: no trait method signature, error variant or associated type may change; no new port traits may be added; `paladin-core` MUST NOT be modified; the CLI MUST remain untouched
- scope: port unit tests, rustdoc preservation, zero-regression constraint

## REQ-battalion-crate-scaffold
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-1 to FR-5, Story 1, Story 4, §7.1)
- description: Scaffold `crates/paladin-battalion/` with an exact dependency set and no infrastructure crates.
- acceptance:
  - `crates/paladin-battalion/` MUST exist with a valid `Cargo.toml` and `src/lib.rs`, `name = "paladin-battalion"`, using `dep = { workspace = true }` for shared dependencies
  - `[dependencies]` MUST contain exactly: `paladin-core`, `paladin-ports`, `tokio`, `async-trait`, `serde`, `serde_json`, `uuid`, `log`, `futures`, `chrono`, `rand`, `tokio-util`, `petgraph`, `regex` — no others unless a gap is discovered and documented
  - `[dependencies]` MUST NOT contain, even transitively: `reqwest`, `actix-web`, `actix-http`, `sqlx`, `redis`, `qdrant-client`, `lettre`, `aws-sdk-*`, `minio`
  - `cargo build -p paladin-battalion` MUST succeed in isolation once populated
  - A project depending only on `paladin-core` + `paladin-ports` + `paladin-battalion` MUST compile with `cargo tree` showing none of `reqwest`, `sqlx`, `redis`, `qdrant-client`, `actix-web`, `lettre`
- scope: paladin-battalion scaffold, exact dependency set, forbidden crates

## REQ-battalion-service-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-6, FR-7, §1)
- description: Relocate all 13 files (~11,180 LOC) from `src/application/use_cases/battalion/`.
- acceptance:
  - The nine execution services MUST move to `crates/paladin-battalion/src/`: `formation_service.rs`, `phalanx_service.rs`, `campaign_service.rs`, `chain_of_command_service.rs`, `conclave_execution_service.rs`, `council_service.rs`, `grove_service.rs`, `maneuver_service.rs`, `commander.rs`
  - The three support utilities MUST also move: `error_aggregation.rs`, `flow_visualizer.rs`, `retry.rs`
  - Only `src/application/use_cases/battalion/` is in scope; `content`, `herald`, `paladin`, `sanctum` and `arsenal` use-case directories MUST NOT be touched
- scope: 13 battalion files, execution services, support utilities

## REQ-battalion-import-migration
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-8 to FR-12, Story 5, §7.2, §7.3, §7.4, §9 OQ1, OQ2)
- description: Rewrite the ~169 import paths inside the extracted battalion files.
- acceptance:
  - Every `crate::application::ports::` MUST become `paladin_ports::`; every `crate::core::` MUST become `paladin_core::`; every `crate::application::use_cases::battalion::` MUST become `crate::`
  - No extracted file may contain a `use` statement referencing `application::`, `infrastructure::`, or any path outside `paladin-battalion`, `paladin-core` or `paladin-ports`
  - A scripted `sed` pass covering the three substitutions is the recommended approach, followed by `cargo build -p paladin-battalion` to catch residual misses; each manual fix MUST be documented
  - `PaladinError`'s location MUST be resolved before `phalanx_service.rs` is extracted — it currently imports from the root crate's use-cases layer. If Epics 1-2 relocated it, update the import; otherwise a decision (move to `paladin-core` or re-export through `paladin-ports`) MUST be made and recorded in Task 1.0 before any code moves
  - A `grep -rn "#\[cfg(feature" src/application/use_cases/battalion/` audit MUST be run before extraction; any feature flags found MUST be mirrored in `paladin-battalion/Cargo.toml`
- scope: 169 import occurrences, sed migration, PaladinError location, feature flag audit

## REQ-battalion-inline-tests
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-13, §5 Non-Goals)
- description: Preserve every inline test module through the move.
- acceptance:
  - All inline `#[cfg(test)]` modules in the extracted files MUST compile and pass via `cargo test -p paladin-battalion`
  - There are 12 such test modules across the service files; none may be dropped or disabled
  - Existing tests MUST be migrated verbatim; no test rewrites are required (new doc-tests in `lib.rs` are welcome)
- scope: inline test modules, verbatim migration

## REQ-battalion-facade-shim
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-14 to FR-18, Story 3, §6 facade strategy, §7.5)
- description: Convert `src/application/use_cases/battalion/mod.rs` into a re-export shim.
- acceptance:
  - Root `Cargo.toml` MUST add `paladin-battalion = { workspace = true }`
  - `mod.rs` MUST be converted to re-export all public items from `paladin_battalion` so existing paths such as `crate::application::use_cases::battalion::formation_service::*` continue to resolve; the shim MUST preserve the sub-module path structure (one `pub use paladin_battalion::<module>;` per module)
  - The original 12 source files MUST be deleted from `src/application/use_cases/battalion/` only after the shim is in place and `cargo test --workspace` passes; only `mod.rs` remains
  - `cargo build --workspace` MUST succeed with zero errors and zero warnings; `cargo test --workspace` MUST pass at or above the pre-epic baseline of 2,610 tests
  - The originals MUST NOT be deleted until all 13 files compile in the new crate, the shim is in place, and the workspace test suite passes — preserving a rollback path
  - The invariant is zero broken import paths, not the exact re-export mechanism
- scope: facade re-export shim, safe migration order, 2,610-test baseline

## REQ-battalion-dependency-validation
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-19 to FR-21, SM-3, SM-9, SM-10, Story 2)
- description: Prove and record `paladin-battalion`'s dependency isolation.
- acceptance:
  - `cargo tree -p paladin-battalion` MUST be run once the crate is fully populated and inspected to confirm no forbidden infrastructure crate appears
  - The `cargo tree` output MUST be saved to `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-dependency-tree.txt`
  - The `cargo build -p paladin-battalion` output (stdout + stderr) MUST be saved to `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-isolation-build.txt`
  - A change to `grove_service.rs` MUST NOT trigger recompilation of any infrastructure crate
- scope: cargo tree evidence, isolation build evidence

## REQ-battalion-example-verification
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-22, FR-23, SM-8, §9 OQ3, §5 Non-Goals)
- description: Verify examples and workspace-level battalion tests still compile against the shim.
- acceptance:
  - These examples MUST compile via `cargo check --example <name>`: `formation_sequential`, `campaign_workflow`, `chain_of_command_delegation`, `commander_basic`, `commander_auto`, `commander_full_config`
  - `tests/unit/battalion/` (6 files) and `tests/integration/battalion/` (7 files listed) MUST pass, plus `tests/battalion_campaign_integration_test.rs` and `tests/battalion_chain_of_command_integration_test.rs`
  - Workspace-level test directories MUST remain in place; migrating them into `crates/paladin-battalion/tests/` is deferred to Epic 6. Preferred resolution if re-export path resolution differs: keep tests at workspace level and fix import paths
- scope: example spot-checks, workspace battalion tests

## REQ-battalion-crate-docs
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_3/prd-paladin-battalion-extraction.md (FR-24, FR-25, §5 Non-Goals, §9 OQ4)
- description: Crate-level documentation and quality gates for `paladin-battalion`.
- acceptance:
  - `crates/paladin-battalion/src/lib.rs` MUST include a crate-level `//!` doc comment describing the crate's purpose, listing the eight orchestration patterns, and providing a minimal usage example
  - `cargo doc -p paladin-battalion --no-deps` MUST produce zero errors and zero warnings
  - `cargo clippy --workspace -- -D warnings` and `cargo fmt --all --check` MUST both exit 0
  - No `paladin::prelude` module is added here — deferred to Epic 6
  - No behavioural or API-shape changes; bugs discovered during migration MUST be tracked separately rather than fixed in this epic
  - `petgraph` version alignment between `paladin-core` and `paladin-battalion` MUST be confirmed to avoid duplicate compilation
- scope: crate docs, quality gates, scope discipline

## REQ-llm-crate-scaffold
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-1 to FR-7, §6.2, Story 1, §7.5)
- description: Scaffold `crates/paladin-llm/` with per-provider feature flags and optional `reqwest`.
- acceptance:
  - `crates/paladin-llm/` MUST exist with a valid `Cargo.toml` and `src/lib.rs`, `name = "paladin-llm"`, using workspace dependency syntax
  - `[features]` MUST define `openai`, `anthropic`, `deepseek`, `mock` and `vision`, with `default = ["openai", "mock"]`
  - `reqwest` (with TLS features `json`, `rustls-tls`) MUST be optional, activated only by `openai`, `anthropic` or `deepseek`, and MUST NOT compile with `--no-default-features`
  - `cargo build -p paladin-llm --no-default-features` MUST succeed and produce a crate with no provider code (an empty public surface is acceptable)
  - `paladin-llm` MUST depend on `paladin-core` and `paladin-ports` as non-optional workspace dependencies
  - `paladin-llm` MUST NOT depend on the root `paladin` crate, `paladin-battalion`, `paladin-memory`, or any infrastructure crate other than the adapters moved into it
  - The workspace root `members` list MUST include `"crates/paladin-llm"`
- scope: paladin-llm scaffold, feature flags, optional reqwest

## REQ-llm-provider-error
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-8 to FR-10, §6.3)
- description: A `LlmProviderError` enum that converts to the shared `LlmError` at the public boundary.
- acceptance:
  - `LlmProviderError` MUST be defined in `crates/paladin-llm/src/error.rs` and re-exported as `paladin_llm::LlmProviderError`, deriving `thiserror::Error` and `Debug`
  - Variants MUST include at minimum: `OpenAI(String)`, `Anthropic(String)`, `DeepSeek(String)`, `Configuration(String)`, `Network(String)`, `RateLimit`, `Timeout(u64)`, `TokenLimitExceeded { limit: usize, requested: usize }`, `Authentication(String)`, `Serialization(String)`
  - `From<LlmProviderError> for LlmError` MUST be implemented, where `LlmError` is `paladin_ports::output::llm_port::LlmError`; the conversion is applied in the `impl LlmPort` method bodies
  - Provider-specific HTTP error structs may remain private within each provider module; only `LlmProviderError` is public
  - Conversion chain: private provider HTTP error -> `LlmProviderError` -> `LlmError`
- scope: LlmProviderError, error conversion boundary

## REQ-openai-provider-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-11 to FR-16, §7.2, §7.3, §9 OQ3, OQ4)
- description: Extract the OpenAI adapter family behind the `openai` feature.
- acceptance:
  - `openai_adapter.rs` MUST move to `crates/paladin-llm/src/openai/adapter.rs` and `openai_embedding_adapter.rs` to `crates/paladin-llm/src/openai/embedding.rs`
  - `openai_vision.rs` MUST move to `crates/paladin-llm/src/openai/vision.rs` gated behind `#[cfg(all(feature = "openai", feature = "vision"))]`
  - `OpenAIConfig` MUST remain a public struct with fields `api_key: String`, `base_url: String`, `organization: Option<String>`, `timeout_seconds: u64`, `max_retries: u32`, and MUST retain its `from_env()` constructor
  - `use crate::core::platform::container::content::{ContentItem, ContentType}` and `…::prompt::{PromptItem, PromptType}` MUST become `paladin_core::…`; the implementer MUST grep for `crate::core::` in each moved file
  - `cargo build -p paladin-llm --features openai` MUST succeed; `--no-default-features` MUST NOT include `OpenAIAdapter`
  - All OpenAI adapter unit tests MUST move with the source and pass under `cargo test -p paladin-llm --features openai`
  - `rand` (used by `openai_adapter.rs` for retry-backoff jitter) MUST be added as an optional dependency under the `openai` feature or extracted into a shared internal utility
  - The embedding adapter is consolidated under `openai`; the monolith's separate `openai-embeddings` flag MUST be confirmed unused downstream before merging the two
- scope: OpenAI adapter, embedding adapter, vision adapter, OpenAIConfig, rand

## REQ-anthropic-provider-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-17 to FR-20)
- description: Extract the Anthropic adapter behind the `anthropic` feature.
- acceptance:
  - `anthropic_adapter.rs` MUST move to `crates/paladin-llm/src/anthropic/adapter.rs`
  - `anthropic_vision.rs` MUST move to `crates/paladin-llm/src/anthropic/vision.rs` gated behind `#[cfg(all(feature = "anthropic", feature = "vision"))]`
  - `AnthropicConfig` MUST be public with at minimum `api_key: String`, `base_url: String`, `timeout_seconds: u64`, `max_retries: u32`, retaining `from_env()`
  - `cargo build -p paladin-llm --features anthropic` MUST succeed independently of the `openai` and `deepseek` flags
  - All Anthropic adapter unit tests MUST move and pass under `cargo test -p paladin-llm --features anthropic`
- scope: Anthropic adapter, Anthropic vision, AnthropicConfig

## REQ-deepseek-provider-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-21 to FR-24, Story 3)
- description: Extract the DeepSeek adapter behind the `deepseek` feature.
- acceptance:
  - `deepseek_adapter.rs` MUST move to `crates/paladin-llm/src/deepseek/adapter.rs` (or `src/deepseek.rs` if it has no sub-modules)
  - `DeepSeekConfig` MUST be public with at minimum `api_key: String`, `base_url: String`, `timeout_seconds: u64`, `max_retries: u32`, retaining `from_env()`
  - `cargo build -p paladin-llm --features deepseek` MUST succeed independently of the `openai` and `anthropic` flags
  - `cargo test -p paladin-llm --features deepseek` MUST run in isolation; DeepSeek-only changes MUST NOT trigger recompilation of `paladin-battalion`, `paladin-core` or the root facade
- scope: DeepSeek adapter, DeepSeekConfig, isolated rebuild

## REQ-llm-mock-adapters
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-25 to FR-27, Story 2, §9 OQ3)
- description: Extract the mock LLM adapters behind a default-on `mock` feature.
- acceptance:
  - `mock_llm_adapter.rs` MUST move to `crates/paladin-llm/src/mock.rs`
  - `MockLlmPort` and `MultiStepMockLlmPort` MUST be re-exported at `paladin_llm::mock::MockLlmPort` and `paladin_llm::mock::MultiStepMockLlmPort`
  - `mock` MUST be enabled by default and MUST compile with no network dependencies — `reqwest` MUST NOT be required when only `mock` is enabled
  - Mock adapter tests MUST compile and pass under `cargo test -p paladin-llm --features mock` with no other feature required
  - `MultiStepMockLlmPort` was not found in `mock_llm_adapter.rs` at spec-authoring time; the implementer MUST grep the workspace to locate it or confirm it is a new type to be created
- scope: MockLlmPort, MultiStepMockLlmPort, mock feature

## REQ-llm-provider-factory
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-28 to FR-30, §9 OQ5)
- description: Co-locate the provider factory with the adapters.
- acceptance:
  - `provider_factory.rs` MUST move to `crates/paladin-llm/src/provider_factory.rs`; `LlmProviderFactory` and `ProviderFactoryError` MUST be re-exported at the crate root
  - `LlmProviderFactory::create()` MUST remain feature-gated internally: it returns an error for providers whose feature is disabled rather than failing to compile, so the factory works in partial-feature builds
  - `ProviderFactoryError` is private to `paladin-llm` and MUST implement `From<ProviderFactoryError> for LlmProviderError`
  - An audit MUST confirm whether the current factory reads `ApplicationSettings` directly; if so that import MUST be removed and replaced with the bridge pattern before the file moves
- scope: LlmProviderFactory, ProviderFactoryError, partial-feature builds

## REQ-llm-config-bridge-location-v1
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-31 to FR-33, §6.4, §6.5)
- description: Configuration bridging lives entirely in the root crate; `paladin-llm` owns only its own `*Config` structs.
- acceptance:
  - `paladin-llm` MUST NOT import from `crate::config::application_settings` or any equivalent root-crate path — doing so would create a circular dependency
  - Each provider's `*Config` struct is the configuration boundary; the root `paladin` crate is **solely responsible** for reading `ApplicationSettings.llm.*` and converting into the `paladin-llm` `*Config` structs
  - The conversion code MUST live in the root crate at `src/infrastructure/adapters/llm/config_bridge.rs`, implementing `From<&LlmProviderConfig> for OpenAIConfig` and equivalents, and MUST be invisible to `paladin-llm`
  - The same bridge pattern MUST apply to `VisionConfig` -> `OpenAIVisionConfig` (and the Anthropic equivalent); the vision adapters MUST have their direct `application_settings` import removed
  - Every `*Config` MUST retain `from_env()` so `paladin-llm` stays usable without `ApplicationSettings`
  - Introducing a `paladin-config` crate is an explicit non-goal
- scope: config bridge, ApplicationSettings coupling, from_env
- note: COMPETING with REQ-llm-config-bridge-location-v2. Shipped code follows v2 — see code-verification.md.

## REQ-llm-config-bridge-location-v2
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.1 paladin-llm table, §7 config_bridge.rs, §7 Cross-Crate Config Re-exports)
- description: `paladin-llm` owns its own configuration module, including `LlmProviderConfig`, `LlmConfig` and the vision configs.
- acceptance:
  - `crates/paladin-llm/src/config/mod.rs`, `config/llm.rs` and `config/vision.rs` MUST be created and exposed via `paladin-llm`'s `lib.rs` as `pub mod config;`
  - `config/llm.rs` MUST hold `LlmProviderConfig`, `LlmConfig` and their `impl` blocks including `get_provider_config()` and `get_default_provider_name()`
  - `config/vision.rs` MUST hold `VisionRetryConfig`, `VisionProviderConfig`, `VisionConfig` and their `impl` blocks
  - `src/infrastructure/adapters/llm/config_bridge.rs` imports MUST be updated after the move and its tests MUST still pass
  - Crate dependencies still flow inward only: `paladin-llm` may import `paladin-core` and `paladin-ports`; the facade may import all sub-crates
- scope: paladin-llm config module, LlmConfig, VisionConfig, config_bridge
- note: COMPETING with REQ-llm-config-bridge-location-v1, which forbids `paladin-llm` from owning `ApplicationSettings`-derived config. This is the later position (2026-05-23 versus 2026-05-18). Shipped code has `crates/paladin-llm/src/config/bridge.rs`.

## REQ-llm-test-architecture
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-34 to FR-36, §6.1 tests layout)
- description: Split unit and integration tests for `paladin-llm`.
- acceptance:
  - Unit tests (individual methods, error conversions) MUST live co-located in `#[cfg(test)]` modules inside `crates/paladin-llm/src/**/*.rs`
  - Integration tests (full `LlmPort` roundtrips, factory creation, feature-flag matrix) MUST live in `crates/paladin-llm/tests/`, each file gated with the `#[cfg(feature = "...")]` appropriate to the providers it tests: `openai_integration.rs`, `anthropic_integration.rs`, `deepseek_integration.rs`, `mock_integration.rs`, `provider_factory_test.rs`
  - No integration test in `crates/paladin-llm/tests/` may make real network calls; all MUST use the mock adapter or a test double
  - Tests requiring a live API key MUST be annotated `#[ignore]` and documented
- scope: paladin-llm test layout, network isolation, ignored live tests

## REQ-llm-facade-prelude
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-37 to FR-40, §7.4, §7.6)
- description: Wire `paladin-llm` into the facade through the prelude and retire the old deep paths.
- acceptance:
  - The root `Cargo.toml` MUST add `paladin-llm` with `default-features = false` and a features list mirroring the root crate's own LLM flags (e.g. `["openai", "anthropic", "deepseek", "mock"]`)
  - `paladin::prelude` MUST include at minimum `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter`, `MockLlmPort`, `MultiStepMockLlmPort`, `LlmProviderFactory`, `LlmProviderError`
  - Old deep import paths (e.g. `paladin::infrastructure::adapters::llm::openai_adapter::OpenAIAdapter`) are explicitly **not** required to be preserved; new consumers use `paladin::prelude::OpenAIAdapter`
  - A grep-and-update sweep over all workspace examples and integration tests MUST fix broken import paths; `grep -r "infrastructure::adapters::llm" --include="*.rs" .` MUST be run and every match updated
  - The original source files under `src/infrastructure/adapters/llm/` MUST be removed — not left as dead code or re-export shims; the module should be removed entirely or reduced to a comment explaining the move
- scope: facade wiring, paladin::prelude, deprecated deep paths

## REQ-llm-build-validation
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md (FR-41 to FR-49, §8 Success Metrics, §5 Non-Goals)
- description: Build, test and quality gates across the `paladin-llm` feature matrix.
- acceptance:
  - `cargo build -p paladin-llm` MUST succeed for each of `--no-default-features`, `--features openai`, `--features anthropic`, `--features deepseek`, `--features mock` and `--all-features`
  - `--no-default-features` MUST succeed in under 5 seconds; `cargo tree -p paladin-llm --features openai` MUST show no Anthropic or DeepSeek deps; zero bytes of dead provider code for a single-provider consumer
  - `cargo test --workspace` MUST pass with all 1,487+ existing tests green; `cargo clippy -p paladin-llm --all-features -- -D warnings` MUST produce zero warnings; `cargo fmt --check -p paladin-llm` MUST pass
  - `cargo doc -p paladin-llm --all-features --no-deps` MUST have zero broken intra-doc links and zero warnings
  - Incremental rebuild for an OpenAI-only change MUST be measurably less than a full workspace rebuild (target >= 50% reduction)
  - Explicit non-goals: no new providers, no `LlmPort` signature change, no per-provider crates, no `paladin-config` crate, no streaming/SSE architecture change, no retry/backoff logic change, no memory or storage adapters, no CI updates
- scope: feature-matrix builds, quality gates, scope exclusions

## REQ-memory-crate-scaffold
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-1, §7 sqlx/qdrant version management, §7 deny unsafe, Story 1)
- description: Scaffold `crates/paladin-memory/` with feature-gated heavy dependencies.
- acceptance:
  - `crates/paladin-memory/` MUST exist with a valid `Cargo.toml` and `src/lib.rs`
  - `[features]` MUST be exactly `default = []`, `sqlite = ["dep:sqlx"]`, `qdrant = ["dep:qdrant-client"]`, `content-processing = ["dep:tiktoken-rs"]`
  - Mandatory dependencies MUST be `paladin-core`, `paladin-ports`, `async-trait`, `serde`, `serde_json`, `uuid`, `chrono`, `thiserror`, `tokio`, `futures`, `log`
  - `sqlx` and `qdrant-client` MUST be hoisted into the root `[workspace.dependencies]` before any source is written (prerequisite of Task 5.1); the workspace `sqlx` entry MUST declare only `runtime-tokio-rustls`, `sqlite`, `chrono`, `uuid`, `json` and MUST NOT include `mysql` (a root-crate-only override); `qdrant-client` MUST be pinned at `1.14`
  - The exact `tiktoken-rs` version MUST be confirmed via `cargo tree -p paladin | grep tiktoken` before writing the manifest
  - `[lib]` MUST set `doctest = false`; `src/lib.rs` MUST add `#![deny(unsafe_code)]`
  - The root workspace `Cargo.toml` MUST add `paladin-memory = { path = "crates/paladin-memory" }` to `[workspace.dependencies]` and to the facade's `[dependencies]` with the appropriate features forwarded
  - `cargo build -p paladin-memory --no-default-features` MUST succeed and `cargo tree` MUST show no `sqlx`, `qdrant-client` or `tiktoken-rs`
- scope: paladin-memory scaffold, feature flags, workspace dependency hoisting

## REQ-memory-module-structure
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-2, Story 4, Story 6)
- description: Three-module structure plus a prelude for `paladin-memory`.
- acceptance:
  - `src/lib.rs` MUST declare `pub mod garrison; pub mod sanctum; pub mod services; pub mod prelude;`
  - `garrison/mod.rs` MUST unconditionally re-export `InMemoryGarrison`; behind `#[cfg(feature = "sqlite")]` re-export `SqliteGarrison`; behind `#[cfg(feature = "content-processing")]` re-export `TokenCounter`, `TiktokenCounter` and `TokenCounterFactory`
  - `sanctum/mod.rs` MUST unconditionally re-export `InMemorySanctum` and, behind `#[cfg(feature = "qdrant")]`, `QdrantSanctumAdapter`
  - `services/mod.rs` MUST unconditionally re-export `MemoryExtractionService`, `ExtractedMemory`, `MemoryExtractionStrategy`, `RagRetrievalService`, `RagConfig`, `RetrievalTrigger` and `retrieve_context_with_timeout`
  - `prelude.rs` MUST unconditionally re-export `InMemoryGarrison`, `InMemorySanctum`, `MemoryExtractionService`, `RagRetrievalService` and `RagConfig`
- scope: paladin-memory module tree, prelude

## REQ-garrison-adapter-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-3, Story 2, Story 6, §7 SQLite migrations, §7 thread safety)
- description: Extract the Garrison adapters, promoting hidden types to documented public API.
- acceptance:
  - `garrison/in_memory_garrison.rs` MUST contain the full `InMemoryGarrison` implementing `GarrisonPort` via `async-trait`, compiled unconditionally
  - `garrison/sqlite_garrison.rs` MUST sit inside a `#[cfg(feature = "sqlite")]` module boundary and implement `GarrisonPort` using `sqlx::SqlitePool`; the `#[doc(hidden)]` on `SqliteGarrison` MUST be removed and replaced with rustdoc covering purpose, construction and usage
  - `garrison/token_counter.rs` MUST sit inside `#[cfg(feature = "content-processing")]` and contain the `TokenCounter` trait, `TiktokenCounter` struct and `TokenCounterFactory`; `#[doc(hidden)]` on `TiktokenCounter` MUST be removed and rustdoc added to it, `TokenCounterFactory` and all public methods
  - `crate::core::platform::container::garrison::*` imports MUST become `paladin_core::platform::container::garrison::*`; `paladin_ports::output::garrison_port::*` imports MUST remain unchanged
  - `SqliteGarrison`'s inline SQL migrations travel with the file; no separate migration files are required
  - All adapter types MUST remain `Send + Sync`, verified with a compile-time assertion helper in tests
- scope: InMemoryGarrison, SqliteGarrison, TokenCounter family, doc(hidden) removal

## REQ-sanctum-adapter-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-4, Story 3)
- description: Extract the Sanctum adapters and promote `InMemorySanctumConfig` to public API.
- acceptance:
  - `sanctum/in_memory_adapter.rs` MUST contain the full `InMemorySanctum` including `InMemorySanctumConfig`, implementing `SanctumPort` via `async-trait`
  - `InMemorySanctumConfig` MUST become a fully public documented type: any `#[doc(hidden)]` removed and rustdoc added explaining its fields and defaults; both it and `InMemorySanctum` MUST appear in `paladin-memory`'s public API surface
  - `sanctum/qdrant_adapter.rs` MUST sit inside a `#[cfg(feature = "qdrant")]` module boundary containing `QdrantSanctumAdapter` implementing `SanctumPort`, and MUST NOT be present in the binary or object files when the feature is absent
  - `crate::core::platform::container::sanctum::*` imports MUST become `paladin_core::platform::container::sanctum::*`
- scope: InMemorySanctum, InMemorySanctumConfig, QdrantSanctumAdapter

## REQ-memory-services-extraction
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-5, Story 4, §7 dependency on paladin-llm)
- description: Extract the memory services unchanged and keep them backend-agnostic.
- acceptance:
  - `services/memory_extraction_service.rs` MUST contain `MemoryExtractionService`, `ExtractedMemory` and `MemoryExtractionStrategy` verbatim from the monolith, logic unchanged, with only import paths updated
  - `services/rag_retrieval_service.rs` MUST contain `RagRetrievalService`, `RagConfig`, `RetrievalTrigger` and `retrieve_context_with_timeout` verbatim, logic unchanged
  - Both services MUST depend only on port traits (`GarrisonPort`, `SanctumPort`, `EmbeddingPort`, `LlmPort`) and `paladin-core` domain types, and MUST NOT reference any concrete adapter
  - No dependency on `paladin-llm` may be introduced — `MemoryExtractionService` imports `LlmPort` and `LlmRequest` from `paladin-ports`, which is correct hexagonal architecture
  - Both MUST compile with `--no-default-features` and accept `Arc<dyn SanctumPort>` / `Arc<dyn EmbeddingPort>` at construction
- scope: MemoryExtractionService, RagRetrievalService, backend agnosticism

## REQ-memory-originals-deletion
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-6)
- description: Delete the original memory sources from the monolith after extraction.
- acceptance:
  - After successful extraction and test passage, these MUST be deleted: `src/infrastructure/adapters/garrison/{in_memory_garrison,sqlite_garrison,token_counter,mod}.rs`, `src/infrastructure/adapters/sanctum/{in_memory_adapter,qdrant_adapter,mod}.rs`, `src/application/use_cases/sanctum/{memory_extraction_service,rag_retrieval_service,mod}.rs`
  - `pub mod garrison;` / `pub mod sanctum;` declarations in `src/infrastructure/adapters/mod.rs` and `pub mod sanctum;` in `src/application/use_cases/mod.rs` MUST be removed
- scope: monolith source deletion, mod.rs declarations

## REQ-memory-facade-reexports
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-7, Story 5)
- description: Preserve every existing `use paladin::…` memory import path via facade re-exports.
- acceptance:
  - `src/lib.rs` MUST add `pub use paladin_memory::…` statements covering: `infrastructure::adapters::garrison::{InMemoryGarrison, SqliteGarrison}`, `garrison::token_counter::{TokenCounter, TiktokenCounter}`, `infrastructure::adapters::sanctum::{InMemorySanctum, QdrantSanctumAdapter}`, and `application::use_cases::sanctum::{MemoryExtractionService, RagRetrievalService, RagConfig, MemoryExtractionStrategy, RetrievalTrigger}`
  - No breakage to examples, integration tests or functional tests
  - If a path is discovered during audit to be unused in examples, tests or documented code, the re-export may be omitted with a comment explaining why
  - All garrison and sanctum examples in `examples/` MUST compile and pass a smoke run without changes to their import statements
- scope: facade re-exports, backward-compatible import paths

## REQ-memory-test-migration
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-8, FR-8 addendum)
- description: Move memory unit tests inline and keep integration tests at workspace level.
- acceptance:
  - `tests/unit/sanctum/memory_extraction_service_test.rs`, `rag_retrieval_service_test.rs` and `qdrant_sanctum_test.rs` MUST become inline `#[cfg(test)]` modules in their corresponding `crates/paladin-memory/src/` files (the Qdrant one gated on `#[cfg(feature = "qdrant")]`)
  - `tests/unit/sanctum_domain_tests.rs` and `tests/unit/sanctum_port_tests.rs` MUST stay at workspace level — they test `paladin-core` domain types and `paladin-ports` traits, not memory adapters
  - These MUST remain at workspace level in `tests/integration/`: `in_memory_sanctum_tests.rs`, `sqlite_garrison_integration_test.rs`, `qdrant_sanctum_tests.rs`, `rag_integration_tests.rs`, `paladin_garrison_integration_test.rs`
  - Before closing Task 5.2, existing `TokenCounterFactory` tests MUST be searched for; if none exist, tests covering factory construction, successful counter creation for a known model, and error handling for an unknown model MUST be written inline in `token_counter.rs`
  - Migrated unit tests MUST pass under `cargo test -p paladin-memory`; integration tests MUST pass under `cargo test --workspace`
- scope: unit test inlining, integration test placement, TokenCounterFactory tests

## REQ-memory-build-gates
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_5/prd-paladin-memory-extraction.md (FR-9, §8 Success Metrics, §5 Non-Goals, §7 content-processing semantics)
- description: Build and quality gates for `paladin-memory`.
- acceptance:
  - All of these MUST succeed without errors or warnings: `cargo build -p paladin-memory` with `--no-default-features`, `--features sqlite`, `--features qdrant`, `--features content-processing` and `--all-features`; `cargo build --workspace`; `cargo test --workspace`; `cargo clippy -p paladin-memory -- -D warnings`; `cargo doc -p paladin-memory --no-deps`
  - In `paladin-memory`, `content-processing` gates **only** `tiktoken-rs` and `token_counter.rs`; it does not gate `pdf-extract`, `scraper` or `rss`, which stay in the root crate. The root crate's aggregate `content-processing` flag is unchanged
  - Explicit non-goals: no new storage backends, no adapter API changes, no `GarrisonPort`/`SanctumPort` trait changes, no domain type changes, no per-adapter sub-crates, no SQLite schema changes, no CLI tooling, no documentation overhaul
  - `crates/paladin-llm` (Epic 4) is the canonical template for crate structure, `Cargo.toml` layout, `lib.rs` gating conventions and rustdoc style
- scope: build gates, content-processing semantics, scope exclusions

## REQ-facade-reexport-audit
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-1.1 to FR-1.3, G1, US-1, SM-1)
- description: Audit and complete the facade crate's re-export coverage.
- acceptance:
  - Every `use paladin::…` import path present in `examples/**/*.rs`, `tests/**/*.rs` and `src/**/*.rs` MUST compile without modification
  - A script or manual audit MUST scan `examples/**/*.rs` and `tests/**/*.rs` for `use paladin::` statements and produce a checklist confirming each path is covered by a re-export in `src/lib.rs`
  - Any uncovered path found during the audit MUST be added
  - `paladin-core` and `paladin-ports` MUST be added to `[workspace.dependencies]` (only `paladin-battalion`, `paladin-llm` and `paladin-memory` are listed there today)
- scope: facade re-export audit, workspace.dependencies completion

## REQ-paladin-prelude
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-1.4 to FR-1.7, G2, US-2, SM-2)
- description: A `paladin::prelude` module carrying the ~20 most commonly used types.
- acceptance:
  - `src/lib.rs` MUST provide `pub mod prelude` (recommended: a dedicated `src/prelude.rs` re-exported as `pub mod prelude`)
  - It MUST contain at minimum: `PaladinBuilder`, `Paladin`, `PaladinData`, `PaladinStatus`, `PaladinConfig`, `PaladinError`; `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`; `BattalionConfig`, `BattalionError`, `BattalionResult`; `CommanderBuilder`, `CouncilBuilder`, `GroveBuilder`; `LlmPort`, `LlmRequest`, `LlmResponse`, `LlmError`; `GarrisonPort`, `GarrisonError`; `SanctumPort`, `SanctumError`; `InMemoryGarrison`, `InMemorySanctum`; `ArsenalPort`, `ArsenalRegistry`, `Armament`; `PaladinResult`, `StopReason`
  - The prelude MUST have a short `//!` module doc comment explaining its contents and usage
  - `use paladin::prelude::*` MUST let a new developer write a working agent in ten lines
  - `cargo build --workspace` MUST succeed with zero errors or warnings after all facade changes
  - `cargo doc -p paladin --no-deps 2>&1 | grep -i "warn\|error"` MUST produce no output
- scope: paladin::prelude, prelude contents, doc cleanliness

## REQ-devcontainer-gh-cli
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-2.0, §7, SM-0)
- description: Install the GitHub CLI in the devcontainer as a prerequisite for CI work.
- acceptance:
  - `gh` is confirmed absent from the container (`which gh` returns nothing) and MUST be installed by adding the official GitHub CLI apt repository and `apt-get install -y --no-install-recommends gh` in `.devcontainer/Dockerfile.dev`, using the upstream Debian keyring method
  - The block MUST be placed after the existing `apt-get` package installation block and before the `rustup` component installation, so it shares the same Docker layer cache
  - After rebuild, `gh --version` MUST succeed inside the container
  - Developers MUST run `gh auth login` once after the rebuild before using `gh workflow run`, `gh run watch` or `gh run list`
  - This is a prerequisite for Task 6.2 and MUST be completed first
- scope: .devcontainer/Dockerfile.dev, gh CLI, CI prerequisite

## REQ-crate-isolation-ci
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-2.1, FR-2.2, FR-2.8, G3, US-3, US-4, SM-3, §6 CI job structure, §7)
- description: A per-crate isolation CI job that catches cross-crate dependency leaks.
- acceptance:
  - A new job named `crate-isolation` MUST be added to `ci.yml`, running in parallel with the existing `test` job and not blocking it, using a matrix strategy in the same style
  - It MUST build and test each crate independently: `paladin-core`, `paladin-ports`, `paladin-battalion` (plain), `paladin-llm` and `paladin-memory` (`--all-features`), and `paladin` (facade)
  - It MUST additionally run `cargo build -p <crate> --no-default-features` for each crate, verifying each compiles cleanly without optional dependencies
  - All 6 crate isolation jobs MUST be green
  - `cargo-nextest` may optionally replace `cargo test` for parallel execution; an optional install step with fallback is recommended
- scope: crate-isolation CI job, per-crate build/test matrix

## REQ-workspace-ci-upgrade
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-2.3 to FR-2.7, FR-2.9, G4, SM-4 to SM-6, §7)
- description: Convert the existing CI workflows to workspace scope.
- acceptance:
  - `ci.yml` workspace-level jobs MUST run against `--workspace` explicitly (`cargo test --workspace`, `cargo build --workspace`) rather than implicitly targeting only the root crate
  - `feature-flags.yml` MUST also run `cargo build --workspace <flags>` in addition to the current root-crate-only default, ensuring flags propagate across all members
  - The lint job MUST run `cargo clippy --workspace -- -D warnings`
  - The documentation check MUST run `cargo doc --workspace --no-deps` and fail the job on any warning (`2>&1 | grep -c "warning:"` must return 0)
  - All existing jobs (`lint`, `api-surface`, `test`, `integration-tests`) MUST remain green; none may be removed or disabled
  - The `test` job matrix (`rust-version: [stable, beta]`) MUST run `cargo test --workspace`
  - `ci.yml`'s deprecated `actions-rs/toolchain@v1` should be upgraded to `dtolnay/rust-toolchain@stable` in the same PR — "a low-risk improvement that should not be deferred"
- scope: ci.yml, feature-flags.yml, workspace-scoped commands, toolchain action

## REQ-build-benchmark-report
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md (FR-3.1 to FR-3.5, G5, US-5, SM-7, SM-8, §6 Benchmark Tooling, §9 OQ-2)
- description: A committed build-time benchmark report comparing workspace to monolith.
- acceptance:
  - Five timed scenarios MUST be recorded with `time cargo build …` or equivalent: workspace clean build; workspace incremental after touching `paladin-core/src/`; after touching `paladin-llm/src/`; after touching `paladin-memory/src/`; and `cargo build -p paladin-battalion` with no memory/llm in the dependency tree
  - Equivalent pre-decomposition times MUST be recorded by checking out the last commit on `main` before the workspace split (recommended: the last commit before `feature/milestone_5-epic_1-paladin-core-extraction` was merged) and running `time cargo build`
  - Results MUST be committed to `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` containing the hardware/OS environment, raw timing numbers and a summary table with percentage improvement per scenario
  - Any incremental regression (workspace slower than monolith for an equivalent change) MUST be called out explicitly with root-cause analysis
  - The report MUST confirm whether the >= 50% incremental rebuild improvement target was achieved and, if not, recommend follow-up actions
  - No scenario may be slower than the pre-decomposition monolith (SM-8)
  - Method: `time` shell built-in, wall-clock `real`, three runs per scenario, report the median, automated by `scripts/benchmark-builds.sh`
- scope: build-benchmarks.md, timing scenarios, >=50% target, benchmark script

## REQ-config-domain-modules
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.1, Goals 1-2, §6 file size)
- description: Replace `application_settings.rs` with per-domain config modules distributed across the workspace crates.
- acceptance:
  - `crates/paladin-memory/src/config/` MUST gain `mod.rs`, `garrison.rs` (`GarrisonSettings`), `sanctum.rs` (`QdrantSanctumConfig`, `SanctumConfig`) and `rag.rs` (`MemoryExtractionConfig` plus the consolidated `RagConfig`)
  - `crates/paladin-llm/src/config/` MUST gain `mod.rs`, `llm.rs` and `vision.rs` (see REQ-llm-config-bridge-location-v2)
  - The facade `src/config/` MUST gain `mod.rs` (root `Settings`, `Settings::new()`, `load_from_file()`, all `get_*_config()` methods and re-exports of sub-crate config types), `arsenal.rs` (`MCPServerConfig`, `ArsenalConfig`), `citadel.rs` (`CitadelConfig`), `file_storage.rs` (`FileStorageConfig`), `herald.rs` (`JsonHeraldConfig`, `MarkdownHeraldConfig`, `TableHeraldConfig`, `HeraldConfig`), `notifications.rs` (`NotificationConfig`), `queue.rs` (`QueueConfig`), `scheduler.rs` (`SchedulerConfig`), `web_server.rs` (`SourceConfig`, `ServerConfig`, `MessageServiceSettings`) and optionally `sanctum_app.rs`
  - No file in any `config/` module may exceed 400 lines; a struct approaching ~350 lines MUST be split further
  - Crate dependencies MUST continue to flow inward only: `paladin-core` must not import from `paladin-memory` or `paladin-llm`
- scope: config/ module layout, per-crate config ownership, 400-line limit
- note: the M6 milestone-overview DOC specifies a different single-directory layout including `agent.rs`, `battalion.rs`, `logging.rs` and `llm.rs` under `src/config/`. PRD wins on precedence; shipped code is a hybrid — see INGEST-CONFLICTS.md INFO and code-verification.md.

## REQ-env-overridable-trait
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.2, Goal 3, §10 OQ1, Task 1.2)
- description: An `EnvOverridable` trait plus `read_env` helper replacing ~30 copies of the env-var override pattern.
- acceptance:
  - `pub trait EnvOverridable { fn apply_env_overrides(&mut self); }` MUST be defined in a shared location — recommended `src/config/env_utils.rs` in the facade crate, since all current env-override logic lives there; `crates/paladin-core/src/config/env_utils.rs` is the alternative if sub-crate config structs need it
  - `pub fn read_env<T: std::str::FromStr>(var_name: &str) -> Option<T>` MUST be provided, returning `Some(value)` when the variable is set and parseable and `None` otherwise
  - Every config struct that currently has env-override logic inside `Settings::get_*_config()` MUST instead implement `EnvOverridable` and move that logic into `apply_env_overrides()`
  - Unit tests MUST cover `read_env::<String>`, `read_env::<u16>`, `read_env::<u64>`, `read_env::<bool>` and `read_env::<Option<_>>` with both set and unset variables
  - `pub mod env_utils;` MUST be added to `src/config/mod.rs`; `cargo test config::env_utils` MUST pass
  - Tests using `#[serial]` (serial_test) to avoid env-var collision MUST keep that attribute
- scope: EnvOverridable trait, read_env helper, env override consolidation

## REQ-settings-root-struct
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.3, Goal 4)
- description: The root `Settings` struct remains the single config entry point with an unchanged public API.
- acceptance:
  - `Settings` MUST stay in `src/config/mod.rs` (facade crate) and compose all sub-configs; all existing fields remain with types updated to the new module paths
  - Its public API MUST NOT change: `Settings::new() -> Result<Self, ConfigError>` and `Settings::load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>>` are preserved
  - `get_queue_config()`, `get_file_storage_config()`, `get_notification_config()`, `get_garrison_config()` and `get_sanctum_config()` MUST remain but now delegate to each sub-struct's `apply_env_overrides()`
- scope: Settings struct, public API stability, get_*_config delegation

## REQ-config-incremental-migration
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.4, Goal 5, Task 1.3, Task 1.4)
- description: A three-step incremental migration keeping the codebase compiling at every step.
- acceptance:
  - Step A per domain: create the new file in the target crate/module, move the struct and its `impl` blocks, then add a temporary `pub use` re-export in `application_settings.rs` so existing consumers keep compiling
  - Step B per file: update all consumers in `src/` to import from the new path
  - Step C cleanup: delete `application_settings.rs` once no re-export is needed, and remove `pub mod application_settings;` from `src/config/mod.rs`
  - Per-domain sequence: create file, copy struct + impls, move env-override logic into `apply_env_overrides()`, replace the struct in `application_settings.rs` with a `pub use`, `cargo build` clean, move the struct's tests, `cargo test` clean, repeat
  - Recommended risk order: HeraldConfig group -> SchedulerConfig -> CitadelConfig -> ArsenalConfig/MCPServerConfig -> QueueConfig -> FileStorageConfig -> NotificationConfig -> web_server trio -> vision configs -> LLM configs -> GarrisonSettings -> Sanctum configs -> RagConfig/MemoryExtractionConfig
  - The 29 known consumer files include `src/application/cli/commands/arsenal.rs`, `src/application/use_cases/paladin/paladin_builder.rs`, `src/infrastructure/repositories/sqlite_user_repository.rs`, `src/infrastructure/adapters/llm/config_bridge.rs`, `src/main.rs`, `src/config/setup/mod.rs`, `src/config/setup/service_runner.rs` and `src/config/user_config.rs`
  - `src/config/user_config.rs` is the highest-risk consumer (15+ references, constructs `Settings` inline with many sub-struct defaults) and MUST be updated last; `UserServiceFactory` MUST still produce correct defaults
  - `grep -r "application_settings" src/` MUST return zero results at the end
- scope: incremental migration, temporary re-exports, 29 consumer files, user_config.rs risk

## REQ-config-yml-backcompat
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.5, §6 serialization attributes, §7 feature flags, §8.6, §10 OQ3)
- description: `config.yml` deserialization must be byte-for-byte compatible after the move.
- acceptance:
  - The deserialization contract MUST NOT change; YAML key names MUST remain identical
  - All `#[serde(default)]` and `#[serde(rename)]` attributes MUST be preserved exactly when moving structs
  - `#[derive(...)]` lines MUST be copied exactly — no derives added or removed; special attention to `#[serde(default)]`, `#[serde(skip_serializing_if = "Option::is_none")]` and `#[cfg(feature = "...")]` gates on structs and fields
  - Feature gates MUST travel with their struct: `FileStorageConfig` behind `s3-storage`, `NotificationConfig` fields behind `notifications`, parts of `LlmConfig` behind provider-specific flags
  - A regression test MUST load the reference `config.test.yml` and assert field values match before and after the migration; whether to use `insta` or plain `assert_eq!` is an open decision
- scope: config.yml schema stability, serde attributes, cfg gates, regression test

## REQ-rag-config-dedup
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§4.6, §8.7, §10 OQ2)
- description: Consolidate the two competing `RagConfig` definitions into one.
- acceptance:
  - `RagConfig` currently exists twice: in `src/config/application_settings.rs` (app-settings struct) and in `crates/paladin-memory/src/services/rag_retrieval_service.rs` (service config)
  - The canonical location MUST be `crates/paladin-memory/src/config/rag.rs`; the `application_settings.rs` version MUST be removed and replaced with a re-export from `paladin-memory`
  - All consumers MUST be updated to the `paladin-memory` version
  - `grep -r "struct RagConfig" crates/` MUST return exactly one result
  - Before deleting the `application_settings` version, the two structs MUST be diffed field-by-field to confirm the `paladin-memory` version covers every field used in config loading
- scope: RagConfig deduplication, canonical location

## REQ-config-success-metrics
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md (§8, §5 Non-Goals, §10 OQ4, Goal 7)
- description: Quality gates and scope boundaries for the config decomposition.
- acceptance:
  - `cargo test --workspace` MUST produce zero failures after each task; all 128+ existing config-related tests MUST keep passing
  - No file may exceed 400 lines, verified by `find crates/*/src/config src/config -name "*.rs" | xargs wc -l`
  - `application_settings.rs` MUST be deleted by the end of Task 1.4
  - `cargo doc --workspace --no-deps` MUST show no new public items beyond what existed before
  - `cargo clippy --workspace -- -D warnings` MUST be clean at the end of each task; `cargo fmt --all -- --check` MUST be clean
  - `serde` MUST be confirmed as an existing dependency of `paladin-memory` before adding config files with serde derives
  - Explicit non-goals: no `config.yml` schema change, no new configuration options, no config-loading performance work, no notification-domain-model change, no further crate extractions, no `STABLE_API.md` change, no CLI change
- scope: config quality gates, 400-line check, scope exclusions

## REQ-orchestration-target-structure
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.1, Goal 7)
- description: Target application-layer directory structure for the four orchestrator modules.
- acceptance:
  - `src/application/use_cases/notification_orchestrator/` MUST contain `mod.rs` (`NotificationOrchestrator`) and `types.rs` (`NotificationServiceError`, `NotificationServiceConfig`, `NotificationServiceStats`, `NotificationDeliveryResult`, `NotificationChannelHandler`, `NotificationTemplateProcessor` — or `paladin-core` if pure value objects)
  - `src/application/use_cases/queue_orchestrator/` MUST contain `mod.rs` (`QueueOrchestrator`) and `types.rs` (`QueueError`, `QueueServiceConfig`, other coordination types not already in `paladin-core`)
  - `src/application/use_cases/orchestration/` MUST contain `mod.rs` (`Orchestrator`), `listener.rs` (`ListenerOrchestrator`), `scheduler.rs` (`SchedulerOrchestrator`) and `types.rs` (`OrchestrationContext` confirm/move, `OrchestratorStats`, `ListenerConfig`, `ListenerStats`, `ScheduledJob`)
  - `src/application/use_cases/log_orchestrator/` MUST contain `mod.rs` (`LogOrchestrator`) and `types.rs` (`LogServiceConfig`, `LogMessageHandler`, other coordination types)
  - The existing `src/application/notifications/` directory (`email_notifications.rs`, `push_notifications.rs`, `system_notifications.rs`) is a separate module of channel-specific adapters and MUST NOT be merged with `notification_orchestrator/`; both coexist
  - No file added by this Epic may exceed 600 lines
- scope: application/use_cases orchestrator directories, 600-line limit
- note: shipped code places these four modules under `src/application/services/` with the same module names; `src/application/use_cases/` no longer exists. See code-verification.md.

## REQ-domain-type-placement-rules
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.2, Goal 2, §7.2, §7.3)
- description: Mechanical rules deciding whether an extracted type belongs in `paladin-core` or the application layer.
- acceptance:
  - Struct with no `Arc<dyn Port>` fields and no async fn in impl -> pure value object -> `paladin-core` container module
  - Enum that is a pure error type with no port references -> domain error -> `paladin-core` container module
  - Struct holding `Arc<dyn SomePort>` or calling port methods -> service coordination type -> application layer `types.rs`
  - Struct requiring `async_trait` to implement -> coordination/service type -> application layer
  - Struct already in `paladin-core/src/platform/container/` -> no move needed
  - Types already confirmed in `paladin-core`: `OrchestrationContext`, `QueueConfig`, `QueueItem`, `LogLevel`/`LogDestination`/`LogMessage`/`LogEntry`, `Notification`/`NotificationChannel`/`NotificationContent`
  - `SchedulerOrchestrator` may still reference the concrete `ContentIndexingService`, `DataBackupService` and `EmailNotificationService` task types because they are `paladin-core` domain types — application -> `paladin-core` is a legal direction
  - `MessageService` (used by both notification and log services) MUST be imported from `paladin-core` via the workspace dependency; `MessageService` itself is unchanged
- scope: type placement rules, paladin-core eligibility, MessageService

## REQ-six-service-relocation
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.3, Goal 1, §6 incremental strategy, §7.1)
- description: Relocate six manager-layer services, not four.
- acceptance:
  - `core/platform/manager/notification_service.rs` -> `application/use_cases/notification_orchestrator/` (imports `paladin_ports`, coordinates delivery via port-backed adapters)
  - `queue_service.rs` -> `queue_orchestrator/` (coordinates between `Orchestrator` and external queue adapters)
  - `orchestrator.rs` -> `orchestration/` (imports `listener_service`, `queue_service`, `scheduler`; complex workflow coordination)
  - `log_service.rs` -> `log_orchestrator/` (imports `paladin_ports::output::log_port`; routes log entries via port adapters)
  - `listener_service.rs` -> `orchestration/listener.rs` (the orchestrator depends on it directly; relocating together prevents cross-layer coupling)
  - `scheduler.rs` -> `orchestration/scheduler.rs` (instantiates concrete `TaskService` implementations; application-layer orchestration, not pure domain scheduling)
  - `orchestrator.rs` imports `QueueService`, `ListenerService` and `Scheduler` directly rather than through traits, so all four MUST move in the same task
  - Each task MUST leave the workspace in a green build state; recommended order: notification -> queue -> log -> orchestrator+listener+scheduler last
  - `log_service` moves before `orchestrator` because it has no dependencies on the other three, reducing the diff size of the most complex task
- scope: six service relocations, sequencing, green-build invariant

## REQ-manager-services-retained
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.4, §4.7, §5 Non-Goals 1-3, §10 OQ1-OQ3)
- description: What stays in `core/platform/manager/` and what is deferred.
- acceptance:
  - `event_manager.rs` remains (confirm in Task 2.1) — appears to be a pure event bus with no port imports
  - `content_service.rs` remains (confirm in Task 2.1) — borderline; evaluate port dependencies
  - `user_service.rs` remains with only an import-path update — it has port dependencies but the relocation scope exceeds this Epic and is flagged for a future Epic
  - `admin/` and `user/` sub-modules remain untouched — all are comment-only stubs with no implementation; they will not be moved, deleted or implemented
  - `src/core/platform/manager/mod.rs` MUST be updated to remove `pub mod` declarations for all relocated services and MUST contain declarations for at most `event_manager`, `content_service`, `user_service`, `admin`, `user`
  - Whether a dedicated Epic should cover the full `user_service` relocation (including `UserServiceFactory`, `user_config.rs`, user CLI commands, user API controller and `SqliteUserRepository`) is an open question
- scope: retained manager services, stub modules, deferred user_service

## REQ-orchestration-consumer-import-updates
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.5, Goal 6, §7.1, §8)
- description: Update the import paths of consumers that do not themselves move.
- acceptance:
  - `src/config/setup/service_runner.rs` MUST switch `NotificationService` -> `application::use_cases::notification_orchestrator::NotificationOrchestrator` and `Scheduler` -> `application::use_cases::orchestration::scheduler::SchedulerOrchestrator`
  - `src/config/user_config.rs` MUST switch `NotificationService` -> `NotificationOrchestrator` and `NotificationServiceConfig` -> `notification_orchestrator::types::NotificationServiceConfig`
  - `src/core/platform/manager/user_service.rs` MUST switch `NotificationService` -> `NotificationOrchestrator` within the same PR/commit as Task 2.3, or its import breaks
  - `src/application/use_cases/content/content_ingestion_service.rs` MUST switch `OrchestrationContext` to `paladin_core::platform::container::orchestration_context::OrchestrationContext`
  - `src/infrastructure/web/user_controller.rs` has no direct notification/orchestrator dependency — verify during Task 2.1
  - `paladin-core/Cargo.toml` MUST be checked for a `paladin-ports` dependency and, if present, it MUST be removed in Task 2.7
- scope: consumer import updates, service_runner, user_config, user_service, content_ingestion_service

## REQ-orchestrator-renaming
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.6)
- description: Rename relocated services so the layer is explicit.
- acceptance:
  - `NotificationService` -> `NotificationOrchestrator`; `QueueService` -> `QueueOrchestrator`; `LogService` -> `LogOrchestrator`; `ListenerService` -> `ListenerOrchestrator`; `Scheduler` -> `SchedulerOrchestrator`
  - `Orchestrator` keeps its name — already descriptive
  - Renaming is optional if the existing name does not conflict with a core domain type; Task 2.1 may revise these recommendations based on actual conflicts found
- scope: service renaming, layer clarity

## REQ-core-isolation-verification
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.7, Goal 3, §9.1, §9.2, §9.7)
- description: Prove `paladin-core` builds with zero port or application dependencies after relocation.
- acceptance:
  - `cargo build -p paladin-core` MUST succeed with zero errors and zero warnings after Task 2.7
  - `cargo tree -p paladin-core` MUST show zero references to `paladin_ports`, `application::` or `infrastructure::`, and no `paladin_ports` edge in the dependency graph
  - `cargo clippy -- -D warnings` MUST produce zero warnings in the modified files
- scope: paladin-core isolation, cargo tree verification

## REQ-orchestration-test-coverage
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§4.8, Goal 5, §9.3, §9.6, §5 Non-Goals 4-8)
- description: Test requirements for the relocation.
- acceptance:
  - Every test that existed before the Epic MUST pass after each individual relocation step; no test may be left failing between tasks
  - For each domain type extracted to `paladin-core` in Task 2.2 (e.g. `QueueStats`, `NotificationServiceStats`), a `#[cfg(test)] mod tests` block MUST exist in the same file covering `Default::default()` validity where applicable, `serde_json` round-trips where `Serialize`/`Deserialize` are derived, and error-variant `Display` formatting where the type is an error enum
  - Existing integration tests in `tests/` referencing relocated services MUST have their import paths updated; no new integration tests are required
  - Explicit non-goals: no new end-to-end notification/queue/log integration suites, no behavioural change to any relocated service, no `config.yml` schema change, no feature-flagging of the six services (all compiled unconditionally)
- scope: test preservation, domain type unit tests, scope exclusions

## REQ-orchestration-no-reexport-shims
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md (§5 Non-Goal 7, §10 OQ4)
- description: No backward-compatibility re-export shims are added for the relocated services.
- acceptance:
  - `pub use` re-exports MUST NOT be added to `src/lib.rs` pointing from old paths to new
  - "Backward compatibility is scoped to compilation — callers will update their import paths"
  - The PRD records this as an open question: whether `src/lib.rs` or `src/prelude.rs` should add old-to-new re-exports "should be confirmed with the team before implementation begins"
- scope: no shim re-exports, migration burden on callers
- note: the M6 milestone-overview DOC (Epic 2 AC 6) requires exactly the opposite — facade re-exports maintaining backward compatibility. See INGEST-CONFLICTS.md WARNINGS.

## REQ-maneuver-submodule-structure
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.1, Goal 3, §9.9)
- description: A unified `maneuver/` sub-module inside `paladin-battalion`.
- acceptance:
  - `paladin-battalion/src/maneuver/` MUST contain `mod.rs` (Maneuver struct, `ManeuverConfig`, re-exports), `parser/{mod.rs, lexer.rs, ast.rs, error.rs}`, `service.rs` and `visualizer.rs`
  - The flat files `maneuver_service.rs` and `flow_visualizer.rs` MUST be replaced by this sub-module
  - All other `paladin-battalion` files remain unchanged: `campaign_service.rs`, `chain_of_command_service.rs`, `commander.rs` (imports updated only), `conclave_execution_service.rs`, `council_service.rs`, `error_aggregation.rs`, `formation_service.rs`, `grove_service.rs`, `in_memory_registry.rs`, `phalanx_service.rs`, `retry.rs`
  - No file added or modified by this Epic may exceed 1,000 lines
- scope: paladin-battalion/src/maneuver, sub-module layout

## REQ-maneuver-files-moved-from-core
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.2, Goals 1-2, §8)
- description: Physically move the parser and Maneuver domain type out of `paladin-core`.
- acceptance:
  - `paladin-core/src/platform/container/battalion/parser/mod.rs` (250 lines) -> `paladin-battalion/src/maneuver/parser/mod.rs`
  - `.../parser/lexer.rs` (269 lines) -> `.../maneuver/parser/lexer.rs`
  - `.../parser/ast.rs` (267 lines) -> `.../maneuver/parser/ast.rs`
  - `.../parser/error.rs` (188 lines) -> `.../maneuver/parser/error.rs`
  - `.../battalion/maneuver.rs` (443 lines) -> `.../maneuver/mod.rs` (combined with new sub-module declarations)
  - Content MUST NOT change — only location and crate context
  - Adding re-exports from `paladin-core` back to `paladin-battalion` is impossible (circular dependency); backward-compatible re-exports live in the facade only
- scope: parser/lexer/ast/error/maneuver.rs relocation

## REQ-maneuver-files-reorganized
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.3, §7.4)
- description: Rename the two existing `paladin-battalion` Maneuver files into the sub-module.
- acceptance:
  - `src/maneuver_service.rs` (984 lines) -> `src/maneuver/service.rs`, with import path updates
  - `src/flow_visualizer.rs` (663 lines) -> `src/maneuver/visualizer.rs`, with import path updates
  - When `maneuver.rs` becomes `maneuver/mod.rs`, Rust treats the module identically; content and `use super::*` references inside `#[cfg(test)]` blocks remain valid beyond cross-crate import updates
- scope: maneuver_service.rs, flow_visualizer.rs renames

## REQ-maneuver-inline-tests
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.4, Goals 6-7, §6 inline test strategy, §7.1, §9.4, §9.5, §11 OQ1)
- description: Inline tests travel with their source; workspace-level test files are untouched.
- acceptance:
  - All Maneuver DSL tests use `#[cfg(test)] mod tests { use super::*; … }` at the bottom of each source file and move automatically with the file; no test code is rewritten or consolidated
  - Confirmed inline counts: `parser/mod.rs` 4, `parser/lexer.rs` 8, `parser/error.rs` 5, `parser/ast.rs` 9, `maneuver.rs` 9, `flow_visualizer.rs` 21, `commander.rs` 26 (not moving); `maneuver_service.rs` count TBD and MUST be re-verified with `grep -c "#\[test\]"` during Task 3.1
  - `tests/unit/parser_tests.rs` (57 tests) and `tests/unit/maneuver_domain_tests.rs` (21 tests) MUST remain unchanged and compile via facade re-exports
  - `cargo test -p paladin-battalion` MUST pass with a total inline count of at least 35 in `src/maneuver/` (4+8+5+9+9 from `paladin-core` plus 21 from `flow_visualizer.rs`)
  - `paladin-core/tests/` MUST be re-confirmed to contain no parser or Maneuver integration test files before proceeding
  - If any test in the original `maneuver.rs` relied on `use super::*` to import types from `paladin-core`'s `battalion/` module, those imports MUST be made explicit
- scope: inline tests, workspace test files, test counts

## REQ-core-maneuver-cleanup
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.5, Goal 5, §7.2, §9.1, §9.6)
- description: Remove every Maneuver/parser reference from `paladin-core`.
- acceptance:
  - `pub mod parser;` and `pub mod maneuver;` MUST be removed from `crates/paladin-core/src/platform/container/battalion/mod.rs`
  - The `parser/` directory and `maneuver.rs` MUST be deleted from `paladin-core`
  - `cargo build -p paladin-core` MUST succeed with zero parser-related code compiled, verified by `cargo build -p paladin-core --message-format=json 2>&1 | grep parser` returning nothing
  - `grep -r "parser\|maneuver" crates/paladin-core/src/platform/container/battalion/` MUST return no file paths
  - The battalion `mod.rs` MUST still compile, retaining `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`, `Conclave`, `Council`, `Grove` and the shared types `BattalionConfig`, `BattalionResult`, `BattalionStrategy` — none of which depend on the parser
- scope: paladin-core cleanup, battalion/mod.rs, retained battalion types

## REQ-maneuver-facade-reexports
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.6, Goal 4, §7.3, §11 OQ2)
- description: Facade re-exports that keep every original Maneuver import path resolving.
- acceptance:
  - `src/core/platform/mod.rs` MUST replace `pub use paladin_core::platform::container;` with an explicit `pub mod container` block
  - A wildcard at the `container` level MUST NOT be used: it would import the `battalion` module name from `paladin-core` and collide with the local `pub mod battalion` declaration (duplicate definition error). Every non-battalion container sub-module MUST be re-exported explicitly, with the complete list confirmed during Task 3.3 by reading `paladin-core/src/platform/container/mod.rs`
  - Inside the local `pub mod battalion`, `pub use paladin_core::platform::container::battalion::*;` IS safe, because by Task 3.3 `paladin-core`'s battalion module no longer exports `parser` or `maneuver`
  - `pub mod parser { pub use paladin_battalion::maneuver::parser::*; }` and `pub mod maneuver { pub use paladin_battalion::maneuver::{AgentResult, ErrorStrategy, ExecutionStatus, Maneuver, ManeuverConfig, ManeuverError, ManeuverResult, OutputFormat}; }` MUST be injected; the type list may be incomplete and Task 3.1 MUST verify every type currently exported from `paladin::core::platform::container::battalion::maneuver` is included (`AgentResult` is one candidate needing addition)
  - No consumer file needs modification: `src/application/cli/commands/{maneuver,battalion}.rs`, `src/application/cli/config/battalion_config.rs`, `tests/unit/parser_tests.rs` and `tests/unit/maneuver_domain_tests.rs` all resolve via these re-exports
  - Task 3.1 MUST confirm no consumer imports `paladin::core::platform::container` as a module and calls methods on it as a module object, which the `pub use` -> `pub mod` change could affect
- scope: facade container re-export block, parser/maneuver forwarding modules

## REQ-maneuver-battalion-import-updates
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.7, Task 3.1 step 5, Task 3.2 step 8, §11 OQ4)
- description: Update import paths inside `paladin-battalion`, including inline fully-qualified paths.
- acceptance:
  - `maneuver/service.rs`: `paladin_core::platform::container::battalion::maneuver::{…}` -> `super::{…}` or `crate::maneuver::{…}`; `…::battalion::parser::FlowExpression` -> `super::parser::FlowExpression`
  - `maneuver/visualizer.rs`: `…::battalion::parser::FlowExpression` -> `super::parser::FlowExpression`
  - `commander.rs`: `…::battalion::maneuver::{Maneuver, ManeuverConfig}` -> `crate::maneuver::{Maneuver, ManeuverConfig}`; `…::battalion::parser::FlowParser` -> `crate::maneuver::parser::FlowParser`
  - `commander.rs` also contains inline fully-qualified path expressions in function bodies (e.g. `paladin_core::platform::container::battalion::maneuver::ErrorStrategy::FailFast` in match arms, `…::Maneuver::new(…)` in constructor calls) that editing `use` blocks alone will not catch. Task 3.1 MUST produce a `grep -n "paladin_core::platform::container::battalion" crates/paladin-battalion/src/commander.rs` list and Task 3.2 MUST address every occurrence
  - `in_memory_registry.rs` MUST be verified during Task 3.1 for imports of `…::battalion::maneuver` or `parser`; if found it joins the update list
- scope: paladin-battalion import updates, commander.rs inline paths

## REQ-maneuver-battalion-lib-exports
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.8, §11 OQ3)
- description: Crate-root re-exports in `paladin-battalion`'s `lib.rs`.
- acceptance:
  - `pub mod maneuver_service;` and `pub mod flow_visualizer;` MUST be removed; `pub mod maneuver;` MUST be added
  - `pub use maneuver::parser::{FlowExpression, FlowParseError, FlowParser};` and `pub use maneuver::{ErrorStrategy, ExecutionStatus, Maneuver, ManeuverConfig, ManeuverError, ManeuverResult, ManeuverExecutionService, OutputFormat};` MUST be added so the facade's forwarding modules resolve
  - Whether `ManeuverExecutionService` needs the crate-root re-export MUST be checked against actual direct consumers
- scope: paladin-battalion lib.rs, crate-root re-exports

## REQ-maneuver-cargo-dependency-check
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_3/prd-co-locate-maneuver-dsl.md (§4.9, §4.10, §9.2, §9.3, §9.6 to §9.8, §5 Non-Goals)
- description: Dependency and quality verification for the Maneuver move.
- acceptance:
  - `paladin-battalion/Cargo.toml`: the parser is dependency-free beyond `serde` and `thiserror`, which `paladin-battalion` already has, so no new dependencies are expected; `paladin-core` remains a dependency for the other battalion types
  - `paladin-core/Cargo.toml`: any dependency needed only by the parser (e.g. `serde` derive for parser types) MUST be removed if no other `paladin-core` type needs it, otherwise left in place
  - `cargo build -p paladin-battalion` and `cargo build --workspace` MUST succeed with zero errors; `cargo test --workspace` MUST pass including the 57 parser and 21 maneuver domain tests with no test-file modification
  - `cargo clippy --workspace -- -D warnings` and `cargo fmt --all -- --check` MUST be clean; `cargo doc -p paladin-battalion --no-deps` MUST document the complete Maneuver subsystem under `paladin_battalion::maneuver`
  - Explicit non-goals: no logic change to the DSL, no new Flow DSL operators, no `config.yml` change, no relocation of the other battalion patterns (their domain types stay in `paladin-core`), no public type renames, no `STABLE_API.md` change, no workspace test-file changes, no relocation of `commander.rs`
- scope: Cargo.toml verification, quality gates, scope exclusions

## REQ-resilience-module-structure
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.1, §4.2, §4.9, §4.10, Goal 2, §6 module visibility, §5 Non-Goals)
- description: Create `src/infrastructure/resilience/` as the canonical home for resilience primitives.
- acceptance:
  - `src/infrastructure/resilience/mod.rs` MUST be created with a module-level rustdoc comment stating it is the canonical home for infrastructure-layer resilience primitives, a `pub mod circuit_breaker;` declaration, and a comment block listing planned additions (`retry` — configurable retry policy with exponential backoff; `rate_limiter` — token-bucket limiter for LLM API calls; `bulkhead` — concurrency limiter for external service calls)
  - `pub mod resilience;` MUST be added to `src/infrastructure/mod.rs`, preferably in alphabetical order
  - `circuit_breaker` MUST be `pub mod` inside `resilience/mod.rs` and `resilience` MUST be `pub mod` inside `infrastructure/mod.rs`, making `paladin::infrastructure::resilience::circuit_breaker` fully public — consistent with the old path's public accessibility
  - `src/lib.rs` MUST be verified to expose `pub mod infrastructure;` or equivalent; if `infrastructure` is private or re-exported differently, visibility MUST be updated
  - The scaffold is documentation-only: no retry policy, rate limiter or bulkhead implementation is part of this Epic; nothing else in the infrastructure layer changes
- scope: src/infrastructure/resilience, module scaffold, visibility

## REQ-circuitbreaker-relocation
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.1, §4.3, Goal 1, §5 Non-Goals, §6 dependency direction, §7)
- description: Move `CircuitBreaker` and `CircuitState` into the infrastructure layer without behavioural change.
- acceptance:
  - `CircuitBreaker` and `CircuitState` MUST move from `src/application/use_cases/paladin/circuit_breaker.rs` to `src/infrastructure/resilience/circuit_breaker.rs`
  - The three states, thresholds and timeout logic MUST NOT change — this is a pure relocation with no functional modification
  - The internal import `use crate::application::use_cases::paladin::error::PaladinError;` remains valid after the move (same facade crate) and MUST NOT be changed unless `cargo build` fails; confirm with `cargo check` before updating consumers
  - `PaladinError::CircuitBreakerOpen` stays in the application layer; `PaladinError` is unchanged
  - No crate extraction: the circuit breaker stays in the `paladin` facade crate and a `paladin-infra` crate is explicitly out of scope
  - No `CircuitBreakerPort` trait abstraction; that is a future consideration
  - `PaladinExecutionService` importing an infrastructure type is a technically-inverted layering accepted as a pragmatic trade-off within the facade crate, because it is a module-level move inside one crate and the breaker is injected as an `Arc<CircuitBreaker>` parameter rather than constructed inside the service
  - The retry logic embedded in `mcp_sse_adapter.rs` and `api_content_deliverer.rs` MUST NOT be consolidated into the resilience module during this Epic
- scope: CircuitBreaker, CircuitState, relocation, layering trade-off

## REQ-circuitbreaker-rustdoc-updates
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.4, Goal 6, §7 doc test compilation, §8.3)
- description: Update every inline rustdoc example in the moved file.
- acceptance:
  - Every `use` statement inside `///` or `//!` doc examples within `circuit_breaker.rs` MUST change from `use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` to `use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - This applies to both the module-level `//!` block examples and all method-level `///` examples
  - `cargo test --doc` MUST pass — all rustdoc examples in `circuit_breaker.rs` and `paladin_execution_service.rs` MUST compile with the new path
- scope: rustdoc examples, doc tests

## REQ-paladin-execution-service-import
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.5, Goal 3, §7 clippy)
- description: Update `PaladinExecutionService` to the new import path atomically with the move.
- acceptance:
  - `src/application/use_cases/paladin/paladin_execution_service.rs` MUST change `use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` to `use crate::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - All inline rustdoc examples inside `paladin_execution_service.rs` referencing the old path MUST be updated
  - This MUST be completed in the same commit as the file move, otherwise the move triggers `unused_imports` warnings
- scope: PaladinExecutionService import, atomic commit

## REQ-circuitbreaker-example-updates
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.6, §4.8, Goal 4, §8.9)
- description: Update all 15 example files and the README code sample.
- acceptance:
  - These 15 files MUST switch to the new canonical path: `basic_paladin.rs`, `agent_handoffs.rs`, `autonomous_full_config.rs`, `autonomous_planning.rs`, `autonomous_prompt_generation.rs`, `battalion_checkpoint_recovery.rs`, `citadel_autosave.rs`, `citadel_restore.rs`, `dynamic_temperature.rs`, `herald_custom_formatter.rs`, `herald_json_output.rs`, `herald_markdown_output.rs`, `paladin_with_config.rs`, `vision_analysis.rs`, `vision_battalion.rs`
  - `README.md` (around lines 659-660) MUST have its `use` statement updated and surrounding prose corrected if it names the module location
  - All 15 examples, the 3 test files and `README.md` MUST compile with the new import path
- scope: 15 example files, README code sample

## REQ-circuitbreaker-test-updates
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.7, Goal 5, §8.1, §8.2, §8.4 to §8.6)
- description: Update the three CLI test files and pass all quality gates.
- acceptance:
  - `tests/cli/paladin_execution_test.rs`, `tests/cli/tool_integration_test.rs` and `tests/cli/error_handling_test.rs` MUST switch to `use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - `cargo build --workspace` MUST succeed with zero errors; `cargo test` MUST pass with all existing `CircuitBreaker` tests compiling and passing
  - `cargo clippy --workspace -- -D warnings` MUST report zero warnings; `cargo fmt --all -- --check` MUST pass; `cargo doc --workspace --no-deps` MUST produce clean documentation with no broken links
- scope: CLI test files, quality gates

## REQ-circuitbreaker-old-path-retired
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.11, Goal 7, §8.7, §4.1)
- description: The old module path is intentionally broken; no re-export is left behind.
- acceptance:
  - `pub mod circuit_breaker;` MUST be removed from `src/application/use_cases/paladin/mod.rs`
  - `paladin::application::use_cases::paladin::circuit_breaker` MUST NO LONGER resolve after this Epic — "no `pub use` re-export is added"; "the old path is intentionally retired"
  - Grepping `mod.rs` MUST confirm no lingering re-export
- scope: old module path retirement, no re-export
- note: the M6 milestone-overview DOC (Epic 4 AC 5) requires the facade to re-export `CircuitBreaker` at the original path for backward compatibility. See INGEST-CONFLICTS.md WARNINGS. Shipped code follows this PRD.

## REQ-circuitbreaker-stable-api-update
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md (§4.12, Goal 8, §7 final-api.txt, §8.8, §9 OQ1)
- description: Record the new canonical path in the API surface documents.
- acceptance:
  - `STABLE_API.md` MUST remove the entry for `paladin::application::use_cases::paladin::circuit_breaker` and add `paladin::infrastructure::resilience::circuit_breaker` as the new stable module location, with `CircuitBreaker` and `CircuitState` as its stable public types
  - `final-api.txt` MUST be regenerated after running `cargo doc --workspace --no-deps`, and the old path MUST be verified absent
  - `api_surface_current.txt` may also need updating alongside `final-api.txt`
  - Whether an automated regeneration script exists (e.g. `make api-surface`) MUST be confirmed with the team before closing the Epic
- scope: STABLE_API.md, final-api.txt, api_surface_current.txt

---

# Ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution`

40 documents consumed (11 PRD, 29 DOC, 0 ADR, 0 SPEC). The 11 PRDs below produce these entries.

Variant IDs (`-v1` / `-v2` / `-v3`) are preserved unmerged per the standing constraint. Where the
shipped tree settles a variant, the entry carries a `- settled-by:` line pointing at
`.planning/intel/code-verification.md` (run-4 section). That is a **fact about the tree**, not a
decision taken here.

Path claims in these PRDs are as-written at authoring time (2026-05-25 to 2026-06-06). Several were
overtaken inside the same milestone — most notably `src/application/use_cases/` (renamed to
`services/` by M8 Epic 4) and the `paladin-storage` / `paladin-web` feature-flag shapes. Resolve
current locations through `.planning/codebase/` or the tree.

---

## REQ-m7-cost-benefit-gate
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.1, Goal 8)
- description: Hard gate requiring a written cost-benefit matrix before any Milestone 7 Epic 1 code is moved.
- acceptance:
  - Before any code is moved, a cost-benefit matrix MUST be produced for each of the four candidate extractions
  - The matrix MUST evaluate each candidate on four criteria: dependency weight, change frequency, consumer selectivity, extraction complexity
  - Each candidate MUST receive a **Go** or **Defer** decision with written justification
  - Definition of done: write the full matrix and all Go/Defer decisions to `project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md`; for each Defer, mark the Epic-tracker task `deferred — see cost-benefit-assessment.md` AND create a backlog ticket `Extract paladin-{name} crate` tagged `milestone-8+-candidate`
  - The cost-benefit matrix is "the authoritative source of record for *why* a decision was made"
- scope: extraction go/defer gate, cost-benefit-assessment.md, backlog tickets
- note: the assessment shipped and returned **four Go decisions, zero Defer** — so sub-tasks 1.4/1.5 (mark deferred, create backlog tickets) were recorded N/A. See `context.md` Topic: Milestone 7 Epic 1 cost-benefit assessment.

## REQ-paladin-web-extraction
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.2, Goal 1)
- description: Extract the web-server subsystem into a `crates/paladin-web` workspace crate.
- acceptance:
  - `crates/paladin-web/` MUST be created with a `Cargo.toml` declaring **`actix-web` and `axum` as direct (non-optional) dependencies**
  - `actix-web`/`axum` MUST NOT appear in the facade crate's `[dependencies]` after extraction
  - `src/infrastructure/web/mod.rs` → `crates/paladin-web/src/lib.rs`; `src/infrastructure/web/user_controller.rs` → `crates/paladin-web/src/user_controller.rs`; `src/infrastructure/adapters/output/api_content_deliverer.rs` → `crates/paladin-web/src/adapters/api_content_deliverer.rs`
  - `cargo build -p paladin-web` MUST succeed in isolation
  - `ServiceRunner` MUST conditionally depend on `paladin-web` when the `web-server` feature is active
  - The facade's `web-server` flag MUST be redefined to activate `paladin-web` rather than raw `actix-web`/`axum`
  - Co-located `#[cfg(test)]` unit tests MUST move with their source; `tests/` integration tests referencing web types MUST import from `paladin-web`
  - `cargo build --workspace` and `cargo test --workspace` MUST pass after this task
- scope: paladin-web crate, actix-web, axum, ServiceRunner, web-server feature
- note: the two-framework clause is **v1** of a competing pair — see `REQ-actix-removal` (M8 Epic 7) which requires actix-web removed and banned. See INGEST-CONFLICTS.md WARNINGS.
- settled-by: code-verification.md run-4 — `crates/paladin-web/` ships with zero `actix` references; facade `web-server = ["dep:paladin-web", "dep:axum"]`.

## REQ-paladin-notifications-extraction
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.3, Goal 1)
- description: Extract the notification adapters into a `crates/paladin-notifications` workspace crate.
- acceptance:
  - `crates/paladin-notifications/` MUST be created with feature flags `email` (gates `lettre` + `handlebars`), `push`, `system`
  - `email_notification_adapter.rs`, `push_notification_adapter.rs`, `system_notification_adapter.rs`, `mod.rs` MUST move from `src/infrastructure/adapters/notifications/`
  - `lettre` and `handlebars` MUST NOT appear in the facade's `[dependencies]` after extraction
  - `cargo build -p paladin-notifications --no-default-features` MUST succeed (crate skeleton without adapter implementations)
  - The facade's `notifications` flag MUST activate `paladin-notifications` with `features = ["email", "push", "system"]`
  - Adapter unit tests relocate into the new crate; notification-delivery integration tests stay at the workspace root
  - `cargo build --workspace` and `cargo test --workspace` MUST pass
- scope: paladin-notifications crate, lettre, handlebars, notifications feature

## REQ-paladin-content-extraction
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.4, Goal 1)
- description: Extract the content-processing pipeline (9 adapters + 13 use-case services) into `crates/paladin-content`.
- acceptance:
  - `crates/paladin-content/` MUST be created with feature flags `pdf` (gates `pdf-extract`), `web-scraping` (gates `scraper`), `rss` (gates the `rss` crate), `news-api` (gates `NewsApiFetcher` HTTP logic), `tiktoken` (gates `tiktoken-rs`)
  - Nine infrastructure adapters MUST move into `crates/paladin-content/src/adapters/`: `document/{pdf_extractor,document_adapter,mod}.rs` and `input/{file_content_fetcher,file_content_list_fetcher,http_content_fetcher,local_file_fetcher,news_api_fetcher,mod}.rs`
  - Fourteen application-layer files MUST move into `crates/paladin-content/src/use_cases/`: `content_{aggregator,analysis,delivery,fetching,filtering,ingestion,list_fetching,list_ingestion,list,llm_analysis,ml_analysis,nlp_analysis,summarizer}_service.rs` plus `mod.rs`
  - `pdf-extract`, `scraper`, `tiktoken-rs` and `rss` MUST NOT appear in the facade's `[dependencies]`
  - `cargo build -p paladin-content --no-default-features` MUST succeed
  - The facade's `content-processing` flag MUST activate `paladin-content` with all capability features enabled
  - `cargo build --workspace` and `cargo test --workspace` MUST pass
- scope: paladin-content crate, content adapters, content use-case services, content-processing feature
- note: the target directory is named `use_cases/` here; M8 Epic 6 later renames it to `services/`. See `REQ-paladin-content-services-rename`.
- note: `content_ingestion_service.rs` is listed as moving to `paladin-content`, but the facade retains its own ~1,211-LOC `content_ingestion_service.rs` — recorded as deferred item D4. See `REQ-m8-deferred-items-register`.

## REQ-paladin-storage-extraction
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.5, Goal 1)
- description: Extract the SQL repository implementations into `crates/paladin-storage`.
- acceptance:
  - `crates/paladin-storage/` MUST be created with feature flags `sqlite` (gates `sqlx` sqlite runtime) and `mysql` (gates `sqlx` mysql runtime)
  - `sqlite_content_repository.rs`, `sqlite_user_repository.rs`, `mysql_content_repository.rs`, `mod.rs` MUST move from `src/infrastructure/repositories/`
  - `src/infrastructure/repositories/file_content_repository.rs` MUST **NOT** be moved — "Despite its filename, it implements `ContentDeliveryService` / `BatchContentDeliveryService` from `paladin-ports` and writes content to the local filesystem; it does not use `sqlx` … It stays in the facade crate. A future content-delivery crate (Milestone 8+) is its correct long-term home."
  - SQLite migration files MUST move from `migrations/` to `crates/paladin-storage/migrations/`, with the `sqlx::migrate!` macro path updated and verified via `cargo sqlx migrate info`
  - `sqlx` MUST NOT be a direct facade dependency after extraction
  - `cargo build -p paladin-storage --features sqlite` and `--features mysql` MUST each succeed independently
  - Repository unit tests relocate; Docker-dependent DB integration tests stay at workspace-root `tests/integration/`
- scope: paladin-storage crate, sqlx, sqlite, mysql, migrations
- note: the `file_content_repository.rs` clause is **v1** of a three-way disagreement — `facade-audit.md` List B assigns it to `paladin-storage` (v2) and the 2026-06-04 reconciliation deletes it outright (v3). See INGEST-CONFLICTS.md WARNINGS.

## REQ-storage-feature-flags-v1
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.5.6, §7.2, §9 Q2)
- description: Two granular facade storage feature flags plus a convenience alias, with `paladin-storage` optional.
- acceptance:
  - `paladin-storage/Cargo.toml` `[features]`: `default = []`, `sqlite = ["dep:sqlx/sqlite"]`, `mysql = ["dep:sqlx/mysql"]`
  - Facade `[features]`: `storage-sqlite = ["dep:paladin-storage", "paladin-storage/sqlite"]`, `storage-mysql = ["dep:paladin-storage", "paladin-storage/mysql"]`, and a `storage = ["storage-sqlite", "storage-mysql"]` convenience alias
  - `paladin-storage` MUST be declared `{ workspace = true, optional = true }` in the facade `[dependencies]`
  - A downstream consumer depending only on SQLite MUST NOT link `libmysqlclient`, and vice versa
  - Open question 2 resolution recorded: "Two granular flags … `storage` convenience alias enables both"
- scope: storage-sqlite, storage-mysql, storage alias, optional paladin-storage dependency
- note: **v1**. Superseded by the 2026-06-04 reconciliation execution log (commit `897e77e`, "Make `paladin-storage` non-optional; drop facade sqlite fallbacks; `storage-sqlite` feature retired") — see `REQ-storage-nonoptional-v2`. Both preserved. See INGEST-CONFLICTS.md WARNINGS.

## REQ-storage-nonoptional-v2
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/facade-cleanup-RECONCILIATION-2026-06-04.md (§4 Category 2, §7 execution log commit `897e77e`)
- description: `paladin-storage` becomes a non-optional facade dependency; the `storage-sqlite` feature is retired and the facade's sqlite fallback repositories are deleted.
- acceptance:
  - `sqlite_content_repository.rs` (810 LOC) and `sqlite_user_repository.rs` (676 LOC) MUST be deleted from the facade — they were the `#[cfg(not(storage-sqlite))]` fallback duplicates
  - `paladin-storage` re-exports MUST become unconditional
  - The `storage-sqlite` feature MUST be retired
  - ~1,486 LOC removed; build/tests/clippy/fmt green on default and all rewired feature flags
  - Correction on record: "`sqlite_*_repository.rs` were **not** redundant in the default build (they were the active default-build impl); resolved via the non-optional-storage change, not a naive delete"
- scope: paladin-storage non-optional, storage-sqlite retirement, facade sqlite fallbacks
- note: **v2**, competing with `REQ-storage-feature-flags-v1`.
- settled-by: code-verification.md run-4 — root `Cargo.toml` declares `paladin-storage = { workspace = true, features = ["sqlite"] }` (non-optional) with an inline comment "SQLite repositories are always available"; `storage-mysql` and `storage = ["storage-mysql"]` survive; `storage-sqlite` is absent.

## REQ-facade-workspace-metadata
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§4.6)
- description: Update the facade crate and workspace manifest after all Go-decision extractions.
- acceptance:
  - Each extracted crate MUST be added to `[workspace.members]`, to `[workspace.dependencies]` with `path = "crates/paladin-<name>"`, and as an **optional** facade dependency
  - Facade feature flags (`web-server`, `notifications`, `content-processing`, `storage-sqlite`, `storage-mysql`) MUST be redefined to activate the corresponding new crate rather than raw third-party dependencies
  - All third-party packages now owned exclusively by an extracted crate MUST be removed from the facade `[dependencies]`
  - The facade's `full` convenience feature MUST continue to enable all extracted crates
  - "All public types previously re-exported from the facade crate and originating in an extracted crate must continue to be re-exported from the same facade path … No public API paths may be silently removed."
  - `cargo doc --workspace --no-deps` MUST produce zero errors and zero warnings
  - `cargo test --workspace --all-features` MUST pass
- scope: workspace members, workspace dependencies, facade feature flags, backward-compatible re-exports

## REQ-extracted-crate-dependency-rule
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§6.1, Goal 2)
- description: Hexagonal dependency direction for the four extracted infrastructure crates.
- acceptance:
  - `paladin-web`, `paladin-notifications`, `paladin-content` and `paladin-storage` MUST each depend only on `paladin-ports`, `paladin-core`, and workspace-shared dependencies
  - "No extracted crate may depend on another extracted crate or on the `paladin` facade."
  - All four crates MUST be **opt-in** — not enabled in the facade's `default` features
  - All new crates MUST be initialized at version `0.1.0`, governed by `[workspace.package]` version until independent versioning is introduced (out of scope)
- scope: crate dependency direction, opt-in defaults, lockstep versioning
- note: `paladin-content` ships an optional `paladin-llm` dependency behind its `llm` feature, which is an extracted-crate-to-extracted-crate edge. The PRD's own §4.4 complexity note anticipated this ("use-case services depend on `paladin-llm` for LLM analysis, creating an inter-crate dependency that must be handled carefully") without amending the rule.

## REQ-extraction-order-and-shims
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§6.2, §6.3, §6.4)
- description: Recommended extraction sequencing and the incremental temporary-re-export migration pattern.
- acceptance:
  - Recommended order: **storage → notifications → content → web** (simplest to most complex); the developer may reorder based on Task 1.1 risk assessment
  - Each extraction MUST follow the incremental pattern: (1) create crate and move sources, (2) add a temporary `pub use paladin_<name>::<module>::*;` re-export in the original location, (3) verify `cargo test --workspace` passes, (4) update internal consumers to import from the new crate, (5) remove the temporary re-export
  - Test migration strategy: co-located `#[cfg(test)]` unit tests move with their source; single-subsystem integration tests move into the new crate's `tests/`; multi-crate or Docker-service integration tests remain at workspace-root `tests/integration/` with updated import paths
- scope: extraction ordering, temporary re-export migration, test placement

## REQ-tensorflow-stays-facade-v1
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§7.3, §9 Q1)
- description: `tensorflow_adapter.rs` stays in the facade for Milestone 7; a `paladin-ml` crate is deferred to Milestone 8+.
- acceptance:
  - `src/infrastructure/adapters/input/tensorflow_adapter.rs` MUST stay in the facade crate; it MUST NOT move into `paladin-content`
  - Rationale of record: "`TensorFlowAdapter` implements `MlPort` from `paladin-ports` and performs ML model inference … It is an ML adapter, not a content-processing adapter … Placing it in `paladin-content` would be semantically incorrect"
  - "The correct long-term home is a future `paladin-ml` crate alongside `NlpPort` and related adapters. However, creating a crate for a single placeholder implementation is premature for Milestone 7."
  - Action before Task 1.4: ensure the adapter is gated behind an `ml` feature flag if not already; document the `paladin-ml` crate as a Milestone 8+ backlog candidate
- scope: tensorflow_adapter.rs, ml feature flag, future paladin-ml crate
- note: **v1** of a three-step chain — `REQ-tensorflow-ml-feature-gate-v2` (M8 Epic 3 applies the gate) then `REQ-deferred-tensorflow-ml-adapter-v3` (M8 reconciliation deletes the adapter and the flag entirely).

## REQ-sqlx-workspace-dependency
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§7.5, §9 Q3)
- description: `sqlx` remains a `[workspace.dependencies]` declaration shared by `paladin-memory` and `paladin-storage`.
- acceptance:
  - The `sqlx` declaration MUST stay in `[workspace.dependencies]` — it MUST NOT be moved exclusively into `crates/paladin-storage/Cargo.toml`
  - Both `paladin-memory` (for `SqliteStore`) and `paladin-storage` MUST reference `sqlx = { workspace = true, optional = true }` and gate it behind their own feature flags
  - The workspace declaration MUST be updated to include the `mysql` feature set so `paladin-storage/mysql` can activate it: `sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "mysql", "chrono", "uuid", "json"] }`
  - The workspace-level declaration provides the version lock and ensures Cargo resolves a single copy in the dependency graph
- scope: sqlx workspace declaration, feature availability vs activation
- note: shipped `[workspace.dependencies] sqlx` sets `default-features = false` and features `["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json", "migrate"]` — `mysql` is **not** in the workspace feature list, and `migrate` was added. The `default-features = false` change is attributed to the RustSec remediation work (see `REQ-rustsec-hardening-actions`).

## REQ-dependency-isolation-metrics
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md (§2 Goal 5, §8)
- description: Measurable dependency-isolation outcome for the extraction epic.
- acceptance:
  - `cargo tree -p paladin-core --all-features` output MUST NOT contain `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, or `sqlx`
  - `cargo tree -p paladin-battalion --all-features` output MUST NOT contain the same six
  - `cargo build --workspace --all-features` MUST succeed; `cargo test --workspace` MUST pass with no regressions
  - `cargo doc --workspace --no-deps` MUST produce zero errors and zero warnings
  - The cost-benefit matrix from Task 1.1 MUST be committed to the repository
  - Stated problem this solves: "Feature flags eliminate compilation but not dependency resolution. Only crate extraction makes these dependencies truly opt-in at the `Cargo.toml` level."
- scope: cargo tree isolation checks, workspace build/test/doc gates

## REQ-docker-workspace-build
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-01 to FR-06, Goals 1-2)
- description: Adapt `Dockerfile.chef` and `Dockerfile` to the multi-crate workspace with tight dependency-layer caching.
- acceptance:
  - `Dockerfile.chef` planner stage MUST copy root `Cargo.toml` + `Cargo.lock`, **all `crates/*/Cargo.toml` files (nine crates)**, and all source trees (`src/` and `crates/*/src/`)
  - `cargo chef prepare` MUST produce a `recipe.json` capturing all workspace member dependencies, not only the root crate's
  - The cook stage MUST run `cargo chef cook --release --workspace --recipe-path recipe.json`
  - The application build stage MUST run `cargo build --release --workspace --bin paladin`
  - `Dockerfile` (simple builder) MUST copy `crates/` alongside `src/`, `Cargo.toml`, `Cargo.lock`, and use `--workspace`
  - Both Dockerfiles MUST continue producing a runnable `paladin` binary in the existing minimal runtime base images (distroless / `debian:12-slim` unchanged)
  - COPY ordering MUST place manifests first, then `chef prepare`, then `chef cook`, then source (`src`, `crates`, `migrations`) last
  - `cargo-chef` MUST be pinned to a specific version, verified compatible with workspace resolver `"2"`
- scope: Dockerfile.chef, Dockerfile, cargo-chef, layer caching

## REQ-build-baselines-doc
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-07, Goal 6)
- description: A measured build-time and image-size baseline document for the current 10-crate workspace.
- acceptance:
  - `docs/BUILD_BASELINES.md` MUST be created after the Docker changes
  - It MUST record `cargo build --workspace` clean build time (3 runs, median)
  - Per-crate incremental build times for `paladin-core`, `paladin-llm`, `paladin-battalion`, `paladin-storage`, `paladin-web` (3 runs each, median)
  - `docker build -f Dockerfile.chef .` total time for cold cache (3 runs, median) and warm cache / source-only change (3 runs, median)
  - Final compressed image size for both `Dockerfile.chef` and `Dockerfile` outputs
  - `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` MUST be treated as historical context only — "**do not** use it as the current baseline — it predates the 4 crates added in Milestone 7 Epic 1 and does not include Docker image size measurements"
  - Docker image size regression vs. a hypothetical monolithic build: within 10%
- scope: docs/BUILD_BASELINES.md, build-time and image-size measurement

## REQ-makefile-workspace-targets
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-08 to FR-17, FR-20, Goal 3)
- description: Every Makefile target uses workspace-aware cargo commands.
- acceptance:
  - `build` → `cargo build --workspace`; `build-release` → `cargo build --release --workspace`
  - `test` → `cargo test --workspace --lib --bins`; `test-doc` → `cargo test --workspace --doc`; `test-all` → `test`, `test-doc`, `test-integration` in sequence
  - `lint` → `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `fmt` → `cargo fmt --all`; `check` → `cargo check --workspace --all-targets`
  - `clean-code` → `fmt`, `lint`, `check` in sequence
  - `doc` → `cargo doc --workspace --no-deps`; `bench` → `cargo bench --workspace`
  - `make clean-code` MUST exit 0
- scope: Makefile workspace flags

## REQ-makefile-per-crate-targets
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-18, FR-19)
- description: Per-crate `make test-<crate>` convenience targets for every workspace member.
- acceptance:
  - Ten targets MUST exist, one per member: `test-core`, `test-ports`, `test-battalion`, `test-llm`, `test-memory`, `test-storage`, `test-notifications`, `test-content`, `test-web`, `test-facade`
  - Each MUST run `cargo test -p <crate-name>` (`test-facade` → `cargo test -p paladin`)
  - All per-crate targets MUST be listed in `make help` output
  - All 10 per-crate targets MUST exit 0
- scope: Makefile per-crate test targets, make help
- note: shipped verbatim — all ten targets exist at `Makefile:167-212`. `test-herald` does not exist; `paladin-herald` postdates this PRD.

## REQ-ci-workflow-triggers
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-21, FR-22, FR-28)
- description: CI workflow file, trigger set, and pinned toolchain.
- acceptance:
  - A GitHub Actions workflow MUST exist at `.github/workflows/ci.yml`
  - It MUST trigger on `push` to `main` and `feature/**` branches, and on `pull_request` targeting `main`
  - All jobs MUST use a pinned Rust toolchain version consistent with `rust-toolchain.toml` (or `Cargo.toml` `rust-version` if present)
  - Cache keys: `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock', '**/Cargo.toml') }}` with `restore-keys: ${{ runner.os }}-cargo-`
- scope: .github/workflows/ci.yml, triggers, toolchain pinning, cache keys
- note: open question 1 (does a `rust-toolchain.toml` exist, or should one be created?) was never recorded as answered in this document set.

## REQ-ci-per-crate-matrix
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-23)
- description: A parallel per-crate CI matrix job.
- acceptance:
  - A `test-crate` matrix job MUST run in parallel for all nine workspace crates plus the facade
  - Each matrix entry MUST execute `cargo test -p <crate-name>`
  - It MUST cache `~/.cargo/registry`, `~/.cargo/git` and `target/` via `actions/cache`, keyed on OS, toolchain, and the hash of all `Cargo.lock` + `Cargo.toml` files
  - Per-crate matrix jobs SHOULD test with `--all-features` unless a crate-specific exception is documented in `docs/FEATURE_FLAGS.md`
- scope: CI per-crate matrix, cache configuration

## REQ-ci-workspace-job
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-24)
- description: A workspace-level CI job gated on the per-crate matrix.
- acceptance:
  - A `test-workspace` job MUST depend on the per-crate matrix completing successfully
  - It MUST run `cargo test --workspace`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check`
  - The GitHub Actions CI pipeline MUST be green on `main`
- scope: CI workspace job, clippy, fmt

## REQ-ci-integration-job
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-25)
- description: A Docker-service-backed integration-test CI job.
- acceptance:
  - An `integration-tests` job MUST depend on `test-workspace`
  - It MUST spin up Redis, MinIO and MySQL via `docker-compose -f docker/docker-compose.test.yml up -d`
  - It MUST run `./scripts/run_integration_tests.sh -m ci`
  - It MUST tear down services in an `if: always()` step
- scope: CI integration job, docker-compose.test.yml, run_integration_tests.sh

## REQ-ci-publish-dry-run-v1
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-26, §6 Per-Crate Publish Order, Goal 7)
- description: A per-crate, dependency-ordered `cargo publish --dry-run` CI job.
- acceptance:
  - A `publish-dry-run` job MUST depend on `test-workspace` and run only on pushes to `main` (not on pull requests)
  - It MUST execute `cargo publish --dry-run -p <crate>` **for each crate in this dependency order**: `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`, `paladin` (facade)
  - It MUST NOT actually publish to crates.io
  - "Violating this order will cause `cargo publish --dry-run` to fail."
  - The job MUST exit 0 for all 10 crates
- scope: publish-dry-run CI job, dependency-ordered publishing
- note: **v1**. The shipped job runs a single workspace-wide `cargo publish --workspace --dry-run` with a recorded counter-rationale — see `REQ-ci-publish-dry-run-v2`. Both preserved.

## REQ-ci-publish-dry-run-v2
- source: /workspace/.planning/intel/code-verification.md (run-4 section; `.github/workflows/ci.yml:617-680` inline rationale)
- description: A single workspace-wide publish dry run replaces the per-crate ordered sequence.
- acceptance:
  - The `publish-dry-run` job runs `cargo publish --workspace --dry-run` as one step
  - Recorded rationale: "A single workspace-wide dry run packages and verifies every crate in dependency order, resolving intra-workspace `version` requirements via their `path` entries. Per-crate `cargo publish --dry-run -p <crate>` cannot work on a version bump: the not-yet-published new version of each sibling fails the `version = \"X\"` requirement of its dependents."
  - The job depends on `[lint, test]` and is gated on `github.event_name == 'push' && github.ref == 'refs/heads/main'`
- scope: publish-dry-run CI job, workspace-wide dry run
- note: **v2**, competing with `REQ-ci-publish-dry-run-v1`. This variant is not carried by any ingested document — it is read from the tree and is recorded here because the rationale is a substantive technical position that contradicts FR-26.

## REQ-ci-feature-flag-matrix
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-27)
- description: A feature-flag matrix CI job.
- acceptance:
  - A `feature-flags` job MUST build the workspace with `--no-default-features`, `--all-features`, and default features only (no extra flags)
  - Purpose: catch compilation errors under different configurations
- scope: CI feature-flag matrix

## REQ-integration-test-placement
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-29 to FR-33, Goal 5)
- description: Integration-test placement rules and script/compose adaptation for the workspace.
- acceptance:
  - Integration tests importing from multiple crates MUST live in the workspace-root `tests/` directory and depend on the `paladin` facade crate
  - Integration tests exercising a single crate in isolation MUST live in that crate's `tests/` directory (e.g. `crates/paladin-storage/tests/`)
  - `scripts/run_integration_tests.sh` MUST be reviewed and updated to run `cargo test --workspace --test '*'` (or the appropriate per-test invocation) rather than a single-crate path
  - `docker/docker-compose.test.yml` MUST be verified to start and connect correctly from the workspace root; no path changes expected — document any discoveries
  - `make test-integration-docker` MUST exit 0
- scope: tests/ placement, run_integration_tests.sh, docker-compose.test.yml

## REQ-integration-tests-doc
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md (FR-34, §9 Q3)
- description: Documentation of integration-test structure and service requirements.
- acceptance:
  - `docs/INTEGRATION_TESTS.md` MUST be created describing which tests live where (workspace root vs per-crate), how to run integration tests locally, and what services each test group requires
  - Open question 3 requires an audit of `tests/integration/` to build the test→service mapping accurately
- scope: docs/INTEGRATION_TESTS.md

## REQ-sanctum-bench-migration
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-01 to FR-05, Goal 2)
- description: Move the one active benchmark into the crate that owns the measured functionality.
- acceptance:
  - `benches/sanctum_benchmarks.rs` MUST move to `crates/paladin-memory/benches/sanctum_benchmarks.rs`
  - `crates/paladin-memory/Cargo.toml` MUST register the migrated benchmark so it is runnable from that crate
  - The workspace root benchmark configuration MUST no longer treat `sanctum_benchmarks` as root-owned
  - `cargo bench -p paladin-memory` MUST execute the migrated benchmark successfully
  - Post-migration results MUST be compared against pre-migration results and documented as within an acceptable noise margin; the PRD sets no specific percentage threshold but the comparison method used MUST be recorded
- scope: sanctum_benchmarks, paladin-memory benches, root Cargo.toml

## REQ-disabled-bench-disposition
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-06 to FR-10, Goal 3, §6.3)
- description: Every disabled benchmark is either reactivated in its owning crate or removed with written rationale — no third option.
- acceptance:
  - Five disabled benchmarks MUST each be reviewed against the current API surface: `battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, `paladin_benchmarks`, `arsenal_benchmarks`
  - Each MUST produce **one of two outcomes only**: reactivate in the owning crate, or remove with a documented deprecation reason
  - "Disabled benchmarks that cannot be meaningfully restored against the refactored API must be removed rather than left disabled for later."
  - Each removal MUST have a short written rationale in the benchmark assessment output, reflected in `CHANGELOG.md` or equivalent milestone documentation
  - After the Epic, no benchmark file may remain marked disabled in workspace manifests or carried forward as a commented-out placeholder without documented disposition
  - Deprecation standard: the removal rationale MUST explain why the benchmark is obsolete, what replaced it if anything, and why restoration is not the right outcome
- scope: disabled benchmark review, deprecation rationale, CHANGELOG

## REQ-battalion-benchmarks
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-11, FR-12, §6.1, §6.2)
- description: New orchestration-overhead benchmarks in the crate that owns battalion execution.
- acceptance:
  - Exactly three scenarios MUST be covered: Formation execution with 3 agents in sequence; Phalanx execution with 5 agents in parallel; Campaign execution using a branching DAG
  - They MUST use mock `PaladinPort` implementations or an equivalent mock execution boundary "so they measure orchestration overhead rather than external model latency"
  - Target location: `crates/paladin-battalion/benches/`
- scope: battalion benchmarks, Formation, Phalanx, Campaign, mock PaladinPort

## REQ-llm-serialization-benchmark
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-13, §6.1, §7)
- description: An LLM adapter benchmark measuring serialization overhead only.
- acceptance:
  - The benchmark MUST measure request/response serialization overhead only
  - It MUST explicitly exclude live HTTP calls and remote provider latency — "using live provider calls would make results noisy and unsuitable as a framework baseline"
  - Target location: `crates/paladin-llm/benches/`
- scope: LLM serialization benchmark, paladin-llm benches

## REQ-garrison-benchmarks
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-14, §6.1)
- description: New garrison in-memory read/write benchmarks at three history sizes.
- acceptance:
  - The benchmarks MUST measure in-memory read and write operations at history sizes of **100, 1000 and 10000 entries**
  - Target location: `crates/paladin-memory/benches/`
- scope: garrison benchmarks, history sizes

## REQ-config-loading-benchmark
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-15, §7, §9 Q1)
- description: A configuration-loading benchmark against the current settings path.
- acceptance:
  - The benchmark MUST measure `Settings::new()` and the current per-domain configuration loading path
  - It MUST target the current settings-loading path used by the refactored workspace, "not legacy configuration entry points that no longer control runtime behavior"
  - Ownership: "The crate that owns application configuration loading" — open question 1 asks which crate that is if config remains shared
- scope: config benchmark, Settings::new, per-domain config loading
- note: `benchmark-assessment.md` closes open question 1 — `Settings` is defined in `src/config/settings.rs` and no extracted crate owns it, so the benchmark belongs in the root crate "unless a later architectural change moves configuration ownership into a dedicated crate."

## REQ-critical-path-bench-scope
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-16, §5)
- description: A closed scope for critical-path benchmark categories.
- acceptance:
  - Critical-path benchmarks are limited to exactly four categories confirmed by the user: battalion orchestration, LLM adapter serialization overhead, garrison read/write performance, config loading
  - "No additional critical-path categories are required by this PRD."
  - Out of scope: benchmark coverage for systems not listed in Epic 3; production optimization work; live provider network latency or end-to-end external HTTP performance; a blocking performance gate in CI; preserving disabled benchmarks in place; reworking unrelated crate APIs solely to save an obsolete benchmark
- scope: benchmark scope boundary

## REQ-workspace-bench-execution
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-17 to FR-19, Goal 4)
- description: `cargo bench --workspace` becomes the authoritative way to run all active benchmarks.
- acceptance:
  - `cargo bench --workspace` MUST run all active benchmarks after migration is complete
  - Benchmark crates and manifests MUST be configured so workspace execution does not require manual per-benchmark file selection
  - If any benchmark requires feature flags, those requirements MUST be documented in the crate-level benchmark setup or the baseline methodology section
- scope: cargo bench --workspace, per-crate manifest registration

## REQ-performance-baseline-doc
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-20 to FR-23, Goal 5, §6.4)
- description: A single published performance baseline report.
- acceptance:
  - A single baseline report MUST be created at `docs/PERFORMANCE_BASELINE.md`
  - It MUST include: benchmark execution date; hardware specification; operating system and Rust toolchain information; benchmark methodology; raw or summarized results for every active benchmark; notes about variance, caveats or unstable measurements
  - Where a comparable pre-workspace or pre-migration measurement exists, a comparison note MUST be included; where none exists, the report MUST explicitly state that the current run is the first baseline
  - The report MUST clearly separate measured results from interpretation
  - Methodology notes MUST be explicit enough for a junior developer to repeat the process without guessing
- scope: docs/PERFORMANCE_BASELINE.md

## REQ-bench-regression-signal
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md (FR-24 to FR-26, Goal 6, §9 Q2)
- description: An optional, explicitly non-blocking CI benchmark regression signal.
- acceptance:
  - A CI-based performance regression check MAY be added, but it MUST be non-blocking for merges
  - If implemented, it MUST flag regressions above a documented threshold and surface the result in CI output, a PR comment, or another team-visible report
  - "Failure or variance in the optional regression check must not fail the required CI pipeline for this Epic."
  - Open question 2: the threshold value is left to the team
- scope: CI benchmark regression signalling, non-blocking gate
- note: `benchmark-assessment.md` records the shipped answer — job `benchmark-regression-signal`, triggered on pull requests and manual dispatch, threshold "more than 3 Criterion `Performance has regressed.` notices in one run", `continue-on-error: true` at job level.

## REQ-crate-metadata-completion
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.1, Goal 1)
- description: crates.io-ready `[package]` metadata for every public crate.
- acceptance:
  - Every public crate MUST have a complete `[package]` section with at least `name`, `version`, `edition`, `authors`, `description`, `readme`, `repository`, `license`, `keywords`, `categories`, `documentation`
  - Metadata MUST match the workspace versioning policy and use the lockstep workspace version for the initial release series
  - Each crate MUST pass `cargo publish --dry-run -p <crate>` without crates.io validation errors
  - Publishable crates MUST have accurate crate ownership boundaries reflected in their dependencies so metadata and dependency declarations agree
  - "Dry-run publish failures should be treated as release blockers, because they usually indicate metadata or dependency problems that downstream consumers will also encounter."
- scope: Cargo.toml [package] metadata, cargo publish dry run

## REQ-per-crate-readme
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.2, Goal 2, §6)
- description: A crate-level README for every public crate, plus an umbrella root README.
- acceptance:
  - Every public crate MUST have a crate-level `README.md`
  - Each README MUST explain the crate's purpose, the problem it solves, the main public types, the feature flags it exposes, and how it relates to the rest of the workspace
  - Each README MUST include enough information for crates.io rendering to be useful to downstream consumers
  - The root `README.md` MUST serve as the umbrella overview and link to the individual crate READMEs
  - READMEs SHOULD prioritize short examples, dependency guidance and feature-flag explanation over internal architecture prose
  - "The documentation should reflect the medieval military naming convention already used in the codebase, but the external presentation must remain clear for crates.io consumers who may not know the internal naming scheme."
- scope: per-crate README.md, root README.md

## REQ-per-crate-changelog
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.3, Goal 2)
- description: A crate-level CHANGELOG for every public crate.
- acceptance:
  - Every public crate MUST have a crate-level `CHANGELOG.md` using Keep a Changelog conventions
  - The crate changelog MUST reflect the crate's own history, including the extraction or stabilization history relevant to that crate
  - Contribution guidance MUST explain how changelog entries are maintained for future releases
- scope: per-crate CHANGELOG.md, contribution guidance
- note: `paladin-herald` — created after this PRD by the M8 reconciliation (commit `66f6c4e`) — ships a `README.md` but **no** `CHANGELOG.md`. See INGEST-CONFLICTS.md WARNINGS.

## REQ-doc-coverage-audit
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.4, Goal 3, §7)
- description: Documentation coverage enforcement and a per-crate coverage audit.
- acceptance:
  - All public crates MUST enable `#![warn(missing_docs)]`
  - Public items MUST have documentation comments unless explicitly exempted by codebase convention
  - `cargo doc --workspace --no-deps` MUST complete without documentation warnings
  - A documentation coverage audit MUST be produced showing documented-public-item percentage per crate, **with the target exceeding 90%**
  - Public API documentation MUST be consistent with the existing `STABLE_API.md` contract and the crate READMEs
  - "Documentation coverage should be checked against the actual public surface after extraction, not against idealized API lists."
- scope: missing_docs lint, cargo doc, coverage audit, 90% target

## REQ-versioning-policy
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.5.1-4.5.3, Goal 4, §7)
- description: A published versioning policy anchored on lockstep versioning.
- acceptance:
  - A versioning policy document MUST define lockstep versioning as the default policy for the first release line
  - The policy MUST state the criteria for transitioning to independent per-crate versioning later
  - The policy MUST define what counts as a breaking change for each crate family
  - The release target is a lockstep pre-1.0 version **anchored to `0.2.0`**, with all public crates versioned together until the project has enough stability to consider independent versioning
  - "Independent per-crate versioning is not the initial release policy and should not be implemented prematurely."
  - The policy MUST be explicit enough that contributors can determine whether a change is breaking without release-engineering context
- scope: versioning policy document, lockstep 0.2.0, breaking-change definition
- note: the actual first publication was at lockstep `0.1.0` under tag `v0.1.0-rc.1`, not `0.2.0`. See INGEST-CONFLICTS.md INFO on the version trajectory.

## REQ-release-checklist
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.5.4-4.5.7, Goal 5)
- description: A repeatable, dependency-aware release checklist and release command.
- acceptance:
  - A release checklist MUST describe the full release path from code freeze to publish and announcement
  - It MUST include these steps in order: code freeze, changelog finalization, version bump, CI green, documentation validation, dry-run publish, publish, tag, announcement
  - Publishing order MUST be dependency-aware and documented as: `paladin-core` first, then `paladin-ports`, then leaf crates, and finally the `paladin` facade
  - The release process MUST be scripted or represented by an equivalent workspace command such as a `make release` target
  - The checklist MUST be concise, repeatable and unambiguous so it can be followed by both maintainers and automation
- scope: release checklist, publishing order, make release

## REQ-stable-api-per-crate
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.6, Goal 6)
- description: Extend `STABLE_API.md` to a per-crate contract with stability tiers.
- acceptance:
  - `STABLE_API.md` MUST be expanded to document the public API surface of each crate individually
  - Every public type and trait MUST carry a stability tier: **Stable, Unstable, or Experimental**
  - Cross-crate dependency contracts MUST be documented so consumers understand which crates are safe to rely on together
  - The API documentation MUST reflect the actual workspace decomposition completed in Epics 1-3
- scope: STABLE_API.md, per-crate sections, stability tiers, cross-crate contracts
- note: `STABLE_API.md` does not exist at the repository root; run-3 verification found the equivalent page shipped as `docs/src/api-reference/stable-api.md` after the Milestone 11 documentation overhaul. Do not plan it as a missing deliverable.

## REQ-release-readiness-audit
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md (§4.7, Goal 7)
- description: Hard release gates that must all pass before a release-candidate tag.
- acceptance:
  - `cargo test --workspace` MUST pass
  - `cargo clippy --workspace -- -D warnings` MUST pass
  - `cargo fmt --all -- --check` MUST pass
  - `cargo doc --workspace --no-deps` MUST pass
  - Every publishable crate MUST pass `cargo publish --dry-run`
  - A security audit MUST be performed with `cargo audit`; `cargo audit` MUST report no blocking security issues for the release candidate
  - License compatibility MUST be verified so dependencies remain compatible with the project's MIT licensing posture
  - The audit MUST review dependency tree and binary size for unexpected bloat before the release candidate tag is approved
- scope: release gates, cargo audit, license compatibility, dependency/binary-size review
- note: `cargo audit` passes only under **policy-managed exceptions** — see `REQ-rustsec-risk-acceptance`. The MIT-only posture in §4.7.7 is superseded by the recorded `MIT OR Apache-2.0` policy in `license-compatibility-decision-checklist.md`; see INGEST-CONFLICTS.md WARNINGS.

## REQ-facade-file-inventory
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/prd-facade-crate-audit.md (Task 1.1, Goal 1)
- description: A complete file-by-file inventory of the facade crate.
- acceptance:
  - Every `.rs` file under `src/` MUST be enumerated using `find src/ -name "*.rs" | sort`
  - For each file, record: **Path** (relative to workspace root), **LOC** (approximate; exact precision not required), **Content type** (one of `re-export shim`, `application service`, `infrastructure adapter`, `config module`, `binary entry point`, `test module`, `dead code`), and **References** (which external crates it imports from or re-exports)
  - The inventory MUST cover all **189** files; no file may be omitted
  - The inventory MUST be saved in the structured table (Appendix A) of `facade-audit.md`
  - The total count in Appendix A MUST match the output of `find src/ -name "*.rs" | wc -l`
- scope: src/ file inventory, facade-audit.md Appendix A

## REQ-facade-file-classification
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/prd-facade-crate-audit.md (Task 1.2, Goals 2, 5)
- description: A disposition for every facade file, using documented rules, producing three derived lists.
- acceptance:
  - Every file MUST be assigned a disposition; no file may have an empty or ambiguous disposition
  - Classification rules: re-export shim → evaluate consumers, delete if zero; application service → **stays**; infrastructure adapter → move to an existing crate if one owns the domain, else flag for Milestone 9+; config module → **stays** (composition root needs config); binary entry point → **stays**; test module → stays with its source; dead code → **delete**
  - "Moves to crate" dispositions MUST specify the target crate; "stays" dispositions MUST carry a one-line justification
  - Three derived lists MUST be produced in the prose section: **List A — Files to Delete**, **List B — Files to Move** (grouped by target crate), **List C — Files That Stay** (with justification)
  - Lists A, B and C MUST be complete, non-overlapping, and together account for all files; Summary totals MUST be arithmetically consistent with the three lists
  - Explicitly requiring classification: `src/application/notifications/*`, `src/application/storage/{sql_store,file_store,user_store}.rs`, all of `src/application/ports/`, all of `src/application/errors/`, all of `src/core/`, all of `src/infrastructure/`
- scope: file disposition rules, List A/B/C, facade-audit.md
- note: the audit shipped with a **PRD correction**: `planning_error.rs` and `prompt_error.rs` under `src/application/errors/` "are real application-service error types with tests — they are NOT shims and STAY." Only `citadel_error.rs` and `handoff_error.rs` are shims.

## REQ-shim-consumer-validation
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/prd-facade-crate-audit.md (Task 1.3, Goals 3-4, §7 `src/lib.rs` special case)
- description: Workspace-wide consumer search for every re-export shim, including each individual `lib.rs` re-export line.
- acceptance:
  - For every `re-export shim` file, the workspace MUST be searched via `grep -r` across `src/`, `crates/`, `tests/`, `examples/` and `benches/`
  - The search MUST look for the **re-exported path as it would appear at the call site** — both the module-qualified form (`crate::application::errors::citadel_error::CitadelError`, `paladin::application::errors::citadel_error::CitadelError`) and, for `lib.rs` crate-root elevations, the short form (`paladin::CitadelError`)
  - Results MUST be recorded in Appendix B as a consumer reference matrix: Shim File | Re-exported Path | Consumers (file:line) | Has Consumers?
  - Any shim with `Has Consumers? = No` MUST be added to List A; any with `Yes` MUST remain on List C with justification "active re-export shim — consumers exist"
  - `src/lib.rs` stays, but **each of its 30+ individual `pub use` re-export lines MUST be checked separately** and appear as its own row in Appendix B, keyed on the crate-root form; dead lines are flagged for Epic 2 removal
- scope: shim consumer matrix, lib.rs per-line audit, facade-audit.md Appendix B

## REQ-facade-audit-document
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/prd-facade-crate-audit.md (§6, §5, Goal 6, §8)
- description: The audit is read-only and delivered as a single combined markdown document that gates Epics 2-5.
- acceptance:
  - The document MUST be saved at `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` with sections: Summary; List A; List B (grouped by target crate); List C; Appendix A — Full Inventory Table; Appendix B — Consumer Reference Matrix
  - Prose sections serve as the human-readable decision record; appendices serve as the structured reference for Epics 2-5
  - **No file may be deleted, moved or modified during this Epic** — verified with `git status`, which should be clean or show only the new `facade-audit.md`
  - No file content refactoring, no import-path updates, no `cargo build` validation step, no auditing of files inside `crates/`, no auditing of non-`.rs` files
  - "This document directly gates Epics 2–5; no cleanup or rename work begins until the audit is complete and approved."
- scope: facade-audit.md deliverable, read-only constraint, Epic 2-5 gate

## REQ-dead-file-batch-deletion
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md (§4.1, Goals 1-2, §6, §7)
- description: Delete the 25 List A dead files in seven module-area batches, each independently verified and committed.
- acceptance:
  - Exactly **25** List A files MUST be deleted, in 7 batches totalling 3 + 4 + 3 + 5 + 4 + 4 + 2
  - Batch 1 — `src/application/notifications/` (3 files: `email_notifications.rs` 392 LOC, `push_notifications.rs` 0, `system_notifications.rs` 0); directory deleted; `application/mod.rs` needs no update (it never declared the module)
  - Batch 2 — `src/application/storage/` stubs (4: `file_store.rs` 6, `key_store.rs` 21, `key_value_store.rs` 13, `nosql_store.rs` 5); four `pub mod` lines removed from `storage/mod.rs`; `sql_store.rs` and `user_store.rs` stay
  - Batch 3 — `src/application/use_cases/content/` empty files (3: `content_list_ingestion_service.rs`, `content_list_service.rs`, `content_ml_analysis_service.rs`)
  - Batch 4 — `src/application/use_cases/subject/` (5 files) **plus cascade** deletion of `subject/mod.rs` and removal of `pub mod subject;` from `use_cases/mod.rs`, all in the same batch operation
  - Batch 5 — `src/core/platform/manager/admin/` (4 files, entire orphaned directory)
  - Batch 6 — `src/core/platform/manager/user/` (4 files, entire orphaned directory)
  - Batch 7 — infrastructure empty stubs (2: `logs/access_log_adapter.rs`, `notifications/push_notification_adapter.rs`) with their `mod.rs` declarations removed
  - After each batch: `rm` the files, remove any dangling `pub mod <name>;`, run `cargo build --workspace` and confirm success **before proceeding**, then commit the batch separately so `git bisect` can isolate regressions
  - After all batches: `cargo test --workspace` MUST pass; `cargo clippy --workspace -- -D warnings` MUST introduce zero new warnings
  - Net file reduction MUST be **26** (25 List A + 1 cascade `mod.rs`); `find src/ -name "*.rs" | wc -l` MUST read **163** (189 − 26)
  - **No deprecation stubs are required** — Epic 1 confirmed zero workspace consumers for all 25 files
  - Before deleting `email_notifications.rs`, confirm it does not contain logic needed by Epic 3; if it does, preserve a copy outside `src/` for reference
- scope: List A deletion, 7 batches, mod.rs cleanup, per-batch build gate
- note: Batch 1 conflicts with the M8 overview DOC, which requires the same three notification files **moved** to `paladin-notifications`. See INGEST-CONFLICTS.md WARNINGS.

## REQ-stale-application-ports-audit
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md (§4.2, Goal 3)
- description: A safety-check sweep for stale `application::ports::` import paths.
- acceptance:
  - Run `grep -rn "application::ports::" src/ crates/ tests/ examples/ benches/ --include="*.rs"`
  - Every match MUST be updated to the correct direct path (typically `paladin_ports::`, or the full facade path depending on context)
  - `cargo build --workspace` MUST succeed after all fixes
  - If no matches are found, the result MUST be documented as "confirmed zero stale `application::ports::` references"
  - `grep -r "application::ports::" src/` MUST return zero matches at completion
  - Context: Epic 1 confirmed `src/application/ports/` does **not** exist — it was removed in a prior milestone; this task is primarily a safety check
- scope: stale import audit, paladin_ports migration

## REQ-core-minimum-structure
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md (§4.3)
- description: Verify `src/core/` is reduced to exactly six legitimate files.
- acceptance:
  - `find src/core/ -name "*.rs" | sort` MUST confirm exactly these six files: `src/core/mod.rs` (bridge shim, 275+ workspace consumers); `src/core/platform/mod.rs` (bridge shim with Maneuver injection logic); `src/core/platform/manager/mod.rs`; `src/core/platform/manager/content_service.rs` (~385 LOC); `src/core/platform/manager/event_manager.rs` (~345 LOC); `src/core/platform/manager/user_service.rs` (~414 LOC)
  - `src/core/mod.rs` MUST still compile with valid re-exports (the deleted `admin/` and `user/` sub-trees were never referenced by it)
  - `src/core/platform/manager/mod.rs` MUST declare exactly three modules — `content_service`, `event_manager`, `user_service` — and nothing else
  - `cargo test --workspace` MUST confirm no regression
- scope: src/core/ minimum structure, bridge shims, manager services
- note: the three manager services are subsequently recorded as mis-layered and deferred — see `REQ-m8-deferred-items-register` item D2.

## REQ-libr-dead-reexport-removal
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md (§4.4, Goal 4, §6)
- description: Remove zero-consumer `pub use` aliases from `src/lib.rs` as a clean v0.2.0 API break.
- acceptance:
  - Approximately **50** `pub use` short-path aliases in `src/lib.rs` have zero workspace consumers (workspace code uses full module paths); all zero-consumer aliases MUST be removed
  - For v0.2.0 this is a **clean break** — no deprecated re-exports are needed since there are no known consumers
  - **Five exceptions MUST stay** (confirmed consumers): `pub use paladin_llm::mock::{MockLlmAdapter, MultiStepMockLlmPort};`, `pub use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig};`, `pub use paladin_llm::anthropic::{AnthropicAdapter, AnthropicConfig};`, `pub use paladin_llm::deepseek::{DeepSeekAdapter, DeepSeekConfig};` (13 consumers as a group), and `pub use core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};` (17 consumers of `paladin::Paladin`)
  - Module-level `pub mod` declarations MUST NOT be touched
  - Each removal MUST be preceded by confirming the type still exists at its source path — only the alias is removed, not the type
  - `STABLE_API.md` MUST be updated and `CHANGELOG.md` MUST gain a `### Removed` section under v0.2.0 listing each removed alias and its replacement
  - **Replacement paths in the CHANGELOG MUST reference stable crate-level locations** (`paladin_ports::`, `paladin_core::`, `paladin_battalion::`, `paladin_llm::`) rather than facade-internal paths (`paladin::application::use_cases::…`), because facade-internal paths change in Epic 4
- scope: src/lib.rs pub use aliases, STABLE_API.md, CHANGELOG.md v0.2.0 Removed
- note: "The removed `lib.rs` aliases represent the first intentional public API contraction for the project."

## REQ-notification-task-closeout
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§4.1, Goal 2)
- description: Formally close the notification relocation task as already resolved — documentation only.
- acceptance:
  - Task 3.1 MUST be marked `[x]` complete with a comment explaining that `src/application/notifications/` was removed in Epic 2 and that `src/infrastructure/adapters/notifications/mod.rs` already implements the correct dual re-export pattern (feature-gated crate re-exports when `notifications` is on; local fallback modules when off)
  - **No files may be moved, added or deleted** as part of this task — it is a documentation-only close-out
  - No changes to `src/infrastructure/adapters/notifications/` — "The dual re-export pattern is correct and deliberately preserved."
  - No changes to the `paladin-notifications` crate
- scope: Task 3.1 close-out, notifications dual re-export pattern
- note: the 2026-06-04 reconciliation execution log later **deletes** the dead notification fallback adapters (commit `cf17559`, ~1,072 LOC). See INGEST-CONFLICTS.md WARNINGS.

## REQ-storage-shim-deletion
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§4.2, Goal 1, §6, §7)
- description: Delete `src/application/storage/` and repoint its six consumers directly at `paladin_ports::`.
- acceptance:
  - Three files MUST be deleted: `src/application/storage/sql_store.rs`, `src/application/storage/user_store.rs`, `src/application/storage/mod.rs`
  - `pub mod storage;` MUST be removed from `src/application/mod.rs`
  - Six consumers MUST be updated with mechanical 1:1 import-path swaps: `sqlite_content_repository.rs` and `mysql_content_repository.rs` (`crate::application::storage::sql_store::{…}` → `paladin_ports::output::repository_port::{…}`); `sqlite_user_repository.rs` and `src/core/platform/manager/user_service.rs` (`…::user_store::UserRepositoryPort` → `paladin_ports::output::user_repository_port::UserRepositoryPort`); `src/config/setup/service_runner.rs` (`…::sql_store::MigrationManager` → `paladin_ports::output::repository_port::MigrationManager`); `tests/repository/mysql_content_repository_test.rs` (`paladin::application::storage::sql_store::ContentRepository` → `paladin_ports::output::repository_port::ContentRepository`)
  - **No `#[deprecated]` re-export may be added** — internal-only path with no public API contract; clean break only
  - `cargo build --workspace` and `cargo test --workspace` MUST pass after deletion and consumer updates
  - `CHANGELOG.md` MUST have a `### Removed` entry documenting the deletion with migration paths
  - Canonical homes of record: `paladin_ports::output::repository_port` owns `SqlStore`, `ContentRepository`, `ContentListRepository`, `MigrationManager`, `RepositoryError`, `RepositoryStats`, `TransactionManager`; `paladin_ports::output::user_repository_port` owns `UserRepositoryPort`
- scope: src/application/storage/ deletion, paladin_ports import repointing, CHANGELOG

## REQ-adapter-disposition-record
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§4.3 items 9, 10, 12, Goal 3)
- description: A written disposition for every adapter group under `src/infrastructure/adapters/`, cross-referenced to `facade-audit.md` List B.
- acceptance:
  - A disposition record MUST be written covering every adapter group, stating for each: **Decision** (stays in facade / extract to crate X / delete / flag for Milestone 9+), **Rationale** (one sentence), **Action required** (none / update imports / create crate / add feature gate), and **M9 extraction candidate?** (yes with target crate / no)
  - For every group flagged as an M9 candidate, the record MUST note the target crate **and the originating List B entry**, so Milestone 9 has a direct cross-reference back to `facade-audit.md`
  - "'Stays in facade' means *stays for Epic 3*; it does not override a List B extraction recommendation — it defers it."
  - Recorded M9 targets: `citadel/file_citadel.rs` (581 LOC) → `paladin-memory`; `document/` (`document_adapter.rs` 480, `pdf_extractor.rs` 350) → `paladin-content`; `file_storage/minio.rs` (1,198) → `paladin-storage`; `output/api_content_deliverer.rs` (**724 LOC**, corrected from 629) → `paladin-web`; `queue/redis.rs` (1,570) → `paladin-storage`; `tensorflow_adapter.rs` → future `paladin-ml`
  - Recorded as **not** M9 candidates: `arsenal/` (MCP wiring is composition-root responsibility), `herald/` (~1,900 LOC, no target crate exists), `logs/`, `llm/` config bridge, `paladin_registry.rs`, `scheduling/`
  - `garrison/` and `sanctum/` are optional consolidation candidates to fold into the M9 `paladin-memory` work
- scope: infrastructure adapter disposition record, M9 extraction flags, List B cross-reference
- note: the delivered `infrastructure-adapter-disposition.md` **disagrees with this PRD table** on `arsenal/` (record says M9 → future `paladin-arsenal`; PRD says No) and on `sanctum/` (record says future `paladin-sanctum`; PRD says fold into `paladin-memory`). See INGEST-CONFLICTS.md WARNINGS.

## REQ-tensorflow-ml-feature-gate-v2
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§4.3 item 11, Goal 4, §6, §7.3)
- description: Gate the TensorFlow adapter behind a new opt-in `ml` feature flag.
- acceptance:
  - `ml = []` MUST be added to `[features]` in the root `Cargo.toml` (under the `paladin-ai` package features)
  - The `tensorflow_adapter` module declaration in `src/infrastructure/adapters/input/mod.rs` MUST be wrapped in `#[cfg(feature = "ml")]`
  - A doc comment MUST be added to `tensorflow_adapter.rs` stating it requires `features = ["ml"]` and is a placeholder for a future `paladin-ml` crate (Milestone 9+)
  - `cargo build --workspace` **without** `--features ml` MUST exit 0 and MUST NOT compile `tensorflow_adapter.rs`
  - The `ml` flag follows the existing `redis-queue` / `s3-storage` / `notifications` pattern: opt-in, disabled by default
  - Rationale: "Speculative ML adapter (629 LOC); never wired in; should not compile by default"
- scope: ml feature flag, tensorflow_adapter.rs gating
- note: **v2** in a three-step chain — see `REQ-tensorflow-stays-facade-v1` and `REQ-deferred-tensorflow-ml-adapter-v3`, which removes the adapter and the flag entirely.

## REQ-garrison-sanctum-bridges-kept
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§4.3 item 13, §5, §6, §8 Q1-Q2)
- description: Garrison and Sanctum bridge shims are explicitly documented as staying, with consumer evidence.
- acceptance:
  - `src/infrastructure/adapters/garrison/mod.rs` and `src/infrastructure/adapters/sanctum/mod.rs` MUST NOT be deleted
  - The record MUST capture that Epic 1 Appendix B Section 1 audited both as **active multi-consumer** re-export bridges: garrison — `cli/config/loader.rs`, `infrastructure/mod.rs`, 4+ integration tests, 3 examples; sanctum — 3 integration tests (`rag_integration_tests.rs`, `in_memory_sanctum_tests.rs`, `qdrant_sanctum_tests.rs`) and 3 examples (`paladin_with_sanctum.rs`, `sanctum_basic_inmemory.rs`, `sanctum_configuration.rs`)
  - Both MUST be recorded as **optional** indirection-reduction candidates to fold into the Milestone 9 `paladin-memory` extraction — "explicitly **not** Epic 4, which is the unrelated `use_cases → services` rename"
  - Each MUST have a written "stays — active bridge" disposition with consumer evidence and an optional-consolidation note
  - Stated asymmetry rationale versus the storage shims: storage shims were internal-only paths never published in `STABLE_API.md`, targeted a single canonical port location making the fix a mechanical 1:1 swap, and had only internal `src/`/test consumers. Garrison and Sanctum re-export from `paladin_memory` **and** serve public-facing `examples/`, and garrison additionally exposes backward-compatible sub-module paths (`in_memory_garrison`, `sqlite_garrison`, `token_counter`)
- scope: garrison/mod.rs, sanctum/mod.rs, active bridge disposition

## REQ-m8-epic3-no-extractions
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md (§1 scope clarification v1.1, §5, §4.4, Goal 5)
- description: Epic 3 performs no crate extractions; every List B move is deferred to Milestone 9.
- acceptance:
  - "Epic 3 performs **no crate extractions.**" All 13 List B files MUST stay; every move is deferred to Milestone 9 and recorded as such in the disposition record
  - Herald, Citadel, Log, MinIO, Redis, the content adapters and the API content deliverer all stay in the facade
  - No new crates created — "`paladin-herald`, `paladin-ml`, etc. are not in scope"
  - No feature flag changes other than adding `ml`
  - No breaking changes to public API beyond what is documented in `CHANGELOG.md`
  - Quality gate: `cargo build --workspace` exits 0; `cargo test --workspace` passes with zero failures and zero new ignored tests; `cargo clippy --workspace -- -D warnings` reports zero warnings; `cargo fmt --all -- --check` exits 0; the task file shows all tasks `[x]`
- scope: Epic 3 no-extraction mandate, M9 deferral, quality gate
- note: directly reversed by the 2026-06-04 reconciliation, which executed the relocations and created `paladin-herald`. See `REQ-m8-reconciliation-relocations` and INGEST-CONFLICTS.md WARNINGS.

## REQ-m8-reconciliation-relocations
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/facade-cleanup-RECONCILIATION-2026-06-04.md (§3, §4 Categories 1-3, §5, §7 execution log)
- description: The Epic 3 relocations are executed after all, on the finding that the prior audit and disposition record contained factual errors.
- acceptance:
  - Category 1 — **orphaned dead files deleted outright** (~4,465 LOC, zero risk, not compiled at all): `document/document_adapter.rs` 480, `document/pdf_extractor.rs` 350, `input/file_content_fetcher.rs` 328, `input/file_content_list_fetcher.rs` 218, `input/http_content_fetcher.rs` 169, `input/local_file_fetcher.rs` 14, `input/news_api_fetcher.rs` 527, `output/api_content_deliverer.rs` 724, `logs/error_log_adapter.rs` 875, `repositories/mysql_content_repository.rs` 780
  - Verification method of record: "`rg \"mod <name>\"` across `src/` returns nothing for each; the `mod.rs` in each directory only does `pub use paladin_<crate>::…`; the leaf crate file exists"
  - Category 2 — compiled but redundant: `file_content_repository.rs` 723 deleted; `paladin_registry.rs` 418 consolidated into `paladin-battalion`; `sqlite_content_repository.rs` 810 and `sqlite_user_repository.rs` 676 resolved via the non-optional-storage change
  - Category 3 — genuine relocations executed: `file_storage/minio.rs` → `paladin-storage` (`s3` feature, crate bumped to edition 2024, facade `rust-s3` dep dropped); `queue/redis.rs` → `paladin-storage` (`redis-queue` feature, facade `redis` dep dropped); `citadel/file_citadel.rs` → `paladin-memory`; notification adapters → `paladin-notifications`; `infrastructure/web/user_controller.rs` → deleted as uncompiled residue (`paladin-web` already owns the live copy); Herald formatters → **new `paladin-herald` crate**
  - Category 5 hygiene: ~100 production `println!` converted to `log::*` in `services/` + `infrastructure/` (CLI output untouched); `application/mod.rs` and `infrastructure/mod.rs` docs refreshed; the 11 remaining `#[allow(dead_code)]` markers justified
  - Each phase MUST end with `cargo build && cargo test && cargo fmt --check && cargo clippy -- -D warnings` and a single conventional commit, stopping for go-ahead between phases
  - Final tally: **15 commits, ~10,250 net LOC removed, one new leaf crate**; build/tests/clippy/fmt green on default and across all rewired feature flags
  - In-execution corrections to the audit: `paladin_registry.rs` was **not** a duplicate (facade's 418-LOC impl is richer than battalion's 67-LOC `pub(crate)` copy — the richer one was consolidated *into* battalion); `sqlite_*_repository.rs` were **not** redundant in the default build; `mysql_content_repository.rs`, the `input/*` fetchers, `document/*`, `output/api_content_deliverer.rs` and `error_log_adapter.rs` **were** orphaned and uncompiled
  - "This doc explicitly supersedes `Epic_1/facade-audit.md` and `Epic_3/infrastructure-adapter-disposition.md`, which contain factual errors."
- scope: Category 1-3 deletions and relocations, paladin-herald creation, hygiene sweep
- settled-by: code-verification.md run-4 — `crates/paladin-herald/` ships with `json_herald.rs`, `markdown_herald.rs`, `table_herald.rs`; `crates/paladin-memory/src/citadel/file_citadel.rs` exists; `paladin-storage` has `s3` and `redis-queue` features; `println!` residue is exactly 17 across 6 files.

## REQ-use-cases-services-rename
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_4/prd-use_cases-services-rename.md (§4.1, §4.2, Goals 1-3, §6.1, §6.2)
- description: Rename `src/application/use_cases/` to `src/application/services/` and update all Rust references.
- acceptance:
  - The rename MUST use `git mv src/application/use_cases src/application/services` (not copy-delete) to preserve git history — a single atomic rename moving all 39 `.rs` files
  - `src/application/mod.rs` MUST change `pub mod use_cases;` to `pub mod services;`
  - All **286 Rust-file references** across `src/`, `tests/`, `examples/`, `benches/` MUST be replaced, covering `use crate::application::use_cases::`, `use paladin::application::use_cases::`, internal `mod.rs` doc-comment cross-references, and any struct field, type alias or identifier containing `use_cases`
  - `cargo build --workspace` MUST exit 0 with zero errors after replacement
  - `grep -r "use_cases" src/ tests/ examples/ benches/` (Rust files) MUST return **0 hits**
  - `find src/ -name "*.rs" | wc -l` MUST remain **160** (no files added or deleted — only moved)
  - Known hotspots requiring manual check after the automated pass: `src/application/mod.rs` (module declaration + 12+ doc-comment links), `src/lib.rs`, `src/config/setup/service_runner.rs`, `docs/QUICKSTART.md`, `README.md`, `STABLE_API.md`
  - Rationale of record: "In Domain-Driven Design, a 'use case' is an **AI agent or orchestration workflow** that a user composes from those services. The name therefore inverts the DDD vocabulary that the rest of the codebase follows."
- scope: src/application/use_cases → services, Rust import paths, git mv

## REQ-rename-clean-break
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_4/prd-use_cases-services-rename.md (§4.1.3, Goal 6, §5, §6.4, §6.5)
- description: The rename is a clean break with no backward-compatible re-export.
- acceptance:
  - "No `pub use services as use_cases;` re-export must be added — this is a clean break."
  - "**No re-exports or deprecation shims.** Task 4.3 from the Epic spec is explicitly rejected"
  - All consumers MUST be updated in the same commit
  - No semantic refactoring — the internal logic of any service file MUST NOT change; this is a pure rename
  - Service struct names (`PaladinExecutionService`, `FormationExecutionService`, …) MUST keep their current names — only the module path changes
  - No changes to files under `project/`; no changes to `crates/` leaf crates (audit and address only if `use_cases` strings are found, none expected); no Milestone 9 extraction
  - `paladin::application::use_cases::*` is part of the stable public API documented in `STABLE_API.md`; the rename is a **breaking change** (semver minor bump deferred to release)
  - No `Cargo.toml` dependency, feature flag or crate name changes are required
- scope: clean break, no deprecation shim, no semantic refactor
- note: the M8 overview DOC Epic 4 Task 4.3 offers the opposite ("Add Backward-Compatible Re-Export (Optional)" with `#[deprecated]`). PRD outranks DOC. See INGEST-CONFLICTS.md INFO.
- note: the "no changes to `crates/` leaf crates" scope decision is what left `paladin-content` broken — see `REQ-paladin-content-services-rename`.

## REQ-rename-doc-updates
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_4/prd-use_cases-services-rename.md (§4.3, Goal 4, §4.5 item 10, §8)
- description: Replace `use_cases` across user-facing documentation.
- acceptance:
  - All **57 markdown references** MUST be replaced in: all `.md` files under `docs/`, `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `STABLE_API.md`
  - Files under `project/` are **explicitly excluded** from this pass
  - `grep -r "use_cases" docs/ README.md CHANGELOG.md CONTRIBUTING.md` MUST return 0 hits, excluding the CHANGELOG migration table itself which documents the old name
  - `grep "use_cases" STABLE_API.md` MUST return 0 hits
  - `api_surface_current.txt` and `final-api.txt` are **not** updated in this Epic — regeneration is deferred to the release-gate step
- scope: docs/, README, CHANGELOG, CONTRIBUTING, STABLE_API.md

## REQ-rename-changelog-breaking
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_4/prd-use_cases-services-rename.md (§4.4, Goal 5)
- description: A mandatory CHANGELOG breaking-change entry with a full migration table.
- acceptance:
  - `CHANGELOG.md` MUST gain a `### Breaking Changes` entry under `[Unreleased]` with the one-line description "`src/application/use_cases/` renamed to `src/application/services/`"
  - It MUST include a migration table with old path, new path and a one-line module description for each of eleven sub-modules: `paladin`, `battalion`, `arsenal`, `content`, `herald`, `orchestration`, `log_orchestrator`, `notification_orchestrator`, `queue_orchestrator`, `sanctum`, `analysis` — each mapping `paladin::application::use_cases::<x>::*` → `paladin::application::services::<x>::*`
- scope: CHANGELOG breaking change, migration table

## REQ-facade-role-lib-docs
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-1, FR-3, Goal 1, §6)
- description: Extend the `src/lib.rs` crate documentation with a Facade Crate Role section.
- acceptance:
  - The existing `//!` doc comment MUST be **extended, not replaced**, with a new `## Facade Crate Role` section
  - It MUST contain a one-paragraph explanation that the crate is the **application assembly point and composition root** for the Paladin workspace
  - It MUST state what the facade contains: `ServiceRunner` (the composition root), application-layer coordination services (`src/application/services/`), configuration loading (`src/config/`), CLI modules (`src/application/cli/`, feature-gated), and binary entry points (`main.rs`, `bin/paladin-cli.rs`)
  - It MUST state what the facade does **not** contain: business logic, port trait definitions, or infrastructure adapter implementations
  - It MUST list the leaf crates and their capabilities: `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-notifications`, `paladin-storage`, `paladin-content`, `paladin-web`
  - The `## Architecture` section MUST update the `Application Layer` description from "Use cases and port trait definitions" to "Application services and coordination logic"
  - Target `##` heading order: (implicit title), `## Core Concepts`, `## Facade Crate Role` (new), `## Architecture`, `## Stable Public API`, `## Quick Start`, `## Feature Flags`
  - Edit strategy: insert the new section between `## Architecture` and `## Stable Public API`; do not rewrite the whole file
- scope: src/lib.rs //! docs, facade role, leaf crate list
- note: the nine-crate leaf list omits `paladin-herald`, which did not exist when this PRD was written.

## REQ-facade-readme
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-2, §6, §7)
- description: A new `src/README.md` for human readers browsing the source.
- acceptance:
  - `src/README.md` MUST be created with a `# Paladin Facade Crate` heading
  - It MUST contain a prose description of the facade role (assembly point, composition root, `ServiceRunner`, application services, CLI, binaries)
  - It MUST include a "What lives here" table with columns **Path**, **Purpose**, **Notes**, covering at minimum `src/application/services/`, `src/application/cli/`, `src/config/`, `src/infrastructure/`, `src/core/`, `src/bin/`, `src/main.rs`
  - It MUST explain the dependency-flow rule: facade → leaf crates, one direction only; leaf crates must not import from the facade
  - It MUST reference `STABLE_API.md` for the public API contract
  - Rationale of record: "Rust's `cargo doc` does not automatically include `src/README.md`; it is for human readers browsing the source. The `//!` docs in `lib.rs` serve the `cargo doc` audience. Both are needed"
- scope: src/README.md, What-lives-here table, dependency-flow rule

## REQ-stable-api-v020-sync
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-4 to FR-8, Goal 2, §7)
- description: Comprehensive audit and update of `STABLE_API.md` against the post-Milestone-8 workspace.
- acceptance:
  - Header block updates: `Version:` → `0.2.0`; `Last Updated:` → `2026-05-30`; `Epic:` → `Milestone 8, Epic 5 - Document Facade Crate Role and Finalize`
  - The breaking-change callout box MUST accurately reflect the current v0.2.0 breaking changes (shim removals from Epics 2/3 **and** the `use_cases` → `services` rename from Epic 4)
  - The `### paladin (facade crate)` per-crate section MUST reflect the post-M8 module layout: no `application/ports/`, no `application/storage/`, `application/services/` instead of `application/use_cases/`
  - Every Stable Public API Catalog item referencing a `use_cases` path segment MUST become `services`
  - Catalog items referencing modules deleted in Milestone 8 (`application::storage::sql_store`, `application::ports::*` shim paths) MUST be removed or annotated as removed
  - The `## Tracking API Changes` section MUST reference `api_surface_current.txt` as the v0.2.0 baseline
  - Audit method: read the current sections, cross-reference against `find src/ crates/ -name "*.rs" | sort`, flag stale paths; **do not delete the stability-tier prose sections** — update only the catalog table entries and header
  - `STABLE_API.md` MUST have zero `use_cases` path references at completion
  - **No API surface changes** — anything found genuinely missing MUST be flagged in Open Questions rather than silently added
- scope: STABLE_API.md audit, v0.2.0 header, catalog sync

## REQ-changelog-v020-cut
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-9 to FR-12, Goal 3, §7, §9 Q3)
- description: Promote `[Unreleased]` to a formal `## [0.2.0]` release section.
- acceptance:
  - The `## [Unreleased]` block MUST be promoted to `## [0.2.0] - 2026-05-30` in Keep-a-Changelog format, with a fresh empty `## [Unreleased]` section inserted above it
  - The `## [0.2.0]` section MUST contain all four sub-sections in this order: `### Breaking Changes`, `### Added`, `### Changed`, `### Removed`, with the accumulated entries redistributed correctly
  - A new `### Changed` entry MUST document this Epic's work: "Documented facade crate role as application assembly point; added `src/README.md` and updated `src/lib.rs` `//!` docs."
  - A `[0.2.0]: <compare URL>` link entry MUST be added to the bottom link-reference block, using the pattern `https://github.com/DF3NDR/paladin-dev-env/compare/v0.1.0...v0.2.0` — confirm the `v0.1.0` tag exists before adding
- scope: CHANGELOG.md v0.2.0 promotion, Keep a Changelog, compare URL

## REQ-api-surface-baseline-v020
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-13, FR-14, Goal 5, §7, §9 Q1)
- description: Regenerate the API-surface baseline artifacts as the v0.2.0 reference.
- acceptance:
  - `api_surface_current.txt` MUST be regenerated using the workspace's public API extraction method documented in `STABLE_API.md` §Automated Tracking; the file header comment MUST note `v0.2.0 baseline — 2026-05-30`
  - `final-api.txt` MUST be updated to reflect the same v0.2.0 snapshot
  - Extraction method: check `STABLE_API.md` §Automated Tracking or the `Makefile` for the exact command; if `cargo public-api` or similar, use it; if no tooling is available, use `cargo doc --workspace --no-deps 2>&1` output — and document which method was used in the file header
  - Open question 1: confirm `cargo-public-api` is installed in the dev container before starting, and fall back if not
- scope: api_surface_current.txt, final-api.txt, v0.2.0 baseline

## REQ-m8-final-quality-gate
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md (FR-15 to FR-19, Goal 4, §5, §9 Q4)
- description: The closing quality gate for Milestone 8.
- acceptance:
  - `cargo build --workspace` — exit 0, zero errors
  - `cargo test --workspace` — all tests pass, zero failures
  - `cargo clippy --workspace -- -D warnings` — zero warnings
  - `cargo fmt --all -- --check` — exit 0, no formatting drift
  - `cargo doc --workspace --no-deps` — exit 0 (warnings acceptable; must not fail)
  - **No logic changes** — Epic 5 is documentation-only for `src/lib.rs`, `src/README.md`, `STABLE_API.md`, `CHANGELOG.md`; no new Rust modules, structs, traits or feature flags
  - Leaf-crate documentation is out of scope — "those belong to Milestone 11 (Documentation Overhaul)"
  - No merge to `main` and no version tag — the branch is left as a release candidate
  - Open question 4: confirm `src/lib.rs.backup` should be deleted before the final gate ("it is not a `.rs` module file and should not be committed")
- scope: M8 quality gate, documentation-only scope, release-candidate branch

## REQ-paladin-content-services-rename
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/prd-paladin-content-use_cases-services-rename.md (FR-1 to FR-6, Goals 1-3, §7.1)
- description: Rename `paladin-content`'s `use_cases` module to `services`, closing six latent E0432 errors in the facade bridge.
- acceptance:
  - `crates/paladin-content/src/use_cases/` MUST be renamed to `crates/paladin-content/src/services/` using `git mv`; `use_cases/` MUST NOT exist afterwards; `git status` MUST show `renamed:` not `deleted:` + `untracked:`
  - Exactly thirteen files MUST be present in `services/` after the rename, no additions and no deletions: `content_{aggregator,analysis,delivery,fetching,filtering,list_fetching,list_ingestion,list,llm_analysis,ml_analysis,nlp_analysis,summarizer}_service.rs` plus `mod.rs`
  - `crates/paladin-content/src/lib.rs` MUST change `pub mod use_cases;` to `pub mod services;`
  - The crate-level `//!` doc comment MUST NOT reference `use_cases` — "Content processing adapters and use-case services…" becomes "…and application services…"
  - Five `crate::use_cases` occurrences across four files MUST become `crate::services`: `adapters/input/http_content_fetcher.rs` (1), `adapters/input/file_content_list_fetcher.rs` (1), `adapters/input/news_api_fetcher.rs` (2), `services/content_llm_analysis_service.rs` (1)
  - `grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"` MUST return zero results; `grep -rn "use_cases" crates/paladin-content/` MUST return zero results
  - Root cause of record: "The Epic 4 rename scope was defined as the facade crate only (`src/`) … the facade's re-export bridge was updated to the new path, but `paladin-content` still publishes the old path, leaving the bridge broken under the `content-processing` feature flag."
  - Public API change is limited to the module path (`paladin_content::use_cases::*` → `paladin_content::services::*`); because `paladin-content` is workspace-internal with no independent crates.io release, this is **not** a semver-breaking change
- scope: paladin-content services rename, lib.rs, internal crate:: references, E0432
- note: the 2026-06-04 reconciliation records this Epic as "Not verified; low priority" with no execution-log entry. The tree shows it complete. See INGEST-CONFLICTS.md INFO.

## REQ-paladin-content-readme-update
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/prd-paladin-content-use_cases-services-rename.md (FR-7, Goal 4)
- description: Update the `paladin-content` README to the `services` vocabulary.
- acceptance:
  - `crates/paladin-content/README.md` MUST replace all `use_cases` references in prose, module descriptions, import examples and type-name examples with `services`
  - Known occurrences: the module description line; the `use paladin_content::use_cases;` import example → `use paladin_content::services;`; any `use_cases::` path prefix in code examples → `services::`
- scope: crates/paladin-content/README.md

## REQ-paladin-content-changelog-fix
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/prd-paladin-content-use_cases-services-rename.md (FR-8, Goal 7)
- description: A `fix:` CHANGELOG entry describing the patch.
- acceptance:
  - A `### Fixed` sub-section entry MUST be added under `## [Unreleased]` in `CHANGELOG.md`
  - It MUST describe: the `use_cases` → `services` rename inside `paladin-content`; resolution of six `E0432 unresolved import` errors in the facade's `content/mod.rs` re-export bridge; and that the errors were previously masked by the `content-processing` feature gate
- scope: CHANGELOG.md Unreleased Fixed entry

## REQ-content-processing-build-gate
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/prd-paladin-content-use_cases-services-rename.md (NFR-1 to NFR-7, Goals 5-6, §7.3)
- description: Verify the workspace under both the default feature set and `content-processing`.
- acceptance:
  - `cargo build -p paladin-content` MUST exit 0 after the internal reference updates, before any workspace-level build
  - `cargo build --workspace` MUST exit 0 under the default feature set
  - `cargo build --workspace --features content-processing` MUST exit 0, confirming the six previously-broken `E0432` errors are resolved (`… 2>&1 | grep -E "^error"` must produce zero output)
  - `cargo test --workspace` and `cargo test --workspace --features content-processing` MUST both exit 0
  - `cargo clippy --workspace -- -D warnings` MUST exit 0 with no new warnings introduced by the rename
  - `cargo fmt --all -- --check` MUST exit 0
  - Out of scope: auditing other leaf crates for surviving `use_cases` modules; any change to the facade's `content/mod.rs` or `content_ingestion_service.rs`; `STABLE_API.md` (paladin-content paths are workspace-internal); regenerating `api_surface_current.txt` or `final-api.txt`
- scope: content-processing feature gate verification, quality gate

## REQ-delivery-endpoints-axum
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 1-3, Goal 2, §6, §7)
- description: Reimplement the three orphaned actix content-delivery endpoints as served axum routes.
- acceptance:
  - `POST /api/delivery/deliver` — body `DeliveryRequest` (JSON) → `ApiContentDeliverer::deliver_content_async`; `Ok` → `200 OK` with `DeliveryResponse` JSON; `Err` → `400 Bad Request` with `{ "error": "<message>" }`
  - `GET /api/delivery/status/{delivery_id}` — path param UUID string → `ApiContentDeliverer::get_delivery_status`; `Ok` → `200 OK` with `DeliveryResponse` JSON; `Err` → `404 Not Found` with `{ "error": … }`; unparseable UUID → `400 Bad Request` with `{ "error": "Invalid delivery ID format" }`
  - `GET /api/delivery/stats` — `ApiContentDeliverer::get_delivery_stats(None)`; `Ok` → `200 OK` with `DeliveryStats` JSON; `Err` → `500 Internal Server Error` with `{ "error": … }`
  - A public route-builder MUST be exposed following the existing convention, e.g. `pub fn create_delivery_routes(deliverer: Arc<ApiContentDeliverer>) -> axum::Router`, using an axum `State<Arc<ApiContentDeliverer>>` extractor for dependency injection
  - The delivery routes MUST be **mounted into the application router** so they are served alongside the user-management routes; the chosen approach MUST keep the existing user routes and their auth middleware unchanged
  - Endpoint parity: keep the exact paths and JSON shapes — "a revival, not a redesign"
  - State injection: actix used `web::Data<ApiContentDeliverer>`; the axum equivalent is `State<Arc<ApiContentDeliverer>>` (`ApiContentDeliverer` is already `Clone` with `Arc<Mutex<…>>` internals)
- scope: /api/delivery/* axum routes, create_delivery_routes, application router mounting

## REQ-actix-removal
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 4-7, Goals 1, 3, §8)
- description: Remove actix-web from `paladin-web` so the crate depends on exactly one HTTP framework.
- acceptance:
  - The actix `configure()` function, the three actix handler functions, and `use actix_web::{HttpResponse, Result as ActixResult, web};` MUST be removed from `api_content_deliverer.rs`
  - `actix-web` MUST be removed from `crates/paladin-web/Cargo.toml` dependencies
  - The `ApiContentDeliverer` struct and its `ContentDeliveryService` / `BatchContentDeliveryService` / reqwest behavior MUST NOT be modified beyond what is needed to call its existing public methods from the new axum handlers
  - The `crates/paladin-web/src/lib.rs` crate-level doc comment MUST state the crate uses **axum**, removing the "actix-web and axum" wording
  - `rg actix crates/paladin-web/` MUST return **zero** matches in source and `Cargo.toml`
  - `cargo tree -p paladin-web` MUST no longer list `actix-web`
  - No regression: the existing axum user-management API and the `ApiContentDeliverer` service behave exactly as before; all workspace tests pass
  - Problem of record: the actix handlers are "orphaned — nothing in the workspace ever starts an actix `HttpServer`, and `configure()` is never called … The dependency pulls an entire second async-HTTP framework into the build solely to compile dead endpoints"
- scope: actix-web removal, api_content_deliverer.rs, paladin-web Cargo.toml, lib.rs docs
- note: **v2** of a competing pair — `REQ-paladin-web-extraction` (M7 Epic 1 §4.2.1) requires actix-web as a direct non-optional dependency of `paladin-web`. Both preserved. See INGEST-CONFLICTS.md WARNINGS.
- settled-by: code-verification.md run-4 — zero `actix` matches anywhere under `crates/paladin-web/`.

## REQ-actix-deny-ban
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 8, Goal 4, §7, §9 Q3)
- description: A cargo-deny guardrail preventing a second web framework from returning.
- acceptance:
  - `actix-web` MUST be added to the banned-crates list in `deny.toml` with a short rationale, under `[bans] deny`
  - `make deny` / the CI dependency-policy job MUST fail if `actix-web` is reintroduced — verified once
  - Open question 3: ban only `actix-web`, or all `actix-*` framework crates? Default `actix-web`; widen if the team wants a hard framework-level guard
- scope: deny.toml bans, make deny, CI dependency policy

## REQ-delivery-handler-tests
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 9, §8)
- description: Unit tests for the three new axum delivery handlers.
- acceptance:
  - Unit tests MUST cover: a successful response; the error/`404` path for an unknown delivery id; and the `400` path for an invalid UUID
  - They MUST mirror the test style already used in `user_controller.rs`
  - The three delivery endpoints MUST be present in the mounted router and covered by passing unit tests
- scope: delivery handler unit tests

## REQ-web-api-baseline-changelog
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 10, Goal 5, §7)
- description: Regenerate the public API-surface baseline and record the change in the CHANGELOG.
- acceptance:
  - The public API-surface baseline MUST be regenerated via `./scripts/extract-public-api.sh project/current-exports.txt`
  - A `CHANGELOG.md` `[Unreleased]` entry MUST describe the framework consolidation and the `paladin-web` public-API change (removal of the actix `configure`/handlers, addition of the axum route-builder)
  - The `API Surface Tracking` CI job MUST pass with the regenerated baseline
  - Rationale: "There are no external consumers, so this is acceptable; document it as a change."
- scope: current-exports.txt baseline, CHANGELOG Unreleased, API Surface Tracking CI job
- note: `project/current-exports.txt` is a **stale path** — the directory was renamed to `.project/` in commit `928c6d5`. This is the same defect run-3 verification recorded against `scripts/check-api-surface.sh` and `ci.yml:171,181,186`. See INGEST-CONFLICTS.md WARNINGS.

## REQ-web-quality-gate
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md (FR 11, §5, §7)
- description: Epic 7 quality gate across the default and `web-server` feature sets.
- acceptance:
  - The workspace MUST build, lint (`cargo clippy -- -D warnings`), format-check and test cleanly on the default feature set **and** with the `web-server` feature enabled
  - CI MUST be green including the `web-server` build/test matrix entry
  - All changes MUST be confined to `crates/paladin-web/` plus `deny.toml`, `project/current-exports.txt` and `CHANGELOG.md` — "No facade (`src/`) source changes are expected"
  - Out of scope: auth on the delivery endpoints (the original actix handlers had none; parity preserved); changing delivery data models, the `ContentDeliveryService` port, or `ApiContentDeliverer` delivery/retry/scheduling logic; migrating other crates; a shared HTTP-server abstraction; OpenAPI/schema generation, rate limiting, new delivery features; wiring the `paladin-web` server into binary runtime startup
  - `axum 0.8` is already a dependency; no new dependency should be required. After removal, confirm `actix-web` and its now-unused transitive deps drop out of `Cargo.lock`
- scope: Epic 7 quality gate, web-server feature matrix, change confinement

## REQ-rustsec-risk-acceptance
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md (Objective, Blocking Findings, Exception governance, Acceptance Criteria)
- description: Formal risk acceptance for two unfixable RustSec advisories blocking the Epic 4 release candidate.
- acceptance:
  - **`RUSTSEC-2023-0071`** — `rsa 0.9.10`, Marvin timing side-channel. Dependency path `rsa -> sqlx-mysql -> sqlx -> workspace crates`. Status: **no fixed upgrade available**
  - **`RUSTSEC-2025-0111`** — `tokio-tar 0.3.1`, PAX header parsing enabling file smuggling. Dependency path `tokio-tar -> testcontainers -> testcontainers-modules`. Status: **no fixed upgrade available**
  - Acceptance criteria: either the vulnerabilities are eliminated from the release dependency graph, **or** formal risk acceptance is documented with owner, expiry date, affected scope, compensating controls, and a tracked follow-up issue
  - Exception governance enforced: `make audit` runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`; the CI security job enforces the same command "to block newly introduced vulnerabilities while allowing only approved exceptions"
  - **Exception owner: Platform Security (Milestone 7). Exception review/expiry target: 2026-09-30** (or earlier if upstream fixes become available)
  - Hardening boundaries required: `RUSTSEC-2023-0071` — ensure no direct RSA private-key decrypt/sign operations are exposed in runtime paths; document dependency-only exposure if true. `RUSTSEC-2025-0111` — ensure untrusted tar extraction is not performed in production runtime paths; limit `testcontainers` usage to test-only contexts
  - Exit evidence required: updated audit output; updated Epic 4 release readiness report; Task 5.6 status updated; CI workflow enforcing RustSec checks with explicit exception IDs
- scope: RUSTSEC-2023-0071, RUSTSEC-2025-0111, cargo audit exceptions, audit.toml, CI security job
- note: **live security acceptances.** The recorded expiry is 2026-09-30. `cargo audit` "still reports both advisories because Cargo.lock includes dev/optional dependency graphs and no fixed upstream versions are available." The tree now carries **five** vulnerability ignores, not two, and the three files that encode them disagree. See INGEST-CONFLICTS.md WARNINGS.

## REQ-rustsec-hardening-actions
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md (Progress Update 2026-05-28, Track A, Track B, Action Plan)
- description: Concrete dependency-surface hardening performed alongside the risk acceptance, plus the still-open action items.
- acceptance:
  - Completed: `testcontainers-modules` moved from normal dependencies to `dev-dependencies` in the root manifest
  - Completed: MySQL repository compilation tightened in `src/infrastructure/repositories/mod.rs` so MySQL module paths are only present when `storage-mysql` is enabled
  - Completed: `sqlx` default features disabled at workspace level with required features listed explicitly (`sqlite`, `migrate`, etc.) to reduce implicit backend activation
  - Validation snapshot: `cargo check` passes; `cargo tree -i tokio-tar` shows only dev-dependency paths through `testcontainers`
  - **Open action items** (recorded, not marked complete): create issue "Epic 4 Security: RUSTSEC-2023-0071 impact analysis and mitigation"; create issue "Epic 4 Security: RUSTSEC-2025-0111 testcontainers/tokio-tar mitigation"; add `audit.toml` exception entries only if approved, each with expiry date and owner; re-run `cargo audit` after mitigation changes and attach evidence to the Epic 4 report
  - Track A also requires: constraining the feature/build surface for release artifacts, confirming whether `sqlx-mysql` is required in the default release path and preferring a SQLite-only release profile where acceptable
  - Track B also requires: investigating `sqlx-mysql` alternatives and a migration path that removes `rsa` when upstream fixes land; ensuring release builds and published crates do not require the `tokio-tar` transitive chain; subscribing to upstream advisory trackers for `rsa`, `sqlx`, `tokio-tar`, `testcontainers`
- scope: dev-dependency isolation, storage-mysql gating, sqlx default-features, open security follow-ups

## REQ-license-policy-signoff
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md (Target Policy, Decision Checklist, Go/No-Go Gate)
- description: Recorded dependency-licensing policy and Task 5.7 sign-off evidence.
- acceptance:
  - Project licensing model of record: **`MIT OR Apache-2.0`** (Rust-style dual licensing)
  - Approval rule: licenses that are MIT, Apache-2.0, or an SPDX expression containing a permissive MIT/Apache branch are acceptable by default; non-permissive or unresolved entries require explicit decision and sign-off
  - Inventory: 551 packages, **0** unknown-license entries after resolution
  - `MPL-2.0` entries (`colored 2.2.0`, `colored 3.0.0`) are **explicitly accepted for unmodified use**; replacement of the `colored` dependency chain is therefore N/A
  - `r-efi 5.3.0` (`MIT OR Apache-2.0 OR LGPL-2.1-or-later`) accepted via its permissive MIT/Apache branch
  - `fuchsia-cprng 0.1.1` resolved from the crates.io artifact (`LICENSE` file plus `license-file = "LICENSE"` in `Cargo.toml`) as BSD-3-Clause-style permissive; no longer treated as unknown
  - Policy approver of record: **`DF3NDR` (repository owner), approval date 2026-05-28**
  - Task 5.7 gate: policy approval recorded; MPL-2.0 has an explicit accept-or-replace decision; unknown entries resolved or replaced; the Epic 4 report and task list updated with sign-off evidence. **Status: COMPLETE**
- scope: MIT OR Apache-2.0 policy, MPL-2.0 acceptance, license inventory sign-off
- note: this dual-license policy conflicts with the M7 Epic 4 PRD §4.7.7, which frames the requirement as "compatible with the project's MIT licensing posture", and with the M7 overview AC 1, which specifies `license (MIT)`. The shipped root `Cargo.toml` declares `license = "MIT"`. See INGEST-CONFLICTS.md WARNINGS.

## REQ-m8-deferred-items-register
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md (D1-D5, Suggested grouping)
- description: Five deliberately deferred facade-cleanup items, verified against `main` on 2026-06-07 — the authoritative Milestone 8 forward-work register.
- acceptance:
  - **D1 — `src/core/` re-export shims (KEEP, by decision).** `src/core/mod.rs` and `src/core/platform/mod.rs` re-export `paladin_core::*` plus the `platform/mod.rs` battalion `maneuver`/`parser` path injection. **~49 facade files** still import via `crate::core::…`. Deferred because removal means rewriting those ~49 files to `paladin_core::` / `paladin_battalion::` paths — high churn, low functional value, no behavior or public-API change. If pursued: mechanical path rewrite plus shim deletion; **verify `platform/mod.rs`'s maneuver/parser injection is preserved (it carries real logic, not just re-exports)**. Effort/risk: medium churn / low risk
  - **D2 — `src/core/platform/manager/` services are mis-layered.** `content_service.rs`, `event_manager.rs`, `user_service.rs` are application/domain services, not facade composition glue. Recommendation from the Epic 1 audit: `content_service.rs` (`ContentItemService`, pure domain) → `paladin-core`; `event_manager.rs` (`EventService`) → `paladin-core` or a facade app-service module; `user_service.rs` → **split**, trait + DTOs → `paladin-core`/`paladin-ports`, concrete impl (depends on repo/log/notification ports + argon2) → a facade app-service. Effort/risk: medium / medium
  - **D3 — entangled Paladin use-case services (KEEP for now).** `src/application/services/paladin/{planning_service, prompt_generation_service, temperature_service, handoff_service}.rs` (~2,750 LOC), tightly coupled to `paladin_builder.rs` and `paladin_execution_service.rs`. Candidates for `paladin-battalion` (planning/handoff) and `paladin-llm` (prompt/temperature), but "moving them safely needs the builder/execution coupling untangled first". Revisit only alongside a builder/execution refactor. Effort/risk: high / high
  - **D4 — `content_ingestion_service.rs` placement.** `src/application/services/content/content_ingestion_service.rs` (~1,211 LOC) arguably belongs in `paladin-content`, but it orchestrates across several facade services; a move needs a dependency-coupling review first. Effort/risk: medium / medium
  - **D5 — residual `println!`/`eprintln!`/`dbg!`.** 17 occurrences across 6 files in `src/application/services/` + `src/infrastructure/`, down from ~435. Review the 6 files; convert genuine diagnostics to `log::*`, keep intentional stdout output. Effort/risk: low / low
  - Suggested grouping: quick wins → D5; architecture pass (one focused milestone) → D2 plus optionally D4; only with a broader refactor → D3, and D1 if a "no re-export aliases" policy is adopted
  - Status framing of record: "Record of intentional non-goals (not bugs / not oversights)"
- scope: D1-D5 deferred register, src/core shims, manager services, Paladin services, content ingestion, println residue
- note: no owners are named and no target milestone is assigned beyond the suggested grouping.
- settled-by: code-verification.md run-4 — D5's count is exact: 17 occurrences across the 6 files. D1's six `src/core/` files and D2's three manager services all still ship.

## REQ-deferred-cli-user-commands
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md (§1)
- description: The CLI user-management command surface, removed from the facade and recorded for deliberate reintroduction.
- acceptance:
  - Removed: `src/application/cli/commands/user.rs` (**1,065 LOC**) and its `pub mod user;` declaration in `src/application/cli/commands/mod.rs`
  - Status when removed: "declared but **never dispatched** from the CLI binary (`src/bin/paladin-cli.rs`) — no `UserCommands` arm existed in the top-level command match, so the subcommands were unreachable. It compiled but did nothing."
  - Backend is intact and in use elsewhere: `core::platform::manager::user_service::{UserService, UserServiceTrait, UserRegistrationRequest, UserLoginRequest, UserProfileUpdateRequest}` and `core::platform::container::user::{User, UserProfile}` — "Re-implementing the CLI surface is therefore mostly re-wiring, not new domain work."
  - Intended clap subcommand surface: `register` (username, email, password, first/last name, bio, timezone, locale); `login` (email, password); `get` (user id or email); `update` (user id, username, email, first/last name, bio, avatar URL); `list` (filter by active, filter by verified, limit); `activate` (user id); `deactivate` (user id); `verify` (user id)
  - To reintroduce: add a `User(UserCommands)` arm to the CLI's top-level command enum plus a dispatch handler constructing `UserService` (see `config/user_config.rs` for the existing DI wiring) and calling the matching `UserServiceTrait` method; add command tests under `src/application/cli/tests/`; **recover the original module from git history (the Milestone 8 removal commit on branch `chore/facade-cleanup-m8-finish`) rather than rewriting from scratch**
- scope: paladin user CLI commands, UserService, CLI dispatch wiring
- settled-by: code-verification.md run-4 — `src/application/cli/commands/` contains ten command modules and `user.rs` is not among them.

## REQ-deferred-tensorflow-ml-adapter-v3
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md (§2)
- description: The TensorFlow ML adapter and the `ml` feature flag are removed entirely; reintroduction is conditioned on a dedicated `paladin-ml` crate.
- acceptance:
  - Removed: `src/infrastructure/adapters/input/tensorflow_adapter.rs` (**636 LOC**), its `#[cfg(feature = "ml")] pub mod tensorflow_adapter;` declaration in `src/infrastructure/adapters/input/mod.rs`, and the now-unused `ml = []` feature flag in `Cargo.toml`
  - Status when removed: "An explicit `#[doc(hidden)]` placeholder for a future `paladin-ml` crate (Milestone 9+). It implemented `paladin_ports::input::ml_port::MlPort` but contained no real TensorFlow integration — model loading/prediction were stubs, with `#[allow(dead_code)]` on unused fields. Gated behind the non-default `ml` feature; nothing consumed it."
  - Intended shape: `TensorFlowAdapter` implementing `MlPort` (`load_model` / `predict` / `model_info`), translating `MlPredictionRequest` → TensorFlow ops → `MlPredictionResponse`, keyed by a `model_path` and a registry of loaded models
  - **Reintroduction condition:** the port contract `paladin_ports::input::ml_port::MlPort` remains in the workspace so the integration point is stable. The real adapter MUST be implemented in a dedicated `paladin-ml` **leaf crate** — "consistent with the hexagonal layout — ML inference is an infrastructure adapter, not facade code — rather than re-adding it to the facade." Re-add an `ml`/provider feature flag on that crate at that time
  - Both removed modules are recoverable verbatim from git history at the Milestone 8 removal commit on branch `chore/facade-cleanup-m8-finish`
- scope: tensorflow_adapter.rs removal, ml feature removal, future paladin-ml leaf crate
- note: **v3**, the terminal state of the chain `REQ-tensorflow-stays-facade-v1` → `REQ-tensorflow-ml-feature-gate-v2` → this entry.
- settled-by: code-verification.md run-4 — no `tensorflow` reference and no `ml` feature exist anywhere in `Cargo.toml` or `src/`.

## REQ-paladin-ports-publish-verification-closed
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md
- description: A deferred Epic 4 Task 5.5 publish blocker, now recorded as Resolved.
- acceptance:
  - Original deferred scope: `cargo publish --dry-run -p paladin-ports` verification failure (Task 5.5)
  - **Status: Resolved.** All four deferral exit criteria satisfied: (1) `paladin-ai-core` published to crates.io non-dry-run; (2) `cargo publish --dry-run -p paladin-ports --manifest-path /workspace/Cargo.toml` now passes; (3) dry-run re-run for all public crates in dependency order with successful evidence captured; (4) Epic 4 Task 5.5 updated from deferred to complete
  - "Task 5.5 is complete and this previously deferred blocker is closed."
  - **Remaining follow-up:** "Keep CI/package guardrails that detect crates.io package-name collisions early."
- scope: paladin-ports publish verification, crates.io package-name collision guardrails
- note: this is a closed item, not forward work. The only residue is the CI/package-guardrail follow-up.

---

# Ingest run 5 of 5 — requirements from Milestones 9-12, Deferred-QA-CICD-Completion, project-management

Source set: `.project/Milestone_9-Classic-Orchestrator-Completion` +
`.project/Milestone_10-CI-Hardening-Release-Automation` +
`.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
`.project/Deferred-QA-CICD-Completion` + `.project/project-management` (46 docs: 25 PRD, 21 DOC).
MODE=merge. Precedence ADR > SPEC > PRD > DOC; no per-doc overrides; no locked docs.

`- settled-by:` lines record a **fact about the shipped tree** (see `code-verification.md` run-5
section), not a decision taken by the synthesizer. Variants are preserved unmerged.

**Milestone 12 is the most recent milestone in the corpus** (Epics created 2026-06-07 to 2026-06-09).
Where its positions overlap earlier milestones, it is the later position — but later is not
authoritative here; both sides are preserved and the tree is cited.

---

## Milestone 9 — Classic Orchestrator Completion (v0.3.0)

## REQ-workflow-execution-loop
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 1-10)
- description: A real `execute_workflow()` on `Orchestrator` covering all four `WorkflowExecutionOrder` variants, replacing four `println!`-only arms in `create_workflow()`.
- acceptance:
  - `execute_workflow(&self, workflow_id: Uuid) -> Result<(), OrchestratorError>`; unknown id returns `OrchestratorError::WorkflowNotFound(id)`
  - **Sequential:** jobs run in `Vec` order; output of job N is placed into `OrchestrationContext` before job N+1 executes; a test asserts N+1 observed N's output
  - **Parallel:** all jobs spawned concurrently (`JoinSet`/`join_all`); a failure in one job must not cancel or drop sibling results; all results aggregated
  - **Custom/staged:** stages run in `Vec` order; `stage.job_ids` run concurrently within a stage; next stage does not begin until all current-stage jobs are terminal
  - **EventDriven:** listeners registered via `create_workflow_listener` so a matching event routes the target job through the real dispatch path (firing/matching validation is Epic 2)
  - Internal, crate-private `WorkflowState`/`JobState` (`Pending → Running → Completed | Failed`) recorded as transitions occur. **No new public state API** is introduced (decision 4C)
  - Unit tests per execution-order variant
- scope: Orchestrator, execute_workflow, WorkflowExecutionOrder, OrchestrationContext, internal state tracking
- settled-by: code-verification.md run-5 — `src/application/services/orchestration/mod.rs:382` `pub async fn execute_workflow` plus `execute_workflow_inner` at :403.

## REQ-taskservice-dispatch
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 11-14, Task 1.2)
- description: `ScheduledJob → TaskService::execute()` dispatch resolving services by name and honouring each job's error strategy.
- acceptance:
  - Resolve each task's `service_name` from `task_services: HashMap<String, Box<dyn TaskService>>` and invoke `TaskService::execute()`, collecting `Option<serde_json::Value>` results
  - An unregistered service returns a typed error (`OrchestratorError::ServiceError(..)`); the dispatch path must not `panic!`/`unwrap()`
  - Honour **fail-fast** (first failing task aborts the job's remaining tasks, job `Failed`) and **continue-on-error** (failing task recorded, remaining tasks still run, terminal state reflects partial completion)
  - The strategy **must reuse** the existing `Job::execute(&services)` / `JobExecutionMode` mechanism rather than inventing a parallel one
  - Retry, backoff and dead-letter are explicitly **out of scope** for this Epic
- scope: TaskService dispatch, Job, JobExecutionMode, error strategy

## REQ-default-task-services-real-logic
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 15-16)
- description: Replace the three placeholder `TaskService` implementations in `crates/paladin-core/src/platform/container/task.rs` with real, observable behaviour.
- acceptance:
  - `DataBackupService` performs a real, verifiable backup against `backup_path` and returns a result describing what was backed up
  - `ContentIndexingService` performs real indexing against `index_name` and returns a result describing the index
  - `EmailNotificationService` performs a real dispatch (SMTP or a pluggable sink the production system wires to a real transport) and returns a delivery result
  - Each removes its `tokio::time::sleep` simulation and `println!` "simulate …" scaffolding and returns a typed `TaskError` on failure rather than succeeding unconditionally
  - Unit tests assert the observable side effect and a forced failure (e.g. unwritable path)
  - File paths used by backup and indexing **must** be validated/constrained against path traversal outside the configured target directory
- scope: DataBackupService, ContentIndexingService, EmailNotificationService, TaskError
- note: Open Question 3 — which production transport `EmailNotificationService` should default to (reuse `paladin-notifications` channels, or is the injectable seam alone sufficient?) — has no recorded answer.

## REQ-workflow-repository-port
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 17-21, Task 1.3)
- description: A `WorkflowRepository` output port in `paladin-ports` with a SQLite-backed adapter, wired optionally into `Orchestrator`.
- acceptance:
  - Trait is `Send + Sync`, `#[async_trait]`, in `crates/paladin-ports/src/output/`; supports persist/update execution state (workflow id, current stage/index, per-job state, job results, error history, terminal state), load by id, and list incomplete workflows for recovery
  - SQLite adapter follows the existing adapter placement convention (`paladin-storage`) and **must use parameterized/bound queries only** — no string-formatted SQL. Persisted job results are externally-influenced data and must never be interpolated into SQL or shell
  - `Orchestrator` holds `Option<Arc<dyn WorkflowRepository>>`; constructing without a repository must keep working (in-memory default); persistence is additive and opt-in
  - When configured, `execute_workflow()` persists state at minimum on each job terminal transition and on workflow terminal transition, sufficient to resume
- scope: WorkflowRepository port, SQLite workflow adapter, Orchestrator persistence
- settled-by: code-verification.md run-5 — `crates/paladin-ports/src/output/workflow_repository_port.rs` and `crates/paladin-storage/src/sqlite_workflow_repository.rs` both ship.

## REQ-workflow-crash-recovery
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 22-23)
- description: Incomplete workflows resume on `Orchestrator::start()` without re-running completed work.
- acceptance:
  - On `start()`, when a `WorkflowRepository` is configured, load incomplete workflows and resume from the last persisted position — the next unfinished job in sequential mode, the next unfinished stage in staged mode
  - Already-completed jobs must not be re-executed
  - Crash-recovery test: persist a partially-completed workflow, construct a new `Orchestrator` on the same repository, `start()`, assert resumption and terminal `Completed`
- scope: workflow recovery, Orchestrator::start

## REQ-workflow-lifecycle-integration-test
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_1/prd-orchestrator-end-to-end-workflow-execution.md (FR 24-25, Task 1.4)
- description: A deterministic full-lifecycle integration test under `tests/`.
- acceptance:
  - Mock `TaskService` implementations with observable side effects (shared synchronized `Vec` or ordered counters)
  - A workflow of **3 sequential jobs**: create → start → execute
  - Asserts ordered execution via the side effect (**not** stdout), terminal state `Completed`, and retrievable per-job results
  - Deterministic — no wall-clock reliance, no log scraping — and passes in CI under `cargo test`
- scope: integration test, workflow lifecycle

## REQ-scheduler-tick-validation
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_2/prd-scheduler-queue-operational-validation.md (FR 1-7)
- description: Deterministic validation of the `SchedulerOrchestrator` tick loop and `calculate_next_run`.
- acceptance:
  - `calculate_next_run` correct for `Interval`, `Daily`, `Weekly`, `Monthly`, `Once` (future → `Some`, past → `None`), `OnStartup` (`None`)
  - `check_and_execute_jobs()` test: a job whose `next_run` is past dispatches its `TaskService` test double exactly once on one tick
  - After dispatch: `last_run` set, `run_count` +1, `next_run` recomputed (non-`None` for recurring, `None` for `Once`/`OnStartup`)
  - A **disabled** job is skipped — `run_count` does not advance, service not invoked
  - A `Schedule::Once` job whose time has passed runs once and does not re-fire
  - A job scheduled a short interval in the future executes (integration-style test, sub-second to a few seconds)
  - **No clock abstraction and no production scheduler refactor** is permitted to make these tests pass
- scope: SchedulerOrchestrator, calculate_next_run, check_and_execute_jobs, Schedule variants
- note: the PRD reads the Epic doc's "Cron" as the recurring `Schedule` variants; the cron-expression path is validated separately via the Tokio adapter.

## REQ-cron-adapter-validation
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_2/prd-scheduler-queue-operational-validation.md (FR 8)
- description: Validation of `TokioCronSchedulerAdapter`.
- acceptance:
  - A cron job scheduled to fire imminently actually fires, observable via a shared counter/flag the job closure increments
  - An invalid cron expression returns `SchedulerError::InvalidCronExpression`
  - Scheduling while not running returns `SchedulerError::NotRunning`
  - `start → schedule → cancel/shutdown` completes without error
  - The **UTC assumption** for cron evaluation is documented in the test or adapter doc comment; DST-aware tests are not attempted
- scope: TokioCronSchedulerAdapter, SchedulerPort, cron expressions

## REQ-queueport-contract-parity
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_2/prd-scheduler-queue-operational-validation.md (FR 9-11, 15)
- description: One reusable `QueuePort` contract test run against both the in-memory `QueueOrchestrator` and `RedisQueueAdapter`.
- acceptance:
  - Contract exercises `create_queue`, `enqueue`, `dequeue` (round-trip preserves payload), `start_processing`, `complete_processing`, `queue_length`, `health_check`
  - Runs against the in-memory `QueueOrchestrator`/`QueueService` as an always-on default test
  - Runs against `RedisQueueAdapter` gated behind the `redis-queue` feature, connecting to the existing docker-compose test stack, and **skipping gracefully** (or `#[ignore]`) when Redis is unreachable so default `cargo test` is not broken
  - The in-memory queue is verified as the working fallback when Redis is unavailable; health checks accurately reflect availability per adapter
  - **No `testcontainers` dependency**; the existing docker-compose test stack is used
- scope: QueuePort contract, QueueOrchestrator, RedisQueueAdapter, redis-queue feature
- note: resolved Open Question 1 — `docker/docker-compose.test.yml` exposes `redis-test` on host port **6380**; the contract test tries `PALADIN_TEST_REDIS_PORT`, then 6380, then 6379, then skips with a short bounded timeout.

## REQ-queue-retry-dead-letter
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_2/prd-scheduler-queue-operational-validation.md (FR 12-14)
- description: Retry and dead-letter behaviour validated for both queue adapters, with minimal parity work only.
- acceptance:
  - With `max_retries = N`, `fail_processing` reports the item re-queued for retry while `attempt_count < max_retries`
  - After `max_retries` is exhausted, `fail_processing` reports no further retry and the item moves to the failed/dead-letter store (`failed_items` in-memory; the `failed` hash for Redis), observable via stats or a getter — for **both** adapters
  - If the in-memory `QueueOrchestrator` lacks dead-letter parity, add **only** the minimal behaviour needed. No retry/back-off/dead-letter redesign is in scope
- scope: retry, dead-letter, failed_items, QueuePort contract

## REQ-event-trigger-job-pipeline
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_2/prd-scheduler-queue-operational-validation.md (FR 16-21)
- description: Validation of the event → trigger → job pipeline including negative, fan-out and rate-limit cases.
- acceptance:
  - A **matching** event through `Orchestrator`/`ListenerOrchestrator` creates exactly one `Trigger`
  - A **non-matching** event creates **no** trigger
  - **Fan-out:** multiple listeners whose conditions all match one event create exactly one trigger **per matching listener**
  - A created trigger is converted to a job and executed via the Epic 1 dispatch path, observable through a `TaskService` test double
  - **Rate limit:** when `max_triggers_per_window` is exceeded, throttled events create no excess triggers; the count is capped at the window limit
  - Only minimal glue may be added to route a trigger into dispatch; building a new listener subsystem is out of scope
- scope: ListenerOrchestrator, EventListener, TriggerCondition, rate limiting, event dispatch
- note: resolved Open Question 2 — the glue already exists: `Orchestrator::process_event()` drains triggers via `ListenerOrchestrator::get_next_trigger()` and dispatches each through `execute_trigger()` → `execute_job()`. No new glue was required.

## REQ-paladin-content-processor
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_3/prd-content-agent-bridge.md (FR 1-9, Task 3.1)
- description: A `ContentProcessor` that turns a `ContentItem` into an agent prompt, runs a single Paladin, and parses the response into a `ContentProcessingResult`.
- acceptance:
  - Implements `ContentProcessor` (`name`, `process_content`, `clone_box`) and lives in the **root crate** beside the trait (`src/application/services/orchestration/processors/`)
  - Converts the `ContentItem` to a prompt via a **configurable prompt template** interpolating body and, where available, metadata/title
  - Executes via `PaladinExecutionService::execute(&paladin, &prompt)`, with the `Paladin` and service supplied at construction
  - Maps `PaladinResult` → `ContentProcessingResult` with `content_id` = source UUID, `processor_name`, wall-clock `processing_time_ms`, `success`, and parsed `result_data`
  - **Configurable output-parsing strategy**, minimum two variants: **RawText (default)** stores the response verbatim (e.g. `{"enrichment": "<agent text>"}`); **Json** attempts to parse the response as JSON into `result_data`
  - Attaches enrichment metadata (agent name, model if available, parsing strategy, a token/length indicator where cheap)
  - **Degraded result on malformed JSON:** `success = false` (or clearly-flagged partial success), `error` populated with a diagnostic, and the raw text preserved so no data is lost — asserted in a unit test, never a panic
  - Depends only on `PaladinExecutionService`/`PaladinPort`/`LlmPort` — no concrete LLM adapter dependency
- scope: PaladinContentProcessor, prompt template, OutputParsing, ContentProcessingResult
- settled-by: code-verification.md run-5 — `src/application/services/orchestration/processors/` ships.

## REQ-battalion-content-processor
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_3/prd-content-agent-bridge.md (FR 10-17, Task 3.2)
- description: A `ContentProcessor` that runs a Battalion pattern over content and merges multi-agent output into one result.
- acceptance:
  - Supports **Formation** (sequential pipeline, e.g. summarizer → classifier → entity extractor) via `FormationExecutionService::execute()`
  - Supports **Phalanx** (parallel analysts) via `PhalanxExecutionService::execute()`
  - Pattern selection is configurable at construction (enum/config choosing Formation vs Phalanx plus the corresponding domain object)
  - A **clearly defined, code-documented merge strategy**: Formation threads outputs through the pipeline and surfaces the final output; Phalanx merges parallel analyst outputs keyed by agent name into `result_data`
  - Metadata identifies the pattern used and the participating agents
  - Depends only on the Battalion execution services (themselves `PaladinPort`-driven); unit tests use mock agents with no network
- scope: BattalionContentProcessor, Formation, Phalanx, merge strategy
- note: Open Question 5 remains **OPEN** — Maneuver-flow-driven configuration of the Battalion processor is deferred unless trivially addable via `paladin-battalion::maneuver`. Only direct Formation/Phalanx configuration is in scope.

## REQ-content-processor-orchestrator-wiring
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_3/prd-content-agent-bridge.md (FR 18-22, Task 3.3)
- description: Both processors registered and dispatched through the orchestrator's existing session lifecycle.
- acceptance:
  - `Orchestrator::register_content_processor()` accepts both as `Box<dyn ContentProcessor>` and registers by name
  - `Orchestrator::process_content(content, processor_name, context)` dispatches to the named processor within the existing session lifecycle and returns its `ContentProcessingResult`
  - An unregistered processor name returns `OrchestratorError::ProcessorNotFound(name)` — typed, asserted in a test
  - The step participates in the Epic 1 job/session lifecycle and context threading; no new lifecycle machinery
- scope: register_content_processor, process_content, ProcessorNotFound

## REQ-content-ingestion-e2e-validation
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_3/prd-content-agent-bridge.md (FR 23-25, Task 3.4)
- description: One deterministic and one live end-to-end ingestion → enrichment path.
- acceptance:
  - **Deterministic:** gated behind the `content-processing` feature, using a local fixture (`FileContentListFetcher` / fixture file / in-test `ContentItem`) and a **mock LLM**; ingest → extract/aggregate → invoke agent → stored enriched result; passes with **no network access**; asserts content id preserved, enrichment present, success true
  - **Live:** a separate test exercising a real fetch (e.g. `HttpContentFetcher`) and a real LLM provider, marked `#[ignore]` and/or credential-gated so it never runs in default CI, with documented invocation
- scope: end-to-end ingestion test, content-processing feature, mock LLM, live test gating

## REQ-orchestrator-port
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md (FR 1-7)
- description: An `OrchestratorPort` trait in `paladin-ports` giving agents four orchestration actions.
- acceptance:
  - `#[async_trait] pub trait OrchestratorPort: Send + Sync` in `crates/paladin-ports/src/output/orchestrator_port.rs`, re-exported from `output`
  - Exactly four methods, each returning `Result<_, OrchestratorBridgeError>`: `schedule_job(ScheduleJobRequest) -> Uuid`, `queue_item(QueueItemRequest) -> Uuid`, `fire_event(FireEventRequest) -> EventDispatchResult`, `send_notification(SendNotificationRequest) -> Uuid`
  - Request/result/error types live in `paladin-ports` and carry **no dependency on the root-crate `Orchestrator`**; they use only types already available there (`paladin-core` domain types such as `Schedule`, `Event`, primitives, `serde_json`) plus new plain structs
  - Request types are simple serializable, LLM-friendly value objects
  - `OrchestratorBridgeError` (thiserror) has at minimum `ActionNotAllowed(String)`, `QuotaExceeded { action, limit }`, `InvalidRequest(String)`, `OrchestratorError(String)` — the last stringified at the boundary so root-crate error types never leak into `paladin-ports`. Messages must be actionable and must not leak secrets
- scope: OrchestratorPort, request value objects, OrchestratorBridgeError, paladin-ports
- settled-by: code-verification.md run-5 — `crates/paladin-ports/src/output/orchestrator_port.rs` ships.

## REQ-bridge-policy-guardrails
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md (FR 8-11)
- description: A typed `BridgePolicy` bounding what an agent may trigger.
- acceptance:
  - An **action allow-list** over `BridgeAction { ScheduleJob, QueueItem, FireEvent, SendNotification }` plus **quantitative caps** (`max_jobs_scheduled`, `max_items_queued`, `max_events_fired`, `max_notifications_sent`)
  - A conservative `Default` (all four actions allowed with small, low-single-digit caps) plus a builder or explicit constructors
  - A disallowed action is rejected with `ActionNotAllowed` and a cap-exceeding action with `QuotaExceeded`, **before** any underlying orchestrator call
  - Caps are enforced **per `PaladinExecutionService` execution** — counters reset per run, not global for the process lifetime — and the counting mechanism is thread-safe
  - This is deliberately minimal — an allow-list plus caps, **not** RBAC — sufficient to bound a misbehaving or prompt-injected agent
- scope: BridgePolicy, BridgeAction, quota enforcement, prompt-injection containment

## REQ-orchestrator-bridge-adapter
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md (FR 12-18)
- description: The concrete `OrchestratorBridgeAdapter` in the root crate.
- acceptance:
  - Lives at `src/application/services/orchestration/orchestrator_bridge.rs` **because it depends on the root-crate `Orchestrator`**; a module doc comment must explain the placement (identical rationale to Epic 3's processors — a lower crate cannot depend on the root crate)
  - Holds `Arc<Orchestrator>`, a `BridgePolicy`, and an optional `Arc<dyn NotificationDeliveryPort>`; absence of the delivery port yields `ActionNotAllowed`/`InvalidRequest` for `send_notification`
  - `schedule_job` builds a `Job` via `Job::new` and calls `Orchestrator::schedule_job(job, schedule, context)`
  - `queue_item` enqueues via `Orchestrator::queue_job` / the underlying `QueueService`
  - `fire_event` builds an `Event` via `Event::new(event_type, payload, source)` and dispatches through `ListenerOrchestrator::process_event` — **the `Orchestrator` exposes no public `fire_event`**, so the adapter dispatches via the listener service it owns — returning an `EventDispatchResult` describing triggers created
  - `send_notification` builds a `Notification` and delivers via `NotificationDeliveryPort::deliver_notification`
  - Every method consults `BridgePolicy` first; underlying errors map to `OrchestratorBridgeError::OrchestratorError` with a descriptive message
- scope: OrchestratorBridgeAdapter, root-crate placement, NotificationDeliveryPort
- settled-by: code-verification.md run-5 — `src/application/services/orchestration/orchestrator_bridge.rs` ships.

## REQ-execution-service-bridge-wiring
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md (FR 19-24)
- description: Optional bridge attachment on `PaladinExecutionService` with byte-for-byte unchanged behaviour when absent.
- acceptance:
  - `PaladinExecutionService` gains `orchestrator_port: Option<Arc<dyn OrchestratorPort>>`, mirroring the existing optional `garrison`/`arsenal` fields
  - A `with_orchestrator_port(...)` builder-style setter (or equivalent) is added **backward-compatibly**: the 4-arg `PaladinExecutionService::new(llm_port, circuit_breaker, garrison, arsenal)` must keep compiling for existing call sites
  - When `orchestrator_port` is `None`, agent execution behaviour is **byte-for-byte unchanged**
  - Unit tests cover all four methods against a mock: success, `ActionNotAllowed`, `QuotaExceeded`
  - Integration test `tests/agent_orchestrator_bridge.rs` deterministically drives a real `PaladinExecutionService` with a scripted mock LLM tool-call to `schedule_job`, then asserts the job is observable in the real `Orchestrator`'s scheduler state. Only `schedule_job` is required end-to-end
- scope: PaladinExecutionService, optional bridge, integration test
- note: **Option B (an LLM-discoverable `OrchestratorArmament` Arsenal tool) was explicitly considered and deferred** as a non-breaking follow-up that can wrap `Arc<dyn OrchestratorPort>` without changing the port. The documented trade-off is that Option A loses LLM self-describing discoverability in exchange for centralised safety enforcement and decoupling.

## REQ-user-role-rbac
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md (FR 1-4)
- description: A persisted `UserRole` on the user record.
- acceptance:
  - `UserRole` enum with at least `Admin` and `User`, string representation `"admin"`/`"user"`, and a parser from string
  - `UserData` carries `role: UserRole` defaulting to `UserRole::User` for new users; `User` exposes `role()` and `set_role(UserRole)` consistent with the `Node<UserData>` accessor pattern
  - The `users` table gains a `role` column (`TEXT NOT NULL DEFAULT 'user'`) applied **idempotently** so existing databases upgrade without data loss; the SQLite repository row mapping reads and writes it
  - The role is stored on `UserData` (persisted) rather than only in the token, so privileges survive re-login and the record is the single source of truth; the token merely **carries** the role to avoid a DB round-trip per request
- scope: UserRole, UserData, users table migration, SQLite user repository

## REQ-auth-port
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md (FR 5-7)
- description: An `AuthPort` in `paladin-ports` with its value objects and error type.
- acceptance:
  - `#[async_trait] AuthPort: Send + Sync` with `issue_token(user_id: Uuid, role: UserRole) -> Result<AuthToken, AuthError>`, `verify_token(&str) -> Result<AuthClaims, AuthError>`, `revoke_token(&str) -> Result<(), AuthError>`
  - `AuthToken { token: String, expires_at: DateTime<Utc> }`; `AuthClaims { user_id: Uuid, role: UserRole, expires_at: DateTime<Utc> }`
  - `AuthError` (thiserror) with at least `MissingToken`, `InvalidToken`, `Expired`, `Internal(String)`
  - `AuthPort` and `UserRole` are always-compiled in `paladin-ports`/`paladin-core`, independent of the `web-server` feature
- scope: AuthPort, AuthToken, AuthClaims, AuthError, paladin-ports

## REQ-opaque-bearer-token-adapter-v1
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md (FR 8-12, §5 Non-Goals, §6.1)
- description: **Variant v1** — the token mechanism behind `AuthPort` is an opaque, randomly-generated bearer token with a server-side hashed store. JWT is an explicit non-goal.
- acceptance:
  - The concrete adapter lives in the **root crate** (it needs `rand`/`sha2`/`chrono`) and issues opaque, cryptographically-random bearer tokens
  - Tokens are stored **hashed** (never plaintext) and compared via hash lookup — no plaintext token comparison
  - Tokens carry an expiry; `verify_token` rejects expired tokens with `AuthError::Expired`; `revoke_token` invalidates a token so subsequent verification fails
  - Login issues a token on successful password verification and returns token string + expiry alongside existing identity fields
  - **Explicit non-goal: "JWT/OIDC/OAuth or any external identity provider integration."**
  - Recorded rationale: avoids a `jsonwebtoken` dependency and a signing-key management story; supports immediate **revocation** which stateless JWTs cannot; trivially deterministic to unit test; the root crate already has `rand` and `sha2`, so **no new dependencies**
  - Recorded trade-off: "tokens are validated against an in-process store, so a **multi-process deployment would later need a shared store**. This is acceptable because validation is hidden behind `AuthPort`, so the store can be swapped without touching the web layer."
- scope: AuthPort adapter, opaque bearer tokens, hashed token store, revocation
- note: **v1 of a competing pair with `REQ-jwt-bearer-auth-v2` (Milestone 12 Epic 5).** See INGEST-CONFLICTS.md WARNINGS.
- settled-by: code-verification.md run-5 — `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` is the only shipped `AuthPort` implementation and **no `jsonwebtoken` dependency exists anywhere in the workspace.** v1 is what ships.

## REQ-auth-middleware-rbac-guards
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md (FR 13-17, 21)
- description: Axum authentication middleware and an admin role guard in `paladin-web`.
- acceptance:
  - Reusable middleware/extractor reads `Authorization: Bearer <token>`, calls `AuthPort::verify_token`, and injects `AuthClaims` into the request on success
  - Missing/malformed header or invalid/expired token produces `401 Unauthorized` with a JSON error body and **must not reveal which part failed**
  - A role guard requiring `UserRole::Admin` produces `403 Forbidden` for non-admin callers
  - Admin-only endpoints — list users, activate, deactivate, verify, delete — are protected by both authentication and the admin guard
  - Self-service endpoints (get/update own profile) require authentication; a non-admin accessing **another** user's record gets `403`
  - A single composition function (e.g. `create_app_router(user_service, auth_port)`) assembles public routes (register, login) and protected routes with the right middleware
  - `paladin-web` depends only on `paladin-ports` + `paladin-core`; its middleware is generic over `Arc<dyn AuthPort>` and **never performs cryptography itself**
- scope: paladin-web auth middleware, admin guard, router composition

## REQ-user-crud-completeness
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md (FR 18-20, 22-23)
- description: Complete the user CRUD surface and prove the auth/RBAC paths offline.
- acceptance:
  - Add a `delete_user` operation and an admin-only `DELETE /users/:id` route (the repository already supports delete)
  - Add an admin `list_users` endpoint backed by existing repository/service query methods
  - All user-data responses continue to **omit the password hash**
  - Unit tests cover issue → verify round-trip, expiry rejection, revoke rejection, invalid-token rejection, and role/string conversions
  - Offline deterministic integration tests on the assembled router assert: protected route without a token → `401`; with a valid token → `200`; admin-only route with a `user`-role token → `403`; with an `admin`-role token → success
- scope: user CRUD, delete_user, list_users, auth integration tests

## REQ-m9-quality-gate-v030
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/Epic_6/prd-finalization-and-release.md (FR 1-15)
- description: Milestone 9 finalization — workspace quality gate, CHANGELOG, lockstep `0.3.0`, `v0.3.0` tag.
- acceptance:
  - `cargo build --workspace`, `cargo test --workspace` (including feature paths such as `redis-queue` and `web-server`, run explicitly where `--workspace` alone does not enable them), `cargo clippy --workspace -- -D warnings` (and `--all-features` where earlier Epics relied on flags), `cargo fmt --all -- --check`, and `cargo doc --workspace --no-deps` all exit 0
  - `CHANGELOG.md` gains a `0.3.0` entry summarising Epics 1-5 grouped by feature area (Orchestration, Scheduler/Queue, Content Pipeline, Agent Bridge, User/Admin & Security), describing user-visible changes rather than commit-by-commit detail
  - Root crate and every workspace member at `0.3.0`; **all internal `[workspace.dependencies]` pins updated in lock-step** or cargo will fail to resolve path dependencies. `paladin-core` uses `package = "paladin-ai-core"`
  - A `v0.3.0` release-candidate tag on a commit where the full gate passes
  - Explicit non-goal: "Reconciling whether the previous published version *should* have been `0.2.0`; this Epic targets `0.3.0` per the Epic specification regardless of intervening version numbers."
- scope: M9 quality gate, CHANGELOG 0.3.0, lockstep version bump, v0.3.0 tag

---

## Milestone 10 — CI Hardening and Release Automation (v0.4.0)

## REQ-pre-commit-framework
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_1/prd-pre-commit-pre-push-hooks.md (FR 1-3)
- description: Adopt the `pre-commit` framework with a version-controlled, revision-pinned configuration.
- acceptance:
  - `.pre-commit-config.yaml` at the repository root is the version-controlled hook manager configuration; the rejected alternative (`cargo-husky`) and the rationale must be recorded
  - The framework must support both Rust-specific hooks (`cargo fmt`, `cargo clippy`) and ecosystem hooks (secrets, TOML, YAML, whitespace, file-size, merge-conflict)
  - Every hook repository is pinned to a specific released revision (`rev:`) so behaviour is reproducible across machines and over time
- scope: pre-commit framework, .pre-commit-config.yaml, pinned revisions

## REQ-pre-commit-hook-set
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_1/prd-pre-commit-pre-push-hooks.md (FR 4-13, 19, 22)
- description: The nine commit-stage hooks and the whole-repo conformance requirement.
- acceptance:
  - `cargo fmt --all -- --check` fails the commit on any diff
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` fails on any warning
  - `gitleaks` secrets detection fails on a detected credential/API key/password pattern
  - `check-toml` (Cargo.toml / deny.toml / audit.toml), `check-yaml` (config.yml / config.test.yml / compose files)
  - `check-added-large-files` with a **1 MB** limit; `check-merge-conflict`; `trailing-whitespace`; `end-of-file-fixer`
  - The Rust hooks run **once for the workspace**, not once per changed file (`pass_filenames: false` system hooks), to avoid redundant `cargo` invocations
  - The untracked ad-hoc `.git/hooks/pre-commit` script is superseded by the version-controlled configuration
  - `pre-commit run --all-files` must pass against the current repository; pre-existing violations are remediated as **formatting/whitespace and config-syntax fixes only — no feature or behavioural code changes**
- scope: pre-commit hooks, gitleaks, check-toml, check-yaml, whitespace hygiene

## REQ-pre-push-hook-set
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_1/prd-pre-commit-pre-push-hooks.md (FR 14-18)
- description: A fast pre-push quality subset installed by the same single command.
- acceptance:
  - Pre-push runs `cargo build --workspace` and the unit-test subset `cargo test --workspace --lib` (fast; excludes `tests/` integration and doc tests)
  - Wired through the same `pre-commit` framework (`pre-commit install --hook-type pre-push` / a `pre-push` stage) so one install command enables both stages
  - A `make hooks` target wraps `pre-commit install` + `pre-commit install --hook-type pre-push`
  - `CONTRIBUTING.md` documents installing `pre-commit`, installing the hooks, running them manually (`pre-commit run --all-files`), and the emergency override (`git commit --no-verify` / `git push --no-verify`)
- scope: pre-push hooks, make hooks, CONTRIBUTING.md, --no-verify override

## REQ-pre-commit-ci-gate
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_1/prd-pre-commit-pre-push-hooks.md (FR 20-21)
- description: CI runs the identical hook suite as a required gate.
- acceptance:
  - A CI step runs `pre-commit run --all-files` so hooks are enforced even for contributors who never installed them locally
  - Runs on every pull request and on pushes to the primary branches, failing the build when any hook fails
- scope: CI pre-commit gate, .github/workflows

## REQ-audit-toml-single-source
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 1-2, 4, §8 Success Metrics)
- description: **The origin policy for dependency-advisory suppression.** `cargo audit` must source its ignore-list from the version-controlled `audit.toml`, never from inline flags.
- acceptance:
  - The CI `security-audit` job invokes `cargo audit` such that its ignore-list is sourced from the version-controlled `audit.toml` (**single source of truth**) rather than inline `--ignore` flags, **"so the workflow and the config cannot drift"**
  - `cargo audit` runs on every pull request and on every push to the primary branches (`main`, `develop`) and **fails the build on any advisory not listed in `audit.toml`**
  - The step must be reproducible locally — `cargo audit` from the repo root honours the same `audit.toml` — and the equivalent command is documented
  - Success metric: **"`audit.toml` and `deny.toml` are the only places policy/exceptions are defined; no inline advisory-ignore flags remain in CI."**
  - Design consideration: "`deny.toml` and `audit.toml` are the version-controlled single sources of truth; CI must read from them rather than re-specifying policy inline."
- scope: cargo audit, .cargo/audit.toml, CI security-audit job, single source of truth
- note: the governing **Epic DOC** (`Milestone_10-Epic_2-dependency-security-license-compliance.md`, Task 2.1) states only "a documented exception process for false positives or unpatched advisories" — it enumerates **no advisory IDs, no suppression counts, and never mentions `.cargo/audit.toml` or inline `ci.yml` ignores.** The PRD is the more specific carrier and is what this entry records.
- settled-by: code-verification.md run-5 — **PARTIALLY VIOLATED IN THE TREE.** `ci.yml:62-77` (`security-audit`) complies exactly, with an inline comment restating the single-source rule. But a **second, duplicate job `security` at `ci.yml:390-406`, bearing the identical display name "Security Audit", still runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`.** Milestone 10 Epic 2 added the compliant job without removing its predecessor, so the success metric above is unmet on a milestone recorded 100% complete.

## REQ-advisory-exception-process
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 3, §5 Non-Goals)
- description: The documented, auditable shape of a RustSec exception.
- acceptance:
  - Each ignored advisory in `audit.toml` must carry a comment stating **(a)** the advisory ID, **(b)** the affected crate and why it is present (e.g. transitive/dev-only), **(c)** why it is not yet fixable, and **(d)** a revisit condition
  - **"The two existing exceptions already follow this shape and must be preserved"** — `RUSTSEC-2023-0071` (rsa via sqlx-mysql) and `RUSTSEC-2025-0111` (tokio-tar via testcontainers)
  - Explicit non-goal: remediating those two exceptions. "These remain tracked until an upstream fix exists; this Epic preserves and documents them, it does not force an upgrade of `sqlx`/`testcontainers`."
- scope: audit.toml exception comment schema, the two baseline advisories
- note: **the PRD's exception baseline is exactly two advisories.** No document in the 199-document ingest authorises a third. See `code-verification.md` run-5 and INGEST-CONFLICTS.md WARNINGS for the current five-advisory state and its governance gap.

## REQ-osv-scanner-supplementary
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 5-7, OQ-1)
- description: OSV-Scanner as a supplementary, non-contradictory advisory source.
- acceptance:
  - Added to CI and configured to scan `Cargo.lock`
  - Findings reported as PR annotations (official OSV-Scanner GitHub Action SARIF upload / reviewdog) **without failing the build on advisories already excepted via the `cargo audit` process**, to avoid contradictory gates. The failure/annotation policy must be explicitly chosen and documented
  - Runs on pull requests, and may additionally run on a schedule for the primary branch so new advisories in already-merged dependencies surface over time
  - Open Question 1 recommendation: **annotate-only initially (non-blocking)**, tightening to blocking once annotation noise is understood
- scope: OSV-Scanner, Cargo.lock, SARIF, PR annotations

## REQ-snyk-evaluation-decision
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 8-9, OQ-2)
- description: A recorded integrate-or-defer decision on Snyk. No silent skip.
- acceptance:
  - A short evaluation comparing Snyk's free tier against the combined `cargo audit` + OSV-Scanner + `cargo deny` coverage, considering added value, required account secrets, and maintenance cost
  - Either integrate Snyk into CI **or** record a documented deferral with rationale and reconsideration conditions. **"(No silent skip.)"**
  - Open Question 2 default recommendation: **defer**, because audit + OSV + deny already cover advisories and licences without an external account secret; revisit if reachability analysis or fix-PR automation is needed
- scope: Snyk evaluation, integrate-or-defer record

## REQ-deny-license-allowlist
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 10-11, 13-14)
- description: A version-controlled `deny.toml` enforcing a permissive-only licence allow-list.
- acceptance:
  - `deny.toml` exists at the repository root using the current cargo-deny schema (advisories/licenses/bans/sources), SPDX identifiers, and the modern `allow = [...]` form
  - `[licenses]` allows **`MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`**. Copyleft (`GPL-*`, `AGPL-*`, `LGPL-*`) is **not** allowed unless explicitly added with recorded justification
  - `cargo deny check` is a required CI gate on every PR and push to primary branches, passing against the current tree
  - A crate whose licence is off-list or unknown is resolved either by (a) adding the specific licence to the allow-list **with justification**, or (b) a narrowly-scoped per-crate `clarify`/exception **with a comment**. **"Blanket disabling of the license check is not acceptable."**
- scope: deny.toml, licence allow-list, cargo deny check, per-crate exceptions
- note: the governing Epic DOC states the same six-licence allow-list, and is **silent on Paladin's own crate `license` field.** Neither document addresses the `license = "MIT"` versus `MIT OR Apache-2.0` question raised by the run-4 licence-policy sign-off (`REQ-license-policy-signoff`). No inference is made here.

## REQ-deny-bans-duplicates
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 12, OQ-4)
- description: The `[bans]` posture — no banned crates initially, duplicates surfaced as warnings.
- acceptance:
  - `[bans]` starts with **no banned crates** (extensible later)
  - Duplicate crate versions (same crate at multiple incompatible versions) are surfaced **at least as a warning**
  - Open Question 4: start `multiple-versions` at `warn`, **not `deny`**, to avoid blocking on transitive duplicates outside our control; revisit promoting to `deny` once the tree is de-duplicated
- scope: deny.toml [bans], multiple-versions, wildcards
- note: superseded in one respect by Milestone 8 Epic 7, which added `actix-web` to `[bans].deny`. That ban predates this requirement's implementation and is recorded as `REQ-actix-deny-ban` (run 4).

## REQ-cyclonedx-sbom-release
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 15-17, OQ-3)
- description: A CycloneDX SBOM generated for every release and attached as an artifact.
- acceptance:
  - `.github/workflows/release.yml` generates a CycloneDX SBOM from the locked dependency graph (e.g. `cargo cyclonedx`)
  - The SBOM is attached as an asset to the corresponding GitHub release
  - Generation is reproducible locally via a documented command and/or Makefile target
  - Open Question 3: CycloneDX is the chosen format; SPDX is not required (assume CycloneDX-only for now)
- scope: CycloneDX SBOM, cargo cyclonedx, release.yml, GitHub release assets

## REQ-security-docs-make-target
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md (FR 18-20)
- description: Documentation and a one-command local security check.
- acceptance:
  - `CONTRIBUTING.md` and/or `docs/` documents how to run `cargo audit` and `cargo deny` locally, how to add an approved exception to `audit.toml`/`deny.toml`, and where SBOMs are published
  - A `make security` (or `make deny`) target wraps `cargo audit` + `cargo deny check` for one-command local verification
  - All newly added CI gates pass against the current dependency tree at implementation time; any unavoidable exception is explicit and commented in the relevant config file
  - Some checks (OSV-Scanner action, Snyk action, SBOM upload-to-release) are **CI-only** and cannot be fully exercised in the dev container; they are validated by config correctness plus the locally-runnable equivalents
- scope: CONTRIBUTING.md security docs, make security, make deny

## REQ-release-tooling-selection
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_3/prd-release-automation.md (FR 1-3)
- description: A written, recorded choice between `cargo-release` and `release-plz`.
- acceptance:
  - A written evaluation comparing the two across at least: trigger model (manual vs PR-bot), changelog handling, workspace publish-order support, required secrets/permissions, operational/maintenance cost
  - An explicit recommendation and selected tool, captured in a version-controlled document (`docs/RELEASE_AUTOMATION.md`)
  - The selected tool is installable in CI via a pinned `--locked` install (or pinned action) and reproducible locally
- scope: cargo-release vs release-plz, RELEASE_AUTOMATION.md

## REQ-workspace-publish-order
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_3/prd-release-automation.md (FR 4, 6-7)
- description: The canonical dependency-first crates.io publish order.
- acceptance:
  - Order (per Milestone 7 Appendix B): 1. `paladin-core`; 2. `paladin-ports`; 3. `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage` (parallel-safe tier); 4. `paladin` (facade); 5. `paladin-cli` **only if/when it exists as a separate publishable crate**
  - A dependency-first `cargo publish --dry-run` succeeds for every workspace crate; where an upstream crate is not yet on crates.io, the ordering and expected-failure behaviour is documented, **not** treated as a hard failure for first-publish crates
  - Non-publishable crates are explicitly `publish = false`; publishable crates carry complete `description`, `license`, `repository`
- scope: publish order, cargo publish --dry-run, publish = false, crate metadata
- note: the tier-3 list names **nine** crates plus the facade, omitting `paladin-herald`, which postdates the Milestone 7 appendix it cites. Run-4 verification established that no `paladin-cli` crate exists — the item 5 conditional was never triggered.

## REQ-lockstep-versioning
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_3/prd-release-automation.md (FR 5, §5 Non-Goals)
- description: One shared version across all public crates, bumped together.
- acceptance:
  - All public crates share one version number bumped in lockstep, consistent with the existing `0.3.0`-everywhere convention and `docs/RELEASE_CHECKLIST.md`
  - Explicit non-goal: **"Changing the versioning *policy* itself (lockstep vs. independent) — this Epic encodes the existing lockstep convention, it does not redesign it."**
- scope: lockstep versioning, workspace.dependencies pins

## REQ-tag-triggered-publish-pipeline
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_3/prd-release-automation.md (FR 8-14)
- description: Extend `release.yml` to publish crates on a `v*.*.*` tag.
- acceptance:
  - Triggered by Git tags matching `v*.*.*` (the existing trigger)
  - The **full test suite runs before any publish/release step**; a release must not proceed if tests fail
  - Publishes all publishable crates to crates.io **in dependency order**, gated on tests, using a `CARGO_REGISTRY_TOKEN` / `CRATES_IO_TOKEN` repository secret
  - The publish job is **safe to re-run**: re-publishing an already-published version must not fail the whole pipeline (idempotent / tolerant of "already uploaded")
  - Existing behaviour is preserved: Docker images, binaries, SBOM generation, and GitHub release creation with the changelog
  - The publish job runs only for real releases; a documented dry-run path or pre-release/`workflow_dispatch` mode tests the pipeline without publishing
  - The workflow YAML must pass `pre-commit run check-yaml`
- scope: release.yml, publish-crates job, CARGO_REGISTRY_TOKEN, idempotent publish

## REQ-make-release-target
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_3/prd-release-automation.md (FR 15-19)
- description: `make release VERSION=x.y.z` orchestrating the local half of a release.
- acceptance:
  - Bumps the version in lockstep across all public crates, finalizes the changelog (`Unreleased` → the new version), commits, creates the `vx.y.z` tag, and pushes — the push triggers the CI pipeline
  - Fails fast with a clear error if `VERSION` is absent or not valid semver
  - Runs release-readiness checks (or reuses `release-check`) before tagging so a broken tree is never tagged and pushed
  - The **existing `make release` behaviour (dry-run publishes) is preserved under a clearly named target** (e.g. `make publish-dry-run`) so no capability is lost
  - The flow is documented in `CONTRIBUTING.md` and cross-referenced from `docs/RELEASE_CHECKLIST.md` / `docs/RELEASE_AUTOMATION.md`, including required secrets and how to dry-run
- scope: make release, make publish-dry-run, make release-check, CONTRIBUTING.md

## REQ-contributing-add-dependency-guide
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_4/prd-milestone10-finalization.md (FR 1-2)
- description: A single "Adding a New Dependency" workflow in `CONTRIBUTING.md`.
- acceptance:
  - Seven documented steps: choose the crate and add it to the right `Cargo.toml`; verify the licence is in `deny.toml` `[licenses].allow` or that an exception exists (**"If not, open a discussion before adding"**); run `make deny` locally (a licence rejection means the crate is not permitted — resolve before merging); run `make audit` (a fresh dependency must produce **zero** new vulnerability errors); if cargo-deny reports a new **unmaintained** advisory, document the rationale and add a scoped `[advisories].ignore` entry in `deny.toml` **with an explanatory comment**; update `CHANGELOG.md [Unreleased]` if user-visible; the CI `cargo-deny` and `security-audit` jobs are the final gate — do not merge if either fails
  - The `CONTRIBUTING.md` Table of Contents covers every top-level `##` section, adding `Security`, `Releasing`, `Per-Crate Changelog Maintenance` and `Adding a New Dependency` where missing
- scope: CONTRIBUTING.md dependency workflow, ToC completeness
- note: step 5 is the **only** ingested authorisation for adding `deny.toml` `[advisories].ignore` entries beyond the Epic 2 baseline, and it authorises **unmaintained** advisories specifically — not vulnerability advisories.

## REQ-m10-v040-release
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_4/prd-milestone10-finalization.md (FR 3-7, §5 Non-Goals)
- description: Milestone 10 finalization — CHANGELOG, lockstep `0.4.0`, annotated tag.
- acceptance:
  - `CHANGELOG.md [Unreleased]` documents all four Epics: hooks and devcontainer provisioning; `.cargo/audit.toml`, `deny.toml` licence policy and advisory exceptions, CycloneDX SBOM, OSV-Scanner annotate-only job, `make security` / `make sbom`, `docs/SECURITY_SCANNING.md`; cargo-release selection and `release.toml`, tag-triggered `publish-crates`, `make release` / `make publish-dry-run`, `docs/RELEASE_AUTOMATION.md`; the dependency guide and v0.4.0 bump
  - Lockstep `0.3.0 → 0.4.0` across every `[package].version` and every internal pin, performed atomically via `cargo release version 0.4.0 --execute --no-confirm --workspace`
  - `make release VERSION=0.4.0` validates semver, runs `make release-check`, bumps, finalizes the changelog, commits `chore(release): version 0.4.0`, creates the annotated `v0.4.0` tag, and pushes
  - Pre-release conformance: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, `cargo audit`, `pre-commit run --all-files` all pass independently
  - Explicit non-goals: **"No new Rust source code changes"**; **"No changes to `deny.toml` or `.cargo/audit.toml` — the existing exception lists are not revisited"**; **"No new CI jobs — the Epic 3 pipeline is complete."**
- scope: CHANGELOG 0.4.0, lockstep bump, annotated v0.4.0 tag, conformance gate

## REQ-verify-tag-source-guard
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_5/prd-milestone10-release-branch-protection.md (FR 1)
- description: A CI guard blocking any release whose commit is not contained in `main`.
- acceptance:
  - A `verify-tag-source` job in `release.yml` runs before any release work, checking out with full history (`fetch-depth: 0`)
  - Resolves the commit under release: `github.sha` for a `push` tag event; the commit `inputs.tag` points to for `workflow_dispatch`
  - Fetches `origin/main` and verifies the release commit is reachable from it via `git merge-base --is-ancestor`
  - On failure, prints a GitHub `::error::` annotation and exits non-zero
  - The `test` and `create-release` jobs (the two roots all other release jobs depend on) declare `needs: verify-tag-source`, so a failed guard prevents publishing, Docker, binaries and SBOM steps
- scope: release.yml verify-tag-source, main-only tag policy
- note: this Epic exists because of a recorded incident — **a `v0.4.0` tag cut from a feature branch.** The Epic 5 PRD names it directly as the reason the policy exists.

## REQ-make-release-branch-guard
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_5/prd-milestone10-release-branch-protection.md (FR 2)
- description: A local guard in `make release` refusing to run off an up-to-date `main`.
- acceptance:
  - Before bumping or tagging, verifies the current branch is `main` and that local `HEAD` is not behind `origin/main` (`git rev-list HEAD..origin/main` is empty)
  - On failure, prints a clear red error and exits non-zero **before any destructive action**
  - `RELEASE_ALLOW_ANY_BRANCH=1` bypasses **only** the branch-name check (rare hotfix/maintenance releases) while still printing a warning; the CI guard remains the authoritative gate
- scope: Makefile release guard, RELEASE_ALLOW_ANY_BRANCH override

## REQ-github-rulesets
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_5/prd-milestone10-release-branch-protection.md (FR 3, §5 Non-Goals)
- description: Importable GitHub ruleset definitions committed to the repository.
- acceptance:
  - `.github/rulesets/protect-main-branch.json` — a branch ruleset targeting `main` (optionally `develop`) requiring a pull request before merging, requiring the CI `lint`, `security-audit` and `cargo-deny` status checks, and blocking force-pushes and branch deletion
  - `.github/rulesets/protect-release-tags.json` — a tag ruleset targeting `refs/tags/v*` restricting tag creation and deletion to bypass actors (repository admins / maintainers)
  - Each file is valid JSON importable via the GitHub UI (Settings → Rules → Rulesets → Import) or `gh api`
  - Explicit non-goal: **no automated application.** Rulesets require repo-admin scope and cannot be safely self-applied from CI; an administrator applies them manually
- scope: .github/rulesets, branch protection, tag protection
- note: the definitions ship; **whether they have been applied to the GitHub repository is outside the tree and cannot be verified here.** This is the one Milestone 10 deliverable whose effect is unverifiable from the repository alone.

## REQ-branch-protection-doc
- source: /workspace/.project/Milestone_10-CI-Hardening-Release-Automation/Epic_5/prd-milestone10-release-branch-protection.md (FR 4-6)
- description: Documentation of the main-only release policy and its three enforcement layers.
- acceptance:
  - `docs/BRANCH_PROTECTION.md` explains the policy and **why** it exists (the `v0.4.0`-from-feature-branch incident); the three enforcement layers (CI guard, Makefile guard, GitHub rulesets) and how they relate; step-by-step admin import instructions for each ruleset (UI and `gh api`); the correct release flow (merge to `main` via PR → pull `main` → `make release VERSION=…` from `main`); and the `RELEASE_ALLOW_ANY_BRANCH=1` override and when it is acceptable
  - `CONTRIBUTING.md` `## Releasing` states releases are cut **only from `main`** after merge via PR, cross-links the new doc, and makes "ensure you are on an up-to-date `main`" step 0
  - `CHANGELOG.md [Unreleased]` records the Epic 5 additions
  - Explicit non-goal: **"No rewrite of the existing `v0.4.0` tag/release. Reconciling `main` with the released code is a maintainer merge action, noted in docs but not performed by this epic."**
- scope: BRANCH_PROTECTION.md, CONTRIBUTING.md Releasing section
- settled-by: code-verification.md run-5 — ships as `docs/src/appendix/branch-protection.md`, not `docs/BRANCH_PROTECTION.md`. Same Milestone 11 mdbook relocation runs 3 and 4 recorded. **Do not plan it as a missing deliverable.**

---

## Milestone 11 — Documentation Overhaul and Publish (v0.5.0)

## REQ-mdbook-scaffold
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_2/prd-mdbook-setup-and-structure.md (§4.1, Goals 1-4)
- description: An MDBook at `docs/` with linkcheck and mermaid preprocessors.
- acceptance:
  - `docs/book.toml` configures the book with `src = "src"`, the `mdbook-linkcheck` backend and the `mdbook-mermaid` preprocessor
  - `mdbook build docs/` completes with zero errors and zero warnings locally
  - `mdbook-linkcheck` passes — no broken internal links
  - `mdbook-mermaid` renders architecture diagrams present in migrated files
  - **No MDBook plugins beyond `mdbook-mermaid` and `mdbook-linkcheck`**; default theme only; single-version site (no `mdbook-versioning`)
- scope: docs/book.toml, mdbook-linkcheck, mdbook-mermaid

## REQ-mdbook-chapter-hierarchy
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_2/prd-mdbook-setup-and-structure.md (§4.2-4.6, Goals 8-9)
- description: The `docs/src` chapter hierarchy, `SUMMARY.md`, and the appendix escape hatch.
- acceptance:
  - Every chapter in `SUMMARY.md` links to a real file; no dangling links
  - All existing docs not flagged for deletion by the Epic 1 audit are present somewhere in the new structure
  - **Docs with no single-chapter home are placed in an `appendix/` chapter rather than dropped**
  - Placeholder files are created where a chapter is planned but unwritten
  - Explicit non-goal: **content accuracy.** "Stale API paths, wrong version strings, broken Rust examples are Epic 3's responsibility. This Epic migrates content as-is."
- scope: docs/src hierarchy, SUMMARY.md, appendix chapter, placeholders
- note: the appendix escape hatch is why `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`, `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md`, `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`, `docs/SECURITY_SCANNING.md`, `docs/RELEASE_AUTOMATION.md`, `docs/BRANCH_PROTECTION.md` and `docs/Design/Design_and_Architecture.md` are all absent from the paths named in runs 3, 4 and 5. **This requirement is the mechanism behind every "missing deliverable" false positive in the corpus.**

## REQ-docs-ci-pages-deploy
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_2/prd-mdbook-setup-and-structure.md (§4.7-4.8, Goals 5-7)
- description: A `docs.yml` workflow building on PR and deploying to GitHub Pages on merge.
- acceptance:
  - `.github/workflows/docs.yml` builds the book on every PR touching `docs/**`
  - On merge to `main`, the same workflow deploys the built site to GitHub Pages
  - The site is reachable at `https://df3ndr.github.io/paladin-dev-env/`
  - Explicit non-goal: `cargo test --doc` in this workflow — the workflow triggers on `docs/**` only
- scope: .github/workflows/docs.yml, GitHub Pages

## REQ-docs-migration-log
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_2/prd-mdbook-setup-and-structure.md (§4.9, §5)
- description: A migration log recording where each pre-existing doc went.
- acceptance:
  - `docs/MIGRATION_LOG.md` records the source → destination mapping for migrated documents
  - Files are moved with `git mv`, preserving history; **no history rewriting** and no removal of the old flat structure from git history
- scope: docs/MIGRATION_LOG.md, git mv

## REQ-doc-link-repair-linkcheck
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-1, FR-33)
- description: Repair **227 broken internal cross-reference links** and re-enable linkcheck as an error.
- acceptance:
  - All 227 broken internal links (stale flat-file paths) resolved to correct MDBook-relative paths
  - `docs/book.toml` re-enables `[output.linkcheck]` with `follow-web-links = false` and `warning-policy = "error"`
  - `mdbook build` passes with zero linkcheck errors
  - `introduction.md`'s 14 broken links to old flat paths (e.g. `QUICKSTART.md` → `getting-started/quickstart.md`) are fixed
  - Explicit non-goal: external link validation — `follow-web-links = false`
- scope: 227 broken links, book.toml linkcheck, introduction.md
- settled-by: code-verification.md run-5 — `docs/book.toml` carries `[output.linkcheck]` with `follow-web-links = false` and `warning-policy = "error"` exactly as specified.

## REQ-doc-example-compile-gate
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-2, FR-3)
- description: Doc code blocks are `cargo check`ed locally and in CI.
- acceptance:
  - A new hook in `.pre-commit-config.yaml` under the existing `local` repo section as a **`pre-push` stage** hook (matching `cargo-build-push` / `cargo-test-lib-push`) runs `scripts/check-doc-examples.sh` — consistent enforcement for every developer with no manual opt-in
  - A `make check-doc-examples` target exists as a convenience alias
  - The `.github/workflows/docs.yml` `build` job extracts and `cargo check`s all fenced Rust code blocks in `docs/src/**/*.md`, running on every PR touching `docs/**` and failing the build if any example does not compile
- scope: scripts/check-doc-examples.sh, pre-push hook, docs.yml code-check, Makefile target

## REQ-getting-started-rewrite
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-4 to FR-6)
- description: Full rewrite of the three Getting Started pages.
- acceptance:
  - `installation.md`: minimum supported Rust (current stable; note `edition = "2024"` requires Rust ≥ 1.85 and that **no `rust-toolchain.toml` is present**), system prerequisites, workspace crate names and versions (`paladin-ai-core`, `paladin-ports`, `paladin-battalion`, … at v0.4.3), feature-flag profiles for common use cases, and a compiling verification snippet
  - `quickstart.md`: an end-to-end "hello world" `PaladinBuilder` example that compiles and runs, service startup via `make dev` or `docker compose`, expected terminal output, pointer to `configuration.md`
  - `configuration.md`: complete `config.yml` schema (all top-level sections — `paladin`, `garrison`, `arsenal`, `llm`, and any others present), every key with type, default and one-line description, environment-variable override syntax, multi-environment patterns
- scope: getting-started/{installation,quickstart,configuration}.md

## REQ-architecture-docs-update
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-7 to FR-11)
- description: In-place update of the five Architecture pages to the final Milestone 8 workspace structure.
- acceptance:
  - `overview.md`: three-layer hexagonal architecture; correct crate-to-layer mapping for `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-content`, `paladin-web`, `paladin-notifications`; the inward-only dependency-flow rule; a high-level Mermaid diagram
  - `hexagonal-design.md`: current port trait locations under `crates/paladin-ports/`, current adapter locations per crate, and a step-by-step new-adapter guide using current module paths
  - `domain-model.md`: all domain entities with current module paths, the `Node<T>` pattern, and the Medieval Military naming table matching `copilot-instructions.md`
  - `crate-map.md`: every workspace crate with layer and purpose, a Mermaid dependency graph, feature flags per crate
  - `design-patterns.md`: `PaladinBuilder` with current signatures, `thiserror` error-handling pattern, the `async_trait` + `Send + Sync` port pattern, and service composition
- scope: architecture/{overview,hexagonal-design,domain-model,crate-map,design-patterns}.md
- note: the crate list here names **nine** crates and omits `paladin-herald`, which the run-4 reconciliation created. Same omission as `REQ-workspace-publish-order`.

## REQ-user-guides-rewrite
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-12 to FR-17)
- description: Full rewrite of six User Guide pages with working examples.
- acceptance:
  - `paladin-agents.md`: current `PaladinBuilder` fluent API, `PaladinExecutionService`, `PaladinStatus` lifecycle, working end-to-end example
  - `battalion-patterns.md`: Formation, Phalanx, Campaign, Chain of Command and Commander, each with current module paths and a working example
  - `arsenal-tools.md`: MCP STDIO and SSE adapters, `ArsenalPort`, tool-discovery lifecycle, `config.yml` configuration, working example
  - `garrison-memory.md`: in-memory and SQLite garrison adapters, `GarrisonPort` methods, memory lifecycle, working example
  - `sanctum-vector-memory.md`: the Sanctum vector store (qdrant-client v1.14), current adapter, configuration, semantic-search usage, working example requiring a running Qdrant from the dev container, with a `> **Prerequisites:** Run \`make dev\` first` callout
  - `herald-output.md`: the output formatting system, available formatters, working example
- scope: user-guides/{paladin-agents,battalion-patterns,arsenal-tools,garrison-memory,sanctum-vector-memory,herald-output}.md

## REQ-deployment-operations-docs-update
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-18 to FR-25)
- description: In-place update of the four Deployment and four Operations pages against the current dev environment.
- acceptance:
  - `deployment/docker.md`: `docker/docker-compose.dev.yml`, `Dockerfile`, `Dockerfile.chef`, `make dev`, `make services-up`, current health checks
  - `deployment/kubernetes.md`: manifests in `k8s/` and Battalion workload scaling
  - `deployment/production.md`: production configuration checklist, secret management, TLS, resource tuning
  - `deployment/cicd.md`: the current `.github/workflows/docs.yml` and every other workflow present under `.github/workflows/`
  - `operations/logging.md`: `tracing`/`log` setup, log-level configuration, structured log format, aggregation recommendations
  - `operations/monitoring.md`: Sentinel integration, health-check endpoints, alerting recommendations
  - `operations/performance-tuning.md`: benchmark results from `benches/`, Tokio runtime tuning, Phalanx concurrency limits, DB/queue pooling
  - `operations/troubleshooting.md`: common error scenarios with current error types and recovery steps
- scope: deployment/*.md, operations/*.md
- note: **this requirement plus the six remaining user-guide updates are the substance of Milestone 11's 26 open checkbox items** (`tasks-content-rewrite.md` tasks 6.0 and 7.0 plus 1.2). All target files exist; whether their content is current cannot be settled by file existence.

## REQ-api-reference-contributing-rewrite
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/prd-content-rewrite.md (FR-26 to FR-32)
- description: Rewrite/update the three API Reference and four Contributing pages against v0.4.3.
- acceptance:
  - `stable-api.md`: full rewrite. **"The current file is a merge of `STABLE_API.md` and `VERSIONING_POLICY.md` from before v0.4.3"** — must reflect current stability guarantees and versioning policy
  - `feature-flags.md`: all current Cargo feature flags with defaults and what they enable, **sourced directly from `Cargo.toml`**
  - `migration-guide.md`: a migration section for every breaking change since the last stable release
  - `development-setup.md`: full rewrite (was the root `CONTRIBUTING.md` pre-workspace-restructuring) — dev container, current `make` targets, Clippy `-D warnings`, pre-commit hooks
  - `testing-guide.md`: unit tests, integration tests (`make test-all`, `make test-integration-docker`), doc tests
  - `architecture-decisions.md`: in-place update, was `adapter-development.md` — current adapter locations and port trait contracts
  - `contributing-providers.md`: current LLM provider adapter structure under `crates/paladin-llm/`
  - Explicit non-goals: the **35 appendix files** are reference/archive material and are **not** rewritten; no `*.rs` or `Cargo.toml` changes
- scope: api-reference/*.md, contributing/*.md
- note: FR-26 independently confirms the run-4 finding that root-path `STABLE_API.md` does not exist — it was **merged**, not merely relocated.

## REQ-orchestration-guide
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_4/prd-new-documentation.md (FR-1 to FR-10)
- description: A full rewrite of `user-guides/orchestration.md` covering all six Battalion patterns plus scheduling and events.
- acceptance:
  - Workflow-patterns overview, then Formation (sequential), Phalanx (parallel), Campaign (DAG), Chain of Command (hierarchical) and Commander (dynamic strategy routing) — **each with a working example**
  - Job scheduling and the event/trigger system, each with a working example
  - Links to the standalone bridge guide rather than duplicating it
  - Links to `user-guides/maneuver-flow-dsl.md` rather than duplicating the Flow DSL, which is already documented
- scope: user-guides/orchestration.md, Battalion patterns, scheduling, events

## REQ-content-processing-guide
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_4/prd-new-documentation.md (FR-11 to FR-14)
- description: A new `user-guides/content-processing.md`.
- acceptance:
  - Every available ingestion source; aggregation, filtering and dedup; the processing/analysis pipeline; the content → agent bridge; and delivery
- scope: user-guides/content-processing.md

## REQ-agent-orchestrator-bridge-guide
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_4/prd-new-documentation.md (FR-15 to FR-19)
- description: A new standalone `user-guides/agent-orchestrator-bridge.md` covering both directions.
- acceptance:
  - Agents triggering orchestration (the Milestone 9 Epic 4 `OrchestratorPort`) and orchestration invoking agents (the Milestone 9 Epic 3 content processors)
  - Configuration examples and **at least four use-case recipes**
  - Linked from the orchestration guide
- scope: user-guides/agent-orchestrator-bridge.md

## REQ-crate-map-feature-flag-reference
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_4/prd-new-documentation.md (FR-20 to FR-21, Goals 5-8)
- description: A new consolidated `api-reference/crate-map.md`.
- acceptance:
  - Full workspace crate table, a Mermaid dependency graph, every feature flag, and **at least three consumer-profile `Cargo.toml` snippets**, cross-linked with `architecture/crate-map.md`
  - All new pages registered in `docs/src/SUMMARY.md`
  - Every fenced Rust example passes `cargo check` via `scripts/check-doc-examples.sh` (pre-push hook + `docs.yml` CI); every `config.yml`/YAML snippet validates via an extended config-check step
  - `mdbook build` succeeds with zero warnings with `[output.linkcheck]` enforcing
  - Explicit non-goal: **"If a documented capability is missing, it is recorded as an open question, not implemented here."**
- scope: api-reference/crate-map.md, SUMMARY.md, doc-example and config gates

## REQ-deployment-topologies-section
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md (FR-1 to FR-14)
- description: A new six-page "Deployment Topologies" section with a decision-matrix landing page.
- acceptance:
  - `deployment-topologies/overview.md`: comparison table + Mermaid decision flowchart + "when to use / avoid" per topology, letting a reader pick in under a minute. **The decision matrix is the single source of routing**
  - `embedded-library.md` (single process): compilable example building and executing one agent, cross-linking `paladin-agents.md`
  - `battalion-orchestration.md` (many agents, one runtime): compilable multi-agent example, cross-linking `orchestration.md` / `battalion-patterns.md`
  - `http-service-host.md`: hosting an agent registry behind Axum by composing `axum` + `PaladinExecutionService`, **with an honest note that `paladin-web::create_app_router` is the user-management API, not an agent endpoint**
  - `queue-worker.md`: a producer enqueuing agent jobs and a worker dequeuing and executing them via the Redis queue adapter, cross-linking the appendix Redis setup
  - `sidecar.md`: composing the HTTP host plus an HTTP client, with an honest **"no built-in IPC/RPC"** callout and guidance on when a sidecar is worth the operational cost
  - New section and all six pages registered in `SUMMARY.md`; every Rust example passes `cargo check`; every YAML snippet validates; `mdbook build` zero-warning; `CHANGELOG.md [Unreleased]` records the section
  - Explicit non-goal: **"A first-class agent-HTTP endpoint or sidecar transport — explicitly out of scope; documented as consumer-composed and recorded as open questions (OQ-2, OQ-3)."**
- scope: docs/src/deployment-topologies/, decision matrix, five topology pages
- note: **this requirement's honest gap statement is what created Milestone 12.** The M12 Epic 1 PRD opens by quoting this page directly — *"Paladin ships no agent-execution endpoint… The agent endpoint is yours to compose"* — and OQ-2/OQ-3 became the milestone. A documentation epic that recorded a capability gap rather than papering over it is the direct cause of the last milestone in the corpus.

## REQ-mdbook-final-review
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_5/prd-publish-and-finalize.md (FR-1 to FR-5)
- description: The Milestone 11 final documentation review gate.
- acceptance:
  - `mdbook build docs/` completes with **zero errors and zero broken links** (linkcheck `warning-policy = "error"`); all HTML present in `docs/book/`
  - Recorded tool-limitation carve-out: linkcheck's non-fatal *"fragment resolution isn't implemented"* notices for `#anchor` links and the mdbook-mermaid version notice **are tool limitations, not content errors, and do not fail the build** — this is the accurate interpretation of "zero warnings"
  - All internal links resolve; no broken relative references in `SUMMARY.md`; external URLs (README badges, Pages URL, repo links) spot-checked; "see also" links correct; `crate-map.md` matches the actual workspace member list and per-crate feature flags
  - `./scripts/check-doc-examples.sh` (compiling `paladin-doc-examples`) and `./scripts/check-doc-config.sh` both pass with zero failures
  - Recorded correction: **"the original Epic text references `cargo test --doc`; that is *not* the project's path — most crates set `doctest = false`. Use the `paladin-doc-examples` + `{{#include}}` mechanism, which is the authoritative and stronger gate."**
  - Every chapter in `SUMMARY.md` has real content (no placeholder-only user-facing pages); Mermaid renders; code blocks are highlighted; tables render; no raw HTML artifacts
- scope: mdbook final review, linkcheck semantics, doc-example and config gates
- note: the `doctest = false` observation is the same workspace fact recorded in runs 3 and 4 as an **open defect** for `paladin-ports`. Here it is treated as a settled project characteristic that the documentation gate routes around. Both framings are preserved.

## REQ-doc-version-sync
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_5/prd-publish-and-finalize.md (FR-6, FR-7)
- description: Reconcile every `0.4.3` documentation reference to `0.5.0` and point `documentation` metadata at Pages.
- acceptance:
  - Update every `0.4.3` (and `"0.4"`-style) reference in `docs/src/**` to `0.5.0`, including the consumer-profile snippets and crate table in `api-reference/crate-map.md` and the Getting Started pages
  - **Do not hand-edit `Cargo.toml` versions** — those are bumped by `make release`
  - Re-run the doc-examples compile gate after editing
  - Root `Cargo.toml` `documentation` field set to `https://df3ndr.github.io/paladin-dev-env/`; docs.rs continues to host generated API docs and the README links both
- scope: docs version sync, Cargo.toml documentation metadata

## REQ-readme-landing-page
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_5/prd-publish-and-finalize.md (FR-8, FR-9)
- description: Replace the 1022-line `README.md` with a concise landing page.
- acceptance:
  - Eleven ordered sections: title + tagline; badge row (CI, crates.io version, docs.rs, MIT licence, MSRV Rust ≥ 1.85 from `edition = "2024"` with no `rust-toolchain.toml`, plus a Pages docs link); one-paragraph description positioning Paladin as a **multi-agent AI orchestration framework, not the old "content processing platform" framing**; a **compile-verified** Quick Example routed through the `paladin-doc-examples` gate — **"No hand-written-only example"**; 5-8 key features; a crate-ecosystem table of the 9 crates consistent with `api-reference/crate-map.md`; documentation links; getting started; project status (0.5.0, stability, changelog); contributing; MIT licence
  - All badges resolve to real URLs; the crate table matches the actual workspace; all internal links point to real files; the Quick Example compiles against the 0.5.0 workspace
- scope: README.md rewrite, compile-verified quick example, crate ecosystem table
- note: the crate table is specified as **9 crates**; ten library crates ship. Third instance of the pre-`paladin-herald` crate count in run 5.

## REQ-m11-v050-release
- source: /workspace/.project/Milestone_11-Documentation-Overhaul-Publish/Epic_5/prd-publish-and-finalize.md (FR-10 to FR-13)
- description: Cut v0.5.0 and publish the book by driving the existing Milestone 10 automation.
- acceptance:
  - `CHANGELOG.md [Unreleased]` gets a complete Milestones **8-11** summary under `### Added`, `### Changed`, `### Fixed`, `### Documentation`; the Documentation subsection must call out the MDBook on Pages, the new orchestration / content-processing / bridge guides, the crate map and feature-flag reference, and that all examples are compile-verified. **Do not hand-date or add the `## [0.5.0]` heading** — `make release` does it
  - A single consolidated **go/no-go checkpoint** before release: doc-examples gate green, config gate green, `mdbook build` clean, `cargo check --workspace` green, README and CHANGELOG ready, branch merged to an up-to-date `main`. Only on all-green does the release proceed
  - `make release VERSION=0.5.0` from an up-to-date `main` runs `release-check`, performs the lockstep bump and pin update via `cargo release version`, finalizes the changelog, commits `chore(release): version 0.5.0`, tags `v0.5.0`, pushes. Prerequisite: `cargo-release` installed; the target refuses to run off `main` or when behind `origin/main`
  - Verify the automation completes: `release.yml` → `verify-tag-source` → test suite → GitHub Release → crates.io publish in dependency order (plus Docker images and binaries); `docs.yml` → MDBook built and deployed to Pages
- scope: CHANGELOG v0.5.0, go/no-go checkpoint, make release, release.yml + docs.yml verification
- note: this is the first requirement in the corpus that **consumes** rather than builds the release automation — Milestone 10 Epic 3's `make release` and `release.yml` are used as-is. The dependency direction M10 → M11 recorded in the run-4 Milestones 8-11 dependency graph as SOFT is confirmed here as real.

---

## Milestone 12 — Web API / HTTP Service Host Topology (v0.6.0)

## REQ-agent-registry
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§4.1)
- description: A shared, thread-safe agent registry in `paladin-web`.
- acceptance:
  - An `AgentRegistry` mapping agent id (`String`) to a **per-agent pair** `(Arc<Paladin>, Arc<dyn PaladinExecutorPort>)`. Per the executor-model decision, **different agents may be backed by different executor instances** (different circuit breakers, RAG, herald, etc.)
  - Safe for concurrent reads during request handling and for runtime mutation, using interior mutability (`RwLock<HashMap<…>>` or an equivalent concurrent map) so a shared `Arc<AgentRegistry>` can be cloned into router state
  - Supports construct-empty, construct-from-an-initial-list, `get(id)`, `list()` (ids + metadata), `insert`, `remove(id)`
  - `get`/`remove` on an unknown id return a clear "not found" signal — **not a panic and not a default**
  - Concurrency guidance: prefer `tokio::sync::RwLock` if provisioning happens while holding the guard, otherwise `std::sync::RwLock`; **do not hold a guard across an `.await`**; document the choice
- scope: paladin-web AgentRegistry, concurrent map, executor pairing
- settled-by: code-verification.md run-5 — `crates/paladin-web/src/agent_registry.rs` ships.

## REQ-agent-execute-endpoint
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§4.2)
- description: `POST /agents/{id}/execute` running an agent via `PaladinExecutorPort`.
- acceptance:
  - Deserializes `ExecuteRequest { input: String }`
  - On a known id, calls `PaladinExecutorPort::execute(&paladin, &input)` and on `Ok(PaladinResult)` returns `200 OK` with `ExecuteResponse` including at least `output: String`, and **should** also surface safe result metadata already on `PaladinResult` — `token_count`, `execution_time_ms`, `loop_count`, `stop_reason`
  - Unknown id → `404`; missing/invalid body → `400`; `Err(PaladinError)` → **`502 Bad Gateway`, not `500`** (upstream/LLM/execution failure) with the error message
  - **The handler must not `unwrap()`/`expect()`/`panic!` on any request-driven path**
  - Open Question 4 default: a single `502` for all `PaladinError` variants in Epic 1; refine with the unified error model in Epic 4
- scope: POST /agents/{id}/execute, ExecuteRequest/ExecuteResponse, status-code contract

## REQ-agent-discovery-endpoints
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§4.3)
- description: `GET /agents` and `GET /agents/{id}` returning secret-free summaries.
- acceptance:
  - `GET /agents` returns a JSON array of agent summaries, each with `id` and safe metadata derived from `PaladinData` (`name`, `model`, a `description`/system-prompt-derived summary)
  - `GET /agents/{id}` returns the single summary, or `404`
  - **Must not include secrets, credentials, or full provider configuration**; must not expose the raw system prompt if sensitive
  - Open Question 1 default: return a short `description` and omit the raw prompt
- scope: GET /agents, GET /agents/{id}, AgentSummary, secret exclusion

## REQ-agent-runtime-registration
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§4.4)
- description: `POST /agents` and `DELETE /agents/{id}` mutating the registry at runtime.
- acceptance:
  - `POST /agents` accepts an `AgentSpec` and returns `201 Created` with the new agent's summary
  - Duplicate id → `409 Conflict`; invalid spec → `400`; provision failure → `422 Unprocessable Entity`
  - `DELETE /agents/{id}` → `204 No Content`, or `404`
  - **If no `AgentProvisioner` is wired into router state, `POST /agents` must fail closed** with `501 Not Implemented` / `503` rather than panicking. Discovery and execute remain functional without a provisioner
  - Open Question 3 default: the id is client-supplied and required; `409` on duplicate
- scope: POST /agents, DELETE /agents/{id}, AgentSpec, fail-closed registration

## REQ-agent-provisioner-port
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§4.4 FR-15, OQ-2)
- description: An `AgentProvisioner` abstraction so `paladin-web` can register agents without building them.
- acceptance:
  - `#[async_trait] pub trait AgentProvisioner: Send + Sync { async fn provision(&self, spec: &AgentSpec) -> Result<(Paladin, Arc<dyn PaladinExecutorPort>), ProvisionError>; }`
  - Rationale: **`paladin-web` cannot itself build a `Paladin`** — that needs an `LlmPort` and the builder, which live behind the facade. The registry/handler calls `provision(&spec)` to materialize the pair, then inserts it
  - The concrete implementation lives in the composition root (Epic 2's binary) and **is the only place that touches the facade/builder**
  - Open Question 2 default: keep the trait in `paladin-web` (single consumer today); promote to `paladin-ports` only if a second consumer (sidecar/worker topology) appears. Either placement is clean since it references `Paladin` + `PaladinExecutorPort`, both already in core/ports
- scope: AgentProvisioner port, ProvisionError, composition root
- note: **Open Question 2 has no recorded answer.** Where the port lives determines whether the queue/worker and sidecar topologies can reuse it.

## REQ-paladin-web-no-facade-dep
- source: /workspace/.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md (§1 Architectural seam, Goal 5, §8 Success Metric 2)
- description: The dependency-flow invariant governing the whole of Milestone 12.
- acceptance:
  - `paladin-web` is an infrastructure/adapter crate depending on `paladin-ports` and `paladin-core` but **not** on the `paladin-ai` facade that contains the concrete `PaladinExecutionService`
  - Registry and handlers depend **only on the `PaladinExecutorPort` trait** (`paladin-ports::output::paladin_executor_port`) and the `Paladin` entity (`paladin-core`)
  - The concrete `PaladinExecutionService` — which already implements `PaladinExecutorPort` — is injected at composition time by the server binary
  - Success metric: `cargo tree -p paladin-web` shows **no `paladin-ai` facade dependency**
  - Re-asserted by Epic 5 FR-17: **"`paladin-web` must not gain a dependency on the `paladin-ai` facade (the JWT `AuthPort` implementation is injected by the binary, mirroring the executor/provisioner seam)."**
- scope: dependency-flow rule, PaladinExecutorPort seam, composition root injection
- note: this is the strongest architectural invariant introduced in run 5 and the clearest **SPEC candidate** in the run. It is stated three times across two Epics and has a mechanical verification command. See `constraints.md`.

## REQ-host-agents-config-schema
- source: /workspace/.project/Milestone_12-Web-API/Epic_2/prd-configurable-web-host-server-binary.md (§4.1, Goals 1-2)
- description: `host` and `agents` sections loaded through the existing `Settings` system.
- acceptance:
  - A `host` section (bind address) and an `agents` list load via the existing `config.yml` + `APP_*` env-override mechanism
  - **API keys come from env, never the config file**
  - A consumer can start a Paladin agent HTTP service with **only** a `config.yml` and the `paladin-server` binary — **writing no Rust**
- scope: config.yml host section, agents section, Settings, env overrides

## REQ-registry-from-config-builder
- source: /workspace/.project/Milestone_12-Web-API/Epic_2/prd-configurable-web-host-server-binary.md (§4.2)
- description: A builder turning the `agents` config into a populated `AgentRegistry`.
- acceptance:
  - Each configured agent is backed by an LLM provider resolved through the existing provider factory and executed via `PaladinExecutionService`
  - Explicit non-goal: **Garrison (memory) and Arsenal (tools/MCP) wiring for agents is a later enhancement; agents are LLM + prompt only here**
- scope: AgentRegistry builder, LLM provider factory, PaladinExecutionService

## REQ-concrete-agent-provisioner
- source: /workspace/.project/Milestone_12-Web-API/Epic_2/prd-configurable-web-host-server-binary.md (§4.3)
- description: A concrete `AgentProvisioner` in the facade using the same logic as config load.
- acceptance:
  - Lives in the `paladin-ai` facade so `POST /agents` can build and register agents at runtime using the same path as startup config load
- scope: concrete AgentProvisioner, paladin-ai facade, runtime registration

## REQ-paladin-server-binary
- source: /workspace/.project/Milestone_12-Web-API/Epic_2/prd-configurable-web-host-server-binary.md (§4.4-4.6, Goals 5-7)
- description: The `paladin-server` binary: load → build → compose → bind → serve.
- acceptance:
  - Loads config → builds the registry + provisioner → composes `agent_router` → binds and serves with **graceful shutdown on SIGINT/SIGTERM**
  - Startup **fails fast** with actionable errors on invalid config; on success logs the bound address and the served routes
  - Config is **read once at startup** — hot-reload of `config.yml` is an explicit non-goal
  - The server binds **plain HTTP**; TLS is expected to be terminated by a proxy/ingress (Epic 7 scope, and out of scope there too)
  - Unit tests plus a smoke integration test
- scope: src/bin/paladin-server.rs, graceful shutdown, startup validation
- settled-by: code-verification.md run-5 — `src/bin/paladin-server.rs` ships with `[[bin]] name = "paladin-server"` and `required-features = ["web-server"]`.

## REQ-execute-stream-service
- source: /workspace/.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md (§4.1-4.3, Goals 2-3)
- description: Real streaming on `PaladinExecutionService` threaded through the registry without changing `PaladinExecutorPort`.
- acceptance:
  - `PaladinExecutionService` gains a working `execute_stream` built on `LlmPort::generate_stream`
  - A streaming executor port is added in `paladin-ports`; registry/state threading is **additive**
  - **`PaladinExecutorPort` and all Epic 1/2 buffered behaviour are unchanged**
  - Agents without a streaming backend **degrade gracefully**
- scope: execute_stream, LlmPort::generate_stream, streaming executor port, additive threading

## REQ-sse-streaming-endpoint
- source: /workspace/.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md (§4.4, Goal 1)
- description: `POST /agents/{id}/execute/stream` emitting real incremental tokens over SSE.
- acceptance:
  - Streams real, incremental LLM tokens to the client over Server-Sent Events, ending with a **terminal event carrying final metadata**
  - Uses the interim `{ "error": ... }` body plus an SSE error event until Epic 4's unified model lands
  - Explicit non-goal: **WebSocket / bidirectional streaming — SSE only**
- scope: POST /agents/{id}/execute/stream, SSE, terminal metadata event

## REQ-execution-timeout-cancellation
- source: /workspace/.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md (§4.5, Goal 4)
- description: A resolved, clamped timeout that actually cancels work, on every execution path.
- acceptance:
  - Every execution path — buffered, streaming, job — honours a timeout resolved as **request → agent → config-default**, clamped to a server maximum
  - On expiry the underlying work is **cancelled** and a `504`-style error returned
- scope: timeout resolution chain, cancellation, 504

## REQ-async-jobs-api
- source: /workspace/.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md (§4.6, Goal 5)
- description: In-process fire-and-poll job execution.
- acceptance:
  - `POST /agents/{id}/jobs` enqueues an in-process job and returns a job id
  - `GET /agents/{id}/jobs/{job_id}` reports status and, when finished, the result
  - Explicit non-goal: **"Distributed / durable jobs, retries, or backpressure — that is the queue/worker topology; Epic 3 jobs are in-process and ephemeral."**
- scope: POST /agents/{id}/jobs, GET job status, in-process job store
- settled-by: code-verification.md run-5 — `crates/paladin-web/src/job_store.rs` ships.
- note: the milestone overview marks Epic 3 Task 3.3 (async job execution) **optional/stretch**; the Epic 3 PRD promotes it to a goal. Both positions recorded; the tree implements it.

## REQ-api-error-envelope
- source: /workspace/.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md (§4.1, Goals 1, 5)
- description: One structured error type rendering every failure across all `paladin-web` controllers.
- acceptance:
  - Every failure renders as `{ "error": { "code", "message", "details" } }` with the correct HTTP status
  - **The SSE `error` events and all handlers use the same envelope**; the interim per-controller `ok_body`/`error_body` helpers are **removed** in favour of the shared model
  - Applies across the agent controller, the user-management controller and the content-delivery controller
- scope: ApiError, unified error envelope, controller consolidation
- settled-by: code-verification.md run-5 — `crates/paladin-web/src/error.rs` ships.

## REQ-health-ready-endpoints
- source: /workspace/.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md (§4.2, Goal 2)
- description: `GET /health` and `GET /ready` returning structured JSON for k8s probes.
- acceptance:
  - `/health` is liveness, `/ready` is readiness; both return structured JSON suitable for Kubernetes probes
  - Re-asserted by Epic 5 FR-11: **both must remain unauthenticated regardless of auth config** — "probes must not require credentials"
- scope: GET /health, GET /ready, k8s probes, unauthenticated

## REQ-request-logging-request-id
- source: /workspace/.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md (§4.3, Goal 3)
- description: One log line per request with a client-visible request id.
- acceptance:
  - Every request logged **once** with method, path, status, latency and request-id, via `log`
  - The request-id is surfaced to the client as an `x-request-id` response header
  - Explicit non-goals: **no metrics/Prometheus, no distributed-tracing exporters, no new `tracing` backend, and no migration of the workspace off `log` to `tracing`**
- scope: request logging middleware, x-request-id
- note: Epic 5 FR-13 extends this — the logger **must redact `Authorization` and `X-API-Key`**; "it already logs no headers/bodies — keep it that way."

## REQ-cors-body-limit-timeout
- source: /workspace/.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md (§4.4, Goal 4)
- description: CORS, body-size limit and a global request timeout applied uniformly.
- acceptance:
  - CORS layer, request body-size limit and a global request timeout are configurable and applied uniformly
  - **Without breaking long-lived SSE streaming** — the global timeout layer is additive and must not interfere with streaming or with Epic 3's per-execution timeouts
- scope: CORS, body-size limit, global timeout, SSE compatibility

## REQ-rate-limiting
- source: /workspace/.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md (§4.5)
- description: A basic global IP-based rate limiter, off by default.
- acceptance:
  - Configurable and **off by default**, implemented via `tower-governor`
  - **IP-based, not identity-based**; explicit non-goal: per-route / per-agent rate limits or quotas
  - Epic 5 restates the boundary: "Epic 4's rate limiter remains IP-based"; **API-key storage backends, rotation and per-key rate limits are out of scope**
- scope: tower-governor, IP rate limiting, off-by-default
- settled-by: code-verification.md run-5 — `tower_governor = { version = "0.8", features = ["axum"] }` in `crates/paladin-web/Cargo.toml`.

## REQ-api-key-auth
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.1 FR-1, FR-3, FR-4)
- description: API-key authentication via the `X-API-Key` header.
- acceptance:
  - Accepts an API key via `X-API-Key`, resolved by **constant-time compare** against a configured map of key → principal `{ name, role }`
  - When both credential headers are present, a **deterministic, documented precedence** applies (e.g. `Authorization` bearer first, then `X-API-Key`)
  - Success attaches a unified `Principal { id: String, role: UserRole }` to the request extensions for downstream authorization; failure returns `401` as `ApiError`
  - Open Question 1 default: a single role per key (`{ key, name, role }` per entry)
  - Open Question 2 default: reuse `UserRole` for both principal kinds, parsing role strings from config; **an unknown role string is a startup error**
- scope: X-API-Key, constant-time compare, Principal, UserRole

## REQ-jwt-bearer-auth-v2
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.1 FR-2, §6 Credentials, §7, OQ-4)
- description: **Variant v2** — `Authorization: Bearer <jwt>` verified through `AuthPort::verify_token`.
- acceptance:
  - "The system **must** accept a **JWT** via `Authorization: Bearer <token>`, verified through the existing `AuthPort::verify_token`, yielding `AuthClaims { user_id, role }`. JWT is available only when an `AuthPort` verifier is configured."
  - Config: `http.auth.jwt.enabled` — "use the AuthPort bearer path (verifier wired by the binary)"
  - `AgentAuthConfig { enabled, api_keys: HashMap<String, Principal>, jwt: Option<Arc<dyn AuthPort>> }`
  - `paladin-server` "constructs an `AuthPort` JWT verifier when configured (injecting the facade's implementation)"
  - **Open Question 4 is unanswered:** "which concrete `AuthPort` impl does `paladin-server` wire, and what does it need (signing secret/algorithm) from config/env? (Confirm the available adapter during implementation.)"
- scope: JWT bearer auth, AuthPort::verify_token, AgentAuthConfig.jwt, paladin-server wiring
- note: **v2 of a competing pair with `REQ-opaque-bearer-token-adapter-v1` (Milestone 9 Epic 5), which lists "JWT/OIDC/OAuth or any external identity provider integration" as an explicit non-goal and chose opaque tokens specifically to avoid a `jsonwebtoken` dependency and signing-key management.** See INGEST-CONFLICTS.md WARNINGS.
- settled-by: code-verification.md run-5 — **neither variant wins cleanly.** `crates/paladin-web/src/agent_auth.rs` implements the v2 *shape* (`jwt: Option<Arc<dyn AuthPort>>`, bearer-first precedence, constant-time API-key compare) but **no `jsonwebtoken` dependency exists anywhere in the workspace** and the only shipped `AuthPort` implementation is `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — v1's opaque, in-process, hashed-token store. The API is **documented as JWT and implemented as opaque tokens.** OQ-4 is unanswered because it is unanswerable for the shipped adapter: there is no signing secret or algorithm.

## REQ-fail-closed-auth-posture
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.2, Goal 4, OQ-3)
- description: Auth required by default, disable-able, and fail-closed when misconfigured.
- acceptance:
  - Configurable under `http.auth`: `enabled` (**default `true`**), an `api_keys` list, and JWT settings
  - When `enabled` and **no** credential source is configured (no API keys and no JWT verifier), the server **fails closed** — refuses to serve protected routes with a clear error telling the operator to configure credentials or set `enabled: false`
  - When `enabled: false`, agent routes serve unauthenticated (intended for trusted/dev environments) and this **must be logged as a warning at startup**
  - Recorded posture rationale: "The agent HTTP API has been **intentionally unauthenticated** through Epics 1-4 — anyone who can reach the port can run, register, or delete agents. That is unacceptable for a real deployment: agent execution spends money (LLM calls) and runtime registration is a powerful capability."
  - Open Question 3 default: `enabled: false` is **permitted in release builds but loudly warned**, rather than debug-only
- scope: http.auth.enabled, fail-closed startup validation, dev escape hatch

## REQ-per-agent-role-authorization
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.3 FR-8, FR-10, §7)
- description: Optional per-agent `allowed_roles` restricting invocation.
- acceptance:
  - Each agent **may** declare `allowed_roles`; on `execute`, `execute/stream` and `jobs` the caller's role must be in it. **Empty or absent ⇒ any authenticated caller.** A disallowed role returns `403` as `ApiError`
  - Discovery (`GET /agents`, `GET /agents/{id}`) and `GET /agents/{id}/jobs/{job_id}` require authentication (any role) when auth is enabled
  - `AgentEntry` gains `allowed_roles: Vec<UserRole>` (or `Vec<String>` parsed to roles); `AgentDefinition` and `AgentSpec` gain it; the config builder and runtime provisioner carry it **through the same seam as `timeout_secs`**; `AgentApiState` gains `auth: AgentAuthConfig`
  - Explicit non-goal: fine-grained scopes/permissions beyond `allowed_roles` plus the admin gate
- scope: allowed_roles, AgentEntry, AgentDefinition, AgentSpec, AgentApiState

## REQ-admin-gated-registration
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.3 FR-9, §4.5 FR-15)
- description: An admin gate on runtime registration and deregistration.
- acceptance:
  - `POST /agents` and `DELETE /agents/{id}` **require an admin role**; non-admin authenticated callers get `403`
  - Authentication is applied as a **layer/middleware over the protected agent routes (not the health routes)**, composed in `with_http_layers` / the router so it runs before handlers
  - The per-agent and admin authorization checks run **in the handlers (or a thin extractor)** using the attached principal and the agent's `allowed_roles`
- scope: admin gate, auth layer composition, with_http_layers

## REQ-secret-hygiene-redaction
- source: /workspace/.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md (§4.4 FR-12 to FR-14, §4.6 FR-19)
- description: No credential, token or prompt reaches a response body or a log line.
- acceptance:
  - Discovery responses must not include the raw system prompt, API keys, JWTs or provider configuration (already true for prompts — reconfirm and test)
  - **No log line may contain an API key or JWT**; the request logger redacts `Authorization` and `X-API-Key`
  - Auth/authz error messages **must not echo the supplied credential**
  - Test matrix: unauthenticated → `401`; invalid key/JWT → `401`; valid API key → success; valid JWT → success; role not in `allowed_roles` → `403`; non-admin register/deregister → `403`; admin register → success; health/ready reachable without a credential; **plus a redaction test proving a key/token does not appear in the logged line or responses**
  - Explicit non-goal: **"Encrypting config at rest — secrets management is the operator's responsibility (as with LLM keys)."** API-key values should come from env/secret indirection in practice, not committed config
- scope: secret hygiene, header redaction, discovery response filtering, redaction test

## REQ-openapi-spec-generation
- source: /workspace/.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md (§4.1, Goals 1, 4)
- description: A code-generated OpenAPI 3.x document — no hand-maintained duplicate.
- acceptance:
  - `GET /openapi.json` returns a valid OpenAPI 3.x document describing every agent-API route, its request/response DTOs, the error envelope and the security schemes, when docs are enabled
  - **The spec is generated from code** via `utoipa` / `utoipa-axum` / `OpenApiRouter`
  - Explicit non-goal: **"Auth on the docs endpoints — the contract is public; values/secrets are never in the spec."**
- scope: utoipa, OpenApiRouter, GET /openapi.json, security schemes

## REQ-swagger-ui-docs-endpoint
- source: /workspace/.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md (§4.2, Goal 5)
- description: An interactive Swagger UI at `/docs`, disable-able in production.
- acceptance:
  - `GET /docs` serves an interactive Swagger UI backed by the generated spec
  - Exposure is controlled by `http.docs.enabled` (**default true**) so production can disable it
- scope: GET /docs, Swagger UI, http.docs.enabled

## REQ-api-v1-versioning
- source: /workspace/.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md (§4.3, Goal 3)
- description: The agent API served under `/v1` with a written stability policy.
- acceptance:
  - The agent API is served under `/v1`; operational and docs endpoints remain **unversioned**
  - A written stability policy explains what `/v1` guarantees and how a `/v2` would arrive
  - Explicit non-goal: **versioning the pre-existing user-management and content-delivery routes — they keep their paths**
- scope: /v1 prefix, versioning policy, unversioned operational endpoints
- note: this **relocates the entire Epic 1 route surface.** Epic 1 §6 tabulates five routes at `/agents/…`; Epics 3, 4 and 5 all cite those unprefixed paths. Epic 6 is the later position. Recorded, not resolved — see INGEST-CONFLICTS.md WARNINGS.

## REQ-openapi-drift-guard
- source: /workspace/.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md (§4.4, Goal 4)
- description: A CI test that fails when the committed spec baseline no longer matches the generated one.
- acceptance:
  - A committed `openapi.json` baseline plus a drift-guard test that **fails CI** when the generated spec diverges
  - Explicit non-goals: multiple spec versions / `/v2`; client SDK generation or publishing (consumers run their own codegen); full OpenAPI expression of SSE event framing (describe the stream endpoint's content type at a high level)
- scope: openapi.json baseline, drift-guard test
- settled-by: code-verification.md run-5 — `crates/paladin-web/openapi.json` ships as the committed baseline.

## REQ-dockerfile-server-compose
- source: /workspace/.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md (§4.1, Goals 1-2)
- description: A minimal container image and compose service for `paladin-server`.
- acceptance:
  - `Dockerfile.server` builds a minimal `debian:12-slim` image running `paladin-server` and responding on `/health`
  - A docker-compose service runs the server with config and secrets injected
  - Explicit non-goals: **multi-arch / static-musl images (single `debian:12-slim` amd64; multi-arch deferred); TLS termination / ingress (a proxy concern, documented not implemented); publishing images to a registry**
- scope: Dockerfile.server, docker-compose service, debian:12-slim

## REQ-k8s-manifests
- source: /workspace/.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md (§4.2, Goal 2)
- description: Plain Kubernetes Deployment and Service manifests with probes.
- acceptance:
  - `k8s/` Deployment + Service run the server with config + secrets injected and **liveness/readiness probes pointing at `/health` and `/ready`**
  - Explicit non-goal: **Helm charts / Kustomize overlays — plain manifests only**
- scope: k8s Deployment, Service, ConfigMap/Secret, probes
- settled-by: code-verification.md run-5 — `k8s/` ships `deployment.yaml`, `service.yaml`, `configmap.yaml`, `secret.yaml.example`, `namespace.yaml`, a `server/` subdirectory, plus `redis.yaml` and `minio.yaml`.
- note: a Kubernetes **Deployment** implies more than one replica over time. Combined with `REQ-opaque-bearer-token-adapter-v1`'s recorded trade-off — "a multi-process deployment would later need a shared store" — this is the point at which the in-process token store becomes an operational constraint. Neither document connects the two.

## REQ-deployment-topology-doc-update
- source: /workspace/.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md (§4.3, Goal 3)
- description: Replace the "compose your own endpoint" framing with the shipped API.
- acceptance:
  - The deployment-topology docs accurately describe the **shipped** API and server binary, **replacing the old "compose your own endpoint" framing**
  - The topology overview table is updated
- scope: docs/src/deployment-topologies/, http-service-host.md, overview.md
- settled-by: code-verification.md run-5 — the stale framing is gone. Greps for "ships no agent-execution", "yours to compose", "compose your own" and "does not run agents" return **zero matches** across `docs/src/`; `http-service-host.md` references `paladin-server` four times. **The Milestone 11 Epic 6 gap statement was closed by Milestone 12 in both code and docs.**

## REQ-server-e2e-tests
- source: /workspace/.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md (§4.4-4.5, Goals 4-5)
- description: A runnable example plus an end-to-end suite against the real assembled server.
- acceptance:
  - A runnable `examples/` program (and a compile-tested `doc-examples` snippet) boots the server from a sample config and exercises an agent
  - An end-to-end suite boots the **real assembled server** and asserts auth, buffered **and** streaming execution, async jobs, health/readiness, the error envelope, and the served spec/UI
  - Explicit non-goal: **"The spawn-the-binary e2e mode — in-process only this epic."**
- scope: examples/, doc-examples snippet, end-to-end test suite

## REQ-m12-v060-release
- source: /workspace/.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md (§4.6, Goals 6-7)
- description: Milestone 12 finalization at v0.6.0.
- acceptance:
  - All workspace crates at **0.6.0**; `CHANGELOG.md` and `project/current-exports.txt` reflect the release
  - `cargo test`, `fmt`, `clippy -D warnings`, `make deny` / `make audit` pass; the container image builds
  - Explicit non-goals: **publishing images or a crates.io release (release automation is its own milestone — this epic produces the artifacts, not the publish); no new API features — "artifacts/docs/tests/release only; no behavior changes to the API."**
- scope: v0.6.0 lockstep bump, CHANGELOG, current-exports.txt
- settled-by: code-verification.md run-5 — root `Cargo.toml` is at `version = "0.6.0"`.
- note: **`project/current-exports.txt` is the stale pre-`.project`-rename path.** Milestone 12 Epics 1, 5, 6 and 7 all name it. This extends `DEBT-01` — the count of stale references to a path renamed in commit `928c6d5` now spans two scripts, three `ci.yml` lines, one Milestone 8 requirement and four Milestone 12 requirements.

---

## Deferred-QA-CICD-Completion — the terminal forward-scope register

## REQ-ci-cli-snapshot-job
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.3)
- description: A `cli-tests` CI job running the 43 `insta` CLI snapshot tests.
- acceptance:
  - A job named `cli-tests` (or a step in `test`) runs `cargo test --test cli`, covering all snapshot tests in `tests/cli/` — table, progress, error and help output, **43 total**
  - Runs on every push and PR to `main`/`develop`, on `stable`, with the same cargo cache config as other jobs, failing the pipeline on any snapshot failure
  - **Requires no external services** — CLI tests are self-contained
  - If a separate job, runs in parallel with `lint` and `test` (no `needs:`)
- scope: ci.yml cli-tests job, insta snapshot tests
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** No `cli-tests` job exists in `ci.yml`; its 14 jobs are `lint`, `security-audit`, `cargo-deny`, `osv-scanner`, `api-surface`, `test`, `crate-isolation`, `integration-tests`, `security`, `docker`, `e2e-tests`, `benchmark`, `benchmark-regression-signal`, `publish-dry-run`.

## REQ-ci-bench-check-job
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.4)
- description: A `bench-check` CI job compiling benchmarks without running them.
- acceptance:
  - A job named `bench-check` runs `cargo bench --no-run`, catching API breakage and benchmark bitrot
  - Runs on every push and PR to `main`/`develop` on `stable`, failing on compile failure, **without executing benchmarks** (no performance numbers, no Criterion output), in parallel with other jobs
  - **The existing `benchmark` job (full runs on schedule/manual) remains unchanged**
  - Explicit non-goal: benchmark **regression detection** (`critcmp` / `github-action-benchmark`) is a future enhancement
- scope: ci.yml bench-check job, cargo bench --no-run
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** No `bench-check` job exists. Note that a `benchmark-regression-signal` job *does* ship at `ci.yml:531` from Milestone 7 Epic 3 — the future enhancement landed before the prerequisite.

## REQ-ci-combined-coverage-job
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.5, FR-25.9)
- description: A combined unit + integration coverage job uploading to Codecov.
- acceptance:
  - A `coverage` job installs `cargo-llvm-cov` via **`taiki-e/install-action@v2` with `tool: cargo-llvm-cov@0.7.1`** (pre-built binaries, ~30 s versus 3-5 min for `cargo install`), runs `cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info`, uploads to Codecov with `flags: combined` and `fail_ci_if_error: true`, and saves an HTML report as an `actions/upload-artifact@v4` artifact with 14-day retention
  - Runs on every push and PR to `main`/`develop` on `stable`, **starting Redis and MinIO** so integration tests execute during coverage collection
  - `integration-tests.yml`'s existing coverage step is evaluated; **recommended: remove it** to avoid duplicate uploads, since the combined report subsumes it
- scope: ci.yml coverage job, cargo-llvm-cov, Codecov upload, HTML artifact
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** No `coverage` job in `ci.yml`; no `cargo-llvm-cov` reference anywhere in `ci.yml`. `integration-tests.yml:117-123` still runs `cargo install cargo-llvm-cov` and `codecov/codecov-action@v3` — the integration-only path this requirement was to supersede.
- note: Open Question 3 (remove versus retain the `integration-tests.yml` step) has no recorded answer.

## REQ-codecov-config-thresholds
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.6, FR-25.10, Appendix C)
- description: A `.codecov.yml` with a **phased** threshold rollout.
- acceptance:
  - `.codecov.yml` at the repository root with `require_ci_to_pass: true`, `precision: 2`, `round: down`, `range: "70...100"`; `status.project.default` `target: 70%` / `threshold: 2%` / `if_ci_failed: error`; `status.patch.default` `target: 80%` / `threshold: 5%`; PR comment layout `"reach,diff,flags,files"`; and `ignore` covering `tests/**`, `benches/**`, `examples/**`, `migrations/**`, `scripts/**`, `flat/**`
  - **Phased rollout:** Phase 1 (Sprint 1-2) project 70% / patch 80%; Phase 2 (Sprint 3-4) project 74%; Phase 3 (Sprint 5+) project 78%. Each phase change is a single `target:` edit
  - A `CODECOV_TOKEN` repository secret must be configured; without it uploads may fail silently, especially on fork PRs
- scope: .codecov.yml, phased coverage thresholds, CODECOV_TOKEN
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** Neither `.codecov.yml` nor `codecov.yml` exists at the repository root.
- note: **the initial threshold competes with the parent PRD.** `prd-deferred-qa-completion.md` FR-25.3 item 10 mandates "a coverage threshold gate of **78%** minimum. PRs dropping below this threshold must fail." Epic 25 starts at **70%** and ramps. See INGEST-CONFLICTS.md WARNINGS.

## REQ-makefile-coverage-targets
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.7)
- description: Four new Makefile targets plus CI-target updates.
- acceptance:
  - A new **Coverage** section between Testing and Code Quality with `coverage` (`cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info`) and `coverage-html` (`--html --output-dir target/coverage`)
  - In the Testing section: `test-cli` (`cargo test --test cli`) and `bench-check` (`cargo bench --no-run`)
  - `ci-test` updated to include `test-cli`; a new `ci-full: ci-test coverage`
- scope: Makefile coverage/coverage-html/test-cli/bench-check, ci-test, ci-full
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** None of `coverage`, `coverage-html`, `test-cli` or `bench-check` exists in the `Makefile`, and it contains no `llvm-cov` reference.

## REQ-modernize-github-actions
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.1, FR-25.2)
- description: Replace deprecated GitHub Actions across all three workflows and fix the invalid YAML block.
- acceptance:
  - `actions-rs/toolchain@v1` (deprecated and unmaintained) → `dtolnay/rust-toolchain@stable`/`@beta`/`@nightly`, using the `components:` input for `rustfmt`/`clippy`
  - `actions/cache@v3` → `@v4`; `codecov/codecov-action@v3` → `@v4` with `token: ${{ secrets.CODECOV_TOKEN }}`; `actions/checkout@v3` → `@v4` where present
  - Remove the **dangling `on: schedule` block** at `ci.yml` lines ~336-340 — syntactically invalid because the top-level `on:` is already defined at line 3
  - Validate all three workflows with `actionlint`/`yamllint`, zero errors
- scope: workflow action versions, dangling schedule block, actionlint
- settled-by: code-verification.md run-5 — **PARTIALLY OPEN.** The dangling `on: schedule` block is **gone** (`ci.yml` has exactly one `on:` at line 3, no `schedule:`/`cron:`). Still open: **`actions-rs/toolchain@v1` at `ci.yml:147`, `:317`, `:507` and `integration-tests.yml:71`; `actions/cache@v3` at `integration-tests.yml:78`, `:84`, `:90`; `codecov/codecov-action@v3` at `integration-tests.yml:123`.** Eight deprecated-action references remain.

## REQ-contributing-coverage-docs
- source: /workspace/.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md (FR-25.8)
- description: A "Code Coverage" section in `CONTRIBUTING.md`.
- acceptance:
  - Prerequisites (`cargo install cargo-llvm-cov` / `cargo binstall`), local generation (`make coverage`, `make coverage-html`), how to read LCOV and HTML output, Codecov PR-comment behaviour and dashboard link, the phased threshold policy (70 → 74 → 78 project, 80 patch) and what `project` versus `patch` mean, and troubleshooting (tool not found, low patch coverage, upload failures / `CODECOV_TOKEN`)
  - Existing `cargo tarpaulin` references updated to note `cargo-llvm-cov` is the project standard (tarpaulin remains an alternative)
- scope: CONTRIBUTING.md Code Coverage section

## REQ-arch-doc-modernization
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-26.1, items 19-25; G3)
- description: Bring the architecture document up to the current system — the largest single documentation gap in the corpus.
- acceptance:
  - Audit the current `Design_and_Architecture.md` (**311 lines, 10 sections**) and expand to ~600-800 lines
  - Expand the AI Agent System section (currently ~20 lines) to cover **all 15+ components**: brief refresh of Paladin, Garrison, Arsenal, Battalion (Formation, Phalanx, Campaign, Chain of Command), Herald, Citadel; **detailed** coverage of **Commander, Council, Conclave, Grove, Maneuver, Sanctum, Sentinel**
  - Add **Mermaid** diagrams (GitHub-native, no external images) for: overall hexagonal system architecture; Battalion orchestration patterns; data flow through a Paladin execution cycle; Arsenal/MCP tool integration flow
  - Update Data Flow to include the AI agent execution pipeline, not just content processing; update Deployment Architecture (currently marked "Draft"); add a Configuration section covering `config.yml` for LLM providers, Garrison, Arsenal and Sanctum; remove or update stale content-management-heavy framing
  - Success metric: components documented **8 of 15+ → 15+ of 15+**
- scope: Design_and_Architecture.md, 7 undocumented subsystems, 4 Mermaid diagrams
- settled-by: code-verification.md run-5 — **VERIFIED OPEN, and the file was relocated without being rewritten.** The document now ships as `docs/src/appendix/design-and-architecture.md` and is **still exactly 311 lines** — the same figure this requirement cites. It contains **zero** occurrences of Commander, Council, Conclave, Grove, Maneuver, Sanctum or Sentinel and **zero** ```mermaid blocks. Milestone 11 moved it into the mdbook appendix, which its own Epic 3 non-goals exempt from rewriting ("the 35 appendix files are reference/archive material and are not rewritten"). **The relocation placed the corpus's largest documentation gap into the one chapter nobody was required to fix.**
- note (dated 2026-08-10, ORCH-03(d)): two separately labelled facts, not merged. **Relocation — closed:** the move from `docs/Design/Design_and_Architecture.md` (absent, confirmed via `test -f`, re-run this session) to `docs/src/appendix/design-and-architecture.md` (present) is done. **Rewrite — open:** the content gap the `settled-by` line above describes is unresolved. Owner **Phase 16 / DOCS-02**. Matches `.planning/ledgers/milestone-09-12.md` row `REQ-arch-doc-modernization` (D-13(d)).

## REQ-rustdoc-zero-warnings
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-26.2; G5)
- description: Zero rustdoc warnings, enforced in CI.
- acceptance:
  - Run `cargo doc --no-deps 2>&1` and catalog all warnings; fix all of them (**recorded as 12 minor formatting issues**)
  - `cargo doc --no-deps` produces zero warnings
  - Add a `cargo doc --no-deps 2>&1 | grep -c warning` check to CI, failing if > 0
- scope: rustdoc warnings, CI doc-warning gate
- note: Milestone 7 Epic 4 §4.4.3 independently requires zero `cargo doc --workspace --no-deps` warnings, while Milestone 8 Epic 5 FR-19 relaxes the same command to "warnings acceptable; must not fail". Three positions on one command across three milestones — see run-4 `constraints.md`.

## REQ-public-api-doc-audit
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-26.3; G9)
- description: 100% rustdoc coverage of public items in `src/`.
- acceptance:
  - Enumerate all `pub` items in `src/` lacking `///` documentation and document every undocumented public function, struct, enum, trait and type alias
  - Add at least one `/// # Examples` block to **all public API entry points** — builders, service constructors, port traits
  - Verify rendering with `cargo doc --open`
- scope: public API rustdoc coverage, examples on entry points

## REQ-asciinema-demos
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-26.4; G4)
- description: Four terminal demo recordings and an index page.
- acceptance:
  - Record **Basic Paladin Execution** (30-60 s), **Battalion Formation** (45-90 s), **Council Discussion** (60-120 s) and **Grove Routing** (45-90 s) with `asciinema`
  - Save originals to `docs/assets/recordings/` in `.cast`; optionally convert to `.gif`/`.svg` (`asciinema-agg` / `svg-term-cli`) for README embedding, with rendered versions in `docs/assets/`
  - Update `README.md` to embed or link demos in the matching sections; add a `docs/DEMOS.md` index listing all demos with descriptions
  - Recordings require live LLM API keys (OpenAI preferred)
- scope: asciinema recordings, docs/assets, docs/DEMOS.md, README embedding
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** `docs/assets/` exists and is **empty**; `docs/DEMOS.md` does not exist.
- note: Open Question 4 (asciinema versus VHS tape files, Terminalizer or plain GIFs) has no recorded answer. The README was subsequently rewritten by Milestone 11 Epic 5 to a concise landing page, which does not include a demos section — so this requirement's README clause targets a document that has since changed shape.
- note (dated 2026-08-10, ORCH-03(e)): correcting the `settled-by` line above — `docs/assets/` does **not** exist at all (`test -d docs/assets` fails, re-run this session), it is not merely empty as previously recorded. The path that does exist, `docs/src/assets/`, holds six architecture SVGs unrelated to demo content (re-confirmed this session, matches `.planning/ledgers/milestone-09-12.md` row `REQ-asciinema-demos` D-13(e)'s own path correction). `docs/DEMOS.md` absence and the README's changed shape are both still accurate. Owner **Phase 16 / DOCS-04**.

## REQ-llm-tool-calling-port
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-27.1, FR-27.2; G6)
- description: Tool definitions on `LlmRequest` and tool calls on `LlmResponse`.
- acceptance:
  - Add `tools: Option<Vec<ToolDefinition>>` to `LlmRequest`
  - `ToolDefinition { name: String, description: String, parameters: serde_json::Value }` (JSON Schema), deriving `Debug`, `Clone`, `Serialize`, `Deserialize`
  - `LlmResponse.function_call` populated from actual API responses — **currently hardcoded to `None` in all three adapters**
  - Add `tool_calls: Option<Vec<ToolCall>>` to `LlmResponse` for parallel tool calls; `ToolCall { id: String, function: FunctionCall }`
  - The `ToolDefinition` type is **provider-agnostic and defined in the port layer**; provider-specific serialization happens in the adapter layer (hexagonal pattern)
  - Recorded risk: this modifies the `LlmPort` trait and is a **breaking change to the port interface** — all adapters must be updated simultaneously. Phased approach: (1) add `tools` as `Option<…>` (backward compatible, `None` = no tools); (2) implement sending; (3) implement parsing; (4) live API tests
- scope: LlmRequest.tools, ToolDefinition, ToolCall, LlmResponse.tool_calls
- settled-by: code-verification.md run-5 — **VERIFIED OPEN.** `crates/paladin-ports/src/output/llm_port.rs` has no `tools` field; greps for `struct ToolDefinition`, `struct ToolCall` and `tool_calls` across `paladin-ports` and `paladin-llm` return **zero matches**. The only `tools` occurrences are two doc-comment references, one of which reads "// No tools, rely on prompting".
- note: the requirement names the path `src/application/ports/output/llm_port.rs`, which **no longer exists** — `src/application/ports/` was deleted by Milestone 5 Epic 2. The current path is `crates/paladin-ports/src/output/llm_port.rs`. Relocation, not contradiction.

## REQ-llm-tool-calling-adapters
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-27.3 to FR-27.7)
- description: Tool calling implemented and tested across all three LLM adapters.
- acceptance:
  - **OpenAI:** include `tools` in the request body when `LlmRequest.tools` is `Some`; parse `tool_calls` from the response; parse single `function_call` for backward compatibility; handle `finish_reason: "tool_calls"` in addition to `"function_call"`; handle streamed tool-call deltas in `generate_stream()`
  - **Anthropic:** include `tools` in Anthropic format (`tools: [{name, description, input_schema}]`); parse `tool_use` content blocks into `LlmResponse.tool_calls`; handle `stop_reason: "tool_use"`; handle streamed `content_block_start`/`content_block_delta` for tool_use blocks. **The adapter must translate `input_schema` versus `parameters`**
  - **DeepSeek:** investigate support (may mirror OpenAI); implement if supported, otherwise update `ProviderCapabilities` to report `supports_tool_calling: false` accurately and document the limitation
  - Unit tests per adapter for request serialization, response deserialization, multi/parallel tool calls, and edge cases (empty tool list, malformed responses, missing tool call ids); test that `ProviderCapabilities` reflects actual support
  - Live tests `test_openai_tool_calling`, `test_anthropic_tool_calling`, `test_deepseek_tool_calling` in `tests/integration/llm_live_api_tests.rs`, all gated behind `#[cfg(feature = "live-api-tests")]` and `#[ignore]`, skipping gracefully when an API key is absent
  - Problem statement: "All three LLM adapters declare tool-calling capabilities in `ProviderCapabilities` but hardcode `function_call: None`"
- scope: OpenAI/Anthropic/DeepSeek tool calling, ProviderCapabilities accuracy, live-api-tests
- note: Open Question 1 (does DeepSeek's API support tool calling?) and Open Question 5 (OpenAI JSON Schema as canonical versus a provider-agnostic schema) both have no recorded answer. **`ProviderCapabilities` currently over-reports capability** — that is a correctness defect independent of whether tool calling is implemented.

## REQ-mock-infrastructure
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-28.1, FR-29.1)
- description: Reusable mock and async-test infrastructure shared by Epics 28 and 29.
- acceptance:
  - `MockUserRepository` (`UserRepositoryPort`, in-memory `HashMap`), `MockLogPort` (`Vec<LogEntry>` for assertion), `MockNotificationService` (sent-messages vector), placed in `tests/common/mocks/` or `tests/unit/mocks/` for reuse
  - `MockEventSource` (configurable event sequences with controlled timing), `MockTriggerExecutor` (records executions), Tokio time-control utilities (`tokio::time::pause()`/`advance()`), and test event generators, placed in `tests/common/event_testing/`
  - **All mocks must be `Send + Sync`** for async test compatibility; use the `Arc<Mutex<Vec<T>>>` pattern for recording calls in async contexts
  - Design as reusable components, **not per-test one-offs**; a `mod.rs` re-exports all mocks
  - Open Question 2: adopt `mockall` or keep hand-written mocks (compile-time cost versus boilerplate) — **unanswered**
- scope: tests/common/mocks, tests/common/event_testing, Send + Sync mocks
- settled-by: code-verification.md run-5 — **VERIFIED OPEN in the specified shape.** No `tests/common/` directory exists. Mocks live at `tests/helpers/{mock_llm_adapter,mock_arsenal_adapter,mock_paladin_port}.rs` — a different location and a different set. None of `MockUserRepository`, `MockLogPort`, `MockNotificationService`, `MockEventSource` or `MockTriggerExecutor` exists.
- note: **this is the shared prerequisite for Epics 28 and 29** and the reason the recommended execution order puts Epic 28 before Epic 29. `DEFERRED_COVERAGE.md` lists "create reusable mock infrastructure patterns", "document testing best practices" and "establish concurrency testing patterns" as the three **unchecked** prerequisites for all deferred coverage work.

## REQ-user-service-test-coverage
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-28.2 to FR-28.7; G7) + Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md (Module 1)
- description: Raise `user_service.rs` from ~4.23% to ≥ 80% coverage — Epic 28.
- acceptance:
  - Registration: happy path (user persisted, welcome email sent, action logged); duplicate username; duplicate email; invalid username formats (too short, too long, special characters); invalid email; password hashing (hashed ≠ plaintext, valid Argon2)
  - Authentication: correct password; incorrect password; non-existent user; deactivated account; login-attempt tracking increments on failure
  - Profile: update, email change with verification requirement, activation/deactivation, email-verification flow
  - Queries: find by id, by email, by active status, by verification status, user count statistics
  - Edge cases: repository error (database-down simulation); notification failure **must not block registration**; concurrent registration with the same username; Unicode username and password; empty/whitespace-only inputs
  - `cargo llvm-cov` targeting the module verifies **≥ 80%**; intentionally untested paths documented with justification
  - Recorded profile: **488 LOC, ~4.23% coverage, complexity High, production status Active** (used in web controllers and CLI commands). Effort 15-20 h (mock infra 6-8, test suite 8-10, edge cases 1-2). Risk of deferral **Medium** — "Authentication logic is critical security component"
  - Deferral mitigation of record: the service is already exercised through CLI and web-controller integration tests and real dev/staging database interaction; core security is Argon2, a battle-tested library, not custom code
- scope: user_service.rs coverage, Epic 28, Argon2 validation
- settled-by: code-verification.md run-5 — the target module still ships at `src/core/platform/manager/user_service.rs` (19,046 bytes). It is one of only **four** files remaining in `src/core/platform/manager/`, and run 4's `REQ-m8-deferred-items-register` D2 recommends **splitting it** — trait + DTOs to `paladin-core`/`paladin-ports`, concrete impl to a facade app-service. **Two ingested registers propose different next actions on the same file.**

## REQ-listener-service-test-coverage
- source: /workspace/.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (FR-29.2 to FR-29.7; G8) + Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md (Module 2)
- description: Raise the listener service from ~57.83% to ≥ 80% coverage with concurrency and observability work — Epic 29.
- acceptance:
  - Registration: register for a specific event type; multiple listeners on one type; unregister; lifecycle (registered → active → paused → unregistered); complex filter conditions
  - Processing: single delivery to a matching listener; delivery to multiple matching listeners; filtering (non-matching not delivered); batch processing; **ordering guarantees**
  - Triggers: creation from a matched event; status tracking (created → executing → completed/failed); condition evaluation; failure handling and retry; execution coordination via `MockTriggerExecutor`
  - Concurrency and stress: concurrent emission from multiple producers; concurrent registration/unregistration **during** event processing; a high-volume burst of **1000+ events**; **deadlock detection under contention (Tokio `Mutex` + `RwLock` interactions)**; graceful shutdown during active processing; `loom` or manual patterns for race detection
  - Statistics: processing-count metrics, trigger success/failure rate calculation, health-check status
  - `cargo llvm-cov` verifies **≥ 80%**; intentionally untested paths documented
  - Recorded profile: **602 LOC, ~57.83% coverage, complexity Very High, production status Active** (event-driven system core). Effort 20-25 h (mock infra 8-10, async framework 4-6, test suite 8-9). Risk of deferral **Medium-High** — "Event-driven systems are notoriously hard to debug; concurrency bugs can be subtle and intermittent; trigger generation logic is business-critical"
  - Future-plan scope beyond coverage: an event testing framework, property-based tests for filtering, concurrency stress tests, chaos-engineering tests (random failures), and **distributed tracing for event flows**; target 85%+
- scope: listener service coverage, Epic 29, concurrency stress, distributed tracing
- settled-by: code-verification.md run-5 — the module was **relocated, not deleted.** Both this PRD and `DEFERRED_COVERAGE.md` name `src/core/platform/manager/listener_service.rs`, which no longer exists; the code ships as `src/application/services/orchestration/listener.rs` (`ListenerOrchestrator`) after the Milestone 6 Epic 2 relocation. Relocation, not contradiction — but every path in this requirement is stale.
- note: Milestone 9 Epic 2 (`REQ-event-trigger-job-pipeline`) subsequently added match/no-match/fan-out/rate-limit/dispatch tests against this exact module. **Epic 29's stated coverage baseline of 57.83% predates that work and is almost certainly no longer accurate.** Re-measure before planning.
- note (dated 2026-08-10, ORCH-03(b)): the relocation above and the stale-baseline finding are both accurate and already recorded; adding only the owner they were missing — **Phase 15 / DEFER-03**. Matches `.planning/ledgers/milestone-09-12.md` row `REQ-listener-service-test-coverage` (D-13(b)) and the dated correction banner atop `DEFERRED_COVERAGE.md`.

## REQ-deferred-coverage-register
- source: /workspace/.project/Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md
- description: The Epic 24 coverage deferral record — the terminal deferred register of the corpus.
- acceptance:
  - Two modules deferred, **1,090 LOC total (~2.2% of a ~50,000-LOC codebase)**, combined effort **35-45 hours**, coverage impact **−2% to −3%** (78-80% without the deferred modules, 76-77% with; target 75%+, "within acceptable range")
  - Target epics named and scoped: **Epic 28 "Platform Services Test Coverage"** (priority Medium, 2-3 sprints, 2-3 story points / 1 dev week) and **Epic 29 "Event System Test Coverage & Observability"** (priority Medium-High, 2-4 sprints, 3-5 story points / 1.5-2 dev weeks)
  - Coverage goals recorded as **achieved**: Paladin core 85%+, Battalion patterns 80%+, Garrison/Arsenal 75%+, CLI 100% snapshot (43 tests). **Deferred**: platform services 60%+
  - Three **unchecked** prerequisites before tackling deferred coverage: create reusable mock infrastructure patterns; document testing best practices; establish concurrency testing patterns. Three **checked**: Epic 24 complete, snapshot testing patterns established, live API integration tests created
  - Quality maintained without full coverage through: integration tests with real database/service interactions, 43 CLI snapshot tests, benchmark tests, production monitoring, Rust's compile-time guarantees, and code review
  - Sign-off: **"Approved By: AI Coding Agent (Epic 24 execution), Date: February 14, 2026, Epic: 24."** Next Review: **Epic 27 or Epic 28 planning**
- scope: deferred coverage register, Epic 28, Epic 29, prerequisites, sign-off
- note: this is the third and last deferred register in the corpus, after Milestone 8's `deferred-items.md` (D1-D5) and `deferred-features.md`. Run 4 established those two as the highest-fidelity documents in the corpus — every verifiable claim matched the tree exactly. **This one is materially less reliable:** its two module paths are both stale (one relocated, one still present), and its coverage baselines predate Milestone 9's test work. Treat its *scope* as real and its *numbers* as needing re-measurement.

---

## project-management

## REQ-master-plan-epics-11-18
- source: /workspace/.project/project-management/paladin-project-plan-final.md
- description: The master expansion plan defining Epics 11-18 — the origin document for what became Milestone 2.
- acceptance:
  - Eight epics over **14-18 weeks**, each scoped at 2 weeks: **Epic 11** Sanctum Memory Foundation; **Epic 12** Sanctum RAG Integration; **Epic 13** Sentinel Vision System; **Epic 14** Autonomous Agent Features; **Epic 15** Conclave Expert Synthesis; **Epic 16** Advanced Battalion Patterns (Council, Grove); **Epic 17** Tactical Flow DSL; **Epic 18** Armory CLI Enhancement
  - Dependency graph: 11 → 12 → {13, 14} → 15 → {16, 17} → 18
  - Carries user stories (`US-11.1` onward), per-epic completion criteria, a risk assessment, success metrics, a Medieval Military naming table, and a glossary; Epics 1-10 are marked **Complete** in the Summary Epic List
  - Status **Draft**, version 1.0, dated **January 29, 2026** — the earliest document in the run-5 set and the highest-level planning document in the corpus
- scope: Epics 11-18, Sanctum, Sentinel, autonomous agents, Conclave, Council, Grove, Maneuver, Armory CLI
- note: **do not double-count.** Every one of these eight epics was ingested in run 2 from `.project/Milestone_2-Missing_features` as `VERIFY-*`/`CLOSE-*`-era requirements, and `code-verification.md` runs 1-2 verified Conclave, Sanctum (Qdrant), Council, Grove, Maneuver and Sentinel vision as **shipped**. This document is the **origin** of that scope, not new scope. Its value is provenance — it is the only place the 11 → 12 → {13,14} → 15 → {16,17} → 18 dependency graph and the epic-level risk assessment are recorded.
- note: the classifier flagged that the content is "strongly PRD-like (user stories, acceptance criteria, success metrics) with embedded SPEC fragments (trait definitions, struct schemas)" but the manifest types it DOC. Retagging it would not add scope; it would only raise the precedence of positions that shipped a year ago.
