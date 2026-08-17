# Phase 17: Additional LLM Provider Adapters - Context

**Gathered:** 2026-08-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Paladin gains adapters for the additional LLM providers that qualify under a recorded selection
study. The study (PROV-01) is the first deliverable and its verdicts set the build list; the
adapters (PROV-02 … PROV-04) implement the full `LlmPort` contract behind per-provider feature
flags.

**In scope:** the provider-selection study and its recorded verdicts; five named adapters (Kimi,
Qwen, Grok, Ollama, Gemini); one generic operator-configured OpenAI-compatible provider; the
shared compatible core those presets sit on; the factory and feature-flag wiring that resolves
them; tests and documentation to the standard already in force.

**Out of scope:** any vision or multimodal surface (`LlmPort` has no vision method — that is the
`vision` feature and the Sentinel subsystem); migrating the shipped `openai/`, `anthropic/` and
`deepseek/` adapters onto the new core; embeddings; changes to `LlmPort` itself.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15 and 15.1 — locked, not re-litigated

- **D-00a [informational]:** ADRs live in `.planning/decisions/`, flat sequential numbering, file
  shape `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter**. Check `.planning/decisions/PROMOTION.md` for the next
  free number before authoring; update that line if this phase authors one.
- **D-00b [informational]:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** *(Phase 1 D-02.)*
- **D-00d:** Ledgers and requirement texts are **amended in place**, dated, superseded text
  retained. Never a separate corrections file. *(Phase 2 D-02.)*
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that
  produced it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10, 12, 13, 14, 15, 15.1.)*
- **D-00f:** Primary key is the requirement ID. **This phase has four: PROV-01 … PROV-04**, defined
  in `REQUIREMENTS.md` under *v1 Requirements — Provider Expansion (Phase 17)*. They are the first
  forward requirements in that file with no ingested `REQ-*` ancestor — they cite user direction.
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory where a domain noun
  is coined, not for terms of art. *(CLAUDE.md; scope as clarified by Phase 15.1.)* **D-09 applies
  this rule directly.**
- **D-00i [informational]:** Provenance of `--auto` decisions is carried forward rather than
  laundered. *(Phase 12 hand-off item 6.)* **Every decision in this file was human-selected in an
  interactive `/gsd-discuss-phase 17` session on 2026-08-16.**
- **D-00n [informational] — the project is at 0.8.0.** `release.toml` sets `shared-version = true`;
  all twelve manifests move lockstep. Pre-1.0 semantics (ADR-0008) absorb a breaking change as a
  minor bump, which is what **D-11** relies on.
- **D-00o [informational] — `workflow.worktree_skip_hooks: true`** in `.planning/config.json`.
  Surface this in executor prompts or every commit cold-compiles the workspace.
- **D-00p [informational] — the coverage floor is 82% and the gate is live and required.**
  `cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82` runs at
  `.github/workflows/ci.yml:664` and `Makefile:261`. ADR-0006 is the binding record. This phase does
  not touch the floor; **PROV-04 binds this phase's new code to it.**
- **D-00q [informational] — ADR-0004 governs temperature validation.**
  `ProviderCapabilities.temperature_range: Option<(f32, f32)>` is provider-aware, falling back to
  the global `[0.0, 1.0]` when a provider declares `None`. Every adapter this phase adds populates
  it; none re-opens the mechanism.
- **D-00r [informational] — ADR-0016 owns the port value types.** `PaladinResult`, `StopReason` and
  `TokenUsage` live in `paladin-core` and are re-exported by `paladin-ports`. No adapter added here
  defines its own `TokenUsage`; DEBT-05 already collapsed the duplicates.
- **D-00s [informational] — ADR-0031 governs crate dependency direction.** The extracted-crate rule
  is a default-build invariant, checkable via `cargo tree --no-default-features`. Nothing added here
  may create a leaf-to-leaf or facade edge in a default build.

---

### PROV-01 — which providers qualify

- **D-01: The build list is Kimi, Qwen, Grok, Ollama and Gemini.** The qualifying bar is
  build cost against the shared core, not brand recognition: every OpenAI-compatible candidate is
  in because its marginal cost approaches zero once the core exists, and Gemini is in as the one
  bespoke adapter worth paying for. Grok (xAI) was added to the candidate field by the user during
  this session and qualifies on the same basis — `api.x.ai` is OpenAI-compatible.

- **D-02: Ollama settles the "Meta (Llama)" row; Groq and Together are not built.** The original
  request named "Meta (Llama)", which is a model family with no endpoint behind it. Ollama is the
  chosen host: it is the only candidate testable without anyone's API key, the only one giving
  self-hosted coverage, and it fits the shipped three-tier test strategy as a Docker-gated Tier 2
  service alongside Redis/MinIO/Qdrant. Groq and Together were considered and not selected.

- **D-03: A generic operator-configured OpenAI-compatible provider ships as public surface.**
  Consumers can point Paladin at any compatible endpoint — `base_url` + key + model + declared
  capabilities — with no new code. **This changes the disposition of the rest of the field: Groq,
  Together, Mistral, Fireworks and Bedrock are rejected in PROV-01 as *already covered*, not
  deferred**, so future requests for them do not become new phases. The five named adapters remain
  as curated presets carrying correct capabilities and model lists.
  — **Reversibility:** one-way — it is public API that operators write configuration against;
  withdrawing it later removes a shipped capability and breaks every deployment pointing at an
  endpoint Paladin never named.

- **D-04: The generic provider's capabilities are operator-declared with pessimistic defaults.**
  Capabilities are config fields. Unset means the conservative answer: `supports_streaming: true`
  (it is in the compatible spec), `supports_tool_calling` / `supports_function_calling` /
  `supports_vision` / `supports_embeddings` all `false`, `temperature_range: None` so ADR-0004's
  global `[0.0, 1.0]` applies, `max_context_tokens: None`. **It must never claim a capability
  nobody asserted** — Phase 14 already paid to fix one capability flag that over-reported, and an
  adapter pointed at an unknown endpoint is the easiest place to reintroduce that defect.
  — **Reversibility:** costly — defaults define what operators must set; loosening them later is
  additive, but tightening them breaks working configurations that relied on an inferred `true`.

### PROV-02 — adapter shape and the `LlmPort` contract

- **D-05: A shared OpenAI-compatible core owns the protocol; named providers are thin presets.**
  One engine owns the request/response types, streaming chunk assembly, retry and error mapping
  into `LlmError`. Each of Kimi, Qwen, Grok and Ollama supplies only `base_url`, env-var name,
  default model, model list and a capabilities block. The alternative — copying
  `deepseek/adapter.rs` four times — was rejected at roughly 5,000 lines of near-duplicate and four
  places to apply every future fix.
  — **Reversibility:** costly — once four presets and the generic provider depend on the core, a
  change to its request shaping or error mapping touches all of them at once.

- **D-06: The three shipped adapters are not migrated onto the core.** `openai/`, `anthropic/` and
  `deepseek/` are left exactly as they are. This phase is additive: refactoring working, covered,
  shipped code would widen the blast
  radius well past what PROV-03's "adding a provider does not change the behaviour of any existing
  one" allows. The duplication between `openai/adapter.rs` and `deepseek/adapter.rs` is recorded as
  a deferred idea, not carried here.

- **D-07: `LlmPort` is not changed; the generic provider returns a fixed literal name.** Presets
  return their own literals (`"kimi"`, `"qwen"`, `"grok"`, `"ollama"`, `"gemini"`); the generic
  adapter returns one honest constant such as `"openai-compatible"`. Widening
  `get_provider_name() -> &'static str` to `&str` was considered and rejected for this phase: it is
  a breaking change to the public port trait that moves every adapter signature, the mock, and
  `.project/current-exports.txt` — the `api-surface` guard DEBT-01 repaired. Leaking the configured
  name was also rejected. **Accepted cost:** two generic adapters pointed at different endpoints
  report the same provider name in logs.
  — **Reversibility:** reversible — nothing shipped changes, and widening the trait later remains
  open as its own decision.

