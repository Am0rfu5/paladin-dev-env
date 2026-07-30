# PRD: Extract `paladin-memory` Crate

**Epic:** Epic 5 — Extract `paladin-memory` Crate
**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split
**Project:** Paladin Framework Refactoring Initiative
**Status:** Draft
**Author:** TBD
**Reviewers:** TBD
**Document Version:** 1.1
**Created:** 2026-05-19
**Last Updated:** 2026-05-19
**Target Audience:** Junior Developer

---

## 1. Introduction / Overview

### What Is This Feature?

This epic extracts all memory and vector-storage adapter implementations — and the application-layer services that orchestrate them — out of the monolithic `paladin` crate and into a new, independent `paladin-memory` crate. The new crate consolidates six concrete components under three feature-flagged modules:

| Module | Component | Feature gate |
|--------|-----------|--------------|
| `garrison/` | `InMemoryGarrison` | always on |
| `garrison/` | `SqliteGarrison` | `sqlite` |
| `garrison/` | `TokenCounter` / `TiktokenCounter` | `content-processing` |
| `sanctum/` | `InMemorySanctum` | always on |
| `sanctum/` | `QdrantSanctumAdapter` | `qdrant` |
| `services/` | `MemoryExtractionService` + `RagRetrievalService` | always on |

A `paladin_memory::prelude` module provides a single-import convenience for the most commonly used types.

### What Problem Does It Solve?

Currently, memory-related code is scattered across three separate areas of the monolith:

- **Garrison adapters** live in `src/infrastructure/adapters/garrison/` (3 files).
- **Sanctum adapters** live in `src/infrastructure/adapters/sanctum/` (2 files).
- **Memory services** live in `src/application/use_cases/sanctum/` (2 files).

This creates three concrete problems for developers:

1. **Unnecessary heavy dependencies are always compiled.** A project that uses only in-memory storage still compiles `sqlx` (SQLite) and `qdrant-client` unless the developer manually opts out. Neither dependency is light — `sqlx` adds dozens of transitive crates; `qdrant-client` brings in gRPC and protobuf machinery. With a dedicated crate, enabling `qdrant` is an explicit opt-in, and forgetting to opt out is no longer a problem.
2. **Memory concerns are hard to navigate and test in isolation.** A developer working on the `RagRetrievalService` must orient themselves in a 30,000-LOC monolith rather than a focused ~2,000-LOC crate. Running memory-specific tests requires compiling the entire framework.
3. **Circular path risk during future refactors.** As the Garrison and Sanctum domain types live in `paladin-core` and the port traits live in `paladin-ports`, the adapters that implement those traits must live in an adapter crate, not alongside the monolith, to keep dependency direction strictly inward-only.

### Why Now (After Epics 1–3)?

`paladin-memory` depends on:

- `paladin-core` — for domain types `GarrisonEntry`, `GarrisonConfig`, `SanctumEntry`, `Memory`, `MemoryBuilder`, `MemoryType`, and `ConversationRole`. ✅ Extracted in Epic 1.
- `paladin-ports` — for the `GarrisonPort`, `SanctumPort`, `EmbeddingPort`, and `LlmPort` trait definitions. ✅ Extracted in Epic 2.

There is no dependency on `paladin-battalion` or `paladin-llm`, so this epic can proceed immediately after Epic 2 is complete and can run in parallel with Epics 3 and 4.

### Spec-Driven Development (SDD) Note

This PRD is a **specification**, not an implementation plan. It defines *what* must be true when Epic 5 is complete. A separate **task list** (`tasks-paladin-memory-extraction.md`) will be derived from this PRD and will contain the concrete, step-by-step implementation work with checkboxes.

---

## 2. Goals

