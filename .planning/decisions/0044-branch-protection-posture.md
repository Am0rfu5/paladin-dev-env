# ADR-0044: Branch protection posture — required checks, review count, and bypass removal

## Status

Accepted

**Date:** 2026-08-14

## Context

Two ruleset payloads (`protect-main-branch.json`, `protect-release-tags.json`) sat committed to
`.github/rulesets/` and unapplied since Milestone 10, Epic 5 (2026-05-31), verified this session:
`gh api /repos/DF3NDR/paladin-dev-env/rulesets` returned `[]` and `gh api
.../branches/main/protection` returned `{"message":"Branch not protected","status":"404"}` for both
`main` and `release/v0.7.0`. Neither the trunk nor the branch doing integration duty had any
protection at all, so no status check — passing or failing — could block anything.

**The committed payload was self-defeating as written.** `protect-main-branch.json` set
`required_approving_review_count: 1` against a repository with exactly one collaborator (`Am0rfu5`,
confirmed via the collaborators API). GitHub forbids self-approval, so the requirement was
satisfiable only through the ruleset's own `bypass_actors` entry — `actor_id: 5` (`RepositoryRole` =
Admin), `bypass_mode: always`. That bypass does not selectively skip the review requirement; it
skips the **required status checks** rule as well. Applying the payload as committed would have left
the state SC2 diagnosed — "no check can block anything" — true for the only account that ever opens
a PR here.

**The cost profile behind the required-check set is lopsided.** From the measured pull-request run
(`31727496744`): `Docker Build` measured **3762 seconds (62.7 minutes)** — the entire run's critical
path, building `linux/amd64,linux/arm64` with `arm64` under QEMU emulation. Every other job measured
under 400 seconds; the slowest job that does join the required set, `Integration Tests`, measured
398s. Combined with the committed ruleset's `strict_required_status_checks_policy: true`, requiring
`Docker Build` would serialize every merge behind an hour-plus emulation run.

## Decision

**The required-check set is every job except `Docker Build` and `Kubernetes Smoke Test`** — 44
contexts, applied and read back from the server (`gh api
/repos/DF3NDR/paladin-dev-env/rulesets/20868126`, required-check list identical to the committed
payload, `server: 44 | committed: 44 | identical: True`). Both excluded jobs still run on every push
and pull request; they simply do not gate the merge button. The exclusion is a recorded decision
with a measurement behind it (**3762s** for `Docker Build` against a required-set critical path of
roughly seven minutes), not an omission — the alternative of taking up a native-arm64 CI rework so
`Docker Build` could join the required set was offered with its scope cost stated, and declined for
this phase.

**`required_approving_review_count` is `0`.** A pull request stays mandatory — every required check
must still pass before the merge button is available, and force-pushes and branch deletion stay
blocked — but no second human approval is required, because the repository has exactly one active
collaborator and GitHub does not allow self-approval. If the project gains a second active
committer, the approval count is the thing to revisit; the PR-and-required-checks requirement does
not change either way.

