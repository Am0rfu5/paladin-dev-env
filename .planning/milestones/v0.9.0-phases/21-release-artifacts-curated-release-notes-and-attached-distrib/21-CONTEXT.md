# Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables - Context

**Gathered:** 2026-08-31
**Status:** Ready for planning

<domain>
## Phase Boundary

A published release says what changed in the words this project already wrote, and hands a
consumer something they can actually run and verify. Concretely: the release body for `vX.Y.Z`
is the `## [X.Y.Z]` section of the curated root `CHANGELOG.md` (a missing section fails the
run — no silent `git log` fallback); every advertised binary is actually built under the
features its target requires, and a leg that produces no executable fails; the body references
only artifacts the run produced (no `docker pull …:latest` that never worked); the container
image is bound to the release by immutable `sha256:` digest; checksums are verifiable in one
command with instructions in the release; the archived `create-release@v1`/
`upload-release-asset@v1` era's `upload_url` plumbing is removed along with the dead Windows
strip guard; and the whole path is proven end-to-end on a throwaway tag or recorded as
unverified.

**In scope:** `create-release`'s body construction (ARTIFACT-01, -03); `build-binaries`'s
feature-correct builds, existence assertions, archive contents (ARTIFACT-02); digest binding
and the image-size check's disposition (ARTIFACT-04); checksum/SBOM presentation and the
signing/provenance decision (ARTIFACT-05); `upload-release-asset@v1` replacement, `upload_url`
removal, dead-branch cleanup, and the throwaway-tag rehearsal (ARTIFACT-06); the release-docs
updates these imply.

**Out of scope:** `publish-crates`, the consistency gate, and the recovery runbook (Phase 20,
shipped — do not regress its D-03 re-run safety); any `ci.yml`/`release.yml` trigger change
(gated by `scripts/check-workflow-triggers.sh`); the WR-05 asymmetry (artifact jobs not gated
on `test` — documented choice, not this phase's to revisit); new build targets (Windows or
otherwise); the real stable catch-up release (operator act).

Requirements: `ARTIFACT-01` … `ARTIFACT-06`.

