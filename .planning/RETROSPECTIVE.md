# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v0.7.1 — Milestone 1 close-out

**Shipped:** 2026-08-04
**Phases:** 4 | **Plans:** 38 | **Tasks:** 88 | **Commits:** 255

### What Was Built

- **Nine ADRs (`0001`–`0009`)** settling every contested definition in the corpus, each naming its
  chosen variant and the shipped code it was checked against. `.planning/decisions/` and
  `.planning/ledgers/` were stood up as new document classes to hold them.
- **Code changes applying those decisions** — `ProviderCapabilities.temperature_range`, Formation's
  1-Paladin minimum, the `BattalionCheckpointConfig` rename, and Formation's per-Paladin
  times/tokens/`node_errors` on `BattalionResult` rendered through all three Heralds.
- **A real UTF-8 panic fix** in `TableHerald::truncate_text` plus the two adjacent panic paths that
  shared the same defective helper.
- **An offline coverage pipeline** (`rustc -C instrument-coverage` → `llvm-profdata` → `llvm-cov`)
  that works with no `cargo-llvm-cov`, no network and no Docker — 84.79% at Phase 1, reproduced at
  85.56% / 85.92% in Phase 3.
- **62 previously dead tests activated** — 25 in `tests/unit/llm/`, 37 in `tests/cli/` — none
  deleted, plus four real Commander error-path tests replacing `#[ignore]`d stubs.
- **Release coherence** — twelve manifests on version 0.7.0 / edition 2024, advisory posture
  recorded, quickstart fixed and measured at 15 minutes, gate suite at 2,924 tests / 185 doc tests
  / 47 example targets.

### What Worked

- **Recording a decision before applying it.** Phases 1→2 ran as "ADR first, code second", and the
  audit's highest-value check — do the six ADRs match shipped code? — came back clean on all of
  them. The ADR gave the code change an unambiguous target.
- **Refusing to paper over a missing prerequisite.** Plan 01-08 declined to write a ledger row it
  could not source, and 01-04 halted at its own Task 1 precondition rather than fabricating a
  coverage number. Both were later closed properly by gap-closure plans. The halts cost time and
  bought correctness.
- **Measuring instead of restating.** The "22 runnable examples" figure had propagated through five
  documents from a single Milestone-1 validation report. Phase 4 counted the tree (47 `.rs` files,
  4 declared targets) and replaced the count with a property — "every example target builds" —
  that cannot go stale the same way.
- **Re-verification after gap closure.** Three of four phases entered verification as `gaps_found`
  and were re-verified to `passed` only after the gaps were actually closed. No phase was talked
  into passing.

### What Was Inefficient

- **The coverage question was answered three times.** Plan 01-04 halted on it, 01-09 measured it,
  01-10 wrote the ADR, 01-12 flipped the checkbox — and Phase 3 then re-measured it twice more. A
  single decision consumed five plans across two phases because the measurement tooling was
  established only after the requirement was already in flight.
- **REQUIREMENTS.md bookkeeping drifted from reality repeatedly.** Two separate gap-closure plans
  (02-11, 03-07/03-08) existed solely to reconcile checkboxes and traceability rows against work
  that was already done. The record lagged the code by a whole verification cycle each time.
- **A premature checkbox flip had to be reverted.** RECON-07 was marked satisfied before ADR-0006
  existed (commit `799c53f` reverted it). Caught, but only after it shipped into the file.
- **Nyquist validation never ran at all.** All four `VALIDATION.md` files sat at `status: draft`
  through the entire milestone and nobody noticed until the closing audit.

### Patterns Established

- **ADR-first precedence.** `.planning/decisions/` outranks PRDs and ingested docs; the shipped tree
  outranks both. Wired into PROJECT.md as an explicit precedence order.
- **Machine-parseable decision records.** ADRs are validated by `adr-parser.cjs`, and checkbox flips
  can be gated on that parse succeeding — a decision cannot be cited before it is well-formed.
- **Provenance-standard measurement (D-17).** Every recorded figure carries `rustc -vV`,
  `cargo --version`, `git rev-parse HEAD`, `date -u` and raw pasted stdout — not a restated number.
  This is what made the 84.79% figure auditable months later.
- **Deferrals carry a named owner or an explicit "no owner assigned".** Never a silent drop. Ten
  deferrals left this milestone; every one names where it goes.
- **Superseded work is preserved, not deleted.** The 2026-05-27 benchmark run stayed in place marked
  superseded; plan 01-04 got a disposition record rather than removal.

