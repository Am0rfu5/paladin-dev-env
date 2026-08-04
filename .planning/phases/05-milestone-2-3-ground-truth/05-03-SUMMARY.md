---
phase: 05-milestone-2-3-ground-truth
plan: 03
subsystem: docs
tags: [adr, vision, encryption, live-api-tests, decision-record]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: 05-CONTEXT.md D-09/D-10/D-11/D-17/D-18/D-19 dispositions, 05-RESEARCH.md Pitfall 2 citation-precision correction
provides:
  - ".planning/decisions/0011-vision-port-surfaces.md — coexistence + entry-point guidance + encryption-at-rest disposition, must change, executed by Phase 6 CLOSE-03"
  - ".planning/decisions/0012-live-api-test-key-behaviour.md — shipped panic stands, double-gate justification, must change (doc comment only), executed by Phase 6 CLOSE-03"
affects: [05-08-vision-ledger-plan, 05-07-live-api-ledger-plan, phase-6-close-03]

# Tech tracking
tech-stack:
  added: []
  patterns: [seven-H2 ADR shape (Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers), must-change conformance naming a downstream executing requirement]

key-files:
  created:
    - .planning/decisions/0011-vision-port-surfaces.md
    - .planning/decisions/0012-live-api-test-key-behaviour.md
  modified: []

key-decisions:
  - "ADR-0011: both vision port surfaces (VisionPort, VisionCapableLlm) coexist long-term; no migration planned. VisionPort is the recommended application-code entry point (execute_with_vision); VisionCapableLlm is the adapter-author surface (enable_vision)."
  - "ADR-0011: the encryption-at-rest capability (EncryptionService, DataRetentionPolicy, VisionError::EncryptionError) is recorded as built, self-tested, and never wired — a third verdict, not shipped and not dropped. Zero-consumer grep re-run for this task returned no output."
  - "ADR-0012: the shipped require_api_key panic stands; not changed to skip. Justified by the double gate (feature flag + 13 #[ignore] attributes) supplying the graceful skip both PRDs require in the default run, not by overruling either PRD position."
  - "ADR-0012: require_api_key treats exactly two conditions as missing — env var absent, or present-but-empty (str::is_empty(), no trimming). A whitespace-only value is treated as present and returned without panicking."
  - "Both ADRs: Code Conformance = must change, executed by Phase 6 CLOSE-03. Neither .rs file, Cargo.toml, nor .github/workflows/ touched by this plan."

patterns-established: []

requirements-completed: [VERIFY-04, VERIFY-06]

coverage:
  - id: D1
    description: "ADR-0011 records both vision surfaces as coexisting with explicit entry-point guidance and the encryption-at-rest capability as built/self-tested/unwired with a freshly re-derived zero-consumer grep result"
    requirement: "VERIFY-04"
    verification:
      - kind: manual_procedural
        ref: "grep -c '^## ' .planning/decisions/0011-vision-port-surfaces.md == 7; grep -rln \"EncryptionService|DataRetentionPolicy|VisionError::EncryptionError\" src/ crates/ | grep -v infrastructure/security returns empty"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0012 records the shipped require_api_key panic as standing, names both missing-key arms and the exact emptiness semantics, and identifies the doc-comment defect at the correctly re-derived line"
    requirement: "VERIFY-06"
    verification:
      - kind: manual_procedural
        ref: "grep -c '^## ' .planning/decisions/0012-live-api-test-key-behaviour.md == 7; grep -c '#\\[ignore\\]' tests/integration/llm_live_api_tests.rs == 13; git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/' empty"
        status: pass
    human_judgment: false

# Metrics
duration: ~30min active work (plus a long cold-cache `cargo clippy --workspace` pre-commit gate the orchestrator has since exempted this worktree from via `workflow.worktree_skip_hooks=true`)
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 03: ADR-0011 and ADR-0012 — vision surfaces and live-API-test key behaviour Summary

**Two new `must change` ADRs give VERIFY-04 and VERIFY-06 exactly one recorded answer each: both vision port surfaces ship deliberately at different layers with the encryption-at-rest capability recorded as built-and-unwired (not dropped), and the shipped `require_api_key` panic stands because the double gate already supplies the graceful skip both PRDs required.**

## Performance

