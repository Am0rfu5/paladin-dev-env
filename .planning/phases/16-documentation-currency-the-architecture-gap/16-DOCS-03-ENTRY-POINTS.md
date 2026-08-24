# DOCS-03 — Public API Entry Point Enumeration (D-05)

This record enumerates, item by item, every "public API entry point" as D-05 binds the term, so
the definition in `16-CONTEXT.md` cannot drift or be silently re-scoped by a later phase. It is the
input the `# Examples` gate (plan 16-08 Task 2, `scripts/check-public-api-examples.sh`) derives its
own set from — by the same rule, with the same commands.

## Selection rule

A **public API entry point** is a `pub` item declared in `crates/*/src/**` or `src/**` matching one
of exactly three shapes:

1. A **builder** — `pub struct *Builder` (a struct whose name ends in `Builder`).
2. A **`*Port` trait** — `pub trait *Port` (a trait whose name ends in `Port`).
3. A **`*Service` struct** — `pub struct *Service` (a struct whose name ends in `Service`).

**Exclusions**, applied in order:

- Anything declared inside a `#[cfg(test)]` region (a mock, test-only helper, or test-gated
  re-export). Verified per-item below by checking whether the nearest enclosing `#[cfg(test)]`
  marker in the file precedes or follows the declaration line; none of the 77 raw matches this run
  found fall inside one — see "Raw match counts" below.
- Anything **not actually exported** — concretely, an item declared inside a workspace member crate
  that carries `publish = false` and is not part of the product's shipped, distributable surface.
  One item is excluded on this basis this run: `MockListService`
  (`crates/doc-examples/src/support.rs:273`), a mock declared in `paladin-doc-examples`, whose own
  `Cargo.toml` sets `publish = false` and describes itself as "Compile-verified documentation
  examples for the Paladin book (**not published**)". It exists to compile-check mdBook-embedded
  examples (`scripts/check-doc-examples.sh` Layer 1), not to be consumed as part of the framework's
  API. No downstream crate can ever depend on it — it is never published.

**Rejected readings** (per D-05, recorded so the boundary cannot be re-opened by accident):

