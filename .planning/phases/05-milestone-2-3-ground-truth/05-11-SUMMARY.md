---
phase: 05-milestone-2-3-ground-truth
plan: 11
subsystem: docs
tags: [ledger, requirements-traceability, maneuver, flow-dsl, herald, token-usage, adr-cited]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: "05-01's ledger scaffold (head notes, verdict legend, 118 PENDING-VERDICT stub rows) and its D-01 evidence bar; 05-02's ADR-0010 (Milestone 3 epic numbering, the Maneuver constructor-argument-order and CLI-form release-notes divergences); 05-04's ADR-0006 Phase 5 amendment (Herald's transcribed 80.49% measured coverage against the 95% target, gap, and Phase 15/PIPE-02 owner); 05-10's precedent for row shape and evidence bar, and its confirmation that no --strategy council/--strategy grove CLI flag exists (background for the Epic 17/17.5 CLI-shape divergence found here)"
provides:
  - "Epic 17 / 17.5 (Flow DSL, Maneuver and CLI consolidation, 11 rows) fully cited in .planning/ledgers/milestone-02-03.md, with an epic-level note recording the applied Epic 17.5 CLI-location decision and two rows citing ADR-0010 for the constructor-order and CLI-command-shape release-notes divergences"
  - "Epic 19 (Herald & Domain Type Consolidation, 5 rows) fully cited, with an epic-level note citing ADR-0010's authoritative numbering, and the Herald coverage row transcribing the amended ADR-0006's single 80.49%/95%/Phase-15-PIPE-02 figure without re-measuring"
  - "REQ-maneuver-validation (run-2-uninspected) directly inspected: Maneuver::validate()'s agent-existence/depth/agent-count checks exist and are cited, but every Maneuver::new(...) call site in the tree only exercises the happy path — zero test constructs an invalid flow and asserts Err — verdicted present, unproven"
  - "The four Epic 19 rows the run-2 ledger left uninspected — REQ-stream-chunk-complete, REQ-execution-metadata-complete, REQ-herald-formatter-autoregistration, and REQ-herald-consolidation-quality-gates — each inspected against the tree with an actual finding recorded, including the TokenUsage field-naming divergence (prompt_tokens/completion_tokens vs. the PRD's input_tokens/output_tokens) and the transcribed Herald coverage gap"
