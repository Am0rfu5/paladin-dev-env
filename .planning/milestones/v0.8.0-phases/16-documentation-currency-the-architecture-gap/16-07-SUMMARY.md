---
phase: 16-documentation-currency-the-architecture-gap
plan: 07
subsystem: docs
tags: [rustdoc, cargo-doc, missing_docs, lint, ci-gate, adr-amendment]

requires:
  - phase: 16-01
    provides: phase scaffolding and the DOCS-03 requirement scope
provides:
  - Zero-warning `cargo doc --workspace --no-deps` gate (ci.yml:63) — was red, now green
  - A uniform `#![warn(missing_docs)]` posture across all ten library crates + facade
  - An explicit, reasoned disposition for `crates/doc-examples` (out of scope)
  - ADR-0033 amended in place with a dated correction to its Finding 1 claim
affects: [DOCS-04, ARCH-05, any future phase touching paladin-web/paladin-battalion/paladin-herald doc comments]

tech-stack:
  added: []
  patterns:
    - "De-link rather than widen visibility: unresolved/private-item rustdoc links are fixed by dropping the `[...]` doc-link syntax to a plain code span, never by making a private item public just to satisfy a lint"
    - "Redundant explicit link targets fixed by dropping the explicit path, keeping the label (rustdoc's own suggested fix)"

key-files:
  created:
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-GATE-EVIDENCE.md
  modified:
    - crates/paladin-web/src/agent_auth.rs
    - crates/paladin-web/src/agent_registry.rs
    - crates/paladin-web/src/delivery_controller.rs
    - crates/paladin-web/src/openapi.rs
    - crates/paladin-web/src/agent_controller.rs
    - crates/paladin-web/src/app.rs
    - crates/paladin-battalion/src/in_memory_registry.rs
    - crates/paladin-herald/src/lib.rs
    - src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs
    - src/infrastructure/web/agent_host.rs
    - .planning/decisions/0033-cargo-doc-warning-bar.md

key-decisions:
  - "crates/doc-examples stays attribute-less by disposition, not oversight: publish=false, doctest=false, its public surface is mdBook-included ANCHOR regions, so requiring rustdoc prose on example fixtures would generate documentation no reader visits"
  - "ADR-0033 amended in place (D-00d): the Finding 1 claim that all ten crates carried #![warn(missing_docs)] is retained verbatim with a dated correction noting paladin-herald did not, until this plan"

requirements-completed: [DOCS-03]

coverage:
  - id: D1
    description: "All 20 pre-existing cargo doc warnings cleared, re-derived fresh (not copied from ADR-0033)"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q \"warning:\" /tmp/doc-output.txt (ci.yml:63 verbatim)"
        status: pass
    human_judgment: false
  - id: D2
    description: "paladin-herald's missing_docs posture flipped to warn, matching the other nine library crates and the facade, at zero new-warning cost"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc -p paladin-herald --no-deps 2>&1 | grep -c \"warning:\" -> 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "ADR-0033 amended in place with a dated 2026-08-24 correction, original text retained, doc-examples disposition recorded"
    verification: []
    human_judgment: true
    rationale: "Prose-quality judgment on whether the ADR amendment reads as an honest correction rather than a rewrite is a human call, not a mechanical check"

duration: 13min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 07: cargo doc zero-warning bar applied Summary

**Cleared all 20 pre-existing `cargo doc` warnings across four crates by de-linking unresolved/private-item doc links (never by widening visibility), then flipped `paladin-herald`'s `missing_docs` posture to match its nine sibling crates at zero new-warning cost — the CI gate at `ci.yml:63` goes from red to green with nothing suppressed.**

## Performance

- **Duration:** ~13 min
- **Started:** 2026-08-24T12:40:40Z
- **Completed:** 2026-08-24T12:53:34Z
- **Tasks:** 2
- **Files modified:** 11 (10 `.rs`/ADR files + 1 new evidence file)

## Accomplishments

