---
phase: 08-verified-defect-closure
plan: 08
subsystem: infra
tags: [docker, ci, cargo-features, cli-isolation, changelog, adr]

# Dependency graph
requires:
  - phase: 08-verified-defect-closure
    provides: plan 08-07 — the `paladin` [[bin]] required-features = ["cli"] gate and paladin-herald's table/color feature split (ADR-0023 Site 1 and Site 2)
provides:
  - "Both Dockerfiles (Dockerfile:33, Dockerfile.chef:74) build the paladin binary with --features cli — release build verified locally end to end"
  - "feature-flags.yml's inverted cli-isolation step repurposed to assert the gate (fails if the un-featured build unexpectedly succeeds) instead of asserting its opposite"
  - "docs/src/deployment/docker.md's build line byte-identical to Dockerfile:33, plus a cli-feature-requirement note"
  - "CHANGELOG.md [Unreleased] carries both DEBT-04 user-visible changes, each with its remedy and an ADR-0023 citation"
  - "ADR-0023's criterion-4 PENDING slot replaced with the captured cargo tree evidence (both invocations, the mockito/colored dev-dependency explanation, per D-16)"
affects: [08-09]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "CI assertion inversion repaired by repurposing the step (fail-if-succeeds body) rather than deleting it — the gate stays guarded by CI, per T-08-08-02's mitigation"
    - "cargo tree -e normal to exclude dev-dependency edges when a literal criterion command's grep hits a mockito-only transitive collision"

key-files:
  created: []
  modified:
    - Dockerfile
    - Dockerfile.chef
    - .github/workflows/feature-flags.yml
    - docs/src/deployment/docker.md
    - CHANGELOG.md
    - .planning/decisions/0023-cli-dependency-isolation.md

key-decisions:
  - "Dockerfile.chef:55's cargo chef cook stage left unchanged — it passes no explicit feature set today (no --features/--no-default-features flag at all), so the plan's conditional ('if that stage passes an explicit feature set, add cli there too') does not apply. Recorded as a caching-efficiency note, not a defect: the cook stage's dependency cache won't pre-build the cli-only deps (clap, colored, comfy-table, dialoguer, indicatif, console, serde_yaml), so app-builder's --features cli build compiles them fresh on every image build. Correctness is unaffected — cargo simply resolves what the cache doesn't have."
  - "docs/src/deployment/docker.md:156's CMD [\"/usr/local/bin/paladin\"] documentation (a single-line simplification of the shipped Dockerfile's two-line ENTRYPOINT+CMD split) left untouched. This is a pre-existing drift unrelated to the --features cli change and outside DEBT-04's scope (not named in RESEARCH.md's nine-row triage); fixing it would be new scope under the deviation rules' scope-boundary guidance. Recorded here rather than silently left unexamined."
  - "feature-flags.yml's repurposed step body uses a shell if/then rather than a one-liner (`! cargo build --bin paladin`) so both branches print an explicit, differently-worded message — easier to read in a CI log than a bare non-zero exit."

requirements-completed: [DEBT-04]

coverage:
  - id: D1
    description: "Dockerfile:33 and Dockerfile.chef:74 build the paladin binary with --features cli"
    requirement: "DEBT-04"
    verification:
      - kind: other
        ref: "cargo build --offline --release --workspace --bin paladin --features cli (exact Dockerfile:33 command, run locally since docker build is not runnable in this environment)"
        status: pass
    human_judgment: false
  - id: D2
    description: "feature-flags.yml's inverted cli-isolation step repurposed to assert the gate holds"
    requirement: "DEBT-04"
    verification:
      - kind: other
        ref: "gate-check.sh (if/then wrapping cargo build --bin paladin) run locally against the gated tree — exits 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "docs/src/deployment/docker.md's build line matches Dockerfile:33 byte-for-byte"
    requirement: "DEBT-04"
    verification:
      - kind: other
        ref: "grep -o 'cargo build --release --workspace --bin paladin[^\"]*' on both files, compared line for line"
        status: pass
    human_judgment: false
  - id: D4
    description: "Criterion 4 proved by running the command: no structopt/colored/comfy-table in a library-only downstream consumer's build graph"
    requirement: "DEBT-04"
    verification:
      - kind: other
        ref: "cargo build --offline --lib --no-default-features (exit 0), then cargo tree --offline --no-default-features -e normal | grep -E 'structopt|colored|comfy-table' (no output)"
        status: pass
    human_judgment: false
  - id: D5
    description: "CHANGELOG.md records both user-visible changes with remedies, citing ADR-0023"
    requirement: "DEBT-04"
    verification: []
    human_judgment: true
    rationale: "Readability/completeness as a consumer-facing document is a judgment call, per VALIDATION.md's Manual-Only row 3 (human_judgment: true)."
  - id: D6
    description: "Downstream sweep: all four repaired surfaces re-read, five out-of-scope surfaces named and left untouched"
    requirement: "DEBT-04"
    verification: []
    human_judgment: true
    rationale: "Confirming reconciliation against RESEARCH.md's nine-row triage table is a judgment call, per VALIDATION.md's Manual-Only row 2 (human_judgment: true)."

