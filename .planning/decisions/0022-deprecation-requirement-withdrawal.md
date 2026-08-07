# ADR-0022: Milestone 4 Epic 2 FR-8 deprecation requirement withdrawn

## Status

Accepted

**Date:** 2026-08-06

## Context

Milestone 4 Epic 2 FR-8 requires every type leaving the public API to carry
`#[deprecated(since = …, note = …)]`. Re-run this session:
`grep -rn '#\[deprecated' src crates` returns **0**. DEBT-02's requirement text permits either
implementing FR-8 or withdrawing it with a recorded reason, and explicitly forbids a third state —
neither implemented nor withdrawn, left dangling as an open promise.

The evidence for which way this settles comes from the epic's own tracking document, not from
inference. `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:81`'s
**⚠ IMMEDIATE DEPRECATION** section — the only category that would produce a `#[deprecated]`
attribute — lists, in full: *"None identified yet - managers are currently `pub(crate)` or will be
moved to application layer (Epic 3)"*, and its sibling sub-categories read "Migration Path: TBD" and
"List: TBD based on usage analysis" (`:171` "Current Status", `:190` "Deprecation Log" — re-read this
session, "*No deprecations added yet.*"). The same document's **SOFT DEPRECATION** category resolves
to `#[doc(hidden)]`, not `#[deprecated]` — and the tree carries `grep -rn 'doc(hidden)' src crates` →
**38** occurrences, confirming that half of the document's own plan **was** executed. Its
**INTERNAL-ONLY** category resolves to `pub(crate)`, an annotation-free outcome by design.

The document's stated timeline — **v0.2.0 → v0.3.0 → v1.0.0** — is stale by five minor versions: the
workspace ships at **`0.7.0`** (root `Cargo.toml:34`, re-verified this session:
`version = "0.7.0"`). A `#[deprecated(since = …)]` attribute written today cannot honour a removal
schedule keyed to a v0.3.0 milestone that has already passed.

Per this corpus's precedence framing (D-00b), the zero grep result is therefore not an unfinished
task — it is the outcome the epic's own tracking document already decided, and that document is the
higher-fidelity source over an inference drawn from the grep count alone.

## Decision

Milestone 4 Epic 2 FR-8 is **withdrawn**. Zero `#[deprecated]` attributes is the correct terminal
state for the 0.7.0 tree; manufacturing deprecations to satisfy a grep would be the dishonest
closure, not the honest one.

The stale timeline is restated, not silently dropped, per D-08 — the same treatment ADR-0020 gave
the stale build-benchmark artefact: judge it, say why it is stale, state what replaces it. The
document's `v0.1.0 → v0.2.0 → v0.3.0 → v1.0.0` schedule is stale by five minor versions against the
shipped `0.7.0` (root `Cargo.toml:34`); a `#[deprecated(since = …)]` attribute written today could
not honour a removal schedule keyed to a version that has already shipped and passed. Per
[ADR-0008](0008-workspace-version-0-7-0.md), the pre-1.0 series absorbs API evolution through minor
bumps rather than a named-release removal schedule, so the deprecation policy's version anchors move
to **"one minor version"** rather than named releases that already shipped.

Stated explicitly, the target of this withdrawal: the *policy* — how a future deprecation will work —
survives in `docs/src/api-reference/stable-api.md`; only the claim that deprecations **exist today**
is withdrawn. Nothing in the deprecation-lifecycle prose is deleted.

Re-instatement is possible but is written down as an instruction, not mechanised: any future ADR
that wants to bring FR-8 back must explicitly supersede this one. This corpus's `PROMOTION.md`
defines a supersession mechanism (superseded ADR's `## Status` becomes `Superseded` with a pointer;
the superseding ADR carries a `## Supersedes` line), but nothing in it addresses two ADRs disagreeing
about a policy's *existence* rather than its *content* — so a future re-instatement means re-deriving
the candidate list Epic 2 never produced, against a 0.7.0-or-later tree, from scratch (edge probe
E-02a, `08-04-PLAN.md`).

## Considered Options

- **Withdraw with a recorded reason** (accepted) — matches the epic's own tracking document, which
  already answered "none identified" for the only category that would produce a `#[deprecated]`
  attribute; requires no invented candidate list; the honest closure.
- **Implement FR-8 by deprecating something** (rejected) — no named candidate exists anywhere in the
  263-document corpus. `DEPRECATIONS.md`'s IMMEDIATE DEPRECATION section says "None identified yet"
  in its own words; an implementer would have to invent a deprecation target purely to make a grep
  return non-zero, which is manufacturing evidence rather than reporting it.
- **Leave it open** (rejected) — DEBT-02's requirement text explicitly permits withdrawal and
  explicitly forbids the third state where a document promises a timeline the tree cannot start
  (the v0.2.0-anchored schedule is already five minor versions behind the shipped tree).

## Code Locations

- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:81` — the **⚠ IMMEDIATE
  DEPRECATION** section header; its "Manager Services" sub-category lists "None identified yet -
  managers are currently `pub(crate)` or will be moved to application layer (Epic 3)".
- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:171` — "## Current Status".
- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:190` — "## Deprecation Log",
  reading "*No deprecations added yet.*".
- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:206-211` — "## Open
  Questions", the four questions Phase 8 / plan 08-06 closes under this withdrawal.
- `docs/src/api-reference/stable-api.md:875` — the single present-tense
  "**[Deprecations Tracking](…CHANGELOG.md)** - Current and planned deprecations" claim that is false
  today; the one line Phase 8 / plan 08-06 corrects.
- `scripts/check-deprecations.sh` — the gate, fixed in Phase 8 / plan 08-02 (DEBT-01) so it can
  genuinely fail, deliberately **without** a "deprecations must exist" check that would have
  prejudged this ADR.
- root `Cargo.toml:34` — `version = "0.7.0"`, re-verified this session.
- `grep -rn '#\[deprecated' src crates` → **0** matches, re-run this session (2026-08-06).
- `grep -rn 'doc(hidden)' src crates` → **38** matches, re-run this session (2026-08-06) — the SOFT
  DEPRECATION category's executed half.

## Code Conformance

must change

**Phase 8 itself** is the named executor. Plan 08-06 performs the three-way reconciliation this
withdrawal requires: a dated D-00c banner on `DEPRECATIONS.md` recording the withdrawal, this ADR's
number, and that its "Current Status" / "Deprecation Log" zeros are the *outcome* of the epic's own
decisions rather than a gap — with its four Open Questions answered or closed, not left dangling;
the `stable-api.md:875` line corrected so it no longer claims current deprecations exist while the
surrounding deprecation-lifecycle policy prose is left intact; and the tree left unchanged at zero
`#[deprecated]` attributes as the recorded-correct state, not a defect to fix.

## Downstream Consumers

- Phase 8 / plan 08-06 — performs the three-way reconciliation this ADR's `must change` requires.
- Phase 8 / plan 08-09 — updates the `REQ-deprecation-warnings` row at
  `.planning/ledgers/milestone-04-06.md:116`, flips the DEBT-02 checkbox in `REQUIREMENTS.md`, adds
  this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index and advances its "Next free
  ADR number" line to 0024, and adds the corresponding row to `PROJECT.md`'s Key Decisions table.
- Any future phase that re-opens API-governance / deprecation policy — must author a new ADR that
  explicitly supersedes this one, per `PROMOTION.md`'s supersession mechanism.
