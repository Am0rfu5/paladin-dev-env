---
phase: 03-verification-depth
plan: 06
subsystem: testing
tags: [rust, coverage, ports, hexagonal, thiserror, llvm-cov]

# Dependency graph
requires:
  - phase: 03-verification-depth
    provides: "03-coverage-measurement.md (03-01) — the per-file entry measurement rows that identified arsenal_port.rs's exact 2 missed lines"
provides:
  - "First #[cfg(test)] unit tests for FileStorageUtils's six default associated functions (file_storage_port.rs)"
  - "First #[cfg(test)] unit test for ArsenalRegistry::list's default body (arsenal_port.rs)"
  - "First caller and full-arm coverage for impl From<LlmProviderError> for LlmError (paladin-llm/src/error.rs)"
affects: [03-07, 03-08]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Exercise a trait's zero-sized `impl Trait for ()` implementor directly via fully-qualified calls (`<() as Trait>::fn(...)`) instead of declaring a redundant delegating wrapper struct"
    - "Exhaustiveness-witness test function with no wildcard arm, to fail the build (not just silently under-cover) when a new error variant is added"

key-files:
  created: []
  modified:
    - crates/paladin-ports/src/output/file_storage_port.rs
    - crates/paladin-ports/src/output/arsenal_port.rs
    - crates/paladin-llm/src/error.rs

key-decisions:
  - "Called the FileStorageUtils default bodies through the trait's existing zero-sized `()` implementor rather than declaring a new test-local struct with an empty `impl FileStorageUtils for TestUtils {}` — the trait declares no default bodies of its own (all six functions end in `;`), so an empty impl block would not compile. `()` already is the zero-sized implementor the plan's acceptance criteria describe; calling through it directly reaches the identical previously-0% code without a pointless delegating wrapper."
  - "LlmProviderError conversion: exercised, not deleted (recorded plan-time decision, carried through unchanged) — every one of the 9 variants (not 10; the file has 9) got its own test asserting the specific mapped LlmError variant, plus one Display-string test and one compile-time exhaustiveness witness with no wildcard arm."
  - "arsenal_port.rs's 2/2 missed lines are ArsenalRegistry::list's default body — confirmed by reading the per-file row in 03-coverage-measurement.md (arsenal_port.rs: 2 Lines, 2 Missed) rather than guessed, then locating the file's only concrete (non-doc-comment, non-signature) executable statements at that default method."

patterns-established:
  - "When a coverage report names a file's exact missed-line count, cross-reference the number against the file's structure before writing tests — a doc-heavy port trait file's true executable surface is often a single default method or one `impl ... for ()` block, not the whole file."

requirements-completed: [QUAL-02]

coverage:
  - id: D1
    description: "FileStorageUtils's six default associated functions (detect_content_type, detect_content_type_with_fallback, validate_content_type_for_domain, calculate_md5, validate_path, sanitize_filename) each have passing unit tests covering their documented accept/reject/fallback behavior"
    requirement: "QUAL-02"
    verification:
      - kind: unit
        ref: "crates/paladin-ports/src/output/file_storage_port.rs#output::file_storage_port::tests (19 tests)"
        status: pass
    human_judgment: false
  - id: D2
    description: "ArsenalRegistry::list's default body has its first caller, proving it returns an empty Vec regardless of implementor storage state"
    requirement: "QUAL-02"
    verification:
      - kind: unit
        ref: "crates/paladin-ports/src/output/arsenal_port.rs#output::arsenal_port::tests (2 tests)"
        status: pass
    human_judgment: false
  - id: D3
    description: "impl From<LlmProviderError> for LlmError has its first caller: every one of the 9 variants is converted and asserted against its exact target LlmError variant (including the ConfigurationError -> ProcessingError prefix remap), all 9 Display strings are pinned, and a compile-time exhaustiveness witness with no wildcard arm exists"
    requirement: "QUAL-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/error.rs#error::tests (11 tests)"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 06: Close paladin-ports and paladin-llm zero-coverage files Summary

**Added `#[cfg(test)]` unit tests exercising FileStorageUtils's six default functions, ArsenalRegistry::list's default body, and every arm of `LlmProviderError -> LlmError` — closing three of the phase's four remaining first-party 0.00% files (the fourth, `src/bin/paladin-server.rs`, is recorded deferred with owner Phase 5/VERIFY-05 per this plan's carried decision, ledger row owned by 03-08).**

## Performance

- **Duration:** ~20 min
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `crates/paladin-ports/src/output/file_storage_port.rs` — 19 new tests exercising all six `FileStorageUtils` default associated functions through the trait's existing zero-sized `()` implementor: content-type detection (known extension, unknown extension, absent extension, with-fallback variants), domain-type validation (accept/reject/empty-list), MD5 digest stability (two pinned test vectors), path validation (accept, and four distinct reject paths: traversal, absolute, empty, over-length — each asserted on the exact returned error message), and filename sanitization (unchanged-when-safe, reserved-character rewriting, control-character rewriting, whitespace trimming).
- `crates/paladin-ports/src/output/arsenal_port.rs` — 2 new tests exercising `ArsenalRegistry::list`'s default body (the file's only 2 previously-missed lines, confirmed against `03-coverage-measurement.md`'s per-file row) via a minimal `MinimalRegistry` implementor that deliberately does not override `list()`.
- `crates/paladin-llm/src/error.rs` — 11 new tests giving `impl From<LlmProviderError> for LlmError` its first caller: one test per variant (9 variants: AuthenticationError, NetworkError, RateLimitExceeded, InvalidPrompt, ProcessingError, TokenLimitExceeded, ModelNotAvailable, Timeout, ConfigurationError) asserting the exact mapped `LlmError` variant and payload, a combined Display-string test pinning all 9 `thiserror` messages, and a compile-time exhaustiveness-witness function with no wildcard arm.

