---
phase: 16-documentation-currency-the-architecture-gap
plan: 03
subsystem: docs
tags: [mdbook, docs-currency, arsenal, mcp, herald, paladin-builder, cli]

# Dependency graph
requires:
  - phase: 16-01
    provides: "Pinned mdbook toolchain, the D-09 verdict record seeded with all fourteen files, and the eight-class signal battery proven on cicd.md"
  - phase: 16-02
    provides: "Three of fourteen D-09 files settled by content (orchestration.md, maneuver-flow-dsl.md, memory-management.md), the reusable verdict-row shape"
provides:
  - "Two more of fourteen D-09 files settled by content: tool-integration.md, paladin-configuration.md; plus output-formatting.md, closing the six-file docs/src/user-guides/ group (Milestone 11 task 6.0)"
  - "tool-integration.md's fabricated Armament/ArmamentCall/ArmamentResult fields, ArsenalPort trait shape, six nonexistent ArsenalError variants, and the arsenal.mcp_servers YAML key (type -> server_type) corrected against the live tree"
  - "paladin-configuration.md's fabricated ApplicationSettings/from_config() config-file-loading mechanism replaced with the real Settings/AgentDefinition API; 16 missing .await on the async PaladinBuilder::build() fixed"
  - "output-formatting.md's Herald trait shape (seven methods, not three) corrected, two entirely fabricated built-in formatters (HtmlHerald, CodeHerald) removed, and fabricated per-formatter builder chains replaced with the real Config-struct construction pattern"
affects: [16-04, 16-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reuse the 16-01 D-09 verdict-row shape and eight-class signal battery verbatim across plans"
    - "When a fabricated API spans an entire section's mental model (e.g. PaladinBuilder::add_armament(), the Herald trait's real 7-method shape), add one prominent correction note documenting the real shape rather than rewriting every downstream illustrative code block individually — record the remaining sketches as a deliberately-scoped gap in the verdict row, not a silent one"
    - "When a real, complete, compiling reference implementation already exists in examples/, cross-reference it from a broken doc sketch instead of duplicating a corrected version inline"

key-files:
  created: []
  modified:
    - docs/src/user-guides/tool-integration.md
    - docs/src/user-guides/paladin-configuration.md
    - docs/src/user-guides/output-formatting.md
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md

key-decisions:
  - "PaladinBuilder::add_armament() does not exist anywhere in the tree (used ~5 times across tool-integration.md and paladin-configuration.md to attach a custom tool). Rather than rewriting every tool-attachment call site's registry-population mechanics — which would exceed D-12's no-restructure guard — added one explanatory wiring note documenting the real split (with_arsenal_registry() attaches metadata-only Armaments; a custom ArsenalPort implementor is wired through PaladinExecutionService's constructor, which PaladinBuilder does not expose) and left the shorthand call sites governed by that note."
  - "output-formatting.md's fabrication was comprehensive enough (nearly the whole file) that a full line-by-line rewrite of every one of ~15 custom-Herald illustrative examples was judged disproportionate to D-12. Fully rewrote the trait sketch, the three real built-in formatters, and the primitives-establishing custom example (UppercaseHerald); cross-referenced the XML/CSV Herald sketches to an already-existing complete implementation in examples/herald_custom_formatter.rs; left ten further illustrative custom-Herald names (AdaptiveHerald, BufferedStreamHerald, CachingHerald, DiffHerald, EnhancingHerald, LazyHerald, MultiFormatHerald, ProgressHerald, SanitizingHerald, TemplateHerald) as partial sketches governed by one scope note at the top of Custom Formatters, rather than individually completed — recorded explicitly in the verdict row, not silently."
  - "The task's own literal grep -rq \"$h\" crates/paladin-herald/src/ one-liner over every [A-Z][A-Za-z]*Herald match cannot resolve to zero UNRESOLVED for legitimately user-authored illustrative custom-Herald names (they were never claimed as shipped by the crate). The verdict row's per-name classification satisfies the acceptance criteria's own second clause (\"or has been corrected/removed, with the per-name result stated in the verdict row\") where the bare one-liner cannot."

requirements-completed: [DOCS-01]

coverage:
  - id: D1
    description: "tool-integration.md settled by content: fabricated Armament/ArmamentCall/ArmamentResult fields, the ArsenalPort trait signature, six nonexistent ArsenalError variants, a fabricated MCPStdioAdapter builder chain, and the arsenal.mcp_servers YAML key (type -> server_type) all corrected against the live tree"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -n 'ToolSchema|ParamType|\\.parameters\\b|schema:' docs/src/user-guides/tool-integration.md == 0 hits; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "paladin-configuration.md settled by content: the fabricated ApplicationSettings/from_config() config-loading mechanism replaced with the real Settings/AgentDefinition API, a nonexistent PaladinError::MaxLoopsExceeded variant, a nonexistent paladin.validate() call, add_armament(), and 16 missing .await on the async PaladinBuilder::build() all corrected"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -n 'build()' docs/src/user-guides/paladin-configuration.md | grep -v await == empty (every build() call followed by .await); mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "output-formatting.md settled by content: the Herald trait sketch corrected to its real seven methods, two entirely fabricated built-in formatters (HtmlHerald, CodeHerald) removed, fabricated per-formatter builder chains replaced with the real Config-struct API, and the primitives-establishing custom-Herald example fully corrected; closes the six-file docs/src/user-guides/ group"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'HtmlHerald|CodeHerald' docs/src/user-guides/output-formatting.md == 1 (explanatory prose only, not code); mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "Three evidence-bearing verdict rows appended to 16-DOCS-01-VERDICTS.md in the declared row order, replacing three pending rows; seven of fourteen remain explicitly pending for 16-04/16-05"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md == 8 after Task 1, 7 after Task 2; grep -c '^| docs/src/' == 14 throughout"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 03: User-Guides Currency Sweep, Part B (tool-integration.md + paladin-configuration.md + output-formatting.md) Summary

**Closed the six-file `docs/src/user-guides/` group against the live 0.8.0 tree by correcting the corpus's densest concentration of fabricated Rust API surface — a wrong Arsenal/ArmamentCall/ArmamentResult shape and six nonexistent `ArsenalError` variants in tool-integration.md, an entirely fabricated `ApplicationSettings`/`from_config()` config-file-loading mechanism plus 16 missing `.await` calls in paladin-configuration.md, and a wrong three-method `Herald` trait sketch plus two entirely fabricated built-in formatters in output-formatting.md — all re-verified against a passing `mdbook build docs/`.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-24T13:08:00Z (approx, first tool call after reading 16-02's artifacts)
- **Completed:** 2026-08-24T13:43:00Z
- **Tasks:** 2
- **Files modified:** 4 (3 doc pages, 1 verdict record)

