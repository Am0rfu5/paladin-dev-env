---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
verified: 2026-08-30T19:32:42Z
status: passed
score: 9/9 must-haves verified
behavior_unverified: 0
overrides_applied: 0
---

# Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate Verification Report

**Phase Goal:** A release that fails partway through must be finishable (same-tag re-run is the
supported recovery, reaching the publish step), and no release begins until the tag, all eleven
manifest versions, the root + ten crate changelogs, and the tagged commit's recorded CI conclusion
agree. Already-published is determined from registry state, not error prose; the fixed sleep-20
index wait is replaced by a real check; a run that publishes nothing does not report success and
one that publishes some records which per crate; the stuck-halfway runbook exists with a yank
policy; and the recovery path was exercised against an induced partial failure.

**Verified:** 2026-08-30T19:32:42Z
**Status:** passed
**Re-verification:** No — initial verification

## Verification method

Beyond static code inspection, this verification independently re-executed the phase's own
regression harnesses (`make test-shell-guards`, 7 suites / 177 assertions, all green), ran the
gate script directly against the live tree, and — because SUMMARY.md and even the phase's own
evidence file are not treated as self-certifying — used an authenticated `gh` CLI session to
independently query the two GitHub Actions run IDs cited in `20-RECOVERY-EVIDENCE.md`
(`33210072054`, `33322587044`) rather than trusting the transcribed conclusions. The live query
confirms, job-by-job, that every job on the actual tag→publish path (`Verify Tag From Main`,
`Create Release`, `Pre-Publish Consistency Gate`, `Test Suite`, `Publish to crates.io`) concluded
`success` on the recovery run, and that the `Publish to crates.io` job's own log contains both
"already on crates.io — skipping publish" and "visible in the sparse index after N poll
iteration(s)" lines — an independent, non-transcribed confirmation of the mixed
already-at-this-version / published-now split the evidence file claims. Only the four
pre-existing, out-of-scope `Build Binaries` matrix jobs (Phase 21 territory, WR-05) failed, which
is why the run's aggregate conclusion reads `failure` while the publish path itself is clean.

## Goal Achievement

### Observable Truths

