# Phase 7: Workspace Ground Truth & Recorded Answers - Context

**Gathered:** 2026-08-06
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision
below carries the reasoning that produced it; none was confirmed by a human. Review before planning
if any answer looks wrong.

<domain>
## Phase Boundary

Make `.planning/` a cited, truthful account of what Milestones 4, 5 and 6 (the three refactor
milestones) actually shipped, correct the five documented positions that shipped code contradicts,
and give the four competing variant pairs plus the numbering, re-export, binary-target and
build-benchmark questions exactly one recorded answer each.

**Four deliverable classes:**

1. **A cited status ledger** (ARCH-01) — `.planning/ledgers/milestone-04-06.md`, with a
   `file:line`-cited verdict for all **115** run-3 requirement IDs across 13 epics, recording the
   real workspace shape: **ten library crates** plus `doc-examples` plus the root facade package
   `paladin-ai` — replacing both the "six crates" the M5/M6 overviews assume and the "9-crate
   workspace" this planning set carried before run 3.
2. **Seven new ADRs** (ARCH-02, ARCH-03(b)(c)(d), ARCH-04, ARCH-06, ARCH-07) — **0014-0020**.
   ARCH-03(a) takes **no** new ADR: Phase 4 already answered and applied it as ADR-0009.
3. **In-repo source corrections under `.project/`** (ARCH-02, ARCH-03(c)(d), ARCH-04, ARCH-05) —
   dated correction banners plus inline annotation, superseded text retained, following the
   Phase 5 D-08 pattern.
4. **One codebase-map correction** (ARCH-01) — `.planning/codebase/STRUCTURE.md` documents only
   **6 of the 10** library crates. It sits third in the precedence order and cannot be left wrong
   while the ledger it feeds records ten.

**This phase writes records and decisions. It does not change product code.** Where a recorded
answer has a code consequence, it is flagged with an owning requirement in a later phase. The one
exception is `.project/`, `.planning/` and REQUIREMENTS.md/ROADMAP.md source corrections —
correcting the record at its source is this phase's whole point.

**Not in this phase:** consolidating the three `TokenUsage` structs (Phase 8, DEBT-05 — Phase 7
only decides *which one is canonical*); fixing the `api-surface` CI job, the missing
`#[deprecated]` annotations, the disabled `paladin-ports` doctests, or the leaked CLI dependencies
(Phase 8, DEBT-01…DEBT-04); building any dependency-allowlist enforcement in CI (Phase 15);
writing the user-facing binary-architecture page into the mdbook (Phase 16); Milestone 7-8 ground
truth (Phase 10).

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 2 and 5 — locked, not re-litigated

Recorded here so downstream agents do not re-derive them from archived CONTEXT.md files.

- **D-00a:** ADRs live in `.planning/decisions/`, one file per decision, flat sequential numbering.
  **0001-0013 are taken.** `PROMOTION.md` records **0014** as the next free number; Phase 7
  allocates **0014-0020** (D-22) and updates that line. *(Phase 1 D-01; PROMOTION.md)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02)*
- **D-00c:** Every ADR carries a `Code Conformance` field valued `conforms` or `must change`, and
  where it is `must change`, names the requirement that executes it. *(Phase 1 D-03)*
- **D-00d:** Ledger is a new file per milestone block — `.planning/ledgers/milestone-04-06.md` —
  with REQUIREMENTS.md's `## Milestone 4-6 as-shipped ledger` section reduced to a pointer (D-24).
  *(Phase 1 D-17)*
- **D-00e:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers. *(Phase 1 D-18)*
- **D-00f:** Ledgers are **amended in place** — a later plan's measured result edits the row
  directly with the new verdict, the command or `file:line` that produced it, and the date. Never a
  separate corrections file. Superseded text is retained, not deleted. *(Phase 2 D-02, which names
  Phases 5/7/10/13 as inheritors.)*
- **D-00g:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner at the top naming what was wrong and pointing at the ADR, each defective claim
  corrected inline with the original text kept and marked superseded. `.project/` is the historical
  ingest corpus; silently rewriting it would destroy the provenance five ingest runs were built on.
  *(Phase 5 D-08)*
- **D-00h:** ADR file shape is `Status / Context / Decision / Considered Options / Code Locations /
  Code Conformance / Downstream Consumers`, no frontmatter, matching 0001-0013.

### ARCH-01 — ledger depth, evidence bar and vocabulary

- **D-01:** **Phase 5's evidence bar carries over unchanged, with one carve-out this run needs.**
  No row gets `satisfied` without a `file:line` citation **plus** something that exercises it. The
  carve-out: Milestones 4-6 are structural milestones, so a large share of their requirements *are*
  manifest declarations (`edition`, feature-flag shapes, dependency lists, `required-features`,
  workspace membership). For those, **the manifest line plus a named CI job or build leg that
  consumes it is the exercising artefact** — e.g. `crate-isolation` (`ci.yml:228`) and the
  `feature-flags.yml` workspace matrix (`:115,118,141`). Behavioural requirements still need a
  test, example or command. Chosen over demanding a runtime test for a manifest fact (would push
  most of the ledger into `present, unproven` for no information gain) and over accepting file
  existence alone (the exact false-positive class the bar exists to reject).

