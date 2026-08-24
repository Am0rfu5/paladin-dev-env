# Phase 17: Additional LLM Provider Adapters - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-16
**Phase:** 17-additional-llm-provider-adapters
**Areas discussed:** Shortlist scope, Adapter shape, Factory & flags, Model list currency

---

## Area selection

| Option | Description | Selected |
|--------|-------------|----------|
| Adapter shape | Copy per provider, shared compatible core, or one generic adapter | ✓ |
| Shortlist scope | What qualifies a provider; how many to maintain | ✓ |
| Model list currency | Hardcoded list vs live `/models` vs permissive validation | ✓ |
| Factory & flags | Extend the hardcoded match vs generalize; two live defects | ✓ |

**User's choice:** all four areas, plus a free-text addition — *"Also adding Grok from x.ai"*.
**Notes:** Grok was accepted as a new candidate row rather than treated as scope creep — PROV-01's
candidate field is explicitly open, and xAI's API is OpenAI-compatible, so it costs the same as
Kimi and Qwen. Candidate field became Kimi, Gemini, Qwen, a Llama host, and Grok.

---

## Cross-referenced todos

| Option | Description | Selected |
|--------|-------------|----------|
| Leave it out | Matched on the keyword "coverage" only; PROV-04 already binds the 82% floor | ✓ |
| Fold it in | Treat local-vs-CI coverage reproduction as part of PROV-04 | |

**User's choice:** Leave it out.
**Notes:** `2026-08-13-verify-local-coverage-reproduction.md`, score 0.60. Recorded under Reviewed
Todos in CONTEXT.md so future phases know it was considered.

---

## Shortlist scope

### Q1 — the qualifying bar

| Option | Description | Selected |
|--------|-------------|----------|
| Cheap ones all in, Gemini too | Every OpenAI-compatible candidate plus Gemini as the one bespoke adapter | ✓ |
| Compatible only — defer bespoke | Only what the shared core gives nearly free; Gemini deferred with a trigger | |
| Demand-ranked, cap the count | Rank by user demand regardless of protocol, take top N | |
| Let the researcher rank it | Defer the bar; researcher returns a scored table at plan time | |

**User's choice:** *"Cheap ones all in, gemini and Grok from x.ai too"*
**Notes:** Cost-against-the-core is the bar, not brand recognition. Grok restated explicitly as in.
Presented with the cost split — compatible (Kimi, Qwen, Grok, Groq, Together, Ollama, Mistral) vs
bespoke (Gemini, Bedrock) — since that split is what makes the answer cheap.

### Q2 — the Llama row

| Option | Description | Selected |
|--------|-------------|----------|
| Ollama — local/self-hosted | Only candidate testable without a key; fits Tier 2 Docker-gating | ✓ |
| Groq — hosted, fast inference | OpenAI-compatible, free tier, low latency on Llama models | |
| Together — hosted, broad catalog | Widest open-model catalog behind one key | |
| Drop the Llama row | Reject with reason; Kimi/Qwen/Grok already cover open-weight compatible space | |

**User's choice:** Ollama only (multi-select offered; one selected).
**Notes:** "Meta (Llama)" named a model family with no endpoint behind it — the original request's
own question mark was the right instinct. Ollama additionally unlocks the only credential-free
end-to-end test of the shared core, which later drove D-15.

### Q3 — generic provider as public surface

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — ship a generic provider too | Rest of field rejected as already covered rather than deferred | ✓ |
| No — named adapters only | Core stays internal; every provider needs a shipped adapter | |
| Yes, but keep it undocumented | Build it, don't advertise it as supported | |

**User's choice:** Yes — ship a generic provider too.
**Notes:** The highest-leverage decision in the session. It changes the disposition of Groq,
Together, Mistral, Fireworks and Bedrock from "defer" to "rejected — already covered", so future
requests for them do not become new phases. Rated one-way in CONTEXT.md: it is public API.

### Q4 — generic provider capabilities

