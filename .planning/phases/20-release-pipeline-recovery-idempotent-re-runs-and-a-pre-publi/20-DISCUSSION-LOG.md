# Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-28
**Phase:** 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
**Mode:** `--auto` — all gray areas auto-selected; recommended option taken on each question.
**Areas discussed:** Create-release idempotency, Registry-state detection & index wait, Pre-publish gate design, CI-conclusion verification, Per-crate outcome reporting, Runbook & yank policy, Recovery rehearsal design

---

## Create-release idempotency

| Option | Description | Selected |
|--------|-------------|----------|
| `gh` CLI create-or-reuse, outputs preserved | No new third-party action; `gh api` fetches `upload_url` on reuse path so `build-binaries`/`sbom` keep working until Phase 21 | ✓ |
| `softprops/action-gh-release` | Popular replacement action; adds a third-party dependency Phase 21 would churn again | |
| Pre-check + skip around `create-release@v1` | Keeps an archived action in the tree; only papers over the 422 | |

**Notes:** `upload_url`/`version` outputs contract preserved deliberately (D-02); removal is Phase 21's job.

---

## Registry-state detection & index wait

| Option | Description | Selected |
|--------|-------------|----------|
| crates.io API pre-check per crate + poll-until-visible | Registry state, never error prose; bounded-timeout poll replaces `sleep 20` | ✓ |
| `cargo publish --workspace` | Native ordering/index-wait; adoption conditional on researcher verifying partial-published tolerance and per-crate outcome derivability (D-06) | (conditional) |
| Keep grep tolerance, widen the regex | Rejected — same fragility PUBOPS-03 names | |

---

## Pre-publish gate design

| Option | Description | Selected |
|--------|-------------|----------|
| Repo script + gate job before first publish; strict for prereleases too; extend release tooling to finalize crate changelogs | Locally runnable, Phase 15.1 pattern; no untested exempted path | ✓ |
| Inline YAML-only checks in publish job | Not locally testable; buried in workflow | |
| Exempt prereleases from changelog checks | Rejected — the rehearsal runs on a prerelease, so the exempted path would be the tested one and the strict path untested | |

---

## CI-conclusion verification

| Option | Description | Selected |
|--------|-------------|----------|
| Resolve recorded `ci.yml` conclusion for tagged SHA via GitHub API | No duplicated jobs, no trigger changes; honest refusal when no success recorded | ✓ |
| Run equivalent checks inside `release.yml` | Duplicates eighteen jobs; drift-prone | |
| Add tag trigger to `ci.yml` | Rejected — gated by `check-workflow-triggers.sh`, redundant with recorded main run | |

---

## Per-crate outcome reporting

| Option | Description | Selected |
|--------|-------------|----------|
| Per-crate table (published-now/already/skipped/failed); zero published-now fails with self-diagnosing message | The honesty posture (Phase 12 / 18 D-06 / 19 D-09) applied to this job | ✓ |
| Warn-and-green with a summary table | Rejected — reproduces the defect PUBOPS-04 exists to remove | |

---

## Runbook & yank policy

| Option | Description | Selected |
|--------|-------------|----------|
| New `docs/src/appendix/release-recovery.md` + yank register table (owner+date) | Beside the docs operators already use; Phase 9/12 convention; out of SECURITY-EXCEPTIONS.md per 19 D-12 reasoning | ✓ |
| Fold into `release-automation.md` | That doc is automation reference; incident procedure deserves its own page | |

---

## Recovery rehearsal design

| Option | Description | Selected |
|--------|-------------|----------|
| Induced mid-loop partial failure on throwaway rc; recovery = re-run same tag-push run; evidence file | Real half-published state, real recovery; avoids untested A1 (`workflow_dispatch` under Trusted Publishing) | ✓ |
| `workflow_dispatch` recovery path | Rejected as the designed path — A1 untested (19-PUBLISH-EVIDENCE); may be incidentally proven, not depended on | |
| Skip rehearsal, label runbook untested | Fallback only — criterion 7 prefers exercised | |

## Claude's Discretion

- Gate script/job/make-target names and `needs` position (before first real publish).
- Loop vs `cargo publish --workspace` carrier, per researcher verification (D-06).
- Induced-failure mechanism and rc version string for the rehearsal.
- CI-check placement (inside gate script vs own step/job).
- Runbook prose structure (D-13 content list binding).

## Deferred Ideas

- Phase 21 `ARTIFACT-*` work (release body, binaries, digest, `upload_url` removal).
- Stable catch-up release (operator act after this phase).
- `crates-io` environment required-reviewer gate (revisit with runbook's approver answer).
- Dedicated A1 (`workflow_dispatch` OIDC) test.
- Reviewed-not-folded todo: local coverage reproduction (carried Phase 19 determination; deviation from auto ≥0.4 fold rule logged in CONTEXT.md).
