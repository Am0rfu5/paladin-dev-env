---
phase: 12-supply-chain-gate-integrity
reviewed: 2026-08-09T00:00:00Z
depth: standard
files_reviewed: 3
files_reviewed_list:
  - scripts/check-workflow-suppressions.sh
  - Makefile
  - .github/workflows/ci.yml
findings:
  critical: 2
  warning: 4
  info: 2
  total: 8
status: issues_found
---

# Phase 12: Code Review Report

**Reviewed:** 2026-08-09
**Depth:** standard
**Files Reviewed:** 3
**Status:** issues_found

## Summary

`scripts/check-workflow-suppressions.sh` is the substantive artifact of this phase — an offline,
structural (PyYAML-based) guard that fails CI if any `.github/workflows/*.yml`/`*.yaml` file
passes an inline advisory-ignore flag to `cargo audit`/`cargo deny`, or if `cargo audit` is
invoked more than once across the corpus. The `Makefile` and `.github/workflows/ci.yml` changes
are minimal, correctly wired, and consistent with the three sibling guard scripts
(`check-advisory-register.sh`, `check-crate-names.sh`, `check-changelogs.sh`) — no findings
against those two files individually.

The script's stated fixtures (the `mc mb --ignore-existing` / `cargo test -- --ignored` false
positives, the backslash-continuation false negative, the empty-directory failure, determinism)
all check out under direct inspection and manual re-derivation of the regexes. However, the two
gate regexes (`CARGO_GATE_RE`/`CARGO_AUDIT_ONLY_RE`) and the suppression-flag regex
(`IGNORE_FLAG_RE`) are narrower than the space-separated, unquoted form they were tuned against.
I constructed three concrete, plausible command shapes that a future workflow author could write
— none contrived, none requiring adversarial intent — that produce a **complete, silent miss**:
the exact failure mode this guard exists to prevent (SUPPLY-01 reintroduced with CI staying
green). These are filed as CR-01 and CR-02 below. A third class of gap (missing PyYAML,
unhandled I/O errors, `${{ }}` expression indirection, composite-action scope) still fails
closed but degrades diagnostics or narrows the scan surface; filed as warnings.

## Critical Issues

### CR-01: Command-position variants of `cargo audit`/`cargo deny` bypass both clauses entirely

**File:** `scripts/check-workflow-suppressions.sh:94-100`
**Issue:**

```python
CARGO_GATE_RE = re.compile(r'\bcargo\s+(?:audit|deny)\b')
CARGO_AUDIT_ONLY_RE = re.compile(r'\bcargo\s+audit\b')
```

Both regexes require the literal token `cargo`, one or more whitespace characters, then
`audit`/`deny` with nothing else between them. Two realistic invocation forms don't match:

1. **Direct hyphenated binary invocation.** `cargo install cargo-audit --locked` installs a
   binary literally named `cargo-audit` on `PATH`; it is common (and documented as valid by
   `cargo-audit`/`cargo-deny` themselves) to invoke it directly, skipping the `cargo` dispatch
   prefix:
   ```yaml
   run: cargo-audit --ignore RUSTSEC-2024-0001
   ```
   Verified: `CARGO_GATE_RE.search(...)` → `False`. Neither clause fires — not clause 1
   (suppression not seen as co-occurring with a "gate"), and not clause 2 (this line never
   increments the audit-invocation counter either, so a *second*, differently-configured audit
   job added this way is invisible on both counts, reproducing SUPPLY-01's exact shape: a
   duplicate job with a baked-in `--ignore`).

2. **Toolchain-pinned invocation.** This repo already installs a nightly toolchain in another
   job (`ci.yml:169`) and uses `cargo +toolchain` idioms are common in Rust CI generally:
   ```yaml
   run: cargo +nightly audit --ignore RUSTSEC-2024-0001
   ```
   Verified: `CARGO_GATE_RE.search(...)` → `False` (the whitespace-only `\s+` between `cargo`
   and `audit` cannot span the `+nightly` token). Same silent-miss consequence as above.

Both were verified against the live regex objects, not inferred:
```
'cargo-audit --ignore RUSTSEC-2024-0001' -> gate=False audit_only=False
'cargo-deny check --ignore RUSTSEC-2024-0001' -> gate=False audit_only=False
'cargo +stable audit --ignore RUSTSEC-2024-0001' -> gate=False audit_only=False
'cargo +nightly deny check --ignore RUSTSEC-2024-0001' -> gate=False audit_only=False
```

**Fix:** Widen the gate to accept an optional `+toolchain` token and the hyphenated binary form,
while still excluding it when the line is actually a `cargo install` step (which currently relies
on the accidental absence of whitespace between `cargo` and `audit` — see the comment at
`scripts/check-workflow-suppressions.sh:96-99` — that reasoning breaks once the hyphenated form is
matched intentionally):