| Option | Description | Selected |
|--------|-------------|----------|
| Operator declares, conservative defaults | Unset means pessimistic; never claims what nobody asserted | ✓ |
| Probe the endpoint at construction | Most accurate; fallible construction, breaks offline testing | |
| Assume full OpenAI parity | Simplest; exactly the over-reporting Phase 14 fixed | |

**User's choice:** Operator declares, conservative defaults.
**Notes:** Framed against Phase 14's capability-flag defect, which this phase could easily
reintroduce through a generic adapter pointed at an unknown endpoint.

---

## Adapter shape

### Q1 — shape of the named providers

| Option | Description | Selected |
|--------|-------------|----------|
| Thin presets over the core | ~150 lines per provider; fixes land once for all | ✓ |
| Full copies of the DeepSeek adapter | Matches shipped layout; ~5,000 lines of near-duplicate | |
| Core now, migrate the existing three too | Cleanest end state; touches shipped covered code | |

**User's choice:** Thin presets over the core.
**Notes:** The generic-provider decision had already forced a core to exist, so the question was
only what the named providers become once it does. The third option was declined, keeping the phase
additive — recorded as D-06 and as a deferred idea.

### Q2 — `get_provider_name() -> &'static str`

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed literal for generic | No trait change, no leak; two generic instances share a name in logs | ✓ |
| Widen the trait to `&str` | True names; breaking public port change, moves `current-exports.txt` | |
| Leak the configured name | True names, no trait change; deliberate bounded leak | |

**User's choice:** Fixed literal for generic.
**Notes:** Keeps `LlmPort` untouched and the phase strictly additive. The trait widening is
preserved as a deferred idea with a trigger.

### Q3 — Gemini scope

| Option | Description | Selected |
|--------|-------------|----------|
| Text only, vision deferred | Implements `LlmPort`, reports `supports_vision: false` | ✓ |
| Text + vision together | Matches the two-file `openai/` and `anthropic/` layout | |
| Defer Gemini entirely | Ship only compatible providers this phase | |

**User's choice:** Text only, vision deferred.
**Notes:** Vision is not part of the `LlmPort` contract PROV-02 defines — it is the `vision` feature
and the Sentinel surface. Deferred idea recorded with a trigger.

### Q4 — naming the generic provider

| Option | Description | Selected |
|--------|-------------|----------|
| Plain technical name | Wire-protocol term of art, like `LlmPort` and `OpenAIAdapter` | ✓ |
| Give it a domain name | New framework concept earning a medieval term like Garrison or Herald | |

**User's choice:** Plain technical name.
**Notes:** Asked because CLAUDE.md makes the ubiquitous language mandatory where a domain noun is
coined, and the generic provider is a coined concept rather than a proper noun like Kimi or Grok.
Phase 15.1's D-00h had already drawn the term-of-art line; this confirms it applies here.

---

## Factory & flags

### Q1 — factory structure

| Option | Description | Selected |
|--------|-------------|----------|
| Table-driven registry | One `cfg`-gated table as single source; fixes the gating defect structurally | ✓ |
| Extend the match, fix the bug | Smallest diff; leaves nine providers × four hand-maintained sites | |
| Registry with runtime registration | Most extensible; new public surface beyond PROV-03 | |

**User's choice:** Table-driven registry.
**Notes:** Presented with the arithmetic — nine providers × four hardcoded sites = 36 hand-maintained
edit points — and with the live defect: `get_default_provider` and `list_available_providers` carry
no `#[cfg]` guards and report compiled-out providers as available.

### Q2 — the inert facade flags

| Option | Description | Selected |
|--------|-------------|----------|
| Fix it — wire all flags properly | Restores the compile-time contract; default build stops including Anthropic/DeepSeek | ✓ |
| New flags wired, existing three untouched | Zero behaviour change; two classes of flag, some real, some inert | |
| Match the existing pattern — stubs all round | Smallest diff; flag surface entirely decorative | |
| Fix it, but in its own phase | Keeps Phase 17 additive; new providers plug into a known-broken system | |

