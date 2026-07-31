# ADR-0003: Minimum Paladin count for Formation

## Status

Accepted

**Date:** 2026-07-31

## Context

What is Formation's minimum Paladin count, and how does it relate to the Commander's Auto
routing rule for a single Paladin? Two REQUIREMENTS.md variants disagree on paper —
`REQ-formation-min-paladins-v1` (Epic 4 FR-4.5 / FR-4.8) requires Formation to validate at
least 2 Paladins; `REQ-formation-min-paladins-v2` (Epic 5 FR-1 / FR-3) has the Commander
validate only that at least 1 Paladin is provided, with Auto rule 1 routing a single Paladin to
Formation as the trivial case.

The usual resolution — "the shipped tree wins" — cannot settle this one, because **shipped code
contains both halves of the contradiction at once.** `test_auto_selects_formation_for_single_paladin`
in `crates/paladin-battalion/src/commander.rs` (around line 1912) is a passing, non-ignored test
that builds a Commander with exactly one Paladin under `BattalionStrategy::Auto`, calls
`analyze_and_select`, and asserts the result is `BattalionStrategy::Formation` with a reason
string containing "Single Paladin". At the same time, `Formation::validate` in
`crates/paladin-core/src/platform/container/battalion/formation.rs` (around lines 109-111)
rejects any Paladin count below 2 with the error message "Formation requires at least 2
Paladins, got {}". A single Paladin routed to Formation by the passing Auto test would fail this
validation the instant it executed. This is a second instance of the Group-29 class the ingest
corpus identified — a variant shipped code does not settle by itself, because the tree argues
with itself rather than with a document. The corpus believed the token mechanism (WEB-01) was
its only member of this class; RECON-04 is the second.

## Decision

- Formation relaxes its minimum to **one Paladin**. The contract is recorded precisely as an
  integer count: fewer than 1 Paladin is rejected, 1 or more is accepted. There is no rounding
  and no float comparison involved — `paladins.len()` is compared directly against the integer
  bound.
- Boundary behavior, stated explicitly: at **0** Paladins, Formation still rejects (empty
  Battalions remain invalid). At **exactly 1** Paladin, Formation now accepts — this is the
  change this ADR makes. At **exactly 2** Paladins, behavior is unchanged from today (Formation
  already accepted 2).
- The passing Commander test, `test_auto_selects_formation_for_single_paladin`, and the Auto
  routing rule it exercises (`analyze_and_select`) are left **untouched**. Relaxing Formation's
  validation is what makes the test and the runtime check consistent with each other — rewriting
  `analyze_and_select` so Auto never routes a single Paladin to Formation would break a currently
  passing test and would directly contradict Phase 2's success criterion 5, which names "a
  single-Paladin Commander in Auto mode that executes instead of failing Formation validation" as
  a thing that must become true.
- **Majority aggregation keeps its independent minimum of 3 Paladins.** That check lives in
  Phalanx's aggregation-strategy validation (`crates/paladin-core/src/platform/container/battalion/phalanx.rs`,
  around lines 141-146, "Majority aggregation requires at least 3 Paladins"), is a different rule
  for a different reason (a `Majority` vote needs a real electorate), and is not touched by this
  decision.

## Considered Options

- `REQ-formation-min-paladins-v1` (Epic 4 FR-4.5 / FR-4.8, Formation MUST validate ≥ 2 Paladins)
  — rejected as the sole answer; it is what shipped code currently enforces, but enforcing it
  literally leaves the passing Commander Auto test asserting a strategy selection that would fail
  at execution time, which is the contradiction this ADR exists to resolve.
- `REQ-formation-min-paladins-v2` (Epic 5 FR-1 / FR-3, Commander validates only ≥ 1 Paladin,
  Auto routes a single Paladin to Formation) — accepted in substance as the chosen decision above,
  but recorded here as a considered option because on its own it says nothing about Formation's
  own validation, which is the actual site of the runtime rejection this ADR fixes.
- Rewriting `analyze_and_select` so Auto never selects Formation for a single Paladin (routing it
  to some other strategy, or refusing to build a single-Paladin Commander under Auto at all) —
  rejected. This breaks `test_auto_selects_formation_for_single_paladin`, a currently passing
  test, and contradicts Phase 2's success criterion 5 as written.
- Failing at `CommanderBuilder::build()` for a single-Paladin Auto configuration — rejected. This
  removes single-Paladin Commander construction as a capability rather than reconciling the two
  halves of the contradiction; that is a capability reduction (a construction path that succeeds
  today would start failing), and this ADR rejects capability reductions dressed up as neutral
  fixes.

## Code Locations

- `crates/paladin-core/src/platform/container/battalion/formation.rs:109-111` — `Formation::validate`, the rejection this ADR relaxes ("Formation requires at least 2 Paladins, got {}")
- `crates/paladin-battalion/src/commander.rs:1911-1927` — `test_auto_selects_formation_for_single_paladin`, the passing test that asserts the Auto rule routes a single Paladin to `BattalionStrategy::Formation`
- `crates/paladin-core/src/platform/container/battalion/phalanx.rs:141-146` — Majority aggregation's independent minimum of 3 Paladins, left unchanged by this decision
- `.planning/REQUIREMENTS.md:1752-1761` — Group 5, the competing `REQ-formation-min-paladins-v1` / `-v2` variant pair this ADR resolves

## Code Conformance

must change

`crates/paladin-core/src/platform/container/battalion/formation.rs:109` currently rejects fewer
than 2 Paladins; this decision requires it to reject only fewer than 1. **GAP-07** in Phase 2 is
the requirement that lands this change (Phase 2 success criterion 5: "a single-Paladin Commander
in Auto mode that executes instead of failing Formation validation"). Nothing in this phase edits
Rust source — this ADR records the decision only.

## Downstream Consumers

- Phase 2 GAP-07 — implements the relaxed `Formation::validate` bound at `formation.rs:109-111`.
- The Commander Auto routing path (`analyze_and_select` in `crates/paladin-battalion/src/commander.rs`)
  — no code change required here, but this is the consumer whose currently-passing test this
  decision makes consistent with runtime behavior.
