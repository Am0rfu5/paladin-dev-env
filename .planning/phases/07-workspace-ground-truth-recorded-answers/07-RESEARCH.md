# Phase 7: Workspace Ground Truth & Recorded Answers - Research

**Researched:** 2026-08-06
**Domain:** Records/documentation triage — no product code. Ledger authoring, ADR authoring,
`.project/` source annotation, codebase-map correction.
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Inherited from Phases 1, 2 and 5 — locked, not re-litigated**

- **D-00a:** ADRs live in `.planning/decisions/`, one file per decision, flat sequential numbering.
  0001-0013 are taken. `PROMOTION.md` records 0014 as the next free number; Phase 7 allocates
  0014-0020 (D-22) and updates that line.
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code.
- **D-00c:** Every ADR carries a `Code Conformance` field valued `conforms` or `must change`, and
  where it is `must change`, names the requirement that executes it.
- **D-00d:** Ledger is a new file per milestone block — `.planning/ledgers/milestone-04-06.md` —
  with REQUIREMENTS.md's `## Milestone 4-6 as-shipped ledger` section reduced to a pointer (D-24).
- **D-00e:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers.
- **D-00f:** Ledgers are amended in place — a later plan's measured result edits the row directly
  with the new verdict, the command or `file:line` that produced it, and the date. Never a separate
  corrections file. Superseded text is retained, not deleted.
- **D-00g:** Source corrections under `.project/` are annotation, not rewriting — a dated correction
  banner at the top naming what was wrong and pointing at the ADR, each defective claim corrected
  inline with the original text kept and marked superseded.
- **D-00h:** ADR file shape is `Status / Context / Decision / Considered Options / Code Locations /
  Code Conformance / Downstream Consumers`, no frontmatter, matching 0001-0013.

**ARCH-01 — ledger depth, evidence bar and vocabulary**

- **D-01:** Phase 5's evidence bar carries over unchanged, with a manifest carve-out: for structural
  requirements (`edition`, feature-flag shapes, dependency lists, `required-features`, workspace
  membership), the manifest line **plus a named CI job or build leg that consumes it** is the
  exercising artefact — e.g. `crate-isolation` and the `feature-flags.yml` workspace matrix.
  Behavioural requirements still need a test, example or command.
- **D-02:** Seven verdict classes: Phase 5's five plus `relocated` and `diverged`. `satisfied` ·
  `present, unproven` · `genuinely outstanding` · `deferred with reason` ·
  `superseded by shipped code` · `relocated` · `diverged`.
- **D-03:** Triage directs effort, not the bar. 22 run-3 claims already verified in
  `intel/code-verification.md` need a citation refresh and an exercising artefact, not
  re-verification. `Verify —` rows get first-pass depth.
- **D-04:** Both systematic path caveats recorded once at the head of the ledger: (a) `src/…` paths
  in run-3 PRDs are internally historical (Milestone 6 moved what Milestone 5 had just placed;
  `src/application/use_cases/` no longer exists); (b) `STABLE_API.md`, `docs/FEATURE_FLAGS.md`,
  `docs/MIGRATION.md`, `docs/CONFIGURATION.md` ship as mdbook chapters. A row whose only divergence
  is caveat (b) is `relocated`, not a gap.
- **D-05:** The workspace-shape correction lands in three places: ledger head note (authoritative),
  REQUIREMENTS.md pointer (D-24), and `.planning/codebase/STRUCTURE.md`'s **Directory Purposes /
  Crate Boundaries prose section** (third in precedence, above `intel/` and every PRD).
- **D-06:** Expect a large `present, unproven` bucket, split by milestone: M4's 20 open items
  corroborated, M5's 17 mostly contradicted, M6's 0 corroborated. Do not apply the "record
  understates the tree" heuristic uniformly.

**ARCH-02 — numbering collision**

- **D-07:** ADR-0014 records the convention (directory/task-list numbering authoritative: 4=Tier 1,
  5=Tier 2, 6=Tier 3); source documents corrected in-repo. Must cite ADR-0010 explicitly.
- **D-08:** Full inline correction on five documents (the two overview titles, the M6 overview
  prerequisite line, and the two PRD cross-references); one-line dated pointer banner on the seven
  byte-equivalent INGEST-CONFLICTS extracts.

**ARCH-03 — four competing variant pairs**

- **D-09 (a) Rust edition** — citation only, no new ADR. Cites ADR-0009. All twelve manifests at
  `edition = "2024"`. Nothing to decide.
- **D-10 (b) dependency allowlists** — ADR-0015 rewrites against reality: enforceable invariant (no
  provider SDK/transport/storage/web framework in `paladin-core`/`paladin-ports`) holds; measured
  current lists (14 / 11) accepted as baseline, not debt; `tokio` in `paladin-core` gets explicit
  justification. Enforcement not built here (Phase 15 candidate).
- **D-11 (c) port value-type ownership** — ADR-0016 ratifies shipped answer (`paladin-core` owns).
  `.project/` FR-7/FR-10 annotated to extend FR-11's carve-out. ADR-0016 *is* the promotion of the
  Epic 1 decision record — not re-tagged via `--manifest`. Input to Phase 8 DEBT-05.
- **D-12 (d) LLM config bridge** — ADR-0017 accepts v2 (`crates/paladin-llm/src/config/bridge.rs`);
  states the circular-dependency concern was real but mis-sited (M6 moved config down, not the
  bridge up). Epic 4 FR-31/FR-32 get a superseded banner.

**ARCH-04 — facade re-export policy**

- **D-13:** No-shim posture stands. ADR-0018 records it as policy. Overview's Epic 2 AC 6, Epic 4
  AC 5, risk register are minority position, annotated superseded. Epic 2 Open Question 4 recorded
  as now confirmed.
- **D-14:** Version consequence: breaking in substance, absorbed as minor bump pre-1.0. Cites
  ADR-0008. **This is ROADMAP criterion 4's one recorded answer. Do not re-derive it.**
- **D-15:** M5→M6 posture flip recorded as history, not contradiction. `REQ-battalion-facade-shim`
  row → `superseded by shipped code` pointing at ADR-0018.
- **D-16:** ADR-0018 is input to Phase 11 FACADE-02 D1, named in `Downstream Consumers`.

**ARCH-05 — five positions shipped code contradicts**

- **D-17:** All five corrected at source (D-00g pattern), all five get `diverged` ledger rows. None
  gets an ADR. Five positions: (1) `vision` gating encryption crates — cross-ref ADR-0011; (2) MCP
  transport flags; (3) `web-server` gating actix-web; (4) `paladin-cli` workspace crate; (5)
  `src/application/use_cases/` orchestration home.
- **D-18:** Four relocated doc deliverables get `relocated` rows and one shared head note; M6 Epic 4
  FR-4.12 re-pointed to `docs/src/api-reference/stable-api.md`.

**ARCH-06 — binary-target architecture**

- **D-19:** ADR-0019 ratifies "Option A extended" — three binary targets, each with a stated
  purpose. `paladin` (`src/main.rs`) is the pre-Paladin content-aggregator entry point
  (`#[structopt(name = "smartcontent-aggregator")]`) — state this plainly, don't invent a tidy
  purpose.
- **D-20:** ADR-0019 must record the coupling: `structopt`'s only consumer in the tree is
  `src/main.rs`, un-gated. The recorded "three-line fix" for CLI dependency isolation is wrong for
  one of its three lines; ADR-0019's answer is its precondition. Named against Phase 8.
- **D-21:** ADR-0019 plus its ledger row satisfy FR9.3's documentation deliverable.
  `Code Conformance: must change`, executor named as Phase 16's mdbook work.

**ARCH-07 — build-time benchmark falsifiability**

- **D-22:** Restate SM-7 per scenario. Do not re-measure. ADR-0020 transcribes the report's own five
  figures, marks two pass / three fail, restates SM-7 as per-scenario. Resolves the report's own
  −6.6%/−5% inconsistency by citing the table and recording the conclusion's figure as a
  transcription error.
- **D-23:** The "Overall verdict: Target achieved" conclusion is judged, not reconciled.
- **D-24:** The recommended mid-tree re-measurement is declined with a recorded reason, not passed
  forward.

**ADR allocation and bookkeeping**

- **D-25:** Seven new ADRs, 0014-0020; `PROMOTION.md` next-free line advances to 0021. Per-ADR
  Conformance verdicts and executors specified (see Decisions section of CONTEXT.md for the full
  table — reproduced in this research's Architecture Patterns section below).
- **D-26:** REQUIREMENTS.md's `## Milestone 4-6 as-shipped ledger` section (line 2830 to the
  `## Milestone 7-8` heading at 3069) reduced to a pointer by the scaffold plan.

**Plan decomposition**

- **D-27 [informational]:** Scaffold → epic fan-out → decisions → source corrections. Suggested
  shape, ~11-12 plans, sized against Phase 5's proven 13 plans for 118 rows. Full suggested plan
  breakdown reproduced in this research's "Ledger mechanics at 115 rows" section below.

### Claude's Discretion

- Exact banner wording and inline-correction markup for every `.project/` edit (pattern fixed by
  D-00g, not the prose).
- Whether ADRs 0015/0016/0017 are three files or one combined ADR (0016 must remain separately
  citable — Phase 8 depends on it by number).
- How the ledger presents the 22 already-verified run-3 claims — inline per row, or a
  cross-reference block.
