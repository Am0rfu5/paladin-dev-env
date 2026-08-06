# ADR-0014: Milestone 4-6 tier numbering

## Status

Accepted

**Date:** 2026-08-06

## Context

Three Milestone 4-6 overview documents and two Milestone 5 PRDs number themselves against a
refactoring-tier scheme that collides with the corpus's own Milestone numbering — the same class of
defect ADR-0010 closed for Milestone 3's epic numbers, now recurring one level up, at the Milestone
integer itself.

`.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md:1`
titles itself "Milestone 1: High-Value, Low-Risk Foundations".
`.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md:1`
titles itself "Milestone 2: Workspace Decomposition" and its own `:19` Prerequisites heading reads
"Completed in Milestone 1".
`.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md:26`
lists its prerequisites as "Completed in Milestones 1 and 2".
`.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md:240` (Non-Goal 2)
cross-references "hardened in Milestone 1 / Epic 2", meaning Milestone 4 Epic 2 (Port Trait
Hardening) in the corpus's own numbering.

A reader who trusts these documents' self-titling and goes looking for "Milestone 1" or "Milestone
2" content finds Milestone 4 or Milestone 5 work instead — the same misroute ADR-0010 fixed for
epic numbers 19-23, now at the Milestone integer. `REQ-*` provenance keys in this ledger and in
`.planning/ledgers/milestone-04-06.md` only resolve uniformly across the corpus if both of the
corpus's two numbering defects close under the same convention.

## Decision

The directory / task-list numbering is authoritative: Milestone 4 is Tier 1, Milestone 5 is Tier 2,
Milestone 6 is Tier 3. Every "Milestone 1 / 2 / 3" reference appearing inside these three
milestones' own documents is a **tier label**, not a milestone number — it describes the
document's position in a three-tier refactoring sequence internal to Milestones 4-6, not the
corpus's Milestone counter. Milestone identity is compared by directory / task-list number, never
by the title a document gives itself; the tier label demotes to a descriptive detail, recorded in
the mapping table below rather than treated as a competing numbering scheme.

| Tier label | Authoritative milestone | Subject |
|---|---|---|
| Milestone 1 (Tier 1) | Milestone 4 | High-value, low-risk foundations — feature flags, port trait hardening, CLI isolation |
| Milestone 2 (Tier 2) | Milestone 5 | Cargo workspace decomposition |
| Milestone 3 (Tier 3) | Milestone 6 | Architectural refinements — config decomposition, orchestration relocation, Maneuver co-location, CircuitBreaker relocation |

## Considered Options

- Taking the tier labels as authoritative and renumbering the corpus's Milestone directories to
  match (Milestone 4 → 1, Milestone 5 → 2, Milestone 6 → 3) — rejected. It would falsify every
  `Milestone_4-…`/`Milestone_5-…`/`Milestone_6-…` directory name, every `REQ-*` provenance
  association already recorded against those directory numbers, and collide directly with the
  corpus's actual Milestone 1-3 (the pre-refactor milestones), which already use the integers 1-3
  for unrelated content.
- Leaving both numbering schemes live and disambiguating per citation (e.g. "Milestone 1" always
  means "check context") — rejected. It reproduces exactly the ambiguity ADR-0010's own
  `## Considered Options` rejected for the epic-number collision: a recorded non-answer leaves
  every downstream Phase 7-16 citation of "Milestone 1/2/3" inside these three milestones' own
  documents ambiguous between the tier label and the corpus's actual Milestone 1-3.
- The directory / task-list numbering as authoritative, with the tier label demoted to a
  descriptive detail — accepted. This is the same convention ADR-0010 already applied to the
  Milestone 3 epic-number collision: the numbering scheme agreed by the majority of the corpus's
  own documents and by every task-list filename wins, and the minority scheme's content is
  preserved as history, not deleted.

## Code Locations

- `.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md:1`
  — H1 title "Milestone 1: High-Value, Low-Risk Foundations"; authoritative Milestone is 4, tier
  label is Tier 1. Corrected by this plan (07-01) per D-08(1).