- **All 1,971 items in `.project/current-exports.txt`.** That file is `cargo-public-api`'s
  simplified dump of the root `paladin` facade crate only (confirmed from its own header: "tracks
  all publicly exported items from the paladin crate") — it does not cover the ten library crates
  under `crates/*` directly, is dominated by `impl` lines and re-exports for which "has an example"
  is meaningless, and was explicitly rejected as the entry-point set by D-05.
- **The "79-plus-204-`pub fn new`" reading.** Widening the definition to include every associated
  `new` constructor was considered and rejected — it would roughly quadruple the surface without
  changing what FR-26.3's own wording names (builders, `*Port` traits, `*Service` structs).

## Derivation commands (re-runnable, D-00e)

Run from the workspace root. Each of the three kinds is a separate, independently re-runnable
command; the file-level `#[cfg(test)]` check is a fourth command applied to each match.

```bash
# 1. Builders
grep -rnE '^\s*pub struct [A-Za-z0-9_]*Builder\b' crates/*/src src/ --include='*.rs'

# 2. *Port traits
grep -rnE '^\s*pub trait [A-Za-z0-9_]*Port\b' crates/*/src src/ --include='*.rs'

# 3. *Service structs
grep -rnE '^\s*pub struct [A-Za-z0-9_]*Service\b' crates/*/src src/ --include='*.rs'

# 4. Per matched file, confirm the declaration line precedes any #[cfg(test)] marker
#    (a match found *after* the file's #[cfg(test)] mod boundary would be excluded):
grep -n '#\[cfg(test)\]' <matched-file>
```

**Raw match counts** (before the "not actually exported" exclusion):

| Kind | Raw grep count | D-05's stated count |
|------|-----------------|----------------------|
| `*Builder` | 11 | 11 |
| `*Port` trait | 35 | 35 |
| `*Service` struct | 31 | 33 |
| **Total** | **77** | **79** |

All 77 raw matches were individually checked against the nearest `#[cfg(test)]` marker in their
file (command 4 above) and confirmed to precede it — none is test-scoped. One of the 31 raw
`*Service` matches (`MockListService`) is excluded under the "not actually exported" rule, leaving
**76 legitimate entry points**: 11 Builders + 35 `*Port` traits + **30** `*Service` structs.

## Delta against D-05's 11 / 35 / 33

- **Builders: 11 vs 11 — exact match.** No delta.
- **`*Port` traits: 35 vs 35 — exact match.** No delta.
- **`*Service` structs: 30 (legitimate) vs 33 — a 3-item shortfall.**
  - **1 item accounted for:** `MockListService` (`crates/doc-examples/src/support.rs:273`) — the
    "a non-exported service" case D-05's own text anticipates. `paladin-doc-examples` is
    `publish = false`; the item is a compile-checked test double, not a shipped entry point.
  - **2 items unaccounted for.** An exhaustive re-run of the `*Service` grep (command 3 above)
    across all eleven crate directories under `crates/*/src` plus the facade `src/` returns exactly
    31 raw matches — no additional `pub struct *Service` declaration exists anywhere in the two
    path globs this rule covers. No specific excluded item explains the remaining 2. The most
    plausible cause, stated rather than guessed away: `16-CONTEXT.md`'s own "Measured baseline"
    note already records **77 resolvable entry-point files** against FR-26.3's 79 — a 2-item gap
    identical in size to this run's unaccounted residue, recorded *before* this plan's fresh,
    command-backed re-derivation existed. That prior note gives no `file:line` for its 2 dropped
    items either. The simplest honest account: FR-26.3's original "33" figure was carried into
    D-05 without an independent grep-based re-derivation at the time D-05 was written, and this
    enumeration — with its printed, re-runnable commands — is the first one. The definition itself
    is **not** adjusted to fit 33; the table below lists the 30 that the rule, applied literally
    and exhaustively, actually produces.

**Total entry points this record enumerates: 76** (11 + 35 + 30), against FR-26.3's stated 79 — a
net 3-item delta, 1 explained (`MockListService`), 2 recorded as unattributable to any specific
excluded item after an exhaustive re-run.

## Enumeration

One row per entry point. "Has example block today" is satisfied by an `# Example`/`# Examples`
heading (either spelling) in the item's own preceding `///` doc block, **or** in the file's leading
`//!` module doc — matching where the existing 38 actually put them.

| Kind | Item | file:line | Has example block today | Heading spelling |
|------|------|-----------|--------------------------|-------------------|
| Builder | `CommanderBuilder` | `crates/paladin-battalion/src/commander.rs:1328` | Yes — own doc block | plural (`# Examples`) |
| Builder | `CouncilBuilder` | `crates/paladin-core/src/platform/container/battalion/council.rs:288` | Yes — own doc block | singular (`# Example`) |
| Builder | `GroveBuilder` | `crates/paladin-core/src/platform/container/battalion/grove.rs:311` | Yes — own doc block | singular (`# Example`) |
| Builder | `StreamChunkBuilder` | `crates/paladin-core/src/platform/container/herald.rs:292` | Yes — file module doc (`//!`) | plural (`# Examples`) |
| Builder | `ExecutionMetadataBuilder` | `crates/paladin-core/src/platform/container/herald.rs:557` | Yes — file module doc (`//!`) | plural (`# Examples`) |
| Builder | `LogEntryBuilder` | `crates/paladin-core/src/platform/container/log.rs:195` | No | — |
| Builder | `PaladinConfigBuilder` | `crates/paladin-core/src/platform/container/paladin_config.rs:100` | Yes — own doc block | singular (`# Example`) |
| Builder | `MemoryBuilder` | `crates/paladin-core/src/platform/container/sanctum.rs:119` | Yes — own doc block | plural (`# Examples`) |
| Builder | `ProgressBarBuilder` | `src/application/cli/formatters/progress.rs:59` | No | — |
| Builder | `PromptBuilder` | `src/application/cli/interactive/prompts.rs:7` | No | — |
| Builder | `PaladinBuilder` | `src/application/services/paladin/paladin_builder.rs:77` | Yes — own doc block | singular (`# Example`) |
| Port | `ContentIngestionPort` | `crates/paladin-ports/src/input/content_input_port.rs:10` | No | — |
| Port | `DocumentPort` | `crates/paladin-ports/src/input/document_port.rs:114` | No | — |
| Port | `MlPort` | `crates/paladin-ports/src/input/ml_port.rs:97` | No | — |
| Port | `ArsenalPort` | `crates/paladin-ports/src/output/arsenal_port.rs:470` | Yes — own doc block | plural (`# Examples`) |
| Port | `AuthPort` | `crates/paladin-ports/src/output/auth_port.rs:57` | No | — |
| Port | `BattalionPort` | `crates/paladin-ports/src/output/battalion_port.rs:622` | Yes — own doc block | plural (`# Examples`) |
| Port | `CitadelPort` | `crates/paladin-ports/src/output/citadel_port.rs:567` | Yes — own doc block | plural (`# Examples`) |
| Port | `EmbeddingPort` | `crates/paladin-ports/src/output/embedding_port.rs:371` | Yes — own doc block | plural (`# Examples`) |
| Port | `FileStoragePort` | `crates/paladin-ports/src/output/file_storage_port.rs:980` | Yes — own doc block | plural (`# Examples`) |
| Port | `BatchFileStoragePort` | `crates/paladin-ports/src/output/file_storage_port.rs:1241` | No | — |
| Port | `AdvancedFileStoragePort` | `crates/paladin-ports/src/output/file_storage_port.rs:1264` | No | — |
| Port | `FileVersioningPort` | `crates/paladin-ports/src/output/file_storage_port.rs:1309` | No | — |
| Port | `FullFileStoragePort` | `crates/paladin-ports/src/output/file_storage_port.rs:1341` | No | — |
| Port | `GarrisonPort` | `crates/paladin-ports/src/output/garrison_port.rs:380` | Yes — own doc block | plural (`# Examples`) |
| Port | `LongTermGarrisonPort` | `crates/paladin-ports/src/output/garrison_port.rs:656` | Yes — own doc block | plural (`# Examples`) |
| Port | `LlmPort` | `crates/paladin-ports/src/output/llm_port.rs:962` | Yes — own doc block | plural (`# Examples`) |
| Port | `LogPort` | `crates/paladin-ports/src/output/log_port.rs:219` | No | — |
| Port | `NotificationDeliveryPort` | `crates/paladin-ports/src/output/notification_port.rs:796` | Yes — own doc block | plural (`# Examples`) |
| Port | `NotificationTemplatePort` | `crates/paladin-ports/src/output/notification_port.rs:1120` | Yes — own doc block | plural (`# Examples`) |
| Port | `BasicNotificationPort` | `crates/paladin-ports/src/output/notification_port.rs:1266` | Yes — own doc block | plural (`# Examples`) |
| Port | `OrchestratorPort` | `crates/paladin-ports/src/output/orchestrator_port.rs:232` | No | — |
| Port | `PaladinExecutorPort` | `crates/paladin-ports/src/output/paladin_executor_port.rs:60` | Yes — own doc block | singular (`# Example`) |
| Port | `PaladinPort` | `crates/paladin-ports/src/output/paladin_port.rs:631` | Yes — own doc block | plural (`# Examples`) |
| Port | `QueuePort` | `crates/paladin-ports/src/output/queue_port.rs:549` | Yes — own doc block | plural (`# Examples`) |
| Port | `TypedQueuePort` | `crates/paladin-ports/src/output/queue_port.rs:617` | No | — |
| Port | `BatchQueuePort` | `crates/paladin-ports/src/output/queue_port.rs:641` | No | — |
| Port | `PriorityQueuePort` | `crates/paladin-ports/src/output/queue_port.rs:678` | No | — |
| Port | `QueueManagementPort` | `crates/paladin-ports/src/output/queue_port.rs:705` | No | — |
| Port | `FullQueuePort` | `crates/paladin-ports/src/output/queue_port.rs:744` | No | — |
| Port | `SanctumPort` | `crates/paladin-ports/src/output/sanctum_port.rs:585` | Yes — own doc block | plural (`# Examples`) |
| Port | `SchedulerPort` | `crates/paladin-ports/src/output/scheduler_port.rs:237` | No | — |
| Port | `StreamingExecutorPort` | `crates/paladin-ports/src/output/streaming_executor_port.rs:66` | Yes — own doc block | singular (`# Example`) |
| Port | `UserRepositoryPort` | `crates/paladin-ports/src/output/user_repository_port.rs:12` | No | — |
| Port | `VisionPort` | `crates/paladin-ports/src/output/vision_port.rs:63` | No | — |
| Port | `WorkflowRepositoryPort` | `crates/paladin-ports/src/output/workflow_repository_port.rs:109` | No | — |
| Service | `CampaignExecutionService` | `crates/paladin-battalion/src/campaign_service.rs:58` | Yes — own doc block | singular (`# Example`) |
| Service | `ChainOfCommandExecutionService` | `crates/paladin-battalion/src/chain_of_command_service.rs:65` | Yes — file module doc (`//!`) | plural (`# Examples`) |
| Service | `ConclaveExecutionService` | `crates/paladin-battalion/src/conclave_execution_service.rs:38` | Yes — own doc block | singular (`# Example`) |
| Service | `CouncilExecutionService` | `crates/paladin-battalion/src/council_service.rs:57` | Yes — own doc block | singular (`# Example`) |
| Service | `FormationExecutionService` | `crates/paladin-battalion/src/formation_service.rs:37` | Yes — own doc block | singular (`# Example`) |
| Service | `GroveExecutionService` | `crates/paladin-battalion/src/grove_service.rs:103` | Yes — own doc block | singular (`# Example`) |
| Service | `ManeuverExecutionService` | `crates/paladin-battalion/src/maneuver/service.rs:15` | No | — |
| Service | `PhalanxExecutionService` | `crates/paladin-battalion/src/phalanx_service.rs:41` | Yes — own doc block | singular (`# Example`) |
| Service | `CollectionVersionService` | `crates/paladin-core/src/base/service/collection_versioning_service.rs:100` | No | — |
| Service | `FieldVersionService` | `crates/paladin-core/src/base/service/field_version_service.rs:93` | No | — |
| Service | `MessageService` | `crates/paladin-core/src/base/service/message_service.rs:163` | No | — |
| Service | `NodeVersionService` | `crates/paladin-core/src/base/service/node_version_service.rs:94` | No | — |
| Service | `DataBackupService` | `crates/paladin-core/src/platform/container/task.rs:333` | No | — |
| Service | `ContentIndexingService` | `crates/paladin-core/src/platform/container/task.rs:399` | No | — |
| Service | `EmailNotificationService` | `crates/paladin-core/src/platform/container/task.rs:535` | No | — |
| Service | `LlmAnalysisService` | `crates/paladin-llm/src/llm_analysis_service.rs:54` | No | — |
| Service | `MemoryExtractionService` | `crates/paladin-memory/src/services/memory_extraction_service.rs:41` | No | — |
| Service | `RagRetrievalService` | `crates/paladin-memory/src/services/rag_retrieval_service.rs:28` | No | — |
| Service | `ArsenalExecutionService` | `src/application/services/arsenal/arsenal_execution_service.rs:60` | Yes — own doc block | singular (`# Example`) |
| Service | `ArsenalRegistryService` | `src/application/services/arsenal/arsenal_registry_service.rs:42` | Yes — own doc block | singular (`# Example`) |
| Service | `DefaultContentIngestionService` | `src/application/services/content/content_ingestion_service.rs:240` | No | — |
| Service | `HandoffService` | `src/application/services/paladin/handoff_service.rs:42` | Yes — own doc block | singular (`# Example`) |
| Service | `PaladinExecutionService` | `src/application/services/paladin/paladin_execution_service.rs:105` | Yes — file module doc (`//!`) | singular (`# Example`) |
| Service | `PlanningService` | `src/application/services/paladin/planning_service.rs:45` | Yes — file module doc (`//!`) | plural (`# Examples`) |
| Service | `PromptGenerationService` | `src/application/services/paladin/prompt_generation_service.rs:48` | Yes — file module doc (`//!`) | plural (`# Examples`) |
| Service | `TemperatureService` | `src/application/services/paladin/temperature_service.rs:51` | No | — |
| Service | `ContentItemService` | `src/core/platform/manager/content_service.rs:20` | No | — |
| Service | `EventService` | `src/core/platform/manager/event_manager.rs:69` | No | — |
| Service | `UserService` | `src/core/platform/manager/user_service.rs:29` | No | — |
| Service | `EncryptionService` | `src/infrastructure/security/encryption.rs:161` | Yes — file module doc (`//!`) | singular (`# Example`) |

## Crate coverage note

The five crates D-05's measured baseline names as having zero examples anywhere — `paladin-llm`,
`paladin-storage`, `paladin-web`, `paladin-content`, `paladin-notifications` — resolve against this
enumeration as follows: `paladin-llm` contributes exactly one entry point (`LlmAnalysisService`,
no example); `paladin-storage`, `paladin-web`, `paladin-content` and `paladin-notifications`
contribute **zero** entry points under this rule at all — they hold adapter implementations of
`*Port` traits declared elsewhere (in `paladin-ports`), not their own `*Builder`/`*Port`/`*Service`
declarations. This is a structural fact about where types are declared versus implemented, not a
gap in this derivation.

## Closing totals

- **Entry points enumerated: 76** (11 Builders + 35 `*Port` traits + 30 `*Service` structs).
- **Already carry an example block: 38** (of 76).
- **Do not carry an example block: 38** (of 76) — the figure plans 16-09 through 16-12 close.
- **Use the singular heading spelling (`# Example`) today and need normalising under D-06: 17**
  (of the 38 that have an example at all) — 4 Builders, 2 Ports, 11 Services. Listed above; every
  one is a candidate for plan 16-12's normalisation sweep, scoped to exactly these 17 sites and no
  others.

## Closing totals — end of phase (after plan 16-12, D-05/D-06 fully closed)

The figures above are D-05's baseline, measured at the start of the `# Examples` wave (plan
16-08). This section records the end state after plans 16-09 through 16-12 closed both halves of
the gate — kept below the opening figures, not overwriting them (D-00d's amend-in-place ethic
applied to a phase record).

