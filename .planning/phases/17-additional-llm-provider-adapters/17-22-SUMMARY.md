---
phase: 17-additional-llm-provider-adapters
plan: 22
subsystem: api
tags: [rust, llm, compat-engine, observability, qwen, dashscope, live-verification, gap-closure]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "QWEN_DEFAULT_BASE_URL reversed to US-Virginia (17-21) and every operator-facing document brought current with it (17-20) — the forward reference in qwen/adapter.rs's rustdoc this plan removes, and the state that had to be reachable before a region-mismatch demonstration meant anything"
provides:
  - "classify_fetch_failure(&LlmError) -> FetchFailureClass in crates/paladin-llm/src/compat/engine.rs — an exhaustive, no-wildcard classification distinguishing a misconfiguration (AuthenticationError) from every state the D-13/D-14 fallback design already supports quietly (NetworkError, Timeout, RateLimitExceeded, UsageLimitExceeded, ModelNotAvailable, TokenLimitExceeded, EmptyCompletion, InvalidPrompt, ProcessingError)"
  - "base_url_without_userinfo() so a base URL carrying a userinfo component never appears verbatim in the new warn line (T-17-91)"
  - "available_models()'s Err(e) arm reads that classification: a misconfiguration is raised to warn (naming the endpoint), everything else keeps its original debug wording — the D-13/D-14 fallback CONTRACT (Vec<String>, never errors, curated list on every failure) is unchanged and pinned by two new mockito/connection-refused tests asserting the returned value"
  - "A dependency-free log::Log implementation in live_vendor_smoke.rs (log::set_logger with a 'static zero-sized instance, not set_boxed_logger, which needs a feature this workspace does not enable) so the harness can actually show the new diagnostic"
  - "Two live harness runs recorded verbatim below: a deliberately mismatched DASHSCOPE_BASE_URL produces the new warn line naming the rejecting endpoint with no credential in the output; the shipped defaults produce no warning at all"
  - "The Qwen module rustdoc's region-mismatch symptom paragraph amended from the pre-fix silent-list description to what the code now does, with plan 17-22's forward reference removed"
  - "A generic, vendor-neutral 'A rejected credential now announces itself' section in docs/src/getting-started/configuration.md, quoting the message shape from this plan's own recorded harness run"
