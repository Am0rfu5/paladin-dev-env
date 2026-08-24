---
phase: 17-additional-llm-provider-adapters
reviewed: 2026-08-18T00:00:00Z
depth: standard
files_reviewed: 29
files_reviewed_list:
  - .github/workflows/feature-flags.yml
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
  critical: 0
  warning: 2
  info: 3
  total: 5
status: issues_found
---

# Phase 17: Code Review Report (Round 3)

**Reviewed:** 2026-08-18T00:00:00Z
**Depth:** standard
**Files Reviewed:** 29
**Status:** issues_found

## Summary

This is the third review pass on this phase. All findings from round 2 (CR-01, WR-01
through WR-04) were independently re-verified against the live tree and confirmed fixed
before this pass began, per the task's briefing — none are re-reported here.

This pass read every file in scope directly (no reliance on prior summaries) and traced the
provider registry, the shared `CompatEngine`, all six new adapters, the config bridge, the
YAML config docs, and the two test files end to end. The six new adapters
(kimi/qwen/grok/ollama/gemini/openai-compatible) are structurally sound, consistently
redirect-safe, and consistently redact credentials before any diagnostic text is emitted.
`unwrap()`/`expect()`/`panic!` are absent from every non-test code path in the reviewed
files, matching CLAUDE.md's library-code convention.

Two new WARNING-level findings surfaced, both genuine functional gaps rather than style
preferences:

1. `provider_factory.rs`'s `list_available_providers()`/`get_default_provider()` can report
   `"openai-compatible"` as configured/default when only its API key env var is set, even
   though `create()` also requires `OPENAI_COMPATIBLE_BASE_URL`/`OPENAI_COMPATIBLE_MODEL` and
   will then fail — violating the function's own documented contract for this one row.
2. The Ollama YAML example in `docs/src/getting-started/configuration.md` instructs the
   reader to omit `api_key` entirely, but `LlmProviderConfig.api_key` is a required
   (non-`Option`) `String` field with no `#[serde(default)]`, so following the doc literally
   produces a config-deserialization failure.

Three INFO-level observations are included for completeness; none are blocking.

## Warnings

### WR-01: `openai-compatible`'s registry row can be reported "available" when it cannot actually construct

**File:** `crates/paladin-llm/src/provider_factory.rs:282-287, 387-395, 406-415`

**Issue:** `ProviderRegistration` tracks exactly one "is this provider configured" signal per
row: `env_var: Option<&'static str>`. For every curated preset (openai, deepseek, anthropic,
kimi, qwen, grok, gemini) this is sufficient, because every other required field
(`base_url`, `model`) has a vendor-supplied default (see each `*Config::from_parts`). Ollama
needs no credential at all and also defaults every field, so `env_var: None` is likewise
sufficient there.

`openai-compatible` is the one row where this single-variable model breaks down: by design
(D-03), it has **no** defensible default for `base_url` or `model` —
`OpenAiCompatibleConfig::from_parts` (`openai_compatible/adapter.rs:409-443`) returns `Err`
if either is absent. The registry row only wires up `OPENAI_COMPATIBLE_API_KEY`
(`provider_factory.rs:285`), so:

- `list_available_providers()` (`provider_factory.rs:406-415`) will include
  `"openai-compatible"` whenever only `OPENAI_COMPATIBLE_API_KEY` is set — its own doc
  comment states "a name reported here is always one `create()` can actually construct"
  (`provider_factory.rs:404-405`), which is false for this row when `BASE_URL`/`MODEL` are
  unset.
- `get_default_provider()` (`provider_factory.rs:376-395`) can select `"openai-compatible"`
  as the default provider under the same condition.
- `factory.create("openai-compatible")` then fails with `ProviderFactoryError::ConfigurationMissing`
  naming the missing `BASE_URL`/`MODEL` vars — a caller that trusted either of the two
  functions above to mean "this will construct" gets a runtime failure instead.