- **Entry points enumerated: 76** (11 Builders + 35 `*Port` traits + 30 `*Service` structs) —
  unchanged; no entry point was added or removed by any plan in this wave.
- **Carry an example block: 76 of 76 (100%).** Every MISSING row from the 16-08 baseline (38 of
  76) was closed across plans 16-09 (19), 16-10 (11) and 16-11 (8, the last 8 tree-wide).
- **Heading spelling — plural `# Examples`: 76 of 76 (100%).** Every SINGULAR row from the
  post-16-08 baseline was closed across plans 16-09 (2 heading fixes, in `paladin-ports`), 16-10
  (9 heading fixes, in `paladin-core`/`paladin-memory`/`paladin-battalion`/`paladin-herald`) and
  16-12 (the last 6, in `src/`: `PaladinBuilder`, `ArsenalRegistryService`,
  `ArsenalExecutionService`, `HandoffService`, `PaladinExecutionService`, `EncryptionService`).
  `bash scripts/check-public-api-examples.sh --list` closing run: `TOTAL: 76 entry points -- 76
  OK, 0 MISSING, 0 SINGULAR`.
- **Compile-and-run vs. `no_run` split, aggregated across the examples plans 16-09 → 16-11 (the
  plans that authored new example content; 16-12 only touched headings, adding zero new
  examples):**
  - 16-09: 19/19 new examples compile-and-run (0 `no_run`/`ignore`/`text` fences introduced).
  - 16-10: 11/11 new examples compile-and-run (0 non-running fences introduced).
  - 16-11: 8/8 new examples compile-and-run (0 non-running fences introduced).
  - **Total new examples across the wave: 38/38 compile-and-run, 0 non-running.** The
    pre-existing 87-fence `no_run`/`ignore`/`text` count this phase inherited (ADR-0033 Finding 3)
    is untouched — none of those pre-existing fences were audited, converted, or counted as part
    of this wave's closure, per this phase's explicit prohibition on touching
    `paladin-ports`'s 94 pre-existing ignored doctests.
- **`cargo test --workspace --doc` at phase close: 318 passed, 0 failed, 205 ignored**, across the
  12 doctest-bearing crates (`paladin`, `paladin_core`, `paladin_battalion`, `paladin_content`,
  `paladin_doc_examples`, `paladin_herald`, `paladin_llm`, `paladin_memory`,
  `paladin_notifications`, `paladin_ports`, `paladin_storage`, `paladin_web`). Verbatim per-crate
  breakdown recorded in `16-DOCS-03-GATE-EVIDENCE.md`'s Closing section.
- **`cargo doc --workspace --no-deps`: 0 warnings** at phase close, matching the bar 16-07
  established and every subsequent plan held.
- **The 76-vs-79 delta (§"Delta against D-05's 11 / 35 / 33" above) is left unresolved by this
  plan, as instructed.** Closing DOCS-03 did not surface any new evidence attributing the 2
  remaining unaccounted `*Service` items; the delta stands recorded, not silently closed by
  adjusting the expected total to match the observed one.
