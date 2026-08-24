# Roadmap: Paladin

## Overview

**Paladin already works.** It ships at v0.7.0 with a Cargo workspace of ten library crates plus a
`doc-examples` crate and the root `paladin-ai` facade, 22 runnable examples, an HTTP API with
OpenAPI and SSE streaming, a `paladin-server` binary, a 112 MB multi-arch Docker image and
reference Kubernetes manifests. (**Amended by Phase 4, dated 2026-08-03, citing
`04-release-measurement.md`**: the "22 runnable examples" figure traces to a Milestone 1 Epic 10
validation report ("22/22 examples compiling") and has since gone stale — the shipped tree carries
**47** `.rs` files under `examples/`, of which four are declared `[[example]]` targets gating on
non-default features (`vision`, `content-processing`, `web-server`); no crate under `crates/` ships
its own `examples/` directory. The shipped tree outranks an ingested count under this project's
precedence order. Going forward the gate REL-05 and ROADMAP criterion 5 express is "every example
target builds", not a count, so this figure cannot go stale the same way again.)

**This planning corpus is a historical record of twelve shipped milestones plus a verified-defect
and deferred-work forward scope. It is not a greenfield plan.** Across the 263 documents in
`.project/`, 7,511 of 8,053 task items are checked (93%) — and five runs of direct code
verification found the shipped tree *ahead of* even that figure in most places. **This roadmap does
not build the framework. It closes out milestones that already shipped, fixes what verification
proved broken, and builds the one epic-set nobody ever started.**

The sixteen phases fall into four kinds of work, and it is worth naming them before the detail:

| Kind | Phases | What it is |
|---|---|---|
| **Record** | 1, 5, 7, 10, 13 | Make `.planning/` describe the code as it actually is, so nobody re-plans shipped work or applies a superseded PRD literally |
| **Verified defect** | 8, 12 | Fix the things direct verification proved broken — a permanently red CI job, missing annotations, disabled doctests, leaked dependencies, a duplicated audit job |
| **Decision** | 9, 11, 14 | Answer the questions the corpus left open, including the ones with a correctness or security consequence attached |
| **Genuinely unbuilt** | 2, 3, 4, 6, 15, 16 | The residual functional gaps, the quality gates, and the two deferred registers whose work was never started |

### The milestone arc this roadmap closes out

**M1-M3 built capability.** Paladin, Garrison, Arsenal, the four base Battalion patterns, Herald,
Citadel, Commander and the Armory CLI (M1); then Sanctum vector memory and RAG, Sentinel vision,
autonomous agents, Conclave, Council, Grove and the Maneuver Flow DSL (M2); then the completion
pass over all of it (M3).

**M4-M8 dismantled and rebuilt the structure that capability lived in**, at considerable cost and
with almost no feature work: feature-flag expansion and port hardening (M4), the monolith becoming
a Cargo workspace (M5), four layer relocations (M6), four more crate extractions and the first
crates.io publish (M7), and a facade cleanup that a dated reconciliation then audited against the
tree and took further than its own plan allowed (M8).

**M9-M12 finished, hardened, documented and exposed it.** M9 completed the half of the platform
M4-M8 had left alone — a real `execute_workflow()`, a workflow repository with crash recovery,
scheduler/queue/event validation, the bidirectional content-agent bridge and user/admin RBAC. M10
made it releasable: pre-commit, cargo-audit + cargo-deny + OSV-Scanner, a CycloneDX SBOM,
cargo-release with dependency-ordered publishing, and — after an incident — main-only tag
enforcement. M11 documented it into an mdbook with 227 broken links repaired and linkcheck as an
error. M12 exposed it over HTTP, **and it exists because M11's documentation epic wrote down a
capability gap instead of papering over it.**

### What the phases do, in order

**Milestone 1 close-out (Phases 1-4)** is short and specific. Make the planning record match the
shipped code and give each of the six contested type/gate definitions one recorded answer
(Phase 1). Close the residual functional gaps verification exposes and apply those definitions in
code (Phase 2). Make the quality numbers real rather than aspirational (Phase 3). Make the release
coherent — one version, one edition, a defensible advisory posture, reviewed docs, the whole gate
suite green (Phase 4).

**Milestone 2-3 close-out (Phases 5-6)** is shorter still, and that is the finding rather than an
omission. Sanctum, RAG, Sentinel vision, autonomous planning and handoffs, Conclave, Council,
Grove, the Maneuver Flow DSL, the enhanced CLI, Herald consolidation, the Paladin registry and the
scheduler port **all ship in the v0.7.0 tree.** What is missing is the record (Phase 5). Exactly one
defect in run-2 scope is verified open, and it closes alongside whatever Phase 5 exposes (Phase 6).

**Milestone 4-6 close-out (Phases 7-8)** covers the three milestones that restructured what M1-M3
built. All of it shipped, and unusually for this corpus it was verified directly against
`Cargo.toml` contents and type definitions rather than inferred. Phase 7 records what shipped and
answers the variant pairs; **Phase 8 is the first phase whose scope is entirely verified defects.**

**Milestone 7-8 close-out (Phases 9-11)** is the first block where the *record* is in better shape
than the gates. The 2026-06-04 reconciliation is the most reliable document in the corpus — every
verifiable claim in it matches the tree, including a `println!` residue count exact to the
occurrence. So Phase 9 fixes the gates rather than the record and **carries the only dated item in
the corpus, a RustSec acceptance expiring 2026-09-30**. Phase 10 writes down what M7-M8 delivered.
Phase 11 disposes of the deferred registers.

**Milestone 9-12 + Deferred-QA close-out (Phases 12-16)** is where the last of the forward work
lives, and it splits cleanly. Phase 12 deletes eighteen lines of CI that falsify a completed
milestone's own success metric, and gives thirteen advisory suppressions an owner and a date.
Phase 13 records what four milestones delivered and answers two seams M12 left as defaults.
Phase 14 closes the gap between what the project's interfaces *advertise* and what they *do* — an
API documented as JWT and implemented as opaque tokens, a Kubernetes Deployment against an
in-process token store, and an LLM capability flag that over-reports. Phase 15 builds the quality
gates Deferred-QA Epic 25 specified and nobody started, then closes the coverage register those
gates measure. Phase 16 finishes Milestone 11's documentation currency — **the only open checkbox
count in all 542 that survives verification** — and decides the fate of an architecture document
frozen at 311 lines that two milestones made invisible.

## Milestones

| Milestone | Phases | Status | Source |
|---|---|---|---|
| **Milestone 1 close-out** | 1-4 | ✅ **Shipped v0.7.1 (2026-08-04)** — [archive](milestones/v0.7.1-ROADMAP.md) | Ingest run 1 — `.project/Milestone_1-MVP` (36 docs) |
| **Milestone 2-3 close-out** | 5-6 | ◆ **Current — v0.7.2, started 2026-08-04** | Ingest run 2 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs) |
| **Milestone 4-6 close-out** | 7-8 | Not started | Ingest run 3 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements` (32 docs) |
| **Milestone 7-8 close-out** | 9-11 | Not started | Ingest run 4 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs) |
| **Milestone 9-12 + Deferred-QA close-out** | 12-16 | Not started | Ingest run 5 (FINAL) — `.project/Milestone_9-Classic-Orchestrator-Completion` + `.project/Milestone_10-CI-Hardening-Release-Automation` + `.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` + `.project/Deferred-QA-CICD-Completion` + `.project/project-management` (46 docs) |
| **Provider Expansion** | 17- | Not started | Forward work — not ingest-derived. Added 2026-08-15 per *Roadmap Extension Protocol* item 1. |

**The ingest is complete.** All 263 documents in `.project/` are covered — 199 classified across
five runs and 64 `tasks-*.md` measured deterministically by `intel/task-completion-state.md`. There
is no run 6. The *Roadmap Extension Protocol* at the end of this file still governs any future
addition, but nothing is pending.

Milestone numbering follows the **directory / task-list numbering**. Four source milestones number
themselves differently — the M4-M6 overviews use refactoring tiers ("Milestone 1/2/3"), the M3
release notes assign Epics 19-23 to four M2 features, and the M7 overview titles itself
"Milestone 4" — and none of those labels is used as a key anywhere in this file (VERIFY-03,
ARCH-02, HARD-04). **The protocol predicted a fifth instance in run 5; run 5 found none, and
ORCH-05 records the prediction closed.**

## Phases

**Phase Numbering:**

- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

<details>
<summary>✅ <strong>Milestone 1 close-out (Phases 1-4)</strong> — SHIPPED v0.7.1 2026-08-04 · 38 plans, 25/25 requirements</summary>

- [x] **Phase 1: Ground Truth & Decision Records** - Verify the planning record against shipped v0.7.0 code and record one answer per competing variant pair (completed 2026-07-31)
- [x] **Phase 2: Functional Gap Closure** - Finish the residual Milestone-1 functionality and apply the recorded definitions in code (completed 2026-08-01)
- [x] **Phase 3: Verification Depth** - Make coverage, error-path testing and performance baselines real and measured (completed 2026-08-02)
- [x] **Phase 4: Release Coherence** - One version, one edition, defensible dependencies, reviewed docs, green gate suite (completed 2026-08-03)

Full detail: [`milestones/v0.7.1-ROADMAP.md`](milestones/v0.7.1-ROADMAP.md) ·
Audit: [`milestones/v0.7.1-MILESTONE-AUDIT.md`](milestones/v0.7.1-MILESTONE-AUDIT.md) ·
Phase artifacts: `milestones/v0.7.1-phases/`

</details>

**Milestone 2-3 close-out** — ◆ **CURRENT MILESTONE v0.7.2** (started 2026-08-04, 9 requirements)

- [x] **Phase 5: Milestone 2-3 Ground Truth** - Record what Epics 11-24 actually shipped, verify the three unverified blocks, and fix the epic-numbering defect at its source (completed 2026-08-05)
- [x] **Phase 6: Verified Gap Closure** - Close the one verified defect plus whatever Phase 5 proves genuinely outstanding (completed 2026-08-05)

**Milestone 4-6 close-out**

- [x] **Phase 7: Workspace Ground Truth & Recorded Answers** - Record what the refactor milestones actually shipped, correct the five positions the code contradicts, and answer the four variant pairs and two policy questions (completed 2026-08-06)
- [x] **Phase 8: Verified Defect Closure** - Fix the five defects verification proved open: the broken API-surface CI job, missing deprecations, disabled port doctests, leaked CLI dependencies, and duplicate `TokenUsage` (completed 2026-08-07)

**Milestone 7-8 close-out**

- [x] **Phase 9: Release & Security Gate Integrity** - Reconcile the four divergent RustSec exception sets before the 2026-09-30 expiry, settle the licence posture, and close the three small release-gate defects (completed 2026-08-08)
- [x] **Phase 10: Milestone 7-8 Ground Truth & Recorded Account** - Record what production hardening and facade cleanup actually delivered, make the 2026-06-04 reconciliation authoritative, and answer the three architecture questions the documents left ambiguous (completed 2026-08-08)
- [x] **Phase 11: Facade Residue & Deferred Register Disposition** - Give each of the five deferred items and both deliberately removed features a decision, and triage the Milestone 9 candidate list (completed 2026-08-09)

**Milestone 9-12 + Deferred-QA close-out**

- [x] **Phase 12: Supply-Chain Gate Integrity** - Delete the duplicate audit job that falsifies a completed milestone's success metric, and give every advisory suppression an owner and a date (completed 2026-08-10)
- [x] **Phase 13: Milestone 9-12 Ground Truth & Recorded Account** - Record what the orchestrator, release-automation, documentation and Web API milestones delivered, and turn two recorded defaults into decisions (completed 2026-08-10)
- [x] **Phase 14: API Contract Truthfulness** - Make every capability the project advertises through an interface one it actually has — the token mechanism, the multi-replica store, and the LLM capability flag (completed 2026-08-12)
- [x] **Phase 15: Coverage & CI Quality Gates** - Build the quality gates Deferred-QA Epic 25 specified and nobody started, then close the coverage register those gates measure (completed 2026-08-13)
- [ ] **Phase 16: Documentation Currency & the Architecture Gap** - Settle Milestone 11's fourteen content-currency files by content, and decide whether the 311-line architecture document is archive or deliverable

**Provider Expansion** — first forward work beyond the ingest (added 2026-08-15)

- [x] **Phase 17: Additional LLM Provider Adapters** - Decide which additional providers qualify against recorded criteria, then ship each survivor as a feature-gated adapter meeting the full `LlmPort` contract (completed 2026-08-23)

## Phase Details

<details>
<summary>✅ <strong>Milestone 1 close-out — Phases 1-4</strong> — SHIPPED v0.7.1 2026-08-04 (full detail archived at <code>milestones/v0.7.1-ROADMAP.md</code>)</summary>

### Phase 1: Ground Truth & Decision Records

**Goal**: `.planning/` describes the v0.7.0 code as it actually is, and each of the six contested definitions has exactly one recorded, evidence-cited answer that later milestones can build on.
**Depends on**: Nothing (first phase)
**Requirements**: RECON-01, RECON-02, RECON-03, RECON-04, RECON-05, RECON-06, RECON-07, RECON-08
**Success Criteria** (what must be TRUE):

  1. A developer can open one status ledger and see, for every outstanding Milestone-1 task item, whether shipped code already satisfies it — each verdict carrying a `file:line` citation rather than a task-list checkbox.
  2. Six ADRs exist, one per competing variant pair (`BattalionConfig`, `BattalionResult`, Formation minimum Paladin count, temperature range, `Herald` trait signature, coverage gate), each naming the chosen variant and the shipped code it was checked against.
  3. The ledger records the places where shipped code has already superseded an ingested requirement — MCP Streamable-HTTP in place of SSE, Sanctum/Qdrant in place of `sqlite-vss`, the interactive REPL that Epic 9 declared a non-goal — so no later phase mistakes divergence for a defect.
  4. The coverage question has one number and one scope, so Phase 3 can objectively pass or fail against it instead of choosing between 80% and 85%.
  5. The Epic 10 Task 7.0 dispute is answered — either the Final Documentation Review is outstanding work with an owner, or the validation report is recorded as wrong — and the 102-vs-103 subtask discrepancy is explained.

**Plans**: 12 plans

Plans:
**Wave 1**

- [x] 01-01-PLAN.md — Tracer: decision-record substrate end-to-end (`decisions/` + `ledgers/` + PROJECT.md precedence order) and the Herald trait ADR

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 01-02-PLAN.md — ADR-0001 `BattalionConfig` and ADR-0002 `BattalionResult`
- [x] 01-03-PLAN.md — ADR-0003 Formation minimum Paladin count and ADR-0004 temperature validation
- [x] 01-04-PLAN.md — Measure workspace coverage and record ADR-0006, the coverage gate *(**superseded, not executed**: halted at its own precondition — `cargo-llvm-cov` not installable, crates.io HTTP 403, no Docker. Scope delivered by gap-closure plans 01-09 and 01-10 via an offline measurement path. Closed out 2026-08-03; disposition recorded in [01-04-SUMMARY.md](phases/01-ground-truth-decision-records/01-04-SUMMARY.md))*
- [x] 01-05-PLAN.md — Ledger divergences, the Epic 10 Task 7.0 verdict, and the ingest-bookkeeping corrections

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 01-06-PLAN.md — Ledger rows for Epics 1-5 with nested outstanding task items

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 01-07-PLAN.md — Ledger rows for Epics 6-10 plus the outstanding-item reconciliation

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 01-08-PLAN.md — Source corrections: PROJECT.md Key Decisions, REQUIREMENTS.md pointer, ROADMAP Phase 3 criterion *(partial — Tasks 2 and 3 correctly withheld pending a complete ledger and ADR-0006)*

**Wave 6** *(gap closure — from `01-VERIFICATION.md`; blocked on Wave 5 completion)*

- [x] 01-09-PLAN.md — Tracer: prove the offline coverage pipeline end-to-end, then measure the workspace and record the raw evidence
- [x] 01-11-PLAN.md — Add the missing `REQ-battalion-result-v1` ledger row, complete the REQUIREMENTS.md pointer reduction, and close Broken Windows item 1

**Wave 7** *(blocked on Wave 6 completion)*

- [x] 01-10-PLAN.md — Lock the recorded coverage scope, then author ADR-0006: one number, one scope, one floor, one target, one ratchet

**Wave 8** *(blocked on Wave 7 completion)*

- [x] 01-12-PLAN.md — Fill PROJECT.md's ADR-0006 row, amend ROADMAP Phase 3's coverage criterion, and record RECON-07 as satisfied

**Cross-cutting constraints:**

- Every row carries exactly one of the five legend verdicts and no sixth class is invented (D-20).
- Every `satisfied` row carries a `file:line` citation PLUS a named passing test, example or command that exercises it; a row with a citation and no named exerciser is `present, unproven` (D-19).

### Phase 2: Functional Gap Closure

**Goal**: Every Milestone-1 functional requirement is either working and tested, or explicitly deferred with a recorded reason — and the types in code match the Phase 1 decisions.
**Depends on**: Phase 1
**Requirements**: GAP-01, GAP-02, GAP-03, GAP-04, GAP-05, GAP-06, GAP-07
**Success Criteria** (what must be TRUE):

  1. `cargo test --workspace` passes with zero failures, including `test_auto_selects_campaign_for_workflow_keywords`.
  2. A developer can run a Chain of Command battalion from an example and watch the commander select specialists, survive a specialist failure through fallback logic, and return a synthesized answer — with tests covering all four delegation strategies.
  3. A Battalion result rendered through the JSON, Markdown and Table Heralds shows Battalion name, ID and type, per-Paladin results in execution order, aggregated token usage across Paladins, and partial results when something failed.
  4. Commander execution returns a normalized result carrying strategy used, per-Paladin timings, success/failure counts and preserved errors, and writes telemetry metadata to `metadata_output_dir` when one is configured.
  5. The shipped types match the Phase 1 ADRs: the duplicate `BattalionConfig` in `citadel.rs` resolved (renamed `BattalionCheckpointConfig`, ADR-0001), and a single-Paladin Formation that executes instead of failing validation (ADR-0003).

*(Amended 2026-08-01, Phase 2 plan 02-09: criterion 1's stale premise, asserting the named test
still failed at ROADMAP authoring time, is dropped — plan 02-01's baseline re-proof found
`test_auto_selects_campaign_for_workflow_keywords` passing on this tree, `milestone-01.md:316-317`
already recorded the premise as stale, and the outcome the criterion asks for is otherwise
unchanged. Criterion 5's "one `BattalionResult`" clause is dropped —
ingest run 3 code verification already found `BattalionResult` resolves to a merged superset
satisfying all consumers (`PROJECT.md`, `ADR-0002`), so the clause asserted a premise that was
already true in shipped code before this phase started; the remaining two clauses restate what
GAP-07 actually narrowed to. Phase 1 set the precedent of correcting the ROADMAP at source for the
same reason when it amended Phase 3's criterion 1.)*

**Plans**: 11 plans

Plans:
**Wave 1**

- [x] 02-01-PLAN.md — Measured `cargo test --workspace` baseline with full provenance, plus one executable re-proof each for GAP-01, GAP-02, GAP-04 and GAP-05 (D-01)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 02-02-PLAN.md — GAP-07 ports tracer: `temperature_range` on `ProviderCapabilities`, provider-aware temperature validation, and WEB-03's honest tool-calling flag (D-13/D-14/D-15/D-16)
- [x] 02-03-PLAN.md — GAP-07 domain edits: `Formation::validate` relaxed to one Paladin (ADR-0003) and the `citadel.rs` checkpoint-config rename (ADR-0001)
- [x] 02-04-PLAN.md — GAP-03 producer and renderers: Formation per-Paladin aggregation, plus JSON/Markdown/Table Herald battalion rendering (the Table Herald stub replaced)
- [x] 02-06-PLAN.md — Reactivate the never-compiled `tests/unit/llm/` module and write the provider-switching integration test (D-10/D-11, Epic 6 tasks 7.10-7.12)
- [x] 02-07-PLAN.md — Unblock the CLI test cluster with a helper shim and run the five suites (D-09, Epic 9 tasks 13.4-13.6)
- [x] 02-08-PLAN.md — GAP-06: the Epic 2 Garrison PRD-acceptance review, with task 11.5's coverage check dispositioned as superseded (D-04)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 02-05-PLAN.md — GAP-03 proof: one Formation-driven end-to-end test through all three Heralds, including partial results (D-06, Epic 8 task 7.13)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 02-09-PLAN.md — Close the record: D-12 test-wiring sweep, ADR-0007 cancellation deferral (D-05/D-08), in-place ledger amendments (D-02) and the ROADMAP criterion corrections

**Wave 5** *(gap closure — added 2026-08-01 from 02-VERIFICATION.md)*

- [x] 02-10-PLAN.md — Fix the Table Herald's byte-index truncation panic on multi-byte Paladin names: a char-boundary-safe, total `truncate_text`, closing the blocker that falsified success criterion 3 and GAP-03

**Wave 6** *(blocked on Wave 5 completion)*

- [x] 02-11-PLAN.md — Check off GAP-01…GAP-07 in REQUIREMENTS.md and set their Traceability rows to Complete, with dated provenance citing each requirement's closing plan

### Phase 3: Verification Depth

**Goal**: The project's quality claims are measurements rather than targets — coverage at the recorded gate, error paths executed rather than skipped, and performance baselines that exist.
**Depends on**: Phase 2
**Requirements**: QUAL-01, QUAL-02, QUAL-03, QUAL-04, QUAL-05
**Success Criteria** (what must be TRUE):

  1. The workspace-wide line-coverage figure is at or above the hard-fail floor recorded in [ADR-0006](.planning/decisions/0006-coverage-gate.md) — one number, from one reproducible command, with feature set, path exclusions and the doctest decision fixed by that ADR rather than restated here. (Amended by Phase 1 under RECON-07 and D-08, replacing the prior unit/integration split and its stale baselines.)
  2. No first-party source file reports 0% coverage — the arsenal execution and registry services, the Redis and MinIO adapters, the user controller and repository, and `main.rs` all have exercising tests. (**Amended by Phase 3, dated 2026-08-02, citing `03-coverage-measurement.md`**: the named file list was re-derived from a measurement of the shipped tree, not restated from the ingested pre-workspace claim it originally came from. The arsenal execution and registry services, the user controller and repository, and `main.rs` were already covered when this criterion was written — none was ever a true 0% file at this measurement's scope. The Redis and MinIO adapters are **not in the same position**: `redis.rs` was a true 0.00% at entry and is now closed at 34.69% (plan 03-05's Docker-free unit tests), with its live-server paths deferred with reason to Phase 15 / PIPE; `minio.rs` has no denominator entry at all under the default feature set — the `s3` feature that gates it is out of ADR-0006's recorded scope, not a file the pipeline measured and found empty — deferred with reason, owner VERIFY-05 / PIPE-02. `src/bin/paladin-server.rs`, not named in this criterion's original text, is the one file that does remain a true 0% today, deferred with reason to Phase 5 / VERIFY-05.)
  3. Commander failure behaviour is proven by tests that actually run: retry counts increment, partial failures are collected and returned, and a timeout stops sibling agents — none of it behind `#[ignore]`.
  4. Each MCP tool-invocation failure mode has a passing test: expired or rejected token, malformed response, handshake timeout, unknown tool, and bad arguments.
  5. `cargo bench` completes across the restored suites and a baseline document records throughput, P50/P95/P99 latency, memory per Paladin and startup time — so the next performance change can be compared against something.

