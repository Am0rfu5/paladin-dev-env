---
phase: 02-functional-gap-closure
plan: 06
subsystem: testing
tags: [mockito, deepseek, anthropic, provider-factory, unsafe-env-var, tokio-test]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-01's measured cargo test --workspace baseline (2790 passed / 0 failed / 126
      ignored on commit 7e55655) — the pre-change tree this plan's own full-suite runs are
      compared against"
  - phase: 02-functional-gap-closure
    provides: "02-02's ProviderCapabilities.temperature_range field (DeepSeek Some((0.0, 2.0)),
      the other adapters Some((0.0, 1.0)) or None) — the capability discriminator plan 02-06's
      provider-switching test uses to prove a switch took effect"
provides:
  - "`tests/unit/llm/`'s 25 test functions (8 DeepSeek + 9 Anthropic + 8 Factory) compiling and
    running for the first time — `tests/unit/mod.rs` was missing the single `pub mod llm;` line
    that connected them to the `unit` [[test]] target"
  - "The 401/429/timeout/streaming/malformed-response HTTP-level failure paths for DeepSeek and
    Anthropic now exercised by tests that actually execute, closing a gap the 67 live
    `paladin-llm` tests did not reach"
  - "`tests/integration/provider_switching_test.rs` — Epic 6 task 7.10's provider-switching test,
    which existed nowhere in the tree under any name before this plan"
  - "A CleanProviderEnv RAII guard pattern in provider_factory_test.rs (Mutex-serialized,
    panic-safe env-var save/clear/restore) available as a reusable precedent for any future test
    that mutates process-wide provider API-key env vars"
affects: [02-09-amend-ledger, phase-3-qual-work]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "RAII env-var guard: a Mutex-backed struct that saves/clears three process-wide env vars on
      construction and restores them (present-or-absent) on Drop, serializing every test in a file
      that touches shared process state and surviving panics — see provider_factory_test.rs's
      CleanProviderEnv"
    - "Raw TCP listener (bind, never accept()) to force a genuine client-side request timeout in a
      test, where a local mockito server would instead respond immediately (with 501) and never
      exercise the timeout path at all"

key-files:
  created:
    - tests/integration/provider_switching_test.rs
  modified:
    - tests/unit/mod.rs
    - tests/unit/llm/mod.rs
    - tests/unit/llm/deepseek_adapter_test.rs
    - tests/unit/llm/anthropic_adapter_test.rs
    - tests/unit/llm/provider_factory_test.rs
    - tests/integration/mod.rs

