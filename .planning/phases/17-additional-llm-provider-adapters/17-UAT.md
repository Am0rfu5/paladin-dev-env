---
status: diagnosed
phase: 17-additional-llm-provider-adapters
source: [17-VERIFICATION.md]
started: 2026-08-18T02:35:00Z
updated: 2026-08-22T03:05:00Z
---

## Current Test

[testing complete — 4 passed, 1 issue (test 4, blocker); G-17-4c resolved, G-17-4d opened]

## Tests

### 1. Security scan over every file this phase modified

expected: The project's static-analysis and dependency-security gates report no unresolved issues on the modified files.
result: pass
verified: 2026-08-18 (rerun)
tool_change: |
  The original expectation named Snyk. Snyk has been REMOVED from the project — it provides no
  coverage for Rust. Evidence: a probe file carrying four textbook vulnerabilities (hardcoded
  credential, command injection via `sh -c`, path traversal, SQL injection) returned 0 findings
  from `snyk code test`; the same logic in JavaScript returned 3 findings (HIGH/MEDIUM/LOW),
  confirming the scanner and auth worked and the gap is Rust rule coverage. `snyk test` (Snyk
  Open Source) has no Cargo support at all and exits SNYK-CLI-0008 on this workspace. The
  earlier "556 Rust files, 0 issues" result was therefore vacuous — not evidence of clean code.
