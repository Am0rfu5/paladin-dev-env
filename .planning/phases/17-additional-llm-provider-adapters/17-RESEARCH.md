# Phase 17: Additional LLM Provider Adapters - Research

**Researched:** 2026-08-16
**Domain:** Rust LLM provider adapters (OpenAI-compatible protocol + Google Gemini bespoke protocol), Hexagonal Architecture adapter layer
**Confidence:** MEDIUM (wire-level facts CITED against official docs but not tool-verified via `npm view`-equivalent; shipped-code facts are HIGH/VERIFIED via direct file reads)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Inherited (informational, not re-litigated):** D-00a…D-00s — ADR conventions, precedence order,
evidence bar, requirement-ID-as-primary-key, the 82% coverage floor (ADR-0006, live and required),
ADR-0004 provider-aware `temperature_range`, ADR-0016 port value-type ownership (`TokenUsage` etc.
live in `paladin-core`/`paladin-ports` — no adapter here redefines it), ADR-0031 extracted-crate
dependency rule (default-build invariant, checkable via `cargo tree --no-default-features`).

**PROV-01 — which providers qualify:**
- **D-01:** The build list is **Kimi, Qwen, Grok, Ollama and Gemini**. Every OpenAI-compatible
  candidate is in because marginal cost approaches zero once the shared core exists; Gemini is in
  as the one bespoke adapter worth paying for.
- **D-02:** **Ollama settles the "Meta (Llama)" row**; Groq and Together are not built. Ollama is
  the only candidate testable without anyone's API key, gives self-hosted coverage, and fits the
  Docker-gated Tier 2 shelf.
- **D-03:** **A generic operator-configured OpenAI-compatible provider ships as public surface** —
  `base_url` + key + model + declared capabilities, no new code. This changes disposition of the
  rest of the field: Groq, Together, Mistral, Fireworks, Bedrock are **rejected as already
  covered**, not deferred. (Reversibility: one-way — public API.)
- **D-04:** **The generic provider's capabilities are operator-declared with pessimistic
  defaults.** Unset means the conservative answer: `supports_streaming: true` (in-spec),
  `supports_tool_calling`/`supports_function_calling`/`supports_vision`/`supports_embeddings` all
  `false`, `temperature_range: None` (global `[0.0, 1.0]` applies), `max_context_tokens: None`. It
  must never claim a capability nobody asserted. (Reversibility: costly — tightening later breaks
  working configs.)

**PROV-02 — adapter shape and the `LlmPort` contract:**
- **D-05:** **A shared OpenAI-compatible core owns the protocol; named providers are thin
  presets.** One engine owns request/response types, streaming chunk assembly, retry, and error
  mapping into `LlmError`. Kimi/Qwen/Grok/Ollama each supply only `base_url`, env-var name,
  default model, model list, capabilities block. (Reversibility: costly — all presets + generic
  provider depend on this core.)
- **D-06:** **The three shipped adapters (`openai/`, `anthropic/`, `deepseek/`) are NOT migrated
  onto the core.** This phase is additive only. The duplication is a recorded deferred idea.
- **D-07:** **`LlmPort` is not changed; the generic provider returns a fixed literal name**
  (`"openai-compatible"`). Presets return their own literals (`"kimi"`, `"qwen"`, `"grok"`,
  `"ollama"`, `"gemini"`). Widening `get_provider_name() -> &'static str` to `&str` was rejected
  (breaking change, `api-surface` guard consequence). Accepted cost: two generic-provider instances
  pointed at different endpoints report the same name in logs.
- **D-08:** **Gemini is bespoke and text-only.** Cannot use the compatible core —
  `generateContent` is its own protocol, closer to the 1,180-line Anthropic adapter. Implements
  `LlmPort`, reports `supports_vision: false`. Vision is a different, deferred surface.
- **D-09:** **The generic provider gets a plain technical name, not medieval-military**:
  `OpenAiCompatibleAdapter` / `"openai-compatible"`. Per D-00h, ubiquitous language binds coined
  domain nouns, not protocol terms of art (same class as `LlmPort`, `ProviderCapabilities`).

**PROV-03 — factory, feature flags and naming:**
- **D-10:** **A table-driven registry replaces the factory's hardcoded match.** One `cfg`-gated
  static table of `(name, env var, constructor)` becomes the single source; `create()`,
  `get_default_provider()`, `list_available_providers()`, and the `UnknownProvider` error text all
  derive from it. This structurally fixes a live defect: `get_default_provider`/
  `list_available_providers` at `provider_factory.rs:123-149` are **not** `cfg`-gated today and
  report a provider available whenever its env var is set, even when its feature is compiled out.
- **D-11:** **The facade's `llm-*` flags are wired for real — BREAKING.** Root `Cargo.toml` today
  declares `llm-openai/llm-anthropic/llm-deepseek = []` (empty stubs) while unconditionally pulling
  `paladin-llm` with `features = ["openai", "anthropic", "deepseek", "mock", "vision"]` at line 55
  — every build compiles all three regardless of flags. Each flag becomes
  `llm-<provider> = ["paladin-llm/<provider>"]`; `llm-all`/`full` extended. **A default build stops
  silently including Anthropic and DeepSeek** — CHANGELOG `BREAKING` entry required.
- **D-12:** **Provider names and API-key env vars follow each vendor's own convention:**
  `"kimi"`/`MOONSHOT_API_KEY`, `"qwen"`/`DASHSCOPE_API_KEY`, `"grok"`/`XAI_API_KEY`,
  `"gemini"`/`GEMINI_API_KEY`, `"ollama"`/none required. Matches existing pattern
  (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `DEEPSEEK_API_KEY`). Paladin-namespaced vars rejected.

