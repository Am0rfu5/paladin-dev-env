# Options Analysis: Resolving `src/core/` → `application::` Upward Dependencies

**Epic:** Milestone 5 Epic 1 — Workspace Initialization and `paladin-core` Extraction  
**Decision Scope:** Task 3.0 — Resolve upward dependency so `src/core/platform/container/` can be extracted into `paladin-core` with zero `application::` imports  
**Author:** Automated analysis (2026-05-13)  
**Status:** Awaiting decision (Task 3.3)

---

## 1. Problem Statement

`paladin-core` must have zero `use` statements referencing `application::` (FR-11, FR-16). However, four files in `src/core/platform/container/` currently import types from `application::`:

| File | Imports from `application::` | Usage |
|------|-------------------------------|-------|
| `herald.rs` | `TokenUsage` (llm_port), `PaladinResult` (paladin_port), `StopReason` (paladin_port), `PaladinError` (error) | `pub use` re-exports only + test |
| `battalion/mod.rs` | `PaladinResult` (paladin_port), `StopReason` (paladin_port), `RegistryError` (paladin_registry) | Actual type usage in structs/methods |
| `battalion/conclave.rs` | `PaladinResult` (paladin_port) | Actual type usage |
| `arsenal/handoff_tool.rs` | `HandoffError` (errors) | Actual type usage |

**Note:** `src/core/platform/manager/` also imports from `application::` but is NOT in the extraction scope for this Epic. No changes required there.

---

## 2. Type Inventory

The following types need resolution. Their definitions and transitive dependencies are listed below.

| Type | Currently Defined In | External Deps | Transitive Core Deps |
|------|---------------------|---------------|----------------------|
| `PaladinResult` | `application::ports::output::paladin_port` | `serde` | `TaskPlan`, `HandoffRecord` (both already in `src/core/platform/container/`) |
| `StopReason` | `application::ports::output::paladin_port` | `serde` | none |
| `TokenUsage` | `application::ports::output::llm_port` | `serde` | none |
| `RegistryError` | `application::ports::output::paladin_registry` | `thiserror` | none |
| `HandoffError` | `application::errors::handoff_error` | `thiserror` | none |
| `PaladinError` | `application::use_cases::paladin::error` | `thiserror` | `ArsenalError` (core), `GarrisonError` (application) |
| `GarrisonError` | `application::ports::output::garrison_port` | `thiserror` | none |

**Key observation:** `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, and `HandoffError` are **pure value types** whose only real dependencies are `serde` and `thiserror` — exactly the deps allowed in `paladin-core`. `TaskPlan` and `HandoffRecord` (referenced by `PaladinResult`) are already in `src/core/platform/container/` and will be in `paladin-core`.

`PaladinError` is the outlier: it carries `#[from] GarrisonError` where `GarrisonError` lives in `application::ports::output::garrison_port`. Moving it to `paladin-core` would require either also moving `GarrisonError` (expanding core scope) or removing the `#[from]` `GarrisonError` variant.

---

## 3. Option A — Move Pure Value Types to `paladin-core` (Recommended)

### Description

