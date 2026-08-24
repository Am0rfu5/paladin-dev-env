---
phase: 16-documentation-currency-the-architecture-gap
plan: 06
subsystem: docs
tags: [mdbook, adr, architecture-docs, sentinel, documentation-currency]

# Dependency graph
requires:
  - phase: 16-documentation-currency-the-architecture-gap
    provides: "16-01's D-00a ADR-numbering confirmation (next free number 0047) and DOCS-02 decisions"
provides:
  - "ADR-0047, recording all three D-04 sub-decisions (archive, metric re-anchoring, diagram-clause withdrawal) in one file"
  - "docs/src/appendix/design-and-architecture.md carries an archive banner naming both live signpost targets"
  - "docs/src/architecture/overview.md documents Sentinel, closing the 19-of-19 metric by measurement"
  - ".planning/decisions/PROMOTION.md indexed at 0047, next-free line advanced to 0048"
affects: [docs, architecture-chapter, adr-corpus]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR-0033's Finding-1/Finding-2/Finding-3 structure reused for a second multi-sub-decision ADR"
    - "SVG rendered-text extraction (grep -oE '<text[^>]*>[^<]+</text>') used to verify diagram content before mapping, rather than trusting filenames"

key-files:
  created:
    - .planning/decisions/0047-architecture-appendix-disposition.md
  modified:
    - .planning/decisions/PROMOTION.md
    - docs/src/appendix/design-and-architecture.md
    - docs/src/architecture/overview.md
    - docs/src/SUMMARY.md

key-decisions:
  - "ADR-0047 carries all three D-04 sub-decisions (archive, metric re-anchor, diagram-clause withdrawal) as three visually distinct Finding-N sections, following ADR-0033's precedent rather than ADR-0022's single-narrative shape"
  - "Of the four Mermaid diagram clauses, only 'overall hexagonal system architecture' is answered by existing artifacts (ArchitectureOverview.svg, LayerArchitecture.svg, crate-map.md:23); the other three (Battalion orchestration patterns, Paladin execution cycle data flow, Arsenal/MCP tool integration flow) are withdrawn as genuinely unanswered — none of the six SVGs' rendered text mentions Battalion, Paladin's reasoning loop, Arsenal, or MCP, since all six depict the pre-agent content-processing pipeline"
  - "Sentinel's live-chapter section is a summary-plus-cross-link to the existing 976-line docs/src/appendix/sentinel.md, matching the treatment Garrison/Sanctum/Arsenal already get on the same page, rather than duplicating Sentinel's full content into the live chapter"
  - "ADR-0047's ADR-0047 citation in the archive banner uses backtick-quoted plain text, not a markdown hyperlink, matching every other ADR citation already in docs/src/ (branch-protection.md, testing-guide.md, docker.md, stable-api.md) — a relative link crossing out of the mdbook src/ tree risked linkcheck friction the existing convention already avoids"

requirements-completed: [DOCS-02]

coverage:
  - id: D1
    description: "ADR-0047 authored carrying all three D-04 sub-decisions in the ADR-0022/ADR-0033 shape, with a real SVG-to-diagram mapping and re-instatement written as an instruction"
    requirement: "DOCS-02"
    verification:
      - kind: other
        ref: "test $(ls .planning/decisions/0047-*.md | wc -l) = 1 && test $(grep -c '^## \\(Status\\|Context\\|Decision\\|Considered Options\\|Code Locations\\|Code Conformance\\|Downstream Consumers\\)$' .planning/decisions/0047-*.md) = 7 && grep -q '19 of 19' .planning/decisions/0047-*.md && grep -qi supersede .planning/decisions/0047-*.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "PROMOTION.md indexes 0047 and advances its next-free line to 0048, in the same commit as the ADR file"
    requirement: "DOCS-02"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0048' .planning/decisions/PROMOTION.md && grep -c '^| 0047 ' .planning/decisions/PROMOTION.md; git show --stat d5585e7"
        status: pass
    human_judgment: false
  - id: D3
    description: "docs/src/appendix/design-and-architecture.md carries an archive banner naming both docs/src/architecture/ and docs/src/appendix/sentinel.md, citing ADR-0047; 311 lines retained, diff additions-only, SUMMARY.md TOC entry retained with an (Archived) qualifier"
    requirement: "DOCS-02"
    verification:
      - kind: other
        ref: "wc -l docs/src/appendix/design-and-architecture.md (319, was 311); git diff docs/src/appendix/design-and-architecture.md shows additions only; grep -c 'appendix/design-and-architecture.md' docs/src/SUMMARY.md = 1"
        status: pass
    human_judgment: false
  - id: D4
    description: "Sentinel documented in docs/src/architecture/overview.md, closing the recorded metric at 19 of 19 by a re-derivable whole-word case-insensitive measurement"
    requirement: "DOCS-02"
    verification:
      - kind: other
        ref: "grep -ric --include=*.md -w sentinel docs/src/architecture/ | awk -F: '{s+=$2} END {print s}' — 0 before, 3 after"
        status: pass
    human_judgment: false
  - id: D5
    description: "mdbook build docs/ exits 0 with the linkcheck reporting no broken links, confirming both new relative links resolve"
    requirement: "DOCS-02"
    verification:
      - kind: other
        ref: "mdbook build docs/ (after mdbook-mermaid install docs/ regenerated the gitignored mermaid.min.js/mermaid-init.js assets) — 'No broken links found', exit 0"
        status: pass
    human_judgment: false

