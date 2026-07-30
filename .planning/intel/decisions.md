# Decisions (from ADR-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP` (36 docs).

**No ADR-typed documents were present in this ingest run.**

Classification breakdown for this run: 11 PRD, 25 DOC, 0 ADR, 0 SPEC.

No decision entries are recorded. Nothing in the source set carried an ADR
status field, a Decision/Consequences structure, or a `locked: true` flag, so
no decision statements are asserted here. Several `epic*.md` DOCs contain
technical design blocks that read like decisions (Rust type and trait
contracts); per the precedence rules these are recorded as context, not as
decisions — see `context.md`.

Locked decisions: 0.

Subsequent ingest runs (Milestones 2-12, Deferred-QA-CICD-Completion,
project-management) may add ADR-typed docs; this file is expected to be
appended to in merge mode.

---

## Ingest run 2 of 5 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs)

**No ADR-typed documents were present in this ingest run either.**

Classification breakdown for run 2: 15 PRD, 30 DOC, 0 ADR, 0 SPEC. Every
classification carried `locked: false` and `precedence: null`.

Cumulative across runs 1-2: 81 documents ingested, 26 PRD, 55 DOC, **0 ADR, 0 SPEC,
0 locked decisions**. No LOCKED-vs-LOCKED contradiction is possible and none of the
technical positions recorded in `requirements.md` or `context.md` is protected from
being overridden by a future ADR.

Decision-shaped material found in run 2 but NOT recorded as a decision (it sits at
DOC or PRD precedence, not ADR):

- `.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md` — "Recommended
  Consolidation" chooses `src/application/cli` over `src/cli` on the stated rationale
  that "CLI is an input adapter in the application layer, not infrastructure", and
  directs deletion of the entire `src/cli` tree plus removal of `pub mod cli;` from
  `lib.rs`. This is the only module-ownership decision in the run-2 corpus. It has no
  ADR status field, no Consequences section and no `locked` flag, so it is recorded as
  context. **Strongest ADR candidate in the corpus so far** — see
  `.planning/INGEST-CONFLICTS.md` INFO.
- `.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md` —
  establishes a single source of truth for `PaladinResult`, `BattalionResult` and
  `PaladinError` (Herald imports the real domain types). Decision-shaped, but carried
  by a PRD, so recorded as `REQ-herald-type-consolidation`.
- `.project/Milestone_3-Completion/Post-Epic_24-cleanup/LEGACY_CODE_CLEANUP_PLAN.md` —
  establishes that `adapters/llm/` is the canonical location for LLM adapters and
  `adapters/output/` is legacy. Recorded as context.

Locked decisions: 0.

Runs 3-5 (Milestones 4-12, Deferred-QA-CICD-Completion, project-management) may add
ADR-typed docs; this file remains append-only in merge mode.

---

## Ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements` (32 docs)

**No ADR-typed documents were present in this ingest run either.**

Classification breakdown for run 3: 13 PRD, 19 DOC, 0 ADR, 0 SPEC. Every classification carried
`manifest_override: true`, `confidence: high`, `locked: false` and `precedence: null`.

Cumulative across runs 1-3: **113 documents ingested, 39 PRD, 74 DOC, 0 ADR, 0 SPEC,
0 locked decisions.** No LOCKED-vs-LOCKED contradiction is possible, no locked decision exists in
`.planning/` for an ingest decision to contradict, and none of the technical positions recorded in
`requirements.md` or `context.md` is protected from being overridden by a future ADR.

Locked decisions: 0.

---

### The only decision record in the corpus — and why it is still not a locked decision

Run 3 contains the first and only pair of files in all 263 documents that are structured as a
decision process:

- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-options.md`

The decision document has every structural marker of a real ADR: it sits under a `decisions/`
directory, carries **`Status: Approved`**, a **`Decision Date: 2026-05-13`**, an explicit
**`Chosen Option: Option A`**, a Rationale section, a Rejected Options section naming Option B and
Option C with reasons, and a Consequences-equivalent implementation checklist. Its sibling is a
full three-option trade-off analysis ending in a single recommendation, with `Status: Awaiting
decision (Task 3.3)` and an implementer-interview section.

**It is nevertheless recorded as context, not as a decision.** Both files are manifest-typed
`DOC`, so under `ADR > SPEC > PRD > DOC` they sit at the lowest precedence tier and carry
`locked: false`. Per the ingest constraints, a locked decision must not be manufactured from a
PRD or DOC assertion no matter how decision-shaped the prose is. Consequently the position it
records is overridable by any PRD — and in fact **a PRD published two days later contradicts it**
(see `REQ-port-value-type-ownership-v1` versus `-v2` in `requirements.md`).

**What the decision actually settles — recorded precisely:** the *location* of five pure
value/error types. `PaladinResult` and `StopReason` move to
`core/platform/container/execution_result.rs`, `TokenUsage` to `token_usage.rs`, `RegistryError` to
`registry_error.rs`, and `HandoffError` from `src/application/errors/handoff_error.rs` to
`core/platform/container/arsenal/handoff_error.rs`. The four application-layer files that
previously defined them become thin `pub use` re-exports, so every existing
`paladin::application::ports::output::…` path keeps resolving. `PaladinError` is deliberately
excluded because it carries `#[from] GarrisonError` from the application layer, and the
consequence is that the convenience `pub use PaladinError` in `herald.rs` is removed.

