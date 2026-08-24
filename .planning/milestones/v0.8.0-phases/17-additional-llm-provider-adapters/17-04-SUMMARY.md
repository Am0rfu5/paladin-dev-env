---
phase: 17-additional-llm-provider-adapters
plan: 04
subsystem: api
tags: [llm, rust, openai-compatible, generic-provider, mockito, reqwest, tokio, ssrf]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 01)
    provides: "crates/paladin-llm/src/compat/{mod,types,engine}.rs (CompatEngine), crates/paladin-llm/src/redaction.rs, the ProviderRegistration table-driven registry in provider_factory.rs"
  - phase: 17-additional-llm-provider-adapters (plan 03)
    provides: "crates/paladin-llm/src/ollama/adapter.rs as the closest analog (configurable base_url, keyless credential handling), and the credential-free registry-row placement pattern"
provides:
  - "crates/paladin-llm/src/openai_compatible/{mod,adapter}.rs — OpenAiCompatibleAdapter, the generic operator-configured OpenAI-compatible provider (D-03)"
  - "provider_factory.rs registry row for openai-compatible, placed after every curated preset but before Ollama's credential-free row"
  - "provider_factory.rs::tests::provider_name_round_trip — registry-wide provider-name round-trip invariant test, the companion test to this plan's assumption-delta decision"
  - "CompatEngineConfig.redirect_policy — new optional field on the shared engine (T-17-18 mitigation), None for every existing preset (unchanged behaviour), Some(Policy::none()) for the generic provider"
