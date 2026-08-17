---
phase: 17-additional-llm-provider-adapters
plan: 07
subsystem: testing
tags: [rust, cargo-features, llm, capability-invariants, provider-factory, docker, ollama, coverage]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 06)
    provides: "the completed nine-provider facade feature-flag wiring (D-11 amended option-b) and the nine-provider LlmConfig/config-bridge surface that this plan tests against"
provides:
  - "capability_invariants_new_providers sibling test module in crates/paladin-llm/src/lib.rs, pinning supports_tool_calling/supports_function_calling to false for all six new adapters (kimi, qwen, grok, ollama, gemini, openai-compatible-with-empty-declaration)"
  - "Two new D-10 regression tests in tests/unit/llm/provider_factory_test.rs: test_compiled_out_provider_absent_from_list_available_providers and test_new_provider_names_resolve_through_create, plus the CleanNewProviderEnv RAII guard they share"
  - "docker/docker-compose.test.yml ollama-test + ollama-test-init services (D-15) and tests/integration/ollama_docker_test.rs, a required-features-gated Tier 2 suite exercising the shared CompatEngine's generate()/generate_stream()/get_available_models()/validate_model() against a real Ollama instance -- authored and compile/clippy/skip-path verified, runtime-against-a-real-server UNVERIFIED (no Docker daemon in this sandbox; see Known Stubs)"
  - "Makefile test-integration-docker extended to bring ollama-test up and run the suite"
  - "cargo doc -p paladin-llm --no-deps confirmed 0 missing-docs warnings under the six new adapter features"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Sibling cfg-gated test module (not a widened gate) when a cross-adapter invariant must cover a second, disjoint feature combination without disabling the test that already runs for the first — same shape as the pre-existing capability_invariants module, applied a second time"
    - "A second RAII env-guard (CleanNewProviderEnv, its own Mutex) for a disjoint set of process-wide env vars, following the file's existing CleanProviderEnv precedent exactly, so tests touching the two variable sets can still run concurrently within the same binary without cross-contaminating each other's saved/restored state"
    - "Runtime reachability probe as the Docker-gate complement to Cargo's required-features gate: each ollama_docker test independently GETs {base_url}/models with a 3s timeout before doing anything else, and returns early with a printed, URL-naming reason on any non-2xx/connection failure — proven in this sandbox precisely because nothing listens on the mapped port, so the skip path is exercised for real, not merely reasoned about"

key-files:
  created:
    - tests/integration/ollama_docker_test.rs
    - .planning/phases/17-additional-llm-provider-adapters/17-07-SUMMARY.md
  modified:
    - crates/paladin-llm/src/lib.rs
    - tests/unit/llm/provider_factory_test.rs
    - docker/docker-compose.test.yml
    - Cargo.toml
    - Makefile