| # | Truth (mapped to ROADMAP.md Phase 20 success criteria) | Status | Evidence |
|---|------|--------|----------|
| 1 | Re-running a release on the same tag is the supported recovery and reaches the publish step | ✓ VERIFIED | `scripts/create-or-reuse-release.sh` (HTTP-status create-or-reuse decision, 12+ assertions in its harness); `release.yml` `create-release` job rewritten, `actions/create-release@v1` fully removed (`grep -c 'actions/create-release'` = 0); live-exercised twice in `20-RECOVERY-EVIDENCE.md` and independently re-confirmed via `gh api .../jobs` — `Create Release` job = `success` on the rc.4 recovery run, reusing release `379337092` rather than 422'ing |
| 2 | Already-published decided from registry state, not error prose; fixed `sleep 20` replaced by a bounded real check | ✓ VERIFIED | `scripts/publish-crates.sh`: `_pc_precheck_published()` queries `crates.io/api/v1/crates/<name>/<version>` (200/404/429/other), `_pc_wait_for_index()` polls the sparse index with `--poll-timeout`/`--poll-interval`; `grep -c 'sleep 20'` = 0 in `release.yml` and scripts (one comment-only mention documenting the removal); no `grep -qiE "already (exists\|uploaded)"` tolerance remains; live log for run 33322587044 shows both `"already on crates.io -- skipping publish"` and `"visible in the sparse index after 1 poll iteration(s)"` lines |
| 3 | Nothing is published until the tag, every manifest version and every changelog agree; gate names every mismatch, not the first | ✓ VERIFIED | `scripts/check-release-consistency.sh` clauses 1–2, live-run against `--tag v9.9.9` names all 11 mismatched manifests and all 11 missing changelog sections in one collect-then-report pass (verified directly); `release.yml` `publish-crates.needs` includes `check-release-consistency` (confirmed via `yaml.safe_load`); `check-release-consistency` job holds `permissions: {contents: read, actions: read}` only, no `id-token`/write |
| 4 | "Tagged commit passed CI" is verified against a recorded run, not inferred from branch membership | ✓ VERIFIED | `scripts/check-release-consistency.sh` clause 3 resolves `ci.yml`'s recorded conclusion for the tagged SHA via `gh api .../actions/workflows/ci.yml/runs?head_sha=...`, sorts by `created_at`/`id`, fails closed (`CI_LOOKUP_FAILED`) on transport error, `MISSING_SHA` in CI with no `--sha`; live-exercised: rc.4 attempt 1 was correctly refused with `CI_MISMATCH` before CI had completed on that commit, recovering by re-run rather than re-tag — exactly the documented remedy |
| 4b (backstop) | The whole-run CI-conclusion granularity remains correct for `ci.yml`'s current job list — no non-blocking job can flip the run red | ✓ VERIFIED | Re-derived directly from live `.github/workflows/ci.yml`: `benchmark-regression-signal` (the one job carrying `continue-on-error: true` at job level) is restricted by `if: github.event_name == 'pull_request' \|\| github.event_name == 'workflow_dispatch'` — it does not run on the `push` event this clause inspects; `osv-scanner`'s two `continue-on-error: true` are step-level only, so the job's own conclusion is unaffected by scan findings and it cannot flip the run red either. The script header's documented rationale matches the live file exactly. |
| 5 | A run that publishes nothing does not report success; one that publishes some records which per crate | ✓ VERIFIED | `scripts/publish-crates.sh`: four named states (`published-now`/`already-at-this-version`/`skipped`/`failed`), no-crate-moved rule (`grep -q 'release-recovery.md'` present in the failure path), tested with 20+ fixture cases; live rc.4 recovery run's outcome table: 5 `already-at-this-version`, 6 `published-now` (independently confirmed via job log, not transcribed) |
| 5b (backstop) | Assumption A3 — recovery re-run preserves the tag ref the `crates-io` environment's OIDC policy requires, so the token exchange succeeds on re-run | ✓ VERIFIED | `20-RECOVERY-EVIDENCE.md` records independent crates.io `trustpub_data.run_id` queries matching both run IDs (`33210072054`, `33322587044`); independently re-confirmed here: `gh api .../jobs` for run `33322587044` shows the `Publish to crates.io` job (which mints the OIDC token) concluded `success` |
| 6 | Stuck-halfway runbook exists, names a yank policy, cross-linked and reachable | ✓ VERIFIED | `docs/src/appendix/release-recovery.md` exists (14.6KB), registered in `docs/src/SUMMARY.md`, cross-linked bidirectionally with `release-automation.md`/`release-checklist.md` (confirmed via grep both directions); contains a "Yank register" table (Version/Crates/Reason/Owner/Date), states who may yank (crate-owner account, not CI) and that no workflow/script/Makefile target performs one (`grep -rnE 'cargo yank\|/yank' scripts .github/workflows Makefile` = empty); `make check-doc-config` passes (150 YAML blocks, 0 failed) |
| 7 | Recovery path exercised against a real induced partial failure, not merely written | ✓ VERIFIED | `20-RECOVERY-EVIDENCE.md`: two full rehearsals (`v0.8.1-rc.3` pre-Phase-20, `v0.8.1-rc.4` on Phase 20's own live gate/scripts), each with a genuine three-moment registry split (4-of-11 at 200 mid-interruption for rc.3; a real `failed`+`skipped` outcome table for rc.4), `cargo publish --dry-run` explicitly stated as never used as evidence; runbook status line updated from "untested" to "tested (2026-08-30)" with both run URLs — independently re-verified against GitHub (not transcribed): both run IDs resolve, the recovery run's publish-path jobs are all `success`, matching the claimed outcome |

**Score:** 9/9 truths verified (7 roadmap success criteria + 2 explicitly-flagged `backstop` items
from plan frontmatter, both confirmed with direct primary evidence rather than abstained)

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `scripts/check-release-consistency.sh` | Pre-publish gate, 3 clauses | ✓ VERIFIED | Exists, executable, shellcheck-clean, live-run confirms all 3 clauses fire correctly |
| `tests/scripts/check-release-consistency_test.sh` | Regression harness | ✓ VERIFIED | 25 assertions pass |
| `scripts/create-or-reuse-release.sh` | HTTP-status create-or-reuse | ✓ VERIFIED | Exists, shellcheck-clean, live-exercised (reuse path taken on rc.4 attempt 3) |
| `tests/scripts/create-or-reuse-release_test.sh` | Regression harness | ✓ VERIFIED | 21 assertions pass |
| `scripts/publish-crates.sh` | Registry-state publish loop, outcome table | ✓ VERIFIED | Exists, shellcheck-clean, no bare `sleep`, `User-Agent` present, no `-L`/`--location` |
| `tests/scripts/publish-crates_test.sh` | Regression harness | ✓ VERIFIED | 54 assertions pass |
| `scripts/finalize-crate-changelogs.sh` | Idempotent per-crate changelog stamping | ✓ VERIFIED | Exists, shellcheck-clean; all 11 changelogs presently carry `## [0.8.1-rc.4]` (confirmed live) |
| `tests/scripts/finalize-crate-changelogs_test.sh` | Regression harness | ✓ VERIFIED | 19 assertions pass |
| `.github/workflows/release.yml` job `check-release-consistency` | Gate job, `publish-crates` needs it | ✓ VERIFIED | Present, `needs: verify-tag-source`; `publish-crates.needs` = `[test, create-release, check-release-consistency]` (confirmed via YAML parse) |
| `docs/src/appendix/release-recovery.md` | Runbook + yank policy + register | ✓ VERIFIED | Exists, registered in SUMMARY.md, cross-linked, yank register present, status line "tested (2026-08-30)" |
| `.planning/phases/20-*/20-RECOVERY-EVIDENCE.md` | Rehearsal evidence log | ✓ VERIFIED | Exists, 444 lines, two full rehearsals recorded with independently-checkable run IDs |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `release.yml publish-crates.needs` | `check-release-consistency` job | `needs:` array | ✓ WIRED | Confirmed via `yaml.safe_load` |
| `release.yml check-release-consistency` job | `scripts/check-release-consistency.sh` | thin `run:` invocation via `env:` | ✓ WIRED | `env: RELEASE_TAG, RELEASE_SHA, GH_TOKEN` then `run: ./scripts/check-release-consistency.sh --tag "$RELEASE_TAG" --sha "$RELEASE_SHA"` — no `${{ }}` interpolated into `run:` body |
| `Makefile check-release-consistency` target | `scripts/check-release-consistency.sh` | same script, local runnability | ✓ WIRED | `make check-release-consistency RELEASE_TAG=v0.8.1-rc.4` exits 0 against live tree; no `RELEASE_TAG` exits non-zero naming it |
| `release.yml create-release` job | `scripts/create-or-reuse-release.sh` | thin `run:` invocation via `env:` | ✓ WIRED | `create_or_reuse_release` step id present, outputs `upload_url`/`version` preserved for `build-binaries`/`sbom` consumers (3 references each, unchanged) |
| `release.yml publish-crates` job | `scripts/publish-crates.sh` | thin `run:` invocation via `env:` | ✓ WIRED | No inline `CRATES=(` array, no grep-tolerance, no `sleep` literal remain in the YAML |
| `make release` | `scripts/finalize-crate-changelogs.sh` then `check-release-consistency` then `git tag` | ordered `make -n release` dry-run | ✓ WIRED | Confirmed order via `make -n release VERSION=9.9.9`: finalize → gate → tag |
| `docs/src/SUMMARY.md` | `appendix/release-recovery.md` | book registration | ✓ WIRED | Entry present, positioned after "Release Checklist" |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Gate passes on the real tree at its current tag | `./scripts/check-release-consistency.sh --tag v0.8.1-rc.4` | `✅ OK: 11 publishable package(s) checked...` | ✓ PASS |
| Gate reports every mismatch, not the first | `./scripts/check-release-consistency.sh --tag v9.9.9` | Names all 11 manifest mismatches + all 11 changelog issues in one report | ✓ PASS |
| `MISSING_SHA` fires on the CI path with no `--sha` | `GITHUB_ACTIONS=true ./scripts/check-release-consistency.sh --tag v0.8.1-rc.4` | `❌ ... failed (MISSING_SHA)` | ✓ PASS |
| `make check-release-consistency` with no `RELEASE_TAG` fails with usage error | `make check-release-consistency` | `❌ RELEASE_TAG is required. Usage: ...` | ✓ PASS |
| All 7 shell-guard regression harnesses pass | `make test-shell-guards` | 11+32+21+11+19+25+54 = 173 assertions, all green | ✓ PASS |
| `make check-workflow-triggers` still passes (no trigger surface changed) | `make check-workflow-triggers` | `7 workflow file(s) scanned... all pass` | ✓ PASS |
| `make check-doc-config` passes (runbook YAML fences valid) | `make check-doc-config` | `150 YAML block(s) checked, 0 failed` | ✓ PASS |
| Recovery run's publish-path jobs are all `success`, independently queried | `gh api repos/DF3NDR/paladin-dev-env/actions/runs/33322587044/jobs` | `Verify Tag From Main / Create Release / Pre-Publish Consistency Gate / Test Suite / Publish to crates.io` all `success`; only `Build Binaries` (4 legs, Phase 21 scope) `failure` | ✓ PASS |
| Recovery run's publish log shows a genuine mixed split | `gh run view 33322587044 --job 99295171217` | Log contains both `"already on crates.io -- skipping publish"` and `"visible in the sparse index after N poll iteration(s)"` | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| PUBOPS-01 | 20-01, 20-02, 20-04 | Gate blocks publish until tag/manifests/changelogs agree, reports every mismatch | ✓ SATISFIED | Gate exists, wired into `publish-crates.needs`, live-run confirms both success and failure paths |
| PUBOPS-02 | 20-02 | CI conclusion verified against a recorded run for the tagged SHA | ✓ SATISFIED | Clause 3 implemented, fails closed on lookup error/missing SHA, live-exercised `CI_MISMATCH` refusal and recovery |
| PUBOPS-03 | 20-01, 20-03, 20-05, 20-07 | Idempotent end-to-end re-run; already-published from registry state | ✓ SATISFIED | `create-release` reuse path live-exercised twice; publish loop registry-state detection + bounded poll, live-confirmed mixed outcome |
| PUBOPS-04 | 20-05 | Publish-nothing does not report success; per-crate outcomes recorded | ✓ SATISFIED | Four named states, no-crate-moved failure rule, tested and (indirectly, via the mixed-split live run) exercised |
| PUBOPS-05 | 20-06, 20-07 | Runbook with yank policy, exercised not just written | ✓ SATISFIED | Runbook exists, cross-linked, yank register present, rehearsal recorded twice with independently-verifiable run IDs |

**Note on REQUIREMENTS.md checkbox state:** at the time of this verification, `REQUIREMENTS.md`'s
checkboxes show PUBOPS-01/02/04 as `[ ]` and its traceability table lists them `Pending`, while
PUBOPS-03/05 show `[x]`/`Complete`. This is a bookkeeping lag, not a code gap — all five are
satisfied in the codebase per the evidence above. Updating the checkboxes/table is a doc-sync step
outside this verifier's remit; flagging it here so the ship/close step catches it.

### Anti-Patterns Found

No debt markers (`TBD`/`FIXME`/`XXX`) in any file this phase modified (the `XXXXXX` matches found
are `mktemp` template placeholders, not debt markers). No `TODO`/`HACK`/`PLACEHOLDER` strings.
`shellcheck --severity=warning` is clean on all 4 new scripts and their 4 harnesses.

The phase's own `20-REVIEW.md` (code review, run 2026-08-30) recorded 2 Critical and 6 Warning
findings. Both Criticals are outside this phase's exercised scope and do not contradict any
must-have in the 7 plans:

| Severity | Finding | In scope of Phase 20's must-haves? | Disposition |
|---|---|---|---|
| Critical (CR-01) | `verify-tag-source`'s `git rev-list -n 1 -- "$RELEASE_TAG"` is invalid git usage for the `workflow_dispatch` path (pre-existing code; Phase 20 only added an `outputs: sha` wrapper around it, per 20-03's plan text) — every `workflow_dispatch` run fails immediately at the first job | No — every must-have and every rehearsal in this phase used tag-push events, never `workflow_dispatch`. D-15 explicitly designs recovery around the tag-push re-run and calls `workflow_dispatch` eligibility "untested" by design | ⚠️ WARNING — real defect, pre-existing, does not block phase-goal achievement; should be fixed before `workflow_dispatch` is relied on |
| Critical (CR-02) | `make publish-dry-run` (pre-existing Makefile target, not touched by any Phase 20 plan's file list) uses wrong package names (`paladin-core`/`paladin` instead of `paladin-ai-core`/`paladin-ai`), omits `paladin-herald`, and masks all three with `\|\| true` | No — this target is not in any Phase 20 plan's `files_modified` list, is not a must-have artifact, and is explicitly called out as pre-existing in this verification's scope notes | ⚠️ WARNING — real defect, pre-existing, worth a follow-up fix but not a Phase 20 must-have |

Both are recorded here as WARNING (not BLOCKER) because neither falsifies a Phase 20 must-have
truth, artifact, or key link — they live in code paths this phase's plans deliberately did not
touch or exercise. The 6 additional Warning/Info findings in `20-REVIEW.md` are lower-severity
code-quality items (dry-run exit-code swallowing, missing `ref:` pin on `workflow_dispatch`
checkout, stderr/stdout interleaving in one HTTP-status parse, one un-indirected `${{ }}` in
`build-docker`, duplicated version-resolution logic, a dead test assertion, a stale comment in
`release.toml`) — none contradict a must-have either.

### Human Verification Required

None. Every must-have truth resolved to VERIFIED with direct, independently-reproducible evidence
(live script execution against the real tree, live `gh api`/`gh run view` queries against the two
cited GitHub Actions runs, and passing regression harnesses) — no item required subjective human
judgment beyond what this verification already performed.

### Gaps Summary

No gaps against the phase's must-haves. The two Critical findings in `20-REVIEW.md` are real,
pre-existing defects worth fixing, but both sit in code paths (`workflow_dispatch` triggering, the
standalone `make publish-dry-run` target) that no Phase 20 plan's must-haves, artifacts, or key
links claim to have fixed or exercised — the phase's own scope and its two live rehearsals were
built entirely around the tag-push path, which this verification confirms works end-to-end,
including a real recovery from a genuinely half-published state, independently re-checked against
GitHub rather than taken on the evidence file's word alone.

---

_Verified: 2026-08-30T19:32:42Z_
_Verifier: Claude (gsd-verifier)_
