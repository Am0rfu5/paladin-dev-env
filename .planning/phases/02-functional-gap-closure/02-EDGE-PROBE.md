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
| Authored `covered` (plain `must_haves.truths` strings) | 4 — `02-02` (GAP-07 precision), `02-03` (GAP-07 boundary), `02-04` (GAP-03 empty, encoding) |
| Authored `backstop` (`{ statement, verification: backstop }` markers) | 13 — `02-01` ×9, `02-02` ×1, `02-08` ×3 |
| Flagged assumptions (`unresolved`) | 1 — GAP-04 unclassified, in `02-01` |
| **Total accounted** | **18** |

18 ≥ 17: no silent drops. The surplus of one is a `backstop` marker in `02-02` that the planner's
own return summary omitted from its count (it reported 12 backstop, the plans contain 13). The
plan-checker flagged the discrepancy; the extra predicate is additive, so it is recorded here
rather than removed.

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