1. **Create `crates/paladin-memory/`** as an independent Cargo crate with its own `Cargo.toml` and `src/lib.rs`, edition `2024`, matching the workspace root.
2. **Extract `InMemoryGarrison`** from `src/infrastructure/adapters/garrison/in_memory_garrison.rs` into `crates/paladin-memory/src/garrison/in_memory_garrison.rs`. Available unconditionally (no feature gate required).
3. **Extract `SqliteGarrison`** from `src/infrastructure/adapters/garrison/sqlite_garrison.rs` into `crates/paladin-memory/src/garrison/sqlite_garrison.rs`. Gated behind the `sqlite` feature flag; `sqlx` is only compiled when this flag is active.
4. **Extract `TokenCounter` / `TiktokenCounter` / `TokenCounterFactory`** from `src/infrastructure/adapters/garrison/token_counter.rs` into `crates/paladin-memory/src/garrison/token_counter.rs`. Gated behind the existing `content-processing` feature flag; `tiktoken-rs` is only compiled when this flag is active.
5. **Extract `InMemorySanctum`** from `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` into `crates/paladin-memory/src/sanctum/in_memory_adapter.rs`. Available unconditionally.
6. **Extract `QdrantSanctumAdapter`** from `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` into `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`. Gated behind the `qdrant` feature flag; `qdrant-client` is only compiled when this flag is active.
7. **Extract `MemoryExtractionService`** (with `ExtractedMemory` and `MemoryExtractionStrategy`) from `src/application/use_cases/sanctum/memory_extraction_service.rs` into `crates/paladin-memory/src/services/memory_extraction_service.rs`. Available unconditionally; depends only on port traits.
8. **Extract `RagRetrievalService`** (with `RagConfig`, `RetrievalTrigger`, and `retrieve_context_with_timeout`) from `src/application/use_cases/sanctum/rag_retrieval_service.rs` into `crates/paladin-memory/src/services/rag_retrieval_service.rs`. Available unconditionally; depends only on port traits.
9. **Update all import paths** in extracted files. References to `crate::core::platform::container::*` become `paladin_core::platform::container::*`. References to `paladin_ports::output::*` remain unchanged.
10. **Delete the original source files** from the monolith after extraction. Add facade re-exports in `src/lib.rs` (the root `paladin` crate) so all existing `use paladin::...` import paths for memory types continue to compile without modification.
11. **Add `paladin-memory` as a workspace dependency** in the root `Cargo.toml` `[workspace.dependencies]` section and as a direct dependency in the facade crate's `[dependencies]`.
12. **Expose a `paladin_memory::prelude` module** containing the most commonly used types: `InMemoryGarrison`, `InMemorySanctum`, `MemoryExtractionService`, `RagRetrievalService`, and `RagConfig`.
13. **Move unit tests** from `tests/unit/sanctum/` into `crates/paladin-memory/` as inline `#[cfg(test)]` modules co-located with their source files. Workspace-level integration tests (`tests/integration/`) remain in place.
14. **All existing tests pass** with zero regressions after extraction — `cargo test --workspace` is green.

---

## 3. User Stories

### Story 1 — Lightweight Deployment (In-Memory Only)

> As a **developer building a prototype or running tests**, I want to add `paladin-memory` as a dependency without enabling any optional features so that only the in-memory implementations are compiled and my build does not pull in SQLite or Qdrant machinery.

**Acceptance:** `cargo build -p paladin-memory --no-default-features` completes successfully. `cargo tree -p paladin-memory --no-default-features` does not contain `sqlx`, `qdrant-client`, or `tiktoken-rs`.

---

### Story 2 — Production Deployment with SQLite

> As a **developer building a production Paladin agent** that needs durable conversation history, I want to enable `paladin-memory` with the `sqlite` feature so that `SqliteGarrison` is available and conversation history survives process restarts.

**Acceptance:** Adding `paladin-memory = { features = ["sqlite"] }` to a consuming crate compiles `SqliteGarrison`. A downstream test can call `SqliteGarrison::connect("./test.db", config, "paladin-001").await` and successfully store and retrieve `GarrisonEntry` values.

