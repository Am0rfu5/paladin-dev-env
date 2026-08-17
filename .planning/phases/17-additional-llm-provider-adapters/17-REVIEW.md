---
phase: 17-additional-llm-provider-adapters
reviewed: 2026-08-17T00:00:00Z
depth: standard
files_reviewed: 27
files_reviewed_list:
  - .project/current-exports.txt
  - crates/paladin-llm/Cargo.toml
  - crates/paladin-llm/README.md
  - crates/paladin-llm/src/compat/engine.rs
  - crates/paladin-llm/src/compat/mod.rs
  - crates/paladin-llm/src/compat/types.rs
  - crates/paladin-llm/src/config/bridge.rs
  - crates/paladin-llm/src/config/llm.rs
  - crates/paladin-llm/src/gemini/adapter.rs
  - crates/paladin-llm/src/gemini/mod.rs
  - crates/paladin-llm/src/grok/adapter.rs
  - crates/paladin-llm/src/grok/mod.rs
  - crates/paladin-llm/src/kimi/adapter.rs
  - crates/paladin-llm/src/kimi/mod.rs
  - crates/paladin-llm/src/lib.rs
  - crates/paladin-llm/src/ollama/adapter.rs
  - crates/paladin-llm/src/ollama/mod.rs
  - crates/paladin-llm/src/openai_compatible/adapter.rs
  - crates/paladin-llm/src/openai_compatible/mod.rs
  - crates/paladin-llm/src/provider_factory.rs
  - crates/paladin-llm/src/qwen/adapter.rs
  - crates/paladin-llm/src/qwen/mod.rs
  - crates/paladin-llm/src/redaction.rs
  - docker/docker-compose.test.yml
  - docs/src/api-reference/feature-flags.md
  - docs/src/getting-started/configuration.md
  - tests/integration/ollama_docker_test.rs
  - tests/unit/llm/provider_factory_test.rs
findings:
  critical: 1
  warning: 4
  info: 1
  total: 6
status: issues_found
---

# Phase 17: Code Review Report

**Reviewed:** 2026-08-17T00:00:00Z
**Depth:** standard
**Files Reviewed:** 27 (source/doc files touched by this phase; the crate compiles clean and `cargo clippy -p paladin-llm --all-features --tests -- -D warnings` is clean)
**Status:** issues_found

## Summary

This review covers the six new LLM provider presets (Kimi, Qwen, Grok, Ollama, Gemini,
OpenAI-compatible), the shared `CompatEngine`, the provider registry/factory, config bridge,
and the redaction helper added/extended in Phase 17, plus the associated tests and docs.

The previously-reported CR-01 (Gemini path-injection via `model`), WR-03 (Gemini
credential-failure misclassification) and WR-04 (redirect-based credential replay) gaps
described in the earlier review have been re-verified here, not just trusted: I read the
fix code and its regression tests, and ran the full `paladin-llm` test suite (273 tests,
including the redirect-refusal and model-identifier-guard regression tests) — all pass.
No regression was found in any of the three fixes.

However, direct execution of this phase's own workspace-level test file
(`tests/unit/llm/provider_factory_test.rs`) under the crate's own documented,
CI-exercised `llm-all` feature combination reproducibly fails two tests
(`test_get_default_provider`, `test_list_available_providers`) against this very
sandbox's ambient environment. That failure is not a fluke of this sandbox — it traces to
a genuine logic defect in `provider_factory.rs` (see CR-01 below) that will misfire in any
real deployment where an unrelated, unconfigured provider's credential env var happens to
be present but empty (a common pattern: `.env` templates, Docker/K8s environments that
pass through unset host vars as empty strings — this repository's own `.env` is itself an
example). I am flagging this as the review's one Critical finding. The remaining findings
are quality/consistency gaps (Warning) and one documentation note (Info).

## Critical Issues

### CR-01: `list_available_providers()` / `get_default_provider()` treat an empty-but-set credential env var as "configured"

**File:** `crates/paladin-llm/src/provider_factory.rs:371-395`
**Also affects:** `tests/unit/llm/provider_factory_test.rs:185-226` (`test_get_default_provider`), `tests/unit/llm/provider_factory_test.rs:229-265` (`test_list_available_providers`)

