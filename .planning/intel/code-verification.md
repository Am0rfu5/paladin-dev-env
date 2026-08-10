# Code Verification of Ingest Claims

Direct verification of contested ingest claims against the shipped tree on `release/v0.7.0`,
performed 2026-07-30 during ingest run 2. Evidence is file existence and dependency
declarations, not LLM inference.

**Purpose:** several ingested documents claim work is complete that task checklists show open,
and vice versa. This file records what the code actually contains, so downstream planning does
not re-plan shipped features or drop genuine gaps.

**Precedence note:** shipped code outranks every ingested document. Where this file contradicts
a PRD, DOC, or `task-completion-state.md` count, this file wins.

## Verified SHIPPED

| Feature | Claim in docs | Evidence in tree |
|---|---|---|
| Conclave (mixture-of-agents) | Epic 15 completion report says COMPLETE; `tasks-conclave-mixture-of-agents.md` has **129 open** checkboxes | `crates/paladin-core/src/platform/container/battalion/conclave.rs`, `crates/paladin-battalion/src/conclave_execution_service.rs`, `examples/conclave_expert_panel.rs`, referenced from `battalion/mod.rs` and `commander.rs` |
| Sentinel vision | M3 release notes list vision under "What's Next (Milestone 4)" as *not delivered* | `crates/paladin-ports/src/output/vision_port.rs`, `crates/paladin-ports/src/output/vision_llm_port.rs`, `tests/integration/vision_integration_test.rs`, `examples/vision_analysis.rs`, `examples/vision_battalion.rs`, `docs/src/appendix/battalion-vision-support.md` |
| Qdrant Sanctum adapter | `EPIC_11_COMPLETION_SUMMARY.md` records "Task 5.0: Qdrant Adapter (DEFERRED) — Not implemented" | `qdrant-client = "1.14"` in root `Cargo.toml` (optional, behind `qdrant` feature); integration test target `qdrant_sanctum_integration` |
| Council pattern | Named as "Epic 20" in M3 release notes; belongs to M2 Epic 16 | `examples/council_discussion.rs`, `examples/commander_council.rs`, `tests/integration/commander_integration_tests.rs` |
| Grove pattern | Named as "Epic 21" in M3 release notes; belongs to M2 Epic 16 | `examples/grove_routing.rs`, `examples/commander_grove.rs`, `tests/integration/commander_integration_tests.rs` |
| Maneuver / Flow DSL | Named as "Epic 22" in M3 release notes; belongs to M2 Epic 17 | `examples/maneuver_basic.rs`, `examples/maneuver_nested_flow.rs`, `examples/maneuver_dynamic_flow.rs` |

## Resolved variants

### Vision API surface — BOTH shipped, neither superseded

The run-2 conflict report preserved Epic 13's `VisionCapableLlm` trait surface against Epic 20's
`VisionPort` surface as competing variants. **Both exist in the tree:**

- `crates/paladin-ports/src/output/vision_llm_port.rs` — the Epic 13 lineage
- `crates/paladin-ports/src/output/vision_port.rs` — the Epic 20 lineage

This is not an unresolved contradiction; it is two coexisting ports. Do not plan a migration
from one to the other on the strength of the PRD conflict alone — confirm intent first.

### Milestone 3 epic numbering — plan numbering is authoritative

`RELEASE_NOTES_MILESTONE_3.md` numbers Epics 19-23 as Conclave / Council / Grove / Maneuver /
Commander Enhancement. Those four patterns are Milestone **2** features (Epics 15, 16, 16, 17),
all verified shipped above. `Project_Plan_Milestone_3.md`, the six `epic19..24.md` definitions,
every `prd-*.md`, and every `tasks-*.md` in Milestone 3 instead use: 19 = Herald & Domain Type
Consolidation, 20 = Vision Pipeline Completion, 21 = Autonomous Agent Completion, 22 = Battalion
& Commander Hardening, 23 = CLI/Config/Infrastructure Completion, 24 = Test Hardening.

**Resolution: the plan/epic-definition numbering is authoritative** — 8 of 9 Milestone-3
documents plus all task lists agree. The release-notes numbering is a documentation defect and
must not reach ROADMAP.md as provenance keys.

### Release-notes forward-look is stale

`RELEASE_NOTES_MILESTONE_3.md` "What's Next (Milestone 4)" describes vision and autonomous-agent
work as planned. Vision is verified shipped. Treat that section as a point-in-time forward-look
that was overtaken, not as scope.

## Implication for open-checkbox counts

`task-completion-state.md` records 542 open items across 75 task lists (93.3% complete). The two
largest concentrations — Conclave 129 and Sanctum 111 — are both **shipped**. Checkbox state was
not maintained through to completion in at least these cases, and run 1 independently found the
same pattern (Chain of Command and Herald wiring marked open but implemented).

**Do not treat open checkbox counts as a work backlog.** Every one requires verification against
the tree before it becomes a planned requirement. The genuine remaining-work signal lives in the
Deferred-QA-CICD-Completion and Milestone_8 deferred documents (ingest run 5), not in checkbox
arithmetic.

## Not yet verified

These carry open checkboxes and have not been checked against code:

- `tasks-epic22-battalion-commander-hardening.md` — 81 open
- `tasks-autonomous-agent-features.md` — 45 open
- `tasks-test-hardening-benchmarks-qa.md` — 29 open
- `tasks-content-rewrite.md` — 26 open (Milestone 11 documentation)
- `tasks-harden-port-traits-stable-api.md` — 20 open
- `tasks-provider-expansion.md` — 19 open (Milestone 1; live-API tests explicitly deferred)

---

## Ingest run 3 verification — Milestones 4, 5, 6 (32 docs)

Direct verification against the working tree on `release/v0.7.0`, performed 2026-07-30 during ingest
run 3. Evidence is file existence, `Cargo.toml` contents, and type definitions read from source — not
LLM inference. Same precedence rule applies: this file outranks every ingested document.

### Verified SHIPPED — the workspace decomposition and the M6 relocations

| Claim | Source doc | Evidence in tree |
|---|---|---|
| Cargo workspace with `[workspace] members = [".", "crates/*"]` | M5 Epic 1 PRD FR-1 | root `Cargo.toml` |
| `paladin-core` extracted | M5 Epic 1 | `crates/paladin-core/` (package name `paladin-ai-core`, lib name `paladin_core`) |
| `paladin-ports` extracted, `src/application/ports/` fully deleted | M5 Epic 2 FR-16 | `crates/paladin-ports/`; `src/application/ports` does not exist |
| `paladin-battalion` extracted | M5 Epic 3 | `crates/paladin-battalion/` |
| `paladin-llm` extracted with per-provider features | M5 Epic 4 | `crates/paladin-llm/`; root dep enables `openai, anthropic, deepseek, mock, vision` |
| `paladin-memory` extracted, edition 2024, `doctest = false`, features `sqlite`/`qdrant`/`content-processing` | M5 Epic 5 FR-1.2/1.3/1.6 | `crates/paladin-memory/Cargo.toml` — exact match |
| `paladin::prelude` | M5 Epic 6 FR-1.4 | `src/prelude.rs` |
| `crate-isolation` CI job | M5 Epic 6 FR-2.8 | `.github/workflows/ci.yml:228` |
| `--workspace` clippy / doc / test | M5 Epic 6 FR-2.3/2.5/2.6/2.9 | `ci.yml:54,57,222,225` |
| feature-flags workspace matrix | M5 Epic 6 FR-2.4 | `feature-flags.yml:115,118` |
| `benchmark-builds.sh` | M5 Epic 6 §6 | `scripts/benchmark-builds.sh` |
| `application_settings.rs` deleted, replaced by per-domain config modules | M6 Epic 1 SM-3 | `src/config/{agents,arsenal,citadel,env_utils,file_storage,herald,notifications,queue,scheduler,settings,web_server}.rs`; no `application_settings.rs` |
| Config modules pushed into sub-crates | M6 Epic 1 §4.1 | `crates/paladin-memory/src/config/{garrison,rag,sanctum}.rs`; `crates/paladin-llm/src/config/{llm,vision,bridge}.rs` |
| Orchestration services relocated out of `core/platform/manager/` | M6 Epic 2 §4.3 | `src/application/services/{notification_orchestrator,queue_orchestrator,log_orchestrator,orchestration}/`; `src/core/platform/manager/` retains only `content_service.rs`, `event_manager.rs`, `user_service.rs`, `mod.rs` |
| Maneuver DSL co-located with Battalion | M6 Epic 3 §4.1-4.5 | `crates/paladin-battalion/src/maneuver/{mod.rs,parser/{mod,lexer,ast,error}.rs,service.rs,visualizer.rs}`; no `parser/` dir and no `maneuver.rs` in `paladin-core` |
| `CircuitBreaker` relocated to infrastructure, old path retired | M6 Epic 4 §4.1/4.11 | `src/infrastructure/resilience/{mod.rs,circuit_breaker.rs}`; `src/application/use_cases/` no longer exists at all |
| Epic-1 decision record Option A implemented | M5 Epic 1 decision doc | `paladin-core/src/platform/container/execution_result.rs` (`PaladinResult`, `StopReason`), `token_usage.rs` (`TokenUsage`), `registry_error.rs` (`RegistryError`), `arsenal/handoff_error.rs` (`HandoffError`) |
| `default = ["llm-openai"]` | M4 Epic 1 FR2 | root `Cargo.toml [features]` |
| `full` convenience flag | M4 Epic 1 FR1 | root `Cargo.toml [features]` |
| `cli` feature + `required-features` on the CLI binary | M4 Epic 3 FR1/FR3 | `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`; `[[bin]] paladin-cli` has `required-features = ["cli"]` |
| Library-only isolation test | M4 Epic 3 FR6 | `feature-flags.yml:141` runs `cargo test --test cli_isolation` |
| API-surface tooling | M4 Epic 2 FR-7 | `scripts/{extract-public-api,check-api-surface,check-deprecations,check-all-examples}.sh`; `final-api.txt`, `api_surface_current.txt` |