affects: ["phase close / PROV-02 and PROV-04 adjudication — Qwen's generate() probe remains blocked on .planning/WINDOWS.md id 21 (an Alibaba Model Studio account-entitlement gap), so the four-vendor-PASS clause this plan's must_haves describe is reachable in principle but NOT achieved by this run; see 'Unmet Must-Have' below"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "A total, no-wildcard match over a shared error enum as the mechanism for a vendor-agnostic diagnostic in a shared engine — CompatEngine backs six presets and none of them changed, matching D-05's boundary"
    - "log::set_logger with a 'static zero-sized struct instead of log::set_boxed_logger, when a crate's `log` dependency does not enable the `std` feature — avoids adding a feature (a dependency-surface change) just to install an example-local logger"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/compat/engine.rs
    - crates/paladin-llm/examples/live_vendor_smoke.rs
    - crates/paladin-llm/src/qwen/adapter.rs
    - docs/src/getting-started/configuration.md

key-decisions:
  - "log::set_logger (not set_boxed_logger) with a 'static StderrLogger instance: set_boxed_logger requires log's `std` feature, which `log = { workspace = true }` does not enable in this workspace, and enabling it would itself be the kind of dependency-surface change T-17-SC-22's prohibition rules out. A zero-sized 'static logger needs no allocation and no additional feature — functionally identical to what the plan's interface_context described, adapted to what the workspace's actual Cargo.toml allows. Treated as Rule 3 (blocking build error) rather than a deviation requiring a checkpoint."
  - "The harness's logger is fixed at LevelFilter::Warn rather than parsing RUST_LOG: hand-rolling a level-string parser with no dependency would add complexity this harness does not need to prove either half of task 2 (the warning fires on a mismatch and stays silent on the shipped defaults), and the executor_notes explicitly permit stating a fixed-level writer rather than adding a filter dependency."
  - "Ollama, ModelNotAvailable, TokenLimitExceeded, EmptyCompletion and InvalidPrompt are classified Supported (quiet) even though `fetch_live_models` cannot actually produce four of those five variants — the match must be exhaustive over the TYPE, not merely over what is reachable through this one call path, so every variant needed an explicit arm and a stated reason rather than being grouped under a reachability argument."
  - "The refused-redirect case (map_error's 300..=399 arm, surfaced as ProcessingError) stays quiet by design rather than being split into its own Misconfiguration arm: that message already names the exact setting to check on its own, so raising it to warn here would duplicate an already-actionable message rather than surface a silent one — stated explicitly in the match's own comment, per the plan's instruction."
  - "requirements-completed left empty, matching this phase's established precedent (17-18/17-19/17-20/17-21 SUMMARYs): PROV-02 and PROV-04 remain open for phase-close adjudication. See 'Unmet Must-Have' below for exactly why this run does not close them alone."

patterns-established: []

requirements-completed: []

# Coverage metadata
coverage:
  - id: D1
    description: "classify_fetch_failure is an exhaustive, no-wildcard match over every LlmError variant; only AuthenticationError is Misconfiguration, every other variant is Supported, each pinned by its own test"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/compat/engine.rs#classify_fetch_failure_* (10 tests, one per LlmError variant)"
        status: pass
    human_judgment: false
  - id: D2
    description: "available_models() returns the curated fallback list unchanged on both a 401 (misconfiguration) and a real connection failure (offline) — the D-13/D-14 contract is provably untouched"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/compat/engine.rs#available_models_returns_curated_fallback_on_authentication_failure, #available_models_returns_curated_fallback_on_connection_failure"
        status: pass
    human_judgment: false
  - id: D3
    description: "A base URL carrying a userinfo component does not appear verbatim in the emitted message; a plain URL is left unchanged"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/compat/engine.rs#base_url_without_userinfo_* (4 tests)"
        status: pass
    human_judgment: false
  - id: D4
    description: "The behaviour is demonstrated live: a deliberately mismatched DASHSCOPE_BASE_URL (Singapore) produces the new warn line naming the rejecting endpoint, with no credential anywhere in the output"
    requirement: "PROV-02"
    verification:
      - kind: integration
        ref: "crates/paladin-llm/examples/live_vendor_smoke.rs (live run A, 2026-08-22, verbatim output below)"
        status: pass
    human_judgment: false
  - id: D5
    description: "The shipped-defaults run emits no warning at all, and Kimi, Grok and Gemini PASS both probes (no regression); Qwen PASSES the model-list probe and FAILS generate() on the pre-existing, unrelated WINDOWS id 21 entitlement gap"
    requirement: "PROV-02"
    verification:
      - kind: integration
        ref: "crates/paladin-llm/examples/live_vendor_smoke.rs (live run B, 2026-08-22, verbatim output below; re-run once to rule out a transient Kimi model-list blip on the first attempt)"
        status: pass
    human_judgment: true
    rationale: "This plan's own must_haves ask for 'four vendors PASS' on the shipped-defaults run, which is UNREACHABLE this run: Qwen's generate() probe is blocked on .planning/WINDOWS.md id 21, an Alibaba account-entitlement gap outside this plan's scope (see 'Unmet Must-Have' below). D5 is recorded as pass for the achievable form (3 vendors PASS both probes, Qwen PASSES model-list, no spurious warning) — a human must confirm that partial framing is the correct and honest reading rather than a silently narrowed claim."
  - id: D6
    description: "No document in the tree still describes a rejected credential as producing a silent, plausible model list; the Qwen rustdoc's forward reference to this plan is gone; the configuration guide states the new behaviour generically, not per-vendor"
    requirement: "PROV-04"
    verification:
      - kind: manual_procedural
        ref: "crates/paladin-llm/src/qwen/adapter.rs 'Region default' section; docs/src/getting-started/configuration.md 'A rejected credential now announces itself'; grep confirmed the silent-list symptom existed in exactly one place before this plan"
        status: pass
    human_judgment: true
    rationale: "Whether the amended prose accurately matches what task 2's live run actually recorded, and whether it reads as genuinely vendor-neutral, is a documentation-fidelity judgment"
  - id: D7
    description: "The whole workspace builds, tests and lints clean after every edit; no dependency was added to any Cargo.toml"
    requirement: null
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --features kimi,qwen,grok,ollama,openai-compatible,gemini (259 passed); cargo test --test unit --features llm-all (428 passed, 11 ignored); cargo fmt --check (clean); cargo clippy --workspace --all-targets --features llm-all -- -D warnings (clean); git diff --stat on both Cargo.toml files (empty)"
        status: pass
    human_judgment: false

# Metrics
duration: ~40min
completed: 2026-08-22
status: complete
---

# Phase 17 Plan 22: A rejected credential now announces itself — G-17-4d's diagnosability half closed

**`available_models()` now warns, naming the endpoint, when a `CompatEngine`-backed provider's credential is rejected — and stays exactly as quiet as before for every offline/transient state D-13/D-14 already support — via a total, no-wildcard classification over `LlmError`; the four-vendor-PASS demonstration this plan's own must_haves ask for remains blocked on the pre-existing, unrelated `.planning/WINDOWS.md` id 21 Qwen entitlement gap, reported here as blocked rather than claimed as met.**

## Performance

- **Duration:** ~40 min
- **Completed:** 2026-08-22
- **Tasks:** 3 of 3 completed and committed
- **Files modified:** 4

## Accomplishments

- Added `classify_fetch_failure(&LlmError) -> FetchFailureClass` to `crates/paladin-llm/src/compat/engine.rs`: an exhaustive match with no wildcard arm over all ten `LlmError` variants. Only `AuthenticationError` is `Misconfiguration`; every other variant — `NetworkError`, `Timeout` (the offline/transient states D-13/D-14 exist for), `RateLimitExceeded`, `UsageLimitExceeded`, `ModelNotAvailable`, `TokenLimitExceeded`, `EmptyCompletion`, `InvalidPrompt`, `ProcessingError` (which also covers the refused-redirect message) — is `Supported` and stays quiet, each with its own one-line reason in the match arm's comment and its own unit test.
- Added `base_url_without_userinfo()` so a base URL carrying a `user:pass@` component never appears verbatim in the new log line (T-17-91), with four tests covering the stripped case, an unchanged plain URL, a port/path preserved after stripping, and a malformed string with no scheme separator.
- Rewired `available_models()`'s `Err(e)` arm to read the classification: a `Misconfiguration` is raised to `log::warn!`, naming the configured endpoint (with any userinfo stripped) and stating the returned list is the curated fallback, not the vendor's own catalog. Every `Supported` failure keeps its exact original `debug` wording. The empty-live-list arm and the `OnceCell` memoization are both untouched.
- Pinned the D-13/D-14 fallback contract with two new tests asserting the RETURNED VALUE (not the log): `available_models()` still returns the curated list, unchanged, on a mocked 401 and on a real connection-refused failure (a bound-then-dropped TCP listener, no mock server involved).
- Gave `live_vendor_smoke.rs` a ~25-line dependency-free `log::Log` implementation so the harness can show the new diagnostic — installed via `log::set_logger` with a `'static` zero-sized instance (not `set_boxed_logger`, which needs `log`'s `std` feature; see Decisions).
- Ran the harness twice against live vendor endpoints (verbatim output below): Run A (`DASHSCOPE_BASE_URL` deliberately set to the Singapore endpoint) shows the new warn line naming the rejecting endpoint, with no credential anywhere in the output. Run B (shipped defaults) shows no warning at all, confirming the quiet path stays quiet on a healthy configuration.
- Amended the Qwen module rustdoc's "Region default" section from the pre-fix "plausible three-entry curated list, not an error" description to what the code now does (a `warn` line naming the endpoint), and removed the forward reference to this plan.
- Added a vendor-neutral "A rejected credential now announces itself" section to `docs/src/getting-started/configuration.md`, quoting the message shape from this plan's own recorded harness run rather than from a predicted wording, and stating explicitly that this applies to every `CompatEngine`-backed provider (all of them except Ollama).

## Task Commits

Each task was committed atomically:

1. **Task 1: The engine stops describing a misconfiguration the way it describes being offline** — `510154a` (feat)
2. **Task 2: Show it happening — a deliberately mismatched region, then the run's final regression record** — `83b041a` (feat)
3. **Task 3: The documented symptom matches what the code now does** — `48b1600` (docs)

## Files Created/Modified

- `crates/paladin-llm/src/compat/engine.rs` — `FetchFailureClass` enum, `classify_fetch_failure()`, `base_url_without_userinfo()`, `available_models()`'s `Err(e)` arm rewired to read the classification, rustdoc recording the change and the unchanged D-13/D-14 contract, 16 new unit tests.
- `crates/paladin-llm/examples/live_vendor_smoke.rs` — `StderrLogger` (`log::Log` impl, fixed at `LevelFilter::Warn`), installed at the top of `main` via `log::set_logger`.
- `crates/paladin-llm/src/qwen/adapter.rs` — "Region default" section's symptom paragraph amended to the current behaviour; forward reference to plan 17-22 removed.
- `docs/src/getting-started/configuration.md` — new "A rejected credential now announces itself" section, generic across every `CompatEngine`-backed provider.

## Decisions Made

- **`log::set_logger`, not `set_boxed_logger`.** The plan's own `interface_context` described installing the logger via `log::set_boxed_logger` + `log::set_max_level`; `set_boxed_logger` turned out to require `log`'s `std` feature, which this workspace's `log = { workspace = true }` does not enable, and turning it on would itself be exactly the dependency-surface change the plan's own T-17-SC-22 prohibition rules out. `log::set_logger` takes a `&'static dyn Log` instead of a `Box`, needs no additional feature, and a zero-sized `static StderrLogger` instance satisfies it with no allocation. Treated as a Rule 3 auto-fix (a blocking build error), not a deviation requiring a checkpoint, since it does not touch what the plan actually prohibits (no dependency added, no feature enabled).
- **Fixed `LevelFilter::Warn`, no `RUST_LOG` parsing.** The executor_notes explicitly say to state a fixed-level writer rather than add a filter dependency if that is what the harness ends up needing; `Warn` is exactly the level the new diagnostic fires at, so no lower-level parsing was needed to prove either half of task 2.
- **Every `LlmError` variant carries its own classification and reason, including four this call path cannot currently produce** (`ModelNotAvailable`, `TokenLimitExceeded`, `EmptyCompletion`, `InvalidPrompt`) — because the match's exhaustiveness guarantee is over the *type*, not over what `fetch_live_models` happens to be able to return today. A future change to that function that started producing one of these would not silently inherit a classification nobody chose.
- **The refused-redirect case (`ProcessingError` from `map_error`'s `300..=399` arm) stays `Supported`/quiet by design**, not by omission — it is arguably a misconfiguration too, but its own message already names the exact setting to check, and raising it to `warn` here would duplicate an already-actionable message rather than surface a silent one. Stated in the match arm's own comment per the plan's instruction.
- **`requirements-completed` left empty**, matching this phase's established precedent. See "Unmet Must-Have" below for why PROV-02/PROV-04 are not adjudicated closed by this run alone.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] `log::set_boxed_logger` does not compile without a feature this workspace does not enable**
- **Found during:** Task 2, first `cargo build` of the harness
- **Issue:** `error[E0425]: cannot find function 'set_boxed_logger' in crate 'log'` — the function is gated behind `log`'s `std` feature, which `crates/paladin-llm/Cargo.toml`'s `log = { workspace = true }` does not turn on.
- **Fix:** Used `log::set_logger(&'static dyn Log)` instead, backed by a zero-sized `static StderrLogger = StderrLogger;` — functionally identical (installs a global logger, no allocation), needs no feature change, and adds nothing to any `Cargo.toml`.
- **Files modified:** `crates/paladin-llm/examples/live_vendor_smoke.rs`
- **Verification:** `cargo build -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini` succeeds; `git diff --stat` on both `Cargo.toml` files is empty
- **Committed in:** `83b041a` (Task 2's commit)

---

**Total deviations:** 1 auto-fixed (blocking build error, resolved without touching any dependency surface)
**Impact on plan:** Necessary for task 2 to compile at all; explicitly does not violate the plan's own prohibition against adding a dependency, since no `Cargo.toml` changed.

## Issues Encountered

**A transient Kimi model-list blip on the first Run B attempt, ruled out by an immediate re-run.** The first execution of Run B (shipped defaults) showed Kimi's model-list probe FAIL (byte-identical to the curated fallback) while Kimi's `generate()` probe PASSED with real content — meaning the credential and endpoint were both fine, and only the `/models` fetch specifically hiccupped. Re-running immediately, with no code or configuration change, produced Kimi PASS/PASS, matching this phase's documented `state_of_the_world` (Kimi live-verified both probes as of 2026-08-22). The second run is the one recorded below and used for this plan's regression record; the first run's transient result is noted here rather than silently discarded, per this plan's own standard of reporting exactly what was observed.

## Unmet Must-Have — reported as blocked, not as met

This plan's `must_haves.truths` includes: *"the same harness against the shipped defaults emits none and reports four vendors PASS."* **This clause is UNREACHABLE by this run and is NOT claimed as met.** Per the run's `state_of_the_world` briefing: Qwen's `generate()` probe fails with `HTTP 403 Model.AccessDenied` — an Alibaba Model Studio account-entitlement gap (the credential can list the full regional catalog but has no chat-completion invocation entitlement for any model in that workspace), filed as `.planning/WINDOWS.md` id 21, requiring a human to activate model access in the Alibaba console. No code change in this plan's scope — or any scope — resolves it; it was already diagnosed exhaustively in `17-21-SUMMARY.md`.

**What this plan actually delivers against that clause, and what remains open:**

- Run B (shipped defaults, recorded verbatim below) shows: Kimi PASS/PASS, Grok PASS/PASS, Gemini PASS/PASS, Qwen model-list PASS / generate FAIL (WINDOWS id 21) — **3 of 4 vendors fully PASS, the fourth blocked on a pre-existing, unrelated, human-actionable gap.**
- Run B emits **no warning at all** — the shipped-defaults configuration is correctly classified as healthy, which is the actual, in-scope deliverable this clause was checking for (a clean run does not cry wolf).
- The mismatched-region demonstration (Run A) IS fully available and IS recorded: it proves the new diagnostic fires exactly where G-17-4c's silence caused a five-day misdiagnosis.

**What would close the full clause:** a human activating `qwen-plus` (or any model) for chat-completion invocation in the Alibaba Model Studio console for the US (Virginia) workspace tied to `DASHSCOPE_API_KEY`, per the remedy already documented in `17-21-SUMMARY.md`'s "Authentication Gate" section. Once that happens, re-running the harness with shipped defaults should show all four vendors PASS both probes with no code change required — this plan's diagnostic and fallback-contract work are already complete and do not depend on that activation.

**G-17-4c status, for `/gsd-verify-work`'s reconciliation:** G-17-4c was resolved on 2026-08-22 by live verification (17-21) and was NOT a target of this run. This run's scope was exclusively G-17-4d's diagnosability half (plus the incidental G-17-4a/G-17-4b regression checks Run B's four-vendor table also covers).

## User Setup Required

**Yes — but not newly introduced by this plan.** The same external action `17-21-SUMMARY.md` already documented remains outstanding: a human must activate at least one model (`qwen-plus` recommended) for chat-completion invocation in the Alibaba Model Studio console for the US (Virginia) workspace. This plan does not add a new user-setup requirement; it makes the FAILURE MODE this gap produces visible (a `warn` line would now fire immediately if the entitlement gap were instead a credential/region mismatch) rather than resolving the gap itself, which is outside any code's reach.

## Next Phase Readiness

- **G-17-4d's diagnosability half is closed and live-demonstrated.** Every `CompatEngine`-backed preset (kimi, qwen, grok, ollama, openai-compatible, and the deepseek-style preset family) now surfaces a rejected credential at `warn` instead of looking identical to being offline, with no preset-level change required — the mechanism is a pure function of `LlmError` alone, matching D-05.
- **This phase (17-additional-llm-provider-adapters) has no further plans queued.** Per `17-20-SUMMARY.md` and `17-21-SUMMARY.md`, this was the last plan carrying PROV-04's masked-auth-failure fix. Phase-close adjudication of PROV-02 and PROV-04 should read this SUMMARY's "Unmet Must-Have" section directly: the code-and-documentation deliverables are complete and tested, but the phase's own four-vendor live-PASS bar is not met because of `.planning/WINDOWS.md` id 21, an external account-entitlement gap outside any plan's reach.
- **`.planning/WINDOWS.md` id 21 remains open** and is the sole blocker on this plan's own most literal must-have wording. It requires a human action in the Alibaba Cloud console, not a code or documentation change — verification command is unchanged from `17-21-SUMMARY.md`: `cargo run -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini` with `DASHSCOPE_BASE_URL` unset, expecting Qwen's generate probe to flip to PASS once the console-side activation happens.

---

## Live Verification Evidence

### Run A — deliberately mismatched region (`DASHSCOPE_BASE_URL` set to Singapore), 2026-08-22

Command:
```
DASHSCOPE_BASE_URL=https://dashscope-intl.aliyuncs.com/compatible-mode/v1 \
  cargo run -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini
```

Output (verbatim; no credential value appears anywhere below — confirmed before quoting):

```
[WARN] configured endpoint https://dashscope-intl.aliyuncs.com/compatible-mode/v1 rejected the request while listing models (Authentication failed: Invalid API key. Error: {"error":{"message":"Incorrect API key provided. For details, see: https://www.alibabacloud.com/help/en/model-studio/error-code#apikey-error","type":"invalid_request_error","param":null,"code":"invalid_api_key"},"request_id":"cf63a3a2-35ce-987b-bbc9-e997bbd9a4bc"}); the returned model list is the curated fallback, not this vendor's own catalog — a credential scoped to a different account or region is the usual cause

=== Kimi (MOONSHOT_API_KEY) ===
  base_url      : https://api.moonshot.ai/v1
  default model : kimi-k3
  -- model list probe --
  models returned: 4
  live fetch    : YES — differs from curated fallback
  sample        : kimi-k2.6, kimi-k2.7-code, kimi-k2.7-code-highspeed, kimi-k3
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 61 chars; tokens prompt=86 completion=179 total=265
  RESULT        : PASS

=== Qwen (DASHSCOPE_API_KEY) ===
  base_url      : https://dashscope-intl.aliyuncs.com/compatible-mode/v1
                  [OVERRIDE via *_BASE_URL — shipped default is https://dashscope-us.aliyuncs.com/compatible-mode/v1]
  default model : qwen-plus
  -- model list probe --
  models returned: 3
  live fetch    : NO — result is byte-identical to the curated fallback
  RESULT        : FAIL (live-fetch path not exercised)
  -- generate() probe (default prompt parameters) --
  RESULT        : FAIL — Authentication failed: Invalid API key. Error: {"error":{"message":"Incorrect API key provided. For details, see: https://www.alibabacloud.com/help/en/model-studio/error-code#apikey-error","type":"invalid_request_error","param":null,"code":"invalid_api_key"},"request_id":"8500f388-8f3b-9a45-8856-9e6fa50e7b67"}

=== Grok (XAI_API_KEY) ===
  base_url      : https://api.x.ai/v1
  default model : grok-4.6
  -- model list probe --
  models returned: 12
  live fetch    : YES — differs from curated fallback
  sample        : grok-4.20-0309-non-reasoning, grok-4.20-0309-reasoning, grok-4.20-multi-agent-0309, grok-4.3, grok-4.5, grok-4.6, grok-build-0.1, grok-imagine-image, … (+4 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 4 chars; tokens prompt=637 completion=1 total=682
  RESULT        : PASS

=== Gemini (GEMINI_API_KEY) ===
  base_url      : https://generativelanguage.googleapis.com/v1beta
  default model : gemini-3.6-flash
  -- model list probe --
  models returned: 50
  live fetch    : YES — differs from curated fallback
  sample        : antigravity-preview-05-2026, aqa, deep-research-max-preview-04-2026, deep-research-preview-04-2026, deep-research-pro-preview-12-2025, gemini-2.5-computer-use-preview-10-2025, gemini-2.5-flash, gemini-2.5-flash-image, … (+42 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 36 chars; tokens prompt=2 completion=14 total=248
  RESULT        : PASS

──────────────────────────────────────────
6 of 8 probes passed (4 vendors × 2 probes each; 1 model-list failures, 1 generate failures)
```

**Analysis:** the `[WARN]` line appears exactly once (memoization: one fetch per engine lifetime), names the actual rejecting endpoint (`dashscope-intl`, not the shipped default), states the returned list is the curated fallback, and gives the correct usual cause. This is the demonstration the plan asked for: the identical class of misconfiguration that was silent before this plan (a well-formed 3-model list with nothing above `debug`) now says so.

### Run B — shipped defaults (`DASHSCOPE_BASE_URL` confirmed unset), 2026-08-22

Command:
```
unset DASHSCOPE_BASE_URL
cargo run -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini
```

`DASHSCOPE_BASE_URL is: [unset]` was printed and confirmed before the run. Output below is the **second** of two consecutive runs — the first showed a transient Kimi model-list blip (see "Issues Encountered" above) that cleared immediately with no change; this is the stable result:

```
=== Kimi (MOONSHOT_API_KEY) ===
  base_url      : https://api.moonshot.ai/v1
  default model : kimi-k3
  -- model list probe --
  models returned: 4
  live fetch    : YES — differs from curated fallback
  sample        : kimi-k2.6, kimi-k2.7-code, kimi-k2.7-code-highspeed, kimi-k3
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 41 chars; tokens prompt=86 completion=183 total=269
  RESULT        : PASS

=== Qwen (DASHSCOPE_API_KEY) ===
  base_url      : https://dashscope-us.aliyuncs.com/compatible-mode/v1
  default model : qwen-plus
  -- model list probe --
  models returned: 92
  live fetch    : YES — differs from curated fallback
  sample        : deepseek-v4-flash, deepseek-v4-flash-0731, deepseek-v4-flash-0731-us, deepseek-v4-flash-us, deepseek-v4-pro, deepseek-v4-pro-0813, deepseek-v4-pro-us, glm-5.1, … (+84 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  RESULT        : FAIL — Processing error: API error (403): {"error":{"message":"Model access denied.","type":"Model.AccessDenied","param":null,"code":"Model.AccessDenied"},"id":"chatcmpl-3f0d5d50-fc5c-46ad-8e3d-494b33955c73","request_id":"3f0d5d50-fc5c-46ad-8e3d-494b33955c73"}

=== Grok (XAI_API_KEY) ===
  base_url      : https://api.x.ai/v1
  default model : grok-4.6
  -- model list probe --
  models returned: 12
  live fetch    : YES — differs from curated fallback
  sample        : grok-4.20-0309-non-reasoning, grok-4.20-0309-reasoning, grok-4.20-multi-agent-0309, grok-4.3, grok-4.5, grok-4.6, grok-build-0.1, grok-imagine-image, … (+4 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 4 chars; tokens prompt=637 completion=1 total=683
  RESULT        : PASS

=== Gemini (GEMINI_API_KEY) ===
  base_url      : https://generativelanguage.googleapis.com/v1beta
  default model : gemini-3.6-flash
  -- model list probe --
  models returned: 50
  live fetch    : YES — differs from curated fallback
  sample        : antigravity-preview-05-2026, aqa, deep-research-max-preview-04-2026, deep-research-preview-04-2026, deep-research-pro-preview-12-2025, gemini-2.5-computer-use-preview-10-2025, gemini-2.5-flash, gemini-2.5-flash-image, … (+42 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 36 chars; tokens prompt=2 completion=14 total=146
  RESULT        : PASS

──────────────────────────────────────────
7 of 8 probes passed (4 vendors × 2 probes each; 0 model-list failures, 1 generate failures)
```

**Analysis:** no `[WARN]` line appears anywhere — the shipped-defaults configuration is correctly, silently healthy. Kimi, Grok and Gemini each PASS both probes (no regression from this plan's change). Qwen PASSES its model-list probe (92 live models, differing from the curated fallback) and FAILS only `generate()`, on the pre-existing `Model.AccessDenied` entitlement gap (`.planning/WINDOWS.md` id 21) — a failure this plan's classification correctly has no opinion about, since it is not a model-list-fetch failure at all.

## Gap closure status (for `/gsd-verify-work` reconciliation)

| Gap | Status | Evidence |
|---|---|---|
| G-17-4a (Grok's rejected sampling parameters) | Already closed (plan 17-18) | Not a target of this run; Run A/B's Grok PASS/PASS above is the regression check |
| G-17-4b (Kimi's fixed-temperature rejection) | Already closed (plan 17-19) | Not a target of this run; Run A/B's Kimi PASS/PASS above is the regression check |
| G-17-4c (Qwen base_url region mismatch) | **Already resolved on 2026-08-22 by live verification (plan 17-21) — NOT a target of this run** | `17-21-SUMMARY.md`; recorded here so a reconciliation pass does not re-diagnose it |
| G-17-4d (masked auth failure — this run's target) | **Diagnosability half CLOSED this run; the phase's four-vendor-live-PASS bar remains blocked on WINDOWS id 21, an unrelated account-entitlement gap** | This plan's tasks 1-3; see "Unmet Must-Have" above |

## Threat Flags

None — this plan's `<threat_model>` (T-17-91, T-17-92, T-17-93, T-17-94, T-17-SC-22) already covers every file touched. The base URL written into the new `warn` line is stripped of userinfo (T-17-91, tested); the vendor error text embedded alongside it was already redacted before this plan (T-17-92, accepted exposure increase, stated explicitly rather than mitigated on paper); no package-manager install occurred (T-17-SC-22); the scope boundary around D-13/D-14 was not crossed (T-17-94, pinned by two new tests on the returned value).

## Known Stubs

None — no stub code was written. Every deliverable in this plan is a real, tested implementation; the one thing this plan does NOT deliver (Qwen's four-probe PASS) is reported above as blocked-on-WINDOWS-21, not stubbed or silently narrowed.

## Self-Check: PASSED

- FOUND: crates/paladin-llm/src/compat/engine.rs (FetchFailureClass, classify_fetch_failure, base_url_without_userinfo, rewired available_models Err arm, 16 new tests)
- FOUND: crates/paladin-llm/examples/live_vendor_smoke.rs (StderrLogger, log::set_logger installation)
- FOUND: crates/paladin-llm/src/qwen/adapter.rs (Region default symptom paragraph amended, 17-22 forward reference removed)
- FOUND: docs/src/getting-started/configuration.md ("A rejected credential now announces itself" section)
- FOUND commit 510154a (feat: Task 1, classification + rewired available_models)
- FOUND commit 83b041a (feat: Task 2, harness logger + live runs)
- FOUND commit 48b1600 (docs: Task 3, symptom paragraph + configuration guide)
- cargo test -p paladin-llm --features kimi,qwen,grok,ollama,openai-compatible,gemini: 259 passed, 0 failed
- cargo test --test unit --features llm-all: 428 passed, 0 failed, 11 ignored
- cargo fmt --check: clean
- cargo clippy --workspace --all-targets --features llm-all -- -D warnings: clean
- git diff --stat on Cargo.toml and crates/paladin-llm/Cargo.toml: empty (no dependency added)
- Live harness Run A: [WARN] line present, names the mismatched endpoint, no credential in output
- Live harness Run B: no [WARN] line, Kimi/Grok/Gemini PASS both probes, Qwen model-list PASS / generate FAIL (WINDOWS id 21)
- git status confirmed clean apart from this SUMMARY before it was written

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-22*
