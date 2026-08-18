---
status: testing
phase: 17-additional-llm-provider-adapters
source: [17-VERIFICATION.md]
started: 2026-08-18T02:35:00Z
updated: 2026-08-18T16:40:00Z
---

## Current Test

number: 2
name: Workspace coverage floor (82%, ADR-0006)
expected: |
  `cargo llvm-cov` reports >= 82% workspace line coverage with all nine adapters' code
  and every gap-closure regression test counted, not excluded.
awaiting: user response

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
command: `make coverage` (or `cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82`) with Redis + MinIO reachable via Docker and all nine provider features compiled in.
why_human: No Docker daemon in the verification sandbox; `make coverage`'s preflight fails fast on unreachable Redis (6380) and MinIO (9010). Genuinely UNMEASURED, not failing. Tracked as `WINDOWS.md` id 13. This is the item blocking PROV-04, whose own text names the 82% floor explicitly.
result: [pending]

### 3. Ollama integration test against a real container

expected: All 4 tests (generate round-trip, streaming, `get_available_models`, `validate_model`) exercise the real server and pass with real token-usage and model-list data, not the SKIP path.
command: `docker compose -f docker/docker-compose.test.yml up ollama-test ollama-test-init`, then `cargo test -p paladin-ai --no-default-features --features integration-tests,llm-ollama --test ollama_docker`
why_human: No Docker daemon in the sandbox. The suite gracefully SKIPs with a named `SKIP:` message rather than failing or silently passing. Tracked as `WINDOWS.md` id 12.
result: [pending]

### 4. Live vendor smoke test — Kimi, Qwen, Grok, Gemini

expected: Each vendor's documented `base_url` resolves, the default model ID exists, and `get_available_models()`'s live-fetch path (not just the curated fallback) returns a real, well-formed model list.
why_human: No network egress and no vendor API keys in the sandbox. `README.md`, `config.example.yml` and `docs/src/getting-started/configuration.md` all carry an explicit "not verified against a live endpoint" caveat — these facts are taken from vendor documentation and have never been confirmed live.
result: [pending]

### 5. New CI job behaviour on a real GitHub Actions runner

expected: The `llm-registry-unit-tests` job runs `cargo test --test unit --features llm-all`, passes (428 tests, matching the local reproduction), and a deliberate failure in it correctly fails `feature-matrix-summary`.
why_human: No GitHub Actions runner in the sandbox. Only the YAML's structural validity, the `needs:` dependency edge, and the underlying test command's local result were confirmed — the job's actual behaviour on a runner is unobserved.
result: [pending]

## Summary

total: 5
passed: 1
issues: 0
pending: 4
skipped: 0
blocked: 0

## Gaps

## Deferred Follow-Ups

- test: 1
  idea: "Evaluate a Rust-capable SAST (CodeQL Rust, Semgrep) to replace the removed Snyk. The
    project currently has no static taint analysis for first-party Rust; dependency scanning
    (cargo-audit/cargo-deny) and linting (clippy) do not cover it."
  deferred_at: 2026-08-18
- test: 1
  idea: "Amend .github/instructions/snyk_rules.instructions.md and the CLAUDE.md import that
    mandates a Snyk scan for new first-party code — the mandate is now unsatisfiable. Stale
    Snyk references also remain in .devcontainer/CI-CD.md (snyk/actions/rust@master) and
    .devcontainer/FILES.md."
  deferred_at: 2026-08-18