key-decisions:
  - "Split the plan's three tasks into three atomic commits exactly as structured: Task 1 (compile
    fixes, 465ecdb), Task 2 (runtime fixes, 7c3d4b7), Task 3 (new provider-switching test, 7257f88)."
  - "Imported DeepSeekAdapter/DeepSeekConfig and AnthropicAdapter/AnthropicConfig directly from
    paladin_llm::{deepseek,anthropic} rather than through the paladin facade crate's re-export,
    because the facade's re-export is gated behind the root paladin-ai package's own
    llm-deepseek/llm-anthropic features (off by default), while paladin-llm itself always compiles
    all three adapters plus mock (root Cargo.toml's dependency line: features = [\"openai\",
    \"anthropic\", \"deepseek\", \"mock\", \"vision\"]) — so importing from paladin_llm directly
    keeps every reactivated test and the new provider-switching test running under the default
    feature set with no #[cfg(feature = ...)] guard."
  - "For the provider-switching test, chose MockLlmAdapter (in-crate mock) + a real DeepSeekAdapter
    pointed at a local mockito server, rather than driving two adapters through
    LlmProviderFactory::create() with faked env vars. The factory path would have required setting
    OPENAI_API_KEY/DEEPSEEK_API_KEY plus *_BASE_URL env vars, reintroducing exactly the
    process-wide env-var interference risk (T-02-23) provider_factory_test.rs's own CleanProviderEnv
    guard exists to neutralise — the mock+real-adapter-via-local-server design gets the same 'two
    genuinely distinct LlmPort implementations' proof with zero env-var surface."

patterns-established:
  - "CleanProviderEnv RAII guard for process-wide provider-key env var tests: acquire a Mutex,
    snapshot + clear on construction, restore on Drop (including on panic/unwind)."

requirements-completed: [GAP-01, GAP-02]

coverage:
  - id: D1
    description: "tests/unit/mod.rs declares `pub mod llm;`, wiring the 25 never-compiled LLM
      unit-test functions into the `unit` [[test]] target for the first time"
    verification:
      - kind: unit
        ref: "cargo test --test unit --no-run"
        status: pass
      - kind: other
        ref: "grep -c 'pub mod llm;' tests/unit/mod.rs → 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "All 25 reactivated LLM unit tests (8 DeepSeek + 9 Anthropic + 8 Factory) pass at
      runtime with 0 ignored, covering the 401, 429, timeout, streaming and malformed-response
      HTTP-level failure paths"
    requirement: "GAP-01"
    verification:
      - kind: unit
        ref: "cargo test --test unit -- llm → 41 passed (25 target + 16 pre-existing name matches),
          0 failed, 0 ignored"
        status: pass
    human_judgment: false
  - id: D3
    description: "Every set_var/remove_var call in provider_factory_test.rs is wrapped in its own
      unsafe block with a call-site SAFETY comment; process-wide env-var interference across tests
      in the same file is resolved by a Mutex-serialized, panic-safe restore-on-Drop guard rather
      than by #[ignore]"
    verification:
      - kind: other
        ref: "grep -c 'unsafe {' == grep -cE 'env::(set_var|remove_var)' == 24 in
          tests/unit/llm/provider_factory_test.rs (Task 1); CleanProviderEnv guard added in Task 2"
        status: pass
      - kind: unit
        ref: "cargo test --test unit -- llm::provider_factory_test → 8 passed, 0 failed"
        status: pass
    human_judgment: false
  - id: D4
    description: "tests/integration/provider_switching_test.rs exists, declared in
      tests/integration/mod.rs with no feature guard, runs offline (no API key, no live provider
      URL), proves a runtime provider switch preserves the request/response contract and that the
      two providers' capabilities genuinely differ, and covers the unknown-provider typed-error
      path"
    requirement: "GAP-02"
    verification:
      - kind: integration
        ref: "cargo test --test lib -- integration::provider_switching_test → 2 passed, 0 failed"
        status: pass
      - kind: other
        ref: "grep -c 'pub mod provider_switching_test' tests/integration/mod.rs → 1; file is 150
          lines (≥60 required); no live provider base URL or provider-key env read in the file"
        status: pass
    human_judgment: false
  - id: D5
    description: "Full workspace suite stays green: cargo test --workspace, cargo clippy
      --workspace --all-targets --all-features -- -D warnings, cargo fmt --all -- --check"
    verification:
      - kind: other
        ref: "cargo test --workspace (run after each of the 3 task commits) — all 35 test-result
          groups 0 failed each time"
        status: pass
      - kind: other
        ref: "cargo clippy --workspace --all-targets --all-features -- -D warnings"
        status: pass
      - kind: other
        ref: "cargo fmt --all -- --check"
        status: pass
    human_judgment: false

duration: ~65min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 06: LLM Test Reactivation and Provider-Switching Test Summary

**Wired 25 never-compiled `tests/unit/llm/` test functions into the `unit` test binary for the first time, fixed both their compile-time and runtime breakage without deleting a single test, and wrote the provider-switching integration test Epic 6 task 7.10 named but never produced**

## Performance

- **Duration:** ~65 min
- **Started:** 2026-08-01T01:15:00Z (approximate — base commit `91b3033`)
- **Completed:** 2026-08-01T01:43:30Z (Task 3 commit `7257f88`)
- **Tasks:** 3 (all `type="auto"`, Task 3 also `tdd="true"`)
- **Files modified:** 6 modified, 1 created

## Accomplishments

- Added the single missing `pub mod llm;` line to `tests/unit/mod.rs`, connecting
  `tests/unit/llm/`'s 25 test functions (8 DeepSeek + 9 Anthropic + 8 Factory — not the 27 the
  task list claimed) to the `unit` `[[test]]` target for the first time since the directory was
  written.
