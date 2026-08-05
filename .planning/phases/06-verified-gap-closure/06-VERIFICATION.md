---
phase: 06-verified-gap-closure
verified: 2026-08-05T21:00:00Z
status: gaps_found
score: 8/10 must-haves verified
behavior_unverified: 0
overrides_applied: 0
gaps:
  - truth: "Grove LLM routing hard-errors with no fallback of any kind when `routing_model` is absent under `RoutingStrategy::LlmRouting`, and that error is reachable from `GroveExecutionService::execute()` — the only public entry point (D-02; ROADMAP criterion 1's implicit contract; PLAN 06-01 must_haves `[edge/CLOSE-01/empty]` and `[edge/CLOSE-01/adjacency]`)."
    status: failed
    reason: >
      `route_by_llm` (grove_service.rs:487-510) does correctly return
      `BattalionError::RoutingError` when `routing_model` is `None`/blank under LLM routing,
      with no fallback consulted inside that function — that half of D-02 is real. But
      `route_task` (grove_service.rs:240-290), the only caller `execute()` (grove_service.rs:171)
      goes through, wraps every strategy call in a blanket `match result { Err(e) => ... }` that
      catches *any* error — including this deliberate, no-fallback-by-design `RoutingError` —
      and silently substitutes `fallback_tree` or, failing that, "first agent in first tree."
      `GroveBuilder::build()` (grove.rs:521-560) requires at least one non-empty tree to
      construct a `Grove` at all, so that final fallback can never itself fail. The practical
      effect: a Grove configured for `RoutingStrategy::LlmRouting` with no `routing_model` set,
      called through `execute()`, does not error — it silently and completely bypasses LLM
      routing and falls back to default agent selection, with no error ever surfacing to the
      caller. This is arguably a regression from the pre-phase behaviour (which at least
      attempted an LLM call, hardcoded to `gpt-4`): the new behaviour hides the misconfiguration
      entirely.
      Evidence this is real, not theoretical: `tests/integration/battalion/grove_integration_test.rs::test_grove_llm_routing`
      (line 235) builds a Grove with `RoutingStrategy::LlmRouting` and no `routing_model`, calls
      `service.execute(...)`, and asserts `result.is_ok()` — currently green, with its own inline
      comment reading "Execute task - will use keyword fallback since we don't have real LLM."
      Plan 06-01's own commit (05ee6b4) modified the *sibling* test
      `test_grove_llm_routing_end_to_end` to add `.routing_model("gpt-4")` and explicitly noted in
      the commit message that `test_grove_llm_routing` "hits the pre-existing llm_port guard
      first and was unaffected" — i.e., the executor identified this exact gap during
      implementation and chose not to close it. No test anywhere in the repository calls
      `execute()` on an `LlmRouting` Grove with a configured `llm_port` and an absent
      `routing_model` and asserts an error. This independently reproduces `06-REVIEW.md`'s CR-01,
      which the orchestrator also confirmed by reading the code.
      Nuance for the record: the *happy path* half of criterion 1 is real and reachable —
      `test_grove_llm_routing_end_to_end` proves a correctly-configured Grove routes its
      configured model through `execute()` end to end (Test 1-3 all pass with `.routing_model("gpt-4")`
      set). Only the D-02 hard-error/no-fallback guarantee is unreachable from the public API.
    artifacts:
      - path: "crates/paladin-battalion/src/grove_service.rs"
        issue: "`route_task` (lines 240-290) treats `route_by_llm`'s deliberate missing-config `RoutingError` identically to a transient routing failure and silently substitutes a fallback agent, defeating D-02's 'no fallback of any kind' guarantee for every real caller of `execute()`."
      - path: ".planning/decisions/0013-grove-routing-model.md"
        issue: "States as fact that 'a Grove using RoutingStrategy::LlmRouting today works (silently, against gpt-4) and starts returning BattalionError::RoutingError after this change until its configuration names a model' and that operators 'must set routing_model... or routing now returns BattalionError::RoutingError' — this describes behaviour that does not occur through the only public entry point (`execute()`); an operator will instead see a silent fallback with no error."
      - path: "CHANGELOG.md"
        issue: "The `## [Unreleased]` entry (lines 19-29) makes the identical unreachable claim: 'until it does [set routing_model], LLM-based routing returns BattalionError::RoutingError.'"
      - path: ".planning/REQUIREMENTS.md"
        issue: "CLOSE-01's 2026-08-05 amendment (lines 498-514) states 'ROADMAP criteria 1 and 2 are both met' and cites the guard/tests as proof without noting the guard is unreachable from `execute()`, the requirement's own governing entry point."
    missing:
      - "A code fix that honours the D-02 decision the human already approved as locked (checkpoint:decision, plan 06-01 Task 1): `route_task` must not silently substitute a fallback agent when the underlying error is the deliberate missing-`routing_model` configuration error. Likely shape: either have `route_by_llm`'s missing-model guard short-circuit `route_task` before the generic fallback logic (e.g. a distinct error variant or an early return in `route_task` for this specific case), or otherwise make the no-fallback guarantee observable from `execute()`."
      - "A test that calls `GroveExecutionService::execute()` (not `route_by_llm` directly) on an `LlmRouting` Grove with a configured `llm_port` and an absent/blank `routing_model`, and asserts the call returns `Err(BattalionError::RoutingError(..))` — the currently-green `test_grove_llm_routing` either needs this assertion added or a sibling test needs to supply it, since as of this review that exact configuration (`LlmRouting`, no `routing_model`, real `llm_port`) has never been driven through `execute()` and asserted to fail."
      - "Once the code is fixed (or if the team instead decides to soften D-02 with a human decision), reconcile ADR-0013, CHANGELOG.md, and REQUIREMENTS.md's CLOSE-01 entry with whatever the actually-shipped, `execute()`-reachable behaviour is."
