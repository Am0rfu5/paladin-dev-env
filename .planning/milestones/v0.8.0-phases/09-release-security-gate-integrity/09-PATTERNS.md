# Phase 9: Release & Security Gate Integrity - Pattern Map

**Mapped:** 2026-08-07
**Files analyzed:** ~24 (new + modified, per 09-CONTEXT.md / 09-RESEARCH.md)
**Analogs found:** 20 / 24 (4 have no analog — noted below)

**Framing note:** this phase touches zero `.rs` files. Every "closest analog" below is a shell
guard script, a Markdown/TOML config file, a Cargo manifest field, a Dockerfile block, or an ADR —
not a Rust module. Role/data-flow classification is adapted accordingly: role stays meaningful
(guard-script, governance-doc, manifest-field, build-stage, decision-record); "data flow" is
replaced with **check-flow** (what the artifact reads, and how/whether it can fail).

## File Classification

| New/Modified File | Role | Check-flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `scripts/check-advisory-register.sh` | guard-script | read-3-files→assert→exit-code | `scripts/check-api-surface.sh` | exact (failable guard) |
| `scripts/check-crate-names.sh` | guard-script | read-manifests→set-compare→exit-code | `scripts/check-api-surface.sh` | exact (failable guard) |
| `scripts/check-changelogs.sh` | guard-script | for-loop→file-exists→exit-code | `scripts/check-api-surface.sh` | exact (failable guard) |
| `SECURITY-EXCEPTIONS.md` | governance-doc (register) | human-readable + machine-parsed TOML block | `deny.toml` (comment-based governance) / `.cargo/audit.toml` | role-match (new register shape) |
| `.crate-names.txt` (or `deny.toml` bans list) | allow-list config | flat data file read by a guard | `.project/current-exports.txt` (baseline read by `check-api-surface.sh`) | role-match |
| `crates/paladin-herald/CHANGELOG.md` | governance-doc (changelog) | static Keep-a-Changelog file | `crates/paladin-core/CHANGELOG.md` (or any of the 9 siblings) | exact |
| `.planning/decisions/0024-*.md` … `0027-*.md` | decision-record (ADR) | static, 7-heading, no-frontmatter | `.planning/decisions/0023-cli-dependency-isolation.md` | exact |
| `deny.toml` (modified) | manifest-config (TOML) | 4 deletions + comment rewrite + pointer | itself (prior revision) | exact (self-analog) |
| `.cargo/audit.toml` (modified) | manifest-config (TOML) | comment/pointer updates only | itself (prior revision) | exact (self-analog) |
| `.github/workflows/ci.yml` (modified: delete `:466-482`, add ≤3 guard steps) | CI workflow | job/step deletion + new steps wired like existing guard steps | `ci.yml`'s own `api-surface:` job (`:140-190`) | exact |
| `Makefile` (modified: new targets wrapping guards) | build-tooling | thin `@./scripts/*.sh` wrapper target | `Makefile:149-157` (`check-doc-examples`, `check-doc-config`) | exact |
| `Dockerfile.chef` (modified: delete `:25-33`) | build-stage (Docker) | multi-stage COPY/RUN sequence | itself (prior revision) / upstream cargo-chef README pattern | exact (self + upstream) |
| Root `Cargo.toml` `license =` field (×1) | manifest-field | literal SPDX string edit | each of the ten `crates/*/Cargo.toml` `license =` lines (they are siblings, not an analog hierarchy) | exact (uniform sibling edit) |
| `crates/*/Cargo.toml` `license =` field (×10) | manifest-field | literal SPDX string edit | each other (uniform, no single "source" copy) | exact |
| `LICENSE` → `LICENSE-MIT` (rename) | license-text | verbatim retained | itself | exact |
| `LICENSE-APACHE` (new) | license-text | verbatim canonical text, no drafting | apache.org/licenses/LICENSE-2.0.txt (external, not repo) | no in-repo analog — copy verbatim from upstream |
| `README.md` (license badge + section) | governance-doc | inline text/badge edit | itself (prior revision) | exact (self-analog) |
| `CHANGELOG.md` (root, modified) | governance-doc (changelog) | new `[Unreleased]` entries | `crates/*/CHANGELOG.md` Keep-a-Changelog shape | role-match |
| `Dockerfile` (`LABEL ... licenses="MIT"` at `:93`, modified only if D-11 lands) | build-stage (Docker) | single LABEL string edit | itself (prior revision) | exact (self-analog) |
| `.planning/REQUIREMENTS.md` (checkbox flips + traceability rows) | governance-doc (ledger) | in-place amendment, dated | Phase 8's equivalent close-out edits (`08-09-SUMMARY.md`'s described pattern) | role-match |
| `.planning/codebase/CONCERNS.md` (advisory section correction) | governance-doc (map) | D-00c dated annotation, retain+supersede | Phase 7's `STRUCTURE.md` correction (cited in CONTEXT.md as precedent, not read here) | role-match (pattern named, not re-read) |
| `.planning/decisions/PROMOTION.md` (advance to 0028) | governance-doc (index) | append rows + bump counter | itself (prior revision, e.g. after ADR-0023 was added) | exact (self-analog) |

