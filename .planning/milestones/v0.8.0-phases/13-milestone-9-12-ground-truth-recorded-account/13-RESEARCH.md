# Phase 13: Milestone 9-12 Ground Truth & Recorded Account - Research

**Researched:** 2026-08-10
**Domain:** Not code — this is a records phase. The domain is: how this corpus's own ledger/ADR/
annotation conventions are executed, so a planner can fan out 120 ledger rows, 3 ADRs and 3
documentation-line edits consistently. No Rust library, framework, or testing-strategy research
applies; everything below is mechanics, precedent and independently-re-verified fact.
**Confidence:** HIGH — every claim below was either read from the tree this session (2026-08-10,
independent of 13-CONTEXT.md's own 2026-08-10 verification pass) or is a direct citation to
13-CONTEXT.md with the file:line re-checked.

## Summary

Phase 13 writes no `.rs`. It writes one new ledger file (120 rows, 24 sections), three new ADRs
(0037-0039), roughly ten `.project/` dated-annotation banners (several of which — the four D-09
Milestone 12 stale-path corrections — **already exist**, written by Phase 8 on 2026-08-06, confirmed
this session), and three lines of prose across `docs/src/deployment-topologies/{sidecar,
http-service-host,overview}.md`. 13-CONTEXT.md is unusually thorough (23 decisions, 5 fresh findings,
all independently re-verified this session where re-checked) — this research does not re-litigate
any of it. It adds four things CONTEXT.md's D-14 mechanics did not need and this phase's *planner*
does: (1) the exact structural template the sibling ledgers use, verified line-by-line against
`milestone-07-08.md`; (2) an independently-measured, section-by-section breakdown of the 120-row
ledger's `Verify`/`Shipped`-bare/rich split that reconciles exactly with D-04's 35/53/32 headline
figure once two parenthetical-qualified rows are correctly bucketed as rich, plus confirmation that
**no section header's stated ID count disagrees with its measured row count** (the arithmetic error
D-04 corrects is at the requirement-text summary-sentence level only); (3) live verification that
this environment can run `mdbook build docs/` end-to-end (all three CI-pinned tool versions install
offline from the cached registry) — and that doing so **fails today, before this phase's edits**,
for two pre-existing broken-link defects in files this phase does not touch; and (4) that SUPPLY-01's
one still-pending clause (D-06) has, since 13-CONTEXT.md was written, **stopped being pending** — a
CI run newer than the `30861568499` trigger citation now exists and passed.

**Primary recommendation:** Follow `milestone-07-08.md`'s structural template exactly (head notes →
supersession/highlight summary tables → row-order-and-amendment-convention note → ledger-file
contention table → epic sections in `REQUIREMENTS.md`'s own order → close-out amendment section
appended at the foot, never inserted). Decompose along Phase 10's proven shape: one scaffold plan,
N disjoint-section fan-out plans, ADR plans that touch no ledger file, one close-out plan. Budget
work against the **35 bare-`Verify` rows**, not the 120-row total or the "104" figure D-04 retires.

## Architectural Responsibility Map

This phase has no runtime architecture — it is a documentation/records phase. The "capabilities" it
touches map onto planning-corpus tiers, not application tiers:

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Requirement verdicts (120 rows) | `.planning/ledgers/` (new file) | `.planning/REQUIREMENTS.md` (pointer only) | D-01: the ledger is the fifth sibling; REQUIREMENTS.md's own section is reduced to a pointer, exactly as Phase 7 and Phase 10 did for their siblings |
| Contested positions (route surface, `AgentProvisioner`, Garrison/Arsenal) | `.planning/decisions/` (ADRs 0037-0039) | — | D-00g: contested positions get ADRs; ADRs sit at the top of the precedence order by construction |
| Code-settled defects (stale paths, relocations) | Ledger row + `.project/` annotation | — | D-00g: no ADR for a fact the tree already settles |
| Source-document corrections | `.project/` (dated banners, original text retained) | — | D-00c: annotation, never rewriting |
| The one live-contract defect (`sidecar.md:29`) | `docs/src/deployment-topologies/` | ADR-0037 (records it as the answer's consequence) | D-12: a published mdbook page is a live contract, unlike a `.project/` PRD nobody executes against — this is the one place this phase's edit surface leaves `.planning/`/`.project/` |
| Version trajectory | `.planning/decisions/0029-*.md` (amended, not replaced) | — | D-16: one ADR is the single home for the whole line; ORCH-05 appends, never forks a rival ADR |

## User Constraints (from CONTEXT.md)

`13-CONTEXT.md` is the authoritative, locked source (23 decisions D-01…D-23, five ⚠ fresh findings,
two ⚠ HUMAN REVIEW flags). It is **not** reproduced here in full — at ~1000 lines it is longer than
this research document would be useful at that length, and the planner must read it directly
(`.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-CONTEXT.md`). This section
indexes it so nothing is silently dropped; every line below is a locked decision, not a suggestion.

### Locked Decisions (index — full text in 13-CONTEXT.md)

**Inherited, cross-phase (D-00a…D-00i):** ADR shape and precedence (D-00a/b); `.project/` annotation
is dated, non-destructive, D-00c; ledgers amended in place, D-00d; evidence bar = `file:line` **and**
an exerciser, D-00e; `REQ-*` is the primary key, two rows never merged for sharing a citation, D-00f;
contested positions get ADRs, code-settled defects get ledger rows, D-00g; medieval-military language
mandatory, D-00h; `--auto` provenance is carried forward, never laundered, D-00i.

**ORCH-01 (the ledger):** New file `.planning/ledgers/milestone-09-12.md`, REQUIREMENTS.md's section
becomes a pointer (D-01). Vocabulary is the run-5 eleven-class key already written at
`REQUIREMENTS.md:3632-3636`, mapped onto the series' seven-class key in the head note — **not**
re-keyed (D-02). `Verify` is retired; all 35 bare-`Verify` rows get real verdicts. Phase 10's evidence
bar carries over verbatim including the manifest carve-out; the 53 bare-`Shipped` rows are
**re-derived**, not carried (D-03). The "sixteen already have it / remaining 104" arithmetic is
wrong — it conflates variant-register `settled-by` entries with ledger rows; the real split is
35 `Verify` / 53 bare `Shipped` / 32 already-rich, **all 120 need a verdict**, corrected at source
(D-04, ⚠ fresh finding — independently re-verified this session, see Findings §1 below). The new
verdict class `Shipped, one acceptance criterion false` carries **both halves** dated — SUPPLY-01
failed, and as of 2026-08-08 no longer does (D-05). The three SUPPLY closures are cited with
provenance, not re-verified (D-06 — see Findings §2, the pending clause has since resolved). Phase
12's 87-hit stale-citation inventory is inherited whole (D-07). `ci.yml`'s job list is stale — 15
jobs measured, not 14; corrected at source and handed to Phase 15 (D-08). DEBT-01's stale-path defect
is half closed — the script reads the dotted path and the baseline file exists; only the
*documentation* half (four Milestone 12 requirement texts) remains, and it stays four ledger rows,
**not** a sixth ORCH-03 item (D-09 — see Findings §3, already annotated by Phase 8).

**ORCH-02 (checkbox verdicts):** Five verdicts (M9 corroborated-0-open, M10 corroborated-but-one-
criterion-false, M11 the only genuinely-open-26, M12 vacuous-3-scaffolding, project-management
nonexistent-1), no ADR, the five-run pattern (understated → accurate → overstated → contradicted →
vacuous) written once in the ledger head note (D-10).

**ORCH-03 (positions the tree contradicts):** (a) The agent route surface is `/v1`, confirmed against
`crates/paladin-web/openapi.json`; Epic 1/3/4/5 route text becomes superseded provenance; ADR-0037
(D-11). (a′) ⚠ fresh finding: the one *live* route defect is `docs/src/deployment-topologies/
sidecar.md:29` (`POST /agents/{id}/execute`), not named by ORCH-03's own text — one-line fix, part of
ADR-0037's consequence (D-12). (b)-(e) Four path relocations (`listener_service.rs`, `llm_port.rs`,
`Design_and_Architecture.md`, README asciinema) verified absent-at-old-path/present-at-new-path,
corrected at source, no ADR (D-13).

**ORCH-04 (the two seams):** (a) `AgentProvisioner` stays in `paladin-web` — not because "either
placement is clean" (that Epic 1 §7 claim omits `AgentSpec`, which derives `utoipa::ToSchema` and
would drag an OpenAPI HTTP DTO into `paladin-ports`, violating ADR-0015(i)) but because it is an HTTP
request-body type; ADR-0038; ⚠ HUMAN REVIEW, reversibility costly (D-14). (b) HTTP-served agents have
no Garrison and no Arsenal — recorded as a **permanent property of the topology**, not planned scope;
`http-service-host.md:54`'s diagram currently *advertises* the opposite and must be corrected;
`overview.md`'s comparison table must state the limitation; ADR-0039; ⚠ HUMAN REVIEW, reversibility
costly (D-15).