duration: ~40min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 08: Downstream Build-Surface Sweep for CLI Dependency Isolation (DEBT-04) Summary

**Repaired the four downstream build surfaces plan 08-07's `paladin` binary gate broke (both Dockerfiles, the inverted CI step, the deployment doc), recorded both user-visible changes in CHANGELOG.md, and proved ROADMAP criterion 4 by running the command and filling ADR-0023's PENDING slot with the verbatim evidence.**

## Performance

- **Duration:** ~40 min
- **Tasks:** 3 (all completed)
- **Files modified:** 6

## Accomplishments

- `Dockerfile:33` and `Dockerfile.chef:74` both now run `cargo build --release --workspace --bin paladin --features cli`. Verified by running the exact command locally (release profile, ~5m17s, exit 0) — this is not a build I could run through `docker build` in this environment (see Verification below), but the command that image build stage executes is proven.
- `.github/workflows/feature-flags.yml`'s inverted step — "Verify paladin binary builds without cli feature" running a bare `cargo build --bin paladin` — is repurposed, not deleted, to "Verify paladin binary requires the cli feature": its body now fails the CI job if that build unexpectedly succeeds, and passes when it correctly fails. Locally verified against the gated tree: the un-featured build fails as expected, and the wrapping script therefore exits 0.
- `docs/src/deployment/docker.md`'s documented build line is now byte-identical to `Dockerfile:33`, and a new sentence tells a reader the `paladin` binary requires the `cli` feature before they copy the command.
- The five out-of-scope surfaces RESEARCH.md's triage named (`docker/testserver/Dockerfile`, `docker/redis/Dockerfile:107`, `k8s/deployment.yaml:65,68`, `Makefile:307`, `docker-compose.dev.yml:36`) were each re-confirmed this session and left untouched.
- `CHANGELOG.md`'s `[Unreleased] ### Changed` gained two consumer-readable entries (the binary gate, the Herald feature split), each citing ADR-0023 with its exact remedy command/flag.
- ADR-0023's `Output: PENDING` line is replaced with the full captured evidence: both `cargo tree` invocations (the literal one-hit-on-a-dev-dependency form and the clean `-e normal` form), the preceding successful library-only build, and the `structopt`-removal confirmation.

## Task Commits

Each task was committed atomically:

1. **Task 1: Repair every downstream surface that assumed the binary builds by default** - `24ccbae` (fix)
2. **Task 2: Record both user-visible changes in CHANGELOG.md** - `b0629de` (docs)
3. **Task 3: Prove criterion 4 by running the command and record it in ADR-0023** - `df6935a` (docs)

## Files Created/Modified

- `Dockerfile` - build line gains `--features cli`, with an explanatory comment
- `Dockerfile.chef` - app-builder stage's build line gains `--features cli`, with an explanatory comment; the earlier `cargo chef cook` stage (line 55) is unchanged (see Decisions Made)
- `.github/workflows/feature-flags.yml` - the cli-isolation job's inverted step renamed and its body repurposed to a fail-if-succeeds assertion
- `docs/src/deployment/docker.md` - build line updated to match `Dockerfile:33`; a sentence added noting the `cli` feature requirement
- `CHANGELOG.md` - two `[Unreleased] ### Changed` entries added
- `.planning/decisions/0023-cli-dependency-isolation.md` - the single `Output: PENDING` line replaced with the captured criterion-4 evidence block

