# Milestone 7-8 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 7-8 as-shipped ledger` section (D-01,
Phase 10 plan 10-01). That section becomes a pointer to this file. Phase 13 adds the fifth sibling
ledger, `milestone-09-12.md`, in this same directory — `.planning/ledgers/milestone-04-06.md`'s own
head note already named this file (`milestone-07-08.md`) as Phase 10's deliverable before this plan
existed.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the requirement
they belong to, not given their own identifiers — nesting them keeps this ledger joinable to
`REQUIREMENTS.md` and `ROADMAP.md` without inventing new IDs (D-00f). The same `file:line` citation
may legitimately appear in more than one row: two requirements describing the same shipped artefact
keep separate rows and separate verdicts, because the `REQ-*` ID is the primary key, not the
citation. Two rows are never merged because they cite the same artefact.

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven`. This bar applies to **all 86 rows
below without exception**, including every row REQUIREMENTS.md's run-4 ledger already marked with an
ingest-era status word (`Shipped`, `Verify`, `Variant`, `Code diverges`, etc.). An ingest status word
**is** the bare "the code exists" claim this bar exists to reject — it is re-derived, not carried
forward, for every one of the 86 rows.

**Manifest carve-out.** Milestone 7 is a structural milestone like Milestones 4-6, so a large share
of its requirements *are* manifest declarations — crate extraction, feature-flag shape, CI job
coverage. For those, the manifest line **plus** a named CI job or build leg that consumes it is the
exercising artefact. The M7/M8-specific anchors, all re-grepped fresh against the tree on 2026-08-08
rather than trusted from an earlier document:

- The `lint` job's zero-warning `cargo doc` step — `.github/workflows/ci.yml:58`
  (`cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" ...`).
- The workspace clippy and test steps — `.github/workflows/ci.yml:55` (`cargo clippy --workspace
  --all-targets --all-features -- -D warnings`), `:235` (`cargo test --workspace --lib --bins`),
  `:238` (`cargo test --workspace --doc`, no `--exclude`).
- The feature-matrix and library-only isolation steps in `.github/workflows/feature-flags.yml` —
  the `no-default-features` matrix leg at `:26`, the `cli-isolation` job at `:121`, and its
  "Check library (no-default-features, no cli)" step at `:138` (`cargo check --lib
  --no-default-features`).
- The per-crate, dependency-ordered publish dry-run leg in `.github/workflows/release.yml:410`
  (`cargo publish --dry-run -p "$crate"`, inside the dependency-ordered loop).

A manifest fact cited without its consuming CI job is `present, unproven`, not `satisfied` — a bare
`Cargo.toml` feature declaration is the exact false-positive class this bar exists to reject.

**Path caveats.** Six Milestone 7-8 deliverables ship as mdbook chapters under `docs/src/` rather
than at their PRD paths: `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`,
`docs/VERSIONING_POLICY.md`, `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md` and the root
`STABLE_API.md`. A row whose only divergence from its PRD is this caveat is `relocated`, not a gap.