- **D-02:** **Seven verdict classes: Phase 5's five plus `relocated` and `diverged`.**
  `satisfied` · `present, unproven` · `genuinely outstanding` · `deferred with reason` ·
  `superseded by shipped code` · **`relocated`** · **`diverged`**.
  `relocated` is deliberately *not* folded into `superseded by shipped code`: ARCH-05's entire
  point is "relocated, not missing", and collapsing the class destroys the signal that stops a
  later phase planning the mdbook pages as gaps. `diverged` marks a requirement the shipped tree
  deliberately implements differently (the five ARCH-05 positions), as distinct from one a later
  milestone replaced. ROADMAP criterion 1's five names map onto this vocabulary; the mapping is
  stated in the ledger head note.

- **D-03:** **Triage directs effort, not the bar.** 22 run-3 claims were already verified directly
  against `Cargo.toml` contents and type definitions during ingest (`intel/code-verification.md`)
  and re-confirmed 2026-07-30 — those rows need a citation refresh and an exercising artefact, not
  a re-verification. Rows carrying `Verify —` in the existing REQUIREMENTS.md ledger get first-pass
  depth. The standard applied is identical; only ordering and time budget differ.

- **D-04:** **Both systematic path caveats are recorded once at the head of the ledger, not per
  row.** (a) The `src/…` paths in run-3 PRDs are *internally* historical — Milestone 6 moved what
  Milestone 5 had just placed, and `src/application/use_cases/` no longer exists at all; citations
  are current locations. (b) `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and
  `docs/CONFIGURATION.md` ship as mdbook chapters, not at their PRD paths. A row whose only
  divergence is caveat (b) is `relocated`, not a gap.

- **D-05:** **The workspace-shape correction lands in three places, and `STRUCTURE.md` is one of
  them.** The ledger head note is authoritative; REQUIREMENTS.md's section becomes a pointer
  (D-24); and **`.planning/codebase/STRUCTURE.md` is corrected** — verified this session, it
  documents only `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`,
  `paladin-memory` and `paladin-storage`, omitting `paladin-herald`, `paladin-notifications`,
  `paladin-content`, `paladin-web` and `doc-examples`. It is **third in the precedence order**
  (D-00b), above `intel/` and every PRD, so leaving it at six crates would have the map outrank the
  ledger that corrects it. PROJECT.md already records ten and needs no change.

- **D-06:** **Expect a large `present, unproven` bucket, and expect it to split by milestone.**
  `task-completion-state.md` records M4 at 93.2% (20 open, all in Epic 2), M5 at 96.4% (17 open),
  M6 at 100%. Run-3 verification found M4's 20 open items **corroborated** — the first block in this
  corpus where checkboxes understate nothing — M5's 17 **mostly contradicted**, and M6's zero
  **corroborated**. A verifier who applies runs 1-2's "the record understates the tree" heuristic
  uniformly will produce a wrong M4 Epic 2 section.

### ARCH-02 — the milestone/tier numbering collision

- **D-07:** **ADR-0014 records the convention; the source documents are corrected in-repo.** Two
  deliverables, as with the run-2 analogue (ADR-0010). The convention is the one ARCH-02 already
  states and VERIFY-03 already used: **the directory / task-list numbering is authoritative**
  (4 = Tier 1, 5 = Tier 2, 6 = Tier 3), and every "Milestone 1/2/3" reference *inside* these three
  milestones is a tier label. ADR-0014 must cite ADR-0010 explicitly and state that this is the
  same convention closing the second of the corpus's two numbering defects — that cross-reference
  is what makes `REQ-*` provenance keys resolve uniformly.

- **D-08:** **Full inline correction on five documents; a one-line pointer banner on the seven
  verbatim extracts.** The materially distinct sources get the D-00g treatment (dated banner +
  inline annotation, superseded text retained):
  1. `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md` — titled "Milestone 1".
  2. `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md` — titled "Milestone 2".
  3. `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md` — "Completed in Milestones 1 and 2".
  4. `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` Non-Goal 2 — "hardened in Milestone 1 / Epic 2".
  5. `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` §1 — same cross-reference.
  The seven byte-equivalent extracts INGEST-CONFLICTS lists (M5 Epics 2/3/4/5, M6 Epics 1/2/3/4
  copies) get a **one-line dated pointer to ADR-0014 at the top and nothing else** — they carry no
  independent content, and rewriting a copy seven times multiplies edit risk without adding
  information.
  — **Reversibility:** costly — the corrected numbering becomes the provenance key every later
  reader and every Phase 8-16 citation uses. Reverting means re-checking which of two numbering
  schemes each downstream citation meant.

### ARCH-03 — the four competing variant pairs

- **D-09 (a) Rust edition — citation only, no new ADR.** Phase 4 recorded **and applied** it:
  `.planning/decisions/0009-workspace-rust-edition-2024.md`. Verified this session — **all twelve
  workspace manifests declare `edition = "2024"`** (root plus the eleven crates under `crates/`).
  The ledger rows for `REQ-workspace-crate-edition-v1/-v2` cite ADR-0009 and record v1 (2021) as
  superseded. REL-02's code fix is done. Nothing to decide.

- **D-10 (b) `paladin-core` / `paladin-ports` dependency allowlists — ADR-0015 rewrites the
  allowlist against reality and separates the *invariant* from the *list*.** The PRD's Appendix B
  calls itself "complete and exhaustive" at six crates; the tree carries **14** in `paladin-core`
  and — measured this session — **11** in `paladin-ports`, not the 10 `intel/code-verification.md`
  records (`mime_guess` is an eleventh, added since run 3). A list wrong by eight and by four is
  unenforceable as written.
  ADR-0015 records three things: (i) **the enforceable invariant** — `paladin-core` and
  `paladin-ports` may carry no provider SDK, transport client, storage driver or web framework;
  that is what SM-4 / FR-24 / FR-25 were actually protecting, and it **holds today**; (ii) **the
  measured current lists** as the new baseline, with the eight `paladin-core` extras (`tokio`,
  `sha2`, `blake3`, `petgraph`, `murmur3`, `url`, `regex`, `futures`) and the four `paladin-ports`
  extras (`serde_json`, `futures`, `md5`, `mime_guess`) **accepted, not recorded as debt** — every
  one is a general-purpose or domain-support crate, not infrastructure; (iii) **`tokio` in
  `paladin-core` as the single entry that gets an explicit written justification**, because it is
  an async runtime in a crate documented as "zero external dependencies", and it is the one a
  future purity review would reasonably challenge.
  Chosen over "state the intended target and treat the extra twelve as tracked debt" — that
  manufactures twelve debt items nobody intends to pay and leaves the invariant still unstated.
  **Enforcement is not built here.** A `cargo tree`-based check is recorded as a Phase 15 candidate.

- **D-11 (c) Port value-type ownership — ADR-0016 ratifies the shipped answer (`paladin-core`
  owns), and fixes the record on both sides.** This is the one place mechanical precedence gives
  the architecturally wrong result: the Epic 1 decision record is `Status: Approved` but
  manifest-typed DOC, so the later Epic 2 PRD outranks it and would pull `PaladinResult`,
  `StopReason` and `TokenUsage` back out of `paladin-core`, reintroducing the exact upward
  dependency the decision was written to remove. Shipped code implements the decision record.
  Two corrections, both required:
  - **The `.project/` side:** `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md`
    FR-7 and FR-10 are annotated per D-00g to extend FR-11's core-re-export carve-out from
    `RegistryError` alone to `PaladinResult`, `StopReason` and `TokenUsage`.
  - **The precedence side:** the Epic 1 decision record is **not** re-tagged via `--manifest`.
    ADR-0016 *is* the promotion — it restates the decision inside `.planning/decisions/`, where it
    sits at the top of the precedence order by construction, and cites the DOC as its provenance.
    Chosen over editing ingest manifests: re-typing a `.project/` file changes how five completed
    ingest runs classified their corpus, for an outcome an ADR achieves natively.
  **This decision is Phase 8's DEBT-05 input, and it must be stated as such:** the canonical type
  is `crates/paladin-core/src/platform/container/token_usage.rs:13`; the two other shipped copies —
  `crates/paladin-core/src/platform/container/battalion/mod.rs:497` and
  `crates/paladin-llm/src/llm_analysis_service.rs:51` — become re-exports. ROADMAP records Phase 8
  as depending on Phase 7 for exactly this one answer.
  — **Reversibility:** one-way — ADR-0016 is what DEBT-05 consolidates against. Once the two copies
  are collapsed, reversing means re-splitting a public type across two crates.

- **D-12 (d) LLM config bridge location — ADR-0017 accepts v2, and states the circular-dependency
  concern was *real but mis-sited*.** Shipped code is `crates/paladin-llm/src/config/bridge.rs`.
  Epic 4 FR-31/FR-32 feared a cycle `paladin-llm → root crate`; that cycle does not exist, because
  Milestone 6 moved the config types **down** into `paladin-llm` rather than moving the bridge
  **up** into the root. Milestone 6 did not break the boundary Epic 4 established — it removed the
  need for it. ADR-0017 must say that in those terms rather than declaring Epic 4 simply wrong;
  the concern was sound and the resolution was structural.
  Epic 4's FR-31/FR-32 get a dated superseded banner per D-00g.

### ARCH-04 — the Milestone 6 facade re-export policy

- **D-13:** **The no-shim posture stands. ADR-0018 records it as policy.** Both PRDs (M6 Epic 2
  Non-Goal 7, M6 Epic 4 Goal 7 / FR-4.11) and the shipped tree agree — `src/application/use_cases/`
  does not exist, confirmed this session (`src/application/` holds `cli`, `errors`, `mod.rs`,
  `services`). The overview's Epic 2 AC 6, Epic 4 AC 5 and the risk register's "facade crate
  re-exports absorb the change" are the minority position and are annotated superseded. Epic 2's
  own Open Question 4 ("should be confirmed with the team") is recorded as **now confirmed** rather
  than left dangling.

- **D-14:** **Version consequence: breaking in substance, absorbed as a minor bump by the pre-1.0
  series.** ADR-0018 cites `.planning/decisions/0008-workspace-version-0-7-0.md` — Phase 4 already
  answered the major-bump question. The ADR states plainly that Milestone 6 removed publicly
  reachable import paths (a breaking change under ordinary semver reasoning) and that pre-1.0 Cargo
  semantics make `0.7.0` the correct expression of it, so REL-01's single-version story is
  unaffected. **This is ROADMAP criterion 4's one recorded answer.** Do not re-derive it.

- **D-15:** **The M5→M6 posture flip is recorded as history, not as a contradiction.**
  `REQ-battalion-facade-shim` (M5 Epic 3: keep a re-export shim) was correct for Milestone 5 and
  was retired by Milestone 6. The ledger records M5's row `superseded by shipped code` pointing at
  ADR-0018 rather than reopening it.

- **D-16:** **ADR-0018 is the input to Phase 11's FACADE-02 D1, and says so in
  `Downstream Consumers`.** ROADMAP records Phase 11 as loosely dependent on ARCH-04 for exactly
  this. Naming it in the ADR is what stops Phase 11 re-opening the policy.

### ARCH-05 — the five positions shipped code contradicts

- **D-17:** **All five are corrected at source with the D-00g pattern, and all five get `diverged`
  ledger rows. None gets an ADR.** Phase 1/5 precedent: contested positions get ADRs; divergences
  settled by shipped code with no competing defensible position get ledger rows plus a source
  correction. All five were verified against the tree this session:
  1. **`vision` gating the encryption crates** — `.project/Milestone_4-.../Epic_1/prd-expand-feature-flags.md`
     FR1 and its Design Considerations. Shipped: `Cargo.toml:274` `vision = []` gates nothing;
     `chacha20poly1305` and `zeroize` are unconditional. The Epic 1 `dependency-matrix.md` audit was
     right and the PRD was wrong. Applying the PRD literally breaks
     `cargo build --no-default-features` for user auth and Citadel. **Cross-reference Phase 5's
     ADR-0011**, which dispositioned this same encryption code.
  2. **MCP transport feature flags** (`mcp-transports` / `mcp-stdio` / `mcp-sse`) — M4 overview
     AC 1 + Appendix B. Shipped: no MCP feature flag of any kind. The PRD's dated 2026-04-15
     elimination note is what shipped.
  3. **`web-server` gating actix-web** — same PRD FR1. Shipped: `Cargo.toml:276`
     `web-server = ["dep:paladin-web", "dep:axum"]`; actix-web is not a root dependency.
  4. **A `paladin-cli` workspace crate** — M5 overview target structure + Appendix D. Shipped:
     `crates/` holds `doc-examples` plus the ten library crates and no `paladin-cli`. The CLI is
     `cli = [...]` (`Cargo.toml:284`) plus `[[bin]] paladin-cli` with
     `required-features = ["cli"]` (`:244-247`). M5 Epic 6's non-goal was correct; the overview's
     target structure was not.
  5. **`src/application/use_cases/` as the orchestration home** — M6 Epic 2 PRD. Shipped: the four
     orchestrator modules live under `src/application/services/` with the PRD's exact module names;
     `use_cases/` does not exist.

- **D-18:** **The four relocated documentation deliverables get `relocated` rows and one shared
  head note, and M6 Epic 4's FR-4.12 is re-pointed.** `STABLE_API.md`, `docs/FEATURE_FLAGS.md`,
  `docs/MIGRATION.md`, `docs/CONFIGURATION.md` → `docs/src/api-reference/{stable-api,feature-flags,migration-guide,crate-map}.md`
  and `docs/src/getting-started/installation.md`. FR-4.12's "update `STABLE_API.md`" now applies to
  `docs/src/api-reference/stable-api.md`. Six run-3 documents cross-reference the old root path;
  they are covered by the shared head note rather than six separate banners.

### ARCH-06 — the binary-target architecture question

- **D-19:** **ADR-0019 ratifies "Option A extended" — three binary targets — and states a purpose
  for each, which is what FR3 asked for and the never-produced architecture review owed.**
  Verified this session at `Cargo.toml:240-252`:
  - **`paladin`** (`src/main.rs`, no `required-features`) — **and its purpose is the finding of
    this gray area.** `src/main.rs` is the *content-aggregator* entry point: it declares
    `#[structopt(name = "smartcontent-aggregator")]`, loads `config.yml` via
    `Settings::load_from_file`, and calls `paladin::config::setup::setup_and_run`. It is the
    pre-Paladin service runner, not an agent-orchestration binary. ADR-0019 must say so plainly
    rather than inventing a tidy purpose for it, and must record the stale `structopt` name.
  - **`paladin-cli`** (`src/bin/paladin-cli.rs`, `required-features = ["cli"]`) — the Armory
    developer CLI.
  - **`paladin-server`** (`src/bin/paladin-server.rs`, `required-features = ["web-server"]`) — the
    Axum HTTP API server.

