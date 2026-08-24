---
phase: 15-coverage-ci-quality-gates
plan: 01
subsystem: infra
tags: [github-actions, ci, cargo-llvm-cov, makefile, coverage, benchmarks, cli-testing]

# Dependency graph
requires:
  - phase: 01-milestone-2-3-ledger (Phase 1/ADR-0006)
    provides: the 84% workspace coverage floor and its measurement scope, which this plan's `coverage` job re-measures under D-01's wider feature set
provides:
  - "coverage CI job: measure-only cargo llvm-cov over --features integration-tests, Redis+MinIO backed, no --fail-under-lines yet"
  - "cli-tests CI job: cargo test -p paladin-ai --features cli --test cli with a zero-executed-tests guard"
  - "bench-check CI job: cargo bench --workspace --no-run compile-only prerequisite"
  - "make coverage / make coverage-html: byte-identical local mirror of the CI coverage step, with a Redis/MinIO reachability guard naming make services-up"
  - "make test-cli / make bench-check: local mirrors of the two new CI gates"
  - "make ci-full: ci-test then coverage, the full local CI gate in one invocation"
affects: [15-02, 15-03, 15-04, 15-05]

# Tech tracking
tech-stack:
  added: ["cargo-llvm-cov@0.8.7 (via taiki-e/install-action@v2)"]
  patterns: ["measure-only CI gate landed before its threshold is set (D-04 two-commit sequence)", "dedicated named job per gate instead of a step inside an existing job (D-07)"]

key-files:
  created: []
  modified:
    - .github/workflows/ci.yml
    - Makefile

key-decisions:
  - "bench-check's cache key uses cargo-bench-check- rather than the plan-specified cargo-bench-, because the pre-existing `benchmark` job (ci.yml:800, unmodified by this plan) already owns the literal cargo-bench- key — reusing it would violate the plan's own stated intent (\"a cache key distinct from every other job's so the two do not evict each other\") and let a --no-run compile-check job and a full `cargo bench` execution job thrash the same GitHub Actions cache."

patterns-established:
  - "Byte-identical CI/local command pairs verified by a literal diff of the extracted `llvm-cov ... lcov.info ...` substring, not just eyeballed equality"

requirements-completed: [PIPE-01, PIPE-02, PIPE-03]

coverage:
  - id: D1
    description: "coverage CI job measures the workspace under --features integration-tests behind live Redis/MinIO, producing lcov.info, with no coverage threshold yet (D-04 measure-only)"
    requirement: "PIPE-02"
    verification:
      - kind: other
        ref: "python3 -c \"import yaml; d=yaml.safe_load(open('.github/workflows/ci.yml')); j=d['jobs']['coverage']; assert 'needs' not in j and set(j['services'])=={'redis','minio'}\""
        status: pass
      - kind: other
        ref: "grep -c 'llvm-tools-preview' .github/workflows/ci.yml == 1; grep -c 'tool: cargo-llvm-cov@' .github/workflows/ci.yml == 1"
        status: pass
    human_judgment: true
    rationale: "The job has not executed in real CI yet (Docker/GH Actions runners are absent from this authoring environment) — a human must confirm the first real run passes and produces lcov.info before plan 15-03 derives the threshold from it."
  - id: D2
    description: "make coverage / make coverage-html mirror the CI coverage step byte-for-byte and fail loudly (naming make services-up) when Redis:6380 or MinIO:9010 are unreachable"
    requirement: "PIPE-03"
    verification:
      - kind: other
        ref: "diff of grep -o 'llvm-cov .*lcov.info.*' between ci.yml and `make -n coverage` output — identical"
        status: pass
      - kind: other
        ref: "make -n coverage && make -n coverage-html"
        status: pass
    human_judgment: false
  - id: D3
    description: "cli-tests CI job runs the required-features-gated cli test target and fails on a vacuous zero-test run"
    requirement: "PIPE-01"
    verification:
      - kind: other
        ref: "python3 yaml job-count/needs/unique-name check (18 jobs total, no duplicate name:)"
        status: pass
    human_judgment: true
    rationale: "cargo test -p paladin-ai --features cli --test cli was not run to completion in this session (per orchestrator instruction, to avoid a multi-minute cold workspace compile) — a human/CI run must confirm the 86-snapshot suite actually passes and the zero-count guard fires correctly on a feature-flag regression."
  - id: D4
    description: "bench-check CI job compiles every [[bench]] target without running it, leaving the existing benchmark/benchmark-regression-signal jobs untouched"
    requirement: "PIPE-01"
    verification:
      - kind: other
        ref: "git diff shows zero modified lines inside the pre-existing benchmark/benchmark-regression-signal job bodies (pure insertion, no deletions)"
        status: pass
    human_judgment: true
    rationale: "cargo bench --workspace --no-run was not run to completion in this session (cold workspace compile, per orchestrator instruction) — a human/CI run must confirm it actually exits 0."
  - id: D5
    description: "Makefile test-cli and bench-check targets mirror the two new CI gates' commands exactly; ci-test gains a test-cli step; ci-full chains ci-test then coverage"
    requirement: "PIPE-03"
    verification:
      - kind: other
        ref: "make -n test-cli / make -n bench-check / make -n ci-full / make -n ci-test | grep test-cli; make help lists all five new targets; five .PHONY grep counts == 1 each"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 01: Coverage, CLI-tests and bench-check CI gates Summary

