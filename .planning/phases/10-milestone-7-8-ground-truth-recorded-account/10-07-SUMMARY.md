---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 07
subsystem: docs
tags: [ledger, requirements-traceability, ci-cd, dependency-rule, milestone-7-8]

requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-01's ledger scaffold (86 row stubs, seven-class legend, supersession summary table) and 10-04's ADR-0031 (extracted-crate dependency rule restatement)"
provides:
  - "Milestone 7 Epic 1 (12 rows) and Epic 2 (13 rows) of .planning/ledgers/milestone-07-08.md fully derived, replacing scaffold pending stubs in place"
  - "REQ-extracted-crate-dependency-rule flipped from `Code diverges -> HARD-05` to `satisfied`, citing ADR-0031, with the wording-not-code explanation stated explicitly"
  - "REQ-docker-workspace-build recorded closed by Phase 9 (ADR-0027, commit 52b1943), citation re-run against the current Dockerfile.chef"
  - "Fresh cargo tree measurement confirming REQ-dependency-isolation-metrics' two PRD success metrics hold today"
affects: ["10-08", "10-09", "10-10", "10-11"]

tech-stack:
  added: []
  patterns:
    - "Cell-replacement-only ledger fan-out: two per-task commits inside one file's disjoint epic ranges, verified via grep -c row/section counts and git diff --numstat added==deleted before each commit"
    - "Variant-group cross-referencing where a row's own REQ-* ID does not literally appear in the head-of-file summary table (only its sibling ID sharing the same variant group does) — recorded as a finding rather than silently forced to match"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-07-08.md

key-decisions:
  - "REQ-extracted-crate-dependency-rule verdict flip is explained, not bare: ADR-0031 restates the rule against the default build; the tree conformed since the extraction shipped, so the divergence was always in the PRD's unconditional wording, never in the code"
  - "REQ-tensorflow-stays-facade-v1 marked superseded by outcome, but recorded that its own ID does not literally appear in the head-of-file supersession summary table -- only its variant-group sibling REQ-tensorflow-ml-feature-gate-v2 does. This plan's contract prohibits editing that table, so the cross-reference is by variant group (24), not literal ID match"
  - "REQ-ci-workspace-job and REQ-ci-integration-job marked diverged rather than satisfied: both ship functionally-equivalent CI coverage, but REQ-ci-workspace-job splits FR-24's single test-workspace job across two independent jobs (lint + test) with no needs: gating on crate-isolation, and REQ-ci-integration-job uses native GitHub Actions services: blocks (redis, minio only, no mysql) instead of FR-25's docker-compose form with no needs: gating on test-workspace"
  - "REQ-ci-publish-dry-run-v1 and REQ-ci-publish-dry-run-v2 kept as two separate rows with two separate diverged verdicts, per D-00f (REQ-* ID is the primary key): v1's per-crate dependency-ordered form ships in release.yml (tag-push/dispatch trigger, not push-to-main as its own FR specifies); v2's single-workspace form ships in ci.yml with FR-26's exact trigger/gating shape but a different publish mechanism"
  - "REQ-dependency-isolation-metrics resolved to satisfied by actually re-running the PRD's two named cargo tree commands this session (cargo tree -p paladin-ai-core --all-features and -p paladin-battalion --all-features, both offline), rather than inferring from the manifest"
  - "REQ-facade-workspace-metadata and REQ-extraction-order-and-shims both resolved to present, unproven rather than satisfied: each has one half proven (manifest membership; extraction commit order) and one half not exhaustively audited (the no-silent-removal re-export clause; the full shim/re-export protocol across all four extractions)"

requirements-completed: [HARD-01, HARD-05, HARD-07]

