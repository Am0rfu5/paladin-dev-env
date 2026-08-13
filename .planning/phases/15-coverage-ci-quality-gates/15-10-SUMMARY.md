---
phase: 15-coverage-ci-quality-gates
plan: 10
subsystem: infra
tags: [requirements, ledger, roadmap, adr-0006, adr-0034, documentation-correction]

# Dependency graph
requires:
  - phase: 15-coverage-ci-quality-gates (plans 15-01 through 15-09)
    provides: "every commit, figure and finding this plan transcribes at source: the coverage/cli-tests/bench-check CI jobs, ADR-0006's Phase 15 amendment, the corrected instruction files, src/test_support/, and the user_service.rs/listener.rs coverage measurements"
provides:
  - "eight amended requirement texts (PIPE-01…05, DEFER-01…03) in .planning/REQUIREMENTS.md, each dated, original retained, checked off with command/file:line evidence"
  - "ten closed ledger rows in .planning/ledgers/milestone-09-12.md plus the head-note's re-counted 19-job ci.yml list"
  - "the bidirectional return pointer in .planning/ledgers/milestone-02-03.md closing the Epic 24 cluster 8.0 deferral"
  - "three corrected ROADMAP.md Phase 15 success criteria (1, 3, 6), scoped edits only, no other phase entry touched"
  - "three annotated .project/ source documents (the parent PRD, the Epic 25 PRD, DEFERRED_COVERAGE.md) with dated correction banners, originals retained"
  - "the ci.yml native-arm64 owner comment reassigned away from the now-closed Phase 15, plus a new ledger section recording both of Phase 15's declines"
affects: [16-docs-update]

tech-stack:
  added: []
  patterns:
    - "D-00c/D-00d dated-banner annotation, applied consistently across REQUIREMENTS.md requirement texts, ledger rows, ROADMAP criteria and .project/ source documents — never a separate corrections file"

key-files:
  created: []
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/ledgers/milestone-09-12.md
    - .planning/ledgers/milestone-02-03.md
    - .planning/ROADMAP.md
    - .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md
    - .project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md
    - .project/Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md
    - .github/workflows/ci.yml

key-decisions:
  - "Every figure and line-number citation was re-grepped against the live tree at execution time rather than copied from a planning document, per the plan's own 'Re-grep everything' objective — this caught that PIPE-01's inherited '43 total' snapshot-test figure was itself stale (86 snapshots / 97 tests across seven files, not four), matching the ledger's own REQ-ci-cli-snapshot-job row, which carried the identical stale arithmetic and was corrected in the same pass."
  - "All eight requirement checkboxes were flipped to [x] on this pass (PIPE-01, PIPE-02, DEFER-01, DEFER-02, DEFER-03 were still [ ]; PIPE-03/04/05 were already [x]) because every plan summary from 15-01 through 15-09 records requirements-completed evidence for all eight, and D-00e's evidence bar (an exact command or file:line per checked box) is met by CI run 31727496744 and the cited plan commits for every one."
  - "The native-arm64 CI rework's ci.yml comment was edited to reassign its owner rather than deleted, per the plan's explicit instruction — 'edit that comment in place to reassign the owner... record the reassignment in the ledger so the pointer does not go on naming a phase that has closed.' A new ledger section (milestone-09-12.md, 'Phase 15 declines') is the reassignment's paper trail."
  - "Per the orchestrator's shared_artifact_boundary and build_verification_policy for this worktree, no ROADMAP.md tracking machinery (plan checkboxes, the Progress table, phase status) was touched, and no cargo build/test/clippy was run — this plan is documentation-only (one comment-only edit to ci.yml, otherwise markdown/REQUIREMENTS.md prose), verified instead by re-parsing all six workflow files with python3's yaml.safe_load and confirming git diff --stat on .project/ shows only additions."

patterns-established: []

requirements-completed: [PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, DEFER-01, DEFER-02, DEFER-03]