## Accomplishments

- Ran the full eight-class D-09 signal battery plus end-to-end prose review against `tool-integration.md`, `paladin-configuration.md`, and `output-formatting.md`, recording every producing command and result
- `tool-integration.md`: fixed the `Armament`/`ArmamentCall`/`ArmamentResult` struct fields (a fabricated `schema: ToolSchema`/`ParamType` API that doesn't exist anywhere in the tree, a wrong `parameters` field name that's really `arguments`, an `output: String` that's really `Option<Value>`), the `ArsenalPort` trait signature (`list_armaments` is infallible `Vec<Armament>` not `Result<Vec<Armament>>`; `invoke` takes an owned `ArmamentCall` not a borrowed one), six nonexistent `ArsenalError` variants (`InvalidParameter`/`MissingParameter`/`ExecutionError`/`SecurityViolation` — none exist; real variants are `InvalidArguments`/`TransportError`), a fabricated `MCPStdioAdapter` builder chain (`.command().args().debug_mode().build()`), the `arsenal.mcp_servers` YAML key (`type:` → `server_type:`, the real `MCPServerConfig` field name; a nonexistent `enabled` field removed), two fabricated `examples/*.rs` citations, and documented `PaladinBuilder`'s missing `add_armament()` with a wiring note to the real `with_arsenal_registry()` API
- `paladin-configuration.md`: fixed 16 missing `.await` on the async `PaladinBuilder::build()` (including converting a `PaladinFactory`'s sync `fn`s to `async fn`s, since a sync fn cannot return an async call's result directly), an entirely fabricated "Configuration from File" subsection (`ApplicationSettings`/`ApplicationSettings::load_from()`/`PaladinBuilder::from_config()` — none exist; the real top-level config type is `Settings` with **no** `paladin` field at all) replaced with the real, narrower `Settings.agents: Vec<AgentDefinition>` mechanism, a nonexistent `PaladinError::MaxLoopsExceeded` variant (reaching `max_loops` is `Ok(..., stop_reason: StopReason::MaxLoops)`, never an `Err`), a nonexistent `paladin.validate()` call, and `add_armament()`
- `output-formatting.md` (the phase's most comprehensively fabricated file to date — nearly the entire "Built-in Formatters" and nine of its ~fifteen "Custom Formatters"-style sections described formatters or a trait shape that doesn't exist): corrected the `Herald` trait to its real seven methods (`format_stream_chunk` returns `Result<Option<String>, HeraldError>`, not a bare `Result<String, _>`), removed two entirely fabricated built-in formatters (`HtmlHerald` via a mislabeled `JsonHerald::with_css_framework(...)` chain, and a standalone `CodeHerald`) since the crate ships exactly three (`JsonHerald`, `MarkdownHerald`, `TableHerald` behind the `table` feature), replaced fabricated fluent builder chains with the real `*Config`-struct construction pattern, fully rewrote the primitives-establishing custom-Herald example, and cross-referenced two broken sketches to an already-existing complete implementation in `examples/herald_custom_formatter.rs`
- Wrote three evidence-bearing verdict rows into `16-DOCS-01-VERDICTS.md`, replacing `pending` rows in place, preserving the declared row order; seven of fourteen rows remain explicitly pending for 16-04/16-05 — this closes the six-file `docs/src/user-guides/` group (Milestone 11 task 6.0)
- `mdbook build docs/` exits 0 after every task (once per worktree, ran `mdbook-mermaid install docs/` first)

## Task Commits

1. **Task 1: Sweep tool-integration.md and paladin-configuration.md against the 0.8.0 tree** - `683aa58f` (fix)
2. **Task 2: Sweep output-formatting.md against the 0.8.0 tree** - `d77e7e8e` (fix)

**Plan metadata:** (this commit)

## Files Created/Modified

- `docs/src/user-guides/tool-integration.md` - Fixed Arsenal/ArmamentCall/ArmamentResult fields, the ArsenalPort trait signature, six nonexistent ArsenalError variants, MCPStdioAdapter builder fabrication, arsenal.mcp_servers YAML key, fabricated example citations
- `docs/src/user-guides/paladin-configuration.md` - Fixed 16 missing `.await`, a fabricated config-file-loading section, a nonexistent PaladinError variant, a nonexistent validate() call, add_armament()
- `docs/src/user-guides/output-formatting.md` - Fixed the Herald trait shape, removed two fabricated built-in formatters, replaced fabricated builder chains with the real Config-struct API, fixed the primitives-establishing custom-Herald example
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md` - Appended three evidence-bearing verdict rows (modified)

## Decisions Made

- `PaladinBuilder::add_armament()` does not exist anywhere in the tree (used ~5 times across tool-integration.md and paladin-configuration.md). Rather than rewriting every tool-attachment call site's registry-population mechanics — which would exceed D-12's no-restructure guard — added one explanatory wiring note documenting the real split (`with_arsenal_registry()` attaches metadata-only `Armament`s; a custom `ArsenalPort` implementor is wired through `PaladinExecutionService`'s constructor, which `PaladinBuilder` does not expose) and left the shorthand call sites governed by that note.
- `output-formatting.md`'s fabrication was comprehensive enough (nearly the whole file) that a full line-by-line rewrite of every one of ~15 custom-Herald illustrative examples was judged disproportionate to D-12. Fully rewrote the trait sketch, the three real built-in formatters, and the primitives-establishing custom example; cross-referenced two broken sketches (`XmlHerald`, `CsvHerald`) to an already-existing complete implementation in `examples/herald_custom_formatter.rs`; left ten further illustrative custom-Herald names as partial sketches governed by one scope note, recorded explicitly in the verdict row rather than silently.
- The task's own literal `grep -rq "$h" crates/paladin-herald/src/` one-liner over every `[A-Z][A-Za-z]*Herald` match cannot resolve to zero `UNRESOLVED` for legitimately user-authored illustrative custom-Herald names (they were never claimed as shipped by the crate). The verdict row's per-name classification satisfies the acceptance criteria's own second clause ("or has been corrected/removed, with the per-name result stated in the verdict row") where the bare one-liner cannot.

## Deviations from Plan

None beyond the plan's own explicitly-scoped auto-fix mandate — every correction above is a Rule 1 (bug: code/config that doesn't match the live tree) fix within the plan's own `<action>` instruction to run the signal battery and the file's own structured claim surface, correcting only what the checks found. No architectural changes, no new dependencies, no files touched outside the three named guides and the verdict record.

## Issues Encountered

- The sandboxed Bash tool rejected several multi-statement / loop-containing commands as "too complex to verify [they stay] inside the worktree," consistent with 16-02's note — worked around by splitting `for`-loops into individual single-purpose commands (e.g., per-path `test -f` calls instead of a loop).
- `output-formatting.md`'s fabrication depth exceeded the other two files in this plan by a wide margin (effectively the whole "Built-in Formatters" section plus most of "Custom Formatters" and every downstream advanced-pattern section used a wrong `Herald` trait shape). Resolved per the scope-boundary decision documented above rather than a full rewrite.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The D-09 verdict record now carries six settled `user-guides/` rows (`cicd.md` from 16-01; `orchestration.md`, `maneuver-flow-dsl.md`, `memory-management.md` from 16-02; `tool-integration.md`, `paladin-configuration.md`, `output-formatting.md` from this plan) — the whole `docs/src/user-guides/` group (Milestone 11 task 6.0) is closed. Seven still-`pending` rows remain for 16-04 (`deployment/`) and 16-05 (`operations/`).
- `output-formatting.md`'s deliberately-scoped gap (ten illustrative custom-Herald names left as partial sketches governed by one scope note) is documented in this SUMMARY and in the verdict row's Findings cell — any future plan revisiting that file should check there first.
- No blockers for 16-04 or 16-05.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

All 4 claimed files verified present on disk (`docs/src/user-guides/tool-integration.md`,
`docs/src/user-guides/paladin-configuration.md`, `docs/src/user-guides/output-formatting.md`,
this SUMMARY). Both commit hashes (`683aa58f`, `d77e7e8e`) verified present in
`git log --oneline --all`.
