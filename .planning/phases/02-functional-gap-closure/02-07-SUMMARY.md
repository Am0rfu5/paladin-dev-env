---
phase: 02-functional-gap-closure
plan: 07
subsystem: testing
tags: [cli-tests, mock-llm-adapter, formation, phalanx, tool-integration, path-attribute-shim]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-01's measured cargo test --workspace baseline (2790 passed / 0 failed / 126
      ignored on commit 7e55655) — the pre-change tree this plan's own full-suite run is
      compared against"
provides:
  - "The `cli` `[[test]]` target compiling and running for the first time with its five
    execution/error/tool suites declared — 1,895 lines of already-written test source that had
    been commented out of `tests/cli/mod.rs` since the helper module it imports was never
    created"
  - "Epic 9 tasks 13.4, 13.5 and 13.6 each closed by a named, passing CLI-level exerciser: a
    Paladin run from config through a mock LLM adapter, a Formation run with multiple mock
    Paladins, and a Phalanx run with parallel execution"
  - "tests/cli/helpers.rs — a thin path-attribute shim re-exporting the existing
    tests/helpers/ mock barrel into the `cli` test target's own module tree, with no mock
    redefined"
  - "A corrected finding: the Phase 1 ledger's 'the test itself was never written' verdict for
    Epic 9 tasks 13.4-13.6 is contradicted by the tree — the tests existed, complete and
    unmodified from years ago, and needed only the missing helpers module to compile and pass"
affects: [02-09-amend-ledger]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Path-attribute module shim to bridge a `[[test]]` target that cannot see a sibling
      tests/helpers/ directory: `#[path = \"../helpers/mod.rs\"] mod shared;` inside a new
      tests/cli/helpers.rs, re-exporting only the names the reactivated suites import
      (MockLlmAdapter, MockPaladinPort, MockArsenalPort, create_mock_with_responses) rather than
      forking the barrel."
    - "Crate-local `#[allow(dead_code, unused_imports)]` on a re-loaded shared module rather than
      trimming or forking the shared file: the same tests/helpers/mod.rs barrel is compiled as
      part of multiple independent test-binary crates, and each crate only exercises the subset
      of the barrel its own suites call — the unused subset differs per crate, so the allow is
      scoped to the shim's `mod shared;` declaration, not the shared file itself."

key-files:
  created:
    - tests/cli/helpers.rs
  modified:
    - tests/cli/mod.rs
    - tests/cli/error_handling_test.rs
    - tests/cli/formation_execution_test.rs
    - tests/cli/paladin_execution_test.rs
    - tests/cli/phalanx_execution_test.rs
    - tests/cli/tool_integration_test.rs
    - tests/lib.rs

key-decisions:
  - "Split the plan's two tasks into two commits in principle, but Task 2 required no code
    changes: all 37 tests across the five reactivated suites passed on the first run against the
    current tree, with zero repairs needed. Only Task 1's commit (aa3f2f5) carries a diff; Task 2
    is recorded as a verification-only step with no commit of its own."
  - "[Rule 3 - blocking, outside declared files_modified] Removed the dead
    `#[cfg(feature = \"cli\")] pub mod cli;` from tests/lib.rs. Nothing in the tree ever
    referenced `crate::cli::…` through it — it only re-compiled the whole tests/cli/ tree a
    second time inside the auto-discovered `lib` test binary (tests/lib.rs has no explicit
    `[[test]]` entry in Cargo.toml). Once tests/cli/helpers.rs began re-loading
    tests/helpers/mod.rs via a #[path] attribute, that redundant nested compilation caused the
    same physical file to be loaded twice within one crate ('lib'), which
    `clippy::duplicate_mod` rejects under `-D warnings`. Verified no other file references
    `crate::cli::` before removing it."
  - "Chose a separate tests/cli/helpers.rs file over inlining the `#[path]` attribute directly on
    a `mod helpers;` item in tests/cli/mod.rs, to satisfy the plan's acceptance criteria that
    `tests/cli/mod.rs` contain a literal, non-path `mod helpers;` line and that the shim's
    re-export-not-redefine property be independently greppable in its own file."

