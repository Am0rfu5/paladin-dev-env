## Relevant Files

### New Files
- `src/infrastructure/resilience/mod.rs` — New module scaffold; declares `pub mod circuit_breaker` and documents the resilience boundary for future additions.
- `src/infrastructure/resilience/circuit_breaker.rs` — Moved from `src/application/use_cases/paladin/circuit_breaker.rs`; contains `CircuitBreaker` and `CircuitState` with updated rustdoc paths.

### Modified Files
- `src/infrastructure/mod.rs` — Add `pub mod resilience;` declaration.
- `src/application/use_cases/paladin/mod.rs` — Remove `pub mod circuit_breaker;`.
- `src/application/use_cases/paladin/paladin_execution_service.rs` — Update `CircuitBreaker` import and inline rustdoc examples.
- `src/lib.rs` — Verify/update public visibility of `infrastructure` module.
- `examples/basic_paladin.rs` — Update `CircuitBreaker` import.
- `examples/agent_handoffs.rs` — Update `CircuitBreaker` import.
- `examples/autonomous_full_config.rs` — Update `CircuitBreaker` import.
- `examples/autonomous_planning.rs` — Update `CircuitBreaker` import.
- `examples/autonomous_prompt_generation.rs` — Update `CircuitBreaker` import.
- `examples/battalion_checkpoint_recovery.rs` — Update `CircuitBreaker` import.
- `examples/citadel_autosave.rs` — Update `CircuitBreaker` import.
- `examples/citadel_restore.rs` — Update `CircuitBreaker` import.
- `examples/dynamic_temperature.rs` — Update `CircuitBreaker` import.
- `examples/herald_custom_formatter.rs` — Update `CircuitBreaker` import.
- `examples/herald_json_output.rs` — Update `CircuitBreaker` import.
- `examples/herald_markdown_output.rs` — Update `CircuitBreaker` import.
- `examples/paladin_with_config.rs` — Update `CircuitBreaker` import.
- `examples/vision_analysis.rs` — Update `CircuitBreaker` import.
- `examples/vision_battalion.rs` — Update `CircuitBreaker` import.
- `tests/cli/paladin_execution_test.rs` — Update `CircuitBreaker` import.
- `tests/cli/tool_integration_test.rs` — Update `CircuitBreaker` import.
- `tests/cli/error_handling_test.rs` — Update `CircuitBreaker` import.
- `README.md` — Update code examples referencing old `CircuitBreaker` import path.
- `STABLE_API.md` — Remove old path entry; add new canonical path entry.
- `final-api.txt` — Regenerate to reflect new module path.
- `api_surface_current.txt` — Regenerate to reflect new module path.

### Deleted Files
- `src/application/use_cases/paladin/circuit_breaker.rs` — Removed after successful relocation and verification.

### Notes

- All changes happen within the `paladin` facade crate. No workspace sub-crate (`paladin-core`, `paladin-battalion`, etc.) is modified.
- The internal import `use crate::application::use_cases::paladin::error::PaladinError;` inside `circuit_breaker.rs` **does not change** — it remains valid after the file moves within the same crate.
- The old path `paladin::application::use_cases::paladin::circuit_breaker` is **intentionally retired** — no `pub use` re-export is added.
- Run `cargo check` after each structural change (Tasks 1–3) before bulk-updating consumers. This surfaces compile errors early.
- Run tests with `cargo test` to verify all existing behavior is preserved.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/mileston_6-epic4-relocate-circuitbreaker`

- [x] 1.0 Create the `infrastructure/resilience` module scaffold
  - [x] 1.1 Read `src/infrastructure/mod.rs` to understand the existing module declaration order and doc structure.
  - [x] 1.2 Create `src/infrastructure/resilience/mod.rs` with: (a) a module-level `//!` doc comment explaining this is the canonical home for resilience primitives, (b) `pub mod circuit_breaker;`, and (c) a comment block listing planned future additions (`retry`, `rate_limiter`, `bulkhead`). Use the exact template provided in PRD §6.
  - [x] 1.3 Add `pub mod resilience;` to `src/infrastructure/mod.rs` in alphabetical order among the existing `pub mod` declarations.
  - [x] 1.4 Run `cargo check` to confirm the new empty scaffold compiles cleanly before adding any content.

- [x] 2.0 Relocate `circuit_breaker.rs` to the infrastructure resilience layer
  - [x] 2.1 Copy `src/application/use_cases/paladin/circuit_breaker.rs` to `src/infrastructure/resilience/circuit_breaker.rs` (keep the original in place for now — do not delete yet).
  - [x] 2.2 In the copied file, update every `use paladin::application::use_cases::paladin::circuit_breaker::` path in the `//!` module-level doc block examples to `use paladin::infrastructure::resilience::circuit_breaker::`.
  - [x] 2.3 In the copied file, update every `use paladin::application::use_cases::paladin::circuit_breaker::` path in all `///` method-level doc examples to `use paladin::infrastructure::resilience::circuit_breaker::`.
  - [x] 2.4 Verify that `use crate::application::use_cases::paladin::error::PaladinError;` at the top of the moved file is **unchanged** — this import is still valid from the new location within the same crate.
  - [x] 2.5 Run `cargo check` to confirm the new `infrastructure::resilience::circuit_breaker` module compiles, including that `PaladinError` resolves correctly from the new location.

