---
phase: 07-workspace-ground-truth-recorded-answers
plan: 09
subsystem: docs
tags: [feature-flags, cargo-manifest, adr-citation, cli-isolation, workspace-decomposition, use-cases-relocation]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "ADR-0011 (07-plan prior), ADR-0014, ADR-0018, ADR-0019 (plans 07-01, 07-05, 07-07) — the decisions this plan's source corrections cite"
provides:
  - "Five ARCH-05 positions corrected at source across four `.project/` documents, each with a dated banner, original claim text retained, and a fresh Cargo.toml/tree re-grep backing the correction"
affects: [07-08, 07-12, ledger-plans-citing-diverged-rows]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-00g annotation (dated banner + inline strikethrough-and-append, original retained)", "shape (c) standalone Superseded banner for whole fenced-code sections"]

key-files:
  created: []
  modified:
    - .project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md
    - .project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md
    - .project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md
    - .project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md

key-decisions:
  - "Where a correction site sat inside a fenced code block (Appendix B's mcp-transports TOML, the Target Workspace Structure ASCII tree, Appendix D's Cargo.toml template), used markup shape (c) — a standalone Superseded blockquote banner outside the fence, with a rendered ~~struck~~ reference inside the banner's own prose — instead of injecting literal `~~` markers into the fence body. This keeps the fenced content byte-for-byte original (fully readable as the un-adopted proposal) while still satisfying the `grep -c '~~'` verification and the plan's per-file deletion budgets (M4 overview ≤2, M5 overview ≤5)."
  - "Plan Task 3's action instructed a single combined commit for all four of this plan's files; Tasks 1 and 2 were already committed atomically before that instruction was executed. Continued with three atomic per-task commits rather than a git reset --soft squash (blocked by this sandbox's git-safety classifier), per the executor's parallel_execution deviation guidance. Documented below."

patterns-established:
  - "Shape (c) applied to fenced-code correction sites: banner-before-fence with an in-banner struck reference, zero deletions to the fenced content itself."

requirements-completed: [ARCH-05, ARCH-02, ARCH-04]

coverage:
  - id: D1
    description: "Epic 1 feature-flag PRD (prd-expand-feature-flags.md) carries a dated banner and three inline corrections — vision gating, MCP transport flags, web-server gating — each citing a freshly re-grepped Cargo.toml line and retaining original claim text."
    requirement: "ARCH-05"
    verification:
      - kind: other
        ref: "grep -c '~~' prd-expand-feature-flags.md == 3; grep -c ADR-0011 >= 1; grep -c dependency-matrix.md >= 1; grep -c no-default-features >= 1; grep -c chacha20poly1305 >= 1; git diff --numstat HEAD deletions == 2 (budget ≤4); git diff --numstat HEAD -- dependency-matrix.md empty"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 4 overview's MCP acceptance-criterion (AC1) and Appendix B annotated superseded, added to plan 07-01's existing ADR-0014 banner without disturbing it; Task 1.5's transport argument noted as superseded-by-elimination-note rather than by faulty reasoning."
    requirement: "ARCH-05"
    verification:
      - kind: other
        ref: "grep -c ADR-0014 == 3 (>=2 required); grep -ci MCP == 19 (>=2, at least one inside a strikethrough); git diff --numstat HEAD deletions == 1 (budget ≤2)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Milestone 5 overview's title numbering corrected, and its Target Workspace Structure, Appendix C dependency matrix, and Appendix D Cargo.toml template's paladin-cli entries marked superseded — no such crate was ever built; CLI ships as the cli feature plus a required-features-gated [[bin]] paladin-cli target (ADR-0019 cited)."
    requirement: "ARCH-05"
    verification:
      - kind: other
        ref: "grep -c '~~' == 4 (>=4 required); grep -c ADR-0019 == 4 (>=2); grep -c ADR-0014 == 3 (>=2); test -d crates/paladin-cli fails; git diff --numstat HEAD deletions == 2 (budget ≤5)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Milestone 6 Epic 2 PRD's src/application/use_cases/ target directory corrected inline to src/application/services/ with the four shipped module paths and line numbers cited; Non-Goal 7 and Open Question 4 get confirmation notes citing ADR-0018 without re-opening either."
    requirement: "ARCH-04"
    verification:
      - kind: other
        ref: "grep -c ADR-0018 == 5 (>=4 required); grep -c src/application/services >= 1; grep -c use_cases == 33 (original survives); grep -c '~~' >= 1; test -d src/application/use_cases fails; git diff --numstat HEAD deletions == 1 (budget ≤2)"
        status: pass
    human_judgment: false

