# Constraints (from SPEC-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP` (36 docs).

**No SPEC-typed documents were present in this ingest run.**

Classification breakdown for this run: 11 PRD, 25 DOC, 0 ADR, 0 SPEC.

No constraint entries are recorded. Note that api-contract-shaped and
schema-shaped material (port trait signatures, SQLite DDL, YAML config
schemas, JSON state schemas, CLI command grammar) does exist in the source
set, but every such document was manifest-typed as PRD or DOC — not SPEC.
That material is therefore captured as requirement acceptance criteria in
`requirements.md` or as context in `context.md`, with its original source
path preserved.

Classifier notes on 9 of the `epic*.md` DOCs explicitly recorded SPEC-like
content signals (Rust type/trait contracts) that were overridden by
`MANIFEST_TYPE=DOC`. If SPEC-level precedence is wanted for that material,
re-tag those docs via `--manifest` and re-run ingest.

---

## Ingest run 2 of 5 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs)

**No SPEC-typed documents were present in this ingest run either.**

Classification breakdown for run 2: 15 PRD, 30 DOC, 0 ADR, 0 SPEC.

No constraint entries are recorded. As in run 1, constraint-shaped material does exist
in the source set but every carrier document was manifest-typed PRD or DOC, so the
material lives in `requirements.md` acceptance criteria or `context.md` instead, with
its original source path preserved. In run 2 that material includes:

- **api-contract shaped:** `EmbeddingPort`, `SanctumPort`, `DocumentPort`, `VisionPort`,
  `VisionCapableLlm`, `PaladinRegistry`, `SchedulerPort` and `ArsenalPort` trait
  signatures; the Grove LLM routing JSON contract
  (`{"tree_name","agent_id","confidence","reasoning"}`); the `handoff_to_agent` /
  handoff tool JSON schema; the OpenAI and Anthropic vision request/response shapes
  including Anthropic's `{type:"image",source:{type,media_type,data}}` content block.
- **schema shaped:** the Qdrant collection schema (1536-dim vectors, Cosine distance,
  indexed `paladin_id`/`memory_type`/`created_at`/`importance`); the Commander metadata
  export JSON document; the CLI YAML schemas for garrison, arsenal/MCP, Conclave,
  Council, Grove, Maneuver and autonomous features; the `paladin features --format json`
  output shape.
- **nfr shaped:** Sanctum search latency (<500ms at 100K vectors on Qdrant, <100ms at
  10K in-memory), RAG retrieval <500ms p95 and extraction <3s p95, Grove routing <3s,
  Maneuver parse <1ms and orchestration overhead <10ms, vision single-image <5s, PDF
  extraction <2s small / <10s large, registry lookup <1ms, metadata export <50ms,
  Phalanx metrics overhead <1%, and the several competing test-coverage gates.
- **protocol shaped:** MCP STDIO and SSE transport configuration; SSE streaming chunk
  handling for LLM providers.

If any of this should bind at SPEC precedence, re-tag the carrier documents via
`--manifest` and re-run ingest.

---

## Ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements` (32 docs)

**No SPEC-typed documents were present in this ingest run either.**

Classification breakdown for run 3: 13 PRD, 19 DOC, 0 ADR, 0 SPEC. Cumulative across runs 1-3:
113 documents, 39 PRD, 74 DOC, **0 SPEC**.

No constraint entries are recorded. As in runs 1 and 2, constraint-shaped material exists in
abundance in this source set, but every carrier document was manifest-typed PRD or DOC, so the
material lives in `requirements.md` acceptance criteria or `context.md` instead, with its original
source path preserved.

Run 3 is the most constraint-dense set so far, because these three milestones are almost entirely
about build-system contracts, dependency layering and module boundaries rather than features. The
inventory of what would become constraints if the carrier docs were re-tagged SPEC:

