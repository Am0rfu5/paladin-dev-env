---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 04
subsystem: docs
tags: [ledger, mdbook, requirements-traceability, milestone-11]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "plan 13-01's ledger scaffold (.planning/ledgers/milestone-09-12.md), all 120 row stubs and head notes"
provides:
  - "20 cited Verdict cells for Milestone 11 (Epics 1-2, 3, 4, 6, and 5&7) in .planning/ledgers/milestone-09-12.md"
  - "The corpus's only genuinely-open checkbox count (Milestone 11's 26 open items) recorded as Verified open (content), carried to Phase 16 / DOCS-01, with all fourteen target files confirmed present"
  - "A measured mdbook build docs/ baseline (exit 101, two pre-existing unrelated broken-link errors) attributed correctly, not to this phase"
affects: [16-documentation-currency-and-the-architecture-gap]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-00e evidence bar: every verdict backed by a file:line citation plus a command run this session"]

key-files:
  created: []
  modified: [.planning/ledgers/milestone-09-12.md]

key-decisions:
  - "REQ-architecture-docs-update kept separate from REQ-arch-doc-modernization (D-00f): the former is verified against the five docs/src/architecture/*.md pages, the latter against the appendix relocation — not merged despite touching adjacent subject matter"
  - "Milestone 11's 26 open items recorded as Verified open (content), not converted into a task; all fourteen target files (6 user-guide + 8 deployment/operations) confirmed present this session, carried to Phase 16 / DOCS-01 per D-10/ORCH-02"
  - "REQ-deployment-topologies-section records the ADR-0039 placement pointer only (overview.md as 'the single source of routing') without stating the Garrison/Arsenal limitation itself, deferring to plan 13-09's blocking human checkpoint"
  - "mdbook build docs/ requires mdbook-mermaid install docs (a gitignored, generated-asset prerequisite) before it can run in a fresh worktree checkout; ran it this session to reproduce the documented exit-101 baseline exactly (docker.md:118, tool-integration.md:324)"

requirements-completed: [ORCH-01, ORCH-02]

coverage:
  - id: D1
    description: "Milestone 11 Epics 1-2 and 3 (11 rows): mdbook scaffold, chapter hierarchy, CI/Pages deploy, migration log, linkcheck repair, doc-example compile gate, getting-started/architecture/user-guides/deployment-operations/api-reference rewrites"
    requirement: "ORCH-01"
    verification:
      - kind: manual_procedural
        ref: "awk '/^### Milestone 11 Epics 1-2/,/^### Milestone 11 Epic 4/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 11; grep -c 'pending' in range -> 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 11 Epics 4, 6, 5&7 (9 rows): three new-subsystem guides, crate-map/feature-flag reference, deployment-topologies routing pointer to ADR-0039, mdbook final review, doc-version-sync, README landing page, v0.5.0 release"
    requirement: "ORCH-02"
    verification:
      - kind: manual_procedural
        ref: "awk '/^### Milestone 11 Epic 4/,/^### Milestone 12 Epic 1/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 9; grep -c 'single source of routing' -> 1; grep -c 'ADR-0039' -> 1"
        status: pass
    human_judgment: false

duration: 25min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 04: Milestone 11 Ledger Derivation (Epics 1-2, 3, 4, 6, 5&7) Summary

