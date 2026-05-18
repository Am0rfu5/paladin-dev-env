# PRD: Extract `paladin-battalion` Crate

**Epic:** Epic 3 — `paladin-battalion` Extraction
**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split
**Project:** Paladin Framework Refactoring Initiative
**Status:** Draft
**Author:** TBD
**Reviewers:** TBD
**Document Version:** 1.0
**Created:** 2026-05-17
**Last Updated:** 2026-05-17
**Target Audience:** Junior Developer

---

## 1. Introduction / Overview

### What Is This Feature?

This epic extracts the complete orchestration runtime from the monolithic `paladin` crate into a
dedicated `paladin-battalion` crate. The orchestration runtime lives at
`src/application/use_cases/battalion/` and consists of approximately 11,180 LOC across 13 files:
nine execution service modules, three utility/support modules, and a module declaration file.

The execution services include: `FormationExecutionService`, `PhalanxExecutionService`,
`CampaignExecutionService`, `ChainOfCommandExecutionService`, `ConclaveExecutionService`,
`CouncilExecutionService`, `GroveExecutionService`, `ManeuverExecutionService`, and the
`Commander` strategy router. The support utilities are `error_aggregation`, `flow_visualizer`,
and `retry`.

### What Problem Does It Solve?

The orchestration runtime is the primary public value of the Paladin framework — it is the
code that most consumers actually want. Yet today it is compiled as part of the monolithic
`paladin` crate, which means a consumer who only needs multi-agent orchestration must also
compile every LLM provider SDK, SQL driver, Redis client, MinIO client, and notification
adapter in the dependency graph.

By extracting these services into `paladin-battalion`, a downstream consumer who wants only
orchestration logic can declare three crates in their `Cargo.toml`:

```toml
paladin-core     = { version = "..." }  # domain types
paladin-ports    = { version = "..." }  # port trait contracts
paladin-battalion = { version = "..." } # orchestration runtime
```

…and the Rust toolchain will compile **nothing** from the infrastructure layer.

### Why Now?

Epic 1 extracted `paladin-core` (the domain types). Epic 2 extracted `paladin-ports` (the port
trait definitions). `paladin-battalion`'s execution services depend only on those two crates —
they use `Arc<dyn PaladinPort>`, `Arc<dyn PaladinRegistry>`, and similar trait objects and
never reference any concrete adapter implementation. With Epics 1 and 2 complete, the
dependencies of `paladin-battalion` are cleanly available and the extraction is straightforward.

---

## 2. Goals

1. **Create `crates/paladin-battalion/`** as an independently compilable crate with its own
   `Cargo.toml`.
2. **Move all 13 files** from `src/application/use_cases/battalion/` into
   `crates/paladin-battalion/src/`.
3. **Migrate all import paths** within the moved files: `crate::application::ports::` →
   `paladin_ports::`, `crate::core::` → `paladin_core::`, and internal cross-module references
   to bare `crate::` paths.
4. **Enforce dependency isolation** — `cargo tree -p paladin-battalion` must contain no
   transitive dependency on any infrastructure crate.
5. **Preserve backward compatibility** — all existing import paths resolve through the root
   `paladin` facade crate re-exports without modification.
6. **All 2,610+ workspace tests continue to pass** after extraction with zero regressions.
7. **All quality gates pass** — `cargo clippy --workspace -- -D warnings` and
   `cargo fmt --all --check` clean.

---

## 3. User Stories

### Story 1 — Orchestration-Only Consumer

> As a **downstream consumer** who only needs multi-agent orchestration, I want to add
> `paladin-battalion` to my `Cargo.toml` without pulling in LLM provider SDKs, database
> drivers, or Redis/MinIO clients, so that my compile times and binary size remain minimal.

**Acceptance:** A project that depends only on `paladin-core`, `paladin-ports`, and
`paladin-battalion` compiles successfully with `cargo build` and `cargo tree` shows none of
`reqwest`, `sqlx`, `redis`, `qdrant-client`, `actix-web`, or `lettre` in the dependency tree.

---

### Story 2 — Battalion Pattern Contributor

> As a **framework contributor** working on an orchestration pattern (e.g., improving Grove
> routing logic), I want to rebuild only `paladin-battalion` after my change so that I don't
> wait for LLM adapter, database, or web-server crates to recompile.

