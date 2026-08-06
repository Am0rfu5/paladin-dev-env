# Phase 7: Workspace Ground Truth & Recorded Answers - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-06
**Phase:** 7-workspace-ground-truth-recorded-answers
**Mode:** `--auto` — all gray areas auto-selected, every question resolved to its recommended
option without user confirmation.
**Areas discussed:** Ledger evidence bar & verdict vocabulary · Milestone/tier numbering correction
scope · The four variant-pair answers · Facade re-export policy · The five contradicted positions ·
Binary-target architecture · Build-benchmark falsifiability · ADR allocation & plan decomposition

---

## Ledger evidence bar & verdict vocabulary (ARCH-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Inherit Phase 5's bar unchanged, with a manifest carve-out | `file:line` + exercising artefact; for manifest-declaration requirements the manifest line plus a consuming CI job counts | ✓ |
| Inherit Phase 5's bar with no carve-out | Demand a runtime test for every row, including `edition = "2024"` | |
| Accept file existence for structural requirements | Lower the bar for a structural milestone block | |

**Choice:** carve-out variant (D-01).
**Notes:** M4-M6 are structural milestones, so a large share of their 115 requirements *are*
manifest declarations. Demanding a runtime test for a manifest fact pushes most of the ledger into
`present, unproven` for no information gain; accepting bare file existence reimports the exact
false-positive class the bar was written to reject. `crate-isolation` (`ci.yml:228`) and the
`feature-flags.yml` matrix supply the consuming artefact.

| Option | Description | Selected |
|--------|-------------|----------|
| Seven classes — Phase 5's five plus `relocated` and `diverged` | Keeps the "relocated, not missing" signal distinct | ✓ |
| Phase 5's five classes only | Fold relocation into `superseded by shipped code` | |

**Choice:** seven classes (D-02).
**Notes:** ARCH-05's whole point is "relocated, not missing" — collapsing the class destroys the
signal that stops a later phase planning the mdbook pages as gaps. ROADMAP criterion 1's five names
map onto the seven; the mapping goes in the ledger head note.

---

## Milestone/tier numbering correction scope (ARCH-02)

| Option | Description | Selected |
|--------|-------------|----------|
| ADR-0014 + inline correction on 5 sources + pointer banner on 7 extracts | Full treatment where content is distinct, cheap pointer where it is a copy | ✓ |
| ADR only | Record the convention, leave `.project/` untouched | |
| ADR + inline correction on all 12 documents | Rewrite the byte-equivalent extracts too | |

**Choice:** the split treatment (D-07, D-08).
**Notes:** INGEST-CONFLICTS records seven of the nineteen run-3 DOCs as byte-equivalent copies of
overview sections. Editing a copy seven times multiplies edit risk without adding information; a
one-line dated pointer stops propagation just as well. ADR-0014 must cite ADR-0010 explicitly —
that cross-reference is what makes `REQ-*` provenance keys resolve across both numbering defects.

---

## The four variant-pair answers (ARCH-03)

| Option | Description | Selected |
|--------|-------------|----------|
| (a) Cite ADR-0009; no new ADR | Phase 4 already recorded and applied edition 2024 | ✓ |
| (a) Author a Phase-7 edition ADR | Re-record what ADR-0009 already says | |

**Choice:** citation only (D-09). Verified: all twelve manifests declare `edition = "2024"`.

| Option | Description | Selected |
|--------|-------------|----------|
| (b) ADR-0015 rewrites the allowlist and separates invariant from list | Records the enforceable purity rule, the measured lists, and accepts the extras | ✓ |
| (b) State the intended six-crate target, treat the extras as tracked debt | Creates twelve debt items nobody intends to pay | |

**Choice:** rewrite against reality (D-10).
**Notes:** measured this session — `paladin-core` 14 deps, `paladin-ports` **11** (not the 10 the
intel file records; `mime_guess` is new since run 3). `tokio` in `paladin-core` is the one entry
that gets an explicit written justification.

| Option | Description | Selected |
|--------|-------------|----------|
| (c) ADR-0016 ratifies `paladin-core` ownership; PRD annotated; ADR *is* the promotion | GSD-native; avoids editing ingest manifests | ✓ |
| (c) Re-tag the Epic 1 decision record as an ADR via `--manifest` | Changes how five completed ingest runs classified the corpus | |
| (c) Follow mechanical precedence and move the types back to `paladin-ports` | Reintroduces the upward dependency the decision removed | |

**Choice:** ADR-as-promotion (D-11).
**Notes:** this is Phase 8's DEBT-05 input — canonical `TokenUsage` is
`crates/paladin-core/src/platform/container/token_usage.rs:13`.

| Option | Description | Selected |
|--------|-------------|----------|
| (d) ADR-0017 accepts v2 and states the cycle concern was real but mis-sited | Config moved *down* into `paladin-llm`; the feared cycle never existed | ✓ |
| (d) Declare Epic 4's concern simply wrong | Loses the reason the bridge was built | |

**Choice:** real-but-mis-sited (D-12).

---

## Facade re-export policy (ARCH-04)

| Option | Description | Selected |
|--------|-------------|----------|
| No-shim posture stands; ADR-0018 records it; version consequence cites ADR-0008 | PRDs and tree agree; overview is the minority position | ✓ |
| Adopt the overview's backward-compatibility position | Would require re-adding shims to shipped code | |