**ORCH-05 (version trajectory):** Append `v0.3.0`(M9)/`v0.4.0`(M10)/`v0.5.0`(M11)/`v0.6.0`(M12) rows
to ADR-0029's `## Trajectory` table, ascending order, never re-sorted; no second version ADR; REL-01
not re-opened (D-16). The self-numbering prediction is **already** closed by ADR-0030 — cite it, run
the one confirmation grep, do not re-decide (D-17, ⚠ fresh finding). ORCH-05's own current-state
figures (`Cargo.toml` "0.6.0", latest tag "v0.5.1") are two releases stale — actual is `0.7.0`/
`v0.7.1` — corrected at source, the identical defect class Phase 10's D-11 already fixed once in
HARD-03 (D-18, ⚠ fresh finding).

**Cross-cutting:** Code-change boundary is record-writing plus exactly three documentation lines, zero
`.rs` — asserted by `git diff --name-only <base>..HEAD -- '*.rs' | wc -l` → `0` at close-out (D-19).
ADR allocation is 0037-0039 exactly; `PROMOTION.md` advances to 0040 (D-20 — see Findings §6 below,
a fourth PROMOTION.md Part B candidate assigned to Phase 13 is *not* covered by this allocation).
Every closure claim is proved by a command run in this environment (D-21 — `cargo audit`/`cargo deny`/
`cargo llvm-cov`/Docker unrunnable, confirmed again this session; `gh` reads work; `mdbook build`
should be attempted — see Environment Availability). Three forward hand-off blocks are written for
Phase 14 (WEB-01…04), Phase 15 (PIPE-01…, D-08's job list, D-09's finding), Phase 16 (DOCS-01…04,
D-10's M11 verdict, D-13(d)/(e)) (D-22). Suggested decomposition: ~11 plans, 4 waves (D-23 —
independently reconciled against Phase 10's actual shape below, see Decomposition Precedent).

### Claude's Discretion (from CONTEXT.md — copied verbatim)

- Whether the two head-note class tables (D-02) sit at the head of the ledger, at the foot, or both.
- Whether ADR-0038 and ADR-0039 are two files or one (D-20 recommends two).
- The exact wording/placement of ADR-0039's limitation statement in `overview.md`.
- Whether the ledger presents run-5's 37 verified-shipped claims inline per row or as a
  cross-reference block.
- Exact banner wording and inline-correction markup for every `.project/` annotation.
- Whether ORCH-05's provenance-key confirmation (D-17) rides in its own plan or folds into close-out.
- Whether the ledger's Deferred-QA section is ordered by epic number or by recommended implementation
  order.
- Whether D-08's corrected `ci.yml` job list is written into the ledger head note, PIPE-01's text, or
  both.

### Deferred Ideas (OUT OF SCOPE — from CONTEXT.md, do not re-propose)

WEB-01's JWT-vs-opaque mechanism; WEB-02's multi-replica token verification; WEB-03/04's
`ProviderCapabilities`/tool-calling; wiring Garrison/Arsenal into HTTP-served agents; promoting
`AgentProvisioner` to `paladin-ports`; Deferred-QA Epic 25's coverage-threshold variant (78% vs
70→74→78 ramp); the eight deprecated GitHub Action references; the 311-line architecture-document
rewrite; `docs/assets/`/`docs/DEMOS.md`; three unused `paladin-content` optional deps; seven crates
with `doctest = false`; a `cargo tree`-based dependency-allowlist CI check; applying GitHub rulesets;
stray root artefacts; `dotenv`→`dotenvy`; owner/expiry fields for 13 ungoverned `deny.toml` ignores;
a `SECURITY.md`; retiring `src/main.rs`; Nyquist validation for Phases 1-4; publishing ADRs to the
mdbook. All 19 items carry a named owner phase in CONTEXT.md's `<deferred>` block — read it there for
the full reasoning per item.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-------------------|
| ORCH-01 | 120-row cited ledger, both-halves verdict class, corrected arithmetic | Ledger Format Contract (below), 120-Row Inventory Sectioned (below), SUPPLY-01 trigger now fired (Findings §2) |
| ORCH-02 | Five checkbox verdicts, five-run pattern, no task created | Ledger Format Contract's head-note precedent (`milestone-07-08.md`'s own "Per-milestone checkbox corroboration" head note is the template) |
| ORCH-03 | Route surface ADR + one live doc fix; four path relocations | ADR File Shape (below), exact current line content of `sidecar.md:29` (verified, Code Examples below) |
| ORCH-04 | Two ADRs recording the two default-seams as decisions | ADR File Shape (below), exact current line content of `http-service-host.md:54` and `overview.md`'s table (verified, Code Examples below), PROMOTION.md Part B gap (Findings §6) |
| ORCH-05 | Append 4 rows to ADR-0029's Trajectory table, cite ADR-0030 | ADR-0029 Trajectory Table Mechanics (below) |

## Ledger Format Contract

`milestone-07-08.md` (549 lines, Phase 10's deliverable, the most recent and closest-analogue
sibling) is the template to copy structurally. Read in full this session; the shape, in the order it
appears in the file:

1. **Opening supersession line** — states which `REQUIREMENTS.md` section this file supersedes and
   that the section becomes a pointer, naming the *next* sibling this file itself will name (Phase 13
   should state that `milestone-09-12.md` is the **fifth and final** sibling — the series is complete
   after this phase; no sixth name to forward).
2. **Primary-key note** — `REQ-*` is the key; outstanding task items nest under their requirement;
   two rows sharing a citation stay separate rows (D-00f, verbatim in the file).
3. **Evidence bar paragraph** — states the bar (`file:line` + exerciser) and explicitly that it
   applies to **all N rows without exception**, including rows an earlier ingest already marked with
   a status word — "an ingest status word **is** the bare 'the code exists' claim this bar exists to
   reject." Phase 13's version of this paragraph should say the same about the 53 bare-`Shipped` rows
   (D-03).
4. **Manifest carve-out paragraph** — names the specific CI jobs/build legs that count as "exercising"
   a manifest-only claim, each with a `file:line`. Phase 13's manifest-heavy sections (Milestone 10,
   most of Milestone 12) need the equivalent: name the specific `ci.yml` job (per D-08's corrected
   15-job list) or `Makefile` target that exercises each manifest-only row, not a bare "CI runs it"
   assertion.
5. **Path-caveat / workspace-shape paragraphs** — project-specific factual anchors re-grepped fresh
   this session, not trusted from an earlier document. Phase 13's equivalent anchors: the `/v1` prefix
   fact (`openapi.json`), the 15-job `ci.yml` list (D-08), the `.project/current-exports.txt`
   existence fact (D-09), and the corrected version/tag state (D-18).
6. **Per-milestone checkbox corroboration paragraph** — this is `milestone-07-08.md`'s analogue of
   ORCH-02: it states each milestone's checkbox count and whether it is corroborated or contradicted,
   *once*, in the head note, exactly as D-10 requires for Phase 13's five verdicts.
7. **Provenance paragraph** — names the scaffold plan and what it derives fully vs. what the fan-out
   plans replace.
