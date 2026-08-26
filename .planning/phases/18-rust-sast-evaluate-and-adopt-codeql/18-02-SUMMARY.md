---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 02
subsystem: infra
tags: [codeql, sast, rust, fixture, gitleaks, github-actions]

requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql
    provides: .github/workflows/codeql.yml (proven end-to-end), scripts/codeql-analysed-files.sh, 18-CODEQL-EVIDENCE.md schema
provides:
  - fixtures/codeql-probe/ — standalone, workspace-excluded crate carrying five deliberately planted vulnerability classes (four unconditional, reused verbatim from the Snyk-era methodology, plus one behind a non-default cargo feature)
  - workspace.exclude entry on the root Cargo.toml keeping the fixture out of every build/lint/coverage run
  - Empirical proof that the planted credential survives the repository's own gitleaks gate with no allowlist bypass
  - .github/codeql/codeql-config.yml (steady-state scan scope, fixture excluded) and .github/codeql/codeql-config-probe.yml (probe scan scope, fixture included)
  - scan_probe_fixture workflow_dispatch input on codeql.yml, wired into the init step's config-file selection
affects: [18-03, 18-04]

tech-stack:
  added: []
  patterns:
    - "Standalone fixture crate with its own empty [workspace] table, located outside crates/, plus a defence-in-depth workspace.exclude entry — three independent mechanisms keeping intentionally-vulnerable code out of the real build graph"
    - "Two named, committed CodeQL configs (steady-state vs. probe) selected by a workflow_dispatch boolean input, so a fixture can be re-scanned on demand without emitting standing alerts on every PR"

key-files:
  created:
    - fixtures/codeql-probe/Cargo.toml
    - fixtures/codeql-probe/Cargo.lock
    - fixtures/codeql-probe/src/lib.rs
    - fixtures/codeql-probe/src/credential.rs
    - fixtures/codeql-probe/src/command_injection.rs
    - fixtures/codeql-probe/src/path_traversal.rs
    - fixtures/codeql-probe/src/sql_injection.rs
    - fixtures/codeql-probe/src/feature_gated.rs
    - fixtures/codeql-probe/.gitignore
    - .github/codeql/codeql-config.yml
    - .github/codeql/codeql-config-probe.yml
  modified:
    - Cargo.toml
    - .github/workflows/codeql.yml

key-decisions:
  - "The planted credential (fixtures/codeql-probe/src/credential.rs) survived gitleaks's default ruleset unmodified — a low-entropy, obviously-synthetic string (\"planted-fixture-credential-not-real-0000000000\") did not match any default rule. .gitleaks.toml was left completely unchanged; no allowlist entry was needed."
  - "The [features] table with probe-feature-gated was declared in fixtures/codeql-probe/Cargo.toml during Task 1 (ahead of Task 3's own scope) so Task 1's crate compiled cleanly against its final shape from the start; Task 3 only added the fifth defect file and the two CodeQL configs plus the workflow_dispatch wiring on top of that pre-existing feature table."
  - "fixtures/codeql-probe/.gitignore (/target) was added as an out-of-plan but necessary addition (Rule 2/3): the fixture crate is its own independent workspace root (D-07/D-11 requirement), so the repository root's /target gitignore entry does not cover fixtures/codeql-probe/target/ — without this, cargo check would leave build artifacts as untracked cruft in every future git status."

patterns-established:
  - "A vulnerability probe fixture is proven inert against the whole toolchain (cargo metadata, cargo build --workspace, cargo clippy --workspace --all-targets --all-features, pre-commit run --all-files) rather than just against cargo metadata alone, closing the gap between 'not a workspace member' and 'genuinely never compiled/linted'."

requirements-completed: [SAST-01]

