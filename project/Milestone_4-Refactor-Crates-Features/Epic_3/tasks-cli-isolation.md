# Task List: CLI Isolation from Library Compilation Path

**Epic:** Epic 3 - CLI Isolation
**Milestone:** Tier 1 - High-Value, Low-Risk Improvements
**PRD:** `prd-cli-isolation.md`
**Created:** 2026-04-20
**Status:** Planning

---

## Relevant Files

- `Cargo.toml` - Add the `cli` feature, mark CLI-only dependencies as optional, and gate binaries/tests with `required-features` where appropriate.
- `src/application/mod.rs` - Gate the `application::cli` module at the application-layer boundary.
- `src/application/cli/mod.rs` - Confirm the CLI module remains internally coherent once feature-gated.
- `src/lib.rs` - Conditionally re-export CLI types or remove unconditional CLI exposure from the root API.
- `src/bin/paladin-cli.rs` - Primary CLI binary entry point that should compile only when the `cli` feature is enabled.
- `src/main.rs` - Existing `paladin` binary entry point; requires an architecture decision on whether it remains independent, wraps the CLI, or is feature-gated.
- `tests/cli/mod.rs` - Top-level CLI integration/snapshot test entry point that should run only with the `cli` feature.
- `src/application/cli/formatters/tests.rs` - In-module CLI formatter tests that should not compile in library-only builds.
- `src/application/cli/interactive/tests.rs` - In-module interactive CLI tests that should be guarded behind the `cli` feature.
- `tests/library_only_cli_isolation_test.rs` - Potential new integration test file to verify library-only usage remains functional without CLI support.
- `.github/workflows/feature-flags.yml` - Existing feature matrix workflow to extend with CLI-enabled and library-only checks.
- `.github/workflows/ci.yml` - Main CI workflow if broader build/test coverage needs to include CLI isolation checks.
- `README.md` - Update build and feature-flag documentation for CLI usage versus library usage.
- `CONTRIBUTING.md` - Document required commands for CLI-enabled tests and library-only verification.
- `CHANGELOG.md` - Record the CLI isolation change and any migration implications.
- `docs/MIGRATION.md` - Document any required downstream changes if CLI exports or defaults change.

### Notes

- Unit tests in Rust are typically placed in the same file as the code they test, inside a `#[cfg(test)]` module.
- Integration tests go in the `tests/` directory at the project root.
- Use `cargo test` to run all tests, or `cargo test test_name` for specific tests.
- Run `cargo test --features cli` to test CLI-specific functionality.
- Run `cargo test --lib --no-default-features` to test library-only builds.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

**Completion Protocol (Rust Projects):**
1. When you finish a **sub-task**, immediately mark it as completed by changing `[ ]` to `[x]`.
2. If **all** subtasks underneath a parent task are now `[x]`, follow this sequence:
   - **First**: Run the full test suite with `cargo test`
   - **Check formatting**: Run `cargo fmt --check` to ensure code follows Rust style guidelines
   - **Run linter**: Run `cargo clippy` and address any warnings
   - **Only if all tests pass and checks succeed**: Stage changes (`git add .`)
   - **Clean up**: Remove any temporary files, debug prints (`dbg!`, `println!`), and temporary code before committing
   - **Commit**: Use a descriptive commit message with conventional commit format
   - Mark the **parent task** as completed

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [ ] 0.0 Create feature branch for Epic 3
   - [ ] 0.1 Confirm whether to continue on the current branch or create a dedicated Epic 3 branch following repo workflow.
   - [ ] 0.2 If needed, create and check out a branch such as `feature/milestone_4-epic_3-cli-isolation`.
- [ ] 1.0 Analyze CLI dependency boundaries and create classification matrix
   - [ ] 1.1 Audit `Cargo.toml` for CLI-adjacent crates (`clap`, `dialoguer`, `indicatif`, `comfy-table`, `console`, `serde_yaml`, `colored`) and classify each as CLI-only, shared, or core.
   - [ ] 1.2 Verify actual source usage across `src/application/cli`, binaries, and non-CLI modules so shared crates are not incorrectly gated.
   - [ ] 1.3 Identify every current entry point that exposes or depends on `application::cli`, including `src/application/mod.rs`, `src/lib.rs`, and `src/bin/paladin-cli.rs`.
   - [ ] 1.4 Capture a dependency matrix and recommended gating approach in the task notes, PR description, or linked implementation notes before changing code.
- [ ] 2.0 Resolve critical architecture decisions
   - [ ] 2.1 Decide how `src/main.rs` and `src/bin/paladin-cli.rs` should coexist after CLI isolation.
   - [ ] 2.2 Confirm whether the `cli` feature remains out of the default feature set.
   - [ ] 2.3 Confirm the naming and placement for new library-only regression tests under `tests/`.
   - [ ] 2.4 Record the approved decisions in the PRD, task list, or implementation notes so follow-on work is unambiguous.