evidence: |
  - `make security` -> exit 0. `cargo audit`: 1217 advisories loaded, 677 crate dependencies
    scanned, no vulnerabilities (9 pre-existing allowlisted warnings per `.cargo/audit.toml`).
    `cargo deny`: "advisories ok, bans ok, licenses ok, sources ok".
  - This gate was previously DOWN and masking a real advisory. Two defects fixed to restore it:
    RUSTSEC-2026-0258 (`h2` 0.4.14 -> 0.4.16, unbounded empty DATA frames) in commit 82b1a9e,
    and a stale untracked advisory-db file that broke DB loading, self-healed in commit f93d306.
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` -> exit 0.
  - `cargo check --workspace --all-targets` -> exit 0.
  - Targeted credential review of the five in-scope files: `compat/engine.rs` applies
    `diagnostic_excerpt` (redact-then-bound) at every error path — generate (436), stream (564),
    model-list (658) and body-read (457); `gemini/adapter.rs` applies it at 3 sites;
    `openai_compatible/adapter.rs` and `provider_factory.rs` perform no HTTP and need none.
    No log statement interpolates an `api_key`; `LlmProviderConfig` is never `Debug`-formatted
    or serialized outward. Presets set `redirect_policy: Policy::none()` so a 3xx cannot forward
    the credential header to an attacker-influenced host (WR-04).
residual_gap: |
  The project now has NO SAST tool for Rust. `cargo-audit`/`cargo-deny` are dependency scanners
  and `clippy` is a lint, so neither is a substitute for taint analysis of first-party code.
  Recorded as a deferred follow-up, not a Phase 17 blocker — no first-party defect was found by
  any available means, and evaluating a Rust-capable SAST (CodeQL, Semgrep) is its own scope.
scope: `crates/paladin-llm/src/provider_factory.rs`, `crates/paladin-llm/src/openai_compatible/adapter.rs`, `crates/paladin-llm/src/gemini/adapter.rs`, `crates/paladin-llm/src/compat/engine.rs`, `tests/unit/llm/provider_factory_test.rs`

### 2. Workspace coverage floor (82%, ADR-0006)

expected: `cargo llvm-cov` reports >= 82% workspace line coverage with all nine adapters' code and every gap-closure regression test counted, not excluded.
result: pass
verified: 2026-08-19
measured: |
  85.01% workspace line coverage (46180/54326 lines), 3055 tests passed, 0 failed,
  `cargo llvm-cov --workspace --features integration-tests,llm-all --fail-under-lines 82` exit 0.
  All nine adapters instrumented: gemini 93.4%, grok 95.7%, kimi 98.3%, qwen 95.7%,
  ollama 95.7%, openai-compatible 96.2%, compat core 85.4%, provider_factory 81.6%
  (pre-existing: anthropic 81.7%, deepseek 83.5%, openai 32.3%).
was_not_actually_blocked: |
  Recorded as "genuinely UNMEASURED — no Docker daemon". That was a misdiagnosis. Redis, MinIO
  and MySQL are all running and reachable from inside the devcontainer as compose peers
  (redis:6379 -> PONG, minio:9000 -> HTTP 200, mysql:3306 open). What fails is `make coverage`'s
  preflight, which probes localhost:6380/9010 — the HOST-mapped ports of the separate `docker/`
  services stack, unreachable from inside this container. Pointing REDIS_HOST/MINIO_ENDPOINT at
  the real peers lets the CI coverage command run verbatim.
finding: |
  DEFECT FOUND — the coverage gate measures the wrong codebase. `make coverage` and CI's
  `coverage` job both run `--features integration-tests`, which resolves to
  `default = ["llm-openai","llm-anthropic","llm-deepseek"]` — the three adapters that existed
  BEFORE this phase. The six built by Phase 17 are behind non-default flags, so they are never
  compiled, never instrumented, and contribute zero lines. Measured both ways to prove it:
  default features 84.32% over 49209 lines with those six showing 0 instrumented files;
  `llm-all` 85.01% over 54326 lines (+5117) with all nine present. The gate therefore passes
  while ignoring the phase's entire deliverable, and would not catch a regression in any new
  adapter. Coverage itself is NOT the problem — the new code is the best-covered in the crate.
  Tracked as a follow-up below. RESOLVED 2026-08-19: both `make coverage` and CI's `coverage`
  job now call `scripts/coverage.sh` with `--features integration-tests,llm-all`. CI evidence
  (run 32269584177): "Lines: 46100/54326 = 84.86%" — the 54326 denominator matches the local
  llm-all measurement exactly and is +5117 over the pre-fix 49209, i.e. the six new adapters
  are now counted. A second defect surfaced on the first CI attempt: the script probed Redis
  with `redis-cli`, which GitHub runners do not ship, so it reported "Redis unreachable" while
  Redis was healthy as a service container. Probes now fall back to a TCP connect via
  bash /dev/tcp. Note CI's own pre-existing readiness loop hits the same missing binary
  ("redis-cli: command not found" x30) but is best-effort, so it silently spins instead.

### 3. Ollama integration test against a real container

expected: All 4 tests (generate round-trip, streaming, `get_available_models`, `validate_model`) exercise the real server and pass with real token-usage and model-list data, not the SKIP path.
result: pass
verified: 2026-08-19 on a GitHub Actions runner (ci.yml run 32269584177, job 96122463095)
evidence: |
  CI never ran this suite before: the `docker-integration` job starts only redis-test and
  minio-test, and no workflow referenced ollama-test. A new `ollama-integration` job was added
  to close that gap (commit ca21164 / earlier).
  - Live model list from the real server: `{"object":"list","data":[{"id":"qwen2.5:0.5b",...}]}`
  - `running 4 tests` ... `test result: ok. 4 passed; 0 failed` in 1.79s:
      validate_model_distinguishes_pulled_from_unpulled
      get_available_models_returns_the_pulled_model
      generate_round_trip_returns_nonempty_content_and_real_token_usage
      generate_stream_produces_multiple_chunks_with_nonempty_concatenation
  - Guard output: "All ollama_docker tests exercised the live server."
not_a_vacuous_pass: |
  The suite SKIPS-and-PASSES when the server is unreachable, by design, so a plain green job
  would prove nothing. The job therefore asserts qwen2.5:0.5b is in the live model list and
  FAILS if any `SKIP:` appears in the output. Both guards held, so the four tests ran against
  a real Ollama server rather than short-circuiting.

### 4. Live vendor smoke test — Kimi, Qwen, Grok, Gemini

expected: Each vendor's documented `base_url` resolves, the default model ID exists, and `get_available_models()`'s live-fetch path (not just the curated fallback) returns a real, well-formed model list.
result: issue
verified: 2026-08-22 against the real vendor endpoints
reported: "Ran the live smoke test with real credentials. 1 of 4 vendors passed. Grok cannot generate against any current model; Kimi's default model is retired and current models reject the default temperature; Qwen's credential is rejected 401 by both endpoints."
severity: blocker
harness: |
  `crates/paladin-llm/examples/live_vendor_smoke.rs` (new, gated behind
  `required-features = ["kimi","qwen","grok","gemini"]`, never run in CI). Run with:
      cargo run -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini
  It does NOT accept a non-empty model list as proof of a live fetch. `available_models()`
  swallows every failure and returns the curated `*_FALLBACK_MODELS` constant (engine.rs:682),
  so a plausible list is not evidence the network path worked — exactly the vacuous-pass trap
  flagged for test 3. The harness discriminates by comparing the result byte-for-byte against
  that constant, then asserts the adapter's default model ID appears in the live list.
credentials: |
  Host-mounted keys resolved for all four vendors via the devcontainer import
  (`~/.config/paladin` bind-mount -> `.devcontainer/paladin-env.sh`): GEMINI_API_KEY,
  XAI_API_KEY, MOONSHOT_API_KEY, DASHSCOPE_API_KEY. `paladin-keys` reports 4 of 8 available.
  Note the loader only reaches INTERACTIVE shells (`~/.bashrc` returns early when
  non-interactive), so non-interactive runners must source `.devcontainer/paladin-env.sh`
  explicitly. No credential value was printed at any point.
measured: |
  Gemini  PASS    50 models live, differs from fallback, default `gemini-2.5-flash` present.
  Kimi    FAIL     4 models live (kimi-k2.6, kimi-k2.7-code, kimi-k2.7-code-highspeed, kimi-k3);
                   default `moonshot-v1-8k` ABSENT.
  Grok    FAIL    12 models live (grok-4.3, grok-4.5, grok-4.6, grok-4.20-*, ...);
                   default `grok-4` ABSENT.
  Qwen    FAIL     result byte-identical to QWEN_FALLBACK_MODELS -> live fetch silently failed.
findings: |
  G-17-4a GROK IS NON-FUNCTIONAL OUT OF THE BOX (blocker). Not a stale-model-ID problem.
    `CompatEngine::build_request` (engine.rs:207) unconditionally serialises `presence_penalty`,
    and the default prompt parameters set it to `Some(0.0)` (paladin-core prompt.rs:154), so it
    is always on the wire. xAI rejects the PARAMETER, for every current model tested:
      grok-4   -> {"code":"invalid-argument","error":"Model grok-4 does not support parameter presencePenalty."}
      grok-4.6 -> same    grok-4.5 -> same    grok-4.3 -> same
    Correcting `GROK_DEFAULT_MODEL` alone does NOT fix this — all four models fail identically.
    The adapter cannot complete a single generate call with default parameters.
  G-17-4b KIMI IS NON-FUNCTIONAL OUT OF THE BOX (blocker). Two independent causes:
    1. `KIMI_DEFAULT_MODEL = "moonshot-v1-8k"` is retired ->
       {"message":"Not found the model moonshot-v1-8k or Permission denied","type":"resource_not_found_error"}
    2. Even against a live-listed model, the default temperature is rejected:
       kimi-k2.6 -> {"message":"invalid temperature: only 1 is allowed for this model"}
       Default prompt parameters send temperature 0.7 (prompt.rs:151).
    So the model-ID fix is necessary but not sufficient; the temperature constraint must also
    be honoured (the preset carries `temperature_range`, engine.rs:64, but it is not enforced).
  G-17-4c QWEN — SUPERSEDED 2026-08-22, see the corrected entry under `## Gaps`. The live path
    is now VERIFIED (92 models via the US Virginia endpoint) and the credential is VALID. The
    conclusion below — that the base_url was confirmed correct and only auth failed — was WRONG:
    DashScope keys are region-scoped, so a well-formed 401 is returned by every endpoint except
    the key's own region, and cannot be read as confirming the URL. Superseded by gap G-17-4d,
    which records the real defect: the shipped default hardcodes one region.
  ORIGINAL (retained for the record, conclusion since falsified):
    Both endpoints return 401 `invalid_api_key` with the documented Alibaba error envelope:
      https://dashscope-intl.aliyuncs.com/compatible-mode/v1/models  -> 401 invalid_api_key
      https://dashscope.aliyuncs.com/compatible-mode/v1/models       -> 401 invalid_api_key
    The base_url is therefore CONFIRMED CORRECT (it resolves, TLS-terminates and returns the
    vendor's documented OpenAI-compatible error schema); what fails is authentication. The
    stored `dashscope_api_key` is a single 117-char token with no `=`, whitespace or newline,
    so the loader is not corrupting it — it is structurally clean but not accepted. DashScope
    keys are conventionally `sk-` + 32 hex (~35 chars), so 117 chars suggests the wrong secret
    is in the file. Needs a valid key before Qwen's default model ID can be verified at all.
observation: |
  The silent-fallback design (D-13/D-14) hid Qwen's 401 completely: `get_available_models()`
  returned a plausible 3-model list with no error surfaced at any level above `debug`. This is
  documented intentional behaviour, not a defect, but it is why the original UAT expectation
  demanded the live-fetch path specifically. Worth considering whether `available_models()`
  should expose which path produced the result.
scope: `crates/paladin-llm/src/compat/engine.rs`, `crates/paladin-llm/src/grok/adapter.rs`, `crates/paladin-llm/src/kimi/adapter.rs`, `crates/paladin-llm/src/qwen/adapter.rs`, `crates/paladin-llm/examples/live_vendor_smoke.rs`

### 5. New CI job behaviour on a real GitHub Actions runner

expected: The `llm-registry-unit-tests` job runs `cargo test --test unit --features llm-all`, passes (428 tests, matching the local reproduction), and a deliberate failure in it correctly fails `feature-matrix-summary`.
result: pass
verified: 2026-08-19 on a GitHub Actions runner (feature-flags.yml run 32269584207, also 32262069917)
evidence: |
  - Job "LLM Registry Unit Tests (llm-all)" -> success on ubuntu-latest, twice (two pushes).
  - Its run step is exactly `cargo test --test unit --features llm-all`
    (`.github/workflows/feature-flags.yml:172`).
  - Local reproduction of the same command: 428 passed, 0 failed, 11 ignored, exit 0.
  - `feature-matrix-summary` declares `needs: [feature-matrix, cli-isolation,
    llm-registry-unit-tests]` and succeeded alongside it.
scope_note: |
  The failure-propagation half is verified STRUCTURALLY, not empirically: the summary job is
  `if: always()` with a conjunction requiring all three `needs.*.result` values to equal
  "success", else `exit 1` (feature-flags.yml:194-210). A deliberate failure was NOT injected —
  doing so would mean pushing a knowingly broken commit. The logic admits no other outcome, but
  recording the distinction rather than implying it was observed.

## Summary

total: 5
passed: 4
issues: 1
pending: 0
skipped: 0
blocked: 0

## Gaps

- gap_id: G-17-4a
  truth: "Grok's adapter can complete a generate call against a current xAI model with default parameters"
  status: failed
  reason: "Live run 2026-08-22: every current model (grok-4, grok-4.6, grok-4.5, grok-4.3) rejects the request with 'does not support parameter presencePenalty'. The adapter is non-functional out of the box."
  severity: blocker
  test: 4
  root_cause: "CompatEngine::build_request unconditionally serialises presence_penalty (compat/engine.rs:207) from default prompt parameters (paladin-core prompt.rs:154, Some(0.0)). xAI rejects the parameter's presence for all grok-4.x models. GROK_DEFAULT_MODEL='grok-4' is additionally absent from the live model list."
  artifacts:
    - path: "crates/paladin-llm/src/compat/engine.rs"
      issue: "presence_penalty / frequency_penalty serialised unconditionally; no per-preset parameter-support gate"
    - path: "crates/paladin-llm/src/grok/adapter.rs"
      issue: "GROK_DEFAULT_MODEL 'grok-4' and GROK_FALLBACK_MODELS ['grok-4','grok-3'] are both stale vs the live list"
  missing:
    - "Suppress presence_penalty (and verify frequency_penalty) for the xAI preset, or make unsupported-parameter omission a preset capability"
    - "Refresh GROK_DEFAULT_MODEL and GROK_FALLBACK_MODELS against the live list (grok-4.6 / grok-4.5 / grok-4.3)"
    - "A regression test asserting the xAI wire body omits presencePenalty"
  debug_session: ""

- gap_id: G-17-4b
  truth: "Kimi's adapter can complete a generate call using its default model and default parameters"
  status: failed
  reason: "Live run 2026-08-22: default model moonshot-v1-8k returns resource_not_found_error; a live-listed model (kimi-k2.6) rejects the default temperature with 'only 1 is allowed for this model'."
  severity: blocker
  test: 4
  root_cause: "Two independent causes. (1) KIMI_DEFAULT_MODEL='moonshot-v1-8k' is retired by Moonshot. (2) Current Kimi models accept only temperature=1, but default prompt parameters send 0.7 (paladin-core prompt.rs:151); the preset's temperature_range (compat/engine.rs:64) is declared but never enforced against the outgoing request."
  artifacts:
    - path: "crates/paladin-llm/src/kimi/adapter.rs"
      issue: "KIMI_DEFAULT_MODEL and KIMI_FALLBACK_MODELS reference the retired moonshot-v1-* family"
    - path: "crates/paladin-llm/src/compat/engine.rs"
      issue: "temperature_range is carried on the preset but never applied when building the request"
  missing:
    - "Refresh KIMI_DEFAULT_MODEL and KIMI_FALLBACK_MODELS against the live list (kimi-k2.6, kimi-k2.7-code, kimi-k2.7-code-highspeed, kimi-k3)"
    - "Enforce or clamp temperature to the preset's temperature_range before sending"
    - "A regression test covering the fixed-temperature constraint"
  debug_session: ""

- gap_id: G-17-4c
  truth: "Qwen's live get_available_models path returns a real model list containing the default model"
  status: resolved
  resolved_by: "live verification 2026-08-22 against the US (Virginia) endpoint"
  resolved_at: 2026-08-22
  reason: "RESOLVED. Re-run with DASHSCOPE_BASE_URL=https://dashscope-us.aliyuncs.com/compatible-mode/v1 returns 92 live models, differs from the curated fallback, and QWEN_DEFAULT_MODEL 'qwen-plus' is present. The stored credential is VALID; the shipped default base_url pointed at the wrong region."
  severity: major
  test: 4
  correction: |
    This gap's original diagnosis reached the WRONG CONCLUSION and is superseded. It recorded
    "base_url is CONFIRMED CORRECT ... only authentication fails", reasoning that a 401 carrying
    Alibaba's documented error envelope proved the URL was right and implicated the key. That
    inference does not hold. Alibaba documents that "a Base URL must be used together with an
    API Key from the same billing plan; otherwise, a 401 error occurs. API Keys are independent
    across regions and cannot be used across regions." A region-scoped key therefore returns a
    well-formed 401 from every endpoint EXCEPT its own — which is exactly the signature observed
    on both intl (Singapore) and mainland. The 117-char length, also cited as evidence of a
    wrong secret, was a red herring: the key is valid.
    Credit for the correction: the operator identified that their Model Studio workspace is set
    to US (Virginia).
  measured: |
    Same credential, same binary, same run, only DASHSCOPE_BASE_URL differing:
      A) https://dashscope-intl.aliyuncs.com/compatible-mode/v1  (shipped default, Singapore)
         -> 3 models, byte-identical to QWEN_FALLBACK_MODELS -> live fetch silently failed -> FAIL
      B) https://dashscope-us.aliyuncs.com/compatible-mode/v1    (US Virginia)
         -> 92 models, differs from fallback, qwen-plus present -> PASS
  harness_defect_fixed: |
    The first Virginia run appeared to pass while printing the INTL base_url, because the
    harness printed the `*_DEFAULT_BASE_URL` constant rather than the URL the run actually
    used — it would have attributed a result to the wrong endpoint. `live_vendor_smoke.rs` now
    reads `base_url` back off the resolved config and flags an override explicitly.