affects: [05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Same-defect-section divergence recording (extended from 05-08/05-09/05-10): both the Maneuver constructor-argument-order (REQ-maneuver-domain-model) and CLI-command-shape (REQ-maneuver-cli) release-notes divergences live in the same RELEASE_NOTES_MILESTONE_3.md 'Epic 22'-mislabeled section ADR-0010 already corrects — cited by pointing at ADR-0010's existing Code Locations entry rather than re-deriving a fresh divergence write-up for the constructor claim, with a New finding added only for the CLI-shape claim ADR-0010's Code Locations section did not itemize"
    - "Distinct-citation discipline across a shared directory: REQ-flow-dsl-syntax, REQ-flow-parser and REQ-flow-expression-ast all cite crates/paladin-battalion/src/maneuver/parser/, but each resolves to a different file within it (lexer.rs's Token enum, mod.rs's precedence-climbing parser, ast.rs's FlowExpression enum) rather than repeating one directory path three times"
    - "Doctest-as-evidence: the module-level doctest at parser/mod.rs:15-26 (containing the PRD's literal nested-grouping example) and the ExecutionMetadata doctest at herald.rs:424-450 (which .unwrap()s duration_ms immediately after calling calculate_duration(), so it would panic if the method left the field unset) are both cited as executed, passing evidence via cargo test --doc, not merely as documentation"
    - "Coverage-gate transcription, not re-derivation (same convention 05-04 established): REQ-herald-consolidation-quality-gates transcribes the amended ADR-0006's Herald figure (80.49% measured / 95% target / ~14.5pt gap / Phase 15-PIPE-02 owner) byte-identical, with an acceptance-criteria assertion that no cargo llvm-cov or cargo tarpaulin ran during this task"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md

key-decisions:
  - "REQ-maneuver-domain-model and REQ-maneuver-cli both verdicted against ADR-0010: the shipped Maneuver::new(name, agents, flow, config) signature (mod.rs:148-153) answers the constructor-order divergence ADR-0010 already cites; the CLI-command-shape divergence (no inline --flow flag on battalion run, no battalion visualize subcommand, visualization instead shipping as a separate top-level paladin maneuver visualize/validate command group requiring -c/--config rather than a positional flow string) is recorded as a New finding against the same release-notes section, since ADR-0010's Code Locations section names the constructor claim but not this CLI-shape claim."
  - "REQ-maneuver-config and REQ-maneuver-execution-service both verdicted superseded by shipped code rather than satisfied. ManeuverConfig ships without max_nesting_depth, max_parallel_branches, agent_timeout_seconds or capture_intermediate_outputs (grep across crates/ and src/ returns zero matches for all four); Maneuver::validate() hardcodes the depth-5/agent-count-30 bounds directly rather than reading them from any config. ManeuverResult.step_outputs is HashMap<String, String>, not HashMap<String, PaladinResult> as FR-6.5 specifies, losing per-step token-usage/status metadata. Both rows record the shipped, narrower surface as authoritative rather than inventing satisfaction of the PRD's fuller spec."
  - "REQ-maneuver-validation verdicted present, unproven, not satisfied, despite Maneuver::validate()'s agent-existence/depth/agent-count checks having real file:line citations. Every one of the nine Maneuver::new(...) call sites in the tree (service.rs tests, commander.rs, battalion.rs, three examples) constructs a valid Maneuver and either .unwrap()s or propagates via ?; zero test anywhere constructs an invalid flow and asserts the resulting Err. Self-reference rejection (PRD literal requirement) does not exist at all -- a flow like 'a -> a' parses and validates successfully. Per D-01, a row cannot be satisfied on the strength of its better-tested half (partial-results/error-clarity, which IS tested via test_error_strategy_ignore_errors/test_error_strategy_fail_fast) when its construction-time-rejection half has zero passing test."
  - "REQ-herald-type-consolidation verdicted satisfied as the later position on run-1 variant group 6: herald.rs re-exports PaladinResult/BattalionResult/PaladinError/TokenUsage from their single-source-of-truth modules via pub use rather than defining local placeholder types, confirmed by a zero-match grep for TODO/FIXME/placeholder. The PRD's src/application/ports/output/herald_port.rs path does not exist -- the Herald trait lives directly in herald.rs post-workspace-decomposition -- verdicted against the ledger's own D-04 path caveat, not as a fresh divergence."
  - "REQ-execution-metadata-complete records a New finding: TokenUsage's shipped fields are prompt_tokens/completion_tokens/total_tokens, not the PRD's literal input_tokens/output_tokens. This is the same TokenUsage this corpus already tracks as one of three shipped definitions needing consolidation (DEBT-05, Phase 7-8 scope) -- recorded here as a fact about the field names actually shipped, not re-adjudicated or treated as new forward work."
  - "REQ-herald-consolidation-quality-gates verdicted deferred with reason, transcribing ADR-0006's Phase 5 amendment byte-identical: Herald measures 80.49% line coverage against a 95% target, a ~14.5 point gap, owner Phase 15/PIPE-02. No cargo llvm-cov or cargo tarpaulin was invoked during this task -- the D-16/T-05-18 no-re-measurement prohibition was honored."

requirements-completed: []  # VERIFY-01/VERIFY-02 span all of plans 05-01..05-13; not individually completable until 05-13 closes the ledger out

coverage:
  - id: D1
    description: "Epic 17/17.5's 11 REQ-* rows filled to the D-01 evidence bar, with the epic-level note confirming src/cli absence, three distinct parser-directory citations, and both ADR-0010 divergence rows (constructor order, CLI command shape) cited rather than restated"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion maneuver:: -- 65/65 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion parser:: -- 26/26 passed"
      - kind: other
        ref: "cargo test --offline -p paladin-battalion --doc -- maneuver::parser -- 10/10 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion test_maneuver_strategy_explicit -- 1/1 passed"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::commands::maneuver -- 4/4 passed"
    human_judgment: true
    rationale: "Requires a human to confirm the two ADR-0010-cited divergences (constructor order, CLI command shape) are read correctly from the tree and correctly attributed to the same release-notes section ADR-0010 already corrects, that the three parser-directory rows genuinely carry three distinct citations rather than a disguised repeat, and that the present-unproven/superseded verdicts on REQ-maneuver-validation/REQ-maneuver-config/REQ-maneuver-execution-service are not overstated or understated (same class of manual check as sibling wave plans 05-05/05-09/05-10)."
  - id: D2
    description: "Epic 19's 5 REQ-* rows filled to the D-01 evidence bar, with the epic-level note citing ADR-0010's numbering, the four run-2-uninspected rows recording genuine findings, and the coverage-gate row transcribing ADR-0006 without re-measuring"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core herald:: -- 7/7 passed"
      - kind: other
        ref: "cargo test --offline -p paladin-ai-core --doc -- herald -- 8/8 passed, 3 ignored (trait-signature snippets)"
      - kind: unit
        ref: "cargo test --offline -p paladin-herald -- 70/70 passed"
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::services::herald::herald_registry -- 14/14 passed"
    human_judgment: true
    rationale: "Requires a human to confirm the TokenUsage field-naming finding (prompt_tokens/completion_tokens vs. input_tokens/output_tokens) is correctly scoped as a fact-recording rather than a re-adjudication of DEBT-05, and that the transcribed 80.49%/95%/Phase-15-PIPE-02 coverage figure is byte-identical to ADR-0006's amendment with no re-measurement having occurred (same class of manual check as sibling plan 05-09's ADR-0006 citation)."
  - id: D3
    description: "Ledger integrity preserved: exactly 118 REQ-* rows, 14 epic sections, no row inserted/deleted/reordered, no .rs/Cargo.toml/.github file touched"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-02-03.md equals 118; grep -c '^### Epic ' equals 14; git diff --stat HEAD~2 -- '*.rs' 'Cargo.toml' '.github/' empty; git log -1 --name-only shows only the ledger file"
        status: pass
    human_judgment: false

duration: ~75min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 11: Epic 17/17.5 Maneuver/Flow DSL and Epic 19 Herald consolidation ledger rows Summary

**Filled Epic 17/17.5 (Flow DSL, Maneuver, CLI consolidation — 11 rows) and Epic 19 (Herald & Domain Type Consolidation — 5 rows) in the Milestone 2-3 ledger, finding that Maneuver ships a narrower ManeuverConfig and a `String`-only `ManeuverResult.step_outputs` than the PRD specifies, its construction-time validation is never exercised by a failing test anywhere in the tree, its CLI surface diverges from both the PRD and the release notes in three concrete ways, and Herald's consolidated types, StreamChunk/ExecutionMetadata completeness and zero-config registry are all genuinely shipped and tested.**

## Performance

- **Duration:** ~75 min (dominated by cold compiles of `paladin-battalion`, `paladin-ai-core`, `paladin-herald` and the `paladin-ai` facade's `--features cli --lib` target)
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-02-03.md`)

## Accomplishments

- Filled all 11 Epic 17/17.5 rows: `REQ-flow-dsl-syntax`, `REQ-flow-parser`, `REQ-flow-expression-ast`, `REQ-maneuver-domain-model`, `REQ-maneuver-config`, `REQ-maneuver-error-strategy-v2`, `REQ-maneuver-execution-service`, `REQ-maneuver-commander-integration`, `REQ-maneuver-cli`, `REQ-flow-visualization`, `REQ-maneuver-validation`, plus an epic-level note confirming `src/cli` absence (`ls src/`) and Epic 17.5's applied CLI-location decision.
- Filled all 5 Epic 19 rows: `REQ-herald-type-consolidation`, `REQ-stream-chunk-complete`, `REQ-execution-metadata-complete`, `REQ-herald-formatter-autoregistration`, `REQ-herald-consolidation-quality-gates`, plus an epic-level note citing ADR-0010's authoritative Epic 19 numbering.
- Verdict distribution across the 16 rows: **9 `satisfied`**, **5 `superseded by shipped code`** (`REQ-maneuver-domain-model`, `REQ-maneuver-config`, `REQ-maneuver-execution-service`, `REQ-maneuver-cli`, and none in Epic 19), **1 `present, unproven`** (`REQ-maneuver-validation`), **1 `deferred with reason`** (`REQ-herald-consolidation-quality-gates`), **0 `genuinely outstanding`**.
- **Resolved the three parser-directory citations distinctly**: `REQ-flow-dsl-syntax` cites `lexer.rs:9-20` (Token enum) plus `mod.rs:122-197` (precedence-climbing grammar), `REQ-flow-parser` cites `mod.rs:78-97` (`FlowParser::parse`), `REQ-flow-expression-ast` cites `ast.rs:34-41` (`FlowExpression` enum) — three distinct files, not one directory path repeated.
- **Both ADR-0010 divergence rows resolved from the tree.** `REQ-maneuver-domain-model`: shipped `Maneuver::new(name, agents, flow, config)` (`mod.rs:148-153`) answers the constructor-order divergence ADR-0010's Code Locations section already cites against the release notes' `Maneuver::new(flow3, paladins, config)`. `REQ-maneuver-cli`: **new finding** — the release notes' own CLI example (`paladin maneuver visualize "<expr>" --format mermaid`, positional flow string) is *itself* divergent from the shipped form, which requires `-c/--config <file>`; additionally, the PRD's `battalion run --type maneuver --flow "<expr>"` inline flag and `battalion visualize --flow` subcommand do not exist — flow only comes from a YAML config file, and visualization ships as a wholly separate top-level `paladin maneuver visualize`/`validate` command group with a third, PRD-unnamed `validate` subcommand.
- **Found `ManeuverConfig` and `ManeuverResult` both narrower than their PRDs.** `ManeuverConfig` (`mod.rs:40-58`) has no `max_nesting_depth`, `max_parallel_branches`, `agent_timeout_seconds` or `capture_intermediate_outputs` field anywhere in the tree (`grep -rn` across `crates/` and `src/` returns zero matches for all four); `Maneuver::validate()` hardcodes depth-5 and agent-count-30 directly rather than reading them from config. `ManeuverResult.step_outputs` is `HashMap<String, String>` (`mod.rs:229`), not `HashMap<String, PaladinResult>` as FR-6.5 requires — per-step token-usage/status metadata is lost. Both verdicted `superseded by shipped code`.
- **Directly inspected the run-2-uninspected `REQ-maneuver-validation` row.** `Maneuver::validate()`'s agent-existence/depth/agent-count checks are real and cited, but every one of the nine `Maneuver::new(...)` call sites in the tree constructs a valid Maneuver and either `.unwrap()`s or propagates via `?` — zero test anywhere constructs an invalid flow and asserts the resulting `Err`. Self-reference rejection (a literal PRD requirement) does not exist at all: `"a -> a"` parses and validates successfully. Verdicted `present, unproven`.
- **Found Council's `paladin muster`/`REQ-battalion-error-strategy` cross-check confirms variant group 15 closed cleanly.** `REQ-maneuver-error-strategy-v2`'s `FailFast`/`ContinueParallel`/`IgnoreErrors` (`mod.rs:17-26`) is a structurally distinct enum from run-1's `FailFast`/`ContinueOnError`/`RetryThenContinue` (`crates/paladin-core/.../battalion/mod.rs:239-249`) — `grep -rn 'pub enum ErrorStrategy'` returns exactly two hits, one per crate, confirming two non-competing implementations rather than one contested type.
- **Confirmed Herald's type consolidation is complete and untainted.** `herald.rs:25-28` re-exports `PaladinResult`/`BattalionResult`/`PaladinError`/`TokenUsage` from their single-source-of-truth modules via `pub use`; a zero-match grep for `TODO`/`FIXME`/`placeholder` across `herald.rs` confirms no residue. `StreamChunk` (6 required fields + flattened metadata + builder) and `ExecutionMetadata` (8 required fields + flattened metadata + builder + `calculate_duration()`) are both complete, with `calculate_duration()`'s doctest (`herald.rs:424-450`) genuinely exercising the method — it `.unwrap()`s `duration_ms` immediately after calling it, so a broken implementation would panic the doctest.
- **New finding: `TokenUsage`'s field names diverge from the Epic 19 PRD.** Shipped fields are `prompt_tokens`/`completion_tokens`/`total_tokens` (`token_usage.rs:13-20`), not the PRD's literal `input_tokens`/`output_tokens`/`total_tokens` — recorded as a fact about the field names, scoped explicitly as the same `TokenUsage` this corpus already tracks under DEBT-05 (three shipped definitions needing consolidation, Phase 7-8), not re-adjudicated here.
- **Confirmed `HeraldRegistry: Default` auto-registers all three built-in formatters** (`herald_registry.rs:216-254`) with the manual `register()` API preserved and duplicate-key overwrite behavior documented and tested.
- **Transcribed, did not re-measure, the Herald coverage gate.** `REQ-herald-consolidation-quality-gates` records the amended ADR-0006's byte-identical figures — 80.49% measured, 95% target, ~14.5-point gap, Phase 15/PIPE-02 owner — verdicted `deferred with reason`. No `cargo llvm-cov` or `cargo tarpaulin` ran during this task.
- Ran 9 distinct scoped `cargo test`/`ls`/`grep` commands, all passing where expected (65 `maneuver::` + 26 `parser::` + 10 parser doctests + 1 commander-integration + 4 CLI-maneuver = 106 test passes for Epic 17/17.5; 7 `herald::` + 8 herald doctests + 70 `paladin-herald` crate + 14 `herald_registry` = 99 test passes for Epic 19), cited by name in the rows that rely on them.

## Task Commits

1. **Task 1: Fill Epic 17/17.5's 11 rows** — `9ed279d` (docs)
2. **Task 2: Fill Epic 19's 5 rows** — `f0fadd6` (docs)

`9ed279d` — `docs(05-11): fill Epic 17/17.5 Flow DSL and Maneuver ledger rows`
`f0fadd6` — `docs(05-11): fill Epic 19 Herald and domain type consolidation ledger rows`

_No separate plan-metadata commit — SUMMARY.md is committed by this same worktree per the parallel-execution instructions; STATE.md/ROADMAP.md updates are owned by the orchestrator after the wave merges._

**Worktree hook policy note:** this repo's pre-commit hooks (`cargo fmt`, `cargo clippy --workspace --all-targets --all-features`, both `always_run: true`) would cold-compile the entire 12-crate workspace on every commit including markdown-only ones. Per `workflow.worktree_skip_hooks=true`, `--no-verify` was used for both commits, matching plans 05-01/05-05/05-06/05-07/05-08/05-09/05-10's precedent in this phase.

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — Epic 17/17.5 section (11 rows plus one epic-level note) and Epic 19 section (5 rows plus one epic-level note): replaced all 16 `PENDING-VERDICT` stub rows with cited verdicts. No other epic section touched; row count (118) and section count (14) both verified unchanged outside Epics 17/17.5/19.

## Decisions Made

See `key-decisions` in the frontmatter for the full, citation-bearing list. Summarized:
- `REQ-maneuver-domain-model` → `superseded by shipped code`, citing ADR-0010's existing constructor-order divergence record.
- `REQ-maneuver-config` → `superseded by shipped code`; four PRD fields absent, bounds hardcoded rather than config-driven.
- `REQ-maneuver-execution-service` → `superseded by shipped code`; `step_outputs` is `String`-valued, not `PaladinResult`-valued.
- `REQ-maneuver-cli` → `superseded by shipped code`, with a new finding that even the release notes' own CLI example diverges from the shipped `-c/--config` form.
- `REQ-maneuver-validation` → `present, unproven`; validation code exists, its rejection paths are never tested.
- `REQ-herald-type-consolidation` → `satisfied`, applying the ledger's D-04 path caveat for the `herald_port.rs` location rather than writing a fresh divergence.
- `REQ-execution-metadata-complete` → `satisfied` with a new finding on `TokenUsage`'s field-name divergence, explicitly scoped to the existing DEBT-05 tracking.
- `REQ-herald-consolidation-quality-gates` → `deferred with reason`, transcribing ADR-0006's amendment verbatim with zero re-measurement.

## Deviations from Plan

**1. [Process] Committed each task immediately after its own verification passed, rather than a single end-of-plan commit.** Task 1's action says "Do not commit yet — this plan commits once, after Task 2," and Task 2's action says "Commit this plan's single file in one commit at the end... Do not pass `--no-verify`." This executor's parallel-execution instructions explicitly direct committing early and often within a plan, and separately authorize `--no-verify` for every commit given `workflow.worktree_skip_hooks=true`. Followed the more specific, risk-mitigating runtime instruction — committed Epic 17/17.5's rows (`9ed279d`) after Task 1's verification passed, then Epic 19's rows (`f0fadd6`) after Task 2's verification passed, both with `--no-verify`, matching the identical override already documented and applied by sibling plans 05-01, 05-05, 05-06, 05-07, 05-08, 05-09 and 05-10 in this same phase. Neither the row content, the verdicts, nor the verification results differ from what a single end-of-plan commit would have produced.

Neither deviation changed the ledger's content, verdicts, or evidence — this is a process-only accommodation to this worktree's execution environment, consistent with prior plans in this phase.

## Issues Encountered

None. All scoped test commands passed on first run; no mismeasurement or correction was needed before committing.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Epic 17/17.5's and Epic 19's ledger sections are complete: 16 cited `REQ-*` rows, two epic-level notes (one confirming the applied Epic 17.5 CLI-location decision, one citing ADR-0010's authoritative Epic 19 numbering). Ledger integrity preserved for the remaining fan-out plans: row count still 118, section count still 14, no row order disturbed outside Epics 17/17.5/19.
- Four named, concrete gaps are available for Phase 6 to scope if prioritized (not raised as new CLOSE-0x requirements here, since this plan's `must_haves` scope only the ledger rows themselves): (1) `Maneuver::new`'s construction-time validation rejection paths have zero test coverage anywhere in the tree; (2) self-reference rejection (`"a -> a"`) does not exist despite being a literal PRD requirement; (3) `ManeuverConfig`'s four missing fields (`max_nesting_depth`, `max_parallel_branches`, `agent_timeout_seconds`, `capture_intermediate_outputs`) would need to be added for full PRD conformance; (4) `ManeuverResult.step_outputs` would need to carry `PaladinResult` rather than `String` to preserve per-step token-usage/status metadata.
- `REQ-execution-metadata-complete`'s `TokenUsage` field-naming finding is scoped explicitly to existing DEBT-05 tracking (Phase 7-8) — not raised as new forward work here.
- No blockers for the next wave.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md` (modified, Epic 17/17.5 and Epic 19 sections)
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-11-SUMMARY.md`
- FOUND: commit `9ed279d` (Task 1, ledger file only)
- FOUND: commit `f0fadd6` (Task 2, ledger file only)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
