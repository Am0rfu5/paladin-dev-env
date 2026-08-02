# Phase 4: Release Coherence - Context

**Gathered:** 2026-08-02
**Status:** Ready for planning

> **Auto-resolved.** This context was produced by `/gsd-discuss-phase 4 --auto`. Every gray area
> was selected and every question answered with the recommended option; no user prompts were
> issued. The per-question audit trail is in `04-DISCUSSION-LOG.md`. Decisions marked
> **`one-way`** or that create an ADR are the ones a human should skim before planning.

<domain>
## Phase Boundary

The v0.7.0 release tells **one story**. A developer can clone the release tag, build it, trust the
version and the dependency posture, follow QUICKSTART to a working agent, and see the gate suite
prove all of it.

**This phase's centre of gravity is coherence, not construction.** Phase 1 wrote records, Phase 2
edited product code, Phase 3 measured. Phase 4 makes the *release artefacts* agree with each other
and with the tree — and, where they cannot be proven in this environment, records the gap honestly
with a named owner rather than claiming it.

**Five deliverable classes:**

1. **One version.** Workspace `Cargo.toml`, every member crate, the git tag, `CHANGELOG.md` and the
   release notes converge on a single figure (D-01), with the trajectory `v0.1.0-rc.1 → 0.3.0 →
   0.4.0 → 0.5.0 → 0.5.1 → 0.6.0 → 0.7.0` in view rather than a fragment of it.
2. **One edition.** The `edition = "2024"` / `"2021"` split across eleven manifests is closed in one
   direction, recorded as an ADR, and the stale `CONCERNS.md` claim that motivated the opposite
   direction is corrected at source (D-04, D-05).
3. **A measured, defensible advisory posture.** `cargo audit` and `cargo deny check` run and their
   verdicts are *recorded* to the Phase 3 provenance standard, the one stale suppression is removed,
   the newly-surfaced advisories are recorded as a dated finding, and the owner/expiry governance is
   left to its named owners (D-06 … D-09).
4. **The QUICKSTART measurement, taken for the first time** — pass or fail — under stated
   environment conditions, with the contested target figure settled to one number (D-10, D-11).
5. **The gate suite, proven where it can be and gated where it cannot.** Everything runnable
   locally is run and recorded; the CI configuration is repaired so it *can* prove the rest on the
   release branch; Docker and Kubernetes gates are deferred with reason and an owner (D-12 … D-16).

**Not in this phase:**

- Deleting the duplicate `ci.yml:389-406` audit job — **SUPPLY-01, Phase 12** (measured non-blocking
  here, see D-08).
- The RustSec owner/expiry schema, the 2026-09-30 risk-acceptance disposition, and the three
  unratified 2026 ignores — **SEC-01 (Phase 9) / SUPPLY-02 (Phase 12)**.
- Fixing the permanently-red `api-surface` CI job — **DEBT-01, Phase 8**.
- Re-enabling `paladin-ports` doctests (`doctest = false`) — **DEBT-03, Phase 8**; the `cargo doc`
  bar that governs it is **HARD-07, Phase 10**.
- The `cli-tests`, `bench-check` and `coverage` CI jobs, `.codecov.yml`, the Makefile coverage
  targets and the eight deprecated GitHub Actions — **PIPE-01 … PIPE-04, Phase 15**.
- Pushing the tag or publishing to crates.io — **a human action, not an agent action** (D-03).
- Any new product capability. This phase edits manifests, CI configuration, docs and records.

</domain>

<decisions>
## Implementation Decisions

### Version convergence (REL-01, SC1)

- **D-01: Phase 4 converges on `0.7.0`, and records the answer itself rather than waiting for
  Phase 7.** The three-way disagreement is verified live: branch `release/v0.7.0`, root
  `Cargo.toml` and all eleven member manifests at `0.6.0`, latest tag `v0.5.1`. `0.7.0` is chosen
  because it is (a) the branch's own declared intent, (b) the next lockstep minor after the
  `M9 → 0.3.0 / M10 → 0.4.0 / M11 → 0.5.0 / M12 → 0.6.0` chain ORCH-05 records, and (c) **not an
  `rc.1` figure**, which HARD-03 explicitly forbids REL-01 from converging on.

  The recorded coupling table assigns "whether Milestone 6 forces a major version bump" to
  **ARCH-04 (Phase 7)**, with REL-01 applying it. **Phase 4 runs first, so Phase 4 records it** —
  the same "whichever of Phase 4 / Phase 7 executes first records the answer, the other applies it"
  convention REL-02's own text already states for the edition. The recorded reasoning must state
  plainly: the Milestone 6 facade change *was* breaking, but it **already shipped inside the 0.x
  series** (`src/application/use_cases/` no longer exists in a tree that has been publishing since
  `v0.1.0-rc.1`), and under SemVer a pre-1.0 project expresses breaking changes as minor bumps. A
  `1.0.0` bump would additionally assert API stability this corpus has not established.
  Chosen over `1.0.0` (asserts a stability guarantee nothing in the corpus supports, and would make
  every future breaking change a major bump in a project still resolving port ownership) and over
  blocking on Phase 7 (a nine-phase-deep circular wait: ARCH-04 → REL-01 → ARCH-04, on a phase whose
  entire purpose is ending three-way disagreements).
  — **Reversibility:** `one-way` once the tag is pushed and the crates are published — ten crates
  at a lockstep version on crates.io cannot be unpublished, only yanked. Reversible right up to
  that point, which is exactly why D-03 puts the push behind a human.

