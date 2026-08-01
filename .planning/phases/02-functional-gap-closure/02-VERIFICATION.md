---
phase: 02-functional-gap-closure
verified: 2026-08-01T15:46:49Z
status: gaps_found
score: 4/5 ROADMAP success criteria fully verified (1 partially failed); 6/7 GAP requirements fully verified (1 partially failed, same root cause)
behavior_unverified: 0
overrides_applied: 0
gaps:
  - truth: "The Table Herald renders a Battalion result whose Paladin name contains multi-byte UTF-8 without panicking (02-04-PLAN.md must_haves; feeds ROADMAP SC3 and GAP-03)"
    status: failed
    reason: "TableHerald::truncate_text (crates/paladin-herald/src/table_herald.rs:93-100) truncates by byte index (&text[..N]) rather than char boundary. Reproduced directly: a multi-byte Paladin name whose byte length exceeds the default max_column_width (60) and whose truncation cut point does not land on a char boundary panics with 'byte index N is not a char boundary'. This is reachable from format_battalion_result's per-Paladin row rendering (table_herald.rs:203, Cell::new(self.truncate_text(&name))), a call site plan 02-04 newly wired onto real Paladin names. The plan's own added test (test_table_herald_renders_multibyte_paladin_name) uses a 21-byte name, well under the 60-byte threshold, so it never exercises the truncation branch and the bug ships green. Independently confirmed by 02-REVIEW.md CR-02 (Critical)."
    artifacts:
      - path: "crates/paladin-herald/src/table_herald.rs"
        issue: "truncate_text (lines 93-100) byte-slices &str; panics on multi-byte UTF-8 input longer than max_column_width. truncate_text itself pre-dates Phase 2 (commit 3d2dedb7, 2026-01-26), but plan 02-04 (commit eea94bf) added the first call site that applies it to real, execution-service-sourced Paladin names in the Battalion rendering path."
    missing:
      - "Fix truncate_text to truncate on a char boundary (e.g. via text.chars().take(n).collect()), not a byte index."
      - "Add a test with a multi-byte string long enough to force truncation (30+ multi-byte characters) asserting no panic and no replacement-character corruption — the existing multibyte test only covers the sub-threshold case."
  - truth: "Every GAP requirement this phase completed has its REQUIREMENTS.md checkbox and status-table entry updated to reflect completion, matching the precedent Phase 1 set for its own RECON items (all checked [x] on completion)"
    status: failed
    reason: "REQUIREMENTS.md still shows GAP-01, GAP-02, GAP-04, GAP-05, GAP-06 and GAP-07 as unchecked ([ ]) with the bottom-of-file status table marking all six 'Pending', even though every plan closing them is complete, the phase's own ledger (milestone-01.md) records all seven as satisfied/deferred-with-reason, and ROADMAP.md's Phase 2 entry is checked complete. Only GAP-03 was checked ([x], commit a5f8c27). This is a requirements-traceability document going stale relative to actually-completed work, not a functional defect — the underlying capabilities are independently verified working in this report."
    artifacts:
      - path: ".planning/REQUIREMENTS.md"
        issue: "Lines 197, 204, 214, 220, 224, 228 (GAP-01/02/04/05/06/07) remain '- [ ]'; lines 3803-3809's status table marks the same six 'Pending'."
    missing:
      - "Check off GAP-01, GAP-02, GAP-04, GAP-05, GAP-06, GAP-07 in REQUIREMENTS.md and update the status table to 'Complete', citing the ledger rows and plan SUMMARYs this report and 02-09-SUMMARY.md already name as evidence."
---

# Phase 2: Functional Gap Closure Verification Report

**Phase Goal:** Every Milestone-1 functional requirement is either working and tested, or explicitly deferred with a recorded reason — and the types in code match the Phase 1 decisions.
**Verified:** 2026-08-01T15:46:49Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Already-Established Gates (not re-run, per orchestrator note)

