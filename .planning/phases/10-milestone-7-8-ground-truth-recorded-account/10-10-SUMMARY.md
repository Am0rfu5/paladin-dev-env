---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 10
subsystem: docs
tags: [ledger, requirements-traceability, adr-0032, adr-0033, milestone-8, facade-8, deferred-register]

requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-01's ledger scaffold (86 row stubs), ADR-0032 (PDF unconditional / inert feature deleted), ADR-0033 (one cargo doc bar, 20-warning measured residue, DEBT-03 discharged), ADR-0028 (reconciliation authoritative, D1-D5 register, non-goal split), and 10-09's completed M8 Epic 1-3 range leaving M8 Epic 4 as the last untouched neighbor section"
provides:
  - "Milestone 8 Epic 5 (6 rows), Epic 6 (4 rows), Epic 7 (6 rows) and the five cross-milestone rows of .planning/ledgers/milestone-07-08.md fully derived, closing the ledger's 86-row inventory with zero pending stubs left anywhere"
  - "A fresh, code-verified finding: REQ-paladin-content-changelog-fix flipped to genuinely outstanding — the crate's CHANGELOG.md has no entry describing the use_cases/services rename or the six E0432 fixes FR-8 requires"
  - "A fresh, code-verified correction: REQ-web-api-baseline-changelog and REQ-api-surface-baseline-v020 both re-derived away from their stub's 'open defect' framing — DEBT-01 is Complete (Phase 8), both the tooling path and the requirement-text annotations"
affects: ["10-11"]

tech-stack:
  added: []
  patterns:
    - "Cell-replacement-only ledger fan-out, same as 10-07/10-08/10-09: two per-task commits inside disjoint epic ranges, each verified via grep -c row/section counts and git diff --numstat added==deleted before committing"
    - "Whole-file completeness sweep as the last fan-out plan: after both tasks, grep for any row-level empty Verdict/Evidence cell and any remaining 'pending — plan' marker across the entire file, not just this plan's own range"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-07-08.md

key-decisions:
  - "REQ-facade-role-lib-docs recorded superseded by outcome citing the exact ID already present in the head-of-file supersession summary table (built by plan 10-01), rather than re-deriving a second, possibly-diverging finding under the same ID"
  - "REQ-facade-readme satisfied on FR-2's literal acceptance criteria (heading, prose, seven-row What-lives-here table, dependency-flow rule, STABLE_API.md reference all present and cross-checked against the tree), with two residuals noted but not fixed: a stale nine-crate Leaf Crates table and a dead ../STABLE_API.md relative link (the file relocated under Milestone 11)"
  - "REQ-paladin-content-changelog-fix flipped from the stub's implied satisfied/Verify state to genuinely outstanding: FR-8 requires the CHANGELOG entry describe the use_cases->services rename and the six E0432 fixes; the crate's actual ### Fixed entry names neither, and a direct grep for 'E0432|rename|content/mod.rs|masked' returns zero matches"
  - "REQ-content-processing-build-gate satisfied via ADR-0032's post-deletion manifest (pdf = [] confirmed gone, grep -cE '^pdf +=' returns 0) plus a fresh cargo build --workspace --features content-processing run this session, which exits 0"
  - "REQ-api-surface-baseline-v020 satisfied (history): FR-13/14's literal ask (regenerate api_surface_current.txt/final-api.txt bearing a v0.2.0 baseline header) was executed at the time (commit a33e22d) and both files still carry that exact header. Distinguished from a different artefact's defect: the api-surface CI job's .project/current-exports.txt mechanism (DEBT-01's subject) is unrelated and is itself Complete (Phase 8) — re-deriving this away from the stub's 'open defect' framing rather than carrying it forward"
  - "REQ-web-api-baseline-changelog satisfied, also re-derived away from an 'open defect' framing: DEBT-01 closed both halves (tooling path fix, ci.yml/scripts already read .project/current-exports.txt; and the requirement-text annotation, this exact PRD carries a dated 2026-08-06 D-00c banner) before this session started"
  - "A fresh residual noted but not fixed (D-23 boundary): docs/src/api-reference/stable-api.md (the relocated STABLE_API.md) still documents the pre-rename project/current-exports.txt path in at least four places — DEBT-01's own closure scope was .project/ PRD/DOC sources, not the docs/src/ mdbook, so this chapter was never in its nine-reference count"
  - "REQ-m8-final-quality-gate's evidence cell states the tree does not clear the ratified zero-warning cargo doc gate today, carefully avoiding any phrasing the acceptance grep (gate (currently )?passes|already enforces|already satisfies) would flag as a false compliance claim, while still citing the numeric 20-warning count and four-crate split verbatim from ADR-0033"
  - "REQ-storage-nonoptional-v2 and REQ-storage-feature-flags-v1 (M7 Epic 1, plan 10-07's range) kept as two separate rows with two separate verdicts (satisfied vs superseded by outcome) describing the same commit 897e77e from two different source documents, per D-00f's primary-key rule"
  - "The three deferred-register cross-milestone rows each re-verify their register's facts against the tree independently rather than transcribing the register — D5's 17-occurrences/6-files println count reproduced exactly, D1's 49 crate::core:: importers reproduced exactly, D2/D3/D4's named files all confirmed still present — and each names its owning FACADE-0x requirement ID"