The workspace is **larger** than run 3 describes: 10 library crates ship
(`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`, `paladin-llm`,
`paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`)
plus a `doc-examples` crate — not the 6 that `build-benchmarks.md` and the M5/M6 overviews
describe. The additional five crates come from milestones outside this run.

### Resolved variants — settled by shipped code

#### `BattalionResult` field set — run-1 variant CLOSED (merged superset shipped)

`crates/paladin-core/src/platform/container/battalion/mod.rs:549`. The shipped struct is a
**superset of all three run-1 consumers**, with Epic 5's `metadata` map flattened into top-level
fields:

`battalion_id`, `battalion_name`, `started_at`, `completed_at`, `final_output`,
`paladin_results: Vec<PaladinResult>`, `status: BattalionStatus`,
`strategy_used: BattalionStrategy`, `strategy_selection_reasoning: Option<String>`,
`strategy_selection_time_ms: u64`, `per_paladin_times: HashMap<String, u64>`,
`per_paladin_tokens: HashMap<String, TokenUsage>`, `total_tokens: u64`,
`paladin_success_count: usize`, `paladin_failure_count: usize`,
`node_errors: Vec<NodeError>`.

- Epic 4's field set (`REQ-battalion-result-v1`): fully present.
- Epic 5's field set (`REQ-battalion-result-v2`): present except `execution_time_ms`
  (superseded by `per_paladin_times`) and `errors: Vec<PaladinError>` (superseded by
  `node_errors: Vec<NodeError>` — a plain-data struct, because `BattalionError` does not derive
  `Serialize`/`Deserialize` while `BattalionResult` does).
- Epic 8's Herald expectation (`REQ-herald-battalion-result-fields`): satisfied —
  Battalion type is available as `strategy_used`, aggregated token usage as `total_tokens` plus
  `per_paladin_tokens`.

**Resolution: the run-1 `BattalionResult` variant is closed by code.** Do not plan a
reconciliation task. The M5 Epic 1 decision record does *not* settle this — it never mentions
`BattalionResult` — the shipped struct does.

#### `BattalionConfig` field set — run-1 variant CLOSED (Epic 4 form shipped)

`battalion/mod.rs:37`: `name`, `description: Option<String>`, `timeout_seconds`,
`retry_policy: RetryPolicy`, `error_strategy: ErrorStrategy`,
`metadata_output_dir: Option<PathBuf>`. This is `REQ-battalion-config-v1` exactly. Epic 5's
`retry_attempts: u32` and `enable_checkpointing: bool` are **not** present, and `description`
was not dropped.

#### `metadata_output_dir` ownership — run-2 three-owner warning CLOSED

Exactly one owner in the tree: `BattalionConfig` (`battalion/mod.rs:54`). `CommanderConfig`
**does not exist anywhere** in `crates/` or `src/`, so Epic 22's
`REQ-commander-config-metadata-dir-v3` was never built. No reconciliation is needed.

#### Competing `ErrorStrategy` variant sets — run-2 warning CLOSED (two distinct enums)

Two enums, two crates, both shipped, exactly as documented:

- `crates/paladin-core/src/platform/container/battalion/mod.rs:240` — `FailFast` (default),
  `ContinueOnError`, `RetryThenContinue` (Battalion).
- `crates/paladin-battalion/src/maneuver/mod.rs:18` — `FailFast` (default), `ContinueParallel`,
  `IgnoreErrors` (Maneuver).

M6 Epic 3 physically separated them into different crates, which removes the name collision as a
practical concern. Both requirement entries stand as describing different types.

#### Battalion base module path — run-1 warning CLOSED

`battalion/mod.rs` is confirmed; `battalion.rs` does not exist. The Epic 4 section of
`Paladin Project Completion Plan.md` was wrong.

#### Documentation deliverables — NOT missing, relocated into the mdbook

`STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and `docs/CONFIGURATION.md` do not
exist at the paths named by six run-3 documents, but equivalent pages ship in the mdbook:
`docs/src/api-reference/{stable-api,feature-flags,migration-guide,crate-map}.md` and
`docs/src/getting-started/installation.md`. The relocation happened during the Milestone 11
documentation overhaul (ingest run 5). **Do not plan these as missing deliverables.**

### Verified OPEN — genuine remaining work found in run 3

1. **`api-surface` CI job is broken by a stale path.** `scripts/check-api-surface.sh` and
   `scripts/extract-public-api.sh` default their baseline to `project/current-exports.txt`, and
   `ci.yml:171,181,186` pass that literal path. The directory was renamed in commit `928c6d5`
   ("chore: moved project to .project"); the baseline now lives at `.project/current-exports.txt`.
   `check-api-surface.sh` exits 1 with "No baseline found" when the file is absent, so the job
   fails on every run. Five stale references to fix (2 scripts, 3 workflow lines).

2. **Zero `#[deprecated]` annotations in the tree.** `grep -rn '#\[deprecated' src crates`
   returns 0. M4 Epic 2 FR-8 requires them for transitional types, and `DEPRECATIONS.md`
   self-reports "Deprecated Items: 0 (none yet)". This is consistent with the 20 open checkboxes
   in `tasks-harden-port-traits-stable-api.md` — Epic 2 of Milestone 4 is the one genuinely
   incomplete epic in this run.

3. **`.public-api-baseline.txt` was never created.** M4 Epic 2 FR-7.3 names it; the project
   instead uses `.project/current-exports.txt` plus `final-api.txt` / `api_surface_current.txt`.
   Path naming, not missing capability — but item 1 above must be fixed for it to work.

4. **`paladin-ports` doctests are disabled with a named follow-up.**
   `crates/paladin-ports/Cargo.toml` sets `[lib] doctest = false` with the comment: "Doctests in
   copied port files reference `paladin::` (root crate) which would require a circular
   dev-dependency. Re-enable in Task 7.0 after rewriting examples to use `paladin_ports::` /
   `paladin_core::` paths." `ci.yml:225` correspondingly runs
   `cargo test --workspace --doc --exclude paladin-ports`. This directly contradicts M5 Epic 2
   FR-21 and Success Metric 8.

5. **CLI dependency isolation is only partly done.** `cli` gates 5 of the 8 dependencies the
   PRD and the dependency matrix classify as CLI-only. Still unconditional in root
   `Cargo.toml`: `structopt = "0.3"` (line 93), `colored = "2.1"` (line 125),
   `comfy-table = "7.1"` (line 126). `tasks-cli-isolation` shows no open items, so this is a
   checkbox-versus-code gap in the opposite direction from runs 1-2.

