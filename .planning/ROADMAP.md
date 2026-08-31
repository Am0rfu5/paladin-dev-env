# Roadmap: Paladin

## Overview

**Paladin already works.** It ships at v0.7.0 with a Cargo workspace of ten library crates plus a
`doc-examples` crate and the root `paladin-ai` facade, 22 runnable examples, an HTTP API with
OpenAPI and SSE streaming, a `paladin-server` binary, a 112 MB multi-arch Docker image and
reference Kubernetes manifests. (**Amended by Phase 4, dated 2026-08-03, citing
`04-release-measurement.md`**: the "22 runnable examples" figure traces to a Milestone 1 Epic 10
validation report ("22/22 examples compiling") and has since gone stale — the shipped tree carries
**47** `.rs` files under `examples/`, of which four are declared `[[example]]` targets gating on
non-default features (`vision`, `content-processing`, `web-server`); no crate under `crates/` ships
its own `examples/` directory. The shipped tree outranks an ingested count under this project's
precedence order. Going forward the gate REL-05 and ROADMAP criterion 5 express is "every example
target builds", not a count, so this figure cannot go stale the same way again.)

**This planning corpus is a historical record of twelve shipped milestones plus a verified-defect
and deferred-work forward scope. It is not a greenfield plan.** Across the 263 documents in
`.project/`, 7,511 of 8,053 task items are checked (93%) — and five runs of direct code
verification found the shipped tree *ahead of* even that figure in most places. **This roadmap does
not build the framework. It closes out milestones that already shipped, fixes what verification
proved broken, and builds the one epic-set nobody ever started.**

The sixteen phases fall into four kinds of work, and it is worth naming them before the detail:

| Kind | Phases | What it is |
|---|---|---|
| **Record** | 1, 5, 7, 10, 13 | Make `.planning/` describe the code as it actually is, so nobody re-plans shipped work or applies a superseded PRD literally |
| **Verified defect** | 8, 12 | Fix the things direct verification proved broken — a permanently red CI job, missing annotations, disabled doctests, leaked dependencies, a duplicated audit job |
| **Decision** | 9, 11, 14 | Answer the questions the corpus left open, including the ones with a correctness or security consequence attached |
| **Genuinely unbuilt** | 2, 3, 4, 6, 15, 16 | The residual functional gaps, the quality gates, and the two deferred registers whose work was never started |

### The milestone arc this roadmap closes out

**M1-M3 built capability.** Paladin, Garrison, Arsenal, the four base Battalion patterns, Herald,
Citadel, Commander and the Armory CLI (M1); then Sanctum vector memory and RAG, Sentinel vision,
autonomous agents, Conclave, Council, Grove and the Maneuver Flow DSL (M2); then the completion
pass over all of it (M3).

**M4-M8 dismantled and rebuilt the structure that capability lived in**, at considerable cost and
with almost no feature work: feature-flag expansion and port hardening (M4), the monolith becoming
a Cargo workspace (M5), four layer relocations (M6), four more crate extractions and the first
crates.io publish (M7), and a facade cleanup that a dated reconciliation then audited against the
tree and took further than its own plan allowed (M8).

**M9-M12 finished, hardened, documented and exposed it.** M9 completed the half of the platform
M4-M8 had left alone — a real `execute_workflow()`, a workflow repository with crash recovery,
scheduler/queue/event validation, the bidirectional content-agent bridge and user/admin RBAC. M10
made it releasable: pre-commit, cargo-audit + cargo-deny + OSV-Scanner, a CycloneDX SBOM,
cargo-release with dependency-ordered publishing, and — after an incident — main-only tag
enforcement. M11 documented it into an mdbook with 227 broken links repaired and linkcheck as an
error. M12 exposed it over HTTP, **and it exists because M11's documentation epic wrote down a
capability gap instead of papering over it.**

### What the phases do, in order

**Milestone 1 close-out (Phases 1-4)** is short and specific. Make the planning record match the
shipped code and give each of the six contested type/gate definitions one recorded answer
(Phase 1). Close the residual functional gaps verification exposes and apply those definitions in
code (Phase 2). Make the quality numbers real rather than aspirational (Phase 3). Make the release
coherent — one version, one edition, a defensible advisory posture, reviewed docs, the whole gate
suite green (Phase 4).

**Milestone 2-3 close-out (Phases 5-6)** is shorter still, and that is the finding rather than an
omission. Sanctum, RAG, Sentinel vision, autonomous planning and handoffs, Conclave, Council,
Grove, the Maneuver Flow DSL, the enhanced CLI, Herald consolidation, the Paladin registry and the
scheduler port **all ship in the v0.7.0 tree.** What is missing is the record (Phase 5). Exactly one
defect in run-2 scope is verified open, and it closes alongside whatever Phase 5 exposes (Phase 6).

**Milestone 4-6 close-out (Phases 7-8)** covers the three milestones that restructured what M1-M3
built. All of it shipped, and unusually for this corpus it was verified directly against
`Cargo.toml` contents and type definitions rather than inferred. Phase 7 records what shipped and
answers the variant pairs; **Phase 8 is the first phase whose scope is entirely verified defects.**

**Milestone 7-8 close-out (Phases 9-11)** is the first block where the *record* is in better shape
than the gates. The 2026-06-04 reconciliation is the most reliable document in the corpus — every
verifiable claim in it matches the tree, including a `println!` residue count exact to the
occurrence. So Phase 9 fixes the gates rather than the record and **carries the only dated item in
the corpus, a RustSec acceptance expiring 2026-09-30**. Phase 10 writes down what M7-M8 delivered.
Phase 11 disposes of the deferred registers.

**Milestone 9-12 + Deferred-QA close-out (Phases 12-16)** is where the last of the forward work
lives, and it splits cleanly. Phase 12 deletes eighteen lines of CI that falsify a completed
milestone's own success metric, and gives thirteen advisory suppressions an owner and a date.
Phase 13 records what four milestones delivered and answers two seams M12 left as defaults.
Phase 14 closes the gap between what the project's interfaces *advertise* and what they *do* — an
API documented as JWT and implemented as opaque tokens, a Kubernetes Deployment against an
in-process token store, and an LLM capability flag that over-reports. Phase 15 builds the quality
gates Deferred-QA Epic 25 specified and nobody started, then closes the coverage register those
gates measure. Phase 16 finishes Milestone 11's documentation currency — **the only open checkbox
count in all 542 that survives verification** — and decides the fate of an architecture document
frozen at 311 lines that two milestones made invisible.

## Milestones

| Milestone | Phases | Status | Source |
|---|---|---|---|
| **Milestone 1 close-out** | 1-4 | ✅ **Shipped v0.7.1 (2026-08-04)** — [archive](milestones/v0.7.1-ROADMAP.md) | Ingest run 1 — `.project/Milestone_1-MVP` (36 docs) |
| **Milestone 2-3 close-out** | 5-6 | ✅ **Shipped v0.8.0 (2026-08-24)** — [archive](milestones/v0.8.0-ROADMAP.md) | Ingest run 2 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs) |
| **Milestone 4-6 close-out** | 7-8 | ✅ **Shipped v0.8.0 (2026-08-24)** — [archive](milestones/v0.8.0-ROADMAP.md) | Ingest run 3 — `.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements` (32 docs) |
| **Milestone 7-8 close-out** | 9-11 | ✅ **Shipped v0.8.0 (2026-08-24)** — [archive](milestones/v0.8.0-ROADMAP.md) | Ingest run 4 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs) |
| **Milestone 9-12 + Deferred-QA close-out** | 12-16 | ✅ **Shipped v0.8.0 (2026-08-24)** — [archive](milestones/v0.8.0-ROADMAP.md) | Ingest run 5 (FINAL) — `.project/Milestone_9-Classic-Orchestrator-Completion` + `.project/Milestone_10-CI-Hardening-Release-Automation` + `.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` + `.project/Deferred-QA-CICD-Completion` + `.project/project-management` (46 docs) |
| **Provider Expansion** | 17 | ✅ **Shipped v0.8.0 (2026-08-24)** — [archive](milestones/v0.8.0-ROADMAP.md) | Forward work — not ingest-derived. Added 2026-08-15 per *Roadmap Extension Protocol* item 1. |
| **Security Tooling** | 18- | ◆ **Current — v0.9.0, started 2026-08-24** | Forward work — not ingest-derived. Added 2026-08-24 per *Roadmap Extension Protocol* item 1, closing the Rust-SAST gap the v0.8.0 milestone audit left as its one genuinely open item. |