**Plans**: 8 plans in 4 waves

Plans:
**Wave 1**

- [x] 03-01-PLAN.md — Coverage re-measurement tracer: reproduce ADR-0006's pipeline end-to-end at HEAD, re-derive the zero-coverage set (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 03-02-PLAN.md — `FaultyPaladinPort` and the four relocated Commander error-path tests (wave 2)
- [x] 03-03-PLAN.md — Five MCP failure modes plus the additive `connect_streamable_http_with_timeout` seam (wave 2)
- [x] 03-04-PLAN.md — Performance baseline: five bench targets, derived P50/P95/P99, memory and startup harness (wave 2)
- [x] 03-05-PLAN.md — `redis.rs` helper refactor then Docker-free unit tests (wave 2)
- [x] 03-06-PLAN.md — `paladin-ports` and `paladin-llm` zero-coverage closure (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 03-07-PLAN.md — Exit re-measurement and QUAL-03 critical-path exerciser evidence (wave 3)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 03-08-PLAN.md — Amend QUAL-02, QUAL-03, ROADMAP criterion 2 and the ledger at source; human confirmation (wave 4)

### Phase 4: Release Coherence

**Goal**: A developer can clone the release tag, build it, trust its version and its dependency posture, follow the quickstart to a working agent, and see CI prove all of it.
**Depends on**: Phase 3
**Requirements**: REL-01, REL-02, REL-03, REL-04, REL-05
**Success Criteria** (what must be TRUE):

  1. Version metadata tells one story — workspace `Cargo.toml`, member crate versions, the git tag and the release notes agree, replacing today's three-way disagreement between branch `release/v0.7.0`, `Cargo.toml` 0.6.0 and tag v0.5.1.
  2. Every workspace crate declares the same valid Rust edition and `cargo build --workspace` succeeds under it, ending the `edition = "2024"` / `"2021"` split.
  3. `cargo audit` and `cargo deny` report no high or critical advisories, and every ignored advisory carries a written rationale plus a migration or review note — no silent suppressions.
  4. A developer following QUICKSTART on a clean machine reaches a working agent, and the elapsed time is recorded against the documented under-15-minute target (measured for the first time, pass or fail).
  5. CI on the release branch proves the full gate suite: format, clippy with warnings as errors, workspace tests, doc tests, all 22 examples, the multi-arch Docker build inside its size and time budget (**Amended by Phase 4, dated 2026-08-03, citing `04-ci-gate-deferrals.md` §"Second CI execution"**: the "< 5 min" build-time figure is **scoped to single-arch** and is advisory for the multi-arch build. It derives from `PROJECT.md:767`'s single-arch "112 MB built in 5m31s" (Milestone 1) but is applied here to a multi-arch build, and on that build it has never been met in this repository's history: Release runs v0.4.2 48m09s, v0.4.3 47m58s, v0.5.0 41m47s, v0.5.1 44m03s, and the first CI execution of this gate 49m43s — **measured 2946 s against 300 s** on 2026-08-03. `Dockerfile:33` builds natively per platform, so `linux/arm64` compiles the whole workspace under QEMU emulation; the number measures the GitHub runner, not Paladin. The CI job now reports it as a warning. **The < 500 MB size budget remains a hard gate** — size is a property of the artifact, and the last successful multi-arch build measured **86 MB**. Replacing QEMU with native `ubuntu-24.04-arm` runners (free for public repositories) and reinstating a hard, evidence-backed time budget is **Phase 15 / PIPE**.)**, and the Kubernetes smoke test inside its startup budget. (**Amended by Phase 4, dated 2026-08-03, citing `04-release-measurement.md`**: "all 22 examples" is stale — the same Milestone 1 Epic 10 validation-report figure amended in the Overview above. The shipped tree carries 47 `.rs` files under `examples/`, 4 declared `[[example]]` targets gating on non-default features, 0 crate-level `examples/` directories. This criterion's own gate is corrected to "every example target builds" — a property proven by a 4-invocation feature matrix plus a binary-presence assertion, not a count restated from an ingested report.)

**Plans**: 7 plans

Plans:

**Wave 1** *(parallel — no shared files)*

- [x] 04-01-PLAN.md — Edition unification (tracer): bump `paladin-ports` and `paladin-notifications` to 2024, prove both build legs, create the measurement record (wave 1)
- [x] 04-02-PLAN.md — Advisory posture: remove the stale suppression, complete six migration/review notes, record the measured verdicts and four newly-surfaced advisories (wave 1)
- [x] 04-03-PLAN.md — CI repair: `release/**` push trigger, examples feature-matrix job, Docker size/time budgets, kind-based Kubernetes smoke job, deferral register (wave 1)

**Wave 2** *(blocked on 04-01)*

- [x] 04-04-PLAN.md — Local gate suite proven and recorded; the "22 examples" figure re-derived and amended at source (wave 2)

**Wave 3** *(blocked on 04-04 — the tag must land on a gate-green commit)*

- [x] 04-05-PLAN.md — Version convergence to 0.7.0: twelve manifests, every internal pin, CHANGELOG finalize, local unpushed tag, human release gate (wave 3)

**Wave 4** *(blocked on 04-05)*

- [x] 04-06-PLAN.md — QUICKSTART structural repair and the first recorded timing measurement (wave 4)

**Wave 5** *(blocked on all prior waves)*

- [x] 04-07-PLAN.md — ADR-0008 (version) and ADR-0009 (edition), the CONCERNS correction, Phase 7 requirement citations, and the REL-01..REL-05 ledger (wave 5)

</details>

**Milestone 2-3 close-out — Phases 5-6 (not started)**

### Phase 5: Milestone 2-3 Ground Truth

**Goal**: A developer can open `.planning/` and get a truthful account of what Epics 11-24 delivered — which of the 118 requirements the shipped tree satisfies, which of two competing surfaces each feature actually implements, and what the three unverified open-checkbox blocks contain — with the epic-numbering defect corrected at its source so it stops propagating.
**Depends on**: Phase 1 (RECON-07 must exist before VERIFY-05 can extend it; the rest of Phase 5 is independent of Phases 2-4). **Satisfied — Phase 1 shipped in v0.7.1.**
**Requirements**: VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06

**Inherited from the v0.7.1 close-out** (STATE.md → Deferred Items, 2026-08-04). Both land on
VERIFY-05 and must be dispositioned by it rather than rediscovered:

- `src/bin/paladin-server.rs` at **0.00% coverage** — closing it needs a `run()` seam extracted from
  `main()`; owner recorded as Phase 5 / VERIFY-05.

- `minio.rs` sits **outside ADR-0006's default-feature scope** — owner recorded as
  VERIFY-05 / PIPE-02, so VERIFY-05 decides the scope question and Phase 15 applies it.

**Open-ended risk to watch:** VERIFY-04 asks whether Epic 13's encryption-at-rest requirement
(`REQ-vision-security-encryption`, recorded *"Not found in tree… not carried as forward work until
that answer exists"*) was consciously dropped. **If the answer is no, that is new security work with
no phase home anywhere in Phases 5-16** and needs a placement decision, not a silent absorption.
**Success Criteria** (what must be TRUE):

  1. A developer can look up any of the 118 Milestone 2-3 requirement IDs and see a `file:line`-cited verdict — satisfied, diverged, partial, or genuinely outstanding — instead of a PRD path that predates the workspace decomposition and no longer resolves.
  2. Asking "is Epic 22 / Epic 14 / Epic 24 done?" returns a written verdict per block rather than a checkbox count, and the answer is reproducible from the tree — closing the last three blocks `intel/code-verification.md` left unverified.
  3. Reading `RELEASE_NOTES_MILESTONE_3.md` no longer teaches the wrong epic numbers: Epics 19-24 name Herald consolidation, Vision, Autonomous, Battalion hardening, CLI/Config and Test hardening, its Milestone-4 forward-look is marked superseded, and the two claims verified absent from the tree (`RoutingStrategy::PerformanceBased`, the divergent Council and Maneuver API forms) are corrected or withdrawn.
  4. A developer choosing a vision entry point can see that both `vision_llm_port.rs` and `vision_port.rs` ship deliberately, which one to use, and whether Epic 13's encryption-at-rest requirement was dropped on purpose — instead of inferring a migration from a PRD conflict.
  5. One coverage number and one scope survive across all four competing positions (80 / 85 / 75-layered / 80-Epic-24), the two module-scoped gates are placed relative to it, and the ~78% measured figure can be judged pass or fail against it.
  6. A CI run with no API keys behaves the way one recorded decision says it should — loud failure or clean skip — and `llm_live_api_tests.rs` matches that decision rather than contradicting the PRDs that specified it.

**Plans:** 13/13 plans complete

Plans:
**Wave 1**

- [x] 05-01-PLAN.md — Ledger scaffold with 118 keyed rows plus Epic 11 fully cited (tracer), and REQUIREMENTS.md reduced to a pointer (wave 1)
- [x] 05-02-PLAN.md — ADR-0010 epic numbering, and the annotated RELEASE_NOTES_MILESTONE_3.md correction (wave 1)
- [x] 05-03-PLAN.md — ADR-0011 vision surfaces plus encryption disposition, and ADR-0012 live-API missing-key behaviour (wave 1)
- [x] 05-04-PLAN.md — ADR-0006 amended in place: module-scoped gates, the two inherited dispositions, the ~78% falsifiability statement (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 05-05-PLAN.md — Epic 22 block verdict and cluster table, plus Epic 22's 10 ledger rows (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 05-06-PLAN.md — Epic 14 block verdict and cluster table, plus Epic 14's 8 ledger rows (wave 3)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 05-07-PLAN.md — Epic 24 block verdict and cluster table, plus Epic 24's 9 ledger rows (wave 4)

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 05-08-PLAN.md — Epic 13 and Epic 20 ledger rows (19), including the corrected encryption row (wave 5)

**Wave 6** *(blocked on Wave 5 completion)*

- [x] 05-09-PLAN.md — Epic 12 and Epic 15 ledger rows (13) (wave 6)

**Wave 7** *(blocked on Wave 6 completion)*

- [x] 05-10-PLAN.md — Epic 16 and Epic 18 ledger rows (18) (wave 7)

**Wave 8** *(blocked on Wave 7 completion)*

- [x] 05-11-PLAN.md — Epic 17 / 17.5 and Epic 19 ledger rows (16) (wave 8)

**Wave 9** *(blocked on Wave 8 completion)*

- [x] 05-12-PLAN.md — Epic 21 and Epic 23 ledger rows (17) (wave 9)

**Wave 10** *(blocked on Wave 9 completion)*

- [x] 05-13-PLAN.md — Ledger summary, consolidated CLOSE-02 scope, final counts, PROMOTION.md to 0013 (wave 10)

**Cross-cutting constraints:**

- The block verdict is `satisfied by shipped code` only if every parent-task cluster verifies; otherwise it is `partially outstanding`, the failing clusters are named, and those named clusters are exactly Phase 6 CLOSE-02's scope for this block and nothing else.

**Sequencing note:** the ledger is a single file, so the ten ledger-writing plans are chained one per
wave — same-wave plans must not share `files_modified`. The three ADR plans (05-02, 05-03, 05-04)
touch disjoint files and run in parallel with the scaffold in wave 1, so ADR-0010's corrected epic
numbering is available before any ledger row cites it. **Phase 6 must not be planned until 05-07
completes** — CLOSE-02's scope is set by the three block verdicts from 05-05, 05-06 and 05-07.

### Phase 6: Verified Gap Closure

**Goal**: Every Milestone 2-3 gap that verification actually proved is closed or explicitly deferred with a recorded reason — and no shipped surface is removed without a decision behind it.
**Depends on**: Phase 5 — **hard, and for scope as well as sequence.** CLOSE-02's size is set by VERIFY-02's verdicts (0 to 155 items) and CLOSE-03 derives from VERIFY-04 and VERIFY-06. **Plan this phase after Phase 5 reports, not alongside it.**
**Requirements**: CLOSE-01, CLOSE-02, CLOSE-03

**Inherited from the v0.7.1 close-out** (STATE.md → Deferred Items, 2026-08-04):

- **WARN-01 — Herald is not reachable from Campaign, Chain of Command, or the Commander router.**
  Formation and Phalanx wire Herald; the other three carry zero references. `format_battalion_result`
  is pattern-agnostic so no requirement's text is falsified, but the composite Chain-of-Command
  developer flow does not compose without the caller invoking a Herald directly. Recorded
  *"Unassigned — candidate for Phase 6"*; this milestone should either adopt it under CLOSE-02/03 or
  record a decision declining it.

  **Outcome, recorded 2026-08-05 by plan 06-07:** this milestone **adopted** WARN-01 under CLOSE-02
  rather than declining it. All three previously Herald-less services —
  `crates/paladin-battalion/src/campaign_service.rs`, `chain_of_command_service.rs`, and
  `commander.rs` — now carry the Herald triad (field, `with_herald` setter, format-wrapper),
  replicated from the established `formation_service.rs`/`phalanx_service.rs` pattern, shipped by
  plan 06-02. The composite Chain-of-Command developer flow has an executable end-to-end witness,
  not just a compile check: `tests/integration/battalion_chain_of_command_herald_test.rs#chain_of_command_result_renders_through_json_herald`
  drives a real `ChainOfCommandExecutionService::execute` over mock Paladins and formats the result
  through a real `JsonHerald` (`06-02-SUMMARY.md`).
**Success Criteria** (what must be TRUE):

  1. A Grove battalion configured with Anthropic or DeepSeek routes through *that* provider: the hardcoded `model: "gpt-4"` at `grove_service.rs:537` is gone, the routing model comes from configuration, and a test proves a non-OpenAI model reaches the LLM call.
  2. `grep -rn 'TODO' crates/paladin-battalion/src/` returns nothing that Epic 22's completion criteria claimed was already resolved.
  3. Every item VERIFY-02 classified as genuinely outstanding across Epics 14, 22 and 24 is either passing in `cargo test --workspace` or recorded as deferred with a written reason — and if verification found nothing outstanding, that verdict is recorded rather than the requirement quietly dropped.
  4. The live-API test harness and both vision surfaces match their Phase 5 recorded decisions in code, with any removal carrying a migration note.

**Plans**: 10 plans (7 executed + 3 gap-closure)

Plans:
**Wave 1**

- [x] 06-01-PLAN.md — CLOSE-01: Grove routing model from configuration, with a hard error when unset (wave 1, has a one-way decision checkpoint)
- [x] 06-02-PLAN.md — CLOSE-02 / WARN-01: Herald reachability for Campaign, Chain of Command and the Commander, with a composite end-to-end witness (wave 1)
- [x] 06-03-PLAN.md — CLOSE-02 / Epic 14 cluster 8.0: `autonomous` YAML section plus additive CLI flag overrides (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 06-04-PLAN.md — CLOSE-02 / Epic 24 cluster 1.0: ChainOfCommand benchmark and its dated baseline (wave 2)
- [x] 06-05-PLAN.md — CLOSE-03: vision entry-point rustdoc, the unimposed-encryption verdict, ADR-0011 amendment, live-API doc correction (wave 2)
- [x] 06-06-PLAN.md — CLOSE-01 record: ADR-0013, PROMOTION index, CHANGELOG breaking-change entry, PROJECT Key Decisions row (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 06-07-PLAN.md — CLOSE-01/02/03 close-out: ledger amendments, CI-job deferral recorded bidirectionally, Epic 22 "no work required" verdict, WARN-01 outcome (wave 3)

**Gap closure** *(planned after `06-VERIFICATION.md` reported `gaps_found`, 8/10 truths — waves restart at 1 for this run)*

`06-VERIFICATION.md` proved two failures against the four success criteria above. Truth 3: D-02's
"no fallback of any kind" guard in `route_by_llm` is correct in isolation but unreachable from
`GroveExecutionService::execute()`, because `route_task`'s blanket `Err` arm (untouched by plans
06-01 through 06-07) intercepts the deliberate configuration error and substitutes a fallback agent.
Truth 10: four permanent records — ADR-0013, `CHANGELOG.md`, `.planning/PROJECT.md` and
`.planning/REQUIREMENTS.md` — assert that unreachable behaviour as shipped fact. Truths 1, 2, 4-9 are
verified and are not re-planned; CLOSE-02 and CLOSE-03 are ✓ SATISFIED and get re-affirmation only.

- [x] 06-08-PLAN.md — CLOSE-01 gap: resolve the missing-`routing_model` configuration error before strategy dispatch in `route_task`, proved end to end through `execute()` with a configured `llm_port` (gap-closure wave 1)
- [x] 06-09-PLAN.md — CLOSE-01 record reconciliation: ADR-0013, `CHANGELOG.md` and PROJECT Key Decisions row amended to describe the `execute()`-reachable behaviour, with dated provenance (gap-closure wave 2)
- [x] 06-10-PLAN.md — CLOSE-01/02/03 re-close: REQUIREMENTS.md CLOSE-01 amendment reconciled, CLOSE-02/CLOSE-03 re-affirmed against commands re-run at HEAD, checkboxes and traceability rows flipped behind that evidence (gap-closure wave 3)

**Milestone 4-6 close-out — Phases 7-8 (not started)**

### Phase 7: Workspace Ground Truth & Recorded Answers

**Goal**: A developer can open `.planning/` and get a truthful account of the three milestones that restructured this codebase — which of the 115 requirements the workspace actually satisfies, which competing position each of the four variant pairs resolved to, and which five documented positions must never be applied literally because shipped code contradicts them.
**Depends on**: Nothing hard. Phase 7 is independent of Phases 1-6; see the coupling notes under Progress before running it after Phase 4.
**Requirements**: ARCH-01, ARCH-02, ARCH-03, ARCH-04, ARCH-05, ARCH-06, ARCH-07
**Success Criteria** (what must be TRUE):

  1. A developer can look up any of the 115 Milestone 4-6 requirement IDs and see a `file:line`-cited verdict — satisfied, relocated, superseded, diverged, or genuinely outstanding — and the ledger states the real workspace shape (ten library crates plus `doc-examples` plus the root `paladin-ai` facade), replacing both the "six crates" the milestone overviews assume and the "9-crate workspace" this planning set carried before.
  2. Someone applying an old PRD literally cannot break the build: the five positions shipped code contradicts are corrected at source — `vision` gating the encryption crates, the MCP transport flags, `web-server` gating actix-web, a `paladin-cli` crate, and `src/application/use_cases/` as the orchestration home — and the four relocated documentation deliverables are recorded as living in the mdbook rather than missing.
  3. Each of the four competing variant pairs has exactly one recorded answer citing the shipped code that settles it, and each answer says whether the documents get amended or the code is accepted as the resolution — including the one case where mechanical precedence gives the architecturally wrong result, so `PaladinResult`, `StopReason` and `TokenUsage` stop being pulled back out of `paladin-core` by a rule in a PRD.
  4. Asking "was Milestone 6 a breaking change?" returns one recorded answer with its version consequence, instead of a milestone overview and two PRDs that flatly disagree about whether the old import paths still resolve.
  5. Asking "which milestone is Milestone 1?" inside these three source milestones returns one answer, and `REQ-*` provenance keys resolve — closing the second numbering defect in this corpus with the same convention that closes the first.
  6. The ≥ 50% incremental-rebuild target can be judged pass or fail: either it is re-measured against the mid-tree baseline the benchmark report itself recommends, or it is restated per scenario — instead of a report whose table fails it four times out of five and whose conclusion declares it achieved.
  7. Every binary target has a documented purpose, closing the architecture review that Milestone 4 Epic 3 made a blocker and never produced.

**Plans**: 13 plans across 9 waves. Exactly one plan writes `.planning/ledgers/milestone-04-06.md` per wave, so the six ledger plans never collide. **ADR-0016 lands in wave 2**, which is what unblocks Phase 8 / DEBT-05.

Plans:
**Wave 1**

- [x] 07-01-PLAN.md — Tracer: ledger scaffold with 115 rows and Milestone 6 Epic 3 fully cited, REQUIREMENTS.md reduced to a pointer, `STRUCTURE.md` corrected to ten crates, ADR-0014 and the first `.project/` annotation (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 07-02-PLAN.md — ADR-0016 port value-type ownership plus the Epic 2 ports-extraction PRD annotation; carries the one-way reversibility gate and unblocks Phase 8 / DEBT-05 (wave 2)
- [x] 07-03-PLAN.md — ARCH-02 pointer banners on the eight byte-equivalent overview extracts, and the seven-versus-eight-versus-nine count reconciliation recorded in ADR-0014 (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 07-04-PLAN.md — ADR-0015 dependency allowlist and purity invariant, ADR-0017 LLM config bridge, plus the Epic 4 llm-extraction PRD annotation (wave 3)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 07-05-PLAN.md — ADR-0018 Milestone 6 facade re-export policy and its version consequence, plus the Milestone 6 overview and CircuitBreaker PRD annotations (wave 4)
- [x] 07-06-PLAN.md — Ledger: Milestone 5 Epics 1-2, 20 rows (wave 4)

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 07-07-PLAN.md — ADR-0019 binary-target architecture, ADR-0021 CLI placement, ADR-0020 build-benchmark restated per scenario (wave 5)
- [x] 07-08-PLAN.md — Ledger: Milestone 6 Epics 1, 2 and 4, 25 rows (wave 5)

**Wave 6** *(blocked on Wave 5 completion)*

- [x] 07-09-PLAN.md — ARCH-05's five source corrections across four `.project/` documents (wave 6)
- [x] 07-10-PLAN.md — Ledger: Milestone 5 Epics 3-4, 20 rows (wave 6)

**Wave 7** *(blocked on Wave 6 completion)*

- [x] 07-11-PLAN.md — Ledger: Milestone 5 Epics 5-6, 16 rows (wave 7)

**Wave 8** *(blocked on Wave 7 completion)*

- [x] 07-12-PLAN.md — Ledger: Milestone 4 Epics 1-3, 25 rows; the ledger reaches 115 of 115 verdicted (wave 8)

**Wave 9** *(blocked on Wave 8 completion)*

- [x] 07-13-PLAN.md — Close-out: counted verdict distribution, the five forward-scope handoffs, `PROMOTION.md` advanced to 0022, PROJECT.md Key Decisions, `COVERAGE.md`, phase-boundary check (wave 9)

**Cross-cutting constraints:**

- Row count in the ledger is still exactly 115, no `REQ-*` ID appears twice, and rows in these two sections appear in the order the scaffold fixed.

### Phase 8: Verified Defect Closure

**Goal**: The five defects that direct code verification proved open are fixed, so the guards the project believes it has actually work — and no shipped surface is removed without a recorded decision behind it.
**Depends on**: Phase 7 (ARCH-03(c) decides which crate owns the canonical `TokenUsage` before DEBT-05 consolidates to it; the other four DEBT items are independent and can start immediately)
**Requirements**: DEBT-01, DEBT-02, DEBT-03, DEBT-04, DEBT-05
**Success Criteria** (what must be TRUE):

  1. An intentional change to the public API makes CI fail and an unchanged tree makes it pass — the `api-surface` job stops exiting 1 with "No baseline found" on every run, which it has done since the `project/` → `.project/` rename, and `check-deprecations.sh` gets to execute. All nine stale `project/current-exports.txt` references are gone: five in tooling and four in requirement text.
  2. `grep -rn '#\[deprecated' src crates` either returns the transitional types Milestone 4 Epic 2 requires, each naming its replacement and its removal version, or `DEPRECATIONS.md` and the stable-API page record that the requirement was withdrawn and why — with no third state where the document promises a deprecation timeline the tree cannot start.
  3. `cargo test --workspace --doc` runs `paladin-ports` instead of excluding it, and the port traits' rustdoc examples compile — restoring executing documentation to the ~25 traits that are the framework's primary integration contract.
  4. A downstream project depending on `paladin` compiles no CLI crates: `cargo tree --lib --no-default-features` shows none of `structopt`, `colored` or `comfy-table`, and the library-only build is genuinely library-only rather than 5-of-8 isolated.
  5. `grep -rn 'pub struct TokenUsage' crates src` returns exactly one result, and token figures no longer need conversion when they cross the battalion/ports boundary.

**Plans**: 9 plans

Plans:

**Wave 1** *(fully parallel — zero file overlap)*

- [x] 08-01-PLAN.md — DEBT-05: extend the canonical `TokenUsage`, then collapse both duplicates into `pub use` re-exports (blocking `checkpoint:decision`, D-18 rated one-way)
- [x] 08-02-PLAN.md — DEBT-01 tooling: five stale path literals, baseline regeneration, the guard proven in both directions, and `check-deprecations.sh` made able to fail
- [x] 08-04-PLAN.md — ADR-0022 (deprecation withdrawal) and ADR-0023 (CLI dependency isolation), authored before the code they authorise
- [x] 08-05-PLAN.md — DEBT-01 records: the five `.project/` requirement-text sources annotated and the five REQUIREMENTS.md traceability rows corrected

**Wave 2** *(blocked on Wave 1)*

- [x] 08-03-PLAN.md — DEBT-03: remove `[lib] doctest = false` and `ci.yml`'s `--exclude paladin-ports` in one commit, and prove the doctests run (depends on 08-02 for `ci.yml` file serialization only — zero line contention)
- [x] 08-06-PLAN.md — DEBT-02: the three-way reconciliation of `DEPRECATIONS.md`, `stable-api.md` and the tree behind ADR-0022
- [x] 08-07-PLAN.md — DEBT-04 core: `structopt`→clap v4, `required-features = ["cli"]` on the `paladin` binary, `paladin-herald`'s first `[features]` section, and all six Herald construction sites gated (both root-`Cargo.toml` halves in one plan)

**Wave 3** *(blocked on Wave 2)*

- [x] 08-08-PLAN.md — DEBT-04 downstream: both Dockerfiles, the inverted `feature-flags.yml` step, the deployment page, two `CHANGELOG.md` entries, and criterion 4 proved by command into ADR-0023

**Wave 4** *(blocked on Wave 3)*

- [x] 08-09-PLAN.md — Close-out: five ledger rows amended in place, DEBT checkboxes flipped behind evidence, `PROMOTION.md` → 0024, PROJECT.md Key Decisions, `COVERAGE.md`, the ADR-0006 84% floor re-check, and a blocking human seal on the three shipped-surface changes

**Cross-cutting constraints:**

- Every plan is subject to the CLAUDE.md workspace gate (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and to ADR-0006's 84% workspace line-coverage floor. Prefer `--offline`: `crates.io` returns HTTP 403 in this environment while the tree builds and tests offline.
- Prohibited across the whole phase: `ci.yml:148/393/792`'s `actions-rs/toolchain@v1` (Phase 15 / PIPE-04); converging `VisionTokenUsage` (out of ADR-0016's five); auditing the 87 pre-existing `ignore`/`no_run`/`text` fences in `paladin-ports` (Phase 16 / DOCS-03); deciding the `cargo doc --workspace --no-deps` warning bar (Phase 10 / HARD-07); retiring `src/main.rs` (ADR-0019 recorded its purpose).
- CI is knowingly red between 08-07 and 08-08: `feature-flags.yml:144`'s step "Verify paladin binary builds without cli feature" inverts the moment the binary gate lands, and 08-08 repairs it. Do not merge the phase to a protected branch between those two plans.

**Milestone 7-8 close-out — Phases 9-11 (not started)**

### Phase 9: Release & Security Gate Integrity

**Goal**: The security, licensing and release gates this project believes it already has actually hold — one advisory exception set instead of four, a licence the manifests declare, and a published crate family that passes its own release criteria.
**Depends on**: Nothing hard, and it should not wait. **This phase carries the only dated item in the entire corpus** — a formal RustSec risk acceptance whose review/expiry target is 2026-09-30, roughly two months out. Two couplings run forward: SEC-01 cannot be honestly reconciled until HARD-06 establishes whether `pdf-extract` is reachable at all, and Phase 12 (SUPPLY-01, SUPPLY-02) carries the concrete CI deletion and the corrected governance scope.
**Requirements**: SEC-01, SEC-02, SEC-03, SEC-04, SEC-05
**Success Criteria** (what must be TRUE):

  1. Asking "which RustSec advisories does this project suppress, and why?" returns **one** answer with one owner, instead of four different answers from `rustsec-remediation-plan.md` (2), `.cargo/audit.toml` (5), `deny.toml` (15) and `ci.yml:406` (2 inline, alongside a second bare-`cargo audit` job at `ci.yml:77`).
  2. Every suppressed advisory carries the governance the project's own acceptance criteria demand — owner, expiry date, affected scope, compensating control — closing the thirteen `deny.toml` entries that today carry only an inline comment, and the 2026-09-30 acceptance is renewed with a new date, closed, or replaced before it lapses.
  3. `cargo audit` behaves identically locally and in CI: `make audit`, `ci.yml:77` and `ci.yml:406` cannot pass different advisory sets, because there is one configuration rather than three.
  4. Asking "what licence is this project?" returns one answer that the root package, all ten library crates and `deny.toml` agree on — replacing today's split between a signed `MIT OR Apache-2.0` policy with a named approver and the `license = "MIT"` the manifests actually declare, on which a 551-package sign-off rests.
  5. `crates/paladin-herald/CHANGELOG.md` exists (or its exemption is recorded), `Dockerfile.chef`'s planner stage covers every crate manifest by a mechanism that cannot go stale on the next crate, and a crates.io name collision is detectable before a release cycle rather than at dry-run time — the three release-gate criteria a published crate family currently fails.

**Plans**: 7 plans in 4 waves

Plans:
**Wave 1**

- [x] 09-01-PLAN.md — Tracer: herald CHANGELOG.md + `scripts/check-changelogs.sh` end-to-end through Makefile and a required CI context (SEC-04)
- [x] 09-02-PLAN.md — `SECURITY-EXCEPTIONS.md` register (ten rows, eleven governance fields) + ADR-0024 (SEC-01)
- [x] 09-03-PLAN.md — Delete `Dockerfile.chef`'s nine-manifest planner enumeration + ADR-0027 (SEC-05)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 09-04-PLAN.md — `.crate-names.txt` + bidirectional name guard, wired to make and CI, + ADR-0026 (SEC-03)
- [x] 09-05-PLAN.md — Licence posture: blocking `checkpoint:decision`, then the selected branch across eleven manifests and every other site, + ADR-0025 (SEC-02)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 09-06-PLAN.md — Reconcile `deny.toml`/`.cargo/audit.toml`, land `scripts/check-advisory-register.sh`, delete the duplicate CI audit job (SEC-01)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 09-07-PLAN.md — Close-out: REQUIREMENTS.md evidence flips, source corrections, PROMOTION.md → 0028, Phase 10/12 hand-offs, phase gate (all five)

### Phase 10: Milestone 7-8 Ground Truth & Recorded Account

**Goal**: A developer can open `.planning/` and get a truthful account of the two milestones that took this workspace to a published crate family and then cleaned up after it — which of the 86 requirements the tree satisfies, which 14 must never be implemented as written, which document actually describes what Milestone 8 did, and what the three unresolved architecture questions are.
**Depends on**: Nothing hard. Phase 10 is independent of Phases 1-9; see the coupling notes under Progress. Running it before Phase 9 is cheaper than running it after, because HARD-06 feeds SEC-01.
**Requirements**: HARD-01, HARD-02, HARD-03, HARD-04, HARD-05, HARD-06, HARD-07
**Success Criteria** (what must be TRUE):

  1. A developer can look up any of the 86 Milestone 7-8 requirement IDs and see a `file:line`-cited verdict — shipped, relocated, superseded by outcome, deferred with a register, or genuinely outstanding — and the ~~fourteen~~ **Corrected (dated 2026-08-08, HARD-01):** thirteen "superseded by outcome" entries (see `.planning/ledgers/milestone-07-08.md`'s summary table, built by counting the table at `intel/code-verification.md:365-381` directly — `sed -n '365,381p' .planning/intel/code-verification.md | grep -c '^|'` → 15 lines = 1 header + 1 separator + 13 data rows) are unmissable, because implementing any of them as written would undo shipped work.
  2. Asking "what happened in Milestone 8?" returns the 2026-06-04 reconciliation rather than the Epic 1 audit or the Epic 3 disposition record — with the reason recorded (~4,400 LOC of orphaned uncompiled duplicates described as "active bridges that stay"), the reproducible verification method preserved, and the three in-execution corrections intact so nobody re-deletes `paladin_registry.rs` or the sqlite repositories on the strength of the original audit.
  3. Nobody plans Milestone 8 Epic 3 or Epic 6 as outstanding work: Epic 3 is complete in substance rather than punted to Milestone 9, Epic 6 is complete despite being recorded "not verified", and `paladin-herald` exists in the tree — which is why the earlier "9 crates" figure was wrong and why the "no new crates" non-goal is recorded as overridden for herald and still holding for `paladin-ml`.
  4. Reading the version record teaches history, not current state: `v0.1.0-rc.1` at commit `a9530fc` with all ten crates published at `0.1.0` and a GO sign-off is dated and closed, and REL-01 converges the branch/`Cargo.toml`/tag disagreement without adopting any rc.1 figure.
  5. The extracted-crate dependency rule reads the same way as the tree behaves — either "never" with `paladin-content → paladin-llm` fixed, or "never, except behind an optional feature" with the rule restated — so the invariant that keeps extraction from re-creating its own coupling is enforceable rather than merely asserted.
  6. Asking "is PDF extraction supported?" returns one answer, and it is consistent with the advisory suppression that assumes `pdf-extract` is in the dependency graph — closing a contradiction where `pdf = []` gates nothing, `content-processing` omits it, and `.cargo/audit.toml` says otherwise.
  7. `cargo doc --workspace --no-deps` has one bar, applied consistently, rather than zero-warnings in Milestone 7 and warnings-acceptable in Milestone 8 on the same command — and it is settled together with the `paladin-ports` doctest exclusion that has been deferred to an unwritten "Task 7.0" since run 3.

**Plans**: 11 plans in 4 waves

Plans:
**Wave 1**

- [x] 10-01-PLAN.md — Ledger scaffold (tracer): head notes, seven-class legend, 13-row superseded-by-outcome summary table, 86 row stubs, M8 Epic 4 derived end-to-end; REQUIREMENTS.md pointer; stale-figure corrections in HARD-01/03/05/07 and ROADMAP criterion 1
- [x] 10-02-PLAN.md — ADR-0028 (reconciliation authoritative, orphan test, three do-not-re-delete corrections, Epic 3/6 completeness, non-goal split) + three M8 source annotations
- [x] 10-03-PLAN.md — ADR-0029 (version trajectory, ORCH-05-extensible table) + ADR-0030 (fourth numbering collision, citing 0010/0014) + M7 overview annotation
- [x] 10-04-PLAN.md — Blocking `checkpoint:decision` on the three flagged branches (D-15, D-18, the `cargo doc` boundary), then ADR-0031 (default-build dependency invariant) + M7 Epic 1 §6.1/Goal 2 annotations

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 10-05-PLAN.md — ADR-0032 (PDF unconditional; inert feature) + the manifest, CHANGELOG and `.cargo/audit.toml` changes + M7 Epic 1 §4.4.1/§4.4.6 annotations
- [x] 10-06-PLAN.md — ADR-0033 (one `cargo doc` bar, the measured 20-warning debt with an owner, DEBT-03 discharged, doctest posture) + `Makefile` release-gate fix + M8 Epic 5 FR-19 and Epic 4 summary annotations

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 10-07-PLAN.md — Ledger fan-out: M7 Epic 1 (12) + Epic 2 (13) = 25 cited rows
- [x] 10-08-PLAN.md — Ledger fan-out: M7 Epic 3 (10) + Epic 4 (12) = 22 cited rows, including the six Phase-9-closed
- [x] 10-09-PLAN.md — Ledger fan-out: M8 Epics 1-3 = 14 cited rows with the do-not-re-delete markers
- [x] 10-10-PLAN.md — Ledger fan-out: M8 Epics 5-7 + 5 cross-milestone = 21 cited rows

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 10-11-PLAN.md — Close-out: seven evidence-backed checkbox flips, traceability rows, PROMOTION.md → 0034, PROJECT.md decisions, four forward hand-off blocks, ledger close-out amendment, phase gate

**Cross-cutting constraints:**

- This plan writes only its own two epic sections and replaces Verdict and Evidence cells in place, so `grep -c '^| REQ-'` still reads 86 and the ledger's added and deleted line counts are equal
- Two requirements citing the same artefact keep separate rows and separate verdicts; the `REQ-*` ID is the primary key (D-00f)

**Dated completion note, 2026-08-08 (plan 10-11):** Phase 10 is closed. All seven HARD requirements
are `[x]` in `REQUIREMENTS.md`, each behind a dated closure note naming its artefact and a re-run
command or `file:line`. `.planning/ledgers/milestone-07-08.md` carries all 86 `REQ-*` rows across
12 sections with zero pending stubs and zero dangling `ADR-*` citations, plus an appended
`## Phase 10 close-out amendments (2026-08-08)` section recording the per-class verdict tally, the
12-vs-13 supersession-count reconciliation, and the three Manual-Only dispositions. Six new ADRs are
indexed in `PROMOTION.md` (next free number **0034**): ADR-0028 (M8 reconciliation authoritative),
ADR-0029 (version trajectory), ADR-0030 (M7 self-numbering), ADR-0031 (extracted-crate dependency
rule), ADR-0032 (PDF extraction capability), ADR-0033 (`cargo doc` warning bar). Three config
changes executed the phase's D-23 boundary in full: `crates/paladin-content/Cargo.toml`'s inert
`pdf = []` feature deleted (plan 10-05), `.cargo/audit.toml`'s `RUSTSEC-2026-0187` reachability
comment corrected (plan 10-05), `Makefile`'s stale `--exclude paladin-ports` release-gate flag
deleted (plan 10-06). `git diff --name-only 6a6f175..HEAD -- '*.rs' | wc -l` → `0` across the whole
phase — D-23's zero-`.rs`-change boundary held and is checkable by that command rather than only
asserted. Four forward hand-off blocks are written in `REQUIREMENTS.md` for Phase 11 (FACADE-02,
FACADE-03(b)), Phase 12 (SUPPLY-02/03, including the `scraper`/`rss`/`tiktoken-rs` dead-dependency
finding named to Phase 15), and Phase 13 (ORCH-05). Criterion 1's plan-10-01 correction (thirteen,
not fourteen, superseded-by-outcome rows) is left in place, uncorrected further. The goal line,
requirement list, and success criteria above are unchanged by this note.

### Phase 11: Facade Residue & Deferred Register Disposition

**Goal**: Everything Milestone 8 deliberately left behind has a decision rather than a rating — the five deferred items, the two removed features and their reintroduction conditions — and the Milestone 9 candidate list is triaged so nobody re-plans relocations that already happened.
**Depends on**: Phase 10 (HARD-05 decides whether leaf-to-leaf crate edges are permitted, which determines D2/D3/D4's relocation targets in FACADE-02) and, more loosely, Phase 7 (ARCH-04's facade re-export policy decides D1). FACADE-01, FACADE-03 and FACADE-04 are independent and can start immediately. **FACADE-02's D2 is coupled forward to DEFER-02 in Phase 15.**
**Requirements**: FACADE-01, FACADE-02, FACADE-03, FACADE-04
**Success Criteria** (what must be TRUE):

  1. `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` returns only occurrences that are deliberate stdout — each of the 17 remaining across 6 files either converted to `log::*` or annotated with its reason, closing the register's own quick win. CLI output is untouched and stays that way. **Corrected by Phase 11, dated 2026-08-08 (plan 11-01):** No conversion is possible or required. `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` returns exactly **17** occurrences across exactly **6** files, and every one of the 17 is a `///` or `//!` doc-comment line inside a fenced code block — a rustdoc example, not runtime library stdout. The same grep filtered to non-doc-comment lines (`grep -v '///' | grep -v '//!'`) returns **0**. The closing condition for this criterion is a recorded per-occurrence disposition — see `.planning/registers/facade-01-rustdoc-stdout-disposition.md`, which names all 17 as deliberate rustdoc-example stdout — rather than a `log::*` conversion, which would degrade the documentation these lines exist to illustrate. CLI output remains untouched, unaffected by this correction. Original criterion text retained above.
  2. Each of D1 through D4 carries a decision with an owner — do it, defer it with a stated trigger, or withdraw it — instead of an effort/risk rating and a recommendation. Nothing in that set is planned twice: D2's `user_service` half is already a run-3 v2 item, the reconciliation already established that no user-service split was needed for the controller case, and Deferred-QA Epic 28 plans to *test* the same file D2 plans to *split*.
  3. Someone asking "why can I not run `paladin user register`?" finds the answer in `.planning/` — the surface was 1,065 LOC that was declared but never dispatched, the backend is intact, and reintroduction is re-wiring recoverable verbatim from a named commit — rather than concluding it was lost.
  4. The condition on returning ML support survives outside a single DOC: any future TensorFlow adapter goes into a dedicated `paladin-ml` leaf crate with the feature flag on that crate, never back into the facade, and `MlPort` stays in the workspace so the integration point does not move.
  5. Every row of the Milestone 9 candidate list is marked done, not-a-candidate, or still-open, and `paladin-arsenal` and `paladin-sanctum` are either confirmed as real future crates or recorded as artefacts of a table that contradicts its own governing PRD — so nobody plans relocations the reconciliation already executed against a milestone that is 100% complete.

**Plans:** 5/5 plans complete

Plans:
**Wave 1**

- [x] 11-01-PLAN.md — FACADE-01: record all 17 D5 occurrences as deliberate rustdoc stdout, correct the framing in `deferred-items.md` and ROADMAP criterion 1, and gate the ADR allocation (tracer, wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 11-02-PLAN.md — FACADE-02: ADR-0034 giving D1–D4 verdicts with owners and triggers, and dated corrections on the four clauses (wave 2)
- [x] 11-03-PLAN.md — FACADE-03: the `.planning/` register for both removed features plus ADR-0035 holding the `paladin-ml` leaf-crate placement condition (wave 2)
- [x] 11-04-PLAN.md — FACADE-04: the 20-row Milestone 9 candidate triage and the `paladin-arsenal` / `paladin-sanctum` artefact finding (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 11-05-PLAN.md — close-out: five ledger rows amended, two REQUIREMENTS.md source corrections, `PROMOTION.md` and `PROJECT.md` updated last (wave 3)

**Cross-cutting constraints:**

- No Rust source file is modified by this plan (D-13)

**Milestone 9-12 + Deferred-QA close-out — Phases 12-16**

### Phase 12: Supply-Chain Gate Integrity

**Goal**: The supply-chain gates this project runs on every push give one verdict, and every suppression behind them has a name and a date attached to it.
**Depends on**: Nothing hard, and **it is the cheapest phase in this roadmap**. SUPPLY-01 is an 18-line deletion. Two couplings: it narrows and corrects **SEC-01** in Phase 9 (which owns the whole exception set and the 2026-09-30 disposition), and **HARD-06** in Phase 10 decides whether `pdf-extract` is reachable and therefore whether one of the three contested suppressions is needed at all. Running Phase 10 first saves SUPPLY-02 from guessing; running Phase 12 first makes Phase 9 smaller.
**Requirements**: SUPPLY-01, SUPPLY-02, SUPPLY-03
**Success Criteria** (what must be TRUE):

  1. `ci.yml` invokes `cargo audit` exactly once, no job passes `--ignore` on the command line, and no two jobs share a display name — closing a state where **two jobs both called "Security Audit" are configured to reach different verdicts on the same `Cargo.lock`**, one reading five advisories from `.cargo/audit.toml` and one hardcoding two. **Satisfied by Phase 9, dated 2026-08-08 (plan 09-07):** plan 09-06 (commit `cb75b2b`) deleted the duplicate `security:` job; `grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1`, and the surviving `security-audit:` job is the only one carrying the `"Security Audit"` display name. Original criterion text retained above.
  2. Milestone 10 Epic 2's own success metric is true: "`audit.toml` and `deny.toml` are the only places policy/exceptions are defined; no inline advisory-ignore flags remain in CI" — a criterion that is false today on a milestone recorded 100% complete with zero open checkboxes. **Satisfied by Phase 9, dated 2026-08-08 (plan 09-07):** the deleted job was the only inline `--ignore` user in any workflow; zero remain. Original criterion text retained above.
  3. Asking "who owns this suppression and when is it reviewed?" returns an answer for **all fifteen** `deny.toml` entries, not two — the thirteen that today carry documented reasoning but no owner and no date have both, and the exception schema itself requires them so the next entry cannot be added without one. **Corrected and satisfied by Phase 9, dated 2026-08-08 (plan 09-07):** the live count was never fifteen — `deny.toml` held **fourteen** entries pre-Phase-9 (nine unmaintained, not ten; `RUSTSEC-2025-0121`/`gcc` was already gone with no record of removal), and Phase 8's clap v4 migration had already taken four further entries dead (`structopt`, `ansi_term`, `atty`, `proc-macro-error`, all confirmed absent from `Cargo.lock`). Plan 09-06 deleted those four; the live set is **ten**, and `SECURITY-EXCEPTIONS.md` (plan 09-02) gives every one of the ten an owner (`DF3NDR`) and a review date (`2026-12-31`) — not thirteen backfilled, since only ten suppressions are live. Original criterion text retained above.
  4. The three 2026 vulnerability advisories that no ingested document authorises are either ratified by a recorded decision or removed, so the suppressed set matches what a document sanctions rather than exceeding it by three. **Satisfied by Phase 9, dated 2026-08-08 (plan 09-07):** ADR-0024 decision 3 ratifies all three (`RUSTSEC-2026-0187`, `-0194`, `-0195`), each with a named, concrete compensating control in `SECURITY-EXCEPTIONS.md`. Original criterion text retained above.
  5. The 2026-09-30 acceptance has a disposition — renewed with a new date, closed, or replaced — and a decision is on record about whether the two supply-chain ADR candidates are promoted, so the invariant either gates future work or is knowingly left advisory. **Half satisfied by Phase 9, dated 2026-08-08 (plan 09-07):** the 2026-09-30 acceptance is renewed to per-advisory `2026-12-31` review dates (ADR-0024). The ADR-promotion half of this criterion is **not** satisfied by Phase 9 — the audit-suppression single-source invariant candidate (`Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8) remains **SUPPLY-03's** subject, unchanged. Original criterion text retained above.

**Dated closure note, 2026-08-08 (Phase 9 plan 09-07), per D-07 (`09-CONTEXT.md`):** SUPPLY-01's
concrete `ci.yml` deletion and all three of SUPPLY-02's clauses (ratify-or-remove the three 2026
vulnerability ignores; extend the FR-3 schema with owner and expiry; backfill the live
suppressions) were executed by Phase 9's plans 09-02 and 09-06 — commits `a587e5a`, `7ee741c`,
`6513cb7`, `9cef391`, `cb75b2b`. Phase 12 inherits SUPPLY-01 and SUPPLY-02 as **closed items to
verify**, not work to re-plan: confirming the required status check still resolves on the first
real CI run after the deletion, and that `cargo audit`/`cargo deny check` actually pass against the
reconciled configuration (neither tool is installable in Phase 9's sandboxed environment — `crates.io`
returns HTTP 403). **What remains for Phase 12 to actually plan is SUPPLY-03 alone** — the
ADR-promotion decision for the two supply-chain candidates. The goal line, requirement list, and
every other phase section in this document are unchanged by this note.

(**Corrected by Phase 12 (plan 12-01), dated 2026-08-09, citing this plan's own re-run:** the
HTTP-403 blocker cited two paragraphs above has lifted — `cargo-audit` and `cargo-deny` are both on
`PATH` in this environment as of 2026-08-09, and both tools plus
`./scripts/check-advisory-register.sh` were re-run and exited `0` on 2026-08-09 by plan 12-01,
transcripts recorded in `REQUIREMENTS.md`'s SUPPLY-01/SUPPLY-02 "Verified by Phase 12" blocks.
Phases 9 and 10 were both correct at the time they wrote this caveat; it was not permanent. The
CI-run-observation half of this note — confirming the required status check still resolves on the
first real CI run — is **not** bannered here: it is still true and remains recorded pending, per
D-07.)

**Plans:** 4/4 plans complete

Plans:
**Wave 1**

- [x] 12-01-PLAN.md — tracer: re-run all three supply-chain gates, close SUPPLY-01 and SUPPLY-02 on verbatim transcripts, record the CI-run observation as pending and the unapplied-rulesets finding, sweep the stale `ci.yml:389-406` citations, and gate the ADR allocation at a blocking decision checkpoint (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 12-02-PLAN.md — SUPPLY-03 / D-08: the offline `check-workflow-suppressions.sh` regression guard, proven with a positive/negative pair and wired into `make check-gates` and the `cargo-deny:` CI job (wave 2)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 12-03-PLAN.md — SUPPLY-03: ADR-0036 promoting the audit-suppression single-source topology invariant with a `conforms` verdict, plus dated corrections on the four passages claiming the promotion is impossible (wave 3)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 12-04-PLAN.md — close-out: the Phase 13 / ORCH-01 hand-off block, three closed requirement rows, the `PROJECT.md` Key Decisions row, and `PROMOTION.md` advanced to 0037 last (wave 4)

**Cross-cutting constraints:**

- No Rust source file is modified by any plan in this phase
- ADR-0024's suppression set, schema, owner and review dates are not touched (D-00i)
- `.planning/ledgers/milestone-09-12.md` is not created — it is ORCH-01 / Phase 13's deliverable (D-09)
- Nothing is applied to live GitHub repository administration state (D-10)

### Phase 13: Milestone 9-12 Ground Truth & Recorded Account

**Goal**: A developer can open `.planning/` and get a truthful account of the four milestones that finished, hardened, documented and exposed this framework — what the 120 requirements actually delivered, which paths and routes in them are historical, and what the two seams Milestone 12 left as defaults have been decided to be.
**Depends on**: Nothing hard. Phase 13 is independent of Phases 1-12. One answer feeds backwards: **ORCH-05** completes the version trajectory HARD-03 (Phase 10) starts and REL-01 (Phase 4) converges, so whichever of the three runs first records the answer and the others apply it.
**Requirements**: ORCH-01, ORCH-02, ORCH-03, ORCH-04, ORCH-05
**Success Criteria** (what must be TRUE):

  1. A developer can look up any of the 120 Milestone 9-12 requirement IDs and see a `file:line`-cited verdict, and the ledger states plainly that the whole Milestone 9 orchestrator subsystem, the whole Milestone 10 tooling set, the mdbook and the whole Milestone 12 web API ship — so none of it is re-planned.
  2. Asking "is Milestone 10 done?" returns the honest answer rather than the checkbox answer: every deliverable ships, the count is 100%, **and one of its own acceptance criteria is false** — a verdict class this corpus had not produced before, and the reason Phase 12 exists.
  3. Asking "are Milestone 12's three open items work?" returns *no, they are feature-branch scaffolding*, and asking the same of project-management's one returns *no, it is a formatting example inside a template* — with the five-run pattern of checkbox unreliability written down once instead of rediscovered a sixth time.
  4. Someone implementing a run-5 requirement literally cannot write to a path that does not exist: the four stale module and document paths are corrected at source, and the agent API's route surface has one answer confirmed against the committed `openapi.json` rather than four Epics naming paths a fifth relocates.
  5. Both seams Milestone 12 recorded as defaults are decisions with reasoning: where `AgentProvisioner` lives — which determines whether the queue/worker and sidecar topologies can reuse it or must duplicate it — and whether Garrison and Arsenal for HTTP-served agents are planned scope or a permanent property of the topology, stated in the decision matrix that routes readers between topologies rather than in a single non-goal line.
  6. The version story runs unbroken from `v0.1.0-rc.1` through v0.3.0, v0.4.0, v0.5.0 and v0.6.0 to the tree, so REL-01 converges with the whole trajectory in view.

**Plans**: 13 plans

Plans:

- [x] 13-01-PLAN.md — ledger scaffold (tracer): head notes, verdict legend and seven-class mapping, both highlight tables, contention table, 120 row stubs, and Milestone 9 Epic 1 derived end-to-end (wave 1)
- [x] 13-02-PLAN.md — ledger fan-out: Milestone 9 Epics 2-6, 19 rows, the densest bare-`Verify` cluster (wave 2)
- [x] 13-03-PLAN.md — ledger fan-out: Milestone 10 Epics 1-5, 23 rows, the both-halves verdict and the SUPPLY provenance (wave 2)
- [x] 13-04-PLAN.md — ledger fan-out: Milestone 11, 20 rows, the 26-open verdict carried to Phase 16 (wave 2)
- [x] 13-05-PLAN.md — ledger fan-out: Milestone 12 Epics 1-4, 19 rows, the `/v1` route surface and the provisioner facts (wave 2)
- [x] 13-06-PLAN.md — ledger fan-out: Milestone 12 Epics 5-7, 15 rows, the WEB-01 and WEB-02 hand-off inputs (wave 2)
- [x] 13-07-PLAN.md — ledger fan-out: Deferred-QA Epics 25-29 and project-management, 18 rows, the `Verified open` block (wave 2)
- [x] 13-08-PLAN.md — ADR-0037 (agent route surface `/v1`), the `sidecar.md` correction, and seven Milestone 12 route annotations (wave 3)
- [x] 13-09-PLAN.md — blocking checkpoint, then ADR-0038 (`AgentProvisioner` placement) and ADR-0039 (Garrison/Arsenal on the HTTP topology) with the two documentation corrections (wave 3)
- [x] 13-10-PLAN.md — REQUIREMENTS.md source corrections: ORCH-01's arithmetic, ORCH-05's version figures, PIPE-01's job list, and the ledger pointer (wave 3)
- [x] 13-11-PLAN.md — `.project/` and `.planning/intel/` source corrections for the four relocations and the three superseded verification statements (wave 3)
- [x] 13-12-PLAN.md — ORCH-05: four ADR-0029 trajectory rows and the ADR-0030 citation with the provenance-key confirmation (wave 3)
- [x] 13-13-PLAN.md — close-out: ledger amendment and phase gate, five closed requirements, three forward hand-offs, PROMOTION.md to 0040 (wave 4)

**Cross-cutting constraints:**

- No Rust source file is modified by any plan in this phase; the close-out asserts it with a `git diff --name-only <base>..HEAD -- '*.rs' | wc -l` → `0` (D-19)
- The entire in-tree edit surface is three files under `docs/src/deployment-topologies/`: `sidecar.md` (plan 13-08), `http-service-host.md` and `overview.md` (plan 13-09)
- Exactly three ADRs are authored — 0037, 0038, 0039 — and `PROMOTION.md` advances to 0040; ORCH-01, ORCH-02 and ORCH-05 get no ADR (D-20)
- `.planning/ledgers/milestone-09-12.md` is written by plan 13-01 and appended by plans 13-02 through 13-07 on disjoint section ranges; `.planning/REQUIREMENTS.md` is touched in waves 1, 3 and 4 only, never concurrently

### Phase 14: API Contract Truthfulness

**Goal**: Every capability this project advertises through an interface is one it actually has — so a developer reading the auth contract, deploying the Kubernetes manifests, or branching on a provider capability flag gets the behaviour the interface promised.
**Depends on**: Nothing hard. WEB-03 is independent, small and can be done today. WEB-01's answer shapes WEB-02: if the mechanism stays opaque tokens, the shared store is an adapter swap the port was explicitly designed to permit; if it becomes JWT, the question changes shape entirely. WEB-04 is a scope decision that WEB-03 does not wait for.
**Requirements**: WEB-01, WEB-02, WEB-03, WEB-04
**Success Criteria** (what must be TRUE):

  1. A developer reading the agent API's authentication documentation, its config keys and its OpenAPI security scheme sees the mechanism the code actually runs — closing a state where the API is **documented as JWT throughout and implemented as opaque in-process tokens**, with no `jsonwebtoken` dependency anywhere in the workspace and an open question that is unanswerable because the shipped adapter has no signing secret and no algorithm.
  2. A token issued against one server instance either verifies against another, or the deployment artefacts and documentation say it will not — so scaling the shipped Kubernetes Deployment past one replica cannot silently produce authentication failures that depend on which pod a request lands on.
  3. `ProviderCapabilities` reports tool-calling support that matches what the OpenAI, Anthropic and DeepSeek adapters actually do, with a test asserting the correspondence — so a consumer branching on the flag stops getting the wrong answer, whether or not tool calling is ever built.
  4. Asking "does Paladin support LLM tool calling?" returns one recorded answer with reasoning — built with the phased plan the PRD names, or withdrawn given that Arsenal/MCP already provides tool execution — rather than a fourth appearance as a deferred register entry.

**Plans**: 8 plans in 5 waves

Plans:
**Wave 1** *(parallel — no shared files)*

- [x] 14-01-PLAN.md — Tracer: the WEB-01 token-vocabulary rename end-to-end through config, binary wiring, web middleware and the published contract, with both machine baselines regenerated; plus the prose/YAML sweep and the BREAKING changelog entries. *Leads with a one-way `checkpoint:decision` ratifying the replacement identifiers.*
- [x] 14-02-PLAN.md — WEB-03's last asymmetric capability flag flipped and the correspondence test extended to pin both flags; plus D-15a's sidecar route fix and its `API_V1_PREFIX` drift guard (closes T-13-20 / AR-13-01)
- [x] 14-03-PLAN.md — D-13's tool-call reachability statement: rustdoc on the capability type and the response field that gates the tool branch, plus the four documentation pages that imply otherwise

**Wave 2** *(blocked on Wave 1)*

- [x] 14-04-PLAN.md — WEB-02: the unconditional startup warning when the in-process token store is wired, the fail-closed test `REQ-fail-closed-auth-posture` never had, and the single-replica limitation stated in the ConfigMap, the Kubernetes README and the topology page — with the shipped Deployment left unchanged (D-06)
- [x] 14-06-PLAN.md — ADR-0042 (WEB-04: LLM-native tool calling deferred with a named trigger and an owner) and the dated, additive correction banner on the Deferred-QA Epic 27 source

**Wave 3** *(blocked on Wave 2)*

- [x] 14-05-PLAN.md — ADR-0040 (WEB-01: opaque server-issued bearer tokens; M12 Epic 5 OQ-4 dissolved) and ADR-0041 (WEB-02: verification scope, the deliberate deviation from the literal done-when, and the deferred shared store)

**Wave 4** *(blocked on Wave 3)*

- [x] 14-07-PLAN.md — Close the record: six ledger rows amended in place with carried-forward `--auto` provenance, WEB-01…04 checked with `Complete` traceability, WEB-02's manifest citation corrected at source, and the ADR index advanced

**Wave 5** *(blocked on Wave 4 — the bump must land last, D-18)*

- [x] 14-08-PLAN.md — Lockstep 0.8.0 bump across all twelve manifests and the dated changelog section, then the **second** OpenAPI baseline regeneration and the drift guard proven in checking mode

**Cross-cutting constraints:**

- The committed `crates/paladin-web/openapi.json` baseline moves **twice** — once for the security-scheme rename (14-01) and once for the version bump (14-08), because the document's advertised version is sourced from the crate version. A phase that regenerates only once leaves the drift guard red (D-18).
- Both machine baselines are regenerated in the same commit that moves them (D-20).
- `k8s/server/deployment.yaml` and the root `k8s/deployment.yaml` placeholder are byte-identical at phase end (D-06, D-08).
- No git tag is created: publishing is tag-triggered and irreversible (D-17).

### Phase 15: Coverage & CI Quality Gates

**Goal**: The project measures its own quality on every push instead of asserting it — CLI snapshots and benchmarks compile in CI, coverage is collected, gated and reproducible locally, and the two modules deliberately excluded from Milestone 3's coverage work are no longer blind spots.
**Depends on**: Nothing hard, but **the two halves are strictly sequential and their own register says so**. Epic 25 (PIPE-01 … PIPE-05) comes first because it "establishes quality gates that validate all subsequent work"; the coverage register (DEFER-01 … DEFER-03) comes second because it is what those gates then measure. Two couplings reach outward: **PIPE-02's threshold must land on the number RECON-07 (Phase 1) and VERIFY-05 (Phase 5) record**, or record why the CI gate differs; and **DEFER-02 is coupled to FACADE-02's D2 in Phase 11** — one splits `user_service.rs`, the other tests it, and doing them independently means doing the work twice.
**Requirements**: PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, DEFER-01, DEFER-02, DEFER-03
**Success Criteria** (what must be TRUE):

  1. A pull request that breaks a CLI snapshot or stops a benchmark compiling fails CI — closing a state where 43 snapshot tests never run in CI at all, and where benchmark regression *signalling* ships while the compile check that should precede it does not.
     **Corrected (dated 2026-08-13, Phase 15, plan 15-10):** "43 snapshot tests" is superseded — the
     real count, re-measured at execution time (`ls tests/cli/snapshots | wc -l`), is **86** snapshot
     files backed by **97** `#[test]` functions across seven files. The criterion's substance holds
     unchanged: a broken snapshot or a non-compiling benchmark now fails CI (`cli-tests`,
     `bench-check` jobs, plan 15-01), only the "43" figure was stale. Original text retained above.

  2. A pull request that drops coverage below the recorded gate fails CI, and a developer can reproduce that number locally with `make coverage` — replacing today's split where the only coverage collection lives in an integration-only workflow, no gate exists, and the `Makefile` has no coverage target at all.
  3. The coverage threshold has one number with one rationale, chosen between a 78% hard gate and a phased 70 → 74 → 78 ramp against a measured 76-77% baseline — so the gate is set deliberately rather than by whichever document is read first, and it is consistent with the project-wide coverage answer.
     **Corrected (dated 2026-08-13, Phase 15, plan 15-10):** all three figures in this criterion's
     framing above are superseded. `.planning/decisions/0006-coverage-gate.md`'s Phase 15 amendment
     rejected **both** the 78% hard gate and the 70→74→78 ramp (see its `## Considered Options`),
     and the 76-77% baseline this criterion cites is not what the gate was derived from — the number
     that landed (82%, floored from a fresh 82.39% workspace measurement under
     `--features integration-tests`) comes from a scope neither position contemplated. The earlier
     framing was wrong because it treated the threshold as a choice between two inherited PRD
     positions rather than a fresh measurement under the extended scope ADR-0006 itself calls for.
     See `.planning/decisions/0006-coverage-gate.md`'s `## Phase 15 amendment (2026-08-13)` for the
     number, its derivation, and the rejected alternatives — not restated here. Original framing
     retained above.

  4. `actionlint` reports zero errors across all three workflows and no deprecated action remains — the eight `actions-rs/toolchain@v1`, `actions/cache@v3` and `codecov/codecov-action@v3` references are gone.
  5. A developer writing an async service test reaches for shared, `Send + Sync` mocks that already exist rather than writing another one-off — closing the prerequisite that three registers name and none has built, and that roughly a quarter of the deferred coverage estimate consists of.
  6. `user_service.rs` and the listener orchestrator are covered to the gate, with the `user_service.rs` split-versus-test collision resolved by sequence rather than by whoever schedules first, and with the listener's stale 57.83% baseline re-measured against a module that Milestone 9 has since added tests to.
     **Corrected (dated 2026-08-13, Phase 15, plan 15-10):** "covered to the gate" is ambiguous
     between the workspace-wide CI floor and a module-specific bar; it resolves to the **≥ 80%
     module bar as a phase-acceptance criterion, checked once by plans 15-07/15-09, not a standing
     CI gate** (D-12) — no `--fail-under-lines` is scoped to either file individually.
     `user_service.rs` reached **94.21%**; the listener orchestrator reached **96.90%**. The
     "resolved by sequence" framing is also corrected: the split-versus-test collision was
     **dissolved by ADR-0034**, not sequenced — the split is withdrawn, owned by nobody, so there
     was no sequence to resolve by the time DEFER-02 executed. The listener's stale 57.83% baseline
     was re-measured, with the delta stated: an entry attempt recorded honestly as NOT MEASURED
     (`cargo-llvm-cov` uninstallable in this environment), then a real exit figure of 96.90% via
     ADR-0006's own local raw-`llvm-cov` substitute. Original framing retained above.

**Plans**: 10 plans in 8 waves. The two halves are wave-separated as the register requires: PIPE-01 … PIPE-05 occupy waves 1-4, DEFER-01 … DEFER-03 waves 5-7, and the record closes in wave 8. Waves 6 and 7 each run two plans in parallel (`user_service.rs` and `listener.rs` share no files). Plan 15-03 carries a blocking checkpoint because D-04's two-step landing needs a CI-produced figure that no local environment can generate — Docker is absent.

Plans:
**Wave 1**

- [x] 15-01-PLAN.md — tracer: the `coverage` job (measure-only) and `make coverage`, plus the `cli-tests` and `bench-check` gates and the Makefile targets

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 15-02-PLAN.md — retire every deprecated action, delete the duplicate coverage path, land `.codecov.yml` and an `actionlint` job

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 15-03-PLAN.md — capture the CI-produced figure, amend ADR-0006 in place, arm the gate

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 15-04-PLAN.md — the contributor coverage documentation and the three corrected instruction files

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 15-05-PLAN.md — `src/test_support/`: the shared `Send + Sync` doubles both coverage epics consume

**Wave 6** *(blocked on Wave 5 completion)*

- [x] 15-06-PLAN.md — `user_service.rs`: registration, validation, hashing and the notification-failure path
- [x] 15-08-PLAN.md — `listener.rs`: re-measure the stale baseline, then lifecycle, delivery, filtering and health

**Wave 7** *(blocked on Wave 6 completion)*

- [x] 15-07-PLAN.md — `user_service.rs`: authentication, profile, queries, concurrency, and the module measurement
- [x] 15-09-PLAN.md — `listener.rs`: the concurrency and stress suite, and the exit measurement

**Wave 8** *(blocked on Wave 7 completion)*

- [x] 15-10-PLAN.md — amend the eight requirements, ten ledger rows, ROADMAP criteria and three source documents at source

**Cross-cutting constraints:**

- The production half of `listener.rs` — everything above the `#[cfg(test)]` marker — is byte-identical to its pre-plan state.

### Phase 15.1: Git & CI Governance (INSERTED)

**Goal**: The branching model, the branch protection rules and the CI trigger surface agree with each other and are written down — so a broken workflow cannot survive undetected, a release tag cannot be cut from unmerged work, and a new feature branch is a normal thing to create rather than something CI fails on.
**Requirements**: TBD — minted during `/gsd-discuss-phase 15.1`

  **Amended 2026-08-14 (plan 15.1-10), per D-00f, original text above retained and marked
  superseded:** this phase mints no requirement identifier. The requirements document
  (`.planning/REQUIREMENTS.md`) is corpus-wide, not phase-local; retroactively inventing `REQ-*` or
  similar identifiers for an inserted phase would create entries with no upstream requirement text
  and no consumer to point at them, and the seven success criteria below already carry the
  traceability an identifier would add. This is a recorded answer to the open question the original
  line left pending, not an omission — see `.planning/decisions/0043-github-flow-trunk-and-trigger-surface.md`
  and `.planning/decisions/0044-branch-protection-posture.md`, whose `Downstream Consumers` sections
  are what a later reader traces instead.

**Depends on**: Phase 15 (its `actionlint` job and workflow edits land first). Also consumes the outcome of branch `fix/ci-workflow-health`, which already repaired the eight broken `dtolnay/rust-toolchain@master` references and restored `push:` coverage — **this phase does not redo that work**; it addresses why it went unnoticed for two weeks.
**Success Criteria** (what must be TRUE):

  1. A broken workflow cannot sit undetected for two weeks again — every workflow's trigger surface is deliberate and recorded, closing the state where three of six workflows were `pull_request`-only and *every* workflow filtered `pull_request` to `[main, develop]`, so a branch taking direct pushes and a PR into a release branch both exercised only a fraction of CI.

     **Amended 2026-08-14 (plan 15.1-10), original claim superseded:** the trigger-edit half of
     this criterion was already true when this phase began — `fix/ci-workflow-health` had already
     merged, giving every workflow a deliberate branch list with a comment naming the failure it
     closed (`15.1-CONTEXT.md` finding 1). What this phase actually delivered is the recorded
     policy and the mechanical guard: the six-row trigger-policy register in
     `docs/src/contributing/branching-model.md` and `scripts/check-workflow-triggers.sh`, wired
     into `ci.yml`'s `cargo-deny` job (plan `15.1-09`), and ratified as the decided model by
     ADR-0043 (`.planning/decisions/0043-github-flow-trunk-and-trigger-surface.md`).

  2. `main` and the integration branch are protected with required status checks, replacing today's state where **neither `main` nor `release/v0.7.0` has any protection** and therefore no check can block anything.
  3. `docs/BRANCH_PROTECTION.md` exists and matches reality — `release.yml`'s `verify-tag-source` job already enforces the "main is the source of truth" invariant and points contributors at that document, which **does not exist**.

     **Amended 2026-08-14 (plan 15.1-10), original premise superseded:** the file does not exist,
     and per D-04/D-12 it is not reintroduced. Its *content* already existed — relocated by
     Milestone 11 to `docs/src/appendix/branch-protection.md` (149 lines) before this phase began
     (`15.1-CONTEXT.md` finding 2). This criterion is satisfied by redirecting the three surviving
     stale pointers (`release.yml`'s `verify-tag-source` failure message, two in the `Makefile`'s
     release-branch guard) to the page that actually exists, and bringing that page current with
     the applied protection state (plan `15.1-10`, Task 2).

  4. The branching model is decided and documented, resolving that `release/v0.7.0` is being used as `develop` (a branch that already exists, unused) while `main` sits **909 commits behind with nothing ahead**, and that the branch name says `v0.7.0` while every manifest says **0.8.0**. Covers the long-lived-vs-short-lived release branch question and the version/naming policy.

     **Amended 2026-08-14 (plan 15.1-10), original framing superseded:** the reconciliation was not
     a hard problem — `git merge-base --is-ancestor origin/main origin/release/v0.7.0` returned
     true, a clean, zero-conflict fast-forward with nothing on the trunk the integration branch
     lacked. The commit count above also differs from the one this phase measured: **921** commits
     behind at discovery (`15.1-CONTEXT.md` finding 5), not 909, and the fast-forward actually
     executed moved **994** commits — the 921 from `release/v0.7.0` plus 73 more accumulated by
     this phase's own waves 1-5 before the move (`15.1-07-SUMMARY.md`). It was one command
     (`git push origin HEAD:main`) run behind a blocking checkpoint because it was irreversible, not
     because it was difficult (ADR-0043).

  5. Creating a new feature branch does not break CI — `ci.yml`'s `examples` job runs `cargo build --examples --offline` with **no `restore-keys` cache fallback**, so a fresh branch gets a cache miss and fails with `failed to download <crate> ... --offline was specified`. This currently fails on the first run of every new branch and directly blocks adopting feature-branch flow.

     **Noted 2026-08-14 (plan 15.1-10) — framing understated the defect; not counted as one of the
     four premise corrections above:** the failure was not confined to "the first run of every new
     branch" — it recurred on every run after any `Cargo.lock` change, on any branch, because 25 of
     26 cache blocks carried no `restore-keys` fallback at all (`15.1-CONTEXT.md` finding 9). Fixed
     at the class, not only in the one job that failed loudest: `Swatinem/rust-cache@v2` repo-wide
     plus an explicit `cargo fetch --locked` step ahead of the four `--offline` builds (plans
     `15.1-01`/`15.1-03`).

  6. No CI job is dark — jobs reachable only by `schedule`/`workflow_dispatch` are either exercised on a known cadence or removed, closing the state where `Performance Benchmarks` was `skipped` in every observed run and failed the first time it was actually dispatched.

     **Amended 2026-08-14 (plan 15.1-10), original premise superseded — one dark job named, four
     actually found:** re-verified against the tree, four jobs were non-continuous, not one.
     `End-to-End Tests` and `Publish Dry Run` gated on `push && ref == 'refs/heads/main'` — dark for
     921 commits, revived by the trunk fast-forward with no edit to their conditions (plan
     `15.1-07`). `benchmark-regression-signal` is `pull_request || workflow_dispatch` with
     `continue-on-error` — correctly conditional, not dark. `benchmark` (rendering as `Performance
     Benchmarks`) was the one genuinely dark job this criterion names — `schedule || workflow_dispatch`
     with `ci.yml` never carrying a `schedule:` key — fixed and moved to a dedicated weekly
     `benchmarks.yml` (plan `15.1-06`; `15.1-CONTEXT.md` finding 7).

  7. The `smartstring` unmaintained advisory (`cargo-deny`, `License & Dependency Policy`) has a recorded disposition in `SECURITY-EXCEPTIONS.md` per the existing governance mechanism, rather than a permanently red required check.

**Plans:** 10/10 plans complete

**Status:** Complete — verified 2026-08-14, verification amended 2026-08-15
(`15.1-VERIFICATION.md`, status `passed`). All seven success criteria independently
verified `achieved` against the live tree and the live GitHub API. Two future human
actions remain open and are recorded there: a Snyk scan of the two new shell scripts
once the tool is available, and confirmation of `benchmarks.yml`'s first scheduled tick.

Plans:

**Wave 1**

- [x] `15.1-01` — Tracer: match-all push triggers (D-03), `examples` cache fix (D-10), `RUSTSEC-2026-0249` disposition (D-11)

**Wave 2**

- [x] `15.1-02` — Live CI evidence gate for SC1/SC5; D-05 context inventory (`15.1-CI-EVIDENCE.md`)

**Wave 3**

- [x] `15.1-03` — Cache fix widened to its class: 19 remaining blocks → `Swatinem/rust-cache@v2`
- [x] `15.1-04` — Three protection payloads: empirical required-check set (D-05), no bypass (D-07), release-branch ruleset (D-08b)

**Wave 4**

- [x] `15.1-05` — `integration-tests.yml` absorbed into `ci.yml` and deleted; context-name collision resolved (D-09)

**Wave 5**

- [x] `15.1-06` — `benchmarks.yml` weekly; dark job removed; four non-continuous dispositions recorded (D-08)

**Wave 6**

- [x] `15.1-07` — Trunk fast-forward (+994 commits), `develop` and `release/v0.7.0` retired (D-01, D-02)

**Wave 7**

- [x] `15.1-08` — Rulesets applied and read back verbatim (D-06); `15.1-RULESET-EVIDENCE.md`

**Wave 8**

- [x] `15.1-09` — `branching-model.md` + trigger-policy register and drift guard, wired into CI (D-04, D-09b)

**Wave 9**

- [x] `15.1-10` — ADR-0043/0044, appendix currency, pointer redirects, ROADMAP amended at source (D-12)

**Post-verification fix (2026-08-15):** a path-filtered required check (`Build MDBook`) was
found to deadlock any PR touching no docs path. Fixed by dropping `docs.yml`'s `pull_request`
paths filter, and closed structurally by a new `CLAUSE_REACHABILITY` in
`scripts/check-workflow-triggers.sh` with three covering test cases. Recorded in
`15.1-VERIFICATION.md` § Post-Verification Amendment.

### Phase 16: Documentation Currency & the Architecture Gap

**Goal**: The documentation describes the system that exists — the fourteen guides Milestone 11 left mid-update are current against the tree, and the architecture document either covers the seven subsystems it omits or says out loud that it is archive material.
**Depends on**: Phase 10 for one requirement only — **HARD-07 picks which `cargo doc` bar governs and DOCS-03 applies it**, adding the CI gate. DOCS-03 is also coupled to **DEBT-03** in Phase 8, which re-enables `paladin-ports` doctests and is what makes the port traits' rustdoc examples executable rather than merely present. DOCS-01, DOCS-02 and DOCS-04 depend on nothing.
**Requirements**: DOCS-01, DOCS-02, DOCS-03, DOCS-04
**Success Criteria** (what must be TRUE):

  1. Each of the fourteen user-guide, deployment and operations pages Milestone 11 left open is checked against the current tree — crate names, module paths, `make` targets, workflow names, error types, feature flags — and marked current or updated, with the linkcheck report reviewed. **This is the only open checkbox count in all 542 that survives verification**, and it is settled by content rather than by the files existing.
  2. A developer looking for how Commander, Council, Conclave, Grove, Maneuver, Sanctum or Sentinel fit into the architecture either finds them documented, or finds a clear statement that the architecture appendix is historical and where to look instead — closing a document that is **still exactly 311 lines with zero mentions of any of those seven shipped subsystems and zero diagrams**, because Milestone 11 relocated it into the one chapter its own rewrite epic was exempted from touching.
  3. `cargo doc` has one bar across the workspace, CI enforces it, and every public item in `src/` carries documentation with at least one example on each entry point — so the framework's primary integration contract is documented to a standard that three milestones previously set three different ways.
  4. `docs/assets/` either holds the demo recordings or does not exist — an empty directory and a missing `docs/DEMOS.md` stop implying work in flight, and the decision to record or withdraw the demos is on record along with the reason.

**Plans**: 14 plans

Plans:
**Wave 1**

- [x] 16-01-PLAN.md — TRACER: pinned doc toolchain (local + both devcontainer images), verbatim linkcheck report, the D-09 verdict record, and `cicd.md` settled by content

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 16-02-PLAN.md — DOCS-01 sweep A: orchestration, maneuver-flow-dsl, memory-management
- [x] 16-06-PLAN.md — DOCS-02: ADR-0047, the archive banner, Sentinel in the live chapter, PROMOTION.md → 0048
- [x] 16-07-PLAN.md — DOCS-03: clear the 20 `cargo doc` warnings, uniform `missing_docs` posture, ADR-0033 amended
- [x] 16-08-PLAN.md — DOCS-03: the D-05 entry-point enumeration, `scripts/check-public-api-examples.sh`, the D-06 heading rule
- [ ] 16-13-PLAN.md — DOCS-04: recorder toolchain in both images, behind a blocking supply-chain checkpoint

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 16-03-PLAN.md — DOCS-01 sweep B: tool-integration, paladin-configuration, output-formatting (closes the user-guides group)
- [x] 16-09-PLAN.md — DOCS-03: executable `# Examples` on the `paladin-ports` trait surface
- [ ] 16-14-PLAN.md — DOCS-04: four `.tape`-driven recordings, `docs/DEMOS.md`, one README line, requirement amendments

**Wave 4** *(blocked on Wave 3 completion)*

- [ ] 16-04-PLAN.md — DOCS-01 sweep C: docker, kubernetes, production (closes the deployment group)
- [ ] 16-10-PLAN.md — DOCS-03: executable `# Examples` on paladin-core, memory, battalion, herald

**Wave 5** *(blocked on Wave 4 completion)*

- [ ] 16-05-PLAN.md — DOCS-01 sweep D: the four operations pages, end-state linkcheck re-review, and two in-place ledger amendments
- [ ] 16-11-PLAN.md — DOCS-03: executable `# Examples` on the five zero-example crates and the facade

**Wave 6** *(blocked on Wave 5 completion)*

- [ ] 16-12-PLAN.md — DOCS-03: heading normalisation on the enumeration, both gates green, closing evidence

**Cross-cutting constraints:**

- `mdbook build docs/` exits 0 after the edits, and no file outside the three named pages is modified (D-12)
- The violation count reported by scripts/check-public-api-examples.sh is strictly lower at the end of this plan than at its start, and both figures are recorded

**Provider Expansion**

### Phase 17: Additional LLM Provider Adapters

**Goal**: Paladin talks to the providers its users actually deploy — the candidate field (Gemini, Kimi, Qwen, Llama-family hosts, and whatever else the study surfaces) is narrowed to a shortlist against recorded criteria rather than brand recognition, and every provider that survives ships as a feature-gated adapter meeting the same `LlmPort` contract the existing three do.
**Depends on**: Nothing hard. Phase 17 is the first phase beyond the ingest-derived roadmap and is independent of Phases 1-16. One soft coupling runs backwards: **Phase 14 made `ProviderCapabilities` truthful** (the LLM capability flag), so any new adapter inherits that standard rather than re-litigating it — a provider that cannot stream must report that it cannot stream. Phase 15's coverage floor applies to the new crate surface from the first commit.
**Requirements**: PROV-01, PROV-02, PROV-03, PROV-04

**Success Criteria** (what must be TRUE):

  1. **The shortlist is a decision, not a wish list.** A recorded provider-selection study evaluates the candidate field against explicit, written criteria — API compatibility with the existing adapter shape (OpenAI-compatible vs. bespoke), streaming and tool/function-calling support, whether a usable Rust HTTP path exists without a new heavyweight dependency, auth model, self-host vs. hosted-only, and licence/ToS constraints — and each candidate is marked **build**, **defer** or **reject** with the reason attached. Candidates the user named (Kimi, Gemini, Qwen, Meta/Llama) are each explicitly dispositioned, including any that are rejected. **Note that "Llama" names a model family, not a provider** — the study must decide which *host* (Bedrock, Groq, Together, Ollama, …) the adapter would actually target, or reject the row for lacking one.
  2. **Every provider marked *build* implements the full `LlmPort` contract**, not a subset: `generate`, `generate_stream`, `validate_model`, `get_available_models`, `get_provider_name`, and a `get_capabilities` that reports what the provider genuinely does. No adapter ships with a stubbed or optimistic capability response.
  3. **Each new adapter is feature-gated and additive.** `paladin-llm` gains one feature per provider alongside `openai`/`anthropic`/`deepseek`, the default feature set is unchanged, `cargo build --no-default-features --features <provider>` succeeds for each, and `provider_factory.rs` resolves the new providers from configuration the same way it resolves the existing three. Adding a provider does not change the behaviour of any existing one.
  4. **The adapters are tested to the standard already in force**, not exempted from it: mock-transport unit tests for request shaping, response parsing, streaming chunk assembly, and error mapping into `LlmError`; the workspace stays above the 82% line-coverage floor with the new code included; every public item carries rustdoc, and any live-API test is gated behind a credential-requiring feature so CI stays green without secrets.
  5. **The advertised surface matches the shipped one.** `paladin-llm`'s `Cargo.toml` description/keywords, the crate README, and the configuration documentation name exactly the providers that exist — so this phase does not create the same documentation-currency debt Phase 16 is closing.

**Plans**: 22 plans *(11 executed; 6 added 2026-08-18 by a second `/gsd-plan-phase 17 --gaps` run to close the one remaining blocking gap, the four review Warnings the developer put in scope, and two bookkeeping gaps; 4 added 2026-08-22 by a third `--gaps` run after UAT test 4 ran live against the real vendor endpoints and returned two blockers and one credential gap; **amended later the same day** — G-17-4c was resolved by live verification rather than planned around, its replacement G-17-4d added plan 17-22, and 17-20/17-21 swapped wave order so the documentation quotes the shipped constant instead of predicting it)*

Plans:
**Wave 1**

- [x] 17-01-PLAN.md — Tracer: shared OpenAI-compatible core (`compat/`), the Kimi preset on it, and the `cfg`-gated provider registry table, proven end-to-end against a mock transport
- [x] 17-02-PLAN.md — PROV-01's recorded provider-selection study: ADR-0045 with criteria-before-verdicts and a build/defer/reject row per candidate

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 17-03-PLAN.md — Qwen, Grok and Ollama presets on the shared core, with mock-transport tests and registry rows

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 17-04-PLAN.md — Generic operator-configured `openai-compatible` provider with pessimistic capability defaults *(blocking decision checkpoint: public surface naming, D-03 one-way)*

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 17-05-PLAN.md — Gemini bespoke adapter (`generateContent`, `systemInstruction`, `alt=sse` streaming), text-only

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 17-06-PLAN.md — Facade `llm-` provider flags wired for real (ADR-0046) and the config surface extended to nine providers *(default set widened to preserve current behaviour — D-11 amended 2026-08-17, option-b; no breaking change, checkpoint pre-resolved)*

**Wave 6** *(blocked on Wave 5 completion)*

- [x] 17-07-PLAN.md — Capability invariants for the six new adapters, the factory regression test, the Ollama Docker Tier 2 suite, and the 82% coverage gate

**Wave 7** *(blocked on Wave 6 completion)*

- [x] 17-08-PLAN.md — Advertised surface brought in line (Cargo metadata, README, config example and docs), exports baseline regenerated *(blocking human-verify checkpoint)*

**Wave 8** *(gap closure — blocked on Wave 7; verification returned `gaps_found` 2026-08-17)*

- [x] 17-09-PLAN.md — Tracer: close CR-01, the blocking Critical — allow-list guard on the caller-supplied Gemini model identifier before any request URL is built, red-then-green

**Wave 9** *(blocked on Wave 8 — shares `gemini/adapter.rs`)*

- [x] 17-10-PLAN.md — WR-04: `Policy::none()` for Kimi, Qwen, Grok, Ollama and Gemini so a redirect can never replay a credential header to another host, four inaccurate rationales corrected

**Wave 10** *(blocked on Wave 9 — shares `gemini/adapter.rs`)*

- [x] 17-11-PLAN.md — WR-03: every Gemini 401/403 and every credential-shaped 400 classifies as an auth failure, so a doomed request is attempted once rather than four times while holding a live key

*Scoped to CR-01 + the two security-adjacent findings by developer decision, 2026-08-17. WR-01, WR-02, WR-05, WR-06, WR-07, IN-01 and IN-02 remain open, developer-accepted review debt; the three `human_verification` items (coverage floor, Ollama live server, vendor smoke test) remain blocked on Docker / network egress / vendor credentials.*

**Wave 11** *(second gap closure — re-verification 2026-08-17 returned `gaps_found` on one new Critical; tracer)*

- [x] 17-12-PLAN.md — Tracer: close CR-01, the one blocking gap — a credential env var set to the empty string is no longer reported as a configured provider, with the test guard widened from three variables to all ten the nine-row registry reads

**Wave 12** *(blocked on Wave 11 — the tracer is proven before the Warnings expand on it)*

- [x] 17-13-PLAN.md — WR-01: `create()` accepts the `openai_compatible` underscore spelling `LlmConfig` already blesses, so a validated config cannot fail at the point of use on spelling alone
- [x] 17-14-PLAN.md — WR-02: an inverted or non-finite `OPENAI_COMPATIBLE_TEMPERATURE_MIN`/`_MAX` pair is a configuration error rather than a silently-accepted inverted range
- [x] 17-15-PLAN.md — WR-03 *(new — distinct from the closed auth-classification WR-03)*: Gemini reports a truncated-to-empty completion as `EmptyCompletion`, the same signal every compat preset gives

**Wave 13** *(blocked on Wave 12 — shares `gemini/adapter.rs` with 17-15)*

- [x] 17-16-PLAN.md — WR-04 *(new — distinct from the closed redirect-replay WR-04)*: both `generate_stream` implementations retry a transient connection-open failure exactly as their own `generate()` does, and attempt an auth failure exactly once

**Wave 14** *(blocked on Wave 13 — records what the run could not verify)*

- [x] 17-17-PLAN.md — Bookkeeping: the `WINDOWS.md` rows for 17-11's and this run's not-run Snyk scans plus IN-01 as carried-forward debt, and a CI job that finally runs the registry regression tests under `llm-all`

*Scoped to CR-01 + WR-01/WR-02/WR-03(new)/WR-04(new) by developer decision in an interactive checkpoint, 2026-08-18. **IN-01 explicitly excluded** and carried forward as tracked, accepted debt (plan 17-17 files the row). The four `human_verification` items (Snyk scan, 82% coverage floor, Ollama live server, vendor smoke test) remain blocked on Docker / network egress / vendor credentials / an unavailable Snyk tool — none is closeable by planning. Note the finding-ID collision: `17-REVIEW.md` reused the labels WR-03 and WR-04 for findings distinct from the ones plans 17-11 and 17-10 closed; all four are closed once this wave completes.*

**Wave 15** *(third gap closure — UAT test 4 ran live on 2026-08-22 with real credentials and returned `issue`/blocker; tracer)*

- [x] 17-18-PLAN.md — Tracer (G-17-4a): a preset declares which sampling parameters its request path carries, the shared engine honours the declaration, and Grok completes a live `generate()` call against a refreshed, live-listed xAI model

**Wave 16** *(blocked on Wave 15 — shares `compat/engine.rs`)*

- [x] 17-19-PLAN.md — G-17-4b: Kimi's retired default model refreshed against the live Moonshot catalog, its fixed-temperature constraint honoured by option (a), and the builder's ADR-0004 gate narrowed to temperatures a caller actually expressed so a truthful degenerate range is not a build-time outage *(the blocking decision checkpoint is **answered** — the developer selected option (a) on 2026-08-22, so this plan is now autonomous)*

**Wave 17** *(blocked on Wave 16 — shares `compat/engine.rs`)*

- [x] 17-21-PLAN.md — G-17-4d, part 1: Qwen's shipped default region becomes US (Virginia) by developer decision, the region-scoped-credential rule and the known regional endpoints are documented where `DASHSCOPE_BASE_URL` is described, and Qwen is live-verified at the endpoint it now ships with *(**re-ordered ahead of 17-20**: the constant must change before the documentation can quote it; the human-action checkpoint is gone — the credential was valid all along)*

**Wave 18** *(blocked on Wave 17 — the refreshed identifiers and the new Qwen endpoint must exist before they can be advertised)*

- [x] 17-20-PLAN.md — PROV-04's advertised surface brought back in line: every operator-facing default model and base URL matches the shipped constant, one blanket unverified-endpoint caveat becomes a per-vendor dated verification status, and the DashScope region constraint reaches the operator-facing documents

**Wave 19** *(blocked on Wave 18 — shares `compat/engine.rs` and the configuration guide; carries the run's final regression record)*

- [x] 17-22-PLAN.md — G-17-4d, part 2: the shared engine stops describing an authentication failure the way it describes being offline, so a region/credential mismatch is audible instead of returning a plausible curated list — vendor-agnostic, so all six compat-backed adapters gain it, and the D-13/D-14 fallback contract is deliberately unchanged

*Third gap-closure run, planned 2026-08-22 from `17-UAT.md` test 4's diagnosed gaps. G-17-4a and G-17-4b are blockers — Grok cannot complete any `generate()` call because the shared engine unconditionally serialises a parameter xAI rejects, and Kimi's default model is retired while its current models accept only `temperature: 1`. Gemini passes and is the regression control across every plan in the run. The other four UAT tests passed on 2026-08-18/19 and are untouched here.*

*(**Amended in place 2026-08-22, per D-00d.** The paragraph above previously continued: "G-17-4c is a **credential** problem, not a code problem: both DashScope endpoints return `401 invalid_api_key` in Alibaba's documented envelope, which confirms `QWEN_DEFAULT_BASE_URL` rather than implicating it, so no base-URL change is planned." **That conclusion was falsified the same day and is superseded.** Alibaba documents that API keys are region-scoped and cannot be used across regions, so a region-scoped key returns a well-formed 401 from every endpoint except its own — the envelope proves nothing about the URL. Measured with one credential and one binary, differing only in the endpoint: the shipped Singapore default returned a list byte-identical to the curated fallback, i.e. a silently failed fetch, while the US Virginia endpoint returned 92 live models with the default model present. **G-17-4c is resolved**; the real defect is G-17-4d — the adapter models DashScope as a single global endpoint, and the mismatch is silent. The developer's binding call is that the default becomes US (Virginia), which moves the problem to Singapore and mainland operators rather than eliminating it, hence the documentation half. The run grows from four plans to five.)*

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17

Real dependencies are much looser than the numbering, and several couplings run *backwards*
through it. Recorded here so neither side gets planned twice:

**The four cheapest high-value items, none of which needs to wait for anything:**

1. **SUPPLY-01 (Phase 12)** — delete `ci.yml:389-406`. Eighteen lines, and a Milestone 10
   acceptance criterion becomes true.
   (**Corrected by Phase 12 (plan 12-01), dated 2026-08-09, citing `ci.yml:465-482` and commit
   `cb75b2b`:** this citation was already stale — the duplicate job was actually at
   `ci.yml:465-482`, and Phase 9's plan 09-06 deleted it in commit `cb75b2b` before this item was
   read against the live tree. SUPPLY-01 is closed; see `REQUIREMENTS.md`'s "Verified by Phase 12"
   block.)

2. **DEBT-01 (Phase 8)** — fix nine `project/current-exports.txt` references (five tooling, four
   requirement texts) and turn the `api-surface` CI job green for the first time since commit
   `928c6d5`. **The longest-lived unfixed defect in the corpus**, unchanged across three ingest
   runs.

3. **WEB-01/WEB-02 (Phase 14)** — resolve the token mechanism, because it has a correctness
   consequence under the Kubernetes Deployment that Milestone 12 shipped.

4. **DOCS-02 (Phase 16)** — decide whether `design-and-architecture.md` is archive material or a
   live deliverable. It cannot be both, and it has been invisible for two milestones.

**Time-boxed work:**

- **Phase 9 is the only time-boxed phase, and it depends on nothing.** The RustSec risk acceptance
  it reconciles carries a **2026-09-30** review/expiry target — the only date anywhere in the
  263-document corpus, and the only item where leaving it unrecorded has an ongoing operational
  cost. Numeric order puts it ninth; **urgency does not.** **Phase 12 should run with or before
  it**, since SUPPLY-01 and SUPPLY-02 carry the concrete fix and the corrected scope.

**Couplings that run backwards through the numbering:**

- **Phase 5 depends on Phase 1 only** (for RECON-07). If Milestone 2-3 ground truth becomes urgent
  before Phases 2-4 complete, Phase 5 can run directly after Phase 1.

- **Phase 7 depends on nothing**, and two of its answers are inputs to Phase 4: ARCH-03(a) records
  which Rust edition the workspace standardises on, which REL-02 applies; ARCH-04 records whether
  Milestone 6 was a breaking change, which determines REL-01's single version story. **Whichever
  phase executes first records the answer; the other applies or ratifies it.**

- **Phase 8 depends on Phase 7** for one requirement only (ARCH-03(c) → DEBT-05). DEBT-01 through
  DEBT-04 are independent, verified and small.

- **Phase 10 depends on nothing**, and three of its answers feed backwards: HARD-06 (is
  `pdf-extract` reachable?) determines whether SEC-01's and SUPPLY-02's `RUSTSEC-2026-0187`
  suppression is needed at all; HARD-07 (which `cargo doc` bar governs) determines what DEBT-03 in
  Phase 8 and DOCS-03 in Phase 16 must satisfy; HARD-03 records the version trajectory REL-01
  converges. **Running Phase 10 before Phases 9 and 12 saves both from guessing.**

- **HARD-03 → ORCH-05 → REL-01.** Phase 10 records `v0.1.0-rc.1` as history; Phase 13 completes the
  chain v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0; Phase 4 converges the branch/`Cargo.toml`/tag
  disagreement. REL-01 must not converge on an rc.1 figure.

- **Phase 11 depends on Phase 10** (HARD-05 → FACADE-02) and loosely on Phase 7 (ARCH-04 →
  FACADE-02's D1). FACADE-01 is a low-risk quick win with an exactly-verified scope of 17
  occurrences.

- **FACADE-02's D2 (Phase 11) ↔ DEFER-02 (Phase 15) is the sharpest sequencing question in this
  roadmap.** Two ingested registers propose incompatible next actions on `user_service.rs`: D2
  splits it, Deferred-QA Epic 28 tests it to ≥ 80%. Splitting first and testing the resulting units
  is cheaper, but it changes Epic 28's estimate and its mock set. **Do not schedule them
  independently.**

- **Phase 12 depends on nothing** and narrows Phase 9. **Phase 13 depends on nothing.**
  **Phase 14's WEB-03 depends on nothing** and is a one-line correctness fix.

- **RECON-07 → VERIFY-05 → PIPE-02.** The coverage gate now has **six** recorded positions across
  the corpus. Phase 1 records the answer, Phase 5 extends it across the four earlier ones, and
  Phase 15 must land the CI threshold on the same number or record why it differs.

- **Phase 15's two halves are sequential**: Epic 25's gates (PIPE) before the coverage register
  (DEFER), per the register's own recommendation.

- **Phase 16 depends on Phase 10** for HARD-07 only; three of its four requirements are
  independent.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Ground Truth & Decision Records | v0.7.1 | 12/12 | Complete    | 2026-07-31 |
| 2. Functional Gap Closure | v0.7.1 | 11/11 | Complete    | 2026-08-01 |
| 3. Verification Depth | v0.7.1 | 8/8 | Complete    | 2026-08-02 |
| 4. Release Coherence | v0.7.1 | 7/7 | Complete    | 2026-08-03 |
| 5. Milestone 2-3 Ground Truth | **v0.7.2** | 13/13 | Complete    | 2026-08-05 |
| 6. Verified Gap Closure | **v0.7.2** | 10/10 | Complete    | 2026-08-05 |
| 7. Workspace Ground Truth & Recorded Answers | M4-6 | 13/13 | Complete    | 2026-08-06 |
| 8. Verified Defect Closure | M4-6 | 9/9 | Complete    | 2026-08-07 |
| 9. Release & Security Gate Integrity | M7-8 | 7/7 | Complete    | 2026-08-08 |
| 10. Milestone 7-8 Ground Truth & Recorded Account | M7-8 | 11/11 | Complete    | 2026-08-08 |
| 11. Facade Residue & Deferred Register Disposition | M7-8 | 5/5 | Complete    | 2026-08-09 |
| 12. Supply-Chain Gate Integrity | M9-12 | 4/4 | Complete    | 2026-08-10 |
| 13. Milestone 9-12 Ground Truth & Recorded Account | M9-12 | 13/13 | Complete    | 2026-08-10 |
| 14. API Contract Truthfulness | M9-12 | 8/8 | Complete    | 2026-08-12 |
| 15. Coverage & CI Quality Gates | M9-12 | 10/10 | Complete    | 2026-08-13 |
| 16. Documentation Currency & the Architecture Gap | M9-12 | 0/TBD | Not started | - |

## Not In This Roadmap

Deliberate omissions, so a later reader does not mistake them for oversights.

### Shipped work — the large majority of the corpus

- **Shipped Milestone-1 work.** 98% of the milestone's task items are done. The per-requirement
  record is the *Milestone 1 as-shipped ledger* in `REQUIREMENTS.md`; re-planning it as phases
  would be fiction.

- **Shipped Milestone 2-3 work — which is nearly all of it.** Sanctum and RAG (Epics 11-12),
  Sentinel vision (Epics 13, 20), autonomous planning and handoffs (Epics 14, 21), Conclave
  (Epic 15), Council and Grove (Epic 16), the Maneuver Flow DSL (Epic 17), the CLI consolidation
  and enhancement (Epics 17.5, 18), Herald consolidation (Epic 19), the Paladin registry and
  Commander metadata export (Epic 22), the scheduler port and CLI configuration wiring (Epic 23)
  and the test/benchmark hardening (Epic 24) all have shipped artefacts in the tree. Phase 5
  verifies the record; it does not rebuild the features.

- **Shipped Milestone 4-6 work — which is all of it except five defects.** The Cargo workspace and
  every crate extraction, the feature-flag matrix and CLI feature gate, and all four Milestone 6
  relocations are **verified shipped against the tree**, not merely claimed.

- **Shipped Milestone 7-8 work — which is all of it bar six verified items.** The four crate
  extractions behind the cost-benefit gate, the `Dockerfile.chef` workspace adaptation, the ten
  per-crate Makefile targets, the five-benchmark migration, the whole `v0.1.0-rc.1` release cycle,
  the 25 List A deletions, `src/core/` reduced to exactly six files, the `use_cases` → `services`
  rename, the actix removal and cargo-deny ban, the three mounted axum delivery routes, and the
  reconciliation's fifteen commits (~10,250 net LOC removed).

- **Shipped Milestone 9-12 work — which is all of it bar the record and four defects.** The whole
  Milestone 9 orchestrator subsystem (`execute_workflow()` at
  `src/application/services/orchestration/mod.rs:382`, the `WorkflowRepository` port and its SQLite
  adapter, the content processors, the orchestrator bridge, `AuthPort` and RBAC); the whole
  Milestone 10 tooling set (pre-commit with a CI gate, cargo-audit reading `audit.toml`, cargo-deny,
  OSV-Scanner with SARIF, a CycloneDX SBOM in the release pipeline, `release.toml` with
  tag-triggered publishing, the `verify-tag-source` guard and committed GitHub rulesets); the mdbook
  with `warning-policy = "error"`, mdbook-mermaid, the full chapter hierarchy and all six
  deployment-topology pages; and the whole Milestone 12 web API (agent registry and controller,
  `paladin-server`, SSE streaming, in-process jobs, the unified error envelope, health/ready,
  request logging, CORS/body-limit/timeout layers, tower-governor rate limiting, API-key and bearer
  auth with per-agent roles, OpenAPI with a committed drift baseline, `Dockerfile.server`,
  `docker-compose.yml` and `k8s/`). **37 rows verified directly against the tree.** Phase 13 records
  them; no phase rebuilds them.

### Signals that are not work

- **Open checkbox counts as a backlog.** 542 items are unchecked across 75 task lists. Five runs of
  verification found them wrong in *both* directions — understating shipped reality (Conclave 129
  and Sanctum 111, both shipped), overstating completion (CLI isolation fully checked with three
  dependencies still unconditional), contradicted outright (Milestone 8's three), vacuous
  (Milestone 12's three are feature-branch scaffolding) and nonexistent (project-management's one is
  a formatting example inside a template). **Exactly one block survives: Milestone 11's 26**, and
  DOCS-01 owns it.

- **Milestone 5's, Milestone 6's, Milestone 9's and Milestone 10's checkbox counts** — all
  corroborated or contradicted by code, none converted into tasks.

- **`REQ-master-plan-epics-11-18` as new scope.** It is the origin document for Epics 11-18, dated
  2026-01-29; every one of those epics was ingested in run 2 and most are verified shipped. Its
  value is provenance — the dependency graph and the epic-level risk assessment — not scope.

### Relocations, not gaps

- **`STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`,
  `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`,
  `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md`.** Absent from the paths their PRDs name,
  but shipping as mdbook chapters after the Milestone 11 overhaul — which `docs/MIGRATION_LOG.md`
  records. Recording the relocation is ARCH-05 and HARD-01; building them would be duplicate work.

- **Four stale module and document paths in run-5 requirements** — `listener_service.rs`,
  `src/application/ports/output/llm_port.rs`, `docs/Design/Design_and_Architecture.md` and the
  README demos clause. Corrected at source by ORCH-03, not rebuilt.

### Positions that would break things if implemented as written

- **The 14 requirements that shipped code superseded by outcome** (HARD-01) — actix-web as a
  `paladin-web` dependency, the `storage-sqlite` flag, the per-crate ordered publish dry run, the
  `ml` feature gate, the Milestone 8 Epic 3 no-extraction mandate, the 160-file facade target (the
  tree reads 136), and the root-path documentation deliverables.

- **A `paladin-cli` crate, MCP transport feature flags, and `vision` gating the encryption
  crates.** The last would break `cargo build --no-default-features`, because `chacha20poly1305`
  and `zeroize` serve user auth and Citadel encryption, not vision.

- **A migration between the two shipped vision surfaces.** Both ship;
  `intel/code-verification.md` records this as coexistence and says to confirm intent first.

### Explicit non-goals from the source milestones

- **Hot-reloading `config.yml`**, **terminating TLS in `paladin-server`**, **fine-grained scopes
  beyond `allowed_roles` plus the admin gate**, and **encrypting configuration at rest** — all
  Milestone 12 non-goals, recorded so they are not mistaken for omissions.

- **Rewriting the 35 mdbook appendix files** — Milestone 11 Epic 3 non-goal. One exception is under
  decision: `design-and-architecture.md`, whose relocation into that exempt chapter is precisely
  why its gap survived (DOCS-02).

- **Benchmark regression detection (`critcmp`, `github-action-benchmark`)** — Deferred-QA Epic 25
  non-goal. Note the inversion: it already ships as `benchmark-regression-signal` from Milestone 7
  Epic 3, while the `bench-check` compile prerequisite does not (PIPE-01).

- **Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml`.** None exists. The first two are
  named only by a superseded disposition record that contradicts its own governing PRD (FACADE-04);
  the third is a *placement condition* on reintroducing a removed feature (FACADE-03), not a
  deliverable.

- **A future content-delivery crate.** Reserved by Milestone 7 Epic 1 as the "correct long-term
  home" for `file_content_repository.rs`; the file was then deleted and no later document mentions
  the crate. Carried as a v2 note, not a phase.

### Decisions this roadmap records but does not take

- **Resolving the 30 competing variant groups / 69 warnings.** Recording answers is in scope
  (RECON-02 … RECON-07, VERIFY-03 … VERIFY-06, ARCH-03, ARCH-04, SEC-01, SEC-02, HARD-01 … HARD-07,
  WEB-01, PIPE-02). Picking winners inside `REQUIREMENTS.md` is not — the user has stated that
  variants are expected and that settling past disagreements is not the goal of this ingest. Where
  shipped code settles a variant, that is recorded as a **fact about the tree**, at the top of the
  precedence order, not as a decision taken here. **Group 29 is the one variant shipped code cannot
  settle**: the tree carries the Milestone 12 shape and the Milestone 9 mechanism simultaneously.

- **Promoting the eleven ADR candidates.** **Zero locked decisions exist across all 263 corpus
  documents** — no ADR-typed and no SPEC-typed document exists anywhere. Promotion requires
  re-tagging the source via `--manifest` and re-running ingest; manufacturing a lock inside a
  planning artefact would fabricate authority the corpus does not contain. SEC-01 and SUPPLY-03
  record the recommendation for the two candidates with a live operational cost — the same subject,
  from two different milestones — and do not act on it.

### Tech debt tracked as v2

- **Decomposing the three oversized service files** (2,757 / 2,294 / 1,840 lines) — real debt, no
  ingested requirement demands it.

- **Clone/lock-contention work** — the 383 `.clone()` calls and nine orchestrator locks flagged in
  `codebase/CONCERNS.md`. Blocked on Phase 3 producing benchmark evidence first.

- **The `paladin-core` / `paladin-ports` dependency allowlists** — declared 6 and 7, shipping 14 and
  10. The architectural invariant holds; this is document-versus-code drift needing ARCH-03(b) to
  choose a direction.

## Roadmap Extension Protocol

**The ingest is complete.** Five runs covered all 263 documents in `.project/` — 199 classified
(188 prose + 11 task lists) and 64 `tasks-*.md` measured deterministically. **There is no run 6.**
This section is retained because the rules below still govern any *future* addition to this
roadmap, from any source.

This roadmap is **appended to**, not restructured.

1. **Do not renumber or rewrite Phases 1-16.** Phases 1-4 are Milestone 1 close-out; 5-6 are
   Milestone 2-3; 7-8 are Milestone 4-6; 9-11 are Milestone 7-8; 12-16 are Milestone 9-12 +
   Deferred-QA. New phases start at **Phase 17** and continue upward. Use decimal insertions (e.g.
   2.1) only for urgent work that must execute *between* existing integer phases.

2. **Keep the milestone-grouped form.** Add a row to the `## Milestones` table, a labelled block
   under `## Phases`, and a new expanded `## Phase Details` section for the incoming phases. Wrap
   **only genuinely completed or superseded** milestone sections in a `<details>` block labelled
   with their milestone and status. Keep the `### Phase N: Name` header format verbatim.
   **`<details>` is a scope signal, not a rendering choice: GSD's roadmap parser strips every
   `<details>` block before phase lookup** (`stripShippedMilestones` →
   `markdown-sectionizer.stripTaggedBlocks`), so any phase wrapped in one is invisible to
   `roadmap.get-phase`, `roadmap.analyze`, and every workflow built on them — `/gsd-plan-phase`
   included. Use a plain bold label line for milestones that are not started or in progress.

3. **Add new requirement ID prefixes; do not recycle. Seventeen are spent**: `RECON-*`, `GAP-*`,
   `QUAL-*`, `REL-*` (Milestone 1); `VERIFY-*`, `CLOSE-*` (Milestone 2-3); `ARCH-*`, `DEBT-*`
   (Milestone 4-6); `SEC-*`, `HARD-*`, `FACADE-*` (Milestone 7-8); `SUPPLY-*`, `ORCH-*`, `WEB-*`,
   `PIPE-*`, `DEFER-*`, `DOCS-*` (Milestone 9-12 + Deferred-QA). Ingested `REQ-*` IDs are stable
   merge keys — match on them rather than re-deriving. **Extending an existing requirement in place
   is preferred to creating a near-duplicate**: run 4 extended ARCH-01, DEBT-01 and DEBT-03; run 5
   extended DEBT-01 again (six stale references became nine) and *corrected* SEC-01. Record the
   extension at the requirement and in the footer.

4. **Expect supersession, and record the chain.** **Zero locked decisions exist across the whole
   corpus** (0 ADR, 0 SPEC across 199 classified documents), and later milestones deliberately
   restructure earlier ones. Run 2 produced eight documented supersessions of run-1 requirements;
   run 3 produced eleven more, including the entire monolith → workspace path migration and one
   requirement a later milestone reversed outright; run 4 produced eleven more still — and the first
   case of a **document superseding another document by name**,
   `facade-cleanup-RECONCILIATION-2026-06-04.md`; run 5 produced twelve more, including the first
   case of a later run **correcting an earlier run's direct code verification**. See *Superseded but
   preserved* in `REQUIREMENTS.md`. **Relocation is not contradiction.** An ADR arriving later
   outranks anything asserted in these phases; record the supersession in `PROJECT.md` Key Decisions
   rather than silently editing a phase.

5. **Re-check the ledgers, not the phases.** If a later document claims earlier work is incomplete,
   verify against shipped code and update the relevant as-shipped ledger in `REQUIREMENTS.md`.
   Precedence for this project is **shipped tree > `.planning/codebase/` map >
   `intel/code-verification.md` > PRD > DOC > task-list checkbox.**

6. **Checkbox counts cut both ways — verify each one.** The five-run record is conclusive: counts
   understated shipped reality (runs 1-2), were accurate once and overstated once (run 3), were
   contradicted outright (run 4), and were vacuous or nonexistent (run 5). **Never convert a count
   into a requirement without checking the tree.** The trustworthy remaining-work signal in this
   corpus is the **three deferred registers** — Milestone 8's `deferred-items.md` and
   `deferred-features.md` (whose every verifiable claim matches the tree exactly, including a
   `println!` residue count exact to the occurrence), and `Deferred-QA-CICD-Completion` with
   `DEFERRED_COVERAGE.md` (whose *scope* is real and largely unbuilt, but whose *paths and numbers*
   need re-measurement) — plus the verified defects in `intel/code-verification.md`.

7. **Path claims in old PRDs are historical, including some of the newest ones.** Every
   `src/core|application|infrastructure` path in the run-1 and run-2 corpus predates the workspace
   decomposition; several run-3 paths were moved again by Milestone 6 or 8; and **four run-5
   requirements — written in June 2026 — name paths that were already gone**. Resolve current
   locations through `.planning/codebase/` or the tree, never through a PRD.

8. **Milestone numbers in source documents are not always milestone numbers.** Four instances
   exist: the M4-M6 overviews number themselves by refactoring tier, the M3 release notes assign
   Epics 19-23 to four M2 features, PRDs cross-reference "Milestone 1 / Epic 2" meaning M4 Epic 2,
   and the M7 overview titles itself "Milestone 4". In all cases the directory / task-list numbering
   is authoritative here. **A fifth was predicted in run 5 and did not occur** (ORCH-05).

9. **The Milestones 8-11 dependency graph is spent.** It described M8 → M9 **HARD**, M8 → M11
   **HARD** on path stability with M11 Epics 3-4 waiting on M9 Epics 1-3, M9 → M11 **HARD** on API
   stability, and M8 → M10 **SOFT**; critical path M8 → M9 → M11 Epics 3-5 = 11-17 sprints, M10
   entirely off it. **Run 5 confirms every dependency was honoured and every release gate was cut**
   — v0.3.0, v0.4.0, v0.5.0, v0.6.0. Keep its dependency semantics and release-gate criteria as a
   pattern; the schedule is history.

---
*Roadmap created: 2026-07-30 (ingest run 1 of 5 — `.project/Milestone_1-MVP`, 36 docs)*

*Extended: 2026-07-30 (ingest run 2 of 5 — `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs; Phases 5-6 added, Phases 1-4 unchanged)*

*Extended: 2026-07-30 (ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` +
`.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements`,
32 docs; Phases 7-8 added, Phases 1-6 unchanged. Three earlier requirements were **narrowed** by
shipped-code verification rather than renumbered — RECON-02, RECON-03 and GAP-07 — and REL-02
gained the exact edition state.)*

*Extended: 2026-07-30 (ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` +
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution`, 40 docs; **Phases 9-11 added, Phases 1-8
unchanged and unrenumbered.** 16 new requirements: SEC-01 … SEC-05, HARD-01 … HARD-07,
FACADE-01 … FACADE-04. ARCH-01, DEBT-01 and DEBT-03 were **extended in place** rather than
duplicated. The Milestone 4-6 detail section was wrapped in a `<details>` block per protocol
item 2; the `### Phase N:` headers are unchanged.)*

*Extended: 2026-07-30 — **INGEST RUN 5 OF 5, FINAL. THE INGEST IS COMPLETE.**
`.project/Milestone_9-Classic-Orchestrator-Completion` +
`.project/Milestone_10-CI-Hardening-Release-Automation` +
`.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
`.project/Deferred-QA-CICD-Completion` + `.project/project-management`, 46 docs.
**Phases 12-16 added; Phases 1-11 unchanged and unrenumbered.** 24 new requirements:
SUPPLY-01 … SUPPLY-03, ORCH-01 … ORCH-05, WEB-01 … WEB-04, PIPE-01 … PIPE-05, DEFER-01 … DEFER-03,
DOCS-01 … DOCS-04. DEBT-01 was **extended in place** a second time (six stale references became
nine) and shed its four `actions-rs` references to PIPE-04; SEC-01 was **corrected in place** —
run 4's `deny.toml`-out-of-sync finding is withdrawn, and SUPPLY-01/SUPPLY-02 carry the corrected
scope. The Milestone 7-8 detail section was wrapped in a `<details>` block per protocol item 2, and
the Overview was rewritten so this file reads as one roadmap rather than five appended fragments;
every `### Phase N:` header is unchanged and verbatim.
**Cumulative: 263 documents covered, 554 requirements, 86 forward requirements across 16 phases,
60 variant entries across 30 groups, 69 warnings, 0 blockers, 0 locked decisions, 11 ADR
candidates.***

*Corrected: 2026-07-30 (structural defect, no scope change). Runs 3, 4 and 5 wrapped the
**not-started** Milestone 1, 2-3, 4-6 and 7-8 detail sections in `<details>` blocks, citing
protocol item 2 — but item 2 reserves that wrapper for **completed or superseded** milestones, and
its claim that "downstream tooling parses it, including inside `<details>`" was false. GSD's
roadmap parser strips every `<details>` block before phase lookup, so **Phases 1-11 were invisible
to `roadmap.get-phase` and every workflow built on it**; `/gsd-plan-phase 1` failed with
`malformed_roadmap`. The four wrappers were replaced with plain bold label lines matching the
Milestone 9-12 form already used in this file, and protocol item 2 was corrected to state the
parser contract. **No phase, requirement, goal, success criterion or `### Phase N:` header was
changed** — only the four `<details>`/`<summary>`/`</details>` wrapper lines were removed. All 16
phases now resolve.*

*Extended: 2026-08-15 — **first forward addition, not ingest-derived.** Phase 17 (Additional LLM
Provider Adapters) added under a new **Provider Expansion** milestone label, per *Roadmap Extension
Protocol* item 1 ("New phases start at Phase 17 and continue upward"). Phases 1-16 unchanged and
unrenumbered; every `### Phase N:` header is verbatim. One new requirement prefix — **`PROV-*`**
(PROV-01 … PROV-04) — the eighteenth, recycling none of the seventeen spent. The phase leads with a
**provider-selection study** rather than a build list: which candidates qualify is itself the first
deliverable, and PROV-02's size is set by PROV-01's verdicts.*
