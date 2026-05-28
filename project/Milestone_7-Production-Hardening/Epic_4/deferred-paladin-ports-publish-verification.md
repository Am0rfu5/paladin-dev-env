# Deferred Item: paladin-ports Publish Verification Blocker

Date: 2026-05-28
Epic: Milestone 7, Epic 4
Status: Deferred

## Deferred Scope

Task reference: 5.5 in the Epic 4 task list.

Deferred item:
- `cargo publish --dry-run -p paladin-ports` verification failure.

## Current Failure

Observed during dry-run verification:
- `cargo publish --dry-run --allow-dirty -p paladin-ports --manifest-path /workspace/Cargo.toml` fails before verify with:
  - `no matching package named 'paladin-ai-core' found`
  - `location searched: crates.io index`
  - `required by package 'paladin-ports v0.1.0'`

Resolved root cause that led to prior verify-time compile mismatch:
- `paladin-ports` was previously binding to an unrelated crates.io `paladin-core` package.
- Workspace manifests now remap internal dependencies to package `paladin-ai-core`.
- `cargo publish --dry-run --allow-dirty -p paladin-ai-core --manifest-path /workspace/Cargo.toml` passes.

Downstream impact:
- Public crates depending on `paladin-ports` cannot complete dry-run publish verification.

## Why Deferred

Epic 4 focus is release readiness documentation and audit closure.
`paladin-ports` dry-run verification now depends on crates.io availability of `paladin-ai-core`, which requires executing the publish order documented in Task 2.5.

## Deferral Exit Criteria

1. Publish `paladin-ai-core` to crates.io (non-dry-run).
2. Re-run `cargo publish --dry-run -p paladin-ports` and confirm verification passes.
3. Re-run dry-run for all public crates in dependency order and capture successful evidence.
4. Update Epic 4 Task 5.5 from deferred to complete.

## Proposed Follow-up

- Execute a release-sequenced publish run: `paladin-ai-core` first, then `paladin-ports`, then dependents.
- Add a CI guard that validates internal dependency package-name collisions against crates.io before release.
