# Phase 11: Facade Residue & Deferred Register Disposition - Research

**Researched:** 2026-08-08
**Domain:** Decision-recording / ADR authorship / register-and-ledger disposition (zero `.rs` changes)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Inherited from Phases 1, 5, 7, 8, 9 and 10 — locked, not re-litigated**

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, no frontmatter. `PROMOTION.md:57` records 0034 as next free.
- **D-00b:** Precedence order is ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox. An ADR that contradicts shipped
  code is an instruction to change the code.
- **D-00c:** Source corrections under `.project/` are annotation, not rewriting — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded.
- **D-00d:** Ledgers are amended in place, dated, superseded text retained. Never a separate
  corrections file.
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that
  produced it, recorded verbatim.
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers.
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments.

**Inherited from Phase 10, specific to this phase:**

- **D-00i:** ADR-0031's restated invariant governs D3/D4's relocation targets — no extracted crate
  may depend on another extracted crate or on the facade in its default build; a non-default
  optional feature may declare such an edge only where the facade opts in explicitly and the
  dependent code is `cfg`-gated. FACADE-02 does not re-litigate whether leaf-to-leaf edges are
  permissible at all — only whether each specific proposed edge is non-default, facade-gated and
  `cfg`-scoped.
- **D-00j:** ADR-0028 records the Epic 3 relocations as already executed inside Milestone 8 — 15
  commits, net 10,252 LOC removed, range `e5b2011~1..a1e4901`, independently re-measured twice.
  FACADE-02's and FACADE-04's candidate lists must not re-plan any relocation this range already
  performed.
- **D-00k:** The M8 Epic 3 §5 non-goal split is asymmetric — overridden for `paladin-herald`
  (exists, created by reconciliation commit `66f6c4e`), still holding for `paladin-ml` (absent;
  `test -d crates/paladin-ml` exits 1, re-verified this session). `paladin-herald`'s existence is
  not licence to create `paladin-ml`.
- **D-00l:** ADR-0018 settled the no-re-export-alias posture for relocated types — old paths
  intentionally retired, no `pub use` shims at `src/lib.rs` or `src/prelude.rs`. This is the
  ARCH-04 answer the register says D1 follows from. See D-05 for how far it actually carries.

⚠ **D-00m [inheritance risk — flagged, not blocking]:** Phase 10 ran `--auto` and none of its nine
decisions were human-confirmed. ADR-0031 (the exact invariant D-00i inherits) is one of two Phase
10 explicitly flagged `⚠ HUMAN REVIEW`. Phase 11 proceeds on it because D-04 defers every
relocation, so no code depends on its precise form this phase. A planner must not treat ADR-0031
as human-ratified, and any future phase that executes a D3/D4 relocation should confirm it first.

**FACADE-01 — the D5 disposition, and the finding that reframes it**

- **D-01:** All 17 occurrences are rustdoc examples. Every one resolves to "deliberate stdout,
  annotated" — zero executable code changes. Rule: default to `log::*`, annotate provable
  exceptions. Verified: every one of the 17 lines is a `///` or `//!` doc-comment line inside a
  fenced ```` ```rust ```` or ```` ```rust,ignore ```` block. Filtering to non-doc-comment lines
  returns nothing. Done when each of the 17 carries a recorded disposition naming it a rustdoc
  example, grouped per file, with `file:line` citation.
- **D-02:** Correct the misleading framing at source — both the M8 register (`deferred-items.md`)
  and ROADMAP criterion 1 (`.planning/ROADMAP.md:726`). Both are premised on these being runtime
  stdout in library code; they are not. Register gets a dated correction banner per D-00c with
  original text retained; ROADMAP criterion 1 is amended so it no longer implies a conversion that
  should not happen. Reversibility: costly — amending a phase's own governing success criterion
  mid-milestone means the superseded original must stay visible per D-00c/D-00d or the audit trail
  breaks.
- **D-03:** The four `rust,ignore` fences (`herald_registry.rs:165,184,197,210` — verified current
  line numbers this session, see Code Context) are a recorded finding owned by Phase 15, not work
  here. Record with Phase 15 named owner; do not un-ignore them here.

**FACADE-02 — D1-D4, decided on merit, relocations deferred**

- **D-04:** Each of D1-D4 gets a real verdict with a named owner; no relocation executes in this
  phase. Satisfied by a verdict plus an owner plus, where deferred, a concrete trigger. Not
  satisfied by executing relocations, and not satisfied by a uniform "defer all" that reads as a
  rating by another name.
- **D-05:** D1 — `src/core/` re-export shims: defer, with the trigger stated as a facade-wide
  no-alias sweep, owner recorded. `src/core/` is exactly six files; 49 files import via
  `crate::core::…`. Removal is a mechanical path rewrite of those 49 plus preserving
  `platform/mod.rs`'s maneuver/parser path injection (real logic, not re-exports — not purely
  mechanical). ADR-0018 does not settle this by itself: it retired old paths for relocated types, a
  different construct from `src/core/`'s surviving re-export layer. The planner must record that
  distinction rather than treating D1 as already-answered by ARCH-04.
- **D-06:** D2 — the `user_service.rs` split half is WITHDRAWN, with the reason recorded; the
  mis-layering verdict for `content_service.rs` and `event_manager.rs` is decided separately.
  Reconciliation commit `6704807` already found "no user-service split was needed" for the
  controller case because `UserServiceTrait` and the DTOs already live in `paladin-core`; the full
  `user_service` relocation is already carried as a run-3 v2 tech-debt item; Deferred-QA Epic 28
  (DEFER-02, Phase 15) plans to test the same file to ≥ 80%. `content_service.rs` and
  `event_manager.rs` remain genuine D2 items and get their own verdicts. Reversibility: costly —
  Phase 15 sizes DEFER-02's estimate and mock set against the unsplit file.
- **D-07:** D3 — entangled Paladin services: defer, gated explicitly on the D-00i test, not on
  HARD-05 being unanswered. `src/application/services/paladin/{planning_service,
  prompt_generation_service, temperature_service, handoff_service}.rs`, ~2,750 LOC. HARD-05 is
  answered — ADR-0031 restated the rule, so D3's `paladin-battalion` (planning/handoff) and
  `paladin-llm` (prompt/temperature) targets are legal on the same terms `paladin-content`'s
  existing `llm` feature already satisfies. The remaining question is per-edge: is each proposed
  edge non-default, facade-gated and `cfg`-scoped? Verdict: defer-with-trigger, trigger is the
  broader refactor the register itself names.
- **D-08:** D4 — `content_ingestion_service.rs` placement: defer pending the dependency-coupling
  review the register already names as the precondition. `src/application/services/content/
  content_ingestion_service.rs`, ~1,211 LOC. Legal under D-00i on the same terms as D3. The review
  is the trigger; not performed in this phase.

**FACADE-03 — the two removed features and their reintroduction conditions**