- **D-08: Gemini is bespoke and text-only.** It cannot use the compatible core — `generateContent`
  is its own protocol, closer to the 1,180-line Anthropic adapter. It implements `LlmPort` and
  reports `supports_vision: false`, which is true of what ships. **Vision is a different surface**
  (`openai/vision.rs`, `anthropic/vision.rs`, the `vision` feature, Sentinel) with no `LlmPort`
  method behind it; a Gemini vision adapter is deferred with a recorded trigger.
  — **Reversibility:** reversible — adding a `gemini/vision.rs` later is purely additive and the
  capability flag already reports the truth in the meantime.

- **D-09: The generic provider gets a plain technical name, not a medieval-military one.**
  `OpenAiCompatibleAdapter` / `"openai-compatible"`. It is a wire-protocol term of art in the same
  class as `LlmPort`, `ProviderCapabilities` and the existing `OpenAIAdapter` / `DeepSeekAdapter`
  names, none of which are themed. Per D-00h the ubiquitous language binds coined domain nouns, not
  protocol terms.

### PROV-03 — factory, feature flags and naming

- **D-10: A table-driven registry replaces the factory's hardcoded match.** One `cfg`-gated static
  table of (name, env var, constructor) becomes the single source, and `create()`,
  `get_default_provider()`, `list_available_providers()` and the `UnknownProvider` error text all
  derive from it. Adding a provider becomes one row instead of four hand-maintained sites.
  **This fixes a live defect structurally rather than by remembering to:** `get_default_provider`
  and `list_available_providers` (`crates/paladin-llm/src/provider_factory.rs:123-149`) are not
  `cfg`-gated today and report a provider as available whenever its env var is set, even when its
  feature was compiled out. Under a `cfg`-gated table a compiled-out provider simply is not present.
  — **Reversibility:** costly — the registry becomes the contract every provider registers through;
  returning to a match means unpicking all nine call sites.

