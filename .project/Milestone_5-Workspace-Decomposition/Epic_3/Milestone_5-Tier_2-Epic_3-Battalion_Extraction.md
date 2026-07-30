## Epic 3: Extract `paladin-battalion` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (paladin-core), Epic 2 (paladin-ports)

### Objective

Extract the complete orchestration runtime — all eight Battalion patterns plus the Commander strategy router — into a dedicated `paladin-battalion` crate. This is the primary public value of the framework. After extraction, a downstream consumer who only needs multi-agent orchestration can depend on `paladin-core` + `paladin-ports` + `paladin-battalion` without pulling in any LLM provider implementation, storage backend, web server, or notification dependency.

### Background & Rationale

The Battalion subsystem consists of approximately 13k LOC of execution services across the application use-cases layer (`src/application/use_cases/battalion/`) plus ~9k LOC of domain types already extracted to `paladin-core` in Epic 1. The execution services include `FormationExecutionService`, `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`, `ConclaveExecutionService`, `CouncilExecutionService`, `GroveExecutionService`, `ManeuverExecutionService`, and the `Commander` strategy router.

These services depend only on port traits (from `paladin-ports`) and domain types (from `paladin-core`). They operate through `Arc<dyn PaladinPort>`, `Arc<dyn PaladinRegistry>`, and similar trait objects — they never reference concrete adapter implementations. This clean dependency inversion makes extraction straightforward.

The Maneuver DSL (lexer, AST, parser) is currently split between `core/platform/container/battalion/parser/` (already in `paladin-core` after Epic 1) and `application/use_cases/battalion/maneuver_service.rs`. The execution service moves to `paladin-battalion`; the parser primitives remain in `paladin-core`.

### Acceptance Criteria

1. `crates/paladin-battalion/` exists with its own `Cargo.toml`.
2. `paladin-battalion` depends only on `paladin-core` and `paladin-ports` (plus `tokio`, `log`, `uuid`, `async-trait`, `serde`).
3. `paladin-battalion` does NOT depend on any concrete adapter, LLM provider, storage backend, or infrastructure crate.
4. All eight orchestration patterns are functional and tested within the crate.
5. The `Commander` strategy router and auto-detection heuristics work correctly.
6. `cargo build -p paladin-battalion` succeeds in isolation.
7. All battalion-related tests (unit + integration where applicable) pass.
8. The `paladin` facade crate re-exports battalion types at the existing paths.

### Tasks

#### Task 3.1: Extract Execution Services

**Description:** Move all battalion execution service modules from `src/application/use_cases/battalion/` to `crates/paladin-battalion/src/`. This includes: `formation_service.rs`, `phalanx_service.rs`, `campaign_service.rs`, `chain_of_command_service.rs`, `conclave_execution_service.rs`, `council_service.rs`, `grove_service.rs`, `maneuver_service.rs`, and `commander.rs`.

**Deliverables:**
- All execution service files relocated to `paladin-battalion`.
- Internal `use` paths updated to reference `paladin_core::` and `paladin_ports::`.
- `cargo build -p paladin-battalion` succeeds.

**Estimated Effort:** Large

#### Task 3.2: Extract Flow Visualizer and Battalion Utilities

**Description:** Move the battalion flow visualizer, configuration utilities, and any shared battalion helpers that support the execution services.

**Deliverables:**
- Visualizer and utility modules relocated.
- No remaining battalion execution code in the main `paladin` crate's `use_cases/`.
- All battalion-related functionality accessible through `paladin-battalion`.

**Estimated Effort:** Medium

#### Task 3.3: Verify Dependency Isolation

**Description:** Run dependency analysis to confirm `paladin-battalion` has no transitive dependency on infrastructure crates, LLM providers, or storage backends. Verify that `cargo build -p paladin-battalion` does not download or compile `reqwest`, `actix-web`, `sqlx`, `redis`, `qdrant-client`, `lettre`, or any other infrastructure dependency.

**Deliverables:**
- `cargo tree -p paladin-battalion` output showing clean dependency tree.
- Dependency graph visualization confirming isolation.
- Any leaking dependencies identified and resolved.

**Estimated Effort:** Small

#### Task 3.4: Wire `paladin-battalion` into the Facade Crate

**Description:** Update the root `paladin` crate to depend on `paladin-battalion` and re-export its types. Remove duplicated battalion execution services from the main crate source. Verify all examples and integration tests compile.

**Deliverables:**
- `paladin` crate depends on `paladin-battalion`.
- `src/application/use_cases/battalion/` removed or reduced to re-export shim.
- All existing examples compile and run correctly.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

#### Task 3.5: Battalion Crate Documentation and Examples

**Description:** Write crate-level documentation for `paladin-battalion` including usage examples showing how to use the crate independently with custom `PaladinPort` implementations. Verify `cargo doc -p paladin-battalion` produces clean output.

**Deliverables:**
- Crate-level `//!` documentation in `lib.rs`.
- At least one standalone example per orchestration pattern.
- `cargo doc -p paladin-battalion --no-deps` clean with no warnings.

**Estimated Effort:** Medium

---