- gap_id: G-17-4d
  truth: "An operator whose Alibaba Model Studio workspace is in any region other than Singapore can use the Qwen adapter's shipped defaults and reach their own account"
  status: failed
  reason: "Discovered 2026-08-22 while resolving G-17-4c. QWEN_DEFAULT_BASE_URL hardcodes the Singapore/intl endpoint, but DashScope API keys are region-scoped and are rejected 401 by every other region's endpoint. Any operator on US (Virginia), Tokyo, Hong Kong or mainland gets a silent failure with the shipped defaults."
  severity: major
  test: 4
  root_cause: "The adapter models DashScope as a single global endpoint. Alibaba operates per-region endpoints with region-scoped credentials (`dashscope-us.aliyuncs.com` for US Virginia, `dashscope-intl.aliyuncs.com` for Singapore, `dashscope.aliyuncs.com` for mainland, plus workspace-dedicated `{workspace-id}.{region}.maas.aliyuncs.com` domains recommended for production). The failure is invisible because `available_models()` swallows the 401 and returns the curated fallback, so the adapter reports a plausible 3-model list instead of an auth error — the same masking that hid G-17-4c for five days."
  artifacts:
    - path: "crates/paladin-llm/src/qwen/adapter.rs"
      issue: "QWEN_DEFAULT_BASE_URL hardcodes one region; nothing documents that the key must match it"
    - path: "crates/paladin-llm/src/compat/engine.rs"
      issue: "available_models() reports an auth failure identically to an offline failure, masking a region/credential mismatch"
  missing:
    - "Document the region-scoped-credential constraint wherever DASHSCOPE_BASE_URL is described, naming the known regional endpoints"
    - "Decide whether the shipped default stays Singapore, and make the mismatch diagnosable rather than silent"
    - "Consider distinguishing an auth failure (misconfiguration, warrants `warn`) from an offline failure (supported state, `debug`) in available_models()"
  debug_session: ""