patterns-established:
  - "Path-attribute shim for a `[[test]]` target needing a sibling directory's helpers, applied
    to any future `[[test]]` target with the same `tests/<name>/mod.rs` root shape that cannot
    see tests/helpers/ automatically."

requirements-completed: [GAP-01, GAP-02]

coverage:
  - id: D1
    description: "tests/cli/helpers.rs exists as a re-export shim (no MockLlmAdapter,
      MockPaladinPort or MockArsenalPort redefined), tests/cli/mod.rs declares `mod helpers;`
      once and uncomments exactly the five in-scope suites, leaving the four D-09-out-of-scope
      suites commented with an updated boundary note"
    verification:
      - kind: other
        ref: "grep -cE 'struct Mock|impl .*Port for' tests/cli/helpers.rs -> 0; grep -c 'mod
          helpers;' tests/cli/mod.rs -> 1; grep -cE '^mod
          (error_handling_test|formation_execution_test|paladin_execution_test|
          phalanx_execution_test|tool_integration_test);' tests/cli/mod.rs -> 5; grep -cE '^mod
          (arsenal_config_test|environment_tests|garrison_config_test|integration_tests);'
          tests/cli/mod.rs -> 0"
        status: pass
      - kind: unit
        ref: "cargo build --tests --features cli; cargo test --features cli --test cli --no-run"
        status: pass
    human_judgment: false
  - id: D2
    description: "The cli test target compiles and all five reactivated suites pass: 37 tests
      (14 error_handling_test + 4 formation_execution_test + 6 paladin_execution_test + 5
      phalanx_execution_test + 8 tool_integration_test), 0 failed, 0 ignored, 0 removed"
    requirement: "GAP-01"
    verification:
      - kind: unit
        ref: "cargo test --features cli --test cli -> 99 passed; 0 failed; 0 ignored (37 from the
          five reactivated suites, 43 from the four pre-existing snapshot suites, 19 from
          tests/helpers/'s own #[cfg(test)] modules pulled in by the shim)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Epic 9 tasks 13.4 (Paladin from config via mock LLM), 13.5 (Formation with
      multiple mock Paladins) and 13.6 (Phalanx with parallel execution) each have a named
      passing exerciser"
    requirement: "GAP-02"
    verification:
      - kind: unit
        ref: "cargo test --features cli --test cli -- paladin_execution_test::test_paladin_basic_execution
          (13.4); cargo test --features cli --test cli --
          formation_execution_test::test_formation_basic_sequential_execution (13.5); cargo test
          --features cli --test cli -- phalanx_execution_test::test_phalanx_basic_parallel_execution
          (13.6)"
        status: pass
    human_judgment: false
  - id: D4
    description: "No test was silenced with #[ignore]; cargo test --workspace and cargo clippy
      --workspace --all-targets --features cli -- -D warnings and cargo fmt --all -- --check all
      stay green; no .github/workflows/ file touched"
    verification:
      - kind: other
        ref: "git diff -U0 aa3f2f5~1 aa3f2f5 -- tests/cli/ | grep '^+' | grep -c '#\\[ignore' -> 0;
          git diff --name-only 4f940d55..HEAD | grep -c '^\\.github/' -> 0"
        status: pass
      - kind: unit
        ref: "cargo test --workspace -> 0 failed across all groups (default feature set)"
        status: pass
      - kind: other
        ref: "cargo clippy --workspace --all-targets --features cli -- -D warnings; cargo fmt
          --all -- --check"
        status: pass
    human_judgment: false

duration: ~50min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 07: CLI Test Target Reactivation Summary

**Wired the never-compiled `tests/cli/` execution/error/tool suites into the `cli` `[[test]]` target for the first time via a path-attribute helper shim, and corrected the Phase 1 ledger's "the test itself was never written" finding for Epic 9 tasks 13.4-13.6 — all 37 reactivated tests passed on the first run with zero repairs needed**

## Performance

- **Duration:** ~50 min
- **Started:** ~2026-08-01T01:28:00Z (approximate — base commit `4f940d55f4136264f6febc37cf105f0163d54264`)
- **Completed:** 2026-08-01T02:18:53Z
- **Tasks:** 2 (both `type="auto"`; Task 2 required no code changes)
- **Files modified:** 7 modified, 1 created

