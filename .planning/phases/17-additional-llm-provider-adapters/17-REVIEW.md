---
phase: 17-additional-llm-provider-adapters
reviewed: 2026-08-17T00:00:00Z
depth: standard
files_reviewed: 27
files_reviewed_list:
  - crates/paladin-llm/src/compat/engine.rs
  - crates/paladin-llm/src/compat/mod.rs
  - crates/paladin-llm/src/compat/types.rs
  - crates/paladin-llm/src/redaction.rs
  - crates/paladin-llm/src/kimi/adapter.rs
  - crates/paladin-llm/src/kimi/mod.rs
  - crates/paladin-llm/src/qwen/adapter.rs
  - crates/paladin-llm/src/qwen/mod.rs
  - crates/paladin-llm/src/grok/adapter.rs
  - crates/paladin-llm/src/grok/mod.rs
  - crates/paladin-llm/src/ollama/adapter.rs
  - crates/paladin-llm/src/ollama/mod.rs
  - crates/paladin-llm/src/openai_compatible/adapter.rs
  - crates/paladin-llm/src/openai_compatible/mod.rs
  - crates/paladin-llm/src/gemini/adapter.rs
  - crates/paladin-llm/src/gemini/mod.rs
  - crates/paladin-llm/src/provider_factory.rs
  - crates/paladin-llm/src/config/bridge.rs
  - crates/paladin-llm/src/config/llm.rs
  - crates/paladin-llm/src/lib.rs
  - crates/paladin-llm/Cargo.toml
  - crates/paladin-llm/README.md
  - tests/unit/llm/provider_factory_test.rs
  - tests/integration/ollama_docker_test.rs
  - docker/docker-compose.test.yml
  - docs/src/api-reference/feature-flags.md
  - docs/src/getting-started/configuration.md
findings:
  critical: 1
  warning: 7
  info: 2
  total: 10
status: issues_found
---

# Phase 17: Code Review Report

**Reviewed:** 2026-08-17T00:00:00Z
**Depth:** standard
**Files Reviewed:** 27 (plus `.project/current-exports.txt`, `src/lib.rs`, and root `Cargo.toml` consulted cross-referentially per finding CR-01/WR-05)
**Status:** issues_found

## Summary

Reviewed the six new LLM provider adapters (Kimi, Qwen, Grok, Ollama, Gemini,
openai-compatible), the shared `compat` engine they largely sit on, the redaction module,
the table-driven provider factory, and the surrounding config/docs/test surface added in
Phase 17. The five OpenAI-compatible presets are structurally near-identical thin wrappers
over `CompatEngine` and are internally consistent and well-tested. `openai_compatible`'s
SSRF mitigation (`Policy::none()` redirect policy, plaintext-to-non-loopback warning) is
correctly implemented and tested.

The bespoke Gemini adapter — flagged by the phase context as the highest-risk file, since
its protocol mapping was written without a live endpoint — has one real defect worth
blocking on: `request.model` is interpolated unescaped into the request URL path, unlike
every compat-engine preset (which carries `model` in the JSON body instead). It also has an
error-classification gap for common real-world Gemini auth-failure shapes, and does not
replicate the compat engine's empty-completion detection for truncated reasoning
completions.

Two further issues degrade robustness/diagnostics without being security-critical: the
Ollama adapter's placeholder "credential" is a common English word, which causes the shared
redaction routine to over-redact benign diagnostic text; and `provider_factory`'s
availability check for `openai-compatible` only verifies one of the adapter's three required
environment variables, so `list_available_providers()`/`get_default_provider()` can report
it as available when construction will actually fail.

## Critical Issues

### CR-01: Gemini interpolates `request.model` unescaped into the request URL

**File:** `crates/paladin-llm/src/gemini/adapter.rs:564-567, 622-625`
**Issue:** `GeminiAdapter::generate()` and `generate_stream()` build the request URL with

```rust
let url = format!(
    "{}/models/{}:generateContent",
    self.config.base_url, request.model
);
```

`request.model` comes from the caller-supplied `LlmRequest` (a field a higher layer — e.g.
a model-selection UI — plausibly threads through from end-user input) and is never
URL-encoded, never checked against `validate_model()`, and never restricted to a
safe character set before being spliced into the path. A `model` value containing `/`,
`?`, `#`, or `:` characters can alter the request path, inject/override query parameters
(e.g. defeating the mandatory `alt=sse` parameter on the streaming path, or appending
attacker-chosen parameters), or otherwise produce a request the operator did not intend —
all while still carrying the configured `x-goog-api-key` credential. Every other adapter in
this crate that sits on `CompatEngine` instead carries `model` inside the JSON request body
(`compat/types.rs::CompatRequest.model`), which `serde_json` encodes safely; Gemini is the
one adapter in this phase that puts it in the URL, and it is the one bespoke-protocol
adapter this phase's own context flagged as unverified against a live endpoint.

