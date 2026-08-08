---
phase: 10-milestone-7-8-ground-truth-recorded-account
verified: 2026-08-08T19:50:31Z
status: passed
score: 7/7 must-haves verified
behavior_unverified: 0
overrides_applied: 0
---

# Phase 10: Milestone 7-8 Ground Truth & Recorded Account Verification Report

**Phase Goal:** A developer can open `.planning/` and get a truthful account of the two milestones
that took this workspace to a published crate family and then cleaned up after it — which of the 86
requirements the tree satisfies, which 14 (corrected to 13) must never be implemented as written,
which document actually describes what Milestone 8 did, and what the three unresolved architecture
questions are.
**Verified:** 2026-08-08T19:50:31Z
**Status:** passed
**Re-verification:** No — initial verification

This is a record-writing phase. Its deliverables are markdown artefacts (a ledger, six ADRs, four
`.project/` annotations, a three-file config surface) rather than application code. Verification
below re-runs the cited commands directly against the repository rather than trusting SUMMARY.md's
narration, and treats the phase's own "known honest gaps" (a red `cargo doc` gate, one genuinely
outstanding requirement, three dead optional dependencies, CI-only tooling) as correctly recorded
rather than as defects — per the phase-specific verification brief.

## Goal Achievement

### Observable Truths (ROADMAP Success Criteria, HARD-01 … HARD-07)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 (HARD-01) | Every one of the 86 Milestone 7-8 requirement IDs has a `file:line`-cited verdict, and the "superseded by outcome" class (corrected from 14 to 13 rows) is unmissable | ✓ VERIFIED | `.planning/ledgers/milestone-07-08.md`: `grep -c '^| REQ-'` → 86, `grep -c '^### '` → 12, zero `pending — plan` stubs (only the explanatory sentence describing the convention matches), zero empty Verdict/Evidence cells (`awk` column check). Per-class tally re-counted independently (`satisfied` 41 + `satisfied (closed by Phase 9)` 7 + `satisfied (history)` 3 + `superseded by outcome` 12 + `relocated` 8 + `present, unproven` 6 + `diverged` 5 + `deferred with register` 3 + `genuinely outstanding` 1 = 86) — matches the ledger's own self-reported tally exactly. Head-of-file summary table holds 13 data rows (re-counted via `sed -n '365,381p' intel/code-verification.md \| grep -c '^|'` → 15 = 1 header + 1 separator + 13 data). |
| 2 (HARD-02) | The 2026-06-04 reconciliation is recorded as Milestone 8's authoritative account, superseding the Epic 1 audit and Epic 3 disposition record, with the orphan test and three in-execution corrections preserved | ✓ VERIFIED | ADR-0028 exists, `conforms`, correct 7-heading shape. Both `.project/` documents carry dated `SUPERSEDED BY [ADR-0028]` banners with original text retained (`grep -c` → 1 on each). Orphan-test re-run against `api_content_deliverer.rs`: `grep -rn "mod api_content_deliverer" src/` → zero matches, `src/infrastructure/adapters/output/` holds only `mod.rs`. Registry consolidation re-confirmed: `paladin_registry.rs` absent from the facade, `HashMapPaladinRegistry` now lives in `crates/paladin-battalion/src/in_memory_registry.rs:64`. |
| 3 (HARD-02 cont.) | Milestone 8 Epic 3/Epic 6 are not planned as outstanding; `paladin-herald` exists and the non-goal is recorded overridden for it, still holding for `paladin-ml` | ✓ VERIFIED | `ls crates/` returns eleven entries including `paladin-herald`; `test -d crates/paladin-ml` exits 1. `src/application/services/` exists with `services` module declared in eleven leaf crates' worth of sub-modules; workspace-wide `use_cases` grep returns zero matches. ADR-0028 Decision (iii)/(iv) and its ledger rows record both facts. |
| 4 (HARD-03) | The version record teaches history, not current state; REL-01 is not re-opened | ✓ VERIFIED | `Cargo.toml:34` → `version = "0.7.0"`. `git tag --sort=-v:refname \| head -3` → `v0.7.1`, `v0.7.0`, `v0.5.1` — matches ADR-0029 and the REQUIREMENTS.md correction exactly. `REQUIREMENTS.md:1445` and the traceability table confirm REL-01 is still `[x]` and untouched by this phase. |
| 5 (HARD-05) | The extracted-crate dependency rule reads the same way the tree behaves (default-build invariant, non-default optional feature permitted) | ✓ VERIFIED | `crates/paladin-content/Cargo.toml:23` `llm = ["dep:paladin-llm"]` (non-default), `:28` the optional dependency line, `src/services/mod.rs:7` `#[cfg(feature = "llm")]`. Independently re-run: `cargo tree -p paladin-content --no-default-features --offline` contains no extracted crate or facade package (only permitted `paladin-ai-core`). ADR-0031 restates the rule against exactly this evidence and names Phase 11/FACADE-02 and Phase 15 as downstream consumers. |
| 6 (HARD-06) | PDF extraction has one consistent answer, matching the RustSec suppression's stated reachability path | ✓ VERIFIED | `pdf = []` is absent from `crates/paladin-content/Cargo.toml` (`grep -cE '^pdf +=' ` → 0); `news-api = []` remains (retained comparator). `pdf-extract` remains unconditional (`:40`, no `optional = true`). `.cargo/audit.toml:26-29`'s `RUSTSEC-2026-0187` comment now states the true (unconditional-dependency, facade-gated-one-level-up) path. `crates/paladin-content/CHANGELOG.md` records the removal and the consumer-visible cost under `### Removed`, citing ADR-0032. |
| 7 (HARD-07) | `cargo doc` has one bar, applied consistently, and is settled together with the `paladin-ports` doctest exclusion | ✓ VERIFIED | `grep -n 'exclude paladin-ports' Makefile` returns nothing (exit 1); `Makefile:427-436`'s `release-check` now runs the bare `cargo test --workspace --doc`. `crates/paladin-ports/Cargo.toml` has no `[lib]` section (DEBT-03 discharged by Phase 8, correctly attributed, not claimed by this phase). ADR-0033 ratifies the zero-warning bar and records the measured debt; independently re-run `cargo doc --workspace --no-deps` this session: exits with 24 warning-lines (20 individual + 4 per-crate summaries), exact crate split `paladin-web` 13 / `paladin-ai` 3 / `paladin-battalion` 3 / `paladin-herald` 1 — matches ADR-0033's cited figures precisely. Debt is named with owner **Phase 16 / DOCS-03**, not hidden as passing. |

