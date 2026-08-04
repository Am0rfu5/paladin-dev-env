# Phase 1: Ground Truth & Decision Records - Context

**Gathered:** 2026-07-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Make `.planning/` a cited, truthful account of what v0.7.0 actually ships for Milestone 1, and
give six contested definitions exactly one recorded, evidence-backed answer each.

**Two deliverable classes:**

1. **A cited status ledger** (RECON-01, RECON-08) — every Milestone-1 requirement carries a
   `file:line`-cited verdict, replacing the 2026-01 task-list snapshot as the source of truth.
2. **Six ADRs** (RECON-02 … RECON-07) — one per competing variant pair, each naming the chosen
   variant and the shipped code it was checked against.

**This phase writes records and decisions. It does not change product code.** Where an ADR
concludes that shipped code must change, it flags that and Phase 2's GAP-07 executes it. The one
exception is `.planning/` and roadmap/requirements source corrections, which are this phase's
whole point.

**Not in this phase:** re-implementing shipped Milestone-1 work (98% of task items are done);
raising actual coverage (Phase 3's QUAL-01); applying the ADRs in code (Phase 2's GAP-07);
Milestone 2-3 ground truth (Phase 5).

</domain>

<decisions>
## Implementation Decisions

### ADR home & authority

- **D-01:** ADRs live in `.planning/decisions/`, one file per decision. Phases 5, 7, 10 and 13
  append to the same directory. Chosen over `docs/src/appendix/adr/` (Milestone 11 Epic 3 declared
  the appendix a rewrite non-goal, and DOCS-02 in Phase 16 is already fighting a document that went
  invisible after being relocated there) and over `docs/adr/` (sits apart from `.planning/`, so
  downstream GSD agents need an explicit pointer to find it).

- **D-02:** The precedence order gains a new top tier:
  **ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
  task-list checkbox.** An ADR that contradicts shipped code is an instruction to change the code.
  This is what routes RECON-04 and the `citadel.rs` duplicate into Phase 2's GAP-07.
  PROJECT.md's stated precedence order must be updated to match.
  — **Reversibility:** costly — every ADR written from Phase 1 onward is authored against this
  ordering, and PROJECT.md, REQUIREMENTS.md and four sibling ground-truth phases cite it. Undoing
  it means re-deciding whether each recorded answer still binds.

- **D-03:** Because ADRs now top the order, **every ADR MUST carry a `Code conformance` field**
  with value `conforms` or `must change`. Where the value is `must change`, the ADR names the
  requirement that will execute it (normally GAP-07). This field is what keeps "authoritative"
  from being confused with "already true".

- **D-04:** ADR file shape is lean and evidence-first:
  `Status · Date · Question · Chosen variant · Evidence (file:line) · Rejected variants (the REQ-*
  IDs it resolves) · Code conformance · Downstream consumers`.
  Chosen over MADR (its "Considered Options" section would restate REQUIREMENTS.md's variant groups
  verbatim) and over the corpus's own two-file `decisions/` + `-options.md` house style (twelve
  files for six decisions).

- **D-05:** Phase 1 authors its six ADRs and **writes down the promotion procedure** for the eleven
  existing ADR candidates. Promotion is now viable without re-running the closed ingest, because
  ADRs live in `.planning/decisions/` and top the precedence order. **Phase 1 promotes none of the
  eleven** — each stays with its owning phase (7, 9, 10, 12, 13, 14), which gains a path where it
  previously had a blocker.

