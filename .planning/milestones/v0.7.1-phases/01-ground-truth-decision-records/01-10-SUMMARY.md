---
phase: 01-ground-truth-decision-records
plan: 10
subsystem: testing
tags: [coverage, llvm-cov, adr, ci-gate, governance]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plan 01-09)
    provides: "The measurement of record — 84.79% workspace line coverage, human-approved, recorded in 01-coverage-measurement.md"
provides:
  - "ADR-0006: the single recorded coverage gate (84% floor, hard-fail, no operative target) that Phase 3 (QUAL-01), Phase 5 (VERIFY-05) and Phase 15 (PIPE-02) are authored against"
affects: [phase-3-verification-depth, phase-5-milestone-2-3-ground-truth, phase-15-coverage-ci-quality-gates]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR house shape (Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers), per PROMOTION.md and 0001-0005"]

key-files:
  created: [".planning/decisions/0006-coverage-gate.md"]
  modified: []

key-decisions:
  - "Scope: option-a selected at Task 1's blocking-human checkpoint — the locally-measured workspace default-feature scope (84.79%) is recorded as binding; the Docker-backed CI scope is named as scheduled work for VERIFY-05 and PIPE-02, not a defect."
  - "Exclusions confirmed as pinned: examples/, benches/, crates/doc-examples/, cargo registry and rustlib stdlib excluded via --ignore-filename-regex; doctests excluded."
  - "Deviation from D-09: the 80% target is retired as a superseded historical aspiration rather than recorded alongside the floor, because the measured baseline (84.79%) already exceeds it — recording both would ship a self-contradictory ADR (floor above target) and hand Phase 15 two numbers. Exactly one number (84%) is binding."
  - "Floor = 84% (measured 84.79% truncated down to a whole percent), hard-fail from the first run, at-or-above comparison, so the gate cannot be red on the run that set it."
  - "Ratchet trigger named per D-09 (raise to next whole percent below the then-current figure when a milestone closes 2+ points above the standing floor) but with no upper stop recorded, since D-09's literal 80% stop was contingent on floor < target, which no longer holds."
  - "Herald >= 95% and autonomous >= 90% module-scoped gates preserved, explicitly not withdrawn, handed to VERIFY-05."
  - "Code Conformance: must change — no coverage gate exists in ci.yml today; PIPE-02 named as the requirement that wires the 84% floor into CI."

requirements-completed: []  # RECON-07 intentionally NOT marked complete here — that is plan 01-12 Task 3's job, gated on this ADR existing. REQUIREMENTS.md is untouched by this plan.

coverage:
  - id: D1
    description: "ADR-0006 authored at .planning/decisions/0006-coverage-gate.md with the seven required H2 headings, status Accepted, and non-empty decisions/options_considered/key_files sections per adr-parser.cjs"
    verification:
      - kind: unit
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0006-coverage-gate.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR records exactly one measured figure (84.79%, byte-identical to 01-coverage-measurement.md) and one explicitly stated scope (workspace default-feature, Docker gap named)"
    verification:
      - kind: unit
        ref: "grep -c '84.79%' .planning/decisions/0006-coverage-gate.md (11 occurrences, all byte-identical)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Floor/target deviation from D-09 (84% floor, no operative target, 80% retired) recorded explicitly in the ADR's Decision section and in this SUMMARY's deviations, per the human's Task-1-adjacent instruction"
    verification: []
    human_judgment: true
    rationale: "Whether the deviation is framed clearly enough to bind Phase 15 without ambiguity is an editorial judgment call, not mechanically checkable."

duration: ~20min (post-checkpoint: Task 2 authoring + verification)
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 10: Coverage Gate ADR-0006 Summary

**ADR-0006 records the coverage gate as a single number — 84% hard-fail floor, no operative target — derived from the human-approved 84.79% workspace measurement, with the 80% target explicitly retired as superseded rather than silently dropped.**

## Performance

