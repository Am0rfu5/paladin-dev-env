# Phase 10: Milestone 7-8 Ground Truth & Recorded Account - Context

**Gathered:** 2026-08-08
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision below
carries the reasoning that produced it; **none was confirmed by a human.** Two decisions are flagged
`⚠ HUMAN REVIEW` — restating the extracted-crate dependency invariant that Phase 11 plans its
relocation targets against (**D-15**), and deleting a declared feature from a published crate
(**D-18**). Read those first if you read nothing else.

**Nine gray areas were identified and all nine were auto-selected and resolved:** ledger shape and
evidence bar (HARD-01) · the reconciliation's authority (HARD-02) · the version trajectory (HARD-03) ·
the fourth numbering collision (HARD-04) · the extracted-crate dependency rule (HARD-05) · the PDF
capability answer (HARD-06) · the `cargo doc` bar and doctest posture (HARD-07) · this phase's
code-change boundary (cross-cutting) · ADR allocation and plan decomposition.

<domain>
## Phase Boundary

Make `.planning/` a cited, truthful account of the two milestones that took this workspace to a
published crate family (Milestone 7) and then cleaned up after it (Milestone 8) — and give the three
architecture questions those milestones left ambiguous exactly one recorded answer each. Seven
requirements, HARD-01 … HARD-07.

**Four deliverable classes:**

1. **A cited status ledger** (HARD-01) — `.planning/ledgers/milestone-07-08.md`, the fourth sibling in
   a series the Milestone 4-6 ledger already names by filename, with a `file:line`-cited verdict for
   all **86** run-4 requirement IDs across 11 epics plus 5 cross-milestone entries. Must carry the
   `Superseded by outcome` class unmissably — those are requirements that must **not** be planned as
   written, because implementing them would undo shipped work.
2. **Six new ADRs** (HARD-02 … HARD-07) — **0028-0033**. HARD-01 gets no ADR; a ledger is not a
   contested position (D-00g).
3. **In-repo source corrections under `.project/`** (HARD-02, HARD-03, HARD-04, HARD-05, HARD-06,
   HARD-07) — dated correction banners plus inline annotation, superseded text retained, per D-00c.
4. **A narrow, named config surface** — `crates/paladin-content/Cargo.toml` (one feature line),
   `.cargo/audit.toml` (one comment), `Makefile:433` (one flag). **No `.rs` file is touched.**
   See D-23 for the boundary rule and why it is not zero.

**Not in this phase:**

- **The facade residue and the deferred registers (Phase 11 / FACADE-01 … FACADE-04).** HARD-05 and
  HARD-02(e) *feed* Phase 11 and do not execute against it.
- **Removing the `paladin-content → paladin-llm` edge.** D-21 restates the rule so the edge is legal
  rather than deleting a shipped capability. If a human overturns D-21, the removal is architecture
  work with its own phase, not a clause of a ground-truth phase.
- **The three dead optional dependencies in `paladin-content`** (`scraper`, `rss`, `tiktoken-rs` are
  declared optional and consumed nowhere). Fresh finding, real defect, outside HARD-06. See Deferred.
- **The seven-crate `doctest = false` posture.** HARD-07 records it; Phase 15 / the coverage-and-CI
  quality gates own changing it.
- **REL-01.** It is already `[x] Complete` (Phase 4, ADR-0008). HARD-03 confirms it did not converge
  on an rc.1 figure; it does not re-open it. See D-16.
- **SUPPLY-01 and SUPPLY-02.** Closed by Phase 9 (plan 09-07). Phase 12 inherits verification.
- **Any `.rs` source change.**

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8 and 9 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0027, not the `adr-parser.cjs` schema).
  **`PROMOTION.md:51` records 0028 as next free** — verified this session.
  *(Phase 1 D-01/D-03, Phase 7 D-00a/D-00h, Phase 9 D-00a)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. *(Phase 5 D-08)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02, which names Phase 10 as an inheritor by number.)*
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that
  produced it, recorded verbatim. *(Phases 3, 5, 7, 8, 9)*
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers. *(Phase 1 D-18, Phase 7 D-00e)*
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17, applied by Phases 8 and 9.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md — a standing project-wide convention.)*

---

### HARD-01 — the ledger: home, vocabulary, evidence bar, and what is already closed

- **D-01: New file `.planning/ledgers/milestone-07-08.md`; REQUIREMENTS.md's section becomes a pointer.**
  This is not a judgement call so much as a commitment already made: `.planning/ledgers/milestone-04-06.md`'s
  own head note states "Phases 10 and 13 each add a sibling ledger (`milestone-07-08.md`,
  `milestone-09-12.md`) rather than growing REQUIREMENTS.md further". REQUIREMENTS.md is 4,136 lines
  today; its `## Milestone 7-8 as-shipped ledger` section runs **3121-3317** and is reduced to a
  pointer by the scaffold plan, exactly as Phase 7's D-26 did for Milestone 4-6.
  **Do not leave the REQUIREMENTS.md section in place as a second, diverging copy.**

- **D-02: Seven verdict classes — Phase 7's five plus `deferred with register`, and `superseded by outcome` kept visually distinct.**
  HARD-01 mandates four dispositions (`Shipped`, `Superseded by outcome`, `Relocated`,
  `Deferred with register`); the ledger series already runs a seven-class vocabulary. Carry the
  series vocabulary forward and map HARD-01's four names onto it in the head note, rather than
  inventing a fifth vocabulary for the fourth ledger:
  `satisfied` · `present, unproven` · `genuinely outstanding` · `relocated` ·
  **`superseded by outcome`** · **`deferred with register`** · `diverged`.
  `superseded by outcome` is the one class this ledger must make **unmissable** — ROADMAP criterion 1
  says so in terms, because implementing any of those rows as written would undo shipped work.
  Recommended mechanism: a dedicated summary table at the head of the ledger listing every such row
  by `REQ-*` ID with a one-line "what the tree says instead", so a reader never has to find them by
  scanning 86 rows.
  Chosen over collapsing `relocated` into `superseded by outcome`: the mdbook relocations are the
  single largest false-gap generator in this corpus (`docs/PERFORMANCE_BASELINE.md`,
  `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`, `docs/BUILD_BASELINES.md`,
  `docs/INTEGRATION_TESTS.md`, root `STABLE_API.md` — six documents, four run-4 requirements), and
  collapsing the class destroys the signal that stops a later phase planning them as missing.

- **D-03: Phase 7's evidence bar carries over, manifest carve-out included.**
  No row gets `satisfied` without a `file:line` citation **plus** something that exercises it.
  Milestone 7 is a structural milestone like 4-6, so a manifest line plus a named CI job or build leg
  that consumes it is the exercising artefact (Phase 7 D-01). Behavioural requirements still need a
  test, example or command. An ingest-era status word (`Shipped`, `Verify`, `Variant`) **is** the
  bare "the code exists" claim the bar exists to reject — every one of the 86 rows is re-derived,
  including the ones REQUIREMENTS.md already marked `Shipped`.

- **D-04: Phase 9's seven closed rows are cited, not re-verified — but their citations are re-derived.**
  `REQUIREMENTS.md:1320-1355` carries an explicit hand-off block (Phase 9 plan 09-07, per its D-20)
  naming seven `REQ-*` rows that HARD-01 must record as **already closed by Phase 9**:
  `REQ-rustsec-risk-acceptance`, `REQ-rustsec-hardening-actions`, `REQ-license-policy-signoff`,
  `REQ-crate-metadata-completion`, `REQ-per-crate-changelog`, `REQ-docker-workspace-build`,
  `REQ-paladin-ports-publish-verification-closed`. Cite ADR-0024…0027 and the listed commits; do not
  re-open them. **But re-run each citation** — Phase 9's own sharpest findings all came from
  re-reading `file:line` references that had gone stale, and its close-out amendments in
  REQUIREMENTS.md:3211-3222 are the newest text in the section, not the oldest.