- **D-02: An ADR records the version answer, so Phase 7's ARCH-04 and Phase 10's HARD-03 inherit it
  rather than re-deciding it.** Every ADR in `.planning/decisions/` carries a `Code conformance`
  field (`conforms` / `must change`); this one is `must change`, naming REL-01 as executor. Without
  the ADR, the answer sits at DOC precedence and the next document that mentions a version
  auto-overrides it — the precise failure mode PROJECT.md's "not one protected decision in twelve
  milestones" finding names. Number it as the next free ADR after `0007`.

- **D-03: Phase 4 prepares the release; it does not push the tag or publish.** Concretely: bump all
  eleven manifests plus `[workspace.dependencies]` internal pins to `0.7.0`, finalize `CHANGELOG.md`
  (move `## [Unreleased]` under a new dated `## [0.7.0] - <date>` heading **and give the existing
  undated `## [0.6.0]` heading its date** — it is the only version heading in the file without
  one), and create the annotated tag `v0.7.0` **locally, unpushed**. Pushing a `v*.*.*` tag triggers
  `.github/workflows/release.yml`, which publishes ten crates to crates.io in dependency order —
  irreversible and outward-facing. `release.toml` already sets `publish = false` / `push = false`,
  so the local `cargo release version` flow is safe; `make release` is **not**, because it pushes.
  The plan records the exact push+publish command sequence as a documented human gate.
  — **Reversibility:** `reversible` as specified (a local tag is one `git tag -d`); the step it
  deliberately stops short of is the `one-way` one.

### Rust edition (REL-02, SC2)

- **D-04: Standardize on `edition = "2024"` — bump the two stragglers, do not downgrade the nine.**
  Verified live: the root `paladin-ai` package and nine crates declare `2024`; exactly
  `crates/paladin-ports` and `crates/paladin-notifications` declare `2021`. Moving two manifests
  forward is a two-line change against a 38-file surface; moving nine backward is a workspace-wide
  regression that would also have to reconcile any 2024-only syntax already in those nine crates.
  Chosen over standardizing on `2021` (which is what `codebase/CONCERNS.md:25` recommends — see
  D-05 for why that recommendation is void) and over leaving the split (SC2 forbids it).

- **D-05: `codebase/CONCERNS.md`'s edition finding is factually wrong at the pinned toolchain and is
  corrected at source with dated provenance.** It states (line 9) that `edition = "2024"` *"does not
  exist in Rust's stable channel. Rust only defines editions 2015, 2018, and 2021."* **Rust 2024
  stabilized in Rust 1.85; `rust-toolchain.toml` pins `1.97.1`** (verified: `rustc 1.97.1
  (8bab26f4f 2026-07-14)`), and the workspace builds under 2024 today. The map's "current build
  succeeds possibly via lenient parsing" hypothesis is void — it succeeds because the edition is
  real. This is the precedence order applied at full strength (shipped tree beats
  `.planning/codebase/` map), and the amendment follows the same in-place, dated convention Phase 1,
  2 and 3 used. **Do not plan an edition decision that cites the stale claim.**

- **D-06: The edition answer is recorded as an ADR carrying ARCH-03(a)'s answer, and REL-02 applies
  it in the same phase.** REL-02's text authorizes exactly this ("Whichever of Phase 4 / Phase 7
  executes first records the answer, the other applies it"). The ADR must note that Milestone 5
  Epics 1-4 require 2021 while Epic 5 and the milestone overview require 2024 — neither the code
  nor the record was self-consistent — and that the documents are **amended**, not merely
  overridden. Phase 7's ARCH-03(a) then cites the ADR instead of re-adjudicating.
  Proof obligation: `cargo build --workspace` **and** `cargo build --workspace
  --no-default-features` must both succeed after the bump, recorded with the D-17 provenance block.
  Watch for the 2024 migration's real breaking changes in those two crates —
  `unsafe_op_in_unsafe_fn`, `static_mut_refs`, RPIT lifetime capture, and `gen` becoming a reserved
  keyword. `cargo fix --edition` is the documented migration route.

### Advisory posture (REL-03, SC3) — what this phase owns

