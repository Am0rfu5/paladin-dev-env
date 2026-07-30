# PRD: Relocate `CircuitBreaker` to the Infrastructure Layer

**Feature Name:** relocate-circuitbreaker-infra
**Milestone:** 6 — Architectural Refinements
**Epic:** 4
**Status:** Ready for Implementation
**Created:** 2026-05-25
**Author:** AI-assisted, reviewed by team
**Depends on:** Milestone 5 complete (workspace crates and facade re-exports in place)

---

## 1. Introduction / Overview

The `CircuitBreaker` implementation currently lives at
`src/application/use_cases/paladin/circuit_breaker.rs`. It is a generic, reusable
fault-tolerance pattern — it wraps any fallible operation with three states (Closed,
Open, HalfOpen) and timeout-based recovery. It contains no business logic and no domain
rules about what a Paladin *does* — it only describes how failure is *handled*.

**The problem this solves:** In hexagonal architecture, infrastructure concerns — circuit
breaking, retry policies, rate limiting, connection pool management — belong in the
infrastructure layer, not in the application use-cases layer. Placing `CircuitBreaker`
inside `application/use_cases/paladin/` implies it is a domain concept specific to Paladin
execution, when it is actually a generic resilience primitive reusable by any adapter. It
also sets a precedent that makes the use-cases layer increasingly polluted with operational
plumbing over time.

**The goal:** Move `CircuitBreaker` and `CircuitState` to
`src/infrastructure/resilience/circuit_breaker.rs` within the facade crate. Create a
`resilience/` module scaffold in the infrastructure layer to serve as the canonical home
for all future resilience utilities (retry policies, rate limiters, bulkheads). Update
every consumer — `PaladinExecutionService`, 15 example files, 3 test files, and all
inline rustdoc examples — to import from the new canonical path. Remove the old file and
its module registration. Update `STABLE_API.md` to record the new canonical path.

---

## 2. Goals

1. Move `CircuitBreaker` and `CircuitState` from `src/application/use_cases/paladin/` to
   `src/infrastructure/resilience/`.
2. Create `src/infrastructure/resilience/mod.rs` as a module scaffold documenting the
   resilience boundary for future additions (retry, rate-limiter, bulkhead).
3. Update `PaladinExecutionService` to import `CircuitBreaker` from the new path.
4. Update all 15 example files that import `CircuitBreaker` to use the new path.
5. Update all integration/unit test files that import `CircuitBreaker` to use the new path.
6. Update all inline rustdoc examples inside `circuit_breaker.rs` itself to use the new
   canonical path.
7. Remove `circuit_breaker` from the `application/use_cases/paladin/mod.rs` module
   registration (no re-export left behind — old path is intentionally broken).
8. Update `STABLE_API.md` to record
   `paladin::infrastructure::resilience::circuit_breaker` as the new stable module path.
9. `cargo build --workspace` must succeed and `cargo test` must pass.

---

## 3. User Stories

**As a developer adding a new resilience primitive (e.g., a retry policy),**
I want to place it alongside `CircuitBreaker` in `src/infrastructure/resilience/`,
so that all fault-tolerance utilities are co-located and consistently categorized as
infrastructure concerns.

**As a developer reading `src/application/use_cases/paladin/`,**
I want to see only domain logic about how a Paladin executes a reasoning loop,
so that I am not confused by operational plumbing that has nothing to do with the
Paladin domain.

**As a developer reading `PaladinExecutionService`,**
I want the `CircuitBreaker` import to come from `infrastructure::resilience::`,
so that the import statement itself communicates that circuit-breaking is an
infrastructure-layer resilience concern injected into the service.

**As a developer onboarding to the codebase and looking for resilience utilities,**
I want a single `src/infrastructure/resilience/` directory that documents all
available patterns,
so that I immediately know where to look and where to add new utilities.

---

## 4. Functional Requirements

### 4.1 Target Module Structure

After this Epic, the infrastructure module must contain the following new files. Nothing
else in the infrastructure layer changes.

```
src/infrastructure/
├── mod.rs                          # Updated: add `pub mod resilience;`
├── resilience/
│   ├── mod.rs                      # New: module doc, pub mod circuit_breaker;
│   │                               #   (scaffold for future: retry, rate_limiter)
│   └── circuit_breaker.rs          # Moved from application/use_cases/paladin/
└── adapters/                       # Unchanged
    └── ...
```

The `application/use_cases/paladin/` directory must remove `circuit_breaker.rs` and its
`pub mod circuit_breaker;` declaration from `mod.rs`. No re-export of the old path is
added — the old path is intentionally retired.

### 4.2 `src/infrastructure/resilience/mod.rs` Content

The new `mod.rs` must:
- Have a module-level rustdoc comment explaining that this module is the canonical home
  for infrastructure-layer resilience primitives.
- Declare `pub mod circuit_breaker;`.
- Include a comment block listing the patterns planned for future addition (retry,
  rate-limiter, bulkhead) so the next developer knows this is the intended location.

