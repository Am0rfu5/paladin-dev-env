# Phase 6: Verified Gap Closure - Context

**Gathered:** 2026-08-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Close every Milestone 2-3 gap that Phase 5's verification actually proved — and nothing else.
Phase 5 handed forward an unusually well-bounded scope, so this phase is about **how** to close
four named items, not **what** to close.

**The entire scope, fixed:**

1. **CLOSE-01** — the one verified defect: `grove_service.rs:537`'s hardcoded
   `model: "gpt-4".to_string(), // TODO: Make configurable`.
2. **CLOSE-02** — exactly three clusters, consolidated by plan 05-13 into
   `.planning/ledgers/milestone-02-03.md` → `## Summary` → `### Phase 6 CLOSE-02 scope`:
   - Epic 14 cluster `8.0` — YAML & CLI Configuration Support (autonomous flags)
   - Epic 24 cluster `1.0` — the missing ChainOfCommand benchmark
   - Epic 24 cluster `8.0` — the three absent CI jobs
   Plus **WARN-01** (Herald reachability), adopted here — see D-14.
3. **CLOSE-03** — the code consequences of the Phase 5 ADRs: ADR-0011 (vision) and
   ADR-0012 (live-API harness).

**Not in this phase — settled by Phase 5, do not re-open:**

- **Epic 22 is `satisfied by shipped code`** across all fifteen parent-task clusters, including the
  three the source task list still marks open (Council/Grove registry integration, Grove LLM
  routing — shipped via commits `761c49c`, `0cdf8dd`, `5f05db7`). Per D-06 and REQUIREMENTS.md's
  own CLOSE-02 text this is recorded as an explicit **"no work required"** verdict, not quietly
  dropped. Nothing in Epic 22 is work for this phase.
- **ADR-0011** — both vision surfaces ship deliberately. No migration, no deprecation, neither
  trait removed.
- **ADR-0012** — the `require_api_key` panic **stands**. The live-API half of CLOSE-03 is
  documentation only; no behavioural change to the harness.
