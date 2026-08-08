---
phase: 09-release-security-gate-integrity
plan: 01
subsystem: infra
tags: [changelog, ci, guard-script, python3-tomllib, keep-a-changelog, cargo-deny]

# Dependency graph
requires: []
provides:
  - "crates/paladin-herald/CHANGELOG.md — the tenth per-crate changelog (SEC-04 / D-14)"
  - "scripts/check-changelogs.sh — a demonstrably-failable guard asserting every publishable crate has a CHANGELOG.md (D-15)"
  - "Makefile check-changelogs target wrapping the guard"
  - "A CI step ('Check per-crate changelogs') inside the cargo-deny job, whose display name 'License & Dependency Policy' is a required status-check context"
  - "A proven failable-guard idiom (bash-level single exit-0/exit-1 conditional) for plans 09-04 and 09-06 to copy"
affects: [09-04, 09-06]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Failable-guard idiom: python3 heredoc computes and reports, bash-level if/else performs the single exit 0 / exit 1 (mirrors scripts/check-api-surface.sh:31-46), so exactly one success exit exists and neither branch falls through."
    - "python3 stdlib tomllib heredoc for structured Cargo.toml parsing (mirrors scripts/check-doc-config.sh's command -v python3 guard + heredoc idiom), reading [package].publish to decide crate exemption rather than hard-coding directory names."

key-files:
  created:
    - crates/paladin-herald/CHANGELOG.md
    - scripts/check-changelogs.sh
  modified:
    - Makefile
    - .github/workflows/ci.yml

key-decisions:
  - "Guard exemption reads Cargo.toml's [package].publish field via tomllib, not directory name — proven in Task 2 mode 3 by flipping crates/doc-examples/Cargo.toml's publish field to true and observing the guard start naming paladin-doc-examples."
  - "Zero-publishable-crates-discovered is treated as its own distinct failure message, separate from 'crate X is missing a changelog' — proven in Task 2 mode 4 against an empty scratch crates/ tree."
  - "The guard reports every offending crate in one run rather than stopping at the first (glob is fully walked and offenders collected in a list before any output) — proven in Task 2 mode 2 with two changelogs removed simultaneously."
  - "CI placement: inside the cargo-deny job (display name 'License & Dependency Policy'), ahead of the cargo-deny install step, for fail-fast — not a new job, per the plan's explicit instruction not to touch any actions-rs/dtolnay reference in that job."

patterns-established:
  - "Any future *-guard.sh script in this phase (09-04, 09-06) should structure its bash body the same way: python3/awk does the analysis and prints a single status line plus detail, and the outer bash script does exactly one if/else with one literal exit 0 and one literal exit 1."

requirements-completed: [SEC-04]

coverage:
  - id: D1
    description: "crates/paladin-herald/CHANGELOG.md created, matching the sibling Keep-a-Changelog shape, backfilling the crate's real creation history (commit 66f6c4e) and recording ADR-0023's comfy-table/colored feature gating as a breaking default-features change"
    requirement: "SEC-04"
    verification:
      - kind: other
        ref: "test -f crates/paladin-herald/CHANGELOG.md && grep -c '^## \\[Unreleased\\]$' (=1) && grep -c '0023' (>=1)"
        status: pass
    human_judgment: false
  - id: D2
    description: "scripts/check-changelogs.sh — mechanical every-publishable-crate-has-a-changelog guard, demonstrably failable"
    requirement: "SEC-04"
    verification:
      - kind: other
        ref: "./scripts/check-changelogs.sh (clean tree, exit 0) and the four Task 2 negative-path invocations below (all non-zero)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Makefile check-changelogs target wrapping the guard"
    verification:
      - kind: other
        ref: "make -n check-changelogs"
        status: pass
    human_judgment: false
  - id: D4
    description: "CI step 'Check per-crate changelogs' wired into the cargo-deny job (required status-check context 'License & Dependency Policy')"
    verification:
      - kind: other
        ref: "grep -c check-changelogs.sh .github/workflows/ci.yml (=1), step placed between the job's toolchain-install and cache-registry steps"
        status: pass
    human_judgment: true
    rationale: "The step's placement and invocation are verified structurally (grep + manual read of the job body); the step has not been executed by an actual GitHub Actions runner within this sandboxed worktree, so a human should confirm the next real CI run on this branch exercises it as expected."
  - id: D5
    description: "All four guard failure modes demonstrated non-zero (missing changelog, multiple missing, exemption-by-field not by name, vacuous-pass resistance) with exact commands and exit codes recorded verbatim"
    requirement: "SEC-04"
    verification:
      - kind: other
        ref: "See 'Negative-path evidence' section below for all four verbatim transcripts"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 01: Herald changelog + failable changelog guard tracer Summary

**Added the missing tenth per-crate CHANGELOG.md and proved the phase's failable-guard idiom end-to-end with a `scripts/check-changelogs.sh` guard wired to `make` and a required CI context, observed failing four distinct ways.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-07T23:58:00Z (approx.)
- **Completed:** 2026-08-08T00:02:39Z
- **Tasks:** 2
- **Files modified:** 4 (2 created, 2 modified)