---

# Phase 6: Verified Gap Closure Verification Report

**Phase Goal:** Every Milestone 2-3 gap that verification actually proved is closed or explicitly
deferred with a recorded reason — and no shipped surface is removed without a decision behind it.
**Verified:** 2026-08-05T21:00:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A correctly-configured Grove (routing_model set, LlmRouting strategy) routes its configured, non-OpenAI-capable model through `LlmPort::generate` via the public `execute()` entry point | ✓ VERIFIED | `tests/integration/battalion/grove_integration_test.rs::test_grove_llm_routing_end_to_end` drives `service.execute(...)` three times over a Grove built with `.routing_model("gpt-4")`, asserting correct agent selection each time; `grove_service.rs:554-556` sources `LlmRequest.model` from `grove.node.config.routing_model`, not a literal |
| 2 | No OpenAI model literal remains in `grove_service.rs`'s production region | ✓ VERIFIED | `awk '/^#\[cfg\(test\)\]/{exit} {print}' crates/paladin-battalion/src/grove_service.rs \| grep -c gpt-4` → 0 (re-run during this verification) |
| 3 | Grove LLM routing hard-errors with **no fallback of any kind** when `routing_model` is absent, and that guarantee is reachable from `GroveExecutionService::execute()`, the only public entry point (D-02) | ✗ FAILED | See gap above. `route_by_llm` itself errors correctly; `route_task`'s blanket `Err` handler (lines 256-289) swallows that error and substitutes a fallback agent before it ever reaches `execute()`'s caller. `test_grove_llm_routing` is green while calling `execute()` on exactly this misconfiguration and asserting `Ok`. |
| 4 | `grep -rn 'TODO' crates/paladin-battalion/src/` returns nothing Epic 22's completion criteria claimed already resolved | ✓ VERIFIED | 5 TODOs remain, all in `commander.rs` (lines 3128, 3161, 3201, 3242) and `council_service.rs` (line 733); all confirmed inside each file's `#[cfg(test)]` module (commander.rs test module starts line 1613, council_service.rs at line 521) — test-code only, matching `06-CONTEXT.md`'s `<deferred>` disposition |
| 5 | Every VERIFY-02 item genuinely outstanding across Epics 14, 22, 24 is closed or explicitly deferred with a recorded reason; Epic 22's "nothing outstanding" verdict is itself recorded, not dropped | ✓ VERIFIED | Epic 14 cluster 8.0 (autonomous YAML+CLI) closed by 06-03 — `PaladinYamlConfig.autonomous: Option<AutonomousConfig>` at `paladin_config.rs:111`, applied via `apply_autonomous_config` (`agent.rs:553`), overrides wired at `agent.rs:576-586`. Epic 24 cluster 1.0 (ChainOfCommand benchmark) closed by 06-04 — `benchmark_chain_of_command` registered in `criterion_group!` (`battalion_benchmarks.rs:225-230`), measured run recorded at `performance-baseline.md:3` (`## Run — 2026-08-05`, dated separately from the 2026-08-02 table). Epic 24 cluster 8.0 (3 CI jobs) recorded deferred with reason, bidirectionally, in `.planning/ledgers/milestone-02-03.md` and `.planning/REQUIREMENTS.md` PIPE-01/PIPE-02 (D-09/D-10) — no `.github/` file touched (`git log --oneline -- .github/` shows no phase-6 commits). Epic 22 "no work required" verdict carried into `.planning/REQUIREMENTS.md` CLOSE-02 text (lines 547-556) and `milestone-02-03.md:163`, not left ledger-only |
| 6 | WARN-01 (Herald reachability) adopted across `campaign_service.rs`, `chain_of_command_service.rs`, `commander.rs`, proved by one executable composite witness | ✓ VERIFIED | `herald: Option<Arc<dyn Herald>>` + `with_herald()` + `format_result()` present in all three files (grep confirmed); `tests/integration/battalion_chain_of_command_herald_test.rs::chain_of_command_result_renders_through_json_herald` drives a real `ChainOfCommandExecutionService::execute` and formats through a real `JsonHerald`, registered in `tests/integration/mod.rs` |
| 7 | Live-API harness (ADR-0012) matches its Phase 5 recorded decision in code: doc-only correction, `require_api_key` panic stands | ✓ VERIFIED | `tests/integration/llm_live_api_tests.rs:61` doc comment corrected (no longer claims "skip"); `tests/integration/mod.rs:34-35` feature gate documented; `require_api_key`'s panic behaviour unchanged (no `.rs` behavioural diff in this function per plan 06-05 scope) |
| 8 | Both vision surfaces (ADR-0011) match their Phase 5 recorded decision: both retained, entry-point rustdoc added, encryption disposition (D-16/D-17) documented, no trait removed | ✓ VERIFIED | `VisionPort` and `VisionCapableLlm` both still `pub trait`, no `#[deprecated]` (`grep -rn '#\[deprecated'` → 0); entry-point rustdoc present at `vision_port.rs:49` and `vision_llm_port.rs:54`; ADR-0011 amended in place with a dated 2026-08-05 resolution note and `## Code Conformance` reflecting the doc-only outcome |
| 9 | No shipped surface is removed without a recorded decision behind it | ✓ VERIFIED | No `.rs` public API removals found in this phase's diff beyond the documented additive changes; `.github/` untouched (D-11 constraint honoured, confirmed via git log) |
| 10 | ADR-0013 and CHANGELOG.md accurately describe the shipped, `execute()`-reachable runtime behaviour of the CLOSE-01 fix | ✗ FAILED | Both documents assert operators "must set `routing_model`... or routing now returns `BattalionError::RoutingError`" — this is not true through `execute()`, the only path a real operator uses (see truth 3). The documents describe the *intended* design (which the human approved at the Task 1 checkpoint) rather than the *shipped* behaviour. |

