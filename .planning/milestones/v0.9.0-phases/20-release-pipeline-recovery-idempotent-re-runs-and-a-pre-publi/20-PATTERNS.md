# Phase 20: Release Pipeline Recovery — Pattern Map

**Mapped:** 2026-08-28
**Files analyzed:** 8 (2 modified workflow/config, 3 new scripts/docs+1 test, Makefile edits, changelog-shape files)
**Analogs found:** 8 / 8

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|-----------------|---------------|
| `.github/workflows/release.yml` (`create-release` job rewrite) | CI workflow step / controller | request-response (GitHub API create-or-reuse) | `.github/workflows/release.yml` `sbom` job's `RELEASE_VERSION` env-indirection + `upload-release-asset@v1` output-consumption pattern (same file, different job) | exact (same file, sibling job) |
| `.github/workflows/release.yml` (`publish-crates` / `publish_one()` rewrite) | CI workflow step / batch loop | batch + request-response (registry polling) | `.github/workflows/release.yml` existing `publish_one()` (same job, being edited in place) | exact |
| `.github/workflows/release.yml` (new gate job, e.g. `check-release-consistency`) | CI workflow job (thin script invocation) | request-response | `.github/workflows/release.yml` `test` job (simple `needs:`-gated job wrapping one shell invocation) | role-match |
| `scripts/check-release-consistency.sh` (new) | utility / validation script | batch (collect-then-report), request-response (CI-conclusion API) | `scripts/check-workflow-triggers.sh` | exact (collect-all-failures house style, python3-heredoc parsing, offline-first) |
| `scripts/finalize-crate-changelogs.sh` (new, D-09 fallback) or `Makefile` loop extension | utility / transform script | file-I/O (text substitution across N files) | `Makefile` `release` target's `perl -0pi -e 's/## \[Unreleased\]/…/'` root-changelog finalize step | role-match |
| `tests/scripts/check-release-consistency_test.sh` (new) | test | fixture-driven (mktemp + trap) | `tests/scripts/check-workflow-triggers_test.sh` | exact |
| `docs/src/appendix/release-recovery.md` (new) | doc | N/A | `docs/src/appendix/release-checklist.md` + `docs/src/appendix/release-automation.md` | role-match |
| `Makefile` (new `check-release-consistency` / `release-consistency-check` target, `release` target extension for per-crate changelogs) | config / build target | request-response (thin wrapper) | `Makefile` `check-changelogs` target (`@./scripts/check-changelogs.sh`) and `check-gates` composite target | exact |

## Pattern Assignments

### `.github/workflows/release.yml` — `create-release` job (D-01/D-02)

**Analog:** same file, `sbom` job (lines 353–391) for the env-indirection convention, and the job's own current body (lines 100–172) for what must be preserved (outputs contract).

