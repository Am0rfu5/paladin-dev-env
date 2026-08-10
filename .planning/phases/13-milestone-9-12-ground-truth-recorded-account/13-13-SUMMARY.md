---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 13
subsystem: docs
tags: [close-out, ledger-integrity, adr-promotion, requirements-traceability, phase-gate]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "all twelve prior plans (13-01 through 13-12) — the complete 120-row ledger, ADR-0037/0038/0039, the source-level ORCH-03 relocation annotations, and ADR-0029's completed Trajectory table"
provides:
  - "The ledger's dated close-out amendment: whole-file integrity re-check, per-class verdict distribution, cited-not-re-derived and CI-only lists, the zero-.rs-diff boundary with its base SHA, an honest mdbook comparison"
  - "ORCH-01 through ORCH-05 flipped to [x] with closing evidence, traceability rows Complete"
  - "Three forward hand-off blocks (Phase 14 / WEB-01..WEB-04, Phase 15 / PIPE-01.., Phase 16 / DOCS-01..DOCS-04)"
  - "PROMOTION.md at 0040 with a complete advancing note; Part B candidates 8 and 9 both explicitly dispositioned"
  - "PROJECT.md Key Decisions rows for ADR-0037, ADR-0038, ADR-0039"
affects: [phase-14-web-api-followups, phase-15-deferred-qa-closure, phase-16-docs-closure]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Close-out amendment shape copied verbatim from milestone-07-08.md's Phase 10 close-out: integrity re-check, verdict distribution, cited-not-re-derived list, CI-only claims, phase gate, series-completion statement"
    - "Self-referential grep avoidance: a close-out amendment that quotes its own verification commands must not reproduce the exact literal pattern it is counting, or the count is inflated by its own quotation — worked around by describing patterns in prose rather than literal reproduction where the same file both runs and reports the check"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-09-12.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md

key-decisions:
  - "The plan's own acceptance criterion expecting 29 section headers in the ledger is a plan-authoring miscount, not a tree defect — the actual, correctly-scoped count is 28, independently corroborated by REQUIREMENTS.md:3672's own pointer text (written by plan 13-10, before this plan ran). Recorded in the close-out amendment rather than manufacturing a spurious 29th heading, per this phase's own established pattern for 13-09/13-10/13-12's comparable acceptance-criteria defects."
  - "The ledger's 'Verified open' highlight table (19 rows) does not literally match the count of rows carrying 'Verified open' as their primary Verdict-cell classification (12) — 8 of the 19 use variant phrasing ('Partially open', 'Open,', 'Open register', or embed 'Verified open' only as one half of a dual/split-verdict row) that four different fan-out plans wrote without a shared vocabulary for the nuance. Documented in full in the close-out amendment rather than silently reconciled or rewritten (rewriting five other plans' rows is out of this plan's files_modified scope)."
  - "REQ-fail-closed-auth-posture (Milestone 12 Epic 5) is a genuine 'Verified open' finding this close-out's integrity re-check surfaced that was not previously in the Deferred-QA highlight table and has no owning downstream requirement. Recorded in the ledger close-out amendment and named to Phase 14 as the nearest natural owner; not written to 13-CONTEXT.md's <deferred> register because that file is outside this plan's declared files_modified, and no new phase scope is opened for it here per the plan's own prohibition."
  - "PROMOTION.md Part B candidate 9 (the Milestone 9 Epic 4 agent/orchestrator bridge decision) is recorded NOT promoted this phase, redirected to Phase 14, citing 13-09-SUMMARY.md's own Checkpoint Status record rather than re-deriving or re-deciding the human's disposition."

requirements-completed: [ORCH-01, ORCH-02, ORCH-03, ORCH-04, ORCH-05]

