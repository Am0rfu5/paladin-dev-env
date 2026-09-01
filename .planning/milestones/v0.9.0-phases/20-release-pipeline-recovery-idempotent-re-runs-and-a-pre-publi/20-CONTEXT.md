# Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate - Context

**Gathered:** 2026-08-28
**Status:** Ready for planning

<domain>
## Phase Boundary

A release that fails partway through becomes finishable by re-running the same tag, and no
crate is published until the tag, all eleven manifest versions, the root changelog, the ten
crate changelogs and the tagged commit's recorded CI conclusion agree. Concretely: replace
`actions/create-release@v1` so a same-tag re-run reaches the publish step; replace error-prose
grep and the fixed `sleep 20` with registry-state reads; put a verification gate in front of
the first `cargo publish`; make the publish job's outcome state per crate exactly one of
published-now / already-at-this-version / skipped / failed, with a no-crate-moved run failing;
and write the stuck-halfway runbook including a yank policy. The recovery path is exercised
against an induced partial failure on a throwaway prerelease, or the runbook is labelled
untested.

**In scope:** `create-release` job idempotency (PUBOPS-03); registry-state already-published
detection and index waiting (PUBOPS-03); the pre-publish consistency gate and the release
tooling changes needed to make it satisfiable (PUBOPS-01); CI-conclusion verification for the
tagged SHA (PUBOPS-02); per-crate outcome reporting and nothing-published honesty (PUBOPS-04);
the recovery runbook, yank policy and rehearsal (PUBOPS-05).

**Out of scope:** release body content, attached binaries, Docker digest binding, SBOM scope,
and removal of `upload_url` plumbing / `upload-release-asset@v1` (all Phase 21, `ARTIFACT-*`);
any change to `ci.yml` or `release.yml` triggers (constrained by
`scripts/check-workflow-triggers.sh` and the branching-model trigger-policy table); the stable
catch-up release itself (this phase builds the machinery; cutting 0.8.1+ for real is an
operator act after it exists).

Requirements: `PUBOPS-01`, `PUBOPS-02`, `PUBOPS-03`, `PUBOPS-04`, `PUBOPS-05`.

**Ground truth correcting the roadmap text (verified 2026-08-28 on this branch):** Phase 19
already rewrote `publish-crates` — OIDC via `rust-lang/crates-io-auth-action@v1`, `crates-io`
environment, eleven-crate `CRATES` array, `dry_run` mode from dispatch input, skip branch
deleted. The roadmap's references to `CARGO_REGISTRY_TOKEN` at `:385`/`:401` and a ten-crate
array are stale. Still present and owned by this phase: `create-release@v1`
(`release.yml` `create-release` job), the `grep -qiE "already (exists|uploaded)…"` tolerance
and `sleep 20` in `publish_one()`, the green-when-nothing-published loop, no pre-publish gate,
and zero occurrences of "yank" in docs/workflows/scripts. All eleven crates are live on
crates.io at `0.8.1-rc.2` (published via OIDC 2026-08-27, run 33089177606); manifests are at
`0.8.1-rc.2`; the ten crate changelogs all sit at `## [Unreleased]` with no versioned section.

</domain>

<decisions>
## Implementation Decisions

*All decisions below were auto-selected under `--auto` (recommended option taken on each).
Each carries its reasoning so a human can overturn any single one without re-running the
discussion.*

### Create-release idempotency (PUBOPS-03, criterion 1)

- **D-01:** `actions/create-release@v1` (archived since 2021) is **replaced with `gh` CLI /
  `gh api` create-or-reuse logic**: if a release for the tag exists, reuse it; if not, create
  it. No new third-party action is introduced — the repo's posture is to pin and minimize
  action surface, `gh` is authenticated by `GITHUB_TOKEN` on the runner, and Phase 21 will
  rewrite this job's body/assets anyway, so a marketplace action adopted now would be churned
  twice.
