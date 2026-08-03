# Phase 3: Verification Depth - Pattern Map

**Mapped:** 2026-08-02
**Files analyzed:** 6 deliverable classes (test modules, shared mock, doc/record updates — no product features)
**Analogs found:** 6 / 6

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `tests/helpers/mock_paladin_port.rs` (extend with `FaultyPaladinPort`) | test-utility (mock) | request-response, fault-injection | `tests/helpers/mock_llm_adapter.rs` (interior-mutability + invocation tracking); `crates/paladin-battalion/src/formation_service.rs` in-file `MockPaladinPort` (fail_until_attempt); `crates/paladin-battalion/src/phalanx_service.rs` in-file `MockPaladinPort` (fail_paladin_names + delay_ms); `tests/integration/commander_integration_tests.rs` `IntegrationMockPaladinPort` (execution_log pattern) | exact (union of 4 existing mocks) |
| `tests/integration/commander_error_paths_test.rs` (new) | test (integration) | event-driven / error-path | `tests/integration/commander_integration_tests.rs` | exact |
| `tests/integration/mod.rs` (wiring edit) | barrel/module | — | itself (existing alphabetical `pub mod` list) | exact |
| `crates/paladin-storage/src/redis.rs` (refactor + `#[cfg(test)] mod tests`) | infrastructure adapter / unit test | CRUD (key construction, serialize/deserialize) | `crates/paladin-storage/src/scheduler.rs` `#[cfg(test)] mod tests` (pure, engine-free test style); `crates/paladin-storage/src/sqlite_user_repository.rs` `#[cfg(test)] mod tests` | role-match (in-crate unit-test shape); redis.rs itself for the refactor target |
| `tests/integration/mcp_streamable_http_test.rs` (extend) + possibly a new sibling file for malformed-response/handshake-timeout | test (integration, hermetic fixture server) | request-response over real HTTP | itself — `FixtureServer` + `spawn_fixture_server()` + the two existing auth-rejection tests | exact (primary analog IS the target file) |
| `docs/src/appendix/performance-baseline.md` (amend in place, new dated section) | doc/record | batch (bench run + write-up) | itself — existing Scope/Environment/Methodology/Results structure | exact |
| Coverage-measurement record for Phase 3 (new, e.g. `03-coverage-measurement.md`) | doc/record | batch (measurement provenance) | `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` | exact |

## Pattern Assignments

### `tests/helpers/mock_paladin_port.rs` — add `FaultyPaladinPort`

**Analogs (read in full/relevant part):**
- `tests/helpers/mock_llm_adapter.rs:1-90` — interior-mutability + invocation-tracking idiom
- `crates/paladin-battalion/src/formation_service.rs:379-458` — `fail_until_attempt` counter pattern
- `crates/paladin-battalion/src/phalanx_service.rs:478-550` — `fail_paladin_names` + `delay_ms` pattern
- `tests/integration/commander_integration_tests.rs:1-100` — `IntegrationMockPaladinPort` with `execution_log`

