---
phase: 07-workspace-ground-truth-recorded-answers
plan: 05
subsystem: docs
tags: [adr, decisions, milestone-6, facade-reexport, semver, d-00g]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "ADR-0008 (workspace version converges on 0.7.0), cited rather than re-derived"
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "ADR-0014 (Milestone 4-6 tier numbering), cited for the prerequisite-line correction"
provides:
  - "ADR-0018: the Milestone 6 facade no-shim re-export policy, with its version consequence cited from ADR-0008"
  - "Milestone 6 overview annotated: prerequisite line, Epic 2 AC 6, Epic 4 AC 5, risk register — all Superseded per D-00g"
  - "Epic 4 CircuitBreaker PRD annotated: Goal 7 / FR-4.11 confirmed, FR-4.12 re-pointed to the mdbook"
affects: [phase-11-facade-cleanup, plan-07-08, plan-07-10, plan-07-13]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "D-00g annotation: dated banner + strikethrough-and-append / Superseded blockquote, original text always retained"
    - "ADR citing another ADR by number instead of re-deriving an already-answered question"

key-files:
  created:
    - .planning/decisions/0018-m6-facade-reexport-policy.md
  modified:
    - .project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md
    - .project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md

key-decisions:
  - "D-13: no-shim posture stands as policy; Epic 2 Open Question 4 confirmed rather than left dangling"
  - "D-14: version consequence cites ADR-0008 rather than re-deriving the major-bump question"
  - "D-15: the Milestone 5 -> Milestone 6 posture flip is recorded as history, not a contradiction"
  - "D-16: ADR-0018 names Phase 11 / FACADE-02 D1 explicitly as its downstream consumer"
  - "D-18: FR-4.12's STABLE_API.md deliverable is re-pointed to docs/src/api-reference/stable-api.md"

patterns-established: []

requirements-completed: [ARCH-04, ARCH-02, ARCH-05]

coverage:
  - id: D1
    description: "ADR-0018 records the M6 facade no-shim posture as policy, confirms Epic 2 Open Question 4, and states the version consequence by citing ADR-0008"
    requirement: ARCH-04
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0018-m6-facade-reexport-policy.md == 7; grep -c '0008' >= 2; grep -c 'FACADE-02' >= 1; grep -c 'REQ-battalion-facade-shim' >= 1; test -d src/application/use_cases fails"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 6 overview annotated with a dated banner (ADR-0018, ADR-0014), the prerequisite line corrected inline, and three Superseded banners above Epic 2 AC 6, Epic 4 AC 5, and the risk-register CircuitBreaker row — original text retained"
    requirement: ARCH-02
    verification:
      - kind: other
        ref: "grep -c 'ADR-0018' >= 4; grep -c 'ADR-0014' >= 2; grep -ci 'Superseded' >= 3; grep -c '~~' >= 1; git diff --numstat deletions <= 3; original AC6/AC5 sentences still greppable"
        status: pass
    human_judgment: false
  - id: D3
    description: "Epic 4 CircuitBreaker PRD annotated with a dated ADR-0018 banner, confirmation notes on Goal 7 and FR-4.11, and FR-4.12 re-pointed to a verified mdbook path"
    requirement: ARCH-05
    verification:
      - kind: other
        ref: "grep -c 'ADR-0018' >= 3; grep -c 'docs/src/api-reference/stable-api.md' >= 1; test -f docs/src/api-reference/stable-api.md; grep -c 'STABLE_API.md' >= 1; git diff --numstat deletions <= 2"
        status: pass
    human_judgment: false

duration: 4min
completed: 2026-08-06
status: complete
---

# Phase 07 Plan 05: Milestone 6 Facade Re-Export Policy Summary

**ADR-0018 records the M6 no-shim posture as policy, cites ADR-0008 for the version consequence
instead of re-deriving it, and annotates the overview and the CircuitBreaker PRD to match.**

## Performance

- **Duration:** ~4 min (18:28:37 -> 18:31:36 UTC across three task commits)
- **Started:** 2026-08-06T18:28:37Z
- **Completed:** 2026-08-06T18:31:36Z
- **Tasks:** 3
- **Files modified:** 3 (1 new, 2 annotated in place)

## Accomplishments

