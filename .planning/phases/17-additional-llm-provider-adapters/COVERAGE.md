# API Coverage — Gemini Generative Language API + OpenAI-compatible Chat Completions

> Full coverage by default. Opt-outs are explicit, reasoned decisions.

**Detector result:** `detected: true` (signal `(surface)/api`, snippet `"Generative Language API
actually returns."`). Confirmed by re-reading the phase scope: **this phase genuinely integrates
external APIs.** It ships six new adapters — Kimi (Moonshot), Qwen (DashScope), Grok (xAI), Ollama,
Gemini, and an operator-configured generic OpenAI-compatible provider — across two distinct wire
protocols. A `No external API integration: …` declaration would be false here, so a real matrix
follows.

This is **not** the Phase 13 situation, where this same gate fired on the token `api-surface` (an
internally-named CI job) and was correctly overridden as a false positive. Here the trigger names
Google's real `generativelanguage.googleapis.com` endpoint.

**Surface enumerated from:** the shipped tree — `crates/paladin-llm/src/gemini/adapter.rs`,
`crates/paladin-llm/src/compat/{engine,types}.rs`, and the five presets under `kimi/`, `qwen/`,
`grok/`, `ollama/`, `openai_compatible/` — cross-checked against each vendor's documented protocol.
Decisions cite the phase's recorded rulings in `17-CONTEXT.md` (**D-01 … D-15**).

**Scope note.** The pre-existing `openai/`, `anthropic/` and `deepseek/` adapters are deliberately
out of scope: **D-06** ruled they are *not* migrated onto the shared compatible core in this phase.
This matrix covers the surfaces this phase newly integrates.

## Surfaces

- **`[port]`** — the `LlmPort` consumer contract. This bounds everything else: a vendor feature with
  no `LlmPort` method behind it is unreachable by design, not merely unimplemented.
- **`[gemini]`** — Google's Generative Language API `v1beta`. Bespoke, not OpenAI-compatible: the
  operation rides the URL as a `:generateContent` suffix rather than a path segment (**D-08**).
  Base `https://generativelanguage.googleapis.com/v1beta`, default model `gemini-2.5-flash`.
- **`[compat]`** — the shared OpenAI-compatible core (**D-05**), one engine with five thin presets:
  Kimi (`api.moonshot.ai/v1`, `moonshot-v1-8k`), Qwen
  (`dashscope-intl.aliyuncs.com/compatible-mode/v1`, `qwen-plus`), Grok (`api.x.ai/v1`, `grok-4`),
  Ollama (`localhost:11434/v1`, `llama3`), and the operator-configured generic provider (**D-03**).
- **`[ollama]`** — Ollama's *native* `/api/*` protocol, decided separately from the `/v1` shim.

Per the gate's re-decide rule, each `[compat]` capability is decided for this surface on its own
merits rather than inherited from the existing OpenAI adapter's choices.

## Matrix

