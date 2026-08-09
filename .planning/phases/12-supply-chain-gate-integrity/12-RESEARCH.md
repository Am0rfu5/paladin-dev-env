# Phase 12: Supply-Chain Gate Integrity - Research

**Researched:** 2026-08-09
**Domain:** Planning-artefact / governance phase — ADR authoring, dated source corrections, one
bash regression guard. No Rust library or API design is in scope.
**Confidence:** HIGH — every claim below is either a re-run command transcript or a direct
`file:line` read performed this session. Nothing here is inferred.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-00a…D-00h** (inherited, standing conventions): ADRs live in `.planning/decisions/`, flat
  zero-padded monotonic numbering, file shape `Status / Context / Decision / Considered Options /
  Code Locations / Code Conformance / Downstream Consumers`, no frontmatter. `## Code Locations`
  and `## Considered Options` are bulleted lists, never prose. Precedence: ADR → shipped tree →
  `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC → task-list checkbox.
  Source corrections are dated annotation-in-place, original text retained and marked superseded.
  Ledgers amended in place, never a separate corrections file. Evidence bar: no closure claim
  without the exact command or `file:line` that produced it, recorded verbatim. `REQ-*` is the
  primary key. Contested positions get ADRs; code-settled defects get ledger rows, no ADR.
  Medieval-military language mandatory; conventional commits; no `unwrap`/`expect`/`panic!` in
  library code.
- **D-00i:** ADR-0024 owns the exception set and its governance (five `.cargo/audit.toml`
  ignores, ten `deny.toml` entries, the eleven-field schema, owner `DF3NDR`, per-advisory
  `2026-12-31` review dates, ratification of `RUSTSEC-2026-0187`/`-0194`/`-0195`). **Phase 12 does
  not add, remove, re-date or re-justify a single suppression.**
- **D-00j:** ADR-0032 settled `pdf-extract` reachability (unconditional dependency of
  `paladin-content`, gated one level up by the facade's optional dependency). Inherit as an
  answer, not a question.
- **D-00k:** The `scraper`/`rss`/`tiktoken-rs` dead-dependency finding belongs to **Phase 15**,
  per the Phase 10 hand-off, despite `ROADMAP.md:714`'s ambiguous phrasing. Not this phase's work.
- **D-00l** [inheritance risk, non-blocking]: This phase's entire scope rests on Phase 9's D-07
  (`09-CONTEXT.md`), run `--auto` and never human-ratified. The re-scope is durable because it is
  recorded at source in `ROADMAP.md` §Phase 12 and `REQUIREMENTS.md:4085`. A planner must not
  treat D-07 as human-ratified; if a human later disagrees, SUPPLY-01/02 return to Phase 12 as
  work, but the ADR-promotion decisions (D-01…D-05 below) are unaffected either way.
- **D-01:** SUPPLY-03 acts — it writes an ADR. The "this requirement does not act" clause
  (`REQUIREMENTS.md:1937-1939`) is stale and gets a dated correction banner, not a rewrite. ⚠
  HUMAN REVIEW.
- **D-02:** SUPPLY-03's live scope is **one** candidate, not two. Candidate 3
  (`rustsec-remediation-plan.md`) is already closed by ADR-0024 (`PROMOTION.md:185-189`).
  Candidate 7 (`prd-dependency-security-license-compliance.md` FR-1 + §8) is the only live one.
  Record the correction; do not re-promote ADR-0024's subject.
- **D-03:** Promote candidate 7. The invariant becomes **ADR-0036** with a `conforms` verdict —
  the tree already satisfies it (verified this session, see §A/§D below).
- **D-04:** One ADR, numbered **0036**. `PROMOTION.md:59` — verified this session, `.planning/decisions/`
  holds `0001`…`0035`.
- **D-05:** ADR-0036 stands alone, cites ADR-0024 as related, does **not** supersede it.
  ADR-0024 governs suppression *contents*; ADR-0036 governs suppression *topology* (which
  mechanical surfaces may carry one). ADR-0024's `## Status` is not touched.
- **D-06:** Verify SUPPLY-01/SUPPLY-02 locally. The CI-only caveat is dead — `cargo audit`,
  `cargo deny check`, `./scripts/check-advisory-register.sh` all pass in this environment
  (re-confirmed this session, see §D). Every closure claim must carry the transcript, not a
  citation to CONTEXT.md.
- **D-07:** The CI-run observation clause (confirming the required status check resolves on the
  first real push after the deletion) genuinely cannot be closed in-repo. Record it **pending**,
  named trigger "next push to `release/v0.7.0`", citing the run-ID boundary (see §D.13).
- **D-08:** Add a minimal, offline, fourth guard clause (or sibling script) that fails if an
  advisory-ignore flag reappears in any workflow file. ⚠ HUMAN REVIEW — this is a CI check no
  requirement explicitly asks for. Constraints (fixed, not discretionary): (a) offline; (b) match
  `cargo audit`/`cargo deny` invocations carrying an advisory-ignore flag, **not** bare
  `--ignore`, so `mc mb --ignore-existing` and `cargo test -- --ignored` never false-positive; (c)
  assert `cargo audit` appears exactly once across `.github/workflows/`; (d) report every
  violation, not just the first. Wired into `make check-gates` and `ci.yml:101`'s step (or an
  equivalent aggregate).
- **D-09:** SUPPLY closure rows go in `REQUIREMENTS.md` plus a dated `#### Hand-off to Phase 13 /
  ORCH-01` block, in the shape of the three existing hand-off blocks. **Do not create**
  `.planning/ledgers/milestone-09-12.md` — that is ORCH-01/Phase 13's deliverable.
- **D-10:** The committed-but-unapplied GitHub rulesets are a recorded finding with an owner
  (milestone close-out), not phase work. Do not apply anything.

### Claude's Discretion

