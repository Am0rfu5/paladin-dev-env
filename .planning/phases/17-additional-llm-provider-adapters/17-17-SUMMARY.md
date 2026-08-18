---
phase: 17-additional-llm-provider-adapters
plan: 17
subsystem: infra
tags: [ci, github-actions, windows-ledger, broken-windows, snyk, gap-closure]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "Five gap-closure plans (17-12..17-16) that closed CR-01, WR-01, WR-02, WR-03(new), WR-04(new) and each recorded its own Snyk-not-run status in its SUMMARY.md"
provides:
  - "WINDOWS.md ids 17 (17-11's missing Snyk row), 18 (this run's five-plan Snyk-not-run row), 19 (IN-01 carried forward as accepted debt)"
  - "A CI job (llm-registry-unit-tests) that runs the tests/unit/mod.rs binary under --features llm-all, the first CI configuration to both compile all nine provider rows into the registry and execute tests/unit/llm/provider_factory_test.rs"
  - "feature-matrix-summary now gates on llm-registry-unit-tests's result"
affects: [phase-17-verification, gsd-ship, gsd-verify-work]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Dedicated CI job modelled on cli-isolation for a feature-set x test-binary intersection no matrix leg covers"]

key-files:
  created: []
  modified:
    - .planning/WINDOWS.md
    - .github/workflows/feature-flags.yml

key-decisions:
  - "Filed three ledger rows rather than two, per the plan's explicit Row A/B/C split, using gsd-tools windows append exclusively (D-00d) so table, JSON block and frontmatter counters stayed in step"
  - "Added one dedicated CI job (llm-registry-unit-tests) rather than widening ci.yml or adding --test unit to all fifteen matrix legs -- narrowest fix for the exact blind spot CR-01 exposed"
  - "Corrected a plan-authoring detail in the SUMMARY rather than the workflow: the plan's prose describes 'the fourteen-leg matrix', but feature-matrix.strategy.matrix.include has always had 15 entries (no-default-features, default, all-features, full, llm-openai, llm-anthropic, llm-deepseek, llm-all, web-server, content-processing, notifications, vision, redis-queue, s3-storage, cli) -- verified by direct count both before and after this plan's edit, unchanged by this plan"

requirements-completed: [PROV-01, PROV-04]

coverage:
  - id: D1
    description: "WINDOWS.md carries the 17-11 Snyk-not-run row 17-VERIFICATION.md flagged as missing, plus this run's own five-plan Snyk-not-run row, plus IN-01 carried forward as accepted debt"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "gsd-tools windows status (open_count 15->18, waived/fixed unchanged, total 16->19); python3 JSON-block parse (19 entries) vs table row count (19) agreement check"
        status: pass
    human_judgment: false
  - id: D2
    description: "A dedicated CI job runs the tests/unit/mod.rs binary under --features llm-all, gated into feature-matrix-summary, closing the blind spot that let CR-01 ship green through every CI configuration"
    requirement: "PROV-01"
    verification:
      - kind: other
        ref: "python3 yaml.safe_load assertion (job exists, needs edge exists); cargo test --test unit --features llm-all (428 passed, 0 failed, both default parallelism and --test-threads=1)"
        status: pass
    human_judgment: true
    rationale: "The workflow itself cannot be executed in this sandbox (no GitHub Actions runner) -- only its YAML validity, its dependency edge, and its command's local result were observed. A human (or a subsequent CI run) must confirm the job actually passes on a runner."

duration: ~25min
completed: 2026-08-18
status: complete
---

# Phase 17 Plan 17: Gap-closure bookkeeping and CI blind-spot closure Summary

**Filed three WINDOWS.md rows (17-11's missing Snyk scan, this run's five-plan Snyk-not-run row, IN-01 carried forward) and added a dedicated `llm-registry-unit-tests` CI job that runs the workspace unit binary under `--features llm-all`, closing the blind spot that let CR-01 ship green through every CI configuration.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-18
- **Tasks:** 2
- **Files modified:** 2 (`.planning/WINDOWS.md`, `.github/workflows/feature-flags.yml`)

