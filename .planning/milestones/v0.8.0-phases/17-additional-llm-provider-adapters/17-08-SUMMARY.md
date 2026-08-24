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
  - "Human sign-off (Task 3) confirming the advertised surface matches the shipped surface (PROV-04), ADR-0045's reasoning reads as sound (PROV-01), and the default-build outcome (D-11 option-b, no break) is accepted -- phase 17's final gate is now closed"
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
  - "Task 3 (human currency checkpoint) resolved APPROVED. Provenance: the human user approved via an interactive AskUserQuestion prompt raised by the /gsd-execute-phase 17 orchestrator on 2026-08-17, after the orchestrator independently performed and reported the mechanical portions of the verification (nine-provider parity across modules/features/facade-flags, ADR-0045 structure, default-build outcome, OPENAI_COMPATIBLE_API_KEY distinction). Recorded here as a human approval obtained at that checkpoint, not as this executor's self-assessment."
  - "17-08-PLAN.md's own Task 3 prose (\"a default build of paladin-ai no longer compiles Anthropic and DeepSeek in\") is stale: it describes the SUPERSEDED option-a shape of D-11. The human amended D-11 to option-b (preserve the default provider set) in commit f9abcb6, before this phase executed. The 17-07-SUMMARY.md continuation agent and this plan's own prior-session partial SUMMARY both independently flagged this. The human explicitly chose NOT to amend the already-executed plan document; this SUMMARY notes the discrepancy instead, per that instruction. There is no default-build break to accept -- the CHANGELOG [Unreleased] entry and root Cargo.toml default set (openai+anthropic+deepseek) both confirm the default build is unchanged from before this phase."

requirements-completed: [PROV-01, PROV-02, PROV-03, PROV-04]

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
    rationale: "The plan's Task 3 checkpoint requires a human to read the Cargo.toml [features] block, description/keywords, README, and root Cargo.toml llm-* flags side by side and confirm no provider is named that does not exist and none is omitted. RESOLVED: human approved via AskUserQuestion on 2026-08-17, after the orchestrator confirmed 1:1 correspondence across nine provider modules, nine paladin-llm features, and nine llm-* facade flags plus llm-all -- nothing advertised without a shipping counterpart, nothing shipped left undocumented."
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
    verification:
      - kind: other
        ref: "cat .planning/decisions/0045-additional-llm-provider-selection.md -> \"### Criteria (recorded before scoring)\" at line 45 precedes \"## Decision\" at line 67, confirming criteria genuinely precede verdicts"
        status: pass
    human_judgment: true
    rationale: "This is explicitly the 'is the reasoning sound' reading the plan's checkpoint states no automated test can perform. RESOLVED: human approved via AskUserQuestion on 2026-08-17, after the orchestrator confirmed the structural ordering (criteria at line 45, decision at line 67)."
  - id: D4
    description: "The default-build provider-set change described in CHANGELOG.md is the outcome selected at plan 17-06's checkpoint, and is one the human accepts shipping"
    requirement: "PROV-04"
    verification:
      - kind: other
        ref: "CHANGELOG.md [Unreleased] states the default build compiles the same three providers as before (openai, anthropic, deepseek), matching root Cargo.toml's default = [\"llm-openai\", \"llm-anthropic\", \"llm-deepseek\"] -- there is no default-build break to accept, contrary to Task 3's own stale prose (see key-decisions)"
        status: pass
    human_judgment: true
    rationale: "RESOLVED: human approved via AskUserQuestion on 2026-08-17. Important correction preserved for the record: Task 3's own literal checkpoint text asks the human to confirm 'a default build of paladin-ai no longer compiles Anthropic and DeepSeek in' -- that describes an EARLIER, SUPERSEDED shape of D-11 (option-a). D-11 was amended to option-b (preserve the default provider set) in commit f9abcb6, before this phase executed. The human's approval was obtained against the CURRENT, correct CHANGELOG.md/Cargo.toml state (default set unchanged), not against the plan's stale paraphrase. The human explicitly chose not to edit the already-executed plan document; this discrepancy is recorded here instead so a future auditor sees the correct picture without the plan being rewritten after the fact."

# Metrics
duration: ~50min (Tasks 1-2, prior session) + ~10min (Task 3 close-out, this continuation)
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 08: Advertised-Surface Currency Summary

**Cargo.toml description/keywords, the paladin-llm README, config.example.yml, and both configuration docs pages now name the shipped nine-provider set (openai/anthropic/deepseek/kimi/qwen/grok/ollama/gemini/openai-compatible); the public-API baseline is regenerated and green; and a human has confirmed, side by side, that the advertised surface matches the shipped surface, that ADR-0045's provider-selection reasoning is sound, and that the default-build outcome (D-11 option-b, no break) is accepted. All three tasks complete -- this is phase 17's final plan and final gate.**

## Performance

- **Duration:** ~50 min (Tasks 1-2, prior session) + ~10 min (Task 3 close-out, this continuation)
- **Completed:** 2026-08-17
- **Tasks:** 3 of 3 executed. Task 3 (blocking human-verify checkpoint) resolved APPROVED.
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