## Decisions Made

- **`Dockerfile.chef:55`'s `cargo chef cook` stage left unchanged.** It passes no explicit feature set (no `--features`/`--no-default-features` flag at all), so the plan's own conditional — "if that stage passes an explicit feature set, add cli there too" — does not apply. This is a caching-efficiency note, not a defect: the chef cook layer's dependency cache won't include the `cli`-only crates (`clap`, `colored`, `comfy-table`, `dialoguer`, `indicatif`, `console`, `serde_yaml`), so `app-builder`'s `--features cli` build compiles them fresh every image build rather than reusing a cached layer. The build still succeeds; only the caching optimization cargo-chef exists to provide is partially bypassed for these specific dependencies.
- **`docs/src/deployment/docker.md:156`'s `CMD ["/usr/local/bin/paladin"]` left unchanged.** This line is a pre-existing simplification of the shipped Dockerfile's actual `ENTRYPOINT ["/usr/local/bin/paladin"]` / `CMD ["--help"]` two-line split — it predates this plan, is unrelated to the `--features cli` change, and is not one of RESEARCH.md's nine triaged rows. Fixing it would be new scope under the deviation rules' scope-boundary guidance ("only auto-fix issues directly caused by the current task's changes"). Recorded here rather than silently passed over.
- **Repurposed CI step uses an `if`/`then` shell body**, not a one-line `! cargo build --bin paladin`, so both the pass and fail paths print an explicit, differently-worded message to the CI log — easier to diagnose than a bare non-zero exit with no context.

## Deviations from Plan

None - plan executed exactly as written. No Rule 1-4 auto-fixes were needed; the two items above (chef cook stage, docs CMD line) are explicit **non-changes** the plan directed me to evaluate and record, not deviations.

## Downstream Sweep — Reconciled Against RESEARCH.md's Nine-Row Triage

Re-ran the enumerating grep this session:

```
$ grep -rnE '\-\-bin paladin\b|bin/paladin\b|\./paladin\b' Dockerfile* Makefile docker/ k8s/ docs/src/ .github/workflows/
```

Reconciliation, row by row:

| # | Surface | Disposition | Evidence |
|---|---|---|---|
| 1 | `Dockerfile:33` | **Fixed** | `--features cli` added; release build verified to exit 0 |
| 2 | `Dockerfile.chef:74` | **Fixed** | `--features cli` added; `cargo build --bin paladin --features cli` verified to exit 0 |
| 3 | `.github/workflows/feature-flags.yml:143-144` | **Fixed (repurposed)** | Step renamed and body inverted to a fail-if-succeeds assertion; verified locally |
| 4 | `docs/src/deployment/docker.md:135` | **Fixed** | Build line now byte-identical to `Dockerfile:33` |
| 5 | `docker/testserver/Dockerfile:44` (`CMD ["./paladin"]`, `production` stage) | **Out of scope, untouched** | `docker/docker-compose.test.yml:66-69` targets the `test` stage, not `production` — this stage is unreferenced by any tracked build path |
| 6 | `docker/redis/Dockerfile:107` (`CMD ["./paladin"]`) | **Out of scope, untouched** | Orphaned command in a Redis-purpose Dockerfile; pre-existing, unrelated to the `paladin` binary's own build |
| 7 | `k8s/deployment.yaml:65` (`image: paladin:test`) | **Out of scope, untouched** | Explicitly-labelled placeholder image tag, not a build instruction |
| 8 | `Makefile:307` (`build-docker` target, `-f docker/Dockerfile`) | **Out of scope, untouched** | References a nonexistent path (only `Dockerfile`, `Dockerfile.chef`, `Dockerfile.server` exist at repo root); pre-existing breakage unrelated to the feature gate |
| 9 | `docker/docker-compose.dev.yml:36` (`command: cargo run`) | **Out of scope, untouched** | Bare `cargo run` against three `[[bin]]` targets with no `default-run` was already ambiguous before this change; the gate sharpens the ambiguity but did not create it |

