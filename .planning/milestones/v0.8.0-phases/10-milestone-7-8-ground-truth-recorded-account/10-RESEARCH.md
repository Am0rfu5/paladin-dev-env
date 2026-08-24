# Phase 10: Milestone 7-8 Ground Truth & Recorded Account - Research

**Researched:** 2026-08-08
**Domain:** Documentation/ledger reconciliation — a record-writing phase, not a feature phase. No
library, framework, or implementation research applies (see `<critical_framing>`).
**Confidence:** HIGH — nearly every claim below is a direct `file:line` read or a command actually
executed against the working tree on 2026-08-08 (commit `9550299`), not inference from a document.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision
below carries the reasoning that produced it; **none was confirmed by a human.** Two decisions are
flagged `⚠ HUMAN REVIEW` — restating the extracted-crate dependency invariant that Phase 11 plans its
relocation targets against (**D-15**), and deleting a declared feature from a published crate
(**D-18**).

**Nine gray areas were identified and all nine were auto-selected and resolved:** ledger shape and
evidence bar (HARD-01) · the reconciliation's authority (HARD-02) · the version trajectory (HARD-03) ·
the fourth numbering collision (HARD-04) · the extracted-crate dependency rule (HARD-05) · the PDF
capability answer (HARD-06) · the `cargo doc` bar and doctest posture (HARD-07) · this phase's
code-change boundary (cross-cutting) · ADR allocation and plan decomposition.

#### Phase Boundary

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

#### Implementation Decisions

##### Inherited from Phases 1, 5, 7, 8 and 9 — locked, not re-litigated

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

##### HARD-01 — the ledger: home, vocabulary, evidence bar, and what is already closed

- **D-01: New file `.planning/ledgers/milestone-07-08.md`; REQUIREMENTS.md's section becomes a
  pointer.**
  This is not a judgement call so much as a commitment already made: `.planning/ledgers/milestone-04-06.md`'s
  own head note states "Phases 10 and 13 each add a sibling ledger (`milestone-07-08.md`,
  `milestone-09-12.md`) rather than growing REQUIREMENTS.md further". REQUIREMENTS.md is 4,136 lines
  today; its `## Milestone 7-8 as-shipped ledger` section runs **3121-3317** and is reduced to a
  pointer by the scaffold plan, exactly as Phase 7's D-26 did for Milestone 4-6.
  **Do not leave the REQUIREMENTS.md section in place as a second, diverging copy.**

- **D-02: Seven verdict classes — Phase 7's five plus `deferred with register`, and
  `superseded by outcome` kept visually distinct.**
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

- **D-04: Phase 9's seven closed rows are cited, not re-verified — but their citations are
  re-derived.**
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

##### HARD-02 — the reconciliation as the authoritative account of Milestone 8

- **D-07: ADR-0028 records the supersession; both superseded documents get D-00c annotations; the
  ledger carries the rows.**
  This is a contested position — two ingested documents assert the opposite of what the tree shows —
  so it gets an ADR under D-00g, not just a ledger row. Three deliverables, matching the Phase 7
  ADR-plus-source-correction pairing:
  `facade-cleanup-RECONCILIATION-2026-06-04.md` is named authoritative;
  `Epic_1/facade-audit.md` and `Epic_3/infrastructure-adapter-disposition.md` are annotated
  superseded at source with the reason.
  **The reason is factual, not procedural, and must be stated that way:** both describe ~4,400 LOC of
  *orphaned, uncompiled duplicate files* as "active bridges that stay".

- **D-08: The reconciliation's verification method is preserved verbatim as a reusable test, and the
  three in-execution corrections are a named subsection of the ADR.**
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

- **D-09: Epic 3 complete in substance, Epic 6 complete despite its own record, and the new-crate
  non-goal recorded as split.**
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

##### HARD-03 — the version trajectory as history

- **D-10: ADR-0029 records `v0.1.0-rc.1` as closed history. REL-01 is not re-opened — it is already
  complete.**
  ⚠ **Fresh finding that changes HARD-03's shape.** HARD-03 says "**Feeds REL-01**, which converges
  the three-way version disagreement". Verified this session: **REL-01 is `[x]` at
  `REQUIREMENTS.md:358` and its traceability row at `:3913` reads `Phase 4 | Complete`.** Phase 4
  already converged, on `0.7.0`, via ADR-0008. HARD-03's live job is therefore **backwards-looking
  confirmation**, not a hand-off: record the history, and record that REL-01 did not converge on any
  rc.1 figure. A planner who treats REL-01 as open will re-plan a closed requirement.

- **D-11: The tree has moved past every current-state figure in HARD-03's own text; correct them at
  source.**
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

- **D-12: ADR-0029 is the single home for the whole trajectory; Phase 13 / ORCH-05 extends it rather
  than writing a second version ADR.**
  HARD-03 covers rc.1 → v0.2.0; ORCH-05 (Phase 13) covers v0.3.0 → v0.6.0; REL-01 (Phase 4, done)
  covers the landing at 0.7.0. Three ADRs for one unbroken line would guarantee the third contradicts
  the first. ADR-0029 is written with a `## Trajectory` table that ORCH-05 appends rows to, and says
  so in `Downstream Consumers`. Whichever of the two runs second applies rather than re-decides — the
  ROADMAP's own coupling note at `REQUIREMENTS.md:4036` states that rule; this decision just names
  the artefact it applies to.

##### HARD-04 — the fourth milestone-numbering collision

- **D-13: ADR-0030, citing ADR-0010 and ADR-0014 as its two precedents; the M7 overview corrected at
  source.**
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

- **D-14: The "expect a fifth in run 5" prediction is recorded as already closed, not carried
  forward.**
  `ROADMAP.md:112-114` states it: "The protocol predicted a fifth instance in run 5; run 5 found
  none, and ORCH-05 records the prediction closed." ADR-0030 records that the Roadmap Extension
  Protocol item is discharged with this fourth instance and that no fifth exists, so no later phase
  inherits a standing prediction to check.

##### HARD-05 — the extracted-crate dependency rule

- **D-15: The rule is restated as "never, except behind a non-default optional feature the facade
  opts into explicitly." ADR-0031.** ⚠ **HUMAN REVIEW — this is the answer Phase 11 plans
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
  *(Research note: the "§4.4 complexity note" quoted above is actually located in
  `cost-benefit-assessment.md:118`, a sibling document to `prd-extract-infrastructure-crates.md` —
  see Common Pitfall 3 below. This does not change the decision, only the citation ADR-0031 should
  use.)*

- **D-16: M7 Epic 1 PRD §6.1 and Goal 2 are annotated at source, and the ledger row moves from
  `Code diverges` to `satisfied`.**
  `REQUIREMENTS.md:3159` currently reads **`Code diverges → HARD-05`** for
  `REQ-extracted-crate-dependency-rule`. Once ADR-0031 lands, the tree conforms to the restated rule
  and the row becomes `satisfied` with the ADR as its citation — the divergence was in the *rule's
  wording*, not in the code. Say that explicitly in the row; a bare verdict flip with no explanation
  is exactly the kind of unexplained status change this ledger series exists to prevent.