**Fix:** Percent-encode the model segment before interpolating it into the path, and/or
reject a `model` value containing characters outside an allow-listed set (e.g.
`[A-Za-z0-9._-]`) before constructing the URL:

```rust
let encoded_model = percent_encoding::utf8_percent_encode(
    &request.model,
    percent_encoding::NON_ALPHANUMERIC,
);
let url = format!(
    "{}/models/{}:generateContent",
    self.config.base_url, encoded_model
);
```

or, more defensively, validate `request.model` against `self.available_models()` /a
restricted character set and return `LlmError::InvalidPrompt` before ever formatting the
URL.

## Warnings

### WR-01: Ollama's placeholder "credential" causes the shared redactor to over-redact benign diagnostic text

**File:** `crates/paladin-llm/src/ollama/adapter.rs:55-60`, `crates/paladin-llm/src/redaction.rs:101-110`
**Issue:** `OLLAMA_PLACEHOLDER_API_KEY = "ollama"` is passed as the engine's `api_key`, which
`CompatEngine::diagnostic_excerpt` / `redact_credentials` uses for an **exact-match
substring replace** across the entire diagnostic body: `body.replace(api_key,
CREDENTIAL_PLACEHOLDER)`. Because `"ollama"` is a common English word (the product's own
name), any error or model-list body that legitimately contains the substring "ollama" —
e.g. `"failed to reach ollama server"`, a model tag like `ollama/llama3`, or simply the
word appearing in a provider error message — has every occurrence silently replaced with
`[REDACTED]`, corrupting the diagnostic text for the one adapter this phase's own Tier-2
suite (`tests/integration/ollama_docker_test.rs`) exercises against a real server. This is
not a leak (over-redaction is the safe direction) but it is a real robustness/diagnostics
defect: operator-facing error messages for Ollama become misleading or nonsensical whenever
the word "ollama" appears in the upstream response for reasons unrelated to credentials.
**Fix:** Skip the exact-match credential pass entirely when the configured "credential" is
the known placeholder (or more generally, when it is empty/known-non-secret), relying on
the `Bearer `/`sk-` shape-based passes instead:

```rust
let is_real_credential = !api_key.is_empty() && api_key != OLLAMA_PLACEHOLDER_API_KEY;
let exact = if is_real_credential { body.replace(api_key, CREDENTIAL_PLACEHOLDER) } else { body.to_string() };
```

### WR-02: `provider_factory`'s availability check for `openai-compatible` verifies only one of its three required env vars

