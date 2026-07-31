---
phase: 01-ground-truth-decision-records
plan: 06
subsystem: docs
tags: [ground-truth, decision-records, ledger, paladin, garrison, arsenal, battalion, commander, requirements-audit]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records
    provides: ".planning/ledgers/milestone-01.md header, D-19 evidence bar, D-20 verdict legend, and the flagged REPL/NG-7 divergence row (plan 01-01); RECON-08 Epic 10 dispute resolution, the two remaining divergence rows and the RECON-01 bookkeeping corrections (plan 01-05)"
provides:
  - "60 REQ-* rows authored across Epics 1-5 (Paladin Domain, Garrison Memory, Arsenal Tools, Battalion Orchestration, Commander Strategy Router), each re-verdicted under the D-20 legend with a freshly re-checked file:line citation"
  - "13 outstanding Epic 2/3/4/5 task items nested under their owning requirement, quoted by literal checkbox text with task-file citations"
  - "Six upgrades from the 2026-01 task-list snapshot's Partial/Verify status to satisfied, each backed by a live 2026-07-31 test run: REQ-arsenal-port, REQ-arsenal-resilience, REQ-phalanx-concurrency, REQ-campaign-execution, REQ-chain-of-command-execution, REQ-commander-auto-selection, REQ-commander-result-normalization/telemetry"
  - "One new genuinely-outstanding finding (RECON-04-class): REQ-formation-min-paladins-v2's full behavioral claim (a single-Paladin Battalion executes via Formation) does not hold in the shipped tree today — Formation::validate rejects it even though the Commander's Auto-selection test passes — forward-owned by GAP-07 per ADR-0003"
  - "REQ-battalion-config-v1/-v2, REQ-battalion-result-v2 and REQ-formation-min-paladins-v1/-v2 rows point at ADR-0001/0002/0003 instead of re-deciding"
