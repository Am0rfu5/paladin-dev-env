---
phase: 03-verification-depth
plan: 01
subsystem: testing
tags: [coverage, llvm-cov, llvm-profdata, rustc-instrument-coverage, adr-0006, qual-01, qual-02]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records
    provides: ADR-0006's coverage-gate command, ignore regex, doctest exclusion, and the
      01-coverage-measurement.md provenance skeleton this plan reproduces
provides:
  - "03-coverage-measurement.md: entry measurement of workspace line coverage at Phase 3 HEAD"
  - "Confirmed 85.56% workspace line coverage PASSES ADR-0006's 84.00% floor by 1.56 points"
  - "Re-derived first-party zero-coverage set (5 files), identical to Phase 1's D-04 set"
  - "QUAL-02 staleness table: 9 of 11 named offenders contradicted, 1 confirmed, 1 not in denominator"
  - "Ratchet readiness recorded: 1.56pt delta, below the 2pt trigger; ADR-0006 not amended"
affects: [03-02, 03-03, 03-04, 03-05, 03-06, 03-07, VERIFY-05, PIPE-02]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Offline rustc -C instrument-coverage -> llvm-profdata merge -> llvm-cov report pipeline,
       reproduced verbatim from ADR-0006 with zero flag/regex changes"

key-files:
  created:
    - .planning/phases/03-verification-depth/03-coverage-measurement.md
  modified: []

key-decisions:
  - "Measured 85.56% workspace line coverage at HEAD bb35554d — PASSES the 84.00% floor by 1.56 points"
  - "Zero-coverage set unchanged from Phase 1 (5 files: redis.rs, paladin-server.rs, file_storage_port.rs, error.rs, arsenal_port.rs) — no files entered or left the set across 98 commits"
  - "QUAL-02's 11 named offenders: 9 contradicted by this measurement, 1 confirmed (redis.rs), 1 not in denominator (minio.rs, s3 feature not in default set)"
  - "Ratchet trigger not met (1.56pt delta < 2.00pt threshold) — Phase 3 does not amend ADR-0006; raise decision belongs to the milestone-close audit"
  - "Corrected a column-selection inconsistency in CONTEXT.md D-04's prose (it cited the Regions column, not the Lines column, for counted-line figures) — recorded as a footnote, not a re-decision"

patterns-established: []

requirements-completed: [QUAL-01]

