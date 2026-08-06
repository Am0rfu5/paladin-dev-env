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
| REQ-cargo-workspace-root | satisfied | `Cargo.toml:1` (`[workspace]`) and `:2` (`members = [".", "crates/*"]`) — the repository root is a Cargo workspace with the existing `paladin-ai` package as a member alongside every `crates/*` entry, confirmed by direct read during this task. `:5-30` (`[workspace.dependencies]`) declares shared versions for every crate FR-2 names as a minimum: `serde` (`:6`, `features = ["derive"]`), `serde_json` (`:7`), `uuid` (`:8`, `["v4","serde"]`), `chrono` (`:9`, `["serde"]`), `thiserror` (`:10`), `tokio` (`:11`, `["full"]`), `async-trait` (`:12`), `reqwest` (`:13`, `["json"]`), `log` (`:14`). Manifest carve-out (D-01): exercised by the `test` job's `cargo test --workspace --lib --bins` (`ci.yml:223`, re-grepped this task) and the `crate-isolation` matrix (`ci.yml:304-330`), whose ten legs each run `cargo build -p ${{ matrix.crate }}` (`:348`) against this workspace manifest — both re-confirmed passing this task via `cargo check --offline --workspace --lib` (0 errors, `Finished` in 54.29s). |
| REQ-workspace-crate-edition-v1 | superseded by shipped code | Cites `.planning/decisions/0009-workspace-rust-edition-2024.md` (ADR-0009), which already records and applies the answer to ARCH-03(a) — Phase 7's scope for this pair is citation only per CONTEXT.md D-09, not re-adjudicated here. Re-verified fresh during this task (`grep -n "^edition" Cargo.toml crates/*/Cargo.toml`): all twelve edition-carrying manifests declare `edition = "2024"` — `Cargo.toml:35` and `crates/{paladin-core,paladin-ports,paladin-battalion,paladin-llm,paladin-memory,paladin-storage,paladin-notifications,paladin-content,paladin-web,paladin-herald,doc-examples}/Cargo.toml:4` — none remain on the `"2021"` this row's FR-5/§7 text specified. REL-02 (the code fix) is already applied; the `edition = "2021"` position is superseded, not merely stale. |
| REQ-paladin-core-scaffold | satisfied | `crates/paladin-core/Cargo.toml:1-2` (`name = "paladin-ai-core"`), `:14-15` (`[lib] name = "paladin_core"`, satisfying FR-4's "valid Cargo.toml and src/lib.rs" together with the crate's existing `src/lib.rs`), `:18-24` referencing workspace dependencies via `{ workspace = true }` syntax exactly as FR-5 specifies (`serde`, `serde_json`, `uuid`, `chrono`, `thiserror`, `async-trait`, `tokio`). FR-6's exact dependency list is tracked separately by the row below. Exercised by `cargo doc --offline -p paladin-ai-core --no-deps` (run this task) — builds the crate in isolation and completes with `Finished ... Generated .../paladin_core/index.html`, 0 errors — and the `crate-isolation` CI job's `paladin-ai-core` leg (`ci.yml:311-312`, build step `:348`). |
| REQ-paladin-core-dependency-allowlist-v1 | superseded by shipped code | Cites ADR-0015 (`.planning/decisions/0015-core-ports-dependency-allowlist.md`), which rebaselines this row's Appendix B six-crate "complete and exhaustive" claim (`.project/Milestone_5-Workspace-Decomposition/Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md:321-334`) against the measured tree rather than recording the difference as debt. Re-measured fresh this task: `crates/paladin-core/Cargo.toml:17-31` lists **fourteen** `[dependencies]` entries — the six PRD-permitted crates (`serde`, `serde_json`, `uuid`, `chrono`, `thiserror`, `async-trait`) plus `tokio`, `sha2`, `blake3`, `petgraph`, `murmur3`, `url`, `regex`, `futures` — matching ADR-0015's own recount exactly. The enforceable invariant ADR-0015 states — no provider SDK, transport client, storage driver, or web framework — holds: `cargo tree --offline -p paladin-ai-core` (run this task) contains zero matches for `application\|infrastructure\|redis\|sqlx\|mysql\|axum\|actix\|minio\|openai\|anthropic\|deepseek`. |
| REQ-core-base-extraction | satisfied | `crates/paladin-core/src/base/{entity,component,service}/*.rs` (directory listing run this task) contains every FR-9 type: `node.rs` (`Node<T>`), `collection.rs` (`Collection`), `field.rs` (`Field`), `message.rs` (`Message`), `action.rs` (`Action`), `event.rs` (`Event`). `grep -rn "use crate::application::\|use crate::infrastructure::" crates/paladin-core/src/base/` (run this task) returns zero matches, satisfying FR-11. Exercised by `cargo test --offline -p paladin-ai-core --lib base::` — 50 passed, 0 failed, run during this task. |
| REQ-core-container-extraction | satisfied | `crates/paladin-core/src/platform/container/` (directory listing run this task) holds `paladin.rs`, `paladin_config.rs`, the `battalion/` sub-tree, `garrison.rs`, `arsenal/`, `citadel.rs`, `herald.rs`, `sanctum.rs` and the other FR-14 types, module tree preserved. **Known partial supersession**: FR-14's inclusion of the Maneuver lexer/AST/parser inside this extraction was reversed one milestone later — see `REQ-maneuver-files-moved-from-core`'s row (Milestone 6 Epic 3 section above, filled by plan 07-01), which confirms `find crates/paladin-core/src -iname "*maneuver*" -o -iname "*parser*"` returns no results today. Recorded here inline per this plan's instruction, not reopened as a nested outstanding item. Exercised by `cargo test --offline -p paladin-ai-core --lib platform::container::` — 316 passed, 0 failed, run during this task. |
| REQ-core-upward-dependency-resolution | satisfied | `grep -rn "application::" crates/paladin-core/src/` (run this task) returns exactly one hit — a doc-comment cross-reference in `arsenal/handoff_error.rs:6` ("The `application` layer re-exports this type from `application::errors::handoff_error`"), not a `use` statement; `battalion/mod.rs:21-23` imports `PaladinResult`, `PaladinError`, `RegistryError` from crate-local `crate::platform::container::*` paths only. FR-17's hard constraint — `battalion/mod.rs` as shipped must not import from `application::` — holds. SM-10's decision artifact exists at `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` (confirmed present this task) and is promoted as ADR-0016, cited by the row below. Exercised by `cargo test --offline -p paladin-ai-core --lib platform::container::` — 316 passed, 0 failed (same run cited above, includes `battalion::` submodule tests). |
| REQ-port-value-type-ownership-v1 | satisfied | Cites ADR-0016 (`.planning/decisions/0016-port-value-type-ownership.md`), which ratifies this row's decision-record position (`paladin-core` owns) as the surviving one — the source record is `Status: Approved`, `2026-05-13`, `Chosen Option: Option A`. Confirmed shipped this task: `crates/paladin-core/src/platform/container/token_usage.rs:13` (canonical `TokenUsage`), `execution_result.rs:38` (`PaladinResult`), `execution_result.rs:76` (`StopReason`), `registry_error.rs:10` (`RegistryError`), `arsenal/handoff_error.rs:27` (`HandoffError`) — all five types the decision record names, all defined in `paladin-core`. Exercised by `cargo test --offline -p paladin-ai-core --lib platform::container::` — 316 passed, 0 failed (includes `token_usage::tests`, `execution_result::tests`, `registry_error::tests`). |
| REQ-facade-core-reexports | satisfied | `src/core/mod.rs:109` (`pub use paladin_core::base;`) and `src/core/platform/mod.rs:6-66` (`pub mod container { ... }`, re-exporting every FR-14 flat module plus `arsenal` and a `battalion` sub-block at `:47-65` that layers `maneuver`/`parser` shims on top of `paladin_core::platform::container::battalion::*`) preserve every pre-extraction `paladin::core::...` import path. Root `paladin` crate lists `paladin-core = { workspace = true }` at `Cargo.toml:51`. `src/core/` (directory listing run this task) contains only `mod.rs`, `platform/{mod.rs,README.md}` and `platform/manager/` (core services outside FR-14's extraction scope) — no relocated source remains, satisfying SM-8. Exercised by `cargo check --offline --workspace --lib` (this task) — `Finished` in 54.29s, 0 errors, satisfying FR-22 — and `cargo test --offline --test unit maneuver` — 21 passed, 0 failed, run during this task, resolving through this exact `container`/`battalion` re-export block via `paladin::core::platform::container::battalion::maneuver`. |
| REQ-core-dependency-validation | satisfied | `cargo tree --offline -p paladin-ai-core` (run this task) contains zero matches for `application\|infrastructure\|redis\|sqlx\|mysql\|axum\|actix\|minio\|openai\|anthropic\|deepseek`, satisfying FR-24/FR-25. `cargo doc --offline -p paladin-ai-core --no-deps` (run this task) completes with `Finished`, 0 errors, 0 broken intra-doc links, satisfying FR-26. The historical `.project/Milestone_5-Workspace-Decomposition/Epic_1/paladin-core-dependency-tree.txt` and `baseline-test-count.txt` artifacts remain locatable (not casualties of the `project/`→`.project/` rename) but are pre-rename-era snapshots (`v0.1.0`, a since-removed `fasthash` dependency) rather than current evidence; this row's verdict rests on the fresh commands above, not on those historical files. |

### Milestone 5 Epic 2 — paladin-ports Extraction (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-ports-scaffold | superseded by shipped code | Cites ADR-0015, which rebaselines this row's FR-3 seven-crate allowlist (`paladin-core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, `tokio`) against the measured tree. Re-measured fresh this task: `crates/paladin-ports/Cargo.toml:1-2` (`name = "paladin-ports"`), `:20-31` lists **eleven** `[dependencies]` entries — the base seven plus `serde_json`, `futures`, `md5`, `mime_guess` (`:31`) — not the ten `intel/code-verification.md` records, matching ADR-0015's own recount exactly (`mime_guess` postdates that intel figure). `:14-18` (`[lib] doctest = false`) is tracked separately by `REQ-ports-doctest-compilation` below. Exercised: the `crate-isolation` job's `paladin-ports` leg (`ci.yml:313-314`, build step `:348`) and `cargo test --offline -p paladin-ports --lib` (this task) — 98 passed, 0 failed, which requires a successful isolated build (FR-4). |
| REQ-output-ports-extraction | satisfied | `crates/paladin-ports/src/output/` (directory listing run this task) contains all 18 files FR-7's table names — `arsenal_port.rs`, `battalion_port.rs`, `citadel_port.rs`, `content_delivery_port.rs`, `embedding_port.rs`, `file_storage_port.rs`, `garrison_port.rs`, `llm_port.rs`, `log_port.rs`, `notification_port.rs`, `paladin_executor_port.rs`, `paladin_port.rs`, `paladin_registry.rs`, `queue_port.rs`, `sanctum_port.rs`, `scheduler_port.rs`, `search_engine_port.rs`, `vision_llm_port.rs`, `vision_port.rs` — plus six output ports added since (`auth_port.rs`, `orchestrator_port.rs`, `repository_port.rs`, `streaming_executor_port.rs`, `user_repository_port.rs`, `workflow_repository_port.rs`), a superset rather than a shortfall. `grep -rn "use crate::application::\|use crate::infrastructure::\|use crate::core::" crates/paladin-ports/src/` (run this task) returns zero matches, satisfying FR-9. Exercised by `cargo test --offline -p paladin-ports --lib output::` — 86 passed, 0 failed, run during this task. |
| REQ-input-ports-extraction | satisfied | `crates/paladin-ports/src/input/` (directory listing run this task) contains exactly FR-12's six files: `content_input_port.rs`, `document_port.rs`, `listener_port.rs`, `ml_port.rs`, `nlp_port.rs`, `rpc_port.rs`. The same zero-match `use crate::application::/infrastructure::/core::` grep as the row above applies (`crates/paladin-ports/src/` was checked as a whole). Exercised by `cargo test --offline -p paladin-ports --lib input::` — 12 passed, 0 failed, run during this task. |
| REQ-ports-facade-wiring | satisfied | Confirmed this task: `src/application/ports` does not exist (`ls` returns "No such file or directory"), satisfying FR-16's "no shim files left behind" — `.project/…prd-paladin-ports-extraction.md` §9 Resolved Design Decision 2 ("Backward-compatibility strategy: Full deletion selected (Option B)... produces a cleaner final state with no shim debt") is what shipped, not FR-17's original literal "re-export at old paths" text; the PRD's own later resolution supersedes its own earlier requirement text, and the tree matches the resolution. Root `paladin-ai` package lists `paladin-ports = { workspace = true }` at `Cargo.toml:52` (FR-15). Re-export wiring is centralized in `src/prelude.rs:41-45` (e.g. `pub use paladin_ports::output::llm_port::{LlmError, LlmPort, LlmRequest, LlmResponse};`) rather than `src/lib.rs` directly, and at least twenty source files import `paladin_ports::` at their call sites directly (`grep -rln "paladin_ports::" src/` run this task). Exercised by `cargo check --offline --workspace --lib` (this task) — `Finished`, 0 errors. |
| REQ-ports-import-migration | satisfied | `grep -rn "application::ports::" src/ tests/ examples/ docs/` (run this task) returns exactly one hit — a doc-comment historical cross-reference in `tests/unit/herald_consolidation_test.rs:22`, not a `use` statement — and zero hits inside `crates/paladin-ports/src/` itself, satisfying FR-19/FR-20's "only the `use` statement path strings change" scope. Exercised by the same `cargo check --offline --workspace --lib` run cited above — `Finished`, 0 errors — which would fail to compile on any unmigrated `crate::application::ports::` reference. |
| REQ-ports-doctest-compilation | genuinely outstanding | `crates/paladin-ports/Cargo.toml:14-18` sets `[lib] doctest = false` with an inline comment deferring re-enablement to "Task 7.0"; `.github/workflows/ci.yml:226` (`cargo test --workspace --doc --exclude paladin-ports`, re-grepped this task — the citation elsewhere in this corpus at `:225` is off by one line) excludes the crate from the workspace doctest run entirely. Neither line has changed. Points at Phase 8 / DEBT-03, which re-enables the doctests; no code change is made in this phase per this plan's prohibitions. |
| REQ-ports-docs-markdown-update | satisfied | `grep -rn "application::ports::" docs/` (run this task) returns zero matches. `docs/src/api-reference/stable-api.md:9` states the canonical import path is `crates/paladin-ports/` and gives the example `use paladin_ports::output::llm_port::LlmPort`; the trait-reference table (`:493` onward) lists every port at its `paladin_ports::output::...` path. `docs/src/api-reference/migration-guide.md:67` instructs replacing short paths with `paladin_ports::`/`paladin_core::`/etc. crate-level paths. These are the mdbook-relocated FR-22 deliverable (D-04 caveat (b) governs the file-location half; content itself verified current here). |
| REQ-ports-layering-validation | satisfied | `cargo tree --offline -p paladin-ports` (run this task) shows exactly one workspace-internal dependency, `paladin-ai-core` (satisfying FR-24), and contains zero matches for `redis\|sqlx\|aws-sdk-s3\|minio\|openai\|anthropic` (satisfying FR-23); `reqwest` does not appear at all. `cargo build -p paladin-ports` succeeds independently, confirmed via the same `cargo test --offline -p paladin-ports --lib` run cited above (98 passed — a passing test run requires a successful prior build) and the `crate-isolation` job's dedicated `paladin-ports` matrix leg (`ci.yml:313-314,348`). FR-25/FR-26's specific artifact-save paths (`project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-{isolation-build,dependency-tree}.txt`) are historical `project/`-era paths not reproduced under `.project/` for this crate (unlike Epic 1's dependency-tree file); the underlying commands both pass fresh, which is what FR-23/FR-24 actually gate. |
| REQ-ports-tests-and-rustdoc | present, unproven | FR-27 holds: `cargo test --offline -p paladin-ports --lib` — 98 passed, 0 failed, run during this task. FR-29's rustdoc-preservation claim largely holds: `grep -rc "^///\|^//!"` across `crates/paladin-ports/src/{output,input}/*.rs` (run this task) totals 6683 doc-comment lines, and `cargo doc --offline -p paladin-ports --no-deps` under `RUSTDOCFLAGS="-D warnings"` (run this task) completes with `Finished`, 0 warnings, satisfying FR-30's "zero broken intra-doc links" on the literal check. It does not clear the `satisfied` bar for the row as a whole: FR-29's "no documentation may be lost" clause is undermined by the doctest disablement above — `cargo test --workspace --doc --exclude paladin-ports` never compiles a single one of this crate's embedded doc examples, so any example that silently rotted since extraction would not be caught, which is exactly what "no documentation may be lost" is meant to guard against. Points at Phase 8 / DEBT-03 for the same reason as the row above; recorded `present, unproven` rather than `satisfied` or `genuinely outstanding` because the rustdoc text itself is present and its structural link-integrity is proven, only its example-code correctness is unverified. |
| REQ-port-value-type-ownership-v2 | superseded by shipped code | Cites ADR-0016. This row's FR-7/FR-10 text (`.project/…prd-paladin-ports-extraction.md`, annotated 2026-08-06) would move `PaladinResult`, `StopReason` and `TokenUsage` out of `paladin-core` and into this crate, applied literally reintroducing the exact upward dependency the Epic 1 decision removed. Shipped code instead extends FR-11's `RegistryError` core-re-export carve-out to all three: `crates/paladin-ports/src/output/paladin_port.rs:389` (`pub use paladin_core::platform::container::execution_result::{PaladinResult, StopReason};`) and `crates/paladin-ports/src/output/llm_port.rs:671` (`pub use paladin_core::platform::container::token_usage::TokenUsage;`) — both confirmed thin re-exports, not independent bodies, during this task. The `.project/` source annotation is already in place (top-of-file banner citing ADR-0016 and ADR-0014, FR-7/FR-10 struck through with corrected text inline) — not repeated here per D-00g. Row kept separate from `REQ-port-value-type-ownership-v1`, per this plan's prohibition against merging variant-pair rows. |

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
