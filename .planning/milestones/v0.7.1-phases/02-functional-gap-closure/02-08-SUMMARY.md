---
phase: 02-functional-gap-closure
plan: 08
subsystem: testing
tags: [garrison, prd-review, ledger-evidence, adr-0006, milestone-1]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-01's measured cargo test --workspace baseline (2790 passed / 0 failed / 126 ignored, commit 7e55655) — the corroborating source for every exerciser cited in this review, since this plan touches no Rust source and the tree is unmodified from that measurement"
provides:
  - "A written, per-criterion Epic 2 Garrison PRD-acceptance review (50 rows: 15 user-story bullets across Stories 1-5, plus FR1-FR10 split into 35 sub-clauses) at the D-19 evidence bar, closing task 11.6"
  - "Task 11.5's coverage check dispositioned superseded by shipped code, citing ADR-0006 and naming QUAL-01 as forward owner, per D-04"
  - "Two newly-surfaced, previously-undocumented gaps: PaladinError::GarrisonRequired is dead code (never constructed), and GarrisonSettings::validate() is disconnected from the config-loading path"
affects: [02-09-amend-ledger, phase-3-qual-work]

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created: [.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md]
  modified: []

key-decisions:
  - "Split each PRD Functional Requirement and User Story bullet into its own row (50 total) rather than one row per FR/Story, so a genuine per-sub-clause divergence (e.g. FR7.2 satisfied, FR7.3 genuinely outstanding, both under Story 5/FR7) could not be averaged away into a single verdict."
  - "Recorded PaladinError::GarrisonRequired as genuinely outstanding rather than present, unproven: the variant and its is_terminal() match arm exist, but grep across src and crates shows zero construction sites anywhere in the execution path — this is unimplemented behavior, not merely untested behavior."
  - "Recorded GarrisonSettings::validate() as present, unproven rather than satisfied: the method exists and is directly unit-tested, but returns Result<(), String> (not Result<(), GarrisonError>) and has no call site in the Settings-loading path, so the PRD's literal FR8.3 claim (invalid config -> GarrisonError::Configuration on load) is unexercised end-to-end."
  - "Sourced 'passing' confirmation for every cited exerciser from 02-01's already-measured cargo test --workspace baseline rather than re-running cargo test in this session, per this plan's own build-environment guidance restricting additional cargo builds in a disk-constrained worktree; corroborated by a direct #[ignore] grep across every Garrison test file (zero matches)."
  - "Did not assign a forward owner for the two newly-surfaced gaps (FR7.3/S5.3, FR8.3) inside the review itself — recorded that decision as belonging to plan 02-09, which amends the ledger from this review's verdicts."

patterns-established: []

requirements-completed: [GAP-06]

coverage:
  - id: D1
    description: "Epic 2 Garrison PRD-acceptance review written: one verdict per criterion (50 rows) at the D-19 evidence bar, with Task 11.5's coverage check dispositioned superseded by shipped code"
    requirement: "GAP-06"
    verification:
      - kind: other
        ref: "Task 1 <verify>: test -s ...review.md && wc -l >= 80 && zero coverage-percentage grep matches && grep -q '0006' ...review.md — all passed"
        status: pass
      - kind: other
        ref: "Orchestrator-independent verification of the two flagged gaps: test_sqlite_connection_pooling and test_paladin_with_garrison_stores_conversation confirmed present at their cited lines; PaladinError::GarrisonRequired confirmed to have no construction site; GarrisonSettings::validate() confirmed disconnected with Result<(), String> signature"
        status: pass
    human_judgment: false
  - id: D2
    description: "Human confirmed the review's verdicts are accurate and honest by spot-checking cited file:line/test pairs directly"
    verification: []
    human_judgment: true
    rationale: "Task 2 is this plan's own checkpoint:human-verify (gate=blocking) and this phase's only manual-only verification — the review's deliverable is a judgement recorded in prose, which only a human reading the cited evidence against the PRD can confirm is right, not merely present and well-formed."

duration: ~55min (executor work) + a paused interval awaiting human checkpoint review
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 08: Garrison PRD-Acceptance Review Summary

**Epic 2 Garrison PRD reviewed criterion-by-criterion against shipped code (50 rows, 37 satisfied / 9 superseded / 2 genuinely outstanding / 2 present-unproven), closing task 11.6 and dispositioning task 11.5's coverage check as superseded by ADR-0006**