- Whether `present, unproven` and `diverged` counts are headline figures in the ledger summary.
- Ordering within the epic fan-out (no dependency; M6 is cheapest, good first fan-out plan).
- Whether the `STRUCTURE.md` correction rides in the scaffold plan or gets its own small plan.

### Deferred Ideas (OUT OF SCOPE)

- Consolidating the three `TokenUsage` structs — Phase 8 / DEBT-05. Phase 7 decides which is
  canonical (D-11) and stops there.
- Fixing `api-surface` CI job, `#[deprecated]` annotations, `paladin-ports` doctests, leaked CLI
  deps — Phase 8 / DEBT-01…DEBT-04.
- Building the `cargo tree`-based dependency-allowlist check into CI — Phase 15.
- The user-facing binary-architecture mdbook page — Phase 16.
- Re-measuring the build-time benchmark against a mid-tree baseline — declined with reason in
  ADR-0020 (D-24), not passed forward.
- Retiring or migrating `src/main.rs` — a real question ARCH-06 exposes but does not own. New scope
  for Phase 8 CLI-isolation or its own phase.
- `paladin-herald` shipping `colored`/`comfy-table` unconditionally with no `[features]` section —
  code change outside this phase.
- Nyquist validation for Phases 1-4 — carried forward unresolved.
- Whether ADRs should be published to the mdbook — carried forward, belongs with Phase 16.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ARCH-01 | 115-row ledger, real workspace shape (10 crates + doc-examples + facade), corrected at 3 sites | §"Ledger mechanics at 115 rows", §"The 115 rows themselves", §"The STRUCTURE.md correction site" below |
| ARCH-02 | Milestone/tier numbering collision — ADR-0014 + 5 inline corrections + 7 pointer banners | §".project/ correction sites", ADR-0010 precedent (already read in full) |
| ARCH-03 | Four competing variant pairs — edition (cited), allowlist (ADR-0015), port ownership (ADR-0016), LLM bridge (ADR-0017) | §"ADR conventions", §"Crate-level facts" below; ADR-0008/0009/0011 precedent |
| ARCH-04 | M6 facade re-export policy + version consequence — ADR-0018 | §"ADR conventions"; ADR-0008 precedent (already answers the version half) |
| ARCH-05 | Five positions shipped code contradicts — source corrections, no ADR, `diverged` rows | §".project/ correction sites", §"Crate-level facts" |
| ARCH-06 | Binary-target architecture — ADR-0019, per-binary purpose | §"Crate-level facts", §"Binary targets" below |
| ARCH-07 | Build-benchmark falsifiability — ADR-0020, per-scenario restatement | §"The build-benchmark source" below (verbatim figures transcribed) |

</phase_requirements>

## Summary

This is a records phase: no Rust code changes, no library research. The work is (1) writing a
115-row `file:line`-cited status ledger at `.planning/ledgers/milestone-04-06.md`, (2) authoring
seven ADRs (0014-0020) recording answers to a numbering collision, three of four competing variant
pairs (the fourth was already closed by Phase 4), the M6 facade policy, the binary-target
architecture, and the build-benchmark falsifiability question, (3) annotating eleven `.project/`
documents in place per the established dated-banner pattern, and (4) correcting
`.planning/codebase/STRUCTURE.md`'s crate-boundary prose section, which — verified this session —
lists all ten crates plus `doc-examples` in its top-level directory tree but describes only six of
them in its detailed "Directory Purposes" prose section (lines 255-283; insertion point confirmed
immediately before `**`src/`:**` at line 285).

Two completed analogues (`milestone-01.md`, `milestone-02-03.md`, and Phase 5's thirteen plans) give
the exact document shape to copy. All source documents CONTEXT.md names as correction targets were
confirmed to exist at their stated paths (14/14 checked). One drift was found and must be corrected
before the ledger scaffold cites it: `intel/code-verification.md`'s claim that the `crate-isolation`
CI job lives at `ci.yml:228` is now stale — the job has moved to **line 304** because three jobs
(`security-audit`, `cargo-deny`, `osv-scanner`, `api-surface`) were added earlier in the file since
that citation was written. The `feature-flags.yml:115,118,141` citations, by contrast, are still
exactly correct. **Every exercising-artefact line number cited in this research or copied from
CONTEXT.md must be re-grepped fresh at ledger-authoring time, not trusted from an earlier document.**

**Primary recommendation:** Follow Phase 5's plan shape almost exactly — one scaffold plan (head
notes + all 115 row stubs + `STRUCTURE.md` fix), five epic fan-out plans grouped by milestone (M4
Epics 1-3 = 25 IDs, M5 Epics 1-2 = 20, M5 Epics 3-4 = 20, M5 Epics 5-6 = 16, M6 Epics 1-4 = 34),
three ADR-authoring plans (ADR-0014+corrections; ADR-0015+0016+0017+corrections; ADR-0018+0019+0020),
one or two plans for ARCH-05's five source corrections, and one summary/bookkeeping plan. Total
~11-12 plans, matching D-27's suggested shape and Phase 5's proven scale (13 plans, 118 rows).

## Architectural Responsibility Map

Not applicable in the conventional (browser/server/API/CDN/DB) sense — this is a documentation
phase with no runtime tiers. The equivalent mapping is by **precedence tier** (D-00b), since every
artefact this phase writes sits at a specific point in the project's own precedence order and must
be legible against it:

| Capability | Precedence Tier | Secondary Tier | Rationale |
|------------|-----------------|----------------|-----------|
| Recording which competing PRD/decision wins | ADR (tier 1, top) | — | ADR-0014…0020 are the only artefacts this phase writes that outrank shipped code interpretation disputes |
| Recording the corrected workspace shape | `.planning/codebase/` map (tier 3) | Ledger head note (new sibling artefact, cited alongside) | D-05: `STRUCTURE.md` sits above `intel/` and every PRD; leaving it wrong outranks the ledger that corrects it |
| Recording per-requirement verdicts | New ledger file (not itself a precedence tier — a derived index) | `intel/code-verification.md` (tier 4, the evidence it upgrades) | The ledger doesn't out-rank anything; it's the citable index a reader consults, sourced from tiers 2-4 |
| Correcting historical PRD/DOC claims | `.project/` annotation (tiers 5-6, PRD/DOC) | — | D-00g: correction is inline annotation at the *same* tier, never a rewrite — the corpus's provenance is preserved, not elevated |
| Task-list checkbox corroboration | Checkbox (tier 7, bottom) | — | Not corrected by this phase; only *interpreted* per D-06's per-milestone heuristic |

## Standard Stack

Not applicable — this phase installs no packages, writes no Rust code, and touches no `Cargo.toml`.
See Package Legitimacy Audit below for the explicit N/A disposition.

## Package Legitimacy Audit

**N/A.** This phase writes only Markdown into `.planning/` and `.project/`. No package is installed,
upgraded, or referenced in a manifest. The Package Legitimacy Gate protocol does not apply; no
`gsd-tools query package-legitimacy check` invocation is needed. If a future plan in this phase
somehow needs a tool not already in the environment (unlikely — everything needed is `grep`, `sed`,
`git`, `cargo`), that would be a scope violation of D-00's "no code changes" boundary and should be
flagged rather than silently installed.

## Architecture Patterns

### Document-flow diagram (records phase, not runtime architecture)

```
CONTEXT.md (D-00a…D-27, locked)
      │
      ▼
┌─────────────────────────────────────────────────────────────────┐
│  Plan 1: Ledger scaffold                                        │
│  → .planning/ledgers/milestone-04-06.md                         │
│    (head notes: D-02 vocab, D-04 caveats, D-01 evidence bar,    │
│     D-05 workspace shape; 115 REQ-* row stubs, 13 epic headings)│
│  → REQUIREMENTS.md §"Milestone 4-6 ledger" reduced to pointer   │
│  → .planning/codebase/STRUCTURE.md corrected (5 new crate       │
│     entries inserted before line 285)                           │
└───────────────────────────┬───────────────────────────────────┘
                             │ (row stubs exist; fan-out plans fill them)
        ┌────────────────────┼────────────────────┬──────────────────┐
        ▼                    ▼                    ▼                  ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ Plan 2: M4     │  │ Plan 3: M5    │  │ Plan 4: M5    │  │ Plan 5: M5    │
│ Epics 1-3      │  │ Epics 1-2     │  │ Epics 3-4     │  │ Epics 5-6     │
│ (25 IDs)       │  │ (20 IDs)      │  │ (20 IDs)      │  │ (16 IDs)      │
└───────────────┘  └───────────────┘  └───────────────┘  └───────────────┘
        │                                                          │
        ▼                                                          ▼
┌───────────────┐                                        (feeds Plan 9 summary)
│ Plan 6: M6     │
│ Epics 1-4      │
│ (34 IDs, 0 open)│
└───────────────┘
        │
        │  (ledger rows cite ADRs by number — ADRs must exist first
        │   for those citations to resolve, OR the ledger plans and
        │   the ADR plans can run in parallel if row text is written
        │   to reference "ADR-0015" etc. before the file exists —
        │   Phase 5 precedent allows either; see Plan ordering note)
        ▼
┌─────────────────────────────────────────────────────────────────┐
│  Plans 7-8: ADR authoring                                        │
│  → .planning/decisions/0014-milestone-4-6-tier-numbering.md      │
│    + 5 inline .project/ corrections + 7 pointer banners (ARCH-02)│
│  → 0015-paladin-core-ports-dependency-allowlist.md (ARCH-03b)    │
│  → 0016-port-value-type-ownership.md (ARCH-03c)                  │
│    + .project/…prd-paladin-ports-extraction.md FR-7/FR-10 anno.  │
│  → 0017-llm-config-bridge-location.md (ARCH-03d)                 │
│    + .project/…prd-paladin-llm-extraction.md FR-31/FR-32 anno.   │
│  → 0018-m6-facade-reexport-policy.md (ARCH-04, cites ADR-0008)   │
│  → 0019-binary-target-architecture.md (ARCH-06)                  │
│  → 0020-build-benchmark-per-scenario.md (ARCH-07)                │
└───────────────────────────┬───────────────────────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  Plan 9: ARCH-05's five source corrections                       │
│  → 3 .project/ documents annotated (dependency-matrix PRD,        │
│     M5 overview, M6 Epic 2 PRD) producing 5 diverged + 4 relocated│
│     ledger rows (fed back into the ledger file from Plans 2-6)   │
└───────────────────────────┬───────────────────────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  Plan 10 (or 11): Summary and bookkeeping                        │
│  → PROMOTION.md next-free line → 0021                            │
│  → Ledger summary section (verdict distribution, counted not     │
│     assumed — Phase 5/6's own recomputation precedent)           │
│  → Forward-scope handoffs: Phase 8 DEBT-05 (D-11), Phase 8       │
│     structopt precondition (D-20), Phase 11 FACADE-02 D1 (D-16), │
│     Phase 15 allowlist check (D-10), Phase 16 mdbook page (D-21) │
└─────────────────────────────────────────────────────────────────┘
```

