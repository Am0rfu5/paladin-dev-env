# Phase 7: Workspace Ground Truth & Recorded Answers - Pattern Map

**Mapped:** 2026-08-06
**Files analyzed:** 7 artefact classes (~24 individual files: 1 ledger, 7-8 ADRs, REQUIREMENTS.md
edit, STRUCTURE.md edit, PROMOTION.md edit, 11+ `.project/` annotations)
**Analogs found:** 7 / 7 classes (every class has a complete, working prior instance in this repo)

**Read this file's framing note first:** this is a records phase. There is no Rust source, no
role/data-flow classification in the conventional sense. Each artefact class below is mapped to
the *prior instance of the same document class* — the "pattern" is markdown structure (head notes,
row format, heading set, banner markup), not code architecture.

## File Classification

| New/Modified File | Document Class | Closest Analog | Match Quality |
|---|---|---|---|
| `.planning/ledgers/milestone-04-06.md` | cited status ledger | `.planning/ledgers/milestone-02-03.md` (118 rows) | exact — same generation, same phase family |
| `.planning/decisions/0014-*.md` … `0020-*.md` (7 files) | ADR | `.planning/decisions/0001`-`0013` (esp. `0008`, `0009`, `0010`, `0011`) | exact |
| `.planning/decisions/0021-*.md` (conditional, D-25a) | ADR | same as above | exact, if authored |
| `.planning/REQUIREMENTS.md` §"Milestone 4-6 as-shipped ledger" (2830-3069) | pointer-reduction edit | same file's own §"Milestone 2-3 as-shipped ledger" (2817-2828), already reduced by Phase 5 | exact — same file, same edit class, done by a prior phase |
| `.planning/codebase/STRUCTURE.md` "Directory Purposes" (~250-285) | codebase-map prose correction | the same section's existing 6-crate entries (self-analog: extend the existing pattern, no external analog needed) | exact |
| `.planning/decisions/PROMOTION.md` numbering index | index-table append | its own existing rows (0001-0013) + "Next free ADR number" line | exact — self-analog |
| 11+ `.project/` documents | dated correction banner + inline annotation | `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` (Phase 5 / ADR-0010's D-08 pattern) | exact |

## Pattern Assignments

### 1. `.planning/ledgers/milestone-04-06.md`

**Analog:** `.planning/ledgers/milestone-02-03.md` (full file read; 118 rows, closest in scale to
this phase's 115). Secondary analog: `.planning/ledgers/milestone-01.md` (the first instance,
established the shape `milestone-02-03.md` continues).

**Head note structure to copy, in this fixed order** (verbatim shape from
`milestone-02-03.md:1-49`):
1. `# Milestone 4-6 cited status ledger` (H1 title)
2. **Supersession paragraph** — names the REQUIREMENTS.md section this file supersedes, cites the
   decision (`D-26` here, `D-21` there), names the sibling files other phases add:
   ```markdown
   This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 4-6 as-shipped ledger` section
   (D-26). That section becomes a pointer to this file. Phases 10 and 13 each add a sibling
   ledger (`milestone-07-08.md`, `milestone-09-12.md`)...
   ```
3. **Primary-key paragraph** (D-00e) — copy verbatim, only the row/ID count changes:
   > "**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
   > requirement they belong to, not given their own identifiers..."
4. **Evidence-bar paragraph** (D-01) — copy the shape, but this phase's version MUST add the
   manifest carve-out sentence D-01 introduces (`milestone-02-03.md` has no such carve-out; do not
   drop it silently — it is new content, not boilerplate):
   ```markdown
   **Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named
   passing test, example, or command that exercises it... **Carve-out for structural
   requirements** (edition, feature-flag shapes, dependency lists, `required-features`, workspace
   membership): the manifest line **plus** a named CI job or build leg that consumes it satisfies
   the bar — e.g. `crate-isolation` (`ci.yml:304` — re-grep this at authoring time, it drifted
   from 228) and `feature-flags.yml`'s workspace matrix (`:115,118,141`).
   ```
5. **Path-caveat paragraph** (D-04) — two caveats stated once, matching the structure of
   `milestone-02-03.md`'s own path caveat (`(a)` internally-historical `src/…` paths since M6 moved
   what M5 placed; `(b)` the four mdbook-relocated docs).
6. `## Verdict legend` (H2) — table with **seven** rows (this phase's D-02 vocabulary), not five:
   `satisfied` · `present, unproven` · `genuinely outstanding` · `deferred with reason` ·
   `superseded by shipped code` · `relocated` · `diverged`. Copy the five shared definitions
   verbatim from `milestone-02-03.md`'s legend table; write two new rows for `relocated` and
   `diverged` per D-02's own definitions in CONTEXT.md.
7. `## Row order and amendment convention` (H2) — copy verbatim structure: fixed epic order
   (never re-sorted — for this phase, the 13 headings in REQUIREMENTS.md's existing M4/M5/M6
   order), in-place amendment rule citing D-00f/D-00g, same-citation-different-rows note.
8. `## Summary` section — **written last**, by the final plan, "every count... produced by
   counting the rows in this file — none is re-judged." Copy this self-discipline sentence
   verbatim; it is a load-bearing anti-pattern guard (see Common Pitfall 2 below).

**Row format** (exact table shape, `milestone-02-03.md:230-259` and similar):
```markdown
| REQ-<id> | <verdict> | <file:line citation>; exercised by <command> — <N/M> passed, run during
this task. <Divergence/caveat prose if any> |
```
- Table header is exactly `| ID | Verdict | Evidence |` per epic section (confirm against
  `milestone-02-03.md`'s actual per-section header — reuse verbatim).
- Evidence cells are long-form prose, not terse — they carry the citation, the exercising
  command, its pass count, and any divergence note inline in the same cell. Do not split
  divergence notes into a second column.
- **Nested outstanding items** use a blank-first-two-column row directly beneath the parent
  `REQ-*` row (see `milestone-02-03.md`'s "Nested outstanding items" section, count: 2) — reserve
  this format only for genuinely separate outstanding items, not general findings, which fold
  inline into the host row's Evidence cell instead (12 "New finding" annotations in
  `milestone-02-03.md` took the inline route, only 2 took the nested-row route).

**Epic section headings**: `## Epic N — Name (K IDs)`, in REQUIREMENTS.md's own order — for this
phase: M4 Epics 1-3, M5 Epics 1-6, M6 Epics 1-4 (13 headings total, matching the 13-epic census in
RESEARCH.md).

**Amendment convention in practice** — `milestone-02-03.md:99-103` shows the exact "amended
2026-08-05, plan 06-07" pattern for later-phase row updates: retain the original row unchanged
immediately above, add a dated "Amended..." note explaining what changed and why, never delete.

### 2. `.planning/decisions/0014-*.md` through `0020-*.md` (and conditional `0021-*.md`)

**Analog:** `.planning/decisions/0010-milestone-3-epic-numbering.md` (full file read) for
ADR-0014 specifically (numbering-collision class, explicitly required by D-07 to be cited).
Secondary analogs: `0008-workspace-version-0-7-0.md` and `0009-workspace-rust-edition-2024.md`
(the "cite, don't re-decide" pattern for ARCH-03(a) and ARCH-04's version half) and
`0011-vision-port-surfaces.md` (the "both surfaces intentional, not competing" / "built, never
wired" disposition pattern, relevant background for ARCH-05(1)).

**Exact required heading set, in order** (from `PROMOTION.md`'s "Required heading set" section —
copy this list mechanically, no frontmatter per D-00h):
```markdown
## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```

**Hard rule, easy to get wrong:** `## Code Locations` and `## Considered Options` **must be
bulleted lists, never prose paragraphs**. `.claude/gsd-core/bin/lib/adr-parser.cjs`'s
`splitEntries` only yields structured entries from bullet/numbered lines — a paragraph collapses
into one opaque blob and silently defeats machine consumption. Every existing ADR (0001-0013)
observes this; `0010`'s `## Code Locations` is a clean bulleted list of `file:line` citations, one
per bullet, each self-contained enough to be read out of context.

**`## Status` sub-shape** (from `0010`):
```markdown
## Status

Accepted

**Date:** 2026-08-04
```

**`## Considered Options` sub-shape** — each option is a bullet stating the option and "—
rejected/accepted", with the reasoning inline, not a separate column:
```markdown
- The release-notes numbering as the authoritative set — rejected. It is one document against
  eight of the nine Milestone-3 documents...
```

**`## Code Conformance` sub-shape** — first line is the bare word `conforms` or `must change`;
if `must change`, name the executing requirement/phase in the following prose, per D-00c:
```markdown
## Code Conformance

conforms

This is a documentation defect, not a code defect...
```
For ADR-0016, ADR-0019 (both `must change` per D-25's table), the pattern to copy is `0009`'s or
any `must change` exemplar's phrasing: state the verdict word first, then name the phase/
requirement executing the change.

**`## Downstream Consumers` sub-shape** — bulleted list naming specific plans/phases and exactly
what they do with this ADR:
```markdown
## Downstream Consumers

- Phase 5 ledger plans 05-08 through 05-12 — the Epic 15... ledger rows... cite this ADR's
  mapping table when recording the release-notes attribution defect against each requirement.
- Plan 05-13 — advances `.planning/decisions/PROMOTION.md`'s "Next free ADR number" line past 0010.
```
For this phase: ADR-0016 must name Phase 8/DEBT-05 explicitly (blocked-on relationship, D-11);
ADR-0018 must name Phase 11/FACADE-02 D1 (D-16); ADR-0019 must name Phase 16 (mdbook page, D-21)
and Phase 8 (the `structopt` precondition, D-20); ADR-0015 must name Phase 15 (allowlist
enforcement candidate, D-10).

**"Cite, don't re-decide" cross-reference pattern** (ADR-0008/0009 precedent, applies to ARCH-03(a)
and the version half of ARCH-04/ADR-0018): copy `0009`'s own `## Downstream Consumers` entry shape:
```markdown
## Downstream Consumers

- **Phase 7's ARCH-03(a)** — the Rust edition half of ARCH-03's four competing-variant pairs.
  ARCH-03(a)'s requirement text is amended... to cite this ADR instead of re-adjudicating...
```
ADR-0018 must open its `## Context` by citing `0008`'s three-way version disagreement framing
almost verbatim, then add only the facade re-export policy as new scope Phase 4 did not touch.

**ADR promotion without `--manifest` re-tagging** (ADR-0016 specifically): this ADR is not a fresh
decision but the *promotion* of
`.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
(`Status: Approved`, already carries a Decision/Considered-Options/Rejected-Options shape). Per
`PROMOTION.md`'s five-step procedure: take the next number, author the substance into the standard
heading set, cite the source `.project/` path as an entry in `## Code Locations`, set
`Code Conformance`, update `PROMOTION.md`'s next-free line, add a `PROJECT.md` Key Decisions row.
No `--manifest` re-tagging of the source document is needed or wanted.

**Discretion note carried forward:** whether ADR-0015/0016/0017 are three files or one combined
file is left to the planner (CONTEXT.md Claude's Discretion); 0016 must stay separately citable by
number regardless, since Phase 8 depends on it by number (`ADR-0016`, not a section within a
combined file).

### 3. `.planning/decisions/PROMOTION.md` (numbering index append)

**Analog:** the file's own existing rows — self-referential pattern, no external analog needed.

**Exact row shape to append** (from the existing table, `PROMOTION.md:20-33`):
```markdown
| 0014 | `milestone-4-6-tier-numbering` | Milestone 4-6 milestone/tier numbering (Phase 7) |
```
(slug-subject-phase, matching the existing `0010`/`0011`/`0012`/`0013` rows' exact phrasing style
— slug is kebab-case, subject is a short noun phrase, phase is parenthetical at the end).

**Next-free-line edit:** the exact line to change is `**Next free ADR number: 0014**` →
`**Next free ADR number: 0021**` (or `0022` if D-25a's ADR-0021 candidate is promoted — see
CONTEXT.md D-25a and RESEARCH.md's Open Question 2; this discrepancy between D-25 and
`PROMOTION.md`'s own inventory is flagged, not silently resolved, per the research).

### 4. `.planning/REQUIREMENTS.md` §"Milestone 4-6 as-shipped ledger" → pointer

**Analog:** the same file's own §"Milestone 2-3 as-shipped ledger" (lines 2817-2828), which Phase
5 already reduced to a pointer. This is a same-file, same-repo instance — no external analog
search needed. Read directly:

```markdown
## Milestone 2-3 as-shipped ledger

Per-requirement verdicts for Milestones 2 and 3 now live in
[`.planning/ledgers/milestone-02-03.md`](ledgers/milestone-02-03.md) rather than inline here
(D-21). REQUIREMENTS.md is already ~4,000 lines and holds five as-shipped ledger sections; five
sets of `file:line`-cited verdicts inline would make it unreadable. That ledger carries **118**
`REQ-*` rows and **2** nested outstanding-task items, each with a `file:line` citation and, where
the verdict is `satisfied`, a named passing test, example, or command that exercises it.
Phases 7, 10 and 13 add the remaining sibling ledger files (`milestone-04-06.md`,
`milestone-07-08.md`, `milestone-09-12.md`) in the same directory.
```

**Copy this shape exactly for the Milestone 4-6 section** (currently at lines 2830-3069, ending at
the `## Milestone 7-8` heading), substituting: ledger filename `milestone-04-06.md`; decision
citation `D-26` (this phase's own pointer-reduction decision, not `D-21`); row count `115`
`REQ-*` rows and whatever nested-item count the finished ledger carries (count it, don't assume);
remaining sibling list drops to just `milestone-07-08.md`, `milestone-09-12.md` (Phases 10, 13 —
Phase 7 itself is no longer "remaining"). All content currently between lines 2830 and the
`## Milestone 7-8` heading (~239 lines of inline per-requirement detail) is deleted and replaced by
this ~10-line pointer paragraph — this is a large deletion in this one location, which is expected
and correct (unlike `.project/` edits, REQUIREMENTS.md's own ledger sections are *designed* to be
superseded and shrunk, per D-00d/D-26; do not apply the "no-deletion" D-00g rule here — that rule
is specific to `.project/` corpus files, not to this file's own ledger-pointer mechanism).

### 5. `.planning/codebase/STRUCTURE.md` "Directory Purposes" section

**Analog:** the section's own existing six crate entries — self-analog, extend the established
per-crate entry shape.

**Exact entry shape to copy** (verbatim structure, from the existing `paladin-storage` entry,
`STRUCTURE.md:279-282`):
```markdown
**`crates/paladin-storage/`:**
- Purpose: Repository and persistence adapters
- Contains: SQLite, MySQL, Redis queue, MinIO/S3, scheduler
- Key files: `src/{sqlite,mysql,redis,minio,scheduler}/*.rs`
```

**Insertion point, verified:** immediately after the `paladin-storage` entry (line 282) and
immediately before the `**`src/`:**` entry (line 285) — five new entries go in that exact gap, one
each for `paladin-herald`, `paladin-notifications`, `paladin-content`, `paladin-web`, and
`doc-examples`. Each entry needs Purpose / Contains / Key files, matching the three-bullet shape
above — pull the actual purpose/contents/key-files facts from each crate's own `Cargo.toml` and
`src/lib.rs`, do not invent generic text.

**Hard rule, easy to get wrong (from Common Pitfall/Anti-Pattern in RESEARCH.md):** the gap is
specifically in this *prose* section (lines 250-283 originally), not the ASCII directory tree
higher in the file (lines ~8-72), which **already lists all ten crates correctly**. Verify the fix
against the prose section, or the correction is a no-op that leaves the actual defect unfixed.

### 6. Eleven-plus `.project/` documents — dated correction banner + inline annotation

**Analog:** `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` — the exact document
Phase 5 annotated per D-00g/ADR-0010, read in full at lines 1-8, 28-31, 83, 360 for this mapping.

**Three markup shapes, transcribed verbatim — copy exactly, do not re-derive prose style:**

**(a) Top-of-file banner** (`RELEASE_NOTES_MILESTONE_3.md:1-8`):
```markdown
# Release Notes: Milestone 3

> **Correction (dated 2026-08-04, ADR-0010):** This document's Epic 19-24 numbering does not
> match the authoritative plan/epic-definition set, and two further claims are verified absent
> from the tree. See
> [`.planning/decisions/0010-milestone-3-epic-numbering.md`](../../.planning/decisions/0010-milestone-3-epic-numbering.md)
> for the full mapping. Original text is retained below with inline corrections — nothing is
> deleted.
```
For this phase, substitute the date (this phase's authoring date), the ADR number (0014 for the
numbering-collision documents, or the relevant ARCH-03/04/05 ADR number for the others), and a
one-sentence description of what's wrong specific to that document.

**(b) Inline per-section correction** (strikethrough heading + bold note,
`RELEASE_NOTES_MILESTONE_3.md:28-31`):
```markdown
### ~~Epic 19: Conclave Pattern (Multi-Expert Synthesis)~~ Epic 19: Herald & Domain Type Consolidation
**Corrected numbering (ADR-0010):** this section's content (Conclave) is Milestone 2 **Epic 15**,
not Epic 19. See ADR-0010's mapping table for the full correction.
**Status**: ✅ Complete
```
Original text (`**Status**: ✅ Complete`) survives unchanged beneath the correction. For FR-level
corrections (e.g. FR-7/FR-10 in `prd-paladin-ports-extraction.md`, FR-31/FR-32 in
`prd-paladin-llm-extraction.md`), apply the same shape to the FR bullet itself:
```markdown
- **FR-7:** ~~<original text>~~
  **Corrected (ADR-0016, dated <date>):** <what actually applies, citing the ADR>. Original text
  retained above, superseded.
```

**(c) Standalone "Superseded" banner for a whole forward-looking section**
(`RELEASE_NOTES_MILESTONE_3.md:360`):
```markdown
> **Superseded (dated 2026-08-04, ADR-0010):** The section below is a point-in-time forward-look,
```
Use this shape for FR-31/FR-32's circular-dependency concern (ADR-0017 — "the concern was real but
mis-sited", not simply wrong) and for the M6 overview's Epic 2 AC 6 / Epic 4 AC 5 / risk-register
re-export language (ADR-0018, D-13).

**(d) One-line pointer banner only, no inline strikethrough** — reserved for the byte-equivalent
INGEST-CONFLICTS extracts (7 or 8 files, count discrepancy flagged in RESEARCH.md Open Question 1
— enumerate directly rather than trusting either count):
```markdown
> **See ADR-0014** (dated <date>) for the corrected Milestone/Tier numbering this document's
> Milestone-numbering references predate. Not corrected inline here — this document is a
> byte-equivalent copy carrying no independent content beyond the source already corrected
> elsewhere.
```

**Exact correction targets and which markup applies** (from RESEARCH.md's ".project/ correction
sites" table, cross-checked against CONTEXT.md D-08/D-11/D-12/D-13/D-17/D-18 — 14/14 paths
existence-verified):

| Path | Target | Markup |
|---|---|---|
| `.project/Milestone_4-.../Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md` | title | (a)+(b) |
| `.project/Milestone_5-.../overview/Milestone_5-Tier_2-Workspace-Decomposition.md` | title + Appendix D | (a)+(b) |
| `.project/Milestone_6-.../overview/Milestone_6-Tier_3-Architectural-Refinements.md` | prereq line + Epic 2 AC 6 + Epic 4 AC 5 + risk register | (a)+(b)/(c) |
| `.project/Milestone_5-.../Epic_4/prd-paladin-llm-extraction.md` | Non-Goal 2 (`:32`?) + FR-31 (`:197`) + FR-32 (`:199`) | (a)+(b) for Non-Goal 2, (c) for FR-31/32 |
| `.project/Milestone_5-.../Epic_2/prd-paladin-ports-extraction.md` | §1 + FR-7/FR-10 | (a)+(b) for §1, (b) for FR-7/FR-10 (extending FR-11's carve-out) |
| `.project/Milestone_4-.../Epic_1/prd-expand-feature-flags.md` | FR1 + Design Considerations (vision, MCP, web-server) | (a)+(b), three separate corrections in one document |
| `.project/Milestone_6-.../Epic_2/prd-relocate-orchestration-services.md` | Non-Goal 7, Open Question 4, `use_cases/` target | (a)+(b) |
| `.project/Milestone_6-.../Epic_4/prd-relocate-circuitbreaker-infra.md` | Goal 7, FR-4.11, FR-4.12 (re-pointed to new mdbook path) | (a)+(b) |
| 7-8 byte-equivalent INGEST-CONFLICTS extracts | top only | (d) |

**Files cited but NOT edited** (evidence sources only): `dependency-matrix.md` (the audit that was
*right*), `battalion-result-upward-dependency-decision.md` (promoted by ADR-0016, not edited
directly — ADR-0016 restates it), `build-benchmarks.md` (transcribed verbatim by ADR-0020, not
corrected — its own internal −6.6%/−5% inconsistency is *judged* by the ADR, not fixed at source),
`prd-workspace-finalization-epic-6.md` (cited for FR-3.5/SM-7, likely no edit), `prd-cli-isolation.md`
(cited for D-20's finding; any correction belongs to Phase 8, confirm scope boundary in the plan).

## Shared Patterns

### Precedence-order legibility (D-00b)
**Source:** CONTEXT.md D-00b, applied structurally.
**Apply to:** every artefact this phase writes. ADRs sit at the top of the precedence order;
`STRUCTURE.md` third; `.project/` corrections at PRD/DOC tier (fifth/sixth) — annotation only,
never elevation. Each artefact's edit must be legible against this ordering: an ADR states the
final answer; `.project/` only records what was claimed and what corrects it, pointing upward at
the ADR or ledger row, never restating itself as authoritative.

### Amend-in-place, retain superseded text, date every amendment (D-00f, D-00g)
**Source:** `.planning/ledgers/milestone-02-03.md:99-103` (ledger amendment example);
`RELEASE_NOTES_MILESTONE_3.md:1-8,28-31,360` (`.project/` correction example).
**Apply to:** the ledger (later plans amend rows in place), every `.project/` correction
(never delete, always strikethrough-and-append with a dated note), and `STRUCTURE.md` (extend,
don't rewrite the six existing entries).

### ADR heading-set and bullet-list discipline (D-00h, `adr-parser.cjs` compatibility)
**Source:** `PROMOTION.md`'s "Required heading set" section.
**Apply to:** all 7-8 new ADRs. `## Code Locations` and `## Considered Options` must be bulleted
lists; deviating breaks `adr-parser.cjs`'s `splitEntries` silently (no error, just an empty/opaque
parse result) — this is the single easiest-to-miss convention in the whole phase.

### Fresh citation verification, never trust a transcribed line number
**Source:** RESEARCH.md Common Pitfall 1 — `intel/code-verification.md`'s `ci.yml:228` citation
for `crate-isolation` is stale; the job is now at `ci.yml:304` (confirmed this session).
`feature-flags.yml:115,118,141`, by contrast, checked out exactly.
**Apply to:** every exercising-artefact citation the ledger writes. Re-`grep -n` at authoring
time; do not copy a line number forward from CONTEXT.md, RESEARCH.md, or `intel/`.

## No Analog Found

None. Every artefact class this phase produces has at least one complete, working prior instance
in this same repository, read directly during this mapping pass. This is itself a phase
characteristic worth flagging to the planner: there is no "invent a new shape" risk anywhere in
Phase 7 — the entire task is locating the exact prior instance and copying its structure precisely
enough for an executor to follow without re-deriving conventions.

## Metadata

**Analog search scope:** `.planning/ledgers/`, `.planning/decisions/`, `.planning/codebase/`,
`.planning/REQUIREMENTS.md`, `.project/Milestone_3-Completion/`, `.project/Milestone_4/5/6-.../`
(14 named correction-target paths, all existence-verified).
**Files scanned:** `milestone-01.md` (partial), `milestone-02-03.md` (full), ADRs 0008/0009/0010/
0011/PROMOTION.md (full), `RELEASE_NOTES_MILESTONE_3.md` (targeted sections), `STRUCTURE.md`
(targeted sections 1-100, 245-339), `REQUIREMENTS.md` (targeted sections, both ledger pointers and
both ledger bodies), `prd-paladin-llm-extraction.md` (targeted FR/Non-Goal sections).
**Pattern extraction date:** 2026-08-06