- **Task 3: human currency check -- RESOLVED APPROVED.**
  - Type: `human-verify`, gate `blocking`. This was the phase's final gate (all three of 17-VALIDATION.md's manual-only verifications).
  - The orchestrator gathered mechanical evidence and presented it to the human via an interactive `AskUserQuestion` prompt on 2026-08-17:
    1. **Advertised vs shipped -- exact 1:1 correspondence.** Nine provider modules on disk (`anthropic`, `deepseek`, `gemini`, `grok`, `kimi`, `ollama`, `openai`, `openai_compatible`, `qwen`); nine `paladin-llm` provider features; nine facade `llm-*` flags plus `llm-all`. Nothing advertised without a shipping counterpart; nothing shipped left undocumented.
    2. **ADR-0045 structure sound.** `### Criteria (recorded before scoring)` at line 45 precedes `## Decision` at line 67 -- criteria genuinely precede verdicts.
    3. **Default-build outcome accepted.** CHANGELOG `[Unreleased]` states the default build compiles the same three providers as before (`openai`, `anthropic`, `deepseek`), matching root `Cargo.toml`'s `default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`. There is no default-build break to accept.
    4. **`OPENAI_COMPATIBLE_API_KEY` vs `OPENAI_API_KEY` distinction** -- the condition the human attached to their wave-3 naming choice -- is satisfied by a README callout, a note in `docs/src/getting-started/configuration.md`, and an inline comment in `config.example.yml`.
  - **Human response: "approved."**
  - **Known stale plan prose, recorded not acted on:** Task 3's own written text in `17-08-PLAN.md` claims "a default build of paladin-ai no longer compiles Anthropic and DeepSeek in." That describes the SUPERSEDED option-a shape of D-11; the human amended D-11 to option-b (preserve the default provider set) in commit `f9abcb6`, before this phase executed. The human explicitly chose NOT to amend the already-executed plan document -- `17-08-PLAN.md` was not edited by this close-out. This discrepancy is recorded here (and was recorded in the prior-session partial SUMMARY, and independently in 17-07-SUMMARY.md) so a future auditor sees the correct, current picture without the executed plan being rewritten after the fact.

## Task Commits

1. **Task 1: Bring the advertised surface in line with the shipped provider set** - `7c04dd7` (docs)
2. **Task 2: Regenerate the public-API baseline so the api-surface guard stays green** - `4e8aac9` (chore)
3. **Task 3: Human currency check** - resolved APPROVED; no code changes (verification-only task). Outcome recorded in this SUMMARY per the plan's own `<verification>` instruction ("do not commit anything for Task 3 beyond recording the outcome in the SUMMARY").

## Files Created/Modified

- `crates/paladin-llm/Cargo.toml` - description/keywords replaced to name the shape of the shipped provider set rather than a stale subset
- `crates/paladin-llm/README.md` - full nine-provider table, openai-compatible explainer, OPENAI_COMPATIBLE_API_KEY vs OPENAI_API_KEY warning
- `config.example.yml` - new `llm:` block, one sub-block per provider, `${ENV_VAR}` credentials throughout
- `docs/src/getting-started/configuration.md` - `llm:` YAML example and API-key table extended to nine providers
- `docs/src/api-reference/feature-flags.md` - LLM Provider Flags table, Default vs. Full table, Default Configuration section, and dependency tree extended/corrected to nine providers
- `.project/current-exports.txt` - regenerated; 4 new lines, `check-api-surface.sh` green

## Decisions Made

See `key-decisions` in the frontmatter above. Most consequential: the `Cargo.toml` keyword-replacement rationale (one-line justification per entry, since the five-slot cap forced a full replacement rather than an append); the documented, provenance-recorded human approval of Task 3's checkpoint; and the explicit record that Task 3's own literal checkpoint text describes an outdated (pre-amendment) shape of D-11 -- the human's approval was obtained against the CURRENT `CHANGELOG.md`/`Cargo.toml` text, not the plan's stale paraphrase, and the plan document itself was deliberately left unedited per the human's own choice.

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