- **api-contract shaped:** the complete `paladin-ports` extraction inventory — 19 output port
  files with their primary exported types (`LlmPort` with `LlmRequest`/`LlmResponse`/`LlmError`/
  `TokenUsage`/`FinishReason`/`StreamingResponse`/`ToolCall`/`ToolResult`; `GarrisonPort` with
  `LongTermGarrisonPort`/`GarrisonError`/`GarrisonStats`; `SanctumPort` with `SanctumError`/
  `SanctumQuery`/`SanctumFilter`/`SanctumSearchResult`; `EmbeddingPort` with `Embedding`/
  `EmbeddingError`; `PaladinPort` with `PaladinResult`/`StopReason`; `PaladinRegistry` with
  `RegistryError`; plus `ArsenalPort`/`ArsenalRegistry`, `CitadelPort`, `QueuePort`,
  `NotificationPort`, `FileStoragePort`, `PaladinExecutorPort`, `BattalionPort`, `LogPort`,
  `SchedulerPort`, `SearchPort`, `ContentDeliveryPort`, `VisionCapableLlm`, `VisionPort`) and 6
  input port files (`ContentIngestionPort`, `DocumentPort`, `ListenerPort`, `MlPort`, `NlpPort`,
  `RpcGatewayPort`); the `EnvOverridable` trait signature and `read_env<T: FromStr>(&str) ->
  Option<T>` helper; the `LlmProviderError` variant list and its `From<LlmProviderError> for
  LlmError` boundary conversion; the `TokenCounter` trait plus `TokenCounterFactory`; the exact
  `paladin::prelude` type list (~30 names); the `Settings` public API (`new()`,
  `load_from_file()`, `get_queue_config()`, `get_file_storage_config()`,
  `get_notification_config()`, `get_garrison_config()`, `get_sanctum_config()`); the
  `OpenAIConfig`/`AnthropicConfig`/`DeepSeekConfig` field sets with their preserved `from_env()`
  constructors; the `CircuitBreaker`/`CircuitState` public surface; and the facade
  `pub mod container` re-export block including the exact `paladin_battalion::maneuver` type list.
- **schema shaped:** the workspace `Cargo.toml` template (`[workspace] members`, `resolver = "2"`,
  `[workspace.package]` version/edition/authors/license/repository, and the full
  `[workspace.dependencies]` version pin list); every per-crate `[features]` table
  (`paladin-llm`: `openai`/`anthropic`/`deepseek`/`mock`/`vision` with `default = ["openai",
  "mock"]` and `reqwest` optional; `paladin-memory`: `default = []`, `sqlite = ["dep:sqlx"]`,
  `qdrant = ["dep:qdrant-client"]`, `content-processing = ["dep:tiktoken-rs"]`); the root crate's
  full pre- and post-milestone `[features]` blocks; the exact dependency allowlists for
  `paladin-core` (6 crates, declared exhaustive), `paladin-ports` (7) and `paladin-battalion`
  (14 permitted, 9 forbidden); the `sqlx` workspace feature set
  (`runtime-tokio-rustls`, `sqlite`, `chrono`, `uuid`, `json`, `mysql` excluded) and
  `qdrant-client = "1.14"`; the `config.yml` deserialization contract with all `#[serde(default)]`,
  `#[serde(rename)]` and `#[serde(skip_serializing_if)]` attributes required to survive the move
  byte-for-byte; the `crate-isolation` CI matrix definition; and the `[[bin]]` target definitions
  with their `required-features`.
- **nfr shaped:** `cargo build -p paladin-core` under 30 seconds; `cargo build -p paladin-llm
  --no-default-features` under 5 seconds; >= 50% incremental rebuild improvement (Milestone 5
  target, restated as SM-7 in Epic 6 and as the >= 50% goal for an OpenAI-only change in Epic 4);
  >= 30% reduction in affected-module recompilation (Milestone 4); a 30%+ faster incremental build
  for agents-only use cases and `cargo build` under 2 minutes versus ~3+ minutes (M4 Epic 1);
  public API surface <= 50 exported types against a ~200+ baseline (M4 Epic 2 PRD) versus 104-124
  types / 40-50% reduction (the api-audit DOC); 100% rustdoc coverage and 0 broken intra-doc links;
  no file over 400 lines in any `config/` module; no file over 600 lines added by M6 Epic 2; no
  file over 1,000 lines added or modified by M6 Epic 3; the ~60-dependency baseline dropping to
  ~40 for orchestration-only consumers; and every "zero warnings" gate
  (`cargo clippy --workspace -- -D warnings`, `cargo fmt --all --check`,
  `cargo doc --workspace --no-deps`).
