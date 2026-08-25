---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 07
subsystem: docs
tags: [ledger, deferred-qa, requirements-traceability, tool-calling, coverage-register]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "plan 13-01's ledger scaffold (.planning/ledgers/milestone-09-12.md), all 120 row stubs and head notes"
provides:
  - "18 cited Verdict cells for Deferred-QA Epics 25, 26, 27, 28-29 and project-management in .planning/ledgers/milestone-09-12.md"
  - "The corpus's only item-by-item-verified register re-confirmed with fresh file:line evidence, correcting three stale run-5 claims found during re-derivation"
  - "Four D-13 relocation rows (REQ-listener-service-test-coverage, REQ-llm-tool-calling-port, REQ-arch-doc-modernization, REQ-asciinema-demos) each recording old-path-absent + new-path-present"
affects: [14-web-tool-calling-and-auth-contract, 15-deferred-qa-completion, 16-documentation-currency-and-the-architecture-gap]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-00e evidence bar applied to negatives: every absence claim carries the command that found nothing, re-run this session", "Dated corrections superseding stale ingest claims rather than silent overwrites (D-00d)"]

key-files:
  created: []
  modified: [.planning/ledgers/milestone-09-12.md]

key-decisions:
  - "REQ-llm-tool-calling-adapters split into two halves: the ProviderCapabilities over-reporting defect the run-5 register described is already fixed (Phase 2, commit a2cc1c5, dated 2026-08-01, supports_tool_calling: false with a passing correspondence test) -- 'Shipped -> WEB-03' for the flag; the functional capability (LlmResponse.function_call still hardcoded None) remains 'Verified open -> WEB-04'"
  - "REQ-rustdoc-zero-warnings corrected from 'contested' to 'already decided': HARD-07/ADR-0033 (Phase 10, dated 2026-08-08) already ruled the bar is zero-warnings-enforced-in-CI (ci.yml:58, pre-existing since the repo's original CI setup); the tree is measured red (20 warnings/4 crates), re-verified unchanged since ADR-0033's measurement commit via git log"
  - "REQ-user-service-test-coverage's run-5-recorded collision with M8 deferred-items.md D2 is already resolved by ADR-0034, which withdrew the split -- Phase 15/DEFER-02 sizes against the unsplit file, not a live sequencing question"
  - "REQ-asciinema-demos path corrected: docs/assets/ does not exist at all (not merely empty as previously recorded); docs/src/assets/ holds unrelated architecture SVGs"
  - "REQ-mock-infrastructure corrected: MockLogPort does exist (two module-local #[cfg(test)] doubles), while the other four named mocks are genuinely absent -- 4/5, not 5/5"
  - "REQ-master-plan-epics-11-18 records two facts under one ID: the master expansion plan's provenance-only classification (unchanged) plus project-management's single 'open' checkbox item quoted verbatim and recorded nonexistent (D-10), a corpus-naming artefact rather than a merge"

requirements-completed: [ORCH-01, ORCH-02]

coverage:
  - id: D1
    description: "Deferred-QA Epic 25 (7 rows) and Epic 26 (4 rows): CI/CD pipeline gaps re-confirmed with corrected line numbers (D-08), coverage-threshold variant recorded with both sides, architecture-doc relocation+content-gap split into two labelled facts, rustdoc-zero-warnings bar corrected from contested to already-decided via ADR-0033"
    requirement: "ORCH-01"
    verification:
      - kind: manual_procedural
        ref: "awk '/^### Deferred-QA Epic 25/,/^### Deferred-QA Epic 27/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 11; grep -c 'run-5 input (not yet re-derived)' in range -> 0; grep -c 'Verified open' in range -> 9"
        status: pass
    human_judgment: false
  - id: D2
    description: "Deferred-QA Epic 27 (2 rows), Epics 28-29 & coverage register (4 rows), project-management (1 row): LLM tool-calling split verdict correcting a stale defect claim, mock-infrastructure correction, ADR-0034 collision resolution, listener-service relocation, and project-management's single open item recorded nonexistent"
    requirement: "ORCH-02"
    verification:
      - kind: manual_procedural
        ref: "awk '/^### Deferred-QA Epic 27/,0' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 7; grep -c 'crates/paladin-ports/src/output/llm_port.rs' -> 1; grep -c 'ADR-0034' -> 2; grep -c 'Create template' -> 2; grep -c '^| REQ-' (whole file) -> 120"
        status: pass
    human_judgment: false

duration: 55min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 07: Deferred-QA Ledger Derivation (Epics 25-29 & project-management) Summary