- **D-02:** The job's **outputs contract is preserved**: it continues to emit `upload_url` and
  `version` (fetched via `gh api repos/{owner}/{repo}/releases/tags/{tag}` on the reuse path),
  because `build-binaries` and `sbom` consume `needs.create-release.outputs.upload_url` via
  `actions/upload-release-asset@v1` today. Phase 21 owns removing that plumbing; Phase 20 must
  not break it. — **Reversibility:** reversible — Phase 21 deletes the contract deliberately.
- **D-03:** Every job on the tag→publish path must be safe to run twice on the same tag.
  Both re-run shapes are supported: "Re-run failed jobs" (primary recovery) and "Re-run all
  jobs" (must not fail on already-done work — create-release reuses, publish loop skips
  published crates, gate re-verifies).

### Already-published detection and index wait (PUBOPS-03, criterion 2)

- **D-04:** "Already published" is determined **from registry state, never from matched error
  prose**: the crates.io API (or sparse index) is queried for `crate@version` *before*
  attempting `cargo publish`, and a crate already at the tagged version is skipped and
  recorded as `already-at-this-version`. The `grep -qiE` tolerance is deleted. All crates.io
  API calls send a `User-Agent` header (crates.io answers 403 without one — ADR-0026 /
  19-PUBLISH-EVIDENCE convention).
- **D-05:** The fixed `sleep 20` between crates is replaced by **polling the registry until
  the just-published version is visible, with a bounded timeout** — a check, not a guess.
- **D-06:** **Researcher must evaluate `cargo publish --workspace` as the carrier** for D-04/
  D-05: `ci.yml`'s `publish-dry-run` already uses it, and native workspace publishing does its
  own ordering and index waiting. Adoption is conditional on verifying (a) the pinned
  toolchain supports it for real publishes, (b) it tolerates a partially-published workspace
  (the half-published recovery case — some crates at the version, some not), and (c) per-crate
  outcomes (PUBOPS-04) remain derivable, e.g. from registry state before/after. If any of the
  three fails, keep the explicit per-crate loop with API pre-checks. Either way the detection
  principle in D-04 is binding.

### Pre-publish gate (PUBOPS-01)

- **D-07:** The gate is a **repo script** (e.g. `scripts/check-release-consistency.sh` —
  exact name at planner discretion, following the Phase 15.1 `check-workflow-triggers.sh`
  pattern) invoked by a **new job in `release.yml` that `publish-crates` `needs`**, and
  runnable locally (make target). It runs before the first `cargo publish`; whether
  `create-release` also needs it is planner discretion.
- **D-08:** The gate checks that ALL of the following agree, and **reports every mismatch
  found, not the first**: the tag version; the eleven publishable manifests' `version` fields
  (read from `cargo metadata`, not a hardcoded list — the crate set is Phase 19's D-01/D-02
  enumeration, consumed not re-derived); a `## [X.Y.Z]` section in the root `CHANGELOG.md`;
  and a `## [X.Y.Z]` section in each of the ten crate changelogs. It also performs the
  PUBOPS-02 CI-conclusion check (D-10) or depends on the job that does.
- **D-09:** **Prerelease tags get the same gate as stable tags — no exempted path.** An
  exempted path is an untested path, and the rehearsal (D-14) runs on a prerelease, so the
  strict rule is what gets exercised. To make this satisfiable without an eleven-file manual
  chore, the release tooling (`make release` / cargo-release configuration) is **extended in
  this phase to finalize the ten crate changelogs mechanically** alongside the root one.
  Researcher verifies cargo-release's per-crate changelog/replacement support; fallback is a
  small finalize script driven by the same version string. — **Reversibility:** reversible —
  gate policy and tooling are script/config edits.

### CI-conclusion verification (PUBOPS-02)

- **D-10:** Of the two permitted resolutions, this phase takes **"resolve the recorded CI
  conclusion for the tagged SHA"**: query the GitHub API for `ci.yml`'s workflow run(s) on the
  exact tagged commit SHA and require a recorded successful conclusion; refuse to publish
  without one. The alternative — running the equivalent eighteen jobs inside `release.yml` —
  duplicates CI and drifts. The merge commit a release tag points at always has a `ci.yml` run
  because `ci.yml` triggers on push to every branch including `main`.
