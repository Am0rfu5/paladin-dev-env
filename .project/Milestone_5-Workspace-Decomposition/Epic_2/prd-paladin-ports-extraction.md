# PRD: Extract `paladin-ports` Crate

> **Correction (dated 2026-08-06, ADR-0016 and ADR-0014):** FR-7 and FR-10 as written would move
> `PaladinResult`, `StopReason` and `TokenUsage` out of `paladin-core` and back into this crate,
> reintroducing the upward dependency the Milestone 5 Epic 1 decision record removed. See
> [`../../../.planning/decisions/0016-port-value-type-ownership.md`](../../../.planning/decisions/0016-port-value-type-ownership.md)
> for the corrected answer: `paladin-core` owns those three types, and this crate holds thin
> re-exports, extending FR-11's existing `RegistryError` carve-out.
>
> Separately, ADR-0014
> ([`../../../.planning/decisions/0014-milestone-4-6-tier-numbering.md`](../../../.planning/decisions/0014-milestone-4-6-tier-numbering.md))
> flagged a possible §1 "Milestone 1 / Epic 2" cross-reference in this document (CONTEXT.md D-08(5),
> mirroring `prd-paladin-llm-extraction.md` Non-Goal 2's "hardened in Milestone 1 / Epic 2").
> Re-verified during this correction (`grep -in "milestone 1"` and `grep -in "milestone-1"` across
> the whole document, 2026-08-06): no such text exists in this document as it currently ships —
> only this file's own correct self-identification as "Milestone 5 (Tier 2)" below. No §1 numbering
> correction is made here for that reason. Inside these three milestones, "Milestone 1" would mean
> the Tier 1 label for Milestone 4 under ADR-0014's convention, if such a reference is ever added to
> this document in the future.
>
> Original text is retained below with inline corrections; nothing is deleted.

**Epic:** Epic 2 — Extract `paladin-ports` Crate
**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split
**Project:** Paladin Framework Refactoring Initiative
**Status:** Draft
**Author:** TBD
**Reviewers:** TBD
**Document Version:** 1.1
**Created:** 2026-05-15
**Last Updated:** 2026-05-15
**Target Audience:** Junior Developer

---

## 1. Introduction / Overview

### What Is This Feature?

This epic extracts the port trait definitions from `src/application/ports/` (~6k LOC, ~25 trait files) into a dedicated `paladin-ports` crate. Port traits are the architectural contracts that define **what** the application layer requires from its adapters — things like "I need something that can store memories," "I need something that can call an LLM," or "I need something that can store files." These contracts are defined as Rust traits, and they are the backbone of the hexagonal architecture.

### What Problem Does It Solve?

After Epic 1, all pure domain types (Paladin, Battalion types, Garrison, Sanctum, etc.) live in `paladin-core`. However, the architectural contracts — the port traits — still live inside the monolithic `paladin` crate's `src/application/ports/` directory. This means:

1. Any downstream crate that wants to **implement** a port trait (e.g., a new LLM adapter) must still depend on the entire `paladin` crate, pulling in all its infrastructure dependencies.
2. Any downstream crate that wants to **use** a port trait to declare a dependency (e.g., `fn new(llm: Arc<dyn LlmPort>)`) also pulls in the entire framework.
3. There is no independent compilation of port traits in isolation, so every change to infrastructure code forces a full recompile of the port definitions.

Extracting port traits into `paladin-ports` solves this by creating a minimal, stable crate that:
- Only depends on `paladin-core` (for domain types in trait signatures) and a handful of utility crates.
- Can be compiled and tested in isolation.
- Serves as the single dependency for both adapter implementors and application service authors.

### Why `paladin-ports` Second?

`paladin-ports` depends on `paladin-core` (domain types appear in trait signatures) but on nothing else from the workspace. Every subsequent crate — `paladin-battalion`, `paladin-llm`, `paladin-memory` — will depend on `paladin-ports`. It must be extracted before any of those later epics begin.

### Spec-Driven Development (SDD) Note

This PRD is a **specification**, not an implementation plan. It defines *what* must be true when the epic is complete. A separate **task list** will be derived from this PRD. The task list will follow the TDD discipline (Red-Green-Refactor) and the Rust completion protocol defined in the project's coding instructions.

---

## 2. Goals

1. **Scaffold `crates/paladin-ports/`** — a new crate with its own `Cargo.toml` and `src/lib.rs`.
2. **Extract all output port traits** from `src/application/ports/output/` into `paladin-ports`.
3. **Extract all input port traits** from `src/application/ports/input/` into `paladin-ports`.
4. **Co-locate associated types** — each port's error types, request/response structs, config types, and supporting enums move alongside their trait into `paladin-ports`.
5. **Wire `paladin-ports` into the root `paladin` crate** — add `paladin-ports` as a direct dependency and update `src/lib.rs` re-exports to resolve from `paladin_ports::` directly.
6. **Fully delete `src/application/ports/`** — all original port source files are deleted. Every reference to `crate::application::ports::` across the codebase is migrated to `paladin_ports::` or `crate::paladin_ports::` via the updated `src/lib.rs` re-exports.
7. **Update all import sites** — all 314 import-path references across 76 `.rs` files (infrastructure adapters, use-case services, `src/lib.rs`, and inline doc examples) and 12 references in 5 docs markdown files are updated to the new paths.
8. **All existing tests continue to pass** — no regressions are acceptable.

---

## 3. User Stories

### Story 1 — LLM Adapter Author

> As a **developer writing a new LLM provider adapter**, I want to depend only on `paladin-ports` (not the full `paladin` crate) so that I don't have to compile Redis, MinIO, SQLite, or other infrastructure crates just to implement `LlmPort`.

**Acceptance:** A crate that adds `paladin-ports` to its `[dependencies]` can `impl LlmPort for MyAdapter` and compile without any transitive dependency on `redis`, `sqlx`, `aws-sdk-s3`, `reqwest` (the MinIO/storage client), or any LLM SDK from other providers.

---

### Story 2 — Application Service Author

> As a **developer writing an application service** that takes a `GarrisonPort` dependency, I want to declare that dependency via `paladin-ports` so that my service crate remains independent of the specific storage backend chosen at runtime.

**Acceptance:** A service crate can import `GarrisonPort` from `paladin_ports::output::garrison_port` and use it in function signatures without depending on `sqlx`, the Redis adapter, or any specific Garrison implementation.

---

### Story 3 — Downstream Library Consumer

> As a **library consumer** using Paladin in my own application, I want the existing import paths (e.g., `paladin::application::ports::output::llm_port::LlmPort`) to continue working after this refactor so that I do not need to change my code.

**Acceptance:** All public import paths that existed before this epic continue to resolve correctly after the epic is complete. The root `paladin` crate re-exports every type at its pre-existing path.

---

### Story 4 — Maintainer Enforcing Architecture

> As a **maintainer**, I want the Rust compiler to guarantee that port trait definitions never import from the infrastructure layer so that adapter-layer code can never sneak into the contract definitions.

**Acceptance:** `paladin-ports`'s `Cargo.toml` dependency list contains only `paladin-core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, and `tokio`. Any attempt to import from `infrastructure::` would fail to compile because that crate is not in `paladin-ports`'s dependency graph.

---

### Story 5 — Framework Contributor

> As a **framework contributor** working on a port trait definition (e.g., adding a new method to `SanctumPort`), I want to compile and test only `paladin-ports` in isolation so that I don't wait for LLM adapters, storage backends, or the web server to compile.

**Acceptance:** `cargo build -p paladin-ports` and `cargo test -p paladin-ports` succeed without building any other workspace member.

---

## 4. Functional Requirements

> **Note for implementer:** Each requirement below is numbered. All requirements must be satisfied for the epic to be considered complete.

### 4.1 `paladin-ports` Crate Scaffold

- **FR-1:** The directory `crates/paladin-ports/` must exist with a valid `Cargo.toml` and `src/lib.rs`.
- **FR-2:** `crates/paladin-ports/Cargo.toml` must set `name = "paladin-ports"`, `edition = "2021"`, and reference workspace dependencies using `dep = { workspace = true }` syntax.
- **FR-3:** `paladin-ports`'s `[dependencies]` section must contain only: `paladin-core` (path dependency to `crates/paladin-core`), `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, and `tokio`. No other dependencies are permitted without explicit justification and PRD update.
- **FR-4:** `cargo build -p paladin-ports` must succeed in isolation (without building any other workspace member beyond its declared dependencies).
- **FR-5:** `crates/paladin-ports/src/lib.rs` must declare `pub mod input;` and `pub mod output;` and include crate-level doc comments describing the crate's purpose in the hexagonal architecture.
- **FR-6:** `paladin-ports` must include **all** port modules unconditionally — including `vision_llm_port` and `vision_port`. `paladin-ports` does not define any feature flags of its own. Feature-gating of vision functionality is the responsibility of the root `paladin` crate, which controls what it re-exports from `paladin-ports`. The `#[cfg(feature = "vision")]` attributes currently on `vision_llm_port` and `vision_port` in `src/application/ports/output/mod.rs` must **not** be carried into `paladin-ports/src/output/mod.rs`.

### 4.2 Extraction of Output Port Traits

- **FR-7:** ~~All files from `src/application/ports/output/` must be relocated to `crates/paladin-ports/src/output/`. The full inventory of files to extract is:~~
  **Corrected (ADR-0016, dated 2026-08-06):** `llm_port.rs` and `paladin_port.rs` relocate as
  files, but their `TokenUsage`, `PaladinResult` and `StopReason` type bodies do not travel with
  them — per ADR-0016, FR-11's `RegistryError` core-re-export carve-out (below) extends to these
  three types, so `paladin-ports` holds thin re-exports of the `paladin-core` definitions rather
  than independent bodies. Original text retained above, superseded; the file inventory table
  below is otherwise unaffected. The full inventory of files to extract is:

  | File | Primary Exported Types |
  |------|----------------------|
  | `llm_port.rs` | `LlmPort`, `LlmRequest`, `LlmResponse`, `LlmError`, `TokenUsage`, `FinishReason`, `StreamingResponse`, `ToolCall`, `ToolResult` |
  | `garrison_port.rs` | `GarrisonPort`, `LongTermGarrisonPort`, `GarrisonError`, `GarrisonStats` |
  | `sanctum_port.rs` | `SanctumPort`, `SanctumError`, `SanctumQuery`, `SanctumFilter`, `SanctumSearchResult` |
  | `embedding_port.rs` | `EmbeddingPort`, `Embedding`, `EmbeddingError` |
  | `arsenal_port.rs` | `ArsenalPort`, `ArsenalRegistry` |
  | `citadel_port.rs` | `CitadelPort` |
  | `queue_port.rs` | `QueuePort` |
  | `notification_port.rs` | `NotificationPort` and all associated notification types |
  | `file_storage_port.rs` | `FileStoragePort`, `FileStorageError` |
  | `paladin_port.rs` | `PaladinPort`, `PaladinResult`, `StopReason` |
  | `paladin_executor_port.rs` | `PaladinExecutorPort` and associated types |
  | `paladin_registry.rs` | `PaladinRegistry`, `RegistryError` |
  | `battalion_port.rs` | `BattalionPort` and associated types |
  | `log_port.rs` | `LogPort` and associated types |
  | `scheduler_port.rs` | `SchedulerPort` and associated types |
  | `search_engine_port.rs` | `SearchPort` (or equivalent) and associated types |
  | `content_delivery_port.rs` | `ContentDeliveryPort` and associated types |
  | `vision_llm_port.rs` | `VisionCapableLlm` |
  | `vision_port.rs` | `VisionPort` |

- **FR-8:** The module tree structure (`mod.rs` declarations, sub-modules) within `src/output/` must be preserved identically in `crates/paladin-ports/src/output/`.
- **FR-9:** All `use` statements within moved output port files must be updated so that:
  - Domain types imported from `crate::core::platform::container::*` are replaced with imports from `paladin_core::platform::container::*`.
  - Self-referential imports within `paladin-ports` (e.g., a port file importing another port's error type) use `crate::output::*`.
  - No moved file contains a `use` statement referencing `crate::application::`, `crate::infrastructure::`, or `crate::core::`.
- **FR-10:** ~~All associated types that are defined **within** a port module file (error enums, request/response structs, config structs, supporting enums) must move with their port trait into `paladin-ports`. Types must not be split across crates.~~
  **Corrected (ADR-0016, dated 2026-08-06):** FR-11's `RegistryError` core-re-export carve-out
  extends to `PaladinResult`, `StopReason` and `TokenUsage` — these three types are defined in
  `paladin-core`, and `paladin-ports` holds thin re-exports, not independent bodies. Original text
  retained above, superseded. All other associated types not named by this carve-out still move
  with their port trait per the original rule.
- **FR-11:** `RegistryError`, which is currently re-exported in `paladin_registry.rs` from `crate::core::platform::container::registry_error`, must remain accessible from `paladin_ports::output::paladin_registry::RegistryError`. If the underlying type lives in `paladin-core`, the re-export must be updated to `paladin_core::platform::container::registry_error::RegistryError`.

### 4.3 Extraction of Input Port Traits

- **FR-12:** All files from `src/application/ports/input/` must be relocated to `crates/paladin-ports/src/input/`. The full inventory is:

  | File | Primary Exported Types |
  |------|----------------------|
  | `content_input_port.rs` | `ContentIngestionPort` |
  | `document_port.rs` | `DocumentPort` and associated types |
  | `listener_port.rs` | `ListenerPort` (or equivalent) and associated types |
  | `ml_port.rs` | `MlPort` and associated types |
  | `nlp_port.rs` | `NlpPort` and associated types |
  | `rpc_port.rs` | `RpcGatewayPort` (or equivalent) and associated types |

- **FR-13:** All `use` statements within moved input port files must follow the same rules as FR-9: no references to `crate::application::`, `crate::infrastructure::`, or `crate::core::`.
- **FR-14:** Domain types imported from the old monolith path (e.g., `crate::core::platform::container::content::ContentItem`) must be updated to their `paladin-core` equivalents.

### 4.4 Root `paladin` Crate Updates

- **FR-15:** `paladin-core = { path = "crates/paladin-core" }` must already exist in the root `paladin` crate's `[dependencies]` (satisfied by Epic 1). Add `paladin-ports = { path = "crates/paladin-ports" }` alongside it.
- **FR-16:** The entire `src/application/ports/` directory must be **fully deleted** after all port modules have been successfully extracted to `paladin-ports` and all import sites have been updated. No shim files are left behind.
- **FR-17:** The top-level re-exports in `src/lib.rs` (e.g., `pub use application::ports::output::llm_port::{...}`) must be updated to resolve from `paladin_ports::` directly (e.g., `pub use paladin_ports::output::llm_port::{...}`). All public types that were previously accessible at `paladin::application::ports::*` paths from external consumers must continue to be re-exported from `src/lib.rs` at equivalent paths.
- **FR-18:** The `pub mod application;` declaration in `src/lib.rs` and the `src/application/ports/mod.rs`, `src/application/ports/input/mod.rs`, and `src/application/ports/output/mod.rs` files must be removed or updated so that `src/application/ports/` no longer exists as a module path in the compiled output.

### 4.5 Full Import Migration

- **FR-19:** All 314 occurrences of `use crate::application::ports::` (or equivalent path forms) across 76 `.rs` source files must be updated to import from `paladin_ports::` directly or from the updated `src/lib.rs` re-exports. This migration spans all source layers: `src/infrastructure/`, `src/application/use_cases/`, `src/application/`, and `src/lib.rs` itself.
- **FR-20:** The import path updates described in FR-19 must not change the behavior of any code. Only the `use` statement path strings change; no function bodies, trait implementations, struct definitions, or test assertions may change.
- **FR-21:** Rustdoc examples embedded in port files (many of the 314 occurrences are inside `//!` or `///` doc comment code blocks) must also be updated to use valid import paths that compile under `cargo test --doc`. Broken doc examples are treated as test failures.

### 4.6 Documentation File Updates

- **FR-22:** The 12 occurrences of `application::ports::` paths in 5 files under `docs/` (Markdown documentation) must be updated to reflect the new import paths. The docs files must accurately reflect how consumers import port traits after the refactor.

### 4.7 Dependency Layering Validation

- **FR-23:** After the epic is complete, `cargo tree -p paladin-ports` must show **no** transitive dependency on any of: `redis`, `sqlx`, `aws-sdk-s3`, `minio`, `reqwest` (in a context where it would originate from a storage/queue client), `openai`, `anthropic`, or any LLM provider SDK.
- **FR-24:** `cargo tree -p paladin-ports` must show `paladin-core` as a direct dependency and no other workspace crates.
- **FR-25:** `cargo build -p paladin-ports` must succeed independently. The output of this command, once confirmed green, must be saved to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-isolation-build.txt`.
- **FR-26:** The output of `cargo tree -p paladin-ports` must be saved to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-dependency-tree.txt`.

### 4.8 Tests and Documentation

- **FR-27:** All unit tests that previously lived in `src/application/ports/` (inside `#[cfg(test)]` modules) must compile and pass when run via `cargo test -p paladin-ports`.
- **FR-28:** All existing workspace tests (`cargo test --workspace`) must continue to pass with zero regressions.
- **FR-29:** All public items in `paladin-ports` (traits, structs, enums, type aliases) must have rustdoc comments (`///`). Existing doc comments from the source files must be preserved; no documentation may be lost during the move.
- **FR-30:** `cargo doc -p paladin-ports --no-deps` must produce clean output with no broken intra-doc links.

---

## 5. Non-Goals (Out of Scope)

- **No behavioral changes.** This epic is a structural refactor only. No trait method signatures, error variants, or associated types may change.
- **No new port traits.** Do not add any port trait that does not already exist in `src/application/ports/`.
- **Import path updates only — no logic changes.** The 314 import-path updates across 76 files change `use` statement strings only. No function body, trait implementation, or test assertion logic changes.
- **No extraction of `paladin-battalion`, `paladin-llm`, or `paladin-memory`.** Those are subsequent epics. This epic only establishes the ports crate.
- **No changes to `paladin-core`.** The domain types in `paladin-core` must not be modified as part of this epic.
- **No CLI changes.** The CLI binary and its configuration remain untouched.

---

## 6. Design Considerations

### Module Structure Inside `paladin-ports`

```
crates/paladin-ports/
├── Cargo.toml
└── src/
    ├── lib.rs                      # crate-level docs, pub mod input/output
    ├── input/
    │   ├── mod.rs                  # pub mod declarations
    │   ├── content_input_port.rs
    │   ├── document_port.rs
    │   ├── listener_port.rs
    │   ├── ml_port.rs
    │   ├── nlp_port.rs
    │   └── rpc_port.rs
    └── output/
        ├── mod.rs                  # pub mod declarations (all unconditional)
        ├── arsenal_port.rs
        ├── battalion_port.rs
        ├── citadel_port.rs
        ├── content_delivery_port.rs
        ├── embedding_port.rs
        ├── file_storage_port.rs
        ├── garrison_port.rs
        ├── llm_port.rs
        ├── log_port.rs
        ├── notification_port.rs
        ├── paladin_executor_port.rs
        ├── paladin_port.rs
        ├── paladin_registry.rs
        ├── queue_port.rs
        ├── sanctum_port.rs
        ├── scheduler_port.rs
        ├── search_engine_port.rs
        ├── vision_llm_port.rs      # always included; feature-gating done by root crate
        └── vision_port.rs          # always included; feature-gating done by root crate
```

### Full Deletion + Mass Import Migration (FR-16)

`src/application/ports/` is fully deleted after extraction. All callers are updated to import from `paladin_ports::` directly. The measured scope of this migration is:

- **314 import-path occurrences** across **76 `.rs` files** (many inside rustdoc `//!` and `///` code blocks)
- **12 occurrences** across **5 `docs/` Markdown files**

A sed-based or `cargo fix`-assisted bulk rename pass is the recommended approach. The exact substitution pattern (e.g., `s/crate::application::ports::/paladin_ports::/g` in files under `src/`) should be scripted, then followed by `cargo build --workspace` to catch any misses.

After migration, `src/lib.rs` re-exports update from:

```rust
// Before
pub use application::ports::output::llm_port::{LlmPort, LlmRequest, LlmResponse};
```

to:

```rust
// After
pub use paladin_ports::output::llm_port::{LlmPort, LlmRequest, LlmResponse};
```

### Vision Port Handling (FR-6)

`paladin-ports` exports `vision_llm_port` and `vision_port` unconditionally — no `[features]` section needed in `paladin-ports/Cargo.toml`. The root `paladin` crate continues to gate these behind its existing `vision` feature by conditionally re-exporting them from `src/lib.rs`:

```rust
// src/lib.rs (root crate)
#[cfg(feature = "vision")]
pub use paladin_ports::output::vision_llm_port::VisionCapableLlm;
#[cfg(feature = "vision")]
pub use paladin_ports::output::vision_port::VisionPort;
```

The `vision` feature flag remains in the root `paladin` crate's `Cargo.toml` exactly as before.

---

## 7. Technical Considerations

### Dependency on `paladin-core`

`paladin-ports` depends on `paladin-core` for domain types referenced in trait signatures. Examples:

- `GarrisonPort` methods reference `GarrisonEntry` and `ConversationRole` (from `paladin-core::platform::container::garrison`).
- `SanctumPort` methods reference `SanctumEntry` and `Memory` (from `paladin-core::platform::container::sanctum`).
- `PaladinPort` methods reference `Paladin` (from `paladin-core::platform::container::paladin`).
- `PaladinRegistry` methods reference `Paladin` (same).
- `ContentIngestionPort` methods reference `ContentItem` (from `paladin-core::platform::container::content`).

All of these types are already in `paladin-core` after Epic 1. No new types need to be added to `paladin-core` as part of this epic.

### `RegistryError` Location

`paladin_registry.rs` currently re-exports `RegistryError` from `crate::core::platform::container::registry_error`. After Epic 1, this type lives at `paladin_core::platform::container::registry_error::RegistryError`. The re-export in `paladin-ports` must point to that location. The public path `paladin_ports::output::paladin_registry::RegistryError` must continue to work.

### `CitadelError` Lives in `application::errors`

`CitadelError` is currently defined in `src/application/errors/citadel_error.rs` and re-exported from `src/lib.rs`. This type is **not** a port trait and is **not** in scope for this epic. It should remain in the `paladin` crate's `application::errors` module. Only port traits and their directly associated types (request/response/error types defined **within** the port module file) are in scope.

### Tokio Dependency in `paladin-ports`

Several streaming port methods use `tokio::sync::mpsc` channels (e.g., `LlmPort::generate_stream`). `tokio` must therefore appear in `paladin-ports`'s `[dependencies]` with at minimum the `sync` feature. The workspace-level `tokio` declaration (added in Epic 1 with `features = ["full"]`) covers this.

### Incremental Extraction Strategy

To minimize risk, the recommended extraction sequence within this epic is:

1. Scaffold the empty `paladin-ports` crate and confirm `cargo build -p paladin-ports` passes.
2. Extract output ports one file at a time, running `cargo build --workspace` after each file.
3. Extract input ports one file at a time.
4. Update `src/lib.rs` re-exports to resolve from `paladin_ports::` directly.
5. Run a bulk find-and-replace to migrate the 314 import-path occurrences in `.rs` files (scripted pass, then `cargo build --workspace` to verify).
6. Run a bulk update for the 12 occurrences in `docs/` Markdown files.
7. Delete `src/application/ports/` and remove its `pub mod` declaration.
8. Run full test suite.

This approach ensures that any compilation error is isolated to a single file change.

---

## 8. Success Metrics

1. `cargo build -p paladin-ports` succeeds in isolation with no errors or warnings.
2. `cargo test -p paladin-ports` runs all unit tests extracted from port modules with zero failures.
3. `cargo test --workspace` reports the same number of passing tests as the Epic 1 baseline (zero regressions).
4. `cargo tree -p paladin-ports` contains zero entries from: `redis`, `sqlx`, any object storage SDK, or any LLM provider SDK.
5. `cargo tree -p paladin-ports` lists exactly one workspace-internal dependency: `paladin-core`.
6. `cargo clippy --workspace -- -D warnings` reports zero warnings.
7. `cargo fmt --all --check` passes without changes.
8. `cargo doc -p paladin-ports --no-deps` completes with zero broken intra-doc link errors.
9. At least three existing examples (`basic_paladin.rs`, `formation_sequential.rs`, `garrison_in_memory.rs`) pass `cargo check --example <name>` confirming no public import path was broken.

---

## 9. Resolved Design Decisions

The following questions were raised during PRD review and resolved before finalization:

1. **Port file scope (all five unspecified files):** `content_delivery_port.rs`, `listener_port.rs`, `paladin_executor_port.rs`, `scheduler_port.rs`, and `search_engine_port.rs` are all included in the extraction. The Epic 2 outline's intent is "all input ports" and "all output ports", so every file currently under `src/application/ports/` is in scope.

2. **Backward-compatibility strategy:** **Full deletion selected** (Option B). The existing import paths throughout the codebase (`use crate::application::ports::`) are migrated via a scripted bulk find-and-replace covering 314 occurrences in 76 `.rs` files and 12 occurrences in 5 `docs/` Markdown files. This produces a cleaner final state with no shim debt.

3. **Vision port feature flags:** **No feature flag in `paladin-ports`** (Option B). Both `vision_llm_port` and `vision_port` are exported unconditionally from `paladin-ports`. The root `paladin` crate retains its existing `vision` feature flag and gates the re-exports of those types behind `#[cfg(feature = "vision")]` in `src/lib.rs`. No additional dependencies are required in `paladin-ports` for vision support because `VisionRequest` is a `paladin-core` type.
