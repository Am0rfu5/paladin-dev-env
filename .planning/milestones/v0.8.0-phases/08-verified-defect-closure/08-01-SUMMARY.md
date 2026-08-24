---
phase: 08-verified-defect-closure
plan: 01
subsystem: infra
tags: [rust, cargo-workspace, adr-0016, token-usage, pub-use, re-export]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: ADR-0016 (port value-type ownership), naming the canonical `TokenUsage` site and Phase 8/DEBT-05 as executor
provides:
  - One `pub struct TokenUsage` in the tree, at `crates/paladin-core/src/platform/container/token_usage.rs`
  - `battalion::TokenUsage` and `llm_analysis_service::TokenUsage` as `pub use` re-exports of the canonical type
  - Canonical type gains `Default`, `PartialEq`, `new()`, `from_total()` (previously only on the battalion duplicate)
affects: [phase-15-pipe-04, phase-10-hard-07, phase-16-docs-03]

# Tech tracking
tech-stack:
  added: []
  patterns: ["pub use re-export for canonical cross-crate/intra-crate value types (ADR-0016)"]

key-files:
  created: []
  modified:
    - crates/paladin-core/src/platform/container/token_usage.rs
    - crates/paladin-core/src/platform/container/battalion/mod.rs
    - crates/paladin-llm/src/llm_analysis_service.rs

key-decisions:
  - "Extend the canonical type before collapsing duplicates (D-17): Task 1 landed Default/PartialEq/new()/from_total() on the canonical type while all three struct definitions still existed, so the 11 battalion call sites and the checkpoint's irreversible step never had a window where a call site could break."
  - "Checkpoint (gate=\"blocking\", decision between option-a/option-b) was auto-selected as option-a by the orchestrator under auto-mode, NOT a human confirmation. Rationale recorded by the orchestrator: ADR-0016 is Accepted with Code Conformance: must change and names Phase 8/DEBT-05 as executor, so option-a executes a landed decision rather than making a new one; option-b would leave a top-of-precedence ADR unexecuted (violates D-00b). The phase's close-out plan (08-09) carries the human review gate for this collapse."

requirements-completed: [DEBT-05]

coverage:
  - id: D1
    description: "Exactly one `pub struct TokenUsage` definition survives in the tree, at the ADR-0016 canonical path"
    requirement: "DEBT-05"
    verification:
      - kind: other
        ref: "grep -rn 'pub struct TokenUsage' crates src | wc -l"
        status: pass
    human_judgment: false
  - id: D2
    description: "Both former duplicate import paths (`battalion::TokenUsage`, `llm_analysis_service::TokenUsage`) still resolve via `pub use`, with zero call-site edits, and the canonical type carries the inherited `new()`/`from_total()`/`Default`/`PartialEq`"
    requirement: "DEBT-05"
    verification:
      - kind: unit
        ref: "cargo test --offline --workspace --lib"
        status: pass
      - kind: integration
        ref: "cargo build --offline --workspace --all-targets"
        status: pass
    human_judgment: false

# Metrics
duration: 55min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 01: TokenUsage Consolidation Summary

**Collapsed three shipping `TokenUsage` struct definitions into one canonical `paladin-core` type, with both duplicate crates converted to `pub use` re-exports per ADR-0016/DEBT-05 — zero call-site edits across ~182 references.**

## Performance

- **Duration:** 55 min (baseline test run through final commit; checkpoint pause not counted in wall-clock task time)
- **Started:** 2026-08-06 (Task 1) — resumed 2026-08-07 after checkpoint resolution
- **Completed:** 2026-08-07
- **Tasks:** 2 (plus 1 blocking checkpoint between them)
- **Files modified:** 3