requirements-completed: [HARD-01, HARD-02, HARD-06, HARD-07]

coverage:
  - id: D1
    description: "Milestone 8 Epic 5's six facade-documentation/v0.2.0-finalization rows re-derived, including the final-quality-gate row citing ADR-0033's ratified bar and measured 20-warning residue without claiming present compliance"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 5/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 6; grep -n REQ-m8-final-quality-gate .planning/ledgers/milestone-07-08.md | grep -ciE 'gate (currently )?passes|already enforces|already satisfies' == 0; grep -c 'ADR-0033' .planning/ledgers/milestone-07-08.md >= 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 8 Epic 6's four rows re-derived, including the content-processing build gate resolved by ADR-0032's post-deletion manifest and a fresh passing cargo build, and the changelog-fix row flipped to genuinely outstanding on direct evidence"
    requirement: "HARD-06"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 6/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 4; grep -cE '^pdf +=' crates/paladin-content/Cargo.toml == 0; grep -n REQ-paladin-content-changelog-fix .planning/ledgers/milestone-07-08.md shows 'genuinely outstanding'"
        status: pass
    human_judgment: false
  - id: D3
    description: "Milestone 8 Epic 7's six rows re-derived, all satisfied, with the delivery-endpoints row citing both the app.rs import and merge lines and the handler-tests row confirmed by a passing 6/6 cargo test run"
    requirement: "HARD-01"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-web --lib delivery_controller"
        status: pass
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 7/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 6; grep -c '^| REQ-actix-removal\\|^| REQ-actix-deny-ban' .planning/ledgers/milestone-07-08.md == 2"
        status: pass
    human_judgment: false
  - id: D4
    description: "The five cross-milestone rows re-derived, with the three deferred-register rows each independently re-verifying their register facts against the tree and naming their owning FACADE-0x requirement"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "awk '/^### Cross-milestone/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 5; grep -c 'FACADE-0' .planning/ledgers/milestone-07-08.md >= 3; grep -rn 'println!\\|eprintln!\\|dbg!' src/application/services/ src/infrastructure/ | wc -l == 17"
        status: pass
    human_judgment: false
  - id: D5
    description: "Whole-file completeness: ledger row/section inventory unchanged (86/12), zero empty Verdict/Evidence cells anywhere, zero 'pending — plan' markers left in any row anywhere in the file, and no .rs file touched"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md == 86; grep -c '^### ' == 12; grep -nE '^\\| REQ-[a-zA-Z0-9_-]+ \\| *\\| ' .planning/ledgers/milestone-07-08.md (empty cells) == none; grep -n '^| REQ-.*pending' == none; git status --porcelain -- '*.rs' — empty"
        status: pass
    human_judgment: false

duration: ~90min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 10: Milestone 8 Epic 5-7 and Cross-Milestone Ledger Derivation Summary