**Plan-ordering note:** ADR-0016 is a hard blocker for planning Phase 8 at all (ROADMAP records
DEBT-05 as blocked on it) — land it early, not last, if plan sequencing allows any flexibility
within this phase. It need not block the *ledger* fan-out plans (which can write "Variant (group
19) — resolved by ADR-0016, see decisions/" as a forward citation and let the ADR plan fill the
citation in later, same as Phase 5's amend-in-place convention permits).

### Recommended plan-to-artefact structure

```
.planning/
├── ledgers/
│   └── milestone-04-06.md          # NEW — 115 REQ-* rows, 13 epic sections
├── decisions/
│   ├── 0014-milestone-4-6-tier-numbering.md     # NEW
│   ├── 0015-core-ports-dependency-allowlist.md  # NEW (or combined per Discretion)
│   ├── 0016-port-value-type-ownership.md        # NEW — Phase 8 DEBT-05 blocker
│   ├── 0017-llm-config-bridge-location.md       # NEW
│   ├── 0018-m6-facade-reexport-policy.md        # NEW
│   ├── 0019-binary-target-architecture.md       # NEW
│   ├── 0020-build-benchmark-per-scenario.md     # NEW
│   └── PROMOTION.md                             # AMENDED — next-free 0014 → 0021
├── codebase/
│   └── STRUCTURE.md                # AMENDED — 5 crate entries inserted (line ~284)
├── REQUIREMENTS.md                 # AMENDED — §"Milestone 4-6 ledger" → pointer
└── phases/07-.../07-RESEARCH.md    # this file

.project/                            # ANNOTATED IN PLACE (D-00g), never rewritten
├── Milestone_4-Refactor-Crates-Features/
│   ├── Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md   # D-08(1)
│   ├── Epic_1/prd-expand-feature-flags.md                            # ARCH-05(1)(2)(3)
│   └── Epic_3/prd-cli-isolation.md                                   # D-20 citation only, no edit needed
├── Milestone_5-Workspace-Decomposition/
│   ├── overview/Milestone_5-Tier_2-Workspace-Decomposition.md        # D-08(2), ARCH-05(4)
│   ├── Epic_1/prd-workspace-init...-core-extraction.md               # ARCH-03(b) citation
│   ├── Epic_1/decisions/battalion-result-upward-dependency-decision.md  # cited, not edited (ADR-0016 promotes it)
│   ├── Epic_2/prd-paladin-ports-extraction.md                        # FR-7/FR-10 annotated (D-11), §1 D-08(5)
│   ├── Epic_4/prd-paladin-llm-extraction.md                          # FR-31/FR-32 annotated (D-12), Non-Goal 2 D-08(4)
│   └── Epic_6/build-benchmarks.md                                    # cited, not edited (ADR-0020 transcribes)
└── Milestone_6-Architectural-Refinements/
    ├── overview/Milestone_6-Tier_3-Architectural-Refinements.md      # D-08(3), Epic2 AC6/Epic4 AC5/risk register D-13
    ├── Epic_2/prd-relocate-orchestration-services.md                 # ARCH-05(5), Non-Goal 7/OQ4 D-13
    └── Epic_4/prd-relocate-circuitbreaker-infra.md                   # FR-4.11/FR-4.12 D-18
```

### Pattern 1: The dated correction banner (D-00g), verbatim example from Phase 5

**What:** Every `.project/` correction this phase makes must follow the exact pattern Phase 5
already executed and proved. Read the whole pattern once from the live file rather than
reconstructing it from the decision text.

**Verified example — `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`:**

Top-of-file banner (lines 1-8):
```markdown
# Release Notes: Milestone 3

> **Correction (dated 2026-08-04, ADR-0010):** This document's Epic 19-24 numbering does not
> match the authoritative plan/epic-definition set, and two further claims are verified absent
> from the tree. See
> [`.planning/decisions/0010-milestone-3-epic-numbering.md`](../../.planning/decisions/0010-milestone-3-epic-numbering.md)
> for the full mapping. Original text is retained below with inline corrections — nothing is
> deleted.
```

Inline per-section correction (line 28-31), retaining and striking through the original, never
deleting it:
```markdown
### ~~Epic 19: Conclave Pattern (Multi-Expert Synthesis)~~ Epic 19: Herald & Domain Type Consolidation
**Corrected numbering (ADR-0010):** this section's content (Conclave) is Milestone 2 **Epic 15**,
not Epic 19. See ADR-0010's mapping table for the full correction.
**Status**: ✅ Complete
```

Inline API-form correction (line 83), same strikethrough-and-append shape:
```markdown
**Corrected API form (ADR-0010):** ~~`council_service.execute(&council, &experts, topic)`~~ and
```

A "Superseded" banner variant for a whole forward-looking section that is now stale (line 360):
```markdown
> **Superseded (dated 2026-08-04, ADR-0010):** The section below is a point-in-time forward-look,
```