**Env-indirection pattern to reuse** (`release.yml` lines 359–363):
```yaml
    env:
      # Indirect through env: rather than interpolating the tainted output
      # directly into run:/with: blocks (CR-01) -- the version ultimately
      # traces back to the workflow_dispatch `tag` input.
      RELEASE_VERSION: ${{ needs.create-release.outputs.version }}
```
Apply this same CR-01 indirection convention to any tag-derived value the new
`gh api` create-or-reuse step interpolates (do not put `${{ steps.get_version.outputs.version }}`
directly inside a `run:` block's shell command — pass via `env:` first).

**Outputs contract that MUST be preserved** (`release.yml` lines 106–108):
```yaml
    outputs:
      upload_url: ${{ steps.create_release.outputs.upload_url }}
      version: ${{ steps.get_version.outputs.version }}
```
`build-binaries` (lines 331–349) and `sbom` (lines 383–391) both consume
`needs.create-release.outputs.upload_url` via `actions/upload-release-asset@v1` — the
rewritten step must still emit `upload_url` to `$GITHUB_OUTPUT` under the same step id
convention (`steps.create_release.outputs.upload_url`) or these two jobs break.

**Replace this block** (lines 147–172, `actions/create-release@v1`) with `gh api`
create-or-reuse (see RESEARCH.md Pattern 1 / Code Example 3) — checking the actual HTTP
status (`gh api -i`) rather than trusting exit code alone (Pitfall 1), and emitting
`upload_url`/`version` to `$GITHUB_OUTPUT` exactly as the current `get_version` step does
(lines 115–124, for the `version` output pattern to mirror).

---

### `.github/workflows/release.yml` — `publish-crates` / `publish_one()` (D-04/D-05/D-12)

**Analog:** the existing `publish_one()` function itself (lines 477–499) — kept as the
carrier per the researcher's D-06 verdict (native `cargo publish --workspace` rejected).

**Current defect to remove** (lines 480–494):
```bash
if [ "$DRY_RUN" = "true" ]; then
  cargo publish --dry-run -p "$crate"
else
  # Tolerate an already-published version so the job is re-runnable.
  if cargo publish -p "$crate" 2>&1 | tee /tmp/publish.log; then
    echo "${crate} published."
  elif grep -qiE "already (exists|uploaded)|is already uploaded|already published" /tmp/publish.log; then
    echo "::warning::${crate} version already published — continuing."
  else
    echo "::error::Failed to publish ${crate}."
    exit 1
  fi
  # Allow the crates.io index to update before publishing dependents.
  sleep 20
fi
```
Replace the `grep` branch with a `curl`-based registry pre-check (RESEARCH.md Pattern 3)
run *before* `cargo publish -p "$crate"` is attempted at all, and replace `sleep 20` with
the bounded index-visibility poll (RESEARCH.md Pattern 3 `wait_for_index_visibility` /
Code Example 4 sparse-index check). Keep the `CRATES=(...)` array (lines 464–476) and its
dependency-order comment verbatim — Phase 19 already reconciled it against `cargo metadata`.

**Outcome-table pattern to add** (RESEARCH.md Pattern 4) — mirrors this same job's existing
`::notice::`/`::warning::`/`::error::` GitHub Actions annotation style already used at lines
429, 487, 489 — extend that same annotation vocabulary for the per-crate outcome table and
the zero-`published-now` failure message.

**User-Agent convention** (required for every new `curl` call against crates.io, per D-04):
no local file currently sets this in `release.yml`; the convention comes from
`19-PUBLISH-EVIDENCE.md`'s registry-check calls (referenced, not reproduced verbatim here —
see RESEARCH.md Code Example 4 for the exact header shape: `-H "$UA"` with
`UA='User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)'`).

---

### `.github/workflows/release.yml` — new gate job (D-07/D-08)

