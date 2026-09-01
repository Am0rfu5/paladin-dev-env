---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
plan: 04
subsystem: infra
tags: [crates-io, trusted-publishing, oidc, github-actions, credential-revocation, secrets]

requires:
  - phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
    provides: "Registry-attested, non-dry-run proof that the OIDC path publishes all eleven crates (PUB-03, run 33089177606)"
provides:
  - "The crates.io publish-scoped token ('Paladin') revoked at the registry, by the account owner, via the crates.io UI"
  - "The CARGO_REGISTRY_TOKEN repository secret deleted from DF3NDR/paladin-dev-env, verified absent via two independent gh calls"
  - "A whole-directory sweep of .github/workflows/ proving no file references secrets.CARGO_REGISTRY_TOKEN and the one remaining bare-name occurrence is the OIDC auth step's own output, not a secret expression"
  - "PUB-04 marked complete in REQUIREMENTS.md (checkbox + traceability table)"
  - "A Revocation Ledger in 19-PUBLISH-EVIDENCE.md recording both destruction events in ratchet order, each with a date, a named actor, and what was actually observed — including fields the operator did not attest, recorded as 'not reported by operator' rather than inferred"
affects: [19-05]

tech-stack:
  added: []
  patterns:
    - "Ratchet-ordered credential destruction: the load-bearing revocation (crates.io) recorded and verified before the scriptable half (GitHub secret deletion) is attempted — a precondition on Task 3 makes the ordering structural, not just documented"
    - "Attestation-honest ledger rows: fields a human did not report are recorded as 'not reported by operator', never inferred from adjacent facts or invented to make a row look complete"
    - "Local permission-classifier block treated the same as the plan's anticipated 403 fallback: agent stops, reports, hands the one destructive step to a human, then independently re-verifies with read-only calls"

key-files:
  created:
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-04-SUMMARY.md
  modified:
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "Task 1's one-way-door decision (revoke-now) was made by the human operator (Am0rfu5, 2026-08-27) against the eleven-row Trust Link Ledger and Registry-Side Provenance table already on record from 19-03 — no new evidence was gathered, this was a comparison against what was already measured, per the plan's own framing."
  - "Task 2's revocation confirmation arrived as the bare word 'revoked' (plus a follow-up naming the token 'Paladin'), without the last-used timestamp, revocation timestamp, or other-publish-scoped-token sweep the plan's instructions requested. Per the plan's kept prohibition against recording intent as observation, these three fields are recorded as 'not reported by operator' rather than inferred or left silently absent — and T-19-21 (a second forgotten token) is stated as still open, not assumed closed."
  - "Task 3's agent-side gh secret delete and the equivalent gh api --method DELETE were both denied by the local Claude Code auto-mode permission classifier — a session Bash safety gate, not a GitHub 403/scope failure the plan anticipated. Rather than attempting a workaround (e.g. a raw curl with an extracted credential), this executor stopped and reported it, matching the plan's own designed fallback in shape: a human performs the one destructive step, the agent independently re-verifies with read-only gh api / grep calls afterward."

patterns-established:
  - "Deletion-mechanism honesty in the ledger: the Revocation Ledger's secret-deletion row names not just who deleted the secret but which mechanism was attempted first and why it failed (local classifier block), so a future reader does not mistake this for the plan's anticipated 403/scope-failure path."

requirements-completed: [PUB-04]

