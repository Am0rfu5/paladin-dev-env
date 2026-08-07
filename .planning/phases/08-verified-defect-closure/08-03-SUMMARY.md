---
phase: 08-verified-defect-closure
plan: 03
subsystem: testing
tags: [rustdoc, doctest, ci, paladin-ports, cargo]

# Dependency graph
requires:
  - phase: 08-verified-defect-closure
    provides: "08-02 (file-serialization only — no content dependency; DEBT-01's api-surface job edits ci.yml:172,182,187, this plan edits ci.yml:226, zero line contention)"
provides:
  - "paladin-ports' ~25 port traits' rustdoc examples execute in CI and locally (96 doctests)"
  - "cargo test --workspace --doc runs with no --exclude flag"
  - "HARD-07 seam recorded for Phase 10: doctest execution and the cargo doc warning-bar question are separable"
affects: [phase-10-hard-07, phase-16-docs-03]

# Tech tracking
tech-stack:
  added: []
  patterns: ["doctest re-enablement via manifest flag removal (no code changes needed when justification is stale)"]

key-files:
  created: []
  modified:
    - crates/paladin-ports/Cargo.toml
    - .github/workflows/ci.yml

key-decisions:
  - "Removed [lib] doctest = false and its stale Task 7.0 comment — the circular-dev-dependency problem it described no longer exists (D-09)."
  - "Both manifest and CI edits landed in one commit (2bffe22) per D-11 — no window where doctests exist but CI still excludes them."
  - "No example repair was needed — the crate-scoped and workspace-scoped runs both confirm 96 passed, 0 failed, matching the research-measured baseline exactly."

patterns-established: []

requirements-completed: [DEBT-03]

coverage:
  - id: D1
    description: "crates/paladin-ports/Cargo.toml no longer sets doctest = false; ci.yml no longer passes --exclude paladin-ports to the workspace doctest run — both landed in one commit"
    requirement: "DEBT-03"
    verification:
      - kind: unit
        ref: "grep -c 'doctest' crates/paladin-ports/Cargo.toml (returns 0); grep -c 'exclude paladin-ports' .github/workflows/ci.yml (returns 0); git log -1 --name-only 2bffe22 (lists both files)"
        status: pass
    human_judgment: false
  - id: D2
    description: "paladin-ports' ~25 port traits' rustdoc examples compile and execute at crate scope (cargo test --offline -p paladin-ports --doc)"
    requirement: "DEBT-03"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-ports --doc"
        status: pass
    human_judgment: false
  - id: D3
    description: "The workspace doctest run (cargo test --offline --workspace --doc, the exact command CI now runs) includes paladin-ports' contribution, proven by comparison against the same run with --exclude paladin-ports"
    requirement: "DEBT-03"
    verification:
      - kind: unit
        ref: "cargo test --offline --workspace --doc (281 total passed) vs cargo test --offline --workspace --doc --exclude paladin-ports (185 total passed) — difference is exactly 96, paladin-ports' own count"
        status: pass
    human_judgment: false
  - id: D4
    description: "Workspace gate passes with no regressions from this change"
    requirement: "DEBT-03"
    verification:
      - kind: unit
        ref: "cargo test --offline --workspace; cargo fmt --check; cargo clippy --workspace --offline -- -D warnings"
        status: pass
    human_judgment: false

duration: 46min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 03: paladin-ports Doctest Re-enablement Summary

**Removed the stale `[lib] doctest = false` flag and CI's `--exclude paladin-ports`, in one commit — the ~25 port traits' 96 rustdoc examples now compile and execute in CI and locally, with zero example repair needed.**

## Performance

- **Duration:** 46 min
- **Started:** 2026-08-07T13:47Z (approx, first commit)
- **Completed:** 2026-08-07T14:34Z
- **Tasks:** 2
- **Files modified:** 2 (`crates/paladin-ports/Cargo.toml`, `.github/workflows/ci.yml`)

## Accomplishments