coverage:
  - id: D1
    description: "Milestone 7 Epic 1's twelve rows re-derived from the tree with file:line citations and named exercisers, replacing the scaffold's pending stubs in place"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 7 Epic 1/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 12; same range grep -c 'pending — plan' == 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQ-extracted-crate-dependency-rule flipped to satisfied citing ADR-0031, with the divergence-was-in-the-wording explanation stated in the row"
    requirement: "HARD-05"
    verification:
      - kind: other
        ref: "grep -c 'ADR-0031' .planning/ledgers/milestone-07-08.md >= 1; the REQ-extracted-crate-dependency-rule row contains both 'wording' and the ADR number"
        status: pass
    human_judgment: false
  - id: D3
    description: "Milestone 7 Epic 2's thirteen rows re-derived, every ci.yml citation re-grepped this session, the stale ci.yml:225 exclusion citation absent from the whole file"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "grep -c 'ci.yml:225' .planning/ledgers/milestone-07-08.md == 0; awk '/^### Milestone 7 Epic 2/{p=1;next}/^### /{p=0}p' | grep -c '^| REQ-' == 13"
        status: pass
    human_judgment: false
  - id: D4
    description: "Ledger row/section inventory unchanged by this plan's cell-replacement-only edits; each task's diff shows added lines equal to deleted lines"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md == 86; grep -c '^### ' == 12; git diff --numstat for each of the two task commits shows added == deleted (13/13, then 14/14)"
        status: pass
    human_judgment: false
  - id: D5
    description: "No .rs file modified by this plan (D-23 boundary held)"
    verification:
      - kind: other
        ref: "git status --porcelain -- '*.rs' — empty"
        status: pass
    human_judgment: false

duration: ~50min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 07: Milestone 7 Epic 1-2 Ledger Derivation Summary

**Derived all 25 Milestone 7 Epic 1/2 ledger rows from the tree — flipping the extracted-crate dependency rule to `satisfied` via ADR-0031, splitting two CI-job rows to `diverged` on mechanism mismatches, and re-grepping every `ci.yml` citation to retire the stale `:225`/`:644`/`:141` line numbers.**

## Performance

- **Duration:** ~50 min
- **Completed:** 2026-08-08
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-07-08.md`)

## Accomplishments

- Re-derived all twelve Milestone 7 Epic 1 rows from the current tree: confirmed the extraction order (storage→notifications→content→web) via extraction-commit timestamps, flipped `REQ-extracted-crate-dependency-rule` from `Code diverges → HARD-05` to `satisfied` citing ADR-0031 with the wording-not-code explanation, re-measured `REQ-dependency-isolation-metrics` by actually running the PRD's two named `cargo tree --all-features` commands, and recorded `REQ-paladin-storage-extraction` as `superseded by outcome` after tracing `file_content_repository.rs`'s deletion to a deliberate M8 dead-code commit rather than an omission.
- Re-derived all thirteen Milestone 7 Epic 2 rows: resolved five run-4 `Verify → HARD-01` stubs by checking each against the current `ci.yml`, `Makefile`, `feature-flags.yml` and `release.yml`; recorded `REQ-docker-workspace-build` closed by Phase 9 with its citation re-run against today's `Dockerfile.chef`; split `REQ-ci-workspace-job` and `REQ-ci-integration-job` to `diverged` after finding each ships via a materially different job structure than its own FR specifies; and kept the two publish-dry-run requirements as separate rows with separate `diverged` verdicts.
- Retired every stale run-4 `ci.yml` line-number citation in this range — the mid-220s doc-test exclusion, `:644`, and `:141` — none of which resolve against the file as it stands today; every citation in both epic sections was produced by a `grep -n` run this session.
- Confirmed the ledger's row/section inventory is unchanged: `grep -c '^| REQ-'` still reads `86`, `grep -c '^### '` still reads `12`, and each of the two task commits shows equal added/deleted line counts (13/13, then 14/14) — cell replacement only, no row inserted, deleted, or reordered.

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 7 Epic 1's twelve rows** — `bdb867d` (feat)
2. **Task 2: Derive Milestone 7 Epic 2's thirteen rows** — `6d3c335` (feat)

_No plan-metadata commit: this executor ran in worktree mode. STATE.md and ROADMAP.md are owned by the orchestrator after all wave-3 agents complete; this SUMMARY.md is committed separately per the worktree execution contract._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` — Milestone 7 Epic 1 (12 rows) and Epic 2 (13 rows) Verdict/Evidence cells replaced in place; epic notes filled for both sections. No other section touched.

