---
phase: 17-additional-llm-provider-adapters
plan: 12
subsystem: llm
tags: [rust, provider-factory, tdd, credential-validation, cargo-test, cr-01]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: the nine-row provider_factory registry (D-10), the six new adapter presets (kimi/qwen/grok/gemini/ollama/openai-compatible) and 17-REVIEW.md's CR-01 finding
provides:
  - a non-blank credential check (is_ok_and(|v| !v.trim().is_empty())) at both get_default_provider() and list_available_providers(), matching the crate's own provider_name_round_trip test
  - a single ten-variable CleanProviderEnv guard replacing the two-lock CleanProviderEnv/CleanNewProviderEnv split
  - a registry-declaration-order contract pinned by two new characterisation tests
affects: [17-13, 17-14, 17-15, 17-16, 17-17]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "single merged test-environment guard over one lock, built from a const variable list, instead of per-feature-group guards"
    - "runtime-derived (provider_names()-based) test tables instead of hardcoded provider-count assertions, so tests hold under any feature combination"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/provider_factory.rs
    - tests/unit/llm/provider_factory_test.rs

key-decisions:
  - "Assumption-delta 'promote': replaced the two-guard (CleanProviderEnv/CleanNewProviderEnv) model with one guard over one lock covering all ten registry-read env vars, because get_default_provider()/list_available_providers() scan the whole registry and the two variable sets were never actually disjoint from those functions' point of view — this disjointness assumption is exactly what let CR-01 hide behind a passing test suite"

patterns-established:
  - "CleanProviderEnv::set(var, value) with a debug_assert! against a const REGISTRY_ENV_VARS list, carried forward from the deleted CleanNewProviderEnv, as the one way tests mutate registry-read env vars"

requirements-completed: [PROV-03, PROV-04]

coverage:
  - id: D1
    description: "get_default_provider() and list_available_providers() treat a credential env var set to the empty string, ASCII whitespace, or non-ASCII Unicode whitespace (U+00A0) as absent — CR-01 closed"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#an_empty_credential_env_var_is_not_a_configured_provider"
        status: pass
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#a_whitespace_only_credential_env_var_is_not_a_configured_provider"
        status: pass
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider"
        status: pass
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#a_real_credential_env_var_is_a_configured_provider"
        status: pass
    human_judgment: false
  - id: D2
    description: "Registry declaration order is a tested contract: list_available_providers() is a same-order subsequence of provider_names(), and get_default_provider()'s tie-break follows declaration order"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#list_available_providers_preserves_registry_declaration_order"
        status: pass
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#get_default_provider_breaks_ties_by_declaration_order"
        status: pass
    human_judgment: false
  - id: D3
    description: "Merged CleanProviderEnv guard is concurrency-safe: identical pass counts under --test-threads=1 and default thread parallelism, across both the root default feature set and --features llm-all"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test --test unit --features llm-all -- provider_factory --test-threads=1 (17 passed) vs cargo test --test unit --features llm-all -- provider_factory default parallelism (17 passed)"
        status: pass
    human_judgment: false

# Metrics
duration: ~45min
completed: 2026-08-18
status: complete
---

# Phase 17 Plan 12: CR-01 Blank-Credential Tracer Summary

**Closed CR-01 by changing `get_default_provider()`/`list_available_providers()`'s credential-presence check from `std::env::var(var).is_ok()` to `is_ok_and(|v| !v.trim().is_empty())`, and separately merged two test-environment guards into one ten-variable `CleanProviderEnv` so the reviewer's own reproduction test runs green for the right reason.**

## Performance

- **Duration:** ~45 min
- **Started:** 2026-08-18T00:39:29Z (per STATE.md `stopped_at`)
- **Completed:** 2026-08-18T01:11:12Z
- **Tasks:** 3
- **Files modified:** 2 (`crates/paladin-llm/src/provider_factory.rs`, `tests/unit/llm/provider_factory_test.rs`)

## Accomplishments