**The ingest is complete.** All 263 documents in `.project/` are covered — 199 classified across
five runs and 64 `tasks-*.md` measured deterministically by `intel/task-completion-state.md`. There
is no run 6. The *Roadmap Extension Protocol* at the end of this file still governs any future
addition, but nothing is pending.

Milestone numbering follows the **directory / task-list numbering**. Four source milestones number
themselves differently — the M4-M6 overviews use refactoring tiers ("Milestone 1/2/3"), the M3
release notes assign Epics 19-23 to four M2 features, and the M7 overview titles itself
"Milestone 4" — and none of those labels is used as a key anywhere in this file (VERIFY-03,
ARCH-02, HARD-04). **The protocol predicted a fifth instance in run 5; run 5 found none, and
ORCH-05 records the prediction closed.**

## Phases

**Phase Numbering:**

- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

<details>
<summary>✅ <strong>Milestone 1 close-out (Phases 1-4)</strong> — SHIPPED v0.7.1 2026-08-04 · 38 plans, 25/25 requirements</summary>

- [x] **Phase 1: Ground Truth & Decision Records** - Verify the planning record against shipped v0.7.0 code and record one answer per competing variant pair (completed 2026-07-31)
- [x] **Phase 2: Functional Gap Closure** - Finish the residual Milestone-1 functionality and apply the recorded definitions in code (completed 2026-08-01)
- [x] **Phase 3: Verification Depth** - Make coverage, error-path testing and performance baselines real and measured (completed 2026-08-02)
- [x] **Phase 4: Release Coherence** - One version, one edition, defensible dependencies, reviewed docs, green gate suite (completed 2026-08-03)

Full detail: [`milestones/v0.7.1-ROADMAP.md`](milestones/v0.7.1-ROADMAP.md) ·
Audit: [`milestones/v0.7.1-MILESTONE-AUDIT.md`](milestones/v0.7.1-MILESTONE-AUDIT.md) ·
Phase artifacts: `milestones/v0.7.1-phases/`

</details>

<details>
<summary>✅ <strong>v0.8.0 — Milestone 2-12 close-out &amp; Provider Expansion (Phases 5-17)</strong> — SHIPPED 2026-08-24 · 149 plans, 65/65 requirements</summary>

**Milestone 2-3 close-out**

- [x] **Phase 5: Milestone 2-3 Ground Truth** - Record what Epics 11-24 actually shipped, verify the three unverified blocks, and fix the epic-numbering defect at its source (completed 2026-08-05)
- [x] **Phase 6: Verified Gap Closure** - Close the one verified defect plus whatever Phase 5 proves genuinely outstanding (completed 2026-08-05)

**Milestone 4-6 close-out**

- [x] **Phase 7: Workspace Ground Truth & Recorded Answers** - Record what the refactor milestones actually shipped, correct the five positions the code contradicts, and answer the four variant pairs and two policy questions (completed 2026-08-06)
- [x] **Phase 8: Verified Defect Closure** - Fix the five defects verification proved open: the broken API-surface CI job, missing deprecations, disabled port doctests, leaked CLI dependencies, and duplicate `TokenUsage` (completed 2026-08-07)

**Milestone 7-8 close-out**

- [x] **Phase 9: Release & Security Gate Integrity** - Reconcile the four divergent RustSec exception sets before the 2026-09-30 expiry, settle the licence posture, and close the three small release-gate defects (completed 2026-08-08)
- [x] **Phase 10: Milestone 7-8 Ground Truth & Recorded Account** - Record what production hardening and facade cleanup actually delivered, make the 2026-06-04 reconciliation authoritative, and answer the three architecture questions the documents left ambiguous (completed 2026-08-08)
- [x] **Phase 11: Facade Residue & Deferred Register Disposition** - Give each of the five deferred items and both deliberately removed features a decision, and triage the Milestone 9 candidate list (completed 2026-08-09)

**Milestone 9-12 + Deferred-QA close-out**

- [x] **Phase 12: Supply-Chain Gate Integrity** - Delete the duplicate audit job that falsifies a completed milestone's success metric, and give every advisory suppression an owner and a date (completed 2026-08-10)
- [x] **Phase 13: Milestone 9-12 Ground Truth & Recorded Account** - Record what the orchestrator, release-automation, documentation and Web API milestones delivered, and turn two recorded defaults into decisions (completed 2026-08-10)
- [x] **Phase 14: API Contract Truthfulness** - Make every capability the project advertises through an interface one it actually has — the token mechanism, the multi-replica store, and the LLM capability flag (completed 2026-08-12)
- [x] **Phase 15: Coverage & CI Quality Gates** - Build the quality gates Deferred-QA Epic 25 specified and nobody started, then close the coverage register those gates measure (completed 2026-08-13)
- [x] **Phase 16: Documentation Currency & the Architecture Gap** - Settle Milestone 11's fourteen content-currency files by content, and decide whether the 311-line architecture document is archive or deliverable (completed 2026-08-24)

**Provider Expansion** — first forward work beyond the ingest (added 2026-08-15)

- [x] **Phase 17: Additional LLM Provider Adapters** - Decide which additional providers qualify against recorded criteria, then ship each survivor as a feature-gated adapter meeting the full `LlmPort` contract (completed 2026-08-23)

Full detail: [`milestones/v0.8.0-ROADMAP.md`](milestones/v0.8.0-ROADMAP.md) ·
Audit: [`milestones/v0.8.0-MILESTONE-AUDIT.md`](milestones/v0.8.0-MILESTONE-AUDIT.md) ·
Requirements: [`milestones/v0.8.0-REQUIREMENTS.md`](milestones/v0.8.0-REQUIREMENTS.md) ·
Phase artifacts: `milestones/v0.8.0-phases/`

</details>

**Security Tooling** — forward work opened by the v0.8.0 milestone audit (added 2026-08-24)

- [x] **Phase 18: Rust SAST — Evaluate and Adopt CodeQL** - Prove a Rust-capable SAST actually analyses this tree before adopting it, then wire it as a non-blocking scan and only afterwards as a required check (completed 2026-08-25)
- [x] **Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token** - Exchange the standing `CARGO_REGISTRY_TOKEN` secret for OIDC-issued ephemeral publish tokens, prove the new path works before revoking the old credential, and record the per-crate trust configuration the eleven-crate workspace needs (completed 2026-08-28)
- [x] **Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate** - Make a re-run on the same tag the supported way to finish a half-published release, refuse to publish until tag, manifest versions, changelogs and the tagged commit's CI conclusion agree, and write the stuck-halfway runbook including a yank policy (completed 2026-08-30)
- [ ] **Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables** - Build the release body from the curated `CHANGELOG.md` section instead of a commit log, and make the attached distributables real: binaries that actually compile under the features their targets require, an image bound to the release by digest, and verifiable checksums

## Phase Details

*Phases 1-4 are archived in [`milestones/v0.7.1-ROADMAP.md`](milestones/v0.7.1-ROADMAP.md).
Phases 5-17 — every phase of the v0.8.0 milestone, with their full goals, success criteria,
amendment banners and per-plan checklists — are archived in
[`milestones/v0.8.0-ROADMAP.md`](milestones/v0.8.0-ROADMAP.md), together with
[`v0.8.0-REQUIREMENTS.md`](milestones/v0.8.0-REQUIREMENTS.md) and
[`v0.8.0-MILESTONE-AUDIT.md`](milestones/v0.8.0-MILESTONE-AUDIT.md). Only phases in the current
and future milestones are detailed below, which is what keeps this file a constant size per
milestone.*

### Phase 18: Rust SAST — Evaluate and Adopt CodeQL

