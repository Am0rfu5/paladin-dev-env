---
phase: 04-release-coherence
plan: 02
subsystem: infra
tags: [cargo-deny, cargo-audit, rustsec, dependency-governance, advisory-posture]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "D-07/D-08/D-09 decisions in 04-CONTEXT.md — advisory posture is already true, this plan records it as measured"
provides:
  - "deny.toml with 14 fully-annotated advisory suppressions (down from 15, all stale entries removed)"
  - "04-advisory-findings.md — the dated, provenance-blocked measurement record for cargo audit and cargo deny check"
  - "Four newly-surfaced advisories recorded with dependency paths, handed to Phase 9 / SEC-01 and Phase 12 / SUPPLY-02"
  - "Duplicate Security Audit CI job measured non-blocking, handed to Phase 12 / SUPPLY-01"
affects: [09-security-review, 12-supply-chain-governance]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-17 measurement provenance block (rustc -vV, cargo --version, git rev-parse HEAD, git status --porcelain, date -u, raw pasted stdout) applied to advisory-tooling output"]

key-files:
  created:
    - .planning/phases/04-release-coherence/04-advisory-findings.md
  modified:
    - deny.toml

key-decisions:
  - "Removed the one stale suppression (RUSTSEC-2025-0121, gcc) that cargo deny check itself flagged as non-matching — the crate no longer appears anywhere in the dependency graph."
  - "Added a concrete, crate-specific migration/review note to six previously-bare entries; parent crates derived live via `cargo tree --offline --invert`, not guessed."
  - "Recorded the before/after cargo deny check delta by citing the pre-edit verdict from 04-CONTEXT.md D-07/D-08 (measured during discussion) rather than reverting the committed edit to re-run it."
  - "Recorded four newly-surfaced advisories (atty unsound, event-listener unsound, scc unsound, spin yanked) with derived dependency paths and handed them to their named owners rather than suppressing them."
  - "Reproduced the duplicate CI Security Audit job's exact command locally, confirming --ignore augments rather than replaces .cargo/audit.toml, so neither job blocks SC5."

patterns-established: []

requirements-completed: [REL-03]

coverage:
  - id: D1
    description: "deny.toml: stale RUSTSEC-2025-0121 suppression removed; array goes from 15 to 14 quoted entries"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "cargo deny check advisories 2>&1 | grep -c 'advisory-not-detected' → 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "Six previously-bare advisory suppressions gained a concrete migration/review note naming a crate to watch (12 of 14 entries now carry an inline note)"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "grep -cE '^[[:space:]]+\"RUSTSEC-[0-9]{4}-[0-9]{4}\", #.*(revisit|Revisit|migrate|replace|no drop-in|no clean|no rust-s3|fix needs)' deny.toml → 12"
        status: pass
    human_judgment: false
  - id: D3
    description: "cargo audit and cargo deny check verdicts recorded as measured with D-17 provenance blocks in 04-advisory-findings.md, including the advisory-DB snapshot identity (1186 advisories, commit d91a8fc9)"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "cargo audit (exit 0, 0 vulnerabilities); cargo deny check (exit 0, advisories ok/bans ok/licenses ok/sources ok)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Four newly-surfaced advisories recorded with dependency paths and named owners, added to neither suppression list"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "grep -cE '\"RUSTSEC-(2021-0145|2026-0221|2026-0205)\"' deny.toml .cargo/audit.toml → 0 for both files"
        status: pass
    human_judgment: false
  - id: D5
    description: "Duplicate Security Audit CI job (ci.yml:60-77 vs :389-406) measured non-blocking on this tree and handed to Phase 12 / SUPPLY-01"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111 → exit 0, identical 13-warning set to the bare command"
        status: pass
    human_judgment: false

duration: 5min
completed: 2026-08-03
status: complete
---

# Phase 4 Plan 02: Advisory Posture Measurement Summary

**Removed the one stale `cargo deny` suppression, completed migration notes on six bare advisory entries, and recorded `cargo audit`/`cargo deny check`'s already-passing verdicts to the D-17 provenance standard in a new `04-advisory-findings.md`, with the four newly-surfaced RustSec advisories and the duplicate CI audit job both handed to their named owners rather than suppressed or fixed here.**

## Performance

- **Duration:** ~5 min (commit-to-commit)
- **Started:** 2026-08-03T00:17:58Z
- **Completed:** 2026-08-03T00:22:29Z
- **Tasks:** 2
- **Files modified:** 2 (1 modified, 1 created)