## Task Commits

Each task was committed atomically:

1. **Task 1: Cover the FileStorageUtils default associated functions and the arsenal_port gap** - `3034d42` (test)
2. **Task 2: Give the LlmProviderError conversion its first caller** - `4aa791a` (test)

**Plan metadata:** (this commit, following)

## Files Created/Modified

- `crates/paladin-ports/src/output/file_storage_port.rs` - Added `#[cfg(test)] mod tests` (19 tests) at file end; no other change.
- `crates/paladin-ports/src/output/arsenal_port.rs` - Added `#[cfg(test)] mod tests` (2 tests) at file end; no other change.
- `crates/paladin-llm/src/error.rs` - Added `#[cfg(test)] mod tests` (11 tests) at file end; no other change.

## Decisions Made

- **`file_storage_port.rs` test implementor:** the plan's action text suggested declaring a test-local zero-sized struct with `impl FileStorageUtils for TestUtils {}`. `FileStorageUtils` has no default method bodies (every signature ends in `;` — only `impl FileStorageUtils for ()` has bodies), so an empty impl block on a new struct would not compile. Called the six functions through `<() as FileStorageUtils>::fn_name(...)` instead — `()` is already the zero-sized implementor the acceptance criteria describe, and this reaches the identical previously-0% lines without a redundant delegating wrapper. (Rule 1 — the literal plan instruction as written would not compile; fixed inline, behavior and intent preserved.)
- **`arsenal_port.rs` target confirmed from the measurement record, not guessed:** `03-coverage-measurement.md`'s per-file row reads `crates/paladin-ports/src/output/arsenal_port.rs  2  2  0.00% ... 2  2  0.00%` (2 counted/2 missed lines). Cross-referencing the file's structure (extensive doc comments and bare trait signatures with no bodies) identified `ArsenalRegistry::list`'s default `{ Vec::new() }` body as the file's only concrete executable statement — confirmed, not guessed, per the plan's explicit instruction.
- **`LlmProviderError` variant count:** the plan's action text says "cover all ten variants" but lists nine by name; the file itself has exactly 9 variants (AuthenticationError, NetworkError, RateLimitExceeded, InvalidPrompt, ProcessingError, TokenLimitExceeded, ModelNotAvailable, Timeout, ConfigurationError). All 9 are covered; the verify command's `-ge 9` threshold on `error::tests::` count and this plan's own "Cover all ten variants: [nine named]" list both anticipate the actual count.
- **QUAL-02 deliberately NOT marked complete in REQUIREMENTS.md**, consistent with the decision plan 03-05 recorded in STATE.md: sibling plans 03-07 (final adjudication against the exit coverage measurement) and 03-08 (the deferred-item ledger row for `src/bin/paladin-server.rs`) have not yet produced their SUMMARY.md, and `paladin-server.rs` itself still measures 0.00% coverage (deferred with a named owner, not closed) — so the requirement's literal text ("No first-party source file reports 0% coverage") is not yet true program-wide even after this plan's three closures. Running `requirements mark-complete QUAL-02` did flip the checkbox; that write was reverted via `git checkout -- .planning/REQUIREMENTS.md` before this plan's final commit so the premature mark does not ship. Final adjudication remains 03-07's to make.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `file_storage_port.rs` test-module wrapper struct as literally specified would not compile**
- **Found during:** Task 1
- **Issue:** Plan text asked for `struct TestUtils;` with `impl FileStorageUtils for TestUtils {}` (an empty impl block), but `FileStorageUtils` declares no default method bodies — every one of its six functions must be implemented, so an empty impl fails to compile with "not all trait items implemented" errors.
- **Fix:** Called the six functions through the trait's existing zero-sized `()` implementor directly (`<() as FileStorageUtils>::fn_name(...)`), which reaches the exact same previously-0% code (`impl FileStorageUtils for ()`, lines ~1370-1447) without declaring a new type.
- **Files modified:** `crates/paladin-ports/src/output/file_storage_port.rs`
- **Verification:** `cargo test -p paladin-ports --offline` — 19/19 new tests pass; `cargo fmt --check` and `cargo clippy --offline --all-targets --all-features -- -D warnings` both clean.
- **Committed in:** `3034d42` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug fix, literal-instruction-wouldn't-compile class)
**Impact on plan:** No scope change — the fix reaches the identical target lines the plan specified, via a simpler and compiling path. No behavior, public API, or test count differs from what the plan's acceptance criteria require.

## Issues Encountered

None beyond the deviation above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Three of the phase's four remaining first-party 0.00% files are now closed with passing, non-Docker, offline unit tests: `file_storage_port.rs`, `arsenal_port.rs`, `error.rs`.
- `src/bin/paladin-server.rs` remains deferred with owner Phase 5 / VERIFY-05, per this plan's carried decision; the ledger row recording that deferral is written by plan 03-08, not this plan.
- No public API changed and no `Cargo.toml` changed in either `paladin-ports` or `paladin-llm` — safe for any sibling or downstream plan to build on without a re-audit of these crates' surfaces.
- `cargo fmt --check` and `cargo clippy -p paladin-ports --offline --all-targets --all-features -- -D warnings` / `cargo clippy -p paladin-llm --offline --all-targets --all-features -- -D warnings` are both clean as of this plan's final commit.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

All created/modified files and task commits verified present on disk / in git log.