**Goal**: This project stops having to say "there is no static taint analysis for first-party Rust" — either because a scanner is running that provably finds real defects in *this* tree, or because the evaluation proved it does not and that verdict is recorded with its evidence. The failure mode this phase exists to prevent is the Snyk one: a scan that reads as assurance while analysing nothing.

**Depends on**: Nothing hard. The v0.8.0 milestone audit (`.planning/v0.8.0-MILESTONE-AUDIT.md`) names this as the milestone's one genuinely open item. Three soft couplings run backwards: **Phase 9/12** built `SECURITY-EXCEPTIONS.md` and `scripts/check-advisory-register.sh`, the governance shape any new scanner's findings must fit; **Phase 15.1** applied the three GitHub rulesets and wrote `scripts/check-workflow-triggers.sh`, whose `CLAUSE_CONTEXT` and reachability clauses constrain how a new required check may be added; and **Phase 16** removed the last `SNYK_TOKEN` references, so this phase starts from a clean documentation state rather than correcting one.

**Requirements**: SAST-01, SAST-02, SAST-03, SAST-04

  **New prefix, minted 2026-08-24 per *Roadmap Extension Protocol* item 3.** Nineteen prefixes are
  now spent (`RECON-*`, `GAP-*`, `QUAL-*`, `REL-*`, `VERIFY-*`, `CLOSE-*`, `ARCH-*`, `DEBT-*`,
  `SEC-*`, `HARD-*`, `FACADE-*`, `SUPPLY-*`, `ORCH-*`, `WEB-*`, `PIPE-*`, `DEFER-*`, `DOCS-*`,
  `PROV-*`, `SAST-*`). These IDs are minted **now, at roadmap time**, deliberately: Phase 15.1
  carried `Requirements: TBD` into execution and had to settle the question retroactively, which
  the v0.8.0 audit then had to record as a traceability silence (TRACE-01). This phase does not
  repeat that.

**Success Criteria** (what must be TRUE):

  1. **The scanner is proven against a deliberate-vulnerability probe before it is adopted, not after.** A Rust fixture carrying a hardcoded credential, command injection via `sh -c`, path traversal and SQL injection is scanned, and the finding count is recorded. This is the same probe methodology that disqualified Snyk (`.github/instructions/security.instructions.md`: 0 findings in Rust against 3 for the identical JavaScript, proving the scanner ran and the Rust analysis did not). **A zero-finding result is a valid and publishable outcome of this phase** — it disqualifies the tool and the manual review stays the control. Adoption without this probe is out of bounds.

  2. **If a scanner qualifies, it runs on every pull request and cannot be path-filtered into silence.** Its workflow triggers on `pull_request` with no path filter, plus `push` on `main` and a schedule. This is not stylistic: `scripts/check-workflow-triggers.sh`'s reachability clause exists because a pinned context in a path-filtered workflow never reports, leaving the PR unmergeable forever with no failing check to point at.

  3. **It runs non-blocking first, and the observation window produces numbers.** Before the scan can gate any merge, a recorded window measures its false-positive rate and its wall-clock cost against this tree's real size (385 `.rs` files, ~142k lines). A scanner is pinned as a required check on the strength of measured behaviour, never on the strength of its vendor's claims.

  4. **Promotion to a required check updates every place the check-set is written down, in one change.** The context is added to `.github/rulesets/protect-main-branch.json` (44 → 45), the live ruleset `20868126` is re-applied, `docs/src/appendix/branch-protection.md`'s context table is updated to match, and `scripts/check-workflow-triggers.sh` passes. A required-check set that disagrees with itself across those four places is the defect this criterion forbids.

  5. **`security.instructions.md`'s "Known gap: no Rust SAST" section is rewritten to match the outcome**, stating plainly what the adopted tool does and does not cover and what the manual credential-handling review still owns. The section is narrowed or replaced by evidence — never deleted to imply coverage the phase did not establish.

  6. **Nothing in this phase makes a green result mean less than it says.** If the scan is advisory, it is described as advisory. If it gates, it gates. No `continue-on-error` step is left reporting success on a job that did not run, and no documentation claims automated coverage the probe in criterion 1 did not demonstrate.

**Plans**: 7 plans

Plans:

- [x] 18-01-PLAN.md — Tracer: wire CodeQL end-to-end (workflow + trigger-policy row in one commit), the analysed-file evidence script, and the evidence document with its promotion threshold written before any measurement
- [x] 18-02-PLAN.md — The workspace-excluded probe fixture: four planted defect classes in the recorded order, a fifth behind a non-default cargo feature, and the probe/steady-state scan split
- [x] 18-03-PLAN.md — Run the probe, record the per-class result beside the coverage evidence, and settle the SAST-01 verdict
- [x] 18-04-PLAN.md — Governed alert-dismissal register plus its offline guard, regression harness and required-check wiring
- [x] 18-05-PLAN.md — Observation window: historical backfill via disposable push branches, live advisory period, and the full metric block
- [x] 18-06-PLAN.md — Correct the ruleset re-application procedure, then promote across all four recorded places or record the hold with its trigger condition
- [x] 18-07-PLAN.md — SAST-04 close: rewrite the Rust-SAST section to the measured outcome and propagate the verdict everywhere it is asserted

### Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token

**Goal**: The ability to publish `paladin-*` to crates.io stops living in a long-lived repository secret. Today `.github/workflows/release.yml:385` reads `secrets.CARGO_REGISTRY_TOKEN` — a credential with publish scope over every crate in this workspace, valid until somebody manually rotates it, readable by any workflow change that can reach that secret. crates.io Trusted Publishing replaces it with a token minted per run from a GitHub OIDC identity and expiring in roughly 30 minutes. This is the same class of standing-credential risk the `SUPPLY-*` work policed on the dependency side, applied to the publish side, which is the one direction that writes to a public registry. **The phase is not finished when the OIDC path works; it is finished when the old secret no longer exists.** A repository that has both has not reduced its attack surface — it has widened it.

**Depends on**: Nothing hard; Phase 18 and Phase 19 are independent and either order is valid. Four soft couplings run backwards. **`docs.yml`** already runs the exact GitHub OIDC pattern this phase needs — `id-token: write` at line 30 with a named `environment:` on the deploying job — so the mechanism is proven in this repository rather than novel. **Phase 9/12** established that a security control needs a recorded owner and expiry; a revoked token must be recorded as revoked, not merely unused. **Phase 15.1** wrote `scripts/check-workflow-triggers.sh`, whose clauses constrain how release workflow triggers may change. **Phase 12** deleted a job that reported success while measuring nothing — the defect class this phase must not reintroduce, because `release.yml`'s current `dry_run=skip` branch (lines 393-395) already does exactly that: when the token is absent it emits a `::warning::`, skips the publish, and the job still ends green.

**Requirements**: PUB-01, PUB-02, PUB-03, PUB-04, PUB-05

  **New prefix `PUB-*`, minted 2026-08-25 per *Roadmap Extension Protocol* item 3** — the twentieth. `SUPPLY-*` is deliberately not recycled despite the topical overlap: it is spent on Milestone 9-12 and its requirements are archived in `milestones/v0.8.0-REQUIREMENTS.md`. As with Phase 18, these IDs are minted **now, at roadmap time**, so this phase does not repeat Phase 15.1's `Requirements: TBD` and the retroactive settlement the v0.8.0 audit recorded as TRACE-01.