### Key Lessons

1. **Establish the measurement before writing the requirement that depends on it.** The coverage
   gate cost five plans because the requirement (RECON-07) was written before anyone knew whether
   the number could be produced in this sandbox. Cheap fix: a feasibility spike on the measurement
   tool during discuss-phase.
2. **Bookkeeping is not free — schedule it inside the plan that does the work.** Every phase needed
   a dedicated follow-up plan to reconcile REQUIREMENTS.md. Flipping the checkbox should be the
   last task of the plan that earns it, not a separate cleanup pass.
3. **A count copied from a report is a liability; a property proven by a command is not.** Prefer
   success criteria that re-derive themselves ("every example target builds") over criteria that
   restate a number ("22 examples").
4. **Verification timestamps are load-bearing.** A documentation-only commit added after the fact
   invalidated Phase 1's verification and forced this milestone to close as `override_closeout`.
   If a phase is verified, later edits to its directory need either a re-verify or a conscious
   override.
5. **Workflow defaults can be wrong for the project.** The stock milestone-close step deletes
   REQUIREMENTS.md; here that file carries forward scope for twelve unstarted phases. Read what a
   destructive step actually targets before running it.

### Cost Observations

- Sessions: not tracked this milestone.
- Notable: the offline coverage pipeline was the single highest-leverage artifact produced — it
  unblocked QUAL-01/02/03 and is reusable by Phase 15's PIPE-02 without modification.
- Notable: three phases required a second verification pass. Budgeting one re-verification per
  phase as the expected case, rather than the exception, would have made the schedule honest.

---

## Milestone: v0.8.0 — Milestone 2-12 close-out & Provider Expansion

**Shipped:** 2026-08-24
**Phases:** 14 (5-17, incl. inserted 15.1) | **Plans:** 149 | **Commits:** 1,014

### What Was Built

Four ingest-derived milestone blocks closed out, plus the first forward work beyond the ingest.
Five as-shipped ledgers now carry 554 `REQ-*` rows with `file:line` verdicts. The quality gates
Deferred-QA Epic 25 specified and nobody started exist and run. Nine LLM providers ship where three
did. Branch protection went from literally nothing to three applied rulesets with 44 required
contexts.

### What Worked

- **Record-then-apply, split across phases.** Phases 5, 7, 10 and 13 recorded; 6, 8, 9, 12, 14
  changed code. The zero-`.rs` boundary on the recording phases was independently re-measured at
  each close and held every time. Discovery never quietly became implementation.
- **Verifiers that abstain rather than pass.** Four phases came back `gaps_found` or `human_needed`
  on their first pass and were re-verified after real fixes (06, 14, 16, 17). Phase 12 refused to
  mark a clause passing because no CI run existed that could confirm it — and it was right to.
- **Disclosure over silent scope-cutting.** Phase 13 found a route defect it could not fix without
  breaching its own zero-`.rs` boundary, and handed it forward with the exact fix and a named
  owner. Phase 14 fixed it. That is the mechanism working.
- **Measuring a tool instead of trusting it.** The Snyk probe is the single highest-value thing this
  milestone produced: four deliberate vulnerabilities, 0 findings in Rust, 3 in JavaScript. It
  turned six plans' worth of unsatisfiable blocking into a recorded, evidence-backed removal.

### What Was Inefficient

- **A stale requirement blocked verification in six plans before anyone measured it.** The Snyk
  mandate sat in an untracked instructions file from Phase 15.1 through Phase 17, recorded as
  "not run" five times, before being tested. The probe took an afternoon. It should have been the
  first response to the second failure, not the tenth.
- **The ROADMAP's milestone table went stale for four blocks and thirteen phases.** Every phase was
  `[x]` while the table read "Not started". Nothing was unbuilt; the record simply was not
  maintained, in a milestone whose entire purpose was making the record true.
- **`Requirements: TBD` carried into execution once and cost a retroactive settlement.** Phase 15.1
  shipped seven verified success criteria with no identifiers, which the milestone audit then had
  to record as a traceability silence rather than close.
- **Disk exhaustion silently degraded verification twice.** Plans 14-01 and 14-04 could not run
  `cargo test --workspace` at all (99% full). Targeted verifies covered it, but the gap was
  environmental and unflagged until close.

### Patterns Established

- **Probe the scanner, not just the code.** A clean result from a tool that cannot analyse the
  language reads as assurance while meaning nothing. Adopted as a hard precondition in SAST-01.
