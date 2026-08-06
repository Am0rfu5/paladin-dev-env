# ADR-0020: Build-time benchmark target restated per scenario

## Status

Accepted

**Date:** 2026-08-06

## Context

Milestone 5 SM-7 sets a single target: "≥ 50% faster incremental rebuild for isolated crate changes"
(`prd-workspace-finalization-epic-6.md:260`), measured by `build-benchmarks.md`. FR-3.5 required the
benchmark report to "confirm whether the ≥ 50% incremental rebuild improvement target … was achieved.
If not achieved, the report must recommend follow-up actions" (`prd-workspace-finalization-epic-6.md:160`).

`build-benchmarks.md`'s own Summary Table (`:63-71`) records five scenarios against that single
target: two meet it, three do not. The report's Conclusion (`:101-111`) nevertheless declares
"**Overall verdict: Target achieved.**" (`:111`) — a single pass/fail verdict layered over a table
that itself shows three of five rows failing.

The same document also disagrees with itself on one figure: the Summary Table records the clean-build
improvement as **−6.6%** (`:67`), while the Conclusion paragraph states "The clean build scenario does
not meet the 50% target (**−5%**)" (`:109`) — two different numbers for the same measurement in the
same document.

The report's own methodology note (`:59`) states the monolith baseline's incremental figure (BL-B,
17,302 ms) was produced by touching `src/lib.rs` — "primarily a module-tree re-export file", so
Rust's incremental pipeline "detects no fingerprint changes in downstream modules and can skip the
bulk of recompilation." The note states plainly that this is a **best-case** monolith figure and that
"the comparisons below are *conservative estimates* of the workspace advantage" — a caveat that
qualifies how far the failing rows (B, C) can be read as final, without erasing that they fail the
table's own stated target as measured.

## Decision

**(i) The report's five figures, transcribed verbatim from `build-benchmarks.md`'s Summary Table
(`:65-71`) during this task:**

| Scenario | Monolith median | Workspace median | Improvement | Verdict (≥ 50%) |
| --- | --- | --- | --- | --- |
| Clean build | 275,681 ms | 257,492 ms | −6.6% | ❌ fail |
| Incremental — `paladin-core` change | 17,302 ms (best-case) | 14,029 ms | −18.9% | ❌ fail |
| Incremental — `paladin-llm` adapter change | 17,302 ms (best-case) | 9,583 ms | −44.6% | ❌ fail |
| Incremental — `paladin-memory` adapter change | 17,302 ms (best-case) | 8,618 ms | −50.2% | ✅ pass |
| Incremental — `paladin-battalion` only | 17,302 ms (best-case) | 1,571 ms | −90.9% | ✅ pass |

Two of five scenarios pass the ≥ 50% target as measured; three fail.

**(ii) SM-7 is restated as a per-scenario target**, so each row above can be judged independently
rather than folded into one pass/fail headline. Given the methodology note's best-case-baseline
caveat, the target is most meaningfully judged for the two scenarios that already pass it outright
(memory, battalion) and the clean-build scenario, all three of which are one-time or
whole-workspace measurements unaffected by the caveat's *incremental*-specific concern; the two
`core`/`llm` incremental scenarios that fail the target are the ones the caveat most directly
qualifies, since a realistic (non-root-file) monolith incremental baseline would very likely be
slower than the recorded 17,302 ms best case and could plausibly push those two figures over 50% —
but that is what the report's own recommended follow-up would have to confirm, not something this
ADR asserts as measured.

**(iii) The document's internal −6.6%/−5% inconsistency is resolved by citing the Summary Table as
authoritative and recording the Conclusion's figure as a transcription error**, not by attempting to
reconcile the two as if they measured different things — the table is the primary data artifact; the
Conclusion is prose summarizing it, and prose summarizing a number should reproduce it, not diverge
from it.

**(iv) The "Overall verdict: Target achieved" conclusion is judged against the report's own table,
not reconciled with it** — the same treatment ADR-0006 gave the ~78% Milestone-3 coverage figure and
the Milestone-1 coverage baselines (accepted and noted as contradicted by the current measurement,
not explained away). Three of five rows in the report's own table fail the ≥ 50% target it states;
"Target achieved" as a single headline verdict is contradicted by that table and is recorded as such.

**(v) The re-measurement `build-benchmarks.md` itself recommends is declined, with the reason
recorded rather than passed forward as a task.** The report's own "Recommended follow-up"
(`:107`) proposes strengthening the baseline methodology "by touching a mid-tree implementation file
… rather than just the crate root, to produce a more representative monolith incremental time for
scenarios B and C." This ADR declines that re-measurement: the ≥ 50% figure is a comparison against
the **pre-workspace monolith** at baseline commit `08dc944` (`build-benchmarks.md:20`), and that tree
no longer exists in buildable form on this branch — Milestones 5 and 6 have since decomposed and
relocated the very structure the monolith baseline would need to reproduce. Re-measuring means
resurrecting a historical commit and running full clean and incremental builds for a metric about a
restructuring that completed three milestones ago, in an environment that carries documented offline
and no-Docker constraints (the same constraints that already halted Phase 1's coverage measurement,
per ADR-0006). The decline is deliberate: no re-measurement task is passed forward, so no later phase
inherits an unfundable one.

## Considered Options

- Restate SM-7 per scenario and judge the report against its own table (accepted) — makes each row
  independently falsifiable without requiring a new measurement, and is honest about which rows the
  methodology caveat qualifies and which it does not.
- Re-measure against a mid-tree monolith baseline as the report itself recommends (rejected, with the
  reason recorded above) — the baseline tree no longer exists in buildable form and this environment
  cannot run the full clean/incremental build cycle the re-measurement would require.
- Accept the report's "Target achieved" conclusion as written (rejected) — it is directly contradicted
  by the table immediately above it in the same document.
- Amend `build-benchmarks.md` at source to fix the −6.6%/−5% inconsistency or the conclusion
  (rejected) — the report is a historical measurement record; D-00g's pattern is annotation at the ADR
  level, not rewriting the source. This ADR transcribes and judges it; it does not correct it.

## Code Locations

- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md:63-71` — the Summary Table,
  the five figures transcribed verbatim above.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md:101-111` — the Conclusion
  paragraph, including the `−5%` transcription error (`:109`) and the "Overall verdict: Target
  achieved" line (`:111`).
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md:59` — the methodology note
  explaining the best-case monolith incremental baseline.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md:107` — the recommended
  follow-up this ADR declines.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md:20` — the baseline commit
  reference, `08dc944` (`origin/main` — last pre-decomposition commit).
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md:160` —
  FR-3.5, the report requirement this ADR's restated target answers.
- `.project/Milestone_5-Workspace-Decomposition/Epic_6/prd-workspace-finalization-epic-6.md:260` —
  SM-7, the single target this ADR restates per scenario.

## Code Conformance

conforms

This ADR restates a target and judges a historical measurement already on record; it changes no code
and re-runs no benchmark. `build-benchmarks.md` itself is not edited by this ADR — its figures are
transcribed and judged, per D-00g's annotation-not-rewriting pattern for historical `.project/`
records.

## Downstream Consumers

- **Plan 07-11** — the `REQ-build-benchmark-report` and `REQ-llm-build-validation` ledger rows cite
  this ADR for their per-scenario verdicts.
- **Plan 07-13** — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index.
- **No phase inherits a re-measurement task.** The decline recorded in Decision clause (v) is final
  for this corpus: a future reader who wishes to re-propose the mid-tree measurement will find the
  reason it was declined here, in view, rather than an unowned task drifting forward with no
  record of why it was never picked up.
