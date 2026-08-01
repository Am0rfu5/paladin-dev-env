# ADR-0007: Battalion-wide cancellation deferral

## Status

Accepted

**Date:** 2026-08-01

## Context

`REQ-battalion-cancellation` (Epic 4) asks for cancellation support across Battalion orchestration
as a whole — the requirement is written pattern-agnostic, not scoped to one of the four execution
services. The shipped reality is narrower: `execute_with_cancellation`
(`crates/paladin-battalion/src/phalanx_service.rs:151`) exists **only on Phalanx**, and is exercised
by the passing test `test_cancellation_support`
(`crates/paladin-battalion/src/phalanx_service.rs:758`), confirmed re-run clean on this phase's
2026-08-01 baseline (`02-test-baseline.md`).

Verified directly against the current tree: `grep -rn "execute_with_cancellation\|CancellationToken"
crates/paladin-battalion/src/{formation_service.rs,campaign_service.rs,chain_of_command_service.rs,
commander.rs}` returns **zero matches**. Formation, Campaign and ChainOfCommand expose no
cancellation entry point at all — not a partial or untested one, an absent one. The requirement as
written is therefore citation-backed for exactly one of the four Battalion patterns and unbuilt for
the other three.

`.planning/ledgers/milestone-01.md`'s `REQ-battalion-cancellation` row already records this same
finding as `present, unproven` (Phase 1) and Phase 2's `02-CONTEXT.md` D-05 named the split
disposition and D-08 named the escalation to an ADR, because a `deferred with reason` ledger row
alone would silently override a written requirement (D-03's exception rule) without a recorded
reason a later phase could find.

## Decision

- `REQ-battalion-cancellation` is **satisfied** for Phalanx, citing `phalanx_service.rs:151` and its
  passing test at `:758`.
- `REQ-battalion-cancellation` is **deferred with reason** for Formation, Campaign and
  ChainOfCommand. Building cancellation for three more execution services is **new capability, not
  gap closure**: it requires a cancellation *contract* across four services, including two genuinely
  open design questions this ADR does not answer — what a cancelled run returns mid-DAG in Campaign
  (which nodes' partial results survive, and in what state) and mid-delegation in ChainOfCommand
  (does an in-flight specialist's partial output count, and does the chain's own retry/fallback logic
  need to observe cancellation separately from the specialist it delegated to). Neither question has
  an existing answer anywhere in this codebase or in the ingested corpus.
- Corroborating evidence that this was never Phase 2's scope: `.planning/ROADMAP.md` § "Phase 2:
  Functional Gap Closure"'s five success criteria never mention cancellation in any form.
- **Forward owner, named explicitly: the v2 backlog, gated on a recorded cancellation-contract
  decision.** Phase 3 (`QUAL-01` … `QUAL-04`) was considered and rejected as owner: every QUAL item
  is verification-depth work — raising coverage to ADR-0006's floor, un-ignoring four existing
  Commander error tests, and MCP failure-mode tests — and none of them builds new Battalion
  capability, confirmed by reading `.planning/ROADMAP.md` § "Phase 3: Verification Depth"'s own five
  success criteria directly. Assigning this deferral to Phase 3 would be assigning work to a phase
  whose stated goal excludes it.
- **Prerequisite, named as its own line:** the mid-run-return contract for Campaign and
  ChainOfCommand — what a cancelled run's `BattalionResult` (or equivalent partial type) contains for
  each pattern — must be decided *before* any implementation begins. That decision is itself
  ADR-shaped: it fixes a public-facing return contract across two execution services, the same class
  of decision this ADR itself is.

## Considered Options

- **Implement cancellation across all four patterns in this phase** — rejected. This is the single
  largest unbudgeted item Phase 2 could take on: three new execution-service code paths plus the two
  open design questions above, none of which any Phase 2 plan scoped for, and none of which any
  ROADMAP Phase 2 success criterion asks for.
- **A Formation-only half-measure** (implement cancellation for Formation alone, leaving Campaign and
  ChainOfCommand deferred) — rejected. A partial extension still requires deciding Formation's own
  mid-run-return shape without deciding it for the other two, which would produce three inconsistent
  cancellation contracts across the four patterns instead of one designed together — worse than
  deferring all three uniformly.
- **Record this as a plain `deferred with reason` ledger row with no ADR** — rejected per D-03's own
  exception: a deferral that overrides a written requirement (as this one does — `REQ-battalion-cancellation`
  is written battalion-wide, not Phalanx-only) escalates to an ADR so the override is a decision with
  a recorded reason, not a ledger row a later reader could mistake for an oversight.

## Code Locations

- `crates/paladin-battalion/src/phalanx_service.rs:151` — `execute_with_cancellation`, the only
  shipped cancellation entry point in the workspace.
- `crates/paladin-battalion/src/phalanx_service.rs:758` — `test_cancellation_support`, the passing
  test that exercises it (re-confirmed passing on this phase's `02-test-baseline.md` measurement,
  commit `7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c`).
- `crates/paladin-battalion/src/formation_service.rs`,
  `crates/paladin-battalion/src/campaign_service.rs`,
  `crates/paladin-battalion/src/chain_of_command_service.rs`,
  `crates/paladin-battalion/src/commander.rs` — verified to have **no** corresponding site: `grep -rn
  "execute_with_cancellation\|CancellationToken"` across all four files returns zero matches. This
  absence is the point the ADR records, not an oversight in the grep.

## Code Conformance

conforms

This ADR records a scope decision for `REQ-battalion-cancellation` and mandates **no code change**.
The Phalanx half is already shipped and tested; the other three patterns stay unbuilt until the
prerequisite contract decision above is made and a future phase (the named v2 backlog owner) picks
up the implementation. No later phase should read this ADR as pending work for Phase 2 or Phase 3.

## Downstream Consumers

- `.planning/ledgers/milestone-01.md` § Epic 4, the `REQ-battalion-cancellation` row — amended by
  plan 02-09 to cite this ADR (`0007`) as the deferring authority for the Formation/Campaign/ChainOfCommand
  half, with the Phalanx half's existing `satisfied` citation preserved unchanged.
- **The v2 backlog** — the named forward owner, gated on the cancellation-contract decision named
  above as this ADR's prerequisite.