duration: 15min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 06: Architecture Appendix Disposition Summary

**ADR-0047 archives `docs/src/appendix/design-and-architecture.md`, re-anchors FR-26.1's metric to 19 of 19 by giving Sentinel a home in the live chapter, and withdraws three of four Mermaid diagram clauses as genuinely unanswered by the existing SVG inventory**

## Performance

- **Duration:** 15 min
- **Started:** 2026-08-24T12:43:00Z
- **Completed:** 2026-08-24T12:46:20Z
- **Tasks:** 2
- **Files modified:** 5 (1 created, 4 modified)

## Accomplishments
- Authored ADR-0047, carrying all three D-04 sub-decisions (archive; metric re-anchoring naming Sentinel; diagram-clause withdrawal with its SVG mapping) as three visually distinct Finding-N sections, and advanced `.planning/decisions/PROMOTION.md`'s next-free-number to 0048 in the same commit
- Prepended an archive banner to `docs/src/appendix/design-and-architecture.md` naming both `docs/src/architecture/` and `docs/src/appendix/sentinel.md` as where to look instead, citing ADR-0047; the 311 lines and the book's TOC entry both survive, the diff is additions-only
- Gave Sentinel a home in `docs/src/architecture/overview.md`, closing the recorded metric at 19 of 19 by a re-derivable measurement (0 → 3 whole-word case-insensitive matches across `docs/src/architecture/`)
- Read each of the six existing SVGs' rendered `<text>` content directly (not inferred from filenames) and mapped FR-26.1's four named diagrams against them: one answered, three withdrawn with the reason recorded — none of the six depicts Battalion, the Paladin execution loop, or Arsenal/MCP, because all six predate the agent system and still show the old content-processing pipeline

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0047 with all three sub-decisions and advance PROMOTION.md** - `d5585e7` (docs)
2. **Task 2: Add the archive banner and close the metric by giving Sentinel a home in the live chapter** - `9fb9f79` (docs)

**Plan metadata:** committed as part of this SUMMARY.

_Note: both commits used `git commit --no-verify` per D-00o (worktree_skip_hooks)._

## Files Created/Modified
- `.planning/decisions/0047-architecture-appendix-disposition.md` - New ADR carrying all three D-04 sub-decisions
- `.planning/decisions/PROMOTION.md` - Added the `0047` index row; advanced next-free line to `0048`; appended the dated note
- `docs/src/appendix/design-and-architecture.md` - Archive banner prepended after the title, before the Table of Contents
- `docs/src/architecture/overview.md` - New "Sentinel (Vision System)" section added after "Arsenal (Tool System)"
- `docs/src/SUMMARY.md` - Appendix TOC entry label changed to "Design and Architecture (Archived)"

