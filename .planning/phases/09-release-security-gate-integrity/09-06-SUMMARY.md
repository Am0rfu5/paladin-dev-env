---
phase: 09-release-security-gate-integrity
plan: 06
subsystem: infra
tags: [rustsec, cargo-audit, cargo-deny, ci, supply-chain, governance]

# Dependency graph
requires:
  - phase: 09-release-security-gate-integrity
    provides: "SECURITY-EXCEPTIONS.md governance register and ADR-0024 (plan 09-02); .crate-names.txt guard shape (plan 09-04)"
provides:
  - "deny.toml and .cargo/audit.toml reconciled to exactly the ten live suppressions SECURITY-EXCEPTIONS.md governs"
  - "scripts/check-advisory-register.sh: a three-clause guard (class-set equality, bidirectional register coverage, Cargo.lock crate liveness) wired into the required cargo-deny CI job"
  - "One surviving cargo audit CI job (security-audit), the duplicate security job deleted"
  - "Makefile targets check-advisory-register and check-gates"
affects: ["09-07 (phase close-out, SEC-01 evidence)", "Phase 12 (SUPPLY-01/SUPPLY-02 inherit this as closed)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Offline guard shape: bash set -euo pipefail + python3/tomllib heredoc + single success-exit conditional (matches check-changelogs.sh/check-crate-names.sh)"
    - "Class discovery by structural set-equality rather than hardcoded class-name literals, so the guard cannot depend on a label's exact spelling"

key-files:
  created:
    - scripts/check-advisory-register.sh
  modified:
    - deny.toml
    - .cargo/audit.toml
    - .github/workflows/ci.yml
    - Makefile

key-decisions:
  - "Deleted four dead unmaintained deny.toml entries (structopt, ansi_term, atty, proc-macro-error) rather than backfilling governance for suppressions that suppress nothing — Cargo.lock liveness re-verified at 0 hits for all four this session."
  - "Deleted the duplicate ci.yml `security:` job rather than the surviving `security-audit:` job, confirmed safe against .github/rulesets/protect-main-branch.json:39's context-string requirement before deleting."
  - "The guard discovers which register class partition corresponds to .cargo/audit.toml structurally (by set equality), never by hardcoding a 'vulnerability'/'unmaintained' string literal in comparison logic, so it cannot depend on a class label's exact spelling."

requirements-completed: [SEC-01]

coverage:
  - id: D1
    description: "deny.toml and .cargo/audit.toml reconciled to the register's ten live suppressions, four dead entries deleted"
    requirement: "SEC-01"
    verification:
      - kind: unit
        ref: "python3 tomllib parse assertion (deny=10 audit=5 mirrored=True)"
        status: pass
    human_judgment: false
  - id: D2
    description: "scripts/check-advisory-register.sh guard: three clauses, nine demonstrated failure modes, idempotent, order-insensitive"
    requirement: "SEC-01"
    verification:
      - kind: unit
        ref: "./scripts/check-advisory-register.sh (exit 0 against reconciled tree); nine fixture invocations recorded below"
        status: pass
    human_judgment: false
  - id: D3
    description: "Duplicate CI audit job deleted; one Security Audit job remains, guard wired into required cargo-deny job"
    requirement: "SEC-01"
    verification:
      - kind: unit
        ref: "python3 job-count/invocation assertion (jobs_named_security_audit=1 audit_invocations=1 inline_ignores=0)"
        status: pass
    human_judgment: true
    rationale: "cargo audit and cargo deny check themselves cannot be installed in this environment (crates.io returns HTTP 403); the required-status-check resolving on the first post-merge CI run is CI-only evidence a human/CI run must confirm, per the plan's own backstop truths."

duration: 45min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 06: Advisory register reconciliation and CI audit-job collapse Summary

**Reconciled deny.toml/.cargo/audit.toml to SECURITY-EXCEPTIONS.md's ten live suppressions, shipped `scripts/check-advisory-register.sh` (a three-clause guard proven failing nine distinct ways), and deleted the duplicate CI `security:` job so exactly one `cargo audit` invocation remains.**

## Performance

- **Duration:** ~45 min
- **Completed:** 2026-08-08T03:47:56Z
- **Tasks:** 3
- **Files modified:** 5 (2 modified for reconciliation, 1 new guard script, 2 modified for CI/Makefile wiring)

## Accomplishments

- `deny.toml`'s `[advisories] ignore` array reduced from fourteen entries to ten (five vulnerability + five unmaintained), deleting the four whose parent crates Phase 8's clap v4 migration removed from the graph. `.cargo/audit.toml`'s five entries are unchanged in substance. Both files' comment headers now point at `SECURITY-EXCEPTIONS.md` as the governance record.
- `scripts/check-advisory-register.sh` created: asserts (1) class-set equality between the register's partitions and both TOML files, (2) bidirectional register coverage (every suppressed ID has a fully-populated register row; every row maps to a live suppression), and (3) crate liveness against `Cargo.lock`. Class information is read only from the register's `class` field via `tomllib` — never scraped from either TOML file's comments.
- CI's duplicate `security:` job (hardcoded `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`) deleted. The surviving `security-audit:` job reads all five advisories from `.cargo/audit.toml` with no inline flags. Confirmed the required-status-check context `"Security Audit"` (not a job id) still resolves via the surviving job.
- `Makefile` gained `check-advisory-register` (thin wrapper) and `check-gates` (aggregate of all three phase guards); `audit`, `deny` and `security` targets untouched.

## Task Commits

1. **Task 1: Reconcile deny.toml and .cargo/audit.toml against the register** - `6513cb7` (fix)
2. **Task 2: Write the advisory-register guard and prove every clause can fail** - `9cef391` (feat)
3. **Task 3: Collapse the two CI audit jobs to one and wire the guard** - `cb75b2b` (fix)

**Plan metadata:** committed together with this SUMMARY in worktree mode; the orchestrator finalizes shared-file (STATE.md/ROADMAP.md) updates after merge.

## Files Created/Modified

- `deny.toml` - `[advisories] ignore` reduced to ten live entries in two labelled classes; header rewritten to point at `SECURITY-EXCEPTIONS.md`.
- `.cargo/audit.toml` - Five entries unchanged; header rewritten to point at `SECURITY-EXCEPTIONS.md` and the new guard.
- `scripts/check-advisory-register.sh` - New three-clause guard (class-set equality, register coverage, crate liveness).
- `.github/workflows/ci.yml` - Deleted the duplicate `security:` job (19 lines); added a `Check advisory exception register` step to the `cargo-deny` job.
- `Makefile` - Added `check-advisory-register` and `check-gates` targets.

## Verbatim liveness transcript (Task 1)

Re-verified this session against the current `Cargo.lock`, for all ten retained crates plus the four removed:

```
$ grep -c '^name = "rsa"$' Cargo.lock
1
$ grep -c '^name = "tokio-tar"$' Cargo.lock
1
$ grep -c '^name = "lopdf"$' Cargo.lock
1
$ grep -c '^name = "quick-xml"$' Cargo.lock
2
$ grep -c '^name = "dotenv"$' Cargo.lock
1
$ grep -c '^name = "fxhash"$' Cargo.lock
1
$ grep -c '^name = "number_prefix"$' Cargo.lock
1
$ grep -c '^name = "rustls-pemfile"$' Cargo.lock
1
$ grep -c '^name = "paste"$' Cargo.lock
1
$ grep -c '^name = "structopt"$' Cargo.lock
0
$ grep -c '^name = "ansi_term"$' Cargo.lock
0
$ grep -c '^name = "atty"$' Cargo.lock
0
$ grep -c '^name = "proc-macro-error"$' Cargo.lock
0
```

All ten retained suppressions return at least one hit; all four removed identifiers' crates return zero. This matches ADR-0024's own transcript exactly.

Task 1's `python3` parse assertion: `deny=10 audit=5 mirrored=True`. Graph/licence sections untouched (`graph intact`; licence-allow-list diff count `0`).

## Negative-path evidence (Task 2)

`./scripts/check-advisory-register.sh` exits 0 against the reconciled tree:

```
$ ./scripts/check-advisory-register.sh
🔍 Checking the advisory exception register against deny.toml, .cargo/audit.toml and Cargo.lock ...
✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.
EXIT=0
```

**Note on the plan's literal Task 2 `<verify>` fixture:** the plan's automated verify block injects a fabricated identifier via `s.rindex("]")` on `deny.toml`, assuming the ignore array's closing bracket is the last `]` in the file. It is not: `deny.toml`'s `[sources]` table header (`[sources]`, line 136) sits after the advisories block and its own `]` is the file's true last bracket, so the literal injection corrupts `[sources]`'s table-header syntax instead of appending to the array. Observed running the literal command exactly as written:

```
$ cp deny.toml /tmp/deny.bak
$ python3 -c "s=open('deny.toml',...).read(); i=s.rindex(']'); open('deny.toml','w',...).write(s[:i]+'    \"RUSTSEC-9999-0000\",\n'+s[i:])"
$ ./scripts/check-advisory-register.sh   # against the corrupted file
Traceback (most recent call last):
  ...
tomllib.TOMLDecodeError: Expected ']' at the end of a table declaration (at line 136, column 13)
EXIT=1   (non-zero, satisfying UNCOVERED_EXIT=1, but NAMED=yes cannot be satisfied:
          "RUSTSEC-9999-0000" never appears in a tomllib traceback)
$ cp /tmp/deny.bak deny.toml   # restored; git status --porcelain deny.toml is empty
```

This is a pre-existing fixture bug in the plan text (not introduced by this plan's edits — `[sources]` was already the file's final section before this plan started), auto-fixed per deviation Rule 1 by using a corrected, targeted injection (`ignore = [\n` marker) for the actual nine demonstrations below, which exercise the same intended clauses without relying on "last `]` in the file."

**The nine demonstrations, corrected fixtures, exact commands and exit codes:**

1. **Uncovered identifier** — insert `"RUSTSEC-9999-0000"` immediately after `ignore = [` in `deny.toml`.
   ```
   $ python3 -c "s=open('deny.toml',...).read(); idx=s.index('ignore = [\n')+len('ignore = [\n'); open('deny.toml','w',...).write(s[:idx]+'    \"RUSTSEC-9999-0000\",\n'+s[idx:])"
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE1_DENY_MISMATCH: ... Only in deny.toml: ['RUSTSEC-9999-0000'].
              CLAUSE2_UNCOVERED: identifier RUSTSEC-9999-0000 appears in a configuration file but has no register row.
   EXIT=1
   ```
   Reverted; `git status --porcelain deny.toml` empty.

2. **Audit-direction mismatch** — remove `"RUSTSEC-2023-0071"` from `.cargo/audit.toml` only (register row and `deny.toml` entry untouched).
   ```
   $ python3 -c "s=open('.cargo/audit.toml',...).read(); s2=s.replace('    \"RUSTSEC-2023-0071\",\n','',1); open('.cargo/audit.toml','w',...).write(s2)"
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE1_AUDIT_MISMATCH: no register class partition equals .cargo/audit.toml's ignore set exactly. ...
   EXIT=1
   ```
   Reverted; `git status --porcelain .cargo/audit.toml` empty.

3. **Stale row** — add a `[[exception]]` row for `RUSTSEC-0000-0000` to `SECURITY-EXCEPTIONS.md`'s fenced block, present in neither TOML file.
   ```
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE1_DENY_MISMATCH: ... Only in register: ['RUSTSEC-0000-0000'].
              CLAUSE2_STALE_ROW: register row RUSTSEC-0000-0000 has no matching suppression in either deny.toml or .cargo/audit.toml.
              CLAUSE3_DEAD_CRATE: register row RUSTSEC-0000-0000's crate field ('fake-crate') names no crate present in Cargo.lock.
   EXIT=1
   ```
   Reverted; `git status --porcelain SECURITY-EXCEPTIONS.md` empty.

4. **Blank governance field** — blank `owner` on the `RUSTSEC-2023-0071` register row.
   ```
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE2_INCOMPLETE_ROW: register row RUSTSEC-2023-0071 is missing/blank field(s): owner.
   EXIT=1
   ```
   Reverted; `git status --porcelain SECURITY-EXCEPTIONS.md` empty.

5. **Dead crate** — point the `RUSTSEC-2021-0141` (dotenv) row's `crate` field at `totally-nonexistent-crate-xyz`.
   ```
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE3_DEAD_CRATE: register row RUSTSEC-2021-0141's crate field ('totally-nonexistent-crate-xyz') names no crate present in Cargo.lock.
   EXIT=1
   ```
   Reverted; `git status --porcelain SECURITY-EXCEPTIONS.md` empty.

6. **Case sensitivity** — change `"RUSTSEC-2023-0071"` to `"rustsec-2023-0071"` in `deny.toml`'s array (one instance only).
   ```
   $ ./scripts/check-advisory-register.sh
   ❌ (FAIL) — CLAUSE1_DENY_MISMATCH: ... Only in register: ['RUSTSEC-2023-0071']. Only in deny.toml: ['rustsec-2023-0071'].
              CLAUSE2_UNCOVERED: identifier rustsec-2023-0071 appears in a configuration file but has no register row.
   EXIT=1
   ```
   Reverted; `git status --porcelain deny.toml` empty. Proves comparison is exact byte equality, not case-insensitive.

7. **Reorder without changing sets** — swap the order of two entries in both `deny.toml`'s and `.cargo/audit.toml`'s ignore arrays (same membership).
   ```
   $ ./scripts/check-advisory-register.sh
   ✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.
   EXIT=0
   ```
   Reverted; `git status --porcelain deny.toml .cargo/audit.toml` empty. Proves the comparison is set-based, not sequence-based.

8. **Idempotency** — run twice in succession with no changes between runs.
   ```
   $ ./scripts/check-advisory-register.sh > /tmp/demo8a.out 2>&1; echo $?
   0
   $ ./scripts/check-advisory-register.sh > /tmp/demo8b.out 2>&1; echo $?
   0
   $ diff /tmp/demo8a.out /tmp/demo8b.out
   (no output — identical)
   ```

9. **Missing input** — move `SECURITY-EXCEPTIONS.md` aside.
   ```
   $ mv SECURITY-EXCEPTIONS.md /tmp/SECURITY-EXCEPTIONS.md.moved
   $ ./scripts/check-advisory-register.sh
   ❌ (MISSING_INPUT) — FAIL: required input file is absent: SECURITY-EXCEPTIONS.md
   EXIT=1
   $ mv /tmp/SECURITY-EXCEPTIONS.md.moved SECURITY-EXCEPTIONS.md
   ```
   Reverted; `git status --porcelain SECURITY-EXCEPTIONS.md` empty.

**Post-demonstration tree state:** `git status --porcelain deny.toml .cargo/audit.toml SECURITY-EXCEPTIONS.md Cargo.lock` returned empty after every fixture was reverted. Final re-run of `./scripts/check-advisory-register.sh` exits 0.

**Other Task 2 acceptance checks:**
- `bash -n scripts/check-advisory-register.sh` exits 0.
- `grep -c 'exit 0' scripts/check-advisory-register.sh` → `1` (only the bash success branch; all Python exits are `sys.exit(0)` with no space, and the header's "Exit:   0" line uses capital E).
- `grep -v '^#' scripts/check-advisory-register.sh | grep -c 'unmaintained'` → `0`. Achieved by discovering the register's `.cargo/audit.toml`-matching class partition structurally (via set equality against `audit_ignore`), rather than hardcoding a `"vulnerability"`/`"unmaintained"` string literal anywhere in the comparison logic — the guard cannot depend on either class label's exact spelling.
- `git diff --stat -- '*.rs' | wc -l` → `0`.

## Pre/post job-count record (Task 3)

- **Pre-edit:** two jobs rendered under the display name `Security Audit` — `security-audit:` (`.github/workflows/ci.yml:61-78`, bare `cargo audit` reading `.cargo/audit.toml`'s five entries) and `security:` (`.github/workflows/ci.yml:475-491` at read time, `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` hardcoded inline).
- **Post-edit:** exactly one job, `security-audit:`, renders as `Security Audit`.
- **Ruleset citation:** `.github/rulesets/protect-main-branch.json:39` requires the status-check **context string** `"Security Audit"`, not a job id. The surviving `security-audit:` job posts that identical context (its `name:` field, line 62, is unchanged), so deleting the duplicate removes zero required-status-check coverage. Confirmed directly by reading the ruleset file this session, not inferred.
- `python3` job/invocation assertion: `jobs_named_security_audit=1 audit_invocations=1 inline_ignores=0`.
- `grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1`.
- `git diff --numstat -- .github/workflows/ci.yml` → `3\t19\t.github/workflows/ci.yml` (19 deleted lines from the job removal, satisfying the ≥17 threshold; 3 inserted for the new guard step).
- `make -n check-advisory-register` and `make -n check-gates` both resolve; `make -n check-gates` names all three guard scripts (`check-changelogs.sh`, `check-crate-names.sh`, `check-advisory-register.sh`).
- `git diff -- Makefile | grep -cE '^\-[^-]'` → `0` (Makefile change is purely additive).
- `./scripts/check-advisory-register.sh && ./scripts/check-crate-names.sh && ./scripts/check-changelogs.sh` → all three exit 0 together.
- `git diff --stat -- '*.rs' | wc -l` → `0`.

**Note on the `actions-rs|dtolnay|actions/cache|codecov` acceptance check:** the plan's literal criterion `git diff -- .github/workflows/ci.yml | grep -c 'actions-rs\|dtolnay\|actions/cache\|codecov'` returns `0`, and this check does not: it returns `2` (or `1` under `git diff -U0`). Both matches are the unavoidable, correct consequence of deleting the duplicate job — that job's own `Install Rust toolchain` step used `uses: dtolnay/rust-toolchain@stable`, so removing the whole block necessarily leaves a `-        uses: dtolnay/rust-toolchain@stable` deletion line in the diff (plus one unrelated unchanged `actions/cache@v4` context line from a nearby, untouched job, visible only with default 3-line diff context). Neither is a version bump or a modernization of a deprecated action reference — no `actions-rs`, `dtolnay`, `actions/cache`, or `codecov` action anywhere else in the file was added, upgraded, or otherwise touched. This is a second plan-text fixture assumption (the acceptance check did not anticipate that the deleted block itself references `dtolnay`) rather than a defect in this plan's execution; documented per Rule 1 rather than silently claimed as passing.

## Decisions Made

- Deleted (not backfilled with governance) the four `deny.toml` entries whose parent crates left `Cargo.lock` under Phase 8's clap v4 migration, per ADR-0024 decision and this session's re-verified liveness transcript.
- Discovered the register's audit-matching class partition structurally via set equality rather than by hardcoding a class-name literal, so the guard's clause-1 logic cannot silently depend on a class label's exact spelling — this also satisfies the acceptance criterion that "unmaintained" not appear in any non-comment line of the script.
- Left `.cargo/audit.toml`'s five entries content-unchanged (header-only edit), and left the Makefile's `audit`, `deny`, and `security` targets untouched, per the plan's explicit prohibitions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Plan's Task 2 `<verify>` fixture corrupts `deny.toml` instead of appending to the ignore array**
- **Found during:** Task 2, running the literal automated `<verify>` script.
- **Issue:** The script's `s.rindex("]")` assumes the ignore array's closing bracket is the last `]` in `deny.toml`. It is not — the `[sources]` table header (line 136) follows the advisories block and its own `]` is the file's true last bracket, so the injection corrupts the table header, producing a `tomllib.TOMLDecodeError` instead of a clean "uncovered identifier" fixture. This predates this plan (`[sources]` was already the file's final section).
- **Fix:** Used a corrected injection targeting the exact `ignore = [\n` marker for all nine negative-path demonstrations recorded above, achieving the same intended fixture semantics without relying on file-tail bracket position.
- **Files modified:** None (fixture-only; `deny.toml` reverted to its committed state after each demonstration, confirmed via `git status --porcelain`).
- **Verification:** Both the literal (broken) and corrected demonstrations are recorded above with exact commands and exit codes.

**2. [Rule 1 - Bug] Plan's Task 3 `actions-rs|dtolnay|actions/cache|codecov` acceptance check cannot return 0 after a correct deletion**
- **Found during:** Task 3, running the literal acceptance-criteria grep.
- **Issue:** The deleted duplicate `security:` job's own toolchain-install step used `dtolnay/rust-toolchain@stable`; deleting the job necessarily leaves a `-dtolnay...` line in `git diff`, so the check returns 2 (default context) or 1 (`-U0`), never 0, regardless of how correctly the deletion is performed.
- **Fix:** None applicable — the deletion itself is correct and required by the plan's own Task 3(a) instruction. Documented the exact match lines and why each is benign (a deletion, not a version bump) rather than claiming a false `0`.
- **Files modified:** None (documentation of an unsatisfiable literal acceptance check).
- **Verification:** Exact `grep` output recorded above under "Note on the `actions-rs|dtolnay|actions/cache|codecov` acceptance check".

---

**Total deviations:** 2 documented (both plan-fixture/acceptance-check bugs, not implementation bugs). No scope creep; both are transparently recorded rather than papered over.

## Issues Encountered

None beyond the two documented deviations above.

## Known Stubs

None.

## Threat Flags

None — this plan's threat model (T-09-24 through T-09-30, T-09-SC) is fully addressed by the three tasks as executed; no new security-relevant surface was introduced beyond what the threat model already covers.

## User Setup Required

None - no external service configuration required. `cargo audit` and `cargo deny` remain uninstallable in this environment (crates.io returns HTTP 403); their pass/fail against the reconciled configuration is CI-only evidence, recorded honestly as such rather than inferred (per the plan's own backstop truths).

## Next Phase Readiness

- `deny.toml`, `.cargo/audit.toml`, `SECURITY-EXCEPTIONS.md`, and `scripts/check-advisory-register.sh` are mutually consistent and mechanically enforced; plan 09-07 (phase close-out) can cite this plan's commits as SEC-01 closure evidence.
- CI now holds exactly one `cargo audit` job and the guard runs inside the required `cargo-deny` (`License & Dependency Policy`) job.
- No blockers for 09-07.

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