- Repaired every mechanical compile-time breakage the `paladin-ports` extraction and facade
  rewiring left behind: `LlmRequest`'s current six-field shape, `PromptItem::new`'s current
  `Result`-returning `PromptType`-only signature, `SystemPrompt`/`UserPrompt`'s current field sets
  (no `examples`, `UserPrompt` now requires `query`), and `DeepSeekConfig`/`AnthropicConfig`'s
  current fields (`model` added, `max_retries` removed, `max_tokens` added for Anthropic).
  **Zero tests were deleted** — every one of the 25 was mechanically repairable; no structural
  breakage was found in any of the three files.
- Fixed the two known runtime hazards research flagged: mockito's blocking `Server::new()`
  panicking with a runtime-nesting error inside `#[tokio::test]` (converted both adapter test
  files' `setup_mock_server()` helpers to `async fn` using `Server::new_async().await`, and every
  `.create()` to `.create_async().await`), and process-wide environment-variable interference
  between `provider_factory_test.rs`'s tests (added a `CleanProviderEnv` RAII guard, Mutex-backed,
  restore-on-drop including on panic).
- Discovered and fixed a runtime-only issue neither the plan nor the research anticipated: this
  sandbox predefines `OPENAI_API_KEY`/`DEEPSEEK_API_KEY`/`ANTHROPIC_API_KEY` as empty-but-set
  strings, which made `ConfigurationMissing`-expecting assertions fail against a polluted rather
  than clean environment. `CleanProviderEnv` neutralises this for every test that needs a clean
  environment, not just the two that mutate the vars mid-test.
- Discovered and fixed a second runtime-only issue: `test_deepseek_timeout`'s assumption that a
  mockito server left unmocked would hang was wrong — mockito responds immediately with 501 to
  any unmatched request — so the test never actually timed out and instead hit a different
  `LlmError` variant entirely. Rewrote it to bind a raw TCP listener that never `accept()`s,
  which completes the TCP handshake but produces no HTTP response, forcing a real client-side
  timeout within the configured 1-second budget.
- Wrote `tests/integration/provider_switching_test.rs` — Epic 6 task 7.10's provider-switching
  test, confirmed absent from the tree under any name before this plan — proving two distinct
  `LlmPort` implementations can be selected at runtime behind the same `Arc<dyn LlmPort>`, that
  the switch preserves the request/response contract, that the two providers' capabilities
  genuinely differ (`temperature_range` — `None` vs `Some((0.0, 2.0))`, the discriminator plan
  02-02 made available), and that an unknown provider name returns a typed
  `ProviderFactoryError::UnknownProvider` rather than panicking or silently defaulting. Runs fully
  offline, no provider feature flag required, no `.github/workflows/` file touched.
- `cargo test --workspace` stayed green (0 failed across all 35 test-result groups) after every
  task; `cargo clippy --workspace --all-targets --all-features -- -D warnings` and
  `cargo fmt --all -- --check` were clean at every commit point.

## Task Commits

Each task was committed atomically:

1. **Task 1: Wire the LLM unit-test module in and make it compile** - `465ecdb` (feat)
2. **Task 2: Make the reactivated LLM tests pass at runtime** - `7c3d4b7` (fix)
3. **Task 3: Write the provider-switching integration test (Epic 6 task 7.10)** - `7257f88` (test)