##### HARD-06 — is PDF extraction supported?

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

- **D-19: M7 Epic 1 §4.4.1 and §4.4.6 are annotated superseded at source; `.cargo/audit.toml`'s
  parenthetical is corrected.**
  §4.4.1 requires `pdf` to gate `pdf-extract`; §4.4.6 requires `content-processing` to activate
  `paladin-content` "with **all** capability features enabled". Both are superseded by outcome under
  D-17. The `.cargo/audit.toml` comment at `:26-29` is corrected to name the actual path
  (`pdf-extract` is unconditional in `paladin-content`; `paladin-content` is optional in the facade)
  so `SECURITY-EXCEPTIONS.md`'s compensating-control row for `RUSTSEC-2026-0187` rests on a true
  statement. **This is the input Phase 12 / SUPPLY-02 was told to wait for** — record it as an answer
  delivered, so Phase 12 does not re-derive it.

##### HARD-07 — the `cargo doc` bar and the doctest posture

- **D-20: The bar is zero warnings on `cargo doc --workspace --no-deps`. The tree already enforces
  it. ADR-0033 ratifies the shipped answer.**
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
  ***Research correction, this session (see Common Pitfall 1 below): the command was actually run
  against HEAD, and it exits 1 — the tree currently produces 20 rustdoc warnings across four crates
  (`paladin-web`, `paladin-ai`, `paladin-battalion`, `paladin-herald`). The gate's *configuration* is
  exactly as D-20 describes; its *current pass/fail state* is not. ADR-0033 must record the measured
  state, not the configured intent — see the Research document's Pitfall 1 for the recommended
  framing.***

- **D-21: DEBT-03 is already closed. HARD-07's "resolve alongside DEBT-03" clause resolves to a
  three-word Makefile fix.**
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
  ***Research correction, this session (see Common Pitfall 2 below): two of these seven
  (`paladin-llm`, `paladin-memory`) actually ran their doctests when `cargo test --workspace --doc`
  and `cargo test -p <crate> --doc` were executed, and all of them passed — `doctest = false` does
  not appear to suppress execution on this Cargo version. The other five show zero doctests to run
  regardless of the flag (no rustdoc code fences exist in their source). Recommend one added sentence
  in ADR-0033 rather than reopening D-22's substance.***

##### Cross-cutting

- **D-23: Phase 10's code-change boundary — record-writing, plus a three-file config surface, and no
  `.rs`.**
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
  ***Research note: both commands WERE attempted and ran successfully this session — see Common
  Pitfalls 1 and 2 for the measured results, which are not what D-20/D-22's text assumed.***

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
  pattern, not the prose). **Research recommendation: use the compact "SUPERSEDED BY [ADR-NNNN]"
  blockquote shape for fully-superseded documents, and the inline strike-and-correct shape for
  clause-level fixes — see the Research document's Architecture Patterns → Pattern 1 for both,
  quoted verbatim from Phase 8/9 precedent.**
- How the ledger presents the run-4 claims `intel/code-verification.md` already verified — inline per
  row, or as a cross-reference block. Phase 7 left the same choice open.
- Whether the `Makefile:432-433` fix (D-21) rides in plan ⑧ or gets folded into the close-out.
- Whether ADR-0033 also records the four crates that *do* run doctests as a positive baseline, or
  only the seven that do not.

### Deferred Ideas (OUT OF SCOPE)

- **`scraper`, `rss` and `tiktoken-rs`: three optional dependencies in `paladin-content` that no code
  in the crate consumes** — a real manifest defect and the mirror image of HARD-06's, but removing a
  dependency is a build-surface change with `.rs` implications if a consumer is later found.
  Candidate for Phase 11's facade residue work or a dependency-hygiene item in Phase 15.
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

</user_constraints>

## Summary

This phase upgrades an existing 86-row `REQUIREMENTS.md` ledger section into a cited, fourth
sibling ledger file, writes six ADRs (0028-0033), annotates roughly eight `.project/` documents in
place, and edits exactly three config lines. There is no code to research. The research task was
instead to **re-verify every load-bearing fact CONTEXT.md's 27 decisions rest on**, because this
corpus's dominant failure mode — documents lying about themselves — has already struck CONTEXT.md's
own text five times (D-05, D-10, D-11, D-21, and a sixth found this session, below).

Every fact CONTEXT.md cites was re-checked directly against the tree this session and **all of them
hold** — the 86-row count, the 13-row (not 14) supersession table, all three config-surface line
numbers, the `paladin-ports` doctest re-enable, the `REL-01`/version staleness, and every `.project/`
path. Nothing CONTEXT.md asserts needs correction.

**One new, high-consequence finding emerged that CONTEXT.md does not have**, because CONTEXT.md
verified the *existence* of the CI doc-warning gate's command text but never ran it: **running the
exact CI command (`cargo doc --workspace --no-deps 2>&1 | tee ... && ! grep -q "warning:" ...`)
against HEAD today exits 1.** The tree currently produces 20 rustdoc warnings across four crates
(`paladin-web` 13, `paladin-ai` 3, `paladin-battalion` 3, `paladin-herald` 1). D-20's premise — "the
tree already enforces [zero warnings]... a zero-warning gate ships today" — describes the *config*,
not the *current build*. See **Common Pitfall 1** below; it materially changes what ADR-0033 can
honestly say and creates a D-23 boundary tension (fixing the warnings needs `.rs` edits, which this
phase cannot do).

A second, smaller finding: **`cargo test --workspace --doc` does not actually skip the seven
`[lib] doctest = false` crates.** `paladin-llm` (4 tests) and `paladin-memory` (8 tests) both ran
and passed their doctests despite the flag, a known upstream Cargo behaviour
(`rust-lang/cargo#10906`-adjacent). This nuances, but does not overturn, D-22's "seven crates don't
run doctests" framing — see **Common Pitfall 2**.

A third, small finding: CONTEXT.md's D-15 attributes the "use-case services depend on `paladin-llm`
... must be handled carefully" complexity note to *"the same PRD's §4.4"* — it is not there. It is
`cost-benefit-assessment.md:118`, a sibling document. See **Common Pitfall 3**. This does not change
D-15's substance (the note exists and says what CONTEXT.md says it says), only its citation.