**Acceptance:** `cargo build -p paladin-battalion` succeeds in isolation. A change to
`grove_service.rs` does not trigger recompilation of any infrastructure crate.

---

### Story 3 — Existing Framework User

> As an **existing user** of the Paladin framework, I want my current import paths (e.g.,
> `use paladin::application::use_cases::battalion::formation_service::FormationExecutionService`)
> to continue working without modification so that upgrading to the workspace version requires
> no changes to my application code.

**Acceptance:** The root `paladin` facade crate re-exports all battalion public types at their
existing module paths. All workspace examples (`formation_sequential.rs`,
`campaign_workflow.rs`, `commander_basic.rs`, etc.) compile without import path changes.

---

### Story 4 — Maintainer Enforcing Architecture

> As a **maintainer**, I want the compiler to prevent any future battalion service from
> accidentally importing a concrete infrastructure adapter so that the hexagonal architecture
> boundary is enforced by the build system rather than code review.

**Acceptance:** Because `paladin-battalion`'s `Cargo.toml` does not list any infrastructure
crate as a dependency, any attempt to import one inside a battalion service produces a compiler
error — no linting tool or convention required.

---

### Story 5 — Junior Developer Implementing the Task List

> As the **implementer working from the task list**, I want clear guidance on the scripted
> import path migration so that I can update ~169 import occurrences accurately and efficiently
> without manually editing each line.

**Acceptance:** The task list derived from this PRD includes an explicit `sed` migration
command, a verification step using `cargo build -p paladin-battalion` to catch any missed
occurrences, and a rollback path (the originals are not deleted until all tests pass).

---

## 4. Functional Requirements

> **Note for implementer:** Each requirement below is numbered. All requirements must be
> satisfied for the epic to be considered complete.

### 4.1 `paladin-battalion` Crate Scaffold

- **FR-1:** The directory `crates/paladin-battalion/` must exist with a valid `Cargo.toml`
  and `src/lib.rs`.
- **FR-2:** `crates/paladin-battalion/Cargo.toml` must set `name = "paladin-battalion"`,
  `edition = "2021"`, and use `dep = { workspace = true }` syntax for all shared dependencies.
- **FR-3:** `paladin-battalion`'s `[dependencies]` must contain exactly these crates (all via
  workspace references where available): `paladin-core`, `paladin-ports`, `tokio`,
  `async-trait`, `serde`, `serde_json`, `uuid`, `log`, `futures`, `chrono`, `rand`,
  `tokio-util`, `petgraph`, `regex`. No other dependencies are permitted unless a gap is
  discovered during implementation and documented.
- **FR-4:** `paladin-battalion`'s `[dependencies]` must NOT contain any of the following, even
  transitively: `reqwest`, `actix-web`, `actix-http`, `sqlx`, `redis`, `qdrant-client`,
  `lettre`, `aws-sdk-*`, `minio`.
- **FR-5:** `cargo build -p paladin-battalion` must succeed in isolation (without building any
  other workspace member) after the crate is fully populated.

### 4.2 Extraction of Execution Service Files

- **FR-6:** All nine execution service files must be relocated from
  `src/application/use_cases/battalion/` to `crates/paladin-battalion/src/`:
  - `formation_service.rs`
  - `phalanx_service.rs`
  - `campaign_service.rs`
  - `chain_of_command_service.rs`
  - `conclave_execution_service.rs`
  - `council_service.rs`
  - `grove_service.rs`
  - `maneuver_service.rs`
  - `commander.rs`
- **FR-7:** All utility and support files must also be relocated to
  `crates/paladin-battalion/src/`:
  - `error_aggregation.rs`
  - `flow_visualizer.rs`
  - `retry.rs`

### 4.3 Import Path Migration

- **FR-8:** Every occurrence of `crate::application::ports::` in the extracted files must be
  replaced with `paladin_ports::`.
- **FR-9:** Every occurrence of `crate::core::` in the extracted files must be replaced with
  `paladin_core::`.
- **FR-10:** Every occurrence of `crate::application::use_cases::battalion::` (cross-module
  references within the battalion directory) must be replaced with `crate::` (since all
  modules now live in the same crate).
- **FR-11:** After migration, no extracted file may contain a `use` statement referencing
  `application::`, `infrastructure::`, or any path that implies access outside of
  `paladin-battalion`, `paladin-core`, or `paladin-ports`.