**File:** `crates/paladin-llm/src/provider_factory.rs:282-297, 371-395`
**Issue:** Every registry row's "is this provider available" check
(`get_default_provider()` / `list_available_providers()`) is `std::env::var(var).is_ok()`
against a single `env_var: Option<&'static str>`. For `openai-compatible`, that single
variable is `OPENAI_COMPATIBLE_API_KEY` — but `OpenAiCompatibleConfig::from_env()` (called by
`construct_openai_compatible`) requires **three** variables:
`OPENAI_COMPATIBLE_API_KEY`, `OPENAI_COMPATIBLE_BASE_URL`, and `OPENAI_COMPATIBLE_MODEL`
(see `openai_compatible/adapter.rs:362-397`). If only `OPENAI_COMPATIBLE_API_KEY` is set (a
plausible partial-configuration state), `list_available_providers()` and
`get_default_provider()` report `"openai-compatible"` as available/selected, but a
subsequent `factory.create("openai-compatible")` — or `get_default_provider()` immediately
followed by `create(default)` — fails with `ConfigurationMissing`. A caller that trusts the
"available" signal (a reasonable assumption given the doc comment "have their credential
configured (or need none)") gets a runtime surprise.
**Fix:** Either widen `ProviderRegistration` to carry a list of required env vars (checking
all of them), or add a small per-row "is fully configured" closure so `openai-compatible`'s
row can express its three-variable requirement:

```rust
struct ProviderRegistration {
    name: &'static str,
    env_vars: &'static [&'static str], // ALL must be set
    construct: fn() -> Result<Arc<dyn LlmPort>, ProviderFactoryError>,
}
```

### WR-03: Gemini's `map_error` misclassifies common real-world credential-failure shapes

**File:** `crates/paladin-llm/src/gemini/adapter.rs:389-417`
**Issue:** `map_error` only produces `LlmError::AuthenticationError` for `401 | 403` paired
with `rpc_status == Some("PERMISSION_DENIED")`. Google's Generative Language API commonly
reports an invalid/malformed API key as **`400` with `status: "INVALID_ARGUMENT"`** and a
message such as `"API key not valid. Please pass a valid API key."` — under this adapter's
current mapping that lands on the `400 if rpc_status == Some("INVALID_ARGUMENT")` arm,
which returns `LlmError::InvalidPrompt`, telling the caller their *prompt* is malformed
rather than their *credential*. Separately, any `401`/`403` whose `rpc_status` is something
other than exactly `"PERMISSION_DENIED"` (e.g. `"UNAUTHENTICATED"`) falls through to the
final catch-all arm, which returns the **retryable** `LlmError::ProcessingError` — so an
unrecognised auth-failure shape is retried up to 3 times with exponential backoff before
surfacing, burning quota/latency on a request that cannot succeed. This is exactly the
class of protocol-mapping error the phase context flagged as plausible for this
never-live-tested adapter.
**Fix:** Broaden the authentication-failure match to cover the documented invalid-key shape
and any `401`/`403` regardless of the exact `rpc_status` string, and keep the true
`400`/`INVALID_ARGUMENT` mapping only for arguments that are not credential-shaped:

```rust
401 | 403 => LlmError::AuthenticationError(format!("Gemini authentication failed: {excerpt}")),
400 if rpc_status == Some("INVALID_ARGUMENT")
    && !raw_message.to_ascii_lowercase().contains("api key") =>
    LlmError::InvalidPrompt(excerpt),
400 if rpc_status == Some("INVALID_ARGUMENT") =>
    LlmError::AuthenticationError(format!("Gemini rejected the configured API key: {excerpt}")),
```

### WR-04: Kimi/Qwen/Grok/Ollama/Gemini keep the default follow-redirects policy despite operator-overridable `base_url`

**File:** `crates/paladin-llm/src/kimi/adapter.rs:170-174`, `qwen/adapter.rs:185-189`,
`grok/adapter.rs:170-174`, `ollama/adapter.rs:197-202`
**Issue:** Each of these presets sets `redirect_policy: None` with a comment asserting the
endpoint is "a fixed vendor host, not operator-supplied." That is not accurate: each
adapter's `*_BASE_URL` env var (`MOONSHOT_BASE_URL`, `DASHSCOPE_BASE_URL`, `XAI_BASE_URL`,
`OLLAMA_BASE_URL`, and Gemini's `GEMINI_BASE_URL`) is documented and operator-settable.
Because `redirect_policy: None` preserves `reqwest`'s default (follow up to 10 hops), a
`3xx` response from whatever host `base_url` resolves to — vendor or operator-overridden —
can cause the `Authorization`/`x-goog-api-key` header carrying the real credential to be
replayed to a different host, exactly the class of risk `openai_compatible` explicitly
mitigates with `Policy::none()` (T-17-18). This mirrors a pre-existing pattern already
present for DeepSeek/Anthropic, so it is not a phase-17 regression in isolation, but the
in-code rationale ("not operator-supplied") is factually wrong for every one of these five
presets and could mislead a future maintainer auditing this exact threat.
**Fix:** Either correct the comment to acknowledge the residual exposure is an accepted,
operator-trust-boundary risk (matching `openai_compatible`'s own documented posture), or
extend `Policy::none()` to every preset whose `base_url` is env-var-overridable.

### WR-05: Six new providers are missing from the facade's top-level re-exports

**File:** `src/lib.rs:174-188` (root crate), `Cargo.toml:279-288` (root crate)
**Issue:** The root `Cargo.toml` defines `llm-kimi`, `llm-qwen`, `llm-grok`, `llm-ollama`,
`llm-gemini`, and `llm-openai-compatible` feature flags, each forwarding into the matching
`paladin-llm` feature — mirroring `llm-openai`/`llm-anthropic`/`llm-deepseek` exactly. But
`src/lib.rs` only re-exports adapter types for the original three
(`pub use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig}` etc., gated on their
`llm-*` features); there is no corresponding `pub use paladin_llm::kimi::{KimiAdapter,
KimiConfig}` (or qwen/grok/ollama/gemini/openai_compatible) anywhere in the facade. A
consumer of the `paladin` crate who enables `--features llm-kimi` gets the adapter compiled
into `paladin-llm` but has no `paladin::KimiAdapter` to import — they must depend on
`paladin-llm` directly to reach it, unlike the original three providers. Confirmed against
`.project/current-exports.txt`'s regenerated snapshot (this diff), which lists
`paladin::AnthropicAdapter`/`paladin::DeepSeekAdapter` newly but nothing for the six new
providers.
**Fix:** Add the same `#[cfg(feature = "llm-<provider>")] pub use paladin_llm::<provider>::{...}`
pattern for each of the six new providers, or explicitly document that only
`LlmProviderFactory`-mediated (string-keyed) access is supported for them going forward.

### WR-06: Gemini never detects a truncated/empty completion the way every compat-engine preset does

**File:** `crates/paladin-llm/src/gemini/adapter.rs:333-362`
**Issue:** `CompatEngine::detect_empty_completion` (used by Kimi/Qwen/Grok/Ollama/
openai-compatible) raises `LlmError::EmptyCompletion` when `finish_reason == Length` and
the visible content is empty — the "reasoning consumed the whole budget" signature. Gemini's
`parse_response` has no equivalent check: a `MAX_TOKENS` finish reason with empty
`candidate_text(candidate)` (a real, achievable case for a Gemini "thinking" model whose
hidden reasoning exhausts `maxOutputTokens`) returns `Ok(LlmResponse { content: "", finish_reason:
FinishReason::Length, .. })` instead of surfacing the truncation as an error, silently
diverging from every other adapter's contract in this crate.
**Fix:** Mirror the compat engine's check in `parse_response`:

```rust
if matches!(finish_reason, FinishReason::Length) && content.trim().is_empty() {
    return Err(LlmError::EmptyCompletion(format!(
        "finish_reason=MAX_TOKENS with empty content — reasoning likely consumed the \
         entire max_output_tokens budget; retry with a larger budget"
    )));
}
```

### WR-07: `CompatEngine::call_api_with_retry`'s post-loop error path is unreachable

**File:** `crates/paladin-llm/src/compat/engine.rs:317-357`
**Issue:** The retry loop is `for attempt in 0..=max_retries { ... }`. On every iteration,
either the operation succeeds (`return Ok`), the error is non-retryable (`return Err(e)`),
or `attempt >= max_retries` (`return Err(e)`) — and the last possible value of `attempt`
in the range is exactly `max_retries`, which always satisfies `attempt >= max_retries`. The
loop therefore can never run to completion without returning from inside it, which makes
the `last_error` variable and the trailing `Err(last_error.unwrap_or_else(|| ...))` after
the loop dead code that can never execute. This is not presently causing incorrect
behaviour, but it is misleading: it reads as a real fallback path and could hide a future
off-by-one regression (e.g. if the loop bound is ever changed to `0..max_retries`) since
the "dead" branch would silently start executing with a stale/synthetic error rather than
failing a test.
**Fix:** Restructure as a `loop { }` with an explicit break/return on the final attempt (as
`GeminiAdapter::execute_with_retry` already does), removing the unreachable trailing
`Err(...)` expression, or add a `#[allow(unreachable_code)]`-documented comment explaining
why the dead branch is intentionally retained as a defensive fallback.

## Info

### IN-01: `compat/mod.rs` module doc is stale

**File:** `crates/paladin-llm/src/compat/mod.rs:1-7`
**Issue:** The module doc still reads "shared by every OpenAI-compatible provider preset
(Kimi, and future presets under this feature gate)" — Qwen, Grok, Ollama, and
`openai-compatible` have all since landed on this engine (per `lib.rs`'s own `cfg(any(...))`
gate list and this phase's own plans 17-03/17-04).
**Fix:** Update the doc comment to name the current preset set, or phrase it generically
("every OpenAI-compatible preset in this crate") so it does not need updating per preset
added.

### IN-02: Inconsistent `base_url` scheme validation across presets

**File:** `crates/paladin-llm/src/kimi/adapter.rs:123-125`, `qwen/adapter.rs:134-136`,
`grok/adapter.rs:123-125`, `ollama/adapter.rs:144-146`,
`openai_compatible/adapter.rs:435-437`, vs. `gemini/adapter.rs:168-170`
**Issue:** Five presets validate with `!self.base_url.starts_with("http")`, which accepts a
malformed scheme like `httpfoo://` or `httpsevil://host` (anything merely prefixed with the
four characters `http`). `GeminiConfig::validate()` instead correctly checks
`!starts_with("http://") && !starts_with("https://")`. The looser check would still fail
later at `reqwest::Url` parse/request time, so the practical impact is a less specific error
message rather than a functional gap, but the inconsistency is worth normalizing.
**Fix:** Standardize every preset's `validate()` on the stricter Gemini-style check.

---

_Reviewed: 2026-08-17T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
