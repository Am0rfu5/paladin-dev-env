# Phase 2: Functional Gap Closure - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-31
**Phase:** 2-functional-gap-closure
**Areas discussed:** Scope posture vs the ledger, The `present, unproven` line, Dead `tests/unit/llm/` module, GAP-07 ↔ Phase 14 WEB-03

---

## Area selection

| Option | Description | Selected |
|--------|-------------|----------|
| Scope posture vs the ledger | SC1/SC2/SC4 already satisfied per the 2026-07-31 ledger re-verification — re-prove or trust? | ✓ |
| The `present, unproven` line | Which of the 23 items get an exerciser here, which are deferred, which belong to Phase 3 | ✓ |
| Dead `tests/unit/llm/` module | 25 test functions never compiled into any target; Epic 6 task 6.0 is `[x]` on their strength | ✓ |
| GAP-07 ↔ Phase 14 WEB-03 | ADR-0004 forbids scheduling them independently — same struct, same file | ✓ |

**User's choice:** All four.
**Notes:** Presented alongside the reframe that the Phase 1 ledger had overtaken three of Phase 2's five ROADMAP success criteria.

---

## Scope posture vs the ledger

### Q1 — Posture toward SC1/SC2/SC4

| Option | Description | Selected |
|--------|-------------|----------|
| Re-prove by execution (Recommended) | Ledger is the map, not the proof. Measured `cargo test --workspace` baseline with full provenance; one executable check per criterion. | ✓ |
| Trust the ledger, touch only residue | Ledger already re-verified 2026-07-31 at the D-19 bar; spend the whole budget on GAP-07 and the residue. | |
| Baseline run only, no per-criterion re-proof | One workspace run (what SC1 literally demands); accept SC2 and SC4 from the ledger. | |

**User's choice:** Re-prove by execution.
**Notes:** Rationale carried into CONTEXT.md D-01 — this corpus's own repeated finding is that checkbox-and-citation evidence has been wrong in both directions.

### Q2 — When measurement contradicts the ledger

| Option | Description | Selected |
|--------|-------------|----------|
| Amend the ledger in place (Recommended) | Living record; edit the row with new verdict, command and date. Sets the convention for Phases 5/7/10/13. | ✓ |
| Record corrections in a Phase 2 artifact | Phase 1's output stays immutable; a corrections file supersedes rows by ID. | |
| Amend in place, and flag drift loudly | Same as in-place, plus per-row `was: {prior verdict}` notes and a drift-count summary. | |

**User's choice:** Amend the ledger in place.
**Notes:** Rated `costly` for reversibility in CONTEXT.md D-02 — four sibling ground-truth phases inherit the convention.

### Q3 — Where deferrals are recorded

| Option | Description | Selected |
|--------|-------------|----------|
| Ledger row, `deferred with reason` (Recommended) | D-20 already defines the class; the ledger already carries one such row. No new document class. | ✓ |
| New ADR per deferral | Strongest authority, machine-readable, tops precedence per D-02. | |
| Ledger row, ADR only when contested | Default to a row; escalate to an ADR when the deferral overrides a written requirement. | |

**User's choice:** Ledger row, `deferred with reason`.
**Notes:** The escalation clause from option 3 was retained in CONTEXT.md D-03 as an exception and applied to the cancellation deferral (D-08), since D-05 does override `REQ-battalion-cancellation` as written.

### Q4 — How GAP-06 closes

| Option | Description | Selected |
|--------|-------------|----------|
| 11.5 superseded, 11.6 as a real review (Recommended) | ADR-0006's workspace number replaced the per-module 80% target by decision; 11.6 gets a written PRD-acceptance pass at the D-19 bar. | ✓ |
| Measure Garrison anyway, as advisory | Garrison-scoped figure from plan 01-09's offline path, advisory-only. | |
| Both items closed by the 11.6 review | Fold the coverage disposition into the review's findings. | |

**User's choice:** 11.5 superseded, 11.6 as a real review.
**Notes:** Rejected option 2 explicitly because an advisory number in this record gets read as a gate later — the failure ADR-0006 exists to end.

---

## The `present, unproven` line

### Q1 — `REQ-battalion-cancellation`

| Option | Description | Selected |
|--------|-------------|----------|
| Record Phalanx-only, defer the rest (Recommended) | Cancellation for three more patterns is feature work needing a cross-service contract; record and defer with a named owner. | ✓ |
| Implement across all four patterns | Honest to the requirement text; the single largest item in Phase 2. | |
| Implement for Formation only, defer graph/hierarchy | Split by difficulty — Formation is well-defined between steps; Campaign and ChainOfCommand raise semantic questions. | |

