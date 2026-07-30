# Roadmap: Paladin

## Overview

Paladin already works. It ships at v0.7.0 with a Cargo workspace of ten library crates plus a
`doc-examples` crate and the root `paladin-ai` facade, 22 runnable examples, a 112 MB multi-arch
Docker image and reference Kubernetes manifests. Across the 113 documents ingested so far, 7,511 of
8,053 task items are checked (93%) — and the shipped tree is *ahead of* even that figure.
**This roadmap does not build the framework; it closes out milestones that already shipped and
makes the planning record match the code.**

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

Milestone 4-6 close-out is different in kind, because these three milestones **restructured what
Milestones 1-3 built** — the feature-flag surface and port-trait hardening (4), the workspace
decomposition (5), and four layer relocations (6). All of it shipped, and unusually for this
corpus it was verified directly against `Cargo.toml` contents and type definitions rather than
inferred: ten library crates exist, `application_settings.rs` is deleted, the orchestrators moved
to `src/application/services/`, the Maneuver DSL sits in `paladin-battalion`, and `CircuitBreaker`
sits in `src/infrastructure/resilience/`. So Phase 7 records what shipped, corrects the five
documented positions the code contradicts, and gives one answer each to the four competing variant
pairs and the two open policy questions. Phase 8 is the first phase in this roadmap whose scope is
**entirely verified defects** rather than verification: a CI job that has been failing silently on
a stale path, an API-deprecation requirement with zero annotations in the tree, port doctests
disabled behind an unwritten follow-up task, three CLI dependencies still compiled into library
builds, and three `TokenUsage` structs where the record says there should be one.

## Milestones