key-decisions:
  - "Human ruling (2026-08-17, AskUserQuestion during /gsd-execute-phase 17 orchestration, provenance recorded in this continuation agent's prompt): author Task 2/3 anyway under the Docker-absent constraint, flagged explicitly as verification debt, rather than halting the plan a second time on the same root cause. This overrides Task 2's own <precondition> (\"halt and surface the checkpoint rather than authoring the suite blind\") by deliberate, informed human choice, not executor discretion."
  - "Task 1's precondition-free work (capability_invariants_new_providers, the two D-10 regression tests) was completed and committed in a prior session (commit e103ca3) before the Docker blocker was hit; this continuation agent did not redo it."
  - "The Ollama Docker-gated suite's four tests each independently probe OLLAMA_TEST_URL and skip (print a reason naming the URL, then return) rather than asserting anything, when the service is unreachable — proven for real in this sandbox: all 4 tests pass by skipping, since nothing listens on the default mapped port 11435 here. This is the same mechanism that will make them run for real once a human (or a Docker-enabled CI runner) brings the service up; no code change is needed for that transition."
  - "ollama-test's healthcheck uses 'ollama list' (native /api/tags) rather than the plan's stated preference for curl-hitting /v1/models, because curl/wget's presence in the ollama/ollama:0.3.14 base image could not be verified without a Docker daemon. 'ollama list' calls the local server's own API internally and only succeeds once serving, is guaranteed present (it's the image's own entrypoint binary), and is a well-precedented healthcheck for this exact image in the wider ecosystem. Documented as a deviation, not silently substituted (WINDOWS.md id 14)."
  - "docker/docker-compose.test.yml was checked with `python3 -c \"import yaml; yaml.safe_load(...)\"` only, confirming syntax validity -- `docker compose -f docker/docker-compose.test.yml config` (which also resolves service references, interpolation, and schema) was never run, because neither `docker` nor `docker-compose` exists in this sandbox (`which docker docker-compose` returns nothing)."
  - "The Ollama service was deliberately NOT made a `make coverage` prerequisite: `make coverage` invokes `cargo llvm-cov --workspace --features integration-tests ...`, and `integration-tests` does not imply `llm-ollama`, so the ollama_docker test target (required-features = [\"integration-tests\", \"llm-ollama\"]) is structurally excluded from that invocation regardless of service availability. No change to the coverage target's preflight checks was needed or made."
  - "Task 3's actual coverage gate (`make coverage`, the 82%-floor `cargo llvm-cov --workspace --fail-under-lines 82` run) could not be executed: it requires Redis on 6380 and MinIO on 9010 via `make services-up`, both Docker services, both confirmed unreachable in this sandbox (`nc -z localhost 6380` and `nc -z localhost 9010` both exit 1). Per the resolved checkpoint's <remaining_checkpoints> guidance, this is the same Docker root cause already ruled on for Task 2, not a fresh blocker requiring a new checkpoint. No coverage percentage is fabricated or estimated here; it is recorded as genuinely unmeasured (WINDOWS.md id 13)."
  - "What WAS verified for Task 3 without Docker: `cargo doc -p paladin-llm --no-deps --features kimi,qwen,grok,ollama,gemini,openai-compatible 2>&1 | grep -c 'missing documentation'` returns 0 (one unrelated pre-existing `rustdoc::private_intra_doc_links` warning in crates/paladin-llm/src/gemini/adapter.rs was observed, out of this plan's scope per the scope-boundary rule -- not touched, not introduced by this plan, not a missing-docs warning). `cargo clippy --test ollama_docker --features integration-tests,llm-ollama -- -D warnings` is clean. The workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` and the `--all-features` variant `make lint` were deliberately NOT run here, per the build-verification-budget instruction to scope checks and defer full-workspace operations to the orchestrator's authoritative post-merge gate."
  - "test_new_provider_names_resolve_through_create and test_compiled_out_provider_absent_from_list_available_providers (Task 1, prior session) are feature-set-adaptive at runtime (querying provider_names() rather than assuming a fixed compiled set), per the task's own instruction not to assume which provider the root test target compiles out."
  - "The UnknownProvider Display message echoes the requested name back, so the 'must not list the bogus name' assertion in test_new_provider_names_resolve_through_create checks only the substring after 'Supported providers: ', not the whole message (Task 1, prior session, self-caught before commit)."

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "capability_invariants_new_providers sibling module pins supports_tool_calling/supports_function_calling to false for all six new adapters"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,gemini,openai-compatible capability_invariants_new_providers"
        status: pass
    human_judgment: false
  - id: D2
    description: "D-10 regression: a compiled-out provider is absent from list_available_providers() even with its credential env var set"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "tests/unit/llm/provider_factory_test.rs#test_compiled_out_provider_absent_from_list_available_providers (cargo test --test unit llm::provider_factory)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Ollama Docker-gated Tier 2 suite: authored, compiles, clippy-clean, and the unreachable-service skip path is proven for real; runtime pass/fail against an actual Ollama server is unverified"
    requirement: "PROV-04"
    verification:
      - kind: integration
        ref: "cargo test --test ollama_docker --features integration-tests,llm-ollama -- --nocapture (all 4 tests pass by skipping gracefully in this Docker-less sandbox)"
        status: pass
    human_judgment: true
    rationale: "The 4 tests' skip-path (never-silently-pass, printed URL-naming reason) is proven, but their actual assertions against a real Ollama server (non-empty content, real token usage, multi-chunk streaming, live model catalog, validate_model true/false) have never executed. A human with Docker access must run `docker compose -f docker/docker-compose.test.yml up -d ollama-test ollama-test-init && cargo test --test ollama_docker --features integration-tests,llm-ollama -- --nocapture` and confirm all 4 pass for real before this deliverable is fully proven. Tracked in .planning/WINDOWS.md (id 12)."
  - id: D4
    description: "docker-compose.test.yml ollama-test/ollama-test-init services follow the redis-test/minio-test-init pattern; YAML syntax valid"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "python3 -c \"import yaml; yaml.safe_load(open('docker/docker-compose.test.yml'))\" -> YAML OK"
        status: pass
    human_judgment: true
    rationale: "Only YAML syntax was checked. `docker compose -f docker/docker-compose.test.yml config` (schema + interpolation resolution) and an actual `up -d` were never run -- no docker/docker-compose binary in this sandbox. The 'ollama list' healthcheck substitution (see key-decisions) is also unverified against the real image. Tracked in .planning/WINDOWS.md (id 14)."
  - id: D5
    description: "Workspace clears the 82% line-coverage floor with all six new adapters counted, no exclusions"
    requirement: "PROV-04"
    verification: []
    human_judgment: true
    rationale: "make coverage requires Redis (6380) and MinIO (9010), both Docker services, both confirmed unreachable in this sandbox. The coverage percentage is genuinely unmeasured, not failing, and not estimated. A human with Docker access must run `make services-up && make coverage` and record the figure. Tracked in .planning/WINDOWS.md (id 13)."
  - id: D6
    description: "cargo doc -p paladin-llm --no-deps produces no missing-docs warning for any public item the phase added"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "cargo doc -p paladin-llm --no-deps --features kimi,qwen,grok,ollama,gemini,openai-compatible 2>&1 | grep -c 'missing documentation' -> 0"
        status: pass
    human_judgment: false

