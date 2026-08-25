# Phase 18: Rust SAST — Evaluate and Adopt CodeQL - Research

**Researched:** 2026-08-25
**Domain:** GitHub Advanced Security / CodeQL Rust static analysis, GitHub Actions workflow
governance, GitHub ruleset administration
**Confidence:** MEDIUM — the CodeQL platform mechanics are well documented and directly verified
against official GitHub docs; the one question that matters most for this phase's honesty test
(D-12's cargo-feature coverage) has **no documented answer anywhere in official GitHub sources**,
and is flagged as an open question requiring empirical measurement, not assumed.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Use CodeQL **advanced setup** (a committed workflow file), not default setup — Clause 3
  of `scripts/check-workflow-triggers.sh` requires every pinned required-check context to resolve
  to a job name declared in a workflow file; default setup declares no job anywhere in the tree.
- **D-02:** The analysis job's display name is a **single stable literal** — `CodeQL Analysis
  (Rust)` — no `strategy.matrix`, no `${{ }}` expression in the name.
- **D-03:** CodeQL lives in a **new `.github/workflows/codeql.yml`**, not a job inside `ci.yml`
  (which deliberately carries no `schedule:` key).
- **D-04:** Triggers are `pull_request` (branches `[main, 'release/**']`, **no path filter**),
  `push` (branches **`['**']`**), `schedule`, and `workflow_dispatch`. `push: ['**']` is required
  by Clause 2 of the trigger guard (only `docs.yml`/`release.yml` may narrow it), and is a strict
  superset of "push on main," so it satisfies SAST-02 by over-coverage rather than exemption. Cost
  absorbed by a `concurrency` group with `cancel-in-progress` on non-`main` refs.
- **D-05:** The workflow file and its row in `branching-model.md`'s trigger-policy table land in
  the **same commit**. The row's "Triggers" cell names all four trigger types and "Push branch
  filter" cell reads `['**']`, matching the YAML literally (Clause 2 compares them literally).
- **D-06:** The advisory phase uses an explicit, **visible** non-blocking posture — no
  `continue-on-error`. Non-blocking is achieved by the context simply **not being in the ruleset
  yet**. Unlike the existing `osv-scanner` job, which stacks `continue-on-error` on both the scan
  and the SARIF upload.
- **D-07:** The probe fixture is a **Rust crate in-tree, excluded from the workspace** — its own
  `Cargo.toml`, not a `members` entry — so `cargo build`/`clippy`/`llvm-cov` at the workspace root
  never see it and workspace coverage is unaffected. Evidence must be **reproducible**.
- **D-08:** The fixture carries the **same four vulnerability classes, in the same order**, that
  disqualified Snyk — hardcoded credential, command injection via `sh -c`, path traversal, SQL
  injection — for direct comparison against the recorded Snyk baseline (0 in Rust vs 3 in
  identical JavaScript).
- **D-09:** The probe is scanned on a **dedicated evaluation branch / `workflow_dispatch` run**,
  and excluded from the steady-state PR scan path thereafter — a permanently scanned fixture would
  emit four standing alerts forever.
- **D-10:** The planted credential must not trip the repo's secret-scanning/pre-commit hooks. Use
  an obviously-synthetic, non-resolving value and confirm `pre-commit run --all-files` stays green
  with the fixture present. If it cannot be made to pass, that is a finding to record, not
  something to bypass with `--no-verify`.
- **D-11:** **A zero-finding result ends the phase in the disqualified branch and that is a
  success, not a failure.** No adoption work proceeds past a failed probe.
- **D-12:** Scan coverage is itself measured and recorded, not assumed. The plan must determine
  CodeQL's Rust build mode and, if any build is involved, configure it so feature-gated code
  (`vision`, `content-processing`, `web-server`, `llm-*`, `redis-queue`, `s3-storage`,
  `storage-mysql`, `qdrant`, `cli`, `notifications`) is reached.
- **D-13:** **The number of `.rs` files CodeQL reports as analysed is recorded as first-class
  evidence, alongside the finding count.** The denominator is **385** —
  `crates/**/*.rs` (246, includes each crate's own `tests/`/`benches`/`examples`) +
  root `src/**/*.rs` (139) — verified against the tree on 2026-08-25. An analysed count far below
  385 is a disqualifying result even if findings are non-zero.
- **D-14:** The window produces numbers **without blocking the phase on calendar time** — measure
  via dispatching the scan across a set of recent merged-PR-equivalent commits (backfill) for an
  FP rate over real diffs, plus a short live advisory period on actual PR traffic.
- **D-15:** Recorded metrics, at minimum: total alerts raised; true-positive count; false-positive
  count; the resulting FP rate; wall-clock per run (cold and warm cache); the D-13 analysed-file
  count.
- **D-16:** The evidence document is committed under **`.planning/`**; the **conclusion** (not the
  raw log) propagates into `security.instructions.md` under SAST-04.
- **D-17:** Dismissed CodeQL alerts get a **governed register modelled on `SECURITY-EXCEPTIONS.md`**
  — named owner, review date, scope, concretely-stated compensating control per dismissal. Whether
  it's a new file, a section in the existing one, or guarded by a `check-*.sh` script is a
  planning call; ungoverned dismissal is ruled out.
- **D-18:** Promotion is attempted **within this phase**, conditional on D-14/D-15 numbers
  qualifying. If they don't qualify, the phase closes with the scanner advisory and the promotion
  criteria are written down as a named open item with its trigger condition.
- **D-19:** Promotion is **one change touching all four recorded places**: (1)
  `.github/rulesets/protect-main-branch.json` context list, 44 → 45; (2) re-apply live ruleset
  `20868126`; (3) `docs/src/appendix/branch-protection.md`'s context table **and every prose
  occurrence of the count** (lines 85, 117, 180 as of this research); (4)
  `scripts/check-workflow-triggers.sh` passes (Clause 3 resolves the new context to D-02's job
  name).
- **D-20:** Semgrep is a **contingency, not parallel work** — only evaluated against the identical
  fixture if CodeQL fails the D-11 probe.
- **D-21:** The SAST-04 rewrite's blast radius: `security.instructions.md`'s "Known gap" section
  and its "No tool above performs taint analysis" claim; `CLAUDE.md`'s Security bullet and
  `.github/copilot-instructions.md` if either states the gap; the Snyk section stays (standing
  prohibition, not stale text); `.planning/STATE.md`'s deferred-item row and `.planning/MILESTONES.md`
  updated at phase close.
- **D-22:** The section is **narrowed by evidence, never deleted**. Must state what the adopted
  tool does *not* cover and what the manual credential-handling review still owns (redact-before-
  truncate, no key interpolation in logs, no redirect-following on credentialed clients) — those
  three checks remain owned by humans unless the probe proved otherwise for a specific one.

### Claude's Discretion

Build mode and query suite selection (`security-extended` vs default), exact schedule cadence,
cache strategy, the probe crate's file layout, whether the triage register is a new file or a
section, and the size of the D-14 backfill sample.

### Deferred Ideas (OUT OF SCOPE)

- Remediating real findings CodeQL surfaces (triaged and recorded here, fixed in follow-up work).
- Adding a GitHub-facing `SECURITY.md` (separate deliverable, `SECURITY-EXCEPTIONS.md` already
  records this).
- Semgrep as a standing complement alongside a qualified CodeQL (D-20 scopes it to contingency
  only).
- Refreshing `.planning/codebase/INTEGRATIONS.md`'s stale "CI Pipeline: Not integrated" claim.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SAST-01 | Probe a candidate SAST against a deliberate-vulnerability fixture before adoption; a zero-finding result is a valid, publishable disqualification. | See "The Probe Fixture" and "CodeQL Rust Extraction Mechanics" below — confirms build-mode `none` is the only mode, so the probe validates whether buildless extraction finds real Rust defects, matching what disqualified Snyk. |
| SAST-02 | If qualified, the scanner runs on every PR with no path filter, plus push-on-main and a schedule. | See "Workflow Wiring" — D-04's trigger shape is locked; research confirms the `push: ['**']`/Clause 2 mechanics and that `pull_request` carries no `paths:` key anywhere in the plan. |
| SAST-03 | Non-blocking first; promotion on measured FP rate and wall-clock cost; promotion updates all four recorded places in one change. | See "Measuring What Was Analysed," "False-Positive-Rate Methodology," and "Promotion Mechanics" — the `PUT /rulesets/{id}` endpoint, the `src.zip` debug-artifact mechanism, and the push-based backfill technique are the concrete deliverables this requirement needs. |
| SAST-04 | Rewrite `security.instructions.md`'s "Known gap" section to match the measured outcome, narrowed not deleted. | See "SAST-04 Rewrite Mechanics" — D-21/D-22 scope is confirmed against the current file text read directly. |
</phase_requirements>

## Summary

CodeQL's Rust support reached general availability on 2025-10-14 [CITED: github.blog/changelog].
It runs in exactly one build mode — `none` — meaning CodeQL never invokes a full `cargo build`;
instead it uses `rust-analyzer` internally to expand macros and execute `build.rs` scripts
[CITED: docs.github.com]. This is the single most consequential fact for planning this phase: it
means the probe (SAST-01) is testing whether *buildless* extraction finds real defects, and it
means D-12's feature-coverage question — does CodeQL see code behind `vision`, `web-server`,
`cli`, etc. — has **no documented answer**. No official GitHub page (build-options reference,
extractor-options reference, `codeql-config.yml` reference, or the `codeql-action/init` action's
input list) mentions cargo feature selection at all. This is not a case of "the docs are just
thin on an edge case" — every plausible doc location was checked directly and none of them
mentions cargo features, `--all-features`, or workspace member selection for Rust. The plan must
treat this as an open question requiring empirical testing against this tree, not an assumption
either way.

Two other findings are load-bearing for the plan. First, `codeql-action/init`'s `debug: true`
input produces a debug artifact containing `src.zip` — an archived copy of the exact source files
CodeQL scanned — which is the concrete, reproducible mechanism for D-13's "number of `.rs` files
analysed" evidence; `codeql database print-baseline` gives a lines-of-code baseline per language
but is not documented to report a file count, so `src.zip` is the primary mechanism and
`print-baseline` a secondary corroborating number. Second, `workflow_dispatch` (and `gh workflow
run --ref`) **only accepts a branch or tag ref, never a bare commit SHA** [CITED: GitHub Actions
API constraint, cross-checked against multiple `cli/cli` and `github/docs` issue reports] — this
rules out the naive approach to D-14's historical-commit backfill (dispatching a scan directly
against old merge-commit SHAs) and points instead at reusing D-04's own `push: ['**']` trigger:
push a disposable branch pointing at each sampled historical commit, let `codeql.yml` fire
naturally, then delete the branch.

**Primary recommendation:** Build `codeql.yml` with `build-mode: none`, `security-extended`
queries, and explicit `--all-features`-equivalent feature enablement attempted via toolchain setup
before the CodeQL init step (Rust needs `rustup`+`cargo` on the runner regardless); treat D-12's
actual feature coverage as unverified until the probe (with a feature-gated 5th planted defect, or
an explicit coverage check against the `src.zip` artifact) proves it empirically. Use the
`src.zip` debug artifact as the D-13 file-count mechanism, `PUT /repos/{owner}/{repo}/rulesets/20868126`
for D-19's ruleset re-application, and disposable push-branches (not `workflow_dispatch`) for
D-14's historical backfill.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Static taint analysis of first-party Rust | CI/CD (GitHub Actions) | GitHub platform (Code Scanning UI, SARIF store) | CodeQL runs as a CI job; results are ingested by GitHub's platform-level code-scanning alert store, not the application itself. Neither the Rust workspace's runtime tiers (there are none relevant here — this is tooling, not app architecture) nor the database/storage tier is involved. |
| Deliberate-vulnerability probe fixture | CI/CD (excluded Cargo crate) | — | The fixture is source code that exists only to be scanned; it has no runtime tier of its own and is deliberately kept out of the build graph (D-07). |
| Required-check governance (ruleset, trigger-policy table, guard scripts) | CI/CD / repo administration | Documentation (`docs/src/appendix/`) | GitHub rulesets are a platform-level (not workflow-level) control; the guard scripts and docs mirror that state so it stays internally consistent — this is a governance capability, not an application capability. |
| Alert-triage / dismissal register | Documentation / governance (`.planning/` or repo root) | CI/CD (an optional enforcing guard script) | Modelled directly on `SECURITY-EXCEPTIONS.md`, which is a governance artifact enforced (optionally) by a CI guard, not application code. |

## Standard Stack

### Core

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|---------------|
| `github/codeql-action/init` + `/analyze` | `v3` (matches this repo's existing `upload-sarif@v3` wiring) or `v4` (current parallel major) | Creates the CodeQL database and runs the query suite | GitHub's own first-party action; `v3` and `v4` are both actively released in lockstep (e.g. `v3.37.8`/`v4.37.8` released the same day) [CITED: github.com/github/codeql-action releases]. This repo already trusts `v3` for `upload-sarif` in `ci.yml`'s `osv-scanner` job — staying on `v3` for the new workflow avoids introducing a second major-version convention in the same file family. |
| CodeQL Rust query pack | Bundled with the CodeQL CLI version the action resolves (no separate install) | Supplies the `rust/*` query set | Query packs version with the CodeQL bundle, not independently — no separate dependency to pin. |

### Supporting

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| `dtolnay/rust-toolchain@stable` | pinned action, already used 9× in `ci.yml` | Ensures `rustup`+`cargo` are present before the CodeQL init step (Rust analysis "requires `rustup` and `cargo` to be installed" [CITED: docs.github.com]) | Every job in `codeql.yml` that runs the init/analyze steps — matches this repo's existing convention rather than relying on the runner image's pre-baked Rust toolchain. |
| `Swatinem/rust-cache@v2` | pinned action, already used 9× in `ci.yml` | Cargo build/registry caching | Alongside `codeql-action/init`'s own `trap-caching`/`dependency-caching` inputs, to keep warm-cache wall-clock down for the D-15 metric. |
| `rusqlite` (crates.io) | `0.40.2` current [VERIFIED: crates.io — `cargo search rusqlite` confirms; package-legitimacy check verdict OK, published 2014-11-21, ~2.39M weekly downloads, repo `github.com/rusqlite/rusqlite`] | SQL-injection-class defect in the probe fixture (D-08) | If the probe's SQL-injection example needs a real query surface. `sqlx` (already a workspace dependency, `0.8.6`) is an equally valid choice and avoids adding a new dependency name to the tree — **this choice is Claude's Discretion per CONTEXT.md**; either is legitimate. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| CodeQL | Semgrep | D-20: contingency only, evaluated against the identical fixture only if CodeQL fails D-11. Semgrep is pattern matching, not interprocedural taint analysis; its Rust ruleset exists (`registry.semgrep.dev/ruleset/rust`) and recently gained reachability analysis for Rust dependencies [ASSUMED — WebSearch summary, not independently fetched from the registry page], but coverage depth relative to CodeQL's Rust queries is unverified and out of this research's scope per the discussion's explicit boundary. |
| `codeql-action@v3` | `codeql-action@v4` | v4 is the "latest" per the action's own README messaging, but v3 is still actively maintained in lockstep and matches this repo's existing `upload-sarif@v3` convention. Switching to v4 is low-cost later; starting on v3 avoids a mixed-version footprint on day one. |

**Installation:** No new package manager install is required — `codeql-action` is consumed as a
GitHub Action reference (`uses:`), not a Cargo dependency. If the probe fixture's SQL-injection
example needs a crate, add it to the **probe crate's own, standalone `Cargo.toml`** only (it is
excluded from the workspace per D-07, so it does not touch the root `Cargo.lock`).

**Version verification:** `github/codeql-action` current releases confirmed via WebFetch of the
project's GitHub releases page: `v4.37.8`/`v3.37.8`, dated 2026-08-21, bundling CodeQL CLI
`2.26.3` [CITED: github.com/github/codeql-action]. `rusqlite` confirmed via
`cargo search rusqlite` → `0.40.2` [VERIFIED: crates.io registry, 2026-08-25].

## Package Legitimacy Audit

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|--------------|---------|-------------|
| `rusqlite` | crates.io | ~11 years (published 2014-11-21) | ~2.39M/week | `github.com/rusqlite/rusqlite` | **OK** | Approved, if the planner chooses it over reusing `sqlx` for the probe's SQL-injection example |
| `sqlx` | crates.io | ~7 years (published 2019-06-06), already a workspace dependency at `0.8.6` | ~2.60M/week | `github.com/launchbadge/sqlx` | **OK** | Already approved and in the tree; reusing it for the probe avoids introducing a new package name at all |

**Packages removed due to [SLOP] verdict:** none.
**Packages flagged as suspicious [SUS]:** none.

Both names above were checked with `gsd-tools query package-legitimacy check --ecosystem crates
rusqlite sqlx` (verdict `OK` for both) and cross-verified against the crates.io registry directly
via `cargo search`. Per the package-name-provenance rule, both names were originally surfaced from
training knowledge/WebSearch rather than an authoritative source, so they are tagged `[ASSUMED]`
as **candidate names** even though their registry legitimacy is `[VERIFIED]` — the planner should
treat "which crate to use for the SQL-injection example" as a discretionary choice to confirm, not
a settled fact. No `postinstall`-equivalent risk applies to either (Rust crates have no npm-style
install scripts).

## Architecture Patterns

### System Architecture Diagram

```
PR opened/pushed, or push to any branch, or weekly schedule, or manual dispatch
        │
        ▼
.github/workflows/codeql.yml  (new, advanced setup — D-01)
        │
        ├─ actions/checkout@v5
        ├─ dtolnay/rust-toolchain@stable   (rustup + cargo present — CodeQL requires both)
        ├─ Swatinem/rust-cache@v2          (warm-cache wall-clock)
        │
        ▼
github/codeql-action/init@v3
   languages: rust
   build-mode: none          ← ONLY mode Rust supports; no autobuild/manual
   queries: security-extended (or default — Claude's Discretion)
   debug: true                ← produces src.zip debug artifact (D-13 evidence mechanism)
        │
        ▼  (internally: rust-analyzer expands macros, runs build.rs — no full `cargo build`)
CodeQL database created
        │
        ▼
github/codeql-action/analyze@v3
        │
        ├─→ SARIF results ──→ github/codeql-action/upload-sarif  ──→ GitHub code-scanning alerts UI
        │                                                              (advisory: NOT in the ruleset yet — D-06)
        └─→ debug artifact (src.zip + diagnostics) ──→ unzip, count .rs files ──→ compare to 385 (D-13)

Separately, offline of the main PR path:
  Historical-commit backfill (D-14) ──→ push disposable branch at each sampled commit
                                          (NOT workflow_dispatch — SHA refs are rejected)
                                     ──→ codeql.yml fires via push:['**'] naturally
                                     ──→ record alert count / FP rate / wall-clock
                                     ──→ delete disposable branch

Promotion (SAST-03, gated on D-14/D-15 numbers qualifying):
  .github/rulesets/protect-main-branch.json (44→45)
        │
        ▼
  gh api --method PUT /repos/.../rulesets/20868126 --input <updated-file>
        │
        ▼
  docs/src/appendix/branch-protection.md (table + 3 prose occurrences of "44")
        │
        ▼
  scripts/check-workflow-triggers.sh passes (Clause 3 resolves "CodeQL Analysis (Rust)")
```

### Recommended Project Structure

```
.github/
├── workflows/
│   └── codeql.yml                    # new — D-01, D-02, D-03
├── rulesets/
│   └── protect-main-branch.json      # context list 44→45 on promotion only
fixtures/                             # NOT under crates/ — the crates/* glob would
│   └── codeql-probe/                 # auto-include anything placed there (D-07)
│       ├── Cargo.toml                # standalone, no [workspace] table entry anywhere
│       └── src/
│           ├── lib.rs                # or main.rs — Claude's Discretion
│           ├── credential.rs         # hardcoded, synthetic, non-resolving (D-10)
│           ├── command_injection.rs  # sh -c pattern, matching the Snyk probe exactly (D-08)
│           ├── path_traversal.rs
│           └── sql_injection.rs
docs/src/contributing/branching-model.md   # new trigger-policy row, same commit as codeql.yml (D-05)
docs/src/appendix/branch-protection.md     # updated only on promotion (D-19)
.github/instructions/security.instructions.md  # "Known gap" section rewritten at close (SAST-04)
<triage register — new file or a section; Claude's Discretion, modelled on SECURITY-EXCEPTIONS.md>  # D-17
.planning/
└── <evidence document, e.g. 18-codeql-evaluation.md or similar>  # D-16: conclusion propagates
                                                                    # to security.instructions.md;
                                                                    # raw log stays here
```

### Pattern 1: Buildless (build-mode none) extraction for Rust

**What:** CodeQL's only supported Rust build mode. No `cargo build` is invoked; CodeQL uses an
internal `rust-analyzer` to resolve macros and run `build.rs` scripts, then extracts directly from
source [CITED: docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-build-options-and-steps-for-compiled-languages].

**When to use:** Always, for this phase — it is the *only* option for Rust; there is no `autobuild`
or `manual` mode to choose between (unlike C/C++, C#, Java, Go, Kotlin, Swift).

**Example:**
```yaml
# Source: docs.github.com (compiled-languages build-options reference) + codeql-action/init's
# documented `build-mode` input (values: none | autobuild | manual)
- name: Initialize CodeQL
  uses: github/codeql-action/init@v3
  with:
    languages: rust
    build-mode: none
    queries: security-extended
    debug: true   # produces the src.zip debug artifact used for D-13 evidence
```

### Pattern 2: Debug-artifact-based analysed-file-count evidence (D-13)

**What:** `debug: true` on the init step (or the standalone `debug-artifact-name` /
`debug-database-name` inputs) causes `codeql-action` to upload a debug artifact per run, which
contains — among diagnostic logs — `src.zip`, an archive of the exact source files CodeQL
extracted for the database [CITED: docs.github.com "CodeQL scanned fewer lines than expected"
troubleshooting page]. This is GitHub's own documented mechanism for exactly the discrepancy this
phase exists to catch (files present in the repo vs files actually scanned).

**When to use:** Every observation-window run in D-14/D-15, and certainly the initial probe run in
D-11, so the analysed-file count is measured, not assumed.

**Example (post-run evidence extraction, run in a follow-up step or manually against the
downloaded artifact):**
```bash
# After downloading the debug artifact (actions/download-artifact or the Actions UI):
unzip -q codeql-debug-artifacts.zip -d /tmp/codeql-debug
find /tmp/codeql-debug -name 'src.zip' -exec unzip -l {} \; | grep -c '\.rs$'
# Compare this count against the D-13 denominator of 385.
```
No official page documents the exact internal path of `src.zip` inside the debug artifact zip, or
whether its contents are 1:1 with the CLI's own `codeql database print-baseline` output — treat
this as something to confirm on the *first* real run of `codeql.yml`, not something to assume
works exactly as sketched above.

### Pattern 3: Historical-commit backfill via disposable push branches, not `workflow_dispatch`

**What:** `workflow_dispatch` (both the Actions UI and `gh workflow run --ref <ref>`) only accepts
an existing branch or tag name as its `ref` — passing a bare commit SHA fails with "No ref found
for: `<sha>`" [CITED: cross-checked against multiple `cli/cli` and `github/docs` GitHub issue
reports describing this exact API-level constraint]. Since this repository's release flow keeps
feature branches short-lived (`docs/src/contributing/branching-model.md`), the head branches of
already-merged PRs are very likely already deleted, so there is no live branch/tag ref pointing at
most historical PR-merge commits to dispatch against directly.

**When to use:** D-14's backfill sample. Because `codeql.yml`'s `push` trigger is
`branches: ['**']` (D-04), pushing **any** branch — including a disposable one whose tip is an old
commit SHA — triggers the workflow on that exact tree state, with no `workflow_dispatch` ref
restriction to work around.

**Example:**
```bash
# Sample N recent commits on main (each one, in this repo's trunk-based flow, represents one
# merged PR's tree state) and scan each in isolation:
for sha in $(git log --format=%H -n 20 main); do
  git push origin "${sha}:refs/heads/tmp/codeql-backfill-${sha:0:8}"
  # ... wait for codeql.yml to complete on that ref, record alert count / FP-triage / wall-clock ...
  git push origin --delete "tmp/codeql-backfill-${sha:0:8}"
done
```
The exact sample size (N) and selection criteria (every commit vs only PR-merge commits) is
Claude's Discretion per CONTEXT.md.

### Pattern 4: Ruleset re-application via `PUT`, not `POST`

**What:** The procedure documented in `docs/src/appendix/branch-protection.md` today
(`gh api --method POST ... /rulesets --input <file>`) **creates a new ruleset** — it is the
correct command for the original, one-time application, but re-running it on promotion would
create a duplicate ruleset rather than updating the existing one (id `20868126`). The correct
update call is `PUT /repos/{owner}/{repo}/rulesets/{ruleset_id}`
[CITED: docs.github.com REST API reference, "Update a ruleset for a repository" — accepts partial
updates, but a full-payload body is equally valid].

**When to use:** D-19 step 2, promotion.

**Example:**
```bash
# Source: docs.github.com REST API reference (PUT /repos/{owner}/{repo}/rulesets/{ruleset_id}),
# adapted to match this repo's existing whole-file --input convention used for the original POST
# in docs/src/appendix/branch-protection.md.
gh api --method PUT \
  -H "Accept: application/vnd.github+json" \
  /repos/DF3NDR/paladin-dev-env/rulesets/20868126 \
  --input .github/rulesets/protect-main-branch.json
```
No script or Makefile target in this repo currently wraps this call — `grep -rn "gh api"
scripts/ Makefile` returns nothing — so the plan should either add one (mirroring the existing
apply-ruleset documentation pattern) or record the manual command in the same doc section that
already documents the original `POST` procedure.

### Anti-Patterns to Avoid

- **`continue-on-error` on the CodeQL job or its SARIF upload while advisory:** D-06 forbids this
  explicitly — it is exactly the `osv-scanner` pattern this phase must not copy, and Success
  Criterion 6 ("nothing makes a green result mean less than it says") rules it out categorically.
- **Assuming `--all-features`-equivalent coverage without measuring it:** no official doc supports
  the assumption that buildless Rust extraction sees non-default-feature code. Treat this as
  unverified (see "Open Questions") rather than writing a plan step that silently assumes it works.
- **Placing the probe crate under `crates/`:** the root `Cargo.toml`'s `members = [".", "crates/*"]`
  glob would auto-include anything placed there, defeating D-07's exclusion requirement outright
  (see Pattern in "Standard Stack" / Cargo workspace membership rules below).
- **Dispatching the D-14 backfill via `workflow_dispatch` against historical SHAs:** the API
  rejects bare commit SHAs as a `ref` — use the disposable-push-branch technique (Pattern 3)
  instead.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|--------------|-----|
| SARIF ingestion / alert UI | A custom findings dashboard or a script that parses CodeQL's SARIF output for display | GitHub's native code-scanning alerts UI, fed by `github/codeql-action/upload-sarif` (already proven in this repo via `osv-scanner`) | The repository already has this wired and working; code scanning is enabled today at no incremental cost since the repo is public. |
| Analysed-file-count evidence | A bespoke script that walks the repo and guesses what CodeQL "should" have seen | The `debug: true` → `src.zip` mechanism (Pattern 2) | It is the actual artifact of what CodeQL scanned, not an inference about what it should have scanned — this is precisely the distinction between "analysed 0 files" and "found 0 issues" that the Snyk failure blurred. |
| Ruleset promotion | Hand-editing the ruleset via the GitHub web UI and hoping the committed JSON stays in sync | `gh api --method PUT .../rulesets/{id} --input <file>` (Pattern 4), keeping the committed file as the single source of truth exactly as the original `POST` procedure already establishes | Matches the existing, documented, server-verified procedure in `branch-protection.md` rather than introducing a second, undocumented path to the same state. |

**Key insight:** Every "don't hand-roll" item above already has a precedent *in this repository*
(`osv-scanner`'s SARIF wiring, the original ruleset-application procedure, the
`check-advisory-register.sh`/`SECURITY-EXCEPTIONS.md` governance pair for D-17). This phase's job
is largely to extend proven local patterns to a new tool, not to invent new mechanisms.

## Common Pitfalls

### Pitfall 1: Assuming build-mode `none` sees the same code a full `cargo build --all-features` would

**What goes wrong:** A scan that only ever sees the default-feature-set code silently reports
"clean" while never having analysed the `vision`/`web-server`/`llm-*`/`cli`/etc. subsystems — the
exact Snyk failure shape (a scan that reads as assurance while analysing a fraction of the tree)
recurring with a different mechanism, which is the specific risk D-12 names.
**Why it happens:** There is no official documentation describing how (or whether) the Rust
extractor's internal `rust-analyzer` invocation selects cargo features, and buildless extraction
means there is no explicit `cargo build --all-features` command in the workflow to point at as
proof either way.
**How to avoid:** Treat feature coverage as an empirical question. Options for the plan to
consider: (a) plant one of the probe's four defects — or a fifth, dedicated one — behind a
non-default feature flag and dispatch the workflow to see whether it's found; (b) inspect the
`src.zip` debug artifact from a real run and check whether feature-gated source files (e.g. files
under `paladin-web/src/` that only compile with `web-server`) appear in it at all, independent of
whether a finding was raised.
**Warning signs:** An analysed-file count (D-13) suspiciously close to the *default-feature-only*
file count rather than 385 — that gap is itself the disqualifying signal D-13 exists to catch.

### Pitfall 2: The probe crate silently becoming a workspace member

**What goes wrong:** If the probe crate is placed under `crates/` (matching the `crates/*` glob)
or is ever added as a path dependency of any real crate, Cargo auto-includes it as a workspace
member [CITED: doc.rust-lang.org/cargo/reference/workspaces.html] — at which point `cargo build`,
`clippy --workspace`, and `llvm-cov` at the root all see it, contaminating the 82% coverage floor
and the clippy/fmt gates with intentionally-bad code, and defeating D-07 outright.
**Why it happens:** The `crates/*` glob is broad by design (it's meant to auto-pick-up new
extracted crates); a probe fixture created without checking this rule can trip it by accident.
**How to avoid:** Place the fixture outside `crates/` entirely (e.g. `fixtures/codeql-probe/`),
add an explicit `workspace.exclude` entry as defense-in-depth even though it shouldn't be strictly
necessary given the fixture's location, and confirm with `cargo metadata --no-deps | jq
'.workspace_members'` that it does not appear.
**Warning signs:** `cargo build --workspace` or `cargo clippy --workspace --all-targets
--all-features` referencing the fixture's package name in their output.

### Pitfall 3: The planted credential tripping `gitleaks` / the pre-commit gate

**What goes wrong:** `gitleaks` (already wired via `.pre-commit-config.yaml`, `useDefault = true`
ruleset) will very likely flag a plausible-looking hardcoded API key or password with its default
`generic-api-key`/provider-specific rules, and `pre-commit run --all-files` is a **required
status check** — a probe fixture that turns that check red blocks the PR that adds the fixture
itself.
**Why it happens:** `.gitleaks.toml` currently has exactly one narrow allowlist entry (for the GSD
file manifest's SHA-256 digests) and no general "test fixture" carve-out.
**How to avoid:** Per D-10, use an obviously-synthetic, non-resolving credential value (e.g. a
string that fails Shannon-entropy heuristics, or is explicitly recognizable as fake — `gitleaks`'s
default rules are tuned on entropy plus regex patterns, so a value like
`"sk-THIS-IS-A-PLANTED-TEST-CREDENTIAL-NOT-REAL"` may or may not still match depending on the
specific rule's regex). If it still trips, add a narrowly-scoped `[[allowlist]]` path entry in
`.gitleaks.toml` for the fixture file, with a comment explaining why — matching the file's existing
convention for the GSD-manifest carve-out — rather than reaching for `--no-verify`. If neither
works, D-10 requires recording that as a finding, not bypassing it.
**Warning signs:** `pre-commit run --all-files` (or the `gitleaks` hook specifically) failing on
the commit that adds the probe crate.

### Pitfall 4: "Low Rust analysis quality" diagnostic noise

**What goes wrong:** CodeQL's own `rust/diagnostic/database-quality` query has historically
produced a "Low Rust analysis quality" warning on the tool-status page for newly-onboarded Rust
repositories, which could be mistaken for a real coverage problem (or dismissed as noise when it
isn't) [CITED: WebFetch of `github.com/orgs/community/discussions/161754`, corroborated by a
CodeQL-team maintainer comment in that thread].
**Why it happens:** The warning's threshold was originally tuned too aggressively for a newly
GA'd language; GitHub has been iterating on it (improvements landed around CodeQL 2.23.0, with
further tuning planned).
**How to avoid:** Don't treat this diagnostic as a pass/fail signal on its own. Cross-check it
against the `src.zip` file count (D-13) and the actual finding count on the known-vulnerable probe
(D-11) — those are the load-bearing signals; the quality diagnostic is corroborating context at
best, given its documented history of false alarms.
**Warning signs:** A "Low Rust analysis quality" banner appearing on the code-scanning
configuration page even when the probe's four defects are all correctly found — a real disagreement
between file-count evidence and this diagnostic is worth recording, but the diagnostic alone is
not disqualifying.

### Pitfall 5: `sqlx::query_as!`-style macros producing known false positives

**What goes wrong:** A community-reported false positive on `rust/unused-variable` specifically
involving `sqlx::query_as!` (and similar compile-time-checked-query macros) [CITED: WebFetch of
`github.com/orgs/community/discussions/161754`, comment from user `praseodym`, with a documented
`SQLX_OFFLINE: true` workaround from `hvitved` in the same thread]. Since `sqlx` is already a
real dependency in this workspace (used by `paladin-storage`), this specific FP class is a
realistic possibility during the D-14/D-15 observation window, not a hypothetical.
**Why it happens:** CodeQL's macro-expansion handling for Rust proc-macros that generate code from
external inputs (like a `.sqlx/` query-cache directory or a live DB connection) is imperfect.
**How to avoid:** If `sqlx::query_as!`/`query!` false positives show up during the observation
window, they are exactly the kind of alert the D-17 triage register exists to dismiss with a
recorded, owned, dated compensating-control entry — not evidence against CodeQL's overall
qualification.
**Warning signs:** `rust/unused-variable` alerts specifically inside functions using `sqlx`'s
query macros.

## Code Examples

### Minimal advanced-setup CodeQL Rust workflow skeleton

```yaml
# Source: synthesized from github/codeql-action's init/action.yml documented inputs (WebFetch),
# docs.github.com's build-options-for-compiled-languages page (build-mode: none is the only
# value Rust supports), and this repo's own trigger-policy conventions (D-04/D-05).
name: CodeQL Analysis (Rust)   # D-02: literal, no matrix, no ${{ }} expression

on:
  push:
    branches: [ '**' ]                          # D-04, Clause 2 of the trigger guard
  pull_request:
    branches: [ main, 'release/**' ]             # D-04, deliberately NO paths: filter
  schedule:
    - cron: '0 7 * * 3'                          # Claude's Discretion — offset from
                                                  # benchmarks.yml's Monday 06:00 UTC slot
  workflow_dispatch: {}

concurrency:
  group: codeql-${{ github.head_ref || github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}

jobs:
  codeql:
    name: CodeQL Analysis (Rust)                  # must match the workflow `name:` per D-02
    runs-on: ubuntu-latest
    permissions:
      contents: read
      security-events: write
      actions: read
    steps:
      - uses: actions/checkout@v5

      - uses: dtolnay/rust-toolchain@stable        # CodeQL Rust requires rustup + cargo present

      - uses: Swatinem/rust-cache@v2

      - uses: github/codeql-action/init@v3
        with:
          languages: rust
          build-mode: none                         # the ONLY mode Rust supports
          queries: security-extended                # Claude's Discretion vs default
          debug: true                                # produces src.zip — D-13 evidence

      - uses: github/codeql-action/analyze@v3
        with:
          category: '/language:rust'
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|-------------------|---------------|--------|
| Rust unsupported by CodeQL / public preview requiring manual `autobuild` friction | Rust GA with buildless (`build-mode: none`) extraction via internal `rust-analyzer` | 2025-06-30 preview → 2025-10-14 GA [CITED: github.blog/changelog] | Removes the historical "CodeQL can't do Rust" objection this phase exists to retest; also means there is genuinely no `autobuild`/`manual` fallback to reach for if buildless extraction under-covers feature-gated code. |
| `actions/create-release@v1`-era ad hoc SARIF handling | Native `github/codeql-action/upload-sarif` already wired in this repo for OSV results | Already in place (`ci.yml` `osv-scanner` job) | Zero incremental setup cost for SARIF ingestion — the phase only needs to add the CodeQL-producing job, not plumb a new results pathway. |

**Deprecated/outdated:**
- The framing "CodeQL has no meaningful Rust support" (true when Snyk was evaluated in mid-2026
  and still true as recently as CodeQL's June 2025 preview) is now stale as of GA — but staleness
  of the *previous* objection does not resolve the *new* open question this research surfaces
  (cargo-feature coverage under buildless extraction).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|----------------|
| A1 | `security-extended` is the right query suite (vs. plain `default`) for this phase | Standard Stack / Code Examples | Low — this is explicitly Claude's Discretion per CONTEXT.md; either suite is a defensible choice, and switching later is a one-line YAML change. |
| A2 | `rusqlite` or `sqlx` are suitable SQL-injection vehicles for the probe's fourth defect class | Standard Stack, Package Legitimacy Audit | Low — both verified `OK` on the crates.io registry; the choice itself (which crate, or reusing sqlx to avoid a new name) is explicitly Claude's Discretion. |
| A3 | A weekly Wednesday 07:00 UTC schedule cadence for `codeql.yml` (offset from `benchmarks.yml`'s Monday slot) | Code Examples | Low — cadence is explicitly Claude's Discretion; any reasonable non-colliding cadence satisfies SAST-02's "plus a schedule" clause. |
| A4 | Semgrep's Rust ruleset recently gained "reachability analysis" (WebSearch-summarized claim about `registry.semgrep.dev/ruleset/rust`, not independently fetched) | Alternatives Considered | Low — Semgrep is out-of-scope contingency work per D-20; this claim is contextual color only and does not drive any plan decision in this phase. |
| A5 | `src.zip`'s internal structure exactly mirrors the source tree 1:1 (so a plain `find ... -name '*.rs' | grep -c` against its unzipped contents is a valid proxy for "files analysed") | Architecture Patterns (Pattern 2) | Medium — if `src.zip` includes vendored/external dependency source or excludes some first-party files for a reason not yet observed, the raw file count could over- or under-state D-13's evidence. **This must be confirmed on the first real workflow run**, not assumed from documentation, since no official page describes `src.zip`'s exact contents in detail. |

**Confirm before finalizing:** A5 in particular should be the first thing checked once `codeql.yml`
runs for the first time — it's the mechanism this whole research recommends for D-13's evidence,
and its exact fidelity is unverified by documentation.

## Open Questions

1. **Does buildless Rust extraction reach code gated behind non-default cargo features?**
   - What we know: build-mode `none` is the only mode; CodeQL uses an internal `rust-analyzer`
     invocation rather than a documented, configurable `cargo build`/`cargo check` command; no
     official doc page (build-options reference, extractor-options reference, `codeql-config.yml`
     reference, or `codeql-action/init`'s input list) mentions cargo feature selection for Rust at
     all.
   - What's unclear: whether `rust-analyzer`'s own default feature-resolution behavior (which,
     per rust-analyzer's own docs, defaults to a `cargo.features` config that is *not* documented
     to default to "all" when invoked headlessly by another tool) results in default-features-only
     analysis, all-features analysis, or something else when driven by CodeQL rather than an IDE.
   - Recommendation: the plan must include an explicit empirical test — either a fifth planted
     defect behind a non-default feature flag (e.g. inside `paladin-web`'s `web-server`-gated
     code, or a small addition to the probe crate mirroring one real feature-gated module) scanned
     with the feature both off and on, or a direct inspection of the `src.zip` debug artifact
     against a known feature-gated file list. Do not let the plan proceed on an assumption either
     way — this is precisely the class of unverified-coverage risk D-12 exists to prevent.

2. **Does `src.zip`'s file count exactly equal "files CodeQL actually extracted," or could it
   include files that were present but not successfully parsed?**
   - What we know: `src.zip` is documented as "an archived copy of the source files scanned by
     CodeQL" on GitHub's own troubleshooting page.
   - What's unclear: whether "scanned" here means "present in the extraction working set" (which
     could include files CodeQL attempted but failed to fully extract, similar to the `Unextracted`
     concept surfaced in CodeQL's own Rust standard-library documentation) versus "successfully
     produced usable database facts for."
   - Recommendation: cross-check the `src.zip` count against `codeql database print-baseline`'s
     lines-of-code number and, if available in the CLI, any per-file diagnostic/success indicator,
     on the first real run — don't treat a single number as sufficient evidence without a second,
     independent corroborating signal.

3. **Does "Clause 4" (referenced in `REQUIREMENTS.md`'s SAST-02 text: "`scripts/check-workflow-
   triggers.sh` Clause 4 exists because a required context living in a path-filtered workflow
   never reports...") literally exist as a fourth numbered clause?**
   - What we know: direct reading of `scripts/check-workflow-triggers.sh`'s own header comment
     enumerates exactly **three** clauses (coverage, drift, context resolution) as of this
     research (2026-08-25); no fourth clause exists in the script.
   - What's unclear: whether `REQUIREMENTS.md`'s "Clause 4" is a documentation drift (an informal
     reference to the *reasoning* behind Clause 2's push-filter enforcement, misnumbered), or
     whether a fourth clause was planned but never implemented, or removed.
   - Recommendation: this is a minor, non-blocking discrepancy — D-04 already independently locks
     in the correct trigger shape (no `paths:` filter on `pull_request`) regardless of which
     clause number enforces the underlying principle. Worth a one-line correction to
     `REQUIREMENTS.md` if the planner has spare scope, but it does not change any implementation
     decision in this phase.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|--------------|-----------|---------|----------|
| GitHub code scanning (platform feature) | SARIF ingestion, alert UI | ✓ | — | — (already enabled; repo is public, no licence cost) |
| `github/codeql-action` | The whole scan | ✓ | v3.37.8 / v4.37.8 current [CITED: GitHub releases page] | — |
| `rustup` + `cargo` on `ubuntu-latest` | CodeQL Rust extraction | ✓ (installed explicitly via `dtolnay/rust-toolchain@stable`, matching this repo's convention rather than relying on the runner image's pre-baked version) | matches `rust-toolchain.toml` pin (1.97.1) if the toolchain action is configured to honor it, else "stable" | — |
| `gh` CLI with repository-admin scope | Ruleset `PUT` re-application (D-19) | Assumed available in this repo's existing operator workflow (the original `POST` procedure already documents `gh api` usage) | — | — |
| `gitleaks` (via pre-commit) | D-10's credential-safety check | ✓ | `v8.21.2` pinned in `.pre-commit-config.yaml` | — |

**Missing dependencies with no fallback:** none identified.
**Missing dependencies with fallback:** none identified — every dependency this phase needs is
either already present in the repo's toolchain or is a GitHub-platform feature already enabled.

## Validation Architecture

`.planning/config.json`'s `workflow` key does not set `nyquist_validation`, so per the default
policy this section is included — but this phase's "tests" are fundamentally different from unit
tests: the deliverable is a scanner's measured behavior against a fixture, not application code
covered by `cargo test`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | N/A in the traditional sense — this phase's verification unit is a CI workflow run's outcome (alert count, analysed-file count, wall-clock), not a `cargo test` assertion |
| Config file | `.github/workflows/codeql.yml` itself is the "test config" |
| Quick run command | `gh workflow run codeql.yml` (once merged) or push the probe branch and observe the run |
| Full suite command | The full D-14/D-15 observation-window procedure (backfill + live advisory period) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|---------------------|--------------|
| SAST-01 | Probe finds (or doesn't find) all 4 planted defect classes | manual-only (a CI run's SARIF output is read by a human/agent, not asserted by an automated test) | `gh run view <run-id> --log` / code-scanning alerts API for the probe branch | ❌ Wave 0 — no probe fixture exists yet |
| SAST-02 | Workflow triggers correctly on PR/push/schedule/dispatch, no path filter | automated | `scripts/check-workflow-triggers.sh` (already exists, already enforces Clause 1/2/3 against the new workflow once it exists) | ✓ exists (script), ❌ Wave 0 (workflow file itself) |
| SAST-03 | FP rate / wall-clock recorded; promotion updates all 4 places | manual-only (requires the observation-window procedure, not a unit test) | the D-14/D-15 evidence-gathering procedure this phase's plan must define | ❌ Wave 0 |
| SAST-04 | `security.instructions.md` rewritten and internally consistent | manual review (a documentation-content check, not a `cargo test`) | `grep -n "no taint analysis\|Known gap"  .github/instructions/security.instructions.md` (sanity check only) | ✓ exists (target file) |

### Sampling Rate

- **Per task commit:** whatever the plan's individual task boundaries are (e.g., after adding the
  probe crate, after adding the workflow file) — no automated "quick run" exists for this domain
  beyond `check-workflow-triggers.sh` and `pre-commit run --all-files`.
- **Per wave merge:** a real dispatched or pushed CodeQL run, read for its actual outcome.
- **Phase gate:** the D-14/D-15 evidence document complete and committed under `.planning/` before
  `/gsd-verify-work`.

### Wave 0 Gaps

- [ ] The probe fixture crate itself (`fixtures/codeql-probe/` or wherever the plan places it) —
      does not exist yet.
- [ ] `.github/workflows/codeql.yml` — does not exist yet.
- [ ] The triage register (D-17) — does not exist yet, new file or section TBD by the plan.
- [ ] The D-16 evidence document under `.planning/` — does not exist yet.

*(No `cargo test`-shaped gap exists — this phase's verification surface is CI-run outcomes and
documentation content, not application unit tests.)*

## Security Domain

`security_enforcement` is absent from `.planning/config.json`, so per default policy this section
is included — though this entire phase **is** a security-tooling phase, so the "standard control"
column below mostly points back at this phase's own deliverables rather than an external library.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|----------------|---------|-------------------|
| V1 Architecture, Design and Threat Modeling | Partial | This research's Architectural Responsibility Map; the phase's own D-01–D-22 decisions constitute the threat-modeling record for this control surface. |
| V5 Input Validation | No (indirect only) | Not applicable to the scanner-adoption tooling itself; the probe fixture's *planted* defects are deliberately unvalidated input handling (command injection, path traversal, SQL injection) — that is the point of the fixture, not a gap to fix in it. |
| V6 Cryptography | No | Not applicable — no cryptographic material is introduced by this phase. |
| V14 Configuration | Yes | GitHub ruleset configuration (`.github/rulesets/*.json`), workflow trigger configuration (`docs/src/contributing/branching-model.md`'s register), and the guard scripts that keep them internally consistent — this is the actual security-relevant configuration surface this phase touches. |
| (GitHub Actions / supply-chain, not a numbered ASVS category but directly relevant) | Yes | Pin `codeql-action` to a specific major version (`v3`, matching this repo's existing convention) rather than `@main`/unpinned; scope `id-token`/`security-events` permissions narrowly per job, matching this repo's existing per-job `permissions:` convention (seen in `osv-scanner`'s job-level `permissions:` block). |

### Known Threat Patterns for {this phase's stack}

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|------------------------|
| A permanently-scanned probe fixture emitting standing "wontfix" alerts that get silently dismissed forever | Repudiation (an alert dismissal with no accountable trail looks the same as "never had a problem") | D-09 (probe excluded from steady-state PR path after evaluation) + D-17 (governed dismissal register modelled on `SECURITY-EXCEPTIONS.md`, with named owner/review-date/compensating-control per dismissal) |
| A scanner reporting "0 findings" that reads as "clean" when it actually analysed nothing | Repudiation / false assurance (the specific failure this whole phase exists to prevent) | D-13 (analysed-file-count as first-class, separately-recorded evidence alongside the finding count) |
| Overly broad `id-token`/`security-events: write` permissions granted at workflow level rather than job level | Elevation of Privilege | This repo's established per-job `permissions:` block convention (seen in `osv-scanner`); the new `codeql.yml` job should declare `contents: read`, `security-events: write`, `actions: read` at the job level, not workflow level. |

## Sources

### Primary (HIGH confidence)

None — no source in this research was independently cross-verified by two authoritative sources
in a way that would qualify for the seam's `--verified` HIGH tier, though several individual facts
(GitHub REST API endpoint shapes, Cargo workspace membership rules, the `workflow_dispatch`
SHA-ref rejection) are corroborated across multiple independent official/semi-official pages and
are treated as effectively CITED-grade in the prose above.

### Secondary (MEDIUM confidence) — `[CITED: ...]` tags above

- `docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-build-options-and-steps-for-compiled-languages` — Rust build-mode `none`, `rust-analyzer` internal usage, `rustup`+`cargo` runner prerequisite.
- `docs.github.com/en/code-security/code-scanning/troubleshooting-code-scanning/fewer-lines-scanned-than-expected` — `debug: true` → `src.zip` mechanism.
- `docs.github.com` REST API reference — `PUT /repos/{owner}/{repo}/rulesets/{ruleset_id}`.
- `doc.rust-lang.org/cargo/reference/workspaces.html` — exact workspace-membership rules (`members` globs, path-dependency auto-inclusion, `exclude`).
- `github.blog/changelog/2025-10-14-codeql-scanning-rust-and-c-c-without-builds-is-now-generally-available/` — GA date.
- `github.com/github/codeql-action` releases — current `v3`/`v4` version numbers.
- `github.com/github/codeql-action/blob/main/init/action.yml` — full input list (no cargo-feature-passthrough input exists).
- `github.com/orgs/community/discussions/161754` — "Low Rust analysis quality" diagnostic history, `sqlx::query_as!` false-positive report, `build-mode: manual` rejected for Rust (corroborating that `none` is the only mode).

### Tertiary (LOW confidence) — WebSearch-only, not independently fetched

- CodeQL default/security-extended query counts (491/135 additional queries, 166/35 CWE) — WebSearch-aggregated from `codeql.github.com` changelog pages, not independently fetched and quoted.
- Semgrep Rust reachability-analysis claim — WebSearch summary of a Semgrep blog post, not independently fetched.
- Any wall-clock time estimate for CodeQL Rust analysis on a codebase this size — explicitly **not found** anywhere; flagged as requiring empirical measurement during this phase's own observation window rather than sourced externally.

## Metadata

**Confidence breakdown:**
- Standard stack (CodeQL mechanics, action versions, build-mode): MEDIUM — directly fetched and quoted from official docs.github.com/github.blog pages, but not cross-verified by a second independent authoritative source for every claim.
- Architecture (workflow wiring, ruleset promotion mechanics, workspace-exclusion mechanics): MEDIUM-HIGH — Cargo workspace rules and the GitHub ruleset `PUT` endpoint are both quoted verbatim from their respective official reference docs.
- Pitfalls (feature coverage, gitleaks, analysis-quality diagnostic, sqlx FP): MEDIUM for the documented ones (gitleaks, analysis-quality, sqlx FP — all sourced from official docs or a GitHub-hosted community discussion with maintainer participation); the feature-coverage pitfall itself is the phase's central **unverified** risk, deliberately reported as an open question rather than resolved.

**Research date:** 2026-08-25
**Valid until:** ~30 days (2026-09-24) — CodeQL's Rust support is actively evolving post-GA (multiple point releases with new Rust queries observed within the last few months of this research), so extractor behavior and query coverage should be re-checked if this research is consumed significantly later than the phase it was written for.
