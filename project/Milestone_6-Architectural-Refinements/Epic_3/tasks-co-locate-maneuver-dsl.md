## Relevant Files

### New Files (created by this Epic)

- `crates/paladin-battalion/src/maneuver/mod.rs` — Maneuver DSL domain types; created from `paladin-core`'s `maneuver.rs` with `pub mod parser;` / `pub mod service;` / `pub mod visualizer;` declarations added
- `crates/paladin-battalion/src/maneuver/parser/mod.rs` — Flow parser entry point; relocated from `paladin-core`
- `crates/paladin-battalion/src/maneuver/parser/ast.rs` — Flow AST types; relocated from `paladin-core`
- `crates/paladin-battalion/src/maneuver/parser/lexer.rs` — Flow lexer; relocated from `paladin-core`
- `crates/paladin-battalion/src/maneuver/parser/error.rs` — Parser error types; relocated from `paladin-core`

### Renamed / Relocated Files

- `crates/paladin-battalion/src/maneuver/service.rs` — Maneuver execution service; renamed from `maneuver_service.rs` (git mv)
- `crates/paladin-battalion/src/maneuver/visualizer.rs` — Flow visualizer; renamed from `flow_visualizer.rs` (git mv)

### Modified Files

- `crates/paladin-battalion/src/lib.rs` — Remove `maneuver_service` / `flow_visualizer` pub mod entries; add `pub mod maneuver;`
- `crates/paladin-battalion/src/commander.rs` — Update `use` statements and inline FQPs in function bodies to use `paladin_battalion::maneuver` paths
- `crates/paladin-core/src/platform/container/battalion/mod.rs` — Remove `pub mod maneuver;` and `pub mod parser;` declarations
- `src/core/platform/mod.rs` — Replace wholesale `pub use paladin_core::platform::container;` with explicit per-sub-module re-exports that preserve all consumer import paths

### Deleted Files

- `crates/paladin-core/src/platform/container/battalion/maneuver.rs`
- `crates/paladin-core/src/platform/container/battalion/parser/mod.rs`
- `crates/paladin-core/src/platform/container/battalion/parser/ast.rs`
- `crates/paladin-core/src/platform/container/battalion/parser/lexer.rs`
- `crates/paladin-core/src/platform/container/battalion/parser/error.rs`

### Reference Files (unchanged — verified by facade re-exports)

- `tests/unit/parser_tests.rs` — 57 workspace-level parser tests; must continue to pass without modification
- `tests/unit/maneuver_domain_tests.rs` — 21 workspace-level maneuver tests; must continue to pass without modification
- `src/application/cli/commands/maneuver.rs` — Uses `crate::core::platform::container::battalion::parser::FlowParser`; resolved by facade
- `src/application/cli/commands/battalion.rs` — Uses `crate::core::platform::container::battalion::maneuver::Maneuver`; resolved by facade
- `src/application/cli/config/battalion_config.rs` — Uses `crate::core::platform::container::battalion::parser::FlowParser`; resolved by facade

---

### Notes

- Unit tests in Rust live **in the same file as the code they test** inside `#[cfg(test)] mod tests { use super::*; }`. Tests move automatically when their source file moves — no separate migration step is needed.
- Use `git mv` when relocating files within `paladin-battalion` to preserve git history.
- After Task 5.0, run `cargo build -p paladin-core` in isolation before `cargo build --workspace` to confirm the core crate no longer contains any parser or maneuver code.
- The facade (`src/core/platform/mod.rs`) currently uses a wholesale re-export: `pub use paladin_core::platform::container;`. This **cannot** stay as-is after Task 5.0 because `battalion::parser` and `battalion::maneuver` will no longer exist in paladin-core. Task 6.0 replaces it with explicit sub-module re-exports.
- `commander.rs` contains inline fully-qualified paths (FQPs) inside function bodies — not just `use` statements at the top. Task 1.6 must grep for these before Task 4.0 begins.
- The `paladin-core/src/platform/container/mod.rs` currently declares **36 sub-modules** (arsenal through workflow). All 36 must be explicitly re-exported in Task 6.0's new facade `pub mod container` block — the `battalion` entry is the one that changes source crate.
- `maneuver.rs` in paladin-core uses `crate::platform::container::battalion::parser::FlowExpression` and `FlowParseError` internally (2 refs at lines 6 and 303). After the move to `maneuver/mod.rs`, these become `super::parser::FlowExpression` / `super::parser::FlowParseError` (parser is now a direct sub-module of maneuver).

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

