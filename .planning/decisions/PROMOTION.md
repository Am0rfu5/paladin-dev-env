# ADR Conventions and Promotion

This file is the small shared index that every phase appending to `.planning/decisions/` reads
before writing an ADR. It answers four questions the numbering scheme, the required headings, and
the supersession mechanism raise once — so Phases 5, 7, 10 and 13 do not have to re-derive them.

## Numbering scheme

ADR files use a **flat, zero-padded, monotonic counter**: `NNNN-kebab-slug.md`.

Chosen over a phase-scoped prefix (`p01-…`, `p05-…`) because a phase prefix breaks the moment an
ADR is superseded by a *later* phase's ADR — the reader would have to know which phase number is
"newer" rather than just comparing the counter. A flat counter surviving Phases 1, 5, 7, 10 and 13
needs only one shared piece of state: the next free number, tracked below.

## Numbering index

Reserved for Phase 1 (this phase authors ADR-0005 only; 0001-0004 and 0006 are reserved slots for
the plans that follow in this same phase):

| Number | Slug | Subject |
|---|---|---|
| 0001 | `battalion-config` | `BattalionConfig` field set (RECON-02) |
| 0002 | `battalion-result` | `BattalionResult` field set (RECON-03) |
| 0003 | `formation-min-paladins` | Formation minimum Paladin count (RECON-04) |
| 0004 | `temperature-validation` | Provider-aware temperature range (RECON-05) |
| 0005 | `herald-trait` | `Herald` trait signature (RECON-06) |
| 0006 | `coverage-gate` | Project-wide test coverage gate (RECON-07) |
| 0007 | `battalion-cancellation-deferral` | Battalion-wide cancellation deferred to Phalanx-only, per D-05/D-08 (Phase 2) |
| 0008 | `workspace-version-0-7-0` | Workspace version converges on 0.7.0, per D-01/D-02 (Phase 4) |
| 0009 | `workspace-rust-edition-2024` | Workspace Rust edition converges on 2024, per D-04/D-06 (Phase 4) |
| 0010 | `milestone-3-epic-numbering` | Milestone 3 epic numbering (Phase 5) |
| 0011 | `vision-port-surfaces` | Vision port surfaces and the encryption-at-rest disposition (Phase 5) |
| 0012 | `live-api-test-key-behaviour` | Live-API-test missing-key behaviour (Phase 5) |
| 0013 | `grove-routing-model` | Grove routing model from configuration, per D-01/D-02/D-03 (Phase 6) |
| 0014 | `milestone-4-6-tier-numbering` | Milestone 4-6 tier numbering convention (Phase 7) |
| 0015 | `core-ports-dependency-allowlist` | `paladin-core` / `paladin-ports` dependency allowlist and purity invariant (Phase 7) |
| 0016 | `port-value-type-ownership` | Port value-type ownership — `paladin-core` canonical (Phase 7) |
| 0017 | `llm-config-bridge-location` | LLM configuration bridge location (Phase 7) |
| 0018 | `m6-facade-reexport-policy` | Milestone 6 facade re-export policy and its version consequence (Phase 7) |
| 0019 | `binary-target-architecture` | Binary-target architecture and per-binary purpose (Phase 7) |
| 0020 | `build-benchmark-per-scenario` | Build-time benchmark target restated per scenario (Phase 7) |
| 0021 | `cli-application-layer-placement` | CLI placement in the application layer (Phase 7) |
| 0022 | `deprecation-requirement-withdrawal` | Milestone 4 Epic 2 FR-8 deprecation requirement withdrawn (Phase 8) |
| 0023 | `cli-dependency-isolation` | CLI dependency isolation and the binary/Herald surface (Phase 8) |
| 0024 | `rustsec-exception-governance` | RustSec exception governance register, schema and disposition (Phase 9) |
| 0025 | `licence-posture` | Project licence posture — `MIT OR Apache-2.0` (Phase 9) |
| 0026 | `crate-name-collision-guard` | crates.io package-name collision guard (Phase 9) |
| 0027 | `dockerfile-chef-planner-stage` | `Dockerfile.chef` planner-stage supersession of M7 Epic 2 FR-01 (Phase 9) |
| 0028 | `m8-reconciliation-authoritative` | Milestone 8's authoritative account — the 2026-06-04 reconciliation supersedes the Epic 1 audit and Epic 3 disposition record (Phase 10) |
| 0029 | `version-trajectory-history` | Version trajectory — `v0.1.0-rc.1` recorded as closed history, with a `## Trajectory` table extended by Phase 13 (Phase 10) |
| 0030 | `milestone-7-self-numbering` | Milestone 7's self-numbering collision — directory numbering is authoritative, citing ADR-0010 and ADR-0014 (Phase 10) |
| 0031 | `extracted-crate-dependency-rule` | The extracted-crate dependency rule restated as a default-build invariant (Phase 10) |
| 0032 | `pdf-extraction-capability` | PDF extraction is unconditional; the inert `pdf` feature deleted and the `RUSTSEC-2026-0187` reachability path corrected (Phase 10) |
| 0033 | `cargo-doc-warning-bar` | One `cargo doc` bar ratified, the measured warning residue recorded with an owner, DEBT-03 discharged (Phase 10) |
| 0034 | `d1-d4-facade-relocation-disposition` | D1–D4 verdicts — `src/core/` shims defer, the `user_service.rs` split withdrawn, D3/D4 defer-with-trigger under ADR-0031 (Phase 11) |
| 0035 | `paladin-ml-leaf-crate-placement` | The `paladin-ml` leaf-crate placement condition for a future ML adapter, promoted out of DOC precedence (Phase 11) |
| 0036 | `audit-suppression-single-source-topology` | The audit-suppression single-source topology invariant — which files may legally carry a `cargo audit`/`cargo deny` suppression — promoted out of PRD precedence (Phase 12) |