---

### Story 3 — Semantic Memory with Qdrant

> As a **developer building a long-running agent** with semantic memory retrieval, I want to enable `paladin-memory` with the `qdrant` feature so that `QdrantSanctumAdapter` is available for vector similarity search over stored memories.

**Acceptance:** Adding `paladin-memory = { features = ["qdrant"] }` compiles `QdrantSanctumAdapter`. It is not present in the binary or object files when the `qdrant` feature is absent.

---

### Story 4 — RAG Pipeline Author

> As a **developer implementing a RAG pipeline**, I want `MemoryExtractionService` and `RagRetrievalService` to be available unconditionally from `paladin-memory` so that I can compose them with any storage backend without enabling extra feature flags.

**Acceptance:** `use paladin_memory::services::{MemoryExtractionService, RagRetrievalService};` compiles with `--no-default-features`. Both services accept `Arc<dyn SanctumPort>` and `Arc<dyn EmbeddingPort>` at their construction sites, keeping them backend-agnostic.

---

### Story 5 — Existing Consumer (Backward Compatibility)

> As a **developer using the `paladin` facade crate today**, I want my existing `use paladin::InMemoryGarrison;`, `use paladin::InMemorySanctum;`, and related import paths to continue compiling after Epic 5 is merged so that I do not need to change any of my existing code.

**Acceptance:** All `use paladin::*` paths for types listed in the facade re-export mapping (Section 7) compile without modification after the extraction. No breakage to examples, integration tests, or functional tests.

---

### Story 6 — Token Budget Aware Agent

> As a **developer building a token-budget-aware agent**, I want `TiktokenCounter` available when I enable `content-processing` so that I can count tokens before inserting garrison history into a prompt.

**Acceptance:** `use paladin_memory::garrison::token_counter::TiktokenCounter;` compiles when `--features content-processing` is set. It does not compile (and is not linked) when that feature is absent.

---

## 4. Functional Requirements

### FR-1: Crate Scaffold

**FR-1.1** A `crates/paladin-memory/` directory must exist with a valid `Cargo.toml` and `src/lib.rs`.

**FR-1.2** `Cargo.toml` must declare `edition = "2024"` (matching the workspace root).

**FR-1.3** `Cargo.toml` must declare the following feature flags:

```toml
[features]
default = []
sqlite = ["dep:sqlx"]
qdrant = ["dep:qdrant-client"]
content-processing = ["dep:tiktoken-rs"]
```

**FR-1.4** `Cargo.toml` must list the following mandatory (non-optional) dependencies (using workspace versions where available):

```toml
[dependencies]
paladin-core  = { path = "../paladin-core" }
paladin-ports = { path = "../paladin-ports" }
async-trait   = { workspace = true }
serde         = { workspace = true }
serde_json    = { workspace = true }
uuid          = { workspace = true }
chrono        = { workspace = true }
thiserror     = { workspace = true }
tokio         = { workspace = true }
futures       = { workspace = true }
log           = { workspace = true }
```

**FR-1.5** Before scaffolding the crate, `sqlx` and `qdrant-client` must be hoisted from direct dependencies in the root `Cargo.toml` into the `[workspace.dependencies]` section. This is a prerequisite step of Task 5.1. Additionally, run `cargo tree -p paladin --features content-processing | grep tiktoken` to confirm the exact `tiktoken-rs` version before writing the `Cargo.toml`. Once hoisted, the optional dependencies in `paladin-memory/Cargo.toml` must read:

```toml
[dependencies]
sqlx          = { workspace = true, features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json"], optional = true }
qdrant-client = { workspace = true, optional = true }
tiktoken-rs   = { version = "<confirmed-version>", optional = true }  # confirm via cargo tree before writing
```

**FR-1.6** `[lib]` must set `doctest = false` (consistent with other workspace crates).

