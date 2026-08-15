---
phase: 09-release-security-gate-integrity
verified: 2026-08-08T05:30:00Z
status: passed
score: 5/5 must-haves verified (all five ROADMAP success criteria substantively met; one documentation-consistency gap flagged as human-review item, not a functional failure)
behavior_unverified: 0
overrides_applied: 0
human_verification:

  - test: "Open .project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md directly (not via REQUIREMENTS.md or SECURITY-EXCEPTIONS.md) and check whether a reader lands on the superseded owner/date without a pointer forward."
    expected: "A dated D-00c-style annotation banner at or near line 39-40 should say the owner ('Platform Security (Milestone 7)') and the 2026-09-30 date are superseded by ADR-0024 / SECURITY-EXCEPTIONS.md, with the original text retained below — matching the treatment plan 09-05 gave license-compatibility-decision-checklist.md and the two PRD documents for SEC-02."
    why_human: "This is a documentation-completeness judgment, not a mechanically checkable pass/fail: the register and guards are real and enforced regardless of this file's state, so the functional criterion (one authoritative answer, mechanically enforced) is met even though this one formal source document was left unannotated. A human should decide whether this asymmetry (SEC-02's source docs were annotated; SEC-01's principal source document was not) needs a follow-up doc-only fix or is acceptable as-is given ADR-0024 and REQUIREMENTS.md already carry the correction."
---

# Phase 9: Release & Security Gate Integrity Verification Report

**Phase Goal:** The security, licensing and release gates this project believes it already has
actually hold — one advisory exception set instead of four, a licence the manifests declare, and a
published crate family that passes its own release criteria.
**Verified:** 2026-08-08
**Status:** human_needed (one documentation-consistency item flagged; no functional gap found)
**Re-verification:** No — initial verification

## Method

This report is built from direct tree inspection and live command execution in this sandboxed
environment — not from re-reading the seven SUMMARY.md files' narrative. Every guard script named
below was independently broken and restored by this verifier (not merely re-run in its
already-passing state) to confirm it is genuinely failable, per the phase's own D-15/D-02 mandate
and Phase 8's `check-deprecations.sh` cautionary precedent. `deny.toml`/`.cargo/audit.toml` were
read directly; `SECURITY-EXCEPTIONS.md`'s TOML payload was read directly; `ci.yml` was greped and
diffed directly against the phase base commit `7ae7dd4f3b59f4d40aab74d86bc035476d8f3d5e`.

## Goal Achievement

