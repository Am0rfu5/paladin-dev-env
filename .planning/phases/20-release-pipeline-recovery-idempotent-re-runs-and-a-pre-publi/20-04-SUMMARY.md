---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 04
subsystem: infra
tags: [bash, python3, cargo-metadata, cargo-release, changelog, shellcheck, tdd, makefile]

# Dependency graph
requires:
  - phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
    provides: "20-01: scripts/check-release-consistency.sh (the manifest-agreement gate this plan makes satisfiable) and its cargo metadata enumeration convention"
provides:
  - "scripts/finalize-crate-changelogs.sh -- stamps a dated '## [X.Y.Z]' section into every publishable package's changelog (root + ten crates) in one idempotent, collect-then-report invocation, driven by the same cargo metadata enumeration check-release-consistency.sh uses"
  - "tests/scripts/finalize-crate-changelogs_test.sh -- 25-assertion regression harness covering all eight plan <behavior> cases plus MISSING_VERSION and unknown-flag usage-error cases"
  - "make finalize-crate-changelogs VERSION=x.y.z -- local entry point, VERSION-required guard"
  - "make release now finalizes all eleven changelogs and runs check-release-consistency before tag/push, replacing the root-only perl one-liner"
  - "release.toml comment extending the root-changelog pre-release-replacements rationale to the ten crate changelogs, naming the new script as carrier, recording the observed cargo-release version"