- [ ] 3.0 Gate CLI module behind feature flag
   - [ ] 3.1 Add a `cli` feature to `Cargo.toml` and wire in only the dependencies confirmed to be CLI-exclusive.
   - [ ] 3.2 Add `optional = true` to CLI-only dependencies and ensure shared dependencies remain available to non-CLI code.
   - [ ] 3.3 Gate the `application::cli` module in `src/application/mod.rs` using `#[cfg(feature = "cli")]`.
   - [ ] 3.4 Update `src/lib.rs` so CLI exports are conditional and the stable root API no longer exposes CLI items unconditionally.
   - [ ] 3.5 Resolve compile errors caused by gated imports, docs, or module references.
- [ ] 4.0 Update binary entry points for CLI feature
   - [ ] 4.1 Update `Cargo.toml` binary metadata if `required-features = ["cli"]` is needed for `paladin-cli` and possibly `paladin`.
   - [ ] 4.2 Refactor `src/bin/paladin-cli.rs` so it builds cleanly only when the `cli` feature is enabled.
   - [ ] 4.3 Update `src/main.rs` according to the architecture decision from Task 2.0.
   - [ ] 4.4 Verify `cargo build --bin paladin` and `cargo build --bin paladin-cli --features cli` behave as intended.
- [ ] 5.0 Migrate CLI tests to feature-gated compilation
   - [ ] 5.1 Gate `tests/cli/mod.rs` and any CLI-only test targets so they compile only with the `cli` feature.
   - [ ] 5.2 Gate in-module CLI tests such as `src/application/cli/formatters/tests.rs` and `src/application/cli/interactive/tests.rs`.
   - [ ] 5.3 Verify snapshot and color-control helpers still produce deterministic output when CLI tests run with the feature enabled.
   - [ ] 5.4 Confirm library-only test runs do not compile or execute CLI-specific tests.
- [ ] 6.0 Add library-only integration tests
   - [ ] 6.1 Create a new regression test file under `tests/` dedicated to library-only CLI isolation behavior.
   - [ ] 6.2 Add tests that exercise core library workflows without importing or depending on `application::cli`.
   - [ ] 6.3 Cover at least one Paladin builder/execution path and one Battalion-related path that should remain available without the `cli` feature.
   - [ ] 6.4 Add assertions or build-oriented checks that would fail if CLI-only paths leak back into the library build.
- [ ] 7.0 Update CI pipeline for feature matrix testing
   - [ ] 7.1 Extend `.github/workflows/feature-flags.yml` to include explicit CLI-disabled library checks.
   - [ ] 7.2 Add CLI-enabled binary and test jobs, including `cargo build --bin paladin-cli --features cli` and `cargo test --features cli`.
   - [ ] 7.3 Add or update CI steps that run the new library-only regression tests.
   - [ ] 7.4 Update any main CI workflow entries that also need to understand the new `cli` feature boundary.
- [ ] 8.0 Measure and document build time impact
   - [ ] 8.1 Capture baseline clean and incremental build timings for the current library build path.
   - [ ] 8.2 Capture the same measurements after CLI isolation for library-only builds.
   - [ ] 8.3 Compare dependency trees before and after using `cargo tree` or equivalent commands.
   - [ ] 8.4 Record the measured impact in the implementation notes, milestone report, or PR summary.
- [ ] 9.0 Update project documentation
   - [ ] 9.1 Update `README.md` with clear instructions for library consumers versus CLI users.
   - [ ] 9.2 Update `CONTRIBUTING.md` with the expected test/build commands for CLI-enabled and library-only validation.
   - [ ] 9.3 Add a `CHANGELOG.md` entry describing the CLI isolation change and any compatibility impact.
   - [ ] 9.4 Update `docs/MIGRATION.md` if downstream consumers need to explicitly enable `cli` or stop relying on CLI exports from the library root.
- [ ] 10.0 Final verification and quality gates
   - [ ] 10.1 Run `cargo build --lib --no-default-features` to verify the CLI is excluded from the library-only compilation path.
   - [ ] 10.2 Run `cargo build --bin paladin --features cli` and `cargo build --bin paladin-cli --features cli` as applicable to the chosen architecture.
   - [ ] 10.3 Run `cargo test` and `cargo test --features cli` to verify both standard and CLI-enabled test surfaces.
   - [ ] 10.4 Run `cargo fmt --check` and `cargo clippy -- -D warnings` and fix any issues introduced by the feature gating.
   - [ ] 10.5 Update this task list as work completes, including any additional relevant files discovered during implementation.

---

**Phase 2 Complete:** Detailed sub-tasks generated based on the Epic 3 PRD and the current repository structure.
