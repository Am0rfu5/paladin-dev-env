# ADR-0016: Port value-type ownership

## Status

Accepted

**Date:** 2026-08-06

## Context

Five pure value/error types — `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`,
`HandoffError` — were the subject of an Epic 1 decision record,
`.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
(`Status: Approved`, `Decision Date: 2026-05-13`), which chose "Option A — Move Pure Value Types to
`paladin-core`" specifically to resolve an upward dependency from `src/core/` into `application::`.
That decision record settles the *location* of exactly those five types; despite its filename
("battalion-result-upward-dependency-decision.md") it never mentions `BattalionResult`, so this ADR
does not settle that separate variant.

That decision record is manifest-typed DOC with `locked: false`. Under this corpus's precedence
order (D-00b: ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD →
DOC → task-list checkbox), a DOC-typed record sits below a PRD. The Epic 2 PRD,
`.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md`, was published
two days later (`Last Updated: 2026-05-15`) and outranks the Epic 1 decision record on paper. That
PRD's FR-10 — "Types must not be split across crates" — applied literally would pull
`PaladinResult`, `StopReason` and `TokenUsage` back out of `paladin-core` and into `paladin-ports`,
reintroducing the exact upward dependency the Epic 1 decision was written to remove. This is the one
place in the corpus where mechanical precedence, applied without correction, produces the
architecturally wrong result.

Shipped code implements the Epic 1 decision record, not the Epic 2 PRD's FR-10: `TokenUsage` is
defined in `paladin-core`, `PaladinResult` and `StopReason` are defined in `paladin-core`, and
`paladin-ports` holds re-exports (in the case of `RegistryError`, already; see `## Decision` below
for the other two).

## Decision

`paladin-core` owns `PaladinResult`, `StopReason` and `TokenUsage`. `paladin-ports` holds thin
re-exports of the `paladin-core` definitions, not independent bodies. This extends Epic 2 FR-11's
existing `RegistryError` core-re-export carve-out — which already permits exactly this pattern for
one of the five types — to the other two of the three DEBT-05 targets.

This ADR *is* the promotion of the Epic 1 decision record into `.planning/decisions/`. It restates
the decision's substance inside the ADR corpus, where it sits at the top of the precedence order by
construction (D-00b), rather than leaving the answer dependent on the Epic 1 record's DOC manifest
type outranking or being outranked by any particular PRD. The source `.project/` decision record is
cited as this ADR's provenance and is **not** re-tagged via `--manifest`: re-typing a `.project/`
file would change how five already-completed ingest runs classified their corpus, for an outcome an
ADR achieves natively without touching ingest state, and the ingest is closed (there is no run 6).

The DEBT-05 target, stated explicitly: the canonical `TokenUsage` is
`crates/paladin-core/src/platform/container/token_usage.rs:13`. The two other shipped
`TokenUsage` structs — `crates/paladin-core/src/platform/container/battalion/mod.rs:497` and
`crates/paladin-llm/src/llm_analysis_service.rs:51` — are duplicates that Phase 8 / DEBT-05
collapses into re-exports of the canonical definition. `PaladinResult` and `StopReason` have no
duplicate copies to collapse; only `TokenUsage` carries the three-way split.

## Considered Options

- Ratify the shipped answer and promote the Epic 1 decision record into an ADR (accepted) — matches
  shipped code, matches the Approved Epic 1 decision record, removes the upward dependency the
  decision was written to remove, and requires no code change in this phase because ADR-0016 sits
  above the Epic 2 PRD by construction rather than by manifest re-tagging.
- Apply Epic 2 FR-10 literally and move `PaladinResult`, `StopReason` and `TokenUsage` into
  `paladin-ports` (rejected) — reintroduces the exact upward dependency the Epic 1 decision was
  written to remove, contradicts every shipped call site, and would require a real code change this
  phase is explicitly forbidden from making.
- Re-tag the Epic 1 decision record via `--manifest` so mechanical precedence alone produces the
  right answer (rejected) — changes how five already-completed ingest runs classified their corpus,
  for an outcome an ADR achieves natively; the ingest is closed and re-running it is out of scope.
- Record the conflict between the Epic 1 decision record and the Epic 2 PRD without picking a side
  (rejected) — leaves Phase 8 / DEBT-05 blocked with no consolidation target, which is exactly the
  state ROADMAP records as this phase's one cross-phase dependency on DEBT-05.

## Code Locations

- `crates/paladin-core/src/platform/container/token_usage.rs:13` — the canonical `TokenUsage`
  definition (`pub struct TokenUsage`).
- `crates/paladin-core/src/platform/container/battalion/mod.rs:497` — a duplicate `TokenUsage`
  struct (`pub struct TokenUsage`); DEBT-05 collapses this into a re-export of the canonical
  definition.
- `crates/paladin-llm/src/llm_analysis_service.rs:51` — a second duplicate `TokenUsage` struct
  (`pub struct TokenUsage`); DEBT-05 collapses this into a re-export of the canonical definition.
- `crates/paladin-core/src/platform/container/execution_result.rs:38` — the `PaladinResult` struct
  definition (`pub struct PaladinResult`).
- `crates/paladin-core/src/platform/container/execution_result.rs:76` — the `StopReason` enum
  definition (`pub enum StopReason`).
- `crates/paladin-core/src/platform/container/registry_error.rs:10` — the `RegistryError` enum
  (`pub enum RegistryError`), the type whose existing re-export carve-out (Epic 2 FR-11) this ADR
  extends to `PaladinResult`, `StopReason` and `TokenUsage`.
- `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs:27` — the `HandoffError`
  enum (`pub enum HandoffError`), settled by the same Epic 1 decision record but not itself a
  DEBT-05 target (no duplicate copy exists).
- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
  — the Approved-but-DOC decision record this ADR promotes; cited as provenance, not re-tagged.
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` FR-7, FR-10
  and FR-11 — the corrected source; FR-7 and FR-10 are annotated by plan 07-02 (this plan) to extend
  FR-11's `RegistryError` carve-out to the three types this ADR settles.

## Code Conformance

must change

Phase 8 / DEBT-05 is the executing requirement. The required change is collapsing the two duplicate
`TokenUsage` definitions (`battalion/mod.rs:497` and `llm_analysis_service.rs:51`) into re-exports of
the canonical `paladin-core` definition (`token_usage.rs:13`). No code changes are made in this
phase — Phase 7 decides which `TokenUsage` is canonical; Phase 8 / DEBT-05 performs the
consolidation.

## Downstream Consumers

- Phase 8 / DEBT-05 — blocked on this ADR by number. Consolidates the two duplicate `TokenUsage`
  copies into re-exports of the canonical `paladin-core` definition this ADR names.
- Plan 07-06 — the ledger rows for `REQ-port-value-type-ownership-v1` and
  `REQ-port-value-type-ownership-v2` cite this ADR: v1 (the shipped `paladin-core` ownership) is
  recorded as the surviving position, and v2 (the Epic 2 FR-10 "not split across crates" reading) is
  recorded `superseded by shipped code`, pointing at this ADR. Neither row is deleted or merged into
  the other.
- Plan 07-13 — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index and
  advances the "Next free ADR number" line past this phase's full 0014-0020 (plus 0021, if D-25a's
  Candidate 2 is promoted) allocation.
