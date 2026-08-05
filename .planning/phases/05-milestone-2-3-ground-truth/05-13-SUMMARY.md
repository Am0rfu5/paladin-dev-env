---
phase: 05-milestone-2-3-ground-truth
plan: 13
subsystem: docs
tags: [requirements-ledger, adr-numbering, milestone-close-out]

requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: 118 filled REQ-* rows across 14 epic sections in milestone-02-03.md, plus three VERIFY-02 block verdicts (05-05, 05-06, 05-07)
provides:
  - "A `## Summary` section on `.planning/ledgers/milestone-02-03.md`: five-bucket verdict distribution (64/25/3/5/21 = 118), a 2-item nested-outstanding-item count, a three-row block roll-up, an explicit Phase 6 CLOSE-02 scope, a findings register, and a zero-unresolved citation sweep"
  - "REQUIREMENTS.md's Milestone 2-3 pointer finalised with true counts (118 rows, 2 nested items)"
  - "PROMOTION.md's ADR numbering index advanced past 0010-0012; next free number is 0013"
affects: [phase-06-close-out]

tech-stack:
  added: []
  patterns: ["Summary-section-as-close-out: an H2 Summary placed before the first epic heading, counting rows rather than re-judging them"]

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md

key-decisions:
  - "Folded the 3 'satisfied, with caveat' verdict-cell rows into the satisfied bucket for the distribution table (rather than a 6th ad-hoc bucket), with an explicit legend-deviation note in the Summary explaining why — their evidence text confirms the D-01 bar is cleared and the caveat does not defeat the row's own claim"
  - "Counted the nested-outstanding-item figure strictly per the plan's own definition (blank-first-two-column rows only = 2), not the looser count of all 'New finding' annotations anywhere in the ledger (12) — the 10 difference is documented in the Summary so a reader is not misled by the smaller number"

requirements-completed: [VERIFY-01, VERIFY-02]

coverage:
  - id: D1
    description: "Ledger `## Summary` section: five-bucket verdict distribution summing to 118, 2-item nested-outstanding count, three-row block roll-up, explicit Phase 6 CLOSE-02 scope, findings register, citation-resolution sweep"
    requirement: "VERIFY-01"
    verification:
      - kind: other
        ref: "grep -c '^## Summary' / '| Verdict | Count |' / '| Block | Open items | Verdict |' / '^### Phase 6 CLOSE-02 scope' / '^### Findings the ingest record did not contain' / '^| REQ-' against .planning/ledgers/milestone-02-03.md — all pass"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQUIREMENTS.md's Milestone 2-3 pointer finalised with true counts (118 rows, 2 nested items), placeholder sentence removed"
    requirement: "VERIFY-02"
    verification:
      - kind: other
        ref: "grep -c 'finalised by plan 05-13' .planning/REQUIREMENTS.md == 0; sed range grep '118' >= 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md numbering index lists 0010-0012 with titles; next free ADR number advanced to 0013"
    requirement: "VERIFY-02"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0013' == 1; grep -c 'Next free ADR number: 0010' == 0"
        status: pass
    human_judgment: false

duration: ~40min
completed: 2026-08-05
status: complete
---

# Phase 05 Plan 13: Milestone 2-3 Ledger Close-Out Summary

**Closed the 118-row Milestone 2-3 ledger with an honest five-bucket verdict distribution (64 satisfied / 25 present-unproven / 3 genuinely outstanding / 5 deferred / 21 superseded), consolidated Phase 6's entire CLOSE-02 scope to three named clusters, and advanced the ADR numbering index to 0013.**

## Performance

- **Duration:** ~40 min
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Counted every one of the 118 `REQ-*` rows in `.planning/ledgers/milestone-02-03.md` directly (via `awk`/`grep`, never re-judged) and wrote a `## Summary` section stating the five-bucket distribution, a two-item nested-outstanding-item count, a three-row block-verdict roll-up, the consolidated Phase 6 CLOSE-02 scope, a findings register naming all twelve `New finding` annotations, and the result of a citation-resolution sweep across 141 unique cited paths (zero unresolved).
- Finalised REQUIREMENTS.md's Milestone 2-3 pointer with the ledger's true counts (118 rows, 2 nested items), removing plan 05-01's placeholder sentence.
- Advanced `.planning/decisions/PROMOTION.md`'s numbering index: added 0010 (`milestone-3-epic-numbering`), 0011 (`vision-port-surfaces`), 0012 (`live-api-test-key-behaviour`) with their exact titles, and moved "Next free ADR number" from 0010 to 0013.

