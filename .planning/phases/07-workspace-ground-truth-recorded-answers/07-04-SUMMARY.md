---
phase: 07-workspace-ground-truth-recorded-answers
plan: 04
subsystem: docs
tags: [adr, hexagonal-architecture, dependency-allowlist, llm-config, paladin-core, paladin-ports, paladin-llm]

# Dependency graph
requires:
  - phase: 07-01
    provides: ledger scaffold and ARCH-01/D-05/D-26 conventions this plan's ADRs reference
  - phase: 07-02
    provides: ADR-0016 (port value-type ownership), the sibling ARCH-03 ADR this plan's ADR-0015/0017 sit beside
provides:
  - "ADR-0015 — the paladin-core/paladin-ports dependency purity invariant, stated separately from the measured 14/11 dependency baseline, with tokio in paladin-core explicitly justified"
  - "ADR-0017 — the LLM configuration bridge location (crates/paladin-llm/src/config/bridge.rs), with Epic 4's FR-31/FR-32 circular-dependency concern recorded as real-but-mis-sited"
  - "Annotated .project/Milestone_5-.../Epic_4/prd-paladin-llm-extraction.md — dated banner, FR-31/FR-32 Superseded block, Non-Goal 2 inline numbering correction"
affects: [phase-08, phase-15]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR promotion via the seven-heading PROMOTION.md shape", "D-00g annotate-not-rewrite .project/ correction banners"]

key-files:
  created:
    - .planning/decisions/0015-core-ports-dependency-allowlist.md
    - .planning/decisions/0017-llm-config-bridge-location.md
  modified:
    - .project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md

key-decisions:
  - "ADR-0015 states the enforceable invariant (no provider SDK/transport client/storage driver/web framework in paladin-core or paladin-ports) separately from the measured dependency list, per D-10."
  - "paladin-core carries 14 dependencies (6 PRD-permitted + 8 extras: tokio, sha2, blake3, petgraph, murmur3, url, regex, futures), measured this task from crates/paladin-core/Cargo.toml:17-31."
  - "paladin-ports carries 11 dependencies (7 base + 4 extras: serde_json, futures, md5, mime_guess), measured this task from crates/paladin-ports/Cargo.toml:20-31 — correcting intel/code-verification.md's stale figure of 10 (mime_guess was added since)."
  - "tokio in paladin-core is justified explicitly: message_service.rs's background worker pool (tokio::spawn, RwLock/mpsc) and task.rs's async persistence helpers (tokio::fs, tokio::time::sleep) are written directly against it — it is not incidental."
  - "ADR-0017 accepts the shipped bridge location and states Epic 4's FR-31/FR-32 concern was real but mis-sited: Milestone 6 moved the config types (LlmProviderConfig, VisionConfig) down into paladin-llm rather than moving the bridge up into the root, removing the cycle risk rather than violating the original constraint."
  - "Plan 07-04's Task 3 instruction to land all three files in one combined commit could not be honored — the sandbox's git-safety classifier blocked the git reset --soft needed to squash the two prior per-task commits. All three files are committed across three atomic commits instead, matching the executor's default per-task protocol."

requirements-completed: [ARCH-03]

coverage:
  - id: D1
    description: "ADR-0015 records the paladin-core/paladin-ports dependency purity invariant and rebaselines the dependency counts against live-measured manifests"
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0015-core-ports-dependency-allowlist.md == 7; grep -q mime_guess and tokio"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0017 records the LLM config bridge location and the structural resolution of Epic 4's circular-dependency concern"
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0017-llm-config-bridge-location.md == 7; test -f crates/paladin-llm/src/config/bridge.rs"
        status: pass
    human_judgment: false
  - id: D3
    description: "Epic 4 llm-extraction PRD annotated in place with a dated banner, Superseded block above FR-31/FR-32, and Non-Goal 2 inline correction, all original text retained"
    requirement: "ARCH-03"
    verification:
      - kind: manual_procedural
        ref: "07-VALIDATION.md §Manual-Only Verifications, row 2 — confirm FR-31/FR-32 original text is present and unmodified beneath the Superseded block"
        status: unknown
    human_judgment: true
    rationale: "Plan's own <verify><human-check> requires visual confirmation that FR-31/FR-32's requirement text survives unmodified beneath the Superseded banner; not mechanically provable beyond the grep/diff checks already run."

# Metrics
duration: 20min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 04: Core/Ports Dependency Allowlist and LLM Config Bridge Location Summary

**ADR-0015 rebaselines paladin-core (14 deps) and paladin-ports (11 deps) against measurement with tokio explicitly justified; ADR-0017 ratifies the shipped LLM config bridge and explains why Epic 4's cycle concern evaporated structurally.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-06T18:05:00Z (approx.)
- **Completed:** 2026-08-06T18:22:00Z
- **Tasks:** 3
- **Files modified:** 3 (2 created, 1 annotated in place)