coverage:
  - id: D1
    description: "Eight requirement texts (PIPE-01…05, DEFER-01…03) amended at source in .planning/REQUIREMENTS.md, dated, originals retained and marked superseded, each checked off with a re-verified command or file:line"
    requirement: "PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, DEFER-01, DEFER-02, DEFER-03"
    verification:
      - kind: other
        ref: "git show 888492c --stat -- .planning/REQUIREMENTS.md; python3 anchor-presence check (all eight IDs + ADR-0006/ADR-0034/testing-guide.md/src/test_support/listener.rs/actionlint tokens) — pass, per plan Task 1's own automated verify"
        status: pass
    human_judgment: false
  - id: D2
    description: "Ten milestone-09-12.md ledger rows closed with dated verdicts naming what shipped and its commit; REQ-ci-cli-snapshot-job's own stale '43' arithmetic corrected in place; the head-note's ci.yml job count updated to 19; milestone-02-03.md's Epic 24 cluster 8.0 row and Phase 6 CLOSE-02 scope section both close the cli-tests/bench-check/coverage/.codecov.yml deferral bidirectionally"
    requirement: "PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, DEFER-01, DEFER-02, DEFER-03"
    verification:
      - kind: other
        ref: "git show 5f1f378 --stat; python3 row-presence check for all ten REQ-* ids — pass, per plan Task 2's own automated verify"
        status: pass
    human_judgment: false
  - id: D3
    description: "ROADMAP.md Phase 15 criteria 1, 3 and 6 amended in place (stale 43-snapshot figure, superseded threshold framing now pointing at ADR-0006, ambiguous 'covered to the gate' language resolved to the ≥80% module bar as a phase-acceptance criterion not a standing CI gate); no other phase entry touched, phase count unchanged (17 before and after)"
    verification:
      - kind: other
        ref: "git diff --stat .planning/ROADMAP.md (single hunk, lines 905-942); grep -c '^### Phase ' .planning/ROADMAP.md == 17 == git show HEAD:.planning/ROADMAP.md count"
        status: pass
    human_judgment: false
  - id: D4
    description: "Three .project/ source documents annotated with dated correction banners (additions only, git diff --stat confirms zero deletions); both Open Question 3s (parent PRD, Epic 25 PRD) recorded answered; DEFERRED_COVERAGE.md's three unchecked prerequisites each closed with named evidence"
    verification:
      - kind: other
        ref: "git show fc76249 --stat -- .project/; git diff --stat .project/ shows 81 insertions(+), 0 deletions(-) across three files"
        status: pass
    human_judgment: false
  - id: D5
    description: "The ci.yml 'Owner: Phase 15 / PIPE' comment reassigned to a future infrastructure phase (comment-only edit, all six workflow files still parse); both of Phase 15's declines (native-arm64 rework, second feature-scoped coverage measurement) recorded with a reason, owner and reopening condition in a new milestone-09-12.md section"
    verification:
      - kind: other
        ref: "grep -c 'Owner: Phase 15 / PIPE' .github/workflows/ci.yml == 0; python3 yaml.safe_load over all six .github/workflows/*.yml files — all pass; git diff .github/workflows/ci.yml shows a comment-only 6-line addition"
        status: pass
    human_judgment: false

duration: ~2h
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 10: Requirements, Ledger and Source-Document Corrections Summary

**Amended eight requirement texts, ten ledger rows, three ROADMAP criteria and three `.project/` source documents at source — correcting eleven false premises this phase found (stale snapshot/action counts, a superseded coverage-threshold framing, two stale file paths, a dissolved sequencing collision) and recording every verdict, open-question answer and decline this phase's nine prior plans left scattered across their summaries.**

## Performance

- **Duration:** ~2h (dominated by re-grepping every citation against the live tree before writing it)
- **Completed:** 2026-08-13T20:12:06Z
- **Tasks:** 3 of 3
- **Files modified:** 8 (`.planning/REQUIREMENTS.md`, `.planning/ledgers/milestone-09-12.md`, `.planning/ledgers/milestone-02-03.md`, `.planning/ROADMAP.md`, three `.project/` documents, `.github/workflows/ci.yml`)

## Accomplishments