## Decisions Made

- **REQ-extracted-crate-dependency-rule** verdict flip is explained, not bare: ADR-0031 restates the rule against the default build; the tree has conformed since the extraction shipped, so the divergence was always in the PRD's unconditional wording, never in the code. Re-confirmed via a fresh `cargo tree -p paladin-content --no-default-features` run this session (zero occurrences of any extracted crate or the facade).
- **REQ-tensorflow-stays-facade-v1** is `superseded by outcome`, but its own `REQ-*` ID does not literally appear in the head-of-file supersession summary table — only its variant-group sibling `REQ-tensorflow-ml-feature-gate-v2` does. Recorded as a finding (this plan's contract prohibits editing the summary table) rather than silently forcing a match.
- **REQ-ci-workspace-job** and **REQ-ci-integration-job** are `diverged`, not `satisfied`: both ship functionally-equivalent CI coverage but via a materially different job structure than their own FR text specifies (split jobs with no `needs:` gating vs. FR-24's single gated job; native `services:` blocks with no MySQL vs. FR-25's docker-compose form).
- **REQ-ci-publish-dry-run-v1** and **REQ-ci-publish-dry-run-v2** are kept as two separate rows with two separate `diverged` verdicts, per D-00f (the `REQ-*` ID is the primary key, never merged on shared citation): v1's mechanism ships in `release.yml` under a different trigger than its own FR specifies; v2's trigger matches FR-26 exactly but its mechanism (a single workspace-wide dry run) diverges from FR-26's per-crate loop.
- **REQ-dependency-isolation-metrics** resolved to `satisfied` by actually re-running the PRD's two named `cargo tree --all-features` commands this session, rather than inferring compliance from the manifest.
- **REQ-facade-workspace-metadata** and **REQ-extraction-order-and-shims** both resolved to `present, unproven`: each has one half proven (manifest membership; extraction-commit order) and one half not exhaustively audited (the no-silent-removal re-export clause; the full shim/re-export protocol across all four extractions).

## Deviations from Plan

None — plan executed exactly as written, including its acceptance criteria's stricter bar (e.g. the `ci.yml:225` string must appear zero times anywhere in the file, which required rephrasing two explanatory sentences that had quoted the stale citation verbatim while describing its staleness).

One self-correction during execution, not a deviation from the plan: the first draft of both epic notes was written as multi-line paragraphs, which broke the plan's "cell replacement only, added lines equal deleted lines" acceptance criterion (the diff showed 52 added / 27 deleted). Both epic notes were rewritten as single physical lines before committing; the final per-task diffs are exactly balanced (13/13, then 14/14).

## Issues Encountered

None beyond the self-correction above, caught by this plan's own `git diff --numstat` verification step before either commit.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None introduced by this plan. 57 of the ledger's 86 rows remain `pending — plan 10-NN` after this plan (82 pending after the wave-1 scaffold, minus the 25 this plan derived) — this is the documented wave-3 fan-out contract (10-08, 10-09, 10-10 derive the remaining sections in parallel), not a defect of this plan's own range.

## Next Phase Readiness

- Milestone 7 Epic 1 and Epic 2 are fully derived; plans 10-08, 10-09 and 10-10 can proceed independently over their own disjoint epic ranges (no file-content dependency between fan-out plans beyond the shared row/section-count invariant, which this plan leaves unchanged at 86/12).
- ADR-0031's `Downstream Consumers` section names Phase 11/FACADE-02 as depending on the restated dependency-rule invariant this plan's ledger row now cites as `satisfied` — Phase 11 can proceed against that answer.
- No blockers. No `.rs` file was touched; `git status --porcelain -- '*.rs'` is empty.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-07-08.md`
- FOUND: commit `bdb867d` (Task 1)
- FOUND: commit `6d3c335` (Task 2)

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
