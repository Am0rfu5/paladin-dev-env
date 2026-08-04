# Phase 4: Release Coherence - Pattern Map

**Mapped:** 2026-08-02
**Files analyzed:** ~20 edit targets (0 wholly new Rust source files; 2 new ADR docs, 1 new
measurement-record doc, N config/manifest/doc edits)
**Analogs found:** 6 / 6 pattern categories

> **Scope note.** This phase edits manifests, CI YAML, dependency-policy TOML, markdown docs and
> `.planning/` records — it creates no new Rust modules. There is no controller/service/component
> classification to do. The useful "analogs" are existing **configuration and document** shapes:
> CI job blocks to copy verbatim, the ADR house template, the measurement-provenance template, and
> the ROADMAP amendment-prose convention. RESEARCH.md (Parts A and B) already carries verbatim
> excerpts with line numbers for the CI and manifest work; this file does not re-derive those, it
> indexes and supplements them with the document-side analogs RESEARCH.md did not need to cover.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.github/workflows/ci.yml` (examples job, new) | CI job / config | batch (build matrix) | `.github/workflows/feature-flags.yml:19-118` (feature-matrix job) | exact |
| `.github/workflows/ci.yml` (docker job, budget assertions added) | CI job / config | batch (build + assert) | `.github/workflows/release.yml:160-220` (multi-arch buildx + size check) | exact |
| `.github/workflows/ci.yml` (new kind smoke job) | CI job / config | event-driven (cluster lifecycle) | `.github/workflows/integration-tests.yml:171-264` (kind/kubectl smoke) | exact |
| `.github/workflows/ci.yml` (push trigger) | config | — | `.github/workflows/ci.yml:3-12` itself (commented-out stanza) | exact (in-file) |
| `.planning/decisions/0008-<version-slug>.md` (new ADR, D-02) | document / record | CRUD (append-only record) | `.planning/decisions/0006-coverage-gate.md` + `.planning/decisions/PROMOTION.md` | exact |
| `.planning/decisions/0009-<edition-slug>.md` (new ADR, D-06) | document / record | CRUD (append-only record) | `.planning/decisions/0007-battalion-cancellation-deferral.md` | exact |
| `.planning/phases/04-release-coherence/04-release-measurement.md` (new, D-17) | document / record | batch (measurement snapshot) | `.planning/phases/03-verification-depth/03-coverage-measurement.md` and `01-coverage-measurement.md` | exact |
| `Cargo.toml` + 11 `crates/*/Cargo.toml` (version, edition) | config manifest | CRUD (field rewrite) | `release.toml` + `cargo-release` dry-run output (RESEARCH Part A, Q2) — tool-driven, no hand-analog needed | role-match (tool, not file) |
| `CHANGELOG.md` (finalize heading) | document | CRUD (append/rewrite) | `Makefile:477-479` (`perl -0pi` heading-insert one-liner) | exact |
| `deny.toml` / `.cargo/audit.toml` (remove stale entry, add migration notes) | config | CRUD (list edit) | in-file precedent — the 14 other `ignore` entries with rationale + note (RESEARCH Part A, Q4 table) | exact (in-file) |
| `docs/src/getting-started/quickstart.md` | document | request-response (tutorial steps) | `docs/src/appendix/performance-baseline.md` (Phase 3's dated-section-add-without-overwrite model) | role-match |
| `.planning/ROADMAP.md`, `REQUIREMENTS.md`, `PROJECT.md`, `codebase/CONCERNS.md` (amendments) | document / record | CRUD (in-place amend) | `.planning/ROADMAP.md:274` (Phase 3's own amendment of criterion 2) | exact (in-file) |
| `.planning/ledgers/milestone-01.md` (REL-01..05 rows) | document / record | CRUD (append rows) | `.planning/ledgers/milestone-01.md:88-109` (Phase 3's QUAL-02/04/05 rows, same file) | exact (in-file) |

## Pattern Assignments

### `.github/workflows/ci.yml` — examples-build job (new)

**Analog:** `.github/workflows/feature-flags.yml:19-118` (`feature-matrix` job)

Copy its caching + toolchain scaffold (three `actions/cache@v4` steps for registry/index/build,
`dtolnay/rust-toolchain@master`), but use `ci.yml`'s own `actions/checkout@v5` (one version ahead
of `feature-flags.yml`'s `@v4` — follow the file the new job lands in, not the file it's modeled
on).

**Core pattern — the 4-invocation feature matrix** (from RESEARCH Part A/B, verified this session):
```bash
cargo build --examples --offline                                                    # 43 default-feature examples
cargo build --example vision_analysis --example vision_battalion \
  --features "vision,llm-openai" --offline                                          # 2
cargo build --example document_processing --features "content-processing" --offline # 1
cargo build --example http_service_host --features "web-server" --offline           # 1
```
**Do not** write a bare `cargo build --examples --offline` step — it silently skips the 4
required-features examples with exit code 0 (confirmed live: cargo's bulk selector omits unmet
`required-features` targets without warning). This is the one behavior an executor cannot infer
without the research's live verification — state it as a code comment in the new job.

---

### `.github/workflows/ci.yml` — docker job budget assertions

**Analog:** `.github/workflows/release.yml:160-220` (multi-arch buildx + size check)

**Size-check excerpt to adapt** (`release.yml:196-220`):
```yaml
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

      - name: Verify image size
        run: |
          IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
          docker pull "$IMAGE"
          SIZE=$(docker image inspect "$IMAGE" --format='{{.Size}}')
          SIZE_MB=$((SIZE / 1024 / 1024))
          echo "Image size: ${SIZE_MB} MB"
          if [ $SIZE_MB -gt 500 ]; then
            echo "::warning::Image size (${SIZE_MB} MB) exceeds 500 MB target"
          fi
```
**Two required adaptations** for `ci.yml`'s job (which builds locally, `push: false`, at `:409-434`):
1. No registry pull — inspect the local tag directly: `docker image inspect paladin:test --format='{{.Size}}'` (reuse the tag `ci.yml:426` already assigns).
2. `release.yml`'s check only warns; SC5 wants the CI gate to **fail** on overage. Change
   `echo "::warning..."` → `echo "::error::..."; exit 1` for the new `ci.yml` step. Leave
   `release.yml`'s own non-blocking warning untouched (out of scope).

**Action versions to use** (already running in this repo, cited by file:line, not invented):
`docker/setup-buildx-action@v4`, `docker/build-push-action@v6` (`ci.yml`'s own current pins —
stay on these rather than downgrading to `release.yml`'s older `@v3`/`@v5`), plus
`docker/setup-qemu-action@v3` (the one action `ci.yml` lacks; `release.yml` is the only in-repo
precedent for its version).

**Time budget — no precedent exists.** State this explicitly in the plan rather than inventing a
false analog. Model the epoch-diff shape on the kind job's startup-time check below (`date +%s`
before/after, diff, warn-or-fail at threshold).

---

### `.github/workflows/ci.yml` — Kubernetes kind smoke job (new)

**Analog:** `.github/workflows/integration-tests.yml:171-264` (`kubernetes-smoke-test`, complete
and working — reuse, do not reinvent)

**Startup-budget check to copy verbatim** (`integration-tests.yml:238-249`):
```bash
START_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.startTime}')
READY_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.conditions[?(@.type=="Ready")].lastTransitionTime}')
STARTUP_TIME=$((READY_EPOCH - START_EPOCH))
if [ $STARTUP_TIME -gt 30 ]; then
  echo "::warning::Startup time (${STARTUP_TIME}s) exceeds 30 second target"
fi
```
**Known caveat to record, not fix:** `k8s/deployment.yaml:66-68` runs a placeholder
`sleep 3600` with all readiness probes commented out (`:137-174`) — the < 30s figure measures
container scheduling, not real app readiness. Recommended disposition (research's call, restate in
plan): accept the placeholder-based reuse as satisfying D-14.3's literal ask (proves kind/kubectl
orchestration), and record probe-wiring as a named deferral — do not wire `paladin-web`'s health
endpoints into the probes this phase (new capability, out of boundary).

`docker`, `kind`, `kubectl` are all absent here — this job and the docker-budget job can only be
**authored and statically validated** (YAML parses, `k8s/*.yaml` references resolve, action refs
exist), never executed, per D-15. State that distinction explicitly in the plan's verification
section for both new jobs — do not report SC5 as met on configuration alone.

---

### `.github/workflows/ci.yml` — push trigger

**Analog:** the file's own commented-out stanza, `ci.yml:3-12`:
```yaml
on:
  # push:
  #   branches: [ main, develop, 'feature/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
**Replacement** (adds `release/**`, keeps the PR-only-avoid-double-run convention):
```yaml
on:
  push:
    branches: [ main, develop, 'feature/**', 'release/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
Two sibling files (`integration-tests.yml:3-16`, `feature-flags.yml:3-9`) carry the identical
commented convention; whether to extend them too is out of D-14's named scope — leave as an open
question in the plan, do not silently touch them.

---

### `.planning/decisions/0008-*.md` and `0009-*.md` — the two new ADRs (D-02 version, D-06 edition)

**Analog:** `.planning/decisions/PROMOTION.md` (house rules) + `0006-coverage-gate.md` (worked
example of a contested-number ADR) + `0007-battalion-cancellation-deferral.md` (worked example of
a `## Downstream Consumers` section naming a ledger row to amend).

**Required heading set, in this order** (from `PROMOTION.md`):
```
## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```
`## Code Locations` and `## Considered Options` **must be bulleted lists, never prose** — the
project's `adr-parser.cjs` only extracts structured entries from bullet/numbered lines.

**`## Code Conformance` field convention** — exact excerpt to mirror (`0006-coverage-gate.md:211-220`):
```
## Code Conformance

must change

No coverage gate currently exists in `.github/workflows/ci.yml` — ... This is recorded as pending
work, not smoothed into conformance: **PIPE-02** in Phase 15 is the requirement that wires this
ADR's 84% floor into CI either way.
```
Both new ADRs are `must change` (per CONTEXT D-02/D-06) — name the executing requirement (REL-01
for the version ADR, REL-02 for the edition ADR) exactly as this excerpt names PIPE-02.

**`## Downstream Consumers` convention** — exact excerpt to mirror (`0007-...md:98-104`):
```
## Downstream Consumers

- `.planning/ledgers/milestone-01.md` § Epic 4, the `REQ-battalion-cancellation` row — amended by
  plan 02-09 to cite this ADR (`0007`) as the deferring authority for the ...
- **The v2 backlog** — the named forward owner, ...
```
For the two new ADRs, the required Downstream Consumers entries are: Phase 7's ARCH-04 (version
ADR) / ARCH-03(a) (edition ADR), whose requirement text must be updated to cite the ADR instead of
re-adjudicating (per CONTEXT's Integration Points).

**Numbering:** next free number is `0008` per `PROMOTION.md`'s index line — the version ADR takes
`0008`, the edition ADR `0009`; **update `PROMOTION.md`'s index table and "Next free ADR number"
line** in the same edit (both `0006-coverage-gate.md` and `0007-...md` show this file being kept
current — treat `PROMOTION.md` itself as a file this phase modifies, not just reads).

---

### `.planning/phases/04-release-coherence/04-release-measurement.md` — new measurement record (D-17)

**Analog:** `.planning/phases/03-verification-depth/03-coverage-measurement.md` (and its own
Phase-1 ancestor `01-coverage-measurement.md`)

**Provenance-block shape to copy verbatim** (`03-coverage-measurement.md:1-60`):
```markdown
# <Title> — Raw Evidence Record (Phase 4)

This file is raw evidence only: verbatim commands, verbatim tool output, toolchain versions,
commit SHA, and UTC dates. ...

## Entry measurement — <what>

### Environment probes (verbatim)

Command: `rustc -vV`
```
<raw output>
```

Command: `cargo --version`
```
<raw output>
```

Command: `git rev-parse HEAD`
```
<raw output>
```

Command: `git rev-parse --abbrev-ref HEAD`
```
<raw output>
```

Command: `git status --porcelain`
```
<raw output, plus one prose sentence disambiguating unrelated pre-existing dirty files from the
measurement's own scope — see the exact disambiguation sentence at 03-coverage-measurement.md:44-47>
```

Command: `date -u`
```
<raw output>
```
```
This single file should carry **every** D-17 figure this phase produces — `cargo fmt --check`,
`cargo clippy`, `cargo test --workspace`, `cargo build --workspace [--no-default-features]`,
`cargo audit`, `cargo deny check`, and the QUICKSTART timing — each as its own `## Entry
measurement — <name>` section following the same probe block, per D-17's "every figure carries the
Phase 1/Phase 3 provenance block" instruction. For `cargo audit` specifically, add the advisory-DB
snapshot fields the exception in D-17 calls for: advisory count (1186, per this session's live run)
and fetch date, since audit is the one command that must reach `github.com` rather than run
`--offline`.

---

### `CHANGELOG.md` — finalize `[Unreleased]` → `[0.7.0]`, date `[0.6.0]`

**Analog:** `Makefile:477-479`, the exact transformation already encoded (safe to reproduce by
hand or by re-running this line standalone, without invoking `make release`):
```make
@DATE=$$(date +%Y-%m-%d); \
    perl -0pi -e "s/## \\[Unreleased\\]/## [Unreleased]\n\n## [$(VERSION)] - $$DATE/" CHANGELOG.md
```
This finds the first literal `## [Unreleased]` line and inserts a new dated heading immediately
below it; all body content that followed the old heading becomes the new section's content
automatically (nothing moved by hand). **Second, separate edit required:** `CHANGELOG.md:63`'s
`## [0.6.0]` heading has no trailing ` - <date>` — every other heading in the file does (date
format precedent: `## [0.5.1] - 2026-06-04`, `YYYY-MM-DD` throughout). No tooling exists for this
half; it is a one-line hand edit, and the source date is not derivable from the tree with certainty
(`v0.6.0` was never tagged) — RESEARCH's Open Gap 1 recommends `git log -S'## [0.6.0]'
CHANGELOG.md` to find the introducing commit, or a `checkpoint:human-verify` if precision matters.

---

### `.planning/ROADMAP.md` / `REQUIREMENTS.md` / `PROJECT.md` / `codebase/CONCERNS.md` — amendments (D-05, D-13)

**Analog — the amendment-prose convention every amendment in this phase must match**
(`.planning/ROADMAP.md:274`, Phase 3's own amendment of its criterion 2):
```
2. No first-party source file reports 0% coverage — ... (**Amended by Phase 3, dated 2026-08-02,
   citing `03-coverage-measurement.md`**: the named file list was re-derived from a measurement of
   the shipped tree, not restated from the ingested pre-workspace claim it originally came from.
   ...)
```
**Structure to copy exactly:** `(**Amended by Phase <N>, dated <date>, citing `<evidence file>`**:
<one prose paragraph — what was wrong, what the corrected figure/claim is, and why the correction
wins under the precedence order>.)` — parenthetical, inline, appended to the original sentence
rather than replacing it. Apply this verbatim shape to:
- D-05's `CONCERNS.md:7-25` edition-finding correction (cite the ADR + `rustc -vV`/toolchain proof
  as the evidence file instead of a coverage-measurement file).
- D-13's five "22 examples" restatements (`ROADMAP.md:6`, `ROADMAP.md:313`, `PROJECT.md:21`,
  `PROJECT.md:136`, `REQUIREMENTS.md:382`) — cite the example-count re-derivation (47 files, 4
  gated) as the evidence.

This is Phase 3's own "amend at source with dated provenance" convention (also used by Phase 1 and
Phase 2, per CONTEXT's Established Patterns) — never silently substitute a corrected figure without
the parenthetical.

---

### `.planning/ledgers/milestone-01.md` — REL-01..REL-05 rows and hand-off rows

**Analog:** the file's own existing rows for QUAL-02/QUAL-04/QUAL-05 (`milestone-01.md:88-109`) —
extract the row format directly rather than from a different file, since the target file already
carries the convention.

**Row shape** (`| ID | Verdict | Evidence |` table, one row per requirement-facet):
```
| QUAL-02 — `src/bin/paladin-server.rs` | deferred with reason | 0.00% at both entry and exit
(145/145 missed lines, `03-coverage-measurement.md`). Closing it needs a testable `run()` seam
extracted from `#[tokio::main] async fn main()` — a refactor outside a measurement phase's charter,
per plan 03-06's recorded decision. **Owner: Phase 5 / VERIFY-05.** |
```
```
| QUAL-04 — MCP failure mode: expired/rejected bearer token | satisfied |
`streamable_http_rejects_expired_bearer_token`
(`tests/integration/mcp_streamable_http_test.rs:428`); re-run 2026-08-02 passing. |
```
**Verdict legend** — read the file's own §"Verdict Meaning" table near line 113 before writing new
rows; it is the single source for the five verdict classes (`satisfied`, `deferred with reason`,
etc.) that every row in this phase (REL-01..REL-05, plus D-09's advisory hand-off rows to SEC-01/
SUPPLY-02, D-15's Docker/K8s deferral, D-11's clean-machine deferral) must use. Every `deferred with
reason` row needs a named owner in bold, exactly as `**Owner: Phase 5 / VERIFY-05.**` demonstrates —
this satisfies Phase 1's D-20 evidence bar (`file:line` plus a named passing exerciser, or an
explicit deferral with owner) that CONTEXT's canonical refs require.

## Shared Patterns

### Measurement provenance (D-17) — applies to every figure this phase records
**Source:** `.planning/phases/03-verification-depth/03-coverage-measurement.md:1-60`,
`.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`
**Apply to:** the new `04-release-measurement.md`, and any inline measurement quoted in an ADR or
ledger row. `rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `git rev-parse --abbrev-ref HEAD`,
`git status --porcelain` (with a disambiguating sentence for unrelated pre-existing dirty files),
`date -u` — every command's raw stdout pasted verbatim, no paraphrase. `--offline` on every cargo
invocation except `cargo audit` (needs `github.com/RustSec/advisory-db`; record its DB snapshot
count/date instead).

### ADR house structure
**Source:** `.planning/decisions/PROMOTION.md` (rules) + `0006-coverage-gate.md` /
`0007-battalion-cancellation-deferral.md` (worked examples)
**Apply to:** the two new ADRs (`0008` version, `0009` edition). Seven required headings in fixed
order; `Code Conformance` always states `conforms`/`must change` plus the executing requirement;
`Downstream Consumers` names the specific file/section that inherits the answer next; supersession
never deletes a file, only flips `## Status` to `Superseded` with a pointer.

### Amendment-at-source prose
**Source:** `.planning/ROADMAP.md:274`
**Apply to:** every D-05/D-13 correction across `ROADMAP.md`, `REQUIREMENTS.md`, `PROJECT.md`,
`codebase/CONCERNS.md`. Parenthetical, inline, `(**Amended by Phase 4, dated 2026-08-02, citing
`<evidence file>`**: ...)` — appended after the original claim, never a silent replacement.

### Ledger row / verdict legend
**Source:** `.planning/ledgers/milestone-01.md:88-113`
**Apply to:** every REL-01..REL-05 row and every hand-off row (D-09's advisories to SEC-01/
SUPPLY-02, D-15's Docker/K8s deferral, D-11's clean-machine deferral, D-08's duplicate-audit-job
hand-off to SUPPLY-01). `| ID | Verdict | Evidence |`, five verdict classes, named bold owner on
every `deferred with reason` row.

## No Analog Found

| File | Role | Data Flow | Reason |
|---|---|---|---|
| Docker-build time-budget assertion (`ci.yml` docker job) | CI step | batch | No time-budget precedent exists anywhere in this repo (RESEARCH Part B, Open Gap 1) — author from scratch, modeled structurally on the kind job's epoch-diff pattern, not copied from an existing Docker-timing block. Also unresolved: whether the 5-min figure is per-arch or whole multi-arch build — flag as an open question for the plan, do not silently pick one. |

## Metadata

**Analog search scope:** `.github/workflows/*.yml`, `.planning/decisions/*.md`,
`.planning/phases/0{1,3}-*/*.md`, `.planning/ledgers/milestone-01.md`, `Makefile`, `release.toml`,
`Cargo.toml` + `crates/*/Cargo.toml`, `docs/src/getting-started/quickstart.md`,
`docs/src/appendix/performance-baseline.md`.
**Files scanned:** ~15 direct reads/greps this pass, plus every excerpt RESEARCH.md Parts A and B
already extracted (not re-read here).
**Pattern extraction date:** 2026-08-02
</content>
