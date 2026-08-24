---
phase: 15-coverage-ci-quality-gates
plan: 05
subsystem: testing
tags: [rust, test-doubles, notifications, event-listener, tdd-infrastructure]

requires:
  - phase: 15-coverage-ci-quality-gates (waves 1-4, PIPE plans 15-01..15-04)
    provides: "the gates that validate subsequent coverage work exist before the coverage work they measure begins (D-11 sequential PIPE-then-DEFER boundary)"
provides:
  - "src/test_support/, a #[cfg(test)]-gated module (declaration-site gate in src/lib.rs) that coexists with tests/helpers/ without overlap"
  - "FailingChannelHandler: a NotificationChannelHandler double whose handle_notification always errors, registrable on a real NotificationService via register_channel_handler with no production signature change"
  - "event_factory: build_event, build_non_matching_event, build_event_batch -- deterministic, clock-free, random-free Event construction including a 1000-plus-event bulk constructor"
affects: [15-06, 15-07, 15-08, 15-09, 15-10]

tech-stack:
  added: []
  patterns:
    - "src/test_support/ as the src/-side twin of tests/helpers/: same Arc<Mutex<..>> recording shape, disjoint placement, #[cfg(test)]-gated at the module declaration so it never reaches a release build"
    - "Deterministic test-double identity via a fixed-seed std::collections::hash_map::DefaultHasher over caller arguments, rather than Uuid::new_v5 (uuid crate's 'v5' feature is not enabled in this workspace and this plan may not touch any manifest)"
    - "Poisoned-lock recovery via .lock().unwrap_or_else(|poisoned| poisoned.into_inner()) instead of .lock().unwrap() everywhere a Mutex is read in test-support code"

key-files:
  created:
    - src/test_support/mod.rs
    - src/test_support/failing_channel_handler.rs
    - src/test_support/event_factory.rs
  modified:
    - src/lib.rs

key-decisions:
  - "D-09/D-10 honored: no mockall dependency added; the mock set is demand-driven (only FailingChannelHandler and event_factory, exactly what plans 15-06..15-09 are known to need) rather than pre-building MockUserRepository/MockLogPort/MockNotificationService/MockEventSource/MockTriggerExecutor speculatively"
  - "FailingChannelHandler::new() hardcodes NotificationChannel::Email rather than taking a channel parameter, because UserService::send_welcome_notification (the DEFER-02 consumer) dispatches through Email specifically -- confirmed by direct read of src/core/platform/manager/user_service.rs:148, not assumed"
  - "Deterministic UUIDs use a fixed-seed DefaultHasher-derived 16-byte array via Uuid::from_bytes, not Uuid::new_v5 -- the uuid crate's 'v5' feature is not enabled anywhere in the 12-manifest workspace, and this plan's acceptance criteria forbid touching any Cargo.toml to add it"
  - "build_non_matching_event returns Result<Event, EventFactoryError> rather than an infallible Event, because a condition whose event_type_pattern is exactly \"*\" (or otherwise matches the constructor's own sentinel type) has no non-matching event type to construct -- caught by re-checking the sentinel against a local matches_wildcard mirror of Trigger::matches_pattern rather than assumed safe"

patterns-established:
  - "Any future src/test_support/ double follows FailingChannelHandler's shape: #[derive(Clone)] over Arc<Mutex<..>> interior state, .lock().unwrap_or_else(poison-recovery), zero .unwrap()/.expect()/panic!, and rustdoc examples fenced ```ignore (cfg(test)-gated items are invisible to a normal cargo doc/doctest pass, so a live doctest against them would fail to resolve the module)"

requirements-completed: [DEFER-01]

