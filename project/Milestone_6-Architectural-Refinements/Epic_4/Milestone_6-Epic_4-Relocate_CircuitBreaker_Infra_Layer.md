
## Epic 4: Relocate `CircuitBreaker` to the Infrastructure Layer

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Milestone 2 complete (workspace crates exist)

### Objective

Move the `CircuitBreaker` implementation from `application/use_cases/paladin/circuit_breaker.rs` to the infrastructure layer, alongside retry logic and rate limiting. The circuit breaker pattern is an infrastructure resilience concern, not a domain use case.

### Background & Rationale

The `CircuitBreaker` is currently located at `src/application/use_cases/paladin/circuit_breaker.rs`. It implements the classic circuit breaker pattern with three states (Closed, Open, HalfOpen), failure/success thresholds, and timeout-based recovery. It is generic and reusable — it wraps any fallible operation, not just Paladin-specific operations.

In hexagonal architecture, infrastructure concerns like circuit breaking, retry logic, rate limiting, connection pooling, and timeout management belong in the infrastructure layer. They are implementation details of how the system handles failure, not business logic. The `PaladinExecutionService` would consume the circuit breaker via dependency injection or a port trait, maintaining the proper layering.

The circuit breaker has comprehensive tests (concurrent access, state transitions, threshold behavior) and is used by `PaladinExecutionService` and various example files.

### Acceptance Criteria

1. `CircuitBreaker` and `CircuitState` are relocated from `application/use_cases/paladin/` to an infrastructure module (e.g., `infrastructure/resilience/circuit_breaker.rs` or a dedicated `paladin-infra` utility within the workspace).
2. `PaladinExecutionService` imports `CircuitBreaker` from its new location.
3. All circuit breaker tests pass in the new location.
4. All integration tests and examples that use `CircuitBreaker` compile and pass.
5. The facade crate re-exports `CircuitBreaker` at the original path for backward compatibility.
6. The `application/use_cases/paladin/` directory no longer contains infrastructure concerns.

### Tasks

#### Task 4.1: Determine Target Location

**Description:** Evaluate where the `CircuitBreaker` should live in the workspace:

- **Option A:** `paladin/src/infrastructure/resilience/circuit_breaker.rs` — Keep it in the main facade crate's infrastructure layer. Simple, minimal disruption.
- **Option B:** A new `paladin-infra` crate for shared infrastructure utilities (circuit breaker, retry policies, rate limiters). Cleaner separation but introduces a new crate.
- **Option C:** Within `paladin-battalion` since it's primarily used by execution services. Pragmatic but semantically imprecise.

**Deliverables:**
- Decision document with trade-off analysis.
- Selected target location.

**Estimated Effort:** Small

#### Task 4.2: Relocate `CircuitBreaker`

**Description:** Move `circuit_breaker.rs` to the selected location. Update all imports in `PaladinExecutionService`, examples, and test files.

**Deliverables:**
- `CircuitBreaker` and `CircuitState` relocated.
- All imports updated.
- `cargo build --workspace` succeeds.

**Estimated Effort:** Small

#### Task 4.3: Add Facade Re-Export and Update Documentation

**Description:** Add a re-export in the facade crate so the original import path continues to work. Update the `STABLE_API.md` to reflect the new canonical location. Update `rustdoc` for the circuit breaker module.

**Deliverables:**
- Facade re-export added.
- `STABLE_API.md` updated.
- `cargo doc --workspace --no-deps` clean.
- All tests pass.

**Estimated Effort:** Small

---
