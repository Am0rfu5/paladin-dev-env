# ADR-0021: CLI placement in the application layer

## Status

Accepted

**Date:** 2026-08-06

## Context

`.planning/decisions/PROMOTION.md` Part B names two promotable ADR candidates owned by Phase 7.
Candidate 1 (`battalion-result-upward-dependency-decision.md`) is promoted by ADR-0016. Candidate 2
— `.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md`, a run-2 document — was not
allocated a number by `07-CONTEXT.md`'s original D-25 ADR table and was given one, ADR-0021, by
D-25a during research, on the grounds that leaving a candidate whose recorded owner is this phase
unowned is not an acceptable disposition.

The candidate's substance: `epic17-5.md` resolves a duplicate-CLI-module consolidation question
between two competing implementations (`src/cli/`, Epic 10's "Armory CLI Tools", vs
`src/application/cli/`, Epic 18's richer formatters and wizard framework) and recommends
consolidating into `src/application/cli/`, reasoned explicitly from Hexagonal Architecture: "CLI is
an **input adapter** in the application layer, not infrastructure" (`epic17-5.md:34`). Shipped code
already implements this: `src/application/cli/` exists, `src/cli/` does not (confirmed this task),
and `src/application/mod.rs:59` declares `pub mod cli;`. The candidate is nonetheless outranked in
the corpus by a PRD that says otherwise on the numbering question this ADR does not re-decide — this
is exactly the class of already-shipped-but-outranked-on-paper decision an ADR exists to protect,
per `PROMOTION.md`'s own promotion procedure.

`.planning/codebase/ARCHITECTURE.md:88-93` states the layering rule the candidate is expressed
against: the "Application Layer (Facade `paladin`)" is described as containing
"PaladinExecutionService, Battalion services, orchestrators, registry services, CLI" and depending on
"Core + Ports + Infrastructure adapters" — the CLI is already documented in this codebase map as an
application-layer occupant, consistent with the candidate's reasoning.

## Decision

**`src/application/cli` is ratified as the recorded home for the CLI input adapter**, per the
candidate's Hexagonal Architecture reasoning: a CLI is an input adapter that drives the application
layer, not an infrastructure implementation detail. This closes `PROMOTION.md`'s second Phase-7-owned
candidate.

**A separate observed fact from the same surface, recorded here rather than corrected:** during this
task, `src/application/mod.rs:57-59` was re-read and found to gate `pub mod cli;` behind
`#[cfg(feature = "cli")]`, and `src/lib.rs:155-156` gates the corresponding `pub use application::cli;`
re-export the same way. This is stated plainly because `07-CONTEXT.md`'s D-25a research described
this declaration as **un-gated** — that description does not hold against the tree as it stands at
authoring time and is corrected here rather than silently repeated. The module-gating layer (FR1 of
the CLI-isolation PRD) is satisfied; ADR-0019 records the surface that remains unresolved for
library-only builds — the *dependency*-declaration layer (`structopt`, `colored`, `comfy-table` as
unconditional `Cargo.toml` entries), not the module-gating layer this ADR touches. Cross-reference
ADR-0019 for the binary-target side of the same surface.

## Considered Options

- Promote the candidate as its own ADR (accepted) — it is separately citable, `PROMOTION.md` records
  Phase 7 as its owner, and D-25a's disposition requires either a promotion or an explicit deferral
  rather than silence.
- Record an explicit deferral in `PROMOTION.md` naming a later owner (rejected) — the recorded owner
  is this phase; leaving an unowned candidate whose recorded owner is Phase 7 is the exact state
  D-25a exists to close.
- Treat the candidate as already covered by ARCH-05's `src/application/use_cases/` correction
  (rejected) — that correction (ADR-0018) is about the orchestration-services home, not the CLI's
  layer; the two subjects share a directory tree but not a question.

## Code Locations

- `src/application/mod.rs:57-59` — `#[cfg(feature = "cli")] pub mod cli;`, the gated module
  declaration.
- `src/lib.rs:155-156` — `#[cfg(feature = "cli")] pub use application::cli;`, the gated re-export.
- `src/application/cli/` — the directory itself, confirmed to exist; `src/cli/` confirmed absent.
- `.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md:34` — the Hexagonal Architecture
  reasoning this ADR promotes, and the source document's provenance.
- `.planning/codebase/ARCHITECTURE.md:88-93` — the Application Layer description already naming CLI
  as an application-layer occupant.
- `.planning/decisions/0019-binary-target-architecture.md` — the sibling answer on the same
  binary/CLI surface.

## Code Conformance

conforms

Shipped code already implements this placement — `src/application/cli/` exists and `src/cli/` does
not. No code changes result from this ADR. The module-gating finding recorded above (both the module
declaration and its re-export are correctly feature-gated) is a correction to an earlier research
description, not a conformance failure of this decision.

## Downstream Consumers

- **Phase 8's CLI-isolation requirement** — receives this ADR's corrected module-gating finding: FR1
  (module gating) is satisfied at both `src/application/mod.rs` and `src/lib.rs`; the unresolved
  surface is the dependency-declaration layer ADR-0019 names.
- **Plan 07-12** — Milestone 4 Epic 3 ledger rows may cite this ADR for the CLI-placement question.
- **Plan 07-13** — adds this ADR's row to `.planning/decisions/PROMOTION.md`, closes candidate 2 in
  the Part B inventory, and advances the next-free ADR number to **0022** rather than 0021.