```python
INSTALL_LINE_RE = re.compile(r'\bcargo\s+install\b')
CARGO_GATE_RE = re.compile(
    r'\bcargo\s+(?:\+\S+\s+)?(?:audit|deny)\b'   # cargo [+toolchain] audit|deny
    r'|(?<!\w)cargo-(?:audit|deny)\b'             # direct binary invocation
)
CARGO_AUDIT_ONLY_RE = re.compile(
    r'\bcargo\s+(?:\+\S+\s+)?audit\b'
    r'|(?<!\w)cargo-audit\b'
)

def violates(line):
    if INSTALL_LINE_RE.search(line):
        return False
    return bool(CARGO_GATE_RE.search(line) and IGNORE_FLAG_RE.search(line))
```
Apply the same `INSTALL_LINE_RE` exclusion before incrementing `clause2_locations`. Add
`cargo-audit --ignore ...`, `cargo +nightly deny check --ignore ...`, and
`cargo install cargo-audit --locked` (must stay clean) as permanent regression fixtures.

---

### CR-02: `IGNORE_FLAG_RE`'s trailing-character set misses quoted and tab-separated flags

**File:** `scripts/check-workflow-suppressions.sh:107`
**Issue:**

```python
IGNORE_FLAG_RE = re.compile(r'--ignore(?:[= ]|$)')
```

This requires the character immediately after `--ignore` to be a literal space, `=`, or
end-of-string — intentionally excluding `--ignore-existing`/`--ignored`. But it also excludes two
plausible real forms, unlike CR-01 this one leaves `CARGO_GATE_RE`/`CARGO_AUDIT_ONLY_RE` intact
(clause 2's invocation counter still fires normally), so the failure mode is specifically clause 1
silently missing a real suppression while the overall run still reports `OK`:

```
'cargo audit "--ignore" RUSTSEC-2024-0001' -> False
"cargo audit '--ignore' RUSTSEC-2024-0001" -> False
'cargo audit --ignore\tRUSTSEC-2024-0001'   -> False   (tab instead of space)
```

A quoted flag token is a very ordinary defensive-quoting habit (and a `\t`-separated flag can
appear in a `run: |` block pasted from a tab-formatted source — YAML block scalars preserve tabs
that occur inside the content, only leading indentation is constrained). Either form lets
`cargo audit --ignore ...`/`cargo deny check --ignore ...` sail through clause 1 undetected while
clause 2 reports the expected single invocation, so the overall check prints `✅ ... no inline
advisory-ignore suppression detected` over a workflow file that has one.

**Fix:** Broaden the trailing-character class to cover any whitespace and an optional surrounding
quote:
```python
IGNORE_FLAG_RE = re.compile(r'''--ignore(?:[\s=]|['"]|$)''')
```
Add `cargo audit "--ignore" RUSTSEC-...` and a literal-tab variant to the regression fixtures
alongside `--ignore-existing`/`--ignored` (which must still stay excluded).

## Warnings

### WR-01: Unguarded `import yaml` — no fallback for missing PyYAML, unlike its sibling script

**File:** `scripts/check-workflow-suppressions.sh:87`
**Issue:** `python3` availability is checked explicitly (`scripts/check-workflow-suppressions.sh:74-77`),
but PyYAML is not — `import yaml` runs unguarded at the top of the heredoc. PyYAML is a
third-party pip package, not stdlib, and nothing in `ci.yml`'s `cargo-deny` job (or anywhere else
that invokes this script) installs it before running `./scripts/check-workflow-suppressions.sh`
(`.github/workflows/ci.yml:100-104`); the guard's correctness currently depends on the
`ubuntu-latest` runner image happening to ship PyYAML preinstalled in system Python. If it's
absent (any other OS, a slimmer image, or a future runner image change), `import yaml` raises an
uncaught `ModuleNotFoundError`. This still fails closed — I verified `set -euo pipefail` plus the
`REPORT=$(python3 ...)` assignment idiom does propagate the non-zero exit — but the CI log shows a
raw Python traceback instead of an actionable message.

The sibling script that shares this exact dependency already established the correct pattern in
this repo:
```python
# scripts/check-doc-config.sh:33-36
try:
    import yaml
except ImportError:
    print("ERROR: PyYAML is required (pip install pyyaml).", file=sys.stderr)
    sys.exit(2)
```
**Fix:** Apply the same guard here, before any other heredoc logic:
```python
try:
    import yaml
except ImportError:
    print("ERROR: PyYAML is required (pip install pyyaml).", file=sys.stderr)
    sys.exit(2)
```

---

### WR-02: Unhandled I/O/decode exceptions inside the per-file loop escape the narrow `except yaml.YAMLError`