**When to use:** Every one of the eleven `.project/` documents this phase touches (D-08's five
inline targets, D-11's FR-7/FR-10, D-12's FR-31/FR-32, D-13's overview AC clauses, D-17's five
positions, D-18's four-document shared head note). The seven byte-equivalent INGEST-CONFLICTS
extracts get only the top-of-file banner, no inline strikethrough (they carry no independent
content — see `INGEST-CONFLICTS.md:656` for the exact byte-equivalence finding and the file list:
`Milestone_5/Epic_2/…Epic_2.md`, `Epic_3/…Battalion_Extraction.md`, `Epic_4/Milestone_5-Epic_4.md`,
`Epic_5/Milestone_5-Epic_5.md`, `Milestone_6/Epic_1/…Decompose_App_Settings.md`,
`Epic_2/…Relocation_Manager_Layer_Orch_services.md`, `Epic_3/…Co-locate_Maneuver_DSL_w_Battalion.md`,
`Epic_4/…Relocate_CircuitBreaker_Infra_Layer.md` — **eight** files listed there, though D-08 states
"seven"; verify the exact count against this list when writing the plan, since D-08's prose and
INGEST-CONFLICTS' file list do not obviously agree at eight vs. seven — flag rather than silently
pick one).

### Pattern 2: The ADR "cites, doesn't re-decide" cross-reference (ADR-0008 ↔ ADR-0009 precedent)

**What:** Two of ARCH-03/04's four sub-questions were already answered by Phase 4, which ran first.
ADR-0009 (edition) and the version half of ADR-0008 both explicitly name the *other* phase's
requirement in their `Downstream Consumers` section and instruct it to cite rather than re-derive.

**When to use:** ADR-0009 already exists and is complete — Phase 7's only remaining scope for
ARCH-03(a) is a ledger-row citation. Similarly, ADR-0008 already answers ARCH-04's major-version-
bump question — ADR-0018 (this phase's own ARCH-04 ADR) must cite ADR-0008 for that half and add
only the facade re-export policy itself, which Phase 4 did not touch.

**Example (from ADR-0009's own `Downstream Consumers`):**
```markdown
## Downstream Consumers

- **Phase 7's ARCH-03(a)** — the Rust edition half of ARCH-03's four competing-variant pairs.
  ARCH-03(a)'s requirement text is amended (plan 04-07, Task 2) to cite this ADR instead of
  re-adjudicating; its remaining scope becomes citing and applying this answer, not deciding it.
```

### Pattern 3: ADR promotion of a `.project/` decision record without `--manifest` re-tagging

**What:** ADR-0016 (port value-type ownership) is not a fresh decision — it is the **promotion** of
an existing `.project/` decision record
(`Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`,
`Status: Approved`, `2026-05-13`) that already carries a `Decision`/`Considered Options`/
`Rejected Options` shape but sits at PRD/DOC precedence because ingest never re-ran to re-tag it.
`PROMOTION.md`'s own five-step procedure (already read in full — see Part A) is the mechanism: take
the next free number, author the substance into the standard heading set, cite the source document's
path in `## Code Locations`, set `Code Conformance`, update the next-free line. **This phase does
not need `--manifest` re-tagging** — `PROMOTION.md` states explicitly that promotion is now "an
ordinary write to a directory plus a table row," because ADRs live outside the ingest manifest
system entirely and top the precedence order by construction (D-00b, D-02 in `PROMOTION.md`).

**Two named candidates in `PROMOTION.md`'s inventory are owned by Phase 7:** candidate 1 (the
`battalion-result-upward-dependency-decision.md`, → ADR-0016) and candidate 2 (`Epic_17.5/
epic17-5.md`, the CLI-belongs-in-`src/application/cli` decision — **note this is NOT named anywhere
in CONTEXT.md's D-01…D-27 and is not part of this phase's locked scope**; its `PROMOTION.md` owner
assignment is "Phase 7" but CONTEXT.md's D-25 ADR allocation table lists only 0014-0020 for seven
subjects that do not include candidate 2. **This is a discrepancy the planner should flag**:
`PROMOTION.md`'s inventory names Phase 7 as owner of two promotable candidates, but CONTEXT.md's
locked D-25 only allocates ADR numbers for one of them. Do not silently promote candidate 2 without
a CONTEXT.md decision backing it — this is exactly the "contradicts a decision's premise" case the
task instructions call out; report it, do not resolve it. (One resolution consistent with what *is*
locked: candidate 2's substance — `src/cli` is absent, `src/application/cli` exists per `mod.rs:59`
— may already be adequately covered by ARCH-05(5)'s `src/application/use_cases/` correction and the
D-20 `structopt`/CLI finding, making a dedicated ADR redundant. But this is the researcher's
inference, not a locked answer — flag for the planner/discuss-phase, do not decide it here.)

### Anti-Patterns to Avoid

- **Trusting a cited `file:line` from an earlier document without re-grepping.** Verified this
  session: `intel/code-verification.md`'s `crate-isolation` citation (`ci.yml:228`) is stale — the
  job is now at `ci.yml:304`. `feature-flags.yml:115,118,141`, by contrast, checked out exactly.
  Every exercising-artefact citation in the ledger must be freshly verified against the current
  tree at authoring time, not copied forward.
- **Rewriting `.project/` instead of annotating it.** D-00g is explicit and Phase 5 proved the
  pattern works at scale (eleven-plus corrections in Phase 5 alone). A rewrite destroys the
  provenance five ingest runs were built on.
- **Applying the "record understates the tree" heuristic uniformly across M4/M5/M6.** D-06 is
  explicit that this corpus's dominant pattern does *not* hold here: M4's 20 open items are
  corroborated (real remaining work), M5's 17 are mostly contradicted (shipped despite the
  checkboxes), M6's 0 is corroborated (genuinely complete). Apply per-milestone, not corpus-wide.
- **Treating `STRUCTURE.md`'s top ASCII directory tree as sufficient evidence the map is correct.**
  The tree (lines 8-72) already lists all ten crates plus `doc-examples` — it is *not* the six-crate
  gap D-05 describes. The gap is specifically in the "Directory Purposes" prose section (lines
  250-283), which stops at `paladin-storage` and omits `paladin-content`, `paladin-notifications`,
  `paladin-herald`, `paladin-web`, and `doc-examples`. Verify against the *prose section*, not the
  tree, or the correction will be a no-op.
- **Awarding `satisfied` on a manifest fact alone, without the named CI job.** D-01's carve-out
  requires the manifest line *plus* the consuming CI job/build leg — a `Cargo.toml` feature
  declaration alone is `present, unproven`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Ledger document shape | A new head-note/legend/row format from scratch | Copy `milestone-01.md:1-19,152-160` and `milestone-02-03.md`'s head notes verbatim, extending only for D-02's two new verdict classes | Two working, proven instances already exist at comparable scale (118 rows); reinventing risks a third incompatible shape |
| ADR heading set | A custom decision-record template | The exact seven H2 headings from `PROMOTION.md` (`Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers`), no frontmatter | `adr-parser.cjs`'s `splitEntries` only yields structured entries from bullet/numbered lines under these exact headings; a differently-shaped ADR silently breaks machine consumption |
| `.project/` correction markup | A new banner/strikethrough convention | The exact Phase 5 pattern quoted verbatim above (blockquote banner + `~~old~~ new` heading + bold inline note) | D-00g fixes the *pattern*; Phase 5 already proved it renders correctly and preserves provenance across eleven-plus edits |
| Exercising-artefact discovery | Guessing which CI job covers a feature flag | `grep -n` against `.github/workflows/ci.yml` and `feature-flags.yml` fresh, every time — job line numbers drift as jobs are inserted | Confirmed drift this session: `crate-isolation` moved from line 228 to 304 between when `code-verification.md` was written and now |

**Key insight:** Every artefact class this phase produces already has one or two working instances
in this same repository at comparable scale. The entire research task is locating those instances
precisely enough that a planner can point an executor at exact line ranges to copy, not inventing
new shapes.

## The 115 rows themselves — per-epic census (triage, not re-verification)

Read from `REQUIREMENTS.md:2830-3068`, cross-checked against `intel/code-verification.md:82-260` and
`intel/task-completion-state.md`. **This is a triage census for planning purposes — do not re-verify
these rows; that is the phase's own execution work.**

| Epic | IDs | Existing verdict distribution (ingest-era, pre-upgrade) | Cost signal |
|------|-----|------------------------------------------------------------|-------------|
| M4 Epic 1 — Feature Flag Expansion | 7 | 1 diverged (vision), 1 partly-diverged, 4 Shipped/Verify | Cheap — mostly Shipped, one ARCH-05 divergence already fully evidenced |
| M4 Epic 2 — Port Trait Hardening & Stable API | 9 | 1 relocated, 3 Verify, 2 open defects (DEBT-01/02), 1 partial | **Expensive — the one genuinely incomplete epic; 20 corroborated open items** |
| M4 Epic 3 — CLI Isolation | 9 | 1 open defect (DEBT-04), 1 question-formally-open (ARCH-06), 2 partial, rest Shipped | Medium — checkboxes overstate completion here (inverse pattern), needs care not speed |
| M5 Epic 1 — Workspace Init & `paladin-core` | 10 | 2 Variant (edition, allowlist), 1 partly superseded, 1 Verify, rest Shipped | Medium — hosts two of the four ARCH-03 variant pairs |
| M5 Epic 2 — `paladin-ports` Extraction | 10 | 1 Variant, 2 open defects (DEBT-03), 4 Verify, rest Shipped | Medium-expensive — hosts the ARCH-03(c) port-ownership variant and both DEBT-03 doctest defects |
| M5 Epic 3 — `paladin-battalion` Extraction | 9 | 1 shipped-then-superseded (facade shim → ARCH-04), 4 Verify, 1 Variant, rest Shipped | Cheap-medium — 4 open checkboxes, contradicted (all ship) |
| M5 Epic 4 — `paladin-llm` Extraction | 11 | 1 Variant (bridge location), 3 Verify, rest Shipped | Cheap — 1 open checkbox, contradicted |
| M5 Epic 5 — `paladin-memory` Extraction | 10 | 1 Variant (edition v2), 4 Verify, rest Shipped | Cheap — the only epic verified line-for-line against its PRD |
| M5 Epic 6 — Workspace Finalization | 6 | 1 partial (DEBT-01), 1 contested (ARCH-07 benchmark), 2 Verify, rest Shipped | Medium — hosts ARCH-07's benchmark-report contest |
| M6 Epic 1 — `application_settings.rs` Decomposition | 8 | 1 diverged (config map), 3 Verify, 1 Variant (bridge v2), rest Shipped | Cheap — 0 open checkboxes, corroborated |
| M6 Epic 2 — Orchestration Service Relocation | 9 | 1 diverged (ARCH-05(5)), 1 contested (ARCH-04), 5 Verify, rest Shipped | Cheap-medium — hosts ARCH-04's facade policy and ARCH-05(5) |
| M6 Epic 3 — Maneuver DSL Co-location | 9 | 6 Verify, rest Shipped | Cheap — mechanical, mostly file-existence Verify rows |
| M6 Epic 4 — `CircuitBreaker` Relocation | 8 | 1 contested (ARCH-04), 1 diverged (ARCH-05 docs), 4 Verify, rest Shipped | Cheap — 0 open checkboxes, hosts ARCH-04's second facade row |

**Per-milestone open-checkbox corroboration (D-06), confirmed from `intel/code-verification.md:246-260`:**
- **M4: 93.2% complete, 20 open (all Epic 2), corroborated.** Real remaining work.
- **M5: 96.4% complete, 17 open, mostly contradicted.** All six crates, prelude, CI job, benchmark
  report exist despite the open checkboxes; items 1 (api-surface stale path) and 4 (CLI-only deps
  unconditional) are the genuine residue.
- **M6: 100% complete, 0 open, corroborated.** All four relocations verifiably complete.

**Cheapest block:** M6's four epics (34 IDs) — 0 open items, all relocations verified complete,
mostly file-existence `Verify` rows that resolve quickly. **Most expensive:** M4 Epic 2 (9 IDs) —
the one genuinely incomplete epic, 20 corroborated open items, two DEBT items to cross-reference.

## Ledger mechanics at 115 rows (Phase 5 precedent)

`milestone-02-03.md` (118 rows, the closest-scale analogue) was built in exactly this shape:

1. **Head notes, in this fixed order**: title → supersession note (points at REQUIREMENTS.md
   section it replaces) → primary-key note (D-00e) → evidence-bar note (D-01, stated in full,
   "applies to all N rows without exception, including every row already marked Shipped by ingest")
   → path-caveat note (D-04's two caveats, stated once) → `## Verdict legend` (H2, exact five/seven-
   value table) → `## Row order and amendment convention` (H2).
2. **Fixed epic-section order**, matching REQUIREMENTS.md's own order, never re-sorted. For Phase 7:
   the 13 headings in REQUIREMENTS.md's existing order (M4 Epic 1, 2, 3; M5 Epic 1-6; M6 Epic 1-4).
3. **Every row starts `| REQ-`, table header exactly `| ID | Verdict | Evidence |`.**
4. **The scaffold plan (Phase 5's 05-01) fully cites one epic end-to-end** (Epic 11, 8 rows) to
   prove the citation workflow before fan-out, leaving the other epics as `PENDING-VERDICT` stubs
   with the plan number that will fill them. **Apply the same tracer-task pattern here**: pick one
   small, well-evidenced epic (M6 Epic 3 or Epic 4, both cheap per the census above) to fully cite
   in the scaffold plan, stub the rest.
5. **Automated verification gates Phase 5 used** (reusable verbatim, substituting 115 for 118, 13
   for 14): row count equals total IDs; section-heading count equals epic count; no duplicate
   `REQ-*` IDs (`sort | uniq -d` prints nothing); heading order matches a fixed `grep -n` sequence;
   `PENDING-VERDICT` count equals (total − tracer-epic rows); REQUIREMENTS.md's old ledger section
   has zero `^| REQ-` rows after the pointer-reduction task; the plan's own commit touches no `.rs`,
   `Cargo.toml`, or `.github/workflows/` file.
6. **Summary section written last**, by the final plan, counting rows rather than assuming — Phase
   5/6's own precedent (`06-07`'s "recomputed by counting" entry in STATE.md) is explicit that
   verdict-distribution counts must be recomputed, not carried forward from an earlier draft.
7. **Pre-commit hooks compile the whole workspace even for markdown-only commits** (per the repo's
   `.pre-commit-config.yaml` `always_run: true` on `cargo-fmt`/`cargo-clippy`) — Phase 5's plans
   commit once per plan, not per row, and budget Bash timeouts ≥ 300000ms. `worktree_skip_hooks=true`
   is already set in `.planning/config.json`'s `workflow` block for this project — confirm this flag
   is surfaced in every executor prompt this phase spawns, or every commit cold-compiles the
   12-crate workspace.

## The evidence bar in practice — exercising artefacts available in this repo

**CI jobs in `.github/workflows/ci.yml`** (verified fresh this session; **line numbers below
supersede any earlier document's citations, including `intel/code-verification.md`'s stale
`ci.yml:228`**):

| Job key | Line | Purpose | Relevant to |
|---|---|---|---|
| `lint` | 21 | fmt/clippy | general |
| `security-audit` | 61 | bare `cargo audit` | out of scope (Phase 9) |
| `cargo-deny` | 81 | `cargo deny check` | out of scope (Phase 9) |
| `osv-scanner` | 111 | OSV scan | out of scope |
| `api-surface` | 140 | API surface tracking — **still references stale `project/current-exports.txt`** (line ~171 `./scripts/check-api-surface.sh project/current-exports.txt`) | DEBT-01, cited not fixed here |
| `test` | 191 | unit + doc tests; **line 226: `cargo test --workspace --doc --exclude paladin-ports`** — the doctest-exclusion citation for DEBT-03/`REQ-ports-doctest-compilation` | ARCH-01 rows re: doctests |
| `examples` | 230 | Example Muster (feature matrix) | REL-05 (out of scope) |
| `crate-isolation` | **304** (was cited as 228 — now stale) | per-crate isolated build matrix, FR-2.8's exercising artefact | D-01's manifest carve-out; `REQ-crate-isolation-ci` |
| `integration-tests` | 359 | integration test suite | general exercising artefact |
| `security` | 466 | second `cargo audit` job (the SEC-01 duplicate) | out of scope |
| `docker` | 498 | Docker build | out of scope |
| `kubernetes-smoke` | 615 | k8s smoke test | out of scope |
| `benchmark` | 783 | perf benchmarks | out of scope |
| `benchmark-regression-signal` | 816 | regression detection | out of scope |
| `publish-dry-run` | 902 | crates.io dry-run | out of scope |

**`.github/workflows/feature-flags.yml`** (verified fresh; **these line numbers are exactly correct
against CONTEXT.md's D-01 citations, no drift found**):

| Job key | Line | Purpose | Relevant to |
|---|---|---|---|
| `feature-matrix` | 19 | 10-combination feature matrix (`no-default-features`, `default`, `all-features`, `full`, per-provider LLM flags, `web-server`, …) | `REQ-feature-ci-matrix`, ARCH-03(b) allowlist rows |
| — build step | **115** | `cargo build --workspace ${{ matrix.flags }}` | D-01's manifest carve-out exercising leg |
| — test step | **118** | `cargo test --workspace --lib ${{ matrix.flags }}` | same |
| `cli-isolation` | 121 | dedicated CLI-isolation job | `REQ-cli-test-isolation` |
| — cli_isolation regression test | **141** | `cargo test --test cli_isolation` | `REQ-cli-test-isolation`, D-20/DEBT-04's evidence |
| `feature-matrix-summary` | 150 | gate requiring all matrix legs pass | general |

**Test targets in `tests/`** (root-level integration test binaries, plus `tests/integration/`
subdirectory with 20+ per-feature test files):

- `tests/cli_isolation_test.rs` — the exact target `feature-flags.yml:141` runs; the file backing
  `REQ-cli-test-isolation`/`REQ-library-only-integration-tests`.
- `tests/integration/mcp_streamable_http_test.rs` — the five MCP failure-mode tests (already
  exercised in Phase 3; relevant only if a Milestone 4-6 row needs an MCP exercising artefact — none
  of the 115 rows appear to, since ARCH-05(2) records the MCP flags as *absent*, not exercised).
- No dedicated `crate_edition_test.rs` or `dependency_allowlist_test.rs` exists — the manifest
  carve-out (D-01) is exactly why: these are structural facts proven by the CI job existing and
  passing, not by a Rust test file.

**Manifest facts directly checkable** (no test needed under D-01's carve-out; cite the manifest line
plus the CI job above):
- `Cargo.toml` root, all 12 manifests: `edition = "2024"` — verified this session, matches D-09's
  already-recorded ADR-0009 answer exactly.
- `Cargo.toml:240-252` — three `[[bin]]` targets: `paladin` (no `required-features`), `paladin-cli`
  (`required-features = ["cli"]`), `paladin-server` (`required-features = ["web-server"]`) —
  verified this session at exactly these lines.
- `Cargo.toml` feature block (lines ~270-286 in the current file — verify exact line at authoring
  time, drift is possible): `vision = []` (gates nothing), `web-server = ["dep:paladin-web",
  "dep:axum"]`, `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console",
  "dep:serde_yaml"]`, `full = [...]` — all verified this session.
- `crates/paladin-core/Cargo.toml` `[dependencies]`: 14 entries verified this session — `serde`,
  `serde_json`, `uuid`, `chrono`, `thiserror`, `async-trait`, `tokio`, `sha2`, `blake3`, `petgraph`,
  `murmur3`, `url`, `regex`, `futures`. Matches CONTEXT.md's D-10 figure exactly.
- `crates/paladin-ports/Cargo.toml` `[dependencies]`: **11 entries** verified this session —
  `paladin_core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, `tokio`, `serde_json`,
  `futures`, `md5`, `mime_guess`. Matches CONTEXT.md's D-10 figure of 11 (correcting
  `intel/code-verification.md`'s stale figure of 10, which predates `mime_guess`).

## The `.project/` correction sites

All fourteen documents CONTEXT.md names as reading or correction targets were confirmed to exist at
their stated paths (verified this session via direct filesystem check, 14/14 `OK`, 0 `MISSING`). No
path in CONTEXT.md failed to resolve.

| # | Path | What's there | Section/line to annotate |
|---|------|--------------|---------------------------|
| 1 | `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md` | Titled "Milestone 1: High-Value, Low-Risk Foundations" | Title/heading — D-08(1) |
| 2 | `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md` | Titled "Milestone 2: Workspace Decomposition"; also carries the `paladin-cli` target structure + Appendix D (ARCH-05(4)) | Title + Appendix D — D-08(2), ARCH-05(4) |
| 3 | `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md` | Prerequisites "Completed in Milestones 1 and 2"; Epic 2 AC 6, Epic 4 AC 5, risk register re-export language | Prerequisites line + AC 6/AC 5/risk register — D-08(3), D-13 |
| 4 | `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` | Non-Goal 2 "hardened in Milestone 1 / Epic 2"; FR-31/FR-32 circular-dependency concern | Non-Goal 2 (D-08(4)) + FR-31/FR-32 (D-12) |
| 5 | `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` | §1 same cross-reference; FR-7, FR-10, FR-11 (port ownership) | §1 (D-08(5)) + FR-7/FR-10 (D-11) |
| 6 | `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md` | FR1 + Design Considerations: `vision` gating claim, MCP transport flags, `web-server` gating | FR1 + Design Considerations — ARCH-05(1)(2)(3) |
| 7 | `.project/Milestone_4-Refactor-Crates-Features/Epic_1/dependency-matrix.md` | The audit that was **right** — cited, not corrected | No edit — citation source only |
| 8 | `.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md` | FR-6 + Appendix B, "exhaustive" 6-dep allowlist | FR-6 + Appendix B — ARCH-03(b), ADR-0015 cites |
| 9 | `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` | The Approved-but-DOC decision — cited/promoted by ADR-0016, **not edited directly** | No edit — ADR-0016 restates it |
| 10 | `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md` | Non-Goal 7, Open Question 4, `src/application/use_cases/` target | Non-Goal 7/OQ4 (D-13) + target structure (ARCH-05(5)) |
| 11 | `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md` | Goal 7, FR-4.11, FR-4.12 | Goal 7/FR-4.11 (D-13) + FR-4.12 re-pointing (D-18) |
| 12 | `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` | Five scenarios, contradictory conclusion — cited verbatim by ADR-0020, **not edited** | No edit — citation source only |
| 13 | `.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md` | FR-3.5 + SM-7, the target ADR-0020 restates | Cited, likely no edit needed (ADR restates, doesn't correct a false claim here) |
| 14 | `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md` | FR2.1, FR5.4, §8.3 — the isolation target D-20 re-scopes | Cited for D-20's finding; correction (if any) is Phase 8's, not this phase's — confirm scope boundary in the plan |

**Byte-equivalent extracts (D-08's "seven", INGEST-CONFLICTS' list of eight — flag the count
discrepancy to the planner, see Pattern 1 above):**
`.project/Milestone_5-Workspace-Decomposition/Epic_2/…Epic_2.md`,
`Epic_3/…Battalion_Extraction.md`, `Epic_4/Milestone_5-Epic_4.md`, `Epic_5/Milestone_5-Epic_5.md`,
`.project/Milestone_6-Architectural-Refinements/Epic_1/…Decompose_App_Settings.md`,
`Epic_2/…Relocation_Manager_Layer_Orch_services.md`,
`Epic_3/…Co-locate_Maneuver_DSL_w_Battalion.md`, `Epic_4/…Relocate_CircuitBreaker_Infra_Layer.md`
— one-line dated pointer banner only, no inline strikethrough (per `INGEST-CONFLICTS.md:656`, these
carry no independent content).

## The Phase 5 correction pattern — verbatim (reproduced in full above)

See "Pattern 1" under Architecture Patterns. The exact three markup shapes (top banner, inline
strikethrough heading + bold note, standalone "Superseded" banner) were transcribed verbatim from
`.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:1-8,28-31,83,360` during this
research session — copy these forms exactly rather than re-deriving prose style.

## ADR conventions

`PROMOTION.md` was read end-to-end. Key facts for the planner:

- **Numbering:** flat, zero-padded, monotonic (`NNNN-kebab-slug.md`). Next free is **0014**; Phase 7
  takes 0014-0020 and must update the "Next free ADR number" line to **0021** in its own bookkeeping
  plan (D-25/D-26 territory).
- **Required heading set, exact order:** `## Status` → `## Context` → `## Decision` →
  `## Considered Options` → `## Code Locations` → `## Code Conformance` → `## Downstream Consumers`.
  No frontmatter (D-00h). `## Code Locations` and `## Considered Options` **must be bulleted lists**,
  never prose — `adr-parser.cjs`'s `splitEntries` only yields structured entries from bullet/numbered
  lines.
- **Supersession mechanism:** superseded ADRs keep their file; `## Status` becomes bare `Superseded`
  plus a prose line naming the superseding ADR; the superseding ADR carries a `## Supersedes` line.
  Not directly relevant to 0014-0020 (none of them supersede an existing ADR — they're all net-new),
  but relevant if a future phase revisits one of these seven.
- **Promotion procedure** (five steps, already summarized under Pattern 3 above): take next number,
  author into standard heading set, cite the source `.project/` path in `## Code Locations`, set
  `Code Conformance`, update the next-free line, add a `PROJECT.md` Key Decisions row.
- **ADR-0008 and ADR-0009 (both read in full)** demonstrate the citation pattern D-09/D-14 use: a
  later phase's requirement text is *amended* to cite the earlier ADR rather than re-deciding, and
  the earlier ADR's own `## Downstream Consumers` names the later phase's requirement explicitly.
  **Structural template for ADR-0018's version-consequence half**: copy ADR-0008's `## Context`
  framing (three-way version disagreement, cross-phase coupling table entry, "whichever phase runs
  first records the answer") almost verbatim, substituting the facade re-export policy as ADR-0018's
  *additional* scope beyond what ADR-0008 already settled.
- **ADR-0010 (read in full)** is the exact structural precedent for ADR-0014 (a numbering-collision
  ADR with a mapping table, `## Considered Options` rejecting both "trust the minority document" and
  "renumber the majority," and a `## Code Locations` section citing exact `file:line`s in the
  document being corrected). Copy this ADR's shape almost mechanically for ADR-0014 — same
  collision class, same corpus, same convention (D-07 explicitly requires ADR-0014 to cite ADR-0010).
- **ADR-0011 (read in full)** is the precedent for a "both surfaces are intentional, not competing"
  disposition plus a "built, never wired" third verdict class — relevant background for ARCH-05(1)'s
  `vision` gating correction (D-17 explicitly cross-references ADR-0011 as dispositioning "this same
  encryption code"). ADR-0011's `## Considered Options` also demonstrates the "record as dropped —
  rejected" / "record as shipped — rejected" balancing act useful for framing ADR-0015's dependency-
  allowlist decision (accept the extras as legitimate, don't manufacture debt, per D-10).

## The build-benchmark source (verbatim, transcribed for ADR-0020)

Read in full from `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md`.
**ADR-0020 needs these figures verbatim — reproduced here so the planner doesn't need to re-open the
source file:**

**Summary table (exact, `build-benchmarks.md:63-71`):**

| Scenario | Monolith median | Workspace median | Improvement | Meets ≥ 50% target? |
|----------|----------------|-----------------|-------------|---------------------|
| Clean build | 275,681 ms (4m 35s) | 257,492 ms (4m 17s) | −6.6% | ❌ No |
| Incremental — core change | 17,302 ms (best-case) | 14,029 ms | −18.9% | ❌ No |
| Incremental — LLM adapter change | 17,302 ms (best-case) | 9,583 ms | −44.6% | ❌ No |
| Incremental — memory adapter change | 17,302 ms (best-case) | 8,618 ms | −50.2% | ✅ Yes |
| Incremental — battalion only | 17,302 ms (best-case) | 1,571 ms | **−90.9%** | ✅ Yes |

**Conclusion sentence (`build-benchmarks.md:101-111`, verbatim):**
> "The workspace decomposition **achieves the ≥ 50% incremental build time improvement target** for
> 2 of 4 incremental scenarios (D: 50.2%, E: 91%)..."
> "The clean build scenario does not meet the 50% target (**−5%**), which is expected by design..."
> "**Overall verdict: Target achieved.**"

**The transcription-error inconsistency (D-22 must resolve this):** the summary table states the
clean-build figure as **−6.6%**; the conclusion paragraph restates it as **"(−5%)"**. These are two
different numbers for the same measurement in the same document. ADR-0020 cites the table's −6.6%
as authoritative and records the conclusion's "(−5%)" as a transcription error — do not attempt to
reconcile them as if they measured different things.

**Methodology note (`build-benchmarks.md:59`, verbatim, the report's own caveat):**
> "The monolith baseline incremental (BL-B) was measured by touching `src/lib.rs` (the crate root)
> and running `cargo build`. Because `src/lib.rs` in the monolith is primarily a module-tree
> re-export file, Rust's incremental compilation pipeline detects no fingerprint changes in
> downstream modules and can skip the bulk of recompilation — producing a fast relink. This
> represents the *best-case* monolith incremental... This means the comparisons below are
> *conservative estimates* of the workspace advantage."

**Recommended follow-up the report itself states (`build-benchmarks.md:107`, this is what D-24
declines with a recorded reason):**
> "The baseline methodology should be strengthened in a future measurement by touching a mid-tree
> implementation file (e.g. `src/infrastructure/adapters/llm_adapter.rs`) rather than just the crate
> root, to produce a more representative monolith incremental time for scenarios B and C."

**Environment note for ADR-0020's own framing:** the report's baseline was measured against commit
`08dc944` (origin/main, pre-decomposition), a monolith commit that **no longer exists on this
branch's history in a buildable form** — this is exactly why D-22 declines re-measurement rather than
attempting it (resurrecting a historical commit for a metric about a restructuring completed three
milestones ago, under this environment's documented offline/no-Docker constraints).

## Crate-level facts (cross-checked against `intel/code-verification.md`'s own contradiction table)

`intel/code-verification.md:230-244`'s "Crate-level facts that contradict run-3 requirement text"
table is essentially ARCH-03's and ARCH-05's evidence pre-assembled. All entries were spot-checked
against the live tree this session; no discrepancies found beyond the `paladin-ports` dependency
count (10 → **11**, `mime_guess` added since, already recorded in CONTEXT.md D-10 and confirmed
above) and the `crate-isolation` CI line-number drift (already flagged). The table's other rows
(edition mixed/now-uniform per ADR-0009, `paladin-core` allowlist 6→14, LLM bridge location,
`vision`/`web-server` gating, no MCP flags, no `paladin-cli` crate, config decomposition hybrid
shape, three binary targets, M6 Epic 2 target directory) all check out as stated.

## Binary targets — the three, confirmed at `Cargo.toml:240-252`

```
[[bin]]
name = "paladin"
path = "src/main.rs"
# no required-features — this is the un-gated default binary

[[bin]]
name = "paladin-cli"
path = "src/bin/paladin-cli.rs"
required-features = ["cli"]

[[bin]]
name = "paladin-server"
path = "src/bin/paladin-server.rs"
required-features = ["web-server"]
```

`src/main.rs`'s content (per CONTEXT.md, already verified this session — not re-derived here):
`#[structopt(name = "smartcontent-aggregator")]`, loads `config.yml` via
`Settings::load_from_file`, calls `paladin::config::setup::setup_and_run`. This is ADR-0019's D-19
finding — the `paladin` binary is not an agent-orchestration entry point but the pre-Paladin content
aggregator, and ADR-0019 must state this plainly per D-19's own instruction not to invent a tidy
purpose.

`structopt`'s coupling (D-20): its only consumer in the tree is `src/main.rs`. `paladin-herald`
declares `colored` and `comfy-table` unconditionally with **no `[features]` section at all** — a
separate, additional finding beyond `structopt` that also frustrates a library-only build via a
different route (Herald is a dependency of the facade, not gated by the `cli` feature at all).

## Runtime State Inventory

**Not applicable.** This phase is not a rename/refactor/migration phase in the runtime-state sense
— it touches no database, no live service configuration, no OS-registered state, and no build
artefacts. It corrects historical *documents* about a workspace decomposition that already happened
in Milestones 5-6; the decomposition itself is not this phase's work. Stated explicitly per the
verification protocol's instruction not to leave this blank: **none of the five Runtime State
Inventory categories apply — verified by the phase's own explicit boundary statement in CONTEXT.md
("This phase writes records and decisions. It does not change product code") and confirmed by this
research finding no database, CI-only, or OS-level state anywhere in this phase's artefact list.**

## Common Pitfalls

### Pitfall 1: Citing a stale line number from an earlier verification pass
**What goes wrong:** A ledger row cites `ci.yml:228` for `crate-isolation`, copied from
`intel/code-verification.md`, and the citation resolves to the wrong job (or blank/wrong content)
because the file has grown since that citation was written.
**Why it happens:** `ci.yml` gained three new jobs (`security-audit`, `cargo-deny`, `osv-scanner`,
plus expansion of `api-surface`) between when `code-verification.md`'s run-3 verification was done
(2026-07-30) and now, shifting every job below them down by ~76 lines.
**How to avoid:** Re-`grep -n` every citation at the moment of writing the ledger row, never trust a
line number transcribed from CONTEXT.md, `intel/`, or this research document without a fresh check.
**Warning signs:** A `sed -n 'N,N+5p'` on the cited line doesn't show the expected job/content.

### Pitfall 2: Applying the "checkbox understates the tree" heuristic uniformly
**What goes wrong:** A verifier assumes, as in runs 1-2 of this corpus, that every milestone's open
checkboxes are stale pessimism and the tree actually ships more than claimed — and marks M4 Epic 2's
20 open items `satisfied` on the strength of that prior.
**Why it happens:** It has been the dominant pattern in this corpus for two prior runs.
**How to avoid:** D-06 explicitly breaks the pattern here — M4's 20 open items are the *first
corroborated* block in the corpus (verified: zero `#[deprecated]` annotations, no baseline file).
Verify each milestone's open-item claim independently; do not extrapolate from M5's or M6's pattern.
**Warning signs:** An epic-level note that reads "checkboxes overstate/understate" without a fresh
grep backing the specific claim for that epic.

### Pitfall 3: Treating a `.project/` correction as license to delete or rewrite
**What goes wrong:** An executor "cleans up" a `.project/` document by removing the now-wrong
Milestone-numbering text instead of striking it through and annotating.
**Why it happens:** Natural instinct when correcting a factual error in prose.
**How to avoid:** D-00g is a hard rule, not a style preference — the corpus's provenance (five ingest
runs) depends on original text surviving. Follow the exact Phase 5 pattern (banner + `~~old~~ new` +
bold inline note) transcribed verbatim above.
**Warning signs:** A `git diff` on a `.project/` file shows deletions, not just additions.

### Pitfall 4: Awarding `satisfied` to a manifest fact with no named CI job
**What goes wrong:** A row for a feature-flag shape (e.g. `vision = []`) gets marked `satisfied` on
the strength of the `Cargo.toml` line alone.
**Why it happens:** The fact is trivially verifiable by reading the manifest, so it feels "obviously
true."
**How to avoid:** D-01's carve-out is explicit: manifest line **plus** a named CI job or build leg
that consumes it. Cite both, e.g. "`vision = []` at `Cargo.toml:274`, exercised by
`feature-flags.yml`'s `no-default-features`/`all-features` matrix legs (lines 25-30, 115-118)."
**Warning signs:** A `satisfied` row's Evidence cell has only one citation, not two.

### Pitfall 5: Blocking Phase 8 planning by sequencing ADR-0016 last
**What goes wrong:** The plan decomposition puts ADR-0016 in the final "decisions" batch, and
Phase 8 cannot be planned until this entire phase completes even though only ADR-0016 gates it.
**Why it happens:** Natural to batch all seven ADRs together for authoring efficiency.
**How to avoid:** D-27's own plan-decomposition note flags this; land ADR-0016 as early as sequencing
allows, even if it means splitting it out of the "0015+0016+0017" batch D-27 suggests.
**Warning signs:** ROADMAP.md still shows Phase 8 "blocked" after every other Phase 7 plan has
landed except the ADR-authoring batch.

## Code Examples

Not applicable in the conventional sense (no Rust code is written by this phase). The load-bearing
"code examples" for this phase are the markdown templates already transcribed verbatim above under
Architecture Patterns (the correction-banner pattern, the ADR heading set, the ledger head-note
shape) — reproducing them again here would be redundant.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Inline per-milestone ledgers grown directly in REQUIREMENTS.md | Sibling files in `.planning/ledgers/`, REQUIREMENTS.md reduced to pointers | Phase 1 (D-17), continued by Phase 5 (D-21), this phase (D-24/D-26) | REQUIREMENTS.md stays readable at ~4,000 lines instead of growing five inline verdict sets |
| Re-tagging a `.project/` source document via `--manifest` to promote a decision | ADRs as an independent document class outside the ingest manifest, "an ordinary write to a directory plus a table row" | `PROMOTION.md`'s own stated rationale, "viable now, where it previously was not" — the ingest is closed (no run 6) | ADR-0016 can promote the port-ownership decision without re-running a closed ingest pipeline |
| Five-value verdict vocabulary (Phase 1/2) | Seven-value vocabulary adding `relocated` and `diverged` (Phase 5 D-16/D-17 precedent, this phase's D-02) | This phase, following Phase 5's own extension pattern | Distinguishes "moved, not missing" from "shipped code deliberately differs" — both distinct from `superseded by shipped code` |

**Deprecated/outdated:** The "6-crate workspace" and "9-crate workspace" figures both predate this
phase's correction — the real shape is ten library crates + `doc-examples` + the root `paladin-ai`
facade, confirmed at all three correction sites (ledger head note, REQUIREMENTS.md pointer,
`STRUCTURE.md` prose section).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | ADR-0015/0016/0017 should be three separate files rather than combined, following the D-25-recommended default | Architecture Patterns / Standard Stack | Low — CONTEXT.md's own Claude's Discretion section explicitly permits combining, with the sole hard constraint that 0016 stays separately citable. This research assumes three files as the default shape but does not lock it. |
| A2 | The "seven byte-equivalent extracts" (D-08) and the eight-file list at `INGEST-CONFLICTS.md:656` describe the same set, with D-08's count being an off-by-one in prose rather than a different set | .project/ correction sites, Pattern 1 | Medium — if the sets actually differ, a plan following D-08's "seven" literally could miss one file's pointer banner. Flagged explicitly for the planner to resolve by direct enumeration, not by trusting either count. |
| A3 | `PROMOTION.md`'s inventory candidate 2 (`Epic_17.5/epic17-5.md`, the CLI-placement decision, "Owner phase: Phase 7") is **not** part of this phase's locked scope, since CONTEXT.md's D-25 ADR allocation table only names 0014-0020 for seven ARCH-*-numbered subjects that do not include it | Pattern 3 | Medium — if PROMOTION.md's ownership assignment is meant to be binding independent of CONTEXT.md, this phase should also promote candidate 2 (an eighth ADR, 0021, pushing the next-free line to 0022). This research treats CONTEXT.md as authoritative (it is the locked, discussed decision record) and flags the PROMOTION.md assignment as a discrepancy rather than silently acting on it either way. |
| A4 | The `Cargo.toml` feature-flag block's exact line numbers (`vision` at 274, `web-server` at 276, `cli` at 284) may drift by the time plans execute, the same way `ci.yml:228` drifted to 304 | The evidence bar in practice | Low-medium — these were freshly verified this session (2026-08-06), same day as CONTEXT.md's own verification, so drift risk before plan execution is low, but should still be re-grepped rather than hard-coded into a plan's prose. |

## Open Questions

1. **Does the seven-vs-eight byte-equivalent-extracts count (D-08 vs. `INGEST-CONFLICTS.md:656`)
   resolve to the same file set?**
   - What we know: `INGEST-CONFLICTS.md:656` names exactly eight `.project/` files as byte-equivalent
     copies; D-08's prose in CONTEXT.md says "seven."
   - What's unclear: Whether D-08 excludes one of the eight deliberately (e.g. because one of them
     is separately corrected elsewhere in D-08's own five-document list, making it not purely a
     "pointer banner only" case) or whether this is a simple count discrepancy.
   - Recommendation: The scaffold or correction-batch plan should enumerate the exact file list by
     re-running the `INGEST-CONFLICTS.md:656` grep and diffing against D-08's implied set, then
     record whichever count is correct with the reasoning, rather than picking a number silently.

2. **Is `PROMOTION.md` candidate 2 (`Epic_17.5/epic17-5.md`) in scope for this phase?**
   - What we know: `PROMOTION.md`'s own inventory names "Owner phase: Phase 7" for this candidate.
     CONTEXT.md's locked D-25 allocates exactly seven ADR numbers (0014-0020) for seven named
     subjects, none of which is this candidate.
   - What's unclear: Whether CONTEXT.md's D-25 is an intentional narrowing of `PROMOTION.md`'s
     inventory (i.e., candidate 2 is quietly out of scope for this phase and belongs to a later
     phase or is considered already adequately covered by ARCH-05(5)) or an oversight.
   - Recommendation: Flag for the planner/discuss-phase rather than resolve. Since D-25 is locked
     and explicit about "seven new ADRs," this research treats candidate 2 as **not** this phase's
     scope, but the discrepancy against `PROMOTION.md`'s own inventory should be visible to whoever
     next edits `PROMOTION.md`'s ownership column.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `git` | Reading `.project/` history, confirming file existence | ✓ | present | — |
| `grep`/`sed` | Every citation-verification step in this phase | ✓ | present | — |
| `cargo` | Confirming manifest facts (`cargo tree`, if a plan wants to double-check the allowlist) | ✓ | present (per repo's `rust-toolchain.toml`, 1.97.1) | Not required — this phase reads `Cargo.toml` directly, doesn't need to build |
| Docker | Not required — this phase runs no integration tests, builds nothing | n/a | n/a | n/a |

No missing dependencies. This phase's entire toolchain is already present and sufficient — it writes
Markdown and reads the existing tree.

## Validation Architecture

> This is a records phase — "validation" means how a later reader or verifier confirms a ledger row
> is honest, an ADR is well-formed, and a `.project/` annotation is present, not a test suite.

### Greppable honesty checks a later verifier (or the plan-checker) can run

| Check | Command | What it confirms |
|-------|---------|-------------------|
| Ledger row count matches requirement count | `grep -c '^| REQ-' .planning/ledgers/milestone-04-06.md` equals `115` | No row silently dropped |
| No duplicate `REQ-*` IDs | `grep -o '^| REQ-[a-z0-9-]*' .planning/ledgers/milestone-04-06.md \| sort \| uniq -d` prints nothing | Primary key integrity (D-00e) |
| No stub rows survive | `grep -c 'PENDING-VERDICT' .planning/ledgers/milestone-04-06.md` equals `0` after the phase closes | All 115 rows actually verdicted |
| Every ADR carries a Conformance verdict | `grep -L 'conforms\|must change' .planning/decisions/001{4..9}*.md .planning/decisions/0020*.md` prints nothing | D-00c compliance |
| Every ADR uses the exact required heading set | `for f in .planning/decisions/001{4..9}*.md .planning/decisions/0020*.md; do grep -c '^## ' "$f"; done` — each should show 7 matching the canonical set | `adr-parser.cjs` compatibility (D-00h) |
| `PROMOTION.md`'s next-free line advanced | `grep -c 'Next free ADR number: 0021' .planning/decisions/PROMOTION.md` equals `1` at phase close | D-25/D-26 bookkeeping complete |
| REQUIREMENTS.md's old ledger section has zero inline rows | `sed -n '/^## Milestone 4-6 as-shipped ledger/,/^---/p' .planning/REQUIREMENTS.md \| grep -c '^| REQ-'` equals `0` | D-26 pointer-reduction complete, no diverging second copy |
| `.project/` corrections are additive, never destructive | `git diff --stat <correction-commit>` shows only insertions plus a small number of context-line changes for strikethrough markup, no large deletion blocks | D-00g provenance preservation |
| No code file touched | `git diff --stat <phase-range> -- '*.rs' 'Cargo.toml' '.github/'` is empty | The phase's own hard boundary ("does not change product code") |
| `STRUCTURE.md` correction lands at the right section | `sed -n '250,290p' .planning/codebase/STRUCTURE.md \| grep -c '^\*\*`crates/'` equals `11` (6 original + 5 new) after the fix | D-05 targets the prose section, not just the ASCII tree |

### Sampling rate
- **Per plan commit:** the plan's own automated `<verify>` block (row counts, heading counts,
  duplicate checks) — same shape as Phase 5's 05-01 plan, reusable near-verbatim.
- **Phase gate:** the greppable checks table above, run once at phase close by whoever runs
  `/gsd-verify-work` or the phase's own summary plan.

### Wave 0 gaps
None — every check above is a `grep`/`sed`/`git diff` one-liner against files this phase itself
creates; no test framework or fixture needs to be built first.

## Security Domain

**Not applicable — `security_enforcement` disposition: this phase touches no authentication,
authorization, cryptographic, or input-validation surface.** It reads and annotates documentation.
The one adjacent security-flavored finding (ARCH-05(1), `vision` gating `chacha20poly1305`/
`zeroize`) is a **build-configuration** correction, not a security-control change — the encryption
code itself is untouched; only the (incorrect) claim that a feature flag gates it is corrected. No
ASVS category applies to a phase that writes zero application code.

## Sources

### Primary (HIGH confidence — direct tree/file reads this session)
- `.planning/phases/07-workspace-ground-truth-recorded-answers/07-CONTEXT.md` — full read, all 27
  decisions plus canonical refs, code context, specifics, deferred sections.
- `.planning/REQUIREMENTS.md:695-810` (ARCH-01…07 full text) and `:2830-3068` (115-row ledger).
- `.planning/ledgers/milestone-01.md` (partial, 295/698 lines) and `milestone-02-03.md` (partial,
  150 lines) — head notes, legend, row/summary shape.
- `.planning/phases/05-milestone-2-3-ground-truth/05-01-PLAN.md` — full read, scaffold-plan shape,
  verification gates, probe-fallback disposition.
- `.planning/decisions/PROMOTION.md`, `0008`, `0009`, `0010`, `0011` — full reads.
- `.planning/intel/code-verification.md:82-260` and `.planning/intel/task-completion-state.md` —
  full reads.
- `.planning/INGEST-CONFLICTS.md:251-346` and byte-equivalence note at `:656` and mdbook-relocation
  note at `:706-707` — full reads.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:1-60` — the Phase 5 correction
  pattern, read directly.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` — full read.
- `.planning/codebase/STRUCTURE.md:1-100,245-339` — full read, confirmed the exact gap D-05
  describes and its insertion point.
- `.github/workflows/ci.yml` and `feature-flags.yml` — job listings and targeted line reads,
  confirmed one line-number drift (`crate-isolation` 228→304) and one exact match
  (`feature-flags.yml:115,118,141`).
- `Cargo.toml` (root) lines 235-290, `crates/paladin-core/Cargo.toml`,
  `crates/paladin-ports/Cargo.toml` — direct reads confirming dependency counts, bin targets,
  feature flags.
- Fourteen `.project/` source paths existence-checked directly (`test -f`), 14/14 confirmed.
- `.planning/STATE.md` (partial, 603/752 lines) — full ingest-run history, ADR candidates,
  blockers/concerns register.
- `.planning/ROADMAP.md:510-569` — Phase 7-10 goal/dependency/success-criteria text.

### Secondary (MEDIUM confidence)
- None — every claim in this document is either a direct tree read or a locked CONTEXT.md decision
  cited verbatim.

### Tertiary (LOW confidence)
- None.

## Metadata

**Confidence breakdown:**
- Ledger mechanics and document shape: HIGH — two working analogues at comparable scale, both read
  directly, plus the exact scaffold plan that produced one of them.
- ADR conventions: HIGH — `PROMOTION.md` read end-to-end, four exemplar ADRs read in full.
- `.project/` correction sites: HIGH — all 14 named paths existence-verified directly; exact
  correction pattern transcribed verbatim from a proven prior instance.
- CI/exercising-artefact citations: HIGH, with one confirmed drift flagged (`ci.yml` crate-isolation
  line number) — freshly re-grepped this session, not trusted from an earlier document.
- The 115-row triage census: HIGH for distribution/counts (directly read from REQUIREMENTS.md and
  intel files); explicitly NOT a re-verification, per the task's own instruction.
- Open questions (byte-equivalent count, PROMOTION.md candidate-2 scope): flagged rather than
  resolved, per instruction not to substitute a decision for a CONTEXT.md contradiction.

**Research date:** 2026-08-06
**Valid until:** This research cites specific line numbers in files under active development
(`Cargo.toml`, `ci.yml`, `feature-flags.yml`). Line-number citations should be treated as valid for
guidance only and re-verified at the moment each ledger row or ADR is authored — this research
already found one such drift within the same repository history. Treat as valid for ~7 days or until
any commit touches `.github/workflows/ci.yml`, `Cargo.toml`, or `.planning/codebase/STRUCTURE.md`,
whichever comes first.
