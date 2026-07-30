---
gsd_state_version: '1.0'  # placeholder; syncStateFrontmatter overwrites on first state.* call
status: planning
progress:
  total_phases: 8
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

Phase: 1 of 8 (Ground Truth & Decision Records)
Plan: 0 of TBD in current phase
Status: Ready to plan
Last activity: 2026-07-30 — ingest run 3 of 5 merged (`.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements`, 32 docs); Phases 7-8 appended, Phases 1-6 unchanged

**Note on ordering:** Phase 7 has no hard dependencies and is the cheapest phase to run early. Two of its recorded answers are inputs to Phase 4 (ARCH-03(a) → REL-02 on the Rust edition; ARCH-04 → REL-01 on whether Milestone 6 forces a major version bump), and DEBT-01 … DEBT-04 in Phase 8 are verified, small and independent of everything. See the Execution Order notes in `ROADMAP.md`.

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

Decisions are logged in PROJECT.md Key Decisions table — **currently empty by evidence**: 113
ingested documents surfaced 0 ADR-typed and 0 SPEC-typed docs, so nothing is locked. Everything
asserted in the ingested PRDs and DOCs is supersedable, including by shipped code. Run 2 proved
this with eight documented supersessions of run-1 requirements; run 3 produced eleven more,
including the whole monolith → workspace path migration.

First entries expected from Phase 1 (six ADRs, one per competing variant pair), Phase 5 (four
recorded answers) and Phase 7 (six more).

**Two ADR candidates now exist, neither entered as a locked decision:**

1. **`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`** (run 3) — the
   only decision/options pair in all 263 documents. `Status: Approved`, `Decision Date: 2026-05-13`,
   `Chosen Option: Option A`, with a Rationale, a Rejected Options section and an implementation
   checklist. Settles where `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError` and
   `HandoffError` live; shipped code implements it. Manifest-typed **DOC**, so a PRD published two
   days later outranks it — and that PRD's FR-10 ("types must not be split across crates") would
   undo the fix. **Strongest candidate in the corpus, and the one with real consequences if left
   unprotected.** Two caveats: it settles *location* for five types only, and despite its filename
   it **never mentions `BattalionResult`**.
2. **`Epic_17.5/epic17-5.md`** (run 2) — the CLI belongs in `src/application/cli` because "CLI is an
   input adapter in the application layer, not infrastructure". Already applied in code (`src/cli`
   is absent from the tree), also outranked by a PRD that says otherwise.

Promoting either requires re-tagging the source document via `--manifest` and re-running ingest.
Entering them here would fabricate authority the corpus does not contain.

**Decisions applied by direction, not derived** (ingest run 3, from the user):

1. The workspace decomposition SHIPPED. All crates exist and are documented in the codebase map.
   No forward phases were created for Milestone 5 extraction work.
2. The Milestone 6 relocations SHIPPED (`application_settings.rs` deleted, orchestration services
   under `src/application/services/`, Maneuver DSL under `crates/paladin-battalion/src/maneuver/`,
   `CircuitBreaker` under `src/infrastructure/resilience/`). Not re-planned.
3. The `battalion-result-upward-dependency-decision.md` pair is recorded accurately and **not**
   overclaimed: it creates no locked decision, it settles the location of five value/error types,
   and it does **not** resolve the run-1 `BattalionResult` field-set variant.
4. The verified open defects from the run-3 verification ARE genuine forward work and became
   DEBT-01 … DEBT-05. Stale open-checkbox counts did not.

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

- **Checkbox state is the least reliable signal in this project — and it is wrong in both
  directions.** Precedence is **shipped tree > `.planning/codebase/` > `intel/code-verification.md`
  > PRD > DOC > checkbox.** Runs 1 and 2 found checkboxes *understating* shipped reality (Chain of
  Command and Herald wiring; Conclave 129 open and shipped; Sanctum/Qdrant 111 open and shipped).
  Run 3 found the first *accurate* count — Milestone 4's 20 open items, corroborated by zero
  `#[deprecated]` annotations in the tree — **and** the first count that *overstates* completion:
  Milestone 4 Epic 3's CLI-isolation list is fully checked while three CLI-only dependencies remain
  unconditional. Verify each count against the tree before implementing anything.
