# Phase 11: Facade Residue & Deferred Register Disposition - Context

**Gathered:** 2026-08-08
**Status:** Ready for planning
**Mode:** interactive — six gray areas presented across two rounds, every one answered by a human.

**Read this first.** Phase 11's own verification overturned the premise of one of its four
requirements. FACADE-01's 17 `println!` occurrences are **all rustdoc example lines**, not runtime
stdout in library code (D-01). The register's *count* is exact; its *characterisation* is not. This
changes FACADE-01 from a code-change requirement into an annotate-and-correct-at-source requirement,
and it is the single most important fact a downstream agent can carry into planning.

<domain>
## Phase Boundary

Give everything Milestone 8 deliberately left behind a **decision with an owner** instead of an
effort rating — the five deferred items (D1–D5), the two removed features and their reintroduction
conditions, and the Milestone 9 candidate list the 2026-06-04 reconciliation superseded. Four
requirements, FACADE-01 … FACADE-04.

**Four deliverable classes:**

1. **A per-occurrence disposition for D5** (FACADE-01) — all 17 `println!`/`eprintln!`/`dbg!`
   occurrences across 6 files in `src/application/services/` + `src/infrastructure/`, each recorded
   as deliberate rustdoc-example stdout, with the misleading framing corrected at source in both the
   M8 register and ROADMAP criterion 1.
2. **Four dispositions with owners** (FACADE-02) — D1, D2, D3, D4 each resolving to *do* /
   *defer with a stated trigger* / *withdraw*. **No relocation executes in this phase.**
3. **Two removed features recorded in `.planning/`** (FACADE-03) — the `paladin user …` CLI surface
   and the TensorFlow ML adapter, with their reintroduction conditions intact and durable outside a
   single `.project/` DOC.
4. **A triaged Milestone 9 candidate list** (FACADE-04) — every surviving row of
   `infrastructure-adapter-disposition.md` marked *done* / *not a candidate* / *still open*, and the
   `paladin-arsenal` / `paladin-sanctum` names confirmed real or recorded as artefacts.

**Not in this phase:**

- **Executing any D1–D4 relocation.** D-04 decides each on merit and defers every relocation. A
  "do" verdict here means *the verdict is recorded*, not *the code moves*.
- **Rewriting the 49 `crate::core::` importers.** That is D1's "do" branch and it is deferred (D-05).
- **Splitting or testing `user_service.rs`.** D2's split half is withdrawn (D-06); the testing half
  is DEFER-02 / Phase 15 and is untouched here.
- **Creating `paladin-ml`, `paladin-arsenal` or `paladin-sanctum`.** PROJECT.md lists all three under
  *Out of Scope*. FACADE-03(b) records a *placement condition*; FACADE-04 *triages a list*. Neither
  builds a crate.
- **Changing doctest posture.** The four `rust,ignore` fences are recorded and handed to Phase 15
  (D-03), which Phase 10 already made the owner of doctest posture.
- **Reintroducing the `paladin user …` CLI or the ML adapter.** FACADE-03 closes on *recorded
  deferral with conditions intact*; promotion to scope would be a phase of its own.
- **Any executable `.rs` change.** See D-13 for the boundary and why it is zero, not merely narrow.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9 and 10 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter**. `PROMOTION.md:57` records **0034 as next free** —
  verified this session. *(Phase 1 D-01/D-03, Phase 7 D-00a/D-00h, Phase 9 D-00a, Phase 10 D-00a)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. *(Phase 5 D-08)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02)*
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that produced
  it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10)*
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers. *(Phase 1 D-18, Phase 7 D-00e)*
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17, applied by Phases 8, 9 and 10.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md — standing project-wide convention.)*

**Inherited from Phase 10, specific to this phase:**

