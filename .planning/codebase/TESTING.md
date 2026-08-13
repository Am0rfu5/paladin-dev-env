# Testing Patterns

**Analysis Date:** 2026-07-30

## Test Framework

**Runner:**
- `tokio` v1 with `#[tokio::test]` macro for async tests
- Standard `cargo test` for synchronous unit tests
- Config: `Cargo.toml` specifies `[[test]]` blocks for organized test suites

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`, `assert_ne!` macros
- `proptest` v1.4 for property-based testing

**Run Commands:**
```bash
cargo test --workspace --lib --bins           # Unit tests
cargo test --workspace --doc                  # Documentation tests
make test                                     # Alias: unit tests
make test-doc                                 # Alias: doc tests
make test-all                                 # Unit + integration + doc
make test-integration-docker                  # Integration with Docker services
cargo test -p paladin-battalion               # Single crate tests
cargo test --test system_log_integration      # Named test target
```

**Test Organization in Cargo.toml:**
```toml
[[test]]
name = "unit"
path = "tests/unit/mod.rs"

[[test]]
name = "paladin_garrison_integration"
path = "tests/integration/paladin_garrison_integration_test.rs"

[[test]]
name = "vision_integration"
path = "tests/integration/vision_integration_test.rs"
required-features = ["vision", "llm-openai", "llm-anthropic"]
```

## Test File Organization

**Location:**
- **Unit tests**: Co-located in same file as code with `#[cfg(test)]` modules
- **Integration tests**: Separate files in `tests/` directory at project root
- **Functional tests**: `tests/` directory (examples: `tests/functional.rs`, `tests/content_ingestion_pipeline.rs`)
- **Test helpers**: `tests/helpers/` directory (mocks, fixtures)

**Naming:**
- Unit test modules: End of file with `#[cfg(test)] mod tests { ... }`
- Integration test files: `*_test.rs` or `*_integration_test.rs` (e.g., `paladin_garrison_integration_test.rs`)
- Test functions: `test_<behavior_description>` (e.g., `test_paladin_status_transitions()`)

**Directory Structure:**
```
tests/
├── unit/                              # Unit test registry
│   ├── mod.rs                         # Lists all unit test modules
│   ├── paladin_entity_test.rs
│   ├── paladin_builder_test.rs
│   ├── paladin_execution_service_test.rs
│   ├── arsenal_domain_test.rs
│   ├── battalion/                     # Organized by domain
│   │   └── formation_test.rs
│   └── ... (30+ test files)
├── integration/                       # Integration test suite
│   ├── paladin_garrison_integration_test.rs
│   ├── arsenal_execution_integration_test.rs
│   ├── battalion_campaign_integration_test.rs
│   ├── mcp_stdio_test.rs
│   ├── redis_queue_integration_test.rs
│   └── ... (20+ test files)
├── helpers/                           # Shared test utilities
│   ├── mod.rs                         # Barrel file: re-exports
│   ├── mock_llm_adapter.rs           # MockLlmAdapter + variants
│   ├── mock_arsenal_adapter.rs       # MockArsenalPort
│   └── mock_paladin_port.rs          # MockPaladinPort
├── lib.rs                             # Test helpers library root
├── functional.rs                      # End-to-end functional tests
└── ... (additional suites)
```

**Barrel File Example** (`tests/unit/mod.rs`):
```rust
pub mod arsenal;
pub mod arsenal_config_test;
pub mod arsenal_domain_test;
pub mod battalion;
pub mod circuit_breaker_test;
pub mod cli_agent_commands_test;  // Feature-gated
pub mod paladin_builder_test;
pub mod paladin_execution_service_test;
// ... (31 modules total)
```

## Test Structure

**Suite Organization (from** `tests/unit/paladin_entity_test.rs`):
```rust
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};

#[test]
fn test_paladin_status_transitions() {
    // Test state transitions
    let idle = PaladinStatus::Idle;
    let reasoning = PaladinStatus::Reasoning;

    // Verify states
    assert_eq!(idle, PaladinStatus::Idle);

    // Test properties
    assert!(!idle.is_terminal());
    assert!(reasoning.is_active());
}

#[test]
fn test_paladin_data_serialization_roundtrip() {
    let data = PaladinData {
        system_prompt: "You are a helpful assistant".to_string(),
        name: "TestPaladin".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        status: PaladinStatus::Idle,
        ..Default::default()
    };

    // Serialize to JSON
    // Assert round-trip succeeds
}
```

