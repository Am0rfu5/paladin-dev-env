---
gsd_state_version: '1.0'  # placeholder; syncStateFrontmatter overwrites on first state.* call
status: planning
progress:
  total_phases: 11
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

Phase: 1 of 11 (Ground Truth & Decision Records)
Plan: 0 of TBD in current phase
Status: Ready to plan
Last activity: 2026-07-30 — ingest run 4 of 5 merged (`.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution`, 40 docs); Phases 9-11 appended, Phases 1-8 unchanged

**Note on ordering — one phase now has a date attached.** Phase 9 carries the only dated item in the
153-document corpus: a formal RustSec risk acceptance with a **2026-09-30** review/expiry target,
roughly two months out, on a repository that gates CI on both `cargo audit` and `cargo deny`.
Numeric order puts it ninth; urgency does not. Phase 10 depends on nothing and feeds three earlier
phases (HARD-06 → SEC-01 on whether `pdf-extract` is reachable; HARD-07 → DEBT-03 on the `cargo doc`
bar; HARD-03 → REL-01 on the version story), so running it before Phase 9 saves SEC-01 from
guessing. Phase 7 remains the cheapest early phase for Phases 1-8, and DEBT-01 … DEBT-04 remain the
highest confidence-per-effort work. FACADE-04 in Phase 11 should land **before ingest run 5** reads
the Milestone 9 candidate list. See the Execution Order notes in `ROADMAP.md`.

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

Decisions are logged in PROJECT.md Key Decisions table — **currently empty by evidence**: 153
ingested documents surfaced 0 ADR-typed and 0 SPEC-typed docs, so nothing is locked. Everything
asserted in the ingested PRDs and DOCs is supersedable, including by shipped code. Run 2 proved
this with eight documented supersessions of run-1 requirements; run 3 produced eleven more,
including the whole monolith → workspace path migration; run 4 produced eleven more still, and the
corpus's first case of a document superseding another document by name.

First entries expected from Phase 1 (six ADRs, one per competing variant pair), Phase 5 (four
recorded answers), Phase 7 (six more) and Phases 9-10 (the RustSec exception set, the licence
posture, the leaf-crate dependency rule, the PDF capability and the `cargo doc` bar).

**Six ADR candidates now exist, none entered as a locked decision. Run 4 added four, and one of
them has an operational cost attached to leaving it untagged:**

0. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) — a formal risk acceptance with
   **owner Platform Security** and **review/expiry target 2026-09-30**. **The only item in all 153
   documents carrying an expiry date.** Nothing else in `.planning/` surfaces that date; SEC-01 is
   what carries it forward. The other three run-4 candidates — `cost-benefit-assessment.md`
   (self-approval block, named approver, 2026-05-25),
   `license-compatibility-decision-checklist.md` (approver `DF3NDR`, 2026-05-28) and
   `facade-cleanup-RECONCILIATION-2026-06-04.md` (an explicit supersession notice that resolved six
   open decisions in execution) — are recorded in PROJECT.md Key Decisions.

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

**Decisions applied by direction, not derived** (ingest run 4, from the user):

1. The RustSec exception sprawl is **genuine forward work**, not a ledger note. Recorded as SEC-01
   with the exact per-file counts read from the tree on 2026-07-30: `rustsec-remediation-plan.md`
   documents 2 risk-accepted advisories (owner Platform Security, expiry 2026-09-30);
   `ci.yml:406` passes exactly those 2 as `--ignore` flags; `.cargo/audit.toml` `[advisories]
   ignore` holds **5**; `deny.toml` `[advisories] ignore` holds **15**. *(The user's brief cited 7
   and 17 — those are the counts of RUSTSEC IDs **mentioned** in each file, which include
   `RUSTSEC-2026-0185` quinn-proto and `-0190` anyhow named in comments as **upgraded rather than
   ignored**. The substance is unchanged and confirmed: four divergent surfaces, 13 suppressions
   with no risk-acceptance record, and a two-month expiry.)* A fifth fact was found during
   verification and added: `ci.yml` runs **two** independent, differently-configured `cargo audit`
   jobs (`:77` bare, `:406` with two inline ignores).
2. The `api-surface` defect was **extended, not duplicated** — DEBT-01 now records the sixth stale
   reference, M8 Epic 7 FR-10, which writes the broken path into an ingested requirement. No new
   requirement was created for it.
3. `facade-cleanup-RECONCILIATION-2026-06-04.md` is the **authoritative account of Milestone 8**,
   superseding the Epic 1 audit and the Epic 3 disposition (HARD-02). Milestone 8 Epic 6 is complete
   despite being recorded "not verified", Epic 3 is complete in substance, and `paladin-herald`
   exists in the tree — which is why the earlier "9 crates" figure was wrong.
