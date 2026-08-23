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
  **Amended 2026-08-22 (plan 17-20), further amended 2026-08-23 (plan 17-21 gap closure):** the
  identifiers above are the values this phase originally shipped and are preserved here rather
  than deleted, per D-00d. Live verification (plans 17-18, 17-19, 17-21) has since falsified three
  of them: Kimi's default is now `kimi-k3` (`moonshot-v1-8k` is retired, returns `404`), Grok's
  default is now `grok-4.6` (`grok-4` is absent from the live catalog), and Qwen's base URL moved
  twice — to `dashscope-us.aliyuncs.com/compatible-mode/v1` (US/Virginia) on 2026-08-22, when a
  Virginia-scoped credential proved the Singapore endpoint above returned a well-formed `401`
  disguised as an authentication failure, then back to
  `dashscope-intl.aliyuncs.com/compatible-mode/v1` (Singapore) on 2026-08-23, when the project's
  credential was replaced with a Singapore-scoped key and the Virginia default began failing the
  same way in the opposite direction. Neither move is a universally correct answer — see the
  "Reversal record" in `crates/paladin-llm/src/qwen/adapter.rs` for why. Gemini's
  `gemini-2.5-flash` default (line 31 above) is likewise retired for new users and has been
  refreshed to `gemini-3.6-flash`. The shipped code and every operator-facing document now carry
  these refreshed values; this paragraph is the historical record of what changed and why.
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
| `[compat] sampling fields (temperature, max_tokens, top_p, penalties)` | INTEGRATE | The five tunables the wire type carries; each is omitted when unset rather than sent as null. **Amended 2026-08-22 (plan 17-20):** the original reason above described only the caller-unset case. Live measurement (plans 17-18, 17-19) found "unset" is an incomplete description of what a vendor may require: each preset now declares, via `CompatRequestParameters`, which of the five fields its request path carries at all — Grok's request path omits `frequency_penalty`/`presence_penalty` unconditionally (xAI rejects them by presence, measured live), and Kimi's omits `temperature`/`top_p` unconditionally (Moonshot enforces fixed values on both, measured live: `temperature` accepts only `1.0`, `top_p` only `0.95`). This is a per-preset request-shaping contract, not a per-call caller choice — the field can be absent even when the caller supplied a value. |
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

**Superseded 2026-08-22 (plan 17-20) — original text preserved below per D-00d, not deleted,
because it was true when written and is the record of why a live run mattered.**

> Recorded for honesty, and consistent with `17-UAT.md` test 4: the base URLs, default model IDs
> and live-fetch behaviour above are taken from **vendor documentation and the shipped code**, and
> have **not** been confirmed against a live endpoint — the phase's sandbox has no network egress
> and no vendor API keys. `README.md`, `config.example.yml` and
> `docs/src/getting-started/configuration.md` all carry that caveat explicitly. An `INTEGRATE`
> decision in this matrix therefore means *this phase implements and unit-tests that capability
> against a mock transport*, not *this capability was exercised live*.

### What changed (2026-08-22, superseded in part 2026-08-23)

Network egress and all four hosted-vendor credentials now exist in this environment. A live run
(plans 17-18, 17-19, 17-21) both **confirmed** facts the paragraph above could only cite, and
**falsified** others outright:

- **Confirmed live:** Grok (xAI), Kimi (Moonshot) and Gemini each PASS both a live model-list
  fetch and a live `generate()` round trip with the framework's default prompt parameters, using
  the currently-shipped default model. **Qwen (DashScope) now PASSES both probes too** — see
  "2026-08-23 update" below for what changed since the entitlement-gap text this paragraph
  originally carried.
- **Falsified:** Grok's shipped default model (`grok-4`) and Kimi's (`moonshot-v1-8k`) were both
  **absent** from their vendor's live catalog and rejected outright; Qwen's shipped base URL
  (originally the Singapore endpoint) returned a well-formed `401` that was previously read as
  "the URL is right, only the key is wrong" — that reading does not hold for a region-scoped
  credential, which returns the identical `401` envelope from every endpoint except its own. All
  three are now corrected in the shipped constants and every operator-facing document (17-20,
  17-21 gap closure).