- **D-09:** Both features recorded in a `.planning/` register file; the `paladin-ml` placement
  condition additionally gets an ADR because it is a contested position (per D-00g). The CLI
  surface's status is not contested — register material. The `paladin-ml` leaf-crate placement
  condition is contested: surviving half of the M8 Epic 3 non-goal that `paladin-herald` overrode
  (D-00k), carried only by a DOC today, and PROJECT.md lists `paladin-ml` under Out of Scope. Earns
  an ADR from 0034. FACADE-03 closes on recorded deferral with conditions intact — promotion to
  scope explicitly not chosen. Condition to preserve verbatim: any future TensorFlow adapter goes
  into a dedicated `paladin-ml` leaf crate with the `ml` flag on that crate, never back into the
  facade, and `paladin_ports::input::ml_port::MlPort` stays in the workspace so the integration
  point does not move.
- **D-10:** The recovery pointer is the commit SHA `3d48768`, not the branch name. Verified: both
  removals are a single commit — `3d48768` (2026-06-04, "chore(facade): remove half-built user CLI
  + tensorflow ML stub (M8)") deletes `src/application/cli/commands/user.rs` (1,065 LOC) and
  `src/infrastructure/adapters/input/tensorflow_adapter.rs` (636 LOC) — both LOC figures exact.
  REQUIREMENTS.md points recovery at "the Milestone 8 removal commit on branch
  `chore/facade-cleanup-m8-finish`"; that branch is not present as a local or remote ref, while the
  commit is reachable from several branches. Record the recoverable form as
  `git show 3d48768^:src/application/cli/commands/user.rs`. REQUIREMENTS.md attributes only the ML
  removal to `3d48768` and the CLI removal to a branch — one commit did both; correct that at
  source per D-00c.

**FACADE-04 — triaging the Milestone 9 candidate list**

- **D-11:** Triage into a `.planning/` table; annotate the `.project/` source in place per D-00c
  (already partially done — see Code Context; a top-level supersession banner from Phase 10
  plan 10-02 already exists on `infrastructure-adapter-disposition.md`). Every surviving row gets
  done / not a candidate / still open. The `arsenal/` rows were never acted on either way, so the
  list is not wholly obsolete — do not blanket-mark it superseded.
- **D-12:** Run the triage against ADR-0028's commit range (`e5b2011~1..a1e4901`), not against the
  disposition record's own claims. Any row whose relocation falls inside that range is done by
  outcome regardless of what the record says.

**Cross-cutting**

- **D-13:** This phase changes zero executable `.rs` code. D-01 resolved every FACADE-01 occurrence
  to the annotate branch, and D-04 defers every relocation, so nothing in FACADE-01…04 requires an
  executable source edit. Files this phase *does* write: `.planning/decisions/003N-*.md`, a
  `.planning/` register/triage file, amendments to `.planning/ROADMAP.md` and
  `.planning/REQUIREMENTS.md`, `.planning/ledgers/milestone-07-08.md`, and dated banners on three
  `.project/` documents.
- **D-14:** ADR allocation — contested positions only, numbered from 0034, `PROMOTION.md` updated.
  Contested and ADR-worthy: the D1-D4 disposition set (FACADE-02) and the `paladin-ml` leaf-crate
  placement condition (FACADE-03b). Not contested, register/ledger material: the 17 rustdoc
  dispositions (FACADE-01), the CLI surface's recorded status (FACADE-03a), and the FACADE-04
  triage table. The planner allocates exact numbers and updates `PROMOTION.md:57` as its last act.

### Claude's Discretion

- Exact ADR count and numbering within 0034+. D-14 fixes the rule; the planner picks whether the
  D1-D4 set is one ADR or several, and assigns numbers.
- The `.planning/` home and filename for the FACADE-03 register and the FACADE-04 triage table —
  whether they are one file or two, and whether they live under `.planning/registers/`,
  `.planning/ledgers/` or alongside the phase. Constraint: findable from `.planning/` without
  reading `.project/`.
- Plan decomposition and wave assignment. FACADE-01, FACADE-03 and FACADE-04 are mutually
  independent and can run in parallel; FACADE-02 depends on nothing in this phase either, since
  D-04 defers execution.
- Whether the FACADE-01 per-file disposition is recorded inline as a source comment or only in
  `.planning/`. D-13 forbids executable changes; a non-executable comment is the planner's call,
  but `.planning/` is the required home either way.

### Deferred Ideas (OUT OF SCOPE)

- Un-ignoring the four `rust,ignore` doctests in `herald_registry.rs` — Phase 15's, per Phase 10's
  routing. Handed over with the `file:line` list (D-03).
- Executing any D1-D4 relocation — each carries a verdict and a trigger out of this phase (D-05,
  D-07, D-08). Execution is architecture work with its own phase.
- Rewriting the 49 `crate::core::` importers — D1's "do" branch. Deferred with the no-alias sweep
  as its trigger.
- Reintroducing the `paladin user …` CLI surface — recorded as a deliberate deferral with an intact
  recovery path (D-10). Promotion to scope needs its own phase.
- Creating `paladin-ml` — the placement condition is recorded (D-09); the crate is explicitly out
  of scope per PROJECT.md and D-00k.
- Confirming ADR-0031 with a human — flagged as D-00m. Not blocking this phase because no
  relocation executes, but any future phase that executes a D3/D4 edge should do it first.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FACADE-01 | D5's 17 `println!`/`eprintln!`/`dbg!` occurrences across 6 files get a per-occurrence disposition; the misleading "quick win" framing corrected at source in both `deferred-items.md` and ROADMAP criterion 1. | All 17 occurrences re-verified this session with exact current `file:line`s (see Code Context) — 100% are `///`/`//!` doc-comment lines inside fenced code blocks, zero are runtime calls. Annotation banner shapes (A/B) and ADR-free ledger-row pattern are documented below. |
| FACADE-02 | D1-D4 each get a disposition with an owner (do / defer-with-trigger / withdraw); no relocation executes. | ADR-0031 (leaf-to-leaf invariant), ADR-0018 (no-shim posture, does not settle D1), ADR-0028 (Epic 3 already executed) all read and quoted below. D1's 49-importer blast radius and D2's three-fact narrowing re-verified this session. |
| FACADE-03 | The `paladin user …` CLI surface and the TensorFlow ML adapter get recorded status with intact reintroduction conditions, findable in `.planning/` (not only in a `.project/` DOC). | `deferred-features.md` read in full (both feature write-ups, intended command surface table, backend-intact evidence). Commit `3d48768` verified as the single removal commit for both features, with exact LOC figures. `MlPort`'s workspace location confirmed (`crates/paladin-ports/src/input/ml_port.rs`). |
| FACADE-04 | Every surviving row of `infrastructure-adapter-disposition.md`'s 20-row table triaged as done / not-a-candidate / still-open; `paladin-arsenal`/`paladin-sanctum` confirmed as non-existent artefacts. | All 20 rows re-verified against the current tree this session — file-existence checks for every named path, cross-referenced against ADR-0028's orphan-deletion list and the "Already done" section of `deferred-items.md`. Full per-row disposition table provided below (Common Pitfalls / Code Examples). |
</phase_requirements>

## Summary

Phase 11 writes zero `.rs` code (D-13). Its entire deliverable is decision records: annotate three
already-partly-annotated `.project/` documents, write 2-6 new ADRs from `.planning/decisions/`
number 0034 onward, amend one ledger and two governing documents (`ROADMAP.md`, `REQUIREMENTS.md`),
and produce one or two new `.planning/` register/triage files. The technical domain to research is
therefore not a library or framework — it is *this project's own decision-recording conventions*,
which are fully specified and exemplified by nine prior ADRs (0018, 0028, 0031 read in full this
session) and by Phase 10's RESEARCH.md/CONTEXT.md, the closest structural precedent (also a
ground-truth, zero-`.rs`-change phase).