coverage:
  - id: D1
    description: "Probe crate fixtures/codeql-probe/ compiles standalone, carries the four Snyk-era defect classes (credential, command_injection, path_traversal, sql_injection) declared in that exact order in lib.rs, and is provably absent from the workspace's package graph and from any crate's path-dependency list"
    requirement: "SAST-01"
    verification:
      - kind: integration
        ref: "cargo check --manifest-path fixtures/codeql-probe/Cargo.toml"
        status: pass
      - kind: integration
        ref: "cargo metadata --no-deps --format-version 1 (paladin-codeql-probe absent, 12 workspace members)"
        status: pass
      - kind: unit
        ref: "python3 module-order assertion against fixtures/codeql-probe/src/lib.rs"
        status: pass
      - kind: integration
        ref: "grep -rn codeql-probe --include=Cargo.toml crates/ src/ (no output)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Planted credential survives the repository's required gitleaks secret-scanning gate with no bypass, and the workspace build/lint toolchain never compiles or lints the fixture"
    requirement: "SAST-01"
    verification:
      - kind: integration
        ref: "pre-commit run --all-files (all 10 hooks, including gitleaks, cargo-fmt, cargo-clippy)"
        status: pass
      - kind: integration
        ref: "cargo build --workspace output grep for paladin-codeql-probe (0 matches)"
        status: pass
      - kind: integration
        ref: "cargo clippy --workspace --all-targets --all-features -- -D warnings output grep for paladin-codeql-probe (0 matches)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Fifth defect class planted behind the non-default probe-feature-gated cargo feature, and two named CodeQL configs (steady-state excludes the fixture, probe includes it) wired into codeql.yml via a scan_probe_fixture workflow_dispatch input, with the job's D-02 name contract and trigger-policy register row unchanged"
    requirement: "SAST-01"
    verification:
      - kind: integration
        ref: "cargo check --manifest-path fixtures/codeql-probe/Cargo.toml --features probe-feature-gated"
        status: pass
      - kind: integration
        ref: "bash scripts/check-workflow-triggers.sh"
        status: pass
      - kind: unit
        ref: "python3 YAML-structural assertion (paths-ignore split, workflow_dispatch input shape, config-file expression, job name/no-strategy)"
        status: pass
      - kind: integration
        ref: "git diff --name-only HEAD~1 HEAD (branching-model.md register row unchanged)"
        status: pass
    human_judgment: false

duration: ~35min active work
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 02: CodeQL Probe Fixture and Steady-State/Probe Scan Split Summary

**A standalone, workspace-excluded `fixtures/codeql-probe/` crate carrying five deliberately planted vulnerability classes (the four Snyk-era classes plus one behind a non-default cargo feature) — proven invisible to `cargo metadata`, `cargo build --workspace`, `cargo clippy --workspace --all-features`, and green against the repository's own gitleaks gate with no bypass — plus two named CodeQL configs wired into `codeql.yml` via a `scan_probe_fixture` dispatch input so the fixture scans on demand without ever emitting standing PR alerts.**

## Performance

- **Duration:** ~35 min active work
- **Started:** 2026-08-25 (Task 1 commit `cebabb2e`)
- **Completed:** 2026-08-25 (Task 3 commit `fc2ae7b5`)
- **Tasks:** 3/3 (Task 2 required no file changes — verification only)
- **Files modified:** 13 (9 new fixture files, 2 new CodeQL config files, 2 modified: root Cargo.toml, codeql.yml)

## Accomplishments

