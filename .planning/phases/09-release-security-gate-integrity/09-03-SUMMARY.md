---
phase: 09-release-security-gate-integrity
plan: 03
subsystem: infra
tags: [docker, cargo-chef, dockerfile, build-cache, adr]

# Dependency graph
requires: []
provides:
  - "Dockerfile.chef planner stage with structural (not enumerated) crate coverage"
  - "ADR-0027 recording the M7 Epic 2 FR-01 supersession"
affects: [10-milestone-7-8-ledger, 09-05-dockerfile-chef-label]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dockerfile planner-stage cache tightness delivered via recipe.json content + cross-stage COPY --from, not per-manifest enumeration"

key-files:
  created:
    - .planning/decisions/0027-dockerfile-chef-planner-stage.md
  modified:
    - Dockerfile.chef

key-decisions:
  - "Deleted Dockerfile.chef's nine-manifest planner-stage enumeration rather than adding paladin-herald's line (D-16 primary branch, ADR-0027)"
  - "M7 Epic 2 FR-01 superseded under D-00b precedence: an enumerated list is structurally the defect SEC-05 names, not the one missing crate"
  - "Caching claim recorded as established-from-cargo-chef-documentation, not measured — Docker is absent from this environment"

patterns-established:
  - "Docker planner-stage isolation claims must cite the content-addressed cross-stage COPY --from mechanism, not planner-stage layer-cache status, which a later full-tree COPY already dominates"

requirements-completed: [SEC-05]

coverage:
  - id: D1
    description: "Dockerfile.chef planner stage's nine-manifest enumeration deleted; crate coverage is now structural (COPY crates ./crates covers any crate count)"
    requirement: "SEC-05"
    verification:
      - kind: other
        ref: "grep -c 'COPY crates/paladin' Dockerfile.chef (returns 0); grep -c 'cargo chef prepare' Dockerfile.chef (returns 1); git diff -- Dockerfile.chef | grep -cE '^\\+(COPY|RUN)' (returns 0)"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0027 records the FR-01 supersession, D-16 branch taken, both upstream cargo-chef citations, and the not-measured evidence disclaimer"
    requirement: "SEC-05"
    verification:
      - kind: other
        ref: "diff of '## ' headings against 0022/0023's seven-heading shape (no output = match); grep -c '(rejected)' returns 3; grep -qx 'must change' exits 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "Confirmed Dockerfile and Dockerfile.server carry no per-crate manifest enumeration, closing SEC-05 across the whole Docker surface"
    verification:
      - kind: other
        ref: "grep -c 'COPY crates/paladin' Dockerfile Dockerfile.server (returns 0 for both)"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 3: Dockerfile.chef Planner-Stage Supersession Summary

**Deleted Dockerfile.chef's nine-manifest planner-stage enumeration (the structural defect SEC-05
names) and recorded the M7 Epic 2 FR-01 supersession as ADR-0027, citing cargo-chef's own upstream
documentation rather than a measurement Docker's absence here makes impossible.**

## Performance

- **Duration:** ~20 min
- **Completed:** 2026-08-08T00:03:14Z
- **Tasks:** 2
- **Files modified:** 2 (1 modified, 1 created)

## Accomplishments