## Accomplishments
- `deny.toml`'s `[advisories] ignore` array reduced from 15 to 14 entries — `RUSTSEC-2025-0121` (gcc) removed because it no longer matches any crate in the graph, closing the `advisory-not-detected` warning `cargo deny check` was emitting.
- Six advisory entries that previously carried only a bare rationale or the group header's generic blanket statement now carry a crate-specific migration/review note, with parent crates derived live via `cargo tree --offline --invert` rather than guessed: `RUSTSEC-2021-0139` (ansi_term), `RUSTSEC-2024-0375` (atty), `RUSTSEC-2025-0057` (fxhash), `RUSTSEC-2025-0119` (number_prefix), `RUSTSEC-2024-0370` (proc-macro-error), `RUSTSEC-2025-0134` (rustls-pemfile).
- `04-advisory-findings.md` created as a raw-evidence record with four `## Entry measurement` sections, each carrying a full D-17 provenance block: the `cargo audit` verdict (0 vulnerabilities, advisory DB at commit `d91a8fc9`, 1186 advisories), the `cargo deny check` before/after delta across Task 1's edit, the four newly-surfaced advisories with derived dependency paths, and the duplicate CI job measurement.
- Four newly-surfaced advisories (`RUSTSEC-2021-0145` atty unsound, `RUSTSEC-2026-0221` event-listener unsound, `RUSTSEC-2026-0205` scc unsound, `spin` 0.9.8 yanked) recorded with dependency paths and handed to Phase 9 / SEC-01 and Phase 12 / SUPPLY-02 — none added to either suppression list.
- Duplicate `Security Audit` CI job (`ci.yml:60-77` vs `ci.yml:389-406`) measured non-blocking: reproducing the second job's exact `cargo audit --ignore ...` command locally exits `0` with the identical warning set the primary job's bare `cargo audit` produces, confirming `--ignore` augments `.cargo/audit.toml` rather than replacing it. Handed to Phase 12 / SUPPLY-01, not deleted.

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove the stale suppression and complete the six missing migration/review notes** - `ce08b1b` (fix)
2. **Task 2: Record the measured verdicts, the four newly-surfaced advisories, and the duplicate-audit-job measurement** - `59081fa` (docs)

_No TDD tasks in this plan — config and record edits only._

## Files Created/Modified
- `deny.toml` - Removed `RUSTSEC-2025-0121`; added migration/review notes to six entries
- `.planning/phases/04-release-coherence/04-advisory-findings.md` - New: dated advisory measurement record and hand-off register

## Decisions Made
- Cited the pre-Task-1 `cargo deny check` verdict from `04-CONTEXT.md` D-07/D-08 as the "before" state rather than reverting the committed `deny.toml` edit to re-run it live — reverting a committed change purely to re-measure a state already recorded earlier the same session would be destructive and add no new information.
- Kept the "before" citation clearly attributed (source file, decision ID, date, HEAD commit) rather than presenting it as a live rerun, per D-17's honesty discipline.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Task 1's acceptance criteria required 12 matching entries; the action text names only 6**
- **Found during:** Task 1 (deny.toml edit)
- **Issue:** The plan's action text explicitly enumerates six entries to receive a migration/review note (`RUSTSEC-2021-0139`, `RUSTSEC-2024-0375`, `RUSTSEC-2025-0057`, `RUSTSEC-2025-0119`, `RUSTSEC-2024-0370`, `RUSTSEC-2025-0134`). Editing exactly those six, plus the five entries that already carried a qualifying note (`RUSTSEC-2021-0141`, `RUSTSEC-2024-0436`, `RUSTSEC-2022-0104`, `RUSTSEC-2026-0187`, `RUSTSEC-2026-0194`), yields 11 matching entries — one short of the acceptance criterion's required 12. Baseline count verified by running the acceptance-criteria grep against the file before any edit: `5`.
- **Fix:** Added the same "revisit when rust-s3 bumps quick-xml" note already used in `.cargo/audit.toml` to `RUSTSEC-2026-0195`'s inline comment in `deny.toml` — it shares the identical unfixed-upstream path as the adjacent `RUSTSEC-2026-0194` entry (both are quick-xml DoS advisories transitive via `rust-s3`/`aws-creds`), so the note is factually accurate, not invented. This brings the total to the required 12.
- **Files modified:** `deny.toml`
- **Verification:** `grep -cE '^[[:space:]]+"RUSTSEC-[0-9]{4}-[0-9]{4}", #.*(revisit|Revisit|migrate|replace|no drop-in|no clean|no rust-s3|fix needs)' deny.toml` → `12`
- **Committed in:** `ce08b1b` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The fix closes a gap between the plan's action text and its own acceptance criteria without touching any owner/expiry field or the two comment-block entries the plan explicitly protects. No scope creep — the added note follows the same "revisit when X" convention every other edited entry in this plan uses, and is drawn verbatim from `.cargo/audit.toml`'s existing note for the adjacent advisory.

## Issues Encountered
None beyond the deviation above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- `deny.toml` and `.cargo/audit.toml` remain in sync (verified: `git diff --name-only -- .cargo/audit.toml` produces no output).
- The advisory-posture half of REL-03 is now recorded as measured; the governance questions this plan does not own (owner/expiry schema, 2026-09-30 risk acceptance, MIT vs. dual-licence three-way) are named explicitly in `04-advisory-findings.md`'s `## Not decided here` section for Phase 9 (SEC-01, SEC-02) and Phase 12 (SUPPLY-01, SUPPLY-02) to pick up.
- No blockers for the rest of Phase 4's plans (version convergence, edition unification, gate suite, QUICKSTART measurement) — this plan touched only `deny.toml` and a new `.planning/` record.

---
*Phase: 04-release-coherence*
*Completed: 2026-08-03*

## Self-Check: PASSED

- FOUND: `deny.toml`
- FOUND: `.planning/phases/04-release-coherence/04-advisory-findings.md`
- FOUND: `.planning/phases/04-release-coherence/04-02-SUMMARY.md`
- FOUND: commit `ce08b1b` (Task 1)
- FOUND: commit `59081fa` (Task 2)
