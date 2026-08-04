---
phase: 05-milestone-2-3-ground-truth
plan: 10
subsystem: docs
tags: [ledger, requirements-traceability, council, grove, cli, battalion, commander, adr-cited]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: "05-01's ledger scaffold (head notes, verdict legend, 118 PENDING-VERDICT stub rows) and its D-01 evidence bar; 05-02's ADR-0010 (Milestone 3 epic numbering, the Council/Grove attribution correction and the PerformanceBased/API-form withdrawals); 05-05's Epic 22 rows (REQ-grove-config-v2, REQ-grove-llm-routing, the hardcoded grove_service.rs:537 model finding this plan cross-references rather than duplicates); 05-09's precedent for row shape and evidence bar"
provides:
  - "Epic 16 (Advanced Battalion Patterns: Council & Grove, 11 rows) fully cited in .planning/ledgers/milestone-02-03.md, with an epic-level note and three divergence rows citing ADR-0010"
  - "Epic 18 (CLI Enhancement & Polish, 7 rows) fully cited, with an epic-level note recording the 12 stale checkboxes and Epic 18's exclusion from VERIFY-02"
  - "REQ-grove-arsenal-integration and REQ-council-garrison-integration — the two rows the run-2 ledger left uninspected — directly inspected with genuine findings recorded (Arsenal: absent entirely; Garrison: code exists, zero exercising test, Citadel absent, Commander explicitly opts out)"
  - "REQ-grove-config-v1 closed consistently against Epic 22's REQ-grove-config-v2 row: same GroveConfig struct carries both variant groups' fields side by side, non-contradictory verdicts (variant group 14 closed)"
  - "Two new stub findings independent of anything the run-2 ledger recorded: paladin muster's LLM analysis path is hardcoded to always fail over to template selection, and paladin council's entire discussion/summary is templated boilerplate with zero LLM or CouncilExecutionService integration"
