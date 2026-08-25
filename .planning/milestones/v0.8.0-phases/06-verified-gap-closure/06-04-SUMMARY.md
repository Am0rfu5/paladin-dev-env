---
phase: 06-verified-gap-closure
plan: 04
subsystem: testing
tags: [criterion, benchmarks, battalion, chain-of-command, performance-baseline, rust]

# Dependency graph
requires:
  - phase: 06-verified-gap-closure (06-CONTEXT.md D-12, D-13)
    provides: the "write the benchmark, then measure and record it as a separate dated run" decision pair
  - phase: 06-verified-gap-closure (06-02-SUMMARY.md)
    provides: the Herald field on ChainOfCommandExecutionService — measured tree includes it
provides:
  - benchmark_chain_of_command in battalion_benchmarks.rs, registered in criterion_group!, driving a real ChainOfCommandExecutionService with three cases (3/5/10 specialists, Broadcast strategy)
  - a new dated "## Run — 2026-08-05" section in docs/src/appendix/performance-baseline.md recording throughput and derived P50/P95/P99 for all three cases
affects: [CLOSE-02 ledger, Epic 24 cluster 1.0 block verdict]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Multi-case criterion benchmark fn: one fn builds N independent ChainOfCommand fixtures (one per specialist-count case) and registers N c.bench_function calls, modelled on benchmark_campaign_branching_dag's multi-node-construction-before-bench_function shape"

key-files:
  created: []
  modified:
    - crates/paladin-battalion/benches/battalion_benchmarks.rs
    - docs/src/appendix/performance-baseline.md

key-decisions:
  - "Task 2's precondition (exclusive machine use) was genuinely unmet on first attempt — a sibling wave-2 executor (06-05, later identified) was running cargo test concurrently. Per executor protocol this is never auto-approved: the plan halted with a checkpoint:human-verify rather than recording a contended figure, and only resumed after the orchestrator confirmed all sibling executors had returned and this agent independently re-verified via pgrep and /proc/loadavg."
  - "The new run section was inserted at the top of performance-baseline.md, above the existing '## Run — 2026-08-02' section, preserving newest-first ordering (matching how 2026-08-02 already sits above the superseded 2026-05-27 run) while leaving the 2026-08-02 section's diff purely additive."
  - "The cargo bench invocation for Task 2 was filtered to `-- chain_of_command` rather than running the whole battalion_benchmarks target unfiltered, per the plan's own Scope requirement that this run 'is not a whole-suite re-measurement' — only the three new criterion ids were exercised and recorded."

requirements-completed: [CLOSE-02]

coverage:
  - id: D1
    description: "benchmark_chain_of_command registers three criterion cases (2_levels_3_subordinates, 2_levels_5_subordinates, wide_10_subordinates) driving a real ChainOfCommandExecutionService, making battalion-benchmarks.md's 'Compiling and enabled' claim true"
    requirement: "CLOSE-02"
    verification:
      - kind: other
        ref: "cargo bench --no-run -p paladin-battalion (exit 0) + cargo fmt --check + cargo clippy --workspace --all-targets -- -D warnings (both exit 0)"
        status: pass
      - kind: other
        ref: "cargo bench -p paladin-battalion --bench battalion_benchmarks -- chain_of_command --sample-size 10 (trial run, all 3 cases executed and reported timings)"
        status: pass
    human_judgment: false
  - id: D2
    description: "docs/src/appendix/performance-baseline.md carries a new dated 2026-08-05 run section with throughput and derived P50/P95/P99 for the three new ChainOfCommand cases, produced by the document's own verbatim jq derivation, additive-only against the 2026-08-02 section"
    requirement: "CLOSE-02"
    verification:
      - kind: other
        ref: "cargo bench -p paladin-battalion --bench battalion_benchmarks -- chain_of_command --noplot (full 100-sample run, uncontended)"
        status: pass
      - kind: other
        ref: "git diff docs/src/appendix/performance-baseline.md shows zero removed/changed lines inside the 2026-08-02 section (additive-only diff, verified by grep -c '^## Run — ' == 3 and manual inspection)"
        status: pass
    human_judgment: false

# Metrics
duration: ~80min (including one precondition-halt/resume cycle)
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 04: ChainOfCommand Benchmark and Baseline Summary

**`benchmark_chain_of_command` added to `battalion_benchmarks.rs` with three criterion cases (3/5/10 specialists) driving a real `ChainOfCommandExecutionService`, and a new dated 2026-08-05 run recorded in `performance-baseline.md` with throughput and derived P50/P95/P99 for all three, uncontended and additive-only against the 2026-08-02 baseline.**

## Performance

- **Duration:** ~80 min total (Task 1 ~35 min; Task 2 halted on an unmet precondition, resumed after orchestrator-confirmed exclusive machine access, then ~15 min to measure and write)
- **Tasks:** 2 completed
- **Files modified:** 2

## Accomplishments

