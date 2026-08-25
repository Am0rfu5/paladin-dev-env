---
phase: 15-coverage-ci-quality-gates
plan: 06
subsystem: testing
tags: [rust, tokio-test, argon2, notification, coverage, defer-02]

# Dependency graph
requires:
  - phase: 15-coverage-ci-quality-gates
    provides: "src/test_support/ shared test doubles (FailingChannelHandler, event_factory) built by plan 15-05"
provides:
  - "14 new #[tokio::test]s in the existing co-located test module of src/core/platform/manager/user_service.rs covering registration, validation, duplicate detection, idempotency, argon2 hashing, and notification-dispatch-failure tolerance"
  - "RecordingLogPort — a lightweight local LogPort test double proving a Warn-level log entry is actually written, not merely inferred"
  - "build_service_with_failing_notifications fixture wiring a FailingChannelHandler + PassthroughTemplateProcessor + cached template onto a real NotificationService before injection"
affects: [15-10]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "LogPort is Arc<dyn LogPort>, so a narrowly-scoped local test double can substitute for it directly (no mocking framework), unlike the concrete Arc<NotificationService> which needs the register_channel_handler seam instead"
    - "NotificationService::send_notification resolves the notification's template before dispatching to a channel handler; a channel-handler-failure test must cache a template and register a template processor first, or the failure happens earlier (at template resolution) and the channel handler is never invoked"

key-files:
  created: []
  modified:
    - src/core/platform/manager/user_service.rs

key-decisions:
  - "Split the file's single continuous edit into two atomic commits along task boundaries (Task 1 needed zero new imports; Task 2's imports and RecordingLogPort/fixture were isolated into the second commit) by reconstructing an intermediate file state, rather than committing the whole diff in one task-spanning commit."
  - "Added a local RecordingLogPort test double (not part of src/test_support/, not in the plan's Artifacts table) because LogPort's stats API (SystemLogAdapter::get_stats) never populates entries_by_level, so there was no way to assert a Warn entry was actually written without either a process-global log-crate capture (risky in a shared multi-hundred-test binary) or a local port substitute. The port field is already Arc<dyn LogPort>, so this is a sanctioned seam, not a new dependency."
  - "Wired a cached "user_welcome" NotificationTemplate and a minimal PassthroughTemplateProcessor into the Task 2 fixture after discovering that NotificationService::send_notification's process_template step runs BEFORE channel-handler dispatch: without a cached template, notification delivery already fails at template resolution (ConfigurationError: No template processor configured) even in the five pre-existing tests, and the registered FailingChannelHandler would never be invoked, making the call_count() > 0 assertion always false regardless of the channel-handler seam."

requirements-completed: [DEFER-02]

coverage:
  - id: D1
    description: "Registration happy path persists an argon2 PHC-format hash that verifies only against the original password"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_persists_an_argon2_hash_that_only_verifies_the_original_password"
        status: pass
    human_judgment: false
  - id: D2
    description: "Byte-identical duplicate email rejected with EmailAlreadyExists; observed verdict for a case-variant username recorded"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_a_byte_identical_duplicate_email"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_accepts_a_case_variant_username_because_the_duplicate_check_is_on_email"
        status: pass
    human_judgment: false
  - id: D3
    description: "Double registration with the same request leaves exactly one persisted user"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_called_twice_with_the_same_request_leaves_exactly_one_user_persisted"
        status: pass
    human_judgment: false
  - id: D4
    description: "Empty username, whitespace-only username, empty email, and empty password each produce a specific UserError variant"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_empty_username"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_a_whitespace_only_username"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_empty_email"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_empty_password"
        status: pass
    human_judgment: false
  - id: D5
    description: "A multi-byte Unicode username is validated against the observed rule (byte length via str::len(), not char count)"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_accepts_a_multi_byte_unicode_username_within_the_byte_length_rule"
        status: pass
    human_judgment: false
  - id: D6
    description: "Three distinct invalid-email shapes (missing @, missing local part, missing domain) each rejected with InvalidEmail"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_email_missing_the_at_symbol"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_email_missing_the_local_part"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#register_user_rejects_an_email_missing_the_domain_part"
        status: pass
    human_judgment: false
  - id: D7
    description: "A notification channel that always fails does not block registration: write commits, the failure path is genuinely taken, and a warning is logged"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#notification_failure_does_not_block_registration"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#registration_succeeds_when_no_failing_handler_is_registered"
        status: pass
    human_judgment: false

