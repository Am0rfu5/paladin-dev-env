---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 01
subsystem: docs
tags: [ledger, requirements-traceability, orchestrator, task-service, workflow-repository]

# Dependency graph
requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: milestone-07-08.md structural template (head notes, legend, contention table shape)
  - phase: 12-supply-chain-gate-integrity
    provides: SUPPLY-01/02/03 closures, 87-hit stale-citation inventory, D-09 four annotated .project/ files
provides:
  - .planning/ledgers/milestone-09-12.md — the fifth and final sibling ledger, 120 rows / 28 sections
  - Milestone 9 Epic 1's six rows fully derived to the D-00e evidence bar (real file:line + exerciser
    citations, one command actually run and its output recorded)
  - Ledger-file contention table fixing disjoint section ranges for plans 13-02..13-07
affects: [13-02, 13-03, 13-04, 13-05, 13-06, 13-07, 13-13]

# Tech tracking
tech-stack:
  added: []
  patterns: [ledger scaffold + fan-out contention table (Phase 7/10 precedent), pending-marker interim
    state for undeprived rows, run-5-input prefix for transcribed-but-not-re-derived rows]

key-files:
  created: [.planning/ledgers/milestone-09-12.md]
  modified: []

key-decisions:
  - "Milestone 9 Epic 1's three previously-bare-Verify rows (taskservice-dispatch, default-task-services-real-logic, workflow-lifecycle-integration-test) turned into real Shipped verdicts by reading acceptance criteria in intel/requirements.md, then grepping src/ and crates/ for the implementing symbol and its test"
  - "Actually ran `cargo test --test lib orchestrator_workflow_lifecycle` this session (1 passed) rather than citing the integration test unverified — satisfies D-00e's 'something that exercises it' bar with a live command, not just a file:line"
  - "REQ-workflow-repository-port's verdict records a consumer caveat found this session: no production binary or facade wires SqliteWorkflowRepository into a running Orchestrator — only a FakeWorkflowRepository test double is used in-tree. Recorded as a limitation of the Shipped verdict, not downgraded, because the adapter's own database tests exercise it directly"
  - "Section count is 28, not the plan's stated 29 — grep -c '^### ' against REQUIREMENTS.md:3607-3931 (ground truth) returns exactly 28 section headers, matching the ledger's own must_have of a one-for-one match. Documented as a deviation rather than forcing a fabricated 29th header"

requirements-completed: [ORCH-01, ORCH-02]

coverage:
  - id: D1
    description: "Fifth and final sibling ledger .planning/ledgers/milestone-09-12.md created with all head-note elements (supersession, primary key, evidence bar, verdict legend + seven-class mapping + tie-break, manifest carve-out, four re-grepped factual anchors, corrected 35/53/32 arithmetic, five-run checkbox pattern, precedence order, inherited stale-citation inventory, provenance paragraph)"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "test -f .planning/ledgers/milestone-09-12.md && grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md → 120"
        status: pass
      - kind: other
        ref: "grep -c 'understated → accurate → overstated → contradicted → vacuous' .planning/ledgers/milestone-09-12.md → 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 9 Epic 1's six rows derived end-to-end this session with file:line citations plus a live exerciser for each; the integration test was actually run, not just cited"
    requirement: "ORCH-01"
    verification:
      - kind: integration
        ref: "cargo test --test lib orchestrator_workflow_lifecycle -- --nocapture (1 passed)"
        status: pass
      - kind: other
        ref: "grep -c '| Verify' .planning/ledgers/milestone-09-12.md → 0 (no bare Verify verdict ships in the derived section)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Both highlight tables (Shipped-one-acceptance-criterion-false single instance, Verified-open Deferred-QA block) and the ledger-file contention table naming plans 13-02..13-07's disjoint section ranges"
    requirement: "ORCH-02"
    verification:
      - kind: other
        ref: "grep -c 'Shipped, one acceptance criterion false' → 4 (≥2 required); grep -c 'Verified open' → 5 (≥2 required); grep -cE '13-0[2-7]' → 8 (≥6 required)"
        status: pass
    human_judgment: false
  - id: D4
    description: "All 120 requirement rows present, exact ID match against REQUIREMENTS.md:3638-3931 (same order, same set), 32 bare-Verify rows converted to pending markers naming their owning plan, 88 rows transcribed with the run-5-input prefix"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "diff of extracted REQ-* IDs (ledger vs REQUIREMENTS.md:3638-3931) → zero output; grep -c 'pending — plan 13-' → 32"
        status: pass
    human_judgment: false

# Metrics
duration: ~45min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 01: Milestone 9-12 Ledger Scaffold Summary

