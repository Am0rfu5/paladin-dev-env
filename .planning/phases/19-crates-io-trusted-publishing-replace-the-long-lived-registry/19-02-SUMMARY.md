---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
plan: 02
subsystem: infra
tags: [crates-io, github-actions, oidc, trusted-publishing, github-environments]

requires:
  - phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
    provides: eleven-crate publish set reconciled in release.yml's CRATES array; all eleven crates live on crates.io at 0.8.1-rc.1 (19-01)
provides:
  - "crates-io GitHub Environment on DF3NDR/paladin-dev-env, deployment policy restricted to v*.*.* tags, no reviewer gate, no secrets"
  - "publish-crates job rewritten to mint its crates.io credential per run via rust-lang/crates-io-auth-action@v1 (job-scoped id-token: write, environment: crates-io)"
  - "the dry_run=skip silent-success branch deleted (not rewritten) -- a real-mode publish run can no longer report success without publishing"
  - "human-confirmed environment posture (D-08: no required-reviewer gate) recorded in 19-PUBLISH-EVIDENCE.md"
affects: [19-03, 19-04, 19-05, phase-20-publish-ops]

tech-stack:
  added: [rust-lang/crates-io-auth-action@v1]
  patterns: [job-scoped OIDC permission + protected GitHub Environment, GitHub Environment tag-ref restriction via deployment-branch-policies]

key-files:
  created: []
  modified:
    - .github/workflows/release.yml
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md

key-decisions:
  - "crates-io environment created via gh api on the first attempt (both PUT and POST calls returned 2xx) -- no 403, no web-UI fallback needed"
  - "D-08 confirmed by the repository owner as the plan's deliberate default: no required-reviewer gate on crates-io, unattended tag-push releases stay working"

patterns-established:
  - "Job-level environment: key + job-scoped permissions.id-token: write, never workflow-level permissions -- keeps the OIDC subject claim narrow and preserves this file's per-job least-privilege shape"
  - "Deleted modes are deleted, not rewritten with a warning-and-continue -- a three-way conditional that can silently skip work is the defect class this plan exists to remove (D-09)"

requirements-completed: [PUB-02, PUB-05]

coverage:
  - id: D1
    description: "publish-crates job obtains its crates.io credential at run time from rust-lang/crates-io-auth-action@v1 via steps.auth.outputs.token; no step reads a repository secret for the registry"
    requirement: "PUB-02"
    verification:
      - kind: unit
        ref: "python3 yaml structural assertion: job['permissions']=={'contents':'read','id-token':'write'}, auth step uses rust-lang/crates-io-auth-action@v1 with if steps.mode.outputs.dry_run != 'true', publish step env.CARGO_REGISTRY_TOKEN == steps.auth.outputs.token"
        status: pass
      - kind: unit
        ref: "grep -c 'secrets.CARGO_REGISTRY_TOKEN' .github/workflows/release.yml == 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "publish-crates runs under the crates-io GitHub Environment restricted to v*.*.* tags, with id-token: write scoped to the job only (no workflow-level permissions key)"
    requirement: "PUB-02"
    verification:
      - kind: unit
        ref: "python3 yaml: 'permissions' not in top-level doc; job['environment']=='crates-io'"
        status: pass
      - kind: other
        ref: "gh api repos/DF3NDR/paladin-dev-env/environments/crates-io/deployment-branch-policies --jq '[.branch_policies[]|select(.type==\"tag\" and .name==\"v*.*.*\")]|length' == 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "a real-mode release run that publishes nothing cannot end green -- the dry_run=skip branch and its ::warning:: are deleted, and the credential step has no continue-on-error"
    requirement: "PUB-05"
    verification:
      - kind: unit
        ref: "grep -c 'dry_run=skip' .github/workflows/release.yml == 0; grep -c 'continue-on-error' .github/workflows/release.yml == 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "the crates-io GitHub Environment exists with the correct tag policy, no reviewer gate and no secrets, and a human confirmed that posture before any release depends on it"
    verification:
      - kind: manual_procedural
        ref: "checkpoint:human-verify Task 3, resolved 2026-08-27 by Am0rfu5 -- 'approved', reviewer-gate decision: none (D-08)"
        status: pass
    human_judgment: true
    rationale: "Repository-settings mutation and its security posture (no required-reviewer gate) is a deliberate human decision, not something automation can rubber-stamp -- exactly why the plan gated it behind a blocking checkpoint."

duration: ~25min
completed: 2026-08-27
status: complete
---

# Phase 19 Plan 02: OIDC credential minting and the crates-io environment gate

