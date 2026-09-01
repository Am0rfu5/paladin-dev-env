---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
plan: 03
subsystem: infra
tags: [crates-io, trusted-publishing, oidc, github-actions, release, provenance]

requires:
  - phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
    provides: "publish-crates job rewritten to mint its crates.io credential via OIDC, crates-io GitHub Environment restricted to v*.*.* tags (19-02)"
  - phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
    provides: "all eleven crates live on crates.io at 0.8.1-rc.1, paladin-herald published for the first time (19-01)"
provides:
  - "Eleven crates.io Trust Publisher Configurations, human-reported linked (owner DF3NDR, repo paladin-dev-env, workflow release.yml, environment crates-io), recorded in a Trust Link Ledger"
  - "A real, non-dry-run, tag-push-triggered release (v0.8.1-rc.2) publishing all eleven crates through the OIDC-minted credential — the standing CARGO_REGISTRY_TOKEN was not read"
  - "Registry-side proof: all eleven 0.8.1-rc.2 versions carry non-null trustpub_data (provider github, run_id 33089177606), independently re-queried and contrasted against the 0.8.1-rc.1 null-provenance baseline"
  - "PUB-03 marked complete in REQUIREMENTS.md (checkbox + traceability table)"
affects: [19-04, 19-05]

tech-stack:
  added: []
  patterns:
    - "Registry-side provenance (trustpub_data) as proof, not workflow self-report — a green run and a real OIDC exchange are different claims"
    - "Precondition-halt-then-resume: an unmet plan precondition (19-02 not yet on main) surfaced as a checkpoint rather than silently worked around"

key-files:
  created: []
  modified:
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "Task 2's precondition (19-02's publish-crates rewrite present on origin/main) was unmet when first checked — the phase's whole body of work sat on chore/19-trusted-publishing, unmerged. Per the executor's precondition protocol, this was surfaced as a blocking checkpoint rather than auto-fixed. Resolved by the coordinator as option 1: merge chore/19-trusted-publishing to main now (PR #38, 08aa4528), then cut the proof release."
  - "Task 1's checkpoint resolution (\"linked\", no count, no crate named as unconfigurable) was recorded per its own resume-signal contract as all eleven saved — but explicitly flagged in the ledger as a human-reported claim, not an executor-verified one (crates.io exposes no API to read back Trusted Publishing configs). The proof release's all-success outcome is the closest available corroboration."
  - "Task 2 and Task 3 evidence, though delivered to this executor as one combined coordinator message, were split back into two atomic commits (08e32e89, 531a97d9) plus Task 1's own commit (b5eaedc2) — preserving per-task commit granularity via a targeted git checkout -- <file> revert-and-reapply rather than committing the bundle as one task."

patterns-established:
  - "Split-and-reapply for atomic commits: when a combined edit lands in one Edit call but two plan tasks require separate commits, use git checkout -- <file> (sanctioned single-file revert) to back out to the prior commit, then reapply and commit each task's content independently."

requirements-completed: [PUB-03]

coverage:
  - id: D1
    description: "Eleven crates.io Trust Publisher Configurations created (GitHub owner DF3NDR, repo paladin-dev-env, workflow release.yml, environment crates-io), reported linked by the human operator"
    requirement: "PUB-03"
    verification:
      - kind: manual_procedural
        ref: "checkpoint:human-action Task 1, resolved 2026-08-27 by Am0rfu5 — \"linked\", no exceptions named"
        status: pass
    human_judgment: true
    rationale: "crates.io exposes no public API or CLI to read back a Trusted Publishing configuration, and the settings pages require an authenticated crate-owner session this executor does not hold. Confirmation rests on the human's report; Task 2's all-success proof run is corroborating but not independently conclusive for each of the eleven."
  - id: D2
    description: "Real, non-dry-run, tag-push-triggered v0.8.1-rc.2 release publishes all eleven crates through the rewritten publish-crates job, minting its credential via OIDC rather than reading CARGO_REGISTRY_TOKEN"
    requirement: "PUB-03"
    verification:
      - kind: other
        ref: "gh run view 33089177606 --json jobs -> 'Publish to crates.io' job conclusion: success; 'Authenticate with crates.io' step conclusion: success; event: push; grep -c 'secrets.CARGO_REGISTRY_TOKEN' .github/workflows/release.yml == 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "crates.io's own provenance field (trustpub_data) attributes all eleven 0.8.1-rc.2 versions to the OIDC exchange, contrasted against the 0.8.1-rc.1 null baseline"
    requirement: "PUB-03"
    verification:
      - kind: other
        ref: "19-03-PLAN.md Task 3 <verify> automated script, re-run by this executor against https://crates.io/api/v1/crates/<name>/0.8.1-rc.2 for all eleven crates -> 'PUB-03 evidence OK'"
        status: pass
    human_judgment: false