coverage:
  - id: D1
    description: "Ledger's dated close-out amendment: whole-file integrity re-check (120 rows, 120 distinct IDs, all ADR citations resolve), per-class verdict distribution reconciled against both highlight tables, cited-not-re-derived and CI-only lists, D-19 boundary with base SHA, honest mdbook comparison"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md -> 120; grep -oE '^\\| REQ-[a-z0-9-]+' | sort -u | wc -l -> 120; grep -c 'pending — plan' -> 0; ADR resolution loop over 9 cited numbers -> no MISSING output; git diff --name-only <base>..HEAD -- '*.rs' | wc -l -> 0; mdbook build docs/ -> exit 101, identical two pre-existing errors, no error naming sidecar.md/http-service-host.md/overview.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "ORCH-01..05 flipped to [x] with closing evidence named inline; five traceability rows read Phase 13 | Complete; three dated hand-off blocks name Phases 14, 15, 16 with per-requirement citations"
    requirement: "ORCH-02"
    verification:
      - kind: other
        ref: "grep -cE '^- \\[x\\] \\*\\*ORCH-0[1-5]\\*\\*' REQUIREMENTS.md -> 5; grep -cE '^- \\[ \\] \\*\\*ORCH-0[1-5]\\*\\*' -> 0; grep -cE '^\\| ORCH-0[1-5] \\| Phase 13 \\| Complete \\|' -> 5; grep -cE '^#### Hand-off to Phase 1[456]' -> 3; grep -c 'WEB-04' -> 10; grep -c 'kubernetes-smoke' -> 3; grep -c 'DOCS-02' -> 7; grep -c 'tool-integration.md:324' -> 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md next-free line reads 0040 with a complete advancing note naming plan 13-08 (ADR-0037) and plan 13-09 (ADR-0038, ADR-0039), both Part B dispositions (candidate 8 closed, candidate 9 not promoted/redirected) stated explicitly; PROJECT.md carries one Key Decisions row per new ADR"
    requirement: "ORCH-04"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0040' PROMOTION.md -> 1; grep -c 'Next free ADR number: 0037' -> 0; grep -ciE 'candidate 9' -> 2; grep -cE '13-08|13-09' -> 9; grep -cE 'ADR-0037|ADR-0038|ADR-0039' PROJECT.md -> 3; ls .planning/decisions/003[789]-*.md | wc -l -> 3"
        status: pass
    human_judgment: false
  - id: D4
    description: "ORCH-05 closed citing ADR-0029's completed Trajectory table (plan 13-12) and ADR-0030's existing closure of the fifth-numbering-collision prediction"
    requirement: "ORCH-05"
    verification:
      - kind: other
        ref: "grep -c 'ADR-0030' .planning/REQUIREMENTS.md ORCH-05 closing note -> present; grep -cE '\\| ?v0\\.[3-6]\\.0' .planning/decisions/0029-version-trajectory-history.md -> 4 (pre-existing, confirmed not re-derived)"
        status: pass
    human_judgment: false

duration: ~70min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 13: Milestone 9-12 Close-Out Summary

**Ledger internally re-verified whole (120/120 rows, all ADR citations resolving, a documented 28-vs-29 section-count plan-authoring defect and a documented 12-vs-19 "Verified open" phrasing gap — including one genuinely new open finding, `REQ-fail-closed-auth-posture`), the code-change boundary measured zero against a named base SHA, all five ORCH requirements closed with evidence, three dated hand-offs written for Phases 14/15/16, and the ADR index advanced to 0040 with both Part B dispositions stated.**

## Performance

- **Duration:** ~70 min
- **Started:** 2026-08-10
- **Completed:** 2026-08-10
- **Tasks:** 3 of 3
- **Files modified:** 4 (`.planning/ledgers/milestone-09-12.md`, `.planning/REQUIREMENTS.md`, `.planning/decisions/PROMOTION.md`, `.planning/PROJECT.md`)

## Accomplishments

- Appended `## Phase 13 close-out amendments (2026-08-10)` to the ledger: a whole-file integrity
  re-check (120 rows, 120 distinct IDs, 28 section headers, zero blank verdict cells, all 9 cited
  ADR numbers resolving), a per-class verdict distribution table built by classifying every row's
  actual Verdict-cell text (not assumed from the input 35/53/32 split), the rows-cited-rather-than-
  re-derived and CI-only lists with commands re-run this session, and a phase gate recording the
  base SHA (`e12f18306ca9a80b1c3301e6afca31602e7c41ec`), a zero `.rs`-file diff, and an `mdbook build`
  re-run matching the recorded baseline error-for-error.
- Discovered and documented two genuine discrepancies during the integrity re-check, rather than
  silently reconciling or "fixing" them: the plan's own acceptance criterion expecting 29 section
  headers is a miscount (actual, correctly-scoped count is 28, independently corroborated by
  `REQUIREMENTS.md:3672`); and the ledger's "Verified open" highlight table (19 rows) does not
  literally match the 12 rows whose primary classification reads "Verified open" — 8 of the 19 use
  variant phrasing four different fan-out plans wrote without a shared vocabulary, and one row
  (`REQ-fail-closed-auth-posture`) is genuinely `Verified open` but was never added to the highlight
  table or given a downstream owner.
- Flipped all five ORCH-01 through ORCH-05 checkboxes to `[x]` (ORCH-03 was already flipped by plan
  13-08 without evidence prose; evidence added here), each with the closing evidence named inline,
  and updated all five traceability rows to `Phase 13 | Complete`.
- Wrote three dated hand-off blocks — Phase 14 / WEB-01..WEB-04, Phase 15 / PIPE-01.., Phase 16 /
  DOCS-01..DOCS-04 — each citing ledger rows and ADRs by ID rather than repeating their content,
  stating in the Phase 16 block that this is the corpus's last ground-truth phase.
- Advanced `PROMOTION.md`'s `Next free ADR number` line from `0037` to `0040`, added a dated
  advancing note naming plan 13-08 as ADR-0037's author and plan 13-09 as ADR-0038's and ADR-0039's,
  and stated both Part B dispositions explicitly: candidate 8 (`AgentProvisioner` placement) closed
  by ADR-0038; candidate 9 (the M9 Epic 4 agent/orchestrator bridge decision) **not** promoted this
  phase, redirected to Phase 14, citing `13-09-SUMMARY.md`'s own Checkpoint Status record for the
  human's disposition rather than re-deriving it.