**Workspace shape and provenance.** The workspace is, authoritatively, **ten library crates plus
`doc-examples` plus the root facade package `paladin-ai`** — verified this session via
`.planning/codebase/STRUCTURE.md:51-71` (already corrected by Phase 7; this phase inherits a correct
map) and `ls crates/`, which returns exactly: `doc-examples`, `paladin-battalion`, `paladin-content`,
`paladin-core`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-notifications`,
`paladin-ports`, `paladin-storage`, `paladin-web`. The root facade is confirmed at `Cargo.toml:33`
(`name = "paladin-ai"`). Of the five crates ARCH-01 once marked "provenance pending": `paladin-storage`,
`paladin-notifications`, `paladin-content` and `paladin-web` trace to M7 Epic 1's extraction PRD and
its four-Go cost-benefit gate (`cost-benefit-assessment.md`, self-approved 2026-05-25). **`paladin-herald`
traces to the 2026-06-04 reconciliation, not to any PRD** — it was created by reconciliation commit
`66f6c4e`, inside an Epic (M8 Epic 3) whose own §5 Non-Goals state "No new crates created.
`paladin-herald`, `paladin-ml`, etc. are not in scope." That non-goal is recorded elsewhere in this
ledger (M8 Epic 3) as **overridden for `paladin-herald`, still holding for `paladin-ml`**. This is
why no ingested requirement described `paladin-herald` before ingest run 4, and why this planning
set's earlier "nine-crate workspace" figure was wrong.

**Per-milestone checkbox corroboration.** Per `intel/task-completion-state.md`: Milestone 7 is 98.8%
complete with 3 open items, all in `tasks-production-build-infra-adaptation.md` — **plausible but
uncorroborated** by any single artefact (run 4's finding, restated here rather than re-derived).
Milestone 8 is 99.1% complete with 3 open items (2 in `tasks-remove-dead-shims-empty-modules.md`, 1
in `tasks-relocate-remaining-misplaced-modules.md`) — **contradicted by code**: Epics 2 and 3 are both
verifiably complete in the tree, and Epic 3 went further than its own task list scoped. These counts
are recorded here as inputs to the per-row verdicts below, not trusted as verdicts themselves.

**Provenance.** Phase 10, plan 10-01 (wave 1 scaffold), dated 2026-08-08. This plan writes the head
notes below, the legend, the supersession summary table, all twelve section headings and all 86 row
stubs, and fully derives Milestone 8 Epic 4's four rows end-to-end. Waves 2-3 fan-out plans replace
the remaining 82 rows' Verdict and Evidence cells in place, per the contention table below.

## Verdict legend

| Verdict | Meaning |
| --- | --- |
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `relocated` | The deliverable exists, but at a different path than the requirement names |
| `superseded by outcome` | Shipped code answers the requirement differently than the ingested document specified, and implementing the requirement as written would undo shipped work |
| `deferred with register` | Work was removed deliberately and recorded with a reintroduction condition, distinct from work never done |
| `diverged` | The shipped tree deliberately implements the requirement differently, as distinct from a later milestone replacing it |

This wording differs deliberately from `milestone-04-06.md`'s legend, which uses `superseded by
shipped code` and `deferred with reason` — this ledger's D-02 vocabulary is `superseded by outcome`
and `deferred with register` instead. The shape, the columns, and the two callouts below are copied
from that file; the six-word class names are not.

**HARD-01 mapping callout.** HARD-01 requires four dispositions: `Shipped`, `Superseded by outcome`,
`Relocated`, `Deferred with register`. These map onto this legend as: `Shipped` → `satisfied` (once
re-derived) or `present, unproven` (where nothing exercises it yet); `Superseded by outcome` →
`superseded by outcome`; `Relocated` → `relocated`; `Deferred with register` → `deferred with
register`. `present, unproven`, `genuinely outstanding` and `diverged` are this ledger series' own
finer vocabulary, not named by HARD-01 directly.

**Tie-break rule.** A row that qualifies for both `relocated` and `superseded by outcome` takes
`relocated`, because the mdbook relocations are this corpus's single largest false-gap generator
(six documents, four run-4 requirements — see the summary table below) and collapsing the class
would destroy the moved-not-missing signal that stops a later phase from planning them as missing.

## Superseded by outcome — do not plan these as written

Placed here, at the head of the file, so a reader never has to scan 86 rows to find these.

**Counting command run this session:** `sed -n '365,381p' .planning/intel/code-verification.md |
grep -c '^|'` → **15** (13 data rows + 1 header + 1 separator = 15 lines beginning `| `). The table
holds **13 data rows**, not the "14" figure HARD-01's own text and `REQUIREMENTS.md:3136`/`:3870`
both assert (D-05). That figure is corrected at source by this same plan's Task 3.

**Caveat on this table's contents.** This table transcribes `intel/code-verification.md:365-381`'s
13 rows directly, per Task 1's instruction to build it from the table rather than from a remembered
figure. Three of the 13 rows below (marked †) are **mdbook relocations** — per this ledger's own
tie-break rule (above), their eventual per-row verdict in the epic sections is `relocated`, not
`superseded by outcome`. They appear in this summary table because `code-verification.md`'s own
table lists them under this heading; a future reader should not conclude their epic-section verdict
will read `superseded by outcome`. Two further rows (marked ‡) are HARD-05 and HARD-06 subjects
whose restated-rule ADRs (ADR-0031, ADR-0032 — Phase 10 wave 3) are expected to flip their epic-row
verdict to `satisfied`; this table's "what the tree does instead" column describes the pre-ADR
finding, which is what `code-verification.md` recorded.

| `REQ-*` ID | What the requirement says | What the tree does instead | Citation |
|---|---|---|---|
| `REQ-paladin-web-extraction` | `paladin-web` declares `actix-web` **and** `axum` as direct non-optional dependencies (M7 Epic 1 PRD §4.2.1) | Zero `actix` references in `crates/paladin-web/`; facade `web-server = ["dep:paladin-web", "dep:axum"]`. M8 Epic 7 reversed it deliberately and added a cargo-deny ban | `crates/paladin-web/`; `Cargo.toml:276`; `deny.toml:99-103` |
| `REQ-storage-feature-flags-v1` | Facade `storage-sqlite` flag; `paladin-storage` optional; `storage` alias enables both backends (M7 Epic 1 PRD §4.5.6, §7.2) | `storage-sqlite` retired; `paladin-storage` non-optional with `sqlite` always on; `storage = ["storage-mysql"]` only | root `Cargo.toml` (`paladin-storage` dependency line); commit `897e77e` |
| `REQ-sqlx-workspace-dependency` | `sqlx` workspace declaration includes `mysql` in its feature list (M7 Epic 1 PRD §7.5) | `sqlx = { version = "0.8", default-features = false, features = [...] }` — no `mysql`; `default-features = false` and `migrate` added by the RustSec hardening work | root `Cargo.toml` `[workspace.dependencies]` sqlx line |
| `REQ-ci-publish-dry-run-v1` | `publish-dry-run` runs `cargo publish --dry-run -p <crate>` for ten crates in dependency order (M7 Epic 2 FR-26) | A single `cargo publish --workspace --dry-run` ships at `ci.yml:644`, with an inline rationale that per-crate dry runs cannot work on a version bump — **but** `release.yml:410` also ships the per-crate, dependency-ordered form. The 86-row run-4 ledger's own text records this row `Coexists, not superseded`, not superseded — flagged here as a discrepancy between `code-verification.md`'s run-4 finding and a later re-check; the owning fan-out plan (10-07) re-derives which verdict holds | `.github/workflows/release.yml:410`; `.github/workflows/ci.yml:644` |
| `REQ-tensorflow-ml-feature-gate-v2` | `tensorflow_adapter.rs` gated behind a new `ml = []` feature (M8 Epic 3 PRD §4.3 item 11) | Both the adapter and the flag were deleted outright (commit `3d48768`); neither exists | `git log` commit `3d48768`; `grep -rn "tensorflow\|^ml = " Cargo.toml src/` — zero matches |
| `REQ-m8-epic3-no-extractions` | Every adapter group "Stays in facade", all List B moves deferred to Milestone 9 (M8 Epic 3 PRD §4.3, §5; `infrastructure-adapter-disposition.md`) | The relocations were executed in Milestone 8 by the reconciliation; `paladin-herald` was created despite the explicit non-goal naming it | reconciliation §7 commits `66f6c4e`, `8bd7073`, `ff829e2`, `5a7c901`, `897e77e`, `4c7857e` |
| `REQ-dead-file-batch-deletion` | `find src/ -name "*.rs" \| wc -l` = **163** after Epic 2, **160** after Epic 4 (M8 Epic 2 §7; M8 Epic 4 §4.5 item 9) | Run-4 recorded **136**. The two PRD figures are internally consistent with each other; the further reduction is the reconciliation's Category 1-2 deletions, which the PRDs did not anticipate | `find src/ -name "*.rs" \| wc -l` (run 4, `intel/code-verification.md:375`) |
| `REQ-stable-api-per-crate` † | `STABLE_API.md` at the repository root, updated by four separate run-4 requirements (M7 Epic 4 §4.6; M8 Epics 2, 4, 5) | No `STABLE_API.md` at the root. The equivalent ships as `docs/src/api-reference/stable-api.md` after the Milestone 11 overhaul. `api_surface_current.txt` (881 KB) and `final-api.txt` (198 KB) **do** exist at the root | `docs/src/api-reference/stable-api.md`; also touches `REQ-stable-api-v020-sync` (M8 Epic 5) |
| `REQ-build-baselines-doc`, `REQ-integration-tests-doc`, `REQ-performance-baseline-doc`, `REQ-release-checklist`, `REQ-versioning-policy` † | `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`, `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md` (M7 Epics 2, 3, 4) | `docs/` holds only `MIGRATION_LOG.md` plus the mdbook. Equivalents ship as `docs/src/appendix/{performance-baseline,release-checklist,release-automation,integration-tests,build-baselines}.md` — the same Milestone 11 relocation run 3 documented. **Do not plan these as missing deliverables** | `docs/src/appendix/*.md` (five files) |
| `REQ-facade-role-lib-docs` | Nine leaf crates named in the facade role documentation (M8 Epic 5 FR-1) | Ten library crates ship; the FR-1 list predates `paladin-herald` | `src/README.md`; `ls crates/` (eleven entries including `doc-examples`) |
| *(no `REQ-*` owner — M7 overview Appendices A/B, not part of the 86-row inventory)* | `crates/paladin-cli/` as a workspace crate, publish Step 5 (M7 overview Appendices A and B) | No `paladin-cli` crate — re-confirming the run-3 finding. `crates/` holds `doc-examples` plus the ten library crates | `ls crates/` |
| `REQ-extracted-crate-dependency-rule` ‡ | Extracted crates depend only on `paladin-ports`, `paladin-core` and workspace-shared deps; "no extracted crate may depend on another extracted crate" (M7 Epic 1 PRD §6.1) | `crates/paladin-content/Cargo.toml` declares `paladin-llm = { ..., optional = true }` behind its `llm` feature — an extracted-to-extracted edge, but non-default and facade-gated. The PRD's own §4.4 complexity note (actually at `cost-benefit-assessment.md:118`, not the same PRD's §4.4) anticipated it without amending the rule. **HARD-05/ADR-0031 (Phase 10 wave 3) is expected to restate the rule and flip this row to `satisfied`** — recorded here as the pre-ADR finding | `crates/paladin-content/Cargo.toml:23,28` |
| `REQ-content-processing-build-gate` ‡ | Facade `content-processing` activates `paladin-content` "with all capability features enabled" (M7 Epic 1 PRD §4.4.6) | `content-processing` enables `web-scraping`, `rss`, `news-api`, `tiktoken`, `llm` — but **not** `pdf`. `paladin-content` does declare `pdf = []`, gating no dependency. **HARD-06/ADR-0032 (Phase 10 wave 3) is expected to record PDF extraction as unconditionally shipped and this feature as inert-and-deleted** — recorded here as the pre-ADR finding | `Cargo.toml:275`; `crates/paladin-content/Cargo.toml:18` |

## Row order and amendment convention

The twelve epic sections below appear in `REQUIREMENTS.md`'s own run-4 order — Milestone 7 Epics 1-4,
Milestone 8 Epics 1-7, then the cross-milestone entries — and are never re-sorted. Rows within a
section appear in the ID order `REQUIREMENTS.md` lists them. Later plans replace a row's **Verdict**
and **Evidence** cells in place; they never insert, delete, or reorder rows. Amendments follow D-00d:
edit in place, retain superseded text, date every amendment, never a separate corrections file.

**Ledger file contention** — the rule every ledger-writing plan in this phase obeys:

| Plan | Wave | Owns | May |
|---|---|---|---|
| 10-01 | 1 | the whole file | create head notes, legend, summary table, twelve section headings, all 86 row stubs; fully derive M8 Epic 4's four rows |
| 10-07 | 3 | M7 Epic 1 (12 rows) + M7 Epic 2 (13 rows) | replace Verdict and Evidence cells **in place** inside its own two sections only |
| 10-08 | 3 | M7 Epic 3 (10 rows) + M7 Epic 4 (12 rows) | same, its own two sections only |
| 10-09 | 3 | M8 Epic 1 (4) + M8 Epic 2 (4) + M8 Epic 3 (6) | same, its own three sections only |
| 10-10 | 3 | M8 Epic 5 (6) + M8 Epic 6 (4) + M8 Epic 7 (6) + Cross-milestone (5) | same, its own four sections only |
| 10-11 | 4 | a dated close-out amendment section appended at the foot | append only |

M8 Epic 4's four rows are derived by plan 10-01 and are owned by no fan-out plan. The four wave-3
plans run in parallel over **disjoint, contiguous** section ranges and perform **cell replacement
only** — never row insertion, deletion or reordering — so the four diffs are non-adjacent hunks in
one file and merge without conflict. `grep -c '^| REQ-'` reads `86` before and after every one of
them, and `git diff --numstat` on this file shows added lines equal to deleted lines for those plans.

For every row this plan does not derive, the Verdict cell reads `pending — plan 10-NN` naming the
owning fan-out plan from the table above, and the Evidence cell carries the run-4 ledger's current
text verbatim, prefixed `run-4 input (not yet re-derived):`. No cell is ever left blank.

### Milestone 7 Epic 1 — Extended Workspace Decomposition (12 IDs)

**Epic note:** *(reserved for plan 10-07)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-m7-cost-benefit-gate | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — `cost-benefit-assessment.md` produced with a self-approval block dated 2026-05-25; four Go decisions, so PRD sub-tasks 1.4/1.5 (mark deferred, create backlog tickets) were correctly recorded N/A |
| REQ-paladin-web-extraction | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped, superseded** — `crates/paladin-web/` exists; its two-framework clause is reversed by `REQ-actix-removal`. Variant group 21 |
| REQ-paladin-notifications-extraction | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — `crates/paladin-notifications/` with README and CHANGELOG; the per-feature `email`/`push`/`system` criteria were not individually re-checked → HARD-01 |
| REQ-paladin-content-extraction | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped (relocated)** — crate exists; its `use_cases/` target directory was renamed to `services/` by M8 Epic 6, and `content_ingestion_service.rs` stayed in the facade → deferred item D4 (FACADE-02) |
| REQ-paladin-storage-extraction | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped, superseded** — crate exists; `file_content_repository.rs` was deleted rather than kept in the facade. Variant group 28 |
| REQ-storage-feature-flags-v1 | pending — plan 10-07 | run-4 input (not yet re-derived): **Superseded by outcome** — `storage-sqlite` retired, `paladin-storage` non-optional. Variant group 22 |
| REQ-facade-workspace-metadata | pending — plan 10-07 | run-4 input (not yet re-derived): Verify — all four crates are in `[workspace.members]` and `[workspace.dependencies]`; the "no public API paths may be silently removed" clause was not audited → HARD-01 |
| REQ-extracted-crate-dependency-rule | pending — plan 10-07 | run-4 input (not yet re-derived): **Code diverges → HARD-05** — `crates/paladin-content/Cargo.toml` declares optional `paladin-llm`, an extracted-to-extracted edge the rule forbids absolutely and the same PRD's §4.4 anticipated |
| REQ-extraction-order-and-shims | pending — plan 10-07 | run-4 input (not yet re-derived): Verify — the storage-first order was followed; the shim protocol was not re-checked → HARD-01 |
| REQ-tensorflow-stays-facade-v1 | pending — plan 10-07 | run-4 input (not yet re-derived): **Superseded by outcome** — adapter and flag deleted. Variant group 24 |
| REQ-sqlx-workspace-dependency | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped, narrowed** — `sqlx` stays in `[workspace.dependencies]`, but as `default-features = false` with `["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json", "migrate"]`. **`mysql` is absent** from the workspace feature list against §7.5's explicit form, and `migrate` was added; both changes trace to the RustSec hardening work → SEC-01 |
| REQ-dependency-isolation-metrics | pending — plan 10-07 | run-4 input (not yet re-derived): Verify — the dep-tree reduction targets were not re-measured → HARD-01 |

### Milestone 7 Epic 2 — Production Build Infrastructure (13 IDs)

**Epic note:** *(reserved for plan 10-07)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-docker-workspace-build | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped, defect → SEC-05** — `Dockerfile.chef` pins `cargo-chef 0.1.77 --locked`, runs `chef prepare` / `chef cook --release --workspace`, and uses `rust:1.93-slim-bookworm`; its planner COPY list enumerates nine manifests and omits `crates/paladin-herald/Cargo.toml` — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-05 **closed** — plan 09-03 (commit `52b1943`) deleted the nine-manifest enumeration rather than adding a tenth line, per ADR-0027; planner-stage crate coverage is now structural (`COPY crates ./crates`), not enumerated. **This row is also one of Phase 9's D-04 hand-off rows — cite ADR-0027 and commit `52b1943`, do not re-verify from scratch (see Phase 9's hand-off block, `REQUIREMENTS.md:1320-1355`)** |
| REQ-build-baselines-doc | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped (relocated)** — `docs/BUILD_BASELINES.md` does not exist; the equivalent ships as `docs/src/appendix/build-baselines.md` after the Milestone 11 overhaul. **Do not plan as missing** |
| REQ-makefile-workspace-targets | pending — plan 10-07 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-makefile-per-crate-targets | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — all ten targets at `Makefile:167-212` (`test-core` … `test-facade`) |
| REQ-ci-workflow-triggers | pending — plan 10-07 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-ci-per-crate-matrix | pending — plan 10-07 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-ci-workspace-job | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — `--workspace` clippy / doc / test at `ci.yml:54,57,222,225`, but `:225` carries `--exclude paladin-ports` → DEBT-03, HARD-07. **Re-derive this citation — Task 3 of this plan finds `ci.yml:238` is the current bare `cargo test --workspace --doc` with no `--exclude`; the `:225` line number and the exclusion itself are stale (D-21)** |
| REQ-ci-integration-job | pending — plan 10-07 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-ci-publish-dry-run-v1 | pending — plan 10-07 | run-4 input (not yet re-derived): **Coexists, not superseded** — the per-crate dependency-ordered form ships at `release.yml:410`. Variant group 23 |
| REQ-ci-publish-dry-run-v2 | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — `ci.yml:644` runs a single `cargo publish --workspace --dry-run` with an inline counter-rationale. No document carrier, so no precedence standing. Variant group 23 |
| REQ-ci-feature-flag-matrix | pending — plan 10-07 | run-4 input (not yet re-derived): Shipped — `feature-flags.yml:115,118`; the library-only isolation test at `:141`. **Re-derive — this session's re-grep finds the `cli-isolation` job at `:121` with its library-only check at `:138`, not `:141`; re-verify before citing** |
| REQ-integration-test-placement | pending — plan 10-07 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-integration-tests-doc | pending — plan 10-07 | run-4 input (not yet re-derived): **Shipped (relocated)** — `docs/src/appendix/integration-tests.md`. Do not plan as missing |

### Milestone 7 Epic 3 — Benchmark Suite Migration (10 IDs)

**Epic note:** *(reserved for plan 10-08)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-sanctum-bench-migration | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — `crates/paladin-memory/benches/sanctum_benchmarks.rs`, with imports rewritten to `paladin_core` / `paladin_memory` / `paladin_ports` and Criterion registration owned by the crate |
| REQ-disabled-bench-disposition | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — none of the five was directly restored; `herald_benchmarks`, `paladin_benchmarks` and `arsenal_benchmarks` were deprecated and removed, `battalion` and `garrison` removed and replaced at narrower scope |
| REQ-battalion-benchmarks | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — `crates/paladin-battalion/benches/battalion_benchmarks.rs` |
| REQ-llm-serialization-benchmark | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` |
| REQ-garrison-benchmarks | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — `crates/paladin-memory/benches/garrison_benchmarks.rs` |
| REQ-config-loading-benchmark | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — root `benches/config_benchmarks.rs`; the ownership finding (`Settings` lives in `src/config/settings.rs`, no extracted crate owns it) closes the PRD's open question 1 |
| REQ-critical-path-bench-scope | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — four categories; all six PRD success metrics recorded Satisfied in the assessment's own status table → HARD-01 |
| REQ-workspace-bench-execution | pending — plan 10-08 | run-4 input (not yet re-derived): Verify — `cargo bench --workspace --no-run` is the recorded structural compile-validation command → HARD-01 |
| REQ-performance-baseline-doc | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped (relocated)** — `docs/src/appendix/performance-baseline.md`. Note this does **not** close QUAL-05, which owns producing measured *runtime* numbers rather than the document |
| REQ-bench-regression-signal | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — `ci.yml:531` job `benchmark-regression-signal`, threshold "more than 3 Criterion regression notices in one run", non-blocking via `continue-on-error` |

### Milestone 7 Epic 4 — API Stabilization & Pre-Release Preparation (12 IDs)

**Epic note:** *(reserved for plan 10-08)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-crate-metadata-completion | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped, contested → SEC-02** — the `paladin-ai` / `paladin-ai-core` renames are applied with lib names preserved (`paladin`, `paladin_core`); the `license` field reads MIT against the signed dual-licence checklist — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-02 **closed** — plan 09-05 (commits `6bf860f`, `74a05fe`) relicensed the root package and all ten library crates to `MIT OR Apache-2.0`, per ADR-0025 and the repository owner's checkpoint answer; the contest is resolved, not merely narrowed. **This row is one of Phase 9's D-04 hand-off rows — cite ADR-0025, do not re-verify from scratch** |
| REQ-per-crate-readme | pending — plan 10-08 | run-4 input (not yet re-derived): Shipped — all ten library crates have a `README.md` |
| REQ-per-crate-changelog | pending — plan 10-08 | run-4 input (not yet re-derived): **Open defect → SEC-04** — nine of ten; `crates/paladin-herald/` has none, and the completion summary records this criterion Met — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-04 **closed** — plan 09-01 (commit `0458b6a`) created `crates/paladin-herald/CHANGELOG.md`; ten of ten library crates now have a `CHANGELOG.md`, mechanically enforced by `scripts/check-changelogs.sh` in the required CI job. **D-04 hand-off row — cite commit `0458b6a`, do not re-verify from scratch** |
| REQ-doc-coverage-audit | pending — plan 10-08 | run-4 input (not yet re-derived): **Contested → HARD-07** — the >90% coverage posture is recorded Met while `paladin-ports` sets `doctest = false` and CI excludes it from `--doc`. **Re-derive — Task 3/wave-3 finds `paladin-ports` has no `[lib]` section at all (`git log` shows `2bffe22 feat(08-03): re-enable paladin-ports doctests`) and `ci.yml:238` carries no `--exclude`; this row's premise is stale** |
| REQ-versioning-policy | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped (relocated), superseded by outcome** — no `docs/VERSIONING_POLICY.md`; the lockstep `0.2.0` target was superseded by the `0.1.0` publish → HARD-03 |
| REQ-release-checklist | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped (relocated)** — `docs/src/appendix/{release-checklist,release-automation}.md` |
| REQ-stable-api-per-crate | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped (relocated)** — no root `STABLE_API.md`; the equivalent ships at `docs/src/api-reference/stable-api.md`. `api_surface_current.txt` (881 KB) and `final-api.txt` (198 KB) do exist at the root → ARCH-05, DEBT-01 |
| REQ-release-readiness-audit | pending — plan 10-08 | run-4 input (not yet re-derived): **Shipped (history)** — every gate PASS, GO sign-off, tag `v0.1.0-rc.1` at `a9530fc`, all ten crates verified on docs.rs, external smoke project compiled → HARD-03 |
| REQ-rustsec-risk-acceptance | pending — plan 10-08 | run-4 input (not yet re-derived): **Open → SEC-01** — the accepted set has grown beyond the two documented advisories and diverges across four surfaces; the acceptance expires **2026-09-30** — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-01 **closed** — `SECURITY-EXCEPTIONS.md` (plan 09-02) is now the one register for all ten live suppressions; the 2026-09-30 acceptance is renewed to per-advisory `2026-12-31` review dates, owner `DF3NDR`, per ADR-0024. **D-04 hand-off row (item 1) — cite ADR-0024, do not re-verify from scratch** |
| REQ-rustsec-hardening-actions | pending — plan 10-08 | run-4 input (not yet re-derived): **Partially shipped → SEC-01** — `testcontainers-modules` is in `dev-dependencies`, MySQL compilation is gated on `storage-mysql`, and `sqlx` runs `default-features = false`; the four named open action items (two impact-analysis issues, approved `audit.toml` entries with owner and expiry, post-mitigation re-audit evidence) are unclosed — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-01 **closed** — the "approved `audit.toml` entries with owner and expiry" action item is satisfied by `SECURITY-EXCEPTIONS.md`'s ten fully-governed rows (plan 09-02/09-06); post-mitigation re-audit evidence (`cargo audit`/`cargo deny check` passing) remains CI-only, not run in this environment (HTTP 403 against crates.io). **D-04 hand-off row (item 2) — cite ADR-0024, do not re-verify from scratch** |
| REQ-license-policy-signoff | pending — plan 10-08 | run-4 input (not yet re-derived): **Contested → SEC-02** — a signed policy with a named approver that the manifests do not declare — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-02 **closed** — plan 09-05 declared `MIT OR Apache-2.0` in the root package and all ten library crates, matching the signed checklist; the PRD's single-licence claim is annotated superseded per ADR-0025. **D-04 hand-off row (item 3) — cite ADR-0025, do not re-verify from scratch** |
| REQ-paladin-ports-publish-verification-closed | pending — plan 10-08 | run-4 input (not yet re-derived): **Closed** — not forward work. The only residue is the collision guardrail → SEC-03 — **Amended by Phase 9 (plan 09-07), dated 2026-08-08:** SEC-03 **closed** — plan 09-04 (commits `264721a`, `2758a9d`, `5cde208`) shipped `.crate-names.txt` + `scripts/check-crate-names.sh`, an offline pre-dry-run guard; ADR-0026 records the decision and its accepted residual cost (a genuinely novel name is still a human crates.io check). **D-04 hand-off row (item 7) — cite ADR-0026, do not re-verify from scratch** |

### Milestone 8 Epic 1 — Facade Crate Audit (4 IDs)

**Epic note:** *(reserved for plan 10-09)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-facade-file-inventory | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped, superseded — 189 files audited 2026-05-29 |
| REQ-facade-file-classification | pending — plan 10-09 | run-4 input (not yet re-derived): **Shipped, superseded → HARD-02** — 151 stay / 13 move / 25 delete, with ~4,400 LOC of orphaned uncompiled duplicates classified as "active bridges that stay" |
| REQ-shim-consumer-validation | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped, superseded — the reconciliation's reproducible orphan test (`rg "mod <name>"` returns nothing; the directory `mod.rs` only does `pub use paladin_<crate>::…`) is the version to keep |
| REQ-facade-audit-document | pending — plan 10-09 | run-4 input (not yet re-derived): **Shipped, explicitly superseded** — the reconciliation's header names this document by path → HARD-02 |

### Milestone 8 Epic 2 — Dead Shim & Empty Module Removal (4 IDs)

**Epic note:** *(reserved for plan 10-09)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-dead-file-batch-deletion | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — all 25 List A files gone, plus the orphaned `notifications/`, `storage/`, `subject/`, `admin/` and `user/` directories. **Residue:** the `email_notifications.rs` (392 LOC) overlap review the PRD's Open Question 1 required is recorded nowhere. Variant group 27 |
| REQ-stale-application-ports-audit | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — `src/application/ports/` did not exist even at audit time; removed before Milestone 8 began |
| REQ-core-minimum-structure | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — `src/core/` is **exactly** the six named files, verified 2026-07-30 |
| REQ-libr-dead-reexport-removal | pending — plan 10-09 | run-4 input (not yet re-derived): Verify — the `lib.rs` alias removals were not individually re-checked → HARD-01 |

### Milestone 8 Epic 3 — Relocate Remaining Misplaced Modules (6 IDs)

**Epic note:** *(reserved for plan 10-09. Note for the owning plan: this section's non-goal clause is
where D-09's "overridden for `paladin-herald`, still holding for `paladin-ml`" split belongs, per
this ledger's own head note above.)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-notification-task-closeout | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — adapters live in `paladin-notifications` with a facade re-export; the three channel *services* were deleted rather than moved. Variant group 27 |
| REQ-storage-shim-deletion | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — superseded in *mechanism* by commit `897e77e`, which made `paladin-storage` non-optional rather than deleting shims naively |
| REQ-adapter-disposition-record | pending — plan 10-09 | run-4 input (not yet re-derived): **Shipped, superseded → HARD-02, FACADE-04** — 20 rows all "Stays"; two rows disagree with the governing PRD; names `paladin-arsenal` and `paladin-sanctum`, neither of which exists. Dated `2025-01`, inconsistent with every other M8 document. Variant group 26 |
| REQ-tensorflow-ml-feature-gate-v2 | pending — plan 10-09 | run-4 input (not yet re-derived): **Superseded by outcome** — the gate and the adapter were both deleted. Variant group 24 |
| REQ-garrison-sanctum-bridges-kept | pending — plan 10-09 | run-4 input (not yet re-derived): Shipped — both bridges remain with consumer evidence, and the §8 resolved-decisions record stands. Note its own factual correction: `api_content_deliverer.rs` is **724 LOC, not 629** (629 belongs to `tensorflow_adapter.rs`) — and the file was later deleted anyway |
| REQ-m8-epic3-no-extractions | pending — plan 10-09 | run-4 input (not yet re-derived): **Superseded by outcome → HARD-02** — the relocations were executed in Milestone 8. Variant group 25 |

### Milestone 8 Epic 4 — `use_cases` → `services` Rename (4 IDs)

**Epic note:** Fully derived by plan 10-01 (Phase 10 wave 1), dated 2026-08-08 — the end-to-end proof
of this ledger's head-note → legend → section → row → summary-table mechanism, before the 86-row
fan-out runs. Selected because it is the only complete section whose rows cite no ADR this phase has
not yet written, so it proves the mechanism without a forward reference. All citations below were
re-read this session; none is copied from the run-4 ledger text.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-use-cases-services-rename | satisfied | `src/application/services/` exists with eleven sub-modules — `analysis`, `arsenal`, `battalion`, `content`, `herald`, `log_orchestrator`, `notification_orchestrator`, `orchestration`, `paladin`, `queue_orchestrator`, `sanctum` (verified via `ls src/application/services/`, this session). Exercising artefact: the workspace-wide zero-match grep — `grep -rn "use_cases" src/ crates/ tests/ examples/ benches/ --include="*.rs"` returns **zero matches** (re-run this session; confirms the run-4 ledger's own claim of zero matches still holds against the current tree) |
| REQ-rename-clean-break | satisfied | `grep -rn "pub use services as use_cases" src/` returns **zero matches** (re-run this session) — no compatibility shim of any kind exists; the overview's optional Task 4.3 (an alias) was explicitly rejected by the PRD. Exercising artefact: the same zero-match grep above, which would also catch any lingering `use_cases` re-export, since the literal string is what is grepped |
| REQ-rename-doc-updates | present, unproven | Run-4 recorded `Verify → HARD-01` — "the 57 markdown references were not individually re-checked". Re-derived this session: `grep -rln "use_cases" --include="*.md" .` outside `.planning/`, `.git/` and `.project/` returns `CHANGELOG.md`, `docs/src/api-reference/migration-guide.md`, `docs/src/api-reference/stable-api.md`, `benches/BENCHMARK_FIXES.md` and `.github/copilot-instructions.md`. The first three **intentionally** document the old path as migration guidance (correct usage — a migration guide is supposed to name the path being migrated from). `.github/copilot-instructions.md`'s Architecture diagram (imported into `CLAUDE.md`) still shows `application/use_cases/` as the live module structure with no rename annotation — a genuine stale reference this row surfaces, but its fix sits outside this plan's `.planning`/`.project`/three-file config boundary (D-23) and outside `.github/`'s scope entirely. **Not `satisfied`**: the "57 references" claim was not individually re-checked in full, and at least one non-migration stale reference exists |
| REQ-rename-changelog-breaking | satisfied | `CHANGELOG.md:670-686` — "Services Directory Rename (Milestone 8, Epic 4)" carries the full breaking-change migration table (eleven old→new module paths). Exercising artefact: cross-checked this session against `ls src/application/services/` — all eleven documented target module names (`paladin`, `battalion`, `arsenal`, `content`, `herald`, `orchestration`, `log_orchestrator`, `notification_orchestrator`, `queue_orchestrator`, `sanctum`, `analysis`) exist exactly as named, confirming the changelog's table is accurate and not stale |

### Milestone 8 Epic 5 — Facade Role Documentation & v0.2.0 Finalization (6 IDs)

**Epic note:** *(reserved for plan 10-10)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-facade-role-lib-docs | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `src/lib.rs` carries the facade / composition-root documentation |
| REQ-facade-readme | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `src/README.md` (3,750 bytes) |
| REQ-stable-api-v020-sync | pending — plan 10-10 | run-4 input (not yet re-derived): **Shipped (relocated)** — applies to `docs/src/api-reference/stable-api.md` after the Milestone 11 overhaul → ARCH-05 |
| REQ-changelog-v020-cut | pending — plan 10-10 | run-4 input (not yet re-derived): **Shipped (history)** — v0.2.0 shipped and the tree is four minors past it → HARD-03 |
| REQ-api-surface-baseline-v020 | pending — plan 10-10 | run-4 input (not yet re-derived): **Open defect → DEBT-01** — regenerating the baseline depends on the `api-surface` job working, which it has not since commit `928c6d5` |
| REQ-m8-final-quality-gate | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — but its FR-19 `cargo doc` bar ("warnings acceptable") contradicts M7 Epic 4 §4.4.3 ("without documentation warnings") on the same command → HARD-07 |

### Milestone 8 Epic 6 — `paladin-content` Services Rename (4 IDs)

**Epic note:** *(reserved for plan 10-10)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-content-services-rename | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `crates/paladin-content/src/services/` exists and `lib.rs` declares `pub mod services;`, closing the broken bridge the Epic 6 DOC's Root Cause section describes |
| REQ-paladin-content-readme-update | pending — plan 10-10 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-paladin-content-changelog-fix | pending — plan 10-10 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-content-processing-build-gate | pending — plan 10-10 | run-4 input (not yet re-derived): **Shipped, narrowed → HARD-06** — the workspace builds under `content-processing`, but the facade flag enables five of six capability features and omits `pdf`, whose feature gates no dependency |

### Milestone 8 Epic 7 — `paladin-web` Single Framework (axum) (6 IDs)

**Epic note:** *(reserved for plan 10-10)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-delivery-endpoints-axum | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `crates/paladin-web/src/delivery_controller.rs` documents `POST /api/delivery/deliver`, `GET /api/delivery/status/{delivery_id}` and `GET /api/delivery/stats`; `app.rs:24` imports and `app.rs:63` merges `create_delivery_routes(deliverer)`, so they are **mounted**, not merely ported |
| REQ-actix-removal | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `grep -rn "actix" crates/paladin-web/` returns zero matches. Variant group 21 |
| REQ-actix-deny-ban | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — `deny.toml:99-103` bans `actix-web` with the reason "paladin-web standardizes on axum; no second web framework" |
| REQ-delivery-handler-tests | pending — plan 10-10 | run-4 input (not yet re-derived): Verify → HARD-01 |
| REQ-web-api-baseline-changelog | pending — plan 10-10 | run-4 input (not yet re-derived): **Open defect → DEBT-01** — FR-10 mandates `./scripts/extract-public-api.sh project/current-exports.txt`, the path that has been stale since commit `928c6d5`. **The defect is now written into a requirement as well as into the tooling**, so DEBT-01 must correct both. **Corrected (Phase 8, dated 2026-08-06):** FR-10 is now annotated in place at source (`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md`) with a dated D-00c banner and inline struck-and-corrected text naming `.project/current-exports.txt`; original text retained, nothing deleted → DEBT-01 |
| REQ-web-quality-gate | pending — plan 10-10 | run-4 input (not yet re-derived): Verify — the `web-server` feature-matrix entry and the change-confinement clause were not re-checked → HARD-01 |

### Cross-milestone entries carried by DOCs rather than PRDs (5 IDs)

**Epic note:** *(reserved for plan 10-10)*

| ID | Verdict | Evidence |
|---|---|---|
| REQ-storage-nonoptional-v2 | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — variant group 22 |
| REQ-m8-reconciliation-relocations | pending — plan 10-10 | run-4 input (not yet re-derived): Shipped — 15 commits, ~10,250 net LOC removed, one new leaf crate; every target confirmed in the tree → HARD-02 |
| REQ-m8-deferred-items-register | pending — plan 10-10 | run-4 input (not yet re-derived): **Open register → FACADE-01 (D5), FACADE-02 (D1-D4)** — D5's count is verified exact; D1's six `src/core/` files and D2's three manager services all still ship. No owners named, no target milestone assigned |
| REQ-deferred-cli-user-commands | pending — plan 10-10 | run-4 input (not yet re-derived): **Deferred with register → FACADE-03(a)** — `user.rs` verified absent from the ten CLI command modules; backend intact; recoverable verbatim from git history |
| REQ-deferred-tensorflow-ml-adapter-v3 | pending — plan 10-10 | run-4 input (not yet re-derived): **Deferred with register → FACADE-03(b)** — adapter and `ml` flag verified absent; the `paladin-ml` leaf-crate placement condition is the live artefact. Variant group 24 |