The re-run grep additionally surfaced references to `paladin-cli` and `paladin-server` (already `required-features`-gated siblings, unaffected by this plan), `paladin-chart`/`./paladin-chart` (a Helm chart directory name, not a binary invocation), and `docs/src/appendix/*.md`'s `./paladin ...` CLI examples (which document `paladin-cli` symlinked to the name `paladin` per `cli-usage.md:71`, not the root `paladin` binary this ADR gates). None of these are part of the un-gated `paladin` binary's reference set; none required changes.

## Two `CHANGELOG.md` Entries (verbatim)

```markdown
- **Breaking (build): the `paladin` binary now requires the `cli` feature.** `cargo run` and
  `cargo build --bin paladin` no longer build that binary without `--features cli` — it now
  carries `required-features = ["cli"]`, the same gate its two siblings (`paladin-cli`,
  `paladin-server`) already had, making all three `[[bin]]` targets consistent with
  ADR-0019's three-binary architecture. **Remedy:** run `cargo build --bin paladin --features cli`
  (or `cargo run --features cli`) instead of the bare command. `Dockerfile` and `Dockerfile.chef`
  were updated to pass `--features cli` in their release build stage. Underneath this gate,
  `src/main.rs` was also migrated from `structopt` (removed from the workspace entirely) to
  `clap` v4, with identical flags (`-c` / `--config`, default `config.yml`) and an unchanged
  binary name (`smartcontent-aggregator`) — a caller's invocation and arguments do not change,
  only the feature requirement does. See
  [ADR-0023](.planning/decisions/0023-cli-dependency-isolation.md).
- **Breaking (library): `paladin-herald`'s table and coloured-markdown formatters moved behind
  features.** `paladin-herald` gained its first `[features]` section (`default = []`, `table`,
  `color`). `TableHerald` is now available only under the `table` feature, and `MarkdownHerald`'s
  coloured rendering path (status badges, bold fields, the coloured error heading) is now behind
  the `color` feature — `MarkdownHerald` itself stays constructible and functional without it,
  falling back to plain text. `JsonHerald` and the uncoloured `MarkdownHerald` remain available in
  a default (featureless) build. **Remedy:** add `features = ["table", "color"]` to the
  `paladin-herald` dependency, or depend on the root `paladin` crate's `cli` feature, which enables
  both. Two consequences a downstream consumer can observe: (1)
  `paladin::infrastructure::adapters::herald::TableHerald` is available only when the root `cli`
  feature is enabled; (2) `Settings::create_default_herald()` called with
  `herald.default_formatter = "table"` in a build without `cli` returns the existing
  `Unknown formatter 'table'. Valid options: json, markdown` error instead of constructing a table
  Herald. `paladin-herald` is published on crates.io, so this is a change to its default public
  API. See [ADR-0023](.planning/decisions/0023-cli-dependency-isolation.md).
```

## ADR-0023 `PENDING`→Evidence Replacement (verbatim)

The single line `Output: PENDING — filled by plan 08-08 once the code lands.` was replaced with:

```
Output, captured 2026-08-07 by plan 08-08 (worktree HEAD after 08-07's manifest/source changes
landed):

$ cargo build --offline --lib --no-default-features
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 41s
Exit 0 — the precondition per RESEARCH.md Pitfall 2.

$ cargo tree --offline --no-default-features | grep -E 'structopt|colored|comfy-table'
│   ├── colored v3.1.1
One match on the literal command — not a criterion-4 violation. Traced:
`cargo tree --offline --no-default-features -i colored` →
colored v3.1.1 └── mockito v1.7.2 [dev-dependencies] └── paladin-ai v0.7.0.
This is colored 3.1.1, a transitive dev-dependency of the pre-existing mockito = "1.7.0" entry
(Cargo.toml:143, untouched by this decision) — a different major version from the colored 2.1
this decision gates, reached only through a dev-dependency edge. Cargo never propagates
dev-dependencies into a downstream consumer's build graph.

$ cargo tree --offline --no-default-features -e normal | grep -E 'structopt|colored|comfy-table'
(no output — exit 1)
Zero matches — the true criterion-4 view of what a downstream library consumer compiles.

$ cargo build --offline --bin paladin --features cli
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 20.13s
Exit 0.

$ grep -c structopt Cargo.toml
0
$ grep -rln structopt src/ crates/
(no output)
```