- **D-11: The facade's `llm-` provider flags are wired for real.** The resulting default-build change is
  recorded as BREAKING. Root `Cargo.toml` declares `llm-openai = []`, `llm-anthropic = []` and
  `llm-deepseek = []` — empty stubs — while pulling `paladin-llm` in at line 55 with
  `features = ["openai", "anthropic", "deepseek", "mock", "vision"]` unconditionally, so every build
  today compiles all three providers regardless of flags and PROJECT.md's "unavailable adapters must
  fail at compile time, never at runtime" is not what ships. Each flag becomes
  `llm-<provider> = ["paladin-llm/<provider>"]`, the new providers get flags on the same shape, and
  `llm-all` and `full` are extended. **A default build stops silently including Anthropic and
  DeepSeek**; that is a behaviour change and gets a CHANGELOG `BREAKING` entry. Leaving the existing
  three as stubs while wiring only the new five was rejected — two classes of flag, some real and
  some inert, is harder to explain than either end state.
  — **Reversibility:** one-way — it changes what a default build contains for every consumer;
  undoing it is a second breaking change, not a revert.

- **D-12: Provider names and API-key env vars follow each vendor's own convention.**
  `"kimi"` / `MOONSHOT_API_KEY`, `"qwen"` / `DASHSCOPE_API_KEY`, `"grok"` / `XAI_API_KEY`,
  `"gemini"` / `GEMINI_API_KEY`, `"ollama"` / none required. This matches how the shipped three
  already work (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `DEEPSEEK_API_KEY`) and means anyone who
  already exports those variables for other tooling gets a working setup with no extra
  configuration. Paladin-namespaced variables were considered and rejected as breaking that
  drop-in property.
  — **Reversibility:** costly — env-var names are an operator-facing configuration contract;
  renaming later breaks every deployment and every documented example.

### PROV-04 — model currency and testing