## Accomplishments

- Created `crates/paladin-herald/CHANGELOG.md`, closing SEC-04, backfilled from real history (crate creation commit `66f6c4e`) rather than a generic boilerplate stub, and recording ADR-0023's `table`/`color` feature gating as a breaking default-features change with the exact feature-to-dependency mapping read from `Cargo.toml`.
- Wrote `scripts/check-changelogs.sh`, a `python3`+`tomllib` guard that walks `crates/*/Cargo.toml`, exempts only manifests with `publish = false`, collects every offending crate before reporting, and closes with a single bash-level `exit 0`/`exit 1` conditional (mirroring `scripts/check-api-surface.sh`) so the guard cannot fall through to a false pass — the exact failure mode `scripts/check-deprecations.sh` exhibited in Phase 8.
- Added a `Makefile` `check-changelogs` target and a CI step inside the `cargo-deny` job (display name `License & Dependency Policy`, a required status-check context per `.github/rulesets/protect-main-branch.json`), placed ahead of the `cargo-deny` install for fail-fast, without touching any other step or toolchain reference in that job.
- Drove the guard through all four required failure modes and recorded each transcript verbatim (below), then restored the tree to a byte-identical clean state.

## Task Commits

1. **Task 1: End-to-end "a missing per-crate changelog fails CI" — one path only** - `0458b6a` (feat)
2. **Task 2: Prove every failure mode of the guard, then record the evidence** - no commit (all four modes tripped correctly on first attempt; `scripts/check-changelogs.sh` required no repair, so there is no diff beyond Task 1's committed version — evidence is recorded here in the SUMMARY per D-19)

**Plan metadata:** committed alongside this SUMMARY.

## Files Created/Modified

- `crates/paladin-herald/CHANGELOG.md` - the tenth per-crate changelog (SEC-04)
- `scripts/check-changelogs.sh` - the failable changelog guard (D-15)
- `Makefile` - added `check-changelogs` target adjacent to `check-doc-config`
- `.github/workflows/ci.yml` - added "Make scripts executable" + "Check per-crate changelogs" steps inside the `cargo-deny` job

## Decisions Made

- Followed `scripts/check-api-surface.sh`'s bash-level `if/else` exit-code shape rather than letting the python3 heredoc call `sys.exit()` directly, so the file contains exactly one literal `exit 0` (verified: `grep -c 'exit 0' scripts/check-changelogs.sh` → `1`) and exactly one success path, per the plan's explicit acceptance criterion.
- Did not add an `@echo` banner to the `Makefile` target, per the plan's explicit instruction, even though the two neighboring targets (`check-doc-examples`, `check-doc-config`) do have one — the plan text takes precedence over the "copy verbatim" framing where the two conflict.

## Deviations from Plan

None — plan executed exactly as written. Both tasks completed on the first attempt with no auto-fixes, no blocking issues, and no repairs needed to the guard script during Task 2's failure-mode sweep.

## Negative-path evidence (Task 2, per D-19)

All four fixtures were reverted after their run; the working tree in this worktree was confirmed byte-identical to its post-Task-1 state throughout (`git status --porcelain crates/` empty after each restoration, `git diff --stat -- crates/doc-examples/Cargo.toml` empty after mode 3).

### Mode 1 — missing changelog on a single publishable crate

Command: `mv crates/paladin-battalion/CHANGELOG.md /tmp/mode1-b.md && ./scripts/check-changelogs.sh; echo "EXIT=$?"` (then restored)

```
🔍 Checking per-crate CHANGELOG.md coverage in .../crates ...
❌ Per-crate changelog check failed

FAIL: 1 publishable crate(s) missing CHANGELOG.md:
  - paladin-battalion

If this failure is unexpected:
  1. Add a CHANGELOG.md to each crate named above, matching the
     Keep a Changelog shape used by its sibling crates.
  2. A crate should only be exempt if its Cargo.toml genuinely sets
     publish = false.
EXIT=1
```

### Mode 2 — multiple missing changelogs, order-independent

Command: `mv crates/paladin-battalion/CHANGELOG.md /tmp/mode2-b.md && mv crates/paladin-content/CHANGELOG.md /tmp/mode2-c.md && ./scripts/check-changelogs.sh; echo "EXIT=$?"` (then restored both)

```
🔍 Checking per-crate CHANGELOG.md coverage in .../crates ...
❌ Per-crate changelog check failed

FAIL: 2 publishable crate(s) missing CHANGELOG.md:
  - paladin-battalion
  - paladin-content

If this failure is unexpected:
  1. Add a CHANGELOG.md to each crate named above, matching the
     Keep a Changelog shape used by its sibling crates.
  2. A crate should only be exempt if its Cargo.toml genuinely sets
     publish = false.
EXIT=1
```

Both offending crate names appear in one run — the guard's verdict does not stop at the first offender and does not depend on glob-iteration order.

### Mode 3 — exemption follows the manifest field, not the directory name

Command (untouched tree): `./scripts/check-changelogs.sh; echo "EXIT=$?"`

```
🔍 Checking per-crate CHANGELOG.md coverage in .../crates ...
✅ 10 publishable crate(s) checked, all have a CHANGELOG.md.
EXIT=0
```

`crates/doc-examples/` carries no `CHANGELOG.md` and the guard still exits 0, because `doc-examples/Cargo.toml` sets `publish = false`.

Command (manifest flipped): `sed -i 's/^publish = false$/publish = true/' crates/doc-examples/Cargo.toml && ./scripts/check-changelogs.sh; echo "EXIT=$?"` (then restored from a pre-edit copy)

```
🔍 Checking per-crate CHANGELOG.md coverage in .../crates ...
❌ Per-crate changelog check failed

FAIL: 1 publishable crate(s) missing CHANGELOG.md:
  - paladin-doc-examples

If this failure is unexpected:
  1. Add a CHANGELOG.md to each crate named above, matching the
     Keep a Changelog shape used by its sibling crates.
  2. A crate should only be exempt if its Cargo.toml genuinely sets
     publish = false.
EXIT=1
```

Flipping only the `publish` field (no directory rename) flips the verdict and names `paladin-doc-examples` — the exemption is read from the manifest field, not hard-coded by directory name. `git diff --stat -- crates/doc-examples/Cargo.toml` was empty after restoring.

### Mode 4 — vacuous-pass resistance

Run from a scratch directory (`/tmp/claude-1000/.../scratchpad/mode4-test`) holding a copy of the guard script, entirely outside this git worktree.

**4a — empty `crates/` directory:**

Command: `<scratch>/scripts/check-changelogs.sh; echo "MODE4A_EXIT=$?"`

```
🔍 Checking per-crate CHANGELOG.md coverage in <scratch>/crates ...
❌ Per-crate changelog check failed

FAIL: zero publishable crates discovered under crates/*/ -- this looks like a broken glob or an empty workspace, not success.

If this failure is unexpected:
  1. Add a CHANGELOG.md to each crate named above, matching the
     Keep a Changelog shape used by its sibling crates.
  2. A crate should only be exempt if its Cargo.toml genuinely sets
     publish = false.
MODE4A_EXIT=1
```

Exits non-zero with a message distinct from "N crates missing a changelog" — a broken glob cannot present as success.

**4b — a `crates/<name>/` directory with no `Cargo.toml`, alone:**

Added `crates/not-a-crate/README.md` (no `Cargo.toml`) to the same scratch tree and re-ran:

```
🔍 Checking per-crate CHANGELOG.md coverage in <scratch>/crates ...
❌ Per-crate changelog check failed

FAIL: zero publishable crates discovered under crates/*/ -- this looks like a broken glob or an empty workspace, not success.
...
MODE4B_EXIT=1
```

The non-manifest directory is skipped silently and is *not* counted as a crate — the guard still reports "zero publishable crates discovered" rather than treating `not-a-crate` as a satisfied or missing crate.

**4c — the same non-manifest directory alongside one real crate (proves silent-skip independent of the zero-crates branch):**

Added `crates/real-crate/Cargo.toml` (no `CHANGELOG.md`) beside `not-a-crate/` and re-ran:

```
🔍 Checking per-crate CHANGELOG.md coverage in <scratch>/crates ...
❌ Per-crate changelog check failed

FAIL: 1 publishable crate(s) missing CHANGELOG.md:
  - real-crate
...
MODE4C_EXIT=1
```

Only `real-crate` is named; `not-a-crate` raises no error and is absent from both the publishable count and the offender list, confirming it is skipped rather than merely tolerated by the zero-crates branch.

Scratch directory removed after the run. No file inside this git worktree was touched by mode 4.

### Post-fixture guard state

`bash -n scripts/check-changelogs.sh` → exit 0. `grep -c 'exit 0' scripts/check-changelogs.sh` → `1`. `./scripts/check-changelogs.sh` (final run) → exit 0, `git status --porcelain crates/` empty.

## Issues Encountered

None. All four failure modes tripped correctly on the first implementation; no repair cycle was needed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The failable-guard idiom (bash `if/else`, single `exit 0`, python3/tomllib heredoc for structured parsing, `command -v python3` presence guard) is proven end-to-end and ready for plans 09-04 (crate-name guard) and 09-06 (advisory-register guard) to copy.
- `crates/paladin-herald/` now has 10 of 10 publishable crates covered by a changelog; `make check-changelogs` and the CI `cargo-deny` job both enforce it going forward.
- No blockers for wave 2 (plan 09-02, the RustSec exception register, which shares no file with this plan).

## Self-Check: PASSED

- FOUND: `crates/paladin-herald/CHANGELOG.md`
- FOUND: `scripts/check-changelogs.sh`
- FOUND: `.planning/phases/09-release-security-gate-integrity/09-01-SUMMARY.md`
- FOUND commit: `0458b6a`

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
