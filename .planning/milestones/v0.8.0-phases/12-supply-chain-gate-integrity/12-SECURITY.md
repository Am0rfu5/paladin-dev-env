---
phase: 12
slug: supply-chain-gate-integrity
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-08-10
---

# Phase 12 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

This phase changed **zero `.rs` files**. Its threat model is therefore not about application-level
ASVS controls — it is about **gate integrity**: whether the supply-chain checks this repository runs
on every push can be trusted to fail when they should. The recurring threat classes are a suppression
silently re-entering CI while the gate stays green, a guard that stops firing without anyone noticing,
the governance register drifting from the two TOML surfaces it mirrors, a closure claim asserted from
a context file rather than a re-run command, and stale `file:line` citations sending a future phase to
the wrong place.

**The phase's own deliverable is a security control.** `scripts/check-workflow-suppressions.sh`
enforces ADR-0036's invariant that advisory suppressions live only in `.cargo/audit.toml` and
`deny.toml`, never inline in a workflow. Every finding below that concerns "the guard" concerns that
control.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| a live gate command → a written closure claim | The moment a transcript becomes prose is where a false pass enters; a claim not backed by output run in this execution is unfalsifiable later | Command exit codes and stdout transcripts |
| a workflow `run:` string → the advisory suppression set | Any inline `--ignore` reaching `cargo audit`/`cargo deny` silently widens the suppressed set outside the two governed TOML surfaces | Advisory identifiers (`RUSTSEC-*`) |
| a stale citation → a governing document a future phase plans against | `PROJECT.md` checkboxes and `STATE.md` narrative are read as live work queues; a stale entry causes completed work to be planned twice | `file:line` references, commit SHAs |
| this phase → live GitHub repository administration state | `gh api` both reads and writes; the rulesets finding sits one flag away from an unauthorised outward-facing change | Repository ruleset / branch-protection configuration |
| `.planning/` records → the supply-chain gate configuration | `deny.toml`, `.cargo/audit.toml` and `SECURITY-EXCEPTIONS.md` belong to ADR-0024 and are read-only inputs here | Suppression entries, owners, review dates |

---

## Threat Register

37 threats were authored at plan time across all four PLAN.md `<threat_model>` blocks
(`register_authored_at_plan_time: true`). Audited 2026-08-10 by `gsd-security-auditor` at ASVS L1,
block threshold `high`, deepened to L2/L3-equivalent for the code-review cross-check.

**Distribution:** 18 high/mitigate · 8 medium/mitigate · 11 low/accept.
**Outcome: 37 closed, 0 open.**

Rather than reproduce all 37 rows, the register below records the audit's material findings. The full
per-threat evidence table with `file:line` citations is preserved in the audit return recorded in the
Audit Trail. Every threat not listed here was verified CLOSED with direct evidence.

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-12-01 | Spoofing | SUPPLY-01/02 closure records | high | mitigate | Transcripts re-run in-execution, not copied from context files; `REQUIREMENTS.md:1897,2001` | closed |
| T-12-02 | Spoofing | SUPPLY-01 CI-run clause | high | mitigate | Recorded `pending` with boundary run `30861568499`; since satisfied by a genuine post-deletion run `31320378772` (2026-08-09) | closed |
| T-12-03 | Tampering | `deny.toml`, `.cargo/audit.toml`, `SECURITY-EXCEPTIONS.md` | high | mitigate | Read-only inputs; last touch predates Phase 12 entirely (`a1559f3`, `6513cb7`) | closed |
| T-12-05 | Elevation of Privilege | live repository administration | high | mitigate | Only read-only `gh api` calls; no ruleset applied by this phase | closed |
| T-12-09 | Tampering | the D-08 guard's detection completeness | high | mitigate | **Reopened by audit, then closed.** See "Resolved During Audit" below | closed |
| T-12-10 | Spoofing | guard degenerate-input handling | high | mitigate | `check-workflow-suppressions.sh` — `ZERO_FILES` and `PARSE_ERROR` named failures; pass summary states file/step/invocation counts | closed |
| T-12-11 | Spoofing | guard line-joining | high | mitigate | `logical_lines()` joins backslash continuations before matching | closed |
| T-12-12 | DoS | guard false-positive contract | high | mitigate | Same-logical-line co-occurrence in `violates()`; negative test against the real tree carrying both known false-positive tokens | closed |
| T-12-17 | Spoofing | ADR-0036 citations | high | mitigate | Every `ci.yml` citation re-derived against the post-12-02 tree and resolves exactly | closed |
| T-12-18 | Tampering | ADR-0024 | high | mitigate | Untouched since `7ee741c`; ADR-0036 cites without superseding, carries no `## Supersedes` | closed |
| T-12-25 | Tampering | `PROMOTION.md` numbering index | high | mitigate | 36 rows, ascending, 36 unique; next-free advanced to 0037 | closed |
| T-12-26 | Elevation of Privilege | ADR number allocation ordering | high | mitigate | ADR-0036 written in 12-03 (`931fc29`) before the numbering advance in 12-04 (`140b5c4`) | closed |
| T-12-27 | Tampering | `PROMOTION.md` single-writer | high | mitigate | Only plan 12-04 commits touch it; `depends_on` chain enforces the sequential wave | closed |
| T-12-29 | Elevation of Privilege | Phase 13 ledger scope | high | mitigate | `.planning/ledgers/` holds exactly 4 pre-existing files; no M9-12 ledger created | closed |