| Milestone | Phases | Status | Source |
|---|---|---|---|
| **Milestone 1 close-out** | 1-4 | Not started | Ingest run 1 — `.project/Milestone_1-MVP` (36 docs) |
| **Milestone 2-3 close-out** | 5-6 | Not started | Ingest run 2 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs) |
| **Milestone 4-6 close-out** | 7-8 | Not started | Ingest run 3 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements` (32 docs) |
| *Milestones 7-8* | TBD | Awaiting ingest run 4 | Production hardening, facade cleanup and shim resolution |
| *Milestones 9-12 + Deferred-QA + project-management* | TBD | Awaiting ingest run 5 | Classic orchestrator, CI hardening, docs overhaul, Web API |

Later runs append new milestone sections and continue phase numbering upward — see
*Roadmap Extension Protocol* at the end of this file. Milestone numbering follows the
**directory / task-list numbering** (Milestone 4 = Tier 1, 5 = Tier 2, 6 = Tier 3); the "Milestone
1 / 2 / 3" titles inside those three source milestones are refactoring-tier labels and are not used
as keys anywhere in this file (ARCH-02).

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

**Milestone 4-6 close-out**

- [ ] **Phase 7: Workspace Ground Truth & Recorded Answers** - Record what the refactor milestones actually shipped, correct the five positions the code contradicts, and answer the four variant pairs and two policy questions
- [ ] **Phase 8: Verified Defect Closure** - Fix the five defects verification proved open: the broken API-surface CI job, missing deprecations, disabled port doctests, leaked CLI dependencies, and duplicate `TokenUsage`

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

<details>
<summary><strong>Milestone 2-3 close-out — Phases 5-6 (not started)</strong></summary>

### Phase 5: Milestone 2-3 Ground Truth
**Goal**: A developer can open `.planning/` and get a truthful account of what Epics 11-24 delivered — which of the 118 requirements the shipped tree satisfies, which of two competing surfaces each feature actually implements, and what the three unverified open-checkbox blocks contain — with the epic-numbering defect corrected at its source so it stops propagating.
**Depends on**: Phase 1 (RECON-07 must exist before VERIFY-05 can extend it; the rest of Phase 5 is independent of Phases 2-4)
**Requirements**: VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06
**Success Criteria** (what must be TRUE):
  1. A developer can look up any of the 118 Milestone 2-3 requirement IDs and see a `file:line`-cited verdict — satisfied, diverged, partial, or genuinely outstanding — instead of a PRD path that predates the workspace decomposition and no longer resolves.
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

</details>

**Milestone 4-6 close-out — Phases 7-8**

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
**Plans**: TBD

### Phase 8: Verified Defect Closure
**Goal**: The five defects that direct code verification proved open are fixed, so the guards the project believes it has actually work — and no shipped surface is removed without a recorded decision behind it.
**Depends on**: Phase 7 (ARCH-03(c) decides which crate owns the canonical `TokenUsage` before DEBT-05 consolidates to it; the other four DEBT items are independent and can start immediately)
**Requirements**: DEBT-01, DEBT-02, DEBT-03, DEBT-04, DEBT-05
**Success Criteria** (what must be TRUE):
  1. An intentional change to the public API makes CI fail and an unchanged tree makes it pass — the `api-surface` job stops exiting 1 with "No baseline found" on every run, which it has done since the `project/` → `.project/` rename, and `check-deprecations.sh` gets to execute.
  2. `grep -rn '#\[deprecated' src crates` either returns the transitional types Milestone 4 Epic 2 requires, each naming its replacement and its removal version, or `DEPRECATIONS.md` and the stable-API page record that the requirement was withdrawn and why — with no third state where the document promises a deprecation timeline the tree cannot start.
  3. `cargo test --workspace --doc` runs `paladin-ports` instead of excluding it, and the port traits' rustdoc examples compile — restoring executing documentation to the ~25 traits that are the framework's primary integration contract.
  4. A downstream project depending on `paladin` compiles no CLI crates: `cargo tree --lib --no-default-features` shows none of `structopt`, `colored` or `comfy-table`, and the library-only build is genuinely library-only rather than 5-of-8 isolated.
  5. `grep -rn 'pub struct TokenUsage' crates src` returns exactly one result, and token figures no longer need conversion when they cross the battalion/ports boundary.
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8

Real dependencies are looser than the numbering, and three couplings run *backwards* through it.
Recorded here so neither side gets planned twice:

- **Phase 5 depends on Phase 1 only** (for RECON-07). If Milestone 2-3 ground truth becomes urgent
  before Phases 2-4 complete, Phase 5 can run directly after Phase 1.
- **Phase 7 depends on nothing.** It is the cheapest phase to run early, and two of its answers are
  inputs to Phase 4: ARCH-03(a) records which Rust edition the workspace standardises on, which
  REL-02 then applies; ARCH-04 records whether Milestone 6 was a breaking change, which determines
  what REL-01's single version story should be. **Whichever phase executes first records the
  answer; the other applies or ratifies it.** Running Phase 7 before Phase 4 avoids Phase 4 having
  to invent both answers.
- **Phase 8 depends on Phase 7** for one requirement only (ARCH-03(c) → DEBT-05). DEBT-01 through
  DEBT-04 are independent, verified, and small — they are the highest confidence-per-effort work in
  this roadmap and do not need to wait for anything.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Ground Truth & Decision Records | 0/TBD | Not started | - |
| 2. Functional Gap Closure | 0/TBD | Not started | - |
| 3. Verification Depth | 0/TBD | Not started | - |
| 4. Release Coherence | 0/TBD | Not started | - |
| 5. Milestone 2-3 Ground Truth | 0/TBD | Not started | - |
| 6. Verified Gap Closure | 0/TBD | Not started | - |
| 7. Workspace Ground Truth & Recorded Answers | 0/TBD | Not started | - |
| 8. Verified Defect Closure | 0/TBD | Not started | - |

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
- **Shipped Milestone 4-6 work — which is all of it except five defects.** The Cargo workspace
  and every crate extraction (Milestone 5, six epics), the feature-flag matrix and CLI feature gate
  (Milestone 4 Epics 1 and 3), and all four Milestone 6 relocations — the `application_settings.rs`
  decomposition, the orchestration-service move to `src/application/services/`, the Maneuver DSL
  co-location in `paladin-battalion`, and the `CircuitBreaker` move to
  `src/infrastructure/resilience/` — are **verified shipped against the tree**, not merely claimed.
  No forward phase re-plans them. Phase 7 records them; Phase 8 fixes only what verification
  proved broken.
- **Milestone 5's and Milestone 6's open checkbox counts.** Milestone 5 shows 17 open and the
  crates, prelude, CI isolation job and benchmark report all exist; Milestone 6 shows 0 open and
  code agrees. Milestone 4's 20 open items are the exception — they *are* real, they are all in
  `tasks-harden-port-traits-stable-api.md`, and DEBT-02 is what they amount to.
- **`STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`.**
  Absent from the paths six run-3 documents name, but shipping as mdbook chapters after the
  Milestone 11 overhaul. Recording the relocation is ARCH-05; building them would be duplicate
  work.
- **A `paladin-cli` crate, MCP transport feature flags, and `vision` gating the encryption
  crates.** Three documented positions that shipped code contradicts. The last would break
  `cargo build --no-default-features` if implemented, because `chacha20poly1305` and `zeroize`
  serve user auth and Citadel encryption, not vision.
- **Resolving the 20 competing variant groups.** Recording answers is in scope (RECON-02 …
  RECON-07, VERIFY-03 … VERIFY-06, ARCH-03, ARCH-04). Picking winners inside `REQUIREMENTS.md` is
  not — the user has stated that variants are expected and that settling past disagreements is not
  the goal of this ingest. Where shipped code settles a variant, that is recorded as a **fact about
  the tree**, at the top of the precedence order, not as a decision taken here.
- **Promoting the two ADR candidates.** The corpus now holds two: Epic 17.5's CLI-location
  decision (run 2) and the Milestone 5 Epic 1 `battalion-result-upward-dependency-decision.md`,
  which carries `Status: Approved` and a `Chosen Option` but is manifest-typed DOC. Promoting
  either requires re-tagging the source document via `--manifest` and re-running ingest.
  Manufacturing the lock inside a planning artefact would fabricate authority the corpus does not
  contain. ARCH-03(c) records the recommendation; it does not act on it.
- **Deferred test suites.** The Milestone-1 CLI end-to-end deferral is effectively closed by
  Epic 23's mock provider and Tier-1 suites; what remains disputed is the live-API skip semantics
  (VERIFY-06), not the suites themselves.
- **Tech-debt refactors.** Service-file decomposition and clone/lock-contention work are tracked
  as v2. The second of those is blocked on Phase 3 producing benchmark evidence first.
- **Milestones 7-12, Deferred-QA-CICD-Completion and project-management.** Awaiting ingest runs
  4-5. Five shipped crates — `paladin-herald`, `paladin-storage`, `paladin-content`,
  `paladin-notifications` and `paladin-web` — still have no ingested requirement; run 4 is expected
  to supply it. Run 3 supplied the requirements behind the other five and the workspace manifest
  itself, which is why every run-1 and run-2 `src/...` path is now formally recorded as
  historical.

## Roadmap Extension Protocol (ingest runs 4-5)

This roadmap is **appended to**, not restructured. Later runs merge Milestones 7-12,
Deferred-QA-CICD-Completion and project-management into the same `.planning/intel/` files.

When a later ingest run adds scope:

1. **Do not renumber or rewrite Phases 1-8.** Phases 1-4 are Milestone 1 close-out; Phases 5-6 are
   Milestone 2-3 close-out; Phases 7-8 are Milestone 4-6 close-out. New phases start at
   **Phase 9** and continue upward. Use decimal insertions (e.g. 2.1) only for urgent work that
   must execute *between* existing integer phases.
2. **Keep the milestone-grouped form.** Add a row to the `## Milestones` table, a labelled block
   under `## Phases`, and a new expanded `## Phase Details` section for the incoming phases. Wrap
   completed or superseded milestone sections in a `<details>` block labelled with their milestone
   and status. Keep the `### Phase N: Name` header format verbatim — downstream tooling parses it,
   including inside `<details>`.