**Issue:** Both `LlmProviderFactory::get_default_provider()` and
`LlmProviderFactory::list_available_providers()` decide whether a provider's credential is
"configured" with:

```rust
match row.env_var {
    Some(var) => std::env::var(var).is_ok(),
    None => true,
}
```

`std::env::var(var).is_ok()` is `true` for an env var that is set to the **empty string** —
it only distinguishes "set" from "unset," not "set to a usable value." Every adapter's own
`*Config::validate()` (e.g. `GrokConfig::validate()`,
`crates/paladin-llm/src/grok/adapter.rs:117-118`) correctly rejects an empty `api_key`, so
`factory.create(name)` for such a provider fails with `ConfigurationMissing` — but
`list_available_providers()` had already reported that same provider as *available*,
directly contradicting its own doc comment: "the names of all providers that are both
compiled in and have their credential configured (**or need none**)." A caller that trusts
`list_available_providers()`/`get_default_provider()` to hand back a provider that will
actually construct is misled.

This is not hypothetical. The crate's own test suite already knows the correct check —
`provider_factory.rs`'s `provider_name_round_trip` test module (lines 714-773) explicitly
uses `std::env::var(var).is_ok_and(|v| !v.trim().is_empty())` and documents exactly why:
"an env var set to an empty (or whitespace-only) string is *set* but is not a usable
credential." That fix was applied to a *test*, never to the production code it was
guarding against.

**Reproduction (verified in this sandbox):**

```
$ env | grep -i KEY
GEMINI_API_KEY=
XAI_API_KEY=
...

$ cargo test --test unit --features llm-all -- provider_factory --test-threads=1
---- llm::provider_factory_test::test_get_default_provider stdout ----
thread '...' panicked: assertion `left == right` failed
  left: Some("grok")
 right: None

---- llm::provider_factory_test::test_list_available_providers stdout ----
thread '...' panicked: assertion `left == right` failed
  left: 3
 right: 0
```

