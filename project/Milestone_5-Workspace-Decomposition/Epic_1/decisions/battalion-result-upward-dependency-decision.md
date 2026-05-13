# Decision: Resolving `src/core/` → `application::` Upward Dependencies

**Epic:** Milestone 5 Epic 1 — Workspace Initialization and `paladin-core` Extraction  
**Decision Date:** 2026-05-13  
**Chosen Option:** **Option A — Move Pure Value Types to `paladin-core`**  
**Status:** Approved — implementation sub-tasks appended to task list as 3.6a–3.6k

---

## Chosen Option: A

Move the five pure value types into `src/core/platform/container/` (their permanent home for this Epic; they travel to `paladin-core` in Task 5.0). The application ports that currently define them become thin re-exports.

### Rationale

- All five types (`PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) are pure value/error types whose only external dependencies are `serde` and `thiserror` — exactly the deps allowed in `paladin-core`.
- `TaskPlan` and `HandoffRecord`, referenced by `PaladinResult`, are already in `src/core/platform/container/`. No circular dependency is introduced.
- Application ports re-exporting from core is architecturally correct: the application layer depends on core, never the reverse.
- Zero breaking changes: all existing `paladin::application::ports::output::...` paths continue to resolve via re-exports.
- `PaladinError` is deliberately excluded: it depends on `GarrisonError` which lives in `application::`. The `pub use PaladinError` in `herald.rs` is a convenience re-export with no actual core usage — removing it is the correct architectural decision.

### Rejected Options

- **Option B (New parallel types):** Introduces duplicate type hierarchies and large refactor scope, violating the "structural refactor only" constraint of this Epic.
- **Option C (Defer to Epic 2):** Explicitly fails FR-16. Deferring creates a technical debt dependency between Epics.

---

## Confirmed Implementation Checklist

The following files will be created/modified. Sub-tasks 3.6a–3.6k are appended to the task list.

### New files in `src/core/platform/container/`

| File | Content |
|------|---------|
| `execution_result.rs` | `PaladinResult` struct + `StopReason` enum (moved from `application::ports::output::paladin_port`) |
| `token_usage.rs` | `TokenUsage` struct (moved from `application::ports::output::llm_port`) |
| `registry_error.rs` | `RegistryError` enum (moved from `application::ports::output::paladin_registry`) |

### Moved file

| From | To |
|------|----|
| `src/application/errors/handoff_error.rs` | `src/core/platform/container/arsenal/handoff_error.rs` |

`HandoffError` is an Arsenal-domain error type; placing it under `arsenal/` is correct.  
`src/application/errors/handoff_error.rs` will be replaced by a thin re-export module.

### Application layer re-export changes

| File | Change |
|------|--------|
| `src/application/ports/output/paladin_port.rs` | `PaladinResult` and `StopReason` struct/enum bodies removed; replaced with `pub use crate::core::platform::container::execution_result::{PaladinResult, StopReason};` |
| `src/application/ports/output/llm_port.rs` | `TokenUsage` struct body removed; replaced with `pub use crate::core::platform::container::token_usage::TokenUsage;` |
| `src/application/ports/output/paladin_registry.rs` | `RegistryError` enum body removed; replaced with `pub use crate::core::platform::container::registry_error::RegistryError;` |
| `src/application/errors/handoff_error.rs` | Entire content replaced with `pub use crate::core::platform::container::arsenal::handoff_error::HandoffError;` |

### Core layer import updates

| File | Change |
|------|--------|
| `src/core/platform/container/mod.rs` | Add `pub mod execution_result; pub mod token_usage; pub mod registry_error;` |
| `src/core/platform/container/arsenal/mod.rs` | Add `pub mod handoff_error;` |
| `src/core/platform/container/battalion/mod.rs` | Replace `use crate::application::ports::output::paladin_port::{PaladinResult, StopReason}` with `use crate::core::platform::container::execution_result::{PaladinResult, StopReason}` and `use crate::application::ports::output::paladin_registry::RegistryError` with `use crate::core::platform::container::registry_error::RegistryError` |
| `src/core/platform/container/battalion/conclave.rs` | Replace `use crate::application::ports::output::paladin_port::PaladinResult` with `use crate::core::platform::container::execution_result::PaladinResult` |
| `src/core/platform/container/herald.rs` | Remove `pub use crate::application::use_cases::paladin::error::PaladinError`; update `PaladinResult` and `TokenUsage` imports to core paths |
| `src/core/platform/container/arsenal/handoff_tool.rs` | Replace `use crate::application::errors::handoff_error::HandoffError` with `use crate::core::platform::container::arsenal::handoff_error::HandoffError` |