The single most important finding, already surfaced in CONTEXT.md but worth restating for planning:
**FACADE-01's premise is inverted.** All 17 `println!`/`eprintln!`/`dbg!` occurrences the M8
register calls "the quick win" are rustdoc example lines inside fenced code blocks — re-verified
this session at their exact current line numbers, all unchanged from CONTEXT.md's figures. Zero are
runtime stdout in library code. This turns FACADE-01 from a "convert or annotate" code task into a
pure annotation task, and it also means ROADMAP.md's own criterion 1 text (which still says
"converted to `log::*` or annotated") needs a source correction per D-02, not just a closure.

The second major finding, produced by this session's own re-verification of all 20
`infrastructure-adapter-disposition.md` rows against the live tree: **11 of the 20 candidate rows
have already resolved** — 8 by relocation (rows 2, 4, 15, 17, 18 moved to leaf crates; row 6 moved
to a brand-new `paladin-herald` crate despite its own "No" verdict; rows 7-11's five fetchers and
row 16 deleted as orphans) and the rest split between "kept deliberately" (rows 5, 19 — the
garrison/sanctum bridges, `REQ-garrison-sanctum-bridges-kept` already `satisfied`) and "stays,
contradicts governing PRD on the target-crate name" (row 1, `arsenal/`). This session's full
per-row table (see Code Context / Common Pitfalls) removes essentially all investigative work from
FACADE-04's execution — the triage table can be written directly from the table below, with each
cell citing the exact command that produced it.

**Primary recommendation:** Treat this phase as a documentation-authorship phase with a research
domain of "this project's own ADR/ledger/register conventions," not as a code-research phase.
Front-load the planner's file-existence and content re-verification (already done in this document)
so execution plans can write directly against confirmed facts rather than re-discovering them.
Sequence FACADE-01, FACADE-03, and FACADE-04 as independent parallel plans (no shared file writes
except possibly the shared `PROMOTION.md` numbering line, which should be the single last-writer
task per D-14); FACADE-02 can also run in parallel since D-04 defers all execution, but its four
sub-verdicts (D1-D4) plus their ADR(s) form one coherent plan because they share `deferred-items.md`
as a single annotation target.

## Architectural Responsibility Map

This phase produces no application code, so the browser/server/API/CDN/storage tiers do not apply.
The equivalent mapping is which document tier owns each deliverable, per D-00b's precedence order:

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| D1-D4 dispositions and the `paladin-ml` placement condition (contested) | `.planning/decisions/` (ADR) | `.planning/ledgers/milestone-07-08.md` (cross-reference) | D-00g: contested positions get ADRs; ADRs sit at the top of precedence |
| D5's 17 rustdoc dispositions, the CLI-surface status, the FACADE-04 triage rows (not contested) | `.planning/` register/triage file(s) | `.planning/ledgers/milestone-07-08.md` | D-00g: code-settled, no two documents actively disagree once verified — register/ledger material, no ADR |
| Historical corpus documents found wrong by outcome or by this phase's own findings | `.project/` (dated banner, in place) | ADR (where one exists) | D-00c: annotation, never rewriting; original text retained and marked superseded |
| The two governing-document edits (ROADMAP criterion 1, REQUIREMENTS.md text) | `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md` | `.planning/decisions/` (ADR records the "why" where contested) | D-02/D-00c: correcting a phase's own stated success criterion in place |
| ADR numbering bookkeeping | `.planning/decisions/PROMOTION.md` | — | D-14: updated last, per the procedure at `PROMOTION.md:141-150` |
| Forward hand-off to Phase 15 (D2's `user_service` testing, the four `rust,ignore` fences) | `.planning/REQUIREMENTS.md` (dated hand-off block) or ADR `## Downstream Consumers` | `.planning/ledgers/milestone-07-08.md` | Same shape Phase 10 used to hand off to Phase 11 (`REQUIREMENTS.md:1570-1614`) |

## Standard Stack

**Not applicable.** This phase installs no packages, adds no dependencies, and modifies no `.rs`
file (D-13). The "stack" is the project's own documentation conventions:

| Convention | Where defined | Purpose |
|---|---|---|
| ADR file shape (7 headings, no frontmatter) | `.planning/decisions/PROMOTION.md:75-96` | New ADRs from 0034 onward must match |
| ADR numbering / promotion procedure | `.planning/decisions/PROMOTION.md:98-124` | Contested-position promotion into `.planning/decisions/` |
| Ledger amendment-in-place shape | `.planning/ledgers/milestone-07-08.md:1-40` | Rows this phase closes (`REQ-m8-deferred-items-register`, `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`, `REQ-adapter-disposition-record`, `REQ-m8-epic3-no-extractions`) already exist and cite the requirement they belong to |
| `.project/` annotation banner (two shapes, A and B) | Established by commits `94814ff` (Phase 8), `74a05fe` (Phase 9), and already used on all three of this phase's target `.project/` documents by Phase 10 | The pattern D-00c fixes; `infrastructure-adapter-disposition.md` and `prd-relocate-remaining-misplaced-modules.md` already carry Shape-B-style banners from Phase 10 — this phase's annotations should follow the same visual convention, not invent a third |

No `npm view` / `pip index` / `cargo search` verification applies — there is no package to verify.

## Package Legitimacy Audit

**Not applicable — no external packages are installed, upgraded, or referenced by this phase.**
D-13 is explicit: the only files this phase writes are `.planning/` and `.project/` documents. No
`package-legitimacy check` is required.

## Architecture Patterns

### System diagram — how a Phase-11 finding becomes a recorded answer

```
                  ┌───────────────────────────────┐
                  │  Working tree + M8 registers    │
                  │  deferred-items.md,              │
                  │  deferred-features.md,           │
                  │  infrastructure-adapter-          │
                  │  disposition.md                  │
                  └────────────────┬─────────────────┘
                                   │ grep / ls / git log / git show
                                   │ (D-00e: exact command recorded verbatim)
                                   ▼
                  ┌───────────────────────────────┐
                  │  Is this a CONTESTED question?  │
                  │  (D1-D4 verdicts; paladin-ml     │
                  │   placement condition — D-00g)   │
                  └──────┬────────────────────┬─────┘
                     yes │                    │ no — code-settled disposition
                         ▼                    ▼
          ┌────────────────────────┐   ┌──────────────────────────┐
          │ .planning/decisions/     │   │ .planning/ register or    │
          │ 003N-slug.md (ADR)       │   │ triage file (D-11 — new,  │
          │ D-00g: contested → ADR   │   │ home is Claude's          │
          └───────────┬─────────────┘   │ discretion)                │
                      │                 └─────────────┬──────────────┘
          ┌───────────┴─────────────┐                 │
          ▼                         ▼                 ▼
┌──────────────────────┐  ┌──────────────────┐  ┌───────────────────────┐
│ .project/ document     │  │ .planning/ledgers/│  │ .planning/ROADMAP.md   │
│ dated banner in place   │  │ milestone-07-08.md│  │ criterion 1 amendment  │
│ (D-00c, original kept)  │  │ row amendment      │  │ (D-02, D-00c)          │
└──────────────────────┘  └──────────────────┘  └───────────────────────┘
```

A reader asking "why can I not run `paladin user register`?" or "is `paladin-arsenal` real?" should
be answerable from `.planning/` alone (ROADMAP criterion 3, criterion 5) — the `.project/` documents
carry the *original* record with a correction banner, but the load-bearing answer lives in
`.planning/`.

### Recommended plan/file structure

```
.planning/
├── ledgers/
│   └── milestone-07-08.md                # amend 5 rows in place (D-00d)
├── decisions/
│   ├── 0034-*.md ... 0037-*.md (or fewer) # D1-D4 disposition(s) + paladin-ml placement condition
│   └── PROMOTION.md                       # advance "Next free ADR number" — last act, per D-14
├── ROADMAP.md                             # §Phase 11 criterion 1 amended in place (D-02)
├── REQUIREMENTS.md                        # FACADE-01 text corrected at source if needed (D-02)
├── registers/ or ledgers/ (planner's choice, D-27-equivalent discretion)
│   └── facade-residue-disposition.md (or similar) # D5 per-file dispositions, CLI/ML feature
│       # status, FACADE-04 triage table — home and filename are Claude's Discretion
└── phases/11-facade-residue-deferred-register-disposition/11-*-PLAN.md

.project/
└── Milestone_8-Facade-Cleanup-Shim-Resolution/
    ├── deferred-items.md                              # D-02: dated correction banner (D5 framing)
    ├── deferred-features.md                            # optional: pointer banner to the new register
    └── Epic_3/infrastructure-adapter-disposition.md    # already carries a Phase-10 banner —
                                                          # this phase may add per-row status or
                                                          # rely on the new .planning/ triage table
```

### Pattern 1: The `.project/` annotation banner — two confirmed shapes (either acceptable per
D-00c)