- Re-derived the 20-warning list fresh via `cargo doc --workspace --no-deps` rather than trusting ADR-0033's citations — found one genuine drift (`crates/paladin-web/src/agent_auth.rs`'s `AuthPort` link is at line 8 today, ADR-0033 cited line 7), confirming Pitfall 1's prediction
- Fixed all 20 warnings by class: 14 unresolved intra-doc links de-linked to plain code spans (11 in `//!` module docs where rustdoc's own scope-resolution quirk applies even to imported types, 3 crate-name mentions mistaken for item links), 3 private-item links de-linked rather than made public, 2 redundant explicit link targets dropped (rustdoc's own suggested fix), 1 unclosed HTML tag (`Arc<Paladin>`) wrapped in a code span
- Flipped `crates/paladin-herald/src/lib.rs:20` from `#![allow(missing_docs)]` to `#![warn(missing_docs)]`, byte-identical in form to `crates/paladin-storage/src/lib.rs:18` — measured zero new warnings twice (once in `16-RESEARCH.md`'s M-07, once by this plan's own `cargo doc -p paladin-herald --no-deps` run)
- Recorded `crates/doc-examples`'s missing-docs disposition explicitly (out of scope, with reason) rather than leaving it as a silent eleventh case
- Amended ADR-0033 in place with a dated 2026-08-24 note: the original Finding 1 sentence is retained verbatim, with a correction that it was inaccurate when written (`paladin-herald` was the one exception until this plan)
- Ran the exact `ci.yml:63` gate command four times this session (before, after task 1, herald-scoped after task 2's flip, full workspace final) and recorded all of them verbatim in `16-DOCS-03-GATE-EVIDENCE.md`

## Task Commits

Each task was committed atomically:

1. **Task 1: Re-derive the warning list fresh and clear all 20 by class** - `844aac0` (fix)
2. **Task 2: Make the missing-docs bar uniform, disposition doc-examples, and amend ADR-0033 in place** - `67b8568` (feat)

_Note: no test/tdd split — this plan's tasks are `type="auto"`, not `tdd="true"`._

## Files Created/Modified

- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-GATE-EVIDENCE.md` - New; verbatim before/after `ci.yml:63` output across all four runs this session, with the re-derived warning list and its one confirmed drift from ADR-0033
- `crates/paladin-web/src/agent_auth.rs` - De-linked `[`AuthPort`]` → `` `AuthPort` `` in the module doc
- `crates/paladin-web/src/agent_registry.rs` - De-linked 4 `[`Paladin`]`/`[`PaladinExecutorPort`]` occurrences in the module doc
- `crates/paladin-web/src/delivery_controller.rs` - De-linked 4 handler-name links in the module-doc routing table
- `crates/paladin-web/src/openapi.rs` - De-linked `[`build_openapi`]` and `[`docs_router`]` in the module doc
- `crates/paladin-web/src/agent_controller.rs` - Dropped the redundant explicit target on `[`JobRecord`]` at line 651
- `crates/paladin-web/src/app.rs` - Dropped the redundant explicit target on `[`agent_router`]` at line 69
- `crates/paladin-battalion/src/in_memory_registry.rs` - De-linked 2 crate-name mentions (`paladin-core`, `paladin-ports`); wrapped `Arc<Paladin>` in a code span to fix the unclosed-HTML-tag warning
- `crates/paladin-herald/src/lib.rs` - De-linked the feature-gated `[`TableHerald`]` reference (task 1); flipped `#![allow(missing_docs)]` to `#![warn(missing_docs)]` (task 2)
- `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs` - De-linked `[`BearerToken::expose_secret`]` (private method) in the module doc
- `src/infrastructure/web/agent_host.rs` - De-linked both `[`build_agent`]` references (`pub(crate)` function referenced from public docs)
- `.planning/decisions/0033-cargo-doc-warning-bar.md` - Amended in place with a dated 2026-08-24 correction note; original Finding 1 text unedited

## Decisions Made

- **`crates/doc-examples` recorded out of scope for the `missing_docs` bar, no attribute added.** Reasoning written into the ADR amendment: it is `publish = false` and `doctest = false`, and its entire public surface is `// ANCHOR:` regions consumed by mdBook `{{#include}}` — the documentation *for* those items is the guide page that includes them, not a rustdoc page any reader visits. Requiring `missing_docs` compliance there would generate prose with no audience.
- **Module-doc (`//!`) intra-doc links to imported types were de-linked, not fixed by import reordering.** Even though `Paladin`/`PaladinExecutorPort` etc. are legitimately `use`-imported into scope in the affected files, rustdoc's link resolver treats links inside multi-line `//!` module-doc comments differently and still reports them unresolved — a documented, confirmed-this-session rustdoc behavior (RESEARCH.md Pitfall 1). De-linking to a plain code span was the correct mechanical fix per the plan's Class 1 guidance, not a workaround.
- **Private-item links de-linked, never made public.** `BearerToken::expose_secret` and `build_agent` stay `pub(crate)`/private; their doc mentions became plain code spans. Widening visibility to satisfy a doc link would have changed the public API surface, which `.project/current-exports.txt` tracks as a baseline — explicitly prohibited by the plan.

## Deviations from Plan

None - plan executed exactly as written. One clarification worth recording: to keep the task-1 and task-2 gate evidence cleanly separable, `paladin-herald`'s `missing_docs` attribute was briefly reverted to `allow` mid-execution (after an initial combined run showed 0 warnings with both fixes already applied) so a clean task-1-only run could be captured before the task-2 flip was re-applied and re-verified. This is a verification-sequencing detail, not a deviation from the plan's specified changes — both task-1 and task-2 file diffs match the plan exactly.

## Issues Encountered

- A cold `cargo doc --workspace --no-deps` run took ~6m17s the first time (matching the plan's "~2 minutes... has already timed out a pre-commit hook" warning, actually longer); subsequent incremental runs completed in 12-17s. Ran with `run_in_background: true` and monitored via the Read tool on the background task's output file rather than blocking on a single long-running foreground call.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `ci.yml:63`'s `lint` job's doc-warning gate is green against the current tree; no further `.rs` doc-comment work is owed by this plan.
- ADR-0033's residue (Finding 2's "the tree does not currently pass it") is now stale by outcome — a future reader of ADR-0033 sees both the original claim and this plan's dated correction, per D-00d.
- No workflow file was touched; DOCS-03's "adds the CI gate" clause remains satisfied by the pre-existing `ci.yml:63`, not by this plan (D-00u), which only proves the gate green against the fixed tree.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-GATE-EVIDENCE.md`
- FOUND: commit `844aac0` (Task 1)
- FOUND: commit `67b8568` (Task 2)