With `PROVIDER_ENV_LOCK` clearing only `OPENAI_API_KEY`/`DEEPSEEK_API_KEY`/
`ANTHROPIC_API_KEY` (the three original providers), `GEMINI_API_KEY=""` and
`XAI_API_KEY=""` (both merely *present*, ambient in this sandbox's `.env`) are enough to
make `grok` and `gemini` — and `ollama`, which needs no credential at all — all read back
as "available" with zero real credentials configured. This reproduces **deterministically**
with `--test-threads=1`, so it is not a cross-test race; it is the `.is_ok()` defect alone.
`--features llm-all` is a first-class, CI-defined build target
(`.github/workflows/feature-flags.yml`'s `llm-all` matrix leg, and the `full` convenience
feature in `feature-flags.md`), so this is a supported configuration, not an edge case.

Note also that this specific CI leg would not have caught the failure even with the
defect present: `feature-flags.yml`'s matrix runs `cargo test --workspace --lib
${{ matrix.flags }}`, and `--lib` does not execute the `tests/unit/mod.rs`
(`provider_factory_test.rs`) integration-test binary at all. The one CI job that *does*
run the full `--workspace` suite without `--lib`
(`ci.yml:502`, `cargo test --workspace --features integration-tests ...`) does not add
`llm-all`, so it never compiles the six new provider rows into the registry either. The
defect is real and currently invisible to CI in every configuration CI actually runs.

**Fix:** Apply the same `is_ok_and(|v| !v.trim().is_empty())` check the crate's own
`provider_name_round_trip` test already uses, in the two production call sites:

```rust
pub fn get_default_provider() -> Option<String> {
    provider_registry()
        .iter()
        .find(|row| match row.env_var {
            Some(var) => std::env::var(var).is_ok_and(|v| !v.trim().is_empty()),
            None => true,
        })
        .map(|row| row.name.to_string())
}

pub fn list_available_providers() -> Vec<String> {
    provider_registry()
        .iter()
        .filter(|row| match row.env_var {
            Some(var) => std::env::var(var).is_ok_and(|v| !v.trim().is_empty()),
            None => true,
        })
        .map(|row| row.name.to_string())
        .collect()
}
```

Additionally, `test_get_default_provider` and `test_list_available_providers` should guard
against the six new-provider env vars too (they call functions that scan the *entire*
registry, not just the three original providers) — either merge `CleanProviderEnv` and
`CleanNewProviderEnv` into one guard covering all nine vars, or have these two tests
acquire both locks. The current split's rationale ("a disjoint variable set... so a
separate lock avoids serializing tests that touch unrelated variables," provider_factory
_test.rs:294-300) does not hold for these two tests specifically, since
`get_default_provider()`/`list_available_providers()` treat the two "disjoint" sets as one
union.

## Warnings

### WR-01: `LlmProviderFactory::create()` does not accept the `openai_compatible` (underscore) alias that `LlmConfig` does

**File:** `crates/paladin-llm/src/provider_factory.rs:352-363`

**Issue:** `LlmConfig::get_provider_config()`
(`crates/paladin-llm/src/config/llm.rs:153-166`) explicitly accepts both
`"openai-compatible"` and `"openai_compatible"` (case-insensitively) and
`LlmConfig::validate()`'s `is_recognised_provider_field_name` accepts both spellings for
`default_provider` too — so `default_provider: "openai_compatible"` is a config file that
validates successfully. But `LlmProviderFactory::create()` matches only the registry's
literal name (`"openai-compatible"`, hyphenated) after lower-casing:

```rust
let lower = provider_name.to_lowercase();
provider_registry().iter().find(|row| row.name == lower)
```

Any caller that plumbs `LlmConfig::get_default_provider_name()` (or a config value that
passed `LlmConfig::validate()`) straight into `LlmProviderFactory::create()` — the natural
composition of these two types — gets an unexpected `UnknownProvider` for the underscore
spelling, even though the config layer accepted it as valid.

**Fix:** Normalize the underscore variant in `create()` the same way `get_provider_config`
does, e.g. `let lower = provider_name.to_lowercase().replace('_', "-");` before the
registry lookup (or narrower: special-case `"openai_compatible" => "openai-compatible"`).

### WR-02: `OPENAI_COMPATIBLE_TEMPERATURE_MIN`/`_MAX` accepts an inverted range

**File:** `crates/paladin-llm/src/openai_compatible/adapter.rs:222-248` (`parse_temperature_range_env`)

**Issue:** `parse_temperature_range_env` validates that both `min` and `max` are set (or
neither) and that each parses as `f32`, but never checks `min <= max`. An operator setting
`OPENAI_COMPATIBLE_TEMPERATURE_MIN=2.0` and `OPENAI_COMPATIBLE_TEMPERATURE_MAX=0.0` (e.g. a
copy-paste swap) gets `Some((2.0, 0.0))` accepted silently and surfaced as
`ProviderCapabilities::temperature_range`, an inverted range every other adapter in this
crate declares with the tuple always ordered `(min, max)` (see `GrokConfig`'s
`Some((0.0, 2.0))`, `KimiConfig`'s `Some((0.0, 1.0))`, etc.). Any downstream consumer that
clamps a requested temperature into this range (e.g. `if t < range.0 { t = range.0 }`)
will silently misbehave against an inverted tuple.

**Fix:** Add a `min <= max` check to `parse_temperature_range_env`'s `(Some, Some)` arm,
returning a configuration error (matching this function's existing error-on-half-set
behavior) when violated:

```rust
(Some(min_raw), Some(max_raw)) => {
    let min: f32 = min_raw.trim().parse()...?;
    let max: f32 = max_raw.trim().parse()...?;
    if min > max {
        return Err(format!(
            "OPENAI_COMPATIBLE_TEMPERATURE_MIN ({min}) must not exceed \
             OPENAI_COMPATIBLE_TEMPERATURE_MAX ({max})"
        ));
    }
    Ok(Some((min, max)))
}
```

### WR-03: Gemini's `parse_response` does not detect the reasoning-truncation-to-empty-content case every `CompatEngine` preset detects

**File:** `crates/paladin-llm/src/gemini/adapter.rs:400-429` (`parse_response`)

**Issue:** `CompatEngine::detect_empty_completion`
(`crates/paladin-llm/src/compat/engine.rs:277-287`) explicitly maps `finish_reason ==
Length` (i.e. the vendor truncated the response) combined with empty content to
`Err(LlmError::EmptyCompletion(...))`, on the documented rationale that this is the
"reasoning model consumed its whole `max_tokens` budget" failure signature, and every
Kimi/Qwen/Grok/Ollama/openai-compatible call goes through that check. `GeminiAdapter::
parse_response` has no equivalent: a Gemini response with `finishReason: "MAX_TOKENS"` and
an empty `parts[]` array (the same underlying failure mode — a reasoning-capable Gemini
model consuming its token budget before producing visible text) returns `Ok(LlmResponse
{ content: "", finish_reason: FinishReason::Length, .. })` instead of the
`EmptyCompletion` error every other preset in this crate gives for the identical
condition. A caller written against this crate's error-signal contract (checking for
`LlmError::EmptyCompletion` rather than inspecting `finish_reason` on every success path)
silently gets a truncated-empty "success" only from the Gemini adapter.

**Fix:** Add the same check `parse_response` already has available (it already computes
`finish_reason` and `content` before constructing the response):

```rust
if matches!(finish_reason, FinishReason::Length) && content.trim().is_empty() {
    return Err(LlmError::EmptyCompletion(
        "Gemini finishReason=MAX_TOKENS with empty content — reasoning likely \
         consumed the entire max_tokens budget; retry with a larger max_tokens"
            .to_string(),
    ));
}
```

### WR-04: `generate_stream()` never retries a transient failure on stream-open, unlike `generate()`

**File:** `crates/paladin-llm/src/gemini/adapter.rs:756-809`, `crates/paladin-llm/src/compat/engine.rs:486-582`

**Issue:** Both `GeminiAdapter::generate()` and every `CompatEngine`-based preset's
`generate()` wrap the outbound request in a retry-with-backoff loop
(`execute_with_retry`/`call_api_with_retry`, non-retryable set: auth/invalid-prompt/
empty-completion/usage-limit). Neither `GeminiAdapter::generate_stream()` nor
`CompatEngine::generate_stream()` wraps the initial POST in any retry logic at all — a
single transient network blip (a `NetworkError` or `Timeout` opening the connection,
before any bytes are streamed) fails the whole call immediately, where the exact same
failure on the non-streaming path would be retried up to `max_retries` times. This is an
inconsistency between the two methods of the same `LlmPort` implementation, not a
regression this phase introduced (the pattern is inherited from the DeepSeek adapter this
engine was extracted from) — but every provider this phase adds inherits it, so it is
worth recording here rather than letting it look intentional-by-omission.

**Fix:** At minimum, document the asymmetry on `generate_stream()`'s rustdoc (so it is a
recorded decision rather than a silent gap); ideally, wrap the connection-opening POST
(only — not the byte stream itself) in the same retry helper, since the failure being
retried occurs before any SSE bytes are read.

## Info

### IN-01: `.project/current-exports.txt` was regenerated under this build's default feature set only

**File:** `.project/current-exports.txt`

**Issue:** The regenerated public-API surface snapshot lists only the three default
providers (`OpenAIAdapter`, `DeepSeekAdapter`, `AnthropicAdapter`) via
`pub use paladin::LlmProviderFactory`/etc. — none of the six new adapter types
(`KimiAdapter`, `QwenAdapter`, `GrokAdapter`, `OllamaAdapter`, `GeminiAdapter`,
`OpenAiCompatibleAdapter`) appear anywhere in the file, because it was generated (per its
own header) without `--features llm-all`. This is consistent with the facade crate's
unchanged default feature set (D-11) and is not itself wrong, but it means this snapshot
cannot be used to detect an accidental public-API break in any of the six new adapters —
only in the three defaults. If this file is meant to gate public-API drift for the whole
crate (its stated purpose: "tracks all publicly exported items from the paladin crate"),
consider also generating (or documenting the absence of) an `--all-features` variant.

**Fix:** No action required for this phase to ship; note for a follow-up: regenerate (or
add a sibling) `current-exports.txt` under `--features llm-all` so the six new adapters'
public surface is tracked too.

---

_Reviewed: 2026-08-17T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
