
## Epic 3: Co-locate the Maneuver DSL with the Battalion Execution Layer

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Milestone 2 Epic 1 (paladin-core) and Epic 3 (paladin-battalion) must be complete

### Objective

Consolidate the Maneuver DSL components — the lexer, AST, parser (currently in `paladin-core` at `core/platform/container/battalion/parser/`), and the `ManeuverExecutionService` (in `paladin-battalion`) — into a single cohesive location within `paladin-battalion`. These components are tightly coupled and should travel together.

### Background & Rationale

The Maneuver Flow DSL consists of:

- **Lexer** (`core/platform/container/battalion/parser/lexer.rs`) — Tokenizes flow expression strings into `Token` sequences.
- **AST** (`core/platform/container/battalion/parser/ast.rs`) — Defines `FlowExpression` as the parsed tree structure (Agent, Sequential, Parallel nodes).
- **Parser** (`core/platform/container/battalion/parser/mod.rs`) — The `FlowParser` that converts strings to `FlowExpression` ASTs.
- **Error types** (`core/platform/container/battalion/parser/error.rs`) — `FlowParseError` with position tracking and suggestions.
- **Maneuver domain type** (`core/platform/container/battalion/maneuver.rs`) — The `Maneuver` struct and `ManeuverConfig`.
- **Execution service** (`application/use_cases/battalion/maneuver_service.rs`) — `ManeuverExecutionService` that interprets the AST and orchestrates agent execution.
- **Visualization** — ASCII and Mermaid diagram generation for flow expressions.

The parser and AST are only consumed by the Maneuver execution service and the visualization layer. No other battalion pattern uses the Flow DSL parser directly. Keeping the parser in `paladin-core` means it is available to crates that will never use it, and it cannot be modified without rebuilding the core crate.

Moving the parser to `paladin-battalion` means the entire Maneuver subsystem — from string parsing to AST construction to execution to visualization — lives in one crate. The `FlowExpression` AST type, which is the primary public type, can be re-exported from `paladin-core` if needed for backward compatibility, or consumers can depend on `paladin-battalion` directly.

### Acceptance Criteria

1. The `parser/` directory (lexer, AST, parser, error types) moves from `paladin-core` to `paladin-battalion`.
2. The `Maneuver` domain type and `ManeuverConfig` move from `paladin-core` to `paladin-battalion`.
3. `FlowExpression` and `FlowParseError` are publicly exported from `paladin-battalion`.
4. The `paladin` facade crate re-exports `FlowExpression` and `FlowParser` at the original paths for backward compatibility.
5. `paladin-core`'s `battalion/` module no longer contains parser or maneuver modules (only Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove domain types, and shared battalion types like `BattalionConfig`, `BattalionResult`, `BattalionStrategy`).
6. All 113 Maneuver-related tests pass from within `paladin-battalion`.
7. All 32 Maneuver benchmarks pass.
8. `cargo build -p paladin-core` no longer compiles any parser code.
9. `cargo doc -p paladin-battalion` documents the complete Maneuver subsystem including parser.

### Tasks

#### Task 3.1: Assess Cross-Crate Impact

**Description:** Identify every file and test that imports from `core::platform::container::battalion::parser` or `core::platform::container::battalion::maneuver`. Map the full impact radius of the move to determine which re-exports are needed in the facade crate.

**Deliverables:**
- Import reference map for parser and maneuver types.
- List of facade re-exports needed for backward compatibility.
- Identification of any types in other crates that depend on `FlowExpression`.

**Estimated Effort:** Small

#### Task 3.2: Move Parser and Maneuver Modules to `paladin-battalion`

**Description:** Physically relocate the `parser/` directory (lexer.rs, ast.rs, error.rs, mod.rs) and `maneuver.rs` from `paladin-core/src/platform/container/battalion/` to `paladin-battalion/src/maneuver/`. Reorganize within `paladin-battalion` so the Maneuver subsystem has a clean internal structure:

```
paladin-battalion/src/
├── maneuver/
│   ├── mod.rs           # Maneuver domain type + ManeuverConfig + re-exports
│   ├── parser/
│   │   ├── mod.rs       # FlowParser
│   │   ├── lexer.rs     # Lexer + Token
│   │   ├── ast.rs       # FlowExpression
│   │   └── error.rs     # FlowParseError
│   ├── service.rs       # ManeuverExecutionService (already here)
│   └── visualizer.rs    # Flow visualization (ASCII + Mermaid)
```

**Deliverables:**
- Parser and maneuver modules relocated.
- `paladin-battalion`'s `Cargo.toml` updated if any new dependencies are needed (unlikely — parser is dependency-free beyond `serde`).
- `cargo build -p paladin-battalion` succeeds.
- `cargo build -p paladin-core` succeeds without parser code.

**Estimated Effort:** Medium

#### Task 3.3: Update Imports and Facade Re-Exports

**Description:** Update all import paths throughout the workspace. Add re-exports in the `paladin` facade crate so that `use paladin::core::platform::container::battalion::parser::FlowParser` continues to resolve (pointing to the new location in `paladin-battalion`).

**Deliverables:**
- All import paths updated.
- Facade re-exports added.
- All examples and integration tests compile.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

#### Task 3.4: Migrate and Consolidate Tests

**Description:** Move all 57 parser tests and 21 maneuver domain tests from their original locations into `paladin-battalion`'s test suite. Consolidate with the existing 3 execution service tests, 16 commander integration tests, and 12 visualization tests. Verify the full 113-test Maneuver suite passes from `paladin-battalion`.

**Deliverables:**
- All Maneuver tests consolidated in `paladin-battalion`.
- 113 tests passing.
- 32 benchmarks passing.
- No orphaned test files in `paladin-core`.

**Estimated Effort:** Small

---
