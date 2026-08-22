---
phase: 17-additional-llm-provider-adapters
plan: 21
subsystem: api
tags: [rust, llm, qwen, dashscope, alibaba, compat-engine, live-verification, auth-gate]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: CompatRequestParameters (17-18) and the option-(a) precedent (17-19) — the mechanism this plan's blocked half would have reused for any rejected Qwen sampling parameter
provides:
  - "QWEN_DEFAULT_BASE_URL reversed to the US (Virginia) compatible-mode endpoint, live-verified 2026-08-22 (92 models, qwen-plus present), per the developer's binding decision D-17-02"
  - "The region-scoped-credential rule, the three known compatible-mode regional endpoints, and the mandatory-override consequence for Singapore/mainland operators, recorded in the module rustdoc at the DASHSCOPE_BASE_URL override site"
  - "The reversal of the earlier 'QWEN_DEFAULT_BASE_URL MUST NOT be changed' prohibition recorded in code, with the two-endpoint measurement that falsified it, so it cannot be silently re-derived"
  - "CHANGELOG.md Unreleased entry announcing the changed default to upgrading operators"
  - "A NEW live-measured finding not previously recorded anywhere: the available DASHSCOPE_API_KEY has no chat-completion invocation entitlement for ANY model in the US (Virginia) workspace, despite full read access to the model catalog — task 2 is blocked on this until a human activates model access"