- **D-20:** **ADR-0019 must also record the coupling it exposes, because it re-scopes Phase 8.**
  Verified this session: **`structopt`'s only consumer in the entire tree is `src/main.rs`**
  (`grep -rn structopt src/ crates/` returns three hits, all in that file). The default `paladin`
  binary is not feature-gated, so `structopt` cannot be made `optional = true` without first
  deciding the fate of `src/main.rs` — gate it, migrate it to `clap`, or retire it. The recorded
  "three-line fix" for CLI dependency isolation is therefore **wrong for one of its three lines**,
  and ADR-0019's answer is its precondition. Named as a `Downstream Consumers` entry against
  Phase 8's CLI-isolation requirement.

- **D-21:** **ADR-0019 plus its ledger row satisfy FR9.3's documentation deliverable. The
  user-facing mdbook page is flagged for Phase 16, not written here.** `Code Conformance:
  must change`, executor named as Phase 16's documentation-currency work. This keeps Phase 7's
  record-only boundary intact — the same treatment Phase 5 gave every code consequence it found.

### ARCH-07 — making the build-time benchmark falsifiable

- **D-22:** **Restate SM-7 per scenario. Do not re-measure.** ADR-0020 transcribes the report's own
  five figures (clean build −6.6%, `paladin-core` incremental −18.9%, `paladin-llm` incremental
  −44.6%, `paladin-memory` incremental −50.2%, battalion-only −90.9%), marks two pass and three
  fail against ≥ 50%, and restates Milestone 5 SM-7 as a **per-scenario** target so each row can be
  judged. It also resolves the report's internal inconsistency by citing the table's −6.6% and
  recording the conclusion's "(−5%)" as a transcription error.
  Chosen over re-measuring: the ≥ 50% figure is a *comparison against the pre-workspace monolith*,
  and that tree no longer exists — re-measuring means resurrecting a historical commit and running
  full clean and incremental builds for a metric about a restructuring that completed three
  milestones ago. This environment carries the same offline/no-Docker constraints that already
  halted Phase 1's coverage measurement.

