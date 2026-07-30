# Roadmap: Paladin

## Overview

Paladin already works. It ships at v0.7.0 with a 9-crate Rust workspace, 22 runnable examples, a
112 MB multi-arch Docker image and reference Kubernetes manifests. Across the 81 documents ingested
so far, 7,511 of 8,053 task items are checked (93%) — and the shipped tree is *ahead of* even that
figure. **This roadmap does not build the framework; it closes out milestones that already shipped
and makes the planning record match the code.**

Milestone 1 close-out is short and specific. First make the planning record match the shipped code
and give each of the six contested type/gate definitions one recorded answer (Phase 1). Then close
the residual functional gaps that verification exposes and apply those recorded definitions in code
(Phase 2). Then make the quality numbers real rather than aspirational — coverage to the recorded
gate, error paths actually executed, benchmarks re-enabled with documented baselines (Phase 3).
Finally make the release coherent: one version, one edition, a defensible advisory posture,
reviewed docs, and the whole gate suite green in CI (Phase 4).

Milestone 2-3 close-out is shorter still, and that is the finding rather than an omission. Sanctum,
RAG, Sentinel vision, autonomous planning and handoffs, Conclave, Council, Grove, the Maneuver Flow
DSL, the enhanced CLI, Herald consolidation, the Paladin registry and the scheduler port **all ship
in the v0.7.0 tree.** What is missing is the record: which PRD criteria those artefacts satisfy,
which of two competing surfaces each one implements, what three unverified open-checkbox blocks
actually contain, and one documentation defect that has been propagating epic numbers incorrectly
across the corpus (Phase 5). Exactly one defect in run-2 scope is verified open against the tree,
and it gets closed alongside whatever Phase 5's verification exposes (Phase 6).

## Milestones

| Milestone | Phases | Status | Source |
|---|---|---|---|
| **Milestone 1 close-out** | 1-4 | Not started | Ingest run 1 — `.project/Milestone_1-MVP` (36 docs) |
| **Milestone 2-3 close-out** | 5-6 | Not started | Ingest run 2 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs) |
| *Milestones 4-6* | TBD | Awaiting ingest run 3 | Crate/feature refactor, workspace decomposition, architectural refinements |
| *Milestones 7-8* | TBD | Awaiting ingest run 4 | Production hardening, facade cleanup and shim resolution |
| *Milestones 9-12 + Deferred-QA + project-management* | TBD | Awaiting ingest run 5 | Classic orchestrator, CI hardening, docs overhaul, Web API |

Later runs append new milestone sections and continue phase numbering upward — see
*Roadmap Extension Protocol* at the end of this file.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

**Milestone 1 close-out**

- [ ] **Phase 1: Ground Truth & Decision Records** - Verify the planning record against shipped v0.7.0 code and record one answer per competing variant pair
- [ ] **Phase 2: Functional Gap Closure** - Finish the residual Milestone-1 functionality and apply the recorded definitions in code
- [ ] **Phase 3: Verification Depth** - Make coverage, error-path testing and performance baselines real and measured
- [ ] **Phase 4: Release Coherence** - One version, one edition, defensible dependencies, reviewed docs, green gate suite

**Milestone 2-3 close-out**

- [ ] **Phase 5: Milestone 2-3 Ground Truth** - Record what Epics 11-24 actually shipped, verify the three unverified blocks, and fix the epic-numbering defect at its source
- [ ] **Phase 6: Verified Gap Closure** - Close the one verified defect plus whatever Phase 5 proves genuinely outstanding

## Phase Details

<details>
<summary><strong>Milestone 1 close-out — Phases 1-4 (not started)</strong></summary>

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
**Plans**: TBD