- **No row remains blocked on a credential or an account entitlement.** This reverses the previous
  text's premise ("the sandbox has no network egress and no vendor API keys"), which is no longer
  true for any of the four hosted vendors.

### 2026-08-23 update: Qwen's `generate()` blocker is resolved, not merely re-diagnosed

The account-entitlement gap this section originally described (`HTTP 403 Model.AccessDenied` on
every model, `.planning/WINDOWS.md` id 21) was tied to a specific DashScope credential and its US
(Virginia) workspace. That credential was replaced with a **Singapore-scoped** key on 2026-08-23.
Against the new key and the Singapore endpoint, every measured request succeeded: `GET /models`
returned 162 entries, `generate()` returned real completions for both `qwen-plus` and the
candidate `qwen3.7-plus`, and each of the five optional sampling parameters plus both
`temperature_range` endpoints were probed individually with no rejection below DashScope's
documented `[0.0, 2.0)` temperature ceiling. `.planning/WINDOWS.md` id 21 is closed by this
credential change plus the corrected shipped default (17-21 gap closure) — the entitlement gap
was never a code defect, and the fix that resolved it was external, not a fix to `paladin-llm`.

### Per-surface: live-exercised vs. mock-transport-only

| Surface | Live-exercised end to end | Remains mock-transport-only |
|---|---|---|
| `[compat] POST /chat/completions` (non-streaming) | Grok, Kimi, Qwen | — |
| `[compat] GET /models` | Grok, Kimi, Qwen, Ollama (local) | — |
| `[compat] response usage (token counts)` | Grok, Kimi, Qwen (all three `generate()` responses carry live token usage) | — |
| `[compat] request field stream` / SSE streaming | — | Grok, Kimi, Qwen, Ollama, openai-compatible (no live streaming probe exists; unit-tested against a mock transport only) |
| `[compat] response finish_reason` | Grok, Kimi, Qwen (live completions returned a mapped finish reason) | Ollama, openai-compatible |
| `[compat] response reasoning_content` | — | All five presets — no vendor in the live run emitted a `reasoning_content` field, so this path is still mock-transport-only |
| `[compat] sampling fields (temperature, max_tokens, top_p, penalties)` | Grok's and Kimi's *omission* declarations (measured against the live rejection each vendor returned); Qwen's *acceptance* of all five, individually measured, plus its narrowed `temperature_range` upper bound | The remaining accepted fields on Ollama |
| `[gemini]` every INTEGRATE row above | Model list, `generateContent` (non-streaming), `usageMetadata` mapping | `streamGenerateContent` (no live streaming probe) |
| `[ollama] GET /v1/models` | — (local Ollama in the Docker Tier 2 suite, not this live-vendor harness) | Chat completions against a local Ollama instance are exercised only in the Docker Tier 2 suite (UAT test 3), not the hosted-vendor live harness this section otherwise describes |

**Qwen's `generate()` — resolved 2026-08-23, live-exercised end to end.** Every one of the five
optional sampling parameters (`temperature`, `max_tokens`, `top_p`, `frequency_penalty`,
`presence_penalty`), probed individually against both `qwen-plus` and `qwen3.7-plus` at the
shipped Singapore endpoint, returned `HTTP 200` with a real completion — no rejection was
observed, so `CompatRequestParameters::all()` remains the measured (not merely inherited)
declaration. Both `temperature_range` endpoints were probed: `0.0` is accepted, `2.0` is rejected
(`HTTP 400 InternalError.Algo.InvalidParameter: "Temperature should be in [0.0, 2.0)"`), which
narrowed the declared range to `(0.0, 1.99)` — DashScope's own documented range is half-open, and
the framework's validation gate treats a declared range as inclusive on both ends.

An `INTEGRATE` decision in the Matrix above still means, at minimum, *this phase implements and
unit-tests that capability against a mock transport* — the table in this section is what narrows
that baseline down to what a live run has now additionally proven, surface by surface.
