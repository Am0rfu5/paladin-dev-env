# Phase 10: Milestone 7-8 Ground Truth & Recorded Account - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-08
**Phase:** 10-milestone-7-8-ground-truth-recorded-account
**Mode:** `--auto` — all nine gray areas auto-selected; every question auto-resolved to its
recommended option. **No answer was confirmed by a human.**
**Areas discussed:** Ledger home, vocabulary and evidence bar (HARD-01) · The reconciliation's
authority (HARD-02) · The version trajectory (HARD-03) · The fourth numbering collision (HARD-04) ·
The extracted-crate dependency rule (HARD-05) · The PDF capability answer (HARD-06) · The `cargo doc`
bar and doctest posture (HARD-07) · This phase's code-change boundary · ADR allocation and plan
decomposition

---

## Ledger home, vocabulary and evidence bar (HARD-01)

| Option | Description | Selected |
|--------|-------------|----------|
| New `.planning/ledgers/milestone-07-08.md`; REQUIREMENTS.md section → pointer | Fourth sibling in the ledger series; the M4-6 ledger head note already names this file | ✓ |
| Grow REQUIREMENTS.md's existing §3121-3317 in place | Keeps one file; REQUIREMENTS.md is already 4,136 lines | |
| Both — ledger plus a retained inline copy | Guarantees two diverging records | |

**Choice:** New ledger file, REQUIREMENTS.md section reduced to a pointer (D-01).
**Notes:** Vocabulary: carry the series' seven verdict classes forward and map HARD-01's four
mandated dispositions onto them in the head note, rather than inventing a fifth vocabulary for the
fourth ledger (D-02). `superseded by outcome` gets a dedicated summary table so a planner never has
to find those rows by scanning 86. Evidence bar is Phase 7's, manifest carve-out included — an
ingest-era status word is the bare claim the bar rejects (D-03). Phase 9's seven closed rows are
cited from the `REQUIREMENTS.md:1320-1355` hand-off, not re-verified — but their citations are
re-derived (D-04). **Correction found during scouting:** the "14-row table" HARD-01 sizes the
supersession class from holds **13** data rows (D-05).

---

## The reconciliation's authority (HARD-02)

| Option | Description | Selected |
|--------|-------------|----------|
| ADR + source annotations on both superseded documents + ledger rows | Contested position — two documents assert the opposite of the tree | ✓ |
| Ledger rows and source annotations only, no ADR | Cheaper; leaves the supersession without a top-precedence record | |
| ADR only | Leaves `facade-audit.md` and the disposition record uncorrected at source | |

**Choice:** ADR-0028, plus D-00c annotations on `Epic_1/facade-audit.md` and
`Epic_3/infrastructure-adapter-disposition.md`, plus ledger rows (D-07).
**Notes:** The reconciliation's orphan-verification method is preserved verbatim as a reusable
procedure, and the three in-execution corrections (`paladin_registry.rs` was not a duplicate; the
sqlite repositories were not redundant; the rest genuinely were orphaned) get "do not re-delete"
markers in the **ledger rows**, not only in the ADR — a planner looks at the ledger before asking a
question, at the ADR after (D-08). The M8 Epic 3 §5 non-goal is recorded as **overridden for
`paladin-herald`, still holding for `paladin-ml`**; FACADE-03 is named in Downstream Consumers so
Phase 11 cannot re-open it (D-09).

---

## The version trajectory (HARD-03)

| Option | Description | Selected |
|--------|-------------|----------|
| ADR-0029 with a trajectory table ORCH-05 appends to | One home for the whole line rc.1 → 0.7.0 | ✓ |
| Ledger head note only | History without a citable, top-precedence record | |
| Separate ADRs per milestone segment | Three ADRs for one line guarantees the third contradicts the first | |

**Choice:** ADR-0029, single home, extended by Phase 13 / ORCH-05 rather than duplicated (D-12).
**Notes:** **Two corrections found during scouting.** HARD-03 describes REL-01 as downstream work
awaiting this answer; REL-01 is `[x]` at `REQUIREMENTS.md:358` with a `Complete` traceability row at
`:3913`, converged by Phase 4 on `0.7.0` via ADR-0008 — HARD-03's live job is backwards-looking
confirmation, not a hand-off (D-10). And HARD-03's current-state figures are two releases stale: the
tree is `0.7.0` (`Cargo.toml:34`) with tags `v0.7.0` and `v0.7.1` present, not `0.6.0` with `v0.5.1`
latest (D-11). The historical facts it records — the `0.2.0` target, the ten crates at `0.1.0`, tag
`v0.1.0-rc.1` at `a9530fc`, the GO sign-off, docs.rs verification — are unchanged.

---

## The fourth numbering collision (HARD-04)

| Option | Description | Selected |
|--------|-------------|----------|
| ADR-0030 citing ADR-0010 and ADR-0014 + source annotation on the M7 overview | Third application of a settled convention | ✓ |
| A footnote in the ledger head note | Breaks `REQ-*` provenance-key resolution across four ledgers | |
| Re-tag the overview via `--manifest` and re-ingest | Changes how five completed ingest runs classified their corpus | |

