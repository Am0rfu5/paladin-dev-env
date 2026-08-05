# Phase 6: Verified Gap Closure - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-05
**Phase:** 6-verified-gap-closure
**Areas discussed:** Grove routing model source, Autonomous CLI flags, The three missing CI jobs, ChainOfCommand benchmark, Vision encryption wiring, WARN-01 Herald reachability

**Area selection:** all six gray areas presented were selected for discussion.

---

## Grove routing model source (CLOSE-01)

### Q1 — Where should Grove's LLM routing model come from?

| Option | Description | Selected |
|--------|-------------|----------|
| GroveConfig field, `Option<String>` | `routing_model: Option<String>` alongside the five existing GroveConfig knobs, threaded through GroveBuilder; `Some` used verbatim, `None` resolved at call time. Additive to YAML, no OpenAI literal survives in the routing path. | ✓ |
| GroveConfig field, `String` w/ serde default | `#[serde(default)]` returning `"gpt-4"`. Simplest, every existing config keeps working — but moves the OpenAI literal into the config default, so an unconfigured Anthropic Grove still routes with `gpt-4`. | |
| Constructor arg on the service | Pass the model to `GroveExecutionService::new` next to `llm_port`. Pairs model with port, but breaking signature change and invisible to YAML. | |

**User's choice:** GroveConfig field, `Option<String>`
**Notes:** Established during discussion that `GroveConfig` is `Serialize + Deserialize`, so anything added there becomes a YAML surface; and that `LlmPort` exposes `get_available_models()` and `get_provider_name()` but **no** default-model accessor.

### Q2 — When `routing_model` is None, what should `grove_service.rs` do?

| Option | Description | Selected |
|--------|-------------|----------|
| Ask the port, fall back on failure | `get_available_models()` first entry; on error/empty honour the existing `routing_fallback` path. Cost: provider round-trip per routing call. | |
| Hard error, require explicit config | Return `BattalionError::RoutingError` immediately, no fallback. Deterministic, no network call, no guessing — but existing LlmRouting Groves start failing at runtime. | ✓ |
| Per-provider default map | Switch on `get_provider_name()` to a default-model table in `paladin-battalion`. Deterministic and unconfigured-friendly, but puts provider knowledge in the orchestration crate and goes stale on model renames. | |

**User's choice:** Hard error, require explicit config
**Notes:** Chosen with the runtime-break consequence stated in the option text and re-confirmed immediately after. The loud option was preferred over three quieter ones.

### Q3 — How should that behavioural break be recorded?

| Option | Description | Selected |
|--------|-------------|----------|
| CHANGELOG + rustdoc on the field | Record where a consumer looks; matches ADR-0011's doc-only vision half. | |
| A new ADR (0013) as well | ADR recording the decision, rejected alternatives and the deliberate break, on top of CHANGELOG and rustdoc. PROMOTION.md already has 0013 free. | ✓ |
| Rustdoc only | Document on the field and in the error message; skip the CHANGELOG. | |

**User's choice:** A new ADR (0013) as well
**Notes:** ADR-0010/0011/0012 all got one, and this is the phase's only code-behaviour decision.

**Area close:** Next area. Test shape (recording mock `LlmPort` asserting the configured model reaches `generate()`) left to the planner.

---

## Autonomous CLI flags (CLOSE-02, Epic 14 cluster 8.0)

### Q1 — How far should Epic 14 cluster 8.0 go?

| Option | Description | Selected |
|--------|-------------|----------|
| Full: YAML section + flag override | `autonomous:` section on `PaladinYamlConfig` mirroring `AutonomousConfig`'s four sub-configs, with the four CLI flags applied as overrides on top. Matches the cluster's own title and the flags' own comment. | ✓ |
| Flags only, no YAML section | Wire the four flags straight to `PaladinBuilder`. Removes the dead surface, but the "override config file" comment stays false and the block verdict only partly clears. | |
| Remove the four flags | Delete them, record autonomous features as a library-API surface with a migration note. Smallest and honest, but removes a shipped CLI surface and walks back Epic 14's intent. | |

