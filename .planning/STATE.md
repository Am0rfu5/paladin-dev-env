---
gsd_state_version: '1.0'  # placeholder; syncStateFrontmatter overwrites on first state.* call
status: planning
progress:
  total_phases: 6
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-07-30)

**Core value:** A Rust developer can compose and run multi-agent workflows against any supported
LLM provider through stable port abstractions — without their own domain code depending on a
provider, transport, or storage implementation.
**Current focus:** Phase 1 — Ground Truth & Decision Records

## Current Position

Phase: 1 of 6 (Ground Truth & Decision Records)
Plan: 0 of TBD in current phase
Status: Ready to plan
Last activity: 2026-07-30 — ingest run 2 of 5 merged (`.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion`, 45 docs); Phases 5-6 appended, Phases 1-4 unchanged

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Velocity:**
- Total plans completed: 0
- Average duration: —
- Total execution time: —

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| - | - | - | - |

*Updated after each plan completion*

**Recent Trend:**
- Last 5 plans: —
- Trend: —

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table — **currently empty by evidence**: 81
ingested documents surfaced 0 ADR-typed and 0 SPEC-typed docs, so nothing is locked. Everything
asserted in the ingested PRDs and DOCs is supersedable, including by shipped code. Run 2 proved
this concretely with eight documented supersessions of run-1 requirements.

First entries expected from Phase 1 (six ADRs, one per competing variant pair) and Phase 5 (four
recorded answers).

**Strongest ADR candidate in the corpus** — Epic 17.5: the CLI belongs in `src/application/cli`
because "CLI is an input adapter in the application layer, not infrastructure". Already applied in
code (`src/cli` is absent from the tree). Recorded in PROJECT.md Context, deliberately **not**
entered as a locked decision — it has no ADR status field and would fabricate authority the corpus
does not contain. Promote it if the CLI location should be protected from future override.

**Decisions applied by direction, not derived** (ingest run 2, from the user):
1. Milestone 3 epic numbering — the plan/epic-definition numbering is authoritative
   (19 Herald, 20 Vision, 21 Autonomous, 22 Battalion hardening, 23 CLI/Config, 24 Test hardening).
   The `RELEASE_NOTES_MILESTONE_3.md` mapping is a documentation defect and is not used as a
   provenance key anywhere in `ROADMAP.md` or `REQUIREMENTS.md`.
2. Conclave, Council, Grove, Maneuver, Sentinel vision and the Qdrant Sanctum adapter are verified
   shipped. No forward phases or requirements were created for them; they are in the as-shipped
   ledger.
3. The Epic 13 vs Epic 20 vision API surfaces coexist (`vision_llm_port.rs` and `vision_port.rs`
   both exist). Recorded as coexistence, not as a variant awaiting resolution.
4. Open checkbox counts are not a backlog. Only the six blocks listed under "Not yet verified" in
   `intel/code-verification.md` may be recorded as unverified candidates, explicitly labelled.

### Pending Todos

None yet.

### Blockers/Concerns

- **Checkbox state is the least reliable signal in this project.** Precedence is
  **shipped tree > `.planning/codebase/` > `intel/code-verification.md` > PRD > DOC > checkbox.**
  Two ingest runs independently found checkboxes understating shipped reality: run 1 on Chain of
  Command and Herald wiring, run 2 on Conclave (129 open, shipped) and Sanctum/Qdrant (111 open,
  shipped). Verify before implementing anything.
- **One verified open defect in Milestone 2-3 scope.** `grove_service.rs:537` builds its routing
  request with `model: "gpt-4".to_string(), // TODO: Make configurable` in production code
  (`#[cfg(test)]` begins at line 732), so Grove routing ignores the configured provider. This is the
  same defect class Epic 21 removed elsewhere, and it means Epic 22's "all inline TODOs resolved"
  criterion is unmet. Tracked as CLOSE-01.
- **Three open-checkbox blocks still unverified** — Epic 22 hardening (81), Epic 14 autonomous
  (45), Epic 24 test hardening (29). These are the only run-2 blocks `code-verification.md` leaves
  unchecked, and they are *claims*, not work. VERIFY-02 resolves them; CLOSE-02 acts on whatever
  they prove.
- **16 competing variant groups / 30 entries preserved unmerged** across runs 1-2 (6 groups from
  run 1, 10 from run 2). No winners picked — deliberately, and at the user's explicit direction.
  Recording answers is RECON-02 … RECON-07 and VERIFY-03 … VERIFY-06. Highest-consequence:
  `BattalionConfig`/`metadata_output_dir` ownership (3 positions), `BattalionResult`/
  `BattalionMetadata` shape (4 positions), the coverage gate (4 positions), the handoff tool name
  and parameters (3 names / 2 parameter sets), and the Grove routing threshold (3 names /
  3 defaults).
- **Two contradictions are live in shipped code**: `formation.rs:109` rejects fewer than 2 Paladins
  while the Commander's Auto rule routes a single Paladin to Formation; and `require_api_key()` in
  the live-API test harness panics by design, reversing the graceful-skip criterion in both the
  Epic 23 and Epic 24 PRDs.
- **A documentation defect is propagating epic numbers.** `RELEASE_NOTES_MILESTONE_3.md` assigns
  Milestone 3 Epics 19-23 to four Milestone **2** features, and four further documents mislabel
  epics in cross-references. Epic numbers are the corpus's provenance keys, so this misroutes any
  lookup. VERIFY-03 fixes it at the source.
