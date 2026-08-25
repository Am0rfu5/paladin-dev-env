---
phase: 08-verified-defect-closure
plan: 02
subsystem: infra
tags: [ci, cargo-public-api, github-actions, shell, api-surface, deprecations]

# Dependency graph
requires: []
provides:
  - "api-surface CI job reaches its baseline end to end (was permanently red since commit 928c6d5)"
  - "regenerated .project/current-exports.txt baseline matching HEAD (1968 items)"
  - "check-deprecations.sh as a gate that can genuinely fail, scanning src/ and crates/"
affects: [08-05, "Phase 15 / PIPE-04 (actions-rs/toolchain@v1 sweep, untouched here)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Both-direction guard proof: throwaway pub fn probe added, exit code + message captured, then git checkout -- reverted before commit"
    - "pipefail-safe branch restructuring: capture command output instead of discarding to /dev/null, fall through to the real check instead of early exit 0"

key-files:
  created: []
  modified:
    - scripts/check-api-surface.sh
    - scripts/extract-public-api.sh
    - .github/workflows/ci.yml
    - .project/current-exports.txt
    - scripts/check-deprecations.sh

key-decisions:
  - "Baseline regeneration treated as the proven default path (per D-02/D-03), not a best-effort attempt -- ran to completion in this environment exactly as CONTEXT.md's amended framing predicted"
  - "Baseline regeneration committed as its own commit, separate from the five-line path fix, per D-24 discretion (the diff reads better alone)"
  - "check-deprecations.sh restructured to remove both unconditional early exit 0 branches while keeping the primary/fallback jq+grep structure intact, per D-05's narrow scope"

requirements-completed: [DEBT-01]

coverage:
  - id: D1
    description: "Five stale project/current-exports.txt path literals corrected to .project/current-exports.txt across two scripts and three ci.yml lines"
    requirement: "DEBT-01"
    verification:
      - kind: other
        ref: "grep -rn 'project/current-exports.txt' scripts/ .github/ | grep -v '\\.project/' (no output)"
        status: pass
    human_judgment: false
  - id: D2
    description: "api-surface baseline regenerated from HEAD (1956 -> 1968 items) and proven to guard in both directions"
    requirement: "DEBT-01"
    verification:
      - kind: other
        ref: "bash scripts/check-api-surface.sh .project/current-exports.txt (positive: exit 0, unchanged; negative with probe: exit 1, changed)"
        status: pass
    human_judgment: false
  - id: D3
    description: "check-deprecations.sh scans src/ and crates/ and can fail on a malformed #[deprecated] attribute, with no new gate on zero-deprecation trees"
    requirement: "DEBT-01"
    verification:
      - kind: other
        ref: "bash scripts/check-deprecations.sh (clean tree: exit 0; probe in crates/paladin-core/src/lib.rs: exit 1)"
        status: pass
    human_judgment: false

duration: 25min
completed: 2026-08-07
status: complete
---

# Phase 08 Plan 02: api-surface CI Job Restoration Summary

**Corrected all five stale `project/current-exports.txt` tooling references, regenerated the public-API baseline from HEAD (1956 -> 1968 items), and made `check-deprecations.sh` a gate that can genuinely fail on a malformed attribute across `src/` and `crates/` — all four claims proven with verbatim command output, not inferred.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-07T00:32:48Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- The `api-surface` CI job's guard reaches its baseline end to end for the first time since commit `928c6d5` — it no longer exits 1 with `⚠️  No baseline found`.
- The baseline is regenerated and current: `.project/current-exports.txt` now reflects HEAD (1968 public items, dated 2026-08-07), replacing a 2026-06-10 snapshot (1956 items).
- The guard is proven in **both** directions in this environment, not asserted: it passes (exit 0, `✅ API surface unchanged`) on the unchanged, regenerated tree, and fails (exit 1, `❌ API surface has changed!`) when a throwaway `pub fn` is added, correctly identifying it by name.
- `check-deprecations.sh` is a real gate now: it scans `src/` **and** `crates/` (previously `src/` only, missing all eleven workspace crates), removed both unconditional early `exit 0` branches, and is proven to fail (exit 1) on a deliberately malformed `#[deprecated]` attribute placed under `crates/paladin-core/src/lib.rs` — which the pre-fix script provably could not detect.
- `ci.yml:148`'s `actions-rs/toolchain@v1` (Phase 15 / PIPE-04 scope) is untouched, confirmed via `grep -n 'actions-rs/toolchain@v1' .github/workflows/ci.yml` returning `148`, `393`, `792` unchanged.
- Full workspace gate (CLAUDE.md) run at the end of the plan: `cargo fmt --check` (exit 0), `cargo clippy --workspace -- -D warnings` (exit 0, zero warnings), `cargo test --workspace --offline` (exit 0, zero failures across every crate, including `paladin-ports`' 96 doctests).

## Task Commits

Each task was committed atomically:

1. **Task 1: Correct all five tooling path literals** — `7e02da3` (fix)
2. **Task 2: Regenerate the baseline and prove the guard in both directions** — `1f7ea39` (fix; the both-direction proof itself was verification-only and left no residue — probe added and reverted with `git checkout --` before this commit was made)
3. **Task 3: Make check-deprecations.sh a gate that can actually fail** — `0be3486` (fix)

**Plan metadata:** committed separately per worktree protocol (this SUMMARY + STATE.md/ROADMAP.md are owned by the orchestrator after wave merge; per-task commits above are the plan's substantive record).

## Files Created/Modified

- `scripts/check-api-surface.sh` — `BASELINE` default corrected to `.project/current-exports.txt` (line 6)
- `scripts/extract-public-api.sh` — `OUTPUT_FILE` default corrected to `.project/current-exports.txt` (line 6)
- `.github/workflows/ci.yml` — three literals in the `api-surface` job corrected (`Check API surface changes` step, and the two lines of the `Generate API diff on failure` step); `:148` left untouched
- `.project/current-exports.txt` — regenerated from HEAD via `bash scripts/extract-public-api.sh .project/current-exports.txt`
- `scripts/check-deprecations.sh` — malformed-attribute grep extended to `src/ crates/`; both unconditional early `exit 0` branches removed so execution always reaches that check

## Decisions Made

- **Baseline regeneration treated as first-class, not a fallback.** Per CONTEXT.md D-02/D-03 as amended by RESEARCH.md, `cargo-public-api` 0.52.0 was already installed (no `cargo install`, no crates.io) and the nightly toolchain was already present in this environment (`rustup toolchain list` showed `nightly-x86_64-unknown-linux-gnu`), so no contingency/blocker path was needed. Confirmed verbatim: `cargo public-api --version` → `cargo-public-api 0.52.0`.
- **Baseline committed as its own commit** (`1f7ea39`), separate from the five-line path fix (`7e02da3`), per D-24's discretion — the 40-insertion/8-deletion diff reads more clearly alone.
- **check-deprecations.sh restructuring kept minimal per D-05.** Only two changes: (a) extend the malformed-attribute grep's file scope from `src/` alone to `src/ crates/`; (b) make the primary jq branch capture its parsed output instead of discarding to `/dev/null`, and remove the fallback's two unconditional `exit 0` arms so both branches report findings and fall through to the malformed-attribute check rather than short-circuiting past it. No new "deprecations must exist" gate was added — a zero-deprecation tree still exits 0, confirmed against today's tree (`grep -rn '#\[deprecated' src crates` → 0 matches, script exits 0).

## Deviations from Plan

None — plan executed exactly as written. All five path literals, the baseline regeneration, and the `check-deprecations.sh` restructuring matched the plan's `<action>` blocks precisely; no Rule 1-4 auto-fixes were needed.

## Evidence (D-00e / D-21 bar — verbatim commands and output)

**Task 1 — tracer verify, before/after:**
- Before this task, `check-api-surface.sh` exited 1 with `⚠️  No baseline found at project/current-exports.txt` (recorded in RESEARCH.md, re-confirmed structurally by the stale literal grep before editing).
- After: `bash scripts/check-api-surface.sh` (default path) → `❌ API surface has changed!` against the still-stale (pre-regeneration) baseline — the correct intermediate state, not `No baseline found`. Tracer grep assertions both passed:
  ```
  grep -q 'No baseline found' /tmp/gsd-08-02-t1.log   → not found
  grep -qE 'API surface (unchanged|has changed)' /tmp/gsd-08-02-t1.log → TRACER-OK
  ```
- `grep -rn 'project/current-exports.txt' scripts/ .github/ | grep -v '\.project/'` → no output.
- `grep -c '\.project/current-exports\.txt' scripts/check-api-surface.sh scripts/extract-public-api.sh` → `1` for each.
- `grep -c '\.project/current-exports\.txt' .github/workflows/ci.yml` → `3`.
- `grep -n 'actions-rs/toolchain@v1' .github/workflows/ci.yml` → `148`, `393`, `792` (unchanged).
- `git diff --stat .github/workflows/ci.yml` → `3 insertions(+), 3 deletions(-)`.

**Task 2 — toolchain confirmation, determinism, regeneration, both directions:**
- `cargo public-api --version` → `cargo-public-api 0.52.0`.
- `rustup toolchain list` → includes `nightly-x86_64-unknown-linux-gnu` (already present; no install needed).
- Determinism: two independent extractions to `/tmp/api-a.txt` and `/tmp/api-b.txt` (both reporting `1968 items`), filtered to exclude the `# Public API Surface - Generated` timestamp header, diffed: **0 lines differ.**
- Regeneration: `bash scripts/extract-public-api.sh .project/current-exports.txt` → `✅ API surface extracted to .project/current-exports.txt (1968 items)`. `head -1` shows `# Public API Surface - Generated 2026-08-07 00:18:19 UTC` (today), replacing `2026-06-10 00:47:43 UTC`.
- `git diff --stat .project/current-exports.txt` → **48 changed lines (40 insertions, 8 deletions)**; item count `1956` → `1968` in the file's own summary line.
- **Diff characterization:** `ArsenalExecutionService` gained `invoke_tool_direct` and `register_client`; `ArsenalRegistryService` gained `list`; `MCPServerConfig` gained `auth_token_env` (both the `config::arsenal` and re-exported `config` paths); `PaladinBuilder::add_mcp_sse` was removed (both its module path and its `prelude` re-export); the `MCPTransport` trait and `mcp_sse_adapter` module were removed and replaced by a new `mcp_streamable_http_adapter` module (`BearerToken`, `MCPStreamableHttpAdapter` with `connect`/`endpoint`/`new`/`with_bearer_token`/`with_custom_headers`).
- **Positive direction (VALIDATION.md row 1):** `bash scripts/check-api-surface.sh .project/current-exports.txt` → `✅ API surface unchanged`, `EXIT:0` (captured via explicit `echo "EXIT:$?"` after the run, not through a masking pipe).
- **Negative direction (VALIDATION.md row 1b):** added `pub fn gsd_debt01_negative_probe() {}` to `src/lib.rs` (root facade, a public module of the extracted surface). Same command → `❌ API surface has changed!`, diff shows `+pub fn paladin::gsd_debt01_negative_probe()`, `EXIT:1`. Reverted via `git checkout -- src/lib.rs`; `git status --short` confirmed clean immediately after.

**Task 3 — check-deprecations.sh, both directions:**
- Clean-tree run: `bash scripts/check-deprecations.sh` → `🔍 Checking deprecation warnings...` / `✅ No deprecation warnings found` / `Checking for properly formatted deprecation attributes...` / `✅ All deprecation attributes are properly formatted`, `EXIT:0` (reproduced twice, identical both times).
- Negative-direction proof (VALIDATION.md row 1d): added a bare `#[deprecated]` (no `since`, no `note`) above `pub fn gsd_debt03_negative_probe() {}` in `crates/paladin-core/src/lib.rs`. Same command → `crates/paladin-core/src/lib.rs:#[deprecated]` / `❌ Found deprecation without 'since' or 'note' fields!` / remediation hint, `EXIT:1` — proving the check now covers `crates/`, which it provably did not before this task. Reverted via `git checkout -- crates/paladin-core/src/lib.rs`; confirmed clean.
- `grep -c 'crates' scripts/check-deprecations.sh` → `3` (≥ 1). `grep -vE '^\s*#' scripts/check-deprecations.sh | grep -c 'exit 0'` → `0` (both old unconditional early `exit 0`s gone). `grep -c 'set -euo pipefail' scripts/check-deprecations.sh` → `1`.
- Zero-deprecation-tree pass confirmed: `grep -rn '#\[deprecated' src crates | wc -l` → `0`; the script still exits 0 on this tree (no new gate, per D-05 prohibition).

**Phase-end workspace gate (CLAUDE.md):**
- `cargo fmt --check` → exit 0, no output.
- `cargo clippy --workspace -- -D warnings` → `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1m 00s`, exit 0, zero warnings.
- `cargo test --workspace --offline` → every `test result:` line in the run shows `0 failed` (representative samples: `691 passed; 0 failed; 14 ignored`, `419 passed; 0 failed; 11 ignored`, `96 passed; 0 failed` for `paladin-ports` doctests), overall exit 0.

## Issues Encountered

None blocking. Several long-running `cargo` invocations (the `-Dwarnings` recompile inside `check-deprecations.sh`'s primary branch, `cargo clippy --workspace`, and `cargo test --workspace`) each triggered a from-scratch recompile of large portions of the dependency graph because their `RUSTFLAGS`/flag sets differ from the pre-populated `target/` cache's fingerprint — each took several minutes but completed cleanly with no errors.

## Next Phase Readiness

- DEBT-01's tooling half (this plan) is closed: five stale path literals, a regenerated baseline proven end-to-end in both directions, and a `check-deprecations.sh` that can genuinely fail. Plan 08-05 owns DEBT-01's remaining half — the five stale requirement-text references (M8 Epic 7 FR-10, M12 Epic 1 §7, Epic 5 §7, Epic 6 `cross_refs`, Epic 7 FR-4.6).
- No blockers for downstream plans in this wave. The regenerated `.project/current-exports.txt` is now the correct baseline for any future public-API comparison in this phase or later ones.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*

## Self-Check: PASSED

All five claimed files verified present (`scripts/check-api-surface.sh`, `scripts/extract-public-api.sh`, `.github/workflows/ci.yml`, `.project/current-exports.txt`, `scripts/check-deprecations.sh`); all three claimed task commits (`7e02da3`, `1f7ea39`, `0be3486`) verified present in `git log --oneline --all`.