- `fixtures/codeql-probe/` — standalone crate with its own empty `[workspace]` table, `publish = false`, located outside `crates/` (so the `crates/*` glob cannot pick it up), plus an explicit `workspace.exclude = ["fixtures/codeql-probe"]` entry on the root manifest as defence in depth. `cargo metadata --no-deps` confirms it is absent from the 12-member workspace package graph.
- Four planted defect classes in the exact order the Snyk evaluation was measured against — `credential.rs` (hardcoded, synthetic, low-entropy `Authorization` header value), `command_injection.rs` (`sh -c` with interpolated caller input), `path_traversal.rs` (unsanitised `PathBuf::join` + `read_to_string`), `sql_injection.rs` (`format!`-interpolated query string through `sqlx::query_as` against a `SqlitePool`) — declared in that order in `lib.rs`, structurally verified.
- Planted credential survived `gitleaks` (via `pre-commit run --all-files`, all 10 hooks including `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`) with **zero changes to `.gitleaks.toml`** — the synthetic value alone was enough, no allowlist bypass needed.
- Fifth defect (`feature_gated.rs`) planted behind a new `probe-feature-gated` cargo feature (default `[]`), reusing the exact command-injection shape so the empirical D-12 answer isolates a single variable: whether extraction reaches feature-gated code at all.
- Two named, committed CodeQL configs — `.github/codeql/codeql-config.yml` (steady-state, `paths-ignore: [fixtures/codeql-probe]`) and `.github/codeql/codeql-config-probe.yml` (probe mode, no ignore) — wired into `codeql.yml`'s init step via a `scan_probe_fixture` boolean `workflow_dispatch` input (default `false`), selecting the probe config only on an explicit dispatch with that input `true`.
- `codeql.yml`'s D-02 job-name contract (`CodeQL Analysis (Rust)`, no `strategy`/matrix) and the `docs/src/contributing/branching-model.md` trigger-policy register row are both provably unchanged by this plan — `scripts/check-workflow-triggers.sh` passes and the register row is absent from the diff.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create the workspace-excluded probe crate with the four planted defect classes** - `cebabb2e` (feat)
2. **Task 2: Make the planted credential survive secret scanning without a bypass** - _no commit — verification-only task, zero file changes required_ (gitleaks passed the synthetic value as-is)
3. **Task 3: Plant the feature-gated fifth defect and split the probe scan from the steady-state scan** - `fc2ae7b5` (feat)

_No TDD tasks in this plan — all three tasks are `type="auto"`._

## Files Created/Modified

- `fixtures/codeql-probe/Cargo.toml` - Standalone fixture crate manifest, own `[workspace]` table, `publish = false`, `[features] probe-feature-gated`
- `fixtures/codeql-probe/Cargo.lock` - Committed lockfile for reproducibility (its own independent resolution, not the root lockfile)
- `fixtures/codeql-probe/src/lib.rs` - Declares the five defect modules, four unconditional in the recorded order plus `feature_gated` behind `#[cfg(feature = "probe-feature-gated")]`
- `fixtures/codeql-probe/src/credential.rs` - Planted defect class 1: hardcoded credential
- `fixtures/codeql-probe/src/command_injection.rs` - Planted defect class 2: `sh -c` shell command injection
- `fixtures/codeql-probe/src/path_traversal.rs` - Planted defect class 3: unsanitised path join + read
- `fixtures/codeql-probe/src/sql_injection.rs` - Planted defect class 4: `format!`-interpolated SQL via `sqlx::query_as`
- `fixtures/codeql-probe/src/feature_gated.rs` - Planted defect class 5: identical command-injection shape, gated behind `probe-feature-gated`
- `fixtures/codeql-probe/.gitignore` - `/target` — this fixture is its own workspace root, not covered by the repo-root `/target` ignore entry
- `.github/codeql/codeql-config.yml` - Steady-state CodeQL analysis scope, excludes the fixture
- `.github/codeql/codeql-config-probe.yml` - Probe-mode CodeQL analysis scope, includes the fixture
- `Cargo.toml` - Root workspace table gains `exclude = ["fixtures/codeql-probe"]`
- `.github/workflows/codeql.yml` - `workflow_dispatch.inputs.scan_probe_fixture` boolean input added; init step's `config-file` now selects between the two configs based on that input

## Decisions Made

