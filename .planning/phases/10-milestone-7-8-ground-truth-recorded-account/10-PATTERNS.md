# Phase 10: Milestone 7-8 Ground Truth & Recorded Account - Pattern Map

**Mapped:** 2026-08-08
**Files analyzed:** ~18 (1 ledger, 6 ADRs, PROMOTION.md, REQUIREMENTS.md sections, ~8 `.project/`
annotations, 3 config edits)
**Analogs found:** 18 / 18 — this phase is entirely record-writing; every deliverable has a direct,
recent analog already in the corpus. There is no "no analog found" section.

**Framing note:** this is not a source-code phase. "Role/data flow" classification is replaced below
with the document-tier classification the phase's own RESEARCH.md uses (ledger / ADR / annotation /
config edit), since that is what actually determines which analog applies.

## File Classification

| New/Modified File | Tier | Closest Analog | Match Quality |
|---|---|---|---|
| `.planning/ledgers/milestone-07-08.md` (new) | ledger | `.planning/ledgers/milestone-04-06.md` | exact (named successor) |
| `.planning/decisions/0028-m8-reconciliation-authoritative.md` (new) | ADR, `conforms` | `0027-dockerfile-chef-planner-stage.md` (shape) + `0015-...` (multi-part Decision) | exact |
| `.planning/decisions/0029-version-trajectory-history.md` (new) | ADR, `conforms`, extensible | `0015-core-ports-dependency-allowlist.md` (has a `## Trajectory`-style forward-extension precedent in its Downstream Consumers) | role-match |
| `.planning/decisions/0030-milestone-7-self-numbering.md` (new) | ADR, `conforms` | `0010-milestone-3-epic-numbering.md`, `0014-milestone-4-6-tier-numbering.md` (the two it must cite) | exact |
| `.planning/decisions/0031-extracted-crate-dependency-rule.md` (new) | ADR, `conforms` | `0015-core-ports-dependency-allowlist.md` | exact (explicitly named as the structural model) |
| `.planning/decisions/0032-pdf-extraction-capability.md` (new) | ADR, `must change` | `0027-dockerfile-chef-planner-stage.md` | exact (must-change shape, deletion-only fix) |
| `.planning/decisions/0033-cargo-doc-warning-bar.md` (new) | ADR, `must change` | `0027-dockerfile-chef-planner-stage.md` | role-match |
| `.planning/decisions/PROMOTION.md` (modified) | index | itself, prior append entries (0024, 0022, 0021) | exact |
| `.planning/REQUIREMENTS.md:3121-3317` (modified → pointer) | pointer | Phase 7's reduction of the Milestone 4-6 section (see `milestone-04-06.md:1-7` describing the pointer it became) | exact |
| `.planning/REQUIREMENTS.md` hand-off blocks (new, x4) | hand-off block | `REQUIREMENTS.md:1320-1355` (Phase 9 → Phase 10 block, quoted below) | exact |
| `.project/.../facade-audit.md` (annotated) | full-supersession banner | Shape B, commit `74a05fe` (`09-05`) | exact |
| `.project/.../infrastructure-adapter-disposition.md` (annotated) | full-supersession banner | Shape B, commit `74a05fe` | exact |
| `.project/Milestone_7.../overview/....md` (annotated, title+prereqs) | full-supersession banner | Shape B, commit `74a05fe` | exact |
| `.project/.../Epic_3/...non-goal §5` (annotated, clause) | inline strike-and-correct | Shape A, commit `94814ff` (`08-05`) | exact |
| `.project/.../Epic_5/...FR-19` (annotated, clause) | inline strike-and-correct | Shape A | exact |
| `.project/.../Epic_1/prd-extract-infrastructure-crates.md` §6.1/§4.4.1/§4.4.6 (annotated, clauses) | inline strike-and-correct | Shape A | exact |
| `crates/paladin-content/Cargo.toml:18` (delete `pdf = []`) | config edit | ADR-0027's Dockerfile deletion-only diff pattern | role-match |
| `.cargo/audit.toml:26-29` (correct parenthetical) | config edit (comment only) | same deletion/correction discipline as above | role-match |
| `Makefile:432-433` (delete flag + echo) | config edit | same | role-match |

## Pattern Assignments

### `.planning/ledgers/milestone-07-08.md` (new ledger)

**Analog:** `.planning/ledgers/milestone-04-06.md` (456 lines) — this file's own head note names
`milestone-07-08.md` as its sibling, so structure is not discretionary.