duration: ~45min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 06: User service registration, validation and notification-failure coverage Summary

**14 new characterization tests for `user_service.rs`'s registration/validation/argon2-hashing path plus its notification-dispatch-failure tolerance, using a local `RecordingLogPort` double and a `FailingChannelHandler` wired through the public `register_channel_handler` seam — production code untouched (byte-identical above `#[cfg(test)]`).**

## Performance

- **Duration:** ~45 min
- **Tasks:** 2
- **Files modified:** 1 (`src/core/platform/manager/user_service.rs`, test module only)

## Accomplishments

- Registration happy path, argon2 hash persistence/verification, duplicate-email rejection, double-registration idempotency, empty/whitespace-input rejection, a Unicode-username byte-length-rule pin, and three invalid-email shapes — all as discriminating `#[tokio::test]`s reusing the existing `build_service`/`registration` fixtures against the real `sqlite::memory:` repository.
- Proved (rather than assumed) that a notification-channel failure does not block registration: added a `RecordingLogPort` test double and a `build_service_with_failing_notifications` fixture that registers a `FailingChannelHandler` on a real `NotificationService` via the public `register_channel_handler` seam, then asserted `register_user` returns `Ok`, the user is retrievable afterward, the failing handler's invocation count is `> 0`, and a `Warn`-level "Failed to send welcome notification" entry was actually written.
- Discovered and worked around a template-resolution gate in `NotificationService::send_notification` that would otherwise make the failing-channel-handler path unreachable (see Decisions Made).

## Task Commits

Each task was committed atomically:

1. **Task 1: Registration, validation and password-hashing coverage** - `a249f68` (test)
2. **Task 2: Notification failure must not block registration** - `298d6c3` (test)

**Plan metadata:** (this commit)

## Files Created/Modified
- `src/core/platform/manager/user_service.rs` - Extended the existing `#[cfg(test)] mod tests` block with 14 new tests, a `RecordingLogPort` double, a `PassthroughTemplateProcessor` double, and a `build_service_with_failing_notifications` fixture. Nothing above the `#[cfg(test)]` marker changed (verified byte-identical against `git show HEAD:` after every task).

## Decisions Made