**Ground truth (verified 2026-08-31 on `chore/20-close`, post-Phase-20):** Phase 20 already
replaced `create-release@v1` with `scripts/create-or-reuse-release.sh` (`gh` CLI,
create-or-reuse by tag, body passed via `--body-file`) — but the body file is still generated
from `git log --pretty=format:"- %s"` and still hardcodes the `:latest` pull instruction and a
static platform list. `build-binaries` still runs bare `cargo build` (no `paladin` produced —
`required-features = ["cli"]`, `cli` not in defaults), still carries the
`if: matrix.os != 'windows-latest'` strip guard, and still uploads via
`actions/upload-release-asset@v1` consuming `needs.create-release.outputs.upload_url`; `sbom`
does the same. `Verify image size` still ends green on `::warning::`. The root `CHANGELOG.md`
has dated sections through `## [0.8.1-rc.4]` (note: rc.4's section is heading-only/empty);
`scripts/finalize-crate-changelogs.sh` (Phase 20 D-09) stamps sections mechanically, and
`scripts/check-release-consistency.sh` Clause 2 already enforces section existence — but only
on the `publish-crates` path, not on `create-release` (WR-05), so ARTIFACT-01's
missing-section failure must live in the extraction step itself.

</domain>

<decisions>
## Implementation Decisions

*All decisions below were auto-selected under `--auto` (recommended option taken on each).
Each carries its reasoning so a human can overturn any single one without re-running the
discussion.*

### Release body composition and assembly order (ARTIFACT-01, ARTIFACT-03)

- **D-01:** The release notes for `vX.Y.Z` are **extracted from the root `CHANGELOG.md`
  `## [X.Y.Z]` section** by a repo script (logic-in-scripts, the Phase 20
  `create-or-reuse-release.sh` / `check-release-consistency.sh` pattern; whether it extends
  `create-or-reuse-release.sh` or is a new `extract-changelog-section.sh` invoked before it is
  planner discretion — either way it is unit-testable locally like
  `tests/scripts/publish-crates_test.sh`). Extraction is exact: begins after the
  `## [X.Y.Z]` heading, stops at the next `## [` heading. **A tag whose version has no
  matching section fails `create-release`** — no `git log` fallback in any form. The
  git-log changelog generation step is deleted, not bypassed.
- **D-02:** An **empty section (heading only) is allowed** — presence is the contract,
  content is the author's. Rationale: `finalize-crate-changelogs.sh` legitimately produces
  heading-only sections for quiet rc's (rc.4 is one today), and the rehearsal tag will be
  another; failing on emptiness would block exactly the throwaway tags this phase must test
  on. The body then carries the heading and whatever it holds.
- **D-03:** **The ten per-crate changelogs do not contribute to the release body** (decision
  recorded either way per criterion 1: root section only). They ship inside the published
  crates and are enforced in lockstep by the Phase 20 gate; the body may link to them in the
  repo at the tag, but inlining eleven changelog fragments makes the notes worse, not better.
- **D-04:** **Assembly order inverts the current "advertise first, build later" shape.**
  `create-release` creates (or reuses) the release with the curated changelog section as its
  body — nothing else. A new terminal job (e.g. `finalize-release-body`,
  `needs: [create-release, build-docker, build-binaries, sbom]`) then appends the artifact
  sections from **real job outputs**: the image digest and the tag list actually pushed, the
  asset names actually uploaded plus verification instructions, and the SBOM's stated scope —
  via `gh release edit`/`gh api` with the body passed as a file (CR-01 discipline). A leg that
  failed or was skipped gets its section omitted or stated as absent — never advertised. The
  finalize job recomputes the body idempotently from current outputs so both GitHub re-run
  shapes stay safe (Phase 20 D-03 must not regress). — **Reversibility:** costly — the job
  graph, outputs plumbing, and body-assembly script all encode this order; reverting to
  static-template composition touches every artifact job.

### Binary set and feature matrix (ARTIFACT-02)

- **D-05:** **All three declared binaries ship**: `paladin` + `paladin-cli` (need `cli`) and
  `paladin-server` (needs `web-server`). One build per target with an **explicit
  `--features cli,web-server`** (defaults remain on, so the compiled provider set stays the
  ADR-0046 shape: openai + anthropic + deepseek). The aarch64 leg composes
  `vendored-openssl` on top, as today. Rationale: the workspace declares three binaries and
  the release currently attaches none; shipping the full declared set under one explicit
  feature line is the simplest honest inventory. **Researcher must verify** `cli,web-server`
  (clap/dialoguer + axum) cross-compiles cleanly under `cross` for
  `aarch64-unknown-linux-gnu`; if a target provably cannot build a binary, that binary is
  dropped from that target's manifest explicitly (assert list per target) and recorded — not
  silently skipped.
- **D-06:** **The archive step asserts every expected executable exists before creating the
  tarball** — a missing binary fails the leg loudly (the anti-Phase-12 posture: no leg may
  produce nothing and end green). The tarball contains all three binaries; the existing
  `paladin-<os>-<arch>.tar.gz` asset naming is kept (consumers and checksums key off it).
  `strip` runs on each binary that shipped.

### Asset upload mechanism and dead plumbing (ARTIFACT-06)

- **D-07:** `actions/upload-release-asset@v1` (archived 2021) is replaced with **`gh release
  upload --clobber`** in `build-binaries` and `sbom` — same posture as Phase 20's D-01: no
  new marketplace action, `gh` is already authenticated via `GITHUB_TOKEN`, and `--clobber`
  makes re-runs idempotent (a re-run re-uploads the same asset name instead of failing on a
  duplicate). The **`upload_url` output and all its plumbing are deleted** from
  `create-release`, `build-binaries`, and `sbom` — this is the deliberate removal Phase 20's
  D-02 preserved the contract for. The `version` output stays (consumed by `sbom` and
  `publish-crates`).
- **D-08:** The `if: matrix.os != 'windows-latest'` guard on the strip step is **removed** —
  the matrix has no Windows leg and never has; the condition implies a target the release
  does not ship.

### Docker digest binding, `latest`, and the size check (ARTIFACT-03, ARTIFACT-04)

- **D-09:** `build-docker` captures the **`sha256:` digest** from `docker/build-push-action`'s
  `digest` output and the actually-pushed tag list from `metadata-action`, and exposes both as
  job outputs. The finalize job (D-04) writes into the body a pinnable
  `docker pull ghcr.io/<image>@sha256:…` line plus the real tag list. The **`:latest` pull
  instruction is deleted** — this workflow has never pushed `latest` on a tag
  (`enable={{is_default_branch}}` is false on `refs/tags/*`) and the tagging config itself is
  not changed; the body stops advertising what the run does not produce. The platform list in
  the body states what the build actually pushed.
- **D-10:** The image-size check takes ARTIFACT-04's **advisory branch**: the measured size
  in MB is stated in the release body (via the finalize job) as advisory against the 500 MB
  target, replacing the `::warning::`-then-green shape with honest consumer-visible
  reporting. Not converted to a hard failure now: the current image size is unmeasured on a
  real release, and a new red gate on an unvalidated threshold could block the first release
  through this pipeline. The rehearsal (D-14) records the measured size; promoting the
  threshold to a hard failure afterwards is a recorded deferred idea, not a silent drop.

### Checksums, SBOM labeling, and the signing decision (ARTIFACT-05)

- **D-11:** A single aggregated **`SHA256SUMS`** file covering every uploaded binary archive
  is generated and attached alongside the existing per-asset `.sha256` files (kept for
  continuity), and the release body carries the **one-command verification**
  (`sha256sum -c SHA256SUMS` after download, with the macOS `shasum -a 256 -c` variant
  noted). Generation must happen where all archives are visible (the finalize job or a small
  aggregation step downloading released assets — mechanism at planner discretion; the
  binding constraint is the sums file covers what was actually uploaded).
- **D-12:** The body identifies the attached CycloneDX SBOM as covering the **root
  `paladin-ai` package**, not the whole workspace — `cargo cyclonedx --all` writes one
  document per crate and the job attaches `paladin-ai.cdx.json` alone; an unqualified "SBOM"
  overstates scope.
- **D-13:** **Signing and build provenance are deferred, with reasoning recorded** (criterion
  5 explicitly permits deferral if examined): adopting cosign or GitHub artifact attestations
  would add new action surface and key/identity management in the same phase that is removing
  archived actions, and no consumer requirement demands it yet. GitHub's native
  `actions/attest-build-provenance` is named as the natural candidate when it is taken up.
  The decision and reasoning land in `docs/src/appendix/release-automation.md`, not only in
  planning files — a consumer asking "are these signed?" reads the docs, not `.planning/`.

### End-to-end rehearsal (ARTIFACT-06, criterion 7)

- **D-14:** The path is proven on a **real throwaway tag on the current prerelease line**
  (next free rc — exact string at executor discretion; `finalize-crate-changelogs.sh` +
  `make release` produce its changelog sections so the Phase 20 gate passes). Evidence: the
  assets download and `sha256sum -c` verifies; the image pulls by the digest the release
  names; the body matches the root `CHANGELOG.md` section for that version; the binaries in
  the tarball execute (`--version` or equivalent) on at least the native runner. Recorded in
  **`21-ARTIFACT-EVIDENCE.md`** in the phase directory, following the
  `19-PUBLISH-EVIDENCE.md` / `20-RECOVERY-EVIDENCE.md` shape (run URLs, dated, measured).
  **If the rehearsal is not run, the artifact path is recorded as unverified** — the Phase
  18/19/20 honesty rule; re-reading the workflow is explicitly not evidence (ARTIFACT-02's
  defect survived every prior reading). — **Reversibility:** one-way — the rehearsal tag
  triggers the full workflow including `publish-crates`, permanently occupying that rc
  version on crates.io for all eleven crates (established acceptable by Phase 19 D-04 /
  Phase 20 D-14: prereleases never win default resolution).
- **D-15:** `docs/src/appendix/release-automation.md` is updated to describe the new body
  source (curated changelog section), the artifact inventory (three binaries, features,
  targets), the verification instructions, and the D-13 signing decision;
  `docs/src/appendix/release-checklist.md` picks up any operator-visible change. The
  trigger-policy table in `docs/src/contributing/branching-model.md` is untouched (no
  trigger changes anywhere in this phase).

### Claude's Discretion

- Whether changelog extraction extends `create-or-reuse-release.sh` or is its own script;
  exact script/job names.
- The finalize job's exact mechanism (`gh release edit` vs `gh api` PATCH) and body-section
  layout, provided failed legs are never advertised and re-runs are idempotent.
- How `SHA256SUMS` aggregation sees all archives (job outputs vs downloading released
  assets).
- The rehearsal rc version string and the order of rehearsal vs. any needed fix-up commits.
- Whether the body links to per-crate changelogs (D-03 requires only that they not be
  inlined).

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements and phase definition
- `.planning/ROADMAP.md` §Phase 21 — goal, seven success criteria, Phase 20 coordination
  notes
- `.planning/REQUIREMENTS.md` §Release artifacts — ARTIFACT-01 … ARTIFACT-06 binding text

### The workflow and scripts being changed
- `.github/workflows/release.yml` — `create-release` (git-log body generation, the step to
  delete; `upload_url`/`version` outputs), `build-binaries` (bare `cargo build`, strip
  guard, `upload-release-asset@v1` ×2), `build-docker` (metadata-action tags, build-push
  digest, `Verify image size`), `sbom` (`paladin-ai.cdx.json`, `upload-release-asset@v1`),
  WR-05 comments, `needs` graph
- `scripts/create-or-reuse-release.sh` — Phase 20's create-or-reuse logic; D-01's extraction
  feeds its `--body-file`
- `scripts/check-release-consistency.sh` — Clause 2 changelog-section enforcement (the
  publish-path guarantee D-01's create-release-path check complements, not duplicates)
- `scripts/finalize-crate-changelogs.sh` — what makes a rehearsal tag's changelog sections
  exist mechanically (D-02's empty-section rationale)
- `tests/scripts/` — the unit-test pattern new/changed scripts follow
  (`publish-crates_test.sh` as reference)

### Build/feature ground truth
- `Cargo.toml` — `[[bin]]` ×3 with `required-features` (`:266-279`), `[features]`:
  `default` (no `cli`), `cli` (`:326`), `web-server` (`:318`), `vendored-openssl` (`:333`),
  ADR-0046 comment on the default provider set
- `rust-toolchain.toml` — pinned channel `build-binaries` reads (do not add a second source
  of truth)

### Phase 19/20 outputs this phase consumes (do not re-derive, do not regress)
- `.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-CONTEXT.md`
  — D-01 (no new marketplace actions; `gh` CLI posture), D-02 (`upload_url` contract kept
  alive *for this phase to remove*), D-03 (every job safe to run twice — binding on all new
  jobs here)
- `.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-RECOVERY-EVIDENCE.md`
  — rehearsal evidence shape; rc.3/rc.4 history the rehearsal rc numbering continues from
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md`
  — crates.io User-Agent convention, prerelease blast-radius reasoning (D-14's one-way
  rating leans on it)

### Constraints on workflow changes
- `scripts/check-workflow-triggers.sh` + `docs/src/contributing/branching-model.md` — no
  trigger changes; any workflow edit must keep the trigger-policy table literally consistent
- `.github/workflows/ci.yml` — untouched by this phase; referenced only for conventions

### Documentation to update
- `docs/src/appendix/release-automation.md` — body source, artifact inventory, verification
  instructions, D-13 signing decision (records the existing hand-authored-changelog choice
  this phase finally honors)
- `docs/src/appendix/release-checklist.md` — operator-visible changes
- `docs/src/appendix/release-recovery.md` — Phase 20's runbook; extend only if recovery
  semantics of the new jobs need a note (finalize job re-run behavior)
- `CHANGELOG.md` (root) — D-01's extraction source; sections through `0.8.1-rc.4` exist
  today (rc.4 heading-only — the D-02 case, live)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `scripts/create-or-reuse-release.sh` — already takes `--body-file`; the extraction script
  only has to produce a better file. Create-or-reuse semantics (200 reuse / 404 create /
  else fail) already handle re-runs.
- `gh` CLI authenticated on runners via `GITHUB_TOKEN` — carries release edit, asset upload
  (`gh release upload --clobber`), and API PATCH without new actions.
- `docker/build-push-action@v5` — already returns the image `digest` output; currently
  discarded. `docker/metadata-action@v5` `json` output already used by the size check for
  exact tag reconstruction.
- `scripts/finalize-crate-changelogs.sh` + `make release` — produce the changelog sections
  the rehearsal tag needs; no new tooling required to make a testable tag.
- `tests/scripts/` harness — Phase 20 unit-tested `publish-crates.sh` and
  `check-release-consistency.sh` there; extraction and body-assembly scripts follow suit.
- CR-01 pattern throughout `release.yml` — tainted values (tag input, versions) reach `run:`
  bodies via `env:` indirection, bodies via files; every new step must follow it.

### Established Patterns
- Logic in repo scripts, YAML as thin invocation (Phase 20 D-07 shape) — locally runnable
  and unit-testable.
- No new marketplace actions; pin any action ref used in a `contents: write` job (WR-02 —
  see the `dtolnay/rust-toolchain` SHA pin in `build-binaries`).
- Failure honesty: no job/leg may produce nothing and end green (Phase 12; Phase 18 D-06;
  Phase 19 D-09; Phase 20 D-12) — D-06's existence asserts and D-01's no-fallback are this
  posture.
- Evidence files: measured, dated, run-URL-sourced (`19-PUBLISH-EVIDENCE.md`,
  `20-RECOVERY-EVIDENCE.md`).
- WR-05 asymmetry is documented and deliberate — artifact jobs stay un-gated on `test`; new
  jobs added here must not silently change that posture.

### Integration Points
- `create-release` job — extraction script replaces the git-log step; `upload_url` output
  removed, `version` kept (consumed by `sbom` env and `publish-crates`).
- `build-binaries` / `sbom` — `gh release upload --clobber` replaces both
  `upload-release-asset@v1` uses; needs `contents: write` (already present).
- `build-docker` — gains digest/tags outputs; size check reshaped to report-into-body.
- New `finalize-release-body` job — terminal in the artifact `needs` graph; the only writer
  of artifact sections into the body.
- `publish-crates` — untouched; its `needs` on `create-release` and
  `check-release-consistency` are unaffected by body/asset changes.

### Live facts (verified 2026-08-31, branch `chore/20-close`)
- Three `[[bin]]` targets; `cli` and `web-server` both absent from `default` — a bare
  `cargo build --release` produces zero of the three release binaries (ARTIFACT-02's defect
  confirmed at source).
- `upload-release-asset@v1` appears 3× (`build-binaries` ×2, `sbom` ×1); `upload_url`
  plumbing spans `create-release` outputs + both consumers.
- Root `CHANGELOG.md`: `## [Unreleased]`, then dated sections `0.8.1-rc.4` (empty),
  `0.8.1-rc.3`, `0.8.1-rc.2`, … — extraction test corpus exists in-tree.

</code_context>

<specifics>
## Specific Ideas

- The missing-section failure message should name the fix: "no `## [X.Y.Z]` section in
  CHANGELOG.md — run `make release VERSION=X.Y.Z` (finalizes changelogs) before tagging" —
  an operator mid-release reads the job log, not the appendix.
- Body layout: curated notes first (they are the release), artifact/verification sections
  appended below a separator — the notes must not be buried under generated boilerplate.
- The digest line in the body should be copy-paste runnable
  (`docker pull ghcr.io/<lowercased-image>@sha256:…`) — reuse metadata-action's lowercasing
  rather than reconstructing the reference by hand (the size-check comment already warns
  about this).

</specifics>

<deferred>
## Deferred Ideas

- **Hard-fail image-size threshold** — D-10 keeps it advisory this phase; after the
  rehearsal records a measured size, promoting 500 MB (or a corrected figure) to a red gate
  is a candidate follow-up.
- **Artifact signing / build provenance** (cosign or `actions/attest-build-provenance`) —
  examined and deferred with reasoning per D-13; revisit when a consumer or registry
  requirement demands it.
- **Windows (or additional) build targets** — the matrix ships 4 Unix legs; adding targets
  is new capability, its own phase.
- **Per-crate changelog content in release notes** — D-03 records root-only; revisiting
  belongs to a future notes-format decision, not this phase.
- **The real stable catch-up release** — operator act once this phase's artifact path is
  proven (carried forward from Phase 20's deferred list).

### Reviewed Todos (not folded)
- **"Verify local make coverage reproduces CI's 82.39% figure"**
  (`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`) — matched at
  score 0.6 on keyword overlap. Not folded, deviating from the auto-mode ≥0.4 fold rule,
  because Phases 19 and 20 both reviewed this same match and recorded why it must not be
  folded: it concerns coverage-measurement reproduction (orthogonal to release artifacts),
  is explicitly owned by the repo maintainer, requires a human on a Docker-capable machine,
  and its own text forbids silent closure by a phase. That determination carries forward.

</deferred>

---

*Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib*
*Context gathered: 2026-08-31*