6. **Three competing `TokenUsage` definitions ship simultaneously.**
   `paladin-core/src/platform/container/token_usage.rs:13`,
   `paladin-core/src/platform/container/battalion/mod.rs:497`, and
   `paladin-llm/src/llm_analysis_service.rs:51`. This is exactly the duplication run 1 warned
   about and run 2's `REQ-herald-type-consolidation` was meant to close. The Epic-1 decision
   record moved *one* `TokenUsage` into `paladin-core/token_usage.rs` but the battalion-local and
   llm-local copies remain.

### Crate-level facts that contradict run-3 requirement text

| Requirement | Doc position | Shipped |
|---|---|---|
| Workspace crate edition | 2021 (M5 Epics 1-4 PRDs) / 2024 (M5 overview + Epic 5 PRD) | **Mixed**: root, `paladin-core`, `paladin-memory` = 2024; `paladin-ports` = 2021 |
| `paladin-core` dependency allowlist | "complete and exhaustive": serde, serde_json, uuid, chrono, thiserror, async-trait (6) | **14**: those 6 plus `tokio`, `sha2`, `blake3`, `petgraph`, `murmur3`, `url`, `regex`, `futures` |
| `paladin-ports` dependency allowlist | paladin-core, async-trait, serde, thiserror, uuid, chrono, tokio (7) | **10**: those 7 plus `serde_json`, `futures`, `md5` |
| LLM config bridge location | root crate `src/infrastructure/adapters/llm/config_bridge.rs`; `paladin-llm` must not own config | `crates/paladin-llm/src/config/bridge.rs` — the bridge moved **into** `paladin-llm` |
| `vision` feature gates `chacha20poly1305` + `zeroize` | M4 Epic 1 PRD FR1 | `vision = []` gates no dependencies; neither crate is feature-gated. The dependency-matrix DOC ("general encryption in `security/encryption.rs`, **not** vision-specific") was correct and the PRD was wrong |
| `web-server` gates `actix-web` and `axum` | M4 Epic 1 PRD FR1 | `web-server = ["dep:paladin-web", "dep:axum"]` — axum only; actix-web is no longer a root dependency |
| MCP transport feature flags (`mcp-transports` / `mcp-stdio` / `mcp-sse`) | M4 milestone overview AC 1 + Appendix B | No MCP feature flag of any kind exists. The PRD's 2026-04-15 elimination note is what shipped |
| `paladin-cli` as a workspace crate | M5 overview target structure + Appendix D | No `paladin-cli` crate. CLI is a `cli` feature plus `[[bin]] paladin-cli`. M5 Epic 6 PRD's non-goal was correct |
| Config decomposition target map | M6 overview AC 1 vs M6 Epic 1 PRD §4.1 | **Hybrid**: PRD's split into sub-crates shipped (`paladin-memory/config`, `paladin-llm/config`) and the PRD's facade files shipped (`herald.rs`, `citadel.rs`, `scheduler.rs`), but the file is `agents.rs` (the overview's name) and there is a separate `settings.rs`; no `battalion.rs`, `logging.rs`, `llm.rs` or `garrison.rs` in `src/config/` |
| Binary targets | M4 Epic 3 Q1 unresolved ("architecture review") | Three targets ship: `paladin` (`src/main.rs`), `paladin-cli` (`required-features = ["cli"]`), `paladin-server` (`required-features = ["web-server"]`). Resolved by outcome, not by a recorded decision |
| M6 Epic 2 target directory | `src/application/use_cases/` | `src/application/use_cases/` no longer exists; the four orchestrator modules ship under `src/application/services/` with the PRD's exact module names |

### Implication for run-3 open-checkbox counts

`task-completion-state.md` records Milestone 4 at 93.2% (20 open, all in
`tasks-harden-port-traits-stable-api.md`), Milestone 5 at 96.4% (17 open) and Milestone 6 at
100.0% (0 open).

- **Milestone 4's 20 open items are corroborated** — items 2 and 3 above (no `#[deprecated]`
  annotations, no baseline file) are real. This is the first run where the checkbox count
  understates nothing and is directly supported by code.
- **Milestone 5's 17 open items are contradicted for the most part** — all six crates, the
  prelude, the CI isolation job and the benchmark report exist. Items 1 and 4 above are the
  residue worth carrying forward.
- **Milestone 6's 0 open items are corroborated** — all four relocations are verifiably complete
  in the tree.

---

## Ingest run 4 verification — Milestones 7 and 8 (40 docs)

Direct verification against the working tree on `release/v0.7.0`, performed 2026-07-30 during ingest
run 4. Evidence is file existence, `Cargo.toml` / `deny.toml` / `audit.toml` / workflow contents, and
grep counts read from source — not LLM inference. Same precedence rule applies: this file outranks
every ingested document.

Run 4 is the first run where the ingest corpus contains a document that **audits itself against the
tree** — `facade-cleanup-RECONCILIATION-2026-06-04.md`. Its findings are corroborated below almost
without exception, which makes it the most reliable status document in the 153-document corpus.

### Verified SHIPPED — Milestone 8 landed further than its own planning documents record

| Claim | Source doc | Evidence in tree |
|---|---|---|
| `paladin-herald` created as a new leaf crate | Reconciliation §7 commit `66f6c4e` | `crates/paladin-herald/src/{lib,json_herald,markdown_herald,table_herald}.rs`; `paladin-herald = { version = "0.6.0", path = "crates/paladin-herald" }` in `[workspace.dependencies]`; non-optional facade dependency |
| `FileCitadel` relocated to `paladin-memory` | Reconciliation §7 commit `8bd7073` | `crates/paladin-memory/src/citadel/file_citadel.rs` |
| MinIO/S3 and Redis queue relocated to `paladin-storage` | Reconciliation §7 commits `ff829e2`, `5a7c901` | `crates/paladin-storage/Cargo.toml [features]`: `s3 = ["dep:rust-s3"]`, `redis-queue = ["dep:redis"]`; facade `s3-storage = ["paladin-storage/s3"]`, `redis-queue = ["paladin-storage/redis-queue"]` |
| `paladin-storage` made non-optional; `storage-sqlite` retired | Reconciliation §7 commit `897e77e` | root `Cargo.toml`: `paladin-storage = { workspace = true, features = ["sqlite"] }` (no `optional = true`), with the inline comment "SQLite repositories are always available: `paladin-storage` is a non-optional dependency with its `sqlite` feature enabled". Only `storage-mysql` and `storage = ["storage-mysql"]` remain |
| All 25 List A files deleted, plus the orphaned directories | M8 Epic 2 PRD; Reconciliation §2 | `src/application/` contains only `cli`, `errors`, `mod.rs`, `services`; no `notifications/`, no `storage/`; `src/core/platform/manager/` has no `admin/` or `user/` |
| `src/core/` reduced to exactly six files | M8 Epic 2 PRD §4.3 | `src/core/{mod.rs, platform/mod.rs, platform/manager/{mod,content_service,event_manager,user_service}.rs}` — exactly the six named, no more |
| `use_cases` → `services` rename complete, facade **and** leaf crate | M8 Epics 4 and 6 | `src/application/services/` with 11 sub-modules; `crates/paladin-content/src/services/`; `crates/paladin-content/src/lib.rs` declares `pub mod services;`; `grep -rn "use_cases" src/ crates/ tests/ examples/ benches/ --include="*.rs"` returns **zero matches** |
| actix-web removed from `paladin-web` and banned | M8 Epic 7 | `grep -rn "actix" crates/paladin-web/` returns **zero matches**; `deny.toml:99-103` `[bans] deny = [{ crate = "actix-web", reason = "paladin-web standardizes on axum; no second web framework" }]` |
| Delivery endpoints revived as mounted axum routes | M8 Epic 7 FR 1-3 | `crates/paladin-web/src/delivery_controller.rs` documents `POST /api/delivery/deliver`, `GET /api/delivery/status/{delivery_id}`, `GET /api/delivery/stats`; `app.rs:24` imports `create_delivery_routes`, `app.rs:63` calls `.merge(create_delivery_routes(deliverer))` |
| TensorFlow adapter and `ml` feature removed entirely | `deferred-features.md` §2 | `grep -rn "tensorflow\|^ml = " Cargo.toml src/` returns **zero matches** |
| CLI `user` command removed | `deferred-features.md` §1 | `src/application/cli/commands/` contains `agent.rs`, `arsenal.rs`, `battalion.rs`, `council.rs`, `features.rs`, `maneuver.rs`, `mod.rs`, `muster.rs`, `onboarding.rs`, `setup_check.rs` — no `user.rs` |
| Facade role documented | M8 Epic 5 FR-1, FR-2 | `src/README.md` exists (3,750 bytes) |
| Benchmarks migrated to owning crates; zero disabled | M7 Epic 3 | `benches/config_benchmarks.rs`; `crates/paladin-battalion/benches/battalion_benchmarks.rs`; `crates/paladin-llm/benches/llm_serialization_benchmarks.rs`; `crates/paladin-memory/benches/{sanctum,garrison}_benchmarks.rs`. No `*.disabled` benchmark file anywhere; no `herald_benchmarks`, `paladin_benchmarks` or `arsenal_benchmarks` |
| CI benchmark regression signal | M7 Epic 3 FR-24 | `ci.yml:531` job `benchmark-regression-signal` |
| All ten per-crate Makefile test targets | M7 Epic 2 FR-18 | `Makefile:167-212` — `test-core`, `test-ports`, `test-battalion`, `test-llm`, `test-memory`, `test-storage`, `test-notifications`, `test-content`, `test-web`, `test-facade` |
| Dockerfile.chef workspace adaptation | M7 Epic 2 FR-01 to FR-06 | `Dockerfile.chef`: `cargo install cargo-chef --version 0.1.77 --locked` (pinned per §7); per-crate `COPY crates/*/Cargo.toml` lines; `cargo chef prepare --recipe-path recipe.json`; `cargo chef cook --release --workspace --recipe-path recipe.json`; `rust:1.93-slim-bookworm` |
| crates.io package renames | Epic 4 completion summary | root `[package] name = "paladin-ai"` with `[lib] name = "paladin"`; `paladin-core = { package = "paladin-ai-core", … }` in `[workspace.dependencies]` |
| Per-crate READMEs and CHANGELOGs | M7 Epic 4 §4.2, §4.3 | Nine of ten library crates have both. See the gap below for `paladin-herald` |
| `println!` hygiene sweep | Reconciliation §7 `4c7857e`; `deferred-items.md` D5 | `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` returns **exactly 17** matches across **exactly 6** files: `services/herald/herald_registry.rs`, `services/paladin/paladin_execution_service.rs`, `infrastructure/adapters/arsenal/{mcp_protocol,tool_result_formatter}.rs`, `infrastructure/adapters/scheduling/tokio_cron_adapter.rs`, `infrastructure/resilience/circuit_breaker.rs` |

