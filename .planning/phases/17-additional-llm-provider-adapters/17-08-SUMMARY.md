---
phase: 17-additional-llm-provider-adapters
plan: 08
subsystem: documentation
tags: [rust, cargo-features, llm, docs, api-surface, cargo-public-api, provider-currency]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 07)
    provides: "the completed, tested nine-provider paladin-llm crate (capability invariants, D-10 factory regression, Ollama Docker-gated suite authored) that this plan's documentation surface must now describe accurately"
provides:
  - "paladin-llm's Cargo.toml description/keywords, crate README, config.example.yml, docs/src/getting-started/configuration.md and docs/src/api-reference/feature-flags.md all name the shipped nine-provider set (openai/anthropic/deepseek/kimi/qwen/grok/ollama/gemini/openai-compatible) plus mock -- closing PROV-04's documentation-currency half"
  - "config.example.yml gained a full llm: block (absent before this plan) with one sub-block per provider, every api_key using ${ENV_VAR} indirection, Ollama's block carrying no api_key key at all, openai-compatible's base_url/default_model marked required"
  - ".project/current-exports.txt regenerated via the same invocation the CI api-surface job uses; ./scripts/check-api-surface.sh exits 0 against it"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Crate description names the shape of a provider set (\"major hosted providers plus a generic OpenAI-compatible adapter\") rather than enumerating members, so it does not go stale every time a provider is added -- the README carries the authoritative enumerable list instead"

key-files:
  created:
    - .planning/phases/17-additional-llm-provider-adapters/17-08-SUMMARY.md
  modified:
    - crates/paladin-llm/Cargo.toml
    - crates/paladin-llm/README.md
    - config.example.yml
    - docs/src/getting-started/configuration.md
    - docs/src/api-reference/feature-flags.md
    - .project/current-exports.txt

key-decisions:
  - "keywords replaced wholesale (crates.io caps at five, the existing five were already spent): kept ai + llm as category anchors, added openai-compatible (the newest, most search-differentiating capability -- the generic escape valve for any endpoint not named), gemini (the crate's only bespoke non-OpenAI-compatible-protocol vendor, most likely to be searched by name), and agents (categorization synergy with the workspace's actual orchestration purpose). Dropped literal per-vendor keywords openai/anthropic/deepseek since a 5-slot keyword list cannot enumerate nine providers without misrepresenting completeness -- exactly the debt this task exists to close."
  - "config.example.yml's openai-compatible block does NOT include a capability declaration, contrary to the plan's action text (\"should show at least one capability declaration\"). Verified against crates/paladin-llm/src/config/llm.rs and config/bridge.rs: LlmProviderConfig (the YAML-facing shape) carries no capabilities field at all -- OpenAiCompatibleCapabilitiesConfig exists as a standalone type for \"a future config-file loader\" per its own module doc, not yet wired into LlmConfig/LlmProviderConfig. Capability declaration is only reachable via OPENAI_COMPATIBLE_SUPPORTS_* environment variables today. Documented as a comment in the config block and in the README rather than inventing a YAML field the deserializer does not accept."
  - "Task 2's acceptance criterion \"git diff | grep '^+' | grep -c 'paladin_llm'\" returns 0, not >=1, and is a plan-authoring assumption mismatch (same class of issue 17-07 documented for its own doc-table acceptance criterion). cargo-public-api's simplified format renders a `pub use` re-export as only its target path (`pub use paladin::AnthropicAdapter`), never the qualified source path -- confirmed by checking the pre-existing baseline's other `pub use paladin::LlmProviderFactory` line, which also carries no `paladin_llm::` substring despite `LlmProviderFactory` being a paladin_llm type. Substituted an equivalent check: both new types (AnthropicAdapter/AnthropicConfig, DeepSeekAdapter/DeepSeekConfig) are confirmed paladin_llm-crate-sourced by cross-referencing src/lib.rs:183/186's `pub use paladin_llm::anthropic::{...}` / `pub use paladin_llm::deepseek::{...}` re-export lines."
  - "The exports diff is not perfectly additive at the raw-file level: 2 lines change beyond the 4 new pub-use lines -- the regenerated timestamp header and the trailing 'Total public items' count. Neither is a real API surface change; check-api-surface.sh's own comparison filters both out before diffing (grep -v on exactly those two line shapes), which is why ./scripts/check-api-surface.sh reports the surface unchanged/pass even though git diff shows 2 deletions. Recorded per the acceptance criterion's own instruction to explain any non-purely-additive discrepancy before committing."

requirements-completed: [PROV-03]
# PROV-04 intentionally NOT listed as complete: Task 3 (the human currency checkpoint) is the
# gate for PROV-04's "advertised surface matches shipped surface" truth and has not run yet in
# this session. See ## CHECKPOINT section below.

coverage:
  - id: D1
    description: "paladin-llm's Cargo.toml description/keywords, README, config.example.yml, and the two docs/src pages under this plan's files_modified all name the shipped nine-provider set with no provider omitted and none invented"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "for p in kimi qwen grok ollama gemini openai-compatible; do for f in crates/paladin-llm/README.md config.example.yml docs/src/api-reference/feature-flags.md; do grep -qi \"$p\" \"$f\" || echo MISSING; done; done -> scan complete, 0 MISSING lines"
        status: pass
      - kind: other
        ref: "grep -c 'DeepSeek, and mock' crates/paladin-llm/Cargo.toml -> 0; keywords array has 5 entries all <=20 chars; MOONSHOT_API_KEY/DASHSCOPE_API_KEY/XAI_API_KEY/GEMINI_API_KEY each grep >=1 in README.md; config.example.yml parses as valid YAML; zero literal-looking api_key values; llm-gemini and llm-openai-compatible both named in feature-flags.md; cargo check -p paladin-llm exits 0"
        status: pass
    human_judgment: true
    rationale: "The plan's Task 3 checkpoint explicitly requires a human to read the Cargo.toml [features] block, description/keywords, README, and root Cargo.toml llm-* flags side by side and confirm no provider is named that does not exist and none is omitted -- 'the is the reasoning sound reading no test can perform' framing the plan itself uses. All automated acceptance criteria pass, but the plan's own must_haves list this specific human confirmation as a required truth, and that checkpoint has not yet run in this session."
  - id: D2
    description: ".project/current-exports.txt regenerated so the api-surface CI guard passes with the phase's new public types included; compat wire types confirmed crate-private"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "./scripts/check-api-surface.sh -> API surface unchanged / exit 0 against the regenerated baseline; git diff .project/current-exports.txt shows exactly 4 new pub-use lines (AnthropicAdapter, AnthropicConfig, DeepSeekAdapter, DeepSeekConfig) plus 2 boilerplate (timestamp/count) changes; grep -cE 'CompatRequest|CompatMessage|CompatStreamDelta' over added lines -> 0; grep for the dot-less project/current-exports.txt path in both scripts -> 0 (DEBT-01 held)"
        status: pass
    human_judgment: false
  - id: D3
    description: "The provider-selection study (.planning/decisions/0045-additional-llm-provider-selection.md) reads as sound reasoning -- criteria before verdicts, Kimi/Gemini/Qwen/Llama each dispositioned, Llama routed to Ollama as its host, Groq/Together/Mistral/Fireworks/Bedrock rejected as already-covered rather than deferred"
    requirement: "PROV-01"
    verification: []
    human_judgment: true
    rationale: "This is explicitly the 'is the reasoning sound' reading the plan's checkpoint states no automated test can perform. Pending Task 3."
  - id: D4
    description: "The default-build provider-set change described in CHANGELOG.md is the outcome selected at plan 17-06's checkpoint, and is one the human accepts shipping"
    requirement: "PROV-04"
    verification: []
    human_judgment: true
    rationale: "Pending Task 3. See the CHECKPOINT section below for an important correction to this checkpoint's own literal wording: as authored, Task 3 asks the human to confirm 'a default build of paladin-ai no longer compiles Anthropic and DeepSeek in' -- that describes an EARLIER shape of D-11 that was subsequently amended to option-b (commit f9abcb6, 'preserve the default provider set'). The actual, currently-committed CHANGELOG.md [Unreleased] entry and root Cargo.toml both state the opposite: the default build compiles the SAME three providers as before (openai+anthropic+deepseek), unchanged, no consumer action required. The human should verify against the current CHANGELOG.md/Cargo.toml text, not the plan's now-stale paraphrase of it."

# Metrics
duration: ~50min (Tasks 1-2, this session; Task 3 pending)
completed: 2026-08-17
status: checkpoint
---

# Phase 17 Plan 08: Advertised-Surface Currency Summary

**Cargo.toml description/keywords, the paladin-llm README, config.example.yml, and both configuration docs pages now name the shipped nine-provider set (openai/anthropic/deepseek/kimi/qwen/grok/ollama/gemini/openai-compatible), and the public-API baseline is regenerated and green -- Tasks 1 and 2 complete; Task 3, the phase's final human currency-check checkpoint, has not yet run.**

## Performance

- **Duration:** ~50 min (Tasks 1-2)
- **Completed (Tasks 1-2):** 2026-08-17
- **Tasks:** 2 of 3 executed (Task 3 is a blocking human-verify checkpoint, gate="blocking")
- **Files modified:** 6

## Accomplishments

- **Task 1: advertised surface brought in line with the shipped provider set.**
  - `crates/paladin-llm/Cargo.toml`: replaced the stale `"...OpenAI, Anthropic, DeepSeek, and mock"` description with a shape-not-enumeration description, and replaced the five-slot `keywords` array (`ai`, `llm`, `openai-compatible`, `gemini`, `agents` -- see key-decisions for the one-line justification per entry).
  - `crates/paladin-llm/README.md`: full provider table (feature flag / credential env var / default endpoint) for all nine providers plus `mock`, a dedicated section on the generic `openai-compatible` adapter explaining what it's for and pointing readers there when their provider isn't named, and an explicit `OPENAI_COMPATIBLE_API_KEY` vs `OPENAI_API_KEY` warning (the near-collision the human accepted at the wave-3 checkpoint, per this plan's promise that the docs make the distinction unmissable).
  - `config.example.yml`: added a structured `llm:` block (previously entirely absent from this file -- only the legacy top-level `llm_type`/`llm_url`/`llm_api_key` fields existed) with one sub-block per provider using `LlmConfig`'s real field names, hyphenated `openai-compatible` key included. Every `api_key` uses `${ENV_VAR}` indirection naming the real variable (`${MOONSHOT_API_KEY}`, `${DASHSCOPE_API_KEY}`, `${XAI_API_KEY}`, `${GEMINI_API_KEY}`, `${OPENAI_COMPATIBLE_API_KEY}`). Ollama's block carries no `api_key` key at all with a comment explaining D-12. `openai-compatible`'s `base_url`/`default_model` are commented as required with no vendor fallback.
  - `docs/src/getting-started/configuration.md`: extended the `llm:` YAML example and the API-key table to all nine providers.
  - `docs/src/api-reference/feature-flags.md`: extended the LLM Provider Flags table, the "Default vs. Full" table, the "Default Configuration" section, and the feature dependency tree to the full nine-provider set; corrected two pre-existing stale claims found while grepping for "deepseek" (a "Current Default (as of v0.5.0): llm-openai only" section, and a "Default: llm-openai only" table row) to state the actual current default (`openai`+`anthropic`+`deepseek`, three providers, unchanged by the `llm-*` flag rewiring per `CHANGELOG.md`).
  - All Task 1 automated acceptance criteria verified passing (see `coverage: D1`).

- **Task 2: public-API baseline regenerated.**
  - Ran `scripts/extract-public-api.sh` with the exact invocation `.github/workflows/ci.yml`'s `api-surface` job uses (no explicit `--features`/`--no-default-features` flag; the crate's own default features apply, matching the CI job byte-for-byte).
  - Diff is 4 new `pub use` lines: `AnthropicAdapter`, `AnthropicConfig`, `DeepSeekAdapter`, `DeepSeekConfig`. These surfaced under default features because `src/lib.rs:183/186` gates their re-export behind `#[cfg(feature = "llm-anthropic")]` / `#[cfg(feature = "llm-deepseek")]`, and those two facade flags moved from empty inert stubs into the real `default = [...]` set as part of this phase's `llm-*` rewiring (ADR-0046) plus D-11's amendment (option-b) that kept the default provider *count and names* unchanged. No new provider from this phase (kimi/qwen/grok/ollama/gemini/openai-compatible) appears in the baseline, because none of them is in the compiled default feature set -- exactly the D-11-amended shape.
  - `./scripts/check-api-surface.sh` exits 0 against the regenerated baseline.
  - Compat wire types (`CompatRequest`/`CompatMessage`/`CompatStreamDelta`) confirmed absent from the added lines -- still crate-private.
  - DEBT-01 held: zero dot-less `project/current-exports.txt` references remain in either script.
  - See key-decisions for two documented deviations from the plan's literal acceptance criteria (the `paladin_llm` substring check, and the 2 non-additive boilerplate lines).

## Task Commits

1. **Task 1: Bring the advertised surface in line with the shipped provider set** - `7c04dd7` (docs)
2. **Task 2: Regenerate the public-API baseline so the api-surface guard stays green** - `4e8aac9` (chore)
3. **Task 3: Human currency check** - NOT YET RUN. See `## CHECKPOINT` below.

_Plan metadata commit deferred until Task 3 resolves and the plan is fully complete._

## Files Created/Modified

- `crates/paladin-llm/Cargo.toml` - description/keywords replaced to name the shape of the shipped provider set rather than a stale subset
- `crates/paladin-llm/README.md` - full nine-provider table, openai-compatible explainer, OPENAI_COMPATIBLE_API_KEY vs OPENAI_API_KEY warning
- `config.example.yml` - new `llm:` block, one sub-block per provider, `${ENV_VAR}` credentials throughout
- `docs/src/getting-started/configuration.md` - `llm:` YAML example and API-key table extended to nine providers
- `docs/src/api-reference/feature-flags.md` - LLM Provider Flags table, Default vs. Full table, Default Configuration section, and dependency tree extended/corrected to nine providers
- `.project/current-exports.txt` - regenerated; 4 new lines, `check-api-surface.sh` green

## Decisions Made

See `key-decisions` in the frontmatter above. Most consequential: the `Cargo.toml` keyword-replacement rationale (one-line justification per entry, since the five-slot cap forced a full replacement rather than an append), and flagging that Task 3's own literal checkpoint text describes an outdated (pre-amendment) shape of D-11 -- the human verifying Task 3 should check against the CURRENT `CHANGELOG.md`/`Cargo.toml`, not the plan's paraphrase.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - plan-authoring bug, documented substitution] Task 2's `grep -c 'paladin_llm'` acceptance criterion cannot pass for re-export-shaped diff lines**
- **Found during:** Task 2, acceptance-criteria verification
- **Issue:** `cargo-public-api --simplified`'s `pub use` re-export lines never carry the qualified source-crate path (confirmed against the pre-existing `pub use paladin::LlmProviderFactory` baseline entry, itself a `paladin_llm` type with no `paladin_llm::` substring in its own line). The literal acceptance criterion `git diff ... | grep '^+' | grep -c 'paladin_llm'` therefore returns 0, not >=1, for this diff shape.
- **Fix:** Verified paladin_llm-crate provenance by cross-referencing `src/lib.rs:183/186`'s `pub use paladin_llm::anthropic::{AnthropicAdapter, AnthropicConfig}` / `pub use paladin_llm::deepseek::{DeepSeekAdapter, DeepSeekConfig}` re-export declarations instead. No content change needed -- the baseline itself is correct.
- **Files modified:** None (verification-only finding)
- **Committed in:** N/A