- Created `.planning/decisions/0018-m6-facade-reexport-policy.md`: seven canonical H2 headings,
  no-shim posture ratified, Epic 2 Open Question 4 confirmed, version consequence cited from
  ADR-0008 (not re-derived), the Milestone 5 -> Milestone 6 posture flip recorded as history, and
  Phase 11 / FACADE-02 D1 named as the downstream consumer.
- Annotated the Milestone 6 overview
  (`.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md`)
  with a top-of-file banner (ADR-0018, ADR-0014), the "Completed in Milestones 1 and 2"
  prerequisite line corrected inline to "Milestones 4 and 5" (ADR-0014), and three standalone
  Superseded banners immediately above Epic 2 Acceptance Criterion 6 (`:230`, post-banner line
  number), Epic 4 Acceptance Criterion 5 (`:449`), and the risk register's CircuitBreaker
  mitigation row (`:87` heading, since the row itself is inside a Markdown table and a blockquote
  cannot be inserted mid-table without breaking it) — original acceptance-criterion and
  risk-register text unchanged beneath every banner.
- Annotated the Epic 4 CircuitBreaker PRD
  (`.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md`)
  with a top-of-file banner (ADR-0018), confirmation notes beneath Goal 7 and FR-4.11 ratifying
  the no-shim position, and FR-4.12 re-pointed with a strikethrough-and-append heading plus a bold
  note to `docs/src/api-reference/stable-api.md` — verified to exist and already carry the
  `CircuitBreaker`/`CircuitState` canonical-path note before the path was written.

## Ground Truth Confirmed This Session

- `src/application/` was listed directly: it holds `cli`, `errors`, `mod.rs`, `services`.
  `src/application/use_cases/` does **not** exist anywhere in the tree.
- `docs/src/api-reference/stable-api.md` exists and already documents the `CircuitBreaker` /
  `CircuitState` relocation to `paladin::infrastructure::resilience::circuit_breaker` with no
  re-export at the old path (`stable-api.md:618-625`).
- Epic 2's Non-Goal 7 (`prd-relocate-orchestration-services.md:192`) and Open Question 4 (`:330`)
  and Epic 4's Goal 7 (`prd-relocate-circuitbreaker-infra.md:50-51`) / FR-4.11 (`:239-244`,
  post-banner section 4.11) were re-grepped fresh at authoring time, not trusted from CONTEXT.md's
  transcribed line numbers.