affects: [17-22 (diagnosability of exactly this failure mode — an entitled-but-unauthorized key currently looks identical to an offline vendor in available_models()), 17-20 (docs currency once task 2's live parameter verdicts exist)]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reversal-of-a-prohibition recorded in the same rustdoc the constant lives in, naming the falsified argument, why it was wrong, and the measurement that settled it — so a later reader who re-derives the old argument meets the counter-evidence in the same place, not just in a planning artifact"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/qwen/adapter.rs
    - CHANGELOG.md

key-decisions:
  - "Task 1 executed exactly as planned: QWEN_DEFAULT_BASE_URL is now https://dashscope-us.aliyuncs.com/compatible-mode/v1, pinned by a literal-value test that failed against the old Singapore default before the change"
  - "Task 2 HALTED rather than partially completed. The plan's own instruction governs: 'If live access fails and you cannot satisfy a must_have, HALT and report it plainly rather than substituting documented or snapshot values.' Refreshing QWEN_FALLBACK_MODELS from the live catalog alone (which IS achievable without generate() working) was deliberately NOT done, because task 2's done criteria bundle model refresh together with live-measured parameter verdicts and a PASS on the generate probe — landing only the model-ID half would leave the capabilities rustdoc claiming 'not live-verified' language that a partial, inconsistent commit could not honestly resolve either way."
  - "The 403 Model.AccessDenied response is treated as an authentication/authorization gate per the executor's <authentication_gates> protocol (403 is an explicitly listed indicator), not as a code defect to auto-fix under Rules 1-3 — no code change can grant an Alibaba Cloud account invocation entitlement it does not have."

patterns-established: []

requirements-completed: []

# Coverage metadata
coverage:
  - id: D1
    description: "QWEN_DEFAULT_BASE_URL names the US (Virginia) compatible-mode endpoint, pinned by a test that failed against the previous Singapore value before the change"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/qwen/adapter.rs#qwen_config_defaults_to_the_us_virginia_endpoint_by_literal"
        status: pass
    human_judgment: false
  - id: D2
    description: "The module rustdoc states the region-scoped-credential rule, names the three known regional endpoints, states the mandatory-override consequence for Singapore/mainland operators, and records the reversal with its two-endpoint measurement"
    requirement: "PROV-04"
    verification:
      - kind: manual_procedural
        ref: "crates/paladin-llm/src/qwen/adapter.rs module doc comment, 'Region default' and 'Reversal record' sections"
        status: pass
    human_judgment: true
    rationale: "Rustdoc content quality (does it say what the plan requires, findably) is a documentation judgment, not something a unit test asserts"
  - id: D3
    description: "CHANGELOG.md announces the changed default under Unreleased, naming what a Singapore or mainland operator must now set"
    requirement: "PROV-04"
    verification:
      - kind: manual_procedural
        ref: "CHANGELOG.md ## [Unreleased] ### Changed entry"
        status: pass
    human_judgment: false
  - id: D4
    description: "Qwen's live model-list probe PASSES at the shipped default with no DASHSCOPE_BASE_URL override present (92 models, differs from curated fallback, qwen-plus present)"
    requirement: "PROV-02"
    verification:
      - kind: integration
        ref: "crates/paladin-llm/examples/live_vendor_smoke.rs (live run, 2026-08-22, verbatim output below)"
        status: pass
    human_judgment: false
  - id: D5
    description: "Qwen's live generate() probe with default prompt parameters, at the shipped default, with per-parameter verdicts declared on the preset for anything rejected"
    requirement: "PROV-02"
    verification: []
    human_judgment: true
    rationale: "BLOCKED — every model in the live catalog (78 qwen-prefixed identifiers plus non-Qwen models hosted on the same DashScope workspace) returns HTTP 403 Model.AccessDenied on both the OpenAI-compatible chat/completions endpoint and the native DashScope generation endpoint, for the same credential that successfully lists models. This is an account/workspace entitlement gap in the Alibaba Model Studio console, not a code defect or a stale model identifier — no model substitution or code change resolves it. Requires human action (see Authentication Gate section) before this deliverable can be verified."
  - id: D6
    description: "Kimi, Grok and Gemini still PASS both live probes after this plan's changes (regression control)"
    requirement: null
    verification:
      - kind: integration
        ref: "crates/paladin-llm/examples/live_vendor_smoke.rs (live run, 2026-08-22: Kimi PASS/PASS, Grok PASS/PASS, Gemini PASS/PASS)"
        status: pass
    human_judgment: false

# Metrics
duration: ~55min
completed: 2026-08-22
status: blocked
---

# Phase 17 Plan 21: Qwen default base_url reversal to US Virginia — Task 1 complete, Task 2 blocked on a live account-entitlement gate

**`QWEN_DEFAULT_BASE_URL` is now the US (Virginia) DashScope compatible-mode endpoint (reversing the earlier, now-falsified Singapore default), live-verified via a 92-model catalog fetch — but the live `generate()` probe cannot be measured because the available credential has no chat-completion invocation entitlement for any model in that workspace, an account-side gate no code change can resolve.**

## Performance

- **Duration:** ~55 min
- **Completed:** 2026-08-22
- **Tasks:** 1 of 2 completed and committed; task 2 halted at an authentication/authorization gate
- **Files modified:** 2 (`crates/paladin-llm/src/qwen/adapter.rs`, `CHANGELOG.md`)

## Accomplishments

- **Task 1 (complete):** Wrote a failing test (`qwen_config_defaults_to_the_us_virginia_endpoint_by_literal`) asserting the literal Virginia URL, confirmed it failed against the shipped Singapore default (RED), then changed `QWEN_DEFAULT_BASE_URL` to `https://dashscope-us.aliyuncs.com/compatible-mode/v1` and confirmed the test passes (GREEN).
- Rewrote the module's "Region default" rustdoc to state Alibaba's region-scoped-credential rule in Alibaba's terms, the three known compatible-mode regional endpoints in a table, the mandatory-override consequence for Singapore/mainland operators, and what a mismatch looks like today (a plausible three-entry fallback list, not an error).
- Added a "Reversal record" rustdoc section on the same constant recording the falsified argument ("a well-formed 401 proves the URL"), why it does not hold (region-scoped keys return that same envelope from every endpoint but their own), and the exact two-endpoint measurement (Singapore: 3 models byte-identical to fallback; Virginia: 92 live models) that settled it.
- Added a `CHANGELOG.md` `### Changed` entry under `## [Unreleased]` announcing the moved default and the required remedy for Singapore/mainland operators.
- Ran the live harness with the shipped defaults (`DASHSCOPE_BASE_URL` confirmed unset) and confirmed: Qwen's model-list probe now PASSES (92 models, `qwen-plus` present, differs from the curated fallback) — resolving the first half of G-17-4d's live evidence. Kimi, Grok and Gemini all still PASS both probes (no regression).
- **Task 2 discovery, not fixable in code:** the live `generate()` probe fails for `qwen-plus` with `HTTP 403 {"code":"Model.AccessDenied","message":"Model access denied."}`. Investigated exhaustively before concluding this is an entitlement gate, not a stale-model-ID problem (see "Authentication Gate" below).

## Task Commits

1. **Task 1: The shipped default names a region the credential can reach, and the reversal goes on the record** — `8208dec` (fix)

Task 2 was not committed — halted at the authentication gate before any adapter.rs change was made for it, per the plan's own instruction not to substitute snapshot/guessed values when live access fails a must-have.

## Files Created/Modified

- `crates/paladin-llm/src/qwen/adapter.rs` — `QWEN_DEFAULT_BASE_URL` changed to the Virginia endpoint; module rustdoc rewritten with the region table, mandatory-override consequence, and reversal record; one new unit test pinning the literal endpoint.
- `CHANGELOG.md` — new `### Changed` entry under `## [Unreleased]` announcing the default change.

## Decisions Made

- **Task 1 executed exactly as the plan specified** — no deviation.
- **Task 2 halted rather than partially landed.** The plan explicitly instructs: "If live access fails and you cannot satisfy a must_have, HALT and report it plainly rather than substituting documented or snapshot values." Refreshing `QWEN_FALLBACK_MODELS` from the live catalog alone was deliberately not done even though it is technically achievable (the `/models` list works fine) — task 2's done criteria bundle the model refresh together with live-measured per-parameter verdicts and a PASS on the generate probe, all of which require a working `generate()` call this credential cannot make. Landing only the model-ID half would leave the capabilities rustdoc's "not live-verified" language unresolved either way, and would not move the plan's actual must-have (Qwen PASS on both probes) any closer to true.
- **The 403 is treated as an authentication gate, not a bug.** `403` is one of the indicators explicitly listed in the executor's `<authentication_gates>` protocol. No amount of code change — different model ID, different endpoint shape, different request body — can grant an Alibaba Cloud account chat-completion entitlement it does not have. This is squarely outside Rules 1-3 (nothing is broken in the code; a resource the code correctly requests is denied by the account layer) and outside Rule 4 (no architectural change is being proposed or needed).

## Deviations from Plan

None in Task 1 — executed exactly as written. Task 2 was not attempted beyond diagnosis; nothing was auto-fixed or substituted, per the plan's explicit instruction to halt rather than substitute when live access fails a must-have.

## Issues Encountered — Authentication Gate (blocks Task 2)

**What was attempted:** `generate()` against `qwen-plus` (the shipped default, confirmed present in the live 92-model catalog) via the OpenAI-compatible `chat/completions` endpoint at `https://dashscope-us.aliyuncs.com/compatible-mode/v1`, using the framework's default prompt parameters, exactly as `live_vendor_smoke.rs`'s `probe_generate` does.

**What happened:** `HTTP 403 Forbidden` — `{"error":{"message":"Model access denied.","type":"Model.AccessDenied","param":null,"code":"Model.AccessDenied"}}`.

**Why this is not a stale-model-ID problem (ruled out systematically, via a temporary uncommitted diagnostic example, deleted immediately after use — no credential value was ever printed):**
1. Every one of the 78 `qwen`-prefixed identifiers sampled across tiers (`qwen-plus`, `qwen-flash`, `qwen3-max`, `qwen3.7-max`, `qwen3.7-plus`, `qwen3-8b`/`14b`/`32b`, `qwen3-coder-flash`, `qwen-mt-flash`, `qwen3-max-preview`) and their `-us`-suffixed regional variants (`qwen-plus-us`, `qwen-flash-us`, `qwen3.7-max-us`, `qwen3.7-plus-us`, `qwen3.6-flash-us`, `qwen3-vl-flash-us`) returned the identical `Model.AccessDenied` error.
2. Two entirely different model families hosted on the same DashScope US workspace (`deepseek-v4-flash`, `glm-5.1`) returned the identical error — ruling out a Qwen-specific restriction.
3. The **native** DashScope generation endpoint (`https://dashscope-us.aliyuncs.com/api/v1/services/aigc/text-generation/generation`, not the OpenAI-compatible surface) returned the identical `Model.AccessDenied` for `qwen-plus` — ruling out a compatible-mode-specific misconfiguration.
4. The same credential fully authenticates and succeeds on `GET /models` (92 entries returned, matching G-17-4c's earlier finding) — ruling out an invalid or malformed key. The `/models` catalog is evidently a regional catalog, not a per-workspace entitlement list: it lists everything DashScope offers in that region regardless of what the calling account is authorized to invoke.

**Conclusion:** the available `DASHSCOPE_API_KEY` can list DashScope's US (Virginia) catalog but has no chat-completion invocation entitlement for any model in that workspace. This is consistent with a common Alibaba Model Studio pattern — a workspace can browse the full regional model catalog before activating billing / accepting a model's terms, and invocation is denied uniformly until that account-side step is taken. No model substitution, retry, or code change can resolve it from inside `paladin-llm`.

**Required human action (outside this environment):** the account owner needs to sign in to the Alibaba Cloud Model Studio console, select the US (Virginia) workspace, and activate/enable at least the `qwen-plus` model for API invocation (or resolve whatever billing/quota gate the console reports — the API only returns the generic `Model.AccessDenied` code, not a more specific reason). Once done, task 2 can proceed: re-run

```
cargo run -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini
```

with `DASHSCOPE_BASE_URL` unset, and confirm Qwen's generate probe returns `RESULT : PASS`. From there, task 2's parameter probes (temperature endpoints, the five optional sampling parameters) become measurable using the recipe in the plan's `<executor_notes>`.

**Nothing was left uncommitted or half-landed as a workaround.** The temporary diagnostic example (`crates/paladin-llm/examples/qwen_probe.rs`) used to rule out the above was deleted before this SUMMARY was written; `git status` is clean apart from this SUMMARY and this plan's completed Task 1 commit.

## User Setup Required

**Yes — external service configuration required before task 2 can complete.** See "Authentication Gate" above: the Alibaba Model Studio console for the DASHSCOPE_API_KEY's US (Virginia) workspace needs at least one model (`qwen-plus` recommended, since it is the shipped default and already confirmed present in the live catalog) activated for chat-completion invocation. Verification command given above.

## Next Phase Readiness

- **Task 1's deliverable is complete and safe to build on**: the shipped default now names a reachable region, live-verified by the model-list probe, and the reversal is on the record so it will not be silently re-derived.
- **Task 2 is blocked, not abandoned.** A continuation run (after the console-side activation) should: refresh `QWEN_FALLBACK_MODELS` from the live 92-model catalog (candidates observed live: `qwen-plus`, `qwen-flash`, `qwen3-max`, `qwen3.7-plus`, `qwen3.7-max` — none carry a date/build suffix), measure the five optional sampling-parameter verdicts and both `temperature_range` endpoints against whichever model activation grants access to, and re-run the live harness expecting all four vendors PASS on both probes.
- **Plan 17-22** (making an auth failure audible instead of masked by the curated fallback in `available_models()`) is directly relevant to this exact failure class: a 403 entitlement gate at the model-list layer would look identical to being offline today. Worth noting in that plan's context that entitlement gaps (403) and credential gaps (401) are the same masked-failure family this phase keeps re-discovering.
- **Live model catalog note for 17-20/17-22:** the DashScope US catalog now includes non-Qwen model families (`deepseek-v4-flash`, `glm-5.1`, etc.) hosted on the same platform — irrelevant to this adapter (which only ever requests `qwen*` identifiers) but worth knowing if a future plan greps the catalog for "any model" rather than "any qwen-prefixed model."

---

## Live Verification Evidence

### Live harness output — after Task 1's change, shipped defaults, `DASHSCOPE_BASE_URL` unset (2026-08-22)

```
=== Kimi (MOONSHOT_API_KEY) ===
  base_url      : https://api.moonshot.ai/v1
  default model : kimi-k3
  -- model list probe --
  models returned: 4
  live fetch    : YES — differs from curated fallback
  sample        : kimi-k2.6, kimi-k2.7-code, kimi-k2.7-code-highspeed, kimi-k3
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 40 chars; tokens prompt=86 completion=195 total=281
  RESULT        : PASS

=== Qwen (DASHSCOPE_API_KEY) ===
  base_url      : https://dashscope-us.aliyuncs.com/compatible-mode/v1
  default model : qwen-plus
  -- model list probe --
  models returned: 92
  live fetch    : YES — differs from curated fallback
  sample        : deepseek-v4-flash, deepseek-v4-flash-0731, deepseek-v4-flash-0731-us, deepseek-v4-flash-us, deepseek-v4-pro, deepseek-v4-pro-0813, deepseek-v4-pro-us, glm-5.1, … (+84 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  RESULT        : FAIL — Processing error: API error (403): {"error":{"message":"Model access denied.","type":"Model.AccessDenied","param":null,"code":"Model.AccessDenied"},"id":"chatcmpl-d697c16d-5cd5-45f0-91ee-5d1143eec426","request_id":"d697c16d-5cd5-45f0-91ee-5d1143eec426"}

=== Grok (XAI_API_KEY) ===
  base_url      : https://api.x.ai/v1
  default model : grok-4.6
  -- model list probe --
  models returned: 12
  live fetch    : YES — differs from curated fallback
  sample        : grok-4.20-0309-non-reasoning, grok-4.20-0309-reasoning, grok-4.20-multi-agent-0309, grok-4.3, grok-4.5, grok-4.6, grok-build-0.1, grok-imagine-image, … (+4 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 31 chars; tokens prompt=207 completion=10 total=376
  RESULT        : PASS

=== Gemini (GEMINI_API_KEY) ===
  base_url      : https://generativelanguage.googleapis.com/v1beta
  default model : gemini-3.6-flash
  -- model list probe --
  models returned: 50
  live fetch    : YES — differs from curated fallback
  sample        : antigravity-preview-05-2026, aqa, deep-research-max-preview-04-2026, deep-research-preview-04-2026, deep-research-pro-preview-12-2025, gemini-2.5-computer-use-preview-10-2025, gemini-2.5-flash, gemini-2.5-flash-image, … (+42 more)
  default model in live list: YES
  RESULT        : PASS
  -- generate() probe (default prompt parameters) --
  content       : 31 chars; tokens prompt=2 completion=9 total=191
  RESULT        : PASS

──────────────────────────────────────────
7 of 8 probes passed (4 vendors × 2 probes each; 0 model-list failures, 1 generate failures)
```

**Qwen's model-list probe moved from FAIL (byte-identical to fallback, at Singapore) to PASS (92 live models, at Virginia).** Kimi, Grok and Gemini are unaffected — all 6/6 of their probes pass, confirming no regression from the `QWEN_DEFAULT_BASE_URL` change. Qwen's generate probe is the sole remaining failure, and it is an account-entitlement gate, not a code defect (see Authentication Gate above).

### 403 reproduction across candidates (temporary diagnostic, not committed)

| Model tested | Endpoint | Result |
|---|---|---|
| `qwen-plus` | compat-mode `chat/completions` | 403 `Model.AccessDenied` |
| `qwen-flash`, `qwen3-max` | compat-mode `chat/completions` | 403 `Model.AccessDenied` |
| `qwen-plus-us`, `qwen-flash-us`, `qwen3.7-max-us`, `qwen3.7-plus-us`, `qwen3.6-flash-us`, `qwen3-vl-flash-us` | compat-mode `chat/completions` | 403 `Model.AccessDenied` (all six) |
| `qwen3-8b`, `qwen3-14b`, `qwen3-32b`, `qwen3-max-preview`, `qwen3-coder-flash`, `qwen-mt-flash`, `qwen3.7-flash`, `qwen3.7-max` | compat-mode `chat/completions` | 403 `Model.AccessDenied` (all eight) |
| `deepseek-v4-flash`, `glm-5.1`, `qwen3-max-2025-09-23` | compat-mode `chat/completions` | 403 `Model.AccessDenied` (all three — non-Qwen families too) |
| `qwen-plus` | native DashScope `text-generation/generation` | 403 `Model.AccessDenied` (identical error, different endpoint shape) |

## Threat Flags

None — this plan's `<threat_model>` already covers `QWEN_DEFAULT_BASE_URL` (T-17-89, the accepted new default-destination threat) and the reversal record (T-17-86). The blocked half of task 2 introduces no new code and therefore no new surface; the diagnostic probe example used to characterize the 403 was deleted before this commit and never touched a tracked file.

## Known Stubs

None — no stub code was written. Task 2's incompleteness is a live-measurement blocker, not a stubbed deliverable.

## Self-Check: PASSED

- FOUND: crates/paladin-llm/src/qwen/adapter.rs (QWEN_DEFAULT_BASE_URL changed, region/reversal rustdoc, new pinning test)
- FOUND: CHANGELOG.md (Unreleased ### Changed entry)
- FOUND commit 8208dec (fix: Task 1, Qwen default base_url reversal)
- cargo test -p paladin-llm --features kimi,qwen,grok,ollama,openai-compatible,gemini qwen: 12 passed, 0 failed
- cargo test --test unit --features llm-all: 428 passed, 0 failed, 11 ignored
- cargo fmt --check: clean
- cargo clippy --workspace --all-targets --features llm-all -- -D warnings: clean
- Live harness re-run (shipped defaults, DASHSCOPE_BASE_URL unset): Qwen model-list PASS, Qwen generate FAIL (403, entitlement gate, not a code defect); Kimi PASS/PASS, Grok PASS/PASS, Gemini PASS/PASS (no regression)
- git status confirmed clean of any diagnostic/temporary files before this SUMMARY was written

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-22 (Task 1 only — Task 2 blocked)*