- **protocol shaped:** the MCP STDIO and SSE transport adapters as the gating boundary for the
  proposed (and later eliminated) `mcp-stdio` / `mcp-sse` / `mcp-transports` flags; the
  `#[cfg(feature = "...")]` guard placement protocol (module declarations, `use` statements, port
  implementation registrations, adapter instantiations, test modules); the three-substitution
  import-migration protocol for the battalion extraction
  (`crate::application::ports::` -> `paladin_ports::`,
  `crate::application::use_cases::battalion::` -> `crate::`, `crate::core::` -> `paladin_core::`);
  and the safe-migration ordering protocol (originals are not deleted until the new crate compiles,
  the shim is in place and `cargo test --workspace` passes at or above baseline).
- **test-count and evidence artifacts** that read as contracts: 1,487+ tests (M4/M5/M6 overviews),
  2,610+ (M5 Epic 3), 193 CLI tests, 128+ config tests, 314 import occurrences across 76 files,
  169 import occurrences across 13 files, 12 inline test modules, 35 minimum inline Maneuver tests,
  57 parser tests, 21 maneuver domain tests, 113 Maneuver tests, 32 Maneuver benchmarks; plus the
  four committed evidence files (`paladin-ports-isolation-build.txt`,
  `paladin-ports-dependency-tree.txt`, `paladin-battalion-isolation-build.txt`,
  `paladin-battalion-dependency-tree.txt`) and `build-benchmarks.md`.

Nine of the 19 run-3 DOCs are verbatim extracts of two milestone-overview documents (Milestone 5
Epics 2-5 and Milestone 6 Epics 1-4), so re-tagging them SPEC would duplicate rather than add
constraint coverage. The materially distinct constraint carriers are `dependency-matrix.md`,
`api-audit.md`, `DEPRECATIONS.md`, `dependency-analysis.md`, `build-benchmarks.md`, the two
`decisions/` files, and the two milestone overviews themselves.

If any of this should bind at SPEC precedence — the dependency allowlists and the `config.yml`
deserialization contract are the strongest candidates, since both are already contradicted by
shipped code — re-tag the carrier documents via `--manifest` and re-run ingest.

---

## Ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs)

**No SPEC-typed documents were present in this ingest run either.**

Classification breakdown for run 4: 11 PRD, 29 DOC, 0 ADR, 0 SPEC. Cumulative across runs 1-4:
**153 documents, 0 SPEC.** No entry in this file is asserted at SPEC precedence.

No constraint entries are recorded for run 4. The material below is constraint-shaped — it reads as
a contract a future change could violate — but every carrier is a PRD or a DOC, so it lives in
`requirements.md` and `context.md` at those precedences.

### Constraint-shaped material found in run 4 (recorded elsewhere, listed here for coverage)

- **Crate dependency direction (architectural invariant).** The four extracted infrastructure
  crates may depend only on `paladin-ports`, `paladin-core` and workspace-shared dependencies;
  *"No extracted crate may depend on another extracted crate or on the `paladin` facade."*
  (M7 Epic 1 §6.1, `REQ-extracted-crate-dependency-rule`.) The shipped `paladin-content` breaks it
  via an optional `paladin-llm` dependency behind its `llm` feature. This is the single strongest
  SPEC candidate in the run — it is the kind of rule that should bind at a precedence a PRD cannot
  quietly amend.

- **Dependency-isolation assertions.** `cargo tree -p paladin-core --all-features` and
  `cargo tree -p paladin-battalion --all-features` must not contain `actix-web`, `axum`, `lettre`,
  `pdf-extract`, `scraper` or `sqlx` (M7 Epic 1 §8.7-8.8). Mechanically checkable; carried by a PRD.

- **Banned-crate policy.** `actix-web` under `[bans] deny` in `deny.toml`, enforced by `make deny`
  and the CI dependency-policy job, so *"a second web framework cannot silently return"*
  (M8 Epic 7 FR-8). This is a live guardrail with a shipped enforcement point.

