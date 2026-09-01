# Phase 18: Rust SAST — Evaluate and Adopt CodeQL - Context

**Gathered:** 2026-08-25
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase settles, with recorded evidence, whether a Rust-capable SAST actually analyses *this*
tree — and then acts on the answer. Two outcomes are equally valid deliverables:

- **Qualified** — the scanner finds the planted defects, is wired as an advisory scan, measured
  over a recorded window, and (if the numbers hold) promoted to a required check with all four
  places the required-check set is written down updated in one change.
- **Disqualified** — the probe returns zero findings, the tool is rejected, and the verdict plus
  its evidence is published. `security.instructions.md`'s "Known gap" section is then narrowed to
  match what was actually measured, not deleted.

Requirements: `SAST-01`, `SAST-02`, `SAST-03`, `SAST-04`.

**In scope:** the deliberate-vulnerability Rust probe; a CodeQL workflow; the observation window
and its evidence; alert-triage governance; the required-check promotion; the `SAST-04` doc rewrite.

**Out of scope:** fixing whatever real defects the scanner finds (findings are triaged and
recorded here; remediation is its own work), reintroducing Snyk in any form, adding a
`SECURITY.md` disclosure surface, and any change to the manual credential-handling review's
substance beyond restating what it still owns.

</domain>

<decisions>
## Implementation Decisions

*All decisions below were auto-selected under `--auto` (recommended option taken on each). Each
carries the reasoning that produced it so a human can overturn any single one without re-running
the discussion.*

### Scanner setup mode

- **D-01:** Use CodeQL **advanced setup** — a committed workflow file — not GitHub's default
  setup. This is not a style preference: `scripts/check-workflow-triggers.sh` Clause 3 requires
  every required-status-check context pinned in `.github/rulesets/protect-main-branch.json` to
  resolve to a job display name **declared in some workflow file**. Default setup is configured in
  repository settings and declares no job anywhere in the tree, so pinning its context would fail
  Clause 3 at the exact moment `SAST-03`'s promotion fires. Advanced setup also makes the query
  suite, the build mode, and the feature set reviewable in a pull request.
  — **Reversibility:** costly — undoing means deleting the workflow, re-enabling default setup in
  repo settings, unpinning the context from the ruleset and re-applying ruleset `20868126`, and
  reverting the `branching-model.md` trigger-policy row. Four coordinated edits, not one.

