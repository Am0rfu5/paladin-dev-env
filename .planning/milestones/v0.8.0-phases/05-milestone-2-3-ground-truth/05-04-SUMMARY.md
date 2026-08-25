---
phase: 05-milestone-2-3-ground-truth
plan: 04
subsystem: planning-decisions
tags: [coverage-gate, adr-amendment, verify-05]
dependency-graph:
  requires: [RECON-07 (Phase 1, ADR-0006 original)]
  provides: [VERIFY-05]
  affects: [Phase 15 PIPE-02, Phase 5 ledger plans 05-11, 05-12]
tech-stack:
  added: []
  patterns: ["in-place ADR amendment via dated H2 section + inline markers, per milestone-01.md convention"]
key-files:
  created: []
  modified:
    - .planning/decisions/0006-coverage-gate.md
decisions:
  - "Extended ADR-0006 in place with a dated '## Phase 5 amendment (2026-08-04)' H2 inserted between the existing Decision and Considered Options sections — no second coverage ADR was created, matching D-12."
  - "Two run-2 positions (75% layered per-tier table; Epic 24's 80%/70% re-assertion) recorded as two new Considered Options bullets, appended after the existing rejected-position bullets and cross-referencing them rather than restating their arguments."
  - "Module-scoped gate table added with Herald first, autonomous components second (VERIFY-05's own text order): Herald 80.49% measured against a 95% target (~14.5pt gap); autonomous components 92.80% line-weighted aggregate against a 90% target (aggregate clears by ~2.8pt, though prompt_generation_service.rs individually sits at 88.89%, ~1.1pt below target)."
  - "src/bin/paladin-server.rs (0.00%) dispositioned deferred with reason, run()-seam extraction named as the concrete prerequisite, owner Phase 15 / PIPE-02."
  - "minio.rs recorded as outside the gated denominator by construction (s3-storage is non-default), with the feature-scoped-measurement question assigned to Phase 15 / PIPE-02."
  - "The ~78% Milestone-3 release-notes figure recorded as failing the 84% floor and predating the measurement that set it — judged, not reconciled, in the same shape ADR-0006 already gives the 60.88%/67.79% Milestone-1 baselines."
metrics:
  duration: "~35min"
  completed: 2026-08-04
status: complete
---

# Phase 5 Plan 04: Amend ADR-0006 for VERIFY-05 Summary

Extended ADR-0006's single coverage answer (84% workspace line-coverage floor, default-feature
scope) across the two run-2 positions, placed the two module-scoped gates against it with
transcribed figures, and dispositioned the two v0.7.1-inherited items — all via one in-place
amendment, never a second ADR.

## What Was Built

Both tasks amend `.planning/decisions/0006-coverage-gate.md` in place using `Edit` (never `Write`),
inside a single new `## Phase 5 amendment (2026-08-04)` H2 section inserted between the existing
`## Decision` and `## Considered Options` sections. The file's original seven H2 headings
(`Status`, `Context`, `Decision`, `Considered Options`, `Code Locations`, `Code Conformance`,
`Downstream Consumers`) are retained in their original relative order; this is the only new
heading introduced.

**Task 1 — amendment preamble, arithmetic restatement, two new Considered Options entries, and the
falsifiability statement:**
- Preamble states VERIFY-05 extends this same ADR (D-12), that superseded text is retained (D-00g),
  that no re-measurement was performed and the 84.79% figure recorded 2026-07-31 stands (D-16), and
  that Phase 15 / PIPE-02 is the enforcement owner.
- The floor arithmetic (truncation toward zero, at-or-above comparison, 84% passes / 83.99% fails)
  is restated by cross-reference only — the exact phrase `a later run measuring exactly 84%`
  appears exactly once in the file (the original), never duplicated.
- Two bullets appended to the existing `## Considered Options` list (after the existing
  rejected-position bullets, not interleaved): the Milestone 3 plan's 75%-layered-per-tier position,
  and Epic 24's ≥80%/≥70% re-assertion — both rejected against the measured 84.79% and
  cross-referencing the ADR's existing generic rejections rather than restating them.
- The ~78% Milestone-3 falsifiability statement, copying the shape of the existing "accepted and
  noted, not explained" paragraph that already treats the 60.88%/67.79% Milestone-1 baselines this
  way.

**Task 2 — module-scoped gate table and the two inherited dispositions:**
- A two-row `| Module scope | Target | Measured | Gap | Owner |` table, Herald listed first
  (matching VERIFY-05's own requirement-text order), autonomous components second.
- `src/bin/paladin-server.rs` and `minio.rs` dispositioned per D-14a/D-14b, both naming Phase 15 /
  PIPE-02 as owner.
- `## Downstream Consumers` extended with PIPE-02's new obligations (run() seam, minio.rs
  feature-scope decision, module-gate enforcement) and a pointer to ledger plans 05-11/05-12.

## Transcribed Figures and Their Source Lines

All figures below are transcribed character-for-character from
`.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-coverage-measurement.md`
(the archived Phase 1 measurement record), never re-typed from memory or re-derived:

| File | Source line | Lines | Missed | Line coverage |
|---|---|---|---|---|
| `crates/paladin-core/src/platform/container/herald.rs` | `01-coverage-measurement.md:317` | 246 | 48 | **80.49%** |
| `src/bin/paladin-server.rs` | `01-coverage-measurement.md:426` | 145 | 145 | **0.00%** |
| `src/application/services/paladin/planning_service.rs` | `01-coverage-measurement.md:421` | 577 | 43 | 92.55% |
| `src/application/services/paladin/prompt_generation_service.rs` | `01-coverage-measurement.md:422` | 234 | 26 | 88.89% |
| `src/application/services/paladin/temperature_service.rs` | `01-coverage-measurement.md:423` | 394 | 23 | 94.16% |
| `src/application/services/paladin/handoff_service.rs` | `01-coverage-measurement.md:418` | 239 | 12 | 94.98% |

**Autonomous-component measurement: found, not absent.** All four per-file rows the plan named
(`planning_service.rs`, `prompt_generation_service.rs`, `temperature_service.rs`,
`handoff_service.rs`) exist in `01-coverage-measurement.md`, so the "no per-file row" empty-edge
case (Task 2's fallback instruction) did not trigger. The derived scope is the line-weighted
aggregate across all four files: `(577+234+394+239 − 43−26−23−12) / (577+234+394+239) =
(1444 − 104) / 1444 = 92.80%`, computed directly from the transcribed Lines/Missed columns — not
interpolated or guessed. The aggregate clears the 90% target by ~2.8 points, though one file
individually (`prompt_generation_service.rs`, 88.89%) sits below the target; both facts are stated
in the table entry.

## No Figure Disagreed With the Measurement File

Every transcribed figure (Herald 80.49%, `paladin-server.rs` 0.00%, and the four autonomous-service
line-coverage figures) matched the source `01-coverage-measurement.md` row exactly on re-read after
writing — no discrepancy was found between what ADR-0006's stub language already asserted
(Herald's 80.49% was already present at the pre-amendment stub, `:139`) and what this plan
transcribed.

## Verification

All automated checks from both tasks' `<verify>` blocks and the plan-level `<verification>` block
were run and passed after the edits:

- `ls .planning/decisions/ | grep -c 'coverage'` → `1` (still the only coverage ADR)
- `grep -c '^## Phase 5 amendment (2026-08-04)'` → `1`
- `grep -c 'Amended by Phase 5, dated 2026-08-04'` → `1`
- `grep -c 'a later run measuring exactly 84%'` → `1` (original arithmetic phrase not duplicated)
- `grep -c '80.49'` → `3` (three legitimate references: existing stub, new table row, Downstream
  Consumers pointer)
- `grep -c '| Module scope | Target | Measured | Gap | Owner |'` → `1`, Herald row precedes
  autonomous-components row (line 200 vs 201)
- `grep -c '95'` → `4`, `grep -c '90'` → `3` (both module targets stated, neither withdrawn)
- `paladin-server.rs` entry contains `0.00`, `deferred with reason`, `run()`, `PIPE-02`
- `minio.rs` entry contains `s3-storage`, `PIPE-02`
- `sed -n '/^## Downstream Consumers/,$p' | grep -c 'PIPE-02'` → `3`
- `git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/'` → empty (no code touched)
- `git status --porcelain` → clean after the single commit
- `git log -1 --name-only` → exactly `.planning/decisions/0006-coverage-gate.md`, no other file

**Human-check item** (`05-VALIDATION.md` §Manual-Only Verifications, row 4 — read the amended ADR
end to end and confirm no second floor or target is stated as operative): performed during
authoring; the amendment states in three places (preamble, module-scoped-gate-table preface, and
the original unmodified "target: superseded" clause) that 84% remains the sole binding number and
both module targets are explicitly non-binding until Phase 15 wires them into CI.

## Deviations from Plan

**Commit timing (process deviation, not a plan-content deviation).** The plan's Task 2 instructed a
single commit with a 300000ms Bash timeout because `.pre-commit-config.yaml` sets `always_run: true`
on `cargo-fmt`/`cargo-clippy`, compiling the workspace even for a markdown-only commit. The first
commit attempt still exceeded 5 minutes on a cold target cache and was retried as a background
process; it completed successfully as `9f8b357`. The orchestrator subsequently corrected the
dispatch: this project sets `workflow.worktree_skip_hooks=true`, authorizing `--no-verify` for
worktree commits (the orchestrator runs the full gate once, warm-cache, in the main checkout after
merge). This SUMMARY's own commit uses `--no-verify` per that correction; the ADR-0006 amendment
commit itself (`9f8b357`) already ran the full hook suite to completion before the correction
arrived, so its content is unaffected.

None - plan content executed exactly as written; no Rule 1-4 deviations occurred.

## Self-Check: PASSED

- FOUND: `.planning/decisions/0006-coverage-gate.md` (amended, verified via `git log -1 --name-only`
  against commit `9f8b357`)
- FOUND: commit `9f8b357` in `git log --oneline --all`
- No `.rs`, `Cargo.toml`, or `.github/` file touched (confirmed via `git diff --stat HEAD~1`)
- No second coverage ADR created (confirmed via `ls .planning/decisions/ | grep -c coverage` = 1)