**FR-1.7** The root workspace `Cargo.toml` must:
1. Hoist `sqlx` and `qdrant-client` into `[workspace.dependencies]` (prerequisite of Task 5.1).
2. Add `paladin-memory = { path = "crates/paladin-memory" }` to `[workspace.dependencies]`.
3. Add `paladin-memory` to the facade crate's `[dependencies]` section with the appropriate feature flags forwarded.

---

### FR-2: Module Structure

**FR-2.1** `src/lib.rs` must declare three top-level public modules:

```rust
pub mod garrison;
pub mod sanctum;
pub mod services;
pub mod prelude;
```

**FR-2.2** `src/garrison/mod.rs` must unconditionally re-export `InMemoryGarrison` and, behind `#[cfg(feature = "sqlite")]`, re-export `SqliteGarrison`. Behind `#[cfg(feature = "content-processing")]`, it must re-export `TokenCounter`, `TiktokenCounter`, and `TokenCounterFactory`.

**FR-2.3** `src/sanctum/mod.rs` must unconditionally re-export `InMemorySanctum` and, behind `#[cfg(feature = "qdrant")]`, re-export `QdrantSanctumAdapter`.

**FR-2.4** `src/services/mod.rs` must unconditionally re-export `MemoryExtractionService`, `ExtractedMemory`, `MemoryExtractionStrategy`, `RagRetrievalService`, `RagConfig`, `RetrievalTrigger`, and `retrieve_context_with_timeout`.

**FR-2.5** `src/prelude.rs` must re-export the following types unconditionally: `InMemoryGarrison`, `InMemorySanctum`, `MemoryExtractionService`, `RagRetrievalService`, and `RagConfig`.

---

### FR-3: Garrison Adapters

**FR-3.1** `src/garrison/in_memory_garrison.rs` must contain the full `InMemoryGarrison` implementation, implementing `GarrisonPort` via `async-trait`.

**FR-3.2** `src/garrison/sqlite_garrison.rs` must exist inside a `#[cfg(feature = "sqlite")]` module boundary. It must implement `GarrisonPort` using `sqlx::SqlitePool`. The `#[doc(hidden)]` attribute on `SqliteGarrison` must be removed; a proper rustdoc comment (`///`) explaining the type's purpose, construction, and usage must be added or restored.

**FR-3.3** `src/garrison/token_counter.rs` must exist inside a `#[cfg(feature = "content-processing")]` module boundary. It must contain the `TokenCounter` trait, `TiktokenCounter` struct, and `TokenCounterFactory` helper. The `#[doc(hidden)]` attribute on `TiktokenCounter` must be removed; proper rustdoc comments must be added to `TiktokenCounter`, `TokenCounterFactory`, and all public methods.

**FR-3.4** All `crate::core::platform::container::garrison::*` imports in the extracted garrison files must be updated to `paladin_core::platform::container::garrison::*`.

**FR-3.5** All `paladin_ports::output::garrison_port::*` imports must remain unchanged.

---

### FR-4: Sanctum Adapters

**FR-4.1** `src/sanctum/in_memory_adapter.rs` must contain the full `InMemorySanctum` implementation, including `InMemorySanctumConfig`, implementing `SanctumPort` via `async-trait`. `InMemorySanctumConfig` must be promoted to a fully public, documented type: any `#[doc(hidden)]` attribute must be removed and a rustdoc comment explaining its fields and defaults must be present. Both `InMemorySanctum` and `InMemorySanctumConfig` must appear in the public API surface of `paladin-memory`.

**FR-4.2** `src/sanctum/qdrant_adapter.rs` must exist inside a `#[cfg(feature = "qdrant")]` module boundary. It must contain `QdrantSanctumAdapter` implementing `SanctumPort`.

**FR-4.3** All `crate::core::platform::container::sanctum::*` imports in extracted sanctum files must be updated to `paladin_core::platform::container::sanctum::*`.