**Success Criteria** (what must be TRUE):

  1. **The publishable crate set is enumerated and reconciled with the publish order before any trust link is created.** Trusted Publishing is configured per crate, so the crate list stops being an implementation detail and becomes the security boundary. That enumeration must be done against `Cargo.toml`, not against the workflow — because the two already disagree. **Eleven workspace crates are publishable; `release.yml`'s `CRATES` array lists ten.** `paladin-herald` is a workspace member with no `publish = false`, is a `version`+`path` dependency of the root `paladin-ai` crate, and is absent from the publish order — which means a real `cargo publish -p paladin-ai` depends on a crate this workflow never publishes. Whether that gap is closed here or recorded and deferred, it is not carried forward unnoticed a second time.

  2. **Publishing authenticates through an OIDC exchange, not a stored secret.** The `publish-crates` job obtains its credential at run time via `rust-lang/crates-io-auth-action` and `cargo publish` consumes it. `id-token: write` is granted **on that job only** and not at workflow level — `permissions:` in `release.yml` is currently declared per job (`publish-crates` holds `contents: read`), and that shape is preserved rather than replaced with a workflow-wide grant. The job runs under a protected GitHub Environment, matching the pattern `docs.yml` already uses for Pages.

  3. **The new path is proven to publish before the old credential is destroyed, and the proof is not a dry run alone.** `cargo publish --dry-run` needs no credential at all, so a green dry run demonstrates nothing about the OIDC exchange. The evidence must show a token actually minted and accepted — a real publish, a prerelease, or an equivalent recorded exchange. Ordering is a criterion, not a preference: revoking first and testing after leaves the project unable to ship a fix until the new path is debugged.

  4. **The long-lived credential is revoked at crates.io *and* deleted from the repository's secrets, and both are recorded.** A migration that leaves the secret in place has added a publish path rather than replaced one. Revocation on crates.io is the load-bearing half — deleting the repository secret alone leaves a live token that still exists wherever else it was pasted. The date and the actor are recorded in the same manner Phase 9/12 required of advisory suppressions.

  5. **No release can silently skip publishing and still report success.** The current `dry_run=skip` branch reports a warning and exits green when no token is present — with Trusted Publishing there is no token to be absent, so that branch is either removed or rewritten to fail. A release workflow that ends green while having published nothing is the Phase 12 defect wearing different clothes, and this phase must not leave one behind.

  6. **The per-crate trust configuration is written down where the next person will look.** With eleven crates the configuration is a table, not a step: crate name, the workflow filename, the environment name, and the date the link was created — kept alongside the existing release documentation. Crate names do not match directory names in this workspace (`crates/paladin-core` publishes as `paladin-ai-core`, the root publishes as `paladin-ai`), which is exactly the kind of detail that makes an undocumented trust configuration expensive to reconstruct.

  7. **Nothing in this phase claims a protection it did not establish.** If a crate cannot be linked — for instance one never yet published, which therefore has no crates.io entry to configure trust against — that crate is named, its interim auth path is stated plainly, and it is not described as covered.

**Plans**: 5 plans

Plans:
**Wave 1**

- [x] 19-01-PLAN.md — Reconcile the eleven-crate publish set and bootstrap `paladin-herald` onto the registry (PUB-01)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 19-02-PLAN.md — Rewrite `publish-crates` to mint its credential via OIDC under the `crates-io` environment, and delete the silent-skip branch (PUB-02, PUB-05)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 19-03-PLAN.md — Create eleven trust configurations and prove the OIDC path publishes, before anything is revoked (PUB-03)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 19-04-PLAN.md — Revoke at crates.io, then delete the repository secret, recording both (PUB-04)

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 19-05-PLAN.md — Document the trust table, the credential history and the claim boundaries (PUB-05, PUB-01, PUB-04)

### Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate

**Goal**: A release that fails partway through must be finishable, and no release should begin until the things it is about to publish have been checked against each other. Neither holds today. `release.yml`'s publish loop does tolerate an already-published crate — but the workflow around it does not: `create-release` uses `actions/create-release@v1`, which returns 422 `already_exists` for a tag that already has a GitHub release, and `publish-crates` declares `needs: [test, create-release]`. **Re-running the workflow on the same tag therefore never reaches the publish step at all**, so the one recovery mechanism that exists is unreachable on exactly the run that needs it. Nor is anything verified first: the eleven manifests, the ten crate changelogs and the tag are never compared, and `ci.yml` triggers on `push: branches: [ '**' ]` — which does not match a tag ref — while `release.yml`'s own `test` job runs `cargo test --workspace` and nothing else. This phase makes the re-run the supported recovery, puts a verification gate in front of the first `cargo publish`, and writes down what an operator does when a run stops with four crates on crates.io and six not.

**Depends on**: **Phase 19, by ordering rather than by capability.** Phase 19 rewrites the same `publish-crates` job's authentication and removes or rewrites its `dry_run=skip` branch; doing this phase first means that control flow is written twice. Nothing here requires OIDC, so the reverse order works and is merely wasteful — and Phase 19's crate-set enumeration (PUB-01) settles whether the protected set is ten crates or eleven, which this phase's gate reads rather than re-deriving. Two soft couplings run backwards. **Phase 12** deleted a job that reported success while measuring nothing; the silent no-op below is that same defect in a different job — with every crate already at the published version, the loop emits ten `::warning::` lines and the run ends green. **Phase 15.1** wrote `scripts/check-workflow-triggers.sh`, whose reachability clauses constrain any trigger change or new required check this phase introduces.

**Requirements**: PUBOPS-01, PUBOPS-02, PUBOPS-03, PUBOPS-04, PUBOPS-05

  **New prefix `PUBOPS-*`, minted 2026-08-25 per *Roadmap Extension Protocol* item 3** — the twenty-first. `PUB-*` is deliberately not extended: it is spent on Phase 19 and scopes the *credential*, whereas `PUBOPS-*` scopes the *operation* — what the pipeline verifies before it publishes and what an operator does when it stops halfway. Minted now, at roadmap time, for the same reason Phases 18 and 19 minted theirs: Phase 15.1 carried `Requirements: TBD` into execution and had to settle it retroactively, which the v0.8.0 audit recorded as TRACE-01.

**Success Criteria** (what must be TRUE):

  1. **Re-running a release on the same tag is the supported recovery, and it reaches the publish step.** Today it cannot. `actions/create-release@v1` — archived upstream since 2021 — hard-fails on a tag that already carries a release, and `publish-crates` needs that job, so the publish loop's already-published tolerance is dead code on a retry. Idempotency is a property of the whole workflow, not of one bash function inside it: every job on the path from tag to `cargo publish` must be safe to run a second time on the same tag, or be made skippable.

  2. **A crate counts as already published because the registry says so, not because an error message matched a regex.** The current test is `grep -qiE "already (exists|uploaded)|is already uploaded|already published"` over the combined output of `cargo publish`. A wording change at crates.io silently converts a recoverable re-run into a hard failure, and the same loop hand-rolls its index wait as a fixed `sleep 20` between crates — a guess, not a check, and the source of the flake that produces the half-published state in the first place. Both are replaced by something that reads publication state directly; note that `ci.yml`'s `publish-dry-run` job already uses `cargo publish --workspace`, whose ordering and index-waiting semantics the release workflow's loop predates.

  3. **Nothing is published until the tag, every manifest version and every changelog agree.** This is not a hypothetical guard. The eleven workspace manifests each carry a literal `version = "0.8.0"` — none uses `version.workspace = true`, so a bump is eleven edits — the root `CHANGELOG.md` has a `## [0.8.0]` section, and **none of the ten crate changelogs do**; every one still sits at `## [Unreleased]`. A `v0.8.0` tag pushed today would publish ten crates whose own changelogs describe no such release. The gate runs before the first `cargo publish`, names every mismatch it found rather than the first, and fails the run.

  4. **"The tagged commit passed CI" is verified against a run, not inferred from a branch.** `ci.yml` triggers on `push: branches: [ '**' ]`, which does not match `refs/tags/*` — no tag push runs its eighteen jobs. `verify-tag-source` proves the commit is an ancestor of `main`, which establishes provenance, not that anything passed on it. And `release.yml`'s own `test` job runs `cargo test --workspace` alone: no `fmt`, no `clippy`, no coverage floor, no `cargo-deny`, no OSV scan. Either the gate resolves the CI conclusion for the tagged SHA and refuses to publish without a recorded success, or the release workflow runs the equivalent checks itself. Assuming is not a third option.

  5. **A run that publishes nothing does not report success, and a run that publishes some crates says which.** With every crate already at the tagged version the loop currently warns ten times and ends green — indistinguishable, from the outside, from a release that worked. The publish job's outcome states per crate exactly one of: published now, already at this version, skipped, or failed; a run in which no crate moved to `published now` is not a successful release. This is distinct from PUB-05, which governs the missing-secret `dry_run=skip` branch; this criterion governs the case where the credential worked and nothing needed publishing.

  6. **The stuck-halfway case has a written runbook, and it names a yank policy.** The word `yank` appears nowhere in `docs/src/`, `.github/workflows/` or `scripts/` today. The runbook belongs beside the existing `docs/src/appendix/release-automation.md` and `release-checklist.md`, and answers concretely: how to determine which crates reached crates.io and which did not; whether to complete forward or roll back; that a published version can never be deleted or re-uploaded, so a bad publish is corrected by a new patch version plus `cargo yank`, never by a retry of the same version; who may yank; and what is recorded when one happens — an owner and a date, the convention Phase 9/12 established for advisory suppressions.

  7. **The recovery path is exercised, not merely written.** A partial failure is induced — a rehearsal on a throwaway version, or an equivalent recorded exercise — and the documented re-run finishes it. `cargo publish --dry-run` cannot demonstrate any of this: it never reaches the registry's publish endpoint, so it can neither create the half-published state nor prove recovery from it. If the exercise is not run, the runbook is labelled untested rather than presented as a procedure — the same honesty rule Phases 18 and 19 apply to their own evidence.

