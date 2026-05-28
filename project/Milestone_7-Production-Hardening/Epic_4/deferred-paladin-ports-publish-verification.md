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
- `E0433`: unresolved `paladin_core` module/crate references in `src/output/queue_port.rs`.
- Representative symbols:
  - `paladin_core::base::entity::message::MessagePriority`
  - `paladin_core::base::entity::message::Message::new`
  - `paladin_core::base::entity::message::Message::with_priority`

Downstream impact:
- Public crates depending on `paladin-ports` cannot complete dry-run publish verification.

## Why Deferred

Epic 4 focus is release readiness documentation and audit closure.
The unresolved verify-time compile mismatch for `paladin-ports` requires targeted dependency/path investigation and likely source adjustments that are out-of-scope for the current partial-progress docs checkpoint.

## Deferral Exit Criteria

1. Reproduce failure in a packaged-tarball verification context.
2. Implement a fix so `paladin-ports` verifies under `cargo publish --dry-run -p paladin-ports`.
3. Re-run dry-run for all public crates in dependency order and capture successful evidence.
4. Update Epic 4 Task 5.5 from deferred to complete.

## Proposed Follow-up

- Open a focused follow-up work item under Epic 4 or Epic 5 for publish verification hardening.
- Add a CI guard that runs `cargo package --allow-dirty` plus verification for `paladin-ports` to prevent regressions.