**Async Test Pattern** (`tests/integration/paladin_garrison_integration_test.rs`):
```rust
#[tokio::test]
async fn test_paladin_with_garrison_stores_conversation() {
    // Setup: Create test dependencies
    let config = GarrisonConfig::default();
    let garrison = Arc::new(InMemoryGarrison::new(config)) as Arc<dyn GarrisonPort>;
    let llm_port = Arc::new(MockLlmPort::new("I am a helpful assistant"));

    // Build: Construct system under test
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a coding assistant")
        .name("TestPaladin")
        .model("gpt-4")
        .with_garrison(garrison.clone())
        .build()
        .await
        .expect("Failed to build paladin");

    let service = PaladinExecutionService::new(llm_port, circuit_breaker, Some(garrison.clone()), None);

    // Execute
    let result = service.execute(&paladin, "Hello, can you help me?").await;

    // Assert
    assert!(result.is_ok(), "First execution should succeed");

    let stats = garrison.stats().await.expect("Failed to get stats");
    assert!(stats.entry_count >= 2, "Should have at least user input and assistant response");
}
```

**Patterns:**
- **Setup**: Create mocks, fixtures, initialize test data
- **Execute**: Perform the action under test
- **Assert**: Verify expectations using `assert!` macros
- **Cleanup**: Implicit via RAII and Arc/scope exit

## Mocking

**Framework & Approach:**
- Custom mock implementations (no mocking library dependency)
- Thread-safe shared state using `Arc<Mutex<VecDeque<T>>>`
- Implements actual port traits (`LlmPort`, `GarrisonPort`, `ArsenalPort`, etc.)

**MockLlmAdapter** (`tests/helpers/mock_llm_adapter.rs`):
```rust
/// Mock response types for flexible testing
#[derive(Debug, Clone)]
pub enum MockResponse {
    Text(String),
    ToolCall { tool_name: String, arguments: String },
    Streaming(Vec<String>),
    Error(LlmError),
}

/// Mock LLM adapter with configurable responses
#[derive(Clone)]
pub struct MockLlmAdapter {
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
    invocations: Arc<Mutex<Vec<Invocation>>>,
}

impl MockLlmAdapter {
    pub fn new() -> Self { ... }

    pub fn add_success(&self, content: impl Into<String>) {
        self.add_response(MockResponse::Text(content.into()));
    }

    pub fn add_failure(&self, error: LlmError) {
        self.add_response(MockResponse::Error(error));
    }

    pub fn add_tool_call(&self, tool_name: impl Into<String>, arguments: impl Into<String>) {
        self.add_response(MockResponse::ToolCall { ... });
    }

    pub fn get_invocations(&self) -> Vec<Invocation> {
        self.invocations.lock().unwrap().clone()
    }
}

#[async_trait]
impl LlmPort for MockLlmAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        // Track invocation
        // Return queued response or default
    }
}
```

**Mock Factory Functions** (`tests/helpers/mock_llm_adapter.rs`):
```rust
pub fn create_mock_with_responses(responses: Vec<String>) -> Arc<MockLlmAdapter> {
    let mock = Arc::new(MockLlmAdapter::new());
    for response in responses {
        mock.add_success(response);
    }
    mock
}

pub fn create_mock_with_tool_calls(tool_calls: Vec<(String, String)>) -> Arc<MockLlmAdapter> {
    let mock = Arc::new(MockLlmAdapter::new());
    for (tool_name, args) in tool_calls {
        mock.add_tool_call(tool_name, args);
    }
    mock
}
```

**Invocation Tracking** (`tests/helpers/mock_llm_adapter.rs`):
```rust
#[derive(Debug, Clone)]
pub struct Invocation {
    pub prompt: String,        // The prompt text sent to LLM
    pub model: String,         // The model requested
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}
```

**What to Mock:**
- External services: LLM providers, databases, message queues
- Port implementations: `LlmPort`, `GarrisonPort`, `ArsenalPort`, `SanctumPort`
- Tool dependencies: MCP clients, file systems, HTTP endpoints

**What NOT to Mock:**
- Domain entities: `Paladin`, `Battalion`, `Formation`, `Phalanx`
- Core business logic: Actual orchestration algorithms
- Value objects: Configuration types, enums
- Port trait contracts: Use real implementations when testing cross-layer boundaries

## Fixtures and Factories

**Test Data Location:**
- `tests/helpers/mod.rs` — Barrel file re-exporting factories
- Inline in test files when data is specific to one test
- Shared factories in helper modules (e.g., `create_test_armament()`)