### Phase 2: Functional Gap Closure
**Goal**: Every Milestone-1 functional requirement is either working and tested, or explicitly deferred with a recorded reason — and the types in code match the Phase 1 decisions.
**Depends on**: Phase 1
**Requirements**: GAP-01, GAP-02, GAP-03, GAP-04, GAP-05, GAP-06, GAP-07
**Success Criteria** (what must be TRUE):
  1. `cargo test --workspace` passes with zero failures, including `test_auto_selects_campaign_for_workflow_keywords`, which fails today.
  2. A developer can run a Chain of Command battalion from an example and watch the commander select specialists, survive a specialist failure through fallback logic, and return a synthesized answer — with tests covering all four delegation strategies.
  3. A Battalion result rendered through the JSON, Markdown and Table Heralds shows Battalion name, ID and type, per-Paladin results in execution order, aggregated token usage across Paladins, and partial results when something failed.
  4. Commander execution returns a normalized result carrying strategy used, per-Paladin timings, success/failure counts and preserved errors, and writes telemetry metadata to `metadata_output_dir` when one is configured.
  5. The shipped types match the Phase 1 ADRs: one `BattalionConfig` (the duplicate in `citadel.rs` resolved), one `BattalionResult`, and a single-Paladin Commander in Auto mode that executes instead of failing Formation validation.
**Plans**: TBD

### Phase 3: Verification Depth
**Goal**: The project's quality claims are measurements rather than targets — coverage at the recorded gate, error paths executed rather than skipped, and performance baselines that exist.
**Depends on**: Phase 2
**Requirements**: QUAL-01, QUAL-02, QUAL-03, QUAL-04, QUAL-05
**Success Criteria** (what must be TRUE):
  1. `cargo llvm-cov` reports unit coverage at or above the gate recorded in Phase 1 (baseline 60.88%) and integration coverage at or above 70% (baseline 67.79%).
  2. No first-party source file reports 0% coverage — the arsenal execution and registry services, the Redis and MinIO adapters, the user controller and repository, and `main.rs` all have exercising tests.
  3. Commander failure behaviour is proven by tests that actually run: retry counts increment, partial failures are collected and returned, and a timeout stops sibling agents — none of it behind `#[ignore]`.
  4. Each MCP tool-invocation failure mode has a passing test: expired or rejected token, malformed response, handshake timeout, unknown tool, and bad arguments.
  5. `cargo bench` completes across the restored suites and a baseline document records throughput, P50/P95/P99 latency, memory per Paladin and startup time — so the next performance change can be compared against something.
**Plans**: TBD

### Phase 4: Release Coherence
**Goal**: A developer can clone the release tag, build it, trust its version and its dependency posture, follow the quickstart to a working agent, and see CI prove all of it.
**Depends on**: Phase 3
**Requirements**: REL-01, REL-02, REL-03, REL-04, REL-05
**Success Criteria** (what must be TRUE):
  1. Version metadata tells one story — workspace `Cargo.toml`, member crate versions, the git tag and the release notes agree, replacing today's three-way disagreement between branch `release/v0.7.0`, `Cargo.toml` 0.6.0 and tag v0.5.1.
  2. Every workspace crate declares the same valid Rust edition and `cargo build --workspace` succeeds under it, ending the `edition = "2024"` / `"2021"` split.
  3. `cargo audit` and `cargo deny` report no high or critical advisories, and every ignored advisory carries a written rationale plus a migration or review note — no silent suppressions.
  4. A developer following QUICKSTART on a clean machine reaches a working agent, and the elapsed time is recorded against the documented under-15-minute target (measured for the first time, pass or fail).
  5. CI on the release branch proves the full gate suite: format, clippy with warnings as errors, workspace tests, doc tests, all 22 examples, the multi-arch Docker build inside its size and time budget, and the Kubernetes smoke test inside its startup budget.
**Plans**: TBD

</details>

**Milestone 2-3 close-out — Phases 5-6**

