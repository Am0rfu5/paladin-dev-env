# PRD: Co-locate the Maneuver DSL with the Battalion Execution Layer

**Feature Name:** co-locate-maneuver-dsl
**Milestone:** 6 — Architectural Refinements
**Epic:** 3
**Status:** Ready for Implementation
**Created:** 2026-05-25
**Author:** AI-assisted, reviewed by team
**Depends on:** Milestone 5 Epic 1 (`paladin-core`) and Epic 3 (`paladin-battalion`) must be complete

---

## 1. Introduction / Overview

The Maneuver Flow DSL subsystem is currently split across two workspace crates. The lexer, AST, parser, and error types live in `paladin-core` at `crates/paladin-core/src/platform/container/battalion/parser/`, and the `Maneuver` domain type lives at `crates/paladin-core/src/platform/container/battalion/maneuver.rs`. The execution service (`maneuver_service.rs`) and visualization (`flow_visualizer.rs`) already live in `paladin-battalion`. Nothing else in `paladin-core` uses the parser, lexer, or `Maneuver` type — they exist there solely because they were placed there before the workspace decomposition in Milestone 5.

**The problem this solves:** The parser and its consumers (the execution service and visualizer) are tightly coupled, but live in different crates. Any change to the Maneuver DSL requires touching both `paladin-core` and `paladin-battalion`, rebuilding two crates, and navigating two separate module trees. The `paladin-core` crate compiles parser code that no other core component ever uses — wasted compile time and a misleading module boundary that implies the parser is a domain primitive when it is actually specific to the Maneuver orchestration pattern.

**The goal:** Move the entire Maneuver subsystem — lexer, AST, parser, error types, Maneuver domain type, `ManeuverConfig` — into `paladin-battalion`, co-located with the execution service and visualizer. Reorganize these files into a coherent `maneuver/` sub-module within `paladin-battalion`. Add facade re-exports so all existing consumers (`paladin::core::platform::container::battalion::parser::FlowParser`, etc.) continue to compile without changes.

---

## 2. Goals

1. Move the Maneuver DSL parser (`parser/mod.rs`, `lexer.rs`, `ast.rs`, `error.rs`) from `paladin-core` to `paladin-battalion/src/maneuver/parser/`.
2. Move the `Maneuver` domain type and `ManeuverConfig` from `paladin-core` to `paladin-battalion/src/maneuver/mod.rs`.
3. Reorganize `paladin-battalion` so all Maneuver components form a unified `maneuver/` sub-module: `mod.rs` (domain type), `parser/` (DSL), `service.rs` (execution, renamed from `maneuver_service.rs`), and `visualizer.rs` (renamed from `flow_visualizer.rs`).
4. Add facade re-exports so that `paladin::core::platform::container::battalion::parser::*` and `paladin::core::platform::container::battalion::maneuver::*` paths continue to compile for all existing consumers (CLI commands, workspace-level tests, integration tests).
5. After the move, `cargo build -p paladin-core` must succeed with zero parser-related code compiled.
6. All inline `#[cfg(test)]` tests travel with their source files — no test code is lost or needs to be re-written.
7. Workspace-level test files in `tests/unit/` remain unchanged (they rely on facade re-exports to continue resolving their import paths).

---

## 3. User Stories

**As a developer working on the Maneuver Flow DSL,**
I want to find the lexer, AST, parser, domain type, execution service, and visualizer all in `paladin-battalion/src/maneuver/`,
so that I can understand, modify, and test the entire Maneuver subsystem without switching between two crates.

**As a developer building `paladin-core` in isolation,**
I want `cargo build -p paladin-core` to succeed without compiling any parser or Maneuver-specific code,
so that the core crate contains only domain primitives shared across all battalion patterns.

**As a developer adding a new Flow DSL operator,**
I want to change `lexer.rs`, `ast.rs`, `parser/mod.rs`, and `service.rs` in one place in `paladin-battalion`,
so that a single `cargo test -p paladin-battalion` verifies my entire change.

**As a developer using the CLI `maneuver` command,**
I want `use paladin::core::platform::container::battalion::parser::FlowParser` to keep compiling,
so that none of my existing code breaks after this architectural move.

---

## 4. Functional Requirements

### 4.1 Target Directory Structure in `paladin-battalion`

After this Epic, `paladin-battalion/src/` must be organized as follows. The `maneuver/` sub-module replaces the existing flat files `maneuver_service.rs` and `flow_visualizer.rs`:

