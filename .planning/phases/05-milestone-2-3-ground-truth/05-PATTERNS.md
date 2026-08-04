# Phase 5: Milestone 2-3 Ground Truth - Pattern Map

**Mapped:** 2026-08-04
**Files analyzed:** 7 (all markdown planning artefacts; no `.rs` files in scope)
**Analogs found:** 7 / 7

This phase writes documentation, not code. "Role/data-flow" classification below is repurposed for
document type; every analog is a shipped markdown file in this same repo, not a code file.

## File Classification

| New/Modified File | Doc Type | Analog | Match Quality |
|---|---|---|---|
| `.planning/ledgers/milestone-02-03.md` (new, 118 rows) | cited status ledger | `.planning/ledgers/milestone-01.md` | exact — same document class, same author, sibling file per D-00d |
| `.planning/decisions/0010-*.md` (new ADR, epic numbering) | ADR, `conforms` | `.planning/decisions/0001-battalion-config.md` (shape); `0004-temperature-validation.md` (as a `Considered Options`-heavy exemplar) | exact — same repo ADR format |
| `.planning/decisions/0011-*.md` (new ADR, vision surfaces + encryption) | ADR, `must change` | `0004-temperature-validation.md` | exact — closest existing ADR whose `Code Conformance` is `must change` and names a downstream executing requirement (GAP-07 in 0004 ≈ CLOSE-03 here) |
| `.planning/decisions/0012-*.md` (new ADR, live-API key semantics) | ADR, `must change` (doc-only) | `0004-temperature-validation.md` | role-match |
| `.planning/decisions/0006-coverage-gate.md` (amended in place) | ADR amendment | `.planning/ledgers/milestone-01.md`'s "Phase 2/3/4 amendments" sections (amendment *style*); ADR-0006's own current text (amendment *target*) | exact |
| `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` (corrected in place) | annotate-don't-rewrite correction | `.planning/ROADMAP.md`'s own inline `**Amended by Phase 4, dated 2026-08-03, citing …**` banners (Overview section, line ~8, and success-criterion 5, line ~329) | exact |
| `.planning/REQUIREMENTS.md` §"Milestone 2-3 as-shipped ledger" (reduced to pointer) | pointer-reduction | `.planning/REQUIREMENTS.md`'s own §"Milestone 1 as-shipped ledger" (lines 2628-2638) — Phase 1's D-17 already did this exact reduction | exact — same file, same author, same convention (D-00d) |

## Pattern Assignments

### `.planning/ledgers/milestone-02-03.md` (new ledger)

**Analog:** `.planning/ledgers/milestone-01.md` (697 lines, read in full)

**Head-notes pattern** (`milestone-01.md:1-19`) — copy this shape verbatim, substituting counts:
```markdown
# Milestone 1 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section (D-17).
That section becomes a pointer to this file. Phases 5, 7, 10 and 13 each add a sibling ledger
(`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`, `milestone-09-12.md`) rather than
growing REQUIREMENTS.md further — REQUIREMENTS.md is already ~4,000 lines and five inline sets of
`file:line`-cited verdicts would make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — ... nesting them keeps this ledger
joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing new IDs (D-18).

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-19). ...
```
For Phase 5, add the two D-04 path caveats as a **third** head-note block (CONTEXT.md names this
explicitly as new for this phase) — model it on the existing path-caveat paragraph already sitting
in REQUIREMENTS.md's own Milestone 2-3 section header (`REQUIREMENTS.md:2646-2650`):
```markdown
**Read this section with the path caveat.** Every run-2 PRD assumes a single-crate
`src/core|application|infrastructure` layout. The workspace was decomposed in Milestone 5
(ingested in run 3) into what is now ten library crates plus the root facade, so the `src/...`
paths in those PRDs are historical. Citations below are the **current** locations, verified by
direct inspection of `release/v0.7.0`.
```