**Next free ADR number: 0037**

*Dated note, 2026-08-09 (plan 12-04):* the line advances by **one**, from 0036 to 0037, because
Phase 12 authored ADR-0036 (plan 12-03, the audit-suppression single-source topology invariant). The
ADR carries a `conforms` verdict, so — like Phase 11's two — it instructs no code change: this phase
changed zero executable Rust. `ls .planning/decisions/0036-*.md` (re-run before writing this note)
returned `.planning/decisions/0036-audit-suppression-single-source-topology.md`, confirming the file
exists with a contiguous number, none skipped or reused, and no existing index row above was
renumbered, reworded or reordered. This is the first advancing note since Phase 7's to cover exactly
**one** ADR rather than a multi-ADR batch. And unlike Phase 11's note above, which stated that
neither of its ADRs closed an inventory entry — **this ADR does close a Part B entry**, candidate 7,
noted at its own entry below.

*Dated note, 2026-08-08 (plan 11-05):* the line advances by **two** in one phase, from 0034 to
0036, because Phase 11 authored ADR-0034 (plan 11-02, the D1–D4 disposition set) and ADR-0035
(plan 11-03, the `paladin-ml` placement condition) across its own plans. Both carry `conforms`
verdicts, so unlike Phase 10 neither instructs a code change (D-13 — this phase changed zero
executable Rust). `ls .planning/decisions/0034-*.md .planning/decisions/0035-*.md` (re-run before
writing this note) confirms both files exist with contiguous numbers, none skipped or reused, and
no existing index row above was renumbered, reworded or reordered. **Neither ADR closes an entry
in Part B's eleven-candidate inventory below** — `deferred-features.md` is not among the eleven
listed candidates, so no Part B "Closed by" note is added by this phase.

*Dated note, 2026-08-08 (plan 10-11):* the line advances by **six** in one phase, from 0028 to
0034, because Phase 10 authored all six of ADR-0028 through ADR-0033 across its own plans — a
fifth phase, after Phases 1, 8 and 9, whose executing phase is also each ADR's owning phase.
Plan 10-02 authored ADR-0028; plan 10-03 authored ADR-0029 and ADR-0030 (citing ADR-0010 and
ADR-0014 as its own two precedents); plan 10-04 authored ADR-0031; plan 10-05 authored ADR-0032;
plan 10-06 authored ADR-0033. This matches the precedent Phase 9's own note above established for
four ADRs in one phase, now extended to six. **Two of the six carry `must change` verdicts,
executed by the same phase that authored them:** ADR-0032 (`must change`, the `pdf = []` feature
deletion and the two config corrections, executed by plan 10-05) and ADR-0033 (`must change` for
the `Makefile` fix only, executed by plan 10-06 — the warning residue itself is left to Phase 16 /
DOCS-03, not executed here). `ls .planning/decisions/00{28,29,30,31,32,33}-*.md` (re-run before
writing this note) confirms all six files exist with contiguous numbers; none of the six was
skipped or reused, and no existing index row above was renumbered, reworded or reordered.

