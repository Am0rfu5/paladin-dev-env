---
phase: 12-supply-chain-gate-integrity
verified: 2026-08-09T14:32:09Z
status: human_needed
score: 8/9 must-haves verified
behavior_unverified: 0
overrides_applied: 0
human_verification:
  - test: "Confirm the required-status-check clause resolves on the first real CI run against release/v0.7.0 after Phase 9's 2026-08-08 deletion (commit cb75b2b)."
    expected: "A `gh run view <run-id>` citation, for a run whose creation timestamp postdates 2026-08-08, shows the `Security Audit` context reporting `success` and the ruleset (once applied) resolving against it."
    why_human: "This is a future CI-run fact, not derivable from the repository at rest. Re-run live in this verification: `gh run list --workflow=ci.yml --limit 5` still returns run `30861568499` (2026-08-03T23:14:24Z) as the most recent run against `release/v0.7.0` — five days *before* the deletion. No run exists that could confirm or deny the clause. Per the phase's own D-07 constraint and 12-VALIDATION.md's Manual-Only table, marking this VERIFIED here would be exactly the false positive the phase warns against. This is also the truth carrying `verification: backstop` in 12-01's must_haves — per the honest-verifier contract it abstains rather than passes."
  - test: "Apply (or decline to apply) the committed GitHub rulesets (`.github/rulesets/protect-main-branch.json`, `protect-release-tags.json`) to the live repository, and confirm main-branch protection actually enforces the `Security Audit` required status check."
    expected: "A repository-owner decision and action, recorded at the milestone close-out, on whether/when to apply the version-controlled ruleset JSON."
    why_human: "Live repository administration state, explicitly out of this phase's scope (D-10). Re-verified live in this session: `gh api repos/:owner/:repo/rulesets` returns `[]` and `gh api repos/:owner/:repo/branches/main/protection` returns `404 Branch not protected` — unchanged from the phase's own recorded finding. The phase correctly did not apply anything; a human/repo-owner decision is what remains outstanding, not a verification gap in the phase's own work."
---

# Phase 12: Supply-Chain Gate Integrity Verification Report

**Phase Goal:** The supply-chain gates this project runs on every push give one verdict, and every suppression behind them has a name and a date attached to it.
**Verified:** 2026-08-09T14:32:09Z
**Status:** human_needed
**Re-verification:** No — initial verification

## Phase Shape Note

