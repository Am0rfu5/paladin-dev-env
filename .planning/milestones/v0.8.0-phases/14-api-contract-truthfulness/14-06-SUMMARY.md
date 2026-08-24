---
phase: 14-api-contract-truthfulness
plan: 06
subsystem: docs
tags: [adr, deferred-work, tool-calling, epic-27, correction-banner, llm-port]

# Dependency graph
requires:
  - phase: 14-api-contract-truthfulness
    provides: "14-02's flag-honesty fix (OpenAIAdapter::get_capabilities().supports_function_calling now false, correspondence test pinning both flags) and 14-03's tool-call reachability rustdoc/docs sweep pointing at this plan's ADR-0042"
provides:
  - "ADR-0042: LLM-native tool calling (Deferred-QA Epic 27) recorded as a future capability improvement, not built, with a named reintroduction trigger and owner"
  - "A dated 2026-08-12 correction banner at all five D-11 sites in the Epic 27 source (.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md), plus a top-of-document banner, all pointing at ADR-0042"
affects: ["14-07 (amends the REQ-llm-tool-calling-port / REQ-llm-tool-calling-adapters ledger rows against this ADR and closes WEB-04's checkbox)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Deferred-with-trigger ADR shape (ADR-0035 precedent): a reintroduction condition and an owning surface promoted into a decision record without building the thing"
    - "Dated correction banner as pure-addition annotation (D-00c): new table rows and new blockquote paragraphs inserted around existing content, never editing an existing line, verified by git diff --numstat reporting 0 deletions"

key-files:
  created:
    - .planning/decisions/0042-llm-native-tool-calling-deferred.md
  modified:
    - .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md

key-decisions:
  - "REQUIREMENTS.md's WEB-04 checkbox is deliberately left unticked by this plan. Its own 'done when' text needs the decision (this ADR) AND the ledger/checkbox closure, which the plan explicitly assigns to 14-07 as 'the designated closer for the phase's requirements'. This plan's files_modified list does not include REQUIREMENTS.md or the milestone-09-12 ledger. Marking it here would misstate the record the way 14-03's SUMMARY already flagged as a risk."
  - "The bundled mock (crates/paladin-llm/src/mock.rs) is recorded as already truthful and left unchanged — both capability flags already false, function_call never populated — closing CONTEXT.md's Claude's-Discretion item about giving it the ability to emit one, per 14-02's decision to answer 'no' this phase and record the closure in this ADR."
  - "Table-row annotations (priority-order row, both open-question rows) use a new '| — |' row beneath the original rather than editing the original row's cells, because editing an existing line would register as a deletion in git diff --numstat and violate the plan's 0-deleted-lines requirement."

requirements-completed: []

coverage:
  - id: D1
    description: "ADR-0042 records LLM-native tool calling as a future capability improvement with a named reintroduction trigger and owner, quoting the user's WEB-04 framing verbatim and pairing with ADR-0039's Arsenal/HTTP half"
    requirement: "WEB-04"
    verification:
      - kind: other
        ref: "head -1 + per-section grep against .planning/decisions/0042-llm-native-tool-calling-deferred.md: 7 '## ' headers, no frontmatter, ADR-0039 x4, ADR-0035 x1, 'trigger' x7, 'owner' x4, all 9 unique Code Locations paths pass test -e"
        status: pass
    human_judgment: false
  - id: D2
    description: "Dated correction banner applied at all five D-11 sites in the Deferred-QA Epic 27 source, purely additive with zero deleted lines and the pre-existing 2026-08-10 ORCH-03 banner untouched"
    requirement: "WEB-04"
    verification:
      - kind: other
        ref: "git diff --numstat -- .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md (32 insertions(+), 0 deletions); grep -c ADR-0042 == 10; grep -c ORCH-03 unchanged at 5 before/after"
        status: pass
    human_judgment: false

duration: ~45min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 06: ADR-0042 — LLM-Native Tool Calling Deferred Summary

**Authored ADR-0042 recording Deferred-QA Epic 27 (LLM-native tool calling) as a future capability improvement — not built — with a named reintroduction trigger and owner, and annotated the Epic 27 source PRD with a dated, purely-additive correction banner at all five D-11 sites.**

## Performance

- **Duration:** ~45min
- **Completed:** 2026-08-12
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 modified)

## Accomplishments

