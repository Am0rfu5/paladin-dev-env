# Phase 11: Facade Residue & Deferred Register Disposition - Pattern Map

**Mapped:** 2026-08-08
**Files analyzed:** 8 (all `.planning/`/`.project/` records — zero `.rs` files, per D-13)
**Analogs found:** 8 / 8

**Note on this phase's domain:** Phase 11 writes zero executable code. "Files" below are ADRs,
register/ledger/triage documents, and dated correction banners on `.project/` sources. The
"closest analog" for each is an existing document of the same class, extracted the way source
code would normally be extracted — heading order, status vocabulary, evidence-cell shape,
numbering rules, banner shape.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.planning/decisions/0034-*.md` … `003N-*.md` (D1-D4 disposition set, FACADE-02) | ADR | request-response (verdict record) | `.planning/decisions/0031-extracted-crate-dependency-rule.md` | exact — same "restates, doesn't instruct a change" shape, same Phase-10-authored precedent this phase's D-00i/D-05/D-07/D-08 directly inherit |
| `.planning/decisions/003N-paladin-ml-placement.md` (FACADE-03b, contested) | ADR | request-response (verdict record) | `.planning/decisions/0018-m6-facade-reexport-policy.md` | role-match — a posture ADR about where code may/may not live, contested-position shape |
| `.planning/decisions/PROMOTION.md` (numbering index amendment) | config/index | CRUD (append-only ledger of ADR numbers) | itself — amended in place per its own dated-note convention | exact — self-referential; only existing analog is its own prior dated notes |
| `.planning/ledgers/milestone-07-08.md` (5 named rows amended: `REQ-m8-deferred-items-register`, `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`, `REQ-adapter-disposition-record`, `REQ-m8-epic3-no-extractions`) | ledger row | CRUD (cell replacement in place) | same file, rows already amended by Phase 10 plans (e.g. `REQ-extracted-crate-dependency-rule`, `REQ-content-processing-build-gate`) | exact — same file, same amendment convention, same evidence bar |
| new `.planning/` register/triage file (FACADE-01 rustdoc dispositions + FACADE-03 CLI/ML feature register + FACADE-04 triage table — one or two files, planner's discretion) | register | batch (triage table, per-occurrence disposition list) | `.project/.../infrastructure-adapter-disposition.md` (structure) + `milestone-07-08.md` (verdict-legend/evidence-bar convention) | role-match — no existing `.planning/`-native register of this exact shape; closest structural precedent is the `.project/` disposition record's table shape combined with the ledger's verdict vocabulary |
| `.planning/ROADMAP.md` §Phase 11 criterion 1 (amended in place) | governing doc | request-response (criterion text correction) | Phase 10's own `10-CONTEXT.md`/ledger corrections to REQUIREMENTS.md text (D-00c pattern) | role-match — same "amend a governing criterion, retain superseded text" shape |
| `.planning/REQUIREMENTS.md` (FACADE-01 text corrected at source, D-02) | governing doc | request-response (spec text correction) | `.planning/decisions/0031-...md:149` — same ADR corrects a `REQUIREMENTS.md:1438` citation error in place | exact — same document, same correction convention already modeled by ADR-0031 |
| `.project/Milestone_8-.../deferred-items.md` (Shape-A banner, D5 framing correction) | historical corpus doc | request-response (annotation, not rewrite) | Phase 8 commit `94814ff` (Shape A) — struck-and-corrected inline clause | exact — D-02/CONTEXT.md explicitly names Shape A as the fit for this correction |
| `.project/Milestone_8-.../Epic_3/infrastructure-adapter-disposition.md` (already has a Shape-B banner from Phase 10 plan 10-02 — extend only if the planner judges it adds value) | historical corpus doc | request-response (annotation, not rewrite) | itself — its own existing banner, Phase 10 plan 10-02, `2026-08-08` | exact — do not duplicate; extend in place only |

## Pattern Assignments

### `.planning/decisions/0034-*.md` … (D1-D4 disposition set) (ADR, request-response)

**Analog:** `.planning/decisions/0031-extracted-crate-dependency-rule.md` (full file read)

**Heading skeleton to copy verbatim** (`## Status` / `## Context` / `## Decision` /
`## Considered Options` / `## Code Locations` / `## Code Conformance` / `## Downstream Consumers`,
no frontmatter) — confirmed as the required set by `PROMOTION.md:95-105`:

```markdown
## Status

Accepted

**Date:** 2026-08-08
```

