---
phase: 17-additional-llm-provider-adapters
plan: 07
subsystem: testing
tags: [rust, cargo-features, llm, capability-invariants, provider-factory, docker, ollama]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 06)
    provides: "the completed nine-provider facade feature-flag wiring (D-11 amended option-b) and the nine-provider LlmConfig/config-bridge surface that this plan tests against"
provides:
  - "capability_invariants_new_providers sibling test module in crates/paladin-llm/src/lib.rs, pinning supports_tool_calling/supports_function_calling to false for all six new adapters (kimi, qwen, grok, ollama, gemini, openai-compatible-with-empty-declaration)"
  - "Two new D-10 regression tests in tests/unit/llm/provider_factory_test.rs: test_compiled_out_provider_absent_from_list_available_providers and test_new_provider_names_resolve_through_create, plus the CleanNewProviderEnv RAII guard they share"
affects: [17-07-continuation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Sibling cfg-gated test module (not a widened gate) when a cross-adapter invariant must cover a second, disjoint feature combination without disabling the test that already runs for the first — same shape as the pre-existing capability_invariants module, applied a second time"
    - "A second RAII env-guard (CleanNewProviderEnv, its own Mutex) for a disjoint set of process-wide env vars, following the file's existing CleanProviderEnv precedent exactly, so tests touching the two variable sets can still run concurrently within the same binary without cross-contaminating each other's saved/restored state"

key-files:
  created:
    - .planning/phases/17-additional-llm-provider-adapters/17-07-SUMMARY.md
  modified:
    - crates/paladin-llm/src/lib.rs
    - tests/unit/llm/provider_factory_test.rs

key-decisions:
  - "Task 2's <precondition> (\"A Docker daemon is reachable — docker info exits 0. If it is not, halt and surface the checkpoint rather than authoring the suite blind\") is unmet in this sandbox — the docker CLI itself is not installed (docker: command not found), not merely a daemon-unreachable case. Per the executor's precondition protocol, an unmet precondition is never auto-approved (even under auto-mode) and must halt with a checkpoint rather than be worked around. Task 1 was completed and committed first since it carries no such precondition; Task 2 and Task 3 (which depends on Task 2's docker-compose changes and, per its own precondition, may depend on the ollama-test service) were not started."
  - "test_new_provider_names_resolve_through_create and test_compiled_out_provider_absent_from_list_available_providers are written to be feature-set-adaptive at runtime (querying provider_names() rather than assuming a fixed compiled set), per the task's own instruction not to assume which provider the root test target compiles out. Verified under both the root crate's actual default feature set (llm-openai/llm-anthropic/llm-deepseek, where all six new providers are absent) and under an additive nine-feature combination (where kimi/qwen/grok/ollama/gemini/openai-compatible are all present) to confirm both branches of the runtime logic execute correctly."
  - "The UnknownProvider Display message echoes the *requested* name back (\"Unknown provider: {name}. Supported providers: ...\"), so the 'must not list the bogus name' assertion in test_new_provider_names_resolve_through_create checks only the substring after \"Supported providers: \", not the whole message — an initial version of this assertion checked the whole message and failed for the wrong reason (self-referential false positive), caught and fixed before commit."

requirements-completed: []

# Metrics
duration: ~55min (Task 1 only; plan incomplete)
completed: 2026-08-17
status: blocked
---

# Phase 17 Plan 07: Capability Invariants and Factory Regression for Six New Adapters (Partial — Task 1 of 3) Summary

**Task 1 complete and committed: capability_invariants_new_providers sibling module plus two D-10 factory regression tests, all green under both the default and combined-feature builds. Task 2 (Ollama Docker-gated Tier 2 suite) and Task 3 (coverage gate) are blocked — this sandbox has no `docker` binary at all, and Task 2's own `<precondition>` mandates a checkpoint rather than authoring the suite blind.**

## Performance

- **Duration:** ~55 min for Task 1 (research, implementation, debugging two test-authoring mistakes, verification)
- **Completed:** 2026-08-17 (Task 1 commit `e103ca3`)
- **Tasks:** 1 of 3 complete (Task 1: `type="auto" tdd="true"`); Task 2 and Task 3 not started
- **Files modified:** 2

## Accomplishments

- **Task 1: capability invariants for the six new adapters, and the factory regression test.**
  - `crates/paladin-llm/src/lib.rs` gained `capability_invariants_new_providers`, a **sibling** module to the pre-existing `capability_invariants` (not a widened gate — widening would have made the shipped three's invariant stop compiling unless every new feature were also enabled, silently disabling a test that runs today under `cargo test --workspace`). Gated on `#[cfg(all(test, feature = "kimi", feature = "qwen", feature = "grok", feature = "ollama", feature = "gemini", feature = "openai-compatible"))]`. Its own two source-of-truth constants (`REQUEST_SURFACE_SUPPORTS_TOOL_CALLING`, `RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING`, both `false`) pin `get_capabilities().supports_tool_calling`/`.supports_function_calling` for `KimiAdapter`, `QwenAdapter`, `GrokAdapter`, `OllamaAdapter`, `GeminiAdapter`, and `OpenAiCompatibleAdapter` — the last constructed from an **empty** `OpenAiCompatibleCapabilitiesConfig` (every field its own pessimistic default per D-04), covering the exact configuration an operator gets when they declare nothing.
  - `tests/unit/llm/provider_factory_test.rs` gained `CleanNewProviderEnv` (a second RAII env guard, its own `NEW_PROVIDER_ENV_LOCK`, mirroring the file's existing `CleanProviderEnv`/`PROVIDER_ENV_LOCK` pattern exactly) plus two tests:
    - `test_compiled_out_provider_absent_from_list_available_providers` — finds, at runtime via `provider_names()`, the first of `{kimi, qwen, grok, gemini}` genuinely not compiled into this test binary's current feature set, sets its credential env var anyway, and asserts it is still absent from `list_available_providers()` (the D-10 regression this plan closes). Falls back to the plan's own specified structural assertion (every reported-available name is in the compiled-in registry) if every candidate happens to be compiled in.
    - `test_new_provider_names_resolve_through_create` — for each of the six new providers actually compiled into this binary, sets its credential(s) and asserts `create()` does not return `UnknownProvider`; separately asserts a definitely-unregistered name always returns `UnknownProvider`, and that the message's "Supported providers:" segment lists every compiled-in name and never the bogus one.
  - Verified the invariant is load-bearing, not decorative: temporarily flipped `KimiAdapter`'s `supports_tool_calling` to `true` in `crates/paladin-llm/src/kimi/adapter.rs`, re-ran `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,gemini,openai-compatible capability_invariants_new_providers`, confirmed it failed with the expected assertion message, then reverted (`git diff --stat` on that file is empty — no residual change).

## Task Commits

1. **Task 1: Capability invariants for the six new adapters, and the factory regression test** - `e103ca3` (test)
2. Task 2 and Task 3 — not started (blocked; see Deviations)

## Files Created/Modified

- `crates/paladin-llm/src/lib.rs` - Added `capability_invariants_new_providers` sibling test module (163 lines added, 0 removed from the pre-existing `capability_invariants` module — confirmed via `git diff -U0 | grep '^-[^-]'` returning empty)
- `tests/unit/llm/provider_factory_test.rs` - Added `CleanNewProviderEnv` guard and two D-10 regression tests (282 lines added)

## Decisions Made

See `key-decisions` in the frontmatter above. Most consequential: **halting at Task 2's unmet `<precondition>`** rather than writing the Docker-gated suite blind. This sandbox has no `docker` binary at all (`docker: command not found`), which is a stronger unmet condition than "daemon unreachable" — there is no way to bring up `ollama-test`, prove the compose file valid, or run `cargo test --test ollama_docker` here. Per the executor's precondition protocol, this is never auto-approved even in an autonomous run; it requires a human to either provide Docker access in this environment or explicitly direct the continuation differently.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Task 1's own acceptance-criterion grep for the doc-table row check doesn't match the file's existing formatting convention**
- **Found during:** Task 1, acceptance-criteria verification
- **Issue:** The plan's literal check — `grep -cE '^//! \| (kimi|qwen|grok|ollama|gemini|openai-compatible) ' crates/paladin-llm/src/lib.rs` returning `6` — assumes table rows are written as `//! | kimi | ...`. The actual convention already in force in `lib.rs` (established for `openai`/`anthropic`/`deepseek` before this phase, and already applied to all six new providers by plan 17-05) wraps every provider name in backticks: `` //! | `kimi` | ... ``. The literal command returns `0` against the real file.
- **Fix:** No content change needed — verified with a backtick-aware pattern instead: `grep -cE '^//! \| \`(kimi|qwen|grok|ollama|gemini)\`|^//! \| \`openai-compatible\`' crates/paladin-llm/src/lib.rs` returns `6`. All six rows were already present (added by plan 17-05, not this plan) with correct content; this is a verification-methodology mismatch, not a missing-content defect. Documented here per D-00e rather than silently substituted.
- **Files modified:** None
- **Verification:** `grep -cE '^//! \| `(kimi|qwen|grok|ollama|gemini)`|^//! \| `openai-compatible`' crates/paladin-llm/src/lib.rs` → `6`
- **Committed in:** N/A (verification-only finding; no code change)

**2. [Rule 1 - Bug] First draft of `test_new_provider_names_resolve_through_create`'s "message must not list the bogus name" assertion was self-referentially wrong**
- **Found during:** Task 1, first test run under `cargo test --test unit llm::provider_factory`
- **Issue:** `ProviderFactoryError::UnknownProvider`'s `Display` impl is `"Unknown provider: {0}. Supported providers: {supported}"` — the *requested* (bogus) name is necessarily echoed back in the message itself. An initial `!message.contains(bogus_name)` assertion failed because the bogus name legitimately appears in the "Unknown provider: {bogus_name}" prefix, which is not the property under test.
- **Fix:** Narrowed the assertion to the substring after `"Supported providers: "` only, so it correctly checks that the *list of accepted names* — not the whole message — excludes the bogus one.
- **Files modified:** `tests/unit/llm/provider_factory_test.rs`
- **Verification:** `cargo test --test unit llm::provider_factory` — 11/11 passed after the fix
- **Committed in:** `e103ca3` (Task 1 commit; the fix was made before committing, so the commit contains only the corrected version)

---

**Total deviations:** 2 (1 verification-methodology note with no code change, 1 self-caught-and-fixed test-authoring bug, corrected before commit)
**Impact on plan:** No scope creep. Neither affected Task 1's actual deliverables.

## Issues Encountered

**Blocking (not a deviation — a precondition halt):** Task 2's `<precondition>` — "A Docker daemon is reachable — `docker info` exits 0" — is unmet in this execution environment. `which docker` and `docker version` both report `docker: command not found`; the CLI is not installed, not merely a daemon-connectivity issue. Per the executor's precondition protocol this is never auto-approved (even in an autonomous/auto-mode run) and the correct response is to halt with a checkpoint rather than write `docker/docker-compose.test.yml` and `tests/integration/ollama_docker_test.rs` without any way to prove they work. Task 1's own work has no such dependency and was completed and committed cleanly.

## User Setup Required

**Docker must be available for Task 2 and Task 3 to proceed.** Specifically:
- A `docker` CLI and daemon reachable via `docker info` (Task 2's own precondition).
- `docker compose` support (used by the plan's verification commands).
- Redis reachable on `localhost:6380` and MinIO on `localhost:9010` via `make services-up` (Task 3's precondition), plus the `ollama-test` service once Task 2 stands it up, if Task 2 makes it a `make coverage` prerequisite.

Once Docker is available, resume from Task 2 using this SUMMARY's Task Commits table — Task 1 (`e103ca3`) is complete and does not need to be redone.

## Next Phase Readiness

- **Not ready.** This plan (17-07) is 1 of 3 tasks complete. Task 2 (Ollama Docker-gated Tier 2 suite: `docker/docker-compose.test.yml`, `tests/integration/ollama_docker_test.rs`, the root `Cargo.toml` `[[test]]` block, and the `Makefile` `test-integration-docker` extension) and Task 3 (clearing the 82% coverage floor with the new code counted, plus the `cargo doc` missing-docs check) both remain, and both require capabilities (Docker; `make services-up`) not available in this sandbox.
- What Task 1 delivered stands independently and is safe to build on: the capability-truthfulness invariant now covers all nine adapters (not six in prose, three in test), and a compiled-out provider is proven absent from `list_available_providers()` by a test, not just by the production code's own `#[cfg]` gates.
- **Blocker for phase completion:** PROV-04's "Docker-gated Tier 2 suite exercises the shared compatible core against a real Ollama instance" and "workspace stays at or above the 82% line-coverage floor" truths are not yet demonstrated. A continuation agent (or the same agent re-run in an environment with Docker) must complete Tasks 2 and 3 before this plan — and per its dependency chain, the phase — can be considered done.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17 (partial — Task 1 of 3)*