### Phase 5: Milestone 2-3 Ground Truth
**Goal**: A developer can open `.planning/` and get a truthful account of what Epics 11-24 delivered — which of the 118 requirements the shipped tree satisfies, which of two competing surfaces each feature actually implements, and what the three unverified open-checkbox blocks contain — with the epic-numbering defect corrected at its source so it stops propagating.
**Depends on**: Phase 1 (RECON-07 must exist before VERIFY-05 can extend it; the rest of Phase 5 is independent of Phases 2-4)
**Requirements**: VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06
**Success Criteria** (what must be TRUE):
  1. A developer can look up any of the 118 Milestone 2-3 requirement IDs and see a `file:line`-cited verdict — satisfied, diverged, partial, or genuinely outstanding — instead of a PRD path that predates the nine-crate workspace and no longer resolves.
  2. Asking "is Epic 22 / Epic 14 / Epic 24 done?" returns a written verdict per block rather than a checkbox count, and the answer is reproducible from the tree — closing the last three blocks `intel/code-verification.md` left unverified.
  3. Reading `RELEASE_NOTES_MILESTONE_3.md` no longer teaches the wrong epic numbers: Epics 19-24 name Herald consolidation, Vision, Autonomous, Battalion hardening, CLI/Config and Test hardening, its Milestone-4 forward-look is marked superseded, and the two claims verified absent from the tree (`RoutingStrategy::PerformanceBased`, the divergent Council and Maneuver API forms) are corrected or withdrawn.
  4. A developer choosing a vision entry point can see that both `vision_llm_port.rs` and `vision_port.rs` ship deliberately, which one to use, and whether Epic 13's encryption-at-rest requirement was dropped on purpose — instead of inferring a migration from a PRD conflict.
  5. One coverage number and one scope survive across all four competing positions (80 / 85 / 75-layered / 80-Epic-24), the two module-scoped gates are placed relative to it, and the ~78% measured figure can be judged pass or fail against it.
  6. A CI run with no API keys behaves the way one recorded decision says it should — loud failure or clean skip — and `llm_live_api_tests.rs` matches that decision rather than contradicting the PRDs that specified it.
**Plans**: TBD

### Phase 6: Verified Gap Closure
**Goal**: Every Milestone 2-3 gap that verification actually proved is closed or explicitly deferred with a recorded reason — and no shipped surface is removed without a decision behind it.
**Depends on**: Phase 5
**Requirements**: CLOSE-01, CLOSE-02, CLOSE-03
**Success Criteria** (what must be TRUE):
  1. A Grove battalion configured with Anthropic or DeepSeek routes through *that* provider: the hardcoded `model: "gpt-4"` at `grove_service.rs:537` is gone, the routing model comes from configuration, and a test proves a non-OpenAI model reaches the LLM call.
  2. `grep -rn 'TODO' crates/paladin-battalion/src/` returns nothing that Epic 22's completion criteria claimed was already resolved.
  3. Every item VERIFY-02 classified as genuinely outstanding across Epics 14, 22 and 24 is either passing in `cargo test --workspace` or recorded as deferred with a written reason — and if verification found nothing outstanding, that verdict is recorded rather than the requirement quietly dropped.
  4. The live-API test harness and both vision surfaces match their Phase 5 recorded decisions in code, with any removal carrying a migration note.
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

Phase 5 depends on Phase 1 only (for RECON-07). If Milestone 2-3 ground truth becomes urgent
before Phases 2-4 complete, Phase 5 can run directly after Phase 1 without violating any
dependency.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Ground Truth & Decision Records | 0/TBD | Not started | - |
| 2. Functional Gap Closure | 0/TBD | Not started | - |
| 3. Verification Depth | 0/TBD | Not started | - |
| 4. Release Coherence | 0/TBD | Not started | - |
| 5. Milestone 2-3 Ground Truth | 0/TBD | Not started | - |
| 6. Verified Gap Closure | 0/TBD | Not started | - |

## Not In This Roadmap

Deliberate omissions, so a later reader does not mistake them for oversights:

- **Shipped Milestone-1 work.** 98% of the milestone's task items are done. The per-requirement
  record is the *Milestone 1 as-shipped ledger* in `REQUIREMENTS.md`; re-planning it as phases
  would be fiction.
- **Shipped Milestone 2-3 work — which is nearly all of it.** Sanctum and RAG (Epics 11-12),
  Sentinel vision (Epics 13, 20), autonomous planning and handoffs (Epics 14, 21), Conclave
  (Epic 15), Council and Grove (Epic 16), the Maneuver Flow DSL (Epic 17), the CLI consolidation
  and enhancement (Epics 17.5, 18), Herald consolidation (Epic 19), the Paladin registry and
  Commander metadata export (Epic 22), the scheduler port and CLI configuration wiring (Epic 23)
  and the test/benchmark hardening (Epic 24) all have shipped artefacts in the tree. They are
  recorded in the *Milestone 2-3 as-shipped ledger*, not re-planned. Phase 5 verifies the record;
  it does not rebuild the features.
