# Phase 5: Milestone 2-3 Ground Truth - Context

**Gathered:** 2026-08-04
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision
below carries the reasoning that produced it; none was confirmed by a human. Review before planning
if any answer looks wrong.

<domain>
## Phase Boundary

Make `.planning/` a cited, truthful account of what Milestones 2 and 3 (Epics 11-24) actually
shipped, and give the four questions those milestones left open exactly one recorded answer each.

**Three deliverable classes:**

1. **A cited status ledger** (VERIFY-01, VERIFY-02) — `.planning/ledgers/milestone-02-03.md`, with a
   `file:line`-cited verdict for all 118 run-2 requirement IDs, plus a written verdict for each of
   the three open-checkbox blocks `intel/code-verification.md` leaves unverified.
2. **Three new ADRs** (VERIFY-03, VERIFY-04, VERIFY-06) — epic numbering, the two vision surfaces
   plus the encryption-at-rest question, and live-API-test missing-key behaviour.
3. **One in-place ADR amendment** (VERIFY-05) — ADR-0006's coverage answer extended across the two
   positions run 2 added, with the module-scoped gates placed and the two inherited items
   dispositioned.

Plus **one in-repo source correction**: `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`,
the document that propagates the epic-numbering defect.

**This phase writes records and decisions. It does not change product code.** Where a recorded
answer has a code consequence, it is flagged and Phase 6's CLOSE-03 executes it. The one exception is
`.project/`, `.planning/` and roadmap/requirements source corrections — correcting the record at its
source is this phase's whole point.

**Not in this phase:** fixing `grove_service.rs:537`'s hardcoded model (Phase 6, CLOSE-01); closing
whatever VERIFY-02 classifies as outstanding (Phase 6, CLOSE-02); applying the VERIFY-04/VERIFY-06
answers in code (Phase 6, CLOSE-03); wiring any coverage floor into CI (Phase 15, PIPE-02);
re-measuring coverage (Phase 15); Milestone 4-6 ground truth (Phase 7).

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phase 1 — locked, not re-litigated

These are recorded here so downstream agents do not re-derive them from the archived Phase 1
CONTEXT.md. Source: `.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-CONTEXT.md`.

- **D-00a:** ADRs live in `.planning/decisions/`, one file per decision, flat sequential numbering.
  **0001-0009 are taken.** Phase 5 allocates **0010, 0011, 0012** (D-14 below). *(Phase 1 D-01)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02)*
- **D-00c:** Every ADR carries a `Code Conformance` field valued `conforms` or `must change`, and
  where it is `must change`, names the requirement that executes it. *(Phase 1 D-03)*
- **D-00d:** Ledger is a new file per milestone — `.planning/ledgers/milestone-02-03.md` — with
  REQUIREMENTS.md's `## Milestone 2-3 as-shipped ledger` section reduced to a pointer.
  *(Phase 1 D-17)*
- **D-00e:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers. *(Phase 1 D-18)*
- **D-00f:** Verdict classes are `satisfied` · `present, unproven` · `genuinely outstanding` ·
  `deferred with reason` · `superseded by shipped code`. *(Phase 1 D-20)*
- **D-00g:** Ledgers are **amended in place** — when a later plan's measured result contradicts a
  row, that row is edited directly with the new verdict, the command or `file:line` that produced
  it, and the date. Never a separate corrections file. Superseded text is retained, not deleted.
  *(Phase 2 CONTEXT.md D-02, which names Phases 5/7/10/13 as inheritors.)*

### Ledger depth & evidence (VERIFY-01)

- **D-01:** **No row gets `satisfied` without Phase 1's full evidence bar** — a `file:line` citation
  **plus** a named passing test, example, or command that exercises it. A citation with nothing
  exercising it is `present, unproven`. This applies to all 118 rows without exception, including
  the ~40 that the run-2 ingest already recorded as `Shipped` or `Shipped (relocated)`: an ingest
  `Shipped` verdict **is** the bare "the code exists" claim that D-19 exists to reject.
  Chosen over accepting existing `Shipped` rows as pre-satisfied (would import the exact
  false-positive class this bar was written for) and over a lowered two-tier bar (turns the ledger
  into two documents with two meanings under one verdict vocabulary).

