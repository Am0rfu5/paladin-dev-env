---
phase: 17-additional-llm-provider-adapters
verified: 2026-08-23T13:32:37Z
status: human_needed
score: 14/17 must-haves verified
behavior_unverified: 0
overrides_applied: 0
re_verification:
  previous_status: human_needed
  previous_score: 11/12
  gaps_closed:
    - "All four UAT gaps (G-17-4a Grok, G-17-4b Kimi, G-17-4c Qwen — superseded, G-17-4d Qwen region) independently re-confirmed resolved: CompatRequestParameters mechanism (`compat/engine.rs`) genuinely gates the five optional sampling parameters per preset with no vendor-name branching in the shared engine; Grok declares frequency_penalty/presence_penalty absent (xAI rejects them by presence); Kimi declares temperature/top_p absent (Moonshot enforces fixed values); GROK_DEFAULT_MODEL='grok-4.6', KIMI_DEFAULT_MODEL='kimi-k3', QWEN_DEFAULT_BASE_URL='https://dashscope-intl.aliyuncs.com/compatible-mode/v1' (Singapore, moved twice and reversed with a documented, non-rehabilitating 'Reversal record'), GEMINI_DEFAULT_MODEL='gemini-3.6-flash' (orchestrator fix, commit 954b750) — all confirmed live in the shipped constants by direct read, matching .env.example, config.example.yml, README.md and the configuration guide bidirectionally."
    - "The three 17-REVIEW-gaps.md findings (CR-01 auto-temperature bypassing ADR-0004, WR-01 base_url_without_userinfo under-stripping on a multi-'@' userinfo, WR-02 Kimi's struct-update spread undermining CompatRequestParameters' no-silent-inheritance guarantee) are each independently confirmed fixed in commit 9ce90b7, with dedicated regression tests exercising the exact defect (auto_selected_temperature_is_validated_against_the_provider_range drives .auto_temperature(true) end-to-end against a narrow mock range; base_url_without_userinfo_strips_a_password_containing_an_unescaped_at pins the rfind fix; Kimi's request_parameters now names all five fields explicitly, matching Grok's pattern)."
    - "Plan 17-20's mid-run contract amendment (commit faa6bcb, adding .env.example as Task 3) is coherent and delivered: every env::var call site across all nine adapters (46 variables) is declared in .env.example, in both directions — no adapter reads an undeclared variable and no declared variable is unread."
    - "The live_vendor_smoke.rs harness (crates/paladin-llm/examples/live_vendor_smoke.rs) genuinely exercises both the model-list probe (byte-equality discriminator against the curated *_FALLBACK_MODELS constant, so a plausible-looking fallback cannot be mistaken for a live fetch) and the generate() probe (vacuous-pass guards on empty content and zero token usage) for all four hosted vendors — confirmed by direct read, and the example compiles cleanly with all four provider features."
  gaps_remaining:
    - "Workspace 82% coverage floor: measured 85.01% and confirmed on a real CI run, but at commit ca211644 (2026-08-19) — before ~2,160 lines of gap-closure code landed across plans 17-18 through 17-22 and two orchestrator commits (2026-08-22/23). No coverage re-measurement exists at current HEAD (3f478cd), and no CI run has executed on the branch since 2026-08-19. Routed to human verification."
    - "ADR-0004 (.planning/decisions/0004-temperature-validation.md) was never amended, despite 17-19-SUMMARY.md explicitly flagging this as a phase-close obligation twice ('Flagged for an ADR-0004 amendment at phase close... amending the Accepted ADR is a phase-close act, not a plan's' and 'ADR-0004 amendment recommended at phase close'). The ADR's Decision text still reads as validating temperature unconditionally; the shipped `validate()` now narrows the gate to caller-expressed values only (`manual_temperature_override`). Confirmed absent by direct read — no 2026-08-2x dated addendum exists in the file. Routed to human verification."
    - "WINDOWS.md ledger staleness: rows 12 (Ollama Docker-gated Tier 2 suite) and 13 (workspace coverage floor) remain status 'open' despite 17-UAT.md recording both as 'result: pass' with independently-confirmed CI evidence (this pass verified the underlying CI job conclusions via the public GitHub API). Row 20 (Gemini model-deprecation deviation) remains status 'open' despite the fixing commit 954b750's own message stating 'Closes WINDOWS.md id 20' and the fix being independently confirmed in the shipped constant. This is a bookkeeping gap, not a code defect — but /gsd-ship gates on `open_count > 0`, and the ledger currently misrepresents phase 17's true resolved state on three rows."
  regressions: []
  new_findings: []