**Score:** 7/7 truths verified (0 present-but-behavior-unverified — this phase's deliverables are
records, not runtime behavior, so no truth required a behavioral test).

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/ledgers/milestone-07-08.md` | 86-row cited ledger, 12 sections, evidence bar applied | ✓ VERIFIED | 86 rows, 12 `### ` headings, zero blank cells, zero pending stubs, close-out amendment appended per D-00d |
| `.planning/decisions/0028-m8-reconciliation-authoritative.md` | ADR, 7-heading shape, no frontmatter | ✓ VERIFIED | Matches 0001-0027 shape exactly |
| `.planning/decisions/0029-version-trajectory-history.md` | ADR + `## Trajectory` table | ✓ VERIFIED | Present, extra `## Trajectory` heading before Considered Options |
| `.planning/decisions/0030-milestone-7-self-numbering.md` | ADR citing 0010/0014 | ✓ VERIFIED | Present, correct shape |
| `.planning/decisions/0031-extracted-crate-dependency-rule.md` | ADR, `conforms`, names FACADE-02 downstream | ✓ VERIFIED | Downstream Consumers section names Phase 11/FACADE-02 and Phase 15 explicitly |
| `.planning/decisions/0032-pdf-extraction-capability.md` | ADR, `must change`, executed | ✓ VERIFIED | Present; manifest/CHANGELOG/audit.toml changes match |
| `.planning/decisions/0033-cargo-doc-warning-bar.md` | ADR, `must change` for Makefile only | ✓ VERIFIED | Present; measured 20-warning debt independently re-confirmed |
| `.planning/decisions/PROMOTION.md` | Next free 0034, six rows registered | ✓ VERIFIED | `Next free ADR number: 0034`, rows 0028-0033 present with descriptions |
| `crates/paladin-content/Cargo.toml` | `pdf` feature deleted, `llm` feature unchanged | ✓ VERIFIED | Confirmed by direct read |
| `.cargo/audit.toml` | `RUSTSEC-2026-0187` parenthetical corrected | ✓ VERIFIED | Confirmed by direct read |
| `Makefile` | `--exclude paladin-ports` and stale echo removed | ✓ VERIFIED | Confirmed absent |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| `REQUIREMENTS.md`'s HARD-01…07 checkboxes | `.planning/ledgers/milestone-07-08.md` and the six ADRs | Dated closure notes citing artefact + command/`file:line` | WIRED | All seven closure notes name a concrete artefact and a re-runnable command; spot-checked five of seven directly |
| `REQUIREMENTS.md`'s "Milestone 7-8 as-shipped ledger" section (:3313) | `.planning/ledgers/milestone-07-08.md` | Pointer text, per D-01 | WIRED | Section reduced to a pointer; no diverging duplicate copy left behind |
| ADR-0028/0031 `Downstream Consumers` | Phase 11 requirements FACADE-02, FACADE-03(b), FACADE-04 | Named citation | WIRED | Confirmed present in both ADRs' Downstream Consumers sections |
| Four D-26 hand-off blocks | Phase 11/FACADE-02, Phase 11/FACADE-03(b), Phase 12/SUPPLY-02+03, Phase 13/ORCH-05 | `#### Hand-off to Phase N / ID` blocks in REQUIREMENTS.md | WIRED | `grep -c '^#### Hand-off to Phase 1'` → 4, each naming its receiving requirement, an artefact, and something not to re-derive |
| `.project/` superseded documents | ADR-0028 | Dated D-00c banner | WIRED | Both `facade-audit.md` and `infrastructure-adapter-disposition.md` carry the banner with original text retained |

