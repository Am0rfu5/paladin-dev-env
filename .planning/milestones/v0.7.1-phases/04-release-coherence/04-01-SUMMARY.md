---
phase: 04-release-coherence
plan: 01
subsystem: infra
tags: [rust, cargo, edition-2024, cargo-fix, workspace-manifest, provenance]

# Dependency graph
requires:
  - phase: 03-verification-depth
    provides: the D-17 provenance-block template (03-coverage-measurement.md) this plan's
      04-release-measurement.md copies verbatim
provides:
  - "Workspace-wide Rust edition uniformity: all twelve edition-carrying manifests (root
    Cargo.toml + eleven member crates) declare edition = \"2024\", zero remain on 2021"
  - "04-release-measurement.md — the phase's single D-17 raw-evidence record, seeded with
    two complete measurement sections; plans 04-04/04-05/04-06 append to this same file"
  - "A recorded, honest finding that cargo build --workspace --no-default-features --offline
    is a structural no-op for the root paladin-ai package (workspace feature unification via
    crates/doc-examples/Cargo.toml:15), scoped and left for whichever future work touches
    that manifest or the CI feature-matrix job"
affects: [04-02, 04-03, 04-04, 04-05, 04-06, 04-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "cargo fix --edition run AFTER the manifest edition bump (not before, as the tool's own
      migration-route text suggests) — both orders were verified equivalent for these two
      crates (zero source rewrites either way), and the plan's task text prescribes edit-first"
    - "D-17 raw-evidence record: environment probes (rustc -vV, cargo --version, git rev-parse
      HEAD/--abbrev-ref HEAD, git status --porcelain with a disambiguating sentence, date -u)
      followed by verbatim command + verbatim stdout, one ## Entry measurement section per
      measured fact, sections accumulate and are never overwritten"

key-files:
  created:
    - .planning/phases/04-release-coherence/04-release-measurement.md
  modified:
    - crates/paladin-ports/Cargo.toml
    - crates/paladin-notifications/Cargo.toml

