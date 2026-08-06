---
phase: 07-workspace-ground-truth-recorded-answers
plan: 02
subsystem: docs
tags: [adr, precedence, port-value-types, token-usage, records]

requires:
  - phase: 07-workspace-ground-truth-recorded-answers plan 01
    provides: ".planning/ledgers/milestone-04-06.md scaffold, ADR-0014 (numbering convention)"
provides:
  - "ADR-0016: paladin-core owns PaladinResult, StopReason, TokenUsage; canonical TokenUsage target for Phase 8 / DEBT-05"
  - "Annotated .project/ Epic 2 ports-extraction PRD (FR-7, FR-10 corrected; §1 cross-reference re-verified absent)"
affects: [phase-08-verified-defect-closure, plan-07-06-ledger-fanout, plan-07-13-promotion-bookkeeping]

tech-stack:
  added: []
  patterns: ["ADR promotion of a DOC-typed .project/ decision record without --manifest re-tagging", "D-00g annotation-not-rewriting on a .project/ PRD"]

key-files:
  created:
    - .planning/decisions/0016-port-value-type-ownership.md
  modified:
    - .project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md

key-decisions:
  - "Checkpoint Task 1 decision option-a ratified by the project owner (interactive, pre-dispatch): paladin-core owns PaladinResult, StopReason, TokenUsage — matches shipped code and the Approved Epic 1 decision record."
  - "ADR-0016 promotes the Epic 1 decision record without --manifest re-tagging; it is the promotion, cited as its own provenance."
  - "CONTEXT.md D-08(5)'s anticipated §1 'Milestone 1 / Epic 2' cross-reference in this PRD was re-verified and confirmed absent from the live tree — no correction fabricated for text that isn't there."

patterns-established:
  - "ADR promotion procedure (PROMOTION.md Part A) applied end-to-end for a second ADR (0016, following 0014's first use in 07-01): next number taken, standard heading set authored, source .project/ path cited as provenance, no manifest re-tagging."

requirements-completed: [ARCH-03]

coverage:
  - id: D1
    description: "ADR-0016 exists with all seven canonical H2 headings in order, no frontmatter, bulleted Considered Options and Code Locations, must-change conformance naming Phase 8 / DEBT-05 twice, and freshly re-grepped file:line citations for the canonical TokenUsage and its two duplicates."
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "grep -c '^## ' == 7; grep -n '^## ' order check; head -1 != '---'; Considered Options bullets == 4; Code Locations bullets == 9 (7 with :line); Code Conformance 'must change' == 1; DEBT-05 count == 3; provenance cite == 1; token_usage.rs cite == 3 — all run and confirmed passing"
        status: pass
    human_judgment: false
  - id: D2
    description: "Epic 2 ports-extraction PRD carries a dated top-of-file banner naming ADR-0016 and ADR-0014, FR-7/FR-10 struck through inline with a bold ADR-0016-citing correction retaining original text, and no fabricated §1 correction for a cross-reference confirmed absent from the live document."
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "grep -c ADR-0016 == 4 (>=3); grep -c ADR-0014 == 3 (>=2); grep -c FR-11 == 4 (>=2); git diff --numstat deletions == 2 (<=5); git diff --stat HEAD~1 -- *.rs Cargo.toml .github/ empty"
        status: pass
    human_judgment: true
    rationale: "Task 3's <verify> carries a human-check: confirm the struck-through FR-7/FR-10 original sentences remain readable and the corrected note names ADR-0016 (07-VALIDATION.md §Manual-Only Verifications, row 2) — a visual/readability judgment this executor cannot self-certify."

duration: ~25min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 02: Port value-type ownership ADR and PRD correction Summary

**ADR-0016 ratifies `paladin-core` as the sole owner of `PaladinResult`, `StopReason` and `TokenUsage` (promoting the Approved-but-DOC Epic 1 decision record without re-tagging it), and hands Phase 8 / DEBT-05 the canonical `TokenUsage` target at `token_usage.rs:13`; the competing Epic 2 PRD is annotated in place so FR-7/FR-10 can no longer be applied literally.**

## Performance

- **Duration:** ~25 min
- **Started:** ~2026-08-06T17:45:00Z (approx — PLAN_START_TIME bash capture was not run at session start)
- **Completed:** 2026-08-06T18:07:00Z
- **Tasks:** 3 (1 checkpoint, pre-resolved; 2 auto)
- **Files modified:** 2

## Accomplishments