## Performance

- **Duration:** ~55 min of executor work (source reading, evidence-gathering, review authoring, commit), across a session that paused for the plan's own human checkpoint
- **Started:** 2026-08-01T01:15:00Z (approx.)
- **Completed:** 2026-08-01T08:42:33Z (checkpoint approval and SUMMARY commit)
- **Tasks:** 2 (1 auto + 1 checkpoint:human-verify, gate="blocking")
- **Files modified:** 2 created (`02-garrison-prd-review.md`, this SUMMARY)

## Accomplishments

- Wrote `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md`: a per-criterion
  review covering both PRD criterion sets — all five user stories' acceptance-criteria bullets (15
  rows) and all ten functional requirements split into their PRD-numbered sub-clauses (35 rows) —
  50 rows total, each carrying exactly one of the ledger's five verdict classes.
- **Verdict arithmetic (carried forward verbatim for plan 02-09):** 37 `satisfied`, 9
  `superseded by shipped code`, 2 `genuinely outstanding`, 2 `present, unproven`, 0
  `deferred with reason`. Part 1 (User Stories): 11 satisfied / 3 superseded / 1 genuinely
  outstanding / 0 present-unproven = 15. Part 2 (Functional Requirements): 26 satisfied / 6
  superseded / 1 genuinely outstanding / 2 present-unproven = 35. 15 + 35 = 50, matching the
  table's row count exactly.
- **Surfaced FR7.3 / Story 5 AC3 — genuinely outstanding.** `PaladinError::GarrisonRequired`
  (`crates/paladin-core/src/platform/container/paladin_error.rs:54`) is defined and
  pattern-matched in `is_terminal()` (`:78`), but is never *constructed* anywhere in the tree —
  `grep -rn "GarrisonRequired" src crates` shows only the definition, its own unit test (`:111`),
  and one unrelated routing `match` arm in `crates/paladin-battalion/src/conclave_execution_service.rs:364`.
  No code path detects "multi-turn attempted without Garrison"; execution simply proceeds with an
  empty conversation history. **No forward owner is assigned** — the review states this belongs to
  plan 02-09's ledger-amendment decision, not to the review itself.
- **Surfaced FR8.3 — present, unproven.** `GarrisonSettings::validate()`
  (`crates/paladin-memory/src/config/garrison.rs:46`) exists and is directly unit-tested, but
  returns `Result<(), String>` rather than `Result<(), GarrisonError>`, and has zero call sites in
  the `Settings`-loading path (`grep -rn "\.validate()" src` confirms). An actually-invalid
  `garrison:` config block currently produces no error at load time. **No forward owner is
  assigned** by the review, for the same reason as above.
- Dispositioned task 11.5 (`- [ ] 11.5 Verify test coverage >= [PRD threshold] using cargo
  llvm-cov`) as **`superseded by shipped code`** in a dedicated section, citing ADR-0006's
  single workspace-wide floor and naming **QUAL-01** (Phase 3) as forward owner. The review
  produces no Garrison-scoped coverage figure of any kind, per D-04.
- Referenced (did not re-decide) the two PRD criteria the milestone-01 ledger's Divergences
  table already dispositions: the long-term/vector-search port shipping as Sanctum/Qdrant rather
  than the PRD's `sqlite-vss` SQLite extension (Story 4's three bullets, FR4.1-4.3, FR6.4,
  FR10.2e).
- Added a dedicated "Boundary, empty-input and tie-break notes" section addressing three review
  properties explicitly: (1) both windowing implementations use strict `>` comparisons, so an
  entry count or token total sitting exactly at the configured maximum triggers no eviction; (2)
  empty and single-entry conversations each have an explicit, cited passing verdict rather than
  being skipped as trivially true; (3) the in-memory adapter's eviction tie-break (oldest-first
  among equal-importance candidates) is specified by loop order, while the SQLite adapter's
  `ORDER BY timestamp DESC` has no secondary sort key, so ties there are SQLite-engine-emergent —
  a genuine, narrow divergence between the two adapters recorded rather than smoothed into one
  claim.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write the Epic 2 Garrison PRD-acceptance review** - `217d4e2` (docs)