**Plans:** 7/7 plans complete

Plans:
**Wave 1**

- [x] 20-01-PLAN.md — Tracer: pre-publish consistency gate wired script → make target → `release.yml` job → `publish-crates` needs edge (manifest-version clause)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 20-02-PLAN.md — Gate expansion: changelog-section clause across all eleven packages, plus CI-conclusion resolution for the tagged SHA
- [x] 20-03-PLAN.md — `create-release` idempotency: `gh api` create-or-reuse preserving the `upload_url`/`version` contract, and the release-commit SHA exposed to the gate
- [x] 20-04-PLAN.md — Release tooling: mechanical per-crate changelog finalization, and the gate wired into `make release` before the tag is pushed

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 20-05-PLAN.md — Publish loop: registry-state already-published detection, bounded index-visibility poll, per-crate outcome table and the no-crate-moved failure

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 20-06-PLAN.md — Recovery runbook: `docs/src/appendix/release-recovery.md` with the yank policy and yank register, cross-linked and registered in the book

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 20-07-PLAN.md — Recovery rehearsal against an induced partial failure, evidence file, and an honest tested/untested status line

### Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables

**Goal**: A published release should say what changed in the words this project already wrote, and hand a user something they can actually run. Neither is true today. `release.yml`'s `create-release` job builds its release body from `git log --pretty=format:"- %s" "$PREV_TAG"..HEAD` — raw commit subjects — while the repository maintains a curated Keep-a-Changelog `CHANGELOG.md` plus ten per-crate changelogs, `make release` finalizes `## [Unreleased]` into `## [VERSION] - <date>`, and `docs/src/appendix/release-automation.md` records the deliberate decision to keep authoring that file by hand rather than generate notes from Conventional Commits. The pipeline discards the artifact the project chose to curate: v0.8.0 spans 1,014 commits (`be2ff05..48ac11a5`), so the notes it would emit are a thousand-line dump including `chore:` and `docs(phase-16):` entries. **And the binaries are worse than unhelpful — they cannot be produced at all.** `[[bin]] paladin` declares `required-features = ["cli"]`, `cli` is not in `default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`, and `build-binaries` runs a bare `cargo build --release --target ...`; Cargo silently skips a binary whose required features are off, so the next step's `strip target/${{ matrix.target }}/release/paladin` has no file to strip. This phase makes the curated changelog the source of the release body and makes the attached artifacts real, verifiable, and named correctly.

**Depends on**: **Phase 20, by ordering and by shared blast radius.** Phase 20 replaces `actions/create-release@v1` to make a same-tag re-run reachable; this phase rewrites that same job's `body`, and `build-docker`, `build-binaries` and `sbom` all declare `needs: create-release` — the latter two consuming `needs.create-release.outputs.upload_url`, an output that disappears the moment `create-release@v1` is replaced. Doing this phase first means writing the artifact-upload path twice against an action that is about to be removed. Nothing here requires Phase 20's idempotency work, so the reverse order is merely wasteful. Two soft couplings run backwards. **Phase 12** deleted a job that reported success while measuring nothing; the `Verify image size` step's `::warning::` above 500 MB and the four `fail-fast: false` binary legs are that same shape — an artifact path that can produce nothing and still end green. **Phase 15.1**'s `scripts/check-workflow-triggers.sh` constrains any trigger change or new required check introduced here, exactly as it does for Phases 18-20.

**Requirements**: ARTIFACT-01, ARTIFACT-02, ARTIFACT-03, ARTIFACT-04, ARTIFACT-05, ARTIFACT-06

  **New prefix `ARTIFACT-*`, minted 2026-08-25 per *Roadmap Extension Protocol* item 3** — the twenty-second. `REL-*` is deliberately not extended: it is spent on Milestone 1 and scoped *whether the project could be released at all*. `PUBOPS-*` is likewise not reused — Phase 20 scopes what the pipeline **verifies before it publishes** and how it **recovers**; `ARTIFACT-*` scopes what the release **hands to a consumer** once publishing succeeds. Minted now, at roadmap time, for the same reason Phases 18-20 minted theirs: Phase 15.1 carried `Requirements: TBD` into execution and had to settle it retroactively, which the v0.8.0 audit recorded as TRACE-01.

**Success Criteria** (what must be TRUE):

  1. **The release body is the curated changelog section for that version, not a commit log.** The notes for `vX.Y.Z` are extracted from the `## [X.Y.Z]` section of `CHANGELOG.md` — the section `make release` creates by moving `## [Unreleased]` under a dated heading. Extraction is exact: it starts after the version heading and stops at the next `## [` heading, so a release never absorbs the section below it or the whole file. A tag whose version has no matching `CHANGELOG.md` section fails the run rather than silently falling back to `git log` — a silent fallback reproduces the current defect on exactly the release where someone forgot to finalize the changelog. Whether the ten per-crate changelogs also contribute is a decision this phase records either way; what it may not do is leave the root section unused.

  2. **Every binary the release advertises is actually built, and a build that produces no binary fails.** `cargo build --release --target <triple>` builds no `paladin` today, because `required-features = ["cli"]` is unmet under the default feature set — the `strip` step is the first thing to notice, by failing on a missing path. The build names its feature set explicitly, and the archive step asserts the expected executables exist before it tars anything. The workspace declares three binaries — `paladin` (`cli`), `paladin-cli` (`cli`) and `paladin-server` (`web-server`) — and the release currently attaches none of them; which of the three ship, and under which feature set, is decided and written down rather than left to whichever one the matrix happened to name.

  3. **The release notes do not advertise an artifact the release did not create.** The body hardcodes `docker pull ${REGISTRY}/${IMAGE_NAME}:latest`, but `latest` comes from `type=raw,value=latest,enable={{is_default_branch}}` — false on a `refs/tags/v*` push — so this workflow never pushes `latest`, and the instruction it prints has never worked on a release. Every reference in the body resolves to something the run produced: image tags that were actually pushed, asset names that were actually uploaded, and the platform list stated as fact only where a leg succeeded.

  4. **The Docker image is bound to the release, by digest.** `build-docker` needs `create-release`, pushes multi-arch images, and then records nothing back: no digest, no tag list, no link from the GitHub release to the image it was cut alongside. The immutable `sha256:` digest that `docker/build-push-action` already returns appears in the release, so a consumer can pin to the exact image this tag produced rather than to a mutable tag. The `Verify image size` step's 500 MB threshold either becomes a real failure or is stated as advisory in the body it prints — a `::warning::` in an artifact job is the Phase 12 defect again.

  5. **Attached artifacts are verifiable, and the verification instructions are in the release.** Per-asset `.sha256` files are produced today and explained nowhere. The release carries what a consumer needs to check what they downloaded — the checksums in a form that can be verified in one command, and the SBOM `sbom` already attaches identified as covering the root `paladin-ai` package rather than the whole workspace. Whether artifacts are additionally signed or carry build provenance is decided in this phase and recorded with its reasoning; deferring it is an acceptable outcome, leaving it unexamined is not.

  6. **The upload path no longer runs on archived actions, and dead matrix branches are gone.** `actions/create-release@v1` and `actions/upload-release-asset@v1` have both been archived upstream since 2021, and the `upload_url` plumbing exists only to serve them. Replacing them is coordinated with Phase 20 rather than duplicated — Phase 20 owns `create-release`'s idempotency, this phase owns the asset attachment that depends on its output. In the same pass, `build-binaries`'s `if: matrix.os != 'windows-latest'` guard on the `strip` step goes: the matrix has no Windows leg, so the condition implies a target the release does not ship and never has.

  7. **A release is proven end-to-end on a throwaway tag before this is called done.** The evidence is a real run that produced downloadable assets whose checksums verify, an image that pulls by the digest the release names, and a body that matches the `CHANGELOG.md` section for that version. A workflow that is merely re-read and reasoned about does not satisfy this — the binaries defect above survived every previous reading of this file precisely because no release since it was introduced was checked for the artifacts it claimed to attach. If the rehearsal is not run, the phase records the artifact path as unverified rather than presenting it as working — the same honesty rule Phases 18-20 apply to their own evidence.