- **D-13: `get_available_models()` queries the provider's live endpoint with a curated fallback.**
  All five expose a models endpoint (Ollama's is local). When the call fails or the process is
  offline, the adapter falls back to a curated list; `validate_model` returns true for anything the
  live list contains. Today's pattern — a hardcoded `vec!` at `deepseek/adapter.rs:793` that
  `validate_model` checks membership in — rejects any model the provider ships after the release,
  which is tolerable for DeepSeek's two entries but wrong for Kimi and Qwen, and actively wrong for
  Ollama where the catalog is whatever the operator pulled. The fallback path keeps this inside
  PROJECT.md's "optional features degrade gracefully, never fatally" constraint.

- **D-14: The model list is fetched lazily and memoized for the adapter's lifetime.** Hosted
  providers do not add models mid-process, so one call per adapter is right. **Accepted
  consequence:** an operator who pulls a new Ollama model must restart to see it — consistent with
  PROJECT.md's existing "config is read once at startup, no hot-reload" posture. A TTL cache and a
  per-provider split (memoize hosted, always-live for Ollama) were both considered and rejected as
  extra time-dependent code paths to test.

- **D-15: Mock transport in Tier 1 for all six; Ollama additionally Docker-gated in Tier 2.** Every
  provider gets mock-transport unit tests covering request shaping, response parsing, streaming
  chunk assembly and error mapping, so CI stays fast and offline. Ollama additionally gets a
  Docker-gated suite with a small model — **the only end-to-end exercise of the shared compatible
  core against a real implementation of the protocol that costs no credentials**, which validates
  the engine every preset depends on. Live key-gated tests for the hosted four were considered and
  not added; the existing `live-api-tests` flag remains available for them.

### Claude's Discretion

- **Module layout** — whether the core and presets live under a `compat/` module or as per-provider
  top-level directories mirroring `openai/` and `anthropic/`. Both satisfy the decisions above.
- **Where the core lives** — inside `paladin-llm` rather than a new crate is the working assumption;
  the crate's own description is "LLM provider adapters for the Paladin framework". ADR-0035's
  leaf-crate precedent concerns a *TensorFlow/ML* adapter and does not bind LLM providers.
- **Retry and streaming parity** — whether the core's retry behaviour (modelled on
  `call_api_with_retry`) applies unchanged to a local Ollama endpoint where retry means less.
- **Config surface shape** — whether `config.yml` gains a per-provider block or a provider list;
  constrained only by PROV-03's requirement that existing config files keep loading.
- **How PROV-01's study is recorded** — an ADR versus requirement-row verdicts. D-00g's rule
  applies: the contested parts (D-03's rejection of the rest of the field, D-11's breaking change)
  are ADR-shaped; the rest is ledger material.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope

- `.planning/ROADMAP.md` — Phase 17 entry: goal, dependencies, and the five success criteria.
  Criterion 1 requires each named candidate be explicitly dispositioned; **D-01 and D-02 supply
  those verdicts**, and criterion 1's "Llama names a model family, not a provider" note is what
  D-02 settles.
- `.planning/REQUIREMENTS.md` §*v1 Requirements — Provider Expansion (Phase 17)* — PROV-01 …
  PROV-04, the four requirements this phase closes. **PROV-02's size is set by PROV-01's verdicts**;
  D-01 fixes it at five named adapters plus the generic provider.
- `.planning/PROJECT.md` §Constraints — the feature-gating compile-time contract, the offline
  testing rule, the three-tier test strategy, and the graceful-degradation rule D-13 relies on.

### Decisions this phase applies or must not re-open

- `.planning/decisions/0004-temperature-validation.md` — provider-aware `temperature_range`.
  Every adapter populates it; D-04 sets `None` for the generic provider.
- `.planning/decisions/0006-coverage-gate.md` — the binding 82% floor. Not amended here.
- `.planning/decisions/0016-port-value-type-ownership.md` — `TokenUsage` ownership; no adapter
  redefines it.
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — the default-build invariant any
  new module placement must respect.
- `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` — the leaf-crate precedent, scoped
  to ML adapters; read before concluding it forces a new crate for LLM providers.
- `.planning/decisions/PROMOTION.md` — next free ADR number; update it if this phase authors one.
- `.planning/phases/14-api-contract-truthfulness/14-CONTEXT.md` — **the capability-truthfulness
  standard D-04 inherits.** Phase 14 fixed a capability flag that over-reported; do not reintroduce
  the defect through a generic adapter.

### Code this phase reads or changes

