# Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-26
**Phase:** 19-crates-io-trusted-publishing-replace-the-long-lived-registry
**Areas discussed:** Crate set & paladin-herald gap, Proof event design, Environment & permission shape, Failure-honesty rewrite, Documentation & recording
**Mode:** `--auto` — all areas auto-selected; recommended option taken on every question without AskUserQuestion. Live crates.io API lookups (2026-08-26) grounded the crate-set decisions.

---

## Crate set & the paladin-herald gap (PUB-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Close the gap now | Add `paladin-herald` to the publish order; first publish during the proof window | ✓ |
| Record and defer | Document the exclusion, leave `CRATES` at ten, hand the gap to Phase 20 | |

**Auto-selected:** Close the gap — a real `cargo publish -p paladin-ai` is broken without it, and PUB-01 forbids carrying it forward unnoticed a second time.
**Notes:** Live check confirmed all ten `CRATES` entries exist on crates.io at 0.5.1; `paladin-herald` is 404 (never published). Researcher must verify whether crates.io Trusted Publishing now supports not-yet-existing crates; otherwise the first publish uses the old token inside the proof-before-revoke window.

---

## Proof event design (PUB-03)

| Option | Description | Selected |
|--------|-------------|----------|
| Real prerelease publish via release.yml | Prerelease tag or dispatch, `dry_run=false`, evidence = run URL + crates.io listing | ✓ |
| Full 0.8.0 catch-up release | Clears the 0.5.1→0.8.0 registry backlog as the proof | |
| Out-of-band manual `cargo publish` | Human publishes one crate with a personal token | |

**Auto-selected:** Real prerelease publish — proves the workflow's OIDC identity with minimal blast radius. A full catch-up release depends on Phase 20 recovery machinery that does not exist yet; a manual publish proves a human's token, not the workflow.
**Notes:** Ordering ratchet locked: trust links → proof publish → revoke at crates.io → delete repo secret. Steps 3–4 never precede step 2.

---

## Environment & permission shape (PUB-02)

| Option | Description | Selected |
|--------|-------------|----------|
| One `crates-io` environment, pinned in trust configs, tag-restricted, no reviewer gate | Unattended tag releases stay possible; tightening later is a settings change | ✓ |
| Reviewer-gated environment | Manual approval before every publish | |
| No environment pinning on crates.io side | Leave the optional environment field empty in trust configs | |

**Auto-selected:** Pinned `crates-io` environment without a reviewer gate initially. Leaving the environment field empty would let any workflow with `id-token: write` mint a publish token — pinning is the protection.
**Notes:** `id-token: write` goes on the `publish-crates` job only; `docs.yml`'s workflow-level placement is a proven mechanism but its permission placement is explicitly not copied (PUB-02 text requires job-level).

---

## Failure-honesty rewrite of `dry_run=skip` (PUB-05)

| Option | Description | Selected |
|--------|-------------|----------|
| Delete the skip branch | Mode is exactly `true|false`; auth-action failure fails the job | ✓ |
| Rewrite skip to fail | Keep the three-way mode but make `skip` exit non-zero | |

**Auto-selected:** Delete — under Trusted Publishing there is no secret whose absence could be detected, so the branch's premise disappears; a rewritten `skip` would preserve dead logic.
**Notes:** Dry-run mode skips the OIDC mint entirely (needs no credential; keeps fork dry runs working and the green-dry-run claim honest). No `continue-on-error` anywhere on the publish path.

---

## Documentation & recording (PUB-04, PUB-05)

| Option | Description | Selected |
|--------|-------------|----------|
| Trust table + credential history in `release-automation.md`, CHANGELOG entry | The doc that currently documents the token; convention (owner+date) from Phase 9/12 | ✓ |
| New standalone trust-config doc | Separate file beside the appendices | |
| Extend `SECURITY-EXCEPTIONS.md` | Reuse the advisory register file itself | |

**Auto-selected:** `release-automation.md` — it is where the next operator will look and must be rewritten anyway. `SECURITY-EXCEPTIONS.md` is mechanically checked by `scripts/check-advisory-register.sh` and scoped to RustSec advisories; adding a credential event would break that contract.

---

## Claude's Discretion

- Exact prerelease version string; prerelease tag vs `workflow_dispatch` for the proof run.
- Pilot-crate-first vs all-ten-at-once trust-link creation (either satisfies the ratchet).
- Wording/placement details inside `release-automation.md` given the fixed table columns.

## Deferred Ideas

- Full registry catch-up release (0.5.1 → current) — after Phase 20.
- Environment required-reviewer gate — revisit with Phase 20's runbook.
- `create-release@v1` replacement, index-wait fix, yank policy — Phase 20 (`PUBOPS-*`).
- Todo "Verify local make coverage reproduces CI's 82.39% figure" — reviewed (score 0.9 keyword
  match), not folded: orthogonal to the publish credential, human-owned, must not be closed by a
  phase per its own text.