**Derived the ledger's final 21 rows (M8 Epic 5, 6, 7, and the five DOC-carried cross-milestone entries), closing all 86 rows with zero pending stubs, and along the way corrected two of the ledger's own inherited framings on fresh code evidence: the content-changelog row is genuinely outstanding (a real gap), and the two api-surface-baseline rows are no longer open — DEBT-01 closed both halves before this session started.**

## Performance

- **Duration:** ~90 min
- **Completed:** 2026-08-08
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-07-08.md`)

## Accomplishments

- Derived all six M8 Epic 5 rows: `REQ-facade-role-lib-docs` lands `superseded by outcome`, citing the exact `REQ-*` ID the head-of-file supersession summary table already carries (FR-1's nine-crate list predates `paladin-herald`); `REQ-facade-readme` `satisfied` on FR-2's literal acceptance criteria, with two residuals (a stale nine-crate Leaf Crates table, a dead `../STABLE_API.md` link) noted but not fixed; `REQ-stable-api-v020-sync` `relocated` to the confirmed mdbook chapter; `REQ-changelog-v020-cut` and `REQ-api-surface-baseline-v020` both `satisfied (history)`, citing ADR-0029's trajectory table and the `v0.2.0`-headered baseline files respectively; `REQ-m8-final-quality-gate` `superseded by outcome`, citing ADR-0033's ratified zero-warning bar and its measured 20-warning/four-crate residue, with careful phrasing that never implies the gate is currently met.
- Derived all four M8 Epic 6 rows: `REQ-paladin-content-services-rename` and `REQ-paladin-content-readme-update` `satisfied` on direct re-verification (zero-match `use_cases` search, README content cross-check); `REQ-content-processing-build-gate` `satisfied`, citing ADR-0032's post-deletion manifest (`pdf = []` confirmed gone) plus a fresh `cargo build --workspace --features content-processing` run this session (exit 0); `REQ-paladin-content-changelog-fix` flipped to `genuinely outstanding` — FR-8's required CHANGELOG entry describing the rename and the six `E0432` fixes is simply absent from the crate's `CHANGELOG.md`, confirmed by direct read and a zero-match grep for `E0432|rename|content/mod.rs|masked`.
- Derived all six M8 Epic 7 rows, all `satisfied`: `REQ-delivery-endpoints-axum` cites both the `app.rs:24` import and the `app.rs:63` merge line to prove the delivery routes are mounted, not merely ported; `REQ-actix-removal` and `REQ-actix-deny-ban` kept as two distinct rows/verdicts per D-00f; `REQ-delivery-handler-tests` confirmed by a passing `cargo test -p paladin-web --lib delivery_controller` run (6/6, covering the success/404/400 categories FR-9 names); `REQ-web-api-baseline-changelog` and `REQ-web-quality-gate` both checked fresh (never actually verified in run-4) and found to hold, with the baseline-changelog row re-derived away from an "open defect" framing since DEBT-01 is Complete.
- Derived all five cross-milestone rows: `REQ-storage-nonoptional-v2` and `REQ-m8-reconciliation-relocations` `satisfied`, the latter independently re-measuring ADR-0028's 15-commit/net-10,252-LOC figures rather than restating them; the three deferred-register rows (`REQ-m8-deferred-items-register`, `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`) each `deferred with register`, each re-verifying its register's facts against the tree independently (D1's 49 `crate::core::` importers, D2/D3/D4's named files, D5's 17-occurrences/6-files `println!` count, the CLI `user.rs` absence, the tensorflow/`ml` absence and the `paladin-herald`/`paladin-ml` non-goal split) and each naming its owning `FACADE-0x` requirement ID by number.
- Ran the mandatory whole-file completeness sweep (this being the last fan-out plan): `grep -c '^| REQ-'` reads `86`, `grep -c '^### '` reads `12`, a grep for any row with an empty Verdict or Evidence cell returns nothing, and `grep -n '^| REQ-.*pending'` returns nothing — zero unfilled stubs remain anywhere in the ledger, in any of the four plans' ranges.
- Confirmed both task commits kept the diff balanced (12 added/12 deleted, then 13 added/13 deleted) and touched only the ledger file (`git status --porcelain` after each commit), with the M8 Epic 4 section (owned by plan 10-01) confirmed byte-unchanged by inspecting the diff hunk boundaries.

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 8 Epic 5's six rows and Epic 6's four rows** — `3dbbca7` (feat)
2. **Task 2: Derive Milestone 8 Epic 7's six rows and the five cross-milestone rows** — `6e3213a` (feat)

_No plan-metadata commit: this executor ran in worktree mode. STATE.md and ROADMAP.md are owned by the orchestrator after all wave-3 agents complete; this SUMMARY.md is committed separately per the worktree execution contract._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` — Milestone 8 Epic 5 (6 rows), Epic 6 (4 rows), Epic 7 (6 rows) and the Cross-milestone section (5 rows) Verdict/Evidence cells replaced in place; all four sections' epic notes filled. No other section touched — confirmed via `git diff` hunk boundaries on both task commits (the M8 Epic 4 section immediately above Epic 5 is unmodified).