- **Corrected all eleven false premises this phase found, at their source, dated, with originals retained (D-00c/D-00d):**
  1. **PIPE-01's "43 total" CLI snapshot figure** — the real count, re-measured, is **86 snapshot files / 97 test functions across seven files** (the original "43" counted four of the seven `tests/cli/*.rs` files that carry `#[test]`). Corrected in `REQUIREMENTS.md`, `milestone-09-12.md`'s `REQ-ci-cli-snapshot-job` row (whose own "sums to 43" arithmetic carried the identical bug), and `ROADMAP.md` criterion 1.
  2. **PIPE-02's coverage-threshold framing** — replaced with a pointer to `ADR-0006`'s Phase 15 amendment (82% floor, derived fresh from an 82.39% measurement) rather than restating a number; `--all-features` recorded rejected in favor of `--features integration-tests`; the stale `cargo-llvm-cov@0.7.1` pin corrected to the shipped `@0.8.7`.
  3. **PIPE-04's three defects** — the cited line numbers were stale (corrected, with a note that any line-number citation goes stale the moment the file next changes); the upgrade count is **seven, not eight** (`codecov/codecov-action@v3` was deleted, not upgraded); "all three workflows" is wrong — **six** workflow files exist, all linted by a standing `actionlint` job, not a one-time check.
  4. **PIPE-05's `CONTRIBUTING.md` premise** — corrected as relocated-by-outcome to `docs/src/contributing/testing-guide.md`; the `cargo tarpaulin` references it names were never in contributor docs at all, only in `.planning/codebase/TESTING.md`.
  5. **DEFER-01's `tests/common/` placement** — corrected as stale-by-structure (both coverage requirements test from inside `src/` via co-located modules, a separate `tests/` crate could never serve them); Open Question 2 answered (hand-written, `mockall` not adopted); a five-name mock verdict table added, reconciled with the ledger's own `MockLogPort` present-but-unshared finding.
  6. **DEFER-02's recorded profile** — 583 lines (not 488) at ADR-0034's citation, grown to 1628 by this phase's own test additions; the split-versus-test collision corrected as dissolved by ADR-0034 (not a live sequencing question); every observed-versus-assumed difference and production finding from plans 15-06/15-07 recorded with an owner.
  7. **DEFER-03's stale path and baseline** — the path was already self-corrected in the requirement's own prior text; the 57.83%/2026-02-14 baseline is now superseded by a real re-measurement (NOT MEASURED entry attempt, 96.90% exit figure); effort re-derived to low single-digit hours, superseding the inherited 20-25h.
  8. **`milestone-09-12.md`'s own stale figures** — the 15-job `ci.yml` head-note count updated to 19 (four Phase 15 additions: `actionlint`, `coverage`, `cli-tests`, `bench-check`).
  9. **`ROADMAP.md`'s three superseded criteria** (1, 3, 6) — amended in place, scoped edits only, no other phase entry touched (verified: 17 `### Phase ` headers before and after).
  10. **The two Deferred-QA PRDs' Open Question 3s** — both recorded answered (hard-fail from the first run; `integration-tests.yml`'s coverage step removed).
  11. **`DEFERRED_COVERAGE.md`'s three unchecked prerequisites** — each closed with named evidence (`src/test_support/`, `testing-guide.md`, `listener.rs`'s four concurrency tests) rather than left as unchecked boxes with satisfied content.
- **Recorded DEFER-01's five-name mock verdict table**, reconciling every name (`MockUserRepository`, `MockLogPort`, `MockNotificationService`, `MockEventSource`, `MockTriggerExecutor`) against what plans 15-06 through 15-09 actually consumed — none silently dropped.
- **Closed the ten `milestone-09-12.md` ledger rows** and added the bidirectional return pointer in `milestone-02-03.md`'s Epic 24 cluster `8.0` row and `### Phase 6 CLOSE-02 scope` section, naming the four plans that shipped what Phase 6 deferred.
- **Annotated three `.project/` source documents** (the parent PRD, the Epic 25 PRD, `DEFERRED_COVERAGE.md`) with dated correction banners — additions only, confirmed via `git diff --stat` (81 insertions, 0 deletions across the three files).
- **Reassigned the `ci.yml` native-arm64 owner comment** away from the now-closed Phase 15 (comment-only edit; all six workflow files re-confirmed to parse), and **recorded both of Phase 15's declines** (the native-arm64 rework, a second feature-scoped `minio.rs` coverage measurement) in a new `milestone-09-12.md` section, each with a reason, an owner and a reopening condition.

## Task Commits

Each task was committed atomically:

1. **Task 1: Amend the eight requirement texts at source** - `888492c` (docs)
2. **Task 2: Amend the ledger rows and the ROADMAP criteria** - `5f1f378` (docs)
3. **Task 3: Annotate the source documents and record the declines** - `fc76249` (docs)

**Plan metadata:** this SUMMARY commit (below)

## Files Created/Modified