---

### FR-5: Memory Services

**FR-5.1** `src/services/memory_extraction_service.rs` must contain `MemoryExtractionService`, `ExtractedMemory`, and `MemoryExtractionStrategy` verbatim from the monolith (logic unchanged), with only the import paths updated.

**FR-5.2** `src/services/rag_retrieval_service.rs` must contain `RagRetrievalService`, `RagConfig`, `RetrievalTrigger`, and `retrieve_context_with_timeout` verbatim from the monolith (logic unchanged), with only the import paths updated.

**FR-5.3** Both services depend only on port traits (`GarrisonPort`, `SanctumPort`, `EmbeddingPort`, `LlmPort`) and domain types from `paladin-core`. They must not reference any concrete adapter.

**FR-5.4** All `crate::core::platform::container::*` imports in extracted service files must be updated to `paladin_core::platform::container::*`.

---

### FR-6: Deletion of Originals

**FR-6.1** After successful extraction and test passage, the following original files must be **deleted** from the monolith:

| Original path | Replaced by |
|---|---|
| `src/infrastructure/adapters/garrison/in_memory_garrison.rs` | `crates/paladin-memory/src/garrison/in_memory_garrison.rs` |
| `src/infrastructure/adapters/garrison/sqlite_garrison.rs` | `crates/paladin-memory/src/garrison/sqlite_garrison.rs` |
| `src/infrastructure/adapters/garrison/token_counter.rs` | `crates/paladin-memory/src/garrison/token_counter.rs` |
| `src/infrastructure/adapters/garrison/mod.rs` | replaced by facade re-exports |
| `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` | `crates/paladin-memory/src/sanctum/in_memory_adapter.rs` |
| `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` | `crates/paladin-memory/src/sanctum/qdrant_adapter.rs` |
| `src/infrastructure/adapters/sanctum/mod.rs` | replaced by facade re-exports |
| `src/application/use_cases/sanctum/memory_extraction_service.rs` | `crates/paladin-memory/src/services/memory_extraction_service.rs` |
| `src/application/use_cases/sanctum/rag_retrieval_service.rs` | `crates/paladin-memory/src/services/rag_retrieval_service.rs` |
| `src/application/use_cases/sanctum/mod.rs` | replaced by facade re-exports |

**FR-6.2** Any `pub mod garrison;` or `pub mod sanctum;` declarations in `src/infrastructure/adapters/mod.rs` that point to the deleted modules must be removed. Any `pub mod sanctum;` in `src/application/use_cases/mod.rs` that points to the deleted module must be removed.

---

### FR-7: Facade Re-Exports

The following `use paladin::...` import paths must continue to compile after extraction. They must be satisfied by new `pub use paladin_memory::...` re-export statements added to `src/lib.rs`:

| Existing path | Re-exported from |
|---|---|
| `paladin::infrastructure::adapters::garrison::InMemoryGarrison` | `paladin_memory::garrison::InMemoryGarrison` |
| `paladin::infrastructure::adapters::garrison::SqliteGarrison` | `paladin_memory::garrison::SqliteGarrison` |
| `paladin::infrastructure::adapters::garrison::token_counter::TokenCounter` | `paladin_memory::garrison::token_counter::TokenCounter` |
| `paladin::infrastructure::adapters::garrison::token_counter::TiktokenCounter` | `paladin_memory::garrison::token_counter::TiktokenCounter` |
| `paladin::infrastructure::adapters::sanctum::InMemorySanctum` | `paladin_memory::sanctum::InMemorySanctum` |
| `paladin::infrastructure::adapters::sanctum::QdrantSanctumAdapter` | `paladin_memory::sanctum::QdrantSanctumAdapter` |
| `paladin::application::use_cases::sanctum::MemoryExtractionService` | `paladin_memory::services::MemoryExtractionService` |
| `paladin::application::use_cases::sanctum::RagRetrievalService` | `paladin_memory::services::RagRetrievalService` |
| `paladin::application::use_cases::sanctum::RagConfig` | `paladin_memory::services::RagConfig` |
| `paladin::application::use_cases::sanctum::MemoryExtractionStrategy` | `paladin_memory::services::MemoryExtractionStrategy` |
| `paladin::application::use_cases::sanctum::RetrievalTrigger` | `paladin_memory::services::RetrievalTrigger` |