## Accomplishments

- Created `tests/cli/helpers.rs`, a thin path-attribute re-export
  (`#[path = "../helpers/mod.rs"] mod shared;`) that makes the existing `tests/helpers/` mock
  barrel (`MockLlmAdapter`, `MockPaladinPort`, `MockArsenalPort`, `create_mock_with_responses`)
  visible inside the `cli` `[[test]]` target's own module tree, without redefining any mock.
- Declared `mod helpers;` in `tests/cli/mod.rs` and uncommented exactly the five in-scope suites
  named by D-09 (`error_handling_test`, `formation_execution_test`, `paladin_execution_test`,
  `phalanx_execution_test`, `tool_integration_test`), leaving the four out-of-scope suites
  (`arsenal_config_test`, `environment_tests`, `garrison_config_test`, `integration_tests`)
  commented, with the barrel's stale note ("missing helpers module") replaced by a statement of
  the D-09 scope boundary and a pointer to plan 02-09's D-12 sweep.
- `cargo build --tests --features cli` succeeded with **zero repairs needed** in any of the five
  reactivated test files themselves — their construction-API calls (`PaladinData`,
  `BattalionConfig::new`, `MockLlmAdapter`, `MockPaladinPort::new`, `MockArsenalPort`) all matched
  the current shapes exactly. The only compile-time work was in the new shim file and the barrel
  edit.