- Deleted the nine per-crate `COPY crates/paladin-*/Cargo.toml ...` lines at
  `Dockerfile.chef:25-33`, leaving the planner stage's crate coverage structural: `COPY crates
  ./crates` (unchanged) now covers any crate count with zero enumeration to go stale.
- Rewrote the planner-stage comment to describe the mechanism that actually delivers caching —
  `recipe.json`'s manifest-and-lockfile-only content plus the builder stage's content-addressed
  cross-stage `COPY --from` — rather than the per-manifest isolation the deleted lines never
  delivered (confirmed: a strictly later `COPY crates ./crates` at `:36` already dominated the
  cache decision for all ten crates before this edit).
- Confirmed `Dockerfile` and `Dockerfile.server` carry no equivalent per-crate manifest
  enumeration (`grep -c 'COPY crates/paladin' Dockerfile Dockerfile.server` → `0` for both), so
  SEC-05 closes across the whole Docker surface, not just `Dockerfile.chef`.
- Wrote `.planning/decisions/0027-dockerfile-chef-planner-stage.md` in ADR-0022/0023's exact
  seven-heading, no-frontmatter shape, recording: the FR-01 supersession under D-00b precedence;
  that this is D-16's primary (delete) branch, taken because the researcher confirmed rather than
  refuted the cargo-chef reading; three genuinely-considered rejected options (add herald's line,
  keep-enumeration-plus-guard-script, adopt upstream `COPY . .` verbatim); both upstream citations
  (cargo-chef's own README, a corroborating Rust-Dockerfile source); and an explicit
  not-measured disclaimer plus the CI-only build-twice verification that would upgrade the claim.
- Found and cited an additional piece of evidence beyond the plan's `<read_first>` list: the
  originating PRD's own §6 "Design Considerations" (`prd-production-build-infra-adaptation.md:
  208-233`) prescribes running `cargo chef prepare` *before* the full-source `COPY src`/`COPY
  crates` lines specifically "to keep the dependency cache layer tight" — the shipped
  `Dockerfile.chef` inverted that ordering (full-source copy at `:36` runs before `cargo chef
  prepare` at `:38`), which is *why* the enumeration was inert for all nine named crates, not only
  the omitted tenth. This strengthens ADR-0027's Context section and is cited there.

## Task Commits

Each task was committed atomically:

1. **Task 1: Delete the planner-stage manifest enumeration and correct its comment** - `52b1943`
   (fix)
2. **Task 2: Write ADR-0027 — Dockerfile.chef planner-stage supersession** - `d1ae033` (docs)

_No plan-metadata commit in this worktree — STATE.md/ROADMAP.md are updated centrally by the
orchestrator after the wave merges (per this plan's worktree execution mode)._

## Files Created/Modified

- `Dockerfile.chef` - Planner stage: deleted the nine per-crate manifest `COPY` lines, rewrote the
  explanatory comment; four surviving `COPY`/`RUN` instructions kept in their original relative
  order; OCI `LABEL` block at (now) `:~84` left byte-unchanged for plan 09-05.
- `.planning/decisions/0027-dockerfile-chef-planner-stage.md` - New ADR recording the FR-01
  supersession, the D-16 branch taken, both upstream cargo-chef citations, and the
  not-measured evidence scoping.

## Decisions Made

- **Took D-16's primary (delete) branch, not its fallback (guard script).** The research
  confirmed cargo-chef's own README does not enumerate manifests and that `recipe.json`'s
  content-addressed cross-stage copy is what actually delivers cache tightness — so the
  enumeration is inert dead weight, not a partially-working mechanism. Recorded in ADR-0027 with
  both upstream citations.
- **Superseded M7 Epic 2 FR-01 rather than satisfying its letter.** Adding
  `paladin-herald`'s line would have closed the visible gap while leaving the exact defect
  SEC-05's done-condition names on record — "an enumerated list that goes stale on every crate
  addition is the defect, not just the one missing line." Deletion is the only option that makes
  an eleventh crate's coverage automatic.
- **Recorded the caching claim as documentation-established, not measured.** Docker is absent
  from this environment (consistent with `04-ci-gate-deferrals.md`'s prior finding); ADR-0027
  states this explicitly in `## Context` and `## Code Locations`, and records the exact CI-only
  build-twice-and-check-CACHED measurement that would upgrade the claim, per D-19's evidence bar.
- **Left the OCI `LABEL org.opencontainers.image.licenses="MIT"` block untouched.** It shifted
  from `Dockerfile.chef:93` to approximately `:84` as a side effect of the nine-line deletion;
  plan 09-05 (SEC-02) owns editing this label in wave 2 and must re-derive the new line number
  rather than trust `09-RESEARCH.md` §4's line reference (which additionally misattributed the
  label to the runtime `Dockerfile` — confirmed this session that only `Dockerfile.chef` carries
  it).
- **Did not advance `.planning/decisions/PROMOTION.md`.** Per this plan's scope note, plan 09-07
  advances the promotion index to 0028 in the phase close-out; ADR-0027 exists as a standalone
  file only, at this point in the wave.

## Deviations from Plan

None - plan executed exactly as written. The one addition beyond the plan's `<read_first>` list
(citing the PRD's own §6 COPY-ordering prescription, which shipped code inverted) is evidence
strengthening the ADR's existing argument, not a change to the planned action, and required no
extra file edits.

## Issues Encountered

None. Both tasks' automated verification commands and acceptance criteria passed on first
execution once the `(rejected)` tag-count fix (see below) was applied.

One self-correction during Task 2: the initial ADR draft used the phrase "(rejected here, but
recorded as the genuinely considered D-16 fallback)" for the guard-script option, which the
acceptance criterion's literal `grep -c '(rejected)'` (requiring at least 3) did not count as a
match. Reworded to `(rejected) — the genuinely considered D-16 fallback, not an afterthought: ...`
to carry the exact literal tag while preserving the "genuinely considered, not omitted" meaning
the plan required. Re-verified: `grep -c '(rejected)'` now returns `3`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- SEC-05 is closed: `Dockerfile.chef`'s planner stage has no crate-count-dependent enumeration to
  go stale, and `Dockerfile`/`Dockerfile.server` are confirmed clean of the same defect.
- Phase 10 / HARD-01 can cite ADR-0027 directly when it upgrades the
  `REQ-docker-workspace-build` ledger row from "Shipped, defect → SEC-05" to a per-criterion
  verdict with "Superseded by outcome" for FR-01's enumeration clause.
- Plan 09-05 (wave 2, same file `Dockerfile.chef`) must re-derive the OCI `LABEL` block's new line
  number (approximately `:84`, was `:93`) before editing it — the nine-line deletion in this plan
  shifted it.
- The two backstop `must_haves` truths (recipe.json determinism across concurrent builds; the
  builder-stage `cargo chef cook` layer reporting `CACHED` on source-only rebuilds) remain
  unconfirmed locally by design — Docker is absent from this environment. Plan 09-07's close-out
  should restate them as CI-only, per this plan's `<verification>` block.

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