- `benchmark_chain_of_command` added to `crates/paladin-battalion/benches/battalion_benchmarks.rs`, registered in the existing `criterion_group!` after `benchmark_campaign_branching_dag`. It builds three independent `ChainOfCommand` fixtures (one commander + 3, 5, and 10 specialist Paladins respectively, `DelegationStrategy::Broadcast` so every specialist runs on every iteration) and registers three criterion ids matching the three case names `docs/src/appendix/battalion-benchmarks.md` already claimed: `battalion/chain_of_command_2_levels_3_subordinates`, `battalion/chain_of_command_2_levels_5_subordinates`, `battalion/chain_of_command_wide_10_subordinates`.
- `cargo bench --no-run -p paladin-battalion`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` all exit 0. `docs/src/appendix/battalion-benchmarks.md` and `crates/paladin-battalion/Cargo.toml` were not touched, per D-12.
- A dedicated, uncontended `cargo bench -p paladin-battalion --bench battalion_benchmarks -- chain_of_command --noplot` run (100 samples per case) produced the three cases' timings, and P50/P95/P99 were derived from the resulting `sample.json` files using the document's own verbatim `jq` filter and nearest-rank formula.
- A new `## Run — 2026-08-05` section was appended to (technically: inserted at the top of, newest-first) `docs/src/appendix/performance-baseline.md`, carrying its own `### Scope`, `### Environment` (explicitly stating the measurement was uncontended and how that was verified), `### Methodology`, results block, benchmark table, and `### Latency percentiles` table — following the file's own established format. The 2026-08-02 section's diff is additive-only: zero lines removed or changed inside it.
- No cross-run delta was computed against either the 2026-08-02 or 2026-05-27 runs; the new section states in its opening sentence that none of the three runs were taken in one sitting.

## Task Commits

1. **Task 1: Write the ChainOfCommand benchmark and register it** - `d7ee3d7` (feat)
2. **Task 2: Measure the new target and append a dated baseline run** - `b703dc6` (docs)

## Files Created/Modified

- `crates/paladin-battalion/benches/battalion_benchmarks.rs` - new `benchmark_chain_of_command` fn, three criterion cases, registered in `criterion_group!`
- `docs/src/appendix/performance-baseline.md` - new `## Run — 2026-08-05` section (additive-only against the 2026-08-02 section)

## Decisions Made

- **Task 2's precondition genuinely tripped on first attempt.** Before starting Task 2, this agent's own read-only check (`ps aux`) found a sibling wave-2 executor's `cargo test -p paladin-ai --lib -- security::encryption` actively running with a live `rustc` child process. Per the executor protocol, an unmet precondition is never auto-approved — the plan halted with a `checkpoint:human-verify` (gate `blocking-human`) after Task 1's commit, rather than measuring under contention. The orchestrator subsequently confirmed all three wave-2 sibling executors (06-04, 06-05, 06-06) had returned, that `pgrep` for `cargo`/`rustc`/`docker build` returned nothing, and that load average had settled to `0.97 3.15 5.75`. This agent independently re-verified the same three checks (plus `/proc/loadavg`, which read `0.66 2.86 5.57` at time of measurement) before proceeding — both checks are recorded verbatim in the new run section's `### Environment` block for auditability.
- **New run section placement:** inserted at the very top of `performance-baseline.md`, above `## Run — 2026-08-02`, preserving the document's newest-first ordering (2026-08-02 already sits above the superseded 2026-05-27 run) while keeping the 2026-08-02 section's own diff purely additive — no line inside it was touched.
- **`cargo bench` filtered to `-- chain_of_command`** rather than run unfiltered, matching the plan's explicit Scope requirement that this is "not a whole-suite re-measurement" of `battalion_benchmarks` — only the three new criterion ids were exercised, measured, and recorded; `formation_3_agents`, `phalanx_5_agents`, and `campaign_branching_dag` were not re-run or re-recorded by this plan.

## Deviations from Plan

None — plan executed exactly as written. The precondition halt/resume was not a deviation from the plan; it is the precondition mechanism the plan itself specifies working as intended (Phase 3 plan 03-04's sequential-only benchmark lesson, applied here).

## Issues Encountered

- Task 2's `<precondition>` (no other `cargo build`/`test`/`bench`/container build running) was unmet at the start of this agent's Task 2 attempt, due to genuine concurrent activity from a sibling wave-2 executor. Resolved by halting (no partial commit), waiting for the orchestrator's confirmation of exclusive machine access, independently re-verifying, and then proceeding. No code or measurement was affected — the eventual measurement is uncontended.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Epic 24 cluster `1.0` (the missing ChainOfCommand benchmark) is closed: the benchmark exists, compiles under `cargo bench --no-run`, registers all three cases `docs/src/appendix/battalion-benchmarks.md` already claimed, and its baseline is recorded as its own honestly-dated, uncontended run. D-12 and D-13 are both satisfied.
- No blockers for the remaining 06-verified-gap-closure plans; this plan's scope (Epic 24 cluster 1.0 only) is fully closed.

## Self-Check: PASSED

- FOUND: `crates/paladin-battalion/benches/battalion_benchmarks.rs`
- FOUND: `docs/src/appendix/performance-baseline.md`
- FOUND: `.planning/phases/06-verified-gap-closure/06-04-SUMMARY.md`
- FOUND commit: `d7ee3d7`
- FOUND commit: `b703dc6`
- FOUND commit: `97dec89`

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*