- **Duration:** ~20 min (Task 2 authoring, verification, and SUMMARY after the human resolved Task 1's checkpoint)
- **Completed:** 2026-07-31T16:33:53Z
- **Tasks:** 2 (Task 1: blocking-human checkpoint; Task 2: author ADR-0006)
- **Files modified:** 1 (`.planning/decisions/0006-coverage-gate.md`, created)

## Accomplishments

- Presented Task 1's `checkpoint:decision` (`gate="blocking-human"`) straight, without self-selecting, per the plan's explicit instruction that this gate is never auto-selected even under active auto-chain
- Authored `.planning/decisions/0006-coverage-gate.md` carrying D-07 through D-10 transcribed from the measurement record, plus a human-directed, explicitly-flagged deviation from D-09's literal 80%-as-target clause
- Recorded the single binding number — **84% floor, hard-fail from day one, no operative target** — with the truncation arithmetic shown explicitly (84.79% → 84%) and the at-or-above comparison stated
- Preserved both module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%) as not withdrawn, and flagged the 77.34%-vs-84.79% function/line coverage gap as context for VERIFY-05
- Named `VERIFY-05` (Phase 5), `PIPE-02` (Phase 15) and `QUAL-01` (Phase 3) as downstream consumers, and plan `01-12` as where the ROADMAP Phase 3 amendment lands
- Verified the ADR parses via `adr-parser.cjs` with `status: accepted` and non-empty `decisions` (98 entries), `options_considered` (30 entries) and `key_files` (7 entries)
- Left `.planning/REQUIREMENTS.md` untouched — RECON-07 remains `[ ]` / `Pending`, confirmed by diff against the pre-checkpoint commit; that flip belongs to plan 01-12 Task 3

## Task Commits

1. **Task 1: Lock the recorded coverage scope before ADR-0006 makes it binding** — checkpoint, no commit (human decision resolved via coordinator message: option-a selected, exclusions confirmed pinned, floor/target deviation directed)
2. **Task 2: Author ADR-0006 — one coverage number, one scope, one floor, one target, one ratchet** — `2635601` (docs)

**Plan metadata:** this commit (docs: complete plan — SUMMARY.md, STATE.md, ROADMAP.md, REQUIREMENTS.md unchanged)

## Files Created/Modified

- `.planning/decisions/0006-coverage-gate.md` - ADR-0006: the coverage gate record (84.79% measured, 84% floor, no target, both module gates preserved, `must change` conformance naming PIPE-02)

## Decisions Made

See `key-decisions` in frontmatter. In prose:

1. **Scope (Task 1, human decision via checkpoint): option-a.** The locally-measured workspace default-feature scope (84.79%) is recorded as binding. The Docker-backed `--features integration-tests` scope CI runs could not execute in this environment (Docker absent) and is named as scheduled work for VERIFY-05 and PIPE-02 rather than a defect swallowed into the number.
2. **Exclusions confirmed as pinned.** `examples/`, `benches/`, `crates/doc-examples/`, cargo registry, and rustlib stdlib sources stay excluded via `--ignore-filename-regex`; doctests stay excluded. Both stated plainly in the ADR's scope bullet, matching plan 01-09's original pinning.
3. **Floor vs. target — deviation from D-09, directed by the human outside Task 1's checkpoint (see Deviations below).** Floor = 84% (measured 84.79% truncated down), hard-fail, at-or-above comparison. 80% is retired as a superseded historical aspiration rather than recorded as a live target, because the measured baseline already exceeds it — recording both would ship a self-contradictory ADR and hand Phase 15 two numbers, the exact failure RECON-07 exists to close.
4. **Ratchet trigger has no recorded upper stop.** D-09's literal text stopped the ratchet at 80%, but that stop was contingent on the floor sitting below the target — no longer true once the floor (84%) exceeds the retired 80% figure. The ADR states this explicitly rather than silently carrying forward a now-meaningless stop condition.

## Deviations from Plan

### Auto-fixed / Human-directed Issues

**1. [Rule 4 - Architectural, resolved by explicit human decision, not self-selected] D-09's literal 80%-target clause set aside**