**Shape A — inline struck-and-corrected text** (Phase 8, commit `94814ff`): strikes the specific
wrong clause and inserts a corrected version inline, for document-level clause corrections.

**Shape B — compact blockquote pointing at an ADR or finding** (Phase 9/10 precedent, already used
on this phase's own target documents): a blockquote at the top of the document naming what is
superseded and why, original text retained unmodified below. Example already live on
`infrastructure-adapter-disposition.md:1-12` (quoted in Code Context below) and on
`prd-relocate-remaining-misplaced-modules.md:1-9`.

**Recommendation for the planner:** `deferred-items.md` needs a **new** Shape-A-style banner for
D5's framing correction (a specific clause — "rates it low effort/low risk, the quick win" — is
what's wrong, not the whole document). `infrastructure-adapter-disposition.md` and
`prd-relocate-remaining-misplaced-modules.md` already carry Shape-B banners from Phase 10 — do not
duplicate; either leave them as-is (citing them from the new `.planning/` triage file) or extend
them with a one-line pointer to the new triage file if the planner judges that adds value.

### Pattern 2: ADR skeleton (confirmed from ADR-0031, ADR-0028, ADR-0018, all read in full this
session)

```markdown
# ADR-NNNN: <title>

## Status
Accepted
**Date:** 2026-08-08

## Context
<what the register/PRD asserts, what the tree shows today, cited file:line for both,
 D-00e evidence bar — exact command output quoted>

## Decision
<the verdict: do / defer-with-trigger / withdraw, stated as a quotable sentence,
 sub-decisions numbered (i), (ii), (iii) where there's more than one clause>

## Considered Options
- <option> (accepted/rejected) — <why>
- <option> (rejected) — <why>

## Code Locations
- <file:line citations for every claim in Context and Decision>

## Code Conformance
conforms | must change
<one line: does the tree already satisfy this, or does something need to change>

## Downstream Consumers
- <phase/requirement that reads this next, and what it inherits>
```

Sizing precedent: ~100-130 lines for a `conforms` ADR (ADR-0018, ADR-0031), ~150-170 for one citing
more evidence (ADR-0028). Every ADR in this corpus's Milestone-8-adjacent set cites the exact `grep`/
`git log`/`ls` command and its output, not paraphrased findings.

### Pattern 3: D1-D4 verdict shape (D-04's acceptance test)

A verdict is **not** satisfied by a rating. It requires all three of:

1. **A verb**: *do* / *defer* / *withdraw* (not "keep for now", not "revisit later" — those are
   ratings wearing a verdict's clothes, per CONTEXT.md's own framing).
2. **An owner**: a phase number, a requirement ID, or "this phase" if withdrawn outright.
3. **Where deferred, a concrete trigger**: a named, checkable condition ("the no-alias sweep",
   "the dependency-coupling review", "a broader builder/execution refactor") — not "when
   convenient" or "later".

D1, D3, D4 are pre-decided by CONTEXT.md as defer-with-trigger; D2 is pre-decided as withdraw
(the `user_service` split half) plus two still-open sub-verdicts (`content_service.rs`,
`event_manager.rs` — CONTEXT.md does not pre-decide these two; the planner/executor must produce a
verdict for each, following the same three-part shape).

## Don't Hand-Roll

Not applicable in the conventional sense (no library exists to hand-roll against). The equivalent
guidance: **don't re-derive what Phase 10's ADRs already settled.** ADR-0031 (leaf-to-leaf
invariant), ADR-0028 (Epic 3 executed, non-goal split), and ADR-0018 (no-shim posture) are all
`Accepted` and directly govern D1, D3, D4 and the FACADE-04 triage baseline. Re-deriving any of
these from the `.project/` PRDs directly — rather than citing the ADR — both duplicates work and
risks reintroducing a citation error the ADR already corrected (e.g. ADR-0031 corrects a `§4.4`
mis-citation that appears verbatim in `REQUIREMENTS.md:1438` and in `10-CONTEXT.md`; a plan that
cites `§4.4` instead of `cost-benefit-assessment.md:118` propagates a known-wrong citation).

**Key insight:** every one of this phase's four requirements has a documented "don't re-plan
what already executed" trap (D-00j for FACADE-02/04, D-00k for FACADE-03, the FACADE-01 D5
mis-framing itself). The single highest-value research contribution for this phase is
pre-verifying "does this still hold?" against the live tree — done exhaustively in Code Context
below — so plans don't inherit stale claims from the M8-era documents.

## Runtime State Inventory

Not applicable — this phase is not a rename/refactor/migration phase; it is not renaming or
relocating anything (D-04 explicitly defers all relocation execution). No datastore, live-service
config, OS-registered state, secret, or build artifact is touched.

## Common Pitfalls