- **D-23:** **The "Overall verdict: Target achieved" conclusion is judged, not reconciled.** Same
  treatment ADR-0006 gave the ~78% coverage figure and the M1 baselines: ADR-0020 records that the
  conclusion is contradicted by the report's own table and does not attempt to explain it away.

- **D-24:** **The recommended re-measurement against a mid-tree baseline is declined with a
  recorded reason, not passed forward.** The report itself recommends it; ADR-0020 declines it on
  D-22's grounds and says so explicitly, so no later phase inherits an unfundable task. This is a
  deliberate difference from Phase 5's D-14a, which *did* pass a task forward — there the seam
  extraction was achievable, here the baseline is gone.

### ADR allocation and record bookkeeping

- **D-25:** **Seven new ADRs, 0014-0020, and `PROMOTION.md`'s next-free line advances to 0021.**
  - **ADR-0014** — Milestone 4-6 milestone/tier numbering (ARCH-02). Conformance: `conforms`
    (documentation defect; the executing work is D-08's source corrections).
  - **ADR-0015** — `paladin-core` / `paladin-ports` dependency allowlist and the purity invariant
    (ARCH-03b). Conformance: `conforms`.
  - **ADR-0016** — Port value-type ownership; `paladin-core` is canonical (ARCH-03c). Conformance:
    `must change`, executed by Phase 8 / DEBT-05.
  - **ADR-0017** — LLM configuration ownership and the bridge location (ARCH-03d). Conformance:
    `conforms`.
  - **ADR-0018** — Milestone 6 facade re-export policy and its version consequence (ARCH-04).
    Conformance: `conforms`.
  - **ADR-0019** — Binary-target architecture and per-binary purpose (ARCH-06). Conformance:
    `must change`, executed by Phase 16 (mdbook page) with the `structopt` coupling named against
    Phase 8.
  - **ADR-0020** — Build-time benchmark target restated per scenario (ARCH-07). Conformance:
    `conforms`.

- **D-26:** **REQUIREMENTS.md's `## Milestone 4-6 as-shipped ledger` section (line 2830 to the
  `## Milestone 7-8` heading at 3069) is reduced to a pointer** by the scaffold plan, per D-00d —
  not left in place as a second, diverging copy.