## Accomplishments

- `.planning/WINDOWS.md` now carries three new rows (ids 17, 18, 19): the 17-11 Snyk-not-run row `17-VERIFICATION.md` explicitly named as missing (sibling of ids 15/16), this gap-closure run's own five-plan (17-12..17-16) Snyk-not-run row, and IN-01 (`.project/current-exports.txt` untracked for six new adapter types) carried forward as developer-accepted debt with its 2026-08-18 exclusion decision on record.
- A new CI job, `llm-registry-unit-tests`, runs `cargo test --test unit --features llm-all` — the first CI configuration to both compile all nine provider rows into the registry (via `llm-all`) and execute the `tests/unit/mod.rs` integration-test binary that holds `provider_factory_test.rs`. Wired into `feature-matrix-summary`'s `needs:` list and result-check shell so a failure there fails the workflow.
- Every acceptance criterion in both tasks verified against the live tree; the exact command the new job will run was proven green locally under both default thread parallelism and `--test-threads=1` (428 passed, 0 failed, identical counts both times).

## Task Commits

Each task was committed atomically:

1. **Task 1: File the three ledger rows** — `868b777` (docs)
2. **Task 2: Add the llm-registry-unit-tests CI job** — `e8f541b` (ci)

_No plan-metadata commit in this worktree — the orchestrator commits `STATE.md`/`ROADMAP.md` centrally after merge per this plan's worktree-execution instructions._

## Files Created/Modified

- `.planning/WINDOWS.md` — three new rows (ids 17, 18, 19); no existing row's status changed. `open_count` 15→18, `waived_count`/`fixed_count` unchanged (0/1), `total_count` 16→19. Table and JSON block agree (19 rows each), verified via `python3` parse.
- `.github/workflows/feature-flags.yml` — added job `llm-registry-unit-tests` (id, name, `runs-on`, checkout/toolchain/cache/test steps, a three-sentence provenance comment naming CR-01 and `17-REVIEW.md`); extended `feature-matrix-summary`'s `needs:` list and its success/failure shell branches. The matrix legs, `on:`/`concurrency:` triggers, and `ci.yml` are untouched.

## Decisions Made