**Note:** If any of the above paths are not currently used in examples, tests, or documented code (discovered during audit), those re-exports may be omitted with a comment explaining why.

---

### FR-8: Tests

**FR-8.1** The following unit test files must be migrated from `tests/unit/sanctum/` into inline `#[cfg(test)]` modules within `crates/paladin-memory/`:

| Current location | New location |
|---|---|
| `tests/unit/sanctum/memory_extraction_service_test.rs` | Inline `#[cfg(test)]` module in `crates/paladin-memory/src/services/memory_extraction_service.rs` |
| `tests/unit/sanctum/rag_retrieval_service_test.rs` | Inline `#[cfg(test)]` module in `crates/paladin-memory/src/services/rag_retrieval_service.rs` |
| `tests/unit/sanctum/qdrant_sanctum_test.rs` | Inline `#[cfg(test)]` module in `crates/paladin-memory/src/sanctum/qdrant_adapter.rs` (gated on `#[cfg(feature = "qdrant")]`) |
| `tests/unit/sanctum_domain_tests.rs` | Stays at workspace level (tests `paladin-core` domain types, not memory adapters) |
| `tests/unit/sanctum_port_tests.rs` | Stays at workspace level (tests port traits in `paladin-ports`, not memory adapters) |

**FR-8.1 (addendum — TokenCounterFactory):** Before closing Task 5.2, search for existing tests of `TokenCounterFactory` (in `tests/unit/` or inline). If none exist, write unit tests covering at minimum: factory construction, successful counter creation for a known model name, and error handling for an unknown model name. Place these tests in the inline `#[cfg(test)]` module of `crates/paladin-memory/src/garrison/token_counter.rs`.

**FR-8.2** The following integration tests must remain at the workspace level in `tests/integration/`:

- `in_memory_sanctum_tests.rs`
- `sqlite_garrison_integration_test.rs`
- `qdrant_sanctum_tests.rs`
- `rag_integration_tests.rs`
- `paladin_garrison_integration_test.rs`

These tests exercise adapter behavior end-to-end through the port traits and may depend on external services (SQLite files, Qdrant daemon); they belong at the workspace level.

**FR-8.3** All migrated unit tests must pass when run with `cargo test -p paladin-memory` (with appropriate feature flags enabled per test).

**FR-8.4** All integration tests must pass when run with `cargo test --workspace` (with appropriate feature flags).

---

### FR-9: Build Verification Gates

The following commands must all succeed without errors or warnings after extraction:

| Command | Must pass |
|---|---|
| `cargo build -p paladin-memory --no-default-features` | Only in-memory adapters compiled |
| `cargo build -p paladin-memory --features sqlite` | Adds `SqliteGarrison` |
| `cargo build -p paladin-memory --features qdrant` | Adds `QdrantSanctumAdapter` |
| `cargo build -p paladin-memory --features content-processing` | Adds `TiktokenCounter` |
| `cargo build -p paladin-memory --all-features` | All components compiled |
| `cargo build --workspace` | Full workspace builds |
| `cargo test --workspace` | All tests pass |
| `cargo clippy -p paladin-memory -- -D warnings` | No linter warnings |
| `cargo doc -p paladin-memory --no-deps` | Clean documentation |

---

## 5. Non-Goals (Out of Scope)