- **D-00i:** **ADR-0031's restated invariant governs D3/D4's relocation targets** — *no extracted
  crate may depend on another extracted crate or on the facade in its default build; a non-default
  optional feature may declare such an edge only where the facade opts in explicitly and the
  dependent code is `cfg`-gated.* FACADE-02 does **not** re-litigate whether leaf-to-leaf edges are
  permissible at all — only whether each specific proposed edge is non-default, facade-gated and
  `cfg`-scoped. *(Phase 10 hand-off, `REQUIREMENTS.md:1570-1592`.)*
- **D-00j:** **ADR-0028 records the Epic 3 relocations as already executed** inside Milestone 8 —
  15 commits, net 10,252 LOC removed, range `e5b2011~1..a1e4901`, independently re-measured twice.
  FACADE-02's and FACADE-04's candidate lists **must not re-plan any relocation this range already
  performed**. *(Phase 10 hand-off, `REQUIREMENTS.md:1583-1586`.)*
- **D-00k:** **The M8 Epic 3 §5 non-goal split is asymmetric** — overridden for `paladin-herald`
  (which exists, created by reconciliation commit `66f6c4e`), **still holding for `paladin-ml`**
  (absent; `test -d crates/paladin-ml` exits 1, re-verified this session). `paladin-herald`'s
  existence is **not** licence to create `paladin-ml`. *(Phase 10 hand-off,
  `REQUIREMENTS.md:1594-1610`.)*
- **D-00l:** **ADR-0018 settled the no-re-export-alias posture** for *relocated* types — old paths
  intentionally retired, no `pub use` shims at `src/lib.rs` or `src/prelude.rs`. This is the
  ARCH-04 answer the register says D1 follows from. See D-05 for how far it actually carries.

⚠ **D-00m [inheritance risk — flagged, not blocking]:** **Phase 10 ran `--auto` and none of its nine
decisions were human-confirmed.** ADR-0031 (Phase 10 D-15) — the exact invariant D-00i inherits — is
one of two Phase 10 explicitly flagged `⚠ HUMAN REVIEW`. Phase 11 proceeds on it because D-04 defers
every relocation, so no code depends on its precise form this phase. **A planner must not treat
ADR-0031 as human-ratified**, and any future phase that *executes* a D3/D4 relocation should confirm
it first. Record this in the phase's artefacts; do not silently build on it.

---

### FACADE-01 — the D5 disposition, and the finding that reframes it

- **D-01: All 17 occurrences are rustdoc examples. Every one resolves to "deliberate stdout, annotated" — zero executable code changes.**
  The rule chosen was *default to `log::*`, annotate provable exceptions*. Verified directly this
  session: `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/`
  returns exactly 17 across exactly 6 files (register count **exact**), and **every single line is a
  `///` or `//!` doc-comment line inside a fenced ```` ```rust ```` or ```` ```rust,ignore ```` block.**
  Filtering the grep to non-doc-comment lines returns **nothing**. `println!` is the idiomatic way a
  rustdoc example shows output; converting these to `log::*` would make the documentation worse and
  would break the examples' illustrative purpose. The exception branch therefore fires for all 17.
  **Done when** each of the 17 carries a recorded disposition naming it a rustdoc example, grouped
  per file, with the `file:line` citation per D-00e.

- **D-02: Correct the misleading framing at source — both the M8 register and ROADMAP criterion 1.**
  The register rates D5 "low effort / low risk, the quick win" and ROADMAP criterion 1
  (`.planning/ROADMAP.md:726`) requires each occurrence "either converted to `log::*` or annotated
  with its reason". Both are premised on these being runtime stdout in library code. They are not.
  The register gets a dated correction banner per D-00c with original text retained; ROADMAP
  criterion 1 is amended so it no longer implies a conversion that should not happen. Chosen over
  correcting the register only, because leaving the ROADMAP text intact would keep asserting that
  runtime calls exist and would make the next reader re-derive this finding from scratch.
  — **Reversibility:** costly — amending a phase's own governing success criterion mid-milestone
  means any later audit of Phase 11 reads the amended text, not the text the phase was scoped
  against; the superseded original must stay visible per D-00c/D-00d or the audit trail breaks.