- **D-02:** The analysis job's display name is a **single stable literal** — `CodeQL Analysis
  (Rust)` — with **no `strategy.matrix`** and no `${{ }}` expression in the name. Clause 3 matches
  matrix and expression jobs by *prefix*, which works but is looser; a single-language repository
  gains nothing from a matrix and a literal name resolves exactly. Renaming this job later
  silently drops the gate, which is precisely why Clause 3 exists — treat the string as a
  contract.
  — **Reversibility:** costly — the name is pinned as a required context in the ruleset and
  transcribed into `docs/src/appendix/branch-protection.md`; renaming it requires both to change
  in the same commit or `main` becomes unmergeable.

### Workflow placement and trigger shape

- **D-03:** CodeQL lives in a **new `.github/workflows/codeql.yml`**, not as a job inside
  `ci.yml`. `SAST-02` requires a `schedule` trigger, and `ci.yml` deliberately carries no
  `schedule:` key — `docs/src/contributing/branching-model.md`'s trigger-policy row records the
  reason: a cron on `ci.yml` would fire the entire pipeline weekly, including the hour-plus
  multi-architecture Docker build. Adding a schedule there to accommodate CodeQL would overturn a
  recorded decision to buy convenience.

- **D-04:** Triggers are `pull_request` (branches `[main, 'release/**']`, **no path filter**),
  `push` (branches **`['**']`**), `schedule`, and `workflow_dispatch`.

  The `push: ['**']` choice needs stating explicitly because it looks like it exceeds the
  requirement. `SAST-02` and PROJECT.md both say "push on `main`". But
  `scripts/check-workflow-triggers.sh` Clause 2 fails **any** workflow whose `on.push.branches` is
  a list other than `['**']` unless the file is in `EXCEPTION_FILES = {'docs.yml',
  'release.yml'}` (line 118). Writing `push: [main]` therefore forces one of two things: minting a
  third exception (editing the guard script *and* the register to weaken a guard, in the same
  phase whose whole point is not weakening assurance), or a red required check. `['**']` conforms
  to the guard **unchanged** and is a strict superset of `main`, so the requirement is satisfied
  by over-coverage rather than by exemption. Cost is absorbed by a `concurrency` group with
  `cancel-in-progress` on non-`main` refs, mirroring `ci.yml`'s pattern.
  — **Reversibility:** reversible — narrowing later is a one-line YAML change plus a register row
  update, but it would then require the exception that this decision exists to avoid.

- **D-05:** The workflow file and its **row in the `branching-model.md` trigger-policy table land
  in the same commit**. Clause 1 fails any workflow file with no row, so a workflow committed
  alone turns `main` red immediately. The row's "Triggers" cell must name `push`,
  `pull_request`, `schedule`, `workflow_dispatch` and its "Push branch filter" cell must read
  `['**']`, matching the YAML exactly — Clause 2 compares them literally. The table is parsed by a
  line-based reader: one row, no merged cells, no multi-line cells.

- **D-06:** The advisory phase uses an explicit, **visible** non-blocking posture. No
  `continue-on-error` on a step whose failure would then report success — Success Criterion 6
  forbids a green result that means less than it says. Non-blocking is achieved by the context
  simply **not being in the ruleset yet**, which is honest: the job genuinely fails when it fails,
  it just does not gate a merge. (This is deliberately unlike the existing `osv-scanner` job,
  which stacks `continue-on-error` on both the scan and the SARIF upload.)

### The probe (SAST-01)

- **D-07:** The probe fixture is a **Rust crate in-tree, excluded from the workspace** — its own
  `Cargo.toml`, not a `members` entry, so `cargo build`/`clippy`/`llvm-cov` at the workspace root
  never see it and workspace coverage is unaffected. Rationale: the evidence must be
  **reproducible**. A throwaway fixture on a scratch branch produces a number nobody can re-derive
  in six months, which is the same "trust the record" weakness the Snyk episode exposed.

- **D-08:** The fixture carries the **same four vulnerability classes, in the same order**, that
  disqualified Snyk — hardcoded credential, command injection via `sh -c`, path traversal, SQL
  injection — so the finding counts are directly comparable against the recorded Snyk baseline
  (0 in Rust vs 3 in identical JavaScript). Reuse the methodology verbatim; do not improve it.

- **D-09:** The probe is scanned on a **dedicated evaluation branch / `workflow_dispatch` run**,
  and the fixture is **excluded from the steady-state PR scan path** thereafter. A permanently
  scanned fixture would emit four standing code-scanning alerts forever, and the standing
  dismissals needed to silence them are exactly the "green means nothing" erosion this phase
  exists to prevent. Keeping the fixture re-runnable on demand preserves reproducibility without
  polluting the alert surface.

- **D-10:** The planted credential must not trip the repository's own secret-scanning and
  pre-commit hooks. Use an obviously-synthetic, non-resolving value and confirm `pre-commit run
  --all-files` (a required check) stays green with the fixture present. If it cannot be made to
  pass, that is a finding to record, not something to bypass with `--no-verify`.

- **D-11:** **A zero-finding result ends the phase in the disqualified branch and that is a
  success, not a failure.** No adoption work proceeds past a failed probe. This is stated here so
  no downstream agent treats "CodeQL not adopted" as an incomplete phase.

### Analysis coverage across feature-gated code

- **D-12:** Scan coverage is itself measured and recorded, not assumed. This workspace gates large
  subsystems behind non-default features (`vision`, `content-processing`, `web-server`,
  `llm-openai/anthropic/deepseek/all`, `redis-queue`, `s3-storage`, `storage-mysql`, `qdrant`,
  `cli`, `notifications`). A scan that only sees the default feature set analyses a fraction of
  the tree while reporting clean — the Snyk failure shape with a different mechanism. The plan
  must determine CodeQL's Rust build mode (`none` vs a manual build) and, if any build is
  involved, configure it so feature-gated code is reached.

- **D-13:** **The number of `.rs` files CodeQL reports as analysed is recorded as first-class
  evidence, alongside the finding count.** The denominator is **385** — `crates/**/*.rs` (246) +
  `src/**/*.rs` (139), verified against the tree on 2026-08-25, which is exactly the figure
  `SAST-03` and PROJECT.md cite. (The full tree carries 575 `.rs` files / ~196k lines once
  `tests/`, `examples/`, `benches/` and `doc-examples` are counted; 385 / ~142k is the first-party
  library-and-binary figure and is the right denominator.) An analysed count far below 385 is a
  disqualifying result even if findings are non-zero.

### Observation window (SAST-03)

- **D-14:** The window produces numbers **without blocking the phase on calendar time**. Measure
  by dispatching the scan across a set of recent merged PR head commits (backfill) to obtain a
  false-positive rate over real diffs, **plus** a short live advisory period on actual PR traffic.
  A phase that idles for two weeks waiting for organic PRs invites the outcome the v0.8.0 audit
  criticised — work recorded as open rather than settled.

- **D-15:** Recorded metrics, at minimum: total alerts raised; alerts triaged true-positive;
  alerts triaged false-positive; the resulting FP rate; wall-clock per run (cold and warm cache);
  and the analysed-file count from D-13. Vendor claims are not evidence — only numbers produced
  against this tree count.

- **D-16:** The evidence document is committed under **`.planning/`** (the project's planning
  corpus, where the Snyk evaluation's evidence pattern already lives) and the **conclusion** —
  not the raw log — is what propagates into `.github/instructions/security.instructions.md` under
  `SAST-04`.

### Alert-triage governance

- **D-17:** Dismissed CodeQL alerts get a **governed register modelled on
  `SECURITY-EXCEPTIONS.md`** — named owner, review date, scope, and a concretely-stated
  compensating control per dismissal. Without this, "dismissed as false positive" becomes an
  unaudited escape hatch and the gate decays into the assurance theatre this phase is repairing.
  Whether the register is a new file or a section in the existing one, and whether a
  `check-*.sh` guard enforces it the way `check-advisory-register.sh` does, is a planning call —
  but ungoverned dismissal is ruled out.
  — **Reversibility:** reversible — a register is additive; removing it later costs nothing
  structural.

### Promotion to required check #45

- **D-18:** Promotion is attempted **within this phase**, conditional on the D-14/D-15 numbers
  qualifying — not deferred to unnamed future work. If the numbers do not qualify, the phase
  closes with the scanner advisory and the promotion criteria written down explicitly as a named
  open item with its trigger condition, never as silence.

- **D-19:** Promotion is **one change touching all four recorded places**, per `SAST-03`:
  1. `.github/rulesets/protect-main-branch.json` — add the context, 44 → 45 (44 verified present
     on 2026-08-25).
  2. Re-apply live ruleset `20868126`.
  3. `docs/src/appendix/branch-protection.md` — the context table **and every prose occurrence of
     the count**: the number `44` appears at lines 85, 117 and 180, not only in the table. All must
     move together or the doc contradicts itself.
  4. `scripts/check-workflow-triggers.sh` passes (Clause 3 resolves the new context to D-02's job
     name).
  — **Reversibility:** costly — a required check that disagrees with itself across those four
  places is the exact defect `SAST-03` was written to forbid; unwinding means reversing all four
  in lockstep.

### Semgrep

- **D-20:** Semgrep is a **contingency, not parallel work**. Probe CodeQL first. Only if CodeQL
  fails the D-11 probe does Semgrep get evaluated against the identical fixture. PROJECT.md
  records that Semgrep is pattern matching rather than interprocedural taint analysis and that
  thin Rust rule coverage is "the same failure shape as Snyk" — so it is never the primary
  control, and evaluating both up front doubles the phase for a tool already judged secondary.

### SAST-04 documentation rewrite

- **D-21:** The rewrite's blast radius is bounded to the places that currently **assert the gap**,
  and every one moves in the same change:
  - `.github/instructions/security.instructions.md` §"Known gap: no Rust SAST" (the requirement's
    named target) and its line-26 claim that "No tool above performs taint analysis of first-party
    Rust".
  - `CLAUDE.md`'s Security bullet and `.github/copilot-instructions.md`, if either states the gap.
  - The Snyk section stays. It is a standing prohibition, not stale text.
  - `.planning/STATE.md`'s deferred-item row and `.planning/MILESTONES.md` are updated at phase
    close to reflect the settled verdict.
- **D-22:** The section is **narrowed by evidence, never deleted**. It must state what the adopted
  tool does *not* cover and what the manual credential-handling review still owns — the three
  concrete checks in `security.instructions.md` (redact-before-truncate, no key interpolation in
  logs, no redirect-following on credentialed clients) remain owned by humans unless the probe
  proved otherwise for a specific one.

### Claude's Discretion

Downstream research and planning decide freely: CodeQL build mode and query suite selection
(`security-extended` vs default), the exact schedule cadence, cache strategy, the file layout of
the probe crate, whether the triage register is a new file or a section, and the size of the
backfill sample in D-14. None of these were constrained by the discussion.

### Folded Todos

None folded. See Reviewed Todos below.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements and phase intent
- `.planning/ROADMAP.md` §"Phase 18: Rust SAST — Evaluate and Adopt CodeQL" — goal and the six
  success criteria, including the criterion-6 rule that nothing may make a green result mean less
  than it says.
- `.planning/REQUIREMENTS.md` §"Requirements — Security Tooling (Phase 18)" — `SAST-01` … `SAST-04`
  in full, with the Snyk baseline numbers and the cost note that code scanning is already enabled.
- `.planning/PROJECT.md` §"Current Milestone: v0.9.0 Security Tooling" (lines ~106-145) — the
  carried-in context: CodeQL primary, Semgrep complement, the Snyk lesson as governing constraint,
  and the three backwards couplings.
- `.planning/v0.8.0-MILESTONE-AUDIT.md` — names this as the milestone's one genuinely open item.

### The Snyk precedent (the methodology being reused)
- `.github/instructions/security.instructions.md` §"Snyk was evaluated and removed (2026-08-18)"
  and §"Known gap: no Rust SAST" — the probe design, the 0-vs-3 result, the standing prohibition,
  and the exact section `SAST-04` requires rewritten.

### Trigger-surface and required-check governance (constrains the wiring)
- `scripts/check-workflow-triggers.sh` — read the header comment (lines 1-70) and
  `EXCEPTION_FILES` at line 118. Clause 1 (every workflow needs a register row), Clause 2 (push
  filter must be `['**']` outside two exceptions), Clause 3 (every pinned context must resolve to
  a declared job name).
- `docs/src/contributing/branching-model.md` §"trigger-policy register" (table at lines 47-54) —
  the table a new `codeql.yml` row must join, and its formatting constraint.
- `.github/rulesets/protect-main-branch.json` — the 44 pinned contexts; becomes 45 on promotion.
- `docs/src/appendix/branch-protection.md` — the context table plus the count at lines 85, 117 and
  180; the ruleset-application procedure.
- `.planning/decisions/0043-github-flow-trunk-and-trigger-surface.md` — ADR behind the trigger
  surface.
- `.planning/decisions/0044-branch-protection-posture.md` — ADR behind what is and is not required.

### Governance shape for suppressions/dismissals
- `SECURITY-EXCEPTIONS.md` — the eleven-field governed-register pattern D-17 mirrors.
- `scripts/check-advisory-register.sh` and `.planning/decisions/0036-audit-suppression-single-source-topology.md`
  — the register-plus-enforcing-guard shape.

### Existing CI to integrate with
- `.github/workflows/ci.yml` lines 1-40 (trigger and concurrency comments — the `['**']`
  rationale), lines 155-181 (`osv-scanner`: the existing `github/codeql-action/upload-sarif@v3`
  wiring and the `continue-on-error` pattern D-06 deliberately does **not** copy).

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets
- **`github/codeql-action/upload-sarif@v3` is already wired** in `ci.yml`'s `osv-scanner` job
  (line 178). Code scanning is enabled on this repository today; SARIF ingestion is proven. No
  token, vendor account, or licence is needed — the repository is public.
- **`concurrency` group pattern** (`ci.yml` lines 26-33): `ci-${{ github.head_ref || github.ref }}`
  with `cancel-in-progress` on non-`main` refs. Copy this to absorb the duplicate push+PR runs that
  D-04's `['**']` push filter creates.
- **`SECURITY-EXCEPTIONS.md` + `scripts/check-advisory-register.sh`** — a working
  governed-register-plus-guard pair to model D-17 on.
- **`scripts/check-workflow-suppressions.sh`** — the register/guard shape that
  `check-workflow-triggers.sh` itself was modelled on; useful as a second reference implementation.

### Established patterns that constrain this phase
- **Every workflow file must have a trigger-policy register row** (Clause 1) — a new workflow
  committed without one turns `main` red.
- **`push: ['**']` is the house rule**, with exactly two recorded exceptions. D-04 conforms rather
  than minting a third.
- **Required contexts are pinned by job display name** — the only thing GitHub matches on — so job
  names are contracts (Clause 3).
- **`actionlint` (`Workflow Lint`) and `pre-commit run --all-files` are required checks** and will
  both see the new workflow and the probe fixture.
- Guard scripts here are offline, read-only, idempotent, and accumulate all violations before
  reporting — match that style if D-17 adds one.

### Integration points
- New file `.github/workflows/codeql.yml`.
- New row in `docs/src/contributing/branching-model.md`'s trigger-policy table (same commit).
- New probe crate, excluded from the workspace `members` list in the root `Cargo.toml`.
- On promotion: `.github/rulesets/protect-main-branch.json` (44 → 45), live ruleset `20868126`,
  `docs/src/appendix/branch-protection.md`.
- On close: `.github/instructions/security.instructions.md`, plus the deferred-item row in
  `.planning/STATE.md` and `.planning/MILESTONES.md`.

### Stale map warning
`.planning/codebase/INTEGRATIONS.md` (dated 2026-07-30) states "CI Pipeline: Not integrated in
this codebase". That is wrong — six workflows exist, `ci.yml` alone is 51 KB with 22 jobs and 44
required contexts. Read `.github/` directly; do not trust that map for CI facts.

</code_context>

<specifics>
## Specific Ideas

- **The Snyk probe is reused verbatim, not improved.** Same four vulnerability classes, so the
  finding count is directly comparable to the recorded 0-in-Rust / 3-in-JavaScript baseline. A
  "better" probe produces a number that cannot be compared to anything.
- **"Analysed 0 files" and "found 0 issues" must be distinguishable in the evidence.** The entire
  Snyk failure was that these two look identical in a report. D-13's analysed-file count is what
  separates them.
- **The disqualified outcome is a first-class deliverable.** Plans should carry both branches, not
  treat rejection as an error path.

</specifics>

<deferred>
## Deferred Ideas

- **Remediating real findings.** If CodeQL surfaces genuine defects in first-party code, they are
  triaged and recorded in this phase but fixed in follow-up work — mixing a scanner-adoption phase
  with an unbounded remediation phase makes both unshippable.
- **Adding a GitHub-facing `SECURITY.md`.** `SECURITY-EXCEPTIONS.md` explicitly records this as a
  separate deliverable for a different audience (external researchers vs. internal auditors).
  Still true; still separate.
- **Semgrep as a standing complement alongside a qualified CodeQL.** D-20 scopes Semgrep to the
  contingency path only. Running both permanently is a defensible future choice, not this phase's.
- **Refreshing `.planning/codebase/INTEGRATIONS.md`'s CI section.** Its "CI Pipeline: not
  integrated" claim is stale by six workflows. Out of scope here; worth a `/gsd-map-codebase` pass.

### Reviewed Todos (not folded)
- **"Verify local `make coverage` reproduces CI's 82.39% figure"**
  (`2026-08-13-verify-local-coverage-reproduction.md`, area `testing`, match score 0.60).
  **Not folded.** The matcher scored it on generic keyword overlap (`verify`, `coverage`, `docs`,
  `src`, `github`) — "coverage" here means *test-line coverage*, an unrelated sense from this
  phase's *scan coverage*. Folding a test-coverage reproduction task into a SAST-adoption phase is
  scope creep, and the workflow's scope guardrail outranks the score threshold that would
  otherwise have auto-folded it under `--auto`. Recorded here so it is not silently dropped.

</deferred>

---

*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Context gathered: 2026-08-25*
