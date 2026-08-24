# ADR-0047: `docs/src/appendix/design-and-architecture.md` disposition — archived, Sentinel re-anchored, diagram clause withdrawn

## Status

Accepted

**Date:** 2026-08-24

## Context

DOCS-02 requires `docs/src/appendix/design-and-architecture.md` to get a recorded disposition —
archive material, or live deliverable — because it cannot stay both. This ADR carries all three of
D-04's sub-decisions in one file, kept visually distinct because they have different dispositions
and must not be merged into one narrative (per ADR-0033's Finding-N precedent).

**The stale premise, restated rather than dropped (ADR-0022's Context pattern).** Milestone 11
Epic 2 relocated `docs/src/appendix/design-and-architecture.md` from the book's main body into the
appendix as an escape hatch. Milestone 11 Epic 3's non-goals then exempted the appendix from the
rewrite sweep that touched everything else. The mechanism, not just the outcome, is the finding:
relocation plus a rewrite exemption is what put the corpus's largest documentation gap into the one
chapter nobody was required to fix.

Re-measured this session: `wc -l docs/src/appendix/design-and-architecture.md` returns **311**.
`grep -ricw -e commander -e council -e conclave -e grove -e maneuver -e sanctum -e sentinel
docs/src/appendix/design-and-architecture.md` returns **0** for every one of the seven named
subsystems. `grep -c '```mermaid' docs/src/appendix/design-and-architecture.md` returns **0**. This
is the identical 311-line figure the February 2026 PRD cites as the file's *pre-rewrite* state — the
file has not moved since.

**Finding 1 — Archive.** `docs/src/appendix/design-and-architecture.md` is recorded historical,
superseded by `docs/src/architecture/` plus `docs/src/appendix/sentinel.md`. FR-26.1 stops being
tracked against this file. This is a contested position — a reasonable reader could argue the 311
lines should instead be rewritten in place — so per D-00g it gets an ADR rather than a silent ledger
row.

**Finding 2 — Metric re-anchoring.** FR-26.1's recorded success metric, "components documented 8 of
15+ → 15+ of 15+", is re-anchored to the live chapter. Measured this session:
`docs/src/architecture/` (5 pages, 1,216 lines) already covers Commander, Sanctum, Maneuver,
Council, Conclave and Grove — six of the seven subsystems the appendix has zero mentions of — and
`docs/src/appendix/sentinel.md` (976 lines) covers Sentinel, but only in the appendix, not in the
live chapter. Against the 19 shipped ubiquitous-language components (Paladin, Battalion, Formation,
Phalanx, Campaign, Chain of Command, Commander, Garrison, Arsenal, Armament, Citadel, Herald, Quest,
Sanctum, Conclave, Council, Grove, Maneuver, Sentinel), the live chapter covers **18 of 19** before
this ADR's companion task runs; only **Sentinel** is absent from it. The metric re-anchors from
FR-26.1's original 8-of-15+ framing to **18 of 19 → 19 of 19**, closed by Task 2 of this plan giving
Sentinel a home in `docs/src/architecture/overview.md`. The re-derivable command is:
`grep -ric --include=*.md -w sentinel docs/src/architecture/ | awk -F: '{s+=$2} END {print s}'` —
**0** before Task 2, **1 or more** after (recorded with exact before/after output in
`16-06-SUMMARY.md` per D-00e).

**Finding 3 — Diagram-clause withdrawal.** FR-26.1's four-Mermaid-diagram clause named: overall
hexagonal system architecture; Battalion orchestration patterns; data flow through a Paladin
execution cycle; Arsenal/MCP tool integration flow. Each of the six SVGs in `docs/src/assets/` was
read for its rendered text content this session (`grep -oE '<text[^>]*>[^<]+</text>'`), not assumed
from filename:

- `ArchitectureOverview.svg` and `LayerArchitecture.svg` both render a four-tier layer diagram
  (Infrastructure / Application / Platform / Domain Core) with port arrows between tiers — the same
  hexagonal ports-and-adapters topology the live chapter's Three-Layer diagram still uses, even
  though the named services inside each tier (`Aggregator`, `Summarizer`, `TensorFlow`) are the
  pre-agent content-pipeline system, not Paladin/Battalion/Garrison/Arsenal.
- `ComponentInteractionFlow.svg`, `ContentProcessingPipeline.svg` and `data-flow.svg` all render the
  same four-stage **content** pipeline (Ingestion → Validation → Processing → Storage,
  Aggregator/Filter/Summarizer/ML Analyzer/NLP Analyzer) — `data-flow.svg` and
  `ContentProcessingPipeline.svg` are text-content-identical, confirmed by diffing their extracted
  `<text>` elements this session. None of the three mentions Paladin, Battalion, Arsenal, MCP,
  Garrison, or Citadel anywhere in their rendered text.
- `DeploymentArchitecture.svg` renders a Kubernetes/service-mesh deployment topology
  (API Gateway, Content Service, ML Processor, MySQL Cluster) — again the pre-agent system, and not
  named by any of FR-26.1's four clauses.
- `docs/src/architecture/crate-map.md:23`'s existing ```mermaid block renders the current, accurate,
  code-verified crate dependency graph — the one artifact in this inventory that reflects the
  *current* tree rather than the pre-agent one.

Mapped against the four named diagrams:

1. **Overall hexagonal system architecture — answered.** `ArchitectureOverview.svg` and
   `LayerArchitecture.svg` depict the layer/port topology (stale service names, current ring
   structure); `docs/src/architecture/crate-map.md:23`'s mermaid dependency graph supplies the
   currently-correct expression of the same invariant — dependencies flow inward only.
2. **Battalion orchestration patterns — withdrawn, genuinely unanswered.** None of the six SVGs nor
   `crate-map.md:23`'s mermaid block mentions Formation, Phalanx, Campaign, Chain of Command,
   Conclave, Council, Grove or Maneuver anywhere in their rendered text — Battalion did not exist as
   a concept when these content-pipeline-era diagrams were authored. Stretching
   `ComponentInteractionFlow.svg`'s orchestration legend (`Scheduler`, `Queue Manager`, `Event
   Manager` — the pre-agent job-scheduling system) to stand in for Battalion's eight patterns would
   misrepresent a diagram of a different subsystem; per D-03, this diagram is withdrawn instead. The
   live chapter already covers the eight patterns textually: `docs/src/architecture/overview.md`'s
   pattern table (lines 189-200).
3. **Data flow through a Paladin execution cycle — withdrawn, genuinely unanswered.** `data-flow.svg`
   and `ContentProcessingPipeline.svg` depict the pre-agent four-stage **content** ingestion pipeline,
   not the Paladin LLM reasoning loop (`Idle` → `Running` → tool-call / stop-word / max-loops) that
   `docs/src/architecture/overview.md` already draws in ASCII at lines 162-178. Withdrawn rather than
   stretched, per D-03's explicit instruction not to author diagrams into the file being archived and
   not to force an SVG onto a diagram it does not depict.
4. **Arsenal/MCP tool integration flow — withdrawn, genuinely unanswered.** No SVG and no mermaid
   block mentions Arsenal, MCP, STDIO or SSE anywhere in their rendered text.
   `ComponentInteractionFlow.svg`'s "Content Port" is the pre-agent content-delivery port, not
   `ArsenalPort`. The live chapter already covers Arsenal textually:
   `docs/src/architecture/overview.md` lines 216-222.

Per D-03, **no diagram is authored into the file being archived** — three of the four clauses are
withdrawn rather than answered by a stretch, and the fourth is answered by artifacts that already
exist.

## Decision

**Finding 1 (Archive).** `docs/src/appendix/design-and-architecture.md` is recorded historical
material, superseded by `docs/src/architecture/` (the live chapter) and
`docs/src/appendix/sentinel.md` (the Sentinel reference). FR-26.1 stops being tracked against this
file. The 311 lines are retained — archiving records a disposition, it does not destroy the record
(D-00d). Task 2 of this plan prepends a banner naming both live targets and this ADR; the file is
neither deleted, truncated, nor removed from `docs/src/SUMMARY.md`.

**Finding 2 (Metric re-anchoring).** FR-26.1's success metric is re-anchored, explicitly, from its
original "8 of 15+ → 15+ of 15+" framing to **18 of 19 → 19 of 19** against
`docs/src/architecture/`, with Sentinel named as the one component the live chapter was missing
before Task 2 of this plan. The metric closes by measurement (D-02, D-00e), not by assertion — the
re-derivable command is given above in Context, Finding 2.

**Finding 3 (Diagram-clause withdrawal).** FR-26.1's four-Mermaid-diagram clause is withdrawn for
three of its four named diagrams (Battalion orchestration patterns; data flow through a Paladin
execution cycle; Arsenal/MCP tool integration flow) with the reason recorded above — no existing
artifact in `docs/src/assets/` or `docs/src/architecture/crate-map.md` depicts them, and forcing one
to would misrepresent it. The fourth (overall hexagonal system architecture) is answered by existing
artifacts, mapped above. No new diagram is authored into
`docs/src/appendix/design-and-architecture.md` or anywhere else by this ADR or its companion task.

**Re-instatement, written as an instruction, not a mechanism (ADR-0022's exact pattern).** Any future
ADR that wants the withdrawn diagrams (Battalion patterns, the Paladin execution cycle, Arsenal/MCP
integration flow) authored into the live chapter, or that wants
`docs/src/appendix/design-and-architecture.md` treated as a live deliverable again rather than
archive material, must explicitly **supersede** this ADR per `PROMOTION.md`'s supersession
mechanism — this ADR's `## Status` becomes `Superseded` with a pointer, and the superseding ADR
carries a `## Supersedes` line naming `0047`. Nothing here mechanises that re-instatement; it is a
future decision, not a scheduled one.

## Considered Options

- **Archive with a signpost banner, re-anchor the metric, withdraw three of four diagram clauses**
  (accepted) — matches the measured state: the seven subsystems are not undocumented, they are
  undocumented in one relocated pre-rewrite artifact (M-03); rebuilding the appendix would create a
  second architecture document competing with the live chapter, the exact duplication that produced
  this gap; and the four diagrams' existing-artifact inventory genuinely answers only one of the
  four, so the other three are withdrawn rather than force-mapped.
- **Rewrite the 311-line appendix in place to cover all seven subsystems and author all four Mermaid
  diagrams into it** (rejected) — would create the second competing architecture document `D-01`'s
  rationale explicitly rejects, and would violate D-03's prohibition on authoring diagrams into a
  file being archived (this option only makes sense if the file is *not* archived, which contradicts
  DOCS-02's own "cannot stay both" framing).
- **Leave the appendix's disposition unstated and merge Sentinel content into it instead of the live
  chapter** (rejected) — DOCS-02 explicitly forbids the third state of "neither archived nor kept
  current," and merging Sentinel into the appendix rather than the live chapter would leave the
  live-chapter metric at 18 of 19 permanently, never closing FR-26.1's re-anchored success metric.
- **Force-map all four diagram clauses to the nearest available SVG regardless of content fit**
  (rejected) — the six SVGs' extracted text content shows three of them depict the pre-agent
  content-processing pipeline with no mention of Battalion, Paladin, Arsenal, or MCP; stretching them
  to answer diagrams they do not depict is the exact failure mode D-03 names and this ADR's own
  action text forbids ("do not stretch an SVG to cover a diagram it does not depict").

## Code Locations

- `docs/src/appendix/design-and-architecture.md` — 311 lines, the file this ADR dispositions; 0
  mentions of Commander/Council/Conclave/Grove/Maneuver/Sanctum/Sentinel, 0 mermaid blocks (re-run
  this session).
- `docs/src/architecture/overview.md`, `docs/src/architecture/hexagonal-design.md`,
  `docs/src/architecture/domain-model.md`, `docs/src/architecture/design-patterns.md`,
  `docs/src/architecture/crate-map.md` — the five live chapter pages (1,216 lines total) this ADR
  re-anchors FR-26.1's metric to.
- `docs/src/appendix/sentinel.md` — 976 lines, the existing Sentinel reference this ADR names as a
  signpost target and Task 2 cross-links from the live chapter.
- `docs/src/assets/ArchitectureOverview.svg`, `docs/src/assets/LayerArchitecture.svg`,
  `docs/src/assets/ComponentInteractionFlow.svg`, `docs/src/assets/ContentProcessingPipeline.svg`,
  `docs/src/assets/DeploymentArchitecture.svg`, `docs/src/assets/data-flow.svg` — the six existing
  SVGs whose rendered text content this ADR's Finding 3 maps against the four named diagrams.
- `docs/src/architecture/crate-map.md:23` — the existing ```mermaid dependency-graph block, the one
  artifact reflecting the current tree rather than the pre-agent one.
- `docs/src/SUMMARY.md:100` — the appendix's table-of-contents entry, retained per Finding 1 and
  amended with an `(Archived)` qualifier by Task 2, following the `Contributing (Legacy)` precedent
  at `docs/src/SUMMARY.md:109`.
- `.planning/decisions/0022-deprecation-requirement-withdrawal.md` — the structural and rhetorical
  pattern this ADR follows (restate the stale premise, write re-instatement as an instruction).
- `.planning/decisions/0033-cargo-doc-warning-bar.md` — the Finding-1/Finding-2/Finding-3 structure
  this ADR reuses to keep three differently-dispositioned sub-decisions visually distinct.

## Code Conformance

must change

**This phase itself (Phase 16, plan 16-06) is the named executor.** Task 2 of this same plan
performs the `must change` work this ADR records: prepending the archive banner to
`docs/src/appendix/design-and-architecture.md`, adding a Sentinel section to
`docs/src/architecture/overview.md`, and appending the `(Archived)` qualifier to the
`docs/src/SUMMARY.md:100` link label. No Rust source is touched — this ADR's scope is documentation
disposition only.

## Downstream Consumers

- Task 2 of this plan (16-06) — performs the banner, the Sentinel section, and the `SUMMARY.md`
  qualifier this ADR's `must change` verdict requires, in the same phase.
- `.planning/decisions/PROMOTION.md` — indexes this ADR at `0047` and advances its next-free line to
  `0048` in the same commit as this file (T-16-13's mitigation).
- Any future phase auditing `docs/src/architecture/` for currency (explicitly out of this phase's
  scope per `16-CONTEXT.md`'s Deferred Ideas) — inherits the 19-of-19 baseline this ADR establishes
  as its starting point.
- Any future ADR wanting to re-instate the appendix as a live deliverable, or to author the three
  withdrawn diagrams into the live chapter — must explicitly supersede this ADR per `PROMOTION.md`'s
  supersession mechanism.
