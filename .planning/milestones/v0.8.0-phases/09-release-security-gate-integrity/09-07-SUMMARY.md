---
phase: 09-release-security-gate-integrity
plan: 07
subsystem: infra
tags: [close-out, requirements-ledger, adr-numbering, coverage-declaration, ci-only-claims]

# Dependency graph
requires:
  - phase: 09-release-security-gate-integrity
    provides: "All six sibling plans' SUMMARYs (09-01..09-06) — the verbatim command transcripts this plan cites as closure evidence"
provides:
  - "REQUIREMENTS.md: SEC-01..SEC-05 closed behind cited evidence, stale suppression arithmetic corrected at source, Phase 10/HARD-01 hand-off block, Phase 12 SUPPLY-01/SUPPLY-02 closure notes"
  - "codebase/CONCERNS.md: advisory sections corrected at source (nine not ten unmaintained pre-phase, gcc already absent, deny.toml:120-122 re-derived)"
  - "decisions/PROMOTION.md: ADR numbering index advanced to 0028 with rows for ADR-0024..0027"
  - "PROJECT.md: four Key Decisions rows for ADR-0024..0027"
  - "ROADMAP.md: Phase 12 re-scoped — SUPPLY-01/SUPPLY-02 closed-by-Phase-9, SUPPLY-03 is what remains"
  - "09-COVERAGE.md: confirmed pre-existing, unchanged (already carries the required 'No external API integration' declaration)"
