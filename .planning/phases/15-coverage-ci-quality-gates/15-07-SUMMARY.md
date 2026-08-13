---
phase: 15-coverage-ci-quality-gates
plan: 07
subsystem: testing
tags: [rust, tokio-test, argon2, sqlite, coverage, defer-02, concurrency]

# Dependency graph
requires:
  - phase: 15-coverage-ci-quality-gates
    provides: "15-06's registration/validation/argon2-hashing/notification-failure test module and fixtures (build_service, registration, RecordingLogPort, build_service_with_failing_notifications)"
provides:
  - "26 new #[tokio::test]s in the existing co-located test module of src/core/platform/manager/user_service.rs covering authentication, profile lifecycle, queries, additional validation edge cases, and concurrent same-username registration"
  - "A recorded, commanded 94.21% module-line-coverage measurement for user_service.rs, plus a DEFER-02 justification block naming every intentionally untested path"
  - "Confirmation that concurrent same-username registration has no real race in register_user's own logic -- the database's username UNIQUE constraint catches what the email-scoped application check does not"
affects: [15-10]

# Tech tracking
tech-stack:
  added: [cargo-llvm-cov (dev tool, installed via rustup component + cargo install, not a project dependency)]
  patterns:
    - "sqlx's SqlitePool against `sqlite::memory:` shares one database across all pooled connections in this codebase's observed behavior (confirmed empirically with a concurrent two-connection probe before writing the final test), so a #[tokio::test(flavor = \"multi_thread\")] test with tokio::join! over two Arc-cloned service handles genuinely exercises the database's own unique constraints rather than two isolated in-memory databases"
    - "cargo-llvm-cov builds its instrumented artifacts under a separate llvm-cov-target/ subdirectory inside CARGO_TARGET_DIR, isolated from the normal debug/ cache that `cargo test`/`cargo build` reuse"

key-files:
  created: []
  modified:
    - src/core/platform/manager/user_service.rs

key-decisions:
  - "Verified the sqlite::memory: pool's cross-connection sharing behavior empirically with a throwaway probe test (logged both concurrent results and count_users via eprintln!) before writing the final concurrent-registration assertions, per the plan's own directive to assert the observed outcome rather than an assumption -- removed the probe once the real behavior was confirmed stable across 5 repeated runs."
  - "Added four cheap additional tests beyond Task 1/Task 2's explicit action lists (username too-short-non-whitespace, username >50 chars, username with invalid characters, password >128 chars, verify_password against a malformed hash) to close low-effort coverage gaps discovered while reading the llvm-cov JSON output, rather than leaving them for the justification block -- reduced the file's uncovered line count from 14 to 9 in the production half."
  - "Recorded the coverage measurement's commit SHA as 432d514 (this plan's Task 1 commit, immediately preceding Task 2) rather than Task 2's own resulting commit hash, since the SHA had to be embedded inside the file Task 2 commits -- explicitly noted in the justification block that Task 2 is test-only and the production half stays byte-identical, so the figure holds for Task 2's own commit as well."
  - "Ran cargo llvm-cov --workspace --lib (required by the plan's own verify block and acceptance criteria) despite this worktree's build_verification_policy generally discouraging workspace-wide builds that touch paladin-web -- verified afterward that cargo-llvm-cov's instrumented artifacts land in an isolated llvm-cov-target/ subdirectory, not the shared debug/ cache paladin-web's own cargo test would reuse, so the staleness risk the policy warns about (a worktree-path-baked binary surviving into a later run from the main checkout) does not apply to this specific build path. Recorded as a transparency note in Issues Encountered."

requirements-completed: []