- Discovered and fixed two lint-only consequences of pulling the shared `tests/helpers/` tree into
  the `cli` test crate: crate-local `dead_code` (items the five reactivated suites don't
  individually call, though the `unit`/`lib` targets do) and `unused_imports` (re-export names
  this shim doesn't forward), both suppressed with scoped, commented `#[allow(...)]` on the
  shim's `mod shared;` declaration — `tests/helpers/mod.rs` itself stays byte-for-byte unchanged.
- Discovered and fixed a genuine blocking compile error the plan's research did not anticipate:
  `tests/lib.rs`'s pre-existing, unreferenced `#[cfg(feature = "cli")] pub mod cli;` caused
  `tests/helpers/mod.rs` to be loaded twice within the single auto-discovered `lib` test binary
  crate (once via `lib.rs`'s own top-level `pub mod helpers;`, once via the new shim nested under
  the re-compiled `tests/cli/` tree) — `clippy::duplicate_mod` rejects this under `-D warnings`.
  Verified nothing in the tree referenced `crate::cli::…` through that declaration, then removed
  it as dead code (Rule 3, outside the plan's declared `files_modified` — documented below).
- `cargo test --features cli --test cli` ran **99 tests, 0 failed, 0 ignored** on the very first
  attempt: 37 from the five newly reactivated suites, 43 from the four pre-existing CLI output
  snapshot suites, and 19 from `tests/helpers/`'s own `#[cfg(test)]` unit tests pulled in by the
  shim. Task 2 required no runtime repairs and no test deletions.
- Confirmed and corrected the Phase 1 ledger's finding for Epic 9 tasks 13.4-13.6: the tests were
  never "never written" — they existed, complete and using current APIs, and had simply never
  been wired into the `cli` target's module tree. Plan 02-09 amends the ledger rows against the
  named exercisers below.
- `cargo test --workspace` (default features, no `cli`) stayed green — 0 failed across every
  reported group. `cargo clippy --workspace --all-targets --features cli -- -D warnings` and
  `cargo fmt --all -- --check` both exit 0.

## Task Commits

1. **Task 1: Create the helper shim, uncomment the five in-scope suites, and compile** -
   `aa3f2f5` (feat)
2. **Task 2: Run the reactivated CLI suites and make them pass** - no commit; all 37 tests passed
   on the first run against the tree left by Task 1, so no code change was required. Verified via
   `cargo test --features cli --test cli` (99 passed / 0 failed / 0 ignored),
   `cargo test --workspace` (0 failed), `cargo clippy --workspace --all-targets --features cli --
   -D warnings` (clean) and `cargo fmt --all -- --check` (clean).

**Plan metadata:** (this SUMMARY's own commit, made immediately after this file)

## Files Created/Modified

- `tests/cli/helpers.rs` (new) - Path-attribute shim re-exporting `MockArsenalPort`,
  `MockLlmAdapter`, `MockPaladinPort`, `create_mock_with_responses` from `tests/helpers/mod.rs`
  into the `cli` test target; carries the two scoped lint allows described above.
- `tests/cli/mod.rs` - Added `mod helpers;`; uncommented the five in-scope suite declarations;
  replaced the stale "missing helpers module" note with the D-09 scope-boundary note for the four
  still-commented suites.
- `tests/cli/error_handling_test.rs`, `formation_execution_test.rs`, `paladin_execution_test.rs`,
  `phalanx_execution_test.rs`, `tool_integration_test.rs` - No logic changes; `cargo fmt --all`
  reformatted each file's `use` block (rustfmt's import-grouping/sorting had never reached these
  files while their `mod` declarations were commented out) and one multi-line method-chain call
  each in `formation_execution_test.rs`/`paladin_execution_test.rs`/`phalanx_execution_test.rs`.
- `tests/lib.rs` - Removed the dead `#[cfg(feature = "cli")] pub mod cli;` declaration (Rule 3
  deviation, see below); outside the plan's declared `files_modified` list.

## Per-file test counts (for plan 02-09's ledger amendment)

| File | Present | Passing | Removed | Closes |
|---|---|---|---|---|
| `tests/cli/error_handling_test.rs` | 14 | 14 | 0 | — |
| `tests/cli/formation_execution_test.rs` | 4 | 4 | 0 | Epic 9 task 13.5 |
| `tests/cli/paladin_execution_test.rs` | 6 | 6 | 0 | Epic 9 task 13.4 |
| `tests/cli/phalanx_execution_test.rs` | 5 | 5 | 0 | Epic 9 task 13.6 |
| `tests/cli/tool_integration_test.rs` | 8 | 8 | 0 | — |
| **Total** | **37** | **37** | **0** | |

**Epic 9 task closure — named exercisers and commands, for plan 02-09:**

- **Task 13.4 (run a Paladin from config with a mock LLM adapter):**
  `paladin_execution_test::test_paladin_basic_execution` — builds a `PaladinData` config, wires a
  `MockLlmAdapter` through `PaladinExecutionService`, and asserts the response and invocation
  tracking. Run: `cargo test --features cli --test cli --
  paladin_execution_test::test_paladin_basic_execution`.
- **Task 13.5 (run a Formation with multiple mock Paladins):**
  `formation_execution_test::test_formation_basic_sequential_execution` — builds a 3-Paladin
  `Formation`, drives it through `FormationExecutionService` with a `MockPaladinPort` wrapping a
  `MockLlmAdapter`, and asserts sequential invocation and chained output. Run:
  `cargo test --features cli --test cli --
  formation_execution_test::test_formation_basic_sequential_execution`.
- **Task 13.6 (run a Phalanx with parallel execution):**
  `phalanx_execution_test::test_phalanx_basic_parallel_execution` — builds a 3-Paladin `Phalanx`,
  drives it through `PhalanxExecutionService`, and asserts all three Paladins were invoked with
  aggregated results. Run: `cargo test --features cli --test cli --
  phalanx_execution_test::test_phalanx_basic_parallel_execution`.

No test was removed under the plan's structural-breakage rule. Every one of the five files
compiled and passed against the current tree without a single construction-API repair — the
research's "medium confidence, only import lines spot-checked" caveat did not surface any actual
breakage; the sole real-world surprise was the `tests/lib.rs` duplicate-module lint issue below.

## Decisions Made

- Task 1 and Task 2 were structured as the plan specified, but Task 2 produced no diff: the five
  reactivated suites passed in full on their first run, so there was nothing to repair or commit
  beyond Task 1's changes. This is recorded explicitly rather than manufacturing an empty commit.
- Used a separate `tests/cli/helpers.rs` file (rather than an inline `#[path]` attribute directly
  on `mod helpers;` in `tests/cli/mod.rs`, per one analog form in `02-PATTERNS.md`) to satisfy the
  plan's acceptance criteria literally: `tests/cli/mod.rs` needed a plain, greppable
  `mod helpers;` line, and the shim's "re-exports, does not redefine" property needed to be
  independently verifiable by grepping `tests/cli/helpers.rs` alone.
- See `key-decisions` in frontmatter for the full reasoning on the `tests/lib.rs` deviation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking, outside declared files_modified] Removed dead `pub mod cli;` from `tests/lib.rs`**
- **Found during:** Task 1, first `cargo clippy --workspace --all-targets --all-features -- -D
  warnings` run at commit time (pre-commit hook)
- **Issue:** `tests/lib.rs` has no explicit `[[test]]` entry in `Cargo.toml` and is therefore an
  auto-discovered test binary (implicit target named "lib"). It declared
  `#[cfg(feature = "cli")] pub mod cli;`, nesting the entire `tests/cli/` tree a second time
  inside that binary — a pre-existing but previously harmless redundancy, since the four
  pre-existing snapshot suites don't use `#[path]`. Once `tests/cli/helpers.rs` (this plan) began
  re-loading `tests/helpers/mod.rs` via a `#[path]` attribute, the same physical file was loaded
  twice within the single `lib` crate compilation (`lib.rs`'s own top-level `pub mod helpers;`,
  and again via the newly nested `crate::cli::helpers::shared`), which
  `clippy::duplicate_mod` rejects under `-D warnings`, failing the pre-commit hook.
- **Fix:** Verified `grep -rn "crate::cli::" tests/` returns no reference anywhere in the tree
  other than a comment in the new shim itself, confirming the declaration was dead. Removed
  `#[cfg(feature = "cli")] pub mod cli;` from `tests/lib.rs` with an explanatory comment left in
  its place.
- **Files modified:** `tests/lib.rs` (outside the plan's declared `files_modified` list — every
  other change stayed inside it).
- **Verification:** `cargo build --tests --features cli` (0 warnings), `cargo clippy --workspace
  --all-targets --all-features -- -D warnings` (exit 0), `cargo test --workspace` (0 failed),
  `cargo fmt --all -- --check` (exit 0).
- **Committed in:** `aa3f2f5` (Task 1 commit).

---

**Total deviations:** 1 auto-fixed (Rule 3, blocking, outside declared file scope). Zero tests
deleted; zero architectural questions raised (no Rule 4 escalation).
**Impact on plan:** Necessary to reach a compiling, `-D warnings`-clean `cli` target — the fix
removed genuinely dead wiring rather than working around a real usage, and left no other target's
behavior changed (`cargo test --workspace` stayed at 0 failed before and after).

## Issues Encountered

None beyond the deviation documented above — resolved within the deviation rules without
escalation.

## User Setup Required

None - no external service configuration required. All five reactivated suites and the
pre-existing snapshot suites run fully offline against in-repo mocks; no provider keys are read.

## Next Phase Readiness

- **Plan 02-09** can amend `.planning/ledgers/milestone-01.md`'s Epic 9 nested items for tasks
  13.4, 13.5 and 13.6 using the named exercisers and commands above — each is now `satisfied`,
  correcting the ledger's "the test itself was never written" finding — and can record the
  per-file test-count table for the five reactivated suites (37/37 passing, 0 removed).
- No blockers for sibling Phase 2 plans: this plan's only out-of-declared-scope touch
  (`tests/lib.rs`) is a pure deletion of dead code with no behavior change, verified by the full
  workspace suite staying green before and after.
- The four out-of-scope CLI suites (`arsenal_config_test`, `environment_tests`,
  `garrison_config_test`, `integration_tests`) remain commented in `tests/cli/mod.rs`, with the
  boundary note pointing to plan 02-09's D-12 sweep as the place they get reported on.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*

## Self-Check: PASSED

- FOUND: `tests/cli/helpers.rs`
- FOUND: `.planning/phases/02-functional-gap-closure/02-07-SUMMARY.md`
- FOUND: commit `aa3f2f5` (Task 1)
- FOUND: commit `9d6eb2c` (this SUMMARY)