**Choice:** ADR-0030 with both precedents cited explicitly (D-13).
**Notes:** Directory / task-list numbering is authoritative, as in VERIFY-03 and ARCH-02. The six
"Milestones 1-3" prerequisites the M7 overview credits are all Milestone 4-6 deliverables already
ledgered in `milestone-04-06.md`; cite that ledger rather than re-asserting the mapping. The Roadmap
Extension Protocol's "expect a fifth in run 5" prediction is recorded **discharged** — run 5 found
none and ORCH-05 closed it — so no later phase inherits a standing check (D-14).

---

## The extracted-crate dependency rule (HARD-05) ⚠ HUMAN REVIEW

| Option | Description | Selected |
|--------|-------------|----------|
| "Never, except behind a non-default optional feature the facade opts into" — restate the rule | Describes the tree; makes the default-build invariant the enforceable one | ✓ |
| "Never" absolutely — remove `paladin-content → paladin-llm` from the tree | Deletes or re-architects a shipped, facade-exposed capability | |
| Re-tag the PRD via `--manifest` so §6.1 binds above PRD precedence | HARD-05 suggests it; Phase 7's D-11 settled that an ADR *is* the promotion | |

**Choice:** Restate as a default-build invariant, recorded in ADR-0031 (D-15).
**Notes:** Verified this session — `llm` is non-default (`Cargo.toml:23`), the facade opts in
explicitly (root `Cargo.toml:275`), and the edge gates exactly one `cfg`-guarded module
(`services/mod.rs:7`). **The default build of `paladin-content` carries no leaf-to-leaf edge at
all**, which is the fact the rule should be written against. ADR-0015 is the model: separate the
enforceable invariant from the list, and leave enforcement (`cargo tree --no-default-features`) to
Phase 15. Option 2 was declined as architecture work outside a ground-truth phase's boundary; option
3 was declined because re-typing a `.project/` file changes five completed ingest runs'
classifications for an outcome an ADR achieves natively. `REQ-extracted-crate-dependency-rule`
(`REQUIREMENTS.md:3159`) moves from `Code diverges` to `satisfied` **with the reason stated** — the
divergence was in the rule's wording, not in the code (D-16).
**Flagged:** this answer gates Phase 11's FACADE-02 D2/D3/D4 relocation targets. Rated `costly`.

---

## The PDF capability answer (HARD-06) ⚠ HUMAN REVIEW

| Option | Description | Selected |
|--------|-------------|----------|
| Supported unconditionally; **delete** the inert `pdf` feature | Makes the manifest true in one line; cannot change any build's behaviour except `--features pdf` | ✓ |
| Wire `pdf` to gate `pdf-extract` and add it to `content-processing` | Requires `cfg`-gating a struct field, its constructor and two call sites; makes PDF extraction opt-out for existing consumers | |
| Record the answer, change nothing | Closes HARD-06 but leaves a manifest that lies quietly | |

**Choice:** ADR-0032; the answer is "yes, unconditionally", and `pdf = []` is deleted (D-17, D-18).
**Notes:** `grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/` returns **zero**;
`adapters/document/mod.rs` declares `pub mod pdf_extractor;` unconditionally; `document_adapter.rs:22`
holds `PdfExtractor` as an ungated struct field. `news-api = []` is a second empty feature and it
**does** gate code (`adapters/input/mod.rs:5`), so "empty feature" is not itself the defect — `pdf`
is the only one inert in both directions. Phase 9's D-17 reached the same conclusion from the
manifest and handed the capability question here; this session adds the source-level half.
`.cargo/audit.toml`'s `-0187` parenthetical is corrected: it is right about reachability and
misleading about the mechanism (D-19). M7 Epic 1 §4.4.1 and §4.4.6 annotated superseded.
**Flagged:** removing a declared feature from a published crate is a minor public-contract change —
`cargo build --features pdf` starts failing where it previously succeeded-and-did-nothing. Rated
`costly`. Fallback branch (keep `pdf = []`, add it to `content-processing`, record it as a
documentation marker) is written into D-18; the plan must state which branch it took.

---

## The `cargo doc` bar and doctest posture (HARD-07)

| Option | Description | Selected |
|--------|-------------|----------|
| Zero warnings — ratify the bar CI already enforces | `ci.yml:58` runs it today in the required `lint` job | ✓ |
| Warnings acceptable (M8 Epic 5 FR-19) | The minority position; weaker than what ships | |
| Set a new bar (e.g. warning budget) | Invents a third position for a question the tree already settled | |

