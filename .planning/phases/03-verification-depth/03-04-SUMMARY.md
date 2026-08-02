---
phase: 03-verification-depth
plan: 04
subsystem: testing
tags: [rust, criterion, benchmarking, performance, jq, percentiles]

# Dependency graph
requires:
  - phase: 03-verification-depth
    provides: "01-coverage-measurement.md provenance standard (Phase 1) — the raw-command-plus-pasted-output template this plan's D-16 block follows"
provides:
  - "New 2026-08-02 baseline run in docs/src/appendix/performance-baseline.md: throughput/latency for all 5 shipped bench targets, derived P50/P95/P99 for all 39 sample.json files, memory-per-Paladin and startup time from a new recorded harness"
  - "examples/muster_baseline.rs — reusable measurement harness for memory-per-Paladin and startup time, independent of criterion"
  - "Documented, reproducible P50/P95/P99 derivation formula and jq filter against criterion's internal SavedSample schema"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Percentile derivation from criterion's internal sample.json (per-iteration time = times[i]/iters[i], nearest-rank index round((n-1)*p), no interpolation) — reusable for any future criterion-based percentile need in this workspace"
    - "Measurement harnesses that criterion cannot produce (memory, startup) live in examples/ as documented, re-runnable binaries rather than as one-off shell commands"

key-files:
  created:
    - examples/muster_baseline.rs
  modified:
    - examples/README.md
    - docs/src/appendix/performance-baseline.md

key-decisions:
  - "Ran the five bench targets strictly sequentially with no two `cargo bench` processes ever active concurrently (including during their build phase) — a build/compile step for a second target running while the first target's criterion measurement was in flight would have contaminated its CPU-timing figures. One accidental double-queue (battalion_benchmarks briefly started building alongside config_benchmarks) was caught and the second process killed by PID before it reached measurement."
  - "jq's round() (round-half-away-from-zero) is the derivation's tie-breaking rule, not a language's default banker's rounding — verified by cross-checking a Python reimplementation against the jq output on sanctum_search_scale/vector_count/100 (n=50, index 24.5 ties), which disagreed under Python's default round() until round-half-away-from-zero was implemented explicitly to match jq's behavior."
  - "Latency-percentile table cells show both a human-readable unit (matching the unit criterion itself used for that benchmark in the Results tables) and the raw nanosecond figure in parentheses, so every table value is a literal substring of the pasted jq output — satisfying the plan's traceability requirement without forcing every row into a single fixed unit spanning nanoseconds to milliseconds."
  - "The old 2026-05-27 run's flat H2 sections (Scope/Environment/Methodology/Results/...) were demoted to H3 and nested under a new '## Run — 2026-05-27 (superseded)' wrapper heading, matching the new run's own '## Run — 2026-08-02' wrapper — an organizational change only; no figure, command, or commit SHA in the old run's content was edited."

patterns-established:
  - "Any future baseline-document amendment adds a new '## Run — <date>' section above the prior one and demotes nothing it doesn't own; only the immediately-preceding run gets its own superseded callout added."

requirements-completed: [QUAL-05]

coverage:
  - id: D1
    description: "All five shipped bench targets (config_benchmarks, battalion_benchmarks, sanctum_benchmarks, garrison_benchmarks, llm_serialization_benchmarks) ran to completion under cargo bench --offline, sequentially, with raw criterion stdout pasted into the baseline document"
    requirement: "QUAL-05"
    verification:
      - kind: other
        ref: "docs/src/appendix/performance-baseline.md ## Run — 2026-08-02 → ### Methodology / ### Results (5 pasted criterion stdout blocks, one per target)"
        status: pass
    human_judgment: false
  - id: D2
    description: "P50/P95/P99 latency percentiles derived from criterion's raw per-iteration sample.json data for all 39 samples this run produced, with the derivation formula, nearest-rank rule, and exact jq invocation documented and reproducible"
    requirement: "QUAL-05"
    verification:
      - kind: other
        ref: "docs/src/appendix/performance-baseline.md ### P50 / P95 / P99 Derivation and ### Latency percentiles (39-row table, every cell traceable to the pasted jq output)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Memory-per-Paladin and startup time measured by a new purpose-built harness (examples/muster_baseline.rs), independent of criterion, and recorded in the baseline document with source and scope named"
    requirement: "QUAL-05"
    verification:
      - kind: other
        ref: "examples/muster_baseline.rs; docs/src/appendix/performance-baseline.md ### Memory-per-Paladin and ### Startup Time"
        status: pass
    human_judgment: false

duration: 35min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 4: Performance Baseline Summary

**Ran all five shipped criterion bench targets offline, derived P50/P95/P99 from their raw per-iteration samples via `jq`, and measured memory-per-Paladin (479 bytes) and startup time with a new `examples/muster_baseline.rs` harness — recorded as a new dated run in the existing baseline document, with the 2026-05-27 run retained in place as superseded.**

## Performance

- **Duration:** ~35 min (dominated by cold `bench`-profile builds; five sequential `cargo bench --offline` builds/runs totaling ~14 minutes of wall-clock build+measure time, plus tooling/derivation work)
- **Started:** 2026-08-02T15:55:18Z
- **Completed:** 2026-08-02T16:30:09Z
- **Tasks:** 3
- **Files modified:** 3 (1 created: `examples/muster_baseline.rs`)

