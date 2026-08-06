# Milestone 4-6 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 4-6 as-shipped ledger` section (D-26).
That section becomes a pointer to this file. Phases 10 and 13 each add a sibling ledger
(`milestone-07-08.md`, `milestone-09-12.md`) rather than growing REQUIREMENTS.md further —
REQUIREMENTS.md is already ~4,000 lines and five inline sets of `file:line`-cited verdicts would
make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — nesting them keeps this ledger
joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing new IDs (D-00e). The same
`file:line` citation may legitimately appear in more than one row: two requirements describing the
same shipped artefact keep separate rows and separate verdicts, because the `REQ-*` ID is the
primary key, not the citation. Two rows are never merged because they cite the same artefact.

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-01). This bar applies to all 115
rows below **without exception**, including every row REQUIREMENTS.md's run-3 ledger already marked
with an ingest-era status word (`Shipped`, `Verify`, `Variant`, etc.) — an ingest status word **is**
the bare "the code exists" claim this bar exists to reject.

**Manifest carve-out.** Milestones 4-6 are structural milestones, so a large share of their
requirements *are* manifest declarations — `edition`, feature-flag shapes, dependency lists,
`required-features`, workspace membership. For those, the manifest line **plus** a named CI job or
build leg that consumes it is the exercising artefact. Two exercising artefacts anchor this ledger,
both re-grepped fresh against the tree on 2026-08-06 rather than trusted from an earlier document:
the `crate-isolation` job in `.github/workflows/ci.yml`, confirmed at **line 304**
(`intel/code-verification.md`'s citation of that job at line 228 is stale and must not be copied
forward),
and the workspace matrix build/test steps plus the `cli_isolation` regression step in
`.github/workflows/feature-flags.yml`, confirmed at **lines 115, 118 and 141**. A manifest fact
cited without its consuming CI job is `present, unproven`, not `satisfied` — a bare `Cargo.toml`
feature declaration is the exact false-positive class this bar exists to reject.