affects: ["phase-10-hard-01", "phase-12-supply-01-supply-02-supply-03"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Amend-in-place ledger correction (D-00c/D-00d): every stale figure gets a dated annotation naming what was wrong, with the original text retained below it — applied across REQUIREMENTS.md, CONCERNS.md, PROJECT.md, ROADMAP.md and PROMOTION.md's inventory rows in this plan"
    - "Cross-phase hand-off block: an explicit, headed list of REQ-* IDs a not-yet-built ledger (Phase 10's Milestone 7-8 ledger) must record as already-closed, used because the amend-in-place pattern has no target to amend when the downstream ledger does not exist yet"

key-files:
  created: []
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/codebase/CONCERNS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md
    - .planning/ROADMAP.md

key-decisions:
  - "SEC-01, SEC-03 and SEC-05 flipped to closed, joining SEC-02 and SEC-04 which plans 09-05 and 09-01 had already flipped — all five now closed behind commit/file:line citations lifted verbatim from the six sibling SUMMARYs"
  - "SUPPLY-01 and SUPPLY-02 (Phase 12) are annotated 'closed by Phase 9' with commit references rather than having their own checkboxes flipped — those checkboxes remain Phase 12's to check on verification, per this phase's own precedent of not claiming closure on unexecuted evidence, since the CI-only confirmation (required status check resolving, cargo audit/deny actually passing) is explicitly Phase 12's to run"
  - "The stale 'fifteen entries / ten unmaintained' figure is corrected to fourteen/nine (pre-Phase-9) at every location it appears (REQUIREMENTS.md's SEC-01 block, run-5 correction banner and Milestone 10 criterion 3; CONCERNS.md's advisory section) — annotated in place, original text retained, never rewritten"
  - "09-COVERAGE.md required no edit — it already existed from plan-time and already opens with the literal required phrase; verified rather than recreated"
  - "PROMOTION.md's live 'Next free ADR number' counter value is replaced in place (0024 -> 0028), not annotated-and-retained, because that line is the file's own designed live-counter mechanism (the explanatory dated note beneath it is the retention mechanism for this specific file, matching Phase 8's precedent for 0022/0023) — distinct from the D-00c/D-00d ledger-correction pattern applied everywhere else in this plan"

patterns-established: []

requirements-completed: [SEC-01, SEC-03, SEC-05]

coverage:
  - id: D1
    description: "SEC-01, SEC-02(already closed), SEC-03, SEC-04(already closed), SEC-05 all read [x] in REQUIREMENTS.md, each with a closure note carrying a verbatim commit/file:line citation; SEC-01's closure note names both dependency auditors as not run in this environment"
    requirement: "SEC-01, SEC-03, SEC-05"
    verification:
      - kind: other
        ref: "bash -c 'for id in SEC-01 SEC-02 SEC-03 SEC-04 SEC-05; do grep -q \"\\[x\\] \\*\\*$id\\*\\*\" REQUIREMENTS.md; done' -- see Task 1 <verify> output below"
        status: pass
    human_judgment: false
  - id: D2
    description: "All five SEC traceability rows read Complete; stale suppression arithmetic corrected at source in REQUIREMENTS.md and CONCERNS.md with original text retained"
    verification:
      - kind: other
        ref: "grep -E '^\\| SEC-0[1-5] \\| Phase 9 \\| Complete \\|' REQUIREMENTS.md (5 matches); git diff --word-diff confirms zero real content deletions in the corrected blocks"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md advances to 'Next free ADR number: 0028' with four new index rows and a dated note explaining the four-number jump; PROJECT.md gains four Key Decisions rows; ROADMAP.md's Phase 12 section carries a dated closure note"
    verification:
      - kind: other
        ref: "Task 2 <verify> script (below) — all assertions pass"
        status: pass
    human_judgment: false
  - id: D4
    description: "Workspace gate green (cargo test/fmt/clippy), no Rust source changed, all three guards + make check-gates pass, 09-COVERAGE.md confirmed, CI-only claims listed honestly"
    verification:
      - kind: other
        ref: "See 'Workspace gate transcript' and 'CI-only claims' sections below"
        status: pass
      - kind: other
        ref: "cargo audit / cargo deny check themselves were NOT run — crates.io returns HTTP 403 in this environment"
        status: pass
    human_judgment: true
    rationale: "The required-status-check resolving on the first real CI run, and cargo audit/cargo deny check actually passing against the reconciled configuration, are CI-only claims this sandboxed environment cannot execute — recorded as unverified-here per D-19, not inferred as passing."

duration: ~50min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 07: Close Phase 9 in the Planning Record Summary

**Closed SEC-01 through SEC-05 in REQUIREMENTS.md behind verbatim evidence from the six sibling plans, corrected the corpus's stale suppression arithmetic at three locations, advanced the ADR numbering line to 0028, handed Phases 10 and 12 explicit closure notes instead of guesswork, and confirmed the workspace gate green with zero Rust source changed across the whole phase.**

## Performance

- **Duration:** ~50 min (most of it the cold workspace `cargo test`/`cargo clippy` compile)
- **Completed:** 2026-08-08T04:15:42Z
- **Tasks:** 3 (2 produced commits; Task 3 is verification-only, no file changes required beyond confirming the pre-existing `09-COVERAGE.md`)
- **Files modified:** 5 (`REQUIREMENTS.md`, `codebase/CONCERNS.md`, `decisions/PROMOTION.md`, `PROJECT.md`, `ROADMAP.md`)

## Accomplishments

- **Task 1 — REQUIREMENTS.md closed behind evidence.** Flipped SEC-01, SEC-03 and SEC-05 to `[x]`
  (SEC-02 and SEC-04 were already closed by plans 09-05 and 09-01). Each of the five now carries a
  closure note citing verbatim commits and `file:line` locations lifted from the six sibling
  SUMMARYs — SEC-01's note explicitly states that `cargo audit` and `cargo deny check` were **not**
  run in this environment. Corrected the stale suppression arithmetic at three points inside the
  SEC-01 block (the "fifteen entries / ten unmaintained" figure, the "thirteen of fifteen ungoverned"
  figure, and the stale `ci.yml:389-406`/`:406` line citations for a job plan 09-06 has since
  deleted), each annotated in place with the original text retained. Amended all seven Milestone 7-8
  ledger rows SEC-01 through SEC-05 touch. Updated all five SEC traceability rows from `Pending` to
  `Complete`. Recorded SUPPLY-01 and SUPPLY-02 as closed-by-Phase-9 with commit references at their
  own requirement definitions and at the cross-phase coupling table, and wrote an explicit,
  clearly-headed Phase 10 / HARD-01 hand-off block naming all seven `REQ-*` rows the not-yet-built
  Milestone 7-8 ledger must record as already closed. Updated the HARD-06 coupling row to record
  that SEC-01 discharges its side of the coupling on tree evidence without waiting for Phase 10, and
  states plainly that this phase does not answer the PDF capability question.

- **Task 2 — the codebase map, the ADR line, and Phase 12's scope.** Corrected `CONCERNS.md`'s two
  advisory sections at source: the unmaintained count was nine (not ten) before this phase touched
  anything, `RUSTSEC-2025-0121` (`gcc`) was already gone with no record of removal, and the
  `deny.toml:141-147` citation for the three 2026 advisories was already stale — re-derived to
  `deny.toml:120-122` and confirmed against the current file. Added a pointer to
  `SECURITY-EXCEPTIONS.md` as the authoritative register in both sections. Advanced
  `decisions/PROMOTION.md`'s numbering index from 0024 to 0028 with rows for ADR-0024 through
  ADR-0027 (all four authored by this phase's own plans 09-02, 09-05, 09-04 and 09-03 respectively),
  a dated note explaining the four-number jump, and updated the ADR-candidate inventory to record
  that candidate 3 (the RustSec remediation plan) closed via ADR-0024 and candidate 5 (the licence
  checklist) closed two phases early, via ADR-0025, rather than waiting for its originally-forecast
  Phase 10. Added four Key Decisions rows to `PROJECT.md` and amended the table's evidence note to
  record that Phase 9 supplied its forecast entries. Re-scoped Phase 12 in `ROADMAP.md`: marked
  success criteria 1, 2, 3 and 4 as satisfied by Phase 9 with commit references, criterion 5 as
  half-satisfied (the ADR-promotion half remains SUPPLY-03's subject), and added a dated closure
  note stating plainly that SUPPLY-01 and SUPPLY-02 are inherited closed items to verify and
  SUPPLY-03 is the only requirement Phase 12 still has to plan.

- **Task 3 — the phase gate, the coverage declaration, and the CI-only list.** Ran the full CLAUDE.md
  workspace gate order (`cargo test --offline --workspace`, `cargo fmt --check`,
  `cargo clippy --offline --workspace -- -D warnings`) — all green, recorded verbatim below. Proved
  the no-Rust-change claim by diffing `*.rs` against both the phase-base commits named in the plan
  (`49ad74c`) and independently against the earlier base `7ae7dd4` — both return `0`. Confirmed the
  ADR-0006 84% coverage floor is unmoved by the no-source-change argument rather than re-measuring
  it (`cargo llvm-cov` is not installable here). Ran all three guards plus `make check-gates`
  together — all exit 0. Re-verified the five drift counts this phase depended on. Confirmed
  `09-COVERAGE.md` already existed from plan time and already opens with the required literal
  phrase; no edit was needed. Wrote the headed CI-only claims list below, naming every claim this
  phase could not verify locally.

## Task Commits

1. **Task 1: Close the five SEC rows in REQUIREMENTS.md behind recorded evidence** — `e84e0eb` (docs)
2. **Task 2: Correct the codebase map, advance the ADR line, and re-scope Phase 12** — `656fa20` (docs)
3. **Task 3: Run the phase gate, declare coverage, and state what CI still has to prove** — no
   commit (verification-only; `09-COVERAGE.md` required no edit since it already carried the
   required content from plan time)

**Plan metadata:** `SUMMARY.md` commit follows immediately after this file is written, per
worktree protocol.

## Files Created/Modified

- `.planning/REQUIREMENTS.md` — SEC-01..SEC-05 closed, stale arithmetic corrected at source,
  seven ledger rows amended, five traceability rows updated, Phase 10 hand-off block added,
  SUPPLY-01/SUPPLY-02/HARD-06 coupling rows amended.
- `.planning/codebase/CONCERNS.md` — both advisory sections corrected at source, pointer to
  `SECURITY-EXCEPTIONS.md` added.
- `.planning/decisions/PROMOTION.md` — numbering index advanced to 0028, dated note added,
  ADR-candidate inventory updated for items 3 and 5.
- `.planning/PROJECT.md` — four Key Decisions rows added, evidence-note amended.
- `.planning/ROADMAP.md` — Phase 12 section re-scoped with a dated closure note; no other section
  touched.

## Decisions Made

- **SUPPLY-01/SUPPLY-02's own checkboxes are left unchecked for Phase 12 to flip.** This plan
  annotates both as "closed by Phase 9" with commit references at their definitions and at the
  cross-phase coupling row, per D-07 — but does not pre-emptively check Phase 12's own requirement
  boxes, since the plan's own prohibition ("a closure must not be claimed on evidence that was not
  executed in this environment") extends to the CI-only confirmation those two requirements still
  need (the required status check resolving on a real CI run; `cargo audit`/`cargo deny check`
  actually passing). Phase 12 inherits a closed item to verify, not work to re-plan, and its own
  plan is the one that runs the verification and checks its own box.
- **The stale-arithmetic corrections are annotations, not rewrites, everywhere except PROMOTION.md's
  live counter.** Every ledger/map correction in Task 1 and Task 2 retains the original text below
  a dated banner (D-00c/D-00d). The one exception is PROMOTION.md's "Next free ADR number" line
  itself, whose value is replaced in place (`0024` -> `0028`) because that line is the file's own
  designed live-counter mechanism — the dated explanatory note beneath it (matching Phase 8's
  precedent for the 0022/0023 jump) is this specific file's retention mechanism, not the literal
  counter text.
- **Recorded ADR-0025 as closed two phases early** in PROMOTION.md's inventory (originally forecast
  "Owner phase: Phase 10") rather than leaving the stale forecast silently wrong — the original
  "Owner phase: Phase 10" text is retained, with a note explaining the promotion happened via Phase
  9's SEC-02 instead.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - plan-fixture artifact, not an implementation bug] Literal `git diff | grep -cE '^\-[^-]'` returns 12, not 0, for REQUIREMENTS.md — confirmed as a unified-diff line-replacement artifact, not an actual deletion**

- **Found during:** Task 1 acceptance-criteria verification.
- **Issue:** The acceptance criterion expects `0` deleted content lines "for the SEC block and the
  ledger rows," permitting only the single-character checkbox-flip lines as an exception. Because
  seven Milestone-7-8 ledger rows and two cross-phase coupling rows are each a **single markdown
  table line**, appending an annotation to the end of the line (the correct, non-destructive way to
  extend a one-line table cell) makes git's unified diff render the *whole line* as one deletion
  plus one addition — even though no text within it was removed. Counted literally, the file shows
  12 deleted lines: 3 are the permitted SEC-01/03/05 checkbox single-character flips; 3 are the
  traceability table's `Pending` -> `Complete` word changes (explicitly instructed by Task 1(d), a
  different table than "the SEC block or ledger rows" the deletion-count restriction names); and 6
  are single-line table/coupling rows where an annotation was appended.
- **Fix:** No code/content change — verified via `git diff --word-diff=porcelain`, which reports
  only 6 genuine word-level deletion tokens in the entire file: the 3 checkbox characters and the 3
  `Pending` tokens. The 6 apparent "ledger row" deletions carry **zero** word-level deletions; every
  character of the original row text is present verbatim as a prefix of the new line, with only
  appended text after it. This is the same class of diff-artifact plan 09-04's and 09-06's SUMMARYs
  documented (`git diff | grep -c ...` counting unified-diff header/context lines rather than actual
  content changes) — a plan-fixture reading issue, not an execution defect.
- **Files modified:** None beyond the intended annotations already described above.
- **Verification:**
  ```
  $ git diff -- .planning/REQUIREMENTS.md | grep -cE '^\-[^-]'
  12
  $ git diff --word-diff=porcelain -- .planning/REQUIREMENTS.md | grep -E '^-' | grep -v '^---' | grep -vc '^-$'
  6
  $ git diff --word-diff -- .planning/REQUIREMENTS.md | grep -E '\[-.*-\]'
  - [-[ ]-]{+[x]+} **SEC-01**: ...
  - [-[ ]-]{+[x]+} **SEC-03**: ...
  - [-[ ]-]{+[x]+} **SEC-05**: ...
  | SEC-01 | Phase 9 | [-Pending-]{+Complete+} |
  | SEC-03 | Phase 9 | [-Pending-]{+Complete+} |
  | SEC-05 | Phase 9 | [-Pending-]{+Complete+} |
  ```
  Exactly six real content changes exist in the entire file: three checkbox flips (permitted) and
  three `Pending`->`Complete` traceability-row changes (explicitly instructed by the plan, outside
  the "SEC block and ledger rows" scope the zero-deletion criterion names). **Permitted deletions:
  6.** The remaining 6 apparent line deletions in the literal count are diff-rendering artifacts of
  single-line markdown rows with zero actual content removed.
- **Committed in:** `e84e0eb` (Task 1 commit).

**2. [Rule 1 - plan-fixture artifact] Similarly for `codebase/CONCERNS.md` and `ROADMAP.md`**

- **Found during:** Task 2 acceptance-criteria verification.
- **Issue:** `git diff -- .planning/codebase/CONCERNS.md | grep -cE '^\-[^-]'` returns `0` (fully
  clean — the annotations there were added as new trailing paragraphs, not appended to existing
  single lines). `git diff -- .planning/ROADMAP.md | grep -cE '^\-[^-]'` returns `5`, one per
  amended success-criterion line in the Phase 12 section (each criterion is a single markdown list
  item). `git diff --word-diff=porcelain -- .planning/ROADMAP.md` confirms **zero** word-level
  deletions across the entire file — every one of the five apparent deletions is a pure append.
- **Fix:** No content change; documented per the same reasoning as deviation 1.
- **Files modified:** None beyond the intended annotations.
- **Committed in:** `656fa20` (Task 2 commit). `PROMOTION.md` additionally shows 2 literal line
  deletions in this same commit: one is the intentional "Next free ADR number: 0024 -> 0028" counter
  update (see Decisions Made above — this file's own live-counter design, not a ledger correction),
  and the other is a pure-append artifact on the RustSec-candidate inventory row (confirmed via
  `git show 656fa20 -- .planning/decisions/PROMOTION.md`, the full original sentence survives
  verbatim as a prefix of the new line).

---

**Total deviations:** 2 documented, both plan-fixture/verification-reading issues inherent to how
unified diffs render single-line markdown table/list rows, not execution defects. No content was
lost in either file; all corrections are confirmed additive by `git diff --word-diff`.

## Workspace gate transcript (Task 3, CLAUDE.md order)

```
$ cargo test --offline --workspace
... (cold workspace compile, ~40 min in this sandboxed environment)
35 "test result: ok." lines, 0 "FAILED" lines
Total passed: 3013 (sum of all per-crate "N passed" figures)
EXIT=0

$ cargo fmt --check
EXIT=0

$ cargo clippy --offline --workspace -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 06s
EXIT=0
```

**Base vs. current test count, recorded side by side per the plan's own instruction (not asserted
equal, though they are):** Phase 8's exit measurement (`08-09-SUMMARY.md`) recorded **3013 passed, 0
failed** as the phase-base count entering Phase 9. This session's `cargo test --offline --workspace`
also returns **3013 passed, 0 failed**. The counts match exactly, which is the expected outcome
given zero `.rs` files changed across the entire phase — not a coincidence requiring investigation.

## Proof of the no-Rust-change claim

```
$ git diff --stat 49ad74c..HEAD -- '*.rs' | wc -l
0
$ git diff --stat 7ae7dd4f3b59f4d40aab74d86bc035476d8f3d5e..HEAD -- '*.rs' | wc -l
0
```

Both the plan's own cited base (`49ad74c`) and the phase's actual first commit's parent
(`7ae7dd4`) confirm zero Rust source files changed anywhere in Phase 9. **The ADR-0006 84%
workspace line-coverage floor is therefore confirmed unmoved by this argument, not re-measured** —
`cargo llvm-cov` was not run for a phase that touched no `.rs` file, and no coverage percentage this
phase did not measure appears anywhere in this document.

## Guard transcript (Task 3)

```
$ ./scripts/check-advisory-register.sh
✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.
EXIT=0

$ ./scripts/check-crate-names.sh
✅ 11 publishable crate(s) checked, all match the allow-list exactly.
EXIT=0

$ ./scripts/check-changelogs.sh
✅ 10 publishable crate(s) checked, all have a CHANGELOG.md.
EXIT=0

$ make check-gates
(runs all three guards above in sequence)
EXIT=0
```

## Drift re-verification counts (Task 3(d))

Re-verified against the live tree in this session, independent of anything carried forward from
earlier plans:

| Count | Command | Result |
|---|---|---|
| `deny.toml` `[advisories] ignore` size | `python3 -c "import tomllib; print(len(tomllib.load(open('deny.toml','rb'))['advisories']['ignore']))"` | `10` |
| `.cargo/audit.toml` `[advisories] ignore` size | same, against `.cargo/audit.toml` | `5` |
| `SECURITY-EXCEPTIONS.md` register row count | parsed the fenced TOML block, counted `[[exception]]` tables | `10` |
| `.crate-names.txt` publishable-name count | `grep -vc '^\s*#' .crate-names.txt` | `11` |
| Per-crate `CHANGELOG.md` count | `find crates -maxdepth 2 -name CHANGELOG.md \| wc -l` against 11 crate directories | `10` (the 11th, `doc-examples`, is `publish = false`-exempt) |
| `Dockerfile.chef` planner-stage per-crate manifest `COPY` count | `grep -c 'COPY crates/paladin' Dockerfile.chef` | `0` (structural `COPY crates ./crates` at `:30` and `:61` instead) |

None of the six drifted from the values the sibling plans landed — no investigation required.

## CI-only claims — what a CI runner still has to prove

None of the following is reported as passing. Each is recorded as unverified-here, with the reason
and the exact command a CI runner will execute:

1. **`cargo audit` and `cargo deny check` actually passing against the reconciled `deny.toml` /
   `.cargo/audit.toml`.** Neither tool is installable in this sandboxed environment — `crates.io`
   returns HTTP 403 on every `cargo install cargo-audit` / `cargo install cargo-deny` attempt.
   **Verifying command:** `cargo audit` and `cargo deny check` (or `make security`), run on a real
   CI runner with network access to crates.io.
2. **The required status check `"Security Audit"` still resolving after the duplicate-job
   deletion.** Only the GitHub Actions platform can evaluate this on a real pull request against
   `.github/rulesets/protect-main-branch.json:39`. The risk is *assessed*, not open — the surviving
   `security-audit:` job's `name:` field is byte-unchanged and posts the identical context string —
   but assessment is not the same as a platform confirming it. **Verifying command:** open a real PR
   against this branch and observe the "Security Audit" check resolve in the GitHub UI / API.
3. **The container build's dependency-compilation layer reporting `CACHED` on a source-only
   rebuild.** ADR-0027's caching claim rests on cargo-chef's own documented `recipe.json` semantics,
   not a measurement — Docker is entirely absent from this environment. **Verifying command:**
   `docker build -f Dockerfile.chef -t paladin:test .` twice in a row with only a `.rs` file changed
   between runs, then inspect `docker build` output for `CACHED` on the `cargo chef cook` layer.
4. **The dual-licence expression (`MIT OR Apache-2.0`) accepted by crates.io at the next real
   publish.** No publish occurred in this phase (the same HTTP 403 blocks `cargo publish
   --dry-run`), so this is a future release-cycle action recorded as out of scope here rather than
   inferred as accepted. **Verifying command:** `cargo publish --dry-run -p <crate>` (or the real
   publish) on a CI runner with crates.io access, for each of the ten library crates plus the root
   facade.

## Known Stubs

None.

## Threat Flags

None — this plan's threat model (T-09-31 through T-09-35, T-09-SC) is fully addressed by the two
executed tasks; no new security-relevant surface was introduced. This is a planning-record-only
plan: zero `.rs` files, zero build-config files, and zero CI workflow files were touched.

## Issues Encountered

None beyond the two documented diff-artifact deviations above, both confirmed harmless via
`git diff --word-diff`.

## User Setup Required

None — no external service configuration required. `cargo audit` and `cargo deny check` remain
uninstallable in this environment (crates.io HTTP 403); their pass/fail is CI-only evidence,
recorded honestly as unverified-here per D-19, never inferred as passing.

## Next Phase Readiness

- **Phase 9 is closed.** All five SEC-01..SEC-05 requirements read `[x]` and `Complete` in both the
  checkbox list and the traceability table, each behind a verbatim citation.
- **Phase 10 / HARD-01** inherits an explicit hand-off block naming all seven `REQ-*` rows already
  closed by this phase, plus the HARD-06 coupling row recording the tree evidence (`pdf-extract`
  unconditional, `pdf = []` gating nothing) that discharges SEC-01's side without waiting for
  HARD-06's own capability-question answer.
- **Phase 12** inherits SUPPLY-01 and SUPPLY-02 as closed-by-Phase-9 items to verify (the CI-only
  confirmations named above), with SUPPLY-03 (the ADR-promotion decision) as the only requirement it
  still has to plan — recorded in both `REQUIREMENTS.md` and `ROADMAP.md`'s Phase 12 section.
- **The ADR numbering line reads 0028**, with a dated note explaining the four-number jump, matching
  the shape of Phase 8's earlier 0022/0023 precedent.
- No blockers for Phase 10 or Phase 12. This plan touched zero `.rs` files, zero build-config files,
  and zero CI workflow files — confirmed by `git diff --stat -- '*.rs' | wc -l` returning `0` against
  both the plan's cited base and the phase's actual first commit.

## Self-Check: PASSED

- FOUND: `.planning/REQUIREMENTS.md` (SEC-01..SEC-05 all `[x]`, all five traceability rows `Complete`)
- FOUND: `.planning/codebase/CONCERNS.md` (advisory sections corrected, `SECURITY-EXCEPTIONS.md` pointer present)
- FOUND: `.planning/decisions/PROMOTION.md` (`Next free ADR number: 0028`)
- FOUND: `.planning/PROJECT.md` (four new Key Decisions rows for ADR-0024..0027)
- FOUND: `.planning/ROADMAP.md` (Phase 12 section carries the dated closure note)
- FOUND: `.planning/phases/09-release-security-gate-integrity/09-COVERAGE.md` (pre-existing, opens with "No external API integration")
- FOUND: commit `e84e0eb` (Task 1)
- FOUND: commit `656fa20` (Task 2)

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