**Existing file's imports/shape** (`tests/helpers/mock_paladin_port.rs`, full file, 62 lines):
```rust
use async_trait::async_trait;
use std::sync::Arc;

use paladin::application::services::paladin::error::PaladinError;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::paladin::Paladin;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin_ports::output::llm_port::LlmPort;
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult, PaladinStreamChunk};

use super::MockLlmAdapter;

pub struct MockPaladinPort {
    execution_service: Arc<PaladinExecutionService>,
}
```
Note: this is a **different `PaladinResult`/`PaladinStream` shape** than the one Commander uses.
Commander's tests use `paladin_ports::output::paladin_port::{PaladinResult, PaladinStream, StopReason}`
(non-chunked `Receiver<Result<PaladinStreamChunk, PaladinError>>` is `mock_paladin_port.rs`'s local
type alias `PaladinStream` — confirm the exact `PaladinPort` trait signature `FaultyPaladinPort` must
implement by reading `crates/paladin-ports/src/output/paladin_port.rs:631` before writing it; the
CommanderBuilder-side sketch below (Code Examples §"FaultyPaladinPort construction sketch") already
verified the imports that compile against `commander_error_paths_test.rs`.

**Interior-mutability idiom to copy** (`tests/helpers/mock_llm_adapter.rs:16-30,62-78`):
```rust
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Invocation {
    pub prompt: String,
    pub model: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone)]
pub struct MockLlmAdapter {
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
    invocations: Arc<Mutex<Vec<Invocation>>>,
}

impl MockLlmAdapter {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(VecDeque::new())),
            invocations: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn add_response(&self, response: MockResponse) {
        self.responses.lock().unwrap().push_back(response);
    }
}
```

**Fail-until-attempt (retry-count) pattern to copy** (`crates/paladin-battalion/src/formation_service.rs:379-458`):
```rust
struct MockPaladinPort {
    call_count: Arc<Mutex<usize>>,
    should_fail: bool,
    fail_until_attempt: Option<usize>,
}

impl MockPaladinPort {
    fn new_with_retry_success(fail_until: usize) -> Self {
        Self {
            call_count: Arc::new(Mutex::new(0)),
            should_fail: false,
            fail_until_attempt: Some(fail_until),
        }
    }
    fn get_call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl PaladinPort for MockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str)
        -> Result<PaladinResult, paladin_core::platform::container::paladin_error::PaladinError>
    {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        let current_count = *count;
        drop(count);

        if let Some(fail_until) = self.fail_until_attempt
            && current_count <= fail_until
        {
            return Err(paladin_core::platform::container::paladin_error::PaladinError::ExecutionError(
                format!("Intentional failure for testing (attempt {})", current_count),
            ));
        }
        if self.should_fail {
            return Err(paladin_core::platform::container::paladin_error::PaladinError::ExecutionError(
                "Mock Paladin execution failed".to_string(),
            ));
        }
        Ok(PaladinResult {
            output: format!("Processed: {} by {}", input, paladin.node.name),
            token_count: 100,
            execution_time_ms: 100,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            ..Default::default()
        })
    }
}
```

**Fail-by-name + delay (partial-failure / timeout) pattern to copy** (`crates/paladin-battalion/src/phalanx_service.rs:478-550`):
```rust
struct MockPaladinPort {
    call_count: Arc<Mutex<usize>>,
    fail_paladin_names: Arc<Mutex<Vec<String>>>,
    delay_ms: u64,
    output_override: Arc<Mutex<HashMap<String, String>>>,
}

impl MockPaladinPort {
    fn with_failures(self, names: Vec<String>) -> Self {
        *self.fail_paladin_names.lock().unwrap() = names;
        self
    }
}

#[async_trait]
impl PaladinPort for MockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        *self.call_count.lock().unwrap() += 1;
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;

        let should_fail = self.fail_paladin_names.lock().unwrap().contains(&paladin.node.name);
        if should_fail {
            return Err(PaladinError::ExecutionError(format!("Mock failure for {}", paladin.node.name)));
        }
        // ... success path with output_override lookup
    }
}
```

**Execution-log pattern to copy** (`tests/integration/commander_integration_tests.rs:20-100`):
```rust
#[derive(Clone)]
struct IntegrationMockPaladinPort {
    execution_log: Arc<Mutex<Vec<String>>>,
    failure_config: Arc<Mutex<FailureConfig>>,
}

#[derive(Clone, Debug)]
struct FailureConfig {
    fail_paladin_names: Vec<String>,
    fail_count: usize,
    delay_ms: u64,
}

impl IntegrationMockPaladinPort {
    fn with_failures(self, paladin_names: Vec<String>) -> Self {
        self.failure_config.lock().unwrap().fail_paladin_names = paladin_names;
        self
    }
    fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.lock().unwrap().clone()
    }
}

#[async_trait]
impl PaladinPort for IntegrationMockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        let log_entry = format!("Executing {}: {}", paladin.node.name, input);
        self.execution_log.lock().unwrap().push(log_entry);
        let delay = self.failure_config.lock().unwrap().delay_ms;
        tokio::time::sleep(Duration::from_millis(delay)).await;
        let should_fail = self.failure_config.lock().unwrap().fail_paladin_names.contains(&paladin.node.name);
        if should_fail {
            return Err(PaladinError::ExecutionError(format!("Simulated failure for {}", paladin.node.name)));
        }
        // ... success
    }
}
```

**`FaultyPaladinPort` synthesis (what to build, per D-09/D-10):** combine `call_count: Arc<Mutex<usize>>`
(retry counter, from `formation_service.rs`), `fail_paladin_names: Arc<Mutex<Vec<String>>>` (Nth-Paladin
failure, from `phalanx_service.rs`), a `fail_always: bool` flag, and `delay_ms: u64` (controllable delay,
from `phalanx_service.rs`). All fields use `Arc<Mutex<_>>` — never `Rc`/`RefCell` — which is what makes
every one of these four source mocks trivially `Send + Sync` already; carry the same idiom forward.
Barrel export: add `pub use mock_paladin_port::{FaultyPaladinPort, MockPaladinPort};` to `tests/helpers/mod.rs`
next to the existing `pub use mock_paladin_port::MockPaladinPort;` line (`tests/helpers/mod.rs:16`).

---

### `tests/integration/commander_error_paths_test.rs` (new)

**Analog:** `tests/integration/commander_integration_tests.rs` (full file header, imports, and mock
shown above apply directly).

**Imports pattern to copy** (`tests/integration/commander_integration_tests.rs:1-18`):
```rust
use async_trait::async_trait;
use paladin::application::services::battalion::commander::CommanderBuilder;
use paladin::application::services::paladin::error::PaladinError;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::battalion::{
    BattalionConfig, BattalionStatus, BattalionStrategy, ErrorStrategy, RetryPolicy,
};
use paladin::core::platform::container::paladin::MaxLoops;
use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult, PaladinStream, StopReason};
use std::sync::{Arc, Mutex};
use std::time::Duration;
```
Also import the new mock: `use paladin_test_helpers::helpers::mock_paladin_port::FaultyPaladinPort;`
— **verify the actual crate-root path used by sibling integration files first** (they import via the
`tests/lib.rs` `pub mod helpers;` barrel, reached as `crate::helpers::...` from within the `tests/lib.rs`
binary, NOT as an external crate — confirm against another file under `tests/integration/` that already
imports from `tests/helpers/` before writing this line).

**Test-body shape to copy for all four relocated tests** — pull the CommanderBuilder construction
pattern from `tests/integration/commander_integration_tests.rs` (the file already builds Commander with
a fail-capable mock port for its passing tests) and pair with the four empty test *names and doc intent*
at `crates/paladin-battalion/src/commander.rs:2179-2208`:
```rust
#[tokio::test]
async fn test_fail_fast_stops_on_first_error() {
    // Test that FailFast strategy stops on first Paladin error
    // Verify that subsequent Paladins are not executed
    // Verify that error is propagated immediately
}
```
(same for `test_continue_on_error_collects_all_errors`, `test_retry_then_continue_retries_failed_paladins`,
`test_partial_results_returned_with_errors` — all four move here, un-`#[ignore]`d, with real bodies built
against `FaultyPaladinPort` using its execution-log getter to assert "Paladin-3 never ran" the way
`IntegrationMockPaladinPort::get_execution_log()` already demonstrates.)

**Construction sketch** (verified trait/type surface, from RESEARCH.md Code Examples):
```rust
use paladin_ports::output::paladin_port::PaladinPort;
use paladin::application::services::battalion::commander::CommanderBuilder;
use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy, RetryPolicy};

let port = Arc::new(FaultyPaladinPort::new().fail_paladin("Paladin-2"));
let config = BattalionConfig::new("t").with_error_strategy(ErrorStrategy::FailFast);
// CommanderBuilder::new(port).strategy(...).paladins(...).config(config)
//   .build().unwrap().execute("input").await — assert Err + Paladin-3 never ran
```

**Wiring — no `[[test]]` entry, one `pub mod` line only** (verified live in this mapping session:
`tests/lib.rs:60` declares `pub mod integration;` and `tests/integration/mod.rs` already lists ~35
`pub mod X;` entries alphabetically with none owning a separate `[[test]]` Cargo.toml target):
```rust
// tests/integration/mod.rs — insert alphabetically next to `citadel_integration_test`:
pub mod commander_error_paths_test;
```
Do not add anything to `Cargo.toml`. Adding a `[[test]]` entry risks `clippy::duplicate_mod` under
`-D warnings` — see the documented precedent at `tests/lib.rs:70-77` (a prior `pub mod cli;` was
removed for exactly this reason).

---

### `crates/paladin-storage/src/redis.rs` (refactor + test module)

**Analog for the refactor target (redis.rs itself, current shape, lines 58-200):** the eight
`&self`-taking private helpers to convert to free functions / `&RedisQueueConfig`-taking associated
functions:
```rust
fn queue_key(&self, queue_name: &str) -> String {
    format!("{}:queue:{}", self.config.key_prefix, queue_name)
}
fn priority_queue_key(&self, queue_name: &str, priority: MessagePriority) -> String { /* ... */ }
fn queue_meta_key(&self, queue_name: &str) -> String { /* ... */ }
fn processing_key(&self, queue_name: &str) -> String { /* ... */ }
fn completed_key(&self, queue_name: &str) -> String { /* ... */ }
fn failed_key(&self, queue_name: &str) -> String { /* ... */ }
fn serialize_item<T>(&self, item: &QueueItem<T>, queue_name: &str) -> Result<String, QueueError>
    where T: Serialize { /* does not read self at all */ }
fn deserialize_item(&self, data: &str) -> Result<QueueItem<serde_json::Value>, QueueError> {
    /* does not read self at all */
}
```
Already directly testable with **no refactor**: `fn get_priority_levels() -> Vec<MessagePriority>`
(`redis.rs:193-200`) — no `&self`, callable as `RedisQueueAdapter::get_priority_levels()` today.

Refactor shape: change signatures to `fn queue_key(config: &RedisQueueConfig, queue_name: &str) -> String`
(or move onto `impl RedisQueueConfig`), update the ~6 call sites inside `RedisQueueAdapter`'s trait impls
from `self.queue_key(name)` to `queue_key(&self.config, name)`; drop `&self` entirely from
`serialize_item`/`deserialize_item` since neither body references it.

**Analog for the in-file `#[cfg(test)] mod tests` shape:** `crates/paladin-storage/src/scheduler.rs:403-482`
— idiomatic in-crate pure/engine-free test module this crate already ships:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Pure, engine-free tests -- the default-suite coverage. None of these
    // construct TokioCronSchedulerAdapter or JobScheduler.
    // ------------------------------------------------------------------

    #[test]
    fn validate_cron_field_count_accepts_valid_six_field_input() {
        assert!(validate_cron_field_count("0 0 9 * * *").is_ok());
    }

    #[test]
    fn validate_cron_field_count_rejects_five_field_input_naming_the_expected_form() {
        let err = validate_cron_field_count("0 9 * * *").unwrap_err();
        match err {
            SchedulerError::InvalidCronExpression { expression, reason } => {
                assert_eq!(expression, "0 9 * * *");
                assert!(reason.contains("sec min hour day month weekday"));
            }
            other => panic!("expected InvalidCronExpression, got {other:?}"),
        }
    }
}
```
This is the pattern `redis.rs`'s new module should follow: a `// Pure, engine-free tests` banner
comment, `use super::*;`, plain `#[test]` (not `#[tokio::test]`, since key/serialize helpers are
synchronous) functions named `<function>_<condition>_<expected>`, testing config defaults, key/namespace
construction for each of the six key builders, `serialize_item`/`deserialize_item` round-trips, priority
ordering (`get_priority_levels()` order), and error-mapping (`QueueError::SerializationError` on bad
JSON). A secondary analog with the same `#[cfg(test)] mod tests` idiom in this crate:
`crates/paladin-storage/src/sqlite_user_repository.rs:386+`.