- **D-02:** **Triage directs effort, not the bar.** Rows already carrying a divergence note or a
  `Verify —` verdict in REQUIREMENTS.md's run-2 ledger get first-pass depth (find the exercising
  artefact, cite it, or record why none exists). Rows recorded `Shipped (relocated)` get a citation
  refresh against the current crate layout first, then the same exercising-artefact search. The
  difference is ordering and time budget, not the standard applied.

- **D-03:** **Expect a large `present, unproven` bucket and budget for it.** Phase 1 predicted this
  at Milestone-1 scale (~40 requirements) and it materialised. Milestone 2-3 is 118 requirements
  across 14 epics with a documented pattern of checkbox state understating reality in both
  directions. A large third bucket is the honest outcome, not a plan deviation.

- **D-04:** **Both systematic path caveats are recorded once at the head of the ledger, not repeated
  per row.** (a) Every `src/core|application|infrastructure` path in the run-2 PRDs predates the
  Milestone 5 workspace decomposition — the current layout is in the *Milestone 4-6 as-shipped
  ledger* and `.planning/codebase/STRUCTURE.md`. (b) The Milestone-1 benchmark files those PRDs
  reference have been relocated into per-crate `benches/` directories. A row whose only divergence
  is one of these two caveats is **not** a divergence — it is `superseded by shipped code` with a
  pointer to the head note.

### VERIFY-02 — the three unverified blocks

- **D-05:** **One written verdict per block, justified by a parent-task cluster table.** The unit of
  verification is the task list's own **parent-task heading**, not the individual checkbox. For each
  block, cluster the open items under their parent tasks, verify each cluster's distinct capability
  claim against the tree, and record a `parent task → verdict → evidence` table as the block
  verdict's backing. This satisfies VERIFY-02's "verdict per block, **not** a task list derived from
  checkbox arithmetic" while still giving CLOSE-02 a **named** scope instead of a mood.
  The three blocks: `tasks-epic22-battalion-commander-hardening.md` (81 open),
  `tasks-autonomous-agent-features.md` (45 open), `tasks-test-hardening-benchmarks-qa.md` (29 open).

- **D-06:** **A block is `satisfied by shipped code` only if every parent-task cluster verifies.** If
  any cluster fails, the block verdict is `partially outstanding` and the failing clusters are named
  — **those named clusters are exactly CLOSE-02's scope and nothing else.** If all three blocks
  verify clean, VERIFY-02 records that verdict and CLOSE-02 closes with a recorded "no work
  required" per REQUIREMENTS.md, rather than being deleted.

### VERIFY-03 — the epic-numbering defect

- **D-07:** **Two deliverables, because VERIFY-03 asks for two things.** "Recorded once and
  permanently" is **ADR-0010**; "the defective source document is corrected in-repo" is an edit to
  `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`. The ADR is what downstream phases
  cite; the document edit is what stops the defect propagating to the next reader.

- **D-08:** **The release-notes correction is annotation, not rewriting. Superseded text is
  retained.** A dated correction banner at the top of the file names what was wrong and points at
  ADR-0010; each defective claim is corrected inline with the original text kept and marked
  superseded. This reuses the pattern ROADMAP.md already uses on itself
  (`**Amended by Phase 4, dated 2026-08-03, citing …**`) and matches D-00g's retain-don't-delete
  convention. `.project/` is this project's historical ingest corpus — silently rewriting history
  would destroy the provenance five ingest runs were built on.
  Three corrections land in that file: (a) Epics 19-24 renumbered to the authoritative
  plan/epic-definition set — 19 Herald & Domain Type Consolidation, 20 Vision Pipeline Completion,
  21 Autonomous Agent Completion, 22 Battalion & Commander Hardening, 23 CLI/Config/Infrastructure
  Completion, 24 Test Hardening; (b) "What's Next (Milestone 4)" marked a superseded point-in-time
  forward-look (vision shipped); (c) the two claims verified absent from the tree —
  `RoutingStrategy::PerformanceBased` and the divergent Council/Maneuver API forms — corrected or
  withdrawn with the shipped form cited.
  — **Reversibility:** costly — the corrected numbering becomes the provenance key every later
  reader and every Phase 6-16 citation uses. Reverting means re-checking which of two numbering
  schemes each downstream citation meant.