**File:** `scripts/check-workflow-suppressions.sh:147-151`
**Issue:**
```python
try:
    with open(path, 'r', encoding='utf-8') as fh:
        data = yaml.safe_load(fh)
except yaml.YAMLError as exc:
    failures.append(f'PARSE_ERROR: {path} did not parse as YAML: {exc}')
    continue
```
A non-UTF-8 workflow file raises `UnicodeDecodeError` (a `ValueError` subclass, not
`yaml.YAMLError`); a permission-denied or broken-symlink file raises `PermissionError`/
`FileNotFoundError` (`OSError` subclasses). None of these are caught here, so they propagate
as an unhandled Python exception — again fails closed via the same `set -e` mechanism as WR-01,
but contradicts the header's own stated contract: "A workflow file that fails to parse as YAML is
also a named failure, never a silent skip" (`scripts/check-workflow-suppressions.sh:56-57`) reads
as a blanket guarantee about any file the script can't process, and it isn't one for this class of
error.
**Fix:** Widen the except clause and report these as a distinct named failure:
```python
except (yaml.YAMLError, OSError, UnicodeDecodeError) as exc:
    failures.append(f'READ_ERROR: {path} could not be read/parsed: {exc}')
    continue
```

---

### WR-03: `${{ }}` expression indirection is an undocumented, unaddressed blind spot

**File:** `scripts/check-workflow-suppressions.sh` (design-level, no single line)
**Issue:** The guard scans literal YAML-decoded text. GitHub Actions expressions
(`${{ vars.AUDIT_EXTRA_ARGS }}`, `${{ matrix.audit_flags }}`, `${{ inputs.ignore_id }}`) are
resolved at workflow-run time, not by this script — a suppression assembled this way never
appears as the literal substring `--ignore` in the YAML source the script reads, e.g.:
```yaml
run: cargo audit ${{ vars.AUDIT_EXTRA_ARGS }}   # AUDIT_EXTRA_ARGS = "--ignore RUSTSEC-2024-0001"
```
would pass clause 1 cleanly. This is an inherent limitation of static-text scanning and may be
acceptable, but nothing in the script's header (which otherwise enumerates its threat model in
detail, `scripts/check-workflow-suppressions.sh:24-57`) documents it as a known, accepted gap.
**Fix:** At minimum, document this as an explicit out-of-scope limitation in the header comment.
Consider also flagging (as a distinct, lower-confidence "needs manual review" finding rather than
a hard failure) any `cargo audit`/`cargo deny` logical line that contains a `${{ ... }}`
expression, since that's exactly the shape a suppression-via-indirection would take.

---

### WR-04: Scan surface is `.github/workflows/*.yml`/`*.yaml` only — composite actions are invisible

**File:** `scripts/check-workflow-suppressions.sh:71-72, 128-131`
**Issue:** The script only walks `jobs.*.steps[].run` inside files directly under the workflows
directory. A composite action (`.github/actions/<name>/action.yml`, with its own
`runs.steps[].run`) invoked via `uses: ./.github/actions/<name>` from a workflow file is
structurally identical in risk — a `cargo audit --ignore ...` line inside one would never be
scanned. No composite action exists in this repo today (verified: no `action.yml`/`action.yaml`
files under `.github/`), so this is not an active false negative, but it's exactly the kind of
scope narrowing that would let a future refactor (extracting the audit/deny steps into a reusable
composite action — a natural next step for a codebase already leaning on `actions/cache@v4` and
`dtolnay/rust-toolchain@stable` extensively) silently escape detection while CI stays green.
**Fix:** Either extend the glob to also scan `.github/actions/**/action.yml`/`action.yaml` under
their `runs.steps[].run` key, or explicitly document the workflows-only scope as an accepted
limitation in the header so a future reviewer knows to re-derive this guard's coverage if that
refactor happens.

## Info

### IN-01: No persisted regression test for a security-critical guard

**File:** `scripts/check-workflow-suppressions.sh`
**Issue:** The header comments make several specific, testable claims ("pinned by a test: a
differently-cased planted line ... must NOT be flagged", `scripts/check-workflow-suppressions.sh:48-50`).
Per `12-02-SUMMARY.md`, verification was performed with ephemeral `mktemp -d` scratch copies
during plan execution and then deleted — there is no committed test file in the repo that
re-runs these fixtures on every future change to the regexes (consistent with the other three
sibling guard scripts, none of which have committed tests either, so this isn't a regression
specific to this phase — but it is why CR-01/CR-02 above were able to ship without a test catching
them). **Fix:** Not blocking, but worth a follow-up: a small `tests/scripts/check-workflow-suppressions_test.sh`
(or similar) that builds the known-good/known-bad fixture set into a scratch dir and asserts exit
codes would catch exactly the class of regression found in this review before it reaches `main`.

### IN-02: New guard step lives in the "License & Dependency Policy" job rather than "Security Audit"

**File:** `.github/workflows/ci.yml:100-104`
**Issue:** `check-workflow-suppressions.sh` checks both `cargo audit` and `cargo deny` usages, but
the step was added to the `cargo-deny` job (`License & Dependency Policy`) rather than the
`security-audit` job. Not a functional problem — the `cargo-deny` job already hosts the other
offline `check-*.sh` guards (`check-changelogs.sh`, `check-crate-names.sh`,
`check-advisory-register.sh`) and this placement is consistent with that existing grouping — but
the job's own name doesn't fully describe what it's guarding as of this change. Purely cosmetic.

---

_Reviewed: 2026-08-09_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