- `.planning/decisions/0042-llm-native-tool-calling-deferred.md` authored in the standard no-frontmatter ADR shape (`Status / Context / Decision / Considered Options / Code Locations / Code Conformance / Downstream Consumers`), matching the ADR-0035/ADR-0039 precedent files re-read before writing.
- Context re-verifies, this session, that Epic 27 is entirely unbuilt: `LlmRequest` carries no tool-definition field, no `ToolDefinition`/`ToolCall` type exists anywhere in `paladin-ports` or `paladin-llm`, and every populated `function_call` in the workspace is a test double under `tests/` (four files, all named).
- Decision quotes the user's WEB-04 framing verbatim as a block quote, names the reintroduction trigger (a consumer needing a shipped adapter to initiate a tool call, conditioned on both open questions being answered) and the owner (`LlmPort` in `paladin-ports` plus its three adapters and the mock in `paladin-llm`), and pairs with ADR-0039's already-recorded Arsenal/HTTP half.
- Records that `crates/paladin-llm/src/mock.rs` is left unchanged — already declares both capability flags `false` and never populates `function_call` — closing the Claude's-Discretion item CONTEXT.md raised.
- A new dated (2026-08-12) top-of-document banner plus five site-level annotations were added to `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md`: the Epic 27 user-story block (US-27.1–27.3), the functional-requirements section (FR-27.1–27.7), the breaking-change/phased-approach risk note, the epic-priority ordering row, and both open questions (OQ-1, OQ-5) — all ten total `ADR-0042` references, zero lines deleted, the pre-existing 2026-08-10 ORCH-03 banner byte-identical and untouched.

## Task Commits

Each task was committed atomically:

1. **Task 1: ADR-0042 — LLM-native tool calling recorded as a future capability, with a trigger and an owner** - `09374f6` (docs)
2. **Task 2: Dated correction banner on the Deferred-QA Epic 27 source** - `e86d27f` (docs)

## Files Created/Modified

- `.planning/decisions/0042-llm-native-tool-calling-deferred.md` - New ADR: Epic 27 deferred as a future capability with a named trigger and owner, Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers, no frontmatter
- `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` - New top-of-document banner plus five site-level annotations pointing at ADR-0042, all additive; original text and the pre-existing ORCH-03 banner unchanged

## Decisions Made

- **REQUIREMENTS.md's WEB-04 checkbox and the milestone-09-12 ledger rows are NOT touched by this plan.** WEB-04's "done when" is "LLM tool calling is either in scope with a plan, or withdrawn with a reason" — this plan supplies the decision (ADR-0042), but the plan's own `files_modified` list is exactly `[.planning/decisions/0042-llm-native-tool-calling-deferred.md, .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md]` and explicitly hands the ledger/checkbox closure to plan 14-07 ("the designated closer for the phase's requirements," per the upstream context this plan was spawned with). Left `requirements-completed: []` and the checkbox untouched for 14-07 to close with full evidence.
- **Table-row annotations use a new row, not an edited cell.** For the epic-priority ordering table and the open-questions table, the plan's own acceptance criteria require `git diff --numstat` to report 0 deleted lines. Editing an existing markdown table row's cell text would show as one deleted line plus one added line in the diff, so each annotation is instead a new `| — | **Correction ...** | ... |` row inserted immediately after the row it corrects, leaving every original row byte-identical.
- **Line numbers in `## Code Locations` were re-derived this session, not copied from CONTEXT.md.** CONTEXT.md's own D-11 citations (`:124,250-298`) predate this session's fresh grep, which found the current tree's Epic 27 sites at `:124-131` (user stories, including US-27.3 which CONTEXT.md's citation range did not reach) and `:250-304` (functional requirements, through FR-27.7's item 70). The plan's action text explicitly requires re-derivation against the post-14-02/14-03 tree, so the ADR and the banner both cite the freshly-grepped ranges.

## Deviations from Plan

None — plan executed exactly as written. Both tasks' `<action>` and `<acceptance_criteria>` were followed literally.

## Issues Encountered

None. This is a documentation-only plan; no compilation, test run, or `cargo` command was required by either task, and none of the plan's verification steps depend on a build.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0042 exists at `.planning/decisions/0042-llm-native-tool-calling-deferred.md`, resolving the forward reference plan 14-03 already placed in five spots (`crates/paladin-ports/src/output/llm_port.rs` rustdoc and four documentation pages) from a dangling pointer into a live one.
- The Epic 27 source now carries a decision-pointing banner at every site D-11 named; a future reader of that PRD meets the recorded decision rather than an unbuilt epic, without any original text lost (`git diff --numstat` proves 0 deletions).
- **Outstanding for plan 14-07 (or the orchestrator):** amend the `REQ-llm-tool-calling-port` / `REQ-llm-tool-calling-adapters` rows in `.planning/ledgers/milestone-09-12.md` against this ADR, and tick REQUIREMENTS.md's WEB-04 checkbox with the evidence trail (14-02's flag fix, 14-03's docs sweep, this plan's ADR-0042 and banner) now that all three closing plans have landed.
- No blockers for sibling plans in this wave or for Phase 14's remaining waves — this plan touched only the two files in its `files_modified` list, neither shared with any other plan's scope.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*

## Self-Check: PASSED

- FOUND: .planning/decisions/0042-llm-native-tool-calling-deferred.md
- FOUND: .planning/phases/14-api-contract-truthfulness/14-06-SUMMARY.md
- FOUND commit: 09374f6 (Task 1)
- FOUND commit: e86d27f (Task 2)
- FOUND commit: 75e5394 (plan metadata / this SUMMARY)