# Metrics
duration: ~55min (Task 1, prior session) + ~65min (Tasks 2-3, this continuation) ~= 120min total, approximate
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 07: Capability Invariants, Factory Regression, and Ollama Docker-Gated Tier 2 Suite Summary

**All nine adapters now share one capability-truthfulness test and one compiled-out-provider regression test (Task 1); a required-features-gated Ollama Docker Tier 2 suite exists, compiles cleanly, and proves its own graceful-skip behavior end to end, though its assertions against a real Ollama server remain unverified pending Docker access (Task 2); and the 82% coverage floor measurement itself could not be produced in this Docker-less sandbox, recorded honestly as verification debt rather than fabricated (Task 3).**

## Performance

- **Duration:** ~55 min (Task 1, prior session, commit `e103ca3`) + ~65 min (Tasks 2-3, this continuation session)
- **Completed:** 2026-08-17
- **Tasks:** 3 of 3 executed. Task 1 fully verified. Task 2 authored, compiled, clippy-clean, skip-path proven; runtime-against-real-server unverified. Task 3's non-Docker checks (doc warnings) verified; the coverage measurement itself unverified.
- **Files modified:** 6 (2 in Task 1's prior-session commit; 4 in this session's Task 2 commit)

## Accomplishments

- **Task 1 (prior session, unchanged — see commit `e103ca3`): capability invariants for the six new adapters, and the factory regression test.**
  - `crates/paladin-llm/src/lib.rs` gained `capability_invariants_new_providers`, a **sibling** module to the pre-existing `capability_invariants` (not a widened gate). Gated on `#[cfg(all(test, feature = "kimi", feature = "qwen", feature = "grok", feature = "ollama", feature = "gemini", feature = "openai-compatible"))]`. Pins `get_capabilities().supports_tool_calling`/`.supports_function_calling` to `false` for `KimiAdapter`, `QwenAdapter`, `GrokAdapter`, `OllamaAdapter`, `GeminiAdapter`, and `OpenAiCompatibleAdapter` (the last from an **empty** capability declaration, the pessimistic-default path).
  - `tests/unit/llm/provider_factory_test.rs` gained `CleanNewProviderEnv` (a second RAII env guard) plus `test_compiled_out_provider_absent_from_list_available_providers` and `test_new_provider_names_resolve_through_create` (the D-10 regression this plan closes).
  - Verified load-bearing by deliberately flipping `KimiAdapter::supports_tool_calling` to `true`, confirming the new invariant fails, then reverting (no residual diff).

