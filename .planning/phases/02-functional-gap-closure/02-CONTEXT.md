# Phase 2: Functional Gap Closure - Context

**Gathered:** 2026-07-31
**Status:** Ready for planning

<domain>
## Phase Boundary

Every Milestone-1 functional requirement is either working and tested, or explicitly deferred with
a recorded reason — and the shipped types match the Phase 1 ADRs.

**This is the first phase in the roadmap that edits Rust source.** Phase 1 wrote records and
decisions and explicitly changed no product code; Phase 2 executes the three ADRs whose
`Code conformance` is `must change`, and closes the functional residue the Phase 1 ledger exposed.

**Three deliverable classes:**

1. **The GAP-07 code changes** — the three ADR-mandated edits, plus Phase 14's WEB-03 pulled
   forward into the same change (see D-13).
2. **Test closure** — writing the missing exercisers that turn `present, unproven` and
   `genuinely outstanding` ledger items into `satisfied`, within the scope boundary D-09 sets.
3. **Recorded verdicts** — a measured `cargo test --workspace` baseline, GAP-06's PRD-acceptance
   review, and ledger amendments for everything deferred or overturned.

**The reframe that governs this phase.** Phase 2's five ROADMAP success criteria were written
before the Phase 1 ledger existed, and the ledger's 2026-07-31 re-verification overtook three of
them. `test_auto_selects_campaign_for_workflow_keywords` (SC1's named failure) **passes today**.
Chain of Command (SC2) is `satisfied` with 54/54 passing and `examples/chain_of_command_delegation.rs`
shipping. Commander result normalization and `export_metadata` telemetry (SC4) are `satisfied`.
What actually remains is the three GAP-07 edits (SC5), the Battalion→Herald end-to-end gap (SC3),
and a defined set of `present, unproven` / `genuinely outstanding` items. **Do not plan SC1, SC2 or
SC4 as build work** — plan them as the executable re-proof D-01 requires.

**Not in this phase:** raising coverage to ADR-0006's 84% floor (Phase 3, QUAL-01); un-ignoring the
four empty-bodied Commander error tests (Phase 3, QUAL-04); MCP failure-mode tests (Phase 3,
QUAL-03); CI workflow changes (Phase 15, PIPE); the live-API harness skip-vs-fail semantics
(Phase 5, VERIFY-06); WEB-04's build-or-withdraw decision on LLM tool calling (Phase 14).

</domain>

<decisions>
## Implementation Decisions

### Scope posture toward the Phase 1 ledger

- **D-01: Re-prove by execution, not by citation.** The ledger is the map, not the proof. Phase 2
  opens with a measured `cargo test --workspace` run recorded the way Phase 1 recorded its coverage
  measurement — raw pasted stdout, `rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `date -u`
  — and each of SC1, SC2 and SC4 gets one executable check whose output is pasted into the phase
  record. This converts three ROADMAP criteria from "the ledger says so" to "this command said so
  on this commit."
  Chosen over trusting the ledger outright (no single command has ever been shown to pass
  workspace-wide, so SC1 would remain an inference) and over a baseline-run-only middle path.
  Rationale the user endorsed: this corpus's own repeated finding is that checkbox-and-citation
  evidence has been wrong **in both directions**.

- **D-02: On disagreement, amend `milestone-01.md` in place.** When a measured result contradicts a
  ledger verdict, Phase 2 edits the contradicted row with the new verdict, the command that
  produced it, and the date — same D-19 evidence bar, same D-20 verdict classes. The ledger is a
  living record, not a frozen Phase 1 artifact. This sets the convention Phases 5, 7, 10 and 13
  inherit for their own ledgers.
  Chosen over a separate Phase 2 corrections file (a reader would need two documents to know a
  row's current verdict, and four sibling phases would inherit that split).
  — **Reversibility:** costly — four sibling ground-truth phases author their ledgers against this
  convention, and per-row provenance must be dated once Phase 2 has edited a Phase 1 artifact.

- **D-03: Deferrals are `deferred with reason` ledger rows.** D-20 already defines the verdict
  class and the ledger already carries one such row (Garrison's `test_large_conversation_performance`).
  A Phase 2 deferral is a ledger row amendment citing the deferring authority and a named forward
  owner. No new document class; it lands where a reader already looks.
  Chosen over an ADR per deferral (D-04 scoped ADRs to competing variant pairs; a deferral is a
  scope call, not a variant resolution, and minting ADRs for scope calls dilutes what an ADR means).
  **Exception:** where a deferral *overrides a written requirement*, it escalates to an ADR — see
  D-08.

- **D-04: GAP-06 closes as `superseded` + a real review.** Epic 2 task 11.5 ("verify coverage ≥ 80%
  using `cargo llvm-cov`") is recorded `superseded by shipped code`: ADR-0006's workspace-wide 84%
  floor replaced the per-module 80% target by decision (D-08 of Phase 1), and re-measuring Garrison
  alone would reintroduce the second scope that ADR exists to eliminate. Task 11.6 is done
  properly — a written pass over Epic 2's PRD acceptance criteria against shipped code, one verdict
  per criterion at the D-19 bar, landing as the Garrison ledger rows' evidence.
  Chosen over producing an advisory Garrison-scoped figure (an advisory number in this record gets
  read as a gate later, which is exactly the failure ADR-0006 ended) and over folding 11.5 into the
  11.6 review narrative (the coverage disposition would be buried rather than carrying its own
  ledger verdict).

### The `present, unproven` line

- **D-05: `REQ-battalion-cancellation` is recorded Phalanx-only; the other three patterns are
  deferred.** `execute_with_cancellation` exists only on Phalanx (`phalanx_service.rs:151`, tested
  at `:758`); Formation, Campaign and ChainOfCommand expose no cancellation entry point at all.
  Building it for three more patterns is feature work, not gap closure — it needs a cancellation
  contract across four execution services, including what a cancelled run returns mid-DAG in
  Campaign and mid-delegation in ChainOfCommand. The Phalanx half stays `satisfied`; the rest is
  `deferred with reason` with a named forward owner. The ROADMAP's Phase 2 criteria never mention
  cancellation, which corroborates that this was never Phase 2's scope.
  Chosen over implementing across all four (the single largest thing in Phase 2, and unbudgeted)
  and over a Formation-only half-measure.
  **This contradicts `REQ-battalion-cancellation` as written, so per D-03's exception it escalates
  to an ADR** — see D-08.

- **D-06: SC3 closes with one Formation-driven end-to-end test file, three Heralds.** Drive a real
  `FormationExecutionService` run with mock Paladins, pipe the resulting `BattalionResult` through
  the JSON, Markdown and Table Heralds, and assert SC3's five named content requirements —
  Battalion name/ID/type, per-Paladin results in execution order, aggregated token usage, and
  partial results — including one deliberately-failed Paladin for the partial-results case. This
  closes Epic 8 task 7.13 and SC3 together.
  Chosen over the full 3 Heralds × 4 patterns matrix (`BattalionResult` is a single merged type, so
  a Herald cannot tell which pattern produced it — eleven of twelve combinations add nothing) and
  over extending the existing Herald unit tests with a hand-constructed struct (task 7.13's own
  note names the gap as "needs Battalion execution setup", so hand-construction is precisely what
  it declared insufficient — that route closes SC3 on paper and leaves 7.13 `present, unproven`).

- **D-07: Existence in Phase 2, depth in Phase 3.** Phase 2 proves each of the four Battalion
  patterns has a passing, non-`#[ignore]`d integration exerciser and that the Phalanx performance
  claims hold (≥ 10 concurrent Paladins, < 1 s orchestration overhead — `tests/integration/battalion/load_test.rs`
  already covers this). Phase 3 owns raising coverage (QUAL-01/02), un-ignoring the four
  empty-bodied Commander error tests at `commander.rs:2180,2188,2196,2204` (QUAL-04), and MCP
  failure-mode tests (QUAL-04). This honours the ledger's own forward-owner assignments so nothing
  is planned twice.
  Chosen over pulling the four `#[ignore]`d tests into Phase 2 and over building the shared
  failing-mock harness here. **Note for the planner:** the shared `Send + Sync` failing-mock asset
  those tests need is also what Phase 15's DEFER register names as a prerequisite three registers
  have asked for and none has built — Phase 3 should be told it is building a shared asset, not a
  local one.

- **D-08: An ADR records the cancellation deferral.** Per D-03's exception, D-05 overrides
  `REQ-battalion-cancellation` as written and therefore gets an ADR in `.planning/decisions/`,
  numbered after 0006, recording: the requirement text, the Phalanx-only shipped reality, why a
  four-pattern cancellation contract is out of Phase 2's scope, and the forward owner. Written to
  ADR-0004's shape (`Status · Date · Context · Decision · Considered Options · Code Locations ·
  Code Conformance · Downstream Consumers`) so `adr-parser.cjs` parses it.

### Genuinely-outstanding clusters

- **D-09: CLI tests in, provider-switching in, CI config out.** Epic 9 tasks 13.4-13.6 (CLI-level
  tests running Paladin / Formation / Phalanx through a mock LLM — the blocking mock provider
  shipped, the tests were simply never written) and Epic 6 tasks 7.10-7.12 (provider-switching
  integration tests, which exist nowhere in the tree under any name) are in scope: they are
  genuinely-missing functional tests inside Milestone 1 and are what "working and tested" means for
  GAP-01/GAP-02. Epic 6 task 7.14 (CI configuration for the `live-api-tests` feature — currently
  zero matches in `.github/workflows/`) is **out**: it is a CI-workflow change, which is Phase 15's
  PIPE territory, and VERIFY-06 in Phase 5 has not yet decided whether a keyless run should fail
  loudly or skip cleanly — which is exactly what such a job would encode. It gets a
  `deferred with reason` row naming PIPE as owner.

### The dead `tests/unit/llm/` module

- **D-10: Wire it in and fix it, with a per-file fallback rule.** `tests/unit/llm/` holds 25 test
  functions across three mockito-based files, and its own `mod.rs` declares all three — but
  `tests/unit/mod.rs` never declares `pub mod llm;`, so none has ever compiled or run. Epic 6 task
  6.0 is checked `[x]` on the strength of them. Phase 2 adds the missing declaration, compiles, and
  repairs the fallout, because these tests cover HTTP-level paths (401 → `AuthenticationError`,
  429 → `RateLimitExceeded`, streaming) that the live 67 `paladin-llm` tests do not reach, and
  Phase 3's QUAL-02 names `deepseek_adapter.rs` at 15.02% — deleting this would mean rewriting it
  there.

- **D-11: The fallback rule is per-file and keyed on failure kind, not a clock.**
  **Mechanical breakage** — import paths, renamed types, `mockito::Server::new()` needing
  `new_async()` inside `#[tokio::test]`, signature drift — **gets fixed**, however many
  occurrences. **Structural breakage** — the test asserts behaviour the adapter no longer has, or
  needs a type the `paladin-ports` extraction removed — means those specific tests are **deleted
  with a per-test `superseded by shipped code` note naming which behaviour vanished.** Judged per
  file, so two easily-repaired files are not discarded because a third was hard. The executor
  applies this rule without escalating.
  **Grounding for the planner:** the directory's last two commits are `be131f3` (paladin-ports
  extraction) and `69a15af` (facade rewiring) — both landed *after* `4761762 feat: llm unit tests`
  and neither could have compiled against these files, so some breakage is certain.

