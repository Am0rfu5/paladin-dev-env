# Phase 2 — Spec-less Probe Fallback Record

> Orchestrator artifact, written by `/gsd-plan-phase 2`. Phase 2 has no `SPEC.md`, so both
> `## Edge Coverage` and `## Prohibitions` were absent (`EDGE_ABSENT=1`, `PROHIB_ABSENT=1`) and the
> spec-less probe fallback fired (`references/specless-probe-fallback.md`). This file records the
> deterministic edge-probe output that was passed to `gsd-planner`, so the plans' probe accounting is
> checkable against its source rather than against an untraceable claim.

- **Toggle:** `workflow.specless_probe_fallback` = `true` (default-ON)
- **Engine:** `gsd-core/bin/lib/edge-probe.cjs`
- **Input:** requirement text for GAP-01 … GAP-07, lifted from `REQUIREMENTS.md:195-234`
- **Date:** 2026-07-31

## Coverage totals (as emitted)

| Field | Value |
|-------|-------|
| applicable | 17 |
| resolved | 0 |
| unresolved | 17 |
| byVerification.explicit | 0 |
| byVerification.backstop | 0 |

All 17 rows were emitted `unresolved` — the engine surfaces the probes; `--auto` resolution into
`covered` / `backstop` / flagged-`unresolved` is the planner's job (§A of the reference).

## The 17 surfaced edges

| # | Requirement | Category | Probe |
|---|-------------|----------|-------|
| 1 | GAP-01 | adjacency | When two things are exactly equal or just touch, do they merge, collide, or separate? |
| 2 | GAP-01 | empty | What is the result for empty, single-element, or null input? |
| 3 | GAP-01 | ordering | When elements compare equal, is output order specified and stable? |
| 4 | GAP-02 | adjacency | When two things are exactly equal or just touch, do they merge, collide, or separate? |
| 5 | GAP-02 | empty | What is the result for empty, single-element, or null input? |
| 6 | GAP-02 | ordering | When elements compare equal, is output order specified and stable? |
| 7 | GAP-03 | empty | What is the result for empty, single-element, or null input? |
| 8 | GAP-03 | encoding | Whose definition of length/equality applies — bytes, code points, grapheme clusters, or normalized form? |
| 9 | GAP-04 | **unclassified** | unclassified — review manually |
| 10 | GAP-05 | adjacency | When two things are exactly equal or just touch, do they merge, collide, or separate? |
| 11 | GAP-05 | empty | What is the result for empty, single-element, or null input? |
| 12 | GAP-05 | ordering | When elements compare equal, is output order specified and stable? |
| 13 | GAP-06 | adjacency | When two things are exactly equal or just touch, do they merge, collide, or separate? |
| 14 | GAP-06 | empty | What is the result for empty, single-element, or null input? |
| 15 | GAP-06 | ordering | When elements compare equal, is output order specified and stable? |
| 16 | GAP-07 | boundary | What happens exactly at each min/max/threshold — and one step either side? |
| 17 | GAP-07 | precision | Where can precision loss, overflow, or rounding/tie-breaking occur — and what is the exact contract (e.g. half-up vs half-to-even, ceil/floor/truncate)? |

Row 9 is `unclassified`. Per #1110 it stays `unresolved` and is surfaced as a flagged planner
assumption — never auto-`backstop`ped, never silently dropped. It is recorded in `02-01-PLAN.md`
task 2 and proposed forward to Phase 3.

## No-silent-drop accounting

The contract is `(# probe-surfaced) <= (# authored into must_haves + # surfaced as flagged
assumptions)` — a surplus is fine (extra predicates), a shortfall is a silent drop.

| Source | Count |
|--------|-------|
| Probe-surfaced edges | 17 |
| Authored `covered` (plain `must_haves.truths` strings) | 4 — `02-02` (GAP-07 precision), `02-03` (GAP-07 boundary), `02-04` (GAP-03 empty), `02-04` & `02-10` (GAP-03 encoding, **re-authored** — see the dated amendment below) |
| Authored `backstop` (`{ statement, verification: backstop }` markers) | 15 — `02-01` ×9, `02-02` ×1, `02-08` ×3, `02-10` ×2 |
| Flagged assumptions (`unresolved`) | 1 — GAP-04 unclassified, in `02-01` |
| **Total accounted** | **20** |

20 ≥ 17: no silent drops. The surplus of three is a `backstop` marker in `02-02` that the planner's
own return summary omitted from its count (it reported 12 backstop, the plans contain 13). The
plan-checker flagged the discrepancy; the extra predicate is additive, so it is recorded here
rather than removed.

### Row 8 disposition amendment (2026-08-01, authority: plan 02-10)

**Original disposition:** `02-04` authored probe row 8 (GAP-03 encoding) as `covered` — a plain
`must_haves.truths` string asserting the Table Herald "renders a Paladin name containing
multi-byte UTF-8 without panicking."

**Why that was insufficient:** the string restated the claim rather than expressing a predicate
that could fail against the code. Its proving test, `test_table_herald_renders_multibyte_paladin_name`,
used a 21-byte name far below the 60-char default budget, so it never reached the truncation
branch. The defect shipped behind a green suite, was graded a blocker by `02-VERIFICATION.md`,
and was independently graded Critical by `02-REVIEW.md` CR-02.

**What replaced it:** `02-10` re-authors row 8 with an admissibility rule attached to the
predicate — an input proves the truncation branch only if the byte offset at `width - 3` is not
a char boundary of that input. This rule is necessary because a repeated pure 3-byte CJK name at
the default width 60 cuts at byte 57 (= 19 × 3, a valid boundary), so it does not reach the
branch and would have produced a second self-confirming test.

Plans `02-10` and `02-11` also add prohibitions beyond the seven §B lists below; `02-10` sharpens
existing prohibition 4, "No self-confirming Herald tests," into the arithmetic form stated above.

## Prohibitions (§B — LLM prose pass, no engine)

There is no compiled prohibition engine (ADR-550 D7b); the planner ran the two-stage recall pass
in-prompt. **7 distinct prohibitions** survived Stage 2, appearing as **13 statements** across the
9 plans (several are asserted by more than one plan):

1. Evidence fabrication — `02-01`, `02-09`
2. Ledger-verdict / deferral integrity — `02-01`, `02-09`
3. No silent test deletion — `02-06`, `02-07`
4. No self-confirming Herald tests — `02-04`, `02-05`
5. No silent temperature clamping — `02-02`
6. Schema stability — `02-03`
7. No second coverage number — `02-08`

All are authored **descriptor-less** (no `check_kind` / `check_target` / `check_rule` /
`check_violation_fixture` / `check_clean_fixture`) in the `must_haves.prohibitions:` sibling block,
never under `truths` (ADR-550 D3), so each disposes flagged-unverified downstream. Verified by the
plan-checker.

**Canon-referral drop (ADR-550 D6):** credential / PII handling in test fixtures is canon security
and was dropped with a breadcrumb rather than minted — `/gsd-secure-phase` owns it.