coverage:
  - id: D1
    description: "Authentication covered on both sides of the credential check: correct password issues a token, incorrect password does not, login for a never-registered identity and login against a deactivated account each return a defined, mutually distinct UserError variant"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#login_with_incorrect_password_issues_no_token_and_does_not_succeed"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#login_for_a_never_registered_email_returns_authentication_failed"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#login_against_a_deactivated_account_is_rejected_as_a_distinct_variant"
        status: pass
    human_judgment: false
  - id: D2
    description: "Profile lifecycle covered: update_user_profile hit/miss, email-change verification-state reset observed and pinned, activate_user/deactivate_user/verify_user each on an existing and an unknown user with state read back via a query"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#update_user_profile_on_existing_user_changes_the_stored_profile"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#update_user_profile_on_an_unknown_user_returns_user_not_found"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#update_user_profile_email_change_resets_the_verification_state"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#activate_user_on_existing_user_is_reflected_when_read_back"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#deactivate_user_on_existing_user_is_reflected_when_read_back"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#verify_user_on_existing_user_is_reflected_when_read_back"
        status: pass
    human_judgment: false
  - id: D3
    description: "Queries covered: get_user_by_id/get_user_by_email hit and miss (None on miss, not an error), count_users empty and after N registrations, find_by_active_status and find_by_verification_status in both polarities asserting membership"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#get_user_by_id_miss_returns_none"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#get_user_by_email_miss_returns_none"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#count_users_is_zero_on_an_empty_repository"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#find_by_active_status_asserts_membership_in_both_polarities"
        status: pass
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#find_by_verification_status_asserts_membership_in_both_polarities"
        status: pass
    human_judgment: false
  - id: D4
    description: "Concurrent same-username registration exercised under an explicit tokio::time::timeout; observed outcome is exactly one persisted user, the loser surfacing a RepositoryError from the database's unique-constraint violation -- no application-level race exists"
    requirement: "DEFER-02"
    verification:
      - kind: unit
        ref: "src/core/platform/manager/user_service.rs#concurrent_registration_with_the_same_username_leaves_exactly_one_user_persisted"
        status: pass
    human_judgment: false
  - id: D5
    description: "user_service.rs measures 94.21% line coverage (927/984 lines) under cargo llvm-cov --workspace --lib, default features, recorded with commit SHA and date, and explicitly labelled non-comparable with ADR-0006's 84% integration-tests-scoped gate"
    requirement: "DEFER-02"
    verification:
      - kind: other
        ref: "cargo llvm-cov --workspace --lib --json --output-path /tmp/cov.json (figure extracted for user_service.rs's summary.lines.percent)"
        status: pass
    human_judgment: false

duration: ~1h 20min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 07: User service authentication, profile, query and concurrency coverage Summary

**26 new characterization tests closing DEFER-02's remaining scope on `user_service.rs` -- login, profile lifecycle, queries, extra validation edge cases and a database-arbitrated concurrent-registration race -- reaching 94.21% module line coverage, with a recorded justification block for every remaining gap.**

## Performance

- **Duration:** ~1h 20min
- **Tasks:** 2
- **Files modified:** 1 (`src/core/platform/manager/user_service.rs`, test module only)

## Accomplishments