- **D-03: The four `rust,ignore` fences are a recorded finding owned by Phase 15, not work here.**
  `src/application/services/herald/herald_registry.rs:163,182,195,208` sit in
  ```` ```rust,ignore ```` blocks, so those doctests never compile and can drift from the real API
  silently. Phase 10 already routed doctest posture to Phase 15 ("the seven-crate `doctest = false`
  posture — HARD-07 records it; Phase 15 owns changing it"). Record with Phase 15 named as owner;
  do not un-ignore them here. Chosen over fixing them now to avoid a second owner for one subject.

---

### FACADE-02 — D1–D4, decided on merit, relocations deferred

- **D-04: Each of D1–D4 gets a real verdict with a named owner; no relocation executes in this phase.** The ROADMAP goal is "a decision rather than a rating" — that is satisfied by a verdict
  plus an owner plus, where deferred, a concrete trigger. It is *not* satisfied by executing
  relocations, and it is *not* satisfied by a uniform "defer all" that reads as a rating by another
  name. Chosen over "execute what is cheap" because a disposition phase that also refactors has two
  jobs and a much larger blast radius.

- **D-05: D1 — `src/core/` re-export shims: defer, with the trigger stated as a facade-wide no-alias sweep, owner recorded.** Verified this session: `src/core/` is **exactly six files**
  (`mod.rs`, `platform/mod.rs`, `platform/manager/{mod,content_service,event_manager,user_service}.rs`)
  and **49 files import via `crate::core::…`**. Removal is a mechanical path rewrite of those 49
  **plus** preserving `platform/mod.rs`'s maneuver/parser path injection, which carries real logic
  rather than re-exports — so it is not purely mechanical. **ADR-0018 does not settle this by
  itself:** it retired old paths for *relocated types* (`application::use_cases::*`,
  `CircuitBreaker`), which is a different construct from `src/core/`'s surviving re-export layer.
  The planner must record that distinction rather than treating D1 as already-answered by ARCH-04.

- **D-06: D2 — the `user_service.rs` split half is WITHDRAWN, with the reason recorded; the mis-layering verdict for `content_service.rs` and `event_manager.rs` is decided separately.**
  Three facts narrow D2 to almost nothing on the `user_service` axis: reconciliation commit
  `6704807` already found **"no user-service split was needed"** for the controller case because
  `UserServiceTrait` and the DTOs **already live in `paladin-core`**; the *full* `user_service`
  relocation is already carried as a **run-3 v2 tech-debt item**; and Deferred-QA Epic 28 (DEFER-02,
  Phase 15) plans to **test** the same file to ≥ 80%. Withdrawing the split resolves the collision
  REQUIREMENTS.md flags ("do not schedule independently") rather than sequencing it, and frees
  Phase 15 to test the file as-shipped against a stable mock set. `content_service.rs` and
  `event_manager.rs` remain genuine D2 items and get their own verdicts.
  — **Reversibility:** costly — Phase 15 will size DEFER-02's estimate and mock set against the
  unsplit file; reinstating the split later invalidates that sizing and requires DEFER-02 to be
  re-sequenced against a moving target, which is the exact coupling this withdrawal removes.

- **D-07: D3 — entangled Paladin services: defer, gated explicitly on the D-00i test, not on HARD-05 being unanswered.** `src/application/services/paladin/{planning_service,
  prompt_generation_service,temperature_service,handoff_service}.rs`, ~2,750 LOC, tightly coupled to
  `paladin_builder.rs` and `paladin_execution_service.rs`. HARD-05 **is answered** — ADR-0031
  restated the rule, so D3's `paladin-battalion` (planning/handoff) and `paladin-llm`
  (prompt/temperature) targets are legal *on the same terms `paladin-content`'s existing `llm`
  feature already satisfies*. The remaining question is per-edge, not categorical: is each proposed
  edge non-default, facade-gated and `cfg`-scoped? Record the verdict as defer-with-trigger; the
  trigger is the broader refactor the register itself names.

- **D-08: D4 — `content_ingestion_service.rs` placement: defer pending the dependency-coupling review the register already names as the precondition.** `src/application/services/content/
  content_ingestion_service.rs`, ~1,211 LOC. M7 Epic 1's extraction PRD listed it as moving to
  `paladin-content`; the facade kept its own copy. Legal under D-00i on the same terms as D3. The
  review is the trigger; it is not performed in this phase.

---

### FACADE-03 — the two removed features and their reintroduction conditions

- **D-09: Both features are recorded in a `.planning/` register file, and the `paladin-ml` placement condition additionally gets an ADR because it is a contested position.**
  Per D-00g: the CLI surface's status is not contested (it was declared but never dispatched, the
  backend is intact, reintroduction is re-wiring) — that is register material. The **`paladin-ml`
  leaf-crate placement condition is contested**: it is the surviving half of the M8 Epic 3 non-goal
  that `paladin-herald` overrode (D-00k), it is carried **only by a DOC** today, and PROJECT.md lists
  `paladin-ml` under *Out of Scope*. It earns an ADR from 0034. FACADE-03 closes on **recorded
  deferral with conditions intact** — promotion to scope is explicitly not chosen.
  The condition to preserve verbatim: any future TensorFlow adapter goes into a dedicated
  `paladin-ml` **leaf crate** with the `ml` flag on that crate, **never back into the facade**, and
  `paladin_ports::input::ml_port::MlPort` **stays in the workspace** so the integration point does
  not move.

- **D-10: The recovery pointer is the commit SHA `3d48768`, not the branch name.**
  Verified this session: **both removals are a single commit** — `3d48768` (2026-06-04,
  *"chore(facade): remove half-built user CLI + tensorflow ML stub (M8)"*) deletes
  `src/application/cli/commands/user.rs` (**1,065 LOC**) and
  `src/infrastructure/adapters/input/tensorflow_adapter.rs` (**636 LOC**) — both register LOC figures
  **exact**. REQUIREMENTS.md points recovery at "the Milestone 8 removal commit on branch
  `chore/facade-cleanup-m8-finish`"; that branch is **not present as a local or remote ref**, while
  the commit is reachable from several branches including `docs/m8-deferred-items`. Record the
  recoverable form as `git show 3d48768^:src/application/cli/commands/user.rs`. Note also that
  REQUIREMENTS.md attributes only the ML removal to `3d48768` and the CLI removal to a branch — one
  commit did both; correct that at source per D-00c.
  Verified absent today: `src/application/cli/commands/` holds ten modules and `user.rs` is not
  among them; `crates/paladin-ml` does not exist.

---

### FACADE-04 — triaging the Milestone 9 candidate list

- **D-11: Triage into a `.planning/` table; annotate the `.project/` source in place per D-00c.**
  `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md`
  is designated by the Epic 3 PRD §6 as "the authoritative cross-reference for the §4.3 M9 flags".
  It is unsafe on three counts: all 20 rows record "Stays / No change" while the reconciliation
  executed most of its List B; its M9 targets name **two crates that do not exist**
  (`paladin-arsenal` at lines 10/36/81, `paladin-sanctum` at lines 10/54 — located this session);
  and it disagrees with its own governing PRD on two rows (`arsenal/` is an M9 candidate here but
  "No" in the PRD table; `sanctum/` targets `paladin-sanctum` here but `paladin-memory` in the PRD).
  It is also dated `2025-01`, inconsistent with every other M8 document. **Every surviving row gets
  *done* / *not a candidate* / *still open*.** The `arsenal/` rows were never acted on either way,
  so the list is not wholly obsolete — do not blanket-mark it superseded.
  The two crate names resolve as **artefacts of a mis-written table**, not as future crates: PROJECT.md
  *Out of Scope* already records "none exists… named only by a superseded disposition record that
  contradicts its own governing PRD". Record that finding; do not create a crate.

- **D-12: Run the triage against ADR-0028's commit range, not against the disposition record's own claims.** Per D-00j, any row whose relocation falls inside `e5b2011~1..a1e4901` is *done* by
  outcome regardless of what the record says. This is the mechanism that stops FACADE-04 re-planning
  relocations that already happened — the failure mode the requirement was written to prevent.

---

### Cross-cutting

- **D-13: This phase changes zero executable `.rs` code.** D-01 resolved every FACADE-01 occurrence
  to the annotate branch, and D-04 defers every relocation, so nothing in FACADE-01…04 requires an
  executable source edit. Phase 10's boundary was "no `.rs` file is touched"; Phase 11 lands in the
  same place **by finding, not by fiat**. If a planner concludes an executable change is needed, that
  is a signal to re-check D-01 — not to widen the boundary silently. Files this phase *does* write:
  `.planning/decisions/003N-*.md`, a `.planning/` register/triage file, amendments to
  `.planning/ROADMAP.md` and `.planning/REQUIREMENTS.md`, `.planning/ledgers/milestone-07-08.md`,
  and dated banners on three `.project/` documents.

- **D-14: ADR allocation — contested positions only, numbered from 0034, `PROMOTION.md` updated.**
  Per D-00g. Contested and therefore ADR-worthy: the D1–D4 disposition set (FACADE-02) and the
  `paladin-ml` leaf-crate placement condition (FACADE-03b). Not contested and therefore
  register/ledger material: the 17 rustdoc dispositions (FACADE-01), the CLI surface's recorded
  status (FACADE-03a), and the FACADE-04 triage table. Chosen over one-ADR-per-requirement, which
  would manufacture ADRs for things nobody disputes, and over ledger-only, which would bury the
  FACADE-03 reintroduction conditions the requirement specifically wants findable in `.planning/`.
  The planner allocates exact numbers and updates `PROMOTION.md:57` (`Next free ADR number`) as its
  last act, per the procedure at `PROMOTION.md:141-150`.

### Claude's Discretion

- **Exact ADR count and numbering within 0034+.** D-14 fixes the *rule*; the planner picks whether
  the D1–D4 set is one ADR or several, and assigns numbers.
- **The `.planning/` home and filename for the FACADE-03 register and the FACADE-04 triage table** —
  whether they are one file or two, and whether they live under `.planning/registers/`,
  `.planning/ledgers/` or alongside the phase. Constraint: findable from `.planning/` without
  reading `.project/`.
- **Plan decomposition and wave assignment.** FACADE-01, FACADE-03 and FACADE-04 are mutually
  independent and can run in parallel; FACADE-02 depends on nothing in this phase either, since
  D-04 defers execution.
- **Whether the FACADE-01 per-file disposition is recorded inline as a source comment or only in
  `.planning/`.** D-13 forbids executable changes; a non-executable comment is the planner's call,
  but `.planning/` is the required home either way.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### This phase's requirements and goal
- `.planning/ROADMAP.md` §`### Phase 11` — goal, five success criteria, dependency declaration.
  **Criterion 1 is at line 726 and is amended by D-02.**