**Head note skeleton to copy verbatim in shape** (`milestone-04-06.md:1-58`):
```markdown
# Milestone 7-8 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 7-8 as-shipped ledger` section (D-01).
That section becomes a pointer to this file. ...

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers ...

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` ...

**Manifest carve-out.** [Milestone 7-8 is structural like 4-6 — reuse this clause almost verbatim,
substituting the M7/M8-specific exercising CI jobs/build legs per D-03.]

**Path caveats.** [if any paths moved; otherwise state none apply]

**Workspace shape.** [D-06's paragraph: ten library crates + doc-examples + facade `paladin-ai`;
`paladin-herald` traces to the 2026-06-04 reconciliation, not a PRD — state this explicitly, it is
the one crate whose provenance differs from the other four.]

**Per-milestone checkbox corroboration (D-06 in 04-06's numbering).** [state Milestone 7 at 98.8%
and Milestone 8 at 99.1%, and whether each open item is corroborated or contradicted, per
`intel/task-completion-state.md` — but do not trust its counts uncritically per RESEARCH.md.]
```

**Verdict legend table — reuse `milestone-04-06.md`'s legend but map onto the seven-class run
CONTEXT.md's D-02 specifies** (`satisfied` · `present, unproven` · `genuinely outstanding` ·
`relocated` · `superseded by outcome` · `deferred with register` · `diverged`) rather than
`milestone-04-06.md`'s own six-word legend (which used `superseded by shipped code` and
`deferred with reason` — **note the wording difference**: this ledger's D-02 vocabulary is
`superseded by outcome` / `deferred with register`, not the 04-06 ledger's phrasing — do not copy
the legend text verbatim, only its shape, table columns and the "ROADMAP-criterion mapping" /
"Tie-break rule" callouts that follow it (`milestone-04-06.md:72-80`).

**Row order and amendment convention section** — copy verbatim in structure
(`milestone-04-06.md:82-91`): epic sections in REQUIREMENTS.md's own run-4 order, never re-sorted;
rows never inserted/deleted/reordered, only Verdict/Evidence cells replaced in place; state which
wave's plan owns writing vs. appending (per this phase's D-27: wave 1 writes head notes + row
stubs, waves 2 fan out appends by disjoint epic range).