*Dated note, 2026-08-08 (plan 09-07):* the line advances by **four** in one phase, from 0024 to
0028, not by the single number a reader might expect from one phase's close-out. Phase 9 authored
all four of ADR-0024 through ADR-0027 across its own plans (09-02, 09-05, 09-04, 09-03
respectively) — a fourth phase, after Phases 1 and 8, whose executing phase is also each ADR's
owning phase. None of the four numbers was skipped or reused; each is a distinct decision (the
RustSec exception register, the licence posture, the crates.io name-collision guard, and the
Dockerfile planner-stage supersession) that this phase's own plans both authored and executed the
code consequences of, matching the precedent Phase 8's D-22 established for 0022/0023.

*Dated note, 2026-08-06 (plan 07-13):* the line advances to **0022**, not 0021, because D-25a
(`07-CONTEXT.md`) allocated an eighth ADR — 0021, promoting Part B candidate 2 below — beyond the
seven D-25 originally reserved (0014-0020). A reader who expects the jump from thirteen prior ADRs
to land on 0021 should read this note rather than treat 0022 as a skipped number.

*Dated note, 2026-08-06 (plan 08-09):* the line advances again, to **0024**. Phase 8 consumed both
0022 and 0023 — the first two ADRs in this corpus whose executing phase is their own (D-22): Phase 8
both authored them (plan 08-04) and performed the `must change` work they record (plans 08-06, 08-07,
08-08). Neither number was skipped or reused.

Phases 5, 7, 10 and 13 take the next free number from this line when they author further ADRs —
they do not need to `ls` the directory to find it. Each phase updates this line when it appends.

## Required heading set

Every ADR uses the following H2 headings, in this order:

- `## Status`
- `## Context`
- `## Decision`
- `## Considered Options`
- `## Code Locations`
- `## Code Conformance`
- `## Downstream Consumers`

`## Code Locations` and `## Considered Options` are **bulleted lists, never prose paragraphs** —
`.claude/gsd-core/bin/lib/adr-parser.cjs`'s `splitEntries` only yields structured entries from
bullet or numbered lines; a paragraph collapses into one opaque blob and defeats the whole point of
citable, checkable entries.

`## Code Conformance` and `## Downstream Consumers` have no synonym in `adr-parser.cjs`'s
`CANONICAL_HEADERS` table and land in the parser's `unmapped_headers` bucket. That is acceptable —
nothing currently consumes either field programmatically — but they are still required, since
`## Code Conformance` is D-03's contract (every ADR MUST carry a `conforms` / `must change` verdict)
and `## Downstream Consumers` names who reads the decision next.

## Supersession mechanism

Exactly one live ADR answers each question at any time. When a later ADR supersedes an earlier one:

- The **superseded ADR keeps its file** — it is never deleted or renamed.
- Its `## Status` body becomes the bare word `Superseded`, followed by a prose line naming the
  superseding ADR's number and the reason it no longer holds.
- The **superseding ADR** carries a `## Supersedes` line naming the ADR number it replaces.
- `adr-parser.cjs` recognises `superseded` as a status word (see `STATUS_REJECT_SET` /
  `parseStatusFromSections`), so a downstream consumer can mechanically tell a live ADR from a
  retired one without reading prose.

## Promotion procedure for existing ADR candidates

**Phase 1 promotes none of the eleven existing ADR candidates.** Each candidate stays with its
owning phase, listed in the inventory below — Phase 1 builds the mechanism (this file, and the
worked example at `.planning/decisions/0005-herald-trait.md`) but does not use it on any of the
eleven itself.

### Part A — the procedure

An owning phase promotes one of its candidates into `.planning/decisions/` by:

1. Taking the next free number from the **Numbering index** line above and decrementing nothing —
   numbers are never reused, even if a candidate is later rejected instead of accepted.
2. Authoring the candidate's substance into the standard heading set (`## Status` · `## Context` ·
   `## Decision` · `## Considered Options` · `## Code Locations` · `## Code Conformance` ·
   `## Downstream Consumers`), following `0005-herald-trait.md`'s shape.
3. Setting `## Code Conformance` to `conforms` or `must change` per D-03 — naming the executing
   requirement (e.g. a `GAP-*`, `ARCH-*`, or phase-specific ID) where the verdict is `must change`.
4. Citing the source document's path in `## Code Locations` alongside the shipped-code citations,
   so a reader can trace the promoted decision back to the corpus document it came from.
5. Updating the `Next free ADR number` line in this file.
6. Adding a row to `.planning/PROJECT.md`'s `## Key Decisions` table, linking to the new ADR file.