This is a governance phase by design: zero `.rs` files change across all four plans (confirmed:
`git diff --name-only 2ad8488^..5f01c76 -- '*.rs' | wc -l` → `0`). SUPPLY-01 and SUPPLY-02 were
executed by Phase 9 and inherited here as closed items to *verify*, not re-plan (ROADMAP.md §Phase
12's dated 2026-08-08 closure note). SUPPLY-03 (the ADR-0036 promotion + D-08 regression guard) was
the only genuinely open work. This report is scored against that shape — Success Criteria 1–4 in
ROADMAP.md §Phase 12 were already satisfied by Phase 9; criterion 5 was half-satisfied going in and
Phase 12's job was the other half (the ADR-promotion decision).

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `cargo audit`, `cargo deny check`, `./scripts/check-advisory-register.sh` (twice) all exit 0, transcripts recorded in this execution | ✓ VERIFIED | Re-ran all three independently in this verification session: `cargo audit` exit 0 (8 allowed warnings, all covered by `SECURITY-EXCEPTIONS.md`); `cargo deny check` exit 0, tail `advisories ok, bans ok, licenses ok, sources ok`; `./scripts/check-advisory-register.sh` exit 0, `10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries`. `REQUIREMENTS.md:1904-1930, 2001-2022` carries plan 12-01's own re-run transcripts (not copied from context files) |
| 2 | Exactly one `run: cargo audit` and one `name: Security Audit` across all workflow files — the SUPPLY-01 adjacency defect stays closed | ✓ VERIFIED | `grep -n "security-audit:\|name: Security Audit\|run: cargo audit" .github/workflows/ci.yml` → single occurrence each, at `ci.yml:61-62,78` |
| 3 | SUPPLY-01's CI-run-observation clause is recorded `pending` with trigger and run-ID boundary `30861568499`, never closed | ✓ VERIFIED (record honesty) / see human item 1 for the underlying fact | `REQUIREMENTS.md:1920-1930` and the hand-off block (`:2118-2126`) both record it pending, not closed, with the correct boundary run and trigger. Live re-check this session: `gh run list --workflow=ci.yml --limit 5` still shows `30861568499` (2026-08-03T23:14:24Z) as the newest run against `release/v0.7.0` — the record is still accurate today, five days pre-deletion. `gh run view 30861568499 --json jobs` confirms `API Surface Tracking: failure` was the only failing job and both `Security Audit` entries were `success`, matching the phase's citation exactly |
| 4 | The unapplied-GitHub-rulesets finding is recorded with an owner (milestone close-out), and nothing is applied to the live repository | ✓ VERIFIED (record honesty) / see human item 2 for the live state | `REQUIREMENTS.md:1932-1939` records the finding correctly. Live re-check: `gh api repos/:owner/:repo/rulesets` → `[]`; `gh api repos/:owner/:repo/branches/main/protection` → `404 Branch not protected` — unchanged, confirming the phase applied nothing and the finding is still accurate |
| 5 | A `cargo audit`/`cargo deny` invocation carrying an inline advisory-ignore flag in any workflow file fails `make check-gates` and CI — the D-08 guard enforces, not merely asserts | ✓ VERIFIED | Re-ran the guard behaviorally against fresh scratch fixtures in this session (not trusting the SUMMARY's own claims): planted `cargo-audit --ignore ...` (hyphenated form), `cargo +nightly audit --ignore ...` (toolchain form), `cargo audit "--ignore" ...` (quoted form), and `cargo install cargo-audit --locked && cargo-audit --ignore ...` (chained form) — all four fire `CLAUSE1_INLINE_SUPPRESSION`, exit non-zero. `cargo install cargo-audit --locked` alone stays clean (exit 0). Negative test against the real, unmodified tree: exit 0, silent on both known false-positive tokens. Idempotent: two runs byte-identical. `make check-gates` exits 0 with all 4 guards reported |
| 6 | ADR-0036 exists, `Accepted`, `conforms`, cites ADR-0024 without superseding, seven headings in prescribed order, no frontmatter, parses cleanly | ✓ VERIFIED | `node adr-parser.cjs --input .planning/decisions/0036-*.md` → `status=accepted`, `key_files=12` (== 12 visible bullets), `options_considered=4` (== 4 visible bullets), `decisions` non-empty, `unmapped_headers` includes `Code Conformance` and `Downstream Consumers`. No `## Supersedes` line. `git status --porcelain -- .planning/decisions/0024-rustsec-exception-governance.md` empty — ADR-0024 untouched. Every `ci.yml` line citation (`:61-78`, `:74-76`, `:78`, `:103-104`, `:121`) re-verified against the current tree and resolves exactly |
| 7 | ADR-0036 governs topology, not contents; the two-file framing (`.cargo/audit.toml` and `deny.toml`) is adopted explicitly over the narrower FR-1 framing | ✓ VERIFIED | `## Context` states the FR-1-vs-§8 distinction explicitly and cites both by line. `## Code Locations` cites both `.cargo/audit.toml:11,37` and `deny.toml:115-116` as the two legal surfaces |
| 8 | All corrections (10 dated banners across REQUIREMENTS.md/PROJECT.md/ROADMAP.md/STATE.md) are purely additive — original text retained beside every correction | ✓ VERIFIED | All 7 originally-cited passages still grep-match verbatim (`This requirement does not act`, `Eleven ADR candidates exist and none is promoted`, `Promoting the two ADR candidates into locked decisions`, `Eleven ADR candidates have accumulated`, `crates.io returns HTTP 403` ×2, `Delete \`ci.yml:389-406\``). SUPPLY-01's still-pending CI-run clause (`confirming the required status check still resolves`) correctly carries **no** "Corrected by Phase 12" banner — left honestly unbannered per D-07. Non-zero deletions seen in the raw `git diff --numstat` trace to (a) legitimate checkbox/traceability-row flips, (b) one documented content-neutral line-rewrap (12-01-SUMMARY §Deviations #1), and (c) routine `STATE.md` frontmatter/tracking-field updates (timestamps, `Current Position`) — none is a deletion of corrected content |
| 9 | `PROMOTION.md` advances correctly: 36 contiguous ascending unique index rows, candidate 7 closed, next-free advanced to 0037 as the final act, single writer in the final wave | ✓ VERIFIED | `grep -c '^| 00' PROMOTION.md` → 36; ascending/unique confirmed; `Next free ADR number: 0037` present (the plan's own documented verify-script imprecision — a second, unrelated match on Part A step 5's prose — is a known, harmless false-count noted in `12-04-SUMMARY.md`, not a substantive defect); `Closed 2026-08-09 by ADR-0036` present once, inside candidate 7's entry only; `PROJECT.md`'s Key Decisions table carries the ADR-0036 row with Outcome `conforms`, matching the ADR's own verdict verbatim |

**Score:** 8/9 truths independently re-verified in this session (the 9th, SUPPLY-01's `verification: backstop` truth about the ruleset/required-status-check resolving on a real post-deletion CI run, is correctly *not* claimable locally and is routed to human verification below, matching the phase's own honest framing rather than being asserted).

### The One Clause Deliberately Not Marked Passing

Per this task's explicit instruction, the CI-run-observation clause (SUPPLY-01) and the unapplied
GitHub-rulesets finding (D-10) are **not** marked verified as facts-in-the-world — only the phase's
own *recording* of them as pending/owner-only is verified as honest and accurate. Both were
re-checked live in this session (`gh run list`, `gh api rulesets`, `gh api branches/main/protection`)
and both are unchanged from what the phase recorded: no CI run postdates the 2026-08-08 deletion,
and the rulesets are still unapplied. This is exactly the state 12-VALIDATION.md's Manual-Only
Verifications table predicts, and it is why overall status is `human_needed` rather than `passed`.

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `scripts/check-workflow-suppressions.sh` | D-08 regression guard | ✓ VERIFIED | Exists, executable, offline (no `curl`/`wget`/`http`/`urllib`/`requests`), behaviorally re-tested this session including the two Critical bypasses code review found (CR-01, CR-02) — both confirmed fixed |
| `Makefile` `check-workflow-suppressions` target | wired into `check-gates` | ✓ VERIFIED | `Makefile:171-176`; `make check-gates` exits 0, runs 4 guards |
| `.github/workflows/ci.yml` guard step | in `cargo-deny:` job | ✓ VERIFIED | `ci.yml:103-104`, structurally parses inside `jobs.cargo-deny.steps`; `security-audit:` job byte-identical |
| `.planning/decisions/0036-audit-suppression-single-source-topology.md` | ADR-0036 | ✓ VERIFIED | Structurally valid, `conforms`, all citations resolve |
| `.planning/REQUIREMENTS.md` hand-off block | Phase 13 / ORCH-01 hand-off | ✓ VERIFIED | `REQUIREMENTS.md:2084-2175`, 7 numbered items, closing Evidence line, all citations resolve |
| `.planning/decisions/PROMOTION.md` | index row, dated note, closure note, `Next free ADR number: 0037` | ✓ VERIFIED | See truth #9 |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `Makefile` | `scripts/check-workflow-suppressions.sh` | `check-workflow-suppressions` target | ✓ WIRED | `make check-workflow-suppressions` exits 0 |
| `.github/workflows/ci.yml` | `scripts/check-workflow-suppressions.sh` | `cargo-deny:` job step | ✓ WIRED | Confirmed by structural YAML parse, not grep |
| `.planning/decisions/0036-*.md` | `scripts/check-workflow-suppressions.sh` | `## Code Locations` citation | ✓ WIRED | Cited by path, wiring points named |
| `.planning/decisions/0036-*.md` | `.planning/decisions/0024-*.md` | `## Context` cites by number, no supersession | ✓ WIRED | `ADR-0024` cited; ADR-0024's file untouched |
| `.planning/REQUIREMENTS.md` hand-off | `.planning/decisions/0036-*.md` | `**Evidence:**` line | ✓ WIRED | Cited by path |
| `.planning/decisions/PROMOTION.md` | `.planning/decisions/0036-*.md` | Numbering-index row + dated note | ✓ WIRED | `ls .planning/decisions/0036-*.md` output quoted in the dated note |

### Behavioral Spot-Checks (independently re-run this session, scratch fixtures via `mktemp -d`, never the real tree)

| Behavior | Command | Result | Status |
|---|---|---|---|
| Guard silent on real, unmodified tree | `./scripts/check-workflow-suppressions.sh` | exit 0, `6 workflow file(s) scanned, 109 run step(s) examined, 1 cargo audit invocation(s) found` | ✓ PASS |
| Guard idempotent | run twice, `diff -q` | identical | ✓ PASS |
| Guard fires — hyphenated form (`cargo-audit --ignore ...`) | scratch copy, planted | exit 1, `CLAUSE1_INLINE_SUPPRESSION` fired | ✓ PASS |
| Guard fires — toolchain form (`cargo +nightly audit --ignore ...`) | scratch copy, planted | exit 1, fired | ✓ PASS |
| Guard fires — quoted-flag form (`cargo audit "--ignore" ...`) | scratch copy, planted | exit 1, fired | ✓ PASS |
| Guard fires — chained-install form (`cargo install cargo-audit --locked && cargo-audit --ignore ...`) | scratch copy, planted | exit 1, fired | ✓ PASS |
| `cargo install cargo-audit --locked` alone stays clean | scratch copy, planted | exit 0 | ✓ PASS |
| `make check-gates` | 4 guards | exit 0 | ✓ PASS |
| `cargo audit` (live re-run) | — | exit 0, 8 allowed warnings | ✓ PASS |
| `cargo deny check` (live re-run) | — | exit 0, `advisories ok, bans ok, licenses ok, sources ok` | ✓ PASS |
| `./scripts/check-advisory-register.sh` (live re-run) | — | exit 0, 10/10/5 | ✓ PASS |
| `node adr-parser.cjs` structural check | ADR-0036 | `status=accepted`, both unmapped headers present | ✓ PASS |
| Working tree clean after all scratch tests | `git status --porcelain` | empty | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan(s) | Status | Evidence |
|---|---|---|---|
| SUPPLY-01 | 12-01, 12-04 | ✓ SATISFIED (with pending sub-clause honestly recorded) | Checkbox `[x]`, traceability `Complete`; underlying CI-run confirmation genuinely pending (see human item 1) |
| SUPPLY-02 | 12-01, 12-04 | ✓ SATISFIED | Checkbox `[x]`, traceability `Complete`; all three gate transcripts pass |
| SUPPLY-03 | 12-02, 12-03, 12-04 | ✓ SATISFIED | Checkbox `[x]`, traceability `Complete`; ADR-0036 `conforms`, D-08 guard enforced |

No orphaned requirements: `REQUIREMENTS.md` and `ROADMAP.md` name only SUPPLY-01/02/03 against Phase 12, and all four plans' `requirements:` frontmatter fields cover exactly this set.

### Anti-Patterns Found

No `TBD`/`FIXME`/`XXX` debt markers, no unreferenced `TODO`/`HACK`/`PLACEHOLDER`, no stub returns, and no hardcoded-empty data in `scripts/check-workflow-suppressions.sh`, `Makefile`, or the `ci.yml` diff hunk. `.planning/` documentation edits are all correction banners (additive, dated, original retained) or new closure records — none is a stub.

**Known open code-review findings (not blockers, reported as follow-up per this task's explicit instruction):**

| ID | Severity | File | Issue | Status |
|---|---|---|---|---|
| WR-01 | Warning | `check-workflow-suppressions.sh:87` | `import yaml` unguarded — `ModuleNotFoundError` on missing PyYAML fails closed but with a raw traceback, not the sibling scripts' actionable message | Still open — confirmed present this session |
| WR-02 | Warning | `check-workflow-suppressions.sh:189-192` | `except yaml.YAMLError` doesn't catch `OSError`/`UnicodeDecodeError` — fails closed but contradicts the header's "always a named failure" claim for that error class | Still open — confirmed present this session |
| WR-03 | Warning | design-level | `${{ }}` GitHub Actions expression indirection is an undocumented static-scan blind spot | Still open |
| WR-04 | Warning | design-level | Composite actions (`.github/actions/**/action.yml`) are outside the scan surface; no such files exist today, but a future refactor could silently escape detection | Still open — confirmed no `action.yml` files exist in this repo currently |
| IN-01 | Info | — | No persisted regression test file for the guard; verification was ephemeral `mktemp -d` fixtures | Still open — confirmed no `tests/scripts/check-workflow-suppressions*` file exists |
| IN-02 | Info | `ci.yml:103-104` | Guard step lives in "License & Dependency Policy" job rather than "Security Audit" — cosmetic, consistent with sibling-guard placement | Still open |

Both Critical findings (CR-01: command-position variants bypass both clauses; CR-02: quoted/tab-separated flags bypass clause 1) were fixed in commits `530227d` and `5f01c76`, and independently re-verified behaviorally in this session — all four original bypass shapes now correctly fire, and the chained-install edge case (the follow-up bug in the first CR-01 fix) also correctly fires.

### Human Verification Required

### 1. SUPPLY-01's CI-run-observation clause

**Test:** After the next push to `release/v0.7.0`, run `gh run list --workflow=ci.yml --limit 5` and confirm a run exists with a creation timestamp after 2026-08-08, then `gh run view <run-id> --json jobs -q '.jobs[] | select(.name=="Security Audit")'`.
**Expected:** The `Security Audit` job reports `success`, confirming the required-status-check clause resolves against the single surviving job now that the duplicate has been deleted.
**Why human:** This is a future CI-run fact. No local proof exists — re-checked live in this verification, the newest run against `release/v0.7.0` is still `30861568499` (2026-08-03T23:14:24Z), five days before the deletion commit `cb75b2b`. This is the truth carrying `verification: backstop` in 12-01's `must_haves`; per the honest-verifier contract it must abstain here rather than be asserted passing.

### 2. The unapplied GitHub rulesets

**Test:** Repository owner decides whether/when to apply `.github/rulesets/protect-main-branch.json` and `protect-release-tags.json` to the live repository.
**Expected:** A recorded decision at the milestone close-out (the owner this phase named), either applying the rulesets or explicitly deferring with a reason.
**Why human:** Live repository administration state, explicitly out of this phase's scope (D-10) and requiring repository-owner permissions this phase does not have and should not use. Re-checked live: `gh api repos/:owner/:repo/rulesets` → `[]`; `gh api repos/:owner/:repo/branches/main/protection` → `404`.

### Gaps Summary

No blocking gaps. The phase's substantive deliverables — the D-08 guard (behaviorally re-verified,
including the two review-fixed Critical bypasses), ADR-0036 (structurally re-verified), the three
gate transcripts (independently re-run live), the ten correction banners (all originals retained),
and the Phase 13 hand-off — all check out under direct re-derivation, not SUMMARY-trust. The only
reason overall status is `human_needed` rather than `passed` is that this phase itself, by design,
left one clause (SUPPLY-01's CI-run confirmation) and one finding (the unapplied rulesets) as
things it explicitly could not and should not close from inside this environment — and marking
either "passed" here would be exactly the false positive the phase's own D-07/D-10 constraints and
12-VALIDATION.md's Manual-Only table warn against. This is the phase working as designed, not a
defect in it.

---

_Verified: 2026-08-09T14:32:09Z_
_Verifier: Claude (gsd-verifier)_
