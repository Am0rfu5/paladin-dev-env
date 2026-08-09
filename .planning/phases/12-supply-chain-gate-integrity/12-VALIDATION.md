---
phase: 12
slug: supply-chain-gate-integrity
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-09
---

# Phase 12 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
>
> **Domain note (from `12-RESEARCH.md` § Validation Architecture):** this phase's validation applies
> to **records**, not code — the framing Phase 11 established. A "test" is a shell command that
> proves a citation resolves, a count matches, or an annotation exists at a named path. No Rust is
> written, so no Rust test harness is added. **Phase 12 differs from Phase 11 in one material way:**
> it carries three genuine external gate invocations (`cargo audit`, `cargo deny check`,
> `./scripts/check-advisory-register.sh`) that hit a real advisory database and a real crates.io
> index, so its full suite is an actual command run with real (if short) latency — not purely
> instantaneous greps.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None — direct shell verification (`grep`, `ls`, `find`, `git log`) for every record-shaped deliverable, **plus** three real compiled-tool invocations (`cargo-audit`, `cargo-deny`), both already installed and proven passing this session |
| **Config file** | None new. The three gates read existing config from `.cargo/audit.toml` and `deny.toml` — **unchanged by this phase per D-00i** |
| **Quick run command** | `./scripts/check-advisory-register.sh` (~1s), or per-claim `grep -n "<pattern>" <file>`, or `node .claude/gsd-core/bin/lib/adr-parser.cjs --input <path>` for a single ADR |
| **Full suite command** | `cargo audit && cargo deny check && ./scripts/check-advisory-register.sh` + the ADR structural check + the D-08 positive/negative pair |
| **Estimated runtime** | ~30–60s total, dominated by `cargo audit`'s advisory-db fetch. No `cargo build` / `clippy` / `test` required — this phase expects zero `.rs` changes |

---

## Sampling Rate

- **After every task commit:** re-run the specific citation/grep that task's own banner, ADR section
  or hand-off line depends on. For a gate-touching task, also re-run the one gate command its
  evidence cites.
- **After every plan wave:** re-run the full three-gate suite. Once the D-08 guard has landed, also
  re-run its positive/negative pair at each subsequent wave merge, so a later edit cannot silently
  disable its detection.
- **Before `/gsd-verify-work`:** full suite green, including a live planted-violation test for the
  D-08 guard (against a scratch copy, torn down afterward — never the real tree) and the ADR
  structural check against the final `0036-*.md`.
- **Max feedback latency:** ~30–60s (network-bound on `cargo audit`); ~1s for every record check.

---

## Per-Task Verification Map