affects: [17-06, 17-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Generic operator-configured preset over CompatEngine — every CompatEngineConfig field is operator-supplied with no vendor default, contrasted with the five named presets (17-01/17-03) which each supply defaults from vendor knowledge"
    - "Pessimistic-default capabilities struct with dual sourcing — one Deserialize impl (config-file path) and one from_env()/from_parts() pure-logic path (env-var path) constructing the identical struct, both defaulting every non-streaming field to the conservative answer"
    - "Registry-row placement between the curated-preset block and the credential-free fallback row — required because get_default_provider()'s scan treats env_var: None as an unconditional match, so any credentialed row placed after it becomes unreachable through that path"

key-files:
  created:
    - crates/paladin-llm/src/openai_compatible/mod.rs
    - crates/paladin-llm/src/openai_compatible/adapter.rs
  modified:
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-llm/src/provider_factory.rs
    - crates/paladin-llm/Cargo.toml
    - crates/paladin-llm/src/compat/engine.rs
    - crates/paladin-llm/src/kimi/adapter.rs
    - crates/paladin-llm/src/qwen/adapter.rs
    - crates/paladin-llm/src/grok/adapter.rs
    - crates/paladin-llm/src/ollama/adapter.rs

key-decisions:
  - "Task 1 checkpoint resolved option-a (provider literal \"openai-compatible\", env prefix OPENAI_COMPATIBLE_, capabilities declarable via BOTH individual env vars AND a config-file block). Provenance: selected by the human user in an interactive AskUserQuestion prompt raised by the /gsd-execute-phase 17 orchestrator at the Task 1 blocking checkpoint on 2026-08-17. The orchestrator presented the plan's three options verbatim; the user chose option-a. Recorded verbatim in this SUMMARY per the resolved-checkpoint instruction, not attributed to any agent."
  - "Widened CompatEngineConfig with an optional redirect_policy field (Rule 2 security deviation) to satisfy the plan's own T-17-18 threat-model mitigation, which requires the generic provider's engine to disable HTTP redirects so a 302 from an operator-configured base_url can never carry the Authorization header to a different host. None preserves every existing preset's original behaviour; only OpenAiCompatibleAdapter sets Some(Policy::none())."
  - "Registered the openai-compatible row between the curated-preset block and Ollama's credential-free row in provider_factory.rs's table, not after Ollama — Ollama's env_var: None row unconditionally matches get_default_provider()'s scan, so a credentialed row placed after it would never be selected even with its own credential present."
  - "Widened provider_factory.rs's four_new_preset_build gate (Rule 1 auto-fix, mirroring plan 17-03's own precedent for this exact gate) to exclude openai-compatible, and added five_new_preset_build proving the new five-row declared order under the plan's combined verification command."
  - "The round-trip invariant test's credential-presence check treats an env var set to an empty string as absent rather than present (Rule 1 auto-fix, discovered while running the plan's own combined verification command — this execution environment has XAI_API_KEY set to an empty string, which made construct_grok() fail validation despite std::env::var(\"XAI_API_KEY\").is_ok() being true)."

patterns-established:
  - "Optional per-preset transport-policy override on CompatEngineConfig (redirect_policy) — the first field on this struct that is Some for one preset and None for every other, establishing the pattern for any future per-preset transport hardening without forking the engine."

requirements-completed: [PROV-02, PROV-03, PROV-04]

coverage:
  - id: D1
    description: "OpenAiCompatibleAdapter implements all six LlmPort methods by delegating to CompatEngine; base_url, api_key, model and capabilities are entirely operator-supplied with no vendor default; get_provider_name() returns the fixed literal \"openai-compatible\" and never leaks the configured base_url"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#tests (34 tests: pessimistic-default deserialization x2, capability-value/unparseable-error handling x5, required-field errors x4, request shaping, streaming assembly, 4 HTTP error mappings, provider-name round-trip x2, plaintext/loopback detection x4, capabilities-are-config-only x2)"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm --no-default-features --features openai-compatible && cargo test -p paladin-llm --no-default-features --features openai-compatible (76 tests) && cargo clippy -p paladin-llm --no-default-features --features openai-compatible -- -D warnings"
        status: pass
    human_judgment: false
  - id: D2
    description: "Every undeclared capability takes the conservative value (D-04): supports_streaming is the sole default-true field; supports_tool_calling/supports_function_calling/supports_vision/supports_embeddings/supports_system_messages default false; max_context_tokens and temperature_range default None. get_capabilities() never consults the engine, the endpoint, or a live /models response — verified with a fabricated capability field in a mocked /models response"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs#tests::deserializing_empty_json_object_yields_pessimistic_defaults, ::from_env_with_no_capability_variables_set_yields_the_identical_pessimistic_defaults, ::a_models_response_with_a_fabricated_capability_field_does_not_change_capabilities"
        status: pass
    human_judgment: false
  - id: D3
    description: "T-17-18 mitigated: the generic provider's engine is constructed with reqwest's redirect policy set to none, so a 3xx response from the configured base_url can never replay the Authorization header to a different host. Every existing preset (kimi/qwen/grok/ollama) is unaffected — each explicitly sets redirect_policy: None, preserving original behaviour"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm (default features, 53 tests) and cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible (137 tests) both pass, confirming no behavioural regression in any preset"
        status: pass
      - kind: other
        ref: "code inspection: crates/paladin-llm/src/openai_compatible/adapter.rs sets redirect_policy: Some(reqwest::redirect::Policy::none()); crates/paladin-llm/src/{kimi,qwen,grok,ollama}/adapter.rs each set redirect_policy: None"
        status: pass
    human_judgment: false
  - id: D4
    description: "openai-compatible resolves through provider_factory.rs's cfg-gated ProviderRegistration table (D-10), registered between the curated presets and Ollama's credential-free row; the registry-wide provider-name round-trip invariant test proves every compiled-in row's constructed adapter reports its own registered name"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --no-default-features --features ollama,openai-compatible provider_name_round_trips (1 test passed) — provider_factory.rs#tests::provider_name_round_trip::provider_name_round_trips_for_every_registry_row"
        status: pass
      - kind: unit
        ref: "provider_factory.rs#tests::five_new_preset_build::provider_names_returns_exactly_kimi_qwen_grok_openai_compatible_ollama_in_table_order, ::provider_names_are_lowercase_and_whitespace_free, ::provider_names_has_no_duplicate_entries"
        status: pass
    human_judgment: false
  - id: D5
    description: "Default paladin-llm feature set (openai, mock) is unchanged; openai/, anthropic/, deepseek/ are byte-unchanged (D-06); compat/mod.rs and compat/types.rs are byte-unchanged"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm (default features, 53 tests passed) and cargo fmt -p paladin-llm --check (clean)"
        status: pass
      - kind: other
        ref: "grep -c 'default = [\"openai\", \"mock\"]' crates/paladin-llm/Cargo.toml (returns 1); git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ (empty diff)"
        status: pass
    human_judgment: false
  - id: D6
    description: "Task 1's blocking naming checkpoint resolved to option-a and implemented exactly as selected — provider literal openai-compatible, env prefix OPENAI_COMPATIBLE_, capabilities declarable via both individual env vars and a config-file Deserialize block"
    verification:
      - kind: other
        ref: "crates/paladin-llm/src/openai_compatible/adapter.rs module rustdoc \"Naming — resolved at the Task 1 checkpoint\" section records the selection and provenance verbatim"
        status: pass
    human_judgment: false

# Metrics
duration: ~1min (git commit timestamps: 12:06:48Z Task 2 to 12:07:56Z Task 3; excludes read/planning time and the prior checkpoint-blocked attempt before this continuation agent started)
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 04: Generic OpenAI-Compatible Provider (D-03) Summary

**`OpenAiCompatibleAdapter` — the operator-configured OpenAI-compatible provider (base_url + key + model + declared capabilities, zero new code per endpoint) that disposes of Groq/Together/Mistral/Fireworks/Bedrock as already-covered rather than deferred, plus a redirect-policy hardening on the shared `CompatEngine` for its untrusted `base_url` trust boundary.**

## Performance

- **Duration:** ~1 min (Task 2 → Task 3 commit-to-commit; see frontmatter note — excludes context-reading time and the prior checkpoint-blocked attempt)
- **Completed:** 2026-08-17T12:07:56Z (Task 3 commit)
- **Tasks:** 3 (Task 1 checkpoint resolved by continuation prompt, no code; Task 2 and Task 3 executed and committed)
- **Files modified:** 10 (2 new under `openai_compatible/`, 8 modified: `lib.rs`, `provider_factory.rs`, `Cargo.toml`, `compat/engine.rs`, and the four existing CompatEngine presets)

## Accomplishments

- Built `OpenAiCompatibleAdapter`/`OpenAiCompatibleConfig`/`OpenAiCompatibleCapabilitiesConfig` (`crates/paladin-llm/src/openai_compatible/`) — the generic operator-configured provider. `base_url`, `api_key` and `model` are all **required** with no defensible default (unlike every named preset); `OPENAI_COMPATIBLE_TIMEOUT_SECONDS` defaults to `60`.
- D-04 pessimistic capability defaults, both sourcing paths tested: deserializing an empty JSON object and calling `from_env()` with zero capability variables set both yield `supports_streaming: true` (the sole default-`true` field) and everything else `false`/`None`. Setting a capability to an unparseable value is a configuration error, never a silent coercion.
- `get_capabilities()` reads only from operator configuration — proven with a mocked `/models` response carrying a fabricated `supports_tool_calling: true` field that changes nothing.
- `get_provider_name()` returns the fixed literal `"openai-compatible"` (D-07/D-09) for every configured endpoint, verified never to contain the configured host string, with the accepted T-17-20 cost (two generic instances are indistinguishable in diagnostics) documented on the method itself.
- **T-17-18 (SSRF-adjacent credential exfiltration via redirect):** widened `CompatEngineConfig` with an optional `redirect_policy: Option<reqwest::redirect::Policy>` field. `None` preserves every existing preset's exact original behaviour (Kimi, Qwen, Grok, Ollama each explicitly set `None`); `OpenAiCompatibleAdapter` is the only adapter setting `Some(Policy::none())`, because it is the only one whose `base_url` is untrusted operator input rather than a fixed vendor host.
- **T-17-22:** logs a `warn!` once at construction when `base_url` is plain HTTP to a non-loopback host (the configured API key would be sent in clear text); loopback and HTTPS hosts are not flagged.
- Registered the `openai-compatible` row in `provider_factory.rs`'s table-driven registry (D-10), placed after every curated preset but **before** Ollama's credential-free row — required because `get_default_provider()`'s scan treats `env_var: None` as an unconditional match, so a credentialed row placed after Ollama would never be reachable through that path even with its own credential set.
- Widened the `four_new_preset_build` D-10 regression gate (Rule 1, mirroring plan 17-03's own precedent) to exclude `openai-compatible`, and added `five_new_preset_build` proving the new declared order `["kimi", "qwen", "grok", "openai-compatible", "ollama"]` under the plan's combined verification command.
- Added `provider_name_round_trips_for_every_registry_row` (Task 3) — the companion test to this plan's `<assumption_delta_decision>`: for every registry row whose credential is present and non-empty, constructing it and calling `get_provider_name()` returns that row's own registered name. Gated on `feature = "ollama"` so the "at least one row exercised" assertion is always safe regardless of which secrets are present. Plus the two hygiene assertions: no duplicate names, every name lowercase and whitespace-free.
- `compat/mod.rs` and `compat/types.rs` are **byte-unchanged**. `openai/`, `anthropic/`, `deepseek/` are byte-unchanged (D-06, confirmed via `git diff --stat`). Default `paladin-llm` feature set (`openai`, `mock`) is unchanged (PROV-03).

## Task Commits

1. **Task 1: Confirm the generic provider's public surface shape before it ships** — no commit (checkpoint resolved via continuation prompt; the selected naming is implemented in Task 2's commit and recorded verbatim in the module rustdoc's "Naming — resolved at the Task 1 checkpoint" section)
2. **Task 2: OpenAiCompatibleAdapter with pessimistic operator-declared capabilities** - `e193b8d` (feat)
3. **Task 3: Registry-wide provider-name round-trip invariant** - `9c67474` (test)

_Note: this plan's Task 2/3 are `type="auto" tdd="true"`, not TDD-gated at the plan level; each landed as a single commit with tests written alongside the implementation, following plans 17-01/17-03's precedent._

## Files Created/Modified

- `crates/paladin-llm/src/openai_compatible/mod.rs` - Module re-export shape mirroring `ollama/mod.rs`
- `crates/paladin-llm/src/openai_compatible/adapter.rs` - `OpenAiCompatibleConfig`/`OpenAiCompatibleCapabilitiesConfig`/`OpenAiCompatibleAdapter`, module-level rustdoc recording Task 1's checkpoint resolution and the `OPENAI_COMPATIBLE_API_KEY` vs `OPENAI_API_KEY` distinction, 34 mockito-backed and pure-logic tests
- `crates/paladin-llm/src/lib.rs` - Declares `openai_compatible` module (feature-gated), widens the `compat` module's `cfg(any(...))` gate, adds the doc-table row
- `crates/paladin-llm/src/provider_factory.rs` - `construct_openai_compatible` + registry row (Task 2); widened `four_new_preset_build` gate and added `five_new_preset_build`, `provider_names_are_lowercase_and_whitespace_free`, and the `provider_name_round_trip` module (Task 3)
- `crates/paladin-llm/Cargo.toml` - Adds `openai-compatible = ["dep:reqwest", "dep:rand"]`; `default = ["openai", "mock"]` unchanged (PROV-03)
- `crates/paladin-llm/src/compat/engine.rs` - Adds `CompatEngineConfig.redirect_policy: Option<reqwest::redirect::Policy>` and applies it in `CompatEngine::new` (T-17-18); its own `test_config()` sets `None`
- `crates/paladin-llm/src/{kimi,qwen,grok,ollama}/adapter.rs` - Each adds `redirect_policy: None` to its `CompatEngineConfig` literal — mechanical, no behavioural change

## Decisions Made

- **Task 1 checkpoint resolution: option-a.** Provider name literal `"openai-compatible"` (D-09, already fixed); env-var prefix `OPENAI_COMPATIBLE_`; capability declaration via **both** individual environment variables and a structured config-file block, with the deciding rationale that the provider must be fully usable from environment variables alone, matching every other adapter in this crate and PROJECT.md's env-var-only credential posture. **Provenance:** selected by the human user in an interactive `AskUserQuestion` prompt raised by the `/gsd-execute-phase 17` orchestrator at the Task 1 blocking checkpoint on 2026-08-17. The orchestrator presented the plan's three options verbatim; the user chose `option-a`. This selection is not an agent decision — it is recorded here per the resolved-checkpoint's own instruction, and verbatim in the module rustdoc.
- **Accepted trade-off from the resolved checkpoint:** the env surface is wide (nine capability variables), and `OPENAI_COMPATIBLE_API_KEY` reads visually close to `OPENAI_API_KEY`. Addressed in the module rustdoc with a dedicated "`OPENAI_COMPATIBLE_API_KEY` is not `OPENAI_API_KEY`" section, explicit about the distinction so an operator cannot silently set the wrong one.
- **`fallback_models: Vec::new()`** for the generic provider (not part of the plan's explicit spec, but a necessary choice) — unlike the five named presets, there is no vendor-curated model list for an arbitrary operator-configured endpoint; an empty live `/models` response or a failed fetch resolves to an empty list, which is the honest answer for an endpoint this adapter has no advance knowledge of.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] `CompatEngineConfig` had no way to restrict HTTP redirects, but the plan's own T-17-18 threat-model mitigation requires it**
- **Found during:** Task 2, before writing the adapter — the plan's threat model (`T-17-18`, disposition `mitigate`) explicitly specifies "Task 2 builds the engine with `reqwest`'s redirect policy set to none for this adapter," but `CompatEngine::new()` built its `reqwest::Client` with no way to override the redirect policy, and the upstream context's own preservation instruction ("delegate without modifying `compat/`... zero changes across four presets") would otherwise have blocked this.
- **Issue:** Without a redirect-policy override, a `3xx` response from an operator-configured `base_url` could cause the `Authorization` header carrying the operator's API key to be replayed to a different, attacker-influenced host — the SSRF-adjacent credential-exfiltration path the threat model names as `high` severity.
- **Fix:** Added `redirect_policy: Option<reqwest::redirect::Policy>` to `CompatEngineConfig`, applied in `CompatEngine::new()` via `.redirect(policy)` only when `Some`. Set `None` in the engine's own test config and all four existing presets (Kimi, Qwen, Grok, Ollama) — each preset's `base_url` is a fixed or operator-overridden-but-trusted vendor/local host, not arbitrary operator input, so their behaviour is exactly unchanged. Set `Some(Policy::none())` only in `OpenAiCompatibleAdapter`.
- **Files modified:** `crates/paladin-llm/src/compat/engine.rs`, `crates/paladin-llm/src/kimi/adapter.rs`, `crates/paladin-llm/src/qwen/adapter.rs`, `crates/paladin-llm/src/grok/adapter.rs`, `crates/paladin-llm/src/ollama/adapter.rs` (all outside this plan's stated `files_modified` frontmatter list)
- **Verification:** `cargo test -p paladin-llm` (default features, 53 tests) and `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible` (137 tests) both pass with zero behavioural change to any existing preset; `cargo clippy` clean on every feature combination exercised.
- **Committed in:** `e193b8d` (Task 2 commit)

**2. [Rule 1 - Bug] `four_new_preset_build`'s exact-match assertion would fail under the plan's own combined verification command**
- **Found during:** Task 2, immediately after registering the `openai-compatible` row — plan 17-03's `four_new_preset_build` test module asserts `provider_names() == ["kimi", "qwen", "grok", "ollama"]` exactly, and the plan's own `<verification>` block runs `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible`, under which the table now has five rows.
- **Fix:** Widened the gate to `not(feature = "openai-compatible")` (mirroring the identical pattern plan 17-03 established for this exact gate against qwen/grok/ollama's own additions) and added `five_new_preset_build`, gated the opposite way, asserting the new five-row order directly — turning the plan's own combined-verification acceptance criterion into a real test.
- **Files modified:** `crates/paladin-llm/src/provider_factory.rs`
- **Verification:** `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible` — 137 tests passed, including `five_new_preset_build::provider_names_returns_exactly_kimi_qwen_grok_openai_compatible_ollama_in_table_order`.
- **Committed in:** `e193b8d` (Task 2 commit)

**3. [Rule 1 - Bug] Task 3's own round-trip test panicked on a stray empty `XAI_API_KEY` in this execution environment**
- **Found during:** Task 3, running the plan's combined verification command — `std::env::var("XAI_API_KEY").is_ok()` was `true` (the variable is set) but its value is an empty string, so `construct_grok()` failed `GrokConfig::validate()`'s empty-key check, and the round-trip test's original "credential present == env var Ok" check treated that as an unexpected failure and panicked.
- **Issue:** The test's credential-presence check mirrored `get_default_provider()`'s own `.is_ok()` semantics too literally — "the variable is set" is not the same claim as "the variable holds a usable credential," and every preset's own `validate()` already treats an empty key as absent.
- **Fix:** Changed the check to `std::env::var(var).is_ok_and(|v| !v.trim().is_empty())`, treating an empty/whitespace-only value the same as an absent one (skip, not panic). Documented the rationale inline.
- **Files modified:** `crates/paladin-llm/src/provider_factory.rs`
- **Verification:** `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible` now passes (137/137) in this environment; also manually confirmed (temporary edit, reverted) that the test still correctly fails when zero rows are exercisable, and that `provider_names_has_no_duplicate_entries` still correctly fails on a deliberately introduced duplicate name.
- **Committed in:** `9c67474` (Task 3 commit)

---

**Total deviations:** 3 auto-fixed (1 missing-critical-functionality/security, 2 bugs)
**Impact on plan:** Deviation 1 is a required security mitigation the plan's own threat model specifies; deviations 2 and 3 are mechanical fixes with no behavioural consequence to any shipped adapter, necessary for the plan's own stated verification commands to pass. No scope creep.

## Issues Encountered

- **Task 1 was a blocking `checkpoint:decision`** reached by a prior executor that returned with 0/3 tasks complete and no commits (its worktree was reclaimed). This continuation agent started fresh from the same base commit with the human's `option-a` selection already resolved and recorded in `<resolved_checkpoint>`; Task 1 itself produced no code and no commit — its resolution is implemented directly in Task 2.
- **Vendor facts:** not applicable to this plan — the generic provider has no vendor to cite; every value (`base_url`, `model`, capabilities) is operator-supplied at runtime, so there is no `[CITED]` fact requiring live verification the way Kimi/Qwen/Grok/Ollama's defaults did in plans 17-01/17-03.
- **Empty `XAI_API_KEY` in this execution environment** (see Deviation 3) — an environmental quirk unrelated to this plan's own scope, surfaced only because Task 3's new test happened to exercise Grok's row.

## User Setup Required

None - no external service configuration required. An `OPENAI_COMPATIBLE_API_KEY`, `OPENAI_COMPATIBLE_BASE_URL` and `OPENAI_COMPATIBLE_MODEL` would be required to actually call a live endpoint, but no test in this plan requires any of them — all 34 new-adapter tests run offline against `mockito`.

## Next Phase Readiness

- All five named D-01 presets (Kimi, Qwen, Grok, Ollama, plus the shipped-three) and the generic `openai-compatible` provider now ship. Per D-03, Groq, Together, Mistral, Fireworks and Bedrock are disposed of as *already covered* rather than deferred — a future request for any of them is answered by pointing `OpenAiCompatibleAdapter` at their endpoint, not by a new phase.
- Only Gemini (bespoke protocol, plan 17-05) remains from the PROV-01 build list.
- `OpenAiCompatibleCapabilitiesConfig` already implements `Deserialize`, ready for plan 17-06's config-file loader to consume directly with no adapter-side changes.
- `CompatEngineConfig.redirect_policy` establishes the pattern for any future per-preset transport hardening without forking the engine — plan 17-05's bespoke Gemini adapter does not use `CompatEngine` and is unaffected.
- No blockers.

## Self-Check: PASSED

All created/modified files verified present on disk; both task commit hashes (`e193b8d`, `9c67474`) verified present in `git log`.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