**PROV-04 — model currency and testing:**
- **D-13:** **`get_available_models()` queries the provider's live endpoint with a curated
  fallback.** All five expose a models endpoint (Ollama's is local). On failure/offline, fall back
  to a curated list; `validate_model` returns true for anything the live list contains. The
  existing pattern (DeepSeek's hardcoded `vec!` at `:793`) is wrong for Kimi/Qwen and actively
  wrong for Ollama. **The shipped three keep their hardcoded/live-no-fallback lists** — D-06
  leaves them alone.
- **D-14:** **The model list is fetched lazily and memoized for the adapter's lifetime.** One call
  per adapter process lifetime. Accepted consequence: an operator who pulls a new Ollama model
  must restart to see it. TTL cache and per-provider split were considered and rejected.
- **D-15:** **Mock transport in Tier 1 for all six; Ollama additionally Docker-gated in Tier 2.**
  Every provider gets mock-transport unit tests (request shaping, response parsing, streaming
  chunk assembly, error mapping). Ollama additionally gets a Docker-gated suite with a small model
  — the only end-to-end exercise of the shared compatible core against a real protocol
  implementation that costs no credentials. Live key-gated tests for the hosted four:
  **considered, not added** — `live-api-tests` flag remains available.

### Claude's Discretion

- **Module layout** — `compat/` shared module vs. per-provider top-level directories mirroring
  `openai/`/`anthropic/`. Both satisfy the decisions above.
- **Where the core lives** — inside `paladin-llm` (working assumption) rather than a new crate.
  ADR-0035's leaf-crate precedent concerns a TensorFlow/ML adapter and does not bind LLM providers.
- **Retry and streaming parity** — whether the core's retry behaviour (modelled on
  `call_api_with_retry`) applies unchanged to a local Ollama endpoint where retry means less.
- **Config surface shape** — whether `config.yml`/`LlmConfig` gains a per-provider block (current
  shape: hardcoded `openai`/`deepseek`/`anthropic` fields) or a provider list/map; constrained only
  by PROV-03's "existing config files keep loading."
- **How PROV-01's study is recorded** — ADR vs. requirement-row verdicts. D-00g: contested parts
  (D-03's field-rejection, D-11's breaking change) are ADR-shaped; the rest is ledger material.

### Deferred Ideas (OUT OF SCOPE)

- **Gemini vision adapter** (`gemini/vision.rs`) — no `LlmPort` method exists for vision; trigger:
  a Gemini multimodal use case or any phase reopening the vision surface.
- **Migrating `openai/`/`deepseek/` onto the shared compatible core** — trigger: the next phase
  with reason to touch those two adapters anyway.
- **Groq, Together, Mistral, Fireworks, Bedrock as named presets** — rejected, not deferred; the
  generic provider already reaches them. Trigger: a specific request for a preset's curated
  capabilities/model list, not merely for access.
- **Widening `get_provider_name()` to `&str`** — breaking public-port change. Trigger: any phase
  already making a breaking `LlmPort` change.
- **Live key-gated tests for Kimi/Qwen/Grok/Gemini** — trigger: acquiring the four API keys as
  project secrets.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROV-01 | Additional-provider field narrowed to a build/defer/reject decision against written criteria; Kimi/Gemini/Qwen/Meta-Llama each explicitly dispositioned. | See `## Wire-Level Facts` and `## Package Legitimacy Audit` (no new heavyweight dependency needed — confirms one of the criteria); D-01/D-02 already supply the verdicts, this research supplies the wire-level evidence each verdict rests on. |
| PROV-02 | Every `build`-marked provider implements the full `LlmPort` contract truthfully (no stubs, no optimistic capabilities). | `## Architecture Patterns` documents the exact trait surface (`crates/paladin-ports/src/output/llm_port.rs`) and the truthful-capability precedent (`ProviderCapabilities`, the `capability_invariants` test module in `lib.rs`). `## Common Pitfalls` documents the two known-wrong patterns (unbounded `get_available_models`, capability over-reporting) so new adapters don't reproduce them. |
| PROV-03 | Feature-gated, additive; `--no-default-features --features <provider>` builds; `provider_factory` resolves new providers like the existing three; config surface stays backward compatible. | `## Architecture Patterns` (registry-driven factory), `## Code Examples` (current `Cargo.toml` feature graph, current `LlmConfig` shape) document exactly what D-10/D-11 must change and what must stay compatible. |
| PROV-04 | Mock-transport tests, 82% coverage floor holds, rustdoc on all public items, `Cargo.toml`/README/config docs name the shipped providers exactly. | `## Validation Architecture` and `## Testing approach` document the existing `mockito`-based pattern (`tests/integration/provider_switching_test.rs`) that mock-transport tests should follow, plus the exact `Cargo.toml` description/keywords/README lines PROV-04 requires be updated. |
</phase_requirements>

## Summary

Paladin's LLM adapter layer (`crates/paladin-llm/`) already contains a working template for
everything PROV-02 asks for: `deepseek/adapter.rs` (1,368 lines) is a complete, tested,
OpenAI-compatible adapter — request/response structs, SSE stream assembly with a `data: `/`[DONE]`
sentinel, retry-with-backoff, credential redaction, and status-code-to-`LlmError` mapping. Four of
the five build-list providers (Kimi, Qwen, Grok, Ollama) speak the identical wire protocol
`deepseek/adapter.rs` and `openai/adapter.rs` (741 lines) already speak twice independently — this
research corroborates that with live vendor documentation for all four: Moonshot AI's Kimi API is
literally described by its own docs as "migrating from OpenAI," xAI's Grok docs say the same, and
Alibaba's DashScope "compatible-mode" and Ollama's `/v1/*` OpenAI-compatibility layer are the
vendors' own accommodation of exactly this pattern. The fifth, Gemini, genuinely is not
OpenAI-shaped: it uses a resource-oriented URL (`/models/{model}:generateContent`), a
`contents[]`/`parts[]` request body, `x-goog-api-key` (or `?key=`) auth, and its own streaming
framing (`:streamGenerateContent?alt=sse`) — closer in shape to `anthropic/adapter.rs` (1,180
lines) than to the compatible four, which is exactly what D-08 already concluded.

No new heavyweight dependency is implied by any of the six new surfaces (five presets + generic
provider): every one of them is a plain JSON-over-HTTPS REST API reachable with the `reqwest` +
`serde_json` combination already in `paladin-llm`'s dependency graph. That satisfies one of
PROV-01's own study criteria directly and keeps `make deny`/`make audit` unaffected.

Two structural facts from the shipped tree bound how PROV-03 must be implemented: (1)
`provider_factory.rs`'s `get_default_provider()`/`list_available_providers()` are not `cfg`-gated
today (`:123-149`) — a **live defect** D-10's table-driven registry is designed to fix
structurally rather than patch; (2) the root `Cargo.toml`'s `llm-*` feature flags are empty stubs
while `paladin-llm` is pulled in unconditionally with all three provider features enabled
(`Cargo.toml:55`) — D-11's fix is a genuine breaking change to what a default build contains, and
must ship with a CHANGELOG `BREAKING` entry.

**Primary recommendation:** Extract a `compat/` (or similarly named) module inside `paladin-llm`
that generalizes `deepseek/adapter.rs`'s request/response/streaming/retry/error-mapping machinery
into one engine parameterized by `(base_url, api_key, model, capabilities)`; give each of
Kimi/Qwen/Grok/Ollama a thin preset module that supplies those four things plus its own
env-var-driven `Config::from_env()`; give the generic provider (`openai-compatible` /
`OpenAiCompatibleAdapter`) the same engine with all four fields config-driven and
pessimistic-by-default; write Gemini as its own module modeled on `anthropic/adapter.rs`'s shape,
not on the compatible core; replace `provider_factory.rs`'s hardcoded match with the D-10
`cfg`-gated table; wire the root `Cargo.toml` `llm-*` flags for real per D-11.

## Architectural Responsibility Map

This is a pure Rust backend crate system (Hexagonal Architecture, no browser/SSR/CDN tiers), so
the generic tier taxonomy is mapped onto Paladin's own layers instead.

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Wire-protocol request/response translation (compat core) | Infrastructure Adapter (`paladin-llm`) | — | Provider-specific HTTP/JSON shaping is exactly what an adapter owns; `paladin-ports`/`LlmPort` must stay provider-agnostic. |
| Streaming chunk assembly / SSE parsing | Infrastructure Adapter (`paladin-llm`) | — | Same reasoning; the `Stream<Item = Result<StreamingResponse, LlmError>>` return type is already provider-agnostic at the port. |
| Retry / backoff policy | Infrastructure Adapter (`paladin-llm`) | — | Retry-worthiness depends on provider-specific status codes (e.g. DeepSeek's 402); the port only sees the mapped `LlmError`. |
| Error mapping into `LlmError` | Infrastructure Adapter (`paladin-llm`) | Application Port (`paladin-ports`, error taxonomy) | The taxonomy itself (`LlmError` variants) is an application-port concern already fixed; mapping *into* it is the adapter's job. |
| Provider selection / construction | Infrastructure Adapter (`provider_factory.rs`) | Facade build config (root `Cargo.toml` feature graph) | The factory is inside `paladin-llm`, but *which providers exist at all* is decided at compile time by the facade's `llm-*` flags — two tiers cooperate here (D-10 + D-11). |
| Capability truthfulness (`ProviderCapabilities`) | Application Port (`paladin-ports::llm_port`) | Infrastructure Adapter (implements it) | The struct and its contract (Phase 14's truthfulness standard) live at the port; each adapter is merely required to report accurately. |
| Config surface (env vars, `config.yml`) | Infrastructure Adapter (`paladin-llm/src/config/`) | — | `LlmConfig`/`LlmProviderConfig`/`bridge.rs` are all inside `paladin-llm`; no other tier is involved. |
| Model-list currency (live-fetch + fallback) | Infrastructure Adapter (`paladin-llm`) | — | Per-provider network call + memoization; no port-level type change required. |

## Wire-Level Facts

All entries below are `[CITED: <url>]` against vendor or vendor-adjacent-official documentation
found via WebSearch/WebFetch this session, not tool-verified against a live endpoint (no API keys
available in this environment — see `## Environment Availability`). Model-ID lists in particular
are fast-moving and are flagged `[ASSUMED: volatile]` in the Assumptions Log — this is exactly
what D-13's live-fetch-with-fallback is designed to tolerate.

### Kimi (Moonshot AI) — preset, OpenAI-compatible

| Field | Value |
|-------|-------|
| Base URL | `https://api.moonshot.ai/v1` (China accounts: `https://api.moonshot.cn/v1`) `[CITED: platform.moonshot.ai]` |
| Chat endpoint | `POST {base_url}/chat/completions` — identical path to `deepseek/adapter.rs:602` |
| Auth header | `Authorization: Bearer $MOONSHOT_API_KEY` — identical shape to DeepSeek/OpenAI |
| Env var (D-12) | `MOONSHOT_API_KEY` |
| Model IDs (volatile) | `moonshot-v1-8k`, `moonshot-v1-32k`, `moonshot-v1-128k`, `kimi-k2.5`, `kimi-k2.6`, `kimi-k3` (flagship as of mid-2026), `kimi-k2.7-code` `[ASSUMED: volatile — confirm via live `/models` at implementation time]` |
| Divergence from OpenAI schema | None documented beyond the standard OpenAI Chat Completions shape; docs explicitly market drop-in SDK compatibility (streaming, tool/function calling, JSON mode all supported) `[CITED: platform.moonshot.ai/docs/guide/migrating-from-openai-to-kimi]` |

### Qwen (Alibaba Cloud DashScope) — preset, OpenAI-compatible mode

| Field | Value |
|-------|-------|
| Base URL | Region-dependent — international `https://dashscope-intl.aliyuncs.com/compatible-mode/v1`; China (Beijing) `https://dashscope.aliyuncs.com/compatible-mode/v1`; also US/Hong Kong/Singapore/Tokyo variants exist `[CITED: alibabacloud.com/help/en/model-studio/compatibility-of-openai-with-dashscope]` |
| Chat endpoint | `POST {base_url}/chat/completions` |
| Auth header | `Authorization: Bearer $DASHSCOPE_API_KEY` (OpenAI-compatible mode uses the standard bearer form) `[CITED, region choice ASSUMED — see Open Questions]` |
| Env var (D-12) | `DASHSCOPE_API_KEY` |
| Model IDs (volatile) | `qwen3-max`, `qwen3-max-preview`, dated snapshots like `qwen-plus-2025-09-11`, `qwen-turbo`, `qwen-coder` (vision variant `qwen-vl` excluded — out of scope per D-08's vision boundary) `[ASSUMED: volatile]` |
| Divergence from OpenAI schema | Not identified beyond region selection; DashScope's compatible-mode is explicitly documented as an OpenAI-SDK drop-in |
| **Open item** | Which region is the adapter default? Not specified by CONTEXT.md. Recommend `dashscope-intl` as default with a `DASHSCOPE_BASE_URL` override env var, matching the existing `DEEPSEEK_BASE_URL`/`ANTHROPIC_BASE_URL` override pattern — see Open Questions. |

### Grok (xAI) — preset, OpenAI-compatible

| Field | Value |
|-------|-------|
| Base URL | `https://api.x.ai/v1` `[CITED: docs.x.ai]` |
| Chat endpoint | `POST {base_url}/chat/completions` |
| Auth header | `Authorization: Bearer $XAI_API_KEY` |
| Env var (D-12) | `XAI_API_KEY` |
| Model IDs (volatile) | `grok-4`, `grok-4-0709`, `grok-4.5`, `grok-3`, `grok-beta` (legacy) `[ASSUMED: volatile]` |
| Divergence from OpenAI schema | Docs describe the API as "OpenAI-compatible endpoint structure... change base_url and api_key" with the official OpenAI SDK — no divergence documented. |

### Ollama — preset, OpenAI-compatible (self-hosted)

| Field | Value |
|-------|-------|
| Base URL (default) | `http://localhost:11434/v1` `[CITED: docs.ollama.com/api/openai-compatibility]` |
| Chat endpoint | `POST {base_url}/chat/completions` (OpenAI-compat layer) — **note:** Ollama's *native* API is `/api/chat` (default port 11434, no `/v1` prefix) using NDJSON streaming by default; the compatible layer at `/v1/*` uses standard `data: ` SSE framing matching OpenAI. **Use the `/v1/*` compat layer for D-05's shared core, not the native API.** |
| Models endpoint | `GET {base_url}/v1/models` (OpenAI-compat) or native `GET {base_url_without_v1}/api/tags` |
| Auth header | "Required but ignored" per Ollama's own docs — an API key value must be present in the request but is never validated. Recommend sending a fixed placeholder string (e.g. `"ollama"`), matching Ollama's own documented example. Aligns exactly with D-12's "none required." |
| Env var (D-12) | none required for auth; recommend `OLLAMA_BASE_URL` (default `http://localhost:11434/v1`) and `OLLAMA_MODEL` following the existing override pattern |
| Unsupported OpenAI request fields (documented) | `/v1/chat/completions`: `tool_choice`, `logit_bias`, `user`, `n`, logprobs features not supported `[CITED: docs.ollama.com]` — none of these are used by Paladin's current `LlmRequest→*Request` translation (confirmed by reading `deepseek/adapter.rs:build_request`), so this divergence has **zero impact** on the shared core as designed. |
| Model IDs | Operator-pulled, not a fixed vendor catalog — this is exactly why D-13's live-fetch (`GET /v1/models`, backed by whatever the operator has pulled) is the only correct design for this provider; a curated fallback list should be small/generic (e.g. `["llama3", "qwen", "mistral"]`) purely as a degrade-gracefully placeholder, not an authoritative catalog. |

### Gemini — bespoke adapter (D-08)

| Field | Value |
|-------|-------|
| API version | `v1beta` (current as of this research) `[CITED: ai.google.dev/api]` |
| `generateContent` | `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent` |
| `streamGenerateContent` | `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent?alt=sse` — emits SSE `data: `-framed `GenerateContentResponse` JSON objects (not NDJSON; `?alt=sse` is required to get SSE framing rather than a raw JSON array) |
| List models | `GET https://generativelanguage.googleapis.com/v1beta/models` |
| Auth | `x-goog-api-key: $GEMINI_API_KEY` header (current docs example for `generateContent`) **or** `?key=$GEMINI_API_KEY` query parameter (current docs example for `ListModels`) — both are shown in live Google documentation; **prefer the header form** to avoid the key appearing in request logs/URLs, matching this codebase's existing credential-redaction posture (`redact_credentials` in `deepseek/adapter.rs`). |
| Env var (D-12) | `GEMINI_API_KEY` |
| Request shape | `{ "contents": [{"role": "user"|"model", "parts": [{"text": "..."}]}], "systemInstruction": {"parts": [{"text": "..."}]}, "generationConfig": {"temperature", "maxOutputTokens", "topP", "topK", "stopSequences", "candidateCount"} }` — **no `system` role inside `contents`**; system prompt is a top-level sibling field, unlike OpenAI/Anthropic's in-array system message. `PromptType::System` in Paladin's prompt model must map to `systemInstruction`, not to a `contents[]` entry. |
| Response shape | `{ "candidates": [{"content": {"role": "model", "parts": [{"text": "..."}]}, "finishReason": "STOP"|"MAX_TOKENS"|"SAFETY"|"OTHER"|"RECITATION"}], "usageMetadata": {"promptTokenCount", "candidatesTokenCount", "totalTokenCount"} }` |
| `finishReason` mapping | `STOP`→`FinishReason::Stop`, `MAX_TOKENS`→`FinishReason::Length`, `SAFETY`→`FinishReason::ContentFilter` (closest existing variant; Gemini's safety block is content-policy, matching `ContentFilter`'s semantics), `OTHER`/`RECITATION`→`FinishReason::Error(reason)` (no direct equivalent — do not silently coerce to `Stop`) |
| Error shape | `{ "error": {"code": int, "message": "string", "status": "INVALID_ARGUMENT"|"NOT_FOUND"|"PERMISSION_DENIED"|"RESOURCE_EXHAUSTED"|...} }` — this is Google's RPC-style status string, not an HTTP-code-only scheme; `map_error` should switch on both the HTTP status AND the JSON `status` field (HTTP 429 alone is ambiguous between rate-limit and quota-exhausted on Google APIs generally — confirm exact 429-vs-`RESOURCE_EXHAUSTED` behavior at implementation time; flagged in Open Questions). |
| Model IDs (volatile) | `gemini-2.5-flash`, `gemini-2.5-pro`, plus newer preview names observed (`gemini-3.1-flash-lite`, `gemini-3-flash-preview`, `gemini-3.1-pro-preview`) `[ASSUMED: volatile — this provider's catalog moves fastest of the five, reinforcing D-13's live-fetch requirement]` |
| No tool-calling in scope | Gemini's `tools`/`functionDeclarations` fields exist in the API but `LlmRequest` has no field to carry a tool definition (confirmed by the existing `capability_invariants` test in `lib.rs`) — omit `tools`/`toolConfig` from the request entirely; `get_capabilities().supports_tool_calling` must be `false`, consistent with all shipped adapters. |

## Package Legitimacy Audit

**No new package dependencies are required for this phase.** All six new surfaces (Kimi, Qwen,
Grok, Ollama, generic OpenAI-compatible, Gemini) are plain JSON-over-HTTPS REST APIs reachable
with `paladin-llm`'s existing dependency set: `reqwest` (with `json`/`stream` features, already
present), `serde`/`serde_json` (already present), `tokio`/`futures` (already present). This
directly satisfies one of PROV-01's own study criteria ("whether `reqwest` suffices or a new
heavyweight dependency is implied — a new dependency is a cost, and `make deny`/`make audit` must
still pass").

| Package | Registry | Disposition |
|---------|----------|-------------|
| *(none proposed)* | — | No `package-legitimacy check` run — nothing to check. If a future implementation task discovers a need for a Gemini-specific SDK crate (not recommended — plain REST is simpler and matches every other adapter's pattern), run the Package Legitimacy Gate protocol on it before adding it to `Cargo.toml`. |

**Packages removed due to `[SLOP]` verdict:** none.
**Packages flagged as suspicious `[SUS]`:** none.

## Architecture Patterns

### System Architecture Diagram

```
                    LlmRequest (provider-agnostic, from paladin-ports)
                              │
                              ▼
                  ┌────────────────────────┐
                  │  LlmProviderFactory     │  provider_factory.rs
                  │  (D-10: cfg-gated       │  registry table, one row per
                  │   registry table)       │  provider — replaces hardcoded match
                  └───────────┬─────────────┘
                              │  .create("kimi" | "qwen" | "grok" |
                              │           "ollama" | "openai-compatible" | "gemini")
                              ▼
        ┌─────────────────────────────────────────────────────────┐
        │                 dyn LlmPort (unchanged trait)             │
        └─────────────────────────────────────────────────────────┘
              │                    │                    │
   ┌──────────┴─────────┐   ┌──────┴───────┐    ┌───────┴────────┐
   │  OpenAI-compatible   │   │  Gemini      │    │  (existing,     │
   │  shared core (D-05)  │   │  bespoke     │    │   untouched)    │
   │  request build +     │   │  adapter     │    │  openai/        │
   │  SSE assembly +      │   │  (D-08,      │    │  anthropic/     │
   │  retry + error map   │   │  modeled on  │    │  deepseek/      │
   └──────────┬───────────┘   │  anthropic/) │    └─────────────────┘
              │                └──────────────┘
   ┌──────────┼──────────┬──────────┬───────────────┐
   ▼          ▼          ▼          ▼                ▼
 Kimi       Qwen        Grok      Ollama      OpenAiCompatibleAdapter
 preset     preset      preset    preset      (D-03/D-04/D-09, config-
 (base_url, (base_url,  (base_url,(base_url,   driven base_url/key/model/
  MOONSHOT_  DASHSCOPE_  XAI_      OLLAMA_      capabilities, all
  API_KEY)   API_KEY)    API_KEY) BASE_URL,     pessimistic-by-default)
                                   no key)
              │ each preset POSTs {base_url}/chat/completions
              ▼
   Vendor HTTP endpoint → JSON response or SSE stream
              │
              ▼
   Shared core parses into LlmResponse / Stream<StreamingResponse>
   (identical shape whichever preset produced it — LlmPort's contract)
```

Gemini's own request/response translation is a parallel path that never touches the shared core —
it builds its own `contents[]`/`systemInstruction`/`generationConfig` body and parses its own
`candidates[]`/`usageMetadata` response, exactly as `anthropic/adapter.rs` does today for its own
bespoke `/messages` protocol.

### Recommended Project Structure

Per "Claude's Discretion — Module layout," either a `compat/` shared module or per-provider
top-level directories satisfy the decisions. Given `paladin-llm`'s existing convention (one
top-level directory per provider: `openai/`, `anthropic/`, `deepseek/`), the lower-friction choice
that avoids inventing a new organizing principle is:

```
crates/paladin-llm/src/
├── compat/                    # NEW — the shared OpenAI-compatible engine (D-05)
│   ├── mod.rs
│   ├── engine.rs              # request build, SSE stream assembly, retry, map_error
│   └── types.rs               # *Request/*Message/*Response/*StreamDelta wire structs
│                               #   (generalized from deepseek/adapter.rs's private structs)
├── kimi/
│   ├── mod.rs
│   └── adapter.rs              # KimiConfig::from_env() + KimiAdapter wrapping compat::engine
├── qwen/
│   ├── mod.rs
│   └── adapter.rs
├── grok/
│   ├── mod.rs
│   └── adapter.rs
├── ollama/
│   ├── mod.rs
│   └── adapter.rs
├── openai_compatible/          # NEW — the generic provider (D-03/D-04/D-09)
│   ├── mod.rs
│   └── adapter.rs               # OpenAiCompatibleAdapter, config-driven capabilities
├── gemini/
│   ├── mod.rs
│   └── adapter.rs               # bespoke, modeled on anthropic/adapter.rs — no compat:: dependency
├── openai/                      # UNCHANGED (D-06)
├── anthropic/                   # UNCHANGED (D-06)
├── deepseek/                    # UNCHANGED (D-06)
├── mock.rs                      # UNCHANGED
├── provider_factory.rs          # MODIFIED — D-10 registry table
├── config/
│   ├── llm.rs                   # MODIFIED — extend LlmConfig for new providers (Claude's discretion on shape)
│   └── bridge.rs                # MODIFIED — add `From<&LlmProviderConfig> for <New>Config` impls
└── lib.rs                       # MODIFIED — new `#[cfg(feature = "...")]` pub mod declarations
```

### Pattern 1: Shared OpenAI-compatible engine (D-05)

**What:** One generic engine that owns request construction, HTTP POST + SSE stream parsing,
retry-with-backoff, and status-code-to-`LlmError` mapping, parameterized by
`(base_url, api_key, model, capabilities)`. Each preset is a thin wrapper supplying those
parameters plus its own `Config::from_env()`.

**When to use:** For Kimi, Qwen, Grok, Ollama, and the generic `openai-compatible` provider — all
five speak the identical `POST {base_url}/chat/completions` protocol with `Authorization: Bearer`
auth and standard OpenAI SSE framing (`data: {...}` lines, terminal `data: [DONE]`).

**Concrete duplication the core must absorb** (measured directly against
`crates/paladin-llm/src/deepseek/adapter.rs`, the template):
- Request struct: `{model, messages[], temperature?, max_tokens?, top_p?, frequency_penalty?, presence_penalty?, stream}` (`:104-119`)
- Message struct with the `deserialize_null_as_empty_string` null-tolerance quirk (`:121-138`,
  `:275-280`) — **recommend generalizing this into the shared core**, since a reasoning-model
  provider (Kimi's `k2.5`+ line advertises reasoning capability similarly to DeepSeek's `-flash`)
  could hit the same null-content-on-truncation shape.
- Response struct + `Choice`/`Usage` (`:142-166`)
- Stream response struct + `StreamChoice`/`StreamDelta` (`:170-187`)
- `map_finish_reason` (`:478-487`) — `stop`/`length`/`content_filter`/`function_call` string
  mapping, identical across all OpenAI-compatible providers
- `call_api_with_retry` (`:546-595`) — the retryable-error-set logic (retry
  `NetworkError|Timeout|ProcessingError|RateLimitExceeded|ModelNotAvailable|TokenLimitExceeded`;
  never retry `AuthenticationError|InvalidPrompt|EmptyCompletion|UsageLimitExceeded`)
- `detect_empty_completion` (`:204-213`) — the reasoning-model-truncation detector; **worth
  keeping in the shared core** since any of the four new providers could ship a reasoning variant
- Credential redaction (`redact_credentials`, `bounded_excerpt`, `:250-356`) — this is a security
  property (never echo a credential into a log line) that every preset needs identically
- `generate()`/`generate_stream()` bodies (`:598-786`) — `{base_url}/chat/completions` POST,
  status check, body-read-then-parse-separately pattern (deliberately NOT using `Response::json()`
  — see Pitfall 3 below)

**What each preset supplies (NOT absorbed into the core):**
- `base_url` default + env-var override name (e.g. `MOONSHOT_BASE_URL`)
- `api_key` env-var name (D-12: `MOONSHOT_API_KEY`, `DASHSCOPE_API_KEY`, `XAI_API_KEY`, none for
  Ollama)
- Default model string
- `get_available_models()` fallback list (curated, per D-13)
- `get_capabilities()` — provider-specific `max_context_tokens`, `temperature_range`,
  `supports_vision`/`supports_embeddings` (all currently `false` for these five per the shipped
  three's pattern — Qwen has a vision model family `qwen-vl` but that is out of scope per D-08's
  vision boundary, so the *text* Qwen adapter reports `supports_vision: false`)
- `get_provider_name()` literal
- Any documented divergence from the OpenAI schema specific to that vendor (Ollama's unsupported
  request fields — confirmed to have zero impact since Paladin never sends them; see Wire-Level
  Facts above)

### Pattern 2: Generic operator-configured provider (D-03/D-04/D-09)

**What:** `OpenAiCompatibleAdapter` — the same `compat::engine` as Pattern 1, but every one of
`base_url`, `api_key`, `model`, and `capabilities` comes from operator-supplied configuration
rather than a vendor-specific default. `get_provider_name()` returns the fixed literal
`"openai-compatible"` regardless of what endpoint it's pointed at (D-07's accepted cost).

**When to use:** Any OpenAI-compatible endpoint not on the named-preset list (Groq, Together,
Mistral, Fireworks, Bedrock's OpenAI-compat mode, a self-hosted vLLM/LiteLLM gateway, etc.) — this
is what D-03 uses to declare those five "already covered" rather than deferred.

**Example (capabilities defaulting, from D-04):**
```rust
// Illustrative — capabilities struct with pessimistic Default per D-04.
// Source: this research, generalized from ProviderCapabilities::default()
// at crates/paladin-ports/src/output/llm_port.rs:869-882 (the existing
// framework-wide pessimistic default already follows the same posture).
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiCompatibleCapabilitiesConfig {
    #[serde(default = "default_true")]
    pub supports_streaming: bool,       // D-04: true by default (in-spec)
    #[serde(default)]
    pub supports_tool_calling: bool,    // D-04: false by default
    #[serde(default)]
    pub supports_function_calling: bool,
    #[serde(default)]
    pub supports_vision: bool,
    #[serde(default)]
    pub supports_embeddings: bool,
    pub max_context_tokens: Option<u32>,   // D-04: None by default
    pub temperature_range: Option<(f32, f32)>, // D-04: None -> global [0.0, 1.0] applies
}
fn default_true() -> bool { true }
```

### Pattern 3: Gemini bespoke adapter (D-08)

**What:** A standalone module that does NOT depend on `compat::engine`, modeled structurally on
`anthropic/adapter.rs` (own request/response types, own streaming parse loop, own `map_error`).

**When to use:** Gemini only — no other build-list provider needs this pattern.

**Example (request/response shape, from live Google documentation, `[CITED: ai.google.dev/api/generate-content]`):**
```rust
// Illustrative shapes — not yet implemented in the tree. Source: Google's
// live Gemini API reference (generateContent), fetched this session.
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiSystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String, // "user" | "model" -- NOT "system"
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart { text: String }

#[derive(Serialize)]
struct GeminiSystemInstruction { parts: Vec<GeminiPart> }

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxOutputTokens")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "topP")]
    top_p: Option<f32>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
    finish_reason: Option<String>, // "STOP" | "MAX_TOKENS" | "SAFETY" | "OTHER" | "RECITATION"
}

// URL: POST {base}/v1beta/models/{model}:generateContent
// Streaming: POST {base}/v1beta/models/{model}:streamGenerateContent?alt=sse
// Auth: header "x-goog-api-key: {api_key}" (preferred over ?key= query param
//       to avoid the credential appearing in request logs/URLs)
```

### Pattern 4: Registry-driven factory (D-10)

**What:** Replace `provider_factory.rs`'s hardcoded `match` (currently 3 arms,
`provider_factory.rs:63-116`) and the two un-`cfg`-gated helper functions
(`get_default_provider`/`list_available_providers`, `:123-149`) with one `cfg`-gated static table
that all four call sites (create, default, list, error message) derive from.

**Example (illustrative registry shape):**
```rust
// Illustrative — the concrete constructor closures differ per feature-gate,
// but the shape generalizes provider_factory.rs's current match arms.
struct ProviderRegistration {
    name: &'static str,
    env_var: &'static str, // "" for Ollama (D-12: none required)
    #[cfg(feature = "kimi")]
    construct: fn() -> Result<Arc<dyn LlmPort>, ProviderFactoryError>,
}
// A `cfg`-gated const slice (or a macro emitting one row per feature) replaces
// the match; get_default_provider/list_available_providers iterate the SAME
// slice, so a compiled-out provider is structurally absent from both --
// fixing the D-10 defect without a second code path to keep in sync.
```

### Anti-Patterns to Avoid

- **Un-memoized live model-list fetch:** `openai/adapter.rs:596-636`'s `get_available_models()`
  calls the live `/models` endpoint on every invocation, with no caching and no fallback on
  failure — `validate_model()` calls it, so a transient network blip makes `validate_model` fail
  even for a model both parties already agree exists. D-13/D-14 explicitly correct this for the
  new adapters (fallback + memoize); do not replicate the OpenAI adapter's existing pattern.
- **Migrating the shipped three onto the new core:** explicitly rejected by D-06. Even if the core
  is a clean generalization of `deepseek/adapter.rs`, touching `openai/`, `anthropic/`, or
  `deepseek/` in this phase widens blast radius past PROV-03's additive requirement.
- **Sending `tools`/`toolConfig` to Gemini:** the vendor API supports it, but `LlmRequest` has no
  field to carry a tool definition (confirmed by the `capability_invariants` test in `lib.rs`).
  Omit it entirely rather than sending an empty/default value that could be misread as a
  capability signal.
- **Reporting a capability nobody asserted (generic provider):** D-04's pessimistic defaults exist
  specifically because Phase 14 already paid to fix one capability flag that over-reported — see
  `## Common Pitfalls` below.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SSE parsing for the 4 compatible presets + generic provider | A bespoke chunked-reader per preset | The generalized `compat::engine` streaming loop (lines-split on `bytes_stream()`, `data: ` prefix strip, `[DONE]` sentinel — already proven in `deepseek/adapter.rs:736-785`) | 5 independent hand-rolled SSE parsers is exactly the ~5,000-line duplication D-05 exists to prevent. |
| Retry/backoff policy | A new retry loop per preset | `compat::engine`'s generalized `call_api_with_retry`, itself generalized from `deepseek/adapter.rs:546-595` | The retryable-error-SET must stay in lockstep across every adapter that shares it — a per-preset copy reintroduces exactly the bug class `call_api_with_retry`'s own doc comment describes fixing between DeepSeek and Anthropic. |
| Credential redaction in diagnostic logs | Per-preset regex/string scanning | The generalized `redact_credentials`/`bounded_excerpt`/`diagnostic_excerpt` trio from `deepseek/adapter.rs:250-356` | Security-sensitive, already implemented, already tested (`deepseek_adapter.rs` tests assert no credential leak even from a gateway echoing the request back). Re-deriving this per preset risks a preset that forgets the `Bearer `/`sk-`-prefix defense-in-depth passes. |
| Provider-registry lookup logic | A second `match` or a `HashMap` built ad hoc in each call site | D-10's single `cfg`-gated table, referenced by all four `provider_factory.rs` call sites | This is the exact defect being fixed (un-gated helper functions reporting compiled-out providers as available) — a second lookup mechanism reintroduces the same class of bug. |

**Key insight:** every "don't hand-roll" item above already has a working, tested implementation
in this codebase (`deepseek/adapter.rs`) — the task is generalization/extraction, not invention.

## Common Pitfalls

### Pitfall 1: Un-`cfg`-gated availability reporting (the exact defect D-10 fixes)
**What goes wrong:** `provider_factory.rs:123-149`'s `get_default_provider()` and
`list_available_providers()` check only `std::env::var(...).is_ok()` — with no `#[cfg(feature =
...)]` guard. A binary built with `--no-default-features --features llm-openai` (Anthropic
compiled out) but with `ANTHROPIC_API_KEY` still set in the environment will report `"anthropic"`
as available, then fail at `create("anthropic")` time with a confusing "unknown provider" error
(the `create()` match arm itself IS `cfg`-gated) rather than a clean compile-time absence.
**Why it happens:** The three call sites (`create`, `get_default_provider`,
`list_available_providers`) were hand-maintained independently as the shipped three were added;
nobody added a `#[cfg]` to the latter two when the former's match arms got theirs.
**How to avoid:** D-10's single table, iterated by all three functions, makes this class of bug
structurally impossible — a compiled-out provider is absent from the table, full stop.
**Warning signs:** Any new helper function added to `provider_factory.rs` that inspects
`std::env::var` directly instead of walking the registry table has reintroduced this bug.

### Pitfall 2: Facade feature flags that don't actually gate anything (the exact defect D-11 fixes)
**What goes wrong:** Root `Cargo.toml:268-270` declares `llm-openai = []`, `llm-anthropic = []`,
`llm-deepseek = []` as empty feature stubs, while line 55 unconditionally requests `features =
["openai", "anthropic", "deepseek", "mock", "vision"]` on the `paladin-llm` path dependency. A
consumer who builds with `--no-default-features` still gets all three providers compiled in —
PROJECT.md's documented "unavailable adapters must fail at compile time, never at runtime" is not
what ships today.
**Why it happens:** The flags were likely added as forward-looking placeholders before the
dependency wiring caught up, and nobody circled back.
**How to avoid:** D-11's `llm-<provider> = ["paladin-llm/<provider>"]` forwarding, applied
uniformly to the three existing AND five new provider flags, with the unconditional feature list
at line 55 removed/made conditional. **This is a breaking default-build change — document it in
CHANGELOG as `BREAKING`, not silently.**
**Warning signs:** `cargo tree -p paladin-ai --no-default-features -e features` still showing
`openai`/`anthropic`/`deepseek` on the `paladin-llm` edge after the fix would mean line 55 wasn't
actually changed.

### Pitfall 3: `Response::json()` collapses two distinct failure modes into one indistinguishable error
**What goes wrong:** `reqwest`'s `Response::json()` maps BOTH a body-read failure (e.g. a timeout
mid-stream) and a serde deserialization failure to the same `Kind::Decode`, whose `Display` is the
constant, contextless string `"error decoding response body"`. This collapses two very different
operator actions (raise timeout vs. investigate schema drift) into one undiagnosable message — a
documented, previously-live incident in this codebase (see `deepseek/adapter.rs`'s
`LIVE_BODY_DECODE_ERROR` regression-guard test and its surrounding comments).
**Why it happens:** `reqwest` intentionally simplifies this at its own API boundary.
**How to avoid:** The shared core must replicate `deepseek/adapter.rs:632-666`'s pattern — read
the body to `String` via `.text()` first (which distinguishes timeout from other network errors
in its own `Err` branch), THEN separately `serde_json::from_str` it (whose error carries the
actual field-mismatch detail) — for every preset, not just the four DeepSeek/Anthropic already
have it.
**Warning signs:** An error message reading exactly `"Failed to parse ... response: error decoding
response body"` with no further detail is this bug recurring.

### Pitfall 4: Capability over-reporting (Phase 14's defect class, WEB-03/D-14)
**What goes wrong:** An adapter declares `supports_tool_calling: true` (or streaming, or vision)
when its own `generate()` never actually exercises that path. Phase 14 already had to correct this
once across the shipped three (the `capability_invariants` test in `lib.rs` now pins the
correspondence permanently for those three).
**Why it happens:** Copying capability metadata from vendor marketing pages instead of checking
what the adapter's own request/response translation actually implements.
**How to avoid:** Every new adapter's `get_capabilities()` must describe what *this adapter's
`generate()`* does, not what the vendor's API offers in the abstract. Since `LlmRequest` has no
tools field today (confirmed structurally, not just by convention), `supports_tool_calling` and
`supports_function_calling` must be `false` on all six new surfaces — no exceptions, regardless of
how capable the underlying vendor API is. **Recommend extending `lib.rs`'s
`capability_invariants` test module (or adding a sibling gated on the new feature combination) so
this correspondence is enforced by a test for the new adapters too, not just asserted in prose.**
**Warning signs:** Any `get_capabilities()` returning `supports_tool_calling: true` or
`supports_function_calling: true` anywhere in the new code.

### Pitfall 5: The generic provider silently claiming a capability nobody set
**What goes wrong:** If `OpenAiCompatibleCapabilitiesConfig`'s deserialization uses
`#[serde(default)]` inconsistently (e.g. defaulting `supports_tool_calling` to `true` because
"most modern OpenAI-compatible gateways support it"), an operator who points the generic adapter
at an unknown/minimal gateway gets a silently wrong capability report — exactly the defect class
D-04 exists to prevent, and exactly the kind of thing that's easy to get backwards when writing
`#[serde(default = "...")]` functions under time pressure.
**Why it happens:** "Reasonable-sounding" defaults feel safer to the implementer than `false`,
but D-04 is explicit: unset must mean the *conservative* answer, and streaming is the only
capability defaulting `true` (because it's baseline OpenAI-compat spec behavior, not an add-on).
**How to avoid:** Write the pessimistic-default test FIRST (Red step): construct the config with
every capability field omitted, assert every non-streaming flag is `false` and
`temperature_range`/`max_context_tokens` are `None`.
**Warning signs:** Any default function other than the streaming one returning `true`.

### Pitfall 6: Region ambiguity for DashScope/Qwen has no locked default
**What goes wrong:** DashScope's OpenAI-compatible mode has at least 5 documented regional base
URLs (intl, China, US, Hong Kong, Singapore/Tokyo via workspace-scoped domains). CONTEXT.md's D-12
fixes the env var name (`DASHSCOPE_API_KEY`) but not which region is the compiled-in default.
**Why it happens:** Not surfaced during the discuss-phase session; genuinely ambiguous without a
stated target audience/region.
**How to avoid:** Default to `dashscope-intl.aliyuncs.com/compatible-mode/v1` (the international
endpoint, most broadly reachable) with a `DASHSCOPE_BASE_URL` override, following the identical
override pattern every other preset already uses (`DEEPSEEK_BASE_URL`, `ANTHROPIC_BASE_URL`). This
is a recommendation, not a locked decision — flagged in Open Questions for confirmation.
**Warning signs:** A default region silently baked in without an override env var, or without a
recorded rationale.

## Code Examples

### Existing retry-set pattern (to generalize into `compat::engine`)
```rust
// Source: crates/paladin-llm/src/deepseek/adapter.rs:546-595 (verified in this
// session by direct file read). The retryable-error-SET is documented as
// deliberately identical between deepseek/adapter.rs and anthropic/adapter.rs's
// own execute_with_retry -- any new preset/generic-provider retry policy that
// reuses this engine inherits the same set, satisfying "Retry and streaming
// parity" (Claude's Discretion) without a new decision needed.
async fn call_api_with_retry<F, Fut, T>(&self, operation: F, max_retries: u32) -> Result<T, LlmError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, LlmError>>,
{
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if matches!(e, LlmError::AuthenticationError(_) | LlmError::InvalidPrompt(_)
                    | LlmError::EmptyCompletion(_) | LlmError::UsageLimitExceeded { .. }) {
                    return Err(e); // non-retryable classes
                }
                if attempt >= max_retries { return Err(e); }
                // exponential backoff + jitter, then retry
            }
        }
    }
    unreachable!()
}
```

### Existing SSE stream assembly pattern (to generalize)
```rust
// Source: crates/paladin-llm/src/deepseek/adapter.rs:736-785 (verified this
// session). This is the shape all four compatible presets + generic provider
// share: split response.bytes_stream() on lines, strip "data: " prefix,
// terminal sentinel is the literal string "[DONE]" (not a JSON object).
let llm_stream = stream.map(|chunk_result| match chunk_result {
    Ok(bytes) => {
        let text = String::from_utf8_lossy(&bytes);
        for line in text.lines() {
            if let Some(json_str) = line.strip_prefix("data: ") {
                if json_str.trim() == "[DONE]" {
                    return Ok(StreamingResponse { /* finish_reason: Some(Stop) */ });
                }
                // parse json_str into the provider's *StreamResponse shape
            }
        }
        Ok(StreamingResponse { delta: String::new(), finish_reason: None, .. })
    }
    Err(e) => Err(LlmError::NetworkError(format!("Stream error: {}", e))),
});
```

### Existing mock-transport test pattern (workspace-level, `mockito`-based)
```rust
// Source: tests/integration/provider_switching_test.rs:59-88 (verified this
// session). This is the pattern D-15's "mock-transport unit tests" should
// follow for each new preset -- point the adapter's base_url config field at
// a local mockito::Server, define the expected POST + response body, assert
// on the parsed LlmResponse/StreamingResponse. Fully offline, no API key.
// NOTE: `mockito` is a root-workspace dev-dependency (Cargo.toml:143) used at
// the tests/integration/ level, NOT currently a paladin-llm crate-local
// dev-dependency -- paladin-llm's own #[cfg(test)] modules today are
// hand-rolled unit tests of pure functions (map_error, detect_empty_completion,
// retry semantics) with no HTTP mocking library at all. The planner must
// decide (or the executor must confirm) whether request/response-shape tests
// for the new presets live at the tests/integration/ level (reusing the
// existing mockito root dev-dependency, consistent with
// provider_switching_test.rs and openai_embedding_tests.rs) or whether
// `paladin-llm/Cargo.toml` gains its own mockito dev-dependency so these
// tests can live in-crate alongside the adapter code -- see Open Questions.
let mut server = mockito::Server::new_async().await;
let _mock = server.mock("POST", "/chat/completions")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(serde_json::json!({ /* provider-shaped response */ }).to_string())
    .create_async().await;
let config = KimiConfig { api_key: "test-key".into(), base_url: server.url(), .. };
```

### Current `provider_factory.rs` match (what D-10 replaces)
```rust
// Source: crates/paladin-llm/src/provider_factory.rs:62-117 (verified this
// session). 3 hardcoded match arms today; D-10 replaces this with a
// cfg-gated registry table that create()/get_default_provider()/
// list_available_providers() all derive from.
pub fn create(&self, provider_name: &str) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    match provider_name.to_lowercase().as_str() {
        #[cfg(feature = "openai")]
        "openai" => { /* OpenAIConfig::from_env() + OpenAIAdapter::new(..) */ }
        #[cfg(feature = "deepseek")]
        "deepseek" => { /* ... */ }
        #[cfg(feature = "anthropic")]
        "anthropic" => { /* ... */ }
        other => Err(ProviderFactoryError::UnknownProvider(other.to_string())),
    }
}
// get_default_provider() and list_available_providers() below this — at
// :123-149 — check std::env::var(...).is_ok() with NO #[cfg] guard at all.
```

### Current `LlmConfig` shape (what PROV-03's config-surface change must extend)
```rust
// Source: crates/paladin-llm/src/config/llm.rs:1-116 (verified this session).
// Config is a struct with one named Option<LlmProviderConfig> field PER
// PROVIDER (openai, deepseek, anthropic), and validate()/get_provider_config()
// both hardcode match arms over exactly those three names. Extending this to
// 9 providers (3 existing + 5 new + generic) either means 6 more named
// fields (least-surprising, matches the existing convention, but repetitive)
// or restructuring into a HashMap<String, LlmProviderConfig>/Vec (Claude's
// Discretion per CONTEXT.md — "constrained only by PROV-03's requirement
// that existing config files keep loading", i.e. any restructuring must
// still deserialize existing openai/deepseek/anthropic-keyed YAML unchanged).
pub struct LlmConfig {
    pub default_provider: Option<String>,
    pub openai: Option<LlmProviderConfig>,
    pub deepseek: Option<LlmProviderConfig>,
    pub anthropic: Option<LlmProviderConfig>,
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Per-provider hand-rolled OpenAI-compatible adapter (openai/, deepseek/) | Shared compatible core + thin per-provider presets | This phase (D-05) | New OpenAI-compatible providers become ~1 preset module instead of ~700-1,400 lines each |
| Hardcoded `match` in `provider_factory.rs` | `cfg`-gated registry table | This phase (D-10) | Adding a provider becomes one table row instead of four hand-maintained call sites |
| Facade `llm-*` flags as inert stubs | Flags that actually forward into `paladin-llm`'s Cargo features | This phase (D-11, BREAKING) | Default build stops silently compiling in Anthropic/DeepSeek |
| `validate_model` against a hardcoded `vec!` (DeepSeek pattern) or an un-fallback-guarded live call (OpenAI pattern) | Live-fetch-with-curated-fallback, memoized once per adapter lifetime | This phase (D-13/D-14), for new adapters only | New adapters tolerate the provider adding models after Paladin's release; existing three unchanged (D-06) |

**Deprecated/outdated:** Nothing in the shipped tree is deprecated by this phase — it is purely
additive per D-06/D-03.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Kimi/Qwen/Grok/Gemini model-ID lists as researched (`moonshot-v1-*`, `kimi-k2.x`/`k3`, `qwen3-max`/`qwen-plus-*`, `grok-4*`, `gemini-2.5-*`/`gemini-3.x-*`) | Wire-Level Facts | These are fast-moving vendor catalogs researched via WebSearch, not verified against a live authenticated endpoint (no API keys in this environment). D-13's live-fetch-with-fallback design is specifically built to tolerate this drift — the curated fallback list only needs to be *plausible*, not perfectly current, since it's a degrade-gracefully path, not the primary source of truth. Risk is low-impact but non-zero: an implementation task should re-verify the default `model` value against the provider's current default at authoring time. |
| A2 | DashScope/Qwen auth uses the standard `Authorization: Bearer $DASHSCOPE_API_KEY` form in compatible-mode (not a custom header) | Wire-Level Facts (Qwen) | If DashScope's compatible-mode actually requires a different header shape, the shared `compat::engine`'s auth-header construction (copied from `deepseek/adapter.rs:378-385`) would fail authentication for Qwen specifically while working for the other three. Low risk — "compatible-mode" strongly implies OpenAI SDK drop-in including its auth header shape, per the vendor's own framing, but this was not confirmed against a live 401/200 response. |
| A3 | Default DashScope region (international vs. China vs. other) | Wire-Level Facts (Qwen), Pitfall 6 | No locked decision exists in CONTEXT.md for which region is the compiled-in default. Recommended default (`dashscope-intl`) is this research's own recommendation, not a user decision — wrong default region means the adapter fails outright for an operator in the "wrong" region until they set `DASHSCOPE_BASE_URL`. Low risk since an override env var is recommended regardless. |
| A4 | Gemini's `RESOURCE_EXHAUSTED` vs. plain rate-limit distinction maps cleanly onto `LlmError::RateLimitExceeded` vs. `LlmError::UsageLimitExceeded` the same way DeepSeek's HTTP 402 does | Wire-Level Facts (Gemini), `map_error` design | Not confirmed against a live Gemini error response. If Gemini conflates transient rate-limiting and hard quota exhaustion into the same `RESOURCE_EXHAUSTED` status with no further distinguishing field, the adapter may need a different heuristic (e.g. inspect `error.details[].reason`) than the HTTP-status-code switch DeepSeek uses. Flagged as an Open Question below rather than asserted as fact. |
| A5 | Ollama's OpenAI-compat `/v1/models` response shape matches OpenAI's `{data: [{id: ...}]}` exactly, so the existing `openai/adapter.rs:628-633` parsing pattern can be reused verbatim | Wire-Level Facts (Ollama) | Not independently confirmed against a running Ollama instance in this session (no Docker/Ollama available in this research environment — see Environment Availability). If the compat layer's `/v1/models` response omits a field the parser expects, `get_available_models()` would need a small provider-specific tweak. Low risk — Ollama's compat layer is explicitly documented as OpenAI-shape-matching for this endpoint. |

## Open Questions

1. **Where do mock-transport unit tests for the new presets physically live?**
   - What we know: `paladin-llm`'s own `#[cfg(test)]` modules (in `deepseek/adapter.rs` etc.) are
     hand-rolled unit tests of pure functions with zero HTTP mocking. The workspace-level
     `tests/integration/` directory already uses `mockito` (root dev-dependency) for exactly this
     kind of test (`provider_switching_test.rs`, `openai_embedding_tests.rs`).
   - What's unclear: D-15 requires "mock-transport unit tests for request shaping, response
     parsing, streaming chunk assembly, and error mapping" for all six new surfaces — CONTEXT.md
     doesn't specify crate-local vs. workspace-level placement, and this is not listed under
     Claude's Discretion either.
   - Recommendation: Add `mockito` as a `paladin-llm` crate-local dev-dependency (small addition,
     already vetted/used elsewhere in the workspace) so full request/response-shape tests can live
     directly beside each preset's adapter code, matching the "one directory per provider, tests
     in-file" convention `openai/`/`anthropic/`/`deepseek/` already follow — rather than splitting
     the six new providers' tests across crate-local pure-function tests AND workspace-level
     `mockito` tests. The planner should confirm this placement before task-writing.

2. **Exact Gemini `RESOURCE_EXHAUSTED` vs. rate-limit distinction (A4 above).**
   - What we know: Gemini's error envelope carries a `status` string (`RESOURCE_EXHAUSTED`,
     `INVALID_ARGUMENT`, etc.) alongside the HTTP code, unlike the compatible four's plain
     HTTP-status switch.
   - What's unclear: Whether `RESOURCE_EXHAUSTED` alone is enough to distinguish a transient
     per-minute rate limit (→ `LlmError::RateLimitExceeded`, retryable) from a hard billing-quota
     exhaustion (→ `LlmError::UsageLimitExceeded`, non-retryable) the way DeepSeek's HTTP 402 does
     unambiguously.
   - Recommendation: Map `RESOURCE_EXHAUSTED` conservatively to `RateLimitExceeded` (retryable) at
     first implementation, since retrying a true quota exhaustion merely wastes retries rather than
     causing harm (the existing retry loop still respects `max_retries`), and record this as a
     `[ASSUMED]` mapping in the adapter's own doc comments (following the DeepSeek 402 precedent's
     own documented uncertainty) pending live-key verification via the `live-api-tests` feature.