- Added three rows to `PROJECT.md`'s `## Key Decisions` table, one per new ADR.

## Task Commits

Each task was committed atomically:

1. **Task 1: Append the dated close-out amendment to the ledger and run the phase gate** - `cf81700` (docs)
2. **Task 2: Close ORCH-01..ORCH-05 and write the three forward hand-off blocks** - `ba4c3eb` (docs)
3. **Task 3: Advance PROMOTION.md to 0040 with a complete advancing note, and add the PROJECT.md Key Decisions rows** - `55b6fe1` (docs)

**Plan metadata:** committed alongside this SUMMARY (worktree mode — STATE.md/ROADMAP.md excluded,
owned by the orchestrator after merge).

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` - `## Phase 13 close-out amendments (2026-08-10)` appended at the foot; no epic section above it modified
- `.planning/REQUIREMENTS.md` - five `[x]` ORCH checkboxes with closing evidence, five `Phase 13 | Complete` traceability rows, three `#### Hand-off to Phase 14/15/16` blocks
- `.planning/decisions/PROMOTION.md` - next free `0040`, dated advancing note, both Part B dispositions (candidates 8 and 9) stated
- `.planning/PROJECT.md` - three Key Decisions rows (ADR-0037, ADR-0038, ADR-0039)

## Decisions Made

See `key-decisions` in frontmatter. In summary: the plan's own 29-header acceptance criterion is a
miscount (real count is 28, corroborated independently); the ledger's Verified-open highlight table
undercounts by phrasing variance and is missing one genuinely open row (`REQ-fail-closed-auth-posture`,
recorded here and named to Phase 14, no new phase scope opened); PROMOTION.md candidate 9 is
explicitly not promoted this phase and redirected to Phase 14, citing the checkpoint record rather
than re-deciding it.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed a self-referential grep count in the ledger's own close-out amendment**

- **Found during:** Task 1, while verifying the amendment's own stated acceptance criteria after writing it
- **Issue:** The amendment's first draft quoted the exact literal grep patterns `pending — plan` and
  `run-5 input (not yet re-derived)` inline (as part of describing what each check does), which
  caused those same patterns to match the amendment's own prose on re-run — inflating
  `grep -c 'pending — plan'` from the required `0` to `1`, purely because the file now quoted its own
  search string.
- **Fix:** Rewrote both bullets to describe the marker conventions in prose without reproducing the
  exact matchable substring, and added a row-anchored search (`^| REQ-.*run-5 input`) to state the
  invariant that actually matters — zero live markers in any row — precisely, while being transparent
  that the raw literal-string count is non-zero for descriptive reasons the amendment itself explains.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** re-ran `grep -c 'pending — plan' .planning/ledgers/milestone-09-12.md` → `0` after
  the fix; `grep -c "^| REQ-.*run-5 input" .planning/ledgers/milestone-09-12.md` → `0`.
- **Committed in:** `cf81700` (Task 1 commit; the fix was made before the task's own commit, not as a
  separate follow-up)

---

**Total deviations:** 1 auto-fixed (1 bug — a self-reference the amendment's own drafting introduced).
No `.rs` file touched, no prohibited file modified, no fourth ADR authored, no requirement checkbox
flipped without evidence named in the same edit.
**Impact on plan:** None on scope or content. The fix was necessary for the amendment's own stated
acceptance criteria to actually hold on re-run, exactly the kind of measured-not-claimed rigor this
plan's own action text requires throughout.

## Issues Encountered

None beyond the self-referential grep fix documented above. The `mdbook-mermaid` gitignored-asset
regeneration step (`mdbook-mermaid install docs/`) that plan 13-09 first performed was repeated here
in this fresh worktree, producing no tracked-file change, exactly as that plan's own precedent
predicted.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All five ORCH-01..05 requirements are closed with evidence; the Milestone 9-12 ledger series is
  complete (`milestone-01.md`, `milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`,
  `milestone-09-12.md` — no sixth sibling to forward).
- Three dated, cited hand-off blocks are in place for Phases 14 (WEB-01..WEB-04), 15 (PIPE-01..) and
  16 (DOCS-01..DOCS-04) — the corpus's final forward-work signal, since this is the last
  ground-truth phase.
- `PROMOTION.md` is at `0040`; the ADR index has no gaps and no unresolved dispositions. Candidate 9's
  redirection to Phase 14 is the one open item a future planner should read before scoping that phase.
- One genuinely new finding surfaces from this close-out with no current owner:
  `REQ-fail-closed-auth-posture` (Milestone 12 Epic 5, `Verified open` — the fail-closed auth posture
  is code-present but untested). Recorded in the ledger's close-out amendment and named to Phase 14
  as the nearest natural owner; not converted into a requirement or written to `13-CONTEXT.md` (out
  of this plan's `files_modified` scope), per the plan's own no-new-scope prohibition.
- No blockers for the orchestrator's post-wave STATE.md/ROADMAP.md/REQUIREMENTS.md-summary updates.

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