1. **No new storage backends.** This epic extracts existing backends only. Adding PostgreSQL garrison, Redis-backed garrison, or Pinecone sanctum adapters is out of scope.
2. **No API changes to existing adapters.** The public interface of every adapter (`InMemoryGarrison`, `SqliteGarrison`, etc.) must remain identical to its current implementation. Logic refactors or performance improvements are out of scope.
3. **No changes to `GarrisonPort` or `SanctumPort` trait definitions.** These live in `paladin-ports` and are not modified by this epic.
4. **No changes to domain types.** `GarrisonEntry`, `SanctumEntry`, `Memory`, `MemoryBuilder`, etc. live in `paladin-core` and are not modified by this epic.
5. **No per-adapter sub-crates.** A single `paladin-memory` crate with feature flags is the target structure. Splitting into `paladin-garrison` and `paladin-sanctum` is explicitly deferred.
6. **No SQLite migrations changes.** The existing SQLite schema and migration logic in `SqliteGarrison` is extracted as-is without modification.
7. **No changes to the `content-processing` feature composition** in the root `Cargo.toml`. That flag remains an aggregate flag there; `paladin-memory` introduces its own `content-processing` flag that gates only `tiktoken-rs`.
8. **No `paladin-memory` CLI tooling.** Database inspection, migration management, or admin commands are out of scope.
9. **No documentation overhaul.** Existing rustdoc comments are preserved as-is during extraction. Adding net-new documentation beyond what is required to fix broken intra-doc links is out of scope.

---

## 6. Design Considerations

### Crate Dependency Diagram

```
paladin-core  ←──────────────────────────┐
paladin-ports ←──────────────────────────┤
                                         │
paladin-memory (new) ─── depends on ─────┘

paladin (facade) ─── depends on ──► paladin-memory
```

`paladin-memory` must never import from `paladin-battalion`, `paladin-llm`, or the root `paladin` crate.

### Directory Layout

```
crates/paladin-memory/
├── Cargo.toml
└── src/
    ├── lib.rs               # pub mod garrison; pub mod sanctum; pub mod services; pub mod prelude;
    ├── prelude.rs           # Curated re-exports of common types
    ├── garrison/
    │   ├── mod.rs           # Re-exports InMemoryGarrison, conditional SqliteGarrison, conditional TokenCounter
    │   ├── in_memory_garrison.rs     # Always compiled
    │   ├── sqlite_garrison.rs        # #[cfg(feature = "sqlite")]
    │   └── token_counter.rs          # #[cfg(feature = "content-processing")]
    └── sanctum/
    │   ├── mod.rs           # Re-exports InMemorySanctum, conditional QdrantSanctumAdapter
    │   ├── in_memory_adapter.rs      # Always compiled
    │   └── qdrant_adapter.rs         # #[cfg(feature = "qdrant")]
    └── services/
        ├── mod.rs           # Re-exports MemoryExtractionService, RagRetrievalService, etc.
        ├── memory_extraction_service.rs
        └── rag_retrieval_service.rs
```

### Import Path Update Pattern

Every occurrence of the following pattern in extracted files must be updated:

**Before:**
```rust
use crate::core::platform::container::garrison::GarrisonEntry;
```

**After:**
```rust
use paladin_core::platform::container::garrison::GarrisonEntry;
```

The same applies to `sanctum`, `prompt`, and any other `crate::core::*` paths. References to `paladin_ports::output::*` remain unchanged because `paladin-ports` is a peer dependency, not a workspace-relative path.

### Reference Implementation

`crates/paladin-llm` (extracted in Epic 4) provides the canonical template for crate structure, `Cargo.toml` layout, `lib.rs` gating conventions, and rustdoc style. Follow it for consistency.

---

## 7. Technical Considerations

### Dependency on `paladin-llm`

`MemoryExtractionService` currently imports `LlmPort` and `LlmRequest` from `paladin-ports`. It does **not** import from `paladin-llm` directly. This is correct hexagonal architecture: the service depends on the port trait, not the adapter. No dependency on `paladin-llm` should be introduced.