- `crates/paladin-ports/src/output/llm_port.rs` — the contract PROV-02 requires in full:
  `generate`, `generate_stream`, `validate_model`, `get_available_models`, `get_provider_name`
  (`:1291`, the `&'static str` D-07 works around), `get_capabilities` (`:1363`), and
  `ProviderCapabilities` with its eight fields.
- `crates/paladin-llm/src/deepseek/adapter.rs` — **the template for a compatible-protocol adapter
  and the source of two findings.** `{base_url}/chat/completions` at `:602` and `:705`,
  `map_error` at `:501`, `call_api_with_retry` at `:546`, the hardcoded model list at `:793`
  (D-13), and the capabilities block at `:804` showing `temperature_range: Some((0.0, 2.0))`.
- `crates/paladin-llm/src/openai/adapter.rs` — the second copy of the same protocol; read alongside
  DeepSeek's to size what the core in D-05 should absorb.
- `crates/paladin-llm/src/anthropic/adapter.rs` — the bespoke-protocol template Gemini follows
  under D-08.
- `crates/paladin-llm/src/provider_factory.rs` — **D-10's target.** The hardcoded match (`:63-116`),
  the `UnknownProvider` message naming supported providers (`:16`), and the un-`cfg`-gated
  `get_default_provider` / `list_available_providers` (`:123-149`).
- `crates/paladin-llm/src/config/llm.rs` and `config/bridge.rs` — the configuration surface new
  providers extend; ADR-0017 accepts the bridge's location.
- `crates/paladin-llm/Cargo.toml` — `default = ["openai", "mock"]` plus per-provider flags; also the
  `description` and `keywords` PROV-04 requires be brought in line.
- `Cargo.toml` (workspace root) — **D-11's target.** `default = ["llm-openai"]` and the empty
  `llm-openai` / `llm-anthropic` / `llm-deepseek` stubs at `:264-268`, against the unconditional
  `paladin-llm` feature list at `:55`. Also `live-api-tests` and `full`.
- `.github/workflows/ci.yml:664` and `Makefile:261` — the 82% coverage gate the new code must clear.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`deepseek/adapter.rs` is a working OpenAI-compatible adapter** — request/response structs,
  SSE stream assembly, retry with backoff, status-to-`LlmError` mapping, and a `base_url` already
  configurable from `DEEPSEEK_BASE_URL`. It is the body of the core D-05 extracts, not a starting
  sketch.
- **`ProviderCapabilities` already carries all eight fields** the new adapters need, including
  ADR-0004's `temperature_range`. No port-side type work is required.
- **The three-tier test strategy already exists** with Docker-gated services (Redis, MinIO, Qdrant)
  and an `integration-tests` / `live-api-tests` flag pair. D-15's Ollama service slots into the
  existing Tier 2 shelf rather than inventing a tier.
- **`mock.rs`** — the existing `LlmPort` test double, and the model for how the new adapters' mock
  transports should behave.

### Established Patterns

- **One directory per provider** (`openai/`, `anthropic/`, `deepseek/`), each with `mod.rs` +
  `adapter.rs`, config struct with `from_env()` / `new()` / `validate()`, private wire types, and
  `#[cfg(test)]` unit tests in-file.
- **Feature flag per provider in `paladin-llm`**, with `default = ["openai", "mock"]`.
- **Env-var-only credentials** — PROJECT.md forbids API keys in CLI args, config files or logs.
- **No `unwrap()` / `expect()` / `panic!` in library code**; layer-specific errors converted at
  boundaries via `From`.

### Integration Points

- `LlmProviderFactory::create()` — where every new provider becomes reachable by name (D-10).
- Root `Cargo.toml` feature graph — where facade flags forward into `paladin-llm` (D-11).
- `config/bridge.rs` — where provider configuration crosses from application settings into adapters.
- `.project/current-exports.txt` — the `api-surface` CI guard; new public types must be regenerated
  into it or the job goes red. This is the guard DEBT-01 repaired; do not break it.

</code_context>

<specifics>
## Specific Ideas

Four findings from scouting the shipped tree drove the decisions above. All four are stated as
observations against `file:line`, per D-00e.