coverage:
  - id: D1
    description: "Task 1 one-way-door decision (revoke-now) recorded with date, actor, and the eleven-row provenance evidence it rests on, before any destruction occurred"
    requirement: "PUB-04"
    verification:
      - kind: manual_procedural
        ref: "checkpoint:decision Task 1, resolved 2026-08-27 by Am0rfu5 — 'revoke-now', no concerns named; recorded in 19-PUBLISH-EVIDENCE.md ## Credential Revocation (PUB-04) → Task 1 decision, commit 5b5866ea"
        status: pass
    human_judgment: true
    rationale: "This is an irreversible destructive decision (D-05 one-way door) that only a human with authority over the crates.io account and the repository can make; the executor's role is to present the evidence and record the decision, not to make it."
  - id: D2
    description: "crates.io publish-scoped token revoked at the registry by the account owner, and the revocation is recorded (attestation-honest — fields not reported by the operator are stated as such, not invented)"
    requirement: "PUB-04"
    verification:
      - kind: manual_procedural
        ref: "checkpoint:human-action Task 2, resolved 2026-08-27 by Am0rfu5 — 'revoked', token name 'Paladin' reported in follow-up; last-used timestamp, revocation timestamp, and other-token sweep not reported. Recorded in 19-PUBLISH-EVIDENCE.md ### Revocation Ledger row 1, commit 39574004"
        status: pass
    human_judgment: true
    rationale: "crates.io exposes no API to read back a token's existence, last-used timestamp, or revocation state (D-13) — this fact is observable only by the account owner in the UI, and this plan's must_haves author the corresponding truth as a backstop for exactly this reason. No automated verification is possible from this executor's side."
  - id: D3
    description: "GitHub repository secret CARGO_REGISTRY_TOKEN deleted and its absence independently verified via two read-only calls; whole-.github/workflows/-directory sweep confirms no remaining secret-expression reference"
    requirement: "PUB-04"
    verification:
      - kind: other
        ref: "gh api repos/DF3NDR/paladin-dev-env/actions/secrets -> total_count 0 (was 1, created/updated 2026-05-31T21:10:18Z); gh secret list --json name -> []; grep -rl 'secrets.CARGO_REGISTRY_TOKEN' .github/workflows/ -> no files; grep -c 'CARGO_REGISTRY_TOKEN' .github/workflows/release.yml -> 1 (line 411, env-var receiving steps.auth.outputs.token, not a secret expression); git diff --exit-code SECURITY-EXCEPTIONS.md -> unchanged; plan's automated <verify> script -> 'PUB-04 destruction verified'"
        status: pass
    human_judgment: false
duration: "spans two human checkpoints (one-way-door decision; crates.io UI revocation) plus one unplanned classifier-block detour (GitHub secret deletion routed to the UI) — executor-side evidence recording, verification, and commits across three resumptions: ~25min"
completed: 2026-08-27
status: complete
---

# Phase 19 Plan 04: Destroy the standing credential on both sides, in the order that makes the destruction real

**The crates.io publish-scoped token ("Paladin") is revoked at the registry and the `CARGO_REGISTRY_TOKEN` repository secret is deleted from `DF3NDR/paladin-dev-env`, in ratchet order, with a Revocation Ledger that records what was actually observed — including fields the operator did not attest, stated as "not reported by operator" rather than invented.**

## Performance

- **Duration:** ~25 min executor-side across three resumptions (two planned checkpoints plus one unplanned classifier-block detour)
- **Completed:** 2026-08-27
- **Tasks:** 3 (1 checkpoint:decision + 1 checkpoint:human-action + 1 auto), plus a requirements-marking step
- **Files modified:** 2 (plus this SUMMARY)

## Accomplishments

