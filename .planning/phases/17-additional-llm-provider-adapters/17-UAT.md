---
status: partial
phase: 17-additional-llm-provider-adapters
source: [17-VERIFICATION.md]
started: 2026-08-18T02:35:00Z
updated: 2026-08-19T15:40:00Z
---

## Current Test

[testing complete — 4 passed; only test 4 blocked, on vendor API credentials]

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
result: blocked
blocked_by: third-party
reason: "No vendor credentials. Every API key in .env is empty (OPENAI, ANTHROPIC, DEEPSEEK, XAI, GEMINI) and LLM_API_KEY is a placeholder; Kimi and Qwen have no entry at all. Network egress IS available, so this unblocks the moment real keys are supplied."

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
issues: 0
pending: 0
skipped: 0
blocked: 1

## Gaps

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