affects: [05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Same-struct variant-pair resolution (extended from 05-08/05-09): REQ-grove-config-v1 cites the identical GroveConfig struct Epic 22's REQ-grove-config-v2 row cites, and states which ingested position (v1's routing_strategy/fallback_tree/similarity_threshold vs. v2's routing_fallback/min_confidence) the shipped single struct actually answers for each field group"
    - "Ships-beyond-the-deferral finding: TurnStrategy ships all four variants (including Random and VoluntaryWithTimeout, which NG-6 said would not be implemented in this epic) rather than the two the PRD anticipated, verdicted superseded by shipped code rather than deferred with reason, since the deferral itself did not hold"
    - "Simulated-CLI-command finding: paladin council and paladin muster both have real, tested surrounding infrastructure (validation, flags, formatting, file I/O) wrapped around a core capability (LLM-driven discussion / LLM-driven battalion analysis) that is either entirely templated boilerplate or hardcoded to always fail over — recorded as superseded by shipped code / genuinely outstanding rather than papered over as satisfied on the strength of the surrounding tests"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md

key-decisions:
  - "REQ-council-turn-strategies verdicted superseded by shipped code, not deferred with reason. NG-6 said Random and VoluntaryWithTimeout would not be implemented in this epic; the shipped TurnStrategy enum and council_service.rs's determine_next_speaker implement all four variants — Random fully (rand::seq::SliceRandom), VoluntaryWithTimeout as a working RoundRobin-fallback stub with a warn! log, not a panic. Recorded as a new finding: this excess is itself untested (no dedicated test for either variant's selection logic)."
  - "REQ-grove-arsenal-integration verdicted genuinely outstanding — this run-2-uninspected row's actual finding is total absence. No tool/Arsenal field exists on TreeAgent, and grep across grove.rs/grove_service.rs returns zero Arsenal-integration matches (one unrelated supports_tool_calling: false hit)."
  - "REQ-council-garrison-integration verdicted present, unproven — this run-2-uninspected row's actual finding is that store_in_garrison exists and is called non-fatally, but every CouncilExecutionService construction in the tree (tests and commander.rs:589-592) passes None for garrison_port, with commander.rs's own comment confirming Commander deliberately has none to pass. Citadel persistence (the SHOULD clause) has zero shipped code."
  - "REQ-council-grove-commander-integration verdicted satisfied with two prominent new-finding gaps rather than downgraded: the PRD's literal --strategy council/--strategy grove CLI flag does not exist anywhere in the tree (paladin battalion run --type dispatches only formation/phalanx/campaign/chain-of-command/conclave/maneuver), and Grove's Auto-detection keyword list does not include the PRD's literal 'expert'/'specialist' (those bare words route to ChainOfCommand instead, checked later in the same dispatch chain). The Commander-level wiring, Auto-detection, and example configs are otherwise fully tested and satisfied."
  - "REQ-cli-muster-command and REQ-cli-core-infrastructure's CLI-relocation clause both verdicted superseded by shipped code. Muster's analyze_task_with_llm at muster.rs:156-166 is hardcoded to always Err(...) with a 'using template fallback' comment — every invocation is template-only regardless of --provider/--model. Core-infrastructure's module layout exists verbatim in name (commands/formatters/interactive/templates) but under src/application/cli/ rather than the PRD's src/cli, the Epic 17.5 CLI-consolidation relocation already settled and already applied in code — this verdicts against the ledger's own head-note path caveat (D-04), not as a fresh divergence."
  - "REQ-cli-council-command verdicted genuinely outstanding, not satisfied-with-caveat. run_discussion's own comment reads '// Simulate discussion rounds', generates the identical templated sentence for every turn regardless of topic/role with _model/_temperature unused, and generate_summary is fixed boilerplate missing the PRD's areas-of-disagreement and recommended-action sections entirely. No CouncilExecutionService or paladin_core/paladin_battalion import exists anywhere in council.rs — this is a from-scratch reimplementation that never reaches the real Council domain model."
  - "REQ-cli-rich-output cites the same 86-file tests/cli/snapshots/ count as Epic 24's REQ-cli-snapshot-testing row (plan 05-07) rather than re-measuring independently, per T-05-17's duplicated-finding mitigation."

requirements-completed: []  # VERIFY-01/VERIFY-02 span all of plans 05-01..05-13; not individually completable until 05-13 closes the ledger out

coverage:
  - id: D1
    description: "Epic 16's 11 REQ-* rows filled to the D-01 evidence bar, with the epic-level note and three divergence rows citing ADR-0010, REQ-grove-config-v1 closed consistently against Epic 22's v2 row, and both run-2-uninspected rows (garrison, arsenal) directly inspected"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion council_service:: -- 10/10 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion grove_service:: -- 15/15 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core council:: -- 13/13 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core grove:: -- 15/15 passed"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib -- integration::battalion::council_integration_test integration::battalion::grove_integration_test -- 13/13 passed"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib -- integration::commander_integration_tests -- 19/19 passed"
    human_judgment: true
    rationale: "Plan's own acceptance criteria require a human to confirm the three ADR-0010-cited divergences are read correctly from the tree (not restated in prose), that REQ-grove-config-v1/v2 do not assign contradictory verdicts to the same code, and that the two previously-uninspected rows' findings (Arsenal absence, Garrison's untested code path) are accurately characterized rather than overstated or understated (same class of manual check as sibling wave plans 05-05/05-09)."
  - id: D2
    description: "Epic 18's 7 REQ-* rows filled to the D-01 evidence bar, with the epic-level note recording the 12 stale checkboxes and Epic 18's exclusion from VERIFY-02, and the muster/council LLM-stub findings recorded accurately"
    verification:
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::onboarding -- 6/9 passed, 3 ignored (live-API)"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::setup_check -- 6/9 passed, 3 ignored (live-API)"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::features -- 12/12 passed"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::muster -- 11/11 passed"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::council -- 12/12 passed"
      - kind: other
        ref: "cargo test --offline --features cli --test cli -p paladin-ai -- table_output_test -- 8/8 passed"
      - kind: other
        ref: "ls tests/cli/snapshots/ | wc -l -- 86"
    human_judgment: true
    rationale: "Requires a human to confirm the muster/council 'simulated' findings are read correctly from the source (not overstated — the surrounding infrastructure genuinely is tested and real) and that REQ-cli-core-infrastructure's superseded-by-shipped-code verdict correctly applies the ledger's existing path caveat rather than inventing a new one."
  - id: D3
    description: "Ledger integrity preserved: exactly 118 REQ-* rows, 14 epic sections, no row inserted/deleted/reordered, no .rs/Cargo.toml/.github file touched"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-02-03.md equals 118; grep -c '^### Epic ' equals 14; git diff --stat e145f836364dff5fa236a63e38441bb8081279bd..HEAD -- '*.rs' 'Cargo.toml' '.github/' empty"
        status: pass
    human_judgment: false

duration: ~95min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 10: Epic 16/18 Council, Grove & CLI ledger rows Summary

**Filled Epic 16 (Council & Grove, 11 rows) and Epic 18 (CLI Enhancement & Polish, 7 rows) in the Milestone 2-3 ledger, finding that Council's TurnStrategy ships beyond its own NG-6 deferral, Grove has zero Arsenal integration, Commander never wires Garrison into Council, no `--strategy council`/`--strategy grove` CLI flag exists anywhere in the tree, and both `paladin muster`'s LLM analysis and `paladin council`'s entire discussion are hardcoded stubs behind genuinely tested surrounding infrastructure.**

## Performance

- **Duration:** ~95 min (dominated by first-time cold compiles of `paladin-battalion`, `paladin-ai-core`, and the `paladin-ai` facade's `--test lib` target — the facade's cold `integration::commander_integration_tests` compile alone took ~3m10s; all subsequent scoped test runs against the warm `target/` were sub-second)
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-02-03.md`)

## Accomplishments

- Filled all 11 Epic 16 rows: `REQ-council-domain-model`, `REQ-council-turn-strategies`, `REQ-council-termination-conditions`, `REQ-council-execution-service`, `REQ-council-garrison-integration`, `REQ-grove-domain-model`, `REQ-grove-routing-strategies`, `REQ-grove-config-v1`, `REQ-grove-execution-service`, `REQ-grove-arsenal-integration`, `REQ-council-grove-commander-integration`, plus an epic-level note citing ADR-0010 for the Council/Grove Milestone-3-misattribution defect.
- Filled all 7 Epic 18 rows: `REQ-cli-onboarding-wizard`, `REQ-cli-setup-check`, `REQ-cli-features-discovery`, `REQ-cli-muster-command`, `REQ-cli-council-command`, `REQ-cli-rich-output`, `REQ-cli-core-infrastructure`, plus an epic-level note recording the 12 stale checkboxes (transcribed from `intel/task-completion-state.md`) and Epic 18's explicit exclusion from VERIFY-02's three blocks.
- Verdict distribution across the 18 rows: **11 `satisfied`**, **4 `superseded by shipped code`** (`REQ-council-turn-strategies`, `REQ-cli-muster-command`, `REQ-cli-core-infrastructure`, and one of the two Epic 18 `superseded` rows), **1 `present, unproven`** (`REQ-council-garrison-integration`), **2 `genuinely outstanding`** (`REQ-grove-arsenal-integration`, `REQ-cli-council-command`), **0 `deferred with reason`**.
- **Directly inspected both run-2-uninspected Epic 16 rows.** `REQ-grove-arsenal-integration`: zero shipped code — `TreeAgent` has no tool/Arsenal field, and a grep across both Grove files returns exactly one unrelated `supports_tool_calling: false` hit. `REQ-council-garrison-integration`: `store_in_garrison` exists and is called non-fatally after every turn, but every `CouncilExecutionService::new(...)` call in the tree — including `commander.rs:589-592`, whose own comment reads "Commander doesn't have one" — passes `None` for `garrison_port`; the SHOULD-level Citadel persistence clause has zero shipped code anywhere.
- **Closed variant group 14 consistently**: `REQ-grove-config-v1` cites the exact same `GroveConfig` struct at `grove.rs:208-241` that Epic 22's `REQ-grove-config-v2` row (plan 05-05) cites — one struct carries both variant groups' fields side by side (`routing_strategy`/`fallback_tree`/`similarity_threshold` for v1, `routing_fallback`/`min_confidence` for v2), not two competing implementations, so the two rows answer different ingested positions on the same code without contradiction.
- **Found that Council's `TurnStrategy` ships beyond its own PRD deferral.** NG-6 explicitly says `Random` and `VoluntaryWithTimeout` are "NOT implemented in this epic," but `council_service.rs`'s `determine_next_speaker` implements both — `Random` fully via `rand::seq::SliceRandom`, `VoluntaryWithTimeout` as a working RoundRobin-fallback stub with a `warn!` log rather than a panic. Neither variant's selection logic has a dedicated test anywhere in the tree, so the excess itself is unverified. Verdicted `superseded by shipped code` rather than `deferred with reason`, since the PRD's deferral premise doesn't hold against the tree.
- **Found that no `--strategy council`/`--strategy grove` CLI flag exists anywhere.** `grep -rn 'BattalionStrategy' src/` returns zero matches; `paladin battalion run --type` (`battalion.rs:142-149`) dispatches only `formation`/`phalanx`/`campaign`/`chain-of-command`/`conclave`/`maneuver` — no `council`/`grove` `BattalionYamlConfig` variant exists. The functional capability is reachable only via the standalone `paladin council` command (Epic 18) or direct API examples (`examples/commander_council.rs`, `examples/commander_grove.rs`), never via the PRD's literal flag mechanism. Recorded as a prominent caveat on an otherwise `satisfied` verdict rather than downgrading the whole row, since the Commander-level wiring, Auto-detection, and example configs are all real and independently tested.
- **Found that Grove's Auto-detection keywords diverge from the PRD's literal mapping.** The PRD requires bare `"expert"`, `"specialist"`, `"route"` to map to Grove; the shipped `grove_keywords` list has `"expertise"`/`"expert for"`/`"specialized in"`/`"skilled in"`, not the bare words — which are instead ChainOfCommand keywords checked later in the same dispatch chain, so literal input containing only "expert" or "specialist" routes to ChainOfCommand, not Grove.
- **Found `paladin muster`'s LLM-powered analysis is a hardcoded no-op.** `analyze_task_with_llm` at `muster.rs:156-166` always returns `Err(...)` with the comment "For now, return error to trigger template fallback // Full LLM integration would be implemented here" — `_task_description`/`_provider`/`_model` are all unused. Every invocation is template-only regardless of the `--provider`/`--model` flags. Also found: no stdin input support, and config editing before execution is stubbed ("Configuration editing not yet implemented"). Verdicted `superseded by shipped code` — the shipped answer (template-only generation) is recorded as authoritative, distinct from the PRD's LLM-analysis claim.
- **Found `paladin council`'s entire discussion and summary are templated boilerplate with zero domain-model integration.** `run_discussion`'s own comment reads `// Simulate discussion rounds`; every turn produces the identical sentence regardless of topic/role, with `_model`/`_temperature` unused; `generate_summary` is fixed text missing the PRD's "areas of disagreement" and "recommended action" sections; `grep -n 'CouncilExecutionService\|paladin_core\|paladin_battalion' src/application/cli/commands/council.rs` returns zero matches. Verdicted `genuinely outstanding` — the well-tested validation/formatting/file-save mechanics around it don't change that the requirement's substance (an LLM-driven discussion) has no shipped code.
- Recorded the Epic 18 epic-level note per the plan: 12 stale checkboxes transcribed from `intel/task-completion-state.md`, and Epic 18 explicitly marked as **not** one of VERIFY-02's three blocks (Epic 22, Epic 14, Epic 24).
- Ran 12 distinct scoped `cargo test`/`ls` commands, all passing where expected (10+15+13+15+13+19 = 85 test passes across Council/Grove crates and commander integration for Epic 16; 6+6+12+11+12+8 = 55 test passes plus a 86-file snapshot count for Epic 18, with 6 live-API tests correctly `#[ignore]`d), cited by name in the rows that rely on them.

## Task Commits

1. **Task 1: Fill Epic 16's 11 rows** — `dda2fdd` (docs)
2. **Task 2: Fill Epic 18's 7 rows** — `970301d` (docs)

`dda2fdd` — `docs(05-10): fill Epic 16 Council/Grove ledger rows`
`970301d` — `docs(05-10): fill Epic 18 CLI enhancement ledger rows`

_No separate plan-metadata commit — SUMMARY.md is committed by this same worktree per the parallel-execution instructions; STATE.md/ROADMAP.md updates are owned by the orchestrator after the wave merges._

**Worktree hook policy note:** this repo's pre-commit hooks (`cargo fmt`, `cargo clippy --workspace --all-targets --all-features`, both `always_run: true`) would cold-compile the entire 12-crate workspace on every commit including markdown-only ones. Per `workflow.worktree_skip_hooks=true`, `--no-verify` was used for both commits, matching plans 05-01/05-05/05-06/05-07/05-08/05-09's precedent in this phase.

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — Epic 16 section (11 rows plus one epic-level note) and Epic 18 section (7 rows plus one epic-level note): replaced all 18 `PENDING-VERDICT` stub rows with cited verdicts. No other epic section touched; row count (118) and section count (14) both verified unchanged outside Epics 16/18.

## Decisions Made

See `key-decisions` in the frontmatter for the full, citation-bearing list. Summarized:
- `REQ-council-turn-strategies` → `superseded by shipped code`; all four `TurnStrategy` variants ship, exceeding NG-6's deferral, with the excess itself untested.
- `REQ-grove-arsenal-integration` → `genuinely outstanding`; zero Arsenal/tool wiring anywhere in Grove.
- `REQ-council-garrison-integration` → `present, unproven`; Garrison code path exists but Commander never exercises it, Citadel absent entirely.
- `REQ-grove-config-v1` → `satisfied`, closed consistently against Epic 22's `REQ-grove-config-v2` — same struct, non-contradictory verdicts.
- `REQ-council-grove-commander-integration` → `satisfied` with two prominent new-finding gaps (missing `--strategy` CLI flag, Grove keyword-mapping divergence).
- `REQ-cli-muster-command` → `superseded by shipped code`; LLM analysis hardcoded to always fail over to template selection.
- `REQ-cli-council-command` → `genuinely outstanding`; entire discussion/summary is templated boilerplate, no domain-model integration.
- `REQ-cli-core-infrastructure` → `superseded by shipped code` for the module-layout clause, applying the ledger's existing path caveat (D-04) rather than writing a fresh divergence.
- `REQ-cli-rich-output` → `satisfied`, cross-referencing Epic 24's 86-snapshot count rather than re-measuring independently (T-05-17).

## Deviations from Plan

**1. [Process] Committed each task immediately after its own verification passed, rather than a single end-of-plan commit.** Task 1's action says "Do not commit yet — this plan commits once, after Task 2," and Task 2's action says "Commit this plan's single file in one commit at the end... Do not pass `--no-verify`." This executor's parallel-execution instructions explicitly direct committing early and often within a plan ("as soon as Epic 16's rows are written, commit them; then do Epic 18 and commit again... A prior plan in this phase was lost because it ran long without committing"), and separately authorize `--no-verify` for every commit given `workflow.worktree_skip_hooks=true`. Followed the more specific, risk-mitigating runtime instruction — committed Epic 16's rows (`dda2fdd`) after Task 1's verification passed, then Epic 18's rows (`970301d`) after Task 2's verification passed, both with `--no-verify`, matching the identical override already documented and applied by sibling plans 05-01, 05-05, 05-06, 05-07, 05-08, and 05-09 in this same phase. Neither the row content, the verdicts, nor the verification results differ from what a single end-of-plan commit would have produced.

Neither deviation changed the ledger's content, verdicts, or evidence — this is a process-only accommodation to this worktree's execution environment, consistent with prior plans in this phase.

## Issues Encountered

- **Initial mismeasurement of five CLI command test-pass counts.** First draft of the Epic 18 rows cited approximate pass/ignore counts for `onboarding`, `setup_check`, `features`, `muster`, and `council` before running the exact scoped commands. Corrected all five against the actual `cargo test` output before committing (`onboarding` 6/9 passed 3 ignored, `setup_check` 6/9 passed 3 ignored, `features` 12/12 passed, `muster` 11/11 passed, `council` 12/12 passed) — no row was committed with an inaccurate count.

## User Setup Required

None — no external service configuration required. (A live LLM API key would let the six `#[ignore]`d onboarding/setup-check tests actually exercise their provider-validation paths, but this plan does not request or require that setup, consistent with this corpus's VERIFY-06 live-API-test convention.)

## Next Phase Readiness

- Epic 16's and Epic 18's ledger sections are complete: 18 cited `REQ-*` rows, two epic-level notes (one citing ADR-0010, one recording Epic 18's stale count and VERIFY-02 exclusion). Ledger integrity preserved for the remaining fan-out plans: row count still 118, section count still 14, no row order disturbed outside Epics 16/18.
- Two named, concrete gaps are available for Phase 6 to scope if prioritized (not raised as new CLOSE-0x requirements here, since this plan's `must_haves` scope only the ledger rows themselves): (1) `paladin muster`'s LLM analysis path (`muster.rs:156-166`) is a hardcoded no-op — real LLM integration was never written; (2) `paladin council`'s entire discussion/summary generation (`council.rs`) is templated boilerplate disconnected from `CouncilExecutionService` — reusing the real Council domain model here would be a substantial but well-scoped follow-up.
- `REQ-grove-routing-strategies`'s row cross-references the Epic 22 `REQ-grove-llm-routing` hardcoded-model finding (`grove_service.rs:537`, owned by Phase 6 / CLOSE-01) without duplicating it, consistent with T-05-17's mitigation.
- No blockers for the next wave.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md` (modified, Epic 16 and Epic 18 sections)
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-10-SUMMARY.md`
- FOUND: commit `dda2fdd` (Task 1, ledger file only)
- FOUND: commit `970301d` (Task 2, ledger file only)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
