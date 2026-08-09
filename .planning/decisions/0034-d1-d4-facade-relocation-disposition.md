# ADR-0034: D1–D4 facade relocation disposition — four verdicts with owners

## Status

Accepted

**Date:** 2026-08-08

## Context

`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` records four items —
D1 through D4 — each with an effort/risk rating and a recommendation, and **no owner and no
target milestone for any of them**:

- **D1** (`deferred-items.md:56-68`) — `src/core/` re-export shims, currently "KEEP, by
  decision." Rated "medium churn / low risk (mechanical, compiler-checked)" at `:68`.
- **D2** (`deferred-items.md:70-83`) — `src/core/platform/manager/` services
  (`content_service.rs`, `event_manager.rs`, `user_service.rs`) are mis-layered. Rated
  "medium / medium (touches consumers across the facade + tests)" at `:83`.
- **D3** (`deferred-items.md:85-97`) — entangled Paladin use-case services, currently "KEEP
  for now." Rated "high / high" at `:97`.
- **D4** (`deferred-items.md:99-108`) — `content_ingestion_service.rs` placement. Rated
  "medium / medium" at `:108`.

A rating with a recommendation is not a decision: nobody owns any of the four, and nothing
names when or under what condition a "keep for now" stops applying. This is the defect
FACADE-02 exists to close — "a decision rather than a rating."

**Re-measured this session, verbatim:**

```
$ find src/core -name "*.rs" | sort
src/core/mod.rs
src/core/platform/manager/content_service.rs
src/core/platform/manager/event_manager.rs
src/core/platform/manager/mod.rs
src/core/platform/manager/user_service.rs
src/core/platform/mod.rs
$ find src/core -name "*.rs" | wc -l
6
```

```
$ grep -rl "crate::core::" src/ | wc -l
49
```

```
$ ls src/core/platform/manager/
content_service.rs  event_manager.rs  mod.rs  user_service.rs
```

All three figures match `deferred-items.md`'s own count and `11-CONTEXT.md`'s D-05 figures
exactly — no drift.

This ADR is tested against two governing ADRs. **ADR-0031** (`.planning/decisions/
0031-extracted-crate-dependency-rule.md`) restates the invariant that legalises D3's and D4's
proposed leaf-to-leaf relocation targets: no extracted crate may depend on another extracted
crate or on the facade in its default build; a non-default optional feature may declare such
an edge only where the facade opts in explicitly and the dependent code is `cfg`-gated.
**ADR-0028** (`.planning/decisions/0028-m8-reconciliation-authoritative.md`) `## Decision
(iii)` records the Epic 3 relocations as already executed inside Milestone 8 — 15 commits, net
10,252 LOC removed, range `e5b2011~1..a1e4901`.

**ADR-0031 was authored by Phase 10 under `--auto` and is one of two decisions Phase 10 flagged
`⚠ HUMAN REVIEW`; it has never been human-ratified.** This ADR is safe to write against it
because every verdict below is *defer* or *withdraw* — D-04 defers every relocation in this
phase, so no code depends on ADR-0031's precise form here. Any future phase that actually
executes a D3 or D4 edge should confirm ADR-0031 with a human first, rather than treating its
citation here as settled authority. Carrying this status forward, not laundering it into
precedence, is itself part of what this ADR records.

## Decision

**Every item in D1–D4 resolves to a verb — *do*, *defer with a stated trigger*, or *withdraw*
— plus a named owner, and no relocation executes in this phase.** A rating with a
recommendation and no owner is not a decision; the four sub-decisions below each supply the
verb, the owner, and, where deferred, the concrete trigger the register itself is missing.

**(i) D1 — `src/core/` re-export shims: DEFER. Trigger: a facade-wide no-alias sweep. Owner:
unassigned-pending-that-sweep — named here as unassigned rather than left blank.** The measured
blast radius: `src/core/` is exactly six files, and 49 facade files import through
`crate::core::…`. Removal is not a pure path rewrite: `src/core/platform/mod.rs` carries
maneuver/parser path injection, which is real logic rather than re-exports, and any sweep must
preserve it rather than delete it along with the shim layer.

