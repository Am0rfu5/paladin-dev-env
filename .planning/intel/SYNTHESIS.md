# Synthesis Summary

Entry point for `gsd-roadmapper`. Produced by `gsd-doc-synthesizer`.

- **Ingest runs completed:** **5 of 5 — INGEST COMPLETE** (see the run-5 section and the corpus-level closing summary at the end of this file)
- **Run 1 source set:** `.project/Milestone_1-MVP` (36 docs), MODE=new
- **Run 2 source set:** `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion`
  (45 docs), MODE=merge
- **Run 3 source set:** `.project/Milestone_4-Refactor-Crates-Features` +
  `.project/Milestone_5-Workspace-Decomposition` +
  `.project/Milestone_6-Architectural-Refinements` (32 docs), MODE=merge
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides in any run)
- **Program note:** the ingest was regrouped from 14 milestone-runs into 5 larger runs; run
  numbering below reflects the 5-run program. Run-1 text elsewhere in `.planning/` may still
  say "run 1 of 14" — same run, renumbered program.

---

## Doc counts by type

| Type | Run 1 | Run 2 | Run 3 | Cumulative |
|---|---|---|---|---|
| PRD | 11 | 15 | 13 | 39 |
| DOC | 25 | 30 | 19 | 74 |
| ADR | 0 | 0 | 0 | 0 |
| SPEC | 0 | 0 | 0 | 0 |
| UNKNOWN | 0 | 0 | 0 | 0 |
| **Total** | **36** | **45** | **32** | **113** |

All 113 classifications consumed. Every one carried `manifest_override: true` and
`confidence: high`. Run-3 source volume read: ~530 KB across 32 files.

Run-3 classifications live in `/workspace/.planning/intel/classifications/run-03/`.

**Nine of the 19 run-3 DOCs are verbatim extracts** of two milestone-overview documents
(Milestone 5 Epics 2-5 and Milestone 6 Epics 1-4). They add no independent content and are
consolidated into two context topics rather than nine. Do not double-count their acceptance
criteria.

## Decisions

- Decisions extracted: **0** (run 1: 0, run 2: 0, run 3: 0)
- Decisions locked: **0**
- Source paths: none

There is still no ADR-typed document anywhere in the corpus after 113 files. `decisions.md`
is intentionally empty of entries. No LOCKED-vs-LOCKED hard block is possible and every
technical position in `requirements.md` remains overridable by any ADR arriving in runs 4-5.

**Run 3 contains the corpus's only decision record — and it is still not a locked decision.**
`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` plus its
`-options.md` companion are the only decision/options pair in all 263 documents. The decision
doc carries `Status: Approved`, `Decision Date: 2026-05-13`, `Chosen Option: Option A`, a
Rationale section and a Rejected Options section — it reads as a genuine ADR — but is
manifest-typed DOC with `locked: false`, so it sits at the lowest precedence tier.