Both invocations (the literal one-hit form and the `-e normal` clean form) and the dev-dependency
explanation are recorded per D-16 — the literal command's one hit is not silently dropped, and the
substitute invocation's equivalence is explained rather than assumed.

## Issues Encountered

**The first edit attempt of the Dockerfile comments broke the acceptance-criteria grep.** My first draft of the explanatory comments above the `RUN cargo build ... --bin paladin --features cli` lines contained the literal substring `` `--bin paladin` `` on its own comment line without `features cli` on the same line — which is exactly the pattern the plan's own acceptance criterion `grep -rnE '\-\-bin paladin\b' Dockerfile Dockerfile.chef | grep -v 'features cli'` checks for (intentionally, to catch a real un-featured build left behind). It correctly flagged my comment as a false positive. Reworded both comments to avoid the bare `--bin paladin` substring; re-ran the grep, confirmed clean (`NO-UNFEATURED-BUILDS`). Not a deviation from the plan — a self-caught authoring mistake in explanatory prose, fixed before committing.

## Verification (recorded verbatim)

`docker build` is not runnable in this environment (confirmed absent from PATH / no daemon) and `mdbook` is not installed here (its `mdbook-linkcheck` install needs crates.io, which returns HTTP 403 in this environment) — both matched the plan's stated environment constraints exactly. Neither was attempted. In their place: the Dockerfile edits are verified by running the *exact* `cargo build` invocation each `RUN` line now contains, and the `docs/src/deployment/docker.md` edit is verified as a byte-identical prose match to the shipped Dockerfile line rather than a rendered/linked mdbook page (the edit is inside a fenced code block and moves no link target, so linkcheck risk is structurally nil).

```
$ cargo build --offline --release --workspace --bin paladin --features cli
    Finished `release` profile [optimized] target(s) in 5m 17s
```
Exit 0 — the exact command `Dockerfile:33` now runs (and, byte-for-byte except for the release
profile flag placement, what `Dockerfile.chef:74` runs too — both share the identical
`--bin paladin --features cli` clause verified separately at dev profile below).

```
$ cargo build --offline --lib --no-default-features
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 41s
```

```
$ cargo tree --offline --no-default-features | grep -E 'structopt|colored|comfy-table'
│   ├── colored v3.1.1
```
One match, explained above (mockito dev-dependency, not a criterion-4 violation).

```
$ cargo tree --offline --no-default-features -e normal | grep -E 'structopt|colored|comfy-table'
(no output)
```

```
$ cargo build --offline --bin paladin --features cli
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 20.13s
```

```
$ grep -c structopt Cargo.toml
0
$ grep -rln structopt src/ crates/
(no output)
```

```
$ grep -c 'cargo build --release --workspace --bin paladin --features cli' Dockerfile
1
$ grep -c 'cargo build --release --workspace --bin paladin --features cli' Dockerfile.chef
1
$ grep -rnE '\-\-bin paladin\b' Dockerfile Dockerfile.chef | grep -v 'features cli'
(no output)
```

```
$ grep -c 'builds without cli feature' .github/workflows/feature-flags.yml
0
$ grep -c 'cargo check --lib --no-default-features' .github/workflows/feature-flags.yml
1
$ python3 -c "import yaml; yaml.safe_load(open('.github/workflows/feature-flags.yml')); print('YAML OK')"
YAML OK
```
The repurposed step's shell body was run directly against the gated tree (as `gate-check.sh`,
identical logic to what the YAML step now contains): `cargo build --bin paladin` failed as
expected, so the wrapping script printed "OK: paladin binary correctly failed to build..." and
exited 0 — confirming the step passes on the current (correctly gated) tree and would fail the
job if the gate ever regressed.