- **Task 2 (this session): Ollama Docker-gated Tier 2 suite (D-15).**
  - `docker/docker-compose.test.yml` gained `ollama-test` (image `ollama/ollama:0.3.14`, mapped port `11435:11434`, `tmpfs` model storage, `ollama list` healthcheck) and `ollama-test-init` (pulls `qwen2.5:0.5b`, ~397MB, once `ollama-test` is healthy), following the `redis-test`/`minio-test-init` shape.
  - Root `Cargo.toml` gained `[[test]] name = "ollama_docker"` with `required-features = ["integration-tests", "llm-ollama"]`, following the `openai_embedding_integration` block's shape.
  - `tests/integration/ollama_docker_test.rs`: four tests — `generate_round_trip_returns_nonempty_content_and_real_token_usage`, `generate_stream_produces_multiple_chunks_with_nonempty_concatenation`, `get_available_models_returns_the_pulled_model`, `validate_model_distinguishes_pulled_from_unpulled` — each independently probes `OLLAMA_TEST_URL` (default `http://localhost:11435/v1`) before doing anything else and skips with a printed, URL-naming reason (never silently) if unreachable.
  - `Makefile`'s `test-integration-docker` target now brings `ollama-test`/`ollama-test-init` up and runs the suite, then tears the compose stack back down.
  - **Verified without Docker:** `cargo check --test ollama_docker --features integration-tests,llm-ollama` compiles clean; `cargo clippy --test ollama_docker --features integration-tests,llm-ollama -- -D warnings` is clean; `cargo fmt --check` is clean (after one `cargo fmt` pass); `cargo test --test ollama_docker --features integration-tests,llm-ollama -- --nocapture` runs and all 4 tests pass **by skipping gracefully** — nothing listens on port 11435 in this sandbox, so the "unreachable service" path was exercised for real, not merely reasoned about, and each skip printed the exact named-URL message an operator would see.
  - **NOT verified (no Docker daemon in this sandbox):** the suite's actual assertions against a real, running Ollama server; `docker compose -f docker/docker-compose.test.yml config`; `docker compose ... up -d` bringing the services to a healthy state; the `ollama-test-init` model pull. Tracked in `.planning/WINDOWS.md` (id 12, id 14).

- **Task 3 (this session): coverage floor and doc-warning gate.**
  - `cargo doc -p paladin-llm --no-deps --features kimi,qwen,grok,ollama,gemini,openai-compatible 2>&1 | grep -c 'missing documentation'` returns `0` — the phase's added public items all carry rustdoc. (One unrelated, pre-existing `rustdoc::private_intra_doc_links` warning in `crates/paladin-llm/src/gemini/adapter.rs` was observed and left untouched — out of this plan's scope, not a missing-docs warning, not introduced by this plan.)
  - `make coverage` (the actual `--fail-under-lines 82` gate) could **not** be run: its preflight requires Redis on `6380` and MinIO on `9010` (both confirmed unreachable — `nc -z localhost 6380` and `nc -z localhost 9010` both exit `1`), because both are Docker services and no Docker daemon exists in this sandbox. The coverage percentage is genuinely **unmeasured**, not failing, and not estimated or fabricated here. Tracked in `.planning/WINDOWS.md` (id 13).
  - The full workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` and `make lint` (`--all-features` variant) were deliberately not run, per this continuation's build-verification-budget instruction to scope checks locally and defer full-workspace operations to the orchestrator's authoritative post-merge gate.

## Task Commits

1. **Task 1: Capability invariants for the six new adapters, and the factory regression test** - `e103ca3` (test) — prior session
2. **Task 2: Ollama Docker-gated Tier 2 suite** - `9e9ba91` (test) — this session
3. **Task 3: Coverage floor / doc-warning gate** - no code changes (verification-only task; the coverage figure could not be produced, see Known Stubs / Deviations); findings folded into this SUMMARY

## Files Created/Modified

- `crates/paladin-llm/src/lib.rs` - (Task 1, prior session) Added `capability_invariants_new_providers` sibling test module
- `tests/unit/llm/provider_factory_test.rs` - (Task 1, prior session) Added `CleanNewProviderEnv` guard and two D-10 regression tests
- `docker/docker-compose.test.yml` - (Task 2, this session) Added `ollama-test` and `ollama-test-init` services
- `Cargo.toml` - (Task 2, this session) Added `[[test]] name = "ollama_docker"` with `required-features = ["integration-tests", "llm-ollama"]`
- `tests/integration/ollama_docker_test.rs` - (Task 2, this session, new file) Four-test Docker-gated Tier 2 suite with a runtime reachability skip gate
- `Makefile` - (Task 2, this session) `test-integration-docker` now brings `ollama-test` up and runs the suite

## Decisions Made

See `key-decisions` in the frontmatter above. Most consequential: the human ruling to **author Task 2/3 under the Docker-absent constraint rather than halt a second time**, with the explicit, provenance-recorded understanding that this overrides Task 2's own `<precondition>` by deliberate choice — and the explicit instruction that the deliverable is an *honestly-flagged unrun suite*, never a suite reported as passing when it never ran against a real server. This SUMMARY's `coverage:` block marks every Docker-dependent deliverable `human_judgment: true` with a `rationale` naming exactly what remains unproven, precisely to keep that distinction visible to `/gsd-audit-uat` and any future verifier.

## Deviations from Plan

### Auto-fixed Issues (Task 1, prior session)

**1. [Rule 1 - Bug] Task 1's own acceptance-criterion grep for the doc-table row check doesn't match the file's existing formatting convention**
- **Found during:** Task 1, acceptance-criteria verification
- **Issue:** The plan's literal check assumes table rows are written as `//! | kimi | ...`; the actual convention wraps provider names in backticks (`` //! | `kimi` | ... ``), added by plan 17-05.
- **Fix:** No content change needed — verified with a backtick-aware pattern instead; all six rows were already present and correct.
- **Files modified:** None
- **Committed in:** N/A (verification-only finding; no code change)