- **D-11:** **`ci.yml` triggers are not touched** (no tag trigger added): the eighteen-job
  suite on tag pushes is redundant with the recorded main-branch run, and any trigger change
  is gated by `scripts/check-workflow-triggers.sh` and must stay literally consistent with
  `docs/src/contributing/branching-model.md`'s trigger-policy table. The failure message when
  no successful run exists tells the operator exactly what to do (re-run CI on `main` at that
  SHA, or fix and re-tag).

### Per-crate outcome reporting (PUBOPS-04)

- **D-12:** The publish job emits a **per-crate outcome table** (written to
  `$GITHUB_STEP_SUMMARY` and echoed in the log): each of the eleven crates gets exactly one of
  `published-now` / `already-at-this-version` / `skipped` / `failed`. A run in which **no**
  crate reached `published-now` **fails** with a distinct message ("all N crates already at
  X.Y.Z — this tag appears fully published; if this was a recovery re-run, nothing was left to
  recover"), so a fully-complete re-run is an honest, diagnosable red rather than a fake green
  — the Phase 12 / 18 D-06 / 19 D-09 honesty posture applied to this job. This is distinct
  from PUB-05 (missing-credential skip), which Phase 19 already deleted.

### Runbook and yank policy (PUBOPS-05)

- **D-13:** The runbook is a **new `docs/src/appendix/release-recovery.md`**, beside
  `release-automation.md` and `release-checklist.md`, cross-linked from both. It answers
  concretely: how to establish which crates reached crates.io (API queries with the User-Agent
  convention, per-crate at the tag version); the default recovery is **complete forward by
  re-running the same tag's workflow run** (D-03); a published version is never deleted or
  re-uploaded, so a bad publish is corrected by a new patch version plus `cargo yank`, never a
  retry of the same version; **who may yank: the crate-owner account (the repo owner —
  crates.io ownership, not CI, holds that power)**; and every yank is recorded in a "Yank
  register" table in the runbook (version, crates, reason, owner, date) — the Phase 9/12
  owner+date convention, kept out of `SECURITY-EXCEPTIONS.md` for the same reason as Phase
  19's D-12 (that file's contract is mechanically checked for advisory suppressions).

### Recovery rehearsal (PUBOPS-05 / criterion 7)

- **D-14:** The rehearsal induces a **real partial failure on a throwaway prerelease
  version** (the next rc on the current line): let the publish loop land some crates, stop it
  mid-loop (cancellation or an injected failure), verify the half-published state via the
  registry, then perform the documented recovery — **re-running the same tag-push workflow
  run** — and verify it completes the set. `cargo publish --dry-run` is explicitly not
  acceptable evidence. Evidence (run URLs, per-crate registry states before/after) is recorded
  in a phase evidence file in the `19-PUBLISH-EVIDENCE.md` style. If the rehearsal is not run,
  the runbook is labelled **untested** — the Phase 18/19 honesty rule.
  — **Reversibility:** one-way — the rehearsal permanently occupies prerelease version
  number(s) on crates.io for all crates it publishes; a published version can never be deleted
  (yank hides, it does not remove). Prerelease versions never win default dependency
  resolution, so blast radius is minimal (established by Phase 19 D-04).
- **D-15:** Recovery is designed around **re-running the existing tag-push run**, not
  `workflow_dispatch`: Phase 19's assumption A1 (`workflow_dispatch` eligibility under
  crates.io Trusted Publishing) is untested, a re-run keeps the `v*.*.*` ref that the
  `crates-io` environment's deployment policy and the OIDC subject claim both require, and
  "Re-run failed jobs" naturally skips completed jobs. If the rehearsal happens to prove
  dispatch works too, record it; do not depend on it.

### Claude's Discretion

- Exact gate script name, make target name, and the new gate job's name/position in the
  `needs` graph (binding constraint: before the first real `cargo publish`).
