---
phase: 04-release-coherence
plan: 06
subsystem: docs
tags: [quickstart, mdbook, release-measurement, provenance, rustdoc]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "Plan 04-05's version convergence to 0.7.0 across every manifest, which this plan's
      dependency-block edit and version greps depend on"
provides:
  - "A QUICKSTART sample whose imports, dependency block, and constructor call match the shipped
    tree — proven by an actual offline compile of the exact repaired sample, not eyeballed"
  - "The first recorded QUICKSTART timing measurement (D-17 provenance), with the target settled
    at 15 minutes and the page reconciled with docs/src/introduction.md"
  - "Two named deferrals (clean-machine timing, live LLM execution) filed to Phase 15 / PIPE"
  - "REL-04's documentation-review clause discharged by citing RECON-08's recorded verdict"
affects: [04-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Path-dependency scratch project (outside the repo tree) as a substitute measurement to
      prove a doc sample compiles against unpublished workspace crates, deleted afterward with an
      empty git status --porcelain examples crates confirming no leak"
    - "D-17 provenance block applied to the QUICKSTART timing entry, reusing the plan
      04-01/04-04/04-05 template verbatim"

key-files:
  created: []
  modified:
    - docs/src/getting-started/quickstart.md
    - .planning/phases/04-release-coherence/04-release-measurement.md

key-decisions:
  - "Settled the QUICKSTART target at 15 minutes rather than retaining the page's untested 'under
    five minutes' claim. Both measurements taken here (4m22s in-workspace prefix, sub-5-minute
    substitute compile) came in under five minutes on their own, but neither covers the two most
    time-variable real steps (cold crates.io resolution, live OpenAI API call) — both unmeasurable
    in this sandbox. Retaining 'five minutes' on partial evidence that excludes the dominant
    unknowns would be exactly the dishonest-number risk D-11.2/T-04-26 warn against. quickstart.md:3
    now reads 'under 15 minutes', agreeing with introduction.md:9."
  - "Fixed paladin-llm's feature name in the dependency block from the invalid 'llm-openai' to the
    crate's actual feature 'openai' (verified against crates/paladin-llm/Cargo.toml's [features]
    table). 'llm-openai' is a feature of the root paladin-ai package, not paladin-llm, and using it
    on the wrong crate would have failed to build even after every other fix landed. Treated as
    Rule 1 (bug fix) since the plan's task text didn't call it out by name but the compile proof
    obligation required it."
  - "Confirmed by direct test that a dependent's extern-crate name follows the target's [lib] name
    ('paladin'), not the package name ('paladin-ai') or the dependency-table key — no `package =`
    rename is needed in the quickstart's Cargo.toml, matching how examples/basic_paladin.rs already
    resolves it."

requirements-completed: [REL-04]

coverage:
  - id: D1
    description: "QUICKSTART's imports, dependency block, and PaladinExecutionService::new call
      corrected to match the shipped tree; proven by compiling the exact repaired sample offline
      against the workspace via a path-dependency scratch project"
    requirement: "REL-04"
    verification:
      - kind: other
        ref: "grep -c 'paladin_ai_core::application::services' quickstart.md == 0; grep -q
          'use paladin::application::services::paladin::paladin_builder::PaladinBuilder;' in both
          quickstart.md and examples/basic_paladin.rs; scratch-project 'cargo build --offline'
          against the exact repaired sample text exits 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "First QUICKSTART timing measurement recorded with full D-17 provenance; target
      settled at 15 minutes; two unmeasurable legs (clean-machine timing, live LLM call) filed as
      deferred-with-reason rows with a named owner; REL-04's doc-review clause discharged by citing
      RECON-08"
    requirement: "REL-04"
    verification:
      - kind: other
        ref: "grep -c 'QUICKSTART elapsed time' == 1; grep -c 'measured under stated conditions,
          not a clean-machine claim' == 1; grep -c 'Owner: Phase 15 / PIPE' >= 2; grep -q
          'RECON-08'; quickstart.md:3 and introduction.md:9 both read '15'"
        status: pass
    human_judgment: false

# Metrics
duration: 25min
completed: 2026-08-03
status: complete
---

# Phase 4 Plan 06: QUICKSTART Repair and Timing Measurement Summary

**Fixed the QUICKSTART's structurally-broken import paths and dependency block (proven by an actual offline compile, not inspection), then recorded the first QUICKSTART timing measurement and settled the contested target at 15 minutes.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-08-03T13:13:00Z (approx.)
- **Completed:** 2026-08-03T13:38:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- **Task 1 — repaired the QUICKSTART sample.** Replaced the imports that referenced a
  non-existent `paladin_ai_core::application::services::…` module path with the correct
  `paladin::application::services::paladin::…` path (the root `paladin-ai` package, `[lib] name =
  "paladin"`), verified byte-identical to `examples/basic_paladin.rs`. Added `paladin-ai` to the
  dependency block at `0.7.0`, dropped the now-unused `paladin-ai-core` line, corrected
  `paladin-llm`'s feature name from the invalid `llm-openai` to its real feature `openai`, and
  rewrote the constructor call to match `PaladinExecutionService::new`'s real four-argument
  signature (`Arc<CircuitBreaker>` as argument two, not a defaulted value). Corrected the phalanx
  example name from the non-existent `phalanx_concurrent` to the shipped `phalanx_parallel` and
  uncommented it. Proved the fix by building the exact corrected sample text offline against this
  workspace via a path-dependency scratch project — not by inspection of the `rust,ignore`-fenced
  block.