**Derived 20 cited Milestone 11 ledger verdicts, re-ran `mdbook build docs/` to reproduce the exit-101 baseline verbatim, and confirmed all fourteen target files behind Milestone 11's 26 open checkbox items exist — carrying the content-currency question to Phase 16 / DOCS-01 rather than settling it here.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-10T17:04:00Z (approx.)
- **Completed:** 2026-08-10T17:17:00Z
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`)

## Accomplishments
- Replaced all 20 `pending`/`run-5 input (not yet re-derived)` Verdict cells in Milestone 11 Epics 1-2, 3, 4, 6, and 5&7 with fresh, cited verdicts (`file:line` + an exerciser per D-03/D-00e)
- Reproduced the `mdbook build docs/` exit-`101` baseline this session, after running the required `mdbook-mermaid install docs` prerequisite (a gitignored, generated-asset step every worktree checkout needs — `docs/mermaid.min.js`/`docs/mermaid-init.js` are absent from a fresh clone by design, per `.gitignore:21-23`), confirming the same two pre-existing errors 13-RESEARCH.md recorded: `deployment/docker.md:118` (link escapes mdbook's root) and `user-guides/tool-integration.md:324` (incomplete rustdoc-style link), both unrelated to this phase
- Recorded Milestone 11's 26 open checkbox items (`tasks-content-rewrite.md` tasks 6.0, 7.0, 1.2) as `Verified open (content)`, confirming all fourteen target files (6 user-guide + 8 deployment/operations) exist this session, and carried the content-currency question to **Phase 16 / DOCS-01** without converting any item into a task (D-10, ORCH-02)
- Kept `REQ-architecture-docs-update` (the five `docs/src/architecture/*.md` pages) separate from `REQ-arch-doc-modernization` (plan 13-07's appendix-relocation row), per D-00f, while finding and recording that `overview.md` itself lacks the Mermaid diagram FR-7 specifies (the dependency graph exists instead in `crate-map.md`)
- Recorded `REQ-deployment-topologies-section`'s pointer to ADR-0039 (`overview.md` as the "single source of routing", sourced verbatim from the PRD's own FR-8 header) without stating the Garrison/Arsenal limitation or pre-empting plan 13-09's blocking human checkpoint (D-15, T-13-12)
- Re-ran and corrected `REQ-doc-version-sync`'s current-state citation (`Cargo.toml:34` `version = "0.7.0"`; tags `v0.7.1`/`v0.7.0`/`v0.5.1`/`v0.5.0`/`v0.4.3`) rather than transcribing ORCH-05's stale `0.6.0`/`v0.5.1` figures (D-18)

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 11 Epics 1-2 and 3 (11 rows) with the 26-open verdict** - `c4e3c2a` (docs)
2. **Task 2: Derive Milestone 11 Epics 4, 6 and 5&7 (9 rows)** - `3338cbe` (docs)

_Note: this plan makes zero `.rs` and zero `docs/` file changes (records-only, D-19); both commits are `docs` type touching only the ledger._

## Files Created/Modified
- `.planning/ledgers/milestone-09-12.md` - Verdict cells replaced in place for the 20 Milestone 11 requirement IDs owned by this plan; no row inserted, deleted, or reordered; row count unchanged at 120

## Decisions Made
- `REQ-architecture-docs-update` and `REQ-arch-doc-modernization` kept as two separate rows/verdicts (D-00f) even though both touch architecture documentation — the former is the five-page chapter, the latter the appendix relocation
- Milestone 11's 26 open items recorded as `Verified open (content)`, explicitly not converted into a task or phase here — carried to Phase 16 / DOCS-01 per D-10/ORCH-02
- `REQ-deployment-topologies-section` records only the ADR-0039 placement pointer, not the limitation itself, respecting plan 13-09's blocking human checkpoint boundary
- `mdbook-mermaid install docs` was run this session as a required, idempotent, gitignored-asset prerequisite before `mdbook build docs/` — not a phase edit (no tracked file changed; `git status --short docs/` was empty both before and after)

## Deviations from Plan

None - plan executed exactly as written. The `mdbook-mermaid install docs` step was anticipated by the plan's own read-first list (13-RESEARCH.md's documented mechanics) and produces no tracked-file change, so it is not a deviation from the record-only boundary (D-19).

## Issues Encountered
- A first `mdbook build docs/` attempt failed with `Unable to copy mermaid.min.js` because the fresh worktree checkout has no untracked files (unlike the main checkout where 13-RESEARCH.md's session had already run the install step). Resolved by running `mdbook-mermaid install docs` (a gitignored, generated-asset regeneration, confirmed via `.gitignore:21-23` and zero `git status --short docs/` output before/after), then re-running the build to reproduce the documented exit-101 baseline exactly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 20 Milestone 11 rows in this plan's section range carry cited verdicts; `grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` still returns 120
- Phase 16 / DOCS-01 inherits: the 26-open verdict with all fourteen files confirmed present, the `overview.md` architecture page's missing top-level Mermaid diagram, and the `rust-toolchain.toml` premise drift in `installation.md`
- Phase 16 / DOCS-04 inherits: the corrected `docs/src/assets/` (not `docs/assets/`) path for the README/demos cross-reference, six architecture SVGs present, no demo content, `docs/DEMOS.md` absent
- No `.rs` or `docs/` file was modified by this plan (`git diff --name-only -- '*.rs' 'docs/*' | wc -l` → `0`)

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