**Factory Pattern** (`tests/integration/arsenal_execution_integration_test.rs`):
```rust
fn create_test_armament(name: &str, required_params: Vec<String>) -> Armament {
    Armament {
        name: name.to_string(),
        description: format!("Test tool: {}", name),
        parameters: json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "Input parameter"
                }
            }
        }),
        required_params,
    }
}

fn create_args(key: &str, value: Value) -> HashMap<String, Value> {
    let mut args = HashMap::new();
    args.insert(key.to_string(), value);
    args
}
```

**Shared Builders** (from `tests/helpers/mock_llm_adapter.rs`):
```rust
pub fn create_test_paladin_with_mock(mock: Arc<MockLlmAdapter>) -> Paladin {
    // Convenience builder for tests needing a real Paladin with mock LLM
}
```

## Coverage

> **Correction, 2026-08-13 (Phase 15 / PIPE-05, per [ADR-0006](../decisions/0006-coverage-gate.md), D-00d):**
> This section previously carried three defects. First, it stated two separate coverage figures —
> a unit-test target of 80 percent and an integration-test target of 70 percent — matching
> ADR-0006's Considered Options position 1, which was explicitly rejected; the single binding
> floor is recorded below. Second, it documented `cargo tarpaulin` as the local measurement tool;
> tarpaulin uses ptrace-based instrumentation and produces numbers that are not comparable with
> `cargo-llvm-cov`'s LLVM source-based instrumentation, ADR-0006's tool of record — retained below
> only as a labelled informal alternative. Third, it claimed coverage was "Enforced in CI
> pipeline" — false when written (no coverage gate existed in `ci.yml`), now true because of this
> phase's `coverage` job, and made precise below.

**Requirements:**
- **Coverage floor: 82% workspace line coverage** (ADR-0006), the single binding figure — there
  is no separate unit-test target and no separate integration-test target
- All public APIs: Documentation tests (`/// # Examples`)

**View Coverage:**
```bash
# Tool of record (ADR-0006): cargo-llvm-cov, LLVM source-based instrumentation.
# Mirrors CI's `coverage` job — requires `make services-up` first.
make coverage

# Browsable HTML report at target/coverage
make coverage-html

# Underlying invocation, for reference:
cargo llvm-cov --workspace --features integration-tests --lcov --output-path lcov.info \
  --fail-under-lines 82 -- --test-threads=1
```

Informal alternative, **not** the tool of record and **not comparable** to the figure above —
`cargo tarpaulin`'s ptrace-based instrumentation measures differently and will not match the CI
gate:
```bash
cargo tarpaulin --out Html
cargo +nightly tarpaulin --exclude-files tests
```

**Coverage Configuration:**
- Not explicitly configured in `Cargo.toml` (uses defaults)
- **Enforced in CI pipeline**: the `coverage` job in `.github/workflows/ci.yml` runs
  `cargo llvm-cov --workspace --features integration-tests --lcov --output-path lcov.info
  --fail-under-lines 82`; a run measuring below 82% workspace line coverage fails the job

**Documentation Test Examples** (from `src/application/services/paladin/paladin_builder.rs`):
```rust
/// # Example
///
/// ```rust,no_run
/// # use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
/// # use paladin_ports::output::llm_port::LlmPort;
/// # use std::sync::Arc;
/// # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
/// let paladin = PaladinBuilder::new(llm_port)
///     .system_prompt("You are a helpful coding assistant")
///     .name("CodePaladin")
///     .model("gpt-4")
///     .build().await?;
/// # Ok(())
/// # }
/// ```
pub fn new(llm_port: Arc<dyn LlmPort>) -> Self { ... }
```

## Test Types

**Unit Tests:**
- **Scope**: Single function/method in isolation
- **Approach**: Fast, no I/O, use mocks for dependencies
- **Location**: `#[cfg(test)] mod tests { ... }` in same file as code
- **Examples**: `node.rs` tests for `Node<T>` creation, update, versioning
- **Run**: `cargo test --lib`

**Integration Tests:**
- **Scope**: Multiple components working together (Paladin + Garrison + LLM)
- **Approach**: Real implementations, may use Docker services (Redis, MinIO)
- **Location**: `tests/integration/*_test.rs`
- **Examples**:
  - `paladin_garrison_integration_test.rs` — Paladin with memory management
  - `arsenal_execution_integration_test.rs` — Tool registry and execution
  - `redis_queue_integration_test.rs` — Async queue with Redis
