# ADR-0018: Milestone 6 facade re-export policy and its version consequence

## Status

Accepted

**Date:** 2026-08-06

## Context

ADR-0008 framed a three-way version disagreement — the `release/v0.7.0` branch name, the
`0.6.0`-pinned manifests, and the untagged `v0.5.1` — and recorded the cross-phase coupling rule
the requirements corpus's own cross-phase-coupling table states: "whether Milestone 6's facade
re-export removal forces a major version bump" is assigned to ARCH-04 (this phase, Phase 7), with
REL-01 (Phase 4) as the applying requirement, and "whichever of Phase 4 / Phase 7 executes first
records the answer, the other applies it." Phase 4 ran first and recorded the version figure as
`0.7.0` in ADR-0008. **ADR-0008 did not touch the re-export policy itself** — only the version
figure that policy's answer feeds into. This ADR settles the policy question ADR-0008 left open.

The disagreement ADR-0008 left standing: the Milestone 6 overview's Epic 2 Acceptance Criterion 6
(`.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md:215`,
"The facade crate re-exports maintain backward compatibility for any types that were publicly
accessible"), Epic 4 Acceptance Criterion 5 (`:434`, "The facade crate re-exports `CircuitBreaker`
at the original path for backward compatibility"), and the risk register's CircuitBreaker row
(`:79`, "Facade crate re-exports absorb the change") all require backward-compatible facade
re-exports at the relocated types' original paths. Both governing PRDs forbid exactly that: Epic 2
Non-Goal 7 (`.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md:192`,
"Adding pub-use re-exports in `src/lib.rs`... No shim re-exports are added") and Epic 4 Goal 7 /
FR-4.11 (`.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md:50-51,239-244`,
"Remove `circuit_breaker` from the `application/use_cases/paladin/mod.rs` module registration (no
re-export left behind — old path is intentionally broken)").

Shipped code follows the PRDs, not the overview. `src/application/` was listed directly during this
task and holds `cli`, `errors`, `mod.rs`, and `services` — **`src/application/use_cases/` does not
exist**, in this tree or in either PRD's target directory (both target `application/services/`
sub-modules by their shipped names: `notification_orchestrator`, `queue_orchestrator`,
`orchestration`, `log_orchestrator`). No facade re-export of `CircuitBreaker` exists at
`paladin::application::use_cases::paladin::circuit_breaker` — `docs/src/api-reference/stable-api.md:618-619`
already records the canonical-path change with no shim.

Epic 2's own Open Question 4 (`:330`) undercuts its own Non-Goal 7: it asks "Should `src/lib.rs` or
`src/prelude.rs` add `pub use` re-exports pointing from old paths to new paths to ease migration for
downstream consumers? The current decision is no re-exports, but this should be confirmed with the
team before implementation begins." That confirmation never happened in any ingested document — this
is an open decision, not a precedence conflict, which is why it is the one ARCH-04 answer three
ledger rows across two milestones (`REQ-battalion-facade-shim`, `REQ-orchestration-no-reexport-shims`,
`REQ-circuitbreaker-old-path-retired`) point at.

## Decision

**(i) The no-shim posture stands as policy.** Relocated types are not re-exported at their
original paths; the old paths (`application::use_cases::*`,
`application::use_cases::paladin::circuit_breaker`) are intentionally retired. This is what both
PRDs specified and what shipped code implements.

**(ii) Epic 2's Open Question 4 is now confirmed by this ADR**, rather than left dangling: no
`pub use` re-exports are added at `src/lib.rs` or `src/prelude.rs` pointing from old paths to new
paths.

**(iii) The version consequence, citing `.planning/decisions/0008-workspace-version-0-7-0.md`
(ADR-0008) by number, not re-derived here:** Milestone 6 removed publicly reachable import paths,
which is a breaking change under ordinary semver reasoning, and pre-1.0 Cargo semantics make
`0.7.0` the correct expression of it — a `0.x` project expresses breaking changes as minor bumps,
not major ones. REL-01's single-version story is therefore unaffected by this ADR and no major bump
is required. This clause cites Phase 4's answer; it does not re-derive it.

**(iv) The Milestone 5 → Milestone 6 posture flip is recorded as history, not as a live
contradiction.** `REQ-battalion-facade-shim` required a re-export shim and was correct for
Milestone 5 (`prd-paladin-*-extraction.md`'s backward-compatible facade re-exports, verified in
place at that milestone). Milestone 6 retired it. The ledger records that row `superseded by
shipped code`, pointing here, not as a contradiction to reopen — Milestone 5's shim requirement was
never wrong for Milestone 5; Milestone 6 changed the posture going forward.