- **D-12: One sweep for other never-compiled test source.** Cross-check every directory under
  `tests/` against the `[[test]]` targets in `Cargo.toml` and every barrel `mod.rs`, and report
  anything else that is never compiled into a target. This is cheap (a directory listing against a
  declaration list) and directly serves the phase goal — test source that never runs is the exact
  shape of "believed working, actually not" this milestone closes. Findings get ledger rows.
  **Fixing anything found beyond the LLM module is a separate call, not automatic** — the sweep's
  result size is unknown before it runs, and committing to repair it sight-unseen is the unbounded
  trap D-11 exists to avoid.

### GAP-07 and Phase 14's WEB-03

- **D-13: WEB-03 is pulled forward into GAP-07 and lands in the same change.** ADR-0004 states the
  two "must not be scheduled independently" — both edit `ProviderCapabilities` at
  `crates/paladin-ports/src/output/llm_port.rs:754` and every adapter's construction site — and the
  ROADMAP already records WEB-03 as depending on nothing and being small. Phase 2 adds
  `temperature_range` per ADR-0004 **and** corrects `supports_tool_calling` per WEB-03, touching
  each construction site exactly once. Phase 14 then records WEB-03 as satisfied by Phase 2 rather
  than re-planning it.
  — **Reversibility:** costly — `ProviderCapabilities` is a published ports-layer type on the
  framework's primary integration contract, re-exported through the prelude; adding a field is
  additive, but every adapter must populate it and downstream consumers gain a field they may
  branch on.