- Filed all three ledger rows through `gsd-tools windows append` exclusively (D-00d) — never hand-edited `WINDOWS.md` — so the table, JSON block and frontmatter counters moved together atomically. Ran the three appends strictly sequentially, verifying `windows status` between the first accidental-scope incident (see Issues Encountered) and the correct re-run.
- Placed the new CI job between `cli-isolation` and `feature-matrix-summary`, matching the file's existing read order (matrix → dedicated jobs → summary), and modelled its checkout/toolchain/cache steps on `cli-isolation`'s shape exactly, per the plan's instruction.
- Did not widen `ci.yml`, did not add `--test unit` to all matrix legs, and did not mark any pre-existing `WINDOWS.md` row `fixed` or `waived` — all three were explicit scope guards in the plan and were honored.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 — Blocking] Recovered from a worktree-isolation escape during Task 1's first append attempt**
- **Found during:** Task 1 (filing the three ledger rows)
- **Issue:** The first three `gsd-tools windows append` invocations were run as `cd /workspace && node .claude/gsd-core/bin/gsd-tools.cjs windows append ...`. The Bash tool's worktree-isolation sandbox intercepts `git` commands that leave the worktree but did not intercept this `cd` + `node` invocation, so all three appends landed on `/workspace/.planning/WINDOWS.md` (the shared main checkout) instead of this worktree's own `.planning/WINDOWS.md`. The worktree's own ledger remained untouched (verified: still `open_count: 15` after the three appends). Confirmed by inspecting `/workspace/.planning/WINDOWS.md` directly, which showed `open_count: 18` while the worktree copy showed `15`.
- **Fix:** Restored `/workspace/.planning/WINDOWS.md` to its pristine pre-task state by copying this worktree's own clean copy over it with `cp` (confirmed byte-identical via `diff`, and confirmed the worktree's own copy was clean against `HEAD` via `git diff --stat` / `git status --short` before the copy). Then re-ran all three `gsd-tools windows append` invocations correctly, using the CLI's `--cwd` global flag pointed at this worktree's absolute path, and verified after each append that the worktree's own `.planning/WINDOWS.md` (not `/workspace`'s) had the expected new `open_count`.
- **Files modified:** `.planning/WINDOWS.md` (this worktree, correctly) — `/workspace/.planning/WINDOWS.md` (the shared main checkout) was touched transiently and then restored to its exact original content; it carries no net change and is not part of this plan's diff.
- **Verification:** Re-ran every acceptance-criteria `grep -c` / `python3` check from the plan against the worktree's ledger after the corrected appends — all matched the earlier (mistakenly-scoped) run's numbers exactly (17-11 count 2, IN-01 count 2, 2026-08-18 count 10, OpenAiCompatibleAdapter count 2, unrun-verify count 24, JSON/table agreement 19/19). Confirmed `/workspace/.planning/WINDOWS.md` was restored to `open_count: 15` / `total_count: 16` before moving on, and confirmed no orphaned `.tmp` files were left behind by the atomic-write mechanism.
- **Committed in:** `868b777` (Task 1 commit, containing only this worktree's correctly-scoped changes)

**2. [Rule 1 — Bug] A plan acceptance-criterion regex false-positived on `runs-on:`**
- **Found during:** Task 2 (adding the CI job)
- **Issue:** The plan's acceptance criterion `git diff -- .github/workflows/feature-flags.yml | grep -cE '^[+-].*(on:|concurrency:|cancel-in-progress|branches:)'` is supposed to return `0` (proving triggers/concurrency untouched), but returned `1` because the unanchored `on:` alternative matches the substring inside `runs-on:` — a line the new job legitimately adds.
- **Fix:** No code change needed. Inspected the full `git diff` for `.github/workflows/feature-flags.yml` directly and confirmed the only changed lines are the new `llm-registry-unit-tests` job block and the two `feature-matrix-summary` edits (`needs:` list and the success/failure shell branches) — the actual `on:`, `concurrency:`, and `cancel-in-progress` blocks at the top of the file are byte-for-byte unchanged.
- **Files modified:** None (verification-only finding).
- **Verification:** `git diff -- .github/workflows/feature-flags.yml` read in full; the single matching diff line was `+    runs-on: ubuntu-latest`.
- **Committed in:** N/A (documented here, no code change required)

**3. [Rule 1 — Bug, minor] Plan text says "fourteen-leg matrix"; the file has always had fifteen legs**
- **Found during:** Task 2 (adding the CI job)
- **Issue:** The plan's acceptance criterion expects `feature-matrix.strategy.matrix.include` to have length `14`. Direct count of the matrix entries (`no-default-features`, `default`, `all-features`, `full`, `llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`, `web-server`, `content-processing`, `notifications`, `vision`, `redis-queue`, `s3-storage`, `cli`) is `15`, both before and after this plan's edit — this plan never touched the matrix section.
- **Fix:** No code change — the matrix is genuinely unchanged (that is the criterion's actual intent). The `14` figure in the plan's prose and acceptance criterion is stale relative to the live file; this plan does not correct it, since it is out of this plan's stated scope (`.planning/WINDOWS.md` and `.github/workflows/feature-flags.yml`'s new-job-plus-gate edits only) and correcting plan prose retroactively is not this executor's job.
- **Files modified:** None.
- **Verification:** `python3 -c "import yaml; ...len(...matrix.include)"` → `15`, both confirmed via direct read of the file before editing and via the post-edit run recorded above.
- **Committed in:** N/A (documented here, no code change required)

---

**Total deviations:** 1 auto-fixed (worktree-isolation escape, Rule 3), 2 documented-only (plan-criterion false positives, Rule 1 — no code changes, both are limitations of the plan's own verification text rather than defects in the delivered work).
**Impact on plan:** None on the shipped artifacts. The worktree-isolation escape was fully recovered before any commit was made, with the shared main checkout restored byte-identical to its pre-task state and confirmed via `diff`. Both acceptance-criterion false positives were investigated and confirmed to be regex/count artifacts, not real defects — the actual diffs and actual matrix contents were inspected directly in each case.

## Snyk

**Not applicable.** This plan touches no Rust source at all (`git diff --stat -- crates/ tests/` is empty for both commits, confirmed above) — only `.planning/WINDOWS.md` and `.github/workflows/feature-flags.yml`. Per `snyk_rules.instructions.md` (imported via `CLAUDE.md`), a Snyk scan applies to newly generated/modified first-party *code*; neither file qualifies. This is a different statement from "not run" — Row B (WINDOWS.md id 18) already covers the five plans (17-12..17-16) whose Snyk scans genuinely could not run because no `snyk_code_scan` MCP tool or Snyk CLI was available.

## D-00e Verification Record — exact commands and exact output

1. **`gsd-tools windows status`** (before) → `open_count: 15, waived_count: 0, fixed_count: 1, total_count: 16`
2. **`gsd-tools windows status`** (after, worktree-scoped) → `open_count: 18, waived_count: 0, fixed_count: 1, total_count: 19` — new ids assigned: **17** (17-11 Snyk row), **18** (this run's five-plan Snyk row), **19** (IN-01 carried forward).
3. `grep -c '17-11' .planning/WINDOWS.md` → `2`
4. `grep -c 'IN-01' .planning/WINDOWS.md` → `2`
5. `grep -c '2026-08-18' .planning/WINDOWS.md` → `10`
6. `grep -c 'OpenAiCompatibleAdapter' .planning/WINDOWS.md` → `2`
7. `grep -c 'unrun-verify' .planning/WINDOWS.md` → `24` (baseline was 20; delta +4 = two new `unrun-verify` rows × table+JSON)
8. `grep -c '| fixed |' .planning/WINDOWS.md` → `1` (unchanged); `grep -c '| open |' .planning/WINDOWS.md` → `18` (15 pre-existing open + 3 new)
9. `python3` JSON-block parse → `19` entries; `python3` table-row regex count → `19` rows. Agree.
10. `git diff --stat -- crates/ tests/ .github/` (Task 1 scope check) → empty.
11. `python3 -c "import yaml; ...'llm-registry-unit-tests' in j and in j['feature-matrix-summary']['needs']"` → `ok`
12. `python3 -c "...len(matrix.include)"` → `15` (see Deviations #3 — plan prose says 14, file has always had 15, unchanged by this plan)
13. `grep -c 'llm-registry-unit-tests' .github/workflows/feature-flags.yml` → `5`
14. `grep -c 'cargo test --test unit --features llm-all' .github/workflows/feature-flags.yml` → `1`
15. `grep -c 'CR-01' .github/workflows/feature-flags.yml` → `2`
16. `git diff -- .github/workflows/ci.yml` → empty
17. `git diff --stat -- crates/ tests/ .planning/` (Task 2 scope check) → empty
18. **`cargo test --test unit --features llm-all`** → **428 passed; 0 failed; 11 ignored;** finished in 5.75s
19. **`cargo test --test unit --features llm-all -- --test-threads=1`** → **428 passed; 0 failed; 11 ignored;** finished in 21.13s — **identical pass count to #18**, proving ordering-independence.
20. `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` → **217 passed; 0 failed** (up from 197 at 17-11's baseline — reflects new regression tests added by plans 17-12 through 17-16)
21. `cargo test -p paladin-llm` (default features) → **59 passed; 0 failed**
22. `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` → exit 0, no warnings
23. `cargo fmt --check -p paladin-llm` → exit 0

**The new CI job's own execution on a GitHub Actions runner was NOT observed** — this sandbox has no runner. Only its YAML validity (#11), its dependency edge (#11), its matrix-noninterference (#12), and its command's local result (#18, #19) were verified, exactly as the plan's `executor_notes` require. Stated plainly, not implied away.

## Closing note for `/gsd-verify-work` — separating closed work from accepted debt

- **Closed by this gap-closure run (5):** CR-01 (blank-credential availability, plan 17-12), WR-01 (`openai_compatible` alias, plan 17-13), WR-02 (inverted temperature range, plan 17-14), WR-03 *new* (Gemini truncated-empty detection, plan 17-15), WR-04 *new* (stream-open retry parity, plan 17-16). Note the two ID collisions: `17-REVIEW.md`'s post-gap-closure WR-03 and WR-04 are **different findings** from the original WR-03/WR-04 that plans 17-11 and 17-10 closed in the prior gap-closure run — all four are now closed.
- **Carried forward as accepted debt (1):** IN-01 (`.project/current-exports.txt` regenerated under default features only, six new adapter types untracked for public-API drift) — excluded from this run's scope by explicit developer decision on 2026-08-18, now on the `WINDOWS.md` ledger as id 19.
- **Still blocked on the environment (4):** the `human_verification` items from `17-VERIFICATION.md` — the Snyk scan (now bookkept as WINDOWS.md ids 17 and 18, but genuinely not run), the 82% coverage floor (id 13, ADR-0006), the Ollama Docker live suite (id 12), and the vendor base-URL/model-ID live smoke test. None is closeable by any plan in this run — no Docker daemon, no network egress, no vendor API keys, no `snyk_code_scan` tool in this sandbox.
- **One flagged, unresolved probe row:** PROV-01 / encoding, surfaced in `17-17-PLAN.md`'s `<probe_resolutions>` as a planner assumption for a human to overturn (ADR-0045's provider-selection study performs no length/equality operation on text, so the probe's question has no clear referent). Blocks nothing; no acceptance criterion depends on it.

## REQUIREMENTS.md checkbox adjudication — deliberately deferred, not this plan's job

Per this plan's own `<deferred>` section and the project precedent plan 17-14 recorded (`.planning/REQUIREMENTS.md`'s note that Phase 17 requirement checkboxes are adjudicated **at phase close**, not per-plan, because several plans share PROV-01..04's IDs): this plan does **not** tick PROV-01 or PROV-04's checkboxes in `.planning/REQUIREMENTS.md`, even though its own frontmatter cites both. `REQUIREMENTS.md` is untouched by this plan's commits (confirmed: neither commit's diff touches that file). **This is the one item this plan's SUMMARY exists specifically to flag for the phase-close step** — plan 17-14 discovered mid-run that `gsd-tools query requirements.mark-complete PROV-02 PROV-04` would tick both checkboxes automatically if invoked, then deliberately reverted that tick before committing, so `REQUIREMENTS.md` currently shows both as open pending a phase-close-time human adjudication across all contributing plans (17-01 through 17-17).

## Issues Encountered

See "Deviations from Plan" #1 (worktree-isolation escape during Task 1, fully recovered) and #2/#3 (plan acceptance-criteria false positives, both investigated and confirmed non-issues in the delivered artifacts).

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Phase 17's bookkeeping gaps are closed: every verification this phase could not run is on `WINDOWS.md` with its reason (ids 5, 6, 8-16 pre-existing plus 17, 18, 19 new), IN-01 is tracked as accepted debt with its human-decision provenance, and the CI blind spot CR-01 exposed now has a dedicated, gated job.
- Phase 17 is ready for re-verification (`/gsd-verify-work` or a fresh `17-VERIFICATION.md` pass) with all five in-scope findings (CR-01, WR-01, WR-02, WR-03-new, WR-04-new) closed and this run's own bookkeeping complete.
- **Blocker for phase close, not for re-verification:** `REQUIREMENTS.md`'s PROV-01/PROV-02/PROV-03/PROV-04 checkboxes remain open pending the phase-close-time adjudication across all eleven contributing plans — see the dedicated section above.
- This is the last plan of the gap-closure run and the last plan of Phase 17's wave 4. The orchestrator owns `STATE.md`/`ROADMAP.md`/`REQUIREMENTS.md` updates centrally after this worktree merges.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-18*