This is not a hypothetical: an operator who sets `OPENAI_COMPATIBLE_API_KEY` in their shell
profile (e.g. left over from a previous session, or set ahead of the other two vars during a
staged rollout) will have this provider silently selected as "available" or even as the
*default* provider, and any code that calls `factory.create(default)` unconditionally at
startup will fail. The existing test suite does not catch this: every test in
`tests/unit/llm/provider_factory_test.rs` that exercises the `openai-compatible` row sets all
three variables together (see the `if compiled.contains(&"openai-compatible")` blocks in
`list_available_providers_preserves_registry_declaration_order` and
`get_default_provider_breaks_ties_by_declaration_order`), so the partial-configuration case
is never exercised.

**Fix:** Either (a) widen `ProviderRegistration` to carry a small `is_configured: fn() -> bool`
closure per row instead of a single `env_var`, so the `openai-compatible` row can check all
three required vars, or (b) narrow the doc comments on `get_default_provider()`/
`list_available_providers()` to state the true, weaker guarantee ("credential is present;
`create()` may still fail for providers with additional required configuration") and add a
regression test pinning the partial-configuration case. Option (a) is preferable since it
keeps the documented "always constructible" contract true for every row rather than weakening
it:

```rust
struct ProviderRegistration {
    name: &'static str,
    is_configured: fn() -> bool,
    construct: fn() -> Result<Arc<dyn LlmPort>, ProviderFactoryError>,
}

#[cfg(feature = "openai-compatible")]
fn openai_compatible_is_configured() -> bool {
    let non_blank = |v: &str| std::env::var(v).is_ok_and(|s| !s.trim().is_empty());
    non_blank("OPENAI_COMPATIBLE_API_KEY")
        && non_blank("OPENAI_COMPATIBLE_BASE_URL")
        && non_blank("OPENAI_COMPATIBLE_MODEL")
}
```

### WR-02: Ollama config docs instruct an omission that fails to deserialize

**File:** `docs/src/getting-started/configuration.md:68-72`, `crates/paladin-llm/src/config/llm.rs:9-11`

**Issue:** The docs' Ollama example reads:

```yaml
  # Ollama (self-hosted) requires no api_key at all (D-12) — omit the field entirely.
  ollama:
    base_url: "http://localhost:11434/v1"
    default_model: "llama3"
    timeout_seconds: 60
```

`LlmProviderConfig` (`config/llm.rs:9-11`) declares:

```rust
pub struct LlmProviderConfig {
    pub api_key: String,        // required, no #[serde(default)]
    pub base_url: Option<String>,
    ...
}
```

`api_key` is a plain (non-`Option`) `String` with no `#[serde(default)]`. Serde's derived
`Deserialize` requires a present key for every non-`Option`, non-defaulted field — an
`Option<T>` field is defaulted to `None` automatically when its key is absent, but a bare
`String` is not. `LlmConfig`'s own test file states this is deliberately format-independent
("a JSON object and a YAML mapping exercise the identical generated code path",
`config/llm.rs:376-379`), and the crate's own fixtures confirm the actual supported shape is
an *explicit empty string*, not an omitted key — e.g. the D-12 regression test
`test_llm_config_validate_ollama_empty_api_key_is_allowed` (`config/llm.rs:483-498`) and the
six-provider fixture (`config/llm.rs:433`) both write `"api_key": ""` for ollama, never omit
the key.

So an operator who copies the documented YAML block verbatim — omitting `api_key` under
`ollama:` — will get a config-deserialization error (`missing field 'api_key'`) at startup,
not the "no credential needed" behavior the comment promises.

**Fix:** Either correct the docs to show the required empty string:

```yaml
  ollama:
    api_key: ""   # required key, but Ollama itself needs no credential (D-12)
    base_url: "http://localhost:11434/v1"
    default_model: "llama3"
    timeout_seconds: 60
```

or add `#[serde(default)]` to `LlmProviderConfig::api_key` so an absent key genuinely
defaults to `""` and the doc's "omit the field entirely" instruction becomes true:

```rust
#[serde(default)]
pub api_key: String,
```

The latter is the more robust fix — it also protects any other credential-free provider a
future phase adds from the same doc/schema mismatch.

## Info

### IN-01: `CompatEngineConfig.model` / `GeminiConfig.model` are recorded but never consulted

**File:** `crates/paladin-llm/src/compat/engine.rs:196-210, 407-500`, `crates/paladin-llm/src/gemini/adapter.rs:727-733, 807-816`

Every preset threads its default model (`KIMI_DEFAULT_MODEL`, `GEMINI_DEFAULT_MODEL`, etc.)
into `CompatEngineConfig.model` / `GeminiConfig.model`, but neither `CompatEngine::generate`/
`generate_stream` nor `GeminiAdapter::generate`/`generate_stream` ever read `self.config.model`
— every outgoing request uses `request.model` (the caller-supplied value) exclusively. In
practice every caller in this workspace sets `LlmRequest.model` explicitly, so this has no
observed runtime effect, but the field is effectively dead: an operator who sets
`KIMI_MODEL`/`GEMINI_MODEL` expecting it to act as a true fallback when a caller passes no
preference has no such fallback available, because `LlmRequest.model` is a required
(non-`Option`) field with no adapter-level default path at all. This mirrors the same pattern
already present in `deepseek/adapter.rs` (the file this engine was extracted from, out of
scope for this phase), so it is not a regression — just worth noting since the doc comments
for the six new presets' `*_MODEL` env vars imply a default that never actually applies.

### IN-02: SSE parsing has no cross-chunk buffering

**File:** `crates/paladin-llm/src/compat/engine.rs:576-632`, `crates/paladin-llm/src/gemini/adapter.rs:1153-1190`

Both `CompatEngine::generate_stream`'s `flat_map` closure and `GeminiAdapter::parse_sse_chunk`
process each network chunk's lines independently, with no buffer carried across chunk
boundaries. If a single `data: {...}` SSE event is split across two TCP chunks at a point
other than a line boundary (plausible for a slow or misbehaving upstream, not observed in
this workspace's mock-based tests, which always write complete lines), the half-line in each
chunk fails the `strip_prefix("data: ")` check and is silently discarded via `continue` —
no error surfaces to the caller, and the delta is lost rather than replayed or reported. This
matches the shipped `openai`/`anthropic`/`deepseek` adapters' pre-existing streaming
architecture (out of scope), so it is inherited rather than newly introduced. Flagged for
awareness only — reassembling across chunk boundaries would require carrying partial-line
state in the stream's closure, which is a larger change than this phase's stated scope.

### IN-03: `stop_sequences` silently dropped by every `CompatEngine`-based preset

**File:** `crates/paladin-llm/src/compat/types.rs:14-28`, `crates/paladin-llm/src/compat/engine.rs:196-210`, `crates/paladin-llm/src/gemini/adapter.rs:367-374`

`PromptParameters.stop_sequences` (`paladin-core`) is read and forwarded to the wire request
by `GeminiAdapter::build_request` (`gemini/adapter.rs:373`), but `CompatRequest` (the shared
wire type for kimi/qwen/grok/ollama/openai-compatible) has no corresponding field at all —
`CompatEngine::build_request` never reads `params.stop_sequences`. Any stop sequence a caller
configures is therefore silently accepted by `PromptParameters` but never transmitted to five
of the six new providers, with no error or log line. This mirrors the pre-existing
`openai`/`anthropic`/`deepseek` behavior (confirmed by grep — none of those three adapters
reference `stop_sequences` either), so it is not a phase-17 regression, but the new
providers' own docs (`README.md`, `docs/src/getting-started/configuration.md`) do not call
out this limitation the way they call out other capability gaps (tool calling, vision), which
would help an operator who configures a stop sequence understand why it has no effect.

---

_Reviewed: 2026-08-18T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