**Analog:** the `test` job (lines 80–97) — the simplest existing example of a job that is
purely `needs: verify-tag-source` plus a couple of setup steps plus one shell invocation,
which is the shape the new gate job should follow (thin CI wrapper around a script, per
D-07's "must be runnable locally" requirement).

```yaml
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    needs: verify-tag-source
    permissions:
      contents: read
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Rust cache
        uses: Swatinem/rust-cache@v2
      - name: Run test suite
        run: cargo test --workspace
```
Mirror this shape for the new job: `checkout` (with `fetch-depth: 0` if the script needs
git history/tags — see `verify-tag-source`'s checkout at lines 35–38), then a single
`run: ./scripts/check-release-consistency.sh` step. Per research's Open Question 2 answer,
scope `needs:` on `publish-crates` only (add the new job to `publish-crates`'s existing
`needs: [test, create-release]` list at line 411 → `needs: [test, create-release,
check-release-consistency]`), not on `create-release` — preserving WR-05's existing
documented asymmetry.

---

### `scripts/check-release-consistency.sh` (new, D-07/D-08)

**Analog:** `scripts/check-workflow-triggers.sh` (472 lines) — the closest existing shape:
a bash wrapper that shells out to a `python3 -` heredoc, accumulates every violation into a
shared `failures` list (never fails fast), and prints a structured `STATUS_LINE` /
`DETAIL` report consumed by the bash wrapper.

**Header/usage-comment convention to copy** (lines 1–86): document what the script asserts,
in numbered clauses, before any code — this repo's house style for gate scripts.

**Bash wrapper skeleton to copy** (lines 88–100, 442–472):
```bash
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOWS_DIR="${1:-${WORKSPACE_ROOT}/.github/workflows}"
POLICY_TABLE="${2:-${WORKSPACE_ROOT}/docs/src/contributing/branching-model.md}"
RULESET_FILE="${3:-${WORKSPACE_ROOT}/.github/rulesets/protect-main-branch.json}"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for YAML/JSON parsing." >&2
    exit 1
fi

echo "🔍 Checking ..."

REPORT=$(python3 - "${WORKFLOWS_DIR}" "${POLICY_TABLE}" "${RULESET_FILE}" <<'PY'
# ... python body, first line printed is a status token, rest is detail ...
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ ... check failed (${STATUS_LINE})"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected: ..."
    exit 1
fi
```
Take positional-argument-with-default convention (real paths default, overridable for
tests) exactly as shown — this is what lets `tests/scripts/check-release-consistency_test.sh`
point the script at fixtures without mutating the real tree (see test analog below).

**Collect-then-report pattern to copy** (lines 115, 269, 316, 374, 431–440): a single
`failures = []` Python list, appended to by every clause, checked once at the very end —
never an early `sys.exit(1)` inside a clause. RESEARCH.md's Pattern 2 example (tag/manifest/
changelog/CI-conclusion accumulation) is the direct application of this same shape to D-08's
four checks.

**`cargo metadata` enumeration** — use RESEARCH.md Code Example 1
(`cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null) | "\(.name) \(.version)"'`)
rather than a hardcoded crate list, called from bash and piped into the python heredoc via
stdin/argv, matching how `check-changelogs.sh` (below) already parses `Cargo.toml` structurally
with `tomllib` instead of regex.

**Zero-crates / zero-files named-failure convention** (`check-workflow-triggers.sh` lines
127–132, and identically in `check-changelogs.sh` lines 58–62): a discovery step that finds
nothing must print a distinct `ZERO_*` status token and fail, never silently report `OK`
over an empty set. Apply this to the eleven-manifest enumeration and the ten-changelog scan.

**CI-conclusion sub-check** — use RESEARCH.md Code Example 2 (`gh api
repos/${GITHUB_REPOSITORY}/actions/workflows/ci.yml/runs -f head_sha=… -f status=completed
--paginate --jq '... | sort_by(.created_at) | last | .conclusion'`), sorting by `created_at`
descending per Pitfall 4, and treating the whole-run `conclusion` as the granularity per
Open Question 1's recommendation (whole-run success, not a job subset).

---

### `scripts/finalize-crate-changelogs.sh` (new, D-09 fallback) OR `Makefile` `release` target loop extension

**Analog:** `Makefile` `release` target's existing root-changelog finalize step (lines
563–565):
```makefile
	@echo "$(CYAN)Finalizing CHANGELOG.md...$(NC)"
	@DATE=$$(date +%Y-%m-%d); \
		perl -0pi -e "s/## \\[Unreleased\\]/## [Unreleased]\n\n## [$(VERSION)] - $$DATE/" CHANGELOG.md
```
Per RESEARCH.md Pitfall 5 / `release.toml`'s own comment (lines 33–38: "a root-level
[`pre-release-replacements`] would run once per crate and duplicate the heading"),
**do not** attempt a workspace-level `cargo-release` config for this. Extend this exact
`perl -0pi -e` one-liner into a loop over `crates/*/CHANGELOG.md`, either inline in the
`Makefile` target or factored into a standalone `scripts/finalize-crate-changelogs.sh`
that the `Makefile` target calls — same substitution regex, same `$(VERSION)`/`$$DATE`
variables, applied to each of the ten crate changelog paths instead of the root one.

**`release.toml` context** (do not modify unless adding a comment) — lines 33–38 already
document *why* this approach and not `pre-release-replacements` was chosen for the root
file; the same rationale extends to the ten crate files and should be referenced, not
re-litigated, in the new script's header comment.

