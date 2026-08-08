# ADR-0026: crates.io package-name collision guard

## Status

Accepted

**Date:** 2026-08-08

## Context

`.project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md`
closed Task 5.5's dry-run blocker with one residue left explicit in its own "Follow-up" section:
"Keep CI/package guardrails that detect crates.io package-name collisions early." That residue was
never picked up. It exists because the collision it warns about already happened: this project
renamed `paladin-core` to package name `paladin-ai-core` and its root package to `paladin-ai`, a
full NO-GO cycle, because the shorter names were unavailable on crates.io. That history is why every
crate in this workspace has a `[package].name` distinct from its `[lib].name` today — for example
`crates/paladin-core/Cargo.toml:2` declares package name `paladin-ai-core` while `Cargo.toml:15` in
the same file declares `[lib] name = "paladin_core"`, and the root manifest's `[package].name` at
`Cargo.toml:33` is `paladin-ai` while `[lib].name` at `Cargo.toml:46` is `paladin`.

The earliest guard against a repeat of that collision sits in two places today, both late and
neither name-specific:

- `.github/workflows/ci.yml:911-915` — the `publish-dry-run` job (display name `Publish Dry Run`)
  runs a single workspace-wide `cargo publish --dry-run` and is gated
  `if: github.event_name == 'push' && github.ref == 'refs/heads/main'` — main-branch only, never on
  a pull request.
- `.github/workflows/release.yml:355-429` — the `publish-crates` job (display name
  `Publish to crates.io`) runs `cargo publish --dry-run -p "$crate"` per crate at `:410`, inside the
  release job itself, the latest point at which a name collision could be discovered.

Neither of these checks the package name itself against anything; both simply attempt the operation
and let Cargo's own registry interaction surface a failure, at main-branch or release time rather
than on the pull request that introduces the new name.

## Decision

The project adopts an offline allow-list guard that runs on every pull request. `.crate-names.txt`
is the authoritative owned-name list: a committed, hand-edited file enumerating the eleven package
names this project already owns on crates.io. `scripts/check-crate-names.sh` parses every workspace
manifest's `[package]` table with `tomllib`, excludes any crate whose manifest sets
`publish = false`, and asserts **set equality in both directions** between the tree's publishable
package names and the allow-list — an unlisted tree name and a stale allow-list entry both fail,
each with its own message. The guard is wired into `make check-crate-names` and into
`.github/workflows/ci.yml`'s `cargo-deny` job (display name `License & Dependency Policy`), one of
the three required status-check contexts in `.github/rulesets/protect-main-branch.json`.

Adding a crate is therefore a deliberate one-line addition to `.crate-names.txt`, and the release
runbook requires a human to confirm the new name's availability on crates.io before that line is
added — CI enforces that the addition was made, not that the name is actually free. **Residual
cost, stated explicitly:** the eleven existing names are already owned, so their collision risk is
zero; but a genuinely novel name is still checked by a human against crates.io, not by CI. That gap
is accepted, not hidden.

## Considered Options

- **A live crates.io sparse-index availability query** (rejected) — `crates.io` returns HTTP 403 in
  this environment, so a network check could be written but never demonstrated passing or failing,
  which is the exact Phase 8 D-03 failure mode: a claim of closure with no command that proves it.
  It would also be flaky in CI even where it did work, since it depends on a third party's
  availability and rate limits, and it does not answer "does this workspace name match what we
  intend to own" — only "does this string happen to be free right now."
- **Pure acceptance of dry-run reliance**, SEC-03's second permitted answer (rejected) — the
  existing guards are real, but the earliest of them fires on a main-branch push
  (`ci.yml:911-915`) and the other fires inside the release job itself
  (`release.yml:355-429`). Roughly twenty lines of guard buys detection on every pull request
  instead of at main-branch or release time, for the one failure mode this project has already paid
  for once.
- **A one-directional guard checking only that allow-listed names still exist** (rejected) — this
  passes vacuously against exactly the failure the guard exists to catch: a *new* colliding name
  added to the tree would never be examined, because the check only looks backward from the
  allow-list toward the tree, never forward from the tree toward the allow-list. The guard would
  report green while never having examined the risk.
- **The offline bidirectional allow-list guard** (accepted) — provable in this environment, not
  flaky, and it fails in exactly the two ways an integrity check on a hand-maintained list needs to
  fail: a new unlisted name, and a stale entry with nothing behind it.

## Code Locations

- `.crate-names.txt` — the committed, hand-edited allow-list of eleven owned package names.
- `scripts/check-crate-names.sh` — the bidirectional set-equality guard.
- `Makefile:163-165` — the `check-crate-names` target, placed immediately after `check-changelogs`.
- `.github/workflows/ci.yml:81-82` — the `cargo-deny` job (`License & Dependency Policy`).
- `.github/workflows/ci.yml:97-98` — the `Check crates.io package names` step, immediately after the
  `Check per-crate changelogs` step plan 09-01 added at `:94-95`.
- `.github/workflows/ci.yml:911-915` — the pre-existing `publish-dry-run` job, main-branch-only,
  unchanged by this decision.
- `.github/workflows/release.yml:355-429` — the pre-existing per-crate dry run inside
  `publish-crates`, `cargo publish --dry-run -p "$crate"` at `:410`, unchanged by this decision.

Verbatim negative-path transcript proving the guard fails (full sweep of six failure modes recorded
in `.planning/phases/09-release-security-gate-integrity/09-04-SUMMARY.md`; one representative mode
reproduced here per D-19's evidence bar):

```
$ mkdir -p crates/scratch-unlisted && cat > crates/scratch-unlisted/Cargo.toml <<'EOF'
[package]
name = "paladin-scratch-unlisted"
version = "0.1.0"
edition = "2024"
EOF
$ ./scripts/check-crate-names.sh; echo "EXIT=$?"
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 1 tree package name(s) not on the allow-list:
  - paladin-scratch-unlisted
...
EXIT=1
```

`rm -rf crates/scratch-unlisted` restored the tree; `./scripts/check-crate-names.sh` returned to
exit 0 immediately after.

## Code Conformance

must change

Phase 9 / plan 09-04, Task 1 (`.crate-names.txt` and `scripts/check-crate-names.sh`) and Task 2
(the `Makefile` and `ci.yml` wiring) are the executors of this decision; both are complete as of
this ADR.

## Downstream Consumers

- The release runbook gains a manual step: before adding any new crate's name to
  `.crate-names.txt`, a human confirms the name is available on crates.io. CI enforces that the
  addition happened and matches the tree; it does not and cannot enforce that the name was actually
  free at the moment of the check — that is the residual cost this design accepts, stated here in
  the same terms as in `## Decision`: the eleven existing names carry zero collision risk because
  they are already owned, and only a genuinely novel name still depends on a human step rather than
  a CI check.
- Phase 10 / HARD-01's ledger row for the publish-verification closure — this ADR is the "CI/package
  guardrails" follow-up the deferred Epic 4 document asked for, and HARD-01 should record this
  decision as satisfying it.