coverage:
  - id: D1
    description: "Reproduce ADR-0006's coverage pipeline verbatim at current HEAD and record the
      entry measurement with full D-16 provenance (rustc -vV, cargo --version, git rev-parse HEAD,
      git status --porcelain, date -u)"
    requirement: "QUAL-01"
    verification:
      - kind: other
        ref: "llvm-cov report TOTAL row pasted in 03-coverage-measurement.md, transcribed
          character-for-character: Lines 62953 9088 85.56%"
        status: pass
    human_judgment: false
  - id: D2
    description: "Compare the measured figure at-or-above ADR-0006's 84% floor with explicit
      PASS/FAIL verdict and truncation-toward-zero arithmetic"
    requirement: "QUAL-01"
    verification:
      - kind: other
        ref: "03-coverage-measurement.md ### Measurement of record: 85.56% >= 84.00% -> PASS,
          margin 1.56pp; truncated 85% >= 84%"
        status: pass
    human_judgment: false
  - id: D3
    description: "Re-derive the first-party zero-coverage set from this run's own per-file rows,
      excluding tests/** and target/, and compare against CONTEXT.md D-04's expected five"
    verification:
      - kind: other
        ref: "03-coverage-measurement.md ### Re-derived first-party zero-coverage set: 5 rows,
          identical set to D-04"
        status: pass
    human_judgment: false
  - id: D4
    description: "Record the QUAL-02 staleness comparison table pairing all 11 named offenders
      with this run's measured figures, and the two scope exclusions (target/ contamination,
      minio.rs absence) with magnitude and owner"
    verification:
      - kind: other
        ref: "03-coverage-measurement.md ### QUAL-02 offender list and ### Scope exclusions and
          denominator notes"
        status: pass
    human_judgment: false
  - id: D5
    description: "Record the ratchet arithmetic and the milestone-close-only decision without
      amending ADR-0006"
    verification:
      - kind: other
        ref: "03-coverage-measurement.md ### Ratchet readiness: 1.56pt delta, condition not met"
        status: pass
    human_judgment: false

duration: 16min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 01: Coverage Entry Measurement Summary

**Reproduced ADR-0006's offline LLVM coverage pipeline verbatim at Phase 3 HEAD: 85.56% workspace line coverage, PASSING the 84% floor by 1.56 points, with an unchanged 5-file zero-coverage set and a QUAL-02 staleness table showing 9 of 11 named offenders are stale claims.**

## Performance

- **Duration:** ~16 min
- **Started:** 2026-08-02T14:33:17Z
- **Completed:** 2026-08-02T14:49:13Z
- **Tasks:** 2 completed
- **Files modified:** 1 (`.planning/phases/03-verification-depth/03-coverage-measurement.md`, new)

## Accomplishments

- Ran the full ADR-0006 coverage pipeline end to end at HEAD `bb35554dd910a143725b6f149a347959130f9456`
  (98 commits past ADR-0006's measurement commit, including all of Phase 2's new tests): instrumented
  `cargo test --workspace --offline` (35 `test result: ok.` lines, 0 failures), `llvm-profdata merge`
  (15.2MB `paladin.profdata`), object discovery (31 test-binary objects, matching Phase 1's count
  exactly), and `llvm-cov report` with the identical `--ignore-filename-regex`.
- Measured workspace line coverage: **85.56%** (62,953 lines counted, 9,088 missed) — **PASS**
  against ADR-0006's 84.00% floor by 1.56 percentage points. Truncation toward zero (85%) also
  clears the floor.
- Re-derived the first-party zero-coverage set from this run's own per-file rows: exactly the same
  5 files Phase 1 found (`redis.rs`, `paladin-server.rs`, `file_storage_port.rs`, `error.rs`,
  `arsenal_port.rs`) — zero entered, zero left across 98 commits.
- Built the QUAL-02 offender staleness table for all 11 files `REQUIREMENTS.md:262-267` names: 9
  contradicted (corrected, not deleted), 1 confirmed (`redis.rs`), 1 not in the denominator
  (`minio.rs`, gated behind the non-default `s3` feature).
- Recorded both scope exclusions with magnitude and owner: the `target/` `utoipa-swagger-ui`
  contamination (1 of 62,953 counted lines, immaterial) and `minio.rs`'s absence from the
  denominator — both flagged for VERIFY-05/PIPE-02, neither fixed here per D-02.
- Recorded the ratchet arithmetic (measured − 84.00 = 1.56 points) and the explicit decision that
  the 1.56-point delta does not meet ADR-0006's 2-point ratchet trigger, and that any future raise
  is a milestone-close action, not Phase 3's.

## Task Commits

Each task was committed atomically:

1. **Task 1: Reproduce the ADR-0006 coverage pipeline end-to-end and record the entry measurement** - `b17b5aa` (docs)
2. **Task 2: Re-derive the zero-coverage set, record the QUAL-02 staleness table, scope exclusions and ratchet arithmetic** - `792818e` (docs)

**Plan metadata:** commit pending (this SUMMARY + STATE/ROADMAP/REQUIREMENTS update)

## Files Created/Modified

- `.planning/phases/03-verification-depth/03-coverage-measurement.md` - New file: Phase 3's entry
  coverage measurement record, matching the Phase 1 provenance standard, plus the re-derived
  zero-coverage set, QUAL-02 staleness table, scope exclusions, and ratchet readiness.

## Decisions Made

- **The measured figure (85.56%) is the run's own, never carried over.** Every number in the record
  is transcribed byte-identical from this run's own `llvm-cov report` stdout at HEAD
  `bb35554dd910a143725b6f149a347959130f9456` — no figure from Phase 1's 84.79% or ADR-0006 was
  reused arithmetically.
- **The zero-coverage set is unchanged from Phase 1's D-04.** All five files remain at exactly
  0.00% line coverage 98 commits and one full gap-closure phase later; this is recorded as a fact,
  not explained away.
- **A column-selection footnote, not a re-decision.** CONTEXT.md D-04's prose cites each
  zero-coverage file's `llvm-cov` **Regions** column value (e.g. "361 counted lines" for
  `redis.rs`) where the plan's task instructions ask specifically for the **Lines** column
  (350 for `redis.rs`). This plan's zero-coverage table uses the Lines-block figures throughout,
  with the discrepancy noted inline rather than silently reproducing D-04's numbers. The *set of
  five files* is identical either way — only the individual line-count figures inside the table
  differ from D-04's prose.
- **Ratchet not applied.** The measured 1.56-point delta above the 84.00% floor does not meet
  ADR-0006's 2-point trigger threshold, and even if it did, applying the ratchet is explicitly
  scoped to a milestone-close action by ADR-0006's own text. Phase 3 does not amend ADR-0006.

## Deviations from Plan

None — plan executed exactly as written. Both `<action>` blocks were followed verbatim: the
pipeline command, `--ignore-filename-regex`, doctest exclusion, and default-feature scope are all
byte-identical to ADR-0006's recorded command (D-02); the object-discovery count (31) was recorded
exactly as produced, not adjusted toward the source-file count (Research Pitfall 3); and the
zero-coverage/staleness/ratchet sections were built only from this run's own per-file rows.

## Issues Encountered

- The instrumented `cargo test --workspace --offline` run was a cold build (the `-C
  instrument-coverage` RUSTFLAGS changes the compilation fingerprint, invalidating the existing
  `target/debug` cache) and was launched with `run_in_background: true`, then polled to completion
  in a loop within the same turn per this session's explicit no-abandon instruction. It completed
  in well under the available time budget — no partial or lost run.
- One minor internal inconsistency was found and corrected during Task 2 authoring: this plan's
  own read-first source (`03-CONTEXT.md`/`03-RESEARCH.md` D-04) cites the `llvm-cov` **Regions**
  column where the plan's own task action text asks for the **Lines** column. Resolved by following
  the task's explicit column instruction and footnoting the discrepancy rather than silently
  reproducing the inconsistent prose figures — see Decisions Made above. Not a Rule 1-4 deviation
  (no code changed, no scope changed) — a data-fidelity correction inside the record itself.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The coverage floor is proven to still hold at Phase 3 HEAD — every later Phase 3 plan (03-02
  through 03-07) that adds tests can measure against this same 85.56%/84.00% baseline without
  re-running the full workspace instrumented pipeline from scratch.
- The re-derived zero-coverage set (5 files) is now the authoritative QUAL-02 target list for any
  plan closing zero-coverage gaps in this phase (per CONTEXT.md D-05, `redis.rs` is the named
  target of a later plan in this phase).
- `minio.rs`'s absence from the denominator and the `target/` contamination row are both recorded
  with owner VERIFY-05/PIPE-02 — no action needed from later Phase 3 plans, but the record exists
  for those phases to pick up directly.
- No blockers. `.planning/STATE.md` and `.planning/config.json` carry pre-existing uncommitted
  changes from the orchestrator's state-load step for this session, unrelated to this plan's Rust
  source or coverage work — noted in the record's `git status --porcelain` transcription rather
  than silently ignored.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

- FOUND: `.planning/phases/03-verification-depth/03-coverage-measurement.md`
- FOUND: `.planning/phases/03-verification-depth/03-01-SUMMARY.md`
- FOUND commit: `b17b5aa`
- FOUND commit: `792818e`