**Three new named CI jobs (`coverage` measure-only, `cli-tests` with a zero-test guard, `bench-check` compile-only) plus five Makefile targets that mirror them byte-for-byte, landing PIPE-01/02/03's gate scaffolding without yet setting a coverage threshold.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-13T01:59:11Z
- **Tasks:** 3 of 3
- **Files modified:** 2 (`.github/workflows/ci.yml`, `Makefile`)

## Accomplishments

- Added a `coverage` job to `.github/workflows/ci.yml` (Redis+MinIO service containers copied verbatim from `integration-tests`, `llvm-tools-preview` toolchain component, `cargo-llvm-cov@0.8.7` pinned via `taiki-e/install-action@v2`, `cargo llvm-cov --workspace --features integration-tests --lcov --output-path lcov.info -- --test-threads=1`). No `--fail-under-lines` yet — deliberately measure-only per D-04; plan 15-03 sets the floor from this job's first real measurement.
- Added `cli-tests` (a dedicated job per D-07, `cargo test -p paladin-ai --features cli --test cli` with a shell guard that fails the step if the reported executed-test count is zero — the exact failure mode Cargo's `required-features` silent-skip would otherwise hide) and `bench-check` (`cargo bench --workspace --no-run`, compile-only, cache key `cargo-bench-check-` kept distinct from the pre-existing `benchmark` job's `cargo-bench-` key). CI job count went from 15 to 18, all `name:` values unique.
- Added a `##@ Coverage` Makefile section (`coverage`, `coverage-html`) placed before `##@ Code Quality`, plus `test-cli` and `bench-check` in `##@ Testing`, plus `ci-full: ci-test coverage`. `ci-test` now runs `test-cli` between `test` and `test-doc`. The `cargo llvm-cov` invocation in the Makefile `coverage` target is byte-identical (verified via `diff` on the extracted flag substring) to the CI step's.

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end coverage measurement — one path, CI and local** - `e54a94f` (feat) — lands the `coverage` job in `.github/workflows/ci.yml` and the `##@ Coverage` Makefile section (`coverage`, `coverage-html`). **This is the commit plan 15-03's checkpoint needs to locate the CI run that produces the measurement.**
2. **Task 2: cli-tests and bench-check jobs — the two PIPE-01 gates** - `16b34f8` (feat)
3. **Task 3: Makefile Testing and CI targets — local parity for the new jobs** - `a6653b5` (feat)

_No plan-metadata commit — orchestrator policy for this worktree defers STATE.md/ROADMAP.md updates to post-wave._

## Files Created/Modified

- `.github/workflows/ci.yml` - Adds `coverage`, `cli-tests`, `bench-check` jobs (167 lines added, 0 removed; existing jobs including `benchmark`/`benchmark-regression-signal` untouched)
- `Makefile` - Adds `##@ Coverage` section (`coverage`, `coverage-html`), `test-cli`, `bench-check` in `##@ Testing`, extends `ci-test`, adds `ci-full` (29 lines added, 0 removed)

## Decisions Made