### Plan decomposition

- **D-27 [informational]:** **Scaffold → epic fan-out → decisions → source corrections.** Suggested
  shape, ~11-12 plans, sized against Phase 5's proven 13 plans for 118 rows:
  1. **Ledger scaffold** (1 plan) — create `.planning/ledgers/milestone-04-06.md` with the head
     notes (D-02 vocabulary and its ROADMAP-criterion mapping, D-04 caveats, D-01 evidence bar and
     its manifest carve-out, D-05 workspace shape), all 115 row stubs keyed by `REQ-*`, and the
     REQUIREMENTS.md pointer (D-26). Also lands the `STRUCTURE.md` correction (D-05).
  2. **Ledger fan-out by epic** (5 plans) — the 13 epics group naturally by milestone:
     M4 Epics 1-3 (25 IDs), M5 Epics 1-2 (20), M5 Epics 3-4 (20), M5 Epics 5-6 (16),
     M6 Epics 1-4 (34). No plan carries more than ~34 rows, and M6's four epics are the cheapest
     block (0 open items, all four relocations verified complete).
  3. **The decisions** (3 plans) — ADR-0014 + its five source corrections and seven pointer
     banners; ADR-0015 + ADR-0016 + ADR-0017 with the Epic 2 FR-7/FR-10 and Epic 4 FR-31/FR-32
     annotations; ADR-0018 + ADR-0019 + ADR-0020.
  4. **ARCH-05's five source corrections** (1-2 plans) — they touch three `.project/` documents and
     produce five `diverged` rows plus the four `relocated` rows.
  5. **Summary and bookkeeping** (1 plan) — `PROMOTION.md` next-free line, the ledger summary
     section, and the forward-scope handoffs (Phase 8 DEBT-05 target from D-11; Phase 8
     `structopt` precondition from D-20; Phase 11 FACADE-02 D1 from D-16; Phase 15 allowlist check
     from D-10; Phase 16 mdbook page from D-21).
  **ADR-0016 must land before Phase 8 is planned at all** — ROADMAP records DEBT-05 as blocked on
  it.

### Claude's Discretion

- Exact banner wording and inline-correction markup for every `.project/` edit (D-00g fixes the
  pattern — dated banner, retain superseded text — not the prose).
- Whether ADRs 0015/0016/0017 are three files or one combined "run-3 variant answers" ADR. D-25
  recommends three (one question per ADR, matching 0001-0013); a planner with a reason to combine
  may, but 0016 must remain separately citable because Phase 8 depends on it by number.
- How the ledger presents the 22 already-verified run-3 claims — inline per row, or as a
  cross-reference block pointing at `intel/code-verification.md`.
- Whether `present, unproven` and `diverged` counts are reported as headline figures in the
  ledger's summary.
- Ordering within the epic fan-out (D-27 step 2). No dependency either way, though M6 is the
  cheapest and makes a good first fan-out plan.
- Whether the `STRUCTURE.md` correction (D-05) rides in the scaffold plan or gets its own small
  plan.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 7: Workspace Ground Truth & Recorded Answers" (from line 510) —
  the goal and the seven success criteria.
- `.planning/ROADMAP.md` §"Phase 8: Verified Defect Closure" (from line ~528) — records Phase 8 as
  depending on Phase 7 for ARCH-03(c) → DEBT-05 only.