- Whether `cargo publish --workspace` or the explicit loop carries D-04/D-05 (per D-06, after
  researcher verification).
- The precise induced-failure mechanism in the rehearsal (cancellation vs injected fault),
  and the rc version string used.
- Whether the CI-conclusion check lives inside the gate script or as its own job/step.
- Runbook prose structure, provided the D-13 content list is fully answered.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements and phase definition
- `.planning/ROADMAP.md` §Phase 20 — goal, seven success criteria, dependency notes (note the
  stale pre-Phase-19 line references corrected in `<domain>` above)
- `.planning/REQUIREMENTS.md` §Publish operations — PUBOPS-01 through PUBOPS-05 binding text

### The workflow being changed
- `.github/workflows/release.yml` — `create-release` job (`create-release@v1`, changelog-from-
  git-log, `upload_url`/`version` outputs), `publish-crates` job (OIDC auth, eleven-crate
  `CRATES` array, `publish_one()` with grep tolerance and `sleep 20`), `verify-tag-source`,
  WR-05 gating comments, `needs` graph
- `.github/workflows/ci.yml` — triggers (`push: branches: ['**']` — why tags run nothing);
  `publish-dry-run` job (`cargo publish --workspace --dry-run`, lines ~1253-1276) — the
  workspace-publish precedent D-06 evaluates

### Phase 19 outputs this phase consumes (do not re-derive)
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md`
  — eleven-crate set and name→directory map, dependency-order constraints (herald after ports,
  before paladin-ai), OIDC proof at `0.8.1-rc.2`, token revocation record, **untested
  assumption A1 (`workflow_dispatch` under Trusted Publishing)**, crates.io User-Agent
  requirement
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-CONTEXT.md`
  — D-01..D-13 (crate set, environment shape, honesty posture, doc placement conventions)

### Constraints on workflow changes
- `scripts/check-workflow-triggers.sh` — gates any trigger change; `release.yml` is a
  documented exception; D-11 plans no trigger edits
- `docs/src/contributing/branching-model.md` — trigger-policy table that must stay literally
  consistent with the YAML

### Release tooling the gate must stay consistent with
- `Makefile` — `release` target (cargo-release lockstep bump + root changelog finalize;
  extended by D-09 to finalize crate changelogs), `release-check`, `publish-dry-run` targets
- `CHANGELOG.md` (root) and `crates/*/CHANGELOG.md` (ten files, all at `## [Unreleased]`) —
  the gate's inputs
- `Cargo.toml` + `crates/*/Cargo.toml` — eleven literal `version` fields (no
  `version.workspace = true`); `cargo metadata` is the enumeration source

### Documentation to create/update
- `docs/src/appendix/release-automation.md` — existing automation doc; cross-links the new
  runbook; trust table lives here (Phase 19 D-11)
- `docs/src/appendix/release-checklist.md` — updated to reference the gate and runbook
- `docs/src/appendix/release-recovery.md` — **new** (D-13): runbook + yank policy + yank
  register
- `SECURITY-EXCEPTIONS.md` — the owner+date convention the yank register follows (convention
  only; entries do NOT go in this file)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `publish_one()` loop structure and the eleven-entry `CRATES` array in `release.yml` — the
  dependency order (with the herald dev-dependency constraint documented inline) is correct
  and reusable; only detection/wait/reporting change.
- `ci.yml` `publish-dry-run` — working `cargo publish --workspace --dry-run` invocation; the
  candidate mechanism for D-06.
- `scripts/check-workflow-triggers.sh` — the shape for a repo-script-plus-CI-job check the
  gate follows (D-07).
- crates.io API call pattern with mandatory `User-Agent` header — worked examples throughout
  `19-PUBLISH-EVIDENCE.md`, including per-version `trustpub_data` provenance queries.
- `gh` CLI available in CI (`GITHUB_TOKEN`) and in this devcontainer (see memory: GH_TOKEN
  reaches non-interactive shells; verify write scope before relying on it locally).