**User's choice:** Full: YAML section + flag override
**Notes:** Established during discussion that all four flags already have domain seams — `PaladinData.{autonomous_planning, autonomous_prompts, dynamic_temperature}` and `PaladinBuilder::{enable_autonomous_planning, enable_autonomous_prompts, with_handoff_config}` — so only CLI plumbing is missing.

### Q2 — Flag override semantics (flags are plain clap bools, can't express "explicitly off")

| Option | Description | Selected |
|--------|-------------|----------|
| Additive only | Present flag forces on; absent leaves YAML untouched. No signature change, no accidental disabling by omission. Cost: cannot turn a YAML-enabled feature off from the CLI. | ✓ |
| Add `--no-*` counterparts | Negating twins via `ArgAction::SetFalse` or `Option<bool>`. Truest "override", but doubles the flag surface to eight and adds conflict handling. | |

**User's choice:** Additive only

### Q3 — YAML type: reuse core's `AutonomousConfig`, or mirror it CLI-side?

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse `AutonomousConfig` directly | Already derives Serialize/Deserialize/Default and is exactly what `PaladinConfig::autonomous` takes — no mapping layer. Cost: YAML schema coupled to a domain type. | ✓ |
| CLI-local mirror type | `AutonomousYamlConfig` alongside `GarrisonConfig`/`ArsenalConfig` with explicit conversion. Matches file convention, keeps YAML independent of domain refactors. Cost: four sub-configs of mapping to maintain. | |

**User's choice:** Reuse `AutonomousConfig` directly
**Notes:** A deliberate deviation from the file's local-type convention, recorded rather than accidental.

**Area close:** Next area.

---

## The three missing CI jobs (CLOSE-02, Epic 24 cluster 8.0)

Context supplied before the question: the three jobs are literally PIPE-01 (`cli-tests` + `bench-check`) and PIPE-02 (coverage + `.codecov.yml`) in Phase 15; PIPE-02 carries an unsettled threshold with six competing positions; and `bench-check` would be the compile prerequisite guarding the cluster 1.0 benchmark.

### Q1 — Which of the three does Phase 6 build?

| Option | Description | Selected |
|--------|-------------|----------|
| Split: `cli-tests` + `bench-check` now | Build the two carrying no unsettled decision, defer coverage to PIPE-02. `bench-check` would guard the same phase's benchmark work. | |
| Defer all three to Phase 15 | Record cluster 8.0 deferred with a written reason (permitted by success criterion 3). Keeps all CI-gate work in the phase whose register says it must come first; Phase 6 touches no workflow file. Cost: Epic 24's verdict stays partially outstanding across a milestone boundary. | ✓ |
| Build all three now | Closes Epic 24 completely. Cost: pre-empts PIPE-02's threshold decision and duplicates most of Phase 15's first half. | |

**User's choice:** Defer all three to Phase 15

### Q2 — Where does the deferral get recorded?

| Option | Description | Selected |
|--------|-------------|----------|
| Bidirectional pointer | Ledger Epic 24 verdict + CLOSE-02 rows, **and** a note on PIPE-01/PIPE-02 in REQUIREMENTS.md naming cluster 8.0 as their inbound scope. Neither end has to rediscover the link. | ✓ |
| Ledger + STATE.md only | Ledger verdict plus a STATE.md Deferred Items entry, matching how WARN-01 was carried forward. Leaves forward-milestone requirement text untouched. | |
| Ledger only | Append to the existing Epic 24 block verdict and stop. Single source of truth, no cross-file sync. | |

**User's choice:** Bidirectional pointer
**Notes:** Chosen explicitly because one-directional records going stale is the failure mode the whole planning corpus exists to correct.

### Q3 — Should Phase 6 be barred from touching `.github/` at all?

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — hard constraint | Cross-cutting "no file under `.github/` is modified", the way Phase 5 barred `.rs`/`Cargo.toml`/`.github`. Keeps the deferral honest and prevents any commit carrying the uncommitted `ci.yml` revert. | ✓ |
| No — leave it open | Allow an incidental workflow tweak if the Grove or autonomous tests need one in CI. | |

**User's choice:** Yes — hard constraint

