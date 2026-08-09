# ADR-0035: `paladin-ml` leaf-crate placement condition for a future ML adapter

## Status

Accepted

**Date:** 2026-08-08

## Context

This is a contested position and therefore ADR material under D-00g, in three measured facts.

**(1) It is the surviving half of an M8 Epic 3 §5 non-goal that `paladin-herald` already
overrode inside the same milestone.** The non-goal clause at
`Epic_3/prd-relocate-remaining-misplaced-modules.md:211` reads "No new crates created.
`paladin-herald`, `paladin-ml`, etc. are not in scope." — and `paladin-herald` was then created in
this same milestone, by reconciliation commit `66f6c4e` (`ADR-0028` (iv)). `paladin-ml` was not.
The non-goal that once bound both crates equally now binds only one of them.

**(2) It is carried today only by a DOC.** `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md`
§2 states the condition in prose, and a DOC sits second-from-bottom in the D-00b precedence order
(`ADR → shipped tree → .planning/codebase/ map → intel/code-verification.md → PRD → DOC →
task-list checkbox`) — auto-overridable by the next document that mentions it. Nothing above DOC
precedence currently carries this condition.

**(3) `PROJECT.md` lists `paladin-ml` under `### Out of Scope`,** so a reader could reasonably
conclude the placement condition is moot when it is not — the condition governs what happens *if*
a future phase reintroduces the adapter; it does not depend on the adapter being in scope today.

**What the tree shows, re-measured this session:**

```
$ test -d crates/paladin-ml; echo $?
1

$ find crates/paladin-ports -iname "*ml_port*"
crates/paladin-ports/src/input/ml_port.rs

$ grep -rn "tensorflow\|^ml = " Cargo.toml src/
(no matches, exit 1)
```

The adapter and its feature flag were deleted outright by commit `3d48768` (2026-06-04), not
feature-gated as the Epic 3 disposition record's own action cell had described. No adapter exists
anywhere in the tree, and `MlPort` is confirmed present at its stable workspace location.

## Decision

**The condition, reproduced verbatim as a quotable sentence:** *any future TensorFlow adapter goes
into a dedicated `paladin-ml` **leaf crate** with the `ml` flag on that crate, **never back into
the facade**, and `paladin_ports::input::ml_port::MlPort` **stays in the workspace** so the
integration point does not move.*

Two clauses this ADR adds around it:

**(i) The non-goal split is asymmetric and stays that way.** Overridden for `paladin-herald`
(created by reconciliation commit `66f6c4e`), still holding for `paladin-ml` (absent from the
tree). The existence of the first is not licence to create the second; each half of the split is
independent and must be judged on its own merits, not by analogy to the other.

**(ii) This ADR creates no crate and authorises none.** It records the condition that governs
re-entry, should a future phase choose to reintroduce the TensorFlow adapter — it is not itself a
decision to re-enter. `paladin-ml` remains out of scope per `PROJECT.md`; reintroduction would
need its own phase and its own decision, made deliberately rather than inferred from this ADR's
existence.

## Considered Options

- **Promote the condition into an ADR and leave the crate out of scope** (accepted) — resolves the
  precedence problem (the condition now sits at the top of D-00b rather than at DOC level) without
  taking any action the corpus has not authorised; `paladin-ml` stays absent exactly as
  `PROJECT.md` records it.
- **Record it in a register only** (rejected) — a register does not outrank the PRD/PROJECT.md
  material that lists `paladin-ml` as out of scope; the condition's whole problem is that it sits
  at DOC precedence today, and a register is not a precedence promotion.
- **Promote `paladin-ml` to scope and create the crate** (rejected) — `PROJECT.md` lists it out of
  scope, D-09 chooses recorded deferral over promotion to scope, and there is no adapter to put in
  the crate even if it were created — the whole feature was deleted, not gated.
- **Treat `paladin-herald`'s creation as having voided the whole non-goal** (rejected) — per D-00k,
  the split is asymmetric: `paladin-herald`'s existence is a fact about one half of a two-crate
  non-goal, and each half stands alone. Voiding the whole non-goal on one crate's evidence would be
  exactly the reasoning error D-00k exists to block.

## Code Locations

- `crates/paladin-ports/src/input/ml_port.rs` — the integration point the condition says must not
  move. Confirmed present this session via `find crates/paladin-ports -iname "*ml_port*"`.
- `.planning/PROJECT.md` — the `### Out of Scope` entry naming `paladin-ml` alongside
  `paladin-arsenal` and `paladin-sanctum`, binding on this ADR's clause (ii).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` §2 — the source
  document this ADR promotes out of DOC precedence, per `PROMOTION.md`'s Part A step 4. The
  original condition wording lives there and is reproduced verbatim in `## Decision` above.
- `Epic_3/prd-relocate-remaining-misplaced-modules.md:211` — the M8 Epic 3 §5 non-goal clause
  naming both `paladin-herald` and `paladin-ml`.
- Commit `3d48768` (2026-06-04) — the removal commit that deleted both
  `src/infrastructure/adapters/input/tensorflow_adapter.rs` (636 LOC) and the `ml = []` feature
  flag outright, rather than feature-gating either.
- Re-run this session: `test -d crates/paladin-ml; echo $?` → `1`;
  `find crates/paladin-ports -iname "*ml_port*"` → `crates/paladin-ports/src/input/ml_port.rs`;
  `grep -rn "tensorflow\|^ml = " Cargo.toml src/` → no matches.

## Code Conformance

conforms

The tree already satisfies the condition: no ML adapter exists anywhere, `MlPort` is already in
the workspace at its stable location, and the facade carries no `ml` feature. This ADR instructs
no code change (D-13).

## Downstream Consumers

- **Any future phase that reintroduces ML support** — inherits the placement condition as
  binding. Any future TensorFlow adapter must go into a dedicated `paladin-ml` leaf crate with the
  `ml` flag on that crate, never back into the facade, and must not move
  `paladin_ports::input::ml_port::MlPort`.
- **`.planning/registers/facade-03-removed-features.md`** — cites this ADR as the condition's
  authoritative, durable home rather than restating the condition as its own authority.
- **Plan 11-05** — amends the `REQ-deferred-tensorflow-ml-adapter-v3` ledger row against this ADR
  and adds this ADR's row to `.planning/PROJECT.md`'s `## Key Decisions` table.