**Context pattern** (ADR-0031 lines 9-56): cite the measured fact first (`grep`/`ls`/`git show`
output, verbatim), then the document(s) whose claim it's tested against, with exact `file:line`.
Correct a stale citation inline rather than silently, e.g. ADR-0031 lines 50-56:

```
**Both `10-CONTEXT.md` D-15 and `.planning/REQUIREMENTS.md:1438` attribute that sentence to "the
same PRD's §4.4."** Re-read directly this session: `prd-extract-infrastructure-crates.md` has no
§4.4 containing this sentence ... `cost-benefit-assessment.md:118` is the correct one.
```

**Decision pattern — numbered sub-decisions in prose, kept visually separate** (ADR-0031 lines
58-91), directly reusable for D1/D3/D4's defer-with-trigger shape:

```
**(i) The enforceable invariant** ...
**(ii) The measured current state, accepted as the baseline** ...
**(iii) The anchor moved deliberately.** ...
```

For D1/D3/D4 specifically, the verdict sentence must satisfy Pattern 3 from RESEARCH.md (verb +
owner + trigger where deferred) — model the "quotable sentence" convention on ADR-0031 line 65-67:

```
**The restated rule, as a single quotable sentence:** *no extracted crate may depend on another
extracted crate or on the facade in its default build; a non-default optional feature may declare
such an edge only where the facade opts in explicitly and the dependent code is `cfg`-gated.*
```

**Considered Options pattern** (ADR-0031 lines 104-126) — bulleted, never prose, `(accepted)` /
`(rejected)` tag with one-clause "why":

```
- **Restate the rule as a default-build invariant ...** (accepted) — the invariant that has teeth
  is the default-build one; it is checkable by `cargo tree --no-default-features` ...
- **Keep the absolute "never" and remove the edge** (rejected) — not a record change but
  architecture work ...
```

**Code Locations pattern** — bulleted `file:line` citations for every claim, including the D-00e
evidence bar (exact command + output) as its own bullet (ADR-0031 lines 152-158):

```
Attempted this session: `cargo tree -p paladin-content --no-default-features`. It resolved
offline ... and produced a full dependency tree with **zero** occurrences of `paladin-llm` ...
```

**Code Conformance pattern** — one word (`conforms` / `must change`) plus one justifying line
(ADR-0031 lines 160-166):

```
conforms

The tree already satisfies the restated invariant ... This ADR instructs no code change.
```

For D1-D4 (all defer-with-trigger or withdraw per D-04), expect `conforms` — no code changes
execute this phase (D-13). Use the words "defer, trigger: <named condition>, owner: <phase/ID>"
inside `## Decision`, not `## Code Conformance`, which is reserved for the conforms/must-change
verdict about the *shipped tree today*.

**Downstream Consumers pattern** (ADR-0031 lines 168-182) — bulleted, one entry per phase/
requirement that inherits the decision, naming exactly what it inherits:

```
- **Phase 11 / FACADE-02** — D2/D3/D4's leaf-to-leaf relocation targets are chosen against this
  restated invariant ...
- **Phase 15** — the `cargo tree --no-default-features` check joins ...
```

---

### `.planning/decisions/003N-paladin-ml-placement.md` (ADR, contested position)

**Analog:** `.planning/decisions/0018-m6-facade-reexport-policy.md` — read for its posture-ADR
shape (a "this stays out, this condition governs re-entry" ruling). Same seven-heading skeleton
as above applies; `## Decision` should state the placement condition verbatim per D-09:

> any future TensorFlow adapter goes into a dedicated `paladin-ml` **leaf crate** with the `ml`
> flag on that crate, **never back into the facade**, and `paladin_ports::input::ml_port::MlPort`
> **stays in the workspace** so the integration point does not move.

`## Code Locations` must cite `crates/paladin-ports/src/input/ml_port.rs` (confirmed present,
RESEARCH.md Code Examples) and `PROJECT.md`'s `### Out of Scope` line naming `paladin-ml`.

---

### `.planning/decisions/PROMOTION.md` (index amendment, last act per D-14)

**Analog:** the file's own existing dated notes (lines 59-91) — each prior phase that authored
multiple ADRs in one phase appended a dated note explaining the jump, rather than silently
advancing the number. Copy this exact shape:

```markdown
*Dated note, 2026-08-08 (plan 11-0N):* the line advances by **N** in one phase, from 0034 to
003X, because Phase 11 authored N ADRs across its own plans — [name each, e.g.] plan 11-02
authored ADR-0034 (D1-D4 disposition set), plan 11-03 authored ADR-0035 (`paladin-ml` placement).
`ls .planning/decisions/00{34,35}-*.md` (re-run before writing this note) confirms both files
exist with contiguous numbers; none was skipped or reused.
```

