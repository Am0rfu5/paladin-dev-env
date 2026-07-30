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

---

## Ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs)

**No ADR-typed documents were present in this ingest run either.**

Classification breakdown for run 4: 11 PRD, 29 DOC, 0 ADR, 0 SPEC. Every classification carried
`locked: false` and `precedence: null`; every one also carried `manifest_override: true`, so the
types are user-asserted rather than inferred.

Cumulative across runs 1-4: **153 documents ingested, 50 PRD, 103 DOC, 0 ADR, 0 SPEC,
0 locked decisions.** No LOCKED-vs-LOCKED contradiction is possible and none of the technical
positions recorded in `requirements.md` or `context.md` is protected from being overridden by a
future ADR. This holds for the full 153-document corpus, exactly as the standing constraint stated.

Locked decisions: 0.

---

### Decision-shaped material found in run 4 but NOT recorded as a decision

Run 4 is the densest run so far in decision-shaped content — it contains four documents whose whole
purpose is to record a choice. None of them carries an ADR status field, a Decision/Consequences
structure, or a `locked` flag, and all four were manifest-typed **DOC**. They are recorded as
context (and, where they change a requirement, as requirement entries or variants).

- **`.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md` — the strongest
  ADR candidate in the run.** It is a formal go/defer decision record with an explicit
  **"Self-Approval (Task 1.6)"** block: *"the Go decisions above are self-approved and documented
  here as the authoritative record … Approved by: AI Agent (GitHub Copilot), acting as sole
  developer on `feature/milestone_7`. Approval date: 2026-05-25. Approval scope: Proceed to
  Task 2.0."* It scores four candidate crate extractions on dependency weight, change frequency,
  consumer selectivity and extraction complexity, records measured evidence (dep-tree deltas of
  +41, +145 and +210 lines over a 1,235-line baseline; commit counts of 9, 15, 21 and 32 since
  2025-01-01), issues **four Go decisions and zero Defer**, and fixes an extraction order. Its own
  governing PRD (§4.1.5) calls it *"the authoritative source of record for *why* a decision was
  made."* Everything an ADR needs is present except the type tag. Recorded as
  `REQ-m7-cost-benefit-gate` and in `context.md`.

- **`.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md` — a formal risk
  acceptance, which is a decision with an owner and an expiry.** It records two blocking advisories
  with no upstream fix, states the acceptance criteria as *"Either vulnerabilities are eliminated
  from release dependency graph, or formal risk acceptance is documented with: owner, expiry date,
  affected scope, compensating controls, tracked follow-up issue"*, and then supplies exactly that:
  **exception owner Platform Security (Milestone 7), review/expiry target 2026-09-30.** This is the
  only decision in the entire 153-document corpus that carries a **stated expiry date**, and it is
  a live security posture rather than a structural choice. Recorded as `REQ-rustsec-risk-acceptance`
  and surfaced prominently in `INGEST-CONFLICTS.md`.