- `crates/paladin-ports/Cargo.toml`'s `[lib]` section (the `doctest = false` flag and its four-line stale "Task 7.0" comment) removed entirely.
- `.github/workflows/ci.yml`'s `Run doc tests` step (`:226`) changed from `cargo test --workspace --doc --exclude paladin-ports` to `cargo test --workspace --doc` — both edits in one commit per D-11.
- Confirmed at crate scope and at workspace scope that the doctests execute and pass, with the workspace-scoped total demonstrably including `paladin-ports`' 96 doctests (not merely a YAML change with no effect).
- No example repair, re-fencing, or `ignore`-labelling was required — D-09's "measured, not estimated" premise held exactly: the stale justification (doc examples referencing the root `paladin::` crate) was already fixed by whoever rewrote the examples to `paladin_ports::` / `paladin_core::` paths.
- Recorded the HARD-07 seam for Phase 10 and the three ledger rows plan 08-09 must amend.

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove doctest = false and the CI exclusion in one commit** - `2bffe22` (feat)
2. **Task 2: Prove the doctests run at workspace scope and record the HARD-07 seam** - no source-file commit (this SUMMARY is the deliverable; no code changed)

**Plan metadata:** committed by orchestrator after wave completion (per parallel-executor instructions, this worktree agent does not update STATE.md/ROADMAP.md)

## Files Created/Modified

- `crates/paladin-ports/Cargo.toml` - Removed the entire `[lib]` section (`doctest = false` plus its stale four-line comment); nothing else in the section, so full removal is correct.
- `.github/workflows/ci.yml` - Line 226's `Run doc tests` step: `cargo test --workspace --doc --exclude paladin-ports` → `cargo test --workspace --doc`.

## Evidence (D-00e / D-21 bar — verbatim command output)

### Crate-scoped run (Task 1's `<verify>`)

Command: `cargo test --offline -p paladin-ports --doc`

```
test result: ok. 96 passed; 0 failed; 94 ignored; 0 measured; 0 filtered out; finished in 0.03s

all doctests ran in 2.04s; merged doctests compilation took 2.01s
```

Matches the research-measured baseline (96 passed, 0 failed, 94 ignored) exactly.

### Workspace-scoped run (Task 2's `<verify>` — the exact command CI now runs)

Command: `cargo test --offline --workspace --doc`

`Doc-tests paladin_ports` section:

```
test result: ok. 96 passed; 0 failed; 94 ignored; 0 measured; 0 filtered out; finished in 0.02s

all doctests ran in 1.32s; merged doctests compilation took 1.30s
```