*Status: open · closed · open — below `high` threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above `workflow.security_block_on` count toward `threats_open`*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Resolved During Audit

**T-12-09 — expression-indirection bypass in the phase's own guard. Found OPEN, fixed, re-verified CLOSED.**

The audit declined to accept the code review's "Warning, not blocking" framing of finding WR-03 and
mapped it onto T-12-09 (high/mitigate). Reproduced by direct execution against the then-committed
script:

```
run: cargo audit ${{ vars.AUDIT_EXTRA_ARGS }}   → exit 0, "✅ no inline advisory-ignore suppression detected"
run: cargo audit --ignore RUSTSEC-2024-0001     → exit 1  (control)
```

The guard matched only the literal YAML-decoded `run:` string, so a suppression carried in a
repository or organization variable never appeared as the substring `--ignore` and the gate certified
a workflow that was suppressing advisories — the precise failure class SUPPLY-01 existed to close.
`grep -c '\${{'` over the script returned `0`: no handling, and no documented limitation either.

**Fixed** in commit `4a5e484`. A third clause, `CLAUSE3_UNRESOLVABLE_EXPRESSION`, now fails any
logical line that invokes `cargo audit` / `cargo deny` (in any form clause 1 recognises — spaced,
hyphenated, `+toolchain`) **and** contains a `${{ … }}` expression. It reuses
`strip_install_segments()`, so a chained `cargo install … && cargo audit ${{ … }}` is still caught.
The failure is deliberately distinct from clause 1: "found a suppression" and "cannot prove there
isn't one" are different verdicts and should not be conflated.

Re-verified independently after the fix:

| Case | Expected | Actual |
|---|---|---|
| `cargo audit ${{ vars.X }}` | fail | ✓ exit 1, `CLAUSE3_UNRESOLVABLE_EXPRESSION` |
| Unrelated `${{ github.sha }}` on a non-gate line | silent | ✓ exit 0 |
| Real unmodified `.github/workflows/` tree | silent | ✓ exit 0 |
| `make check-gates` | pass | ✓ exit 0 |

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-12-01 | WR-01 / WR-02 (code review) | The guard's `import yaml` is unguarded and its `except yaml.YAMLError` does not catch `UnicodeDecodeError` / `OSError`. Both fail **closed** — an uncaught exception propagates through `set -euo pipefail` and the `REPORT=$(...)` assignment, forcing a non-zero exit. They degrade CI diagnostics to a raw traceback; they do not create a path where CI reports green over an undetected suppression. Below the `high` block threshold. | DF3NDR | 2026-08-10 |
| AR-12-02 | WR-04 (code review) | Composite-action `run:` steps sit outside the scanned glob. No `action.yml` / `action.yaml` exists anywhere under `.github/` today, so this is a forward-scope boundary rather than an active bypass. Re-derive if a future refactor extracts the audit/deny steps into a composite action. | DF3NDR | 2026-08-10 |
| AR-12-03 | D-10 (CONTEXT.md) | The committed rulesets `.github/rulesets/protect-main-branch.json` and `protect-release-tags.json` are version-controlled but **not applied** — `gh api …/rulesets` returns `[]` and `main` returns `404 Branch not protected`. The `Security Audit` context therefore runs and reports on every push but nothing *requires* it. Applying a ruleset is an outward-facing change to live repository administration, explicitly out of this phase's scope. **The owner decided APPLY on 2026-08-10** (recorded in `12-UAT.md` test 2); execution is outstanding and blocked on a token carrying repository-administration scope. Tracked to the milestone close-out, not to this phase. | DF3NDR | 2026-08-10 |

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-10 | 37 | 36 | 1 | `gsd-security-auditor` (ASVS L1, block_on `high`) — T-12-09 found OPEN via WR-03 |
| 2026-08-10 | 37 | 37 | 0 | orchestrator — T-12-09 fixed (`4a5e484`), IN-01 closed (`97f42b6`), both re-verified by execution |

**Prior gates feeding this audit:** `12-VERIFICATION.md` (8/9 truths verified independently, 1
`backstop` truth correctly abstained); `12-REVIEW.md` (2 Critical found and fixed in `530227d` /
`5f01c76` — the hyphenated `cargo-audit` and chained-install bypasses; 4 Warning + 2 Info recorded);
`12-UAT.md` (2/2 passed, including live CI run `31320378772`).

**Unregistered flag closed:** IN-01 — the guard shipped with no committed test, so the two Critical
bypasses it carried were caught only by one-time manual review using ephemeral fixtures. A persisted
regression harness now exists at `tests/scripts/check-workflow-suppressions_test.sh` (21 assertions
covering the CR-01 / CR-02 / WR-03 regression classes with clause-name pinning, the false-positive
contract, idempotency, and a no-mutation check), wired to `make test-shell-guards`. Deliberately
**not** added to `check-gates` — that target is the production gate chain, not a test runner.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