- Exact clause line numbers annotated (post-banner, i.e. after this session's own edits shifted
  line numbers within each file):
  - Milestone 6 overview (line numbers re-grepped after all edits): prerequisite heading (line
    37), Epic 2 AC 6 item (line 243, banner immediately above), Epic 4 AC 5 item (line 470,
    banner immediately above), risk register (`### Risk Register` heading at line 87, banner
    immediately above it, table's CircuitBreaker row unchanged beneath).
  - Epic 4 CircuitBreaker PRD: Goal 7 confirmation note (immediately after item 7 in `## 2. Goals`),
    FR-4.11 confirmation note (immediately after `### 4.11 Remove Old Module Registration`'s body),
    FR-4.12 heading correction and note (`### 4.12` heading, immediately before its original body).

## Task Commits

Each task was committed atomically (per the standard executor task-commit protocol — see
Deviations for why this differs from the plan's requested single combined commit):

1. **Task 1: ADR-0018 — the Milestone 6 facade re-export policy and its version consequence** -
   `1c6f6b5` (docs)
2. **Task 2: Annotate the Milestone 6 overview** - `80598b6` (docs)
3. **Task 3: Annotate the Epic 4 CircuitBreaker PRD** - `d928fd6` (docs)

_No plan-metadata commit — the orchestrator owns STATE.md/ROADMAP.md updates after the wave merges
per this plan's execution instructions._

## Files Created/Modified

- `.planning/decisions/0018-m6-facade-reexport-policy.md` - new ADR: no-shim posture as policy,
  cites ADR-0008, names Phase 11 / FACADE-02 D1
- `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md` -
  annotated in place: banner, prerequisite line, three Superseded blocks
- `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md` -
  annotated in place: banner, Goal 7 / FR-4.11 confirmation, FR-4.12 re-pointed

## Decisions Made

- **D-13 / D-14 / D-15 / D-16 / D-18** (all from `07-CONTEXT.md`, applied as written): the no-shim
  posture stands, the version consequence cites ADR-0008, the M5->M6 flip is history not
  contradiction, Phase 11 / FACADE-02 D1 is named explicitly, and FR-4.12 is re-pointed to the
  mdbook path. No new decisions were made beyond what CONTEXT.md already recorded — this plan's
  job was locating and citing the exact fresh evidence for each.

## Deviations from Plan

### Auto-fixed / Adjusted

**1. [Rule 3 - Blocking, structural] Per-task atomic commits instead of one combined commit**
- **Found during:** Task 1 (before any commit was made)
- **Issue:** Task 3's `<action>` instructs "Commit this plan's three files together in a single
  commit at the end of the plan," and its acceptance criteria check
  `git log -1 --name-only` for all three files in one commit. This conflicts directly with the
  executor's standard task-commit protocol (`<task_commit_protocol>`: commit immediately after
  each task, atomically) and with the parallel-execution guidance for this worktree, which
  explicitly forbids using `git reset --soft` to squash already-made commits and instructs
  recording a deviation instead if part of the set is already committed.
- **Fix:** Followed the standard atomic-per-task commit protocol: Task 1 (ADR-0018) committed as
  `1c6f6b5`, Task 2 (overview annotation) committed as `80598b6`, Task 3 (Epic 4 PRD annotation)
  committed as `d928fd6`. All three land in this plan's wave, in the same relative order the plan
  specifies, and `git diff --stat HEAD~3 HEAD -- '*.rs' 'Cargo.toml' '.github/'` (spanning all
  three commits) is empty, satisfying the substance of the plan's no-code-touched constraint even
  though the literal single-commit acceptance check does not apply.
- **Files modified:** none beyond the plan's own three files
- **Verification:** each task's other acceptance criteria (grep counts, `test -f`, numstat
  deletion bounds) independently confirmed passing per-task; `git log --oneline -5` shows all
  three commits present and in order.
- **Committed in:** `1c6f6b5`, `80598b6`, `d928fd6`

**2. [Rule 3 - Blocking, structural] Risk-register Superseded banner placed above the table heading, not above the table row**
- **Found during:** Task 2
- **Issue:** The plan's action text says to place a standalone Superseded blockquote "immediately
  above ... the risk-register re-export row." The CircuitBreaker mitigation is a single cell in
  the middle of a Markdown table (`| Risk | Likelihood | Impact | Mitigation |` ... rows), and a
  blockquote line cannot be inserted between two table rows without breaking the table's Markdown
  parsing (GFM tables require every row, including separator, to be contiguous).
- **Fix:** Placed the Superseded banner immediately above the `### Risk Register` heading (i.e.
  above the whole table, as close to the row as Markdown syntax permits), explicitly naming the
  CircuitBreaker row it corrects so the pointer is unambiguous. The table itself, including the
  original "Facade crate re-exports absorb the change" mitigation text, is untouched.
- **Files modified:**
  `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md`
- **Verification:** `grep -ci 'Superseded'` returns 3 (one per required banner); the original
  mitigation-column text for the CircuitBreaker row is still present verbatim in the table.
- **Committed in:** `80598b6`

---

**Total deviations:** 2 auto-adjusted (both Rule 3 - structural/blocking, both about markup
placement mechanics, not about the recorded substance). No scope creep — every fact, citation, and
decision recorded matches CONTEXT.md D-13 through D-18 and the plan's `must_haves`.

## Issues Encountered

None beyond the two deviations above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 11's FACADE-02 D1 has its answer by name in ADR-0018's `## Downstream Consumers`.
- Plans 07-08 and 07-10 (this same phase) can cite ADR-0018 for the
  `REQ-orchestration-no-reexport-shims`, `REQ-circuitbreaker-old-path-retired`, and
  `REQ-battalion-facade-shim` ledger rows — this plan did not touch
  `.planning/ledgers/milestone-04-06.md` itself, per the wave's file-ownership boundary.
- Plan 07-13 still needs to add ADR-0018's row to `.planning/decisions/PROMOTION.md`'s numbering
  index and advance the "Next free ADR number" line past 0018 — not done here, out of this plan's
  `files_modified` scope.

## Self-Check: PASSED

- FOUND: `.planning/decisions/0018-m6-facade-reexport-policy.md`
- FOUND: commit `1c6f6b5` (Task 1)
- FOUND: commit `80598b6` (Task 2)
- FOUND: commit `d928fd6` (Task 3)

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