**4. [Documented, plan prose stale -- not edited] Task 3's own checkpoint text describes a superseded shape of D-11**
- **Found during:** Task 3 verification (both the prior-session partial SUMMARY and this close-out)
- **Issue:** `17-08-PLAN.md`'s Task 3 `<how-to-verify>` step 3 asks the human to confirm "a default build of paladin-ai no longer compiles Anthropic and DeepSeek in" -- describing the SUPERSEDED option-a shape of D-11. D-11 was amended to option-b (preserve the default provider set) in commit `f9abcb6`, before this phase executed.
- **Fix:** No plan edit made -- the human explicitly chose not to amend the already-executed plan document. The correct, current state (CHANGELOG `[Unreleased]` and root `Cargo.toml`'s `default = [...]` both confirm the default build is unchanged) was surfaced to and accepted by the human at the checkpoint, and is recorded here for a future auditor.
- **Files modified:** None. `17-08-PLAN.md` was NOT edited (explicit human instruction).
- **Committed in:** N/A (documentation-only finding, recorded in this SUMMARY)

---

**Total deviations:** 4 documented (1 plan-authoring-bug substitution, 1 fictional-field avoidance, 1 non-additive-boilerplate note, 1 stale-plan-prose record). None required architectural changes; none affected shipped behavior.
**Impact on plan:** No scope creep. All four are documentation-accuracy or verification-precision findings, consistent with this plan's own purpose.

## Known Remaining Surface (outside this plan's files_modified)

Per the plan's own instruction ("If a provider set is enumerated somewhere this task did not look, record that in the SUMMARY as a known remaining surface rather than silently leaving it"), grepping beyond the five files this plan edits found additional stale three-provider enumerations NOT touched here:

- `README.md` (repo root) -- lines 16, 65, 82 name only OpenAI/Anthropic/DeepSeek.
- `docs/src/appendix/provider-expansion.md` -- an entire "LLM Provider Expansion Guide" with a full comparison table covering only the original three providers.
- `docs/src/architecture/crate-map.md` -- a feature-flag table row and adapter list naming only `llm-deepseek`/`deepseek`.
- `docs/src/contributing/contributing-providers.md`, `docs/src/api-reference/stable-api.md` -- not inspected in detail; flagged by filename as likely candidates for the same debt.

None of these were in this plan's `files_modified` frontmatter and none were edited. This is the same class of currency debt DOCS-01 exists to close generally; it is out of this plan's scope but should not be silently missed.

## Issues Encountered

None blocking. Task 3's checkpoint is resolved (approved). See Deviations above for documented findings during execution.

## User Setup Required

None. Task 3's human review is complete (approved 2026-08-17).

## Broken-Windows Ledger

No new ledger entries from this plan. This close-out did not introduce any new stub, skipped test, unrun `<verify>`, or deviation beyond the four already documented above and folded into this SUMMARY (none of which meet the ledger's bar for a separate `.planning/WINDOWS.md` row -- they are plan-authoring/acceptance-criteria precision notes, not shipped defects).

## Phase-Closing Honesty Ledger (carried forward from 17-07, unresolved)

This is phase 17's final plan and final SUMMARY. The following verification debt from 17-07 remains genuinely open and is **not** resolved by this plan (Task 3's checkpoint scope was documentation currency and the two other manual-only ROADMAP verifications, not these three items):

- **`.planning/WINDOWS.md` id 12** -- The Ollama Docker-gated Tier 2 integration suite (`tests/integration/ollama_docker_test.rs`, authored in 17-07) was authored and proven to compile, lint clean, and skip gracefully, but has **never run against a real Ollama server** -- no Docker daemon was available in any sandbox this phase executed in.
- **`.planning/WINDOWS.md` id 13** -- The 82% workspace line-coverage floor (ADR-0006) is **UNMEASURED**, not estimated or failing -- `make coverage` requires Redis and MinIO via Docker, unavailable in every sandbox this phase executed in.
- **`.planning/WINDOWS.md` id 14** -- The `ollama-test` service's healthcheck substitutes `ollama list` for the plan's originally preferred curl-based `/v1/models` check (curl/wget availability in the base image could not be verified without Docker); the substitution itself is unverified against the real image.
- **Vendor facts for Kimi/Qwen/Grok/Gemini** (base URLs, default/fallback model IDs) were taken from vendor documentation but never verified against a live endpoint -- no network egress in any sandbox this phase executed in (17-RESEARCH.md Assumptions Log A1-A5).

**A human with Docker access and/or live provider credentials must close these before treating phase 17 as fully verified end-to-end**, per 17-07-SUMMARY.md's own "User Setup Required" section. This plan's Task 3 approval covers documentation currency, the ADR-0045 reasoning quality, and the default-build outcome -- it does not and cannot substitute for running the Docker-gated suite or measuring coverage.

## Next Phase Readiness

**Phase 17 (additional LLM provider adapters) is complete as a documentation/gate matter: all 8 plans across 7 waves executed, this plan's Task 3 (the phase's final blocking checkpoint) is resolved APPROVED.** The four items in the Phase-Closing Honesty Ledger above are genuine, tracked, open verification debt (`.planning/WINDOWS.md` ids 12-14, plus the unverified vendor facts) that a human with Docker/network access should close before treating the nine-provider surface as fully proven in production. Nothing in that debt blocks phase closure per this plan's own scope -- it was correctly deferred and documented by 17-07, not silently dropped.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*

## Self-Check: PASSED

- FOUND: `crates/paladin-llm/Cargo.toml`
- FOUND: `crates/paladin-llm/README.md`
- FOUND: `config.example.yml`
- FOUND: `docs/src/getting-started/configuration.md`
- FOUND: `docs/src/api-reference/feature-flags.md`
- FOUND: `.project/current-exports.txt`
- FOUND commit `7c04dd7` (Task 1)
- FOUND commit `4e8aac9` (Task 2)
