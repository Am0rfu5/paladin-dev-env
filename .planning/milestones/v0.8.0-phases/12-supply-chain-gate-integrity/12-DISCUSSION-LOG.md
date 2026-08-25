# Phase 12: Supply-Chain Gate Integrity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-09
**Phase:** 12-supply-chain-gate-integrity
**Mode:** `--auto` — all seven gray areas auto-selected, every question resolved to its recommended
option without a human in the loop. No `AskUserQuestion` call was made.
**Areas discussed:** Promotion viability · Live candidate inventory · Candidate-7 verdict ·
Relationship to ADR-0024 · Verification evidence bar · Inline-`--ignore` regression guard ·
Where closure is recorded

---

## Promotion viability — does SUPPLY-03 act, or only record?

| Option | Description | Selected |
|--------|-------------|----------|
| Act — write ADR-0036, correct the stale text at source (recommended) | Follow `PROMOTION.md` §Part A, which states the `--manifest`/re-ingest path "no longer exists" and that promotion is now an ordinary directory write. Four candidates were already promoted this way by Phases 7 and 9. | ✓ |
| Record a recommendation only | Honour SUPPLY-03's literal "This requirement does not act" clause; write a recommendation and leave the candidate open. | |
| Escalate to the user before acting | Treat the three-document contradiction as unresolvable without a human. | |

**Choice:** Act (→ D-01). Flagged `⚠ HUMAN REVIEW` in CONTEXT.md because it overrides a governing
requirement's own explicit self-limitation.
**Notes:** Three documents (`REQUIREMENTS.md:1937-1939`, `REQUIREMENTS.md:102-110`, `PROJECT.md`
§Out of Scope) say promotion is impossible from a planning artefact. `PROMOTION.md` §Part A lines
165-170 supersedes all three, and `PROMOTION.md` Part B records candidates 1, 2, 3 and 5 as already
closed by ADR-0016/0021/0024/0025 with no re-ingest. Under the project's own precedence order
(D-00b) the ADR-class document plus shipped practice outrank PRD/DOC-tier requirement text.

---

## Live candidate inventory — two candidates, or one?

| Option | Description | Selected |
|--------|-------------|----------|
| One candidate; record the correction (recommended) | Candidate 3 was closed 2026-08-08 by ADR-0024. Only candidate 7 (M10 Epic 2 FR-1 + §8, "Owner phase: Phase 12") survives. | ✓ |
| Two candidates as written | Take SUPPLY-03's text literally and write ADRs for both. | |

**Choice:** One (→ D-02).
**Notes:** Writing a second ADR on ADR-0024's subject would put two live ADRs on one question,
breaking `PROMOTION.md` §Supersession's "exactly one live ADR answers each question at any time".
`REQUIREMENTS.md:1801` already establishes the precedent that a competing second ADR on a settled
subject is prohibited.

---

## Candidate-7 verdict — promote or decline?