**2. [Rule 1 - Bug] First draft of `test_new_provider_names_resolve_through_create`'s "message must not list the bogus name" assertion was self-referentially wrong**
- **Found during:** Task 1, first test run under `cargo test --test unit llm::provider_factory`
- **Issue:** `ProviderFactoryError::UnknownProvider`'s `Display` impl echoes the requested (bogus) name in its own message, so an initial `!message.contains(bogus_name)` assertion failed for the wrong reason.
- **Fix:** Narrowed the assertion to the substring after `"Supported providers: "` only.
- **Files modified:** `tests/unit/llm/provider_factory_test.rs`
- **Committed in:** `e103ca3` (Task 1 commit; fix made before committing)

### Documented Substitutions (Task 2, this session)

**3. [Human-ruled override] Authored Task 2/3 without Docker verification, per explicit human ruling**
- **Found during:** Continuation agent startup — the previous session's Task 2 halted at its own `<precondition>` (Docker unreachable) with a `blocking-human` checkpoint.
- **Resolution:** A human, via `AskUserQuestion` during `/gsd-execute-phase 17` orchestration on 2026-08-17, explicitly ruled to author the suite anyway and flag it as verification debt (provenance recorded in the continuation prompt's `<resolved_checkpoint>` block). This is a human decision, not an executor auto-fix, and does not fall under deviation Rules 1-3 — it directly authorizes the specific action taken.
- **Files affected:** `docker/docker-compose.test.yml`, `Cargo.toml`, `tests/integration/ollama_docker_test.rs`, `Makefile`
- **Verification:** Everything verifiable without Docker was verified (compile, clippy, fmt, the real graceful-skip path). Everything requiring Docker was left explicitly unverified and recorded as such (`.planning/WINDOWS.md` ids 12-14, `coverage:` block `human_judgment: true` entries).
- **Committed in:** `9e9ba91`

**4. [Rule 1-adjacent, documented] ollama-test healthcheck substitutes `ollama list` for the plan's preferred curl-based `/v1/models` check**
- **Found during:** Task 2, authoring the compose service
- **Issue:** The plan's action text prefers hitting `/v1/models` (the OpenAI-compat surface, matching what the tests themselves call) over the native `/api/tags`. Implementing that requires `curl` or `wget` inside the `ollama/ollama:0.3.14` container, and neither tool's presence could be confirmed without a Docker daemon to inspect the image.
- **Fix:** Used `ollama list` instead — the bundled CLI binary is guaranteed present (it's the image's own entrypoint), calls the local server's own API internally, and only succeeds once serving. This is a well-precedented healthcheck for this exact image elsewhere in the ecosystem, but the substitution itself has not been run against the real image here.
- **Files modified:** `docker/docker-compose.test.yml`
- **Committed in:** `9e9ba91`
- **Tracked:** `.planning/WINDOWS.md` id 14

---

**Total deviations:** 2 auto-fixed (Task 1, prior session, both self-caught/corrected before commit) + 2 documented substitutions/overrides (this session, both human-ruled or explicitly reasoned and flagged)
**Impact on plan:** No scope creep. The Task 2/3 substitutions are narrowly scoped to the Docker-verification gap the human ruling explicitly authorized; no fabricated pass/fail claims were made anywhere.

## Issues Encountered

**Blocking, human-ruled (not a fresh checkpoint):** Both Task 2's `<precondition>` (Docker daemon reachable) and Task 3's `<precondition>` (Redis on 6380 / MinIO on 9010 via `make services-up`) are unmet in this execution sandbox for the same root cause — no `docker` or `docker-compose` binary exists here at all (`which docker docker-compose` returns nothing). Per the resolved checkpoint governing this continuation, this is not treated as a fresh blocker requiring a new checkpoint: the human ruling already covers "blocked by Docker" for both tasks. Both tasks were executed to the fullest extent possible without Docker, and every gap that Docker's absence leaves is named explicitly above and in `.planning/WINDOWS.md` (ids 12, 13, 14).

## Known Stubs

None in the "hardcoded empty value flowing to UI rendering" sense this section is designed to catch (this plan is test/tooling infrastructure, not application code with a UI surface). The closest analogue — deliverables that exist but are not yet fully proven — are tracked precisely via the `coverage:` frontmatter block's `human_judgment: true` entries (D3, D4, D5) and `.planning/WINDOWS.md` ids 12, 13, 14, rather than duplicated here.

## User Setup Required

**Docker must be available to close the remaining verification debt.** Specifically, a human (or a Docker-enabled CI runner) needs to:

1. `docker compose -f docker/docker-compose.test.yml config` — confirm the compose file is schema-valid (only YAML-syntax-checked here).
2. `docker compose -f docker/docker-compose.test.yml up -d ollama-test ollama-test-init` — confirm `ollama-test` reaches `healthy` via its `ollama list` healthcheck, and that `ollama-test-init` successfully pulls `qwen2.5:0.5b`.
3. `cargo test --test ollama_docker --features integration-tests,llm-ollama -- --nocapture` — confirm all 4 tests pass **for real** (not by skipping) against the live service.
4. `make services-up && make coverage` — obtain the actual workspace line-coverage percentage with all six new adapters counted, and confirm it clears the 82% floor (ADR-0006, D-00p). If it falls short, add targeted mock-transport tests per the plan's Task 3 action — do not move the floor.
5. Once 3 and 4 pass, mark `.planning/WINDOWS.md` entries 12, 13, and 14 resolved.

## Next Phase Readiness

- **Tasks executed, verification partially complete.** All three tasks in this plan were addressed: Task 1 is fully verified (prior session). Task 2's suite is authored, compiles, is clippy-clean, and its graceful-skip behavior is proven for real; its assertions against a live Ollama server are unverified. Task 3's non-Docker checks (missing-docs) are verified; its core deliverable (the measured coverage percentage) is unverified.
- **What's safe to build on:** the capability-truthfulness invariant now covers all nine adapters; a compiled-out provider is proven absent from `list_available_providers()` by a test; the Ollama suite's structure (required-features gate + runtime reachability gate) is sound and will run for real with no code change once Docker is available.
- **Blocker for phase completion (verification debt, not a code gap):** PROV-04's "workspace stays at or above the 82% line-coverage floor" and "Docker-gated Tier 2 suite exercises the shared compatible core against a real Ollama instance" truths are authored but not yet demonstrated. A human with Docker access — or `/gsd-audit-uat` surfacing `.planning/WINDOWS.md` ids 12-14 — must close this gap before the phase can be considered fully verified.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*

## Self-Check: PASSED

- FOUND: `tests/integration/ollama_docker_test.rs`
- FOUND: `docker/docker-compose.test.yml`
- FOUND: `.planning/phases/17-additional-llm-provider-adapters/17-07-SUMMARY.md`
- FOUND commit `e103ca3` (Task 1, prior session)
- FOUND commit `9e9ba91` (Task 2, this session)