- **`.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md`
  — a signed-off licensing policy decision.** It names a **policy approver of record (`DF3NDR`,
  repository owner) and an approval date (2026-05-28)**, and records an explicit accept-or-replace
  decision on MPL-2.0: *"Explicit acceptance of MPL-2.0 dependencies for unmodified use in this
  project."* It sets the project licensing model to **`MIT OR Apache-2.0`**, which contradicts the
  `license (MIT)` position held by the Milestone 7 overview and by the shipped root `Cargo.toml`.
  A named approver plus a dated approval plus a scoped acceptance is ADR-shaped in substance.
  Recorded as `REQ-license-policy-signoff`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/facade-cleanup-RECONCILIATION-2026-06-04.md`
  — a supersession notice that reverses a prior Epic.** Its header carries
  `Supersedes (corrects): Epic_1/facade-audit.md and Epic_3/infrastructure-adapter-disposition.md`
  and `Status: Proposed`. The status marker is a plan-status field, not an ADR status. §6 lists six
  **"Open decisions (blockers for Phases 3–4)"** — finish relocations now vs. defer to M9; Herald
  formatters home (`paladin-core` vs. new `paladin-herald`); queue home (`paladin-storage` vs. new
  `paladin-queue`); delete vs. wire `commands/user.rs`; delete vs. keep `tensorflow_adapter.rs`;
  keep vs. remove `src/core/` shims — and all six were then resolved **in execution rather than by
  a recorded decision**: relocations done now, Herald → new `paladin-herald`, queue →
  `paladin-storage`, `user.rs` deleted, tensorflow deleted, `src/core/` shims kept. This is the
  same "resolved by outcome, not by a recorded decision" pattern that run 3 flagged for Milestone 4
  Epic 3's binary-target question. Recorded as `REQ-m8-reconciliation-relocations`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md`** — states D1
  (`src/core/` shims) as **"KEEP, by decision"** and D3 as **"KEEP for now"**, each with an
  effort/risk rating and a recommendation. The framing line is *"Record of intentional non-goals
  (not bugs / not oversights)"*. Decision-shaped, DOC precedence; recorded as
  `REQ-m8-deferred-items-register`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md`** — sets a placement
  **condition** on reintroducing the TensorFlow adapter: it must be rebuilt in a dedicated
  `paladin-ml` leaf crate, *"consistent with the hexagonal layout — ML inference is an
  infrastructure adapter, not facade code — rather than re-adding it to the facade."* That is an
  architectural constraint on future work, carried by a DOC. Recorded as
  `REQ-deferred-tensorflow-ml-adapter-v3`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md`
  §8 "Resolved Decisions (formerly Open Questions)"** — the closest a run-4 PRD comes to an ADR,
  matching the M5 Epic 2 §9 pattern run 3 flagged. It closes three questions: `garrison/mod.rs`
  stays (not a deletion candidate); `sanctum/mod.rs` stays (same reasoning); and
  `output/api_content_deliverer.rs` stays for Epic 3 but is a confirmed M9 `paladin-web` extraction
  target — with two factual corrections recorded (the file is **724 LOC, not 629**; the 629 figure
  belongs to `tensorflow_adapter.rs`). §6 also records the reasoned asymmetry for *why* storage
  shims were deleted but garrison/sanctum shims were not. PRD precedence; recorded as
  `REQ-garrison-sanctum-bridges-kept`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_4/prd-use_cases-services-rename.md`
  §4.1.3 / §5** — decides a **clean break** with no backward-compatible re-export, and explicitly
  rejects the option its own Epic DOC offers: *"Task 4.3 from the Epic spec is explicitly rejected;
  there will be no `pub use services as use_cases;`."* An explicit rejection of a named alternative
  is ADR-shaped, but it is carried by a PRD. Recorded as `REQ-rename-clean-break`.

- **`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/Milestone_8-Epic_7-paladin-web-single-framework-axum.md`**
  — carries a section literally headed **"Decisions (from PRD clarification)"**: port the three
  endpoints to axum **and mount them** (revive the API) rather than deleting them; and add
  `actix-web` to `deny.toml`'s banned crates. Two clean decision statements with named
  alternatives, in a DOC. Recorded as `REQ-delivery-endpoints-axum` and `REQ-actix-deny-ban`.

- **`.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` §9** —
  a five-row resolution table closing all pre-implementation questions: `tensorflow_adapter.rs`
  stays in the facade with `paladin-ml` deferred to M8+; two granular storage flags plus a `storage`
  alias rather than one flag; `sqlx` stays in `[workspace.dependencies]` shared by `paladin-memory`
  and `paladin-storage`; `file_content_repository.rs` does **not** go into `paladin-storage`; defer
  decisions are recorded in three places. Recorded as `REQ-tensorflow-stays-facade-v1`,
  `REQ-storage-feature-flags-v1`, `REQ-sqlx-workspace-dependency`, `REQ-paladin-storage-extraction`
  and `REQ-m7-cost-benefit-gate`.

- **`.github/workflows/ci.yml:617-680`** (not an ingested document; read from the tree) — carries an
  inline rationale rejecting the per-crate publish order that M7 Epic 2 FR-26 mandates: *"Per-crate
  `cargo publish --dry-run -p <crate>` cannot work on a version bump: the not-yet-published new
  version of each sibling fails the `version = \"X\"` requirement of its dependents."* Recorded as
  `REQ-ci-publish-dry-run-v2` because the technical position is substantive and contradicts an
  ingested requirement, but it has no document carrier and therefore no precedence standing.

### Promoting any of these to a decision