- **D-05: The "14-row table" figure in HARD-01 is wrong by one — reconcile it at source.**
  ⚠ **Fresh finding.** HARD-01 and REQUIREMENTS.md:3136 both size the `Superseded by outcome` class
  from "the 14-row table in `intel/code-verification.md`". Counted directly this session, the
  `### Superseded by shipped outcome — do not plan these as written` table at
  `intel/code-verification.md:365-381` holds **13 data rows** (14 lines beginning `| ` — one of them
  the header). The ledger's class must be built from **the table**, not from the figure, and the
  figure corrected in place per D-00d wherever it appears. A planner who budgets 14 rows and finds 13
  will spend the difference looking for a missing one that does not exist.

- **D-06: The five "provenance pending" crates get their provenance in the ledger head note.**
  HARD-01 requires it explicitly. `paladin-storage`, `paladin-notifications`, `paladin-content` and
  `paladin-web` trace to M7 Epic 1's extraction PRD and its four-Go cost-benefit gate;
  **`paladin-herald` traces to the 2026-06-04 reconciliation, not to any PRD** — which is why no
  ingested requirement described it before run 4, and why the earlier "9-crate workspace" figure was
  wrong. State the workspace shape once, authoritatively: **ten library crates plus `doc-examples`
  plus the root facade package `paladin-ai`**. `.planning/codebase/STRUCTURE.md` was already corrected
  by Phase 7 (verified this session — all ten plus `doc-examples` present at `:51-71`), so unlike
  Phase 7 this phase inherits a correct map and does not have to fix one.

### HARD-02 — the reconciliation as the authoritative account of Milestone 8

- **D-07: ADR-0028 records the supersession; both superseded documents get D-00c annotations; the ledger carries the rows.**
  This is a contested position — two ingested documents assert the opposite of what the tree shows —
  so it gets an ADR under D-00g, not just a ledger row. Three deliverables, matching the Phase 7
  ADR-plus-source-correction pairing:
  `facade-cleanup-RECONCILIATION-2026-06-04.md` is named authoritative;
  `Epic_1/facade-audit.md` and `Epic_3/infrastructure-adapter-disposition.md` are annotated
  superseded at source with the reason.
  **The reason is factual, not procedural, and must be stated that way:** both describe ~4,400 LOC of
  *orphaned, uncompiled duplicate files* as "active bridges that stay".

- **D-08: The reconciliation's verification method is preserved verbatim as a reusable test, and the three in-execution corrections are a named subsection of the ADR.**
  The method — `rg "mod <name>"` across `src/` returns nothing for the file; the directory's `mod.rs`
  only does `pub use paladin_<crate>::…`; the leaf-crate file exists — is the most valuable thing in
  the run-4 corpus, and it is reproducible. Record it as a procedure, not as prose about a procedure.
  The three corrections must survive into the record so nobody re-executes the original audit:
  - **`paladin_registry.rs` was not a duplicate** — the facade's 418-LOC impl was richer than
    battalion's 67-LOC `pub(crate)` copy, so the richer one was consolidated *into* battalion rather
    than deleted blindly.
  - **`sqlite_*_repository.rs` were not redundant** — they were the active default-build impl,
    resolved by making `paladin-storage` non-optional (commit `897e77e`).
  - **The rest genuinely were orphaned** — `mysql_content_repository.rs`, the `input/*` fetchers,
    `document/*`, `output/api_content_deliverer.rs`, `error_log_adapter.rs`.
  Each of the three gets a "do not re-delete" marker in the ledger row, not only in the ADR. The ADR
  is where a reader looks after a question; the ledger row is where a planner looks before one.

- **D-09: Epic 3 complete in substance, Epic 6 complete despite its own record, and the new-crate non-goal recorded as split.**
  Three separate claims, three separate ledger rows, one ADR section:
  - **Epic 3** executed the relocations it had deferred to Milestone 9 — 15 commits, ~10,250 net LOC
    removed, one new leaf crate. Do not plan it as outstanding, and do not plan its relocations as
    Milestone 9 candidates.
  - **Epic 6** is complete despite the reconciliation recording it "Not verified; low priority" and
    `deferred-items.md` omitting it: `crates/paladin-content/src/services/` ships, `lib.rs` declares
    `pub mod services;`, and a workspace-wide grep for `use_cases` returns zero matches.
  - **The M8 Epic 3 §5 non-goal "No new crates created — `paladin-herald`, `paladin-ml`, etc. are
    not in scope" names the exact crate that was then created, in the same milestone.** Record it
    **overridden for `paladin-herald` and still holding for `paladin-ml`.** That split is what
    **FACADE-03(b)** depends on; name FACADE-03 in ADR-0028's `Downstream Consumers` so Phase 11
    cannot re-open it.
  — **Reversibility:** costly — Phase 11 plans FACADE-02 and FACADE-03 against this record; reversing
  means re-deciding which of the five deferred items and two removed features were already executed.

### HARD-03 — the version trajectory as history

- **D-10: ADR-0029 records `v0.1.0-rc.1` as closed history. REL-01 is not re-opened — it is already complete.**
  ⚠ **Fresh finding that changes HARD-03's shape.** HARD-03 says "**Feeds REL-01**, which converges
  the three-way version disagreement". Verified this session: **REL-01 is `[x]` at
  `REQUIREMENTS.md:358` and its traceability row at `:3913` reads `Phase 4 | Complete`.** Phase 4
  already converged, on `0.7.0`, via ADR-0008. HARD-03's live job is therefore **backwards-looking
  confirmation**, not a hand-off: record the history, and record that REL-01 did not converge on any
  rc.1 figure. A planner who treats REL-01 as open will re-plan a closed requirement.

- **D-11: The tree has moved past every current-state figure in HARD-03's own text; correct them at source.**
  ⚠ **Fresh finding.** HARD-03 states "Current tree: `Cargo.toml` `0.6.0`, branch `release/v0.7.0`,
  latest tag `v0.5.1`". Verified this session:
  - `Cargo.toml:34` — `version = "0.7.0"` (Phase 4 plan 04-05, commit `c2e20a1`, converged every
    manifest and internal pin on 0.7.0).
  - `git tag --sort=-v:refname | head` — **`v0.7.1`, `v0.7.0`**, then `v0.5.1`. Milestone 1's
    close-out shipped `v0.7.1` on 2026-08-04.
  - Branch is still `release/v0.7.0`.
  The historical facts HARD-03 records — the lockstep `0.2.0` target, the ten crates published at
  `0.1.0`, tag `v0.1.0-rc.1` at `a9530fc` on 2026-05-28, the GO sign-off, docs.rs verification of all
  ten including the `paladin-ai-core`/`paladin_core` package-lib split, the external smoke project —
  are unchanged and are what the ADR transcribes. The *current-state* clause is stale and gets the
  D-00c/D-00d treatment in REQUIREMENTS.md and the ROADMAP phase section.

- **D-12: ADR-0029 is the single home for the whole trajectory; Phase 13 / ORCH-05 extends it rather than writing a second version ADR.**
  HARD-03 covers rc.1 → v0.2.0; ORCH-05 (Phase 13) covers v0.3.0 → v0.6.0; REL-01 (Phase 4, done)
  covers the landing at 0.7.0. Three ADRs for one unbroken line would guarantee the third contradicts
  the first. ADR-0029 is written with a `## Trajectory` table that ORCH-05 appends rows to, and says
  so in `Downstream Consumers`. Whichever of the two runs second applies rather than re-decides — the
  ROADMAP's own coupling note at `REQUIREMENTS.md:4036` states that rule; this decision just names
  the artefact it applies to.