**ADR-0018 does not settle D1.** `.planning/decisions/0018-m6-facade-reexport-policy.md`
ratified the no-shim posture for *relocated types* — `application::use_cases::*` and
`CircuitBreaker` — where the old import path was intentionally retired after the type moved.
`src/core/`'s re-export layer is a different construct: it is not a retired path left behind
after a relocation, it is a **surviving re-export layer that still fronts live code** for 49
importers today. D1 therefore does not follow automatically from ARCH-04 (ADR-0018's posture),
and a reader who assumes it does will mis-scope the sweep — the sweep D1 defers to is a new
piece of work, not an application of ADR-0018's already-settled rule.

**(ii) D2 — mis-layered `src/core/platform/manager/` services: three files, three verdicts.**

- **`user_service.rs`: the split is WITHDRAWN.** Three facts narrow it to nothing on this
  axis. Reconciliation commit `6704807` ("chore(facade): delete orphaned user_controller
  duplicate (M8 Phase 3)") already found "no split was needed" for the controller case — its
  own commit message states plainly: "No split was needed — `UserServiceTrait` + DTOs already
  live in `paladin-core` and the controller depends only on `Arc<dyn UserServiceTrait>`." Their
  location is confirmed again this session: `grep -rl "UserServiceTrait" crates/paladin-core/`
  → `crates/paladin-core/src/platform/manager/user_service.rs`. Second, the *full*
  `user_service.rs` relocation (583 lines, verified via `wc -l
  src/core/platform/manager/user_service.rs`) is already carried as a **run-3 v2 tech-debt
  item** — a second, independent owner already exists for the larger move. Third, Deferred-QA
  Epic 28 (**DEFER-02**, Phase 15) plans to **test** the same file to ≥ 80% coverage, sizing its
  estimate and mock set against the file as it ships today. Splitting first would change that
  estimate and mock set out from under Phase 15. Withdrawing the split resolves the collision
  REQUIREMENTS.md flags ("do not schedule independently") rather than sequencing it.

  **The resulting three-owner split, stated in one place:** the split itself is owned by
  **nobody** — it is withdrawn, not deferred, so there is no future trigger that revives it;
  the full `user_service.rs` relocation is owned by the **run-3 v2 tech-debt item**; the tests
  are owned by **DEFER-02 / Phase 15**. This is what ROADMAP criterion 2's "nothing in that set
  is planned twice" requires: three distinct owners, no overlap, and Phase 15 is free to size
  DEFER-02 against the unsplit file.

- **`content_service.rs` (`ContentItemService`, 385 lines) and `event_manager.rs`
  (`EventService`, 345 lines): each gets its own independent verdict — DEFER, not folded into
  `user_service.rs`'s withdrawal.**
  - `content_service.rs`: the Epic 1 audit's recommendation targets `paladin-core` — a pure
    domain service moving to the base crate every extracted crate is already permitted to
    depend on. This is not a leaf-to-leaf edge, so ADR-0031's per-edge test does not gate it;
    nothing in ADR-0028's `e5b2011~1..a1e4901` range already moved this file (that range's
    relocations are adapter-layer: garrison, sanctum, storage, notifications, herald — not this
    manager service). **Verdict: DEFER. Trigger: the same architecture-pass milestone
    `deferred-items.md`'s own `## Suggested grouping` names for D2. Owner: unassigned-pending
    that milestone.**
  - `event_manager.rs`: the Epic 1 audit's recommendation is two-valued — `paladin-core` *or* a
    facade app-service — and that choice is itself unresolved. Not a leaf-to-leaf edge either
    way, and not touched by ADR-0028's executed range. **Verdict: DEFER. Trigger: the same
    architecture-pass milestone, which must also resolve the `paladin-core`-vs-facade-app-service
    choice as part of that pass. Owner: unassigned-pending that milestone.**

**(iii) D3 — entangled Paladin services: DEFER. Trigger: the broader builder/execution refactor
the register itself names. Owner: unassigned-pending that refactor — recorded, not left blank.**
The four files are `src/application/services/paladin/{planning_service,
prompt_generation_service,temperature_service,handoff_service}.rs` — re-measured this session
at 1,008 + 477 + 654 + 610 = **2,749 lines** (register's "~2,750 LOC" confirmed), tightly
coupled to `paladin_builder.rs` (86,276 bytes) and `paladin_execution_service.rs` (104,819
bytes).

**HARD-05 is answered** — ADR-0031 restated the rule, so D3's `paladin-battalion`
(planning/handoff) and `paladin-llm` (prompt/temperature) targets are legal on exactly the same
terms `paladin-content`'s existing `llm` feature already satisfies (non-default optional
feature, `cfg`-gated consumer, explicit facade opt-in — ADR-0031 `## Context`). The remaining
question is per-edge, not categorical: **is each proposed edge non-default, facade-gated and
`cfg`-scoped?** That question is the thing the builder/execution refactor trigger must answer
when it runs; it is not answered here, and D3's verdict does not depend on HARD-05 being
unanswered — it never was the blocker.

**(iv) D4 — `content_ingestion_service.rs` placement: DEFER. Trigger: the dependency-coupling
review the register already names as the precondition. Owner: unassigned-pending that review —
recorded, not left blank.** The file is `src/application/services/content/
content_ingestion_service.rs`, re-measured this session at **1,211 lines** (register's
"~1,211 LOC" confirmed exact). M7 Epic 1's extraction PRD listed it as moving to
`paladin-content`; the facade kept its own copy. Legal under ADR-0031 on the same terms as D3 —
`paladin-content` is an existing leaf crate with a precedent non-default `llm` feature already
satisfying the invariant. The dependency-coupling review is the trigger and is not performed in
this ADR.

**This ADR records verdicts, not moves.** Nothing in D1–D4 executes in Phase 11 (D-04), and no
item's verdict re-plans a relocation inside ADR-0028's already-executed `e5b2011~1..a1e4901`
range (D-00j) — none of D1–D4's four subjects fall inside that range; all four are still
present in the tree today, confirmed by the file-existence measurements above.

## Considered Options

- **Four verdicts, each with a verb and a named owner, and no relocation executes this phase**
  (accepted) — satisfies the ROADMAP goal of "a decision rather than a rating" without
  expanding this phase's scope into architecture work.
- **Execute the cheap items now (e.g. D1's mechanical path rewrite)** (rejected) — a disposition
  phase that also refactors has two jobs and a much larger blast radius; D-04 explicitly scopes
  this phase to recording verdicts only.
- **A uniform "defer all, revisit later"** (rejected) — reads as a rating by another name (no
  named trigger, no named owner) and does not close the ROADMAP goal FACADE-02 exists to
  satisfy.
- **Withdraw all of D2, not only the `user_service.rs` split** (rejected) — `content_service.rs`
  and `event_manager.rs` are genuine mis-layering items with no collision comparable to
  `user_service.rs`'s three-fact narrowing; D-06 requires each to get "their own verdicts",
  plural, and withdrawing them wholesale would silently drop two real items from the register.
- **Sequence the `user_service.rs` split ahead of DEFER-02 instead of withdrawing it** (rejected)
  — that preserves the collision REQUIREMENTS.md flags rather than removing it, and leaves
  Phase 15 sizing DEFER-02's estimate and mock set against a file that might still change shape
  before Phase 15 runs.
- **Treat D1 as already answered by ADR-0018 (ARCH-04)** (rejected) — ADR-0018 settled the
  no-shim posture for *relocated types* whose old paths were retired; `src/core/`'s re-export
  layer is a different, still-live construct (D-05), and treating the two as the same question
  mis-scopes any future no-alias sweep.

## Code Locations

- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md:56-68` — D1's clause,
  effort/risk rating at `:68`.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md:70-83` — D2's clause,
  effort/risk rating at `:83`.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md:85-97` — D3's clause,
  effort/risk rating at `:97`.
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md:99-108` — D4's clause,
  effort/risk rating at `:108`.
- `src/core/platform/mod.rs` — carries the maneuver/parser path injection D1's sweep must
  preserve; it is real logic, not a re-export.
- `src/core/platform/manager/content_service.rs` (385 lines), `event_manager.rs` (345 lines),
  `user_service.rs` (583 lines) — the three D2 files, line counts measured this session via
  `wc -l`.
- `src/application/services/paladin/{planning_service.rs (1,008),
  prompt_generation_service.rs (477), temperature_service.rs (654), handoff_service.rs (610)}` —
  the four D3 files, 2,749 lines total, measured this session via `wc -l`.
- `src/application/services/content/content_ingestion_service.rs` (1,211 lines) — the D4 file,
  measured this session via `wc -l`.
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — the restated default-build
  invariant D3's and D4's targets are tested against; `## Downstream Consumers` names Phase 11 /
  FACADE-02 explicitly.
- `.planning/decisions/0028-m8-reconciliation-authoritative.md` `## Decision (iii)` — the
  `e5b2011~1..a1e4901` executed range this ADR confirms none of D1–D4 falls inside.
- `.planning/decisions/0018-m6-facade-reexport-policy.md` — the no-shim posture for *relocated
  types*, which does not settle D1 (see `## Decision (i)` above, D-05).
- Commit `6704807` (`chore(facade): delete orphaned user_controller duplicate (M8 Phase 3)`) —
  the "no split was needed" finding narrowing D2's `user_service.rs` axis.
- `crates/paladin-core/src/platform/manager/user_service.rs` — `UserServiceTrait`'s confirmed
  location, re-verified this session via `grep -rl "UserServiceTrait" crates/paladin-core/`.
- Re-run this session, verbatim:
  - `find src/core -name "*.rs" | wc -l` → `6`
  - `grep -rl "crate::core::" src/ | wc -l` → `49`
  - `ls src/core/platform/manager/` → `content_service.rs  event_manager.rs  mod.rs
    user_service.rs`

## Code Conformance

conforms

Every verdict above is *defer* or *withdraw*, and D-04 explicitly forbids any relocation from
executing in this phase, so the tree already satisfies this ADR exactly as it stands today.
This ADR instructs no code change (D-13).

## Downstream Consumers

- **Phase 15 / DEFER-02** — inherits the withdrawn `user_service.rs` split and sizes its
  ≥ 80%-coverage estimate and mock set against the unsplit file, as shipped, rather than a
  moving target.
- **The run-3 v2 tech-debt item** — inherits ownership of the full `user_service.rs`
  relocation, independent of and unaffected by the split's withdrawal.
- **The phase that runs the facade-wide no-alias sweep** — inherits D1's verdict, the
  49-importer measurement, and the instruction to preserve `src/core/platform/mod.rs`'s
  maneuver/parser path injection rather than delete it with the shim layer.
- **The phase that runs the architecture-pass milestone `deferred-items.md`'s own `##
  Suggested grouping` names for D2** — inherits `content_service.rs`'s and `event_manager.rs`'s
  deferred verdicts, including `event_manager.rs`'s still-open `paladin-core`-vs-facade-app-
  service choice.
- **Any future phase executing a D3 or D4 edge** — inherits the per-edge ADR-0031 test (is the
  edge non-default, facade-gated and `cfg`-scoped?) *and* the instruction to human-confirm
  ADR-0031 first, since it was authored under Phase 10 `--auto` and flagged `⚠ HUMAN REVIEW`
  (D-00m).
- **Plan 11-05** — amends the `REQ-m8-deferred-items-register` ledger row against this ADR.