```
paladin-battalion/src/
├── lib.rs                        # Updated: pub mod maneuver; replaces maneuver_service and flow_visualizer
├── maneuver/
│   ├── mod.rs                    # Maneuver struct, ManeuverConfig, and re-exports
│   ├── parser/
│   │   ├── mod.rs                # FlowParser (moved from paladin-core)
│   │   ├── lexer.rs              # Lexer, Token (moved from paladin-core)
│   │   ├── ast.rs                # FlowExpression (moved from paladin-core)
│   │   └── error.rs              # FlowParseError (moved from paladin-core)
│   ├── service.rs                # ManeuverExecutionService (renamed from maneuver_service.rs)
│   └── visualizer.rs             # Visualization (renamed from flow_visualizer.rs)
├── campaign_service.rs           # Unchanged
├── chain_of_command_service.rs   # Unchanged
├── commander.rs                  # Unchanged (import paths updated)
├── conclave_execution_service.rs # Unchanged
├── council_service.rs            # Unchanged
├── error_aggregation.rs          # Unchanged
├── formation_service.rs          # Unchanged
├── grove_service.rs              # Unchanged
├── in_memory_registry.rs         # Unchanged
├── phalanx_service.rs            # Unchanged
└── retry.rs                      # Unchanged
```

### 4.2 Files Being Moved (from `paladin-core`)

The following files must be physically moved to `paladin-battalion` as described above. Their content must not change — only their location and crate context change:

| Source path (paladin-core) | Destination path (paladin-battalion) | Lines |
|---|---|---|
| `src/platform/container/battalion/parser/mod.rs` | `src/maneuver/parser/mod.rs` | 250 |
| `src/platform/container/battalion/parser/lexer.rs` | `src/maneuver/parser/lexer.rs` | 269 |
| `src/platform/container/battalion/parser/ast.rs` | `src/maneuver/parser/ast.rs` | 267 |
| `src/platform/container/battalion/parser/error.rs` | `src/maneuver/parser/error.rs` | 188 |
| `src/platform/container/battalion/maneuver.rs` | `src/maneuver/mod.rs` (combined with new content) | 443 |

### 4.3 Files Being Reorganized (within `paladin-battalion`)

The following files already exist in `paladin-battalion` and must be renamed and relocated into the `maneuver/` sub-module. Their content does not change other than import path updates:

| Current path | New path | Lines | Change |
|---|---|---|---|
| `src/maneuver_service.rs` | `src/maneuver/service.rs` | 984 | Rename + import path updates |
| `src/flow_visualizer.rs` | `src/maneuver/visualizer.rs` | 663 | Rename + import path updates |

### 4.4 Inline Tests — No Separate Migration Needed

All Maneuver DSL tests follow the standard Rust pattern: `#[cfg(test)] mod tests { use super::*; ... }` at the bottom of each source file. These tests move automatically when their source file moves. No test code needs to be rewritten or consolidated.

Confirmed test counts per source file (verified during Task 3.1):

| File | Inline tests |
|---|---|
| `parser/mod.rs` | 4 |
| `parser/lexer.rs` | 8 |
| `parser/error.rs` | 5 |
| `parser/ast.rs` | 9 |
| `maneuver.rs` | 9 |
| `maneuver_service.rs` | TBD — confirm during Task 3.1 (file is 984 lines; inline tests likely present) |
| `flow_visualizer.rs` | 21 |
| `commander.rs` | 26 (unchanged, not moving) |

In addition, two workspace-level test files exist:

| File | Tests | Status after Epic |
|---|---|---|
| `tests/unit/parser_tests.rs` | 57 | Unchanged — will compile via facade re-exports |
| `tests/unit/maneuver_domain_tests.rs` | 21 | Unchanged — will compile via facade re-exports |

**Important:** No integration test files exist in `paladin-core/tests/` for parser or Maneuver functionality (confirmed: that directory does not exist). Task 3.1 must re-verify this finding before proceeding.

### 4.5 `paladin-core` Cleanup

After the move, `paladin-core` must have all Maneuver/parser references removed:

1. Remove `pub mod parser;` from `crates/paladin-core/src/platform/container/battalion/mod.rs`.
2. Remove `pub mod maneuver;` from `crates/paladin-core/src/platform/container/battalion/mod.rs`.
3. Delete (or confirm removal of) the `parser/` directory from `paladin-core`.
4. Delete (or confirm removal of) `maneuver.rs` from `paladin-core`.
5. `cargo build -p paladin-core` must succeed with zero parser-related code compiled.