- **CLOSE-01 is genuinely Grove-only.** Verified during discussion: the sibling hardcodes at
  `council_service.rs:816` and `conclave_execution_service.rs:600` are both inside `#[cfg(test)]`
  (those files' `#[cfg(test)]` blocks start at `:521` and `:512`). `grove_service.rs:537` is the
  only production occurrence in the crate.
- **Success criterion 2's TODO sweep is the Grove line only.** The other four TODOs in
  `crates/paladin-battalion/src/` are all test-code (`commander.rs:3072`, `:3105`, `:3145`,
  `:3186`; `council_service.rs:733`) and none is something Epic 22's completion criteria claimed
  resolved.

</domain>

<decisions>
## Implementation Decisions

### CLOSE-01 — Grove routing model

- **D-01:** The routing model comes from a new **`routing_model: Option<String>` field on
  `GroveConfig`** (`crates/paladin-core/src/platform/container/battalion/grove.rs:208`), threaded
  through `GroveBuilder`. It sits alongside the five knobs already there (`routing_strategy`,
  `fallback_tree`, `similarity_threshold`, `routing_fallback`, `min_confidence`), so it is
  config-shaped like every other Grove setting and additive to the YAML surface (`GroveConfig` is
  `Serialize + Deserialize`). Use `#[serde(skip_serializing_if = "Option::is_none")]`, matching
  `fallback_tree`. Rejected: a `String` with a `"gpt-4"` serde default (moves the OpenAI literal
  into the config default rather than eliminating it, leaving criterion 1 half-met), and a
  constructor argument on `GroveExecutionService::new` (breaking signature, and puts a routing knob
  where no other Grove knob lives, invisible to YAML).
  — **Reversibility:** costly — `GroveConfig` is a public, serialized type; removing the field
  later breaks every YAML config that sets it and every builder call that passes it.

- **D-02:** When `routing_model` is `None` and `routing_strategy` is LLM routing, **return a hard
  error** — `BattalionError::RoutingError` naming the missing config — with **no fallback**. Do
  not consult `routing_fallback`, do not call `get_available_models()`, do not guess per provider.
  Deterministic, no network round-trip, no provider knowledge leaking into `paladin-battalion`.
  Rejected: resolving from `llm_port.get_available_models()` (async, fallible, arbitrary ordering)
  and a per-provider default-model table inside `paladin-battalion` (goes stale as providers
  rename models, and puts provider knowledge in the orchestration crate).
  — **Reversibility:** one-way — this is a deliberate runtime behaviour break. A Grove using
  `RoutingStrategy::LlmRouting` today works (silently, against `gpt-4`) and will start failing at
  runtime after this change until its config is updated. Softening it later is easy; the break has
  already been shipped by then.

- **D-03:** The break is recorded three ways: **a new ADR-0013**, plus a CHANGELOG entry under the
  current version, plus rustdoc on `GroveConfig.routing_model` stating it is required for LLM
  routing and what happens when absent. `PROMOTION.md` already records 0013 as the next free
  number. ADR-0013 records the chosen option, the two rejected alternatives from D-01, the
  hard-error choice from D-02, and the deliberate break. This is the phase's only code-behaviour
  decision, and its siblings ADR-0010/0011/0012 all got one.

- **D-04:** The proving test uses a **recording mock `LlmPort` that captures the `LlmRequest.model`
  it receives**, asserting a configured non-OpenAI model reaches `generate()`. `grove_service.rs`
  already carries several mock `LlmPort` implementations from `:1064` onward — extend that pattern
  rather than building a parallel harness. Also assert the D-02 hard-error path. Exact shape is
  the planner's call.

### CLOSE-02 — Epic 14 cluster 8.0 (autonomous CLI flags)

- **D-05:** Take the **full wiring**: add an `autonomous` section to `PaladinYamlConfig`
  (`src/application/cli/config/paladin_config.rs:40`) **and** apply the four CLI flags as overrides
  on top in `handle_agent_run`. This is exactly what the cluster is named ("YAML & CLI
  Configuration Support") and what the flags' own comment at `src/application/cli/commands/agent.rs:77`
  promises ("Autonomous feature flags (override config file)") against a config section that does
  not exist. Rejected: flags-only wiring (leaves the "override config file" comment false and only
  partly clears the block verdict) and deleting the four flags (removes a shipped CLI surface and
  walks back Epic 14's stated intent).

- **D-06:** The YAML section **reuses `paladin-core`'s `AutonomousConfig` directly** —
  `autonomous: Option<AutonomousConfig>` on `PaladinYamlConfig`. It already derives
  `Serialize, Deserialize, PartialEq, Default`
  (`crates/paladin-core/src/platform/container/autonomous_config.rs:69`) and
  `PaladinConfig::autonomous` (`paladin_config.rs:61`) takes exactly that type, so there is no
  mapping layer to write or keep in sync across its four sub-configs (`planning`,
  `prompt_generation`, `dynamic_temperature`, `handoffs`). Rejected: a CLI-local
  `AutonomousYamlConfig` mirror matching the `GarrisonConfig`/`ArsenalConfig` convention in the
  same file — the mapping cost outweighed the decoupling here.
  — **Reversibility:** costly — the YAML schema becomes coupled to a domain type; a later rename
  inside `AutonomousConfig` becomes a config-format break for every user's `paladin.yaml`.

- **D-07:** Flag override is **additive only**. A present flag forces that feature on; an absent
  flag leaves the YAML value untouched. No `--no-*` counterparts, no `Option<bool>`, no CLI
  signature change — the four flags stay plain clap bools. Consequence, stated plainly: a
  YAML-enabled autonomous feature **cannot** be turned off from the command line. Rejected:
  negating twins via `ArgAction::SetFalse`, which would double the flag surface from four to eight
  and add conflict handling.

- **D-08:** All four flags already have domain seams — no new domain work. `PaladinData` carries
  `autonomous_planning` (`paladin.rs:176`), `autonomous_prompts` (`:184`) and `dynamic_temperature`
  (`:199`); `PaladinBuilder` exposes `enable_autonomous_planning` (`:546`),
  `enable_autonomous_prompts` (`:576`) and `with_handoff_config` (`:896`). Only the CLI-side
  plumbing is missing. Domain-level `PaladinConfig.autonomous` is itself already satisfied and
  tested (`paladin_config.rs:226`, `:246`) — do not re-do it.

### CLOSE-02 — Epic 24 cluster 8.0 (the three CI jobs)

- **D-09:** **All three jobs — `cli-tests`, `bench-check`, `coverage` — are deferred to Phase 15**
  with a written reason, which success criterion 3 explicitly permits. They are literally PIPE-01
  (`cli-tests` + `bench-check`) and PIPE-02 (the coverage job with `.codecov.yml`). Two reasons:
  Phase 15's own register says its first half "establishes quality gates that validate all
  subsequent work" and must come first; and PIPE-02 cannot be built without first settling a
  coverage threshold that still has **six competing positions** — the Deferred-QA parent PRD's 78%
  hard gate, Epic 25's 70→74→78 ramp, and ADR-0006's 84% floor among them. Phase 6 does not pick
  that number. Rejected: building all three (pre-empts PIPE-02's threshold decision and duplicates
  Phase 15's first half) and the split option (build `cli-tests` + `bench-check` now, defer
  `coverage`).

- **D-10:** The deferral is recorded **bidirectionally**: in
  `.planning/ledgers/milestone-02-03.md`'s Epic 24 block verdict and CLOSE-02 rows, **and** as a
  note on PIPE-01/PIPE-02 in `.planning/REQUIREMENTS.md` naming Epic 24 cluster `8.0` as their
  inbound scope. Both ends point at each other, so neither a Phase 6 reader nor a Phase 15 planner
  rediscovers the link — which is the exact failure mode this planning corpus exists to fix.
  Rejected: ledger-only, and ledger + STATE.md.

- **D-11:** **Hard cross-cutting constraint — no file under `.github/` is modified in this phase.**
  Stated the way Phase 5 barred `.rs`/`Cargo.toml`/`.github` edits. This keeps D-09's deferral
  honest: if a plan believes it needs a workflow edit, that is a deviation requiring approval, not
  a quiet inclusion. *(Amended 2026-08-05: this decision originally carried a second justification —
  preventing an uncommitted `ci.yml` revert from riding along in a Phase 6 commit. That change was
  discarded the same day by user decision; see Deferred Ideas. The constraint stands on D-09 alone.)*

### CLOSE-02 — Epic 24 cluster 1.0 (ChainOfCommand benchmark)

- **D-12:** **Write the benchmark.** Add a `benchmark_chain_of_command` function to
  `crates/paladin-battalion/benches/battalion_benchmarks.rs` using `ChainOfCommandExecutionService`
  and register it in the `criterion_group!` at `:156`. The file currently registers only
  `benchmark_formation_three_agents`, `benchmark_phalanx_five_agents` and
  `benchmark_campaign_branching_dag` — Campaign **is** present, so only the ChainOfCommand half of
  the doc's claim is false. Makes `docs/src/appendix/battalion-benchmarks.md`'s existing claim true
  rather than retracting it. Phase 2's GAP-01 already built out Chain of Command with tests across
  all four delegation strategies, so the harness exists. Rejected: correcting the doc at source,
  and doing both.

- **D-13:** **Run it and record the baseline.** Append the new target's throughput and derived
  P50/P95/P99 to `docs/src/appendix/performance-baseline.md`, following the nearest-rank derivation
  formula and `jq` filter Phase 3 plan 03-04 documented (`### P50 / P95 / P99 Derivation`, `:624`).
  Record it under a **clearly dated separate run** — do not silently merge a 2026-08 measurement
  into the 2026-08-02 table, whose value is that it was taken in one sitting. Otherwise the
  baseline document silently covers 5 of 6 battalion patterns and the next comparison has nothing
  to compare against.

### CLOSE-02 — WARN-01 (Herald reachability), adopted

- **D-14:** **Adopt WARN-01, all three services.** Replicate the established
  `herald: Option<Arc<dyn Herald>>` field + `with_herald()` setter + format-wrapper pattern into
  `campaign_service.rs`, `chain_of_command_service.rs` and `commander.rs`. Measured during
  discussion: `formation_service.rs` and `phalanx_service.rs` carry **19 Herald references each**;
  the other three carry **zero**. The roadmap explicitly sanctions adopting or declining this, so
  it is not scope creep either way. Adopting closes the inherited item inside the milestone that
  inherited it and makes the composite Chain-of-Command developer flow compose without the caller
  reaching for a Herald directly. Rejected: Commander-only (leaves two services asymmetric) and
  declining on the record.

- **D-15:** WARN-01 is absorbed under **CLOSE-02**, and proved with **one end-to-end composite
  test** rendering a Chain of Command result through a Herald — mirroring the Formation-driven
  three-Herald test Phase 2 plan 02-05 built for GAP-03. A compile check is not sufficient; the
  composite flow gets an executable witness. Rejected: per-service unit tests only, and filing it
  under CLOSE-03 (whose text is about applying the Phase 5 ADRs, which WARN-01 predates).

### CLOSE-03 — vision (ADR-0011)

- **D-16:** The encryption capability is recorded as a **deliberately unimposed, consumer-facing
  utility**. Rationale established during discussion and stronger than ADR-0011 had: **the shipped
  vision path never stores image bytes.** `PaladinExecutionService::execute_with_vision`
  (`src/application/services/paladin/paladin_execution_service.rs:517`) takes `Vec<VisionContent>`
  from the caller — URL, base64, or file path — and hands it straight to the vision adapter. There
  is no framework-owned temp file or cache, so Epic 13 FR-11's premise ("encryption at rest for
  *temporarily stored* image data") has no storage in the shipped design to protect. Deliverables:
  rustdoc on `EncryptionService` stating the framework does not invoke it and when a consumer
  holding image bytes should, plus the `REQ-vision-security-encryption` ledger row amended to that
  verdict. **No behaviour change.** Rejected: wiring `SecureData` zeroization into the in-flight
  path (a real behaviour change that only partly satisfies FR-11) and wiring `encrypt_image_data`
  fully (with no persistence between encrypt and decrypt it is ceremony, not protection, and costs
  every vision request).

  **This closes the open-ended risk the roadmap flagged** — "if VERIFY-04 finds Epic 13's
  encryption-at-rest requirement was *not* consciously dropped, that is new security work with no
  phase home anywhere in Phases 5-16". ADR-0011 already established it was built rather than
  dropped; D-16 establishes there is nothing on the vision path for it to protect. No new phase,
  no new requirement.

- **D-17:** **ADR-0011 is amended in place** with a dated resolution note recording D-16's choice
  and its reason, and its `## Code Conformance` flipped from `must change` to reflect the doc-only
  outcome. Matches the amend-at-source pattern Phases 1-4 established. Rejected: leaving ADR-0011
  immutable and recording only in the ledger/rustdoc; and minting a separate ADR-0014 (ADR-0013 is
  already taken by D-03 this phase, and ADR-0011 framed this as one open consequence).

- **D-18:** The vision **entry-point rustdoc** half of CLOSE-03 is fully specified by ADR-0011's
  `## Decision` — no gray area. `VisionPort` (`crates/paladin-ports/src/output/vision_port.rs:47`)
  is documented as the recommended application-code entry point, reached via
  `execute_with_vision`; `VisionCapableLlm` (`vision_llm_port.rs:52`) as the adapter-author
  surface, reached via `PaladinBuilder::enable_vision`
  (`src/application/services/paladin/paladin_builder.rs:517`). **Neither trait is removed or
  deprecated.**

### CLOSE-03 — live-API harness (ADR-0012)

- **D-19:** Fully specified by ADR-0012 — no gray area, **documentation only**. Correct the doc
  comment's opening line at `tests/integration/llm_live_api_tests.rs:61` (it says *"Skip test if
  API key is not present or empty"* while both matched arms panic), and document the double gate
  as the actual skip mechanism in the module header: `#[cfg(feature = "live-api-tests")]` at
  `tests/integration/mod.rs:34-35` plus the 13 `#[ignore]` attributes. **The panic stands.** Do not
  change `require_api_key`'s behaviour.

### Claude's Discretion

- The exact shape of D-04's recording mock and D-15's composite test — extend the existing
  in-file mock patterns rather than building parallel harnesses.
- Whether `generate_paladin_template` (`src/application/cli/commands/agent.rs`) emits the new
  `autonomous:` YAML section in the template it writes. Emitting a commented-out example is likely
  right, but not decided.
- Plan/wave decomposition, commit granularity, and where the ADR-0013 authoring lands relative to
  the Grove code change.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase 5 output — the scope source of record

- `.planning/ledgers/milestone-02-03.md` — the 118-row Milestone 2-3 ledger. Read its
  `## Summary` → `### Phase 6 CLOSE-02 scope` section **first**: it names this phase's entire
  CLOSE-02 scope as exactly three clusters, already cross-checked against the three block verdicts.
  Also carries the five-bucket verdict distribution (64 satisfied / 25 present-unproven / 3
  genuinely outstanding / 5 deferred / 21 superseded) and the three block-verdict tables.
- `.planning/phases/05-milestone-2-3-ground-truth/05-13-SUMMARY.md` — the close-out summary that
  consolidated the CLOSE-02 scope, with the three block verdicts quoted verbatim and the Epic 22
  "no work required" disposition.
- `.planning/phases/05-milestone-2-3-ground-truth/05-VERIFICATION.md` — Phase 5's verification
  report (passed; one gap closed inline).

### Decisions this phase must apply or extend

- `.planning/decisions/0011-vision-port-surfaces.md` — both vision surfaces ship deliberately;
  entry-point guidance for the rustdoc; and the encryption disposition ("built, self-tested, and
  never wired") this phase resolves under D-16/D-17. Amended in place by this phase.
- `.planning/decisions/0012-live-api-test-key-behaviour.md` — the `require_api_key` panic stands;
  specifies exactly the doc-comment correction and double-gate documentation CLOSE-03 delivers.
- `.planning/decisions/0010-milestone-3-epic-numbering.md` — the authoritative Epic 19-24 numbering
  (19 Herald, 20 Vision, 21 Autonomous, 22 Battalion hardening, 23 CLI/Config, 24 Test hardening).
  Use these numbers, not `RELEASE_NOTES_MILESTONE_3.md`'s.
- `.planning/decisions/0006-coverage-gate.md` — the 84% workspace floor and its recorded scope.
  Relevant only as context for **why** D-09 defers the coverage job; this phase does not measure
  coverage or pick a threshold.
- `.planning/decisions/PROMOTION.md` — the ADR numbering index. Next free number is **0013**,
  claimed by D-03.

### Project-level

- `.planning/ROADMAP.md` → `### Phase 6: Verified Gap Closure` — the four success criteria, the
  hard Phase 5 dependency, and the inherited WARN-01 note.
- `.planning/REQUIREMENTS.md` — CLOSE-01/02/03 requirement text and the traceability table this
  phase closes out; also PIPE-01/PIPE-02, which D-10 annotates.
- `.planning/PROJECT.md` → `## Context` — the precedence order (**ADR → shipped tree →
  `.planning/codebase/` → `intel/code-verification.md` → PRD → DOC → task-list checkbox**) and the
  corpus-level finding that nothing is locked. Checkbox state is the least reliable signal in this
  project and is wrong in both directions.
- `.planning/STATE.md` → Deferred Items — WARN-01's inherited wording, and the uncommitted `ci.yml`
  flag.

### Code and docs this phase touches

- `crates/paladin-core/src/platform/container/battalion/grove.rs:208` — `GroveConfig`, D-01's
  target; `:294` `GroveBuilder`.
- `crates/paladin-battalion/src/grove_service.rs:537` — the defect; `:487` the `llm_port` guard;
  `:1064+` existing mock `LlmPort` implementations to extend for D-04.
- `src/application/cli/config/paladin_config.rs:40` — `PaladinYamlConfig`, D-06's target;
  `GarrisonConfig` at `:100` and `ArsenalConfig` at `:124` show the file's existing section shape.
- `src/application/cli/commands/agent.rs:77-93` — the four declared-but-unread autonomous flags.
- `crates/paladin-core/src/platform/container/autonomous_config.rs:69` — `AutonomousConfig` and its
  four sub-configs, reused directly per D-06.
- `crates/paladin-battalion/benches/battalion_benchmarks.rs:156` — the `criterion_group!` D-12
  extends; `crates/paladin-battalion/Cargo.toml:34` declares the `[[bench]]` target.
- `docs/src/appendix/battalion-benchmarks.md:193, :223, :237` — the three ChainOfCommand
  "Compiling and enabled" claims D-12 makes true.
- `docs/src/appendix/performance-baseline.md:624` — the documented P50/P95/P99 derivation formula
  and `jq` filter D-13 follows.
- `crates/paladin-battalion/src/formation_service.rs:40, :79` — the Herald field and `with_herald`
  pattern D-14 replicates into `campaign_service.rs`, `chain_of_command_service.rs` and
  `commander.rs`.
- `src/infrastructure/security/encryption.rs:200, :217, :68, :131` — `encrypt_image_data`,
  `decrypt_image_data`, `SecureData`'s `Zeroize`/`ZeroizeOnDrop`, `DataRetentionPolicy::should_retain`
  — the utility D-16 documents as unimposed.
- `tests/integration/llm_live_api_tests.rs:61-64` — the doc comment D-19 corrects;
  `tests/integration/mod.rs:34-35` — the feature gate it documents.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **The `with_herald` pattern** — `formation_service.rs` and `phalanx_service.rs` each carry 19
  Herald references implementing an identical shape: an `Option<Arc<dyn Herald>>` field, a
  `with_herald()` setter, and a format wrapper that falls back when no Herald is set. D-14 is a
  three-way replication of an established, tested pattern inside one crate — not new design.
- **`AutonomousConfig` needs no new derives** — it already carries
  `Debug, Clone, Serialize, Deserialize, PartialEq, Default`, so D-06's `Option<AutonomousConfig>`
  field on `PaladinYamlConfig` deserializes with no adapter code.
- **All four autonomous flags already have domain seams** — `PaladinData.autonomous_planning`,
  `.autonomous_prompts`, `.dynamic_temperature`, and `PaladinBuilder::{enable_autonomous_planning,
  enable_autonomous_prompts, with_handoff_config}`. The gap is purely CLI plumbing.
- **Mock `LlmPort` implementations already live in `grove_service.rs`** from `:1064` onward, several
  of them, returning `model: "mock-model"`. D-04 extends this rather than adding a harness.
- **Phase 3's derivation method is documented and reproducible** —
  `performance-baseline.md:624` records the nearest-rank formula, the tie-break rule, and the exact
  `jq` invocation against criterion's `SavedSample` schema. D-13 reuses it verbatim.
- **`ChainOfCommandExecutionService` exists and is tested** across all four delegation strategies
  (Phase 2 GAP-01), so D-12's benchmark has a working service to drive.

### Established Patterns

- **`GroveConfig` uses `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields**
  (`fallback_tree`) — D-01's `routing_model` follows it.
- **`PaladinYamlConfig` uses `#[serde(default)]` / `#[serde(skip_serializing_if)]` per field** and
  defines its section types locally (`GarrisonConfig`, `ArsenalConfig`). D-06 deliberately deviates
  from the local-type half of that convention; the deviation is recorded, not accidental.
- **Amend at source, with dated provenance.** Phases 1-4 established this — original text retained,
  amendment dated and citing the measurement or plan that motivated it. D-17 (ADR-0011) and D-10
  (PIPE requirement notes) both follow it.
- **`file:line` citations with a named exerciser.** Phase 1's D-19 bar: a row is `satisfied` only
  with a citation **plus** a named passing test, example, or command. Anything with a citation and
  no exerciser is `present, unproven`. Ledger rows this phase writes must clear the same bar.

### Integration Points

- `GroveConfig` → `GroveBuilder` → `grove_service.rs`'s LLM routing request construction (D-01/D-02).
- `paladin.yaml` → `PaladinYamlConfig.autonomous` → `PaladinConfig::autonomous` → `PaladinBuilder`,
  with the four CLI flags layered on top additively in `handle_agent_run` (D-05/D-07).
- `campaign_service` / `chain_of_command_service` / `commander` → `Herald::format_battalion_result`
  (`crates/paladin-core/src/platform/container/herald.rs:85`), which is pattern-agnostic, so the
  three concrete Heralds in `paladin-herald` need no change (D-14).
- `battalion_benchmarks.rs` → `criterion_group!` → `cargo bench` → the appended baseline table
  (D-12/D-13).

</code_context>

<specifics>
## Specific Ideas

- **"Grove routing with no explicit model is a configuration error, not a thing to guess at."**
  D-02 is the sharpest decision in the phase: no fallback chain, no provider lookup, no default
  table. The user chose the loud option over three quieter ones, accepting a runtime break for
  existing LLM-routing Groves. Plans must not soften this into a warning-plus-default.

- **The CI deferral is a boundary, not a shortcut.** D-09 + D-11 together mean Phase 6 is a
  source-and-docs phase that touches no workflow file at all. The user made `.github/` a hard
  constraint specifically so the deferral cannot erode plan by plan.

- **Record links must point both ways.** D-10 was chosen over two cheaper options explicitly
  because one-directional records going stale is the failure mode this entire planning corpus
  exists to correct.

- **Make the doc true rather than retracting it** (D-12), but **do not merge measurement runs**
  (D-13). The user wanted the benchmark written and measured, and the measurement kept honest about
  when it was taken.

</specifics>

<deferred>
## Deferred Ideas

- **The `cli-tests`, `bench-check` and `coverage` CI jobs** → **Phase 15, PIPE-01 and PIPE-02.**
  Deferred by D-09 with a written reason, recorded bidirectionally per D-10. Note for Phase 15:
  coverage tooling is **partially built** — `integration-tests.yml:117-123` already runs
  `cargo llvm-cov` and `codecov-action@v3`; PIPE-02 supersedes that integration-only path rather
  than starting from nothing. And PIPE-02 must reconcile six competing threshold positions
  (78% hard gate, 70→74→78 ramp, 80, 85, 75-layered, and ADR-0006's 84% floor) before it can gate.

- **~~An uncommitted working-tree change reverts a shipped v0.7.1 deliverable.~~ — CLOSED
  2026-08-05, discarded by user decision during this discussion.**
  `.github/workflows/ci.yml` carried an uncommitted diff (**+6 / −50**) that stripped Phase 4's
  advisory multi-arch wall-clock rationale and restored a hard `::error::` at 300s — a budget the
  record says has never once been met in this repository's history (measured 2946 s on 2026-08-03).
  Investigation during this discussion found it reverted **two** Phase 4 changes, not one: it also
  deleted the `Load amd64 image for size measurement` step that commit `163f0ee` added, without
  which `docker image inspect paladin:test` has no local image on a multi-arch build — so it would
  have broken the **size** gate, the one budget Phase 4 kept hard. Origin never established; it
  predated this session and was already flagged by Phase 5 on 2026-08-04.

  **It was also blocking every commit.** Verified mechanism: `cargo clippy --workspace
  --all-targets --all-features -- -D warnings` exits **0** and modifies no tracked file, but
  pre-commit stashes the unstaged change, runs the hooks, restores it, sees the tree differ and
  reports `cargo clippy … Failed — files were modified by this hook`. Proven by staging the file
  (no unstaged changes → no stash) and watching every hook pass. The two commits carrying this
  CONTEXT.md and its DISCUSSION-LOG (`63ee282`, `899f310`) used `--no-verify` for that reason and
  that reason only.

  **Disposition:** `git checkout -- .github/workflows/ci.yml`, restoring HEAD's Phase 4 version.
  Working tree clean, both Phase 4 pieces confirmed present. Backup retained outside the repo.
  D-11's `.github/` prohibition still stands on its own merits (the three CI jobs are deferred to
  Phase 15 per D-09) — it simply no longer has this second justification.

- **Feature-gating `chacha20poly1305` and `zeroize`** (`Cargo.toml:134-135`, both unconditional, no
  feature gate) so library consumers stop compiling two crypto crates for a capability D-16 records
  as unimposed → **already owned by ARCH-05 in Phase 7**, which covers "`vision` gating the
  encryption crates (would break `cargo build --no-default-features`)". Not touched here.

- **The four test-code TODOs in `crates/paladin-battalion/src/`** — `commander.rs:3072`, `:3105`,
  `:3145`, `:3186` and `council_service.rs:733`, all requesting `MockPaladinPort` error injection
  or a registry parameter. None is production code and none is something Epic 22's completion
  criteria claimed resolved, so success criterion 2 is unaffected. Candidate for the shared
  `Send + Sync` mock infrastructure work under **DEFER-01, Phase 15**.

- **Nyquist validation for Phases 1-4** — all four `VALIDATION.md` files read `status: draft`.
  Owner: `/gsd-validate-phase 1`…`4`. Carried forward from the v0.7.1 close-out; unrelated to this
  phase.

- **Whether ADRs should be published to the mdbook for framework consumers** — unanswered since
  Phase 1, carried through Phase 5. Belongs with **Phase 16**'s documentation work. Phase 6 mints
  ADR-0013, which makes the question one entry larger.

- **The Grove routing threshold competing-variant group** (3 names / 3 defaults) — a separate
  variant from D-01's model question and not settled here. `GroveConfig` already ships
  `similarity_threshold: 0.7` and `min_confidence: 0.5`; reconciling those against the documented
  variants is recorded work elsewhere in the corpus, not Phase 6 scope.

</deferred>

---

*Phase: 6-Verified Gap Closure*
*Context gathered: 2026-08-05*