**Plans:** 6 plans

Plans:
**Wave 1**

- [x] 21-01-PLAN.md — Curated release body: extract the `CHANGELOG.md` section, fail on a missing one, drop the unreachable pull instruction (ARTIFACT-01, -03)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 21-02-PLAN.md — Feature-correct binaries, existence assert before archive, portable checksums, `gh`-CLI uploads, dead plumbing removed (ARTIFACT-02, -05, -06)

**Wave 3** *(blocked on Wave 2 completion)*

- [x] 21-03-PLAN.md — Docker digest binding, honest image-size reporting, and the terminal `finalize-release-body` job (ARTIFACT-03, -04)

**Wave 4** *(blocked on Wave 3 completion)*

- [x] 21-04-PLAN.md — Aggregated `SHA256SUMS`, one-command verification instructions, SBOM scope stated (ARTIFACT-03, -05)

**Wave 5** *(blocked on Wave 4 completion)*

- [x] 21-05-PLAN.md — Release documentation: body source, artifact inventory, verification, signing/provenance deferral (ARTIFACT-01, -02, -05, -06)

**Wave 6** *(blocked on Wave 5 completion)*

- [ ] 21-06-PLAN.md — End-to-end rehearsal on a throwaway rc tag, or the artifact path recorded unverified (ARTIFACT-01…-06)

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|---|---|---|---|---|
| 1-4 | v0.7.1 | 38/38 | ✅ Shipped | 2026-08-04 |
| 5. Milestone 2-3 Ground Truth | v0.8.0 | 13/13 | ✅ Complete | 2026-08-05 |
| 6. Verified Gap Closure | v0.8.0 | 10/10 | ✅ Complete | 2026-08-05 |
| 7. Workspace Ground Truth & Recorded Answers | v0.8.0 | 13/13 | ✅ Complete | 2026-08-06 |
| 8. Verified Defect Closure | v0.8.0 | 9/9 | ✅ Complete | 2026-08-07 |
| 9. Release & Security Gate Integrity | v0.8.0 | 7/7 | ✅ Complete | 2026-08-08 |
| 10. Milestone 7-8 Ground Truth & Recorded Account | v0.8.0 | 11/11 | ✅ Complete | 2026-08-08 |
| 11. Facade Residue & Deferred Register Disposition | v0.8.0 | 5/5 | ✅ Complete | 2026-08-09 |
| 12. Supply-Chain Gate Integrity | v0.8.0 | 4/4 | ✅ Complete | 2026-08-10 |
| 13. Milestone 9-12 Ground Truth & Recorded Account | v0.8.0 | 13/13 | ✅ Complete | 2026-08-10 |
| 14. API Contract Truthfulness | v0.8.0 | 8/8 | ✅ Complete | 2026-08-12 |
| 15. Coverage & CI Quality Gates | v0.8.0 | 10/10 | ✅ Complete | 2026-08-13 |
| 15.1 Git & CI Governance (INSERTED) | v0.8.0 | 10/10 | ✅ Complete | 2026-08-14 |
| 16. Documentation Currency & the Architecture Gap | v0.8.0 | 14/14 | ✅ Complete | 2026-08-24 |
| 17. Additional LLM Provider Adapters | v0.8.0 | 22/22 | ✅ Complete | 2026-08-23 |
| 18. Rust SAST — Evaluate and Adopt CodeQL | v0.9.0 | 7/7 | Complete    | 2026-08-25 |
| 19. crates.io Trusted Publishing | v0.9.0 | 5/5 | Complete    | 2026-08-28 |
| 20. Release Pipeline Recovery | v0.9.0 | 7/7 | Complete    | 2026-08-30 |
| 21. Release Artifacts | v0.9.0 | 0/6 | Planned | — |

**v0.8.0 shipped 2026-08-24:** 14 phases, 149 plans, 65/65 requirements, 1,014 commits
(`be2ff05..48ac11a5`). Audit status `tech_debt` — no blockers; see
[`milestones/v0.8.0-MILESTONE-AUDIT.md`](milestones/v0.8.0-MILESTONE-AUDIT.md).

## Not In This Roadmap

Deliberate omissions, so a later reader does not mistake them for oversights.

### Shipped work — the large majority of the corpus

- **Shipped Milestone-1 work.** 98% of the milestone's task items are done. The per-requirement
  record is the *Milestone 1 as-shipped ledger* in `REQUIREMENTS.md`; re-planning it as phases
  would be fiction.

- **Shipped Milestone 2-3 work — which is nearly all of it.** Sanctum and RAG (Epics 11-12),
  Sentinel vision (Epics 13, 20), autonomous planning and handoffs (Epics 14, 21), Conclave
  (Epic 15), Council and Grove (Epic 16), the Maneuver Flow DSL (Epic 17), the CLI consolidation
  and enhancement (Epics 17.5, 18), Herald consolidation (Epic 19), the Paladin registry and
  Commander metadata export (Epic 22), the scheduler port and CLI configuration wiring (Epic 23)
  and the test/benchmark hardening (Epic 24) all have shipped artefacts in the tree. Phase 5
  verifies the record; it does not rebuild the features.

- **Shipped Milestone 4-6 work — which is all of it except five defects.** The Cargo workspace and
  every crate extraction, the feature-flag matrix and CLI feature gate, and all four Milestone 6
  relocations are **verified shipped against the tree**, not merely claimed.

- **Shipped Milestone 7-8 work — which is all of it bar six verified items.** The four crate
  extractions behind the cost-benefit gate, the `Dockerfile.chef` workspace adaptation, the ten
  per-crate Makefile targets, the five-benchmark migration, the whole `v0.1.0-rc.1` release cycle,
  the 25 List A deletions, `src/core/` reduced to exactly six files, the `use_cases` → `services`
  rename, the actix removal and cargo-deny ban, the three mounted axum delivery routes, and the
  reconciliation's fifteen commits (~10,250 net LOC removed).

- **Shipped Milestone 9-12 work — which is all of it bar the record and four defects.** The whole
  Milestone 9 orchestrator subsystem (`execute_workflow()` at
  `src/application/services/orchestration/mod.rs:382`, the `WorkflowRepository` port and its SQLite
  adapter, the content processors, the orchestrator bridge, `AuthPort` and RBAC); the whole
  Milestone 10 tooling set (pre-commit with a CI gate, cargo-audit reading `audit.toml`, cargo-deny,
  OSV-Scanner with SARIF, a CycloneDX SBOM in the release pipeline, `release.toml` with
  tag-triggered publishing, the `verify-tag-source` guard and committed GitHub rulesets); the mdbook
  with `warning-policy = "error"`, mdbook-mermaid, the full chapter hierarchy and all six
  deployment-topology pages; and the whole Milestone 12 web API (agent registry and controller,
  `paladin-server`, SSE streaming, in-process jobs, the unified error envelope, health/ready,
  request logging, CORS/body-limit/timeout layers, tower-governor rate limiting, API-key and bearer
  auth with per-agent roles, OpenAPI with a committed drift baseline, `Dockerfile.server`,
  `docker-compose.yml` and `k8s/`). **37 rows verified directly against the tree.** Phase 13 records
  them; no phase rebuilds them.

