---
phase: 11-facade-residue-deferred-register-disposition
plan: 03
subsystem: docs
tags: [adr, register, facade-cleanup, deferred-features, ml-port, cli-recovery]

# Dependency graph
requires:
  - phase: 11-01
    provides: "FACADE-01 D5 disposition; resolved ADR-allocation checkpoint fixing this plan's ADR number at 0035"
provides:
  - ".planning/registers/facade-03-removed-features.md — .planning-native record of both Milestone 8 removed features"
  - "ADR-0035 — the paladin-ml leaf-crate placement condition promoted out of DOC precedence"
  - "Dated banner on .project/deferred-features.md pointing at both new .planning/ homes and correcting the branch-vs-commit attribution"
affects: [11-05]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Shape-B blockquote annotation banner (D-00c)", "ADR posture-decision shape (no frontmatter, 7 fixed headings)"]

key-files:
  created:
    - .planning/registers/facade-03-removed-features.md
    - .planning/decisions/0035-paladin-ml-leaf-crate-placement.md
  modified:
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md

key-decisions:
  - "ADR number 0035 taken per plan 11-01's resolved checkpoint (option-a); PROMOTION.md's Next free ADR number line was re-read at execution time and still showed 0034 (plan 11-02's number, running in parallel) -- this plan does not amend PROMOTION.md, per D-14 step 5 / plan 11-05 ownership"
  - "Re-measured the branch claim rather than propagating it: no local branch chore/facade-cleanup-m8-finish exists, but the remote-tracking ref does resolve and is an ancestor of 3d48768 -- corrected the stronger 'not present as a local or remote ref' claim inherited from 11-CONTEXT.md D-10 and deferred-features.md, in both new/annotated files"
  - "One commit (3d48768) removed both features -- corrected the corpus's split attribution (CLI to a branch, ML to a commit) at source in both the new register and the deferred-features.md banner"

requirements-completed: [FACADE-03]

coverage:
  - id: D1
    description: "Both removed features (paladin user CLI, TensorFlow ML adapter) recorded in a .planning/-native register with reintroduction conditions, recovery pointer and the honestly measured branch state"
    requirement: "FACADE-03"
    verification:
      - kind: other
        ref: "grep -qF checks against .planning/registers/facade-03-removed-features.md for 3d48768, the runnable recovery command, ml_port, paladin-ml, leaf crate, 1,065, 636, and all eight subcommand names -- all pass"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0035 promotes the paladin-ml leaf-crate placement condition out of DOC precedence, with the seven required headings, bulleted Considered Options / Code Locations, verbatim condition text, and a conforms verdict"
    requirement: "FACADE-03"
    verification:
      - kind: other
        ref: "diff of grep '^## ' output against the seven required headings in order (clean); awk bullet counts (4 Considered Options, 6 Code Locations); grep checks for 'leaf crate', 'never back into the facade', 'MlPort', 'stays in the workspace', 'paladin-herald' (6x), 'deferred-features.md', '^conforms' (1x) -- all pass"
        status: pass
    human_judgment: false
  - id: D3
    description: "deferred-features.md annotated with exactly one dated banner pointing at both .planning/ homes and correcting the branch-vs-commit attribution, with zero deletions to the original text"
    requirement: "FACADE-03"
    verification:
      - kind: other
        ref: "git diff --numstat HEAD~1 HEAD -- deferred-features.md reports 23 insertions, 0 deletions; grep checks for facade-cleanup-m8-finish, facade-03-removed-features, 0035-paladin-ml-leaf-crate-placement, 3d48768, 2026-08-08, FACADE-03 (all present, banner sentence count exactly 1); git diff --name-only lists exactly one .project/ file"
        status: pass
    human_judgment: false

# Metrics
duration: ~35min
completed: 2026-08-09
status: complete
---

# Phase 11 Plan 03: Facade Residue Removed-Features Disposition Summary

**Recorded the `paladin user …` CLI surface and TensorFlow ML adapter as deliberate M8 deferrals in `.planning/`, with an immutable SHA recovery pointer, and promoted the `paladin-ml` leaf-crate placement condition into ADR-0035.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-09T00:00:00Z (approx)
- **Completed:** 2026-08-09T00:11:37Z
- **Tasks:** 3 completed
- **Files modified:** 3 (2 created, 1 annotated)