**Why this is viable now, where it previously was not.** Before this phase, promoting a candidate
required re-tagging its source document via `--manifest` and re-running the ingest classifier —
and the ingest is closed (STATE.md: "there is no run 6"). That path no longer exists. It is not
needed either: ADRs now live in `.planning/decisions/` as their own document class, independent of
the ingest manifest, and top the precedence order (D-01, D-02). Promotion is now an ordinary write
to a directory plus a table row — the same six steps any of Phase 1's six ADRs already followed.

### Part B — the inventory

One entry per candidate. Each carries the source document path, the ingest run that surfaced it,
what it decides in one line, and an explicit **Owner phase**.

1. **`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`** (run 3) —
   settles where `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError` and `HandoffError`
   live (`paladin-core`); shipped code implements it, but a later PRD outranks it on paper.
   **Owner phase: Phase 7. Closed 2026-08-06 by ADR-0016** (`0016-port-value-type-ownership.md`).
2. **`Epic_17.5/epic17-5.md`** (run 2) — the CLI belongs in `src/application/cli` as an input
   adapter, not infrastructure; already applied in code, also outranked by a PRD that says
   otherwise. **Owner phase: Phase 7. Closed 2026-08-06 by ADR-0021**
   (`0021-cli-application-layer-placement.md`).
3. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) — formal risk acceptance of two
   RustSec advisories, owner Platform Security, **review/expiry target 2026-09-30 — the only dated
   item in the entire 263-document corpus**. **Owner phase: Phase 9. Closed 2026-08-08 by ADR-0024**
   (`0024-rustsec-exception-governance.md`) — renewed to per-advisory `2026-12-31` review dates,
   owner reassigned to `DF3NDR`.
4. **`Milestone_7/Epic_1/cost-benefit-assessment.md`** (run 4) — go/defer scoring for four
   candidate crate extractions, with a named approver and a Self-Approval block. **Owner phase:
   Phase 10.**
5. **`Milestone_7/Epic_4/license-compatibility-decision-checklist.md`** (run 4) — licensing policy
   accepting MPL-2.0 with a 551-package inventory, approver `DF3NDR`. **Owner phase: Phase 10.**
   **Promoted early, 2026-08-08, by Phase 9 rather than Phase 10 — Closed by ADR-0025**
   (`0025-licence-posture.md`) — SEC-02, sequenced ahead of Phase 10 in the actual execution order,
   confirmed the checklist's `MIT OR Apache-2.0` over the PRD's single-licence claim. The original
   "Owner phase: Phase 10" assignment is retained above rather than rewritten; this note records
   that the promotion happened two phases earlier than planned, not that the assignment was wrong.
6. **`Milestone_8/facade-cleanup-RECONCILIATION-2026-06-04.md`** (run 4) — supersession notice that
   corrected two prior documents and resolved six open decisions in execution. **Owner phase:
   Phase 10.**
7. **`Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8** (run 5) — the
   audit-suppression single-source invariant (exceptions live only in `audit.toml` and
   `deny.toml`), currently violated by the tree. **Owner phase: Phase 12. Closed 2026-08-09 by ADR-0036**
   (`0036-audit-suppression-single-source-topology.md`) — promoted with a `conforms` verdict
   because the tree already satisfies the invariant, and a regression guard
   (`scripts/check-workflow-suppressions.sh`) now enforces it.
8. **`Milestone_12/Epic_1/prd-agent-registry-execution-api.md` §7 + OQ-2** (run 5) — the
   `AgentProvisioner` placement, currently recorded as a default rather than a decision.
   **Owner phase: Phase 13.**
9. **`Milestone_9/Epic_4/prd-agent-orchestrator-bridge.md` §6.1** (run 5) — the bidirectional
   content/agent bridge decision; the cleanest ADR-shaped section anywhere in the corpus (a
   four-criterion comparison table with a `(CHOSEN)` column). No open forward requirement currently
   points at it, so this owner assignment is Claude's Discretion rather than a CONTEXT.md-recorded
   answer: grouped with candidate 8 under the same Milestone 9-12 close-out phase, since both are
   run-5 Milestone 9/12 subjects. **Owner phase: Phase 13.**
10. **`Milestone_9/Epic_5/prd-user-admin-system-completion.md` §6.1** (run 5) — the
    opaque-bearer-token decision; the only decision in the corpus a later milestone contradicts in
    prose while silently preserving in code. **Owner phase: Phase 14.**
11. **`Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md`** (run 5) — the coverage deferral record,
    with a named sign-off and an unreached "Next Review" trigger; weaker than the others, since its
    two module paths are stale and its baselines predate Milestone 9. **Owner phase: Phase 15.**