3. **Whether `capability_invariants` in `lib.rs` should be extended to cover the new adapters.**
   - What we know: The existing test (`crates/paladin-llm/src/lib.rs`, gated on
     `all(test, feature = "openai", feature = "anthropic", feature = "deepseek")`) permanently pins
     the tool-calling/function-calling capability-vs-request-surface correspondence for the shipped
     three, catching exactly the class of regression Pitfall 4 describes.
   - What's unclear: Not covered by CONTEXT.md's Decisions or Discretion sections at all — an
     omission, not a deliberate choice.
   - Recommendation: Add a sibling test module gated on the new features (or extend the existing
     one to also compile under the new feature combination) so the same invariant is enforced for
     Kimi/Qwen/Grok/Ollama/Gemini/generic, not merely asserted in each adapter's doc comments.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Docker | Ollama Tier-2 Docker-gated test service (D-15) | Not probed in this research session — this environment has no interactive Docker daemon access; verify at implementation/CI time via `docker info` | — | If unavailable in an authoring sandbox, the Ollama Docker-gated suite still runs correctly in CI (existing `docker-compose.test.yml` pattern already covers Redis/MinIO in exactly this shape) — author the tests against the pattern, defer live execution to CI. |
| `MOONSHOT_API_KEY`, `DASHSCOPE_API_KEY`, `XAI_API_KEY`, `GEMINI_API_KEY` | Live/`live-api-tests`-gated tests for the four hosted providers | Not available in this environment (no project secrets provisioned) | — | D-15 explicitly does NOT add live key-gated tests for these four this phase — the `live-api-tests` flag remains available but unused for new providers, consistent with the locked decision. No blocking impact. |
| `cargo`, `reqwest`/`serde_json` toolchain | All new adapter code | Available (existing workspace dependency graph; `crates/paladin-llm/Cargo.toml` already declares `reqwest`/`serde`/`serde_json`) | reqwest `0.12.4` (workspace-pinned) | — |
| Ollama binary/service itself | Local manual verification (not CI) | Not probed in this research session | — | Not required for authoring — the Docker-gated Tier 2 suite pulls the Ollama image in CI; a developer can optionally run `ollama serve` locally for manual smoke-testing. |