## Verdict distribution (counted from the ledger, not restated from any SUMMARY)

| Verdict | Count |
|---|---|
| `satisfied` (includes 3 rows literally reading `satisfied, with caveat`) | 64 |
| `present, unproven` | 25 |
| `genuinely outstanding` | 3 |
| `deferred with reason` | 5 |
| `superseded by shipped code` | 21 |
| **Total** | **118** |

The `present, unproven` bucket (25) is reported plainly per CONTEXT.md D-03's own prediction that this bucket would be large — not rounded toward `satisfied`.

**Legend-deviation note (recorded, not silently absorbed):** 3 of the 64 `satisfied` rows (`REQ-autonomous-configurable-model`, `REQ-autonomous-orchestration-layers`, `REQ-mock-llm-adapter`) carry the literal Verdict-cell text `satisfied, with caveat`, not the bare legend value `satisfied`. This was not a value I invented or a row I re-judged — it is text prior plans (05-06, 05-05) wrote into the ledger, and I was prohibited from altering any row's verdict cell. I counted these three within the `satisfied` bucket because their own evidence text confirms the D-01 bar is cleared (citation + passing test) and the caveat does not defeat the row's claim — the same distinction the legend's `satisfied` entry already draws. The discrepancy between the strict 5-value legend and the 6 distinct literal strings actually present in the ledger is called out explicitly in the ledger's own `## Summary` section so a future reader diffing the legend against the table does not mistake the omission for an inconsistency this plan failed to catch.

## Nested-outstanding-item count

**2**, counted strictly per the plan's own definition: rows using the blank-first-two-column format (`REQ-vision-security-encryption` / plan 05-08, `REQ-conclave-domain-model` / plan 05-09). A further 10 `**New finding (plan 05-NN):**` annotations exist in the ledger but are folded inline into their host row's own Evidence cell rather than added as a separate nested row — 12 `New finding` annotations total, 2 of them nested rows in the strict sense REQUIREMENTS.md's pointer now states. Both counts (2 strict, 12 total) are recorded in the ledger's `## Summary` so the difference is visible rather than hidden.

## Three block verdicts (verbatim from the ledger)

| Block | Open items (from `.planning/intel/task-completion-state.md`) | Verdict |
|---|---|---|
| `tasks-epic22-battalion-commander-hardening.md` (Epic 22) | 81 | **`satisfied by shipped code`** — all fifteen parent-task clusters verify against the current tree, including the three the source task list still marks open (Council/Grove registry integration, Grove LLM routing — all shipped via commits `761c49c`, `0cdf8dd`, `5f05db7`). No work required for Phase 6 CLOSE-02. |
| `tasks-autonomous-agent-features.md` (Epic 14) | 45 | **`partially outstanding`** — 11 of 12 clusters verify; cluster `8.0` (YAML & CLI Configuration Support) fails. |
| `tasks-test-hardening-benchmarks-qa.md` (Epic 24) | 29 | **`partially outstanding`** — 7 of 9 clusters verify; clusters `1.0` (Fix Campaign & ChainOfCommand Benchmarks) and `8.0` (Final Quality Verification and CI/CD Integration) fail. |

## Consolidated Phase 6 CLOSE-02 scope

Exactly three named clusters — nothing added beyond what the three block verdicts above named as failing:

1. **Epic 14, cluster `8.0` — YAML & CLI Configuration Support.** `--auto-plan`/`--auto-prompt`/`--dynamic-temp`/`--enable-handoffs` CLI flags are declared but read by no code; the CLI's `PaladinYamlConfig` has no `autonomous` field. Domain-level `PaladinConfig.autonomous` is itself satisfied and tested — only the CLI wiring is missing.
2. **Epic 24, cluster `1.0` — Fix Campaign & ChainOfCommand Benchmarks.** No ChainOfCommand benchmark exists in `battalion_benchmarks.rs`, contradicting `docs/src/appendix/battalion-benchmarks.md`'s repeated claim that it is "Compiling and enabled."
3. **Epic 24, cluster `8.0` — Final Quality Verification and CI/CD Integration.** `.github/workflows/ci.yml` has zero jobs named `cli-tests`, `bench-check`, or `coverage` among its fifteen job keys.

Epic 22 verified clean across all fifteen clusters; per D-06 and REQUIREMENTS.md's own CLOSE-02 text, this is recorded as `satisfied by shipped code` with an explicit **"no work required"** verdict rather than being dropped from consideration.

## Citation resolution sweep