### VERIFY-04 — vision surfaces and encryption-at-rest

- **D-09:** **Both surfaces are intended long-term. Neither is legacy. No migration is planned.**
  They sit at different layers and are reached by different entry points, which is why both ship:
  - `crates/paladin-ports/src/output/vision_llm_port.rs:52` — `VisionCapableLlm: LlmPort`, the
    **provider-capability trait**, reached via `PaladinBuilder::enable_vision`
    (`src/application/services/paladin/paladin_builder.rs:517`). This is the **adapter-author
    surface** — implement it when adding a vision-capable provider.
  - `crates/paladin-ports/src/output/vision_port.rs:47` — `VisionPort`, the **execution-service
    surface**, reached via `PaladinExecutionService::execute_with_vision`
    (`src/application/services/paladin/paladin_execution_service.rs:517`). This is the
    **recommended entry point for application code**.
  The ADR must state that guidance explicitly — a developer choosing an entry point should not have
  to infer it. Neither is deprecated, so **CLOSE-03's vision half is documentation, not removal.**

- **D-10:** **The encryption-at-rest premise is false and the ADR must say so.** REQUIREMENTS.md
  records `REQ-vision-security-encryption` as *"Not found in tree — no encryption-at-rest,
  zeroization or retention-policy artefact was located, and Epic 20's `VisionError` omits
  `EncryptionError`."* **Verified false against the tree during this discussion, on all three
  counts** (see `<specifics>` for the citations). The correct verdict is neither "shipped" nor
  "dropped" but a third thing: **built, self-tested, and never wired** — `EncryptionService`,
  `DataRetentionPolicy` and `VisionError::EncryptionError` have **zero consumers** outside their own
  modules.

- **D-11:** **The roadmap's open-ended risk is therefore closed by evidence, not by a placement
  decision.** ROADMAP.md warns that if Epic 13's encryption requirement was not consciously dropped,
  "that is new security work with no phase home anywhere in Phases 5-16". It is not new work — the
  capability exists. What remains is a small, concrete wiring question, and it gets a **recorded
  decision, not an implementation**, in **Phase 6 under CLOSE-03**: either wire
  `EncryptionService::encrypt_image_data` into the vision path, or record that the service is a
  consumer-facing utility that the framework deliberately does not impose. **No new phase and no new
  requirement is created.** The ledger row for `REQ-vision-security-encryption` changes from
  *"Not found in tree"* to `present, unproven` with the unwired-consumer finding named.

### VERIFY-05 — extending the coverage answer

- **D-12:** **Amend ADR-0006 in place. Do not write a second coverage ADR.** ADR-0006's own ratchet
  clause already specifies in-place amendment, D-00g makes it the house convention, and RECON-07
  exists precisely to eliminate the "choosing between two numbers" failure. A separate ADR-00NN for
  coverage would recreate it.
  The amendment adds: the two run-2 positions dispositioned in `Considered Options`
  (**75% overall with a layered per-tier table** — core ≥ 85 / application ≥ 80 / infrastructure
  ≥ 70 / CLI ≥ 70, from the Milestone 3 plan; and **≥ 80 / ≥ 70 re-asserted by Epic 24**), each
  rejected against the measured 84.79% with the reason stated; the module-scoped gate placement
  (D-13); the two inherited dispositions (D-14a/b); and the falsifiability statement against the
  ~78% figure in the Milestone 3 release notes (D-15).

- **D-13:** **The two module-scoped gates are recorded as targets above the global floor, with
  measured figures and named owners — not converted into hard gates by this phase.**
  Herald ≥ 95% (`REQ-herald-consolidation-quality-gates`) and autonomous components ≥ 90%
  (`REQ-autonomous-completion-quality-gates`) are **explicitly not withdrawn**, per ADR-0006's own
  standing instruction. Each gets its measured figure transcribed from
  `.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-coverage-measurement.md`'s
  per-file rows where one exists — Herald is already recorded at **80.49%**, a live ~15-point gap
  against its 95% target. Enforcement is Phase 15 / PIPE-02's; Phase 5 records the number, the
  scope, and the gap. Phase 5 does not enforce a gate it has no CI to enforce it in.