### HARD-04 — the fourth milestone-numbering collision

- **D-13: ADR-0030, citing ADR-0010 and ADR-0014 as its two precedents; the M7 overview corrected at source.**
  Same convention, third application: **directory / task-list numbering is authoritative.** The
  Milestone 7 overview titles itself "Milestone 4: Production Hardening and Extended Workspace
  Decomposition" while its path is `Milestone_7-Production-Hardening`, and its Prerequisites credit
  "Milestones 1-3" with work the directory numbering assigns to Milestones 4-6 — feature flags and
  the CI matrix, the core workspace crates, `application_settings.rs` decomposition, manager-service
  relocation, Maneuver DSL co-location, `CircuitBreaker` relocation. Every one of those six is a
  Milestone 4-6 deliverable already ledgered in `milestone-04-06.md`; the ADR cites that ledger
  rather than re-asserting the mapping.
  The ADR **must** cite 0010 and 0014 explicitly. That cross-reference is what makes `REQ-*`
  provenance keys resolve uniformly across four ledgers — it is the whole reason the convention is
  worth an ADR each time instead of a footnote.

- **D-14: The "expect a fifth in run 5" prediction is recorded as already closed, not carried forward.**
  `ROADMAP.md:112-114` states it: "The protocol predicted a fifth instance in run 5; run 5 found
  none, and ORCH-05 records the prediction closed." ADR-0030 records that the Roadmap Extension
  Protocol item is discharged with this fourth instance and that no fifth exists, so no later phase
  inherits a standing prediction to check.

### HARD-05 — the extracted-crate dependency rule