affects: [01-07, phase-02, phase-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Evidence-bar re-verification pattern: for every satisfied verdict, re-run the citation's cargo test (not trust the 2026-01 task-list annotation), and record the upgrade explicitly when the gap has since closed"
    - "Stale-parent-checkbox pattern: when all of a parent task's own subtasks are checked but the parent itself is not, verify against the tree before classifying — this corpus has already found this shape wrong in both directions (Epic 4 tasks 6.0/7.0, Epic 5 task 5.0)"
    - "ADR-routing pattern: rows whose subject an ADR settles carry the bare D-20 verdict string in the Verdict column and push the ADR cross-reference into the Evidence column, so no row's verdict is decorated with anything beyond the five legend strings"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-01.md

key-decisions:
  - "REQ-formation-min-paladins-v1 is recorded satisfied (matches the code as it stands today — Formation::validate rejects <2) while REQ-formation-min-paladins-v2 is recorded genuinely outstanding (the Commander already selects Formation for a single Paladin, but that Paladin would fail Formation::validate at execution) — per ADR-0003's own framing, an ADR that has not yet landed does not make the tree conform to it"
  - "REQ-arsenal-port, REQ-arsenal-resilience, REQ-phalanx-concurrency, REQ-campaign-execution, REQ-chain-of-command-construction/execution/aggregation, REQ-commander-auto-selection and REQ-commander-result-normalization/telemetry are upgraded from their 2026-01 Partial/Verify status to satisfied, each backed by a live `cargo test` run on 2026-07-31 rather than carried forward on the strength of the old note — the corpus's own precedent (checkbox state wrong in both directions) applied here in the direction of the code being further along than the January snapshot recorded"
  - "Epic 4 tasks 6.0 (Chain of Command) and 7.0 (integration/perf/docs) are both recorded satisfied despite their own parent checkboxes remaining unchecked: all subtasks under each are individually checked, and direct re-verification (a 54/54-passing `cargo test --test lib chain_of_command` run, plus existence checks for `load_test.rs`, the benchmark suite and the Chain-of-Command example) confirms the parent checkbox is stale, not the work"
  - "Epic 5 task 5.0 and its two open children (5.10, 5.14) are recorded satisfied: `export_metadata` at `commander.rs:880` implements exactly what 5.10 asked for and `test_metadata_export_creates_file`/`test_metadata_export_json_structure` are exactly the test 5.14 asked for, under different names than the task list anticipated — the '(deferred - requires file I/O)' annotations do not match the shipped tree"
  - "REQ-commander-error-strategy keeps its satisfied verdict (three real, non-ignored tests exercise all three strategies) but records the residual caveat honestly: the same four `#[ignore]`-d, empty-bodied edge-case tests the 2026-01 note counted (test_fail_fast_stops_on_first_error, test_continue_on_error_collects_all_errors, test_retry_then_continue_retries_failed_paladins, test_partial_results_returned_with_errors) are still ignored today, forward-owned by QUAL-04"
  - "REQ-paladin-observability and REQ-battalion-logging both stay present, unproven rather than being upgraded on the strength of the code existing — log/tracing calls are real and cited, but no named test in this tree asserts log output content, so the exerciser half of the D-19 bar is genuinely unmet"
  - "Kept every row's Verdict column to the bare D-20 legend string with no decoration (a mid-plan correction — see Deviations) — the six ADR-routed rows carry their ADR cross-reference in the Evidence column instead, so 'exactly one of the five legend verdicts' holds literally, not just in spirit"

requirements-completed: []

coverage:
  - id: D1
    description: "Epic 1-3 (Paladin Domain, Garrison Memory, Arsenal Tools) — 27 REQ-* rows re-verdicted with re-checked citations, 7 outstanding task items nested by literal checkbox text"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "grep -q '### Epic 1 ' .planning/ledgers/milestone-01.md && grep -q '### Epic 2 ' .planning/ledgers/milestone-01.md && grep -q '### Epic 3 ' .planning/ledgers/milestone-01.md && awk row-count >= 27 REQ- rows && tasks-*.md citation count >= 7 && verdict-string grep matches — all pass, EPIC123_OK"
        status: pass
    human_judgment: false
  - id: D2
    description: "Epic 4-5 (Battalion Orchestration, Commander Strategy Router) — 33 REQ-* rows re-verdicted, 6 outstanding task items nested, ADR-0001/0002/0003 linked without contradiction"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "grep -q '### Epic 4 ' .planning/ledgers/milestone-01.md && grep -q '### Epic 5 ' .planning/ledgers/milestone-01.md && awk row-count >= 33 REQ- rows && tasks-*.md citation count >= 6 && grep -q decisions/0001-battalion-config.md && grep -q decisions/0003-formation-min-paladins.md — all pass, EPIC45_OK"
        status: pass
    human_judgment: false

duration: ~50min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 06: Epic 1-5 ledger rows for Milestone 1 Summary

**Authored 60 citation-backed `REQ-*` rows across Epics 1-5 in `.planning/ledgers/milestone-01.md`, re-running the D-19 evidence bar against the shipped tree rather than carrying forward the 2026-01 task-list snapshot — six requirements upgrade to `satisfied` on fresh evidence, one (REQ-formation-min-paladins-v2) is newly recorded `genuinely outstanding` because the tree contradicts itself.**

## Performance

- **Duration:** ~50 min
- **Completed:** 2026-07-31
- **Tasks:** 2/2
- **Files modified:** 1

## Accomplishments

- Epic 1 (Paladin Domain Foundation): 8 rows, all `satisfied` except `REQ-paladin-observability` (`present, unproven` — real `log`/`env_logger` calls exist, but no test asserts output)
- Epic 2 (Garrison Memory System): 10 rows plus 4 nested outstanding items (9.14 deferred with reason, 11.0/11.5/11.6 split into present-unproven/genuinely-outstanding/genuinely-outstanding rather than treated as one undifferentiated block)
- Epic 3 (Arsenal Tool System): 9 rows plus 3 nested items (the epic's git-workflow closure steps, recorded superseded by shipped code since the deliverable they describe is fully present in the tree)
- Epic 4 (Battalion Orchestration): 20 rows plus 2 nested items — both open parent tasks (6.0 Chain of Command, 7.0 Integration/Perf/Docs) are recorded satisfied after direct re-verification found every subtask complete and the parent checkbox stale
- Epic 5 (Commander Strategy Router): 13 rows plus 4 nested items — the one item the 2026-01 task list flagged "FAILING - needs fix" (`test_auto_selects_campaign_for_workflow_keywords`) passes cleanly on a fresh 2026-07-31 run, and the two Task 5.0 children marked "(deferred - requires file I/O)" turn out to already be implemented and tested
- `REQ-battalion-config-v1/-v2`, `REQ-battalion-result-v2` and `REQ-formation-min-paladins-v1/-v2` all point at ADR-0001/0002/0003 rather than re-deciding; no row's stated verdict contradicts its linked ADR
- Live re-verification (not memory or the 2026-01 notes) drove six upgrades to `satisfied`: `REQ-arsenal-port`, `REQ-arsenal-resilience`, `REQ-phalanx-concurrency`, `REQ-campaign-execution`, `REQ-chain-of-command-construction/execution/aggregation`, `REQ-commander-auto-selection`, `REQ-commander-result-normalization`/`REQ-commander-telemetry` — each citation is backed by a `cargo test` run performed during this plan, not carried forward

## Task Commits

Each task was committed atomically:

1. **Task 1: Ledger rows for Epics 1-3** - `d68a15b` (feat)
2. **Task 2: Ledger rows for Epics 4-5** - `9f71c54` (feat)
3. **Fixup: keep Verdict column to the bare D-20 legend string** - `f086c96` (fix) — see Deviations below

## Files Created/Modified

- `.planning/ledgers/milestone-01.md` — appended `### Epic 1` through `### Epic 5` sections (60 `REQ-*` rows, 13 nested outstanding-item bullets)

## Decisions Made

See `key-decisions` in frontmatter above for the full list. In short: the Formation-minimum contradiction (RECON-04) is recorded honestly as split across two verdicts rather than smoothed into one; six requirements upgrade to `satisfied` on fresh evidence rather than being carried forward on the 2026-01 note; and two Epic 4 parent checkboxes plus one Epic 5 parent checkbox are recorded satisfied despite remaining unchecked, because direct re-verification — not inference — found the underlying work complete.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Verdict column carried ADR cross-references and a "(today)" qualifier, violating "exactly one of the five legend verdicts"**
- **Found during:** Self-review after Task 2's commit, before writing this SUMMARY
- **Issue:** Six Epic 4/5 rows settled by ADR-0001/0002/0003 initially wrote their Verdict column as e.g. `satisfied — see [ADR-0001](...)` or `satisfied (today) — see [ADR-0003](...)`. This still greps as containing the word "satisfied" (so the plan's own automated `<verify>` commands passed), but the plan's acceptance criterion — "Every row under those headings carries exactly one of the five legend verdict strings" — reads as requiring the bare string, not a decorated one.
- **Fix:** Moved the ADR cross-reference and qualifier text into the Evidence column on all six affected rows (`REQ-battalion-config-v1`, `REQ-battalion-config-v2` ×2, `REQ-formation-min-paladins-v1`, `REQ-battalion-result-v2`, `REQ-formation-min-paladins-v2`), leaving the Verdict column as exactly `satisfied`, `superseded by shipped code` or `genuinely outstanding`.
- **Files modified:** `.planning/ledgers/milestone-01.md`
- **Verification:** `awk -F'|' '/^### Epic [1-5] /,0' ... | awk '$2 ~ /REQ-/ {print $3}' | sort -u` returns exactly the four verdict strings used in this range (`genuinely outstanding`, `present, unproven`, `satisfied`, `superseded by shipped code`), no decorated variants
- **Committed in:** `f086c96` (separate fix commit, not folded into either task commit, since it corrects both tasks' output)

---

**Total deviations:** 1 auto-fixed (Rule 1 — correctness of the verdict-string contract)
**Impact on plan:** No scope creep; a self-caught correctness fix on the plan's own acceptance bar before handoff.

## Issues Encountered

- The plan's `read_first` pointed at `REQUIREMENTS.md` lines 2361-2450/2420-2465 for the Epic 1-5 as-shipped ledger tables; the actual lines (after prior phase-1 plans added the RECON-08 and divergence sections earlier in the file) are 2531-2614. Located by heading search (`grep -n '^### Epic'`) rather than trusting the stale line numbers — same "verify against the tree" discipline the rest of this plan applies to code citations.
- Several 2026-01 task-list notes proved stale in the direction of *understating* completion (Chain of Command, Commander telemetry export, the one "FAILING" keyword test) — consistent with this corpus's own established pattern (Milestone 1 ingest run 1 already found this for Chain of Command and Herald wiring) — re-verified rather than assumed.
- No blockers.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- `.planning/ledgers/milestone-01.md` now holds header/legend/divergences/RECON-08/bookkeeping (01-01, 01-05) plus complete `REQ-*` rows for Epics 1-5 (this plan, 60 rows / 13 nested items). Epics 6-10 remain `## Epic N` placeholder headings (two `#`, not three) for plan 01-07 to fill and promote to `### Epic N`.
- **RECON-01 is intentionally left un-ticked in `REQUIREMENTS.md`** — this plan covers Epics 1-5 only; plan 01-07 authors the remaining Epics 6-10 rows and owns flipping RECON-01's checkbox and traceability row once the full ledger is complete. `requirements-completed` is deliberately empty in this SUMMARY's frontmatter for the same reason.
- One new forward-work item surfaced, not previously named as sharply: `REQ-formation-min-paladins-v2`'s full behavioral claim does not hold in the tree today (Formation::validate still rejects a single Paladin even though the Commander routes one there). This is not new work — it is exactly what ADR-0003 and GAP-07 already own — but this plan is where it is recorded against the specific `REQ-*` ID with a re-verified citation.
- No blockers for plan 01-07.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
