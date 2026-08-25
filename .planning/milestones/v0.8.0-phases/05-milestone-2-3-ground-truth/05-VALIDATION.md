---
phase: 5
slug: milestone-2-3-ground-truth
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-04
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

**Read this first — this phase is atypical.** Phase 5 produces **documents and decision records
only**: one 118-row cited ledger, three ADRs, one in-place ADR amendment, and one correction to a
historical release-notes document. It ships **zero product-code changes** (CONTEXT.md `<domain>`;
code consequences land in Phase 6 CLOSE-03 and Phase 15 PIPE-02). There is therefore no new test to
write and no `cargo test` that can validate a markdown ledger's citations.

`05-RESEARCH.md` §Validation Architecture reaches the same conclusion independently.

**What replaces test sampling here:** the phase's own evidence bar. CONTEXT.md D-01 requires every
`satisfied` verdict to carry a `file:line` citation **plus** a named passing test, example, or
command. Re-running that named artefact *is* the validation signal — it is a citation-correctness
check, not a regression suite.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (existing; this phase adds no tests) |
| **Config file** | `Cargo.toml` — 11 `[[test]]` targets plus autodiscovered `tests/*.rs`; note `tests/lib.rs:61` declares `pub mod integration;`, so files under `tests/integration/` do compile |
| **Quick run command** | `cargo test -p <crate> <module_path> --offline` — scoped re-run of a single cited exerciser |
| **Full suite command** | `cargo test --workspace --offline` |
| **Estimated runtime** | Scoped: seconds. Full workspace: minutes (2,924 tests at v0.7.1 close) — **not run per task in this phase** |

**No Wave 0 test scaffolding is required.** The phase writes no code, so no test file is missing.

---

## Sampling Rate

Standard per-task-commit sampling does not apply — a document commit cannot regress a test suite.
The substituted contract:

- **Per cited `satisfied` row:** the named exercising artefact must be run (or its existence and
  pass-state confirmed) **at the time the row is written**, not batched at the end. A row whose
  exerciser was never run is `present, unproven` (D-01), not `satisfied`.
- **Per plan:** re-run the scoped commands cited by that plan's rows before committing it.
- **Before `/gsd-verify-work`:** every `satisfied` row in `.planning/ledgers/milestone-02-03.md`
  must name an artefact that was actually executed, and the ledger must contain zero rows citing a
  path that does not resolve.
- **Max feedback latency:** seconds per scoped re-run; the phase never needs the full workspace
  suite as a gate.

**Commit-cadence constraint (from research, HIGH confidence).** `.pre-commit-config.yaml` sets
`always_run: true` on the `cargo-fmt` and `cargo-clippy` hooks, so they fire on **markdown-only**
commits, and `cargo fmt` was observed hanging past a 2-minute timeout on this machine during
discuss-phase. **Commit once per plan, never per ledger row**, and budget generous timeouts. If a
hook hangs, note that it stashes unstaged work to `~/.cache/pre-commit/patch*` — restore from there
rather than assuming the change was lost.

---

## Per-Task Verification Map

Populated by the planner. Because this phase emits documents, the "Automated Command" column holds
the **evidence-check** command for that task's artefact, not a test that exercises new code.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| *(planner fills)* | | | VERIFY-01 | — | N/A | evidence | `test -f .planning/ledgers/milestone-02-03.md` + per-row cited command | ❌ W0 | ⬜ pending |
| *(planner fills)* | | | VERIFY-02 | — | N/A | evidence | scoped `cargo test` per verified parent-task cluster | ❌ W0 | ⬜ pending |
| *(planner fills)* | | | VERIFY-03 | — | N/A | evidence | `grep -n` assertions on `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` + `test -f .planning/decisions/0010-*.md` | ❌ W0 | ⬜ pending |
| *(planner fills)* | | | VERIFY-04 | T-05-01 | N/A — records, does not change, crypto code | evidence | `grep -rn "EncryptionService" src/ crates/` returns zero consumers outside `src/infrastructure/security/` | ❌ W0 | ⬜ pending |
| *(planner fills)* | | | VERIFY-05 | — | N/A | evidence | `grep -n "84%" .planning/decisions/0006-coverage-gate.md` — exactly one binding floor survives | ❌ W0 | ⬜ pending |
| *(planner fills)* | | | VERIFY-06 | — | N/A | evidence | `grep -c "#\[ignore\]" tests/integration/llm_live_api_tests.rs` = 13; `grep -n "cfg(feature = \"live-api-tests\")" tests/integration/mod.rs` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

**Existing infrastructure covers all phase requirements.** No test framework install, no fixture
file, no stub is needed — this phase adds no code under test.

The one genuine prerequisite is a **document** rather than a test: the ledger scaffold
(`.planning/ledgers/milestone-02-03.md` with head notes, verdict vocabulary, and 118 keyed row
stubs, per CONTEXT.md D-20 step 1). Every later ledger plan appends into it, so it must land first.
The planner should treat it as this phase's wave-0 equivalent.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| A verdict correctly classifies a requirement as `satisfied` vs `present, unproven` vs `genuinely outstanding` | VERIFY-01 | Judgement against the D-01 evidence bar; no command can decide whether a cited test actually *exercises* the requirement's acceptance criteria | For a sample of rows, read the cited `file:line` and the named test, and confirm the test asserts the requirement's behaviour rather than merely importing the symbol |
| A block verdict (`satisfied by shipped code` / `partially outstanding`) is justified by its parent-task cluster table | VERIFY-02 | Same — cluster-level judgement, and the verdict sets Phase 6's CLOSE-02 scope | Confirm every parent task in the block appears in the table with a verdict and evidence; confirm a `partially outstanding` verdict names its failing clusters |
| The release-notes correction retains superseded text rather than rewriting history | VERIFY-03 | Convention compliance (D-08), not a testable property | Confirm the original Epic 19-23 numbering is still readable in the file, marked superseded, alongside the corrected 19-24 set |
| ADR-0006's amendment leaves exactly one binding coverage number | VERIFY-05 | RECON-07's whole purpose; a grep can count occurrences but not judge bindingness | Read the amended ADR end-to-end and confirm no second floor/target is stated as operative |

---

## Validation Sign-Off

- [ ] Every `satisfied` ledger row names an exercising artefact that was actually run
- [ ] Zero ledger rows cite a `file:line` that does not resolve in the tree
- [ ] All three VERIFY-02 blocks carry a written verdict plus a parent-task cluster table
- [ ] `.planning/decisions/0010`, `0011`, `0012` exist and match the shipped seven-heading ADR shape
- [ ] ADR-0006 amended in place; no second coverage ADR was created
- [ ] Sampling continuity: N/A — no automated task chain in this phase (documented exception, not a gap)
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s per scoped evidence re-run
- [ ] `nyquist_compliant: true` set in frontmatter

**Note for `/gsd-validate-phase`:** Phases 1-4 all closed with `status: draft` here and that was
recorded as a v0.7.1 deferred item. This file is seeded `draft` by plan-phase; promoting it is
`/gsd-validate-phase 5`'s job, and the "Sampling continuity: N/A" line above is a deliberate,
documented exception for a documents-only phase — not an unfilled gap.

**Approval:** pending