### Data-Flow Trace (Level 4)

Not applicable — this phase produces no runtime data-flowing components (no UI, no API, no service).
The equivalent check for a record-writing phase is citation resolution, performed above: every
`file:line` citation spot-checked resolved to the claimed content, and every `ADR-NNNN` citation in
the ledger resolves to an existing file (independently re-confirmed for 0028, 0029, 0031, 0032, 0033;
the ledger's own close-out amendment additionally confirms 0024-0027 and notes 0030 is cited only by
context/requirements documents, not by a ledger row).

### Behavioral Spot-Checks / Probe Execution

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| No `.rs` file touched by this phase | `git diff --name-only 6a6f175..HEAD -- '*.rs' \| wc -l` | `0` | ✓ PASS |
| Config-surface diff matches D-23's three permitted files exactly | `git diff --name-only 6a6f175..HEAD` (non-`.planning`/`.project` files) | `Makefile`, `.cargo/audit.toml`, `crates/paladin-content/Cargo.toml`, `crates/paladin-content/CHANGELOG.md`, `crates/paladin-content/README.md` (one recorded scope addition, a `.md` file) | ✓ PASS |
| `cargo doc --workspace --no-deps` measured state matches ADR-0033's cited figures | `cargo doc --workspace --no-deps 2>&1 \| grep -c "^warning: "` and `grep "generated"` | 24 total warning-lines; `paladin-web` 13, `paladin-battalion` 3, `paladin-ai` 3, `paladin-herald` 1 | ✓ PASS (matches exactly) |
| Workspace still builds/lints clean despite manifest edit | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, no warnings | ✓ PASS |
| Formatting unaffected | `cargo fmt --check` | exit 0 | ✓ PASS |
| `paladin-content` carries no leaf-to-leaf edge in its default build | `cargo tree -p paladin-content --no-default-features --offline` | no extracted crate/facade present (only permitted `paladin-ai-core`) | ✓ PASS |
| `Makefile` no longer excludes `paladin-ports` from doc-tests | `grep -n 'exclude paladin-ports' Makefile` | no match (exit 1) | ✓ PASS |
| `paladin-ports` doctests re-enabled (DEBT-03 attribution) | `grep -n '\[lib\]\|doctest' crates/paladin-ports/Cargo.toml` | no output | ✓ PASS |
| README/CHANGELOG record the pdf removal | direct read of `crates/paladin-content/{README,CHANGELOG}.md` | Both name ADR-0032 and the consumer-visible cost | ✓ PASS |