8. **Verdict legend** — a table, `Verdict | Meaning`, followed by a **HARD-01-mapping callout**
   explicitly reconciling this ledger's finer vocabulary against the coarser cross-series vocabulary,
   plus a **tie-break rule** for rows that qualify for two classes at once. **Phase 13's D-02 already
   locks the vocabulary choice** (the eleven-class run-5 key, not a new seven-word set) — so this
   legend section becomes a **mapping table** (eleven classes → the series' seven), not a redefinition.
   No tie-break rule is dictated by CONTEXT.md; if one is needed (e.g. a row eligible for both
   `Shipped (relocated)` and `Superseded by outcome`), state it explicitly, mirroring
   `milestone-07-08.md`'s own tie-break precedent (`relocated` wins, because the mdbook-relocation
   signal is the corpus's largest false-gap generator).
9. **A dedicated summary table for the headline finding(s), placed at the head of the file** — this is
   where `milestone-07-08.md` puts its "Superseded by outcome — do not plan these as written" table,
   with an explicit **counting command** run in that session and a stated **caveat** reconciling its
   count against a different count elsewhere in the corpus. Phase 13 needs **two** such tables per D-02:
   one for `Shipped, one acceptance criterion false` (one row, both halves — trivial to place) and one
   for `Verified open` (the Deferred-QA block, ~14+ rows — the "direct input to Phases 14-16" table).
10. **Row order and amendment convention paragraph** — states the epic-section order is
    `REQUIREMENTS.md`'s own order, never re-sorted, and that later plans replace a row's Verdict/
    Evidence cells **in place** — never insert, delete, or reorder rows.
11. **A ledger-file-contention table** — `Plan | Wave | Owns | May`, one row per plan touching the
    ledger, explicit about section ranges so parallel fan-out plans produce non-overlapping diffs that
    merge without conflict. **This is the single most important structural element for Phase 13's
    120-row, larger-than-any-predecessor ledger** — D-23's wave 2 fan-out (5 plans across 24 sections)
    needs this table spelled out with exact section-name ranges per plan, mirroring
    `milestone-07-08.md`'s table exactly in shape.
12. **The epic sections themselves**, in `REQUIREMENTS.md`'s own order, `### Milestone N Epic M —
    Title (K IDs)`, then a `| Requirement | Verdict |` two-column table (not three — `milestone-07-08.md`
    uses a two-column `ID | Verdict` format where the citation is embedded prose inside the Verdict
    cell, not a separate Evidence column, despite the earlier "Verdict and Evidence cells" language in
    its own head note — **note this internal inconsistency and pick the two-column shape**, since it is
    what the file's actual rows use and what `REQUIREMENTS.md`'s own existing 120-row draft ledger
    already uses, verbatim, at `:3607-3931`).
13. **A close-out amendments section, appended at the foot**, dated, per D-00d — never rewriting the
    epic sections above it. Contains: a whole-file integrity re-check (`grep -c '^| REQ-'`, distinct-ID
    count, section-header count, zero unresolved stub markers, zero blank cells, every `ADR-NNNN`
    citation resolves to a file that exists); a per-class verdict-distribution table counted directly
    from the Verdict column that session; a list of rows cited rather than re-derived, and why; a list
    of claims that are CI-only and were never measured in this environment, each with the exact command
    a CI runner executes; a phase-gate section running the D-19 `.rs`-diff-count assertion plus
    `cargo test`/`fmt`/`clippy` (or the reasoning why they don't apply to a zero-`.rs`-diff phase, per
    Phase 10's own precedent — see Validation Architecture below).

**The interim-state contract for rows not yet derived by a given plan** (from `milestone-07-08.md`'s
own head note, "For every row this plan does not derive…"): the Verdict cell reads
`pending — plan 13-NN` naming the owning fan-out plan, and nothing is ever left blank. Phase 13's
scaffold plan should adopt this same interim marker for wave-1-written, wave-2-owned rows — except
that, unlike Phase 10, roughly two-thirds of Phase 13's 120 rows **already carry real content** in
`REQUIREMENTS.md`'s existing draft ledger (`:3607-3931`) that the scaffold plan can transcribe
directly rather than stub as `pending`. Only the 35 bare-`Verify` rows genuinely need a `pending —
plan 13-NN` placeholder; the 53 bare-`Shipped` and 32 rich rows can be transcribed with their existing
text prefixed `run-5 input (not yet re-derived):`, exactly as `milestone-07-08.md`'s convention states
for the equivalent case.

**Sibling ledgers' additional convention — in-place amendment sections stack.** `milestone-01.md`
demonstrates this: `## Phase 2 amendments (2026-08-01)`, `## Phase 3 amendments (2026-08-02)`,
`## Phase 4 amendments (2026-08-03)` are three separate dated sections, each appended after the last,
each retaining the others untouched. This matters for Phase 13's own close-out section: it is
`milestone-09-12.md`'s **first and only** amendment section (the ledger is new this phase), but the
convention it establishes — append, date, never rewrite — is what a hypothetical future phase touching
this ledger would have to follow. `milestone-04-06.md` confirms the same shape at Phase 8's own single
amendment section, `## Phase 8 amendments (2026-08-06)`.

## The 120-Row Inventory, Sectioned

Counting commands run this session against `REQUIREMENTS.md:3607-3931` (independent of, and
reconciling exactly with, D-04's headline 35/53/32 figures):

```
sed -n '3607,3931p' REQUIREMENTS.md | grep -c '^| REQ-'                    → 120
sed -n '3607,3931p' REQUIREMENTS.md | grep -oP '^\| REQ-\S+ \| \K.*' \
  | grep -c '^Verify'                                                       → 35
```

For the `Shipped`-bare/rich split, a naive `grep -c '^Shipped[^,]'` over-counts by 2 (it buckets
`Shipped (relocated)` and `Shipped (partially verified)` as "bare"). D-04's own 53/32 split treats
those two parenthetical-qualified rows as **rich** (they carry a distinguishing disposition beyond a
plain citation), which the exact command below reproduces:

```
grep -c '^Shipped — '  → bare Shipped: 53   (plain em-dash citation, no qualifier)
grep -c '^Shipped → '  → included in the 53 (bare arrow-to-requirement form, no citation prose)
everything else        → rich: 32           (comma-qualified, parenthetical-qualified, or a
                                              named class: Verified open / Contract diverges /
                                              Provenance only / Superseded / Open …)
```

**Per-section breakdown** (rows = measured `grep -c '^| REQ-'` inside each section's own line range;
verify/shipped-bare/rich computed with the reconciled classification above; every row's stated
`(N IDs)` header count is compared against its measured row count):

| Section | Stated | Measured | Match? | Verify | Shipped-bare | Rich |
|---|---|---|---|---|---|---|
| M9 Epic 1 — Orchestrator E2E Workflow Execution | 6 | 6 | ✅ | 3 | 3 | 0 |
| M9 Epic 2 — Scheduler, Queue & Event Op. Validation | 5 | 5 | ✅ | 4 | 0 | 1 |
| M9 Epic 3 — Content Processing Pipeline | 4 | 4 | ✅ | 2 | 2 | 0 |
| M9 Epic 4 — Agent / Orchestrator Bridge | 4 | 4 | ✅ | 2 | 2 | 0 |
| M9 Epic 5 — User / Admin System Completion | 5 | 5 | ✅ | 2 | 3 | 0 |
| M9 Epic 6 — Finalization & Release | 1 | 1 | ✅ | 0 | 1 | 0 |
| M10 Epic 1 — Pre-Commit & Pre-Push Hooks | 4 | 4 | ✅ | 2 | 2 | 0 |
| M10 Epic 2 — Dependency Security & Licence Compliance | 8 | 8 | ✅ | 1 | 5 | 2 |
| M10 Epic 3 — Release Automation | 6 | 6 | ✅ | 2 | 4 | 0 |
| M10 Epic 4 — v0.4.0 Release | 1 | 1 | ✅ | 0 | 1 | 0 |
| M10 Epic 5 — Tag-Source Enforcement | 4 | 4 | ✅ | 2 | 2 | 0 |
| M11 Epics 1-2 — mdbook Scaffold & Chapter Hierarchy | 4 | 4 | ✅ | 1 | 3 | 0 |
| M11 Epic 3 — Content Rewrite | 7 | 7 | ✅ | 3 | 1 | 3 |
| M11 Epic 4 — New Subsystem Guides | 4 | 4 | ✅ | 3 | 0 | 1 |
| M11 Epic 6 — Deployment Topologies | 1 | 1 | ✅ | 0 | 1 | 0 |
| M11 Epics 5 & 7 — README, Version Sync, Final Review, v0.5.0 | 4 | 4 | ✅ | 2 | 1 | 1 |
| M12 Epic 1 — Agent Registry & Execution API | 6 | 6 | ✅ | 0 | 5 | 1 |
| M12 Epic 2 — Configurable Web Host & Server Binary | 4 | 4 | ✅ | 1 | 2 | 1 |
| M12 Epic 3 — Streaming & Async Jobs | 4 | 4 | ✅ | 1 | 3 | 0 |
| M12 Epic 4 — Operational Hardening | 5 | 5 | ✅ | 0 | 5 | 0 |
| M12 Epic 5 — API Security & Authorization | 6 | 6 | ✅ | 3 | 1 | 2 |
| M12 Epic 6 — OpenAPI & Interactive Docs | 4 | 4 | ✅ | 0 | 3 | 1 |
| M12 Epic 7 — Deployment Artefacts, Examples & Docs | 5 | 5 | ✅ | 1 | 3 | 1 |
| Deferred-QA Epic 25 — CI/CD Pipeline Enhancement | 7 | 7 | ✅ | 0 | 0 | 7 |
| Deferred-QA Epic 26 — Documentation & Rustdoc | 4 | 4 | ✅ | 0 | 0 | 4 |
| Deferred-QA Epic 27 — LLM Tool Calling | 2 | 2 | ✅ | 0 | 0 | 2 |
| Deferred-QA Epics 28-29 & the coverage register | 4 | 4 | ✅ | 0 | 0 | 4 |
| project-management | 1 | 1 | ✅ | 0 | 0 | 1 |
| **Total** | **120** | **120** | **✅** | **35** | **53** | **32** |

**Finding: no section header's stated ID count disagrees with its measured row count.** Every one of
the 24 (well, the table above lists 29 rows because three headers bundle multiple epics — the
corpus's own "24 epic sections" figure counts `M11 Epics 1-2` and `M11 Epics 5 & 7` each as one
section) sections is internally accurate. **The arithmetic error D-04 corrects (`16`/`104`) is a
defect in ORCH-01's own requirement-text summary sentence, not in any of the 24 section headers
themselves** — a planner reading only the section headers would never have found the class of error
D-04 flags; it only appears in the prose above the table.

**Budget implication for the planner:** the 35 bare-`Verify` rows are not evenly spread. M9 Epic 2
(4 of 5 rows), M11 Epic 4 (3 of 4), M12 Epic 5 (3 of 6) and M9 Epic 1 (3 of 6) are the densest
`Verify` clusters — plans covering those sections carry more genuinely new verdict-writing work than
plans covering M12 Epics 4/6/7 or the entire Deferred-QA block (0 `Verify` rows each, since those were
already verified item-by-item during ingest run 5). The Deferred-QA and project-management sections
(18 rows total, 0 `Verify`, 0 bare-`Shipped`) are **already fully rich** and need transcription only —
matching D-23's own observation that this is "the highest-value rows in the ledger for Phases 14-16."

**Underlying acceptance-criteria source for the 35 `Verify` rows:** `.planning/intel/requirements.md`
(6,241 lines) is where the full acceptance-criteria text for each `REQ-*` ID lives — `REQUIREMENTS.md`'s
own ledger head note (`:3612-3613`) points there. Fan-out plans deriving `Verify` rows will need to
read the relevant section of this file, not just the requirement ID, to know what "done" means for
that row.

## ADR File Shape

Three ADRs read in full this session (`0036-audit-suppression-single-source-topology.md`,
`0031-extracted-crate-dependency-rule.md`, `0029-version-trajectory-history.md`) confirm the seven
required headings, in order, verbatim, no deviation: `## Status` → `## Context` → `## Decision` →
`## Considered Options` → `## Code Locations` → `## Code Conformance` → `## Downstream Consumers`. No
frontmatter (`PROMOTION.md:119-140` states this explicitly and names the parser reason: `adr-parser.cjs`
requires `## Code Locations` and `## Considered Options` to be **bulleted lists, never prose
paragraphs**, or `splitEntries` collapses them into one opaque blob).

**`## Status` body shape:** `Accepted` (bare word) then a blank line then `**Date:** YYYY-MM-DD`.

**`## Code Conformance` vocabulary:** exactly `conforms` or `must change` (D-03/PROMOTION.md Part A
step 3), optionally followed by a prose paragraph with the measured re-run command and its output.
All three ADRs read this session carry `conforms` — none instructs a code change. **All three of Phase
13's ADRs are expected to carry `conforms` too**, per D-20's own text ("Conformance: `must change`,
executed in this phase (one doc line)" for ADR-0037 and ADR-0039 — note: D-20 actually marks ADR-0037
and ADR-0039 `must change` because each **executes** a documentation-line fix as part of the same
phase, mirroring ADR-0032's precedent of a `must change` ADR whose code consequence is executed by the
same plan that writes it. ADR-0038 is `conforms` — it changes no file, it ratifies where the trait
already lives.) When an ADR is `must change` and the phase executes the change itself, name the
executing plan/task explicitly in `## Code Conformance`, exactly as ADR-0032 and ADR-0033 do.

**`## Downstream Consumers` shape:** a bulleted list, each bullet leading with **`Phase N / REQ-ID`**
in bold, then a sentence describing what that phase inherits and why it doesn't need to re-derive it.
Every one of the three ADRs read this session follows this pattern exactly. Phase 13's three new ADRs
should name Phase 14 (ADR-0038, ADR-0039 per D-14/D-15's own text) and Phase 15 (ADR-0037, since
PIPE-01 is the CI-job owner and any future route work touches the versioned surface) as consumers,
matching the phases CONTEXT.md's D-22 hand-off block already names.

**`## Supersedes` line:** only present when an ADR supersedes an earlier one (none of Phase 13's three
new ADRs supersede an existing ADR — they are new questions, not corrections to an old answer, per
D-20's allocation). If one *did* need to supersede, `PROMOTION.md:142-152` is the exact mechanism: the
superseded ADR's file is never deleted, its `## Status` body becomes the bare word `Superseded` plus a
prose line naming the superseding number and reason; the superseding ADR carries `## Supersedes: 00NN`.

**Promotion procedure** (`PROMOTION.md:161-182`, six steps, re-read this session): (1) take the next
free number from the Numbering Index line, never decrementing even if a candidate is later rejected;
(2) author the standard heading set; (3) set `## Code Conformance`; (4) cite the source `.project/`
document's path in `## Code Locations` alongside shipped-code citations; (5) update the "Next free ADR
number" line; (6) add a row to `PROJECT.md`'s `## Key Decisions` table.

**Next-free confirmed:** `PROMOTION.md:60` — **"Next free ADR number: 0037"** — re-read this session,
matches D-00a and D-20 exactly. `ls .planning/decisions/003[6-9]*.md` this session returns only
`0036-audit-suppression-single-source-topology.md` — 0037-0039 do not yet exist, confirming the
allocation is genuinely free.

**Advancing-note convention** (`PROMOTION.md:62-115`, one dated prose paragraph per phase that
authored ADRs): states how many numbers advance, which plan authored each, whether each closes a Part
B inventory entry, and — critically — **explicitly states when an ADR does *not* close an inventory
entry**, so a reader doesn't have to infer absence. Phase 12's plan 12-04 note is the most recent
example and should be the template for Phase 13's own advancing note (`0037 through 0039, from
0036 to 0040`, naming plans 13-NN per ADR, and explicitly stating whether Part B candidates 8 and 9
are closed — see Findings §6, only candidate 8 is covered by this phase's allocation).

## ADR-0029 `## Trajectory` Table Mechanics

Read in full this session. Columns, exactly: `Version / tag | Status in the tree | Date | What it
was | Evidence`. Five rows exist today, in ascending order:

1. `0.2.0` (lockstep target) — targeted, not shipped
2. `0.1.0` / `v0.1.0-rc.1` — shipped, 2026-05-28
3. `v0.2.0` — shipped
4. **`v0.3.0` … `v0.5.1` — shipped between Epic 5 and Epic 7** — a single **spanning placeholder row**
   whose "What it was" cell reads *"Owned by Phase 13 / ORCH-05 — to be appended"* and whose Evidence
   cell cites `git tag --sort=-v:refname` listing the intermediate tags. **This is the row ORCH-05
   must resolve.**
5. `0.7.0` / `v0.7.0`, `v0.7.1` — shipped, 2026-08-03

**D-16's instruction is "append four rows… without re-sorting or re-keying the existing rows."**
Because row 4 above is a placeholder explicitly reserved for this phase (not a real historical entry
to preserve untouched), the mechanically correct move — consistent with the table's own stated keying
rule ("a row is keyed by the tag or target figure, not by the milestone... a milestone that targeted
two figures gets two rows") — is: **retain row 4's text under a dated D-00d superseded-in-place
marker** (its own placeholder content becomes the "superseded" text a later reader can still see,
exactly as every other in-place ledger amendment in this corpus retains superseded text) and insert
the four new per-tag rows (`v0.3.0`, `v0.4.0`, `v0.5.0`, `v0.6.0`, each with its own Date/What-it-was/
Evidence cells sourced from the corresponding ledger row — `REQ-m9-quality-gate-v030`,
`REQ-m10-v040-release`, `REQ-m11-v050-release`, `REQ-m12-v060-release`) in ascending order **between**
row 3 (`v0.2.0`) and row 5 (`0.7.0`). This keeps the table's own ascending-order invariant intact and
does not delete anything — the placeholder's "to be appended" framing is not itself a historical fact
worth preserving verbatim in the live table the way a shipped/targeted milestone entry is, but D-00d's
"retain superseded text" convention still applies to it as an editorial artifact of an earlier phase's
writing, so do not simply delete the row — strike it or fold its text into a dated note explaining it
has been superseded by the four rows that follow.

**Version figures to cite per new row** (independently re-verified this session, matching D-18
exactly): `Cargo.toml:34` → `version = "0.7.0"`; `git tag --sort=-v:refname | head -8` →
`v0.7.1, v0.7.0, v0.5.1, v0.5.0, v0.4.3, v0.4.2, v0.4.1, v0.4.0`; branch `release/v0.7.0`. The four
new rows' own per-tag evidence should cite the ledger rows named above (`REQ-m9-quality-gate-v030` et
al.), which already carry the lockstep-mechanism citation, rather than re-deriving commit hashes for
each intermediate tag from scratch — none of D-16/D-17/D-18's text asks for a fresh `git log` per tag,
only for the chain to be unbroken and cite the mechanism.

## Package Legitimacy Audit

**N/A — this phase installs no external packages.** Its entire edit surface is `.planning/`,
`.project/`, and three lines across three `docs/src/deployment-topologies/*.md` files (D-19). No
`Cargo.toml`, `package.json`, or equivalent manifest is touched. The Package Legitimacy Gate protocol
is skipped for this reason, stated explicitly rather than silently omitted.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Ledger structure | A new head-note format, a new verdict vocabulary, a new legend | `milestone-07-08.md`'s exact structural template (Ledger Format Contract, above) | Four prior phases already solved this; D-02 additionally *locks* the vocabulary choice — there is no design freedom here, only transcription freedom |
| ADR structure | A new heading set, a new conformance vocabulary | The seven required H2 headings, `conforms`/`must change`, per `PROMOTION.md:119-140` | `adr-parser.cjs` structurally requires bulleted `## Code Locations`/`## Considered Options`; a prose paragraph silently breaks tooling with no error |
| `.project/` corrections | A rewritten source document, a separate corrections file | Dated banner + inline struck-and-corrected text, original retained (D-00c) | The four D-09 Milestone 12 stale-path corrections are **already done this way** by Phase 8 — copy that exact pattern, don't invent a new one |
| Version history | A second version ADR | Append to ADR-0029's existing `## Trajectory` table | D-16 explicitly prohibits a rival ADR: "three ADRs for one unbroken line would guarantee the third contradicts the first" |
| Checking whether the docs build | Assuming `mdbook build` can't run here (as some earlier phases assumed for Docker/cargo-audit) | `mdbook build docs/` — verified runnable this session, see Verification Mechanics | Guessing "CI-only" for a tool that is in fact installable offline from the cached registry produces a weaker close-out than Phase 13's D-19 asks for ("build the book or state why it could not") |

**Key insight:** every mechanical question this phase raises (ledger shape, ADR shape, annotation
shape, trajectory-table shape) has already been answered by a prior phase in this exact corpus. The
research risk in this phase is not "what's the right pattern" — it's "did I re-verify the citation I'm
about to transcribe, and did I re-run the command I'm about to cite," per D-00e and the corpus's own
repeated finding that stale citations propagate silently (D-04, D-08, D-09, D-17, D-18 are all
instances of exactly this failure mode inside CONTEXT.md's own source material).

## Common Pitfalls

### Pitfall 1: Treating the ledger's "Verdict" column as needing an "Evidence" column
**What goes wrong:** `milestone-07-08.md`'s own head note (§Provenance, §ledger-file-contention table)
says "replace Verdict and Evidence cells in place," implying a three-column table, but every actual
row in the file is two columns (`ID | Verdict`), with the citation as prose inside the Verdict cell.
**Why it happens:** the head note was written aspirationally and the actual row-writing plans
converged on the simpler two-column shape that matches `REQUIREMENTS.md`'s own pre-existing ledger.
**How to avoid:** follow the two-column shape (matches Phase 13's own starting-point draft ledger at
`REQUIREMENTS.md:3607-3931` exactly) — do not introduce a third column mid-series.
**Warning signs:** a plan's `must_haves` mentioning "Evidence cell" as if distinct from "Verdict cell."

### Pitfall 2: Re-deriving the four D-09 `.project/` annotations as if they don't exist yet
**What goes wrong:** D-09's own text ("What remains true is the documentation half... still name the
undotted path") reads as if the four Milestone 12 requirement-text corrections are still open work.
**Why it happens:** D-00c's convention is to retain the original (stale) text alongside a correction —
so the stale text is, by design, "still there," which is easy to misread as "still uncorrected."
**How to avoid:** the four `.project/` files (Epic 1 §7, Epic 5 §7, Epic 6, Epic 7) **already carry**
dated `Corrected (dated 2026-08-06, DEBT-01)` banners with struck original text and corrected inline
text — confirmed by direct read this session at each file. **`REQUIREMENTS.md`'s existing draft ledger
rows for these four requirements (lines 3813, 3860, 3871, 3881) already cite this closure verbatim.**
ORCH-01's job for these four rows is transcription into the new ledger, not new `.project/` writing.
**Warning signs:** a plan proposing to edit `.project/Milestone_12-Web-API/Epic_{1,5,6,7}/*.md` for
this specific defect — that edit already happened.

### Pitfall 3: Assuming the environment can't build the mdbook, and skipping D-19's "or state why not"
**What goes wrong:** prior phases (4, 9, 10, 12) established that `cargo audit`, `cargo deny`,
`cargo llvm-cov`, and anything Docker are unrunnable here (crates.io 403, docker absent) — it is easy
to over-generalize this to "the build toolchain is generally offline-blocked" and skip attempting
`mdbook build` entirely.
**Why it happens:** the unrunnable-tools list is long and well-established by precedent; `mdbook` was
never on it because no prior phase touched `docs/src/`.
**How to avoid:** `mdbook` 0.4.40, `mdbook-mermaid` 0.13.0, and `mdbook-linkcheck` 0.7.7 — the exact
versions `.github/workflows/docs.yml` pins — **all install successfully via `cargo install --offline`**
because their crates are pre-cached in `/usr/local/cargo/registry/cache/`. Confirmed this session
(mdbook: 2m30s; mdbook-mermaid: 1m57s; mdbook-linkcheck: 4m11s, all exit 0). Attempt the real build.
**Warning signs:** a close-out task that states "mdbook build is CI-only" without having tried it.

### Pitfall 4: Attributing a failed `mdbook build` to this phase's own three-line edit
**What goes wrong:** `mdbook build docs/` (the exact CI command) **fails today, on the unmodified
tree, before any Phase 13 edit** — exit code 101, one broken-link error (`deployment/docker.md:118`,
a link outside the mdbook root directory) and one incomplete-reference-link error
(`user-guides/tool-integration.md:324`). Both defects are pre-existing, in files this phase does not
touch, introduced by commits dated 2026-07-10 and 2026-08-07 — **after** the last successful `docs.yml`
CI run (2026-07-06, confirmed via `gh run list --workflow=docs.yml`). `docs.yml`'s `push: [main]`
trigger has not fired since, because this work has been happening on `release/v0.7.0` and phase
branches, not merges to `main` — so nothing has caught these two breaks yet.
**Why it happens:** a close-out that runs `mdbook build`, sees it fail, and doesn't check whether the
failure predates the phase's own edits would wrongly conclude either "my fix broke it" (false) or
silently claim a pass it didn't earn.
**How to avoid:** run `mdbook build docs/` **before** making the three D-19 edits (this session's run
constitutes that baseline) and again **after**, and diff the two failure sets. The three D-19 edits
(`sidecar.md:29`, `http-service-host.md:54`, `overview.md`'s table) do not touch either broken-link
site, so both errors are expected to persist unchanged after this phase's edits — record that
explicitly rather than let a failing build read as "this phase broke the docs."
**Warning signs:** a close-out claiming "mdbook build passes" without a re-run this session, or a
close-out claiming "mdbook build fails because of this phase's edits" without checking file paths.

### Pitfall 5: Missing that SUPPLY-01's pending trigger has already fired
**What goes wrong:** D-06 (13-CONTEXT.md, written 2026-08-10 earlier in the session that produced
CONTEXT.md) states the SUPPLY-01 trigger — "the next push to `release/v0.7.0`" — has "never had the
opportunity to fire," citing the newest run as `30861568499` (2026-08-03).
**Why it happens:** a CI run newer than that citation landed **after** CONTEXT.md was written but
**before** this research session: `gh run list --workflow=ci.yml --branch release/v0.7.0` returns run
`31320378772`, dated 2026-08-09T15:10:05Z, `completed`/`success`, whose `Security Audit` job
(singular — no duplicate-named job, confirmed via `gh run view 31320378772 --json jobs`) is itself
`success`.
**How to avoid:** the planner should treat D-06's "pending, trigger not yet fired" framing as
**stale at the moment of planning** and record the trigger as fired: cite run `31320378772` (newer
than `30861568499`) with its `Security Audit` job's `success` conclusion. This closes the one
genuinely-pending clause D-06 flagged, with a live citation rather than a forward promise. Re-run
`gh run list --workflow=ci.yml --limit 3 --branch release/v0.7.0` at actual execution time, since more
runs may land between this research pass and execution.
**Warning signs:** a ledger row or ADR citing `30861568499` as "the newest run" without re-running
`gh run list` at write time — this is exactly the stale-citation failure mode D-04/D-08/D-09/D-17/D-18
already demonstrate elsewhere in this same phase's own source material.

### Pitfall 6: Missing PROMOTION.md's own second Phase-13-owned Part B candidate
**What goes wrong:** D-20 allocates exactly three ADRs (0037-0039) against ORCH-03(a), ORCH-04(a),
ORCH-04(b). But `PROMOTION.md`'s own Part B inventory (re-read this session, entries 8 and 9) assigns
**two** candidates to "Owner phase: Phase 13" — entry 8 is `AgentProvisioner` placement (covered by
ADR-0038, matches D-14) but **entry 9 is `Milestone_9/Epic_4/prd-agent-orchestrator-bridge.md` §6.1**
(the four-criterion, `(CHOSEN)`-column bridge decision — described in `PROMOTION.md` itself as "the
cleanest ADR-shaped section anywhere in the corpus"), grouped with entry 8 under "the same Milestone
9-12 close-out phase" **by `PROMOTION.md`'s own Part B note**, not by any ORCH-01…05 `Derives` list
(none of the five requirements' `Derives` blocks in `REQUIREMENTS.md:2204-2315` cites this PRD
section — confirmed by grep this session).
**Why it happens:** `PROMOTION.md`'s inventory assignment and `13-CONTEXT.md`'s locked ADR allocation
were written independently and disagree on scope for this one candidate.
**How to avoid:** **do not silently promote a fourth ADR** — `13-CONTEXT.md`'s D-20 is locked and
explicit ("ADR allocation — 0037 through 0039… ORCH-01, ORCH-02 and ORCH-05 get no ADR"), and no
ORCH requirement's `Derives` list reaches this PRD section, so there is no requirement-level mandate
to promote it this phase. The correct action is almost certainly the one `PROMOTION.md`'s own
advancing-note convention already demonstrates for a non-closed entry (Phase 11's note: "Neither ADR
closes an entry in Part B's eleven-candidate inventory below") — **state explicitly in Phase 13's own
advancing note that Part B candidate 9 is not promoted this phase**, and either leave its "Owner
phase: Phase 13" assignment as a forward-looking note for a future phase or redirect it (Phase 14 is a
natural fit, since WEB-01/WEB-02 already sit in the same M9 Epic 4/5 neighborhood as candidate 10).
This is a judgment call for the planner/human, not a locked decision — flagging it here satisfies the
"report tree evidence, don't silently re-decide" bar.
**Warning signs:** a close-out that updates `PROMOTION.md`'s next-free line to `0040` without any note
addressing entries 8 *and* 9, or a plan that promotes a fourth ADR without this being asked for by any
locked CONTEXT.md decision.

## Runtime State Inventory

**N/A — this is not a rename/refactor/migration phase.** The four path relocations ORCH-03(b)-(e)
records (`listener_service.rs`, `llm_port.rs`, `Design_and_Architecture.md`, the README) **already
happened** in prior milestones (M5/M6/M11); Phase 13 only records that they happened, in a ledger row
plus a `.project/` annotation. No file is moved, no database key is renamed, no service is
re-registered by this phase's own work. The five-category inventory (stored data / live service
config / OS-registered state / secrets / build artifacts) does not apply because nothing in Phase
13's own D-19 edit surface (`.planning/`, `.project/`, three doc lines) constitutes a rename target.

## Code Examples

Exact current content of the three files this phase's D-19 boundary permits editing, re-read this
session so the planner has the precise before-state:

### `docs/src/deployment-topologies/sidecar.md:26-30` (the D-12 fix site)

```markdown
## The two sides

- **Server side** — the agent runs behind the [HTTP service host](http-service-host.md)
  exactly as documented there (`POST /agents/{id}/execute`), deployed as its own process or
  container.
```

Fix: `POST /agents/{id}/execute` → `POST /v1/agents/{id}/execute` (line 29's parenthetical only —
the surrounding prose is unaffected).

### `docs/src/deployment-topologies/http-service-host.md:45-55` (the D-15 diagram fix site)

The file wraps this in a mermaid fence (`` ```mermaid `` … `` ``` ``); the sequence content is:

```
sequenceDiagram
    participant Client
    participant Server as paladin-server
    participant Service as PaladinExecutionService
    participant Agent as Paladin
    Client->>Server: POST /v1/agents/{id}/execute  (X-API-Key / Bearer)
    Server->>Server: authenticate + authorize (allowed_roles)
    Server->>Service: execute(agent, input)
    Service->>Agent: run (LLM + tools + memory)
    Agent-->>Service: PaladinResult
    Service-->>Server: output
```

Fix: line 54, `Service->>Agent: run (LLM + tools + memory)` — remove the capability claim (Garrison
memory and Arsenal tools are not present on this topology) while keeping the diagram's flow intact,
e.g. `Service->>Agent: run (LLM + prompt)`, per D-15's own reasoning that the honest answer routes a
Garrison/Arsenal-needing consumer to the embedded-library topology instead.

### `docs/src/deployment-topologies/overview.md:13-19` (the D-15 comparison-table fix site)

```markdown
## The five topologies at a glance

| Topology | Process model | Concurrency | Use when | Avoid when | Key crates / features |
|---|---|---|---|---|---|
| [Embedded library](embedded-library.md) | One process, agents in your `main` | ... | ... | You need an external caller or independent scaling | `paladin-ai` |
| [Battalion orchestration](battalion-orchestration.md) | ... |
| [HTTP service host](http-service-host.md) | One long-running process, agents resident behind an API | Concurrent requests over a shared agent registry | You need request/response access to many agents | A single embedded call already suffices | **`paladin-server`** ... |
| [Queue / worker (distributed)](queue-worker.md) | ... |
| [Sidecar (separate process)](sidecar.md) | ... |
```

The HTTP-service-host row's `Avoid when` cell currently says only "A single embedded call already
suffices" — this is where D-15's limitation statement lands (per Claude's Discretion: table cell, a
note under the table, or both), analogous to how `embedded-library.md:31-32` is the only file under
`deployment-topologies/` that currently mentions Garrison/Arsenal at all (confirmed this session,
`grep -in "garrison\|arsenal" docs/src/deployment-topologies/*.md` → one file, `embedded-library.md`).

## State of the Art

Not applicable in the conventional sense (no library/framework version drift to track) — but the
corpus's own internal "state of the art" is worth stating for this phase specifically:

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| ORCH-01's own text: "16 settled-by entries, remaining 104 need treatment" | All 120 rows need a verdict; 35/53/32 split | D-04, this CONTEXT session, 2026-08-10 | Wrong population conflated (variant-register vs. ledger rows) |
| ORCH-05's own text: "`Cargo.toml` 0.6.0, latest tag v0.5.1" | `Cargo.toml` 0.7.0, latest tags v0.7.1/v0.7.0 | D-18, same regrowth pattern Phase 10's D-11 already fixed once in HARD-03 | A phase-13-later reader would plan against a two-release-stale baseline |
| SUPPLY-01's pending trigger, "never fired" (D-06, written earlier this same day) | Fired — run `31320378772`, 2026-08-09, `success` | This research session, 2026-08-10 | The one genuinely-open clause in the whole SUPPLY-01/02/03 closure set is now closeable with a live citation |
| ORCH-01's "sixteen already have it" | Zero ledger rows carry `settled-by` — it's a variant-register mechanism | D-04 | — |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The two-column (`ID | Verdict`) ledger row shape, not three-column, is correct for the new ledger | Ledger Format Contract | Low — directly matches both `milestone-07-08.md`'s actual rows and `REQUIREMENTS.md`'s existing draft ledger; the only counter-signal is one head-note sentence in `milestone-07-08.md` that appears to be an editorial artifact |
| A2 | ADR-0029's spanning placeholder row should be struck/superseded-in-place rather than deleted outright when the four new trajectory rows are inserted | ADR-0029 Trajectory Table Mechanics | Low-medium — D-16 doesn't specify the placeholder row's disposition explicitly; deleting it outright would violate D-00d's general retain-superseded-text convention, but the row is arguably editorial scaffolding rather than a historical fact, so a planner could reasonably choose either — flagged as Claude's Discretion-adjacent, not fully locked |
| A3 | PROMOTION.md Part B candidate 9 (the M9 Epic 4 bridge decision) should NOT be promoted as a fourth ADR this phase | Common Pitfalls §6 | Medium — if a human intended D-20's three-ADR allocation to implicitly exclude candidate 9 deliberately (rather than by oversight), no action is needed beyond the advancing-note explanation; if the omission was accidental, a future phase or this phase (with human confirmation) may need to promote it |

**If this table is empty:** not applicable — see above.

## Open Questions

1. **Does ADR-0029's placeholder-row disposition (strike vs. delete) matter enough to gate on a
   checkpoint?**
   - What we know: D-16 says "append… without re-sorting or re-keying the existing rows," which reads
     as protective of the *other* four historical rows, not necessarily the placeholder.
   - What's unclear: whether "existing rows" was meant to include the placeholder itself as untouchable.
   - Recommendation: strike/supersede in place per D-00d's general convention (Assumption A2); this is
     low-risk enough not to need a `checkpoint:human-verify`, since either disposition produces an
     ascending, non-contradictory table and the underlying historical facts (four lockstep gates) are
     unaffected either way.

2. **Should PROMOTION.md Part B candidate 9 be addressed in this phase's own advancing note, even
   though it isn't promoted?**
   - What we know: `PROMOTION.md`'s own precedent (Phase 11's note) is to explicitly state when an
     ADR-authoring phase does *not* close an inventory entry, rather than leaving the reader to notice
     the gap themselves.
   - What's unclear: whether CONTEXT.md's D-20 silently intended to exclude candidate 9, or whether
     this is a genuine gap between two independently-written planning documents.
   - Recommendation: the close-out plan should add one sentence to the advancing note stating candidate
     9 is not promoted this phase and why (no ORCH `Derives` list reaches it), consistent with the
     corpus's own transparency convention — this needs no human checkpoint, it's a documentation
     completeness fix, not a scope change.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|--------------|-----------|---------|----------|
| `git`, `grep`, `sed`, `awk` | Every ledger row, every ADR citation | ✓ | system | — |
| `gh` CLI (authenticated) | D-06's SUPPLY-01 trigger citation, D-21's `gh run`/`gh api` reads | ✓ | 2.96.0, logged in as `Am0rfu5` | — |
| `mdbook` 0.4.40 | D-19's "build the book or state why it could not" | ✓ (installed this session via `cargo install --offline`, matches `docs.yml`'s pinned version exactly) | 0.4.40 | — |
| `mdbook-mermaid` 0.13.0 | Same | ✓ (installed this session, offline) | 0.13.0 | — |
| `mdbook-linkcheck` 0.7.7 | Same — the `warning-policy = "error"` gate `book.toml` sets | ✓ (installed this session, offline) | 0.7.7 | — |
| `cargo audit` / `cargo deny` | Not required by this phase (no `.rs`/manifest changes) — noted for completeness since D-21 asks the check be re-confirmed | binaries present but network-dependent (crates.io → HTTP 403, unchanged since Phase 4) | cargo-audit 0.22.2 | Not needed — no advisory data changes this phase |
| Docker | Not required by this phase | ✗ (absent, unchanged since Phase 4) | — | Not needed |
| `cargo-llvm-cov` | Not required by this phase (zero `.rs` diff) | Not tested this session — Phase 10 precedent (`10-11-SUMMARY.md`) records it uninstallable | — | Not needed — D-19: "a phase that changes no `.rs` file cannot move coverage," recorded as reasoning not measurement |

**Missing dependencies with no fallback:** none — every dependency this phase actually needs is
available.

**Newly available this session (not available/attempted in prior phases):** `mdbook`,
`mdbook-mermaid`, `mdbook-linkcheck` all install successfully offline from the pre-cached cargo
registry (`/usr/local/cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/`). Install commands (each
run this session, each exits 0):

```bash
cargo install mdbook --version 0.4.40 --offline
cargo install mdbook-mermaid --version 0.13.0 --offline
cargo install mdbook-linkcheck --version 0.7.7 --offline
```

**`mdbook build docs/` result, run against the unmodified tree this session:** exit code **101**.
Two errors, both pre-existing and both in files this phase does not touch:

```
error: Linking outside of the "root" directory is forbidden
    ┌─ deployment/docker.md:118:61
    │ ...(../../../.planning/decisions/0023-cli-dependency-isolation.md)...

error: Potential incomplete link
    ┌─ user-guides/tool-integration.md:324:1
    │ [`MCPClient::connect_streamable_http`]
    │ = hint: declare the link's URL.
```

Both commits post-date the last successful `docs.yml` CI run (`gh run list --workflow=docs.yml`:
newest is 2026-07-06T13:23:05Z, `success`; `docker.md` last touched 2026-08-07T15:18:13+00:00,
`tool-integration.md` last touched 2026-07-10T18:26:28+00:00 — both after the last green run, and
`docs.yml`'s `push: [main]` trigger has not fired since, because the intervening work landed on
`release/v0.7.0`/phase branches rather than `main`). **This means `docs.yml`'s CI gate would fail
today if it ran** — a fact independent of Phase 13 and not discovered by any prior phase (none touched
`docs/src/`). Record this in the close-out per D-19 exactly as it is here: pre-existing, unrelated to
this phase's three-line edit surface, confirmed by file-path and commit-date cross-check.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | None — direct shell verification (`grep`, `sed`, `awk`, `git log`, `git diff`, `gh run`/`gh api`, `mdbook build`), following Phase 10's `10-VALIDATION.md` precedent verbatim |
| Config file | none |
| Quick run command | Per-claim `grep -n "<pattern>" <file>` / `sed -n '<range>p' <file>` — the exact command the ledger row or ADR cites |
| Full suite command | Re-run every row of the Phase Requirements → Test Map below |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|--------------------|--------------|
| ORCH-01 | Ledger exists, exactly 120 rows, 24-section order | row-count | `grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` → `120` | ❌ Wave 1 |
| ORCH-01 | No row left as bare `Verify` | grep | `grep -c 'Verify$' .planning/ledgers/milestone-09-12.md` (or equivalent trailing-marker check) → `0` | ❌ Wave 1 |
| ORCH-01 | REQUIREMENTS.md's own section is a pointer, not a duplicate | grep | `grep -n "Milestone 9-12 as-shipped ledger" .planning/REQUIREMENTS.md` — section body reduced to a one-line pointer | ❌ Wave 1 close-out |
| ORCH-01 | SUPPLY-01's pending clause resolved with a live citation | gh | `gh run list --workflow=ci.yml --limit 3 --branch release/v0.7.0` → newest run's date newer than `2026-08-03`, conclusion `success` | ✅ (confirmed this session, run `31320378772`) |
| ORCH-02 | Five checkbox verdicts recorded once, in the ledger head note | grep | `grep -c "M9.*0 open\|M10.*0 open\|M11.*26 open\|M12.*3 open\|project-management.*1 open"` — presence, not absence | ❌ Wave 1 |
| ORCH-03(a) | Route surface confirmed `/v1` against committed baseline | grep | `grep -c '"/v1/agents' crates/paladin-web/openapi.json` → `≥1` (already true, re-confirm) | ✅ |
| ORCH-03(a′) | `sidecar.md:29` corrected | grep | `grep -n 'POST /v1/agents/{id}/execute' docs/src/deployment-topologies/sidecar.md` → hit; `grep -n 'POST /agents/{id}/execute'` (unprefixed) → no hit | ❌ Wave 3 |
| ORCH-03(b)-(e) | Four stale paths corrected at source | grep | Per-path: old path absent (`ls <old-path>` fails), new path present, `.project/` banner exists | ❌ Wave 3 |
| ORCH-04(a) | ADR-0038 exists, `conforms`, names Phase 14 | file+grep | `ls .planning/decisions/0038-*.md`; `grep -A2 '## Code Conformance' 0038-*.md` → `conforms`; `grep -A3 '## Downstream Consumers'` → mentions Phase 14 | ❌ Wave 3 |
| ORCH-04(b) | ADR-0039 exists, `http-service-host.md:54` no longer promises tools+memory, `overview.md` states the limitation | file+grep | `ls .planning/decisions/0039-*.md`; `grep -n 'tools.*memory\|memory.*tools' docs/src/deployment-topologies/http-service-host.md` → no hit; `grep -in 'garrison\|arsenal' docs/src/deployment-topologies/overview.md` → ≥1 hit | ❌ Wave 3 |
| ORCH-05 | ADR-0029's Trajectory table carries the four new rows in ascending order | grep | `grep -c '| v0\.[3-6]\.0 |' .planning/decisions/0029-version-trajectory-history.md` → `4` | ❌ Wave 4 |
| ORCH-05 | ADR-0030 cited, prediction confirmed closed, not re-decided | grep | `grep -n 'ADR-0030' .planning/ledgers/milestone-09-12.md` or ORCH-05's own text — presence | ❌ Wave 4 |
| D-19 boundary | Zero `.rs` files modified | git diff | `git diff --name-only <base>..HEAD -- '*.rs' \| wc -l` → `0` | ❌ Close-out |
| D-19 boundary | mdbook build attempted, result recorded (pass or documented pre-existing failure) | build | `mdbook build docs/` → exit code recorded verbatim, cross-checked against the pre-phase baseline (this research's own run, exit 101, two pre-existing errors) | ✅ baseline established this session |
| PROMOTION.md | Next-free advances to 0040, advancing note explains all of 0037-0039 (and states candidate 9's disposition) | grep | `grep -n "Next free ADR number" .planning/decisions/PROMOTION.md` → `0040` | ❌ Close-out |

### Sampling Rate

- **Per task commit:** re-run the specific `grep`/`sed`/`git log`/`gh` command that task's ledger row
  or ADR cites, before marking its cell/section complete — the same D-00e bar every prior sibling
  phase applied.
- **Per wave merge:** re-run the 120-row count and the per-section row counts (table above) against
  the in-progress ledger, to confirm the parallel wave-2 fan-out (5 plans across 24 sections) dropped
  or duplicated nothing — mirrors Phase 10's own wave-3 integrity check exactly.
- **Phase gate:** the close-out plan re-runs the full Phase Requirements → Test Map above, plus the
  `git diff --name-only … -- '*.rs' | wc -l` → `0` assertion and the `mdbook build` before/after
  comparison.

### Wave 0 Gaps

None — there is no test suite to scaffold. Every "test" above already runs in this environment (this
research session is itself the Wave 0 proof: every command in the table was executed and its result
recorded). One item (`mdbook build`) currently returns a **failing** result for reasons unrelated to
this phase, which the close-out must record honestly rather than silently reinterpret as a pass or
attribute to this phase's own edits (Common Pitfall 4).

## Decomposition Precedent

Phase 10's actual executed shape (11 plans, 4 waves, 86 rows across 12 sections) — read in full via
all eleven PLAN frontmatter blocks this session — confirms D-23's proposed Phase 13 shape (~11 plans,
4 waves, 120 rows across 24 sections) is structurally sound, with these specific correspondences:

- **Wave 1 (parallel, no dependencies):** Phase 10 ran 4 plans in wave 1 — one ledger scaffold
  (10-01, the whole file: head notes, legend, summary table, all section headings, all row stubs) and
  three ADR-authoring plans (10-02 M8 reconciliation ADR, 10-03 version/numbering ADRs, 10-04 the
  extracted-crate-dependency ADR) — each `files_modified` disjoint from the ledger file. **Phase 13's
  D-23 wave 1 (the ledger scaffold alone) matches this shape for the scaffold half; its three ADRs
  (0037-0039) are assigned to wave 3 instead of wave 1 in D-23's proposal** — this is a deliberate
  divergence from Phase 10's shape, not an oversight: Phase 13's ADR-0038/0039 need the
  `AgentProvisioner`/Garrison-Arsenal ⚠ HUMAN REVIEW checkpoint (D-23's wave 3 gate), which Phase 10's
  ADRs did not need, so running them later, gated behind a checkpoint, while the ledger scaffold
  proceeds in parallel in wave 1, is consistent with — and an improvement on — Phase 10's own shape.
- **Wave 2/3 fan-out (parallel, blocked on scaffold):** Phase 10 ran 4 plans (10-07 through 10-10)
  each owning 2-4 disjoint, contiguous epic sections, replacing only Verdict/Evidence cells in place —
  confirmed via the ledger-file-contention table read this session. **Phase 13's D-23 proposes 5
  fan-out plans (②-⑥) for its 24 sections** — proportionally consistent (Phase 10: ~3 sections/plan
  for 12 sections in 4 plans; Phase 13: ~5 sections/plan for 24 sections in 5 plans).
- **Wave 4 close-out (single plan, depends on everything):** Phase 10's 10-11 flips checkboxes,
  updates `PROMOTION.md`, writes forward hand-offs, appends the dated close-out amendment section, and
  runs the whole-phase gate (`.rs`-diff count, `cargo test`/`fmt`/`clippy`, `cargo metadata`,
  `make -n release-check`, and named CI-only claims with their exact commands). **Phase 13's D-23 wave
  4 (⑩ ORCH-05, ⑪ close-out) matches this shape**, with the addition that Phase 13's close-out gate
  should also run the `mdbook build` before/after comparison (Common Pitfall 4) — a check Phase 10's
  gate did not need since it never touched `docs/src/`.
- **File contention discipline:** confirmed working as designed in Phase 10 — `10-11-SUMMARY.md`'s own
  integrity check (`grep -c '^| REQ-'` → `86` before and after every wave-3 plan, `git diff --numstat`
  showing added lines equal deleted lines) is the exact mechanism to re-run for Phase 13's 120-row
  ledger, scaled up. No friction is recorded in any of the ten Phase 10 SUMMARY files this session
  reviewed beyond the already-CONTEXT.md-documented 12-vs-13 supersession-count reconciliation (which
  Phase 10's own close-out fully explained rather than left as an unresolved discrepancy — Phase 13
  should budget similar reconciliation prose for its own summary-table-vs-epic-section counts, per the
  independently-verified 35/53/32 split above, which already anticipates and resolves the equivalent
  question before it's asked).

## Sources

### Primary (HIGH confidence — read/executed directly this session)

- `.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-CONTEXT.md` — full read, all
  23 decisions, both ⚠ HUMAN REVIEW flags, all five fresh findings, canonical refs, deferred list.
- `.planning/REQUIREMENTS.md:2084-2315` (hand-off blocks + ORCH-01…05 requirement text),
  `:3607-3931` (the 120-row draft ledger — counted and classified directly), `:4251-4255`,
  `:4330-4338`.
- `.planning/ledgers/milestone-07-08.md` (full read, 549 lines) — the structural template.
- `.planning/ledgers/milestone-01.md`, `milestone-04-06.md` — amendment-convention confirmation.
- `.planning/decisions/0036-audit-suppression-single-source-topology.md`,
  `0031-extracted-crate-dependency-rule.md`, `0029-version-trajectory-history.md` — full reads, ADR
  shape and the Trajectory table's exact current state.
- `.planning/decisions/PROMOTION.md` — full read, numbering index, promotion procedure, **Part B
  candidate 9 finding**.
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-*-PLAN.md` (all 11 frontmatter
  blocks) and `10-11-SUMMARY.md`'s close-out amendment section (full read) — decomposition precedent.
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-VALIDATION.md` — Validation
  Architecture template for a records phase.
- `docs/book.toml`, `docs/src/deployment-topologies/{sidecar,http-service-host,overview}.md` — exact
  current content read directly.
- `.project/Milestone_12-Web-API/Epic_{1,5,6,7}/*.md` — grep-confirmed D-09 annotations already exist.
- `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md:290-292,327-329` — D-14's
  §7/OQ-2 citations re-verified exact.
- `.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md:269-271` — Open Question 4.
- `.project/Milestone_12-Web-API/Epic_{2,3}/*.md` — Garrison/Arsenal non-goal text re-verified.
- `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md:217`
  — FR-8.
- `git tag --sort=-v:refname`, `git branch --show-current`, `git log` — version state re-verified.
- `gh run list --workflow=ci.yml --branch release/v0.7.0`, `gh run view 31320378772 --json jobs`,
  `gh api repos/:owner/:repo/rulesets`, `gh api repos/:owner/:repo/branches/main/protection`,
  `gh run list --workflow=docs.yml` — all executed this session.
- `cargo install mdbook{,-mermaid,-linkcheck} --offline` and `mdbook build docs/` — executed this
  session, both before-state confirmed.
- `cargo audit --version`, `curl -o /dev/null -w '%{http_code}' https://crates.io` (→ 403) — confirmed
  unchanged from prior-phase precedent.

### Secondary (MEDIUM confidence)

- None — every claim in this document was either executed directly this session or is a direct
  citation to a CONTEXT.md decision whose own file:line was independently re-checked.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**
- Ledger/ADR mechanics: HIGH — read directly from four sibling ledgers and three ADRs this session,
  cross-checked against `PROMOTION.md`'s own stated rules.
- 120-row inventory counts: HIGH — independently computed via `grep`/`awk`, reconciled exactly against
  D-04's headline figures once the two-row parenthetical-qualifier boundary case is resolved.
- Environment mechanics (`mdbook`, `gh`): HIGH — executed directly, not inferred from precedent.
- The PROMOTION.md Part B candidate-9 gap (Pitfall 6): MEDIUM — a genuine, independently-discovered
  finding, but its correct resolution is a judgment call this research flags rather than settles.

**Research date:** 2026-08-10
**Valid until:** This phase's own execution — several facts here (the SUPPLY-01 trigger CI run, the
`docs.yml` last-green-run date, the `mdbook` toolchain's offline-installability) are timestamped
findings that could shift if execution happens materially later than this research pass. Re-run the
`gh run list` commands and the `mdbook build` baseline at execution time if more than a few days
elapse.