- [x] 3.0 Update `PaladinExecutionService` imports and inline rustdoc examples
  - [x] 3.1 In `src/application/use_cases/paladin/paladin_execution_service.rs`, change: `use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` → `use crate::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - [x] 3.2 In the same file, update every inline `///` doc example that contains `use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` to `use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;`.
  - [x] 3.3 Run `cargo check` to confirm `PaladinExecutionService` compiles with the updated import.

- [x] 4.0 Update all example files (15 files) to use the new import path
  - [x] 4.1 In `examples/basic_paladin.rs`: replace `use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` with `use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - [x] 4.2 In `examples/agent_handoffs.rs`: replace the old import with the new path.
  - [x] 4.3 In `examples/autonomous_full_config.rs`: replace the old import with the new path.
  - [x] 4.4 In `examples/autonomous_planning.rs`: replace the old import with the new path.
  - [x] 4.5 In `examples/autonomous_prompt_generation.rs`: replace the old import with the new path.
  - [x] 4.6 In `examples/battalion_checkpoint_recovery.rs`: replace the old import with the new path.
  - [x] 4.7 In `examples/citadel_autosave.rs`: replace the old import with the new path.
  - [x] 4.8 In `examples/citadel_restore.rs`: replace the old import with the new path.
  - [x] 4.9 In `examples/dynamic_temperature.rs`: replace the old import with the new path.
  - [x] 4.10 In `examples/herald_custom_formatter.rs`: replace the old import with the new path.
  - [x] 4.11 In `examples/herald_json_output.rs`: replace the old import with the new path.
  - [x] 4.12 In `examples/herald_markdown_output.rs`: replace the old import with the new path.
  - [x] 4.13 In `examples/paladin_with_config.rs`: replace the old import with the new path.
  - [x] 4.14 In `examples/vision_analysis.rs`: replace the old import with the new path.
  - [x] 4.15 In `examples/vision_battalion.rs`: replace the old import with the new path.
  - [x] 4.16 Run `cargo check` to confirm all 15 example files compile with the new path.

- [x] 5.0 Update all test files (3 files) to use the new import path
  - [x] 5.1 In `tests/cli/paladin_execution_test.rs`: replace `use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;` with `use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;`
  - [x] 5.2 In `tests/cli/tool_integration_test.rs`: replace the old import with the new path.
  - [x] 5.3 In `tests/cli/error_handling_test.rs`: replace the old import with the new path.
  - [x] 5.4 Also updated: `tests/integration/paladin_garrison_integration_test.rs`, `tests/integration/paladin_integration_test.rs`, `tests/integration/context_injection_test.rs`, `tests/integration/herald_integration_test.rs`, `tests/unit/paladin_execution_service_test.rs`, `tests/unit/circuit_breaker_test.rs`, `tests/functional/paladin_tool_invocation_test.rs`, `tests/cli/formation_execution_test.rs`, `tests/cli/phalanx_execution_test.rs`, `tests/helpers/mock_paladin_port.rs` (11 total, not 3).
  - [x] 5.5 Run `cargo test` to confirm all tests that use `CircuitBreaker` pass. Result: 1200+ tests passed, 0 failed.

- [x] 6.0 Remove the old module registration and verify the old path is retired
  - [x] 6.1 Remove the `pub mod circuit_breaker;` line from `src/application/use_cases/paladin/mod.rs`.
  - [x] 6.2 Delete `src/application/use_cases/paladin/circuit_breaker.rs`.
  - [x] 6.3 Run `cargo build --workspace` to confirm the workspace builds cleanly with the old file gone.
  - [x] 6.4 Confirm no remaining references to the old path: run `grep -r "application::use_cases::paladin::circuit_breaker" src/ tests/ examples/` and verify the output is empty. (Also fixed `crate::` form refs in `src/application/cli/commands/agent.rs` and `battalion.rs`.)

- [x] 7.0 Update documentation and stable API surface
  - [x] 7.1 In `README.md`, updated the code example: split the combined `use` into separate imports, pulling `CircuitBreaker` from `paladin::infrastructure::resilience::circuit_breaker`.
  - [x] 7.2 In `STABLE_API.md`, added a new "Resilience Types" section after "Base Types" with stable entries for `CircuitBreaker` and `CircuitState` at their new canonical path, including a migration note.
  - [x] 7.3 Confirmed `pub mod infrastructure;` is present in `src/lib.rs` — no visibility change needed.
  - [x] 7.4 Updated `final-api.txt` (36 entries) and `api_surface_current.txt` (96 entries) via sed: old path replaced with `paladin::infrastructure::resilience::circuit_breaker` throughout.

- [x] 8.0 Verify build, tests, lint, and formatting pass
  - [x] 8.1 `cargo build --workspace` — ✅ Finished with zero errors
  - [x] 8.2 `cargo test` — ✅ 1762 passed; 0 failed across 12 test suites
  - [x] 8.3 `cargo test --doc` — ✅ 105 passed; 0 failed
  - [x] 8.4 `cargo clippy --workspace -- -D warnings` — ✅ Finished with zero warnings
  - [x] 8.5 `cargo fmt --all -- --check` — applied `cargo fmt --all` to sort imports, then check passed clean
  - [x] 8.6 `cargo doc --workspace --no-deps` — ✅ Finished with zero errors
  - [x] 8.7 Staged all changes and committed: `refactor(infra): relocate CircuitBreaker to infrastructure/resilience layer` (commit `abf4a76`, 41 files changed, 724 insertions, 181 deletions)