---

### MCP failure-mode tests (extend `tests/integration/mcp_streamable_http_test.rs`, add sibling file for the two hard modes)

**This file IS the primary analog** — full fixture-server construction (254 lines total), reproduced
here as the copy-source:

**Imports** (lines 19-35):
```rust
use axum::Router;
use axum::extract::Request;
use axum::http::{StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use paladin::core::platform::container::arsenal::ArsenalError;
use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, Implementation, ListToolsResult,
    PaginatedRequestParams, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
```

**Fixture server + `call_tool` (extension point for "unknown tool" and "bad arguments")** (lines 44-93):
```rust
#[derive(Clone, Default)]
struct FixtureServer;

impl ServerHandler for FixtureServer {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        if request.name != ECHO_TOOL_NAME {
            return Err(McpError::invalid_params(
                format!("unknown tool `{}`", request.name),
                None,
            ));                                    // already covers "unknown tool" — add a test
        }
        let message = request
            .arguments
            .as_ref()
            .and_then(|args| args.get("message"))
            .and_then(|v| v.as_str());
        let Some(message) = message else {
            return Err(McpError::invalid_params("missing required `message` argument", None));
            // ^ ADD THIS for "bad arguments" -- currently `.unwrap_or_default()` silently accepts
            // missing/wrong-typed args; this is a one-line source fix plus a new assertion.
        };
        Ok(CallToolResult::success(vec![ContentBlock::text(format!("echo: {message}"))]))
    }
}
```

