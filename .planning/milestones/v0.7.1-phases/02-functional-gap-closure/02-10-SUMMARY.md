---
phase: 02-functional-gap-closure
plan: 10
subsystem: testing
tags: [rust, herald, utf-8, char-boundary, comfy-table, table-herald]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-04 wired format_battalion_result's real Paladin-name call site (table_herald.rs:203) that first exercises truncate_text against execution-service-sourced names"
provides:
  - "A char-boundary-safe, total TableHerald::truncate_text that measures its budget in Unicode scalar values (chars), never bytes, and never panics for any input at any configured width"
  - "Four tests whose inputs are arithmetically guaranteed to reach the truncation branch they prove, each verified to panic against the pre-fix code"
  - "02-EDGE-PROBE.md's no-silent-drop accounting amended to record that probe row 8's encoding disposition was falsified and re-proven, not silently re-graded"
affects: [02-11, phase-02-verification-reconciliation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Char-count truncation via chars().take(n).collect(), following paladin-llm's bounded_excerpt precedent (crates/paladin-llm/src/anthropic/adapter.rs:611)"
    - "Table-driven width sweep (0..=24 plus default 60) crossed with 2/3/4-byte and mixed-ASCII inputs to cover every char-boundary residue class"

key-files:
  created: []
  modified:
    - "crates/paladin-herald/src/table_herald.rs"
    - ".planning/phases/02-functional-gap-closure/02-EDGE-PROBE.md"

key-decisions:
  - "max_column_width is a character budget, not a byte budget — aligns the code to its own pre-existing doc comment ('Maximum width for table columns (characters)'), rated reversible (no persisted schema, single private call site)"
  - "truncate_text stays total (-> String), never made fallible — preserves ADR-0005's deliberate format_error infallibility and every existing call site"

patterns-established:
  - "Admissibility rule for multi-byte truncation tests: an input only proves the truncation branch if the byte offset at width - 3 is NOT a char boundary of that input — verify this arithmetically before writing the test, not after"

requirements-completed: [GAP-03]

coverage:
  - id: D1
    description: "Rendering a BattalionResult with an over-budget multi-byte Paladin name returns Ok at both the default (60) and a narrow (20) column budget, with no U+FFFD"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "crates/paladin-herald/src/table_herald.rs#test_table_herald_renders_overlong_multibyte_paladin_name"
        status: pass
    human_judgment: false
  - id: D2
    description: "format_error renders a long multi-byte PaladinError display string without panicking, preserving its infallible -> String signature"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "crates/paladin-herald/src/table_herald.rs#test_format_error_renders_overlong_multibyte_message"
        status: pass
    human_judgment: false
  - id: D3
    description: "truncate_text never returns more chars than the configured budget, across a swept width range and 2-byte/3-byte/4-byte/mixed-ASCII inputs"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "crates/paladin-herald/src/table_herald.rs#test_truncate_text_never_exceeds_width_for_any_multibyte_input"
        status: pass
    human_judgment: false
  - id: D4
    description: "truncate_text at widths 0, 1 and 2 against over-long input returns exactly that many chars, with no ellipsis and no panic (usize underflow closed)"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "crates/paladin-herald/src/table_herald.rs#test_truncate_text_handles_width_below_ellipsis"
        status: pass
    human_judgment: false
  - id: D5
    description: "02-EDGE-PROBE.md's no-silent-drop accounting attributes row 8's GAP-03 encoding edge to 02-04 and 02-10 together, with totals updated (20 accounted / 17 surfaced) and a dated amendment explaining why the original covered disposition was insufficient"
    verification:
      - kind: other
        ref: "grep checks against .planning/phases/02-functional-gap-closure/02-EDGE-PROBE.md (02-10 mentions, 're-authored', 'Total accounted..20', engine's 17 rows untouched, §B prohibition count untouched)"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 10: TableHerald char-boundary-safe truncation Summary

**`TableHerald::truncate_text` now measures and slices by Unicode scalar values instead of bytes, closing the panic that shipped behind a self-confirming multi-byte test — plus the two adjacent panic paths (`format_error`, sub-ellipsis widths) that shared the same defective helper.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-01T20:21:19Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- Closed the shipped BLOCKER: `TableHerald::truncate_text` no longer byte-indexes a `&str`, so it cannot panic on a multi-byte Paladin name whose cut point lands mid-character
- Closed two further reachable panic paths discovered while fixing the first: `format_error`'s infallible `String` render (ADR-0005), and `usize` underflow at `max_column_width` values of 1 and 2 (accepted operator configuration per `HeraldConfig::validate`)
- Reinterpreted `max_column_width`'s unit from bytes to characters, aligning the code to its own pre-existing doc comment
- Amended `02-EDGE-PROBE.md`'s no-silent-drop accounting to record that probe row 8's `encoding` edge was falsified by this plan's execution and re-proven with an admissibility rule, rather than silently re-graded

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end tracer — reproduce the Battalion-rendering panic, then fix the truncation unit** — `53a3074` (fix)
2. **Task 2: Close the remaining two reachable panic paths and pin the width invariant** — `617a0bb` (test)
3. **Task 3: Amend the probe record so row 8's encoding disposition shows it was falsified and re-proven** — `f43a9ab` (docs)

_TDD note: Task 1 wrote `test_table_herald_renders_overlong_multibyte_paladin_name` and confirmed it panicked against the pre-fix code before implementing the fix, all within one commit — the plan's `<action>` specified this as a single atomic unit (write failing test, confirm panic, fix, verify green) rather than separate RED/GREEN commits._

## Files Created/Modified

- `crates/paladin-herald/src/table_herald.rs` — `truncate_text` rewritten to count and slice by `chars()`; four new tests added (`test_table_herald_renders_overlong_multibyte_paladin_name`, `test_format_error_renders_overlong_multibyte_message`, `test_truncate_text_never_exceeds_width_for_any_multibyte_input`, `test_truncate_text_handles_width_below_ellipsis`)
- `.planning/phases/02-functional-gap-closure/02-EDGE-PROBE.md` — no-silent-drop tally updated (18→20 accounted), row-8 disposition split and marked re-authored, dated amendment subsection appended

## Pre-fix Failure Transitions (required record per plan `<output>`)

Each new test was run against the code as it stood **before** Task 1's fix, and each failed by panic — the transition that distinguishes these tests from the self-confirming test that let the original defect ship:

1. **`test_table_herald_renders_overlong_multibyte_paladin_name`** (30× U+1F6E1 🛡, 4 bytes/char, default budget 60, cut offset 57 not a multiple of 4):
   ```
   thread '...' panicked at crates/paladin-herald/src/table_herald.rs:98:35:
   end byte index 57 is not a char boundary; it is inside '🛡' (bytes 56..60 of string)
   ```

2. **`test_format_error_renders_overlong_multibyte_message`** (50× 中, 3 bytes/char, wrapped in `PaladinError::ExecutionError`'s `"Execution error: "` 17-byte thiserror `Display` prefix — cut offset 57 total, 40 bytes into the CJK payload, `40 % 3 == 1`, not a boundary):
   ```
   thread '...' panicked at crates/paladin-herald/src/table_herald.rs:114:35:
   end byte index 57 is not a char boundary; it is inside '中' (bytes 56..59 of string)
   ```

3. **`test_truncate_text_never_exceeds_width_for_any_multibyte_input`** (sweep hits a sub-3 width before any mid-character cut):
   ```
   thread '...' panicked at crates/paladin-herald/src/table_herald.rs:114:38:
   attempt to subtract with overflow
   ```

4. **`test_truncate_text_handles_width_below_ellipsis`** (widths 0/1/2, `usize` underflow in `max_column_width - 3`):
   ```
   thread '...' panicked at crates/paladin-herald/src/table_herald.rs:114:38:
   attempt to subtract with overflow
   ```

Verification method: after writing each test against the fixed `truncate_text`, the fixed implementation was temporarily reverted to the pre-Task-1 byte-indexing version in place, the three Task 2 tests were run individually to capture their exact panic messages, then the fix was restored and the full suite (`cargo test -p paladin-herald`, 70 tests) re-confirmed green before committing. Task 1's own test was proven the same way as part of the tracer task's required RED step.

## Decisions Made

- **`max_column_width` reinterpreted as a character budget, not a byte budget** — the field's own doc comment at `table_herald.rs:42` already read "characters"; the code was what had diverged. Rated **reversible**: nothing is persisted (`TableHeraldConfig` is `#[doc(hidden)]`, reconstructed from `config.yml` on every run), and the only observable effect is that a wide multi-byte name now renders further before truncating.
- **`truncate_text` kept total (`-> String`), not made fallible** — preserves every existing call site and, critically, preserves `format_error`'s ADR-0005-mandated infallibility, which the fallible alternative would have cascaded into.
- **`format_error`'s test used a 3-byte CJK character, not Task 1's 4-byte shield emoji** — because `PaladinError::ExecutionError`'s `thiserror` `Display` prepends `"Execution error: "` (17 ASCII bytes) before the payload reaches `truncate_text`. At the default budget the cut offset relative to the payload is `57 - 17 = 40`; `40 % 4 == 0` (a valid boundary for the 4-byte character — would have been a second self-confirming test), while `40 % 3 == 1` (not a boundary for the 3-byte character). This was caught by actually running the test against the buggy code and observing it pass, per the plan's own prohibition against self-confirming inputs.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected `format_error` test's byte arithmetic mid-task**
- **Found during:** Task 2, while verifying `test_format_error_renders_overlong_multibyte_message`'s pre-fix panic per the plan's own prohibition against self-confirming tests
- **Issue:** The first draft reused Task 1's 30× U+1F6E1 (4-byte) input directly as the `ExecutionError` payload. Running it against the pre-fix code showed the test **passing**, not panicking — the payload's char count (47, including the 17-char `"Execution error: "` prefix) never exceeded the default budget of 60, so truncation never triggered. This was exactly the self-confirming-test failure mode the plan prohibits; it was caught before committing rather than shipped.
- **Fix:** Switched to a 50× repetition of a 3-byte CJK character (中) and recomputed the arithmetic accounting for the 17-byte `thiserror` `Display` prefix, verifying `40 % 3 == 1` is not a char boundary. Confirmed the corrected test panics against the pre-fix code with the expected "not a char boundary" message before restoring the fix.
- **Files modified:** `crates/paladin-herald/src/table_herald.rs`
- **Verification:** Panic confirmed against pre-fix code (see Pre-fix Failure Transitions #2 above); test passes against the fixed code; full `cargo test -p paladin-herald` (70/70) and `cargo test --workspace` (0 failures) re-confirmed green afterward.
- **Committed in:** `617a0bb` (Task 2 commit — the corrected version is what was committed; no separate fix-up commit was needed since this was caught before the first commit of this test)

**2. [Rule 3 - Blocking] Fixed a clippy lint (`useless_borrows_in_formatting`) in the new sweep test**
- **Found during:** Task 2, running `cargo clippy --workspace --all-targets -- -D warnings` after adding the three new tests
- **Issue:** `&input.chars().take(5).collect::<String>()` in an `assert!` format argument triggered `clippy::useless_borrows_in_formatting`, which is `-D`-gated in this project's required pre-commit check
- **Fix:** Removed the redundant `&`
- **Files modified:** `crates/paladin-herald/src/table_herald.rs`
- **Verification:** `cargo clippy --workspace --all-targets -- -D warnings` exits 0; `cargo fmt --check` exits 0
- **Committed in:** `617a0bb` (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (1 test-arithmetic bug caught by the plan's own prohibition, 1 blocking lint)
**Impact on plan:** Both were caught and fixed before the affected commit landed; no scope creep, no change to the implementation the plan specified.

## Issues Encountered

None beyond the deviations above.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- ROADMAP success criterion 3 / GAP-03's blocker is closed: `cargo test -p paladin-herald` (70 passed, 0 failed) and `cargo test --workspace` (all crates green, 0 failed) both confirmed after the fix, and again after Task 2's additions, and again after Task 3 (docs-only, no source change).
- `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` both exit 0 on the final state.
- `02-EDGE-PROBE.md`'s accounting is internally consistent again: 20 accounted against 17 surfaced, row 8 attributed to both `02-04` and `02-10`, engine's 17 emitted rows and §B's 7-prohibition numbering untouched.
- Deferred forward (unchanged from plan, not touched by this execution): `02-REVIEW.md` CR-01 (OpenAI adapter reads wrong prompt field) and WR-01 (name-collision mis-attribution in `format_battalion_result`) both proposed to Phase 3; the two `backstop` items (terminal display width, grapheme-cluster splitting) remain intentionally unaddressed pending a future dependency-audited grapheme/width crate.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*

## Self-Check: PASSED

- FOUND: `crates/paladin-herald/src/table_herald.rs`
- FOUND: `.planning/phases/02-functional-gap-closure/02-EDGE-PROBE.md`
- FOUND: `.planning/phases/02-functional-gap-closure/02-10-SUMMARY.md`
- FOUND commit `53a3074` (Task 1)
- FOUND commit `617a0bb` (Task 2)
- FOUND commit `f43a9ab` (Task 3)