**The reconciliation's `deferred-items.md` D5 count is exact.** Both the file count and the
occurrence count match the tree precisely. This is the highest-fidelity claim in the corpus and is
the strongest single reason to treat the two deferred registers as the authoritative Milestone 8
forward-work source.

### Verified OPEN — genuine remaining work found in run 4

1. **The RustSec exception list has grown from two to five, and the three files that encode it
   disagree.** This is the most consequential open finding in run 4.
   - `.cargo/audit.toml` `[advisories] ignore` carries **five vulnerability advisories**:
     `RUSTSEC-2023-0071`, `RUSTSEC-2025-0111`, `RUSTSEC-2026-0187` (lopdf stack overflow via
     deeply nested PDF objects, transitive through `pdf-extract` under `content-processing`),
     `RUSTSEC-2026-0194` and `RUSTSEC-2026-0195` (quick-xml quadratic attribute parsing and
     unbounded namespace allocation, transitive through `rust-s3`/`aws-creds` under `s3`).
   - `deny.toml` `[advisories] ignore` carries a comment stating "the same advisory IDs are
     mirrored here so cargo-deny and cargo-audit do not contradict each other. Keep these two files
     in sync" — but mirrors **only the original two** vulnerability IDs (plus six *unmaintained*
     notices, which are explicitly a different class). The three 2026 advisories are absent.
   - `.github/workflows/ci.yml:406` runs
     `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` — the **original two only**,
     passed on the command line, while `make audit` (`Makefile:244-247`) runs a bare `cargo audit`
     with the comment "Exceptions are sourced from .cargo/audit.toml (single source of truth)".
   - So local and CI audit surfaces are configured differently, and `deny.toml`'s own stated
     invariant is violated. Three files, three different exception sets.
   - `rustsec-remediation-plan.md` records the acceptance owner as **Platform Security
     (Milestone 7)** with a **review/expiry target of 2026-09-30** — approximately two months from
     the ingest date. Nothing in `.planning/` other than this record surfaces that date.
   - The three 2026 advisories carry documented, dated reasoning inside `.cargo/audit.toml`
     (including that the directly-fixable RUSTSEC-2026-0185 quinn-proto and RUSTSEC-2026-0190
     anyhow were **upgraded rather than ignored**), so this is drift in governance surface, not
     undocumented risk-taking.

2. **`paladin-herald` has no `CHANGELOG.md`.** M7 Epic 4 §4.3.1 and AC 3 require a crate-level
   `CHANGELOG.md` for **every** public crate, and the Epic 4 completion summary records that
   criterion as Met. `paladin-herald` was created afterwards by the reconciliation (commit
   `66f6c4e`) and has a `README.md` but no `CHANGELOG.md`. All nine other library crates have both.
   Small, concrete, and directly traceable to a shipped acceptance criterion.

3. **`Dockerfile.chef`'s explicit planner COPY list is one crate stale.** It copies nine
   `crates/*/Cargo.toml` files by name — core, ports, battalion, llm, memory, storage,
   notifications, content, web — and omits `crates/paladin-herald/Cargo.toml`. A later
   `COPY crates ./crates` means the build still works, but M7 Epic 2 FR-01 requires all crate
   manifests in the planner stage precisely so the dependency layer invalidates correctly; a
   `paladin-herald` manifest change will not tighten the recipe layer as intended.

4. **The `api-surface` CI baseline path is still broken, and run 4 re-asserts it.** Run-3
   verification recorded five stale references to `project/current-exports.txt` after the directory
   was renamed to `.project/` in commit `928c6d5`. All five are unchanged: `scripts/check-api-surface.sh:6`
   and `scripts/extract-public-api.sh:6` default to `project/current-exports.txt`, and
   `ci.yml:171,181,186` pass that literal path. `.project/current-exports.txt` exists (442 KB);
   `project/current-exports.txt` does not. **M8 Epic 7 FR-10 mandates
   `./scripts/extract-public-api.sh project/current-exports.txt`** — the same stale path — so the
   defect is now written into a run-4 requirement as well as into the tooling.

5. **`paladin-ports` doctests remain disabled.** `crates/paladin-ports/Cargo.toml` still sets
   `[lib] doctest = false` with the identical run-3 comment naming "Task 7.0" as the re-enable
   point. Unchanged since run 3. This sits directly under M7 Epic 4 §4.4.3's requirement that
   `cargo doc --workspace --no-deps` complete without documentation warnings and §4.4.4's >90%
   coverage target.

6. **The `paladin-ports` publish follow-up has no dedicated guardrail.**
   `deferred-paladin-ports-publish-verification.md` closes with "Keep CI/package guardrails that
   detect crates.io package-name collisions early." The `publish-dry-run` job would surface a
   collision at dry-run time, but there is no earlier or name-specific check. Given that collisions
   cost Epic 4 two package renames and a NO-GO cycle, this is worth a decision rather than silent
   reliance on the dry run.

### Superseded by shipped outcome — do not plan these as written