- **D-07: SC3's first half is already true, and Phase 4's job is to *record it as measured*, not to
  fix it.** Run live during this discussion at HEAD `68ba809`:
  - `cargo audit` → **0 vulnerabilities**; `warning: 13 allowed warnings found`; advisory DB fetched
    successfully from GitHub (1186 advisories loaded).
  - `cargo deny check` → **`advisories ok, bans ok, licenses ok, sources ok`**.

  Both tools are installed and both work in this sandbox despite crates.io returning HTTP 403 —
  the advisory database is a GitHub repository, and github.com is reachable. **This is a material
  difference from Phase 3's environment constraints and should not be re-litigated.** The
  deliverable is a dated measurement record to the D-17 standard, not a remediation campaign.
  REQUIREMENTS.md's REL-03 "Current:" line (2 medium advisories, 3 feature-gated, 10 unmaintained,
  dual `reqwest`) describes a *suppression inventory*, not a failing gate — the plan must not read
  it as five open defects.

- **D-08: One stale suppression is removed; the duplicate CI audit job is measured, recorded, and
  left to its owner.** Two concrete findings:
  1. `cargo deny check` emits `warning[advisory-not-detected]` at `deny.toml:136` —
     **`RUSTSEC-2025-0121` (gcc) no longer matches any crate in the graph.** A suppression for an
     advisory that cannot fire is exactly the "silent suppression" SC3 forbids. Remove it, and check
     the other fourteen entries the same way.
  2. `ci.yml` runs two jobs named `Security Audit` (`:60-77` bare, `:389-406` with two inline
     `--ignore` flags). **Verified live that this does not block SC5:** cargo-audit's `--ignore`
     flags *augment* `.cargo/audit.toml` rather than replacing it, so the `:389-406` job also exits
     `0` on this tree. Phase 4 records that measurement and leaves the 18-line deletion to
     **SUPPLY-01 (Phase 12)**, which owns it and for which it makes a Milestone 10 acceptance
     criterion true.

- **D-09: Four newly-surfaced advisories are recorded as a dated finding and handed to SEC-01 /
  SUPPLY-02 — not suppressed here.** Surfaced by the current advisory DB and present in neither
  `.cargo/audit.toml` nor `deny.toml`: `RUSTSEC-2021-0145` (atty, unsound — distinct from the
  already-ignored `-2024-0375` unmaintained notice for the same crate), `RUSTSEC-2026-0221`
  (event-listener, unsound), `RUSTSEC-2026-0205` (scc, unsound), and `spin 0.9.8` **yanked**. None
  fails either gate today (they are `unsound`/`yanked` warnings, not vulnerabilities). Adding
  suppressions for them would be new governance decisions inside a phase whose governance owner is
  someone else. Record with dates and dependency paths; owner **SEC-01 (Phase 9) / SUPPLY-02
  (Phase 12)**.
  What *is* Phase 4's: SC3's second half — **every surviving ignore carries a written rationale plus
  a migration or review note.** Both files already carry rationale for all fifteen; the audit is
  whether each also carries a *migration or review note* ("revisit when X upgrades" qualifies; a
  bare reason does not). Add the missing notes. **Do not add owner or expiry fields** — that schema
  is SUPPLY-02's, and inventing a second one here would create the exact duplicate-governance
  problem this corpus keeps closing.

### Documentation review and the QUICKSTART measurement (REL-04, SC4)

- **D-10: REL-04's "documentation final review is complete per the RECON-08 answer" is already
  discharged — do not invent the review.** RECON-08's recorded verdict
  (`.planning/ledgers/milestone-01.md` §"Epic 10 Task 7.0 — dispute resolution") is that **the
  validation report is wrong and no Task 7.0 exists**: the task list is 103/103 with no Task 7.0
  heading, independently corroborated by `intel/task-completion-state.md`, and no artifact anywhere
  in the 263-document corpus or in `docs/` supplies content for a "Final Documentation Review".
  Epic 10 is classified `satisfied` on this point with **no owner assigned because none is needed**.
  Phase 4 cites that verdict and moves on. **The live half of REL-04 is the QUICKSTART measurement
  and nothing else.**

- **D-11: The documented target is settled at 15 minutes, the tighter in-page claim is reconciled,
  and the measurement is recorded pass or fail under stated conditions.** Two defects and one
  constraint:
  1. **The target is contested inside the shipped docs.** `docs/src/introduction.md:9` says "in 15
     minutes"; `docs/src/getting-started/quickstart.md:3` says *"under five minutes"*. REL-04 and
     ROADMAP SC4 both name **< 15 minutes**. Settle on **15 minutes** as the gate — it is the figure
     both planning documents carry and the one two of three doc references support — and amend
     `quickstart.md:3` to match, unless the measurement comes in under five, in which case the page
     keeps its claim and the measurement is the evidence for it. Decide **after** measuring.
  2. **"Clean machine" is not reachable here** and must not be faked. crates.io returns HTTP 403,
     Docker is absent, and the cargo registry is already warm — a cold-start dependency fetch cannot
     be timed in this sandbox. Measure what *can* be measured (the documented steps end to end from
     the current tree to a working agent), state the environment plainly in the record — warm
     registry, no network to crates.io, no Docker, this machine's CPU/kernel — and label it
     **"measured under stated conditions, not a clean-machine claim."** File a `deferred with
     reason` row for the true clean-machine timing with a named owner.
  3. ROADMAP SC4 says "**measured for the first time, pass or fail**". A fail is a legitimate,
     recordable outcome. Do not tune the QUICKSTART to hit a number.