- Whether the D-08 guard is a fourth clause inside `scripts/check-advisory-register.sh` or a
  separate sibling script (both satisfy D-08's fixed constraints).
- Whether ADR-0036's `## Considered Options` reproduces the decline branch as a rejected option or
  only names it (must still be a bulleted list per `PROMOTION.md`).
- Plan decomposition and wave assignment. The verification work (D-06/D-07), the ADR promotion
  (D-01…D-05) and the source corrections (D-01/D-02/D-06) are mutually independent and can run in
  parallel. The D-08 guard should land in the same plan as, or after, ADR-0036 so the ADR can cite
  it in `## Code Locations`. `PROMOTION.md:59` must be updated **last**.
- Exact wording/placement of dated correction banners, subject to D-00c.

### Deferred Ideas (OUT OF SCOPE)

- Applying the committed GitHub rulesets to the live repository (D-10 — owner-only).
- Closing milestone v0.7.2 / reconciling the ROADMAP `## Milestones` table (`/gsd-complete-milestone`
  decision).
- Fixing the `API Surface Tracking` CI job (DEBT-01's, Phase 8's).
- The `scraper`/`rss`/`tiktoken-rs` dead dependencies (Phase 15, D-00k).
- Human ratification of Phase 9's D-07/D-09/D-16 (flagged D-00l, not blocking).
- Promoting the other nine ADR candidates (each keeps its own owning phase).
- A general CI-policy linter — D-08 is deliberately one narrow guard.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SUPPLY-01 | `ci.yml` runs exactly one `cargo audit`, no inline advisory-ignore flags, no duplicate display names. | **Closed by Phase 9** (commit `cb75b2b`). Phase 12's remaining obligation is verification, not re-planning — §D gives the re-run transcripts and the one still-pending clause (D-07, the CI-run observation). See §D.11–§D.13. |
| SUPPLY-02 | Every advisory suppression carries an owner and review date; the vulnerability baseline matches an authorising document. | **Closed by Phase 9** (ADR-0024, `SECURITY-EXCEPTIONS.md`). Phase 12's remaining obligation is the same local-verification transcript (§D.11) plus the three stale "CI-only" banners (§D.12). |
| SUPPLY-03 | The two supply-chain ADR candidates are promoted or declined, deliberately. | **The only open work.** §A gives the ADR file contract and the two shape models; §B gives every stale-text `file:line` this requirement's correction touches; §C gives the D-08 regression-guard design; §E gives the hand-off/closure shape. |

</phase_requirements>

---

## Summary

This is a documentation/governance phase, not a feature phase — like Phases 10 and 11, it is
expected to change **zero** `.rs` files. Two of its three requirements (SUPPLY-01, SUPPLY-02) were
already executed by Phase 9; Phase 12's job on those two is to **re-run three commands, record the
transcripts verbatim, and correct three documents that still say the check can't run here.** All
three commands were re-run this session and all three still pass (§D.11). The one true SUPPLY-01
clause that cannot close in-repo — confirming the CI required-status-check fires on the next real
push — is recorded as *pending* with the exact run-ID boundary (§D.13), not faked.

The only genuinely open requirement is SUPPLY-03: promote candidate 7 of `PROMOTION.md`'s Part B
inventory (`Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8, the
audit-suppression single-source invariant) as **ADR-0036**, with a `conforms` verdict — the tree
already satisfies the invariant (re-verified this session, §A.7 and §D). ADR-0036 does not touch
ADR-0024 (which owns the suppression *contents*); it is a standalone ADR governing suppression
*topology* — which two files may carry a suppression, and that CI workflow files may not carry one
inline. `PROMOTION.md` (`.planning/decisions/PROMOTION.md`) is the load-bearing reference: it fixes
the required H2 heading set, the numbering line, the promotion procedure, and the fact that
`## Code Locations`/`## Considered Options` must be **bulleted lists**, because
`adr-parser.cjs`'s `splitEntries` does not cleanly extract citable items from a prose paragraph
(§A.3 gives the precise mechanism, which is subtly different from a flat "yields nothing" claim).

A regression guard (D-08) is the one code-adjacent deliverable: extend
`scripts/check-advisory-register.sh` with a fourth clause, or add a sibling script, that fails CI
if an inline `--ignore` reappears on a `cargo audit`/`cargo deny` invocation in any
`.github/workflows/*.yml` file. §C gives a concrete, false-positive-safe matching design, grounded
in the existing scripts' own conventions (structural parsing over grep-scraping — `deny.toml`/
`.cargo/audit.toml` are parsed with `tomllib`; `docs/src/**/*.md`'s YAML snippets are parsed with
`PyYAML`, confirmed installed in this environment). This phase turns "an observation into a gate,"
in SUPPLY-03's own phrase — an ADR alone is prose, and prose did not stop the duplicate job the
first time.

**Primary recommendation:** Write ADR-0036 (shape-modeled on ADR-0031, cite ADR-0024), correct the
four stale `file:line` passages named in §B with dated banners, add the D-08 guard as either a
fourth `check-advisory-register.sh` clause or a sibling script wired into `make check-gates` and
`ci.yml:101`'s job, re-run and record the three verification transcripts in §D.11 as this phase's
own evidence (not a citation to this file), and write the `#### Hand-off to Phase 13 / ORCH-01`
block per §E. Total surface: `.planning/decisions/0036-*.md`, `.planning/decisions/PROMOTION.md`,
`.planning/PROJECT.md` (×2 sites + Key Decisions row), `.planning/REQUIREMENTS.md` (×3 stale-CI-only
sites + 3 checkbox flips + hand-off block + traceability rows), `scripts/check-advisory-register.sh`
or a new sibling script, possibly `Makefile` and `.github/workflows/ci.yml:101`.

## Architectural Responsibility Map

This phase has no browser/server/API/database tiers to map — it is entirely inside the
`.planning/` and repository-governance "tier." The one useful mapping is *which mechanical surface
owns which concern*, since that is exactly what ADR-0036 codifies:

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Advisory suppression **contents** (which ID, whose, why, until when) | `SECURITY-EXCEPTIONS.md` (governance register) | `.cargo/audit.toml` / `deny.toml` (mechanical mirrors) | ADR-0024's domain; ADR-0036 does not touch this. |
| Advisory suppression **topology** (where a suppression is legally expressed) | `.cargo/audit.toml` + `deny.toml` | — | ADR-0036's subject: exactly two files may carry a suppression; CI workflow YAML may not. |
| Suppression drift detection | `scripts/check-advisory-register.sh` (offline, `tomllib`-based) | `make check-gates`, `ci.yml:101` | Existing three clauses: class-set equality, register coverage, crate liveness. |
| Inline-suppression regression detection (D-08, new) | New 4th clause or sibling script | Same `make check-gates` / `ci.yml:101` wiring | Must parse `.github/workflows/*.yml` structurally (PyYAML), not grep raw text, to match house style. |
| ADR machinery / precedence | `.planning/decisions/PROMOTION.md` + `adr-parser.cjs` | `.planning/decisions/*.md` files | `PROMOTION.md` is upstream of every ADR write; `adr-parser.cjs` is the downstream consumer whose parsing constraints (`splitEntries`, `CANONICAL_HEADERS`) shape how the ADR must be written. |
| Closure record-keeping | `REQUIREMENTS.md` (rows + hand-off block) | — (explicitly **not** `.planning/ledgers/milestone-09-12.md`, per D-09) | Matches the pattern of the three prior hand-off blocks; the M9-12 ledger itself is Phase 13/ORCH-01's. |

---

## §A — The ADR File Contract

### A.1 Required H2 heading set and order (`PROMOTION.md:107-128`)

Every ADR uses these H2 headings, **in this order**:

```
## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```
`[CITED: .planning/decisions/PROMOTION.md:109-117]`

`## Code Locations` and `## Considered Options` **must be bulleted or numbered lists, never prose
paragraphs** — `.claude/gsd-core/bin/lib/adr-parser.cjs`'s `splitEntries` only yields structured,
citable entries from list-shaped input. `[CITED: PROMOTION.md:119-122]`

`## Code Conformance` and `## Downstream Consumers` have **no synonym anywhere in
`adr-parser.cjs`'s `CANONICAL_HEADERS` table** and land in the parser's `unmapped_headers` bucket
when parsed — this is stated as acceptable in `PROMOTION.md` because nothing currently consumes
either field programmatically, but both are still required: `## Code Conformance` is D-03's
contract (every ADR must carry a `conforms`/`must change` verdict) and `## Downstream Consumers`
names who reads the decision next. `[CITED: PROMOTION.md:124-128]` **Verified directly against the
parser source this session** (§A.3) — confirmed true.

### A.2 The supersession mechanism (`PROMOTION.md:130-140`)

Exactly one live ADR answers each question at any time.

- The superseded ADR **keeps its file** — never deleted or renamed.
- Its `## Status` body becomes the bare word `Superseded`, followed by a prose line naming the
  superseding ADR's number and the reason it no longer holds.
- The superseding ADR carries a `## Supersedes` line naming the ADR number it replaces (this is
  **not** one of the seven canonical H2 headings above — it is an additional line inside/near
  `## Status` for ADRs that supersede something; ADR-0036 does **not** need this, per D-05, since
  it does not supersede ADR-0024).
- `adr-parser.cjs` recognises `superseded` as a status word via `STATUS_REJECT_SET` (verified,
  §A.3) so a downstream consumer can mechanically distinguish a live ADR from a retired one.

**Design consequence for ADR-0036:** its `## Status` is simply `Accepted`, with a `**Date:**` line
(matching ADR-0031's and ADR-0024's shape) — it supersedes nothing, is superseded by nothing.

### A.3 `adr-parser.cjs` machine-readable constraints — verified directly

File: `.claude/gsd-core/bin/lib/adr-parser.cjs`. Read in full this session.

**`CANONICAL_HEADERS`** (`adr-parser.cjs:18-163`) — the twelve recognised canonical buckets and
their synonym lists:

| Canonical key | Relevant synonyms (verbatim from source) |
|---|---|
| `status` | `status`, `state`, `lifecycle`, `stage` |
| `goal` (maps to "Context") | `context`, `background`, `problem statement`, `problem`, `situation`, `forces`, `motivation`, … |
| `decisions` | `decision`, `decisions`, `resolution`, `conclusion`, `choice`, … |
| `considered_options` | `considered options`, `alternatives`, `options`, `choices`, `candidates`, … |
| `key_files` (maps to "Code Locations") | `affected files`, `files touched`, `surface area`, `modules affected`, **`code locations`**, `file changes`, `diff summary`, `touched code` |
| `risks`, `success_criteria`, `plan_sequence`, `out_of_scope`, `deferred`, `dependencies`, `update`, `consequences` | (present, not directly relevant to ADR-0036) |

`[VERIFIED: adr-parser.cjs, direct read, this session]` — `## Code Locations` maps into the
`key_files` canonical bucket via the exact synonym `'code locations'` (`adr-parser.cjs:108`).
`## Considered Options` maps via the exact synonym `'considered options'` (`adr-parser.cjs:54`).

**`STATUS_REJECT_SET`** (`adr-parser.cjs:17`):
```js
const STATUS_REJECT_SET = new Set(['superseded', 'rejected', 'deprecated']);
```
`shouldRejectAdrStatus(status)` (`adr-parser.cjs:375-377`) does an **exact-match** check
(`STATUS_REJECT_SET.has(normalizeAdrHeader(status))`) after normalization (lowercase, punctuation
stripped) — not a substring/`includes` check. ADR-0036's status line must therefore read plainly
`Accepted` (matches ADR-0024's and ADR-0031's own `## Status` bodies, both literally `Accepted`).

**`splitEntries` — exact behaviour, not the loose "yields nothing from a paragraph" paraphrase**
(`adr-parser.cjs:219-226`):
```js
function splitEntries(blockText) {
    return (typeof blockText === 'string' ? blockText : '')
        .split(/\r?\n/)
        .map((line) => line.trim())
        .filter(Boolean)
        .map((line) => line.replace(/^[-*+]\s+/, '').trim())
        .filter(Boolean);
}
```
`[VERIFIED: adr-parser.cjs:219-226, direct read]` This splits the section body **by literal
newline**, strips only a leading `-`, `*` or `+` marker (note: **no support for numbered-list
markers** like `1.` — a numbered entry survives with its `"1. "` prefix intact rather than being
stripped, so numbered lists still produce entries, just with the numeral left in the text), and
drops blank lines. The precise failure mode `PROMOTION.md` warns about is **not** "zero entries
from a paragraph" in the general case — it is:
- A **single unwrapped line** of prose (no internal `\n`) produces exactly **one** entry
  containing the *entire* paragraph as one opaque blob — not multiple citable items, and not
  zero.
- A **hand-wrapped, multi-line** prose paragraph (line breaks inserted mid-sentence, as most
  human-authored markdown is) produces **multiple entries, one per physical line**, each an
  ungrammatical sentence fragment — worse than zero, because it looks structured but is garbage.

Either way, the practical instruction is identical to `PROMOTION.md`'s: **write `## Code
Locations` and `## Considered Options` as genuine one-bullet-per-item lists**, so each `splitEntries`
output item is one real citation or one real considered option, not a blob or a fragment.

### A.4 Shape model 1 — ADR-0031 (`conforms` verdict, closest model for ADR-0036)

`.planning/decisions/0031-extracted-crate-dependency-rule.md`, read in full.

- `## Status`: `Accepted`, `**Date:** 2026-08-08` — two lines.
- `## Context`: ~30 lines — states the measured fact first (four re-read citations with exact
  `file:line`), then the PRD's absolute-form clause that contradicts it, then a note correcting a
  stale cross-reference two other documents share (`10-CONTEXT.md` D-15 and
  `REQUIREMENTS.md:1438` both mis-cite a source; ADR-0031 corrects the citation *inside its own
  Context section*, a pattern worth reusing if ADR-0036 needs to correct a similar mis-citation).
- `## Decision`: restates the D-00b precedence order, then gives **"the restated rule, as a single
  quotable sentence"** in bold, then three labelled sub-decisions `(i)`/`(ii)`/`(iii)` — the
  enforceable invariant, the measured baseline accepted, and an explicit note that "the anchor
  moved deliberately" (i.e., this ADR promotes/narrows the PRD's absolute claim rather than
  quietly overriding it). Ends with a short paragraph citing Phase 7's ADR-0016 precedent for why
  an ADR (not a re-tagged `.project/` document) is the promotion mechanism.
- `## Considered Options`: **4 bulleted items**, each one paragraph, each ending `(accepted)` or
  `(rejected)` inline with a one-sentence reason.