### 4.3 `circuit_breaker.rs` — Internal Import Update

The `circuit_breaker.rs` file currently imports:

```rust
use crate::application::use_cases::paladin::error::PaladinError;
```

This import path remains valid after relocation (the module moves within the same
facade crate; `crate::application::use_cases::paladin::error::PaladinError` still
resolves correctly). **Do not change this import** unless `cargo build` fails after the
move.

### 4.4 All Inline rustdoc Examples in `circuit_breaker.rs`

Every `use` statement inside `///` or `//!` doc examples within the moved file must be
updated from:

```rust
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
```

to:

```rust
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
```

This applies to both the module-level `//!` doc block examples and all method-level `///`
doc examples.

### 4.5 `PaladinExecutionService` Import Update

`src/application/use_cases/paladin/paladin_execution_service.rs` must change its import
from:

```rust
use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
```

to:

```rust
use crate::infrastructure::resilience::circuit_breaker::CircuitBreaker;
```

All inline rustdoc examples inside `paladin_execution_service.rs` that reference the old
import path must be updated to the new path as well.

### 4.6 Example Files — Import Update (15 files)

Each of the following example files imports `CircuitBreaker` using the old application
path. Every `use` statement must be updated to the new canonical path:

| File | Line (approx.) |
|------|---------------|
| `examples/basic_paladin.rs` | 17 |
| `examples/agent_handoffs.rs` | 17 |
| `examples/autonomous_full_config.rs` | 19 |
| `examples/autonomous_planning.rs` | 18 |
| `examples/autonomous_prompt_generation.rs` | 17 |
| `examples/battalion_checkpoint_recovery.rs` | 29 |
| `examples/citadel_autosave.rs` | 22 |
| `examples/citadel_restore.rs` | 24 |
| `examples/dynamic_temperature.rs` | 17 |
| `examples/herald_custom_formatter.rs` | 8 |
| `examples/herald_json_output.rs` | 8 |
| `examples/herald_markdown_output.rs` | 8 |
| `examples/paladin_with_config.rs` | 18 |
| `examples/vision_analysis.rs` | 11 |
| `examples/vision_battalion.rs` | 10 |

**Old import (all files):**

```rust
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
```

**New import (all files):**

```rust
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
```

### 4.7 Test Files — Import Update (3 files)

The following workspace test files import `CircuitBreaker` using the old application path
and must be updated to the new path:

| File |
|------|
| `tests/cli/paladin_execution_test.rs` |
| `tests/cli/tool_integration_test.rs` |
| `tests/cli/error_handling_test.rs` |

**Old import:**

```rust
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
```

**New import:**

```rust
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
```

### 4.8 `README.md` Doc Example Update

`README.md` contains at least one inline code example referencing the old path (lines
659–660). Update the `use` statement to the new canonical path and update the surrounding
prose if it names the module location.

### 4.9 `src/infrastructure/mod.rs` — Add `resilience` Module Declaration

Add `pub mod resilience;` to `src/infrastructure/mod.rs`. The placement should be
consistent with the existing module declarations (alphabetical order is preferred).

### 4.10 `src/lib.rs` / Facade Crate — Expose `infrastructure::resilience`

Verify that `src/lib.rs` (the facade crate root) already exposes `pub mod infrastructure;`
or equivalent, so that `paladin::infrastructure::resilience::circuit_breaker` is accessible
from outside the crate. If `infrastructure` is currently private or re-exported differently,
update the visibility to make `infrastructure::resilience::circuit_breaker` part of the
public API surface.

### 4.11 Remove Old Module Registration

Remove the `pub mod circuit_breaker;` line from
`src/application/use_cases/paladin/mod.rs`. The old module path
`paladin::application::use_cases::paladin::circuit_breaker` must no longer resolve after
this Epic — no `pub use` re-export is added.

### 4.12 `STABLE_API.md` Update

Update `STABLE_API.md` to:
- Remove the entry for `paladin::application::use_cases::paladin::circuit_breaker`.
- Add the entry for `paladin::infrastructure::resilience::circuit_breaker` as the new
  stable module location, with `CircuitBreaker` and `CircuitState` as the stable public
  types.

---

## 5. Non-Goals (Out of Scope)

- **No new resilience primitives.** The `resilience/mod.rs` scaffold is documentation-only.
  No retry policy, rate limiter, or bulkhead implementation is part of this Epic.
- **No crate extraction.** The circuit breaker stays in the `paladin` facade crate. A
  `paladin-infra` crate is explicitly out of scope for this Epic.
- **No changes to `CircuitBreaker` behavior.** The three states, thresholds, and
  timeout logic are unchanged. This is a pure relocation with no functional modification.
- **No changes to `PaladinError`.** `PaladinError::CircuitBreakerOpen` remains in the
  application layer. The circuit breaker continues to import `PaladinError` from
  `crate::application::use_cases::paladin::error`.
