---
phase: 05-milestone-2-3-ground-truth
plan: 02
subsystem: planning-provenance
tags: [adr, documentation-correction, milestone-3, epic-numbering, VERIFY-03]
dependency-graph:
  requires: []
  provides:
    - "ADR-0010: authoritative Milestone 3 epic numbering (19-24)"
    - "RELEASE_NOTES_MILESTONE_3.md corrected in place, annotate-don't-rewrite"
  affects:
    - "Phase 5 ledger plans 05-08..05-12 (Epic 15/16/17 attribution defect rows)"
    - "Plan 05-13 (PROMOTION.md next-free-ADR advance)"
    - "Any Phase 6-16 reader citing a Milestone 3 epic number"
tech-stack:
  added: []
  patterns:
    - "ADR seven-H2 heading shape, no frontmatter (matches 0001/0004)"
    - "Annotate-don't-rewrite in-place document correction (D-08), reusing ROADMAP.md's self-amendment banner pattern"
key-files:
  created:
    - .planning/decisions/0010-milestone-3-epic-numbering.md
  modified:
    - .project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md
decisions:
  - "The plan/epic-definition numbering (19 Herald ... 24 Test Hardening) is authoritative; the release-notes numbering is a documentation defect, recorded in ADR-0010 with Code Conformance conforms"
  - "Release-notes correction is annotation, not rewriting: every original heading, claim, and code example is retained verbatim and marked superseded/corrected inline, per D-08 and the .project/ historical-corpus provenance requirement"
metrics:
  duration: "~35min"
  completed: 2026-08-04
status: complete
---

# Phase 5 Plan 02: ADR-0010 and Release Notes Correction Summary

Recorded the Milestone 3 epic-numbering defect once and permanently as ADR-0010 (seven-H2 shape,
`Code Conformance: conforms`), then annotated `RELEASE_NOTES_MILESTONE_3.md` in place — five
corrected epic headings, the verified-absent `RoutingStrategy::PerformanceBased` claim struck and
corrected, the divergent Council/Maneuver API-form claims corrected against the shipped surfaces,
and the Milestone 4 forward-look marked superseded — with every original claim retained.

## What Was Built

**Task 1 — ADR-0010** (`.planning/decisions/0010-milestone-3-epic-numbering.md`): the seven
required H2 headings (`Status`, `Context`, `Decision`, `Considered Options`, `Code Locations`,
`Code Conformance`, `Downstream Consumers`), no YAML frontmatter, matching the shipped shape of
`0001-battalion-config.md` and `0004-temperature-validation.md`. The `## Decision` mapping table
carries one row per integer 19-24 ascending, both sides of every collision recorded — including
the second-order collision at Epic 23 (release notes' "Commander Enhancement" content is itself
the authoritative Epic 22, Battalion & Commander Hardening) and Epic 24's non-colliding row
(`— (release notes' Epic 24 heading at :160 already matches)`), recorded rather than omitted.
`Code Conformance` is `conforms` — this is a documentation defect; no Rust source is wrong.

**Task 2 — RELEASE_NOTES_MILESTONE_3.md correction**, applied bottom-up by line number (320, 160,
147, 111, 106, 76, 48, 21, banner last) exactly as the plan specified, so no earlier insertion
shifted a later citation target:
- Top-of-file blockquote banner citing ADR-0010, stating nothing is deleted.
- Five `### Epic NN: <original title>` headings struck through and followed by the authoritative
  title, each with a `**Corrected numbering (ADR-0010):**` line naming the content's actual home
  (Epic 19 → Milestone 2 Epic 15, Epic 20 → M2 Epic 16, Epic 21 → M2 Epic 16, Epic 22 → M2 Epic
  17/17.5, Epic 23 → Milestone 3 Epic 22).
- Epic 24's heading left uncorrected with a one-line note confirming it already matches, so its
  absence from the correction list does not read as an oversight.
- The `RoutingStrategy::PerformanceBased` bullet (:106) and the related "Dynamic Learning" bullet
  (:85) struck through and corrected against the shipped three-variant `RoutingStrategy` enum.
- The Council code example's `execute`/`summary` claim and the Maneuver code example's
  `Maneuver::new(flow3, paladins, config)` claim each corrected inline (after their code blocks,
  not by rewriting the fenced code itself) against the shipped surfaces.
