# Phase 12: Supply-Chain Gate Integrity - Pattern Map

**Mapped:** 2026-08-09
**Files analyzed:** 6 (governance/artefact files — this is a `.planning/`-only phase like Phases 10
and 11; **zero `.rs` files** are created or modified, confirmed by RESEARCH.md's own "Established
Patterns" section)
**Analogs found:** 6 / 6

> **Framing note:** This phase's deliverables are governance records, dated correction banners, and
> one bash guard script — not Rust code. There is no `crates/`/`src/` analog to search for and none
> is reported below; that omission is intentional, not a gap.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.planning/decisions/0036-<slug>.md` (NEW) | governance-record (ADR) | CRUD (create, append-only) | `.planning/decisions/0031-extracted-crate-dependency-rule.md` (shape) + `.planning/decisions/0024-rustsec-exception-governance.md` (cited sibling) | exact |
| `scripts/check-workflow-suppressions.sh` (NEW, or 4th clause in existing script — Claude's Discretion) | utility (CI guard / test) | batch (offline static scan) | `scripts/check-advisory-register.sh` (primary); `scripts/check-crate-names.sh`, `scripts/check-changelogs.sh` (siblings) | exact |
| `Makefile` (MODIFIED — new target + `check-gates` wiring) | config | batch | `Makefile:167-172` (`check-advisory-register` target + `check-gates` aggregate) | exact |
| `.github/workflows/ci.yml` (MODIFIED — new step under `cargo-deny:` job, ~line 101) | config | event-driven (CI trigger) | `ci.yml:96-101` (existing `check-advisory-register.sh` step) | exact |
| `.planning/REQUIREMENTS.md` (MODIFIED — 2 stale-caveat banners, 3 checkbox flips, 3 traceability rows, 1 new hand-off block) | governance-record | CRUD (in-place amend) | Prior Phase 9/10/11 correction banners + 3 existing `#### Hand-off to Phase N` blocks in the same file | exact |
| `.planning/PROJECT.md` (MODIFIED — 2 stale-text banners + 1 Key Decisions row) | governance-record | CRUD (in-place amend) | `.planning/codebase/CONCERNS.md`'s Phase 4/Phase 9 amendment banners (same corpus-wide convention) | role-match |
| `.planning/decisions/PROMOTION.md` (MODIFIED — numbering line, dated note, Part B candidate-7 closure note) | governance-record (index) | CRUD (in-place amend, updated LAST per procedure) | Same file's own Phase 9/10/11 dated notes (`PROMOTION.md:61-70`) and existing "Closed by ADR-00NN" notes on candidates 1/2/3/5 | exact (self-referential) |

## Pattern Assignments

### `.planning/decisions/0036-<slug>.md` (governance-record, NEW)

**Analogs:** `.planning/decisions/0031-extracted-crate-dependency-rule.md` (shape — a `conforms`
verdict ADR, the exact posture ADR-0036 needs) and `.planning/decisions/0024-rustsec-exception-governance.md`
(the sibling it must cite, never supersede or restate).

**Required H2 heading set, exact order** (`PROMOTION.md:107-128`, cross-checked against
`adr-parser.cjs:18-163`):
```
## Status
## Context
## Decision
## Considered Options
## Code Locations
## Code Conformance
## Downstream Consumers
```
`## Code Locations` and `## Considered Options` **must be bulleted lists** — `adr-parser.cjs`'s
`splitEntries` (`.claude/gsd-core/bin/lib/adr-parser.cjs:219-226`) splits by literal newline and
only strips a leading `-`/`*`/`+`. A single unwrapped prose line becomes one opaque blob; a
hand-wrapped multi-line paragraph becomes multiple ungrammatical per-line fragments. **Do not**
trail an un-bulleted verification-transcript paragraph after the `## Code Locations` bullet list
(ADR-0031's own minor wrinkle, ~18 spurious fragment entries) — if a transcript needs embedding,
put it inside `## Code Conformance` instead (an unmapped header, parser-safe for prose).

**`## Status` shape** (ADR-0031 and ADR-0024 both use this exact two-line form):
```markdown
## Status

Accepted

**Date:** 2026-08-08
```
`adr-parser.cjs`'s `STATUS_REJECT_SET = new Set(['superseded', 'rejected', 'deprecated'])`
(`adr-parser.cjs:17`) does an exact-match check after normalization — the status line must read
plainly `Accepted`.

**`## Code Conformance` shape for a `conforms` verdict** (copy ADR-0031's 3-line pattern):
```markdown
## Code Conformance

conforms

<one sentence explaining why the tree already satisfies the invariant>
```
Do **not** use ADR-0024's `must change` verdict — that ADR instructed real edits; ADR-0036 ratifies
an already-true state (D-03).

**`## Considered Options` shape** (ADR-0031's 4-bullet pattern, `(accepted)`/`(rejected)` suffix):
```markdown
## Considered Options

- Promote candidate 7 as a standalone ADR with a `conforms` verdict (accepted) — ...
- Decline and leave the invariant at PRD precedence (rejected) — ...
- Fold the invariant into ADR-0024 as an amendment (rejected) — ...
```

**`## Code Locations` shape** — cite `ci.yml:62-78`, `ci.yml:101`, `ci.yml:118`,
`.cargo/audit.toml`, `deny.toml`, `SECURITY-EXCEPTIONS.md`, plus the source PRD's full resolved
path (`.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md`
FR-1 + §8), each as its own bullet, per `PROMOTION.md` §Part A step 4.

**`## Downstream Consumers` shape** (ADR-0031's 3-bullet pattern) — name **Phase 13 / ORCH-01** and
**Phase 15 / PIPE-01** (per D-03's own instruction); do **not** duplicate ADR-0024's existing
"Phase 12 / SUPPLY-01 and SUPPLY-02" consumer entry.

**Relationship to ADR-0024 (D-05)** — cite it by number in `## Context`/`## Decision`, never
restate its suppression-content decisions. No `## Supersedes` line is needed (ADR-0036 supersedes
nothing). ADR-0024's own `## Status` is not touched.

**Structural self-check command** (run against the finished file before commit):
```bash
node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0036-<slug>.md \
  | python3 -c "
import json, sys
d = json.load(sys.stdin)
assert d['status'] == 'accepted'
assert len(d['key_files']) > 0
assert len(d['options_considered']) > 0
assert len(d['decisions']) > 0
unmapped = set(d['unmapped_headers'])
assert 'Code Conformance' in unmapped
assert 'Downstream Consumers' in unmapped
print('ADR-0036 STRUCTURAL CHECK: PASS')
"
```
(Verified sound this session against ADR-0031 — output `PASS`, `status=accepted`.)

---

### `scripts/check-workflow-suppressions.sh` (utility/guard, NEW — the D-08 regression guard)

**Analog:** `scripts/check-advisory-register.sh` (primary — richest, three-clause, offline,
accumulate-all-failures shape); `scripts/check-crate-names.sh` and `scripts/check-changelogs.sh`
(siblings, same house conventions).

**Shebang + strict mode + header comment** (`check-advisory-register.sh:1-49`):
```bash
#!/usr/bin/env bash
# check-advisory-register.sh
#
# Enforces that SECURITY-EXCEPTIONS.md (the governance register), deny.toml
# and .cargo/audit.toml ... agree ... Three clauses are asserted, and every
# failure found is reported rather than stopping at the first: ...
#
# Class information comes ONLY from the register's `class` field ... This
# script never scrapes ... comment wording to recover class information.
#
# This script only reads; it writes nothing and creates no temporary file.
#
# Usage:  ./scripts/check-advisory-register.sh
# Exit:   0 if the register and both suppression files agree on all clauses;
#         non-zero otherwise.

set -euo pipefail
```
The new guard's header should state the same three properties in its own terms: offline, structural
(PyYAML) not grep-scraped, accumulate-all-failures, and its exact clause: "fails if a
`cargo audit`/`cargo deny` invocation in any `.github/workflows/*.yml` file carries an inline
advisory-ignore flag."

**`WORKSPACE_ROOT` derivation** (shared convention across all three siblings):
```bash
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
```
D-08's guard should add a root-override hook not present in any of the three existing scripts,
purely for the positive-case scratch-copy test named in RESEARCH.md §D (Validation Architecture):
```bash
WORKFLOWS_DIR="${1:-${WORKSPACE_ROOT}/.github/workflows}"
```

**Status-line / symbol vocabulary** (shared across all three siblings — `check-crate-names.sh`,
`check-changelogs.sh`, `check-advisory-register.sh`):
- `🔍 Checking …` — emitted once at start.
- `✅ …` — single-line pass summary with a count.
- `❌ … failed` — each individual failure, one line per violation, never truncated to the first.

**Accumulate-rather-than-short-circuit contract** — every clause appends to a shared `failures`
list inside the `python3 - <<'PY' ... PY` heredoc; only after every clause runs does the script
decide pass/fail. Mirror this exactly for the new guard's single clause plus its "exactly one
`cargo audit` across all workflow files" sub-check (D-08(c)) — both checks accumulate into the same
list before the final verdict line is printed.

**Exit-code contract**: `0` if clean, non-zero otherwise — the *shell* wrapper reads Python's first
stdout line to decide `exit 1`/`exit 0` (`check-advisory-register.sh:254-272`), matching the other
two scripts.

**Matching design (false-positive-safe)** — verified standalone this session, `PASS`:
```python
import re
CARGO_GATE_RE = re.compile(r'\bcargo\s+(audit|deny)\b')
IGNORE_FLAG_RE = re.compile(r'--ignore(?:[= ]|$)')  # excludes --ignore-existing, --ignored

def violates(run_text):
    return bool(CARGO_GATE_RE.search(run_text) and IGNORE_FLAG_RE.search(run_text))
```
Co-occurrence on the **same `run:` step string** (parsed structurally via PyYAML, walking every
`jobs.*.steps[].run`) is the primary defense — `mc mb --ignore-existing` and
`cargo test -- --ignored` both fail the `CARGO_GATE_RE` half before the flag regex is even
relevant. Known false-positive tokens already present in the tree, must stay silent:
`ci.yml:428-429` (`mc mb ... --ignore-existing`), `ci.yml:463,466,755,757` (`cargo test ... --ignored`).

**Wiring points** (D-08, both required):
```makefile
.PHONY: check-advisory-register
check-advisory-register: ## Verify SECURITY-EXCEPTIONS.md agrees with deny.toml/.cargo/audit.toml/Cargo.lock
	@./scripts/check-advisory-register.sh

.PHONY: check-gates
check-gates: check-changelogs check-crate-names check-advisory-register ## Run all offline release-gate guards
```
`Makefile:167-172` — add a new `.PHONY: check-workflow-suppressions` target in the same shape
immediately below `check-advisory-register`, and add its target name to the `check-gates`
prerequisite list.

```yaml
      - name: Check advisory exception register
        run: ./scripts/check-advisory-register.sh
```
`.github/workflows/ci.yml:100-101` (inside the `cargo-deny:` / `License & Dependency Policy` job,
alongside the `check-changelogs`/`check-crate-names` steps at `:94-99`) — add a sibling `- name: ...`
/ `run: ./scripts/check-workflow-suppressions.sh` step immediately after it.

---

### `Makefile` (config, MODIFIED)

**Analog:** `Makefile:167-172` verbatim:
```makefile
.PHONY: check-advisory-register
check-advisory-register: ## Verify SECURITY-EXCEPTIONS.md agrees with deny.toml/.cargo/audit.toml/Cargo.lock
	@./scripts/check-advisory-register.sh

.PHONY: check-gates
check-gates: check-changelogs check-crate-names check-advisory-register ## Run all offline release-gate guards
```
New entry follows the identical two-line `.PHONY` + recipe shape (tab-indented `@./scripts/<name>.sh`,
one-line `##`-comment doc string matching the existing terse style), and `check-gates`'s
prerequisite list gains the new target name, appended after `check-advisory-register`.

---

### `.github/workflows/ci.yml` (config, MODIFIED)

**Analog:** `ci.yml:60-78` (the `security-audit:` job — model for the "no inline ignore, comment
states why" convention this whole phase is about):
```yaml
  # Security audit with controlled RustSec exceptions
  security-audit:
    name: Security Audit
    ...
      # Exceptions are the single source of truth in `.cargo/audit.toml` (see its
      # `[advisories].ignore` list). cargo-audit auto-discovers `.cargo/audit.toml`
      # from the repo, so no inline `--ignore` flags are used here.
      - name: Run cargo-audit (exceptions from .cargo/audit.toml)
        run: cargo audit
```
**Analog for the new guard's step**, `ci.yml:94-101` (inside the `cargo-deny:` job):
```yaml
      - name: Make scripts executable
        run: chmod +x scripts/*.sh

      - name: Check per-crate changelogs
        run: ./scripts/check-changelogs.sh

      - name: Check crates.io package names
        run: ./scripts/check-crate-names.sh

      - name: Check advisory exception register
        run: ./scripts/check-advisory-register.sh
```
Add a new `- name: Check workflow files for inline advisory suppressions` /
`run: ./scripts/check-workflow-suppressions.sh` step immediately after the last one, same
indentation, same terse imperative `name:` phrasing pattern.

---

### `.planning/REQUIREMENTS.md` (governance-record, MODIFIED)

**Analog for stale-caveat correction banners** — the shared corpus-wide inline-parenthetical
convention (see Shared Patterns below); apply to `REQUIREMENTS.md:1925-1927` (SUPPLY-02's
"CI-only" note — needs a "blocker lifted" banner) and **not** `:1871-1873` (SUPPLY-01's CI-run
clause — genuinely still pending per D-07, leave unbannered).

**Analog for the SUPPLY-03 "does not act" / "two candidates" correction** — same inline-parenthetical
convention, attached to `REQUIREMENTS.md:1929,1937-1939`.

**Analog for the new outbound hand-off block** — `REQUIREMENTS.md:1790-1826`
(`#### Hand-off to Phase 12 / SUPPLY-02 and SUPPLY-03`), the exact four-part shape to copy for the
new `#### Hand-off to Phase 13 / ORCH-01` block:
```markdown
#### Hand-off to Phase 12 / SUPPLY-02 and SUPPLY-03 — dated 2026-08-08 (plan 10-11)

**SUPPLY-02 and SUPPLY-03 inherit D-19's answer to the `pdf-extract` reachability question,
delivered rather than deferred.**

1. **The corrected `.cargo/audit.toml` reasoning, and what it now says:** ...
2. **The suppression set is unchanged.** ...
3. **Phase 12 inherits this as an answer, not a question** — ...
4. **A dead-dependency finding, named with an owner rather than left only in an ADR body:** ...

**Evidence:** `.planning/decisions/0032-pdf-extraction-capability.md` `## Decision`,
`## Downstream Consumers`; `.cargo/audit.toml:26-29`; `.planning/ledgers/milestone-07-08.md`'s
`REQ-content-processing-build-gate` row.
```
Four-part shape: (1) `#### Hand-off to Phase {N} / {REQ-ID(s)} — dated {date} (plan {NN-NN})`
heading; (2) bold one-sentence lead stating posture; (3) **numbered** list (not bulleted), each
item opening with a bold short label, citing exact `file:line`; (4) closing `**Evidence:**`
paragraph, semicolon-separated citation list, no extra prose.

**Analog for traceability-table row flips** — `REQUIREMENTS.md:4007-4010`:
```
| FACADE-04 | Phase 11 | Complete |
...
| SUPPLY-01 | Phase 12 | Pending |
| SUPPLY-02 | Phase 12 | Pending |
| SUPPLY-03 | Phase 12 | Pending |
```
Flip the three `Pending` cells to `Complete` — no other column touched, matching `FACADE-04`'s
already-closed row exactly.

---

### `.planning/PROJECT.md` (governance-record, MODIFIED)

**Analog:** the same inline-parenthetical banner convention (below), applied at
`PROJECT.md:587-588` (§Out of Scope bullet) and `PROJECT.md:625-627` (§Context "eleven candidates"
passage) — both need the banner; plus one new row in the `## Key Decisions` table
(`PROJECT.md:1103` onward) linking to `.planning/decisions/0036-<slug>.md`, per `PROMOTION.md` §Part A
step 6.

---

### `.planning/decisions/PROMOTION.md` (governance-record / index, MODIFIED — update LAST)

**Analog:** the file's own prior dated notes, `PROMOTION.md:59-70`:
```
**Next free ADR number: 0036**

*Dated note, 2026-08-08 (plan 11-05):* the line advances by **two** in one phase, from 0034 to
0036, because Phase 11 authored ADR-0034 (plan 11-02, the D1–D4 disposition set) and ADR-0035
(plan 11-03, the `paladin-ml` placement condition) across its own plans. Both carry `conforms`
verdicts, so unlike Phase 10 neither instructs a code change (D-13 — this phase changed zero
executable Rust). `ls .planning/decisions/0034-*.md .planning/decisions/0035-*.md` (re-run before
writing this note) confirms both files exist with contiguous numbers, none skipped or reused, and
no existing index row above was renumbered, reworded or reordered. **Neither ADR closes an entry
in Part B's eleven-candidate inventory below** — ...
```
New note follows the identical shape: `*Dated note, 2026-08-09 (plan NN-NN):* the line advances by
**one**, from 0035 to 0036, because Phase 12 authored ADR-0036 ...`, ending with the `ls
.planning/decisions/0036-*.md` re-verification command and an explicit statement that **this** ADR
**does** close Part B candidate 7 — unlike Phase 11's note, which closed none.

**Analog for the "Closed by ADR-00NN" note** — candidate 3's existing closure note
(`PROMOTION.md:185-189`, verbatim, quoted in RESEARCH.md §B.4):
```
3. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) … **Owner phase: Phase 9. Closed
2026-08-08 by ADR-0024** (`0024-rustsec-exception-governance.md`) — renewed to per-advisory
`2026-12-31` review dates, owner reassigned to `DF3NDR`.
```
Candidate 7's note should follow the identical shape: `**Owner phase: Phase 12. Closed
2026-08-09 by ADR-0036** (`0036-<slug>.md`) — <one clause naming the verdict>.`

---

## Shared Patterns

### Dated correction banner (D-00c "annotation, not rewriting")
**Source:** `.planning/codebase/CONCERNS.md:9` and `:276` (two verbatim examples, both read in full
this session).

Example 1 (`CONCERNS.md:276`, Phase 9):
```
"**Amended by Phase 9 (plan 09-07), dated 2026-08-08, citing
`.planning/decisions/0024-rustsec-exception-governance.md` (ADR-0024) and
`SECURITY-EXCEPTIONS.md`:** the '10 unmaintained crates' count above and the list itself are
stale. … The live unmaintained set today is five: `dotenv`, `fxhash`, `number_prefix`,
`rustls-pemfile`, `paste`. … the original list and migration plan above are retained verbatim, per
the amend-at-source convention."
```
Example 2 (`CONCERNS.md:9`, Phase 4):
```
"…this edition does not exist in Rust's stable channel… **(Amended by Phase 4,
dated 2026-08-03, citing `.planning/decisions/0009-workspace-rust-edition-2024.md`**: this claim
is factually wrong at this workspace's pinned toolchain. … this map's claim is superseded,
not the toolchain.)"
```
**Shape:** a **parenthetical clause appended directly inside/after the original sentence** — not a
separate paragraph, not a blockquote. Opens `(**Amended by Phase N (plan NN-NN), dated
YYYY-MM-DD, citing <ADR path>**: …)`, states the original claim's status explicitly
(superseded/void/corrected), gives the corrected fact in full, and **the original text is never
deleted**. Apply this identical shape to all correction sites in `REQUIREMENTS.md` and
`PROJECT.md` for this phase, e.g.:
`(**Corrected by Phase 12, dated 2026-08-09, citing ADR-0036 [and ADR-0024 for the candidate-count
correction]:** …)`.
**Apply to:** every stale-text site in §B of RESEARCH.md (`REQUIREMENTS.md:1929,1937-1939`,
`REQUIREMENTS.md:103-109`, `PROJECT.md:587-588`, `PROJECT.md:625-627`), plus the two "CI-only
blocker lifted" sites (`REQUIREMENTS.md:1925-1927`, `ROADMAP.md:772-776` — note ROADMAP.md is not
in this phase's primary edit list per CONTEXT.md's canonical refs but carries the same stale
"neither tool installable" clause; verify at execution time whether it is in scope).

### Offline guard-script conventions (bash)
**Source:** `scripts/check-advisory-register.sh` (full), `scripts/check-crate-names.sh`,
`scripts/check-changelogs.sh` (headers).
**Apply to:** `scripts/check-workflow-suppressions.sh`.
- `#!/usr/bin/env bash` + `set -euo pipefail`.
- `WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"`.
- `command -v python3` guard before use, named `ERROR:` message if absent.
- `🔍`/`✅`/`❌` status-line vocabulary; numbered remediation guidance on failure.
- Heavy logic in a `python3 - <<'PY' ... PY` heredoc; bash wrapper reads only Python's first stdout
  line to decide the shell's own exit code.
- Accumulate every failure into a list; never short-circuit on the first.
- Structural parsing over grep-scraping (`tomllib` for TOML, `PyYAML` for YAML) — "never scrape
  comment prose to recover class information" is the house rule this phase's guard must also obey.

### `make check-gates` / `ci.yml` guard wiring
**Source:** `Makefile:167-172`, `ci.yml:94-101`.
**Apply to:** the new `check-workflow-suppressions` target and its `ci.yml` step — both existing
sites are the exact two places a new guard must be registered; no other file needs touching for
wiring.

## No Analog Found

None — all six deliverable classes have a strong, verified analog in the corpus (this phase's
research explicitly notes every mechanism it needs already exists; see RESEARCH.md "Don't
Hand-Roll").

## Metadata

**Analog search scope:** `.planning/decisions/`, `.planning/codebase/CONCERNS.md`,
`.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`, `scripts/`, `Makefile`,
`.github/workflows/ci.yml`, `.claude/gsd-core/bin/lib/adr-parser.cjs`. No `crates/`/`src/` search
performed — out of scope per RESEARCH.md's own "Established Patterns" framing.
**Files scanned:** 6 primary analogs read in full or targeted-section this session (ADR-0031,
ADR-0024, `check-advisory-register.sh`, `PROMOTION.md`, `REQUIREMENTS.md` hand-off block,
`CONCERNS.md` banners), plus `Makefile`/`ci.yml` targeted line ranges.
**Pattern extraction date:** 2026-08-09