- **Task 1:** The one-way-door decision (D-05: a revoked crates.io token cannot be un-revoked) was confirmed by the repository/crate owner as `revoke-now`, against the eleven-row Trust Link Ledger and Registry-Side Provenance table 19-03 already recorded — a comparison against measured evidence, not fresh judgment. Recorded with date, actor, the evidence cited, and the accepted residual limits (untested `workflow_dispatch` eligibility, untested dry-run behavior under OIDC, no instant fallback after revocation).
- **Task 2:** The publish-scoped crates.io token ("Paladin") was revoked by the account owner directly in the crates.io UI — the only path, since crates.io exposes no API for this (D-13). The confirmation was a bare "revoked" plus a follow-up naming the token; three requested fields (last-used timestamp, revocation timestamp, other-token sweep) were not provided and are recorded honestly as "not reported by operator," not inferred — leaving threat T-19-21 (a second forgotten token) explicitly open rather than silently closed.
- **Task 3:** The `CARGO_REGISTRY_TOKEN` repository secret was deleted. The agent's own `gh secret delete` and the equivalent `gh api --method DELETE` were both blocked by the local Claude Code permission classifier (a session Bash safety gate, not the GitHub 403/scope failure the plan anticipated) — the deletion was routed to the human via the GitHub UI, matching the plan's designed fallback in shape. Absence was then independently verified from two read-only calls (`gh api .../actions/secrets` → `total_count: 0`; `gh secret list` → `[]`), contrasted against the pre-deletion state (`total_count: 1`, created/updated `2026-05-31T21:10:18Z`). A sweep of the entire `.github/workflows/` directory (all seven workflow files, not just `release.yml`) confirms zero `secrets.CARGO_REGISTRY_TOKEN` references and exactly one remaining bare-name occurrence — the OIDC auth step's own output variable, not a secret expression.
- PUB-04 marked complete in `.planning/REQUIREMENTS.md` (checkbox and traceability row).
- Plan-level verification re-run and passing: the automated `<verify>` script (`PUB-04 destruction verified`), `bash scripts/check-advisory-register.sh` (exit 0, unaffected by this plan), `git diff --exit-code SECURITY-EXCEPTIONS.md` (byte-unchanged, D-12 upheld), and `pre-commit run --all-files` (all hooks green).

## Task Commits

Each task was committed atomically:

1. **Task 1: Confirm the one-way door** — `5b5866ea` (docs)
2. **Task 2: Revoke the token at crates.io** — `39574004` (docs)
3. **Task 3: Delete the repository secret and sweep** — `bb818e88` (docs)
4. **Mark PUB-04 complete in REQUIREMENTS.md** — `1a1b8cd8` (docs)

_No separate plan-metadata commit — this SUMMARY commit is the final commit for this plan._

## Files Created/Modified

- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md` — filled `## Credential Revocation (PUB-04)`: Task 1 decision record, `### Revocation Ledger` (three rows: crates.io revocation, secret deletion, sweep), and the closing paragraph stating what the phase does and does not claim
- `.planning/REQUIREMENTS.md` — `PUB-04` checkbox and traceability row marked Complete
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-04-SUMMARY.md` — this file

## Decisions Made

- **Task 1's decision rests entirely on evidence already on record from 19-03** — no new measurement was taken; the human compared the recorded eleven-row Trust Link Ledger and Registry-Side Provenance table against the plan's own stated criteria and chose `revoke-now`.
- **Task 2's ledger row is attestation-honest by construction.** The operator's confirmation was minimal ("revoked", then "Paladin" for the token name). Per this plan's kept prohibition (recording intent as observation is forbidden), the three unreported fields are stated as "not reported by operator" — not filled in from assumption, not left as blank cells that could be misread as omitted-by-accident. T-19-21 stays open in the record rather than being quietly treated as resolved.
- **Task 3's deletion mechanism is recorded honestly, including the failed agent-side attempt.** The plan anticipated a GitHub 403 (fine-grained PAT lacking secrets-write scope) as the reason a human might need to delete the secret manually. What actually happened was different in kind — the local Claude Code auto-mode permission classifier denied both `gh secret delete` and the equivalent `gh api --method DELETE` call before any GitHub-side response was even reached. Rather than treating this as equivalent to the anticipated 403 and glossing over the distinction, the ledger names the actual mechanism (classifier block) and states that the agent did not attempt to route around it (e.g., via a raw `curl` with an extracted credential) before handing the step to the human.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Local permission classifier blocked the planned `gh secret delete` / `gh api DELETE` calls**
- **Found during:** Task 3 (Delete the repository secret)
- **Issue:** Both `gh secret delete CARGO_REGISTRY_TOKEN --repo DF3NDR/paladin-dev-env` and the equivalent `gh api --method DELETE repos/DF3NDR/paladin-dev-env/actions/secrets/CARGO_REGISTRY_TOKEN` were denied by the Claude Code auto-mode classifier — a local Bash safety gate distinct from the GitHub-side 403 the plan's instructions anticipated.
- **Fix:** Did not attempt a workaround. Stopped, reported the exact failure to the coordinator/human, and had the human delete the secret manually via the GitHub web UI — the same practical outcome the plan's own 403 fallback specifies, reached via a different trigger. Independently re-verified absence afterward using two read-only `gh` calls (not subject to the same classifier block).
- **Files modified:** none (procedural; captured only as a ledger entry in `19-PUBLISH-EVIDENCE.md`)
- **Verification:** `gh api repos/DF3NDR/paladin-dev-env/actions/secrets` → `total_count: 0`; `gh secret list --json name` → `[]`
- **Committed in:** `bb818e88` (Task 3 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking, routed to human per plan's own fallback shape)
**Impact on plan:** No scope creep — the deletion still happened, just via the UI instead of the CLI, and the ledger names the mechanism honestly rather than implying the CLI path succeeded.

## Issues Encountered

- **Isolated worktree disappeared between turns**, same phenomenon 19-03's SUMMARY documented. Between the initial checkpoint and the first resumption, `/workspace/.claude/worktrees/agent-afd18d1017121d476` no longer existed — `git worktree list` showed only `/workspace` on branch `chore/19-trusted-publishing`. This executor verified HEAD at `/workspace` matched the expected base commit (`420aed57`) exactly, confirmed a clean working tree, and continued directly on the phase branch (not a protected branch) rather than re-entering a worktree. No work was lost; this was re-verified defensively before every subsequent commit in this plan.
- **The Bash cwd itself drifted mid-session** (from the worktree path back to `/workspace`) even before the worktree's disappearance was confirmed — caught by the cwd-drift assertion in the executor's commit protocol, which prompted the direct worktree-existence check that surfaced the issue above.

## User Setup Required

Two human actions were completed as part of this plan's checkpoints:
- Confirming the one-way-door decision (`revoke-now`) — Task 1, done.
- Revoking the crates.io publish-scoped token via the crates.io UI — Task 2, done.
- Deleting the `CARGO_REGISTRY_TOKEN` repository secret via the GitHub UI, after the agent-side CLI path was blocked by the local permission classifier — Task 3, done.

None remain for this plan.

## Next Phase Readiness

- **PUB-04 is satisfied**: both halves of the standing credential are destroyed, in ratchet order, each recorded with a date and a named actor, and the record states what was actually observed rather than what was intended — including the fields the operator did not report.
- **The repository now has exactly one publish path**: the OIDC exchange via `rust-lang/crates-io-auth-action@v1`, minting a per-run credential from the eleven Trust Publisher Configurations. No repository secret and no workflow reference to one remain.
- **Residual, explicitly not claimed by this plan:** whether any *other* publish-scoped token exists on the crates.io account was requested of the operator and not attested either way — T-19-21 stays open, not silently assumed answered. Any future audit of the account's token list would close this gap; nothing in this phase's evidence claims it is closed.
- **19-05 (documentation) may now proceed** — its dependency on 19-04's completed credential-history record is met. `## Credential Revocation (PUB-04)` in `19-PUBLISH-EVIDENCE.md` is the working record 19-05 copies from into `docs/src/appendix/release-automation.md`'s `### Credential History` subsection.
- **`main` remains ahead of `chore/19-trusted-publishing`** at this plan's start (per 19-03's readiness note); this plan's four commits (`5b5866ea`, `39574004`, `bb818e88`, `1a1b8cd8`) landed directly on `chore/19-trusted-publishing`, continuing from the exact commit (`420aed57`) the phase branch was at before this plan began. No push was performed by this executor.

---
*Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry*
*Completed: 2026-08-27*

## Self-Check: PASSED

- FOUND: `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-04-SUMMARY.md`
- FOUND commit: `5b5866ea` (Task 1)
- FOUND commit: `39574004` (Task 2)
- FOUND commit: `bb818e88` (Task 3)
- FOUND commit: `1a1b8cd8` (REQUIREMENTS.md)
- Plan's automated verify script, re-run directly: `PUB-04 destruction verified`
- `bash scripts/check-advisory-register.sh`: exit 0
- `git diff --exit-code SECURITY-EXCEPTIONS.md`: unchanged
- `pre-commit run --all-files`: all hooks Passed
