---
phase: 08-verified-defect-closure
plan: 09
subsystem: docs
tags: [close-out, ledger-amendment, adr-bookkeeping, coverage-gate, requirements-traceability]

requires:
  - phase: 08-verified-defect-closure
    provides: "08-01 through 08-08 — every closure claim in this plan transcribes a command from a named prior SUMMARY, none re-derived"
provides:
  - "Five Milestone 4-6 ledger rows amended in place (plus two Herald rows per ADR-0023/D-14), verdict distribution recounted"
  - "DEBT-01..05 checkboxes and traceability statuses flipped behind cited evidence"
  - "PROMOTION.md advanced to next-free 0024 with 0022/0023 indexed"
  - "PROJECT.md Key Decisions extended with ADR-0022/ADR-0023 rows"
  - "COVERAGE.md reasoned no-external-API declaration"
  - "ADR-0006's 84% floor re-measured at 85.85% against the phase-8 tree"
affects: ["Phase 9-16 (every planner that reads REQUIREMENTS.md / the ledger / PROMOTION.md instead of re-verifying the tree)"]

tech-stack:
  added: []
  patterns: ["D-00d in-place ledger row amendment with (was: ...) markup", "dated closure notes citing verbatim commands (D-00e/D-21)"]

key-files:
  created:
    - .planning/phases/08-verified-defect-closure/COVERAGE.md
  modified:
    - .planning/ledgers/milestone-04-06.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md
    - .planning/decisions/0006-coverage-gate.md
    - .project/current-exports.txt

key-decisions:
  - "DEBT-01's checkbox is ticked on its own literal done-condition (five tooling + five requirement-text references), which is fully met; the four residual requirement-level sites 08-05 found outside the original nine-reference count are recorded as an open, unowned finding in both the ledger row and REQUIREMENTS.md's closure note, not silently absorbed into the tick."
  - "The Herald half of REQ-cli-dependency-isolation / REQ-library-only-build (Milestone 4 Epic 3), which Phase 7 recorded as superseded by shipped code, is amended to satisfied per ADR-0023/D-14 — ROADMAP criterion 4 is stricter than that verdict, and DEBT-04's fix now satisfies the criterion, so the row is amended rather than the criterion narrowed."
  - "REQ-workspace-ci-upgrade's verdict word is NOT flipped to satisfied — two of three clauses now ship (workspace scoping, the paladin-ports doctest exclusion); clause 2 (actions-rs/toolchain@v1) stays deferred with reason, Phase 15/PIPE-04 named as owner, per the plan's explicit prohibition against a fully-satisfied verdict here."
  - "Rule-1 deviation found and fixed during this plan's own Task 3 spot-check: DEBT-04's table_herald feature-gating (plan 08-07) removed a re-export from the default-feature public API surface after 08-02's baseline was captured; nothing in the phase regenerated it. Fixed by regenerating .project/current-exports.txt (1968 -> 1967 items); the guard now reports unchanged, exit 0."

requirements-completed: [DEBT-01, DEBT-02, DEBT-03, DEBT-04, DEBT-05]

duration: ~2h10min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 09: Phase Close-Out Summary

**Amended five (plus two Herald) Milestone 4-6 ledger rows in place, flipped all five DEBT checkboxes and traceability statuses behind cited evidence, advanced ADR bookkeeping to 0024, wrote a reasoned COVERAGE.md, and re-measured ADR-0006's 84% floor at 85.85% against the phase-8 tree — closing Phase 8 on paper to match what the tree now proves.**

## Performance