## Accomplishments

- **ADR-0015** states the enforceable invariant (no provider SDK, transport client, storage
  driver, or web framework in `paladin-core`/`paladin-ports`) separately from the measured
  dependency lists, and justifies `tokio` in `paladin-core` explicitly by citing its two
  domain-level consumers.
- **ADR-0017** accepts the shipped LLM config bridge at `crates/paladin-llm/src/config/bridge.rs`
  and records Epic 4's FR-31/FR-32 circular-dependency concern as real at the time but resolved
  structurally by Milestone 6's config-type relocation, not by moving the bridge.
- **`prd-paladin-llm-extraction.md`** annotated per D-00g: dated top-of-file banner, a standalone
  Superseded block above FR-31/FR-32, and an inline strikethrough-and-append correction on
  Non-Goal 2's "Milestone 1 / Epic 2" reference — all original requirement text retained.

## Measured Dependency Counts (this task)

- `crates/paladin-core/Cargo.toml` `[dependencies]` (lines 17-31): **14 entries** — `serde`,
  `serde_json`, `uuid`, `chrono`, `thiserror`, `async-trait` (the PRD's original six) plus `tokio`,
  `sha2`, `blake3`, `petgraph`, `murmur3`, `url`, `regex`, `futures` (eight extras). Matches
  CONTEXT.md D-10's figure of 14; no drift.
- `crates/paladin-ports/Cargo.toml` `[dependencies]` (lines 20-31): **11 entries** — `paladin_core`,
  `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, `tokio` (base seven) plus `serde_json`,
  `futures`, `md5`, `mime_guess` (four extras). Matches CONTEXT.md D-10's figure of 11, correcting
  `.planning/intel/code-verification.md`'s stale figure of 10 (predates `mime_guess`).
- `.github/workflows/ci.yml` `crate-isolation` job re-grepped at **line 304** (not the stale `:228`
  some corpus documents cite) — recorded as the exercising artefact in ADR-0015's `## Code
  Locations`.

## `tokio` Justification (as written in ADR-0015)

`tokio` is not an incidental transitive dependency in `paladin-core`. Two domain-level sites are
written directly against it:
- `src/base/service/message_service.rs:20,397,450` — the base `MessageService`'s background
  worker pool: `tokio::sync::{RwLock, mpsc}`, `tokio::spawn`, `tokio::task::JoinHandle`. Every
  platform messaging service (Log, Notification, Event) extends this base service per its own
  header comment.
- `src/platform/container/task.rs:180,355,380,428,456` — `tokio::time::sleep` in domain polling
  logic and `tokio::fs::create_dir_all`/`tokio::fs::write` in the `Task` entity's async
  persistence helpers.

Removing `tokio` would require rewriting both types off `async`/`await` entirely; it is accepted
as baseline, not exempted from scrutiny.

## FR/Non-Goal Lines Annotated (as verified this task)

- **FR-31** (`:211` after banner insertion, originally `:197`) and **FR-32** (`:213`, originally
  `:199`) — a standalone Superseded blockquote inserted immediately above the
  `### 4.8 Configuration Integration` heading; both FRs' own text retained unchanged below it.
- **Non-Goal 2** (`:250` after banner insertion, originally `:240`) — "hardened in Milestone 1 /
  Epic 2" struck through and replaced inline with "Milestone 4 Epic 2 (Port Trait Hardening &
  Stable API)", with a bold ADR-0014 citation explaining the tier-vs-milestone numbering
  convention.
- All original FR-31/FR-32/Non-Goal 2 text bytes survive; total deletion across the file is 1
  line (`git diff --numstat` against the pre-plan base: 15 insertions, 1 deletion).

## Task Commits

Each task was committed atomically (see Deviations below regarding the plan's single-commit
request):

1. **Task 1: ADR-0015 — the purity invariant and the measured dependency baseline** - `331115e`
   (docs)
2. **Task 2: ADR-0017 — LLM configuration ownership and the bridge location** - `4c8450a` (docs)
3. **Task 3: Annotate the Epic 4 llm-extraction PRD** - `ebbe3af` (docs)

## Files Created/Modified

- `.planning/decisions/0015-core-ports-dependency-allowlist.md` - new ADR, seven canonical
  headings, `conforms`, names Phase 15 as the `cargo tree` enforcement candidate
- `.planning/decisions/0017-llm-config-bridge-location.md` - new ADR, seven canonical headings,
  `conforms`, ratifies `crates/paladin-llm/src/config/bridge.rs`
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` - annotated
  in place per D-00g; no text deleted, three corrections added

## Decisions Made

- ADR-0015 records the invariant and rebaselines the list rather than treating the twelve extra
  dependencies as tracked debt — no debt items nobody intends to pay were manufactured.
- ADR-0017 records Epic 4's concern as sound-but-mis-sited rather than mistaken, per D-12's
  explicit instruction not to declare Epic 4 simply wrong.
- Both ADRs cite their measured facts (`grep -c` over the manifests, `grep -rn tokio::` over
  `paladin-core/src/`) rather than transcribing figures from `intel/code-verification.md` or
  CONTEXT.md, per this plan's prohibitions.

## Deviations from Plan

### Auto-fixed / Adapted Issues

**1. [Rule 3 - Blocking, adapted] Single combined commit for all three plan files could not be
made as instructed**
- **Found during:** Task 3 (Annotate the Epic 4 llm-extraction PRD)
- **Issue:** The plan's Task 3 `<action>` explicitly instructs: "Commit this plan's three files
  together in a single commit at the end of the plan." By the time Task 3 was reached, Tasks 1
  and 2 had already been committed individually per the executor's standard per-task atomic
  commit protocol (ADR-0015 in `331115e`, ADR-0017 in `4c8450a`). Combining all three into one
  commit required squashing those two commits together with the PRD change via
  `git reset --soft` back to the plan's base commit.
- **Fix attempted:** `git reset --soft 9257a62...` (the plan's base commit) was run to restage all
  changes for a single combined commit. This command was **blocked by the sandbox's git-safety
  classifier** ("Permission for this action was denied by the Claude Code auto mode classifier").
  Per the destructive-git-prohibition guidance, this is not a restriction to work around; the
  denial was accepted and no alternative reset/rebase mechanism was attempted.
- **Resolution:** Task 3's PRD annotation was committed as its own atomic commit (`ebbe3af`),
  following the executor's default per-task protocol instead. All three files this plan produces
  are committed and present in the worktree's history; they land as three atomic commits (one per
  task) rather than one combined commit.
- **Files affected:** none beyond the normal per-task commits already made.
- **Verification:** `git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/'` is empty for the
  final commit (satisfied); `git log -1 --name-only` shows only the PRD file, not all three (the
  one acceptance criterion this deviation does not satisfy literally — see below).

---

**Total deviations:** 1 adapted (commit-structure only; no content, scope, or correctness impact).
**Impact on plan:** The plan's Task 3 acceptance criterion "`git log -1 --name-only` shows both
new ADRs and the annotated PRD in a single commit" is **not** literally satisfied — the three
files span three consecutive commits (`331115e`, `4c8450a`, `ebbe3af`) instead of one. Every other
acceptance criterion in Tasks 1-3 and the plan-level `<verification>` block is satisfied,
including the deletion-count bound, the `*.rs`/`Cargo.toml`/`.github/` no-touch guarantee, and all
content requirements (both ADRs' seven headings, `conforms` verdicts, `mime_guess`/`tokio`
citations, and the PRD's FR-31/FR-32/Non-Goal 2 preservation). No product code, requirement
content, or ADR substance was affected by this deviation.

## Issues Encountered

- The worktree's git-safety sandbox blocks `git reset` (including `--soft`) even when applied
  only to commits created in the current session on the current agent's own branch. This is a
  hard environmental constraint, not a workaround target — documented above rather than retried
  with a different reset invocation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0015 and ADR-0017 are both live, citable ADRs answering two of ARCH-03's four
  competing-variant pairs (b and d). ARCH-03(a) was already answered by Phase 4 (ADR-0009);
  ARCH-03(c) is ADR-0016, landed by plan 07-02.
- Phase 8 / DEBT-05 is unaffected by this plan (it depends on ADR-0016, not on this plan's ADRs).
- Phase 15 has a named candidate (the `cargo tree`-based allowlist check) recorded in ADR-0015's
  `## Downstream Consumers`, not built here.
- Plans 07-06 and 07-10 (ledger fan-out) and 07-13 (bookkeeping) can now cite ADR-0015 and
  ADR-0017 by number for their `REQ-paladin-core-dependency-allowlist-*` and
  `REQ-llm-config-bridge-location-*` ledger rows.

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0015-core-ports-dependency-allowlist.md`
- FOUND: `.planning/decisions/0017-llm-config-bridge-location.md`
- FOUND: `.planning/phases/07-workspace-ground-truth-recorded-answers/07-04-SUMMARY.md`
- FOUND: commit `331115e` (ADR-0015)
- FOUND: commit `4c8450a` (ADR-0017)
- FOUND: commit `ebbe3af` (PRD annotation)
- FOUND: commit `545ec14` (this SUMMARY)