- **bench-check cache key changed from the plan's literal `cargo-bench-` to `cargo-bench-check-`.** The plan's action text names `${{ runner.os }}-cargo-bench-${{ hashFiles('**/Cargo.lock') }}` for `bench-check`, explicitly justified as "a cache key distinct from every other job's so the two do not evict each other" — but the pre-existing `benchmark` job (ci.yml, untouched by this plan) already owns exactly that key. Using the literal string would have produced two jobs (one `--no-run` compile-check, one full `cargo bench` execution) racing for the same GitHub Actions cache, contradicting the plan's own stated rationale. Kept the literal substring `cargo-bench-` (satisfies the plan's grep-based acceptance criterion) but suffixed `-check-` so the two jobs get genuinely separate caches. Documented here rather than silently deviating — this is a Rule 1 auto-fix (the plan's stated intent and its literal instruction conflicted; the intent won).
- **Split the single combined edit into three atomic per-task commits** by writing the full Task-1+2+3 content first, then temporarily removing the Task 2/3 hunks, committing Task 1, re-adding Task 2's hunk, committing, and finally Task 3. No functional difference from writing them in order; noted because the git history therefore shows clean per-task diffs rather than the actual edit sequence.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `bench-check` cache key changed to avoid colliding with the pre-existing `benchmark` job's cache key**
- **Found during:** Task 2 (cli-tests and bench-check jobs)
- **Issue:** The plan specifies cache key `${{ runner.os }}-cargo-bench-${{ hashFiles('**/Cargo.lock') }}` for the new `bench-check` job, with the explicit stated rationale "a cache key distinct from every other job's so the two do not evict each other." The pre-existing `benchmark` job (ci.yml, unmodified) already uses that exact key.
- **Fix:** Used `${{ runner.os }}-cargo-bench-check-${{ hashFiles('**/Cargo.lock') }}` instead — still contains the literal substring `cargo-bench-` (satisfying the plan's grep-based verify checks) but is a genuinely distinct key from the `benchmark` job's.
- **Files modified:** `.github/workflows/ci.yml`
- **Verification:** `grep -n 'cargo-cli-\|cargo-bench-check-\|cargo-bench-' .github/workflows/ci.yml` shows three distinct key strings (`cargo-cli-`, `cargo-bench-check-`, `cargo-bench-` on the pre-existing `benchmark` job, `cargo-bench-signal-` on `benchmark-regression-signal`)
- **Committed in:** `16b34f8` (Task 2 commit)

**2. [Rule 1 - Bug] Removed a literal `--fail-under-lines` mention from a comment to avoid a false-positive grep**
- **Found during:** Task 1 (coverage job)
- **Issue:** An explanatory comment above the `coverage` job originally named the `--fail-under-lines` flag by its literal string to explain why it's absent (D-04), which would have made `grep -c 'fail-under-lines' .github/workflows/ci.yml` return 1 instead of the required 0 (the plan's acceptance criterion literally checks that this string appears nowhere in the file yet).
- **Fix:** Reworded the comment to describe the same intent ("no minimum-threshold flag") without using the literal flag name.
- **Files modified:** `.github/workflows/ci.yml`
- **Verification:** `grep -c 'fail-under-lines' .github/workflows/ci.yml` returns `0`
- **Committed in:** `e54a94f` (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1 — bugs in the literal instruction text that contradicted the plan's own stated intent or acceptance criteria)
**Impact on plan:** Both fixes preserve the plan's stated intent exactly; no scope change, no architectural decision, no scope creep.

## Issues Encountered

- Initially read and edited files at `/workspace/...` (the main checkout) instead of the worktree-anchored path `/workspace/.claude/worktrees/agent-acaf44020ea247f84/...`. Caught immediately by the Edit tool's worktree-isolation guard before any write landed outside the worktree; confirmed the two paths held byte-identical content at that point (no divergence), then redid the reads/edits against the correct worktree path. No file was actually written to the wrong location.
- `cargo bench --workspace --no-run` and `cargo test -p paladin-ai --features cli --test cli` were not run to completion in this session — the workspace's cold-cache compile time (11+ minutes for `cargo bench --no-run` alone, observed via a background task before the orchestrator intervened) made full-compile verification impractical within this executor turn. Per explicit orchestrator instruction, Tasks 2 and 3 were verified with static checks only (YAML parse, `grep` for job/target names and cache keys, `make -n` dry runs, and a literal `diff` of the CI/Makefile command strings). **This is recorded as `human_judgment: true` on coverage deliverables D3 and D4 above** — the first real CI run (or a local `make ci-full` with a warm cache) must confirm `cli-tests` and `bench-check` actually pass.

## User Setup Required

None - no external service configuration required for this plan. (The `coverage` job itself requires GitHub Actions' Redis/MinIO service containers, which are provisioned automatically by the workflow — no manual setup.)

## Next Phase Readiness

- Plan 15-02 (deprecated-action replacement) can proceed — none of the eight `actions-rs/toolchain@v1` / `actions/cache@v3` sites this plan's three new jobs use are affected (`dtolnay/rust-toolchain@stable` and `actions/cache@v4` used throughout, per PATTERNS.md's already-correct convention).
- **Plan 15-03 needs commit `e54a94f`** to locate the first `coverage` job CI run and read its measured line-coverage figure — that figure is what gets wired into `--fail-under-lines` and ADR-0006's amendment.
- The two `human_judgment: true` deliverables (D3, D4 above — `cli-tests` and `bench-check` actually passing) should be confirmed by the first real CI run on this branch, or by running `make ci-full` locally once a warm build cache is available.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*