141 unique `file:line` citations were extracted from the ledger. 93 carry a full repository-relative path; all 93 resolved via `test -f`. The remaining 48 are bare filenames used as shorthand later in the same row after the full path was already established earlier in that row's own sentence. Two of those shorthand forms retained a partial directory prefix (`maneuver/service.rs`, `parser/mod.rs`) and were confirmed via `find` to resolve to `crates/paladin-battalion/src/maneuver/service.rs` and `crates/paladin-battalion/src/maneuver/parser/mod.rs` respectively. **Zero unresolved citations; no row required a D-00g in-place fix as a result of this sweep.**

## Phase-wide confirmation: no product code changed

- `git diff --stat $(git log --format=%H --grep='05-01' -1)^..HEAD -- '*.rs' 'Cargo.toml' '.github/'` — empty, across the phase's full commit range.
- `git diff --stat -- '*.rs' 'Cargo.toml' '.github/'` on this plan's own working-tree changes before commit — also empty.
- `ls .planning/decisions/00*.md | wc -l` — 12 (0001-0012).
- `ls .planning/decisions/ | grep -c coverage` — 1 (ADR-0006 remains the only coverage ADR).
- The three new ADR files (`0010-milestone-3-epic-numbering.md`, `0011-vision-port-surfaces.md`, `0012-live-api-test-key-behaviour.md`) each carry exactly 7 `## ` H2 headings, matching PROMOTION.md's required heading set.

## Task Commits

Both tasks were staged together and committed once, per this plan's own instruction ("this plan commits once, after Task 2"):

1. **Task 1: Write the ledger summary, roll up the three block verdicts, consolidate CLOSE-02's scope** — combined into the single commit below (no separate commit, per plan instruction).
2. **Task 2: Finalise REQUIREMENTS.md's pointer counts and advance PROMOTION.md to 0013** — combined into the single commit below.

**Plan commit:** `386f95e` — `docs(05-13): close out Milestone 2-3 ledger with summary and finalize counts` (`.planning/ledgers/milestone-02-03.md`, `.planning/REQUIREMENTS.md`, `.planning/decisions/PROMOTION.md`)

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — added `## Summary` H2 (verdict distribution, nested-item count, block roll-up, CLOSE-02 scope, findings register, citation sweep result); zero rows' Verdict/Evidence cells altered, still exactly 118 `REQ-*` rows and 14 epic sections
- `.planning/REQUIREMENTS.md` — Milestone 2-3 pointer's placeholder sentence replaced with the true counts (118 rows, 2 nested items); scoped `Edit`, Milestone 1 and Milestone 4-6 sections untouched
- `.planning/decisions/PROMOTION.md` — numbering index gained 0010-0012 with titles; "Next free ADR number" advanced 0010 → 0013

## Decisions Made

See `key-decisions` in frontmatter: (1) folded "satisfied, with caveat" rows into the `satisfied` bucket with an explicit legend-deviation note rather than inventing a 6th bucket or silently dropping the caveat text; (2) reported the nested-outstanding-item count at the plan's own strict definition (2) rather than the looser "any New finding anywhere" count (12), with both numbers shown side by side in the ledger's Summary so neither is hidden.

## Deviations from Plan

None — plan executed exactly as written. The two counting decisions above are interpretive judgment calls the plan's own action text left implicit (it did not anticipate the "satisfied, with caveat" literal string, and its "blank-first-two-column" nested-row definition turned out narrower than the ledger's actual "New finding" annotation count) — both are documented transparently in the ledger's own `## Summary` section and in this SUMMARY, not treated as silent deviations from a plan instruction.

## Known Stubs

None. This plan is documentation-only close-out work; it wires no new UI or data path and introduces no placeholder values.

## Threat Flags

None. No new network endpoints, auth paths, file access patterns, or schema changes were introduced — this plan edits three planning-corpus markdown files only, per the phase's `.rs`/`Cargo.toml`/`.github/workflows/` prohibition, confirmed empty by the phase-wide `git diff --stat` sweep above.

## Issues Encountered

None.

## Next Phase Readiness

Phase 6's CLOSE-02 planner can read `.planning/ledgers/milestone-02-03.md`'s `## Summary` → `### Phase 6 CLOSE-02 scope` section directly and size itself against exactly three named clusters (Epic 14 `8.0`; Epic 24 `1.0` and `8.0`) without re-reading the three block verdicts or re-deriving the scope itself. Phases 7, 10 and 13 can take ADR numbers starting at 0013 without collision. This is the final plan in Phase 05 — the phase is complete pending orchestrator-level STATE.md/ROADMAP.md updates after all wave agents merge.

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-05*