- **Task 2 — measured the QUICKSTART timing for the first time.** Recorded the offline-reachable
  in-workspace prefix (new-project scaffold + build of the example the page points a user at) at
  **4 minutes 22 seconds**, plus a named substitute measurement proving the repaired sample compiles
  against the shipped tree. Settled the page's target at **15 minutes** — reconciling it with
  `introduction.md:9` — rather than retaining the untested "under five minutes" claim, because the
  two most time-variable real steps (cold crates.io resolution, a live OpenAI API call) could not be
  measured in this sandbox and partial evidence excluding them isn't grounds to keep a tighter
  number. Filed two `deferred with reason` rows (clean-machine timing, live LLM execution), both
  **Owner: Phase 15 / PIPE**. Discharged REL-04's documentation-review clause by citing RECON-08's
  recorded verdict in `.planning/ledgers/milestone-01.md` rather than inventing a review.

## Task Commits

Each task was committed atomically:

1. **Task 1: Repair the QUICKSTART sample so it matches the shipped tree** — `6813725` (fix)
2. **Task 2: Measure the documented steps once, settle the timing claim, and file the deferral** — `76709c4` (docs)

_Worktree mode: STATE.md/ROADMAP.md are owned by the orchestrator after this wave's worktree agents
merge; no plan-metadata commit is made from this worktree._

## Files Created/Modified

- `docs/src/getting-started/quickstart.md` — corrected imports, dependency block (0.7.0, fixed
  `paladin-llm` feature name, added `paladin-ai`, dropped `paladin-ai-core`), constructor call,
  phalanx example name, and the timing claim (five minutes → 15 minutes)
- `.planning/phases/04-release-coherence/04-release-measurement.md` — appended the "QUICKSTART
  elapsed time (first measurement)" section with full D-17 provenance, the substitute measurement,
  the stated-conditions label, two deferral rows, and the RECON-08 citation

## Decisions Made

- **Settled the timing gate at 15 minutes**, not the page's original five-minute claim. See
  `key-decisions` in frontmatter for the full reasoning — both measurements taken here came in
  under five minutes on their own, but neither covers the two real-world legs that dominate a
  developer's actual wall-clock time and neither could be measured in this sandbox, so partial
  evidence was not treated as grounds to keep the tighter claim.
- **Fixed `paladin-llm`'s feature name** (`llm-openai` → `openai`) as part of Task 1, beyond the
  plan's literally-named three corrections, because the compile-proof obligation ("checked line by
  line rather than eyeballed") surfaced it as a real defect that would have blocked compilation.
  Rule 1 (auto-fix bug).
- **Verified the extern-crate-name mechanics directly** rather than assuming them: a dependent
  crate resolves `paladin-ai` (the package name) to `paladin` (the `[lib]` name) in `use`
  statements with no rename needed, confirmed by a scratch build before writing the dependency
  block into the doc.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected `paladin-llm`'s feature name in the dependency block**
- **Found during:** Task 1
- **Issue:** The original quickstart listed `paladin-llm = { version = "0.5.0", features =
  ["llm-openai"] }`. `llm-openai` is a feature of the root `paladin-ai` package
  (`Cargo.toml:259,268`), not of `paladin-llm` itself — `crates/paladin-llm/Cargo.toml`'s
  `[features]` table defines `openai`, `anthropic`, `deepseek`, `mock`, `vision`,
  `openai-embeddings`, with no `llm-openai` entry. Using the wrong feature name on this crate
  would fail to build (`error: Package ... does not have feature llm-openai`) regardless of every
  other correction landing.
- **Fix:** Changed the dependency line to `paladin-llm = { version = "0.7.0", features = ["openai"]
  }` — `paladin-llm`'s own default feature set already includes `openai`, so this is the minimal
  correct form.
- **Files modified:** `docs/src/getting-started/quickstart.md`
- **Verification:** The exact corrected dependency block, used verbatim in the path-dependency
  scratch project (Task 2's substitute measurement), built offline with exit 0.
- **Committed in:** `6813725`

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug)
**Impact on plan:** Necessary for the compile-proof obligation the plan itself set ("structural
equivalence to the in-tree example that does compile, checked line by line rather than eyeballed").
No scope creep — this is the same dependency block the plan's Task 1 already scoped for editing.

## Issues Encountered

None beyond the one deviation above, investigated and resolved.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- QUICKSTART's sample now matches the shipped tree and compiles offline against it (proven, not
  assumed); its timing claim agrees with `introduction.md`.
- The first QUICKSTART measurement is on record with full provenance; the two legs this sandbox
  cannot exercise (clean-machine cold-registry timing, live LLM execution) are named deferrals with
  an owner, not silent gaps.
- REL-04 is fully discharged: the documentation-review clause by citation (D-10/RECON-08), the
  QUICKSTART measurement by this plan's Task 2.
- No blockers for plan 04-07 — this plan's edits (one doc file, one measurement-record append) are
  additive and do not conflict with sibling plans' scopes.

---
*Phase: 04-release-coherence*
*Completed: 2026-08-03*

## Self-Check: PASSED

All claimed files found on disk (`04-06-SUMMARY.md`) and all three commit hashes (`6813725`,
`76709c4`, `36fd198`) found in `git log --oneline --all`.