| Requirement as written | Where it came from | What the tree says |
|---|---|---|
| `paladin-web` declares `actix-web` **and** `axum` as direct non-optional dependencies | M7 Epic 1 PRD §4.2.1 | Zero `actix` references in `crates/paladin-web/`; facade `web-server = ["dep:paladin-web", "dep:axum"]`. M8 Epic 7 reversed it deliberately and added a cargo-deny ban |
| Facade `storage-sqlite` flag; `paladin-storage` optional; `storage` alias enables both backends | M7 Epic 1 PRD §4.5.6, §7.2 | `storage-sqlite` retired; `paladin-storage` non-optional with `sqlite` always on; `storage = ["storage-mysql"]` |
| `sqlx` workspace declaration includes `mysql` in its feature list | M7 Epic 1 PRD §7.5 | `sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json", "migrate"] }` — no `mysql`; `default-features = false` and `migrate` added by the RustSec hardening work |
| `publish-dry-run` runs `cargo publish --dry-run -p <crate>` for ten crates in dependency order | M7 Epic 2 FR-26 | A single `cargo publish --workspace --dry-run`, with an inline rationale that per-crate dry runs "cannot work on a version bump: the not-yet-published new version of each sibling fails the `version = \"X\"` requirement of its dependents" |
| `tensorflow_adapter.rs` gated behind a new `ml = []` feature | M8 Epic 3 PRD §4.3 item 11 | Both the adapter and the flag were subsequently deleted outright (`3d48768`); neither exists |
| Every adapter group "Stays in facade", all List B moves deferred to Milestone 9 | M8 Epic 3 PRD §4.3, §5; `infrastructure-adapter-disposition.md` | The relocations were executed in Milestone 8 by the reconciliation; `paladin-herald` was created despite the explicit non-goal "No new crates created — `paladin-herald`, `paladin-ml`, etc. are not in scope" |
| `find src/ -name "*.rs" \| wc -l` = **163** after Epic 2, **160** after Epic 4 | M8 Epic 2 §7; M8 Epic 4 §4.5 item 9 | **136**. The two PRD figures are internally consistent with each other (163 − 3 storage shims = 160); the further reduction is the reconciliation's Category 1-2 deletions, which the PRDs did not anticipate |
| `STABLE_API.md` at the repository root, updated by four separate run-4 requirements | M7 Epic 4 §4.6; M8 Epics 2, 4, 5 | No `STABLE_API.md` at the root. Run-3 verification established the equivalent page ships as `docs/src/api-reference/stable-api.md` after the Milestone 11 overhaul. `api_surface_current.txt` (881 KB) and `final-api.txt` (198 KB) **do** exist at the root |
| `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`, `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md` | M7 Epics 2, 3, 4 | `docs/` holds only `MIGRATION_LOG.md` plus the mdbook. Equivalents ship as `docs/src/appendix/{performance-baseline,release-checklist,release-automation,integration-tests,build-baselines}.md` — the same Milestone 11 relocation run 3 documented. **Do not plan these as missing deliverables** |
| Nine leaf crates named in the facade role documentation | M8 Epic 5 FR-1 | Ten library crates ship; the FR-1 list predates `paladin-herald` |
| `crates/paladin-cli/` as a workspace crate, publish Step 5 | M7 overview Appendices A and B | No `paladin-cli` crate — re-confirming the run-3 finding. `crates/` holds `doc-examples` plus the ten library crates |
| Extracted crates depend only on `paladin-ports`, `paladin-core` and workspace-shared deps; "no extracted crate may depend on another extracted crate" | M7 Epic 1 PRD §6.1 | `crates/paladin-content/Cargo.toml` declares `paladin-llm = { …, optional = true }` behind its `llm` feature — an extracted-to-extracted edge. The PRD's own §4.4 complexity note anticipated it without amending the rule |
| Facade `content-processing` activates `paladin-content` "with all capability features enabled" | M7 Epic 1 PRD §4.4.6 | `content-processing` enables `web-scraping`, `rss`, `news-api`, `tiktoken`, `llm` — but **not** `pdf`. `paladin-content` does declare `pdf = []`, gating no dependency |

### Claims contradicted by code in the *favourable* direction

- **Milestone 8 Epic 6 is complete.** The reconciliation records it "Not verified; low priority"
  with no execution-log entry, and `deferred-items.md` does not list it. The tree shows the rename
  fully done: `crates/paladin-content/src/services/` exists, `lib.rs` declares `pub mod services;`,
  `use_cases/` is gone, and a workspace-wide grep for `use_cases` across `src/`, `crates/`,
  `tests/`, `examples/` and `benches/` returns zero matches. Do not plan Epic 6 as outstanding.
- **Milestone 8 Epic 3 is complete in substance, not merely deferred.** The Epic 3 PRD and the
  disposition record both leave every relocation to Milestone 9; the reconciliation executed them
  and the tree confirms every target. The `task-completion-state.md` figure of 3 open items for
  Milestone 8 (1 in Epic 3, 2 in Epic 2) reflects checkbox state on documents the reconciliation
  superseded.

### Implication for run-4 open-checkbox counts

`task-completion-state.md` records Milestone 7 at 98.8% (3 open, all in
`tasks-production-build-infra-adaptation.md`) and Milestone 8 at 99.1% (3 open: 2 in
`tasks-remove-dead-shims-empty-modules.md`, 1 in `tasks-relocate-remaining-misplaced-modules.md`).

- **Milestone 8's 3 open items are contradicted.** Both Epic 2 and Epic 3 are verifiably complete in
  the tree, and Epic 3 went further than its own task list scoped. This is the same
  checkbox-understates-reality pattern runs 1 and 2 found for Conclave and Sanctum.
- **Milestone 7's 3 open items are plausible but not corroborated by any single artifact.** Epic 2
  is the one Milestone 7 epic whose deliverables are partly absent from the tree at the paths the
  PRD names (`docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md`), and partly relocated into the
  mdbook. The genuine Epic 2 residue is items 3 and 4 above — the stale `Dockerfile.chef` COPY list
  and the broken `api-surface` baseline path.
- The real Milestone 7/8 forward work is **not** in the checkbox arithmetic. It is: the RustSec
  exception drift and its 2026-09-30 expiry (item 1); the five deferred items D1-D5; the two
  deferred features with their reintroduction conditions; and the four small verified defects
  (items 2-5).

---

## Ingest run 5 verification — Milestones 9-12, Deferred-QA-CICD-Completion, project-management (46 docs)

Direct verification against the working tree on `release/v0.7.0`, performed 2026-07-30 during ingest
run 5. Evidence is file existence, `Cargo.toml` / `deny.toml` / `audit.toml` / workflow / `Makefile`
contents, line-addressed greps and literal line counts read from source — not LLM inference. Same
precedence rule applies: this file outranks every ingested document.

**This is the final run.** Run 5 is the first to verify a milestone whose planning documents are
younger than most of the tree (Milestone 12, created 2026-06-07 to 2026-06-09), and the first to find
an entire ingested epic-set (Deferred-QA Epics 25-27) essentially unimplemented.

### Verified SHIPPED — Milestones 9, 10, 11 and 12 all landed