## Decisions Made
- **ADR-0047 uses ADR-0033's Finding-N structure, not ADR-0022's single-narrative shape.** D-04 asked for the ADR-0022 *pattern* (restate the stale premise, write re-instatement as an instruction) applied to *three* sub-decisions with different dispositions. ADR-0022 itself carries only one decision; ADR-0033's three-Finding structure is the closer precedent for keeping three dispositions visually distinct without merging them.
- **Three of the four Mermaid diagram clauses are withdrawn, not force-mapped.** Reading each SVG's actual rendered text (rather than trusting names like `ComponentInteractionFlow.svg`) showed all six depict the pre-agent content-processing pipeline (Aggregator, Summarizer, Scheduler, Queue Manager) with zero mentions of Battalion, Paladin, Arsenal, or MCP anywhere. Per D-03's explicit instruction ("do not stretch an SVG to cover a diagram it does not depict"), only "overall hexagonal system architecture" is answered (by `ArchitectureOverview.svg`, `LayerArchitecture.svg`, and `crate-map.md:23`'s dependency graph, which together still express the same layered ports-and-adapters topology even with stale service names); the other three are withdrawn with the reason recorded in ADR-0047 Finding 3.
- **Sentinel's live-chapter section is a summary-plus-cross-link, not a duplication.** `docs/src/appendix/sentinel.md` is 976 lines; `docs/src/architecture/overview.md` gets a short section (what Sentinel is, where it lives, one link to the full reference) matching the treatment Garrison/Sanctum/Arsenal already receive on that page — closing the 19-of-19 count without creating a second full Sentinel document.
- **The ADR-0047 citation in the banner is plain backtick text, not a markdown link.** Checked existing precedent (`branch-protection.md`, `testing-guide.md`, `docker.md`, `stable-api.md`) — every existing in-book ADR citation uses `` `.planning/decisions/NNNN-slug.md` `` as plain text, never a hyperlink that would cross out of the mdbook `src/` tree. Matched that convention rather than introducing a new pattern that risked linkcheck friction.

## Deviations from Plan

None - plan executed exactly as written. One environment gap was fixed to enable verification, not a plan deviation: `mdbook build docs/` initially failed with "Unable to copy `docs/mermaid.min.js`" because that file (and `mermaid-init.js`) are gitignored, locally-regenerated assets (see `docs/.gitignore`) that had never been generated in this worktree. Ran `mdbook-mermaid install docs/` (the documented regeneration command) before rebuilding — this is local, gitignored tooling setup, not a tracked-file change (`git status --short docs/` before and after showed the same three tracked files already staged for Task 2, nothing else). After that, `mdbook build docs/` exits 0 with "No broken links found".

## Issues Encountered
None beyond the mdbook-mermaid asset regeneration noted above, which is standard local setup and not specific to this plan's edits.

## User Setup Required
None - no external service configuration required.

## Before/After Measurements (D-00e)

**ADR file existence:**
```
$ ls .planning/decisions/0047-*.md
.planning/decisions/0047-architecture-appendix-disposition.md
```

**Sentinel whole-word case-insensitive count across the live chapter:**
```
$ grep -ric --include=*.md -w sentinel docs/src/architecture/ | awk -F: '{s+=$2} END {print s}'
Before Task 2: 0
After Task 2:  3   (all 3 in docs/src/architecture/overview.md)
```

**Appendix file line count (banner added, nothing removed):**
```
$ wc -l docs/src/appendix/design-and-architecture.md
Before: 311
After:  319
```

**Four-diagram-to-artifact mapping, as recorded in ADR-0047 Finding 3:**

| # | Named diagram | Disposition | Mapped artifact / reason |
|---|---|---|---|
| 1 | Overall hexagonal system architecture | **Answered** | `docs/src/assets/ArchitectureOverview.svg`, `docs/src/assets/LayerArchitecture.svg` (layer/port topology, stale service names but current ring structure), plus `docs/src/architecture/crate-map.md:23`'s mermaid dependency graph (current, code-verified inward-only dependency invariant) |
| 2 | Battalion orchestration patterns | **Withdrawn** | None of the six SVGs nor `crate-map.md:23` mentions Formation, Phalanx, Campaign, Chain of Command, Conclave, Council, Grove, or Maneuver in rendered text — all six predate Battalion. Live chapter already covers this textually (`docs/src/architecture/overview.md` pattern table, lines 189-200). |
| 3 | Data flow through a Paladin execution cycle | **Withdrawn** | `data-flow.svg` and `ContentProcessingPipeline.svg` are text-content-identical and depict the pre-agent four-stage content pipeline (Ingestion→Validation→Processing→Storage), not the Paladin LLM reasoning loop. Live chapter already draws the loop in ASCII (`overview.md` lines 162-178). |
| 4 | Arsenal/MCP tool integration flow | **Withdrawn** | No SVG or mermaid block mentions Arsenal, MCP, STDIO, or SSE. `ComponentInteractionFlow.svg`'s "Content Port" is the pre-agent content-delivery port, not `ArsenalPort`. Live chapter already covers Arsenal textually (`overview.md` lines 216-222). |

## Next Phase Readiness
- DOCS-02 closed: `design-and-architecture.md` carries a recorded disposition (archived, with a signpost), the metric closes at 19 of 19 by measurement, and the diagram clause is honestly resolved (one answered, three withdrawn with reasons) rather than force-mapped or fabricated.
- The five live architecture-chapter pages are **not** audited for currency by this plan (explicitly out of scope per `16-CONTEXT.md` Deferred Ideas) — a future phase auditing them inherits the 19-of-19 baseline this plan establishes.
- No blockers for the rest of Phase 16 (DOCS-01, DOCS-03, DOCS-04 plans are independent of this one).

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*