**The `Superseded by outcome` summary table (D-02, mandatory, placement is Claude's Discretion)** —
no direct analog exists yet in prior ledgers (this is the first ledger with an unmissable-class
requirement), so this is new structure. Recommended shape, modeled on the existing per-row table
columns:
```markdown
## Superseded by outcome — do not plan these as written

| REQ-* ID | What the requirement says | What the tree does instead | Citation |
|---|---|---|---|
| ... | ... | ... | `file:line` |
```
Build this table from `intel/code-verification.md:365-381`'s **13** data rows (not 14 — D-05),
re-verify the count with `sed -n '365,381p' .planning/intel/code-verification.md | grep -c '^|.*|.*|.*|$'`
(expect 15 total lines including header/separator → 13 data rows) before transcribing.

**In-place amendment convention** — copy `milestone-01.md`'s dated-section pattern for any later
wave's corrections to wave 1's row stubs (`milestone-01.md:21-38`, "## Phase 2 amendments" heading
shape): a dated `## Phase 10 wave N amendments (2026-08-08)` section, prose explaining what was
re-derived vs. trusted, never a separate corrections file.

**Phase-9-closed rows (D-04)** — cite verbatim from `REQUIREMENTS.md:1324-1355`'s hand-off block
(quoted in full below under Shared Patterns) rather than re-deriving; the seven `REQ-*` IDs and
their ADR/commit citations are already assembled there.

---

### `.planning/decisions/0028-m8-reconciliation-authoritative.md` … `0033-cargo-doc-warning-bar.md` (six new ADRs)

**Analog:** `.planning/decisions/0027-dockerfile-chef-planner-stage.md` (structure/tone, most
recent) and `.planning/decisions/0015-core-ports-dependency-allowlist.md` (the multi-part Decision
shape ADR-0031 must imitate per explicit instruction in CONTEXT.md D-15).

**Required heading set, exact order, no frontmatter** (from `PROMOTION.md:75-96`):
```
## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```
`## Code Locations` and `## Considered Options` **must be bulleted lists, never prose paragraphs**
(`PROMOTION.md:87-90` — the parser only extracts structured entries from bullet/numbered lines).

**Status block pattern** (`0027-...md:1-7`):
```markdown
# ADR-0027: <title>

## Status

Accepted

**Date:** 2026-08-08
```

**Context pattern for a supersession/contested-position ADR** (`0027-...md:9-66`): state what the
`.project/` document asserts with its exact `file:line`, state what the tree shows with its exact
`file:line`, then a "sharper finding" paragraph if research went further than the surface reading,
ending with an explicit "what is measured vs. established" caveat when something (Docker, network)
could not be run in this environment — this is the exact shape ADR-0033 needs for the "cargo doc is
CI-configured but currently RED" finding (RESEARCH.md Pitfall 1).

**Decision pattern citing precedence explicitly** (`0027-...md:68-89`): open with "Under the D-00b
precedence order (ADR → shipped tree → ... → checkbox — an ADR that contradicts shipped code is an
instruction to change the code...)" then the one-sentence verdict, then the mechanical fix
description (subtraction only, no reordering) with before/after line ranges.

**Multi-part Decision pattern (for ADR-0031, per explicit CONTEXT.md instruction to copy
ADR-0015's shape)** (`0015-...md:38-76`): three lettered/numbered sub-decisions kept visually
separate — (i) the enforceable invariant stated independent of any specific fact list, (ii) the
measured current state accepted as baseline, (iii) any single item needing its own justification.
ADR-0031 should mirror this exactly: (i) *no extracted crate may depend on another extracted crate
or the facade in its default build; an optional non-default feature may declare such an edge only
where the facade opts in explicitly and the code is `cfg`-gated* (ii) `paladin-content`'s measured
`llm` feature-gate state as the accepted baseline (iii) — optional third part not required here
since there's no single-item outlier like `tokio`.

**Considered Options pattern** (`0027-...md:91-113`, `0015-...md:77-92`): each option gets
(accepted)/(rejected) tag plus a one-line reason; the rejected options should include the ones
CONTEXT.md's decisions explicitly named as alternatives (e.g. for ADR-0031: option (a) "never, with
the edge removed" vs (b) "never, except behind optional feature" — accepted).

**Code Locations pattern**: bullet list of exact `file:line` citations for both the corpus
assertion and the tree fact, plus upstream/PRD citations, e.g. (`0027-...md:115-144`):
```markdown
- `Dockerfile.chef:21-27` (post-edit) — ...
- `Milestone_7.../prd-....md:66-69` — FR-01, the superseded requirement.
```

**Code Conformance pattern** — exactly the word `conforms` or `must change` as a standalone line,
followed (if `must change`) by naming the executing plan/task (`0027-...md:152-159`):
```markdown
## Code Conformance

must change

Plan 09-03 task 1 is the executor: it performs the deletion at ..., and confirms ...
```

**Downstream Consumers pattern** — bulleted, names the specific phase/REQ-ID that inherits this
decision, exactly the shape ADR-0031 must use to name "Phase 11 / FACADE-02" per CONTEXT.md D-15,
and ADR-0028 must use to name FACADE-03 per D-09 (`0027-...md:161-171`, `0015-...md:122-129`):
```markdown
## Downstream Consumers

- **Phase 10 / HARD-01** — the ledger's `REQ-...` row currently reads "...", this ADR is the
  record HARD-01 cites when it upgrades that row...
```

**Supersession mechanism** (not needed for this phase's six ADRs, since none supersedes an
existing numbered ADR — but if any ever does): `PROMOTION.md:98-106` — superseded ADR's `## Status`
body becomes the bare word `Superseded` plus a prose line naming the successor; the successor
carries a `## Supersedes` line.

---

### `.planning/decisions/PROMOTION.md` (modified — advance next-free line)

**Analog:** the file's own prior append entries at `PROMOTION.md:49-70` (the 0024, 0022, 0021
dated notes).

**Pattern to copy exactly** — append six new rows to the Numbering index table
(`PROMOTION.md:21-49`), then update the "Next free ADR number" line and add a dated note in the
same voice as the existing ones:
```markdown
| 0028 | `m8-reconciliation-authoritative` | Milestone 8 reconciliation authoritative over Epic 1/3 audits (Phase 10) |
...
| 0033 | `cargo-doc-warning-bar` | `cargo doc` zero-warning bar, DEBT-03 discharge, doctest posture (Phase 10) |

**Next free ADR number: 0034**

*Dated note, 2026-08-08 (plan 10-NN):* the line advances by **six**, from 0028 to 0034. Phase 10
authored all six of ADR-0028 through ADR-0033 across its own plans — a phase whose executing phase
is also each ADR's owning phase, matching the precedent Phase 9's own note above established for
four ADRs in one phase. None of the six numbers was skipped or reused.
```

---

### `.planning/REQUIREMENTS.md:3121-3317` (reduced to pointer)

**Analog:** the fact that `milestone-04-06.md`'s own head note states its own supersession
relationship — i.e. read `milestone-04-06.md:1-7` and the corresponding REQUIREMENTS.md pointer it
produced (search for "Milestone 4-6 as-shipped ledger" in `REQUIREMENTS.md` for the exact pointer
prose Phase 7's plan wrote) and reuse that exact pointer shape for Milestone 7-8:
```markdown
## Milestone 7-8 as-shipped ledger

This section has been superseded by `.planning/ledgers/milestone-07-08.md` (Phase 10, D-01). See
that file for the full 86-row, `file:line`-cited verdict table. This section is retained as a
pointer only.
```

---

### `.planning/REQUIREMENTS.md` — four forward hand-off blocks (D-26)

**Analog:** `REQUIREMENTS.md:1320-1355` — Phase 9's hand-off block to this very phase. **Quote in
full as the exact shape to imitate** (already reproduced above in Read output; key structural
elements to copy):
```markdown
#### Hand-off to Phase 10 / HARD-01 — dated 2026-08-08 (plan 09-07)

**No Milestone 7-8 as-shipped ledger exists yet — HARD-01 (Phase 10) builds it, per D-20
(`09-CONTEXT.md`).** ... This block is the explicit, additional hand-off D-20 requires: when
HARD-01 builds the Milestone 7-8 ledger, it must record each of the following seven `REQ-*` rows
as **already closed by Phase 9**, citing this phase's ADRs and commits, rather than re-verifying or
re-planning them:

1. **`REQ-...`** — closed by .../ADR-.... <one paragraph of evidence>.
...

**Evidence for all seven:** <register file>; <ADR list>; commits <list> (plans ...). None of these
... rows requires further code work; HARD-01's task is to cite them, not re-open them.
```
Reuse this exact shape (heading level `####`, bold opening sentence naming which requirement
inherits what, numbered list of specific IDs/claims, closing "Evidence for all" paragraph) for each
of the four D-26 hand-offs: Phase 11/FACADE-02, Phase 11/FACADE-03(b), Phase 12/SUPPLY-02, Phase
13/ORCH-05.

---

### `.project/` annotation banners (~8 documents)

**Analog A — Shape B (full-section supersession), from commit `74a05fe` (Phase 9, plan `09-05`)**:
```markdown
> **AC 1 SUPERSEDED BY [ADR-0025](../../../.planning/decisions/0025-licence-posture.md) — 2026-08-08.**
> At a blocking checkpoint on 2026-08-08 the repository owner (`DF3NDR`) selected the dual licence
> expression recorded in this same Epic's `license-compatibility-decision-checklist.md` over the
> single `MIT` expression this criterion names. The root package and all ten library crates now
> declare `license = "MIT OR Apache-2.0"`. The original criterion text below is retained unmodified.
```
Use this shape for: `facade-audit.md`, `infrastructure-adapter-disposition.md`, the M7 overview's
title/prereqs (all three CONTEXT.md/RESEARCH.md flag as fully superseded by one ADR each).

**Analog B — Shape A (inline strike-and-correct), from commit `94814ff` (Phase 8, plan `08-05`)**:
```markdown
> **Correction (dated 2026-08-06, DEBT-01):** This document instructs a future implementer to
> write the public-API surface baseline to the pre-rename `project/` path in two places (FR-10 and
> §7 "Technical Considerations", both struck below) — a path that has not existed since commit
> `928c6d5` renamed `project/` to `.project/`. ... Original text is retained below with inline
> corrections — nothing is deleted.

...

10. ~~The system **must** regenerate the public API-surface baseline~~
    ~~(`./scripts/extract-public-api.sh project/current-exports.txt`) and add a `CHANGELOG.md`~~
    ...
    **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
    `.project/current-exports.txt` ... Confirmed via `ls -la ...` ...
```
Use this shape for clause-level fixes: M7 Epic 1 §6.1/§4.4.1/§4.4.6, M8 Epic 3 §5's non-goal
clause, M8 Epic 5 FR-19 — each gets `~~struck~~` original text plus a **Corrected (dated ..., HARD-NN)**
paragraph immediately after, never a deletion.

**Rule governing both shapes (D-00c, do not violate):** annotation, not rewriting. Original text is
always retained (struck-through for Shape A, left intact below the blockquote for Shape B), dated,
and points at the specific ADR or requirement ID that corrects it.

---

### `crates/paladin-content/Cargo.toml:18` — delete `pdf = []`

**Exact current text (verified this session, do not re-derive)**:
```toml
[features]
pdf          = []                        # ← line 18, delete per D-18
web-scraping = ["dep:scraper"]
rss          = ["dep:rss"]
news-api     = []
tiktoken     = ["dep:tiktoken-rs"]
llm          = ["dep:paladin-llm"]
```
Action: delete exactly the `pdf = []` line; leave `news-api = []` (the legitimate empty feature
comparator, per D-18) untouched. Record the change in `crates/paladin-content/CHANGELOG.md` per
D-18's accepted-cost note (a minor public-contract change: `--features pdf` begins failing where it
previously silently succeeded-and-did-nothing).

### `.cargo/audit.toml:26-29` — correct the RUSTSEC-2026-0187 parenthetical

**Exact current text (verified this session)**:
```
# RUSTSEC-2026-0187: stack overflow in lopdf via deeply nested PDF objects.
#   lopdf is transitive via `pdf-extract` (optional `content-processing`). The fix requires
#   `pdf-extract` >= 0.12 (a breaking jump that also pulls a fresh `ttf-parser` advisory);
#   deferred. Revisit when `pdf-extract` ships lopdf >= 0.42 without new advisories.
```
Per D-19, the mechanism named is wrong: `pdf-extract` is unconditional in `paladin-content`
(not gated by any feature); the actual optionality is one level up, at `paladin-content` being
`optional = true` in the root facade (`Cargo.toml:59`). Correct only the parenthetical's wording
(e.g. `(paladin-content is optional in the facade; pdf-extract itself is unconditional within
paladin-content)`) — do not touch the RUSTSEC-2026-0187 suppression decision itself, which stands.

### `Makefile:432-433` — delete flag and stale echo

**Exact current text (verified this session)**:
```makefile
.PHONY: release-check
release-check: ## Check if ready for release
	@echo "$(CYAN)Checking release readiness...$(NC)"
	@$(MAKE) clean-code
	@$(MAKE) test
	@echo "$(CYAN)Running doc tests (excluding paladin-ports: doctests reference root crate not yet published)...$(NC)"
	@$(CARGO) test --workspace --doc --exclude paladin-ports
	@$(MAKE) audit
	@$(MAKE) build-release
	@echo "$(GREEN)✅ Release check passed!$(NC)"
```
Action per D-21: delete the `echo "...excluding paladin-ports..."` line and change
`@$(CARGO) test --workspace --doc --exclude paladin-ports` to `@$(CARGO) test --workspace --doc`
(matching `ci.yml:238`'s bare form and `Makefile:123`'s already-clean `test-doc` target).

## Shared Patterns

### D-00b precedence order — cite in every ADR's Decision section
**Source:** repeated verbatim across `0015-...md:70` and `0027-...md:70-71`.
```
ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
checkbox. An ADR that contradicts shipped code is an instruction to change the code.
```
Apply to: all six new ADRs' `## Decision` opening sentence.

### D-00e evidence bar — every closure claim needs an exact command or `file:line`
**Source:** `milestone-04-06.md:16-21`, restated identically for this ledger by D-03/D-25.
Apply to: every ledger row, every ADR `## Code Locations` entry, every `.project/` annotation's
"Confirmed via ..." clause.

### D-00c annotation convention — never rewrite, always retain + date + point at the correcting ADR/REQ
**Source:** both banner shapes above; `PROMOTION.md`'s own supersession mechanism section
(`:98-106`) for the analogous ADR-level rule.
Apply to: all ~8 `.project/` document edits.

### ADR heading set and ordering — non-negotiable, no frontmatter
**Source:** `PROMOTION.md:75-96`.
Apply to: ADR-0028 through ADR-0033, all six.

## Metadata

**Analog search scope:** `.planning/ledgers/`, `.planning/decisions/`, `.planning/REQUIREMENTS.md`,
`.project/Milestone_7-Production-Hardening/`, `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/`,
`crates/paladin-content/Cargo.toml`, `.cargo/audit.toml`, `Makefile`.
**Files scanned:** ~14 (3 ledgers, 3 ADRs read in full, PROMOTION.md, REQUIREMENTS.md hand-off +
pointer sections, 3 config files, both CONTEXT.md/RESEARCH.md sources).
**Pattern extraction date:** 2026-08-08.