- **CR-01 closed at both production call sites.** `get_default_provider()` and `list_available_providers()` now use `std::env::var(var).is_ok_and(|v| !v.trim().is_empty())`, matching the crate's own `provider_name_round_trip` test's pre-existing check. A provider whose credential variable is present-but-blank is never reported as available; `create()` and the availability report now always agree.
- **Test-environment guard consolidated (assumption-delta `promote`).** `CleanProviderEnv` and `CleanNewProviderEnv` (two locks over two "disjoint" variable sets) are merged into one `CleanProviderEnv` over one `PROVIDER_ENV_LOCK`, holding all ten variables the nine-row registry reads (`REGISTRY_ENV_VARS`). The disjointness the old model assumed was false for the two functions under test — they scan the whole registry — and that false assumption is precisely what let CR-01's bug ship with a passing suite.
- **Registry declaration order pinned as an executable contract.** Two new tests assert `list_available_providers()` is an exact, same-order subsequence of `provider_names()`, and that `get_default_provider()`'s tie-break follows declaration order (never pre-empted by Ollama's credential-free row).
- **The two legacy reviewer-reproduction tests made registry-aware**, so they pass under both the root default feature set (3 providers) and `--features llm-all` (9 rows, including the credential-free `ollama` row that made the old hardcoded assertions arithmetically unsatisfiable).

## Task Commits

Each task was committed atomically, all with `git commit --no-verify` per D-00o (worktree cold-compile avoidance):

1. **Task 1: RED — merged guard, registry-aware legacy tests, four CR-01 regression tests** - `2e50877` (test)
2. **Task 2: GREEN — non-blank credential check at both production call sites** - `9ef01dc` (fix)
3. **Task 3: declaration-order stability and parallel-safety pins** - `d15b8b9` (test)

_TDD tasks: Task 1 (RED, test-only) → Task 2 (GREEN, fix) → Task 3 (additional characterisation tests, test-only)._

## Files Created/Modified

- `crates/paladin-llm/src/provider_factory.rs` — `get_default_provider()` and `list_available_providers()`'s credential-presence check changed from `.is_ok()` to `.is_ok_and(|v| !v.trim().is_empty())`; both doc comments updated to state the semantics; `provider_name_round_trip`'s stale "deliberately stricter than production" rationale comment corrected to "uses the same check". No other line touched.
- `tests/unit/llm/provider_factory_test.rs` — `CleanProviderEnv`/`CleanNewProviderEnv` merged into one guard (`REGISTRY_ENV_VARS: [&str; 10]`, `saved: Vec<(&'static str, Option<String>)>`); added `CREDENTIALED_PROVIDERS`, `credential_free_baseline()`, `first_compiled_credentialed_provider()`; rewrote `test_get_default_provider`/`test_list_available_providers` to be registry-aware; added six new tests (four CR-01 regression tests in Task 1, two declaration-order-stability tests in Task 3); repointed the two `CleanNewProviderEnv`-using tests at the merged guard.

## Decisions Made

- **Assumption-delta `promote`** (recorded in the plan itself, executed as written): one guard/one lock over all ten registry-read variables replaces the two-guard model. Accepted debt: the guard's variable list (`REGISTRY_ENV_VARS`) is hand-maintained against the registry; a tenth provider added without extending it reintroduces a narrower version of the same blind spot. No further action taken this plan — recorded here per the plan's own instruction, not silently absorbed.

## Deviations from Plan

None — plan executed exactly as written. One planned-for reconciliation, called out below because the plan explicitly anticipated and permitted it.

### Reconciliation (anticipated by the plan's own acceptance criteria)

**`grep -c 'CleanProviderEnv::acquire()' tests/unit/llm/provider_factory_test.rs` after Task 1: 13, not the plan's estimated 8.**

The plan's acceptance criterion explicitly allows reconciling this figure against `grep -n '#\[test\]'` if it differs, "record the reconciliation" — doing so here. Breakdown of the 13 matches:

- 12 real call sites: `test_factory_provider_selection`, `test_factory_config_validation`, `test_factory_case_insensitive`, `test_get_default_provider`, `test_list_available_providers`, `default_features_still_resolve_openai_anthropic_and_deepseek` (6 pre-existing) + `test_compiled_out_provider_absent_from_list_available_providers`, `test_new_provider_names_resolve_through_create` (2 repointed from the deleted `CleanNewProviderEnv`) + `an_empty_credential_env_var_is_not_a_configured_provider`, `a_whitespace_only_credential_env_var_is_not_a_configured_provider`, `a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider`, `a_real_credential_env_var_is_a_configured_provider` (4 new CR-01 tests, Task 1's own centrepiece).
- 1 prose reference: `test_get_default_provider`'s doc comment ("`CleanProviderEnv::acquire()` takes `PROVIDER_ENV_LOCK`...") — a comment, not a call site.

The plan's estimate of 8 counted only the pre-existing-plus-repointed sites and did not anticipate that its own Task 1 action item 4 ("Each acquires the merged guard...") would add four more call sites in the same commit. By the end of Task 3 the true count is 15 (13 + 2 more real call sites from `list_available_providers_preserves_registry_declaration_order` and `get_default_provider_breaks_ties_by_declaration_order`), confirmed via direct grep after the final commit.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Verification — D-00e (exact commands and exact outputs)

### Test-path prefix resolution

`cargo test --test unit --features llm-all -- --list 2>&1 | grep provider_factory` resolved every test in this file to the prefix `llm::provider_factory_test::` (confirming the plan's assumed prefix, no reconciliation needed):

```
llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider: test
llm::provider_factory_test::a_real_credential_env_var_is_a_configured_provider: test
llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider: test
llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider: test
llm::provider_factory_test::default_features_still_resolve_openai_anthropic_and_deepseek: test
llm::provider_factory_test::test_compiled_out_provider_absent_from_list_available_providers: test
... (15 total at end of Task 1)
```

### Task 1 RED-state verification

**Reviewer's own reproduction, green after the guard merge alone (the symptom, not the defect):**

```
$ cargo test --test unit --features llm-all -- llm::provider_factory_test::test_get_default_provider llm::provider_factory_test::test_list_available_providers --test-threads=1

running 2 tests
test llm::provider_factory_test::test_get_default_provider ... ok
test llm::provider_factory_test::test_list_available_providers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 435 filtered out; finished in 0.00s
```

**The four CR-01 tests, RED as designed — verbatim failure messages:**

```
$ cargo test --test unit --features llm-all -- credential_env_var --test-threads=1

running 4 tests
test llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider ... FAILED
test llm::provider_factory_test::a_real_credential_env_var_is_a_configured_provider ... ok
test llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider ... FAILED
test llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider ... FAILED

failures:

---- llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider stdout ----

thread 'llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider' (43906) panicked at tests/unit/llm/provider_factory_test.rs:670:5:
openai must be absent from list_available_providers() when its credential variable (OPENAI_API_KEY) is a single U+00A0 NO-BREAK SPACE
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider stdout ----

thread 'llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider' (43908) panicked at tests/unit/llm/provider_factory_test.rs:632:5:
openai must be absent from list_available_providers() when its credential variable (OPENAI_API_KEY) is set to ASCII whitespace only

---- llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider stdout ----

thread 'llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider' (43909) panicked at tests/unit/llm/provider_factory_test.rs:597:5:
openai must be absent from list_available_providers() when its credential variable (OPENAI_API_KEY) is set to the empty string


failures:
    llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider
    llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider
    llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider

test result: FAILED. 1 passed; 3 failed; 0 ignored; 0 measured; 433 filtered out; finished in 0.00s
```

The three failures name `openai` — `first_compiled_credentialed_provider()` resolved to `("openai", "OPENAI_API_KEY")` in this build (openai is compiled under both feature sets exercised and is first in `CREDENTIALED_PROVIDERS`/registry order).

**Root default feature set, same three failures, no others:**

```
$ cargo test --test unit -- provider_factory --test-threads=1

running 15 tests
... (12 ok)
test llm::provider_factory_test::a_non_ascii_whitespace_credential_env_var_is_not_a_configured_provider ... FAILED
test llm::provider_factory_test::a_whitespace_only_credential_env_var_is_not_a_configured_provider ... FAILED
test llm::provider_factory_test::an_empty_credential_env_var_is_not_a_configured_provider ... FAILED

test result: FAILED. 12 passed; 3 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.01s
```

**Task 1 static checks:**

| Check | Expected | Actual |
|---|---|---|
| `grep -c 'CleanNewProviderEnv'` | 0 | 0 |
| `grep -c 'NEW_PROVIDER_ENV_LOCK'` | 0 | 0 |
| `grep -c 'CleanProviderEnv::acquire()'` | 8 (plan estimate) | 13 (reconciled above) |
| `grep -c 'OPENAI_COMPATIBLE_MODEL'` | ≥ 2 | 3 |
| `cargo fmt --check -p paladin-llm` | exit 0 | exit 0 |
| `git diff --stat -- openai/ anthropic/ deepseek/` | empty | empty |
| `git diff --stat -- provider_factory.rs` | empty (test-only task) | empty |

### Task 2 GREEN-state verification

```
$ cargo test --test unit --features llm-all -- provider_factory --test-threads=1
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.66s

$ cargo test --test unit -- provider_factory --test-threads=1   (root default feature set)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.00s

$ cargo test --test unit --features llm-all -- provider_factory   (default thread parallelism)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.72s
```

Serial (`--test-threads=1`) and default-parallelism pass counts are identical (15 = 15) — the concurrency criterion.

**Predicate checks:**

| Check | Expected | Actual |
|---|---|---|
| `grep -c 'is_ok_and(\|v\| !v.trim().is_empty())' provider_factory.rs` | 3 | 3 |
| `sed` scoped to `impl LlmProviderFactory` block, `grep -c 'is_ok()'` | 0 | 0 |
| `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` | 0 failed, baseline 197 | **197 passed; 0 failed** — matches `17-VERIFICATION.md`'s recorded baseline exactly, no regression |
| `cargo test -p paladin-llm` (default features) | 0 failed | 57 passed; 0 failed |
| `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` | exit 0 | exit 0 |
| `cargo fmt --check -p paladin-llm` | exit 0 | exit 0 |
| `cargo doc -p paladin-llm --no-deps --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` | zero `missing_docs`, only pre-existing `rustdoc::private_intra_doc_links` | 3 warnings, all `rustdoc::private_intra_doc_links` (`compat/engine.rs:123`, `gemini/adapter.rs:28`, `gemini/adapter.rs:59`), zero `missing_docs` |
| `git diff --stat -- Cargo.toml Cargo.lock paladin-llm/Cargo.toml` | empty | empty |
| `git diff --stat -- openai/ anthropic/ deepseek/` | empty | empty |

### Task 3 verification

```
$ cargo test --test unit --features llm-all -- provider_factory --test-threads=1
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.90s   (Task 2's 15 + 2)

$ cargo test --test unit --features llm-all -- provider_factory   (default parallelism)
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.82s   (identical to serialised run)

$ cargo test --test unit -- provider_factory --test-threads=1   (root default feature set, 3 credentialed providers compiled — no skip note triggered)
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 422 filtered out; finished in 0.01s
```

| Check | Expected | Actual |
|---|---|---|
| `grep -c 'declaration order'` | ≥ 3 | 15 |
| `cargo clippy` (six-preset, `-D warnings`) | exit 0 | exit 0 |
| `cargo fmt --check -p paladin-llm` | exit 0 | exit 0 |
| `git diff --stat -- crates/paladin-llm/src/` (this task's commit) | empty | empty |

### Full-plan reconciliation (post-Task-3, against base commit `cdd74e7`)

```
$ git diff --stat cdd74e7..HEAD
 crates/paladin-llm/src/provider_factory.rs |  37 +-
 tests/unit/llm/provider_factory_test.rs    | 662 ++++++++++++++++++++---------
 2 files changed, 490 insertions(+), 209 deletions(-)

$ git diff --stat cdd74e7..HEAD -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock
(empty)
```

Exactly the two files the plan authorizes were touched across all three tasks; D-06 held for every commit.

### Snyk — NOT RUN

Neither the `snyk_code_scan` MCP tool nor a `snyk` CLI binary is available in this worktree (`command -v snyk` returns nothing). Per the executor notes, this is recorded as **not run**, not as passed. Plan 17-17 files the corresponding `WINDOWS.md` row for the whole gap-closure run; no row is filed by this plan.

## Why the two reviewer-named tests going green did NOT mean CR-01 was fixed

Recorded explicitly per the plan's own instruction, because a future reader will otherwise mis-attribute the fix: **Task 1's guard merge alone turned `test_get_default_provider` and `test_list_available_providers` green** (2 passed, 0 failed, immediately after the merge, with zero production-code changes). That was the test-environment cause — the old `CleanProviderEnv` only cleared three variables while the functions under test scanned the full nine-row registry, so ambient empty `GEMINI_API_KEY`/`XAI_API_KEY` leaked through and the two legacy tests' hardcoded three-provider assertions were unreachable under `--features llm-all` regardless. **Merging the guards fixed the test failure, not the defect.** It was the four new CR-01 tests added in the same Task 1 commit — three of which failed against the unmodified production code — that isolated and proved the actual defect: a blank credential variable was still reported as configured. Task 2's one-line predicate change at both call sites is what closed CR-01 itself.

## Next Phase Readiness

The RED → GREEN → RED-reproduction loop is proven end to end: a failing test that isolates the exact defect, a minimal production fix that turns it green without touching `create()` or the three shipped adapters, and two follow-on characterisation tests pinning the ordering/concurrency properties the fix depends on. Plans 17-13 through 17-16 (the four review Warnings) may now expand on this pattern. 17-13 depends on this plan and owns `create()`'s missing `openai_compatible` underscore alias (WR-01) in the same file — deliberately untouched here.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-18*