- **D-14a:** **`src/bin/paladin-server.rs` at 0.00% coverage → `deferred with reason`, owner
  Phase 15 / PIPE-02.** The reason is recorded concretely: closing it requires extracting a `run()`
  seam from `main()`, which is a code change and therefore out of this phase's boundary. The seam
  extraction is named as the prerequisite so Phase 15 inherits a task, not a puzzle. A 0%-coverage
  binary is not allowed to sit silently in the denominator without a record.

- **D-14b:** **ADR-0006's scope stays default-feature workspace-wide; `minio.rs` is recorded as
  outside the gated denominator by construction.** `minio.rs` sits behind `s3-storage`, a
  non-default feature, and the recorded 84.79% was measured without it. Widening the denominator to
  non-default features in Phase 5 would move the 84% floor with no measurement behind it — the exact
  thing ADR-0006 forbids. Whether to add a second, feature-scoped measurement is **Phase 15 /
  PIPE-02's decision**, which is the owner REQUIREMENTS.md already records. One number, one scope,
  per RECON-07.

- **D-15:** **The ~78% Milestone-3 figure is judged, not reconciled.** ADR-0006's amendment states
  plainly: the Milestone 3 release notes' ~78% overall figure **fails** the 84% floor and
  **predates** the measurement that set it — it is a stale historical figure, not a competing
  current one, and it is not re-derived or explained away. This is the same treatment ADR-0006
  already gives the 60.88%/67.79% Milestone-1 baselines.

- **D-16:** **No re-measurement in Phase 5.** The 84.79% / 84% floor recorded 2026-07-31 stands.
  Re-measuring is Phase 15's job, is gated on Docker availability this environment does not have,
  and would put a second number in front of a phase whose purpose is to leave exactly one.

### VERIFY-06 — live-API-test missing-key behaviour

- **D-17:** **The shipped panic stands. `require_api_key` is not changed to skip.** Both positions
  are defensible and REQUIREMENTS.md correctly records that precedence cannot settle it — but the
  tree supplies a synthesis neither PRD saw. The suite is **double-gated**:
  `tests/integration/mod.rs:34-35` declares the module behind `#[cfg(feature = "live-api-tests")]`,
  and all **13** tests carry `#[ignore]`. It is therefore unreachable in a default CI run — a run
  with no API keys never compiles or executes it. **The "graceful skip with a clear message" Epic 23
  FR-23.4.4 and Epic 24 US-24.7 require is supplied by the gating, not by the helper.** The panic
  fires only when a developer has explicitly opted into both the feature and `--ignored`, at which
  point a silent skip would be a **false pass** — exactly the failure the post-Epic-24 cleanup
  reversed it for. The recorded answer preserves both positions' intent rather than picking a winner.

- **D-18:** **The code consequence is documentation, not behaviour — and there is a real defect to
  fix.** `require_api_key`'s doc comment at `tests/integration/llm_live_api_tests.rs:63` opens
  *"Skip test if API key is not present or empty, otherwise return the key"* while the body panics
  on both branches. **The doc comment is the only thing that lies about this harness.** The panic
  messages themselves are already correct — they tell the reader *"To skip this test, don't run with
  --ignored flag"*. CLOSE-03's live-API half is therefore: correct the doc comment, and state the
  double gate as the skip mechanism in the module header. No behavioural change.

### ADR allocation

- **D-19:** **Three new ADRs plus one in-place amendment. VERIFY-01 and VERIFY-02 produce the
  ledger, not ADRs.** Phase 1's precedent: contested positions get ADRs; divergences settled by
  shipped code get ledger rows.
  - **ADR-0010** — Milestone 3 epic numbering (VERIFY-03). Code conformance: `conforms`
    (a documentation defect; the executing work is the D-08 source correction).
  - **ADR-0011** — Vision port surfaces: coexistence, entry-point guidance, and the encryption-at-rest
    disposition (VERIFY-04). Code conformance: `must change`, executed by CLOSE-03.
  - **ADR-0012** — Live-API-test missing-key behaviour (VERIFY-06). Code conformance:
    `must change`, executed by CLOSE-03 (doc-comment correction only, per D-18).
  - **ADR-0006 amended in place** — coverage (VERIFY-05).

### Plan decomposition