**Plan metadata:** (this SUMMARY's own commit, made immediately after this file)

## Files Created/Modified

- `tests/unit/mod.rs` - Added `pub mod llm;` in alphabetical position (between
  `herald_consolidation_test` and `maneuver_domain_tests`), no feature guard.
- `tests/unit/llm/mod.rs` - Reordered its three existing `pub mod` declarations alphabetically
  (`anthropic_adapter_test`, `deepseek_adapter_test`, `provider_factory_test`) — the only change,
  forced by rustfmt's `reorder_modules` default now that the module is reachable from the crate
  root for the first time. See "Deviations" below.
- `tests/unit/llm/deepseek_adapter_test.rs` - Fixed imports, `LlmRequest`/`PromptItem`/
  `DeepSeekConfig` construction, made `setup_mock_server` async, `.create_async().await` at every
  call site, rewrote `test_deepseek_timeout` to use a raw TCP listener and assert
  `LlmError::Timeout` instead of the stale `NetworkError` expectation.
- `tests/unit/llm/anthropic_adapter_test.rs` - Same class of fixes: imports,
  `LlmRequest`/`PromptItem`/`UserPrompt`/`AnthropicConfig` construction, async
  `setup_mock_server`, `.create_async().await` at every call site.
- `tests/unit/llm/provider_factory_test.rs` - Wrapped 24 `env::set_var`/`remove_var` calls in
  individual `unsafe` blocks with SAFETY comments (Task 1); routed `Result::unwrap_err()` calls to
  `.err().unwrap()` since `Arc<dyn LlmPort>` is not `Debug`; added `#[allow(
  clippy::default_constructed_unit_structs)]` to `test_factory_default`; added the
  `PROVIDER_ENV_LOCK` static `Mutex` and `CleanProviderEnv` RAII guard, and used it in all five
  tests that read or mutate the three provider API-key env vars (Task 2).
- `tests/integration/provider_switching_test.rs` (new) - `test_provider_switch_preserves_
  request_contract` and `test_provider_switch_rejects_unknown_provider`.
- `tests/integration/mod.rs` - Added `pub mod provider_switching_test;` in alphabetical position
  (between `paladin_integration_test` and `qdrant_sanctum_tests`), no feature guard.

## Per-file test counts (for plan 02-09's ledger amendment)

| File | Present | Passing | Removed |
|---|---|---|---|
| `tests/unit/llm/deepseek_adapter_test.rs` | 8 | 8 | 0 |
| `tests/unit/llm/anthropic_adapter_test.rs` | 9 | 9 | 0 |
| `tests/unit/llm/provider_factory_test.rs` | 8 | 8 | 0 |
| **Total** | **25** | **25** | **0** |

No test was removed under the structural-breakage rule. All breakage found in all three files was
mechanical (construction-API drift from the `paladin-ports` extraction and facade rewiring, plus
two runtime-only hazards) and every occurrence was repaired in place — there was no case where a
test asserted behaviour the current adapters no longer have.

## Decisions Made

- Followed the plan's task structure exactly: three atomic commits, one per task, matching the
  plan's own Task 1 (compile) / Task 2 (runtime) / Task 3 (new test) split.
- Imported `DeepSeekAdapter`/`AnthropicAdapter` and their configs directly from
  `paladin_llm::{deepseek,anthropic}` rather than through the `paladin` facade — see frontmatter
  `key-decisions` for the full reasoning (facade re-export is feature-gated on the root package's
  own `llm-deepseek`/`llm-anthropic` flags, off by default; `paladin_llm` itself always compiles
  all three adapters).
- Chose `MockLlmAdapter` + a real `DeepSeekAdapter` on a local mockito server for the
  provider-switching test, rather than `LlmProviderFactory::create()` with faked environment
  variables — avoids reintroducing the exact process-wide env-var interference class
  `CleanProviderEnv` exists to neutralise, while still proving two genuinely distinct `LlmPort`
  implementations.
- `test_deepseek_timeout` now binds a raw `std::net::TcpListener` and deliberately never
  `accept()`s on it, rather than relying on mockito leaving a request unmocked — mockito's server
  responds immediately (501) to unmatched requests, so it cannot exercise a real timeout at all.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Mechanical construction-API repairs across all three test files (Task 1)**
- **Found during:** Task 1
- **Issue:** `LlmRequest` (4→6 fields), `PromptItem::new` (full-struct→`Result<Self,_>`-returning
  `PromptType`-only signature), `PromptData`/`SystemPrompt`/`UserPrompt` field sets, and
  `DeepSeekConfig`/`AnthropicConfig` field sets had all drifted since the tests were written,
  exactly as CONTEXT.md's D-11 predicted (the directory's last two commits post-date the tests).
- **Fix:** Rebuilt every construction site against the current shapes; imported `DeepSeekAdapter`/
  `AnthropicAdapter` from `paladin_llm` directly instead of the feature-gated `paladin` facade
  re-export.
- **Files modified:** `tests/unit/llm/deepseek_adapter_test.rs`, `tests/unit/llm/
  anthropic_adapter_test.rs`.
- **Verification:** `cargo build --tests` and `cargo build --workspace --all-targets` both exit 0.
- **Committed in:** `465ecdb` (Task 1 commit).