coverage:
  - id: D1
    description: "src/test_support/ module exists, #[cfg(test)]-gated at the declaration site in src/lib.rs, importable by co-located src/ test modules, and produces no artifact under target/release"
    requirement: DEFER-01
    verification:
      - kind: unit
        ref: "python3 regex check against src/lib.rs confirming #[cfg(test)] gates `pub mod test_support;` at the declaration site"
        status: pass
      - kind: other
        ref: "cargo build --release --lib (CARGO_TARGET_DIR=/workspace/target) -- exit 0, and `test ! -e target/release/build/test_support` after"
        status: pass
    human_judgment: false
  - id: D2
    description: "FailingChannelHandler implements the real NotificationChannelHandler trait (channel/can_handle/handle_notification/health_check), is Send + Sync, records invocations in call order by domain identity, and exposes new/with_error/invocations/call_count"
    requirement: DEFER-01
    verification:
      - kind: unit
        ref: "src/test_support/failing_channel_handler.rs#tests (7 tests: new_defaults_to_the_email_channel_with_no_invocations, can_handle_matches_only_the_claimed_channel, handle_notification_always_errors_and_records_by_domain_identity, handle_notification_preserves_call_order_across_two_calls, with_error_lets_a_test_choose_the_error_variant, health_check_reports_healthy, clones_share_recorded_invocations)"
        status: pass
    human_judgment: false
  - id: D3
    description: "event_factory provides deterministic single-event, non-matching-event, and 1000-plus-event bulk construction with no clock and no randomness"
    requirement: DEFER-01
    verification:
      - kind: unit
        ref: "src/test_support/event_factory.rs#tests (8 tests including build_event_batch_is_deterministic and build_event_batch_produces_distinct_identities_for_a_thousand_events)"
        status: pass
    human_judgment: false
  - id: D4
    description: "No new dependency added to any manifest across the 12-crate workspace; mockall not adopted; tests/helpers/ untouched"
    requirement: DEFER-01
    verification:
      - kind: other
        ref: "git diff --stat Cargo.toml crates/*/Cargo.toml (empty); grep -rc mockall across all 12 manifests (all 0); git status --short shows no tests/helpers/ changes"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 05: Shared test_support module (DEFER-01 opening plan) Summary

**`src/test_support/`, `#[cfg(test)]`-gated at the declaration site, ships `FailingChannelHandler` (a real-trait notification-failure double) and `event_factory` (deterministic, clock-free `Event` construction including a 1000-event bulk builder) -- zero new dependencies, `tests/helpers/` untouched.**

## Performance

- **Duration:** ~55 min
- **Tasks:** 2 completed
- **Files modified:** 4 (3 created, 1 modified)

## Accomplishments

- Built `src/test_support/`, the `src/`-side twin to `tests/helpers/` that the register's `tests/common/` placement could never have served (co-located `#[cfg(test)] mod tests` blocks in `src/` cannot import from the separate `tests/` crate). Gated `#[cfg(test)]` on the module *declaration* in `src/lib.rs`, verified against a `cargo build --release --lib` pass and the plan's own regex check, so nothing here reaches a release artifact.
- `FailingChannelHandler`: implements the real `NotificationChannelHandler` trait read directly from `src/application/services/notification_orchestrator/types.rs` at execution time (not from any planning document's quotation). Registers on a real `NotificationService` through the existing public `register_channel_handler` seam -- `UserService.notification_service` is a concrete `Arc<NotificationService>`, so no trait substitution was possible, and none was attempted. The production branch it exercises (`register_user`'s `if let Err(..)` handling in `send_welcome_notification`) is unchanged.
- `event_factory`: `build_event`, `build_non_matching_event`, `build_event_batch` -- all deterministic (identity derived from a fixed-seed `DefaultHasher`, not `Uuid::new_v5`, since the `uuid` crate's `v5` feature is off everywhere in this workspace and this plan touches no manifest). `build_event_batch` is the single call DEFER-03's 1000-plus-event burst needs instead of a hand-copied loop.
- 15 new unit tests (7 + 8) prove both doubles work before any consumer plan exists, including an explicit determinism assertion and a 1000-element distinct-identity assertion for the bulk constructor.

## Task Commits

1. **Task 1: The test_support module and FailingChannelHandler** - `b89b263` (feat)
2. **Task 2: Deterministic event construction for the listener suite** - `a03b043` (feat)

_No TDD tasks in this plan; both are `type="auto"`._

## Files Created/Modified

- `src/test_support/mod.rs` - barrel module: module doc explaining the `src/` vs `tests/` placement rationale, `pub mod`/`pub use` for both submodules
- `src/test_support/failing_channel_handler.rs` - `FailingChannelHandler`, `FailingChannelInvocation`, and 7 unit tests
- `src/test_support/event_factory.rs` - `build_event`, `build_non_matching_event`, `build_event_batch`, `EventFactoryError`, and 8 unit tests
- `src/lib.rs` - added `#[cfg(test)] pub mod test_support;` alongside the existing top-level module declarations

## Decisions Made

- **FailingChannelHandler's channel is hardcoded to `NotificationChannel::Email`, not parameterized.** The plan lists exactly four required public functions (`new`, `with_error`, `invocations`, `call_count`); adding a channel parameter would mean either a fifth function or overloading `new`, and the one real consumer named in this plan (DEFER-02's welcome-notification test) dispatches through `Email` specifically (confirmed by reading `user_service.rs:148` directly, not assumed from the plan text).
- **`Uuid::new_v5` was considered and rejected** in favor of a `DefaultHasher`-derived byte array fed to `Uuid::from_bytes`. `new_v5` requires the `uuid` crate's `"v5"` feature, which is enabled in none of this workspace's 12 manifests, and this plan's own acceptance criteria forbid any manifest diff. `from_bytes` is unconditional in the `uuid` crate (verified by reading the vendored `uuid-1.23.1` source), so this path adds no feature-flag risk.
- **`build_non_matching_event` returns `Result`, not an infallible `Event`.** A `TriggerCondition` whose `event_type_pattern` is `"*"` matches every event type, so no non-matching type can be constructed; the function detects this by checking its own sentinel type against a local `matches_wildcard` (a documented, manually-synced mirror of `Trigger::matches_pattern`, which is private) and returns `EventFactoryError::UnavoidableMatch` rather than silently producing a matching "non-matching" event.
- **No internal doctest execution.** Every rustdoc example is fenced ```` ```ignore ```` rather than left as a live doctest, because `src/test_support/` is declared `#[cfg(test)]` at the module level and is therefore invisible to a normal `cargo doc`/doctest pass (doctests compile against the built rlib, not the source, and cfg(test) items are absent from a non-test build). A live doctest referencing `paladin::test_support::...` would fail to resolve. This mirrors the existing workspace convention (`tests/helpers/mock_arsenal_adapter.rs`, `crates/paladin-core/src/platform/container/herald.rs`, and others already use ```` ```rust,ignore ````/```` ```ignore ```` for the same reason).