Also add a row to the **Numbering index** table (lines 21-55) in the same `| Number | Slug |
Subject |` shape, e.g.:

```
| 0034 | `d1-d4-facade-relocation-disposition` | D1-D4 verdicts — src/core/ shims defer, user_service split withdrawn, D3/D4 defer-with-trigger under ADR-0031 (Phase 11) |
```

Update the "Next free ADR number" line (currently `57:**Next free ADR number: 0034**`) **last**,
per D-14 and `PROMOTION.md:141-150`'s six-step procedure. Also add a `## Key Decisions` row to
`.planning/PROJECT.md` per step 6 of that procedure.

---

### `.planning/ledgers/milestone-07-08.md` (5 named rows, cell-replacement-in-place)

**Analog:** the same file's own prior Phase-10 row amendments — e.g. `REQ-extracted-crate-
dependency-rule` (line 186) and `REQ-content-processing-build-gate` — both amended by Phase 10
citing an ADR this same phase wrote, in place, no row insertion/deletion/reorder.

**Row-amendment pattern** (verbatim structure, verdict cell + evidence cell only):

```
| REQ-m8-deferred-items-register | satisfied | **[what changed].** [ADR/register citation]
(`.planning/decisions/003N-....md`) [restates|corrects] [claim]. Re-confirmed this session:
[file:line / grep output]. This is a wording/framing fix, not a code fix — [why]. |
```

**Contract to respect** (`milestone-07-08.md:144-171`): "Later plans replace a row's Verdict and
Evidence cells in place; they never insert, delete, or reorder rows." Follow the same file's own
"Ledger file contention" table shape if multiple Phase-11 plans touch this ledger — declare which
plan owns which rows, run disjoint.

**Verdict vocabulary to reuse** (legend at lines 79-89): `satisfied` / `present, unproven` /
`genuinely outstanding` / `relocated` / `superseded by outcome` / `deferred with register` /
`diverged`. FACADE-01's D5 disposition and FACADE-03's CLI/ML records are natural fits for
`deferred with register` — the exact verdict word this legend defines for "work was removed
deliberately and recorded with a reintroduction condition."

---

### New `.planning/` register/triage file (FACADE-01 + FACADE-03 + FACADE-04 content)

**Analog (table structure):** `.project/Milestone_8-.../Epic_3/infrastructure-adapter-
disposition.md` — 20-row table, `## Purpose` numbered list of what each row states, dated header.

**Analog (verdict vocabulary + evidence bar):** `.planning/ledgers/milestone-07-08.md`'s
`## Verdict legend` (lines 79-89) and its evidence-bar prose (lines 16-22): *"A `satisfied` verdict
requires a `file:line` citation plus a named passing test, example, or command that exercises it."*

**Recommended per-file/per-row shape for FACADE-01** (17 rows, grouped per file per D-01's "done
when" clause):

```markdown
### `src/application/services/herald/herald_registry.rs` (4 occurrences)

| Line | Snippet | Disposition |
|---|---|---|
| 165 | `println!("Available formatters: {:?}", available_formatters);` | rustdoc example (inside `rust,ignore` fence, `:163`) — deliberate stdout, not runtime library code. No conversion. |
| 184 | `println!("JSON formatter is available");` | rustdoc example (inside `rust,ignore` fence, `:182`) — deliberate stdout. |
...
```

Use the exact re-verified `file:line` set from RESEARCH.md's Code Examples section (165/184/197/
210 for `herald_registry.rs` — **not** 163/182/195/208, per Pitfall 3).

**Recommended shape for FACADE-03** (CLI + ML feature register): mirror `deferred-features.md`'s
own two-write-up shape (feature name / what it was / current state / reintroduction condition /
recovery pointer), but relocate it into `.planning/` per D-09/D-10's "findable from `.planning/`
without reading `.project/`" constraint. Recovery pointer must be the commit SHA, not the branch
(D-10): `git show 3d48768^:src/application/cli/commands/user.rs`.

**Recommended shape for FACADE-04** (20-row triage): copy the FACADE-04 Verification Table
directly from RESEARCH.md (already fully pre-computed, all 20 rows) into the new file's own table,
with a `done` / `not a candidate` / `still open` disposition column replacing the narrative
`Disposition` column's prose with the three required words plus the one-line reason.

---

### `.planning/ROADMAP.md` §Phase 11 criterion 1 (D-02 amendment)