**Score:** 8/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/paladin-core/src/platform/container/battalion/grove.rs` | `routing_model: Option<String>` on `GroveConfig`, threaded through `GroveBuilder` | ✓ VERIFIED | Field at line ~209, serde-additive, `GroveBuilder::routing_model(..)` setter present, `Default` sets `None` |
| `crates/paladin-battalion/src/grove_service.rs` | Missing-model guard + config-sourced `LlmRequest.model` inside `route_by_llm` | ✓ VERIFIED (guard itself) / ⚠️ ORPHANED (guard's no-fallback guarantee, from the caller's perspective) | Guard is correct in isolation (lines 493-510) but its effect is discarded by `route_task`'s catch-all before reaching any caller of `execute()` |
| `src/application/cli/config/paladin_config.rs` | `autonomous: Option<AutonomousConfig>` section | ✓ VERIFIED | Line 111, reuses `paladin-core`'s `AutonomousConfig` directly, validated at line 280 |
| `crates/paladin-battalion/src/campaign_service.rs`, `chain_of_command_service.rs`, `commander.rs` | Herald triad | ✓ VERIFIED | `herald` field, `with_herald`, `format_result` present in all three |
| `crates/paladin-battalion/benches/battalion_benchmarks.rs` | `benchmark_chain_of_command`, registered | ✓ VERIFIED | Line 160 function, line 225-230 `criterion_group!` registration |
| `.planning/decisions/0013-grove-routing-model.md` | New ADR recording the break | ✓ EXISTS, but ⚠️ describes unreachable behaviour | See truth 10 |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `GroveConfig.routing_model` | `LlmRequest.model` (happy path) | `GroveBuilder` → `grove_service::route_by_llm` | ✓ WIRED | Confirmed by `test_llm_routing_uses_configured_routing_model` and `test_grove_llm_routing_end_to_end` |
| `route_by_llm`'s missing-model `RoutingError` | `GroveExecutionService::execute()`'s return value | `route_task` | ✗ NOT WIRED | `route_task`'s blanket fallback intercepts and discards the error before it reaches `execute()`'s caller (see gap) |
| `ChainOfCommandExecutionService::execute` | `Herald::format_battalion_result` | `to_battalion_result` → `format_result` | ✓ WIRED | Composite integration test exercises the full chain |
| `paladin.yaml` `autonomous:` section | `PaladinBuilder` | `PaladinYamlConfig.autonomous` → `apply_autonomous_config` | ✓ WIRED | `agent.rs:350` calls `apply_autonomous_config(builder, &config, &args)` |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| No `gpt-4` literal in grove_service.rs production region | `awk '/^#\[cfg\(test\)\]/{exit}{print}' grove_service.rs \| grep -c gpt-4` | 0 | ✓ PASS |
| No TODO in grove_service.rs | `grep -rn TODO crates/paladin-battalion/src/ \| grep -c grove_service.rs` | 0 | ✓ PASS |
| Existing test proves the swallowed-error defect | `grove_integration_test.rs::test_grove_llm_routing` (existing, not run — reasoned from source) | Builds `LlmRouting` Grove with no `routing_model`, calls `execute()`, asserts `Ok` | ✗ FAIL (as evidence of the gap, not of a broken build) |
| Benchmark compiles | `cargo bench --no-run -p paladin-battalion` (per 06-04-SUMMARY, not re-run here — build/test already green per task's build_state) | exit 0 (reported) | ✓ PASS (relied on reported build state) |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| CLOSE-01 | 06-01, 06-06, 06-07 | Grove routing uses the LLM model from configuration instead of a hardcoded literal | ✗ BLOCKED (partial) | Happy path genuinely fixed and wired; the hard-error/no-fallback half of the design (D-02) is implemented but unreachable from the only public entry point, and ADR-0013/CHANGELOG/REQUIREMENTS.md's satisfaction claims overstate what is actually reachable |
| CLOSE-02 | 06-02, 06-03, 06-04, 06-07 | VERIFY-02 genuinely-outstanding items in Epics 14/22/24 closed or deferred with reason | ✓ SATISFIED | All four items disposed (a-d in REQUIREMENTS.md), each independently verified above |
| CLOSE-03 | 06-05, 06-07 | Phase 5 ADR code consequences applied (vision, live-API harness) | ✓ SATISFIED | Both ADR-0011 and ADR-0012 dispositions verified in code and docs |

No orphaned requirement IDs found — all three CLOSE-* IDs declared across plan frontmatter match `.planning/REQUIREMENTS.md`'s CLOSE section.

### Anti-Patterns Found

No `TBD`/`FIXME`/`XXX`/`TODO` debt markers in any file modified by this phase's plans (grove.rs, grove_service.rs, campaign_service.rs, chain_of_command_service.rs, commander.rs, paladin_config.rs, agent.rs, battalion_benchmarks.rs, vision_port.rs, vision_llm_port.rs, llm_live_api_tests.rs — all checked, all clean).

The gap identified above is not a code-smell anti-pattern (no placeholder, no stub, no empty implementation) — it is a genuine control-flow wiring defect: a correctly-implemented guard whose effect is discarded by a pre-existing, unmodified-by-this-phase catch-all one call frame up (`route_task`, which this phase did not touch). Severity: 🛑 Blocker, because the phase's own locked design decision (D-02, human-approved at a `checkpoint:decision` gate as `proceed-as-locked`) is not actually in effect for any real caller, and three documents (ADR-0013, CHANGELOG.md, REQUIREMENTS.md) now assert in writing that it is.

### Human Verification Required

None. This finding is fully provable by static code-path analysis (`route_task`'s blanket `match result { Err(e) => ... }` unconditionally intercepts `route_by_llm`'s error before `execute()` returns, and `GroveBuilder::build()` guarantees the terminal fallback always succeeds) and by an existing, currently-green test that directly demonstrates the swallowed behavior. No runtime/visual/external-service judgment is needed to close this gap — it is a deterministic code fix plus a missing test.

### Gaps Summary

Eight of the ten observable truths derived from the four ROADMAP success criteria are genuinely
met: the CLOSE-02 and CLOSE-03 halves of this phase (Herald reachability, autonomous CLI/YAML
wiring, the ChainOfCommand benchmark, the vision and live-API-harness documentation work) are all
implemented, wired, and independently re-verified against the tree rather than taken on
SUMMARY.md's word.

CLOSE-01 — the phase's headline defect and the reason ROADMAP success criteria 1 and 2 exist — is
**half-closed**. The happy path (a correctly-configured Grove routes its configured model through
`LlmPort::generate` via `execute()`) is real and independently confirmed. But the other, equally
load-bearing half of the design — that a *misconfigured* Grove (`LlmRouting` with no
`routing_model`) fails loudly with no fallback, per D-02's explicit, human-approved, "one-way"
design decision — is not reachable from `GroveExecutionService::execute()`, the only public entry
point. `route_task`'s pre-existing blanket fallback (unmodified by this phase) intercepts the new
guard's error and silently substitutes default agent selection instead. A currently-green
integration test (`test_grove_llm_routing`) directly demonstrates this: it builds exactly the
misconfiguration D-02 was meant to reject, calls `execute()`, and asserts success.

This independently reproduces `06-REVIEW.md`'s CR-01 finding, which the orchestrator had already
confirmed by reading the code before this verification ran. Because ADR-0013, `CHANGELOG.md`, and
`.planning/REQUIREMENTS.md`'s CLOSE-01 closure text all now assert — in writing, as fact — that
"routing now returns `BattalionError::RoutingError`" when misconfigured, and that "ROADMAP
criteria 1 and 2 are both met," this is not a cosmetic gap: three permanent project records
describe shipped behavior that does not exist. Closing this phase's gap-closure purpose requires
either (a) a code fix making the no-fallback guarantee actually reachable from `execute()`, which
is what the human already approved at the Task 1 checkpoint, plus a test that exercises `execute()`
(not `route_by_llm` directly) with the misconfiguration, or (b) a fresh human decision to soften
D-02 and then correcting ADR-0013/CHANGELOG/REQUIREMENTS.md to match whatever is actually shipped.
Given the checkpoint already recorded `proceed-as-locked`, (a) is the path consistent with the
recorded decision.

No deferred items apply here (CLOSE-01 is fully in this phase's scope, not addressed by any later
milestone phase).

---

*Verified: 2026-08-05T21:00:00Z*
*Verifier: Claude (gsd-verifier)*