| capability | decision | reason |
|---|---|---|
| `[port] generate` | INTEGRATE | Implemented by all six new adapters. |
| `[port] generate_stream` | INTEGRATE | Implemented by all six; every adapter reports `supports_streaming: true`. |
| `[port] validate_model` | INTEGRATE | Checked against the live model list (D-13). |
| `[port] get_available_models` | INTEGRATE | Live endpoint with curated fallback (D-13), lazily fetched and memoized for the adapter's lifetime (D-14). |
| `[port] get_provider_name` | INTEGRATE | Presets return their vendor name; the generic provider returns the fixed literal "openai-compatible" (D-07, D-09). |
| `[port] get_capabilities` | INTEGRATE | Each adapter reports what it implements, not what the vendor offers (D-08). |
| `[port] embeddings` | OPT-OUT | No LlmPort method exists for embeddings, so no adapter can expose one. A port change, out of scope for a provider phase. |
| `[port] tool / function calling` | OPT-OUT | LlmRequest carries no field a tool definition could travel through; all six report false (D-08, D-12). A port change, not an adapter change. |
| `[port] vision / multimodal` | OPT-OUT | A separate surface (openai/vision.rs, the vision feature) with no LlmPort method. D-08 defers a Gemini vision adapter with a recorded trigger. |
| `[gemini] POST /models/{model}:generateContent` | INTEGRATE | The core completion path. |
| `[gemini] POST /models/{model}:streamGenerateContent` | INTEGRATE | Backs generate_stream. |
| `[gemini] GET /models (list)` | INTEGRATE | Backs get_available_models and validate_model, with the D-13 curated fallback when the call fails or the process is offline. |
| `[gemini] request field contents` | INTEGRATE | Carries the conversation turns; assistant turns map to role "model". |
| `[gemini] request field systemInstruction` | INTEGRATE | System prompts go here, never into contents — enforced by a dedicated regression test. |
| `[gemini] request field generationConfig` | INTEGRATE | Carries temperature, maxOutputTokens and topP — the three tunables LlmRequest can express. |
| `[gemini] response usageMetadata` | INTEGRATE | Mapped into LlmResponse token usage. |
| `[gemini] GET /models/{model} (get one)` | OPT-OUT | Not needed — the list endpoint already returns every field validate_model and get_available_models consume. |
| `[gemini] POST :countTokens` | OPT-OUT | Not needed — no LlmPort method exposes pre-flight token counting; usage is reported post-hoc from usageMetadata. |
| `[gemini] POST :embedContent` | OPT-OUT | Not needed — no embeddings method on LlmPort; the adapter reports supports_embeddings false. |
| `[gemini] POST :batchEmbedContents` | OPT-OUT | Not needed — same reason as embedContent. |
| `[gemini] request fields tools / toolConfig` | OPT-OUT | Explicitly out of scope (D-08) — the adapter neither sends them nor parses a function-call part out of a response. |
| `[gemini] multimodal parts inlineData / fileData` | OPT-OUT | Explicitly out of scope — D-08 ships Gemini text-only and defers vision as a purely additive follow-up. |
| `[gemini] request field safetySettings` | OPT-OUT | Not needed yet — no configuration surface expresses per-request safety thresholds; vendor defaults apply. |
| `[gemini] context caching (cachedContents)` | OPT-OUT | Not needed yet — a cost optimisation for repeated large prefixes, with no consumer in this phase. |
| `[gemini] File API (/files CRUD)` | OPT-OUT | Not needed — only reachable through the multimodal path this phase opts out of. |
| `[gemini] tunedModels CRUD` | OPT-OUT | Explicitly out of scope — tuning is not a framework concern; an operator-tuned model ID still works through the normal completion path. |
| `[gemini] semantic retrieval (corpora, :generateAnswer)` | OPT-OUT | Explicitly out of scope — Paladin's retrieval story is the Garrison, not a vendor-hosted corpus. |
| `[gemini] POST :batchGenerateContent` | OPT-OUT | Not needed — LlmPort is a request/response and streaming interface with no asynchronous batch semantics. |
| `[compat] POST /chat/completions (non-streaming)` | INTEGRATE | The core completion path for all five presets. |
| `[compat] POST /chat/completions (SSE streaming)` | INTEGRATE | Backs generate_stream; incremental delta.content frames are parsed. |
| `[compat] GET /models` | INTEGRATE | Backs get_available_models and validate_model with the D-13 fallback. Ollama's is served locally. |
| `[compat] request field model` | INTEGRATE | Per-preset default, operator-overridable. |
| `[compat] request field messages (system/user/assistant)` | INTEGRATE | All five report supports_system_messages true. |
| `[compat] sampling fields (temperature, max_tokens, top_p, penalties)` | INTEGRATE | The five tunables the wire type carries; each is omitted when unset rather than sent as null. |
| `[compat] request field stream` | INTEGRATE | Selects the SSE path. |
| `[compat] response usage (prompt/completion/total tokens)` | INTEGRATE | Mapped into LlmResponse; total_tokens is tolerated as absent and derived. |
| `[compat] response finish_reason` | INTEGRATE | Mapped to FinishReason; an unknown value degrades to FinishReason::Error rather than panicking. |
| `[compat] response reasoning_content` | INTEGRATE | Parsed where the vendor emits it, so reasoning text is not silently dropped into the content field. |
| `[compat] Bearer-token auth header` | INTEGRATE | Set from the per-vendor API-key env var (D-12). Ollama tolerates a placeholder — local servers require no key. |
| `[compat] operator-declared capability flags (generic)` | INTEGRATE | D-04 — declared with pessimistic defaults, so an undeclared capability reads false rather than being assumed true. |
| `[compat] request fields tools / tool_choice` | OPT-OUT | Explicitly out of scope — no LlmPort surface carries a tool definition. The wire type omits the fields entirely. |
| `[compat] request field response_format (JSON mode)` | OPT-OUT | Not needed yet — no consumer requests schema-constrained output, and support is uneven across these five vendors. |
| `[compat] multimodal image_url content parts` | OPT-OUT | Explicitly out of scope — CompatMessage.content is a String, and vision is the separate surface D-08 defers. |
| `[compat] request fields n, stop, seed, logprobs, logit_bias, user` | OPT-OUT | Not needed — LlmRequest cannot express any of them; wire fields with no way to populate them would be dead surface. |
| `[compat] POST /embeddings` | OPT-OUT | Not needed — no embeddings method on LlmPort; all five report supports_embeddings false. |
| `[compat] POST /completions (legacy)` | OPT-OUT | Explicitly out of scope — deprecated by every vendor here in favour of chat completions. |
| `[compat] GET /models/{id} (retrieve one)` | OPT-OUT | Not needed — the list endpoint supplies everything validate_model checks. |
| `[compat] images, audio, files, batches, moderations, assistants` | OPT-OUT | Explicitly out of scope — OpenAI-platform surfaces with no LlmPort equivalent; coverage across Kimi, Qwen, Grok and Ollama is partial-to-absent. |
| `[ollama] GET /v1/models (compat shim)` | INTEGRATE | The integration path, so the operator's pulled catalog is visible (D-13). |
| `[ollama] POST /api/chat, POST /api/generate` | OPT-OUT | Not needed — the /v1 compat shim covers both through one shared engine (D-05). A second protocol would duplicate tested code. |
| `[ollama] GET /api/tags` | OPT-OUT | Not needed — the /v1/models shim returns the same catalog. |
| `[ollama] /api/pull, /api/push, /api/create, /api/delete` | OPT-OUT | Explicitly out of scope — model lifecycle is the operator's job. D-14 already accepts a newly pulled model needs a restart to appear. |
| `[ollama] POST /api/embeddings` | OPT-OUT | Not needed — no embeddings method on LlmPort. |
| `[ollama] GET /api/ps, POST /api/show` | OPT-OUT | Not needed — no consumer for local runtime introspection. |

## Verification status of this surface

Recorded for honesty, and consistent with `17-UAT.md` test 4: the base URLs, default model IDs and
live-fetch behaviour above are taken from **vendor documentation and the shipped code**, and have
**not** been confirmed against a live endpoint — the phase's sandbox has no network egress and no
vendor API keys. `README.md`, `config.example.yml` and
`docs/src/getting-started/configuration.md` all carry that caveat explicitly. An `INTEGRATE`
decision in this matrix therefore means *this phase implements and unit-tests that capability
against a mock transport*, not *this capability was exercised live*.