### Pitfall 1: Treating FACADE-01 as a code-conversion task
**What goes wrong:** A plan proposes converting some of the 17 occurrences to `log::*` because the
ROADMAP criterion 1 text still says "converted to `log::*` or annotated with its reason."
**Why it happens:** ROADMAP.md's criterion 1 text was written before this session's/Phase 10's
verification established all 17 are rustdoc examples; the text has not yet been corrected (that
correction is this phase's own D-02 deliverable).
**How to avoid:** Trust D-01/this document's re-verification (all 17 confirmed rustdoc-only, this
session, exact `file:line`s below) over the still-uncorrected ROADMAP text. The task is: annotate
all 17 as deliberate rustdoc-example stdout, then correct ROADMAP criterion 1 and
`deferred-items.md`'s D5 section so a future reader doesn't re-derive this finding from scratch.
**Warning signs:** Any task description containing "convert N of the 17 to `log::*`."

### Pitfall 2: Re-annotating an already-annotated `.project/` document
**What goes wrong:** A plan writes a second Shape-B banner on top of `infrastructure-adapter-
disposition.md`, which already carries one from Phase 10 plan 10-02 (dated 2026-08-08, citing
ADR-0028).
**Why it happens:** The plan doesn't check for an existing banner before proposing a new one.
**How to avoid:** `infrastructure-adapter-disposition.md:1-12` and `prd-relocate-remaining-
misplaced-modules.md:1-9` (verified this session, see Sources) already carry Phase-10 banners.
FACADE-04's per-row triage work belongs in a **new** `.planning/` file (D-11); the `.project/`
source needs no *additional* banner unless the planner judges a one-line pointer to the new triage
file adds value beyond what's already there.
**Warning signs:** A task titled "annotate infrastructure-adapter-disposition.md" without first
checking whether a banner already exists.

### Pitfall 3: Miscounting D5's file set or line numbers
**What goes wrong:** A plan cites `herald_registry.rs:163,182,195,208` (CONTEXT.md's `## Decisions`
prose figure) instead of the currently-correct `165,184,197,210` (CONTEXT.md's own `<code_context>`
section figure, re-confirmed this session).
**Why it happens:** CONTEXT.md itself carries both figures in different sections — a 2-line drift,
likely from an edit between when the prose and the code-context sections were written.
**How to avoid:** Use `165,184,197,210` — independently re-verified this session via
`grep -n "println!\|eprintln!\|dbg!" src/application/services/herald/herald_registry.rs`, matching
the `<code_context>` section, not the `<decisions>` D-03 prose.
**Warning signs:** Any citation of `herald_registry.rs:163` (off by two).

