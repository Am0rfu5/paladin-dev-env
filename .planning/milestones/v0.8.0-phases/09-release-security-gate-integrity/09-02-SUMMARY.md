---
phase: 09-release-security-gate-integrity
plan: 02
subsystem: infra
tags: [rustsec, cargo-audit, cargo-deny, security-governance, adr, toml]

# Dependency graph
requires: []
provides:
  - "SECURITY-EXCEPTIONS.md — root-level TOML-backed governance register, ten live suppressions (five vulnerability, five unmaintained), all eleven governance fields per row"
  - "ADR-0024 — records the register architecture, the FR-3 schema supersession, the three-advisory ratification, the owner reassignment to DF3NDR, and the 2026-09-30 renewal to per-advisory 2026-12-31 dates"
  - "In-session Cargo.lock liveness transcript proving the ten-live/four-dead suppression split, citable by plan 09-06's guard script and deletions"
affects: [09-06-supply-chain-config, 09-07-close-out, phase-10-hard-06, phase-12-supply-01-supply-02]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Register-of-record + mirrored mechanical config: one human/machine-readable governance file (TOML fenced inside Markdown, BEGIN/END delimited) is authoritative; .cargo/audit.toml and deny.toml remain the tool-native mirrors, reconciled by a guard script landing in a later plan, not by this plan"

key-files:
  created:
    - SECURITY-EXCEPTIONS.md
    - .planning/decisions/0024-rustsec-exception-governance.md
  modified: []

key-decisions:
  - "Ten live suppressions only (five vulnerability, five unmaintained) — the four Phase-8-orphaned deny.toml entries (structopt, ansi_term, atty, proc-macro-error) get no register row because they return zero hits in Cargo.lock, re-verified this session"
  - "Owner is DF3NDR (repository owner) on every row, replacing the closed-milestone team label 'Platform Security (Milestone 7)'"
  - "Review date is 2026-12-31 on every row, renewing rather than closing the 2026-09-30 acceptance, per-advisory rather than blanket"
  - "The three 2026 vulnerability advisories (RUSTSEC-2026-0187/-0194/-0195) are ratified via ADR-0024 rather than removed, each with a concrete, non-generic compensating control"
  - "ADR-0024 records the M10 Epic 2 FR-3 four-field schema supersession explicitly rather than leaving it inferred"

patterns-established:
  - "TOML-in-Markdown governance register: BEGIN/END HTML comment markers wrap a single fenced ```toml block, giving a guard script a real tomllib parse target while GitHub still renders the file as readable prose"

requirements-completed: [SEC-01]

coverage:
  - id: D1
    description: "SECURITY-EXCEPTIONS.md authored with exactly ten fully-governed rows, machine-parseable via delimited TOML block"
    requirement: "SEC-01"
    verification:
      - kind: other
        ref: "python3 tomllib parse + assertion script (10 rows, 11 non-empty fields each, 5/5 class split, owner=DF3NDR, review_date=2026-12-31 future, distinct >40-char compensating_control) — see plan 09-02-PLAN.md Task 1 <verify>"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0024 authored matching the ADR-0022/0023 shape (no frontmatter, seven headings) with a verbatim in-session Cargo.lock liveness transcript as evidence"
    requirement: "SEC-01"
    verification:
      - kind: other
        ref: "grep/diff heading-order and content assertions — see plan 09-02-PLAN.md Task 2 <verify> and <acceptance_criteria>"
        status: pass
    human_judgment: false

duration: ~15min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 2: RustSec Exception Governance Summary

**Authored SECURITY-EXCEPTIONS.md (ten-row TOML-backed governance register) and ADR-0024, establishing the governance surface ahead of the .cargo/audit.toml / deny.toml reconciliation plan 09-06 executes next.**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-08-07T23:53Z (session start, base commit `7ae7dd4`)
- **Completed:** 2026-08-08T00:05Z
- **Tasks:** 2 completed
- **Files modified:** 2 (both new)

## Accomplishments

- `SECURITY-EXCEPTIONS.md` created at repository root: prose explaining the register/mirror
  architecture, a `<!-- BEGIN/END MACHINE-READABLE REGISTER -->`-delimited fenced TOML block with
  ten `[[exception]]` tables (five `vulnerability`, five `unmaintained`), all eleven governance
  fields (`id`, `class`, `crate`, `path`, `why_present`, `why_not_fixable`, `owner`, `review_date`,
  `scope`, `compensating_control`, `revisit_condition`) non-empty on every row, owner `DF3NDR` and
  review date `2026-12-31` on all ten, and a closing note explaining both why the four
  Phase-8-orphaned advisories carry no row and why the `pdf-extract`/`pdf` feature contradiction is
  handed to Phase 10 / HARD-06 rather than answered here.