## Accomplishments
- Extended the canonical `TokenUsage` (`crates/paladin-core/src/platform/container/token_usage.rs`) with `Default`, `PartialEq` derives and `new()`/`from_total()` inherent constructors, copied verbatim from the richer battalion duplicate, plus a `#[cfg(test)] mod tests` covering all four required behaviours — landed while all three struct definitions still existed (D-17 sequencing).
- Collapsed `crates/paladin-core/src/platform/container/battalion/mod.rs:497` (struct + impl, 33 lines) into a single intra-crate `pub use crate::platform::container::token_usage::TokenUsage;` line, copying the shape at `herald.rs:28`.
- Collapsed `crates/paladin-llm/src/llm_analysis_service.rs:51` (struct, 6 lines) into a single cross-crate `pub use paladin_core::platform::container::token_usage::TokenUsage;` line, copying the shape at `crates/paladin-ports/src/output/llm_port.rs:671`. No manifest change — `crates/paladin-llm/Cargo.toml:27` already declared the `paladin-core` dependency.

## Task Commits

Each task was committed atomically (`--no-verify`, per `workflow.worktree_skip_hooks=true` — the fmt/clippy gate was still run explicitly before each commit, evidence below):

1. **Task 1: Extend the canonical TokenUsage before anything is collapsed** - `424f649` (feat)
2. **Task 2: Collapse both duplicates into pub use re-exports and prove the tree still resolves** - `e8051e1` (feat)

**Checkpoint:** blocking `checkpoint:decision` between Task 1 and Task 2, auto-resolved `option-a` by the orchestrator (see Decisions Made below).

_No plan-metadata commit is created by this executor — worktree mode; the orchestrator's wave-merge owns the shared-artifact commit._