- **Open checkbox counts as a backlog.** 542 items are unchecked across 75 task lists. The two
  largest blocks — Conclave 129 and Sanctum 111 — are verified shipped. Only the three blocks
  `intel/code-verification.md` explicitly left unverified become work, and only after VERIFY-02
  says so.
- **A migration between the two shipped vision surfaces.** Both ship;
  `intel/code-verification.md` records this as coexistence and says to confirm intent first.
  VERIFY-04 records the intent; no phase assumes one surface loses.
- **Resolving the 16 competing variant groups.** Recording answers is in scope (RECON-02 …
  RECON-07, VERIFY-03 … VERIFY-06). Picking winners inside `REQUIREMENTS.md` is not — the user has
  stated that variants are expected and that settling past disagreements is not the goal of this
  ingest.
- **Deferred test suites.** The Milestone-1 CLI end-to-end deferral is effectively closed by
  Epic 23's mock provider and Tier-1 suites; what remains disputed is the live-API skip semantics
  (VERIFY-06), not the suites themselves.
- **Tech-debt refactors.** Service-file decomposition and clone/lock-contention work are tracked
  as v2. The second of those is blocked on Phase 3 producing benchmark evidence first.
- **Milestones 4-12, Deferred-QA-CICD-Completion and project-management.** Awaiting ingest runs
  3-5. Notably, the nine-crate workspace layout that makes every run-1 and run-2 `src/...` path
  historical is itself a Milestone 5 deliverable arriving in run 3 — so the paths get their
  requirements then, not before.

## Roadmap Extension Protocol (ingest runs 3-5)

This roadmap is **appended to**, not restructured. Later runs merge Milestones 4-12,
Deferred-QA-CICD-Completion and project-management into the same `.planning/intel/` files.

When a later ingest run adds scope:

1. **Do not renumber or rewrite Phases 1-6.** Phases 1-4 are Milestone 1 close-out; Phases 5-6 are
   Milestone 2-3 close-out. New phases start at **Phase 7** and continue upward. Use decimal
   insertions (e.g. 2.1) only for urgent work that must execute *between* existing integer phases.
2. **Keep the milestone-grouped form.** Add a row to the `## Milestones` table, a labelled block
   under `## Phases`, and a new expanded `## Phase Details` section for the incoming phases. Wrap
   completed or superseded milestone sections in a `<details>` block labelled with their milestone
   and status. Keep the `### Phase N: Name` header format verbatim — downstream tooling parses it,
   including inside `<details>`.
3. **Add new requirement ID prefixes; do not recycle.** `RECON-*`, `GAP-*`, `QUAL-*` and `REL-*`
   are spent on Milestone 1. `VERIFY-*` and `CLOSE-*` are spent on Milestone 2-3. Ingested `REQ-*`
   IDs are stable merge keys — match on them rather than re-deriving.
4. **Expect supersession.** Zero locked decisions exist as of run 2 (0 ADR, 0 SPEC across 81
   documents), and later milestones deliberately restructure earlier ones. Run 2 already produced
   eight documented supersessions of run-1 requirements — see *Superseded but preserved* in
   `REQUIREMENTS.md`. An ADR arriving in a later run outranks anything asserted in these phases;
   when that happens, record the supersession in `PROJECT.md` Key Decisions rather than silently
   editing a phase.
5. **Re-check the ledgers, not the phases.** If a later run's documents claim earlier work is
   incomplete, verify against shipped code and update the relevant as-shipped ledger in
   `REQUIREMENTS.md`. Two runs have now independently found task-list checkboxes understating
   shipped reality — assume the next batch does too. Precedence for this project is
   **shipped tree > `.planning/codebase/` map > `intel/code-verification.md` > PRD > DOC >
   task-list checkbox.**
6. **Path claims in old PRDs are historical.** Every `src/core|application|infrastructure` path in
   the run-1 and run-2 corpus predates the nine-crate workspace. Resolve current locations through
   `.planning/codebase/` or the tree, never through a PRD.

---
*Roadmap created: 2026-07-30 (ingest run 1 of 5 — `.project/Milestone_1-MVP`, 36 docs)*
*Extended: 2026-07-30 (ingest run 2 of 5 — `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs; Phases 5-6 added, Phases 1-4 unchanged)*