**Choice:** ADR-0033 ratifies zero warnings; FR-19 annotated superseded by outcome (D-20).
**Notes:** **Two findings shrank this requirement substantially.** DEBT-03 is already closed —
`crates/paladin-ports/Cargo.toml` has no `[lib]` section and `git log` shows
`2bffe22 feat(08-03): re-enable paladin-ports doctests`; `ci.yml:238` is a bare
`cargo test --workspace --doc` with no `--exclude` (the record's `ci.yml:225` citation is stale in
both line and content). The "unwritten Task 7.0" deferred since run 3 is **discharged by Phase 8**.
The entire surviving residue is `Makefile:432-433`'s `--exclude paladin-ports` inside `release-check`
and its stale echo — which also makes `release-check` weaker than CI (D-21). Separately, **seven**
crates still set `doctest = false`; that number and list are **recorded, not decided**, the
`REQ-doc-coverage-audit` row is marked `present, unproven` rather than `satisfied`, and Phase 15 is
named as owner (D-22).

---

## This phase's code-change boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Record-writing plus a three-file config surface; no `.rs` | ROADMAP criteria 5 and 6 require record and tree to agree; the cheap side to move is the record | ✓ |
| Record-only, like Phase 7 | Leaves criteria 5 and 6 unsatisfiable without a follow-up phase | |
| Full code closure, like Phase 8 | Pulls dependency-edge removal and feature rewiring into a ground-truth phase | |

**Choice:** Record-writing with a named, closed three-file surface (D-23).
**Notes:** The complete permitted surface is `crates/paladin-content/Cargo.toml:18` (delete one
feature line), `.cargo/audit.toml:26-29` (correct one comment) and `Makefile:432-433` (delete one
flag and one echo). Any plan proposing a `.rs` change has found new scope and must say so rather than
absorb it. Every plan still runs the CLAUDE.md workspace gate and the ADR-0006 84% coverage floor
re-check — expected unchanged, since no `.rs` moves.

---

## ADR allocation and plan decomposition

| Option | Description | Selected |
|--------|-------------|----------|
| Six ADRs, 0028-0033, one per contested requirement; HARD-01 gets none | Matches 0001-0027's one-question-per-ADR shape and D-00g | ✓ |
| Fewer, combined ADRs | 0031 must stay separately citable — Phase 11 depends on it by number | |
| Seven ADRs, one per HARD requirement | A ledger is not a contested position | |

**Choice:** ADR-0028 (M8 authoritative account) · 0029 (version trajectory) · 0030 (numbering) ·
0031 (dependency rule) · 0032 (PDF capability) · 0033 (`cargo doc` bar and doctest posture);
`PROMOTION.md` advances 0028 → 0034 (D-24).
**Notes:** Suggested decomposition ~9 plans in 4 waves — scaffold, a four-way ledger fan-out by epic
block (25/22/18/21 IDs), three ADR plans running parallel to the fan-out, and a close-out (D-27).
Plan ⑧ (ADR-0031/0032/0033 plus the config surface) is gated on a **blocking human checkpoint**
before its first task, because both flagged decisions land there. File contention noted: the ledger
is append-only per disjoint epic range; `REQUIREMENTS.md` is touched in three different waves.
Four forward hand-off blocks are owed, in the shape Phase 9 used for this phase (D-26).

---

## Claude's Discretion

- Placement of the `superseded by outcome` summary table — head, foot, or both.
- Whether ADR-0031/0032/0033 are three files or fold into fewer (0031 must stay separately citable).
- Exact wording of the restated dependency invariant, provided it is expressed against the default
  build and is checkable by a command.
- Banner wording and inline-correction markup for every `.project/` annotation.
- How the ledger presents the run-4 claims `intel/code-verification.md` already verified — inline per
  row or as a cross-reference block.
- Whether the `Makefile:432-433` fix rides in plan ⑧ or the close-out.
- Whether ADR-0033 also records the four crates that *do* run doctests as a positive baseline.

## Deferred Ideas

- `scraper`, `rss` and `tiktoken-rs` — three optional dependencies in `paladin-content` consumed by
  no code in the crate. Fresh finding; mirror image of HARD-06's defect; out of HARD-06's scope.
- The seven crates still setting `[lib] doctest = false` — recorded here, owned by Phase 15.
- Removing the `paladin-content → paladin-llm` edge entirely — the option D-15 declined; becomes its
  own architecture phase if D-15 is overturned.
- A `cargo tree`-based dependency-allowlist check in CI — Phase 15, from ADR-0015, now with a second
  clause from D-15. Carried forward from Phases 7, 8 and 9.
- The eight deprecated GitHub Action references — Phase 15 / PIPE-04. Carried forward.
- Stray root artefacts (`api_surface_current.txt`, `final-api.txt`, `flat`, `lcov.info`) — carried
  forward from Phase 9; two of them are cited by the run-4 supersession table.
- Replacing `dotenv` with `dotenvy` and the four other live unmaintained advisories — carried forward
  from Phase 9.
- A `SECURITY.md` for GitHub's advisory UI — carried forward from Phase 9; candidate for Phase 16.
- Retiring or replacing `src/main.rs` — carried forward unresolved from Phases 7, 8 and 9.
- Nyquist validation for Phases 1-4 — carried forward from Phases 5, 7, 8 and 9.
- Whether ADRs should be published to the mdbook — carried forward unanswered from Phases 1, 5, 7, 8
  and 9. Six phases is enough; Phase 16 should answer it or record it declined.