- Authentication covered on both sides of the credential check, with the deactivated-account rejection (`UserNotActive`) and the wrong-password rejection (`AuthenticationFailed`) asserted as distinct variants (T-15-20).
- Profile lifecycle covered end to end: `update_user_profile` hit/miss, an email change's verification-state reset observed and pinned as the module's real (unconditional) behaviour, and `activate_user`/`deactivate_user`/`verify_user` each proven on both an existing and an unknown id with the resulting state read back through a query rather than inferred from the call's return value.
- Every query method (`get_user_by_id`, `get_user_by_email`, `count_users`, `find_by_active_status`, `find_by_verification_status`) covered on hit and miss / both polarities, asserting membership rather than length alone.
- Concurrent same-username registration driven under `#[tokio::test(flavor = "multi_thread")]` with an explicit `tokio::time::timeout`, and the actual outcome empirically verified (via a throwaway probe run before finalizing assertions, then confirmed stable across 5 repeated runs of the final test) rather than assumed: exactly one call succeeds, the loser returns `UserError::RepositoryError` wrapping a SQLite `UNIQUE constraint failed: users.username` error, and `count_users` reports exactly one row afterward. **No race exists in `register_user`'s own logic** -- its email-scoped duplicate check does clear both concurrent calls, but the database's `username TEXT UNIQUE NOT NULL` constraint (declared in `sqlite_user_repository.rs`'s migration) catches the collision downstream. This closes with a finding recorded for 15-10, not a production fix, per the plan's explicit scope boundary.
- Measured `user_service.rs` at **94.21% line coverage (927/984 lines)** via `cargo llvm-cov --workspace --lib --json --output-path /tmp/cov.json` (default features, no `--features integration-tests`), well above the 80% DEFER-02 bar -- see the file's own justification block for the exact command, scope, commit SHA and date, and why this figure is not comparable to ADR-0006's 84% gate.
- A `//!` justification block recorded at the head of `#[cfg(test)] mod tests` names every DEFER-02-scoped path left untested (login-attempt tracking -- not implemented by the module; the generic "repository error" edge case -- would need a dedicated `UserRepositoryPort` test double disproportionate to this plan's scope) plus nine further uncovered lines with individual reasons, and two observed-behaviour findings for plan 15-10.

## Task Commits

Each task was committed atomically:

1. **Task 1: Authentication, profile lifecycle and query coverage** - `432d514` (test)
2. **Task 2: Concurrent registration, the module measurement, and the justification record** - `42ae77f` (test)

**Plan metadata:** (this commit)

## Files Created/Modified
- `src/core/platform/manager/user_service.rs` - Extended the existing `#[cfg(test)] mod tests` block with 26 new tests (20 in Task 1, 6 in Task 2) plus a justification-record doc comment block at the module's head. Nothing above the `#[cfg(test)]` marker changed (verified byte-identical against `git show HEAD:` after every commit).

## Decisions Made

- **Verified `sqlite::memory:`'s cross-connection sharing behaviour empirically before writing the concurrent test's final assertions.** The plan warned this file's own duplicate check is email-scoped, so two concurrent same-username calls both clear it -- but whether sqlx's pooled connections against `sqlite::memory:` share one database (so the DB's own `UNIQUE` constraint can even be reached across two simultaneous connections) or each get an isolated private database was not something to assume. A throwaway probe test (`eprintln!`-based, not committed) confirmed the shared-database behaviour directly; the final test's assertions match that observation, run 5 times with no flake before being finalized.
- **Added four cheap additional tests beyond Task 1/Task 2's explicit action lists.** Reading the `cargo llvm-cov` JSON output after Task 1 surfaced several trivially-closeable gaps (`validate_username`'s length-bound and invalid-character branches; `verify_password` against a malformed hash string). Closing them lowered the uncovered-line count in the production half from 14 to 9 and kept the justification block focused on genuinely hard-to-reach paths rather than easy oversights.
- **Recorded the justification block's commit SHA as Task 1's hash (`432d514`), not Task 2's own.** The SHA has to be written inside the file that Task 2 commits, so Task 2's own hash cannot be known in advance without amending (prohibited). The block explicitly notes Task 2 is test-only and the production half stays byte-identical, so the recorded figure holds for Task 2's commit too -- verifiable directly from the diff.
- **Ran the workspace-wide `cargo llvm-cov --workspace --lib` the plan's own verify block requires**, despite this worktree's `build_verification_policy` generally steering away from workspace-wide builds that touch `paladin-web`. See Issues Encountered for the specific risk assessment and why it was judged acceptable here.

## Deviations from Plan

None - both tasks executed as written, with the additional tests noted above staying inside Task 2's own stated latitude ("If the figure is below 80%, add tests ... or justify each remaining region"). No Rule 1-4 auto-fixes were needed; no production line above `#[cfg(test)]` changed.

## Issues Encountered

- **Workspace-wide coverage build touched `paladin-web`, contrary to this worktree's general build-verification guidance.** The plan's own `<verify>` block and acceptance criteria require `cargo llvm-cov --workspace --lib` for the module measurement -- there is no way to scope `cargo llvm-cov` to a single crate's dependents without changing what "workspace line coverage" means. This built and ran `paladin-web`'s test suite (including `openapi_matches_committed_baseline`, the specific test the orchestrator's guidance called out as risky to compile from inside a worktree into the shared `CARGO_TARGET_DIR`). Checked `/workspace/target/` afterward: `cargo-llvm-cov` places its instrumented build artifacts in a separate `llvm-cov-target/` subdirectory, distinct from the `debug/` cache that a plain `cargo test -p paladin-web` would reuse -- so the specific staleness risk described (a worktree-path-baked binary silently reused after this worktree is removed) does not apply to this build path; nothing was written into the shared normal test-artifact cache. Flagging this transparently rather than silently proceeding, in case the orchestrator wants to confirm `paladin-web`'s own centralized test run separately.
- Coverage tooling (`cargo-llvm-cov`) was not pre-installed in this worktree; installed via the documented procedure (`rustup component add llvm-tools-preview` -- already present -- then `cargo install cargo-llvm-cov --locked`, ~1m46s to compile).

## Known Stubs

None -- no stub patterns (hardcoded empty values, placeholder text, unwired UI props) apply; this plan is test-only.

## Findings for plan 15-10 (recorded per plan's `<output>` instruction)

1. **Concurrent same-username registration is database-arbitrated, not application-arbitrated, and there is no race to fix.** `register_user`'s own duplicate check (established by 15-06) is email-scoped only, so two concurrent calls with the same username and different emails both clear it. The collision is caught downstream by `sqlite_user_repository.rs`'s migration-declared `username TEXT UNIQUE NOT NULL` constraint: exactly one `INSERT` succeeds, the other returns `UserError::RepositoryError` wrapping a SQLite `UNIQUE constraint failed: users.username` error. Confirmed directly, run 5 times with no flake. No production change needed and none made.
2. **The "send a welcome notification" success path (`send_welcome_notification`'s `Ok(())` return, line 163) is unreachable in this file's entire test suite, by design.** `build_service()` -- the fixture nearly every test in this module uses -- never caches a `"user_welcome"` template or registers a template processor, so `NotificationService::send_notification` always fails at template resolution. Every test's welcome-notification path, including the two tests specifically proving "notification failure does not block registration" (15-06 Task 2), exercises the *failure* branch, never the success branch. This does not weaken the guarantee DEFER-02 asks for, but it means the module's 94.21% coverage figure comes entirely through that one call's failure branch. Reaching the success branch would need a third notification fixture (cached template + processor + a channel handler that succeeds), which neither this plan's tasks nor 15-06's asked for.
3. **Login-attempt tracking, named in DEFER-02's authentication scope, is not implemented anywhere in this module.** `login_user` has no attempt counter, no lockout threshold, and `UserData` carries no related field. `grep -n "attempt" src/core/platform/manager/user_service.rs` matches only two log messages, not a counter. There is nothing to test; DEFER-02's own scope text names a path the shipped code does not have.
4. **The generic "repository error" edge case named in DEFER-02's scope is intentionally left untested at the `user_service.rs` level.** Every method in this file propagates `UserRepositoryPort` failures with a bare `?`; forcing a live failure at this layer would need a dedicated test double, and `SqliteUserRepository`'s own error-mapping already has direct unit tests in `sqlite_user_repository.rs`. Recorded as a deliberate scope boundary in the justification block.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `user_service.rs` now carries 45 tests total (5 original + 14 from 15-06 + 26 from this plan; confirmed via `cargo test -p paladin-ai --lib core::platform::manager::user_service -- --list`) covering registration, validation, argon2 hashing, notification-dispatch-failure tolerance, authentication, profile lifecycle, queries, and concurrent registration, all discriminating.
- The module measures 94.21% line coverage under the recorded command and scope, well above DEFER-02's 80% bar. This is a one-time plan-acceptance measurement (D-12) and is not wired into any CI gate by this plan.
- Two ingested registers (Deferred-QA Epic 28 "test it" vs Milestone 8 `deferred-items.md` D2 "split it") previously proposed incompatible next actions on this file; ADR-0034 already withdrew the split, leaving DEFER-02's test scope as the sole remaining owner. This plan closes that scope's second half; plan 15-10 owns amending DEFER-02 at source using this summary's justification block and findings.
- No blockers for phase 15's remaining plans.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*