- `.planning/ROADMAP.md` lines 700-730 — the backwards-coupling notes: Phase 7 depends on nothing;
  ARCH-03(a) and ARCH-04 feed Phase 4 (**both already answered by Phase 4 — see D-09, D-14**);
  Phase 11 depends loosely on ARCH-04.
- `.planning/REQUIREMENTS.md` lines 695-810 — **ARCH-01 … ARCH-07 in full**, with their *Derives*
  provenance and the two Phase-4 amendment banners. **This is the authoritative statement of scope.**
- `.planning/REQUIREMENTS.md` lines 2830-3068 — §"Milestone 4-6 as-shipped ledger", the 115 run-3
  rows with component-level verdicts. **This is the input D-01 upgrades and D-26 replaces with a
  pointer.**

### Conventions this phase inherits

- `.planning/decisions/PROMOTION.md` — the numbering index, the next-free line (**0014**), and the
  five-step append procedure. Read before writing any ADR.
- `.planning/decisions/0001-battalion-config.md` … `0013-grove-routing-model.md` — the ADR file
  shape. **0014-0020 must match it** (no frontmatter, per D-00h).
- `.planning/ledgers/milestone-01.md` — **the shape to copy.** Head notes, verdict vocabulary,
  primary-key convention, and the later in-place amendment sections demonstrating D-00f.
- `.planning/ledgers/milestone-02-03.md` — the second instance, and the closest analogue in scale.
- `.planning/phases/05-milestone-2-3-ground-truth/05-CONTEXT.md` — D-01…D-21, source of D-00a…D-00g
  above. Read for the reasoning, not just the conclusions.
- `.planning/milestones/v0.7.1-phases/01-ground-truth-decision-records/01-CONTEXT.md` — the
  original conventions.

### The recorded answers this phase cites but does not re-decide

- `.planning/decisions/0008-workspace-version-0-7-0.md` — answers ARCH-04's major-bump question
  (D-14).
- `.planning/decisions/0009-workspace-rust-edition-2024.md` — answers ARCH-03(a) (D-09).
- `.planning/decisions/0010-milestone-3-epic-numbering.md` — the numbering convention ADR-0014 must
  cite as its precedent (D-07).
- `.planning/decisions/0011-vision-port-surfaces.md` — dispositions the same encryption code
  ARCH-05(1) corrects the gating of (D-17).

### Verification inputs

- `.planning/intel/code-verification.md` lines 82-260 — **the run-3 verification block**: 22
  verified-shipped claims, the resolved-variants section, six verified-open items, the 12-row
  "crate-level facts that contradict run-3 requirement text" table, and the open-checkbox
  implications. Third in the precedence order. **Note D-10 corrects one figure in it**
  (`paladin-ports` carries 11 dependencies today, not 10).
- `.planning/intel/task-completion-state.md` — M4 93.2% (20 open, all Epic 2), M5 96.4% (17 open),
  M6 100%. **Do not re-derive these counts.**
- `.planning/INGEST-CONFLICTS.md` lines 251-340 — the 13 run-3 entries: warning 1 (ARCH-02),
  warnings 2-5 (ARCH-03 a-d), the `vision` gating warning (ARCH-05(1)), warning 7 (ARCH-04),
  the `TokenUsage` warning (ARCH-03c → DEBT-05), the CLI-isolation warning (D-20), the
  build-benchmark warning (ARCH-07), and the byte-equivalent-extracts INFO (D-08).
- `.planning/INGEST-CONFLICTS.md` line ~707 — the mdbook relocation INFO backing D-18.

### Source documents this phase corrects

- `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md` — D-08(1); also the MCP-flag position, ARCH-05(2).
- `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md` — D-08(2); also the `paladin-cli` target structure and Appendix D, ARCH-05(4).
- `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md` — D-08(3); also Epic 2 AC 6 / Epic 4 AC 5 / risk register, ARCH-04 (D-13).
- `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md` FR1 + Design Considerations — ARCH-05(1)(2)(3).
- `.project/Milestone_4-Refactor-Crates-Features/Epic_1/dependency-matrix.md` — the audit that was right; cited, not corrected.
- `.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md` FR-6 + Appendix B — the exhaustive allowlist, ARCH-03(b).
- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` — the Approved-but-DOC decision ADR-0016 promotes (D-11).
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` FR-7, FR-10, FR-11 and §1 — ARCH-03(c) and D-08(5).
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` FR-31, FR-32, Non-Goal 2 — ARCH-03(d) and D-08(4).
- `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md` Non-Goal 7, Open Question 4, and the `src/application/use_cases/` target — ARCH-04 and ARCH-05(5).
- `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md` Goal 7, FR-4.11, FR-4.12 — ARCH-04 and D-18.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` — the five scenarios, the contradictory conclusion, the −6.6%/−5% inconsistency, and the methodology note ADR-0020 transcribes (ARCH-07).
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md` FR-3.5 + SM-7 — the target ADR-0020 restates.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md` FR2.1, FR5.4, §8.3 — the isolation target D-20 re-scopes.

### Codebase maps

- `.planning/codebase/STRUCTURE.md` — **corrected by this phase (D-05).** Currently documents 6 of
  10 library crates.
- `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/CONVENTIONS.md` — the layering rules
  ADR-0015's invariant is expressed against.
- `.planning/PROJECT.md` — already records ten library crates plus `doc-examples` plus `paladin-ai`;
  needs no correction.

### Shipped code cited by the decisions above