**Bearer-auth middleware (extension point for "expired/rejected token")** (lines 129-146):
```rust
async fn require_bearer_token(request: Request, next: Next) -> Response {
    let authorized = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(|value| value == format!("Bearer {EXPECTED_BEARER_TOKEN}"))
        .unwrap_or(false);
    if !authorized {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Bearer")],
            "missing or invalid bearer token",
        ).into_response();
    }
    next.run(request).await
}
```

**Fixture-server spawn/teardown** (lines 149-169):
```rust
async fn spawn_fixture_server() -> (String, CancellationToken) {
    let ct = CancellationToken::new();
    let config = StreamableHttpServerConfig::default()
        .with_sse_keep_alive(None)
        .with_cancellation_token(ct.child_token());

    let service: StreamableHttpService<FixtureServer, LocalSessionManager> =
        StreamableHttpService::new(|| Ok(FixtureServer), Default::default(), config);

    let router = Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn(require_bearer_token));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind ephemeral port");
    let addr = listener.local_addr().expect("resolve bound local addr");

    tokio::spawn({
        let ct = ct.clone();
        async move {
            let _ = axum::serve(listener, router)
                .with_graceful_shutdown(async move { ct.cancelled_owned().await })
                .await;
        }
    });
    (format!("http://{addr}/mcp"), ct)
}
```