## Deferred Follow-Ups

- test: 1
  idea: "Evaluate a Rust-capable SAST (CodeQL Rust, Semgrep) to replace the removed Snyk. The
    project currently has no static taint analysis for first-party Rust; dependency scanning
    (cargo-audit/cargo-deny) and linting (clippy) do not cover it."
  deferred_at: 2026-08-18
- test: 1
  status: RESOLVED 2026-08-19 (tracked .github/instructions/security.instructions.md; WINDOWS.md 15-18 waived)
  idea: "Amend .github/instructions/snyk_rules.instructions.md and the CLAUDE.md import that
    mandates a Snyk scan for new first-party code — the mandate is now unsatisfiable. Stale
    Snyk references also remain in .devcontainer/CI-CD.md (snyk/actions/rust@master) and
    .devcontainer/FILES.md."
  deferred_at: 2026-08-18
- test: 2
  status: RESOLVED 2026-08-19 (scripts/coverage.sh; CI run 32269584177 measures 54326 lines)
  idea: "Fix the coverage gate to measure all nine adapters. `make coverage` and CI's `coverage`
    job run `--features integration-tests`, which only enables the three default adapters, so the
    six added by Phase 17 contribute zero instrumented lines. Change to
    `--features integration-tests,llm-all`. Also fix the preflight, which probes localhost:6380/9010
    and cannot succeed from inside the devcontainer where the services are redis:6379 / minio:9000."
  deferred_at: 2026-08-19
