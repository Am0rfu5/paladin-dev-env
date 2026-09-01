# Phase 18: Rust SAST — Evaluate and Adopt CodeQL - Pattern Map

**Mapped:** 2026-08-25
**Files analyzed:** 9 (new/modified)
**Analogs found:** 7 / 9

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|--------------------|------|-----------|-----------------|----------------|
| `.github/workflows/codeql.yml` | CI workflow (config) | event-driven (push/PR/schedule/dispatch) | `.github/workflows/ci.yml` (trigger/concurrency block) + `.github/workflows/benchmarks.yml` (schedule shape) + `ci.yml`'s `osv-scanner` job (SARIF wiring) | exact (composite — no single file covers CodeQL end-to-end, but the three pieces together do) |
| `fixtures/codeql-probe/Cargo.toml` (+ `src/*.rs`) | fixture crate / config | file-I/O, event-driven (deliberately vulnerable) | root `Cargo.toml` (`[workspace] members`/exclusion shape) | role-match (no existing "excluded fixture crate" precedent in this tree; `doc-examples/` referenced in prompt does not exist — see No Analog) |
| Row in `docs/src/contributing/branching-model.md` trigger-policy table | documentation / config register | CRUD (row insert) | Existing rows for `ci.yml`/`benchmarks.yml`/`docs.yml` in the same table (lines ~47-54) | exact |
| Alert-triage governance register (D-17) | governance register (model/config) | CRUD | `SECURITY-EXCEPTIONS.md` (eleven-field row schema) | exact |
| Enforcing guard for the triage register (optional, D-17 discretionary) | utility / CI guard script | batch, read-only validation | `scripts/check-advisory-register.sh` and `scripts/check-workflow-triggers.sh` | exact |
| `.github/rulesets/protect-main-branch.json` (promotion edit) | config | CRUD (append one context object) | itself — existing file, same shape, append one `{"context": "CodeQL Analysis (Rust)"}` object to `required_status_checks` | exact (self-referential — this is an edit, not a new-role file) |
| `docs/src/appendix/branch-protection.md` (promotion edit) | documentation | CRUD (table row + 3 prose numeral edits) | itself — existing file, lines 85/117/180 carry the `44` count to bump to `45` | exact |
| `.gitleaks.toml` (possible allowlist entry, D-10 contingency) | config | CRUD | itself — existing `[[allowlist]]`/`[allowlist]` block for the GSD manifest carve-out | exact |
| `.github/instructions/security.instructions.md` (SAST-04 rewrite) | documentation | transform (narrow, not delete) | itself — existing "Known gap: no Rust SAST" section, and its own "Snyk was evaluated and removed" section as the template for how a verdict-with-evidence subsection reads | exact |

## Pattern Assignments

### `.github/workflows/codeql.yml` (CI workflow, event-driven)

**Analog 1 — trigger/concurrency shape:** `.github/workflows/ci.yml` lines 1-33