- Task 1 (checkpoint:decision, `gate="blocking"`): **already resolved** before this executor was dispatched. The project owner selected **option-a** ("Ratify the shipped answer — `paladin-core` owns", CONTEXT.md D-11) in an interactive prompt on 2026-08-06, having reviewed all three options. This executor did not re-present the checkpoint; it proceeded straight to Task 2 per the explicit pre-resolution instruction in its dispatch context.
- Task 2: authored `.planning/decisions/0016-port-value-type-ownership.md` — seven canonical H2 headings (`Status / Context / Decision / Considered Options / Code Locations / Code Conformance / Downstream Consumers`), no frontmatter, bulleted `Considered Options` (4 entries) and `Code Locations` (9 entries, 7 carrying freshly re-grepped `file:line` citations). `Code Conformance: must change`, naming Phase 8 / DEBT-05 as executor. States explicitly that this ADR *is* the promotion of `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`, cited as provenance, with no `--manifest` re-tagging performed.
- Task 3: annotated `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` — dated top-of-file banner naming both ADR-0016 and ADR-0014; FR-7 and FR-10 corrected inline (struck through, bold corrected note citing ADR-0016, original text retained above each); re-verified CONTEXT.md D-08(5)'s anticipated §1 "Milestone 1 / Epic 2" cross-reference and confirmed it does not exist in the live document (see "Re-verified findings" below) — no correction fabricated for it.

## Re-verified findings (fresh `file:line` citations, all re-grepped 2026-08-06)

- `crates/paladin-core/src/platform/container/token_usage.rs:13` — canonical `TokenUsage` (`pub struct TokenUsage`). Matches CONTEXT.md D-11.
- `crates/paladin-core/src/platform/container/battalion/mod.rs:497` — duplicate `TokenUsage` struct. Matches CONTEXT.md D-11.
- `crates/paladin-llm/src/llm_analysis_service.rs:51` — second duplicate `TokenUsage` struct. Matches CONTEXT.md D-11.
- `crates/paladin-core/src/platform/container/execution_result.rs:38` — `PaladinResult` struct.
- `crates/paladin-core/src/platform/container/execution_result.rs:76` — `StopReason` enum.
- `crates/paladin-core/src/platform/container/registry_error.rs:10` — `RegistryError` enum.
- `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs:27` — `HandoffError` enum.
- None of these citations had drifted from CONTEXT.md's recorded values — all confirmed exact on first grep, unlike the `ci.yml:228→304` drift 07-01 found for its own citations.