- `.planning/REQUIREMENTS.md:1616-1719` — FACADE-01 … FACADE-04 full text, including the exact
  file list for D5 and the D1–D4 narrowing facts.
- `.planning/REQUIREMENTS.md:1570-1614` — **the two Phase 10 hand-offs** to FACADE-02 and
  FACADE-03(b). Inherited as D-00i, D-00j, D-00k. Do not re-derive.

### ADRs that govern this phase's answers
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — the restated default-build
  invariant that legalises D3/D4's targets. `Downstream Consumers` names Phase 11 / FACADE-02
  explicitly. **⚠ Authored under Phase 10 `--auto`, flagged `⚠ HUMAN REVIEW`, never human-ratified
  (D-00m).**
- `.planning/decisions/0028-m8-reconciliation-authoritative.md` §`## Decision (iii)` — the 15-commit,
  10,252-LOC range `e5b2011~1..a1e4901`. The triage baseline for D-12.
- `.planning/decisions/0018-m6-facade-reexport-policy.md` — the no-re-export-alias posture (ARCH-04).
  Bears on D1 but **does not settle it** — see D-05.
- `.planning/decisions/PROMOTION.md:57` — `Next free ADR number: 0034`. Procedure at lines 141-150.

### The Milestone 8 registers being disposed of
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` — D1–D5. Highest-fidelity
  document in the corpus by measurement. **Receives a dated correction banner per D-02.**
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` — the `paladin user …`
  CLI surface and the TensorFlow ML adapter, with the reintroduction conditions FACADE-03 must
  preserve.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md`
  — the 20-row M9 candidate list FACADE-04 triages. `paladin-arsenal` at lines 10/36/81,
  `paladin-sanctum` at lines 10/54.

### Prior-phase context and precedent
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-CONTEXT.md` — D-00a…D-00h
  inherited verbatim; the `⚠ HUMAN REVIEW` flags this phase carries forward as D-00m.