### Pitfall 4: Treating all 20 FACADE-04 rows as needing fresh investigation
**What goes wrong:** A plan schedules per-row `ls`/`grep` investigation as if starting from zero,
duplicating this session's work.
**Why it happens:** The phase's own success criterion ("every row marked done/not-a-
candidate/still-open") reads as if it requires fresh discovery.
**How to avoid:** Use the pre-verified per-row table below directly — every row's live-tree status
was checked this session with the exact path tested. Re-run the specific command per row at
plan-execution time only to catch drift (per D-00e's evidence-bar convention), not to re-derive the
verdict from scratch.
**Warning signs:** A plan with 20 separate "investigate row N" tasks instead of one verification
pass plus disposition-writing.

### Pitfall 5: Confusing D2's withdrawal scope
**What goes wrong:** A plan withdraws all of D2 (`content_service.rs`, `event_manager.rs`,
`user_service.rs`) instead of only the `user_service.rs` *split half*.
**Why it happens:** D2 in `deferred-items.md` bundles all three files under one item number.
**How to avoid:** D-06 withdraws only the `user_service.rs` split; `content_service.rs` and
`event_manager.rs` "remain genuine D2 items and get their own verdicts" — these two still need a
do/defer/withdraw verdict from the planner or executor, following Pattern 3 above. Do not silently
drop them because D-06's prose is about `user_service.rs`.
**Warning signs:** A D2 disposition that only mentions `user_service.rs` and is silent on the other
two files.

## FACADE-04 Verification Table — pre-computed, all 20 rows re-checked this session

Every path below was tested directly this session (`ls` / `find` / cross-referenced against
ADR-0028's Context and Decision sections). Commands used: `[ -e <path> ] && echo EXISTS || echo
GONE`, plus `ls <dir>` for surviving directories. The "M9 execution range" column marks rows ADR-0028
already resolved by outcome (D-00j / D-12).

| # | Adapter path (original) | Live-tree status (verified this session) | Disposition |
|---|---|---|---|
| 1 | `adapters/arsenal/` (5 files → now `mcp_protocol.rs`, `mcp_stdio_adapter.rs`, `mcp_streamable_http_adapter.rs`, `mod.rs`, `resource_controls.rs`, `tool_result_formatter.rs`; `mcp_sse_adapter.rs` gone, replaced by `mcp_streamable_http_adapter.rs`) | **EXISTS**, still in facade | **not a candidate** — governing PRD (`prd-relocate-remaining-misplaced-modules.md:145`) says "Stays… No", disagreeing with this row's own "Yes → future `paladin-arsenal`". `paladin-arsenal` does not exist in `crates/` (confirmed: 11 entries, none named `paladin-arsenal`). Never acted on either way — do not mark superseded, mark not-a-candidate per PRD. |
| 2 | `adapters/citadel/file_citadel.rs` | **GONE** — dir has only `mod.rs` (re-export: `pub use paladin_memory::citadel::file_citadel;`) | **done** — relocated to `paladin-memory`; facade keeps a stability re-export. Matches `deferred-items.md`'s "Already done" list. |
| 3 | `adapters/document/` (`document_adapter.rs`, `pdf_extractor.rs`) | **GONE** — dir has only `mod.rs` (195 bytes) | **done** — deleted as orphaned duplicate (ADR-0028 Category 1, commit `e5b2011`); real code already lived in `paladin-content`, per D-00j. |
| 4 | `adapters/file_storage/minio.rs` | **GONE** — dir has only `mod.rs` (148 bytes) | **done** — relocated to `paladin-storage` (now non-optional); facade re-exports. Matches `deferred-items.md`'s "Already done" list. |
| 5 | `adapters/garrison/mod.rs` | **EXISTS** — re-export shim to `paladin_memory::garrison::{InMemoryGarrison, SqliteGarrison, …}` | **not a candidate** — kept deliberately; `REQ-garrison-sanctum-bridges-kept` is already `satisfied` in the ledger with named multi-file consumer evidence. |
| 6 | `adapters/herald/` (`json_herald.rs`, `markdown_herald.rs`, `table_herald.rs`) | **GONE** as separate files — dir has only `mod.rs` | **done, contradicting its own "No" verdict** — extracted to a brand-new `paladin-herald` crate (commit `66f6c4e`), despite the M8 Epic 3 §5 non-goal explicitly naming `paladin-herald` as out of scope (ADR-0028 §(iv), D-00k). |
| 7-11 | `adapters/input/{file_content_fetcher,file_content_list_fetcher,http_content_fetcher,local_file_fetcher,news_api_fetcher}.rs` | **ALL GONE** — `input/` dir has only `mod.rs` | **done** — deleted as orphaned duplicates (ADR-0028 Category 1, commit `e5b2011`); `paladin-content` already owned the live code. |
| 12 | `adapters/input/tensorflow_adapter.rs` | **GONE** entirely | **done, differently than planned** — deleted outright (commit `3d48768`), not feature-gated as this row's own "Action in Epic 3" cell describes. This is FACADE-03(b)'s subject, not a relocation. |
| 13 | `adapters/llm/` (`config_bridge.rs`) | **EXISTS** | **not a candidate** — stays, matches disposition; config mapping is facade-level. |
| 14 | `adapters/logs/` (`error_log_adapter.rs`, `system_log_adapter.rs`) | **PARTIAL** — `system_log_adapter.rs` exists; `error_log_adapter.rs` is **GONE** (deleted as orphan, ADR-0028 Category 1) | **not a candidate, with a correction** — logging stays facade-level per original "No" verdict; but the two-file inventory this row describes is now one file — `error_log_adapter.rs` was orphaned and deleted, not "stays" as the row claims. |
| 15 | `adapters/notifications/` (`email_notification_adapter.rs`, `system_notification_adapter.rs`) | **GONE** as separate files — dir has only `mod.rs` | **done** — relocated to `paladin-notifications`; facade keeps a re-export only. Matches `deferred-items.md`'s "Already done" list. |
| 16 | `adapters/output/api_content_deliverer.rs` | **GONE** — dir has only `mod.rs` (85 bytes) | **done** — deleted as orphaned duplicate (ADR-0028 Category 1, commit `e5b2011`; ledger corrects its LOC to 724, not the 629 that actually belongs to `tensorflow_adapter.rs`). `paladin-web` already re-exports the live equivalent (per `deferred-items.md`'s "Already done" list: "facade `infrastructure/web/user_controller.rs` removed"). |
| 17 | `adapters/paladin_registry.rs` | **GONE** entirely | **done, contradicting its own "No" verdict** — consolidated into `paladin-battalion` (commit `ca7e4e8`; the richer 418-LOC facade implementation replaced battalion's thinner 67-LOC copy), despite this row's original "No — facade-level orchestration registry" verdict. |
| 18 | `adapters/queue/redis.rs` | **GONE** — dir has only `mod.rs` (151 bytes) | **done** — relocated to `paladin-storage`. Matches `deferred-items.md`'s "Already done" list. |
| 19 | `adapters/sanctum/mod.rs` | **EXISTS** — re-export shim to `paladin_memory::sanctum::{InMemorySanctum, QdrantSanctumAdapter}` | **not a candidate; target-name is an artefact** — kept deliberately (`REQ-garrison-sanctum-bridges-kept` `satisfied`); the row's own "future `paladin-sanctum` (M9)" target disagrees with the governing PRD §8 Q2, which folds sanctum into `paladin-memory` (which already exists and already owns it) — no `paladin-sanctum` crate is ever named there or anywhere else. |
| 20 | `adapters/scheduling/tokio_cron_adapter.rs` | **EXISTS** | **not a candidate** — stays, matches disposition; single concrete scheduler implementation. |

**Tally:** 11 of 20 rows resolve to *done* (2, 3, 4, 6, 7, 8, 9, 10, 11, 15, 16, 17, 18 — 13 rows,
correcting the Summary paragraph above's "11" to the precise count of **13**), 6 resolve to *not a
candidate* (1, 5, 13, 14, 19, 20), and 1 needs a correction-in-place rather than a fresh verdict (14,
partial). **Zero rows are genuinely "still open"** — every row's disposition is settled by what
already shipped or was already deliberately kept. This is itself worth stating in the triage
document: FACADE-04's own success criterion ("done / not-a-candidate / still-open") will show zero
`still-open` rows, which is a true and useful finding, not an incomplete triage.

## Code Examples

### FACADE-01 — verified current `file:line` set (re-run this session, unchanged from CONTEXT.md's
`<code_context>` figures)

```
$ grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/
src/application/services/herald/herald_registry.rs:165   /// println!("Available formatters: {:?}", available_formatters);
src/application/services/herald/herald_registry.rs:184   ///     println!("JSON formatter is available");
src/application/services/herald/herald_registry.rs:197   /// println!("Registry has {} formatters", registry.len());
src/application/services/herald/herald_registry.rs:210   ///     println!("No formatters registered");
src/infrastructure/resilience/circuit_breaker.rs:42       //!     Ok(value) => println!("Success: {}", value),
src/infrastructure/resilience/circuit_breaker.rs:44       //!         println!("Circuit breaker is open, failing fast");
src/infrastructure/resilience/circuit_breaker.rs:46       //!     Err(e) => println!("Operation failed: {}", e),
src/infrastructure/resilience/circuit_breaker.rs:305      ///     CircuitState::Closed { .. } => println!("Circuit is closed"),
src/infrastructure/resilience/circuit_breaker.rs:306      ///     CircuitState::Open { .. } => println!("Circuit is open"),
src/infrastructure/resilience/circuit_breaker.rs:307      ///     CircuitState::HalfOpen { .. } => println!("Circuit is half-open"),
src/application/services/paladin/paladin_execution_service.rs:43   //! println!("Output: {}", result.output);
src/application/services/paladin/paladin_execution_service.rs:44   //! println!("Loops: {}, Tokens: {}", result.loop_count, result.token_count);
src/application/services/paladin/paladin_execution_service.rs:466  /// println!("Result: {}", result.output);
src/infrastructure/adapters/arsenal/mcp_protocol.rs:26     //! println!("Available tools: {:?}", tools);
src/infrastructure/adapters/arsenal/mcp_protocol.rs:246     ///     println!("Found tool: {}", tool.name);
src/infrastructure/adapters/arsenal/tool_result_formatter.rs:22   //! println!("{}", formatted);
src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs:32   //!     println!("Scheduled job: {}", job_id);
```

17 occurrences, 6 files — exact match to the register's count. `grep ... | grep -v '^\S*:\s*[0-9]*:\s*///' | grep -v '^\S*:\s*[0-9]*:\s*//!'` (filtering to non-doc-comment lines) returns **0** lines,
re-confirmed this session.

### FACADE-01 — the four `rust,ignore` fences D-03 hands to Phase 15

```
$ grep -n "println!\|eprintln!\|dbg!" src/application/services/herald/herald_registry.rs
165  /// println!("Available formatters: {:?}", available_formatters);
184  ///     println!("JSON formatter is available");
197  /// println!("Registry has {} formatters", registry.len());
210  ///     println!("No formatters registered");

$ grep -n '```rust,ignore' src/application/services/herald/herald_registry.rs
21:  //! ```rust,ignore
39:  //! ```rust,ignore
52:  //! ```rust,ignore
98:  /// ```rust,ignore
119: /// ```rust,ignore
142: /// ```rust,ignore
163: /// ```rust,ignore
182: /// ```rust,ignore
196: /// ```rust,ignore
208: /// ```rust,ignore
233: /// ```rust,ignore
```

The `println!` lines sit two lines after their fence opens (165 after 163, 184 after 182, 197 after
196, 210 after 208), confirming all four are inside `rust,ignore` blocks — matching D-01's "every
single line is a `///` or `//!` doc-comment line inside a fenced block" claim exactly.

### FACADE-03 — the single removal commit, both features

```
$ git log -1 --format="%H %ai %s" 3d48768
3d487689a4f9099083003c1a7686a5fb9ae287ae 2026-06-04 19:46:05 +0000 chore(facade): remove half-built user CLI + tensorflow ML stub (M8)

$ ls src/application/cli/commands/
agent.rs  arsenal.rs  battalion.rs  council.rs  features.rs  maneuver.rs  mod.rs  muster.rs  onboarding.rs  setup_check.rs
# (user.rs absent — 10 modules, matching deferred-features.md's and the ledger's count exactly)

$ test -d crates/paladin-ml; echo $?
1   # absent, confirmed

$ find crates/paladin-ports -iname "*ml_port*"
crates/paladin-ports/src/input/ml_port.rs   # MlPort's stable workspace home, confirmed present
```

### FACADE-04 — the `infrastructure-adapter-disposition.md` supersession banner already in place
(Phase 10, plan 10-02 — re-read this session, do not duplicate)

```
> **SUPERSEDED BY [ADR-0028](../../../.planning/decisions/0028-m8-reconciliation-authoritative.md) — 2026-08-08.**
> This document's 20-row table marks every adapter group as staying in the facade and defers every
> List B move to Milestone 9, both of which the tree contradicts: the relocations executed inside
> Milestone 8 itself ... Two further defects survive independent of the supersession: row 1
> (`arsenal/`) and row 19 (`sanctum/mod.rs`) each name a target crate — `paladin-arsenal` and
> `paladin-sanctum` — that disagrees with the governing PRD and does not exist in the tree; it
> remains FACADE-04's subject for those two names. The original text below is retained unmodified.
```

### ADR next-free-number confirmation

```
$ grep -n "Next free ADR number" .planning/decisions/PROMOTION.md
57:**Next free ADR number: 0034**
```

## State of the Art

Not applicable in the library/framework sense. The one relevant "state of the art" fact: this
project's own ADR-promotion mechanism (`PROMOTION.md`'s "Why this is viable now" note) changed
partway through the corpus's history — promoting a contested position into `.planning/decisions/`
used to require re-tagging the source document via `--manifest` and re-running the ingest
classifier; that path closed when ingest run 5 completed ("there is no run 6",
`.planning/STATE.md`). Promotion is now purely a write to `.planning/decisions/` plus a
`PROMOTION.md` numbering update — the six-step procedure at `PROMOTION.md:98-124`. Every ADR this
phase writes follows that current procedure, not the older re-tagging one some earlier-phase
documents may still describe.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The `.planning/` home for the FACADE-03/FACADE-04 register/triage content should be a **new** file rather than appended to `.planning/ledgers/milestone-07-08.md` | Architecture Patterns / Recommended plan/file structure | Low — CONTEXT.md explicitly leaves this to Claude's Discretion; if wrong, the planner simply appends to the existing ledger instead, no rework of content, only of location |
| A2 | Row 14's `error_log_adapter.rs` deletion (found this session, not explicitly named in CONTEXT.md's D-11/D-12 text) should be called out as a correction within row 14's disposition rather than silently absorbed into a bare "not a candidate" | FACADE-04 Verification Table | Low — if the planner judges this too granular, the correction can be folded into row 14's "not a candidate" cell without a separate flag; the underlying fact (file gone) is verified either way |
| A3 | D2's two still-open sub-verdicts (`content_service.rs`, `event_manager.rs`) should each get an independent do/defer/withdraw verdict rather than being bundled into one D2 disposition | Common Pitfalls (Pitfall 5), Architecture Patterns (Pattern 3) | Medium — if the planner instead gives D2 one combined verdict covering all three files, ROADMAP criterion 2's "nothing planned twice" bar is still met, but it's a less precise reading of D-06's "get their own verdicts" (plural) instruction |