### 4.6 Facade Re-Exports — Backward Compatibility

Existing consumers in the facade crate (`src/`) import the parser and Maneuver types through the `paladin::core::platform::container::battalion::*` path. This path is built from the facade's `src/core/platform/mod.rs`, which currently does:

```rust
pub use paladin_core::platform::container;
```

After the move, `paladin_core::platform::container::battalion` no longer exports `parser` or `maneuver`. To maintain the original import paths, the facade must be updated so that the `container::battalion::parser` and `container::battalion::maneuver` paths resolve to the new locations in `paladin-battalion`.

**Required approach:** In the facade's `src/core/platform/mod.rs`, replace the wholesale re-export of `paladin_core::platform::container` with an explicit `pub mod container` block that re-exports everything from `paladin_core::platform::container` AND adds forwarding sub-modules for the moved types:

```rust
// src/core/platform/mod.rs — updated
//
// NOTE: A wildcard `pub use paladin_core::platform::container::*` cannot be
// used at this level. The wildcard imports the `battalion` module name from
// paladin-core, which then conflicts with the local `pub mod battalion`
// declaration below (duplicate definition error). Instead, each non-battalion
// container sub-module must be re-exported explicitly. The complete list is
// determined during Task 3.3 by reading
// `paladin-core/src/platform/container/mod.rs`.
pub mod container {
    // Re-export every non-battalion container sub-module from paladin-core.
    // Do NOT use a wildcard here — see note above.
    // Example (full list confirmed during Task 3.3):
    pub use paladin_core::platform::container::arsenal;
    pub use paladin_core::platform::container::citadel;
    pub use paladin_core::platform::container::garrison;
    // ... (complete list from paladin-core/src/platform/container/mod.rs,
    //       excluding `battalion`)

    // Battalion — declared as a local module so Maneuver DSL types can be
    // injected from paladin-battalion alongside the remaining battalion types.
    //
    // The wildcard at THIS level is SAFE: by the time Task 3.3 runs,
    // paladin-core's battalion module no longer exports `parser` or `maneuver`
    // (both were removed in Task 3.2). There is no name conflict with the
    // local sub-module declarations below.
    pub mod battalion {
        pub use paladin_core::platform::container::battalion::*;

        // Re-export Maneuver DSL types at their original paths.
        pub mod parser {
            pub use paladin_battalion::maneuver::parser::*;
        }
        // NOTE: The type list below may be incomplete. Task 3.1 must verify
        // every type currently exported from
        // `paladin::core::platform::container::battalion::maneuver` and
        // confirm it is included here. `AgentResult` is one candidate that
        // may require addition.
        pub mod maneuver {
            pub use paladin_battalion::maneuver::{
                AgentResult,       // verify presence during Task 3.1
                ErrorStrategy, ExecutionStatus, Maneuver, ManeuverConfig,
                ManeuverError, ManeuverResult, OutputFormat,
            };
        }
    }
}

#[allow(missing_docs)]
pub mod manager;
```

After this change, all existing consumers — including CLI commands and workspace-level test files — continue to compile without modification:

| Consumer file | Import path | Status |
|---|---|---|
| `src/application/cli/commands/maneuver.rs` | `crate::core::platform::container::battalion::parser::FlowParser` | Resolves via facade re-export |
| `src/application/cli/commands/maneuver.rs` | `crate::core::platform::container::battalion::parser::FlowExpression` | Resolves via facade re-export |
| `src/application/cli/commands/battalion.rs` | `crate::core::platform::container::battalion::parser::FlowParser` | Resolves via facade re-export |
| `src/application/cli/commands/battalion.rs` | `crate::core::platform::container::battalion::maneuver::Maneuver` | Resolves via facade re-export |
| `src/application/cli/config/battalion_config.rs` | `crate::core::platform::container::battalion::parser::FlowParser` | Resolves via facade re-export |
| `tests/unit/parser_tests.rs` | `paladin::core::platform::container::battalion::parser::*` | Resolves via facade re-export |
| `tests/unit/maneuver_domain_tests.rs` | `paladin::core::platform::container::battalion::maneuver::*` | Resolves via facade re-export |
| `tests/unit/maneuver_domain_tests.rs` | `paladin::core::platform::container::battalion::parser::FlowParser` | Resolves via facade re-export |

**No consumer file needs to be modified as part of this Epic.**

### 4.7 Import Path Updates Inside `paladin-battalion`