---

### `tests/scripts/check-release-consistency_test.sh` (new)

**Analog:** `tests/scripts/check-workflow-triggers_test.sh` (290+ lines) — the established
regression-harness shape for every `scripts/check-*.sh` gate script in this repo.

**Fixture-lifecycle pattern to copy** (lines 1–38):
```bash
set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/check-workflow-triggers.sh"
# ... REAL_* path constants for reference-only reads ...

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/check-workflow-triggers-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0
```

**Fixture-builder helpers and assertion helpers to copy the shape of** (lines 50–120):
`mkdir_fixture`, small fixture-writer functions (`write_workflow`, `write_policy_table`
analogs → e.g. `write_cargo_metadata_fixture`, `write_changelog_fixture`), a `run_guard`
helper that captures `$LAST_OUTPUT`/`$LAST_STATUS`, and paired `assert_fire`/`assert_silent`
helpers that check both exit code AND that the output names the specific failing clause
(`grep -qF -- "${needle}"`) — never just "did it fail."

**Wiring:** add the new test file to `Makefile`'s `test-shell-guards` target invocation
list (see `Makefile` line 190–191 `test-shell-guards` target header — the actual body/list
of invoked test files should be read at implementation time to find the exact insertion
point, since it was not captured in this excerpt).

---

### `docs/src/appendix/release-recovery.md` (new, D-13)

**Analog:** `docs/src/appendix/release-checklist.md` (structure: numbered `## N. Section`
headings, terse bullet lists, cross-reference callout at the top) and
`docs/src/appendix/release-automation.md` (existing automation doc it must cross-link to).