**Derived 18 cited Deferred-QA/project-management ledger verdicts, correcting three stale run-5 claims found by re-verification: the LLM tool-calling capability flag was already fixed in Phase 2 (not still over-reporting), the rustdoc-warnings bar was already decided by ADR-0033 (not still contested), and the user-service coverage/split collision was already resolved by ADR-0034 (not still colliding).**

## Performance

- **Duration:** ~55 min
- **Started:** 2026-08-10T18:05:00Z (approx.)
- **Completed:** 2026-08-10T19:00:00Z
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`)

## Accomplishments
- Replaced all 18 `run-5 input (not yet re-derived):`-prefixed Verdict cells in Deferred-QA Epics 25, 26, 27, 28-29 and project-management with fresh, cited verdicts (`file:line` + a re-run command per D-03/D-00e); zero bare `Verify` rows remain anywhere in the 120-row ledger (this was the last fan-out plan)
- **Corrected REQ-llm-tool-calling-adapters**, the plan's most significant finding: the run-5 defect ("all three adapters declare tool-calling capability... hardcode `function_call: None`, over-reports") is stale. `supports_tool_calling: false` ships on all three adapters (`openai/adapter.rs:649`, `anthropic/adapter.rs:550`, `deepseek/adapter.rs:807`), fixed in Phase 2 plan 02-02 (commit `a2cc1c5`, dated 2026-08-01) per D-13/D-14, with a passing correspondence test (`cargo test -p paladin-llm --features openai,anthropic,deepseek test_capabilities_tool_calling_matches_request_surface` -> `1 passed`) run this session. Split the verdict: `Shipped -> WEB-03` for the honest flag, `Verified open -> WEB-04` for the still-unbuilt functional capability (`LlmResponse.function_call` still hardcoded `None` in all three adapters)
- **Corrected REQ-rustdoc-zero-warnings**: HARD-07/ADR-0033 (Phase 10, dated 2026-08-08) already settled which of three competing milestone positions governs the `cargo doc` warning bar — zero-warnings-enforced-in-CI, not "contested." `ci.yml:58` already runs the strict gate (pre-existing since the repo's original CI setup, commit `15eccae`); the tree is measured red at 20 warnings across 4 crates, re-verified unchanged since ADR-0033's own measurement commit via `git log --oneline c048938..HEAD -- <10 cited files>` returning nothing
- **Corrected REQ-user-service-test-coverage**: ADR-0034 already withdrew Milestone 8 D2's planned split of `user_service.rs`, resolving the run-5-recorded collision with Epic 28's coverage plan — Phase 15/DEFER-02 inherits a decided input (test the unsplit file), not a live sequencing question
- **Corrected REQ-mock-infrastructure**: `MockLogPort` does exist (`src/application/services/log_orchestrator/mod.rs:372`, `src/config/user_config.rs:74`) as module-local test doubles, not shared infrastructure — 4 of the 5 named mocks are genuinely absent, not all 5
- **Corrected REQ-asciinema-demos**'s path claim: `docs/assets/` does not exist at all (not merely empty as previously recorded); the path that exists, `docs/src/assets/`, holds six unrelated architecture SVGs
- Recorded the four D-13 relocation rows (`REQ-listener-service-test-coverage`, `REQ-llm-tool-calling-port`, `REQ-arch-doc-modernization`, `REQ-asciinema-demos`) each with old-path-absent + new-path-present, and `REQ-arch-doc-modernization` specifically as two separately labelled facts (relocation + content gap) cross-referencing `REQ-architecture-docs-update` per D-00f
- Recorded the coverage-threshold variant (`REQ-codecov-config-thresholds`) with both sides — parent PRD's 78% hard gate vs. Epic 25's phased 70→74→78 ramp, parent's own OQ-3 recorded Open — resolving neither, per D-19/prohibition
- Recorded `REQ-master-plan-epics-11-18` as two facts under one ID: the master expansion plan's provenance-only classification (retained) plus project-management's single "open" checkbox item (`tasks-project-management-setup.md:33`) quoted verbatim and recorded nonexistent (D-10), not converted into a task
- Re-grepped and corrected all stale run-5 line-number citations (D-08): `actions-rs/toolchain@v1`/`actions/cache@v3`/`codecov/codecov-action@v3` deprecated-reference locations, `benchmark-regression-signal`'s current `ci.yml:812` position

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Deferred-QA Epics 25 and 26 (11 rows)** - `962aaf9` (docs)
2. **Task 2: Derive Deferred-QA Epics 27, 28-29 and project-management (7 rows)** - `2634d92` (docs)

_Note: this plan makes zero `.rs`, `.project/`, and `docs/src/` file changes (records-only, D-19); both commits are `docs` type touching only the ledger._

## Files Created/Modified
- `.planning/ledgers/milestone-09-12.md` - Verdict cells replaced in place for the 18 Deferred-QA/project-management requirement IDs owned by this plan; no row inserted, deleted, or reordered; row count unchanged at 120

## Decisions Made
- `REQ-llm-tool-calling-adapters` split into a fixed half (`Shipped -> WEB-03`) and a still-open half (`Verified open -> WEB-04`) rather than one verdict, because the two halves of the original run-5 problem statement have diverged in truth value since ingest
- `REQ-rustdoc-zero-warnings` reframed from "contested" to "already decided by ADR-0033," per D-00b precedence (ADR outranks a PRD-derived ledger claim)
- `REQ-user-service-test-coverage` reframed from "collides with a run-4 register" to "the collision is already resolved," citing ADR-0034's explicit withdrawal
- `REQ-arch-doc-modernization` kept as two separately labelled facts (relocation `Shipped (relocated)` + content gap `Verified open`) rather than one blended verdict, per D-00f/D-13(d)
- `REQ-master-plan-epics-11-18` retained its existing provenance-only content and added the project-management single-item verdict as a second, separately labelled fact under the same ID, rather than replacing the row's content outright — the ID's single row is this section's only row, and D-00d requires retaining superseded text rather than silently overwriting it

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Correction] Corrected REQ-llm-tool-calling-adapters' inherited defect claim**
- **Found during:** Task 2
- **Issue:** The plan's own task text (based on the run-5 ingest) asserted all three LLM adapters still "declare tool-calling capability... hardcode `function_call: None`... over-report." Reading the actual adapter code showed `supports_tool_calling: false` on all three, with in-code comments citing `(WEB-03, D-14)`.
- **Fix:** Traced the fix to Phase 2 plan 02-02 (commit `a2cc1c5`, dated 2026-08-01) via `02-02-SUMMARY.md`, which explicitly states "Phase 14's WEB-03 is satisfied by this plan per D-13/D-14." Ran the correspondence test this session (`cargo test -p paladin-llm --features openai,anthropic,deepseek test_capabilities_tool_calling_matches_request_surface` → 1 passed) and recorded a split verdict rather than transcribing the stale claim, per D-00e's "where a search that previously found nothing now finds something, record the change with its date."
- **Files modified:** `.planning/ledgers/milestone-09-12.md` (Verdict cell only, no code changed)
- **Verification:** Test command re-run this session, passing; `grep -n 'supports_tool_calling' crates/paladin-llm/src/{openai,anthropic,deepseek}/adapter.rs` confirms all three
- **Committed in:** `2634d92` (Task 2 commit)

**2. [Rule 1 - Correction] Corrected REQ-rustdoc-zero-warnings' "contested" framing**
- **Found during:** Task 1
- **Issue:** The plan's task text and the inherited ledger row both framed the `cargo doc` warning bar as "contested" across three milestone positions. `.planning/REQUIREMENTS.md:4243` shows `HARD-07 | Phase 10 | Complete`, and ADR-0033 exists recording the decision.
- **Fix:** Read ADR-0033 in full, confirmed `ci.yml:58` already runs the strict zero-warnings gate (pre-existing since the repo's original setup, not a phase-13 discovery), confirmed via `git log` that none of ADR-0033's ten cited files changed since its measurement commit, and recorded the corrected framing: the bar is decided, the tree is red, Phase 16/DOCS-03 clears the residue.
- **Files modified:** `.planning/ledgers/milestone-09-12.md` (Verdict cell only)
- **Verification:** `git log --oneline c048938..HEAD -- <10 files>` returns nothing this session
- **Committed in:** `962aaf9` (Task 1 commit)

**3. [Rule 1 - Correction] Corrected REQ-user-service-test-coverage's "collides" framing**
- **Found during:** Task 2
- **Issue:** The plan's task text asked to "record the collision" between Epic 28's test plan and M8 D2's split plan and "sequence deliberately" without resolving it. Reading `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md` showed the split was already withdrawn this same phase.
- **Fix:** Recorded the collision as already resolved by ADR-0034, citing its explicit "the split is WITHDRAWN" language and its named inheritor (Phase 15/DEFER-02), rather than presenting a stale open collision.
- **Files modified:** `.planning/ledgers/milestone-09-12.md` (Verdict cell only)
- **Verification:** ADR-0034 read in full this session, its text quoted verbatim
- **Committed in:** `2634d92` (Task 2 commit)

**4. [Rule 1 - Bug] Corrected REQ-asciinema-demos' path claim**
- **Found during:** Task 1
- **Issue:** The row's inherited text and the plan's own read-first claimed `docs/assets/` "exists and is empty." `test -d docs/assets` fails — the directory does not exist at all. `docs/src/assets/` exists instead, holding six architecture SVGs unrelated to demo content.
- **Fix:** Recorded the corrected path finding, cross-referencing plan 13-04's own prior correction of the same fact (its SUMMARY's Next Phase Readiness note).
- **Files modified:** `.planning/ledgers/milestone-09-12.md` (Verdict cell only)
- **Verification:** `test -d docs/assets` (fails) and `ls docs/src/assets` (6 files) re-run this session
- **Committed in:** `962aaf9` (Task 1 commit)

**5. [Rule 1 - Bug] Corrected REQ-mock-infrastructure's "none of the five" claim**
- **Found during:** Task 2
- **Issue:** The plan's task text asked to confirm "none of `MockUserRepository`, `MockLogPort`, `MockNotificationService`, `MockEventSource` or `MockTriggerExecutor` exists." `grep -rn 'struct MockLogPort'` found it at two locations.
- **Fix:** Recorded the corrected 4/5 split rather than the stated 5/5, citing both `MockLogPort` locations as module-local, non-shared test doubles.
- **Files modified:** `.planning/ledgers/milestone-09-12.md` (Verdict cell only)
- **Verification:** `grep -rn 'struct MockLogPort'` re-run this session, two hits
- **Committed in:** `2634d92` (Task 2 commit)

---

**Total deviations:** 5 auto-corrected (all Rule 1 — factual corrections to stale or inherited claims, discovered by re-verifying rather than transcribing, per this phase's own D-00e/D-03 evidence bar)
**Impact on plan:** All five corrections strengthen accuracy for the three downstream phases (14, 15, 16) that plan directly against these rows; none required code changes (records-only plan, D-19 held throughout). No scope creep — every correction stayed inside this plan's owned section range.

## Issues Encountered
- The plan's own acceptance criterion for Task 2 states `grep -c 'run-5 input (not yet re-derived)' .planning/ledgers/milestone-09-12.md` (whole file) → `0`. After this plan's edits, the whole-file count is `3`, not `0` — but all three remaining occurrences are head-note explanatory prose at lines 25, 238, and 412 (describing the ledger's own interim-state convention), owned by plan 13-01 per the ledger-file contention table, not data rows. Editing them would violate this plan's own explicit prohibitions ("No row outside the Deferred-QA and project-management sections is touched," "Only this plan's section range was modified"). Resolved by leaving the head-note prose untouched and confirming via `awk`-scoped greps that all 18 data rows this plan owns are clean (`run-5 input` count in this plan's section range → `0`). This is a documented discrepancy against one literal acceptance-criterion string, not a defect in the ledger.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 18 Deferred-QA/project-management rows carry cited verdicts; `grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` still returns 120; zero bare `Verify` rows remain anywhere in the file (this was the last fan-out plan)
- Phase 14 / WEB-03, WEB-04 inherits: `REQ-llm-tool-calling-adapters`'s split verdict (flag already fixed, functional capability still open) and `REQ-llm-tool-calling-port`'s relocation + capability-gap findings against the same `llm_port.rs` file
- Phase 15 / PIPE-01..05, DEFER-01..03 inherits: seven re-confirmed CI/CD gap rows with corrected line numbers, the unresolved coverage-threshold variant (both sides recorded), the mock-infrastructure correction (4/5 not 5/5), and the already-resolved user-service split collision (ADR-0034)
- Phase 16 / DOCS-02, DOCS-03, DOCS-04 inherits: the architecture-doc relocation+content-gap split, the corrected (already-decided) rustdoc-warnings bar with ADR-0033's 20-warning residue, and the asciinema/demos path correction
- No `.rs`, `.project/`, or `.planning/intel/` file was modified by this plan (`git diff --name-only -- '*.rs' '.project/*' '.planning/intel/*' | wc -l` → `0`)

## Self-Check: PASSED

- FOUND: `.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-07-SUMMARY.md`
- FOUND: commit `962aaf9` (Task 1)
- FOUND: commit `2634d92` (Task 2)

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