- **Dated at-source correction banners that retain the original text.** Applied consistently across
  ledgers, ADRs and `.project/` annotations; nothing was silently edited away.
- **Mint requirement IDs at roadmap time.** Direct consequence of the 15.1 experience; Phase 18
  minted `SAST-01`…`SAST-04` before any planning began.
- **Guards that parse the artifact, not prose about it.** `check-advisory-register.sh` and
  `check-workflow-triggers.sh` both enforce relationships earlier phases had only asserted.

### Key Lessons

1. **A gate that cannot fail is worse than no gate.** The duplicate audit job, the Snyk mandate, and
   the path-filter trap in `CLAUSE_CONTEXT` are three instances of one defect class: something that
   reports success without doing work.
2. **Check whether a later phase already closed the finding.** The milestone audit's first pass
   carried two Phase 12 items forward as open; Phase 15.1 had closed both five days later. Reading
   a VERIFICATION.md without asking what came after it produces confidently stale conclusions.
3. **A record's account of itself drifts even when the record is correct.**
   `SECURITY-EXCEPTIONS.md` governed eleven suppressions correctly while its own heading said ten.
4. **Deferring with a named owner and a working fix is not scope-cutting.** Deferring without one
   is.

### Cost Observations

- Sessions: not tracked per-milestone
- Notable: four phases required a second verification pass, and three required `--gaps` replanning
  rounds (17 needed three). The re-verification loop, not first-pass execution, is where the
  quality came from — and it is not free.

## Milestone: v0.9.0 — Security Tooling

**Shipped:** 2026-09-01
**Phases:** 4 (18-21) | **Plans:** 25 | **Commits:** 240

### What Was Built

The supply-chain and release posture, settled by measurement. CodeQL proven to analyse all 385
first-party `.rs` files and then disqualified as a required-check Rust SAST by a five-class probe
(retained advisory-only, version-scoped verdict). crates.io publishing moved to per-run OIDC
tokens with the standing credential revoked and deleted. A pre-publish consistency gate,
idempotent same-tag re-runs, registry-state publish detection, and a rehearsed stuck-halfway
runbook. Release artifacts made real: curated-changelog body, feature-correct binaries, a
digest-bound image, verifiable checksums — proven by the first fully-green release run in this
project's history (`v0.8.1-rc.5`).

### What Worked

- **Pre-registered criteria before every measurement.** Phase 18 wrote and committed promotion
  thresholds, re-probe criteria and confound-test criteria *before* each run produced the numbers
  they judge — no threshold was ever retrofitted to a result, across four measurement rounds
  including a self-correction.
- **Live rehearsal on throwaway rc tags as the proof standard.** rc.2 proved the OIDC exchange,
  rc.3/rc.4 proved the recovery path (and found two real gate bugs the phase then fixed),
  rc.5 proved the artifact path end-to-end. Every pipeline claim in this milestone traces to a
  real run ID, not a re-reading of the workflow.
- **Ratchet ordering as a requirement, not a preference.** The OIDC path was proven with a real
  publish *before* the old token was revoked; the archive files were committed *before* originals
  were deleted. Nothing was ever in a state where the only copy of a capability was the untested
  one.
- **Narrow, named human backstops — which then actually closed.** Verifications held three claims
  open as `human_needed` rather than letting CI-internal corroboration stand in (token revocation,
  out-of-band pull-by-digest, `paladin-cli` execution). All three were closed by recorded UAT
  within days, with evidence including the failure shapes met on the way.
- **Independent re-verification against live state.** Verifiers queried the GitHub API, the
  crates.io registry and cited run IDs directly instead of trusting SUMMARY.md or even the
  phases' own evidence files — and in every case the claims held.

### What Was Inefficient

- **The binary-attachment defect survived every prior release run silently.** `build-binaries`
  produced nothing (Cargo silently skips a binary with unmet `required-features`) on every release
  since the matrix was written, and was known-broken through Phases 19-20 before Phase 21 owned
  it. Four `fail-fast: false` legs reported green around a missing artifact for months.
- **Three phases rewrote the same `release.yml`, serially.** The 19 → 20 → 21 ordering was honored
  and correct — but shared-file blast radius made the milestone strictly sequential; nothing in
  it could parallelize across phases.
- **A plan was written for a branch not taken.** 18-05's observation-window measurement was mooted
  by the disqualified verdict and closed as not-applicable — cheap, but a branch-aware plan would
  have gated it on the 18-03 verdict instead of scheduling it unconditionally.