- **Two release-notes claims are verified absent from the tree**:
  `RoutingStrategy::PerformanceBased` with "dynamic learning" (also contradicts Epic 16 non-goal
  NG-3), and the Council/Maneuver API forms that disagree with the shipped surfaces. Do not plan
  against them.
- **A security requirement vanished between PRDs without a recorded decision.** Epic 13 FR-11
  required encryption at rest for temporarily stored image data, memory zeroization and retention
  policies; Epic 20 completed the vision pipeline with none of it and dropped `EncryptionError`
  from the error enum. No artefact for it was found in the tree. VERIFY-04 establishes whether the
  drop was conscious.
- **Quality numbers are below their own gates and the gate has four positions**: 80% (nine
  Milestone-1 PRDs), 85% (unit-test-improvements), 75% overall with a layered per-tier table
  (Milestone 3 plan), 80%/70% re-asserted (Epic 24). Measured: 60.88% unit / 67.79% integration at
  Milestone 1, ~78% overall at Milestone 3. Plus module-scoped gates at 95% (Herald) and 90%
  (autonomous). No performance baseline document exists.
- **Reported test totals are not a monotonic series**: 999 → 1,292 → 1,674 → 1,628 → 853 across
  the corpus. No figure is authoritative; none is used as a gate.
- **All `src/...` paths in the run-1 and run-2 corpus are historical.** Those PRDs assume a
  single-crate layout; the workspace was decomposed into nine crates in Milestone 5, which arrives
  in ingest run 3. Resolve locations through `.planning/codebase/` or the tree.
- **Version metadata disagrees three ways**: branch `release/v0.7.0`, `Cargo.toml` 0.6.0, tag
  v0.5.1.
- **No `.planning/config.json`** — granularity `standard` and sequential phase IDs assumed in both
  runs. Phase IDs are plain (`Phase 5`, `Phase 6`), not milestone-prefixed and not project-coded.
- **3 more ingest runs pending** (run 3: Milestones 4-6; run 4: Milestones 7-8; run 5: Milestones
  9-12 + Deferred-QA-CICD-Completion + project-management). Follow the Roadmap Extension Protocol;
  new phases start at Phase 7; do not restructure Phases 1-6.
- **Hygiene, not planning**: one ingested source document
  (`Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_FIX.md`) contains a plaintext OpenAI
  API key in its body. The value was never copied into any `.planning/` file. The user has confirmed
  it is rotated. Redacting the source document and running a repository-wide secret scan is still
  recommended — the same value may appear in `.env` history or coverage artefacts.

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Testing | Live-provider-API integration tests (Epic 6 task 7.0, 18 subtasks) | **Un-deferred by run 2** — suite ships behind `live-api-tests`; only the skip-vs-fail semantics remain open (VERIFY-06) | Ingest run 1, revised run 2 |
| Testing | CLI end-to-end tests (Epic 9 tasks 13.4-13.6) | **Un-deferred by run 2** — the blocking mock provider shipped (REQ-mock-llm-adapter) along with the Tier-1 CLI suites | Ingest run 1, revised run 2 |
| Testing | Garrison large-conversation perf test (Epic 2 task 9.14) | Deferred — marked future enhancement | Ingest run 1 |
| Testing | Vision and RAG latency targets never measured (single image < 5 s; retrieval < 500 ms p95; extraction < 3 s p95) | Deferred to v2 — no baseline document exists | Ingest run 2 |
| Tech debt | Oversized service file decomposition (2,757 / 2,294 / 1,840 lines) | Deferred to v2 — no ingested requirement | Ingest run 1 |
| Tech debt | Clone/lock-contention optimization | Deferred to v2 — blocked on Phase 3 benchmarks | Ingest run 1 |
| Tech debt | Single-threaded orchestration scheduler (`orchestration/scheduler.rs`) | Deferred to v2 — `tokio-cron-scheduler` is already a dependency and already adapted in `paladin-storage` | Ingest run 2 |
| Scope | MCP WebSocket transport | Deferred — recorded as a known limitation by the Epic 23 completion summary | Ingest run 2 |
| Scope | Garrison semantic search / vector context retrieval in the CLI path (recency-based selection only) | Deferred — Epic 23 known limitation; superseded in spirit by Sanctum | Ingest run 2 |
| Scope | Grove learning from past routing decisions | Out of scope — Epic 16 NG-3; the release-notes `PerformanceBased` claim is verified absent from the tree | Ingest run 2 |
| Scope | Automatic Garrison-to-Sanctum migration | Out of scope — Epic 11 explicit non-goal | Ingest run 2 |
| Scope | Batch vision API | Out of scope — Epic 20 NG-6; concurrency is a Battalion concern | Ingest run 2 |
| Scope | Registry multi-tenancy, persistence, distribution | Out of scope — Epic 22 explicit non-goals | Ingest run 2 |
| Scope | Milestones 4-12 feature work | Awaiting ingest runs 3-5 | Ingest run 1, narrowed run 2 |

## Session Continuity

Last session: 2026-07-30
Stopped at: ingest run 2 of 5 merged into PROJECT.md, REQUIREMENTS.md, ROADMAP.md and STATE.md — 118 run-2 requirements recorded in a new Milestone 2-3 as-shipped ledger, 18 new variant entries preserved unmerged, Phases 5-6 appended
Resume file: None
Next ingest run: 3 of 5 — Milestones 4-6 (crate/feature refactor, workspace decomposition, architectural refinements). This run supplies the requirements behind the nine-crate layout that makes every run-1 and run-2 `src/...` path historical.