**Created the fifth and final sibling ledger (120 rows, 28 sections) and derived Milestone 9 Epic 1's
six rows end-to-end with re-run citations and one command actually executed this session.**

## Performance

- **Duration:** ~45 min
- **Started:** 2026-08-10 (session start)
- **Completed:** 2026-08-10T15:43:38Z
- **Tasks:** 3
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`, created)

## Accomplishments

- `.planning/ledgers/milestone-09-12.md` exists with the complete head-note spine (supersession line,
  primary-key note, evidence bar, eleven-class verdict legend with `Verify` retired and mapped onto
  the series' seven-class vocabulary, tie-break rule, manifest carve-out, four re-grepped factual
  anchors, D-04's corrected 35/53/32 arithmetic, D-10's five-run checkbox pattern, D-00b precedence
  order, D-07's inherited stale-citation inventory, provenance paragraph, both highlight tables, the
  row-order/amendment convention, and the ledger-file contention table)
- Milestone 9 Epic 1's six rows derived to the D-00e evidence bar: three previously-bare-`Verify` rows
  turned into real `Shipped` verdicts with fresh `file:line` + exerciser citations; three previously-
  `Shipped` rows re-confirmed with citations re-run this session, including one genuine finding
  (`REQ-workflow-repository-port`'s SQLite adapter has no production consumer wiring it into a running
  `Orchestrator` — only a test double exercises the port in-tree)
- All 120 rows and 28 section headers present in `REQUIREMENTS.md`'s own order, exact ID match
  (zero diff), 32 bare-`Verify` rows converted to `pending — plan 13-NN` markers naming their owner
  from the contention table, 88 rows transcribed with the `run-5 input (not yet re-derived):` prefix
- Actually ran `cargo test --test lib orchestrator_workflow_lifecycle -- --nocapture` this session —
  `1 passed; 0 failed` — rather than citing the lifecycle integration test unverified

## Task Commits

1. **Task 1: Create the ledger and derive Milestone 9 Epic 1 end-to-end** - `22fd2be` (feat)
2. **Task 2: Complete the head notes, the two highlight tables and the contention table** - `8f79162` (feat)
3. **Task 3: Transcribe the remaining 28 section headers and 114 row stubs** - `679e81e` (feat)

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` - The fifth and final sibling ledger: 120 `REQ-*` rows across
  28 section headers, Milestone 9 Epic 1 fully derived, the remaining 114 rows either transcribed
  (`run-5 input (not yet re-derived):` prefix) or stubbed (`pending — plan 13-NN`)

## Decisions Made

- Milestone 9 Epic 1's three bare-`Verify` rows were derived by reading `.planning/intel/requirements.md`'s
  acceptance criteria for each ID, then locating the implementing symbol and its exerciser with `grep -rn`
  over `src/` and `crates/` — not by inference from the requirement description alone
- `REQ-workflow-repository-port` records a consumer-wiring caveat as a **limitation of** its `Shipped`
  verdict rather than downgrading it: the SQLite adapter's own three tests exercise it directly against
  a real database, independent of whether any production binary constructs an `Orchestrator` with it