**Imports/trigger pattern** (`ci.yml` lines 11-33):
```yaml
on:
  push:
    branches: [ '**' ]
  pull_request:
    branches: [ main, 'release/**' ]
  workflow_dispatch:

concurrency:
  # Collapses the duplicate push+pull_request runs the trigger comment above
  # describes. `head_ref` is set only on pull_request; on push it is empty and
  # this falls back to `ref`, so both runs for one branch share a group.
  # Not cancel-in-progress on main: a completed run on the default branch is a
  # release-history record, not just a PR signal.
  group: ci-${{ github.head_ref || github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}
```
Copy this verbatim in shape (rename `group:` prefix to `codeql-`), and add the `schedule:` key CI
deliberately omits (D-03/D-04). Also copy `ci.yml`'s top-of-file explanatory comment style — every
trigger decision here is explained in a comment block, not left bare; `codeql.yml` should carry an
equivalent comment for the `push: ['**']` choice (D-04's rationale) and cross-reference
`branching-model.md`.

**Analog 2 — schedule-trigger shape:** `.github/workflows/benchmarks.yml` lines 1-11
```yaml
on:
  schedule:
    - cron: '0 6 * * 1'  # weekly, Monday 06:00 UTC
  workflow_dispatch:
```
This is the only existing workflow with a `schedule:` key — copy the comment-then-cron pattern and
its stated rationale style ("Deliberately its own workflow file, not a `schedule:` key on
`ci.yml`... a cron placed there would trigger the entire pipeline weekly"). `codeql.yml` needs an
analogous comment justifying its own cadence choice and offset from `benchmarks.yml`'s slot.

**Analog 3 — SARIF/security-scanning job shape and the exact anti-pattern to avoid:**
`ci.yml` lines 155-181 (`osv-scanner` job)
```yaml
osv-scanner:
    name: OSV Scanner
    runs-on: ubuntu-latest
    permissions:
      contents: read
      security-events: write
      actions: read
    steps:
      - name: Checkout code
        uses: actions/checkout@v5

      - name: Run OSV-Scanner
        uses: google/osv-scanner-action/osv-scanner-action@v1.9.1
        continue-on-error: true
        ...
      - name: Upload OSV-Scanner results (PR annotations)
        if: always()
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: osv-results.sarif
        continue-on-error: true
```
Copy: the job-level `permissions:` block (`contents: read`, `security-events: write`,
`actions: read`) — this is the only existing precedent for a SARIF-producing job's permission
scoping in this repo, and it matches D-02's job-name-as-contract convention (job `name:` here is
also the string GitHub matches on).
**Do NOT copy:** the `continue-on-error: true` on both the scan step and the SARIF upload step.
D-06 explicitly forbids this pattern for `codeql.yml` — the advisory posture must come from the
context simply not being in the ruleset yet, not from a green-but-meaningless step. This is the
one place in the whole phase where the closest analog's core behavior must be deliberately
inverted, not copied — record that inversion in the plan explicitly.

**Error handling / non-blocking posture pattern:** none to copy — this is the negative case above.
No file in the repo demonstrates the "visible non-blocking via ruleset absence" pattern yet;
`codeql.yml` is the first. Treat as **no analog** for this specific sub-pattern; RESEARCH.md's own
code example (lines 525-570) is the reference to follow instead.

---

### `fixtures/codeql-probe/` (probe fixture crate, file-I/O + event-driven vulnerability classes)

**Analog:** root `Cargo.toml` workspace declaration (lines 1-3)
```toml
[workspace]
members = [".", "crates/*"]
resolver = "2"
```
**Pattern to copy:** none of the members glob — the opposite. The fixture crate must have its own
standalone `Cargo.toml` with no `[workspace]` table pointing back at the root, placed outside
`crates/` (which the `crates/*` glob would auto-include) and outside any path a `members` entry or
another crate's path-dependency could reach. If defense-in-depth is wanted, add an explicit
`exclude = ["fixtures/codeql-probe"]` array to the root `[workspace]` table — there is no existing
`exclude` key in the current root `Cargo.toml` to copy from, so this would be a new key, not a
modification of an existing pattern.

**No analog found** for the fixture crate's internal code shape (four vulnerability classes:
hardcoded credential, `sh -c` command injection, path traversal, SQL injection). This repository
has no existing "deliberately vulnerable" code to pattern-match against — by design, since
`clippy -- -D warnings` and the manual credential-handling review exist specifically to prevent
this shape from appearing in first-party code. The planner should treat RESEARCH.md's own
Pattern/Code-Examples section and the Snyk-era probe methodology (D-08, reused verbatim) as the
source of truth here, not a codebase analog.

**doc-examples/ note:** the pattern-mapping brief suggested `doc-examples/` as a precedent for a
non-typical crate layout. That directory does not exist in this tree (`ls doc-examples` returned
nothing) — reporting this honestly rather than inventing a match. No such analog exists.

---

### Row in `docs/src/contributing/branching-model.md` trigger-policy table (documentation register, CRUD)

**Analog:** the table itself, existing rows (lines ~47-54)
```markdown
| Workflow | Triggers | Push branch filter | Rationale |
|----------|----------|---------------------|-----------|
| `ci.yml` | `push`, `pull_request`, `workflow_dispatch` | `['**']` | Core gate ... |
| `benchmarks.yml` | `schedule`, `workflow_dispatch` | not applicable — declares no push trigger at all | Weekly Monday 06:00 UTC cadence ... |
```
**Pattern to copy exactly:** the new `codeql.yml` row's cell values must match D-05: "Triggers"
cell literally names `push`, `pull_request`, `schedule`, `workflow_dispatch` (all four, since this
is the first workflow with all four together — no existing row is a 1:1 template for that
combination, `ci.yml`'s is closest for the push/PR/dispatch trio and `benchmarks.yml`'s for the
schedule slice). "Push branch filter" cell must read `['**']` verbatim, matching the YAML's
`on.push.branches` list literally — `scripts/check-workflow-triggers.sh` Clause 2 compares these
as plain strings. Table format constraint stated directly above the table: "the guard parses this
table with a line-based reader that splits each row on `|`... no merged cells, and no multi-line
cells." One physical line per row, non-negotiable.

---

### Alert-triage governance register (D-17)