When all sub-tasks under a parent task are complete:
1. Run `cargo test` (or scoped variant noted in the sub-task)
2. Run `cargo fmt --check`
3. Run `cargo clippy`
4. Stage and commit if all checks pass
5. Mark the parent task `[x]`

---

## Tasks

- [x] 0.0 Confirm feature branch
  - [x] 0.1 Run `git branch --show-current` and confirm you are on `feature/milestone_6`
  - [x] 0.2 Run `git status` and confirm the working tree is clean (Epics 1 & 2 already committed)
  - [x] 0.3 checkout a new branch for this epic by running `git checkout -b feature/milestone_6-epic_3-co-locate-maneuver-dsl.md`

- [x] 1.0 Assess cross-crate impact (read-only analysis)
  - [x] 1.1 Run `grep -rn "paladin_core::platform::container::battalion::parser" crates/` to find all direct crate-path references to the parser module
  - [x] 1.2 Run `grep -rn "paladin_core::platform::container::battalion::maneuver" crates/` to find all direct crate-path references to the maneuver module
  - [x] 1.3 Run `grep -rn "crate::platform::container::battalion::parser\|crate::platform::container::battalion::maneuver" crates/paladin-core/src/` to find internal paladin-core cross-references
  - [x] 1.4 Count inline tests in `maneuver_service.rs`: `grep -c "#\[test\]" crates/paladin-battalion/src/maneuver_service.rs`
  - [x] 1.5 Run `grep -n "paladin_core::platform::container::battalion" crates/paladin-battalion/src/commander.rs` to find inline FQPs in function bodies (not just `use` statements)
  - [x] 1.6 Run `grep -n "^pub " crates/paladin-core/src/platform/container/battalion/maneuver.rs` to enumerate all public types that must appear in the facade re-export block
  - [x] 1.7 Run `grep -n "^pub mod" crates/paladin-core/src/platform/container/mod.rs` to confirm the full list of 27 container sub-modules for the Task 6.0 facade rewrite
  - [x] 1.8 Record the test counts from 1.4 in a comment at the bottom of this file for verification in Task 7.0

- [x] 2.0 Create `maneuver/` sub-module skeleton and move parser files
  - [x] 2.1 Create directory `crates/paladin-battalion/src/maneuver/parser/` (e.g., `mkdir -p`)
  - [x] 2.2 Copy `crates/paladin-core/src/platform/container/battalion/parser/ast.rs` → `crates/paladin-battalion/src/maneuver/parser/ast.rs`
  - [x] 2.3 Copy `crates/paladin-core/src/platform/container/battalion/parser/lexer.rs` → `crates/paladin-battalion/src/maneuver/parser/lexer.rs`
  - [x] 2.4 Copy `crates/paladin-core/src/platform/container/battalion/parser/error.rs` → `crates/paladin-battalion/src/maneuver/parser/error.rs`
  - [x] 2.5 Copy `crates/paladin-core/src/platform/container/battalion/parser/mod.rs` → `crates/paladin-battalion/src/maneuver/parser/mod.rs`
  - [x] 2.6 Copy `crates/paladin-core/src/platform/container/battalion/maneuver.rs` → `crates/paladin-battalion/src/maneuver/mod.rs`
  - [x] 2.7 Add `pub mod parser;` to the top of `crates/paladin-battalion/src/maneuver/mod.rs` (the parser sub-module now lives under maneuver)
  - [x] 2.8 Add `pub mod maneuver;` to `crates/paladin-battalion/src/lib.rs` (temporarily alongside the existing `maneuver_service` and `flow_visualizer` entries)
  - [x] 2.9 Run `cargo check -p paladin-battalion` — fix any compilation errors in the newly copied files before proceeding