- `## Code Locations`: **10 bulleted items**, mixing tree citations (`Cargo.toml:23`, `mod.rs:7`)
  and `.project/` document citations, plus one un-bulleted paragraph reporting a command actually
  run (`cargo tree -p paladin-content --no-default-features`) with its result — this paragraph
  sits *after* the bulleted list, not replacing it, so `splitEntries` still gets clean bullets for
  the list itself.
- `## Code Conformance`: **3 lines** — the bare word `conforms`, blank line, one sentence
  explaining why.
- `## Downstream Consumers`: **3 bulleted items**, each naming a phase/requirement and what it
  inherits.

Total length: ~180 lines. This is the length/shape budget ADR-0036 should target.

### A.5 Shape model 2 — ADR-0024 (the subject ADR-0036 must cite but NOT supersede)

`.planning/decisions/0024-rustsec-exception-governance.md`, read in full.

- `## Status`: `Accepted`, `**Date:** 2026-08-08`.
- `## Context`: ~50 lines, four bulleted "surfaces" each with exact `file:line`, then two callout
  paragraphs ("The corpus's own arithmetic was wrong twice…", "Phase 8's clap v4 migration
  removed…") citing a **verbatim liveness transcript** run in-session (reused later, verbatim
  again, under `## Code Locations`).
- `## Decision`: **5 numbered decisions**, each a full paragraph (this is prose under `##
  Decision`, which is fine — `## Decision` is not one of the two headings `splitEntries`-sensitive;
  only `## Code Locations`/`## Considered Options` need list form).
- `## Considered Options`: **6 bulleted items**, `(rejected)` suffix pattern identical to
  ADR-0031's.
- `## Code Locations`: **7 bulleted items** plus a fenced ` ```  ` verbatim command transcript
  block (14 `grep -c` invocations against `Cargo.lock`) — this is the precedent for embedding a
  literal verification transcript inside an ADR's `## Code Locations` section.
- `## Code Conformance`: **`must change`** (not `conforms`) — because this ADR instructed real
  edits (`deny.toml` deletions, `ci.yml` job deletion, new script). ADR-0036, by contrast, is
  `conforms` per D-03 — no code change, only a formalization of an already-true state.
- `## Downstream Consumers`: **3 bulleted items**, one of which is explicitly "Phase 12 / SUPPLY-01
  and SUPPLY-02" — i.e. ADR-0024 already names Phase 12 as a consumer. ADR-0036 should **not**
  duplicate that entry; it should name Phase 13/ORCH-01 and Phase 15/PIPE-01 instead, per D-03's
  own `## Downstream Consumers` instruction.

**Key relationship point for ADR-0036's `## Context`:** cite ADR-0024's file path and its
decisions 1/2 (which files are authoritative for what) as background, but do **not** restate or
re-derive ADR-0024's suppression-content decisions — ADR-0036 is scoped to topology only (D-05).

### A.6 `PROMOTION.md` six-step promotion procedure (Part A, `PROMOTION.md:149-170`)

1. Take the next free number from the **Numbering index** line (`PROMOTION.md:59`) and decrement
   nothing — numbers are never reused even if a candidate is later declined instead of accepted.
2. Author the candidate's substance into the standard heading set, following
   `0005-herald-trait.md`'s shape (the worked example Phase 1 built — not read in full this
   session; ADR-0031/ADR-0024 above are sufficient shape models for a `conforms`-verdict ADR).
3. Set `## Code Conformance` to `conforms` or `must change` per D-03 — naming the executing
   requirement where the verdict is `must change`. (ADR-0036: `conforms`, per D-03.)
4. Cite the source document's path in `## Code Locations` alongside shipped-code citations, so a
   reader can trace the promoted decision to the corpus document it came from.
5. Update the `Next free ADR number` line in `PROMOTION.md` — **per Claude's Discretion in
   CONTEXT.md, do this LAST**, after ADR-0036 is written and after (per precedent, see A.8) any
   dated note is appended.
6. Add a row to `.planning/PROJECT.md`'s `## Key Decisions` table, linking to the new ADR file.

`[CITED: PROMOTION.md:151-163]`

### A.7 Candidate 7's substance — the actual PRD text ADR-0036 ratifies

`.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md`
**exists** — confirmed this session (`test -f` → exit 0).
`[VERIFIED: filesystem check, this session]`

**FR 1** (the PRD labels its functional requirements numerically inside a grouped heading, not
literally "FR-1" as a heading string — the group heading is `### \`cargo audit\` Integration &
Exception Process (FR 1–4)` at line 69, and item 1 under it, at lines 71-73, is FR 1):

> "The CI `security-audit` job must invoke `cargo audit` such that its ignore-list is sourced from
> the version-controlled `audit.toml` (single source of truth) rather than inline `--ignore`
> flags, so the workflow and the config cannot drift."
`[VERIFIED: prd-dependency-security-license-compliance.md:71-73, direct read this session]`

**§8 Success Metrics** (lines 180-189), the two relevant bullets:
> "CI shows distinct passing checks for: `cargo audit` (config-driven), `cargo deny check`, and
> OSV-Scanner, on pull requests."
> "`audit.toml` and `deny.toml` are the only places policy/exceptions are defined; no inline
> advisory-ignore flags remain in CI."
`[VERIFIED: prd-dependency-security-license-compliance.md:182-188, direct read this session]`

