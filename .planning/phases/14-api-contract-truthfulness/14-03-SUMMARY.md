---
phase: 14-api-contract-truthfulness
plan: 03
subsystem: docs
tags: [rustdoc, mdbook, llm-port, arsenal, tool-calling, adr-0042]

# Dependency graph
requires:
  - phase: 14-api-contract-truthfulness
    provides: "D-13's tree-scout finding that no shipped LLM adapter ever populates function_call, and the ADR-0042 forward reference reserved by D-16/PROMOTION.md"
provides:
  - "Reachability rustdoc on LlmResponse.function_call and ProviderCapabilities in crates/paladin-ports/src/output/llm_port.rs, pointing at ADR-0042"
  - "Reachability statement on four documentation pages (tool-integration.md, architecture/overview.md, architecture/domain-model.md, contributing/contributing-providers.md), all pointing at ADR-0042"
  - "Honest get_capabilities() template in contributing-providers.md (both flags false, inline rationale, correspondence-test pointer)"
affects: ["14-06 (ADR-0042 authoring)", "16 (DOCS-01 content-currency sweep, inherits the temperature_range staleness hand-off recorded below)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reachability rustdoc paragraph on a port DTO field/struct, pointing at a forward-referenced ADR, matching the file's existing temperature-range doc voice"
    - "mdBook callout/footnote stating a capability's reachability condition immediately after the section that would otherwise imply it is unconditional"

key-files:
  created: []
  modified:
    - crates/paladin-ports/src/output/llm_port.rs
    - docs/src/user-guides/tool-integration.md
    - docs/src/architecture/overview.md
    - docs/src/architecture/domain-model.md
    - docs/src/contributing/contributing-providers.md

key-decisions:
  - "Reachability rustdoc lands as a labeled sub-section ('Tool-call reachability' / a plain paragraph) rather than a bare sentence, matching the file's existing detailed-doc voice (FinishReason, temperature_range)"
  - "The doc callout in tool-integration.md states how a reader does invoke Arsenal today (directly via ArsenalPort) so the note does not read as 'tools do not work' -- required by the plan's must_haves"
  - "contributing-providers.md's separately-missing temperature_range field and other Milestone-11-era staleness were NOT touched -- out of the D-13 scope guard, recorded below as a hand-off to DOCS-01 / Phase 16"

patterns-established:
  - "Forward-referencing an ADR number not yet on disk (ADR-0042, authored by sibling plan 14-06) is acceptable in rustdoc/docs prose within the same phase, per D-16/PROMOTION.md's reserved-number convention"

requirements-completed: [WEB-04]

coverage:
  - id: D1
    description: "LlmResponse.function_call and ProviderCapabilities rustdoc state the tool-call reachability limitation and point at ADR-0042"
    requirement: "WEB-04"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-ports --doc (97 passed, 0 failed)"
        status: pass
      - kind: other
        ref: "cargo clippy -p paladin-ports --all-targets -- -D warnings"
        status: pass
    human_judgment: false
  - id: D2
    description: "Four documentation pages (tool-integration.md, architecture/overview.md, architecture/domain-model.md, contributing-providers.md) state the reachability limitation and point at ADR-0042; contributing-providers.md template declares both capability flags false"
    requirement: "WEB-04"
    verification:
      - kind: other
        ref: "grep -lc 'ADR-0042' across all four pages == 4; grep -lc 'LlmPort' across all four pages == 4; git diff --stat -- docs/ lists exactly these four files"
        status: pass
    human_judgment: false

duration: ~15min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 03: Tool-Call Reachability Rustdoc and Documentation Summary

**Added reachability rustdoc to `LlmResponse.function_call` and `ProviderCapabilities` in `paladin-ports`, plus matching reachability statements on four documentation pages, all pointing at ADR-0042 -- closing D-13's documentation half of WEB-04.**

## Performance

- **Duration:** ~15min
- **Completed:** 2026-08-12
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- `crates/paladin-ports/src/output/llm_port.rs`: added a reachability paragraph to `LlmResponse.function_call`'s rustdoc, a matching "Tool-call reachability" section to `ProviderCapabilities`'s doc block, one-line pointers on both `supports_tool_calling`/`supports_function_calling` field docs, and corrected the struct's illustrative `# Example` to set both flags `false`. No field, signature, or type moved.
- `docs/src/user-guides/tool-integration.md`: added a reachability callout after the Overview section's key concepts, and a one-sentence note at the end of the Tool Flow subsection naming the exact unreachable step ("LLM decides to use tool").
- `docs/src/architecture/overview.md`: added a footnote directly beneath the Paladin reasoning-loop state diagram naming the condition on the `tool call?` branch, linking to the tool-integration guide and ADR-0042.
- `docs/src/architecture/domain-model.md`: added one sentence after the Arsenal (Tool Domain) type listing distinguishing the two seams -- the domain types are real and invocable, the LLM-initiated path that would populate them from a provider response is not exercised by any shipped adapter.
- `docs/src/contributing/contributing-providers.md`: flipped the `get_capabilities()` template's `supports_tool_calling`/`supports_function_calling` example values to `false` with an inline rationale comment, and added a sentence above the code block pointing at the shared correspondence test (`crates/paladin-llm/src/lib.rs`) that fails the build on a false declaration.

## Task Commits

Each task was committed atomically:

1. **Task 1: Reachability rustdoc on ProviderCapabilities and LlmResponse.function_call** - `498adb7` (docs)
2. **Task 2: State the reachability limitation on the four pages that imply otherwise** - `f3e5f82` (docs)

_Note: this plan is documentation-only; both commits use `docs(14-03):` type._

## Files Created/Modified

- `crates/paladin-ports/src/output/llm_port.rs` - reachability rustdoc on `LlmResponse.function_call` and `ProviderCapabilities`, corrected `# Example` fence
- `docs/src/user-guides/tool-integration.md` - reachability callout after Overview, note at end of Tool Flow
- `docs/src/architecture/overview.md` - footnote beneath the reasoning-loop state diagram
- `docs/src/architecture/domain-model.md` - one sentence distinguishing the two seams under Arsenal (Tool Domain)
- `docs/src/contributing/contributing-providers.md` - honest `get_capabilities()` template, correspondence-test pointer

## Decisions Made

- Used a labeled "Tool-call reachability" / "## Tool-call reachability" sub-section in the rustdoc rather than a single trailing sentence, to match the file's existing detailed-doc voice used for `FinishReason` and `temperature_range`.
- Each of the four documentation pages states the limitation in its own register (a callout box in the user guide, a footnote under a diagram, a sentence after a type listing, an inline code comment plus prose in the contributing guide) rather than a single copy-pasted paragraph, per the plan's instruction.
- The tool-integration.md callout explicitly states how a reader invokes Arsenal today (directly, via the `ArsenalPort` API) so the note reads as "the LLM cannot trigger this yet" rather than "tools do not work" -- required by both the plan's must_haves and the WEB-04 prohibition against implying Arsenal/MCP itself is unavailable.

## Deviations from Plan

None - plan executed exactly as written. Both tasks' `<action>` and `<acceptance_criteria>` were followed literally; no Rule 1-4 auto-fixes were needed since this is a documentation-only plan with no compilable behavior beyond the doctest fence, which passed on first edit.

## Issues Encountered

- `mdbook build docs` initially failed with "Unable to copy `docs/mermaid.min.js`" -- `mdbook-mermaid install docs/` had never been run in this fresh worktree (the file is gitignored, generated tooling, not a repo defect). Ran `mdbook-mermaid install docs/` once, then `mdbook build` succeeded through the HTML backend. The linkcheck backend then reported exactly two broken links, both confirmed pre-existing at the phase's base commit (`e9b727d`) via `git show e9b727d:<file> | grep`: `deployment/docker.md:118`'s "linking outside root directory" and `user-guides/tool-integration.md`'s `[`MCPClient::connect_streamable_http`]` incomplete-link syntax (line number shifted by this plan's insertion, content unchanged). Neither is introduced by this plan; no new broken links.
- Cold `cargo test`/`clippy`/`doc` compiles in this fresh worktree took 1-2 minutes each (three parallel executor agents compiling concurrently), consistent with the known worktree cold-build cost -- no action needed since `worktree_skip_hooks=true` meant these were run manually rather than via pre-commit hooks.