Task IDs are assigned by the planner; the Requirement / Test Type / Command columns below are the
contract each task must satisfy. Rows are grouped by deliverable class.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 12-01-T1 | 12-01 | 1 | SUPPLY-01 | T-12-01 | The single surviving audit job reaches one verdict | gate-command | `cargo audit` → exit 0 | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-01 | T-12-01 | Exactly one `cargo audit` invocation across all workflows | grep-count | `grep -rhc 'run: cargo audit' .github/workflows/*.yml \| awk '{s+=$1} END {print s}'` → `1` | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-01 | T-12-01 | No two jobs share the `Security Audit` display name — the adjacency defect | grep-count | `grep -rhc 'name: Security Audit' .github/workflows/*.yml \| awk '{s+=$1} END {print s}'` → `1` | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-02 | T-12-03 | Reconciled suppression set passes the licence/ban/advisory gate | gate-command | `cargo deny check` → exit 0, tail `advisories ok, bans ok, licenses ok, sources ok` | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-02 | T-12-03 | Register agrees with both TOMLs and `Cargo.lock`, case-sensitively and over sets | gate-command | `./scripts/check-advisory-register.sh` → exit 0, `✅ 10 register row(s) …` | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-02 | T-12-03 | The register check is idempotent — two consecutive runs, identical output and exit code | gate-command ×2 | run `./scripts/check-advisory-register.sh` twice; `diff` the two captures → identical | ✅ | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-01 | T-12-02 | The CI-run observation is recorded `pending` with its trigger and run-ID boundary, never closed | grep-presence | `grep -c '30861568499' .planning/REQUIREMENTS.md` → non-zero, inside SUPPLY-01's block | ❌ W0 | ⬜ pending |
| 12-01-T1 | 12-01 | 1 | SUPPLY-01, SUPPLY-02 | — | Both checkboxes and both traceability rows close together | grep ×2 | `grep -c '^\| SUPPLY-0[12] \| Phase 12 \| Complete \|'` → `2` | ❌ W0 | ⬜ pending |
| 12-01-T2 | 12-01 | 1 | SUPPLY-01, SUPPLY-02 | T-12-04 | The two "CI-only" sites carry a dated blocker-lifted banner, original retained | grep-pair ×2 | see "Correction-banner protocol" below | ❌ W0 | ⬜ pending |
| 12-01-T2 | 12-01 | 1 | SUPPLY-01 | T-12-04 | Every live stale `ci.yml:389-406` citation in the canonical set names `ci.yml:465-482` and commit `cb75b2b` | grep-pair ×8 | `grep -rc '465-482' .planning/REQUIREMENTS.md .planning/PROJECT.md .planning/ROADMAP.md .planning/STATE.md` summed → ≥ `8` | ❌ W0 | ⬜ pending |
| 12-02-T1 | 12-02 | 2 | SUPPLY-03 | T-12-09 | Guard fires on a planted inline advisory-ignore (space, `=`, `cargo deny`, backslash-continuation forms) | positive test ×4 | guard invoked against a scratch copy with one planted violation → non-zero exit | ❌ W0 | ⬜ pending |
| 12-02-T1 | 12-02 | 2 | SUPPLY-03 | T-12-12 | Guard stays silent on `mc mb --ignore-existing` and `cargo test -- --ignored` | negative test | guard invoked against the real unmodified `.github/workflows/` → exit 0 | ❌ W0 | ⬜ pending |
| 12-02-T1 | 12-02 | 2 | SUPPLY-03 | T-12-10 | Zero-`run:` file is clean; zero-file scan directory is a named non-zero failure, never a silent pass | empty-input test ×2 | guard invoked against an empty `mktemp -d` → non-zero with a named failure | ❌ W0 | ⬜ pending |
| 12-02-T1 | 12-02 | 2 | SUPPLY-03 | T-12-15 | Report order is deterministic — two runs over an unchanged tree are byte-identical | idempotency test | run guard twice, `diff -q` the two captures → identical | ❌ W0 | ⬜ pending |
| 12-02-T1 | 12-02 | 2 | SUPPLY-03 | T-12-13 | No planted fixture reaches the tree | cleanliness check | `git status --porcelain -- .github/workflows/` → empty at task end | ❌ W0 | ⬜ pending |
| 12-02-T2 | 12-02 | 2 | SUPPLY-03 | T-12-14 | Guard is wired into both `make check-gates` and the `cargo-deny:` CI job | make-run + structural-parse | `make check-gates` → exit 0; PyYAML parse of `ci.yml` finds the step in `jobs.cargo-deny.steps` | ❌ W0 | ⬜ pending |
| 12-03-T1 | 12-03 | 3 | SUPPLY-03 | T-12-22 | ADR-0036 exists, `Accepted`, `conforms`, cites ADR-0024 without superseding | file-exists + structural-parse | `ls .planning/decisions/0036-*.md` + ADR structural check (below) | ❌ W0 | ⬜ pending |
| 12-03-T1 | 12-03 | 3 | SUPPLY-03 | T-12-18 | ADR-0024 is untouched — no `## Supersedes` line, no status change | cleanliness check | `git status --porcelain -- .planning/decisions/0024-*.md` → empty | ❌ W0 | ⬜ pending |
| 12-03-T1 | 12-03 | 3 | SUPPLY-03 | T-12-17 | Every `ci.yml` citation is re-derived after plan 12-02's step insertion | source assertion | each cited line/range resolves to the content the ADR claims | ❌ W0 | ⬜ pending |
| 12-03-T2 | 12-03 | 3 | SUPPLY-03 | T-12-19 | Each of the four stale promotion-viability passages carries a dated correction AND retains its original text | grep-pair ×4 | see "Correction-banner protocol" below | ❌ W0 | ⬜ pending |
| 12-04-T1 | 12-04 | 4 | SUPPLY-01, SUPPLY-02, SUPPLY-03 | T-12-30 | `#### Hand-off to Phase 13 / ORCH-01` exists in the established four-part shape | grep | `grep -n "Hand-off to Phase 13 / ORCH-01" .planning/REQUIREMENTS.md` | ❌ W0 | ⬜ pending |
| 12-04-T1 | 12-04 | 4 | — | T-12-29 | `.planning/ledgers/milestone-09-12.md` is NOT created | absence check | `ls .planning/ledgers/milestone-09-12.md` → fails; `ls .planning/ledgers/` → 4 files | ❌ W0 | ⬜ pending |
| 12-04-T1 | 12-04 | 4 | SUPPLY-01, SUPPLY-02, SUPPLY-03 | T-12-28 | Traceability rows flip `Pending` → closed | grep | `grep -c '\| Phase 12 \| Pending \|' .planning/REQUIREMENTS.md` → `0` | ❌ W0 | ⬜ pending |
| 12-04-T2 | 12-04 | 4 | SUPPLY-03 | T-12-25 | The Numbering index holds 36 ascending, unique, contiguous rows with none renumbered | sort-check ×2 | `awk -F'\|' '/^\| 00/{gsub(/ /,"",$2); print $2}' PROMOTION.md \| sort -c` and `\| sort -u \| wc -l` → `36` | ❌ W0 | ⬜ pending |
| 12-04-T2 | 12-04 | 4 | SUPPLY-03 | T-12-26 | `PROMOTION.md` next-free advances to 0037 with a dated note, written last | grep | `grep -c "Next free ADR number" PROMOTION.md` → `1`, reading `0037` | ❌ W0 | ⬜ pending |
| 12-04-T2 | 12-04 | 4 | SUPPLY-03 | T-12-27 | Exactly one plan writes `PROMOTION.md`, in the final wave — no concurrent allocation of 0036 | structural | scan each plan's `files_modified` block only (a plain `grep -l` also matches plans that merely *cite* the file): exactly one plan lists `decisions/PROMOTION.md`, and it is `12-04-PLAN.md` at `wave: 4` | ✅ | ⬜ pending |
| 12-04-T2 | 12-04 | 4 | SUPPLY-03 | — | `PROMOTION.md` Part B candidate 7 carries a "Closed 2026-08-09 by ADR-0036" note | grep | `grep -c "Closed 2026-08-09 by ADR-0036" .planning/decisions/PROMOTION.md` → `1` | ❌ W0 | ⬜ pending |
| 12-04-T2 | 12-04 | 4 | SUPPLY-03 | — | `PROJECT.md` gains exactly one Key Decisions row for ADR-0036 (31 → 32) | count | `sed -n '/^## Key Decisions/,/^## /p' .planning/PROJECT.md \| grep -c '^\| \['` → `32` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*
*File Exists: ✅ = target exists and passes today; ❌ W0 = target is an artifact this phase authors — the command becomes runnable the moment that artifact is written. **This is not-yet-written content, not missing infrastructure.***