- **D-14: `supports_tool_calling` becomes `false` on all three adapters, with a correspondence
  test.** *Verified during this discussion:* `LlmRequest` (`llm_port.rs:464`) has **no tools field**
  — its fields are `id`, `model`, `prompt`, `attachments`, `stream`, `metadata` — and neither the
  OpenAI nor the Anthropic adapter sends `tools` or parses `tool_calls` (grep returns zero matches
  in both). Yet `openai/adapter.rs:645` and `anthropic/adapter.rs:521` both report `true`, with
  passing tests asserting it at `:703` and `:751`. DeepSeek already reports `false`
  (`deepseek/adapter.rs:562`). The flag describes what **this adapter does**, not what the vendor's
  API offers; nothing in the workspace can express a tool call, so all three report `false`, the
  two asserting tests flip, and a new test asserts the flag matches whether a tool-calling request
  path exists — satisfying WEB-03's success criterion 3 ("a test asserting the correspondence")
  literally.
  Chosen over redefining the field to mean "the provider supports it" (WEB-03's own criterion says
  the flag must match what the adapters actually do, so that answers the requirement by redefining
  it, and a consumer branching on it still gets a capability it cannot use) and over splitting into
  `provider_supports_tool_calling` + `supports_tool_calling` (adds a second field to a published
  ports type in the same change, and pre-builds for WEB-04's build-or-withdraw outcome, which is
  explicitly Phase 14's to choose).

- **D-15: Real adapters populate `temperature_range`; every other construction site takes `None`.**
  The three shipped LLM adapters declare their true ranges — OpenAI and Anthropic `[0.0, 1.0]`,
  DeepSeek `[0.0, 2.0]` per Epic 6 REQ-5, which is the capability ADR-0004 exists to make reachable
  through the normal Paladin path. Every other site is a test fixture or an in-service default and
  gets `temperature_range: None`, which ADR-0004 defines as the named `[0.0, 1.0]` fallback —
  behaviour identical to today. Use `..Default::default()` where a site already permits it so the
  field addition stays mechanical.
  **The full construction-site list** (verified by grep during this discussion, beyond the three
  adapters): `crates/paladin-battalion/src/grove_service.rs:1110`,
  `crates/paladin-llm/src/mock.rs:267,378`,
  `src/application/services/paladin/temperature_service.rs:356`,
  `src/application/services/paladin/planning_service.rs:641`,
  `src/application/services/paladin/prompt_generation_service.rs:299`,
  `src/application/services/paladin/paladin_execution_service.rs:2541,2631,2715`, plus the doc and
  test sites inside `llm_port.rs` itself (`:190,741,775,1287,1304,1326,1336`).
  Chosen over every site declaring a range explicitly (hard-codes a range into fixtures that do not
  care about temperature, and ships the `None` fallback path untested despite it being the contract
  for every future adapter) and over adding `#[non_exhaustive]` + `Default` to solve the class
  (breaking for downstream constructors — a Phase 4 / REL-01 version decision, not Phase 2's).

- **D-16: The ports change lands first, as a tracer.** `ProviderCapabilities` has the widest blast
  radius and is the change that proves this phase can safely edit a published ports type — land it
  first with WEB-03, the three adapters, and the correspondence test. The `citadel.rs` rename and
  the Formation bound follow independently. Mirrors Phase 1's own shape, where plan 01-01 was an
  explicit end-to-end tracer before the fan-out.
  Chosen over cheapest-first (Formation, rename, ports — puts the widest change last, when there is
  least room to absorb surprises) and over all three in one plan (a single plan mixing a
  persisted-schema rename, a validation-semantics change and a published-type extension is hard to
  review and hard to revert selectively).

### Claude's Discretion

- **The renamed identifier for the `citadel.rs:280` struct.** `BattalionCheckpointConfig` was
  Phase 1's discussion example, explicitly not a locked choice (Phase 1 CONTEXT.md's own
  discretion list). ADR-0001's constraints bind: keep the three fields (`max_concurrency`,
  `timeout_seconds`, `continue_on_error`) and the serde shape; no persisted-schema change and no
  migration, because `BattalionState.config` (`citadel.rs:233`) is consumed by
  `crates/paladin-memory/src/citadel/file_citadel.rs:507,541` at `schema_version: "1.0.0"`.