**Area close:** Next area.

---

## ChainOfCommand benchmark (CLOSE-02, Epic 24 cluster 1.0)

Context supplied before the question: `battalion_benchmarks.rs` registers only formation/phalanx/campaign — Campaign **is** present, so only the ChainOfCommand half of the doc claim (`battalion-benchmarks.md:193, :223, :237`) is false. Phase 3 recorded a dated 2026-08-02 baseline across all five bench targets.

### Q1 — Write the benchmark, or correct the doc?

| Option | Description | Selected |
|--------|-------------|----------|
| Write the benchmark | Add `benchmark_chain_of_command` via `ChainOfCommandExecutionService`, register in the `criterion_group!`. Makes the existing claim true; Phase 2's GAP-01 already built the harness. | ✓ |
| Correct the doc | Withdraw the ChainOfCommand claim in all three places, recording that the benchmark was never restored. Cheapest, consistent with "shipped tree is the arbiter". | |
| Both | Write it and amend the "fixed in Epic 24" history framing to say when it actually landed. | |

**User's choice:** Write the benchmark

### Q2 — Does the new benchmark need a recorded baseline measurement?

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — run it and record | Append throughput and derived P50/P95/P99 to `performance-baseline.md` using Phase 3's documented nearest-rank derivation and `jq` filter. Otherwise the baseline silently covers 5 of 6 patterns. | ✓ |
| No — compile-and-run proof only | Prove it compiles and executes; leave the baseline to the next full bench run, since Phase 3's table's value is that it was taken in one sitting. | |

**User's choice:** Yes — run it and record
**Notes:** Resolved in CONTEXT.md D-13 by recording the new measurement under a clearly dated **separate** run rather than merging it into the 2026-08-02 table — taking the "yes" without losing the measurement-conditions objection raised in the rejected option.

**Area close:** Next area.

---

## Vision encryption wiring (CLOSE-03, ADR-0011)

Finding surfaced before the question: `execute_with_vision` takes `Vec<VisionContent>` (URL / base64 / file path) from the caller and hands it straight to the vision adapter — there is **no framework-owned temp file or cache**, so Epic 13 FR-11's "encryption at rest for temporarily stored image data" has no storage in the shipped design to protect. Also noted: `chacha20poly1305` and `zeroize` are unconditional deps, but feature-gating them is already ARCH-05's in Phase 7.

### Q1 — How should CLOSE-03 resolve the wiring question ADR-0011 handed forward?

| Option | Description | Selected |
|--------|-------------|----------|
| Record as unimposed utility | Record `EncryptionService`/`DataRetentionPolicy`/`SecureData` as consumer-facing, framework-not-invoked, with rustdoc and a ledger-row amendment. No behaviour change; closes the open-ended security risk the roadmap flagged. | ✓ |
| Wire `SecureData` zeroization only | Wrap in-flight image bytes so they zeroize on drop — memory hygiene without an at-rest step. Cost: real behaviour change, only partly satisfies FR-11. | |
| Wire `encrypt_image_data` fully | Encrypt before the adapter, decrypt after. Satisfies FR-11 literally, but with no persistence between the two it is ceremony, not protection, and costs every vision request. | |

**User's choice:** Record as unimposed utility

### Q2 — How does the answer get back into the record?

| Option | Description | Selected |
|--------|-------------|----------|
| Amend ADR-0011 in place | Dated resolution note plus flipping `## Code Conformance` to the doc-only outcome. Matches the Phases 1-4 amend-at-source pattern; the ADR shows the whole arc. | ✓ |
| Ledger row + rustdoc only | Leave ADR-0011 as an immutable dated snapshot; record the resolution elsewhere. | |
| New ADR-0014 | Separate ADR cross-linked from 0011. Cost: Phase 6 would mint two ADRs for what 0011 framed as one open consequence. | |

**User's choice:** Amend ADR-0011 in place

**Area close:** Next area. Confirmed that the other two CLOSE-03 halves — vision entry-point rustdoc (ADR-0011) and the live-API doc comment (ADR-0012) — are already fully specified and carry no gray area.

---