- [ ] 3.0 Relocate `maneuver_service.rs` and `flow_visualizer.rs` into `maneuver/`
  - [ ] 3.1 Run `git mv crates/paladin-battalion/src/maneuver_service.rs crates/paladin-battalion/src/maneuver/service.rs`
  - [ ] 3.2 Run `git mv crates/paladin-battalion/src/flow_visualizer.rs crates/paladin-battalion/src/maneuver/visualizer.rs`
  - [ ] 3.3 Add `pub mod service;` and `pub mod visualizer;` to `crates/paladin-battalion/src/maneuver/mod.rs`
  - [ ] 3.4 Remove `pub mod maneuver_service;` and `pub mod flow_visualizer;` from `crates/paladin-battalion/src/lib.rs`
  - [ ] 3.5 Run `cargo check -p paladin-battalion` (compilation errors from broken import paths are expected here — proceed to Task 4.0)

- [ ] 4.0 Update all import paths within `paladin-battalion`
  - [ ] 4.1 Update `use` statements in `crates/paladin-battalion/src/maneuver/service.rs`: replace any `crate::maneuver_service` self-references and update imports of `Maneuver`, parser types, etc. to use `super::` or `crate::maneuver::`
  - [ ] 4.2 Update `use` statements in `crates/paladin-battalion/src/maneuver/visualizer.rs`: update any import paths that referenced the old flat module locations
  - [ ] 4.3 Update `use` statement imports at the top of `crates/paladin-battalion/src/commander.rs` to reference `crate::maneuver::` instead of `paladin_core::platform::container::battalion::maneuver::` and `paladin_core::platform::container::battalion::parser::`
  - [ ] 4.4 Update inline FQPs inside function bodies of `commander.rs` identified in Task 1.5 (e.g., `paladin_core::platform::container::battalion::maneuver::ErrorStrategy::FailFast` → `crate::maneuver::ErrorStrategy::FailFast`)
  - [ ] 4.5 Run `cargo build -p paladin-battalion` — resolve all remaining compilation errors
  - [ ] 4.6 Run `cargo test -p paladin-battalion` — all inline tests must pass before proceeding

- [ ] 5.0 Clean up `paladin-core`
  - [ ] 5.1 Remove `pub mod maneuver;` line from `crates/paladin-core/src/platform/container/battalion/mod.rs`
  - [ ] 5.2 Remove `pub mod parser;` line from `crates/paladin-core/src/platform/container/battalion/mod.rs`
  - [ ] 5.3 Delete `crates/paladin-core/src/platform/container/battalion/maneuver.rs`
  - [ ] 5.4 Delete `crates/paladin-core/src/platform/container/battalion/parser/error.rs`
  - [ ] 5.5 Delete `crates/paladin-core/src/platform/container/battalion/parser/lexer.rs`
  - [ ] 5.6 Delete `crates/paladin-core/src/platform/container/battalion/parser/ast.rs`
  - [ ] 5.7 Delete `crates/paladin-core/src/platform/container/battalion/parser/mod.rs`
  - [ ] 5.8 Run `cargo build -p paladin-core` in isolation to confirm the core crate compiles cleanly without parser or maneuver code
  - [ ] 5.9 Run `cargo build --workspace` — the facade (`src/core/platform/mod.rs`) will break here because it re-exports the now-removed `container::battalion::parser` and `container::battalion::maneuver`; this is expected and resolved in Task 6.0