- Post-merge build gate: `make build` exit 0 — established by orchestrator.
- Post-merge/cross-phase regression gate: `make test` exit 0 (14 suites, 1508 passed, 0 failed, 4 ignored, run twice at two commits) — established by orchestrator.
- Pre-commit hooks (`cargo fmt --check`, `cargo clippy --workspace -D warnings`) — established by orchestrator.
- ROADMAP/ledger acceptance greps from plan 02-09 Task 3 — established by orchestrator.

This report does not repeat those. Below is independent, this-session verification of the phase-specific claims — including the `cargo test --workspace` form ROADMAP success criterion 1 names literally, which is a different scope than `make test`.

## Goal Achievement

### Observable Truths (ROADMAP Success Criteria, current/amended text)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `cargo test --workspace` passes with zero failures, including `test_auto_selects_campaign_for_workflow_keywords` | ✓ VERIFIED | Ran `cargo test --workspace` myself on current HEAD (`164ff00`): 35 test-result groups, **2860 passed, 0 failed, 126 ignored** (higher than plan 02-01's 2790-passed baseline because later Phase 2 plans added tests). `test commander::tests::test_auto_selects_campaign_for_workflow_keywords ... ok` confirmed in the output. |
| 2 | A developer can run a Chain of Command battalion from an example and watch the commander select specialists, survive a specialist failure through fallback, and return a synthesized answer — tests covering all four delegation strategies | ✓ VERIFIED | `examples/chain_of_command_delegation.rs` exists. All four delegation-strategy test modules (`automatic_delegation_tests`, `broadcast_delegation_tests`, `round_robin_delegation_tests`, `custom_delegation_tests`) present and passing in the same `cargo test --workspace` run. |
| 3 | A Battalion result rendered through JSON, Markdown and Table Heralds shows Battalion name, ID and type, per-Paladin results in execution order, aggregated token usage, and partial results when something failed | ⚠️ PARTIALLY FAILED | Happy-path and partial-results rendering through all three Heralds is directly proven: `tests/integration/battalion_herald_end_to_end_test.rs`'s two tests (`test_formation_result_through_json_markdown_table_heralds`, `test_formation_partial_results_through_all_three_heralds`) drive a real `FormationExecutionService` and pass (`cargo test --test lib -- integration::battalion_herald_end_to_end_test` → 2 passed). JSON Herald confirmed to emit `strategy_used`, `total_tokens`, `node_errors` (`json_herald.rs:149-153`). **However**, the Table Herald's own `truncate_text` (`table_herald.rs:93-100`) panics on realistic multi-byte Paladin names — reproduced directly (see Gaps below). The criterion holds for the tested/typical case but is falsified for a class of real input the phase's own plan committed to handling. |
| 4 | Commander execution returns a normalized result carrying strategy used, per-Paladin timings, success/failure counts and preserved errors, and writes telemetry metadata to `metadata_output_dir` when configured | ✓ VERIFIED | `strategy_used`/`strategy_selection_reasoning`/`strategy_selection_time_ms` set at `commander.rs:480` and documented at `:253-255,271-272`; `export_metadata` gated on `metadata_output_dir` at `commander.rs:870-881`. `cargo test -p paladin-battalion commander` passes as part of the full-workspace run above. |
| 5 | Shipped types match Phase 1 ADRs: duplicate `BattalionConfig` in `citadel.rs` resolved (renamed `BattalionCheckpointConfig`, ADR-0001), single-Paladin Formation executes instead of failing validation (ADR-0003) | ✓ VERIFIED | Read directly: `citadel.rs:233,257,281,284,446,460,663` all use `BattalionCheckpointConfig`. `formation.rs:99-118`: `validate()` rejects only `self.paladins.is_empty()`, doc comment and error message both state "at least 1 Paladin". |

**Score:** 4/5 ROADMAP success criteria fully verified (criterion 3 partially failed on a demonstrated, reproducible defect).

### GAP Requirement Coverage (cross-referenced against REQUIREMENTS.md)