**`publish-crates` now mints a short-lived crates.io token per run via `rust-lang/crates-io-auth-action@v1`, gated behind a new `crates-io` GitHub Environment restricted to `v*.*.*` tags, with the old silent-skip publish mode deleted outright.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-27T11:30:00Z
- **Completed:** 2026-08-27T11:55:00Z
- **Tasks:** 3 (2 auto + 1 checkpoint:human-verify)
- **Files modified:** 2

## Accomplishments
- Created the `crates-io` repository Environment on `DF3NDR/paladin-dev-env` with a deployment policy admitting only `v*.*.*` tags (typed as a tag rule, not a branch rule), zero reviewer gates, zero secrets/variables.
- Rewrote the `publish-crates` job: `environment: crates-io` at job level, `id-token: write` added inside the job's own `permissions:` block (no workflow-level `permissions:` key introduced), a new `Authenticate with crates.io` step (`id: auth`, `rust-lang/crates-io-auth-action@v1`) guarded on `dry_run != 'true'`, and the publish step reading `CARGO_REGISTRY_TOKEN` from `steps.auth.outputs.token`.
- Deleted the `dry_run=skip` branch and its `::warning::` — the `Determine publish mode` step now produces exactly `dry_run=true` or `dry_run=false`, closing the failure mode where a real-mode run could report success while publishing nothing (T-19-09).
- Obtained and recorded human confirmation of the environment posture, including the deliberate D-08 no-required-reviewer decision, in `19-PUBLISH-EVIDENCE.md`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create the `crates-io` environment and restrict it to release tags** - `19285c39` (feat)
2. **Task 2: Rewrite the `publish-crates` credential path and delete the silent-skip branch** - `b12789a6` (feat)
3. **Task 3: Confirm the repository-settings change and the rewritten job (checkpoint resolution)** - `0e860e71` (docs)

_No separate plan-metadata commit — this SUMMARY commit is the final commit for this plan in worktree mode._

## Files Created/Modified
- `.github/workflows/release.yml` - `publish-crates` job: `environment: crates-io`, job-scoped `id-token: write`, two-branch publish mode, new OIDC auth step, publish step reads the minted token
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md` - new `### Environment Posture` subsection (policy, no-reviewer rationale, live verification output) and `### Human Confirmation` subsection recording the checkpoint resolution

## Decisions Made
- **`crates-io` environment created via `gh api` on the first attempt.** Both the `PUT .../environments/crates-io` and `POST .../deployment-branch-policies` calls returned 2xx immediately — the fine-grained PAT carried sufficient repository-administration scope, so the documented 403/web-UI fallback path was not needed.
- **D-08 confirmed, not revisited.** The repository owner approved the plan's default posture (no required-reviewer gate on `crates-io`) rather than requesting one added. Unattended tag-push releases stay working; the ref restriction (`v*.*.*` tags only) is the protection.

## Deviations from Plan

None — plan executed exactly as written. The only operational adjustment was committing with `--no-verify` on all three task commits: this project's pre-commit hook runs full-workspace `cargo clippy`, which exceeded the 2-minute command timeout on the first attempt (confirmed via `git log`/`git status` that no partial commit was left behind before retrying). `workflow.worktree_skip_hooks: true` in `.planning/config.json` explicitly permits this for worktree executors; the orchestrator runs a post-wave hook validation pass. This is not a deviation rule 1-4 case — no code behavior changed, only how the commit was invoked.

## Issues Encountered
- The sandboxed Bash tool refused several multi-line composite commands (`&&`-chained `gh api`/`grep` pipelines, and a multi-step git branch-verification block) as "too complex to verify worktree containment." Each was split into individual single-purpose commands and re-run successfully — no functional impact, only extra round-trips.

## User Setup Required
None - no external service configuration required. The `gh api` environment creation and the checkpoint confirmation were both completed within this session; no dashboard step was deferred to the user.

## Next Phase Readiness
- `release.yml`'s `publish-crates` job is structurally ready to authenticate via OIDC, but **no claim is made here that the OIDC exchange itself works** — no Trusted Publishing configuration exists yet on crates.io for any of the eleven crates, so a real (non-dry-run) release run would currently fail at the `Authenticate with crates.io` step with no trust link configured. That proof event is 19-03's job.
- The `crates-io` environment name (`DF3NDR/paladin-dev-env` → `environment:crates-io` in the OIDC subject claim) is now load-bearing and recorded in `19-PUBLISH-EVIDENCE.md`'s `### Environment Posture` subsection — 19-03 must pin this exact string in each of the eleven per-crate Trusted Publishing configurations.
- No blockers for 19-03.

---
*Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry*
*Completed: 2026-08-27*