- **No port trait abstraction for `CircuitBreaker`.** A `CircuitBreakerPort` trait is a
  future consideration, not part of this Epic.
- **No changes to other infrastructure adapters.** The existing retry logic embedded in
  `mcp_sse_adapter.rs` and `api_content_deliverer.rs` is not consolidated into the
  resilience module during this Epic.

---

## 6. Design Considerations

### Module Visibility

`src/infrastructure/resilience/circuit_breaker.rs` should be `pub mod circuit_breaker`
inside `resilience/mod.rs`, and the `resilience` module should be `pub mod resilience`
inside `infrastructure/mod.rs`. This makes `paladin::infrastructure::resilience::circuit_breaker`
a fully public path, consistent with how the old path was publicly accessible.

### Dependency Direction Note

Moving `CircuitBreaker` to the infrastructure layer means `PaladinExecutionService`
(in the application layer) will import a type from the infrastructure layer. This is
technically a layering inversion within the facade crate. However, because:

1. This is a module-level move within a single crate (not a cross-crate dependency), and
2. The circuit breaker is injected as an `Arc<CircuitBreaker>` parameter (dependency
   injection), not constructed inside the service,

this is an acceptable pragmatic trade-off within the facade crate's module organization,
consistent with the project's approach documented in the Milestone 6 overview. A future
`CircuitBreakerPort` trait in `paladin-ports` would resolve this cleanly if ever needed.

### `resilience/mod.rs` Scaffold Comment

The scaffold comment should follow this pattern to guide future contributors:

```rust
//! Infrastructure resilience primitives.
//!
//! This module is the canonical home for fault-tolerance and resilience
//! utilities used by infrastructure adapters and injected into application
//! services.
//!
//! # Current Modules
//! - [`circuit_breaker`] — Three-state circuit breaker (Closed/Open/HalfOpen)
//!
//! # Planned Additions (not yet implemented)
//! - `retry` — Configurable retry policy with exponential backoff
//! - `rate_limiter` — Token-bucket rate limiter for LLM API calls
//! - `bulkhead` — Concurrency limiter for external service calls

pub mod circuit_breaker;
```

---

## 7. Technical Considerations

- **`PaladinError` import direction.** The moved `circuit_breaker.rs` uses
  `crate::application::use_cases::paladin::error::PaladinError`. In the facade crate,
  modules in `infrastructure::` can freely import from `application::` without violating
  Rust's module system. Confirm the import resolves after the file move with
  `cargo check` before proceeding to update all consumers.
- **`final-api.txt`.** This file records the public API surface. After the move, run
  `cargo doc --workspace --no-deps` and regenerate `final-api.txt` (or equivalent) to
  reflect the new module path. Verify the old path no longer appears.
- **`api_surface_current.txt`.** This file may also need updating alongside
  `final-api.txt` to reflect the API surface change.
- **Doc test compilation.** All rustdoc examples in `circuit_breaker.rs` use the full
  `paladin::` path. After the file moves, these doc tests will fail if the path is not
  updated (Requirement 4.4). Run `cargo test --doc` to verify.
- **`cargo clippy` after relocation.** The move will trigger `unused_imports` warnings
  in `paladin_execution_service.rs` if not updated atomically. Complete Requirement 4.5
  in the same commit as the file move.

---

## 8. Success Metrics

1. `cargo build --workspace` succeeds with zero errors after the move.
2. `cargo test` passes — all existing tests that use `CircuitBreaker` compile and pass
   with the new import path.
3. `cargo test --doc` passes — all rustdoc examples in `circuit_breaker.rs` and
   `paladin_execution_service.rs` compile with the new path.
4. `cargo clippy --workspace -- -D warnings` reports zero warnings.
5. `cargo fmt --all -- --check` passes.
6. `cargo doc --workspace --no-deps` produces clean documentation with no broken links.
7. The path `paladin::application::use_cases::paladin::circuit_breaker` no longer
   resolves (grepping `mod.rs` confirms no lingering re-export).
8. `STABLE_API.md` reflects the new canonical path.
9. All 15 example files, 3 test files, and `README.md` compile with the new import path.

---

## 9. Open Questions

1. **`final-api.txt` / `api_surface_current.txt` regeneration process.** Is there an
   automated script (e.g., `make api-surface` or similar) for regenerating these files,
   or are they updated manually? The implementer should confirm with the team before
   closing this Epic.
2. **`PaladinError` long-term home.** `PaladinError::CircuitBreakerOpen` is an
   application-layer error variant named after an infrastructure concern. Should it be
   renamed (e.g., `PaladinError::ResilienceCircuitOpen`) or moved in a future Epic? This
   is out of scope here but worth recording.
3. **Other embedded retry logic.** `mcp_sse_adapter.rs` and `api_content_deliverer.rs`
   each contain inline retry logic not using the `CircuitBreaker`. Should these be
   consolidated into `infrastructure::resilience::retry` in a follow-on Epic? Recommended
   to raise this as a new ticket after Epic 4 closes.