- `.planning/ledgers/milestone-07-08.md` — the `REQ-m8-deferred-items-register`,
  `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`,
  `REQ-adapter-disposition-record` and `REQ-m8-epic3-no-extractions` rows this phase closes against.
- `.planning/PROJECT.md` §`### Out of Scope` — records `paladin-arsenal`, `paladin-sanctum` and
  `paladin-ml` as non-deliverables. **Binding on FACADE-03(b) and FACADE-04.**
- `CLAUDE.md` + `.github/instructions/rust.instructions.md` — no `unwrap`/`expect`/`panic!` in
  library code, ubiquitous-language rule, conventional commits.

### Forward coupling
- **DEFER-02 (Phase 15)** — `user_service.rs` testing. D-06 withdraws D2's split half specifically
  so Phase 15 can size against the unsplit file. Phase 15 must be told.
- **Phase 15 / doctest posture** — receives the four `rust,ignore` fences per D-03.

</canonical_refs>

<code_context>
## Existing Code Insights

### Verified ground truth (re-measured this session, 2026-08-08)

- **The 17 D5 occurrences — all rustdoc, zero runtime.**
  `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` → **17**,
  across **6** files. Filtering out `///` and `//!` lines → **0 remaining**. Per file:
  - `src/application/services/herald/herald_registry.rs` — 4 (lines 165, 184, 197, 210), all in
    ```` ```rust,ignore ```` fences
  - `src/infrastructure/resilience/circuit_breaker.rs` — 6 (42, 44, 46 in the `//!` module doc;
    305, 306, 307 in a `///` item doc)
  - `src/application/services/paladin/paladin_execution_service.rs` — 3 (43, 44, 466)
  - `src/infrastructure/adapters/arsenal/mcp_protocol.rs` — 2 (26, 246)
  - `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` — 1 (22)
  - `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` — 1 (32)