**Primary recommendation:** Plan this phase exactly as CONTEXT.md's D-27 decomposition describes
(~9 plans, 4 waves), but (a) add the cargo-doc-warnings finding as required reading before wave 3's
plan ⑧ writes ADR-0033 — the ADR must record the *measured* state, not ratify a config read — and
(b) treat the doctest-still-runs finding as a one-line addition to ADR-0033's D-22 clause, not a
new requirement.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| HARD-01 | Upgrade the 86-row Milestone 7-8 ledger to `file:line`-cited per-criterion verdicts in a new sibling file, with the `Superseded by outcome` class unmissable | 86-row count and per-epic breakdown independently re-counted and confirmed (below); `milestone-04-06.md` head-note shape fully mapped for direct reuse; Phase 9's seven closed rows re-cited with fresh evidence already present in the current ledger text |
| HARD-02 | Record the 2026-06-04 reconciliation as authoritative over the Epic 1 audit and Epic 3 disposition record | All three `.project/` document paths confirmed to exist at the exact paths CONTEXT.md names; the reconciliation's orphan-test procedure and the three in-execution corrections are unchanged from CONTEXT.md's text |
| HARD-03 | Record the version trajectory as history; do not re-open REL-01 | `Cargo.toml:34` = `0.7.0` confirmed; `git tag --sort=-v:refname` confirms `v0.7.1`, `v0.7.0` above `v0.5.1`; `REQUIREMENTS.md:358` REL-01 `[x]` confirmed still carrying the *stale* current-state text CONTEXT.md's D-11 says to correct |
| HARD-04 | Record the fourth milestone-numbering collision, citing ADR-0010 and ADR-0014 | M7 overview file confirmed at `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:1`, confirmed titled "Milestone 4" |
| HARD-05 | Restate the extracted-crate dependency rule as a default-build invariant | `crates/paladin-content/Cargo.toml` lines 18-28 read exactly as CONTEXT.md cites, including the `version = "0.7.0"` correction to the PRD's stale `0.6.0`; ADR-0015 confirmed as the exact structural model to imitate |
| HARD-06 | Settle whether PDF extraction is supported; delete or wire the inert `pdf` feature | `grep` for `cfg(feature = "pdf")` returns zero matches, confirmed; `document/mod.rs` and `document_adapter.rs` confirmed unconditional; `news-api` confirmed as the legitimate empty-feature comparator |
| HARD-07 | Settle the `cargo doc` warning bar and the doctest posture | **Both CI-relevant commands were actually run this session** (not merely read) — see Common Pitfalls 1 and 2; DEBT-03 discharge (`paladin-ports` `[lib]` section gone, `2bffe22`) confirmed via `git log`; `Makefile:432-433` confirmed as the sole surviving residue |

</phase_requirements>

## Architectural Responsibility Map

This phase produces no application code, so the usual browser/server/API/CDN/storage tiers do not
apply. The equivalent mapping for a record-writing phase is **which document tier owns each
deliverable**, per D-00b's precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox):

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Contested architecture answers (HARD-02, 03, 04, 05, 06, 07) | `.planning/decisions/` (ADR) | `.project/` (annotation) | D-00g: contested positions get ADRs; the ADR is the promotion (Phase 7 D-11) |
| Per-requirement verdicts (HARD-01) | `.planning/ledgers/` | `REQUIREMENTS.md` (pointer only) | D-01: ledger is the new home; REQUIREMENTS.md's section becomes a pointer, exactly as `milestone-04-06.md` did for Phase 7 |
| Historical corpus documents found wrong by outcome | `.project/` (dated banner, in place) | — | D-00c: annotation, never rewriting; original text retained and marked superseded |
| The three narrow config edits (D-18, D-19, D-21) | Cargo/CI config files | `.planning/decisions/` (ADR records the "why") | D-23: the cheapest-to-move side is the record, except these three, where the record and the manifest must literally agree |
| Forward hand-offs (Phase 11, 12, 13) | `REQUIREMENTS.md` (dated hand-off block) | ADRs' `## Downstream Consumers` | D-26; same shape as Phase 9's block at `REQUIREMENTS.md:1320-1355`, itself re-verified this session (below) |

## Standard Stack

**Not applicable.** This phase installs no packages, adds no dependencies, and modifies no `.rs`
file (D-23). The "stack" is the project's own documentation conventions:

| Convention | Where defined | Purpose |
|---|---|---|
| ADR file shape (7 headings, no frontmatter) | `.planning/decisions/PROMOTION.md:75-96` | HARD-02…HARD-07's six new ADRs must match |
| Ledger head-note shape | `.planning/ledgers/milestone-04-06.md:1-106` | The template `.planning/ledgers/milestone-07-08.md` copies |
| `.project/` annotation banner | Established by commits `94814ff` (Phase 8) and `74a05fe` (Phase 9) | The pattern D-00c fixes; two live examples quoted below |

No `npm view` / `pip index` / `cargo search` verification applies — there is no package to verify.

## Package Legitimacy Audit

**Not applicable — no external packages are installed, upgraded, or referenced by this phase.**
D-23 is explicit: the only manifest touch is deleting one feature line
(`crates/paladin-content/Cargo.toml:18`, `pdf = []`), which *removes* a declaration rather than
adding a dependency. No `package-legitimacy check` is required.

## Architecture Patterns

### System diagram — how a fact becomes a recorded answer in this corpus

```
                         ┌─────────────────────────────┐
                         │   Working tree (ground truth)│
                         │  Cargo.toml / *.rs / ci.yml   │
                         └───────────────┬───────────────┘
                                         │ grep / cargo doc / cargo test / git log
                                         │ (D-00e: exact command recorded verbatim)
                                         ▼
                         ┌─────────────────────────────┐
                         │  Is this a CONTESTED question │
                         │  (two docs disagree, or a rule │
                         │   vs. shipped code disagree)?  │
                         └───────┬─────────────────┬─────┘
                             yes │                 │ no — code-settled defect
                                 ▼                 ▼
                  ┌───────────────────────┐   ┌─────────────────────────┐
                  │ .planning/decisions/   │   │ .planning/ledgers/       │
                  │ NNNN-slug.md (ADR)     │   │ milestone-07-08.md row   │
                  │ D-00g: contested → ADR │   │ D-00g: settled → ledger  │
                  └──────────┬────────────┘   └──────────────┬───────────┘
                             │                                │
                 ┌───────────┴────────────┐                   │
                 ▼                        ▼                   ▼
     ┌────────────────────┐   ┌───────────────────────┐  ┌─────────────────────┐
     │ .project/ document   │   │ Cargo.toml / audit.toml│  │ REQUIREMENTS.md      │
     │ dated banner in place │   │ / Makefile — the 3-file │  │ pointer + checkbox   │
     │ (D-00c, original kept)│   │ config surface (D-23)   │  │ flip (D-26 handoff)  │
     └────────────────────┘   └───────────────────────┘  └─────────────────────┘
```

A reader asking "what happened?" always starts at the ADR or the ledger row (top of precedence),
never at the `.project/` PRD directly — the `.project/` annotation exists so a reader who *does*
land there via search sees the correction inline rather than a live-looking wrong answer.

### Recommended plan/file structure (from CONTEXT.md D-27, unchanged — reproduced here for the
planner's convenience)