**Path caveats.** Read every row below with two systematic caveats recorded once here, not repeated
per row (D-04). (a) The `src/…` paths in the run-3 PRDs are *internally* historical — Milestone 6
moved what Milestone 5 had just placed, and `src/application/use_cases/` no longer exists at all.
Citations in this ledger are **current** locations, resolved through
`.planning/codebase/STRUCTURE.md` or the tree, never through a PRD. (b) `STABLE_API.md`,
`docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and `docs/CONFIGURATION.md` ship as mdbook chapters
under `docs/src/api-reference/` and `docs/src/getting-started/installation.md`, not at their PRD
paths. A row whose only divergence from its PRD is caveat (b) is `relocated`, not a gap.

**Workspace shape.** This ledger is authoritative, per this phase (D-05), that the workspace is
**ten library crates** — `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`,
`paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`,
`paladin-web` — plus a `doc-examples` crate plus the root facade package `paladin-ai`. This
replaces both the six crates the Milestone 5/6 overviews assume and the nine-crate figure this
planning set carried before run 3. The same correction lands in REQUIREMENTS.md's pointer (below)
and in `.planning/codebase/STRUCTURE.md`'s Directory Purposes section, corrected by this same plan.

**Per-milestone checkbox corroboration (D-06).** This corpus's dominant record-understates-the-tree
heuristic does not apply uniformly here and must be applied per milestone. Milestone 4 is 93.2%
complete with 20 open items, all in Epic 2, and those 20 are **corroborated** — real remaining work.
Milestone 5 is 96.4% complete with 17 open items, **mostly contradicted** by the tree — the crates,
the CI job and the benchmark report all ship despite the open checkboxes. Milestone 6 is 100%
complete with 0 open items, **corroborated** — all four relocations verifiably complete.

## Verdict legend

| Verdict | Meaning |
| --- | --- |
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |
| `relocated` | The deliverable exists, but at a different path than the requirement names |
| `diverged` | The shipped tree deliberately implements the requirement differently, as distinct from a later milestone replacing it |

**ROADMAP-criterion mapping.** ROADMAP criterion 1 names five verdict words — satisfied, relocated,
superseded, diverged, genuinely outstanding — that map directly onto five of this legend's seven
values (`superseded` → `superseded by shipped code`); the two additional values, `present, unproven`
and `deferred with reason`, are this ledger's own finer-grained vocabulary and are not named by
ROADMAP criterion 1.

**Tie-break rule.** A row that qualifies for both `relocated` and `superseded by shipped code` takes
`relocated`, because D-02 exists to preserve the moved-not-missing signal that `superseded` would
otherwise absorb.

## Row order and amendment convention

The 13 epic sections below appear in REQUIREMENTS.md's own run-3 order — Milestone 4 Epics 1-3,
Milestone 5 Epics 1-6, Milestone 6 Epics 1-4 — and are never re-sorted. Rows within a section appear
in the ID order REQUIREMENTS.md lists them. Later plans replace a row's **Verdict** and **Evidence**
cells in place; they never insert, delete, or reorder rows. Exactly one plan in this phase writes
this file per execution wave (waves 1, 4, 5, 6, 7, 8 and 9 each contain exactly one ledger-writing
plan; waves 2 and 3 contain none), so no two executors edit it concurrently. Amendments follow
D-00f: edit in place, retain superseded text, date every amendment, never a separate corrections
file.

### Milestone 4 Epic 1 — Feature Flag Expansion (7 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-feature-flag-matrix | PENDING-VERDICT | 07-12 |
| REQ-vision-feature-gating | PENDING-VERDICT | 07-12 |
| REQ-feature-default-set | PENDING-VERDICT | 07-12 |
| REQ-feature-full-flag | PENDING-VERDICT | 07-12 |
| REQ-cfg-guard-discipline | PENDING-VERDICT | 07-12 |
| REQ-feature-flag-docs | PENDING-VERDICT | 07-12 |
| REQ-feature-ci-matrix | PENDING-VERDICT | 07-12 |

### Milestone 4 Epic 2 — Port Trait Hardening & Stable API (9 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-curated-lib-exports | PENDING-VERDICT | 07-12 |
| REQ-visibility-hardening | PENDING-VERDICT | 07-12 |
| REQ-port-trait-rustdoc | PENDING-VERDICT | 07-12 |
| REQ-stable-api-doc | PENDING-VERDICT | 07-12 |
| REQ-import-path-updates-m4 | PENDING-VERDICT | 07-12 |
| REQ-doc-build-clean | PENDING-VERDICT | 07-12 |
| REQ-api-surface-ci | PENDING-VERDICT | 07-12 |
| REQ-deprecation-warnings | PENDING-VERDICT | 07-12 |
| REQ-api-surface-reduction-target | PENDING-VERDICT | 07-12 |

### Milestone 4 Epic 3 — CLI Isolation (9 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-cli-feature-gate | PENDING-VERDICT | 07-12 |
| REQ-cli-dependency-isolation | PENDING-VERDICT | 07-12 |
| REQ-binary-target-config | PENDING-VERDICT | 07-12 |
| REQ-cli-test-isolation | PENDING-VERDICT | 07-12 |
| REQ-library-only-build | PENDING-VERDICT | 07-12 |
| REQ-library-only-integration-tests | PENDING-VERDICT | 07-12 |
| REQ-cli-build-time-measurement | PENDING-VERDICT | 07-12 |
| REQ-cli-ci-matrix | PENDING-VERDICT | 07-12 |
| REQ-cli-docs | PENDING-VERDICT | 07-12 |

### Milestone 5 Epic 1 — Workspace Initialization & paladin-core (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-cargo-workspace-root | PENDING-VERDICT | 07-06 |
| REQ-workspace-crate-edition-v1 | PENDING-VERDICT | 07-06 |
| REQ-paladin-core-scaffold | PENDING-VERDICT | 07-06 |
| REQ-paladin-core-dependency-allowlist-v1 | PENDING-VERDICT | 07-06 |
| REQ-core-base-extraction | PENDING-VERDICT | 07-06 |
| REQ-core-container-extraction | PENDING-VERDICT | 07-06 |
| REQ-core-upward-dependency-resolution | PENDING-VERDICT | 07-06 |
| REQ-port-value-type-ownership-v1 | PENDING-VERDICT | 07-06 |
| REQ-facade-core-reexports | PENDING-VERDICT | 07-06 |
| REQ-core-dependency-validation | PENDING-VERDICT | 07-06 |

### Milestone 5 Epic 2 — paladin-ports Extraction (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-ports-scaffold | PENDING-VERDICT | 07-06 |
| REQ-output-ports-extraction | PENDING-VERDICT | 07-06 |
| REQ-input-ports-extraction | PENDING-VERDICT | 07-06 |
| REQ-ports-facade-wiring | PENDING-VERDICT | 07-06 |
| REQ-ports-import-migration | PENDING-VERDICT | 07-06 |
| REQ-ports-doctest-compilation | PENDING-VERDICT | 07-06 |
| REQ-ports-docs-markdown-update | PENDING-VERDICT | 07-06 |
| REQ-ports-layering-validation | PENDING-VERDICT | 07-06 |
| REQ-ports-tests-and-rustdoc | PENDING-VERDICT | 07-06 |
| REQ-port-value-type-ownership-v2 | PENDING-VERDICT | 07-06 |

### Milestone 5 Epic 3 — paladin-battalion Extraction (9 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-crate-scaffold | PENDING-VERDICT | 07-10 |
| REQ-battalion-service-extraction | PENDING-VERDICT | 07-10 |
| REQ-battalion-import-migration | PENDING-VERDICT | 07-10 |
| REQ-battalion-inline-tests | PENDING-VERDICT | 07-10 |
| REQ-battalion-facade-shim | PENDING-VERDICT | 07-10 |
| REQ-battalion-dependency-validation | PENDING-VERDICT | 07-10 |
| REQ-battalion-example-verification | PENDING-VERDICT | 07-10 |
| REQ-battalion-crate-docs | PENDING-VERDICT | 07-10 |
| REQ-paladin-core-dependency-allowlist-v2 | PENDING-VERDICT | 07-10 |

### Milestone 5 Epic 4 — paladin-llm Extraction (11 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-llm-crate-scaffold | PENDING-VERDICT | 07-10 |
| REQ-llm-provider-error | PENDING-VERDICT | 07-10 |
| REQ-openai-provider-extraction | PENDING-VERDICT | 07-10 |
| REQ-anthropic-provider-extraction | PENDING-VERDICT | 07-10 |
| REQ-deepseek-provider-extraction | PENDING-VERDICT | 07-10 |
| REQ-llm-mock-adapters | PENDING-VERDICT | 07-10 |
| REQ-llm-provider-factory | PENDING-VERDICT | 07-10 |
| REQ-llm-config-bridge-location-v1 | PENDING-VERDICT | 07-10 |
| REQ-llm-test-architecture | PENDING-VERDICT | 07-10 |
| REQ-llm-facade-prelude | PENDING-VERDICT | 07-10 |
| REQ-llm-build-validation | PENDING-VERDICT | 07-10 |

### Milestone 5 Epic 5 — paladin-memory Extraction (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-memory-crate-scaffold | PENDING-VERDICT | 07-11 |
| REQ-workspace-crate-edition-v2 | PENDING-VERDICT | 07-11 |
| REQ-memory-module-structure | PENDING-VERDICT | 07-11 |
| REQ-garrison-adapter-extraction | PENDING-VERDICT | 07-11 |
| REQ-sanctum-adapter-extraction | PENDING-VERDICT | 07-11 |
| REQ-memory-services-extraction | PENDING-VERDICT | 07-11 |
| REQ-memory-originals-deletion | PENDING-VERDICT | 07-11 |
| REQ-memory-facade-reexports | PENDING-VERDICT | 07-11 |
| REQ-memory-test-migration | PENDING-VERDICT | 07-11 |
| REQ-memory-build-gates | PENDING-VERDICT | 07-11 |

### Milestone 5 Epic 6 — Workspace Finalization (6 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-facade-reexport-audit | PENDING-VERDICT | 07-11 |
| REQ-paladin-prelude | PENDING-VERDICT | 07-11 |
| REQ-devcontainer-gh-cli | PENDING-VERDICT | 07-11 |
| REQ-crate-isolation-ci | PENDING-VERDICT | 07-11 |
| REQ-workspace-ci-upgrade | PENDING-VERDICT | 07-11 |
| REQ-build-benchmark-report | PENDING-VERDICT | 07-11 |

### Milestone 6 Epic 1 — application_settings.rs Decomposition (8 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-config-domain-modules | PENDING-VERDICT | 07-08 |
| REQ-env-overridable-trait | PENDING-VERDICT | 07-08 |
| REQ-settings-root-struct | PENDING-VERDICT | 07-08 |
| REQ-config-incremental-migration | PENDING-VERDICT | 07-08 |
| REQ-config-yml-backcompat | PENDING-VERDICT | 07-08 |
| REQ-rag-config-dedup | PENDING-VERDICT | 07-08 |
| REQ-config-success-metrics | PENDING-VERDICT | 07-08 |
| REQ-llm-config-bridge-location-v2 | PENDING-VERDICT | 07-08 |

### Milestone 6 Epic 2 — Orchestration Service Relocation (9 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-orchestration-target-structure | PENDING-VERDICT | 07-08 |
| REQ-six-service-relocation | PENDING-VERDICT | 07-08 |
| REQ-domain-type-placement-rules | PENDING-VERDICT | 07-08 |
| REQ-manager-services-retained | PENDING-VERDICT | 07-08 |
| REQ-orchestration-consumer-import-updates | PENDING-VERDICT | 07-08 |
| REQ-orchestrator-renaming | PENDING-VERDICT | 07-08 |
| REQ-core-isolation-verification | PENDING-VERDICT | 07-08 |
| REQ-orchestration-test-coverage | PENDING-VERDICT | 07-08 |
| REQ-orchestration-no-reexport-shims | PENDING-VERDICT | 07-08 |

### Milestone 6 Epic 3 — Maneuver DSL Co-location (9 IDs)

Epic-level note: this epic carries **0 open checkboxes, corroborated** (D-06) — all four
relocations, including this one, are verifiably complete. All 9 rows below were fully cited and
exercised during this task (2026-08-06): citations re-grepped fresh against the tree, and every
`satisfied` row backed by a scoped offline test run whose output is quoted or summarised inline.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-maneuver-submodule-structure | satisfied | `crates/paladin-battalion/src/maneuver/mod.rs:6-8` declares `pub mod parser; pub mod service; pub mod visualizer;`, with `parser/mod.rs:49` (`FlowParser`), `parser/{lexer,ast,error}.rs`, `service.rs:15` (`ManeuverExecutionService`), and `visualizer.rs:55` (`FlowVisualizer`) all present. Exercised by `cargo test --offline -p paladin-battalion maneuver`, run during this task: 74 passed, 0 failed, spanning `maneuver::parser::{ast,error,lexer}::tests`, `maneuver::parser::tests`, `maneuver::service::tests`, `maneuver::visualizer::tests` and `maneuver::tests` — every submodule the requirement names. |
| REQ-maneuver-files-moved-from-core | satisfied | `find crates/paladin-core/src -iname "*maneuver*" -o -iname "*parser*"` returns no results, and `grep -n "maneuver\|parser" crates/paladin-core/src/platform/container/battalion/mod.rs` returns only the `Maneuver` pattern-type enum variant at `battalion/mod.rs:447` (no lexer/parser/AST code) — both run during this task. The moved content lives at `crates/paladin-battalion/src/maneuver/` (cited in the row above) and is exercised by the same `cargo test --offline -p paladin-battalion maneuver` run (74 passed) plus the `crate-isolation` CI job (`ci.yml:304`), which builds `paladin-core` in isolation with no parser dependency to carry. **Supersedes** the Maneuver clause of `REQ-core-container-extraction`, per REQUIREMENTS.md's own note at that row. |
| REQ-maneuver-files-reorganized | satisfied | The flat `maneuver_service.rs` and `flow_visualizer.rs` are confirmed absent from `crates/paladin-battalion/src/` (directory listing, run during this task), replaced by `crates/paladin-battalion/src/maneuver/service.rs:15` and `maneuver/visualizer.rs:55`. Exercised by `cargo test --offline -p paladin-battalion maneuver`: the `maneuver::service::tests::*` (8 tests) and `maneuver::visualizer::tests::*` (17 tests) all pass, run during this task. |
| REQ-maneuver-inline-tests | satisfied | `grep -rn '#\[test\]' crates/paladin-battalion/src/maneuver/` returns **56** inline test-attribute occurrences (≥ the PRD's 35-test minimum), run during this task. Exercised by `cargo test --offline -p paladin-battalion maneuver` — 74 tests passed, 0 failed, run during this task (the pass count exceeds the raw attribute grep because some cases are table-driven; both figures clear the ≥ 35 bar). |
| REQ-core-maneuver-cleanup | satisfied | `crates/paladin-core/src/platform/container/battalion/mod.rs:447` retains only the `Maneuver` pattern-type enum variant (part of `BattalionStrategy`); `grep -n "maneuver\|parser" battalion/mod.rs` returns no lexer, parser, or AST reference, run during this task. Exercised by the `crate-isolation` CI job (`ci.yml:304`), which builds `paladin-core` standalone — a leftover parser reference or dependency would fail that isolated build. |
| REQ-maneuver-facade-reexports | satisfied | `src/core/platform/mod.rs:6` (`pub mod container { … }`) and `:53-64` (`pub mod battalion { … pub mod maneuver { pub use paladin_battalion::maneuver::*; pub mod parser { … } } pub mod parser { … } }`) — the explicit `container` block plus the `maneuver`/`parser` forwarding sub-modules restoring backward-compatible paths for the moved DSL. Exercised by `cargo test --offline --test unit maneuver` (`tests/unit/maneuver_domain_tests.rs`, imports `paladin::core::platform::container::battalion::maneuver::{…}` and `::parser::FlowParser` at lines 3 and 7) — 21 passed — and `cargo test --offline --test unit parser_tests` (`tests/unit/parser_tests.rs`, same import path) — 57 passed. Both run during this task. |
| REQ-maneuver-battalion-import-updates | satisfied | `crates/paladin-battalion/src/commander.rs:18` (`use crate::maneuver::service::ManeuverExecutionService;`) plus fully-qualified `crate::maneuver::*` references at `:172, 798, 815, 817, 819, 822, 825, 834, 873, 876, 879, 1334, 1467, 1486, 1492, 1568, 2648, 2658`. Exercised by `cargo test --offline -p paladin-battalion commander`, run during this task: 52 passed, 0 failed, including `test_maneuver_execution_through_commander`, `test_maneuver_strategy_explicit`, `test_maneuver_with_nested_pattern`, `test_maneuver_with_parallel_pattern`, `test_maneuver_without_flow_expression_fails`, `test_maneuver_with_invalid_flow_expression_fails`. |
| REQ-maneuver-battalion-lib-exports | satisfied | `crates/paladin-battalion/src/lib.rs:38` (`pub mod maneuver;`). Exercised directly by the two facade-path test files cited under `REQ-maneuver-facade-reexports` (`maneuver_domain_tests.rs` 21 passed, `parser_tests.rs` 57 passed, 78 total, run during this task) and indirectly by `src/core/platform/mod.rs:54` (`pub use paladin_battalion::maneuver::*;`), which compiles only because this crate-root export exists. |
| REQ-maneuver-cargo-dependency-check | satisfied | `crates/paladin-core/Cargo.toml:17-31` lists exactly 14 dependencies (`serde`, `serde_json`, `uuid`, `chrono`, `thiserror`, `async-trait`, `tokio`, `sha2`, `blake3`, `petgraph`, `murmur3`, `url`, `regex`, `futures`) — no parser/lexer crate among them (the Maneuver DSL parser is hand-written, so no dependency pruning was required when it moved to `paladin-battalion`). Manifest carve-out (D-01): the manifest line plus the `crate-isolation` CI job (`ci.yml:304`), which builds and isolates `paladin-core` against exactly this dependency list, is the exercising artefact. |

### Milestone 6 Epic 4 — CircuitBreaker Relocation to Infrastructure (8 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-resilience-module-structure | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-relocation | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-rustdoc-updates | PENDING-VERDICT | 07-08 |
| REQ-paladin-execution-service-import | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-example-updates | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-test-updates | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-old-path-retired | PENDING-VERDICT | 07-08 |
| REQ-circuitbreaker-stable-api-update | PENDING-VERDICT | 07-08 |
