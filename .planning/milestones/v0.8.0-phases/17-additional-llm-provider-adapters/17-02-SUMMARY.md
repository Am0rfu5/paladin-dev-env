---
phase: 17-additional-llm-provider-adapters
plan: 02
subsystem: docs
tags: [adr, requirements, provider-selection, llm]

# Dependency graph
requires: []
provides:
  - "ADR-0045: the additional-LLM-provider selection study, recorded with criteria before scoring"
  - "PROV-01 requirement row amended in place with the dated verdict summary"
  - "PROMOTION.md's next-free ADR number advanced from 0045 to 0046"
affects: [17-01, 17-03, 17-04, 17-05, 17-06]

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created:
    - .planning/decisions/0045-additional-llm-provider-selection.md
  modified:
    - .planning/decisions/PROMOTION.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "ADR-0045 records D-01/D-02/D-03 as a durable candidate table: Kimi, Qwen, Grok, Ollama, Gemini build; Groq/Together/Mistral/Fireworks/Bedrock rejected as already covered (not deferred); Meta/Llama dispositioned via Ollama"
  - "PROV-01's REQUIREMENTS.md row amended in place (D-00d), original text retained, checkbox left unticked for phase-close adjudication"

patterns-established: []

requirements-completed: []  # PROV-01's checkbox deliberately left unticked — see Deviations/Notes

coverage:
  - id: D1
    description: "ADR-0045 exists with the seven required headings, no frontmatter, criteria stated before scoring, and every candidate carrying exactly one build/defer/reject verdict"
    requirement: "PROV-01"
    verification:
      - kind: other
        ref: "test -f .planning/decisions/0045-additional-llm-provider-selection.md; grep -c '^## ' returns 7; grep -c '^## \\(Status\\|Context\\|Decision\\|Considered Options\\|Code Locations\\|Code Conformance\\|Downstream Consumers\\)$' returns 7"
        status: pass
    human_judgment: false
  - id: D2
    description: "Groq/Together/Mistral/Fireworks/Bedrock rejected as already covered, not deferred, within the candidate table rows"
    requirement: "PROV-01"
    verification:
      - kind: other
        ref: "grep -i defer over lines 73-83 (candidate table) of 0045-additional-llm-provider-selection.md returns no matches"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md's next-free ADR number line advances from 0045 to 0046, with the 0045 allocation row added"
    requirement: "PROV-01"
    verification:
      - kind: other
        ref: "grep -q 'Next free ADR number: 0046' .planning/decisions/PROMOTION.md && echo OK; grep -n '| 0045 |' .planning/decisions/PROMOTION.md"
        status: pass
    human_judgment: false
  - id: D4
    description: "REQUIREMENTS.md's PROV-01 bullet amended in place, additively, citing ADR-0045, with the checkbox left unticked"
    requirement: "PROV-01"
    verification:
      - kind: other
        ref: "grep -c 'ADR-0045' .planning/REQUIREMENTS.md returns 2; git diff -U0 REQUIREMENTS.md | grep -c '^-[^-]' returns 0; grep -c '^@@' returns 1; grep -c '^- \\[x\\] \\*\\*PROV-01\\*\\*' returns 0"
        status: pass
    human_judgment: false

duration: ~15min
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 02: Additional LLM Provider Selection Study Summary

**ADR-0045 records PROV-01's provider-selection study — five build verdicts, five already-covered rejections, and the Meta/Llama row settled by naming Ollama — and REQUIREMENTS.md's PROV-01 bullet is amended in place to point at it.**

## Performance

- **Duration:** ~15 min
- **Tasks:** 2 completed
- **Files modified:** 3 (1 created, 2 modified)

## Accomplishments

- Authored `.planning/decisions/0045-additional-llm-provider-selection.md` — the seven-section,
  no-frontmatter ADR recording PROV-01's nine scoring criteria before any candidate is scored, and
  the full scored candidate table with exactly one verdict per candidate.
- Advanced `.planning/decisions/PROMOTION.md`'s allocation table with the ADR-0045 row and its
  "Next free ADR number" line from 0045 to 0046, with a dated note explaining plan 17-06 accounts
  for the phase's second allocation (ADR-0046).
- Amended `REQUIREMENTS.md`'s PROV-01 bullet in place with a dated verdict summary block that
  fixes PROV-02's size at five named presets plus one generic operator-configured provider.

## Task Commits

Each task was committed atomically (`--no-verify`, per `workflow.worktree_skip_hooks: true`):

1. **Task 1: Author ADR-0045 — the additional-LLM-provider selection study** - `36b48c1` (docs)
2. **Task 2: Amend PROV-01 in REQUIREMENTS.md in place with the dated verdict summary** - `f1089ad` (docs)

**Plan metadata:** not applicable — worktree mode; the orchestrator makes the final metadata commit
after merge (per this plan's execution instructions, STATE.md/ROADMAP.md are not touched here).

## Files Created/Modified

- `.planning/decisions/0045-additional-llm-provider-selection.md` - ADR-0045: the scored candidate
  table (Kimi/Qwen/Grok/Ollama/Gemini build; Groq/Together/Mistral/Fireworks/Bedrock rejected as
  already covered; Meta/Llama dispositioned via Ollama), criteria recorded before scoring, D-03's
  consequence paragraph, and the D-00i provenance sentence.
- `.planning/decisions/PROMOTION.md` - added the ADR-0045 allocation row; advanced "Next free ADR
  number" from 0045 to 0046 with a dated note; Part A procedural prose untouched.
- `.planning/REQUIREMENTS.md` - PROV-01 bullet amended in place (additive only — `git diff -U0`
  shows 0 removed lines, 1 hunk) with the dated verdict summary citing ADR-0045; checkbox left
  unticked.

## Decisions Made

- **Recorded, not re-litigated.** D-01, D-02 and D-03 were locked in `17-CONTEXT.md` during the
  2026-08-16 `/gsd-discuss-phase 17` session; this plan transcribes them into ADR-0045 verbatim in
  substance. No verdict was changed, added, or reinterpreted during execution.
- **Groq's reason cell was written without the word "not deferred"** to keep the candidate table's
  own text unambiguous against the rejected-not-deferred acceptance check — the same point is made
  in the plan's own considered-options list and in the D-03 consequence paragraph outside the
  table, so no substance was lost.
- **PROV-01's checkbox stays unticked**, per the plan's own instruction and the Phase 3 precedent:
  the requirement is adjudicated complete at phase close, not per-plan, since sibling plans
  (17-01, 17-03 … 17-06) also carry work this requirement's "Done when" clause touches indirectly
  through PROV-02's now-fixed size.

## Deviations from Plan

None — plan executed exactly as written. Both tasks' acceptance criteria were verified via the
exact grep/diff commands the plan specified, and all passed on the first attempt.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. This is a documentation/decision-artifact-only
plan; no code, dependencies, or configuration changed.

## Threat Flags

None. Per this plan's own threat model, T-17-10 (vendor endpoints recorded) is accepted (public
base URLs only, no secret values), and T-17-SC (dependency additions) does not apply — this plan
adds no packages and touches no manifest.

## Next Phase Readiness

- ADR-0045's build list (Kimi, Qwen, Grok, Ollama, Gemini) is now the citable source of truth for
  plans 17-01, 17-03, 17-04 and 17-05's adapter work.
- PROV-02's size question ("one adapter or four") is resolved: five named presets plus the generic
  provider.
- ADR-0046 (plan 17-06, D-11's breaking default-flag change) remains open — PROMOTION.md's line
  now correctly reads 0046 as the next free number for that plan to consume.
- No blockers for downstream plans in this wave or later waves.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