key-decisions:
  - "Followed the plan's edit-manifest-then-cargo-fix order literally, even though cargo fix
    --edition's own guidance text recommends the opposite order. Verified equivalent for both
    crates: git status --porcelain -- crates/<crate>/src showed zero files touched by cargo fix
    in either case, and both crates compiled clean under edition 2024 immediately after the
    manifest edit alone."
  - "Recorded the --no-default-features build leg's structural no-op finding transparently in
    04-release-measurement.md rather than silently reporting the leg as a full proof. Root
    cause traced to crates/doc-examples/Cargo.toml:15's paladin-ai dependency declaration
    (features = [\"web-server\"] without default-features = false), a pre-existing workspace
    fact unrelated to and out of scope for this plan's edition-only files_modified list."
  - "Corrected the plan's own count caveat in the measurement record: twelve manifests carry
    an edition key (root Cargo.toml + eleven member crates), not eleven — 04-CONTEXT.md and
    04-RESEARCH.md's \"eleven\" figures count member crates only."

requirements-completed: [REL-02]

coverage:
  - id: D1
    description: "crates/paladin-ports declares edition = \"2024\" (was 2021); doctest = false
      and the =0.6.0 exact pin left untouched; cargo fix --edition produces zero source
      rewrites; both cargo build --workspace legs exit 0"
    requirement: "REL-02"
    verification:
      - kind: other
        ref: "cargo build --workspace --offline && cargo build --workspace --no-default-features --offline && grep -q 'edition = \"2024\"' crates/paladin-ports/Cargo.toml && grep -q 'doctest = false' crates/paladin-ports/Cargo.toml"
        status: pass
    human_judgment: false
  - id: D2
    description: "crates/paladin-notifications declares edition = \"2024\" (was 2021); every
      one of the twelve edition-carrying manifests in the workspace now agrees on 2024, zero
      remain on 2021"
    requirement: "REL-02"
    verification:
      - kind: other
        ref: "grep -h '^edition' Cargo.toml crates/*/Cargo.toml | sort -u | wc -l == 1 && grep -h '^edition' Cargo.toml crates/*/Cargo.toml | grep -c 2024 == 12"
        status: pass
    human_judgment: false
  - id: D3
    description: "04-release-measurement.md created as the phase's D-17 raw-evidence record,
      carrying two complete provenance-blocked measurement sections plus the workspace
      uniformity assertion with raw output"
    verification:
      - kind: other
        ref: "grep -c '^## Entry measurement' .planning/phases/04-release-coherence/04-release-measurement.md == 2"
        status: pass
    human_judgment: false

duration: 13min
completed: 2026-08-03
status: complete
---

# Phase 4 Plan 01: Edition-2024 Uniformity Summary

**Closed the `edition = "2024"` / `"2021"` split across all twelve workspace manifests by bumping `paladin-ports` and `paladin-notifications`, proving both D-06 build legs green and recording an honest finding that the `--no-default-features` leg is a structural no-op for the root package.**

## Performance

- **Duration:** 13 min
- **Started:** 2026-08-03T00:13:46Z
- **Completed:** 2026-08-03T00:27:06Z
- **Tasks:** 2
- **Files modified:** 3 (2 manifests + 1 new measurement record)

## Accomplishments

- `crates/paladin-ports/Cargo.toml` and `crates/paladin-notifications/Cargo.toml` both bumped from `edition = "2021"` to `edition = "2024"` per D-04 — zero other keys touched (`doctest = false` and the `=0.6.0` exact pin on `paladin-ports` are byte-identical to before)
- `cargo fix --edition --offline -p <crate> --allow-dirty --lib` produced zero source rewrites for both crates — confirms RESEARCH.md's hazard analysis: the four mechanically-detectable hazards (`unsafe`, `static mut`, `gen` identifier, `no_mangle`) were provably absent, and the two semantic hazards (`if let`/tail-expression drop-order, match-ergonomics tightening) did not fire despite being live in `paladin-notifications`
- Both D-06 build legs — `cargo build --workspace --offline` and `cargo build --workspace --no-default-features --offline` — exit 0 on the final workspace state, all nine other member crates path-depending on `paladin-ports` compiled clean
- Workspace-wide uniformity proven: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | sort -u` returns exactly one line naming 2024; twelve manifests total (root `Cargo.toml` + eleven member crates), zero remain on 2021
- `.planning/phases/04-release-coherence/04-release-measurement.md` created as the phase's single D-17 raw-evidence record, seeded with two full provenance-blocked measurement sections that plans 04-04, 04-05 and 04-06 will append to

## Task Commits

Each task was committed atomically:

1. **Task 1 (tracer): End-to-end edition slice — bump `paladin-ports` and prove the whole workspace still builds** - `7d8e730` (feat)
2. **Task 2: Bump `paladin-notifications` and close the split** - `a05e607` (feat)

_Task 1 was a `type="tracer"` task. Per this executor's tracer feedback gate protocol, execution paused at a `checkpoint:human-verify` after Task 1's commit (auto mode initially read as inactive from an uncommitted config flag in this worktree's forked state). The coordinator confirmed the tracer slice was correctly verified — commit `7d8e730`, both build legs green, zero source rewrites, `doctest = false`/`=0.6.0` pin untouched — and clarified auto mode is in fact active for this phase run, so execution proceeded to Task 2 without further gates._

## Files Created/Modified

- `crates/paladin-ports/Cargo.toml` - `edition` key changed from `"2021"` to `"2024"`; no other line touched
- `crates/paladin-notifications/Cargo.toml` - `edition` key changed from `"2021"` to `"2024"`; no other line touched
- `.planning/phases/04-release-coherence/04-release-measurement.md` - new file; the phase's D-17 raw-evidence record, two `## Entry measurement` sections (one per crate), each with full environment probes, verbatim `cargo fix --edition` output, verbatim build-leg output, and acceptance-criteria verification transcripts

## Decisions Made

- **Manifest-edit-then-`cargo fix` order, despite the tool's own contrary guidance.** `cargo fix --edition`'s output text recommends starting from `edition = "2021"`, running the fix, then bumping the manifest — but the plan's task text and RESEARCH.md Part A, Q1 both prescribe editing the manifest first. Verified both crates required zero source rewrites either way (the manifest edit alone was sufficient for a clean edition-2024 build), so the two orderings are equivalent for this specific pair of crates. Recorded verbatim in the measurement file so a later reader isn't confused by the tool's warning text appearing in the transcript.
- **Recorded, not fixed, the `--no-default-features` leg's structural no-op.** `cargo metadata --no-default-features --offline --format-version 1` showed `paladin-ai`'s resolved feature set as `["default", "llm-openai", "web-server"]` even under the workspace-level `--no-default-features` flag. Root cause: `crates/doc-examples/Cargo.toml:15` declares `paladin-ai = { version = "0.6.0", path = "../..", features = ["web-server"] }` without `default-features = false`, and Cargo's additive `features = [...]` syntax doesn't disable a dependency's defaults on its own — workspace feature unification then re-enables `default`/`llm-openai` for every unit building `paladin-ai` in the same invocation, including the flagged one. This is a pre-existing structural fact of the workspace's feature graph, unrelated to the edition bump and outside this plan's `files_modified` scope. It does **not** invalidate this plan's edition proof: neither `paladin-ports` nor `paladin-notifications` declares a `default` feature of its own, so both crates' migration is fully proven by both legs regardless. Recorded transparently in `04-release-measurement.md` per D-17/T-04-02 rather than glossed over as a clean pass — disposition left to whichever future work next touches `crates/doc-examples/Cargo.toml` or CI's feature-matrix job (Plan 04-03's territory).
- **Corrected the plan's own "eleven manifests" language in the record.** Twelve manifests carry an `edition` key — the root `Cargo.toml` (the `paladin-ai` workspace package) plus eleven member crates. `04-CONTEXT.md` and `04-RESEARCH.md` say "eleven" in places because they count member crates only. The measurement record states this plainly so downstream plans cite the twelve-manifest figure going forward.

## Deviations from Plan

None — plan executed exactly as written. The tracer feedback gate pause (Task 1 checkpoint, resolved by the coordinator) and the honest `--no-default-features` finding are both explicitly anticipated by the plan's own text (the tracer task type and D-17's transparency requirement, respectively), not unplanned deviations from it.

## Issues Encountered

- `cargo fix --edition` printed a "unable to migrate further" warning for both crates because the manifest was already at `edition = "2024"` when the fix ran (per the plan's prescribed task order). This was investigated and confirmed harmless: `git status --porcelain -- crates/<crate>/src` showed zero files touched either before or after, and the subsequent `cargo build --workspace --offline` compiled both crates with zero errors and zero edition-migration diagnostics. Not a blocker — resolved by direct verification, documented verbatim in the measurement record rather than suppressed.
- The `cargo build --workspace --no-default-features --offline` leg was found to be a structural no-op for the root `paladin-ai` package (see Decisions Made above). Investigated to root cause via `cargo metadata` inspection rather than accepted at face value; recorded as a finding, not silently passed as a full proof.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- REL-02's code fix is complete and proven: all twelve workspace manifests declare `edition = "2024"`, both D-06 build legs are green, and the proof is recorded with full D-17 provenance.
- `04-release-measurement.md` is seeded and ready for plans 04-04 (advisory posture), 04-05 (version bump / tag), and 04-06 (gate suite) to append their own measurement sections to the same file, per the plan's stated file-ownership boundary.
- The edition ADR (D-06, `0009-*.md`) that records *why* 2024 rather than 2021 is Plan 04-07's — this plan supplies the proof the ADR will cite, not the ADR itself.
- The `--no-default-features` feature-unification finding on `crates/doc-examples/Cargo.toml:15` is flagged for whichever plan next touches that manifest or authors the CI examples/feature-matrix job (04-03's likely territory per D-12) — no action required from this plan, but it should not be silently rediscovered.

---
*Phase: 04-release-coherence*
*Completed: 2026-08-03*