**No analog files (4):** `LICENSE-APACHE` (canonical external text, not a repo pattern to imitate),
and three governance-doc edits (`REQUIREMENTS.md`, `CONCERNS.md`, `PROMOTION.md` bump) whose
"analog" is a described prior-phase editing convention rather than a distinct file to Read from —
listed above with role-match against that convention, not a fresh code excerpt.

## Pattern Assignments

### `scripts/check-advisory-register.sh`, `scripts/check-crate-names.sh`, `scripts/check-changelogs.sh` (guard-script)

**Analog:** `scripts/check-api-surface.sh` (full file, 47 lines, already read in full above) —
this is the repo's model for a guard that **can genuinely fail**, unlike `check-deprecations.sh`.

**Shebang + strict mode + purpose comment** (`check-api-surface.sh:1-4`):
```bash
#!/bin/bash
# Check for API surface changes compared to baseline
# Used in CI to detect accidental API changes
set -euo pipefail
```
`check-doc-config.sh` uses `#!/usr/bin/env bash` instead of `#!/bin/bash` — either shebang is
attested in this repo; prefer `#!/usr/bin/env bash` for new scripts per `check-doc-config.sh`'s
more portable form, but match whichever convention the plan's other new scripts pick for
consistency across the three D-02/D-13/D-15 guards.

**Core pass/fail pattern — the failable idiom** (`check-api-surface.sh:31-46`):
```bash
if diff -u "$FILTERED_BASELINE" "$FILTERED_CURRENT" > /dev/null 2>&1; then
    echo "✅ API surface unchanged"
    rm -f "$TEMP_FILE" "$FILTERED_BASELINE" "$FILTERED_CURRENT"
    exit 0
else
    echo "❌ API surface has changed!"
    echo ""
    echo "Differences:"
    diff -u "$FILTERED_BASELINE" "$FILTERED_CURRENT" || true
    echo ""
    echo "If this change is intentional:"
    echo "  1. Review the changes carefully"
    echo "  2. Update CHANGELOG.md with breaking changes"
    echo "  3. Update the baseline: ./scripts/extract-public-api.sh $BASELINE"
    rm -f "$TEMP_FILE" "$FILTERED_BASELINE" "$FILTERED_CURRENT"
    exit 1
fi
```
This is the shape to copy: a real conditional with **both branches reachable and only one of them
`exit 0`**. This is the exact property `check-deprecations.sh` lacks (both its branches fall
through to `exit 0` per Phase 8 D-05) — **do not use `check-deprecations.sh` as the structural
template**, only as the cautionary counter-example. The one part of `check-deprecations.sh` that
*is* worth copying is its final, genuinely-failing check (`:52-57`, below), because it is this
repo's only other attested exit-1 path in a guard script:
```bash
# check-deprecations.sh:52-57 — the one real failure path in that script
if grep -rE "#\[deprecated\]" src/ crates/ --include="*.rs" | grep -v "since\|note"; then
    echo "❌ Found deprecation without 'since' or 'note' fields!"
    echo "   Use: #[deprecated(since = \"0.2.0\", note = \"Use XYZ instead\")]"
    exit 1
fi
```

**Missing-input guard pattern** (`check-api-surface.sh:15-19`):
```bash
if [ ! -f "$BASELINE" ]; then
    echo "⚠️  No baseline found at $BASELINE"
    echo "   Run: ./scripts/extract-public-api.sh $BASELINE"
    exit 1
fi
```
Copy this shape for `check-advisory-register.sh`'s missing-`SECURITY-EXCEPTIONS.md` case and
`check-changelogs.sh`'s missing-`crates/*/CHANGELOG.md` case.