**What it does NOT settle — recorded equally precisely:** it never mentions `BattalionResult`. The
run-1 competing `BattalionResult` field sets (`REQ-battalion-result-v1` / `-v2`) are entirely
untouched by this record, despite the filename. The Milestone 5 overview's risk register had
proposed "defining `BattalionResult` in `paladin-core`" as the resolution and had assigned the
work to Epic 2; the decision resolved it in Epic 1 by moving `PaladinResult` instead. Anyone
reading the filename alone will draw the wrong conclusion. (The run-1 `BattalionResult` variant is
closed by **shipped code**, not by this document — see `code-verification.md`.)

Recommendation: if the location of these five types should be protected from future override,
promote this record to a real ADR via `--manifest` and re-run ingest. It and
`Epic_17.5/epic17-5.md` (the CLI-location decision from run 2) are the two strongest ADR
candidates in the corpus.

---

### Other decision-shaped material found in run 3 but NOT recorded as a decision

- `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md` — the
  **`NOTE (2026-04-15)`** eliminating the `mcp-arsenal` feature flag from scope, on the recorded
  rationale that "the complexity of gating the Arsenal subsystem was deemed unnecessary given its
  pervasive use throughout the framework and minimal dependency overhead (pure Rust
  implementation)". This is a scope decision with a rationale and a date, but it is carried by a
  PRD, so it is recorded as `REQ-feature-flag-matrix`.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md` §6.1 — the choice of
  "a single `cli` feature flag rather than multiple granular CLI feature flags", with the rationale
  "the CLI is a cohesive unit… granular flags add complexity without providing meaningful value".
  Decision-shaped; PRD precedence; recorded as `REQ-cli-feature-gate`.
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` §9
  "Resolved Design Decisions" — three questions closed before finalisation: all five previously
  unspecified port files are in scope; **full deletion of `src/application/ports/` (Option B)** was
  selected over a shim; and **no feature flag in `paladin-ports` (Option B)** for the vision ports.
  This section is the closest a PRD in this corpus comes to an ADR. Recorded as
  `REQ-input-ports-extraction`, `REQ-ports-facade-wiring` and `REQ-paladin-ports-scaffold`.
- `.project/Milestone_6-Architectural-Refinements/Epic_4/prd-relocate-circuitbreaker-infra.md` §5
  and §6 — closes the Milestone 6 overview's three-way choice by selecting **Option A** (facade
  `src/infrastructure/resilience/`) and explicitly rejecting a `paladin-infra` crate and a
  `CircuitBreakerPort` trait. It also records a reasoned acceptance of a layering inversion
  (`PaladinExecutionService` importing an infrastructure type) as "an acceptable pragmatic
  trade-off within the facade crate's module organization". Recorded as
  `REQ-circuitbreaker-relocation`.
- `.project/Milestone_6-Architectural-Refinements/Epic_2/prd-relocate-orchestration-services.md`
  Non-Goal 7 — decides that **no backward-compatibility re-export shims** are added and that
  "backward compatibility is scoped to compilation — callers will update their import paths". Its
  own Open Question 4 immediately undercuts this ("the current decision is no re-exports, but this
  should be confirmed with the team before implementation begins"), so it is recorded as
  `REQ-orchestration-no-reexport-shims` with a WARNING rather than as a settled decision.
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` FR-39 —
  decides that old deep import paths are **intentionally not preserved** ("New consumers must use
  `paladin::prelude::OpenAIAdapter`"), which is the opposite of the backward-compatibility posture
  every other Milestone 5 epic takes. Recorded as `REQ-llm-facade-prelude`.

Runs 4-5 (Milestones 7-12, Deferred-QA-CICD-Completion, project-management) may add ADR-typed
docs; this file remains append-only in merge mode.
