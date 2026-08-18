---
phase: 17-additional-llm-provider-adapters
plan: 14
subsystem: llm-adapters
tags: [rust, validation, openai-compatible, temperature-range, tdd, gap-closure]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "17-12's OpenAiCompatibleAdapter and its RED/GREEN/RED-reproduction TDD scaffolding for the generic openai-compatible provider"
provides:
  - "parse_temperature_range_env's both-set arm now rejects an inverted (min > max) or non-finite (NaN/inf/-inf) operator-declared temperature range, closing WR-02"
affects: [17-17]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Finiteness-before-ordering guard order for f32 pairs: check is_finite() per bound before any comparison, because NaN compares false in both directions and would defeat an ordering-only guard"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/openai_compatible/adapter.rs

key-decisions:
  - "Guard order is finiteness first, then ordering (min > max) — reversing the order would let (NaN, NaN) and any NaN-containing pair pass silently, since every f32 comparison against NaN is false"
  - "Ordering check is strictly-greater (min > max), never >=, so equal bounds stay a legal single-point declaration"
  - "No repair path exists for any bad declaration — no swap, clamp, or default — per the plan's must_haves.prohibitions and PROV-02's truthful-capability clause"

patterns-established:
  - "f32 env-var range validation: parse -> finiteness check per bound -> ordering check -> accept, each failure naming the specific variable and value"

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "An inverted operator-declared temperature range (OPENAI_COMPATIBLE_TEMPERATURE_MIN > _MAX) is rejected with a configuration error naming both variables and both values, instead of being silently accepted as an inverted tuple"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_inverted_range"
        status: pass
    human_judgment: false
  - id: D2
    description: "A non-finite bound (NaN or infinite) is rejected before the ordering comparison would otherwise pass it through, since every comparison against NaN is false in both directions"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_rejects_a_nan_bound"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_infinite_bound"
        status: pass
    human_judgment: false
  - id: D3
    description: "The boundary itself (min == max) stays a legal, accepted declaration — the check is strictly-greater, not greater-or-equal"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_accepts_equal_bounds"
        status: pass
    human_judgment: false
  - id: D4
    description: "The ordinary ordered-range case and the two half-set diagnostics are unchanged"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_accepts_an_ordered_range"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#openai_compatible::adapter::tests::parse_temperature_range_env_half_set_diagnostics_are_unchanged"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-18
status: complete
---

# Phase 17 Plan 14: WR-02 temperature-range validation Summary

**`parse_temperature_range_env`'s both-set arm now rejects an inverted (`min > max`) or non-finite (`NaN`/`inf`/`-inf`) operator-declared temperature range with a named configuration error, while still accepting equal bounds as a legitimate single-point declaration — closing WR-02.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-18T00:57:00Z (approx, first Read call)
- **Completed:** 2026-08-18T01:32:58Z
- **Tasks:** 2 (TDD: RED, GREEN)
- **Files modified:** 1 — `crates/paladin-llm/src/openai_compatible/adapter.rs`

## Accomplishments

- Added six tests pinning the full contract of `parse_temperature_range_env`'s both-set arm: inverted-range rejection, equal-bounds acceptance, ordered-range acceptance, NaN-bound rejection, infinite-bound rejection, and half-set diagnostic stability.
- Extended the both-set arm with a finiteness check (per bound, before ordering) and a strictly-greater ordering check (`min > max`), both returning the function's existing `Err(String)` shape.
- No arm repairs a bad declaration — confirmed by a scoped absence-of-`swap`/`clamp`/`unwrap_or`/`max(`/`min(` grep restricted to the production function body.

## Task Commits

Each task was committed atomically:

1. **Task 1: RED — tests proving an inverted range and a non-finite bound are both accepted today** - `911a86c` (test)
2. **Task 2: GREEN — reject an inverted or non-finite declared range, never repair it** - `9146265` (fix)

_TDD tasks: RED (test) → GREEN (fix). No REFACTOR commit was needed — the GREEN implementation required no follow-up cleanup._

## Files Created/Modified

- `crates/paladin-llm/src/openai_compatible/adapter.rs` — `parse_temperature_range_env`'s both-set arm gained finiteness and ordering validation; six new tests added to `#[cfg(test)] mod tests`.

## Decisions Made