- `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md:1`
  — H1 title "Milestone 2: Workspace Decomposition"; authoritative Milestone is 5, tier label is
  Tier 2. The same file's `:19` Prerequisites heading, "Completed in Milestone 1", means Milestone
  4. Corrected by plan 07-09 per D-08(2).
- `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md:26`
  — Prerequisites heading "Completed in Milestones 1 and 2", meaning Milestones 4 and 5.
  Authoritative Milestone is 6, tier label is Tier 3. Corrected by plan 07-05 per D-08(3).
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md:240` (Non-Goal
  2) — "The port trait lives in `paladin-ports` and was hardened in Milestone 1 / Epic 2",
  meaning Milestone 4 Epic 2 (Port Trait Hardening & Stable API). Corrected by plan 07-04 per
  D-08(4).
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md:4` —
  self-identifies as "Milestone 5 (Tier 2)". CONTEXT.md D-08(5) recorded a same-shape "Milestone 1
  / Epic 2" cross-reference inside this document's §1 (Introduction / Overview, line 16 onward),
  mirroring the LLM extraction PRD's Non-Goal 2 above. Re-grepped fresh during this task
  (`grep -in "milestone 1"` and `grep -in "milestone-1"` across the whole file, 2026-08-06): no
  such text is present in the document as it currently ships. This is flagged as a drift between
  CONTEXT.md's D-08 and the live tree, not silently resolved — plan 07-02, which owns this
  document's FR-7/FR-10 annotation under ADR-0016 (D-11), should re-verify before treating this
  file as a sixth numbering-collision correction target.
- `.planning/decisions/0010-milestone-3-epic-numbering.md` — the cited precedent. That ADR closed
  the corpus's first numbering defect (Milestone 3's internal epic numbers, 19-24, colliding with
  a release-notes scheme); this ADR closes the second (the Milestone 4-6 tier-vs-milestone
  collision) with the same convention: the numbering scheme the majority of the corpus's own
  documents and every task-list filename agree on is authoritative, and the minority scheme's
  content is preserved as history via a dated correction banner, never deleted.

## Code Conformance

conforms

This is a documentation defect, not a code defect. No Rust source file, `Cargo.toml`, or
`.github/workflows/` file encodes either numbering scheme — the collision exists only in
`.project/` prose. The executing work is the five source corrections listed in `## Downstream
Consumers` below, each an in-place `.project/` annotation per D-00g, never a code change.

## Downstream Consumers

- Plan 07-01 (this plan) — corrects the Milestone 4 overview's title
  (`Milesone-4-Tier-1-High-Value-Low-Risk.md:1`), the first of the five inline corrections, proving
  the D-00g annotation pattern end to end for this ADR.
- Plan 07-02 — `prd-paladin-ports-extraction.md` §1, subject to the drift flagged in `## Code
  Locations` above; re-verify before annotating.
- Plan 07-03 — the eight byte-equivalent extract pointer banners under
  `.project/Milestone_5-.../Epic_{2,3,4,5}/` and `.project/Milestone_6-.../Epic_{1,2,3,4}/`, plus
  the seven-versus-eight count reconciliation between CONTEXT.md D-08 and
  `INGEST-CONFLICTS.md:656`.
- Plan 07-04 — `prd-paladin-llm-extraction.md` Non-Goal 2 (`:240`).
- Plan 07-05 — the Milestone 6 overview's prerequisite line (`:26`).
- Plan 07-09 — the Milestone 5 overview's title (`:1`) and Prerequisites heading (`:19`).
- Plan 07-13 — advances `.planning/decisions/PROMOTION.md`'s "Next free ADR number" line past this
  phase's full allocation (0014-0020, plus ADR-0021 if D-25a's Candidate 2 is promoted).
- Any Phase 8-16 reader citing a "Milestone 1/2/3" reference inside a Milestone 4-6 `.project/`
  document — resolves through this ADR before the document's own title, per this project's
  precedence order (D-00b).