- **D-20:** **Scaffold first, then epic fan-out, then blocks, then decisions.** Suggested shape,
  ~10-11 plans, matching Phase 1's proven shape at comparable scale:
  1. **Ledger scaffold** (1 plan) — create `.planning/ledgers/milestone-02-03.md` with the head
     notes (D-04 path caveats, D-01 evidence bar, D-00e primary key), the verdict vocabulary, and
     all 118 row stubs keyed by `REQ-*`. Every later plan appends into a fixed shape.
  2. **Ledger fan-out by epic** (5-6 plans) — Epics 11-18 (Milestone 2) and Epics 19-24
     (Milestone 3), grouped 2-3 epics / ~20 requirements per plan.
  3. **The three VERIFY-02 blocks** (3 plans, one each) — the deepest verification work, and their
     verdicts set Phase 6's entire CLOSE-02 scope, so they must not be diluted into a shared plan.
  4. **ADR-0010 + the release-notes correction** (1 plan) — they share one source document.
  5. **ADR-0011 + ADR-0012 + the ADR-0006 amendment** (1 plan) — three recorded answers, all small,
     all already evidence-complete from this discussion.
  Plan 3 must complete before Phase 6 is planned at all — REQUIREMENTS.md and ROADMAP.md both say
  Phase 6's size is unknowable until VERIFY-02 reports.

- **D-21:** **REQUIREMENTS.md's `## Milestone 2-3 as-shipped ledger` section is reduced to a
  pointer** by the scaffold plan, per D-00d — not left in place as a second, diverging copy.

### Claude's Discretion

- The exact banner wording and inline-correction markup for
  `RELEASE_NOTES_MILESTONE_3.md` (D-08 fixes the pattern — dated banner, retain superseded text —
  not the prose).
- Whether ADR-0011 and ADR-0012 are two files or one combined "run-2 recorded answers" ADR. D-19
  recommends two (one question per ADR, matching 0001-0009); a planner with a reason to combine may.
- How the parent-task cluster tables in D-05 are formatted inside the ledger — nested under the
  block verdict, or as a sibling appendix.
- Whether the `present, unproven` rows are counted and reported as a headline figure in the ledger's
  summary. D-03 predicts the bucket is large; whether that becomes a number the phase reports is not
  specified.
- Ordering within the epic fan-out (D-20 step 2) — whether Milestone 2 or Milestone 3 epics go
  first. No dependency either way.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 5: Milestone 2-3 Ground Truth" (lines 362-389) — the goal, the two
  inherited v0.7.1 items, the encryption open-ended-risk note (**now closed by D-10/D-11**), and the
  six success criteria.
- `.planning/ROADMAP.md` §"Phase 6: Verified Gap Closure" (lines 391-412) — the hard dependency on
  Phase 5's verdicts, and WARN-01.
- `.planning/REQUIREMENTS.md` lines 399-509 — VERIFY-01 … VERIFY-06 and CLOSE-01 … CLOSE-03 in full,
  with their *Derives* provenance. **This is the authoritative statement of scope.**
- `.planning/REQUIREMENTS.md` §"Milestone 2-3 as-shipped ledger" (from ~line 2653) — the 118 run-2
  rows with their component-level verdicts. **This is the input D-01 upgrades and D-21 replaces with
  a pointer.**
- `.planning/MILESTONES.md` — the v0.7.1 close-out record, including the two items this phase
  inherits (`paladin-server.rs` 0.00%, `minio.rs` scope).
- `.planning/STATE.md` §"Deferred Items" — both the v0.7.1 close-out table and the "Carried from
  earlier ingest runs" table, which record the live-API and CLI-e2e items as **un-deferred by run 2**.

### Phase 1 conventions this phase inherits

- `.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-CONTEXT.md` — D-01 … D-21,
  the source of D-00a … D-00f above. Read for the reasoning, not just the conclusions.
- `.planning/milestones/v0.7.1-phases/02-functional-gap-closure/02-CONTEXT.md` §D-02 — the
  amend-in-place ledger convention, which names Phase 5 as an inheritor (D-00g).
- `.planning/ledgers/milestone-01.md` — **the shape to copy.** Head notes, verdict vocabulary,
  primary-key convention, and the two later amendment sections (Phase 2, Phase 3) that demonstrate
  D-00g in practice.