Full run exits 0 (all 11 crates' `Doc-tests` sections report `ok`).

### With/without `--exclude` total comparison (the direct falsifier per Task 2's instructions)

| Run | Command | Per-crate passed | Total passed |
|---|---|---|---|
| **Without `paladin-ports`** | `cargo test --offline --workspace --doc --exclude paladin-ports` | paladin: 96, paladin_core: 49, paladin_battalion: 28, paladin_content: 0, paladin_doc_examples: 0, paladin_herald: 0, paladin_llm: 4, paladin_memory: 8, paladin_notifications: 0, paladin_storage: 0, paladin_web: 0 | **185** |
| **With `paladin-ports`** (current CI command) | `cargo test --offline --workspace --doc` | same as above, plus paladin_ports: 96 | **281** |

Difference: **281 − 185 = 96**, exactly `paladin-ports`' own doctest count. This proves the exclusion removal changed what CI executes, not merely what the YAML says.

### One-commit proof (D-11)

Command: `git log -1 --name-only 2bffe22`

```
commit 2bffe22b6db18d1ba36d243e3dbb708db2e20876
feat(08-03): re-enable paladin-ports doctests
...
.github/workflows/ci.yml
crates/paladin-ports/Cargo.toml
```

Both files landed in the same commit — no window in which the crate's doctests exist and CI still refuses to run them.

### Workspace gate (CLAUDE.md)

- `cargo test --offline --workspace` — exit 0 (all crates' lib/bin/integration/doc tests pass, including the 96 `paladin-ports` doctests).
- `cargo fmt --check` — exit 0, no output (no formatting drift).
- `cargo clippy --workspace --offline -- -D warnings` — exit 0 (`Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1m 25s`, zero warnings).

### Acceptance-criteria greps (Task 1)

```
grep -c 'doctest' crates/paladin-ports/Cargo.toml          → 0
grep -c 'Task 7.0' crates/paladin-ports/Cargo.toml          → 0
grep -c 'exclude paladin-ports' .github/workflows/ci.yml    → 0
grep -n 'cargo test --workspace --doc' .github/workflows/ci.yml → 226 (exactly one line, no --exclude)
grep -n 'actions-rs/toolchain@v1' .github/workflows/ci.yml  → 148, 393, 792 (unchanged — Phase 15/PIPE-04 territory untouched)
grep -c 'doctest = false' crates/paladin-herald/Cargo.toml  → 1 (out-of-scope crate untouched)
```

### No fence re-labelled (D-10 prohibition, Task 2 acceptance criterion)

`grep -rn 'ignore' crates/paladin-ports/src --include='*.rs' | grep -c '```'` → 96 (this plan made zero edits to any file under `crates/paladin-ports/src/`, so this count is trivially unchanged from its pre-plan value — no fence was touched, re-labelled, or audited, consistent with D-10's prohibition and the plan's own "no example needed repair" outcome).

## Decisions Made

- **D-09 confirmed in production, not just research:** the crate-scope run reproduced the research-session measurement exactly (96/0/94), confirming the "already fixed" hypothesis held at execution time too — no drift between the research session (2026-08-06) and execution (2026-08-07).
- **D-11 applied literally:** both halves of the guard (manifest flag removal, CI exclusion removal) landed in commit `2bffe22`, verified via `git log -1 --name-only`.
- **D-10's contingency did not trigger:** the workspace-level run surfaced zero failures beyond the crate-level run — no example needed compiling, no fence needed `ignore`, no scope expansion occurred. The plan's own warning ("any DEBT-03 task list longer than ~2 tasks is a planning error") held; this plan stayed at 2 tasks.
- **D-12 respected:** the `cargo doc --workspace --no-deps` warning-bar question was not decided here — see the HARD-07 seam section below.

## Deviations from Plan

None - plan executed exactly as written. The plan's own prediction (D-09: "the answer may be 'they already pass'") held for both the crate-scoped and workspace-scoped runs; no example repair, no re-fencing, no scope expansion.

## Issues Encountered

None. The primary environment risk noted in the plan (workspace-level run surfacing failures the crate-level run did not) did not materialize — both runs report identical per-crate totals for `paladin-ports` (96 passed, 0 failed, 94 ignored).

## HARD-07 Seam (D-12) — recorded, not decided

DEBT-03's deliverable is *doctests executing*, and that deliverable is now proven at both crate and workspace scope. This plan does **not** decide which `cargo doc --workspace --no-deps` warning bar governs — that question belongs to **Phase 10 / HARD-07**. The current observable, so Phase 10 inherits a measured state rather than a surprise: the Milestone 4-6 ledger's `REQ-doc-build-clean` row records `cargo doc --offline --no-deps --workspace` completing with **6 warnings** across `paladin-battalion` (3) and `paladin-ai` (3), against M7 Epic 4 §4.4.3's "zero warnings" bar. This plan did not touch, close, or adjudicate those six warnings — they are named here and handed on to Phase 10 unchanged.

## Ledger Rows Plan 08-09 Must Amend (D-23)

This plan's evidence changes the verdict on three rows in `.planning/ledgers/milestone-04-06.md`. Plan 08-09 (the phase close-out plan) applies these amendments in place, dated, superseded text retained (D-00d) — **this plan does not touch the ledger**:

| Ledger row | Current verdict | Evidence produced this plan | Command |
|---|---|---|---|
| `REQ-ports-doctest-compilation` (:157) | `genuinely outstanding` | `paladin-ports` doctests now compile and execute: 96 passed, 0 failed, 94 ignored | `cargo test --offline -p paladin-ports --doc` |
| `REQ-ports-tests-and-rustdoc` (:160) | `present, unproven` | The "no documentation may be lost" clause is now proven — the doctest disablement that undermined it is removed and the examples execute | `cargo test --offline -p paladin-ports --doc` |
| `REQ-workspace-ci-upgrade` clause 3 (:225) | `deferred with reason` | The doctest exclusion is dropped; the workspace doctest run demonstrably includes `paladin-ports` (281 vs 185 passed, +96) | `cargo test --offline --workspace --doc` (compared against `--exclude paladin-ports`) |

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `paladin-ports`' port-trait rustdoc examples are executable and CI-enforced going forward — any future example that stops compiling will fail CI, closing the "documentation guard configured not to guard" gap DEBT-03 was scoped to close.
- **Phase 16 / DOCS-03** inherits executable port-trait examples as input for its documentation-quality work (auditing the 87 pre-existing `ignore`/`no_run`/`text` fences remains out of this plan's scope, per D-10).
- **Phase 10 / HARD-07** inherits the measured 6-warning `cargo doc --workspace --no-deps` state (see HARD-07 Seam above) without this plan deciding the warning bar.
- **Plan 08-09** (phase close-out) has the exact ledger rows and evidence commands it needs to amend `.planning/ledgers/milestone-04-06.md` (see table above).
- No blockers. This was the smallest plan in the phase and stayed that way — a two-line diff plus verification, as the plan predicted.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*