**User's choice:** Record Phalanx-only, defer the rest.
**Notes:** Because this overrides a written requirement, it triggers D-03's exception and gets its own ADR (CONTEXT.md D-08). No forward owner was named during discussion — flagged in CONTEXT.md's deferred section for the planner.

### Q2 — Closing SC3 (Battalion → Herald)

| Option | Description | Selected |
|--------|-------------|----------|
| One end-to-end test per Herald, Formation-driven (Recommended) | Real `FormationExecutionService` run → `BattalionResult` → all three Heralds, asserting SC3's five field requirements plus a partial-failure case. | ✓ |
| Full matrix: every Herald × every Battalion pattern | Twelve combinations; would give the four patterns a shared harness. | |
| Extend existing unit tests, no new integration file | Hand-construct a realistic `BattalionResult` and assert the same fields. | |

**User's choice:** One end-to-end test per Herald, Formation-driven.
**Notes:** Option 3 was rejected on the grounds that task 7.13's own note names "needs Battalion execution setup" as the gap — hand-construction is precisely what it declared insufficient.

### Q3 — The Phase 2 / Phase 3 boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Existence in Phase 2, depth in Phase 3 (Recommended) | Phase 2 proves each pattern has a passing non-ignored exerciser and the Phalanx perf claims hold; Phase 3 owns coverage, the ignored tests and MCP failure modes. | ✓ |
| Pull the four `#[ignore]`d tests into Phase 2 | They sit on the Commander error path, squarely GAP-04 territory. | |
| Phase 2 writes the shared failing-mock, Phase 3 uses it | Build the `Send + Sync` failing mock here; Phase 3 inherits the harness. | |

**User's choice:** Existence in Phase 2, depth in Phase 3.
**Notes:** Option 3's observation was retained as a note to Phase 3 in CONTEXT.md D-07 — the failing-mock is a shared asset three registers have asked for, and Phase 3 should build it as shared rather than local.

### Q4 — The two `genuinely outstanding` clusters

| Option | Description | Selected |
|--------|-------------|----------|
| CLI tests yes, provider-switching yes, CI config no (Recommended) | 13.4-13.6 and 7.10-7.12 are missing functional tests inside Milestone 1; 7.14 is a CI change belonging to Phase 15. | ✓ |
| Take all four clusters including CI config | Closes Epic 6 outright; three lines of YAML. | |
| CLI tests only; defer the Epic 6 cluster | Smallest Phase 2; the whole provider cluster waits on Phase 5's VERIFY-06. | |

**User's choice:** CLI tests yes, provider-switching yes, CI config no.
**Notes:** 7.14 deferred to Phase 15 partly because VERIFY-06 has not yet decided whether a keyless run fails loudly or skips cleanly — which is what such a job would encode.

---

## Dead `tests/unit/llm/` module

### Q1 — Disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Wire in and fix, timeboxed with a recorded fallback (Recommended) | Real mockito HTTP-level tests (401/429, streaming) covering paths the live 67 tests miss; fallback to deletion if breakage is structural. | ✓ |
| Wire in and fix, no escape hatch | Commit to landing all 25 however deep the breakage goes. | |
| Delete now, record as superseded | 853 lines of never-compiled source; cleanest for SC1. | |

**User's choice:** Wire in and fix, timeboxed with a recorded fallback.
**Notes:** Option 3 was weighed against Phase 3's QUAL-02, which names `deepseek_adapter.rs` at 15.02% — deleting would mean rewriting the same coverage there.

### Q2 — The fallback rule

| Option | Description | Selected |
|--------|-------------|----------|
| Per-file, by failure kind (Recommended) | Mechanical breakage → fix; structural breakage → delete those tests with a per-test recorded reason. A rule about the nature of the failure, not a clock. | ✓ |
| One plan-task budget, all-or-nothing | Hard stop; delete the directory if not green. | |
| Fix what compiles, quarantine the rest | Delete only individual tests that still fail after a genuine attempt. | |

**User's choice:** Per-file, by failure kind.
**Notes:** Chosen so two easily-repaired files are not discarded because a third was hard. Option 3 was noted to reintroduce the judgement call the rule was meant to remove.

### Q3 — Sweep for other orphaned test source

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — sweep once, record findings (Recommended) | Cross-check `tests/` against `[[test]]` targets and barrel `mod.rs` files; findings become ledger rows; repair beyond the LLM module is a separate call. | ✓ |
| Yes — sweep and fix everything found | Repair every orphan found. | |
| No — fix the known one only | Other orphans surface when Phases 5/7/10/13 write their ledgers. | |

**User's choice:** Yes — sweep once, record findings.
**Notes:** Option 2 rejected as committing the phase to unbounded work sight-unseen — the same trap the Q2 fallback rule exists to avoid.

---

## GAP-07 ↔ Phase 14 WEB-03

