---
phase: 03-verification-depth
plan: 05
subsystem: testing
tags: [rust, redis, unit-tests, tdd, coverage, paladin-storage]

# Dependency graph
requires:
  - phase: 03-verification-depth (03-01)
    provides: reproduced ADR-0006 coverage pipeline and confirmed the zero-coverage-file set
      that names redis.rs as one of the offenders this plan closes
provides:
  - "Six Redis key-building helpers (queue_key, priority_queue_key, queue_meta_key,
    processing_key, completed_key, failed_key) refactored from &self methods to free
    functions taking &RedisQueueConfig, callable without a connected RedisQueueAdapter"
  - "serialize_item / deserialize_item refactored to free functions taking no self"
  - "First #[cfg(test)] mod tests in crates/paladin-storage/src/redis.rs -- 11 tests,
    Docker-free, ~0s runtime"
  - "In-source record that redis.rs's live-server code paths remain uncovered, deferred
    with reason, owner Phase 15 (PIPE)"
affects: [03-06, 03-07, 03-08]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Connection-independent seam extraction: private &self helpers whose bodies never
      touch a live connection are moved to free functions taking config/data by value
      or reference, making them unit-testable without constructing the adapter that
      would otherwise force a live connection at construction time"

key-files:
  created: []
  modified:
    - crates/paladin-storage/src/redis.rs

key-decisions:
  - "Refactor (Task 1) and tests (Task 2) landed as two separate commits, in that
    strict order, per the plan's explicit two-task structure"
  - "priority_queue_key's second parameter renamed queue_name -> queue solely to fit
    the refactored signature under rustfmt's 100-column width on one line, which the
    plan's own acceptance-criteria grep requires; no other parameter names changed"
  - "QUAL-02 requirement NOT marked complete: `requirements ready-ids` reports it
    blocked because 03-06/03-07/03-08 also carry QUAL-02 in frontmatter and have not
    yet produced their SUMMARY.md. Left for whichever plan's ready-ids check clears it."

requirements-completed: []  # QUAL-02 intentionally NOT marked -- see key-decisions and Deviations

coverage:
  - id: D1
    description: "redis.rs key/serialization helpers refactored off &self to free functions, private-surface-only"
    requirement: "QUAL-02"
    verification:
      - kind: unit
        ref: "cargo build --offline -p paladin-storage && cargo test -p paladin-storage --offline --features redis-queue"
        status: pass
    human_judgment: false
  - id: D2
    description: "First unit test module for redis.rs -- 11 Docker-free tests covering config defaults, all six key builders, priority-key collision-freedom, serialize/deserialize round-trip, error mapping, and get_priority_levels order"
    requirement: "QUAL-02"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-storage --offline --features redis-queue redis (redis::tests::*, 11 passed)"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 05: Redis Queue Adapter Unit-Testability Summary

**Moved eight private `&self` helpers in `redis.rs` to connection-free functions and gave the file its first unit tests — 11 tests, zero Docker, zero live Redis, ~0s runtime, closing the single largest zero-coverage first-party file in the tree.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-02T15:15:00Z (approx, file reads preceding first commit)
- **Completed:** 2026-08-02T15:40:31Z
- **Tasks:** 2 completed
- **Files modified:** 1 (`crates/paladin-storage/src/redis.rs`)

## Accomplishments

- Converted the six key-building helpers (`queue_key`, `priority_queue_key`, `queue_meta_key`,
  `processing_key`, `completed_key`, `failed_key`) from `&self`-taking instance methods to
  private free functions taking `&RedisQueueConfig`, and converted `serialize_item` /
  `deserialize_item` to free functions taking no config at all — a private-surface-only
  refactor with byte-identical key formats and no public API change.
- Updated every call site inside `RedisQueueAdapter`'s trait impls (`QueuePort`,
  `BatchQueuePort`, `PriorityQueuePort`, `QueueManagementPort`) from method to function form,
  including the commented-out dead code inside `cleanup_expired` (kept commented, updated to
  match the new call shape so it stays internally consistent).
- Added `crates/paladin-storage/src/redis.rs`'s first `#[cfg(test)] mod tests` block: 11 tests
  covering `RedisQueueConfig::default()` field-by-field, all six key builders against complete
  expected literals, priority-key collision-freedom across all four `MessagePriority` variants,
  a `serialize_item`/`deserialize_item` round trip of identifying fields, explicit error-variant
  matching on invalid JSON, and `get_priority_levels()`'s documented order.
- Recorded in-source that the live-server code paths of `redis.rs` remain uncovered here,
  deferred with reason, with `tests/integration/redis_queue_integration_test.rs` named as their
  Docker-requiring testcontainers-based exerciser and Phase 15 (PIPE) named as owner.

## Task Commits

Each task was committed atomically:

1. **Task 1: Move the eight private key and serialization helpers off `&self`** - `c895f4f` (refactor)
2. **Task 2: Add the Docker-free unit test module for redis.rs pure seams** - `c169ee2` (test)

**Plan metadata:** (this commit, docs)

## Files Created/Modified

- `crates/paladin-storage/src/redis.rs` - Eight private helpers moved off `&self`; new
  `#[cfg(test)] mod tests` block with 11 unit tests and a deferred-live-server-coverage note.

## Decisions Made

- Renamed `priority_queue_key`'s second parameter from `queue_name` to `queue` — purely to
  bring the refactored one-line signature under rustfmt's 100-column width so it matches the
  plan's acceptance-criteria grep (`fn priority_queue_key(config: &RedisQueueConfig...`) on a
  single line. No behavior or call-site-visible name changed (the parameter isn't named at
  call sites in Rust).