- **Logging facade is `log`, not `tracing`.** `Cargo.toml:14` pins `log = "0.4.21"` as a workspace
  dependency, `:97` consumes it. **20 files under `src/` use `log::`; zero use `tracing::`**
  (`tracing-subscriber` is declared at `:119` but unused in `src/`). Relevant only as the
  counterfactual — D-01 means no conversion happens.
- **D1's blast radius.** `src/core/` = exactly 6 `.rs` files. `grep -rl "crate::core::" src/` = **49
  files**. `src/core/platform/mod.rs` carries maneuver/parser path injection — real logic, not
  re-exports — so a removal is not a pure path rewrite.
- **FACADE-03 removals.** Commit `3d48768` (2026-06-04) deleted `user.rs` (1,065 LOC) and
  `tensorflow_adapter.rs` (636 LOC) in one commit. `src/application/cli/commands/` now holds 10
  modules (`agent, arsenal, battalion, council, features, maneuver, mod, muster, onboarding,
  setup_check`) — no `user.rs`. `test -d crates/paladin-ml` exits 1.
- **ADR series state.** `.planning/decisions/` holds 0001–0033 plus `PROMOTION.md`; next free 0034.

### Established patterns this phase must follow

- **The ADR file shape** — no frontmatter, seven fixed headings, `Downstream Consumers` naming the
  phase/requirement that inherits it. ADR-0031 is the closest model: it *restates* rather than
  *instructs a change* when the tree already conforms, and it corrects a bad citation inline
  (`§4.4` → `cost-benefit-assessment.md:118`) rather than reproducing it.