- **D-15: The rule is restated as "never, except behind a non-default optional feature the facade opts into explicitly." ADR-0031.**
  ⚠ **HUMAN REVIEW — this is the answer Phase 11 plans
  FACADE-02's D2/D3/D4 relocation targets against.**
  The two permitted answers were (a) "never", with `paladin-content → paladin-llm` removed from the
  tree, or (b) "never, except behind an optional feature", with the rule restated. **(b) is
  recommended.**
  Verified this session:
  - `crates/paladin-content/Cargo.toml:23` — `llm = ["dep:paladin-llm"]`, **not** in any default
    feature set.
  - `:28` — `paladin-llm = { version = "0.7.0", path = "../paladin-llm", optional = true }`. (Note
    the version: **0.7.0**, not the `0.6.0` HARD-05's text quotes.)
  - `crates/paladin-content/src/services/mod.rs:7` — `#[cfg(feature = "llm")]`, gating exactly one
    module. `content_llm_analysis_service.rs:8` is its only consumer of `paladin_llm`.
  - Root `Cargo.toml:275` — the facade's `content-processing` enables `paladin-content/llm`, so the
    opt-in is explicit and lives one level up.
  **So the default build of `paladin-content` carries no leaf-to-leaf edge at all.** That is the fact
  the rule should be written against.
  Reasoning, in order of weight:
  1. **The invariant that has teeth is the default-build one**, and it is the same shape ADR-0015
     already used for `paladin-core`/`paladin-ports` — separate the enforceable invariant from the
     list. State it as: *no extracted crate may depend on another extracted crate or on the facade
     in its default build; a non-default optional feature may declare such an edge only where the
     facade opts in explicitly and the dependent code is `cfg`-gated.* That is checkable with
     `cargo tree --no-default-features` — the same Phase 15 mechanism ADR-0015 is waiting on.
  2. **The PRD anticipated this exact case and did not amend itself.** M7 Epic 1 §4.4 says
     "use-case services depend on `paladin-llm` for LLM analysis, creating an inter-crate dependency
     that must be handled carefully". §6.1's absolute form was written without §4.4's case in view.
  3. **Option (a) is not a record change, it is architecture work.** Removing the edge means either
     deleting a shipped, facade-exposed capability or inverting it through a port — real design, real
     `.rs` churn, and outside a ground-truth phase's boundary (D-23).
  4. Milestone 8's reconciliation kept the edge while deleting ~10,250 LOC of everything else. It was
     looked at and left.
  **On the SPEC-candidate clause:** HARD-05 calls this "the strongest SPEC candidate in run 4" and
  suggests re-tagging `prd-extract-infrastructure-crates.md` via `--manifest` and re-running ingest.
  **Do not.** Phase 7's D-11 settled this pattern: **an ADR *is* the promotion** — it restates the
  position inside `.planning/decisions/`, which sits at the top of the precedence order by
  construction, and cites the PRD as provenance. Re-typing a `.project/` file changes how five
  completed ingest runs classified their corpus, for an outcome an ADR achieves natively.
  ADR-0031 names **Phase 11 / FACADE-02** in `Downstream Consumers`.
  — **Reversibility:** costly — D2/D3/D4's relocation targets in FACADE-02 are chosen against this
  answer; reversing means re-planning three relocations and removing a shipped facade capability.

- **D-16: M7 Epic 1 PRD §6.1 and Goal 2 are annotated at source, and the ledger row moves from `Code diverges` to `satisfied`.**
  `REQUIREMENTS.md:3159` currently reads **`Code diverges → HARD-05`** for
  `REQ-extracted-crate-dependency-rule`. Once ADR-0031 lands, the tree conforms to the restated rule
  and the row becomes `satisfied` with the ADR as its citation — the divergence was in the *rule's
  wording*, not in the code. Say that explicitly in the row; a bare verdict flip with no explanation
  is exactly the kind of unexplained status change this ledger series exists to prevent.

### HARD-06 — is PDF extraction supported?

- **D-17: The answer is yes, unconditionally, whenever `paladin-content` builds. ADR-0032.**
  HARD-06 lists three facts pointing in two directions. Read directly this session, there is a fourth
  that settles it:
  - `crates/paladin-content/Cargo.toml:18` — `pdf = []`, an empty feature.
  - `:41` — `pdf-extract = { version = "0.7" }`, **unconditional**, no `optional = true`.
  - **`grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/` returns zero matches**, and
    `crates/paladin-content/src/adapters/document/mod.rs` declares `pub mod pdf_extractor;` and
    `pub use pdf_extractor::PdfExtractor;` unconditionally. `document_adapter.rs:22` holds
    `pdf_extractor: PdfExtractor` as a plain struct field, constructed at `:29` and called at `:123`
    and `:132`.
  - Root `Cargo.toml:275` — `content-processing` omits `pdf`, which is harmless **because `pdf` gates
    nothing.**
  So: PDF extraction ships, always, in every build of `paladin-content`. The facade's five-of-six
  feature list is not a capability gap. Phase 9's D-17 reached the same conclusion from the manifest
  alone and explicitly handed the capability question here; this session adds the source-level half.
  **The `.cargo/audit.toml` note is right about reachability and wrong about the mechanism.** Its
  stated grounds — "lopdf is transitive via `pdf-extract` (optional `content-processing`)" — are
  accurate at the facade level (`Cargo.toml:59`, `paladin-content` is `optional = true`) and
  misleading in implying the `pdf` feature gates it. `RUSTSEC-2026-0187`'s suppression stands; only
  its parenthetical needs correcting.

- **D-18: Delete the inert `pdf` feature. Do not wire it.** ⚠ **HUMAN REVIEW — removing a declared
  feature from a published crate is a public-contract change.**
  Three options existed: wire `pdf` to gate `pdf-extract` and add it to `content-processing`; delete
  the inert feature; or record the answer and change nothing.
  **Delete is recommended**, because wiring it is not a no-op:
  1. `DocumentAdapter` holds `PdfExtractor` as an ungated struct field. Making the dependency optional
     requires `cfg`-gating that field, its constructor and two call sites — a `.rs` change, and one
     that turns PDF extraction into an **opt-out** capability for every existing consumer of the
     published `paladin-content 0.1.0`+.
  2. `news-api = []` is a second empty feature and it is **legitimate** — it gates
     `adapters/input/mod.rs:5`'s `news_api_fetcher` module, which needs no dependency of its own. So
     "empty feature" is not itself the defect. `pdf` is the only feature in the crate that is inert
     in **both** directions: it gates no dependency and no code.
  3. Deletion makes the manifest tell the truth in one line and cannot change any build's behaviour
     except `--features pdf`, which today enables nothing.
  **The accepted cost, which the plan must state:** `cargo build -p paladin-content --features pdf`
  begins to fail where it previously succeeded-and-did-nothing. That is a minor public-contract
  change on a pre-1.0 crate family and belongs in `crates/paladin-content/CHANGELOG.md`.
  **Fallback if a human overturns this:** keep `pdf = []`, add it to the facade's
  `content-processing` list for §4.4.6 literal compliance, and record in the ADR that the feature is
  a documentation marker with no gating effect. That also closes HARD-06 — but it leaves a manifest
  that lies quietly, which is the defect class this whole milestone-close-out exists to retire.
  **Both branches close HARD-06; the plan must state which one it took and why.**
  — **Reversibility:** costly — a published feature name, restorable in one line but visible to
  consumers.

- **D-19: M7 Epic 1 §4.4.1 and §4.4.6 are annotated superseded at source; `.cargo/audit.toml`'s parenthetical is corrected.**
  §4.4.1 requires `pdf` to gate `pdf-extract`; §4.4.6 requires `content-processing` to activate
  `paladin-content` "with **all** capability features enabled". Both are superseded by outcome under
  D-17. The `.cargo/audit.toml` comment at `:26-29` is corrected to name the actual path
  (`pdf-extract` is unconditional in `paladin-content`; `paladin-content` is optional in the facade)
  so `SECURITY-EXCEPTIONS.md`'s compensating-control row for `RUSTSEC-2026-0187` rests on a true
  statement. **This is the input Phase 12 / SUPPLY-02 was told to wait for** — record it as an answer
  delivered, so Phase 12 does not re-derive it.

### HARD-07 — the `cargo doc` bar and the doctest posture

- **D-20: The bar is zero warnings on `cargo doc --workspace --no-deps`. The tree already enforces it. ADR-0033 ratifies the shipped answer.**
  ⚠ **Fresh finding that shrinks HARD-07 substantially.** Verified this session:
  - `.github/workflows/ci.yml:58` —
    `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`.
    **A zero-warning gate ships today, in the required `lint` job.**
  - All ten library crates plus the facade carry `#![warn(missing_docs)]`.
  M7 Epic 4 §4.4.3 and M7 Epic 1 §4.6.4/§8.9 (zero warnings) are what shipped; **M8 Epic 5 FR-19's
  "warnings acceptable; must not fail" is the minority position and is annotated superseded by
  outcome** at source, with the ledger row for `REQ-m8-final-quality-gate` (`REQUIREMENTS.md:3281`)
  updated to say so. This is ratification of a shipped answer, not a new decision — the same move
  Phase 7's D-09 and D-13 made.

- **D-21: DEBT-03 is already closed. HARD-07's "resolve alongside DEBT-03" clause resolves to a three-word Makefile fix.**
  ⚠ **Fresh finding.** HARD-07 and `REQUIREMENTS.md:3178`/`:3214` both rest on
  `crates/paladin-ports/Cargo.toml` setting `[lib] doctest = false` and `ci.yml:225` excluding the
  crate from `--doc`. Verified this session, **both are gone**:
  - `crates/paladin-ports/Cargo.toml` has **no `[lib]` section at all**. `git log --oneline` on that
    file shows **`2bffe22 feat(08-03): re-enable paladin-ports doctests`** — Phase 8 closed it.
  - `ci.yml:238` is a bare `cargo test --workspace --doc`. No `--exclude`. (The record's `:225`
    citation is stale by 13 lines and by content.)
  - `Makefile:123` (`test-doc`) is already clean.
  **The one surviving residue is `Makefile:433`**, inside `release-check`:
  `@$(CARGO) test --workspace --doc --exclude paladin-ports`, preceded at `:432` by an echo that
  still explains the exclusion as "doctests reference root crate not yet published" — a reason that
  stopped being true when the crate was published at `0.1.0` and stopped being implemented when Phase
  8 re-enabled the doctests. Delete the flag and the stale echo. The `--exclude` also makes
  `release-check` **weaker than CI**, which is the wrong direction for a release gate.
  The "unwritten Task 7.0" HARD-07 says has been deferred since run 3 is therefore **discharged**, and
  the ADR records it as discharged by Phase 8 rather than by this phase.

- **D-22: The seven-crate `doctest = false` posture is recorded, not decided, and handed to Phase 15.**
  Second-order finding this session: `[lib] doctest = false` is still set by **seven** crates —
  `paladin-content`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-llm`,
  `paladin-herald` and `doc-examples` (`grep -n doctest crates/*/Cargo.toml`). Only `paladin-core`,
  `paladin-ports`, `paladin-battalion` and `paladin-web` run doctests. That is the live shape of M7
  Epic 4 §4.4.4's ">90% documented-public-item coverage" posture, recorded **Met** by
  `epic-4-completion-summary.md`, and it is the honest context for `REQ-doc-coverage-audit`
  (`REQUIREMENTS.md:3214`, currently `Contested → HARD-07`).
  ADR-0033 states the number and the list, marks the row `present, unproven` rather than `satisfied`
  — the coverage claim is real for the four crates that run doctests and unmeasured for the seven
  that do not — and names **Phase 15 / the coverage-and-CI quality gates** as the owner of changing
  it. Deciding seven crates' doctest posture is not a ground-truth deliverable; leaving the number
  unwritten while flipping the row to `satisfied` would be.

### Cross-cutting

- **D-23: Phase 10's code-change boundary — record-writing, plus a three-file config surface, and no `.rs`.**
  Phase 7 was record-only; Phase 9 was config-changing. Phase 10 sits between them, and the boundary
  is set by ROADMAP criteria 5 and 6, which both require the record and the tree to **agree**. For
  both, the cheaper side to move is the record — D-15 restates a rule rather than removing an edge;
  D-18 deletes a vestige rather than wiring a capability. The complete permitted surface:
  - `crates/paladin-content/Cargo.toml:18` — delete `pdf = []` (D-18).
  - `.cargo/audit.toml:26-29` — correct the `-0187` parenthetical (D-19).
  - `Makefile:432-433` — delete `--exclude paladin-ports` and the stale echo (D-21).
  Everything else this phase touches is under `.planning/` or `.project/`. **No `.rs` file is
  modified.** Any plan proposing one has found new scope and should say so rather than absorb it.
  Every plan is still subject to the CLAUDE.md workspace gate (`cargo test` → `cargo fmt --check` →
  `cargo clippy -- -D warnings`) and to ADR-0006's 84% workspace line-coverage floor — a phase that
  changes no `.rs` should not move coverage, and the close-out should confirm exactly that.

- **D-24: ADR allocation — 0028 through 0033; `PROMOTION.md` advances to 0034.**
  Next free is **0028**, verified this session at `PROMOTION.md:51`.
  - **ADR-0028** — Milestone 8's authoritative account: the reconciliation supersedes the Epic 1
    audit and the Epic 3 disposition record, with the reproducible orphan test, the three
    in-execution corrections, Epic 3/Epic 6 completeness, and the herald/`paladin-ml` non-goal split
    (D-07, D-08, D-09). Conformance: `conforms`.
  - **ADR-0029** — Version trajectory: `v0.1.0-rc.1` as closed history, with a trajectory table
    ORCH-05 appends to (D-10, D-11, D-12). Conformance: `conforms`.
  - **ADR-0030** — Milestone 7 self-numbering collision, citing ADR-0010 and ADR-0014 (D-13, D-14).
    Conformance: `conforms`.
  - **ADR-0031** — Extracted-crate dependency rule, restated as a default-build invariant (D-15,
    D-16). Conformance: `conforms`. `Downstream Consumers`: Phase 11 / FACADE-02, Phase 15.
  - **ADR-0032** — PDF extraction capability and the inert `pdf` feature (D-17, D-18, D-19).
    Conformance: `must change`, executed in this phase.
  - **ADR-0033** — The `cargo doc` zero-warning bar, DEBT-03's discharge, and the seven-crate doctest
    posture (D-20, D-21, D-22). Conformance: `must change` for `Makefile:433`, executed in this
    phase; the doctest posture named against Phase 15.
  **HARD-01 gets no ADR** — a ledger is not a contested position (D-00g).

- **D-25: Every closure claim is proved by a command run in this environment and recorded verbatim.**
  The D-00e bar, and this phase is unusually well-placed to meet it: nearly every HARD-01 verdict is a
  `grep`/`sed`/`git log` read of files in this checkout. **Not runnable here** (unchanged from Phase
  9): `cargo audit`, `cargo deny`, `cargo llvm-cov` and anything Docker — `crates.io` returns HTTP
  403 and `docker` is absent (`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`).
  `cargo doc --workspace --no-deps` and `cargo test --workspace --doc` should be **attempted** —
  they need no network if the lockfile's crates are vendored or cached — and if they cannot run, the
  ADR-0033 claims are scoped as CI-only, never inferred as passing.

- **D-26: Forward hand-offs are written explicitly, in the same shape Phase 9 used for this phase.**
  Phase 9's `REQUIREMENTS.md:1320-1355` hand-off block is the model, and this phase owes four:
  - **Phase 11 / FACADE-02** — ADR-0031's restated rule (which legalises D2/D3/D4's leaf-to-leaf
    relocation targets) and ADR-0028's record that the Epic 3 relocations already executed.
  - **Phase 11 / FACADE-03(b)** — the non-goal split: overridden for `paladin-herald`, still holding
    for `paladin-ml`.
  - **Phase 12 / SUPPLY-02** — D-19's answer to the `pdf-extract` reachability question, delivered
    rather than deferred.
  - **Phase 13 / ORCH-05** — ADR-0029's trajectory table to append to, and the note that REL-01 is
    already converged (D-10).

- **D-27 [informational]: Suggested decomposition — ~9 plans, 4 waves.**
  Sized against Phase 7's 13 plans for 115 rows and Phase 5's 13 for 118. Phase 10 has 86 rows, and
  a materially higher share of them are already-settled (`Shipped`, or closed by Phase 9), so the
  fan-out is cheaper per row than Phase 7's.
  - **Wave 1:** ① **Ledger scaffold** — `.planning/ledgers/milestone-07-08.md` with the head notes
    (D-02 vocabulary and its HARD-01 mapping, D-03 evidence bar and manifest carve-out, D-06
    workspace shape and the five crates' provenance, D-05's corrected 13-row figure), all 86 row
    stubs keyed by `REQ-*`, the `Superseded by outcome` summary table, and the REQUIREMENTS.md
    pointer (D-01).
  - **Wave 2 (fully parallel, blocked on ①):** ② M7 Epics 1-2 (25 IDs) · ③ M7 Epics 3-4 (22 IDs,
    includes the seven Phase-9-closed rows per D-04) · ④ M8 Epics 1-4 (18 IDs) · ⑤ M8 Epics 5-7 plus
    the 5 cross-milestone entries (21 IDs).
  - **Wave 3 (parallel with wave 2 — no file overlap with the ledger):**
    ⑥ **ADR-0028** + the two M8 source annotations (D-07, D-08, D-09).
    ⑦ **ADR-0029 + ADR-0030** + the M7 overview and version-figure corrections (D-10…D-14).
    ⑧ **ADR-0031 + ADR-0032 + ADR-0033** + the three-file config surface (D-15…D-23). **Gate ⑧ on a
    blocking human checkpoint before its first task** — D-15 and D-18 are both flagged, and D-18 is
    `costly` on a published crate.
  - **Wave 4:** ⑨ **Close-out** — HARD-01…HARD-07 checkbox flips behind evidence, the traceability
    rows, `PROMOTION.md` → 0034, `PROJECT.md` Key Decisions rows, the four D-26 hand-off blocks, and
    the ADR-0006 coverage re-check (expected: unchanged, no `.rs` touched).
  Plan-file naming is `10-NN-PLAN.md`.
  **File contention to respect:** `.planning/ledgers/milestone-07-08.md` is written by ① and appended
  by ②-⑤ — give each fan-out plan a disjoint epic range and it is append-only per section.
  `REQUIREMENTS.md` is touched by ①(pointer), ⑧(row verdicts) and ⑨(checkboxes) — serialise those
  three, they are in different waves already.

### Claude's Discretion

- Whether the `Superseded by outcome` summary table (D-02) sits at the head of the ledger, at the
  foot, or in both places. The constraint is that a planner must not have to scan 86 rows to find
  them.
- Whether ADR-0031, 0032 and 0033 are three files or fold into fewer. D-24 recommends three (one
  question per ADR, matching 0001-0027); 0031 must remain separately citable because Phase 11 depends
  on it by number.
- The exact wording of the restated dependency invariant in ADR-0031, provided it is expressed
  against the **default build** and is checkable by a command.
- Exact banner wording and inline-correction markup for every `.project/` annotation (D-00c fixes the
  pattern, not the prose).
- How the ledger presents the run-4 claims `intel/code-verification.md` already verified — inline per
  row, or as a cross-reference block. Phase 7 left the same choice open.
- Whether the `Makefile:432-433` fix (D-21) rides in plan ⑧ or gets folded into the close-out.
- Whether ADR-0033 also records the four crates that *do* run doctests as a positive baseline, or
  only the seven that do not.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 10: Milestone 7-8 Ground Truth & Recorded Account" (line 653) — the
  goal, the independence note, and the **seven** success criteria. Criteria 5 and 6 are the two that
  require the record and the tree to agree; they set D-23's boundary.
- `.planning/REQUIREMENTS.md` lines 1357-1482 — **HARD-01 … HARD-07 in full**, with their *Derives*
  provenance. **This is the authoritative statement of scope** and it is much longer than the ROADMAP
  summary. Note D-05 (the 14-vs-13 row count), D-10 (REL-01 is already complete) and D-11 (the
  current-state version figures are stale) all correct text inside this block.
- `.planning/REQUIREMENTS.md` lines 1320-1355 — **Phase 9's hand-off block**, naming the seven `REQ-*`
  rows HARD-01 must record as already closed, with their ADRs and commits (D-04).
- `.planning/REQUIREMENTS.md` lines 3121-3317 — **the existing Milestone 7-8 as-shipped ledger**, 86
  rows with component-level verdicts. **This is the input D-03 upgrades and D-01 replaces with a
  pointer.** Its Phase-9 amendments at `:3211-3222` are the newest text in it.
- `.planning/ROADMAP.md` §"Phase 11: Facade Residue & Deferred Register Disposition" (line 670) —
  records Phase 11 as gated on HARD-05 for D2/D3/D4's relocation targets (D-15, D-26).
- `.planning/ROADMAP.md` §"Phase 12: Supply-Chain Gate Integrity" (line 687) and its dated closure
  note at lines 700-711 — SUPPLY-01 and SUPPLY-02 are **closed by Phase 9**; only SUPPLY-03 remains,
  and it receives D-19's PDF answer.
- `.planning/ROADMAP.md` §"Phase 13" (line 714) — ORCH-05 completes the trajectory ADR-0029 starts
  (D-12).
- `.planning/REQUIREMENTS.md` lines 4024-4040 — the cross-phase coupling table: HARD-03 → REL-01
  (**already complete**, D-10), HARD-03 → ORCH-05, HARD-05 → FACADE-02, HARD-06 → SUPPLY-02,
  HARD-07 → DOCS-03.

### Conventions this phase inherits

- `.planning/decisions/PROMOTION.md` — the numbering index, **next free 0028** (`:51`), and the
  five-step append procedure. Read before writing ADR-0028…0033; advance to **0034** in the close-out.
- `.planning/decisions/0001-battalion-config.md` … `0027-dockerfile-chef-planner-stage.md` — the ADR
  file shape. **0028-0033 must match it** (no frontmatter, seven headings, per D-00a).
- `.planning/ledgers/milestone-04-06.md` — **the shape to copy**, and the file whose head note already
  names `milestone-07-08.md` as this phase's deliverable (D-01). Read its head notes for the evidence
  bar, the manifest carve-out, and the "an ingest status word is the claim the bar rejects" rule.
- `.planning/ledgers/milestone-01.md`, `.planning/ledgers/milestone-02-03.md` — the first two
  instances; `milestone-01.md` demonstrates the D-00d in-place amendment sections.
- `.planning/phases/07-workspace-ground-truth-recorded-answers/07-CONTEXT.md` — **the closest
  analogue to this phase.** Source of the ledger-plus-ADRs-plus-source-corrections shape, the
  ADR-is-the-promotion rule (its D-11, which D-15 applies), and the record-only boundary D-23 widens
  by exactly three files.
- `.planning/phases/09-release-security-gate-integrity/09-CONTEXT.md` — source of D-00a…D-00h, the
  ⚠ HUMAN REVIEW convention, and **its D-17**, which supplies HARD-06's manifest-level evidence and
  explicitly declines to answer the capability question.

### The recorded answers this phase cites but does not re-decide

- `.planning/decisions/0006-coverage-gate.md` — the 84% workspace line-coverage floor. No `.rs`
  changes here; the close-out confirms the number is unmoved.
- `.planning/decisions/0008-workspace-version-0-7-0.md` — Phase 4's version convergence. **REL-01 is
  closed by this** (D-10); ADR-0029 records the history behind it, not a competing answer.
- `.planning/decisions/0010-milestone-3-epic-numbering.md` and
  `.planning/decisions/0014-milestone-4-6-tier-numbering.md` — the two numbering precedents ADR-0030
  must cite (D-13).
- `.planning/decisions/0015-core-ports-dependency-allowlist.md` — **the model for ADR-0031**:
  separate the enforceable invariant from the list, and leave enforcement to Phase 15 (D-15).
- `.planning/decisions/0023-cli-dependency-isolation.md` — Phase 8's clap v4 migration, which is why
  four `deny.toml` suppressions went dead and why several run-4 citations moved.
- `.planning/decisions/0024-rustsec-exception-governance.md` … `0027-dockerfile-chef-planner-stage.md`
  and `SECURITY-EXCEPTIONS.md` — Phase 9's four ADRs and the register. D-04's seven rows cite these;
  D-19 corrects one sentence of the `-0187` reasoning behind them.

### Verification inputs

- `.planning/intel/code-verification.md` lines **263-414** — **the run-4 verification block.** The
  17-row Verified SHIPPED table, the six Verified OPEN items, the **13-row** (not 14 — D-05)
  `Superseded by shipped outcome` table at `:365-381`, the two favourable-direction contradictions at
  `:383-394` (Epic 6 and Epic 3, which are HARD-02(b) and (d)), and the open-checkbox implications at
  `:396-413`. **Third in the precedence order.** Note that its `ci.yml:406` citation is stale — the
  job it names was deleted by Phase 9.
- `.planning/intel/task-completion-state.md` — Milestone 7 at 98.8% (3 open, all in
  `tasks-production-build-infra-adaptation.md`), Milestone 8 at 99.1% (3 open). **Do not re-derive
  these counts, and do not trust them**: run 4 established that Milestone 8's three are contradicted
  by code and Milestone 7's three are plausible but uncorroborated.
- `.planning/INGEST-CONFLICTS.md` — the run-4 warnings on the disposition record, the new-crate
  non-goal, the dependency rule, the `content-processing` feature, and the two `cargo doc` bars.
- `.planning/intel/context.md` — Topic: Version trajectory across runs 1-4 (HARD-03); Topic:
  Milestone 7 scope, structure and self-numbering (HARD-04).
- `.planning/intel/constraints.md` — the run-4 "strongest SPEC candidate" entry (HARD-05, D-15).

### Source documents this phase records or corrects

- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/.../facade-cleanup-RECONCILIATION-2026-06-04.md`
  — **the authoritative account** (HARD-02). Its §2 orphan test, §7 commit list (`66f6c4e`,
  `8bd7073`, `ff829e2`, `5a7c901`, `897e77e`, `4c7857e`), and the three in-execution corrections.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` — annotated superseded
  (D-07).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md`
  — annotated superseded (D-07); also FACADE-04's subject (`paladin-arsenal` / `paladin-sanctum`).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/` PRD §5 Non-Goals — the "No new crates
  created" clause naming `paladin-herald` and `paladin-ml` (D-09).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/` PRD FR-19 — "warnings acceptable;
  must not fail", annotated superseded (D-20).
- `.project/Milestone_7-Production-Hardening/Milestone-Overview/` — the overview titling itself
  "Milestone 4" and crediting "Milestones 1-3" (D-13).
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` — §6.1 the
  absolute dependency rule, Goal 2, §4.4 the complexity note that anticipated the violation, §4.4.1
  `pdf` gating `pdf-extract`, §4.4.6 "all capability features enabled", §4.6.4 and §8.9 the
  zero-warning `cargo doc` bar (D-15, D-19, D-20).
- `.project/Milestone_7-Production-Hardening/Epic_4/` PRD §4.4.1 `#![warn(missing_docs)]`, §4.4.3 the
  zero-warning bar, §4.4.4 the >90% coverage target, and `epic-4-completion-summary.md` recording
  both **Met** (D-20, D-22); Appendix C's lockstep `0.2.0` target (D-10).
- `.project/Milestone_7-Production-Hardening/Epic_4/` release-readiness audit and post-release
  verification — every gate PASS, GO sign-off, tag `v0.1.0-rc.1` at `a9530fc`, docs.rs verification,
  the external smoke project (D-10).

### Defect and change sites — all verified this session, 2026-08-08

**HARD-05:**
- `crates/paladin-content/Cargo.toml:23` — `llm = ["dep:paladin-llm"]`, non-default.
- `:28` — `paladin-llm = { version = "0.7.0", path = "../paladin-llm", optional = true }`. **The
  record's `version = "0.6.0"` is stale** — Phase 4 converged every pin on 0.7.0 (commit `c2e20a1`).
- `crates/paladin-content/src/services/mod.rs:7` — `#[cfg(feature = "llm")]`.
- `crates/paladin-content/src/services/content_llm_analysis_service.rs:8` — the only `paladin_llm`
  consumer in the crate.
- `Cargo.toml:275` — `content-processing = ["dep:paladin-content", "paladin-content/web-scraping",
  "paladin-content/rss", "paladin-content/news-api", "paladin-content/tiktoken",
  "paladin-content/llm", "paladin-memory/content-processing"]`.

**HARD-06:**
- `crates/paladin-content/Cargo.toml:18` — `pdf = []`; `:21` — `news-api = []`.
- `:41` — `pdf-extract = { version = "0.7" }`, unconditional.
- `crates/paladin-content/src/adapters/document/mod.rs` — `pub mod pdf_extractor;` and
  `pub use pdf_extractor::PdfExtractor;`, both unconditional.
- `crates/paladin-content/src/adapters/document/document_adapter.rs:22,29,123,132` — `PdfExtractor`
  as an ungated struct field, constructor and two call sites.
- `crates/paladin-content/src/adapters/input/mod.rs:5` — `#[cfg(feature = "news-api")]`, the proof
  that an empty feature can legitimately gate code.
- `grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/` — **zero matches.**
- `Cargo.toml:59` — `paladin-content = { workspace = true, optional = true }`.
- `.cargo/audit.toml:26-29` — the `RUSTSEC-2026-0187` entry and its parenthetical (D-19).

**HARD-07:**
- `.github/workflows/ci.yml:58` — the zero-warning `cargo doc --workspace --no-deps` gate.
- `.github/workflows/ci.yml:238` — `cargo test --workspace --doc`, **no `--exclude`**. The record's
  `ci.yml:225` citation is stale.
- `crates/paladin-ports/Cargo.toml` — **no `[lib]` section**; `git log --oneline` shows
  `2bffe22 feat(08-03): re-enable paladin-ports doctests`.
- `Makefile:432-433` — the surviving `--exclude paladin-ports` and its stale echo (D-21).
- `Makefile:123` — `test-doc`, already clean.
- `grep -n doctest crates/*/Cargo.toml` — seven crates still set `doctest = false`:
  `paladin-content:15`, `paladin-memory:15`, `paladin-storage:15`, `paladin-notifications:15`,
  `paladin-llm:15`, `paladin-herald:15`, `doc-examples:9` (D-22).
- `src/lib.rs:116` and all ten `crates/*/src/lib.rs` — `#![warn(missing_docs)]`.

**HARD-03:**
- `Cargo.toml:34` — `version = "0.7.0"`; `:40` — `license = "MIT OR Apache-2.0"` (Phase 9 D-11
  executed).
- `git tag --sort=-v:refname` — `v0.7.1`, `v0.7.0`, `v0.5.1`, `v0.5.0`, `v0.4.3`.
- `REQUIREMENTS.md:358` — REL-01 is `[x]`; `:3913` — traceability row reads `Phase 4 | Complete`.

**HARD-01:**
- `ls crates/` — `doc-examples` plus the ten library crates. `paladin-herald` present.
- `.planning/codebase/STRUCTURE.md:51-71,254-255,292-314` — all ten plus `doc-examples` documented;
  **already corrected by Phase 7**, no map fix owed by this phase.
- `.planning/codebase/CONCERNS.md:276` — Phase 9's dated amendment; the live suppression set is ten,
  five of them unmaintained.

### Codebase maps and conventions

- `.planning/codebase/STRUCTURE.md` — the workspace shape D-06's head note restates. Correct as of
  Phase 7.
- `.planning/codebase/CONVENTIONS.md` — naming and module conventions (analysis date 2026-07-30).
- `.planning/codebase/CONCERNS.md:257-284` — the advisory sections, carrying Phase 9's dated
  amendment. D-19 touches the `-0187` reasoning only.
- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — the workspace gate
  (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and the medieval-military
  ubiquitous-language requirement.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **Three complete ledgers** — `.planning/ledgers/milestone-01.md` (697 lines),
  `milestone-02-03.md` (609), `milestone-04-06.md` (456). The fourth is a known quantity: copy the
  head-note structure, the verdict vocabulary, and the in-place amendment convention. **Do not
  reinvent the format.** `milestone-04-06.md` is the most recent and already names this file.
- **REQUIREMENTS.md's existing 86-row run-4 ledger** (`:3121-3317`) — already carries per-`REQ-*`
  verdicts, epic-level notes, and a status key. D-03 upgrades it; it is a starting point, not a blank
  page. Roughly a quarter of the rows are already `Shipped` with citations, and seven more are closed
  by Phase 9.
- **`intel/code-verification.md`'s run-4 block** — 17 verified-shipped claims with tree evidence, the
  13-row supersession table, and both favourable-direction contradictions. This is HARD-02's evidence
  pre-assembled and a large fraction of HARD-01's `Shipped` rows.
- **`.planning/decisions/0001`-`0027`** — twenty-seven ADRs in the target format. **0015 is ADR-0031's
  direct model** (invariant separated from list); **0010 and 0014 are ADR-0030's precedents**; 0008
  and 0009 show how a later phase cites an earlier answer instead of re-deciding it — the move D-10,
  D-20 and D-21 all make.
- **The reconciliation's orphan test** — `rg "mod <name>"` / `mod.rs` inspection / leaf-crate file
  existence. Reusable, cheap, and the highest-fidelity verification procedure in the corpus.
- **`SECURITY-EXCEPTIONS.md` and ADR-0024** — Phase 9's register. D-19 corrects one sentence of the
  reasoning behind one row; nothing else in it moves.

### Established Patterns

- **Precedence is the project's core mechanic** (D-00b), and this phase writes at three levels of it —
  ADRs (top), the ledger, and `.project/` corrections (fifth/sixth). Unlike Phase 7, the
  `.planning/codebase/` map tier needs no correction: Phase 7 fixed `STRUCTURE.md` and Phase 9 fixed
  `CONCERNS.md`.
- **Contested positions get ADRs; code-settled defects get ledger rows** (D-00g) — D-24 allocates six
  ADRs against seven requirements, and HARD-01 correctly gets none.
- **Retain superseded text; amend in place; date every amendment** (D-00c, D-00d).
- **Documents lie about themselves in both directions.** This session reproduced it five times inside
  HARD-01…HARD-07's own text: the 14-row table holds 13 rows (D-05); REL-01 is described as awaiting
  HARD-03 and is complete (D-10); the tree is described as `0.6.0` with latest tag `v0.5.1` and is
  `0.7.0` with `v0.7.1` (D-11); `paladin-ports` is described as `doctest = false` and is not (D-21);
  `ci.yml:225` is cited for an `--exclude` that is not there (D-21). **Re-read every cited `file:line`
  before acting on it.**
- **Later phases move the ground under this one.** Phase 8 closed DEBT-03 and shrank HARD-07; Phase 9
  closed seven of HARD-01's rows and answered half of HARD-06. A planner reading only the ingest
  record will plan a phase roughly a quarter larger than the one that exists.
- **The reconciliation is the most reliable document in the corpus.** `deferred-items.md` D5's count
  of 17 occurrences across 6 files matches the tree exactly. Where it and a PRD disagree, it wins.

### Integration Points

- **`.planning/ledgers/milestone-07-08.md`** — new file, fourth sibling. Phase 13 adds
  `milestone-09-12.md`.
- **`.planning/decisions/0028`…`0033`** — new files; `PROMOTION.md`'s next-free line advances to 0034.
- **`REQUIREMENTS.md`** — §"Milestone 7-8 as-shipped ledger" reduced to a pointer (D-01); HARD-01…07
  checkboxes flipped; traceability rows updated; four hand-off blocks written (D-26).
- **`crates/paladin-content/Cargo.toml:18`** — one line deleted (D-18). The **only** manifest change.
- **`.cargo/audit.toml:26-29`** — one comment corrected (D-19).
- **`Makefile:432-433`** — one flag and one echo deleted (D-21).
- **Roughly eight `.project/` documents** — annotated, never rewritten.
- **Phase 11 / FACADE-02 and FACADE-03(b)** — receive ADR-0031 and ADR-0028's non-goal split.
- **Phase 12 / SUPPLY-03** — receives D-19's PDF answer as an input already delivered.
- **Phase 13 / ORCH-05** — receives ADR-0029's trajectory table.
- **Phase 15** — receives the leaf-to-leaf `cargo tree --no-default-features` check (D-15) and the
  seven-crate doctest posture (D-22), joining the ADR-0015 allowlist check already queued there.

</code_context>

<specifics>
## Specific Ideas

**Seven findings surfaced during this session that neither the ingest record nor Phases 7-9 contain.**
Each was read from the tree on 2026-08-08. Treat them as verified starting points, not hypotheses.

1. **HARD-07 has already been half-executed by Phase 8, and the record does not know.**
   `crates/paladin-ports/Cargo.toml` no longer carries a `[lib]` section at all —
   `git log --oneline -- crates/paladin-ports/Cargo.toml` shows `2bffe22 feat(08-03): re-enable
   paladin-ports doctests`. `ci.yml:238` runs a bare `cargo test --workspace --doc`. DEBT-03 and the
   "unwritten Task 7.0" deferred since run 3 are **discharged**. The whole surviving residue is
   `Makefile:433`'s `--exclude paladin-ports` inside `release-check`, whose adjacent echo still gives
   a reason ("doctests reference root crate not yet published") that has been false since the crates
   published at `0.1.0`. A planner sizing HARD-07 from REQUIREMENTS.md will plan a doctest re-enable
   that already happened.

2. **The `cargo doc` bar is not an open question — CI already enforces the stricter of the two.**
   `.github/workflows/ci.yml:58` runs
   `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`
   in the required `lint` job. HARD-07 presents M7's zero-warning bar and M8's warnings-acceptable bar
   as an unsettled contest; the tree settled it in M7's favour and nobody wrote it down. ADR-0033 is
   ratification, not adjudication.

3. **`pdf = []` gates nothing in either direction, and `news-api = []` proves that is the anomaly, not
   the pattern.** `grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/` returns zero, and
   `adapters/document/mod.rs` declares `pub mod pdf_extractor;` unconditionally — while
   `adapters/input/mod.rs:5` shows `#[cfg(feature = "news-api")]` gating a dependency-free module
   correctly. So the crate does use empty features legitimately; `pdf` is the single inert one. PDF
   extraction is supported unconditionally, and the facade's five-of-six list is a non-issue.

4. **Three optional dependencies in `paladin-content` are consumed by nothing.** `scraper`, `rss` and
   `tiktoken-rs` are declared optional at `Cargo.toml:42-44` behind `web-scraping`, `rss` and
   `tiktoken` — and `grep -rn "use scraper\|use rss\|tiktoken_rs\|scraper::\|rss::"
   crates/paladin-content/src/` returns **zero matches**. Three features enable three dependencies
   that no code in the crate uses. This is the mirror image of HARD-06's defect and it is not in
   HARD-06's scope; it belongs in the deferred list, but a reader of ADR-0032 will ask about it, so
   the ADR should note it exists and point at where it went.

5. **HARD-01's own row count is wrong by one.** The `Superseded by shipped outcome` table at
   `intel/code-verification.md:365-381` holds **13** data rows; HARD-01 and `REQUIREMENTS.md:3136`
   both say fourteen. Small, but this is the class of error the whole phase exists to retire, and it
   sits inside the requirement that retires it.

6. **HARD-03's forward coupling has already fired.** REL-01 is `[x]` at `REQUIREMENTS.md:358` with a
   `Complete` traceability row at `:3913`, converged by Phase 4 on `0.7.0` via ADR-0008. HARD-03's
   text still describes REL-01 as downstream work awaiting this answer. And its current-state figures
   are two releases behind: the tree is `0.7.0` (`Cargo.toml:34`) with tags `v0.7.0` and `v0.7.1`
   present, not `0.6.0` with `v0.5.1` latest.

7. **The `paladin-llm` edge HARD-05 calls a violation is narrower than the record implies.** It is
   non-default (`llm = ["dep:paladin-llm"]`), the facade opts into it explicitly
   (`Cargo.toml:275`), and it gates exactly one `cfg`-guarded module. The default build of
   `paladin-content` has no leaf-to-leaf edge at all. That is what makes D-15's restatement a
   description of the tree rather than an amnesty for it — and it is the fact ADR-0031 should lead
   with.

**Scale note for the planner:** 86 requirements across 11 epics plus 5 cross-milestone entries,
against Phase 7's 115 across 13 and Phase 5's 118 across 14 — so this ledger is the smallest of the
four, and its rows are the best-evidenced. Run 4 is the only run whose corpus contains a document
that audits itself against the tree. Budget the savings into the three questions that carry real
consequence: HARD-05 (gates Phase 11's D2/D3/D4), HARD-06 (touches a published crate's feature
surface), and HARD-02 (whose record stops three completed Epics being re-planned as outstanding).
**Do not size this phase from the ROADMAP's seven-criterion summary** — findings 1, 2 and 6 shrink
three of the seven requirements to citation work, and finding 5 changes the ledger's own arithmetic.

</specifics>

<deferred>
## Deferred Ideas

- **`scraper`, `rss` and `tiktoken-rs`: three optional dependencies in `paladin-content` that no code
  in the crate consumes** — specifics 4. A real manifest defect and the mirror image of HARD-06's,
  but removing a dependency is a build-surface change with `.rs` implications if a consumer is later
  found. Candidate for Phase 11's facade residue work or a dependency-hygiene item in Phase 15.
- **The seven crates still setting `[lib] doctest = false`** — recorded by D-22, owned by Phase 15.
  Deciding the posture is a coverage-and-CI-gates question, not a ground-truth one.
- **Removing the `paladin-content → paladin-llm` edge entirely** — the option D-15 declined. If a
  human overturns D-15, this becomes architecture work (invert through a port, or drop the
  capability) with its own phase. Recorded here so a later reader sees it was considered and closed
  rather than forgotten.
- **A CI dependency-allowlist check built on `cargo tree`** — Phase 15, from ADR-0015. D-15 adds a
  second clause to it (`--no-default-features` leaf-to-leaf edges). Carried forward unresolved from
  Phases 7, 8 and 9.
- **The eight deprecated GitHub Action references** — Phase 15 / PIPE-04 owns the sweep. Untouched
  here, as in Phases 8 and 9.
- **Stray root artefacts** — `api_surface_current.txt` (881 KB), `final-api.txt` (198 KB), `flat`,
  `lcov.info`. Two of them are named in the run-4 supersession table as the reason no root
  `STABLE_API.md` exists, so the ledger will cite them; cleaning them up remains housekeeping.
  Carried forward from Phase 9.
- **Replacing `dotenv` with `dotenvy`** and the other four live unmaintained advisories' upstream
  paths — carried forward from Phase 9. `SECURITY-EXCEPTIONS.md` records the conditions.
- **A `SECURITY.md` for GitHub's advisory UI** — carried forward from Phase 9. Candidate for
  Phase 16's documentation work.
- **Retiring or replacing `src/main.rs`, the legacy content-aggregator entry point** — carried forward
  unresolved from Phases 7, 8 and 9.
- **Nyquist validation for Phases 1-4** — carried forward unresolved from Phases 5, 7, 8 and 9.
  Owner: `/gsd-validate-phase 1`…`4`.
- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phases 1, 5, 7, 8 and 9. Belongs with Phase 16's documentation work. **Six phases
  is enough; Phase 16 should answer it or record it declined.**

</deferred>

---

*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Context gathered: 2026-08-08*