- `Cargo.toml:35` and `crates/*/Cargo.toml:4` — **all twelve manifests at `edition = "2024"`**
  (D-09).
- `Cargo.toml:240-252` — the three `[[bin]]` targets and their `required-features` (D-19).
- `Cargo.toml:274` `vision = []`; `:276` `web-server = ["dep:paladin-web", "dep:axum"]`;
  `:284` `cli = [...]` (D-17).
- `src/main.rs:1-30` — the `paladin` binary: `#[structopt(name = "smartcontent-aggregator")]`,
  `Settings::load_from_file`, `setup_and_run` (D-19, D-20).
- `crates/paladin-core/Cargo.toml` `[dependencies]` — 14 entries (D-10).
- `crates/paladin-ports/Cargo.toml` `[dependencies]` — **11 entries**, including `mime_guess`
  (D-10).
- `crates/paladin-core/src/platform/container/token_usage.rs:13` — the canonical `TokenUsage`
  (D-11).
- `crates/paladin-core/src/platform/container/battalion/mod.rs:497` and
  `crates/paladin-llm/src/llm_analysis_service.rs:51` — the two copies DEBT-05 collapses (D-11).
- `crates/paladin-llm/src/config/bridge.rs` — the shipped bridge location (D-12).
- `src/application/` — holds `cli`, `errors`, `mod.rs`, `services`; **no `use_cases/`** (D-13,
  D-17(5)).
- `crates/paladin-herald/Cargo.toml` — `comfy-table = "7.1"` and `colored = "2.1"`, unconditional,
  no `[features]` section (D-20 / specifics 3).
- `src/application/mod.rs:59` — `pub mod cli;`, un-gated (specifics 3).
- `.github/workflows/ci.yml:228` (`crate-isolation`) and `feature-flags.yml:115,118,141` — the
  exercising artefacts D-01's manifest carve-out relies on.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`.planning/ledgers/milestone-01.md` and `milestone-02-03.md`** — two complete, working
  instances of exactly the document this phase must produce, the second at comparable scale (118
  rows vs 115). Copy the shape; do not reinvent it.
- **`.planning/decisions/0001`-`0013`** — thirteen ADRs in the target format. 0008/0009 show how a
  later phase cites an earlier answer instead of re-deciding it (the pattern D-09 and D-14 use);
  0010 shows the ADR-plus-source-correction pairing D-07 repeats.
- **REQUIREMENTS.md's existing 115-row run-3 ledger** — already carries per-`REQ-*` verdicts with
  divergence notes and a status key. D-01 upgrades it; it is a starting point, not a blank page.
- **`intel/code-verification.md`'s run-3 block** — 22 claims already verified against the tree,
  plus a 12-row contradiction table that is essentially ARCH-05's and ARCH-03's evidence
  pre-assembled.
- **`.claude/gsd-core/bin/lib/adr-parser.cjs`** — GSD ships an ADR parser. ADRs 0001-0013 shipped
  without frontmatter; 0014-0020 match the shipped files, not the parser (D-00h).

### Established Patterns

- **Precedence is the project's core mechanic** (D-00b), and this phase writes artefacts at three
  different levels of it — ADRs (top), the codebase map (third), and `.project/` corrections
  (fifth/sixth). Every edit must be legible against the ordering, and D-05 exists because a stale
  map silently outranks a correct ledger.
- **Retain superseded text; amend in place; date every amendment** (D-00f, D-00g). Applies to the
  ledger, to every `.project/` correction, and to `STRUCTURE.md` alike.
- **Contested positions get ADRs; code-settled divergences get ledger rows.** D-17 applies it;
  D-25 allocates against it.
- **The corpus's dominant pattern — the record understating the tree — does not hold uniformly
  here.** M4's open checkboxes are corroborated, M5's are contradicted, M6's are zero and correct
  (D-06). This is the first block requiring a per-milestone heuristic.
- **Medieval military ubiquitous language is mandatory** in code, docs and comments — including in
  the ADRs and the ledger.
- **Documents lie about themselves in both directions.** The build-benchmark report contradicts its
  own table (ARCH-07); `src/main.rs` still calls itself `smartcontent-aggregator`; the "exhaustive"
  allowlist is wrong by eight. Read the artefact and the claim, and trust neither alone.

### Integration Points

- **`.planning/ledgers/milestone-04-06.md`** — new file, sibling to `milestone-01.md` and
  `milestone-02-03.md`. Phases 10 and 13 add their own.
- **`.planning/decisions/0014`…`0020`** — new files; `PROMOTION.md`'s next-free line advances to
  0021.
- **`.planning/codebase/STRUCTURE.md`** — corrected in place (D-05).
- **REQUIREMENTS.md §"Milestone 4-6 as-shipped ledger"** — reduced to a pointer (D-26).
- **Eleven `.project/` documents** — annotated, never rewritten (D-08, D-11, D-12, D-13, D-17).
- **Phase 8 / DEBT-05** — receives the canonical `TokenUsage` target from D-11. **Blocked until
  ADR-0016 lands.**
- **Phase 8 / CLI-isolation** — receives D-20's finding that its recorded three-line fix has a
  precondition (`src/main.rs`'s fate) and a hole (`paladin-herald` re-introduces two of the three
  crates).
- **Phase 11 / FACADE-02 D1** — receives ADR-0018's re-export policy (D-16).
- **Phase 15** — receives the allowlist-enforcement candidate (D-10).
- **Phase 16** — receives the binary-architecture mdbook page (D-21).

</code_context>

<specifics>
## Specific Ideas