## Decisions Made

- **`REQ-facade-role-lib-docs`** cites the head-of-file supersession summary table's own existing entry under this exact ID rather than re-deriving a second, potentially-diverging finding — the row states the same fact (FR-1's nine-crate list predates `paladin-herald`) independently, since the summary table itself is out of this plan's edit scope.
- **`REQ-facade-readme`** is `satisfied` on FR-2's literal acceptance criteria; two real content staleness issues (the Leaf Crates table's nine-crate count, the dead `STABLE_API.md` relative link) are recorded as residuals rather than changing the verdict, since neither is part of FR-2's own acceptance text.
- **`REQ-paladin-content-changelog-fix`** is the plan's sharpest fresh finding: the run-4 `Verify → HARD-01` stub was never checked, and checking it this session revealed the required CHANGELOG entry simply does not exist — the rename itself shipped correctly, but its required paper trail did not.
- **`REQ-api-surface-baseline-v020`** and **`REQ-web-api-baseline-changelog`** are both re-derived away from an "open defect, still blocked on the directory rename" framing the stubs and `10-CONTEXT.md`'s read_first notes carried forward from `intel/code-verification.md`'s 2026-07-30 measurement — DEBT-01 closed both the tooling path fix and the requirement-text annotations in Phase 8 (2026-08-06), before this session started. The two rows are kept distinct: one is a `v0.2.0`-era historical snapshot requirement (satisfied then, frozen since), the other is the `api-surface` CI job's own defect (fully closed).
- A fresh residual is recorded but not fixed, per D-23's boundary: `docs/src/api-reference/stable-api.md` (the relocated `STABLE_API.md`) still names the pre-rename `project/current-exports.txt` path in at least four places — DEBT-01's own closure scope was `.project/`-rooted PRD/DOC sources, so this mdbook chapter was never counted or fixed.
- **`REQ-m8-final-quality-gate`**'s evidence cell was worded specifically to avoid the acceptance grep's false-compliance phrasing (`gate (currently )?passes|already enforces|already satisfies`) while still stating plainly, in different words, that the tree does not clear the ratified bar today.

## Deviations from Plan

None in the Rule 1-4 sense — no code was changed, no architectural decision was made. Two evidence-driven departures from the plan's own suggested row framing, both surfaced by direct re-verification rather than carried forward from the plan text or `intel/code-verification.md`'s 2026-07-30 measurement:

**1. `REQ-api-surface-baseline-v020` / `REQ-web-api-baseline-changelog` — re-derived away from "open defect" to "satisfied"**
- **Found during:** Task 1 (Epic 5) and Task 2 (Epic 7)
- **Issue:** Both the plan's `<action>` text and the run-4-derived stub described these rows as blocked on a CI job broken by the `.project/` directory rename (commit `928c6d5`).
- **Finding:** `.planning/REQUIREMENTS.md:828-889` records DEBT-01 `[x]` Complete (Phase 8, dated 2026-08-06), both the tooling half (`ci.yml`, both scripts already read `.project/current-exports.txt`) and the requirement-text half (every named PRD, including these two, already carries a dated D-00c correction banner). Re-running the actual commands confirmed the job works today.
- **Files modified:** `.planning/ledgers/milestone-07-08.md` only (both rows' Evidence cells state the correction and cite DEBT-01's closure record).
- **Verification:** `ls -la .project/current-exports.txt` (446,377 bytes, current); `grep -n 'project/current-exports.txt' ci.yml scripts/check-api-surface.sh scripts/extract-public-api.sh` — zero pre-rename-path matches.
- **Impact on plan:** No scope creep — the plan's own evidence bar (D-00e, D-03) requires re-deriving rather than carrying forward, and this is exactly that mechanism working as intended.

**2. `REQ-paladin-content-changelog-fix` — flipped from an implied pass to `genuinely outstanding`**
- **Found during:** Task 1 (Epic 6)
- **Issue:** The run-4 stub read `Verify → HARD-01` with no indication of the likely outcome.
- **Finding:** `crates/paladin-content/CHANGELOG.md`'s `### Fixed` entry names neither the `use_cases` → `services` rename, nor the six `E0432` errors, nor the feature-gate masking FR-8 explicitly requires it to describe.
- **Files modified:** `.planning/ledgers/milestone-07-08.md` only (the row records the gap; the CHANGELOG itself is out of this plan's `files_modified` scope and was not touched).
- **Verification:** `grep -n 'E0432\|rename\|content/mod.rs\|masked' crates/paladin-content/CHANGELOG.md` — zero matches.
- **Impact on plan:** No scope creep — recording an honest `genuinely outstanding` verdict is precisely what the evidence bar exists to produce; the gap itself is a candidate for a future small-fix plan, not something this record-only phase may close (D-23 boundary — `CHANGELOG.md` is not `.rs`, but it is also not in this plan's `files_modified` list).

---

**Total deviations:** 0 auto-fixed under Rules 1-4 (record-writing phase, no code touched); 2 evidence-driven verdict corrections, both documented above.
**Impact on plan:** Both corrections strengthen the ledger's accuracy; neither required touching any file outside `.planning/ledgers/milestone-07-08.md`.

## Issues Encountered

None blocking. One long-running background command (`cargo build --workspace --features content-processing`) took longer than the default 120s foreground timeout and was moved to background per the harness's own instructions; it completed successfully (exit 0) and its output was read back before drafting the `REQ-content-processing-build-gate` row.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None introduced by this plan. One pre-existing stub was *found* (not introduced) and is recorded in the ledger rather than fixed: `crates/paladin-content/README.md`'s "Feature Flags" section still lists the deleted `pdf` feature — a residue from plan 10-05's manifest deletion that did not extend to the README. Recorded under `REQ-paladin-content-readme-update`'s row; not this plan's own defect to fix (out of `files_modified` scope).

## Next Phase Readiness

- All 86 ledger rows are now derived with zero pending stubs anywhere in `.planning/ledgers/milestone-07-08.md` — plan 10-11's close-out amendment can proceed without any remaining fan-out dependency.
- Phase 11 has three additional citable, requirement-numbered pointers from this plan's range: `FACADE-01` (D5, via `REQ-m8-deferred-items-register`), `FACADE-02` (D1-D4, same row), `FACADE-03(a)` (`REQ-deferred-cli-user-commands`) and `FACADE-03(b)` (`REQ-deferred-tensorflow-ml-adapter-v3`) — all four already named in ADR-0028's own `Downstream Consumers`, now also carried at the row level.
- Two small, well-evidenced gaps are now visible for a future small-fix plan to pick up (neither blocks Phase 10's own gate): the missing `paladin-content` CHANGELOG entry FR-8 requires (`REQ-paladin-content-changelog-fix`), and the stale `pdf` feature line in `paladin-content/README.md`.
- No blockers. No `.rs` file was touched; `git status --porcelain -- '*.rs'` is empty.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-07-08.md`
- FOUND: commit `3dbbca7` (Task 1)
- FOUND: commit `6e3213a` (Task 2)

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