- **Five rc tags were consumed as rehearsal budget.** Deliberate and worth it, but the tag
  namespace now carries five `v0.8.1-rc.*` entries whose only meaning is "pipeline rehearsal" —
  worth a note in the release docs so a future reader does not mistake them for abandoned
  releases.

### Patterns Established

- **Version-scoped tool verdicts with a written re-probe trigger.** "Disqualified at CodeQL
  2.26.3 / rust-queries 0.1.40" plus a kept fixture and a named re-run condition — a verdict that
  can be revisited without being re-derived.
- **Coverage before findings, recursively.** Every scan re-proved its analysed-file denominator
  before any finding count was interpreted — the Snyk lesson turned into a standing discipline.
- **Registry state over error prose.** Already-published detection reads the crates.io index;
  release existence reads the GitHub API by tag; both replaced string-matching on error output.
- **Honest tested/untested status lines.** The runbook said "untested" until the rehearsal ran;
  COVERAGE.md's overstated digest-pull row was corrected rather than left to read as proof.

### Key Lessons

1. **A green run proves what it exercised, nothing more.** The first fully-green release run
   arrived only after this milestone made every silent-success path (skipped publish, missing
   binary, `::warning::`-then-pass) into a hard failure. Everything green before that was green
   around holes.
2. **Backstops close when they are narrow and named.** All three `human_needed` items specified
   the exact command, the expected output and why only a human could run it — and all three were
   discharged within days. A vague "needs manual verification" would still be open.
3. **Ordering is itself a security control.** Prove-then-revoke and archive-then-delete both
   turned potentially irreversible transitions into checkpointed ones.
4. **A measured disqualification is a deliverable.** CodeQL joins Snyk in the evidence record —
   with the crucial difference that its coverage was proven, so the verdict is a detection gap,
   not an analysis mirage. The next person asking "why no Rust SAST?" gets a dated answer.

### Cost Observations

- Sessions: not tracked per-milestone
- Notable: 240 commits over 8 days for 25 plans — the smallest, fastest milestone yet, and the
  first with zero `gaps_found` verification passes (all four phases passed first time, two with
  named human backstops). The rehearsal budget (five rc tags, three live pipeline runs) replaced
  the re-verification loops that consumed v0.8.0.

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v0.7.1 | — | 4 | First milestone with protected decisions. The corpus had 0 locked ADRs across twelve prior milestones and eighteen months; this one produced 9. |
| v0.8.0 | — | 14 | First milestone to disqualify a tool by measurement (Snyk probe), and the first to apply live branch protection. Four phases needed a second verification pass. |
| v0.9.0 | — | 4 | First milestone proven by live pipeline rehearsals (five throwaway rc tags, three real release runs); first fully-green release run; standing publish credential eliminated. Zero failed verification passes. |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v0.7.1 | 2,924 passing (+185 doc tests) | 85.92% (floor 84%) | 0 new dependencies |
| v0.8.0 | 428 workspace unit + 247 `paladin-llm` crate-scoped; 96/96 doctests | 82.39% (floor 82, ADR-0006) | 6 new LLM providers, no new heavyweight deps |
| v0.9.0 | +294 shell-harness assertions (177 Phase 20 + 117 Phase 21) over the release tooling | unchanged (floor 82, ADR-0006 — no first-party `.rs` changes required it) | 0 new runtime dependencies (workflow + script work) |

### Top Lessons (Verified Across Milestones)

1. **Record the decision before writing the code, and gate the record on a parser.** Held across
   both milestones — v0.7.1 produced 9 ADRs where twelve prior milestones produced 0; v0.8.0 added
   38 more and wired two mechanical guards that enforce what earlier prose only asserted.
2. **A gate that cannot fail is worse than no gate.** New in v0.8.0, and the milestone's most
   transferable finding: the duplicate audit job, the unsatisfiable Snyk mandate, and the
   path-filter trap all report success without doing work.
3. **Verify against the current tree, not against the last report about it.** All three milestones
   produced findings that were accurate when written and stale when read — v0.9.0's audit itself
   initially carried three human checks as open that recorded UAT had already closed.
4. **Rehearse the pipeline; do not re-read it.** New in v0.9.0: the binary-attachment defect
   survived every prior *reading* of `release.yml` and fell to the first *run* that asserted its
   outputs. Live rehearsal on throwaway tags is now the proof standard for pipeline work.

---

*Next milestone: v0.9.0 Security Tooling (Phase 18). Start with `/gsd-discuss-phase 18`.*