3. **Add new requirement ID prefixes; do not recycle.** `RECON-*`, `GAP-*`, `QUAL-*` and `REL-*`
   are spent on Milestone 1. `VERIFY-*` and `CLOSE-*` are spent on Milestone 2-3. `ARCH-*` and
   `DEBT-*` are spent on Milestone 4-6. Ingested `REQ-*` IDs are stable merge keys — match on them
   rather than re-deriving.
4. **Expect supersession, and record the chain.** Zero locked decisions exist as of run 3 (0 ADR,
   0 SPEC across 113 documents), and later milestones deliberately restructure earlier ones. Run 2
   produced eight documented supersessions of run-1 requirements; run 3 produced eleven more,
   including the entire monolith → workspace path migration and one requirement
   (`REQ-core-container-extraction`'s Maneuver clause) that a later milestone reversed outright.
   See *Superseded but preserved* in `REQUIREMENTS.md`. **Relocation is not contradiction.** An ADR
   arriving in a later run outranks anything asserted in these phases; when that happens, record
   the supersession in `PROJECT.md` Key Decisions rather than silently editing a phase.
5. **Re-check the ledgers, not the phases.** If a later run's documents claim earlier work is
   incomplete, verify against shipped code and update the relevant as-shipped ledger in
   `REQUIREMENTS.md`. Precedence for this project is **shipped tree > `.planning/codebase/` map >
   `intel/code-verification.md` > PRD > DOC > task-list checkbox.**
6. **Checkbox counts cut both ways — verify each one.** Runs 1 and 2 found checkboxes
   *understating* shipped reality (Conclave 129 open and shipped; Sanctum 111 open and shipped).
   Run 3 found the first count that was *accurate* (Milestone 4's 20 open items, corroborated by
   zero `#[deprecated]` annotations in the tree) **and** the first that *overstated* completion
   (Milestone 4 Epic 3's fully-checked CLI isolation, with three dependencies still unconditional).
   Never convert a count into a requirement without checking the tree.
7. **Path claims in old PRDs are historical, including some run-3 ones.** Every
   `src/core|application|infrastructure` path in the run-1 and run-2 corpus predates the workspace
   decomposition, and several run-3 paths were moved again by Milestone 6 or by milestones arriving
   in runs 4-5 (the PRDs say `src/application/use_cases/`; the code says
   `src/application/services/`). Resolve current locations through `.planning/codebase/` or the
   tree, never through a PRD.
8. **Milestone numbers in source documents are not always milestone numbers.** The Milestone 4-6
   overviews number themselves by refactoring tier ("Milestone 1", "Milestone 2", "Milestone 3"),
   and Milestone 3's release notes assign Epics 19-23 to four Milestone 2 features. In both cases
   the directory / task-list numbering is authoritative here. Expect a third instance.

---
*Roadmap created: 2026-07-30 (ingest run 1 of 5 — `.project/Milestone_1-MVP`, 36 docs)*
*Extended: 2026-07-30 (ingest run 2 of 5 — `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs; Phases 5-6 added, Phases 1-4 unchanged)*
*Extended: 2026-07-30 (ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` +
`.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements`,
32 docs; Phases 7-8 added, Phases 1-6 unchanged. Three earlier requirements were **narrowed** by
shipped-code verification rather than renumbered — RECON-02, RECON-03 and GAP-07 — and REL-02
gained the exact edition state; those edits are recorded in `REQUIREMENTS.md`.)*