**Status-key / verdict-vocabulary pattern** — two working vocabularies exist and must be
reconciled to Phase 1's D-00f five-value set (`satisfied` · `present, unproven` ·
`genuinely outstanding` · `deferred with reason` · `superseded by shipped code`), copied verbatim
from `milestone-01.md:152-160`:
```markdown
## Verdict legend

| Verdict | Meaning |
|---|---|
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |
```
Note REQUIREMENTS.md's own Milestone 2-3 section header (`:2651-2657`) uses a **different**,
five-value ingest-era vocabulary (`Shipped` / `Shipped (relocated)` / `Verify` / `Variant` /
`Unverified candidate` / `Open defect → X`) — this is the vocabulary D-01 explicitly says must be
upgraded, not copied. Use `milestone-01.md`'s legend, not REQUIREMENTS.md's.

**Per-epic row-table pattern** (`milestone-01.md:341-356`, Epic 1 section shown in full):
```markdown
### Epic 1 — Paladin Domain Foundation

No open task items (182/182 complete per `intel/task-completion-state.md`) — every row below carries
no nested block.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-entity | satisfied | `PaladinData` struct at `crates/paladin-core/src/platform/container/paladin.rs:142`, `Paladin` type alias at `paladin.rs:229`; exercised by `test_paladin_data_default` (`paladin.rs:350`) |
```
Column headers are exactly `| ID | Verdict | Evidence |`. Epics with open task items get a
one-line count sentence before the table (`milestone-01.md:359-360`):
```markdown
### Epic 2 — Garrison Memory System

4 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md`.
```

**Nested outstanding-item row pattern** (`milestone-01.md:366`, `:375-379`) — a blank-ID row with
the nested item's checkbox text, source line and its own verdict:
```markdown
| | | **Nested outstanding item:** `- [ ] 9.14 Write test: \`test_large_conversation_performance\` - benchmark with 1000 entries (future enhancement)` (`tasks-garrison-memory-system.md:222`) — **deferred with reason**. `REQUIREMENTS.md:2549` already records this exact test as "deferred → v2" ... |
```
Same three-column-blank-first-two shape is used for "New finding" rows discovered mid-review
(`:378-379`), prefixed `**New finding (plan ...):**` instead of `**Nested outstanding item:**` —
reuse this exact prefix convention for anything VERIFY-01/02 discovers that the ingest ledger
missed.

**In-place amendment banner pattern** (used for both the ledger's own future amendments and
directly informs the ADR-0006 amendment and the release-notes correction) — copy this exact inline
marker style, from `milestone-01.md:130-133`:
```markdown
**(Amended by Phase 4, dated 2026-08-03, citing `04-release-measurement.md` §"Exit re-measurement — full test suite at the final phase commit": the 2,924/0/122 figure below was measured at commit `d2898a3`, before plan 04-05's version bump, and this row originally presented it as if it still held. ... Found by `04-VERIFICATION.md` re-deriving rather than trusting the SUMMARY.)**
```
And the section-level amendment preamble (`milestone-01.md:21-29`, `:77-84`, `:111-120`) —
one such preamble opens each phase's amendment block:
```markdown
## Phase 2 amendments (2026-08-01)

This file is now **amended in place**, per Phase 2 CONTEXT.md D-02: when a Phase 2 plan's measured
result contradicted a row below, that row was edited directly with the new verdict, the command or
`file:line` that produced it, and this date — never split into a separate corrections file. Phases
5, 7, 10 and 13 inherit this same convention for their own sibling ledgers. Every amendment below is
sourced from a named Phase 2 plan's SUMMARY (...) or from `02-test-baseline.md` ...
```

**VERIFY-02 block-verdict table pattern (D-05 parent-task cluster table)** — no exact existing
analog for a *parent-task cluster* table exists in `milestone-01.md` (Milestone 1 had no
comparably-sized open-checkbox blocks), but the closest shape to adapt is the "Phase 4 deferrals
consolidated with named owners" table (`milestone-01.md:140-150`), which already has the
`ID | Verdict | Evidence`-with-named-owner shape D-06's "named clusters" language wants:
```markdown
| ID | Verdict | Evidence |
|---|---|---|
| REL-03 — four newly-surfaced advisories (...) | deferred with reason | Surfaced live by `cargo audit` ... **Owner: Phase 9 / SEC-01 and Phase 12 / SUPPLY-02.** |
```
For the three VERIFY-02 blocks, use the same `| Parent task | Verdict | Evidence |` shape, one row
per parent-task cluster, rolling up into a single block verdict sentence above the table (per D-06:
"a block is `satisfied by shipped code` only if every parent-task cluster verifies").

**Divergence table pattern** (`milestone-01.md:162-169`) — for VERIFY-01 rows where shipped code
contradicts the ingested PRD:
```markdown
## Divergences — shipped code superseded an ingested requirement

> **This divergence is a documented non-goal that shipped anyway.** ...

| Requirement | Ingested position | Shipped position | Verdict |
```

---

### `.planning/decisions/0010-*.md` … `0012-*.md` (new ADRs)

**Analog:** `.planning/decisions/0001-battalion-config.md` (shape source), `0004-temperature-validation.md` (full worked example, 131 lines, read in full)

**Confirmed H2 heading sequence** (load-bearing per `adr-parser.cjs`'s synonym matching, per
RESEARCH.md Pattern 2) — no frontmatter, exactly these seven H2s in this order:
```
# ADR-00NN: <title>

## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```

**Status block** (`0004-temperature-validation.md:1-7`):
```markdown
# ADR-0004: Temperature validation

## Status

Accepted

**Date:** 2026-07-31
```

**Considered Options — must be a bulleted list** (parser requirement, RESEARCH.md Pattern 2):
```markdown
## Considered Options

- `REQ-temperature-range-v1` (Epic 1 FR-2.3 / US-2, builder MUST validate `[0.0, 1.0]` globally)
  — rejected as the sole answer; it is what the builder validates today, but applied literally it
  makes `REQ-temperature-range-v2`'s DeepSeek `0.0-2.0` range permanently unreachable ...
```

**Code Conformance field — the exact two-value contract** (`0004-temperature-validation.md:108-115`):
```markdown
## Code Conformance

must change

`ProviderCapabilities` at `llm_port.rs:754` has no `temperature_range` field today. **GAP-07** in
Phase 2 is the requirement that lands the ports-layer change ... Nothing in this phase edits Rust
source — this ADR records the decision only.
```
For ADR-0010 (epic numbering): `conforms` — model on this same shape but state that the *code* is
fine and the defect is documentary, per CONTEXT.md D-19.

**Downstream Consumers — names phases/requirements, not files** (`0004-temperature-validation.md:117-131`):
```markdown
## Downstream Consumers

- Phase 2 GAP-07 — implements the `temperature_range` field ...
- The three shipped LLM adapters (OpenAI, Anthropic, DeepSeek) — each must populate ...
- **Sequencing note:** Phase 14's **WEB-03** ... modifies the same `ProviderCapabilities` struct ...
```
For ADR-0011/0012, this section names **Phase 6 CLOSE-03** as the consumer (per D-19).

**PROMOTION.md numbering-index update:** RESEARCH.md's structure confirms
`.planning/decisions/PROMOTION.md`'s "Next free ADR number" line must move to 0013 once
0010-0012 land — no analog needed, it's a one-line increment in that same file.

---

### `.planning/decisions/0006-coverage-gate.md` (amended in place)

**Analog:** its own current content (full 232 lines read) — this is the file being edited, not a
separate template. The amendment technique to apply is `milestone-01.md`'s "Phase N amendments"
convention (see ledger section above), adapted to a single ADR file rather than a ledger:

- Add new prose under existing bullets (e.g. "The two module-scoped gates") rather than replacing
  them — ADR-0006 already has stub language at `:134-140` ("Their placement is handed to VERIFY-05
  in Phase 5 ... this measurement's per-file rows show `herald.rs` at 80.49% ...") that this
  amendment fills in with the module-scoped gate table (D-13) and the two inherited dispositions
  (D-14a/b).
- Use the exact inline marker style quoted above under "In-place amendment banner pattern."
- The `## Considered Options` section is the right place to add D-12's two new rejected positions
  (75% layered-tier table from the Milestone 3 plan; ≥80/≥70 re-asserted by Epic 24) — both are
  **already listed** as rejected at `:170-185` (this ADR already rejected them once, generically);
  VERIFY-05's amendment should cross-reference those existing bullets rather than duplicate them,
  per D-12's own "each rejected against the measured 84.79% with the reason stated."
- D-15's falsifiability statement against the ~78% Milestone-3 figure has a direct structural
  analog already in this same file at `:149-154` ("The ~24-point gap above the stale Milestone-1
  baselines is accepted and noted, not explained") — copy that paragraph's shape (state the number,
  state it's stale, do not reconcile it).

---

### `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` (corrected in place)

**Analog:** `.planning/ROADMAP.md`'s own self-amendment banners (`ROADMAP.md:8-13`, `:329`)

**Banner pattern, copied exactly in structure** (`ROADMAP.md:8-13`):
```markdown
(**Amended by Phase 4, dated 2026-08-03, citing
`04-release-measurement.md`**: the "22 runnable examples" figure traces to a Milestone 1 Epic 10
validation report ("22/22 examples compiling") and has since gone stale — the shipped tree carries
**47** `.rs` files under `examples/`, of which four are declared `[[example]]` targets gating on
non-default features (`vision`, `content-processing`, `web-server`); no crate under `crates/` ships
its own `examples/` directory. The shipped tree outranks an ingested count under this project's
precedence order.)
```
Key structural features to reproduce for D-08's three corrections in `RELEASE_NOTES_MILESTONE_3.md`:
1. **Dated, attributed opening clause**: `**Amended by Phase N, dated YYYY-MM-DD, citing
   <source-doc>**:` — bold, inline, immediately followed by the correction prose in the same
   parenthetical/sentence.
2. **States the old figure/claim, then the new one, then the reason the old one is wrong** — never
   just states the new fact.
3. **Original text stays in place** — the amendment wraps around or follows the original clause
   in the same paragraph; nothing is deleted (matches D-00g/D-08's "retain, don't delete").
4. For this phase's specific top-of-file banner (unlike ROADMAP's inline mid-sentence markers),
   use `milestone-01.md`'s section-level preamble shape instead (a standalone paragraph before the
   corrected content begins) — see "In-place amendment banner pattern" above, `milestone-01.md:21-29`,
   as the closer structural fit for a document-level (not sentence-level) banner:
   ```markdown
   > **Correction (dated 2026-08-04, ADR-0010):** This document's Epic 19-24 numbering does not
   > match the authoritative plan/epic-definition set. See ADR-0010 for the corrected mapping.
   > Original text retained below with inline corrections; nothing is deleted.
   ```
5. **Per-claim inline correction** — strike the wrong heading/number, keep it visible, follow with
   the correction:
   ```markdown
   ### ~~Epic 19: Conclave Pattern (Multi-Expert Synthesis)~~ Epic 19: Herald & Domain Type Consolidation
   **Corrected numbering (ADR-0010):** this section's content (Conclave) is **Epic 15**, not
   Epic 19. [... original text retained below ...]
   ```

The three correction targets and their exact source lines (confirmed by grep during pattern
mapping, matching RESEARCH.md's own citations):
- Epic heading mismatches: `RELEASE_NOTES_MILESTONE_3.md` lines 21, 48, 76, 111, 147 (`### Epic 19`
  … `### Epic 23`).
- `PerformanceBased` claim: line 106.
- "🔮 What's Next" stale forward-look section: line 320.

---

### `.planning/REQUIREMENTS.md` §"Milestone 2-3 as-shipped ledger" (reduced to pointer)

**Analog:** the same file's own §"Milestone 1 as-shipped ledger" section, already reduced by
Phase 1's D-17 — this is a same-document, same-convention precedent, not an external analog.

**Exact text to model on** (`REQUIREMENTS.md:2628-2638`, quoted in full):
```markdown
## Milestone 1 as-shipped ledger

Per-requirement verdicts for Milestone 1 now live in
[`.planning/ledgers/milestone-01.md`](ledgers/milestone-01.md) rather than inline here (D-17).
REQUIREMENTS.md is already ~4,000 lines and holds five as-shipped ledger sections; five sets of
`file:line`-cited verdicts inline would make it unreadable. That ledger carries 113 `REQ-*` rows
and 39 nested outstanding task items, each with a `file:line` citation and, where the verdict is
`satisfied`, a named passing test, example, or command that exercises it. Phases 5, 7, 10 and 13
add sibling ledger files (`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`,
`milestone-09-12.md`) in the same directory.
```
For the Milestone 2-3 section (currently at `REQUIREMENTS.md:2641` onward, 118 rows starting at
`## Milestone 2-3 as-shipped ledger` through whatever line the section ends at), replace the full
body with the same shape substituting: `milestone-02-03.md`, "118 `REQ-*` rows" (+ whatever nested
outstanding-item count the scaffold plan settles on), and D-21 as the citation instead of D-17.
**Note the row-count claims in the copied paragraph are load-bearing** (113 rows / 39 nested items
in the Milestone-1 pointer) — the Milestone 2-3 pointer must state its own actual final counts, not
copy 113/39 verbatim.

## Shared Patterns

### ADR heading sequence (governs all three new ADRs)
**Source:** `.claude/gsd-core/bin/lib/adr-parser.cjs` (confirmed by RESEARCH.md Pattern 2, direct
read) + `0001-battalion-config.md` / `0004-temperature-validation.md` / `0006-coverage-gate.md`
(all three share the identical 7-H2 sequence).
**Apply to:** `0010-*.md`, `0011-*.md`, `0012-*.md`.
No frontmatter. `## Considered Options` and `## Code Locations` must be bulleted lists (parser's
`splitEntries` requirement) — a prose paragraph collapses into one opaque blob.

### Amend-in-place / retain-don't-delete
**Source:** `milestone-01.md`'s three "Phase N amendments" sections; `ROADMAP.md`'s own inline
`**Amended by Phase N, dated ..., citing ...**` banners.
**Apply to:** the ledger's own future amendments (not this phase, but the convention this phase
must itself follow when it amends ADR-0006), ADR-0006 (this phase amends it), and
`RELEASE_NOTES_MILESTONE_3.md` (this phase corrects it). Every amendment: (1) is dated, (2) names
its source evidence document, (3) keeps the original text visible, (4) never becomes a separate
corrections file.

### `REQ-*` primary key, evidence bar, verdict vocabulary
**Source:** `milestone-01.md:1-19`, `:152-160`.
**Apply to:** every row of `milestone-02-03.md`. Do not import REQUIREMENTS.md's own ingest-era
verdict words (`Shipped`, `Verify`, `Variant`, etc.) into the new ledger — those get *upgraded*
into the five-value D-00f vocabulary, not carried over as-is.

## No Analog Found

None. All seven artefact classes in this phase's scope have a direct, shipped, in-repo analog —
this corpus already contains a complete worked example of every document type this phase produces
(ledger, ADR, ADR amendment, historical-document correction, pointer-reduction).

## Metadata

**Analog search scope:** `.planning/ledgers/`, `.planning/decisions/`, `.planning/ROADMAP.md`,
`.planning/REQUIREMENTS.md`, `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`.
**Files scanned:** `milestone-01.md` (697 lines, full read), `0001-battalion-config.md` (94 lines),
`0004-temperature-validation.md` (131 lines, full read), `0006-coverage-gate.md` (232 lines, full
read), `ROADMAP.md` (targeted grep + read around amendment banners), `REQUIREMENTS.md` (targeted
reads around lines 2620-2660), `RELEASE_NOTES_MILESTONE_3.md` (grep for heading/claim line numbers,
not fully read — not this pass's job, its correction sites are already fully cited in
CONTEXT.md/RESEARCH.md).
**Pattern extraction date:** 2026-08-04