- **FR-12:** The total estimated migration scope is approximately 169 import-path occurrences
  across the 13 files. A scripted `sed` pass is the recommended approach (see Technical
  Considerations §6 for the exact command). A `cargo build -p paladin-battalion` verification
  step must follow the scripted pass to catch any residual misses.

### 4.4 Inline Unit Tests

- **FR-13:** All inline `#[cfg(test)]` test modules that exist within the extracted files must
  compile and pass when run via `cargo test -p paladin-battalion`. There are 12 such test
  modules spread across the service files — none may be dropped or disabled.

### 4.5 Root `paladin` Facade Crate Wiring

- **FR-14:** The root `paladin` crate's `Cargo.toml` must add `paladin-battalion` as a
  dependency: `paladin-battalion = { workspace = true }`.
- **FR-15:** `src/application/use_cases/battalion/mod.rs` must be converted to a re-export
  shim that re-exports all public items from `paladin_battalion` so that existing callers at
  paths like `crate::application::use_cases::battalion::formation_service::*` continue to
  resolve.
- **FR-16:** The original source files (`formation_service.rs`, `phalanx_service.rs`, etc.)
  must be deleted from `src/application/use_cases/battalion/` after the re-export shim is in
  place and `cargo test --workspace` passes. Only `mod.rs` (the re-export shim) remains.
- **FR-17:** After facade wiring, `cargo build --workspace` must succeed with zero errors and
  zero warnings.
- **FR-18:** After facade wiring, `cargo test --workspace` must pass all tests at or above the
  pre-epic baseline count (≥ 2,610 passing).

### 4.6 Dependency Validation

- **FR-19:** `cargo tree -p paladin-battalion` must be run after the crate is fully populated
  and its output must be inspected to confirm no forbidden infrastructure crate appears in the
  tree (see FR-4 for the forbidden list).
- **FR-20:** The output of `cargo tree -p paladin-battalion` must be saved to the file
  `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-dependency-tree.txt`.
- **FR-21:** The output of `cargo build -p paladin-battalion` (stdout + stderr) must be saved
  to `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-isolation-build.txt`.

### 4.7 Example and Integration Test Verification

- **FR-22:** The following workspace examples must compile via `cargo check --example <name>`
  after facade wiring: `formation_sequential`, `campaign_workflow`,
  `chain_of_command_delegation`, `commander_basic`, `commander_auto`,
  `commander_full_config`.
- **FR-23:** All workspace-level battalion tests must pass:
  - `tests/unit/battalion/` (6 test files: `campaign_service_tests.rs`,
    `campaign_tests.rs`, `chain_of_command_service_tests.rs`,
    `chain_of_command_tests.rs`, `formation_tests.rs`, `phalanx_tests.rs`)
  - `tests/integration/battalion/` (8 test files: `campaign_integration_test.rs`,
    `chain_of_command_integration_test.rs`, `council_integration_test.rs`,
    `formation_integration_test.rs`, `grove_integration_test.rs`,
    `load_test.rs`, `phalanx_integration_test.rs`)
  - Top-level integration test files: `tests/battalion_campaign_integration_test.rs`,
    `tests/battalion_chain_of_command_integration_test.rs`

### 4.8 Documentation

- **FR-24:** `crates/paladin-battalion/src/lib.rs` must include a crate-level `//!` doc
  comment that briefly describes the crate's purpose, lists the eight orchestration patterns,
  and provides a minimal usage example.
- **FR-25:** `cargo doc -p paladin-battalion --no-deps` must produce documentation with zero
  errors and zero warnings.

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** part of this epic:

- **No behavioral changes.** All orchestration logic must remain identical. This is a
  structural refactor only. If a bug is discovered during migration, it should be tracked
  separately and not fixed as part of this epic (to keep the diff reviewable).
- **No API shape changes.** Public types, method signatures, and error variants must not
  change.
- **No `paladin::prelude` module.** A convenience prelude is deferred to Epic 6
  (Facade, CI, and Finalization).
- **No test rewrites.** Existing tests are migrated verbatim. New tests are not required
  (though new doc-tests in `lib.rs` are welcome).
- **No extraction of other use-case modules.** Only `src/application/use_cases/battalion/`
  is in scope. The `content`, `herald`, `paladin`, `sanctum`, and `arsenal` use-case
  directories are not touched.