| Option | Description | Selected |
|--------|-------------|----------|
| Promote, `conforms` verdict (recommended) | The tree now satisfies the invariant (Phase 9's deletion). ADR ratifies a true state rather than mandating change — the ADR-0031 shape. | ✓ |
| Decline with a recorded reason | Leave the invariant at PRD precedence; record why it was not promoted. | |
| Promote with a `must change` verdict | Treat the invariant as still violated and attach remediation. | |

**Choice:** Promote as ADR-0036, `conforms` (→ D-03, D-04).
**Notes:** `grep -c 'run: cargo audit' .github/workflows/ci.yml` → 1; zero advisory-ignore flags in
any workflow. The "currently violated by the tree" note in `PROMOTION.md` Part B candidate 7 is
itself stale. `must change` was rejected on evidence. Declining was rejected because it leaves the
invariant overridable one phase after the project paid for the violation.

---

## Relationship to ADR-0024

| Option | Description | Selected |
|--------|-------------|----------|
| Standalone ADR-0036, cites ADR-0024 (recommended) | Different questions: 0024 governs the exception set's *contents*; 0036 governs the *topology* of where suppressions may live. | ✓ |
| Amend ADR-0024 to absorb the invariant | One ADR covering both governance and topology. | |
| Supersede ADR-0024 | Replace it via the `## Supersedes` mechanism. | |

**Choice:** Standalone (→ D-05).
**Notes:** Superseding is simply false — ADR-0024 is live and its 2026-12-31 review dates are in
force. Amending would make one ADR answer two questions and blur the supersession mechanism.

---

## Verification evidence bar for SUPPLY-01 / SUPPLY-02

| Option | Description | Selected |
|--------|-------------|----------|
| Verify locally; the CI-only caveat is dead (recommended) | All three gates run and exit 0 in this environment. Record transcripts verbatim per D-00e. | ✓ |
| Keep the inherited CI-only posture | Defer all verification to the next CI run, as Phases 9 and 10 did. | |
| Structural greps only | Verify by file inspection without running the tools. | |

**Choice:** Verify locally (→ D-06), with the one genuinely un-closable clause recorded as pending
(→ D-07).
**Notes:** `cargo audit` → exit 0 (1190 advisories, 677 deps, 8 allowed warnings, zero
vulnerabilities); `cargo deny check` → exit 0 (`advisories ok, bans ok, licenses ok, sources ok`);
`./scripts/check-advisory-register.sh` → exit 0 (10 register rows vs 10 `deny.toml` / 5
`.cargo/audit.toml`). Phases 9 and 10 both recorded `crates.io` HTTP 403; that blocker has lifted
and the lifting is itself worth dating. The CI-run clause cannot be closed here — the most recent
run (`30861568499`, 2026-08-03) predates the 2026-08-08 deletion, so it has never had the chance
to fire. Faking it was rejected; blocking the phase on a push it does not own was also rejected.

---

## Inline-`--ignore` regression guard

| Option | Description | Selected |
|--------|-------------|----------|
| Add a minimal offline guard (recommended) | Fourth clause in `check-advisory-register.sh` or a sibling script wired into the same `make check-gates` / `ci.yml:101` sites. | ✓ |
| ADR only, no enforcement | Record the invariant in prose and rely on review. | |
| Build a general CI-policy linter | A broader "workflows may not do X" checker. | |

**Choice:** Minimal guard (→ D-08). Flagged `⚠ HUMAN REVIEW` because no requirement explicitly asks
for a new CI check.
**Notes:** SUPPLY-03's own words — "would turn the run-5 supply-chain finding from an observation
into a gate". Today's enforcement has a real hole: `check-advisory-register.sh` asserts register ↔
TOML ↔ `Cargo.lock` agreement but nothing asserts that no workflow carries an inline suppression —
the exact defect SUPPLY-01 existed to fix. The general linter was rejected as scope creep (a new
capability with its own phase). Constraints fixed in D-08: offline; match advisory-ignore flags
specifically so `mc mb --ignore-existing` and `cargo test -- --ignored` do not false-positive;
assert exactly one `cargo audit` across workflows; report every violation.

---

## Where closure is recorded

| Option | Description | Selected |
|--------|-------------|----------|
| `REQUIREMENTS.md` rows + hand-off block to Phase 13 (recommended) | Match the three existing dated hand-off blocks. Leave the M9-12 ledger to ORCH-01. | ✓ |
| Create `.planning/ledgers/milestone-09-12.md` now | Start the ledger this phase and let Phase 13 extend it. | |

**Choice:** Requirement rows plus a hand-off block (→ D-09).
**Notes:** Building the Milestone 9-12 ledger is ORCH-01's stated deliverable across 120
requirement IDs; a stub here would either be re-planned or would silently constrain its shape. The
hand-off must carry ORCH-01's named verdict class — Milestone 10 recorded 100% complete with one of
its own acceptance criteria false, and no longer false as of 2026-08-08.

**Adjacent finding recorded, not acted on:** `.github/rulesets/` is committed but
`gh api repos/:owner/:repo/rulesets` returns empty and `main` is not protected — so SUPPLY-01's
"required status check" has no live enforcement point. Applying a ruleset is an outward-facing
change to a live repository and owner-only, so it is recorded with evidence and handed to the
milestone close-out (→ D-10).

---

## Claude's Discretion

- Whether D-08's guard is a fourth clause inside `scripts/check-advisory-register.sh` or a separate
  sibling script — constraints fixed, file layout open.
- Whether ADR-0036's `## Considered Options` reproduces the decline branch in full or names it.
- Plan decomposition and wave assignment; `PROMOTION.md:59` must be updated last per its own
  procedure.
- Exact wording and placement of the dated correction banners, subject to D-00c.

## Deferred Ideas

- Applying the committed GitHub rulesets to the live repository — owner-only, milestone close-out.
- Closing milestone v0.7.2 and reconciling the ROADMAP `## Milestones` table — STATE.md flags the
  discrepancy; `/gsd-complete-milestone` decision.
- Fixing the `API Surface Tracking` CI job — DEBT-01, Phase 8.
- The `scraper` / `rss` / `tiktoken-rs` dead optional dependencies — named to Phase 15 by the Phase
  10 hand-off.
- Human ratification of Phase 9's `--auto` decisions D-07, D-09 and D-16 — D-07 re-scoped this very
  phase and was never confirmed.
- Promoting the other nine ADR candidates — each keeps its owning phase per `PROMOTION.md` Part B.
- A general CI-policy linter — its own phase.
</content>