- **No `.gitleaks.toml` changes needed.** The plan's Task 2 anticipated a possible allowlist addition if the default gitleaks rules flagged the planted credential. They did not — `"planted-fixture-credential-not-real-0000000000"` is low-entropy and pattern-obviously-fake enough to pass `pre-commit run --all-files` unmodified. Recorded per the plan's own instruction: "If the gitleaks hook is clean, change nothing and record that in the task's summary."
- **`[features]` table added in Task 1's commit, ahead of its Task 3 acceptance criteria.** Rather than adding an empty features table in Task 1 and expanding it in Task 3, the full `probe-feature-gated` feature declaration was written once in Task 1 so the crate's final shape compiled cleanly from its first commit. Task 3 verified the pre-existing table still satisfies its own acceptance criteria (`default` list excludes `probe-feature-gated`) and added only the new source file plus the workflow/config wiring.
- **`fixtures/codeql-probe/.gitignore` added (Rule 2/3, out-of-plan but necessary).** The fixture is deliberately its own independent Cargo workspace root (a Task 1 design requirement, D-07/D-11), which means `cargo check`/`cargo build` inside it produces a `fixtures/codeql-probe/target/` directory the repository-root `.gitignore`'s `/target` entry (anchored to the repo root) does not match. Left unaddressed, every future `cargo check` against the fixture would leave build artifacts as untracked cruft. Added a scoped `/target` ignore inside the fixture's own directory.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added `fixtures/codeql-probe/.gitignore`**
- **Found during:** Task 1, after `cargo generate-lockfile` / `cargo check` produced a `fixtures/codeql-probe/target/` directory
- **Issue:** The fixture's own `[workspace]` table (required so Cargo treats it as an independent workspace root, per Task 1's design) means its build output lands in `fixtures/codeql-probe/target/`, which the repository-root `.gitignore`'s `/target` entry (anchored, non-recursive) does not cover. Without a fix, `git status` would show untracked build artifacts after every fixture-crate build.
- **Fix:** Added `fixtures/codeql-probe/.gitignore` containing `/target`, scoped to the fixture's own directory.
- **Files modified:** `fixtures/codeql-probe/.gitignore` (new)
- **Verification:** `git status --short` after `cargo check --manifest-path fixtures/codeql-probe/Cargo.toml` shows no untracked `target/` entries.
- **Committed in:** `cebabb2e` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 missing-critical addition)
**Impact on plan:** Necessary build-hygiene fix directly caused by the fixture's own required standalone-workspace design; no scope creep, no change to the plan's deliverables.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `fixtures/codeql-probe/` is committed, compiles clean (both with and without `probe-feature-gated`), and is provably invisible to the workspace's build, lint, and package graph — ready for plan 18-03 to actually dispatch `codeql.yml` with `scan_probe_fixture: true` and read the resulting finding count against the Promotion Criteria's disqualifying condition #1 (D-11: a probe finding count of exactly 0 across all four classes disqualifies the tool).
- The `probe-feature-gated` fifth defect is ready for 18-03 to read alongside the direct file-presence evidence from `scripts/codeql-analysed-files.sh` — if the probe run reports `feature_gated` findings alongside `command_injection` findings, D-12's open question closes empirically for the probe path too (complementing 18-01's file-reach-only answer).
- The two CodeQL configs (`codeql-config.yml`, `codeql-config-probe.yml`) and the `scan_probe_fixture` dispatch input are committed and structurally verified but **not yet exercised by a real run** — 18-03 is expected to be the plan that actually dispatches the probe-mode scan.
- `18-CODEQL-EVIDENCE.md` was read but not modified by this plan (Task 2's "if neither works" fallback path — recording a secret-scanning-interaction finding there — was not reached, since gitleaks passed cleanly).

## Threat Flags

None found — this plan's new surface (a standalone fixture crate excluded from the workspace, two CodeQL analysis-scope configs, and a workflow_dispatch input) matches the phase's own `<threat_model>` register (T-18-07 through T-18-10, T-18-SC) exactly; every mitigation named there (location outside `crates/`, explicit `workspace.exclude`, own `[workspace]` table, `publish = false`, synthetic non-resolving credential value, `paths-ignore` split, fixed defect-class order) was implemented as specified and independently verified. No new trust boundary or security-relevant surface was introduced beyond what PLAN.md already threat-modelled.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*

## Self-Check: PASSED

All files created in this plan verified present on disk; commits `cebabb2e`, `fc2ae7b5`, and `30d3b79b` verified present in `git log --oneline`. No missing items.