**2. [Rule 1 - Bug] `Result::unwrap_err()` on a non-`Debug` `Ok` type (Task 1)**
- **Found during:** Task 1
- **Issue:** `factory.create(...)` returns `Result<Arc<dyn LlmPort>, ProviderFactoryError>`;
  `Arc<dyn LlmPort>` is not `Debug`, so `Result::unwrap_err` (which requires `T: Debug`) failed to
  compile at 7 call sites in `provider_factory_test.rs`.
- **Fix:** Routed each through `.err().unwrap()` (`Result::err` → `Option<E>`, then
  `Option::unwrap`, which has no `Debug` bound on the `Ok` type).
- **Files modified:** `tests/unit/llm/provider_factory_test.rs`.
- **Verification:** compiles; the same 7 assertions still pass at runtime.
- **Committed in:** `465ecdb` (Task 1 commit).

**3. [Rule 1 - Bug] Clippy `default_constructed_unit_structs` on `test_factory_default` (Task 1)**
- **Found during:** Task 1 (pre-commit clippy gate)
- **Issue:** `LlmProviderFactory::default()` on a zero-field unit struct trips
  `clippy::default_constructed_unit_structs`, but the test's entire purpose is exercising the
  `Default` trait impl itself.
- **Fix:** Added a scoped `#[allow(clippy::default_constructed_unit_structs)]` with a comment
  explaining why the suggested rewrite would defeat the test.