**Five findings surfaced during this discussion that the ingest record does not contain.** The
researcher should treat them as verified starting points, not hypotheses — each was read from the
tree during this session.

1. **`paladin-ports` now carries eleven dependencies, not ten.**
   `intel/code-verification.md` records "those 7 plus `serde_json`, `futures`, `md5`" = 10.
   `crates/paladin-ports/Cargo.toml` today also declares `mime_guess = "2"`. ADR-0015's baseline
   must be measured, not transcribed from the intel file.

2. **The `paladin` binary is the pre-Paladin content aggregator.** `src/main.rs` declares
   `#[structopt(name = "smartcontent-aggregator")]`, reads `config.yml` and calls
   `paladin::config::setup::setup_and_run`. ARCH-06 asks for each binary's intended use case; for
   this one the honest answer is "the legacy content-aggregation service runner", and the stale
   `structopt` name should be recorded rather than tidied away.

3. **The recorded "three-line fix" for CLI dependency isolation is wrong in two distinct ways —
   and this re-scopes a Phase 8 item.** Verified by grep this session:
   - **`structopt`'s only consumer is `src/main.rs`**, an un-gated binary. Marking it
     `optional = true` breaks the default build unless `src/main.rs` is gated, migrated to `clap`,
     or retired. That is ARCH-06's question, which is why D-20 names ADR-0019 as its precondition.
   - **`colored` and `comfy-table` re-enter a library-only build through `paladin-herald`.**
     `crates/paladin-herald/Cargo.toml` declares both unconditionally and has **no `[features]`
     section at all** (`markdown_herald.rs` uses `colored`, `table_herald.rs` uses `comfy_table`).
     Gating them in the root manifest cannot satisfy FR5.4's "zero CLI dependencies in
     `cargo tree --lib --no-default-features`". The M4 dependency-matrix classified both as
     CLI-only and was correct *at the time* — `paladin-herald` was extracted in Milestone 8,
     afterwards. The honest ledger verdict is `superseded by shipped code`, not `genuinely
     outstanding`.
   - Related: `src/application/mod.rs:59` declares `pub mod cli;` **un-gated**, so the CLI module
     compiles into the library regardless of the `cli` feature.

4. **`.planning/codebase/STRUCTURE.md` is four crates out of date.** It documents `paladin-core`,
   `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory` and `paladin-storage` and
   omits `paladin-herald`, `paladin-notifications`, `paladin-content`, `paladin-web` and
   `doc-examples` — while sitting **third** in the precedence order, above `intel/` and every PRD.
   ARCH-01's whole point is the workspace shape; correcting the ledger while leaving the map at six
   would leave the wrong number outranking the right one.

5. **ARCH-03(a) and ARCH-04's version clause are already closed — by Phase 4, which ran first.**
   The ROADMAP's backwards-coupling note says "whichever phase executes first records the answer;
   the other applies or ratifies it." Phase 4 executed first on both. All twelve manifests are
   `edition = "2024"` (verified) and ADR-0008 fixes the version story. Phase 7's remaining work on
   these two clauses is **citation, not decision** — a planner who budgets them as open questions
   will produce two ADRs the corpus already has.

**Scale note for the planner:** 115 requirements across 13 epics, against Phase 5's 118 across 14 —
so plan for Phase 5's size, not for the short close-out paragraph in the roadmap. The distribution
is uneven and D-27 exploits it: M6's four epics (34 IDs) are the cheapest block in the corpus
(0 open items, all four relocations verified complete), while M4 Epic 2 (9 IDs) is the most
expensive, being the one genuinely incomplete epic in the run with 20 corroborated open items.

</specifics>

<deferred>
## Deferred Ideas

- **Consolidating the three `TokenUsage` structs** — Phase 8 / DEBT-05. Phase 7 decides which is
  canonical (D-11) and stops there.
- **Fixing the `api-surface` CI job, adding `#[deprecated]` annotations, re-enabling
  `paladin-ports` doctests, and gating the leaked CLI dependencies** — Phase 8 / DEBT-01…DEBT-04.
  Phase 7 records them as `genuinely outstanding` ledger rows, and D-20 hands Phase 8 a corrected
  scope for the CLI one.
- **Building the `cargo tree`-based dependency-allowlist check into CI** — Phase 15. D-10 records
  the invariant; nothing enforces it here.
- **The user-facing binary-architecture page in the mdbook** — Phase 16. D-21 keeps Phase 7 to the
  ADR.
- **Re-measuring the build-time benchmark against a mid-tree monolith baseline** — **declined with
  reason** in ADR-0020 (D-24), not passed forward. Recorded here so a later reader sees it was
  considered and closed rather than forgotten.
- **Retiring or migrating `src/main.rs`** — a real question ARCH-06 exposes (D-19, D-20) but does
  not own. Phase 7 records the binary's purpose and the `structopt` coupling; whether the legacy
  aggregator entry point should survive is new scope belonging to Phase 8's CLI-isolation work or
  its own phase.
- **`paladin-herald` shipping `colored` and `comfy-table` unconditionally with no `[features]`
  section** — surfaced by specifics 3. Making Herald's formatters feature-gated is a code change
  outside this phase; it belongs with Phase 8's CLI-isolation requirement or Phase 11's facade
  work.
- **Nyquist validation for Phases 1-4** — carried forward unresolved from Phase 5's deferred list.
  Owner: `/gsd-validate-phase 1`…`4`.
- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phases 1 and 5. Belongs with Phase 16's documentation work.

</deferred>

---

*Phase: 7-workspace-ground-truth-recorded-answers*
*Context gathered: 2026-08-06*