4. The v0.1.0-rc.1 release is **history** (HARD-03). No rc.1 artefact is treated as current state,
   and REL-01 must not converge on an rc.1 figure.
5. All 53 competing variants stay **unmerged**. No winners picked. PROJECT.md Key Decisions stays
   empty with its evidence note; the six ADR candidates are named in context only.

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

- **🗓 The only deadline in this project is 2026-09-30, and it is a security acceptance.**
  `Milestone_7/Epic_4/rustsec-remediation-plan.md` formally risk-accepts two advisories
  (`RUSTSEC-2023-0071` rsa, `RUSTSEC-2025-0111` tokio-tar) with **owner Platform Security
  (Milestone 7)** and a **review/expiry target of 2026-09-30** — roughly two months from this
  ingest. It is the only dated item in all 153 documents, and nothing in `.planning/` other than
  SEC-01 surfaces it.
- **The RustSec exception set is encoded four different ways, and `deny.toml` violates its own
  stated invariant.** Verified by direct file reads on 2026-07-30: the plan documents **2**;
  `.cargo/audit.toml` `[advisories] ignore` holds **5** (the 2 plus `RUSTSEC-2026-0187` lopdf via
  `pdf-extract`, `-0194` and `-0195` quick-xml via `rust-s3`/`aws-creds`); `deny.toml`
  `[advisories] ignore` holds **15** (those 5 plus 10 unmaintained notices) under a header claiming
  "the same advisory IDs are mirrored here … Keep these two files in sync"; and `ci.yml` runs
  **two independent `cargo audit` jobs** — `:77` bare (reading audit.toml's 5) and `:406` with the
  original 2 passed inline. `make audit` is bare; `cargo deny check` gates at `:105`.
  **Thirteen of `deny.toml`'s fifteen have no entry in the formal risk-acceptance register** — they
  carry inline reasoning but no owner and no expiry, against acceptance criteria that require both.
  Both tools gate CI. Tracked as SEC-01. *(Note for continuity: counting RUSTSEC IDs **mentioned**
  rather than **suppressed** gives 7 and 17, because `RUSTSEC-2026-0185` quinn-proto and `-0190`
  anyhow are named in comments as upgraded rather than ignored. The `ignore` arrays are 5 and 15.)*
- **A defect is now written into a requirement, not just into code.** M8 Epic 7 FR-10
  (`REQ-web-api-baseline-changelog`) mandates
  `./scripts/extract-public-api.sh project/current-exports.txt` — the path that has been stale since
  commit `928c6d5` renamed `project/` to `.project/`. All five original references are unchanged.
  DEBT-01 was **extended** to cover the requirement text as well as the two script defaults and the
  three workflow lines; no duplicate requirement was created.
- **The licence has three answers and one of them is signed.** A decision checklist with approver
  `DF3NDR` (2026-05-28) and a 551-package inventory records `MIT OR Apache-2.0`; the M7 Epic 4 PRD
  and overview say MIT; the shipped root `Cargo.toml` says `license = "MIT"`. The dual-licence rule
  was the stated basis for accepting `r-efi`'s `MIT OR Apache-2.0 OR LGPL-2.1-or-later`. `deny.toml`
  already follows the checklist. SEC-02 — do not resolve by inference.
- **Two open architecture questions from run 4, both worth surfacing rather than assuming.**
  (1) The extracted-crate dependency rule is stated absolutely — "No extracted crate may depend on
  another extracted crate" — and violated once by `crates/paladin-content`'s optional `paladin-llm`
  edge, which the same PRD's §4.4 anticipated without amending the rule → HARD-05, and the strongest
  SPEC candidate in run 4. (2) `paladin-content` declares `pdf = []` gating **nothing** and the
  facade's `content-processing` omits `pdf` entirely, yet `.cargo/audit.toml` suppresses an advisory
  on the grounds that `pdf-extract` **is** in the graph → HARD-06, which SEC-01 depends on.
- **Three small verified defects on a published crate family.** `crates/paladin-herald/` has a
  README but **no `CHANGELOG.md`**, against a criterion the Epic 4 completion summary records as Met
  (the crate was created after Epic 4 closed) → SEC-04. `Dockerfile.chef:25-33` enumerates nine
  crate manifests and omits `paladin-herald`, so the cache-tightness FR-01 exists to deliver is not
  achieved → SEC-05. And the crates.io name-collision guardrail the publish-verification document
  asked for does not exist; collisions cost Epic 4 two package renames and a NO-GO cycle → SEC-03.
- **Milestone 8 shipped beyond its own planning documents, and two of its epics are complete despite
  their records.** The 2026-06-04 reconciliation found the Epic 1 audit and Epic 3 disposition had
  mis-described ~4,400 LOC of orphaned uncompiled duplicates as "active bridges that stay", then
  executed the relocations Epic 3 had deferred to Milestone 9 — 15 commits, ~10,250 net LOC removed,
  and a new `paladin-herald` crate created inside an Epic whose non-goals forbade exactly that.
  Epic 6 is filed "Not verified; low priority" and is complete; Epic 3 is filed "PUNTED" and is
  complete in substance. Milestone 8's three open checkboxes are contradicted by code. HARD-02
  records the reconciliation as authoritative.
- **`infrastructure-adapter-disposition.md` is a live trap for ingest run 5.** The Epic 3 PRD §6
  designates it "the authoritative cross-reference for the §4.3 M9 flags" — the document Milestone 9
  was meant to read — and it records all 20 rows as "Stays", names two crates that do not exist
  (`paladin-arsenal`, `paladin-sanctum`), and disagrees with its own governing PRD on two rows.
  Milestone 9 is recorded 100% complete. **FACADE-04 must land before run 5 consumes it.**
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
- **28 competing variant groups / 56 entries preserved unmerged** across runs 1-4 (6 groups from
  run 1, 10 from run 2, 4 from run 3, 8 from run 4). No winners picked — deliberately, and at the
  user's explicit direction. Recording answers is RECON-02 … RECON-07, VERIFY-03 … VERIFY-06,
  ARCH-03, ARCH-04, SEC-01, SEC-02 and HARD-01 … HARD-07. **Run 4 is the run where shipped code
  settles the most**: six of its eight new groups carry a `settled-by` pointer, which is a fact
  about the tree rather than a decision. The one genuine surprise is group 23 — the two publish
  dry-run forms turned out to **coexist**, per-crate in `release.yml:410` and workspace-wide in
  `ci.yml:644`, which the documents alone could not reveal.
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
- **~~Five shipped crates have no ingested requirement~~ — closed by run 4.** All ten library
  crates now have one: `paladin-storage`, `paladin-notifications`, `paladin-content` and
  `paladin-web` from M7 Epic 1's extraction PRD and its cost-benefit gate, and `paladin-herald` from
  the 2026-06-04 reconciliation rather than from any PRD. What still ships without a requirement is
  Milestone 12's Axum HTTP API surface (auth, rate limiting, OpenAPI, SSE streaming) — run 5.
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
- **1 more ingest run pending** (run 5: Milestones 9-12 + Deferred-QA-CICD-Completion +
  project-management). Follow the Roadmap Extension Protocol; new phases start at **Phase 12**; do
  not restructure Phases 1-11. All **eleven** forward ID prefixes (`RECON`, `GAP`, `QUAL`, `REL`,
  `VERIFY`, `CLOSE`, `ARCH`, `DEBT`, `SEC`, `HARD`, `FACADE`) are spent. Run 4 also ingested
  `Milestones-8-11_Dependency-Graph.md`, which describes dependencies reaching into Milestones 9-11
  that run 5 supplies — preserved as a historical planning artefact whose dependency semantics and
  release gates are usable but whose schedule is spent (M9 and M10 are 100% complete, M11 92%).
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
| Scope | Milestones 9-12 feature work | Awaiting ingest run 5 | Ingest run 1, narrowed runs 2-4 |
| Tech debt | **D1 — `src/core/` re-export shims** (6 files, 49 facade importers) | **KEEP, by decision** — removal means rewriting 49 files and preserving `platform/mod.rs`'s maneuver/parser injection, which carries real logic. Becomes debt only if a no-alias policy is adopted (ARCH-04) → FACADE-02 | Ingest run 4 |
| Tech debt | **D2 — mis-layered `src/core/platform/manager/` services** (`content_service`, `event_manager`, `user_service`) | Deferred, medium/medium — partly overtaken: reconciliation commit `6704807` found "no user-service split was needed" because `UserServiceTrait` and the DTOs already live in `paladin-core`. Overlaps the run-3 v2 `user_service` relocation item; do not plan twice → FACADE-02 | Ingest run 4 |
| Tech debt | **D3 — entangled Paladin services** (`planning`/`prompt_generation`/`temperature`/`handoff`, ~2,750 LOC) | **KEEP for now**, high/high — needs the `paladin_builder.rs` / `paladin_execution_service.rs` coupling untangled first, and the targets (`paladin-battalion`, `paladin-llm`) are leaf-to-leaf edges gated on HARD-05 → FACADE-02 | Ingest run 4 |
| Tech debt | **D4 — `content_ingestion_service.rs` placement** (~1,211 LOC) | Deferred, medium/medium — M7 Epic 1's PRD listed it as moving to `paladin-content`; the facade kept its own copy. Needs a dependency-coupling review → FACADE-02 | Ingest run 4 |
| Tech debt | **D5 — residual `println!`/`eprintln!`/`dbg!`** | **Verified exact: 17 occurrences across 6 files**, down from ~435 across 36. The register's own quick win; low/low → FACADE-01 | Ingest run 4 |
| Scope | The `paladin user …` CLI command surface (1,065 LOC, 8 subcommands) | Deferred on purpose — it was declared but **never dispatched**, so it compiled and did nothing. Backend intact; reintroduction is "mostly re-wiring", recoverable verbatim from the M8 removal commit on `chore/facade-cleanup-m8-finish` → FACADE-03(a) | Ingest run 4 |
| Scope | The TensorFlow ML adapter and the `ml` feature flag (636 LOC) | Deferred on purpose — a `#[doc(hidden)]` stub nothing consumed. **Reintroduction condition is the load-bearing part**: a dedicated `paladin-ml` leaf crate, never the facade, with the flag on that crate; `MlPort` stays in the workspace → FACADE-03(b) | Ingest run 4 |
| Scope | A future **content-delivery crate** | Reserved by M7 Epic 1 §4.5.2 as the "correct long-term home" for `file_content_repository.rs`; the file was then deleted and no later document mentions the crate. Carried so the idea is not lost silently | Ingest run 4 |
| Scope | `paladin-arsenal` and `paladin-sanctum` crates | Out of scope — named only by a superseded disposition record that contradicts its own governing PRD. Neither exists; Milestone 9 is 100% complete. Triaging the list is FACADE-04 | Ingest run 4 |
| Tech debt | `paladin-core` / `paladin-ports` dependency allowlists brought back in line with reality (declared 6 and 7; ship 14 and 10) | Deferred to v2 — the architectural invariant holds; this is document-versus-code drift. Needs ARCH-03(b) to choose a direction | Ingest run 3 |
| Tech debt | `retry`, `rate_limiter` and `bulkhead` primitives in `src/infrastructure/resilience/`, plus consolidating the retry logic in `mcp_sse_adapter.rs` and `api_content_deliverer.rs` | Deferred — explicitly scoped out by Milestone 6 Epic 4, which shipped the module scaffold only | Ingest run 3 |
| Tech debt | Full `user_service` relocation out of `src/core/platform/manager/` (with `UserServiceFactory`, `user_config.rs`, user CLI commands, user API controller, `SqliteUserRepository`) | Deferred — Milestone 6 Epic 2 scoped it out and flagged it for "a future Epic" | Ingest run 3 |
| Scope | A `paladin-cli` workspace crate | Out of scope — the Milestone 5 overview's target structure named it, the Epic 6 PRD's non-goal rejected it, and the code agrees with the PRD (a `cli` feature plus `[[bin]] paladin-cli`) | Ingest run 3 |
| Scope | MCP feature flags (`mcp-arsenal` / `mcp-transports` / `mcp-stdio` / `mcp-sse`) | Out of scope — eliminated by a dated 2026-04-15 PRD note; Arsenal and its transports compile unconditionally | Ingest run 3 |
| Scope | A `paladin-infra` crate, and a `CircuitBreakerPort` trait abstraction | Out of scope — both explicitly rejected by Milestone 6 Epic 4, which accepted the resulting layering inversion as a pragmatic trade-off inside the facade crate | Ingest run 3 |

## Session Continuity

Last session: 2026-07-30
Stopped at: ingest run 4 of 5 merged into PROJECT.md, REQUIREMENTS.md, ROADMAP.md and STATE.md — 86 run-4 requirements recorded in a new Milestone 7-8 as-shipped ledger (the best-evidenced of the four), 18 new variant entries preserved unmerged across 8 new groups, 11 supersession chains recorded including the corpus's first document-supersedes-document notice, and Phases 9-11 appended (16 new requirements: SEC-01 … SEC-05, HARD-01 … HARD-07, FACADE-01 … FACADE-04). Three earlier requirements were extended in place rather than duplicated — ARCH-01 (crate provenance now supplied), DEBT-01 (a sixth stale reference, inside an ingested requirement) and DEBT-03 (the documentation gate it sits under). Phases 1-8 unchanged and unrenumbered; the Milestone 4-6 detail block was wrapped in `<details>` per protocol item 2 with its `### Phase N:` headers intact.
Resume file: None
Next ingest run: 5 of 5 — Milestones 9-12 plus Deferred-QA-CICD-Completion and project-management. Two things to carry in: `Milestones-8-11_Dependency-Graph.md` was ingested in run 4 and describes hard dependencies reaching into M9-M11 that run 5 supplies (use its semantics and release gates, discard its schedule — M9/M10 are 100% complete, M11 92%); and FACADE-04 should triage the superseded Milestone 9 candidate list **before** run 5 reads it, or run 5 will plan relocations that already happened.