duration: "spans two human checkpoints (crates.io UI configuration; release cut) — not a tight wall-clock figure; executor-side evidence recording and verification across three resumptions: ~35min"
completed: 2026-08-27
status: complete
---

# Phase 19 Plan 03: Prove the OIDC path publishes before the old credential is destroyed

**All eleven crates published `0.8.1-rc.2` through a real, tag-push-triggered release using the OIDC-minted crates.io credential, and crates.io's own `trustpub_data` field — not a green workflow badge — is the recorded proof, contrasted explicitly against the `0.8.1-rc.1` token-published baseline.**

## Performance

- **Duration:** spans two human checkpoints (see below); executor-side work ~35min across three resumptions
- **Completed:** 2026-08-27
- **Tasks:** 3 (2 checkpoint:human-action + 1 auto), plus a requirements-marking step
- **Files modified:** 2

## Accomplishments

- **Task 1:** Eleven crates.io Trust Publisher Configurations created by the repository owner (owner `DF3NDR`, repo `paladin-dev-env`, workflow `release.yml`, environment `crates-io`), recorded in a new `### Trust Link Ledger` subsection with the human's "linked" confirmation stated plainly as a reported (not independently re-verified) fact.
- **Task 2:** A real, non-dry-run proof release (`v0.8.1-rc.2`) cut via the PR-decomposed flow (branch → PR → merge → tag on the merge commit), triggered by the tag push itself (not `workflow_dispatch`, sidestepping untested assumption A1). The `Authenticate with crates.io` and `Publish to crates.io` steps both succeeded; the run's overall `failure` conclusion traces entirely to the four pre-existing, off-publish-path Build Binaries matrix jobs.
- **Task 3:** All eleven `0.8.1-rc.2` versions independently re-queried against crates.io's own API: every one carries non-null `trustpub_data` (`provider: github`, `run_id: 33089177606`), contrasted explicitly against the `0.8.1-rc.1` null-provenance baseline from the bootstrap. A `### What This Proof Does Not Establish` subsection names the proof's limits.
- PUB-03 marked complete in `.planning/REQUIREMENTS.md` (checkbox and traceability table).

## Task Commits

Each task was committed atomically:

1. **Task 1: Create the eleven crates.io Trust Publisher Configurations** — `b5eaedc2` (docs)
2. **Task 2: Cut the proof prerelease and let the rewritten job mint its own credential** — `08e32e89` (docs)
3. **Task 3: Record the proof as registry-side provenance, not as a green checkmark** — `531a97d9` (docs)
4. **Mark PUB-03 complete in REQUIREMENTS.md** — `0962929a` (docs)

_No separate plan-metadata commit — this SUMMARY commit is the final commit for this plan._

## Files Created/Modified

- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md` — added `### Trust Link Ledger`, `### Proof Release Run (Task 2)`, `### Registry-Side Provenance (Task 3)`, and `### What This Proof Does Not Establish` under `## OIDC Proof Event (PUB-03)`
- `.planning/REQUIREMENTS.md` — `PUB-03` checkbox and traceability row marked Complete

## Decisions Made