- **Dated correction banners** (D-00c) — see how Phase 10 annotated `.project/` sources.
- **Ledger amendment in place** (D-00d) — `.planning/ledgers/milestone-07-08.md`.

### Integration points

- `PROMOTION.md:57` — must be updated last, per its own procedure at lines 141-150.
- `.planning/ledgers/milestone-07-08.md` — five named rows close against this phase.
- `.planning/ROADMAP.md:726` — criterion 1 amended by D-02.

</code_context>

<specifics>
## Specific Ideas

- **"A decision rather than a rating."** This is the phase's own framing from the ROADMAP goal and it
  is the acceptance test for every FACADE-02 output. A verdict without an owner, or a deferral
  without a concrete trigger, is a rating wearing a verdict's clothes and does not close D1–D4.
- **"Nothing in that set is planned twice."** ROADMAP criterion 2 names three specific double-plans
  to avoid: D2's `user_service` half against the run-3 v2 item, the reconciliation's finding that no
  user-service split was needed, and Deferred-QA Epic 28 planning to *test* the same file D2 plans to
  *split*. D-06 resolves all three by withdrawing the split.
- **"Someone asking 'why can I not run `paladin user register`?' finds the answer in `.planning/`."**
  ROADMAP criterion 3, and the usability test for FACADE-03(a)'s record: the surface was 1,065 LOC
  declared but never dispatched, the backend is intact, and reintroduction is re-wiring recoverable
  verbatim from `3d48768` — **rather than concluding it was lost.**
- **The register was right about the count and wrong about the kind.** Worth stating plainly wherever
  FACADE-01 is recorded — `deferred-items.md` is described across the corpus as the highest-fidelity
  document in it, and this is the first measured case of it being misleading. That is a fact about
  the corpus, not just about D5.

</specifics>

<deferred>
## Deferred Ideas

- **Un-ignoring the four `rust,ignore` doctests in `herald_registry.rs`** — real quality gap, real
  drift risk, but doctest posture is Phase 15's per Phase 10's routing. Handed over with the
  `file:line` list (D-03).
- **Executing any D1–D4 relocation** — each carries a verdict and a trigger out of this phase
  (D-05, D-07, D-08). Execution is architecture work with its own phase.
- **Rewriting the 49 `crate::core::` importers** — D1's "do" branch. Deferred with the no-alias
  sweep as its trigger.
- **Reintroducing the `paladin user …` CLI surface** — recorded as a deliberate deferral with an
  intact recovery path (D-10). Promotion to scope needs its own phase.
- **Creating `paladin-ml`** — the placement *condition* is recorded (D-09); the crate is explicitly
  out of scope per PROJECT.md and D-00k.
- **Confirming ADR-0031 with a human** — flagged as D-00m. Not blocking this phase because no
  relocation executes, but any future phase that executes a D3/D4 edge should do it first.

</deferred>

---

*Phase: 11-facade-residue-deferred-register-disposition*
*Context gathered: 2026-08-08*