### Observable Truths (ROADMAP success criteria)

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | Asking "which RustSec advisories does this project suppress, and why?" returns one answer with one owner, not four | ✓ VERIFIED | `SECURITY-EXCEPTIONS.md` is the single governance register (10 rows, `owner = "DF3NDR"` on all ten, read directly). `deny.toml`/`.cargo/audit.toml` are declared mechanical mirrors and mechanically enforced to stay so by `scripts/check-advisory-register.sh` (independently broken 1 way below, confirmed non-zero exit). **Partial residual:** `.project/.../rustsec-remediation-plan.md:39` still reads "Exception owner: Platform Security (Milestone 7)" verbatim, un-annotated — see Human Verification item 1. |
| 2 | Every suppressed advisory carries owner/expiry/scope/compensating-control, and the 2026-09-30 acceptance is renewed/closed/replaced | ✓ VERIFIED | All 10 register rows read directly: every row carries all 11 fields (`id, class, crate, path, why_present, why_not_fixable, owner, review_date, scope, compensating_control, revisit_condition`) non-empty; `owner = "DF3NDR"`, `review_date = "2026-12-31"` on all ten. The 2026-09-30 acceptance is renewed (not closed) to per-advisory 2026-12-31 dates — recorded in ADR-0024 decision 5 and `REQUIREMENTS.md`'s SEC-01 closure note. |
| 3 | `cargo audit` behaves identically locally and in CI | ✓ VERIFIED | `grep -c 'run: cargo audit' .github/workflows/ci.yml` → `1` (confirmed directly). `make audit` (Makefile:259-263) runs bare `cargo audit`; the sole surviving CI job `security-audit:` (ci.yml:61-78) also runs bare `cargo audit` — both auto-discover the same `.cargo/audit.toml`. The duplicate `security:` job with inline `--ignore` flags is confirmed deleted via `git diff` against the phase base (19 lines removed, no other job touched). **Scoped honestly:** whether `cargo audit`/`cargo deny check` themselves *pass* against the reconciled config is explicitly recorded as CI-only/unverified-here in `REQUIREMENTS.md`'s SEC-01 note and 09-07-SUMMARY's CI-only claims list — not inferred as passing. This is the correct posture, not a gap. |
| 4 | One licence answer the root package, ten library crates and `deny.toml` agree on | ✓ VERIFIED | `grep -h '^license = ' Cargo.toml crates/*/Cargo.toml \| sort -u \| wc -l` → `1` (`MIT OR Apache-2.0`, all eleven governed manifests). `LICENSE-MIT`/`LICENSE-APACHE` both present; `LICENSE-APACHE` confirmed byte-identical (`diff`, zero output) to the local Rust toolchain's canonical Apache-2.0 text. `deny.toml [licenses] allow` already contains both `MIT` and `Apache-2.0` (unchanged, confirmed by reading). `Dockerfile.chef:87`'s OCI label and `README.md`'s badge/License section both read `MIT OR Apache-2.0`. `cargo check --offline --workspace` run live in this session: exits 0. The licence-change decision is recorded in ADR-0025 as taken by the repository owner (`DF3NDR`) at a blocking `checkpoint:decision` task with a `<precondition>` gate (confirmed by reading `09-05-PLAN.md:127,194`) — not inferred by an agent. |
| 5 | Herald changelog exists; Dockerfile.chef planner stage cannot go stale; crates.io name collision is detectable pre-release | ✓ VERIFIED | `crates/paladin-herald/CHANGELOG.md` exists with real backfilled content (creation commit `66f6c4e`, confirmed to exist in `git log`; ADR-0023 feature-gating entry). `grep -c 'COPY crates/paladin' Dockerfile.chef` → `0` (enumeration deleted, not extended); `COPY crates ./crates` is the sole, structural coverage mechanism, confirmed present at `:30`/`:61`; same zero-count confirmed for `Dockerfile` and `Dockerfile.server`. `.crate-names.txt` (11 names) + `scripts/check-crate-names.sh` run on every PR inside the required `cargo-deny` job — independently broken 1 way below (case-only variant), confirmed non-zero exit and correct restoration. |