**Amended 2026-08-06, plan 07-13 (original clause retained above; narrowed, not reversed).**
"Milestone 6 retired it" overstates what shipped, and the overstatement is corrected here rather
than left for a later reader to trip over. The literal FR-14 path this clause means,
`src/application/use_cases/battalion/`, is indeed gone — re-confirmed this task via
`test -d src/application/use_cases` (fails). But the shim *mechanism* was not deleted: it survives
today, carried forward under the unrelated Milestone 8 Epic 4 `use_cases` → `services` rename, at
`src/application/services/battalion/mod.rs` — still a thin `pub use paladin_battalion::<module>;`
re-export block whose own header comment calls itself "a thin shim" (re-read this task),
declared in `src/application/services/mod.rs`, and actively consumed by 36 files, re-grepped this
task via `grep -rln 'application::services::battalion' src/ tests/ examples/ crates/`: 5 in `src/`
(`src/prelude.rs`, `src/application/mod.rs`, the shim file itself, and two CLI command modules),
18 in `tests/`, 13 in `examples/`, 0 in `crates/`. First surfaced as a residual finding in plan
07-10's `REQ-battalion-facade-shim` ledger row; amended at source here per that plan's own
deferral (it did not touch `.planning/decisions/*.md`, per its prohibitions).

What this clause should say, stated plainly: Milestone 6 retired the *directory* the FR-14 path
named and the posture it represented going forward (no new shims are added — clause (i) is
unaffected and holds). The *re-export mechanism itself* — a pre-existing, Milestone-5-era artifact
— outlived that directory under a later, unrelated rename and remains live in the shipped tree
today. This does not reopen Non-Goal 7 or FR-4.11's settled scope, both of which name the
orchestration-service and CircuitBreaker relocations specifically, not this battalion re-export;
it narrows one adjacent historical claim in this clause to match what shipped.

## Considered Options

- Ratify the no-shim posture the PRDs and the shipped tree already agree on (accepted) — the
  minority position is the overview alone; two PRDs and the tree agree.
- Reintroduce facade re-exports to satisfy the overview's Epic 2 AC 6 / Epic 4 AC 5 (rejected) — a
  code change this phase is forbidden to make, against a posture two PRDs and the shipped tree
  already settled.
- Leave the overview and the PRDs both live and disambiguate per reader (rejected) — that is the
  exact state ARCH-04 exists to end.
- Re-derive the version consequence independently of ADR-0008 (rejected) — produces a second
  version story that REL-01 would then have to reconcile against ADR-0008's already-recorded
  `0.7.0`.

## Code Locations

- `src/application/mod.rs` — the modules that do exist: `cli` (feature-gated), `errors`,
  `services`; no `use_cases` module declared anywhere in this file.
- `src/application/services/` — the shipped orchestration home (`orchestration`,
  `queue_orchestrator`, `log_orchestrator`, `notification_orchestrator`, `paladin`, `battalion`,
  `arsenal`, `herald`, `content`, `sanctum`), listed directly during this task.
- `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md:215`
  (Epic 2 AC 6) and `:434` (Epic 4 AC 5) — the overview's two acceptance criteria requiring facade
  re-exports, annotated superseded by plan 07-05 Task 2.
- `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md:79`
  — the risk register's "Facade crate re-exports absorb the change" row, annotated superseded by
  plan 07-05 Task 2.
- `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md:192`
  — Non-Goal 7, "no shim re-exports are added."
- `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md:330`
  — Open Question 4, now confirmed by this ADR.
- `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md:50-51`
  — Goal 7, "no re-export left behind — old path is intentionally broken."
- `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md:239-244`
  — FR-4.11, "Remove Old Module Registration."
- `.planning/decisions/0008-workspace-version-0-7-0.md` — the cited version answer this ADR does
  not re-derive.

## Code Conformance

conforms

Shipped code already implements the no-shim posture — `src/application/use_cases/` does not exist
and no `CircuitBreaker` re-export exists at its retired path. No code change results from this ADR;
the executing work is the source annotations in Task 2 (the Milestone 6 overview) and Task 3 (the
Epic 4 CircuitBreaker PRD) of plan 07-05.

## Downstream Consumers

- **Phase 11 / FACADE-02 D1** — ROADMAP.md records Phase 11 as loosely dependent on ARCH-04 for
  exactly this: D1's re-export question is answered here, and Phase 11 applies this ADR's no-shim
  posture rather than re-opening it.
- **Plans 07-08 and 07-10** — the ledger rows for `REQ-orchestration-no-reexport-shims`,
  `REQ-circuitbreaker-old-path-retired` (07-08) and `REQ-battalion-facade-shim` (07-10) cite this
  ADR for their verdicts.
- **Phase 4 / REL-01** — the single-version convergence story is unaffected; this ADR's version
  clause cites ADR-0008 rather than adding a second figure for REL-01 to reconcile.
- **Plan 07-13** — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index and
  advances the "Next free ADR number" line past 0018.