affects: [20-05, 20-06, 20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Gate/tooling script house shape reused a third time: bash wrapper + python3 heredoc, STATUS_LINE/DETAIL report, collect-then-report, named FINALIZE_FAILED/ZERO_PACKAGES/MISSING_VERSION failures"
    - "manifest_path -> os.path.dirname(manifest_path)/CHANGELOG.md resolution: the same cargo metadata enumeration as check-release-consistency.sh, extended one field (manifest_path) to derive the changelog location without hardcoding a crate list or a crates/* glob"
    - "FINALIZE_CRATE_CHANGELOGS_LIB_ONLY sourcing seam, mirroring CHECK_RELEASE_CONSISTENCY_LIB_ONLY from 20-01"
    - "make release step ordering: direct script invocation (not $(MAKE) target) for any step that must remain inert under `make -n` -- GNU Make always executes a recipe line referencing $(MAKE) even in dry-run mode, which would have made `make -n release VERSION=9.9.9` actually run the consistency gate against a fake tag"

key-files:
  created:
    - scripts/finalize-crate-changelogs.sh
    - tests/scripts/finalize-crate-changelogs_test.sh
  modified:
    - Makefile
    - release.toml

key-decisions:
  - "Already-finalized detection matches on the literal '## [X.Y.Z]' prefix only (ignoring any trailing ' - date' text), so a changelog already carrying that version's heading under a different date is left untouched rather than treated as a mismatch needing correction"
  - "New section insertion uses str.replace(anchor, anchor + new_heading, count=1) so the '## [Unreleased]' anchor is preserved verbatim and any existing older section already below it is pushed down rather than reordered -- this is what keeps a changelog with a stale older section in newest-first order without special-casing that shape"
  - "Task 2's make release wiring calls scripts/check-release-consistency.sh directly rather than via a recursive $(MAKE) call, specifically to keep the step inert under 'make -n' -- GNU Make's documented behavior of always executing $(MAKE)-referencing recipe lines even under -n would otherwise make dry-run testing of the release target actually invoke the gate"
  - "cargo-release --version observed locally: 'cargo-release 1.1.2' (recorded per D-09's verify-don't-assume instruction; release.toml comment cites this figure); note the CLI itself only accepts 'cargo release --version', not 'cargo-release --version' as a subcommand-style invocation"

patterns-established:
  - "Pattern: a tooling script that WRITES files (unlike a pure gate script) still follows the offline, fixture-seam, collect-then-report house shape -- the only addition is a closing real-tree mutation guard in its test harness, since every fixture must route manifest_path into a scratch directory rather than the real crates/ tree"

requirements-completed: [PUBOPS-01]

coverage:
  - id: D1
    description: "finalize-crate-changelogs.sh stamps a dated section into every publishable package's changelog (root + ten crates) in one invocation, is idempotent across re-runs, refuses to guess an insertion point when the anchor is missing, and treats zero discovered publishable packages as a named failure"
    requirement: "PUBOPS-01"
    verification:
      - kind: unit
        ref: "tests/scripts/finalize-crate-changelogs_test.sh (25 assertions, all pass; >= plan's 14-assertion floor)"
        status: pass
      - kind: manual_procedural
        ref: "make test-shell-guards (all 5 shell-guard suites pass, 81 total assertions); git status --porcelain -- CHANGELOG.md crates .github/workflows is empty after the harness runs"
        status: pass
    human_judgment: false
  - id: D2
    description: "make release finalizes every changelog and then runs the consistency gate for the version it is about to tag, before commit/tag/push, replacing the root-only perl step"
    requirement: "PUBOPS-01"
    verification:
      - kind: unit
        ref: "python3 ordering assertion over `make -n release VERSION=9.9.9` output: idx('finalize-crate-changelogs') < idx('check-release-consistency') < idx('git tag') -> 'release ordering OK'"
        status: pass
      - kind: manual_procedural
        ref: "make -n release VERSION=9.9.9 (no perl CHANGELOG.md step remains; branch check, up-to-date check, release-check, cargo release version all present and unmoved); make release with no VERSION exits non-zero; git diff --name-only for this commit is Makefile only"
        status: pass
    human_judgment: false

# Metrics
duration: ~37min
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 04: Finalize Crate Changelogs Summary

**scripts/finalize-crate-changelogs.sh stamps a dated changelog section into every publishable package (root + ten crates) in one idempotent, collect-then-report invocation, then `make release` runs it and the pre-publish consistency gate before every tag push -- making the D-09 strict, no-exemptions gate satisfiable without an eleven-file manual chore.**

## Performance

- **Duration:** ~37 min
- **Started:** 2026-08-28T15:15:14Z (first task commit)
- **Completed:** 2026-08-28T15:52:25Z (last task commit)
- **Tasks:** 2 (Task 1 tracer/tdd, Task 2 auto)
- **Files modified:** 4 (2 created, 2 modified)

## Accomplishments
- `scripts/finalize-crate-changelogs.sh`: enumerates publishable packages via `cargo metadata --no-deps --format-version 1` (the same enumeration `check-release-consistency.sh` uses -- the plan's binding requirement that the two scripts can never disagree about which files matter) and resolves each package's changelog as `CHANGELOG.md` in the directory holding its `manifest_path`, so the root package resolves to the root `CHANGELOG.md` and each crate to its own without a hardcoded list or a `crates/*` glob
- Per file: an existing `## [X.Y.Z]` section for the target version (any date) is left untouched and reported `already-finalized`, guaranteeing byte-identical output across re-runs; a `## [Unreleased]` anchor gets the new dated section inserted immediately after it via a single `str.replace(..., count=1)`, preserving the anchor and pushing any existing older section further down (newest-first order maintained without special-casing); a file with neither is a named failure left completely unmodified, and the run continues to the next package (collect-then-report, never fail-fast) -- zero discovered publishable packages is itself a named `ZERO_PACKAGES` failure
- `Makefile` gains `finalize-crate-changelogs` (VERSION-required guard, same shape as `release`'s own guard) and the `release` target's changelog-finalize step is replaced with a single call to the new script; immediately after, `release` now runs `scripts/check-release-consistency.sh --tag "v$(VERSION)"` directly (not via `$(MAKE)`, to stay inert under `make -n`) before commit/tag/push -- so a tag is never pushed for a tree the gate would reject
- `release.toml` gains a comment immediately after the existing root-changelog rationale, extending it to the ten crate changelogs and naming the new script as carrier; records the observed `cargo-release --version` output (`cargo-release 1.1.2`) per D-09's verify-don't-assume instruction. Diff is comment-only -- no cargo-release key added or changed
- `tests/scripts/finalize-crate-changelogs_test.sh`: 25-assertion regression harness (>= the plan's 14-assertion floor) covering all eight `<behavior>` cases (dual-package finalize, double-run idempotency via `cmp`, already-present-section with exact one-heading-line assertion, missing-anchor collect-then-report with "other package still processed" proof, missing-changelog-file, zero-publishable-packages, prerelease version stamped character-for-character, existing-older-section ordering) plus `MISSING_VERSION` and unknown-flag usage-error cases, plus a closing real-tree mutation guard over `Cargo.toml`/`crates/`/`CHANGELOG.md`/`.github/workflows/`
- Verified against the real tree: `make test-shell-guards` runs all 5 shell-guard suites (81 total assertions) clean; `git status --porcelain -- CHANGELOG.md crates .github/workflows` is empty after every harness run; `make -n release VERSION=9.9.9` shows `finalize-crate-changelogs` before `check-release-consistency` before `git tag`, with no `perl` invocation against `CHANGELOG.md` remaining and every other release step unchanged and in original order

## Task Commits

Each task was committed atomically (Task 1 followed the TDD RED/GREEN cycle since it carries `tdd="true"`):

1. **Task 1 (RED): failing regression harness** - `8c80954f` (test) -- `tests/scripts/finalize-crate-changelogs_test.sh` written first; confirmed to fail (`ERROR: guard script not found`) before any implementation existed
2. **Task 1 (GREEN): finalize script + make target + release.toml comment** - `35ccb2f6` (feat) -- `scripts/finalize-crate-changelogs.sh`, the `Makefile` `finalize-crate-changelogs` target, and the `release.toml` comment, making the RED test (25 assertions) and the plan's own `<verify>` block pass end to end
3. **Task 2: wire finalize and the gate into make release** - `a2ff9810` (feat) -- replaces the root-only `perl` changelog step with a call to the new script, adds the `check-release-consistency` gate invocation immediately before commit/tag/push

**Plan metadata:** pending (this commit, `docs(20-04): complete plan`, made after this SUMMARY)

_TDD gate compliance: `test(20-04)` commit (RED) precedes `feat(20-04)` commit (GREEN) in git log -- gate sequence satisfied for Task 1._

## Files Created/Modified
- `scripts/finalize-crate-changelogs.sh` - the changelog-finalize tooling (D-09); flags-only CLI (`--version`, `--date`, `--metadata-json`, `--workspace-root`); `FINALIZE_CRATE_CHANGELOGS_LIB_ONLY` sourcing seam
- `tests/scripts/finalize-crate-changelogs_test.sh` - 25-assertion regression harness in the repo's established guard-test shape
- `Makefile` - new `finalize-crate-changelogs` target (VERSION-required guard); `release` target's changelog-finalize step replaced with the new script; consistency-gate invocation added immediately before commit/tag/push
- `release.toml` - comment-only addition extending the root-changelog `pre-release-replacements` rationale to the ten crate changelogs, naming the carrier script, recording the observed `cargo-release` version

## Decisions Made
- Already-finalized detection matches on the literal `## [X.Y.Z]` heading prefix only, ignoring any trailing date text -- a changelog already carrying that version's section under a different recorded date (e.g. a manually-edited or re-tagged release) is left alone rather than "corrected," matching the plan's own behavior spec for that case
- New-section insertion is a single `str.replace(anchor, anchor + new_heading, count=1)`, which both preserves the `## [Unreleased]` anchor verbatim and naturally keeps any existing older section below the new one (newest-first) without any additional ordering logic
- Task 2's gate invocation in `make release` calls `scripts/check-release-consistency.sh` directly rather than through a recursive `$(MAKE) check-release-consistency RELEASE_TAG=...` call. GNU Make always executes a recipe line that references the `$(MAKE)` variable, even under `-n`/dry-run -- using the recursive form would have made `make -n release VERSION=9.9.9` actually invoke the gate against the fake `v9.9.9` tag (which would fail, since no real manifest is at that version), breaking the plan's own dry-run verification method. Calling the script directly (mirroring the `check-release-consistency` target's own body) keeps the step inert under `-n` while remaining byte-identical in behavior under real execution
- `cargo-release --version` was recorded via `cargo release --version` (the CLI does not accept `cargo-release --version` directly, despite the binary being named `cargo-release` on `$PATH`) -- observed `cargo-release 1.1.2`, recorded in `release.toml`'s new comment per D-09's explicit "verify, don't assume" instruction

## Deviations from Plan

None - plan executed exactly as written. No Rule 1-4 auto-fixes were needed; the plan's `<action>` and `<read_first>` sections, together with 20-01's already-committed `check-release-consistency.sh` as a direct sibling analog, were detailed enough to implement directly.

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step (same `always_run: true` behavior documented in 20-01-SUMMARY.md) exceeded the Bash tool's default 2-minute timeout on the first commit attempt against a cold build cache. Resolved identically to 20-01: ran `cargo clippy` directly in the background first to warm the cache, then retried the commit (completed in well under 2 minutes against the warm cache). No code change was needed; purely a tooling/timing accommodation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `make release` now finalizes all eleven changelogs and gates on `check-release-consistency` before tagging, so D-09's "prerelease tags get the same gate as stable tags, no exempted path" requirement is satisfiable in practice: a future `make release VERSION=0.9.0` run will actually stamp every crate changelog and verify agreement, rather than leaving ten files at `## [Unreleased]` for the gate to (correctly) reject.
- `scripts/check-release-consistency.sh`'s reserved `--sha`/`--ci-runs-json` flags are unaffected by this plan and remain available for the sibling plan that gives them behavior (CI-conclusion clause, PUBOPS-02).
- No blockers identified for downstream plans in this phase (20-05, 20-06, 20-07).

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*

## Self-Check: PASSED

- FOUND: scripts/finalize-crate-changelogs.sh
- FOUND: tests/scripts/finalize-crate-changelogs_test.sh
- FOUND: 8c80954f (RED test commit)
- FOUND: 35ccb2f6 (GREEN feat commit)
- FOUND: a2ff9810 (Task 2 make-release wiring commit)