1. **The compatible providers are one protocol, implemented twice already.**
   `deepseek/adapter.rs` (1,368 lines) and `openai/adapter.rs` (741) both POST to
   `{base_url}/chat/completions` with their own private `*Request` / `*Message` / `*Response` /
   `*StreamDelta` structs. Kimi, Qwen, Grok and Ollama all speak that same protocol. Four more
   copies would be roughly 5,000 lines of near-duplicate — the direct evidence behind D-05.

2. **`get_available_models()` is a hardcoded `vec!` and `validate_model` gates on it.**
   `deepseek/adapter.rs:793` returns `["deepseek-chat", "deepseek-coder"]`; `:788` checks membership.
   A model the provider ships after release is rejected as invalid. D-13 addresses this for new
   adapters only — **the shipped three keep their hardcoded lists**, since D-06 leaves them alone.

3. **`provider_factory.rs` hardcodes four sites per provider, and two of them are already wrong.**
   The match arm, the env-var name, the priority chain and the `UnknownProvider` message
   (`:16` — "Supported providers: openai, deepseek, anthropic") are maintained by hand.
   `get_default_provider` (`:123`) and `list_available_providers` (`:137`) carry **no `#[cfg]`
   guards**, so both report a provider as available whenever its API key is in the environment, even
   when its feature was compiled out. Same over-reporting class Phase 14 fixed on the capability
   flag. D-10 removes the possibility rather than patching the two functions.

4. **The facade's LLM feature flags are inert.** Root `Cargo.toml` declares `llm-openai = []`,
   `llm-anthropic = []`, `llm-deepseek = []` while line 55 pulls `paladin-llm` in with
   `features = ["openai", "anthropic", "deepseek", "mock", "vision"]` unconditionally. Every build
   compiles all three providers whatever the flags say, so the compile-time contract PROJECT.md
   documents — "unavailable adapters must fail at compile time, never at runtime" — is not the one
   that ships. D-11 repairs it and accepts the resulting default-build break.

**On the original request's wording:** it named "Meta (Llama)?" with the question mark. That
instinct was right — Llama is a model family, and the question of which host to target was the real
one. D-02 answers it with Ollama.

</specifics>

<deferred>
## Deferred Ideas

- **Gemini vision adapter** — `gemini/vision.rs` behind the `vision` feature, joining
  `openai/vision.rs` and `anthropic/vision.rs` on the Sentinel surface. Deferred by D-08 because
  vision is not part of the `LlmPort` contract PROV-02 defines. **Trigger:** a Gemini multimodal use
  case, or any phase that reopens the vision surface.
- **Migrating `openai/` and `deepseek/` onto the shared compatible core** — would remove the
  duplication finding 1 measures and give every provider one implementation of retry, streaming and
  error mapping. Deferred by D-06 to keep this phase additive against shipped, covered code.
  **Trigger:** the next phase that has reason to touch those two adapters anyway.
- **Groq, Together, Mistral, Fireworks and Bedrock as named presets** — rejected rather than
  deferred under D-03, since the generic provider already reaches them. **Trigger:** a specific
  request for a preset's curated capabilities and model list, not merely for access.
- **Widening `get_provider_name()` to `&str`** — would let the generic provider report its true
  configured name instead of D-07's shared constant. Deferred because it is a breaking public port
  change with an `api-surface` guard consequence. **Trigger:** any phase already making a breaking
  `LlmPort` change.
- **Live key-gated tests for Kimi, Qwen, Grok and Gemini** — considered under D-15 and not added;
  the `live-api-tests` flag remains available. **Trigger:** acquiring the four API keys as project
  secrets.

### Reviewed Todos (not folded)

- **"Verify local `make coverage` reproduces CI's 82.39% figure"**
  (`2026-08-13-verify-local-coverage-reproduction.md`, score 0.60) — matched this phase on the
  keyword "coverage" only. It is about reproducing a measurement locally, not about provider
  adapters. PROV-04 already binds this phase's new code to the same 82% floor without adopting the
  todo's scope.

</deferred>

---

*Phase: 17-additional-llm-provider-adapters*
*Context gathered: 2026-08-16*