**Existing passing negative test to copy the shape of** (lines 214-231):
```rust
#[tokio::test]
async fn streamable_http_round_trip_rejects_missing_bearer_token() {
    let (uri, ct) = spawn_fixture_server().await;
    let result = MCPClient::connect_streamable_http(&uri, None, None).await;
    match result {
        Err(ArsenalError::AuthFailed(_)) => {}
        Err(other) => panic!("expected ArsenalError::AuthFailed, got a different error: {other}"),
        Ok(_) => panic!("expected the connection to be rejected without a bearer token, but it succeeded"),
    }
    ct.cancel();
}
```
This is the exact shape for the "expired/rejected token" mode (already 2/5 done — add one more variant
if "expired" needs a distinct fixture from "incorrect"), and the template for "unknown tool"/"bad
arguments" assertions (`client.invoke_tool(...)` → match on the mapped `ArsenalError` variant).

**How `MCPClient::connect_streamable_http` is driven** (line 178, the call every new test uses):
```rust
let client = MCPClient::connect_streamable_http(&uri, Some(EXPECTED_BEARER_TOKEN), None)
    .await
    .expect("authenticated handshake must succeed against the strict fixture server");
```

**Malformed-response / handshake-timeout (2 of 5 modes needing a NEW non-compliant fixture, not this
file's spec-strict `FixtureServer`):** no in-tree analog produces a spec-violating server by
construction (a real `rmcp` `ServerHandler` cannot). Build a second small axum handler in the same
`spawn_fixture_server()` style — bind an ephemeral port, answer `initialize`/`notifications/initialized`
with valid hand-built JSON-RPC (cross-check field shape against `rmcp::model::InitializeResult`'s
`Serialize` impl first), then return truncated JSON for `tools/list`/`tools/call` (malformed response) or
never respond at all (handshake timeout — **blocked pending the Pitfall 4 decision**: `mcp_protocol.rs:50`'s
`STREAMABLE_HTTP_HANDSHAKE_TIMEOUT` is a hardcoded private 30s constant with no test seam; planner must
choose whether to add a `connect_streamable_http_with_timeout` variant, pay the real 30s cost, or defer —
this is explicitly flagged as an open decision in RESEARCH.md, not resolved by this pattern map).

---

### `docs/src/appendix/performance-baseline.md` (amend in place)

**The file itself is the analog** — its existing skeleton (must be matched exactly for the new dated
section, then the current 2026-05-27 run explicitly marked superseded):
```markdown
# Performance Baseline

## Scope
This baseline covers the active Epic 3 benchmark targets:
- `config_benchmarks` (root crate)
- `battalion_benchmarks` (`paladin-battalion`)
- `sanctum_benchmarks` (`paladin-memory`)
- `garrison_benchmarks` (`paladin-memory`)
- `llm_serialization_benchmarks` (`paladin-llm`)

Run timestamp window (UTC): `2026-05-27T22:58:29` to `2026-05-27T23:08:23`

## Environment
| Field | Value |
|---|---|
| Commit SHA | `f4156ff6360aa976d03b2bdb40775e52e1e991be` |
| OS | Debian GNU/Linux 12 (bookworm) |
| Kernel | Linux 6.8.0-111-generic |
| CPU | Intel Xeon E3-1505M v5 @ 2.80GHz |
| Cores / Threads | 4 cores / 8 threads |
| Rust | `rustc 1.95.0 (59807616e 2026-04-14)` |
| Cargo | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| Config Profile | `APP_ENV=test` |

## Methodology
Commands executed:
​```bash
APP_ENV=test cargo bench --bench config_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-battalion --bench battalion_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench sanctum_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench garrison_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-llm --bench llm_serialization_benchmarks -- --noplot
​```

## Results
### Root Config Benchmarks
| Benchmark | Time (lower .. upper) |
|---|---|
| `config/settings_new` | `1.2543 ms .. 1.4626 ms` |
...
## Sanctum Comparison Notes (Post-Migration vs Pre-Migration)
## Historical Data Availability
## Coverage Cross-Check
```
Section headers to reproduce, in the same order, for the new dated run: `## Scope`, `## Environment`,
`## Methodology`, `## Results` (with the same per-bench-target `###` subsections), plus this phase's
additions per D-13/D-14: a `### Memory-per-Paladin` / `### Startup Time` results subsection (sourced
from the small recorded harness, not criterion) and a `### P50 / P95 / P99 Derivation` subsection
documenting the formula from Code Examples below. Mark the whole 2026-05-27 section with an explicit
`> Superseded by the 2026-08-02 run below.` callout rather than deleting it (D-15 / the in-place-amendment
convention Phase 2's D-02 established).

**Percentile derivation to document verbatim (from RESEARCH.md Code Examples, verified against vendored
`criterion-0.5.1` source, `src/lib.rs:1502-1505` `SavedSample { iters: Vec<f64>, times: Vec<f64> }`):**
```rust
fn per_iteration_times_ns(sample: &SavedSample) -> Vec<f64> {
    sample.iters.iter().zip(sample.times.iter())
        .map(|(iters, total_ns)| total_ns / iters)
        .collect()
}
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx]
}
// Read target/criterion/<group>/<function>/new/sample.json, call per_iteration_times_ns,
// sort, then percentile(&sorted, 0.50 | 0.95 | 0.99).
```

---

### Coverage-measurement record for Phase 3 (new file, e.g. `03-coverage-measurement.md`)

**Analog:** `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` (609 lines).
Section skeleton to reproduce exactly, in this order:
```markdown
# Coverage Measurement — Raw Evidence Record

## Tracer — pipeline proof on one crate (`paladin-ai-core`)   [OPTIONAL — Phase 1 did this because
                                                                 the pipeline was unproven; Phase 3 may
                                                                 skip straight to the full run since the
                                                                 pipeline is now proven, but if included,
                                                                 keep the same probe order]
### Environment probes (verbatim)
### Package-name correction (Rule 3 auto-fix, not a fabrication)   [only if applicable]
### Pipeline commands (verbatim)

## Measurement of record — workspace-wide line coverage

## Human confirmation (Task 3 checkpoint)
```
**Environment probes block to copy verbatim in structure** (`01-coverage-measurement.md:13-70`):
```markdown
Command: `rustc -vV`
​```
rustc 1.97.1 (8bab26f4f 2026-07-14)
...
​```

Command: `cargo --version`
​```
cargo 1.97.1 (c980f4866 2026-06-30)
​```

Command: `command -v docker`
​```
(no output)
​```
Exit status: 1 (absent). Docker is not installed in this environment.
```
**Human-confirmation section shape to copy** (`01-coverage-measurement.md:569-609`):
```markdown
## Human confirmation (Task 3 checkpoint)

**Approved by:** <email>, via the plan's `checkpoint:human-verify`, `gate="blocking-human"` resume signal.
**Approved on:** <ISO-8601 UTC>.
**Resume signal given:** `approved`.

**What was confirmed:** the measured figure of **NN.NN%** workspace line coverage (X lines counted,
Y missed), together with the recorded scope stated above.

Before presenting the checkpoint, the orchestrator additionally verified and recorded:
- The TOTAL row arithmetic is exact: `(X - Y) / X = NN.NN%`, matching the pasted stdout character-for-character.
- <commit-distance / tree-identity checks specific to this run>
```
Every figure in the new record must carry: `rustc -vV`, `cargo --version`, `git rev-parse HEAD`,
`date -u`, raw pasted `llvm-cov report` TOTAL row, and — since D-01 requires re-deriving the
zero-coverage set — a per-file table matching the D-04 five-file list (`redis.rs`, `paladin-server.rs`,
`file_storage_port.rs`, `error.rs`, `arsenal_port.rs`) with their post-Phase-3 percentages, in the same
row style as `01-coverage-measurement.md`'s per-file rows (evidence base for D-03's staleness table).

## Shared Patterns

### Interior mutability for mock invocation counters / fault-injection state
**Source:** `tests/helpers/mock_llm_adapter.rs:62-78` (queue + tracking), `crates/paladin-battalion/src/formation_service.rs:379-393` (attempt counter), `crates/paladin-battalion/src/phalanx_service.rs:478-505` (fail-by-name + delay)
**Apply to:** `FaultyPaladinPort` in `tests/helpers/mock_paladin_port.rs`
```rust
pub struct FaultyPaladinPort {
    call_count: Arc<Mutex<usize>>,
    fail_always: bool,
    fail_paladin_names: Arc<Mutex<Vec<String>>>,
    fail_until_attempt: Option<usize>,
    delay_ms: u64,
}
```
Always `Arc<Mutex<_>>`, never `Rc`/`RefCell` — this is what makes every existing mock in this codebase
trivially `Send + Sync` (required by `PaladinPort: Send + Sync`, `crates/paladin-ports/src/output/paladin_port.rs:631`).

### `#[async_trait] impl PaladinPort` shape
**Source:** all four mock analogs above implement the same three methods (`execute`, `execute_stream`,
`validate`) with the same signatures. `execute_stream` in every existing mock either errors
("not supported in mock") or returns an empty/dummy channel — follow that precedent rather than
building real streaming into `FaultyPaladinPort`.

### `tests/integration/` barrel wiring — new file, one line, no Cargo.toml edit
**Source:** `tests/lib.rs:60` (`pub mod integration;`) + `tests/integration/mod.rs`'s alphabetical
`pub mod` list (35+ entries, several `#[cfg(feature = "...")]`-gated, none with a separate `[[test]]`
Cargo.toml target)
**Apply to:** `commander_error_paths_test.rs` and any new MCP failure-mode file
```rust
pub mod commander_error_paths_test;
```
**Do not** add a `[[test]]` entry — see `tests/lib.rs:70-77`'s documented `clippy::duplicate_mod`
precedent (a prior `pub mod cli;` declaration was removed for exactly this reason).

### `#[cfg(test)] mod tests` — this crate's idiomatic in-file unit-test shape
**Source:** `crates/paladin-storage/src/scheduler.rs:403-482`, `crates/paladin-storage/src/sqlite_user_repository.rs:386+`
**Apply to:** `crates/paladin-storage/src/redis.rs`'s new test module
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn <function>_<condition>_<expected>() {
        // plain #[test], not #[tokio::test], for synchronous pure-function seams
    }
}
```

### Dated-provenance measurement record shape
**Source:** `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` (full skeleton
above), `.planning/decisions/0006-coverage-gate.md` (the binding command/scope this phase must reproduce
verbatim per D-02)
**Apply to:** the new Phase 3 coverage-measurement record and the new dated section in
`docs/src/appendix/performance-baseline.md`
- Every figure: `rustc -vV` + `cargo --version` + `git rev-parse HEAD` + `date -u`, raw pasted stdout,
  arithmetic a reader can re-derive, human-confirmation section for the coverage record.

## No Analog Found

None — every one of the six deliverable classes in this phase has a directly-copyable in-tree analog
(most strikingly, three of the six analog files ARE the files being modified). This is consistent with
CONTEXT.md's framing: Phase 3 is measurement and mechanical extension of already-shipped seams, not new
architectural construction.

## Metadata

**Analog search scope:** `tests/helpers/`, `tests/integration/`, `crates/paladin-battalion/src/{commander,formation_service,phalanx_service}.rs`, `crates/paladin-storage/src/{redis,scheduler,sqlite_user_repository}.rs`, `src/infrastructure/adapters/arsenal/mcp_protocol.rs`, `docs/src/appendix/performance-baseline.md`, `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`
**Files scanned:** ~16 read directly (full or targeted ranges), all paths and line numbers verified live against the tree during this mapping session (2026-08-02), not inferred from RESEARCH.md alone
**Pattern extraction date:** 2026-08-02
```