2. **Task 2: Confirm the Garrison PRD-acceptance review** - checkpoint:human-verify, gate="blocking"
   — **approved** by the project user via the plan's resume signal ("approved"), after the
   orchestrator independently re-verified the two flagged-gap claims and the mechanical
   acceptance checks (199 lines, 0 coverage percentages, ADR-0006 cited, no Rust source modified).
   No code or record change accompanies this task; it is the human sign-off itself.

**Plan metadata:** this SUMMARY's own commit, made immediately after this file — see below.

## Files Created/Modified

- `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md` - Epic 2 PRD-acceptance
  review: 50-row criterion table (15 user-story bullets + 35 FR sub-clauses), a boundary/tie-break
  notes section, the Task 11.5 coverage-disposition section, a verdict-arithmetic summary, and a
  scope-boundary section naming what the review does not cover.
- `.planning/phases/02-functional-gap-closure/02-08-SUMMARY.md` - this file.

## Decisions Made

- Split each FR and Story bullet into its own row (50 total) rather than one row per FR/Story
  number, so genuine intra-requirement divergence (e.g. FR7 having both a `satisfied` sub-clause
  and a `genuinely outstanding` one) could not be averaged into a single verdict.
- Recorded `PaladinError::GarrisonRequired` as `genuinely outstanding`, not `present, unproven`:
  the variant exists but has zero construction sites, which is unimplemented behavior rather than
  merely untested behavior.
- Recorded `GarrisonSettings::validate()` as `present, unproven`: it is real, unit-tested code,
  but does not fulfil the PRD's literal end-to-end claim (wrong error type, no call site in the
  config-loading path).
- Sourced "passing" confirmation for every cited exerciser from Phase 2 Plan 01's already-measured
  `cargo test --workspace` baseline rather than re-running `cargo test` in this session, honoring
  this plan's own build-environment guidance restricting additional cargo builds in this
  disk-constrained worktree — corroborated by a direct `#[ignore]` grep across every Garrison
  source and test file cited (zero matches), and by locating every cited test by exact function
  name at its cited line.
- Left forward-owner assignment for the two newly-surfaced gaps (`GarrisonRequired`,
  `validate()`) to plan 02-09's ledger amendment rather than assigning one inside the review,
  since that is a ledger decision and this plan's job was surfacing the finding, not disposing of
  it.

## Deviations from Plan

None — plan executed exactly as written. Task 1's automated `<verify>` and all of its
`<acceptance_criteria>` greps passed on the first attempt (199 lines against an 80-line minimum,
35 `FR`-prefixed rows against a 10-row minimum, 15 story rows against a 5-row minimum, 24 distinct
`file:line` citations into the three named crates against an 8-citation minimum, exactly the five
allowed verdict tokens and no others, zero coverage-percentage matches, `0006` cited). Task 2's
human checkpoint was approved without requested revisions, and the orchestrator's independent
re-verification of the two flagged-gap claims and the mechanical checks confirmed all of them held
— no re-investigation or revision was needed per the resume instructions.

## Issues Encountered

- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  run against a cold `target/` directory exceeded the Bash tool's default 2-minute timeout on the
  first commit attempt. Resolved by retrying with an extended timeout (10 minutes); the hook then
  completed and the commit succeeded on the second attempt — no hook failure, no code change,
  purely a timing issue with the sandboxed tool call, consistent with this plan's own
  `<build_environment_notes>` warning that a cold cargo target directory can take 15+ minutes to
  compile.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Plan 02-09** has the evidence it needs to amend `REQ-garrison-testing` and its two nested
  items (`milestone-01.md:243-246`) from this review: the task 11.5 nested item closes as
  `superseded by shipped code` (ADR-0006, QUAL-01), and the task 11.6 nested item closes as
  `satisfied` by this review's existence and its D-19-bar contents. Plan 02-09 is also the
  natural place to assign forward owners for the two newly-surfaced gaps this review did not
  itself dispose of (`PaladinError::GarrisonRequired`'s dead construction path; the disconnected
  `GarrisonSettings::validate()`).
- **Phase 3 (QUAL-01)** inherits Garrison as part of its workspace-wide coverage-raising work,
  per this plan's Task 11.5 disposition — no Garrison-scoped coverage number exists anywhere in
  this phase's output for QUAL-01 to reconcile against.
- No blockers for the remaining Phase 2 wave: this plan touched no Rust source, so the tree the
  rest of Phase 2 builds on is unchanged from the tree this review was written against.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*