What it settles: the **location** of five pure value/error types (`PaladinResult`,
`StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) moving into `paladin-core`, with
the application ports reduced to thin re-exports and `PaladinError` deliberately excluded.

What it does **not** settle: it never mentions `BattalionResult`. Despite the filename, the
run-1 `REQ-battalion-result-v1/-v2` field-set variant is untouched by it. That variant is
closed by **shipped code** instead — see `code-verification.md`.

**Two ADR candidates now exist:** this decision record and `Epic_17.5/epic17-5.md` (the
run-2 CLI-location decision). Promote both via `--manifest` if their positions should be
protected from future override. A PRD published two days after the decision record already
contradicts it — see WARNINGS.

## Requirements

- Requirements extracted: **348** cumulative (run 1: 115, run 2: 118, run 3: 115)
- Competing variants preserved unmerged: **38** cumulative (run 1: 12, run 2: 18, run 3: 8)

Run-3 requirements grouped by source PRD:

| Source PRD | Requirements |
|---|---|
| M4 Epic_1/prd-expand-feature-flags.md | 7 |
| M4 Epic_2/prd-harden-port-traits-stable-api.md | 9 |
| M4 Epic_3/prd-cli-isolation.md | 9 |
| M5 Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md | 9 (incl. 4 variants) |
| M5 Epic_2/prd-paladin-ports-extraction.md | 9 (incl. 1 variant) |
| M5 Epic_3/prd-paladin-battalion-extraction.md | 8 (incl. 1 variant) |
| M5 Epic_4/prd-paladin-llm-extraction.md | 11 (incl. 1 variant) |
| M5 Epic_5/prd-paladin-memory-extraction.md | 9 (incl. 1 variant) |
| M5 Epic_6/prd-workspace-finalization-epic-6.md | 6 |
| M6 Epic_1/prd-decompose-application-settings.md | 8 (incl. 1 variant) |
| M6 Epic_2/prd-relocate-orchestration-services.md | 9 |
| M6 Epic_3/prd-co-locate-maneuver-dsl.md | 9 |
| M6 Epic_4/prd-relocate-circuitbreaker-infra.md | 8 |

Run-3 variant sets requiring user resolution before routing (4 pairs):
`REQ-workspace-crate-edition-v1/-v2` (Rust edition 2021 versus 2024),
`REQ-paladin-core-dependency-allowlist-v1/-v2` (exhaustive 6-crate list versus petgraph),
`REQ-port-value-type-ownership-v1/-v2` (decision record versus Epic 2 PRD),
`REQ-llm-config-bridge-location-v1/-v2` (root-crate bridge versus paladin-llm config module).

**All four are settled by shipped code**, which is unusual for this corpus — see
`code-verification.md`. The user still needs to decide whether to amend the documents or
accept the code as the resolution, because three of the four PRDs are unamended and their
rules (notably Epic 2's FR-10 "types must not be split across crates") would produce the
wrong answer if applied literally to future work.

Run-3 entries that supersede earlier requirements without deleting them:
`REQ-maneuver-files-moved-from-core` supersedes the Maneuver-parser inclusion in
`REQ-core-container-extraction`; `REQ-circuitbreaker-relocation` supersedes every run-1/run-2
`src/application/use_cases/paladin/circuit_breaker.rs` path reference; the whole M5 crate set
supersedes every `src/{core,application,infrastructure}` path recorded in runs 1-2.

## Constraints

- Constraints extracted: **0**
- Type breakdown: api-contract 0, schema 0, nfr 0, protocol 0

No SPEC-typed documents exist in any run. **Run 3 is by far the most constraint-dense set** —
these three milestones are almost entirely build-system contracts, dependency layering and
module boundaries rather than features. `constraints.md` inventories what would become
constraints if the carriers were re-tagged: the full 25-port extraction inventory, every
per-crate `[features]` table, the three dependency allowlists (6 / 7 / 14-permitted-9-forbidden),
the workspace `Cargo.toml` template, the `config.yml` deserialization contract, ~20 numeric
build/coverage/file-size targets, and the `#[cfg]` guard and import-migration protocols.

The strongest re-tag candidates are the **dependency allowlists** and the **`config.yml`
deserialization contract**, because both are already contradicted by shipped code.

## Context

- Context topics recorded: **73** cumulative (run 1: 31, run 2: 28, run 3: 14)
- Run-3 groups: the origin analysis document (1), the three milestone plans (3), the four
  Milestone-4 analysis artifacts (4: dependency matrix, api audit, deprecations, plus the M4
  plan), the two `decisions/` files (2), the build-benchmark report (1), the Epic 2 dependency
  analysis (1), the two consolidated per-epic-DOC topics (2), and the code-verification anchors
  topic (1)

Load-bearing context for planning:

1. **Run 3 is the first run where checkbox counts are corroborated by code, not contradicted.**
   Milestone 4's 20 open items (all in `tasks-harden-port-traits-stable-api.md`) are real:
   zero `#[deprecated]` annotations exist in the tree. Milestone 6's 0 open items are real:
   all four relocations are verifiably complete. Milestone 5's 17 open items are largely
   contradicted — all crates, the prelude, the CI isolation job and the benchmark report ship.
2. **The workspace is bigger than run 3 describes.** Ten library crates ship, not the six the
   M5/M6 overviews and `build-benchmarks.md` assume. `paladin-herald`, `paladin-storage`,
   `paladin-content`, `paladin-notifications` and `paladin-web` came from later milestones.
3. **Milestone/tier numbering collides.** The M4 overview is titled "Milestone 1", the M5
   overview "Milestone 2", and PRDs cross-reference "Milestone 1 / Epic 2" meaning
   Milestone 4 Epic 2. Same defect class as the run-2 Milestone 3 epic-numbering conflict.
4. **Five documented positions are contradicted by shipped code** and must not be planned
   as-is: edition 2021, the "exhaustive" 6-dependency `paladin-core` allowlist, `vision`
   gating the encryption crates, the MCP transport feature flags, and the `paladin-cli` crate.
5. **STABLE_API.md and the M4 doc deliverables are relocated, not missing** — they ship as
   mdbook pages under `docs/src/api-reference/` after the Milestone 11 overhaul. Do not plan
   them as gaps.
6. **Five verified open defects** (see WARNINGS 8-13): `paladin-ports` doctests disabled with a
   named "Task 7.0" follow-up; the `api-surface` CI job broken by a stale
   `project/current-exports.txt` path after the `.project` rename; zero `#[deprecated]`
   annotations; three CLI-only dependencies still unconditional; three competing `TokenUsage`
   definitions. All are small, concrete and high-value.

## Conflicts

- **Blockers: 0** (run 1: 0, run 2: 0, run 3: 0)
- **Competing variants (warnings): 39** (run 1: 8, run 2: 18, run 3: 13)
- **Auto-resolved / informational: 67** (run 1: 11, run 2: 28, run 3: 28)

Run-3 warnings in descending planning impact:

1. Milestone numbers and refactoring-tier numbers collide across all three milestones
2. Competing Rust edition for the workspace crates (2021 versus 2024; tree is mixed)
3. `paladin-core`'s dependency allowlist is declared exhaustive and then contradicted
4. Competing ownership of `PaladinResult` / `StopReason` / `TokenUsage` (decision record
   versus Epic 2 PRD — precedence hands the win to the architecturally wrong answer)
5. Competing location for the LLM configuration bridge
6. `vision` feature scope contradicts the dependency audit and would break the build
7. Facade re-export policy for the M6 relocations is unsettled and self-contradictory
8. `paladin-ports` doctests disabled in shipped code against an explicit PRD requirement
9. The `api-surface` CI job is broken by a stale baseline path
10. Deprecation annotations required by M4 Epic 2 do not exist anywhere in the tree
11. Three competing `TokenUsage` definitions ship simultaneously
12. CLI dependency isolation is incomplete while its task list shows no open items
13. The build-benchmark report's verdict contradicts its own measurements

**Run 3 closed five earlier warnings using shipped code** — see INFO entries 17-21 and
`code-verification.md`: the run-1 `BattalionResult` variant (merged superset shipped), the
run-1 `BattalionConfig` variant (Epic 4 form shipped), the run-2 `metadata_output_dir`
three-owner conflict (one owner; `CommanderConfig` never built), the run-2 competing
`ErrorStrategy` variant sets (two distinct enums in two crates), and the run-1 Battalion base
module path. Net open variant count is therefore lower than the raw arithmetic suggests.

Cycle detection: run-3 cross-ref graph is acyclic (32 nodes, 3 in-set edges, max depth 2,
cap 50). Runs 1 and 2 were also acyclic.

Security note (carried from run 2): one ingested DOC contains a plaintext OpenAI API key. The
value was not copied into any intel or report file. The user has confirmed it is rotated.

## Files

- Conflicts report: `/workspace/.planning/INGEST-CONFLICTS.md` (cumulative, runs 1-3;
  0 blockers, 39 warnings, 67 info)
- Decisions: `/workspace/.planning/intel/decisions.md` (no entries; run-3 section explains the
  one Approved-status decision record and why it is not locked)
- Requirements: `/workspace/.planning/intel/requirements.md` (348 entries)
- Constraints: `/workspace/.planning/intel/constraints.md` (no entries; run-3 inventory is the
  largest so far)
- Context: `/workspace/.planning/intel/context.md` (73 topics)
- **Direct code verification: `/workspace/.planning/intel/code-verification.md`** — HIGHEST
  precedence. The run-3 section is substantial: 22 verified-shipped claims, 6 resolved variants,
  6 verified-open defects, and a 12-row table of requirement text contradicted by shipped code
- Deterministic completion state: `/workspace/.planning/intel/task-completion-state.md`
- Committed codebase map: `/workspace/.planning/codebase/*.md`
- Classifications: `/workspace/.planning/intel/classifications/` (run 1),
  `classifications/run-02/` (run 2), `classifications/run-03/` (run 3)

## Status

**AWAITING USER** — 0 blockers, but 39 competing variants across runs 1-3 need a decision
before routing. Supersession is expected in this corpus and none of the variants was resolved
by the synthesizer; each is preserved unmerged with the later position noted and, where shipped
code settles it, a pointer to `code-verification.md`.

Three run-3 warnings gate roadmap structure rather than content and should be settled first:

1. **The milestone/tier numbering collision (warning 1)** because milestone numbers are
   provenance keys across 9 of the 19 run-3 DOCs, and this is the second numbering conflict in
   the corpus.
2. **The `PaladinResult` ownership conflict (warning 4)** because it is the one place where
   mechanical precedence produces an architecturally wrong answer — a PRD overriding an
   Approved decision record — and it determines whether the Epic-1 upward-dependency fix
   survives future work.
3. **The M6 facade re-export policy (warning 7)** because it decides whether Milestone 6 was a
   non-breaking internal refactor or a breaking change requiring a major version bump.

Five verified open defects (warnings 8-12) are the clearest forward-work candidates in the
corpus so far. Unlike the run-1/run-2 open-checkbox counts, each is confirmed against the tree
and each has a concrete, small fix.

## Roadmapper notes

- Honour the Roadmap Extension Protocol at the end of `ROADMAP.md`: **new phases start at
  Phase 7**, Phases 1-6 are never renumbered, the `### Phase N:` header format is preserved,
  `REQ-*` IDs are the merge keys, and later positions supersede rather than silently editing
  earlier ones. The `VERIFY-*` and `CLOSE-*` task prefixes are already spent — use new prefixes.
- Do not re-plan completed work. M4 is 93.2% complete (20 open, all in
  `tasks-harden-port-traits-stable-api.md` and all corroborated by code), M5 is 96.4% (17 open,
  mostly contradicted by code), M6 is 100% (0 open, confirmed by code). The 9-crate workspace
  demonstrably shipped — 10 library crates plus `doc-examples` exist.
- Treat every open checkbox count as a claim to verify against `code-verification.md` and
  `.planning/codebase/` first. Run 3 is the first run where a count proved trustworthy.
- Record supersession chains; do not raise relocation as contradiction. Every `src/…` path in
  runs 1-2 and several in run 3 are historical.
- Runs 4-5 will cover Milestones 7-12, Deferred-QA-CICD-Completion and project-management, and
  will merge into these same intel files.

---

# Ingest run 4 of 5 — synthesis update

- **Ingest runs completed:** 4 of 5
- **Run 4 source set:** `.project/Milestone_7-Production-Hardening` +
  `.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs), MODE=merge
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides in run 4 either)
- **Classifications:** `/workspace/.planning/intel/classifications/run-04/`
- **Cross-ref cycle detection:** run over the `cross_refs` graph. The graph is a DAG, maximum
  depth 3, no cycles. Longest chain: `deferred-items.md` → `facade-cleanup-RECONCILIATION-2026-06-04.md`
  → `Epic_1/facade-audit.md`. Most `cross_refs` entries are source-file or artifact paths rather
  than document paths, so the document-to-document graph is small. No traversal cap approached.

## Doc counts by type

| Type | Run 1 | Run 2 | Run 3 | Run 4 | Cumulative |
|---|---|---|---|---|---|
| PRD | 11 | 15 | 13 | 11 | 50 |
| DOC | 25 | 30 | 19 | 29 | 103 |
| ADR | 0 | 0 | 0 | 0 | 0 |
| SPEC | 0 | 0 | 0 | 0 | 0 |
| UNKNOWN | 0 | 0 | 0 | 0 | 0 |
| **Total** | **36** | **45** | **32** | **40** | **153** |

All 40 run-4 classifications consumed. Every one carried `manifest_override: true` and
`confidence: high`. Run-4 source volume read: ~8,080 lines across the 40 documents (the 10
`tasks-*.md` files in the same directories are excluded from ingest and covered by
`task-completion-state.md`).

**Nine of the 29 run-4 DOCs are verbatim extracts** of the two milestone-overview documents
(Milestone 7 Epics 1-4 and Milestone 8 Epics 1-5). They add no independent content and are
consolidated into two context topics. Milestone 8 Epics 6 and 7 are originals with no overview
counterpart — they were added after the milestone plan was written. Do not double-count the
extracts' acceptance criteria.

## Decisions

- Decisions extracted in run 4: **0**. Cumulative: **0**.
- Decisions locked: **0**. Cumulative: **0**.
- Source paths: none.

Run 4 confirms the standing constraint across the full corpus: **0 ADR and 0 SPEC across all 153
documents**, so no LOCKED-vs-LOCKED hard block is possible and nothing in `requirements.md` is
protected from a future ADR.

Run 4 is nonetheless the densest run for decision-shaped content, adding **four ADR candidates**
(six now exist corpus-wide):

- `Epic_1/cost-benefit-assessment.md` — a go/defer record with a "Self-Approval (Task 1.6)" block,
  a named approver and an approval date; its governing PRD calls it "the authoritative source of
  record for *why* a decision was made".
- `Epic_4/rustsec-remediation-plan.md` — a formal risk acceptance with owner **Platform Security**
  and **review/expiry target 2026-09-30**. The **only item in the entire corpus carrying an expiry
  date**, and the only one where leaving it untagged has an ongoing operational cost.
- `Epic_4/license-compatibility-decision-checklist.md` — a licensing policy with approver `DF3NDR`
  and approval date 2026-05-28.
- `facade-cleanup-RECONCILIATION-2026-06-04.md` — an explicit supersession notice that reverses a
  prior Epic and resolves six open decisions in execution.

Promoting any candidate requires re-tagging via `--manifest` and re-running ingest. See
`decisions.md` for the full list and reasoning.

## Requirements

- Requirements extracted in run 4: **86**. Cumulative: **434**.
- No duplicate IDs across the four runs.
- Eight new variant pairs/chains preserved unmerged, per the standing constraint:
  - `REQ-paladin-web-extraction` ↔ `REQ-actix-removal` (actix-web required vs. banned)
  - `REQ-storage-feature-flags-v1` ↔ `REQ-storage-nonoptional-v2` (`storage-sqlite` optional vs.
    retired)
  - `REQ-ci-publish-dry-run-v1` ↔ `REQ-ci-publish-dry-run-v2` (per-crate ordered vs. workspace-wide)
  - `REQ-tensorflow-stays-facade-v1` → `REQ-tensorflow-ml-feature-gate-v2` →
    `REQ-deferred-tensorflow-ml-adapter-v3` (a three-step chain ending in removal)
  - `REQ-m8-epic3-no-extractions` ↔ `REQ-m8-reconciliation-relocations` (defer to M9 vs. execute now)
  - `REQ-adapter-disposition-record` internal split on `arsenal/` and `sanctum/` M9 targets
  - `REQ-dead-file-batch-deletion` ↔ the M8 overview's Epic 3 Task 3.1 (delete vs. move the three
    notification channel services)
  - `REQ-paladin-storage-extraction` three-way on `file_content_repository.rs` (stays / move /
    delete)
- Where the shipped tree settles a variant, the entry carries a `- settled-by:` line pointing at
  `code-verification.md`. That records a **fact about the tree**, not a decision taken here.
- Five entries derive from DOCs rather than PRDs because the DOC is the only carrier of substantive
  forward-work content: `REQ-m8-deferred-items-register`, `REQ-deferred-cli-user-commands`,
  `REQ-deferred-tensorflow-ml-adapter-v3`, `REQ-rustsec-risk-acceptance`,
  `REQ-license-policy-signoff`.

## Constraints

- Constraints extracted in run 4: **0**. Cumulative: **0** (0 SPEC-typed docs corpus-wide).
- Constraint-shaped material is listed in `constraints.md` for coverage. The three strongest SPEC
  candidates in run 4: the **crate dependency-direction invariant** (M7 Epic 1 §6.1, currently
  violated by `paladin-content` → `paladin-llm`), the **RustSec exception list with its expiry**,
  and the **three delivery endpoint contracts** (M8 Epic 7 §4 — the only genuine api-contract
  material in the run).

## Context

- Context topics added in run 4: **14**. Topics cover the Milestone 7 and 8 overviews, the Epic
  definition extracts, the cost-benefit assessment, the benchmark assessment, the v0.1.0-rc.1
  release outcome, the RustSec acceptance, the license policy, the facade audit, the adapter
  disposition record, the 2026-06-04 reconciliation, the two deferred registers, the
  Milestones 8-11 dependency graph, the version trajectory, and code-verification anchors.

## Conflicts

- Run 4 added: **0 blockers, 14 competing-variant warnings, 26 auto-resolved/informational**.
- Cumulative: **0 blockers, 53 warnings, 93 info**.
- Runs 1-3 entries preserved verbatim; only the two section-count headers changed.
- No UNKNOWN classifications and no low-confidence classifications in run 4, so no type-tagging
  blockers. No cross-ref cycles, so no cycle blockers. No locked decisions, so no LOCKED-vs-LOCKED
  blockers. **The corpus remains unblocked after four of five runs.**

**The highest-priority run-4 warning is the RustSec exception drift.** The ingested plan formally
risk-accepts two advisories with an owner and a **2026-09-30 expiry**; the tree now carries five
vulnerability ignores in `.cargo/audit.toml`, only two of them mirrored into `deny.toml` (whose own
comment claims they are in sync), and `ci.yml:406` passes only the original two on the command line
while `make audit` reads all five from `audit.toml`. Three files, three exception sets, on a
repository that gates CI on `cargo audit` and cargo-deny. This is live security governance, not a
documentation nit.

Full detail: `/workspace/.planning/INGEST-CONFLICTS.md`.

## Code verification

`code-verification.md` was **appended to**, not overwritten, per the run-4 instruction. The run-4
section records:

- **Verified SHIPPED (18 rows)** — the `paladin-herald` crate; `FileCitadel` in `paladin-memory`;
  MinIO/S3 and Redis in `paladin-storage`; non-optional storage with `storage-sqlite` retired; all
  25 List A deletions and their orphaned directories; `src/core/` at exactly six files; the
  `use_cases` → `services` rename in **both** the facade and `paladin-content`; actix-web fully
  removed and banned with three mounted axum delivery routes; the TensorFlow adapter, `ml` feature
  and CLI `user` command all gone; `src/README.md`; all five benchmarks in their owning crates with
  zero disabled files; all ten per-crate Makefile targets; the `Dockerfile.chef` workspace
  adaptation; the crates.io package renames.
- **Verified OPEN (6 items)** — the RustSec exception drift; `paladin-herald` missing a
  `CHANGELOG.md`; the stale `Dockerfile.chef` planner COPY list; the still-broken `api-surface`
  baseline path (now re-asserted by M8 Epic 7 FR-10); `paladin-ports` doctests still disabled; and
  the crates.io collision guardrail follow-up.
- **Superseded by outcome (14 rows)** — requirements that must not be planned as written, including
  the actix-web clause, the `storage-sqlite` flag, the per-crate publish order, the `ml` gate, the
  Epic 3 no-extraction mandate, the `160` facade file count, and the root-path `STABLE_API.md` and
  `docs/*.md` deliverables that were relocated into the mdbook.
- **Contradicted in the favourable direction (2)** — Milestone 8 Epics 3 and 6 are both complete in
  the tree despite being recorded as punted and unverified respectively.

`deferred-items.md` D5's count matches the tree **exactly** — 17 occurrences across 6 files — as do
every other verifiable claim in the two deferred registers. That is the strongest reliability signal
in the corpus and the reason those registers, not checkbox arithmetic, are the Milestone 8
forward-work source.

## Status

**STATUS: AWAITING USER — 14 new competing variants need resolution. 0 blockers.**

Safe to route once the warnings are reviewed. Nothing in run 4 gates the workflow.

## Roadmapper notes for run 4

- **Honour the Roadmap Extension Protocol** at the end of `ROADMAP.md`. New phases start at
  **Phase 9** — Phases 1-8 are never renumbered and the `### Phase N: Name` header format is
  parsed by downstream tooling, including inside `<details>` blocks.
- **Requirement-ID prefixes already spent:** `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` (Milestone 1);
  `VERIFY-*`, `CLOSE-*` (Milestones 2-3); `ARCH-*`, `DEBT-*` (Milestones 4-6). **Use fresh prefixes
  for run 4.** Suggested and unused: `HARD-*` (production hardening / build and release
  infrastructure), `FACADE-*` (facade cleanup and shim resolution), `SEC-*` (the RustSec and
  license governance items), `DEFER-*` (the D1-D5 register and the two deferred features).
  Ingested `REQ-*` IDs remain the stable merge keys — match on them rather than re-deriving.
- **Do not re-plan completed work.** Milestone 7 is 98.8% complete (3 open, all in Epic 2) and
  Milestone 8 is 99.1% (3 open). Both counts were treated as claims and verified: **Milestone 8's
  three are contradicted** — Epics 2 and 3 are both verifiably complete and Epic 3 went further
  than its own task list scoped. Milestone 7's three are plausible; the genuine Epic 2 residue is
  the stale `Dockerfile.chef` COPY list and the broken `api-surface` baseline path.
- **The genuine forward work from run 4 is small and concrete**, and it is not in the checkbox
  arithmetic:
  1. Reconcile the RustSec exception set across `.cargo/audit.toml`, `deny.toml` and `ci.yml:406`,
     and decide the disposition of the **2026-09-30** expiry. Highest priority.
  2. The five deferred items D1-D5, with the document's own suggested grouping — quick win D5
     (println residue); architecture pass D2 (mis-layered manager services) plus optionally D4
     (`content_ingestion_service.rs` placement); and D1/D3 only alongside a broader refactor.
  3. The two deferred features, each with a stated reintroduction condition — the `paladin user`
     CLI surface ("mostly re-wiring, not new domain work") and the ML adapter, which **must** be
     rebuilt in a dedicated `paladin-ml` leaf crate rather than returned to the facade.
  4. Four small verified defects: the missing `paladin-herald` CHANGELOG, the stale
     `Dockerfile.chef` COPY line, the `api-surface` baseline path (five references, unchanged since
     run 3 and now written into a run-4 requirement), and the `paladin-ports` doctest re-enable.
- **Two open architecture questions worth surfacing rather than assuming:** whether the
  extracted-crate dependency rule permits optional feature-gated edges between leaf crates (it is
  currently stated absolutely and violated once), and whether PDF extraction is still a supported
  capability (`paladin-content`'s `pdf` feature gates nothing and the facade does not enable it,
  yet `.cargo/audit.toml` treats `pdf-extract` as live in the graph).
- **Record version history, do not act on it.** The M7 Epic 4 documents describe a `v0.1.0-rc.1`
  release at lockstep `0.1.0`; the tree is at `0.6.0` on `release/v0.7.0` with latest tag `v0.5.1`.
  Every rc.1 artifact is history.
- **Preserve the Milestones 8-11 dependency graph for run 5.** M9 hard-depends on M8 Epic 4;
  M11 hard-depends on M8 and partially on M9 (Epics 3-4 wait for M9 Epics 1-3); M10 has no hard
  dependency. Critical path M8 → M9 → M11 Epics 3-5 = 11-17 sprints. Per
  `task-completion-state.md`, M9 and M10 are 100% complete and M11 is 92%, so run 5 will be
  attaching requirements to milestones that have largely shipped.
- **Expect a fourth milestone-numbering collision.** The Milestone 7 overview titles itself
  "Milestone 4"; directory numbering is authoritative, as in the two prior instances.
- Run 5 covers Milestones 9-12, `Deferred-QA-CICD-Completion` and `project-management`, and will
  merge into these same intel files.

---

# Ingest run 5 of 5 — synthesis update, and corpus-level close

- **Ingest runs completed: 5 of 5. THE INGEST IS COMPLETE.**
- **Run 5 source set:** `.project/Milestone_9-Classic-Orchestrator-Completion` +
  `.project/Milestone_10-CI-Hardening-Release-Automation` +
  `.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
  `.project/Deferred-QA-CICD-Completion` + `.project/project-management` (46 docs), MODE=merge
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides in any of the five runs)
- **Classifications:** `/workspace/.planning/intel/classifications/run-05/`
- **Cross-ref cycle detection:** run over the `cross_refs` graph. **Acyclic** — 46 nodes, 8 in-set
  document-to-document edges, maximum depth 2, cap 50 never approached. Most `cross_refs` entries are
  source-file or artifact paths rather than document paths, so the document-to-document graph is
  small. Longest chain: `Epic-6_Deployment-Topologies-Documentation.md` →
  `prd-deployment-topologies-documentation.md`. No cycle blockers.

## Doc counts by type

| Type | Run 1 | Run 2 | Run 3 | Run 4 | Run 5 | Cumulative |
|---|---|---|---|---|---|---|
| PRD | 11 | 15 | 13 | 11 | 25 | **75** |
| DOC | 25 | 30 | 19 | 29 | 21 | **124** |
| ADR | 0 | 0 | 0 | 0 | 0 | **0** |
| SPEC | 0 | 0 | 0 | 0 | 0 | **0** |
| UNKNOWN | 0 | 0 | 0 | 0 | 0 | **0** |
| **Total** | **36** | **45** | **32** | **40** | **46** | **199** |

All 46 run-5 classifications consumed. Every one carries `manifest_override: true` and
`confidence: high`. **Run 5 is the most PRD-dense run in the corpus** — 25 of 46 documents, versus
11-15 in every prior run — because Milestones 9 and 12 generate one PRD per epic.

**Six of the 21 run-5 DOCs are near-verbatim extracts** of the Milestone 9 overview
(`Milestone_9-Epic_1` … `Epic_6`), and four more extract the Milestone 10 and 11 overviews. They add
no independent content and are consolidated into milestone-level context topics. Do not
double-count their acceptance criteria.

### Corpus coverage reconciles exactly

`.project/` holds **263** `.md` files: **188** prose/planning documents and **75** `tasks-*.md`
checklists. The five runs produced **199** classification records with **199 distinct
`source_path`s** and **zero duplicates**, comprising all 188 prose documents plus **11** task lists
that earlier manifests included. The remaining **64** task lists are covered deterministically by
`intel/task-completion-state.md`, which counts literal GFM checkboxes rather than relying on an LLM
classifier. 188 + 75 = 263 and 188 + 11 = 199. **The ingest covers the corpus completely, by one
route or the other.**

## Decisions

- Decisions extracted in run 5: **0**. Cumulative: **0**.
- Decisions locked: **0**. Cumulative: **0**.
- Source paths: none.

**The final corpus position: 0 ADR and 0 SPEC across all 199 classified documents.** No
LOCKED-vs-LOCKED contradiction was ever possible, in any run. Nothing in the 554 requirement entries
is protected from override by a future ADR.

Run 5 adds **five ADR candidates**, bringing the corpus total to **eleven**. The most valuable are:

- **`M9/Epic_5/prd-user-admin-system-completion.md` §6.1** — the opaque-bearer-token decision, with
  rationale, a rejected alternative and a recorded trade-off. **The only decision in the corpus that
  a later milestone contradicts in prose while silently preserving in code.**
- **`M10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8** — the audit-suppression
  single-source invariant, **currently violated in the tree**.
- **`M9/Epic_4/prd-agent-orchestrator-bridge.md` §6.1** — the cleanest ADR-shaped section anywhere in
  the corpus: a four-criterion comparison table, a `(CHOSEN)` column header, an explicit decision, and
  the rejected option preserved as a future non-breaking enhancement.

**The two candidates with a live operational cost are `rustsec-remediation-plan.md` (run 4) and
`prd-dependency-security-license-compliance.md` (run 5), and they are the same subject.** One carries
the corpus's only expiry date — **2026-09-30** — and the other carries the invariant the tree
currently violates. Promoting them together via `--manifest` would turn the run-5 supply-chain
finding from an observation into a gate. See `decisions.md`.

## Requirements

- Requirements extracted in run 5: **120**. Cumulative: **554**.
- No duplicate IDs across the five runs.
- Two new competing variant pairs preserved unmerged, plus fourteen other warnings needing resolution:
  - `REQ-opaque-bearer-token-adapter-v1` ↔ `REQ-jwt-bearer-auth-v2` — opaque bearer tokens versus
    JWT. **Neither variant wins cleanly in the tree**: the shipped `agent_auth.rs` has v2's shape over
    v1's mechanism, with no `jsonwebtoken` dependency anywhere in the workspace.
  - The Deferred-QA coverage threshold — a **78% hard gate** (parent PRD FR-25.3) versus a **phased
    70 → 74 → 78 ramp** (Epic 25 FR-25.6). The parent PRD's own OQ-3 asks this and is recorded Open.
- Run-5 requirements by source milestone: Milestone 9 → 23; Milestone 10 → 21; Milestone 11 → 18;
  Milestone 12 → 32; Deferred-QA → 15; project-management → 1.
- **Sixteen entries carry `- settled-by:` lines** pointing at `code-verification.md` run 5. Those
  record facts about the tree, not decisions taken here.

## Constraints

- Constraints extracted in run 5: **0**. Cumulative: **0** (0 SPEC-typed docs corpus-wide).
- **Run 5 is the most api-contract-dense run in the corpus** — the first with a genuine HTTP surface
  carrying status codes, headers, envelopes, security schemes and a machine-checked specification.
  `constraints.md` inventories what would bind if the carriers were re-tagged.
- The three strongest SPEC candidates in run 5: **the `paladin-web` dependency-flow invariant**
  (stated three times across two Epics, with a mechanical verification command —
  `cargo tree -p paladin-web` must show no facade dependency); **the audit-suppression single-source
  invariant** (the one constraint in run 5 the tree violates); and **the authentication /
  authorization / fail-closed / redaction contract** in M12 Epic 5 §4.

## Context

- Context topics added in run 5: **16**. Cumulative: **103**.
- Topics cover the Milestone 9 epic plan, the bidirectional agent/orchestrator bridge, Milestone 10's
  fifth epic and the incident behind it, the supply-chain governance surface, the mdbook relocation as
  a false-positive generator, how Milestone 11 Epic 6 created Milestone 12, the Milestone 12 epic
  architecture, the JWT-versus-opaque-token divergence, the Deferred-QA register, a comparison of all
  three deferred registers, run-5 checkbox analysis, the completed version trajectory, the
  `project/current-exports.txt` debt, what Milestone 12 explicitly did not build, the master project
  plan, and code-verification anchors.

## Conflicts

- Run 5 added: **0 blockers, 16 competing-variant warnings, 19 auto-resolved/informational**.
- **Cumulative: 0 blockers, 69 warnings, 112 info.**
- Runs 1-4 entries preserved verbatim — verified by diff: only the two section-count headers changed.

**The three highest-priority run-5 findings, in order:**

1. **A duplicate `cargo audit` CI job falsifies a Milestone 10 success metric.** `ci.yml` has two jobs
   with the identical display name "Security Audit". `:60-77` complies with FR-1 exactly; `:390-406`
   still runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`, covering 2 of the
   5 advisories in `.cargo/audit.toml`. Milestone 10 Epic 2 added the compliant job without deleting
   its predecessor; Epic 4's non-goals then froze the area. **Milestone 10 is recorded 100% complete
   and one of its own acceptance criteria is false.** The fix is deleting 18 lines.
2. **Deferred-QA Epics 25-27 are essentially unbuilt** — verified item by item against the tree. No
   `cli-tests`, `bench-check` or `coverage` job; no `.codecov.yml`; no Makefile coverage targets;
   eight deprecated-action references; the architecture document frozen at exactly 311 lines with
   zero of seven newer subsystems and zero Mermaid diagrams; `docs/assets/` empty; no `docs/DEMOS.md`;
   no `tools` field, `ToolDefinition` or `ToolCall` anywhere; no `tests/common/`.
3. **The agent API is documented as JWT and implemented as opaque tokens**, and Milestone 12 ships a
   Kubernetes Deployment against an in-process token store whose own originating PRD warned that
   "a multi-process deployment would later need a shared store".

**Run 5 also corrects a run-4 finding.** `deny.toml` **does** now mirror all five vulnerability
advisories from `.cargo/audit.toml`; run 4's "the three 2026 advisories are absent" is no longer
true. The two files are in sync. The remaining governance gap is that **13 of the 15 `deny.toml`
suppressions carry documented reasoning but no owner and no expiry** — only the original two have a
formal risk-acceptance record, in `rustsec-remediation-plan.md`. Ten of the thirteen (the unmaintained
notices) are sanctioned by M10 Epic 4 FR-1 step 5; the three additional **vulnerability** ignores are
not authorised by any ingested document.

Full detail: `/workspace/.planning/INGEST-CONFLICTS.md`.

## Code verification

`code-verification.md` was **appended to**, not overwritten, per the run-5 instruction. The run-5
section records:

- **Verified SHIPPED (37 rows)** — the whole Milestone 9 orchestrator subsystem; the whole Milestone
  10 tooling set including textbook FR-14(b) licence-exception compliance (eight per-crate MPL-2.0
  exceptions rather than weakening the allow-list); the Milestone 11 mdbook with linkcheck at
  `warning-policy = "error"` and all six deployment-topology pages; and the entire Milestone 12 web
  API — registry, controller, auth, error envelope, health/ready, layers, job store, OpenAPI +
  committed drift baseline, `paladin-server`, `Dockerfile.server`, `k8s/`, and version 0.6.0.
- **One CORRECTION to run 4** — `deny.toml` is in sync.
- **Verified OPEN (8 findings)** — the duplicate `security` job; the owner/expiry gap on 13
  suppressions; Epic 25 unimplemented bar one item; Epic 26's architecture document frozen by the
  appendix rewrite exemption; Epic 27 entirely absent with `ProviderCapabilities` over-reporting;
  the Epic 28/29 mock prerequisite missing; the JWT-versus-opaque divergence and its multi-replica
  consequence; and `project/current-exports.txt` at nine stale references.
- **Checkbox analysis** — M9's 0 open corroborated; M10's 0 open corroborated in artefacts but
  contradicted in one acceptance criterion; **M11's 26 open is the only genuinely open count in run 5**
  and cannot be settled by file existence; **M12's 3 open are all Task 0.0 feature-branch scaffolding
  while the Epic 5 code ships**; **project-management's 1 open is a formatting example inside a
  template.**

---

# CORPUS-LEVEL CLOSING SUMMARY

## The numbers

| Measure | Value |
|---|---|
| Documents in `.project/` | **263** (188 prose + 75 `tasks-*.md`) |
| Documents classified across 5 runs | **199** (188 prose + 11 task lists) |
| Task lists covered deterministically instead | **64** |
| PRD / DOC / **ADR** / **SPEC** / UNKNOWN | 75 / 124 / **0** / **0** / 0 |
| Requirements extracted | **554** |
| Decisions extracted | **0** |
| **Decisions locked** | **0** |
| Constraints extracted | **0** |
| Context topics | **103** |
| Blockers | **0** |
| Competing variants / warnings | **69** |
| Informational entries | **112** |
| ADR candidates identified | **11** |
| Open checkboxes recorded | 542 across 75 task lists (93.3%) |
| Cross-ref cycles found | **0**, in all five runs |

## The milestone arc, M1 → M12

- **M1 — MVP.** Paladin, Garrison, Arsenal, Battalion (Formation, Phalanx, Campaign, Chain of
  Command), Herald, Citadel, Commander, Armory CLI. 97.9% complete. Run 1.
- **M2 — Missing features.** Sanctum vector memory + RAG, Sentinel vision, autonomous agents,
  Conclave, Council, Grove, Maneuver/Flow DSL, CLI enhancement. 86.2% by checkbox — **but Conclave
  (129 open) and Sanctum (111 open) are both verifiably shipped.** Run 2. Originated by
  `project-management/paladin-project-plan-final.md`, ingested in run 5.
- **M3 — Completion.** Herald and domain-type consolidation, vision pipeline, autonomous-agent
  completion, Battalion/Commander hardening, CLI/config/infrastructure, test hardening. 90.0%. Run 2.
- **M4 — Refactor: crates and features.** Feature-flag expansion, port-trait hardening and stable API,
  CLI isolation. 93.2% — **the first checkbox count verified accurate.** Run 3.
- **M5 — Workspace decomposition.** The monolith becomes a Cargo workspace: `paladin-core`,
  `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, plus the prelude and CI
  isolation. 96.4%. Run 3.
- **M6 — Architectural refinements.** Config decomposition, orchestration-service relocation, Maneuver
  DSL co-location, `CircuitBreaker` relocation. **100.0%, fully corroborated by code.** Run 3.
- **M7 — Production hardening.** `paladin-storage`, `paladin-notifications`, `paladin-content`,
  `paladin-web` extracted; Docker/cargo-chef adaptation; benchmark migration; API stabilization and
  the first crates.io publish. 98.8%. Run 4.
- **M8 — Facade cleanup and shim resolution.** 25 dead files deleted, `src/core/` reduced to six
  files, `use_cases` → `services` renamed workspace-wide, actix-web removed and banned,
  `paladin-herald` created. 99.1% — **all three open items contradicted by code.** Run 4.
- **M9 — Classic orchestrator completion.** `execute_workflow()`, `TaskService` dispatch,
  `WorkflowRepository` + SQLite adapter with crash recovery, scheduler/queue/event validation, the
  bidirectional content↔agent bridge, user/admin RBAC. **100.0%, corroborated.** v0.3.0. Run 5.
- **M10 — CI hardening and release automation.** pre-commit/pre-push hooks, cargo-audit + cargo-deny +
  OSV-Scanner, CycloneDX SBOM, cargo-release with dependency-ordered publishing, and — after an
  incident — main-only tag enforcement. **100.0%, corroborated in artefacts, one acceptance criterion
  false.** v0.4.0. Run 5.
- **M11 — Documentation overhaul and publish.** mdbook at `docs/`, 227 broken links repaired,
  linkcheck as an error, content rewritten, new orchestration / content-processing / bridge guides,
  six deployment-topology pages, GitHub Pages. **92.0% — the only genuinely open count in run 5.**
  v0.5.0. Run 5.
- **M12 — Web API.** Agent registry and execution API, config-driven `paladin-server`, SSE streaming
  and in-process jobs, unified error envelope with health/ready/CORS/limits/rate-limiting, API-key and
  bearer auth with per-agent roles, OpenAPI with a committed drift guard, Docker and Kubernetes
  artefacts. **99.0% — all three open items are feature-branch scaffolding.** v0.6.0. Run 5.

The tree is at **0.6.0** on branch **`release/v0.7.0`**, latest tag **v0.5.1**.

**The arc has a shape worth naming.** M1-M3 built capability. M4-M8 dismantled and rebuilt the
structure that capability lived in, at considerable cost and with almost no feature work. M9 finished
the half of the platform that M4-M8 had left alone. M10-M11 made it releasable and documented. M12
exposed it over HTTP — and it exists because M11's documentation epic wrote down a capability gap
instead of papering over it.

## Where the genuine remaining work lives

**It does not live in the checkbox counts.** Five runs of verification against the tree establish the
pattern beyond doubt:

- Runs 1-2 found counts **understating** shipped reality — Conclave 129 open and shipped, Sanctum 111
  open and shipped.
- Run 3 found the first **accurate** count (M4's 20, corroborated by zero `#[deprecated]`
  annotations) and the first **overstating** completion (CLI isolation fully checked, three
  dependencies still unconditional).
- Run 4 found M8's three open items **contradicted outright**.
- Run 5 found M12's three **vacuous** (feature-branch scaffolding) and project-management's one
  **nonexistent** (a formatting example inside a template).

Of the 542 recorded open checkboxes, the only count that survives verification as genuinely open work
is **Milestone 11's 26** — and even those are content-currency tasks against files that all exist,
which must be verified by content rather than by presence.

**It lives in three deferred registers and a short list of verified defects.**

**The three deferred registers**, in descending reliability:

1. **`Milestone_8/deferred-items.md` (D1-D5)** and **`deferred-features.md`** — run 4 verified every
   claim exactly, including a `println!` residue count of 17 across 6 files. Framed as "Record of
   intentional non-goals (not bugs / not oversights)". D5 is a quick win; D2 (mis-layered manager
   services) is an architecture pass; D1 and D3 only alongside a broader refactor. The two deferred
   features carry explicit reintroduction conditions — the `paladin user` CLI surface is "mostly
   re-wiring, not new domain work", and the ML adapter **must** return in a dedicated `paladin-ml`
   leaf crate, never to the facade.
2. **`Deferred-QA-CICD-Completion` (Epics 25-29, 135 FRs)** — the largest by requirement count and the
   least reliable by path accuracy. **Its scope is real and largely unbuilt** (Epic 25 verified open
   item by item; Epic 26's architecture rewrite and demos verified open; Epic 27 verified entirely
   absent). **Its paths and numbers need re-measurement** — four targets were relocated by later
   milestones, and Epic 29's 57.83% baseline predates Milestone 9's tests on the same module.
3. **`DEFERRED_COVERAGE.md`** — the coverage arithmetic behind Epics 28-29, with a named sign-off and
   a "Next Review: Epic 27 or Epic 28 planning" trigger that was never reached.

**The verified defects**, each confirmed against the tree and each small:

- The **duplicate `security` CI job** at `ci.yml:389-406` — delete 18 lines, and a Milestone 10
  acceptance criterion becomes true.
- **`project/current-exports.txt`** — nine stale references across two scripts, three workflow lines
  and four requirements, keeping the `api-surface` CI job permanently red. Unchanged across three
  ingest runs. The cheapest fix in the corpus.
- **`ProviderCapabilities` over-reporting tool-calling support** — correctable today, independent of
  whether Epic 27 is ever built.
- **`paladin-ports` doctests disabled** with a named "Task 7.0" re-enable point, unchanged since run 3.
- **`paladin-herald` has no `CHANGELOG.md`**, against an acceptance criterion recorded as Met.
- **`Dockerfile.chef`'s planner COPY list omits `paladin-herald`.**
- **Zero `#[deprecated]` annotations** against M4 Epic 2 FR-8.
- **Three competing `TokenUsage` definitions** shipping simultaneously.
- **Three CLI-only dependencies still unconditional** (`structopt`, `colored`, `comfy-table`).
- **The RustSec owner/expiry gap** — 13 of 15 suppressions without either, and a **2026-09-30**
  expiry on the two that have them that nothing in `.planning/` will otherwise surface.

**And in two open architectural questions that no document resolves:**

- **The token mechanism.** The agent API is documented as JWT and implemented as opaque in-process
  tokens, and Milestone 12 ships a Kubernetes Deployment against a store its own originating PRD said
  would need replacing for multi-process serving.
- **Garrison and Arsenal on HTTP-served agents.** Agents behind the HTTP host are LLM-plus-prompt
  only — no memory, no tools — stated once, in a non-goal, while the deployment-topologies decision
  matrix routes readers between topologies as though they were equivalent.

## Status

**STATUS: AWAITING USER — 0 blockers across all five runs. 69 competing variants need resolution
before routing, 16 of them new in run 5.**

Nothing gates the workflow. The corpus is fully ingested and internally consistent enough to plan
from, provided the variants are resolved rather than silently collapsed.

## Roadmapper notes for run 5 (and for the whole corpus)

- **Honour the Roadmap Extension Protocol** at the end of `ROADMAP.md`. **New phases start at
  Phase 12.** Phases 1-11 are never renumbered, and the `### Phase N: Name` header format is parsed
  by downstream tooling, including inside `<details>` blocks. Wrap completed milestone sections in
  `<details>` per protocol item 2.
- **Eleven requirement-ID prefixes are spent** and must not be recycled: `RECON-*`, `GAP-*`, `QUAL-*`,
  `REL-*` (M1); `VERIFY-*`, `CLOSE-*` (M2-3); `ARCH-*`, `DEBT-*` (M4-6); `SEC-*`, `HARD-*`,
  `FACADE-*` (M7-8). **Suggested fresh prefixes for run 5:** `ORCH-*` (Milestone 9 orchestrator),
  `SUPPLY-*` (the dependency-security governance items — note `SEC-*` is spent), `DOCS-*` (Milestone
  11 documentation currency and the architecture-document gap), `WEB-*` (Milestone 12 web API items),
  `PIPE-*` (Deferred-QA Epic 25 CI/CD), `DEFER-*` (the three deferred registers — suggested in run 4
  and still unused). Ingested `REQ-*` IDs remain the stable merge keys; match on them rather than
  re-deriving, and prefer extending an existing requirement in place over creating a near-duplicate.
- **Do not re-plan completed work.** M9 100%, M10 100%, M11 92.0%, M12 99.0% — and of those open
  counts only M11's 26 survive verification. The entire Milestone 9 orchestrator subsystem, the
  entire Milestone 10 tooling set, the mdbook, and the entire Milestone 12 web API demonstrably ship.
- **Precedence for this project is: shipped tree > `.planning/codebase/` map >
  `intel/code-verification.md` > PRD > DOC > task-list checkbox.** Every `src/…` path in runs 1-2 and
  several in runs 3-5 are historical; resolve current locations through the tree.
- **The four highest-value things to plan first**, all small and all verified:
  1. Delete `ci.yml:389-406`, then decide the disposition of the three unauthorised vulnerability
     ignores and the **2026-09-30** expiry. Consider promoting the two supply-chain ADR candidates.
  2. Fix the nine `project/current-exports.txt` references — five in tooling, four in requirement
     text — and turn the `api-surface` CI job green for the first time since commit `928c6d5`.
  3. Resolve the token-mechanism variant, because it has a correctness consequence under the
     Kubernetes Deployment that Milestone 12 shipped.
  4. Decide whether `design-and-architecture.md` is archive material or a live deliverable. It cannot
     be both, and it has been invisible for two milestones.
- **Then the deferred registers**, in the order their own documents recommend: Deferred-QA Epic 25
  first ("establishes quality gates that validate all subsequent work"), and **sequence M8 D2 against
  Epic 28 deliberately** — one splits `user_service.rs`, the other tests it, and doing them
  independently means doing the work twice.
- **Two decisions are worth making explicitly rather than by default:** where `AgentProvisioner`
  lives (it constrains the queue/worker and sidecar topologies), and whether Garrison/Arsenal wiring
  for HTTP-served agents is planned scope or a permanent property of the topology.
- **Eleven ADR candidates exist and zero locked decisions.** If any technical position in this
  roadmap should be protected from future override, re-tag its carrier via `--manifest` and re-run
  ingest. Do not manufacture a lock inside a planning artefact.