- **Run**: `cargo test --test paladin_garrison_integration` or `make test-integration-docker`
- **Feature-gated**: May require specific features enabled (e.g., `vision`, `qdrant`)

**Functional Tests:**
- **Scope**: End-to-end application workflows
- **Location**: `tests/*.rs` (e.g., `tests/functional.rs`, `tests/content_ingestion_pipeline.rs`)
- **Examples**: Complete agent orchestration flows, multi-phase tasks
- **Run**: `cargo test --test functional`

**Property-Based Tests:**
- **Framework**: `proptest` v1.4
- **Location**: Within unit or integration test modules
- **Usage**: Verify properties hold across ranges of input values

## Async Testing

**Pattern with** `#[tokio::test]`:
```rust
#[tokio::test]
async fn test_paladin_with_garrison_stores_conversation() {
    // Test async code directly without `.block_on()`
    let result = service.execute(&paladin, "input").await;
    assert!(result.is_ok());
}
```

**Handling Timeouts:**
```rust
#[tokio::test]
async fn test_with_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        service.execute(&paladin, "input")
    ).await;

    assert!(result.is_ok(), "Operation should complete within timeout");
}
```

**Stream Testing** (from integration tests):
```rust
#[tokio::test]
async fn test_streaming_response() {
    let response = llm_port.generate_stream(request).await.unwrap();

    // Pin stream for iteration
    let mut stream = Box::pin(response);

    let mut chunks = Vec::new();
    while let Some(chunk) = stream.next().await {
        chunks.push(chunk.unwrap().content);
    }

    assert!(!chunks.is_empty(), "Should have received chunks");
}
```

## Error Testing

**Pattern:**
```rust
#[test]
fn test_error_variant() {
    let err = PaladinError::ConfigurationError("missing field".into());

    // Verify error type
    match err {
        PaladinError::ConfigurationError(msg) => {
            assert_eq!(msg, "missing field");
        }
        _ => panic!("Expected ConfigurationError"),
    }
}
```

**Result Assertion:**
```rust
#[tokio::test]
async fn test_invalid_configuration_fails() {
    let result = PaladinBuilder::new(llm)
        .system_prompt("")  // Invalid: empty
        .build()
        .await;

    assert!(result.is_err(), "Empty system prompt should fail");

    if let Err(PaladinError::ConfigurationError(msg)) = result {
        assert!(msg.contains("System prompt is required"));
    } else {
        panic!("Expected ConfigurationError");
    }
}
```

## Test Attributes and Features

**Feature-Gated Tests:**
```rust
#[cfg(feature = "vision")]
#[tokio::test]
async fn test_vision_analysis() {
    // Only runs when vision feature is enabled
}
```

**Ignored Tests** (for future work):
```rust
#[test]
#[ignore]  // TODO: Requires mock Paladin that can fail
fn test_retry_on_failure() {
    // Test will be skipped unless run with `cargo test -- --ignored`
}
```

**Serial Execution** (for tests with shared state):
```rust
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_shared_resource_1() { ... }

#[tokio::test]
#[serial]
async fn test_shared_resource_2() { ... }
```

## Common Test Dependencies

**From Cargo.toml** `[dev-dependencies]`:
- `mockito` v1.7.0 — HTTP mocking
- `tempfile` v3.0 — Temporary file fixtures
- `testcontainers` v0.24.0 — Docker container management for integration tests
- `testcontainers-modules` v0.12.1 — Pre-built container specs (MinIO, Redis, etc.)
- `tokio-test` v0.4 — Utilities for testing tokio code
- `proptest` v1.4 — Property-based testing
- `serial_test` v3.2.0 — Enforce test serialization
- `criterion` v0.5 — Benchmarking with `async_tokio` feature
- `insta` v1.34 — Snapshot testing
- `redis` v0.32.2 — Redis client for integration tests
- `wiremock` v0.6 — HTTP server mocking

## CI Test Configuration

**From** `.github/workflows/ci.yml`:
- **Linting job**: `cargo clippy -- -D warnings`, `cargo fmt --check`
- **Testing job**: `cargo test --workspace`
- **Feature flags job**: Tests with different feature combinations
- **Integration tests**: Optional, docker-compose required

**Run Integration Tests with Docker:**
```bash
./scripts/run_integration_tests.sh -m docker -v

# Or by category:
./scripts/run_integration_tests.sh -t "redis" -m local
./scripts/run_integration_tests.sh -t "file_storage" -m local
```

---

*Testing analysis: 2026-07-30*