- `.planning/REQUIREMENTS.md` — All eight `PIPE-*`/`DEFER-*` requirement texts amended in place with dated Phase 15 correction blocks, checked off `[x]`; the Hand-off to Phase 15 block's five numbered items each given a disposition; the traceability table's eight rows flipped to `Complete`
- `.planning/ledgers/milestone-09-12.md` — All ten `REQ-*` rows named in the plan closed with dated verdicts; `REQ-ci-cli-snapshot-job`'s own stale arithmetic corrected; head-note job count updated to 19; new "Phase 15 declines" section added
- `.planning/ledgers/milestone-02-03.md` — Epic 24 cluster `8.0` row and `### Phase 6 CLOSE-02 scope` section both closed with the bidirectional return pointer
- `.planning/ROADMAP.md` — Phase 15 criteria 1, 3 and 6 amended in place, original text retained, no other phase entry touched
- `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` — Dated correction banner added at the document head; OQ-3 row annotated answered
- `.project/Deferred-QA-CICD-Completion/Epic_25/prd-cicd-pipeline-enhancement.md` — Dated correction banner added; OQ-3 row annotated answered
- `.project/Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md` — Module 2 re-measurement note added; all three unchecked prerequisites closed with named evidence
- `.github/workflows/ci.yml` — The "Owner: Phase 15 / PIPE" comment reassigned to a future infrastructure phase (6-line comment-only addition)

## Decisions Made

- **Re-grepped every citation against the live tree rather than trusting a planning-document copy**, per the plan's own "Re-grep everything" objective — this is what surfaced that the ledger's `REQ-ci-cli-snapshot-job` row carried the identical stale "43" arithmetic as the requirement text, rather than being independently correct.
- **Flipped all eight requirement checkboxes to `[x]`** on this pass. Five (`PIPE-01`, `PIPE-02`, `DEFER-01`, `DEFER-02`, `DEFER-03`) were still `[ ]` despite every consuming plan (15-01 through 15-09) recording `requirements-completed` evidence for them; each is now checked with an exact command or `file:line` citation, per D-00e.
- **Edited the `ci.yml` native-arm64 comment in place to reassign its owner**, rather than deleting it, per the plan's explicit instruction — deletion would have dropped a real, documented advisory finding; reassignment keeps the pointer live and points at the reopening condition instead of naming a phase that has closed.
- **Did not touch ROADMAP.md's tracking machinery** (checkboxes, the Progress table, phase status/completion markers) — the shared_artifact_boundary explicitly reserves that to the orchestrator; the `**Plans:**` line and plan list acceptance criterion was already satisfied by the existing text ("10 plans in 8 waves", `15-01-PLAN.md` through `15-10-PLAN.md` already named) with no edit needed.
- **Ran no `cargo build`/`test`/`clippy`** — this plan touches zero `.rs` files (one comment-only line in `ci.yml`, otherwise markdown and `REQUIREMENTS.md` prose); verified instead by re-parsing all six workflow YAML files with `yaml.safe_load` and confirming `git diff --stat` on the `.project/` documents shows only additions, per this worktree's `build_verification_policy`.

## Deviations from Plan

None — plan executed exactly as written across all three tasks. Every correction, verdict and decline named in the plan's `<action>` blocks was applied; no Rule 1-4 auto-fix was needed because this plan's own scope (documentation correction, one comment reassignment) left no room for a code bug, missing functionality, or blocking issue to surface.

## Issues Encountered

None — all nine prior plan summaries and ADR-0006 provided complete, internally consistent figures to transcribe; no contradiction was found between what a plan summary claimed and what re-grepping the live tree confirmed, beyond the already-known "43 total" staleness the plan's own action text flagged in advance.

## Known Stubs

None — this plan is documentation-only; no hardcoded empty values, placeholder text, or unwired data sources were introduced.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

Phase 15 is closed. Every requirement (`PIPE-01`…`05`, `DEFER-01`…`03`) is checked off with command/file:line evidence; every ledger row this phase's requirements derive from is closed; ROADMAP's Phase 15 entry states only what actually shipped; and the two items Phase 15 declined (native-arm64 CI rework, a second feature-scoped coverage measurement) are recorded rather than silently absent.

**For Phase 16's DOCS-01 planner:** this phase touched `docs/src/contributing/testing-guide.md` (Code Coverage section, plan 15-04) and left `docs/src/contributing/development-setup.md`'s stale 80%/70% coverage claim (lines 201-202) untouched — recorded as a Known Stub in `15-04-SUMMARY.md`, owner DOCS-01. This plan did not touch any of the fourteen `docs/src/user-guides/`, `docs/src/deployment/` or `docs/src/operations/` files DOCS-01 owns.

**Still genuinely open, not closed by this plan:** the local `make services-up` → `make coverage` walkthrough of the rewritten testing guide has never been run end-to-end by a human on a Docker-capable machine — tracked at `.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`, owner repo maintainer, no `resolves_phase` tag. Do not record this as closed.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*
