# Phase 1: Ground Truth & Decision Records - Pattern Map

**Mapped:** 2026-07-30
**Files analyzed:** 10 (6 ADRs + 1 promotion index + 1 ledger + 2 edited docs)
**Analogs found:** 10 / 10

This phase produces **no product code** — every new/modified file is a Markdown document in
`.planning/`. "Patterns" here means document shape, section headings, and citation style, not
code idioms. Roles are mapped onto the closest doc-authoring equivalent (`config`≈structured
data record, `model`≈schema-defining doc).

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.planning/decisions/0001-battalion-config.md` | config (decision record) | request-response (write-once, cited) | `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` | role-match (house style precedent) |
| `.planning/decisions/0002-battalion-result.md` | config (decision record) | request-response | same analog + `adr-parser.cjs` heading synonyms | role-match |
| `.planning/decisions/0003-formation-min-paladins.md` | config (decision record) | request-response | same analog + `adr-parser.cjs` | role-match |
| `.planning/decisions/0004-temperature-validation.md` | config (decision record) | request-response | same analog + `adr-parser.cjs` | role-match |
| `.planning/decisions/0005-herald-trait.md` | config (decision record) | request-response | same analog + `adr-parser.cjs` (RESEARCH.md's own worked example) | exact (fully worked example already in RESEARCH.md) |
| `.planning/decisions/0006-coverage-gate.md` | config (decision record) | request-response | same analog + `adr-parser.cjs` | role-match |
| `.planning/decisions/PROMOTION.md` | utility (index/registry doc) | CRUD (append-only registry) | `.planning/PROJECT.md` §"Eleven ADR candidates now exist" (numbered candidate list) | role-match |
| `.planning/ledgers/milestone-01.md` | model (structured status record) | batch (bulk citation transcription) | `.planning/REQUIREMENTS.md` §"Milestone 1 as-shipped ledger" (line 2361) | exact |
| `.planning/PROJECT.md` (edit: precedence order + Key Decisions table) | config | CRUD (in-place edit) | itself — existing `## Key Decisions` and precedence-order sections | exact (editing in place) |
| `.planning/REQUIREMENTS.md` (edit: reduce ledger section to pointer) | config | CRUD (in-place edit) | itself — existing "Milestone 2-3/4-6/7-8/9-12 as-shipped ledger" sibling rows already act as pointers | exact |

## Pattern Assignments

### `.planning/decisions/000N-*.md` (the six ADRs)

**Analog 1 (house style / precedent):**
`.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`