- The `## 🔮 What's Next (Milestone 4)` section prefixed with a superseded marker citing
  `intel/code-verification.md`'s record of Sentinel vision as shipped.

Both files committed together in a single commit (`b37a866`), per the plan's explicit instruction
and D-08's "written by exactly one plan in exactly one wave" invariant — the banner's presence is
the completion sentinel.

## Shipped Code Actually Read (per plan's output spec)

- **`RoutingStrategy` variants** — `crates/paladin-core/src/platform/container/battalion/grove.rs:54`:
  exactly `KeywordMatch` (`#[default]`), `SemanticSimilarity`, `LlmRouting`. `PerformanceBased`
  does not exist; `grep -rn "PerformanceBased" crates/ src/` returns no matches.
- **Council API form** — `crates/paladin-battalion/src/council_service.rs:118`:
  `CouncilExecutionService::convene(&self, council: &Council, topic: &str) ->
  Result<CouncilResult, BattalionError>`; `CouncilResult` (`:25-29`) carries `conclusion:
  Option<String>`, not `summary`. The release notes' `council_service.execute(&council, &experts,
  topic)` / `result.summary` form does not match.
- **Maneuver constructor form** — `crates/paladin-battalion/src/maneuver/mod.rs:148-153`:
  `Maneuver::new(name: impl Into<String>, agents: HashMap<String, Paladin>, flow:
  FlowExpression, config: ManeuverConfig) -> Result<Self, ManeuverError>`. The release notes'
  `Maneuver::new(flow3, paladins, config)` (flow first, config third, no name) does not match.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 — completeness] Corrected the "Dynamic Learning" bullet alongside the explicitly
cited `PerformanceBased` bullet**
- **Found during:** Task 2
- **Issue:** `RELEASE_NOTES_MILESTONE_3.md:85` ("Dynamic Learning: Performance-based routing
  improves over time") carries the same discredited premise as the `PerformanceBased` claim the
  plan explicitly cited at `:106`, but the plan's action text named only `:106` as the correction
  target.
- **Fix:** Added a short cross-reference note beneath the `:85` bullet pointing at the fuller
  `:106` correction, rather than leaving an adjacent instance of the same false claim uncorrected
  in the same section.
- **Files modified:** `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`
- **Commit:** `b37a866`

### Interpretation notes (not deviations — plan text applied as specified)

- The plan's acceptance criteria for the `## 🔮 What's Next (Milestone 4)` correction require the
  superseded marker to appear "within the three lines preceding" the heading match, while the
  plan's action prose says "prefix the section." These were reconciled by placing the marker
  immediately before the `##` heading line (verified: the marker's last line sits directly above
  the heading, satisfying the mechanical acceptance check).
- No release-notes claim was found defective beyond the three the plan's D-08 anticipated
  (epic-numbering, `PerformanceBased`, Council/Maneuver API forms) plus the adjacent "Dynamic
  Learning" instance noted above.

## Known Stubs

None.

## Threat Flags

None — this plan's threat register (T-05-02, T-05-05) is fully mitigated: every original claim
remains greppable after the edit, the file's line count grew (349 → 396), and both verified-absent
claims were re-confirmed with `grep -rn`/direct file reads against the tree rather than transcribed
from `REQUIREMENTS.md`.

## Self-Check: PASSED

- `test -f .planning/decisions/0010-milestone-3-epic-numbering.md` → FOUND
- `test -f .project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` → FOUND
- `git log --oneline --all | grep -q b37a866` → FOUND (`b37a866 docs(05-02): author ADR-0010 and
  correct Milestone 3 release notes epic numbering`)
- ADR-0010: 7 `## ` headings in required order, no frontmatter, mapping table has exactly 6 rows
  (19-24 ascending), `Code Conformance` section contains `conforms` exactly once — all confirmed.
- Release notes: banner present once above every `### Epic` heading, `Corrected numbering
  (ADR-0010)` appears exactly 5 times, all five original epic-pattern names retained,
  `PerformanceBased` retained, `Herald & Domain Type Consolidation` present, line count 396 > 349,
  `git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/'` empty, `git status --porcelain` clean.