No probes (`scripts/*/tests/probe-*.sh`) are declared by or relevant to this phase; none found under
`scripts/`.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| HARD-01 | 10-01, 10-07..10-10, 10-11 | 86-row cited ledger | ✓ SATISFIED | Ledger integrity checks above |
| HARD-02 | 10-02, 10-09, 10-11 | Reconciliation authoritative | ✓ SATISFIED | ADR-0028, banners, orphan test re-run |
| HARD-03 | 10-03, 10-11 | Version trajectory as history | ✓ SATISFIED | ADR-0029, REL-01 untouched |
| HARD-04 | 10-03, 10-11 | Fourth numbering collision | ✓ SATISFIED | ADR-0030 |
| HARD-05 | 10-04, 10-07, 10-11 | Dependency rule restated | ✓ SATISFIED | ADR-0031, `cargo tree` re-run |
| HARD-06 | 10-05, 10-11 | PDF capability answer | ✓ SATISFIED | ADR-0032, manifest/CHANGELOG/audit.toml |
| HARD-07 | 10-06, 10-11 | `cargo doc` bar | ✓ SATISFIED | ADR-0033, `cargo doc` re-run |

No orphaned requirements found: `grep -E "Phase 10" .planning/REQUIREMENTS.md` maps only HARD-01…07
to this phase, and all seven appear in every plan's `requirements:` frontmatter coverage.

### Anti-Patterns Found

None of TBD/FIXME/XXX/TODO/HACK/PLACEHOLDER found in any file this phase modified (checked all
non-`.planning` files in the diff: `Makefile`, `.cargo/audit.toml`, `crates/paladin-content/{Cargo.toml,CHANGELOG.md,README.md}`,
the six new ADRs, and the four `.project/` annotations). No debt markers without a following
issue/phase reference.

Two minor completeness observations, neither of which is a required must-have and neither of which
falsifies any of the seven HARD requirements' own closure evidence:

1. **`REQ-paladin-content-changelog-fix`** is correctly recorded `genuinely outstanding` (a fresh,
   accurately-verified finding — the CHANGELOG entry FR-8 requires is genuinely absent). Unlike the
   phase's other three "known honest gaps" (the `cargo doc` debt → Phase 16/DOCS-03; the dead
   `scraper`/`rss`/`tiktoken-rs` deps → Phase 15; `cargo audit`/`cargo deny`/Docker → CI-only,
   named), this one is not assigned to a named owning phase anywhere in the corpus (checked
   `REQUIREMENTS.md`, `ROADMAP.md`, `PROJECT.md`, and all four D-26 hand-off blocks). It is honestly
   recorded as absent, just not yet routed to a future phase the way its sibling gaps were.
2. **`ROADMAP.md`'s top-of-file phase overview** (lines ~150-151) still lists Phase 9 and Phase 10 as
   `- [ ]` unchecked, inconsistent with both phases' own detailed `### Phase N` sections, which carry
   dated completion notes and all-`[x]` plan lists. This pre-dates Phase 10 (Phase 9 has the same
   gap) and does not affect any of the seven HARD requirements' own evidence, but it is the kind of
   surface a developer skimming `.planning/` first encounters, and it currently reads as if Phase 10
   were still open.

Neither observation changes a HARD-01…07 truth's verdict, blocks a downstream phase (both items are
either self-evidently absent-and-recorded, or cosmetic), or contradicts a phase deliverable — they
are noted for completeness, not as gaps requiring a closure plan.

### Human Verification Required

None. Every truth above was verified by direct command re-execution against the current tree rather
than by trusting a SUMMARY.md claim or an ingested document's own self-assessment.

### Gaps Summary

No gaps found against this phase's seven required truths (HARD-01 through HARD-07). All six ADRs
exist in the correct house shape, `PROMOTION.md` correctly advances to 0034, the 86-row ledger is
complete and internally self-consistent (independently re-tallied to the same class distribution the
ledger itself reports), the three-file config surface exactly matches D-23's permitted boundary, zero
`.rs` files were touched, and the one recorded scope addition (a `.md` correction) is exactly the one
the phase's own context predicted. The `cargo doc` zero-warning gate is honestly recorded as
currently failing (independently reproduced: 20 warnings across the same four crates ADR-0033
names) rather than concealed — which is precisely what this ground-truth phase exists to do. The two
observations above (an unowned fresh finding, and a stale top-level ROADMAP checkbox) are minor and
do not block Phase 10's own goal or gate later phases.

---

_Verified: 2026-08-08T19:50:31Z_
_Verifier: Claude (gsd-verifier)_