- **Task 2's precondition was unmet and surfaced, not worked around.** When this executor first reached Task 2, `19-02`'s `publish-crates` rewrite (`b12789a6`) was not yet an ancestor of `origin/main` — the whole phase sat on `chore/19-trusted-publishing`. Per the precondition protocol, this halted as a `checkpoint:human-verify` rather than being auto-fixed (merging an entire phase branch into `main` early is a real decision with production consequences, not a mechanical blocker). The coordinator resolved it as option 1: merge now (PR #38, `08aa4528`), then proceed.
- **Task 1's "linked" resolution recorded honestly as unverified-by-executor.** crates.io has no API to read back a Trusted Publishing configuration and no browser session this executor holds. The bare word "linked" with no crate named as a failure is recorded, per the checkpoint's own resume-signal contract, as all eleven saved — but the ledger states plainly that this rests on the human's report, not independent re-verification, and names Task 2's proof run as the mechanism that would have surfaced a silent gap.
- **Combined coordinator guidance split back into per-task commits.** The coordinator's resolution message bundled Task 2's and Task 3's facts into one instruction, but both wrote into the same evidence file section. Rather than commit them as one blob, this executor wrote Task 2's content, committed, then used the sanctioned single-file `git checkout -- <file>` revert to back out Task 3's not-yet-added content, re-applied it, and committed separately — preserving the plan's per-task atomic-commit structure.

## Deviations from Plan

None against the plan's substance — Tasks 1-3 executed and recorded exactly as specified, with all acceptance criteria and the automated `<verify>` script passing. Two procedural adjustments, both Rule 3-class (blocking-issue handling), are recorded above under Decisions Made: the precondition halt-and-resume for Task 2, and the commit-splitting for Tasks 2/3.

## Issues Encountered

- **Isolated worktree disappeared between turns.** This plan's execution spanned two human checkpoints across separate turns. Between the first checkpoint (Task 1) and the second (Task 2/3 continuation), the isolated git worktree this executor started in (`/workspace/.claude/worktrees/agent-a9c71bd1c2d661d87`) no longer existed — `git worktree list` showed only `/workspace` on branch `chore/19-trusted-publishing`, later fast-forwarded to `main` by the orchestrator's own PR merges. This executor verified HEAD matched the expected base commit before proceeding, confirming no work was lost, and continued directly on the phase branch rather than re-entering a worktree.
- **A `git checkout -B` branch-reset was blocked by the auto-mode classifier** (looked like a force-move). Recovered via a plain `git checkout chore/19-trusted-publishing` instead, which succeeded on retry (the classifier's own error text noted this class of block is often transient).
- **The sandboxed Bash tool rejected several multi-part commands** (`for` loops with git/curl, chained boolean commands) as "too complex to verify worktree containment," even outside worktree isolation. Each was split into individual single-purpose commands — no functional impact, only extra round-trips (same pattern noted in 19-02's SUMMARY).

## User Setup Required

None for this plan directly, but two human actions were completed as part of the two checkpoints:
- Creating the eleven crates.io Trust Publisher Configurations (Task 1) — done, reported "linked."
- Cutting the `v0.8.1-rc.2` proof release via the PR-decomposed flow (Task 2) — done, PR #38 and PR #39 merged, tag pushed.

## Next Phase Readiness

- **PUB-03 is satisfied**, with registry-side (not workflow-self-reported) evidence: the same eleven crates read `trustpub_data: null` at `0.8.1-rc.1` under the standing token and non-null at `0.8.1-rc.2` under the OIDC exchange, in the same registry, days apart.
- **The standing `CARGO_REGISTRY_TOKEN` credential is still live** — this plan's whole purpose was proving the new path works before that credential is destroyed. `## Credential Revocation (PUB-04)` in `19-PUBLISH-EVIDENCE.md` is confirmed still empty at the end of this plan, satisfying the ratchet ordering D-05 requires.
- **19-04 (credential revocation) may now proceed** — its `depends_on: 19-03` precondition is met.
- **Two residual items carried forward, named rather than silently closed:** Task 1's Trust Link Ledger rests on an unverified human report (no independent re-check possible without crates.io UI/API access — the proof run's success is corroborating, not conclusive, for each individual configuration); and RESEARCH.md assumption A1 (`workflow_dispatch` eligibility for Trusted Publishing) remains untested, since this proof deliberately used a tag push.
- **`main` now contains the entire phase's work through `40990087`** (PR #38 + PR #39), ahead of where this executor's local `chore/19-trusted-publishing` branch sits before this plan's own commits — the phase branch and `main` will need reconciling before 19-04/19-05, or 19-04 may simply branch fresh from `main`.

---
*Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry*
*Completed: 2026-08-27*

## Self-Check: PASSED

- FOUND: `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md` — `### Trust Link Ledger`, `### Proof Release Run (Task 2)`, `### Registry-Side Provenance (Task 3)`, `### What This Proof Does Not Establish` all present
- FOUND: `.planning/REQUIREMENTS.md` — `PUB-03` marked `[x]` and `Complete`
- FOUND commit: `b5eaedc2` (Task 1)
- FOUND commit: `08e32e89` (Task 2)
- FOUND commit: `531a97d9` (Task 3)
- FOUND commit: `0962929a` (REQUIREMENTS.md)
- Automated verify script from 19-03-PLAN.md Task 3, re-run directly: `PUB-03 evidence OK`