## WARN-01 — Herald reachability

Measured before the question: `formation_service.rs` and `phalanx_service.rs` carry **19 Herald references each**; `campaign_service.rs`, `chain_of_command_service.rs` and `commander.rs` carry **zero**. The roadmap explicitly sanctions adopting or declining, so neither answer is scope creep.

### Q1 — Adopt or decline?

| Option | Description | Selected |
|--------|-------------|----------|
| Adopt all three | Replicate the `with_herald` + format-wrapper pattern into campaign, chain_of_command and commander. Closes the inherited item inside the milestone that inherited it; makes the composite Chain-of-Command flow compose without the caller reaching for a Herald. | ✓ |
| Adopt Commander only | Wire the router alone — anything routed through Commander gets formatting regardless of strategy. Cost: two services stay asymmetric. | |
| Decline on the record | `format_battalion_result` is pattern-agnostic and no requirement's text is falsified; keep the phase to gaps verification proved and give WARN-01 a closed verdict. | |

**User's choice:** Adopt all three

### Q2 — Which requirement absorbs it, and what proof?

| Option | Description | Selected |
|--------|-------------|----------|
| CLOSE-02, with a composite test | One end-to-end test rendering a Chain of Command result through a Herald, mirroring Phase 2 plan 02-05's Formation-driven three-Herald test. Executable witness for the composite flow. | ✓ |
| CLOSE-02, compile + unit only | Per-service unit tests that `with_herald` is honoured, matching the existing pattern's own test bar. | |
| CLOSE-03, with a composite test | File it as applying a recorded decision. Cost: CLOSE-03's text is about the Phase 5 ADRs, which WARN-01 predates. | |

**User's choice:** CLOSE-02, with a composite test

**Area close:** Ready for context — no further gray areas raised.

---

## Claude's Discretion

- Exact shape of the recording mock `LlmPort` (D-04) and the composite Herald test (D-15) — extend
  the existing in-file mock patterns rather than building parallel harnesses.
- Whether `generate_paladin_template` emits the new `autonomous:` YAML section (likely a
  commented-out example, but not decided).
- Plan/wave decomposition, commit granularity, and where ADR-0013 authoring lands relative to the
  Grove code change.

## Deferred Ideas

- `cli-tests` / `bench-check` / `coverage` CI jobs → Phase 15, PIPE-01 and PIPE-02.
- ~~The uncommitted `.github/workflows/ci.yml` revert (+6/−50) of a shipped v0.7.1 deliverable.~~
  **Resolved 2026-08-05, after the areas above, in a follow-up exchange.** Investigation found it
  reverted two Phase 4 changes — the advisory wall-clock rationale *and* the `Load amd64 image for
  size measurement` step from commit `163f0ee`, without which the size gate breaks — and that it
  was failing every commit via a pre-commit stash/restore artifact (clippy itself exits 0 and
  modifies nothing; proven by staging the file and watching all hooks pass). Origin never
  established; it predated this session. User chose **discard**; `git checkout --` restored HEAD's
  Phase 4 version, working tree clean.

  | Option | Description | Selected |
  |--------|-------------|----------|
  | Discard — restore HEAD's version | Unblocks commits, keeps the v0.7.1 record true, restores the amd64 load step the size gate depends on. | ✓ |
  | Show the full diff first | Print the complete +6/−50 diff before deciding; nothing changed. | |
  | Stash it for now | Off the working tree and recoverable, but leaves an undated stash entry to rediscover. | |
  | Leave it | Take no action; commits keep needing `--no-verify`. | |
- Feature-gating `chacha20poly1305` / `zeroize` → already owned by ARCH-05, Phase 7.
- The four test-code TODOs in `crates/paladin-battalion/src/` requesting `MockPaladinPort` error
  injection → candidate for DEFER-01's shared mock infrastructure, Phase 15.
- Nyquist validation for Phases 1-4 (all four `VALIDATION.md` files read `status: draft`).
- Whether ADRs should be published to the mdbook for framework consumers → Phase 16.
- The Grove routing threshold competing-variant group (3 names / 3 defaults) — separate from the
  routing-model question, not settled here.