- `.planning/decisions/PROMOTION.md` — the promotion procedure for the eleven existing ADR
  candidates. None is Phase 5's; read before writing 0010-0012 so numbering does not collide.
- `.planning/decisions/0001-battalion-config.md` … `0009-workspace-rust-edition-2024.md` — the ADR
  file shape (`Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`). **0010-0012 must match it.**

### The four recorded answers

- `.planning/decisions/0006-coverage-gate.md` — **amended in place by VERIFY-05 (D-12).** Read in
  full: it already names Phase 5 as the owner of the module-scoped gates, the function-vs-line gap
  (77.34% vs 84.79%), and the Docker-scope narrowing.
- `.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-coverage-measurement.md` —
  the per-file coverage rows D-13 transcribes Herald's 80.49% from. **Transcribe, never re-type.**
- `.planning/INGEST-CONFLICTS.md` — warnings 1, 2, 12, 13, 14 (VERIFY-03); 6, 8 (VERIFY-04); 3 and
  the module-scoped-targets INFO (VERIFY-05); 17 (VERIFY-06).
- `.planning/intel/code-verification.md` §"Vision API surface — BOTH shipped", §"Milestone 3 epic
  numbering", §"Release-notes forward-look is stale", §"Not yet verified" — the run-2 verification
  block. Third in the precedence order.
- `.planning/intel/task-completion-state.md` — the deterministic measurement of all 64 task lists;
  the source of the 81 / 45 / 29 open-item counts D-05 clusters. **Do not re-derive these counts.**

### Source documents this phase corrects or verifies against

- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` — **the document D-08 corrects
  in-repo.**
- `.project/Milestone_3-Completion/Epic_22/tasks-epic22-battalion-commander-hardening.md` — 81 open.
- `.project/Milestone_2-Missing_features/Epic_14/tasks-autonomous-agent-features.md` — 45 open.
- `.project/Milestone_3-Completion/Epic_24/tasks-test-hardening-benchmarks-qa.md` — 29 open.

### Codebase maps

- `.planning/codebase/STRUCTURE.md` — the current crate layout. **Required for D-04(a):** every
  `src/core|application|infrastructure` path in the run-2 PRDs predates it.
- `.planning/codebase/TESTING.md` — the three-tier test strategy and the test-target inventory,
  needed for D-01's exercising-artefact search across 118 rows.
- `.planning/codebase/CONCERNS.md` — including "Grove Service Model Hardcoded" (CLOSE-01, Phase 6).

### Shipped code cited by the decisions above

- `crates/paladin-ports/src/output/vision_port.rs:47` — `VisionPort` (D-09, application entry point).
- `crates/paladin-ports/src/output/vision_llm_port.rs:52` — `VisionCapableLlm: LlmPort` (D-09,
  adapter-author surface).
- `src/application/services/paladin/paladin_builder.rs:517` — `enable_vision`, the `VisionCapableLlm`
  entry point.
- `src/application/services/paladin/paladin_execution_service.rs:517` — `execute_with_vision`, the
  `VisionPort` entry point.
- `crates/paladin-core/src/platform/container/vision.rs:189-212` — `VisionError`, **including the
  `EncryptionError` variant the ingest record says is absent** (D-10).
- `src/infrastructure/security/encryption.rs` — `EncryptionService` (`encrypt_image_data` :200,
  `decrypt_image_data` :217), `SecureData` with `#[derive(Zeroize, ZeroizeOnDrop)]` :68,
  `DataRetentionPolicy::is_expired` :95 (D-10).
- `src/infrastructure/security/mod.rs:44` — the public re-export; `src/infrastructure/mod.rs:47`
  wires the module in.
- `Cargo.toml:134-135` — `chacha20poly1305 = "0.10"`, `zeroize = { version = "1.8", … }`, both
  unconditional (D-10).
- `tests/integration/mod.rs:34-35` — `#[cfg(feature = "live-api-tests")] pub mod llm_live_api_tests;`
  (D-17, the first gate).
- `tests/lib.rs:61` — `pub mod integration;`, the autodiscovered `lib` test target that makes
  `tests/integration/mod.rs` compile at all.