gaps: []
human_verification:
  - test: "Re-run `cargo llvm-cov --workspace --features integration-tests,llm-all --fail-under-lines 82` (or push the branch and let CI's `coverage` job run) against the CURRENT HEAD (3f478cd and later), not the 2026-08-19 commit the 85.01% figure was measured against."
    expected: "Coverage stays at or above 82% with all nine adapters' code counted, including the ~2,160 lines added by plans 17-18 through 17-22 and the two orchestrator commits (CompatRequestParameters, classify_fetch_failure, PaladinBuilder's auto-temperature validation, base_url_without_userinfo)."
    why_human: "No Docker daemon reachable at the expected host-mapped ports in this verification sandbox in a way that lets `make coverage`'s own preflight succeed, and `cargo-llvm-cov` is not installed (a from-source `cargo install` was attempted and did not complete within a reasonable window). This sandbox does independently confirm Redis and MinIO are reachable as compose peers (matching 17-UAT.md test 2's finding), and confirms via the public GitHub API that the CI `coverage` job concluded 'success' on 2026-08-19 (run 32269584177) — but that run predates the code this figure is now being cited to cover. No CI run exists on this branch after 2026-08-19."
  - test: "Amend .planning/decisions/0004-temperature-validation.md's Decision section to record the 2026-08-22 narrowing (plan 17-19): the ADR-0004 temperature-range gate in `PaladinBuilder::validate()` now fires only when the caller actually expressed a temperature (`manual_temperature_override`), not unconditionally on whatever value `self.data.temperature` holds. Decide whether to make this amendment now (mechanical — the reasoning is already written, in `validate()`'s own rustdoc and in 17-19-SUMMARY.md) or accept it as tracked follow-up debt."
    expected: "ADR-0004's own text matches what the shipped code does, so a future reader does not learn a stale validation contract from the ADR that ships-code has since narrowed."
    why_human: "This is an editorial decision on an Accepted ADR (a project source-of-truth document), which plan 17-19's own author explicitly declined to make unilaterally ('amending the Accepted ADR is a phase-close act, not a plan's'). Phase close is now; the amendment was never made by anyone in the interim. No code change is required — only a developer decision on whether/how to update the ADR."
  - test: "Reconcile .planning/WINDOWS.md rows 12, 13 and 20 with the resolutions independently confirmed by this verification pass and by 17-UAT.md (`gsd-tools windows fixed 12`, `13`, `20`, or equivalent), so the ledger's `open_count` reflects phase 17's actual state before `/gsd-ship` is run."
    expected: "Rows 12/13/20 read 'fixed' with a resolution note and timestamp, matching the CI evidence and commit 954b750's own closure claim."
    why_human: "A ledger-bookkeeping correction, not a code change — appropriately made by whoever is authorized to edit WINDOWS.md at phase close, informed by this report's independent re-confirmation of the underlying CI job conclusions."
  - test: "Confirm on a real GitHub Actions runner that the current HEAD (post-gap-closure) state still passes `ci.yml` and `feature-flags.yml` in full, including the `Coverage`, `Ollama Integration Tests (live server)` and `LLM Registry Unit Tests (llm-all)` jobs — the last CI run on this branch (cfa59ccc, 2026-08-19) predates all of plans 17-18 through 17-22."
    expected: "All jobs conclude 'success' against the current tree, the same way they did on 2026-08-19."
    why_human: "No CI trigger is available from this verification sandbox; this pass independently confirmed via the public GitHub API that the 2026-08-19 runs succeeded (not taken from any SUMMARY), but that is not evidence about the current HEAD, which carries substantial new code."
  - test: "Smoke-test the live_vendor_smoke.rs harness fresh, one more time, against current HEAD with real credentials for all four hosted vendors (Kimi, Qwen, Grok, Gemini)."
    expected: "All four vendors PASS both the model-list and generate() probes, matching 17-UAT.md's 2026-08-23 measurement, run against the exact code now shipping (this pass confirmed the harness compiles clean and its discrimination logic is genuine, but could not execute it — no vendor credentials or network egress to the vendor APIs in this sandbox)."
    why_human: "No vendor API keys or egress to api.moonshot.ai / dashscope-intl.aliyuncs.com / api.x.ai / generativelanguage.googleapis.com in this verification sandbox."
---

# Phase 17: Additional LLM Provider Adapters Verification Report

**Phase Goal:** Paladin talks to the providers its users actually deploy — the candidate field is
narrowed to a shortlist against recorded criteria rather than brand recognition, and every provider
that survives ships as a feature-gated adapter meeting the same `LlmPort` contract the existing
three do.

**Verified:** 2026-08-23T13:32:37Z
**Status:** human_needed
**Re-verification:** Yes — third pass, after the gap-closure run covering plans 17-18 through
17-22 (UAT gaps G-17-4a/b/c/d, all four vendor-live-probe blockers), the orchestrator's Gemini
model-constant refresh (commit 954b750), the 17-REVIEW-gaps.md code-review fixes (commit 9ce90b7),
and the mid-run 17-20 contract amendment adding `.env.example` (commit faa6bcb).

## Re-verification Summary

**This pass re-derived every claim in the "Verify these claims independently" brief from the live
tree and from independently-queried CI evidence — not from any SUMMARY, UAT record or commit
message text alone.**

- **Live vendor state (shipped constants).** Confirmed by direct read: `KIMI_DEFAULT_MODEL =
  "kimi-k3"`, `QWEN_DEFAULT_MODEL = "qwen-plus"` at `QWEN_DEFAULT_BASE_URL =
  "https://dashscope-intl.aliyuncs.com/compatible-mode/v1"` (Singapore), `GROK_DEFAULT_MODEL =
  "grok-4.6"`, `GEMINI_DEFAULT_MODEL = "gemini-3.6-flash"`. All four match `.env.example`,
  `config.example.yml`, `crates/paladin-llm/README.md` and
  `docs/src/getting-started/configuration.md` exactly. The `live_vendor_smoke.rs` harness genuinely
  probes both `get_available_models()` (with a byte-equality discriminator against the curated
  fallback, so a plausible list cannot be mistaken for a live fetch) and `generate()` (with
  vacuous-pass guards on empty content / zero token usage) for all four hosted vendors — read
  directly, not inferred — and it compiles cleanly (`cargo build -p paladin-llm --example
  live_vendor_smoke --features kimi,qwen,grok,gemini`). This verifier has no vendor credentials or
  network egress to the vendor APIs, so the actual live run itself could not be reproduced — that
  remains a human-verification item, unchanged from what the brief asked to be checked rather than
  trusted.