**Choice:** no-shim (D-13, D-14).
**Notes:** verified `src/application/` holds only `cli`, `errors`, `mod.rs`, `services` —
`use_cases/` is gone. Breaking in substance, minor bump under pre-1.0 semantics per ADR-0008. This
is ROADMAP criterion 4's single recorded answer, and Phase 11's FACADE-02 D1 input (D-16).

---

## The five contradicted positions (ARCH-05)

| Option | Description | Selected |
|--------|-------------|----------|
| Ledger rows (`diverged`) + dated source banners; no ADRs | Phase 1/5 precedent for code-settled divergences | ✓ |
| An ADR per contradicted position | Five ADRs for questions with no competing defensible side | |

**Choice:** rows plus source corrections (D-17, D-18).
**Notes:** all five re-verified against the tree this session (`vision = []` at `Cargo.toml:274`;
no MCP flags; `web-server` axum-only at `:276`; no `paladin-cli` crate; no `use_cases/`). The four
relocated doc deliverables get `relocated` rows and one shared head note; M6 Epic 4 FR-4.12 is
re-pointed at `docs/src/api-reference/stable-api.md`.

---

## Binary-target architecture (ARCH-06)

| Option | Description | Selected |
|--------|-------------|----------|
| ADR-0019 ratifies three targets with a purpose each; mdbook page deferred to Phase 16 | Keeps Phase 7 record-only | ✓ |
| ADR + write the mdbook page now | Crosses the phase's record-only boundary | |
| Ledger row only | FR9.3 asked for a documented architecture answer, not a status line | |

**Choice:** ADR now, mdbook later (D-19, D-21).
**Notes:** the finding that made this area worth its budget — `src/main.rs` is the *pre-Paladin
content aggregator* (`#[structopt(name = "smartcontent-aggregator")]`, `setup_and_run`), and
`structopt`'s only consumer in the whole tree is that file. So the recorded "three-line fix" for
CLI dependency isolation has a precondition Phase 8 does not know about (D-20).

---

## Build-benchmark falsifiability (ARCH-07)

| Option | Description | Selected |
|--------|-------------|----------|
| Restate SM-7 per scenario using the report's own five figures | Deterministic, needs no historical tree | ✓ |
| Re-measure against the mid-tree baseline the report recommends | Requires resurrecting the pre-workspace monolith | |
| Record the contradiction and leave the target unresolved | Leaves SM-7 unfalsifiable, which is the defect | |

**Choice:** restate (D-22, D-23, D-24).
**Notes:** the ≥ 50% figure is a comparison against a monolith that no longer exists; this
environment carries the same offline/no-Docker constraints that halted Phase 1's coverage
measurement. The recommended re-measurement is **declined with a recorded reason** rather than
passed forward as an unfundable task — a deliberate difference from Phase 5's D-14a, where the
deferred work was achievable.

---

## ADR allocation & plan decomposition

| Option | Description | Selected |
|--------|-------------|----------|
| Seven ADRs, 0014-0020, one question each | Matches 0001-0013; 0016 must stay separately citable for Phase 8 | ✓ |
| Fewer, combined "run-3 answers" ADRs | Phase 8 depends on ARCH-03(c) by number | |

**Choice:** seven (D-25). `PROMOTION.md`'s next-free line advances to 0021.
**Notes:** ~11-12 plans suggested (D-27) — scaffold, five epic fan-out plans across 13 epics,
three decision plans, one-to-two ARCH-05 source-correction plans, one summary. ADR-0016 must land
before Phase 8 is planned at all.

---

## Claude's Discretion

- Banner wording and inline-correction markup for every `.project/` edit.
- Whether ADRs 0015/0016/0017 are three files or one combined ADR (0016 must remain separately
  citable).
- How the ledger presents the 22 already-verified run-3 claims — inline or as a cross-reference
  block.
- Whether `present, unproven` and `diverged` counts become headline figures in the ledger summary.
- Ordering within the epic fan-out.
- Whether the `STRUCTURE.md` correction rides in the scaffold plan or gets its own.

## Deferred Ideas

- `TokenUsage` consolidation → Phase 8 / DEBT-05 (Phase 7 only names the canonical type).
- `api-surface` CI job, `#[deprecated]` annotations, `paladin-ports` doctests, leaked CLI deps →
  Phase 8 / DEBT-01…DEBT-04.
- `cargo tree`-based allowlist enforcement in CI → Phase 15.
- The user-facing binary-architecture mdbook page → Phase 16.
- Re-measuring the build benchmark against a mid-tree baseline → **declined with reason** in
  ADR-0020, recorded so it is not silently forgotten.
- Retiring or migrating `src/main.rs` → new scope exposed by ARCH-06; belongs with Phase 8's
  CLI-isolation work or its own phase.
- `paladin-herald` shipping `colored` and `comfy-table` unconditionally with no `[features]`
  section → code change outside this phase.
- Nyquist validation for Phases 1-4 → `/gsd-validate-phase`.
- Publishing ADRs to the mdbook for framework consumers → carried forward unanswered from Phases 1
  and 5; belongs with Phase 16.