- **Files modified:** `tests/unit/llm/provider_factory_test.rs`.
- **Verification:** `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits 0.
- **Committed in:** `465ecdb` (Task 1 commit).

**4. [Rule 3 - Blocking] `tests/unit/llm/mod.rs` reordered by rustfmt (Task 1)**
- **Found during:** Task 1
- **Issue:** The plan's own acceptance criteria required both "`tests/unit/llm/mod.rs` is
  unchanged" and "`cargo fmt --check` exits 0". Once `pub mod llm;` made the module reachable from
  the crate root for the first time, rustfmt's `reorder_modules` (stable, on by default) reached
  this file and required reordering its three `pub mod` lines alphabetically — a diff that did not
  exist while the module was unreferenced and therefore unvisited by rustfmt.
- **Fix:** Applied the one-line reorder (`anthropic_adapter_test`, `deepseek_adapter_test`,
  `provider_factory_test`) via `cargo fmt --all`. No other change to the file — the module
  declarations it makes and their targets are byte-identical.
- **Files modified:** `tests/unit/llm/mod.rs`.
- **Verification:** `cargo fmt --all -- --check` exits 0; `git diff` on this file shows only the
  reorder.
- **Committed in:** `465ecdb` (Task 1 commit).

**5. [Rule 1 - Bug, D-11 mechanical] `test_deepseek_timeout`'s stale error-variant assertion (Task 2)**
- **Found during:** Task 2
- **Issue:** The test asserted a timeout maps to `LlmError::NetworkError`. The adapter now maps
  `reqwest`'s `is_timeout()` errors to a dedicated `LlmError::Timeout` variant, added after this
  test was written, so callers can retry with a longer timeout rather than treating every network
  failure identically.
- **Fix:** Updated the assertion to `LlmError::Timeout(_)`. This is D-11 mechanical breakage, not
  structural: the tested behaviour (a timeout produces a distinguishable, catchable error) is
  unaffected — only the variant name changed.
- **Files modified:** `tests/unit/llm/deepseek_adapter_test.rs`.
- **Verification:** test passes.
- **Committed in:** `7c3d4b7` (Task 2 commit).

**6. [Rule 1 - Bug] `test_deepseek_timeout` never actually exercised a timeout (Task 2)**
- **Found during:** Task 2
- **Issue:** Even after fixing the runtime-nesting panic and the error-variant assertion, the test
  still failed: mockito's server responds immediately (501) to any unmocked request, so the
  request never actually timed out — it hit the generic `_ => ProcessingError` arm of `map_error`
  instead.
- **Fix:** Rewrote the test to bind a raw `std::net::TcpListener` and never call `.accept()` on it.
  The TCP handshake completes (the kernel's listen backlog accepts the SYN), the client's HTTP
  request bytes are buffered, and no response is ever produced — forcing a genuine client-side
  timeout within the configured 1-second budget.
- **Files modified:** `tests/unit/llm/deepseek_adapter_test.rs`.
- **Verification:** test passes in ~1.16s (consistent with the 1-second configured timeout firing).
- **Committed in:** `7c3d4b7` (Task 2 commit).

**7. [Rule 1 - Bug, T-02-23] Process-wide env-var interference and ambient sandbox pollution (Task 2)**
- **Found during:** Task 2
- **Issue:** Two failure modes surfaced together: (a) `test_get_default_provider`/
  `test_list_available_providers` mutate `OPENAI_API_KEY`/`DEEPSEEK_API_KEY`/`ANTHROPIC_API_KEY`,
  which `cargo test`'s default parallel-thread execution could race against sibling tests in the
  same file reading the same vars; (b) this sandbox predefines those three vars as empty-but-set
  strings, which `std::env::var` treats as present, so `ConfigurationMissing`-expecting tests
  failed against a polluted rather than clean environment.
- **Fix:** Added a `PROVIDER_ENV_LOCK` static `Mutex<()>` and a `CleanProviderEnv` RAII guard that
  acquires the lock, snapshots and clears all three vars on construction, and restores each to its
  prior value (present-with-value or absent) on `Drop` — including on panic/unwind. Used in all
  five tests that read or mutate these vars; the two that already saved/cleared/restored manually
  were simplified to use the guard instead.
- **Files modified:** `tests/unit/llm/provider_factory_test.rs`.
- **Verification:** `cargo test --test unit -- llm::provider_factory_test` passes reliably across
  repeated runs; `cargo test --test unit` (full binary, default parallelism) passes.
- **Committed in:** `7c3d4b7` (Task 2 commit).

---

**Total deviations:** 7 auto-fixed (2 blocking/Rule 3, 5 bug/Rule 1). Zero tests deleted; zero
architectural questions raised (no Rule 4 escalation).
**Impact on plan:** All auto-fixes were necessary to reach a compiling, passing, honest test
suite — none expanded scope beyond what the plan's own D-11 fallback rule and Task 2's named
runtime hazards anticipated, except the ambient-environment-pollution and
never-actually-timing-out discoveries, both of which are runtime facts this sandbox surfaced that
neither the plan text nor 02-RESEARCH.md's "Common Pitfalls" section named explicitly.

## Issues Encountered

None beyond the deviations documented above — every issue found during execution was resolved
within the deviation rules without escalation.

## User Setup Required

None - no external service configuration required. The provider-switching test and all
reactivated LLM tests run fully offline with no API keys.

## Next Phase Readiness

- **Plan 02-09** can amend `.planning/ledgers/milestone-01.md`'s `REQ-provider-testing` row and
  Epic 6's nested task rows using the per-file test counts table above: `tests/unit/llm/` is now
  `satisfied` (25/25 passing, 0 removed), and Epic 6 task 7.10 is now `satisfied` (
  `tests/integration/provider_switching_test.rs`, 2/2 passing).
- The `CleanProviderEnv` RAII pattern in `provider_factory_test.rs` is available as a precedent
  for any future test in this workspace that needs to mutate process-wide environment state
  safely under parallel test execution.
- No blockers for sibling Phase 2 plans: this plan touched only the six files listed above (all
  within its declared `files_modified` set), and the full workspace suite, clippy, and fmt all
  stayed green at every commit point.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*

## Self-Check: PASSED

- FOUND: `tests/integration/provider_switching_test.rs`
- FOUND: `.planning/phases/02-functional-gap-closure/02-06-SUMMARY.md`
- FOUND: commit `465ecdb` (Task 1)
- FOUND: commit `7c3d4b7` (Task 2)
- FOUND: commit `7257f88` (Task 3)
- FOUND: commit `8132cbf` (this SUMMARY)
- FOUND: all 6 key modified/created files verified present on disk (`tests/unit/mod.rs`,
  `tests/unit/llm/mod.rs`, `tests/unit/llm/deepseek_adapter_test.rs`,
  `tests/unit/llm/anthropic_adapter_test.rs`, `tests/unit/llm/provider_factory_test.rs`,
  `tests/integration/mod.rs`)