Within `paladin-battalion`, the existing files that import parser and Maneuver types from `paladin-core` must be updated to import from the new local module:

| File | Old import | New import |
|---|---|---|
| `maneuver/service.rs` (was `maneuver_service.rs`) | `paladin_core::platform::container::battalion::maneuver::{...}` | `super::{...}` or `crate::maneuver::{...}` |
| `maneuver/service.rs` | `paladin_core::platform::container::battalion::parser::FlowExpression` | `super::parser::FlowExpression` |
| `maneuver/visualizer.rs` (was `flow_visualizer.rs`) | `paladin_core::platform::container::battalion::parser::FlowExpression` | `super::parser::FlowExpression` |
| `commander.rs` | `paladin_core::platform::container::battalion::maneuver::{Maneuver, ManeuverConfig}` | `crate::maneuver::{Maneuver, ManeuverConfig}` |
| `commander.rs` | `paladin_core::platform::container::battalion::parser::FlowParser` | `crate::maneuver::parser::FlowParser` |

**Important — inline fully-qualified paths in `commander.rs`:** The import updates above cover `use` statements at the top of the file. However, `commander.rs` also contains inline fully-qualified path expressions scattered through function bodies (e.g., `paladin_core::platform::container::battalion::maneuver::ErrorStrategy::FailFast` in match arms, `paladin_core::platform::container::battalion::maneuver::Maneuver::new(...)` in constructor calls). These are not captured by editing `use` blocks alone. Task 3.1 step 6 produces a grep list of all occurrences; Task 3.2 step 8 must address them.

### 4.8 `paladin-battalion` `lib.rs` Updates

`crates/paladin-battalion/src/lib.rs` must be updated:

