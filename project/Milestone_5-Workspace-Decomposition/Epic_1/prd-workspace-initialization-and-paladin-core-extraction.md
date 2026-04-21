# PRD: Workspace Initialization and `paladin-core` Extraction

**Epic:** Epic 1 — Workspace Initialization and `paladin-core` Extraction
**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split
**Project:** Paladin Framework Refactoring Initiative
**Status:** Draft
**Author:** TBD
**Reviewers:** TBD
**Document Version:** 1.0
**Created:** 2026-04-21
**Last Updated:** 2026-04-21
**Target Audience:** Junior Developer

---

## 1. Introduction / Overview

### What Is This Feature?

This epic initializes a **Cargo workspace** at the repository root and extracts the first independent crate — `paladin-core` — from the existing Paladin monolithic crate. `paladin-core` will contain all pure domain types: the base primitives from `src/core/base/` and the domain entities from `src/core/platform/container/`.

### What Problem Does It Solve?

Today, the entire Paladin framework is a single Cargo crate (`paladin`). Every change — no matter how small — triggers a full recompile of all ~29,000+ lines of code. There are no enforced dependency boundaries between the domain layer, the application layer, and the infrastructure layer beyond code conventions.

This epic solves the foundational part of that problem by:

1. Converting the repository into a **Cargo workspace** so that multiple crates can coexist and be built independently.
2. Extracting the domain layer into `paladin-core` — a crate with zero knowledge of infrastructure, LLM providers, databases, or HTTP clients — enforcing the hexagonal architecture at the compiler level rather than by convention alone.

### Why `paladin-core` First?