Move the five pure value types (`PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) into `paladin-core`. Handle `PaladinError` separately by removing the `pub use PaladinError` from `herald.rs` (it is a convenience re-export with no actual usage in core).

The application ports that currently *define* these types become thin re-exports from `paladin-core`.

### Files Touched

**New files in `paladin-core`:**
- `crates/paladin-core/src/platform/container/execution_result.rs` — `PaladinResult`, `StopReason`
- `crates/paladin-core/src/platform/container/token_usage.rs` — `TokenUsage`
- `crates/paladin-core/src/platform/container/registry_error.rs` — `RegistryError`
- `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs` — `HandoffError` (moved from `src/application/errors/handoff_error.rs`)

**Modified files (application layer becomes re-exports):**
- `src/application/ports/output/paladin_port.rs` — adds `pub use paladin_core::...::PaladinResult; pub use paladin_core::...::StopReason;`
- `src/application/ports/output/llm_port.rs` — adds `pub use paladin_core::...::TokenUsage;`
- `src/application/ports/output/paladin_registry.rs` — adds `pub use paladin_core::...::RegistryError;`
- `src/application/errors/handoff_error.rs` — adds `pub use paladin_core::...::HandoffError;`
- `src/core/platform/container/herald.rs` — removes `pub use PaladinError` (not a core concern); remaining `pub use` statements updated to local crate paths

### Pros

- **Architecturally correct**: Pure value types that are fundamental to the domain belong in `paladin-core`, not in the application ports layer where they currently live by historical accident.
- **Minimal disruption**: The application layer re-exports preserve all existing public paths (`paladin::application::ports::output::paladin_port::PaladinResult` still resolves correctly).
- **Zero breaking changes**: Consumers importing from application layer paths see no change.
- **Unblocks Epic 2**: When `paladin-ports` is extracted in Epic 2, those ports can depend on `paladin-core` to get these types natively.
- **Smallest delta**: Only adding well-scoped new files to `paladin-core`; application files gain a two-line re-export.

### Cons

- Requires coordinating additions to `paladin-core` with the application re-export wiring in a single PR to avoid a transient broken state.
- `PaladinError` is excluded from the move (see `PaladinError` note below), which means `herald.rs` loses its convenience `pub use PaladinError` re-export. Callers who previously imported `PaladinError` via `herald.rs` would need to import it from `application::use_cases::paladin::error` directly — a minor but concrete downstream change.

### PaladinError note

`PaladinError` is NOT moved under Option A. `herald.rs` currently does `pub use crate::application::use_cases::paladin::error::PaladinError` as a convenience re-export. Since `PaladinError` depends on `GarrisonError` from the application layer, moving it to `paladin-core` would violate FR-6 (paladin-core must not depend on application types). The resolution is to simply **remove** that `pub use` line from `herald.rs`. It is a convenience export that does not belong in the domain layer; callers should import `PaladinError` from its canonical application location.

### Test Migration Impact

- Unit tests in `herald.rs` that use `StopReason::Completed` will need their import updated from `application::ports::output::paladin_port::StopReason` to the new `paladin_core` path (or via the re-export, which keeps working).
- No test logic changes; only import path updates.

---

## 4. Option B — Define New Outcome Types in `paladin-core` (New Types)

### Description

Instead of moving the existing types, define new domain-native outcome types in `paladin-core`:
- `ExecutionOutcome` (replacing `PaladinResult` in the domain)
- `ExecutionStopReason` (replacing `StopReason`)
- `LlmTokenUsage` (replacing `TokenUsage`)

The application layer keeps its existing types. Conversion functions (`From<ExecutionOutcome> for PaladinResult`) bridge the layers at the application boundary.

### Files Touched

**New files in `paladin-core`:**
- New outcome types in `crates/paladin-core/src/platform/container/`

**Modified files:**
- `src/core/platform/container/battalion/mod.rs` — use `ExecutionOutcome` instead of `PaladinResult`
- `src/core/platform/container/herald.rs` — use `ExecutionOutcome`
- `src/application/use_cases/paladin/` — add conversion from `ExecutionOutcome` → `PaladinResult`
- All integration tests that construct `PaladinResult` may need updates

### Pros

- Keeps a clean semantic split: core has its own vocabulary; application has its own vocabulary.
- Type conversions at layer boundaries are a recognized DDD pattern.

### Cons

- **Large refactor scope**: Every call site that constructs or pattern-matches `PaladinResult` in the application layer must be updated.
- **Duplicate types**: Maintains two parallel type hierarchies (`PaladinResult` and `ExecutionOutcome`) that represent the same concept, requiring ongoing maintenance.
- **Breaking change risk**: Integration tests, examples, and downstream consumers may be affected.
- **Violates the refactor constraint**: The PRD explicitly states this Epic is a structural refactor only — "no logic changes, no API behavior changes" (§5 Non-Goals).
- High implementation effort with no clear architectural benefit over Option A at this stage.

---

## 5. Option C — Defer to `paladin-ports` (Epic 2)

### Description

Do not move any types in this Epic. Instead, introduce a temporary shim: keep the `application::` imports in `src/core/` but wrap them in a `#[cfg(not(workspace))]` compile flag or a cargo feature. The Epic 2 `paladin-ports` crate will define these types; both `paladin-core` and `application` will depend on `paladin-ports`.

### Files Touched

- Minimal changes in this Epic; deferred to Epic 2.

### Pros

- No changes to type ownership in this Epic, lower risk.
- The three-crate dependency graph (`paladin-core` ← `paladin-ports` ← application) matches the final target architecture exactly.

### Cons

- **Blocks Epic 1 acceptance criteria**: FR-16 requires no `application::` imports in `paladin-core` after extraction. This option explicitly defers satisfying FR-16.
- Requires a cargo feature flag hack to make the monolith still compile — adds complexity that must be cleaned up.
- Delays the architectural benefit of a truly isolated `paladin-core` by a full Epic.
- `paladin-ports` not yet designed/scoped in detail; its exact API surface is TBD.

---

## 6. Recommendation

**Option A** is recommended.

The five pure value types (`PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) semantically belong in `paladin-core` — they are domain vocabulary with no infrastructure dependencies. Their current placement in `application::ports::output` was an implementation convenience, not an architectural decision. Moving them to `paladin-core` satisfies all Epic 1 acceptance criteria, creates zero breaking changes (application re-exports preserve all existing paths), and is the minimal change that unblocks clean extraction.

The `PaladinError` situation is resolved by removing the convenience `pub use` from `herald.rs` — this is a correct architectural decision independent of the workspace split.

---

## 7. Implementer Interview Questions (Task 3.3)

Before proceeding, answer the following:

1. **Scope confirmation**: Are you comfortable with the five pure value types moving to `paladin-core` and the application ports becoming thin re-exports? This is non-breaking but does change "ownership" of those types.
2. **PaladinError**: Confirm you accept removing the `pub use PaladinError` from `herald.rs`. Callers that currently do `use paladin::core::platform::container::herald::PaladinError` will need to change their import to `use paladin::application::use_cases::paladin::error::PaladinError`.
3. **New module placement**: The moved types will live in `crates/paladin-core/src/platform/container/execution_result.rs`, `token_usage.rs`, `registry_error.rs`, and `arsenal/handoff_error.rs`. Does this placement make sense, or should they go in a dedicated `crates/paladin-core/src/types/` module?
4. **Option override**: If Option A is rejected, specify Option B or C and the rationale.