- **The two orchestrator changes, outside plan scope.** `gemini/adapter.rs`'s model-constant refresh
  (commit 954b750) is appropriate: it corrects a vendor-side retirement (`gemini-2.5-flash` "no
  longer available to new users") that three plans' must_haves required Gemini to clear, is scoped
  to exactly the constants and one mock-test literal that needed it, and is fully documented in the
  constant's own rustdoc with the measurement date. The three code-review fixes (commit 9ce90b7)
  are each independently confirmed fixed with dedicated regression tests: CR-01 (auto-temperature
  now validated end-to-end via a test that drives `.auto_temperature(true)` against a mock port
  declaring a narrow `(1.0, 1.0)` range and asserts the build is refused), WR-01
  (`base_url_without_userinfo` now uses `rfind('@')` per RFC 3986, with a regression test on a
  multi-`@` userinfo), WR-02 (Kimi's `CompatRequestParameters` now names all five fields explicitly,
  matching Grok's pattern, closing the struct-update-syntax gap in the type's own documented
  "no silent inheritance" guarantee).
- **Plan 17-20's mid-run amendment.** Coherent and delivered. `.env.example` declares every one of
  the 46 `env::var` names read across all nine adapters (verified bidirectionally by enumerating
  call sites vs. declared names) — the three previously-absent credential names (`MOONSHOT_API_KEY`,
  `DASHSCOPE_API_KEY`, `OPENAI_COMPATIBLE_API_KEY`) are present, and every `*_BASE_URL`/`*_MODEL`/
  `*_TIMEOUT_SECONDS` commented-out override carries the current shipped default value, not a stale
  one.
- **`QWEN_DEFAULT_BASE_URL`'s two moves.** The "Reversal record" rustdoc in `qwen/adapter.rs` is
  read directly and is coherent: it explicitly states the original prohibition against changing the
  constant "was wrong and MUST NOT be reinstated or re-derived," explains why a well-formed `401`
  cannot distinguish a wrong URL from a right URL with a region-scoped wrong key, and explicitly
  refuses to treat Move 2 (the reversal back to Singapore) as vindication of the original default —
  "This is not a rehabilitation of the original Singapore default, and it is not evidence that Move
  1 was wrong." Neither the falsified prohibition nor the invalid inference is rehabilitated
  anywhere in the shipped code.
- **Accepted debt not re-litigated.** `WINDOWS.md` id 19 (`.project/current-exports.txt` generated
  under default features only) remains open, unchanged, by the recorded developer decision — not
  reopened here.

**What this pass found beyond the brief's checklist, independently:** a full, fresh
`cargo test --workspace --features llm-all` at current HEAD passes with zero failures; all nine
providers build individually at current HEAD; `cargo clippy --workspace --all-targets
--all-features -- -D warnings` is clean; `cargo audit`/`cargo deny` are clean (9 pre-existing
allowlisted advisories, `h2` confirmed at the patched `0.4.16`); `cargo doc` produces zero
`missing_docs` warnings (5 pre-existing `private_intra_doc_links` warnings, up from 3, from new
cross-references added by the gap-closure docs). Two CI runs on this repository's public GitHub
API were independently queried (not taken from any SUMMARY) and confirmed successful: `ci.yml` run
`32269584177` (`Coverage` and `Ollama Integration Tests (live server)` jobs both `success`) and
`feature-flags.yml` run `32269584207` (`LLM Registry Unit Tests (llm-all)` and `Feature Matrix
Summary` both `success`). **However, both runs are against commit `ca211644` (2026-08-19) — the
last CI run on this branch — which predates roughly 2,160 lines of gap-closure code added across
plans 17-18 through 17-22 and the two orchestrator commits (2026-08-22/23).** The 85.01%
coverage figure and the Ollama live-server pass are therefore real, CI-confirmed facts about an
earlier commit, not about the code as it ships today. This is the pass's single largest evidentiary
gap and is why the status below is `human_needed` rather than `passed`.

Two further findings, not part of the original brief, surfaced during independent verification:
**(1)** `.planning/decisions/0004-temperature-validation.md` was never amended, despite
`17-19-SUMMARY.md` twice explicitly flagging this as a phase-close obligation ("amending the
Accepted ADR is a phase-close act, not a plan's"). **(2)** `.planning/WINDOWS.md` rows 12, 13 and
20 remain `open` despite being independently confirmed resolved by this pass (CI job conclusions
for 12/13; the shipped `GEMINI_DEFAULT_MODEL` constant and commit 954b750's own "Closes WINDOWS.md
id 20" for row 20). Neither finding is a code defect; both are genuine, checkable gaps in this
phase's own record-keeping, surfaced for human decision rather than silently absorbed or silently
fixed by this verifier.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A recorded provider-selection study evaluates candidates against explicit written criteria, with every candidate carrying one build/defer/reject verdict | ✓ VERIFIED | `.planning/decisions/0045-additional-llm-provider-selection.md`, unchanged since prior passes. Re-confirmed present. |
| 2 | Every build-list adapter (Kimi, Qwen, Grok, Ollama, Gemini, generic openai-compatible) implements all six `LlmPort` methods with no stubbed body | ✓ VERIFIED | `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` → **247 passed; 0 failed** (up from 217, +30 tests from the gap-closure work), run fresh in this pass. |
| 3 | All previously-closed CR-01/WR-01/WR-02/WR-03/WR-04 findings (provider_factory availability, alias normalisation, temperature-range validation, Gemini truncated-completion, stream-open retry parity) stay closed | ✓ VERIFIED | Re-confirmed present by direct read at current HEAD; no regression. |
| 4 | **G-17-4a (Grok):** completes a `generate()` call against a current xAI model with default parameters | ✓ VERIFIED — CLOSED | `CompatRequestParameters` gates `frequency_penalty`/`presence_penalty` false on Grok's declaration (`grok/adapter.rs:211-217`); `GROK_DEFAULT_MODEL = "grok-4.6"`, live-verified per 17-UAT.md test 4/G-17-4a and matching the shipped constant read directly. |
| 5 | **G-17-4b (Kimi):** completes a `generate()` call using its default model and default parameters | ✓ VERIFIED — CLOSED | `KIMI_DEFAULT_MODEL = "kimi-k3"`; `temperature`/`top_p` both declared absent on Kimi's `CompatRequestParameters` (`kimi/adapter.rs:241-247`, all five fields spelled out post-WR-02 fix); `PaladinBuilder::validate()`'s ADR-0004 gate narrowed to caller-expressed values, confirmed by direct read and by the CR-01 regression test exercising the auto-temperature path end to end. |
| 6 | **G-17-4d (Qwen region + diagnosability):** an operator on any DashScope region can reach their account with a documented override, and a region mismatch is audible, not silent | ✓ VERIFIED — CLOSED | `classify_fetch_failure` (10 match arms, exhaustive over `LlmError`'s 10 non-`#[non_exhaustive]` variants, confirmed by direct read and by a clean clippy pass) routes `AuthenticationError` to `Misconfiguration` (→ `warn!`), everything else to `Supported` (→ `debug!`). `qwen/adapter.rs`'s "Reversal record" rustdoc names all three regional endpoints, states the region-scoped-credential rule, and explicitly refuses to rehabilitate the falsified 401-proves-URL inference. |
| 7 | Each new provider is feature-gated: `cargo build -p paladin-llm --no-default-features --features <provider>` succeeds for every provider individually; default feature set unchanged | ✓ VERIFIED | Re-run fresh in this pass at current HEAD: all nine of `openai`, `anthropic`, `deepseek`, `kimi`, `qwen`, `grok`, `ollama`, `openai-compatible`, `gemini` build individually. `paladin-llm`'s `default = ["openai","mock"]` and the facade's `default = ["llm-openai","llm-anthropic","llm-deepseek"]` both unchanged. |
| 8 | Config surface accepts the new providers without a breaking change to existing config | ✓ VERIFIED | `crates/paladin-llm/src/config/bridge.rs` carries `#[cfg(feature = "kimi"/"qwen"/"grok"/"gemini")]`-gated bridging blocks reading each adapter's own `*_DEFAULT_*` constants; existing providers' config paths untouched (`git diff --stat` shows no changes to `openai/`, `anthropic/`, `deepseek/` adapter directories since ca211644). |
| 9 | Mock-transport unit tests cover request shaping, response parsing, streaming, and error mapping for every new adapter; rustdoc on every public item | ✓ VERIFIED | 247 crate-scoped tests passing; `cargo doc -p paladin-llm` → 5 warnings, all pre-existing-class `private_intra_doc_links`, **zero `missing_docs`**, confirmed fresh in this pass. |
| 10 | The workspace stays at or above the 82% line-coverage floor with the new provider code counted (ADR-0006) | ⚠️ UNCERTAIN (stale evidence) | 85.01% was measured and CI-confirmed (`ci.yml` run `32269584177`, job "Coverage" → `success`, independently queried via the public GitHub API in this pass) — but against commit `ca211644` (2026-08-19), before ~2,160 lines of gap-closure code landed. No coverage measurement exists at current HEAD. Routed to Human Verification. |
| 11 | Live-API / real-endpoint behaviour is exercised for the credential-gated and Docker-gated tests | ✓ VERIFIED (as of 2026-08-19/22/23, not re-run at current HEAD by this verifier) | `ci.yml` run `32269584177` job "Ollama Integration Tests (live server)" → `success` (independently confirmed via GitHub API). All four hosted vendors (Kimi, Qwen, Grok, Gemini) PASS both live probes per 17-UAT.md test 4, with a harness this pass confirmed by direct read is genuinely discriminating, not vacuous. This verifier has no vendor credentials to re-run the live probe itself. |
| 12 | New/modified first-party code from this phase has an appropriate security-scanning posture (Snyk mandate formally retired and replaced) | ✓ VERIFIED | `.github/instructions/security.instructions.md` documents the Snyk removal with evidence (0/4 planted-vulnerability detection vs. 3/4 in equivalent JavaScript); `cargo audit` clean (9 pre-existing allowlisted advisories, re-confirmed fresh, exit 0); `cargo deny check` → "advisories ok, bans ok, licenses ok, sources ok"; `h2` confirmed at patched `0.4.16` in `Cargo.lock`. |
| 13 | The advertised surface (Cargo.toml, README, configuration docs) names exactly the providers that exist, with currency accurate per vendor | ✓ VERIFIED | Re-confirmed by direct read: README's provider table and `config.example.yml`/configuration guide all carry the current shipped constants (`kimi-k3`, `qwen-plus`@Singapore, `grok-4.6`, `gemini-3.6-flash`), with per-vendor verification notes replacing the old blanket caveat. |
| 14 | ADR-0004's own text reflects the temperature-validation behaviour the shipped code implements | ✗ NOT DONE — genuine documentation gap | `.planning/decisions/0004-temperature-validation.md` confirmed, by direct read, to carry no amendment for the 2026-08-22 narrowing (caller-expressed-only gate) that `17-19-SUMMARY.md` twice flagged as a required phase-close act. Escalated to Human Verification rather than silently absorbed or unilaterally amended by this verifier. |
| 15 | The cross-phase defect ledger (WINDOWS.md) accurately reflects phase 17's resolved items | ✗ NOT DONE — genuine bookkeeping gap | Rows 12, 13 (Ollama Docker, coverage floor — both independently confirmed `pass`/`success` in this round) and 20 (Gemini model deprecation — fixed by commit 954b750, whose own message claims "Closes WINDOWS.md id 20") all remain `status: open`. Escalated to Human Verification. |
| 16 | REQUIREMENTS.md checkboxes for PROV-01..04 reflect the phase's actual, independently-verified completion state | ⚠️ Judgment offered, not enacted | All four remain unticked (`grep -n "PROV-0[1-4]" REQUIREMENTS.md` → all `- [ ]`), consistent with the developer's explicit "adjudicate at phase close" precedent carried since the prior pass. On the evidence in this report, PROV-01/02/03 are genuinely satisfied and tickable; PROV-04 hinges on the now-stale coverage figure (truth 10) and is not recommended for ticking until re-confirmed at current HEAD. No checkbox is altered by this report. |
| 17 | Full workspace test suite is green at current HEAD (fresh, not carried from any prior pass) | ✓ VERIFIED | `cargo test --workspace --features llm-all` run fresh in this pass at commit `3f478cd` → **zero failures across the entire workspace**, all doctests passing. `cargo clippy --workspace --all-targets --all-features -- -D warnings` → clean. |

**Score:** 14/17 truths verified. 1 uncertain due to stale (pre-gap-closure) evidence (truth 10);
2 genuine, unenacted phase-close obligations (truths 14, 15) that are checkable facts, not code
defects; truth 16 is a judgment offered for the human phase-close decision, not itself a
pass/fail. None of these four blocks the phase goal's core code-level achievement, which this pass
independently re-confirmed end to end at current HEAD.

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/paladin-llm/src/compat/engine.rs` — `CompatRequestParameters` | Per-preset, no-`Default`, no-struct-update declaration of which optional sampling parameters a preset's request path carries | ✓ VERIFIED | Confirmed by direct read; Grok and Kimi both now spell out all five fields explicitly (WR-02 fix applied to Kimi). |
| `crates/paladin-llm/src/compat/engine.rs` — `classify_fetch_failure` | Exhaustive `LlmError` classification, no wildcard arm | ✓ VERIFIED | 10 match arms over `LlmError`'s 10 non-`non_exhaustive` variants, confirmed by direct read and by a clean workspace-wide clippy pass (an inexhaustive match would not compile). |
| `crates/paladin-llm/src/compat/engine.rs` — `base_url_without_userinfo` | Redacts the full userinfo component, including a multi-`@` password | ✓ VERIFIED | `rfind('@')` confirmed at line 316, with a passing regression test on a multi-`@` userinfo. |
| `crates/paladin-llm/src/qwen/adapter.rs` | `QWEN_DEFAULT_BASE_URL`/`QWEN_DEFAULT_MODEL` at shipped, live-verified values; coherent non-rehabilitating "Reversal record" | ✓ VERIFIED | `dashscope-intl.aliyuncs.com` / `qwen-plus`, confirmed by direct read; rustdoc reviewed line by line for the two required negative properties (does not reinstate the falsified prohibition; does not rehabilitate the invalid 401-proves-URL inference) — both hold. |
| `crates/paladin-llm/src/kimi/adapter.rs`, `grok/adapter.rs`, `gemini/adapter.rs` | Live-verified default model constants | ✓ VERIFIED | `kimi-k3`, `grok-4.6`, `gemini-3.6-flash`, confirmed by direct read. |
| `src/application/services/paladin/paladin_builder.rs` | CR-01 fix — auto-selected temperature validated before assignment | ✓ VERIFIED | `build()`'s auto-temperature branch validates `optimal_temp` against the provider's declared range *before* assigning it, refusing with a named `ConfigurationError` if out of range; confirmed by direct read and by a dedicated end-to-end test (`auto_selected_temperature_is_validated_against_the_provider_range`) plus its passing control (`auto_selected_temperature_inside_the_provider_range_still_builds`). |
| `.env.example` | Declares the complete, code-derived environment-variable surface for every LLM adapter | ✓ VERIFIED | All 46 `env::var` call-site names present (directly enumerated and diffed against the file, both directions); `MOONSHOT_API_KEY`/`DASHSCOPE_API_KEY`/`OPENAI_COMPATIBLE_API_KEY` (previously absent) now present. |
| `config.example.yml`, `crates/paladin-llm/README.md`, `docs/src/getting-started/configuration.md` | Refreshed identifiers and base URLs, per-vendor verification notes | ✓ VERIFIED | Confirmed by direct read; no retired identifier or superseded base URL found in any of the three. |
| `.planning/phases/17-additional-llm-provider-adapters/COVERAGE.md` | Verification-status section distinguishing live-exercised from mock-transport-only, amended in place per D-00d | ✓ VERIFIED | Section present, dated, with the superseded text preserved rather than deleted; per-surface live-vs-mock table present. |
| `crates/paladin-llm/examples/live_vendor_smoke.rs` | Non-vacuous live harness for all four hosted vendors | ✓ VERIFIED | Confirmed by direct read (byte-equality fallback discriminator, vacuous-pass guards) and by a clean local build with all four provider features. |
| `.planning/decisions/0004-temperature-validation.md` | Amended to reflect the narrowed ADR-0004 gate (17-19's own flagged phase-close obligation) | ✗ MISSING | Confirmed absent by direct read — no amendment, no dated addendum. Routed to Human Verification. |
| `.planning/WINDOWS.md` rows 12, 13, 20 | Marked `fixed` to match independently-confirmed resolutions | ✗ NOT UPDATED | All three confirmed still `status: open` by direct read, despite independently-confirmed CI evidence (rows 12/13) and the fixing commit's own closure claim (row 20). Routed to Human Verification. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `grok/adapter.rs` / `kimi/adapter.rs` `request_parameters` declarations | `compat/engine.rs build_request` | `CompatRequestParameters` field gating | ✓ WIRED | Confirmed by direct read; no vendor-name branching in the shared engine — the declaration alone controls field presence. |
| `paladin_builder.rs build()`'s auto-temperature branch | `validate()`'s ADR-0004 range check | Value validated in-branch before assignment, rather than deferred to a later `validate()` call gated on a flag never set on this path | ✓ WIRED | Confirmed by direct read and by the dedicated end-to-end test exercising `.auto_temperature(true)`. |
| `.env.example` declared names | `crates/paladin-llm/src/*/adapter.rs` `env::var` call sites | Enumerated bidirectional match | ✓ WIRED | 46/46 names present in both directions; no undeclared read, no unread declaration. |
| `config.example.yml` / README / configuration guide default_model & base_url values | Adapter `*_DEFAULT_*` constants | Direct value match | ✓ WIRED | Confirmed by direct read across all four refreshed vendors. |
| `qwen/adapter.rs`'s `classify_fetch_failure`-driven `warn!` | `compat/engine.rs available_models()` | `AuthenticationError` routed to `Misconfiguration` → `warn!`; all else to `Supported` → `debug!` | ✓ WIRED | Confirmed by direct read of the exhaustive match and its consuming call site. |
| `17-19-SUMMARY.md`'s "ADR-0004 amendment recommended at phase close" | `.planning/decisions/0004-temperature-validation.md` | Phase-close editorial follow-through | ✗ NOT WIRED | The recommendation was never acted on. Genuine, checkable gap. |
| `954b750`'s "Closes WINDOWS.md id 20" | `.planning/WINDOWS.md` row 20 | Ledger status update | ✗ NOT WIRED | Row 20 still reads `open`. Genuine, checkable gap. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Crate-scoped six-preset suite, current HEAD | `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,gemini,ollama,openai-compatible"` | 247 passed; 0 failed | ✓ PASS |
| Full workspace unit-test binary under `llm-all` | `cargo test --test unit --features llm-all` | 428 passed; 0 failed; 11 ignored | ✓ PASS |
| **Full workspace test suite, fresh, current HEAD (not carried from any prior pass)** | `cargo test --workspace --features llm-all` | Zero failures across the entire workspace; all doctests pass | ✓ PASS |
| Workspace clippy, all targets, all features | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, 0 warnings | ✓ PASS |
| Each of the 9 providers builds individually, current HEAD | `cargo build -p paladin-llm --no-default-features --features <provider>` × 9 | all 9 succeed | ✓ PASS |
| `live_vendor_smoke` example compiles | `cargo build -p paladin-llm --example live_vendor_smoke --features kimi,qwen,grok,gemini` | exit 0 | ✓ PASS |
| `cargo audit` | — | 9 pre-existing allowlisted advisories, exit 0 | ✓ PASS |
| `cargo deny check` | — | "advisories ok, bans ok, licenses ok, sources ok" | ✓ PASS |
| `cargo doc` — zero `missing_docs` | `cargo doc -p paladin-llm --no-deps --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` | 5 warnings, all `private_intra_doc_links`, zero `missing_docs` | ✓ PASS |
| CI job conclusions, independently queried via public GitHub API (not from any SUMMARY) | `curl api.github.com/.../actions/runs/32269584177/jobs`, `.../32269584207/jobs` | "Coverage" success, "Ollama Integration Tests (live server)" success, "LLM Registry Unit Tests (llm-all)" success, "Feature Matrix Summary" success | ✓ PASS (but at commit `ca211644`, 2026-08-19 — stale relative to current HEAD; see Human Verification) |
| Coverage floor at CURRENT HEAD | `cargo llvm-cov --workspace --features integration-tests,llm-all --fail-under-lines 82` | Not run — `cargo-llvm-cov` not installed, install-from-source did not complete in time | ? SKIP — routed to Human Verification |

### Probe Execution

No `scripts/*/tests/probe-*.sh` convention exists in this Rust workspace. SKIPPED (no probe scripts
declared for this phase).

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| PROV-01 | 17-02 | Narrow the candidate field to a recorded decision | ✓ SATISFIED (checkbox unticked — see note) | ADR-0045 present and unchanged; re-confirmed. |
| PROV-02 | 17-01, 17-03…17-22 | Full `LlmPort` contract, truthful capabilities, distinguishable failures | ✓ SATISFIED (checkbox unticked — see note) | All six build-list adapters implement the full trait; 247 crate-scoped tests pass fresh; four of six now further confirmed by real live `generate()` calls (per 17-UAT.md test 4, harness independently confirmed non-vacuous by this pass). |
| PROV-03 | 17-01, 17-03…17-13 | Feature-gated, additive, default unchanged, config surface accepts new providers | ✓ SATISFIED (checkbox unticked — see note) | All 9 providers build individually at current HEAD (re-run fresh); both default feature sets unchanged; `config/bridge.rs` confirmed bridging all four gap-closure-refreshed providers. |
| PROV-04 | 17-01…17-22 | Tested/documented to standard, advertised surface accurate, 82% coverage floor held | ⚠️ PARTIALLY SATISFIED — coverage claim stale | Mock-transport tests (247 crate-scoped, 428 workspace-unit), zero `missing_docs`, accurate advertised surface all confirmed fresh at current HEAD. **The 82%-coverage-floor clause is evidenced only against a commit that predates ~2,160 lines of this phase's own later work** — genuinely unconfirmed at the commit this report is verifying. Escalated to Human Verification, not silently absorbed. |

**REQUIREMENTS.md checkbox note, updated for this pass:** All four PROV-01 through PROV-04
checkboxes remain **unticked** (`grep -n "PROV-0[1-4]" .planning/REQUIREMENTS.md` → all `- [ ]`,
confirmed live). This is consistent with the developer's explicit adjudicate-at-phase-close
precedent, carried forward unchanged from the prior verification pass. **On the evidence gathered
in this pass: PROV-01, PROV-02 and PROV-03 remain genuinely satisfied and tickable at phase close.
PROV-04 should still not be ticked** — not for the Snyk reason the prior pass cited (that mandate
has since been formally and adequately retired, per `security.instructions.md`), but because its
own "82% coverage floor" clause is evidenced only against stale code. This judgment is offered for
the human phase-close decision; no checkbox is altered by this report.

No orphaned requirements — `REQUIREMENTS.md`'s Phase 17 section names exactly PROV-01..04, all four
cited in plan frontmatter across all 22 plans.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| — | — | No `TBD`/`FIXME`/`XXX` debt markers found in any file changed since the last verification pass (`git diff --name-only ca211644..HEAD`, 16 files across `crates/paladin-llm/`, `src/application/services/paladin/paladin_builder.rs`, `.env.example`, `config.example.yml`, docs) | — | — |
| `.project/current-exports.txt` | — | Regenerated under default features only; the six new adapter types are not re-exported for public-API-drift tracking (IN-01, carried forward, `WINDOWS.md` id 19) | ⚠️ Warning, accepted debt | Excluded from scope by explicit developer decision recorded at an interactive checkpoint on 2026-08-18. Not re-litigated. |
| `.planning/decisions/0004-temperature-validation.md` | — | Un-amended despite an explicit, twice-recorded phase-close obligation in `17-19-SUMMARY.md` | ⚠️ Warning, genuine gap | Not a code defect — the code's own rustdoc states the current, correct behaviour. But a reader consulting the ADR (the project's designated single source of truth for this decision, per `CLAUDE.md`) would learn a contract the shipped code no longer implements. |
| `.planning/WINDOWS.md` | rows 12, 13, 20 | Ledger rows remain `open` despite independently-confirmed resolutions | ⚠️ Warning, genuine gap | `/gsd-ship` gates on `open_count > 0`; these three rows currently overstate phase 17's outstanding work. |

### Human Verification Required

See `human_verification` in the frontmatter for five items: (1) re-measuring the 82% coverage
floor at current HEAD, since the only measurement on record predates ~2,160 lines of this phase's
own gap-closure work; (2) deciding whether/how to amend ADR-0004's Decision text to match the
shipped `validate()` narrowing, a phase-close obligation this phase's own plan 17-19 flagged twice
and never enacted; (3) reconciling `WINDOWS.md` rows 12, 13 and 20 with the resolutions this pass
independently confirmed; (4) confirming CI is green on the current HEAD, since the last CI run on
this branch predates all of plans 17-18 through 17-22; and (5) re-running the live vendor smoke
test fresh against current HEAD with real credentials, since this verifier has none. None of the
five is a code-level defect in the shipped adapters — the phase's core deliverable (nine
feature-gated adapters implementing the full `LlmPort` contract, independently re-confirmed by a
fresh, zero-failure full workspace test run at current HEAD) is genuinely achieved.

### Gaps Summary

**The phase's code-level goal is achieved and independently re-confirmed at current HEAD**, not
merely claimed by the SUMMARYs: nine feature-gated LLM adapters (six new — Kimi, Qwen, Grok,
Ollama, Gemini, generic openai-compatible — plus the three pre-existing) each implement the full
`LlmPort` contract with truthful capabilities, build individually and in combination, pass 247
crate-scoped and 428 workspace-unit tests, and pass a fresh, zero-failure full `cargo test
--workspace --features llm-all` run plus a clean `cargo clippy --workspace --all-targets
--all-features -- -D warnings`. All four blocking/major UAT gaps found during live-vendor testing
(G-17-4a Grok, G-17-4b Kimi, G-17-4c/G-17-4d Qwen) are independently confirmed closed by direct
code read, not merely by the UAT record's own claim. All three 17-REVIEW-gaps.md findings are
independently confirmed fixed with dedicated regression tests. The mid-run `.env.example` contract
amendment is coherent and delivered in full, verified bidirectionally against every adapter's
`env::var` call site.

**What keeps this pass at `human_needed` rather than `passed`** is not a code defect in the
shipped adapters — it is three genuine, checkable gaps in this phase's own supporting record:
the 82%-coverage-floor evidence is real but stale (measured before this phase's own later work
landed); an ADR this phase's own executors explicitly flagged for phase-close amendment was never
amended; and three `WINDOWS.md` rows this phase itself should have closed remain open. Each is
surfaced above with the specific evidence this verifier gathered independently — not carried
forward on trust from a SUMMARY, and not silently fixed or silently waved through by this
verifier.

---

_Verified: 2026-08-23T13:32:37Z_
_Verifier: Claude (gsd-verifier)_


---

## Orchestrator addendum — 2026-08-23, after the verification pass

Recorded by the `/gsd-execute-phase 17 --gaps-only` orchestrator. The verifier's five
`human_verification` items are reduced to two; the rest were closed or were already stale when
written.

### Closed since the report

**Item 2 — ADR-0004 amendment: DONE** (commit `1deceae`). The Accepted ADR now carries a dated
amendment recording four things phase 17 established: A1 the gate fires only on an *expressed*
temperature (`PaladinData::default()`'s fabricated 0.7 is not a request, and judging it turned
Kimi's truthful degenerate `(1.0, 1.0)` into a denial of service); A2 auto-selected temperatures
are validated at the point of assignment, closing review finding CR-01, refused by name rather
than clamped; A3 parameter *omission* is explicitly distinguished from the adapter-level clamping
the ADR rejected — an absent JSON key states no intent, a clamp silently rewrites one; A4 a
declared range must be the measured one including half-open bounds, hence Qwen's `(0.0, 1.99)`
for a vendor-enforced `[0.0, 2.0)`.

**Item 5 — live smoke against current HEAD: DONE.** The verifier's sandbox had no vendor
credentials or egress; the orchestrator's does. Run against the exact shipped code with
`DASHSCOPE_BASE_URL` unset, harness exit 0:

| Vendor | Shipped default | model-list | generate |
|---|---|---|---|
| Kimi | `kimi-k3` | PASS | PASS |
| Qwen | `qwen-plus` @ `dashscope-intl` | PASS | PASS |
| Grok | `grok-4.6` | PASS | PASS |
| Gemini | `gemini-3.6-flash` | PASS | PASS |

No spurious warning on shipped defaults. This also closes plan 17-22's previously-unmet
"four vendors PASS" clause.

### Correction to the report

**`WINDOWS.md` row 20 was already resolved** when the verifier reported it open. It was marked
resolved earlier in the same session, before the verification pass ran. The likely cause of the
misreport is benign: an orphaned duplicate worktree (`agent-aa420435f9e981266`, since removed)
carried a pre-resolution copy of `WINDOWS.md`, and a glob over the tree would have matched it.
Current ledger state: ids 20 and 21 both `resolved`.

**Rows 12 and 13 were deliberately NOT reconciled.** Their resolution rests on CI job conclusions
the verifier read from the public GitHub API. The orchestrator could not independently re-confirm
those (the `gh` CLI is unauthenticated in this environment and the connected GitHub MCP server
exposes no workflow-run tool), and marking a ledger row resolved on unverified secondhand evidence
is the kind of bookkeeping this ledger exists to prevent. A CI run on current HEAD would settle
them directly — see below.

### The two items that genuinely remain — both are one action

Items 1 (coverage floor at current HEAD) and 4 (full CI on current HEAD) have a single cause:
**the branch has 51 unpushed commits.** The remote `chore/17-discuss-phase` sits at `cfa59cc`
(2026-08-19); local HEAD is well beyond it. No CI has run on any gap-closure code because none of
it has reached the remote. `cargo-llvm-cov` is not installed locally, so coverage cannot be
re-measured here either.

Pushing the branch closes items 1 and 4, and would also produce first-hand evidence for ledger
rows 12 and 13. Until then the 85.01% coverage figure is honestly described as *measured at
`ca211644` (2026-08-19), before ~2,160 lines of gap-closure code landed* — not as a current
measurement.

Everything else in this report stands: the phase's core deliverable is achieved, re-confirmed
fresh at current HEAD, with `cargo test --workspace`, `cargo fmt --check` and
`cargo clippy --workspace --all-targets --features llm-all -- -D warnings` all clean.


---

## CI evidence at current HEAD — 2026-08-23, after the push

The branch was pushed (`cfa59cc..76b859d`) with the developer's approval, which was the single
action all remaining items depended on. CI then ran on gap-closure code for the first time.

**Result: 44 success, 3 skipped, 0 failures.** Read first-hand from the GitHub check-runs API for
commit `76b859d`, not taken from any report.

| Job | Conclusion | Closes |
|---|---|---|
| `Coverage` | success | item 1, `WINDOWS` row 13 |
| `Ollama Integration Tests (live server)` | success | `WINDOWS` row 12 |
| `LLM Registry Unit Tests (llm-all)` | success | item 4 |
| `Build & Test (llm-all)` | success | item 4 |
| `Code Quality` | success | item 4 |
| `Integration Tests` | success | item 4 |
| `Docker Integration Tests` | success | item 4 |

**On the coverage figure specifically.** The `Coverage` job runs
`cargo llvm-cov --fail-under-lines 82`, so a success conclusion *is* the assertion that ADR-0006's
82% workspace line-coverage floor holds against the gap-closure code. The job emitted no
percentage into its check-run output, so what is recorded here is the verdict, not a number. The
stale 85.01% figure from `ca211644` is superseded as evidence and should not be re-cited as a
current measurement.

**One job did not finish:** `Docker Build` was still `in_progress` ~57 minutes after starting. It
is not a failure and gates none of the items above; it does not appear in any must-have. Worth a
glance if it stays stuck, since a genuinely hung image build is its own problem.

### Status of the five `human_verification` items

| # | Item | State |
|---|---|---|
| 1 | Coverage at current HEAD | **CLOSED** — `Coverage` job success at `76b859d` |
| 2 | ADR-0004 amendment | **CLOSED** — commit `1deceae` |
| 3 | `WINDOWS` rows 12/13/20 | **CLOSED** — 20 was already resolved; 12/13 reconciled on this CI evidence |
| 4 | Full CI at current HEAD | **CLOSED** — 44/44 non-skipped checks success |
| 5 | Live four-vendor smoke at current HEAD | **CLOSED** — 8/8 probes, shipped defaults, no override |

All five are closed. The blocking reason for `status: human_needed` no longer holds; the phase is
ready for a final verification pass to move it to `passed`. That re-verification is deliberately
left to a fresh `gsd-verifier` run rather than asserted here, since the orchestrator closing items
against its own execution is exactly the self-evaluation blind spot the verifier exists to catch.