- **Duration:** ~2h10min (dominated by the instrumented coverage measurement's cold, full-workspace compile)
- **Tasks:** 3 of 4 complete; Task 4 (the blocking human checkpoint) is where this plan stops
- **Files modified:** 6 (`.planning/ledgers/milestone-04-06.md`, `.planning/REQUIREMENTS.md`, `.planning/decisions/PROMOTION.md`, `.planning/PROJECT.md`, `.planning/decisions/0006-coverage-gate.md`, `.project/current-exports.txt`), 1 created (`COVERAGE.md`)

## Task Commits

Each task was committed atomically:

1. **Task 1: Amend the five Milestone 4-6 ledger rows in place** — `dba894b` (docs)
2. **Task 2: Flip the DEBT checkboxes and traceability statuses behind evidence** — `e71df30` (docs)
3. **Task 3a-c: ADR bookkeeping, PROJECT.md, COVERAGE.md** — `f4bf080` (docs)
4. **Task 3d: ADR-0006 coverage-floor re-check** — `d1b8350` (docs)
5. **Deviation (Rule 1): regenerate the stale api-surface baseline** — `ce38132` (fix), found and fixed during Task 3's own spot-checking

**Checkpoint:** Task 4, `type="checkpoint:human-verify" gate="blocking"`, is reached next. Per this plan's explicit instruction, it is **not** self-approved regardless of auto-mode — the orchestrator surfaces it to a human. This SUMMARY documents Tasks 1-3 (all substantive work); no further record changes are made until the checkpoint resolves.

## Task 1 — Ledger amendments

`.planning/ledgers/milestone-04-06.md` gained a `## Phase 8 amendments (2026-08-06)` section header and 7 amended rows (the 5 D-23 names plus 2 more per ADR-0023/D-14):

| Row | Was | Now | Evidence cited |
|---|---|---|---|
| `REQ-api-surface-ci` (:130 post-edit) | `genuinely outstanding` | `satisfied` | `08-02-SUMMARY.md`: 5 tooling literals corrected, baseline regenerated (1968 items), guard proven both directions |
| `REQ-deprecation-warnings` | `genuinely outstanding` | `satisfied` | `08-06-SUMMARY.md` + ADR-0022: closes by **withdrawal**, not implementation |
| `REQ-cli-dependency-isolation` (Herald half, D-14) | `superseded by shipped code` | `satisfied` | `08-08-SUMMARY.md`: criterion-4 proof, both `cargo tree` invocations recorded |
| `REQ-library-only-build` (Herald half, D-14) | `superseded by shipped code` | `satisfied` | Same evidence as the row above |
| `REQ-ports-doctest-compilation` | `genuinely outstanding` | `satisfied` | `08-03-SUMMARY.md`: 96/96 doctests, one-commit removal of both halves of the guard |
| `REQ-ports-tests-and-rustdoc` | `present, unproven` | `satisfied` | `08-03-SUMMARY.md`: the doctest disablement that undermined "no documentation may be lost" is removed; HARD-07 seam preserved |
| `REQ-workspace-ci-upgrade` | `deferred with reason` | `deferred with reason` (unchanged word; evidence amended) | Clause 3 now closes; clause 2 (`actions-rs`) stays deferred, Phase 15/PIPE-04 named |

**Recount command** (re-run against the finished file, per the Phase 7/07-13 lesson — recount, not adjust arithmetically):

```
grep -oP '^\| REQ-[a-z0-9-]+ \| \*{0,2}\K[a-z, ]+(?=\*{0,2}( \(was:[^)]*\))? \|)' .planning/ledgers/milestone-04-06.md | sort | uniq -c
```

| Verdict | Before | After |
|---|---|---|
| `satisfied` | 71 | 77 |
| `present, unproven` | 15 | 14 |
| `genuinely outstanding` | 3 | 0 |
| `deferred with reason` | 1 | 1 |
| `superseded by shipped code` | 12 | 10 |
| `relocated` | 5 | 5 |
| `diverged` | 8 | 8 |
| **Total** | **115** | **115** |

Row total unchanged (115 — `grep -c '^| REQ-'`), zero duplicate `REQ-*` IDs. The ledger's own `## Summary` §"Verdict distribution" and §"Per-milestone roll-up" tables were updated in place with the same recount, each carrying its own dated amendment note.

`git diff --numstat` on the ledger shows 22 insertions / 7 deletions (not 0 deleted, because each row is a single very long line — editing content within a row necessarily replaces that whole line in a line-based diff; the 7 "deletions" are the pre-images of the 7 amended single-line rows, not row removals. No row was inserted, deleted, or reordered; all 115 `REQ-*` IDs are unchanged and every amendment retains its superseded verdict text via `(was: ...)`).

## Task 2 — REQUIREMENTS.md checkbox and traceability flips

All five DEBT checkboxes ticked (`grep -cE '^- \[x\] \*\*DEBT-0[1-5]\*\*'` → `5`; `^- \[ \]` variant → `0`), each with a dated `2026-08-06` closure note:

- **DEBT-01**: cites both `08-02` (tooling) and `08-05` (requirement-text) halves; records the FR-7.3 `.public-api-baseline.txt` item as superseded by naming; names the four residual requirement-level sites `08-05` found beyond the original nine-reference count, with no owning phase currently assigned.
- **DEBT-02**: cites ADR-0022 and `08-06`; explicitly records closure **by withdrawal**.
- **DEBT-03**: cites `08-03`; explicitly preserves the HARD-07 seam (D-12 declined to decide the `cargo doc` warning bar; Phase 10 inherits the measured 6-warning state).
- **DEBT-04**: cites ADR-0023, `08-07`, `08-08`; records the 30-test `cli`-gating coverage delta and both `cargo tree` invocations (literal + `-e normal`).
- **DEBT-05**: cites `08-01`; explicitly records `VisionTokenUsage` as deliberately out of scope per D-20.

Traceability rows (`:3659-3663` region) all flip `Phase 8 | Pending` → `Phase 8 | Complete`; no `Pending` remains in that region. Cross-phase coupling table (`:3745-3762` region): `ARCH-03(c)→DEBT-05` marked discharged; `HARD-07→DEBT-03` marked as a live, unresolved coupling (D-12 declined to block); `DEBT-03→DOCS-03` marked as having its input now ready. Zero previously-ticked checkboxes disturbed (`git diff | grep -cE '^\-.*\[x\]'` → `0`); non-DEBT `- [ ]` checkbox count delta is exactly `-5` (45 → 40).

## Task 3 — ADR bookkeeping, PROJECT.md, COVERAGE.md, coverage re-check

**`PROMOTION.md`:** index rows added for `0022` (`deprecation-requirement-withdrawal`) and `0023` (`cli-dependency-isolation`); next-free line advanced `**Next free ADR number: 0022**` → `**Next free ADR number: 0024**`; the pre-existing dated D-25a note (explaining the 0021→0022 jump) preserved verbatim (`grep -c 'D-25a'` unchanged); a new dated note added recording that Phase 8 is both author and executor of both ADRs (D-22).

**`PROJECT.md`:** exactly two Key Decisions rows added, each linking to its ADR file; the existing 23-row table content and the "zero ingested locked decisions" evidence note are byte-identical before/after (confirmed via `git diff`, only 2 insertion lines, 0 deletions, in that section).

**`COVERAGE.md`** (`.planning/phases/08-verified-defect-closure/COVERAGE.md`, new file): opens with `No external API integration: this phase closes five verified code-and-records defects... It integrates, wraps, or consumes no external API, SDK, transport, or service.` — a reasoned declaration, no capability matrix, `grep -c '^|'` → `0`. Explains why a keyword detector might fire (the `api-surface` CI job, `REQ-api-surface-ci`/`REQ-web-api-baseline-changelog`/`REQ-openapi-drift-guard`, the `paladin-web` HTTP surface named in ledger rows this phase edits) and states each refers to this project's own already-shipped surface or a CI job name. Evidence: `git diff Cargo.lock` across the phase shows removals only (14 removed, 0 added, per `08-07-SUMMARY.md`); the only network access is `rustup toolchain install nightly` for the pre-installed `cargo public-api` tooling; no credential or user input handled.

**ADR-0006 coverage-floor re-check** — `.planning/decisions/0006-coverage-gate.md` gained a `## Phase 8 amendment (2026-08-06)` section. Full pipeline, verbatim, using the absolute rustup LLVM tool paths (`cargo-llvm-cov` remains uninstalled; crates.io still returns HTTP 403; **not attempted**, matching ADR-0006's own tool-of-record note ruling out `cargo tarpaulin`):

```
$ RUSTFLAGS="-C instrument-coverage" \
  LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" \
  cargo test --workspace --offline
EXIT:0 — 3013 passed, 0 failed (all 35 `test result: ok.` lines, matching 08-08's carried-forward count)

$ /usr/local/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata \
  merge -sparse target/coverage/*.profraw -o target/coverage/paladin.profdata
EXIT:0 — merged from 2,321 .profraw files

$ RUSTFLAGS="-C instrument-coverage" cargo test --workspace --no-run --message-format=json --offline \
  | jq -r 'select(.profile.test == true) | .filenames[]' | grep -v '\.dSYM' | sort -u
30 unique test-binary object paths (one fewer than ADR-0006's original 31 — table_herald's
default-feature test binary no longer builds; accounted for below)

$ /usr/local/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov \
  report --instr-profile=target/coverage/paladin.profdata \
  --ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/' \
  --object=<each of the 30 discovered objects>

TOTAL   97193   11610   88.05%   7799   1677   78.50%   63999   9059   85.85%   0   0   -
```

**Measured: 85.85% workspace line coverage** (63,999 lines counted, 9,059 missed) — **clears the 84.00% floor by 1.85 points.** Delta from the last recorded measurement (85.92% at HEAD `1ad8be5`, Phase 3 plan 07): **−0.07 points**, effectively flat.

**The `cli`-gating delta (30 tests, not the plan's anticipated 3) accounted for explicitly:** `table_herald.rs`'s own *source* is gated the same way as its tests (behind the `table` feature), so this llvm-cov report's per-file rows contain **no `table_herald.rs` row at all** — confirmed by grep against the report output, which finds only `markdown_herald.rs` (799 regions, 97.12% region cover, 97.81% line cover — the crate's coloured-path-split sibling, which stays compiled by default). Both the 30 removed tests and the source lines they exercised are absent from **both** the numerator and the denominator symmetrically. This is a feature-gating exclusion, not a coverage regression: no line that used to count as covered now counts as missed, and all 30 tests still run and pass under `--features cli` / `--features table,color` (unchanged from `08-07-SUMMARY.md`). **The measured figure did not move meaningfully because of this delta** — the −0.07-point shift from 85.92% is within ordinary measurement noise across a phase's worth of unrelated line changes, not attributable to the cli-gating exclusion specifically.

No coverage claim from this phase is below the floor; the checkpoint below does not need a sub-84.00%-figure decision path.

## Deviation — Rule 1 (auto-fixed bug)

**Found during:** this plan's own Task 3, while spot-checking the five ROADMAP criterion commands ahead of the checkpoint.

**Issue:** `bash scripts/check-api-surface.sh .project/current-exports.txt` reported `❌ API surface has changed!` — plan 08-07's `paladin-herald` feature split (DEBT-04) removed `pub use paladin::infrastructure::adapters::herald::table_herald` from the default-feature public API surface, but `08-02`'s baseline (captured in an earlier wave, before 08-07 landed) was never regenerated to reflect it. This is the guard working exactly as designed — catching a real, already-`CHANGELOG.md`-documented breaking change — not a defect in the guard itself.

**Fix:** `bash scripts/extract-public-api.sh .project/current-exports.txt` re-run; baseline now 1967 items (one fewer, the removed re-export); `bash scripts/check-api-surface.sh .project/current-exports.txt` → `✅ API surface unchanged`, exit 0, re-confirmed.

**Files modified:** `.project/current-exports.txt`; both the ledger's `REQ-api-surface-ci` row and REQUIREMENTS.md's DEBT-01 closure note carry a dated follow-up note recording this finding and fix, rather than silently re-editing the earlier "1968 items" citation.

**Committed in:** `ce38132`.

## Checkpoint spot-checks (Task 4's how-to-verify, run fresh this session)

1. `grep -rn 'pub struct TokenUsage' crates src | wc -l` → `1`
2. `bash scripts/check-api-surface.sh .project/current-exports.txt` → `✅ API surface unchanged`, exit 0 (after the Rule-1 fix above)
3. `grep -rn '#\[deprecated' src crates | wc -l` → `0`
4. `cargo test --offline -p paladin-ports --doc` → `96 passed; 0 failed; 94 ignored`
5. `cargo build --offline --lib --no-default-features` → exit 0; `cargo tree --offline --no-default-features | grep -E 'structopt|colored|comfy-table'` → one hit, `colored v3.1.1` — **this is the documented `mockito` dev-dependency finding from ADR-0023/08-07/08-08, not a criterion-4 violation**; `cargo tree --offline --no-default-features -e normal | grep -E 'structopt|colored|comfy-table'` (the true downstream-library-consumer view) → no output.

**Prohibitions held:** `grep -n 'actions-rs/toolchain@v1' .github/workflows/ci.yml` → `148`, `393`, `792` (unchanged); `git diff --stat crates/paladin-ports/src/output/vision_port.rs` → empty; `src/main.rs` exists. Zero `.rs`, `Cargo.toml`, `.github/workflows/`, `Dockerfile*`, or `CHANGELOG.md` files touched anywhere in this plan's commit range (`git diff --stat f9e3280..HEAD -- '*.rs' 'Cargo.toml' '.github/workflows/' 'Dockerfile*' 'CHANGELOG.md'` → empty).

**Workspace gate (CLAUDE.md):** `cargo fmt --check` → exit 0 (no Rust files were touched by this plan). `cargo test`/`cargo clippy` were not re-run in full a second time beyond the coverage pipeline's own `cargo test --workspace --offline` (3013 passed, 0 failed) and the crate-scoped/no-default-features spot-checks above, since this plan changes no source.

## Issues Encountered

None beyond the Rule-1 deviation documented above. The instrumented coverage test run (`cargo test --workspace --offline` under `RUSTFLAGS="-C instrument-coverage"`) triggered a full cold recompile of the workspace under the new RUSTFLAGS fingerprint, taking the bulk of this plan's wall-clock time — expected, not an issue.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

Phase 8 is closed on paper pending the human checkpoint below. Once approved:
- STATE.md, ROADMAP.md phase-status line, and the final metadata commit are the orchestrator's, not this plan's (per worktree instructions).
- Phase 9 (SEC-01…SEC-05) and all later planners can read `REQUIREMENTS.md`, the Milestone 4-6 ledger, and `PROMOTION.md` as current — DEBT-01…DEBT-05 are closed, ADR-0022/ADR-0023 are indexed, and the 84% coverage floor is confirmed not regressed.
- **Residual, unowned:** DEBT-01's four residual `.project/` requirement-text sites (recorded in the ledger, REQUIREMENTS.md, and `deferred-items.md`) have no assigned owning phase — a future phase touching Milestone 8/11/12 `.project/` records should close them using the D-00c pattern already applied to the other five.
- **HARD-07 (Phase 10)** inherits the `cargo doc --workspace --no-deps` warning-bar decision, unresolved by DEBT-03.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-04-06.md`
- FOUND: `.planning/REQUIREMENTS.md`
- FOUND: `.planning/decisions/PROMOTION.md`
- FOUND: `.planning/PROJECT.md`
- FOUND: `.planning/decisions/0006-coverage-gate.md`
- FOUND: `.planning/phases/08-verified-defect-closure/COVERAGE.md`
- FOUND: `.project/current-exports.txt`
- FOUND commit `dba894b` (Task 1)
- FOUND commit `e71df30` (Task 2)
- FOUND commit `f4bf080` (Task 3a-c)
- FOUND commit `d1b8350` (Task 3d)
- FOUND commit `ce38132` (Rule-1 deviation fix)

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*