- **RustSec exception list.** Two advisory IDs approved for ignore, with an owner
  (Platform Security) and an expiry (**2026-09-30**), enforced identically in `make audit` and the
  CI security job (M7 Epic 4 `rustsec-remediation-plan.md`). The tree now carries five vulnerability
  ignores across three files that disagree with each other — see `INGEST-CONFLICTS.md`. If any
  run-4 material should bind at SPEC precedence, this is the second candidate: an exception list
  with an expiry is exactly the kind of contract that should not be widened by a PRD or by an
  uncommented manifest edit.

- **License policy.** `MIT OR Apache-2.0` with a permissive-branch acceptance rule, MPL-2.0
  accepted for unmodified use, approver `DF3NDR`, approval date 2026-05-28, inventory of 551
  packages with zero unknown entries (M7 Epic 4 `license-compatibility-decision-checklist.md`).
  Contradicted by the `license (MIT)` position in the M7 overview and the shipped root
  `Cargo.toml`.

- **HTTP endpoint contracts.** The three revived delivery endpoints specify paths, methods,
  request/response types and exact status-code semantics including error-body shape
  (`{ "error": "<message>" }`), with 200/400/404/500 mapped per case (M8 Epic 7 FR-1.1 to 1.3).
  This is the only genuine **api-contract**-type material in the run; it is carried by a PRD.

- **Publishing order.** `paladin-core` → `paladin-ports` → leaf crates → `paladin` facade, with the
  warning *"Violating this order will cause `cargo publish --dry-run` to fail"* (M7 Epic 2 FR-26,
  M7 Epic 4 §4.5.6, M7 overview Appendix B). The shipped CI job replaces it with a single
  workspace-wide dry run and records a counter-rationale.

- **Stability tiers.** Every public type and trait must carry a tier of Stable, Unstable or
  Experimental, with cross-crate dependency contracts documented (M7 Epic 4 §4.6). A three-value
  enumeration over the whole public surface is contract-shaped.

- **Documentation coverage threshold.** `#![warn(missing_docs)]` on all public crates, zero
  `cargo doc --workspace --no-deps` warnings, and a per-crate coverage audit **exceeding 90%**
  (M7 Epic 4 §4.4).

- **Quality-gate command sets** appearing verbatim across nine of the eleven run-4 PRDs:
  `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
  `cargo fmt --all -- --check`, `cargo doc --workspace --no-deps`. M8 Epic 5 FR-19 is the only place
  that relaxes one — `cargo doc` must exit 0 but *"warnings acceptable; must not fail"* — which is
  a weaker bar than M7 Epic 4 §4.4.3's zero-warning requirement on the same command.

- **Counting assertions that read as contracts.** 189 facade `.rs` files audited; 151 stay, 13 move,
  25 delete; exactly 26 files removed leaving 163; `find src/ -name "*.rs" | wc -l` = **160** after
  the Epic 4 rename; 286 Rust references and 57 markdown references to `use_cases`; 275+ consumers
  of `crate::core::`; ~49 facade files importing via `crate::core::…`; five `lib.rs` `pub use`
  exceptions with 13 and 17 consumers; 6 storage-shim consumers; 5 `crate::use_cases` occurrences
  across 4 files; 13 files in `paladin-content/src/services/`; six `E0432` errors; 17 residual
  `println!` occurrences across 6 files; 11 justified `#[allow(dead_code)]` markers; 551 licensed
  packages; ~10,250 net LOC removed across 15 commits.

- **Build-baseline measurement protocol.** Three runs and report the median, for clean workspace
  build, five per-crate incremental builds, and cold- and warm-cache Docker builds; plus compressed
  image size for both Dockerfiles and a ≤10% image-size regression target (M7 Epic 2 FR-07).

- **Benchmark isolation rules.** Battalion benchmarks must use mock `PaladinPort` implementations;
  LLM benchmarks must measure serialization only and exclude live HTTP and provider latency;
  garrison benchmarks must run at exactly 100 / 1000 / 10000 entries; the critical-path set is
  closed at four categories (M7 Epic 3 FR-11 to FR-16).

If any of this should bind at SPEC precedence, the strongest carriers are
`prd-extract-infrastructure-crates.md` §6.1 (the crate dependency-direction invariant),
`rustsec-remediation-plan.md` (the exception list with its expiry), and
`prd-paladin-web-single-framework-axum.md` §4 (the endpoint contracts). Re-tag those documents via
`--manifest` and re-run ingest.