## PATTERN MAPPING COMPLETE

**Phase:** 3 - Verification Depth
**Files classified:** 7 (grouped into 6 deliverable classes)
**Analogs found:** 7 / 7

### Coverage
- Files with exact analog: 7 (`mock_paladin_port.rs` extension is a union of 4 exact analogs; `mcp_streamable_http_test.rs` and `performance-baseline.md` and the coverage record are each their own primary analog)
- Files with role-match analog: 1 (`redis.rs`'s new test module, matched against `scheduler.rs`/`sqlite_user_repository.rs`'s in-crate test-module shape)
- Files with no analog: 0

### Key Patterns Identified
- `FaultyPaladinPort` is a mechanical union of four already-shipped mocks (`mock_llm_adapter.rs`'s interior-mutability idiom, `formation_service.rs`'s retry counter, `phalanx_service.rs`'s fail-by-name+delay, `commander_integration_tests.rs`'s execution-log) — all `Arc<Mutex<_>>`-based, no new pattern needed.
- New `tests/integration/` files need exactly one `pub mod` line in `tests/integration/mod.rs` and zero `Cargo.toml` changes — adding a `[[test]]` entry risks `clippy::duplicate_mod` under `-D warnings` (documented precedent at `tests/lib.rs:70-77`).
- `redis.rs` cannot get unit tests until 8 private `&self` helper methods are refactored to take `&RedisQueueConfig` (or move to `RedisQueueConfig` impl) — `ConnectionManager::new()` blocks on a live connection with no Docker available, so no instance can be constructed for testing today.
- `mcp_streamable_http_test.rs` is simultaneously the target file and its own best analog: its `FixtureServer`/`spawn_fixture_server()`/bearer-auth-middleware pattern extends directly for 3 of 5 required MCP failure modes; malformed-response and handshake-timeout need a second, deliberately non-compliant fixture (handshake-timeout is additionally blocked on a Pitfall-4 decision about a hardcoded 30s private constant).
- Every measurement deliverable (coverage record, performance baseline) must match `01-coverage-measurement.md`'s provenance skeleton exactly: `rustc -vV`/`cargo --version`/`git rev-parse HEAD`/`date -u`, raw pasted command stdout, and a human-confirmation section.

### File Created
`/workspace/.planning/phases/03-verification-depth/03-PATTERNS.md`

### Ready for Planning
Pattern mapping complete. Planner can now reference analog patterns in PLAN.md files.