**Score:** 5/5 ROADMAP criteria substantively met. One item (criterion 1's principal source document)
is flagged for human judgment rather than counted as a functional failure — see below.

### Independently-verified guard failability (not taken from SUMMARY narrative)

All three guards were broken and restored by this verifier directly, in this session:

| Guard | Fixture applied | Result | Restored cleanly |
|-------|------------------|--------|-------------------|
| `scripts/check-changelogs.sh` | Moved `crates/paladin-battalion/CHANGELOG.md` aside | `EXIT=1`, names `paladin-battalion` | Yes — `git status --porcelain` clean, re-run exits 0 |
| `scripts/check-crate-names.sh` | Changed `.crate-names.txt`'s `paladin-web` entry to `Paladin-Web` (case-only) | `EXIT=1`, both directions fail (unlisted `paladin-web`, stale `Paladin-Web`) — proves exact, non-case-folded comparison | Yes — re-run exits 0 |
| `scripts/check-advisory-register.sh` | Injected a fabricated `"RUSTSEC-9999-0000"` immediately after `deny.toml`'s `ignore = [` marker | `EXIT=1`, `CLAUSE1_DENY_MISMATCH` + `CLAUSE2_UNCOVERED` reported | Yes — re-run exits 0 |

All three guards parse via Python's `tomllib` (real TOML parsing), not string/regex manipulation —
confirmed by reading each script's body. This directly answers the task's specific concern about
trailing-comment fixtures being a silent no-op: the guards themselves are comment-agnostic by
construction (they parse structured TOML, never scrape comment text), so this class of fragility
does not apply to the shipped scripts (it only applied to the *plan's own test fixture text*, which
09-06-SUMMARY documents was corrected during execution — independently confirmed accurate by
reading `scripts/check-advisory-register.sh`'s actual parsing logic above).

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `SECURITY-EXCEPTIONS.md` | 10-row governance register, 11 fields/row | ✓ VERIFIED | Read directly; matches exactly |
| `deny.toml` | 10 entries (5 vuln + 5 unmaintained), 4 dead entries deleted | ✓ VERIFIED | Read directly: `python3 -c "import tomllib;print(len(tomllib.load(open('deny.toml','rb'))['advisories']['ignore']))"` → `10` |
| `.cargo/audit.toml` | 5 vulnerability entries | ✓ VERIFIED | Read directly, matches |
| `scripts/check-changelogs.sh`, `check-crate-names.sh`, `check-advisory-register.sh` | Present, executable, genuinely failable | ✓ VERIFIED | All three exist, `chmod +x` set, all three independently broken/restored above |
| `.crate-names.txt` | 11 owned package names | ✓ VERIFIED | Read directly, 11 non-comment lines |
| `crates/paladin-herald/CHANGELOG.md` | Keep-a-Changelog format, real content | ✓ VERIFIED | Read directly; substantive, not boilerplate |
| `Dockerfile.chef` | Nine-manifest enumeration deleted | ✓ VERIFIED | `grep -c 'COPY crates/paladin' Dockerfile.chef` → `0` |
| `LICENSE-MIT`, `LICENSE-APACHE` | Present, `LICENSE` renamed via `git mv` | ✓ VERIFIED | Both present; `git log --follow` confirms history-preserving rename; `LICENSE-APACHE` byte-identical to canonical Apache-2.0 text |
| Eleven manifests | `license = "MIT OR Apache-2.0"` | ✓ VERIFIED | Read directly, one distinct value |
| `.planning/decisions/0024-0027*.md` | 4 ADRs, seven-heading no-frontmatter shape matching 0022/0023 | ✓ VERIFIED | Headings compared directly; no frontmatter on any of the four |
| `.planning/decisions/PROMOTION.md` | Advances to "Next free ADR number: 0028" | ✓ VERIFIED | Read directly |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `ci.yml`'s `cargo-deny` job | Three guard scripts | `run:` steps | ✓ WIRED | All three scripts invoked as steps inside `cargo-deny` (`License & Dependency Policy`), confirmed by reading `ci.yml:89-99` |
| `.github/rulesets/protect-main-branch.json` | `security-audit` job | required `context: "Security Audit"` | ✓ WIRED | Confirmed directly: ruleset requires `"Security Audit"` and `"License & Dependency Policy"` contexts; surviving `security-audit:` job's `name:` field (unchanged) posts the former; `cargo-deny` job posts the latter |
| `Makefile` | Three guards | `check-changelogs`, `check-crate-names`, `check-advisory-register`, `check-gates` targets | ✓ WIRED | Confirmed by reading Makefile:159-172 |
| Root `Cargo.toml`/crate manifests | `deny.toml [licenses] allow` | licence expression acceptance | ✓ WIRED | `cargo check --offline --workspace` run live: exits 0 |

### Scope-Fence Verification (must NOT have happened)

| Fence | Check | Result |
|-------|-------|--------|
| No `.rs` changed | `git diff --stat <base>..HEAD -- '*.rs' \| wc -l` | `0` (verified against both `7ae7dd4f3b59f4d40aab74d86bc035476d8f3d5e` and plan-cited `49ad74c`) |
| No deprecated-Action bump (PIPE-04) | `grep -c 'actions-rs/toolchain@v1' ci.yml` before/after | `3` / `3` — unchanged |
| No `dotenv`→`dotenvy` swap | `grep dotenvy Cargo.toml crates/*/Cargo.toml` | No matches; `dotenv = "0.15.0"` still present in root `Cargo.toml` |
| No PDF-capability decision (HARD-06) | `SECURITY-EXCEPTIONS.md`, ADR-0024 wording | Both explicitly state the capability question is handed to Phase 10/HARD-06, not answered |
| No `cargo doc` warning-bar decision (HARD-07) | Not touched by any plan/SUMMARY | Confirmed absent from all diffs |
| `ci.yml` diff scoped to only the intended change | `git diff -- ci.yml` read in full | Exactly: +1 chmod step, +3 guard steps (inside `cargo-deny` job), −19 lines (the duplicate `security:` job). No other job touched. The one incidental `-dtolnay/rust-toolchain@stable` line is a necessary artifact of deleting the whole duplicate job's own toolchain step, not a version bump — confirmed this is the only place that string appears in the diff |

### Requirements Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| SEC-01 | ✓ SATISFIED | `[x]` in REQUIREMENTS.md:1101; register + guard + CI collapse all confirmed live |
| SEC-02 | ✓ SATISFIED | `[x]` in REQUIREMENTS.md:1227; eleven manifests + licence files confirmed |
| SEC-03 | ✓ SATISFIED | `[x]` in REQUIREMENTS.md:1245; guard confirmed wired and failable |
| SEC-04 | ✓ SATISFIED | `[x]` in REQUIREMENTS.md:1278; changelog confirmed present and substantive |
| SEC-05 | ✓ SATISFIED | `[x]` in REQUIREMENTS.md:1289; Dockerfile.chef confirmed structural |
| SUPPLY-01 (Phase 12, hand-off) | ✓ Recorded closed-by-Phase-9 with commit refs, checkbox correctly left open for Phase 12 to verify | Confirmed in REQUIREMENTS.md:1655-1664 and ROADMAP.md's Phase 12 section |
| SUPPLY-02 (Phase 12, hand-off) | ✓ Recorded closed-by-Phase-9 with commit refs | Confirmed similarly |
| Phase 10/HARD-01 hand-off block | ✓ Present | REQUIREMENTS.md:1326-1348, all seven `REQ-*` rows named |

No orphaned requirements found — SEC-01…SEC-05 are the complete requirement set for this phase per
ROADMAP, and all five appear in the plans' `requirements` fields.

### Anti-Patterns Found

None found in the phase-modified files. No `TODO`/`FIXME`/`XXX`/`HACK`/`PLACEHOLDER` markers in any
new script, ADR, or config file. Both `exit 0` counts in each guard script are exactly 1 (confirmed
by reading each script directly, not by trusting the SUMMARY's `grep -c` claim).

### Honesty Discipline (D-19) — CI-only / unverifiable-here claims

Confirmed these are recorded as unverified-here, never inferred as passing, in `REQUIREMENTS.md`'s
SEC-01 closure note and 09-07-SUMMARY's "CI-only claims" section:

1. `cargo audit` / `cargo deny check` actually passing against the reconciled config — correctly
   recorded as not run (neither tool installable, `crates.io` HTTP 403 confirmed in this session too:
   `scripts/check-advisory-register.sh` and friends do not attempt installation).

2. The `"Security Audit"` required status check resolving after the duplicate-job deletion — correctly
   recorded as CI-only (branch protection is GitHub-evaluated).

3. `cargo chef cook` reporting `CACHED` on source-only rebuilds — correctly recorded as
   documentation-established (cargo-chef's own semantics), not measured (Docker absent from this
   environment, confirmed).

4. crates.io accepting the `MIT OR Apache-2.0` expression at next publish — correctly recorded as a
   future release-cycle action, not exercised.

No instance found where any of these four is asserted as passing rather than pending/CI-only.

## Gaps Summary

**No functional/mechanical gap found.** All five ROADMAP success criteria are substantively met by
independently-verified tree evidence (not SUMMARY narrative): the guards are real (broken and
restored by this verifier, not merely re-run passing), the register is real and complete, the
licence is one answer across all governed sites, the CI collapse to one `cargo audit` invocation is
real, and the Dockerfile fix is structural rather than a patch.

**One documentation-consistency item is flagged for human judgment, not classified as a gap:**
`.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md` — the original formal
risk-acceptance document that ROADMAP criterion 1 names as one of the "four different answers" this
phase collapses to one — was never annotated with a dated correction banner pointing at ADR-0024 /
SECURITY-EXCEPTIONS.md, unlike the parallel treatment plan 09-05 gave the SEC-02 source documents
(`license-compatibility-decision-checklist.md`, the M7 Epic 4 PRD, and the M7 overview). A reader
opening that specific file directly still sees "Exception owner: Platform Security (Milestone 7)"
and "2026-09-30" with no forward pointer. The governance itself is not affected — `deny.toml`,
`.cargo/audit.toml`, `SECURITY-EXCEPTIONS.md` and the mechanical guard are all correct and
enforced regardless of this one file's state — so this does not block the phase goal. It is
recorded as a human-verification item because closing it is a judgment call (is a doc-only follow-up
worth a plan, or is the correction already sufficiently visible via `REQUIREMENTS.md` and
`CONCERNS.md`?) rather than a mechanically-resolvable pass/fail.

---
*Verified: 2026-08-08*
*Verifier: Claude (gsd-verifier)*