All seven of this phase's declared requirement IDs (GAP-01 through GAP-07) are present in `.planning/REQUIREMENTS.md` § "Gap closure (GAP)" (lines 197-234) — every ID is accounted for as an entry. Their checkbox/status-table state is a separate finding (see Gaps below).

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| GAP-01 | 02-01, 02-03, 02-06, 02-07 | Chain of Command end-to-end, all four delegation strategies | ✓ SATISFIED | 4 delegation-strategy test modules passing; `tests/unit/llm/` reactivated (41 passed via `cargo test --test unit -- llm`, independently re-run); CLI `paladin_execution_test`/`formation_execution_test` suites passing (99 passed via `cargo test --features cli --test cli`, independently re-run) |
| GAP-02 | 02-01, 02-06, 02-07 | Battalion integration/perf tests for all four patterns, provider-switching test | ✓ SATISFIED | `tests/integration/provider_switching_test.rs` present, declared in `tests/integration/mod.rs`, passes; CLI Phalanx suite passing |
| GAP-03 | 02-04, 02-05 | Herald on Battalion execution path with real per-Paladin aggregation | ⚠️ PARTIALLY SATISFIED | Formation aggregation (`per_paladin_times`/`per_paladin_tokens`/`total_tokens`/`node_errors`) and JSON/Markdown/Table happy-path rendering independently confirmed working via a real execution service. Table Herald's multi-byte truncation defect (see gap above) means "shows...per-Paladin results" is not unconditionally true — it panics instead of showing results for a realistic input class |
| GAP-04 | 02-01 | Commander normalized result + telemetry export | ✓ SATISFIED | `commander.rs:480,870-881` read directly, confirmed as described above |
| GAP-05 | 02-01 | `test_auto_selects_campaign_for_workflow_keywords` passes, all four keyword families correct | ✓ SATISFIED | Confirmed passing directly in this session's `cargo test --workspace` run |
| GAP-06 | 02-08 | Garrison PRD-acceptance review, coverage disposition | ✓ SATISFIED | `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md` exists (199 lines), 50-row criterion table, no coverage percentage, ADR-0006 cited for task 11.5. Independently spot-checked: `PaladinError::GarrisonRequired` confirmed to have zero construction sites outside its own definition/test (`grep -rn "GarrisonRequired" src crates` → only `paladin_error.rs:54,78,111` and one unrelated match arm in `conclave_execution_service.rs:364`) |
| GAP-07 | 02-02, 02-03, 02-09 | Phase 1 ADRs applied in code: temperature_range, tool-calling honesty, Formation minimum, citadel rename | ✓ SATISFIED | `temperature_range` present on `ProviderCapabilities` and all three adapters (confirmed via passing tests); `BattalionCheckpointConfig` and Formation single-Paladin acceptance confirmed by direct code read (SC5 above); ADR-0007 mints the cancellation-deferral decision and parses cleanly under the project's ADR parser (confirmed: `node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0007-battalion-cancellation-deferral.md` exits 0 with the same shape as ADR-0004) |

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/paladin-ports/src/output/llm_port.rs` | `temperature_range` field, `Eq`-free derive | ✓ VERIFIED | Confirmed present |
| `crates/paladin-core/src/platform/container/battalion/formation.rs` | Single-Paladin acceptance | ✓ VERIFIED | Read directly, lines 99-118 |
| `crates/paladin-core/src/platform/container/citadel.rs` | `BattalionCheckpointConfig` rename | ✓ VERIFIED | 7 occurrences confirmed |
| `crates/paladin-battalion/src/formation_service.rs` | Per-Paladin aggregation | ✓ VERIFIED | Exercised end-to-end by `battalion_herald_end_to_end_test.rs` |
| `crates/paladin-herald/src/json_herald.rs` | Battalion strategy/tokens/errors in JSON | ✓ VERIFIED | Lines 149-153 confirmed |
| `crates/paladin-herald/src/markdown_herald.rs` | Battalion strategy/tokens/failures in Markdown | ✓ VERIFIED | Exercised by passing end-to-end test |
| `crates/paladin-herald/src/table_herald.rs` | Real per-Paladin rows, reads its argument | ⚠️ VERIFIED WITH DEFECT | Reads its argument correctly (litmus test passes); `truncate_text` panics on long multi-byte names (see Gaps) |
| `tests/integration/battalion_herald_end_to_end_test.rs` | Formation-driven 3-Herald proof | ✓ VERIFIED | 372 lines, both tests pass, no hand-built `BattalionResult {` literal (`grep -c` → 0) |
| `tests/integration/provider_switching_test.rs` | Provider-switching proof | ✓ VERIFIED | Declared in barrel, passes |
| `tests/unit/llm/` (3 files, 25 tests) | Reactivated LLM tests | ✓ VERIFIED | 41 passed via `cargo test --test unit -- llm` (independently re-run this session) |
| `tests/cli/helpers.rs` + 5 reactivated CLI suites | CLI test cluster unblocked | ✓ VERIFIED | 99 passed via `cargo test --features cli --test cli` (independently re-run this session) |
| `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md` | Epic 2 PRD-acceptance review | ✓ VERIFIED | 199 lines, no coverage %, ADR-0006 cited |
| `.planning/phases/02-functional-gap-closure/02-test-wiring-sweep.md` | D-12 sweep record | ✓ VERIFIED | 196 lines, "Findings requiring a decision" section present, names 4 still-commented CLI files as an honest open finding |
| `.planning/decisions/0007-battalion-cancellation-deferral.md` | Cancellation deferral ADR | ✓ VERIFIED | Parses under the project's ADR parser (exit 0), all 7 sections present, `Code Conformance: conforms` |
| `.planning/ledgers/milestone-01.md` amendments | In-place ledger amendment (D-02) | ✓ VERIFIED | "Phase 2 amendments" note present, `0007` cited |
| `.planning/ROADMAP.md` Phase 2 criteria correction | Stale-premise removal | ✓ VERIFIED | `grep -c 'which fails today'` → 0; `git show 0dd3ae9 -- .planning/ROADMAP.md` confirms the diff is scoped to only the Phase 2 section |

### Key Link Verification

| From | To | Via | Status |
|---|---|---|---|
| `paladin_builder.rs` | `llm_port.rs` | `get_capabilities().temperature_range` | ✓ WIRED |
| `formation_service.rs` | `battalion/mod.rs` | `per_paladin_tokens`/`per_paladin_times` aggregation | ✓ WIRED |
| `table_herald.rs` | `battalion/mod.rs` | reads `result.paladin_results`/`total_tokens`/`node_errors` | ✓ WIRED (but see the `truncate_text` defect on the name it reads) |
| `battalion_herald_end_to_end_test.rs` | `formation_service.rs` | drives a real `FormationExecutionService` | ✓ WIRED |
| `citadel.rs` | `file_citadel.rs` | `BattalionCheckpointConfig` persisted shape unchanged | ✓ WIRED |
| `02-09` ledger amendments | `02-01` through `02-08` SUMMARYs | cited verdicts/commands | ✓ WIRED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Workspace test suite, exact form ROADMAP SC1 names | `cargo test --workspace` | 35 groups, 2860 passed, 0 failed, 126 ignored | ✓ PASS |
| Named test SC1 requires | (in above run) `commander::tests::test_auto_selects_campaign_for_workflow_keywords` | `... ok` | ✓ PASS |
| LLM unit-test reactivation | `cargo test --test unit -- llm` | 41 passed, 0 failed, 0 ignored | ✓ PASS |
| CLI test cluster reactivation | `cargo test --features cli --test cli` | 99 passed, 0 failed, 0 ignored | ✓ PASS |
| Formation→Herald end-to-end proof | `cargo test --test lib -- integration::battalion_herald_end_to_end_test` | 2 passed, 0 failed | ✓ PASS |
| ADR-0007 parses like its siblings | `node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0007-battalion-cancellation-deferral.md` | exit 0, same shape as ADR-0004 | ✓ PASS |
| Table Herald multi-byte truncation, realistic length | standalone repro of the exact `truncate_text` logic against a 106-byte multi-byte string | panics: `byte index 57 is not a char boundary; it is inside 'ュ'` | ✗ FAIL (see Gaps) |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| `crates/paladin-herald/src/table_herald.rs` | 93-100 | Byte-index string slicing on user/domain-controlled multi-byte text | 🛑 Blocker | Panics in a Herald formatter, which per the project's own convention (`rust.instructions.md`: "avoid panics in library code") should never happen; independently confirmed reachable via the new call site plan 02-04 added. Matches `02-REVIEW.md` CR-02 exactly. |
| `.planning/REQUIREMENTS.md` | 197,204,214,220,224,228 + 3803-3809 | Stale requirement-tracking checkboxes/status table for 6 of 7 completed GAP requirements | ⚠️ Warning | Documentation-only; does not affect shipped behavior, but breaks the traceability convention Phase 1 itself established (all RECON items checked on completion) |

Note on `02-REVIEW.md`'s other Critical finding (CR-01, `openai/adapter.rs:226` `user_prompt.context` vs `.query`): confirmed by direct git history (`git log` shows the line unchanged since commit `240eb1f`, 2026-05-19) and by diffing both of plan 02-02's commits (`fff7a80`, `a2cc1c5`) against that file — neither touches the message-building code at line 226, only `get_capabilities`. This bug pre-dates Phase 2 and is outside this phase's file-touch scope; it does not bear on any Phase 2 must-have or ROADMAP success criterion and is correctly out of scope for this verification.

### Requirements Coverage

Covered in the "GAP Requirement Coverage" table above. No orphaned requirements found: every GAP-* ID named in a Phase 2 plan's frontmatter has a corresponding REQUIREMENTS.md entry, and no REQUIREMENTS.md GAP-* entry lacks a claiming plan.

### Human Verification Required

None. Both of this phase's human checkpoints (plan 02-01 Task 3, plan 02-08 Task 2) were already approved during execution, and all remaining truths were verifiable by direct code reading, direct test execution, and direct reproduction.

### Gaps Summary

Two gaps, one functional and one documentation-only:

1. **Table Herald multi-byte truncation panic (BLOCKER).** `TableHerald::truncate_text` byte-slices `&str` without checking char boundaries. This is a pre-existing function (2026-01-26), but plan 02-04 wired its first call site onto real, execution-service-sourced Paladin names (`table_herald.rs:203`), and the plan's own `must_haves` explicitly commits to "renders a Paladin name containing multi-byte UTF-8 without panicking." I reproduced the panic directly against the exact function logic with a 106-byte multi-byte name whose truncation cut point lands mid-character. The test the plan added (`test_table_herald_renders_multibyte_paladin_name`) uses a 21-byte name that never reaches the truncation branch, so the defect ships with a green suite. This is exactly the class of gap goal-backward verification exists to catch — a passing test suite masking a real, demonstrated defect. Independently corroborated by the already-committed `02-REVIEW.md` (CR-02, Critical).

2. **REQUIREMENTS.md tracking staleness (WARNING).** GAP-01, GAP-02, GAP-04, GAP-05, GAP-06 and GAP-07 remain unchecked in `.planning/REQUIREMENTS.md` and its status table still reads "Pending" for all six, despite the ledger, ROADMAP and every plan SUMMARY recording them complete. Only GAP-03 was checked off. This breaks the precedent Phase 1 itself set (every RECON-* item checked `[x]` on completion) and should be corrected so the requirements-traceability document matches the actually-shipped state, but it is a documentation-only issue with no bearing on shipped behavior.

Everything else in this phase — the temperature/tool-calling tracer (GAP-07), the Formation/citadel ADR application (GAP-07/SC5), the Formation aggregation and JSON/Markdown Herald rendering (GAP-03 happy path), the Commander normalization and telemetry (GAP-04/SC4), the LLM and CLI test reactivations (GAP-01/GAP-02), the provider-switching test (GAP-02), the honest Garrison PRD review with two genuinely-surfaced new gaps (GAP-06), and the closing sweep/ADR-0007/ledger-amendment/ROADMAP-correction record (GAP-01 through GAP-07) — is independently verified working in this session, not merely asserted by the SUMMARYs.

---

_Verified: 2026-08-01T15:46:49Z_
_Verifier: Claude (gsd-verifier)_