**User's choice:** Fix it — wire all flags properly.
**Notes:** Accepted as a BREAKING change with a CHANGELOG entry. Rated one-way in CONTEXT.md —
it changes what a default build contains for every consumer.

### Q3 — provider names and env vars

| Option | Description | Selected |
|--------|-------------|----------|
| Vendor's own convention | `MOONSHOT_API_KEY`, `DASHSCOPE_API_KEY`, `XAI_API_KEY`, `GEMINI_API_KEY` | ✓ |
| Brand name for both | `KIMI_API_KEY`, `QWEN_API_KEY`, `GROK_API_KEY` — guessable, not drop-in | |
| Paladin-namespaced env vars | `PALADIN_KIMI_API_KEY` — owns its namespace, breaks the existing pattern | |

**User's choice:** Vendor's own convention.
**Notes:** Matches how the shipped three already work and gives drop-in compatibility for operators
who already export those variables for other tooling.

---

## Model list currency

### Q1 — model listing and validation

| Option | Description | Selected |
|--------|-------------|----------|
| Live `/models`, curated fallback | Current without a release; degrades gracefully offline | ✓ |
| Curated lists only, as today | Offline and deterministic; guaranteed to go stale | |
| Permissive validation | Never wrongly blocks; turns a config typo into a runtime API error | |

**User's choice:** Live `/models`, curated fallback.
**Notes:** Motivated by `deepseek/adapter.rs:793` — a hardcoded two-entry list that `validate_model`
gates on. Tolerable for DeepSeek, wrong for Kimi and Qwen, actively wrong for Ollama where the
catalog is whatever the operator pulled. Applies to new adapters only; D-06 leaves the shipped
three alone.

### Q2 — caching and freshness

| Option | Description | Selected |
|--------|-------------|----------|
| Fetch once, cache for adapter lifetime | One call per adapter; restart needed to see a newly pulled Ollama model | ✓ |
| Short TTL cache | Fresher; adds a time-dependent path to test | |
| Cache hosted, always-live for Ollama | Most correct per provider; two behaviours to document | |

**User's choice:** Fetch once, cache for adapter lifetime.
**Notes:** Consistent with PROJECT.md's existing "config is read once at startup, no hot-reload"
posture, which was cited when presenting the accepted consequence.

### Q3 — Ollama testing depth

| Option | Description | Selected |
|--------|-------------|----------|
| Mock in Tier 1, Ollama also Tier 2 | Only credential-free end-to-end exercise of the shared core | ✓ |
| Mock transport only, all six | Fastest; core never tested against a real implementation | |
| Mock Tier 1, plus Tier 3 for the hosted four | Real-provider confidence; needs four API keys | |

**User's choice:** Mock in Tier 1, Ollama also Tier 2.
**Notes:** Ollama's no-credential property is what makes this possible, and the Tier 2 suite
validates the engine every preset shares — not just Ollama.

---

## Claude's Discretion

Set aside deliberately rather than spent on user turns, per the workflow's rule that codebase
patterns and implementation approach belong to the planner:

- Module layout — `compat/` grouping vs per-provider top-level directories.
- Where the core lives — `paladin-llm` assumed; ADR-0035's leaf-crate precedent is ML-scoped.
- Retry and streaming parity for a local Ollama endpoint where retry means less.
- `config.yml` shape — per-provider block vs provider list.
- How PROV-01's study output is recorded — ADR vs requirement rows (D-00g governs).

Also offered at the closing check and declined: tool/function-calling handling when only some
providers support it; the CHANGELOG/README/docs surface wording.

## Deferred Ideas

- Gemini vision adapter (`gemini/vision.rs`, `vision` feature, Sentinel surface).
- Migrating `openai/` and `deepseek/` onto the shared compatible core.
- Groq, Together, Mistral, Fireworks, Bedrock as named presets — rejected, not deferred, since the
  generic provider reaches them.
- Widening `get_provider_name()` to `&str`.
- Live key-gated tests for Kimi, Qwen, Grok and Gemini.

Each carries a trigger in CONTEXT.md's `<deferred>` section.