1. Remove `pub mod maneuver_service;`
2. Remove `pub mod flow_visualizer;`
3. Add `pub mod maneuver;`
4. Ensure `FlowExpression`, `FlowParser`, `FlowParseError`, `Maneuver`, and `ManeuverConfig` are re-exported at the `paladin-battalion` crate root (so that the facade's `pub use paladin_battalion::maneuver::*` resolves correctly):

```rust
// In lib.rs, add these re-exports:
pub use maneuver::parser::{FlowExpression, FlowParseError, FlowParser};
pub use maneuver::{
    ErrorStrategy, ExecutionStatus, Maneuver, ManeuverConfig,
    ManeuverError, ManeuverResult, ManeuverExecutionService, OutputFormat,
};
```

### 4.9 `paladin-battalion` `Cargo.toml` — Dependency Check

After the move, `paladin-battalion/Cargo.toml` must be verified:

- The parser is dependency-free beyond what `paladin-battalion` already has (`serde`, `thiserror`). No new dependencies are expected.
- `paladin-core` remains a dependency of `paladin-battalion` (for other battalion types like `Formation`, `Phalanx`, `Campaign`, etc.).

### 4.10 `paladin-core` `Cargo.toml` — Dependency Check

After the move, verify that any dependencies in `paladin-core/Cargo.toml` that were only needed for the parser (e.g., `serde` derive for parser types) are still needed for other paladin-core types. If `serde` was only needed for `Maneuver`/`ManeuverConfig`, remove it from `paladin-core`; otherwise leave it.

---

## 5. Non-Goals (Out of Scope)

1. **No logic changes to the Maneuver DSL.** The lexer, parser, AST, and execution service behavior must remain identical. This is a structural move only.
2. **No new Flow DSL operators or syntax.** Adding capabilities to the DSL is a future feature, not part of this refactoring.
3. **No changes to `config.yml` or YAML configuration.** Maneuver configuration loading is unaffected.
4. **No relocation of other battalion patterns** (`Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`, `Conclave`, `Council`, `Grove`). Those domain types stay in `paladin-core`.
5. **No renaming of public types.** `FlowParser`, `FlowExpression`, `FlowParseError`, `Maneuver`, `ManeuverConfig`, `ManeuverExecutionService` — all names are preserved.
6. **No changes to `STABLE_API.md`.** All re-exported paths are backward-compatible; no API surface additions or removals.
7. **No changes to the existing workspace-level test files** in `tests/unit/`. They continue to compile unchanged via facade re-exports.
8. **No relocation of `commander.rs`.** It stays at `paladin-battalion/src/commander.rs` — only its internal import paths are updated (Requirement 4.7).

---

## 6. Design Considerations

### Before and After — Module Ownership

```
BEFORE:
┌─────────────────────────────────────────────────────────┐
│  paladin-core                                           │
│  platform/container/battalion/                          │
│    parser/   ← DSL lives here (used nowhere else)      │
│      mod.rs  lexer.rs  ast.rs  error.rs                │
│    maneuver.rs ← domain type lives here                 │
│    formation.rs  phalanx.rs  campaign.rs  ...          │
└─────────────────────────────────────────────────────────┘
         ↕ cross-crate imports
┌─────────────────────────────────────────────────────────┐
│  paladin-battalion                                      │
│    maneuver_service.rs  ← execution lives here          │
│    flow_visualizer.rs   ← visualization lives here      │
└─────────────────────────────────────────────────────────┘

AFTER:
┌─────────────────────────────────────────────────────────┐
│  paladin-core                                           │
│  platform/container/battalion/                          │
│    formation.rs  phalanx.rs  campaign.rs  ...          │
│    (no parser, no maneuver)                             │
└─────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│  paladin-battalion                                      │
│    maneuver/          ← entire Maneuver subsystem here  │
│      mod.rs           ← Maneuver + ManeuverConfig       │
│      parser/          ← DSL (lexer, AST, parser, error) │
│      service.rs       ← ManeuverExecutionService        │
│      visualizer.rs    ← FlowVisualizer                  │
│    campaign_service.rs  formation_service.rs  ...       │
└─────────────────────────────────────────────────────────┘
         ↕ facade re-exports maintain old paths
┌─────────────────────────────────────────────────────────┐
│  paladin (facade)                                       │
│    core::platform::container::battalion::parser::*      │
│      → paladin_battalion::maneuver::parser::*           │
│    core::platform::container::battalion::maneuver::*    │
│      → paladin_battalion::maneuver::*                   │
└─────────────────────────────────────────────────────────┘
```

### Inline Test Strategy

The codebase consistently uses `#[cfg(test)] mod tests { use super::*; ... }` blocks at the bottom of source files. Tests are not extracted into a separate `tests/` directory at the crate level. This means:

- When `parser/lexer.rs` moves, its 8 inline tests move with it automatically.
- When `maneuver.rs` content moves into `maneuver/mod.rs`, its 9 inline tests move with it.
- When `maneuver_service.rs` becomes `maneuver/service.rs`, its inline tests (count TBD — verify during Task 3.1; the file is 984 lines so tests are expected) move with it.
- When `flow_visualizer.rs` becomes `maneuver/visualizer.rs`, its 21 inline tests move with it.

After all moves, running `cargo test -p paladin-battalion` must show the full Maneuver test suite passing.

### Facade Re-Export — Avoiding a Circular Dependency

Adding re-exports from `paladin-core` back to `paladin-battalion` is not possible — it would create a circular dependency (`paladin-core` → `paladin-battalion` → `paladin-core`). The backward-compatible re-exports must be placed in the facade crate only, as described in Requirement 4.6. The facade depends on both `paladin-core` and `paladin-battalion` and has no circular constraint.

---

## 7. Technical Considerations

### 7.1 `use super::*` in Parser Tests

The parser's inline tests use `use super::*;` to import the types from the enclosing module. After the move, `super::*` refers to the new module location. No changes to import statements inside the `#[cfg(test)]` blocks are needed — `use super::*;` is relative and self-updating.

### 7.2 `paladin-core` Battalion `mod.rs` After Cleanup

After removing `pub mod parser;` and `pub mod maneuver;`, `paladin-core`'s battalion `mod.rs` will still compile. It retains all other battalion types: `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`, `Conclave`, `Council`, `Grove`, and shared types (`BattalionConfig`, `BattalionResult`, `BattalionStrategy`). None of those depend on the parser.

### 7.3 Facade `container` Re-Export — Impact on Other Consumers

The change to `src/core/platform/mod.rs` (Requirement 4.6) replaces a wholesale `pub use paladin_core::platform::container;` with an explicit `pub mod container` block. This is safe because the explicit block re-exports everything via `pub use paladin_core::platform::container::*;`, preserving all existing paths. The only addition is the injected `parser` and `maneuver` forwarding modules.

**Risk:** If any consumer outside the workspace imports `paladin::core::platform::container` as a module and calls methods on it as a module object (not through items), the change from `pub use` to `pub mod` could affect them. This is not a known pattern — modules are used for namespacing, not as values — but Task 3.1 must confirm no such usages exist.

### 7.4 `maneuver/mod.rs` Construction

The existing `maneuver.rs` from `paladin-core` becomes `maneuver/mod.rs` in `paladin-battalion`. When a file named `x.rs` becomes a module directory `x/mod.rs`, Rust treats it identically — the module content and all `use super::*` references inside `#[cfg(test)]` blocks remain valid. No changes to the file content are required beyond updating any cross-crate import paths (see Requirement 4.7).

### 7.5 Incremental Build Strategy

Each task must leave the workspace in a green build state:

```
Task 3.1: Read-only analysis — no build changes
Task 3.2: Move files + update internal paladin-battalion imports
          → cargo build -p paladin-battalion GREEN
          → cargo build -p paladin-core RED (expected: missing modules — fix in same task)
          → After paladin-core cleanup: cargo build --workspace GREEN
Task 3.3: Add facade re-exports
          → cargo build --workspace GREEN
          → cargo test --workspace GREEN
Task 3.4: Verification pass — confirm test counts, run clippy and fmt
```

---

## 8. Relevant Files

### Files to be moved (deleted from paladin-core, created in paladin-battalion)

| File | Action |
|---|---|
| `crates/paladin-core/src/platform/container/battalion/parser/mod.rs` | Delete (content moves to `paladin-battalion/src/maneuver/parser/mod.rs`) |
| `crates/paladin-core/src/platform/container/battalion/parser/lexer.rs` | Delete (content moves to `paladin-battalion/src/maneuver/parser/lexer.rs`) |
| `crates/paladin-core/src/platform/container/battalion/parser/ast.rs` | Delete (content moves to `paladin-battalion/src/maneuver/parser/ast.rs`) |
| `crates/paladin-core/src/platform/container/battalion/parser/error.rs` | Delete (content moves to `paladin-battalion/src/maneuver/parser/error.rs`) |
| `crates/paladin-core/src/platform/container/battalion/maneuver.rs` | Delete (content moves to `paladin-battalion/src/maneuver/mod.rs`) |

### Files to be renamed/relocated within `paladin-battalion`

| File | Action |
|---|---|
| `crates/paladin-battalion/src/maneuver_service.rs` | Rename to `crates/paladin-battalion/src/maneuver/service.rs` + update imports |
| `crates/paladin-battalion/src/flow_visualizer.rs` | Rename to `crates/paladin-battalion/src/maneuver/visualizer.rs` + update imports |

### New files to be created

| File | Purpose |
|---|---|
| `crates/paladin-battalion/src/maneuver/mod.rs` | Maneuver domain type + ManeuverConfig (from deleted maneuver.rs) + sub-module declarations |
| `crates/paladin-battalion/src/maneuver/parser/mod.rs` | FlowParser (from deleted parser/mod.rs) |
| `crates/paladin-battalion/src/maneuver/parser/lexer.rs` | Lexer, Token (from deleted lexer.rs) |
| `crates/paladin-battalion/src/maneuver/parser/ast.rs` | FlowExpression (from deleted ast.rs) |
| `crates/paladin-battalion/src/maneuver/parser/error.rs` | FlowParseError (from deleted error.rs) |

### Existing files to be modified

| File | Change |
|---|---|
| `crates/paladin-core/src/platform/container/battalion/mod.rs` | Remove `pub mod parser;` and `pub mod maneuver;` |
| `crates/paladin-battalion/src/lib.rs` | Remove `pub mod maneuver_service;` and `pub mod flow_visualizer;`; add `pub mod maneuver;` and crate-root re-exports |
| `crates/paladin-battalion/src/commander.rs` | Update import paths for Maneuver types |
| `src/core/platform/mod.rs` | Replace `pub use paladin_core::platform::container;` with explicit `pub mod container` that injects parser and maneuver forwarding modules |

### Reference files (read-only)

| File | Purpose |
|---|---|
| `tests/unit/parser_tests.rs` | 57 workspace-level parser tests — no changes needed |
| `tests/unit/maneuver_domain_tests.rs` | 21 workspace-level domain tests — no changes needed |
| `src/application/cli/commands/maneuver.rs` | Consumer — no changes needed (relies on facade re-exports) |
| `src/application/cli/commands/battalion.rs` | Consumer — no changes needed (relies on facade re-exports) |
| `src/application/cli/config/battalion_config.rs` | Consumer — no changes needed (relies on facade re-exports) |

---

## 9. Success Metrics

1. `cargo build -p paladin-core` succeeds with zero errors. `grep -r "parser\|maneuver" crates/paladin-core/src/platform/container/battalion/` returns no file paths.
2. `cargo build -p paladin-battalion` succeeds with zero errors.
3. `cargo build --workspace` succeeds with zero errors.
4. `cargo test -p paladin-battalion` passes all inline tests. The total inline test count in `paladin-battalion/src/maneuver/` matches the pre-move count from the parser and maneuver source files (35 inline tests minimum: 4 + 8 + 5 + 9 + 9 from paladin-core, plus 21 from `flow_visualizer.rs`).
5. `cargo test --workspace` passes — including the 57 workspace-level parser tests and 21 workspace-level maneuver domain tests, which require no modifications.
6. `cargo clippy --workspace -- -D warnings` produces zero warnings.
7. `cargo fmt --all -- --check` passes.
8. `cargo doc -p paladin-battalion --no-deps` produces clean output documenting the complete Maneuver subsystem under `paladin_battalion::maneuver`.
9. No file added or modified by this Epic exceeds 1,000 lines (verify with `wc -l`).

---

## 10. Task Breakdown

### Task 3.1 — Assess Cross-Crate Impact (Read-Only)

**Goal:** Produce a complete reference map before touching any code.

Steps:
1. Search the entire workspace for imports from `paladin_core::platform::container::battalion::parser` and `paladin_core::platform::container::battalion::maneuver`. List every file and its import lines.
2. Search for imports from `paladin::core::platform::container::battalion::parser` and `paladin::core::platform::container::battalion::maneuver` (facade-path consumers). List every file and its import lines.
3. Confirm that `paladin-core/tests/` does not contain any integration test files for parser or Maneuver functionality (this was verified at planning time — re-confirm before proceeding).
4. Confirm the inline `#[test]` count in `maneuver_service.rs` (the pre-planning finding was zero, but the file is 984 lines — re-verify with `grep -c "#\[test\]" crates/paladin-battalion/src/maneuver_service.rs`). Record the count for the success metric in Section 9.
5. Search `crates/paladin-battalion/src/commander.rs` for inline fully-qualified path expressions in function bodies — not just top-of-file `use` statements: `grep -n "paladin_core::platform::container::battalion" crates/paladin-battalion/src/commander.rs`. List every line number and path found. These must all be updated in Task 3.2 step 8.
6. Enumerate every type exported from `paladin::core::platform::container::battalion::maneuver` in the current codebase. Compare against the re-export list in Requirement 4.6 and note any types that are missing (e.g., `AgentResult`). Produce the final confirmed type list for Task 3.3.
7. Produce the final list of files that need import path updates inside `paladin-battalion` (for Task 3.2) and the final confirmed list of re-export types needed in the facade (for Task 3.3).

**Definition of done:** You can answer "which files change in Task 3.2?" and "what types need facade re-exports in Task 3.3?" without looking anything up.

---

### Task 3.2 — Move Parser and Maneuver Modules + Reorganize `paladin-battalion`

**Goal:** Physically relocate all files and complete the `maneuver/` sub-module structure.

Steps:
1. Create `crates/paladin-battalion/src/maneuver/` directory.
2. Create `crates/paladin-battalion/src/maneuver/parser/` directory.
3. Copy `paladin-core`'s `parser/mod.rs`, `lexer.rs`, `ast.rs`, `error.rs` to `paladin-battalion/src/maneuver/parser/` (same names).
4. Copy `paladin-core`'s `maneuver.rs` to `paladin-battalion/src/maneuver/mod.rs`. Add `pub mod parser;`, `pub mod service;`, `pub mod visualizer;` declarations at the top of `mod.rs`.
5. Rename `paladin-battalion/src/maneuver_service.rs` → `paladin-battalion/src/maneuver/service.rs`. Update its imports: replace `paladin_core::platform::container::battalion::maneuver::*` with `super::*` (or `crate::maneuver::*`) and `paladin_core::platform::container::battalion::parser::FlowExpression` with `super::parser::FlowExpression`.
6. Rename `paladin-battalion/src/flow_visualizer.rs` → `paladin-battalion/src/maneuver/visualizer.rs`. Update its import: replace `paladin_core::platform::container::battalion::parser::FlowExpression` with `super::parser::FlowExpression`.
7. Update `paladin-battalion/src/lib.rs`: remove `pub mod maneuver_service;` and `pub mod flow_visualizer;`; add `pub mod maneuver;`; add crate-root re-exports (Requirement 4.8).
8. Update `paladin-battalion/src/commander.rs` — both the `use` statements at the top of the file and all inline fully-qualified path expressions in function bodies identified during Task 3.1 step 5. Use the grep output from Task 3.1 to confirm every occurrence is addressed. Example: `paladin_core::platform::container::battalion::maneuver::ErrorStrategy::FailFast` in match arms must become `crate::maneuver::ErrorStrategy::FailFast`.
9. Delete the original files from `paladin-core`'s `parser/` directory and delete `maneuver.rs`.
10. Remove `pub mod parser;` and `pub mod maneuver;` from `crates/paladin-core/src/platform/container/battalion/mod.rs`.
11. Run `cargo build --workspace`. Fix any compilation errors.
12. Run `cargo test -p paladin-battalion -- --test-threads=4`. All inline tests must pass.

**Definition of done:** `cargo build --workspace` and `cargo test -p paladin-battalion` both succeed with zero errors.

---

### Task 3.3 — Add Facade Re-Exports

**Goal:** Restore backward-compatible `paladin::core::platform::container::battalion::parser::*` and `paladin::core::platform::container::battalion::maneuver::*` paths.

Steps:
1. Open `src/core/platform/mod.rs`.
2. Replace `pub use paladin_core::platform::container;` with the explicit `pub mod container` block described in Requirement 4.6.
3. Run `cargo build --workspace`. Fix any compilation errors.
4. Run `cargo test --workspace`. All tests must pass, including the 57 parser tests and 21 domain tests in `tests/unit/`.
5. Spot-check: confirm `grep -r "use paladin::core::platform::container::battalion::parser" tests/` still compiles without any file modifications.

**Definition of done:** `cargo test --workspace` passes with zero failures and zero test file modifications.

---

### Task 3.4 — Verification Pass

**Goal:** Confirm correctness, clean up, and close the Epic.

Steps:
1. Confirm inline test counts: run `grep -c "#\[test\]" crates/paladin-battalion/src/maneuver/**/*.rs` and verify total ≥ 35 (the pre-move count).
2. Confirm `crates/paladin-core/src/platform/container/battalion/` contains no `parser/` directory and no `maneuver.rs` file.
3. Run `cargo clippy --workspace -- -D warnings`. Address any warnings introduced by the move.
4. Run `cargo fmt --all`. Commit any formatting corrections.
5. Run `cargo doc -p paladin-battalion --no-deps`. Confirm the `maneuver` module and all its sub-modules are documented cleanly with no broken links.
6. Run `cargo build -p paladin-core` in isolation. Confirm it succeeds and that no parser-related code is compiled (check `cargo build -p paladin-core --message-format=json 2>&1 | grep parser` returns nothing).
7. Update `paladin-battalion/Cargo.toml` comments/description if needed to reflect the added Maneuver DSL ownership.
8. Mark all tasks complete and commit.

**Definition of done:** All success metrics in Section 9 are met.

---

## 11. Open Questions

1. **`use super::*` in `maneuver/mod.rs` tests:** When `maneuver.rs` becomes `maneuver/mod.rs`, inline tests using `use super::*` now import from the parent of `maneuver/` (i.e., `crate::` root of `paladin-battalion`). If any test in the original `maneuver.rs` relied on `use super::*` to import types from the `battalion/` module in `paladin-core`, those imports must be updated explicitly. Task 3.1 should check the test block in `maneuver.rs` for this pattern.

2. **`pub use paladin_core::platform::container::*` wildcard scope (resolved in PRD):** This conflict is pre-empted in Requirement 4.6. Using a wildcard at the `container` level would import the `paladin_core::platform::container::battalion` module name and then conflict with the local `pub mod battalion` declaration (duplicate definition error). The fix — specified in Requirement 4.6 — is to enumerate every non-battalion container sub-module explicitly and skip `battalion` at the container level. The complete list of sub-modules to enumerate must be confirmed during Task 3.3 by reading `paladin-core/src/platform/container/mod.rs`. No open action remains on this question beyond compiling that list.

3. **`ManeuverExecutionService` in `paladin-battalion` `lib.rs`:** Should `ManeuverExecutionService` be re-exported at the `paladin-battalion` crate root alongside the other types in Requirement 4.8? It is currently only consumed through `paladin-battalion` internally and via the facade. Check whether any consumers import `ManeuverExecutionService` directly and ensure the re-export list covers them.

4. **`in_memory_registry.rs` dependency:** This file exists in `paladin-battalion` — verify during Task 3.1 that it does not import from `paladin_core::platform::container::battalion::maneuver` or `parser`. If it does, add it to the import update list in Requirement 4.7.
