# ADR-0030: Milestone 7's self-numbering collision — directory numbering is authoritative

## Status

Accepted

**Date:** 2026-08-08

## Context

The Milestone 7 overview carries two separate defects, kept visually distinct here even though one
document carries both.

**Defect 1 — the self-title.** The overview at
`.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:1`
titles itself:

```
# Milestone 4: Production Hardening and Extended Workspace Decomposition
```

while its own path reads `Milestone_7-Production-Hardening`, and its `**Milestone:**` metadata line
at `:5` reads `Tier 4 — Production Hardening, Extended Crate Extraction, and API Stabilization`.

**Defect 2 — the Prerequisites mis-credit.** The `### Prerequisites (Completed in Milestones 1–3)`
section at `:25` credits "Milestones 1-3" with work that directory numbering assigns to Milestones
4-6. Six of this section's items are Milestone 4-6 deliverables already ledgered in
`.planning/ledgers/milestone-04-06.md`, named individually so none is left as an unenumerated
etcetera:

1. **Feature flags and the CI matrix** — `:27`, "Feature flags expanded and CI matrix in place
   (Milestone 1)." Ledgered as `REQ-feature-flag-matrix`, Milestone 4 Epic 1
   (`.planning/ledgers/milestone-04-06.md:112`).
2. **The core workspace crates** — `:28`, "Core workspace crates extracted: `paladin-core`,
   `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory` (Milestone 2)." Ledgered as
   `REQ-cargo-workspace-root`, Milestone 5 Epic 1 (`.planning/ledgers/milestone-04-06.md:152`).
3. **`application_settings.rs` decomposition** — `:30`, "`application_settings.rs` decomposed into
   per-domain config modules (Milestone 3)." Ledgered as `REQ-config-incremental-migration` /
   `REQ-config-success-metrics`, Milestone 6 Epic 1 (`.planning/ledgers/milestone-04-06.md:250,253`).
4. **Manager-service relocation** — `:31`, "Manager services relocated to the application layer;
   core layer purity verified (Milestone 3)." Ledgered as `REQ-manager-services-retained`,
   Milestone 6 Epic 2 (`.planning/ledgers/milestone-04-06.md:263`).
5. **Maneuver DSL co-location** — `:32`, "Maneuver DSL co-located in `paladin-battalion`
   (Milestone 3)." Ledgered as `REQ-maneuver-cargo-dependency-check`, Milestone 6 Epic 3
   (`.planning/ledgers/milestone-04-06.md:287`).
6. **`CircuitBreaker` relocation** — `:33`, "`CircuitBreaker` relocated to infrastructure layer
   (Milestone 3)." Ledgered as `REQ-circuitbreaker-relocation`, Milestone 6 Epic 4
   (`.planning/ledgers/milestone-04-06.md:294`).

Each of these six is a Milestone 4-6 deliverable already ledgered; this ADR cites that ledger
rather than re-asserting the mapping in a fifth place.

## Decision

Under the D-00b precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox — an ADR that contradicts shipped code is an
instruction to change the code), **directory and task-list numbering is authoritative**: this
document describes Milestone 7, and its Prerequisites describe Milestones 4-6.

This is the **third application** of the convention and the **fourth instance** of the collision
class in this corpus. It cites **ADR-0010** first — `.planning/decisions/0010-milestone-3-epic-numbering.md`,
which closed the Milestone 3 epic-numbering collision (release-notes Epics 19-23 assigned to four
Milestone 2 features) — and **ADR-0014** second —
`.planning/decisions/0014-milestone-4-6-tier-numbering.md`, which closed the Milestone 4-6
tier-versus-milestone collision (the M4-M6 overviews titling themselves "Milestone 1/2/3" by
refactoring tier). Both precedents are cited in this chronological order because that is the order
in which the convention was established and then reapplied.

The cross-reference is mandatory, not decorative: it is what makes `REQ-*` provenance keys resolve
uniformly across four ledgers (`milestone-01.md`, `milestone-02-03.md`, `milestone-04-06.md`, and
this phase's `milestone-07-08.md`), and it is the whole reason the convention is worth an ADR each
time instead of a footnote.

**This ADR cites `milestone-04-06.md` rather than re-asserting the mapping.** All six mis-credited
items are already ledgered there with their own `file:line`-cited verdicts; restating the
assignment in a fifth place is how a fifth version of it starts to diverge from the ledger's own
wording.

**The Roadmap Extension Protocol's predicted fifth instance is closed with this fourth instance.**
`ROADMAP.md:108-114` states: "Milestone numbering follows the directory / task-list numbering. Four
source milestones number themselves differently... The protocol predicted a fifth instance in run
5; run 5 found none, and ORCH-05 records the prediction closed." This ADR records the Roadmap
Extension Protocol item as **discharged with this fourth instance**, so no later phase inherits a
standing prediction to check.

## Considered Options

- **Annotate at source and record the convention in a fourth ADR citing its two precedents**
  (accepted) — matches the shape of ADR-0010 and ADR-0014 exactly, and keeps `REQ-*` provenance
  resolving uniformly.
- **A footnote in the ledger instead of an ADR** (rejected) — the cross-reference to ADR-0010 and
  ADR-0014 is what makes provenance keys resolve, and a footnote is not citable by number the way an
  ADR is.
- **Retitling the overview to match its directory** (rejected) — D-00c forbids rewriting `.project/`
  documents; the self-title is the evidence of the collision, and a quietly retitled document
  destroys the record this ADR exists to make.
- **Carrying the fifth-instance prediction forward unclosed** (rejected) — run 5 found none, and an
  unclosed prediction costs every later phase a check that has no possible finding.

## Code Locations

- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:1`
  — the self-title, "Milestone 4: Production Hardening and Extended Workspace Decomposition".
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:25`
  — the `### Prerequisites (Completed in Milestones 1–3)` heading.
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:27-34`
  — the eight Prerequisites bullets, six of which map to the mis-credited items enumerated above.
- `.planning/ledgers/milestone-04-06.md:112` — `REQ-feature-flag-matrix` (item 1).
- `.planning/ledgers/milestone-04-06.md:152` — `REQ-cargo-workspace-root` (item 2).
- `.planning/ledgers/milestone-04-06.md:250,253` — `REQ-config-incremental-migration` /
  `REQ-config-success-metrics` (item 3).
- `.planning/ledgers/milestone-04-06.md:263` — `REQ-manager-services-retained` (item 4).
- `.planning/ledgers/milestone-04-06.md:287` — `REQ-maneuver-cargo-dependency-check` (item 5).
- `.planning/ledgers/milestone-04-06.md:294` — `REQ-circuitbreaker-relocation` (item 6).
- `.planning/decisions/0010-milestone-3-epic-numbering.md` — the first precedent.
- `.planning/decisions/0014-milestone-4-6-tier-numbering.md` — the second precedent.
- `ROADMAP.md:108-114` — the Roadmap Extension Protocol statement that the fifth instance was
  predicted and that run 5 found none.

## Code Conformance

conforms

This is a documentation defect, not a code defect. No `.rs` file, `Cargo.toml`, or CI workflow
encodes either numbering scheme — the collision exists only in `.project/` prose. The executing
work is plan 10-03's own Task 3 annotation of the overview, an in-place `.project/` correction per
D-00c, never a code change.

## Downstream Consumers

- **Phase 10 / HARD-01** — the ledger's M7 epic sections, whose section headings use directory
  numbering on this ADR's authority.
- **Phase 13 / ORCH-05** — records the Roadmap Extension Protocol's fifth-instance prediction closed
  (per this ADR's third sub-decision) and must not re-open it.