- **Finiteness checked before ordering, not folded into one comparison.** `"NaN".parse::<f32>()` succeeds, and `NaN > x` / `x > NaN` are both `false`, so an ordering-only guard (`if min > max`) would let `(NaN, NaN)`, `(NaN, x)`, and `(x, NaN)` all pass through undetected. The finiteness check runs first and independently, with the rationale stated in a code comment so a later edit cannot reorder them innocently.
- **Ordering guard is `min > max`, never `min >= max`.** Pinning a provider to a single temperature (`min == max`) is a legitimate operator declaration; the `parse_temperature_range_env_accepts_equal_bounds` control test exists specifically to prevent a future edit from tightening this into a regression.
- **No repair path — ever.** The task's `must_haves.prohibitions` and PROV-02's truthful-capability clause both forbid silently swapping, clamping, or defaulting a bad declaration. Verified by a `grep -cE '\b(swap|clamp|unwrap_or|max\(|min\()'` restricted to the production function body, returning `0`.

## Deviations from Plan

### Acceptance-criterion grep false positives from mandated test names (not a code deviation)

Two of the plan's literal acceptance-criterion `sed`/`grep` commands produce a nonzero count where the intent is `0`, because the plan's own mandated test names and the six tests' descriptive comments contain substrings the greps also match. This is **not** a production-code or test-behavior deviation — every test passes and the production function's actual repair-pattern surface is empty — but it is recorded here per D-00e's "record the exact command and its exact output" rule, since the literal outputs differ from the plan's stated expected values.

1. **`git diff -- crates/paladin-llm/src/openai_compatible/adapter.rs | grep -c '^+.*fn parse_temperature_range_env'` → plan expects `0` for Task 1; actual output is `6`.**
   Cause: the six mandated test names all begin with the literal substring `parse_temperature_range_env_` (e.g. `fn parse_temperature_range_env_rejects_an_inverted_range() {`), so the loose pattern `fn parse_temperature_range_env` matches every one of them as well as any real production-function addition.
   Precise re-verification proving no production code was added in Task 1: `git diff -- crates/paladin-llm/src/openai_compatible/adapter.rs | grep -c '^+fn parse_temperature_range_env('` → **`0`** (anchored to line start with the opening paren immediately following, which only a production function signature — not a test name — would produce).