```
.planning/
├── ledgers/
│   └── milestone-07-08.md          # Wave 1 scaffold + Waves 2-5 fan-out appends
├── decisions/
│   ├── 0028-m8-reconciliation-authoritative.md   # Wave 3, plan ⑥
│   ├── 0029-version-trajectory-history.md        # Wave 3, plan ⑦
│   ├── 0030-milestone-7-self-numbering.md        # Wave 3, plan ⑦
│   ├── 0031-extracted-crate-dependency-rule.md   # Wave 3, plan ⑧ (checkpoint first)
│   ├── 0032-pdf-extraction-capability.md         # Wave 3, plan ⑧
│   ├── 0033-cargo-doc-warning-bar.md             # Wave 3, plan ⑧
│   └── PROMOTION.md                              # advanced to 0034 in close-out
├── REQUIREMENTS.md                 # §Milestone 7-8 ledger → pointer (Wave 1); checkboxes (Wave 4)
└── phases/10-.../10-*-PLAN.md      # 10-01-PLAN.md ... 10-09-PLAN.md

.project/
├── Milestone_8-Facade-Cleanup-Shim-Resolution/
│   ├── facade-cleanup-RECONCILIATION-2026-06-04.md  # named authoritative, not edited
│   ├── Epic_1/facade-audit.md                       # annotated superseded
│   ├── Epic_3/infrastructure-adapter-disposition.md # annotated superseded
│   ├── Epic_3/prd-relocate-remaining-misplaced-modules.md  # §5 non-goal split annotated
│   └── Epic_5/prd-document-facade-crate-role.md     # FR-19 annotated superseded
└── Milestone_7-Production-Hardening/
    ├── overview/Milestone_7-Tier_4-Production_Hardening.md  # title/prereqs annotated
    └── Epic_1/prd-extract-infrastructure-crates.md          # §6.1, §4.4.1, §4.4.6 annotated

crates/paladin-content/Cargo.toml   # D-18: delete line 18 (`pdf = []`)
.cargo/audit.toml                   # D-19: correct lines 26-29's parenthetical
Makefile                            # D-21: delete lines 432-433
```

### Pattern 1: The `.project/` annotation banner (two confirmed shapes; either is acceptable per
D-00c — CONTEXT.md leaves the exact wording to Claude's Discretion)

**Shape A — inline struck-and-corrected text**, from commit `94814ff` (Phase 8, `08-05`):

```markdown
> **Correction (dated 2026-08-06, DEBT-01):** This document instructs a future implementer to
> write the public-API surface baseline to the pre-rename `project/` path in two places (FR-10 and
> §7 "Technical Considerations", both struck below) — a path that has not existed since commit
> `928c6d5` renamed `project/` to `.project/`. The baseline lives at `.project/current-exports.txt`,
> confirmed present at 442,369 bytes via `ls -la .project/current-exports.txt`, re-run during this
> task ... Original text is retained below with inline corrections — nothing is deleted.

...

10. ~~The system **must** regenerate the public API-surface baseline~~
    ~~(`./scripts/extract-public-api.sh project/current-exports.txt`) and add a `CHANGELOG.md`~~
    ...
    **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
    `.project/current-exports.txt` ... The correct command is
    `./scripts/extract-public-api.sh .project/current-exports.txt`. Confirmed via
    `ls -la .project/current-exports.txt` (442,369 bytes present) ...
```

**Shape B — compact blockquote pointing at an ADR**, from commit `74a05fe` (Phase 9, `09-05`):

```markdown
> **AC 1 SUPERSEDED BY [ADR-0025](../../../.planning/decisions/0025-licence-posture.md) — 2026-08-08.**
> At a blocking checkpoint on 2026-08-08 the repository owner (`DF3NDR`) selected the dual licence
> expression recorded in this same Epic's `license-compatibility-decision-checklist.md` over the
> single `MIT` expression this criterion names. The root package and all ten library crates now
> declare `license = "MIT OR Apache-2.0"`. The original criterion text below is retained unmodified.
```