### The gate suite (REL-05, SC5)

- **D-12: Run locally everything that can run locally, and record each with the provenance block.**
  In scope and confirmed feasible: `cargo fmt --check --all` (**verified clean at HEAD during this
  discussion**), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
  (Phase 2's verification recorded 2864 passed / 0 failed on this tree; re-run, do not cite),
  doc tests, and building **every example target**. Use `--offline` throughout, per Phase 3's D-16.

- **D-13: The "22 examples" figure is stale; re-derive it from the tree and amend at source.**
  Verified: `examples/` contains **47 `.rs` files** (4 declared as `[[example]]` targets for
  feature-gating, the rest auto-discovered); no crate under `crates/` has an `examples/` directory.
  The "22 examples" figure traces to a Milestone 1 Epic 10 validation report ("22/22 examples
  compiling", `PROJECT.md:767`) and has been restated in `ROADMAP.md:6`, `ROADMAP.md:313`,
  `PROJECT.md:21`, `PROJECT.md:136` and `REQUIREMENTS.md:382`. This is Phase 3's D-03 pattern
  exactly — an ingested count losing to a measurement of the shipped tree. **Gate on "every example
  target builds", not on a count**, and amend the five restatements at source with dated provenance.

- **D-14: CI cannot currently prove SC5 on the release branch, and repairing that is in scope.**
  Three verified configuration gaps, all of which make SC5 unprovable as written rather than merely
  unproven:
  1. **`ci.yml` has no `push` trigger** — line 9's `branches: [main, develop, 'feature/**']` is
     commented out. The workflow fires only on `pull_request` to `main`/`develop` and on
     `workflow_dispatch`. **A push to `release/v0.7.0` runs nothing.** Restore a push trigger
     covering `release/**` (uncommenting the original line is not sufficient — it omits
     `release/**`).
  2. **No job builds the examples.** `grep example .github/workflows/` returns only
     `crates/doc-examples` paths in `docs.yml` and a `.env.example` copy. SC5's "all 22 examples"
     is asserted by nothing.
  3. **No Kubernetes smoke-test job exists, and the Docker job asserts no budget.** `ci.yml`'s
     `docker` job (`:409-434`) builds a single-platform image with no size or time assertion, and
     `grep -l "kind create cluster\|kubectl apply" .github` matches only
     `integration-tests.yml`. SC5 names "the multi-arch Docker build inside its size and time
     budget" (< 500 MB / < 5 min) and "the Kubernetes smoke test inside its startup budget"
     (< 30 s) — neither is expressed in CI.

  Adding these three is **SC5's own scope**, not scope creep: SC5 requires CI to *prove* them.
  It is distinct from Phase 15's PIPE register, which owns the `cli-tests`, `bench-check` and
  `coverage` jobs, `.codecov.yml` and the deprecated-actions sweep — none of which is touched here.

- **D-15: The Docker and Kubernetes gates are written, lint-verified, and then deferred with reason
  — never claimed green.** `docker`, `kind` and `kubectl` are all **absent from this environment**
  (verified), so the new jobs cannot be executed locally. Validate them statically (YAML parse,
  action-reference sanity, `Dockerfile` and `k8s/` manifest references resolve) and file a
  `deferred with reason` row: *"authored and statically validated; first execution requires a CI
  runner with Docker"*, owner **Phase 15 / PIPE**. **Do not fabricate a green CI run, and do not
  report SC5 as met on the strength of configuration alone.** This is the single largest honesty
  risk in the phase.

- **D-16: `gh` is available and the remote is live — use it to *read* CI state, not to trigger
  runs.** Verified: `gh 2.96.0`, remote `https://github.com/DF3NDR/paladin-dev-env.git`. Reading
  workflow-run history for the release branch is legitimate evidence and cheaper than inference.
  Dispatching a workflow run, pushing a branch or opening a PR is an outward-facing action and
  stays behind the same human gate as D-03.

### Measurement provenance — applies to every measurement in this phase

- **D-17: Every figure carries the Phase 1/Phase 3 provenance block, and every cargo command carries
  `--offline` unless it needs the advisory DB.** `rustc -vV`, `cargo --version`,
  `git rev-parse HEAD`, `date -u`, plus raw pasted stdout and arithmetic a reader can re-derive.
  `01-coverage-measurement.md` and `03-coverage-measurement.md` are the templates. The exception is
  `cargo audit`, which must reach `github.com/RustSec/advisory-db` — record the DB's advisory count
  and fetch date as part of the provenance so a later reader can tell which snapshot produced the
  verdict.

### Claude's Discretion

- **Plan decomposition and count.** The natural dependency shape is: edition (D-04) and version
  (D-01) are independent of each other but both gate the "build + gate suite" proof (D-12), which
  in turn should precede the tag (D-03) so the tag lands on a green commit. Whether the two ADRs are
  one plan or two, and whether the CI repair (D-14) runs parallel to the manifest work, is the
  planner's call.
- **Where the ROADMAP / REQUIREMENTS / PROJECT amendments under D-05 and D-13 physically land** —
  in-place edits with provenance notes (Phase 2's route) or in-place plus a Phase 4 amendments
  record (Phase 3's route). Pick one and be consistent.
- **Whether the `v0.7.0` tag is created locally at all**, or whether SC1 is discharged by
  manifests + CHANGELOG + a documented tag command. D-03 recommends creating it locally unpushed;
  a planner who judges even a local tag too close to the release trigger may substitute the
  documented-command form, and must record which.
- **Whether the dual `reqwest` 0.12.28 / 0.13.4 exposure gets a written note in this phase.**
  Verified present in `Cargo.lock`. `codebase/CONCERNS.md:293-303` already documents it and
  `deny.toml` treats duplicates as a warning by deliberate policy (FR 12 / Open Question 4). It is
  not an advisory and does not fail SC3 — either record it as a known, policy-accepted duplicate or
  leave it to CONCERNS.md.
- **Whether `## [Unreleased]`'s "Phase 12.1" heading is renamed during the CHANGELOG finalize.**
  It refers to the *old `.project/` milestone* numbering, not a GSD phase, and a reader in
  `.planning/` will misread it. Renaming is cosmetic; leaving it is defensible if a provenance note
  is added.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### The recorded answers this phase must not re-litigate

- `.planning/ledgers/milestone-01.md` §"Epic 10 Task 7.0 — dispute resolution (RECON-08)"
  (lines ~142-240) — **the reason REL-04 has no documentation-review deliverable** (D-10). The
  verdict is `satisfied`, no owner needed. Read before planning any doc-review task.
- `.planning/decisions/PROMOTION.md` — ADR house conventions, the numbering index, and the
  `Code conformance` field convention. **Two new ADRs land in this phase** (D-02 version, D-06
  edition); `0007` is the highest existing number.
- `.planning/decisions/0006-coverage-gate.md` — not this phase's gate, but the model for how a
  contested number gets one recorded answer with a verbatim command and a scope.

### Requirements and roadmap — including what this phase amends

- `.planning/REQUIREMENTS.md:353-386` — **REL-01 … REL-05 in full** with `Derives:` provenance and
  each requirement's verified "Current state" line.
- `.planning/REQUIREMENTS.md:3896-3900` — the REL-01 … REL-05 Traceability rows, all `Pending`.
- `.planning/REQUIREMENTS.md:4004-4022` — **the cross-phase coupling table.** The four rows that
  bind this phase: ARCH-03(a) → REL-02 (edition), ARCH-04 → REL-01 (major bump), HARD-03 → REL-01
  ("must not converge on an rc.1 figure"), HARD-03 → ORCH-05 → REL-01 (the version trajectory).
- `.planning/REQUIREMENTS.md:978-991` — **HARD-03**, the version history: `v0.1.0-rc.1` at commit
  `a9530fc`, ten crates published at `0.1.0`, and the v0.3.0 → v0.5.1 sequence.
- `.planning/REQUIREMENTS.md:1372-1391` — **ORCH-05**, the lockstep chain M9 → v0.3.0, M10 → v0.4.0,
  M11 → v0.5.0, M12 → v0.6.0, terminating exactly where the tree is. **D-01's evidence base.**
- `.planning/REQUIREMENTS.md:549-568` — **ARCH-03**, whose (a) clause is the edition answer D-06
  records on Phase 7's behalf.
- `.planning/REQUIREMENTS.md:570-583` — **ARCH-04**, the facade re-export policy and its
  "breaking change requiring a major version bump" question. **D-01 answers it.**
- `.planning/ROADMAP.md` §"Phase 4: Release Coherence" (lines 302-315) — the five success criteria.
  **Criterion 5's "all 22 examples" is stale (D-13).**
- `.planning/ROADMAP.md` §"Phase 12" and §"Phase 9" — SUPPLY-01/02 and SEC-01, the owners of
  everything D-08 and D-09 hand off.
- `.planning/PROJECT.md` §Constraints — the deploy budgets (< 500 MB image, < 30 s pod startup),
  the licence three-way (SEC-02 settles it — **not this phase**), the advisory-governance paragraph
  that scopes D-07 … D-09, and the edition paragraph that D-05 corrects.
- `.planning/PROJECT.md` §Context (~lines 735-800) — the precedence order
  (**ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
  task-list checkbox**), the "version state is incoherent right now" paragraph, the two-`Security
  Audit`-jobs mechanism, and the "22/22 examples" claim's origin.

### Prior-phase decisions this phase inherits

- `.planning/phases/03-verification-depth/03-CONTEXT.md` — **D-16 (the provenance block, inherited
  as D-17)**, D-02 (do not quietly change a measured command), and D-09's note that the Commander
  mock was deliberately kept out of a new workspace crate *because* Phase 4 has to reconcile every
  crate version. Its `<deferred>` section names "version, edition and advisory coherence" as this
  phase's.
- `.planning/phases/03-verification-depth/03-coverage-measurement.md` — the provenance template at
  its most recent revision.
- `.planning/phases/01-ground-truth-decision-records/01-CONTEXT.md` — **D-19's evidence bar
  (`file:line` plus a named passing exerciser)** and D-20's five verdict classes, which every
  ledger row this phase writes must satisfy.
- `.planning/phases/02-functional-gap-closure/02-CONTEXT.md` — D-02, the amend-the-ledger-in-place
  convention D-05 and D-13 follow.

### Release machinery this phase edits

- `Cargo.toml` (root) — `[package] version = "0.6.0"`, `edition = "2024"`,
  `[workspace] members = [".", "crates/*"]`, `[workspace.dependencies]` internal pins, four
  `[[example]]` declarations, three `[[bin]]` targets.
- `crates/*/Cargo.toml` — eleven manifests, all at `0.6.0`. **`crates/paladin-ports/Cargo.toml:4`
  and `crates/paladin-notifications/Cargo.toml:4` are the two `edition = "2021"` stragglers**
  (D-04). Note `paladin-ports` also carries `doctest = false` with a "re-enable in Task 7.0" comment
  — **that is DEBT-03, Phase 8, not this phase.**
- `CHANGELOG.md` — `## [Unreleased]` at line 8, `## [0.6.0]` at line 63 **with no date** (every
  other version heading has one), `## [0.5.1] - 2026-06-04` at line 359.
- `release.toml` — `shared-version = true`, `tag-name = "v{{version}}"`,
  `consolidate-commits = true`, `publish = false`, `push = false`. Documents that `make release`
  **does** push, and that pushing the tag is what triggers publication.
- `.github/workflows/release.yml` (428 lines) — the publish pipeline that a pushed `v*.*.*` tag
  triggers. **Read before creating any tag.**
- `rust-toolchain.toml` — `channel = "1.97.1"`, and the note that it overrides whatever toolchain a
  CI action installed.

### Gates and their configuration

- `.github/workflows/ci.yml` (644 lines) — **line 9's commented-out `push` trigger** (D-14.1);
  `lint` `:20`, `security-audit` `:60`, `cargo-deny` `:80`, `osv-scanner` `:110`,
  `api-surface` `:139` (**the permanently-red job — DEBT-01, Phase 8**), `test` `:190`,
  `crate-isolation` `:228`, `integration-tests` `:283`, **`security` `:390` (the duplicate audit
  job — SUPPLY-01, Phase 12, `ci.yml:389-406`)**, `docker` `:409` (**no budget assertion**),
  `e2e-tests` `:438`, `benchmark` `:498`, `publish-dry-run` `:617`.
- `deny.toml` — `[licenses]` allow-list plus eight MPL-2.0 per-crate exceptions, `[bans]` with the
  `actix-web` denial, and `[advisories] ignore` at lines ~112-148. **Line 136's
  `RUSTSEC-2025-0121` is the stale entry cargo-deny flags (D-08.1).**
- `.cargo/audit.toml` — the five-vulnerability ignore list and its rationale comments; the
  authoritative source that `deny.toml` mirrors.
- `Dockerfile`, `docker/docker-compose.yml`, `k8s/` — the artefacts D-14.3's new jobs must exercise.
- `Makefile` — `make release`, `make audit`, `make deny`, `make clean-code`.

### Documentation under REL-04

- `docs/src/getting-started/quickstart.md` (127 lines) — **line 3 claims "under five minutes"**
  (D-11.1). The page whose steps get timed.
- `docs/src/introduction.md:9` — "Get your first Paladin agent running in **15 minutes**", the
  competing figure.
- `docs/src/appendix/performance-baseline.md` — amended by Phase 3; the model for adding a dated
  measurement section without overwriting the prior one.

### Code-state intelligence

- `.planning/codebase/CONCERNS.md` — **§"Edition 2024 in Project Manifests" (lines 7-25) is
  factually wrong at the pinned toolchain and is amended by D-05.** §"Unmaintained Dependencies
  Ignored in cargo-deny" (255-284) and §"reqwest Dual Version Risk" (293-303) are accurate and
  scope D-07/D-09. Audit dated 2026-07-30.
- `.planning/codebase/STACK.md` — the toolchain, feature-flag matrix and dependency posture.
- `.planning/intel/code-verification.md` — third in the precedence order; source of standing
  "do not plan this" instructions.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`cargo audit` and `cargo deny` both work in this environment.** `cargo-audit 0.22.2` and
  `cargo-deny 0.19.8` are installed, and the advisory DB fetches from GitHub successfully despite
  crates.io returning HTTP 403. **This is the phase's most consequential environmental finding** —
  SC3 is measurable here, unlike Phase 3's coverage tooling.
- **`release.toml` already encodes the lockstep model** — `shared-version = true`,
  `consolidate-commits = true`, `publish = false`, `push = false`. `cargo release version 0.7.0`
  bumps every member plus the internal `workspace.dependencies` pins in one step. The manifest work
  in D-03 is a tool invocation, not eleven hand edits.
- **`gh 2.96.0` is installed** and `origin` points at the live repo — workflow-run history is
  readable without inference (D-16).
- **The Phase 1 / Phase 3 measurement records** (`01-coverage-measurement.md`,
  `03-coverage-measurement.md`) are directly reusable as the provenance template for every figure
  this phase records (D-17).
- **`cargo fmt --check --all` is already clean at HEAD `68ba809`** — verified during this
  discussion. One of the five SC5 gates needs recording, not fixing.

### Established Patterns

- **Precedence is the project's core mechanic**: ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox. **D-05 and D-13 are this rule
  applied at full strength** — a stale codebase-map claim and an ingested example count both lose
  to a measurement of the shipped tree.
- **Amend at source with dated provenance.** Phase 1 amended Phase 3's criterion 1; Phase 2 amended
  its own criteria and the ledger in place; Phase 3 amended QUAL-02, QUAL-03 and ROADMAP criterion 2.
  D-05 and D-13 follow the same route. Never silently substitute a corrected figure.
- **Every ADR carries `Code conformance`** (`conforms` / `must change`, naming the executing
  requirement). Both new ADRs are `must change`.
- **"Deferred with reason" needs a named owner.** The corpus's five verdict classes (Phase 1's
  D-20) apply to every row this phase writes, including D-11's clean-machine deferral and D-15's
  Docker/Kubernetes deferral.