- **Plan decomposition and count.** D-16 fixes only that the ports change is the tracer. How the
  test-closure work (D-06, D-07, D-09, D-10) splits across plans, and whether the baseline run
  (D-01) is its own plan or the tracer's first task, is the planner's call.
- **Whether ROADMAP.md's Phase 2 success-criteria wording is amended at source.** The ledger has
  overtaken SC1, SC2 and SC4 as written. Phase 1 set the precedent of correcting the roadmap at
  source (it amended Phase 3's criterion 1 for D-08). Not discussed; the planner should decide and
  record whichever way it goes. Note the criteria still describe the right *outcomes* — only their
  premise ("which fails today") is stale.
- **How the GAP-05 finding is recorded.** SC1 names `test_auto_selects_campaign_for_workflow_keywords`
  as failing; it passes. The ledger already records this at `milestone-01.md:316-317` as
  `satisfied` with the checkbox marked stale. Whether Phase 2 restates it, and where, is open.
- **`REQ-provider-error-mapping`'s dead conversion path.** The ledger records a finding: a
  `LlmProviderError` type with a `From<LlmProviderError> for LlmError` impl exists at
  `crates/paladin-llm/src/error.rs:16,54`, but grep finds zero constructors outside that file — the
  documented conversion path is dead while each adapter maps status codes directly at the call
  site. Not discussed. Related to D-12's dead-code theme; the planner may fold it into the sweep's
  findings or leave it to Phase 3.
- **Whether the sweep (D-12) also covers `benches/` and `examples/`.** Not discussed. Phase 4's SC5
  requires all 22 examples to build in CI, so an examples orphan would surface there anyway.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Binding decisions — the ADRs this phase executes

- `.planning/decisions/0001-battalion-config.md` — `Code conformance: must change`. The
  `citadel.rs:280` placeholder rename: keep three fields and serde shape, no migration, distinct
  name because it is a different concept (checkpoint/resume knobs, not orchestration config).
- `.planning/decisions/0003-formation-min-paladins.md` — `Code conformance: must change`.
  `Formation::validate` relaxes to ≥ 1. Boundary stated explicitly: 0 still rejected, exactly 1 now
  accepted, 2 unchanged. `analyze_and_select` and its passing test are untouched; Phalanx's Majority
  ≥ 3 is untouched.
- `.planning/decisions/0004-temperature-validation.md` — `Code conformance: must change`.
  `temperature_range: Option<(f32, f32)>`, both endpoints inclusive (`t >= min && t <= max`), no
  epsilon and no rounding; `None` → the existing `[0.0, 1.0]` fallback; provider range checked
  **first**, autonomous task-type bands clamp **within** it. **Carries the WEB-03 sequencing note
  D-13 acts on.**
- `.planning/decisions/0002-battalion-result.md` — `Code conformance: conforms`. **No code change.**
  Do not plan a `BattalionResult` reconciliation; `intel/code-verification.md` carries a standing
  instruction to the same effect.
- `.planning/decisions/0005-herald-trait.md` — `Code conformance: conforms`. **No code change.** The
  shipped trait is the v2 fallible form; `format_error`'s infallibility is deliberate (it is what
  makes FR-10 graceful degradation expressible), not an oversight to fix.