**2. [Documented substitution] `config.example.yml`'s `openai-compatible` block omits the plan-requested capability declaration**
- **Found during:** Task 1, reading `crates/paladin-llm/src/config/llm.rs` and `config/bridge.rs`
- **Issue:** The plan's action text asks the `openai-compatible` example block to "show at least one capability declaration." `LlmProviderConfig` (the type `config.example.yml`'s YAML actually deserializes into) carries no capabilities field at all -- `OpenAiCompatibleCapabilitiesConfig` exists as a standalone type documented as being "for a future config-file loader," not yet wired into `LlmConfig`. Adding a fictional YAML key would silently fail to deserialize and mislead an operator.
- **Fix:** Added a comment in the block and a note in the README instead, pointing to the real mechanism (`OPENAI_COMPATIBLE_SUPPORTS_*` environment variables).
- **Files modified:** `config.example.yml`, `crates/paladin-llm/README.md`
- **Committed in:** `7c04dd7`

**3. [Documented, non-blocking] Exports baseline diff carries 2 non-API boilerplate line changes alongside the 4 real additions**
- **Found during:** Task 2, sanity-checking the diff before committing
- **Issue:** The plan's acceptance criterion "`git diff | grep -c '^-[^-]'` returns 0" is not literally satisfied at the raw-file level -- the regenerated timestamp header and "Total public items" count both change (2 deletion lines in `git diff`). `check-api-surface.sh`'s own comparison filters exactly these two line shapes out before diffing, which is why the script reports success.
- **Fix:** No content change needed; documented per the acceptance criterion's own instruction ("if it is not [purely additive], the discrepancy is explained in the SUMMARY before committing").
- **Files modified:** None beyond the intended `.project/current-exports.txt` regeneration
- **Committed in:** `4e8aac9`