`paladin-core` has the fewest external dependencies of any planned crate (only `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, and `serde_json`). Every other planned crate (`paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`) will depend on `paladin-core`. Extracting it first creates the stable foundation that all subsequent extractions build upon.

### Spec-Driven Development (SDD) Note

This PRD is a **specification**, not an implementation plan. It defines *what* must be true when the epic is complete. A separate **task list** will be derived from this PRD. That task list will include a **decision task** (with a structured options-analysis artifact and implementer interview) for any architectural decision that cannot be resolved at specification time — most notably the upward dependency from `battalion/mod.rs` into the application layer. The task list will be updated with concrete implementation tasks once the architectural decision is made.

---

## 2. Goals

1. **Create the Cargo workspace root** — the repository root `Cargo.toml` becomes a workspace manifest; the existing `paladin` crate becomes one member of that workspace.
2. **Scaffold `crates/paladin-core/`** — a new independent crate with its own `Cargo.toml` and `src/lib.rs`.
3. **Extract `src/core/base/`** into `paladin-core` — all base primitives (`Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event`) live inside `paladin-core`.
4. **Extract `src/core/platform/container/`** into `paladin-core` — all domain entities (`Paladin`, Battalion types, `Garrison`, `Arsenal`, `Citadel`, `Herald`, `Sanctum`, and all supporting types) live inside `paladin-core`.
5. **Resolve the upward dependency** from `battalion/mod.rs` into the application layer — through a structured decision process documented in the task list.
6. **Wire the root `paladin` crate** to re-export `paladin-core` types, preserving existing import paths for all callers.
7. **Validate clean dependency layering** — confirm `paladin-core` cannot accidentally import from the application or infrastructure layers.
8. **All existing tests continue to pass** — no regressions are acceptable.

---

## 3. User Stories

### Story 1 — Framework Contributor

> As a **framework contributor**, I want `paladin-core` to compile in isolation so that I can work on domain types without waiting for the entire framework (LLM adapters, Redis, MinIO, etc.) to compile.

**Acceptance:** `cargo build -p paladin-core` succeeds in under 30 seconds on a standard developer machine.

---

### Story 2 — Downstream Library Consumer

> As a **library consumer** who depends on `paladin-core` types in my own crate, I want those types to be available from a stable, minimal crate so that I don't have to pull in HTTP clients, database drivers, or LLM SDK dependencies just to use `Paladin`, `Garrison`, or `Battalion` domain types.

**Acceptance:** A downstream crate can add `paladin-core` to its `[dependencies]` and use `Paladin`, `GarrisonEntry`, `Citadel`, etc. without any transitive dependency on `reqwest`, `redis`, `sqlx`, or any LLM provider SDK.

---

### Story 3 — Maintainer Enforcing Architecture

> As a **maintainer**, I want the Rust compiler to enforce that domain types never import application or infrastructure modules so that architectural violations are caught at compile time rather than in code review.

**Acceptance:** `paladin-core`'s `Cargo.toml` lists only `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, and `serde_json` as dependencies. Any attempt to import from `application::` or `infrastructure::` would fail to compile because those crates are not in `paladin-core`'s dependency graph.

---

### Story 4 — Junior Developer Working on the Task List

> As the **implementer working from the task list**, I want a structured decision process for resolving the `battalion/mod.rs` upward dependency so that I understand all the options, their trade-offs, and can arrive at the correct architectural choice with guidance before writing any implementation code.

**Acceptance:** The task list derived from this PRD includes a dedicated decision task that produces an options-analysis artifact and walks the implementer through a guided selection interview before any code for that sub-problem is written.

---

## 4. Functional Requirements

> **Note for implementer:** Each requirement below is numbered. All requirements must be satisfied for the epic to be considered complete.

### 4.1 Cargo Workspace Root

- **FR-1:** The file `Cargo.toml` at the repository root must contain a `[workspace]` section with `members = ["crates/*"]` (plus any other existing crate paths if needed).
- **FR-2:** The workspace root `Cargo.toml` must contain a `[workspace.dependencies]` section that declares shared dependency versions for at minimum: `serde` (with features `["derive"]`), `uuid` (with features `["v4", "serde"]`), `chrono` (with feature `["serde"]`), `thiserror`, `tokio` (with feature `["full"]`), `async-trait`, `serde_json`, `reqwest`, and `log`.
- **FR-3:** The existing `paladin` crate source (`src/`) must remain buildable. `cargo build` from the workspace root must succeed after the workspace `Cargo.toml` is created, before any source files are moved.

### 4.2 `paladin-core` Crate Scaffold

- **FR-4:** The directory `crates/paladin-core/` must exist with a valid `Cargo.toml` and `src/lib.rs`.
- **FR-5:** `crates/paladin-core/Cargo.toml` must set `name = "paladin-core"`, `edition = "2021"`, and reference workspace dependencies (using `dep = { workspace = true }` syntax) rather than pinning versions directly.
- **FR-6:** `paladin-core`'s `[dependencies]` section must contain only: `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, and `serde_json`. No other dependencies are permitted.
- **FR-7:** `cargo build -p paladin-core` must succeed in isolation (i.e., without building any other workspace member).

### 4.3 Extraction of `src/core/base/`

- **FR-8:** All files from `src/core/base/` must be relocated to `crates/paladin-core/src/base/`. The module tree structure (`mod.rs` declarations, sub-modules) must be preserved.
- **FR-9:** Types included in this extraction are (at minimum): `Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event`, and any internal helper types they reference.
- **FR-10:** All `use` statements within the moved files must be updated to use crate-local paths (e.g., `crate::base::...`) rather than the old monolith paths (e.g., `crate::core::base::...`).
- **FR-11:** No moved file may contain a `use` statement that references `application::`, `infrastructure::`, or any module path outside of the `paladin-core` crate itself.
- **FR-12:** All unit tests that previously lived in `src/core/base/` (inside `#[cfg(test)]` modules) must compile and pass when run via `cargo test -p paladin-core`.

### 4.4 Extraction of `src/core/platform/container/`

- **FR-13:** All files from `src/core/platform/container/` must be relocated to `crates/paladin-core/src/platform/container/`. The module tree structure must be preserved.
- **FR-14:** Types included in this extraction are (at minimum): `Paladin`, `PaladinData`, `PaladinConfig`, `PaladinStatus`, all Battalion domain types (`Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`, `Conclave`, `Council`, `Grove`, `Maneuver` with its lexer/AST/parser), `Garrison`, `GarrisonEntry`, `GarrisonConfig`, `Arsenal`, `Armament`, `Citadel`, `Herald`, `Sanctum`, `SanctumEntry`, `Memory`, `MemoryBuilder`, and all supporting types.
- **FR-15:** All `use` statements within the moved files must be updated to use crate-local paths.
- **FR-16:** No moved file may contain a `use` statement referencing `application::` or `infrastructure::` modules after the upward dependency resolution task is complete (see FR-17).
- **FR-17:** The upward dependency from `battalion/mod.rs` into `application::ports::output::paladin_port` and `application::ports::output::paladin_registry` must be resolved. The resolution approach is **not specified here** — it is the subject of a dedicated architectural decision task in the task list. The task list will include: (a) an options-analysis document task, (b) an implementer interview/decision task, and (c) implementation sub-tasks generated from the chosen approach. The only hard constraint is that after resolution, `battalion/mod.rs` must not import from `application::`.
- **FR-18:** All unit tests that previously lived in `src/core/platform/container/` must compile and pass when run via `cargo test -p paladin-core`.

### 4.5 Root `paladin` Crate Re-exports

- **FR-19:** The root `paladin` crate's `Cargo.toml` must list `paladin-core` as a path dependency: `paladin-core = { path = "crates/paladin-core" }`.
- **FR-20:** `src/lib.rs` (the root crate) must re-export `paladin-core` types under the existing module paths so that any code using `paladin::core::base::Node`, `paladin::core::platform::container::Paladin`, etc. continues to compile without modification.
  - Example re-export pattern: `pub use paladin_core::base;` nested under `pub mod core { pub mod base { ... } }` or a flat `pub use paladin_core as core;` — the exact mechanism is an implementation detail, but the result must be zero breaking changes to existing import paths.
- **FR-21:** After wiring re-exports, `src/core/` source files that have been fully relocated to `paladin-core` must be removed from `src/core/`. The `src/core/` directory becomes either empty or a thin shim of re-export declarations only.
- **FR-22:** `cargo build --workspace` must succeed after the re-export wiring.
- **FR-23:** `cargo test --workspace` must pass all existing tests (the test count must not decrease from the pre-epic baseline).

### 4.6 Dependency Validation

- **FR-24:** After extraction is complete, a dependency graph analysis must be run (using `cargo tree -p paladin-core` or equivalent) to confirm that `paladin-core` has no transitive dependencies on `application` or `infrastructure` crates.
- **FR-25:** The output of `cargo tree -p paladin-core` must not show any crate whose name or path suggests LLM provider SDKs (`openai`, `anthropic`, `deepseek`), database drivers (`sqlx`, `redis`, `mysql`), HTTP frameworks (`axum`, `actix`), or object storage clients (`minio`, `s3`).
- **FR-26:** `cargo doc -p paladin-core --no-deps` must produce documentation with zero broken intra-doc links.

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** part of this epic:

- **Extracting `paladin-ports`** — that is Epic 2. `paladin-core` does not include port trait definitions.
- **Extracting `paladin-battalion`** — that is Epic 3. Battalion *services* (the use-case layer implementations) are not moved here; only the battalion *domain types* are extracted.
- **Extracting `paladin-llm`, `paladin-memory`, or `paladin-cli`** — those are Epics 4, 5, and the CLI work from Milestone 1.
- **Splitting `application_settings.rs`** — that is Tier 3 work.
- **Relocating the Maneuver DSL parser** — that is Tier 3 work.
- **Relocating `CircuitBreaker`** — that is Tier 3 work.
- **Any changes to existing public behavior** — this epic is a structural refactor only. No logic changes, no API behavior changes.
- **Performance optimization** — incremental build speed improvement is a *side effect* of the extraction, not a goal to be engineered directly here.
- **Introducing new test cases** — existing tests must pass; new tests are not required as part of this epic (though they are not prohibited).

---

## 6. Design Considerations

### Directory Layout After Completion

```
paladin/
├── Cargo.toml                        # Workspace root (NEW)
├── crates/
│   └── paladin-core/                 # NEW independent crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── base/                 # Moved from src/core/base/
│           │   ├── mod.rs
│           │   ├── node.rs
│           │   ├── collection.rs
│           │   ├── field.rs
│           │   ├── message.rs
│           │   └── ...
│           └── platform/
│               └── container/        # Moved from src/core/platform/container/
│                   ├── mod.rs
│                   ├── paladin.rs
│                   ├── paladin_config.rs
│                   ├── garrison.rs
│                   ├── arsenal.rs
│                   ├── citadel.rs
│                   ├── herald.rs
│                   ├── sanctum.rs
│                   └── battalion/
│                       ├── mod.rs    # No longer imports from application::
│                       ├── formation.rs
│                       ├── phalanx.rs
│                       ├── campaign.rs
│                       ├── chain_of_command.rs
│                       ├── conclave.rs
│                       ├── council.rs
│                       ├── grove.rs
│                       └── maneuver/
│
├── src/                              # Root paladin facade crate (MODIFIED)
│   ├── lib.rs                        # Re-exports paladin-core types
│   └── core/                         # Thin re-export shim or removed
│
└── ...                               # All other src/ paths unchanged
```

### Module Re-export Strategy

The root `paladin` crate must preserve existing import paths. The recommended approach is to use nested `pub mod` declarations in `src/lib.rs` that re-export from `paladin_core`:

```rust
// In src/lib.rs (illustrative — exact implementation is up to the implementer)
pub mod core {
    pub use paladin_core::base;
    pub mod platform {
        pub use paladin_core::platform;
    }
}
```

The implementer is free to choose the exact re-export mechanism as long as FR-20 is satisfied.

### Workspace `[workspace.dependencies]` Pattern

To keep dependency versions consistent across all crates, workspace-level dependencies are declared once in the root `Cargo.toml` and referenced by member crates:

```toml
# Root Cargo.toml [workspace.dependencies]
serde = { version = "1", features = ["derive"] }
uuid = { version = "1", features = ["v4", "serde"] }

# crates/paladin-core/Cargo.toml [dependencies]
serde = { workspace = true }
uuid = { workspace = true }
```

---

## 7. Technical Considerations

### Known Upward Dependency (Critical Blocker)

Before `paladin-core` can be extracted, a known violation of the hexagonal dependency rule must be resolved:

- **File:** `src/core/platform/container/battalion/mod.rs`
- **Problem:** This file currently imports types from `application::ports::output::paladin_port` and `application::ports::output::paladin_registry`. Since `paladin-core` will not depend on the application layer, these imports cannot exist in the extracted crate.
- **Resolution process:** This will be handled as a **dedicated decision task** in the task list. That task will:
  1. Produce a written options-analysis artifact documenting all viable resolution strategies and their trade-offs.
  2. Walk the implementer through a structured decision interview to select the best approach.
  3. Generate child implementation tasks based on the selected approach.
- **Constraint:** Whatever approach is chosen, the only non-negotiable outcome is that `battalion/mod.rs` (as it exists in `paladin-core`) must not import from `application::` or any module outside of `paladin-core` itself.

### `async-trait` Dependency

Some domain types in `src/core/platform/container/` may use `#[async_trait]`. The `async-trait` crate is therefore a permitted dependency of `paladin-core`.

### `serde_json` Dependency

Some domain types use `serde_json::Value` directly. The `serde_json` crate is therefore a permitted dependency of `paladin-core`.

### Test Infrastructure

Unit tests in Rust live inside the source files they test (inside `#[cfg(test)]` modules). When source files are moved to `paladin-core`, their embedded tests move with them automatically. No test files need to be manually relocated — but all moved tests must be verified to compile and pass via `cargo test -p paladin-core`.

### Cargo Edition

All crates in this workspace must use `edition = "2021"`. Do not use an older edition.

### Visibility Rules

When moving code from a monolith to an independent crate, `pub(crate)` visibility becomes `pub(crate)` scoped to `paladin-core` only. Any type that was `pub(crate)` in the monolith and is referenced by the root `paladin` crate must be changed to `pub`. Review all `pub(crate)` declarations during extraction and determine whether they need to become `pub`.

### Breaking Changes Policy

Breaking changes to `paladin-core`'s public API surface are **permitted** (per answer 2C). The root `paladin` facade crate will absorb any breakage through re-exports. Downstream consumers (examples, tests, integration tests) will be updated as needed.

---

## 8. Success Metrics

The following criteria must all be true before Epic 1 is marked complete:

| # | Metric | How to Verify |
|---|--------|---------------|
| SM-1 | `cargo build -p paladin-core` succeeds | Run command; exit code 0 |
| SM-2 | `cargo build --workspace` succeeds | Run command; exit code 0 |
| SM-3 | `cargo test --workspace` passes all tests | Run command; count ≥ pre-epic baseline |
| SM-4 | `paladin-core` has no application or infrastructure dependencies | `cargo tree -p paladin-core` shows only permitted deps |
| SM-5 | `battalion/mod.rs` in `paladin-core` contains zero imports from `application::` | `grep -r "application::" crates/paladin-core/` returns no results |
| SM-6 | `cargo doc -p paladin-core --no-deps` produces zero errors or broken links | Run command; exit code 0, no warnings |
| SM-7 | Root `paladin` crate re-exports preserve all existing import paths | Existing examples and integration tests compile without path changes |
| SM-8 | `src/core/` in the root crate is empty or contains only re-export shims | Directory inspection |
| SM-9 | All CI pipeline checks pass on the feature branch | CI green |
| SM-10 | The architectural decision task for the upward dependency is completed and its decision artifact is committed | File exists in `project/Milestone_5-Workspace-Decomposition/Epic_1/` |

---

## 9. Open Questions

| # | Question | Owner | Status |
|---|----------|-------|--------|
| OQ-1 | Which of the three resolution strategies for the `battalion/mod.rs` upward dependency is the correct architectural choice? (Options: move `PaladinResult` into core; define a separate `BattalionOutcome`; defer to `paladin-ports` in Epic 2.) | Implementer (via decision task in task list) | Open — resolved in task list decision task |
| OQ-2 | Are there any other upward dependencies from `src/core/` into `application::` or `infrastructure::` beyond the known `battalion/mod.rs` coupling? A full audit must be performed before extraction begins. | Implementer | Open — to be answered during task list execution |
| OQ-3 | Do any `pub(crate)` types in `src/core/` need to become `pub` once they live in an independent crate? | Implementer | Open — to be answered during extraction |
| OQ-4 | Are the `cargo test -p paladin-core` unit tests sufficient, or do some existing unit tests in the root crate test core types through integration with application-layer code (making them impossible to move to `paladin-core` without modification)? | Implementer | Open — to be answered during extraction |
| OQ-5 | Does the workspace root `Cargo.toml` need a `[profile.*]` section to preserve existing build profile settings, or are defaults acceptable? | Implementer | Open |

---

## Appendix A: Reference Files

The following existing files are the primary subjects of this epic. Review them before beginning implementation.

| File | Purpose |
|------|---------|
| `Cargo.toml` (root) | Will be converted to a workspace manifest |
| `src/core/base/mod.rs` | Base module tree entry point — extracted to `paladin-core` |
| `src/core/platform/container/mod.rs` | Domain entity module tree entry point — extracted to `paladin-core` |
| `src/core/platform/container/battalion/mod.rs` | Contains the known upward dependency into `application::` |
| `src/lib.rs` | Will be updated to re-export `paladin-core` types |
| `STABLE_API.md` | Documents the hardened public API surface from Milestone 1 |
| `project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md` | Full Milestone 5 design document |

---

## Appendix B: Permitted `paladin-core` Dependencies

This is the complete and exhaustive list of external crates that `paladin-core` is allowed to depend on:

| Crate | Reason |
|-------|--------|
| `serde` | Serialization/deserialization for domain types |
| `serde_json` | `Value` type used in domain entities |
| `uuid` | Unique identifiers for domain entities |
| `chrono` | Timestamps in domain entities |
| `thiserror` | Custom error type derivation |
| `async-trait` | `#[async_trait]` macro for async trait methods |

**Any other dependency requires explicit approval and a documented justification.**

---

*End of PRD — Epic 1: Workspace Initialization and `paladin-core` Extraction*