**Analog:** Phase 10's own precedent for amending a governing document in place while retaining
superseded text — same convention `.planning/decisions/0031-....md:149-150` describes: "plan
10-01 corrects the line in place." Apply the same in-place, dated, superseded-text-retained
convention (D-00c/D-00d) to `ROADMAP.md:726`. Do not delete the original criterion text; strike or
annotate it and state the corrected criterion beneath, dated.

---

### `.project/Milestone_8-.../deferred-items.md` (Shape-A banner)

**Analog — Shape A, Phase 8 commit `94814ff`:** inline struck-and-corrected clause. RESEARCH.md
names this shape explicitly as the fit for D5's framing correction because "a specific clause...
is what's wrong, not the whole document" (as opposed to Shape B's whole-document blockquote).
Apply to the specific clause rating D5 "low effort / low risk, the quick win" — strike it, insert
the corrected framing inline, retain the original struck text, date the correction.

**Contrast — Shape B, already live on `infrastructure-adapter-disposition.md:1-12`** (Phase 10
plan 10-02, quoted in full in RESEARCH.md's Code Examples): a blockquote at the top naming what's
superseded, with a link to the governing ADR, original text retained unmodified below. Do **not**
duplicate this banner on `infrastructure-adapter-disposition.md` (Pitfall 2) — it already exists.

## Shared Patterns

### ADR file shape (no frontmatter, 7 fixed headings)
**Source:** `.planning/decisions/PROMOTION.md:95-116`
**Apply to:** every new ADR this phase writes (D1-D4 disposition set, `paladin-ml` placement)
```
## Status
## Context
## Decision
## Considered Options   <- bulleted list, never prose (adr-parser.cjs requirement)
## Code Locations       <- bulleted list, never prose
## Code Conformance     <- `conforms` | `must change`, one line
## Downstream Consumers
```

### Evidence bar (D-00e) — no claim without the exact command/`file:line`
**Source:** `.planning/ledgers/milestone-07-08.md:16-22`, applied throughout ADR-0031's `## Code
Locations`/`## Context`
**Apply to:** every disposition record this phase writes (ADRs, ledger rows, the new register)
```
A `satisfied` verdict requires a `file:line` citation **plus** a named passing test, example, or
command that exercises it. A `file:line` citation with nothing exercising it is not `satisfied` —
it gets its own verdict, `present, unproven`.
```

### Dated correction banner, two shapes (D-00c)
**Source:** Phase 8 commit `94814ff` (Shape A, inline struck clause); Phase 9/10 precedent, live
on `infrastructure-adapter-disposition.md:1-12` (Shape B, blockquote pointer)
**Apply to:** `deferred-items.md` (Shape A, new — D5 framing), any further extension of the
already-banner-carrying `.project/` documents (Shape B, extend only, do not duplicate)

### Ledger amendment in place, never a separate corrections file (D-00d)
**Source:** `.planning/ledgers/milestone-07-08.md:144-150`
**Apply to:** the 5 named rows this phase closes against
```
Later plans replace a row's **Verdict** and **Evidence** cells in place; they never insert,
delete, or reorder rows. Amendments follow D-00d: edit in place, retain superseded text, date
every amendment, never a separate corrections file.
```

### ADR promotion procedure, six steps (D-14's mechanism)
**Source:** `.planning/decisions/PROMOTION.md:139-150`
**Apply to:** every new ADR this phase writes, as the closing checklist
```
1. Take the next free number from the Numbering index line, decrementing nothing.
2. Author into the standard heading set.
3. Set ## Code Conformance to conforms/must change, naming the executing requirement if
   must change.
4. Cite the source document's path in ## Code Locations.
5. Update the "Next free ADR number" line — last act (D-14).
6. Add a row to PROJECT.md's ## Key Decisions table.
```

## No Analog Found

None. Every file class this phase produces (ADR, ledger row, `.project/` banner, new register)
has at least one directly reusable existing analog in the corpus, cited above.

## Metadata

**Analog search scope:** `.planning/decisions/` (9 ADRs read/grepped: 0018, 0028, 0031 in full;
PROMOTION.md in full), `.planning/ledgers/milestone-07-08.md` (read through Milestone 7 Epic 3),
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/` (deferred-items.md,
infrastructure-adapter-disposition.md headers/banners read).
**Files scanned:** 3 ADRs full-read, 1 ledger partial-read (550 lines, first 219 read — sufficient
for row-amendment pattern), PROMOTION.md full-read, 2 `.project/` documents partial-read (banners
+ headers).
**Pattern extraction date:** 2026-08-08