Six ADR candidates now exist across the corpus: Epic 17.5's CLI-location decision (run 2), the
Milestone 5 Epic 1 `battalion-result-upward-dependency-decision.md` (run 3), and the four run-4
documents named above. Promoting any of them requires **re-tagging the source document via
`--manifest` and re-running ingest**. Manufacturing a lock inside a planning artefact would
fabricate authority the corpus does not contain. The two strongest candidates in run 4 are
`cost-benefit-assessment.md` (has a self-approval block, an approver and an approval date) and
`rustsec-remediation-plan.md` (has an owner, an expiry date and compensating controls) — the latter
being the only corpus item where *not* promoting it has an ongoing operational cost, because the
acceptance expires 2026-09-30 and nothing else in `.planning/` will surface that date.

Run 5 (Milestones 9-12, Deferred-QA-CICD-Completion, project-management) may add ADR-typed docs;
this file remains append-only in merge mode.

---

# Ingest run 5 of 5 — decisions

**Decisions extracted in run 5: 0. Cumulative across all five runs: 0.**
**Decisions locked: 0. Cumulative: 0.**
**Source paths: none.**

Run 5 closes the ingest with the standing constraint intact and now final:

> **There is no ADR-typed and no SPEC-typed document anywhere in the corpus.**
> **199 classified documents across 263 files in `.project/`. Zero ADR. Zero SPEC. Zero locked decisions.**

The consequences are structural, not cosmetic:

- No LOCKED-vs-LOCKED contradiction has ever been possible, in any run. The zero-blocker result
  across all five runs is a property of the corpus's document typing, not evidence that the corpus
  is free of contradictions. It is not — there are 67 recorded competing variants.
- **Nothing in `requirements.md` is protected from override.** Every one of the 554 requirement
  entries sits at PRD or DOC precedence and can be superseded by any ADR that arrives later.
- Where a document *does* make a decision — and several do, emphatically — that decision carries
  the precedence of its manifest type, not the precedence its content deserves. Milestone 9 Epic 5
  §6.1 chooses opaque bearer tokens over JWT with a written rationale, a named trade-off and a
  rejected alternative. Milestone 9 Epic 4 §6.1 tabulates Option A against Option B across four
  criteria and records "Decision (1A/C): Adopt **Option A**". Both are PRD sections. Neither binds.

## Decision-shaped content in run 5 (ADR candidates)

Run 5 adds **five** candidates, bringing the corpus total to **eleven**. None is promoted here;
promotion requires re-tagging the source via `--manifest` and re-running ingest. Manufacturing a
lock inside a planning artefact would fabricate authority the corpus does not contain.

- **`Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md` §6.1** —
  the cleanest ADR-shaped section in the entire corpus. A four-criterion comparison table
  (discoverability by LLM, safety/authorization, testability/coupling, consistency with
  Arsenal/MCP), an explicit **"Option A — `OrchestratorPort` (CHOSEN)"** column header, a stated
  decision — *"Adopt **Option A**. It maximizes decoupling, testability, and centralized safety
  enforcement"* — and an explicit preservation of the rejected option as a future non-breaking
  enhancement: *"an `OrchestratorArmament` can simply wrap an `Arc<dyn OrchestratorPort>` and
  register in the Arsenal later, without changing the port."* Recorded as `REQ-orchestrator-port`,
  `REQ-bridge-policy-guardrails` and `REQ-orchestrator-bridge-adapter`.

- **`Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md` §6.1** —
  *"**Chosen:** opaque, randomly-generated bearer tokens with a server-side hashed store"*, with
  rationale (no `jsonwebtoken` dependency, no signing-key management story, supports immediate
  revocation which stateless JWTs cannot, trivially deterministic to unit test, no new dependencies)
  **and a recorded trade-off**: *"tokens are validated against an in-process store, so a
  multi-process deployment would later need a shared store."* Recorded as
  `REQ-opaque-bearer-token-adapter-v1`.

  **This is the highest-value promotion candidate in run 5.** It is the only decision in the corpus
  that a *later milestone contradicts in prose while silently preserving in code* — Milestone 12
  Epic 5 specifies JWT throughout, the shipped `agent_auth.rs` is written in JWT vocabulary, and the
  only `AuthPort` implementation in the workspace is still the opaque in-memory token adapter with
  no `jsonwebtoken` dependency anywhere. Had this been an ADR, Milestone 12 would have had to
  supersede it explicitly rather than drift past it.