- `.planning/decisions/0024-rustsec-exception-governance.md` created, matching ADR-0022/0023's
  exact shape (no YAML frontmatter, seven H2 headings in order), recording all five governing
  decisions (register authority, schema supersession of M10 Epic 2 FR-3, ratification of the three
  2026 vulnerability advisories, owner reassignment to DF3NDR, and the per-advisory 2026-12-31
  renewal), six rejected alternatives, and a verbatim `Cargo.lock` liveness transcript run in this
  session proving the ten-live/four-dead split.
- Independently re-verified in this session (not transcribed from the corpus): `.cargo/audit.toml`
  holds five vulnerability advisories; `deny.toml` holds fourteen entries across three labelled
  classes; both `ci.yml` "Security Audit"-named jobs sit at `:61-78` (survives) and `:465-482`
  (deleted by plan 09-06); the branch-protection ruleset at
  `.github/rulesets/protect-main-branch.json:34-44` keys on the context string `"Security Audit"`,
  so deleting the duplicate job carries zero required-status-check risk.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author SECURITY-EXCEPTIONS.md — ten rows, eleven fields, one machine-readable block** - `a587e5a` (feat)
2. **Task 2: Write ADR-0024 — RustSec exception governance** - `7ee741c` (docs)

_No plan-metadata commit — this is a worktree-isolated plan; STATE.md/ROADMAP.md updates and the
final metadata commit are owned by the orchestrator after the wave merges, per this plan's
execution instructions._

## Files Created/Modified

- `SECURITY-EXCEPTIONS.md` - New root-level RustSec exception governance register (ten rows)
- `.planning/decisions/0024-rustsec-exception-governance.md` - New ADR recording the governance
  architecture, schema supersession, ratifications, owner change, and expiry renewal

## Decisions Made

- Sized the register at exactly ten rows (not the corpus's remembered fifteen, and not
  `deny.toml`'s own fourteen) after independently re-running the liveness `grep -c` check against
  `Cargo.lock` in this session rather than trusting any prior document's count — matches
  `09-CONTEXT.md` D-04/D-05 and `09-RESEARCH.md` §3 exactly.
- Wrote all ten `compensating_control` values as distinct, concrete sentences naming the actual
  reachable input path (S3 API responses, dev/test-only certificate fixtures, compile-time-only
  proc-macro, etc.) rather than a shared boilerplate sentence, per the plan's prohibition on
  governance theatre and per threat T-09-06's mitigation.
- Did not edit `deny.toml`, `.cargo/audit.toml`, `ci.yml`, or `.planning/decisions/PROMOTION.md` —
  those are plan 09-06's and the close-out's scope respectively; this plan authors the governance
  surface only, per the plan's stated wave-1 ordering rationale (a guard cannot be written against
  a register that does not exist).

## Deviations from Plan

None — plan executed exactly as written. Both tasks' automated `<verify>` commands and all listed
`<acceptance_criteria>` passed on the first attempt with no auto-fixes required.

## Issues Encountered

None. One environment note: several multi-line/compound `bash` invocations (a single `for`-loop
liveness check, a `diff <(...)` heading comparison) were rejected by the sandbox's worktree-command
complexity guard as "too complex to verify it stays inside the worktree." Each was re-run as a
sequence of single-purpose commands instead, with identical results — no functional impact, only a
larger number of tool calls.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `SECURITY-EXCEPTIONS.md` and ADR-0024 are in place for plan 09-06 (wave 2) to reconcile
  `deny.toml` (delete the four dead entries, rewrite the comment/pointer block) and
  `.cargo/audit.toml` (pointer rewrite only), delete the duplicate `ci.yml:465-482` job, and land
  `scripts/check-advisory-register.sh` against this register's exact row set and schema.
- Phase 10 / HARD-06 has the `crates/paladin-content/Cargo.toml:18,41` finding as an input; this
  plan explicitly does not answer whether PDF extraction is a supported capability.
- Phase 12 / SUPPLY-01 and SUPPLY-02 will be handed closure notes citing this plan's commits once
  plan 09-06 and the phase close-out (09-07) land; no action needed from this plan's artifacts
  alone.
- No blockers for wave 2.

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*

## Self-Check: PASSED

- FOUND: SECURITY-EXCEPTIONS.md
- FOUND: .planning/decisions/0024-rustsec-exception-governance.md
- FOUND: .planning/phases/09-release-security-gate-integrity/09-02-SUMMARY.md
- FOUND: commit a587e5a
- FOUND: commit 7ee741c
- FOUND: commit 4d2b19c