### `sqlx` and `qdrant-client` Version Management

Both `sqlx` and `qdrant-client` must be hoisted into the workspace root's `[workspace.dependencies]` as part of Task 5.1, before any `paladin-memory` source is written. This ensures version consistency across all crates without duplication.

For `sqlx`, the workspace entry must declare only `runtime-tokio-rustls` and storage-neutral features. The `mysql` feature must **not** appear in the workspace entry (it is only needed by the root crate's repository layer and should remain a root-crate-only override):

```toml
# In root Cargo.toml [workspace.dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json"] }
qdrant-client = { version = "1.14" }
```

The root `paladin` crate's own `[dependencies]` entry for `sqlx` then adds `mysql` as a local override. `paladin-memory` references both as `{ workspace = true, optional = true }` without re-specifying features.

### `content-processing` Feature Semantics

In `paladin-memory`, the `content-processing` feature gates **only** `tiktoken-rs` and the `token_counter.rs` module. It does not gate `pdf-extract`, `scraper`, or `rss` — those remain in the root `paladin` crate. This is intentional: `paladin-memory` owns only the token counting concern, not the full content processing pipeline.

### SQLite Migration Files

`SqliteGarrison` runs inline SQL migrations at startup (schema creation). These SQL strings live inline in `sqlite_garrison.rs` and travel with the file during extraction. No separate migration files are required.

### `#[deny(unsafe_code)]`

Add `#![deny(unsafe_code)]` to `crates/paladin-memory/src/lib.rs`, consistent with other workspace crates.

### Thread Safety

All adapter types must remain `Send + Sync`. The existing implementations satisfy this (they use `RwLock` or `Arc`-wrapped state). This requirement must be verified during extraction by adding a compile-time assertion where appropriate:

```rust
fn _assert_send_sync<T: Send + Sync>() {}
// In tests:
_assert_send_sync::<InMemoryGarrison>();
_assert_send_sync::<InMemorySanctum>();
```

---

## 8. Success Metrics

1. `cargo build -p paladin-memory --no-default-features` succeeds and `cargo tree` shows no `sqlx`, `qdrant-client`, or `tiktoken-rs` nodes.
2. `cargo test -p paladin-memory` (with all features enabled) shows all migrated unit tests passing.
3. `cargo test --workspace` shows all existing integration and functional tests passing with zero regressions.
4. `cargo clippy -p paladin-memory -- -D warnings` produces zero warnings.
5. `cargo doc -p paladin-memory --no-deps` produces zero broken intra-doc link warnings.
6. `cargo build --workspace` succeeds, confirming the facade re-exports compile correctly.
7. All garrison and sanctum examples in `examples/` compile and pass a smoke-run without changes to their import statements.

---

## 9. Open Questions

All questions identified during initial drafting have been resolved. No open questions remain.

| # | Question | Resolution | Captured in |
|---|----------|------------|-------------|
| 1 | Hoist `sqlx`/`qdrant-client` to `[workspace.dependencies]` or keep crate-local? | **Hoist** — required as part of Task 5.1 | FR-1.5, FR-1.7, Section 7 |
| 2 | Confirm exact `tiktoken-rs` version before writing `Cargo.toml`? | **Yes** — run `cargo tree \| grep tiktoken` before Task 5.1 scaffolding | FR-1.5 |
| 3 | Promote `InMemorySanctumConfig` to a public, documented type? | **Yes** — remove `#[doc(hidden)]`, add rustdoc | FR-4.1 |
| 4 | Remove `#[doc(hidden)]` from `SqliteGarrison` and `TiktokenCounter`? | **Yes** — both become first-class public API; add proper rustdoc | FR-3.2, FR-3.3 |
| 5 | Write `TokenCounterFactory` tests if none exist? | **Yes** — write and inline in `token_counter.rs` as part of Task 5.2 | FR-8.1 (addendum) |