- [ ] 6.0 Add facade backward-compatibility re-exports
  - [ ] 6.1 Open `src/core/platform/mod.rs` and read the current single-line re-export (`pub use paladin_core::platform::container;`)
  - [ ] 6.2 Replace the wholesale re-export with a `pub mod container { ... }` block that:
    - Re-exports each of the 27 non-battalion `paladin-core` container sub-modules explicitly with `pub use paladin_core::platform::container::<module>::*;`
    - Declares `pub mod battalion { pub use paladin_battalion::*; pub mod maneuver { pub use paladin_battalion::maneuver::*; pub mod parser { pub use paladin_battalion::maneuver::parser::*; } } }` to restore the `container::battalion::maneuver` and `container::battalion::parser` paths for all existing consumers
  - [ ] 6.3 Run `cargo build --workspace` — resolve any remaining compilation errors in the facade
  - [ ] 6.4 Run `cargo test --workspace` — all tests must pass
  - [ ] 6.5 Confirm `tests/unit/parser_tests.rs` and `tests/unit/maneuver_domain_tests.rs` pass without any source changes to those files

- [ ] 7.0 Verification pass and commit
  - [ ] 7.1 Run `grep -rc "#\[test\]" crates/paladin-battalion/src/maneuver/` and verify the total matches the sum of inline test counts from the source files (recorded in Task 1.8 plus Task 1.4)
  - [ ] 7.2 Confirm `crates/paladin-core/src/platform/container/battalion/parser/` directory no longer exists: `ls crates/paladin-core/src/platform/container/battalion/`
  - [ ] 7.3 Run `cargo clippy --workspace -- -D warnings` and fix any new warnings
  - [ ] 7.4 Run `cargo fmt --all` and verify no diffs (`cargo fmt --all --check`)
  - [ ] 7.5 Run `cargo doc -p paladin-battalion --no-deps` and verify no documentation build errors
  - [ ] 7.6 Run `cargo build -p paladin-core` one final time to confirm core compiles cleanly in isolation
  - [ ] 7.7 Stage all changes: `git add .`
  - [ ] 7.8 Commit with message: `git commit -m "feat(maneuver): co-locate Maneuver DSL with paladin-battalion" -m "- Move parser/ (ast, lexer, error, mod) from paladin-core to paladin-battalion/src/maneuver/parser/" -m "- Move maneuver.rs to paladin-battalion/src/maneuver/mod.rs" -m "- Rename maneuver_service.rs -> maneuver/service.rs, flow_visualizer.rs -> maneuver/visualizer.rs" -m "- Update commander.rs use statements and inline FQPs" -m "- Remove parser/ and maneuver.rs from paladin-core/src/platform/container/battalion/" -m "- Add facade re-exports in src/core/platform/mod.rs to preserve all consumer import paths" -m "Closes Epic 3 (Milestone 6)"`

---

<!-- Task 1.8 scratch space — verified during Task 1.0 -->
<!-- maneuver_service.rs: 0 #[test] / 9 #[tokio::test] = 9 async tests -->
<!-- parser/mod.rs: 4 | lexer.rs: 8 | error.rs: 5 | ast.rs: 9 | maneuver/mod.rs (was maneuver.rs): 9 | visualizer.rs (was flow_visualizer.rs): 21 | service.rs (was maneuver_service.rs): 9 -->
<!-- TOTAL expected in maneuver/: 4+8+5+9+9+21+9 = 65 tests -->
<!-- commander.rs FQP lines needing update: 172, 739, 757, 759, 760, 761, 763, 773, 812-817, 1269, 1404, 1432, 2621, 2631, 2643 -->
<!-- maneuver/ public types: ErrorStrategy, OutputFormat, ManeuverConfig, Maneuver, ManeuverResult, ExecutionStatus, ManeuverError -->
<!-- container sub-modules: 36 total (arsenal, autonomous_config, battalion, citadel, citadel_error, comment, content, content_list, document, execution_result, garrison, garrison_error, handoff, herald, herald_error, job, log, notification, orchestration_context, paladin, paladin_config, paladin_error, planning, prompt, queue_config, queue_item, registry_error, sanctum, schedule, task, token_usage, trigger, user, user_group, vision, workflow) -->