This is the corpus's *only* prior decision/options-pair document (`Status: Approved`,
`Decision Date`, `Chosen Option`, Rationale, Rejected Options, an implementation checklist).
D-04 explicitly rejects imitating its two-file `decisions/` + `-options.md` split ("twelve files
for six decisions") but its single-file shape — status line, chosen option, rationale, rejected
options, evidence — is exactly what D-04's leaner one-file-per-decision format is a trim of.

**Header block pattern to copy (lines 1-7):**
```markdown
# Decision: Resolving `src/core/` → `application::` Upward Dependencies

**Epic:** Milestone 5 Epic 1 — Workspace Initialization and `paladin-core` Extraction
**Decision Date:** 2026-05-13
**Chosen Option:** **Option A — Move Pure Value Types to `paladin-core`**
**Status:** Approved — implementation sub-tasks appended to task list as 3.6a–3.6k
```
Adapt to Phase 1's own fields (`Question`, `Chosen variant`, `Code conformance`) but keep the
dense metadata-line-then-body shape.

**Rejected-options pattern to copy:**
```markdown
### Rejected Options

- **Option B (New parallel types):** Introduces duplicate type hierarchies and large refactor
  scope, violating the "structural refactor only" constraint of this Epic.
- **Option C (Defer to Epic 2):** Explicitly fails FR-16. ...
```
Map each rejected `REQ-*` variant ID onto this bullet shape (one bullet per rejected variant,
name + one-sentence reason it lost), per D-04's "Rejected variants (the REQ-* IDs it resolves)".

**Analog 2 (machine-readable heading shape):** `.claude/gsd-core/bin/lib/adr-parser.cjs`
`CANONICAL_HEADERS` table (read in full during research; confirmed again here at
`/workspace/.claude/gsd-core/bin/lib/adr-parser.cjs:19-60`). Use these exact H2 headings so the
new ADRs are parseable by GSD's own tooling:

| D-04 field | Heading to use | Parser bucket |
|---|---|---|
| Status | `## Status` (bare word: `Accepted`) | `status` |
| Date | prose line under `## Status` | *(unmapped, harmless)* |
| Question | `## Context` | `goal` |
| Chosen variant | `## Decision` (bulleted) | `decisions` |
| Evidence (file:line) | `## Code Locations` (bulleted, not prose) | `key_files` |
| Rejected variants | `## Considered Options` (one bullet per rejected REQ-*) | `considered_options` |
| Code conformance | `## Code Conformance` | *(unmapped, still required by D-03)* |
| Downstream consumers | `## Downstream Consumers` | *(unmapped; rename to `## Dependencies` if machine-readability matters later)* |

**Full worked example already produced in RESEARCH.md** (copy this shape verbatim for
`0005-herald-trait.md`, then adapt for the other five):
```markdown
# ADR-0005: Herald trait signature

## Status
Accepted

**Date:** 2026-07-30

## Context
Two documented Herald trait shapes exist in the ingested corpus (Epic 8 FR-1, infallible; Epic 8
§6.2, fallible). Which one is authoritative for the framework's output-formatting contract?

## Decision
- The shipped trait at `crates/paladin-core/src/platform/container/herald.rs:49` is authoritative.
- It ships the fallible (v2) form: ...

## Considered Options
- REQ-herald-trait-v1 (Epic 8 FR-1) — infallible `-> String` returns throughout — rejected, not
  what shipped
- REQ-herald-type-consolidation (run 2) — placeholder-type consolidation — rejected, no
  placeholder or TODO exists in `herald.rs`

## Code Locations
- `crates/paladin-core/src/platform/container/herald.rs:49-153` — the full trait definition

## Code Conformance
conforms

## Downstream Consumers
- Phase 2 GAP-07 (no action required — nothing to change)
- Any future Herald implementor (`crates/paladin-herald`, custom output formats)
```

**Critical anti-pattern (from RESEARCH.md, applies to every ADR):** write `## Code Locations` and
`## Considered Options` as **bulleted lists**, never a prose paragraph — `adr-parser.cjs`'s
`splitEntries` only produces structured multi-entry output from bullet/numbered lines.

---

### `.planning/decisions/PROMOTION.md`

**Analog:** `.planning/PROJECT.md` §"Eleven ADR candidates now exist" (numbered list, currently
at `/workspace/.planning/PROJECT.md` around line 1050+), which already inventories all eleven
candidates with their owning phase, file path, and why each was not promoted.

**Pattern to copy:** the numbered-candidate-with-owner-and-rationale shape:
```markdown
1. **`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`** (run 3) —
   ... **This is the strongest candidate in the corpus and the one with real consequences if left
   unprotected.**
2. **`Epic_17.5/epic17-5.md`** (run 2) — ...
3. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) — a formal **risk acceptance**...
```
`PROMOTION.md` should restate these eleven with an explicit "Owner phase" column (7, 9, 10, 12,
13, 14 per D-05) plus the procedure text, and — per RESEARCH.md's "Don't Hand-Roll" table — fold
in the **next-free-ADR-number pointer** here rather than inventing a second index file, since
Phase 5/7/10/13 need to know the next number without listing the directory.

---

### `.planning/ledgers/milestone-01.md`

**Analog:** `.planning/REQUIREMENTS.md` §"Milestone 1 as-shipped ledger" (starts at line 2361,
confirmed present at `/workspace/.planning/REQUIREMENTS.md:2361`).

**Header/status-key pattern to copy (lines 2361-2370):**
```markdown
## Milestone 1 as-shipped ledger

All 115 requirement IDs extracted by ingest run 1, with verified status. **Not forward scope** —
listed so nothing is lost and so runs 4-5 merge against stable keys. Acceptance criteria are not
repeated; they live in `.planning/intel/requirements.md`.

Status key: `Shipped` = satisfied by v0.7.0 code and a complete task list · `Verify` = code
exists, completion asserted only by the 2026-01 task list, confirmation is part of RECON-01 ·
`Partial → X` = residual work tracked by forward requirement X · `Variant` = see competing
variants · `Deferred → v2` · `Code diverges` = shipped implementation differs from the ingested
requirement.
```
D-20 replaces this status key with the new five-class verdict set (`satisfied` · `present,
unproven` · `genuinely outstanding` · `deferred with reason` · `superseded by shipped code`) —
copy the *shape* of a legend-then-table, not the old verdict names.

**Per-epic table pattern to copy (lines 2372-2398):**
```markdown
### Epic 1 — Paladin Domain Foundation (182/182 items, 100%)

| ID | Status |
|---|---|
| REQ-paladin-entity | Shipped — `crates/paladin-core/src/platform/container/paladin.rs`; `max_loops` superseded by `REQ-max-loops-auto` (enum at `paladin.rs:42`) |
| REQ-paladin-builder | Shipped — `src/application/services/paladin/paladin_builder.rs`; `[1,100]` validation superseded, see variant group 16 |
...
```
This is the D-18 "primary key is `REQ-*`" pattern already in production use — the new ledger
nests D-18's outstanding task items as sub-rows/sub-bullets under the same `REQ-*` keys instead
of a bare `Status` column, and adds the `file:line` + named-test citation D-19 requires (this
existing table already cites `file:line` for the "diverges"/"superseded" rows, e.g. `paladin.rs:42`
— extend that citation discipline to every row, not just the exceptional ones).

**`Code diverges` row pattern for the three D-21 divergences** (lines 2392, 2394 give the
existing house style for exactly this class):
```
| REQ-garrison-longterm-port | Code diverges — semantic retrieval ships as **Sanctum** (Qdrant + in-memory), not as a `sqlite-vss` extension of Garrison. Run 2 supplies the missing requirements: REQ-sanctum-port, REQ-embedding-port, REQ-sanctum-domain-model |
| REQ-garrison-sqlite | Code diverges — SQLite adapter shipped; `sqlite-vss` vector search superseded by Sanctum |
```
Use this exact "shipped-as-X, not Y; see forward pointer" phrasing for the MCP Streamable-HTTP,
Qdrant/Sanctum, and interactive-REPL rows under D-20's renamed `superseded by shipped code` class.

---

### `.planning/PROJECT.md` (edited in place — precedence order + Key Decisions table)

**Analog:** itself. Current state already read:

**Precedence order to replace** (`/workspace/.planning/PROJECT.md:731-732`):
```
**The precedence order this project uses**, most authoritative first:
**shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
task-list checkbox.**
```
D-02 prepends `ADR →` to this exact sentence. Every other place PROJECT.md states or paraphrases
this order (e.g. `/workspace/.planning/PROJECT.md:1057` "Precedence runs **shipped tree → codebase
map → `intel/code-verification.md` → PRD → DOC → checkbox**") must be updated identically —
`grep -n "shipped tree →" .planning/PROJECT.md` before finalizing to catch every restatement.

**Key Decisions table to replace** (`/workspace/.planning/PROJECT.md:1020-1025`):
```markdown
## Key Decisions

<!-- LOCKED DECISIONS (from ADR-typed documents). Empty by evidence, not by omission. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| *(none)* | **FINAL, ACROSS THE WHOLE CORPUS.** ... | — Pending |
```
D-06 replaces the `*(none)*` row with six real rows (one per Phase-1 ADR, linking to
`.planning/decisions/000N-*.md`) and replaces the "Empty by evidence, not by omission" framing
paragraph below the table with a pointer sentence to `.planning/decisions/`. Keep the table's
three-column shape (`Decision | Rationale | Outcome`) — it is already the correct schema for a
locked-decision row, just populate it.

---

### `.planning/REQUIREMENTS.md` (edited in place — ledger section reduced to pointer)

**Analog:** itself — the four sibling ledger rows already in the summary table at
`/workspace/.planning/REQUIREMENTS.md:31-36`:
```markdown
| **Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope. |
| **Milestone 2-3 as-shipped ledger** | All 118 run-2 requirement IDs, with status. Not forward scope. |
```
D-17 turns the *body* section at line 2361 ("## Milestone 1 as-shipped ledger" + all its per-epic
tables) into a short pointer paragraph directing to the new `.planning/ledgers/milestone-01.md`,
matching how compact the summary-table row already is. Do not delete the summary-table row at
line 31 — only the expanded body section needs pointer treatment.

## Shared Patterns

### File-citation discipline (applies to every new file in this phase)
**Source:** RESEARCH.md §Code Examples + §Don't Hand-Roll (re-verified against
`/workspace/crates/...` paths during this mapping pass is unnecessary — RESEARCH.md already did
it on 2026-07-30 for all six ADR subjects).
**Apply to:** every ADR's `## Code Locations` section and every ledger row.
```bash
grep -n "at least 2\|requires at least" \
  crates/paladin-core/src/platform/container/battalion/formation.rs
# → 111:                "Formation requires at least 2 Paladins, got {}",
```
Re-run the specific grep/read immediately before finalizing each citation, not once at research
time — line numbers drift.

### ADR heading synonym table
**Source:** `.claude/gsd-core/bin/lib/adr-parser.cjs:19-60` (`CANONICAL_HEADERS`)
**Apply to:** all six `.planning/decisions/000N-*.md` files.
Use `## Status`, `## Context`, `## Decision`, `## Considered Options`, `## Code Locations` as the
five headings that map onto the parser's synonym table; `## Code Conformance` and
`## Downstream Consumers` are deliberately unmapped custom headings per D-03/D-04.

### Ubiquitous language
**Source:** CLAUDE.md / `.github/copilot-instructions.md` naming table.
**Apply to:** all prose in ADRs and the ledger — CONTEXT.md's own `code_context` section repeats
this as a hard requirement ("Medieval military ubiquitous language is mandatory ... including in
the ADRs and the ledger"). Use Paladin/Battalion/Garrison/Arsenal/Citadel/Herald terms exactly as
the shipped code names them, not paraphrases.

## No Analog Found

None. All ten planned files have a strong existing analog (either a prior decision doc, an
existing ledger section, an in-repo tooling parser, or the same file being edited in place).

## Metadata

**Analog search scope:** `.planning/` (PROJECT.md, REQUIREMENTS.md, INGEST-CONFLICTS.md),
`.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/`, `.claude/gsd-core/bin/lib/`.
**Files scanned:** ~6 (PROJECT.md, REQUIREMENTS.md excerpts, the one prior decision doc,
adr-parser.cjs, plus the two CONTEXT/RESEARCH inputs already carrying pre-verified citations).
**Pattern extraction date:** 2026-07-30