| Claim | Source doc | Evidence in tree |
|---|---|---|
| `execute_workflow()` replaces the `println!` arms | M9 Epic 1 FR 1-10 | `src/application/services/orchestration/mod.rs:382` `pub async fn execute_workflow`, plus `execute_workflow_inner` at `:403` |
| `WorkflowRepository` output port | M9 Epic 1 FR-17 | `crates/paladin-ports/src/output/workflow_repository_port.rs` |
| SQLite `WorkflowRepository` adapter in `paladin-storage` | M9 Epic 1 FR-19, OQ-4 | `crates/paladin-storage/src/sqlite_workflow_repository.rs` — Open Question 4's default placement is what shipped |
| Content processors in the **root crate** | M9 Epic 3 FR-1, §7, OQ-1 | `src/application/services/orchestration/processors/` — the circular-dependency resolution shipped as decided |
| `OrchestratorPort` in `paladin-ports` | M9 Epic 4 FR-1 | `crates/paladin-ports/src/output/orchestrator_port.rs` |
| `OrchestratorBridgeAdapter` in the root crate | M9 Epic 4 FR-12 | `src/application/services/orchestration/orchestrator_bridge.rs` |
| `AuthPort` in `paladin-ports`; argon2 retained | M9 Epic 5 FR-5, §7 | `crates/paladin-ports/src/output/auth_port.rs`; `argon2 = "0.5.3"` in root `Cargo.toml:121` |
| Auth adapter in the root crate | M9 Epic 5 FR-8, §6.2 | `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` |
| `paladin-web` auth middleware generic over `Arc<dyn AuthPort>` | M9 Epic 5 FR-13, §6.2 | `crates/paladin-web/src/auth_middleware.rs`; RBAC tests at `crates/paladin-web/tests/auth_rbac.rs` |
| pre-commit framework, version-controlled | M10 Epic 1 FR-1 | `.pre-commit-config.yaml`; CI gate at `.github/workflows/pre-commit.yml` |
| `cargo audit` reading `.cargo/audit.toml` | M10 Epic 2 FR-1 | `ci.yml:62-77` job `security-audit`, bare `cargo audit`, with the inline comment "Exceptions are the single source of truth in `.cargo/audit.toml` … so no inline `--ignore` flags are used here" |
| `cargo deny check` as a required CI gate | M10 Epic 2 FR-13 | `ci.yml:80-105` job `cargo-deny` |
| OSV-Scanner, annotate-only, SARIF | M10 Epic 2 FR-5 to FR-7, OQ-1 | `ci.yml:110-135` job `osv-scanner` using `google/osv-scanner-action@v1.9.1` with SARIF upload; OQ-1's annotate-only recommendation is what shipped |
| Licence allow-list exactly as specified | M10 Epic 2 FR-11 | `deny.toml [licenses] allow` = MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib — **plus four justified additions** (Unicode-3.0, 0BSD, CC0-1.0, CDLA-Permissive-2.0), each with an inline FR-14(a) justification comment |
| Per-crate `clarify`/exception instead of weakening the allow-list | M10 Epic 2 FR-14(b) | Eight `[[licenses.exceptions]]` entries for MPL-2.0 crates (`colored`, `attohttpc`, `cssparser`, `cssparser-macros`, `dtoa-short`, `minidom`, `selectors`, `smartstring`) with the reasoning that MPL-2.0 is *weak, file-level* copyleft. **Textbook compliance with FR-14** |
| `[bans]` starts empty; duplicates warn | M10 Epic 2 FR-12, OQ-4 | `deny.toml [bans] multiple-versions = "warn"`, `wildcards = "warn"`; the only `deny` entry is the Milestone 8 `actix-web` ban |
| CycloneDX SBOM in the release pipeline | M10 Epic 2 FR-15 to FR-17 | `release.yml:328-336` installs `cargo-cyclonedx --locked` and runs `cargo cyclonedx --all --format json`; `Makefile:264` `sbom` target |
| `make security` wrapping audit + deny | M10 Epic 2 FR-19 | `Makefile:261` `security: audit deny` |
| `release.toml` + tag-triggered publish | M10 Epic 3 FR-2, FR-8 to FR-13 | `release.toml`; `release.yml:355` job `publish-crates` |
| `make release` / `make publish-dry-run` / `make release-check` | M10 Epic 3 FR-15, FR-18 | `Makefile:439`, `:424`, `:413` |
| `make hooks` | M10 Epic 1 FR-17 | `Makefile:282` |
| `verify-tag-source` guard with `needs:` wiring | M10 Epic 5 FR-1 | `release.yml:29` job `verify-tag-source`; `:74` and `:97` both declare `needs: verify-tag-source` — the two roots, exactly as FR-1.5 specifies |
| Committed GitHub rulesets | M10 Epic 5 FR-3 | `.github/rulesets/protect-main-branch.json`, `.github/rulesets/protect-release-tags.json` |
| mdbook with linkcheck as an error | M11 Epic 3 FR-1 | `docs/book.toml` `[output.linkcheck] follow-web-links = false`, `warning-policy = "error"` — verbatim |
| mdbook-mermaid preprocessor | M11 Epic 2 §4.1 | `docs/book.toml [preprocessor.mermaid] command = "mdbook-mermaid"`; `docs/mermaid.min.js`, `docs/mermaid-init.js` |
| Migration log | M11 Epic 2 §4.9 | `docs/MIGRATION_LOG.md` |
| Full chapter hierarchy | M11 Epic 2 §4.2 | `docs/src/{getting-started,architecture,user-guides,deployment,deployment-topologies,operations,api-reference,contributing,appendix}` + `SUMMARY.md` + `introduction.md` |
| All six deployment-topology pages | M11 Epic 6 FR-1 to FR-7 | `docs/src/deployment-topologies/{overview,embedded-library,battalion-orchestration,http-service-host,queue-worker,sidecar}.md` |
| Agent registry + controller in `paladin-web` | M12 Epic 1 §4.1, §7 | `crates/paladin-web/src/agent_registry.rs`, `agent_controller.rs` |
| `paladin-server` binary | M12 Epic 2 §4.4 | `src/bin/paladin-server.rs`; `Cargo.toml:249-251` `[[bin]] name = "paladin-server"` |
| SSE streaming + in-process jobs | M12 Epic 3 §4.4, §4.6 | `crates/paladin-web/src/job_store.rs`, `timeout.rs` |
| Unified error envelope, health/ready, request logging, layers | M12 Epic 4 §4.1-4.6 | `crates/paladin-web/src/{error,health,request_log,http_layers}.rs` |
| Rate limiting via tower-governor | M12 Epic 4 §4.5 | `crates/paladin-web/Cargo.toml:33` `tower_governor = { version = "0.8", features = ["axum"] }` |
| Agent auth: API key + bearer, constant-time, redaction-tested | M12 Epic 5 §4.1, §7 | `crates/paladin-web/src/agent_auth.rs` — bearer checked first then `x-api-key` (FR-3's documented precedence), `AgentAuthConfig { enabled, api_keys, jwt: Option<Arc<dyn AuthPort>> }`, plus a `MockJwt` test double and a test asserting a key value does not leak |
| OpenAPI generation, Swagger UI, drift baseline | M12 Epic 6 §4.1-4.4 | `crates/paladin-web/src/openapi.rs`; `crates/paladin-web/openapi.json` (the committed baseline); `utoipa = "5"`, `utoipa-axum = "0.2"`, `utoipa-swagger-ui = "9"` |
| Container image, compose, k8s manifests with probes | M12 Epic 7 §4.1-4.2 | `Dockerfile.server`; `docker/docker-compose.yml`; `k8s/{deployment,service,configmap,namespace,secret.yaml.example,redis,minio}.yaml` plus a `k8s/server/` directory |
| Runnable server example | M12 Epic 7 §4.4 | `examples/http_service_host.rs` |
| Workspace at v0.6.0 | M12 Epic 7 §4.6 | root `Cargo.toml:34` `version = "0.6.0"` |
| Deployment-topology docs updated to the shipped API | M12 Epic 7 §4.3 | Greps for "ships no agent-execution", "yours to compose", "compose your own" and "does not run agents" across `docs/src/` return **zero matches**; `http-service-host.md` references `paladin-server` four times |

**Correction (dated 2026-08-10, D-18):** the `v0.6.0` figure above was correct as of the 2026-07-30
ingest and is superseded. Live measured this session: `grep '^version' Cargo.toml` -> root
`Cargo.toml:34` `version = "0.7.0"`; `git tag --sort=-v:refname | head -8` -> `v0.7.1`, `v0.7.0`,
`v0.5.1`, `v0.5.0`, `v0.4.3`, `v0.4.2`, `v0.4.1`, `v0.4.0` — the workspace has shipped two further
releases (`v0.7.0`, then `v0.7.1`) since the `v0.6.0` figure was recorded. This document's own
verification banner (line 3 and elsewhere) already frames its checks as run "against the working
tree on `release/v0.7.0`" — this correction brings the workspace-version row into agreement with
that framing. Original figure retained above; both are recorded, not one replacing the other.

**Milestone 9's and Milestone 10's 0-open checkbox counts are corroborated by artefact, and
Milestone 12's route surface, auth, streaming, jobs, OpenAPI and deployment artefacts all ship.**

### CORRECTION to the run-4 finding — `deny.toml` is now in sync

Run 4 recorded that `deny.toml` mirrors "only the original two" vulnerability IDs and that "the three
2026 advisories are absent". **That is no longer true.** Read directly from the tree:

- `.cargo/audit.toml [advisories] ignore` — **five** vulnerability advisories: `RUSTSEC-2023-0071`,
  `RUSTSEC-2025-0111`, `RUSTSEC-2026-0187`, `RUSTSEC-2026-0194`, `RUSTSEC-2026-0195`.
- `deny.toml [advisories] ignore` — **fifteen** entries in three explicitly labelled classes:
  the **same five** vulnerability advisories (2 under "mirrored from .cargo/audit.toml", 3 under
  "New 2026 DoS advisories in transitive deps of OPTIONAL features"), plus **ten** *unmaintained /
  maintenance-mode* notices under a header stating "These are informational 'unmaintained' notices,
  **NOT vulnerabilities**. cargo-audit (the primary advisory gate) does not fail on unmaintained
  crates; these are ignored here so cargo-deny does not contradict it (Epic 2 FR 6)."

**The vulnerability sets match exactly.** `deny.toml`'s own stated invariant — "Keep these two files
in sync" — is now satisfied. The ten additional entries are a different advisory class with a
documented rationale, and Milestone 10 Epic 4 FR-1 step 5 explicitly authorises adding scoped
`[advisories].ignore` entries **for unmaintained advisories** with an explanatory comment. Those ten
are therefore sanctioned.

### Verified OPEN — genuine remaining work found in run 5

1. **A duplicate `cargo audit` job still carries inline `--ignore` flags, violating the Milestone 10
   Epic 2 success metric.** This is the most consequential run-5 finding and it *narrows and
   corrects* the run-4 framing.

   `ci.yml` contains **two** jobs with the identical display name `Security Audit`:
   - `ci.yml:60` job id `security-audit` — `cargo install cargo-audit --locked` then a bare
     `cargo audit`, preceded by the comment "Exceptions are the single source of truth in
     `.cargo/audit.toml` … so no inline `--ignore` flags are used here." **Compliant with FR-1.**
   - `ci.yml:390` job id `security` — `cargo install cargo-audit` (unpinned) then
     `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`. **Violates FR-1 and §8.**

   **Mechanism:** the Epic 25 PRD's Appendix B ("Current `ci.yml` Job Listing (Pre-Change
   Reference)") tabulates the pre-Milestone-10 pipeline as 7 jobs, of which **#4 is `security`**.
   Milestone 10 Epic 2 **added** the compliant `security-audit` job without removing its predecessor.
   Epic 4's non-goals then froze the configs ("No changes to `deny.toml` or `.cargo/audit.toml`", "No
   new CI jobs — the Epic 3 pipeline is complete"), so nothing in the milestone was positioned to
   catch it. `ci.yml` now has 14 jobs.

   **Consequence beyond tidiness:** the `security` job's inline list covers only 2 of the 5 advisories
   in `.cargo/audit.toml`. `cargo audit` scans `Cargo.lock` irrespective of feature selection, so the
   three 2026 advisories are in scope for it. The two jobs are therefore configured to reach
   *different verdicts on the same tree*, and the milestone's own success metric — "no inline
   advisory-ignore flags remain in CI" — is false on a milestone recorded 100% complete.

   **Fix:** delete `ci.yml:389-406`. One deletion satisfies the origin policy.

