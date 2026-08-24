---
phase: 17
slug: additional-llm-provider-adapters
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-17
---

# Phase 17 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (workspace default) + `cargo llvm-cov` for coverage (`Makefile:257`) |
| **Config file** | none dedicated — coverage gate is inline in `Makefile:261` and `.github/workflows/ci.yml:664` (`cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82`) |
| **Quick run command** | `cargo test -p paladin-llm` |
| **Full suite command** | `make coverage` (requires `make services-up`; extend for the Ollama Tier 2 service) |
| **Estimated runtime** | ~30s quick (crate-scoped, offline) / several minutes full |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p paladin-llm`
- **After every plan wave:** Run `cargo build -p paladin-llm --no-default-features --features <each-provider-touched>` plus `cargo test --workspace` (default features)
- **Before `/gsd-verify-work`:** `make coverage` full suite must be green (≥82% workspace line coverage)
- **Max feedback latency:** 60 seconds for the per-task quick run

---

## Per-Task Verification Map

> Seeded at plan time from RESEARCH.md's requirement→test map. Task IDs are filled in by
> `/gsd-execute-phase` / `/gsd-validate-phase` once plans are executed; the requirement rows below
> are the binding contract.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | PROV-01 | — | N/A | documentation | n/a — recorded provider-selection study, verified by human review | N/A | ⬜ pending |
| TBD | TBD | TBD | PROV-02 | — | Capability response reports only what the provider genuinely does (no optimistic flags) | unit | `cargo test -p paladin-llm --features <all-new-providers>` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PROV-02 | — | Capability/request-surface correspondence holds for new adapters | unit | extended `capability_invariants` test in `crates/paladin-llm/src/lib.rs` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PROV-03 | — | N/A | build/smoke | `cargo build -p paladin-llm --no-default-features --features <provider>` (× each new provider) | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PROV-03 | — | Compiled-out providers are absent from `list_available_providers` (no phantom advertising) | unit | extend `tests/unit/llm/provider_factory_test.rs` (282 lines today) | ⚠️ needs cases | ⬜ pending |
| TBD | TBD | TBD | PROV-03 | — | Existing config files keep loading — no breaking change to `LlmConfig` | unit | extend `#[cfg(test)]` in `crates/paladin-llm/src/config/llm.rs` | ⚠️ needs cases | ⬜ pending |
| TBD | TBD | TBD | PROV-04 | T-17-* | API keys never appear in logs or error payloads (credential redaction) | unit | mock-transport tests per provider — placement per plan decision | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PROV-04 | — | N/A | integration | Ollama Docker-gated Tier 2 suite via `make test-integration-docker` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PROV-04 | — | N/A | coverage gate | `make coverage` (≥82% lines) | ✅ gate exists | ⬜ pending |
| TBD | TBD | TBD | PROV-04 | — | N/A | lint | `cargo doc` + `#![warn(missing_docs)]` (`lib.rs:41`) | ✅ enforced | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Shared OpenAI-compatible core module (`crates/paladin-llm/src/compat/` or equivalent) and its `#[cfg(test)]` module — covers PROV-02
- [ ] Per-preset adapter modules (`kimi/`, `qwen/`, `grok/`, `ollama/`, `openai_compatible/`, `gemini/`) with their `#[cfg(test)]` modules — covers PROV-02
- [ ] `mockito` as a `paladin-llm` crate-local dev-dependency **if** mock-transport tests land crate-local (currently only a root-workspace dev-dependency) — Open Question 1
- [ ] New `ollama-test` service block in `docker/docker-compose.test.yml` (image `ollama/ollama`, small pulled model) — covers D-15
- [ ] Docker-gated Tier 2 suite file (e.g. `tests/integration/ollama_docker_test.rs`)
- [ ] New cases in `tests/unit/llm/provider_factory_test.rs` for the D-10 registry table
- [ ] Extended `capability_invariants` test module in `crates/paladin-llm/src/lib.rs` — Open Question 3

*Framework install: none — `cargo test` / `cargo llvm-cov` are already set up workspace-wide.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Provider-selection study records build/defer/reject with reasons for every named candidate | PROV-01 | A recorded decision document is verified by reading it; there is no automated assertion for "the reasoning is sound" | Read the recorded study; confirm Kimi, Gemini, Qwen and Meta/Llama are each explicitly dispositioned and that the Llama row names a concrete host or is rejected for lacking one |
| Live-API behavior of each hosted provider | PROV-04 | Requires real credentials; CI must stay green without secrets | Run the credential-gated feature's tests locally with the provider's key set |
| Advertised surface matches shipped surface (`Cargo.toml` description/keywords, crate README, config docs) | PROV-04 | Cross-document currency check | Read the three surfaces side by side against the shipped feature list |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