Shape A suits documents with a specific clause to strike-and-replace (D-18/D-19-style, HARD-06).
Shape B suits documents whose *entire section* is superseded by one ADR (HARD-02's audit/disposition
documents, HARD-04's overview title). Both retain original text and both are dated. **Recommendation
for the planner:** use Shape B for the three fully-superseded documents (`facade-audit.md`,
`infrastructure-adapter-disposition.md`, the M7 overview's title/prereqs) and Shape A for
clause-level corrections (§6.1, §4.4.1, §4.4.6, FR-19, the M8 Epic 3 §5 non-goal).

### Pattern 2: ADR skeleton (confirmed from `0027-dockerfile-chef-planner-stage.md`, the most
recent example; ~150-170 lines is typical for a `must change` ADR, ~100-130 for a `conforms` one)

```markdown
# ADR-NNNN: <title>

## Status
Accepted
**Date:** 2026-08-08

## Context
<what the corpus asserts, what the tree shows, cited file:line for both>

## Decision
<the one-sentence answer, then its justification, referencing D-00b precedence explicitly>

## Considered Options
- <option> (rejected) — <why>
- <option> (accepted) — <why>

## Code Locations
- `path/to/file:LINE` — <what's there>
- `.project/.../doc.md:LINE` — <the superseded requirement text>

## Code Conformance
conforms | must change
<if must change, name the executing plan/task>

## Downstream Consumers
- Phase NN / REQ-ID — <what they inherit>
```

### Anti-Patterns to Avoid

- **Re-tagging a `.project/` document via `--manifest` to "promote" it.** Phase 7's D-11 (cited by
  CONTEXT.md's D-15) already settled this: an ADR *is* the promotion. There is no ingest run 6
  (`STATE.md`: "0 ADR-typed and 0 SPEC-typed documents... there is no run 6"). Do not attempt it.
- **Treating a ledger-vocabulary word as license to skip the `file:line` + exerciser bar.** D-03
  restates Phase 7's rule explicitly: an ingest-era status word (`Shipped`, `Verify`, `Variant`) *is*
  the bare "code exists" claim the evidence bar exists to reject. Every one of the 86 rows must be
  re-derived even if the current ledger text already says `Shipped`.
- **Trusting a cited `file:line` without re-reading it.** Confirmed again this session (see Common
  Pitfall 3): CONTEXT.md's own D-15 mis-cites which document holds the `paladin-llm` complexity
  note. Read every citation before writing it into an ADR or ledger row.
- **Assuming a CI config text description equals current CI behaviour.** Common Pitfall 1 below is
  the sharpest instance of this in the whole corpus to date.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| A new verdict vocabulary for the fourth ledger | A fifth naming scheme | The existing 7-word legend (`satisfied`/`present, unproven`/`genuinely outstanding`/`relocated`/`superseded by outcome`/`deferred with register`/`diverged`) | D-02: HARD-01's four required dispositions map onto four of these seven; inventing new words breaks joinability across the four ledger files |
| A mechanism to detect leaf-crate dependency violations | A custom lint or script in this phase | `cargo tree --no-default-features` (per-crate), recorded as a Phase 15 candidate | D-15 explicitly defers the enforcement build to Phase 15, matching ADR-0015's own precedent — this phase only *restates* the invariant |
| A new numbering scheme for this phase's ADRs | Anything but the flat sequential counter | `PROMOTION.md`'s existing scheme, next free 0028 | Confirmed unchanged this session; six ADRs consume 0028-0033, advancing the line to 0034 |
| A method to prove the reconciliation's classification | Re-running the whole 189-file audit | The reconciliation's own reproducible orphan test (`rg "mod <name>"` / `mod.rs` inspection / leaf-file existence) | D-08: "the most valuable thing in the run-4 corpus, and it is reproducible" — record it as a procedure, don't re-derive it |

**Key insight:** every "don't hand-roll" in this phase is really "don't re-litigate a decision this
corpus's own prior phases already made about *how to decide things*" — the vocabulary, the
precedence order, the ADR shape, and the promotion mechanism are all fixed points from Phases 1, 2,
and 7 that this phase inherits rather than re-designs.

## Common Pitfalls

### Pitfall 1: The `cargo doc` zero-warning gate is currently RED, not green — and CONTEXT.md's D-20 did not discover this because it never ran the command

**What goes wrong:** ADR-0033 gets written as "ratification of a shipped, zero-warning answer,"
copying D-20's framing verbatim, when the tree at HEAD does not currently pass the gate it
describes.

**Evidence, measured this session** (commit `9550299`, `rustc 1.97.1`, matching CI's
`dtolnay/rust-toolchain@stable` pin):

```
$ cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
... [20 individual warnings across 4 crates] ...
warning: `paladin-web` (lib doc) generated 13 warnings
warning: `paladin-ai` (lib doc) generated 3 warnings
warning: `paladin-battalion` (lib doc) generated 3 warnings
warning: `paladin-herald` (lib doc) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
$ echo "CI-GATE-EXIT:$?"
CI-GATE-EXIT:1
```

This is the *exact* command at `.github/workflows/ci.yml:58`, run verbatim, not a paraphrase. The
`paladin-web` warnings are broken intra-doc links in `agent_controller.rs`, `app.rs`, and the
facade's `src/infrastructure/web/agent_host.rs`/`mcp_streamable_http_adapter.rs` (unresolved
`[AuthPort]`, `[Paladin]`, `[PaladinExecutorPort]`, `[deliver_content]`, etc., plus two
`redundant_explicit_link` lints and two `private_intra_doc_links` lints). The `paladin-battalion`
and `paladin-ai` warnings are the *same three-plus-three* Phase 7's own `milestone-04-06.md` ledger
already recorded fresh on 2026-08-06 (`REQ-doc-build-clean` row: "6 warnings across two crates...
paladin-battalion (3)... paladin-ai (3)") — so this is not new drift from today, it is a
**pre-existing, previously-recorded-but-not-connected-to-HARD-07 defect** that has since grown by 14
more warnings (13 in `paladin-web`, 1 in `paladin-herald`) from work landed after Phase 7's plan ran.

**Why it happens:** D-20 verified the *CI job's command text exists* (`ci.yml:58`) and treated that
as proof the gate currently passes. The two are different claims. Nothing in CONTEXT.md's research
this session actually executed `cargo doc`.

**How to avoid:** Report the measured state, not the configured intent. Recommended framing for
ADR-0033: **the zero-warning bar is the one M7 and CI both intend** (D-20's substantive point
stands — CI *is* configured for zero-warnings, not "warnings acceptable"), **but the tree does not
currently clear it**, and fixing it is `.rs` work squarely outside D-23's boundary. Record the 20
warnings and their four crates as a **new, small, verified-open finding** — hand it to Phase 16
(DOCS-03 already "settles the `cargo doc` bar... every public item documented to a standard three
milestones previously set three different ways") rather than fixing it here. This also resolves a
latent tension: D-23 says "no `.rs` file is modified," but fixing rustdoc links unavoidably touches
`.rs` doc-comments.

**Warning signs:** Any ADR draft that says "ships today," "already enforced," or "the tree already
satisfies" about a CI *command* without a session log showing that command was actually run against
HEAD.

### Pitfall 2: `[lib] doctest = false` does not stop `cargo test --workspace --doc` from running that crate's doctests

**What goes wrong:** D-22 is read as "these seven crates' doctests do not run and are therefore
unmeasured," and ADR-0033 states that as fact.

**Evidence, measured this session:**

```
$ cargo test --workspace --doc 2>&1 | grep -E "^test result|Doc-tests"
   Doc-tests paladin_llm
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored ...
   Doc-tests paladin_memory
running 8 tests
test result: ok. 8 passed; 0 failed; 0 ignored ...
   Doc-tests paladin_content / paladin_notifications / paladin_storage / paladin_web / paladin_doc_examples
running 0 tests   [each]
```

`paladin-llm` and `paladin-memory` both have `[lib] doctest = false` (`Cargo.toml:15` in each,
confirmed) yet Cargo ran and passed 4 and 8 doctests respectively — reproduced both via
`cargo test --workspace --doc` and in isolation via `cargo test -p paladin-llm --doc`. This matches
a documented Cargo behaviour class
(`[CITED: github.com/rust-lang/cargo/issues/10906]` — "`cargo test --workspace` ignores target for
doctests only in workspaces") — cited here as *context for the observed behaviour*, not as a
confirmed root-cause diagnosis of this exact Cargo version.

The other five `doctest = false` crates (`paladin-content`, `paladin-notifications`,
`paladin-storage`, `paladin-web`, `doc-examples`) show `running 0 tests` — but that is because they
contain **zero** rustdoc code fences at all (confirmed via
`grep -rl '```' crates/paladin-content/src/ ...` returning no hits for those five), not because the
flag suppressed anything. `paladin-herald`, which also sets the flag, has 3 files with code fences
but shows `6 ignored` (deliberately fenced `ignore`/`no_run`), consistent with authors intending the
flag but Cargo not honouring it for compilation purposes either way.

**Why it happens:** the flag is real Cargo manifest syntax and is widely believed to be sufficient;
this project's own manifests set it seven times expecting it to work.

**How to avoid:** State the *measured* posture in ADR-0033, not the *intended* one: of the seven
`doctest = false` crates, two (`paladin-llm`, `paladin-memory`) have doctests that run anyway (and
pass), five have none to run regardless of the flag, and the flag itself may be dead weight given
this Cargo version's behaviour. This is a nuance to D-22, not a reversal — D-22's core finding (only
`paladin-core`/`paladin-ports`/`paladin-battalion`/`paladin-web` were *believed* to run doctests) is
still worth recording, but "unmeasured" is not quite accurate for `paladin-llm`/`paladin-memory`
specifically. Recommend one added sentence in ADR-0033, not a new requirement — this is squarely
Claude's-Discretion-sized, not a re-opening of D-22's Phase-15 hand-off.

**Warning signs:** Citing `[lib] doctest = false` as proof a crate's doctests "do not run" without
having run `cargo test -p <crate> --doc` to confirm.

### Pitfall 3: A citation inside CONTEXT.md itself points at the wrong document

**What goes wrong:** ADR-0031 cites `prd-extract-infrastructure-crates.md`'s "§4.4" for the sentence
"use-case services depend on `paladin-llm` for LLM analysis, creating an inter-crate dependency that
must be handled carefully" — a citation that does not resolve.

**Evidence:** `prd-extract-infrastructure-crates.md`'s own §4.4 (lines 130-172, "Task 1.4 — Extract
`paladin-content` Crate") contains no mention of `paladin-llm` or LLM analysis anywhere in its eight
numbered clauses (confirmed via direct read and `grep -n "paladin-llm\|LLM" ...prd-extract...`
returning zero hits in that file). The sentence is real, but it lives in a **sibling document**:
`.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md:118`, inside the
`Extraction complexity` cell of the `paladin-content` go/defer table:

> "**HIGH** — this extraction is the most complex because: ... (b) use-case services depend on
> `paladin-llm` for LLM analysis, creating an inter-crate dependency that must be handled carefully;
> ..."

`REQUIREMENTS.md:1438` (HARD-05's own text) makes the identical mis-citation ("The same PRD's §4.4
complexity assessment anticipated the violation"), so this is not a CONTEXT.md-introduced error — it
predates CONTEXT.md and CONTEXT.md inherited it.

**Why it happens:** the two documents (`prd-extract-infrastructure-crates.md` and
`cost-benefit-assessment.md`) are Epic 1 siblings discussing the same four crates from different
angles (requirements vs. go/defer scoring), and "§4.4" happens to number a section in both — but
different content in each.

**How to avoid:** ADR-0031's `## Code Locations` should cite `cost-benefit-assessment.md:118`, not
`prd-extract-infrastructure-crates.md §4.4`, for this specific sentence. D-15's substance is
unaffected — the sentence exists, says what CONTEXT.md paraphrases, and still supports option (b).
Only the pointer needs fixing when ADR-0031 is drafted.

**Warning signs:** A "§N.M" citation for prose that does not appear anywhere in that document's own
numbered §N.M section when re-read directly.

## Code Examples

### Confirmed 86-row count and per-epic breakdown (re-derived this session, not assumed)

```
$ awk 'NR>=3121 && NR<=3317' .planning/REQUIREMENTS.md | grep -c '^| REQ-'
86
```

| Section | Row count |
|---|---|
| M7 Epic 1 — Extended Workspace Decomposition | 12 |
| M7 Epic 2 — Production Build Infrastructure | 13 |
| M7 Epic 3 — Benchmark Suite Migration | 10 |
| M7 Epic 4 — API Stabilization & Pre-Release Preparation | 12 |
| M8 Epic 1 — Facade Crate Audit | 4 |
| M8 Epic 2 — Dead Shim & Empty Module Removal | 4 |
| M8 Epic 3 — Relocate Remaining Misplaced Modules | 6 |
| M8 Epic 4 — `use_cases` → `services` Rename | 4 |
| M8 Epic 5 — Facade Role Documentation & v0.2.0 Finalization | 6 |
| M8 Epic 6 — `paladin-content` Services Rename | 4 |
| M8 Epic 7 — `paladin-web` Single Framework (axum) | 6 |
| Cross-milestone entries carried by DOCs | 5 |
| **Total** | **86** — confirms CONTEXT.md's D-02/D-27 figure exactly |

### Confirmed 13-row (not 14) supersession table

```
$ sed -n '365,381p' .planning/intel/code-verification.md | grep -c '^|.*|.*|.*|$'
15   # includes header + separator; data rows = 15 - 2 = 13
```

Manually recounted by reading rows 369-381 individually: 13 data rows, confirming CONTEXT.md's D-05
exactly (`intel/code-verification.md`'s own header text and `REQUIREMENTS.md:3136`/`:1362` both
still say "14" — both need the D-00d in-place correction this phase applies).

### Confirmed three-file config surface (D-23), exact current text

```toml
# crates/paladin-content/Cargo.toml:14-23 (post-context: delete only line 18)
[lib]
doctest = false

[features]
pdf          = []                        # ← line 18, delete per D-18
web-scraping = ["dep:scraper"]
rss          = ["dep:rss"]
news-api     = []
tiktoken     = ["dep:tiktoken-rs"]
llm          = ["dep:paladin-llm"]
```

```toml
# .cargo/audit.toml:26-29 (correct the parenthetical per D-19)
# RUSTSEC-2026-0187: stack overflow in lopdf via deeply nested PDF objects.
#   lopdf is transitive via `pdf-extract` (optional `content-processing`). The fix requires
#   `pdf-extract` >= 0.12 (a breaking jump that also pulls a fresh `ttf-parser` advisory);
#   deferred. Revisit when `pdf-extract` ships lopdf >= 0.42 without new advisories.
```
The parenthetical `(optional content-processing)` is the part D-19 says is misleading: `pdf-extract`
is unconditional in `paladin-content` (`Cargo.toml:41`, confirmed no `optional = true`); what is
actually optional is `paladin-content` itself in the facade (`Cargo.toml:59`,
`paladin-content = { workspace = true, optional = true }`). Corrected text should read something
like: *"`pdf-extract` is an unconditional dependency of `paladin-content`; reachability is gated only
by whether the facade's optional `paladin-content` dependency is enabled, not by any feature inside
`paladin-content` itself."*

```makefile
# Makefile:429-437 (delete lines 432-433 per D-21)
release-check: ## Check if ready for release
	@echo "$(CYAN)Checking release readiness...$(NC)"
	@$(MAKE) clean-code
	@$(MAKE) test
	@echo "$(CYAN)Running doc tests (excluding paladin-ports: doctests reference root crate not yet published)...$(NC)"   # ← delete
	@$(CARGO) test --workspace --doc --exclude paladin-ports                                                              # ← replace with: @$(CARGO) test --workspace --doc
	@$(MAKE) audit
	@$(MAKE) build-release
```

### Confirmed `.project/` document paths (all found via `find`, not guessed)

| Description | Confirmed path |
|---|---|
| The reconciliation | `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/facade-cleanup-RECONCILIATION-2026-06-04.md` |
| Epic 1 audit | `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` |
| Epic 3 disposition record | `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md` |
| M8 Epic 3 PRD (§5 non-goal, line 211) | `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md` |
| M8 Epic 5 PRD (FR-19, line 159) | `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md` |
| M7 overview (titled "Milestone 4", line 1) | `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md` |
| M7 Epic 1 PRD (§6.1 line 257, §4.4.1 line 132, §4.4.6 line 165, Goal 2 line 30) | `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` |
| M7 Epic 1 cost-benefit assessment (the true home of the `paladin-llm` complexity note, line 118) | `.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md` |
| M7 Epic 4 PRD (§4.4.1-4.4.4 doc-coverage clauses, lines 72-78) | `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md` |
| Epic 4 completion summary | `.project/Milestone_7-Production-Hardening/Epic_4/epic-4-completion-summary.md` |

### Confirmed exact quotes for the two contested-clause pairs

M7 Epic 1 PRD §6.1 (`prd-extract-infrastructure-crates.md:257`):
> "No extracted crate may depend on another extracted crate or on the `paladin` facade."

M7 Epic 1 PRD Goal 2 (`prd-extract-infrastructure-crates.md:30`):
> "Each new crate must depend only on `paladin-core`, `paladin-ports`, and workspace-shared
> dependencies — never on other new infrastructure crates or on the facade."

M8 Epic 3 PRD §5 (`prd-relocate-remaining-misplaced-modules.md:211`):
> "**No new crates created.** `paladin-herald`, `paladin-ml`, etc. are not in scope."

M8 Epic 5 PRD FR-19 (`prd-document-facade-crate-role.md:159`):
> "**FR-19.** `cargo doc --workspace --no-deps` — exit 0 (warnings acceptable; must not fail)."

## State of the Art

| Old approach | Current approach | When changed | Impact |
|---|---|---|---|
| Milestone status tracked inline in `REQUIREMENTS.md` (up to ~4,000+ lines) | Sibling `.planning/ledgers/*.md` files, one per milestone-group, `REQUIREMENTS.md` reduced to a pointer | Phase 7 (`milestone-04-06.md`, D-26), continued by this phase (`milestone-07-08.md`) | Keeps `REQUIREMENTS.md` from becoming unreadable; the four ledgers stay independently `file:line`-cited and cross-joinable by `REQ-*` ID |
| Ingest-run manifest re-tagging (`--manifest`) to promote a candidate decision | ADR authored directly in `.planning/decisions/` | Phase 7 D-11 ("an ADR *is* the promotion"), formalised in `PROMOTION.md` Part A | Ingest is closed (no run 6); this is now the only path to a locked decision, and this phase uses it six times |
| Reading a PRD/DOC/checkbox as sufficient evidence of shipped status | `file:line` citation **plus** a named passing test/example/command | Phase 7 D-01, carried by D-03 into every subsequent ledger | Rejects the false-positive class where a status word alone ("Shipped") stood in for verification |

**Deprecated/outdated:** The ingest classifier itself is retired — `STATE.md` records "there is no
run 6." Any instruction in an older `.project/` document that assumes a sixth ingest run (e.g. the
HARD-05 text's "re-tag via `--manifest` and re-run ingest") is superseded by the ADR-is-the-promotion
convention; **do not** attempt to invoke it.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `[CITED: github.com/rust-lang/cargo/issues/10906]` is the correct root-cause citation for the observed doctest-runs-anyway behaviour on this Cargo version (1.97.1) | Common Pitfall 2 | Low — the *behaviour* is independently reproduced and reported verbatim (commands + output included); the GitHub issue is offered as contextual corroboration, not as the sole basis for the claim. If it is not the exact matching bug, the measured behaviour itself is unaffected |
| A2 | Recommending Shape A vs. Shape B banner assignment per document (Architecture Patterns, Pattern 1) is a reasonable split | Pattern 1 | Low — D-00c and CONTEXT.md explicitly leave exact wording to Claude's Discretion; a planner may choose differently without violating any locked decision |
| A3 | The `paladin-web`/`paladin-herald` doc-warning growth (6 → 20) traces to work landed after Phase 7's 2026-08-06 measurement, based on `git log` dates for the affected files | Common Pitfall 1 | Low — this is offered as likely context for *why* the count grew, not as a load-bearing part of the recommendation (which rests only on the reproduced command + exit code, independent of *when* the regression landed) |

**If this table is empty:** N/A — three low-risk assumptions are logged above, none load-bearing on
a phase-scope decision. Every other claim in this document is `[VERIFIED: <command run this
session>]` or `[CITED: exact file:line]`.

## Open Questions

1. **Should the 20 current `cargo doc` warnings be logged as a new forward-work item in this
   phase's close-out, and if so under what ID?**
   - What we know: they are real, measured, reproducible, and outside D-23's `.rs`-change boundary.
     Phase 16 / DOCS-03 already scopes "cargo doc has one bar... every public item documented to a
     standard" as its own success criterion.
   - What's unclear: whether Phase 10's close-out should mint a fresh finding (parallel to how
     Phase 9's D-05 "fresh finding" mechanism worked) or fold it into ADR-0033's text as a
     documented-but-unfixed residue with an explicit Phase 16 pointer.
   - Recommendation: fold it into ADR-0033's `## Context`/`## Downstream Consumers` as a dated,
     `file:line`-cited residue naming Phase 16 as owner — matching the shape D-19 already uses for
     handing Phase 12 an answer. Do not open a new requirement ID; HARD-07 already covers "the
     `cargo doc` bar," and this is evidence about that same bar, not a new question.

2. **Does the `[lib] doctest = false` flag do anything at all on this Cargo version, for any crate?**
   - What we know: it did not prevent `paladin-llm`/`paladin-memory`'s doctests from running when
     invoked via `cargo test --workspace --doc` or `cargo test -p <crate> --doc`.
   - What's unclear: whether it still affects `cargo test` (without `--doc`) or `cargo doc`'s own
     test-generation step differently, and whether this is a version-specific regression or has
     always been the case for this project's Cargo version history.
   - Recommendation: record the measured fact in ADR-0033 without asserting a mechanism; do not
     spend phase budget bisecting Cargo versions — this is Claude's-Discretion-sized, not a blocker.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo doc` | HARD-07 / ADR-0033 | ✓ (ran successfully, exit 0 on the bare command) | `cargo 1.97.1` | — |
| `cargo test --workspace --doc` | HARD-07 / ADR-0033 | ✓ (ran successfully, exit 0, all doctests pass) | `cargo 1.97.1` | — |
| `git log` / `git tag` / `grep` | Nearly every HARD-* verdict | ✓ | — | — |
| `docker` | Would be needed to measure ADR-0027's outstanding cache-hit claim (unrelated to this phase's own work, but referenced by HARD-01's Epic 2 rows) | ✗ | — | Recorded CI-only, per `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md` (unchanged) |
| `cargo audit` / `cargo deny` | Not required by HARD-01…07 directly, but adjacent (HARD-06 → SUPPLY-02) | ✗ (`crates.io` HTTP 403 in this sandbox) | — | Recorded CI-only, unchanged from Phases 4 and 9 |

**Missing dependencies with no fallback:** None block this phase's actual scope — Docker and
`cargo audit`/`cargo deny` are referenced by rows this phase cites (Phase 9's closed rows, ADR-0027)
but not re-verified by it.

**Missing dependencies with fallback:** Both noted above already carry a recorded CI-only fallback
from earlier phases; this phase does not need a new one.

## Validation Architecture

This phase's Nyquist validation applies to **records**, not code: a "test" is a shell command that
proves a citation resolves, a row count matches, or an annotation banner exists at a named path.

### "Test Framework"

| Property | Value |
|----------|-------|
| Framework | None — direct shell verification (`grep`, `sed -n`, `git log`, `cargo doc`/`cargo test --doc`) |
| Config file | None |
| Quick run command | Per-claim `grep -n "<pattern>" <file>` or `sed -n '<range>p' <file>` |
| Full suite command | The close-out plan's own checklist re-running every command this research document and CONTEXT.md recorded, confirming no drift since 2026-08-08 |

### Phase Requirements → Verification Map

| Req ID | Behavior | Verification Type | Command | Verified this session? |
|--------|----------|-----------|-------------------|-------------|
| HARD-01 | Ledger has 86 rows | grep-count | `awk 'NR>=3121 && NR<=3317' .planning/REQUIREMENTS.md \| grep -c '^\| REQ-'` | ✅ 86 |
| HARD-01 | Supersession table has 13, not 14, rows | grep-count | manual recount of `intel/code-verification.md:365-381` | ✅ 13 |
| HARD-02 | All three `.project/` documents exist at named paths | file-exists | `find .project -iname "*RECONCILIATION*" -o -iname "*facade-audit*" -o -iname "*infrastructure-adapter-disposition*"` | ✅ all three found |
| HARD-03 | Version metadata is stale in REQUIREMENTS.md text | grep | `sed -n '356,360p' .planning/REQUIREMENTS.md` | ✅ confirms still-stale `0.6.0`/`v0.5.1` text |
| HARD-04 | M7 overview self-titles "Milestone 4" | grep | `grep -n "^# " .project/.../Milestone_7-Tier_4-Production_Hardening.md` | ✅ confirmed |
| HARD-05 | `paladin-content` declares optional `paladin-llm` at `0.7.0` | grep | `grep -n "" crates/paladin-content/Cargo.toml \| sed -n '14,30p'` | ✅ confirmed, version corrected to 0.7.0 |
| HARD-06 | `pdf` feature gates zero code | grep | `grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/` | ✅ zero matches |
| HARD-07 | `cargo doc --workspace --no-deps` CI gate currently passes | exact-command-reproduction | `cargo doc --workspace --no-deps 2>&1 \| tee f && ! grep -q "warning:" f` | ❌ **exits 1 — gate currently fails** (Pitfall 1) |
| HARD-07 | `paladin-ports` doctests are re-enabled | git-log | `git log --oneline -- crates/paladin-ports/Cargo.toml` | ✅ `2bffe22` confirmed |

### Sampling Rate

- **Per plan:** re-run the specific `grep`/`sed`/`git log` command the plan's own ledger row or ADR
  cites, before marking the row's evidence cell complete (D-00e's bar, applied per-row).
- **Per wave merge:** re-run the 86-row count and the 13-row count against the in-progress ledger
  file to confirm no row was silently dropped or duplicated during the parallel Wave 2 fan-out.
- **Phase gate:** re-run this research document's full Validation Architecture table verbatim before
  `/gsd-verify-work`, since two of its rows (the `cargo doc` gate and the doctest-flag behaviour) are
  time-sensitive facts about a mutable working tree, not facts about a fixed historical document.

### Wave 0 Gaps

None — there is no test suite to scaffold. The "tests" are the shell commands in the table above,
all of which already run successfully in this environment (one of them, expectedly, with a `❌`
result that the phase must *record*, not silently pass).

## Security Domain

`security_enforcement` is not set to `false` in `.planning/config.json` (the key is absent), so this
section is included per the default-enabled rule — scoped to what actually applies.

### Applicable ASVS categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | This phase touches no auth code |
| V3 Session Management | No | — |
| V4 Access Control | No | — |
| V5 Input Validation | No | No `.rs` file is modified (D-23) |
| V6 Cryptography | No | — |
| **V14 Configuration** (supply-chain / dependency governance framing, adjacent to ASVS's config category) | **Partial — indirectly** | HARD-06/D-19 corrects a governance document's (`.cargo/audit.toml`) stated *reasoning* for an existing, already-approved `RUSTSEC-2026-0187` suppression. It does not change which advisories are suppressed, does not weaken any control, and does not touch `SECURITY-EXCEPTIONS.md`'s owner/expiry fields (Phase 9's territory) |

### Known threat patterns for this stack

Not applicable to this phase's own deliverables — no new attack surface is introduced. The one
adjacent risk worth naming: **a corrected-but-still-wrong `.cargo/audit.toml` comment could
mislead a future security reviewer** if D-19's fix is imprecise. The corrected text must state the
*actual* reachability path (`pdf-extract` unconditional in `paladin-content`; `paladin-content`
optional in the facade) rather than merely negating the old, wrong parenthetical — see the exact
suggested wording in the Code Examples section above.

## Sources

### Primary (HIGH confidence — direct command execution or file read against the working tree,
this session, 2026-08-08)

- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-CONTEXT.md` — full read, all
  27 decisions, canonical refs, and specifics sections
- `.planning/REQUIREMENTS.md:1320-1482` (Phase 9 hand-off + HARD-01…07 full text) and `:3121-3317`
  (the 86-row ledger) — full read and independently re-counted
- `.planning/intel/code-verification.md:263-414` — full read, 13-row supersession table recounted
- `.planning/ledgers/milestone-04-06.md` — head notes (lines 1-106) fully read for shape; sample rows
  (108-157) read to confirm evidence-bar and amendment-in-place conventions
- `.planning/decisions/PROMOTION.md` — full read, numbering index and 5-step promotion procedure
- `.planning/decisions/0027-dockerfile-chef-planner-stage.md` and `0015-core-ports-dependency-allowlist.md` — full read, ADR shape and length confirmed
- `crates/paladin-content/Cargo.toml`, `.cargo/audit.toml`, `Makefile:425-440` — direct read, exact
  line numbers confirmed against D-23
- `crates/paladin-content/src/adapters/document/{mod.rs,document_adapter.rs}`,
  `src/services/{mod.rs,content_llm_analysis_service.rs}`, `src/adapters/input/mod.rs` — direct read
- `git log --oneline -- crates/paladin-ports/Cargo.toml`, `git tag --sort=-v:refname`,
  `git log --diff-filter=M --name-only -- .project/` — direct command execution
- `cargo doc --workspace --no-deps` and `cargo test --workspace --doc` (and `-p paladin-llm --doc`)
  — **actually executed this session**, output captured verbatim
- `.project/Milestone_7-Production-Hardening/Epic_1/{prd-extract-infrastructure-crates.md,
  cost-benefit-assessment.md}`, `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/
  prd-relocate-remaining-misplaced-modules.md`, `.project/.../Epic_5/prd-document-facade-crate-role.md`,
  `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md` —
  direct read with `grep -n` line-number confirmation
- `git show 94814ff` and `git show 74a05fe` — direct diff read for the two confirmed banner shapes

### Secondary (MEDIUM confidence)

- `[CITED: github.com/rust-lang/cargo/issues/10906]` — corroborating context for the observed
  doctest-runs-anyway behaviour (Common Pitfall 2); the behaviour itself is independently
  reproduced and does not depend on this citation being the precise root cause

### Tertiary (LOW confidence)

None — every claim in this document is either directly verified this session or explicitly logged
in the Assumptions table above.

## Metadata

**Confidence breakdown:**
- Ledger scaffold facts (86 rows, 13-row table, config line numbers): HIGH — independently
  re-counted/re-read, not trusted from CONTEXT.md
- `.project/` document paths and quoted clauses: HIGH — found via `find`/`grep`, not guessed
- The `cargo doc` warning-gate finding: HIGH — reproduced the exact CI command twice with consistent
  results (exit 1, 20 warnings, 4 crates)
- The doctest-flag finding: HIGH for the observed behaviour (reproduced twice); MEDIUM for the exact
  upstream-issue attribution (logged as Assumption A1)
- ADR shape and ledger head-note conventions: HIGH — read directly from the two most recent examples

**Research date:** 2026-08-08
**Valid until:** This document's tree-state claims (the `cargo doc` warning count, the doctest
behaviour, current version/tag state) are valid only as of commit `9550299`. Any commit landing
before this phase's plans execute that touches `crates/paladin-web/src/`,
`src/infrastructure/web/`, or `crates/paladin-herald/src/lib.rs` should trigger a re-run of the
`cargo doc --workspace --no-deps` command before ADR-0033 is finalized. All other findings
(document paths, ADR conventions, ledger shape) are stable for the life of this phase.