---

### ADR structural check (verified sound against ADR-0031 this session)

```bash
node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0036-<slug>.md \
  | python3 -c "
import json, sys
d = json.load(sys.stdin)
assert d['status'] == 'accepted', f\"status={d['status']!r}, expected 'accepted'\"
assert len(d['key_files']) > 0, 'Code Locations parsed to zero entries'
assert len(d['options_considered']) > 0, 'Considered Options parsed to zero entries'
assert len(d['decisions']) > 0, 'Decision section empty'
unmapped = set(d['unmapped_headers'])
assert 'Code Conformance' in unmapped, 'Code Conformance heading missing or misnamed'
assert 'Downstream Consumers' in unmapped, 'Downstream Consumers heading missing or misnamed'
print('ADR-0036 STRUCTURAL CHECK: PASS')
"
```

Two mechanical facts this check surfaced, both actionable for **authoring** ADR-0036, not only
verifying it:

1. `unmapped_headers` always includes the ADR's own H1 title — the parser treats every heading as a
   section boundary. Assert **subset membership**, never `len(unmapped_headers) == 2`, or the check
   spuriously fails on every ADR in the corpus.
2. Trailing **un-bulleted** prose after `## Code Locations`'s bullet list is chopped into per-line
   fragments (ADR-0031's `key_files` reads 28 against ~10 real citations). **ADR-0036 should put any
   verification transcript inside `## Code Conformance`** — an unmapped, unparsed header where prose
   is harmless — rather than trailing after `## Code Locations`.

### Correction-banner protocol (D-00c: annotation, not rewriting)

Every corrected site needs **both** greps to hit. Banner present *and* original retained.

**Plan 12-01's sites** — the two lifted-blocker caveats, plus the stale `ci.yml:389-406` sweep:

```bash
# 1. Banner present, dated, naming the authoring plan
grep -n "Corrected by Phase 12 (plan 12-01), dated 2026-08-09" \
  .planning/REQUIREMENTS.md .planning/PROJECT.md .planning/ROADMAP.md .planning/STATE.md

# 2. Originals still present verbatim
grep -n "crates.io returns HTTP 403" .planning/REQUIREMENTS.md      # SUPPLY-02's caveat
grep -n "returns HTTP 403" .planning/ROADMAP.md                     # the ROADMAP closure note
grep -n 'Delete `ci.yml:389-406`' .planning/PROJECT.md              # the open-checkbox citation

# 3. NOT bannered — SUPPLY-01's CI-run clause is genuinely still pending (D-07)
grep -n "confirming the required status check still resolves" .planning/REQUIREMENTS.md
```

**Plan 12-03's sites** — the four promotion-viability passages:

```bash
# 1. Banner present, dated, naming the authoring plan
grep -n "Corrected by Phase 12 (plan 12-03), dated 2026-08-09" \
  .planning/REQUIREMENTS.md .planning/PROJECT.md

# 2. Originals still present verbatim
grep -n "This requirement does not act" .planning/REQUIREMENTS.md
grep -n "Eleven ADR candidates exist and none is promoted" .planning/REQUIREMENTS.md
grep -n "Promoting the two ADR candidates into locked decisions" .planning/PROJECT.md
grep -n "Eleven ADR candidates have accumulated, and none is promoted" .planning/PROJECT.md
```

A site where grep 1 hits but grep 2 does not — original text deleted — is a **process violation**,
not a content error, and fails verification even if the corrected fact is accurate. A `--numstat`
deletion count above zero on any of the four documents is the same violation seen from the diff side.

**Scope note on the `389-406` sweep.** `12-CONTEXT.md` says "three documents"; `12-RESEARCH.md` §B.5
found one and flagged the discrepancy as Open Question 1. The planner re-ran the broad grep:
`grep -rn '389-406' .planning/ .project/` returns **46 hits across 21 files**. Plan 12-01 re-runs it and
records the live count. In scope: `REQUIREMENTS.md`, `PROJECT.md`, `ROADMAP.md`, `STATE.md`. Out of
scope with reasons: `.planning/milestones/v0.7.1-*` (frozen archive), `.planning/phases/09-*` (prior-phase
record, per Assumption A2), `.planning/intel/*` and `INGEST-CONFLICTS.md` (closed ingest outputs),
`.planning/ledgers/milestone-01.md` (accurate as written), and `0024-*.md` (already annotated, and not
edited under D-00i).

### D-08 guard — positive/negative pair

The discriminating logic was extracted and run standalone this session (`POSITIVE/NEGATIVE PAIR:
PASS`): a violation requires `cargo (audit|deny)` **and** an `--ignore` token on the *same* `run:`
string. That defeats both known false positives (`mc mb --ignore-existing`,
`cargo test -- --ignored`) by construction rather than by regex luck.

Once the script exists, re-run the pair **against the script**, not the extracted regex:

- **Negative (must stay silent):** invoke the guard against the real, unmodified
  `.github/workflows/` tree — which already contains both false-positive tokens → expect exit `0`.
- **Positive (must fire):** invoke it against a **scratch copy** with one planted violation appended
  → expect non-zero. Never mutate the real tree; tear the scratch dir down afterward.

**Design note for the planner:** none of the three existing guard scripts accepts a root-directory
override — each hardcodes `WORKSPACE_ROOT` from `BASH_SOURCE`. D-08's guard should add one (e.g.
`WORKFLOWS_DIR="${1:-${WORKSPACE_ROOT}/.github/workflows}"`) purely so the positive test is runnable
without touching the real tree. That is an addition to the new guard's own contract, not a change to
the three existing scripts.

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.* There is no test framework to install —
`grep`, `ls`, `find`, `git`, `python3`, `node`, `cargo-audit` and `cargo-deny` are all present and
every command above was proven runnable this session, including two that depend on no unwritten
artifact (the ADR structural check against an existing ADR, and the D-08 regex logic standalone).

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| The required status check resolves on the first real CI run after the Phase 9 deletion | SUPPLY-01 (D-07) | Future CI-run fact; no local proof exists. Most recent run is `30861568499` (2026-08-03T23:14:24Z), five days *before* the 2026-08-08 deletion — no run exists that could confirm or deny it | Record as `pending`, trigger = next push to `release/v0.7.0`. **A verifier that marks this done without a `gh run` citation postdating the deletion is reporting a false positive.** Do not simulate or infer a pass |
| Committed GitHub rulesets are not applied to the live repository | — (D-10, recorded finding) | Live repository administration state; explicitly out of phase scope to change | Cite CONTEXT.md's already-run `gh api` commands in the hand-off block. **Add no new verification command** — doing so would imply the phase intends to act on it |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