## Deviations from Plan

None — plan executed exactly as written. The two corrections DEFER-01's register already required (`tests/common/` structurally cannot serve `src/`-side co-located tests; `MockNotificationService` is not constructible against `UserService`'s concrete `Arc<NotificationService>`) were the plan's own stated premise, not a deviation discovered during execution.

One implementation detail diverged from what a literal reading of the plan's `read_first` notes might suggest, and is recorded above under Decisions Made rather than as a deviation, since it does not change any acceptance criterion: `Uuid::new_v5` (implied by "deterministic ... identities") was replaced with a `DefaultHasher`-based equivalent after confirming the `v5` uuid feature is unavailable without a manifest edit, which this plan is expressly forbidden from making.

## Issues Encountered

- **`RUSTDOCFLAGS="--cfg test" cargo doc --no-deps` fails on this crate for reasons unrelated to this plan.** Pre-existing `#[cfg(test)]` modules in `src/config/{citadel,env_utils,file_storage,herald}.rs` import `serial_test`, a dev-dependency not linked during a `cargo doc` invocation. This is out of scope per the plan's scope boundary (pre-existing, unrelated files) and was not touched. Missing-docs coverage for the two new files in this plan was instead confirmed by inspecting `cargo build --lib --tests` output for `missing_docs` warnings (`#![warn(missing_docs)]` is crate-level and active under the `test` cfg) -- none were emitted, and both files were also visually reviewed to confirm every `pub` item carries a `///` doc comment.
- **No Snyk CLI or MCP tool was available in this execution environment** (`command -v snyk` found nothing, and no Snyk MCP tool was present in the available toolset), so the `snyk_rules.instructions.md`-mandated code scan on the two new first-party files could not be run in this session. Flagged here rather than silently skipped; a scan should be run against `src/test_support/failing_channel_handler.rs` and `src/test_support/event_factory.rs` before this work is considered fully closed out per project security policy.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

`src/test_support/` is ready for plans 15-06 through 15-09 to consume `FailingChannelHandler` and `event_factory`. Per the plan's own scoping, the per-name verdict for all five DEFER-01 register names (`MockUserRepository`, `MockLogPort`, `MockNotificationService`, `MockEventSource`, `MockTriggerExecutor`) is deliberately deferred to plan 15-10, after the consuming plans establish which doubles were actually needed. No blockers for wave 6+.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*

## Self-Check: PASSED

- FOUND: `src/test_support/mod.rs`
- FOUND: `src/test_support/failing_channel_handler.rs`
- FOUND: `src/test_support/event_factory.rs`
- FOUND: `.planning/phases/15-coverage-ci-quality-gates/15-05-SUMMARY.md`
- FOUND commit `b89b263` (Task 1)
- FOUND commit `a03b043` (Task 2)