- **`Milestone_9-Classic-Orchestrator-Completion/Epic_3/prd-content-agent-bridge.md` §7 and OQ-1** —
  a buildability decision recorded as a **resolved** open question: *"Initial preference was to place
  the processors inside `paladin-content`. Because the `ContentProcessor` trait and
  `OrchestratorError` live in the root crate (which already depends on `paladin-content` and
  `paladin-battalion`), implementing them in `paladin-content` would create a **circular
  dependency**. **Resolution:** place the processors in the root crate."* The identical constraint is
  restated in Epic 4 §6.2 for the bridge adapter. A crate-placement rule derived from a hard
  buildability constraint, applied twice — this is architecture, carried by two PRDs.

- **`Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md` §"Scope decisions (from PRD
  clarification)"** — a section literally headed *decisions*, resolving four questions: posture
  (**required by default, but disable-able**); mechanisms (**API keys + JWT**); per-agent
  authorization (**role-based** via optional `allowed_roles`); route privilege (**admin** for
  `POST`/`DELETE /agents`). Same shape as the run-4 Milestone 8 Epic 7 *"Decisions (from PRD
  clarification)"* block. Recorded as `REQ-api-key-auth`, `REQ-jwt-bearer-auth-v2`,
  `REQ-fail-closed-auth-posture`, `REQ-per-agent-role-authorization` and
  `REQ-admin-gated-registration`.

- **`Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md`
  FR-1 plus §8** — the **origin policy for dependency-advisory suppression**, and the one run-5
  candidate with an ongoing operational cost from not being promoted. It states the invariant twice:
  FR-1 requires the ignore-list be sourced from `audit.toml` *"so the workflow and the config cannot
  drift"*, and §8 makes it a success metric — *"`audit.toml` and `deny.toml` are the only places
  policy/exceptions are defined; **no inline advisory-ignore flags remain in CI**."*

  **It is violated in the shipped tree.** `ci.yml:390-406` still runs
  `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` from a duplicate `security`
  job that Milestone 10 Epic 2 never removed. Because this is a PRD success metric rather than a
  locked decision, nothing gates on it and the violation survived a milestone recorded 100%
  complete, plus two subsequent milestones. Recorded as `REQ-audit-toml-single-source`.

## Promoting any of these

Eleven ADR candidates now exist across the corpus:

| Run | Candidate |
|---|---|
| 2 | `Epic_17.5/epic17-5.md` — CLI location |
| 3 | `Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` — the only `Status: Approved` decision record |
| 4 | `Epic_1/cost-benefit-assessment.md` — self-approval block, named approver, approval date |
| 4 | `Epic_4/rustsec-remediation-plan.md` — owner **Platform Security**, **expiry 2026-09-30** |
| 4 | `Epic_4/license-compatibility-decision-checklist.md` — approver `DF3NDR`, 2026-05-28 |
| 4 | `facade-cleanup-RECONCILIATION-2026-06-04.md` — explicit supersession notice |
| 5 | `M9/Epic_4/prd-agent-orchestrator-bridge.md` §6.1 — Option A versus Option B |
| 5 | `M9/Epic_5/prd-user-admin-system-completion.md` §6.1 — opaque tokens versus JWT |
| 5 | `M9/Epic_3/prd-content-agent-bridge.md` §7 — root-crate placement (circular-dependency rule) |
| 5 | `M12/Epic_5/prd-api-security-authorization.md` — four scope decisions |
| 5 | `M10/Epic_2/prd-dependency-security-license-compliance.md` — the audit-suppression single-source policy |

**The two with a live operational cost are `rustsec-remediation-plan.md` (run 4) and
`prd-dependency-security-license-compliance.md` (run 5), and they are the same subject.** One
carries the only expiry date in the corpus — **2026-09-30**, roughly two months from the ingest
date, and nothing else in `.planning/` will surface it. The other carries the invariant that the
tree currently violates. Promoting them together would turn the run-5 supply-chain finding from a
recorded observation into a gate.

The ingest is complete; this file remains append-only. Any future ADR arriving through a re-run
outranks everything in `requirements.md`.