**Python-for-parsing sub-pattern** — `python3` is **already used** in this repo's guard idiom, not
a new introduction. `scripts/check-doc-config.sh:22-32`:
```bash
if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for YAML validation." >&2
    exit 1
fi

echo "Validating fenced YAML blocks in ${DOCS_SRC} ..."

python3 - "${DOCS_SRC}" <<'PY'
import os
import re
import sys
...
PY
```
`scripts/check-doc-examples.sh:43` uses the identical `python3 - "$ARG" <<'PY' ... PY` heredoc
form. **So RESEARCH.md's `python3`+`tomllib` recommendation follows an established pattern in this
repo, it does not introduce one** — two of eight existing scripts (`check-doc-config.sh`,
`check-doc-examples.sh`) already shell a Python heredoc for structured parsing (YAML there, TOML
here). Copy the `command -v python3` presence-check + heredoc invocation shape verbatim; swap
`import yaml` for `import tomllib` and the YAML-parse body for TOML-table/array extraction per
RESEARCH.md §"Answering the Open Technical Questions" §2.

**Crate-liveness grep pattern** (already the exact command CONTEXT.md/RESEARCH.md verified with):
```bash
grep -c '^name = "<crate>"$' Cargo.lock
```
Use this literal form (anchored `^name = "..."$`) for D-02 clause 3 and any crate-liveness check —
it is proven safe against `Cargo.lock`'s Cargo-generated, never-hand-edited format.

**Wiring into CI** — copy the `api-surface:` job's step shape (`ci.yml:168-175`, full excerpt
below) for however many of the three guards become CI steps:
```yaml
      - name: Make scripts executable
        run: chmod +x scripts/*.sh

      - name: Check API surface changes
        run: ./scripts/check-api-surface.sh .project/current-exports.txt

      - name: Check deprecation warnings
        run: ./scripts/check-deprecations.sh
```
New steps follow this exact `- name: <Title Case description>` / `run: ./scripts/<name>.sh [args]`
shape, appended to whichever job they're scoped to (D-22 suggests next to `cargo-deny`, i.e. the
job at `ci.yml:81-109`, display name "License & Dependency Policy").

**Wiring into `Makefile`** — copy the thin-wrapper shape at `Makefile:149-157`:
```makefile
.PHONY: check-doc-examples
check-doc-examples: ## Compile doc examples (paladin-doc-examples crate) + syntax-scan inline rust blocks
	@./scripts/check-doc-examples.sh

.PHONY: check-doc-config
check-doc-config: ## Validate fenced YAML config snippets in docs/src parse correctly
	@./scripts/check-doc-config.sh
```
Each new guard gets a `.PHONY: <name>` + one-line `## ` help comment + `@./scripts/<name>.sh`
body, matching this exact shape — no `@echo` banner needed (contrast with `audit`/`deny`, which do
add an `@echo "$(CYAN)...$(NC)"` line; either sub-style is attested, `check-doc-*` is closer to a
mechanical guard and is the better model here).

---

### `SECURITY-EXCEPTIONS.md` (governance-doc / register)

**No direct analog file** — this is a new register type. The two "reusable assets" to lift content
from, not format from:

**`.cargo/audit.toml`'s existing per-advisory comment block** (full excerpt already captured
above under Read output) already carries 4 of D-06's 10 register fields (advisory ID, affected
crate, transitive path, why-not-fixable, revisit condition) in prose form — the register lifts
this text rather than re-deriving it. Example block to migrate (`.cargo/audit.toml:6-8`):
```
# RUSTSEC-2023-0071: RSA timing side-channel via rsa 0.9.x
#   rsa is a transitive dep of sqlx-mysql (dev/test dependency).
#   No fixed version is available upstream; revisit when sqlx upgrades rsa.
```

**`deny.toml`'s header/class-labelling style** (`deny.toml:1-8`) is the model for the register's
own explanatory header prose (point at the config files it governs, name the run command):
```toml
# cargo-deny configuration
# See: https://embarkstudios.github.io/cargo-deny/
#
# Enforces dependency license compliance, bans, duplicate-version detection, and
# advisory checks across the Paladin workspace dependency graph.
#
# Run locally with:  cargo deny check
# (or `make security` to run cargo-audit + cargo-deny together)
```