- **Medieval military ubiquitous language is mandatory** — including in CHANGELOG prose and any new
  CI job names.
- **Repo working agreement**: `cargo test` → `cargo fmt --check` → `cargo clippy` before committing;
  no `unwrap()`/`expect()`/`panic!` in library code; conventional commits.

### Integration Points

- **Eleven `Cargo.toml` manifests plus `[workspace.dependencies]`** — the version bump (D-01/D-03)
  and the two edition bumps (D-04).
- **`CHANGELOG.md`** — the `[Unreleased]` → `[0.7.0]` finalize, plus the missing `[0.6.0]` date.
- **`deny.toml` `[advisories] ignore`** — one stale entry removed, migration/review notes completed.
- **`.github/workflows/ci.yml`** — push trigger for `release/**`, a new examples job, budget
  assertions on the docker job, a new Kubernetes smoke job.
- **`docs/src/getting-started/quickstart.md`** — the timing target reconciled (D-11).
- **`.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`,
  `.planning/codebase/CONCERNS.md`** — amended at source per D-05 and D-13.
- **`.planning/decisions/`** — two new ADRs (version, edition), plus index/PROMOTION updates.
- **`.planning/ledgers/milestone-01.md`** — REL-01 … REL-05 rows, the deferrals, and the
  hand-off rows to SEC-01 / SUPPLY-01 / SUPPLY-02 / DEBT-01 / DEBT-03 / PIPE.