**Analog:** `SECURITY-EXCEPTIONS.md` (full file; eleven-field schema shown at lines 60-73 for one
sample row: `id`, `class`, `crate`, `path`, `why_present`, `why_not_fixable`, `owner`,
`review_date`, `scope`, `compensating_control`, `revisit_condition`)

**Core pattern to copy** (structure, not field names verbatim — D-17 fields differ since this
governs dismissed *alerts*, not suppressed *advisories*):
```toml
[[exception]]
id = "RUSTSEC-2023-0071"
class = "vulnerability"
crate = "rsa"
...
owner = "DF3NDR"
review_date = "2026-12-31"
scope = "dev/test dependency graph only, ..."
compensating_control = "The Marvin timing side-channel is only reachable ..."
revisit_condition = "sqlx-mysql upgrades its transitive rsa dependency ..."
```
Map this to a CodeQL-alert-dismissal register with equivalent fields: alert rule ID (e.g.
`rust/unused-variable`), file/location, why dismissed, owner, review date, scope, compensating
control. Copy the file's framing conventions too: an explanatory prose header stating why a
machine-readable block plus a guard script is needed (a bare GitHub dismissal reason has no
queryable owner/date field), a `<!-- BEGIN MACHINE-READABLE REGISTER -->`-delimited TOML/fenced
block, and an explicit "schema contract, stated plainly" paragraph.

**Enforcing guard analog:** `scripts/check-advisory-register.sh` (full file read, header comment
lines 1-58 and clause structure). House style to copy exactly:
- Offline, no network call.
- Reads structurally (Python `tomllib` for TOML / a line-based reader for Markdown tables), never
  scrapes prose or comment text to infer meaning.
- Accumulates every violation into one shared failure list before deciding the verdict — never
  stops at the first failure.
- Read-only: writes nothing, creates no temp file; running twice with no input change produces
  identical output and exit code.
- A missing input file, or an input that parses to zero rows while a corresponding surface is
  non-empty, is a **named non-zero failure**, never a silent pass.
- Registered as a `make` target following the existing pattern at `Makefile` lines 171-181:
```makefile
.PHONY: check-advisory-register
check-advisory-register: ## Verify SECURITY-EXCEPTIONS.md agrees with deny.toml/.cargo/audit.toml/Cargo.lock
	@./scripts/check-advisory-register.sh
```
and folded into the `check-gates` aggregate target (`Makefile` line 184) alongside the other guards
if D-17's register gets its own script.

---

### `.github/rulesets/protect-main-branch.json` (promotion edit, D-19 step 1)

**Analog:** the file itself — existing `required_status_checks.parameters.required_status_checks`
array (44 objects, each shaped `{"context": "<job display name>"}`), e.g.:
```json
{
  "context": "API Surface Tracking"
},
{
  "context": "Benchmark Compile Check"
},
```
**Pattern:** append one object `{"context": "CodeQL Analysis (Rust)"}` — the literal string must
equal D-02's job `name:` exactly (Clause 3 of `check-workflow-triggers.sh` resolves it). This is a
44→45 array-append, not a schema change. Apply with `PUT`, not `POST` (RESEARCH.md Pattern 4) —
`gh api --method PUT /repos/{owner}/{repo}/rulesets/20868126 --input <file>` — to avoid creating a
duplicate ruleset; no script currently wraps this call (`grep -rn "gh api" scripts/ Makefile`
returns nothing per RESEARCH.md), so record the manual command in
`docs/src/appendix/branch-protection.md`'s existing procedure section rather than inventing a new
guard script for a one-time promotion action.

---

### `docs/src/appendix/branch-protection.md` (promotion edit, D-19 step 3)

**Analog:** the file itself — three prose occurrences of the count plus the context table.
```
line 85:  `protect-main-branch.json` requires all 44 of the following status-check contexts...
line 117: `main`: a pull request with all 44 required checks green...
line 180: # 1. Open a PR for your changes and get it merged into main (all 44 required checks must pass).
```
**Pattern:** all three numeral occurrences plus the context table must move from 44→45 together,
in the same commit as the ruleset edit — D-19 names this explicitly as one of the four coordinated
places; a partial edit leaves the doc self-contradictory, exactly the defect class `SAST-03` exists
to forbid.

---

### `.gitleaks.toml` (contingency edit, D-10 / Pitfall 3)

**Analog:** the file itself — its one existing `[allowlist]` block
```toml
[allowlist]
description = "GSD file manifest — SHA-256 integrity digests, not credentials"
paths = ['''^\.claude/gsd-file-manifest\.json$''']
```
**Pattern to copy, only if the planted credential trips gitleaks:** a narrowly-scoped
`[[allowlist]]` (array form, since a second entry means the file needs the repeatable-table variant
rather than the single-table `[allowlist]` currently used — verify TOML semantics when adding) path
entry scoped to the specific fixture file, with a comment explaining why, mirroring this file's
existing "scoped to this single generated file rather than all of X" reasoning style. Do not widen
the allowlist beyond the one fixture file, and do not use `--no-verify` if it still fails (D-10).