**Cross-link callout pattern to copy** (`release-checklist.md` lines 1–8):
```markdown
# Release Checklist

This checklist defines the required release path from code freeze through publish and announcement.

> **Automation:** Most of this checklist is automated by `make release VERSION=x.y.z` and the
> tag-triggered `.github/workflows/release.yml` pipeline. See
> [RELEASE_AUTOMATION.md](release-automation.md) for the tooling decision (`cargo-release`) and the
> operator guide. This checklist remains the authoritative description of the end-to-end process and
> the manual verification steps.
```
Use the identical callout-blockquote convention in `release-recovery.md` linking back to
both `release-checklist.md` and `release-automation.md`, and add a matching callout to each
of those two files pointing forward at `release-recovery.md` (per D-13's "cross-linked from
both" requirement).

**Section structure to mirror** (`release-checklist.md`'s `## 1.`…`## 5.` numbered headings
with terse bullets under each) — apply the same terse, numbered, checklist-style prose to
the D-13 content list: how to establish which crates reached crates.io; the re-run-forward
recovery default; the yank-never-retry policy; who may yank; and the Yank register table.

**Yank register table shape** — no direct local analog exists for an owner+date table in
this doc set; follow the Phase 9/12 "owner+date convention" referenced in CONTEXT.md D-13
(columns: version, crates, reason, owner, date) — a plain Markdown table, consistent with
this doc set's existing use of Markdown tables elsewhere in `release-automation.md`.

---

### `Makefile` — new gate target + `release`/`release-check` extension

**Analog:** `check-changelogs` target (lines 163–165):
```makefile
.PHONY: check-changelogs
check-changelogs: ## Verify every publishable crate carries a CHANGELOG.md
	@./scripts/check-changelogs.sh
```
Add a `check-release-consistency` target with the identical one-line-wrapper shape, and
add it to the `check-gates` composite target (line 188):
```makefile
check-gates: check-changelogs check-crate-names check-advisory-register check-workflow-suppressions check-workflow-triggers check-codeql-dismissals ## Run all offline release-gate guards
```
→ append the new target name to this list (D-07's "runnable locally via a make target").

**`release-check` / `publish-dry-run` targets** (lines 498–521) — these are the existing
pre-publish validation chain (`release-check` → `clean-code` + `test` + `audit` +
`build-release`; `publish-dry-run` → per-crate `cargo publish --dry-run` loop, lines
509–521, itself an analog for the eleven-crate loop shape but using `|| true` rather than
outcome tracking — do not copy the `|| true` tolerance into the new `publish_one()`, it is
exactly the "hide failures" pattern this phase's D-12 replaces). Extend `release-check`
(or add a new step in `release`, lines 523–571) to invoke `scripts/finalize-crate-changelogs.sh`
alongside the existing root-`CHANGELOG.md` `perl` step (lines 563–565).

## Shared Patterns

### GitHub Actions tainted-value indirection (CR-01)
**Source:** `.github/workflows/release.yml` line 359–363 (`sbom` job's `RELEASE_VERSION:` env block), commented `CR-01`.
**Apply to:** Every new `run:` step in `create-release`, the new gate job, and `publish-crates` that touches a tag-derived version string or SHA — pass through `env:` rather than direct `${{ }}` interpolation inside `run:`.
```yaml
    env:
      RELEASE_VERSION: ${{ needs.create-release.outputs.version }}
```

### Offline gate script: collect-then-report + named zero-input failure
**Source:** `scripts/check-workflow-triggers.sh` (full file) and `scripts/check-changelogs.sh` (full file).
**Apply to:** `scripts/check-release-consistency.sh` — accumulate every mismatch into one list before reporting (never fail on the first); a discovery step that finds zero manifests/changelogs is itself a named failure (`ZERO_*` token), never a vacuous pass.

### GitHub Actions annotation vocabulary (`::notice::`/`::warning::`/`::error::`)
**Source:** `.github/workflows/release.yml` lines 429, 487, 489 (existing `publish-crates` job).
**Apply to:** The rewritten `publish_one()`'s registry pre-check, index-wait poll, and the new outcome-table zero-`published-now` failure message — keep using the same three-level annotation vocabulary already established in this job, do not introduce a new logging convention.

### Regression-test fixture lifecycle (`mktemp -d` + `trap cleanup EXIT`)
**Source:** `tests/scripts/check-workflow-triggers_test.sh` lines 21–120 (full pattern).
**Apply to:** `tests/scripts/check-release-consistency_test.sh` — single scratch dir, trapped cleanup, `assert_fire`/`assert_silent` helper pair that checks both exit code and that the output names the specific failing clause.

### crates.io API discipline: mandatory `User-Agent`, no redirect-following
**Source:** `19-PUBLISH-EVIDENCE.md` (Phase 19, referenced not reproduced) and `security.instructions.md`'s "HTTP clients sending a credential header do not follow redirects" control.
**Apply to:** Every new `curl` call in `publish_one()`'s registry pre-check/poll and in `check-release-consistency.sh`'s CI-conclusion resolution — always send `-H "User-Agent: ..."`; do not pass `-L`/`--location` unless a redirect is genuinely required and explicitly pinned (`--proto '=https' --location-trusted`).

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `.planning/phases/20-.../20-RECOVERY-EVIDENCE.md` | evidence record | N/A (manual/operational record, not code) | No code analog applicable — follow `19-PUBLISH-EVIDENCE.md`'s prose/table shape directly (already named as the reference in CONTEXT.md/RESEARCH.md); not a pattern-mapping target, just a doc to model after during execution. |

## Metadata

**Analog search scope:** `.github/workflows/`, `scripts/`, `tests/scripts/`, `docs/src/appendix/`, `Makefile`, `release.toml`
**Files scanned:** `release.yml` (499 lines, full read), `ci.yml` `publish-dry-run` job (referenced via RESEARCH.md, not re-read), `check-workflow-triggers.sh` (472 lines, full read), `check-changelogs.sh` (94 lines, full read), `check-workflow-triggers_test.sh` (first 120 lines read), `Makefile` (relevant target ranges read via grep + targeted read), `release-checklist.md` (first 40 lines read), `release.toml` (full read, 39 lines)
**Pattern extraction date:** 2026-08-28