- **Phase 7's ARCH-03(a) and ARCH-04** — *receive* answers from this phase instead of producing
  them (D-02, D-06). Their requirement text must be updated to cite the ADRs.
- **Phase 9's SEC-01 and Phase 12's SUPPLY-02** — receive D-09's four newly-surfaced advisories.
- **Phase 15's PIPE** — receives D-15's unexecuted Docker/Kubernetes jobs and D-11's clean-machine
  timing.

</code_context>

<specifics>
## Specific Ideas

**Facts verified live during this discussion at HEAD `68ba809`. Treat them as established, not as
hypotheses to re-check:**

1. **Version state.** Branch `release/v0.7.0`; root `Cargo.toml` and **all eleven** member
   manifests at `0.6.0`; tags present: `v0.1.0-rc.1, v0.3.0-rc.1, v0.4.0, v0.4.1, v0.4.2, v0.4.3,
   v0.5.0, v0.5.1` — **`v0.6.0` was never tagged.** `CHANGELOG.md`'s `## [0.6.0]` heading is the
   only version heading in the file **without a date**.
2. **Edition state.** Root + nine crates at `2024`; `crates/paladin-ports` and
   `crates/paladin-notifications` at `2021`. Toolchain pinned `1.97.1` — **Rust 2024 has been
   stable since 1.85**, so `CONCERNS.md`'s "this edition does not exist" claim is wrong (D-05).