## Files Created/Modified
- `crates/paladin-core/src/platform/container/token_usage.rs` — canonical `TokenUsage`; derive list extended to `Debug, Clone, Default, PartialEq, Serialize, Deserialize`; added `new()`, `from_total()`, and a 4-test `#[cfg(test)] mod tests`.
- `crates/paladin-core/src/platform/container/battalion/mod.rs` — struct + impl block (`:492-524`) replaced with `pub use crate::platform::container::token_usage::TokenUsage;` and a two-line `//` comment (no `///` doc attached to the `pub use`, per the plan's `cargo doc -D warnings` caution).
- `crates/paladin-llm/src/llm_analysis_service.rs` — struct (`:50-55`) replaced with `pub use paladin_core::platform::container::token_usage::TokenUsage;` and a one-line `//` comment.

## Decisions Made

**Checkpoint resolution (auto-mode, NOT human confirmation):** The plan's `checkpoint:decision` (gate="blocking") — "Collapse the two duplicate `TokenUsage` bodies into `pub use` re-exports of the canonical `paladin-core` definition" — was resolved by the orchestrator as **option-a** (proceed) under auto-mode, not by a human reviewer. Orchestrator's recorded rationale: ADR-0016 is `Accepted` with `Code Conformance: must change` and names Phase 8/DEBT-05 as its executor, so option-a executes a landed decision rather than making a new one; option-b (halt, leave duplicates in place) would leave a top-of-precedence ADR unexecuted, violating precedence rule D-00b. The phase's close-out plan (08-09) carries the human review gate for this collapse — this SUMMARY records the auto-selection explicitly so that review has a clear record of what was auto-approved and why.

**D-17 sequencing (extend-then-collapse):** Task 1 was executed as purely additive — the canonical type gained its new derives/methods while `grep -rn 'pub struct TokenUsage' crates src | wc -l` still returned `3` — so that the 11 battalion call sites at `:1135,1143,1151,1159,1172,1173,1201,1209,1210,1211,1267` and the two Herald call sites (`table_herald.rs:403`, `markdown_herald.rs:510,514`) never had a window where `::new()`/`::from_total()` could resolve to a type lacking those methods.

## Deviations from Plan

None — plan executed exactly as written, including the mandated D-17 sequencing and the two named `pub use` precedents.

## Evidence (D-00e / D-21 — verbatim command outputs)

**Pre-change baseline**, `cargo test --offline --workspace --lib` (run before any edit):
```
418 passed (paladin-ai — root binary crate)
366 passed (paladin-ai-core)
219 passed (paladin-battalion)
96 passed  (paladin-content)
0 passed   (doc-examples)
70 passed  (paladin-herald)
78 passed  (paladin-llm)
76 passed  (paladin-memory)
0 passed   (paladin-notifications, no lib tests)
98 passed  (paladin-ports)
32 passed  (paladin-storage)
117 passed (paladin-web)
= 1570 passed, 0 failed, 0 ignored
```

**Task 1 verification:**
- `grep -n 'derive' crates/paladin-core/src/platform/container/token_usage.rs` → `12:#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]`
- `grep -c 'pub fn new\|pub fn from_total' crates/paladin-core/src/platform/container/token_usage.rs` → `2`
- `cargo test --offline -p paladin-ai-core --lib token_usage` → `test result: ok. 9 passed; 0 failed; ...` (4 new `token_usage::tests::*` + 5 pre-existing `battalion::tests::test_token_usage_*` that already exercised the type)
- `cargo build --offline --workspace` → `Finished dev profile [unoptimized + debuginfo] target(s) in 46.01s`
- `grep -rn 'pub struct TokenUsage' crates src | wc -l` → `3` (unchanged — Task 1 is purely additive)

**Task 2 verification (post-collapse):**
- `grep -rn 'pub struct TokenUsage' crates src | wc -l` → `1`, sole hit `crates/paladin-core/src/platform/container/token_usage.rs`
- `grep -c 'pub use crate::platform::container::token_usage::TokenUsage' crates/paladin-core/src/platform/container/battalion/mod.rs` → `1`
- `grep -c 'pub use paladin_core::platform::container::token_usage::TokenUsage' crates/paladin-llm/src/llm_analysis_service.rs` → `1`
- `cargo build --offline --workspace --all-targets` → `Finished dev profile [unoptimized + debuginfo] target(s) in 1m 17s`, exit 0
- `cargo test --offline --workspace --lib` (post-change):
  ```
  418 passed (paladin-ai)
  370 passed (paladin-ai-core)   [+4 vs baseline: the new token_usage tests; the 5 pre-existing
                                   battalion::tests::test_token_usage_* tests still pass unchanged,
                                   now exercising the type through the re-export]
  219 passed (paladin-battalion)
  96 passed  (paladin-content)
  0 passed   (doc-examples)
  70 passed  (paladin-herald)
  78 passed  (paladin-llm)
  76 passed  (paladin-memory)
  0 passed   (paladin-notifications)
  98 passed  (paladin-ports)
  32 passed  (paladin-storage)
  117 passed (paladin-web)
  = 1574 passed, 0 failed, 0 ignored — at/above the 1570 baseline (D-21)
  ```
- `grep -rn 'VisionTokenUsage' crates/paladin-ports/src/output/vision_port.rs | wc -l` → `2` (unchanged from pre-plan; out-of-scope type not touched)
- `cargo fmt --check` → exit 0, no output
- `cargo clippy --workspace -- -D warnings` → `Finished dev profile [unoptimized + debuginfo] target(s) in 51.20s`, exit 0, zero warnings
- `git diff --diff-filter=D --name-only HEAD~1 HEAD` (post Task 2 commit) → empty (no file deletions; both duplicates were collapsed in place, not deleted as files)

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ROADMAP criterion 5 (exactly one `pub struct TokenUsage`) is satisfied and evidenced above.
- `.github/workflows/ci.yml` untouched (`:148,:393,:792` `actions-rs/toolchain@v1` remain for Phase 15/PIPE-04).
- `VisionTokenUsage` (`vision_port.rs:34`) untouched, remains out of DEBT-05 scope.
- This plan's collapse is one-way per D-18 (published-crate type now defined in one place); the phase's close-out plan (08-09) should carry the human review this auto-approved checkpoint deferred.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*