## Known Stubs

None.

## Hand-off to DOCS-01 / Phase 16

Per the plan's explicit scope guard (D-13), the following out-of-scope staleness was observed on `docs/src/contributing/contributing-providers.md` but **not fixed** in this plan:

- The `get_capabilities()` template's `ProviderCapabilities` struct literal is missing the `temperature_range` field entirely (present on the real struct as of ADR-0004 / this phase's own `llm_port.rs` edits). A new-provider author following this template today would fail to compile against the real trait signature.
- No other Milestone-11-era content-currency issues were noticed on the four pages touched by this plan during the read-first pass, beyond the one above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Both `paladin-ports` items that gate/declare the tool path (`LlmResponse.function_call`, `ProviderCapabilities`) now carry the reachability statement; all four documentation pages point at ADR-0042; the provider template declares the capability honestly.
- Sibling plan 14-06 must land `.planning/decisions/0042-llm-native-tool-calling-deferred.md` for the five ADR-0042 references added by this plan to resolve to a real file -- currently a forward reference per D-16/PROMOTION.md's reserved-number convention (this plan does not block on 14-06's completion; the prose reads correctly either way, the pointer just isn't yet a live link).
- The `temperature_range` staleness in `contributing-providers.md` is now on record for Phase 16 / DOCS-01 to pick up alongside the other Milestone 11 content-currency items.
- No blockers for the rest of Phase 14's waves.

## Self-Check: PASSED

All five modified files verified present on disk; all three task/summary commit hashes
(`498adb7`, `f3e5f82`, `f615c34`) verified present in `git log --oneline --all`.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*
