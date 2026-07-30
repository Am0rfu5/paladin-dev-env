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