**§1 "Milestone 1 / Epic 2" cross-reference — confirmed absent.** ADR-0014 (authored by plan 07-01) flagged that CONTEXT.md D-08(5) anticipated a "Milestone 1 / Epic 2" cross-reference inside `prd-paladin-ports-extraction.md` §1 (mirroring `prd-paladin-llm-extraction.md` Non-Goal 2's "hardened in Milestone 1 / Epic 2"), and explicitly asked plan 07-02 to re-verify before treating it as a sixth numbering-collision correction target. This executor ran `grep -in "milestone 1"` and `grep -in "milestone-1"` across the entire live document (2026-08-06): the only "Milestone" occurrence in the file is line 4's own correct self-identification, `**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split`. No "Milestone 1" or "Milestone 1 / Epic 2" text of any kind exists in this document. Per this plan's explicit phase-level instruction, no fabricated correction was added for a claim the document never made — the finding is recorded here and in the top-of-file banner instead, both citing ADR-0014. This closes the drift ADR-0014 flagged: the anticipated sixth correction target does not exist.

## Task Commits

Each task was committed atomically:

1. **Task 1: Confirm the canonical owner of the port value types** — pre-resolved by the project owner before dispatch; no commit (decision-only, no file changes of its own).
2. **Task 2: Author ADR-0016 — port value-type ownership** — `9e8db80` (docs)
3. **Task 3: Annotate the Epic 2 ports-extraction PRD** — `71ea46e` (docs)

**Plan metadata:** this summary's commit (pending)

## Files Created/Modified

- `.planning/decisions/0016-port-value-type-ownership.md` — new ADR; `must change`, Phase 8 / DEBT-05 named as executor
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` — dated correction banner plus inline annotation of FR-7 and FR-10; original text retained

## Decisions Made

- **Checkpoint Task 1 → option-a**, ratified by the project owner interactively before this executor's dispatch (see `<checkpoint_already_resolved>` in this executor's launch context). Recorded here per the plan's `<output>` instruction: "Record: the checkpoint decision and who approved it."
- **ADR-0016 promotes rather than re-decides.** The Epic 1 decision record (`Status: Approved`, manifest-typed DOC) is cited as this ADR's provenance and is not re-tagged via `--manifest` — re-typing a `.project/` file would change how five completed ingest runs classified their corpus, for an outcome an ADR achieves natively (PROMOTION.md Part A / D-11).
- **No fabricated §1 correction.** Given an explicit instruction to re-verify rather than trust a forward-carried claim, and a confirmed-absent grep result, this executor recorded the finding instead of inventing a correction — consistent with T-07-07's repudiation-threat framing (deleting/altering text that was never there is itself a record defect in the other direction).

## Deviations from Plan

### Auto-fixed / Process Issues

**1. [Process deviation, not a Rule 1-4 correctness issue] Two atomic commits instead of one combined commit for Task 3**
- **Found during:** Task 3, at commit time.
- **Issue:** Task 3's `<action>` text says "Commit this plan's two files together in a single commit at the end of the plan." This executor's standard `task_commit_protocol` (execute-plan.md) commits each task atomically as soon as it completes, and Task 2's ADR commit (`9e8db80`) had already landed before Task 3's action text (which contains the combine instruction) was reached.
- **Resolution:** Followed the standard atomic-per-task protocol: `9e8db80` (ADR-0016) and `71ea46e` (PRD annotation) are two separate commits rather than one. Both files are present in this plan's history with correct content; the combine instruction's intent — both artifacts land together, attributably, in this plan — is satisfied by the two adjacent commits, just not literally as one `git commit` invocation.
- **Impact:** `.planning/decisions/PROMOTION.md`-style tooling or any script asserting `git log -1 --name-only` shows both files will not match; a two-commit range (`9e8db80^..71ea46e`) does. No functional or record-accuracy impact — flagged here for the verifier.

**2. [Process deviation] `## Code Locations` line-with-strikethrough count is 2, not the acceptance criterion's "at least 3"**
- **Found during:** Task 3, acceptance-criteria self-check.
- **Issue:** Task 3's acceptance criteria expected three strikethrough corrections (banner is not one; FR-7 and FR-10 are two; a third was expected from a §1 correction). Because the §1 cross-reference does not exist in the live document (see "Re-verified findings" above), only two legitimate strikethrough corrections (FR-7, FR-10) exist. `grep -c '~~'` on the annotated file returns `2` (matching lines), not `>=3`.
- **Resolution:** No fabricated third correction was added. This is a direct consequence of the phase-level instruction to record absence rather than invent correction for text not present — followed deliberately, not an oversight.
- **Impact:** None on record accuracy; the mechanical acceptance criterion assumed a three-correction scenario that this specific document's live content does not support. `<verify>`'s actual automated check for Task 3 (`grep -q 'ADR-0016' && grep -q 'ADR-0014' && numstat deletions <= 5`) is unaffected and passes.

---

**Total deviations:** 2 process/documentation deviations, both deliberate and explained above. No Rule 1-4 auto-fixes were needed — the plan's substantive instructions (ADR content, FR-7/FR-10 correction shape) were followed as written.
**Impact on plan:** Both deviations are process-level (commit granularity, mechanical strikethrough count) rather than correctness or security issues. No scope creep; no fabricated content.

## Issues Encountered

None beyond the two deviations documented above.

## Known Stubs

None.

## Threat Flags

None. This plan's edits stay within the threat model's anticipated boundaries (T-07-05, T-07-06, T-07-07), each mitigated by the acceptance criteria checked above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Phase 8 / DEBT-05 is unblocked.** ADR-0016 names the canonical `TokenUsage` (`crates/paladin-core/src/platform/container/token_usage.rs:13`) and the two duplicates to collapse (`battalion/mod.rs:497`, `llm_analysis_service.rs:51`), with a `must change` Code Conformance verdict.
- **Plan 07-06** (ledger fan-out covering M5 Epics 1-2) can now write `REQ-port-value-type-ownership-v1` and `-v2` rows citing ADR-0016 by number; both row stubs already exist in `.planning/ledgers/milestone-04-06.md` as `PENDING-VERDICT`.
- **Plan 07-13** (bookkeeping) still needs to add ADR-0016's row to `PROMOTION.md`'s numbering index and advance the next-free-number line — not done by this plan, which only authors the ADR content per its `files_modified` scope.
- No blockers for downstream plans in this phase.

## Self-Check: PASSED

- FOUND: `.planning/decisions/0016-port-value-type-ownership.md`
- FOUND: `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md`
- FOUND: commit `9e8db80` (Task 2)
- FOUND: commit `71ea46e` (Task 3)

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