- **D-06:** PROJECT.md's `## Key Decisions` table gets one row per Phase 1 ADR, linking to the file.
  PROJECT.md itself predicts this ("The first real entries in this table are expected from Phase 1
  — six ADRs, one per competing variant pair"), and the table's "Empty by evidence, not by
  omission" note is replaced with a pointer to `.planning/decisions/`.

### Coverage gate (RECON-07)

- **D-07:** **Re-measure before recording.** Run `cargo llvm-cov` against the current tree; the ADR
  records the gate alongside the measured figure, the exact command, and the date. Every documented
  baseline is stale or contested — 60.88% unit / 67.79% integration (Milestone 1, predates
  Milestones 2-12), ~78% overall (Milestone 3 release notes), 76-77% (Deferred-QA, February 2026,
  with two known-stale module paths).

- **D-08:** **Scope is one workspace-wide line-coverage number**, all tests combined, from one
  reproducible command (`cargo llvm-cov --workspace`). This is what RECON-07 literally asks for
  ("a single number and a single scope") and what Phase 15's CI gate and `make coverage` need.
  **Consequence:** Phase 3's success criterion 1, which names unit and integration coverage
  separately, must be amended to match. Flag this to the roadmap.

- **D-09:** **The gate is the re-measured baseline rounded down to a whole percent, hard-fail from
  day one**, with **80% recorded in the same ADR as the target** plus a named ratchet trigger.
  This answers the parent PRD's Open Question 3 ("hard fail or soft warning initially?") as
  hard-fail without the ramp-up problem it worried about — the gate cannot be red on day one by
  construction. Phase 3 raises real coverage; Phase 15 wires the floor into CI.
  Chosen over an immediate 80% hard gate (would make Phase 15's CI gate red until Phase 3 lands)
  and over Epic 25's phased ramp (three numbers where RECON-07 asked for one).

- **D-10:** The two module-scoped gates — **Herald ≥ 95%, autonomous ≥ 90%** — are named in the
  RECON-07 ADR, recorded as sitting above the global floor, and **explicitly not withdrawn**. Their
  placement is handed to **VERIFY-05 in Phase 5**, which the roadmap already assigns as their owner.

### Record vs decide (posture and the five type/gate answers)

- **D-11:** **Default posture: shipped code wins unless the ADR argues otherwise in writing.**
  Deviation requires a stated reason and a `Code conformance: must change` flag. This matches the
  precedence order and `intel/code-verification.md`'s standing instruction not to plan a
  reconciliation for `BattalionResult`.

- **D-12: RECON-02 — `BattalionConfig`.** `crates/paladin-core/src/platform/container/battalion/mod.rs:37`
  is the one authoritative `BattalionConfig` (verified: it is the Epic 4 field set exactly;
  `CommanderConfig` does not exist anywhere in `crates/` or `src/`).
  The duplicate at `crates/paladin-core/src/platform/container/citadel.rs:280` is a self-described
  placeholder — its doc comment reads *"This is a placeholder and will be expanded in Epic 4"* —
  that Epic 4 superseded and nobody removed. **It is renamed** (e.g. `BattalionCheckpointConfig`),
  **keeping its three fields (`max_concurrency`, `timeout_seconds`, `continue_on_error`) and its
  serde shape.** It is a different concept — checkpoint/resume knobs, not orchestration config — so
  a distinct name is the accurate answer rather than a workaround.
  **No persisted-schema change, no migration** — chosen deliberately over replacing it with the
  real `BattalionConfig`, which would change `BattalionState`'s serialized form (`schema_version:
  "1.0.0"`, consumed by `crates/paladin-memory/src/citadel/file_citadel.rs`) and require a version
  bump plus a read path for existing checkpoints.
  — **Code conformance:** must change (rename lands in GAP-07)

- **D-13: RECON-03 — `BattalionResult`.** Pure recording. The shipped struct at
  `battalion/mod.rs:549` is a verified merged superset of all three positions. The ADR records what
  the superset chose: `per_paladin_times` in place of Epic 5's `execution_time_ms`, and
  `node_errors: Vec<NodeError>` in place of `errors: Vec<PaladinError>` (because `BattalionError`
  does not derive `Serialize`/`Deserialize` while `BattalionResult` does). Epic 8's Herald
  expectation is satisfied — Battalion type as `strategy_used`, aggregated tokens as `total_tokens`
  plus `per_paladin_tokens`.
  — **Code conformance:** conforms

- **D-14: RECON-04 — Formation minimum Paladin count.** **Formation relaxes to ≥ 1.**
  *Verified during discussion, and sharper than the documents describe: shipped code contains both
  halves of the contradiction.* `crates/paladin-battalion/src/commander.rs:1912`
  (`test_auto_selects_formation_for_single_paladin`) is a **passing** test asserting Auto routes a
  single Paladin to Formation, and `crates/paladin-core/src/platform/container/battalion/formation.rs:111`
  then rejects fewer than 2 at execution. **"Code wins" cannot resolve this — the tree argues with
  itself**, making this a second instance of the Group 29 class (a variant shipped code does not
  settle).
  Relaxing Formation leaves the passing Commander test and its Auto rule untouched, and matches
  Phase 2's success criterion 5 as written. Majority aggregation keeps its independent ≥ 3 check.
  Rejected: rewriting `analyze_and_select` (breaks a passing test and contradicts Phase 2's
  criterion); failing at `CommanderBuilder::build()` (removes single-Paladin Commander as a
  capability rather than reconciling).
  — **Code conformance:** must change (`formation.rs:109` lands in GAP-07)

- **D-15: RECON-05 — temperature validation.** **Provider-aware.** Add a temperature range to
  `ProviderCapabilities` and validate against the selected provider's range.
  *Verified during discussion:* `ProviderCapabilities` at
  `crates/paladin-ports/src/output/llm_port.rs:754` **has no temperature-range field at all**, so
  the provider-aware position was never implementable as specified. Meanwhile
  `src/application/services/paladin/paladin_builder.rs:1112` clamps to `[0.0, 1.0]` while
  `crates/paladin-llm/src/config/llm.rs:14` documents "Default temperature (0.0–2.0)" — the
  contradiction is live in the tree, not only on paper.
  This makes Epic 6 REQ-5's DeepSeek 0.0-2.0 range reachable through the normal Paladin path
  instead of unreachable by construction. The autonomous band logic
  (`src/application/services/paladin/temperature_service.rs`, bounds validated in
  `autonomous_config.rs:107-110`) stays a layer above as task-type guidance.
  Rejected: a global `[0.0, 1.0]` clamp (would require recording Epic 6 REQ-5 as withdrawn — an
  explicit capability reduction); adapter-level clamping (silent clamping past the point the caller
  can be told they were wrong).
  — **Code conformance:** must change (ports-layer change lands in GAP-07)
  — **Sequencing note:** Phase 14's WEB-03 corrects `supports_tool_calling` on the *same struct*.
  Do not schedule these independently.
  — **Reversibility:** costly — `ProviderCapabilities` is a published ports-layer type on the
  framework's primary integration contract; adding a field is additive, but every adapter must then
  populate it and downstream consumers branch on it.

- **D-16: RECON-06 — `Herald` trait.** Pure recording. *Verified during discussion:* the shipped
  trait at `crates/paladin-core/src/platform/container/herald.rs:49` ships the **v2** form exactly —
  `format_paladin_result`, `format_battalion_result` and `finalize_stream` returning
  `Result<String, HeraldError>`; `format_stream_chunk` returning `Result<Option<String>, HeraldError>`;
  plus `name()` and `mime_type()`. `format_error` is deliberately infallible (`-> String`), which is
  what makes FR-10's graceful-degradation requirement expressible. The ADR should record that
  asymmetry explicitly, not smooth it over.
  — **Code conformance:** conforms

### Ledger shape & evidence (RECON-01, RECON-08)

- **D-17:** The ledger is a **new file per milestone** — `.planning/ledgers/milestone-01.md` — with
  REQUIREMENTS.md's existing "Milestone 1 as-shipped ledger" section reduced to a pointer. Phases 5,
  7, 10 and 13 each add a sibling. REQUIREMENTS.md is already ~4,000 lines and holds four
  as-shipped ledger sections; five sets of `file:line`-cited verdicts inline would make it
  unreadable.

- **D-18:** **Primary key is the `REQ-*` requirement ID, with outstanding task items nested under
  the requirement they belong to.** Satisfies RECON-01's "every outstanding Milestone-1 task item"
  without inventing identifiers for task-list checkboxes that have none (they are numbered
  positions inside 64 files), and keeps the ledger joinable to REQUIREMENTS.md and the roadmap.

- **D-19:** **Evidence bar: `satisfied` requires a `file:line` citation PLUS a named passing test,
  example, or command that exercises it.** A `file:line` with nothing exercising it gets a distinct
  verdict — **`present, unproven`**. This exists because "the code exists" has already burned this
  corpus: Milestone 4 Epic 3's task list is fully checked while three CLI-only dependencies remain
  unconditional.
  **Expect this to produce a third bucket nobody has counted yet.** That is the point, and the
  planner should budget for it rather than treat it as a surprise.

- **D-20:** **Verdict classes for the ledger:** `satisfied` · `present, unproven` ·
  `genuinely outstanding` · `deferred with reason` · `superseded by shipped code`.

- **D-21:** The three known divergences are recorded as **`superseded by shipped code` rows in the
  ledger**, not as separate ADRs: MCP Streamable-HTTP where the Milestone-1 PRD specified SSE;
  Qdrant/Sanctum where it specified `sqlite-vss`; and the shipped interactive REPL against Epic 9
  non-goal NG-7. The REPL row should be flagged loudly — it is a documented non-goal that shipped
  anyway, and it is the corpus's own evidence for why nothing here is treated as locked.
  Rejected: giving each its own ADR (grows Phase 1 from six to nine, and none is a competing
  variant pair); a standalone divergence register (a third document class, where Phases 7, 10 and
  13 already fold their equivalents into their ledgers).

### Claude's Discretion

- ADR file naming and numbering within `.planning/decisions/` (the user did not specify; pick a
  scheme that stays stable as Phases 5/7/10/13 append — a milestone-or-phase-scoped prefix is
  likely right, since a flat global counter across five phases invites collisions).
- Whether an ADR can later be superseded, and by what mechanism. Not discussed. Given D-02 puts
  ADRs at the top of precedence, the planner should record *something* here rather than leave it
  undefined.
- Coverage measurement exclusions — whether `examples/`, `benches/` and the `doc-examples` crate
  count toward the workspace number, and whether doctests are included. Not discussed. Must be
  pinned down in the RECON-07 ADR itself, because the number is meaningless without it and Phase 15
  has to reproduce it in CI.
- The concrete ratchet trigger for raising the coverage floor toward 80% (D-09 requires it be
  named; the user did not name it).
- The renamed identifier for the `citadel.rs` struct — `BattalionCheckpointConfig` was the
  discussion's example, not a locked choice.
- How ledger verification work is split across plans (per-epic fan-out vs sequential). Not
  discussed; Milestone 1 has ten epics.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project-level record (authoritative for this phase)

- `.planning/PROJECT.md` §Context — the precedence order, the eleven ADR candidates, the five
  documented positions contradicted by shipped code, and why the corpus has zero locked decisions.
- `.planning/PROJECT.md` §Constraints — feature-gating contract, hexagonal rules, error-handling
  rules, the edition split, the licence split.
- `.planning/PROJECT.md` §Key Decisions — currently empty *by evidence*; D-06 populates it.
- `.planning/ROADMAP.md` §"Phase 1: Ground Truth & Decision Records" — the five success criteria.
- `.planning/ROADMAP.md` §Progress — the coupling notes, especially `RECON-07 → VERIFY-05 → PIPE-02`.
- `.planning/REQUIREMENTS.md` lines 133-187 — RECON-01 … RECON-08 in full, with their *Derives* and
  *Resolves* provenance.
- `.planning/REQUIREMENTS.md` §"Milestone 1 as-shipped ledger" (from line 2361) — the existing
  per-epic ledger that D-17 supersedes with a pointer.
- `.planning/INGEST-CONFLICTS.md` — warnings 7-8, which RECON-01 and RECON-08 derive from.

### The six variant groups (one ADR each)

- `.planning/REQUIREMENTS.md` §"Group 1 — project-wide test coverage gate" (line 1657) — RECON-07.
- `.planning/REQUIREMENTS.md` §"Group 2 — valid temperature range" (line 1673) — RECON-05.
- `.planning/REQUIREMENTS.md` §"Group 3 — `BattalionConfig` field set" (line 1685) — RECON-02,
  including the run-3 code verification note.
- `.planning/REQUIREMENTS.md` §"Group 4 — `BattalionResult` / `BattalionMetadata`" (line 1714) —
  RECON-03, including the explicit *do not plan a reconciliation task* instruction.
- `.planning/REQUIREMENTS.md` §"Group 5 — minimum Paladin count for Formation" (line 1752) — RECON-04.
- `.planning/REQUIREMENTS.md` §"Group 6 — `Herald` trait signature" (line 1763) — RECON-06.
- `.planning/REQUIREMENTS.md` §"Group 30 — initial coverage threshold for the CI gate" (line 2174) —
  the sixth coverage position and the parent PRD's Open Question 3.

### Code-state intelligence

- `.planning/intel/code-verification.md` — direct code verification across all five ingest runs;
  third in the precedence order and the source of several "do not plan this" instructions.
- `.planning/codebase/ARCHITECTURE.md` — cited by variant groups 2 and 4.
- `.planning/codebase/STRUCTURE.md` — cited by variant group 3.
- `.planning/codebase/CONCERNS.md` — existing error-handling violations and the edition mix.
- `.planning/codebase/TESTING.md` — the three-tier test strategy, needed for D-19's evidence bar
  and for D-07's coverage measurement scope.
- `.planning/intel/task-completion-state.md` — the deterministic measurement of all 64 task lists;
  the source of the "outstanding task item" set D-18 nests.
- `.planning/intel/context.md` — the implementation-status topics RECON-01 derives from.

### Shipped code the ADRs cite

- `crates/paladin-core/src/platform/container/battalion/mod.rs:37` — the authoritative `BattalionConfig`.
- `crates/paladin-core/src/platform/container/battalion/mod.rs:549` — the merged-superset `BattalionResult`.
- `crates/paladin-core/src/platform/container/citadel.rs:280` — the Epic 4 placeholder duplicate.
- `crates/paladin-core/src/platform/container/citadel.rs:233` — `BattalionState.config`, the
  persistence site that constrains D-12.
- `crates/paladin-memory/src/citadel/file_citadel.rs:507,541` — the other consumers of that shape.
- `crates/paladin-core/src/platform/container/battalion/formation.rs:109-111` — the ≥ 2 rejection.
- `crates/paladin-battalion/src/commander.rs:1912` — the passing test asserting the opposite.
- `crates/paladin-core/src/platform/container/herald.rs:49-153` — the shipped `Herald` trait.
- `crates/paladin-ports/src/output/llm_port.rs:754-769` — `ProviderCapabilities`, with no
  temperature range.
- `src/application/services/paladin/paladin_builder.rs:1112` — the `[0.0, 1.0]` clamp.
- `crates/paladin-llm/src/config/llm.rs:14` — the documented 0.0–2.0 default range.
- `src/application/services/paladin/temperature_service.rs` — the shipped task-type band logic.
- `crates/paladin-core/src/platform/container/autonomous_config.rs:107-110` — the band bounds check.

### ADR candidate referenced but not promoted here

- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
  — the corpus's only decision/options pair, `Status: Approved`, adjacent to RECON-03. **Not
  promoted in Phase 1** (D-05); its owner is Phase 7's ARCH-03(c). Read it before writing the
  RECON-03 ADR so the two do not contradict each other.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`.planning/codebase/` maps (refreshed 2026-07-30)** — seven documents already describe the tree
  and sit second in the precedence order. The ledger cites code directly, but the maps are the
  fastest route to *finding* the code to cite.
- **`.planning/intel/task-completion-state.md`** — all 64 task lists already measured
  deterministically. D-18's "outstanding task item" set comes from here; do not re-derive it.
- **The existing per-epic ledger in REQUIREMENTS.md (line 2361 onward)** — already carries
  shipped/partial verdicts per `REQ-*` ID for all ten Milestone-1 epics. The new ledger extends
  this with citations and the evidence bar; it is a starting point, not a blank page.
- **`.claude/gsd-core/bin/lib/adr-parser.cjs`** — GSD ships an ADR parser. Worth checking what
  frontmatter shape it expects before fixing D-04's format, so the ADRs are machine-readable by
  GSD tooling rather than only by humans.

### Established Patterns

- **Precedence is the project's core mechanic**, and D-02 modifies it. Every artefact this phase
  writes must be legible against the ordering, and PROJECT.md's statement of it needs updating.
- **`.planning/` currently has no `decisions/` or `ledgers/` directory.** Both are created by this
  phase and inherited by Phases 5, 7, 10 and 13. Conventions set here are set for five phases.
- **Medieval military ubiquitous language is mandatory** in code, docs and comments — including in
  the ADRs and the ledger.
- **Doc comments carry archaeology.** The `citadel.rs:280` placeholder was identified by its own
  doc comment. When verifying, read the comments, not only the signatures.

### Integration Points

- **PROJECT.md `## Key Decisions`** — populated by D-06; currently empty by evidence.
- **PROJECT.md `## Context`** — the precedence order stated there must be updated for D-02.
- **REQUIREMENTS.md §"Milestone 1 as-shipped ledger"** — reduced to a pointer by D-17.
- **ROADMAP.md Phase 3 success criterion 1** — must be amended for D-08 (it names unit and
  integration coverage separately; the gate is now one workspace-wide number).
- **Phase 2's GAP-07** — receives every ADR whose `Code conformance` is `must change`: currently
  D-12 (rename), D-14 (Formation ≥ 1), D-15 (`ProviderCapabilities` temperature range).
- **Phase 5's VERIFY-05** — receives the module-scoped gates per D-10, and extends RECON-07's
  number across the four earlier positions.
- **Phase 14's WEB-03** — touches `ProviderCapabilities`, the same struct as D-15. Sequence together.
- **Phase 15's PIPE-02** — must land the CI threshold on RECON-07's number or record why it differs.

</code_context>

<specifics>
## Specific Ideas

- **Three findings surfaced during this discussion that the ingest record does not contain.** The
  researcher should treat them as verified starting points, not hypotheses:
  1. **RECON-04 is a Group-29-class variant**, not a doc-vs-code disagreement. Both halves of the
     contradiction ship, and one of them has a passing test. The corpus believed Group 29 (the
     token mechanism) was its only unsettleable variant; this is a second.
  2. **The `citadel.rs` `BattalionConfig` is self-documenting as a placeholder** — its doc comment
     names Epic 4 as the thing that would replace it, and Epic 4 did, elsewhere. This turns RECON-02's
     open half from an ambiguity into an obvious cleanup with one real constraint (the persisted
     schema).
  3. **`ProviderCapabilities` has no temperature-range field**, so RECON-05's provider-aware
     position was never implementable as written by anyone who tried.

- **The 80% figure appears in nine Milestone-1 PRDs and Epic 24.** D-09 keeps it as the recorded
  target rather than the gate, so those acceptance criteria are neither falsified nor pretended to
  be met.

- **Expect the ledger to produce a bucket nobody has counted** (`present, unproven`, per D-19).
  Budget for it in the plan rather than treating it as a deviation.

</specifics>

<deferred>
## Deferred Ideas

- **Promoting the eleven existing ADR candidates.** Phase 1 builds the mechanism (D-05) but promotes
  none. Owners: Phase 7 (battalion-result upward dependency, CLI placement), Phase 9 (RustSec risk
  acceptance — carries the corpus's only expiry date, 2026-09-30), Phase 10 (cost-benefit
  assessment, licence checklist, facade reconciliation), Phase 12 (audit-suppression single-source
  invariant), Phase 13 (`AgentProvisioner` placement), Phase 14 (opaque-bearer-token decision),
  Phase 15 (coverage deferral record).

- **The Herald `format_error` asymmetry as a design question.** D-16 records it as shipped. Whether
  the whole trait should be uniformly fallible is a design change, not a variant resolution — out
  of scope here.

- **Amending ROADMAP.md Phase 3's success criterion for D-08.** The amendment itself is in scope
  (correcting the record at source is this phase's job); *re-scoping Phase 3's testing work* to the
  new single number is Phase 3's.

- **Coverage measurement tooling** — a `make coverage` target and `.codecov.yml` (neither exists).
  Phase 15's PIPE-02 owns both. Phase 1 only needs the number and a reproducible command.

- **Whether ADRs should be published to the mdbook** for framework consumers. Rejected as this
  phase's home (D-01), but "should the decisions be user-visible eventually?" is a real question
  and belongs with Phase 16's documentation work.

</deferred>

---

*Phase: 1-ground-truth-decision-records*
*Context gathered: 2026-07-30*