### Established Patterns
- Per-job `permissions:` blocks; `crates-io` environment restricted to `v*.*.*` tags with the
  environment name in the OIDC subject claim (Phase 19 D-06..D-08) — recovery re-runs must
  stay on a tag ref.
- Failure honesty: no job reports success while measuring/doing nothing (Phase 12; Phase 18
  D-06; Phase 19 D-09) — D-12 applies it to the nothing-published case.
- Evidence files: measured, dated, sourced (`18-CODEQL-EVIDENCE.md`, `19-PUBLISH-EVIDENCE.md`)
  — D-14's rehearsal record follows this shape.
- Owner+date recording for security-relevant events (Phase 9/12) — the yank register.

### Integration Points
- `release.yml` `create-release` job — D-01/D-02 rewrite lands here; `build-binaries` and
  `sbom` consume its `upload_url` output (do not break; Phase 21 removes).
- `release.yml` `publish-crates` job — detection, wait, and outcome-reporting rewrite; gains a
  `needs` edge on the new gate job.
- GitHub API (via `gh`) — release lookup/create (D-01), CI-conclusion resolution (D-10).
- crates.io API — already-published pre-checks, index-visibility polling, recovery-state
  queries in the runbook.

### Live facts (verified 2026-08-28, this branch)
- Twelve manifests at `0.8.1-rc.2` (eleven publishable + `doc-examples`); root `CHANGELOG.md`
  has `## [0.8.1-rc.2]`; all ten crate changelogs have no versioned section — the gate as
  specified would (correctly) fail a tag pushed today, which is the D-09 tooling work's
  motivation.
- All eleven crates on crates.io at `0.8.1-rc.2` with OIDC `trustpub_data` provenance; old
  token revoked 2026-08-27.

</code_context>

<specifics>
## Specific Ideas

- The nothing-published failure message must be self-diagnosing: name the version, say the tag
  appears fully published, and point at the runbook — an operator mid-incident reads the job
  log, not the docs index.
- The gate lists ALL mismatches in one run (the roadmap is explicit: "names every mismatch it
  found rather than the first") — collect-then-report, no fail-fast per check.
- Rehearsal evidence file: `20-RECOVERY-EVIDENCE.md` (or similar) in the phase directory,
  `19-PUBLISH-EVIDENCE.md` as the reference shape — run URLs, per-crate registry state before
  the induced failure, after it, and after the recovery re-run.

</specifics>

<deferred>
## Deferred Ideas

- **Release body from curated `CHANGELOG.md` section, attached binaries/digest/SBOM fixes,
  `upload_url` plumbing removal, `upload-release-asset@v1` replacement** — Phase 21
  (`ARTIFACT-*`). Phase 20 keeps the outputs contract alive (D-02) precisely so Phase 21 can
  remove it deliberately.
- **The real stable catch-up release** (registry at prerelease rc's; last stable is 0.5.1) —
  an operator act once this phase's gate and recovery machinery exist; not a phase
  deliverable.
- **Environment required-reviewer gate on `crates-io`** — deliberately deferred by Phase 19
  D-08 "until Phase 20's runbook defines who approves releases"; the runbook (D-13) names who
  may yank, and whether that person also becomes a required reviewer is a follow-up settings
  decision, not taken silently here.
- **`workflow_dispatch` eligibility under Trusted Publishing (A1)** — remains untested unless
  the rehearsal incidentally proves it (D-15); a dedicated test is not scoped.

### Reviewed Todos (not folded)
- **"Verify local make coverage reproduces CI's 82.39% figure"**
  (`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`) — matched at
  score 0.6 on keyword overlap (verify/coverage/docs/github). Not folded, deviating from the
  auto-mode ≥0.4 fold rule, because Phase 19 already reviewed this same match and recorded
  why it must not be folded: it concerns coverage-measurement reproduction (orthogonal to
  publish operations), is explicitly owned by the repo maintainer, requires a human on a
  Docker-capable machine, and its own text forbids silent closure by a phase. That
  determination carries forward.

</deferred>

---

*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Context gathered: 2026-08-28*
