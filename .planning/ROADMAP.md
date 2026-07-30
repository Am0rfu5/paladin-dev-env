# Roadmap: Paladin

## Overview

Paladin already works. It ships at v0.7.0 with a 9-crate Rust workspace, 1,091 passing tests as
last measured, 22 runnable examples, a 112 MB multi-arch Docker image and reference Kubernetes
manifests — 1,817 of 1,857 Milestone-1 task items are complete (98%). **This roadmap does not
build the framework; it closes out Milestone 1.**

The journey is therefore short and specific. First make the planning record match the shipped code
and give each of the six contested type/gate definitions one recorded answer (Phase 1). Then close
the residual functional gaps that verification exposes and apply those recorded definitions in code
(Phase 2). Then make the quality numbers real rather than aspirational — coverage to the recorded
gate, error paths actually executed, benchmarks re-enabled with documented baselines (Phase 3).
Finally make the release coherent: one version, one edition, a defensible advisory posture,
reviewed docs, and the whole gate suite green in CI (Phase 4).

**Milestone framing:** Phases 1-4 are **Milestone 1 close-out** (ingest run 1 of 14). Milestones
2-12, Deferred-QA-CICD-Completion and project-management arrive in ingest runs 2-14 and append as
new milestone sections starting at Phase 5 — see *Roadmap Extension Protocol* at the end of this
file.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Ground Truth & Decision Records** - Verify the planning record against shipped v0.7.0 code and record one answer per competing variant pair
- [ ] **Phase 2: Functional Gap Closure** - Finish the residual Milestone-1 functionality and apply the recorded definitions in code
- [ ] **Phase 3: Verification Depth** - Make coverage, error-path testing and performance baselines real and measured
- [ ] **Phase 4: Release Coherence** - One version, one edition, defensible dependencies, reviewed docs, green gate suite

## Phase Details

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
  5. `cargo bench` completes across all five restored suites and a baseline document records throughput, P50/P95/P99 latency, memory per Paladin and startup time — so the next performance change can be compared against something.
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

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Ground Truth & Decision Records | 0/TBD | Not started | - |
| 2. Functional Gap Closure | 0/TBD | Not started | - |
| 3. Verification Depth | 0/TBD | Not started | - |
| 4. Release Coherence | 0/TBD | Not started | - |

## Not In This Roadmap

Deliberate omissions, so a later reader does not mistake them for oversights:

- **Shipped Milestone-1 work.** 98% of the milestone's task items are done. The per-requirement
  record is the *Milestone 1 as-shipped ledger* in `REQUIREMENTS.md`; re-planning it as phases
  would be fiction.
- **Code with no ingested requirement.** Sanctum vector search, Grove / Council / Conclave, the
  Maneuver flow DSL, the Axum HTTP API, notifications, the scheduler and the content pipeline all
  exist in the workspace but their requirements live in Milestones 2-12, which have not been
  ingested. They get phases when they get requirements — not before.
- **Deferred test suites.** Live-provider-API tests (Epic 6 task 7.0) and CLI end-to-end tests
  (Epic 9 tasks 13.4-13.6) stay deferred. Phase 1 records the deferral; it does not reverse it.
- **Tech-debt refactors.** Service-file decomposition and clone/lock-contention work are tracked
  as v2. The second of those is blocked on Phase 3 producing benchmark evidence first.

## Roadmap Extension Protocol (ingest runs 2-14)

This roadmap was written to be **appended to**, not restructured. Later runs merge Milestones 2-12,
Deferred-QA-CICD-Completion and project-management into the same `.planning/intel/` files.

When a later ingest run adds scope:

1. **Do not renumber or rewrite Phases 1-4.** They are Milestone 1 close-out. New phases start at
   **Phase 5** and continue upward. Use decimal insertions (e.g. 2.1) only for urgent work that
   must execute *between* existing integer phases.
2. **Convert this file to the milestone-grouped form** on first append: add a `## Milestones`
   section at the top, wrap Phases 1-4 in a `<details>` block labelled with their milestone and
   status, and add a new expanded milestone section for the incoming phases. Keep the
   `### Phase N: Name` header format — downstream tooling parses it.
3. **Add new requirement IDs; do not recycle.** `RECON-*`, `GAP-*`, `QUAL-*` and `REL-*` are spent
   on Milestone 1. Ingested `REQ-*` IDs are stable merge keys — match on them rather than
   re-deriving.
4. **Expect supersession.** Zero locked decisions exist as of run 1, and later milestones
   deliberately restructure earlier ones. An ADR arriving in a later run outranks anything asserted
   in these four phases; when that happens, record the supersession in `PROJECT.md` Key Decisions
   rather than silently editing a phase.
5. **Re-check the ledger, not the phases.** If a later run's documents claim Milestone-1 work is
   incomplete, verify against shipped code and update the *Milestone 1 as-shipped ledger* in
   `REQUIREMENTS.md`. The 2026-01 task lists were already stale on arrival; assume the next batch
   is too.

---
*Roadmap created: 2026-07-30 (ingest run 1 of 14 — `.project/Milestone_1-MVP`, 36 docs)*