2. **The governance gap is owner/expiry coverage, not synchronisation.** All five `.cargo/audit.toml`
   entries satisfy M10 Epic 2 FR-3's four-field schema (advisory ID; affected crate and why present;
   why unfixable; revisit condition) and carry dated reasoning, including the note that the
   directly-fixable `RUSTSEC-2026-0185` (quinn-proto) and `RUSTSEC-2026-0190` (anyhow) were
   **upgraded rather than ignored**. But **FR-3 does not require an owner or an expiry.** Only
   `rustsec-remediation-plan.md` (run 4) adds those, and only for the original two — owner **Platform
   Security**, review/expiry target **2026-09-30**, roughly two months from this ingest.

   So **13 of the 15 `deny.toml` ignores (3 vulnerability + 10 unmaintained) have documented
   reasoning but no named owner and no expiry date.** Nothing in `.planning/` other than the run-4
   remediation plan will surface 2026-09-30, and nothing at all will surface a review date for the
   other thirteen. This is a governance-surface gap, not undocumented risk-taking.

3. **Deferred-QA Epic 25 (CI/CD Pipeline Enhancement) is unimplemented except for one item.**
   Verified item by item:
   - **No `cli-tests` job.** `ci.yml`'s 14 job ids are `lint`, `security-audit`, `cargo-deny`,
     `osv-scanner`, `api-surface`, `test`, `crate-isolation`, `integration-tests`, `security`,
     `docker`, `e2e-tests`, `benchmark`, `benchmark-regression-signal`, `publish-dry-run`.
   - **No `bench-check` job.** (Note the inversion: `benchmark-regression-signal` — which Epic 25
     lists as a *future enhancement* explicitly out of scope — ships from Milestone 7 Epic 3, while
     the compile-check prerequisite does not.)
   - **No `coverage` job.** `grep -n "llvm-cov\|codecov" .github/workflows/ci.yml` returns nothing.
   - **No `.codecov.yml`** (and no `codecov.yml`) at the repository root.
   - **No Makefile targets** `coverage`, `coverage-html`, `test-cli` or `bench-check`; the `Makefile`
     contains no `llvm-cov` reference at all.
   - **Eight deprecated-action references remain:** `actions-rs/toolchain@v1` at `ci.yml:147`,
     `ci.yml:317`, `ci.yml:507` and `integration-tests.yml:71`; `actions/cache@v3` at
     `integration-tests.yml:78`, `:84`, `:90`; `codecov/codecov-action@v3` at
     `integration-tests.yml:123`.
   - **`integration-tests.yml:117-118` still runs `cargo install cargo-llvm-cov` and
     `cargo llvm-cov --features integration-tests --lcov`** — the integration-only coverage path
     Epic 25 was meant to supersede.
   - **DONE:** the dangling `on: schedule` block is gone. `ci.yml` has exactly one `on:` at line 3
     and no `schedule:`/`cron:` key. This is the only FR-25.2 item satisfied.

   **Correction (dated 2026-08-10, D-08):** the 14-job list above is stale. Live measured via
   `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml`, re-run this session: `lint` (`:21`),
   `security-audit` (`:61`), `cargo-deny` (`:81`), `osv-scanner` (`:126`), `api-surface` (`:155`),
   `test` (`:206`), `examples` (`:245`), `crate-isolation` (`:319`), `integration-tests` (`:374`),
   `docker` (`:494`), `kubernetes-smoke` (`:611`), `e2e-tests` (`:718`), `benchmark` (`:779`),
   `benchmark-regression-signal` (`:812`), `publish-dry-run` (`:898`) — **15 jobs**, not 14.

   (The same grep pattern, run against `ci.yml`'s top-level `on:` block at line 3, also matches the
   `push` trigger key at `:9` — that is a workflow trigger, not a fifteenth job, and is excluded
   from the 15-job count above.)

   Two changes account for the delta. The `security` job (this finding's non-compliant duplicate)
   is gone — deleted by Phase 9's plan 09-06, commit `cb75b2b` (SUPPLY-01 closed; see
   `.planning/REQUIREMENTS.md`'s "Verified by Phase 12" block). Two jobs the recorded list does not
   name are present: `examples`, added by commit `8d4ea16` (2026-08-03, "feat(04-03): restore
   release/** CI trigger and add examples feature-matrix job"), and `kubernetes-smoke`, added by
   commit `2526fef` (2026-08-03, "feat(04-03): add Docker budget assertions and kind Kubernetes
   smoke job") — both attributed by `git log -S` on the job-id string, not guessed. Net: 14 − 1 + 2
   = **15**.

   `PIPE-01` (`.planning/REQUIREMENTS.md:2434`) quotes the stale 14-job list verbatim; plan 13-10
   corrects it there. This correction does not re-run or revise Phase 12's 87-hit stale-citation
   inventory (D-07).

4. **Deferred-QA Epic 26's architecture rewrite never happened, and the relocation hid it.**
   `docs/src/appendix/design-and-architecture.md` is **exactly 311 lines** — the identical figure the
   February 2026 PRD cites as the *pre-rewrite* state ("the current `docs/Design/Design_and_Architecture.md`
   (311 lines, 10 sections)"). Case-insensitive whole-word counts in that file:

   `Commander 0`, `Council 0`, `Conclave 0`, `Grove 0`, `Maneuver 0`, `Sanctum 0`, `Sentinel 0`;
   `Paladin 6`, `Garrison 2`, `Arsenal 2`, `Battalion 2`, `Herald 2`, `Citadel 1`.
   ```` ```mermaid ```` blocks: **0**.

   All seven subsystems FR-26.1 requires be documented in detail are absent, and none of the four
   required Mermaid diagrams exists. **Milestone 11 moved the file into `docs/src/appendix/`, and
   Milestone 11 Epic 3's non-goals exempt the appendix from rewriting ("the 35 appendix files are
   reference/archive material and are not rewritten in this Epic").** The relocation preserved the
   document and froze the gap in the one chapter nobody was required to fix.

   Also open from Epic 26: `docs/assets/` **exists and is empty** (no `.cast` recordings), and
   `docs/DEMOS.md` does not exist.

5. **Deferred-QA Epic 27 (LLM tool calling) is entirely unimplemented.**
   `crates/paladin-ports/src/output/llm_port.rs` has **no `tools` field** — the only two occurrences
   of "tools" in the file are doc-comment prose, one of which literally reads
   `// No tools, rely on prompting`. Greps across `crates/paladin-ports/src` and
   `crates/paladin-llm/src` for `struct ToolDefinition`, `struct ToolCall` and `tool_calls` return
   **zero matches**.

   The PRD's own problem statement stands unchanged in the tree: "All three LLM adapters (OpenAI,
   DeepSeek, Anthropic) declare tool-calling capabilities in `ProviderCapabilities` but hardcode
   `function_call: None`." **`ProviderCapabilities` over-reports capability**, which is a correctness
   defect independent of whether tool calling is ever built.

6. **The Epic 28/29 mock infrastructure does not exist in the specified shape.** No `tests/common/`
   directory exists. The workspace's mocks are `tests/helpers/{mock_llm_adapter, mock_arsenal_adapter,
   mock_paladin_port}.rs` plus `tests/unit/mock_llm_adapter_test.rs` — a different location and a
   disjoint set. None of `MockUserRepository`, `MockLogPort`, `MockNotificationService`,
   `MockEventSource` or `MockTriggerExecutor` exists. This is the **shared prerequisite** for both
   coverage epics and the reason the recommended order puts Epic 28 before Epic 29.

7. **The agent API is documented as JWT and implemented as opaque tokens.**
   `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returns **nothing** — the crate is not a
   dependency anywhere in the workspace. The only `AuthPort` implementation is
   `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`, Milestone 9 Epic 5's opaque,
   in-process, hashed-token store. `crates/paladin-web/src/agent_auth.rs` nonetheless documents its
   verifier as JWT throughout — module docs, the `jwt: Option<Arc<dyn AuthPort>>` field, the
   `bearer JWT checked first` comment.

   Milestone 12 Epic 5's **Open Question 4 is unanswered because it is unanswerable for the shipped
   adapter**: "which concrete `AuthPort` impl does `paladin-server` wire, and what does it need
   (signing secret/algorithm) from config/env?" An opaque-token store has no signing secret and no
   algorithm.

   **The operational edge:** Milestone 9 Epic 5 §6.1 recorded the trade-off itself — "tokens are
   validated against an in-process store, so a **multi-process deployment would later need a shared
   store**." Milestone 12 Epic 7 then ships `k8s/deployment.yaml`, whose purpose is multi-process
   serving. Under more than one replica, a token issued by one pod will not verify on another.
   Neither document references the other, and no requirement in the corpus covers the shared store.

8. **`project/current-exports.txt` — the stale path is now written into four more requirements.**
   Run 3 found five references (two scripts, three `ci.yml` lines); run 4 added a sixth (M8 Epic 7
   FR-10). Run 5 adds **Milestone 12 Epics 1 §7, 5 §7, 6 `cross_refs` and 7 FR-4.6**, all naming
   `project/current-exports.txt`. `.project/current-exports.txt` exists (442 KB);
   `project/current-exports.txt` does not. `check-api-surface.sh` exits 1 with "No baseline found"
   when the file is absent, so the `api-surface` CI job fails on every run. **Nine references,
   unchanged across three ingest runs.** Extends `DEBT-01`.

   **Correction (dated 2026-08-10, D-09):** the consequence clause above — "so the `api-surface`
   CI job fails on every run" — is **no longer true**. `scripts/check-api-surface.sh:6` reads
   `BASELINE="${1:-.project/current-exports.txt}"` — the dotted path, not the undotted
   `project/current-exports.txt` this finding names — and `ci.yml:187` invokes it as
   `./scripts/check-api-surface.sh .project/current-exports.txt`, explicitly passing the dotted
   path. Both checks re-run this session: `ls -la .project/current-exports.txt` -> present, 446,377
   bytes; `ls project/current-exports.txt` -> "No such file or directory". The job reads a baseline
   that exists; it does not fail on this ground.

   What remains true, precisely: the **documentation half**. The four Milestone 12 requirement
   texts named above (Epic 1 §7, Epic 5 §7, Epic 6 `cross_refs`, Epic 7 FR-4.6) still name the
   undotted `project/current-exports.txt` path in their own prose — a stale citation, not a broken
   automated guard. Phase 8's dated `DEBT-01` banners already annotate these four at source
   (`.project/Milestone_12-Web-API/...`). This does not become a sixth ORCH-03 item; it is handed
   to **Phase 15** alongside `DEBT-01`'s tooling half.

### Checkbox counts in run 5 — three of four are contradicted or vacuous

`task-completion-state.md` records Milestone 9 at 100.0% (0 open), Milestone 10 at 100.0% (0 open),
Milestone 11 at 92.0% (26 open) and Milestone 12 at 99.0% (3 open), plus project-management at 0.0%
(1 open).

- **Milestone 9's 0 open: corroborated.** Every Epic 1-5 deliverable is present in the tree.
- **Milestone 10's 0 open: corroborated in artefacts, contradicted in one acceptance criterion.**
  Every file, job, target and ruleset exists — but Epic 2's own success metric is false (finding 1).
  A 100% checkbox count that is simultaneously accurate about deliverables and wrong about
  acceptance is a new failure mode for this corpus.
- **Milestone 11's 26 open: plausible, and the only genuinely open count in run 5.** The items are
  `tasks-content-rewrite.md` task 6.0 (six user-guide in-place updates), task 7.0 (eight
  deployment/operations updates) and task 1.2 (review the linkcheck report). **All fourteen target
  files exist** under `docs/src/user-guides/`, `docs/src/deployment/` and `docs/src/operations/`.
  Whether their *content* is current cannot be settled by file existence, and mtimes are too weak an
  inference to record. Re-verify by content, not by presence.
- **Milestone 12's 3 open: contradicted.** All three are Task 0.0 scaffolding — "Create feature
  branch", "Update `main` … and create/checkout `feature/m12-epic5-api-security-authorization`",
  "Confirm a clean baseline". **The Epic 5 work itself shipped** (`agent_auth.rs`, finding 7). Zero
  real work is represented by this count.
- **project-management's 1 open: vacuous.** The item is
  `- [ ] 1.1 Create template → - [x] 1.1 Create template (after completing)` — a formatting example
  inside a template file, not a task.

### Final corpus position on open-checkbox counts

Across five runs the pattern is now complete and consistent: **checkbox arithmetic is not a backlog.**
Runs 1-2 found counts *understating* shipped reality (Conclave 129 open and shipped; Sanctum 111 open
and shipped). Run 3 found the first *accurate* count (Milestone 4's 20, corroborated by zero
`#[deprecated]` annotations) **and** the first *overstating* completion (CLI isolation fully checked
with three dependencies still unconditional). Run 4 found Milestone 8's three contradicted outright.
Run 5 finds Milestone 12's three vacuous and project-management's one nonexistent.

**Of the 542 open checkboxes recorded across 75 task lists, the verified genuine remainder is
Milestone 11's documentation-currency work plus Milestone 4's deprecation items.** Everything else
requires individual verification, and the trustworthy forward-work signal remains the three deferred
registers plus the verified defects in this file.