- Did not run `requirements mark-complete QUAL-02` — `requirements ready-ids` reports it
  `blocked` because sibling plans 03-06, 03-07 and 03-08 in this phase directory also carry
  QUAL-02 in their frontmatter and have not yet produced SUMMARY.md files. Per the plan's own
  "Edge coverage" note, QUAL-02's final adjudication happens in 03-07 against the exit
  coverage measurement.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] `Self::get_priority_levels()` call sites required qualification when moving to free-function names**
- **Found during:** Task 2 (writing the test module)
- **Issue:** `get_priority_levels()` is an inherent associated function on `RedisQueueAdapter` (`impl RedisQueueAdapter { fn get_priority_levels() ... }`), not a free function like the other seven helpers. Calling it bare inside `mod tests` after `use super::*` does not resolve. A blanket text substitution I ran while fixing an unrelated naming issue briefly mis-qualified it everywhere (including its own `fn` definition and the `Self::` call sites inside the impl blocks), which failed to compile.
- **Fix:** Restored `Self::get_priority_levels()` at all seven in-impl call sites (including the one inside commented-out dead code) and the bare `fn get_priority_levels()` definition; qualified only the three new test-module call sites as `RedisQueueAdapter::get_priority_levels()`.
- **Files modified:** `crates/paladin-storage/src/redis.rs`
- **Verification:** `cargo build --offline -p paladin-storage --features redis-queue` succeeds; `cargo test` passes.
- **Committed in:** `c169ee2` (Task 2 commit)

**2. [Rule 3 - Blocking] Task 2's own `<automated>` verify command needs `--features redis-queue` to run at all**
- **Found during:** Task 2 verification
- **Issue:** `crates/paladin-storage/src/lib.rs` gates `pub mod redis;` behind `#[cfg(feature = "redis-queue")]`, and that feature is not in the crate's (or workspace root's) default feature set. Running the plan's literal verify command (`cargo test -p paladin-storage --offline redis`, no `--features`) compiles and runs 0 tests, which still satisfies the `test result: ok.` grep but then fails the `-ge 6` test-count check — the command as written cannot pass in either direction without the feature flag.
- **Fix:** Ran verification with `cargo test -p paladin-storage --offline --features redis-queue redis`, which is otherwise identical to the plan's command. All 11 tests pass. Also ran `cargo clippy -p paladin-storage --all-targets --features redis-queue -- -D warnings` and the workspace-wide `cargo clippy --workspace --all-targets --all-features -- -D warnings` (this project's own `make lint` target) to make sure the new test module was actually type-checked and linted, since a feature-less workspace clippy run silently skips `redis.rs` entirely.
- **Files modified:** None (verification-only; no test-runner script exists in this repo to patch).
- **Verification:** `cargo test -p paladin-storage --offline --features redis-queue redis` → `test result: ok. 11 passed`. `cargo clippy --workspace --all-targets --all-features -- -D warnings` → clean.
- **Committed in:** N/A (documentation-only deviation)

**3. [Rule 3 - Blocking] Task 2's own `<automated>` verify command's credential/hostname grep is unsatisfiable against the whole file**
- **Found during:** Task 2 verification
- **Issue:** The plan's acceptance criterion `grep -cE '(redis://|127\.0\.0\.1:6379|localhost:6379|password)' crates/paladin-storage/src/redis.rs` returns 0 checks the **entire file**, not just the new test module. `redis.rs` already legitimately contains `redis://` connection-URL construction and a `redis_password` field in `RedisQueueAdapter::new()` and `RedisQueueConfig` (pre-existing, functionally required, private-surface-only code this plan is explicitly forbidden from changing). That grep returns 6 before this plan touches anything. Additionally, testing `RedisQueueConfig::default()`'s `redis_password` field (required by the plan's own `<behavior>` spec: "each default is asserted by value") necessarily writes the substring `redis_password` inside the test module too, so even a module-scoped version of this grep can't return exactly 0.
- **Fix:** Verified the actual `must_haves.truths` intent instead — "No test in the new module contains a real hostname, credential, or connection URL — every fixture value is a synthetic literal defined in the test module" — by inspecting the test module's only match (`assert_eq!(config.redis_password, None)`, a field-name reference to a `None` default, not a real credential) and confirming no real hostname, connection URL, or credential value appears anywhere in the new code.
- **Files modified:** None.
- **Verification:** Manual scoped grep against `mod tests { ... }` only, confirmed the sole match is the field-name assertion described above.
- **Committed in:** N/A (documentation-only deviation)

---

**Total deviations:** 3 auto-fixed/documented (1 blocking compile fix, 2 blocking verify-script gaps worked around and documented)
**Impact on plan:** No scope creep. All three deviations were necessary to actually execute and verify the plan as intended; none altered the deliverable's shape.

## Issues Encountered

None beyond the deviations above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `redis.rs` is no longer a 0.00%-coverage first-party file for its pure seams; the live-server
  gap is explicitly recorded with Phase 15 (PIPE) as owner rather than silently left open.
- QUAL-02 remains open pending 03-06 (the other in-phase QUAL-02 plan), 03-07 (exit-measurement
  adjudication), and 03-08 (final ledger). No blocker for proceeding to the next wave.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

- FOUND: `crates/paladin-storage/src/redis.rs`
- FOUND: `.planning/phases/03-verification-depth/03-05-SUMMARY.md`
- FOUND commit: `c895f4f` (Task 1)
- FOUND commit: `c169ee2` (Task 2)