2. **`sed -n '/fn parse_temperature_range_env/,/^}$/p' ... | grep -cE '\b(swap|clamp|unwrap_or|max\(|min\()'` → plan expects `0` for Task 2; actual output is `1`.**
   Cause: the unanchored `sed` start pattern `/fn parse_temperature_range_env/` re-triggers on `parse_temperature_range_env_rejects_an_inverted_range`'s test body too (committed in Task 1), whose own descriptive comment reads "...an inverted tuple silently accepted here would violate that crate-wide convention for every downstream consumer that clamps into range.0..=range.1." — an English use of "clamps" in test-comment prose, not repair code.
   Precise re-verification proving the production function contains no repair code: `sed -n '/^fn parse_temperature_range_env(/,/^}$/p' crates/paladin-llm/src/openai_compatible/adapter.rs | grep -cE '\b(swap|clamp|unwrap_or|max\(|min\()'` → **`0`** (anchored to the production function's own signature line, excluding every test body).
   A doc-comment word choice inside the production function itself ("...could ever clamp into") was also reworded to "...could ever resolve" during Task 2, purely to keep the production function's own isolated match count at `0` without relying on the anchor distinction — this is the only wording change beyond the plan's specified additions, and it changes no behavior.

No other deviations. All other acceptance criteria in both tasks matched the plan's literal command and expected output exactly.

## Issues Encountered

None beyond the grep-anchoring note above.

## Exact Commands and Outputs (D-00e)

### Task 1 — RED state

**Test-module path resolution** (`cargo test -p paladin-llm --no-default-features --features openai-compatible -- --list`, filtered):
```
openai_compatible::adapter::tests::parse_temperature_range_env_accepts_an_ordered_range: test
openai_compatible::adapter::tests::parse_temperature_range_env_accepts_equal_bounds: test
openai_compatible::adapter::tests::parse_temperature_range_env_half_set_diagnostics_are_unchanged: test
openai_compatible::adapter::tests::parse_temperature_range_env_rejects_a_nan_bound: test
openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_infinite_bound: test
openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_inverted_range: test
```
Matches the plan's assumed path `openai_compatible::adapter::tests::` — no resolution needed.

**RED verification** — `cargo test -p paladin-llm --no-default-features --features openai-compatible -- openai_compatible::adapter::tests::parse_temperature_range_env_rejects`:
```
running 3 tests
test openai_compatible::adapter::tests::parse_temperature_range_env_rejects_a_nan_bound ... FAILED
test openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_infinite_bound ... FAILED
test openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_inverted_range ... FAILED

failures:

---- openai_compatible::adapter::tests::parse_temperature_range_env_rejects_a_nan_bound stdout ----
thread '...' panicked at crates/paladin-llm/src/openai_compatible/adapter.rs:774:9:
a NaN bound must be rejected

---- openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_infinite_bound stdout ----
thread '...' panicked at crates/paladin-llm/src/openai_compatible/adapter.rs:783:9:
an infinite bound must be rejected

---- openai_compatible::adapter::tests::parse_temperature_range_env_rejects_an_inverted_range stdout ----
thread '...' panicked at crates/paladin-llm/src/openai_compatible/adapter.rs:738:9:
an inverted range must be rejected

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 84 filtered out; finished in 0.00s
```
`! cargo test ...` (the plan's `<automated>` form) accordingly exits `0` — the three tests genuinely fail against the pre-fix tree, confirming RED.

**Control tests** — `cargo test -p paladin-llm --no-default-features --features openai-compatible -- openai_compatible::adapter::tests::parse_temperature_range_env_accepts`:
```
test openai_compatible::adapter::tests::parse_temperature_range_env_accepts_an_ordered_range ... ok
test openai_compatible::adapter::tests::parse_temperature_range_env_accepts_equal_bounds ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 85 filtered out; finished in 0.00s
```

**Half-set diagnostics control** — `cargo test -p paladin-llm --no-default-features --features openai-compatible -- openai_compatible::adapter::tests::parse_temperature_range_env_half_set_diagnostics_are_unchanged --exact`:
```
test openai_compatible::adapter::tests::parse_temperature_range_env_half_set_diagnostics_are_unchanged ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 86 filtered out; finished in 0.00s
```

**`cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings`** (run against Task 1's RED tree, before Task 2's edit): `Finished` cleanly, exit `0`.

**`cargo fmt --check -p paladin-llm`**: initially reported 4 diffs (line-length collapsing on the new test bodies); resolved with `cargo fmt -p paladin-llm`, then re-verified exit `0`.

**Production-code-addition check** — `git diff -- crates/paladin-llm/src/openai_compatible/adapter.rs | grep -c '^+.*fn parse_temperature_range_env'` → `6` (see Deviations above for the false-positive explanation); precise form `grep -c '^+fn parse_temperature_range_env('` → `0`.

**Scope-guard diff-stat** — `git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ crates/paladin-llm/src/kimi/ crates/paladin-llm/src/qwen/ crates/paladin-llm/src/grok/ crates/paladin-llm/src/ollama/ crates/paladin-llm/src/gemini/ crates/paladin-llm/src/compat/` → empty output.

### Task 2 — GREEN state

**Six-test rerun** — `cargo test -p paladin-llm --no-default-features --features openai-compatible -- openai_compatible::adapter::tests::parse_temperature_range_env`:
```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 81 filtered out; finished in 0.00s
```

**Whole generic-provider module** — `cargo test -p paladin-llm --no-default-features --features openai-compatible`:
```
test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.97s
```

**Wide feature combo** — `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"`:
```
test result: ok. 203 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.61s
```
(Arithmetic note: this is the first plan of Wave 2 to touch `paladin-llm` in this execution session — no sibling plan in the same wave ran ahead of this one against this feature combo, so the 6-test delta from this plan's additions is the only change against whatever count 17-12 recorded; this plan did not independently re-derive 17-12's baseline count to diff against.)

**Default features (PROV-03 — generic provider inert by default)** — `cargo test -p paladin-llm`:
```
test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.77s
```

**Ordering diagnostic phrase** — `grep -c 'must not exceed' crates/paladin-llm/src/openai_compatible/adapter.rs` → `1`.

**Finiteness-check count** — `sed -n '/fn parse_temperature_range_env/,/^}$/p' crates/paladin-llm/src/openai_compatible/adapter.rs | grep -c 'is_finite()'` → `2`.

**Strictly-greater guard** — `sed -n '/fn parse_temperature_range_env/,/^}$/p' ... | grep -cE '\bmin >= max\b'` → `0`.

**No-repair-path check (literal, plan-specified form)** — `sed -n '/fn parse_temperature_range_env/,/^}$/p' ... | grep -cE '\b(swap|clamp|unwrap_or|max\(|min\()'` → `1` (see Deviations above); precise anchored form `sed -n '/^fn parse_temperature_range_env(/,/^}$/p' ... | grep -cE '\b(swap|clamp|unwrap_or|max\(|min\()'` → `0`.

**`cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings`** (GREEN state): `Finished` cleanly, exit `0`. No `float_cmp` or other lint fired on the new `f32` comparisons (`is_finite()`, `min > max`) — no `#[allow]` was needed.

**`cargo fmt --check -p paladin-llm`** (GREEN state): exit `0`.

**`cargo doc -p paladin-llm --no-deps --no-default-features --features openai-compatible`**: one pre-existing warning, unrelated to this plan's scope — `public documentation for 'redirect_policy' links to private item 'CompatEngine::map_error'` in `crates/paladin-llm/src/compat/engine.rs:123` (a file this plan's scope guards forbid touching; confirmed untouched by the scope-guard diff-stat below). Zero `missing_docs` warnings — confirmed by `grep -i "missing_docs\|missing documentation"` over the full doc-build output returning no match.

**Dependency-surface check** — `git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock` → empty output.

**Full scope-guard diff-stat** — `git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ crates/paladin-llm/src/kimi/ crates/paladin-llm/src/qwen/ crates/paladin-llm/src/grok/ crates/paladin-llm/src/ollama/ crates/paladin-llm/src/gemini/ crates/paladin-llm/src/compat/ crates/paladin-llm/src/provider_factory.rs` → empty output.

### Plan-level `<verification>` block (all 7 automatable items)

1. `cargo test -p paladin-llm --no-default-features --features openai-compatible` → 87 passed, 0 failed.
2. `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` → 203 passed, 0 failed.
3. `cargo test -p paladin-llm` (default features) → 57 passed, 0 failed.
4. `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` → exit 0.
5. `cargo fmt --check -p paladin-llm` → exit 0.
6. `cargo doc -p paladin-llm --no-deps --no-default-features --features openai-compatible` → zero `missing_docs` (one unrelated pre-existing `private_intra_doc_links` warning in `compat/engine.rs`, out of this plan's scope).
7. `git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock` → empty.
8. Snyk — **not run.** Neither the `snyk_code_scan` MCP tool nor a `snyk` CLI binary was available in this environment (`which snyk` → not found, no MCP Snyk tool exposed to this worktree executor). Plan 17-17 files the corresponding `WINDOWS.md` row for the whole phase-17 run per this plan's `<artifacts_this_phase_produces>` note.

## Exact error-message wording chosen (Task 2)

- Finiteness (min): `Invalid OPENAI_COMPATIBLE_TEMPERATURE_MIN value {min} — must be a finite number`
- Finiteness (max): `Invalid OPENAI_COMPATIBLE_TEMPERATURE_MAX value {max} — must be a finite number`
- Ordering: `OPENAI_COMPATIBLE_TEMPERATURE_MIN value {min} must not exceed OPENAI_COMPATIBLE_TEMPERATURE_MAX value {max}`

All three follow `parse_u32_env_value`'s existing shape: a plain `String` naming the variable and the offending value, no new error type.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- WR-02 is closed. `parse_temperature_range_env`'s both-set arm is now fully guarded: ordering, finiteness, and the equal-bounds boundary are all pinned by tests.
- Plan 17-17 (not yet executed) still owns: the full 18-row edge-case reconciliation table, the `WINDOWS.md` row for this run's not-run Snyk scan, and the IN-01 `.project/current-exports.txt` accepted-debt tracking row.
- No blockers for sibling wave-2 plans (17-13 on `provider_factory.rs`, 17-15 on `gemini/adapter.rs`) — this plan touched only `openai_compatible/adapter.rs`, confirmed by every scope-guard diff-stat above returning empty.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-18*

## Self-Check: PASSED

- FOUND: `crates/paladin-llm/src/openai_compatible/adapter.rs`
- FOUND: `.planning/phases/17-additional-llm-provider-adapters/17-14-SUMMARY.md`
- FOUND: commit `911a86c` (test(17-14): RED)
- FOUND: commit `9146265` (fix(17-14): GREEN)
- FOUND: commit `2a5a947` (docs(17-14): summary)