**Missing dependencies with no fallback:** none — every gap above has a recorded fallback that
does not block the phase.

**Missing dependencies with fallback:** Docker (defer live execution to CI), the four hosted
providers' API keys (D-15 already excludes live tests for them this phase).

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | `cargo test` (workspace default) + `cargo llvm-cov` for coverage (`Makefile:257`) |
| Config file | none dedicated — coverage gate is inline in `Makefile:261` and `.github/workflows/ci.yml:664` (`cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82`) |
| Quick run command | `cargo test -p paladin-llm` (crate-scoped, offline, Tier 1) |
| Full suite command | `make coverage` (requires `make services-up` for Redis/MinIO — Ollama's Tier-2 suite would need the same pattern extended, or its own `docker-compose.test.yml` service block) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROV-01 | Provider-selection study recorded with build/defer/reject verdicts | documentation (ADR/ledger, not a test) | n/a — verified by human review of the recorded study, not automated | N/A |
| PROV-02 | Each built adapter implements full `LlmPort` contract; capabilities truthful | unit | `cargo test -p paladin-llm --features kimi,qwen,grok,ollama,gemini,openai-compatible` (feature names illustrative — confirm at planning time) | ❌ Wave 0 — no adapter code exists yet |
| PROV-02 | Capability/request-surface correspondence holds for new adapters (Open Question 3) | unit | new/extended `capability_invariants` test in `lib.rs`, gated on the new feature set | ❌ Wave 0 |
| PROV-03 | `cargo build --no-default-features --features <provider>` succeeds per new provider | build/smoke | `cargo build -p paladin-llm --no-default-features --features kimi` (× 5 providers + generic) | ❌ Wave 0 — feature flags don't exist yet |
| PROV-03 | `provider_factory` resolves new providers; compiled-out providers absent from `list_available_providers` | unit | extend `tests/unit/llm/provider_factory_test.rs` | ❌ Wave 0 — file exists (282 lines, verified this session) but needs new cases |
| PROV-03 | Existing config files keep loading (no breaking change to `LlmConfig`) | unit | extend `crates/paladin-llm/src/config/llm.rs`'s existing `#[cfg(test)]` module | ⚠️ File exists; needs new test cases, not a new file |
| PROV-04 | Mock-transport request shaping / response parsing / streaming assembly / error mapping per provider | unit/integration | see Open Question 1 (placement TBD) — either `crates/paladin-llm/src/<provider>/adapter.rs`'s own `#[cfg(test)]` module (needs `mockito` as a new crate-local dev-dependency) or `tests/integration/<provider>_provider_test.rs` (reusing root `mockito` dev-dependency) | ❌ Wave 0 either way |
| PROV-04 | Ollama Docker-gated Tier 2 suite (D-15) | integration | new `tests/integration/ollama_docker_test.rs`-style file, `required-features` gated, run via `make test-integration-docker` | ❌ Wave 0 — also needs a new `ollama-test` service block in `docker/docker-compose.test.yml` |
| PROV-04 | Workspace stays ≥82% line coverage with new code included | coverage gate | `make coverage` / CI's `coverage` job | ✅ Gate exists; just needs the new code to clear it |
| PROV-04 | Every public item carries rustdoc | lint | `cargo doc` combined with `#![warn(missing_docs)]` already set at `lib.rs:41` | ✅ Enforced by existing crate-level lint |

### Sampling Rate

- **Per task commit:** `cargo test -p paladin-llm` (offline, fast, Tier 1 — covers all mock-transport tests for whichever provider the task just touched)
- **Per wave merge:** `cargo build -p paladin-llm --no-default-features --features <each-provider>` for every provider touched in the wave, plus `cargo test --workspace` (default features)
- **Phase gate:** `make coverage` (Tier 1 + Tier 2, requires `make services-up` and, if the Ollama Docker service is added to `docker-compose.test.yml`, that service running too) — full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `crates/paladin-llm/src/compat/` (or equivalent) module and its own test module — covers PROV-02's shared-core contract, no file exists yet
- [ ] Per-preset adapter files (`kimi/`, `qwen/`, `grok/`, `ollama/`, `openai_compatible/`, `gemini/`) and their `#[cfg(test)]` modules — covers PROV-02
- [ ] `mockito` as a `paladin-llm` crate-local dev-dependency, IF Open Question 1 resolves toward crate-local placement — currently only a root-workspace dev-dependency
- [ ] `docker/docker-compose.test.yml` needs a new `ollama-test` service block (image `ollama/ollama`, small pulled model) — covers D-15's Ollama Tier 2 requirement; no existing block to extend
- [ ] `tests/integration/ollama_docker_test.rs` (or similarly named) — the Docker-gated Tier 2 suite itself
- [ ] Extended `tests/unit/llm/provider_factory_test.rs` (282 lines today, verified this session) — new cases for the D-10 registry table
- [ ] Extended `capability_invariants` test module in `lib.rs` — see Open Question 3
- Framework install: none — `cargo test`/`cargo llvm-cov` already fully set up workspace-wide

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes | API-key-in-header auth per vendor convention (`Authorization: Bearer`, or `x-goog-api-key` for Gemini) — never in query string except Gemini's documented `?key=` fallback, which should be avoided in favor of the header form specifically to keep credentials out of URLs/logs |
| V3 Session Management | no | Stateless per-request API-key auth; no session concept in this adapter layer |
| V4 Access Control | no | Not applicable at the adapter layer — access control is the caller's (Paladin's) responsibility, not the LLM provider's |
| V5 Input Validation | yes | `LlmRequest`/`LlmResponse` deserialization via `serde`; the `deserialize_null_as_empty_string` null-tolerance pattern (`deepseek/adapter.rs:275-280`) is the precedent for surviving vendor schema drift without panicking |
| V6 Cryptography | yes (delegated) | TLS is `reqwest`'s responsibility (never hand-rolled); credential redaction in diagnostic/log output (`redact_credentials`/`bounded_excerpt`, `deepseek/adapter.rs:250-356`) is the existing, tested pattern every new adapter must replicate — this is the actual crypto/secret-handling surface in this phase, not transport encryption |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Credential leakage via echoed request bodies in error diagnostics (a misconfigured gateway can echo `Authorization` headers back in a 4xx/5xx body) | Information Disclosure | `redact_credentials`/`diagnostic_excerpt` — three-pass redaction (own configured key exact-match, `Bearer `/`bearer ` prefix, `sk-`-prefixed tokens), MUST run before truncation to avoid slicing a secret in half. Already tested for the DeepSeek case (`diagnostic_excerpt_never_echoes_the_configured_api_key`); replicate identically for every new preset via the shared `compat::engine`. |
| A malicious/compromised generic-provider endpoint (D-03) returning capability-claiming metadata the operator never configured | Spoofing / Elevation of Privilege (of capability) | Capabilities for the generic provider are **entirely operator-config-driven** (D-04) — never derived from anything the remote endpoint itself reports at runtime. The adapter must not, for example, probe the endpoint and infer `supports_tool_calling` from a response header or a `/models` field. |
| Prompt/response body containing provider-injected content interpreted as instructions (e.g. Gemini's `functionCall` parts, if ever added later under the deferred tool-calling surface) | Tampering | Out of scope for this phase — `LlmRequest`/`LlmResponse` carry no tool-call path today (confirmed structurally), so this threat is currently unreachable through any shipped or newly-added adapter; the `capability_invariants` test is the mechanism that keeps it unreachable. |
| SSRF via operator-supplied `base_url` for the generic provider (D-03) pointing at an internal/metadata-service address | Tampering / Information Disclosure | Not addressed by any locked decision in CONTEXT.md. Recommend flagging to the planner as a task-level concern: the generic provider is explicitly "public API... operators write configuration against" (D-03's own reversibility note) — if Paladin ever runs in a multi-tenant context where `base_url` could come from an untrusted party rather than the deploying operator, an allowlist or URL-validation step would be warranted. For this phase (operator-configured, trusted config surface, single-tenant deployment model per PROJECT.md's existing posture) this is a documented residual risk, not a blocking gap. |

## Sources

### Primary (HIGH confidence — direct file reads, this session)
- `crates/paladin-ports/src/output/llm_port.rs` — full `LlmPort` trait, `ProviderCapabilities`, `LlmError` variant set
- `crates/paladin-llm/src/deepseek/adapter.rs` — the compatible-core template (1,368 lines, read in full)
- `crates/paladin-llm/src/openai/adapter.rs` — second compatible-protocol implementation, incl. the un-fallback-guarded live `/models` call (excerpted, `:590-660`)
- `crates/paladin-llm/src/anthropic/adapter.rs` — bespoke-protocol template Gemini should follow (excerpted for structure)
- `crates/paladin-llm/src/provider_factory.rs` — full file, the D-10 target
- `crates/paladin-llm/src/lib.rs` — full file, incl. the `capability_invariants` test module
- `crates/paladin-llm/src/config/llm.rs`, `crates/paladin-llm/src/config/bridge.rs` — full files, the config-surface target
- `crates/paladin-llm/Cargo.toml`, root `Cargo.toml` (feature-flag sections) — the D-11 target
- `tests/integration/provider_switching_test.rs`, `tests/integration/deepseek_provider_test.rs`, `tests/integration/openai_embedding_tests.rs` — existing test patterns (mockito-based mock-transport, and `#[ignore]`d live-key tests)
- `docker/docker-compose.test.yml`, `Makefile` (test/coverage targets) — existing three-tier test infrastructure
- `.planning/decisions/0031-extracted-crate-dependency-rule.md`, `0035-paladin-ml-leaf-crate-placement.md` — read in full for the crate-placement discretion question
- `.planning/decisions/PROMOTION.md` — next free ADR number: **0045**
- `.planning/REQUIREMENTS.md` §*v1 Requirements — Provider Expansion (Phase 17)* — PROV-01…04 full text

### Secondary (MEDIUM confidence — WebSearch/WebFetch against vendor-official or vendor-adjacent-official docs, this session)
- Moonshot AI Kimi: `platform.moonshot.ai/docs/guide/migrating-from-openai-to-kimi`, `platform.kimi.ai/docs/api/overview`
- Alibaba Cloud DashScope Qwen: `alibabacloud.com/help/en/model-studio/compatibility-of-openai-with-dashscope`, `alibabacloud.com/help/en/model-studio/qwen-api-via-openai-chat-completions`
- xAI Grok: `docs.x.ai/docs/guides/chat-completions`, `docs.x.ai/developers/model-capabilities/legacy/chat-completions`
- Ollama: `docs.ollama.com/api/openai-compatibility` (fetched in full via WebFetch)
- Google Gemini: `ai.google.dev/api/generate-content` (fetched in full via WebFetch), `ai.google.dev/api/models` (fetched via WebFetch), `ai.google.dev/api`

### Tertiary (LOW confidence — not used as a primary source; aggregator/blog results deliberately excluded from factual claims above, cited only where they corroborated an official source)
- None promoted to a claim in this document — all wire-level facts above trace to a vendor-official or vendor-adjacent-official (cloud-provider help center) URL.

## Metadata

**Confidence breakdown:**
- Standard stack (no new deps, `reqwest`/`serde_json` sufficient): HIGH — directly verified against `paladin-llm/Cargo.toml`'s existing dependency graph.
- Architecture (shared core + presets + registry factory): HIGH for the *shape* (directly derived from reading the existing `deepseek/adapter.rs`/`provider_factory.rs` shipped code); MEDIUM for the *exact new-module boundaries* (Claude's Discretion per CONTEXT.md — not yet a locked decision).
- Wire-level provider facts (base URLs, auth headers, request/response shapes): MEDIUM — CITED against vendor-official documentation via WebSearch/WebFetch this session, not tool-verified against a live authenticated endpoint (no API keys available in this environment).
- Model-ID catalogs specifically: LOW/`[ASSUMED: volatile]` — explicitly flagged as fast-moving; D-13's live-fetch-with-fallback design is built to tolerate this.
- Pitfalls: HIGH — every pitfall traces to a specific `file:line` read this session, not inferred.

**Research date:** 2026-08-16
**Valid until:** Wire-level provider facts (base URLs, auth shapes): ~30 days (stable vendor infrastructure, but two of the five vendors — xAI, Moonshot — are documented as shipping new model generations mid-2026, so re-verify model IDs at implementation time regardless of this date). Shipped-codebase facts: valid until the next commit touches `crates/paladin-llm/` or root `Cargo.toml`'s feature section.