3. **`cargo audit` passes.** 0 vulnerabilities, `13 allowed warnings`, DB loaded with 1186
   advisories from `github.com/RustSec/advisory-db`.
4. **`cargo deny check` passes** — `advisories ok, bans ok, licenses ok, sources ok` — with exactly
   one complaint: `warning[advisory-not-detected]` at `deny.toml:136`, `RUSTSEC-2025-0121` (gcc)
   matches nothing.
5. **The duplicate CI audit job does not block SC5.** Verified by running
   `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` directly: it exits `0`,
   because `--ignore` **augments** `.cargo/audit.toml` rather than replacing it. SUPPLY-01's
   deletion remains correct and remains Phase 12's.
6. **Four advisories are surfacing that neither config lists**: `RUSTSEC-2021-0145` (atty, unsound),
   `RUSTSEC-2026-0221` (event-listener, unsound), `RUSTSEC-2026-0205` (scc, unsound), and
   `spin 0.9.8` **yanked**. None fails a gate today.
7. **`examples/` holds 47 `.rs` files, not 22.** Four are declared `[[example]]` targets; no crate
   under `crates/` ships examples.
8. **`ci.yml` fires on nothing that a release-branch push would trigger** — the `push:` trigger is
   commented out at line 9; only `pull_request` to `main`/`develop` and `workflow_dispatch` remain.