- **Duration:** ~30 min of active authoring/verification; the single commit was additionally gated by a cold-cache `cargo clippy --workspace --all-targets --all-features -D warnings` pre-commit run (`.pre-commit-config.yaml`'s `always_run: true`), which the orchestrator identified mid-execution as unnecessary for this worktree given `workflow.worktree_skip_hooks=true` in `.planning/config.json` — confirmed present by direct inspection
- **Started:** 2026-08-04 (session start)
- **Completed:** 2026-08-04T17:05:08Z (commit timestamp)
- **Tasks:** 2/2 completed
- **Files modified:** 2 created

## Accomplishments
- `.planning/decisions/0011-vision-port-surfaces.md` — records `VisionPort` (`crates/paladin-ports/src/output/vision_port.rs:47`) as the recommended application-code entry point via `PaladinExecutionService::execute_with_vision` and `VisionCapableLlm` (`crates/paladin-ports/src/output/vision_llm_port.rs:52`) as the adapter-author surface via `PaladinBuilder::enable_vision`; records the encryption-at-rest capability as built, self-tested, and never wired
- `.planning/decisions/0012-live-api-test-key-behaviour.md` — records the shipped `require_api_key` panic as standing, the double gate (`tests/integration/mod.rs:34-35` + 13 `#[ignore]` attributes, compiled via `tests/lib.rs:61`) as the justification, and the exact missing-key semantics read from the shipped body
- Corrected a citation-precision issue this task's own read_first flagged: `SecureData::is_expired` (`encryption.rs:95`) and `DataRetentionPolicy::should_retain` (`encryption.rs:131`) cited as two distinct methods on two distinct types, per 05-RESEARCH.md Pitfall 2
- Identified and recorded a further line-number drift beyond Pitfall 2's scope: CONTEXT.md's D-18 and this plan's own task text cite the doc-comment defect at `tests/integration/llm_live_api_tests.rs:63`; direct re-reading shows the specific lying sentence ("Skip test if API key is not present or empty") is actually at line 61, with line 63 continuing the same four-line doc-comment block

## Task Commits

Both tasks' output was committed in a single commit per the plan's explicit instruction ("Commit both of this plan's ADRs in a single commit at the end"):

1. **Task 1: Author ADR-0011** — vision port surfaces and the encryption-at-rest disposition
2. **Task 2: Author ADR-0012** — live-API-test missing-key behaviour

**Commit:** `06af99a` — `docs(05-03): author ADR-0011 and ADR-0012 for VERIFY-04 and VERIFY-06`

_No separate plan-metadata commit for STATE.md/ROADMAP.md — this plan runs in worktree isolation; the orchestrator owns those shared-file writes after the wave merges._

## Files Created/Modified
- `.planning/decisions/0011-vision-port-surfaces.md` — new ADR, seven H2 headings, `must change`, executed by Phase 6 CLOSE-03
- `.planning/decisions/0012-live-api-test-key-behaviour.md` — new ADR, seven H2 headings, `must change` (doc comment only), executed by Phase 6 CLOSE-03

## Decisions Made

**Zero-consumer grep result (re-derived, not transcribed), for ADR-0011:**
```
grep -rln "EncryptionService\|DataRetentionPolicy\|VisionError::EncryptionError" src/ crates/ | grep -v "infrastructure/security"
```
returned **no output** — confirming zero consumers of the encryption capability outside `src/infrastructure/security/`.

**Measured `#[ignore]` count and gate lines, for ADR-0012:**
- `grep -c '#\[ignore\]' tests/integration/llm_live_api_tests.rs` → **13** (matches 05-RESEARCH.md's recorded figure, no drift)
- `grep -n 'cfg(feature = "live-api-tests")' tests/integration/mod.rs` → line **34** (matches, no drift)
- `tests/lib.rs:61` (`pub mod integration;`) and `Cargo.toml:265` (`live-api-tests = []`) both confirmed unchanged

**`require_api_key`'s exact emptiness check, read from the shipped body:**
`Ok(key) if !key.is_empty() => key` is the only non-panicking arm. `Ok(_)` (present, empty string) panics; `Err(_)` (absent) panics. The check is `str::is_empty()` — a byte-length check with no trimming — so a whitespace-only value (e.g. `" "`) is non-empty by this check and is treated as present, returned as-is without panicking or further validation.

**Line-number drift recorded (beyond 05-RESEARCH.md Pitfall 2's scope):** the doc-comment defect CONTEXT.md's D-18 and this plan's task text cite at `tests/integration/llm_live_api_tests.rs:63` is, on direct re-reading, actually at line 61 (`/// Skip test if API key is not present or empty, otherwise return the key`). Line 63 is part of the same four-line doc-comment block (61-64) but is itself an accurate description of the panic behaviour, not the lying sentence. ADR-0012 records this precisely and still cites `:63` (satisfying the plan's acceptance criterion, which anchors on the same doc-comment block) while stating the correct line for Phase 6 CLOSE-03 to actually edit.

## Deviations from Plan

None — plan executed exactly as written. The one process deviation was operational, not substantive: the plan's Task 2 action itself anticipated the slow pre-commit gate ("a Bash timeout of at least 300000 ms — `.pre-commit-config.yaml` sets `always_run: true` on `cargo-fmt` and `cargo-clippy`"). Even 300000ms was insufficient on this worktree's cold `target/` cache; the commit was run as a background task and completed successfully once given enough wall-clock time, exactly as instructed (`--no-verify` was NOT used for this commit — full hooks ran and passed). The orchestrator subsequently authorized `--no-verify` for any further commits in this plan via the project's own `workflow.worktree_skip_hooks=true` config setting (confirmed present in `.planning/config.json`), but no further commits were needed since both tasks' output was already committed together per the plan's own instruction.

## Issues Encountered

**Pre-commit hook duration on cold worktree cache.** The single commit (both ADRs, markdown-only) triggered `.pre-commit-config.yaml`'s `always_run: true` `cargo fmt`/`cargo clippy --workspace --all-targets --all-features -D warnings` gates, which compile the entire workspace. On this worktree's cold `target/` (186M, freshly created), this took well over 5 minutes. Resolved by running the commit as a background task and waiting for completion rather than treating the timeout as a failure — the commit ultimately succeeded with all hooks passing (`06af99a`).

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- ADR-0011 and ADR-0012 are both live, `must change`, and name Phase 6 CLOSE-03 as the sole executing requirement for their code consequences (rustdoc entry-point guidance + encryption wiring decision; doc-comment correction, respectively). Neither requires further planning work.
- Ledger plan 05-08 can now cite ADR-0011 to rewrite the `REQ-vision-security-encryption` row from "Not found in tree" to `present, unproven`.
- Ledger plan 05-07 can now cite ADR-0012 on the `REQ-provider-live-api-tests` and `REQ-cli-tiered-environment-testing` rows.
- The `unresolved` flagged assumption from this plan's `<flagged_assumptions>` (whether Phase 5's job is to record a disposition vs. determine an encryption-at-rest security *requirement level*) is unchanged by this plan — it remains flagged for `/gsd-verify-work` per the plan's own instruction, not resolved here.

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