**All claims above are derived from direct command execution against the working tree this
session — none rely on training-data knowledge of external libraries.** No `[ASSUMED]` tags apply
to file-existence or line-number claims in this document; they are all `[VERIFIED: local
filesystem/git, this session]`. The three items above are flagged not because they are unverified
facts, but because they involve a judgment call about presentation/granularity that CONTEXT.md
leaves open.

## Open Questions

1. **Should FACADE-04's zero-"still-open" finding be stated as a phase-level finding in its own
   right, alongside the row-by-row table?**
   - What we know: all 20 rows resolve cleanly to done/not-a-candidate, with one needing a minor
     inventory correction (row 14). No row requires further investigation.
   - What's unclear: whether ROADMAP criterion 5's phrasing ("marked done, not-a-candidate, or
     still-open") implies the phase author expected some rows to land in "still-open," and whether
     landing at zero should be flagged prominently (the way FACADE-01's "the register was right
     about the count and wrong about the kind" finding is flagged in CONTEXT.md's `<specifics>`).
   - Recommendation: state it plainly in the triage document's own summary line, mirroring how
     `deferred-items.md`'s own framing was corrected in D-02 — this is the same class of "premise
     partially inverted by verification" finding, and burying it would repeat the mistake FACADE-01
     exists to fix.

2. **Does D2's `content_service.rs` / `event_manager.rs` verdict need its own ADR, or is it
   register/ledger material?**
   - What we know: D-14 names "the D1-D4 disposition set (FACADE-02)" as contested and ADR-worthy,
     without distinguishing D2's withdrawn half from its two still-open files.
   - What's unclear: whether `content_service.rs`/`event_manager.rs`'s placement verdict (do/defer)
     is itself contested (two documents disagree) or code-settled (everyone agrees they're
     mis-layered, only the *when* is undecided) — D-00g's test.
   - Recommendation: fold into the same ADR(s) covering D1/D3/D4 rather than creating a fifth
     separate ADR — the Epic 1 audit's recommendation (`content_service.rs` → `paladin-core`,
     `event_manager.rs` → `paladin-core` or a facade app-service) is uncontested; only the trigger
     and owner need stating, which fits inside the same D1-D4 ADR's `## Decision` as easily as a
     ledger row would, and D-14 already frames "the D1-D4 disposition set" as one bundle.

## Environment Availability

**Skipped — code/config-only phase.** This phase's only "environment dependency" is the local git
repository and filesystem, both already available and used throughout this research (git log, git
show, ls, grep, find — all executed successfully against the current working tree, no external
service, package registry, or network access required).

## Validation Architecture

This phase's Nyquist validation applies to **records**, not code: a "test" is a shell command that
proves a citation resolves, a row count matches, or an annotation banner exists at a named path —
identical in kind to Phase 10's own Validation Architecture (its closest precedent).

### "Test Framework"

| Property | Value |
|----------|-------|
| Framework | None — direct shell verification (`grep`, `ls`, `find`, `git log`, `git show`) |
| Config file | None |
| Quick run command | Per-claim `grep -n "<pattern>" <file>` or `[ -e <path> ] && echo EXISTS` |
| Full suite command | Re-run every command in this document's Code Examples and FACADE-04 Verification Table sections, confirming no drift since 2026-08-08 |

### Phase Requirements → Verification Map

| Req ID | Behavior | Verification Type | Command | Verified this session? |
|--------|----------|-----------|-------------------|-------------|
| FACADE-01 | 17 occurrences, 6 files, all rustdoc | grep-count | `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ \| wc -l` | ✅ 17 |
| FACADE-01 | Zero non-doc-comment occurrences | grep-filter | `grep -rn "println!\|eprintln!\|dbg!" ... \| grep -v '///' \| grep -v '//!'` | ✅ 0 (all filtered out) |
| FACADE-02 D1 | `src/core/` is exactly 6 files, 49 importers | find + grep-count | `find src/core -name "*.rs"` (6), `grep -rl "crate::core::" src/ \| wc -l` (49) | ✅ both confirmed |
| FACADE-02 D2 | `user_service.rs` split narrowing facts hold | file-exists + commit-lookup | `ls src/core/platform/manager/` | ✅ all three files present |
| FACADE-03 | Both features removed by one commit, exact LOC | git-show | `git log -1 --format="%H %ai %s" 3d48768` | ✅ confirmed, both features in one commit |
| FACADE-03 | `MlPort` stays in workspace | find | `find crates/paladin-ports -iname "*ml_port*"` | ✅ `crates/paladin-ports/src/input/ml_port.rs` |
| FACADE-04 | All 20 disposition rows' live-tree status | file-exists (×20) | see FACADE-04 Verification Table | ✅ all 20 checked this session |
| FACADE-04 | `paladin-arsenal`/`paladin-sanctum` absent | ls | `ls crates/` | ✅ 11 entries, neither name present |
| Cross-cutting | ADR next-free number | grep | `grep -n "Next free ADR number" .planning/decisions/PROMOTION.md` | ✅ 0034 |

### Sampling Rate

- **Per plan:** re-run the specific `grep`/`ls`/`git log` command the plan's own ADR or register row
  cites, before marking a disposition's evidence cell complete (D-00e's bar, applied per-row).