## Accomplishments

- Wrote `.planning/registers/facade-03-removed-features.md`, the `.planning`-native answer to "why can I not run `paladin user register`?" — LOC figures, subcommand list, backend-intact citation, and a recovery command addressed by the immutable SHA `3d48768`, not a branch.
- Wrote ADR-0035, promoting the `paladin-ml` leaf-crate placement condition out of DOC precedence, with the condition reproduced verbatim, the asymmetric non-goal split (overridden for `paladin-herald`, still holding for `paladin-ml`) stated in both directions, and an explicit "creates no crate" clause.
- Annotated `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` with a dated Shape-B banner pointing at both new `.planning/` homes and correcting the branch-versus-commit attribution defect, purely additively (0 deletions).
- Re-measured the branch claim rather than propagating an inherited falsehood: confirmed no local `chore/facade-cleanup-m8-finish` branch exists, but the remote-tracking ref does resolve and is an ancestor of the removal commit — recorded honestly in both new/annotated files instead of asserting a blanket absence.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write the `.planning/` register for both removed features** - `f795163` (docs)
2. **Task 2: Write ADR-0035 — the `paladin-ml` leaf-crate placement condition** - `33f5fbc` (docs)
3. **Task 3: Annotate `deferred-features.md` with the `.planning/` pointer and the attribution correction** - `bc03b07` (docs)

_No TDD tasks in this plan — D-13 forbids executable `.rs` changes; all three tasks are documentation-only._

## Files Created/Modified

- `.planning/registers/facade-03-removed-features.md` - New register: both removed features, reintroduction conditions, recovery pointer, honestly measured branch state
- `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` - New ADR-0035: the `paladin-ml` placement condition promoted out of DOC precedence
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` - Dated Shape-B banner added; all original text (including the `:72` branch reference) retained unmodified

## Decisions Made

- **ADR number 0035, not re-derived from `PROMOTION.md` at execution time.** `PROMOTION.md`'s "Next free ADR number" line still read `0034` when re-checked this session — that is plan 11-02's number (D1-D4 disposition set), running in parallel. Per the orchestrator's resolved checkpoint (plan 11-01, option-a), this plan's number is fixed at 0035 regardless of what the shared index currently shows. `PROMOTION.md` itself is not amended by this plan — that is plan 11-05's step, per D-14 step 5.
- **The branch-absence claim inherited from `11-CONTEXT.md` D-10 and `deferred-features.md`'s own original text was re-measured, not propagated.** `git branch --list '*facade-cleanup-m8-finish*'` returns zero local matches (that half holds), but `git rev-parse --verify refs/remotes/origin/chore/facade-cleanup-m8-finish` resolves to `4bf6745…` and `git merge-base --is-ancestor 3d48768 …` succeeds (the "no remote ref" half does not hold). Both new/annotated files record the true measured state and re-ground the SHA recommendation in immutability-versus-mutability rather than in a false absence.
- **One commit removed both features.** `git show --stat 3d48768` confirms `user.rs` and `tensorflow_adapter.rs` were deleted in the same commit — recorded explicitly in both new files to correct the corpus's split attribution (CLI-to-branch, ML-to-commit).

## Deviations from Plan

None - plan executed exactly as written. All three tasks' `<verify>` and `<acceptance_criteria>` blocks pass as specified.

## Issues Encountered

None. The sandboxed Bash tool rejected a small number of multi-command/piped invocations as "too complex to verify worktree containment" (e.g. combined `test -d ... ; echo $?` and `for` loops with `grep`); each was re-run as separate single-purpose commands with identical verification coverage — no impact on the plan's substance or its verification bar.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 11-05 can now cite ADR-0035 when amending the `REQ-deferred-tensorflow-ml-adapter-v3` ledger row and adding this plan's `PROJECT.md` `## Key Decisions` row.
- Plan 11-05 owns updating `PROMOTION.md`'s numbering index (adding the 0035 row) and its "Next free ADR number" line (advancing past 0035) — not performed here per D-14 step 5.
- No blockers. This plan changed zero `.rs` files (confirmed: `git diff --name-only HEAD~3 HEAD | grep -c 'rs$'` → `0`), consistent with D-13.

---
*Phase: 11-facade-residue-deferred-register-disposition*
*Completed: 2026-08-09*