- **No CI pipeline updates.** Updating the CI workflow is scoped to Epic 6.
- **No migration of `tests/unit/battalion/` or `tests/integration/battalion/`** into
  `crates/paladin-battalion/tests/`. The workspace-level test directories remain in place.
  Full per-crate test isolation is deferred to Epic 6.
- **No extraction of `paladin-llm` or `paladin-memory`.** Those are Epics 4 and 5 and may
  proceed in parallel with this epic.

---

## 6. Design Considerations

### Directory Layout After Completion

```
paladin/
├── Cargo.toml                         # Workspace root (existing, updated)
├── crates/
│   ├── paladin-core/                  # Epic 1 (complete)
│   ├── paladin-ports/                 # Epic 2 (complete)
│   └── paladin-battalion/             # NEW — this epic
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                 # Crate root + //! doc comment
│           ├── formation_service.rs   # Moved from src/application/use_cases/battalion/
│           ├── phalanx_service.rs
│           ├── campaign_service.rs
│           ├── chain_of_command_service.rs
│           ├── conclave_execution_service.rs
│           ├── council_service.rs
│           ├── grove_service.rs
│           ├── maneuver_service.rs
│           ├── commander.rs
│           ├── error_aggregation.rs
│           ├── flow_visualizer.rs
│           └── retry.rs
│
├── src/                               # Root paladin facade crate (modified)
│   ├── lib.rs                         # Adds: pub use paladin_battalion;
│   └── application/
│       └── use_cases/
│           └── battalion/
│               └── mod.rs             # CONVERTED to re-export shim:
│                                      # pub use paladin_battalion::*;
│
└── tests/
    ├── unit/battalion/                # Unchanged — tests still reference
    └── integration/battalion/         # facade paths, which still resolve
```

### Facade Re-Export Strategy

The `src/application/use_cases/battalion/mod.rs` file is converted to a re-export shim. The
exact shim content must preserve the sub-module path structure. For example:

```rust
// src/application/use_cases/battalion/mod.rs (after conversion)
pub use paladin_battalion::formation_service;
pub use paladin_battalion::phalanx_service;
pub use paladin_battalion::campaign_service;
pub use paladin_battalion::chain_of_command_service;
pub use paladin_battalion::conclave_execution_service;
pub use paladin_battalion::council_service;
pub use paladin_battalion::grove_service;
pub use paladin_battalion::maneuver_service;
pub use paladin_battalion::commander;
pub use paladin_battalion::error_aggregation;
pub use paladin_battalion::flow_visualizer;
pub use paladin_battalion::retry;
```

This preserves all existing import paths of the form:
`use paladin::application::use_cases::battalion::formation_service::FormationExecutionService;`

The implementer may adjust this pattern if compilation errors reveal a different re-export
shape is needed — the invariant is zero broken import paths, not the exact re-export
mechanism.

---

## 7. Technical Considerations

### 7.1 Dependencies

All 14 required dependencies for `paladin-battalion` are already declared in the root
`Cargo.toml`. They need only be referenced with `dep = { workspace = true }` in the new
crate's manifest:

| Dependency | Why Needed |
|------------|-----------|
| `paladin-core` | Domain types: `BattalionError`, `Paladin`, `Formation`, `Phalanx`, `Campaign`, etc. |
| `paladin-ports` | Port traits: `PaladinPort`, `PaladinRegistry`, `LlmPort`, `EmbeddingPort` |
| `tokio` | Async runtime, `timeout`, `mpsc`, `Semaphore` |
| `async-trait` | `#[async_trait]` macro on service methods |
| `serde` / `serde_json` | Serialization of service configs and results |
| `uuid` | Execution ID generation |
| `log` | `debug!`, `info!`, `warn!` macros throughout services |
| `futures` | `BoxFuture`, `FutureExt`, `select_ok` in `phalanx_service` |
| `chrono` | Timestamps in `phalanx_service` |
| `rand` | Retry jitter (`retry.rs`), Conclave sampling (`conclave_execution_service.rs`) |
| `tokio-util` | `CancellationToken` in `phalanx_service.rs` |
| `petgraph` | DAG topological sort in `campaign_service.rs` |
| `regex` | Edge condition matching in `campaign_service.rs` |

### 7.2 Import Path Migration