**The administrative bypass is removed on the trunk ruleset, and deliberately retained on the tag
ruleset — an asymmetry, not an inconsistency.** `protect-main-branch.json`, applied as "Protect main
branch" (id `20868126`), carries zero bypass actors (`gh api
.../rulesets/20868126 --jq '{name, bypass:(.bypass_actors|length)}'` → `{"bypass":0,"name":"Protect
main branch"}`) — a merge gate any account can bypass at will is not a gate, only a suggestion.
`protect-release-tags.json`, applied as "Protect release tags" (id `20868099`), retains its single
bypass actor (`{"bypass":1,"name":"Protect release tags"}`) because it restricts tag **creation**,
not a merge: without a bypass actor, `v*` tag creation would be restricted to nobody, and no release
could ever be cut. The sole collaborator is the account that cuts releases; the retained bypass is
what makes the tag ruleset usable rather than a self-lock.

**A third ruleset pre-emptively covers the release-branch ref class.** `protect-release-branches.json`
targets `refs/heads/release/*` with the same required-PR, no-bypass posture as the trunk ruleset
(applied as "Protect release branches", id `20868128`), so a future backport branch is born
protected rather than needing a second retrofit.

Both exclusions' measurement — Docker Build's 3762 seconds against a required-set critical path of
roughly seven minutes — is recorded here by number, not left implicit in the ruleset JSON alone.

## Considered Options

- **Take up the native-arm64 CI rework so `Docker Build` could join the required set** (rejected for this phase) — offered with its scope cost stated: replacing QEMU emulation with native `ubuntu-24.04-arm` runners is a runner-topology change to an unrelated job, out of this phase's scope, and left as a future infrastructure phase's reopening condition (see the milestone ledger's amended "native-arm64 CI rework" entry).
- **Prepare the payloads without applying them** (rejected) — this reproduces the exact failure this phase exists to fix: the two rulesets had already sat committed and unapplied for months, which is the state SC2's diagnosis names directly.
- **Narrow the bypass instead of removing it** (rejected) — a narrowed bypass (e.g. restricted to a specific emergency scenario) still lets the sole administrator skip the required status checks from inside a pull request whenever the narrowing condition is met; removing it entirely is the only form that makes the merge gate actually gate for the one active committer.
- **Apply the same no-bypass posture to the tag ruleset, for symmetry with the trunk ruleset** (rejected) — the tag ruleset restricts ref *creation*, not a merge; removing its bypass actor would make cutting any release impossible, since the sole collaborator would then be unable to satisfy the ruleset's own creation restriction. The asymmetry is required by what each ruleset actually gates, not an oversight to correct.

## Code Locations

- `.github/rulesets/protect-main-branch.json` — 44 required-status-check contexts, `required_approving_review_count: 0`, no bypass actors
- `.github/rulesets/protect-release-tags.json` — `refs/tags/v*` creation/deletion restriction, one retained bypass actor (`actor_id: 5`, `RepositoryRole`, `bypass_mode: always`)
- `.github/rulesets/protect-release-branches.json` — `refs/heads/release/*`, same posture as the trunk ruleset
- `docs/src/appendix/branch-protection.md` — the administrator-facing enforcement page, brought current with the applied state this record ratifies

## Code Conformance

conforms

Applied and server-verified 2026-08-14, per `15.1-RULESET-EVIDENCE.md`: the ruleset collection went
from `[]` to three active rulesets (`20868126` branch, `20868128` branch, `20868099` tag, all
`enforcement: active`); `gh api .../rules/branches/main` confirms all four rule types
(`deletion`, `non_fast_forward`, `pull_request`, `required_status_checks`) evaluate against
`refs/heads/main`; the required-check list matches the committed payload exactly (44/44, zero
server-only, zero committed-only); the bypass asymmetry is confirmed on the live server
(`bypass:0` on the trunk ruleset, `bypass:1` on the tag ruleset). No code change is made by this
record beyond what plan `15.1-08` already applied; this ADR ratifies that applied state.

## Downstream Consumers

- **`scripts/check-workflow-triggers.sh`'s `CLAUSE_CONTEXT`** — asserts every one of the 44 pinned
  contexts resolves to a declared job name in a current workflow file, so a job rename does not
  silently drop coverage from the required set this record names.
- **`docs/src/appendix/branch-protection.md`** — the administrator-facing description of the
  applied state; a reader auditing or re-applying the rulesets reads this record's Decision section
  for the reasoning behind each parameter, and the page for the mechanical import steps.
- **A future infrastructure phase taking up the native-arm64 rework** — inherits the reopening
  condition this record and the milestone ledger both name: once `Docker Build` runs natively rather
  than under QEMU, the 3762-second measurement that justifies its exclusion no longer holds, and
  re-evaluating its inclusion in the required set is the next step, not a fresh decision from
  scratch.