- **Split into two atomic commits along task boundaries.** Both tasks' code was written in one editing pass since they share the same file, but Task 1's tests needed zero new imports, so the diff cleanly separated: Task 1 = only the appended test functions (no import hunk); Task 2 = the import hunk + `RecordingLogPort` + `PassthroughTemplateProcessor` + fixture + its two tests. Verified independently buildable/testable/byte-identical at each stage before committing.
- **Added a local `RecordingLogPort`** (not exported to `src/test_support/`, not named in the plan's Artifacts table) because `SystemLogAdapter`'s `LogStats.entries_by_level` is never populated by `update_stats`, so there was no way to assert a specific log level was written through the real adapter. `UserService.log_port` is already `Arc<dyn LogPort>`, so substituting a narrowly-scoped recording double is the same category of seam D-10 endorses for `NotificationChannelHandler` — no mocking framework, no new dependency, `Debug + Default` derived, minimal trait-method bodies for the 17 methods not exercised by these tests.
- **Wired a cached `"user_welcome"` template and a `PassthroughTemplateProcessor` into the Task 2 fixture.** Reading `NotificationService::send_notification` showed it runs `process_template` (template lookup + processor render) *before* looking up a channel handler. Without a cached template, `send_notification` already fails at `ConfigurationError("No template processor configured")` in every existing test (including the five pre-plan ones) — registration still succeeds and a warning is still logged, but purely because of a missing template processor, not because of any registered channel handler. This meant the first draft of `notification_failure_does_not_block_registration` failed with the `FailingChannelHandler`'s `call_count()` staying at `0`: the failure path being asserted was real but was the wrong failure path. Caching a template and registering a trivial passthrough processor lets `send_notification` reach per-channel dispatch, where the `FailingChannelHandler` is genuinely invoked.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Notification template resolution blocked the failing-channel-handler path**
- **Found during:** Task 2 (writing `notification_failure_does_not_block_registration`)
- **Issue:** `send_welcome_notification`'s content carries `template_id: Some("user_welcome")`, and `NotificationService::send_notification` resolves the template before dispatching to a channel handler. With no cached template and no template processor, the very first test run failed the discriminating assertion (`failing_handler.call_count() > 0`) because the notification never reached the channel handler at all.
- **Fix:** Extended `build_service_with_failing_notifications` (test-only, no production change) to cache a `NotificationTemplate` with id `"user_welcome"` and register a minimal `PassthroughTemplateProcessor`, so template resolution succeeds and the failure genuinely happens at channel dispatch.
- **Files modified:** `src/core/platform/manager/user_service.rs` (test module only)
- **Verification:** `notification_failure_does_not_block_registration` passes with `failing_handler.call_count() > 0` and the `Warn` log entry present; the complementary `registration_succeeds_when_no_failing_handler_is_registered` confirms the discrimination.
- **Committed in:** `298d6c3` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary to make the required assertion (failure path genuinely taken) provable at all; no production code touched, no scope creep beyond the plan's own Task 2 objective.

## Issues Encountered

- Early in Task 2, `cargo test`/`cargo clippy` runs briefly picked up a stale compiled binary from the shared `CARGO_TARGET_DIR` (concurrent sibling worktree agent 15-08 building into the same directory) that silently reported only 5 tests instead of 19. Resolved by `touch`ing the source file to force a genuine rebuild before trusting any run; not a defect in this plan's code. No production or test logic was affected.

## Known Stubs

None — no stub patterns (hardcoded empty values, placeholder text, unwired UI props) apply; this plan is test-only.

## Findings for plan 15-10 (recorded per plan's `<output>` instruction)

Three cases where the plan said "assert the observed behaviour rather than assume it":

1. **Case-variant username.** `register_user`'s duplicate check is email-scoped only (`find_by_email`); a username differing solely in letter case from an existing user, registered with a distinct email, is **accepted** (`Ok`). There is no username-uniqueness check anywhere in `register_user`.
2. **Unicode username length rule.** `validate_username` enforces its `>= 3` / `<= 50` bounds via `str::len()` (UTF-8 byte length), not `chars().count()`. A 2-character/4-byte username (`"éé"`) is **accepted**, which a char-count rule would have rejected as too short. Pinned by `register_user_accepts_a_multi_byte_unicode_username_within_the_byte_length_rule`.
3. **Empty/whitespace inputs.** All four cases named in the plan (empty username, whitespace-only username, empty email, empty password) are **rejected**, each with the expected `UserError` variant (`InvalidUsername` x2, `InvalidEmail`, `InvalidPassword`) — no case unexpectedly succeeded.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `user_service.rs`'s registration/validation/hashing/notification-dispatch-failure surface is now covered by 19 total tests (5 pre-existing + 14 new), all discriminating (assert on returned values, not merely "did not panic").
- DEFER-02's Task-2-adjacent half — the module's other operations (login, profile update, activate/deactivate/verify, list/find-by-status) — is not in this plan's scope and remains open for whichever plan owns the rest of `user_service.rs`'s coverage.
- No blockers for phase 15's remaining plans.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*

## Self-Check: PASSED

- FOUND: `src/core/platform/manager/user_service.rs`
- FOUND: commit `a249f68` (Task 1)
- FOUND: commit `298d6c3` (Task 2)
