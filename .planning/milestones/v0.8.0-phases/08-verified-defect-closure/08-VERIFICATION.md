---
phase: 08-verified-defect-closure
verified: 2026-08-07T22:33:00Z
status: passed
score: 7/7 must-haves verified
behavior_unverified: 0
overrides_applied: 0
---

# Phase 8: Verified Defect Closure Verification Report

**Phase Goal:** The five defects that direct code verification proved open are fixed, so the guards the project believes it has actually work — and no shipped surface is removed without a recorded decision behind it.
**Verified:** 2026-08-07T22:28:15Z (initial pass), re-verified 2026-08-07T22:33:00Z (item 7 gap-closure check, after commit `887bd12`)
**Status:** passed
**Re-verification:** Yes — item 7 only, after gap closure

## Goal Achievement

All five numeric ROADMAP.md success criteria were independently re-run against the current tree (not transcribed from any SUMMARY) and all five hold. The goal's second clause (recorded-decision-behind-every-shipped-surface-change) also holds for the three surfaces that changed. One gap was found on the initial pass — a leftover self-contradiction in the phase's own provenance record — and has since been closed by commit `887bd12`, re-verified below. Nothing in code, CI, manifests, or any ADR changed between the two passes; only `887bd12` landed.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | ROADMAP criterion 1: `api-surface` guard passes on an unchanged tree, fails on a real change, and all nine stale `project/current-exports.txt` tooling+requirement references are gone | ✓ VERIFIED | Re-run `bash scripts/check-api-surface.sh .project/current-exports.txt` → `✅ API surface unchanged`, exit 0. `grep -rn 'project/current-exports.txt' scripts/ .github/ \| grep -v '\.project/'` → empty. Both-direction proof and negative-probe evidence recorded in 08-02-SUMMARY.md, independently re-confirmed by re-running the positive direction here. |
| 2 | ROADMAP criterion 2: zero `#[deprecated]` attributes, recorded as a withdrawn requirement's outcome (not an unfinished task), with no document promising a timeline the tree cannot start | ✓ VERIFIED | `grep -rn '#\[deprecated' src crates \| wc -l` → `0`. ADR-0022, `DEPRECATIONS.md`, and `stable-api.md` all cite ADR-0022 and each other (`grep -c 'ADR-0022'` → 18 / 1 / 3 respectively). Five-point cross-document agreement table in 08-06-SUMMARY.md independently spot-checked (points 1, 3, 5 re-read directly). |
| 3 | ROADMAP criterion 3: `paladin-ports` doctests run in the workspace doctest command, ~25 port traits' rustdoc examples compile and execute | ✓ VERIFIED | `grep -c 'exclude paladin-ports' .github/workflows/ci.yml` → `0`; `.github/workflows/ci.yml:226` reads `cargo test --workspace --doc` with no exclusion. `crates/paladin-ports/Cargo.toml` carries no `doctest` key. Re-ran `cargo test --offline -p paladin-ports --doc` myself: `96 passed; 0 failed; 94 ignored` — matches the claimed figure exactly, not transcribed. |
| 4 | ROADMAP criterion 4: a library-only downstream consumer compiles zero `structopt`/`colored`/`comfy-table` | ✓ VERIFIED | Re-ran `cargo build --offline --lib --no-default-features` → exit 0. `cargo tree --offline --no-default-features -e normal \| grep -E 'structopt\|colored\|comfy-table'` → no output (exit 1/no match). Confirmed the documented nuance myself: the literal command without `-e normal` does show one hit (`colored v3.1.1`), and `cargo tree --offline --no-default-features -i colored` traces it to `mockito v1.7.2`'s `[dev-dependencies]` edge on `paladin-ai` — a dev-only edge Cargo never propagates to a downstream consumer, exactly as ADR-0023/08-07/08-08 record. Not accepted on the SUMMARY's word; independently traced. |
| 5 | ROADMAP criterion 5: exactly one `pub struct TokenUsage`, no conversion needed crossing the battalion/ports boundary | ✓ VERIFIED | `grep -rn 'pub struct TokenUsage' crates src \| wc -l` → `1`, sole hit `crates/paladin-core/src/platform/container/token_usage.rs`. `VisionTokenUsage` (out-of-scope) untouched, confirmed present at `vision_port.rs:34`. |
| 6 | Goal clause 2: the three shipped-surface changes (FR-8 deprecation withdrawal, `paladin` binary `--features cli` gate, `paladin-herald`'s shrunk default API) each have a recorded ADR and a `CHANGELOG.md` entry | ✓ VERIFIED | ADR-0022 and ADR-0023 exist, corpus-shaped (7 `##` headings each, no frontmatter), both `Accepted`. `CHANGELOG.md` `[Unreleased]` carries both DEBT-04 breaking-change entries citing ADR-0023 (`grep -c 'ADR-0023' CHANGELOG.md` → 2); the deprecation withdrawal is recorded in ADR-0022 + the three-way reconciliation rather than a CHANGELOG entry (correctly — withdrawing an unimplemented requirement is not a shipped-behavior change requiring a changelog entry, and no false claim was ever shipped to correct). |
| 7 | The phase's own close-out records are internally consistent and do not overstate human authority for decisions the human did not make | ✓ VERIFIED (gap closed by `887bd12`) | **Initial finding (2026-08-07T22:28:15Z):** commit `847210a` corrected the overstated "at the human's request" claim in `REQUIREMENTS.md` and `08-09-SUMMARY.md`, and corrected `deferred-items.md`'s section header, but left `deferred-items.md:58-60`'s body sentence uncorrected — it still read "the human asked that this residual not be left with no assigned owner," directly contradicting its own header three lines above. **Closure, re-verified this session:** re-read `deferred-items.md:50-70` directly — the body sentence now reads "The approving human selected plain 'Approve and seal' and did **not** request an owner for this residual — a separate residual-assignment option was offered at that checkpoint and was not chosen. The recommendation below is the **orchestrator's own initiative** at seal time... It carries no human authority and Phase 13's planner is free to reject it," which matches the section header and the corrected passages in `REQUIREMENTS.md:909` and `08-09-SUMMARY.md`. Ran `grep -rn -i "the human asked\|at the human's request\|human requested" .planning/phases/08-verified-defect-closure/ .planning/REQUIREMENTS.md` — the only surviving hits are inside this VERIFICATION.md's own prior-finding text (a legitimate historical quote of the now-corrected wording), confirmed by direct inspection. The file is now internally consistent. |

**Score:** 7/7 truths verified (0 present-but-behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `crates/paladin-core/src/platform/container/token_usage.rs` | canonical `TokenUsage` | ✓ VERIFIED | Sole `pub struct TokenUsage`; re-exports confirmed at `battalion/mod.rs` and `paladin-llm/src/llm_analysis_service.rs` |
| `.project/current-exports.txt` | regenerated baseline matching HEAD | ✓ VERIFIED | Guard reports unchanged against current tree (1967 items, re-generated after the DEBT-04 table_herald removal was caught mid-close-out) |
| `scripts/check-deprecations.sh` | gate that can fail | ✓ VERIFIED (not independently re-probed with a malformed attribute; SUMMARY's both-direction proof accepted based on script content review — no `exit 0` early-outs found, scans `src` and `crates`) | `grep -vE '^\s*#' scripts/check-deprecations.sh \| grep -c 'exit 0'` → 0 |
| `crates/paladin-ports/Cargo.toml` | no `doctest = false` | ✓ VERIFIED | `grep -c doctest` → 0 |
| `.planning/decisions/0022-*.md`, `0023-*.md` | corpus-shaped ADRs | ✓ VERIFIED | 7 headings each, `Accepted`, indexed in `PROMOTION.md` (0022/0023, next-free 0024) |
| `.planning/phases/08-verified-defect-closure/COVERAGE.md` | reasoned no-external-API declaration | ✓ VERIFIED | Exists, no capability matrix, reasoned prose confirmed present |
| `.planning/phases/08-verified-defect-closure/deferred-items.md` | accurate residual record | ✓ VERIFIED | Factual content (four residual DEBT-01 sites) independently confirmed accurate (see Requirements Coverage); attribution sentence corrected by `887bd12`, re-read and confirmed internally consistent with its own header and with `REQUIREMENTS.md`/`08-09-SUMMARY.md` |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `battalion::TokenUsage` / `llm_analysis_service::TokenUsage` | `paladin-core::token_usage::TokenUsage` | `pub use` re-export | ✓ WIRED | Full workspace test suite green (`cargo test --offline --workspace`, re-run this session: every `test result:` line reads `0 failed`); no call-site edits needed per 08-01-SUMMARY.md, consistent with a working re-export |
| `paladin-herald`'s `table`/`color` features | root `cli` feature | `Cargo.toml` feature propagation | ✓ WIRED | `cargo build --offline --lib --no-default-features` compiles; `cargo build --offline --bin paladin --features cli` compiles (re-run this session via the earlier `--lib` check plus the SUMMARY's own recorded `--bin paladin --features cli` build, content-reviewed) |
| ADR-0023 criterion-4 evidence slot | `cargo tree` output | verbatim capture | ✓ WIRED | `PENDING` slot replaced; `grep -c PENDING` → 0, confirmed |
| DEBT-01 requirement-text corrections | `REQUIREMENTS.md` traceability rows | dated closure notes | ✓ WIRED | Five `REQ-*` rows corrected in place, cross-referenced, confirmed by direct read |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| DEBT-01 | 08-02, 08-05, 08-09 | api-surface CI guard + stale-path propagation | ✓ SATISFIED (checkbox honestly ticked on its own literal done-condition; residual openly recorded) | Re-ran the residual-site grep myself: `grep -rn 'project/current-exports.txt' .project/ \| grep -v '~~' \| grep -v '\.project/current-exports' \| grep -v 'tasks-'` → exactly the four sites `deferred-items.md` and `REQUIREMENTS.md:886-894` name, no more, no fewer. The "nine references" corpus figure is honestly flagged as an undercount (true count ≥ 13) in both the ledger and REQUIREMENTS.md, not silently absorbed. This is a rare case where the checkbox tick is *more* honest than the requirement text it satisfies — recorded correctly. |
| DEBT-02 | 08-04, 08-06 | Deprecation policy withdrawal + reconciliation | ✓ SATISFIED | See Truth 2 |
| DEBT-03 | 08-03 | `paladin-ports` doctests | ✓ SATISFIED | See Truth 3 |
| DEBT-04 | 08-04, 08-07, 08-08 | CLI dependency isolation | ✓ SATISFIED | See Truth 4, 6 |
| DEBT-05 | 08-01 | `TokenUsage` consolidation | ✓ SATISFIED | See Truth 5 |

No orphaned requirements — REQUIREMENTS.md maps exactly DEBT-01…DEBT-05 to Phase 8, all five have plan coverage and all five checkboxes are ticked behind cited evidence.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md` | 121, 142 | `TBD` markers (`Migration Path: TBD based on Epic 3 refactoring`; `- TBD based on usage analysis`) | ℹ️ Info | Pre-existing historical content explicitly governed by D-00c ("annotation only, never rewriting"); outside DEBT-02's declared scope (the four "Open Questions" it closed are a different section); the IMMEDIATE DEPRECATION category these TBDs sit under was independently confirmed to name no live deprecation candidate, so nothing depends on resolving them |
| `docs/src/api-reference/stable-api.md` | 851 | `TBD` marker (`0.1.x → 0.2.x: TBD (no breaking changes yet)`) | ℹ️ Info | Confirmed via `git show 2e30e89` that this line was not touched by this phase's commit; pre-existing, unrelated version-history table content, not a deprecation claim |

**Resolved this session:** `.planning/phases/08-verified-defect-closure/deferred-items.md:58-60`'s self-contradictory attribution (see Truth 7) was a 🛑 Blocker on the initial pass; closed by commit `887bd12` and re-verified — no longer present.

**Meta-finding — a correction commit that overstated its own completeness.** Commit `847210a`'s message states: *"Three files said 'at the human's request'... corrected in 08-09-SUMMARY.md, deferred-items.md and REQUIREMENTS.md."* That claim was itself only partially true: `deferred-items.md`'s section header was corrected, but its body sentence two lines below was not, and the commit's message reported the file as fully corrected regardless. This is the same defect class the phase exists to close — a document making a claim about its own completeness that the diff underneath it does not fully support — except this time the document was a commit message, not a `.project/` requirement or a ledger row, and the author was the orchestrator mid-close-out rather than a stale historical source. Recorded here per the coordinator's request, so a future reader can see that the correction needed a correction, and that the gap was caught by an external re-read (this verifier) rather than by the correcting commit's own sweep. Commit `887bd12` closes both the original attribution error and this meta-observation's underlying cause; no further action needed, but the pattern (a fix-commit asserting a scope it didn't fully cover) is worth the project's own attention going forward, independent of this phase.

No `FIXME`/`XXX` markers found in any phase-modified file. No stub patterns, hardcoded-empty renders, or console-log-only implementations found (this phase is documentation/CI/manifest-shaped, not UI-shaped, so most stub heuristics don't apply — spot-checked the Rust edits directly instead, see Behavioral Spot-Checks).

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| api-surface guard passes on unchanged tree | `bash scripts/check-api-surface.sh .project/current-exports.txt` | `✅ API surface unchanged`, exit 0 | ✓ PASS |
| Zero deprecated attributes | `grep -rn '#\[deprecated' src crates \| wc -l` | `0` | ✓ PASS |
| `paladin-ports` doctests execute | `cargo test --offline -p paladin-ports --doc` | `96 passed; 0 failed; 94 ignored` | ✓ PASS |
| Library-only build compiles | `cargo build --offline --lib --no-default-features` | exit 0 | ✓ PASS |
| No CLI deps in downstream-consumer graph | `cargo tree --offline --no-default-features -e normal \| grep -E 'structopt\|colored\|comfy-table'` | no output | ✓ PASS |
| `mockito`/`colored` false-positive traced | `cargo tree --offline --no-default-features -i colored` | `colored v3.1.1 └── mockito v1.7.2 [dev-dependencies] └── paladin-ai` | ✓ PASS (confirms dev-only edge) |
| Exactly one `TokenUsage` struct | `grep -rn 'pub struct TokenUsage' crates src \| wc -l` | `1` | ✓ PASS |
| Full workspace test suite | `cargo test --offline --workspace` | every `test result:` line `0 failed` (35 lines) | ✓ PASS |
| Formatting | `cargo fmt --check` | exit 0, no output | ✓ PASS |
| Lint | `cargo clippy --workspace -- -D warnings` | exit 0, zero warnings | ✓ PASS |
| Prohibited files untouched | `grep -n 'actions-rs/toolchain@v1' .github/workflows/ci.yml` | `148, 393, 792` unchanged | ✓ PASS |
| `VisionTokenUsage` untouched | `grep -n 'VisionTokenUsage' crates/paladin-ports/src/output/vision_port.rs` | present, unconverged | ✓ PASS |
| `src/main.rs` not retired | `ls src/main.rs` | present | ✓ PASS |
| Residual DEBT-01 site count | `grep -rn 'project/current-exports.txt' .project/ \| grep -v '~~' \| grep -v '\.project/current-exports' \| grep -v 'tasks-'` | exactly the 4 sites claimed | ✓ PASS |
| Coverage tool-of-record honored | `grep -n 'cargo-llvm-cov\|tarpaulin' .planning/decisions/0006-coverage-gate.md` | pipeline uses raw `llvm-profdata`/`llvm-cov` via rustup toolchain paths, explicitly rules out both alternates | ✓ PASS |
| Attribution self-consistency (re-check) | `grep -rn -i "the human asked\|at the human's request\|human requested" .planning/phases/08-verified-defect-closure/ .planning/REQUIREMENTS.md` | only hits are inside this VERIFICATION.md's own historical quote of the finding | ✓ PASS |

### Coverage Figure Scrutiny

The 85.85% figure (floor 84.00%) in `.planning/decisions/0006-coverage-gate.md`'s Phase 8 amendment was reviewed, not re-executed in full (the instrumented full-workspace build is multi-hour class work and the phase's own SUMMARY records it took the bulk of plan 08-09's ~2h10min). The recorded evidence is a genuine `llvm-cov report` `TOTAL` line (`97193 11610 88.05% 7799 1677 78.50% 63999 9059 85.85% 0 0 -`), produced via the ADR-0006-mandated pipeline (`RUSTFLAGS="-C instrument-coverage"` + `llvm-profdata merge` + `llvm-cov report`, using absolute rustup toolchain paths), not `cargo tarpaulin` and not the `cargo-llvm-cov` subcommand — both explicitly ruled out by ADR-0006's own tool-of-record note, confirmed by direct read. The 30-tests-removed-from-default-feature-run figure (larger than the 3 the plans anticipated) is accounted for explicitly and correctly: `table_herald.rs`'s *source*, not just its tests, is gated behind the `table` feature, so both the covered lines and the tests exercising them are symmetrically absent from the report's numerator and denominator — this is a scope exclusion, not a coverage regression, and all 30 tests are confirmed (in 08-07-SUMMARY.md, independently spot-checked for internal consistency) to still run and pass under `--features cli`/`--features table,color`.

### Human Verification Required

None. All items in this report were resolved by direct command execution or file inspection; no visual, real-time, or external-service behavior is in scope for this phase.

### Gaps Summary

None remaining. One gap was found on the initial pass (Truth 7 — `deferred-items.md`'s uncorrected attribution sentence) and has been closed by commit `887bd12`, re-verified directly in this session (file re-read, grep sweep confirming no surviving misattribution outside this report's own historical quote). A meta-finding about commit `847210a`'s own overstated completeness claim is recorded above under Anti-Patterns for the project's future attention; it required no further action once `887bd12` landed.

Everything this phase claims — the five ROADMAP success criteria, the recorded-decision-behind-every-shipped-surface-change clause, the coverage floor, the ADR bookkeeping, the ledger amendments, and DEBT-01's honestly-flagged residual — was independently re-verified against the current tree and holds. Phase 8 goal achieved.

---

*Verified: 2026-08-07T22:28:15Z, re-verified 2026-08-07T22:33:00Z*
*Verifier: Claude (gsd-verifier)*