```
$ grep -c 'cargo build --release --workspace --bin paladin --features cli' Dockerfile Dockerfile.chef  (via grep -o + diff)
cargo build --release --workspace --bin paladin --features cli   [Dockerfile]
cargo build --release --workspace --bin paladin --features cli   [docs/src/deployment/docker.md]
```
Byte-identical, confirmed both ways.

```
$ git status --short docs/book/ docker/ k8s/ Makefile docker-compose*.yml
(no output)
```
No generated or out-of-scope file was touched.

```
$ grep -c 'ADR-0023' CHANGELOG.md
2
$ grep -c -- '--features cli' CHANGELOG.md
4
$ grep -c 'paladin-herald' CHANGELOG.md
5
$ head -8 CHANGELOG.md
(unchanged Keep-a-Changelog preamble)
```

```
$ grep -c 'PENDING' .planning/decisions/0023-cli-dependency-isolation.md
0
$ grep -c '^## ' .planning/decisions/0023-cli-dependency-isolation.md
7
$ head -1 .planning/decisions/0023-cli-dependency-isolation.md
# ADR-0023: CLI dependency isolation and the binary/Herald surface
$ git diff --numstat .planning/decisions/0023-cli-dependency-isolation.md
45      1       .planning/decisions/0023-cli-dependency-isolation.md
```
Single-slot replacement (one line removed, 45 added — the evidence block), ADR shape intact.

**Workspace gate (CLAUDE.md), all exit 0:**
```
$ cargo test --offline --workspace
3013 passed; 0 failed (summed across all per-crate/target test result lines)
$ cargo fmt --check
(no output)
$ cargo clippy --workspace -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 54.42s
$ cargo clippy --workspace --no-default-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s (cached from earlier no-default-features build)
```
The 3013-passed/0-failed count matches the number carried forward from plan 08-07's SUMMARY — no
test count regression introduced by this plan's downstream-only edits (no `.rs`, `Cargo.toml`, or
test file was touched).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0023 is now fully closed: both Site 1/Site 2 code changes (08-07) and the downstream
  build-surface sweep, CHANGELOG entries, and criterion-4 proof (this plan) are landed.
- **Plan 08-09 (close-out) inputs from this plan:**
  1. Add ADR-0023 to `PROMOTION.md`'s numbering index and advance the next-free-ADR-number line to
     0024 (per ADR-0023's own `## Downstream Consumers` section).
  2. Add the corresponding row to `PROJECT.md`'s Key Decisions table.
  3. Flip the DEBT-04 checkbox in `REQUIREMENTS.md`.
  4. Amend the Milestone 4-6 ledger row Phase 7 recorded as `superseded by shipped code` on the
     Herald half (D-23) — this ADR's `## Context` section already states the reasoning to cite.
  5. Carry forward the **30-test** `cli`-gating delta (3 root-crate + 27 `paladin-herald`
     `table_herald` module tests) from plan 08-07's SUMMARY when 08-09 re-checks ADR-0006's 84%
     workspace line-coverage floor — all 30 tests still run and pass under `--features cli` /
     `--features table,color`; none are lost, only excluded from a default-features run.
- The five caching/documentation observations recorded in this SUMMARY's Decisions Made and
  Downstream Sweep sections (`Dockerfile.chef:55`'s cook-stage feature mismatch,
  `docs/src/deployment/docker.md:156`'s CMD line drift, and the five pre-existing out-of-scope
  Docker/Makefile/k8s defects) are **not** new findings — they carry forward RESEARCH.md's own
  triage and 08-07's "Pre-existing unrelated defects" list. None were fixed; fixing any is new
  scope beyond DEBT-04.
- No blockers for plan 08-09 — this plan's own workspace state (`cargo build`/`test`/`fmt`/`clippy`,
  default and `--no-default-features`, all green) is a clean landing point.

## Self-Check: PASSED

- All 6 modified files confirmed present via `git status --short` showing a clean tree after all
  three task commits (no uncommitted or untracked changes remain).
- All three task commit hashes (`24ccbae`, `b0629de`, `df6935a`) confirmed present via
  `git log --oneline -5` above.
- This SUMMARY.md confirmed present on disk before the metadata commit.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*