### Signals that are not work

- **Open checkbox counts as a backlog.** 542 items are unchecked across 75 task lists. Five runs of
  verification found them wrong in *both* directions — understating shipped reality (Conclave 129
  and Sanctum 111, both shipped), overstating completion (CLI isolation fully checked with three
  dependencies still unconditional), contradicted outright (Milestone 8's three), vacuous
  (Milestone 12's three are feature-branch scaffolding) and nonexistent (project-management's one is
  a formatting example inside a template). **Exactly one block survives: Milestone 11's 26**, and
  DOCS-01 owns it.

- **Milestone 5's, Milestone 6's, Milestone 9's and Milestone 10's checkbox counts** — all
  corroborated or contradicted by code, none converted into tasks.

- **`REQ-master-plan-epics-11-18` as new scope.** It is the origin document for Epics 11-18, dated
  2026-01-29; every one of those epics was ingested in run 2 and most are verified shipped. Its
  value is provenance — the dependency graph and the epic-level risk assessment — not scope.

### Relocations, not gaps

- **`STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`,
  `docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`,
  `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md`.** Absent from the paths their PRDs name,
  but shipping as mdbook chapters after the Milestone 11 overhaul — which `docs/MIGRATION_LOG.md`
  records. Recording the relocation is ARCH-05 and HARD-01; building them would be duplicate work.

- **Four stale module and document paths in run-5 requirements** — `listener_service.rs`,
  `src/application/ports/output/llm_port.rs`, `docs/Design/Design_and_Architecture.md` and the
  README demos clause. Corrected at source by ORCH-03, not rebuilt.

### Positions that would break things if implemented as written

- **The 14 requirements that shipped code superseded by outcome** (HARD-01) — actix-web as a
  `paladin-web` dependency, the `storage-sqlite` flag, the per-crate ordered publish dry run, the
  `ml` feature gate, the Milestone 8 Epic 3 no-extraction mandate, the 160-file facade target (the
  tree reads 136), and the root-path documentation deliverables.

- **A `paladin-cli` crate, MCP transport feature flags, and `vision` gating the encryption
  crates.** The last would break `cargo build --no-default-features`, because `chacha20poly1305`
  and `zeroize` serve user auth and Citadel encryption, not vision.

- **A migration between the two shipped vision surfaces.** Both ship;
  `intel/code-verification.md` records this as coexistence and says to confirm intent first.

### Explicit non-goals from the source milestones

- **Hot-reloading `config.yml`**, **terminating TLS in `paladin-server`**, **fine-grained scopes
  beyond `allowed_roles` plus the admin gate**, and **encrypting configuration at rest** — all
  Milestone 12 non-goals, recorded so they are not mistaken for omissions.

- **Rewriting the 35 mdbook appendix files** — Milestone 11 Epic 3 non-goal. One exception is under
  decision: `design-and-architecture.md`, whose relocation into that exempt chapter is precisely
  why its gap survived (DOCS-02).

- **Benchmark regression detection (`critcmp`, `github-action-benchmark`)** — Deferred-QA Epic 25
  non-goal. Note the inversion: it already ships as `benchmark-regression-signal` from Milestone 7
  Epic 3, while the `bench-check` compile prerequisite does not (PIPE-01).

- **Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml`.** None exists. The first two are
  named only by a superseded disposition record that contradicts its own governing PRD (FACADE-04);
  the third is a *placement condition* on reintroducing a removed feature (FACADE-03), not a
  deliverable.

- **A future content-delivery crate.** Reserved by Milestone 7 Epic 1 as the "correct long-term
  home" for `file_content_repository.rs`; the file was then deleted and no later document mentions
  the crate. Carried as a v2 note, not a phase.

### Decisions this roadmap records but does not take

- **Resolving the 30 competing variant groups / 69 warnings.** Recording answers is in scope
  (RECON-02 … RECON-07, VERIFY-03 … VERIFY-06, ARCH-03, ARCH-04, SEC-01, SEC-02, HARD-01 … HARD-07,
  WEB-01, PIPE-02). Picking winners inside `REQUIREMENTS.md` is not — the user has stated that
  variants are expected and that settling past disagreements is not the goal of this ingest. Where
  shipped code settles a variant, that is recorded as a **fact about the tree**, at the top of the
  precedence order, not as a decision taken here. **Group 29 is the one variant shipped code cannot
  settle**: the tree carries the Milestone 12 shape and the Milestone 9 mechanism simultaneously.

- **Promoting the eleven ADR candidates.** **Zero locked decisions exist across all 263 corpus
  documents** — no ADR-typed and no SPEC-typed document exists anywhere. Promotion requires
  re-tagging the source via `--manifest` and re-running ingest; manufacturing a lock inside a
  planning artefact would fabricate authority the corpus does not contain. SEC-01 and SUPPLY-03
  record the recommendation for the two candidates with a live operational cost — the same subject,
  from two different milestones — and do not act on it.

### Tech debt tracked as v2

- **Decomposing the three oversized service files** (2,757 / 2,294 / 1,840 lines) — real debt, no
  ingested requirement demands it.

- **Clone/lock-contention work** — the 383 `.clone()` calls and nine orchestrator locks flagged in
  `codebase/CONCERNS.md`. Blocked on Phase 3 producing benchmark evidence first.

- **The `paladin-core` / `paladin-ports` dependency allowlists** — declared 6 and 7, shipping 14 and
  10. The architectural invariant holds; this is document-versus-code drift needing ARCH-03(b) to
  choose a direction.

## Roadmap Extension Protocol

**The ingest is complete.** Five runs covered all 263 documents in `.project/` — 199 classified
(188 prose + 11 task lists) and 64 `tasks-*.md` measured deterministically. **There is no run 6.**
This section is retained because the rules below still govern any *future* addition to this
roadmap, from any source.

This roadmap is **appended to**, not restructured.

1. **Do not renumber or rewrite Phases 1-16.** Phases 1-4 are Milestone 1 close-out; 5-6 are
   Milestone 2-3; 7-8 are Milestone 4-6; 9-11 are Milestone 7-8; 12-16 are Milestone 9-12 +
   Deferred-QA. New phases start at **Phase 17** and continue upward. Use decimal insertions (e.g.
   2.1) only for urgent work that must execute *between* existing integer phases.

2. **Keep the milestone-grouped form.** Add a row to the `## Milestones` table, a labelled block
   under `## Phases`, and a new expanded `## Phase Details` section for the incoming phases. Wrap
   **only genuinely completed or superseded** milestone sections in a `<details>` block labelled
   with their milestone and status. Keep the `### Phase N: Name` header format verbatim.
   **`<details>` is a scope signal, not a rendering choice: GSD's roadmap parser strips every
   `<details>` block before phase lookup** (`stripShippedMilestones` →
   `markdown-sectionizer.stripTaggedBlocks`), so any phase wrapped in one is invisible to
   `roadmap.get-phase`, `roadmap.analyze`, and every workflow built on them — `/gsd-plan-phase`
   included. Use a plain bold label line for milestones that are not started or in progress.

3. **Add new requirement ID prefixes; do not recycle. Seventeen are spent**: `RECON-*`, `GAP-*`,
   `QUAL-*`, `REL-*` (Milestone 1); `VERIFY-*`, `CLOSE-*` (Milestone 2-3); `ARCH-*`, `DEBT-*`
   (Milestone 4-6); `SEC-*`, `HARD-*`, `FACADE-*` (Milestone 7-8); `SUPPLY-*`, `ORCH-*`, `WEB-*`,
   `PIPE-*`, `DEFER-*`, `DOCS-*` (Milestone 9-12 + Deferred-QA). Ingested `REQ-*` IDs are stable
   merge keys — match on them rather than re-deriving. **Extending an existing requirement in place
   is preferred to creating a near-duplicate**: run 4 extended ARCH-01, DEBT-01 and DEBT-03; run 5
   extended DEBT-01 again (six stale references became nine) and *corrected* SEC-01. Record the
   extension at the requirement and in the footer.

4. **Expect supersession, and record the chain.** **Zero locked decisions exist across the whole
   corpus** (0 ADR, 0 SPEC across 199 classified documents), and later milestones deliberately
   restructure earlier ones. Run 2 produced eight documented supersessions of run-1 requirements;
   run 3 produced eleven more, including the entire monolith → workspace path migration and one
   requirement a later milestone reversed outright; run 4 produced eleven more still — and the first
   case of a **document superseding another document by name**,
   `facade-cleanup-RECONCILIATION-2026-06-04.md`; run 5 produced twelve more, including the first
   case of a later run **correcting an earlier run's direct code verification**. See *Superseded but
   preserved* in `REQUIREMENTS.md`. **Relocation is not contradiction.** An ADR arriving later
   outranks anything asserted in these phases; record the supersession in `PROJECT.md` Key Decisions
   rather than silently editing a phase.

5. **Re-check the ledgers, not the phases.** If a later document claims earlier work is incomplete,
   verify against shipped code and update the relevant as-shipped ledger in `REQUIREMENTS.md`.
   Precedence for this project is **shipped tree > `.planning/codebase/` map >
   `intel/code-verification.md` > PRD > DOC > task-list checkbox.**

6. **Checkbox counts cut both ways — verify each one.** The five-run record is conclusive: counts
   understated shipped reality (runs 1-2), were accurate once and overstated once (run 3), were
   contradicted outright (run 4), and were vacuous or nonexistent (run 5). **Never convert a count
   into a requirement without checking the tree.** The trustworthy remaining-work signal in this
   corpus is the **three deferred registers** — Milestone 8's `deferred-items.md` and
   `deferred-features.md` (whose every verifiable claim matches the tree exactly, including a
   `println!` residue count exact to the occurrence), and `Deferred-QA-CICD-Completion` with
   `DEFERRED_COVERAGE.md` (whose *scope* is real and largely unbuilt, but whose *paths and numbers*
   need re-measurement) — plus the verified defects in `intel/code-verification.md`.

7. **Path claims in old PRDs are historical, including some of the newest ones.** Every
   `src/core|application|infrastructure` path in the run-1 and run-2 corpus predates the workspace
   decomposition; several run-3 paths were moved again by Milestone 6 or 8; and **four run-5
   requirements — written in June 2026 — name paths that were already gone**. Resolve current
   locations through `.planning/codebase/` or the tree, never through a PRD.

8. **Milestone numbers in source documents are not always milestone numbers.** Four instances
   exist: the M4-M6 overviews number themselves by refactoring tier, the M3 release notes assign
   Epics 19-23 to four M2 features, PRDs cross-reference "Milestone 1 / Epic 2" meaning M4 Epic 2,
   and the M7 overview titles itself "Milestone 4". In all cases the directory / task-list numbering
   is authoritative here. **A fifth was predicted in run 5 and did not occur** (ORCH-05).

9. **The Milestones 8-11 dependency graph is spent.** It described M8 → M9 **HARD**, M8 → M11
   **HARD** on path stability with M11 Epics 3-4 waiting on M9 Epics 1-3, M9 → M11 **HARD** on API
   stability, and M8 → M10 **SOFT**; critical path M8 → M9 → M11 Epics 3-5 = 11-17 sprints, M10
   entirely off it. **Run 5 confirms every dependency was honoured and every release gate was cut**
   — v0.3.0, v0.4.0, v0.5.0, v0.6.0. Keep its dependency semantics and release-gate criteria as a
   pattern; the schedule is history.

---
*Roadmap created: 2026-07-30 (ingest run 1 of 5 — `.project/Milestone_1-MVP`, 36 docs)*

*Extended: 2026-07-30 (ingest run 2 of 5 — `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs; Phases 5-6 added, Phases 1-4 unchanged)*

*Extended: 2026-07-30 (ingest run 3 of 5 — `.project/Milestone_4-Refactor-Crates-Features` +
`.project/Milestone_5-Workspace-Decomposition` + `.project/Milestone_6-Architectural-Refinements`,
32 docs; Phases 7-8 added, Phases 1-6 unchanged. Three earlier requirements were **narrowed** by
shipped-code verification rather than renumbered — RECON-02, RECON-03 and GAP-07 — and REL-02
gained the exact edition state.)*

*Extended: 2026-07-30 (ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` +
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution`, 40 docs; **Phases 9-11 added, Phases 1-8
unchanged and unrenumbered.** 16 new requirements: SEC-01 … SEC-05, HARD-01 … HARD-07,
FACADE-01 … FACADE-04. ARCH-01, DEBT-01 and DEBT-03 were **extended in place** rather than
duplicated. The Milestone 4-6 detail section was wrapped in a `<details>` block per protocol
item 2; the `### Phase N:` headers are unchanged.)*

*Extended: 2026-07-30 — **INGEST RUN 5 OF 5, FINAL. THE INGEST IS COMPLETE.**
`.project/Milestone_9-Classic-Orchestrator-Completion` +
`.project/Milestone_10-CI-Hardening-Release-Automation` +
`.project/Milestone_11-Documentation-Overhaul-Publish` + `.project/Milestone_12-Web-API` +
`.project/Deferred-QA-CICD-Completion` + `.project/project-management`, 46 docs.
**Phases 12-16 added; Phases 1-11 unchanged and unrenumbered.** 24 new requirements:
SUPPLY-01 … SUPPLY-03, ORCH-01 … ORCH-05, WEB-01 … WEB-04, PIPE-01 … PIPE-05, DEFER-01 … DEFER-03,
DOCS-01 … DOCS-04. DEBT-01 was **extended in place** a second time (six stale references became
nine) and shed its four `actions-rs` references to PIPE-04; SEC-01 was **corrected in place** —
run 4's `deny.toml`-out-of-sync finding is withdrawn, and SUPPLY-01/SUPPLY-02 carry the corrected
scope. The Milestone 7-8 detail section was wrapped in a `<details>` block per protocol item 2, and
the Overview was rewritten so this file reads as one roadmap rather than five appended fragments;
every `### Phase N:` header is unchanged and verbatim.
**Cumulative: 263 documents covered, 554 requirements, 86 forward requirements across 16 phases,
60 variant entries across 30 groups, 69 warnings, 0 blockers, 0 locked decisions, 11 ADR
candidates.***

*Corrected: 2026-07-30 (structural defect, no scope change). Runs 3, 4 and 5 wrapped the
**not-started** Milestone 1, 2-3, 4-6 and 7-8 detail sections in `<details>` blocks, citing
protocol item 2 — but item 2 reserves that wrapper for **completed or superseded** milestones, and
its claim that "downstream tooling parses it, including inside `<details>`" was false. GSD's
roadmap parser strips every `<details>` block before phase lookup, so **Phases 1-11 were invisible
to `roadmap.get-phase` and every workflow built on it**; `/gsd-plan-phase 1` failed with
`malformed_roadmap`. The four wrappers were replaced with plain bold label lines matching the
Milestone 9-12 form already used in this file, and protocol item 2 was corrected to state the
parser contract. **No phase, requirement, goal, success criterion or `### Phase N:` header was
changed** — only the four `<details>`/`<summary>`/`</details>` wrapper lines were removed. All 16
phases now resolve.*

*Extended: 2026-08-15 — **first forward addition, not ingest-derived.** Phase 17 (Additional LLM
Provider Adapters) added under a new **Provider Expansion** milestone label, per *Roadmap Extension
Protocol* item 1 ("New phases start at Phase 17 and continue upward"). Phases 1-16 unchanged and
unrenumbered; every `### Phase N:` header is verbatim. One new requirement prefix — **`PROV-*`**
(PROV-01 … PROV-04) — the eighteenth, recycling none of the seventeen spent. The phase leads with a
**provider-selection study** rather than a build list: which candidates qualify is itself the first
deliverable, and PROV-02's size is set by PROV-01's verdicts.*