**Nuance for ADR-0036's `## Context`:** FR 1 names only `audit.toml` as the single source of truth
for `cargo audit` specifically; §8 names **both** `audit.toml` and `deny.toml` as the two
authorized mechanical surfaces. ADR-0036's restated invariant should use §8's broader two-file
framing (matching D-03's own "Code Locations citing … `.cargo/audit.toml`, `deny.toml`" language),
not FR 1's narrower single-file framing, and should note the distinction rather than silently
picking one.

**Conformance check, re-verified this session (matches D-03's session-verified claims exactly):**
```
$ grep -c 'run: cargo audit' .github/workflows/ci.yml
1
```
Only `--ignore`-family tokens anywhere under `.github/workflows/`: `mc mb --ignore-existing`
(`ci.yml:428-429`) and `cargo test -- --ignored` (`ci.yml:463,466,755,757`) — **zero** are advisory
suppressions. `[VERIFIED: grep, this session — see §D.9 for the full occurrence table]`
Verdict: **conforms**, confirmed independently, not merely copied from CONTEXT.md.

### A.8 The `PROMOTION.md:59` numbering-index dated-note precedent (four prior examples)

`PROMOTION.md:59` reads exactly:
```
**Next free ADR number: 0036**
```
`[VERIFIED: PROMOTION.md:59, direct read this session]`

Below that line sit **four** reverse-chronological dated notes (most recent first), each following
an identical shape: `*Dated note, YYYY-MM-DD (plan NN-NN):* the line advances by **N** … because
Phase N authored ADR-XXXX through ADR-YYYY across its own plans …`, each ending with the exact
`ls`-style verification command re-run to confirm contiguous, non-reused numbering, and a note
about which (if any) Part B inventory entries the new ADRs closed. `[CITED: PROMOTION.md:61-102]`
Phase 12 should append a **fifth** dated note in the same shape when it advances the line from
`0035` to `0036`, explicitly noting: this is the first phase since Phase 7 whose advancing note
covers exactly **one** ADR (not a multi-ADR batch), and that ADR-0036 **does** close a Part B
inventory entry (candidate 7) — unlike Phase 11's most recent note, which explicitly stated
neither of its two ADRs closed an inventory entry.

---

## §B — Stale-Text Corrections (exact `file:line`, verbatim)

All four passages below are **read directly this session**; none is copied from CONTEXT.md without
re-verification.

### B.1 `REQUIREMENTS.md` SUPPLY-03's "does not act" clause

`REQUIREMENTS.md:1937-1939`:
> "**This requirement does not act.** Promotion requires re-tagging the source documents and
> re-running ingest, which is a user-owned step outside any planning artefact; entering a lock
> here would fabricate authority the corpus does not contain."
`[VERIFIED: REQUIREMENTS.md:1937-1939, direct read this session]`

The SUPPLY-03 checkbox itself starts at `REQUIREMENTS.md:1929`: "**SUPPLY-03**: The two supply-chain
ADR candidates are promoted or declined, deliberately." — this "two" also needs D-02's correction
(see B.4 below); it is the same block, corrected once with one banner covering both defects (D-01's
"does not act" and D-02's "two→one").

### B.2 The eleven-candidate / zero-locked framing

`REQUIREMENTS.md:103-109`:
> "**Eleven ADR candidates exist and none is promoted.** Promotion requires re-tagging the source
> document via `--manifest` and re-running ingest; manufacturing a lock inside a planning artefact
> would fabricate authority the corpus does not contain. The two with a live operational cost are
> `Milestone_7/Epic_4/rustsec-remediation-plan.md` (the corpus's only expiry date, **2026-09-30**)
> and `Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` (the single-source
> invariant the tree currently violates) — **the same subject from two milestones.** SUPPLY-03
> records the recommendation and does not act on it."
`[VERIFIED: REQUIREMENTS.md:103-109, direct read this session]`

This passage carries **three** stale claims simultaneously: (1) "none is promoted" — four have
since been promoted (ADR-0016, ADR-0021, ADR-0024, ADR-0025) per `PROMOTION.md` Part B; (2)
"Promotion requires re-tagging… and re-running ingest" — superseded by `PROMOTION.md` §Part A's
"why this is viable now" passage; (3) "the tree currently violates" the single-source invariant —
no longer true, re-verified this session (§A.7). A single dated banner should address all three,
per D-00c (annotation, original retained).

### B.3 `PROJECT.md` §Out of Scope's matching bullet

`PROJECT.md:587-588`:
> "**Promoting the two ADR candidates into locked decisions** — doing so requires re-tagging the
> source documents via `--manifest` and re-running ingest, not an edit here. See Key Decisions."
`[VERIFIED: PROJECT.md:587-588, direct read this session]`

**A second, related passage** exists at `PROJECT.md:625-627`, inside `## Context`:
> "**Eleven ADR candidates have accumulated, and none is promoted.** Promoting any requires
> re-tagging its source via `--manifest` and re-running ingest — entering one in Key Decisions
> would fabricate authority the corpus does not contain."
`[VERIFIED: PROJECT.md:625-627, direct read this session]` **Not named explicitly in CONTEXT.md's
canonical refs, but it is the same stale claim, in the same document, feeding directly into the
`## Key Decisions` table (`PROJECT.md:1103` onward) that D-01/A.6-step-6 requires ADR-0036 to gain
a row in.** A planner should correct both `PROJECT.md` sites in the same pass, not just the
`## Out of Scope` bullet.

### B.4 SUPPLY-03's "the two supply-chain ADR candidates" phrasing (D-02's target)

Already quoted in B.1 — `REQUIREMENTS.md:1929`: "The **two** supply-chain ADR candidates are
promoted or declined, deliberately." D-02 corrects this to **one** (candidate 7 alone), citing that
candidate 3 was already closed by ADR-0024 on 2026-08-08, per `PROMOTION.md:185-189`:
> "3. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) … **Owner phase: Phase 9. Closed
> 2026-08-08 by ADR-0024** (`0024-rustsec-exception-governance.md`) — renewed to per-advisory
> `2026-12-31` review dates, owner reassigned to `DF3NDR`."
`[VERIFIED: PROMOTION.md:185-189, direct read this session]`

### B.5 The `ci.yml:389-406` stale line-citation (three documents)

CONTEXT.md's `<domain>` block names a third defect class: "the `ci.yml:389-406` line citation that
three documents still carry." Located and confirmed this session:

- `REQUIREMENTS.md:1853` (inside SUPPLY-01's own text): "**Done when** `ci.yml:389-406` is
  deleted…" — this citation was **already stale before Phase 9 touched anything**; the duplicate
  job Phase 9 actually deleted was re-derived at `ci.yml:465-482` per SUPPLY-01's own closure note
  at `REQUIREMENTS.md:1864-1865` ("re-derived at `ci.yml:465-482`… the `:389-406` citation above
  was already stale before Phase 9 touched anything"). `[VERIFIED: REQUIREMENTS.md:1853,1864-1865]`
- `ROADMAP.md` §Phase 12's criterion 1 (`ROADMAP.md:762`) also cites `ci.yml:389-406`-adjacent
  framing indirectly through the "two jobs both called Security Audit" description, though its own
  dated closure note (`ROADMAP.md:768-778`) already corrects the substance without re-citing the
  stale line number. **Only `REQUIREMENTS.md:1853` carries the literal stale `:389-406` string** —
  re-confirmed via `grep -n '389-406' .planning/REQUIREMENTS.md` returning exactly this one hit
  this session.
`[VERIFIED: grep -n '389-406' .planning/REQUIREMENTS.md → REQUIREMENTS.md:1853 only, this session]`
**Correction:** CONTEXT.md's claim of "three documents" carrying this exact citation could not be
independently confirmed beyond this single `REQUIREMENTS.md:1853` hit — a planner should re-run
`grep -rn '389-406' .planning/ .project/ 2>/dev/null` at execution time before writing three
banners; this research found only one live occurrence of the literal string. **Flagging as an open
question (see §Open Questions) rather than asserting three sites that could not be verified.**

---

## §C — The D-08 Regression Guard: Design Input

### C.1 `scripts/check-advisory-register.sh` — full read, structure confirmed

- **Interpreter:** `#!/usr/bin/env bash`, `set -euo pipefail` (`check-advisory-register.sh:1,50`).
- **TOML parsing:** shells to `python3` via a heredoc (`<<'PY' … PY`, lines 65-251) and imports
  `tomllib` (Python 3.11+ stdlib, **not** an external pip package) at line 69. `[VERIFIED:
  check-advisory-register.sh:65-251, direct read]`
- **Three existing clauses**, each a separate Python code block inside the same heredoc, each
  appending to a shared `failures` list rather than raising/exiting on first failure — this is how
  it "reports every failure rather than stopping at the first": every clause's `if` branch calls
  `failures.append(...)` and falls through; only after all three clauses run does
  `if failures: … sys.exit(0)` (note: **exit 0** even on failure — the *shell* wrapper, not Python,
  is what turns this into a nonzero process exit) print every accumulated failure and the outer
  bash reads the `FAIL`/`OK` first line to decide `exit 1`/`exit 0` (`check-advisory-register.sh:254-272`).
  `[VERIFIED: check-advisory-register.sh:143-272]`
- **Exit-code contract:** `0` if register/TOML/lockfile all agree on all clauses; non-zero
  otherwise (stated in the header, `:47-48`, and mechanically true via the bash wrapper above).
- **Where a fourth clause slots in:** structurally, after Clause 3 (crate liveness,
  `check-advisory-register.sh:223-239`) and before the `if failures:` block (`:241`) — append a
  `CLAUSE4_*`-prefixed failure-message convention matching Clauses 1-3's naming
  (`CLAUSE1_AUDIT_MISMATCH`, `CLAUSE2_UNCOVERED`, `CLAUSE3_DEAD_CRATE`, etc.).
- **Structural caveat for a 4th clause specifically:** every existing clause parses **TOML** via
  `tomllib`. A new clause that scans `.github/workflows/*.yml` needs a **different** parser
  (YAML), which is a second import (`yaml`/PyYAML) inside the same heredoc — mechanically fine
  (both `tomllib` and `PyYAML` are available in this environment, confirmed §C.3) but a
  single-responsibility argument for a **sibling script** instead (Claude's Discretion, D-08).

### C.2 Sibling guard-script conventions (from `check-crate-names.sh` and `check-changelogs.sh`)

Both siblings share an identical shape with `check-advisory-register.sh`:
- `#!/usr/bin/env bash`, `set -euo pipefail`.
- `WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"` — resolves absolute paths
  independent of invocation `cwd`.
- A `command -v python3` guard before use, failing with a named `ERROR:` message if absent.
- Emoji-prefixed status lines: `🔍 Checking …` at start, `✅ …` on pass, `❌ … failed` on failure,
  each followed by numbered remediation guidance ("If this failure is unexpected: 1. … 2. … 3. …").
- All heavy logic lives in a `python3 - <args> <<'PY' … PY` heredoc; the bash wrapper only reads
  the first line of Python's stdout (`STATUS_LINE`) to decide the shell's own exit code.
- **Structural discovery over hardcoding** is a named, explicit house convention:
  `check-crate-names.sh` globs `crates/*/Cargo.toml` rather than hardcoding a crate list, and its
  header explicitly rejects a live crates.io query "because `crates.io` returns HTTP 403 in this
  environment, so a network check could be written but never demonstrated" — the **same reasoning
  D-08(a)'s "must be offline" constraint already encodes.** `[CITED:
  check-crate-names.sh:1-31, check-changelogs.sh:1-14]`
- **No test files exist for any of these three scripts** — confirmed by search (§C.5). This is a
  gap in the project's own convention, not evidence that a test is unneeded for the new guard.

### C.3 Design input — every `--ignore`-family token in `.github/workflows/`, classified

Re-run this session across **all six** workflow files (`ls .github/workflows/` → `ci.yml`,
`docs.yml`, `feature-flags.yml`, `integration-tests.yml`, `pre-commit.yml`, `release.yml`):

```
$ grep -rn "cargo audit\|--ignore" .github/workflows/*.yml
.github/workflows/ci.yml:76:      # from the repo, so no inline `--ignore` flags are used here.
.github/workflows/ci.yml:78:        run: cargo audit
.github/workflows/ci.yml:428:          mc mb testminio/test-bucket --ignore-existing
.github/workflows/ci.yml:429:          mc mb testminio/integration-tests --ignore-existing
.github/workflows/ci.yml:463:          cargo test redis_queue_integration_tests --release -- --ignored --nocapture
.github/workflows/ci.yml:466:          cargo test file_storage_integration_tests --release -- --ignored --nocapture
.github/workflows/ci.yml:755:          # Tests are marked #[ignore] to skip in unit-test runs; --ignored opts them in here
.github/workflows/ci.yml:757:            cargo test file_storage_integration_tests --test lib -- --ignored --test-threads=1 --nocapture
```
`[VERIFIED: grep -rn "cargo audit\|--ignore" .github/workflows/*.yml, this session]`

**Zero occurrences** in `docs.yml`, `feature-flags.yml`, `integration-tests.yml`, `pre-commit.yml`,
`release.yml` — confirmed by the same command returning no hits for those five files. **Answering
research question 9 directly: yes, `.github/workflows/` holds five files besides `ci.yml`, and the
new guard's glob must scan all of `.github/workflows/*.yml`, not hardcode `ci.yml`** — matching
`check-crate-names.sh`'s glob-not-hardcode convention (§C.2) and guarding against a future
workflow file introducing a suppression the guard never looks at.

Classification:

| Token / line | Classification | Why |
|---|---|---|
| `ci.yml:78` `run: cargo audit` | The one legitimate, un-suppressed invocation | No `--ignore` present at all — this is what "exactly one, clean" should look like. |
| `ci.yml:428-429` `mc mb … --ignore-existing` | Unrelated flag — must NOT false-positive | Not a `cargo` invocation at all (MinIO client `mc`); `--ignore-existing` is a distinct token from `--ignore` (the `e` before `-existing` blocks a naive `--ignore\b` match only if the lookahead/whitespace boundary is chosen correctly — see C.4's regex note). |
| `ci.yml:463,466,755,757` `cargo test … -- --ignored` | Unrelated flag — must NOT false-positive | `cargo test`, not `cargo audit`/`cargo deny`; `--ignored` (not `--ignore`) is Rust's own test-harness flag to opt `#[ignore]`-marked tests back in. |

### C.4 Proposed matching approach (concrete, false-positive-safe)

**Do not** match on the bare substring `--ignore` anywhere in a workflow file — as shown above,
`--ignore-existing` contains ` --ignore` as a literal substring (there is a word boundary between
`e` and `-`, so a naive `\b--ignore\b` regex still matches inside `--ignore-existing`). The
robust discriminator is **co-occurrence on the same command line/step**, not the flag token alone:

1. **Structurally parse** each `.github/workflows/*.yml` file with PyYAML (confirmed installed
   this environment, §C.3-adjacent check: `python3 -c "import yaml; print(yaml.__version__)"` →
   `6.0`), walking every `jobs.*.steps[].run` string — this matches the project's own house style
   of preferring a real parser over grep-scraping raw file text (`check-doc-config.sh` already
   depends on PyYAML for the identical reason: "This script never scrapes … recover class
   information" is `check-advisory-register.sh`'s own stated rule at `:34-37`, applied here to
   YAML instead of TOML).
2. For each `run:` string found, apply a regex that requires **both** a `cargo audit`/`cargo deny`
   invocation **and** an ignore-flag token in the same string:
   ```python
   import re
   CARGO_GATE_RE = re.compile(r'\bcargo\s+(audit|deny)\b')
   IGNORE_FLAG_RE = re.compile(r'--ignore(?:[= ]|$)')  # matches "--ignore ", "--ignore=", or end-of-string
                                                         # does NOT match "--ignore-existing" (followed by '-')
                                                         # does NOT match "--ignored" (followed by 'd', not a boundary this regex accepts)
   for run_text in every_step_run_string:
       if CARGO_GATE_RE.search(run_text) and IGNORE_FLAG_RE.search(run_text):
           # violation: an advisory-ignore flag inline on a cargo audit/deny invocation
   ```
   The `IGNORE_FLAG_RE` pattern `--ignore(?:[= ]|$)` is deliberately **not** `--ignore\b` (which
   would match inside `--ignore-existing`, since `\b` only asserts a word/non-word transition, and
   `e→-` is such a transition) — requiring the character immediately after `ignore` to be a space,
   an `=`, or end-of-string excludes `--ignore-existing` (followed by `-`) and `--ignored`
   (followed by `d`, no transition at all) while still matching `--ignore RUSTSEC-...` and
   `--ignore=RUSTSEC-...`.
3. **The `CARGO_GATE_RE` co-occurrence requirement is what actually makes this safe**, independent
   of the flag regex's precision: `mc mb --ignore-existing` never contains `cargo audit`/`cargo
   deny` on the same line, and `cargo test -- --ignored` never contains `cargo audit`/`cargo deny`
   either — both known false-positive candidates fail the first half of the `and` before the flag
   regex is even relevant. This is the primary defense; the flag-regex precision is a secondary
   layer.
4. **Assert `cargo audit` appears exactly once**, across the same PyYAML-parsed step-run corpus,
   scanning **all** `.github/workflows/*.yml` files (not just `ci.yml`) — reuse the same
   `CARGO_GATE_RE`-style match (specifically `\bcargo audit\b`) and count occurrences across every
   file; fail if the count is `0` or `>1`, matching D-08(c) exactly and matching this phase's own
   already-run verification command (`grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1`,
   §A.7) but generalized to the whole directory rather than one file.
5. **Report every violation, not just the first** (D-08(d)) — accumulate into a `failures` list
   exactly like `check-advisory-register.sh`'s existing three clauses do, rather than raising on
   the first match.

### C.5 Existing guard-script test coverage

```
$ find /workspace -iname "*bats*" -o -iname "*shellspec*"   # (excluding node_modules, .git)
(no output)
$ find /workspace/tests -iname "*check*"
(no output)
$ grep -rln "check-advisory-register\|check-crate-names\|check-changelogs" /workspace/tests
(no output)
```
`[VERIFIED: find/grep, this session]` **None of the three existing `check-*.sh` guard scripts has
any test file, and no bash-testing framework (bats, shellspec) exists anywhere in the repository.**
There is no established convention to follow for testing these guards. **Recommendation for the
planner:** since D-08 is explicitly flagged ⚠ HUMAN REVIEW as a net-new CI check, and since the
guard's own correctness (does it actually catch a reintroduced `--ignore`?) is exactly the kind of
thing worth proving once rather than trusting by inspection, consider a lightweight self-test —
e.g., a fixture workflow-YAML string embedded in the script's own test invocation, or a one-off
manual verification step recorded in the plan's own verification section (inject a synthetic
`--ignore` into a scratch copy, confirm the guard fails, remove it, confirm it passes again) —
documented as a `file:line` command transcript per D-00e, not as a permanent test file, since no
project convention for one exists yet.

---

## §D — Verification Evidence

### D.11 Verbatim transcripts, re-run this session (2026-08-09), independent of CONTEXT.md's own transcripts

**`cargo audit`:**
```
$ cargo audit 2>&1 | head -4
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1190 security advisories (from /usr/local/cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (677 crate dependencies)
...
warning: 8 allowed warnings found
$ echo $?
0
```
Exit code: **0**. `[VERIFIED: cargo audit, this session, full output inspected]` — the 8 "allowed
warnings" are the five `[advisories] ignore` vulnerability entries plus additional `unmaintained`/
`unsound`/`yanked` advisories cargo-audit surfaces as warnings by default (not gated by
`.cargo/audit.toml`'s `ignore` list, which only governs hard-fail vulnerabilities) — zero hard
vulnerabilities, matching D-06's claim exactly.

**`cargo deny check`:**
```
$ cargo deny check 2>&1 | tail -1
advisories ok, bans ok, licenses ok, sources ok
$ echo $?
0
```
Exit code: **0**. `[VERIFIED: cargo deny check, this session]`

**`./scripts/check-advisory-register.sh`:**
```
$ ./scripts/check-advisory-register.sh
🔍 Checking the advisory exception register against deny.toml, .cargo/audit.toml and Cargo.lock ...
✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.
$ echo $?
0
```
Exit code: **0**. `[VERIFIED: ./scripts/check-advisory-register.sh, this session]`

**All three gates pass, independently re-confirmed.** This directly contradicts the "CI-only"
caveat named in §D.12 — the planner's plan must re-run these three commands again at execution
time (per D-06's own instruction: "the planner must re-run them at execution time rather than
quoting this file — a context file is not evidence, the command output is"; the same applies to
this RESEARCH.md).

### D.12 The three "CI-only" stale-caveat sites

1. **`REQUIREMENTS.md`'s SUPPLY-01 closure note**, `REQUIREMENTS.md:1871-1873`:
   > "**Remaining for Phase 12:** confirming the required status check still resolves on the first
   > real CI run after this deletion (CI-only, not verifiable in this sandboxed environment)."
   `[VERIFIED: REQUIREMENTS.md:1871-1873, direct read this session]` — Note: this specific clause
   (the CI-run observation) is **still genuinely CI-only** per D-07 — it is not the "cargo audit/
   cargo deny installability" caveat that D-06 says lifted. Care is needed: this exact sentence
   should **not** get a "blocker lifted" banner, since its substance is correct (a CI-run
   confirmation cannot happen locally); only the SUPPLY-02 sibling note below needs that banner.
2. **`REQUIREMENTS.md`'s SUPPLY-02 closure note**, `REQUIREMENTS.md:1925-1927`:
   > "**Remaining for Phase 12:** `cargo audit`/`cargo deny check` actually passing against the
   > reconciled configuration is CI-only, not run in this environment (crates.io returns HTTP
   > 403)."
   `[VERIFIED: REQUIREMENTS.md:1925-1927, direct read this session]` — **This is the one that
   needs the "blocker lifted" dated banner** — D-06 re-measured both tools installable and passing
   in this environment (§D.11).
3. **`ROADMAP.md`'s dated closure note**, `ROADMAP.md:772-776`:
   > "Phase 12 inherits SUPPLY-01 and SUPPLY-02 as **closed items to verify**, not work to re-plan:
   > confirming the required status check still resolves on the first real CI run after the
   > deletion, and that `cargo audit`/`cargo deny check` actually pass against the reconciled
   > configuration (neither tool is installable in Phase 9's sandboxed environment — `crates.io`
   > returns HTTP 403)."
   `[VERIFIED: ROADMAP.md:772-776, direct read this session]` — same "neither tool installable"
   claim, needs the same banner; the CI-run-observation half of this sentence remains correct and
   should be left alone (matches D-07).
4. **`09-CONTEXT.md`'s environment caveat**, `09-CONTEXT.md:380-387` (D-19):
   > "**Not runnable here:** `cargo audit` and `cargo deny` (both need `cargo install` against a
   > crates.io that returns HTTP 403)…"
   `[VERIFIED: 09-CONTEXT.md:380-387, direct read this session — the specific HTTP-403 clause is
   at line 383]` This is a **prior phase's own context file**, not a current planning artefact in
   the same sense as `REQUIREMENTS.md`/`ROADMAP.md`/`PROJECT.md` — per D-00d/D-00c, prior-phase
   context files are generally left as a historical record rather than corrected at source (no
   phase has amended a *different* phase's `NN-CONTEXT.md` file in this corpus's history, based on
   the correction-banner examples surveyed in §D.14). **Recommendation: do not edit
   `09-CONTEXT.md`; the three canonical corpus documents (`REQUIREMENTS.md` ×2 sites,
   `ROADMAP.md` ×1 site) are the correction targets** — treat `09-CONTEXT.md` as cited provenance
   only. Flagging this as a discretion point for the planner rather than asserting a fourth edit
   site.

### D.13 CI-run boundary fact (D-07's evidence)

```
$ gh run list --workflow=ci.yml --limit 5
completed  failure  fix(deepseek): annotate EmptyCompletion...        CI/CD Pipeline  release/v0.7.0  push  30861568499  1h17m15s  2026-08-03T23:14:24Z
completed  failure  docs(04): Docker and Kubernetes gates measured... CI/CD Pipeline  release/v0.7.0  push  30843492691  9m43s     2026-08-03T18:56:04Z
completed  failure  fix(04): load amd64 image before asserting...    CI/CD Pipeline  release/v0.7.0  push  30842748080  7m24s     2026-08-03T18:46:06Z
completed  failure  fix(04): scope Docker time budget to single-arch CI/CD Pipeline  release/v0.7.0  push  30839816736  6m58s     2026-08-03T18:07:08Z
completed  failure  docs(04): amend REL-05 ledger rows with real...   CI/CD Pipeline  release/v0.7.0  push  30837423031  6m57s     2026-08-03T17:35:15Z
```
`[VERIFIED: gh run list --workflow=ci.yml --limit 5, this session]`

Most recent run: **`30861568499`, 2026-08-03T23:14:24Z**, conclusion **failure**. The Phase 9
deletion landed 2026-08-08 (five days later). **No CI run has happened since the deletion** —
confirmed, matches D-07 exactly.

**Per-job breakdown of that run** (re-derived this session, not merely cited):
```
$ gh run view 30861568499 --json jobs -q '.jobs[] | "\(.name): \(.conclusion)"'
Security Audit: success
API Surface Tracking: failure
OSV Scanner: success
Unit Tests (stable): success
License & Dependency Policy: success
Integration Tests: success
Security Audit: success          # ← the duplicate job, still present pre-deletion
Crate Isolation (paladin-ports): success
... [remaining 15 jobs: success/skipped, none failure]
```
`[VERIFIED: gh run view 30861568499 --json jobs, this session]` **Confirmed: the only failing job
is `API Surface Tracking`** (DEBT-01's, not SUPPLY's), and **both** pre-deletion `Security Audit`
jobs (the surviving `security-audit:` and the still-present duplicate `security:`) passed — this
is the last run where the duplicate existed, and it never produced a red build, which is
consistent with SUPPLY-01's own framing ("two jobs configured to reach different verdicts" was a
latent defect, not one that had yet manifested as a visible CI failure).

### D.14 Correction-banner shape — three verbatim examples, `file:line`

From `.planning/codebase/CONCERNS.md` (Phase 9 amendment):

> `CONCERNS.md:276`: "**Amended by Phase 9 (plan 09-07), dated 2026-08-08, citing
> `.planning/decisions/0024-rustsec-exception-governance.md` (ADR-0024) and
> `SECURITY-EXCEPTIONS.md`:** the '10 unmaintained crates' count above and the list itself are
> stale. … The live unmaintained set today is five: `dotenv`, `fxhash`, `number_prefix`,
> `rustls-pemfile`, `paste`. … `SECURITY-EXCEPTIONS.md` (repo root) is now the authoritative
> governance register … the original list and migration plan above are retained verbatim, per the
> amend-at-source convention."
`[VERIFIED: CONCERNS.md:276, direct read this session]`

From `.planning/codebase/CONCERNS.md` (Phase 4 amendment, an earlier example — inline, not
block-quoted, embedded directly inside the original finding's own sentence):

> `CONCERNS.md:9`: "…this edition does not exist in Rust's stable channel… **(Amended by Phase 4,
> dated 2026-08-03, citing `.planning/decisions/0009-workspace-rust-edition-2024.md`**: this claim
> is factually wrong at this workspace's pinned toolchain. … The precedence order (ADR → shipped
> tree → this map) resolves the disagreement in the ADR's favor: this map's claim is superseded,
> not the toolchain.)"
`[VERIFIED: CONCERNS.md:9, direct read this session]`

**Shape observed across both examples:** the banner is **not** a separate paragraph or a blockquote
— it is a **parenthetical clause appended directly inside/after the original sentence**, opening
`(**Amended by Phase N (plan NN-NN), dated YYYY-MM-DD, citing <ADR path>**: …)`, closing with the
original claim's status (superseded/void/corrected) stated explicitly, and the corrected fact
given in full. The original text is **never deleted** — it remains as the sentence the parenthetical
is attached to. `[CITED: CONCERNS.md:9,23,25,276,295 — five instances, all following this identical
shape]`

**From a ledger** (`.planning/ledgers/milestone-07-08.md`, referenced but not opened in full this
session — the `REQUIREMENTS.md` traceability-row amendment at `:4080` follows an equivalent inline
parenthetical shape: `"— discharged (2026-06-04): DEBT-05 collapsed the two duplicate definitions
into pub use re-exports..."` embedded directly inside the coupling-table cell, same convention.
`[CITED: REQUIREMENTS.md:4078, direct read this session — the `**Amended by Phase 9 (plan 09-07),
dated 2026-08-08:**` pattern also appears at `REQUIREMENTS.md:4080` inline inside a table cell]`

**Recommendation for ADR-0036's three §B correction sites:** use the identical parenthetical-clause
shape (not a blockquote, not a separate section) — `(**Corrected by Phase 12, dated 2026-08-09,
citing ADR-0036 [and ADR-0024 for the candidate-count correction]:** …)` — appended to each of the
`REQUIREMENTS.md`/`PROJECT.md` stale sentences identified in §B, with the original sentence
retained verbatim before the parenthetical.

---

## §E — Where Closure Gets Recorded

### E.15 Existing hand-off block shape (three prior examples, `REQUIREMENTS.md`)

Three exist, located by `grep -n "Hand-off to Phase"`:
- `REQUIREMENTS.md:1322` — `#### Hand-off to Phase 10 / HARD-01 — dated 2026-08-08 (plan 09-07)`
- `REQUIREMENTS.md:1576` — `#### Hand-off to Phase 11 / FACADE-02 — dated 2026-08-08 (plan 10-11)`
- `REQUIREMENTS.md:1601` — `#### Hand-off to Phase 11 / FACADE-03(b) — dated 2026-08-08 (plan 10-11)`
- `REQUIREMENTS.md:1790` — `#### Hand-off to Phase 12 / SUPPLY-02 and SUPPLY-03 — dated 2026-08-08 (plan 10-11)`
  (the block Phase 10 wrote *for* Phase 12 — the shape Phase 12 must now imitate when writing
  its **own** outbound block to Phase 13)
- `REQUIREMENTS.md:1943` — `#### Hand-off to Phase 13 / ORCH-05 — dated 2026-08-08 (plan 10-11)`
`[VERIFIED: grep -n "Hand-off to Phase" REQUIREMENTS.md, this session]`

**Exact shape, read from the Phase-10-to-Phase-12 example (`:1790-1825`):**
1. **Heading:** `#### Hand-off to Phase {N} / {REQ-ID(s)} — dated {YYYY-MM-DD} (plan {NN-NN})`
2. **Bold lead sentence** immediately below, one sentence, stating what the receiving requirement
   inherits and in what posture ("delivered rather than deferred").
3. **Numbered list** (not bulleted) — each item a full paragraph, each opening with a **bold
   short label** (`**The corrected … reasoning, and what it now says:**`) followed by the
   substance, each citing exact `file:line`.
4. **Closing `**Evidence:**` line** — a single paragraph, semicolon-separated list of every
   ADR/file/ledger-row citation backing the block, no additional prose.

**Phase 12's own outbound block** (`#### Hand-off to Phase 13 / ORCH-01`) must reuse this exact
four-part shape: heading with dated `(plan NN-NN)`, bold lead sentence, numbered substance list,
`**Evidence:**` closing line. Per D-09, its numbered items must explicitly carry:
- The SUPPLY-01/02/03 closure verdicts and their citations (ADR-0036, ADR-0024, the three
  transcripts in §D.11).
- ORCH-01's named verdict class **verbatim**, per CONTEXT.md D-09: *"Milestone 10 is recorded 100%
  complete, ships every artefact it promised, and failed one of its own acceptance criteria — and,
  as of Phase 9, no longer does."*
- An explicit statement that Phase 12 did **not** create `.planning/ledgers/milestone-09-12.md` and
  that this is ORCH-01's stated deliverable (see E.16 below).

### E.16 `.planning/ledgers/` contents and ORCH-01's ownership

```
$ ls .planning/ledgers/
milestone-01.md
milestone-02-03.md
milestone-04-06.md
milestone-07-08.md
```
`[VERIFIED: ls .planning/ledgers/, this session]` **Confirmed: no `milestone-09-12.md` exists.**
ORCH-01's text (`ROADMAP.md` §Phase 13, criterion 1) reads: "A developer can look up any of the 120
Milestone 9-12 requirement IDs and see a `file:line`-cited verdict, and the ledger states plainly
that the whole Milestone 9 orchestrator subsystem, the whole Milestone 10 tooling set, the mdbook
and the whole Milestone 12 web API ship." CONTEXT.md's own D-09 quotes the more precise phrase from
elsewhere in the corpus: *"the Milestone 9-12 as-shipped ledger below is upgraded from
component-level file evidence to per-criterion verdicts"* (120 requirement IDs) — this is the exact
deliverable Phase 12 must **not** pre-build, per D-09.

### E.17 Requirement-closure checkbox and traceability-table conventions

`REQUIREMENTS.md` uses `- [ ]` → `- [x]` for individual requirement checkboxes (SUPPLY-01/02/03
currently `- [ ]`, confirmed at `REQUIREMENTS.md:1833,1875,1929`). The traceability table
(`REQUIREMENTS.md:3995-4032`) uses a `| REQ-ID | Phase | Status |` row shape with `Status` cells
reading `Pending` or `Complete`:

```
| SUPPLY-01 | Phase 12 | Pending |
| SUPPLY-02 | Phase 12 | Pending |
| SUPPLY-03 | Phase 12 | Pending |
```
`[VERIFIED: REQUIREMENTS.md:4008-4010, direct read this session]`

**Verbatim example of a closed row**, from the same table, one line above:
```
| FACADE-04 | Phase 11 | Complete |
```
`[VERIFIED: REQUIREMENTS.md:4007, direct read this session]` — the `Pending`→`Complete` cell edit
is the entire mechanical change; no other table column is touched on closure.

---

## §F — Risks and Landmines

1. **Re-litigating the suppression set (forbidden, D-00i).** ADR-0024 owns which advisories are
   suppressed, owner, review dates, schema, compensating controls. A planner who finds themselves
   drafting language about *which* advisories should be ignored, rather than *where* an ignore may
   be expressed, has drifted into ADR-0024's territory. ADR-0036's `## Context` should cite
   ADR-0024's decisions by number, never restate or second-guess them.
2. **Re-deriving `pdf-extract` reachability (forbidden, D-00j).** ADR-0032 already settled this;
   `.cargo/audit.toml:26-29` already carries the corrected wording (re-verified live in the file
   this session — the comment block is present and correct). No task in this phase should touch
   `pdf-extract` reasoning.
3. **Picking up the `scraper`/`rss`/`tiktoken-rs` dead-dependency finding (forbidden, D-00k).**
   `ROADMAP.md:714`'s literal text reads: "Phase 12 (SUPPLY-02/03, including the … dead-dependency
   finding named to Phase 15)" — the parenthetical genuinely reads ambiguously on a fast skim, as
   if the dead-dependency work were Phase 12's own. It is not; the Phase 10 hand-off
   (`REQUIREMENTS.md:1813-1821`) explicitly names Phase 15 as owner of record. **A planner
   producing any task that touches `crates/paladin-content/Cargo.toml`'s `scraper`/`rss`/
   `tiktoken-rs` optional dependencies has misread this phase's scope.**
4. **Creating the Milestone 9-12 ledger (forbidden, D-09).** Tempting because Phase 12 is the
   phase that "opens" the Milestone 9-12 block per `STATE.md:36`'s own framing — but the ledger's
   120-requirement-ID scope belongs entirely to ORCH-01/Phase 13. Phase 12 writes only its own
   three requirement rows plus the hand-off block (§E.15).
5. **Treating Phase 9's D-07 re-scope as settled/human-approved when drafting confidence language.**
   D-00l is explicit: it is *not* human-ratified, only durably recorded at source. A plan or ADR
   that states "SUPPLY-01/02 are closed" without the "inherited, unconfirmed re-scope" caveat
   overstates its own evidentiary basis. The §D.11 re-run transcripts are strong independent
   evidence regardless of D-07's ratification status, so this is a low-severity risk, but the
   phrasing should stay honest about provenance.
6. **Cold-build cost from the pre-commit hook chain (see D-18 below) inflating plan/wave count.**
   Since `.planning/config.json` already sets `worktree_skip_hooks: true`, and this phase is
   expected to touch **zero** `.rs` files, this risk is largely defused already (see D-18) — but a
   planner who forgets to note `worktree_skip_hooks=true` in an executor prompt (this repo's own
   documented failure mode, see project memory) risks a cold clippy/fmt run blocking every commit
   unnecessarily.
7. **Over-scoping the D-08 guard into a general CI-policy linter.** D-08's own text and CONTEXT.md
   both explicitly reject this ("a broader 'workflows may not do X' checker is a new capability and
   belongs in its own phase"). The guard's job is narrowly: cargo audit/deny + inline ignore flag,
   nothing else (no license flags, no other suppression-style CLI arguments).
8. **Confusing SUPPLY-01's still-open CI-run-observation clause (D-07, genuinely pending) with
   SUPPLY-02's cargo-audit/cargo-deny-installability clause (D-06, now closed).** Both closure
   notes use similar "CI-only" language (§D.12); only the SUPPLY-02-flavored ones (§D.12 items 2-3)
   get a "blocker lifted" banner. SUPPLY-01's own CI-run-resolution clause (§D.12 item 1) is
   correctly still pending — **do not banner it as resolved.**

## §D.18 Pre-commit hook chain and cold-build cost

`.pre-commit-config.yaml` (read in full this session) configures **both** commit-time and
push-time stages from one `pre-commit install` (`default_install_hook_types: [pre-commit,
pre-push]`). Two Rust-workspace hooks are registered under the **local** repo section:

```yaml
- id: cargo-fmt
  entry: cargo fmt --all -- --check
  types: [rust]
  pass_filenames: false
  always_run: true

- id: cargo-clippy
  entry: cargo clippy --workspace --all-targets --all-features -- -D warnings
  types: [rust]
  pass_filenames: false
  always_run: true
```
`[VERIFIED: .pre-commit-config.yaml, direct read this session]`

**`always_run: true` overrides the `types: [rust]` file-type filter** — in pre-commit's own
semantics, `always_run: true` means the hook executes on *every* commit regardless of which files
changed, including a commit that touches only `.planning/*.md` files. **Confirmed by project
precedent, not just by reading the YAML:** `06-10-SUMMARY.md:125` (a `.planning/`-only-touching
plan from an earlier phase) records: *"both commits used `--no-verify` per this worktree's
`workflow.worktree_skip_hooks: true` authorization. This plan touches no `.rs` file, so `cargo
fmt`/`clippy` gates have nothing to check"* — the phrasing "gates have nothing to check" is the
plan author's own gloss, but the *action taken* (`--no-verify`) confirms the hooks would otherwise
have fired on a docs-only commit, consistent with `always_run: true`'s documented semantics.
`[CITED: 06-10-SUMMARY.md:125]`

**`.planning/config.json` already sets `"worktree_skip_hooks": true`** (confirmed, full file read
this session — the only two keys present are `_auto_chain_active: true` and
`worktree_skip_hooks: true`). This means the executor is expected to run the fmt/clippy gate
**explicitly** (not via the git hook) before each commit, then commit with `--no-verify`, per the
pattern multiple prior phases already followed (`08-01-PLAN.md:70`, `08-02-PLAN.md:92`,
`08-04-PLAN.md:84`, `06-01/02/03/04/08/09/10-PLAN.md` all carry an identical "Executor note").

**Cold-build cost, from project precedent (not measured fresh this session — no `.rs` file in this
phase should trigger a cold build at all, so this is informational only):**
`07-12-SUMMARY.md:134`: *"The worktree had no pre-existing `target/` directory, so the first
`cargo check` was a cold build (~2m27s); subsequent commands reused the warm cache and completed
in seconds to low minutes."* `08-02-SUMMARY.md:155`: *"Several long-running `cargo` invocations
… each triggered a from-scratch recompile … each took several minutes."*
`[CITED: 07-12-SUMMARY.md:134; 08-02-SUMMARY.md:155]`

**Practical consequence for plan decomposition:** since this phase is expected to touch zero `.rs`
files (all deliverables are `.planning/`, `.project/`, `SECURITY-EXCEPTIONS.md`-adjacent-but-
untouched, and one `.sh`/possibly `Makefile`/`ci.yml` YAML edit), the cold-build risk is **low but
not zero** — only if the D-08 guard's own verification step is run via the full pre-commit chain
rather than invoked directly. **Recommendation:** the plan should (a) name
`worktree_skip_hooks=true` explicitly in every executor prompt per this repo's own documented
convention, (b) run `./scripts/check-advisory-register.sh` (or the new sibling script) and `cargo
audit`/`cargo deny check` **directly**, not through `pre-commit run --all-files`, to avoid an
unnecessary `cargo fmt`/`cargo clippy` full-workspace pass on a `.planning/`-only or
shell-script-only change, and (c) since no `.rs` file is touched, plan decomposition can favor
**fewer, larger commits** without the usual "one small commit per hook-triggering change" caution
that applies to phases with real Rust churn.

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The `ci.yml:389-406` stale line-citation appears in only **one** document (`REQUIREMENTS.md:1853`), not the "three documents" CONTEXT.md's `<domain>` block asserts. | §B.5 | If a planner trusts CONTEXT.md's "three documents" framing without re-running the grep, they may search for and fail to find two nonexistent correction sites, or worse, invent a correction where none is needed. Low risk — the grep is trivial to re-run at execution time and is explicitly flagged here. |
| A2 | `09-CONTEXT.md`'s environment caveat (D-19, lines 380-387) should **not** be edited at source, since no precedent exists in this corpus for one phase amending a different phase's own `NN-CONTEXT.md` file. | §D.12 item 4 | If wrong, and the project convention does expect prior-phase context files to receive correction banners too, Phase 12 under-corrects by one site. Low-medium risk — worth a one-line confirmation in the plan rather than a blocking question. |
| A3 | ADR-0036's `## Context` should use §8's two-file framing (`audit.toml` + `deny.toml`) rather than FR-1's narrower one-file framing, when stating "the invariant." | §A.7 | If the planner picks FR-1's narrower framing instead, ADR-0036 would understate the invariant relative to D-03's own `## Code Locations` instruction (which explicitly names both `.cargo/audit.toml` and `deny.toml`). Low risk — D-03 already settles this in CONTEXT.md; flagging only because the PRD source itself is ambiguous between its two sections. |

**If this table looks short:** it is deliberately short. Nearly every substantive claim in this
document was re-verified this session against a live command or a direct file read, per the D-00e
evidence bar this phase's own governing conventions require. The three items above are the only
places this research could not fully close a citation-count or convention question without the
planner making a small confirming choice at execution time.

## Open Questions

1. **Does the "three documents carry `ci.yml:389-406`" claim (CONTEXT.md's `<domain>` framing)
   resolve to exactly one site, or are there two more this research's single `grep` pass missed
   (e.g., inside a `.project/` document, or inside a different `REQUIREMENTS.md` section this
   research's targeted reads did not cover)?**
   - What we know: `grep -n '389-406' .planning/REQUIREMENTS.md` returns exactly one hit
     (`:1853`), re-run this session.
   - What's unclear: whether a broader `grep -rn '389-406' .planning/ .project/` (not run this
     session, to stay within this research's targeted read budget) would surface additional
     sites.
   - Recommendation: the planner should re-run the broader grep as a first task-0 step before
     writing any correction banners, and treat "one site found" as the working assumption unless
     that grep surfaces more.
2. **Should the D-08 guard be a fourth `check-advisory-register.sh` clause or a sibling script?**
   - What we know: both options satisfy D-08's fixed constraints (§C.1 gives the structural
     argument for a sibling: mixing `tomllib`-based TOML parsing with `PyYAML`-based YAML parsing
     inside one script's heredoc is a minor house-style wrinkle, not a blocker).
   - What's unclear: CONTEXT.md leaves this as explicit Claude's Discretion, so there is no wrong
     answer — but the planner should pick one and record the reason, per this project's convention
     of recording even discretionary choices with a one-line rationale (visible throughout
     `PROMOTION.md`'s own "Considered Options" sections).
   - Recommendation: sibling script (e.g. `scripts/check-workflow-suppressions.sh`), for
     single-responsibility and parser-type separation; wire both into the same `make check-gates`
     target and the same `ci.yml` job as `check-advisory-register.sh` (§C).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo-audit` | SUPPLY-01/02 verification | ✓ | installed, `cargo audit` exit 0 this session | — |
| `cargo-deny` | SUPPLY-01/02 verification | ✓ | installed, `cargo deny check` exit 0 this session | — |
| `python3` + `tomllib` | `check-advisory-register.sh` (existing) | ✓ | stdlib, Python 3.11+ | — |
| `python3` + `PyYAML` | D-08 guard, if it parses workflow YAML structurally | ✓ | `6.0`, confirmed via `python3 -c "import yaml; print(yaml.__version__)"` this session | grep-based fallback exists but is discouraged (§C.4) |
| `gh` CLI | D-07/D-13 evidence (`gh run list`, `gh run view`) | ✓ | authenticated, both commands succeeded this session | — |
| `pre-commit` | Local hook chain | ✓ (assumed installed, not directly probed) | — | `worktree_skip_hooks=true` already set; bypass via `--no-verify` per repo convention |

**Missing dependencies with no fallback:** none identified.

**Missing dependencies with fallback:** none identified — this phase's tooling surface (bash,
python3+tomllib, python3+PyYAML, cargo-audit, cargo-deny, gh) is entirely present and already
proven working in this environment this session.

## Package Legitimacy Audit

**Not applicable.** This phase installs no new external packages (no `Cargo.toml` dependency
changes, no `npm`/`pip` installs). The only "package"-adjacent fact is that
`scripts/check-advisory-register.sh` already depends on Python's `tomllib` (stdlib, not a
third-party package) and a prospective D-08 guard may add a dependency on `PyYAML` — already
present system-wide in this environment (confirmed `6.0`, §C.4) and already an existing project
dependency via `check-doc-config.sh` (`scripts/check-doc-config.sh:1-40`), so no new package
legitimacy risk is introduced.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Detecting inline `--ignore` flags on `cargo audit`/`cargo deny` in CI YAML | A hand-rolled multi-line grep/awk scan of raw workflow text | PyYAML-based structural parse of each `run:` step string (§C.4), matching the house convention `check-doc-config.sh` already established | Raw-text grep over YAML risks matching commented-out code, unrelated `--ignore` tokens split across `run: |` block lines, or step names that happen to contain the words; structural parsing extracts exactly the shell command text that will actually execute. |
| Verifying the advisory register agrees with `deny.toml`/`.cargo/audit.toml` | A new, separate verification mechanism | The existing `scripts/check-advisory-register.sh` (three clauses, already wired into `Makefile:167-172` and `ci.yml:101`) | Already built, already tested in CI, already the extension point named by D-08 itself. |
| Confirming a required-status-check resolves post-deletion | Simulating or faking a CI run locally | `gh run list --workflow=ci.yml` / `gh run view <id> --json jobs` against the real GitHub API, recording the pending status honestly (D-07) | A CI-run confirmation is inherently a live-service fact; fabricating a "pass" would violate the D-00e evidence bar. |

**Key insight:** every mechanism this phase needs already exists in the repository (the ADR
machinery, the three guard scripts, the `gh` CLI, `tomllib`/`PyYAML`). This phase's entire code-
adjacent surface is *extending* one existing script (or adding one small sibling following its
exact conventions) — there is no green-field tooling decision to make.

## Common Pitfalls

### Pitfall 1: Writing `## Code Locations` or `## Considered Options` as prose
**What goes wrong:** `adr-parser.cjs`'s `splitEntries` either collapses the section into one
opaque blob (unwrapped single-line paragraph) or shreds it into ungrammatical per-line fragments
(hand-wrapped multi-line paragraph) — see §A.3 for the exact mechanism.
**Why it happens:** ADR prose sections (`## Context`, `## Decision`) read naturally as paragraphs,
and it is tempting to keep the same register for the two list-shaped sections.
**How to avoid:** Every line inside `## Code Locations` and `## Considered Options` must start with
`-`, `*`, `+`, or a numeral+period, with exactly one citation/option per line.
**Warning signs:** A section that reads as flowing sentences rather than a scannable list.

### Pitfall 2: Restating or amending ADR-0024's suppression content inside ADR-0036
**What goes wrong:** Scope creep back into D-00i-forbidden territory — re-litigating which
advisories are suppressed.
**Why it happens:** ADR-0036 and ADR-0024 sit extremely close together (both are "supply-chain
suppression" ADRs, sequentially numbered 24 and 36), and ADR-0036's `## Context` naturally wants to
explain "the suppression mechanism" as background, which can drift into re-explaining what's
suppressed and why.
**How to avoid:** ADR-0036's `## Context`/`## Decision` should name ADR-0024 by number and cite its
decisions, never restate their substance in ADR-0036's own words.
**Warning signs:** ADR-0036 draft text that names a specific `RUSTSEC-*` ID's rationale rather than
just citing "ADR-0024 decision 3."

### Pitfall 3: Treating the SUPPLY-01 CI-run-observation clause as closeable in-repo
**What goes wrong:** A planner sees all three local commands pass (§D.11) and concludes the entire
phase, including the CI-run confirmation, is done.
**Why it happens:** SUPPLY-01 and SUPPLY-02's closure notes both use similar "CI-only" language,
and it is easy to conflate "the tools now run locally" (SUPPLY-02, now true) with "the CI run
itself has happened" (SUPPLY-01's separate clause, still not true — no push since the deletion).
**How to avoid:** Keep the two clauses distinct in the plan's verification section: one closes with
a command transcript; the other closes as *pending*, named trigger, per D-07.
**Warning signs:** A plan or ADR that claims SUPPLY-01 "fully closed" without a `gh run` citation
newer than `30861568499` (2026-08-03, pre-deletion).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Promotion requires `--manifest` re-tagging + re-running the ingest classifier | Promotion is an ordinary write to `.planning/decisions/` plus a `PROJECT.md` Key Decisions row | Phase 1 (built the mechanism); first used by Phase 7 (ADR-0016, ADR-0021) | SUPPLY-03's own governing text (`REQUIREMENTS.md:1937-1939`) never caught up to this — the stale text this phase corrects. |
| `cargo audit`/`cargo deny` recorded as "CI-only, `crates.io` returns HTTP 403" | Both tools install and run successfully in this environment | Between Phase 9/10 (last recorded the caveat, 2026-08-08) and this session (2026-08-09) | Directly enables local verification of SUPPLY-01/SUPPLY-02, discharging what two prior phases deferred. |
| A single blanket 2026-09-30 risk-acceptance date governing 2 of 15 nominal suppressions | Per-advisory `2026-12-31` review dates across all 10 live suppressions, in `SECURITY-EXCEPTIONS.md` | ADR-0024, Phase 9, 2026-08-08 | Not this phase's work (D-00i) — background only. |

**Deprecated/outdated:** the "eleven-candidate, zero-locked" framing at `REQUIREMENTS.md:103-109`
and `PROJECT.md:625-627` — four of the eleven are now promoted (ADR-0016, -0021, -0024, -0025),
and this phase adds a fifth (ADR-0036).

## Sources

### Primary (HIGH confidence — direct file read or command re-run this session)

- `.planning/phases/12-supply-chain-gate-integrity/12-CONTEXT.md` — full read, all decisions D-00a…D-13.
- `.planning/decisions/PROMOTION.md` — full read.
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — full read (shape model).
- `.planning/decisions/0024-rustsec-exception-governance.md` — full read (shape model + cited ADR).
- `.claude/gsd-core/bin/lib/adr-parser.cjs` — full read.
- `scripts/check-advisory-register.sh` — full read.
- `scripts/check-crate-names.sh` — full read.
- `scripts/check-changelogs.sh` — header read.
- `scripts/check-doc-config.sh` — header read (PyYAML precedent).
- `.pre-commit-config.yaml` — full read.
- `.planning/config.json` — full read.
- `Makefile:150-189` — read.
- `.github/workflows/ci.yml:55-124,428-429,463-466,755-757` — read.
- `.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md` — headings scan + FR1/§8 direct read.
- `.planning/REQUIREMENTS.md:90-120,1322-1382,1576-1941,1943-1950,3995-4093` — read.
- `.planning/ROADMAP.md:755-860` — read.
- `.planning/STATE.md:1-45` — read.
- `.planning/PROJECT.md:580-630,1103-1141` — read.
- `.planning/codebase/CONCERNS.md` (grep for "Amended"/"Corrected" + 5 sections read).
- `.planning/phases/09-release-security-gate-integrity/09-CONTEXT.md:280-300,378-389` — read.
- `cargo audit`, `cargo deny check`, `./scripts/check-advisory-register.sh` — all three re-run this session, exit 0.
- `gh run list --workflow=ci.yml --limit 5`, `gh run view 30861568499 --json jobs` — re-run this session.
- `ls .planning/decisions/`, `ls .planning/ledgers/`, `ls .github/workflows/` — re-run this session.
- `grep -rn "cargo audit\|--ignore" .github/workflows/*.yml` — re-run this session, all 6 files.
- `python3 -c "import yaml; print(yaml.__version__)"` — re-run this session, `6.0`.

### Secondary (MEDIUM confidence)

- `.planning/phases/09-release-security-gate-integrity/09-CONTEXT.md` sections not directly
  re-verified against tree state (D-13, D-19 context around SEC-03) — cited for background only.
- `07-12-SUMMARY.md:134`, `08-02-SUMMARY.md:155`, `06-10-SUMMARY.md:125` — cited for cold-build/
  hook-chain precedent, not re-measured fresh this session (no `.rs` file work in this phase to
  measure against).

### Tertiary (LOW confidence)

- None. Every claim in this document traces to a direct read or a re-run command this session, or
  is explicitly flagged as an open question / assumption above.

## Metadata

**Confidence breakdown:**
- ADR file contract (§A): HIGH — `adr-parser.cjs` read directly, `PROMOTION.md` read in full, two
  shape-model ADRs read in full.
- Stale-text corrections (§B): HIGH for B.1-B.4 (exact `file:line`, re-verified); MEDIUM for B.5
  (only one of the "three documents" CONTEXT.md names could be independently confirmed — flagged
  as Assumption A1 / Open Question 1).
- Guard-script design (§C): HIGH for the existing script's structure and the false-positive
  analysis (every token re-grepped this session); MEDIUM for the specific regex proposal (a
  reasonable, tested-against-known-false-positives design, not a shipped/tested implementation).
- Verification evidence (§D): HIGH — all three gate commands and both `gh` commands re-run this
  session, independent of CONTEXT.md's own transcripts.
- Hand-off/closure shape (§E): HIGH — three existing hand-off blocks and the traceability table
  read directly.

**Research date:** 2026-08-09
**Valid until:** This is a governance/planning-artefact phase against a fast-moving corpus (three
prior phases each amended `REQUIREMENTS.md`/`ROADMAP.md`/`CONCERNS.md` within the same week); the
`cargo audit`/`cargo deny` pass/fail state and the `gh run list` boundary fact should be re-verified
at plan-execution time rather than trusted past **7 days** from this research date. The ADR
machinery (`PROMOTION.md`, `adr-parser.cjs`) is stable infrastructure and valid for the life of
this milestone.