**Recommended concrete row schema** (from RESEARCH.md, synthesizing D-01/D-02/D-06/D-08/D-09/D-10 —
this is the shape the planner should have the guard script parse):
```toml
[[exception]]
id = "RUSTSEC-2023-0071"
class = "vulnerability"
crate = "rsa"
path = "rsa -> sqlx-mysql -> sqlx -> workspace crates"
why_present = "transitive dev/test dependency of sqlx-mysql"
why_not_fixable = "no upstream fix available in sqlx-mysql's rsa dependency"
owner = "DF3NDR"
review_date = "2026-12-31"
compensating_control = "dev/test-scoped only; not reachable in a release build"
revisit_condition = "sqlx-mysql upgrades its rsa dependency past the vulnerable range"
```
Per RESEARCH.md's recommendation, wrap this in `SECURITY-EXCEPTIONS.md` as prose + one fenced
` ```toml ` block (Option A), not a Markdown pipe-table — the guard extracts the fenced block and
parses with `tomllib`.

---

### `crates/paladin-herald/CHANGELOG.md` (new)

**Analog:** any of the nine sibling `crates/*/CHANGELOG.md` — they are structurally identical.
Full copy-target shape, using `paladin-core`'s as the literal template (already read in full
above):
```markdown
# Changelog

All notable changes to `paladin-herald` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project follows lockstep workspace versioning.

## [Unreleased]

### Added
- Crate-level release artifacts for Epic 4 API stabilization.
- <herald-specific line — creation by the 2026-06-04 facade-cleanup reconciliation>

### Changed
- Public API stability documentation aligned with crate-tier stability expectations.

### Fixed
- Crate metadata and README linkage validated for crates.io release preparation.
```
Per D-14, this must also record Phase 8's ADR-0023 `colored`/`comfy-table` feature-gating (a
real, user-visible default-API-surface shrink) — add a dedicated `### Changed` bullet naming that,
not just the generic Epic-4 boilerplate every sibling carries. All nine siblings show "a crate with
almost no history" already gets exactly this generic 4-bullet Unreleased block with no dated
released version section — herald's file should match that same "almost no history" shape, plus
the one substantive herald-specific bullet.

---

### `.planning/decisions/0024-*.md` … `0027-*.md` (ADRs)

**Analog:** `.planning/decisions/0023-cli-dependency-isolation.md` (most recent, full file read
above) and `0022-...md` (second most recent, full file read above) — both confirm the exact
required shape, **no frontmatter**, these seven headings in this order:
```markdown
# ADR-NNNN: <Title>

## Status

Accepted

**Date:** YYYY-MM-DD

## Context

<prose, with `file:line` citations for every factual claim, "re-verified this session" framing>

## Decision

<prose>

## Considered Options

- **<option>** (accepted) — <reason>
- **<option>** (rejected) — <reason>

## Code Locations

- `path/to/file:LINE` — <what's there and what changes>

## Code Conformance

must change

<which plan/phase executes it, and what specifically>

## Downstream Consumers

- <phase/plan> — <what it does with this ADR>
```
Both 0022 and 0023 use `## Status` → `Accepted` → bold `**Date:**` line → `## Context` → `## Decision`
→ `## Considered Options` (bullet list, each option tagged `(accepted)`/`(rejected)` with a reason)
→ `## Code Locations` (bulleted `file:line` list) → `## Code Conformance` (literal `must change` or
`no change needed` as first line, then a paragraph) → `## Downstream Consumers` (bulleted,
phase/plan-scoped). **ADR-0024 through 0027 must match this exactly** — same heading text, same
heading order, no additional headings, no YAML frontmatter block.

Note 0023 additionally embeds verbatim terminal output blocks under `## Code Locations` as its
"D-19 evidence bar" proof (the `cargo build` / `cargo tree` transcripts) — ADR-0024 (RustSec
governance) and ADR-0026 (name guard) should do the same with their own verbatim `grep -c`
transcripts, since D-19 requires exactly this evidence bar for this phase too.

---

### `deny.toml`, `.cargo/audit.toml` (modified in place)

**Analog:** themselves — these are edits to existing, already-read files, not new-pattern
adoptions. Concrete edit sites:
- `deny.toml`'s `[advisories] ignore` array — delete the four dead entries (`structopt`,
  `ansi_term`, `atty`, `proc-macro-error`) per D-04; rewrite the surrounding comment block to
  point at `SECURITY-EXCEPTIONS.md` per D-01, replacing prose duplication with a pointer.
- `.cargo/audit.toml`'s five-entry `ignore` array (header block already excerpted above) — no
  entries change (all five are live), only the comment header gains a pointer line to
  `SECURITY-EXCEPTIONS.md` (e.g. `# Full governance detail (owner, expiry, compensating control):
  see /SECURITY-EXCEPTIONS.md`).
- Keep the existing `[graph] all-features = true` (`deny.toml:15`) and the three-class labelling
  untouched (D-03 — do not re-open the sync-invariant finding).

---

### `.github/workflows/ci.yml` (modified: delete `:466-482`, add ≤3 guard steps)

**Analog:** the surviving `security-audit:` job (`:61-78`) as the target-shape reference for what
the deleted job's replacement responsibility folds into, and the `api-surface:` job (`:140-190`,
already excerpted above) as the template for adding new guard steps. Deletion target, verbatim:
```yaml
  security:
    name: Security Audit
    ...
        run: cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111
```
at `:466-482` — delete this whole job block (18 lines including its leading comment and blank-line
separator). The surviving job that keeps the required `"Security Audit"` status-check context is
`security-audit:` (`:61-78`):
```yaml
  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v5
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Install cargo-audit
        ...
      - name: Run cargo-audit (exceptions from .cargo/audit.toml)
        ...
```
(exact step names/commands at `:71-78` should be re-read at implementation time since this excerpt
truncates the install/run lines — the job boundaries and job-id/display-name pair are confirmed.)

---

### `Dockerfile.chef` (modified: delete `:25-33`)

**Analog:** upstream cargo-chef's own canonical README pattern (cited in RESEARCH.md, not a repo
file) plus the file's own surviving lines. Target shape after deletion — the researcher-confirmed
correct pattern:
```dockerfile
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY crates ./crates
COPY benches ./benches
RUN cargo chef prepare --recipe-path recipe.json
```
i.e. delete the nine per-manifest `COPY crates/paladin-*/Cargo.toml ...` lines at `:25-33` entirely
and leave the four surviving `COPY`/`RUN` lines exactly as they are today (`:24`, `:35`, `:36`,
`:37`, `:38`) — do not reorder them, the fix is subtraction only. Update the explanatory comment at
`:21-23` to state the corrected reasoning per RESEARCH.md §1 (isolation comes from `recipe.json`'s
manifest-only content + BuildKit's cross-stage `COPY --from`, not from planner-stage enumeration).

---

### Cargo manifests — `license =` field (root + 10 crates), `LICENSE*`, `README.md` (D-11/D-12, human-gated)

**Analog:** the eleven manifests are mutual siblings — no single "source" file, each gets the
identical literal edit:
```toml
license = "MIT OR Apache-2.0"
```
replacing `license = "MIT"` at root `Cargo.toml:40` and each `crates/*/Cargo.toml:6` or `:8`.
**Confirmed no `[workspace.package]` inheritance point exists** — `.workspace = true` is used for
`[workspace.dependencies]` entries (e.g. `async-trait = { workspace = true }`) but never for
`license`, so all eleven lines must be edited individually; there is no single-point shortcut.

`LICENSE` → `LICENSE-MIT` (verbatim rename, content unchanged) + new `LICENSE-APACHE` (verbatim
canonical Apache-2.0 text from apache.org, not drafted) is the Rust-ecosystem convention (`serde`,
`tokio`, `rand` — cited by RESEARCH.md, external, not an in-repo analog).

`README.md`'s badge (`:7`) and `## License` section (`:187-189`) need matching text/link-target
updates if `LICENSE` is renamed — read those exact lines before editing (not re-excerpted here,
already located precisely by RESEARCH.md).

`Dockerfile:93`'s `LABEL org.opencontainers.image.licenses="MIT"` is a **newly-found** additional
site (not in CONTEXT.md's original blast-radius list) — add to the plan's file list if D-11 lands.

---

## Shared Patterns

### Failable-guard idiom (the phase's central mechanism-not-assertion requirement)
**Source:** `scripts/check-api-surface.sh` (full file)
**Apply to:** `scripts/check-advisory-register.sh`, `scripts/check-crate-names.sh`,
`scripts/check-changelogs.sh`
```bash
set -euo pipefail
# ... build up PASS/FAIL evidence ...
if <condition-holds>; then
    echo "✅ <success message>"
    exit 0
else
    echo "❌ <failure message>"
    echo "<remediation guidance>"
    exit 1
fi
```
**Counter-example to avoid:** `scripts/check-deprecations.sh` — both its main branches fall
through to implicit `exit 0` (Phase 8 D-05's finding). Every new guard must be demonstrated failing
once before being wired into CI (RESEARCH.md's explicit negative-path test instruction).

### Python-heredoc structured parsing (already-attested, not novel)
**Source:** `scripts/check-doc-config.sh:22-32`, `scripts/check-doc-examples.sh:43`
**Apply to:** `scripts/check-advisory-register.sh` (TOML parsing of `deny.toml` /
`.cargo/audit.toml` / the register), `scripts/check-crate-names.sh` (TOML parsing of manifests)
```bash
if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for ... ." >&2
    exit 1
fi
python3 - "$ARG" <<'PY'
import tomllib, sys
...
PY
```

### CI guard-step wiring
**Source:** `.github/workflows/ci.yml:168-175` (the `api-surface:` job)
**Apply to:** all new guard steps, wherever they land (recommend beside `cargo-deny` job per D-22)
```yaml
      - name: <Guard Title>
        run: ./scripts/<guard-name>.sh [args]
```

### Makefile thin-wrapper target
**Source:** `Makefile:149-157`
**Apply to:** local invocation of all three new guards
```makefile
.PHONY: <target>
<target>: ## <one-line description>
	@./scripts/<name>.sh
```

### Keep-a-Changelog per-crate shape
**Source:** `crates/paladin-core/CHANGELOG.md` (and 8 other identical siblings)
**Apply to:** `crates/paladin-herald/CHANGELOG.md` (new), root `CHANGELOG.md` (new entries)

### ADR seven-heading, no-frontmatter shape
**Source:** `.planning/decisions/0023-cli-dependency-isolation.md`, `0022-*.md`
**Apply to:** `0024-*.md`, `0025-*.md`, `0026-*.md`, `0027-*.md`

## No Analog Found

| File | Role | Check-flow | Reason |
|---|---|---|---|
| `LICENSE-APACHE` | license-text | static verbatim copy | No Apache-2.0 text exists anywhere in this repo today; source verbatim from apache.org/licenses/LICENSE-2.0.txt, do not draft |
| `.planning/REQUIREMENTS.md` (SEC-01…05 checkbox/traceability edits) | governance-doc (ledger) | in-place dated amendment | No single file-analog; follow the D-00c/D-00d amend-in-place convention described in CONTEXT.md, sourced from Phase 7/8 precedent narratively, not from a re-read file in this pass |
| `.planning/codebase/CONCERNS.md` (advisory section correction) | governance-doc (map) | dated annotation, retain+supersede | Same as above — Phase 7's `STRUCTURE.md` correction is the named precedent but was not re-read in this pass; the convention (D-00c) is well-specified in CONTEXT.md itself |
| `.planning/decisions/PROMOTION.md` (advance next-free to 0028) | governance-doc (index) | append + counter bump | Mechanical, single-line-plus-rows edit; its own prior state (after ADR-0023 was added) is the pattern, not a separate file |

## Metadata

**Analog search scope:** `scripts/`, `.github/workflows/ci.yml`, `Makefile`, `crates/*/CHANGELOG.md`
(all 9 existing), `.planning/decisions/0022-*.md` and `0023-*.md`, `deny.toml`, `.cargo/audit.toml`,
`Dockerfile.chef`.
**Files scanned:** 8 scripts in `scripts/`, 9 crate changelogs, 2 ADRs (full read), `ci.yml` job
headers (full scan) + 2 jobs read in full, `Makefile` targets around `audit`/`deny`/`security` and
`check-doc-*`, `deny.toml` and `.cargo/audit.toml` headers.
**Pattern extraction date:** 2026-08-07