### Q1 — How the two land

| Option | Description | Selected |
|--------|-------------|----------|
| Pull WEB-03 into Phase 2's GAP-07 (Recommended) | Both fields in one change, each construction site touched once; Phase 14 records WEB-03 as satisfied. | ✓ |
| GAP-07 alone; Phase 14 rebases | Each phase keeps to its own requirements; accepts the merge cost ADR-0004 named. | |
| GAP-07 lands the struct shape for both | Add a deliberately-failing `#[ignore]`d correspondence test for Phase 14 to flip. | |

**User's choice:** Pull WEB-03 into Phase 2's GAP-07.
**Notes:** Option 3 was rejected as contradicting SC1's zero-failures and planting an `#[ignore]` on purpose — the pattern this milestone is closing.

### Q2 — What `supports_tool_calling` should report

Grounded by live verification during the discussion: `LlmRequest` (`llm_port.rs:464`) has no tools field; neither the OpenAI nor Anthropic adapter sends `tools` or parses `tool_calls`; both nevertheless report `true`.

| Option | Description | Selected |
|--------|-------------|----------|
| All three `false`, with a correspondence test (Recommended) | The flag describes what the adapter does, not what the vendor offers. Two asserting tests flip; a new test asserts the correspondence WEB-03's SC3 demands. | ✓ |
| Keep `true`, redefine the field's meaning | Document it as "the provider supports tool calling"; no test flips. | |
| Split into provider-supports vs adapter-implements | Two fields, both facts expressible; a landing place if tool calling is later built. | |

**User's choice:** All three `false`, with a correspondence test.
**Notes:** Option 3 rejected as pre-building for WEB-04's build-or-withdraw outcome, which is explicitly Phase 14's to choose.

### Q3 — Reach of the `temperature_range` change

| Option | Description | Selected |
|--------|-------------|----------|
| Real adapters populate; the rest take `None` (Recommended) | OpenAI/Anthropic `[0.0, 1.0]`, DeepSeek `[0.0, 2.0]`; all ten-plus other construction sites take `None`, ADR-0004's named fallback. | ✓ |
| Every site declares a range explicitly | Makes every fixture's assumption visible. | |
| Add `#[non_exhaustive]` and default the field | Solves the class rather than the instance. | |

**User's choice:** Real adapters populate; the rest take `None`.
**Notes:** Option 2 rejected because it would ship the `None` fallback path untested despite it being the contract for every future adapter. Option 3 rejected as a breaking change for downstream constructors — a Phase 4 / REL-01 version decision.

### Q4 — Sequencing the three ADR edits

| Option | Description | Selected |
|--------|-------------|----------|
| Ports change first, as a tracer (Recommended) | Widest blast radius first; proves the phase can safely edit a published ports type. Mirrors plan 01-01's tracer shape. | ✓ |
| Cheapest first: Formation, rename, then ports | Two of five SC5 sub-claims true early. | |
| All three in one plan | One requirement, one success criterion, one commit. | |

**User's choice:** Ports change first, as a tracer.
**Notes:** Option 2 rejected because it puts the widest change last, when there is least room to absorb surprises.

---

## Claude's Discretion

- The renamed identifier for the `citadel.rs:280` struct (`BattalionCheckpointConfig` was Phase 1's example, not a locked choice) — ADR-0001's field-set and serde constraints bind.
- Plan decomposition and count beyond D-16's tracer-first rule, including whether the baseline run is its own plan or the tracer's first task.
- Whether ROADMAP.md's Phase 2 success-criteria wording is amended at source now the ledger has overtaken its premise.
- How the GAP-05 finding (SC1's named failing test already passes) is restated, and where.
- `REQ-provider-error-mapping`'s dead `LlmProviderError` conversion path — fold into the sweep's findings or leave to Phase 3.
- Whether the D-12 sweep also covers `benches/` and `examples/`.

## Deferred Ideas

- Battalion-wide cancellation for Formation, Campaign and ChainOfCommand — needs a cross-service cancellation contract; recorded via ADR (D-08), no forward owner named yet.
- Epic 6 task 7.14, CI configuration for `live-api-tests` — Phase 15 (PIPE), blocked in substance on Phase 5's VERIFY-06.
- The four `#[ignore]`d, empty-bodied Commander error tests — Phase 3 (QUAL-04), together with the shared failing-mock harness.
- Repairing anything the D-12 sweep finds beyond `tests/unit/llm/`.
- `REQ-provider-error-mapping`'s dead conversion path.
- WEB-04 — whether Paladin builds or withdraws LLM tool calling — stays in Phase 14.
- Raising coverage to ADR-0006's 84% floor and the 0%-coverage file list — Phase 3 (QUAL-01/QUAL-02).