- `.planning/decisions/0006-coverage-gate.md` — the single 84% hard-fail floor from a measured
  84.79%. **Constrains D-04**: no second, module-scoped coverage number may be created.
- `.planning/decisions/PROMOTION.md` — the ADR promotion procedure, if D-08's new ADR needs the
  house conventions.

### The evidence base this phase re-proves and amends

- `.planning/ledgers/milestone-01.md` — the cited status ledger. **The single most important input.**
  Phase 2 amends it in place per D-02. Sections that matter most: Epic 4 (`REQ-battalion-cancellation`,
  the two stale parent checkboxes), Epic 5 (`REQ-formation-min-paladins-v2` → GAP-07,
  `REQ-commander-auto-selection`'s GAP-05 finding, the four `#[ignore]`d error tests), Epic 6
  (`REQ-provider-testing`'s dead-module finding, the 19 nested Epic 6 items), Epic 8 (task 7.13),
  Epic 9 (tasks 13.4-13.6), and `## Outstanding item reconciliation` (39/39, verdict distribution).
- `.planning/ledgers/milestone-01.md` §"Verdict legend" — D-19's evidence bar and D-20's five
  verdict classes, which every Phase 2 amendment must satisfy.
- `.planning/phases/01-ground-truth-decision-records/01-CONTEXT.md` — Phase 1's D-01…D-21, the
  precedence order (D-02), the evidence bar (D-19) and the discretion list D-04 above inherits.
- `.planning/phases/01-ground-truth-decision-records/01-VERIFICATION.md` — 5/5 verified; also the
  model for how a measured claim is recorded honestly (§"The measurement's honesty").
- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` — **the template
  D-01's baseline run should follow**: raw pasted stdout, full toolchain and commit provenance, and
  a human-confirmation section.

### Requirements and roadmap

- `.planning/REQUIREMENTS.md:195-235` — GAP-01 … GAP-07 in full, with `Derives:` provenance. Note
  GAP-07's run-3 narrowing: "one `BattalionResult`" and "one `BattalionConfig` field set" are
  already true and were dropped; **the `citadel.rs` duplicate is what survives**.
- `.planning/REQUIREMENTS.md:236-249` — QUAL-01 … QUAL-03, the Phase 3 boundary D-07 draws against.
- `.planning/ROADMAP.md` §"Phase 2: Functional Gap Closure" — the five success criteria (see the
  staleness note in `<domain>`).
- `.planning/ROADMAP.md` §"Phase 14: API Contract Truthfulness" — WEB-03's own success criterion 3,
  which D-14 must satisfy, and WEB-04, which stays in Phase 14.
- `.planning/ROADMAP.md` §Progress — the coupling notes, especially `RECON-07 → VERIFY-05 → PIPE-02`
  and the Phase 15 sequencing that D-07 and D-09 defer into.
- `.planning/PROJECT.md` §Context — the precedence order (ADR → shipped tree → `.planning/codebase/`
  map → `intel/code-verification.md` → PRD → DOC → task-list checkbox).
- `.planning/PROJECT.md` §Key Decisions — the six ADR rows, populated by Phase 1.

### Code-state intelligence

- `.planning/intel/code-verification.md` — direct code verification across all five ingest runs;
  third in the precedence order and the source of several "do not plan this" instructions.
- `.planning/codebase/TESTING.md` — the three-tier test strategy, the `tests/unit/mod.rs` barrel
  pattern D-10 repairs, mock conventions (`MockLlmAdapter`, `MockArsenalAdapter`,
  `MockPaladinPort`), and the `[[test]]` target declarations D-12's sweep checks against.
- `.planning/codebase/ARCHITECTURE.md` — the hexagonal boundaries the ports change in D-13 crosses.
- `.planning/codebase/CONCERNS.md` — existing error-handling violations and the edition mix.

### Shipped code this phase edits or asserts against

- `crates/paladin-ports/src/output/llm_port.rs:754` — `struct ProviderCapabilities`; D-13/D-14/D-15
  all land here.
- `crates/paladin-ports/src/output/llm_port.rs:464` — `struct LlmRequest`, **verified to have no
  tools field** — the evidence behind D-14.
- `crates/paladin-llm/src/openai/adapter.rs:645,703` — `supports_tool_calling: true` and the test
  asserting it; both flip under D-14.
- `crates/paladin-llm/src/anthropic/adapter.rs:521,751` — the same pair for Anthropic.
- `crates/paladin-llm/src/deepseek/adapter.rs:562,628` — already `false`; the range D-15 sets here is
  `[0.0, 2.0]` per Epic 6 REQ-5.
- `crates/paladin-core/src/platform/container/citadel.rs:280` — the Epic 4 placeholder
  `BattalionConfig` D-16's second change renames.
- `crates/paladin-core/src/platform/container/citadel.rs:233` — `BattalionState.config`, the
  persistence site that constrains the rename.
- `crates/paladin-memory/src/citadel/file_citadel.rs:507,541` — the other consumers of that shape.
- `crates/paladin-core/src/platform/container/battalion/formation.rs:109-111,173` — the ≥ 2
  rejection and the test asserting it; both change under ADR-0003.
- `crates/paladin-battalion/src/commander.rs:1912` — `test_auto_selects_formation_for_single_paladin`,
  the passing test that must **stay** passing and untouched.
- `crates/paladin-core/src/platform/container/battalion/phalanx.rs:141-146` — Majority's independent
  ≥ 3, explicitly **not** touched.
- `crates/paladin-battalion/src/formation_service.rs` — the execution service D-06's end-to-end test
  drives.
- `crates/paladin-herald/src/{json_herald.rs:73,markdown_herald.rs:105,table_herald.rs:63}` — the
  three Heralds D-06 pipes through.
- `crates/paladin-battalion/src/phalanx_service.rs:151,758` — the only shipped
  `execute_with_cancellation` and its test; the `satisfied` half D-05 preserves.
- `tests/unit/mod.rs` — the barrel missing `pub mod llm;` (D-10).
- `tests/unit/llm/{mod.rs,deepseek_adapter_test.rs,anthropic_adapter_test.rs,provider_factory_test.rs}`
  — 25 never-compiled test functions (D-10, D-11).
- `tests/integration/battalion/load_test.rs:102,192,273` — the existing load/stress tests that
  already satisfy GAP-02's Phalanx performance claims (D-07).
- `crates/paladin-llm/src/error.rs:16,54` — the dead `LlmProviderError` conversion path (discretion
  item).

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`MockLlmAdapter`** (`crates/paladin-llm/src/mock.rs:73`, and the parallel
  `tests/helpers/mock_llm_adapter.rs:66`) — the blocking dependency Epic 9 tasks 13.4-13.6 were
  deferred on, now shipped and tested. D-09's CLI tests build on it. It supports queued responses,
  tool-call responses, streaming and error injection, plus invocation tracking.
- **`tests/helpers/`** — `mock_arsenal_adapter.rs`, `mock_paladin_port.rs` and the barrel `mod.rs`
  already exist. `IntegrationMockPaladinPort` (`tests/integration/commander_integration_tests.rs:78`)
  is the pattern D-06's Formation-driven test should follow rather than inventing a new one.
- **`tests/integration/battalion/load_test.rs`** — five real, non-`#[ignore]`d load and stress tests
  that already validate Phalanx's ≥ 10 concurrent / < 1 s claims. GAP-02's performance half is
  largely already met; verify, do not rebuild.
- **The offline coverage pipeline from plan 01-09** — `rustc`/`llvm-profdata`/`llvm-cov`
  instrumentation that works without `cargo-llvm-cov` (which is not installable in this sandbox:
  crates.io returns HTTP 403 and there is no Docker). If any Phase 2 work needs a coverage figure,
  this is the path — do not re-hit the blocker plan 01-04 halted on.
- **`.claude/gsd-core/bin/lib/adr-parser.cjs`** — Phase 1 gated every ADR status flip on this
  parser exiting 0. D-08's new ADR should parse under it too.

### Established Patterns

- **Precedence is the project's core mechanic**: ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox. An ADR that contradicts shipped
  code is an instruction to change the code — which is exactly what GAP-07 is.
- **The D-19 evidence bar and D-20 verdict classes govern every ledger amendment.** `satisfied`
  needs a `file:line` **plus** a named passing exerciser. Do not upgrade a row on the strength of
  code existing.
- **Medieval military ubiquitous language is mandatory** in code, docs and comments — including new
  test names, the renamed `citadel.rs` struct, and the D-08 ADR.
- **Doc comments carry archaeology.** The `citadel.rs:280` placeholder was identified by its own doc
  comment naming Epic 4 as its replacement. When verifying, read the comments, not only signatures.
- **Test organization**: unit tests co-located in `#[cfg(test)] mod tests`, integration tests as
  `tests/integration/*_test.rs` declared as `[[test]]` targets in `Cargo.toml`, and the
  `tests/unit/mod.rs` barrel listing every unit module. **The barrel is the failure point D-10
  repairs and D-12 sweeps for.**
- **Repo working agreement**: `cargo test` → `cargo fmt --check` → `cargo clippy` before committing
  a parent task; no `unwrap()`/`expect()`/`panic!` in library code; conventional-commit messages.

### Integration Points

- **`crates/paladin-ports/src/output/llm_port.rs:754`** — one struct, three separate changes
  (`temperature_range`, `supports_tool_calling`, and every construction site). D-16 makes this the
  tracer for exactly this reason.
- **Phase 3's QUAL-01…QUAL-04** — receives coverage work, the four `#[ignore]`d Commander error
  tests, and MCP failure-mode tests per D-07. Also receives the shared failing-mock harness as a
  build item.
- **Phase 5's VERIFY-06** — owns the `llm_live_api_tests.rs` skip-vs-fail semantics
  (`require_api_key()` panics on a missing key today). D-09 defers Epic 6 task 7.14 to Phase 15 in
  part because VERIFY-06 has not decided this yet.
- **Phase 14's WEB-03** — satisfied by Phase 2 per D-13/D-14; Phase 14 should record it as such
  rather than re-plan it. **WEB-04 stays in Phase 14.**
- **Phase 15's PIPE** — receives Epic 6 task 7.14 (CI configuration for `live-api-tests`) per D-09.
- **`.planning/ledgers/milestone-01.md`** — amended in place by this phase (D-02), which sets the
  convention Phases 5, 7, 10 and 13 inherit.
- **`.planning/decisions/`** — gains one new ADR (D-08), numbered after 0006.

</code_context>

<specifics>
## Specific Ideas

- **Three findings verified live during this discussion. Treat them as established, not as
  hypotheses to re-check:**
  1. **`LlmRequest` has no tools field.** Its complete field set is `id`, `model`, `prompt`,
     `attachments`, `stream`, `metadata` (`llm_port.rs:464-477`), and grep for `tool_calls` or
     `tools:` in the OpenAI and Anthropic adapters returns **zero matches**. `supports_tool_calling:
     true` on both adapters is therefore a claim the port cannot express — this is what makes D-14
     an obvious correction rather than a judgement call.
  2. **`tests/unit/llm/` is real, complete, and structurally orphaned.** Its own `mod.rs` correctly
     declares all three test files; the omission is one missing `pub mod llm;` line in
     `tests/unit/mod.rs`. 25 test functions (8 DeepSeek + 9 Anthropic + 8 Factory), not the 27 the
     task list claims. The last two commits touching the directory are the `paladin-ports`
     extraction and the facade rewiring, both *after* the tests were written.
  3. **`ProviderCapabilities` has more construction sites than the three adapters** — ten more,
     enumerated in D-15. Any plan that says "update the three adapters" will not compile.

- **Phase 1's coverage-measurement record is the model for D-01's baseline run.** Raw pasted stdout
  ending in a real `TOTAL` row, `rustc -vV` / `cargo --version` / `git rev-parse HEAD` / `date -u`
  captured immediately before the command, and arithmetic that a reader can re-derive. Phase 1's
  verifier called this out as the highest-value check in the phase. Do the same for
  `cargo test --workspace`.

- **Expect the baseline run to be the phase's first real information.** Nobody in this planning
  record has ever run `cargo test --workspace` end-to-end and recorded the result. The ledger's
  verdicts come from targeted per-crate and per-test runs. D-02 exists because that run will
  probably disagree with something — budget for amendments rather than treating them as deviations.

- **`present, unproven` is the expected outcome class, not a failure.** The ledger's 23-item bucket
  is what D-19's evidence bar was designed to surface. Phase 2 shrinks it where it can and records
  the rest honestly; it does not need to reach zero.

</specifics>

<deferred>
## Deferred Ideas

- **Battalion-wide cancellation for Formation, Campaign and ChainOfCommand** (D-05). Needs a
  cancellation contract across four execution services, including what a cancelled run returns
  mid-DAG in Campaign and mid-delegation in ChainOfCommand. Recorded as `deferred with reason` plus
  an ADR (D-08) because it overrides `REQ-battalion-cancellation` as written. **No forward owner
  assigned in this discussion** — the planner should name one, and Phase 3's QUAL work is the
  natural candidate only if the contract question is scoped there too.

- **Epic 6 task 7.14 — CI configuration for the `live-api-tests` feature** (D-09). Owner: Phase 15
  (PIPE). Blocked in substance on Phase 5's VERIFY-06, which decides whether a keyless CI run fails
  loudly or skips cleanly — the thing such a job would encode.

- **The four `#[ignore]`d, empty-bodied Commander error tests** at `commander.rs:2180,2188,2196,2204`
  (D-07). Owner: Phase 3, QUAL-04. They need a `Send + Sync` mock Paladin that fails deterministically
  — a shared asset three registers have asked for and none has built, also named by Phase 15's
  DEFER register. Phase 3 should build it as shared, not local.

- **Repairing anything D-12's sweep finds beyond `tests/unit/llm/`.** The sweep reports; repair is a
  separate decision once the result size is known.

- **`REQ-provider-error-mapping`'s dead `LlmProviderError` conversion path**
  (`crates/paladin-llm/src/error.rs:16,54` — zero constructors anywhere). Surfaced by the Phase 1
  ledger, not discussed here. Related to D-12's dead-code theme.

- **WEB-04 — "does Paladin support LLM tool calling?"** Stays in Phase 14. D-14 makes the *flag*
  honest; it deliberately does not decide whether tool calling gets built or withdrawn given that
  Arsenal/MCP already provides tool execution.

- **Raising coverage to ADR-0006's 84% floor, and the 0%-coverage file list** (QUAL-01, QUAL-02).
  Phase 3. Phase 2 must not create a second coverage number (D-04).

</deferred>

---

*Phase: 2-functional-gap-closure*
*Context gathered: 2026-07-31*