- Highlight-table rows use backtick-wrapped `` `REQ-*` `` IDs (not bare `| REQ-`) so they don't pollute
  the epic-section row count the plan's own acceptance criteria (and future fan-out plans' `diff`
  checks) depend on — this was caught and fixed mid-task via a `NF` field-count / ID-diff self-check
  before committing, not left for a later plan to discover

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Highlight-table rows were polluting the `^| REQ-` row count**
- **Found during:** Task 2 (self-verification of acceptance criteria before commit)
- **Issue:** The `Verified open` and `Shipped, one acceptance criterion false` highlight tables used
  bare `| REQ-xxx |` row syntax, which the plan's own `awk`/`grep -oE '^\| REQ-'` acceptance checks
  (and every downstream fan-out plan's `diff` against `REQUIREMENTS.md:3638-3931`) treat identically to
  an actual epic-section row — inflating the count from 6 to 25 and (after task 3) would have broken
  the exact-ID-match diff.
- **Fix:** Wrapped every highlight-table ID cell in backticks (`` `REQ-xxx` ``, matching
  `milestone-07-08.md`'s own summary-table convention), which is not a leading `| REQ-` match.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `grep -c '^| REQ-'` returned to `6` (post-task-1 baseline) immediately after the fix,
  and to the correct `120` with an exact zero-diff ID match after task 3.
- **Committed in:** `8f79162` (part of the task 2 commit — fixed before committing, not as a follow-up)

**2. [Rule 1 - Bug] A grep-pattern pipe character was inflating the two-column row-shape check**
- **Found during:** Task 1 (self-verification of the `awk -F'|' ... NF` acceptance check)
- **Issue:** `REQ-default-task-services-real-logic`'s row cited a shell command containing a literal
  regex alternation (`grep -n "tokio::time::sleep\|simulate"`), whose `|` character was parsed by the
  ledger's own `|`-delimited table syntax as a fourth column boundary, breaking the required two-column
  (`Requirement | Verdict`) shape.
- **Fix:** Reworded the citation to describe two separate greps in prose instead of one alternation
  pattern, removing the stray `|`.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `awk -F'|' '/^\| REQ-/ {print NF}' | sort -u` → single value `4` (no row has an
  extra column).
- **Committed in:** `22fd2be` (part of the task 1 commit — fixed before committing)

**3. [Rule 1 - Bug] Three "pending — plan 13-NN" phrases inside prose notes inflated the pending-row count**
- **Found during:** Task 3 (self-verification of `grep -c 'pending — plan 13-'` → `32`)
- **Issue:** Five prose sentences (the evidence-bar paragraph, the contention-table's closing paragraph,
  and three per-section owner notes for 13-02/13-03/13-04) used the literal phrase `pending — plan
  13-NN`/`13-0N` as explanatory text, not as an actual row's Verdict cell — inflating the count from 32
  to 37 against the plan's exact acceptance target.
- **Fix:** Reworded all five to describe the mechanism ("a `pending` marker naming plan 13-0N") without
  repeating the literal em-dash phrase, preserving the same meaning.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `grep -c 'pending — plan 13-'` → exactly `32`, matching the 35-bare-`Verify`-minus-3-
  derived-in-M9E1 figure.
- **Committed in:** `679e81e` (part of the task 3 commit — fixed before committing)

---

**Total deviations:** 3 auto-fixed (all Rule 1 — formatting/counting bugs caught by this plan's own
self-verification against its acceptance criteria before each commit)
**Impact on plan:** All three were caught and fixed inline before committing; no scope creep, no
downstream plan needs to redo work because of them.

### Documented, not auto-fixed: acceptance-criteria discrepancy

**Section-header count is 28, not the plan's stated 29.** Task 3's acceptance criteria state
`grep -c '^### ' .planning/ledgers/milestone-09-12.md` → `29`. Directly re-grepping the source range
this session — `sed -n '3607,3931p' .planning/REQUIREMENTS.md | grep -n '^### '` — returns exactly
**28** section headers (verified twice, independently, before and after transcription). The plan's own
`13-RESEARCH.md` inventory table lists 28 data rows plus one summary "Total" row (29 table rows total,
0 sections), which is consistent with 28 real sections, not 29. Per this plan's own `must_have` — "holds
exactly 120 `| REQ-` rows across 29 section headers, matching REQUIREMENTS.md:3638-3931 one-for-one" —
the one-for-one match with the source is the higher-priority constraint, and the source itself has 28
headers. This is treated as a miscount in the plan's own acceptance criterion (the same class of
arithmetic drift this phase's D-04/D-08/D-09/D-17/D-18 findings catch elsewhere in the corpus), not as
a defect in the ledger. **Not auto-fixed** because there is no code to fix — the ledger already matches
ground truth exactly; flagging it here is the correct action per this plan's own evidence-bar mandate.

## Issues Encountered

None beyond the three self-caught formatting bugs documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 13-02 through 13-07 can start in parallel: the ledger-file contention table fixes their
  disjoint section ranges (Milestone 9 Epics 2-6; Milestone 10 Epics 1-5; Milestone 11 Epics 1-2, 3, 4,
  6, 5&7; Milestone 12 Epics 1-4; Milestone 12 Epics 5-7; Deferred-QA Epics 25/26/27/28-29 +
  project-management), and every row they own carries either a `pending — plan 13-NN` marker or a
  `run-5 input (not yet re-derived):`-prefixed transcription — no cell is blank, no verdict is invented
  ahead of the owning plan.
- One genuine open finding surfaces early: `REQ-workflow-repository-port`'s `SqliteWorkflowRepository`
  has no production consumer wiring it into a running `Orchestrator` in this tree (`grep -rln
  SqliteWorkflowRepository src/ crates/` outside its own file returns nothing). This is recorded as a
  limitation of the `Shipped` verdict, not escalated — plans 13-02 through 13-07 should watch for the
  same "port ships, adapter ships, nothing wires them together in production" pattern elsewhere in the
  35 formerly-bare-`Verify` rows they own.
- **Section-header count flag for plan 13-13 (close-out):** this ledger has 28 section headers, not the
  29 the phase's own planning materials predicted. The close-out's whole-file integrity re-check should
  assert `28`, not `29` — using the plan-predicted figure would fail against the ledger's own
  ground-truth-correct state.

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