- **Per wave merge:** re-run the 17-occurrence count and the 20-row existence table against the
  in-progress register file to confirm no row was silently dropped or miscounted during parallel
  fan-out.
- **Phase gate:** re-run this document's full Validation Architecture table verbatim before
  `/gsd-verify-work`, since it measures a mutable working tree, not a fixed historical document.

### Wave 0 Gaps

None — there is no test suite to scaffold. The "tests" are the shell commands in the table above,
all of which already run successfully in this environment.

## Security Domain

`security_enforcement` is not set to `false` in `.planning/config.json` (the key is absent), so this
section is included per the default-enabled rule — scoped to what actually applies.

### Applicable ASVS categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | This phase touches no auth code |
| V3 Session Management | No | — |
| V4 Access Control | No | — |
| V5 Input Validation | No | No `.rs` file is modified (D-13) |
| V6 Cryptography | No | — |

### Known threat patterns for this stack

Not applicable — no new attack surface is introduced; zero `.rs` files change. The one adjacent
consideration: FACADE-03(a)'s recorded reintroduction path for the `paladin user …` CLI surface
touches user registration/login/auth-adjacent commands. This phase does not reintroduce them, but
its record should not understate that a future reintroduction (not this phase's scope) would need
its own security review — worth a one-line note in the register entry, not a gate on this phase.

## Sources

### Primary (HIGH confidence — direct command execution or file read against the working tree,
this session, 2026-08-08)

- `.planning/phases/11-facade-residue-deferred-register-disposition/11-CONTEXT.md` — full read, all
  decisions, canonical refs, code context, specifics, and deferred sections.
- `.planning/REQUIREMENTS.md:1560-1720` — full read: the two Phase 10 hand-off blocks
  (FACADE-02, FACADE-03(b)) and FACADE-01…04's full requirement text.
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — full read.
- `.planning/decisions/0028-m8-reconciliation-authoritative.md` — full read.
- `.planning/decisions/0018-m6-facade-reexport-policy.md` — full read.
- `.planning/decisions/PROMOTION.md` — numbering scheme, heading set, promotion procedure, current
  index (0001-0033, next free 0034) — full read of lines 1-160.
- `.planning/ledgers/milestone-07-08.md` — the five FACADE-relevant rows (`REQ-m8-deferred-items-
  register`, `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`,
  `REQ-adapter-disposition-record`, `REQ-m8-epic3-no-extractions`, plus
  `REQ-extracted-crate-dependency-rule` and `REQ-garrison-sanctum-bridges-kept`) grepped and read in
  full this session.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` — full read.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` — full read.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md`
  — full read (including the already-live Phase 10 banner).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-
  modules.md` — header/banner and §5 non-goal clause read this session.
- Direct shell verification this session: `grep -rn "println!\|eprintln!\|dbg!"` (D5 set), `find
  src/core`, `grep -rl "crate::core::"`, `ls src/application/cli/commands/`, `test -d
  crates/paladin-ml`, `git log -1 3d48768`, `find crates/paladin-ports -iname "*ml_port*"`, and a
  full 20-path existence check for every row in `infrastructure-adapter-disposition.md`'s table.
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-RESEARCH.md` and
  `10-CONTEXT.md` — read as the closest structural precedent for a zero-`.rs`-change,
  decision-recording phase (Standard Stack / Package Legitimacy / Validation Architecture / Security
  Domain section shapes all modeled on this precedent).

### Secondary (MEDIUM confidence)

None — every claim in this document was either directly re-verified against the working tree this
session or is copied verbatim from an `Accepted` ADR / CONTEXT.md's own locked decisions.

### Tertiary (LOW confidence)

None.

## Metadata

**Confidence breakdown:**
- Standard stack (documentation conventions): HIGH — fully specified by `PROMOTION.md` and
  exemplified by three ADRs read in full this session.
- Architecture (which tier owns which deliverable): HIGH — directly derived from D-00b/D-00g's
  locked precedence rules, no ambiguity.
- FACADE-01/03 factual claims: HIGH — every `file:line` and commit SHA independently re-verified
  this session via direct command execution.
- FACADE-04 per-row table: HIGH — all 20 paths independently existence-checked this session;
  cross-referenced against ADR-0028's own orphan-deletion list for consistency.
- Open judgment calls (A1-A3, Open Questions 1-2): MEDIUM — these are presentation/granularity
  choices CONTEXT.md deliberately leaves to Claude's Discretion, not factual uncertainty.

**Research date:** 2026-08-08
**Valid until:** This document measures a mutable working tree (file existence, line numbers, ADR
numbering state). Re-verify any cited `file:line` or `ls crates/` output if more than a few days
elapse before planning/execution, per this project's own D-00e evidence-bar convention. Estimate:
7 days (fast-moving relative to the tree, since this is an active-development repo), though the
underlying *records* (ADR text, ledger rows) do not expire — only line-number and existence
citations do.
