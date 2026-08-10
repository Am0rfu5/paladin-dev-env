# Requirements: Paladin

**Defined:** 2026-07-30 (ingest run 1 of 5 — source set `.project/Milestone_1-MVP`, 36 docs)
**Last merge:** 2026-07-30 (**ingest run 5 of 5 — FINAL. THE INGEST IS COMPLETE.**
`.project/Milestone_9-Classic-Orchestrator-Completion` +
`.project/Milestone_10-CI-Hardening-Release-Automation` +
`.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
`.project/Deferred-QA-CICD-Completion` + `.project/project-management`, 46 docs)
**Core Value:** A Rust developer can compose and run multi-agent workflows against any supported
LLM provider through stable port abstractions — without their own domain code depending on a
provider, transport, or storage implementation.

## How to read this file

Paladin is **brownfield at v0.7.0**, and this planning corpus is a **historical record of twelve
shipped milestones plus a verified-defect and deferred-work forward scope** — not a greenfield
plan and not a backlog.

The ingest is complete. `.project/` holds **263** `.md` files: **199** classified across five runs
(188 prose documents plus 11 task lists that earlier manifests included) and the remaining **64**
`tasks-*.md` covered deterministically by `intel/task-completion-state.md`, which counts literal
GFM checkboxes rather than relying on a classifier. 188 + 75 = 263 and 188 + 11 = 199 — **every
document is covered by one route or the other.** Across that corpus 7,511 of 8,053 task-list items
are checked (93%), and the shipped tree is *ahead of* even that figure in several places. So this
file separates several different things that are all called "requirements":

| Section | What it holds |
|---|---|
| **v1 Requirements** | Forward scope. Milestone 1 close-out: 25 requirements (Phases 1-4). Milestone 2-3 close-out: 9 (Phases 5-6). Milestone 4-6 close-out: 12 (Phases 7-8). Milestone 7-8 close-out: 16 (Phases 9-11). Milestone 9-12 + Deferred-QA close-out: 25 (Phases 12-16). **87 total**, each mapped to exactly one phase. |
| **Competing variants** | 30 variant groups / 60 entries preserved **unmerged** from conflicting PRDs across all five runs. No winner is picked here. |
| **Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope. |
| **Milestone 2-3 as-shipped ledger** | All 118 run-2 requirement IDs, with status. Not forward scope. |
| **Milestone 4-6 as-shipped ledger** | All 115 run-3 requirement IDs, with status. Not forward scope. |
| **Milestone 7-8 as-shipped ledger** | All 86 run-4 requirement IDs, with status. Not forward scope. |
| **Milestone 9-12 as-shipped ledger** | All 120 run-5 requirement IDs, with status. Not forward scope. |
| **v2 Requirements** | Acknowledged and deferred. Not in the current roadmap. |

**Nine of the twelve milestones are at or above 98% by checkbox**, and every one of them is
corroborated or exceeded by the tree. The forward scope in this file exists because five runs of
direct code verification found a small number of things that are genuinely broken, genuinely
unbuilt, or genuinely undecided — not because the milestones are unfinished.

**Two unrelated uses of "v1/v2" — do not confuse them:**

- `## v1 Requirements` / `## v2 Requirements` = **release scope** (current milestone vs deferred).
- An ID *suffix* like `REQ-temperature-range-v1` / `-v2` / `-v3` = **competing variant** of the
  same scope from two or three different source PRDs. All are live; none has been chosen.

**ID provenance.** `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` are Milestone-1 close-out IDs;
`VERIFY-*` and `CLOSE-*` are Milestone 2-3 close-out IDs; `ARCH-*` and `DEBT-*` are Milestone 4-6
close-out IDs; `SEC-*`, `HARD-*` and `FACADE-*` are Milestone 7-8 close-out IDs; `SUPPLY-*`,
`ORCH-*`, `WEB-*`, `PIPE-*`, `DEFER-*` and `DOCS-*` are Milestone 9-12 + Deferred-QA close-out IDs.
Each cites the ingested `REQ-*` IDs it derives from. `REQ-*` IDs are preserved verbatim from
`.planning/intel/requirements.md`. **Seventeen prefixes are now spent** — eleven before run 5 and
six added by it. There is no run 6; the ingest is closed.

**Ledger verdicts are component-level file evidence, not per-criterion audits.** A `Shipped`
verdict means the named artefact exists in the tree at the cited path. Confirming each PRD
acceptance criterion against that artefact is forward work (RECON-01 / VERIFY-01 / ARCH-01 /
HARD-01 / ORCH-01), not something this file claims to have done. The Milestone 7-8 ledger was the
best-evidenced of the first four: 18 verified-shipped rows, 6 verified-open items, a ~~14-row~~
**Corrected (dated 2026-08-08, HARD-01):** 13-row (`sed -n '365,381p'
.planning/intel/code-verification.md | grep -c '^|'` → 15 lines = 1 header + 1 separator + 13 data
rows) superseded-by-outcome table and 2 favourable contradictions were checked directly against
`Cargo.toml`, `deny.toml`, `.cargo/audit.toml`, workflow contents and grep counts during ingest
run 4, and are recorded in `intel/code-verification.md`. Run 4 is also the only run whose corpus
contains a document that **audits itself against the tree** —
`facade-cleanup-RECONCILIATION-2026-06-04.md` — and its claims are corroborated almost without
exception.

**The Milestone 9-12 ledger is larger still**: 37 verified-shipped rows, 8 verified-open findings,
one **correction to a prior run's verification**, and a per-block checkbox analysis, all read from
the tree during ingest run 5. Its distinguishing feature is a new verdict class — `Shipped, one
acceptance criterion false` — earned by Milestone 10, which is recorded 100% complete, ships every
file, job, target and ruleset it promised, and nonetheless fails its own Epic 2 §8 success metric
because a superseded CI job was never deleted.

**Open checkbox counts are not a backlog — this is now settled across all five runs.**
`intel/task-completion-state.md` records 542 open items across 75 task lists.
`intel/code-verification.md` proves the two largest concentrations (Conclave 129, Sanctum 111) are
**shipped**. The pattern across the five runs is complete and consistent:

| Run | What the counts did |
|---|---|
| 1-2 | **Understated** shipped reality — Conclave 129 open and shipped, Sanctum 111 open and shipped |
| 3 | First **accurate** count (Milestone 4's 20, corroborated by zero `#[deprecated]` annotations) **and** first **overstatement** (CLI isolation fully checked, three dependencies still unconditional) |
| 4 | Milestone 8's three **contradicted outright** — Epics 2 and 3 both complete, Epic 3 beyond its own scope |
| 5 | Milestone 12's three **vacuous** (Task 0.0 feature-branch scaffolding, while the Epic 5 code ships) and project-management's one **nonexistent** (a `- [ ] 1.1 Create template` formatting example inside a template file) |

**Of the 542 open checkboxes, exactly one block survives verification as genuinely open work:
Milestone 11's 26 content-currency items** — and even those are "update in-place" tasks against
fourteen files that all exist, so they must be settled by content rather than by presence
(DOCS-01). Only blocks that `code-verification.md` explicitly lists as "Not yet verified" are
carried here, labelled *Unverified candidate* — never converted into requirements.

**Nothing is locked, and that is now the final corpus position.** **0 ADR-typed and 0 SPEC-typed
documents exist across all 199 classified documents** (75 PRD, 124 DOC). No LOCKED-vs-LOCKED
contradiction was ever possible, in any run. Every one of the 554 `REQ-*` acceptance criteria sits
at PRD or DOC precedence and is auto-overridable — including by the shipped code, which is the real
arbiter wherever the two disagree. Precedence order for this project: **shipped tree >
`.planning/codebase/` map > `intel/code-verification.md` > PRD > DOC > task-list checkbox.**

**Eleven ADR candidates exist and none is promoted.** Promotion requires re-tagging the source
document via `--manifest` and re-running ingest; manufacturing a lock inside a planning artefact
would fabricate authority the corpus does not contain. The two with a live operational cost are
`Milestone_7/Epic_4/rustsec-remediation-plan.md` (the corpus's only expiry date, **2026-09-30**)
and `Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` (the single-source
invariant the tree currently violates) — **the same subject from two milestones.** SUPPLY-03
records the recommendation and does not act on it.

(**Corrected by Phase 12 (plan 12-03), dated 2026-08-09, citing `.planning/decisions/PROMOTION.md`
§Part A and ADR-0016, ADR-0021, ADR-0024, ADR-0025, ADR-0036:** all three claims above are stale.
**Not "none is promoted":** four of the eleven candidates are now promoted — candidate 1 by
ADR-0016, candidate 2 by ADR-0021, candidate 3 by ADR-0024, candidate 5 by ADR-0025 — and this
phase adds a fifth: candidate 7
(`Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8, named here) is
closed by ADR-0036. **The `--manifest`/re-ingest requirement is superseded:** `PROMOTION.md`
§Part A states promotion is now an ordinary write to a directory plus a table row, since ADRs live
in `.planning/decisions/` as their own document class, independent of the ingest manifest, and top
the precedence order. **The tree no longer violates the single-source invariant:** ADR-0036's
`## Code Conformance` section carries the `conforms` verdict and the measurement establishing it,
not restated here. This paragraph's own claim that "SUPPLY-03 records the recommendation and does
not act on it" is likewise stale: SUPPLY-03's own definition above carries the identical correction.
Stated plainly, because it is a fact about the corpus and not only about this passage: three
documents say promotion cannot happen; a fourth says it can, and four ADRs — now five — prove it.)

**Run 3 found the corpus's only decision record — and it is still not a locked decision.**
`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` and its
`-options.md` companion carry `Status: Approved`, `Decision Date: 2026-05-13`,
`Chosen Option: Option A`, a Rationale and a Rejected Options section. Both are manifest-typed
DOC with `locked: false`, so they sit at the lowest precedence tier and create no locked decision.
What the record settles is the **location** of five pure value/error types (`PaladinResult`,
`StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) moving into `paladin-core`, with the
application ports reduced to thin re-exports and `PaladinError` deliberately excluded. **It never
mentions `BattalionResult`**, despite the filename — the run-1 `BattalionResult` field-set variant
is untouched by it and is closed by shipped code instead (see variant group 4). A PRD published
two days later contradicts it; that is variant group 19.

**Relocation is not contradiction.** Milestones 4-6 deliberately restructured what Milestones 1-3
built, so every `src/core|application|infrastructure` path recorded in runs 1-2 — and several
recorded in run 3 — is historical. Supersession chains are recorded in *Superseded but preserved*
below; they are not defects.

---

## v1 Requirements — Milestone 1 close-out (Phases 1-4) — ✅ SHIPPED v0.7.1 (2026-08-04)

Scope: close out Milestone 1 so that the planning record, the code, and the quality numbers all
agree. Nothing here re-implements shipped work.

**All 25 requirements verified.** Archive: `milestones/v0.7.1-ROADMAP.md` ·
Audit: `milestones/v0.7.1-MILESTONE-AUDIT.md`.

### Reconciliation (RECON)

- [x] **RECON-01**: A cited status ledger exists that classifies every outstanding Milestone-1
      task item as *already satisfied by v0.7.0 code* (with `file:line`), *genuinely outstanding*,
      or *deferred with reason* — replacing the 2026-01 task-list snapshot as the source of truth.
      Must also record the three places where shipped code diverges from an ingested requirement
      (MCP Streamable-HTTP vs the specified SSE transport; Qdrant/Sanctum vector search vs the
      specified `sqlite-vss`; the shipped interactive REPL vs Epic 9 non-goal NG-7), the
      code-observed Battalion base module path (`battalion/mod.rs`), and the requirement-count
      discrepancy in the ingest bookkeeping (115 IDs enumerated vs 107 reported in SYNTHESIS.md).
      *Derives: all `intel/context.md` implementation-status topics; INGEST-CONFLICTS warnings 7-8.*

- [x] **RECON-02**: `BattalionConfig` has exactly one authoritative definition, recorded as an ADR
      that names the chosen variant and cites shipped code as evidence. Must account for the fact
      that two distinct `BattalionConfig` structs currently exist in code
      (`battalion/mod.rs` and `citadel.rs`). **Narrowed by ingest run 3:** the *variant choice* is
      settled by shipped code — `battalion/mod.rs:37` is the Epic 4 field set exactly, and
      `CommanderConfig` does not exist anywhere in `crates/` or `src/`, so the v3 position was
      never built. What remains for the ADR is recording that verdict and resolving the
      `citadel.rs` duplicate, which run-3 verification did not address.
      *Resolves: REQ-battalion-config-v1 / -v2 / REQ-commander-config-metadata-dir-v3 (see variant
      group 3).*

- [x] **RECON-03**: `BattalionResult` has exactly one authoritative definition, recorded as an ADR,
      that simultaneously satisfies its four producers (Formation, Phalanx, Campaign, Chain of
      Command) and its consumer (Herald — which needs a Battalion type field and aggregated token
      usage that neither source variant provides). **Narrowed by ingest run 3:** the shipped struct
      at `crates/paladin-core/src/platform/container/battalion/mod.rs:549` is a merged **superset**
      of all three positions, so this is now a recording task, not a reconciliation task —
      `intel/code-verification.md` says explicitly not to plan the latter. The ADR records what the
      superset chose: `per_paladin_times` in place of Epic 5's `execution_time_ms`, and
      `node_errors: Vec<NodeError>` in place of `errors: Vec<PaladinError>` (because
      `BattalionError` does not derive `Serialize`/`Deserialize` while `BattalionResult` does).
      *Resolves: REQ-battalion-result-v1 / -v2 + REQ-herald-battalion-result-fields; must account
      for the later position in REQ-battalion-metadata-extension (run 2).*

- [x] **RECON-04**: The minimum Paladin count for Formation/Phalanx and the Commander's
      single-Paladin Auto routing rule have one consistent answer, recorded as an ADR. Shipped code
      currently contains the contradiction live: `formation.rs:109` rejects fewer than 2 Paladins
      while the Auto rule routes a single Paladin to Formation.
      *Resolves: REQ-formation-min-paladins-v1 / -v2.*

- [x] **RECON-05**: Temperature validation has one recorded answer — provider-aware (range from
      `ProviderCapabilities`) or globally clamped — as an ADR. Must account for the run-2 dynamic
      temperature bands (Factual 0.1-0.3 … Creative 0.7-1.0) and the Epic 14 DOC's 0.1-1.0 bound.
      *Resolves: REQ-temperature-range-v1 / -v2; interacts with REQ-dynamic-temperature.*

- [x] **RECON-06**: The `Herald` trait has one recorded method set (fallible vs infallible,
      `format_paladin_stream` vs `format_stream_chunk` + `finalize_stream`, plus `name()`/
      `mime_type()`), as an ADR citing the shipped trait in
      `crates/paladin-core/src/platform/container/herald.rs`.
      *Resolves: REQ-herald-trait-v1 / -v2; must account for REQ-herald-type-consolidation (run 2).*

- [x] **RECON-07**: One authoritative coverage gate is recorded — a single number and a single
      scope — so that a later phase can objectively pass or fail against it. Measured baselines:
      60.88% unit, 67.79% integration. *Resolves: REQ-test-coverage-target-v1 / -v2; extended to
      the run-2 positions by VERIFY-05.*

- [x] **RECON-08**: Whether Epic 10 Task 7.0 (Final Documentation Review) is outstanding is
      answered, and the 102-vs-103 subtask discrepancy between the task list and the validation
      report is explained in the ledger. *Derives: INGEST-CONFLICTS warning 7.*

### Gap closure (GAP)

- [x] **GAP-01**: A developer can run a Chain of Command battalion end-to-end and see the commander
      select specialists, handle a specialist failure via fallback, and synthesize a final answer —
      with unit and integration tests covering all four delegation strategies. Shipped code already
      contains `chain_of_command_service.rs`; this requirement is satisfied by verifying and
      finishing it, not rewriting it. *Derives: Epic 4 task 6.0; REQ-chain-of-command-construction,
      REQ-chain-of-command-execution, REQ-chain-of-command-aggregation.*

- [x] **GAP-02**: Battalion integration and performance-validation tests exist and pass for all four
      patterns, including the Phalanx concurrency claims (≥ 10 concurrent Paladins, < 1 s
      orchestration overhead). *Derives: Epic 4 task 7.0; REQ-phalanx-concurrency,
      REQ-integration-testing.*

- [x] **GAP-03**: Herald is on the Battalion execution path, not just the Paladin one: a Battalion
      result rendered through JSON, Markdown and Table Heralds shows Battalion name/ID/type,
      per-Paladin results in execution order, aggregated token usage, and partial results on error.
      *Derives: Epic 8 task 7.0 and 7.13; REQ-herald-battalion-result-fields (depends on RECON-03).*

- [x] **GAP-04**: Commander execution produces a normalized result with strategy used, per-Paladin
      timings, success/failure counts and preserved `Vec<PaladinError>`, and writes telemetry
      metadata to `metadata_output_dir` when configured. *Derives: Epic 5 task 5.0 (5.10, 5.14);
      REQ-commander-result-normalization, REQ-commander-telemetry (depends on RECON-03). Note run 2
      adds REQ-commander-metadata-export, which specifies the JSON schema and file naming.*

- [x] **GAP-05**: `test_auto_selects_campaign_for_workflow_keywords`
      (`crates/paladin-battalion/src/commander.rs:1864`) passes, and Auto keyword routing is correct
      for all four keyword families. *Derives: Epic 5 task 3.11; REQ-commander-auto-selection.*

- [x] **GAP-06**: Garrison final validation is closed — measured coverage recorded and every Epic 2
      PRD acceptance criterion reviewed against shipped code. *Derives: Epic 2 task 11.0 (11.5,
      11.6); REQ-garrison-testing.*

- [x] **GAP-07**: The reconciled definitions from Phase 1 are applied in code: agreed
      minimum-Paladin behaviour (a single-Paladin Commander in Auto mode executes instead of
      failing validation), the recorded temperature rule, the recorded `Herald` trait signature,
      and the duplicate `BattalionConfig` in `citadel.rs` resolved. **Narrowed by ingest run 3:**
      "one `BattalionResult`" and "one `BattalionConfig` field set" are already true in shipped
      code and are dropped from this requirement — the `citadel.rs` duplicate is what survives.
      *Derives: RECON-02 … RECON-06; `intel/code-verification.md` run-3 resolved variants.*

*Closed 2026-08-01 by Phase 2, plans 02-01 through 02-11. Per-requirement closing evidence
(see `02-VERIFICATION.md`'s "GAP Requirement Coverage" table for the full citations):*
*GAP-01 — plans 02-01/02-03/02-06/02-07, four delegation-strategy test modules passing.*
*GAP-02 — plans 02-01/02-06/02-07, `tests/integration/provider_switching_test.rs` and the CLI
Phalanx suite passing.*
*GAP-03 — plans 02-04/02-05 **and 02-10**, which closed the Table Herald multi-byte truncation
panic `02-VERIFICATION.md` graded a blocker (`table_herald.rs::truncate_text`, char-boundary fix).
GAP-03 was checked at commit `a5f8c27`, reverted at commit `9e5ec04` when `02-VERIFICATION.md`
returned `gaps_found` on that panic, and is re-checked here only because plan 02-10 closed the
defect that caused the revert — `cargo test -p paladin-herald` (70 passed, 0 failed) confirmed
green on this commit before this flip.*
*GAP-04 — plan 02-01, `commander.rs` normalized-result and telemetry-export code paths exercised
by passing tests.*
*GAP-05 — plan 02-01, `test_auto_selects_campaign_for_workflow_keywords` confirmed passing.*
*GAP-06 — plan 02-08, `02-garrison-prd-review.md`'s 50-row criterion table.*
*GAP-07 — plans 02-02/02-03/02-09, Phase 1 ADRs applied in code (temperature_range,
`BattalionCheckpointConfig` rename, Formation single-Paladin acceptance).*
*Full evidence lives in `.planning/phases/02-functional-gap-closure/02-VERIFICATION.md` and
`.planning/ledgers/milestone-01.md` — this note is a signpost, not a duplicate of either.*

### Quality gates (QUAL)

- [x] **QUAL-01**: `cargo llvm-cov` reports unit coverage at or above the gate recorded in
      RECON-07, up from the 60.88% baseline. *Derives: REQ-test-coverage-target-v1 / -v2,
      REQ-unit-test-gap-closure (unit-test-improvements tasks 2.0, 6.0).*

- [x] **QUAL-02**: No first-party source file reports 0% coverage. Known offenders:
      `arsenal_execution_service.rs` (0/46 lines), `arsenal_registry_service.rs` (0/28),
      `redis.rs`, `minio.rs`, `user_controller.rs`, `sqlite_user_repository.rs`, `main.rs`;
      plus the sub-15% files `campaign_service.rs` (4.26%), `chain_of_command_service.rs` (13.41%),
      `mcp_protocol.rs` (15.83%), `deepseek_adapter.rs` (15.02%).
      *Derives: REQ-unit-test-gap-closure; `unit-test-improvements/COVERAGE_ANALYSIS.md`.*

      **Amended 2026-08-02, plan 03-08, citing `03-coverage-measurement.md`.** The eleven-file
      offender list above derives from the ingested pre-workspace
      `unit-test-improvements/COVERAGE_ANALYSIS.md`, not from a measurement of the shipped tree. A
      workspace-wide measurement of the shipped tree at HEAD `bb35554d` (entry) / `1ad8be5` (exit)
      (`03-coverage-measurement.md`) contradicts nine of its eleven entries — each is retained above
      verbatim and corrected here, claimed figure first, then measured line coverage:
      `arsenal_execution_service.rs` claimed 0/46 lines (0%), measured **90.23%**
      (`src/application/services/arsenal/arsenal_execution_service.rs`, 215 lines, 21 missed);
      `arsenal_registry_service.rs` claimed 0/28 (0%), measured **100.00%**
      (`src/application/services/arsenal/arsenal_registry_service.rs`, 59 lines, 0 missed);
      `user_controller.rs` claimed 0% (implied), measured **72.59%**
      (`crates/paladin-web/src/user_controller.rs`, 518 lines, 142 missed);
      `sqlite_user_repository.rs` claimed 0% (implied), measured **87.72%**
      (`crates/paladin-storage/src/sqlite_user_repository.rs`, 448 lines, 55 missed); `main.rs`
      claimed 0% (implied), measured **41.38%** (`src/main.rs`, 29 lines, 17 missed);
      `campaign_service.rs` claimed 4.26%, measured **80.00%**
      (`crates/paladin-battalion/src/campaign_service.rs`, 180 lines, 36 missed);
      `chain_of_command_service.rs` claimed 13.41%, measured **84.56%**
      (`crates/paladin-battalion/src/chain_of_command_service.rs`, 259 lines, 40 missed);
      `mcp_protocol.rs` claimed 15.83%, measured **95.65%**
      (`src/infrastructure/adapters/arsenal/mcp_protocol.rs`, 253 lines, 11 missed);
      `deepseek_adapter.rs` claimed 15.02%, measured **67.77%**
      (`crates/paladin-llm/src/deepseek/adapter.rs`, 391 lines, 126 missed). `redis.rs` is the one
      confirmed true positive — 0.00% at entry (`crates/paladin-storage/src/redis.rs`, 350/350
      missed) — and is now **closed**: plan 03-05's Docker-free unit tests bring it to **34.69%** at
      exit (441 lines, 288 missed); its live-server paths remain `deferred with reason`, owner
      Phase 15 / PIPE. `minio.rs` never appears in this run's `llvm-cov report` output at all — the
      `s3` feature that gates its compilation is not part of the workspace default-feature set
      ADR-0006 measures under, so `minio.rs` is **outside ADR-0006's recorded scope**, not a 0% file;
      owner **VERIFY-05 / PIPE-02**. Restated surviving substance against this re-derived set: nine
      of the eleven originally named files now measure non-zero (figures above, all `satisfied` at
      the D-19 bar via the workspace test suite); `redis.rs` is closed at 34.69% with its
      live-server remainder `deferred with reason` (owner Phase 15 / PIPE); `minio.rs` is
      `deferred with reason`, out of ADR-0006's scope, owner VERIFY-05 / PIPE-02.
      `src/bin/paladin-server.rs` — not one of QUAL-02's originally named offenders, but part of the
      same re-derived zero-coverage set — remains at 0.00% and is `deferred with reason`, owner
      Phase 5 / VERIFY-05 (closing it needs a `run()` seam extracted from
      `#[tokio::main] async fn main()`).

- [x] **QUAL-03**: Integration coverage of critical paths (Paladin execution, Battalion
      orchestration, tool invocation) is at or above 70%, up from the 67.79% baseline.
      *Derives: REQ-integration-testing.*

      **Amended 2026-08-02, plan 03-08, citing `03-critical-path-exercisers.md` and
      [ADR-0006](.planning/decisions/0006-coverage-gate.md).** The clause "at or above 70%, up from
      the 67.79% baseline" is **superseded by shipped code** — ADR-0006 abolished a second coverage
      number under a second scope in favor of one workspace-wide line-coverage figure (QUAL-01);
      ROADMAP criterion 1 was already amended by plan 01-12 to cite the ADR's single figure, while
      this clause was not, until now. No coverage percentage is recorded for QUAL-03 in this
      amendment, or in `03-coverage-measurement.md`'s exit section, or in any other artifact this
      phase writes. Surviving substance: each of the three named critical paths has a named,
      passing, non-`#[ignore]`d integration exerciser at the D-19 bar
      (`01-CONTEXT.md` D-19: `file:line` citation **plus** a named passing exerciser), evidenced in
      `03-critical-path-exercisers.md` — Paladin execution: `test_end_to_end_paladin_execution`
      (`tests/integration/paladin_integration_test.rs:19`); Battalion orchestration:
      `test_commander_executes_formation_end_to_end`
      (`tests/integration/commander_integration_tests.rs:150`) and
      `test_load_formation_50_concurrent_battalions`
      (`tests/integration/battalion/load_test.rs:102`); tool invocation:
      `function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call`
      (`tests/integration/arsenal_bridge_regression_test.rs:165`) and
      `streamable_http_round_trip_with_correct_bearer_token_succeeds`
      (`tests/integration/mcp_streamable_http_test.rs:342`). All three paths are `satisfied` at the
      D-19 bar; none is `genuinely outstanding`.

- [x] **QUAL-04**: Error-path tests run instead of being skipped — the `#[ignore]`d Commander
      tests exercise real failure scenarios (retry count increments, partial-failure collection,
      timeout cascade), and MCP failure modes each have a passing test (expired/401 token,
      malformed response, handshake timeout, unknown tool, bad arguments). Four `#[ignore]`
      attributes remain in `crates/paladin-battalion/src/commander.rs` as of v0.7.0.
      *Derives: `codebase/CONCERNS.md` test-coverage gaps; REQ-arsenal-resilience;
      REQ-commander-test-hardening (run 2) specifies the six tests by name.*

- [x] **QUAL-05**: `cargo bench` completes and a baseline document records throughput, P50/P95/P99
      latency, memory-per-Paladin and startup time for the Paladin execution loop, the Battalion
      patterns, Garrison operations and Arsenal invocation. Benchmarks were relocated into
      per-crate `benches/` directories during the workspace decomposition — the shipped set is
      `crates/paladin-battalion/benches/battalion_benchmarks.rs`,
      `crates/paladin-memory/benches/{garrison,sanctum}_benchmarks.rs`,
      `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` and
      `benches/config_benchmarks.rs`; the Milestone-1 `paladin_benchmarks.rs`,
      `herald_benchmarks.rs` and `arsenal_benchmarks.rs` are not in the tree.
      *Derives: REQ-performance-benchmarking; REQ-battalion-benchmark-repair (run 2).*

### Release coherence (REL)

- [x] **REL-01**: Version metadata agrees everywhere — workspace `Cargo.toml`, member crate
      versions, the git tag and the release notes tell one story. Current state: branch
      `release/v0.7.0`, `Cargo.toml` `0.6.0`, latest tag `v0.5.1`. *Derives: repo state.*

- [x] **REL-02**: Every workspace crate declares one consistent, valid Rust edition, and
      `cargo build --workspace` succeeds under it. **Exact state verified 2026-07-30:** the root
      `paladin-ai` package and nine crates — `paladin-core`, `paladin-battalion`, `paladin-herald`,
      `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-content`, `paladin-web` and
      `doc-examples` — declare `edition = "2024"`; exactly two, `crates/paladin-ports` and
      `crates/paladin-notifications`, declare `"2021"`. The documented
      answer is contested (variant group 17) and is recorded in ARCH-03(a); REL-02 is the code fix.
      Whichever of Phase 4 / Phase 7 executes first records the answer, the other applies it.
      *Derives: `codebase/CONCERNS.md` tech debt; REQ-workspace-crate-edition-v1 / -v2 (run 3).*

- [x] **REL-03**: `cargo audit` and `cargo deny` report no high/critical advisories, and every
      ignored advisory in `deny.toml` carries a written rationale plus a migration or review note.
      Current: 2 medium transitive advisories (RUSTSEC-2023-0071 rsa, RUSTSEC-2025-0111 tokio-tar),
      3 unfixed feature-gated advisories (lopdf, quick-xml ×2), 10 unmaintained-crate ignores, and
      a dual `reqwest` 0.12/0.13 exposure. *Derives: REQ-epic10-quality-gates; CONCERNS.md.*

- [x] **REL-04**: Documentation final review is complete per the RECON-08 answer, and a developer
      following QUICKSTART on a clean machine reaches a working agent with the elapsed time
      recorded against the documented < 15-minute target. *Derives: REQ-user-documentation,
      REQ-api-documentation, REQ-architecture-documentation, REQ-operations-documentation,
      REQ-contribution-documentation.*

- [x] **REL-05**: The full gate suite passes in CI on the release branch: `cargo fmt --check`,
      `cargo clippy -- -D warnings`, `cargo test --workspace`, doc tests, all 22 examples,
      multi-arch Docker build within the < 500 MB / < 5 min budget (**Amended by Phase 4, dated 2026-08-03, citing `04-ci-gate-deferrals.md` §"Second CI execution"**: the "< 5 min" build-time figure is **scoped to single-arch** and is advisory for the multi-arch build. It derives from `PROJECT.md:767`'s single-arch "112 MB built in 5m31s" (Milestone 1) but is applied here to a multi-arch build, and on that build it has never been met in this repository's history: Release runs v0.4.2 48m09s, v0.4.3 47m58s, v0.5.0 41m47s, v0.5.1 44m03s, and the first CI execution of this gate 49m43s — **measured 2946 s against 300 s** on 2026-08-03. `Dockerfile:33` builds natively per platform, so `linux/arm64` compiles the whole workspace under QEMU emulation; the number measures the GitHub runner, not Paladin. The CI job now reports it as a warning. **The < 500 MB size budget remains a hard gate** — size is a property of the artifact, and the last successful multi-arch build measured **86 MB**. Replacing QEMU with native `ubuntu-24.04-arm` runners (free for public repositories) and reinstating a hard, evidence-backed time budget is **Phase 15 / PIPE**.)**, and the kind-based Kubernetes
      smoke test within the < 30 s pod-startup budget. (**Amended by Phase 4, dated 2026-08-03,
      citing `04-release-measurement.md`**: "all 22 examples" is stale — it traces to a Milestone 1
      Epic 10 validation report ("22/22 examples compiling") restated in `ROADMAP.md` and
      `PROJECT.md`. The shipped tree carries 47 `.rs` files under `examples/`, 4 declared
      `[[example]]` targets gating on non-default features, 0 crate-level `examples/` directories.
      This requirement's own gate is corrected to "every example target builds", proven by a
      4-invocation feature matrix plus a binary-presence assertion rather than a count restated
      from an ingested report — so the figure cannot go stale the same way again.)
      *Derives: REQ-epic10-quality-gates, REQ-deployment-artifacts.*

---

## v1 Requirements — Milestone 2-3 close-out (Phases 5-6) — ◆ CURRENT MILESTONE v0.7.2

**These 9 requirements are the scope of milestone v0.7.2, started 2026-08-04.** The 52 forward
requirements in the three sections below (Phases 7-16) are not in this milestone.

Two of them — CLOSE-02 and CLOSE-03 — are **scope-deferred by construction**: their size is set by
Phase 5's verdicts, not knowable at planning time. CLOSE-02 closes with a recorded "no work
required" verdict if VERIFY-02 finds all three blocks satisfied by shipped code. Plan Phase 6 after
Phase 5 reports, not before.

Added by ingest run 2. Scope is deliberately small: **Milestones 2 and 3 shipped.** Every
capability those 118 requirements describe — Sanctum, RAG, Sentinel vision, autonomous planning
and handoffs, Conclave, Council, Grove, the Maneuver Flow DSL, the enhanced CLI, Herald
consolidation, the Paladin registry, the scheduler port — has a corresponding artefact in the
v0.7.0 tree. What is *not* recorded is which PRD acceptance criteria those artefacts actually
satisfy, which competing surface each one implements, and what the three unverified
open-checkbox blocks contain. That is the forward work below, plus exactly one verified defect.

### Ground truth (VERIFY)

- [x] **VERIFY-01**: The *Milestone 2-3 as-shipped ledger* below is upgraded from component-level
      file evidence to per-criterion verdicts with `file:line` citations, for all 118 run-2
      requirement IDs. Must record, per requirement, whether the shipped artefact satisfies the
      PRD acceptance criteria, diverges from them, or only partially covers them. Must also record
      the two systematic path caveats: **every `src/core|application|infrastructure` path in the
      run-2 PRDs predates the workspace decomposition** (Milestone 5, ingested in run 3 — see the
      *Milestone 4-6 as-shipped ledger* for the current crate layout), and
      the Milestone-1 benchmark files those PRDs reference have been relocated into per-crate
      `benches/` directories. *Derives: `intel/code-verification.md`; INGEST-CONFLICTS INFO
      "Sentinel and Autonomous docs disagree with the codebase map on where features live".*

- [x] **VERIFY-02**: The three run-2 open-checkbox blocks that `intel/code-verification.md` leaves
      unverified are checked against the tree, and each produces a written verdict —
      *satisfied by shipped code*, *genuinely outstanding*, or *deferred with reason*. The blocks
      are `tasks-epic22-battalion-commander-hardening.md` (81 open),
      `tasks-autonomous-agent-features.md` (45 open) and
      `tasks-test-hardening-benchmarks-qa.md` (29 open). The deliverable is a verdict per block,
      **not** a task list derived from checkbox arithmetic — run 1 and run 2 both proved checkbox
      state understates reality. *Derives: `intel/task-completion-state.md`;
      `intel/code-verification.md` "Not yet verified".*

- [x] **VERIFY-03**: The Milestone 3 epic-numbering defect is recorded once and permanently, and
      the defective source document is corrected in-repo. The authoritative numbering is the
      plan/epic-definition set — 19 Herald & Domain Type Consolidation, 20 Vision Pipeline
      Completion, 21 Autonomous Agent Completion, 22 Battalion & Commander Hardening, 23
      CLI/Config/Infrastructure Completion, 24 Test Hardening — agreed by 8 of 9 Milestone-3
      documents and every task list. `RELEASE_NOTES_MILESTONE_3.md` must stop asserting
      19=Conclave / 20=Council / 21=Grove / 22=Maneuver / 23=Commander, and its
      "What's Next (Milestone 4)" section must be marked as a superseded point-in-time
      forward-look (vision shipped). Two further release-notes claims must be corrected or
      withdrawn: `RoutingStrategy::PerformanceBased`, which **does not exist in the tree** and
      contradicts Epic 16 non-goal NG-3, and the Council/Maneuver API forms that disagree with the
      shipped surfaces. *Derives: `intel/code-verification.md` "Milestone 3 epic numbering";
      INGEST-CONFLICTS warnings 1, 2, 12, 13, 14.*

- [x] **VERIFY-04**: The two vision port surfaces are recorded as **coexisting, not competing** —
      `crates/paladin-ports/src/output/vision_llm_port.rs` (Epic 13 `VisionCapableLlm` lineage,
      reached via `PaladinBuilder::enable_vision`) and
      `crates/paladin-ports/src/output/vision_port.rs` (Epic 20 `VisionPort` lineage, reached via
      `PaladinExecutionService::execute_with_vision`). Both ship. The recorded answer must state
      whether both are intended long-term or one is legacy, and must not plan a migration on the
      strength of the PRD conflict alone. Must also record whether Epic 13's
      encryption-at-rest / retention / zeroization security requirements
      (REQ-vision-security-encryption) were consciously dropped when Epic 20 finished the feature
      or merely not restated — Epic 20's `VisionError` has no `EncryptionError` variant.
      *Derives: `intel/code-verification.md` "Vision API surface — BOTH shipped";
      INGEST-CONFLICTS warnings 6, 8.*

- [x] **VERIFY-05**: RECON-07's single coverage answer is extended to cover the two positions
      run 2 added, so the gate has one number and one scope across all four positions: 80% (nine
      Milestone-1 Epic PRDs), 85% (`unit-test-improvements` PRD), **overall ≥ 75% with a layered
      per-tier table** (Milestone 3 plan: core ≥ 85%, application ≥ 80%, infrastructure ≥ 70%, CLI
      ≥ 70%) and ≥ 80% / ≥ 70% re-asserted by Epic 24. The recorded answer must state how the
      module-scoped gates coexist with it — Herald ≥ 95% (REQ-herald-consolidation-quality-gates)
      and autonomous components ≥ 90% (REQ-autonomous-completion-quality-gates) — and must be
      falsifiable against the ~78% overall figure reported in the Milestone 3 release notes.
      *Derives: REQ-test-coverage-target-v1 / -v2, REQ-epic24-quality-gates; INGEST-CONFLICTS
      warning 3 and the module-scoped-targets INFO.*

- [x] **VERIFY-06**: The live-API-test missing-key behaviour has one recorded answer, and the
      shipped harness matches it. Epic 23 FR-23.4.4 and Epic 24 US-24.7 both require graceful skip
      with a clear message; the post-Epic-24 cleanup deliberately reversed this, changing
      `require_api_key()` to panic so that "tests will now properly FAIL when keys are missing".
      Both positions are defensible and the reversal was conscious, so precedence cannot settle it.
      *Derives: REQ-provider-live-api-tests; INGEST-CONFLICTS warning 17.*

### Verified gap closure (CLOSE)

- [x] **CLOSE-01**: Grove routing uses the LLM model from configuration instead of a hardcoded
      literal. `crates/paladin-battalion/src/grove_service.rs:537` builds its routing `LlmRequest`
      with `model: "gpt-4".to_string(), // TODO: Make configurable` in production code
      (`#[cfg(test)]` begins at line 732), so Grove routing silently ignores the configured
      provider. This is the **only defect in run-2 scope verified open against the tree**, it is
      the same defect class Epic 21 removed from `planning_service.rs` and
      `prompt_generation_service.rs`, and it means Epic 22's completion criterion "all inline TODOs
      in Battalion and Commander files resolved" is not met. *Derives: REQ-grove-llm-routing,
      REQ-autonomous-configurable-model; `codebase/CONCERNS.md` "Grove Service Model Hardcoded";
      INGEST-CONFLICTS warning 18.*

      **Amended 2026-08-05, plan 06-07 — satisfied.** `GroveConfig.routing_model: Option<String>`
      (`crates/paladin-core/src/platform/container/battalion/grove.rs:208`), threaded through
      `GroveBuilder.routing_model(..)`; `route_by_llm` in `grove_service.rs` no longer hardcodes
      `"gpt-4"` — a guard hard-errors with `BattalionError::RoutingError` (no fallback of any kind)
      when `routing_model` is absent or blank under `RoutingStrategy::LlmRouting`, and the
      configured model is what reaches `LlmRequest.model`; proved by
      `crates/paladin-battalion/src/grove_service.rs#test_llm_routing_uses_configured_routing_model`
      and the missing/blank/precedence/concurrency tests in `06-01-SUMMARY.md`'s coverage table
      (all pass, `cargo test --workspace` green). ROADMAP criteria 1 and 2 are both met: the
      `"gpt-4"` literal is gone from `grove_service.rs`'s production region
      (`awk '/^#\[cfg\(test\)\]/{exit}{print}' grove_service.rs | grep -c 'gpt-4'` → 0), and no
      inline deferral comment remains anywhere in `crates/paladin-battalion/src/` at that line — the
      file's only TODO is resolved (`grep -rn 'TODO' crates/paladin-battalion/src/ | grep -c
      'grove_service.rs'` → 0). The one-way runtime break (D-02) is recorded three ways per D-03:
      **ADR-0013** (`.planning/decisions/0013-grove-routing-model.md`), a `## [Unreleased]`
      **CHANGELOG.md** entry, and the `GroveConfig.routing_model` rustdoc itself — all three shipped
      by plans 06-01 and 06-06.

      **Amended 2026-08-05, plan 06-10 — correction and closure.** `06-VERIFICATION.md` (truth 3;
      `missing:` item (c)) proved the guard above correct in isolation but unreachable from
      `GroveExecutionService::execute()` — the requirement's own governing entry point — because
      `route_task`'s blanket `Err` arm intercepted the deliberate `routing_model`-absent
      configuration error and substituted `fallback_tree` or the first agent in the first tree
      instead of propagating it. The then-standing `test_grove_llm_routing` integration test was
      green while asserting exactly that fallback, which is what made the gap invisible until
      verification ran the entry point directly rather than the crate-private `route_by_llm`.

      This corrects one specific sentence in the amendment above: "ROADMAP criteria 1 and 2 are both
      met" was accurate for criterion 2 (the `"gpt-4"` literal and the TODO were genuinely gone) and
      for criterion 1's happy path (the configured model does reach `LlmRequest.model`), but
      overstated criterion 1's implicit no-fallback contract — the hard error the amendment describes
      was not, at the time, reachable by any real caller of `execute()`. The original sentence is
      retained above unedited; this paragraph is the correction, not a rewrite.

      **What closed it:** plan 06-08 added `GroveExecutionService::resolve_routing_model`, a single
      shared resolver backed by `MISSING_ROUTING_MODEL_ERROR`, called from both `route_task`'s new
      pre-dispatch early return (above the fallback arm, so `?` propagates the configuration error
      before the fallback arm can see it) and `route_by_llm`'s in-strategy guard, so the two checks
      cannot drift apart.

      **Proof, re-run at HEAD in this plan (2026-08-05):**
      `cargo test -p paladin-ai --test lib grove_integration_test` — 10 passed, 0 failed, naming
      `test_grove_llm_routing_errors_when_routing_model_absent_through_execute` (a configured
      `llm_port`, absent `routing_model`, asserts `Err(BattalionError::RoutingError(..))` and zero
      LLM calls via a recording mock) and the former counter-example `test_grove_llm_routing`, now
      inverted to assert the error, both passing. `cargo test -p paladin-battalion --lib --
      grove_service::` — 23 passed, 0 failed, naming the three `execute()`-level edge tests
      (`test_execute_errors_when_routing_model_absent`, `test_execute_errors_when_routing_model_blank`,
      `test_execute_errors_despite_fallback_tree_when_routing_model_absent`) passing alongside the
      four pre-existing `route_by_llm`-level guard tests, unmodified and still green.
      `awk '/^#\[cfg\(test\)\]/{exit}{print}' crates/paladin-battalion/src/grove_service.rs | grep -c
      'gpt-4'` → `0`. `grep -rn 'TODO' crates/paladin-battalion/src/ | grep -c 'grove_service.rs'` →
      `0`.

      **Scope boundary:** the hard error covers the missing/blank `routing_model` case only. Every
      other Grove routing failure keeps its existing fallback behaviour, proved by
      `test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set` (an absent
      `llm_port` under `LlmRouting` with `routing_model` set still falls back successfully through
      `execute()`), which also passed in the re-run above.

      ADR-0013, `CHANGELOG.md` and `.planning/PROJECT.md` were reconciled with this same
      `execute()`-reachable behaviour by plan 06-09 (06-VERIFICATION.md truth 10); all four records
      now agree.

- [x] **CLOSE-02**: Everything VERIFY-02 classifies as *genuinely outstanding* in Epics 14, 22 and
      24 is either closed or explicitly deferred with a recorded reason. Scope is set by Phase 5's
      verdicts, not by the 155 open checkboxes in those three lists. If VERIFY-02 finds all three
      blocks satisfied by shipped code, this requirement closes with a recorded "no work
      required" verdict rather than being deleted. *Derives: VERIFY-02.*

      **Amended 2026-08-05, plan 06-07 — satisfied, all four items disposed, none omitted (ROADMAP
      success criterion 3):**
      (a) **Epic 14 cluster `8.0` (YAML & CLI Configuration Support) — closed by plan 06-03.**
      `PaladinYamlConfig.autonomous: Option<AutonomousConfig>` deserializes, round-trips, and is
      bounds-validated; the four `--auto-plan`/`--auto-prompt`/`--dynamic-temp`/`--enable-handoffs`
      flags apply as additive-only overrides via `apply_autonomous_config`; proved by
      `src/application/cli/commands/agent.rs#test_autonomous_planning_from_yaml_reaches_paladin_data`
      and the 16-case `#test_yaml_enabled_feature_cannot_be_disabled_from_cli`
      (`06-03-SUMMARY.md`).
      (b) **Epic 24 cluster `1.0` (ChainOfCommand benchmark) — closed by plan 06-04.**
      `benchmark_chain_of_command` registered in `battalion_benchmarks.rs`'s `criterion_group!`
      driving a real `ChainOfCommandExecutionService` across three criterion ids
      (`battalion/chain_of_command_2_levels_3_subordinates`, `_2_levels_5_subordinates`,
      `_wide_10_subordinates`); proved by `cargo bench --no-run -p paladin-battalion` (exit 0) and
      an uncontended measured run recorded in `docs/src/appendix/performance-baseline.md`'s new
      dated `## Run — 2026-08-05` section (`06-04-SUMMARY.md`).
      (c) **Epic 24 cluster `8.0` (the three CI jobs) — deferred with a written reason.** All three
      — `cli-tests`, `bench-check`, `coverage` — are literally **PIPE-01** (`cli-tests` +
      `bench-check`) and **PIPE-02** (`coverage` + `.codecov.yml`), owned by **Phase 15**, per D-09:
      Phase 15's own register states its first half "establishes quality gates that validate all
      subsequent work" and must come first, and PIPE-02 cannot be built before a coverage threshold
      is settled among six competing positions (the Deferred-QA parent PRD's 78% hard gate, Epic
      25's 70→74→78 ramp, and ADR-0006's 84% floor among them) — Phase 6 does not pick one. Recorded
      in `.planning/ledgers/milestone-02-03.md`'s Epic 24 block verdict and `### Phase 6 CLOSE-02
      scope` section, and bidirectionally on PIPE-01/PIPE-02 below (D-10).
      (d) **Epic 22 — recorded "no work required."** All fifteen of
      `tasks-epic22-battalion-commander-hardening.md`'s parent-task clusters verify against the
      current tree, including the three the source task list still marks open (Council/Grove
      registry integration, Grove LLM routing — shipped via commits `761c49c`, `0cdf8dd`, `5f05db7`).
      Per `.planning/ledgers/milestone-02-03.md`'s block verdict (`satisfied by shipped code`), this
      block closes with **no work required** for Phase 6 rather than being deleted from this
      requirement's scope — this sentence is what satisfies CLOSE-02's own "closes with a recorded
      'no work required' verdict" text and ROADMAP success criterion 3's "if verification found
      nothing outstanding, that verdict is recorded rather than the requirement quietly dropped"
      clause.
      Also recorded here: **WARN-01** (Herald reachability from Campaign, Chain of Command and the
      Commander router) was adopted under this requirement and closed by plan 06-02 — the Herald
      triad (field, setter, format wrapper) now exists on all three previously Herald-less services,
      proved by the composite end-to-end witness
      `tests/integration/battalion_chain_of_command_herald_test.rs#chain_of_command_result_renders_through_json_herald`
      (`06-02-SUMMARY.md`).

      **Re-affirmed 2026-08-05, plan 06-10.** `06-VERIFICATION.md` independently verified this
      requirement against the tree (✓ SATISFIED, Requirements Coverage table). The checkbox above
      was reverted alongside CLOSE-01's and CLOSE-03's only because CLOSE-01's gap blocked the whole
      phase, not because anything in (a)-(d) or WARN-01 was found lacking. No new claim is made here;
      the exercisers items (a)-(d) already cite were re-run at HEAD in this plan:
      `cargo test -p paladin-ai --lib --features cli -- autonomous` (11 passed, naming
      `test_load_paladin_config_without_autonomous_section`,
      `test_load_paladin_config_with_autonomous_section`,
      `test_no_autonomous_section_and_no_flags_is_a_no_op`,
      `test_autonomous_flag_application_is_idempotent_and_independent` and
      `test_autonomous_prompts_yaml_and_flag`) plus
      `cargo test -p paladin-ai --lib --features cli -- test_yaml_enabled_feature_cannot_be_disabled_from_cli`
      (1 passed — its name has no literal `autonomous` substring, so it does not match the
      `-- autonomous` filter and was re-run explicitly); `cargo bench --no-run -p paladin-battalion`
      (exit 0) and `grep -c 'benchmark_chain_of_command'
      crates/paladin-battalion/benches/battalion_benchmarks.rs` → `2`; `git log --oneline -- .github/`
      (no phase-6 commit — D-11 still honoured); and
      `cargo test -p paladin-ai --test lib battalion_chain_of_command_herald_test` (2 passed) for
      WARN-01.

- [x] **CLOSE-03**: The Phase 5 recorded answers that have code consequences are applied: the
      VERIFY-06 answer on live-API-test key handling is reflected in
      `tests/integration/llm_live_api_tests.rs`, and the VERIFY-04 answer on the two vision
      surfaces is reflected in the tree (both retained and documented as such, or one deprecated
      with a migration note). No surface is removed without a recorded decision.
      *Derives: VERIFY-04, VERIFY-06.*

      **Amended 2026-08-05, plan 06-07 — satisfied, documentation only.** Both vision entry points
      are documented per ADR-0011's `## Decision`: `VisionPort`
      (`crates/paladin-ports/src/output/vision_port.rs`) as the recommended application-code entry
      point (reached via `execute_with_vision`), and `VisionCapableLlm`
      (`crates/paladin-ports/src/output/vision_llm_port.rs`) as the adapter-author surface (reached
      via `PaladinBuilder::enable_vision`) — neither trait deprecated or removed. `EncryptionService`
      (`src/infrastructure/security/encryption.rs`) carries a "Framework usage" rustdoc section
      recording it as a deliberately unimposed, consumer-facing utility. ADR-0011
      (`.planning/decisions/0011-vision-port-surfaces.md`) is amended in place with a dated
      resolution note and its `## Code Conformance` flipped from `must change` to `conforms`. The
      live-API harness's `require_api_key` doc comment is corrected (it no longer claims a skip it
      does not perform) and the double gate (`cfg(feature = "live-api-tests")` plus 13 `#[ignore]`
      attributes) is documented as the actual skip mechanism in `tests/integration/mod.rs`'s header
      — all four from `06-05-SUMMARY.md`, `cargo test --workspace` green throughout, no behaviour
      changed on any path. **No shipped surface was removed anywhere in this phase** — both vision
      traits remain, `require_api_key`'s panic stands unchanged, the four autonomous CLI flags
      remain, and `GroveExecutionService::new`'s signature is unchanged — so no migration note is
      owed under ROADMAP criterion 4.

      **Re-affirmed 2026-08-05, plan 06-10.** `06-VERIFICATION.md` independently verified this
      requirement against the tree (✓ SATISFIED, Requirements Coverage table). The checkbox above
      was reverted alongside CLOSE-01's and CLOSE-02's only because CLOSE-01's gap blocked the whole
      phase, not because anything in the paragraph above was found lacking. No new claim is made
      here; the exercisers already cited are re-run at HEAD in this plan:
      `grep -rn '#\[deprecated' crates/paladin-ports/src/output/vision_port.rs
      crates/paladin-ports/src/output/vision_llm_port.rs` returns nothing (neither vision trait
      deprecated or removed), and `cargo test --workspace` exits 0 (418+ test-binary results, 0
      failed) — the parallel-run evidence behind the CLOSE-03 concurrency truth: this requirement's
      deliverables are documentation-only with no `.rs` behavioural diff, so `tests/integration/mod.rs`'s
      documented double gate (`#[cfg(feature = "live-api-tests")]` plus 13 `#[ignore]` attributes)
      keeps the live-API module out of the parallel `cargo test --workspace` run entirely.

---

## v1 Requirements — Milestone 4-6 close-out (Phases 7-8)

Added by ingest run 3. These three milestones **restructured what Milestones 1-3 built**:
Milestone 4 expanded the feature-flag surface and hardened the port traits, Milestone 5 decomposed
the monolith into a Cargo workspace, Milestone 6 relocated four layer-violating module groups.
Almost all of it shipped, and unusually for this corpus it was verified directly: the workspace
manifest, all named crates, the prelude, the per-crate feature matrices, the `crate-isolation` CI
job, the `application_settings.rs` deletion and its per-domain replacement, the orchestration
relocation, the Maneuver co-location and the `CircuitBreaker` move are each confirmed against the
tree in `intel/code-verification.md`.

**Run 3 is also the first run where an open-checkbox count proved trustworthy.** Milestone 4's 20
open items — all in `tasks-harden-port-traits-stable-api.md` — are corroborated by code: zero
`#[deprecated]` annotations exist anywhere. Milestone 6's 0 open items are corroborated: all four
relocations are complete. Milestone 5's 17 open items are mostly contradicted; the crates, the
prelude, the CI job and the benchmark report all exist.

Forward scope is therefore narrow and concrete: record what shipped and what the four competing
variant pairs resolved to (ARCH), and close the five defects verification proved open (DEBT).
**No forward phase re-plans the workspace extraction or the Milestone 6 relocations** — they
shipped.

### Ground truth (ARCH)

- [x] **ARCH-01**: The *Milestone 4-6 as-shipped ledger* below is upgraded from component-level
      file evidence to per-criterion verdicts with `file:line` citations, for all 115 run-3
      requirement IDs. Must record the corrected workspace shape: **ten library crates**
      (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`, `paladin-llm`,
      `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`,
      `paladin-web`) plus a `doc-examples` crate plus the root facade package `paladin-ai` —
      not the six the Milestone 5/6 overviews and `build-benchmarks.md` assume, and not the
      "9-crate workspace" figure this planning set carried before run 3. **Narrowed by ingest
      run 4:** the provenance of all five extra crates is now supplied and no longer pending —
      `paladin-storage`, `paladin-notifications`, `paladin-content` and `paladin-web` from
      Milestone 7 Epic 1's extraction PRD and its cost-benefit gate, and `paladin-herald` from the
      2026-06-04 reconciliation (commit `66f6c4e`) rather than from any PRD, which is why no
      ingested requirement described it before run 4. HARD-01 records the detail; ARCH-01 keeps the
      workspace shape. Same for `crates/paladin-battalion/src/in_memory_registry.rs`,
      which is absent from Epic 3's 13-file extraction inventory and appears in Epic 3 of
      Milestone 6 as an existing file.
      *Derives: `intel/code-verification.md` run-3 section (22 verified-shipped claims, 12-row
      contradiction table); INGEST-CONFLICTS run-3 INFO on `in_memory_registry.rs`.*

- [x] **ARCH-02**: The milestone/tier numbering collision is recorded once and corrected at its
      source. The Milestone 4 overview is titled "Milestone 1: High-Value, Low-Risk Foundations",
      the Milestone 5 overview "Milestone 2: Workspace Decomposition", the Milestone 6 overview
      lists its prerequisites as "Completed in Milestones 1 and 2", and PRDs cross-reference the
      port hardening as "Milestone 1 / Epic 2" meaning Milestone 4 Epic 2. The authoritative
      numbering for GSD is the **directory / task-list numbering** (4 = Tier 1, 5 = Tier 2,
      6 = Tier 3); every "Milestone 1/2/3" reference *inside* these three milestones is a tier
      label. This is the second numbering defect in the corpus — VERIFY-03 fixes the first — and
      both must be fixed with the same convention so `REQ-*` provenance keys resolve.
      *Derives: INGEST-CONFLICTS run-3 warning 1; affects 9 of the 19 run-3 DOCs.*

- [x] **ARCH-03**: Each of the four run-3 competing variant pairs has exactly one recorded answer,
      citing the shipped code that settles it and stating whether the documents are amended or the
      code is accepted as the resolution. All four are settled *in code* — which is unusual for
      this corpus — but three of the four PRDs are unamended and would produce the wrong answer if
      applied literally to future work:
      **(a) Rust edition** (group 17) — the answer feeds REL-02, which is the code fix. (**Amended
      by Phase 4, dated 2026-08-03, citing `.planning/decisions/0009-workspace-rust-edition-2024.md`**:
      recorded and applied in the same phase — all twelve workspace manifests now declare
      `edition = "2024"`, both required build legs proven green. This clause's remaining scope is
      citing ADR-0009's answer, not deciding it.);
      **(b) `paladin-core` dependency allowlist** (group 18) — the allowlist is the enforcement
      mechanism for the whole hexagonal-purity argument (M5 Epic 1 FR-24, FR-25, SM-4), so leaving
      it wrong by eight crates makes it unenforceable as written; the same drift applies to
      `paladin-ports`, declared at 7 dependencies and shipping 10;
      **(c) port value-type ownership** (group 19) — the one place where mechanical precedence
      produces an architecturally wrong answer, because a PRD outranks an Approved-status decision
      record; the answer must state whether Epic 2's FR-7/FR-10 are amended or the decision record
      is promoted to a real ADR via `--manifest`;
      **(d) LLM config bridge location** (group 20) — the answer must state whether Epic 4's
      circular-dependency concern was real, since Milestone 6 moved the config into `paladin-llm`
      anyway.
      *Derives: REQ-workspace-crate-edition-v1/-v2, REQ-paladin-core-dependency-allowlist-v1/-v2,
      REQ-port-value-type-ownership-v1/-v2, REQ-llm-config-bridge-location-v1/-v2;
      INGEST-CONFLICTS run-3 warnings 2-5.*

- [x] **ARCH-04**: The Milestone 6 facade re-export policy has one recorded answer and its version
      consequence is recorded. The Milestone 6 overview requires the facade to re-export relocated
      types at their original paths for backward compatibility (Epic 2 AC 6, Epic 4 AC 5, and the
      risk register's "facade crate re-exports absorb the change"); both PRDs require the
      opposite (Epic 2 Non-Goal 7 "no shim re-exports are added", Epic 4 §4.11 "the old path is
      intentionally retired"). Shipped code follows the PRDs — `src/application/use_cases/` no
      longer exists at all. The recorded answer must state whether Milestone 6 therefore
      constitutes a **breaking change requiring a major version bump**, which is a direct input to
      REL-01. Epic 2's own Open Question 4 undercuts its Non-Goal ("this should be confirmed with
      the team before implementation begins"), so this is an open decision rather than a
      precedence conflict.
      *Derives: REQ-orchestration-no-reexport-shims, REQ-circuitbreaker-old-path-retired,
      REQ-battalion-facade-shim (which took the opposite posture one milestone earlier);
      INGEST-CONFLICTS run-3 warning 7.*
      (**Amended by Phase 4, dated 2026-08-03, citing `.planning/decisions/0008-workspace-version-0-7-0.md`**:
      the major-version-bump question is answered — Milestone 6's facade change was breaking but
      shipped inside the pre-1.0 series, so the workspace converges on `0.7.0`, a minor bump, not a
      major one. This requirement's remaining scope — the facade re-export policy itself — is
      untouched by Phase 4 and still applies here.)

- [x] **ARCH-05**: The five documented positions that shipped code contradicts are corrected at
      source, so no later work applies them literally. All five verified:

      1. `vision` gating `chacha20poly1305` and `zeroize` — the Epic 1 `dependency-matrix.md`
         audit classifies both as general-purpose deps of `security/encryption.rs` (user auth,
         Citadel) that must stay unconditional; shipped `vision = []` gates no dependency at all.
         Applying the PRD literally would break `cargo build --no-default-features`.

      2. MCP transport feature flags (`mcp-transports` / `mcp-stdio` / `mcp-sse`) — none exists;
         the PRD's dated 2026-04-15 elimination note is what shipped.

      3. `web-server` gating both `actix-web` and `axum` — shipped
         `web-server = ["dep:paladin-web", "dep:axum"]`; actix-web is no longer a root dependency.

      4. A `paladin-cli` workspace crate — never built; the CLI is a `cli` feature plus
         `[[bin]] paladin-cli` with `required-features`. M5 Epic 6's non-goal was correct and the
         overview's target structure was not.

      5. The Milestone 6 Epic 2 target directory `src/application/use_cases/` — the four
         orchestrator modules ship under `src/application/services/` with the PRD's exact module
         names, and `use_cases/` no longer exists.
      Must also record that `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and
      `docs/CONFIGURATION.md` are **relocated into the mdbook**
      (`docs/src/api-reference/{stable-api,feature-flags,migration-guide,crate-map}.md`,
      `docs/src/getting-started/installation.md`), not missing — so Milestone 6 Epic 4's FR-4.12
      now applies to `docs/src/api-reference/stable-api.md` and nobody plans them as gaps.
      *Derives: REQ-vision-feature-gating, REQ-feature-flag-matrix, REQ-feature-flag-docs,
      REQ-orchestration-target-structure, REQ-circuitbreaker-stable-api-update;
      `intel/code-verification.md` "Crate-level facts that contradict run-3 requirement text".*

- [x] **ARCH-06**: The Milestone 4 Epic 3 binary-target architecture question is answered and
      documented, closing FR9.3's never-produced deliverable. Q1 recorded "User selected Option D —
      requires architecture review" and made it a blocker for Task 3.3; no architecture-review
      record exists anywhere in the ingest set. The tree answers it de facto with **three** binary
      targets — `paladin` (`src/main.rs`), `paladin-cli` (`required-features = ["cli"]`) and
      `paladin-server` (`required-features = ["web-server"]`) — i.e. Option A extended. The
      recorded answer must state each binary's intended use case, which is what FR3 asked for.
      *Derives: REQ-binary-target-config, REQ-cli-docs; INGEST-CONFLICTS run-3 INFO.*

- [x] **ARCH-07**: The build-time benchmark record is made falsifiable.
      `Epic_6/build-benchmarks.md` marks four of five scenarios "Meets ≥ 50% target? No" (clean
      build −6.6%, core incremental −18.9%, llm incremental −44.6%) and two "Yes" (memory −50.2%,
      battalion-only −90.9%), then concludes "**Overall verdict: Target achieved**"; it also states
      the clean-build shortfall as −6.6% in its table and −5% in its conclusion. Its own
      methodology note concedes the monolith baseline was measured by touching `src/lib.rs`,
      "primarily a module-tree re-export file", giving "the *best-case* monolith incremental".
      Either re-run against the fairer mid-tree baseline the report itself recommends, or restate
      Milestone 5 SM-7 as a per-scenario target — so the ≥ 50% figure can be judged pass or fail.
      Distinct from QUAL-05, which covers **runtime** benchmarks; this one is build time.
      *Derives: REQ-build-benchmark-report; INGEST-CONFLICTS run-3 warning 13.*

### Verified defect closure (DEBT)

Unlike the run-1 and run-2 open-checkbox blocks, every item below was confirmed against the
shipped tree during ingest run 3 and re-confirmed on 2026-07-30. Each has a small, concrete fix.

- [x] **DEBT-01**: The `api-surface` CI job works. `scripts/check-api-surface.sh:6` and
      `scripts/extract-public-api.sh:6` default `BASELINE` to `project/current-exports.txt`, and
      `.github/workflows/ci.yml:171,181,186` pass that literal path — but the directory was renamed
      in commit `928c6d5` ("chore: moved project to .project") and the baseline now lives at
      `.project/current-exports.txt`. `check-api-surface.sh` exits 1 with "No baseline found" on
      every run, so the single automated guard against unintended public-API changes has been
      failing silently and `check-deprecations.sh` runs after the failing step. Done when an
      intentional public API change makes the job fail and an unchanged tree makes it pass. One
      adjacent item closes here: M4 Epic 2 FR-7.3's `.public-api-baseline.txt` is recorded as
      superseded by `.project/current-exports.txt` plus `final-api.txt` / `api_surface_current.txt`
      — a naming difference, not a missing capability.
      *Derives: REQ-api-surface-ci, REQ-workspace-ci-upgrade; `intel/code-verification.md` run-3
      verified-open items 1 and 3; INGEST-CONFLICTS run-3 warning 9.*
      **Extended by ingest run 4 — five references became six, and the sixth is a requirement
      rather than code.** All five original references are unchanged as of 2026-07-30
      (`.project/current-exports.txt` exists at 442 KB; `project/current-exports.txt` does not).
      Run 4 adds **M8 Epic 7 FR-10** (`REQ-web-api-baseline-changelog`), which mandates
      `./scripts/extract-public-api.sh project/current-exports.txt` — the same stale path — so the
      defect is now enshrined in an ingested requirement and will be reintroduced by anyone
      implementing that FR literally. `REQ-api-surface-baseline-v020` (M8 Epic 5) depends on the
      same job. *Derives additionally: REQ-web-api-baseline-changelog,
      REQ-api-surface-baseline-v020; `intel/code-verification.md` run-4 verified-open item 4;
      INGEST-CONFLICTS run-4 INFO on the re-asserted path.*
      **Extended again by ingest run 5 — six references became nine, and this is now the
      longest-lived unfixed defect in the corpus and the cheapest to close.** Run 5 adds four
      Milestone 12 requirements that all name the stale path: **Epic 1 §7** ("regenerate the
      baseline"), **Epic 5 §7**, **Epic 6 `cross_refs`**, and **Epic 7 FR-4.6** ("`CHANGELOG.md`
      and `project/current-exports.txt` reflect the release"). Those are among the newest
      requirements in the entire corpus — written 2026-06-07 to 2026-06-09, months after the
      rename — so the defect is propagating forward, not decaying.
      **The nine references, in full:** `scripts/check-api-surface.sh:6`,
      `scripts/extract-public-api.sh:6`, `ci.yml:171`, `ci.yml:181`, `ci.yml:186` (tooling, 5);
      M8 Epic 7 FR-10, M12 Epic 1 §7, M12 Epic 5 §7, M12 Epic 6 `cross_refs`, M12 Epic 7 FR-4.6
      (requirement text, 5 — the M12 Epic 6 entry is a `cross_refs` field rather than an FR, so
      the count of *requirements* to correct is 5 while the count of distinct FR clauses is 4).
      **Done when all five tooling references are fixed (a two-line change plus three workflow
      lines) and all five requirement texts are corrected**, so no future implementer writes to a
      path that does not exist. Unchanged across three consecutive ingest runs.
      *Derives additionally: REQ-agent-registry (Epic 1 §7), REQ-per-agent-role-authorization
      (Epic 5 §7), REQ-openapi-drift-guard (Epic 6 cross_refs), REQ-m12-v060-release (Epic 7
      FR-4.6); `intel/code-verification.md` run-5 verified-open item 8; INGEST-CONFLICTS run-5
      warning on the nine stale references.*
      **Scope split recorded so nothing is planned twice:** the four deprecated
      `actions-rs/toolchain@v1` steps this requirement previously absorbed (`ci.yml:147` — which is
      this very job — plus `:317`, `:507` and `integration-tests.yml:71`) now belong to **PIPE-04**
      in Phase 15, which owns the full eight-reference action-modernization sweep. DEBT-01 keeps
      the baseline path; PIPE-04 keeps the action versions.
      **Closed (Phase 8, dated 2026-08-06), both halves.** Tooling half (`08-02-SUMMARY.md`): all
      five path literals corrected (`scripts/check-api-surface.sh:6`,
      `scripts/extract-public-api.sh:6`, `ci.yml:172,182,187`), the baseline regenerated from HEAD
      (`bash scripts/extract-public-api.sh .project/current-exports.txt` → 1968 items), and the
      guard proven in both directions (`bash scripts/check-api-surface.sh .project/current-exports.txt`
      → `✅ API surface unchanged`, exit 0; a throwaway probe function → `❌ API surface has
      changed!`, exit 1, then reverted). Requirement-text half (`08-05-SUMMARY.md`): the five
      `.project/` sources named by this requirement's own "nine references" count (M8 Epic 7 FR-10
      plus M12 Epic 1/5/6/7 §7) were each annotated with a dated D-00c banner and inline
      struck-and-corrected clauses naming `.project/current-exports.txt`. The adjacent M4 Epic 2
      FR-7.3 `.public-api-baseline.txt` item this requirement's own text already names above is
      closed as **superseded by naming** (`.project/current-exports.txt` plus `final-api.txt` /
      `api_surface_current.txt`) — not a missing capability, per the requirement's own sentence.
      **Residual, not closed by this phase:** `08-05`'s own verification grep, re-run 2026-08-07
      (`grep -rn 'project/current-exports.txt' .project/ 2>/dev/null | grep -v '~~' | grep -v
      '\.project/current-exports' | grep -v 'tasks-'`), finds **four** further requirement-level
      sites this "nine references" count never included —
      `.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md:225-227`,
      `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md:254`,
      `.project/Milestone_12-Web-API/overview/Milestone-12_Web-API.md:318`,
      `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/Milestone_8-Epic_7-paladin-web-single-framework-axum.md:40`
      — full detail in `.planning/phases/08-verified-defect-closure/deferred-items.md`. **This
      checkbox is ticked on the literal done-condition this requirement states** ("all five tooling
      references are fixed... and all five requirement texts are corrected"), **which is met in
      full** — the residual four sites are a real, separate propagation the corpus's own
      nine-reference count undercounted, not a failure of this requirement's own five-and-five
      scope. No owning phase is currently assigned for the residual four; a future phase touching
      `.project/` Milestone 8/11/12 records should close them using the same D-00c pattern.
      **Follow-up finding, this same plan's own Task 3 (2026-08-06):** re-running the guard found it
      genuinely `❌ API surface has changed!` — plan 08-07's `table_herald` feature-gating (DEBT-04)
      removed `pub use paladin::infrastructure::adapters::herald::table_herald` from the
      default-feature surface after this baseline was captured, and no later plan regenerated it.
      The guard caught a real, already-`CHANGELOG.md`-documented breaking change correctly — not a
      guard defect. Fixed (Rule 1): baseline re-regenerated, now 1967 items (one fewer, the removed
      re-export); `bash scripts/check-api-surface.sh .project/current-exports.txt` →
      `✅ API surface unchanged`, exit 0, re-confirmed.
      **Addition, dated 2026-08-06 (plan 08-09, on the orchestrator's initiative at seal time — NOT requested by the approving human):** the
      residual four sites now carry a **recommended, non-binding** owner — **Phase 13 (Milestone
      9-12 Ground Truth & Recorded Account)** — rather than remaining unassigned. Three of the four
      sites are Milestone 12 records and Phase 13's whole scope is the M9-M12 recorded account; the
      fourth (Milestone 8) is the closest-fitting adjacent pickup. Phase 13's own planner accepts,
      reassigns, or declines this recommendation at its own discussion/planning stage. Full detail
      in `.planning/phases/08-verified-defect-closure/deferred-items.md`.

- [x] **DEBT-02**: Every type leaving the public API carries
      `#[deprecated(since = "…", note = "…")]` per Milestone 4 Epic 2 FR-8, **or** FR-8 is
      explicitly withdrawn with a recorded reason. `grep -rn '#\[deprecated' src crates` returns
      **0** today, and `Epic_2/DEPRECATIONS.md` corroborates from the other side: "Deprecated
      Items: 0 (none yet)", "Restricted Items: 0 (to be done in Task 6.0)", an empty deprecation
      log, and Tasks 3.0 and 6.0 still in progress. This is the **one genuinely incomplete epic in
      run-3 scope** and the first place in this corpus where an open-checkbox count (20) is
      directly supported by code. Whichever way it resolves, `DEPRECATIONS.md`, the mdbook
      stable-API page and the tree must agree at the end, and the
      v0.2.0 → v0.3.0 → v1.0.0 removal timeline must either start or be withdrawn.
      *Derives: REQ-deprecation-warnings, REQ-stable-api-doc, REQ-api-surface-reduction-target;
      `intel/code-verification.md` run-3 verified-open item 2; INGEST-CONFLICTS run-3 warning 10.*
      **Closed by withdrawal (Phase 8, dated 2026-08-06).** `.planning/decisions/0022-deprecation-requirement-withdrawal.md`
      withdraws Milestone 4 Epic 2 FR-8, citing `DEPRECATIONS.md:81`'s own IMMEDIATE DEPRECATION
      section naming no candidate ("None identified yet...") as the primary evidence, and restating
      the stale `v0.2.0→v0.3.0→v1.0.0` timeline against the shipped `0.7.0` per ADR-0008.
      `grep -rn '#\[deprecated' src crates | wc -l` → `0` (re-confirmed 2026-08-06, unchanged).
      `08-06-SUMMARY.md`'s three-way reading confirms ADR-0022, `DEPRECATIONS.md` (as annotated) and
      `stable-api.md` (its `:875` false present-tense "Current and planned deprecations" claim
      corrected, forward-looking policy prose retained verbatim) all now tell one story: zero
      `#[deprecated]` attributes is the recorded outcome of the withdrawal, not an unfinished task.
      **This requirement closes by withdrawal, not by implementation** — no `#[deprecated]`
      attribute was added, and none should be expected by a later reader.

- [x] **DEBT-03**: `paladin-ports` doctests compile and run.
      `crates/paladin-ports/Cargo.toml:18` sets `[lib] doctest = false` with the comment
      "Doctests in copied port files reference `paladin::` (root crate) which would require a
      circular dev-dependency. Re-enable in **Task 7.0** after rewriting examples to use
      `paladin_ports::` / `paladin_core::` paths", and `ci.yml:225` runs
      `cargo test --workspace --doc --exclude paladin-ports`. The named "Task 7.0" appears in no
      run-3 task list. The practical consequence: the ~25 port traits whose rustdoc examples are
      the framework's primary integration documentation have **zero** executing example coverage,
      and CI is configured to keep it that way. Done when the port doc examples import via
      `paladin_ports::` / `paladin_core::`, `doctest = false` is removed and the CI exclusion is
      dropped. Directly closes M5 Epic 2 FR-21 and Success Metric 8.
      *Derives: REQ-ports-doctest-compilation, REQ-ports-tests-and-rustdoc, REQ-doc-build-clean;
      `intel/code-verification.md` run-3 verified-open item 4; INGEST-CONFLICTS run-3 warning 8.*
      **Re-asserted by ingest run 4, with a companion question.** `doctest = false` and the identical
      "Task 7.0" comment are unchanged, and `ci.yml:225` still excludes the crate. Run 4 supplies the
      documentation requirements this sits underneath — M7 Epic 4 §4.4.1 (`#![warn(missing_docs)]`
      on all public crates), §4.4.3 (zero `cargo doc --workspace --no-deps` warnings) and §4.4.4
      (>90% documented-public-item coverage per crate) — **all three recorded Met** by
      `epic-4-completion-summary.md`. The exclusion and the coverage claim coexist without either
      document acknowledging the other. **Resolve DEBT-03 together with HARD-07**, which settles
      which `cargo doc` bar the project actually holds. *Derives additionally: REQ-doc-coverage-audit,
      REQ-m8-final-quality-gate; `intel/code-verification.md` run-4 verified-open item 5.*
      **Closed (Phase 8, dated 2026-08-06).** `08-03-SUMMARY.md`: `crates/paladin-ports/Cargo.toml`'s
      `[lib] doctest = false` and its stale "Task 7.0" comment removed; `.github/workflows/ci.yml:226`'s
      `Run doc tests` step no longer carries `--exclude paladin-ports` — both edits in one commit
      (`2bffe22`), no window where the doctests exist but CI still excludes them.
      `cargo test --offline -p paladin-ports --doc` → `96 passed; 0 failed; 94 ignored`. Workspace-scoped
      `cargo test --offline --workspace --doc` totals 281 passed vs. 185 with `--exclude paladin-ports`
      re-applied — a delta of exactly 96, proving the exclusion removal changed what CI executes.
      **HARD-07 seam preserved, not decided here:** which `cargo doc --workspace --no-deps` warning
      bar the project actually holds (D-12) remains **Phase 10 / HARD-07**'s to settle — this phase
      records the measured state (`paladin-battalion` 3 warnings, `paladin-ai` 3 warnings, per the
      Milestone 4-6 ledger's `REQ-doc-build-clean` row) without deciding it.

- [x] **DEBT-04**: A library-only consumer compiles zero CLI dependencies. The shipped `cli`
      feature is `["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`
      — 5 of the 8 dependencies the Epic 3 PRD and the Epic 1 dependency matrix classify as
      CLI-only — while `structopt = "0.3"` (`Cargo.toml:93`), `colored = "2.1"` (`:125`) and
      `comfy-table = "7.1"` (`:126`) remain unconditional root dependencies. Done when those three
      are `optional = true` and enabled by `cli`, and `cargo tree --lib --no-default-features`
      shows none of them — satisfying M4 Epic 3 FR5.4 and the §8.3 dependency-reduction metric.
      Note the interaction with the v2 tech-debt item that replaces `structopt` with clap v4:
      gating it does not remove the need to migrate it. This is the **inverse** of the run-1 and
      run-2 pattern — here the checkboxes overstate completion rather than understating it.
      *Derives: REQ-cli-dependency-isolation, REQ-library-only-build, REQ-cli-build-time-measurement;
      `intel/code-verification.md` run-3 verified-open item 5; INGEST-CONFLICTS run-3 warning 12.*
      **Closed (Phase 8, dated 2026-08-06).** `.planning/decisions/0023-cli-dependency-isolation.md`,
      `08-07-SUMMARY.md`, `08-08-SUMMARY.md`: `structopt` fully removed
      (`grep -c structopt Cargo.toml` → `0`; `grep -rln structopt src/ crates/` → no output);
      `src/main.rs` migrated to `clap` v4 derive; the `paladin` `[[bin]]` gained
      `required-features = ["cli"]`; `paladin-herald` gained its first `[features]` section (`table`
      gates `comfy-table`, `color` gates `colored`'s coloured path) threaded through the root `cli`
      feature, with all three root-facade consumer sites gated in lockstep. Criterion-4 proof, run
      2026-08-07: `cargo build --offline --lib --no-default-features` → exit 0, then
      `cargo tree --offline --no-default-features -e normal | grep -E 'structopt|colored|comfy-table'`
      → no output. (The literal, non-`-e normal` invocation shows one hit, `colored v3.1.1`, traced
      to the pre-existing `mockito` dev-dependency — not a criterion-4 violation, since Cargo never
      propagates dev-dependencies into a downstream consumer's build graph; both invocations
      recorded per D-16.) The four `Dockerfile`/CI/docs downstream surfaces this gate broke were
      repaired (`08-08-SUMMARY.md`) and two `CHANGELOG.md [Unreleased]` entries record the two
      breaking changes for consumers. **Coverage-delta accounting for DEBT-06/close-out:** gating
      `table_herald` removed **30** `#[test]` functions from the default-feature run (3 root-crate +
      27 in `paladin-herald`'s own `table_herald` module) — larger than originally anticipated; all
      30 still run and pass under `--features cli` / `--features table,color`, none lost.

- [x] **DEBT-05**: One `TokenUsage`. Three definitions ship simultaneously —
      `crates/paladin-core/src/platform/container/token_usage.rs:13`,
      `crates/paladin-core/src/platform/container/battalion/mod.rs:497` (carrying its own `new()`
      and `from_total()` constructors) and `crates/paladin-llm/src/llm_analysis_service.rs:51`.
      `BattalionResult.per_paladin_tokens` uses the battalion-local copy while the ports layer uses
      the core copy, so token figures crossing that boundary need conversion. This is exactly the
      duplication run 1 first flagged, that run 2's `REQ-herald-type-consolidation` was meant to
      close, and that the Milestone 5 Epic 1 decision record named `container/token_usage.rs` as
      the canonical answer for — the decision moved *one* copy and left two. Done when
      `grep -rn 'pub struct TokenUsage' crates src` returns exactly one result and the other two
      sites are re-exports. Sequenced after ARCH-03(c), which decides whether `paladin-core` or
      `paladin-ports` owns the canonical type.
      *Derives: REQ-port-value-type-ownership-v1/-v2, REQ-herald-type-consolidation (run 2);
      `intel/code-verification.md` run-3 verified-open item 6; INGEST-CONFLICTS run-3 warning 11.*
      **Closed (Phase 8, dated 2026-08-06).** `08-01-SUMMARY.md`: the two duplicate definitions
      (`battalion/mod.rs:497`, `llm_analysis_service.rs:51`) collapsed into `pub use` re-exports of
      the canonical `paladin-core` type, with zero call-site edits across ~182 references.
      `grep -rn 'pub struct TokenUsage' crates src | wc -l` → `1`. `cargo test --offline --workspace --lib`
      → 1574 passed (up from the 1570 pre-change baseline), 0 failed. **`VisionTokenUsage`
      (`crates/paladin-ports/src/output/vision_port.rs:34`) is deliberately out of scope per D-20** —
      `grep -rn 'VisionTokenUsage' crates/paladin-ports/src/output/vision_port.rs | wc -l` → `2`,
      unchanged; a later reader should not treat it as a missed consolidation site.

---

## v1 Requirements — Milestone 7-8 close-out (Phases 9-11)

Added by ingest run 4 (40 docs, 86 requirements). Milestone 7 hardened the workspace for release —
four more crate extractions, Docker/CI/Makefile/benchmark infrastructure, and API stabilization
through a release candidate. Milestone 8 cleaned the facade — 25 dead files deleted, the
`use_cases` → `services` rename, actix-web removed and banned, and (via a reconciliation the
milestone's own planning documents did not anticipate) the relocations Epic 3 had deferred to
Milestone 9.

**Both milestones are effectively complete** — 98.8% and 99.1% by checkbox, and the tree is ahead
of both figures. Milestone 8's three open items are *contradicted*: Epics 2 and 3 are verifiably
complete and Epic 3 went further than its own task list scoped. Milestone 7's three are plausible,
and their genuine residue is two small defects, both carried below.

**The forward work from run 4 is small, concrete, and not in the checkbox arithmetic.** It is the
security- and release-gate drift (SEC), the record that makes Milestone 7-8 legible and stops the
superseded documents misrouting run 5 (HARD), and the disposition of the five-item deferred
register plus the two deliberately removed features (FACADE).

**No forward requirement below re-plans shipped Milestone 7-8 work.** The four extractions, the
benchmark migration, the release-candidate cycle, the facade deletions, the rename and the
reconciliation's fifteen commits all shipped; they are recorded in the *Milestone 7-8 as-shipped
ledger*.

### Release & security gate integrity (SEC)

Every item below was confirmed by direct inspection of the shipped tree on 2026-07-30. These are
the gates the project believes it already has; four of the five do not hold as documented.

> **CORRECTED BY INGEST RUN 5 — read this before acting on SEC-01.** Run 4 recorded that
> `deny.toml` mirrored "only the original two" vulnerability advisories and that "the three 2026
> advisories are absent", and concluded that `deny.toml` violated its own stated sync invariant.
> **That is no longer true and the framing is withdrawn.** Read directly from the tree during run
> 5: `.cargo/audit.toml [advisories] ignore` carries **five** vulnerability advisories and
> `deny.toml [advisories] ignore` carries **fifteen** entries in three explicitly labelled classes
> — **the same five vulnerability advisories** (2 under "mirrored from .cargo/audit.toml", 3 under
> "New 2026 DoS advisories in transitive deps of OPTIONAL features") plus **ten** unmaintained /
> maintenance-mode notices under a header stating they are "informational 'unmaintained' notices,
> **NOT vulnerabilities**". **The vulnerability sets match exactly; the sync invariant is
> satisfied.** Ten of the fifteen are explicitly sanctioned by M10 Epic 4 FR-1 step 5, which
> authorises scoped `[advisories].ignore` entries for unmaintained advisories with an explanatory
> comment.
>
> **The real gap is narrower and different: owner and expiry coverage, plus one unauthorised
> expansion.** 13 of the 15 `deny.toml` suppressions carry documented reasoning but **no named
> owner and no expiry date**, against a Milestone 10 Epic 2 origin policy that mandates a single
> documented exception process; and the three additional *vulnerability* ignores are authorised by
> **no** ingested document (M10 Epic 2 FR-3 and §5 name exactly two). SEC-01's surface-count
> narrative below is preserved as the run-4 record; **SUPPLY-02 (Phase 12) carries the corrected
> scope**, and **SUPPLY-01 (Phase 12) carries the concrete `ci.yml:389-406` deletion** that
> SEC-01's fourth surface describes. Do not plan the same fix twice.

> **CORRECTED BY PHASE 9 (plan 09-07), dated 2026-08-08 — the arithmetic above and in point 3 below
> was itself wrong, twice, before this phase touched a single file.** Read directly this session
> against the pre-Phase-9 tree (`ADR-0024`'s verbatim liveness transcript): `deny.toml`'s
> `[advisories] ignore` array held **fourteen** entries, not fifteen, with **nine** unmaintained
> notices, not ten — and `RUSTSEC-2025-0121` (`gcc`) was already absent from `deny.toml`, absent
> from `.cargo/audit.toml`, and returned zero hits in `Cargo.lock`, with no record anywhere of when
> or by whom it was removed. Consequently "13 of the 15 … no named owner" above is also off by one:
> the correct pre-Phase-9 figure is **twelve of fourteen** lacking owner and expiry (the same two —
> `RUSTSEC-2023-0071`, `RUSTSEC-2025-0111` — carry the formal `rustsec-remediation-plan.md`
> acceptance). The stale `ci.yml:389-406`/`ci.yml:406` line citations above and in point 4 below are
> also corrected: re-derived this session, the duplicate job actually sat at `ci.yml:465-482` (job
> id `security:` at `:466`, the `run:` line at `:482`) — roughly seventy-seven lines below the cited
> range, not at it — and plan 09-06 (this phase) has since **deleted** that job entirely, so any
> future citation into `ci.yml` for this subject must be re-derived against the current file rather
> than carried forward from either number. **SEC-01 is now closed** — see the closure note appended
> to the SEC-01 item below. The original run-4/run-5 narrative above is retained verbatim, exactly as
> D-00c/D-00d require; nothing in it was deleted, only superseded in place.

- [x] **SEC-01**: The RustSec exception set is **one** set, every entry carries the governance the
      acceptance criteria demand, and the 2026-09-30 expiry has a disposition. **Four surfaces
      encode four different sets** (verified by direct file reads during run 4, and re-read in run
      5 — see the correction above, which withdraws the sync claim in point 3):

      1. `Epic_4/rustsec-remediation-plan.md` formally risk-accepts exactly **two** —
         `RUSTSEC-2023-0071` (`rsa 0.9.10`, Marvin timing side-channel, path
         `rsa → sqlx-mysql → sqlx → workspace crates`) and `RUSTSEC-2025-0111` (`tokio-tar 0.3.1`,
         PAX header file smuggling, path `tokio-tar → testcontainers → testcontainers-modules`),
         both "no fixed upgrade available" — with **owner Platform Security (Milestone 7)** and
         **review/expiry target 2026-09-30**.

      2. `.cargo/audit.toml` `[advisories] ignore` holds **five**: those two plus
         `RUSTSEC-2026-0187` (lopdf stack overflow via deeply nested PDF objects, transitive
         through `pdf-extract` under `content-processing`), `RUSTSEC-2026-0194` and
         `RUSTSEC-2026-0195` (quick-xml quadratic attribute parsing and unbounded namespace
         allocation, transitive through `rust-s3`/`aws-creds` under `s3`). Its comment block names
         two further IDs — `RUSTSEC-2026-0185` (quinn-proto) and `RUSTSEC-2026-0190` (anyhow) — as
         **upgraded rather than ignored**, so the file *mentions* seven and *suppresses* five.

      3. `deny.toml` `[advisories] ignore` holds **fifteen**: those five plus ten *unmaintained*
         notices (`RUSTSEC-2021-0139` ansi_term, `-2021-0141` dotenv, `-2022-0104` structopt,
         `-2024-0370` proc-macro-error, `-2024-0375` atty, `-2024-0436` paste, `-2025-0057` fxhash,
         `-2025-0119` number_prefix, `-2025-0121` gcc, `-2025-0134` rustls-pemfile). It *mentions*
         seventeen, the extra two again being -0185/-0190 in a comment. Its own header states "the
         same advisory IDs are mirrored here so cargo-deny and cargo-audit do not contradict each
         other. **Keep these two files in sync**" — **and run 5 verified that this invariant is
         satisfied**: the five vulnerability IDs match `.cargo/audit.toml` exactly, and the ten
         extra entries are a different advisory class (unmaintained, not vulnerability) carrying a
         documented rationale and explicit FR-1-step-5 authorisation. Run 4's contrary finding is
         withdrawn.

         **Corrected by Phase 9 (plan 09-07), dated 2026-08-08:** "fifteen" and "ten unmaintained"
         above are themselves stale by one. Re-read directly against the pre-Phase-9 tree
         (`ADR-0024`'s verbatim `Cargo.lock` liveness transcript), `deny.toml`'s `[advisories]
         ignore` array held **fourteen** entries and **nine** unmaintained notices, not fifteen and
         ten — `RUSTSEC-2025-0121` (`gcc`) was already absent from this file, from
         `.cargo/audit.toml`, and from `Cargo.lock`, with no record anywhere of its removal. Plan
         09-06 (this phase) has since deleted the four further entries whose parent crates Phase 8's
         clap v4 migration removed from the graph (`RUSTSEC-2022-0104` structopt, `-2021-0139`
         ansi_term, `-2024-0375` atty, `-2024-0370` proc-macro-error), leaving **ten** live entries
         (five vulnerability, five unmaintained) — see `SECURITY-EXCEPTIONS.md` and the SEC-01
         closure note below. Original text retained verbatim above.

      4. `.github/workflows/ci.yml` runs **two independent, differently configured `cargo audit`
         jobs**: `security-audit` at `:77` runs a bare `cargo audit` under a comment declaring
         `.cargo/audit.toml` "the single source of truth" (so: five), and `security` at `:406` runs
         `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` (so: two, inline).
         `Makefile:244-247` `make audit` is bare; `make security` = `audit` + `deny`;
         `cargo deny check` gates at `ci.yml:105`.

         **Corrected by Phase 9 (plan 09-07), dated 2026-08-08:** the `:406`/`:77` line citations
         above are stale — re-derived this session before deletion, the duplicate job actually sat
         at `ci.yml:465-482` (job id `security:` at `:466`, its `run:` line at `:482`), roughly
         seventy-seven lines below the cited range, and the surviving job was `security-audit:` at
         `ci.yml:61-78` (bare `cargo audit` at `:78`), not `:77`. Plan 09-06 (this phase) has since
         **deleted** the `security:` job entirely (`.github/workflows/ci.yml`, commit `cb75b2b`);
         `.github/rulesets/protect-main-branch.json:39`'s required status-check context string
         `"Security Audit"` still resolves via the surviving job. Any future citation into `ci.yml`
         for this subject must be re-derived against the current file rather than carried forward
         from either stale range.
      **Thirteen of `deny.toml`'s fifteen have no entry in the formal risk-acceptance register.**
      They carry one-line inline comments, which is documented reasoning but not the owner, expiry
      date, affected scope and compensating control that the plan's own acceptance criteria
      require. Note that **M10 Epic 2 FR-3's four-field schema does not require an owner or an
      expiry either** — it asks only for advisory ID, affected crate and why present, why not yet
      fixable, and revisit condition, all four of which every `.cargo/audit.toml` entry satisfies.
      Only `rustsec-remediation-plan.md` adds owner and expiry, and only for the original two. This
      is governance-surface drift on a repository that gates CI on both `cargo audit` and
      `cargo deny` — not undocumented risk-taking, but it needs an owner's decision. The plan's own
      open action items are also unclosed: two tracked impact-analysis issues, and "add
      `audit.toml` exception entries only if approved, each with expiry date and owner".

      **Corrected by Phase 9 (plan 09-07), dated 2026-08-08:** "Thirteen of fifteen" above compounds
      point 3's stale count. With the tree-verified pre-Phase-9 figure of fourteen entries (not
      fifteen) and the same two advisories carrying the formal risk acceptance, the correct
      pre-Phase-9 figure is **twelve of fourteen** lacking owner and expiry, not thirteen of fifteen.
      This gap is now closed for all ten surviving entries — see the SEC-01 closure note below.
      Original text retained verbatim above.
      **Done when** one register is authoritative and the other surfaces mirror it exactly; every
      suppressed ID carries owner, expiry, affected scope and compensating control; the two
      `cargo audit` CI jobs are reconciled to one configuration; and the 2026-09-30 acceptance is
      renewed with a new date, closed, or replaced.
      **Sequenced with HARD-06** — the `RUSTSEC-2026-0187` suppression rests on `pdf-extract` being
      reachable, which HARD-06 establishes. **Delegated to Phase 12 in two places:** the concrete
      CI reconciliation is **SUPPLY-01** (delete `ci.yml:389-406`, one deletion) and the corrected
      governance scope is **SUPPLY-02** (ratify or remove the three unauthorised 2026 vulnerability
      ignores; extend the FR-3 schema with owner and expiry; backfill thirteen). SEC-01 remains the
      requirement of record for the whole exception set and for the 2026-09-30 disposition.
      (**Corrected by Phase 12 (plan 12-01), dated 2026-08-09, citing `ci.yml:465-482` and commit `cb75b2b`:**
      the `ci.yml:389-406` citation two paragraphs above never held the duplicate job — it was
      re-derived at `ci.yml:465-482` and deleted by Phase 9's plan 09-06 in commit `cb75b2b`, so the
      line citation was already stale before Phase 9 touched anything. The deletion this clause
      delegates to SUPPLY-01 is done — see SUPPLY-01's own "Verified by Phase 12" closure block
      above.)
      *Derives: REQ-rustsec-risk-acceptance, REQ-rustsec-hardening-actions;
      `intel/code-verification.md` run-4 verified-open item 1 **as corrected by run-5 findings 1
      and 2**; INGEST-CONFLICTS run-4 warning on the RustSec exception list and the two run-5
      warnings that narrow it. **The only item in the 199-document corpus carrying an expiry
      date.***

      **CLOSED by Phase 9 (plan 09-07), dated 2026-08-08.** One register is now authoritative:
      `SECURITY-EXCEPTIONS.md` (repo root, authored by plan 09-02, commits `a587e5a` feat +
      `7ee741c` docs) holds exactly ten fully-governed rows (five `vulnerability`, five
      `unmaintained`), every row carrying all eleven governance fields non-empty — `id`, `class`,
      `crate`, `path`, `why_present`, `why_not_fixable`, `owner` (`DF3NDR` on all ten), `review_date`
      (`2026-12-31` on all ten), `scope`, `compensating_control`, `revisit_condition`.
      `.planning/decisions/0024-rustsec-exception-governance.md` (ADR-0024) records the governance
      architecture, the M10 Epic 2 FR-3 schema supersession, the ratification of the three 2026
      vulnerability advisories, the owner reassignment, and the expiry renewal. Plan 09-06
      (commits `6513cb7` fix, `9cef391` feat, `cb75b2b` fix) reconciled `deny.toml` and
      `.cargo/audit.toml` to exactly those ten live suppressions (deleting the four dead entries
      confirmed by `grep -c '^name = "<crate>"$' Cargo.lock` returning `0` for `structopt`,
      `ansi_term`, `atty`, `proc-macro-error`), landed `scripts/check-advisory-register.sh` (a
      three-clause guard demonstrated failing nine distinct ways in `09-06-SUMMARY.md`), and deleted
      the duplicate `ci.yml` `security:` job so exactly one `cargo audit` invocation remains
      (`grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1`), confirmed against
      `.github/rulesets/protect-main-branch.json:39`'s required context string `"Security Audit"`
      before deletion. The 2026-09-30 acceptance is **renewed**, not closed — every register row now
      carries its own `review_date` of `2026-12-31`, owner `DF3NDR` (the repository owner), replacing
      the closed-milestone label "Platform Security (Milestone 7)". **`cargo audit` and
      `cargo deny check` themselves were NOT run against the reconciled configuration in this
      environment** — `crates.io` returns HTTP 403 here, so neither tool is installable; their
      pass/fail verdict against the reconciled `deny.toml`/`.cargo/audit.toml` is CI-only evidence,
      recorded as unverified-here rather than inferred (see plan 09-07's CI-only claims list).
      Sequencing with HARD-06 is resolved without waiting for Phase 10: `RUSTSEC-2026-0187`'s
      suppression is warranted on tree evidence alone (`crates/paladin-content/Cargo.toml:41`
      declares `pdf-extract` unconditionally while `:18`'s `pdf = []` gates nothing), independent of
      how HARD-06 answers the capability question — HARD-06 receives this finding as input and is
      not itself answered here. SUPPLY-01 and SUPPLY-02 (Phase 12) are recorded closed by this
      phase's execution in the Phase 12 hand-off block below.

- [x] **SEC-02**: The project's licence posture has one answer and the manifests declare it. Three
      positions are live: `Epic_4/license-compatibility-decision-checklist.md` records
      **`MIT OR Apache-2.0`** with approver **`DF3NDR` (repository owner)**, approval date
      **2026-05-28**, an inventory of 551 packages with zero unknown entries, and an explicit
      acceptance of MPL-2.0 for unmodified use; M7 Epic 4 PRD §4.7.7 and the M7 overview AC 1 say
      **MIT**; the shipped root `Cargo.toml` declares `license = "MIT"`. The dual-licence approval
      rule ("any SPDX expression containing a permissive MIT/Apache branch is acceptable by
      default") was the stated basis for accepting `r-efi 5.3.0`'s
      `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; if the project is MIT-only that rationale is weaker
      than recorded, and a 551-package sign-off rests on a policy the manifests do not declare. Note
      that the *enforcement* surface already follows the checklist: `deny.toml [licenses] allow` is
      permissive-only with eight narrowly-scoped per-crate MPL-2.0 exceptions.
      **Done when** the root package and all ten library crates declare the same licence
      expression, `deny.toml` agrees, and either the checklist or the PRD is marked superseded. This
      is a sign-off artefact with a named approver — it must not be resolved by inference.
      *Derives: REQ-license-policy-signoff, REQ-crate-metadata-completion; INGEST-CONFLICTS run-4
      warning on the three-way licence posture.*

- [x] **SEC-03**: crates.io package-name collisions are caught before they cost a release cycle.
      `Epic_4/deferred-paladin-ports-publish-verification.md` closes Task 5.5 as **Resolved** and
      leaves exactly one residue: "Keep CI/package guardrails that detect crates.io package-name
      collisions early." Collisions were not hypothetical — they cost Epic 4 two package renames
      (`paladin-core` → `paladin-ai-core`, root → `paladin-ai`, which is why every crate's package
      name and lib name now diverge) and a full NO-GO cycle. Today the earliest guard is
      `ci.yml:617` `publish-dry-run` running `cargo publish --workspace --dry-run`, with
      `release.yml:410` doing a per-crate dry run inside the release job itself — both late, and
      neither name-specific.
      **Done when** a name-availability check runs earlier than dry-run, **or** reliance on the dry
      run is recorded as an accepted decision with its known cost. A decision either way closes it.
      *Derives: REQ-paladin-ports-publish-verification-closed, REQ-ci-publish-dry-run-v1/-v2;
      `intel/code-verification.md` run-4 verified-open item 6.*

      **CLOSED by Phase 9 (plan 09-07), dated 2026-08-08.** A name-availability check now runs on
      every pull request, earlier than either dry run: plan 09-04 (commits `264721a` feat,
      `2758a9d` feat, `5cde208` docs) added `.crate-names.txt` (the committed, hand-edited allow-list
      of the eleven package names this project owns on crates.io) and
      `scripts/check-crate-names.sh`, a bidirectional set-equality guard wired into the required
      `cargo-deny` CI job (`License & Dependency Policy`) and `make check-crate-names`. The guard was
      demonstrated failing six distinct ways (unlisted tree name, stale allow-list entry, emptied
      allow-list, missing allow-list, case-only variant, and a `publish = false` exemption-flip) in
      `09-04-SUMMARY.md`, and is offline by design — `crates.io` returns HTTP 403 in this
      environment, so it cannot be a live registry query. `.planning/decisions/0026-crate-name-collision-guard.md`
      (ADR-0026) records the decision and states the accepted residual cost explicitly: the guard
      catches collisions among the eleven names this project already owns; a genuinely *novel* crate
      name is still a human check against crates.io, not a CI one. That residual cost is the "accepted
      decision with its known cost" branch of this requirement's own done-condition, recorded rather
      than left implicit. ADR-0026 also closes the residue named in
      `deferred-paladin-ports-publish-verification.md` ("Keep CI/package guardrails that detect
      crates.io package-name collisions early") — Phase 10 / HARD-01's ledger row should cite this
      ADR as satisfying it.

- [x] **SEC-04**: `crates/paladin-herald/CHANGELOG.md` exists, or an exemption is recorded. M7
      Epic 4 §4.3.1 and AC 3 require a Keep-a-Changelog `CHANGELOG.md` for **every** public crate,
      and `epic-4-completion-summary.md` records that criterion **Met** ("Per-crate changelogs
      created/backfilled"). Verified 2026-07-30: nine of ten library crates have both a `README.md`
      and a `CHANGELOG.md`; `crates/paladin-herald/` has the README only. The crate was created
      *after* Epic 4 closed, by reconciliation commit `66f6c4e`, so the audit that marked the
      criterion Met never covered it. Small, but it is a release-gate criterion on a published crate
      family.
      *Derives: REQ-per-crate-changelog, REQ-crate-metadata-completion, REQ-release-readiness-audit;
      `intel/code-verification.md` run-4 verified-open item 2.*

- [x] **SEC-05**: `Dockerfile.chef`'s planner stage cannot silently go stale as crates are added.
      M7 Epic 2 FR-01 requires all `crates/*/Cargo.toml` files in the planner stage, and §6 states
      the purpose: "the dependency layer only invalidates when a `Cargo.toml` changes". Verified:
      `Dockerfile.chef:25-33` enumerates exactly nine crate manifests by name — core, ports,
      battalion, llm, memory, storage, notifications, content, web — omitting
      `crates/paladin-herald/Cargo.toml`. The later `COPY crates ./crates` at `:36` means the image
      still builds, so nothing fails; the cache-tightness FR-01 exists to deliver is simply not
      achieved for `paladin-herald`.
      **Done when** the herald manifest is covered **and** the mechanism cannot miss an eleventh
      crate — an enumerated list that goes stale on every crate addition is the defect, not just the
      one missing line.
      *Derives: REQ-docker-workspace-build; `intel/code-verification.md` run-4 verified-open
      item 3.*

      **CLOSED by Phase 9 (plan 09-07), dated 2026-08-08 — closed by deletion, not by addition.**
      Plan 09-03 (commits `52b1943` fix, `d1ae033` docs) deleted `Dockerfile.chef:25-33`'s nine
      per-crate `COPY crates/paladin-*/Cargo.toml` lines entirely, rather than adding
      `paladin-herald`'s tenth line: `grep -c 'COPY crates/paladin' Dockerfile.chef` now returns `0`,
      and the pre-existing `COPY crates ./crates` instruction (unchanged, now the sole coverage
      mechanism) is structural — it cannot miss an eleventh crate the way an enumerated list can.
      `grep -c 'COPY crates/paladin' Dockerfile Dockerfile.server` also returns `0` for both,
      confirming SEC-05 closes across the whole Docker surface, not `Dockerfile.chef` alone.
      `.planning/decisions/0027-dockerfile-chef-planner-stage.md` (ADR-0027) records the M7 Epic 2
      FR-01 supersession, citing cargo-chef's own upstream documentation for why the deleted
      enumeration never delivered the cache-tightness FR-01 named (a strictly later full-tree `COPY`
      already dominated the cache decision for all ten crates before this edit). **The caching claim
      itself is recorded as established-from-cargo-chef-documentation, not measured** — Docker is
      absent from this environment, so the builder-stage `cargo chef cook` layer reporting `CACHED`
      on a source-only rebuild is CI-only evidence, stated as such in ADR-0027 and in plan 09-07's
      CI-only claims list, never inferred as passing.

#### Hand-off to Phase 10 / HARD-01 — dated 2026-08-08 (plan 09-07)

**No Milestone 7-8 as-shipped ledger exists yet — HARD-01 (Phase 10) builds it, per D-20
(`09-CONTEXT.md`).** Because Phase 9 runs before Phase 10, there is no ledger row here to amend in
place; the *closest available surrogate* — the Milestone 7 Epic 4 ledger table rows above — has
already been amended with dated "Closed by Phase 9" notes. This block is the explicit, additional
hand-off D-20 requires: when HARD-01 builds the Milestone 7-8 ledger, it must record each of the
following seven `REQ-*` rows as **already closed by Phase 9**, citing this phase's ADRs and commits,
rather than re-verifying or re-planning them:

1. **`REQ-rustsec-risk-acceptance`** — closed by SEC-01 / ADR-0024. `SECURITY-EXCEPTIONS.md` is the
   one register for all ten live suppressions; the 2026-09-30 acceptance is renewed to per-advisory
   `2026-12-31` review dates, owner `DF3NDR`.

2. **`REQ-rustsec-hardening-actions`** — closed by SEC-01 / ADR-0024. The "approved `audit.toml`
   entries with owner and expiry" action item is satisfied by the register's ten fully-governed
   rows; post-mitigation `cargo audit`/`cargo deny check` re-audit evidence remains CI-only (HTTP 403
   against crates.io in this environment).

3. **`REQ-license-policy-signoff`** — closed by SEC-02 / ADR-0025. The root package and all ten
   library crates declare `MIT OR Apache-2.0`, matching the signed 551-package checklist; the PRD's
   single-licence claim is annotated superseded.

4. **`REQ-crate-metadata-completion`** — closed by SEC-02 and SEC-03 / ADR-0025 and ADR-0026. The
   licence field agrees with the checklist across all eleven manifests, and a crates.io
   name-collision guard now runs on every pull request.

5. **`REQ-per-crate-changelog`** — closed by SEC-04. All ten library crates have a `CHANGELOG.md`,
   mechanically enforced by `scripts/check-changelogs.sh`.

6. **`REQ-docker-workspace-build`** — closed by SEC-05 / ADR-0027. `Dockerfile.chef`'s planner-stage
   crate coverage is structural (`COPY crates ./crates`), not an enumerated list that can go stale.

7. **`REQ-paladin-ports-publish-verification-closed`** — closed by SEC-03 / ADR-0026. The residue
   named in `deferred-paladin-ports-publish-verification.md` ("keep CI/package guardrails that
   detect crates.io package-name collisions early") is satisfied by `scripts/check-crate-names.sh`.

**Evidence for all seven:** `SECURITY-EXCEPTIONS.md`; `.planning/decisions/0024-rustsec-exception-governance.md`,
`0025-licence-posture.md`, `0026-crate-name-collision-guard.md`, `0027-dockerfile-chef-planner-stage.md`;
commits `0458b6a`, `a587e5a`, `7ee741c`, `52b1943`, `d1ae033`, `264721a`, `2758a9d`, `5cde208`,
`6bf860f`, `74a05fe`, `6513cb7`, `9cef391`, `cb75b2b` (plans 09-01 through 09-06). None of these seven
rows requires further code work; HARD-01's task is to cite them, not re-open them.

### Milestone 7-8 ground truth & recorded account (HARD)

- [x] **HARD-01**: The *Milestone 7-8 as-shipped ledger* below is upgraded from component-level file
      evidence to per-criterion verdicts with `file:line` citations, for all **86** run-4
      requirement IDs. Must carry four dispositions, not two: `Shipped`, `Superseded by outcome`
      (the ~~14-row~~ **Corrected (dated 2026-08-08, HARD-01):** 13-row — confirmed via `sed -n
      '365,381p' .planning/intel/code-verification.md | grep -c '^|'` → 15 lines (1 header + 1
      separator + 13 data rows) — table in `intel/code-verification.md` — requirements that must **not** be planned
      as written), `Relocated` (deliverables that exist at a different path, chiefly the mdbook), and
      `Deferred with register` (the D1-D5 items and the two removed features). Must also record that
      **the five crates ARCH-01 marked "provenance pending" now have one**: `paladin-storage`,
      `paladin-notifications`, `paladin-content` and `paladin-web` from M7 Epic 1's extraction PRD
      and its cost-benefit gate, and `paladin-herald` from the 2026-06-04 reconciliation rather than
      from any PRD — which is the reason no ingested requirement described it before run 4, and the
      reason the earlier "9-crate workspace" figure was wrong.
      *Derives: all 86 run-4 `REQ-*` IDs; `intel/code-verification.md` run-4 section; narrows
      ARCH-01's pending-provenance clause.*
      **Closed, dated 2026-08-08 (plan 10-11):** `.planning/ledgers/milestone-07-08.md` closes this
      requirement — 86 `REQ-*` rows across 12 sections, re-verified this session:
      `grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md` → `86`,
      `grep -o '^| REQ-[a-z0-9-]*' .planning/ledgers/milestone-07-08.md | sort -u | wc -l` → `86`,
      `grep -c '^### ' .planning/ledgers/milestone-07-08.md` → `12`, zero `pending — plan` markers,
      zero blank Verdict/Evidence cells. The four HARD-01 dispositions are unmissable via the
      ledger's head-of-file 13-row "Superseded by outcome" summary table. The five previously
      provenance-pending crates carry their provenance in the ledger's head note (D-06). See
      `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-01-SUMMARY.md` through
      `10-10-SUMMARY.md` for the per-section derivation record.

- [x] **HARD-02**: `facade-cleanup-RECONCILIATION-2026-06-04.md` is recorded as **the authoritative
      account of Milestone 8**, superseding `Epic_1/facade-audit.md` and
      `Epic_3/infrastructure-adapter-disposition.md`. The record must carry:
      **(a) why it supersedes them** — both describe ~4,400 LOC of *orphaned, uncompiled duplicate
      files* as "active bridges that stay". The reconciliation's verification method is stated and
      reproducible (`rg "mod <name>"` across `src/` returns nothing for each; the directory's
      `mod.rs` only does `pub use paladin_<crate>::…`; the leaf-crate file exists) and the tree
      confirms every relocation target;
      **(b) that it executed relocations Epic 3 had deferred to Milestone 9**, so Epic 3 is complete
      *in substance*, not punted — 15 commits, ~10,250 net LOC removed, one new leaf crate;
      **(c) three in-execution corrections to the audit that must survive into the record** —
      `paladin_registry.rs` was **not** a duplicate (the facade's 418-LOC impl was richer than
      battalion's 67-LOC `pub(crate)` copy, so the richer one was consolidated *into* battalion
      rather than deleted blindly); `sqlite_*_repository.rs` were **not** redundant (they were the
      active default-build impl, resolved by making `paladin-storage` non-optional); the rest —
      `mysql_content_repository.rs`, the `input/*` fetchers, `document/*`,
      `output/api_content_deliverer.rs`, `error_log_adapter.rs` — genuinely were orphaned;
      **(d) that Epic 6 is complete** despite the reconciliation recording it "Not verified; low
      priority" and `deferred-items.md` omitting it — `crates/paladin-content/src/services/` ships,
      `lib.rs` declares `pub mod services;`, and a workspace-wide grep for `use_cases` across
      `src/`, `crates/`, `tests/`, `examples/` and `benches/` returns **zero** matches. Do not plan
      Epic 6 as outstanding;
      **(e) that `paladin-herald` exists**, created inside an Epic whose §5 Non-Goals state "No new
      crates created. `paladin-herald`, `paladin-ml`, etc. are not in scope" — the non-goal names
      the exact crate that was then created, in the same milestone. Record the non-goal as
      **overridden for `paladin-herald` and still holding for `paladin-ml`**, which is the
      distinction FACADE-03 depends on.
      *Derives: REQ-m8-reconciliation-relocations, REQ-facade-audit-document,
      REQ-adapter-disposition-record, REQ-m8-epic3-no-extractions, REQ-paladin-content-services-rename;
      `intel/code-verification.md` run-4 "Claims contradicted by code in the favourable direction";
      INGEST-CONFLICTS run-4 warnings on the disposition record and the new-crate non-goal.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0028
      (`.planning/decisions/0028-m8-reconciliation-authoritative.md`), `conforms`, names the
      reconciliation authoritative, naming FACADE-02, FACADE-03(b) and FACADE-04 downstream.
      `facade-audit.md` and `infrastructure-adapter-disposition.md` each carry a dated 2026-08-08
      `SUPERSEDED BY [ADR-0028]` banner (`grep -c 'SUPERSEDED BY \[ADR-0028\]'` on each file → `1`).
      The ledger's Milestone 8 Epic 1 and Epic 3 sections carry the do-not-re-delete markers on
      `REQ-storage-shim-deletion` and `REQ-garrison-sanctum-bridges-kept`. See `10-02-SUMMARY.md`
      and `10-09-SUMMARY.md`.

- [x] **HARD-03**: The version trajectory is recorded as **history**, and no `v0.1.0-rc.1` artefact
      is treated as current state. What happened: M7 Epic 4's PRD and the overview Appendix C
      anchored the first publishable release at lockstep `0.2.0`; what shipped was **all ten crates
      published at `0.1.0`**, tagged **`v0.1.0-rc.1`** at commit `a9530fc` on 2026-05-28, with a
      release-readiness audit recording every gate PASS, a **GO** sign-off, and post-release
      verification confirming all ten crates resolve on docs.rs (including the package/lib split
      made visible at `docs.rs/paladin-ai-core/latest/paladin_core/` and
      `docs.rs/paladin-ai/latest/paladin/`) plus an external smoke project compiling against
      `paladin-ai = "0.1.0"`. Milestone 8 targeted v0.2.0 throughout; its Epic 7, written
      2026-06-06, targets "post-v0.5.1 (Unreleased)" — so v0.3.0 through v0.5.1 all shipped between
      Epic 5 and Epic 7, and the M8-11 dependency graph's v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 sequence
      completed. ~~Current tree: `Cargo.toml` `0.6.0`, branch `release/v0.7.0`, latest tag `v0.5.1`.
      **Feeds REL-01**, which converges the three-way version disagreement — and REL-01 must not
      converge on any rc.1 figure.~~ **Corrected (dated 2026-08-08, HARD-03):** `Cargo.toml:34` reads
      `version = "0.7.0"` (Phase 4 plan 04-05, commit `c2e20a1`, converged every manifest and internal
      pin on 0.7.0, per ADR-0008). `git tag --sort=-v:refname | head -3` (run this session) returns
      `v0.7.1`, `v0.7.0`, `v0.5.1` in that order — Milestone 1's close-out shipped `v0.7.1` on
      2026-08-04. The branch is still `release/v0.7.0`, unchanged. **`REL-01` is already `[x]`
      complete** (`REQUIREMENTS.md:360`, traceability row `REQUIREMENTS.md:3740` reads `Phase 4 |
      Complete`) — it converged on `0.7.0` via ADR-0008 and did **not** converge on any `rc.1` figure.
      HARD-03's "Feeds REL-01" clause has therefore already fired: this HARD-03 record is
      backwards-looking confirmation of closed history, not a hand-off to open work, and REL-01's own
      checkbox is left untouched.
      *Derives: REQ-versioning-policy, REQ-release-checklist, REQ-release-readiness-audit,
      REQ-crate-metadata-completion; `context.md` Topic: Version trajectory across runs 1-4.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0029
      (`.planning/decisions/0029-version-trajectory-history.md`), `conforms`, records `v0.1.0-rc.1`
      at commit `a9530fc` as closed history with a `## Trajectory` table Phase 13/ORCH-05 extends.
      `REQUIREMENTS.md:360` confirms REL-01 is still `[x]` and was not re-opened; its traceability
      row reads `Phase 4 | Complete`. See `10-03-SUMMARY.md`.

- [x] **HARD-04**: The **fourth** milestone-numbering collision is recorded with the same convention
      that closes the first three. The Milestone 7 overview titles itself "Milestone 4: Production
      Hardening and Extended Workspace Decomposition" while its path is
      `Milestone_7-Production-Hardening`, and its Prerequisites credit "Milestones 1-3" with work
      the directory numbering assigns to Milestones 4-6 (feature flags and CI matrix; core workspace
      crates; `application_settings.rs` decomposition; manager-service relocation; Maneuver DSL
      co-location; `CircuitBreaker` relocation). Directory / task-list numbering is authoritative,
      as in VERIFY-03 and ARCH-02. Roadmap Extension Protocol item 8 predicted a third instance;
      this is it. Expect a fifth in run 5.
      *Derives: `context.md` Topic: Milestone 7 scope, structure and self-numbering; same defect
      class as ARCH-02 and VERIFY-03.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0030
      (`.planning/decisions/0030-milestone-7-self-numbering.md`), `conforms`, citing ADR-0010 and
      ADR-0014, records the Roadmap Extension Protocol's predicted fifth instance discharged by this
      fourth one. The Milestone 7 overview carries a dated Shape B banner beneath its title plus six
      inline Prerequisites corrections
      (`.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`).
      See `10-03-SUMMARY.md`.

- [x] **HARD-05**: The extracted-crate dependency rule has **one** stated form. M7 Epic 1 PRD §6.1
      states it absolutely — "No extracted crate may depend on another extracted crate or on the
      `paladin` facade" — and Goal 2 restricts each new crate to `paladin-core`, `paladin-ports` and
      workspace-shared dependencies. ~~The same PRD's §4.4 complexity assessment anticipated the
      violation without amending the rule: "use-case services depend on `paladin-llm` for LLM
      analysis, creating an inter-crate dependency that must be handled carefully".~~
      **Corrected (dated 2026-08-08, HARD-05):** this complexity-assessment sentence does not resolve at "the same
      PRD's §4.4" — `prd-extract-infrastructure-crates.md` §4.4 is Task 1.4 and contains no mention of
      `paladin-llm`. The sentence lives in the sibling document
      `.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md:118`, inside the
      `paladin-content` row's `Extraction complexity` cell, and reads: "use-case services depend on
      `paladin-llm` for LLM analysis, creating an inter-crate dependency that must be handled
      carefully" — the quoted claim itself is unchanged, only its citation. Verified:
      `crates/paladin-content/Cargo.toml` declares
      ~~`paladin-llm = { version = "0.6.0", path = "../paladin-llm", optional = true }`~~
      **Corrected (dated 2026-08-08, HARD-05):** `paladin-llm = { version = "0.7.0", path =
      "../paladin-llm", optional = true }` at `crates/paladin-content/Cargo.toml:28` — converged by Phase 4's commit
      `c2e20a1` — behind its
      `llm` feature, and the facade's `content-processing` enables `paladin-content/llm`. This is
      the invariant that keeps the extraction from re-creating the coupling it was built to remove;
      stated absolutely and violated once, it is unclear whether the rule is "never" or "never,
      except behind an optional feature". **Strongest SPEC candidate in run 4** — if it should bind
      above PRD precedence, re-tag `prd-extract-infrastructure-crates.md` via `--manifest` and
      re-run ingest. **The answer gates FACADE-02**, because D2/D3/D4's proposed relocation targets
      create exactly this class of edge.
      *Derives: REQ-extracted-crate-dependency-rule, REQ-paladin-content-extraction;
      INGEST-CONFLICTS run-4 warning on the dependency rule; `constraints.md` run-4 strongest SPEC
      candidate.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0031
      (`.planning/decisions/0031-extracted-crate-dependency-rule.md`), `conforms`, restates the rule
      as a default-build invariant, checkable via `cargo tree -p paladin-content
      --no-default-features` (re-run this session, zero occurrences of any extracted crate or the
      facade). `prd-extract-infrastructure-crates.md` Goal 2 and §6.1 carry the two dated
      corrections naming ADR-0031. The ledger's `REQ-extracted-crate-dependency-rule` row reads
      `satisfied`, citing ADR-0031, with the wording-not-code explanation stated in the row. See
      `10-04-SUMMARY.md` and `10-07-SUMMARY.md`.

- [x] **HARD-06**: Whether PDF extraction is still a supported capability has one answer. Three
      facts point in two directions. `crates/paladin-content/Cargo.toml:18` declares `pdf = []` — a
      feature gating **no dependency at all**, so `pdf-extract` is not reachable through it. The
      facade's `content-processing` enables `web-scraping`, `rss`, `news-api`, `tiktoken`, `llm`
      and `paladin-memory/content-processing` — five of six capability features, omitting `pdf`,
      against M7 Epic 1 §4.4.6's requirement that it activate `paladin-content` "with **all**
      capability features enabled" and §4.4.1's requirement that `pdf` gate `pdf-extract`. Yet
      `.cargo/audit.toml` suppresses `RUSTSEC-2026-0187` on the stated grounds that "lopdf is
      transitive via `pdf-extract` (optional `content-processing`)", which asserts `pdf-extract`
      **is** in the graph. PDF extraction was one of the four subsystems the extraction epic was
      built around (~350 LOC `pdf_extractor.rs`, called out in the overview background and the
      cost-benefit assessment). **SEC-01 cannot be reconciled honestly until this is settled** — if
      `pdf-extract` is unreachable, the -0187 suppression is unnecessary rather than accepted.
      *Derives: REQ-paladin-content-extraction, REQ-content-processing-build-gate,
      REQ-rustsec-risk-acceptance; INGEST-CONFLICTS run-4 warning on the `content-processing`
      feature.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0032
      (`.planning/decisions/0032-pdf-extraction-capability.md`), `must change`, executed in this
      phase. `grep -cE '^pdf +=' crates/paladin-content/Cargo.toml` → `0`;
      `grep -cE '^news-api +=' crates/paladin-content/Cargo.toml` → `1` (the retained comparator).
      `.cargo/audit.toml:26-29`'s `RUSTSEC-2026-0187` comment now states the true reachability path,
      with the suppression itself untouched. `crates/paladin-content/CHANGELOG.md` records the
      consumer-visible cost under `### Removed`, citing ADR-0032. See `10-05-SUMMARY.md`.

- [x] **HARD-07**: One `cargo doc` bar, applied consistently. M7 Epic 4 §4.4.3 and M7 Epic 1 §4.6.4
      / §8.9 require `cargo doc --workspace --no-deps` to complete with **zero warnings**; M8 Epic 5
      FR-19 requires only exit 0 with "**warnings acceptable; must not fail**". The same command is
      a zero-warning gate in one milestone and a warnings-tolerated gate in the next. Combined with
      ~~`crates/paladin-ports/Cargo.toml` still setting `[lib] doctest = false` and `ci.yml:225`
      excluding the crate from `--doc` (DEBT-03)~~, and with M7 Epic 4 §4.4.1's
      `#![warn(missing_docs)]` and §4.4.4's >90% documented-public-item coverage target — both
      recorded **Met** by `epic-4-completion-summary.md` — the documentation bar the project
      actually holds itself to is ambiguous. **Resolve alongside DEBT-03**, not separately: the
      "Task 7.0" doctest re-enable and the gate bar are the same question asked twice.
      **Corrected (dated 2026-08-08, HARD-07):** `crates/paladin-ports/Cargo.toml` has **no `[lib]`
      section at all** — `git log --oneline -- crates/paladin-ports/Cargo.toml` shows `2bffe22
      feat(08-03): re-enable paladin-ports doctests` — and `.github/workflows/ci.yml:238` is a bare
      `cargo test --workspace --doc`, carrying no `--exclude` of any kind (the `:225` citation is
      stale by both line number and content). DEBT-03 and the "unwritten Task 7.0" are therefore
      **discharged by Phase 8**, not by this phase. The surviving residue is one line in the
      `Makefile`'s `release-check` target (`Makefile:432-433`, `--exclude paladin-ports` plus a stale
      echo explaining it) — weaker than CI, the wrong direction for a release gate — and plan 10-06
      executes that fix.
      *Derives: REQ-doc-coverage-audit, REQ-crate-metadata-completion, REQ-m8-final-quality-gate;
      INGEST-CONFLICTS run-4 warning on the two `cargo doc` bars; extends DEBT-03.*
      **Closed, dated 2026-08-08 (plan 10-11):** ADR-0033
      (`.planning/decisions/0033-cargo-doc-warning-bar.md`), `must change` for the `Makefile` fix
      only, executed in this phase. `Makefile` greps: `grep -c 'exclude paladin-ports' Makefile` →
      `0`, `grep -c 'not yet published' Makefile` → `0`. ADR-0033 records the measured warning
      state — exit 1, 20 warnings across four crates (`paladin-web` 13, `paladin-battalion` 3,
      `paladin-ai` 3, `paladin-herald` 1) — with Phase 16 / DOCS-03 named owner. FR-19
      (`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md`)
      carries its dated correction. **The `cargo doc --workspace --no-deps` gate itself is still red
      today** — this requirement closes because the bar is now recorded once with its measured
      state and a named owner, not because the tree clears it. See `10-06-SUMMARY.md`.

#### Hand-off to Phase 11 / FACADE-02 — dated 2026-08-08 (plan 10-11)

**FACADE-02 inherits ADR-0031's restated default-build invariant and ADR-0028's record that the
Epic 3 relocations already executed — do not re-derive either.**

1. **The invariant that legalises D2/D3/D4's leaf-to-leaf relocation targets** is ADR-0031's
   restated form: no extracted crate may depend on another extracted crate or on the `paladin`
   facade *in its default build*; a non-default optional feature may declare such an edge only
   where the facade opts in explicitly and the dependent code is `cfg`-gated. D3's proposed
   `paladin-battalion`/`paladin-llm` targets and D4's `paladin-content` target are legal under this
   form on exactly the same terms `paladin-content`'s existing `llm` feature already satisfies —
   FACADE-02 does not need to re-litigate whether a leaf-to-leaf edge is permissible at all, only
   whether each specific proposed edge is non-default, facade-gated and `cfg`-scoped.

2. **The Epic 3 relocations are already executed, inside Milestone 8, not deferred to Milestone 9.**
   ADR-0028 `## Decision (iii)` records 15 commits and net 10,252 LOC removed
   (`e5b2011~1..a1e4901`), independently re-measured twice (plans 10-02 and 10-09). FACADE-02's own
   candidate list must not re-plan any relocation this range already performed.

**Evidence:** `.planning/decisions/0031-extracted-crate-dependency-rule.md` (`conforms`,
`Downstream Consumers` names Phase 11 / FACADE-02 explicitly);
`.planning/decisions/0028-m8-reconciliation-authoritative.md` `## Decision (iii)`;
`.planning/ledgers/milestone-07-08.md`'s `REQ-m8-epic3-no-extractions` and
`REQ-extracted-crate-dependency-rule` rows.

#### Hand-off to Phase 11 / FACADE-03(b) — dated 2026-08-08 (plan 10-11)

**FACADE-03(b) inherits the Epic 3 §5 non-goal split ADR-0028 records: overridden for
`paladin-herald`, still holding for `paladin-ml`.**

1. The M8 Epic 3 PRD §5 non-goal "No new crates created — `paladin-herald`, `paladin-ml`, etc. are
   not in scope" names the exact crate that was then created, in the same milestone. `ls crates/`
   (re-run this session) returns `paladin-herald` present, `paladin-ml` absent
   (`test -d crates/paladin-ml` exits `1`).

2. **The directory evidence for both halves:** `crates/paladin-herald/` exists (created by
   reconciliation commit `66f6c4e`), overriding the non-goal for that crate specifically, not for
   the non-goal as a class. `paladin-ml` remains absent — the TensorFlow adapter and its `ml`
   feature were deleted outright (commit `3d48768`), not gated into the facade, and the
   reintroduction condition (`REQ-deferred-tensorflow-ml-adapter-v3`) requires a dedicated
   `paladin-ml` leaf crate, never a return to the facade.

3. FACADE-03(b) must not treat `paladin-herald`'s existence as licence to create `paladin-ml`
   without its own decision — the two halves of the split are independent.

**Evidence:** `.planning/decisions/0028-m8-reconciliation-authoritative.md` (`Downstream Consumers`
names FACADE-03(b) explicitly); `.planning/ledgers/milestone-07-08.md`'s
`REQ-deferred-tensorflow-ml-adapter-v3` row (`deferred with register`).

### Facade residue & deferred register disposition (FACADE)

`deferred-items.md` and `deferred-features.md` are the two most reliable documents in the corpus by
measurement: D5's claim of 17 `println!`/`eprintln!`/`dbg!` occurrences across 6 files matches the
tree **exactly**, as does every other verifiable claim in either register. They — not checkbox
arithmetic — are the Milestone 8 forward-work source.

- [x] **FACADE-01**: D5 is closed. `grep -rn "println!\|eprintln!\|dbg!" src/application/services/
      src/infrastructure/` returns **exactly 17 occurrences across exactly 6 files**:
      `services/herald/herald_registry.rs`, `services/paladin/paladin_execution_service.rs`,
      `infrastructure/adapters/arsenal/{mcp_protocol,tool_result_formatter}.rs`,
      `infrastructure/adapters/scheduling/tokio_cron_adapter.rs` and
      `infrastructure/resilience/circuit_breaker.rs` — down from ~435 across 36 files before commit
      `4c7857e`. The register rates it low effort / low risk and names it the quick win. Scope is
      `services/` + `infrastructure/` only; `cli/` stdout is intentional and out of scope.
      **Done when** each of the 17 is either converted to `log::*` or annotated as deliberate
      stdout, with the disposition recorded per file.
      **Corrected 2026-08-08 (plan 11-05):** No conversion is possible or required. Re-measured:
      `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` returns
      exactly **17** occurrences across exactly **6** files, and every one of the 17 is a `///` or
      `//!` doc-comment line inside a fenced code block — a rustdoc example, not runtime library
      stdout. The same grep filtered to non-doc-comment lines
      (`grep -v '///' | grep -v '//!'`) returns **0**. All 17 therefore resolve to the annotate
      branch and the conversion branch above is not available. See
      `.planning/registers/facade-01-rustdoc-stdout-disposition.md`, which records all 17 as
      deliberate rustdoc-example stdout with a per-occurrence disposition, and the amended
      `.planning/ROADMAP.md` §Phase 11 criterion 1, which states the same two commands and the same
      17/0 figures word for word.
      *Derives: REQ-m8-deferred-items-register (D5); `intel/code-verification.md` run-4 — count
      verified exact.*

- [x] **FACADE-02**: D1-D4 each carry a **disposition with an owner**, not an effort rating. The
      register states ratings and recommendations; it names no owners and assigns no target
      milestone. Each must resolve to *do* / *defer with a stated trigger* / *withdraw*:
      **D1 — `src/core/` re-export shims (currently "KEEP, by decision").** Verified: `src/core/`
      is exactly six files (`mod.rs`, `platform/mod.rs`, `platform/manager/{mod,content_service,
      event_manager,user_service}.rs`) and **49 facade files** import via `crate::core::…`. Removal
      means a mechanical path rewrite of those 49 plus preserving `platform/mod.rs`'s
      maneuver/parser path injection, which carries real logic rather than re-exports. Interacts
      with **ARCH-04** — if the facade adopts a "no re-export aliases" policy, D1 follows from it.
      **D2 — mis-layered `src/core/platform/manager/` services.** `content_service.rs`,
      `event_manager.rs` and `user_service.rs` are application/domain services, not composition
      glue. The Epic 1 audit recommends `ContentItemService` → `paladin-core`, `EventService` →
      `paladin-core` or a facade app-service, and `user_service` **split** (trait + DTOs →
      `paladin-core`/`paladin-ports`, concrete impl → a facade app-service). Note two things that
      narrow it: reconciliation commit `6704807` already found "no user-service split was needed"
      for the controller case because `UserServiceTrait` and the DTOs **already live in
      `paladin-core`**; and the *full* `user_service` relocation is already carried as a run-3 v2
      tech-debt item. Do not plan it twice.
      **D3 — entangled Paladin services (currently "KEEP for now").**
      `src/application/services/paladin/{planning_service,prompt_generation_service,
      temperature_service,handoff_service}.rs`, ~2,750 LOC, tightly coupled to `paladin_builder.rs`
      and `paladin_execution_service.rs`. Candidates for `paladin-battalion` (planning/handoff) and
      `paladin-llm` (prompt/temperature) — **both leaf-to-leaf edges, so gated on HARD-05.**
      **D4 — `content_ingestion_service.rs` placement.**
      `src/application/services/content/content_ingestion_service.rs`, ~1,211 LOC. M7 Epic 1's
      extraction PRD listed it as moving to `paladin-content`; the facade kept its own copy, and a
      move needs a dependency-coupling review first.
      The register's own suggested grouping stands as input: architecture pass → D2 plus optionally
      D4; only alongside a broader refactor → D3, and D1 only if a no-alias policy is adopted.
      *Derives: REQ-m8-deferred-items-register (D1-D4); gated on HARD-05; interacts with ARCH-04 and
      the run-3 v2 `user_service` deferral.*

- [x] **FACADE-03**: The two deliberately removed features have a recorded status, and their
      reintroduction conditions survive in `.planning/` rather than only in a DOC.
      **(a) The `paladin user …` CLI surface** — 1,065 LOC, eight subcommands (`register`, `login`,
      `get`, `update`, `list`, `activate`, `deactivate`, `verify`), removed because it was declared
      but **never dispatched**: no `UserCommands` arm existed in the binary's top-level match, so it
      compiled and did nothing. The backend is intact and in use elsewhere
      (`UserService`/`UserServiceTrait` and the request DTOs), so reintroduction is "mostly
      re-wiring, not new domain work" — add the enum arm plus a dispatch handler using the existing
      `config/user_config.rs` DI wiring, add tests, and **recover the module verbatim from git
      history at the Milestone 8 removal commit on branch `chore/facade-cleanup-m8-finish`** rather
      than rewriting. Verified: `src/application/cli/commands/` holds ten modules and `user.rs` is
      not among them.
      **Corrected 2026-08-08 (plan 11-05):** **One commit did both removals** — `3d48768`,
      2026-06-04, "chore(facade): remove half-built user CLI + tensorflow ML stub (M8)" — so the
      CLI removal above and the ML removal in (b) below are one event, not two, and should carry
      one pointer, not a branch for one and a commit for the other. The durable pointer is the
      immutable SHA in its runnable form `git show 3d48768^:src/application/cli/commands/user.rs`,
      because a branch ref is mutable and deletable at any time by anyone with push access, while a
      commit SHA is immutable once created. The branch state, recorded as it was actually measured
      rather than asserted: at planning time no *local* branch named
      `chore/facade-cleanup-m8-finish` existed in this checkout, but the *remote-tracking* ref
      `refs/remotes/origin/chore/facade-cleanup-m8-finish` did resolve and is an ancestor of
      `3d48768` — which contradicts the stronger "not present as a local or remote ref" framing this
      phase's own `11-CONTEXT.md` D-10 carried. See
      `.planning/registers/facade-03-removed-features.md` §1 for the full re-measurement.
      **(b) The TensorFlow ML adapter** — 636 LOC `#[doc(hidden)]` placeholder implementing
      `MlPort` with stub model loading and prediction, plus the `ml = []` flag; both removed
      outright (commit `3d48768`), verified absent from `Cargo.toml` and `src/`. **The load-bearing
      part is the placement condition**: the real adapter MUST be implemented in a dedicated
      `paladin-ml` **leaf crate** — "consistent with the hexagonal layout — ML inference is an
      infrastructure adapter, not facade code — rather than re-adding it to the facade" — with the
      `ml` flag moved onto that crate. `paladin_ports::input::ml_port::MlPort` remains in the
      workspace so the integration point is stable. This condition is carried **only by a DOC**, and
      it is the surviving half of the M8 Epic 3 non-goal that `paladin-herald` overrode (HARD-02e).
      **Done when** both are recorded in `.planning/` as deliberate deferrals with their conditions
      intact, **or** promoted to scope with a phase. Either outcome closes it; silent omission does
      not.
      *Derives: REQ-deferred-cli-user-commands, REQ-deferred-tensorflow-ml-adapter-v3
      (and its `-v1`/`-v2` predecessors, variant group 24); `intel/code-verification.md` run-4 —
      both removals verified.*

- [x] **FACADE-04**: The Milestone 9 candidate lists the reconciliation superseded are triaged.
      **Run-5 outcome, recorded so the urgency is not overstated:** run 5 read the Milestone 9
      documents directly rather than through this record and did **not** re-plan any relocation —
      `code-verification.md` verified the whole Milestone 9 orchestrator subsystem shipped, and
      Milestone 9's 0-open checkbox count is corroborated. **The trap did not spring, but the list
      is still uncorrected at source**, so anyone reading it in future gets the same wrong answer.
      `Epic_3/infrastructure-adapter-disposition.md` is the artifact
      the Epic 3 PRD §6 designates "the authoritative cross-reference for the §4.3 M9 flags" — the
      document Milestone 9 was meant to read. It is unsafe as-is on three counts: all 20 of its rows
      record "Stays / No change" while the reconciliation executed most of its List B; its M9
      targets name **two crates that do not exist** (`paladin-arsenal`, `paladin-sanctum`); and it
      disagrees with its own governing PRD on two rows (`arsenal/` is an M9 candidate here but "No"
      in the PRD table on the reasoning that "MCP wiring is facade composition-root
      responsibility"; `sanctum/` targets `paladin-sanctum` here but `paladin-memory` in the PRD).
      It is also dated `2025-01`, inconsistent with every other Milestone 8 document. Note that the
      `arsenal/` rows were **never acted on** either way, so the list is not wholly obsolete.
      **Done when** each surviving row is marked *done* / *not a candidate* / *still open*, and the
      `paladin-arsenal` / `paladin-sanctum` names are confirmed real or recorded as artefacts of a
      mis-written table. `task-completion-state.md` records Milestone 9 at **100% complete**, so run
      5 will be attaching requirements to work that has largely shipped — reading this list
      uncorrected would plan relocations that already happened.
      *Derives: REQ-adapter-disposition-record, REQ-m8-epic3-no-extractions,
      REQ-garrison-sanctum-bridges-kept; INGEST-CONFLICTS run-4 warnings on the disposition record
      and the arsenal/sanctum M9 targets. **Explicitly scoped to unblock ingest run 5.***

---

## v1 Requirements — Milestone 9-12 + Deferred-QA close-out (Phases 12-16)

Added by ingest run 5 (46 docs, 120 requirements) — **the final run.** Milestone 9 finished the
half of the platform Milestones 4-8 had left alone (`execute_workflow()`, the workflow repository
with crash recovery, scheduler/queue/event validation, the bidirectional content-agent bridge,
user/admin RBAC). Milestone 10 made the project releasable (pre-commit, cargo-audit + cargo-deny +
OSV-Scanner, CycloneDX SBOM, cargo-release with dependency-ordered publishing, and — after an
incident — main-only tag enforcement). Milestone 11 documented it (mdbook, 227 broken links
repaired, linkcheck as an error, six deployment-topology pages, GitHub Pages). Milestone 12 exposed
it over HTTP (agent registry and execution API, config-driven `paladin-server`, SSE streaming and
in-process jobs, unified error envelope, API-key and bearer auth, OpenAPI with a committed drift
guard, Docker and Kubernetes artefacts).

**All four milestones landed.** M9 100%, M10 100%, M11 92.0%, M12 99.0% — and of those open counts
only Milestone 11's 26 survive verification as genuine work. The entire Milestone 9 orchestrator
subsystem, the entire Milestone 10 tooling set, the mdbook and the entire Milestone 12 web API
demonstrably ship. **No forward requirement below re-plans any of it.**

**Run 5 is nonetheless the run that found the most genuinely unbuilt scope**, because it is the
first to ingest a register whose work was never started. `Deferred-QA-CICD-Completion` Epics 25-27
were verified open **item by item** against the tree, not inferred: no `cli-tests` job, no
`bench-check` job, no `coverage` job, no `.codecov.yml`, no Makefile coverage targets, eight
deprecated GitHub Actions, an architecture document frozen at exactly 311 lines with zero of seven
newer subsystems and zero Mermaid diagrams, an empty `docs/assets/`, no `docs/DEMOS.md`, and no
`tools` field, `ToolDefinition` or `ToolCall` anywhere in the workspace.

The forward work from run 5 is therefore of four kinds: the **supply-chain gate** that a completed
milestone left falsified (SUPPLY); the **record** that makes four milestones legible and stops
their superseded positions misrouting future work (ORCH); the **contracts that describe something
the code does not do** (WEB); and the **two registers of work that was deliberately deferred and
never revisited** — Epic 25's quality gates plus the Epic 28/29 coverage arithmetic (PIPE, DEFER),
and the documentation currency Milestone 11 left open plus the architecture gap two milestones hid
(DOCS).

#### Hand-off to Phase 12 / SUPPLY-02 and SUPPLY-03 — dated 2026-08-08 (plan 10-11)

**SUPPLY-02 and SUPPLY-03 inherit D-19's answer to the `pdf-extract` reachability question,
delivered rather than deferred.**

1. **The corrected `.cargo/audit.toml` reasoning, and what it now says:** `RUSTSEC-2026-0187`'s
   comment block (`.cargo/audit.toml:26-29`) no longer attributes `pdf-extract`'s reachability to
   the facade's `content-processing` feature gating it — that framing was wrong. It now states the
   true path positively: `pdf-extract` is an unconditional dependency of `paladin-content`
   (`crates/paladin-content/Cargo.toml:40`, no `optional = true`), and reachability from the
   workspace root is gated one level up, by whether the facade's own optional `paladin-content`
   dependency is enabled (`Cargo.toml:59`).

2. **The suppression set is unchanged.** This is a comment correction only — the `ignore = [...]`
   array in `.cargo/audit.toml` was not touched, and `RUSTSEC-2026-0187` remains suppressed on the
   same grounds (the advisory is warranted regardless of the deleted `pdf` feature's disposition).

3. **Phase 12 inherits this as an answer, not a question** — do not re-derive whether
   `pdf-extract` is reachable; ADR-0032 and this session's source-level re-verification already
   settled it. The `cargo audit` / `cargo deny check` confirmation that the reconciled suppression
   set actually passes remains **CI-only** — `crates.io` returns HTTP 403 in this environment,
   unchanged since Phase 9, and no local pass is claimed for either tool.

4. **A dead-dependency finding, named with an owner rather than left only in an ADR body:**
   `scraper`, `rss` and `tiktoken-rs` are declared optional in `crates/paladin-content/Cargo.toml`
   and consumed by no code in the crate — confirmed independently by both plan 10-05
   (`grep -rn "scraper::\|tiktoken_rs\|::rss::" crates/paladin-content/src/` → zero matches) and
   plan 10-10. ADR-0032 named this out of scope for Phase 10 and left the owner as "Phase 11 or
   Phase 15." **This hand-off assigns it to Phase 15** (the coverage-and-CI quality gates phase,
   which already inherits the seven-crate doctest posture from ADR-0033 as a related
   dependency-and-build-surface hygiene item) — Phase 11's facade-residue work may still pick it up
   incidentally if it touches the same manifest, but Phase 15 is the named owner of record.

**Evidence:** `.planning/decisions/0032-pdf-extraction-capability.md` `## Decision`,
`## Downstream Consumers`; `.cargo/audit.toml:26-29`; `.planning/ledgers/milestone-07-08.md`'s
`REQ-content-processing-build-gate` row.

### Supply-chain gate integrity (SUPPLY)

Confirmed by direct file reads on 2026-07-30. **This section corrects a run-4 finding** — see the
callout above SEC-01: `deny.toml` **is** in sync with `.cargo/audit.toml`. The gap is owner and
expiry coverage, and one unauthorised expansion.

- [x] **SUPPLY-01**: `ci.yml` runs exactly one `cargo audit`, with no inline advisory-ignore flags.
      **Two jobs currently carry the identical display name `Security Audit`.** Job id
      `security-audit` at `ci.yml:60-77` installs `cargo-audit --locked` and runs a bare
      `cargo audit` under the comment "Exceptions are the single source of truth in
      `.cargo/audit.toml` … so no inline `--ignore` flags are used here" — **compliant with M10
      Epic 2 FR-1.** Job id `security` at `ci.yml:389-406` installs `cargo-audit` **unpinned** and
      runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` — **non-compliant**,
      and its inline list covers only **2 of the 5** advisories in `.cargo/audit.toml`.
      **This is not tidiness.** `cargo audit` scans `Cargo.lock` irrespective of feature selection,
      so the three 2026 advisories are in scope for the second job too. The two jobs are configured
      to reach **different verdicts on the same tree**, and Milestone 10 Epic 2's own §8 success
      metric — "`audit.toml` and `deny.toml` are the only places policy/exceptions are defined; **no
      inline advisory-ignore flags remain in CI**" — is **false on a milestone recorded 100%
      complete with 0 open checkboxes**.
      **Mechanism, recorded so it is not repeated:** the Epic 25 PRD's Appendix B ("Current
      `ci.yml` Job Listing (Pre-Change Reference)") tabulates the pre-Milestone-10 pipeline as 7
      jobs, of which **#4 is `security`**. Milestone 10 Epic 2 **added** the compliant job without
      removing its predecessor, and Epic 4's non-goals then froze the area ("No changes to
      `deny.toml` or `.cargo/audit.toml`", "No new CI jobs — the Epic 3 pipeline is complete"), so
      nothing in the milestone was positioned to catch it. `ci.yml` now has 14 jobs.
      **Done when** `ci.yml:389-406` is deleted, no job in any workflow passes `--ignore` to
      `cargo audit`, no two jobs share a display name, and M10 Epic 2 §8 is true. **One deletion of
      18 lines satisfies the origin policy** — the single cheapest high-value fix surfaced in the
      entire ingest.
      *Derives: REQ-audit-toml-single-source (M10 Epic 2 FR-1, §8), REQ-advisory-exception-process;
      `intel/code-verification.md` run-5 verified-open finding 1; INGEST-CONFLICTS run-5 warning on
      the duplicate `cargo audit` job.*

      **Closed by Phase 9, dated 2026-08-08 — see D-07 (`09-CONTEXT.md`).** REQUIREMENTS.md's own
      SEC-01 block warned "do not plan the same fix twice"; Phase 9 ran first and made the deletion
      itself rather than leaving it for this requirement. Plan 09-06's Task 3 deleted the duplicate
      `security:` job — re-derived at `ci.yml:465-482` (the `:389-406` citation above was already
      stale before Phase 9 touched anything), commit `cb75b2b` (`.github/workflows/ci.yml`, 19 lines
      removed per `git diff --numstat`). `grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1`;
      the surviving `security-audit:` job (`:61-78`) posts the identical `"Security Audit"` context
      string `.github/rulesets/protect-main-branch.json:39` requires, confirmed before deletion, so
      no required-status-check coverage was lost. **This requirement's own checkbox is left for
      Phase 12 to check** — this note records the substance as done and cites the evidence; Phase 12
      inherits a closed item to verify, not work to re-plan. **Remaining for Phase 12:**
      confirming the required status check still resolves on the first real CI run after this
      deletion (CI-only, not verifiable in this sandboxed environment).

      **Verified by Phase 12, dated 2026-08-09 (plan 12-01):** the two structural measurements Phase
      9's note claims were re-run fresh this session, over every workflow file, not only `ci.yml`, so
      a future workflow cannot hide a second invocation:

      - `grep -rhc 'run: cargo audit' .github/workflows/*.yml | awk '{s+=$1} END{print s}'` → `1`
      - `grep -rhc 'name: Security Audit' .github/workflows/*.yml | awk '{s+=$1} END{print s}'` → `1`

      `cargo audit` was re-run in this execution: exit `0`.
      ```
      $ cargo audit
          Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
            Loaded 1190 security advisories (from /usr/local/cargo/advisory-db)
          Updating crates.io index
          Scanning Cargo.lock for vulnerabilities (677 crate dependencies)
      [8 unmaintained/unsound/yanked warnings for dotenv, fxhash, number_prefix, paste,
       rustls-pemfile, event-listener, scc, spin — all covered by SECURITY-EXCEPTIONS.md]
      warning: 8 allowed warnings found
      ```
      This run's own numbers (`1190` advisories, `677` crate dependencies, `8` allowed warnings)
      match yesterday's research figures coincidentally — they were not copied from
      `12-CONTEXT.md`/`12-RESEARCH.md`; the transcript above was produced by re-running the command
      in this execution.

      **Pending — trigger: the next push to `release/v0.7.0`.** The CI-run-observation clause above
      has not failed; it has never had the opportunity to fire. `gh run list --workflow=ci.yml
      --limit 5` confirms the most recent run against `release/v0.7.0` is still `30861568499`,
      dated `2026-08-03T23:14:24Z` — five days before Phase 9's 2026-08-08 deletion (commit
      `cb75b2b`), so no CI run has executed against the reconciled `ci.yml` yet. `gh run view
      30861568499 --json jobs -q '.jobs[] | "\(.name): \(.conclusion)"'` shows the only failing job
      in that boundary run was **`API Surface Tracking`** — DEBT-01's, Phase 8's, unrelated to
      supply-chain — while every `Security Audit` job entry (two, at that run, since the duplicate
      job had not yet been deleted) reported `success`. This clause is not closed here and must not
      be inferred or simulated closed; it closes only on a `gh run` citation newer than `30861568499`
      (D-07).

      **GitHub-rulesets finding.** `.github/rulesets/` is version-controlled
      (`protect-main-branch.json`, `protect-release-tags.json`) but is not applied to the live
      repository: `gh api repos/:owner/:repo/rulesets` returned `[]`, and `gh api
      repos/:owner/:repo/branches/main/protection` returned `404 Branch not protected`. SUPPLY-01's
      "required status check" clause therefore currently has no live enforcement point on `main` —
      the committed ruleset JSON is correct and ready, but nothing evaluates it. **Owner: the
      milestone close-out.** This phase applied nothing and enables nothing; both `gh api` calls
      above are the only ones made, and both are reads.

- [x] **SUPPLY-02**: Every advisory suppression carries an owner and a review date, and the
      vulnerability baseline matches what a document authorises. Three separable facts, all read
      from the tree:

      1. **`.cargo/audit.toml` and `deny.toml` agree on all five vulnerability advisories** —
         `RUSTSEC-2023-0071`, `RUSTSEC-2025-0111`, `RUSTSEC-2026-0187`, `RUSTSEC-2026-0194`,
         `RUSTSEC-2026-0195`. `deny.toml`'s ten extra entries are *unmaintained / maintenance-mode*
         notices, a different advisory class, filed under a header that says so, and **explicitly
         authorised** by M10 Epic 4 FR-1 step 5. **There is no synchronisation defect.**

      2. **13 of the 15 `deny.toml` suppressions have documented reasoning but no named owner and
         no expiry date.** Only `RUSTSEC-2023-0071` and `RUSTSEC-2025-0111` carry a formal risk
         acceptance — owner **Platform Security (Milestone 7)**, review/expiry target
         **2026-09-30** — and that record lives in `rustsec-remediation-plan.md`, not in either
         config file. **M10 Epic 2 FR-3's four-field schema does not require an owner or an
         expiry**, so the configs are compliant with their governing policy and the policy is the
         gap.

      3. **The three 2026 vulnerability ignores are authorised by no ingested document.** M10 Epic
         2 FR-3 and §5 name exactly two preserved advisories. The three additions are DoS-class,
         transitive through *optional* features (`pdf-extract` under `content-processing`;
         `rust-s3`/`aws-creds` under `s3`), and carry dated four-field reasoning — including the
         note that the directly-fixable `RUSTSEC-2026-0185` (quinn-proto) and `RUSTSEC-2026-0190`
         (anyhow) were **upgraded rather than ignored**, which is evidence of a working process, not
         a lax one. Ratification is defensible; it is simply unrecorded.
      **Done when** (a) each of the three 2026 vulnerability ignores is ratified by a recorded
      decision or removed; (b) M10 Epic 2 FR-3's schema is extended to require **owner** and
      **expiry**, and all thirteen entries are backfilled; and (c) the 2026-09-30 acceptance has a
      disposition — renewed with a new date, closed, or replaced. **Sequenced with HARD-06**, which
      decides whether `pdf-extract` is reachable at all and therefore whether `RUSTSEC-2026-0187`
      needs suppressing. **Narrows and corrects SEC-01**; do not plan both against the same fix.
      *Derives: REQ-advisory-exception-process (M10 Epic 2 FR-3, §5), REQ-audit-toml-single-source,
      REQ-deny-license-allowlist, REQ-rustsec-risk-acceptance (run 4);
      `intel/code-verification.md` run-5 CORRECTION section and verified-open finding 2;
      INGEST-CONFLICTS run-5 warning on the exception set exceeding its authorising baseline.*

      **Closed by Phase 9, dated 2026-08-08 — see D-07 (`09-CONTEXT.md`).** All three of this
      requirement's clauses are executed by this phase's plans, not left for Phase 12 to re-plan:
      (a) the three 2026 vulnerability ignores are **ratified**, not removed, by ADR-0024 decision 3,
      each with a concrete compensating control naming the actual reachable input path
      (`SECURITY-EXCEPTIONS.md`); (b) M10 Epic 2 FR-3's schema is extended with `owner` and
      `review_date` by ADR-0024 decision 2, and all ten surviving suppressions are backfilled in
      `SECURITY-EXCEPTIONS.md` (owner `DF3NDR`, `review_date` `2026-12-31` on every row) — note the
      corrected baseline is **ten** entries, not the thirteen this clause names, since four of the
      original fifteen/fourteen were already dead (deleted, not backfilled — see the SEC-01
      correction above); (c) the 2026-09-30 acceptance is renewed to per-advisory 2026-12-31 dates,
      recorded in ADR-0024. Evidence: plan 09-02 (commits `a587e5a`, `7ee741c`) authored the register
      and ADR; plan 09-06 (commits `6513cb7`, `9cef391`, `cb75b2b`) reconciled both TOML files and
      landed `scripts/check-advisory-register.sh`, demonstrated failing nine distinct ways in
      `09-06-SUMMARY.md`. **This requirement's own checkbox is left for Phase 12 to check** — this
      note records the substance as done; Phase 12 inherits a closed item to verify. **Remaining for
      Phase 12:** `cargo audit`/`cargo deny check` actually passing against the reconciled
      configuration is CI-only, not run in this environment (crates.io returns HTTP 403).
      (**Corrected by Phase 12 (plan 12-01), dated 2026-08-09, citing this plan's own re-run:**
      the HTTP-403 blocker has lifted — `cargo-audit` and `cargo-deny` are both on `PATH` in this environment as
      of 2026-08-09. Phases 9 and 10 were both correct at the time they wrote this caveat; it is not
      permanent. Both tools, plus the register script, were re-run and exited `0` on 2026-08-09 by
      plan 12-01 — see the "Verified by Phase 12" transcripts immediately below rather than a
      restatement here.)

      **Verified by Phase 12, dated 2026-08-09 (plan 12-01):** `cargo deny check` and
      `./scripts/check-advisory-register.sh` were both re-run in this execution, on `PATH`, with no
      installability blocker.

      `cargo deny check` — exit `0`, tail line:
      ```
      advisories ok, bans ok, licenses ok, sources ok
      ```

      `./scripts/check-advisory-register.sh` — run twice in succession to demonstrate the
      idempotence property the script's own header asserts at `:44-45`, not merely quote it. Both
      runs exited `0` and produced byte-identical output:
      ```
      $ ./scripts/check-advisory-register.sh
      🔍 Checking the advisory exception register against deny.toml, .cargo/audit.toml and Cargo.lock ...
      ✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.

      $ ./scripts/check-advisory-register.sh
      🔍 Checking the advisory exception register against deny.toml, .cargo/audit.toml and Cargo.lock ...
      ✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.
      ```
      `diff` of the two captured transcripts is empty; both exit codes are `0`.

      This run demonstrates four register properties, each citing the script line that asserts it:

      - **Set-based, case-sensitive comparison** (`check-advisory-register.sh:11-15`) — class
        partitions are compared as sets of raw strings; an identifier differing only in letter case
        fails, and reordering rows or TOML entries does not change the verdict, because the
        comparison is over Python `set` equality, not list order or string casefolding.

      - **The one-sided-empty case is a distinct named failure** (`:31-33`) — zero register rows
        against a non-empty `deny.toml`/`.cargo/audit.toml` ignore array prints `ONE_SIDED_EMPTY` and
        fails; zero rows with two empty arrays is the only legitimate pass in that branch. This run's
        live state (10 rows, 10 `deny.toml` entries, 5 `.cargo/audit.toml` entries) does not hit this
        branch, but the branch is inspected in source at the cited lines and confirmed present.

      - **Read-only, no temporary file** (`:43`) — the script's own header states it, and `git status
        --porcelain` immediately after both runs is unchanged, consistent with no write occurring.

      - **Identical output and exit code across two consecutive runs** (`:44-45`) — demonstrated
        above by the two captured transcripts, not merely asserted.

      Live counts: **10 register rows** against **10 `deny.toml`** and **5 `.cargo/audit.toml`**
      ignore entries — matching the corrected baseline this closure note already records above (ten
      surviving suppressions, not the original thirteen/fifteen).

- [x] **SUPPLY-03**: The two supply-chain ADR candidates are promoted or declined, deliberately.
      `Milestone_7/Epic_4/rustsec-remediation-plan.md` carries the corpus's **only expiry date**;
      `Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8 carries the
      **single-source invariant the tree currently violates**. They are the same subject from two
      milestones, and both sit at DOC/PRD precedence where any future document can override them.
      Promoting them together via `--manifest` would turn the run-5 supply-chain finding from an
      observation into a **gate**. **Done when** a recorded decision exists either way — and if
      declined, the reason is recorded so the next reader does not re-open it.
      **This requirement does not act.** Promotion requires re-tagging the source documents and
      re-running ingest, which is a user-owned step outside any planning artefact; entering a lock
      here would fabricate authority the corpus does not contain.
      *Derives: `intel/decisions.md` run-5 ADR-candidate list (11 candidates, 0 locked);
      `intel/SYNTHESIS.md` run-5 Decisions section.*

      (**Corrected by Phase 12 (plan 12-03), dated 2026-08-09, citing `.planning/decisions/PROMOTION.md`
      §Part A and `.planning/decisions/0036-audit-suppression-single-source-topology.md` (ADR-0036):**
      both stale claims above are superseded. **The does-not-act clause is superseded:** the
      `--manifest`/re-ingest path this clause describes no longer exists and is not needed —
      `PROMOTION.md` §Part A states plainly "Promotion is now an ordinary write to a directory plus a
      table row," because ADRs live in `.planning/decisions/` as their own document class,
      independent of the ingest manifest, and top the precedence order. Four prior promotions already
      prove the practice: candidate 1 closed by ADR-0016, candidate 2 by ADR-0021, candidate 3 by
      ADR-0024, candidate 5 by ADR-0025. Under the project's own precedence order, shipped practice
      plus a higher-precedence ADR-class document outrank PRD/DOC-tier requirement text (D-01, D-00b).
      ADR-0036 is this requirement's own act. **The candidate-count clause is corrected:** "the two
      supply-chain ADR candidates" is now one. Candidate 3
      (`Milestone_7/Epic_4/rustsec-remediation-plan.md`, the corpus's only dated item, review target
      2026-09-30) was discharged 2026-08-08 by ADR-0024, which renewed it to per-advisory
      `2026-12-31` review dates and reassigned the owner; `PROMOTION.md:185-189` records the closure.
      Candidate 7 (`Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8) is
      the surviving subject, and it is what ADR-0036 promotes. No second ADR is written on ADR-0024's
      subject — exactly one live ADR answers each question. **Stated plainly, because it is a fact
      about the corpus and not only about SUPPLY-03:** three documents say this requirement cannot
      act; a fourth says it can, and four ADRs prove it. The corpus's rule that "nothing is locked"
      was itself superseded when Phase 1 built `.planning/decisions/`, and the requirement text never
      caught up. This requirement's checkbox and traceability row are left `Pending` for plan 12-04.)

#### Hand-off to Phase 13 / ORCH-01 — dated 2026-08-09 (plan 12-04)

**ORCH-01 inherits all three SUPPLY requirements closed with dated `file:line`-cited verdicts,
delivered rather than deferred, plus one observation that is honestly pending and one finding that
is honestly owner-only.**

1. **The three SUPPLY verdicts and where their evidence lives.** SUPPLY-01 and SUPPLY-02 were
   closed by Phase 9 (dated 2026-08-08) and verified in-repo by plan 12-01 (dated 2026-08-09) with
   three re-run gate transcripts (`cargo audit`, `cargo deny check`,
   `./scripts/check-advisory-register.sh`, all exit `0`); SUPPLY-03 was closed by ADR-0036. The
   full transcripts live in this document's own SUPPLY-01 (`REQUIREMENTS.md:1855-1940`) and
   SUPPLY-02 (`:1941-2046`) blocks; SUPPLY-03's block is `:2047-2082`. ADR-0036 itself is
   `.planning/decisions/0036-audit-suppression-single-source-topology.md`, and its enforcement
   mechanism is `scripts/check-workflow-suppressions.sh`, wired into `Makefile:171-176` and
   `.github/workflows/ci.yml:103-104`. This item points at that evidence rather than repeating it.

2. **ORCH-01's named verdict class, both halves, verbatim.** Milestone 10 is recorded 100%
   complete, ships every artefact it promised, **and failed one of its own acceptance criteria** —
   and, as of 2026-08-08, **no longer does**. The criterion is M10 Epic 2 §8's "`audit.toml` and
   `deny.toml` are the only places policy/exceptions are defined; no inline advisory-ignore flags
   remain in CI." Phase 9 made it true (plan 09-06, commit `cb75b2b`, deleting the duplicate
   `security:` job at pre-deletion `ci.yml:465-482`); Phase 12 promoted it to ADR-0036 and put a
   regression guard behind it so it stays true. **ORCH-01 must carry both halves** — the corpus's
   only verdict of this class, and Phase 12 is the phase that gets to date the second half.

3. **What Phase 12 deliberately did not build, and why.** `.planning/ledgers/milestone-09-12.md`
   does not exist, and Phase 12 did not create it, not even as a stub — `ls
   .planning/ledgers/` returns exactly the four pre-existing files (`milestone-01.md`,
   `milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`). Its 120-requirement-ID scope
   is ORCH-01's own stated deliverable (`ROADMAP.md` §Phase 13 criterion 1); a stub here would
   either be re-planned by Phase 13 or would silently constrain its shape (D-09). Phase 12 wrote its
   evidence where every prior phase wrote pre-ledger evidence: the requirement rows themselves, plus
   this block.

4. **One observation genuinely pending, with its trigger.** SUPPLY-01's clause "confirming the
   required status check still resolves on the first real CI run after the deletion"
   (`REQUIREMENTS.md:1893-1895`) has not failed — it has never had the opportunity to fire. The
   most recent run against `release/v0.7.0` is still `30861568499`, dated `2026-08-03T23:14:24Z`,
   five days before Phase 9's 2026-08-08 deletion — no CI run has executed against the reconciled
   `ci.yml` yet. **Trigger: the next push to `release/v0.7.0`.** A verifier marking this clause done
   without a `gh run` citation newer than `30861568499` is reporting a false positive (D-07). In
   that last pre-deletion run, the only failing job was `API Surface Tracking` (DEBT-01's, Phase
   8's, unrelated to supply-chain), while every `Security Audit` job entry reported `success`.

5. **The unapplied GitHub rulesets, a finding with an owner.** `.github/rulesets/` is
   version-controlled (`protect-main-branch.json`, `protect-release-tags.json`) but the rulesets
   are not applied and `main` is not protected: `gh api repos/:owner/:repo/rulesets` returned `[]`,
   and `gh api repos/:owner/:repo/branches/main/protection` returned `404 Branch not protected`
   (both transcribed in SUPPLY-01's own block, `REQUIREMENTS.md:1932-1939`). SUPPLY-01's "required
   status check" clause therefore currently has no live enforcement point. **Owner: the milestone
   close-out** — only the repository owner can apply a ruleset or protect a branch. Phase 12 applied
   nothing; both `gh api` calls above are reads.

6. **The provenance of the re-scope, carried forward rather than laundered.** Phase 12 treated
   SUPPLY-01 and SUPPLY-02 as verification rather than work because Phase 9's D-07 re-scoped this
   phase — a decision made under `--auto`, flagged "⚠ HUMAN REVIEW — this changes another phase's
   scope" in `09-CONTEXT.md`, and never itself ratified by a human. It is durable because it is
   recorded at source in `ROADMAP.md`'s dated Phase 12 closure note (`ROADMAP.md:768-778`) and in
   this document's own SEC-01/SUPPLY traceability coupling row (`REQUIREMENTS.md:4236`), not
   because a human approved it. The three re-run gate transcripts plan 12-01 produced are strong
   independent evidence regardless. ORCH-01 should record the closure **and** its provenance rather
   than a bare `Complete`. This phase carries its own two unratified `--auto` decisions alongside
   Phase 9's: **D-01** (SUPPLY-03 acts and writes an ADR, overriding its own "does not act" clause)
   and **D-08** (the regression guard adds a CI check no requirement explicitly asked for) — both
   flagged `⚠ HUMAN REVIEW` in `12-CONTEXT.md` and resolved only when a human selected `option-a` at
   plan 12-01's blocking checkpoint, dated 2026-08-09 (`12-01-SUMMARY.md` §Checkpoint Status).

7. **A measurement that contradicts two of this phase's own upstream artefacts.** The stale
   `ci.yml:389-406` citation appears far more widely than either `12-CONTEXT.md` ("three documents")
   or `12-RESEARCH.md` ("one") recorded: plan 12-01 measured **87 hits across 25 files** in
   `.planning/` and `.project/` this session (`grep -rn '389-406' .planning/ .project/`). Of those,
   **8 sites across four canonical governance documents** (`REQUIREMENTS.md`, `PROJECT.md`,
   `ROADMAP.md`, `STATE.md`) were in scope and each received one dated correction naming the true
   location `ci.yml:465-482` and commit `cb75b2b`, with every original citation retained. The
   remaining hits were excluded by a stated scoping rule, class by class: frozen archived-milestone
   snapshots (`.planning/milestones/v0.7.1-*`); prior-phase context/log/summary files left as
   historical record (`.planning/phases/09-*`); closed ingest outputs with no run 6
   (`.planning/intel/*`, `.planning/INGEST-CONFLICTS.md`); a closed prior-milestone ledger row
   whose deferral text is accurate as written (`.planning/ledgers/milestone-01.md:144`); ADR-0024's
   own already-self-annotated stale citation (`0024-rustsec-exception-governance.md:223`, not
   edited, D-00i); Phase 9's own pre-existing correction banners that already correct the citation
   within their own regions; and this phase's own context/research/plan files, which describe the
   defect rather than citing it live. ORCH-01 inherits this measured inventory (`12-01-SUMMARY.md`
   §Grep Inventory) rather than re-deriving it, and knows which sites are deliberately left as
   historical record.

**Evidence:** `.planning/decisions/0036-audit-suppression-single-source-topology.md`;
`scripts/check-workflow-suppressions.sh`; `Makefile:171-176`; `.github/workflows/ci.yml:103-104`;
`cargo audit`, `cargo deny check`, `./scripts/check-advisory-register.sh`; `gh run
list --workflow=ci.yml --limit 5`, `gh run view 30861568499 --json jobs`, `gh api
repos/:owner/:repo/rulesets`, `gh api repos/:owner/:repo/branches/main/protection`;
`REQUIREMENTS.md:1855-1940` (SUPPLY-01), `:1941-2046` (SUPPLY-02), `:2047-2082` (SUPPLY-03).

#### Hand-off to Phase 13 / ORCH-05 — dated 2026-08-08 (plan 10-11)

**ORCH-05 inherits ADR-0029's `## Trajectory` table to append to, and the note that REL-01 is
already converged so ORCH-05 applies rather than re-decides.**

1. **The table to append to, not replace:** `.planning/decisions/0029-version-trajectory-history.md`'s
   `## Trajectory` table records `v0.1.0-rc.1` (commit `a9530fc`, 2026-05-28, all ten crates
   published at `0.1.0`, GO sign-off) through the current `0.7.0`/`v0.7.0`/`v0.7.1` state, with a
   labelled placeholder row naming Phase 13 / ORCH-05 as owner of `v0.3.0` through `v0.6.0`.
   ORCH-05 appends rows for `v0.3.0`, `v0.4.0`, `v0.5.0` and `v0.6.0` in ascending order, without
   re-sorting or re-keying the existing rows.

2. **REL-01 is already converged and must not be re-opened.** `REQUIREMENTS.md:360` — `REL-01` is
   `[x]` — and its traceability row reads `Phase 4 | Complete`, converged on `0.7.0` via ADR-0008.
   HARD-03's "Feeds REL-01" clause has already fired; it is backwards-looking confirmation of closed
   history, not a hand-off to open work. ORCH-05 applies the already-converged result rather than
   re-deciding it, per the ROADMAP's own coupling note.

3. **Three ADRs, one unbroken line:** HARD-03 (ADR-0029) covers rc.1 → v0.2.0; ORCH-05 extends the
   same ADR's table through v0.6.0; REL-01 (ADR-0008, Phase 4, done) covers the landing at v0.7.0.
   Writing a second, competing version ADR for ORCH-05 is prohibited — whichever of HARD-03/ORCH-05
   runs second applies to the one artefact rather than authoring a rival.

**Evidence:** `.planning/decisions/0029-version-trajectory-history.md` `## Trajectory` table and
`## Downstream Consumers` (names Phase 13 / ORCH-05 explicitly); `REQUIREMENTS.md:360` (REL-01
checkbox) and its traceability row (`grep -n '^| REL-01' REQUIREMENTS.md`).

### Milestone 9-12 ground truth & recorded account (ORCH)

- [x] **ORCH-01**: A developer can look up any of the **120** Milestone 9-12 + Deferred-QA
      requirement IDs and see a `file:line`-cited verdict — shipped, relocated, superseded by
      outcome, verified open, or genuinely outstanding — instead of a PRD path that may predate the
      workspace decomposition, the Milestone 6 relocations or the Milestone 11 mdbook move.
      Sixteen entries already carry `settled-by` pointers into `intel/code-verification.md` run 5;
      those are **facts about the tree**, not decisions, and the remaining 104 need the same
      treatment. **Done when** the *Milestone 9-12 as-shipped ledger* below is upgraded from
      component-level file evidence to per-criterion verdicts, and when the ledger states plainly
      that the whole M9 orchestrator subsystem, the whole M10 tooling set, the mdbook and the whole
      M12 web API ship — so nothing in them is re-planned.
      **Corrected (dated 2026-08-10, D-04, plan 13-10):** ~~"Sixteen entries already carry
      `settled-by` pointers into `intel/code-verification.md` run 5 … and the remaining 104 need the
      same treatment"~~ counts two different populations. The sixteen are **variant-register**
      entries, not ledger rows —
      `intel/SYNTHESIS.md:335` defines the `settled-by` mechanism as applying "where the shipped
      tree settles a *variant*", and `:546` counts "sixteen entries carry `- settled-by:` lines"
      under the variant register, above, not under this ledger. Measured this session:
      `grep -c "settled-by" .planning/REQUIREMENTS.md` → **10**, none inside the ledger region
      (`sed -n '3607,3931p' .planning/REQUIREMENTS.md | grep -c "settled-by"` → **0**). **No ledger
      row carries a `settled-by` pointer, and all 120 rows need a verdict** — not 104. The measured
      split, re-run this session against `REQUIREMENTS.md:3607-3931` before plan 13-10 replaced that
      range with a pointer: `grep -c '^| REQ-'` → **120**; bare `Verify` rows → **35**; bare
      `Shipped` rows (`Shipped — ` plus the `Shipped → ` arrow form) → **51 + 2 = 53**; the
      remainder, already carrying a richer verdict → **32** (120 − 35 − 53). This is the same class
      of error as Phase 10's D-05 — an arithmetic claim inside the very requirement that exists to
      retire it — and a planner budgeting 104 rows against this ledger would have been wrong in both
      directions at once: too low by the 16 already-settled variant entries this ledger never held,
      and too high by counting them against a 120-row surface where 0 carry the pointer. See
      `.planning/ledgers/milestone-09-12.md`'s own "Corrected arithmetic" paragraph, which reconciles
      the identical 35/53/32 figures against the same commands.
      **One verdict class is new and must survive into the ledger:** Milestone 10 is recorded 100%
      complete, ships every file, job, target and ruleset it promised, **and fails one of its own
      acceptance criteria** (SUPPLY-01). A count that is simultaneously accurate about deliverables
      and wrong about acceptance is a failure mode this corpus had not previously produced.
      **Closed (dated 2026-08-10, plan 13-13):** the ledger exists at
      `.planning/ledgers/milestone-09-12.md` with all **120** requirement IDs carrying a
      `file:line`-cited verdict (`grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` → `120`),
      and this document's own `## Milestone 9-12 as-shipped ledger` section
      (`REQUIREMENTS.md:3664-3681`) is reduced to a pointer exactly as the done-when criterion
      requires. The whole M9 orchestrator subsystem, the whole M10 tooling set, the mdbook and the
      whole M12 web API are all stated shipped in the ledger's own per-milestone corroboration
      paragraph, so none of them is re-planned. The ledger's own `## Phase 13 close-out amendments
      (2026-08-10)` section (this same plan) re-confirms the whole-file integrity bar — 120 rows, 120
      distinct IDs, zero blank verdict cells, all cited `ADR-NNNN` numbers resolving — one final time
      before this checkbox closes.
      *Derives: all 120 run-5 `REQ-*` IDs; `intel/code-verification.md` run-5 (37 verified-shipped
      rows, 8 verified-open findings, 1 correction).*

- [x] **ORCH-02**: Each run-5 open-checkbox block has a written verdict, and the corpus-level
      pattern is stated once. Five blocks, five different answers, none of which is a task list:
      **M9's 0 open** — corroborated; every Epic 1-5 deliverable is present.
      **M10's 0 open** — corroborated in artefacts, contradicted in one acceptance criterion
      (SUPPLY-01).
      **M11's 26 open** — the **only** genuinely open count in run 5, and the only one in all 542
      that survives verification; carried to DOCS-01, and settleable only by content.
      **M12's 3 open** — vacuous: all three are Task 0.0 scaffolding ("Create feature branch",
      "Update `main` … and create/checkout `feature/m12-epic5-api-security-authorization`",
      "Confirm a clean baseline") while the Epic 5 work itself ships as
      `crates/paladin-web/src/agent_auth.rs`.
      **project-management's 1 open** — nonexistent: the item is
      `- [ ] 1.1 Create template → - [x] 1.1 Create template (after completing)`, a formatting
      example inside a template file.
      **Done when** these five verdicts are recorded, none is converted into a task, and the
      five-run pattern (understated → accurate → overstated → contradicted → vacuous) is written
      down in exactly one place so it stops being rediscovered.
      **Closed (dated 2026-08-10, plan 13-13):** all five verdicts and the five-run pattern are
      recorded exactly once, in `.planning/ledgers/milestone-09-12.md`'s own "Per-milestone checkbox
      corroboration (D-10, ORCH-02)" paragraph — M9 0 open corroborated; M10 0 open corroborated in
      artefacts, contradicted in one acceptance criterion (the ledger's own `Shipped, one acceptance
      criterion false` highlight row); M11 26 open, the only genuinely open count across all 542
      items in the corpus, carried to DOCS-01; M12 3 open, vacuous Task 0.0 feature-branch
      scaffolding while the Epic 5 code ships; project-management 1 open, a template formatting
      example, not a real task line. None of the five is converted into a task, and the pattern
      (understated → accurate → overstated → contradicted → vacuous) is written down in exactly the
      one place the requirement's own done-when criterion names.
      *Derives: `intel/task-completion-state.md`; `intel/code-verification.md` run-5 checkbox
      analysis and *Final corpus position on open-checkbox counts*.*

- [x] **ORCH-03**: The run-5 positions the tree contradicts are corrected at source, and the
      relocations are recorded as relocations rather than gaps. Five specific items:
      (a) **The agent route surface.** Milestone 12 Epics 1, 3, 4 and 5 all write acceptance
      criteria, test assertions and examples against unprefixed paths (`/agents`,
      `/agents/{id}/execute`, `/agents/{id}/execute/stream`, `/agents/{id}/jobs`), while Epic 6 §4.3
      requires "the agent API is served under `/v1`; operational/docs endpoints remain unversioned".
      Four Epics' requirement text names paths a fifth Epic relocates. **Confirm the shipped prefix
      against `crates/paladin-web/openapi.json`** — the committed drift-guard baseline, which locks
      in whichever form actually shipped — and mark the Epic 1-5 route text as **superseded
      provenance, not a live contract**. Preserved as a run-5 unsettled position, not resolved
      here.
      (b) `REQ-listener-service-test-coverage` names `src/core/platform/manager/listener_service.rs`
      — **the file does not exist**; the code ships as `src/application/services/orchestration/
      listener.rs` (`ListenerOrchestrator`) after the Milestone 6 Epic 2 relocation.
      (c) `REQ-llm-tool-calling-port` names `src/application/ports/output/llm_port.rs` — **the
      directory does not exist**; `src/application/ports/` was fully deleted by Milestone 5 Epic 2.
      The current path is `crates/paladin-ports/src/output/llm_port.rs`.
      (d) `REQ-arch-doc-modernization` names `docs/Design/Design_and_Architecture.md` — it ships as
      `docs/src/appendix/design-and-architecture.md` after the Milestone 11 overhaul (DOCS-02).
      (e) `REQ-asciinema-demos` requires README embedding, but Milestone 11 Epic 5 rewrote the
      README into a concise landing page with **no demos section** — the clause targets a document
      that has since changed shape (DOCS-04).
      **Done when** anyone applying a run-5 requirement literally cannot write to a path that does
      not exist, and (a) has one recorded answer.
      **Closed (dated 2026-08-10, plan 13-13; checkbox flipped 2026-08-10 by plan 13-08, evidence
      recorded here):** (a) ADR-0037 (`.planning/decisions/0037-agent-route-surface-v1.md`, plan
      13-08) is the recorded answer on the `/v1` route surface, citing `crates/paladin-web/openapi.json`
      as the committed drift-guard baseline that settles which form shipped; `docs/src/deployment-topologies/sidecar.md:29`
      is corrected to the `/v1`-prefixed form in the same plan (D-12). (b)-(e) the four stale paths —
      `REQ-listener-service-test-coverage`, `REQ-llm-tool-calling-port`, `REQ-arch-doc-modernization`,
      `REQ-asciinema-demos` — are corrected at source with dated banners naming the current path,
      originals retained (plan 13-11, Source-Level ORCH-03 Relocation Annotations, in
      `.project/Deferred-QA-CICD-Completion/{prd-deferred-qa-completion.md,DEFERRED_COVERAGE.md}` and
      `.planning/intel/requirements.md`). Every item this requirement names now has a `file:line`-cited
      answer; none is left as a run-5 position the tree merely contradicts.
      *Derives: REQ-agent-execute-endpoint, REQ-api-v1-versioning, REQ-openapi-drift-guard,
      REQ-listener-service-test-coverage, REQ-llm-tool-calling-port, REQ-arch-doc-modernization,
      REQ-asciinema-demos; INGEST-CONFLICTS run-5 warning on the competing agent route surface.*

- [x] **ORCH-04**: The two seams Milestone 12 recorded as *defaults* get **decisions**.
      (a) **`AgentProvisioner` placement.** Epic 1 Open Question 2 reads "keep in `paladin-web`
      (single consumer today) or promote to `paladin-ports` now for reuse by future topologies
      (sidecar/worker)? (Default: `paladin-web`; promote only if a second consumer appears.)" §7
      adds that "either placement is clean since it references `Paladin` + `PaladinExecutorPort`,
      both already in core/ports". But `docs/src/deployment-topologies/queue-worker.md` and
      `sidecar.md` **both describe topologies that must provision agents from a spec** — so under
      the default a worker process either depends on the HTTP adapter crate to reuse the trait, or
      duplicates it. This is a load-bearing seam recorded as a default rather than a decision, and
      it is **cheap to answer now and expensive to answer after a second consumer exists**.
      (b) **Garrison and Arsenal for HTTP-served agents.** Stated once, in an Epic 2 non-goal —
      "Garrison (memory) and Arsenal (tools/MCP) wiring for agents — a later enhancement; agents are
      LLM + prompt only here" — and restated once by Epic 3. Meanwhile M11 Epic 6 FR-8 makes
      `docs/src/deployment-topologies/overview.md` "the single source of routing" between
      topologies. A reader routed to the HTTP host by that decision matrix gets a **materially less
      capable agent** than the embedded-library topology, unless the page says so. One line in a
      non-goal is not enough surface for a capability difference this large.
      **Done when** (a) has a recorded placement with its reasoning, and (b) is either planned scope
      with a target or a **permanent property of the topology** stated explicitly in the
      decision matrix.
      **Closed (dated 2026-08-10, plan 13-13):** (a) ADR-0038
      (`.planning/decisions/0038-agent-provisioner-placement.md`, plan 13-09) records
      `AgentProvisioner` staying in `crates/paladin-web` — `AgentSpec` derives `utoipa::ToSchema` and
      is an OpenAPI-annotated HTTP request DTO, not a portable core type; `paladin-ports` carries no
      `utoipa` dependency; `FacadeProvisioner` is `#[cfg(feature = "web-server")]`-gated, confirming
      it is the HTTP composition root rather than a second topology. Verdict `conforms`. (b) ADR-0039
      (`.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md`, plan 13-09) records the
      absence of Garrison and Arsenal on HTTP-served agents as a **permanent property of the shipped
      topology**, not planned scope, and the limitation is now stated in prose in
      `docs/src/deployment-topologies/http-service-host.md` and `overview.md` (plan 13-09, Task 3).
      Verdict `must change` — executed by the same plan that authored it. **Both decisions were
      ratified at plan 13-09's blocking checkpoint** (`type="checkpoint:decision" gate="blocking"`),
      resolved by a human operator via the runtime's `AskUserQuestion` interactive mechanism during
      the `/gsd-execute-phase 13` orchestrator session, dated **2026-08-10**, with auto-mode
      confirmed off for that session — the human selected the recommended option (`option-a`) on all
      three items presented (D-14, D-15, and PROMOTION.md Part B candidate 9), recorded in full,
      including the options shown and the provenance mechanism, in `13-09-SUMMARY.md` §Checkpoint
      Status (D-00i). This closing note cites that record rather than re-deriving or re-deciding it,
      per the checkpoint provenance convention Phase 12's plan 12-01 established.
      *Derives: REQ-agent-provisioner-port (Epic 1 §4.4 FR-15, OQ-2), REQ-concrete-agent-provisioner,
      REQ-registry-from-config-builder (Epic 2 §4.2 non-goal), REQ-deployment-topologies-section
      (M11 Epic 6 FR-8); INGEST-CONFLICTS run-5 warnings on the AgentProvisioner placement and on
      HTTP-served agents having no Garrison and no Arsenal.*

- [x] **ORCH-05**: The version trajectory is complete through the tree, and the milestone-numbering
      prediction is closed. Run 5 supplies the four release gates that HARD-03's history stopped
      short of: **M9 → v0.3.0**, **M10 → v0.4.0**, **M11 → v0.5.0**, **M12 → v0.6.0** — a
      lockstep-versioned chain in which each milestone's finalization Epic bumps the root crate and
      every workspace member together and cuts a tag. That chain terminates exactly where the tree
      is: root `Cargo.toml` at `version = "0.6.0"`, branch `release/v0.7.0`, latest tag `v0.5.1`.
      **Corrected (dated 2026-08-10, D-18, plan 13-10):** ~~"That chain terminates exactly where the
      tree is: root `Cargo.toml` at `version = "0.6.0"`, branch `release/v0.7.0`, latest tag
      `v0.5.1`"~~ — two of the three clauses are two releases
      stale. Re-run this session: `grep -n '^version' Cargo.toml` → `Cargo.toml:34` `version =
      "0.7.0"`; `git tag --sort=-v:refname | head -8` → `v0.7.1, v0.7.0, v0.5.1, v0.5.0, v0.4.3,
      v0.4.2, v0.4.1, v0.4.0`; `git branch --show-current` → `release/v0.7.0` — the one accurate
      clause of the three. This is the identical defect Phase 10's D-11 already corrected once, in
      HARD-03 (see that entry's own `**Corrected (dated 2026-08-08, HARD-03)**` block, which reads
      `Cargo.toml:34` `version = "0.7.0"` and `git tag --sort=-v:refname | head -3` → `v0.7.1,
      v0.7.0, v0.5.1`), regrown one requirement later. `intel/code-verification.md:469` carries the
      same stale `0.6.0` figure — correct as of the 2026-07-30 ingest and superseded since; Phase 13
      plan 13-11 corrects it there. **The historical facts below are unchanged**: the four lockstep
      gates M9 → `v0.3.0`, M10 → `v0.4.0`, M11 → `v0.5.0`, M12 → `v0.6.0` are exactly what plan 13-12
      transcribes into ADR-0029 — only this current-state clause is corrected.
      **HARD-03 records `v0.1.0-rc.1` as history; ORCH-05 completes the line from rc.1 to 0.6.0**,
      so REL-01 converges a three-way disagreement with the full trajectory in view rather than a
      fragment of it.
      Second half: the Roadmap Extension Protocol predicted a **fifth** milestone-numbering
      collision after the four found in runs 2-4 (M4-6 overviews titled "Milestone 1/2/3" by
      refactoring tier; M3 release notes assigning Epics 19-23 to four M2 features; PRDs
      cross-referencing "Milestone 1 / Epic 2" meaning M4 Epic 2; the M7 overview titling itself
      "Milestone 4"). **Run 5 found none.** **Done when** run-5 provenance keys are confirmed to
      resolve directly against directory numbering, and the prediction is recorded closed — or, if
      a fifth collision is found on closer reading, it is corrected at source with the same
      convention that closed the first four.
      **Closed (dated 2026-08-10, plan 13-13):** ADR-0029's `## Trajectory` table
      (`.planning/decisions/0029-version-trajectory-history.md`, plan 13-12) now runs unbroken from
      `v0.1.0-rc.1` through `v0.2.0`, `v0.3.0` (M9), `v0.4.0` (M10), `v0.5.0` (M11), `v0.6.0` (M12) to
      `0.7.0`/`v0.7.1` — four new rows appended in ascending order, each citing its own
      `.planning/ledgers/milestone-09-12.md` row and `REQ-lockstep-versioning` as the shared bump
      mechanism; the `v0.6.0` row records the missing `v0.6.0` tag by citing the three finalization
      commits (`90ca591`, `67b6207`, `23b187b`) rather than asserting a tag that does not exist. The
      spanning Phase-13 placeholder row is superseded in place, original text retained (D-00d). The
      milestone-numbering prediction's second half is closed: plan 13-12 ran the provenance-key
      confirmation across all 120 run-5 `REQ-*` provenance source paths (Milestone 9-12,
      Deferred-QA-CICD-Completion, project-management) and found **no fifth self-numbering
      collision** — every overview H1, Epic-level PRD H1 and `**Milestone:**` metadata line
      self-titles with its own directory's milestone number — citing **ADR-0030:79-84** as the
      existing closure rather than authoring a rival numbering ADR, exactly as this requirement's own
      done-when criterion and D-20's locked-allocation rule require.
      *Derives: REQ-m9-quality-gate-v030, REQ-m10-v040-release, REQ-lockstep-versioning,
      REQ-m11-v050-release, REQ-doc-version-sync, REQ-m12-v060-release; couples to HARD-03
      (Phase 10) and REL-01 (Phase 4).*

#### Hand-off to Phase 14 / WEB-01 … WEB-04 — dated 2026-08-10 (plan 13-13)

**Phase 13 is the last ground-truth phase in the corpus. This block, and the two below it, are the
final forward-work signal Phases 14, 15 and 16 receive (D-22).**

1. **The token mechanism has one un-settled variant, and shipped code carries both halves at once.**
   `.planning/ledgers/milestone-09-12.md`'s `REQ-opaque-bearer-token-adapter-v1` row (Milestone 9
   Epic 5) is `Contract diverges → WEB-01` — variant group 29, the only variant in five verification
   runs shipped code itself cannot settle. Its sibling `REQ-jwt-bearer-auth-v2` row (Milestone 12
   Epic 5) carries the same pointer. WEB-01 owns the resolution; this hand-off does not pre-empt it.
2. **The multi-replica correctness edge, `Verified open`.** `REQ-k8s-manifests`'s row (Milestone 12
   Epic 7) is `Shipped, correctness question open → Phase 14 / WEB-02` — `k8s/deployment.yaml` ships
   liveness/readiness probes for multi-replica serving against an in-process token store, and
   `REQ-health-ready-endpoints`'s row cross-references it: the readiness probe's shallow check does
   not address the shared-store question. Not a scaling optimisation; a correctness question WEB-02
   owns.
3. **ADR-0038's placement answer and its stated cost.** ADR-0038
   (`.planning/decisions/0038-agent-provisioner-placement.md`) ratifies `AgentProvisioner` staying in
   `crates/paladin-web`. Its own `## Downstream Consumers` names the cost explicitly: a future
   non-HTTP consumer (the queue-worker or sidecar topology) would need to depend on the HTTP adapter
   crate to reuse the trait, or duplicate it — the split-`AgentSpec`-into-a-domain-spec-and-an-HTTP-DTO
   alternative (option-b at plan 13-09's checkpoint) was declined on `.rs`-boundary grounds, not
   because the cost is zero. Cite the ADR directly if this seam is revisited.
4. **ADR-0039 as half of WEB-04's required "stated relationship".** ADR-0039
   (`.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md`) ratifies the absence of
   Garrison and Arsenal on HTTP-served agents as a **permanent property of the shipped topology**,
   `must change` verdict already executed (the two doc-page corrections, this same plan's Task 1
   phase-gate). WEB-04 needs a "stated relationship" between Arsenal/MCP and a future `LlmPort` tool
   surface; ADR-0039 supplies the HTTP-agent half of that relationship (no Arsenal wiring, by design)
   — WEB-04 still owns the LLM-tool-calling half (`REQ-llm-tool-calling-port`,
   `REQ-llm-tool-calling-adapters`, both `Verified open → WEB-04` in the ledger).
5. **Two `Verified open` rows this ledger's own close-out amendment found, not previously in any
   Phase-14 hand-off.** `REQ-fail-closed-auth-posture` (Milestone 12 Epic 5) — the fail-closed auth
   posture's code path exists and matches its requirement's shape, but no test drives the `Err` branch
   or observes a real refusal; per D-03 this cannot be marked `Shipped`. No downstream requirement
   currently owns it; Phase 14 is the nearest natural owner given it sits in the same
   `crates/paladin-web`/`paladin-server` auth surface as WEB-01/WEB-02.

6. **One ORCH-03 residue the `.rs` boundary put out of reach — dated 2026-08-10, from
   `13-REVIEW.md` CR-01.** `docs/src/deployment-topologies/sidecar.md` embeds its caller-side
   example verbatim from `crates/doc-examples/src/sidecar.rs` via mdBook `{{#include}}`, and the
   page claims that example "matches the current API". Plan 13-08 corrected the page's own prose to
   the ADR-0037 route (`POST /v1/agents/{id}/execute`, line 30), but the embedded example still
   builds `{base_url}/agents/{agent}/execute` at `crates/doc-examples/src/sidecar.rs:34`, and the
   same unprefixed path appears in that file's doc comment at `:25`. The live server serves the
   agent API only under `API_V1_PREFIX = "/v1"`
   (`crates/paladin-web/src/agent_controller.rs:723`, asserted by
   `spec_paths_are_versioned_under_v1` at `crates/paladin-web/src/openapi.rs:103`), so a reader
   copying the rendered example writes a client that receives `404 Not Found`. This is ORCH-03's
   own failure mode — a reader applying the document literally writes against a path that does not
   exist — surviving inside a rendered page whose prose is correct.
   **Why it was not fixed in Phase 13:** the defect lives in a `.rs` file, and Phase 13's D-19
   boundary admits no `.rs` change; the close-out's zero-`.rs` boundary assertion
   (`git diff --stat` against base `e12f18306ca9a80b1c3301e6afca31602e7c41ec`) depends on that
   holding. Deferring was chosen deliberately over breaching the boundary — recorded here rather
   than left silent, per D-22. **ORCH-03 remains `[x]`**: its five named items were all corrected
   at source; this is a sixth site the requirement's own done-when reaches but its scope did not
   name, found by post-execution review.
   **What Phase 14 owes it:** change `:34` to `{base_url}/v1/agents/{agent}/execute` and the `:25`
   doc comment to match. `cargo check` cannot catch this (the URL is an opaque string literal), so
   pair the fix with an assertion tying the literal back to
   `paladin_web::agent_controller::API_V1_PREFIX`, or the drift silently returns on the next prefix
   change. Phase 14 is the natural owner: it already holds WEB-01/WEB-02 on the same
   `crates/paladin-web` route-and-auth surface.

**Evidence:** `.planning/ledgers/milestone-09-12.md` rows `REQ-opaque-bearer-token-adapter-v1`,
`REQ-jwt-bearer-auth-v2`, `REQ-k8s-manifests`, `REQ-health-ready-endpoints`,
`REQ-fail-closed-auth-posture`, `REQ-llm-tool-calling-port`, `REQ-llm-tool-calling-adapters`;
`.planning/decisions/0038-agent-provisioner-placement.md`;
`.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md`; this ledger's own `## Phase 13
close-out amendments (2026-08-10)` section, "Reconciled against the two highlight tables" paragraph;
`.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-REVIEW.md` CR-01;
`crates/doc-examples/src/sidecar.rs:25,34`; `crates/paladin-web/src/agent_controller.rs:723`;
`crates/paladin-web/src/openapi.rs:103`.

#### Hand-off to Phase 15 / PIPE-01 … — dated 2026-08-10 (plan 13-13)

1. **The measured 15-job `ci.yml` list, corrected at source.** `.planning/ledgers/milestone-09-12.md`'s
   own head-note paragraph 2 re-ran `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml` this
   phase and found **fifteen** job ids, not the fourteen `intel/code-verification.md:539-540`
   recorded — `examples` and `kubernetes-smoke` are present with no corresponding run-5 entry, and
   `security` is gone (Phase 9, D-05). PIPE-01's own text is corrected at source by plan 13-10 (D-08);
   this hand-off points at that correction rather than repeating the job list.
2. **`scripts/check-api-surface.sh:6` — closed in the script, open in four requirement texts.** The
   `.project/current-exports.txt` baseline exists and the script reads the dotted path correctly
   (ledger head-note paragraph 3); the *documentation* half — four Milestone 12 requirement texts
   still naming the undotted path, carrying Phase 8's dated banners already — stays open. Not a sixth
   ORCH-03 item (D-09); PIPE-04's own scope.
3. **Deferred-QA Epic 25's coverage-threshold variant, recorded on both sides, unresolved.** A **78%
   hard gate** (parent PRD FR-25.3) versus a **phased 70 → 74 → 78 ramp** (Epic 25 FR-25.6), with the
   parent PRD's own OQ-3 recorded Open. Joins variant group 30 alongside the coverage gate's other
   three positions. `.planning/ledgers/milestone-09-12.md` rows `REQ-ci-combined-coverage-job` and
   `REQ-codecov-config-thresholds` (both `Verified open → PIPE-02`) carry both sides; PIPE-02 picks
   one.
4. **The eight deprecated GitHub Action references, with stale line numbers.** `REQ-modernize-github-actions`'s
   row is `Partially open → PIPE-04` — the dangling `on: schedule` block (FR-25.2) is the one closed
   item; `actions-rs/toolchain@v1` (×4), `actions/cache@v3` (×3), `codecov/codecov-action@v3` (×1)
   remain, and run 5's own line numbers for them are stale by D-08 — re-grep before acting, don't
   trust the cited line numbers.
5. **The shared mock-infrastructure prerequisite and the two coverage registers it unblocks.**
   `REQ-mock-infrastructure` (`Verified open, with one correction → DEFER-01`) is the shared
   prerequisite for both `REQ-user-service-test-coverage` (`DEFER-02`, its Milestone-8-vs-Deferred-QA
   sequencing collision already resolved by ADR-0034 — split withdrawn, unsplit file is the sizing
   basis) and `REQ-listener-service-test-coverage` (`DEFER-03`, stale by both path and baseline
   number — the module ships as `src/application/services/orchestration/listener.rs`, 538 lines, and
   Milestone 9 Epic 2 already added direct-exercising tests since the 57.83% baseline was struck).

**Evidence:** `.planning/ledgers/milestone-09-12.md` head-note paragraph 2 (the 15-job list) and rows
`REQ-modernize-github-actions`, `REQ-mock-infrastructure`, `REQ-user-service-test-coverage`,
`REQ-listener-service-test-coverage`, `REQ-ci-combined-coverage-job`,
`REQ-codecov-config-thresholds`; `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`;
`REQUIREMENTS.md`'s own PIPE-04 D-09 four-requirement-text pointer.

#### Hand-off to Phase 16 / DOCS-01 … DOCS-04 — dated 2026-08-10 (plan 13-13)

**This is the last ground-truth phase in the corpus, so this block is the final word on what Phase 16
inherits — nothing further will re-derive these findings.**

1. **ORCH-02's verdict: Milestone 11's 26 open items are the corpus's only genuine open count.** All
   fourteen target files exist (`REQ-user-guides-rewrite`, `REQ-deployment-operations-docs-update`,
   both `Verified open (content) → Phase 16 / DOCS-01`), so file existence settles nothing — they are
   settleable only by content, per ORCH-02's own closed verdict above.
2. **D-13(d)'s split — a closed relocation, an open rewrite.** `REQ-arch-doc-modernization`'s row is a
   dual-fact row per D-00f: `Shipped (relocated)` (the file moved from
   `docs/Design/Design_and_Architecture.md` to `docs/src/appendix/design-and-architecture.md`,
   Milestone 11 Epic 2) **and** `Verified open → DOCS-02` (re-measured this session's own ledger
   amendment: still exactly **311** lines, zero mentions of Commander, Council, Conclave, Grove,
   Maneuver, Sanctum or Sentinel, zero mermaid blocks — the seven missing subsystems). The relocation
   hid the gap rather than closing it, because Milestone 11 Epic 3's own non-goals exempt exactly the
   chapter its own Epic 2 moved the file into.
3. **D-13(e)'s README and demos finding.** `REQ-asciinema-demos`'s row: `docs/assets/` **does not
   exist at all** (not merely empty, a correction plan 13-11 made to the plan's own stale premise),
   `docs/DEMOS.md` does not exist, and the README (193 lines, rewritten by Milestone 11 Epic 5) has no
   demos section at all — the clause targets a document that has changed shape. `docs/src/assets/`
   (a different, unrelated path) holds six architecture SVGs, not demo content.
4. **The pre-existing `mdbook build` failure, unowned by any phase.** Two broken links —
   `deployment/docker.md:118` (a link outside the book root) and
   `user-guides/tool-integration.md:324` (an incomplete reference link) — both discovered by this
   phase's own research and confirmed still present, unchanged, by this same plan's Task 1 phase-gate
   re-run (`mdbook build docs/` → exit `101`, identical two errors, re-confirmed 2026-08-10). Both
   commits post-date the last successful `docs.yml` CI run (2026-07-06). Neither is caused by, nor
   corrected by, this phase's three documentation edits (`sidecar.md`, `http-service-host.md`,
   `overview.md`) — no phase owns fixing them yet.
5. **`REQ-rustdoc-zero-warnings` and `REQ-public-api-doc-audit`**, both carrying DOCS-03: the
   `cargo doc` zero-warning bar is already ratified (ADR-0033, Phase 10) and the measured 20-warning
   residue is already assigned to Phase 16 by that ADR — DOCS-03 applies the already-decided bar, it
   does not re-litigate whether one should exist.

**Evidence:** `.planning/ledgers/milestone-09-12.md` rows `REQ-user-guides-rewrite`,
`REQ-deployment-operations-docs-update`, `REQ-arch-doc-modernization`, `REQ-asciinema-demos`,
`REQ-rustdoc-zero-warnings`, `REQ-public-api-doc-audit`; this same plan's Task 1 `mdbook build`
phase-gate re-run; `.planning/decisions/0033-cargo-doc-warning-bar.md`.

### API contract truthfulness (WEB)

Four items with one shape: **the project advertises a capability through an interface, and the
implementation behind that interface does something else.** Two are in the HTTP auth surface, two
in the LLM port. All four are verified against the tree, and three of the four are cheap.

- [ ] **WEB-01**: The agent API's token mechanism has **one** answer, and the contract matches the
      implementation. Two documents specify incompatible mechanisms:
      **M9 Epic 5 §6.1** records "**Chosen:** opaque, randomly-generated bearer tokens with a
      server-side hashed store", with rationale (avoids a `jsonwebtoken` dependency and a
      signing-key management story; supports **immediate revocation**, which stateless JWTs cannot;
      trivially deterministic to unit test; no new dependencies) and lists "JWT/OIDC/OAuth or any
      external identity provider integration" as an explicit §5 **non-goal**.
      **M12 Epic 5 FR-2** requires "a **JWT** via `Authorization: Bearer <token>`, verified through
      the existing `AuthPort::verify_token`", with `http.auth.jwt.enabled`,
      `AgentAuthConfig { enabled, api_keys, jwt: Option<Arc<dyn AuthPort>> }`, and a
      `paladin-server` that "constructs an `AuthPort` **JWT verifier** when configured".
      **Neither variant wins cleanly in the tree, which is why this cannot be auto-resolved.**
      `crates/paladin-web/src/agent_auth.rs` implements the **v2 shape** — bearer checked first then
      `x-api-key`, constant-time key comparison, the `jwt` field, a `MockJwt` test double, a
      redaction test — while `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returns
      **nothing** and the only `AuthPort` implementation in the workspace is
      `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`, **v1's opaque, in-process,
      hashed-token store**. The module documents its verifier as JWT throughout — module docs, the
      field name, the "bearer JWT checked first" comment. M12 Epic 5 **Open Question 4** ("which
      concrete `AuthPort` impl does `paladin-server` wire, and what does it need — signing
      secret/algorithm — from config/env?") is **unanswered because it is unanswerable for the
      shipped adapter**: an opaque-token store has neither.
      **Done when** one mechanism is recorded and the other side is brought into line: either
      (a) keep opaque tokens and correct the Milestone 12 vocabulary, config keys, module
      documentation and the OpenAPI security scheme, or (b) add a real JWT `AuthPort` implementation
      and answer OQ-4. **Not done while the contract and the implementation describe different
      things.** Variants preserved unmerged as **group 29**; this requirement records the answer and
      applies it, it does not pick a winner inside this file.
      *Derives: REQ-opaque-bearer-token-adapter-v1, REQ-jwt-bearer-auth-v2, REQ-auth-port,
      REQ-api-key-auth, REQ-openapi-spec-generation; `intel/code-verification.md` run-5
      verified-open finding 7; INGEST-CONFLICTS run-5 warning on the competing token mechanism.*

- [ ] **WEB-02**: Multi-replica token verification is correct, or the deployment says it is not
      supported. **M9 Epic 5 §6.1 recorded the trade-off in its own words:** "tokens are validated
      against an in-process store, so a **multi-process deployment would later need a shared
      store**. This is acceptable because validation is hidden behind `AuthPort`, so the store can
      be swapped without touching the web layer." **M12 Epic 7 §4.2 then shipped
      `k8s/deployment.yaml` and `k8s/service.yaml` with liveness and readiness probes** — artefacts
      whose entire purpose is multi-process serving.
      Under more than one replica, **a token issued by one pod will not verify on another**, so
      authenticated requests fail non-deterministically depending on which pod they land on. No
      requirement anywhere in the 199-document corpus covers the shared store M9 anticipated, and
      **neither document references the other**.
      **Done when** either `k8s/deployment.yaml` pins `replicas: 1` with the reason recorded and the
      deployment documentation states the limitation, **or** a shared-store `AuthPort`
      implementation exists and a test proves a token issued against one instance verifies against
      another. **This is a correctness question, not a scaling optimisation** — and M9 §6.2 states
      the port was designed to permit exactly this swap, so the expensive option is cheaper than it
      looks.
      *Derives: REQ-opaque-bearer-token-adapter-v1 (§6.1 trade-off), REQ-auth-port (§6.2),
      REQ-k8s-manifests, REQ-deployment-topology-doc-update; `intel/code-verification.md` run-5
      verified-open finding 7; INGEST-CONFLICTS run-5 warning on the in-process store versus the
      Kubernetes Deployment.*

- [ ] **WEB-03**: `ProviderCapabilities` reports the capability the adapters actually have.
      The Deferred-QA problem statement stands **unchanged in the tree**: "All three LLM adapters
      (OpenAI, DeepSeek, Anthropic) declare tool-calling capabilities in `ProviderCapabilities` but
      hardcode `function_call: None`." Verified: `crates/paladin-ports/src/output/llm_port.rs` has
      **no `tools` field** — its only two occurrences of "tools" are doc-comment prose, one of which
      reads `// No tools, rely on prompting` — and greps across `crates/paladin-ports/src` and
      `crates/paladin-llm/src` for `struct ToolDefinition`, `struct ToolCall` and `tool_calls`
      return **zero matches**.
      **Any consumer branching on the capability flag gets the wrong answer.** That is a correctness
      defect **independent of whether tool calling is ever built** (WEB-04), and it is correctable
      today. **Done when** the flag matches the implementation for each of the three adapters and a
      test asserts the correspondence, so the two cannot drift apart again.
      *Derives: REQ-llm-tool-calling-adapters (FR-27.3 to FR-27.7), REQ-llm-tool-calling-port;
      `intel/code-verification.md` run-5 verified-open finding 5; INGEST-CONFLICTS run-5 warning on
      Epic 27 and `ProviderCapabilities`.*

- [ ] **WEB-04**: LLM tool calling is either in scope with a plan, or withdrawn with a reason.
      Deferred-QA **Epic 27 is verified entirely unimplemented** (see WEB-03 for the evidence). It
      is also the most expensive item in the register: it modifies the `LlmPort` trait, which the
      PRD itself flags as "**a breaking change to the port interface**" requiring all adapters to
      change together, and **both** of its open questions are unanswered — does DeepSeek's API
      support tool calling, and is OpenAI's JSON Schema canonical or is a provider-agnostic schema
      needed? Weighing against it: **Arsenal/MCP already provides tool execution** through a
      different seam, and Milestone 12 explicitly excludes Arsenal from HTTP-served agents
      (ORCH-04b), so the two tool surfaces would need a stated relationship.
      **Done when** a recorded decision exists — build it, following the phased approach the PRD
      itself names (1: add `tools` as `Option<…>`, backward compatible with `None` = no tools;
      2: implement sending; 3: implement parsing, including Anthropic's `input_schema` versus
      `parameters` translation and streamed tool-call deltas; 4: live API tests behind
      `live-api-tests` and `#[ignore]`) — **or** withdraw Epic 27 with the reason recorded, leaving
      `ProviderCapabilities` honest by WEB-03. **Not done by leaving it as a deferred register entry
      for a fourth time.**
      *Derives: REQ-llm-tool-calling-port (FR-27.1, FR-27.2), REQ-llm-tool-calling-adapters
      (FR-27.3 to FR-27.7); INGEST-CONFLICTS run-5 warning on Epic 27.*

### Coverage & CI quality gates (PIPE, DEFER)

Deferred-QA Epic 25 first — the register's own recommendation, because it "establishes quality
gates that validate all subsequent work" — then the Epic 28/29 coverage register those gates
measure. **Verified open item by item**, not inferred.

**One correction to how this scope is usually described: coverage tooling is partially built, not
absent.** `.codecov.yml` does not exist and `ci.yml` contains no coverage gate or `llvm-cov`
reference at all — but `integration-tests.yml:117-123` **does** run `cargo install cargo-llvm-cov`,
`cargo llvm-cov --features integration-tests --lcov` and `codecov/codecov-action@v3`. The
integration-only coverage path exists; what is missing is the combined unit + integration job, the
threshold configuration and the local targets. Scope accordingly.

- [ ] **PIPE-01**: The 43 CLI snapshot tests and a benchmark compile check run in CI.
      A **`cli-tests`** job (or a step in `test`) runs `cargo test --test cli` across every snapshot
      test in `tests/cli/` — table, progress, error and help output, **43 total** — on every push
      and PR to `main`/`develop`, on `stable`, with the same cargo cache config as other jobs,
      failing the pipeline on any snapshot failure. It **requires no external services**, so if it
      is a separate job it runs in parallel with `lint` and `test` with no `needs:`.
      A **`bench-check`** job runs `cargo bench --no-run`, catching API breakage and benchmark
      bitrot **without executing benchmarks** — no performance numbers, no Criterion output. The
      existing scheduled/manual `benchmark` job is unchanged.
      Neither exists today; `ci.yml`'s 14 job ids are `lint`, `security-audit`, `cargo-deny`,
      `osv-scanner`, `api-surface`, `test`, `crate-isolation`, `integration-tests`, `security`,
      `docker`, `e2e-tests`, `benchmark`, `benchmark-regression-signal`, `publish-dry-run`.
      **Note the inversion**: `benchmark-regression-signal` — which Epic 25's own non-goals list as
      a *future enhancement, explicitly out of scope* — ships at `ci.yml:531` from Milestone 7
      Epic 3, while the compile-check prerequisite does not.
      *Derives: REQ-ci-cli-snapshot-job (FR-25.3), REQ-ci-bench-check-job (FR-25.4);
      `intel/code-verification.md` run-5 verified-open finding 3.*

      **Corrected (dated 2026-08-10, D-08, plan 13-10):** ~~"Neither exists today; `ci.yml`'s 14 job
      ids are `lint`, `security-audit`, `cargo-deny`, `osv-scanner`, `api-surface`, `test`,
      `crate-isolation`, `integration-tests`, `security`, `docker`, `e2e-tests`, `benchmark`,
      `benchmark-regression-signal`, `publish-dry-run`"~~ — this list, quoting
      `intel/code-verification.md:539-540` verbatim, is stale: `security` was deleted by Phase 9
      (D-05) and two jobs the run-5 list does not name are present. Re-run this session:
      `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml` returns fifteen entries — `lint`
      (`:21`), `security-audit` (`:61`), `cargo-deny` (`:81`), `osv-scanner` (`:126`), `api-surface`
      (`:155`), `test` (`:206`), **`examples`** (`:245`), `crate-isolation` (`:319`),
      `integration-tests` (`:374`), `docker` (`:494`), **`kubernetes-smoke`** (`:611`), `e2e-tests`
      (`:718`), `benchmark` (`:779`), `benchmark-regression-signal` (`:812`), `publish-dry-run`
      (`:898`) — **fifteen jobs**, not fourteen. The two additions (`examples`, `kubernetes-smoke`)
      are recorded as present without attributing them to a commit — what is there, not a guess about
      how it got there. **Hand-off: Phase 15 / PIPE-01** inherits this measured fifteen-job list so
      it does not plan against a fourteen-job pipeline that no longer exists.
      **D-09 residue, recorded in the same hand-off:** `sed -n '1,10p' scripts/check-api-surface.sh`
      confirms `check-api-surface.sh:6`: `BASELINE="${1:-.project/current-exports.txt}"`, and
      `test -f .project/current-exports.txt` confirms the file is present (446 KB). Run-5 finding 8's
      "the `api-surface` CI job fails on every run" consequence clause is therefore **closed in the
      script** and open only in the four Milestone 12 requirement texts already carrying Phase 8's
      dated banners (M12 Epic 1 §7, Epic 5 §7, Epic 6 `cross_refs`, Epic 7 FR-4.6) — not as a sixth
      ORCH-03 item.

      **Inbound scope note, added 2026-08-05 by plan 06-07 (D-10):** Phase 6's Epic 24 cluster
      `8.0` (`tasks-test-hardening-benchmarks-qa.md`) named the `cli-tests` and `bench-check` jobs
      as genuinely outstanding and deferred them here with a written reason rather than building
      them in Phase 6 — Phase 15's own register puts its quality-gate work first, ahead of
      everything it gates. See `.planning/ledgers/milestone-02-03.md`'s Epic 24 block verdict (the
      `8.0` row) and `### Phase 6 CLOSE-02 scope` section for the full reason and the two rejected
      alternatives; this note is the reverse pointer so a Phase 15 planner does not have to
      rediscover the link.

- [ ] **PIPE-02**: Unit and integration coverage is measured together in CI and gated at **one
      recorded threshold**. A `coverage` job installs `cargo-llvm-cov` via
      `taiki-e/install-action@v2` with `tool: cargo-llvm-cov@0.7.1` (pre-built binaries, ~30 s
      versus 3-5 min for `cargo install`), runs
      `cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info`, **starts Redis and
      MinIO** so integration tests execute during collection, uploads to Codecov with
      `flags: combined` and `fail_ci_if_error: true`, and saves an HTML report as an
      `actions/upload-artifact@v4` artifact with 14-day retention. `.codecov.yml` lands at the
      repository root with `require_ci_to_pass: true`, `precision: 2`, `round: down`,
      `range: "70...100"`, project and patch status blocks, PR comment layout
      `"reach,diff,flags,files"`, and `ignore` covering `tests/**`, `benches/**`, `examples/**`,
      `migrations/**`, `scripts/**`, `flat/**`. A `CODECOV_TOKEN` repository secret must be
      configured or uploads may fail **silently**, especially on fork PRs.
      **The entry threshold is contested and must be settled before `.codecov.yml` is written.**
      The parent PRD FR-25.3 item 10 mandates "a coverage threshold gate of **78%** minimum. PRs
      dropping below this threshold must fail", and G2 measures success against it. Epic 25 FR-25.6
      and Appendix C specify a **phased ramp** — Phase 1 project 70%, Phase 2 74%, Phase 3 78%, with
      patch 80% throughout, each phase a single `target:` edit. **The parent PRD's own Open Question
      3 asks exactly this and is recorded Open**; the child Epic answered it unilaterally. Measured
      coverage is 76-77% with the deferred modules included, so **78% would fail on day one and 70%
      would pass**. Preserved as **group 30**.
      **This is the sixth position on the coverage gate in this corpus** — after 80% (nine
      Milestone-1 PRDs), 85% (unit-test-improvements), 75% layered per tier (the Milestone 3 plan)
      and 80/70 re-asserted (Epic 24). RECON-07 records the first answer and VERIFY-05 extends it
      across the four earlier positions; **PIPE-02 must land on the same number**, or record why the
      CI gate differs from the project gate.
      Epic 25's **Open Question 3** — remove or retain `integration-tests.yml`'s existing coverage
      step, given the combined report subsumes it — must be answered rather than left open; the
      PRD's own recommendation is removal, to avoid duplicate uploads.
      *Derives: REQ-ci-combined-coverage-job (FR-25.5, FR-25.9), REQ-codecov-config-thresholds
      (FR-25.6, FR-25.10, Appendix C); `intel/code-verification.md` run-5 verified-open finding 3;
      INGEST-CONFLICTS run-5 warning on the competing initial coverage threshold. Couples to
      RECON-07 (Phase 1) and VERIFY-05 (Phase 5).*

      **Inbound scope note, added 2026-08-05 by plan 06-07 (D-10):** Phase 6's Epic 24 cluster
      `8.0` (`tasks-test-hardening-benchmarks-qa.md`) named the `coverage` job and `.codecov.yml`
      as genuinely outstanding and deferred them here with a written reason rather than picking a
      threshold in Phase 6 — this requirement's own contested-threshold text above (six competing
      positions) is exactly why Phase 6 declined to settle it; **PIPE-02 still owns that decision.**
      See `.planning/ledgers/milestone-02-03.md`'s Epic 24 block verdict (the `8.0` row) and
      `### Phase 6 CLOSE-02 scope` section for the full reason and the two rejected alternatives;
      this note is the reverse pointer so a Phase 15 planner does not have to rediscover the link.

- [ ] **PIPE-03**: Coverage and the two new test targets are runnable locally. A new **Coverage**
      section in the `Makefile` between Testing and Code Quality with `coverage`
      (`cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info`) and
      `coverage-html` (`--html --output-dir target/coverage`); `test-cli` (`cargo test --test cli`)
      and `bench-check` (`cargo bench --no-run`) in the Testing section; `ci-test` updated to
      include `test-cli`; and a new `ci-full: ci-test coverage`. **None of the four targets exists
      today, and the `Makefile` contains no `llvm-cov` reference at all** — so the CI gate PIPE-02
      adds would be unreproducible locally, which is the failure mode this requirement prevents.
      *Derives: REQ-makefile-coverage-targets (FR-25.7); `intel/code-verification.md` run-5
      verified-open finding 3.*

- [ ] **PIPE-04**: No deprecated GitHub Action remains in any workflow. **Eight references,
      verified by line:** `actions-rs/toolchain@v1` (deprecated and unmaintained) at `ci.yml:147`,
      `ci.yml:317`, `ci.yml:507` and `integration-tests.yml:71`; `actions/cache@v3` at
      `integration-tests.yml:78`, `:84` and `:90`; `codecov/codecov-action@v3` at
      `integration-tests.yml:123`. Replace with `dtolnay/rust-toolchain@stable`/`@beta`/`@nightly`
      using the `components:` input for `rustfmt`/`clippy`; `actions/cache@v4`;
      `codecov/codecov-action@v4` with `token: ${{ secrets.CODECOV_TOKEN }}`; and
      `actions/checkout@v4` where a v3 remains. Validate all three workflows with
      `actionlint`/`yamllint`, **zero errors**.
      **One FR-25.2 item is already satisfied**: the dangling `on: schedule` block at the old
      `ci.yml` lines ~336-340 — syntactically invalid because the top-level `on:` is already defined
      at line 3 — is **gone**. `ci.yml` has exactly one `on:` at line 3 and no `schedule:`/`cron:`
      key. That is the only Epic 25 item found closed.
      **Scope note:** these four `actions-rs` references were previously folded into DEBT-01 because
      `ci.yml:147` sits inside the `api-surface` job. **They now belong here.** DEBT-01 keeps the
      `project/current-exports.txt` baseline path; PIPE-04 owns the action versions. Recorded so
      neither is planned twice.
      *Derives: REQ-modernize-github-actions (FR-25.1, FR-25.2), REQ-workspace-ci-upgrade (run 3);
      `intel/code-verification.md` run-5 verified-open finding 3.*

- [ ] **PIPE-05**: `CONTRIBUTING.md` has a Code Coverage section that matches what CI does.
      Prerequisites (`cargo install cargo-llvm-cov` / `cargo binstall`), local generation
      (`make coverage`, `make coverage-html`), how to read LCOV and HTML output, Codecov PR-comment
      behaviour and the dashboard link, **the threshold policy PIPE-02 settles** and what `project`
      versus `patch` mean, and troubleshooting (tool not found, low patch coverage, upload failures
      and `CODECOV_TOKEN`). Existing `cargo tarpaulin` references are updated to note that
      `cargo-llvm-cov` is the project standard, with tarpaulin retained as an alternative.
      **Done when** a new contributor can reproduce the CI coverage number locally from the
      document alone.
      *Derives: REQ-contributing-coverage-docs (FR-25.8); depends on PIPE-02 for the threshold and
      PIPE-03 for the targets it documents.*

- [ ] **DEFER-01**: The shared mock and async-test infrastructure that three registers name as a
      prerequisite exists. `MockUserRepository` (`UserRepositoryPort`, in-memory `HashMap`),
      `MockLogPort` (`Vec<LogEntry>` for assertion), `MockNotificationService` (sent-messages
      vector), `MockEventSource` (configurable event sequences with controlled timing),
      `MockTriggerExecutor` (records executions), Tokio time-control utilities
      (`tokio::time::pause()`/`advance()`) and test event generators — **all `Send + Sync`** for
      async test compatibility, using the `Arc<Mutex<Vec<T>>>` pattern for recording calls, designed
      as **reusable components rather than per-test one-offs**, with a `mod.rs` re-exporting them.
      **Verified open in the specified shape.** No `tests/common/` directory exists. The workspace's
      mocks are `tests/helpers/{mock_llm_adapter,mock_arsenal_adapter,mock_paladin_port}.rs` plus
      `tests/unit/mock_llm_adapter_test.rs` — **a different location and a disjoint set**; none of
      the five named mocks exists. Two sub-decisions are unanswered and should not be made by
      default: whether to place this at the PRD's `tests/common/` or reconcile with the existing
      `tests/helpers/` convention, and **Open Question 2** — adopt `mockall` or keep hand-written
      mocks (compile-time cost versus boilerplate).
      **This is the shared prerequisite for both coverage epics** and the stated reason the
      recommended order runs Epic 28 before Epic 29; roughly **6-10 of the 35-45 estimated hours**
      are this infrastructure. `DEFERRED_COVERAGE.md` lists "create reusable mock infrastructure
      patterns", "document testing best practices" and "establish concurrency testing patterns" as
      its three **unchecked** prerequisites for all deferred coverage work.
      *Derives: REQ-mock-infrastructure (FR-28.1, FR-29.1), REQ-deferred-coverage-register;
      `intel/code-verification.md` run-5 verified-open finding 6; INGEST-CONFLICTS run-5 warning on
      the missing mock prerequisite.*

- [ ] **DEFER-02**: `user_service.rs` reaches the coverage gate — **sequenced deliberately against
      deferred item D2, not scheduled independently of it.** Recorded profile: **488 LOC, ~4.23%
      coverage, complexity High, production status Active** (used in web controllers and CLI
      commands); effort 15-20 h; risk of deferral **Medium** — "Authentication logic is critical
      security component". Test scope: registration (happy path, duplicate username, duplicate
      email, invalid username formats, invalid email, password hashing), authentication (correct and
      incorrect password, non-existent user, deactivated account, login-attempt tracking), profile
      (update, email change with verification, activation/deactivation, verification flow), queries,
      and edge cases (repository error, **notification failure must not block registration**,
      concurrent same-username registration, Unicode inputs, empty/whitespace inputs), verified
      at **≥ 80%** by a module-targeted `cargo llvm-cov` with intentionally untested paths justified.
      **Two ingested registers propose incompatible next actions on this one file.** Deferred-QA
      Epic 28 plans to **test** it. Milestone 8 `deferred-items.md` **D2** — carried as FACADE-02 in
      Phase 11 — plans to **split** it: trait plus DTOs to `paladin-core`/`paladin-ports`, the
      concrete implementation (which depends on repository, log and notification ports plus argon2)
      to a facade app-service, because it is mis-layered. The file still ships at
      `src/core/platform/manager/user_service.rs` (19,046 bytes) and is one of only **four** files
      left in that directory. Run 4 established `deferred-items.md` as the highest-fidelity document
      in the corpus, so D2's recommendation carries real weight.
      **Done when** the sequence is recorded and executed in that order. Splitting first and testing
      the resulting units is the cheaper order, but it **changes Epic 28's estimate and its mock
      set**, so DEFER-01's scope depends on this answer. Doing them independently means doing the
      work twice.
      *Derives: REQ-user-service-test-coverage (FR-28.2 to FR-28.7; DEFERRED_COVERAGE Module 1),
      REQ-m8-deferred-items-register D2 (run 4); INGEST-CONFLICTS run-5 warning on two registers
      proposing incompatible actions on `user_service.rs`. **Couples to FACADE-02 (Phase 11).***

- [ ] **DEFER-03**: The listener service's coverage scope is **re-measured before it is planned**.
      Recorded profile: **602 LOC, ~57.83% coverage, complexity Very High, production status
      Active** (event-driven system core); effort 20-25 h; risk of deferral **Medium-High** —
      "Event-driven systems are notoriously hard to debug; concurrency bugs can be subtle and
      intermittent; trigger generation logic is business-critical".
      **Both the path and the number are stale.** The module ships as
      `src/application/services/orchestration/listener.rs` (`ListenerOrchestrator`) after the
      Milestone 6 Epic 2 relocation — `src/core/platform/manager/listener_service.rs`, which both
      the PRD and `DEFERRED_COVERAGE.md` name, no longer exists. And **Milestone 9 Epic 2 (FR-16 to
      FR-21) subsequently added match, no-match, fan-out, rate-limit and trigger-to-dispatch tests
      against that exact module**, so the 57.83% baseline dated 2026-02-14 predates roughly four
      months of test work and its effort estimate, risk rating and story-point sizing are all
      anchored to a figure that has almost certainly moved.
      **Treat the scope as real and the arithmetic as needing re-measurement.** The scope that
      survives: registration and lifecycle, delivery and filtering with **ordering guarantees**,
      trigger status tracking and retry, statistics and health-check status, and — the genuinely
      valuable part — **concurrency and stress**: concurrent emission from multiple producers,
      concurrent registration/unregistration *during* processing, a **1000+ event burst**,
      **deadlock detection under Tokio `Mutex` + `RwLock` contention**, graceful shutdown during
      active processing, and distributed tracing for event flows.
      **Done when** a current `cargo llvm-cov` figure for the module exists, the remaining scope is
      stated against it, and the effort estimate is re-derived rather than inherited.
      *Derives: REQ-listener-service-test-coverage (FR-29.2 to FR-29.7; DEFERRED_COVERAGE Module 2),
      REQ-event-trigger-job-pipeline (M9 Epic 2 FR-16 to FR-21), REQ-deferred-coverage-register;
      INGEST-CONFLICTS run-5 warning on the stale Epic 29 baseline.*

### Documentation currency & the architecture gap (DOCS)

- [ ] **DOCS-01**: Milestone 11's 26 open items are settled **by content**, not by presence. This is
      **the only checkbox count in all 542 that survives verification as genuinely open work.** The
      items, all in `tasks-content-rewrite.md`: **task 6.0** — six user-guide in-place updates
      (orchestration, maneuver-flow-dsl, memory-management, tool-integration, paladin-configuration,
      output-formatting); **task 7.0** — eight deployment/operations updates (docker, kubernetes,
      production, cicd, logging, monitoring, performance-tuning, troubleshooting); and **task 1.2**
      — review the full linkcheck report.
      **All fourteen target files exist** under `docs/src/user-guides/`, `docs/src/deployment/` and
      `docs/src/operations/`, and `docs/book.toml` has linkcheck at
      `warning-policy = "error"` with `follow-web-links = false`. But "update in-place" is a
      **content-currency** task, and neither file existence nor modification time is adequate
      evidence either way — mtimes are too weak an inference to record.
      **Done when** each of the fourteen is checked against the current tree — crate names (ten
      library crates, not six or nine), module paths (post-M5/M6/M8 relocations), `make` targets,
      workflow and job names, error types, feature flags — and marked current or updated; and the
      linkcheck report is reviewed. **Do not convert the count into 26 tasks and do not dismiss
      it.**
      *Derives: REQ-user-guides-rewrite, REQ-deployment-operations-docs-update,
      REQ-doc-link-repair-linkcheck, REQ-mdbook-final-review; `intel/task-completion-state.md`
      Milestone 11; INGEST-CONFLICTS run-5 warning on the 26 items needing content review.*

- [ ] **DOCS-02**: `design-and-architecture.md` is **either archive material or a live deliverable,
      and it says which.** It cannot be both, and it has been invisible for two milestones.
      The file ships at `docs/src/appendix/design-and-architecture.md` at **exactly 311 lines** —
      the identical figure the February 2026 PRD cites as the *pre-rewrite* state ("the current
      `docs/Design/Design_and_Architecture.md` (311 lines, 10 sections)"). Whole-word,
      case-insensitive counts in that file: **Commander 0, Council 0, Conclave 0, Grove 0,
      Maneuver 0, Sanctum 0, Sentinel 0**; Paladin 6, Garrison 2, Arsenal 2, Battalion 2, Herald 2,
      Citadel 1. ```mermaid blocks: **0**.
      **All seven subsystems FR-26.1 requires be documented in detail are absent — and all seven are
      verified shipped in the tree by ingest runs 1 and 2.** None of the four required Mermaid
      diagrams exists.
      **The mechanism is the finding.** Milestone 11 Epic 2's appendix escape hatch ("docs with no
      single-chapter home are placed in an `appendix/` chapter rather than dropped") relocated the
      file, and Milestone 11 Epic 3's §5 non-goals then exempted exactly that chapter from
      rewriting: "the 35 appendix files are reference/archive material and are **not rewritten in
      this Epic**". **The relocation placed the corpus's largest documentation gap into the one
      chapter nobody was required to fix.**
      **Done when** a recorded decision exists: archive it — say so, and stop tracking FR-26.1 —
      **or** give it a home outside the appendix and expand it to ~600-800 lines covering all 15+
      components, with detailed sections on Commander, Council, Conclave, Grove, Maneuver, Sanctum
      and Sentinel, the four Mermaid diagrams FR-26.1 names (overall hexagonal system architecture;
      Battalion orchestration patterns; data flow through a Paladin execution cycle; Arsenal/MCP
      tool integration flow), an AI-agent execution pipeline in the Data Flow section, an updated
      Deployment Architecture (currently marked "Draft"), a Configuration section covering
      `config.yml`, and the stale content-management framing removed. Success metric of record:
      components documented **8 of 15+ → 15+ of 15+**.
      *Derives: REQ-arch-doc-modernization (FR-26.1, items 19-25; G3), REQ-architecture-docs-update
      (M11 Epic 3); `intel/code-verification.md` run-5 verified-open finding 4; INGEST-CONFLICTS
      run-5 warning on the relocated architecture document.*

- [ ] **DOCS-03**: One `cargo doc` bar, applied, with the public API documented to it.
      **Three positions on one command across three milestones**: M7 Epic 4 §4.4.3 and M7 Epic 1
      §4.6.4 require **zero** `cargo doc --workspace --no-deps` warnings; M8 Epic 5 FR-19 relaxes
      the same command to "**warnings acceptable**; must not fail"; Deferred-QA FR-26.2 requires
      **zero** `cargo doc --no-deps` warnings **enforced in CI** by a
      `cargo doc --no-deps 2>&1 | grep -c warning` check failing if > 0, recording **12 minor
      formatting issues** at the time it was written.
      **HARD-07 (Phase 10) picks the bar; DOCS-03 applies it and adds the CI gate**, so the two are
      not planned twice. Plus FR-26.3: enumerate every `pub` item in `src/` lacking `///`
      documentation, document all of them, and add at least one `/// # Examples` block to **all
      public API entry points** — builders, service constructors, port traits — verified by
      rendering.
      **Coupled to DEBT-03 (Phase 8)**, which re-enables `paladin-ports` doctests and drops the
      `ci.yml:225 --exclude paladin-ports`: that is what makes the ~25 port traits' examples
      *executable* rather than merely present, and the port traits are the framework's primary
      integration contract.
      *Derives: REQ-rustdoc-zero-warnings (FR-26.2), REQ-public-api-doc-audit (FR-26.3),
      REQ-doc-example-compile-gate (M11 Epic 3); couples to HARD-07 (Phase 10) and DEBT-03
      (Phase 8).*

- [ ] **DOCS-04**: The demos have a decision, and `docs/assets/` stops implying pending work.
      FR-26.4 requires four `asciinema` recordings — **Basic Paladin Execution** (30-60 s),
      **Battalion Formation** (45-90 s), **Council Discussion** (60-120 s) and **Grove Routing**
      (45-90 s) — saved as `.cast` under `docs/assets/recordings/`, optionally converted to
      `.gif`/`.svg` for embedding, with README links and a `docs/DEMOS.md` index.
      **Verified open**: `docs/assets/` **exists and is empty**; `docs/DEMOS.md` does not exist.
      Two complications make this a decision rather than a task. **Open Question 4** — asciinema
      versus VHS tape files, Terminalizer or plain GIFs — is unanswered. And the README the
      requirement targets was rewritten by **Milestone 11 Epic 5** into a concise landing page with
      **no demos section**, so the embedding clause aims at a document that has since changed shape.
      Recordings also require **live LLM API keys**, which puts them outside any offline gate.
      **Done when** the demos are recorded and indexed, **or** withdrawn with the reason recorded —
      and either way `docs/assets/` is populated or removed, so an empty directory stops signalling
      work in flight.
      *Derives: REQ-asciinema-demos (FR-26.4; G4), REQ-readme-landing-page (M11 Epic 5);
      `intel/code-verification.md` run-5 verified-open finding 4.*

---

## Competing variants (preserved unmerged)

**30 variant groups, 60 entries**, carried verbatim in scope from `.planning/intel/requirements.md`
(12 from run 1, 18 from run 2, 8 from run 3, 18 from run 4, 4 from run 5). **No winner is selected
here.** Where a variant matters, the pointer below is to the codebase map,
`intel/code-verification.md` or a shipped `file:line` — the real arbiters — not to a resolution.
`INGEST-CONFLICTS.md` counts **69 warnings** across all five runs; the entry count differs because
several groups carry three entries and many warnings are not `-v1`/`-v2` pairs at all (those are
listed after group 30).

**All 69 are preserved unmerged, deliberately and at the user's explicit direction.** Variants are
expected in this corpus, and resolving past disagreements is not the goal of this ingest. Nothing
below picks a winner, including where shipped code plainly favours one side.

**Run 4 is the run where shipped code settles the most variants** — six of the eight new groups
carry a `settled-by` pointer. That is recorded as a **fact about the tree**, at the top of the
precedence order, and never as a decision taken here.

**Run 5 produced the corpus's only variant that shipped code does *not* settle.** Group 29's token
mechanism has the M12 *shape* and the M9 *mechanism* in the same tree — the API is documented as
JWT and implemented as opaque tokens — so neither side can be marked `settled-by`. Every other
variant in five runs either has one side in the tree or has neither; this is the first with half of
each.

**Run 3 closed five earlier warnings with shipped code** — groups 3, 4 and 15 below carry a
*Run-3 code verification* note, and two further run-1 warnings (the Battalion base module path,
and `metadata_output_dir`'s three claimed owners) are resolved inside groups 3 and 4. Those notes
are **observations of the tree recorded at the highest precedence tier**, not decisions taken
here; the entries themselves stay unmerged, and none is deleted.

Observations of shipped code in this section are **facts about the tree, not decisions**. Several
of them plainly favour one variant; recording that as the answer is a Phase 1 / Phase 5 ADR, and
it is deliberately not asserted here. The user has stated that variants are expected and that
resolving past disagreements is not the goal of this ingest.

### Group 1 — project-wide test coverage gate (4 positions)

- **REQ-test-coverage-target-v1** — nine Epic PRDs (Epics 1-8, 10): unit coverage ≥ 80%,
  integration ≥ 70%, measured via cargo-llvm-cov.

- **REQ-test-coverage-target-v2** — `unit-test-improvements/prd-improve-unit-test-coverage.md`:
  overall coverage MUST exceed 85%; functions under 50% MUST reach 80%; stated baseline 67.79%.

- *Run-2 third position (context, not a `REQ-*` entry)* — `Project_Plan_Milestone_3.md`
  "Cross-Cutting Concerns": layered targets, core domain ≥ 85%, application services ≥ 80%,
  infrastructure adapters ≥ 70%, CLI commands ≥ 70%, **overall ≥ 75%**.

- *Run-2 fourth position* — **REQ-epic24-quality-gates** re-asserts ≥ 80% for all modules and
  ≥ 70% integration.

- Status: **unresolved, four ways.** Measured 60.88% unit / 67.79% integration (Milestone 1) and
  ~78% overall (Milestone 3 release notes). ~78% passes the M3 plan gate, fails Epic 24's, fails
  the 85% gate. Module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%) coexist above whatever the
  global number becomes. Recording the answer is RECON-07 + VERIFY-05.

### Group 2 — valid temperature range

- **REQ-temperature-range-v1** — Epic 1 FR-2.3 / US-2: builder MUST validate `[0.0, 1.0]` and
  reject values above 1.0.

- **REQ-temperature-range-v2** — Epic 6 REQ-5: DeepSeek adapter MUST support temperature 0.0-2.0.
- Status: **unresolved.** A build-time `[0.0, 1.0]` clamp makes the DeepSeek range unreachable
  through the normal Paladin path. Run 2 adds a third overlapping surface without settling it:
  **REQ-dynamic-temperature** classifies tasks into bands (Factual 0.1-0.3, Analytical 0.3-0.5,
  Conversational 0.5-0.7, Creative 0.7-1.0) with configurable `temperature_bounds`, and the
  Epic 14 DOC states bounds of 0.1-1.0. Shipped: `TaskType` and the band logic exist in
  `src/application/services/paladin/temperature_service.rs`. See `codebase/ARCHITECTURE.md`.

### Group 3 — `BattalionConfig` field set and `metadata_output_dir` ownership (3 positions)

- **REQ-battalion-config-v1** — Epic 4 FR-4.1: `name`, `description`, `timeout_seconds`,
  `retry_policy` (struct: max_attempts / base_delay / max_delay / exponential_backoff / jitter),
  `error_strategy`, `metadata_output_dir`.

- **REQ-battalion-config-v2** — Epic 5 FR-7: `name: String`, `timeout_seconds: u64`,
  `retry_attempts: u32`, `error_strategy: ErrorStrategy`, `enable_checkpointing: bool`,
  `metadata_output_dir: Option<PathBuf>`; defaults for all fields.

- **REQ-commander-config-metadata-dir-v3** *(run 2, Epic 22 FR-10.1)* — relocates the same field to
  `CommanderConfig` in `src/core/platform/container/battalion/commander_config.rs`, with YAML
  surface `commander.metadata_output_dir` and writability validated before first execution.

- Status: **unresolved, and Epic 22 relocated the conflict rather than closing it.** Two distinct
  `BattalionConfig` structs exist in the tree
  (`crates/paladin-core/src/platform/container/battalion/mod.rs:37` and
  `crates/paladin-core/src/platform/container/citadel.rs:280`, the latter carrying
  `max_concurrency` / `timeout_seconds` / `continue_on_error`). Tree observation, not a decision:
  `metadata_output_dir` ships on `BattalionConfig` at `battalion/mod.rs:54` with a writability
  check at `:116-124`, and the export path is `crates/paladin-battalion/src/commander.rs:870` —
  i.e. the v3 `CommanderConfig` position is not the shipped location. See `codebase/STRUCTURE.md`.

- **Run-3 code verification (highest precedence, not a decision):** the shipped
  `BattalionConfig` at `battalion/mod.rs:37` is `REQ-battalion-config-v1` **exactly** —
  `name`, `description: Option<String>`, `timeout_seconds`, `retry_policy: RetryPolicy`,
  `error_strategy: ErrorStrategy`, `metadata_output_dir: Option<PathBuf>`. Epic 5's
  `retry_attempts: u32` and `enable_checkpointing: bool` are absent and `description` was not
  dropped. `CommanderConfig` **does not exist anywhere** in `crates/` or `src/`, so
  `REQ-commander-config-metadata-dir-v3` was never built and the run-2 "three competing owners"
  warning has exactly one owner. RECON-02 records this; the `citadel.rs` duplicate is untouched by
  it and still needs resolving (GAP-07).

### Group 4 — `BattalionResult` / `BattalionMetadata` field set (4 positions)

- **REQ-battalion-result-v1** — Epic 4 FR-4.2: `battalion_id`, `battalion_name`, timestamps,
  `final_output`, `paladin_results`, `status`; all intermediate results; per-Paladin and overall
  timing.

- **REQ-battalion-result-v2** — Epic 5 FR-5: `battalion_id: Uuid`, `strategy_used`,
  `paladin_results`, `final_output`, `execution_time_ms`, `status`, `metadata`
  (strategy_selection_reasoning, strategy_selection_time_ms, per_paladin_times,
  paladin_success_count, paladin_failure_count, timestamp), plus `errors: Vec<PaladinError>`.

- Third consumer, unmerged: **REQ-herald-battalion-result-fields** (Epic 8 FR-7) additionally
  requires a Battalion **type** field and **aggregated token usage** — which neither producer
  variant defines.

- **REQ-battalion-metadata-extension** *(run 2, Epic 22 FR-8)* — later position: `BattalionMetadata`
  gains `per_paladin_times`, `per_paladin_tokens` and `total_tokens`, all `Serialize`/`Deserialize`,
  located in `battalion/battalion_result.rs`. Note Epic 5 typed `per_paladin_times` as `Vec<u64>`.

- Status: **unresolved on paper.** Tree observation, not a decision: `BattalionMetadata` ships
  `per_paladin_times: HashMap<String, u64>` (`battalion/mod.rs:582`) and
  `per_paladin_tokens: HashMap<String, TokenUsage>` (`:585`) — the run-2 field names with the
  run-1 module path (`battalion/mod.rs`, not `battalion_result.rs`). See `codebase/ARCHITECTURE.md`.

- **Run-3 code verification (highest precedence, not a decision):** the shipped `BattalionResult`
  at `crates/paladin-core/src/platform/container/battalion/mod.rs:549` is a **merged superset of
  all three consumers**, with Epic 5's `metadata` map flattened into top-level fields:
  `battalion_id`, `battalion_name`, `started_at`, `completed_at`, `final_output`,
  `paladin_results: Vec<PaladinResult>`, `status: BattalionStatus`,
  `strategy_used: BattalionStrategy`, `strategy_selection_reasoning: Option<String>`,
  `strategy_selection_time_ms: u64`, `per_paladin_times: HashMap<String, u64>`,
  `per_paladin_tokens: HashMap<String, TokenUsage>`, `total_tokens: u64`,
  `paladin_success_count: usize`, `paladin_failure_count: usize`,
  `node_errors: Vec<NodeError>`. Epic 4's field set is fully present. Epic 5's is present except
  `execution_time_ms` (superseded by `per_paladin_times`) and `errors: Vec<PaladinError>`
  (superseded by `node_errors: Vec<NodeError>`, a plain-data struct, because `BattalionError` does
  not derive `Serialize`/`Deserialize` while `BattalionResult` does). Epic 8's Herald expectation
  is satisfied: Battalion type as `strategy_used`, aggregated tokens as `total_tokens` plus
  `per_paladin_tokens`. `intel/code-verification.md` states explicitly: **do not plan a
  reconciliation task.** RECON-03 records the verdict; GAP-07 no longer carries the code change.
  Note that the Milestone 5 Epic 1 decision record — despite its filename — does **not** settle
  this and never mentions `BattalionResult` (see group 19).

### Group 5 — minimum Paladin count for Formation

- **REQ-formation-min-paladins-v1** — Epic 4 FR-4.5 / FR-4.8: Formation MUST validate ≥ 2
  Paladins; Phalanx accepts ≥ 2; Majority aggregation requires ≥ 3.

- **REQ-formation-min-paladins-v2** — Epic 5 FR-1 / FR-3: Commander validates only that ≥ 1
  Paladin is provided, and Auto rule 1 routes a single Paladin to Formation as the trivial case.

- Status: **unresolved and live in shipped code** —
  `crates/paladin-core/src/platform/container/battalion/formation.rs:109` errors with
  "Formation requires at least 2 Paladins", so the documented single-Paladin Auto happy path
  fails validation at runtime.

### Group 6 — `Herald` trait signature

- **REQ-herald-trait-v1** — Epic 8 FR-1: infallible `-> String` returns;
  `format_paladin_result`, `format_battalion_result`, `format_paladin_stream -> Option<String>`,
  `format_error`; `Send + Sync`.

- **REQ-herald-trait-v2** — same PRD, section 6.2: fallible `-> Result<String, HeraldError>`;
  `format_stream_chunk`, added `finalize_stream(&ExecutionMetadata)`, `name()`, `mime_type()`.

- Status: **unresolved on paper.** The shipped trait at
  `crates/paladin-core/src/platform/container/herald.rs:49` documents a `HeraldError` return —
  i.e. the code leans fallible — and FR-10's graceful-degradation requirement cannot be expressed
  by the infallible form. Recording that as the answer is RECON-06; it is not asserted here.
  Run-2 note: **REQ-herald-type-consolidation** is a later position on the *placeholder-type* half
  of this question, and the tree carries no placeholder or TODO in `herald.rs`.

### Group 7 — Qdrant Sanctum adapter *(run 2)*

- **REQ-qdrant-sanctum-adapter-v1** — Epic 11 US-11.4 / FR-8: `QdrantSanctumAdapter` in
  `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`, official Qdrant Rust client, host +
  port **6334** + `api_key` + collection + `use_grpc: true`, collection auto-creation, connection
  pooling and retry, feature flag `qdrant`, collection name `paladin_memories_{environment}`,
  1536 dims, Cosine, indexed `paladin_id`/`memory_type`/`created_at`/`importance`,
  < 500 ms top-10 on 100 K vectors.

- **REQ-qdrant-sanctum-adapter-v2** — Epic 12 US-12.1 / FR-1: `QdrantSanctum` in
  `qdrant_sanctum.rs`, URL `http://localhost:6333`, optional API key, `collection_name`,
  `vector_size` 1536, `distance` Cosine, `on_disk: true`, health check verifying the collection,
  error mapping to `SanctumError`, `qdrant-client = "1.7"`.

- Status: **unresolved on naming, connection shape and version — but the feature shipped, so the
  Epic 11-vs-Epic 12 ownership question is moot for forward planning.**
  `intel/code-verification.md` records the adapter as **verified shipped**, contradicting
  `EPIC_11_COMPLETION_SUMMARY.md`'s "Task 5.0: Qdrant Adapter (DEFERRED)". Tree observation:
  `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`, `qdrant-client` **1.14** behind the
  `qdrant` feature, integration target `qdrant_sanctum_integration`. Consult
  `codebase/STACK.md` and `codebase/STRUCTURE.md` for the current shape, not either PRD.

### Group 8 — image format validation ownership *(run 2)*

- **REQ-vision-format-validation-v1** — Epic 13 FR-1.2: the **framework** validates formats and
  accepts only PNG, JPEG, GIF, WebP before dispatch; `VisionError::UnsupportedFormat` and
  `FileTooLarge { size, max }`; CLI reports unsupported formats clearly.

- **REQ-vision-format-validation-v2** — Epic 20 US-20.1 / US-20.2 / NG-3 / NG-5: **adapters
  delegate** validation to the provider API and support every format the provider accepts; no
  conversion, no preprocessing; size limits are the provider's (OpenAI ~20 MB, Anthropic
  model-dependent), documented in `docs/SENTINEL.md`.

- Status: **unresolved, opposite responsibility assignment.** Determines whether an unsupported
  image fails locally with a typed error or as a provider 400, and whether `FileTooLarge` exists
  at all. Tree observation: `VisionError` lives in
  `crates/paladin-core/src/platform/container/vision.rs:189` (the Epic 13 module path).

### Group 9 — OpenAI vision adapter *(run 2)*

- **REQ-openai-vision-adapter-v1** — Epic 13 US-13.2 / FR-2: extend the existing
  `OpenAILlmAdapter`; support `gpt-4-vision-preview`, `gpt-4o`, `gpt-4o-mini`; convert
  `VisionContent` to OpenAI message format for URLs and base64; image token counting.

- **REQ-openai-vision-adapter-v2** — Epic 20 US-20.1 / FR-1: a **dedicated**
  `OpenAIVisionAdapter::analyze_image()` in `openai_vision.rs` POSTing to
  `/v1/chat/completions`; explicit status mapping (400 → `InvalidImage`, 401 →
  `AuthenticationError`, 429/5xx → backoff); configurable `max_retries` (3),
  `initial_backoff_ms`, `backoff_multiplier`; never retry 400/401/403/404; mocked-HTTP unit tests
  plus `ENABLE_VISION_TESTS=true` integration tests.

- Status: **unresolved.** Tree observation: `crates/paladin-llm/src/openai/vision.rs` exists
  alongside `openai/adapter.rs`, i.e. a dedicated vision module *and* a general adapter.

### Group 10 — Anthropic vision adapter *(run 2)*

- **REQ-anthropic-vision-adapter-v1** — Epic 13 US-13.3 / FR-3: extend `AnthropicLlmAdapter`;
  Claude 3 Opus / Sonnet / Haiku; auto-convert URLs to base64; content-block format; rate limiting.

- **REQ-anthropic-vision-adapter-v2** — Epic 20 US-20.2 / FR-2: dedicated
  `AnthropicVisionAdapter::analyze_image()` in `anthropic_vision.rs` POSTing to `/v1/messages`
  with `x-api-key` and `anthropic-version: 2023-06-01`; the same configurable retry contract as
  the OpenAI adapter.

- Status: **unresolved.** Tree observation: `crates/paladin-llm/src/anthropic/vision.rs` exists
  alongside `anthropic/adapter.rs`.

### Group 11 — Paladin vision entry point *(run 2)*

- **REQ-paladin-vision-api-v1** — Epic 13 US-13.4 / FR-5: `Paladin::run_with_vision(task, images)`
  and `PaladinBuilder::enable_vision(bool)`; validate adapter vision support before execution.

- **REQ-paladin-vision-api-v2** — Epic 20 US-20.3 / FR-3:
  `PaladinExecutionService::execute_with_vision(paladin, prompt, images)`; provider derived from
  `paladin.model()` (`gpt-*` → OpenAI, `claude-*` → Anthropic) with
  `VisionError::UnsupportedProvider` otherwise; non-streaming; respects `max_loops`, `stop_words`,
  `timeout_seconds`; Garrison persistence; returns `VisionResult`.

- Status: **both surfaces ship — see VERIFY-04, and `intel/code-verification.md`, which records
  this as coexistence rather than an unresolved contradiction.** Tree observation:
  `PaladinBuilder::enable_vision` at `src/application/services/paladin/paladin_builder.rs:516`
  and `PaladinExecutionService::execute_with_vision` at
  `src/application/services/paladin/paladin_execution_service.rs:517`, both reached from
  `src/application/cli/commands/agent.rs:326,433`. Related, also coexisting rather than
  competing: **REQ-vision-capable-llm-trait** (`vision_llm_port.rs`) and **REQ-vision-port**
  (`vision_port.rs`).

### Group 12 — `VisionError` variant set *(run 2)*

- **REQ-vision-error-model-v1** — Epic 13 FR-12: `UnsupportedFormat`, `FileTooLarge`,
  `InvalidImage`, `ModelNotSupported`, `NetworkError`, `EncryptionError`, `IoError`; plus
  `DocumentError` with `UnsupportedFormat`, `EncryptedPdf`, `CorruptedFile`, `ExtractionFailed`,
  `IoError`.

- **REQ-vision-error-model-v2** — Epic 20 FR-5.1: `InvalidImage`, `UnsupportedFormat`,
  `AuthenticationError`, `RateLimitExceeded(u64)`, `ProviderError`, `NetworkError`,
  `Timeout(u64)`, `UnsupportedProvider`, `MaxRetriesExceeded`; located in
  `container/sentinel/vision_types.rs`; **no** `EncryptionError`, **no** `FileTooLarge`.

- Status: **unresolved, and the divergence is load-bearing** — dropping `EncryptionError` drops
  the only error surface for Epic 13's encryption-at-rest requirement
  (**REQ-vision-security-encryption**), which Epic 20 does not restate. Whether that was a
  conscious decision is part of VERIFY-04. Tree observation: `VisionError` is at
  `crates/paladin-core/src/platform/container/vision.rs:189`, not
  `container/sentinel/vision_types.rs`.

### Group 13 — handoff tool name and parameters *(run 2, 3 names / 2 parameter sets)*

- **REQ-handoff-tool-v1** — Epic 14 FR-5: tool named **`handoff_to_agent`**, required parameters
  **`agent_name`** (enum of available agents) and **`message`**; auto-registered when handoffs are
  configured; validates `agent_name`, executes via `HandoffService`, returns the specialist result
  for synthesis, tracks the chain, errors on circular handoff / invalid name / exceeded depth.

- **REQ-handoff-tool-v2** — Epic 21 FR-3 + Epic 23 Non-Goal 5: auto-registered from
  `PaladinBuilder::build()` detecting a prior `with_handoffs()`; parameters **`specialist_name`**
  (enum of configured specialists) and **`task_description`**; schema carries specialist names,
  descriptions and parameter requirements; registration idempotent and updated when handoffs are
  reconfigured; Epic 23 refers to the tool as **`handoff_to_specialist`**.

- Status: **unresolved on paper.** The tool name and parameter names are part of the JSON schema
  sent to the model, so a mismatch means unroutable tool calls. Tree observation, not a decision:
  `crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs:63` emits
  `"name": "handoff_to_agent"` with `"required": ["agent_name", "message"]` at `:78`.

### Group 14 — Grove routing threshold *(run 2, 3 names / 3 defaults)*

- **REQ-grove-config-v1** — Epic 16 FR-2.3: `GroveConfig { routing_strategy, fallback_tree:
  Option<String>, similarity_threshold }`, defaults `KeywordMatch` and **0.7**, validated on Grove
  creation.

- **REQ-grove-config-v2** — Epic 22 FR-6: `GroveConfig` gains `routing_fallback: String`
  ("keyword" | "error") and `min_confidence: f32` default **0.5**, range 0.0-1.0, with validation
  rejecting invalid fallback values and out-of-range confidence.

- *Third form (context, not a `REQ-*` entry)* — `RELEASE_NOTES_MILESTONE_3.md`:
  `GroveConfig { routing_strategy, confidence_threshold: 0.6 }`.

- Status: **unresolved — either Grove has two thresholds with different semantics (similarity for
  embedding routing, confidence for LLM routing) or one field was renamed twice.** Related
  release-notes claim, **verified absent from the tree**: `RoutingStrategy::PerformanceBased` and
  "dynamic learning" are advertised as shipped but contradict Epic 16 NG-3 and do not exist —
  correcting that is part of VERIFY-03. Shipped routing strategies live in
  `crates/paladin-core/src/platform/container/battalion/grove.rs`.

### Group 15 — `ErrorStrategy` variant sets *(run 2, one type name / two disjoint variant sets)*

- **REQ-battalion-error-strategy** *(run 1, Epic 4 FR-4.3)* — `FailFast`, `ContinueOnError`,
  `RetryThenContinue`. Epic 22 US-22.5 names its Commander tests after exactly these three
  behaviours plus a `continue_on_error: true` config flag.

- **REQ-maneuver-error-strategy-v2** *(run 2, Epic 17 FR-5.1)* — `FailFast`, `ContinueParallel`,
  `IgnoreErrors` for `ManeuverConfig`.

- Status: **unresolved on paper, resolved in the tree as two distinct types.**
  `RELEASE_NOTES_MILESTONE_3.md` documents both sets in different sections of the same document.
  Shipped error aggregation: `crates/paladin-battalion/src/error_aggregation.rs`.

- **Run-3 code verification (highest precedence, not a decision):** these are **two enums in two
  crates**, exactly as each PRD describes —
  `crates/paladin-core/src/platform/container/battalion/mod.rs:240` carries `FailFast` (default),
  `ContinueOnError`, `RetryThenContinue` for Battalion; and
  `crates/paladin-battalion/src/maneuver/mod.rs:18` carries `FailFast` (default),
  `ContinueParallel`, `IgnoreErrors` for Maneuver. Milestone 6 Epic 3 physically separated them
  into different crates, which removes the name collision as a practical concern. Both entries
  stand; they describe different types, not competing definitions of one type.

### Group 16 — `max_loops` scalar vs enum *(run 2 supersession, both preserved)*

- **Run-1 position** — `REQ-paladin-entity` treats `PaladinData.max_loops` as part of a flat field
  set and `REQ-paladin-builder` requires validation in range `[1, 100]`, default 3.

- **REQ-max-loops-auto** *(run 2, Epic 14 FR-1.1)* — `MaxLoops` becomes an enum with
  `MaxLoops::Fixed(u32)` and `MaxLoops::Auto { max_subtasks: u32 }`;
  `PaladinBuilder::max_loops(MaxLoops)` accepts the enum; `Auto` routes execution through
  `PlanningService`; planning loops must not exceed `max_subtasks`.

- Status: **genuine type-level supersession, not a documentation slip.** A range validation defined
  on an integer cannot apply unchanged to an enum, and every Milestone-1 requirement or test
  assuming a numeric `max_loops` is affected. The run-1 criterion is marked superseded, **not
  deleted**. Tree observation: `MaxLoops` is an enum at
  `crates/paladin-core/src/platform/container/paladin.rs:42`.

### Group 17 — Rust edition for the workspace crates *(run 3)*

- **REQ-workspace-crate-edition-v1** — Milestone 5 Epic 1 FR-5 / §7: `edition = "2021"` for
  `paladin-core`, and "All crates in this workspace must use `edition = \"2021\"`. Do not use an
  older edition." Repeated verbatim by Epic 2 FR-2, Epic 3 FR-2 and Epic 4 FR-2 for
  `paladin-ports`, `paladin-battalion` and `paladin-llm`.

- **REQ-workspace-crate-edition-v2** — Milestone 5 Epic 5 FR-1.2: `paladin-memory` declares
  `edition = "2024"` "(matching the workspace root)", corroborated by the milestone overview's
  Appendix D, which sets `[workspace.package] edition = "2024"`.

- Status: **unresolved on paper and genuinely mixed in the tree.** Verified 2026-07-30: the root
  `paladin-ai` package and every crate except two declare `2024`; `crates/paladin-ports` and
  `crates/paladin-notifications` declare `2021`. Edition affects name resolution, `unsafe`
  handling and macro hygiene, and Epic 5 names `paladin-llm` as the canonical template for new
  crates — so a scaffolded crate inherits whichever value its author read. Recording the answer is
  ARCH-03(a); applying it is REL-02.

### Group 18 — `paladin-core` dependency allowlist *(run 3)*

- **REQ-paladin-core-dependency-allowlist-v1** — Milestone 5 Epic 1 FR-6 / Appendix B: exactly
  six crates — `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, `serde_json` — and "No other
  dependencies are permitted"; Appendix B calls itself "the complete and exhaustive list", with
  anything else requiring explicit approval and documented justification.

- **REQ-paladin-core-dependency-allowlist-v2** — Milestone 5 Epic 3 §9 Open Question 4 (the later
  PRD, 2026-05-17 vs 2026-04-21): `petgraph` "is used by both `paladin-core` (Campaign DAG domain
  types) and `paladin-battalion`", requiring workspace version alignment — a seventh dependency
  never approved against Appendix B. `Milestone_4/Epic_1/dependency-matrix.md` independently
  classifies `petgraph` as core.

- Status: **unresolved, and wrong by eight in the tree.** Tree observation, not a decision:
  `paladin-core` ships the six plus `tokio`, `sha2`, `blake3`, `petgraph`, `murmur3`, `url`,
  `regex` and `futures`. The same drift affects `paladin-ports`, whose Epic 2 FR-3 allowlist of 7
  ships as 10 (`+ serde_json`, `futures`, `md5`) — though FR-23's substantive constraint (no
  `redis`, `sqlx`, object-storage SDK or LLM provider SDK) still holds, so the architectural
  invariant is intact and only the allowlist text is stale. The Milestone 5 overview additionally
  contradicts itself, quoting four crates in its headline success criteria and five in Epic 1's
  objective paragraph. Recording the answer is ARCH-03(b).

### Group 19 — ownership of `PaladinResult`, `StopReason` and `TokenUsage` *(run 3)*

- **REQ-port-value-type-ownership-v1** — `Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
  (`Status: Approved`, 2026-05-13, `Chosen Option: Option A`): `PaladinResult` and `StopReason`
  move to `core/platform/container/execution_result.rs`, `TokenUsage` to `token_usage.rs`,
  `RegistryError` to `registry_error.rs`, `HandoffError` to `arsenal/handoff_error.rs`; the four
  application-layer files become thin `pub use` re-exports so every existing
  `paladin::application::ports::output::…` path keeps resolving. `PaladinError` is deliberately
  **excluded** because it carries `#[from] GarrisonError` from the application layer, and the
  convenience `pub use PaladinError` in `herald.rs` is removed.

- **REQ-port-value-type-ownership-v2** — Milestone 5 Epic 2 PRD (2026-05-15) FR-7 lists
  `PaladinResult` / `StopReason` as primary exported types of `paladin-ports`' `paladin_port.rs`
  and `TokenUsage` of `llm_port.rs`; FR-10 rules that "all associated types that are defined
  **within** a port module file … must move with their port trait into `paladin-ports`. Types must
  not be split across crates." FR-11 grants a core-re-export carve-out for `RegistryError` **only**.

- Status: **unresolved, and this is the one place where mechanical precedence gives the wrong
  answer.** The decision record has every structural marker of an ADR but is manifest-typed DOC, so
  a PRD published two days later outranks it. Following FR-10 literally would either duplicate the
  three types in `paladin-ports` or move them back out of `paladin-core`, reintroducing the exact
  upward dependency Epic 1 was written to remove. Tree observation, not a decision: shipped code
  implements **v1** — `paladin-core/src/platform/container/{execution_result,token_usage,registry_error}.rs`
  and `arsenal/handoff_error.rs` all exist. Recording the answer is ARCH-03(c); the record is one
  of the corpus's two strongest ADR-promotion candidates. **Scope note:** this record settles
  *location* for five types only. It never mentions `BattalionResult` — see group 4.

### Group 20 — LLM configuration bridge location *(run 3)*

- **REQ-llm-config-bridge-location-v1** — Milestone 5 Epic 4 FR-31 to FR-33: `paladin-llm` "must
  NOT import from `crate::config::application_settings` or any equivalent path in the root
  `paladin` crate. Doing so would create a circular dependency"; the root crate is "solely
  responsible" for converting `ApplicationSettings.llm.*` into the `paladin-llm` `*Config` structs,
  and that conversion lives at `src/infrastructure/adapters/llm/config_bridge.rs`. A
  `paladin-config` crate is an explicit non-goal.

- **REQ-llm-config-bridge-location-v2** — Milestone 6 Epic 1 §4.1 (the later PRD, 2026-05-23 vs
  2026-05-18): `crates/paladin-llm/src/config/{mod,llm,vision}.rs` are created and exposed as
  `pub mod config;`, holding `LlmProviderConfig`, `LlmConfig`, `VisionRetryConfig`,
  `VisionProviderConfig` and `VisionConfig` — i.e. the config Epic 4 built the bridge to keep out.

- Status: **unresolved on paper; both are PRD precedence so no tiebreaker applies.** Either the
  circular-dependency concern was wrong or the Milestone 6 decomposition breaks the boundary
  Epic 4 established. Tree observation, not a decision: `crates/paladin-llm/src/config/bridge.rs`
  and `crates/paladin-llm/src/config/{llm,vision}.rs` exist — the bridge moved **into**
  `paladin-llm`, which is v2. Recording the answer is ARCH-03(d).

### Group 21 — `paladin-web` web framework *(run 4)*

- **REQ-paladin-web-extraction** — M7 Epic 1 PRD §4.2.1: `crates/paladin-web/Cargo.toml` MUST
  declare **`actix-web` and `axum` as direct (non-optional) dependencies**; the M7 overview Epic 1
  Task 1.2 describes the crate as an "Actix-web application factory, REST API route handlers,
  WebSocket handlers, middleware".

- **REQ-actix-removal** — M8 Epic 7 PRD Goal 1 / FR 5: `actix-web` MUST be removed from
  `crates/paladin-web/Cargo.toml`, FR 8 adds it to `deny.toml`'s banned crates, and Success Metric 1
  requires `rg actix crates/paladin-web/` to return zero matches.

- Status: **the two cannot both hold.** M7's rationale was extracting an existing two-framework
  subsystem; M8's was that the actix handlers were orphaned — "nothing in the workspace ever starts
  an actix `HttpServer`, and `configure()` is never called". Tree observation, not a decision:
  zero `actix` references in `crates/paladin-web/`; facade
  `web-server = ["dep:paladin-web", "dep:axum"]`; the ban lives at `deny.toml:99-103`. Recording the
  answer is HARD-01.

- settled-by: `intel/code-verification.md` run-4.

### Group 22 — `paladin-storage` feature-flag shape *(run 4)*

- **REQ-storage-feature-flags-v1** — M7 Epic 1 PRD §4.5.6 / §7.2 / §9 Q2: facade
  `storage-sqlite = ["dep:paladin-storage", "paladin-storage/sqlite"]`,
  `storage-mysql = [...]`, a `storage = ["storage-sqlite", "storage-mysql"]` convenience alias, and
  `paladin-storage = { workspace = true, optional = true }`.

- **REQ-storage-nonoptional-v2** — the 2026-06-04 reconciliation §4 Category 2 / §7 commit
  `897e77e`: `paladin-storage` becomes **non-optional**, the facade sqlite fallback repositories
  (810 + 676 LOC) are deleted, and the `storage-sqlite` feature is **retired**. ~1,486 LOC removed.

- Status: a downstream consumer following the M7 PRD would look for a flag that no longer exists,
  and the M7 promise that a SQLite-only consumer must not link `libmysqlclient` is now delivered by
  a different mechanism. Tree observation: root `Cargo.toml` declares
  `paladin-storage = { workspace = true, features = ["sqlite"] }` with the inline comment "SQLite
  repositories are always available"; only `storage-mysql` and `storage = ["storage-mysql"]`
  survive.

- settled-by: `intel/code-verification.md` run-4.

### Group 23 — publish dry-run shape *(run 4 — coexistence, not supersession)*

- **REQ-ci-publish-dry-run-v1** — M7 Epic 2 FR-26 / M7 Epic 4 §4.5.6 / overview Appendix B: run
  `cargo publish --dry-run -p <crate>` for the ten crates **in dependency order**
  (`paladin-core` → `paladin-ports` → leaf crates → facade), warning that "violating this order will
  cause `cargo publish --dry-run` to fail".

- **REQ-ci-publish-dry-run-v2** — `.github/workflows/ci.yml:617-644`: a single
  `cargo publish --workspace --dry-run`, with an inline counter-rationale that per-crate dry runs
  "cannot work on a version bump: the not-yet-published new version of each sibling fails the
  `version = \"X\"` requirement of its dependents". This position has **no document carrier** and
  therefore no precedence standing.

- Status: tree observation — **both shipped, in different workflows.** `ci.yml:644` is the
  workspace-wide form; `release.yml:410` runs `cargo publish --dry-run -p "$crate"` per crate inside
  the release job. So this is coexistence rather than one superseding the other, which the ingest
  report could not see from the documents alone. Recording it is HARD-01; SEC-03 depends on the
  answer for *when* a collision is detectable.

### Group 24 — TensorFlow / `ml` adapter disposition *(run 4, three-step chain)*

- **REQ-tensorflow-stays-facade-v1** — M7 Epic 1 PRD §9: `tensorflow_adapter.rs` stays in the
  facade; a `paladin-ml` crate is deferred to M8+.

- **REQ-tensorflow-ml-feature-gate-v2** — M8 Epic 3 PRD §4.3 item 11: the adapter is gated behind a
  new `ml = []` feature.

- **REQ-deferred-tensorflow-ml-adapter-v3** — `deferred-features.md` §2: both the 636-LOC adapter
  and the `ml` flag are **deleted outright** (commit `3d48768`), with a placement condition on
  reintroduction — a dedicated `paladin-ml` leaf crate, never the facade.

- Status: a three-step chain ending in removal, not a two-way disagreement. All three are preserved
  because the v3 *condition* is the live artefact and it is carried only by a DOC. FACADE-03
  records it.

- settled-by: `intel/code-verification.md` run-4 — no `tensorflow` reference and no `ml` feature
  anywhere in `Cargo.toml` or `src/`.

### Group 25 — Milestone 8 Epic 3 scope: defer to M9 versus execute now *(run 4)*

- **REQ-m8-epic3-no-extractions** — M8 Epic 3 PRD §4.3 / §5: "Epic 3 performs **no crate
  extractions**"; every List B move defers to Milestone 9; each adapter group is recorded as an
  active bridge that stays; §5 Non-Goals additionally state "**No new crates created.**
  `paladin-herald`, `paladin-ml`, etc. are not in scope".

- **REQ-m8-reconciliation-relocations** — `facade-cleanup-RECONCILIATION-2026-06-04.md` §3 / §7:
  the deferred relocations are executed inside Milestone 8, `paladin-herald` is created, and both
  the audit and the disposition record are declared to "contain factual errors".

- Status: tree observation — every relocation target exists, `paladin-herald` ships as a
  non-optional facade dependency, and `task-completion-state.md`'s three open Milestone 8 items are
  contradicted by code. HARD-02 records the supersession; the non-goal's `paladin-ml` half still
  holds.

- settled-by: `intel/code-verification.md` run-4.

### Group 26 — `arsenal/` and `sanctum/` Milestone 9 targets *(run 4, internal split)*

- **REQ-adapter-disposition-record (PRD position)** — M8 Epic 3 PRD §4.3 table: `arsenal/` is
  **"M9 extraction candidate? No"** ("MCP wiring is facade composition-root responsibility");
  `sanctum/` is an optional consolidation "→ fold into M9 `paladin-memory` work".

- **REQ-adapter-disposition-record (record position)** —
  `Epic_3/infrastructure-adapter-disposition.md` row 1: `arsenal/` (all 5 adapters) is
  **"Yes (List A) → future `paladin-arsenal` (M9)"**; row 19: `sanctum/` → future
  **`paladin-sanctum`**.

- Status: a single requirement whose two carriers disagree, naming **two crates that do not exist**.
  Neither `paladin-arsenal` nor `paladin-sanctum` is in the tree, and Milestone 9 is recorded 100%
  complete. This lands directly in run-5 scope → FACADE-04.

### Group 27 — notification channel services: move or delete *(run 4)*

- **M8 overview success criteria + Epic 3 Task 3.1** —
  `src/application/notifications/{email_notifications,push_notifications,system_notifications}.rs`
  are **moved** to `crates/paladin-notifications/src/`, "with facade re-exports added if needed for
  backward compatibility", and `cargo test -p paladin-notifications` passes.

- **REQ-dead-file-batch-deletion** — M8 Epic 2 PRD §4.1 Batch 1 **deletes** all three as orphaned
  dead code, on the finding that "`src/application/mod.rs` never declared `pub mod notifications;`,
  making all three files unreachable from the module tree".

- Status: deletion is what shipped — `src/application/` contains only `cli`, `errors`, `mod.rs` and
  `services`. The open question the documents leave behind is narrower: `email_notifications.rs` was
  392 LOC, `facade-audit.md` List A row 1 flagged it "Code has value — review for possible move…
  before deletion", and the Epic 2 PRD's own Open Question 1 asked whether it overlapped the 752-LOC
  `email_notification_adapter.rs`. **Neither the overlap review nor its outcome is recorded
  anywhere.** The file is recoverable from git history.

- settled-by: `intel/code-verification.md` run-4.

### Group 28 — `file_content_repository.rs` disposition *(run 4, three-way)*

- **REQ-paladin-storage-extraction (v1 clause)** — M7 Epic 1 PRD §4.5.2 / §9 Q4: it is **not**
  moved. "Despite its filename, it implements `ContentDeliveryService` / `BatchContentDeliveryService`
  from `paladin-ports` and writes content to the local filesystem; it does not use `sqlx` … It stays
  in the facade crate. A future content-delivery crate (Milestone 8+) is its correct long-term
  home."

- **`facade-audit.md` List B (v2)** — assigns it to **`paladin-storage`** (723 LOC, "File-backed
  content repository").

- **Reconciliation §4 Category 2 / §7 commit `2edc031` (v3)** — **deletes it outright**: "No
  consumers (only its own `mod` line) → delete + drop `pub mod`".

- Status: stays-in-facade, move-to-storage and delete-outright are three futures for one file.
  Deletion shipped. The residue is that the M7 position explicitly reserved a **future
  content-delivery crate** that no later document mentions again — if that idea is still live it
  needs a home in run-5 planning; otherwise the M7 note is retired. HARD-01 records the disposition.

- settled-by: `intel/code-verification.md` run-4.

### Group 29 — token mechanism behind `AuthPort` *(run 5 — neither side settled by code)*

- **REQ-opaque-bearer-token-adapter-v1** — M9 Epic 5 §6.1: "**Chosen:** opaque,
  randomly-generated bearer tokens with a server-side hashed store." Rationale of record: avoids a
  `jsonwebtoken` dependency and a signing-key management story; supports **immediate revocation**,
  which stateless JWTs cannot; trivially deterministic to unit test; the root crate already has
  `rand` and `sha2`, so **no new dependencies**. §5 lists "JWT/OIDC/OAuth or any external identity
  provider integration" as an explicit **non-goal**. Recorded trade-off: "tokens are validated
  against an in-process store, so a **multi-process deployment would later need a shared store**."

- **REQ-jwt-bearer-auth-v2** — M12 Epic 5 FR-2: "The system **must** accept a **JWT** via
  `Authorization: Bearer <token>`, verified through the existing `AuthPort::verify_token`, yielding
  `AuthClaims { user_id, role }`." Config `http.auth.jwt.enabled`;
  `AgentAuthConfig { enabled, api_keys, jwt: Option<Arc<dyn AuthPort>> }`; `paladin-server`
  "constructs an `AuthPort` **JWT verifier** when configured".

- **Status: neither variant wins in the tree, and that is the point.**
  `crates/paladin-web/src/agent_auth.rs` implements v2's **shape** — bearer checked first then
  `x-api-key` (FR-3's documented precedence), constant-time key comparison, the `jwt` field, a
  `MockJwt` test double and a test asserting a key value does not leak. But
  `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returns **nothing**, and the only
  `AuthPort` implementation in the workspace is
  `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — v1's opaque, in-process,
  hashed-token store. M12 Epic 5 **Open Question 4** is unanswered *because it is unanswerable for
  the shipped adapter*: an opaque-token store has no signing secret and no algorithm.

- **Operational consequence, recorded because no document connects the two**: M12 Epic 7 ships
  `k8s/deployment.yaml`. Under more than one replica a token issued by one pod will not verify on
  another. → WEB-01 (the mechanism), WEB-02 (the store).

- settled-by: `intel/code-verification.md` run-5 finding 7 — **records that neither side is
  settled**, which is itself the fact about the tree.

### Group 30 — initial coverage threshold for the CI gate *(run 5)*

- **Parent PRD position (v1)** — `prd-deferred-qa-completion.md` FR-25.3 item 10: "Configure a
  coverage threshold gate of **78%** minimum. PRs dropping below this threshold must fail." G2
  restates it as "threshold gate ≥ 78% overall".

- **Epic 25 position (v2)** — `Epic_25/prd-cicd-pipeline-enhancement.md` FR-25.6 and Appendix C: a
  **phased rollout** — Phase 1 (Sprint 1-2) project **70%**, Phase 2 (Sprint 3-4) **74%**, Phase 3
  (Sprint 5+) **78%**, with patch target 80% throughout. Each phase change is a single `target:`
  edit.

- **Status: the parent PRD's own Open Question 3 asks exactly this and is recorded Open** —
  "Should the coverage threshold gate be a hard fail (block merge) or a soft warning initially?
  Moving from no gate to 78% hard fail could block legitimate PRs during ramp-up." The child Epic
  answered it unilaterally by phasing. Measured coverage is **76-77%** with the deferred modules
  included, so a 78% hard gate would fail on day one while a 70% gate would pass. Neither position
  is in the tree: **`.codecov.yml` does not exist**.

- **This is the sixth position on the coverage gate in this corpus**, after 80% (nine Milestone-1
  PRDs), 85% (unit-test-improvements), 75% layered per tier (the Milestone 3 plan) and 80/70
  re-asserted (Epic 24) — see group 1. → PIPE-02, and it must land on the same number RECON-07 /
  VERIFY-05 record, or record why the CI gate differs.

### Run-3 unsettled positions that are not `-v1`/`-v2` variant pairs

Recorded here so they are not lost, and kept out of the group numbering so the variant arithmetic
stays honest (20 groups / 38 entries).

- **Milestone 6 facade re-export policy** — the milestone overview (DOC) requires backward-compatible
  facade re-exports for the relocated types; `REQ-orchestration-no-reexport-shims` and
  `REQ-circuitbreaker-old-path-retired` (both PRD) forbid them, and Epic 2's own Open Question 4
  undercuts its own Non-Goal. Shipped code follows the PRDs. Decides whether Milestone 6 was a
  non-breaking internal refactor or a breaking change requiring a major version bump → ARCH-04.

- **`vision` feature scope** — `REQ-vision-feature-gating` (PRD) requires `vision` to gate
  `chacha20poly1305` and `zeroize`; `Epic_1/dependency-matrix.md` (DOC, and the Task 1.1 audit
  deliverable of that same epic) classifies both as general-purpose encryption dependencies that
  must stay unconditional. Precedence favours the PRD, but applying it would break
  `cargo build --no-default-features`. Shipped `vision = []` gates nothing, so the audit was right
  → ARCH-05(1).

- **Milestone/tier numbering** — the same class of defect as the run-2 Milestone 3 epic-numbering
  conflict, spanning 9 of the 19 run-3 DOCs → ARCH-02.

- **Build-benchmark verdict** — `build-benchmarks.md` both fails and passes Milestone 5 SM-7
  → ARCH-07.

### Run-4 unsettled positions that are not `-v1`/`-v2` variant pairs

Same treatment as run 3: recorded, and kept out of the group numbering so the variant arithmetic
stays honest (28 groups / 56 entries).

- **The RustSec exception set** — four surfaces, four sets (2 documented / 5 in `.cargo/audit.toml`
  / 15 in `deny.toml` / 2 inline at `ci.yml:406`, with a second bare-`cargo audit` job at
  `ci.yml:77`), and `deny.toml`'s own stated sync invariant violated. Not a `-v1`/`-v2` pair because
  no two documents disagree — the *tree* disagrees with itself → SEC-01.

- **The project's licence posture** — `MIT OR Apache-2.0` (signed checklist, named approver) versus
  MIT (PRD, overview, shipped `Cargo.toml`) → SEC-02.

- **The extracted-crate dependency rule** — stated absolutely in M7 Epic 1 §6.1, anticipated and
  unamended in §4.4 of the same PRD, violated once by `paladin-content → paladin-llm` behind an
  optional feature → HARD-05.

- **The `pdf` capability** — `pdf = []` gates nothing and `content-processing` omits it, while
  `.cargo/audit.toml` suppresses an advisory on the grounds that `pdf-extract` is in the graph
  → HARD-06.

- **The `cargo doc` bar** — zero warnings (M7 Epic 4 §4.4.3, M7 Epic 1 §4.6.4) versus warnings
  acceptable (M8 Epic 5 FR-19), on the same command → HARD-07.

- **Milestone/tier numbering, fourth instance** — the M7 overview titles itself "Milestone 4"
  → HARD-04.

### Superseded but preserved — run-2 later positions on run-1 requirements

Recorded so that a later reader does not mistake supersession for a contradiction. In every case
the run-1 entry is left intact in the ledger and the run-2 entry is the later position.

| Run-1 requirement | Run-2 later position | Nature of the change |
|---|---|---|
| `REQ-paladin-entity`, `REQ-paladin-builder` (`max_loops` scalar, `[1,100]`) | `REQ-max-loops-auto` | Scalar → enum; see variant group 16 |
| `REQ-herald-trait-v1/-v2`, Herald placeholder types | `REQ-herald-type-consolidation` | Placeholders removed, real domain types imported; no shim required |
| `REQ-battalion-result-v1/-v2` | `REQ-battalion-metadata-extension` | Adds `per_paladin_times`, `per_paladin_tokens`, `total_tokens` |
| `REQ-battalion-config-v1/-v2` (`metadata_output_dir`) | `REQ-commander-config-metadata-dir-v3` | Field relocated to `CommanderConfig`; see variant group 3 |
| `REQ-autonomous-configuration` (Epic 14) | `REQ-autonomous-completion-config-schema` (Epic 21) | Consolidated YAML schema, handoff retry/history/eviction settings, `validate_at: planning_time` |
| `REQ-commander-telemetry` (Epic 5, deferred) | `REQ-commander-metadata-export` (Epic 22) | Adds the JSON schema, `<strategy>_<timestamp>_<uuid>.json` naming, non-fatal I/O, < 50 ms overhead |
| `REQ-provider-testing` (Epic 6, live-API suite deferred) | `REQ-provider-live-api-tests` (Epic 24) + post-cleanup reversal | Un-defers the suite, then reverses the skip semantics; see VERIFY-06 |
| `REQ-performance-benchmarking` (Epic 10, suites disabled) | `REQ-battalion-benchmark-repair` (Epic 24) | Repairs Campaign / ChainOfCommand benchmarks and requires `cargo bench --no-run` in CI |

### Superseded but preserved — run-3 later positions on run-1 and run-2 requirements

Milestones 4-6 exist to restructure Milestones 1-3, so this table is longer than run 2's and every
row is intentional. **Relocation is not contradiction.** In every case the earlier entry stays
intact in its ledger.

| Earlier position | Run-3 later position | Nature of the change |
|---|---|---|
| Every `src/{core,application,infrastructure}` path in the run-1 and run-2 corpus | `REQ-cargo-workspace-root`, `REQ-paladin-core-scaffold`, `REQ-paladin-ports-scaffold`, `REQ-battalion-crate-scaffold`, `REQ-llm-crate-scaffold`, `REQ-memory-crate-scaffold` | Single-crate monolith → Cargo workspace. Those paths are historical; resolve current locations through `.planning/codebase/` or the tree |
| `REQ-core-container-extraction` (Maneuver lexer/AST/parser moved *into* `paladin-core`) | `REQ-maneuver-files-moved-from-core`, `REQ-core-maneuver-cleanup` | Moved *out* again into `paladin-battalion` one milestone later. Both moves shipped; the second is current |
| Run-1/run-2 references to `src/application/use_cases/paladin/circuit_breaker.rs` | `REQ-circuitbreaker-relocation`, `REQ-circuitbreaker-old-path-retired` | Relocated to `src/infrastructure/resilience/circuit_breaker.rs`; the old path is **intentionally** unresolvable, with no re-export |
| Run-1/run-2 references to `src/config/application_settings.rs` | `REQ-config-domain-modules`, `REQ-config-incremental-migration` | File deleted; replaced by per-domain config modules split across the facade, `paladin-memory` and `paladin-llm` |
| Run-1/run-2 references to `src/core/platform/manager/{notification_service,queue_service,log_service,orchestrator,listener_service,scheduler}.rs` | `REQ-six-service-relocation`, `REQ-orchestrator-renaming` | Six services relocated and renamed to `*Orchestrator`. Shipped under `src/application/services/`, not the PRD's `src/application/use_cases/` |
| Run-1 `REQ-paladin-port` / `REQ-llm-port-interface` type homes | `REQ-port-value-type-ownership-v1` | `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, `HandoffError` moved to `paladin-core`; ports became thin re-exports. `PaladinError` deliberately excluded. See group 19 |
| Run-1 `REQ-mcp-sse-transport` and the proposed `mcp-arsenal` / `mcp-stdio` / `mcp-sse` flags | `REQ-feature-flag-matrix` (dated 2026-04-15 elimination note) | MCP feature-gating eliminated from scope: Arsenal and its transports compile unconditionally. No MCP feature flag exists in the tree |
| Run-1 default features `["redis-queue", "s3-storage", "openai-embeddings"]` | `REQ-feature-default-set` | Replaced by `default = ["llm-openai"]` — an intentional breaking change with no deprecation cycle |
| Run-1/run-2 benchmark paths (`benches/paladin_benchmarks.rs`, `herald_benchmarks.rs`, `arsenal_benchmarks.rs`) | The M5 per-crate `benches/` layout | Already recorded in QUAL-05; run 3 supplies the requirement behind the move |
| Run-2 `REQ-herald-type-consolidation` (single source of truth for shared domain types) | `REQ-port-value-type-ownership-v1` moved one `TokenUsage`; two more remain | **Not** a completed supersession — the consolidation is still open. Tracked as DEBT-05 |
| `REQ-battalion-facade-shim` (M5 Epic 3: keep a re-export shim so old paths resolve) | `REQ-orchestration-no-reexport-shims`, `REQ-circuitbreaker-old-path-retired` (M6) | The shim posture **flipped** between milestones. Which posture governs Milestone 6 is an open decision → ARCH-04 |

### Superseded but preserved — run-4 later positions on run-1 to run-3 requirements

Run 4 is the first run where a **document supersedes another document by name**:
`facade-cleanup-RECONCILIATION-2026-06-04.md` carries
`Supersedes (corrects): Epic_1/facade-audit.md and Epic_3/infrastructure-adapter-disposition.md`.
Every earlier entry stays intact in its ledger.

| Earlier position | Run-4 later position | Nature of the change |
|---|---|---|
| `REQ-paladin-web-extraction` (actix-web + axum as direct deps) | `REQ-actix-removal`, `REQ-actix-deny-ban` | Framework **removed and banned** one milestone after it was mandated; see variant group 21 |
| `REQ-storage-feature-flags-v1` (`storage-sqlite`, optional `paladin-storage`) | `REQ-storage-nonoptional-v2` | Flag retired, dependency made non-optional; see group 22 |
| `REQ-tensorflow-stays-facade-v1` → `REQ-tensorflow-ml-feature-gate-v2` | `REQ-deferred-tensorflow-ml-adapter-v3` | Stays → gated → deleted, with a `paladin-ml` placement condition on return; see group 24 |
| `REQ-m8-epic3-no-extractions`, `REQ-adapter-disposition-record` | `REQ-m8-reconciliation-relocations` | Relocations deferred to M9 were **executed in M8**; both source documents declared factually wrong; see group 25 → HARD-02 |
| `REQ-paladin-content-extraction` (target dir `use_cases/`) | `REQ-paladin-content-services-rename` | M8 Epic 6 renamed it to `services/`; the facade bridge had been updated and the leaf crate had not, breaking `content-processing` |
| M8 overview Epic 4 Task 4.3 (optional `pub use services as use_cases;`) | `REQ-rename-clean-break` | Explicitly rejected by the Epic 4 PRD: "there will be no `pub use services as use_cases;`" |
| `REQ-paladin-storage-extraction` (`file_content_repository.rs` stays) | reconciliation commit `2edc031` | Deleted outright; the "future content-delivery crate" it reserved is unmentioned since; see group 28 |
| M8 Epic 3 §5 Non-Goal "No new crates created — `paladin-herald`, `paladin-ml` … not in scope" | reconciliation commit `66f6c4e` | **Overridden for `paladin-herald`** inside the same milestone; **still holding for `paladin-ml`** → HARD-02(e), FACADE-03(b) |
| M7 Epic 4 PRD / overview Appendix C target of lockstep `0.2.0` | tag `v0.1.0-rc.1` at `a9530fc`, all ten crates published at `0.1.0` | Superseded by outcome; the tree has since moved to `0.6.0` on `release/v0.7.0` → HARD-03 |
| M7 Epic 4 §4.6 / M8 Epics 2, 4, 5 root-path `STABLE_API.md`; `docs/{PERFORMANCE_BASELINE,RELEASE_CHECKLIST,VERSIONING_POLICY,BUILD_BASELINES,INTEGRATION_TESTS}.md` | mdbook pages under `docs/src/{api-reference,appendix}/` | Same Milestone 11 relocation ARCH-05 already records for the run-3 deliverables. **Relocated, not missing** — do not plan them |
| `REQ-ci-publish-dry-run-v1` (per-crate, dependency-ordered) | `REQ-ci-publish-dry-run-v2` (workspace-wide) | **Not a supersession** — both ship, in different workflows; see group 23 |

### Run-5 unsettled positions that are not `-v1`/`-v2` variant pairs

Same treatment as runs 3 and 4: recorded, and kept out of the group numbering so the variant
arithmetic stays honest (30 groups / 60 entries). Fourteen of run 5's sixteen warnings land here.

- **The agent route surface** — M12 Epics 1, 3, 4 and 5 write against unprefixed `/agents/...`
  paths; Epic 6 §4.3 serves the agent API under `/v1`. Four Epics' text names paths a fifth Epic
  relocates, and `crates/paladin-web/openapi.json` locks in whichever shipped. Not a `-v1`/`-v2`
  pair because it is one milestone disagreeing with itself across five documents → ORCH-03(a).

- **The `AgentProvisioner` placement** — Epic 1 OQ-2 records a *default* (`paladin-web`), not a
  decision, while two shipped deployment-topology pages describe consumers that would need it
  → ORCH-04(a).

- **Garrison and Arsenal for HTTP-served agents** — a capability asymmetry between the
  embedded-library and HTTP-service-host topologies, stated once in an Epic 2 non-goal, against a
  decision matrix M11 Epic 6 FR-8 makes "the single source of routing" → ORCH-04(b).

- **The duplicate `cargo audit` CI job** — not a `-v1`/`-v2` pair because no two documents
  disagree; the *tree* disagrees with itself, and a milestone recorded 100% complete fails its own
  §8 success metric → SUPPLY-01.

- **The advisory owner/expiry gap and the three unauthorised vulnerability ignores** — M10 Epic 2
  FR-3's schema does not require an owner or an expiry, and FR-3/§5 authorise exactly two
  advisories against the five in the tree → SUPPLY-02.

- **Deferred-QA Epic 25** — verified unimplemented item by item; the largest concrete unbuilt scope
  in the corpus → PIPE-01 … PIPE-05.

- **The architecture document's status** — archive material or live deliverable, frozen at 311
  lines inside the one chapter exempt from rewriting → DOCS-02.

- **Deferred-QA Epic 27 and `ProviderCapabilities`** — the capability flag over-reports today,
  independent of whether tool calling is ever built → WEB-03 (the flag), WEB-04 (the scope).

- **The Epic 28/29 mock prerequisite** — `tests/common/` does not exist and the five named mocks do
  not exist; placement and `mockall`-versus-hand-written are both unanswered → DEFER-01.

- **`user_service.rs` — test it or split it** — two registers, two incompatible next actions on one
  file → DEFER-02, sequenced against FACADE-02.

- **Epic 29's stale coverage baseline** — 57.83% dated 2026-02-14, predating M9 Epic 2's tests on
  the same module, at a path that no longer exists → DEFER-03.

- **`project/current-exports.txt`** — nine stale references across two scripts, three workflow
  lines and five requirement texts; unchanged across three ingest runs → DEBT-01 (extended in
  place, not duplicated).

- **Milestone 11's 26 open items** — the only genuinely open checkbox count in the corpus, and not
  settleable by file existence → DOCS-01.

- **The fifth milestone-numbering collision that did not happen** — predicted after four
  instances; run 5 found none. Recorded so the prediction is closed rather than left standing
  → ORCH-05.

### Superseded but preserved — run-5 later positions on runs 1-4 requirements

Run 5 is the last run, so this table closes the supersession record. Every earlier entry stays
intact in its ledger. **Relocation is not contradiction**, and neither is a later milestone
completing what an earlier one scoped.

| Earlier position | Run-5 later position | Nature of the change |
|---|---|---|
| `REQ-commander-telemetry` / `REQ-orchestrator-*` `println!`-only workflow arms (M1 Epic 5, M6 Epic 2) | `REQ-workflow-execution-loop`, `REQ-taskservice-dispatch` | The four `println!` arms in `create_workflow()` become a real `execute_workflow()` covering all four `WorkflowExecutionOrder` variants |
| `REQ-m8-deferred-items-register` **D2** — split `user_service.rs` | `REQ-user-service-test-coverage` — test `user_service.rs` to ≥ 80% | **Not a supersession; a collision.** Two registers, two incompatible next actions on one file → DEFER-02 |
| Run-1 `REQ-provider-testing` / run-2 `REQ-provider-live-api-tests` capability claims | `REQ-llm-tool-calling-adapters` problem statement | Establishes that `ProviderCapabilities` has been **over-reporting tool-calling support** since the adapters were written → WEB-03 |
| Run-3/run-4 references to `src/core/platform/manager/listener_service.rs` | `src/application/services/orchestration/listener.rs` (`ListenerOrchestrator`) | Relocated by M6 Epic 2; `REQ-listener-service-test-coverage` still names the old path → ORCH-03(b) |
| Run-1 `REQ-llm-port-interface` path `src/application/ports/output/llm_port.rs` | `crates/paladin-ports/src/output/llm_port.rs` | `src/application/ports/` fully deleted by M5 Epic 2; `REQ-llm-tool-calling-port` still names the old path → ORCH-03(c) |
| `docs/Design/Design_and_Architecture.md` (Deferred-QA FR-26.1) | `docs/src/appendix/design-and-architecture.md` (M11 Epic 2) | Relocated **without being rewritten**, into the one chapter M11 Epic 3's non-goals exempt → DOCS-02 |
| M1 Epic 10 / Deferred-QA FR-26.4 README demo embedding | `REQ-readme-landing-page` (M11 Epic 5) | The README became a concise landing page with no demos section; the embedding clause targets a document that changed shape → DOCS-04 |
| `REQ-web-api-baseline-changelog` (M8 Epic 7 FR-10) as the sole requirement-level `project/current-exports.txt` reference | M12 Epic 1 §7, Epic 5 §7, Epic 6 `cross_refs`, Epic 7 FR-4.6 | Four more requirements adopt the stale path — the newest documents in the corpus propagating a defect from commit `928c6d5` → DEBT-01 |
| Run-4 finding "`deny.toml` mirrors only the original two advisories; the three 2026 advisories are absent; the sync invariant is violated" | Run-5 direct read: **five vulnerability advisories in both files, matching exactly** | **A correction, not a supersession.** The run-4 claim is withdrawn. The real gap is owner/expiry coverage on 13 of 15 → SUPPLY-02 |
| `REQ-rustsec-risk-acceptance` (2 advisories, owner, expiry) as the whole governance story | `REQ-advisory-exception-process` (M10 Epic 2 FR-3, four-field schema, no owner, no expiry) | The origin policy is **weaker than the run-4 remediation plan**, which is why 13 entries are compliant with their policy and still ungoverned → SUPPLY-02 |
| M7 Epic 4 `v0.1.0-rc.1` → tree at 0.6.0 with the intermediate versions unexplained (HARD-03) | `REQ-m9-quality-gate-v030`, `REQ-m10-v040-release`, `REQ-m11-v050-release`, `REQ-m12-v060-release` | The gap is closed: v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0, one lockstep bump per milestone → ORCH-05 |
| `REQ-master-plan-epics-11-18` (project-management, dated 2026-01-29) | The entire run-2 Milestone 2 requirement set | **Provenance, not new scope.** This is the *origin* document for Epics 11-18; every one was ingested in run 2 and most are verified shipped. Do not double-count |

---

## Milestone 1 as-shipped ledger

Per-requirement verdicts for Milestone 1 now live in
[`.planning/ledgers/milestone-01.md`](ledgers/milestone-01.md) rather than inline here (D-17).
REQUIREMENTS.md is already ~4,000 lines and holds five as-shipped ledger sections; five sets of
`file:line`-cited verdicts inline would make it unreadable. That ledger carries 113 `REQ-*` rows
and 39 nested outstanding task items, each with a `file:line` citation and, where the verdict is
`satisfied`, a named passing test, example, or command that exercises it. Phases 5, 7, 10 and 13
add sibling ledger files (`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`,
`milestone-09-12.md`) in the same directory.

---

## Milestone 2-3 as-shipped ledger

Per-requirement verdicts for Milestones 2 and 3 now live in
[`.planning/ledgers/milestone-02-03.md`](ledgers/milestone-02-03.md) rather than inline here (D-21).
REQUIREMENTS.md is already ~4,000 lines and holds five as-shipped ledger sections; five sets of
`file:line`-cited verdicts inline would make it unreadable. That ledger carries **118** `REQ-*`
rows and **2** nested outstanding-task items, each with a `file:line` citation and, where the
verdict is `satisfied`, a named passing test, example, or command that exercises it.
Phases 7, 10 and 13 add the remaining sibling ledger files (`milestone-04-06.md`,
`milestone-07-08.md`, `milestone-09-12.md`) in the same directory.

---

## Milestone 4-6 as-shipped ledger

Per-requirement verdicts for Milestones 4, 5 and 6 now live in
[`.planning/ledgers/milestone-04-06.md`](ledgers/milestone-04-06.md) rather than inline here (D-26).
REQUIREMENTS.md is already ~4,000 lines and holds five as-shipped ledger sections; five sets of
`file:line`-cited verdicts inline would make it unreadable. That ledger carries **115** `REQ-*`
rows, each with a `file:line` citation and, where the verdict is `satisfied`, a named passing test,
example, command or CI job that exercises it. The ledger records the real workspace shape of ten
library crates plus `doc-examples` plus the root facade package `paladin-ai` — replacing both the
six crates the Milestone 5/6 overviews assume and the nine-crate figure this planning set carried
before run 3. Nested outstanding-item count: **0** — no row uses the blank-first-two-column
nested-row format, per the ledger's own `## Summary` section. Phases 10 and 13 add the remaining
sibling ledger files (`milestone-07-08.md`, `milestone-09-12.md`) in the same directory.

---

## Milestone 7-8 as-shipped ledger

Per-requirement verdicts for Milestones 7 and 8 now live in
[`.planning/ledgers/milestone-07-08.md`](ledgers/milestone-07-08.md) rather than inline here (D-01,
Phase 10 plan 10-01, dated 2026-08-08). REQUIREMENTS.md is already ~4,000 lines and holds five
as-shipped ledger sections; five sets of `file:line`-cited verdicts inline would make it unreadable.
That ledger carries the full **86**-row `file:line`-cited verdict table across twelve epic sections,
a seven-class evidence bar, the corrected workspace-shape provenance, and a dedicated "Superseded by
outcome" summary table listing the thirteen requirements that must not be planned as written. The
`Amended by Phase 9 (plan 09-07), dated 2026-08-08` notes previously carried inside this section on
the seven Phase-9-closed rows are carried forward into the ledger's own rows and are not lost by this
reduction. This section is retained as a pointer only. Phase 13 adds the remaining sibling ledger
file (`milestone-09-12.md`) in the same directory.

Retained for a reader arriving here by line number: this section's original status key (superseded
by the ledger's own seven-class vocabulary, above) read — "Status key (extends the run-3 key):
`Shipped` · `Shipped (relocated)` · `Shipped, superseded` · `Superseded by outcome` = do not plan as
written · `Deferred with register` = removed on purpose, condition recorded · `Verify` → HARD-01 ·
`Variant` · `Code diverges` · `Open defect → X`."

---

## Milestone 9-12 as-shipped ledger

Per-requirement verdicts for Milestones 9-12 plus the `Deferred-QA-CICD-Completion` and
`project-management` registers now live in
[`.planning/ledgers/milestone-09-12.md`](ledgers/milestone-09-12.md) rather than inline here (D-01,
Phase 13 plan 13-10, dated 2026-08-10). REQUIREMENTS.md is already ~4,200 lines and holds five
as-shipped ledger sections; five sets of `file:line`-cited verdicts inline would make it unreadable.
The ledger is **authoritative** for Milestone 9-12: it carries the full **120**-row `file:line`-cited
verdict table across **28** epic/register sections, an eleven-class evidence bar extending the run-4
seven-class vocabulary, the corrected 35/53/32 arithmetic (ORCH-01, D-04), the corrected fifteen-job
`ci.yml` list (PIPE-01, D-08), the corrected `0.7.0`/`v0.7.1` current-state figures (ORCH-05, D-18),
and a per-milestone checkbox corroboration paragraph (ORCH-02). Preconditions asserted before this
section's body was removed: `grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` → `120`;
`grep -c 'pending — plan' .planning/ledgers/milestone-09-12.md` → `0`; the ID list in the ledger
diffs clean against the ID list this section held. This section is retained as a pointer only,
exactly as Phase 7's D-26 did for Milestone 4-6 and Phase 10's D-01 did for Milestone 7-8. **This is
the fifth and final sibling** — `milestone-01.md`, `milestone-02-03.md`, `milestone-04-06.md` and
`milestone-07-08.md` complete the series; there is no sixth name to forward.

---

## Out of Scope

| Feature | Reason |
|---|---|
| Re-implementing shipped Milestone-1/2/3 work | The ledgers record it; the roadmap does not re-plan it |
| Re-planning the Milestone 5 workspace decomposition or the Milestone 6 relocations | Verified shipped against the tree in `intel/code-verification.md`. Ten library crates exist; `application_settings.rs` is deleted; the orchestrators, the Maneuver DSL and `CircuitBreaker` are all at their new homes |
| Extracting a `paladin-cli` crate, or adding MCP transport feature flags | Both are documented positions the shipped code contradicts, and in both cases the later PRD agreed with the code |
| Picking a winner for the **30** competing variant groups (69 warnings) inside this document | Deliberate, and explicitly requested across all five runs: shipped code is the arbiter and the decision belongs in an ADR (RECON-02 … RECON-07, VERIFY-03 … VERIFY-06, ARCH-03, ARCH-04, SEC-01, SEC-02, HARD-01 … HARD-07, WEB-01, PIPE-02), not in an ingest artefact. Where shipped code settles a variant, that is recorded as a fact about the tree at the top of the precedence order, not as a decision taken here. **Group 29 is the one variant shipped code cannot settle** — the tree carries the M12 shape and the M9 mechanism simultaneously |
| Synthesizing locked decisions from PRD/DOC assertions | **0 ADR-typed and 0 SPEC-typed docs exist across all 199 classified documents** — the final corpus position. Asserting locks would fabricate authority. Eleven ADR candidates are named in context; promoting any requires re-tagging the source via `--manifest` and re-running ingest, not an edit here |
| Converting open checkbox counts into requirements | 542 open items, and the two largest blocks are verified shipped. Verification first (RECON-01, VERIFY-02) |
| Migrating between the two shipped vision surfaces | Both ship; `code-verification.md` explicitly says confirm intent first (VERIFY-04) |
| Live LLM API calls in the default test run | Cost, flakiness, secret handling; feature-gated only |
| JSON or TOML CLI configuration | Epic 9 NG-4 — YAML only |
| Encrypted config files, keychain/secret-manager integration | Epic 9 NG-2, NG-10 — env vars only |
| Grove learning from past routing decisions | Epic 16 NG-3; the release-notes `PerformanceBased` claim is verified absent from the tree |
| Automatic Garrison-to-Sanctum migration | Epic 11 explicit non-goal |
| Batch vision API | Epic 20 NG-6 — concurrency is a Battalion concern |
| Registry multi-tenancy, persistence, distribution | Epic 22 explicit non-goals |
| Non-Rust client SDKs | The product *is* a Rust library surface |
| Re-planning shipped Milestone 7-8 work | The four crate extractions, the build/CI/benchmark infrastructure, the release-candidate cycle, the facade deletions, the rename, the actix consolidation and the reconciliation's fifteen commits are all verified in the tree |
| Implementing the ~~14~~ **Corrected (dated 2026-08-08, HARD-01):** 13 requirements marked *Superseded by outcome* — see `.planning/ledgers/milestone-07-08.md`'s summary table | Shipped code went a different way, deliberately. Recording that is HARD-01; implementing them would undo it |
| Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml` | None exists. The first two are named only by a superseded disposition record that contradicts its own PRD (FACADE-04); the third is a placement *condition* on reintroducing a removed feature (FACADE-03b), not a deliverable |
| Rebuilding `docs/{PERFORMANCE_BASELINE,RELEASE_CHECKLIST,VERSIONING_POLICY,BUILD_BASELINES,INTEGRATION_TESTS}.md` or a root `STABLE_API.md` | Relocated into the mdbook by the Milestone 11 overhaul, not missing — same finding ARCH-05 records for the run-3 deliverables |
| Treating any `v0.1.0-rc.1` artefact as current | It is history. The published-crate list, docs.rs verification and GO sign-off all describe the 0.1.0 state; the tree is at 0.6.0 → HARD-03 |
| Re-planning shipped Milestone 9-12 work | The entire Milestone 9 orchestrator subsystem (`execute_workflow()`, the `WorkflowRepository` port and its SQLite adapter, the content processors, the orchestrator bridge, `AuthPort` and RBAC), the entire Milestone 10 tooling set (pre-commit, cargo-audit + cargo-deny + OSV-Scanner, CycloneDX SBOM, cargo-release, the tag-source guard and committed rulesets), the mdbook with linkcheck at `warning-policy = "error"` and all six deployment-topology pages, and the entire Milestone 12 web API (registry, controller, auth, error envelope, health/ready, layers, job store, OpenAPI with a committed drift baseline, `paladin-server`, `Dockerfile.server`, `k8s/`) are **verified shipped against the tree**. Phase 13 records them; no phase rebuilds them |
| Treating Milestone 12's three open checkboxes, or project-management's one, as work | M12's three are Task 0.0 feature-branch scaffolding while the Epic 5 code ships; project-management's one is a `- [ ] 1.1 Create template` formatting example inside a template file. Neither represents anything |
| Re-ingesting `REQ-master-plan-epics-11-18` as new scope | It is the **origin** document for Epics 11-18 (dated 2026-01-29), every one of which was ingested in run 2 and most of which are verified shipped. Its value is provenance — the dependency graph and the epic-level risk assessment — not scope |
| Hot-reloading `config.yml` in `paladin-server` | M12 Epic 2 explicit non-goal — config is read once at startup |
| Terminating TLS in `paladin-server` | M12 Epic 2 and Epic 7 non-goal — the server binds plain HTTP; TLS is a proxy/ingress concern |
| Fine-grained scopes or permissions beyond `allowed_roles` plus the admin gate | M12 Epic 5 explicit non-goal |
| Encrypting configuration at rest | M12 Epic 5 explicit non-goal — "secrets management is the operator's responsibility (as with LLM keys)". API-key values should come from env/secret indirection, not committed config |
| Rewriting the 35 mdbook appendix files | M11 Epic 3 explicit non-goal — reference/archive material. **One exception is under decision**: `design-and-architecture.md`, whose relocation into that exempt chapter is exactly why the gap survived → DOCS-02 |
| Benchmark **regression detection** (`critcmp`, `github-action-benchmark`) | Deferred-QA Epic 25 explicit non-goal, listed as a future enhancement. Note the inversion: it already ships as `benchmark-regression-signal` at `ci.yml:531` from M7 Epic 3, while the `bench-check` compile prerequisite does not → PIPE-01 |
| Publishing images or cutting a crates.io release as part of the Web API work | M12 Epic 7 explicit non-goal — "artifacts/docs/tests/release only; no behavior changes to the API". Release automation is Milestone 10's, and it shipped |

---

## Traceability

Forward (v1) requirements only. Shipped requirements are tracked in the two ledgers above.

| Requirement | Phase | Status |
|-------------|-------|--------|
| RECON-01 | Phase 1 | Complete |
| RECON-02 | Phase 1 | Complete |
| RECON-03 | Phase 1 | Complete |
| RECON-04 | Phase 1 | Complete |
| RECON-05 | Phase 1 | Complete |
| RECON-06 | Phase 1 | Complete |
| RECON-07 | Phase 1 | Complete |
| RECON-08 | Phase 1 | Complete |
| GAP-01 | Phase 2 | Complete |
| GAP-02 | Phase 2 | Complete |
| GAP-03 | Phase 2 | Complete |
| GAP-04 | Phase 2 | Complete |
| GAP-05 | Phase 2 | Complete |
| GAP-06 | Phase 2 | Complete |
| GAP-07 | Phase 2 | Complete |
| QUAL-01 | Phase 3 | Complete |
| QUAL-02 | Phase 3 | Complete |
| QUAL-03 | Phase 3 | Complete |
| QUAL-04 | Phase 3 | Complete |
| QUAL-05 | Phase 3 | Complete |
| REL-01 | Phase 4 | Complete |
| REL-02 | Phase 4 | Complete |
| REL-03 | Phase 4 | Complete |
| REL-04 | Phase 4 | Complete |
| REL-05 | Phase 4 | Complete |
| VERIFY-01 | Phase 5 | Complete |
| VERIFY-02 | Phase 5 | Complete |
| VERIFY-03 | Phase 5 | Complete |
| VERIFY-04 | Phase 5 | Complete |
| VERIFY-05 | Phase 5 | Complete |
| VERIFY-06 | Phase 5 | Complete |
| CLOSE-01 | Phase 6 | Complete |
| CLOSE-02 | Phase 6 | Complete |
| CLOSE-03 | Phase 6 | Complete |
| ARCH-01 | Phase 7 | Complete |
| ARCH-02 | Phase 7 | Complete |
| ARCH-03 | Phase 7 | Complete |
| ARCH-04 | Phase 7 | Complete |
| ARCH-05 | Phase 7 | Complete |
| ARCH-06 | Phase 7 | Complete |
| ARCH-07 | Phase 7 | Complete |
| DEBT-01 | Phase 8 | Complete |
| DEBT-02 | Phase 8 | Complete |
| DEBT-03 | Phase 8 | Complete |
| DEBT-04 | Phase 8 | Complete |
| DEBT-05 | Phase 8 | Complete |
| SEC-01 | Phase 9 | Complete |
| SEC-02 | Phase 9 | Complete |
| SEC-03 | Phase 9 | Complete |
| SEC-04 | Phase 9 | Complete |
| SEC-05 | Phase 9 | Complete |
| HARD-01 | Phase 10 | Complete |
| HARD-02 | Phase 10 | Complete |
| HARD-03 | Phase 10 | Complete |
| HARD-04 | Phase 10 | Complete |
| HARD-05 | Phase 10 | Complete |
| HARD-06 | Phase 10 | Complete |
| HARD-07 | Phase 10 | Complete |
| FACADE-01 | Phase 11 | Complete |
| FACADE-02 | Phase 11 | Complete |
| FACADE-03 | Phase 11 | Complete |
| FACADE-04 | Phase 11 | Complete |
| SUPPLY-01 | Phase 12 | Complete |
| SUPPLY-02 | Phase 12 | Complete |
| SUPPLY-03 | Phase 12 | Complete |
| ORCH-01 | Phase 13 | Complete |
| ORCH-02 | Phase 13 | Complete |
| ORCH-03 | Phase 13 | Complete |
| ORCH-04 | Phase 13 | Complete |
| ORCH-05 | Phase 13 | Complete |
| WEB-01 | Phase 14 | Pending |
| WEB-02 | Phase 14 | Pending |
| WEB-03 | Phase 14 | Pending |
| WEB-04 | Phase 14 | Pending |
| PIPE-01 | Phase 15 | Pending |
| PIPE-02 | Phase 15 | Pending |
| PIPE-03 | Phase 15 | Pending |
| PIPE-04 | Phase 15 | Pending |
| PIPE-05 | Phase 15 | Pending |
| DEFER-01 | Phase 15 | Pending |
| DEFER-02 | Phase 15 | Pending |
| DEFER-03 | Phase 15 | Pending |
| DOCS-01 | Phase 16 | Pending |
| DOCS-02 | Phase 16 | Pending |
| DOCS-03 | Phase 16 | Pending |
| DOCS-04 | Phase 16 | Pending |

**Coverage:**

- v1 requirements: **86 total** (25 Milestone-1 close-out + 9 Milestone 2-3 close-out +
  12 Milestone 4-6 close-out + 16 Milestone 7-8 close-out + 24 Milestone 9-12 + Deferred-QA
  close-out)

- Mapped to phases: 86
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓

**No run-4 requirement duplicates an existing one.** Three earlier requirements were **extended in
place** rather than duplicated, per the Roadmap Extension Protocol: ARCH-01 (pending crate
provenance now supplied), DEBT-01 (a sixth stale reference, this one inside a requirement) and
DEBT-03 (the documentation gate it sits under, now named). Those edits are recorded at the point of
each requirement.

**No run-5 requirement duplicates an existing one either.** Two earlier requirements were
**extended or corrected in place** rather than duplicated:

- **DEBT-01** gained the four Milestone 12 `project/current-exports.txt` references, taking the
  count from six to **nine**, and **shed** the four deprecated `actions-rs/toolchain@v1` references
  it had absorbed in run 3 — those move to PIPE-04, which owns the full eight-reference
  modernization sweep. DEBT-01 keeps the baseline path; PIPE-04 keeps the action versions.

- **SEC-01** was **corrected**, not extended. Run 4 recorded `deny.toml` as out of sync with
  `.cargo/audit.toml`; run 5 read both files and found the five vulnerability advisories match
  exactly. The "out of sync" framing is withdrawn at the requirement, and the corrected scope —
  owner/expiry coverage on 13 of 15 entries, plus three vulnerability ignores no document
  authorises — is carried by SUPPLY-02. The concrete `ci.yml:389-406` deletion SEC-01's fourth
  surface describes is carried by SUPPLY-01.
  (**Corrected by Phase 12 (plan 12-01), dated 2026-08-09, citing `ci.yml:465-482` and commit `cb75b2b`:**
  the deletion cited above was already stale at `:389-406` — the true location was `ci.yml:465-482`,
  deleted by Phase 9's plan 09-06 in commit `cb75b2b`. SUPPLY-01 is now closed; see its "Verified by
  Phase 12" block.)

**Two run-5 requirements deliberately record rather than act.** SUPPLY-03 (promote or decline the
two supply-chain ADR candidates) and the recording halves of ORCH-01 … ORCH-05 produce decisions and
ledger entries, not code. That is the same treatment runs 1-4 gave RECON-*, VERIFY-*, ARCH-01 …
ARCH-07 and HARD-01 … HARD-07, and it is why the roadmap has more record-keeping phases than build
phases: **the corpus documents twelve shipped milestones, so most of the work is making the record
match the code rather than changing the code.**

**Cross-phase couplings** (not duplication — one requirement records the answer, another applies
it; recorded here so neither is planned twice):

| Recording requirement | Applying requirement | Subject |
|---|---|---|
| ARCH-03(a) (Phase 7) | REL-02 (Phase 4) | One Rust edition across the workspace |
| ARCH-04 (Phase 7) | REL-01 (Phase 4) | Whether Milestone 6 forces a major version bump |
| ARCH-03(c) (Phase 7) | DEBT-05 (Phase 8) | Which crate owns the canonical `TokenUsage` — **discharged (2026-08-06):** DEBT-05 collapsed the two duplicate definitions into `pub use` re-exports of the `paladin-core` canonical type; `grep -rn 'pub struct TokenUsage' crates src \| wc -l` → `1` |
| RECON-07 (Phase 1) → VERIFY-05 (Phase 5) | QUAL-01 / QUAL-03 (Phase 3) | The coverage gate |
| HARD-06 (Phase 10) | SEC-01 (Phase 9) | Whether `pdf-extract` is reachable — decides if the `RUSTSEC-2026-0187` suppression is needed at all — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** discharged in the SEC-01 direction on tree evidence — `crates/paladin-content/Cargo.toml:41` declares `pdf-extract` unconditionally (not `optional = true`) while `:18`'s `pdf = []` feature gates nothing, so `lopdf` is reachable in the graph whenever `paladin-content` builds regardless of how HARD-06 answers the capability question. SEC-01 ratifies `RUSTSEC-2026-0187` on this evidence (ADR-0024 decision 3) without waiting for Phase 10. This phase does **not** answer whether PDF extraction is a supported capability — that contradiction (a mandatory dependency behind an empty feature flag) remains HARD-06's subject, handed over as a `file:line` finding, not resolved |
| HARD-03 (Phase 10) | REL-01 (Phase 4) | The version trajectory; REL-01 must not converge on an rc.1 figure |
| HARD-05 (Phase 10) | FACADE-02 (Phase 11) | Whether leaf-to-leaf crate edges are permitted, which decides D2/D3/D4's relocation targets |
| HARD-07 (Phase 10) | DEBT-03 (Phase 8) | Which `cargo doc` bar governs, and therefore what re-enabling `paladin-ports` doctests must satisfy — **live coupling, unresolved by DEBT-03 (2026-08-06):** DEBT-03 re-enabled the doctests (`ci.yml:226` no longer excludes `paladin-ports`) but deliberately declined to decide the warning bar (D-12); Phase 10 / HARD-07 still owns that question and inherits the measured 6-warning `cargo doc --workspace --no-deps` state |
| ARCH-04 (Phase 7) | FACADE-02 (Phase 11) | Whether a no-re-export-alias policy is adopted, which decides D1 |
| SEC-01 (Phase 9) | SUPPLY-01 / SUPPLY-02 (Phase 12) | The RustSec exception set. SEC-01 owns the whole set and the 2026-09-30 disposition; SUPPLY-01 makes the CI deletion and SUPPLY-02 carries the corrected governance scope. **Phase 12 should not wait for Phase 9** — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** the coupling is discharged, not merely scheduled — Phase 9 ran first (plans 09-02 and 09-06) and executed both SUPPLY-01's CI deletion and all three of SUPPLY-02's clauses itself, per D-07 (`09-CONTEXT.md`). Both requirements carry a "Closed by Phase 9" note with commit references at their own definitions above. Phase 12 inherits SUPPLY-01 and SUPPLY-02 as closed items to verify (a CI-only confirmation that the required status check still resolves, and that `cargo audit`/`cargo deny check` pass against the reconciled configuration); what remains open for Phase 12 to actually plan is **SUPPLY-03** alone |
| HARD-06 (Phase 10) | SUPPLY-02 (Phase 12) | Whether `pdf-extract` is reachable, which decides whether `RUSTSEC-2026-0187` needs suppressing at all |
| HARD-07 (Phase 10) | DOCS-03 (Phase 16) | Which `cargo doc` bar governs; DOCS-03 applies it and adds the CI gate |
| DEBT-03 (Phase 8) | DOCS-03 (Phase 16) | Re-enabling `paladin-ports` doctests is what makes the port traits' rustdoc examples executable rather than merely present — **input now ready (2026-08-06):** 96/96 doctests pass at both crate and workspace scope; Phase 16 / DOCS-03 inherits executable examples as input for its documentation-quality work, including the 87 pre-existing `ignore`/`no_run`/`text` fences DEBT-03 deliberately left un-audited (D-10) |
| HARD-03 (Phase 10) | ORCH-05 (Phase 13) | The version trajectory. HARD-03 records `v0.1.0-rc.1` as history; ORCH-05 completes the chain v0.3.0 → v0.6.0, and REL-01 (Phase 4) converges on the result |
| FACADE-02 (Phase 11) — D2 | DEFER-02 (Phase 15) | **Split or test `user_service.rs`.** Two registers, two incompatible next actions on one file. Splitting first is cheaper but changes Epic 28's estimate and mock set. Do not schedule independently |
| RECON-07 (Phase 1) → VERIFY-05 (Phase 5) | PIPE-02 (Phase 15) | The coverage gate, sixth position. The CI threshold must land on the recorded number or record why it differs |
| ORCH-04(a) (Phase 13) | — | `AgentProvisioner` placement; it constrains the queue/worker and sidecar topologies and is cheap now, expensive after a second consumer exists |
| WEB-01 (Phase 14) | WEB-02 (Phase 14) | The token mechanism decides whether the shared-store question is about swapping an adapter or replacing a scheme |

**Ledger coverage:**

- Requirement IDs enumerated from `intel/requirements.md`: **554** (run 1: 115, run 2: 118,
  run 3: 115, run 4: 86, run 5: 120)

- Recorded in the ledgers: **554 ✓** — 115 in the Milestone 1 ledger, 118 in the Milestone 2-3
  ledger, 115 in the Milestone 4-6 ledger, 86 in the Milestone 7-8 ledger, 120 in the
  Milestone 9-12 ledger. **Enumerated and cross-checked against `intel/requirements.md`: 120 rows,
  120 distinct IDs, zero missing and zero extra.**

- Competing-variant entries preserved unmerged: **60 across 30 groups** (12 from run 1, 18 from
  run 2, 8 from run 3, 18 from run 4, 4 from run 5), against **69 cumulative warnings** in
  `INGEST-CONFLICTS.md`

- Document coverage: **263 of 263** — 199 classified (188 prose + 11 task lists) plus 64
  `tasks-*.md` measured deterministically by `intel/task-completion-state.md`

- Bookkeeping notes:
  - `intel/SYNTHESIS.md` reports 107 requirements for run 1 and its per-PRD table sums to a third
    figure. The enumerated count (115) is authoritative here; reconciling the arithmetic is
    RECON-01.

  - `intel/SYNTHESIS.md`'s run-2 per-PRD table sums to 116 against the 118 IDs actually present.
    The two-entry difference is attribution: `REQ-council-grove-commander-integration` and
    `REQ-maneuver-validation` are counted into adjacent epic groups. The enumerated count (118) is
    authoritative here.

  - `intel/SYNTHESIS.md`'s run-3 per-PRD table sums to 112 against the 115 IDs actually present.
    Three-entry difference, all attribution: the table has no row for the
    `Epic_1/decisions/` source that carries `REQ-port-value-type-ownership-v1`, and it undercounts
    Milestone 5 Epic 2 and Epic 5 by one each (`REQ-port-value-type-ownership-v2` and
    `REQ-workspace-crate-edition-v2`). Enumerated per-source counts: M4 E1 7, E2 9, E3 9; M5 E1 9

    + 1 decision doc, E2 10, E3 9, E4 11, E5 10, E6 6; M6 E1 8, E2 9, E3 9, E4 8 = 115. The
    enumerated count is authoritative here. Third consecutive run with this class of
    discrepancy — RECON-01 owns the reconciliation.

  - SYNTHESIS reports 38 cumulative variant entries; INGEST-CONFLICTS counts 39 warnings. Both are
    correct — several variant groups carry three entries under one warning, and several run-3
    warnings are not `-v1`/`-v2` pairs at all (see *Run-3 unsettled positions* above).

  - Run 4's per-source enumeration reconciles exactly against SYNTHESIS's headline of 86, so this
    is the **first run without an arithmetic discrepancy**. Enumerated: M7 E1 12, E2 12, E3 10,
    E4 12; M8 E1 4, E2 4, E3 6, E4 4, E5 6, E6 4, E7 6; DOC-carried 5; plus
    `REQ-ci-publish-dry-run-v2`, which is sourced from `.github/workflows/ci.yml` rather than any
    document = 86. Note that this last ID has **no document carrier and therefore no precedence
    standing** — it is recorded because the technical position is substantive and contradicts an
    ingested requirement, not because a document asserts it.

  - Run 4 counts 53 cumulative warnings against 56 variant entries. The gap is the same class as
    run 3's: variant group 24 is a three-step chain under one warning, group 26 is an internal
    split inside one requirement, and six run-4 warnings are not `-v1`/`-v2` pairs at all (see
    *Run-4 unsettled positions* above).

  - **Run 5 reproduces the run-1/2/3 discrepancy class, at its largest.**
    `intel/SYNTHESIS.md` headlines **120** requirements for run 5 and its per-milestone table
    ("Milestone 9 → 23; Milestone 10 → 21; Milestone 11 → 18; Milestone 12 → 32; Deferred-QA → 15;
    project-management → 1") sums to **110**. The enumerated count is authoritative and is **120**,
    reconciled per source: **M9 25** (E1 6, E2 5, E3 4, E4 4, E5 5, E6 1); **M10 23** (E1 4, E2 8,
    E3 6, E4 1, E5 4); **M11 20** (E1-2 4, E3 7, E4 4, E6 1, E5+E7 4); **M12 34** (E1 6, E2 4, E3 4,
    E4 5, E5 6, E6 4, E7 5); **Deferred-QA 17** (E25 7, E26 4, E27 2, E28/29 + register 4);
    **project-management 1**. 25+23+20+34+17+1 = 120. Each milestone is undercounted by two in the
    synthesis table, which is a consistent attribution offset rather than missing content — no ID is
    absent from `intel/requirements.md`. **Fourth run with this class of discrepancy**; RECON-01
    owns the reconciliation across all of them, and this is the last input it will receive.

  - Run 5 counts **69** cumulative warnings against **60** variant entries. Same class again:
    fourteen of run 5's sixteen warnings are not `-v1`/`-v2` pairs at all (see *Run-5 unsettled
    positions* above), and only two produced new groups.

  - **One run-5 finding is a correction rather than a count.** `intel/code-verification.md` run 5
    withdraws the run-4 claim that `deny.toml` was out of sync with `.cargo/audit.toml`. Both files
    carry the same five vulnerability advisories. This is the first time a later run has reversed an
    earlier run's direct code verification, and it is recorded at SEC-01 rather than silently
    edited away.

---
*Requirements defined: 2026-07-30*
*Last updated: 2026-07-30 after **ingest run 5 of 5 — FINAL, THE INGEST IS COMPLETE**
(`.project/Milestone_9-Classic-Orchestrator-Completion` +
`.project/Milestone_10-CI-Hardening-Release-Automation` +
`.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
`.project/Deferred-QA-CICD-Completion` + `.project/project-management`, 46 docs; **cumulative 263
documents covered — 199 classified plus 64 task lists measured deterministically — 554
requirements, 60 variant entries across 30 groups, 69 warnings, 0 locked decisions, 0 blockers,
11 ADR candidates**).*

*Run 5 added 24 forward requirements (SUPPLY-01 … SUPPLY-03, ORCH-01 … ORCH-05, WEB-01 … WEB-04,
PIPE-01 … PIPE-05, DEFER-01 … DEFER-03, DOCS-01 … DOCS-04) across Phases 12-16, and a 120-row
Milestone 9-12 as-shipped ledger. **Phases 1-11 are unchanged and unrenumbered.** DEBT-01 was
extended in place (six stale references became nine) and SEC-01 was corrected in place (the
`deny.toml` out-of-sync finding is withdrawn); no requirement was duplicated. Seventeen ID prefixes
are now spent: `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*`, `VERIFY-*`, `CLOSE-*`, `ARCH-*`, `DEBT-*`,
`SEC-*`, `HARD-*`, `FACADE-*`, `SUPPLY-*`, `ORCH-*`, `WEB-*`, `PIPE-*`, `DEFER-*`, `DOCS-*`.
`REQ-*` IDs remain the stable keys. **There is no run 6.***