---

### `.github/instructions/security.instructions.md` (SAST-04 rewrite)

**Analog:** the file's own existing "Snyk was evaluated and removed (2026-08-18)" section (full
section, ~20 lines) — this is the template for how a verdict-with-evidence subsection should read:
states the date, states what was measured (not assumed), gives the concrete counts (0 Rust findings
vs 3 JS findings), states the standing prohibition. The "Known gap: no Rust SAST" section
immediately below it (last ~10 lines of the file) is the section D-21/D-22 require rewritten.

**Core pattern to copy:** the Snyk section's rhetorical structure — "Measured, not assumed:" followed
by concrete bullet evidence — should be echoed in the CodeQL verdict subsection, whichever branch
(qualified/disqualified) applies. D-22 requires narrowing "by evidence, never deleted": the three
concrete manual-review checks currently listed (redact-before-truncate, no key interpolation in
logs, no redirect-following on credentialed clients) must remain stated as human-owned unless the
probe specifically proved CodeQL covers one of them.

## Shared Patterns

### Job-level permissions scoping
**Source:** `.github/workflows/ci.yml` lines 158-161 (`osv-scanner` job)
**Apply to:** `codeql.yml`'s single job
```yaml
permissions:
  contents: read
  security-events: write
  actions: read
```
Matches this repo's existing per-job (not workflow-level) permissions convention, and RESEARCH.md's
Security Domain section calls this out explicitly as the required scoping.

### Concurrency-group cancellation, non-`main`-preserving
**Source:** `.github/workflows/ci.yml` lines 26-33
**Apply to:** `codeql.yml`
```yaml
concurrency:
  group: codeql-${{ github.head_ref || github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}
```

### Guard-script house style (offline, read-only, accumulate-all-violations)
**Source:** `scripts/check-advisory-register.sh` header comment (lines 1-58) and
`scripts/check-workflow-triggers.sh` header comment (lines 1-70) — nearly identical stated
conventions in both.
**Apply to:** any new guard script D-17 adds for the triage register.
Key rules: parse structurally not textually; accumulate every violation before verdict; a missing
or zero-row input against a non-empty counterpart is a **named** failure; writes nothing; running
twice produces identical output.

### Trigger-policy-table-plus-workflow same-commit landing
**Source:** `docs/src/contributing/branching-model.md`'s own stated rule ("a workflow added without
a row... fails a required check instead of merging unnoticed") and D-05.
**Apply to:** `codeql.yml` + its table row — must land together, not as a follow-up commit.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `fixtures/codeql-probe/src/{credential,command_injection,path_traversal,sql_injection}.rs` | fixture / vulnerable source | event-driven | No deliberately-vulnerable code exists anywhere in this codebase by design (clippy + manual review actively prevent this shape). RESEARCH.md's own methodology (D-08, reusing the Snyk-era probe verbatim) is the correct source, not a codebase analog. |
| The "visible non-blocking via ruleset absence" posture (D-06) | CI workflow behavior | event-driven | The only existing SARIF-producing job (`osv-scanner`) uses the opposite pattern (`continue-on-error: true` stacked twice), which D-06 explicitly forbids copying. No positive-example analog exists yet in this tree; `codeql.yml` will be the first. |
| `doc-examples/` as a "non-typical crate layout" reference | — | — | Directory does not exist in this tree; the pattern-mapping brief's suggested analog is stale/incorrect. Use the root `Cargo.toml` workspace-membership rules directly instead. |
| Fifth feature-gated planted defect / feature-coverage probe (Pitfall 1, D-12) | test/probe extension | event-driven | No existing pattern in this repo for verifying a scanner's cargo-feature reach — this is a genuinely novel empirical-verification step RESEARCH.md flags as an open question, not something to pattern-match. |

## Metadata

**Analog search scope:** `.github/workflows/`, root `Cargo.toml`, `SECURITY-EXCEPTIONS.md`,
`scripts/check-advisory-register.sh`, `scripts/check-workflow-triggers.sh`,
`scripts/check-workflow-suppressions.sh`, `docs/src/contributing/branching-model.md`,
`docs/src/appendix/branch-protection.md`, `.github/rulesets/protect-main-branch.json`,
`.gitleaks.toml`, `.github/instructions/security.instructions.md`, `Makefile`.
**Files scanned:** 12
**Pattern extraction date:** 2026-08-25