- **Found during:** Task 2 authoring, immediately after Task 1's checkpoint resolved
- **Issue:** D-09 specifies "the gate is the re-measured baseline rounded down to a whole percent... with 80% recorded in the same ADR as the target." D-09 was written when every candidate baseline (60.88%, 67.79%, ~76-78%) sat below 80%. The actual re-measurement is 84.79%, so the derived floor (84%) sits *above* 80%. Recording 80% as a live target alongside an 84% floor is self-contradictory (a floor exceeding its own target) and would hand Phase 15 two numbers to reconcile — precisely the "80 vs 85" failure RECON-07 exists to eliminate.
- **This was not resolved by my own judgment** — the coordinator raised it and the human decided, outside Task 1's checkpoint scope, per the message: "record floor = 84%, hard-fail from day one, and retire 80% as a superseded historical aspiration that the tree has already cleared. There is no operative target."
- **Fix:** ADR-0006's Decision section states the deviation explicitly, by name — a dedicated bullet titled "The target: superseded. There is no operative target — this is a deviation from D-09, recorded as one," explaining why (the baseline overtook the target) and that D-09's *intent* (a floor derived from the real measurement, unable to be red on day one) is honored while its stale literal 80%-target clause is set aside. The Considered Options section separately records "Retaining 80% as a recorded target alongside the 84% floor (D-09's literal text)" as a rejected option with its own reason.
- **Files modified:** `.planning/decisions/0006-coverage-gate.md`
- **Verification:** `grep` confirms the literal string "deviation from D-09" appears in the Decision section; `adr-parser.cjs` still parses the file with `status: accepted` and non-empty required fields after the addition.
- **Committed in:** `2635601` (Task 2 commit)

**2. [Not a code fix — documented for transparency] The plan's `<verify>` one-liner has a pre-existing extraction bug, unrelated to this task's output**

- **Found during:** Running the plan's exact automated `<verify>` command before committing
- **Issue:** The verify command's final check extracts the "measurement of record" figure via `grep -oE '[0-9]+\.[0-9]+%' .../01-coverage-measurement.md | head -1` and requires the ADR to contain that string byte-identical. The *first* percentage-shaped match in `01-coverage-measurement.md` is `16.32%`, from the earlier Task 1 tracer-probe section (a `paladin-ai-core`-only sanity check, not the workspace measurement of record) — not `84.79%` from the "Measurement of record" section further down the file. Running the verify command literally as written therefore fails at the last check, even though the ADR correctly and byte-identically records `84.79%` (confirmed 11 occurrences via direct grep).
- **This is a defect in the plan's own automated verify one-liner** (an ordering assumption that does not hold given the tracer section precedes the measurement-of-record section in the same file), not an issue with the ADR content. No plan file was edited to fix it — out of this plan's `files_modified` scope (`.planning/decisions/0006-coverage-gate.md` only).
- **Resolution:** Manually confirmed the true acceptance criterion — "The percentage stated in the ADR as the measured figure is byte-identical to the workspace percentage in `01-coverage-measurement.md`" — is met: `84.79%` appears 11 times in the ADR, byte-identical to the TOTAL row and the "single workspace line-coverage percentage" prose sentence in the measurement record. Re-ran the full verify one-liner substituting the correct measurement-of-record figure and confirmed `ADR0006_OK`.
- **Files modified:** none (documentation-only finding)
- **Verification:** `grep -c '84.79%' .planning/decisions/0006-coverage-gate.md` → 11; full verify one-liner with corrected `M="84.79%"` → `ADR0006_OK`
- **Committed in:** N/A (no fix required to my deliverable; noted here per the broken-windows/deviation transparency requirement)

---

**Total deviations:** 1 human-directed architectural deviation (Rule 4, explicit human decision) + 1 pre-existing plan-verify-script defect (documented, not fixed, no impact on the ADR's correctness)
**Impact on plan:** The floor/target deviation is exactly what the human asked to be recorded as a deviation rather than silently absorbed — done. The verify-script defect has zero impact on ADR-0006's content or on the acceptance criteria it was meant to check; it is flagged for visibility rather than silently worked around.

## Issues Encountered

None beyond the two items above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0006 exists and is machine-parseable; Phase 3's QUAL-01, Phase 5's VERIFY-05, and Phase 15's PIPE-02 can now author work against a single 84% floor rather than a contested six-way corpus.
- Plan 01-12 Task 3 still owns flipping RECON-07 to complete in `REQUIREMENTS.md` and amending ROADMAP.md's Phase 3 success criterion 1 — both deliberately left untouched here.
- VERIFY-05 (Phase 5) inherits two concrete, unfinished obligations recorded in this ADR: extending the scope to the Docker-backed integration-tests suite, and addressing the 77.34%-vs-84.79% function/line coverage gap.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0006-coverage-gate.md`
- FOUND: `.planning/phases/01-ground-truth-decision-records/01-10-SUMMARY.md`
- FOUND: commit `2635601` in git log