- `tests/integration/llm_live_api_tests.rs:63-88` — `require_api_key`, its lying doc comment, and
  both panic arms (D-17, D-18).
- `Cargo.toml:265` — `live-api-tests = []`.
- `crates/paladin-battalion/src/grove_service.rs:537` — the hardcoded `model: "gpt-4"`. **Phase 6,
  CLOSE-01. Do not fix it here.**

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`.planning/ledgers/milestone-01.md`** — a complete, working instance of exactly the document
  this phase must produce, including two rounds of in-place amendment. Copy the shape; do not
  reinvent it.
- **`.planning/decisions/0001`-`0009`** — nine ADRs in the target format, several of which
  (0004, 0006) show how to record a decision that names a downstream executing phase.
- **REQUIREMENTS.md's existing 118-row run-2 ledger** — already carries per-`REQ-*` verdicts with
  divergence notes. D-01 upgrades it with citations and the evidence bar; it is a starting point,
  not a blank page.
- **`intel/task-completion-state.md`** — all 64 task lists already measured. The 81/45/29 counts
  come from here.
- **`.claude/gsd-core/bin/lib/adr-parser.cjs`** — GSD ships an ADR parser. Phase 1's discretion note
  flagged checking its expected frontmatter; ADRs 0001-0009 shipped without frontmatter, so 0010-0012
  should match the shipped files rather than the parser unless a planner deliberately changes both.

### Established Patterns

- **Precedence is the project's core mechanic** (D-00b), and this phase writes three artefacts that
  sit at the *top* of it. Everything must be legible against the ordering.
- **The dominant corpus pattern is the record understating the tree.** Runs 1-5 found it repeatedly
  (Conclave 129 open / shipped; Sanctum 111 open / shipped; Milestone 8's three open items
  contradicted). D-10 is the fourth instance and the first with a *security* requirement attached.
  A verifier who assumes the ingest record is right will produce a wrong ledger.
- **Retain superseded text; amend in place; date every amendment** (D-00g). Applies to the ledger,
  to ADR-0006, and to the release-notes correction alike.
- **Medieval military ubiquitous language is mandatory** in code, docs and comments — including in
  the ADRs and the ledger.
- **Doc comments carry archaeology, and they also lie.** Phase 1 found the `citadel.rs` placeholder
  by its own doc comment; D-18 found `require_api_key`'s doc comment contradicting its body. Read
  comments *and* bodies, and trust neither alone.

### Integration Points

- **`.planning/ledgers/milestone-02-03.md`** — new file, created by this phase, sibling to
  `milestone-01.md`. Phases 7, 10 and 13 add their own.
- **`.planning/decisions/0010`, `0011`, `0012`** — new files; the numbering sequence continues.
- **`.planning/decisions/0006-coverage-gate.md`** — amended in place, not replaced.
- **REQUIREMENTS.md §"Milestone 2-3 as-shipped ledger"** — reduced to a pointer (D-21).
- **`.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`** — the one file outside
  `.planning/` this phase edits.
- **Phase 6's CLOSE-02** — receives the named parent-task clusters from D-06, and **only** those.
- **Phase 6's CLOSE-03** — receives two doc-level changes: the vision entry-point guidance (D-09),
  the encryption wiring decision (D-11), and the `require_api_key` doc-comment correction (D-18).
- **Phase 15's PIPE-02** — receives `paladin-server.rs`'s `run()` seam (D-14a), the `minio.rs`
  feature-scope decision (D-14b), and the module-scoped gate enforcement (D-13).

</code_context>

<specifics>
## Specific Ideas

**Four findings surfaced during this discussion that the ingest record does not contain.** The
researcher should treat them as verified starting points, not hypotheses — each was read from the
tree during this session.