9. **CI has no examples job, no Kubernetes smoke job, and no Docker size/time budget assertion.**
10. **`cargo fmt --check --all` is clean.**
11. **Local tooling:** `cargo-audit` ✓, `cargo-deny` ✓, `gh` ✓ — **`docker` ✗, `kind` ✗,
    `kubectl` ✗, `cargo-llvm-cov` ✗.** crates.io HTTP 403; github.com reachable.
12. **`reqwest` ships twice in `Cargo.lock`** — `0.12.28` and `0.13.4` — as `CONCERNS.md` records.

**The phase's honesty risk is specific and nameable.** Four of the five success criteria can be
*fully* discharged in this environment. The fifth — SC5's Docker and Kubernetes gates — cannot be
executed here at all. The failure mode is writing a plausible CI job and reporting the gate as met.
**Configuration authored is not a gate proven**, and D-15 exists to keep those two claims apart. The
same discipline Phase 3 applied to unmeasurable coverage applies here.

**Expect the version decision to be the one a human wants to see.** Everything else in this phase is
a measurement or a two-line manifest edit. D-01 picks `0.7.0` on defensible evidence, but it is the
decision with the largest and least reversible downstream consequence, and it is being made in a
phase that runs before the two requirements the corpus nominally assigned it to.

</specifics>

<deferred>
## Deferred Ideas

- **Deleting the duplicate `Security Audit` job at `ci.yml:389-406`** (18 lines). Owner:
  **SUPPLY-01, Phase 12**. Measured non-blocking for SC5 here (D-08.2); deleting it makes a
  Milestone 10 acceptance criterion true, which is Phase 12's payoff to claim.
- **Owner and expiry fields for the fifteen advisory suppressions**, and the disposition of the
  **2026-09-30** RustSec risk acceptance. Owners: **SEC-01 (Phase 9)** for the set and the expiry;
  **SUPPLY-02 (Phase 12)** for the schema and the three unratified 2026 ignores. Phase 4 adds
  migration/review notes only (D-09).
- **The four newly-surfaced advisories** — `RUSTSEC-2021-0145`, `RUSTSEC-2026-0221`,
  `RUSTSEC-2026-0205`, and yanked `spin 0.9.8`. Recorded as a dated finding here; disposition is
  **SEC-01 / SUPPLY-02**.
- **The `api-surface` CI job, red since commit `928c6d5`.** Owner: **DEBT-01, Phase 8**. It is not
  one of SC5's named gates.
- **Re-enabling `paladin-ports` doctests** (`doctest = false`, with a "re-enable in Task 7.0"
  comment that RECON-08 proved refers to a task that never existed). Owner: **DEBT-03, Phase 8**;
  the governing `cargo doc` bar is **HARD-07, Phase 10**.
- **The clean-machine QUICKSTART timing** — unreachable here (warm registry, crates.io 403, no
  Docker). Phase 4 measures under stated conditions (D-11.2); the true cold-start figure needs a
  runner with network. No owner assigned; nominate one when the row is written.
- **First execution of the Docker multi-arch and Kubernetes smoke jobs** (D-15). Authored and
  statically validated here; needs a Docker-capable runner. Owner: **Phase 15 / PIPE**.
- **The `cli-tests`, `bench-check` and `coverage` CI jobs, `.codecov.yml`, the Makefile coverage
  targets, and the eight deprecated GitHub Actions.** Owner: **PIPE-01 … PIPE-04, Phase 15**.
  Explicitly untouched by D-14, which adds only the three jobs SC5 itself names.
- **Wiring ADR-0006's 84% coverage floor into CI.** Owner: **PIPE-02, Phase 15**. No coverage job
  exists in `ci.yml` today, and SC5 does not name one.
- **The licence three-way** (`Cargo.toml` says MIT; a signed 2026-05-28 decision checklist says
  `MIT OR Apache-2.0`). Owner: **SEC-02, Phase 9**. `deny.toml`'s allow-list already follows the
  checklist; **do not infer an answer while editing `deny.toml` for D-08.**
- **The dual `reqwest` 0.12/0.13 exposure.** Documented in `CONCERNS.md:293-303`, treated as a
  warning by deliberate `deny.toml` policy. Not an advisory; a discretion item at most (see
  Claude's Discretion).
- **Pushing `v0.7.0` and publishing ten crates to crates.io.** A human action behind an explicit
  gate (D-03), not a deferred phase item.

</deferred>

---

*Phase: 4-release-coherence*
*Context gathered: 2026-08-02*