The ~169 import-path occurrences within the battalion files require updating after the files
are copied. A single `sed` pass handles the bulk of the migration:

```bash
find crates/paladin-battalion/src -name "*.rs" \
  -exec sed -i \
    -e 's/crate::application::ports::/paladin_ports::/g' \
    -e 's/crate::application::use_cases::battalion::/crate::/g' \
    -e 's/crate::core::/paladin_core::/g' \
  {} \;
```

After running this command, `cargo build -p paladin-battalion` will surface any residual
unresolved paths — fix those manually and document each fix.

### 7.3 `PaladinError` Usage

`phalanx_service.rs` imports `PaladinError` from
`crate::application::use_cases::paladin::error::PalalandinError`. Before extracting that file,
verify where `PaladinError` is accessible from in the new dependency graph:

- If `PaladinError` was moved to `paladin-core` or `paladin-ports` during Epics 1–2, update
  the import accordingly.
- If it remains in the root `paladin` crate's use-cases layer, a solution must be found before
  `phalanx_service.rs` can be fully extracted (options: move `PaladinError` to `paladin-core`,
  or re-export it through `paladin-ports`). This must be resolved in Task 1.0 of the task list
  before any code is moved.

### 7.4 Feature Flag Audit

Before extracting any file, run:
```bash
grep -rn "#\[cfg(feature" src/application/use_cases/battalion/
```
If any feature-gated code is found, those feature flag names must be added to
`paladin-battalion/Cargo.toml`'s `[features]` section before the corresponding file is moved.

### 7.5 Safe Migration Order

The originals in `src/application/use_cases/battalion/` must **not** be deleted until:
1. All 13 files are successfully copied and compiling in `paladin-battalion`.
2. The facade re-export shim is in place.
3. `cargo test --workspace` passes at or above baseline.

This ensures the workspace remains in a buildable state throughout the migration and provides
a rollback path if an issue is discovered late in the process.

---

## 8. Success Metrics

The epic is complete when **all** of the following are true:

1. `cargo build -p paladin-battalion` exits with code 0.
2. `cargo test -p paladin-battalion` exits with code 0 (all inline tests pass).
3. `cargo tree -p paladin-battalion` shows none of the forbidden infrastructure crates
   (`reqwest`, `sqlx`, `redis`, `qdrant-client`, `actix-web`, `lettre`, etc.).
4. `cargo test --workspace` passes at or above the pre-epic baseline of 2,610 tests.
5. `cargo clippy --workspace -- -D warnings` exits with code 0.
6. `cargo fmt --all --check` exits with code 0.
7. `cargo doc -p paladin-battalion --no-deps` exits with code 0 and zero warnings.
8. All six spot-check examples compile via `cargo check --example <name>`.
9. `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-dependency-tree.txt`
   exists and contains the clean `cargo tree` output.
10. `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-isolation-build.txt`
    exists and contains the clean `cargo build -p paladin-battalion` output.

---

## 9. Open Questions

1. **`PaladinError` location:** `phalanx_service.rs` imports `PaladinError` from the root
   crate's use-cases layer. Epics 1 and 2 may have already relocated it — this must be
   confirmed before implementing Task 2.0. If it has not been relocated, a resolution decision
   (move to `paladin-core`? add to `paladin-ports`?) must be made and recorded before any
   extraction begins.

2. **Feature-flag guards:** A grep for `#[cfg(feature = "...")]` inside the battalion files
   must be run before extraction begins (Task 1.0). If any flags are found, they must be
   mirrored in `paladin-battalion/Cargo.toml`. This is unlikely but must be verified.

3. **Workspace-level test location:** The `tests/unit/battalion/` and
   `tests/integration/battalion/` directories are currently at the workspace level and
   reference facade import paths. After the re-export shim is in place, these tests should
   continue to compile unchanged. However, if compilation errors arise (e.g., re-export path
   resolution differences), they may need to be updated to import directly from
   `paladin_battalion::`. The preferred resolution is to keep them at the workspace level and
   fix import paths if needed — moving them into `crates/paladin-battalion/tests/` is
   deferred to Epic 6.

4. **`petgraph` version alignment:** `petgraph` is used by both `paladin-core` (Campaign DAG
   domain types) and `paladin-battalion` (Campaign execution service topological sort). Confirm
   both use the same workspace-pinned version to avoid duplicate compilation of `petgraph`.