---

**Total deviations:** 3 documented (1 plan-authoring-bug substitution, 1 fictional-field avoidance, 1 non-additive-boilerplate note). None required architectural changes; none affected shipped behavior.
**Impact on plan:** No scope creep. All three are documentation-accuracy or verification-precision findings, consistent with this plan's own purpose.

## Known Remaining Surface (outside this plan's files_modified)

Per the plan's own instruction ("If a provider set is enumerated somewhere this task did not look, record that in the SUMMARY as a known remaining surface rather than silently leaving it"), grepping beyond the five files this plan edits found additional stale three-provider enumerations NOT touched here:

- `README.md` (repo root) -- lines 16, 65, 82 name only OpenAI/Anthropic/DeepSeek.
- `docs/src/appendix/provider-expansion.md` -- an entire "LLM Provider Expansion Guide" with a full comparison table covering only the original three providers.
- `docs/src/architecture/crate-map.md` -- a feature-flag table row and adapter list naming only `llm-deepseek`/`deepseek`.
- `docs/src/contributing/contributing-providers.md`, `docs/src/api-reference/stable-api.md` -- not inspected in detail; flagged by filename as likely candidates for the same debt.

None of these were in this plan's `files_modified` frontmatter and none were edited. This is the same class of currency debt DOCS-01 exists to close generally; it is out of this plan's scope but should not be silently missed.

## Issues Encountered

None blocking. See Deviations above for documented findings during execution.

## User Setup Required

None - no external service configuration required for Tasks 1-2. Task 3 requires human review (see CHECKPOINT below), not external service setup.

## Next Phase Readiness

**Blocked on Task 3 (this plan's own final task) before phase 17 can be considered complete.** Tasks 1 and 2 are fully committed and their automated acceptance criteria all pass. The remaining work is exclusively the human currency-check checkpoint this plan's Task 3 defines -- see `## CHECKPOINT REACHED` in the executor's return message for the full structured state.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed (Tasks 1-2): 2026-08-17*

## Self-Check: PASSED

- FOUND: `crates/paladin-llm/Cargo.toml`
- FOUND: `crates/paladin-llm/README.md`
- FOUND: `config.example.yml`
- FOUND: `docs/src/getting-started/configuration.md`
- FOUND: `docs/src/api-reference/feature-flags.md`
- FOUND: `.project/current-exports.txt`
- FOUND commit `7c04dd7` (Task 1)
- FOUND commit `4e8aac9` (Task 2)