1. **`REQ-vision-security-encryption` is recorded wrong on all three counts.** REQUIREMENTS.md says
   *"no encryption-at-rest, zeroization or retention-policy artefact was located, and Epic 20's
   `VisionError` omits `EncryptionError`."* Against the tree:
   - `VisionError::EncryptionError(String)` **exists** at
     `crates/paladin-core/src/platform/container/vision.rs:210-212`.
   - `EncryptionService` ships at `src/infrastructure/security/encryption.rs` with
     `encrypt_image_data` (:200) / `decrypt_image_data` (:217) over ChaCha20-Poly1305, and is
     re-exported publicly (`security/mod.rs:44`) and wired (`infrastructure/mod.rs:47`).
   - Zeroization ships — `SecureData` is `#[derive(Zeroize, ZeroizeOnDrop)]` (:68) and key material
     is explicitly zeroized (:161).
   - Retention ships — `DataRetentionPolicy::is_expired` (:95).
   - Both dependencies are declared and **unconditional**: `Cargo.toml:134-135`.

2. **But it has zero consumers.** `EncryptionService`, `DataRetentionPolicy` and
   `VisionError::EncryptionError` are never constructed or called anywhere outside
   `src/infrastructure/security/`. The capability is built and self-tested; nothing uses it. This is
   a sharper verdict than either "shipped" or "dropped", and it is what makes D-11 a small wiring
   decision rather than the new security work the roadmap feared.

3. **The live-API suite is double-gated, which dissolves VERIFY-06's apparent deadlock.**
   `tests/integration/mod.rs:34-35` gates the module on `#[cfg(feature = "live-api-tests")]`, and
   all 13 tests carry `#[ignore]`. The suite compiles only via `tests/lib.rs:61`'s
   `pub mod integration;` (the autodiscovered `lib` test target) and runs only under
   `--features live-api-tests -- --ignored`. **Neither PRD position knew this**, and it is why D-17
   can preserve both intents instead of overruling one.

4. **`require_api_key`'s doc comment contradicts its body.** The comment says *"Skip test if API key
   is not present or empty"*; both match arms `panic!`. The panic *messages* are correct and even
   tell the reader how to skip. This is the entire code consequence of VERIFY-06 — one doc comment,
   not a behavioural reversal.

**Scale note for the planner:** this is 118 requirements across 14 epics, versus Phase 1's ~40
across 10. D-02's triage exists so effort lands on the contested rows first; D-20's fan-out exists so
no single plan carries more than ~20 rows. Phase 1 ran 12 plans at a third of this scale — do not
plan Phase 5 as a small phase because its close-out summary in the roadmap reads as one.

</specifics>

<deferred>
## Deferred Ideas

- **WARN-01 — Herald is not reachable from Campaign, Chain of Command, or the Commander router.**
  Inherited from the v0.7.1 close-out as *"Unassigned — candidate for Phase 6"*. **Not Phase 5's** —
  it is an integration gap, not a record gap. Phase 6 must either adopt it under CLOSE-02/03 or
  record a decision declining it.

- **Nyquist validation for Phases 1-4** — all four `VALIDATION.md` files read `status: draft`.
  Recorded at v0.7.1 close as a coverage TODO. Owner: `/gsd-validate-phase 1`…`4`. Unrelated to this
  phase's scope.

- **An uncommitted working-tree change reverts a shipped v0.7.1 deliverable.**
  `.github/workflows/ci.yml` currently has an uncommitted diff (6 insertions, 50 deletions) that
  removes Phase 4's advisory multi-arch wall-clock rationale and restores a hard
  `::error::` at 300s — a budget MILESTONES.md records as never once met in this repository's
  history (measured 2946s). **Out of Phase 5's scope entirely**, but it should not be committed
  silently: it would falsify a v0.7.1 record. Flagged for the user, not adopted.

- **`crates/paladin-battalion/src/grove_service.rs:537` hardcoded `model: "gpt-4"`** — CLOSE-01,
  Phase 6. The only defect in run-2 scope verified open. Do not fix it while verifying Epic 16's
  ledger rows; record the row and move on.

- **Re-measuring workspace coverage under the Docker-backed `--features integration-tests` scope** —
  ADR-0006 records the narrowing explicitly and assigns it to Phase 15 / PIPE-02. D-16 keeps Phase 5
  out of it.

- **Enforcing the Herald ≥ 95% and autonomous ≥ 90% module gates in CI** — D-13 records the numbers
  and the ~15-point Herald gap; wiring any gate into CI is Phase 15 / PIPE-02.

- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phase 1's deferred list. Belongs with Phase 16's documentation work.

</deferred>

---

*Phase: 5-milestone-2-3-ground-truth*
*Context gathered: 2026-08-04*
