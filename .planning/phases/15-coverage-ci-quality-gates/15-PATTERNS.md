# Phase 15: Coverage & CI Quality Gates - Pattern Map

**Mapped:** 2026-08-13
**Files analyzed:** 10 (new/modified)
**Analogs found:** 8 / 10 (2 have no in-repo analog — noted explicitly, not stretched)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|--------------------|------|-----------|-----------------|----------------|
| `.github/workflows/ci.yml` (`coverage` job, new) | CI config / job | batch (measure + gate) | `ci.yml:374-432` (`integration-tests` job) | exact — same services, same feature scope |
| `.github/workflows/ci.yml` (`cli-tests` job, new) | CI config / job | batch (test run) | `ci.yml:319-372` (`crate-isolation`, `paladin-ai` leg) | role-match — same crate, adds `--features cli --test cli` |
| `.github/workflows/ci.yml` (`bench-check` job, new) | CI config / job | batch (compile-check) | `ci.yml:319-372` (`crate-isolation`, build-only steps) | partial — no existing compile-only bench job; steps shape copied from `crate-isolation`'s build steps |
| `.github/workflows/ci.yml` / `integration-tests.yml` (8 deprecated-action replacements) | CI config | transform (in-place edit) | Existing `dtolnay/rust-toolchain@stable` and `actions/cache@v4` usages already in `ci.yml` (e.g. `ci.yml:21-30`, `crate-isolation`'s cache step) | exact — the replacement action is already the dominant pattern in the same file |
| `Makefile` (`coverage`, `coverage-html`, `test-cli`, `bench-check`, `ci-full` targets) | build config / target | batch | `Makefile:426-427` (`ci-test`) and the `##@ Testing` section (`test`, `test-integration*`) | exact — same file, same section conventions |
| `.codecov.yml` (new root file) | config | request-response (report upload consumption) | none | **no analog — see below** |
| `src/test_support/mod.rs` + `failing_channel_handler.rs` (new) | utility / test-double | event-driven (async trait double) | `tests/helpers/mock_llm_adapter.rs` | exact shape, different placement (D-08 moves it into `src/`, `#[cfg(test)]`-gated) |
| `src/core/platform/manager/user_service.rs` (`#[cfg(test)] mod tests`, extended) | service / test module | CRUD + event-driven (registration + notification) | itself — `user_service.rs:467-583` (existing test module) | exact — extend in place |
| `src/application/services/orchestration/listener.rs` (`#[cfg(test)] mod tests`, extended) | service / test module | event-driven | itself — `listener.rs:398-538` (existing test module) | exact — extend in place |
| `docs/src/contributing/testing-guide.md` (extended) | docs | — | itself (existing file, D-13 confirms it exists) | exact — extend, not create |

## Pattern Assignments

### `.github/workflows/ci.yml` — new `coverage` job

**Analog:** `.github/workflows/ci.yml:374-432` (`integration-tests` job) — copy the `services:` block verbatim, replace the toolchain/test steps.

**Services block to copy verbatim** (`ci.yml:374-401`):
```yaml
  integration-tests:
    name: Integration Tests
    runs-on: ubuntu-latest
    services:
      redis:
        image: redis:7-alpine
        ports:
          - 6380:6379
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

      minio:
        image: minio/minio:latest
        ports:
          - 9010:9000
          - 9011:9001
        env:
          MINIO_ROOT_USER: testuser
          MINIO_ROOT_PASSWORD: testpass123
        options: >-
          --health-cmd "curl -f http://localhost:9000/minio/health/live"
          --health-interval 30s
          --health-timeout 20s
          --health-retries 3
        command: server /data --console-address ":9001"

    steps:
      - name: Checkout code
        uses: actions/checkout@v5
```

**IMPORTANT — do not copy the toolchain step as-is.** The `integration-tests` job's own toolchain
step is one of the eight deprecated-action sites this phase fixes (`actions-rs/toolchain@v1` at
`ci.yml:408` — see finding 1 in CONTEXT.md). The new `coverage` job must use the *already-correct*
pattern seen elsewhere in the same file, plus the `llvm-tools-preview` component (not present
anywhere in the file today — RESEARCH.md's Pitfall 1):
```yaml
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-llvm-cov
```

**Cache step pattern** (copy shape from `crate-isolation`, `ci.yml:349-354`):
```yaml
      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-coverage-${{ hashFiles('**/Cargo.lock') }}
```

**MinIO client + bucket setup steps to copy verbatim** (`ci.yml:404-432`, unchanged — the coverage
job needs the same Redis/MinIO env the `integration-tests` job already establishes):
```yaml
      - name: Install MinIO Client
        run: |
          wget https://dl.min.io/client/mc/release/linux-amd64/mc
          chmod +x mc
          sudo mv mc /usr/local/bin/

      - name: Setup MinIO buckets
        run: |
          mc alias set testminio http://localhost:9010 testuser testpass123
          mc mb testminio/test-bucket --ignore-existing
          mc mb testminio/integration-tests --ignore-existing
```

**Core measurement step** (new — no analog, follows RESEARCH.md's documented `cargo-llvm-cov`
syntax and D-04's two-commit sequence):
```yaml
      - name: Measure coverage
        env:
          USE_EXTERNAL_TEST_SERVICES: "true"
          TEST_REDIS_HOST: localhost
          TEST_REDIS_PORT: 6380
          TEST_MINIO_ENDPOINT: localhost:9010
          TEST_MINIO_ACCESS_KEY: testuser
          TEST_MINIO_SECRET_KEY: testpass123
        run: |
          cargo llvm-cov --workspace --features integration-tests \
            --lcov --output-path lcov.info
          # commit 1: no --fail-under-lines (measure-only, D-04)
          # commit 2: cargo llvm-cov ... --fail-under-lines <re-derived floor>
```

---

### `.github/workflows/ci.yml` — new `cli-tests` job

**Analog:** `ci.yml:319-372` (`crate-isolation`, `paladin-ai` matrix leg) — same crate (`paladin-ai`
per `Cargo.toml`'s package name), extend rather than invent.

**Checkout + toolchain + cache steps to copy verbatim** (`ci.yml:346-354`, already-correct pattern,
no deprecated action here):
```yaml
    steps:
      - name: Checkout code
        uses: actions/checkout@v5

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-cli-${{ hashFiles('**/Cargo.lock') }}
```

**Test invocation** — the one-line difference from `crate-isolation`'s own test step
(`ci.yml:370-372`, `cargo test -p ${{ matrix.crate }} ${{ matrix.extra_flags }}`): this job needs no
matrix, just the specific feature + target:
```yaml
      - name: Run CLI snapshot tests
        run: cargo test -p paladin-ai --features cli --test cli
```

---

### `.github/workflows/ci.yml` — new `bench-check` job

**No exact analog in the repo — no existing compile-only-benchmark job.** Closest shape is
`crate-isolation`'s build-only step (`ci.yml:367-369`, `cargo build -p ${{ matrix.crate }}`, which
compiles without running). Steps assembled from that shape plus RESEARCH.md's documented
`cargo bench --no-run` pattern:
```yaml
      - name: Checkout code
        uses: actions/checkout@v5
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-bench-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo bench --workspace --no-run
```

---

### `.github/workflows/ci.yml` / `.github/workflows/integration-tests.yml` — deprecated action replacement

**Analog:** the already-correct usages in the same files — `dtolnay/rust-toolchain@stable` appears
15+ times in `ci.yml` already (e.g. `ci.yml:21-30`, `crate-isolation` at `ci.yml:350`); `actions/cache@v4`
appears already at `ci.yml:349-354`. This is a same-file, same-convention transform, not a new
pattern.

**Sites confirmed by direct grep this session (re-grepped — CONTEXT.md finding 1 and PATTERNS
Pitfall 5 both warn every cited line number is stale in older docs):**
```
actions-rs/toolchain@v1  →  dtolnay/rust-toolchain@stable
  ci.yml:163  (api-surface job)
  ci.yml:408  (integration-tests job)
  ci.yml:788  (benchmark job)
  integration-tests.yml:71

actions/cache@v3  →  actions/cache@v4
  integration-tests.yml:78, :84, :90

codecov/codecov-action@v3  →  DELETED (D-03), not upgraded
  integration-tests.yml:123 — this whole step block is removed per D-03, along with its
  continue-on-error: true wrapping (verified present at integration-tests.yml:113-127)
```

---

### `Makefile` — new Coverage section + `test-cli` / `bench-check` targets

**Analog:** `Makefile:426-427` (`ci-test`) for the target-comment style, and the `##@ Testing`
section (`Makefile:113-186`, e.g. `test`, `test-integration`, `test-integration-docker`) for the
section-header and `.PHONY` conventions.

**Existing target shape to copy** (`Makefile:115-119`, `test`):
```makefile
.PHONY: test
test: ## Run unit tests
	@echo "$(CYAN)Running unit tests...$(NC)"
	@$(CARGO) test --workspace --lib --bins
```

**New Coverage section, following the same `##@`/`.PHONY`/echo-with-color convention** (goes between
`##@ Testing` and `##@ Code Quality` per RESEARCH.md's structure recommendation):
```makefile
##@ Coverage

.PHONY: coverage
coverage: ## Measure workspace coverage (mirrors CI's `coverage` job — requires make services-up)
	@echo "$(CYAN)Measuring coverage...$(NC)"
	@$(CARGO) llvm-cov --workspace --features integration-tests \
		--lcov --output-path lcov.info

.PHONY: coverage-html
coverage-html: ## Generate an HTML coverage report at target/coverage
	@echo "$(CYAN)Generating HTML coverage report...$(NC)"
	@$(CARGO) llvm-cov --workspace --features integration-tests \
		--html --output-dir target/coverage
	@echo "Report at target/coverage/html/index.html"
```

**`test-cli` and `bench-check` join the existing `##@ Testing` section**, matching `test`'s comment
style (`## Run ...`):
```makefile
.PHONY: test-cli
test-cli: ## Run CLI snapshot tests (86 snapshots, requires --features cli)
	@echo "$(CYAN)Running CLI snapshot tests...$(NC)"
	@$(CARGO) test -p paladin-ai --features cli --test cli

.PHONY: bench-check
bench-check: ## Compile-check benchmarks without running them
	@echo "$(CYAN)Checking benchmark compilation...$(NC)"
	@$(CARGO) bench --workspace --no-run
```

`ci-full` is a new composite target — model on `test-all` (`Makefile:145-146`,
`test-all: test test-doc test-integration ## Run all tests`), same one-line dependency-chain style.

---

### `.codecov.yml` (new root file)

**No in-repo analog — stated explicitly rather than invented.** No `.codecov.yml` exists anywhere in
this tree (confirmed by RESEARCH.md's direct-read pass). Its shape is fully specified by D-02
(no blocking status) and D-06 (`src/bin/**` in `ignore`) in CONTEXT.md, plus RESEARCH.md's Standard
Stack table naming `codecov/codecov-action@v5` as the current upload action if the `coverage` job
adds an upload step (RESEARCH.md Open Question 2 flags this for planner confirmation). Follow
CONTEXT.md/RESEARCH.md directly; there is no existing project file to pattern-match against.

---

### `src/test_support/` (new module, D-08)

**Analog:** `tests/helpers/mock_llm_adapter.rs` (full file read this session — 617 lines) is the
canonical hand-written mock shape to replicate. Copy the *shape*, not the placement — D-08 requires
this to live in `src/`, `#[cfg(test)]`-gated, not `tests/helpers/` (a separate crate `src/` can't
import from).

**Struct + queue + recording pattern to copy** (`tests/helpers/mock_llm_adapter.rs:65-78`):
```rust
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
    // ... add_response / add_success / add_failure / call_count / reset, all
    // Arc<Mutex<..>>-backed for Send + Sync async-test compatibility
}
```

**`#[async_trait]` port implementation pattern to copy** (`tests/helpers/mock_llm_adapter.rs:166-179`,
`237`): record the invocation, pop a canned response/error from the queue, return `Result<T, E>`.
This is exactly the shape `FailingChannelHandler` needs against `NotificationChannelHandler`.

**Concrete seam this mock targets** — `NotificationChannelHandler` trait
(`src/application/services/notification_orchestrator/types.rs:51-66`):
```rust
#[async_trait]
pub trait NotificationChannelHandler: Send + Sync {
    fn channel(&self) -> NotificationChannel;
    fn can_handle(&self, notification: &Notification) -> bool;
    async fn handle_notification(
        &self,
        notification: Notification,
    ) -> NotificationOrchestratorResult<NotificationDeliveryResult>;
    async fn health_check(&self) -> bool;
}
```

**Registration seam** — `NotificationService::register_channel_handler`
(`src/application/services/notification_orchestrator/mod.rs:424-429`), confirmed `pub async fn`:
```rust
pub async fn register_channel_handler(&self, handler: Arc<dyn NotificationChannelHandler>) {
    let channel = handler.channel();
    log::info!("Registering channel handler for: {:?}", channel);
    let mut handlers = self.channel_handlers.write().await;
    handlers.insert(channel, handler);
}
```
A `FailingChannelHandler::handle_notification` that always returns
`Err(NotificationOrchestratorError::DeliveryFailed(..))` forces the failure path
`user_service.rs:228` already handles non-blockingly — see next section.

**Barrel/re-export pattern** — `tests/helpers/mod.rs` is the existing barrel to mirror for
`src/test_support/mod.rs`'s module-declaration + re-export shape (not read in full this session;
follow the same `pub mod x; pub use x::*;` idiom visible from how `mock_llm_adapter` is consumed
elsewhere in `tests/helpers/`).

---

### `src/core/platform/manager/user_service.rs` — extend existing `#[cfg(test)] mod tests`

**Analog:** itself. The module already exists at `user_service.rs:467-583` with five
`#[tokio::test]`s built on real in-memory adapters — extend in place, do not replace.

**Imports pattern to extend** (`user_service.rs:469-476`):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::service::message_service::{MessageService, MessageServiceConfig};
    use crate::infrastructure::adapters::auth::InMemoryTokenAuthAdapter;
    use crate::infrastructure::adapters::logs::system_log_adapter::{
        SystemLogAdapter, SystemLogAdapterConfig,
    };
    use crate::infrastructure::repositories::sqlite_user_repository::SqliteUserRepository;
    use paladin_core::platform::container::notification::NotificationServiceConfig;
```

**Fixture pattern to copy/extend** (`user_service.rs:478-493`, `build_service`) — real in-memory
adapters, not mocks, is the established pattern (D-10's rationale for declaring `MockUserRepository`
unnecessary):
```rust
async fn build_service(with_auth: bool) -> UserService {
    let repo = Arc::new(SqliteUserRepository::new("sqlite::memory:").await.unwrap());
    let log_port =
        Arc::new(SystemLogAdapter::new_for_test(SystemLogAdapterConfig::default()).unwrap());
    let message_service = Arc::new(MessageService::new(MessageServiceConfig::default()));
    let notification_service = Arc::new(NotificationService::new(
        NotificationServiceConfig::default(),
        message_service,
    ));
    let service = UserService::new(repo, log_port, notification_service);
    if with_auth {
        service.with_auth_port(Arc::new(InMemoryTokenAuthAdapter::new()))
    } else {
        service
    }
}
```
DEFER-02's failure-path test needs a variant of this fixture that first calls
`notification_service.register_channel_handler(Arc::new(FailingChannelHandler::new()))` before
constructing/injecting it into `UserService::new`, to exercise `register_user:228`'s
`if let Err(e) = self.send_welcome_notification(...)` non-blocking branch — that branch already
exists and is correct; the new test proves it, per D-00/finding 6.

**Test pattern to copy** (`user_service.rs:504-515`, `delete_user_removes_the_user`):
```rust
#[tokio::test]
async fn delete_user_removes_the_user() {
    let service = build_service(false).await;
    let user = service
        .register_user(registration("alice", "alice@example.com"))
        .await
        .unwrap();

    service.delete_user(user.uuid).await.unwrap();

    assert!(service.get_user_by_id(user.uuid).await.unwrap().is_none());
}
```

**Registration/notification code under test** (`user_service.rs:190-235`, `register_user`) — the
exact non-blocking notification-failure branch DEFER-02 must cover:
```rust
async fn register_user(&self, request: UserRegistrationRequest) -> Result<User, UserError> {
    self.validate_username(&request.username)?;
    let email = Email::new(request.email)?;
    if self.user_repository.find_by_email(email.value()).await?.is_some() {
        return Err(UserError::EmailAlreadyExists(email.value().to_string()));
    }
    let password_hash = self.hash_password(&request.password)?;
    let user = User::new_user(request.username.clone(), email, password_hash, request.profile);
    let saved_user = self.user_repository.save(user).await?;
    self.log_action(LogLevel::Info, format!("User registered successfully: {}", request.username), Some(saved_user.uuid)).await;
    if let Err(e) = self.send_welcome_notification(&saved_user).await {
        self.log_action(LogLevel::Warn, format!("Failed to send welcome notification: {}", e), Some(saved_user.uuid)).await;
        // registration still succeeds — this is the behavior to prove, not change
    }
    // ...
}
```

**Validation/error-handling pattern** (`?` operator, domain `UserError` enum) is already established
throughout the file — DEFER-02's "invalid username/email/Unicode/empty" edge-case tests
(D-05's scope text) should call `register_user`/`validate_username`/`Email::new` directly and assert
on `UserError` variants, matching `delete_unknown_user_is_not_found`'s
`assert!(matches!(err, UserError::UserNotFound(_)))` idiom (`user_service.rs:517-522`).

---

### `src/application/services/orchestration/listener.rs` — extend existing `#[cfg(test)] mod tests`

**Analog:** itself. Existing module at `listener.rs:398-538`, three `#[tokio::test]`s, built on a
hand-rolled `MockEventListener` implementing the `EventListener` trait directly (not the
`Arc<Mutex<..>>` recording-mock shape — a simpler in-module fake).

**Imports + fixture pattern to extend** (`listener.rs:398-403`):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::component::action::Action;
    use crate::core::base::component::event::Event;
    use serde_json::json;

    struct MockEventListener {
        name: String,
        config: ListenerConfig,
        conditions: Vec<TriggerCondition>,
    }

    #[async_trait]
    impl EventListener for MockEventListener {
        // fn name / description / conditions / should_process / create_trigger /
        // config / update_config / health_check — all trivial trait passthroughs
    }
```

**Test pattern to copy** (`listener.rs:466-478`, `test_listener_registration`):
```rust
#[tokio::test]
async fn test_listener_registration() {
    let service = ListenerOrchestrator::new();
    let listener = Box::new(MockEventListener {
        name: "test_listener".to_string(),
        config: ListenerConfig::default(),
        conditions: vec![],
    });
    let result = service.register_listener(listener).await;
    assert!(result.is_ok());
    let listeners = service.list_listeners().await;
    assert!(listeners.contains(&"test_listener".to_string()));
}
```

DEFER-03's concurrency suite (CONTEXT.md's Claude's Discretion item on `tokio::time::pause()`/
`advance()`) has no existing analog in this file — the three current tests are all sequential
single-listener flows. New concurrency tests should still follow the same
`ListenerOrchestrator::new()` → register → `process_event` → assert-on-stats shape shown above, just
issuing concurrent `process_event` calls (e.g. via `tokio::join!` or a `JoinSet`) rather than
sequential ones. No in-repo concurrency-test analog exists for this specific orchestrator; std
`tokio::time` utilities need no wrapper per CONTEXT.md's discretion note.

---

## Shared Patterns

### GitHub Actions: already-correct toolchain/cache actions
**Source:** `.github/workflows/ci.yml` — `dtolnay/rust-toolchain@stable` (15+ existing sites) and
`actions/cache@v4` (`ci.yml:349-354` and elsewhere).
**Apply to:** All three new jobs (`coverage`, `cli-tests`, `bench-check`) and all eight
deprecated-action replacement sites (`ci.yml:163,408,788`; `integration-tests.yml:71,78,84,90,123`).
This is the dominant, already-established action set in the file — new jobs should not introduce any
other toolchain-install or cache action.

### Redis + MinIO service container block
**Source:** `.github/workflows/ci.yml:374-401` (`integration-tests` job).
**Apply to:** The new `coverage` job only (the only other job needing `--features integration-tests`
under D-01's scope). Copy verbatim; do not modify ports or health-check timing.

### Real in-memory adapters over hand-written mocks where one exists
**Source:** `user_service.rs:478-493` (`build_service`) — `SqliteUserRepository::new("sqlite::memory:")`,
`SystemLogAdapter::new_for_test`, `InMemoryTokenAuthAdapter`.
**Apply to:** DEFER-02's test extensions. Per D-10, `MockUserRepository` is not needed because this
fixture already fills that role — do not introduce a new mock repository.

### Arc<Mutex<..>> recording-mock shape for genuinely-needed test doubles
**Source:** `tests/helpers/mock_llm_adapter.rs` (full pattern).
**Apply to:** `src/test_support/failing_channel_handler.rs` (D-08/D-10) — the one new mock this phase
actually needs, relocated into `src/`, `#[cfg(test)]`-gated per D-08. Do not adopt `mockall` (D-09).

### Makefile target conventions
**Source:** `Makefile:113-186` (`##@ Testing` section), `Makefile:426-427` (`ci-test`).
**Apply to:** All five new/extended Makefile targets (`coverage`, `coverage-html`, `test-cli`,
`bench-check`, `ci-full`) — `.PHONY` + `## comment` + `@echo "$(CYAN)...$(NC)"` + `@$(CARGO) ...` is
the fixed shape every existing target in this file follows; no deviation observed anywhere in the
file.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `.codecov.yml` | config | report-upload consumption | No `.codecov.yml` exists anywhere in the tree (confirmed by direct search this session and by RESEARCH.md's independent read). Its content is fully specified by CONTEXT.md D-02/D-06 and RESEARCH.md's Standard Stack table — follow those documents directly rather than inventing a codebase precedent. |
| `bench-check` job body (compile-only benchmark CI job) | CI config / job | batch (compile-check) | No existing job in any of the six workflows runs `cargo bench --no-run` or any compile-only-benchmark pattern; the existing `benchmark` job (`ci.yml:779`) actually executes benchmarks and is a different (heavier) pattern, explicitly out of scope to reuse per CONTEXT.md D-06's sibling reasoning (`benchmark-regression-signal` already exists separately at `ci.yml:812` and is unrelated). Steps were assembled from `crate-isolation`'s build-only step shape (`ci.yml:367-369`) plus RESEARCH.md's documented `cargo bench --no-run` command — a partial, not exact, match. |

## Metadata

**Analog search scope:** `.github/workflows/*.yml` (all six workflows), `Makefile` (full file),
`tests/helpers/mock_llm_adapter.rs` (full read), `src/core/platform/manager/user_service.rs` (full
read), `src/application/services/orchestration/listener.rs` (header + full test-module read),
`src/application/services/notification_orchestrator/{mod.rs,types.rs}` (seam-confirmation reads).
**Files scanned:** 6 workflow files (grep), 1 Makefile (grep + targeted read), 4 Rust source files
(targeted/full reads), 1 mock helper file (full read).
**Pattern extraction date:** 2026-08-13

---

*Phase: 15-coverage-ci-quality-gates*
*Patterns mapped: 2026-08-13*