## Accomplishments
- All five shipped bench targets (`config_benchmarks`, `battalion_benchmarks`, `sanctum_benchmarks`, `garrison_benchmarks`, `llm_serialization_benchmarks`) ran to completion under `cargo bench --offline`, each with full D-16 provenance and raw criterion stdout pasted verbatim into `docs/src/appendix/performance-baseline.md`
- `examples/muster_baseline.rs` — a new, documented, re-runnable harness that measures memory-per-Paladin (via `/proc/self/status` `VmRSS` delta across 1000 constructed `Paladin` aggregates) and startup time (in-process to first Paladin, plus whole-process wall clock), since criterion produces neither metric
- P50/P95/P99 derived from all 39 `target/criterion/*/new/sample.json` files this run produced, using criterion's internal `SavedSample { iters, times }` schema, the `times[i]/iters[i]` per-iteration transform, and nearest-rank selection (`round((n-1)*p)`, ties by sorted position, no interpolation) — documented with the exact `jq` filter and its full raw output
- The 2026-05-27 run's original figures, commit SHA and commands survive unedited under an explicit `> Superseded by the 2026-08-02 run above.` callout; no delta, ratio, or percentage change is computed between the two runs anywhere in the document
- The two bench suites QUAL-05 names that do not exist in the tree (Paladin execution loop, Arsenal invocation) are recorded under `### Not produced by this run` as deferred with reason, not fabricated or silently dropped

## Task Commits

Each task was committed atomically:

1. **Task 1: Run the five shipped bench targets offline and capture raw criterion output with full provenance** - `8022fd6` (docs)
2. **Task 2: Build and run the muster harness for memory-per-Paladin and startup time** - `a6c4c14` (feat)
3. **Task 3: Derive P50/P95/P99 from criterion's raw samples and record the derivation** - `adb9972` (docs)

## Files Created/Modified
- `examples/muster_baseline.rs` - New measurement harness: constructs 1000 `Paladin`s via the shared `PaladinBuilder` path, reads `/proc/self/status` before/after to derive `bytes_per_paladin`, times elapsed to first-Paladin-constructed. No `unwrap()`/`expect()`/`panic!`; `main` returns `Result`
- `examples/README.md` - Added a new "Performance Benchmarking Examples" section listing `muster_baseline.rs`
- `docs/src/appendix/performance-baseline.md` - Added `## Run — 2026-08-02` (Scope, Environment, Methodology, Results ×5 targets, Memory-per-Paladin, Startup Time, P50/P95/P99 Derivation, Latency percentiles, Not produced by this run); demoted the prior flat sections under a new `## Run — 2026-05-27 (superseded)` wrapper heading with an explicit superseded callout, figures unchanged

## Decisions Made
- Sequential-only bench execution to avoid CPU-timing contamination between targets (see `key-decisions` in frontmatter for the near-miss and recovery)
- jq's round-half-away-from-zero tie-breaking rule adopted explicitly over a language's default banker's rounding, verified against a real tie case (n=50, index 24.5)
- Percentile table cells carry both a human unit and the raw ns figure for full traceability to the pasted jq output
- Old run's sections demoted to H3 under a new wrapper heading (organizational only, no figures touched) to give both runs a matching, unambiguous structure

## Deviations from Plan

None - plan executed exactly as written. Task 1's read-only `<precondition>` (criterion 0.5.1 present in the local cargo registry) was verified true before any bench command ran.

## Issues Encountered
- A background `battalion_benchmarks` bench invocation was accidentally queued while `config_benchmarks` was still mid-compile; caught before it reached the measurement phase and killed by exact PID (a `pkill -f` attempt first self-matched and killed its own invoking shell — recovered by targeting the PID directly instead). No measurement was contaminated; `battalion_benchmarks` was re-run cleanly afterward, in isolation.
- Two rounding-library discrepancies were caught before they reached the document: (1) Python's default `round()` disagreed with jq's `round()` on a tied index (n=50, `(n-1)*0.50 = 24.5`), requiring an explicit round-half-away-from-zero reimplementation to match jq's authoritative output exactly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `docs/src/appendix/performance-baseline.md` now carries a complete, traceable, host-scoped performance baseline (throughput, latency percentiles, memory-per-Paladin, startup time) that Phase 15 (PIPE) or any future performance-sensitive plan can diff against on this same host
- The two deferred bench suites (Paladin execution loop, Arsenal invocation) remain unowned; no phase in the current roadmap claims them
- QUAL-05 is satisfied by this plan's deliverables; sibling plans 03-05/03-06/03-07/03-08 (if any share QUAL-02/QUAL-04) are unaffected — this plan touched only `examples/` and `docs/`, no product code or `Cargo.toml`

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

- FOUND: examples/muster_baseline.rs
- FOUND: docs/src/appendix/performance-baseline.md
- FOUND: .planning/phases/03-verification-depth/03-04-SUMMARY.md
- FOUND: 8022fd6 (Task 1 commit)
- FOUND: a6c4c14 (Task 2 commit)
- FOUND: adb9972 (Task 3 commit)