- **Five verified open defects in Milestone 4-6 scope, all small, all confirmed against the tree
  on 2026-07-30.** (1) The `api-surface` CI job fails on every run: `ci.yml:171,181,186` and both
  `scripts/{check-api-surface,extract-public-api}.sh` defaults point at
  `project/current-exports.txt`, but the directory was renamed in commit `928c6d5` and the baseline
  lives at `.project/current-exports.txt` — so the only automated public-API guard is inert, and
  `check-deprecations.sh` never runs. (2) `grep -rn '#\[deprecated' src crates` returns 0 against
  Milestone 4 Epic 2 FR-8. (3) `crates/paladin-ports/Cargo.toml:18` sets `[lib] doctest = false`
  deferring the fix to an unwritten "Task 7.0", and `ci.yml:225` excludes the crate from `--doc`.
  (4) `structopt`, `colored` and `comfy-table` are still unconditional root dependencies.
  (5) Three `TokenUsage` structs ship (`token_usage.rs:13`, `battalion/mod.rs:497`,
  `llm_analysis_service.rs:51`). Tracked as DEBT-01 … DEBT-05.
- **Two structural questions gate Milestone 4-6 planning rather than its content.** The
  milestone/tier numbering collision (the Milestone 4-6 overviews number themselves "Milestone
  1/2/3" by refactoring tier, and PRDs cross-reference "Milestone 1 / Epic 2" meaning Milestone 4
  Epic 2) → ARCH-02; and the Milestone 6 facade re-export policy, where the overview requires
  backward-compatible re-exports and both PRDs forbid them, which decides whether Milestone 6 was a
  breaking change requiring a major version bump → ARCH-04.
- **Five documented positions would break things if applied literally**: `vision` gating
  `chacha20poly1305`/`zeroize` (would break `cargo build --no-default-features`), the MCP transport
  feature flags, `web-server` gating actix-web, a `paladin-cli` crate, and
  `src/application/use_cases/` as the orchestration home. All five are contradicted by shipped
  code → ARCH-05.
- **One verified open defect in Milestone 2-3 scope.** `grove_service.rs:537` builds its routing
  request with `model: "gpt-4".to_string(), // TODO: Make configurable` in production code
  (`#[cfg(test)]` begins at line 732), so Grove routing ignores the configured provider. This is the
  same defect class Epic 21 removed elsewhere, and it means Epic 22's "all inline TODOs resolved"
  criterion is unmet. Tracked as CLOSE-01.
- **Three open-checkbox blocks still unverified** — Epic 22 hardening (81), Epic 14 autonomous
  (45), Epic 24 test hardening (29). These are the only run-2 blocks `code-verification.md` leaves
  unchecked, and they are *claims*, not work. VERIFY-02 resolves them; CLOSE-02 acts on whatever
  they prove.
- **20 competing variant groups / 38 entries preserved unmerged** across runs 1-3 (6 groups from
  run 1, 10 from run 2, 4 from run 3). No winners picked — deliberately, and at the user's explicit
  direction. Recording answers is RECON-02 … RECON-07, VERIFY-03 … VERIFY-06, ARCH-03 and ARCH-04.
  Highest-consequence now: **ownership of `PaladinResult` / `StopReason` / `TokenUsage`** (group
  19 — the one place where mechanical precedence gives the architecturally wrong answer, because a
  PRD outranks an Approved-status decision record and its FR-10 would reintroduce the upward
  dependency the decision removed), the coverage gate (4 positions), the handoff tool name and
  parameters (3 names / 2 parameter sets), the Grove routing threshold (3 names / 3 defaults), and
  the `paladin-core` dependency allowlist (declared exhaustive at 6, ships 14).
- **Three run-1/run-2 variants were CLOSED by run-3 code verification** — recorded as facts about
  the tree, not decisions. `BattalionResult` resolves to a merged superset at `battalion/mod.rs:549`
  satisfying all three consumers (so RECON-03 became a recording task and GAP-07 lost its code
  change); `BattalionConfig` resolves to the Epic 4 form exactly and `CommanderConfig` does not
  exist anywhere, collapsing the three-owner `metadata_output_dir` warning to one owner; and the
  competing `ErrorStrategy` variant sets turned out to be two distinct enums in two crates, which
  Milestone 6 physically separated. No entry was deleted.
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
- **All `src/...` paths in the run-1 and run-2 corpus are historical — and several run-3 paths
  are too.** Those PRDs assume a single-crate layout; the workspace was decomposed in Milestone 5
  (run 3) into what is now **ten library crates plus a `doc-examples` crate plus the root
  `paladin-ai` facade** — not the "9-crate workspace" this planning set previously recorded, and not
  the six the Milestone 5/6 overviews assume. Milestone 6 then moved several things Milestone 5 had
  just placed (the Maneuver parser out of `paladin-core`, `CircuitBreaker` into infrastructure), and
  the Milestone 6 Epic 2 PRD's own target directory `src/application/use_cases/` no longer exists.
  Resolve locations through `.planning/codebase/` or the tree, never through a PRD.