# Metrics
duration: ~45min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 09: Correct the Five ARCH-05 Divergences Summary

**Five shipped-code-vs-PRD divergences (vision gating, MCP flags, web-server gating, paladin-cli crate, use_cases/ target) corrected at source across four `.project/` documents with dated D-00g banners, all original text retained.**

## Performance

- **Duration:** ~45 min
- **Completed:** 2026-08-06T19:17:28Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- Corrected the Epic 1 feature-flag PRD (`prd-expand-feature-flags.md`) inline for all three of its own defects: `vision = []` (`Cargo.toml:274`) gates no dependency and `chacha20poly1305`/`zeroize` (`Cargo.toml:134-135`) are unconditional; no MCP feature flag of any kind shipped (the PRD's own 2026-04-15 elimination note is what shipped); and `web-server = ["dep:paladin-web", "dep:axum"]` (`Cargo.toml:276`) does not gate `actix-web` (`actix` returns zero hits in root `Cargo.toml`, re-confirmed).
- Extended plan 07-01's ADR-0014 banner on the Milestone 4 overview with the MCP finding, corrected acceptance criterion 1 inline, marked Appendix B's `mcp-transports`/`mcp-stdio`/`mcp-sse` block Superseded, and annotated Task 1.5's transport-isolation argument as superseded by the elimination note rather than wrong reasoning.
- Added a dated ADR-0014/ADR-0019 banner to the Milestone 5 overview, corrected its title numbering, and marked its Target Workspace Structure, Appendix C dependency matrix, and Appendix D `Cargo.toml` template's `paladin-cli` entries Superseded — no such crate exists in `crates/` (ten library crates plus `doc-examples`, listed directly); the CLI ships as `cli = [...]` (`Cargo.toml:284`) plus `[[bin]] paladin-cli` with `required-features = ["cli"]` (`Cargo.toml:245-247`).
- Corrected the Milestone 6 Epic 2 PRD's target directory inline (`src/application/use_cases/` struck through, `src/application/services/` appended, all four shipped orchestrator module paths cited with line numbers), and added confirmation notes on Non-Goal 7 and Open Question 4 citing ADR-0018 without re-opening either.

## Task Commits

Each task was committed atomically:

1. **Task 1: Correct the Epic 1 feature-flag PRD** - `0562cc3` (docs)
2. **Task 2: Correct the two milestone overviews** - `bd28e27` (docs)
3. **Task 3: Correct the Milestone 6 Epic 2 PRD** - `abf061d` (docs)

No separate plan-metadata commit was made prior to this SUMMARY; the plan-metadata commit follows this file per the standard `<final_commit>` step.

_Note: this is a records-only phase; no `test`/`feat`/`refactor` commit types apply._

## Files Created/Modified

- `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md` - dated banner + three inline corrections (vision, MCP, web-server)
- `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md` - MCP correction on AC1 + Appendix B, added to plan 07-01's ADR-0014 banner
- `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md` - dated banner, title correction, three `paladin-cli` corrections (Target Structure, Appendix C, Appendix D)
- `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md` - dated banner, target-directory correction, Non-Goal 7 / Open Question 4 confirmations

**Files cited but deliberately not edited (verified via `git diff --numstat HEAD~3`, empty):** `dependency-matrix.md` (the audit that was right) and `prd-workspace-finalization-epic-6.md` (whose non-goal was correct).

## Fresh Citations Recorded This Session

Per the plan's `<output>` requirement, every `Cargo.toml`/tree fact cited was re-grepped during this task rather than transcribed:

- `Cargo.toml:274` — `vision = []`
- `Cargo.toml:276` — `web-server = ["dep:paladin-web", "dep:axum"]`
- `Cargo.toml:284` — `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`
- `Cargo.toml:134` — `chacha20poly1305 = "0.10"` (unconditional)
- `Cargo.toml:135` — `zeroize = { version = "1.8", features = ["derive"] }` (unconditional)
- `Cargo.toml:241-243` — `[[bin]] name = "paladin"`, `path = "src/main.rs"`, no `required-features`
- `Cargo.toml:245-247` — `[[bin]] name = "paladin-cli"`, `required-features = ["cli"]`
- `Cargo.toml:250-252` — `[[bin]] name = "paladin-server"`, `required-features = ["web-server"]`
- `grep -rc 'actix' Cargo.toml` → `0` (no root `actix-web`/`actix` dependency)
- `grep -n 'mcp' Cargo.toml` → no output (no MCP feature flag of any kind)
- `grep -rn 'chacha20poly1305\|zeroize' Cargo.toml crates/*/Cargo.toml` → both confined to root `Cargo.toml`, unconditional
- `crates/` listing → `doc-examples`, `paladin-battalion`, `paladin-content`, `paladin-core`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-notifications`, `paladin-ports`, `paladin-storage`, `paladin-web` (ten library crates + `doc-examples`; no `paladin-cli`)
- `src/application/` listing → `cli`, `errors`, `mod.rs`, `services` (no `use_cases`)
- `src/application/services/{notification_orchestrator,queue_orchestrator,orchestration,log_orchestrator}/mod.rs` — the four shipped module paths, with `NotificationOrchestrator` at `:64`, `QueueOrchestrator` at `:46`, `Orchestrator` at `:56`, `LogOrchestrator` at `:47`

## Decisions Made

- **Shape (c) for fenced-code correction sites.** Three of the five positions (MCP flags in Appendix B, the `paladin-cli` entry in the Target Workspace Structure ASCII tree, and the `paladin-cli` member in Appendix D's `Cargo.toml` template) live inside fenced code blocks where literal `~~` markers would not render as strikethrough and would count as deletions against the plan's tight per-file deletion budgets (M4 overview ≤2, M5 overview ≤5). Used PATTERNS.md markup shape (c) — a standalone "Superseded" blockquote banner placed immediately before the fence, with a rendered `~~struck~~` reference inside the banner's own prose (which does render, since blockquotes are ordinary markdown) — leaving the fenced content itself completely untouched. This satisfies both the `grep -c '~~'` verification and the letter of D-00g (original retained, correction dated and cited) while staying inside budget.
- **Single-line strikethrough for compact clauses.** Where the defective claim was a single bullet or line (the Epic 1 PRD's `vision`/`web-server` FR1 bullets, the M4 overview's AC1 MCP bullet, the M5 overview's H1 and Appendix C table row, the M6 PRD's target-directory path line), struck the line directly inline per the plan's literal instruction, since each costs only one deletion.
- **MCP correction in the Epic 1 PRD did not add a new strikethrough.** The PRD's own `mcp-arsenal` flag was already struck through and marked ELIMINATED by its original 2026-04-15 author note — that pre-existing strikethrough is not this plan's correction. Added a bold "Confirmed" note beneath it (pure insertion, no deletion) formalizing it as the citation the ledger's `diverged` rows and the Milestone 4 overview's own MCP correction point at.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule N/A — commit-strategy deviation, documented per `<parallel_execution>` guidance] Combined-commit instruction executed as three atomic commits**
- **Found during:** Task 3
- **Issue:** Task 3's `<action>` block instructs: "Commit this plan's four files together in a single commit at the end of the plan, with a Bash timeout of at least 300000 ms." Tasks 1 and 2 were already committed atomically (per the standard per-task commit protocol) before Task 3's full action text was executed, so a genuinely single combined commit covering all four files was no longer possible without rewriting history.
- **Fix:** Per this executor's `<parallel_execution>` guidance ("If your plan asks for a combined commit, stage everything and commit once the FIRST time. Otherwise commit atomically per task and record the deviation in SUMMARY.md."), continued with atomic per-task commits (`0562cc3`, `bd28e27`, `abf061d`) rather than attempting `git reset --soft` to squash — this sandbox's git-safety classifier blocks that operation, and the executor's `<destructive_git_prohibition>` step independently forbids attempting a workaround.
- **Files modified:** all four of this plan's target files, split across the three commits exactly as the per-task boundaries describe.
- **Verification:** `git log --oneline` shows three commits, each containing exactly the files their respective task edited; the combined content across the three commits is byte-identical to what a single combined commit would have produced. The plan's Task 3 acceptance criterion "`git log -1 --name-only` shows all four annotated documents in a single commit" is the one criterion this plan does not literally satisfy — it is satisfied in substance (all four files' final committed content matches the plan's requirements) but not in the literal single-commit form.
- **Committed in:** `0562cc3`, `bd28e27`, `abf061d`

---

**Total deviations:** 1 (commit-strategy only; no code or content deviations)
**Impact on plan:** No impact on the substance of any correction — all `<acceptance_criteria>` for individual files pass; only the single-combined-commit framing of Task 3's acceptance criteria is not literally met. Every file's final content, banner, and citation matches what the plan specified.

## Issues Encountered

- **Deletion-budget vs. literal-strikethrough tension inside fenced code blocks.** An initial pass struck through every line of the Appendix B `mcp-transports` TOML block and the Target Workspace Structure's `paladin-cli` sub-tree individually, which produced 7 deletions in the Milestone 4 overview (budget: ≤2) and would have produced a similarly over-budget diff in the Milestone 5 overview. Resolved by reverting those in-fence strikethroughs to pristine original text and applying markup shape (c) instead (see Decisions Made above) — final deletion counts: Epic 1 PRD 2, Milestone 4 overview 1, Milestone 5 overview 2, Milestone 6 PRD 1 — all within their respective budgets.

## User Setup Required

None — no external service configuration required.

## Flagged Assumption (ARCH-05, carried from PLAN.md)

The plan's `<flagged_assumptions>` block surfaces this explicitly rather than silently resolving
it: **the five positions ARCH-05 enumerates are treated as exactly five, closed, and each settled
by the shipped tree with no competing defensible position** — so a source correction plus a
`diverged` ledger row is the whole remedy and no ADR was warranted for any of them. This plan
executed on that assumption without independently re-deriving it. If a sixth literal-application
hazard exists in the Milestone 4-6 corpus that this enumeration misses, it is not corrected by this
plan and will surface only when someone applies it. The mitigation available is the ledger's own
`diverged` verdict class, which a later fan-out plan (07-08 or 07-12) may assign to a row in the
same shape if one is found.

## Next Phase Readiness

- All five ARCH-05 positions now have a source correction a future implementer will read before applying the old PRD literally; the `vision` clause's build-breaking consequence is stated in the document itself.
- Plans 07-08 and 07-12 (Milestone 6 and Milestone 4 ledger fan-outs) can cite this plan's corrected documents when writing their `diverged` ledger rows — the citation targets (banner date, ADR numbers, struck/appended text) are now stable.
- `.planning/ledgers/milestone-04-06.md` was not touched by this plan, as required (owned by a concurrent wave plan).
- No `*.rs`, `Cargo.toml`, or `.github/` file was modified by this plan (verified via `git diff --stat HEAD~3 -- '*.rs' 'Cargo.toml' '.github/'`, empty).

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*

## Self-Check: PASSED

- FOUND: `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md`
- FOUND: `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md`
- FOUND: `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md`
- FOUND: `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md`
- FOUND: commit `0562cc3`
- FOUND: commit `bd28e27`
- FOUND: commit `abf061d`