- **Five shipped crates have no ingested requirement yet** — `paladin-herald`, `paladin-storage`,
  `paladin-content`, `paladin-notifications`, `paladin-web`. Run 4 is expected to supply them.
- **Version metadata disagrees three ways**: branch `release/v0.7.0`, `Cargo.toml` 0.6.0 (root
  package and every workspace crate path dependency), tag v0.5.1. REL-01 converges them, but
  ARCH-04's answer on whether Milestone 6 was a breaking change determines what they converge *to*.
- **Edition is mixed and the documents disagree too**: root plus nine crates on `edition = "2024"`,
  `crates/paladin-ports` and `crates/paladin-notifications` on `"2021"`. Milestone 5 Epics 1-4
  require 2021; Epic 5 and the milestone overview require 2024. ARCH-03(a) records the answer,
  REL-02 applies it.
- **Four CI jobs still use the deprecated `actions-rs/toolchain@v1`** (`ci.yml:147,317,507`,
  `integration-tests.yml:71`) against Milestone 5 Epic 6's "low-risk improvement that should not be
  deferred". Folded into DEBT-01 because `ci.yml:147` is the `api-surface` job.
- **No `.planning/config.json`** — granularity `standard` and sequential phase IDs assumed in both
  runs. Phase IDs are plain (`Phase 5`, `Phase 6`), not milestone-prefixed and not project-coded.
- **2 more ingest runs pending** (run 4: Milestones 7-8; run 5: Milestones 9-12 +
  Deferred-QA-CICD-Completion + project-management). Follow the Roadmap Extension Protocol; new
  phases start at **Phase 9**; do not restructure Phases 1-8. All eight forward ID prefixes
  (`RECON`, `GAP`, `QUAL`, `REL`, `VERIFY`, `CLOSE`, `ARCH`, `DEBT`) are spent.
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
| Scope | Milestones 7-12 feature work | Awaiting ingest runs 4-5 | Ingest run 1, narrowed runs 2-3 |
| Tech debt | `paladin-core` / `paladin-ports` dependency allowlists brought back in line with reality (declared 6 and 7; ship 14 and 10) | Deferred to v2 — the architectural invariant holds; this is document-versus-code drift. Needs ARCH-03(b) to choose a direction | Ingest run 3 |
| Tech debt | `retry`, `rate_limiter` and `bulkhead` primitives in `src/infrastructure/resilience/`, plus consolidating the retry logic in `mcp_sse_adapter.rs` and `api_content_deliverer.rs` | Deferred — explicitly scoped out by Milestone 6 Epic 4, which shipped the module scaffold only | Ingest run 3 |
| Tech debt | Full `user_service` relocation out of `src/core/platform/manager/` (with `UserServiceFactory`, `user_config.rs`, user CLI commands, user API controller, `SqliteUserRepository`) | Deferred — Milestone 6 Epic 2 scoped it out and flagged it for "a future Epic" | Ingest run 3 |
| Scope | A `paladin-cli` workspace crate | Out of scope — the Milestone 5 overview's target structure named it, the Epic 6 PRD's non-goal rejected it, and the code agrees with the PRD (a `cli` feature plus `[[bin]] paladin-cli`) | Ingest run 3 |
| Scope | MCP feature flags (`mcp-arsenal` / `mcp-transports` / `mcp-stdio` / `mcp-sse`) | Out of scope — eliminated by a dated 2026-04-15 PRD note; Arsenal and its transports compile unconditionally | Ingest run 3 |
| Scope | A `paladin-infra` crate, and a `CircuitBreakerPort` trait abstraction | Out of scope — both explicitly rejected by Milestone 6 Epic 4, which accepted the resulting layering inversion as a pragmatic trade-off inside the facade crate | Ingest run 3 |

## Session Continuity

Last session: 2026-07-30
Stopped at: ingest run 3 of 5 merged into PROJECT.md, REQUIREMENTS.md, ROADMAP.md and STATE.md — 115 run-3 requirements recorded in a new Milestone 4-6 as-shipped ledger, 8 new variant entries preserved unmerged across 4 new groups, 11 supersession chains recorded, Phases 7-8 appended (12 new requirements: ARCH-01 … ARCH-07, DEBT-01 … DEBT-05). Three earlier requirements were narrowed by shipped-code verification rather than renumbered — RECON-02, RECON-03, GAP-07 — and REL-02 gained the exact edition state.
Resume file: None
Next ingest run: 4 of 5 — Milestones 7-8 (production hardening, facade cleanup and shim resolution). Expected to supply the requirements behind the five workspace crates that currently ship without one, and — per `intel/code-verification.md` — the Milestone 8 deferred documents are one of the two places where the genuine remaining-work signal lives.
