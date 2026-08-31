# Requirements: Paladin

**Milestone: v0.9.0 — Security Tooling** (started 2026-08-24)

*Fresh requirements file, opened at the v0.8.0 milestone close on 2026-08-24. Everything the
previous milestones defined — the 90 forward requirements across `RECON-*` … `PROV-*`, the 554
ingested `REQ-*` IDs, the five as-shipped ledgers, the 30 competing-variant groups and the full
traceability history — is archived and unchanged in
[`milestones/v0.8.0-REQUIREMENTS.md`](milestones/v0.8.0-REQUIREMENTS.md) and
[`milestones/v0.7.1-REQUIREMENTS.md`](milestones/v0.7.1-REQUIREMENTS.md). Consult those before
re-deriving anything; this file starts the new milestone rather than restating them.*

**`SAST-01` … `SAST-04` are carried forward, not re-minted.** They were written on 2026-08-24
(commit `48ac11a5`) alongside Phase 18 and appear in the v0.8.0 archive only because the archive is
a snapshot taken at close — they were never v0.8.0 work. Their text below is the text that shipped
with the roadmap entry, byte-identical. `.planning/ROADMAP.md`'s Phase 18 section references these
IDs directly, so they must resolve here.

---

## Requirements — Security Tooling (Phase 18)

*Added 2026-08-24. Forward work, not ingest-derived — opened by the v0.8.0 milestone audit, which
records the absence of first-party Rust static analysis as the milestone's one genuinely open item.
`SAST-*` is the nineteenth prefix; per Roadmap Extension Protocol item 3 no earlier prefix is
recycled. These IDs are minted at roadmap time rather than during execution, which is the lesson
from Phase 15.1 carrying `Requirements: TBD` into execution and settling it retroactively.*

### Rust static analysis (SAST)

- [x] **SAST-01**: A candidate Rust SAST is **measured against a deliberate-vulnerability probe on
      this tree before any adoption decision**, and the finding count is recorded either way.
      The probe is the one that disqualified Snyk, reused verbatim so the results are comparable: a
      Rust fixture carrying a hardcoded credential, command injection via `sh -c`, path traversal
      and SQL injection. Snyk Code returned **0 findings** on that fixture while the identical four
      in JavaScript returned 3 (HIGH/MEDIUM/LOW), which is what proved the scanner and credentials
      worked and the Rust analysis did not — see `.github/instructions/security.instructions.md`.
      **A zero-finding result satisfies this requirement**: it disqualifies the tool, and the
      verdict plus its evidence is the deliverable. What does not satisfy it is adopting a scanner
      without running the probe.
      Primary candidate: **CodeQL**, whose Rust support left public preview and reached general
      availability in October 2025, is supported in both default and advanced setup, and carries
      real Rust queries rather than file ingestion alone. Secondary: **Semgrep**, which is pattern
      matching rather than interprocedural taint analysis and is therefore evaluated as a
      complement, not as the primary control.

- [x] **SAST-02**: If a scanner qualifies under SAST-01, it **runs on every pull request and cannot
      be path-filtered into silence.**
      Its workflow triggers on `pull_request` with no path filter, plus `push` on `main` and a
      schedule. This is a hard constraint rather than a preference: `scripts/check-workflow-triggers.sh`
      Clause 2 (drift) exists because a required context living in a workflow whose trigger surface
      has silently narrowed — a filtered `pull_request` path, a dropped trigger type, a
      reintroduced branch filter — never reports on a PR touching no matching path, and that PR is
      then unmergeable forever with no failing check to point at. Cost note for planning: the
      repository is public, so GitHub code scanning
      and CodeQL carry no licence cost, and `github/codeql-action/upload-sarif@v3` is already wired
      into `ci.yml` for OSV results — code scanning is enabled today. Unlike Snyk, no token or
      vendor account is required.

- [x] **SAST-03**: The scanner **runs non-blocking first, and is promoted on measured behaviour.**
      A recorded observation window reports its false-positive rate and wall-clock cost against
      this tree's real size (385 `.rs` files, ~141,717 lines). Only then may it become a required
      check — and promotion updates all four places the required set is written down in a single
      change: the context is added to `.github/rulesets/protect-main-branch.json` (44 → 45), the
      live ruleset `20868126` is re-applied, `docs/src/appendix/branch-protection.md`'s context
      table is brought to match, and `scripts/check-workflow-triggers.sh` passes. Pinning an
      unmeasured scanner as a 45th required check is how a gate ends up permanently red or
      routinely bypassed — the defect class Phase 12 deleted when it removed the duplicate audit
      job.

- [x] **SAST-04**: `.github/instructions/security.instructions.md`'s **"Known gap: no Rust SAST"
      section is rewritten to match the measured outcome**, stating what the adopted tool does and
      does not cover and what the manual credential-handling review still owns.
      The section is narrowed or replaced by evidence, never deleted to imply coverage the probe
      did not establish. If SAST-01 disqualifies every candidate, this requirement is satisfied by
      updating the section to record which tools were measured, on what date, with what result —
      so the next person to ask does not repeat the evaluation blind.

---

## Requirements — Publishing Auth (Phase 19)

*Added 2026-08-25. Forward work, not ingest-derived. `PUB-*` is the **twentieth** prefix; per
Roadmap Extension Protocol item 3 no earlier prefix is recycled. `SUPPLY-*` is the near neighbour
and is deliberately **not** reused: it is spent on Milestone 9-12 and its requirements live in
[`milestones/v0.8.0-REQUIREMENTS.md`](milestones/v0.8.0-REQUIREMENTS.md). `SUPPLY-*` governed
supply-chain risk arriving through **dependencies**; `PUB-*` governs the one direction this project
**writes** to a public registry. As with `SAST-*`, these IDs are minted at roadmap time rather than
during execution.*

### Publish credential (PUB)

- [x] **PUB-01**: The **publishable crate set is enumerated from `Cargo.toml` and reconciled with
      the publish order in `release.yml` before any trust link is created.**
      crates.io Trusted Publishing is configured per crate, which promotes the crate list from an
      implementation detail to the security boundary — an unlisted crate is an unprotected crate.
      The enumeration must be made against the manifests, because the two sources already disagree:
      **eleven workspace crates are publishable and `release.yml`'s `CRATES` array lists ten.**
      `paladin-herald` is a workspace member (`members = [".", "crates/*"]`), sets no
      `publish = false`, and is a `version`+`path` dependency of the root `paladin-ai` crate, yet
      does not appear in the publish order — so a real `cargo publish -p paladin-ai` depends on a
      crate this workflow never publishes. Closing the gap and recording it as a deliberate
      exclusion are both acceptable outcomes; carrying it forward unexamined is not.

- [x] **PUB-02**: **Publishing authenticates through an OIDC exchange rather than a stored
      secret.** The `publish-crates` job obtains a short-lived crates.io token at run time via
      `rust-lang/crates-io-auth-action`, which `cargo publish` then consumes. `id-token: write` is
      granted **on that job alone**, never at workflow level: `release.yml` already declares
      `permissions:` per job (`publish-crates` currently holds `contents: read`), and that
      least-privilege shape is preserved. The job runs under a protected GitHub Environment — the
      pattern `docs.yml` already uses, which declares `id-token: write` and pins its deploy job to
      the `github-pages` environment, so the mechanism is established in this repository rather
      than introduced by this phase.

- [x] **PUB-03**: **The new path is proven to publish before the old credential is destroyed, and
      the proof is not a dry run.**
      `cargo publish --dry-run` requires no credential whatsoever, so a green dry run is evidence
      about packaging and nothing at all about the OIDC exchange. Acceptable evidence shows a token
      actually minted and accepted by crates.io — a real publish, a prerelease, or an equivalent
      recorded exchange. The ordering is itself the requirement: revoking first and testing second
      leaves the project unable to ship a release until the new path is debugged under pressure.

- [x] **PUB-04**: **The long-lived credential is revoked at crates.io and deleted from repository
      secrets, and both actions are recorded with a date and an actor.**
      A migration that leaves `CARGO_REGISTRY_TOKEN` in place has added a publish path rather than
      replaced one, which is a net increase in attack surface. **Revocation at crates.io is the
      load-bearing half** — deleting the repository secret alone leaves a live, publish-scoped
      token valid wherever else it was ever pasted. Recording follows the convention Phase 9/12
      established for advisory suppressions: an owner and a date, not an implication.

- [x] **PUB-05**: **No release silently skips publishing while reporting success, and the per-crate
      trust configuration is documented.**
      `release.yml`'s `dry_run=skip` branch currently emits `::warning::CARGO_REGISTRY_TOKEN is not
      set — skipping crates.io publish.` and lets the job finish green. Under Trusted Publishing
      there is no secret to be absent, so that branch is removed or rewritten to fail; a release
      that publishes nothing must not end green. Alongside it, the trust configuration is written
      down as a table — crate name, workflow filename, environment name, link date — because with
      eleven crates it is a configuration rather than a step, and because crate names diverge from
      directory names here (`crates/paladin-core` publishes as `paladin-ai-core`; the workspace root
      publishes as `paladin-ai`). Any crate that **cannot** be linked — one never yet published has
      no crates.io entry to configure trust against — is named explicitly with its interim auth
      path, never described as covered.

---

## Requirements — Publish Operations (Phase 20)

*Added 2026-08-25. Forward work, not ingest-derived. `PUBOPS-*` is the **twenty-first** prefix; per
Roadmap Extension Protocol item 3 no earlier prefix is recycled. `PUB-*` is the immediate neighbour
and is deliberately **not** extended: `PUB-*` scopes the publish **credential** (Phase 19), while
`PUBOPS-*` scopes the publish **operation** — what the pipeline proves before it publishes, and what
an operator does when it stops with some crates on crates.io and some not. As with `SAST-*` and
`PUB-*`, these IDs are minted at roadmap time rather than during execution.*

### Publish operations (PUBOPS)

- [x] **PUBOPS-01**: **No crate is published until the tag, every manifest version and every
      changelog agree, and the gate reports every mismatch rather than the first.**
      The workspace currently offers three independent sources of the release version and no check
      that they match. Each of the eleven manifests carries a literal `version = "0.8.0"` — none
      uses `version.workspace = true`, so a bump is eleven separate edits — the root `CHANGELOG.md`
      has a `## [0.8.0]` section, and **none of the ten per-crate changelogs has one**; all ten
      still sit at `## [Unreleased]`. A `v0.8.0` tag pushed today would publish ten crates whose own
      changelogs describe no such release, and the pipeline would not notice. The gate runs before
      the first `cargo publish`, not after.

- [x] **PUBOPS-02**: **The claim "the tagged commit passed CI" is verified against a recorded run,
      never inferred from the branch the commit sits on.**
      `ci.yml` triggers on `push: branches: [ '**' ]`, which does not match `refs/tags/*` — pushing
      a tag runs none of its eighteen jobs. `verify-tag-source` establishes that the commit is an
      ancestor of `main`, which is provenance, not verification. `release.yml`'s own `test` job runs
      `cargo test --workspace` and nothing else: no `fmt`, no `clippy`, no coverage floor, no
      `cargo-deny`, no OSV scan. Two resolutions are acceptable — resolve the CI conclusion for the
      tagged SHA and refuse to publish without a success, or run the equivalent checks inside the
      release workflow. Assuming is not one of them.

- [x] **PUBOPS-03**: **Re-running a release on the same tag is idempotent end-to-end, and
      already-published is determined from registry state rather than from matched error prose.**
      Idempotency must hold for every job between the tag and `cargo publish`, not only inside the
      publish loop. It does not today: `create-release` uses `actions/create-release@v1` — archived
      upstream since 2021 — which returns 422 `already_exists` for a tag that already has a GitHub
      release, and `publish-crates` declares `needs: [test, create-release]`, so on a retry the
      publish step is never reached and its tolerance is dead code. That tolerance is itself
      `grep -qiE "already (exists|uploaded)|is already uploaded|already published"` over the
      combined output of `cargo publish`: a wording change at crates.io turns a recoverable re-run
      into a hard failure. The same loop hand-rolls its index wait as a fixed `sleep 20` between
      crates — a guess rather than a check, and a contributor to the half-published state this
      requirement exists to recover from. `ci.yml`'s `publish-dry-run` job already invokes
      `cargo publish --workspace`, whose ordering and index-waiting semantics the release loop
      predates.

- [x] **PUBOPS-04**: **A release run that publishes nothing does not report success, and one that
      publishes only some crates records which.**
      With every crate already at the tagged version, the publish loop emits ten
      `::warning::<crate> version already published — continuing.` lines and the job ends green —
      externally indistinguishable from a release that worked. The job's outcome states, per crate,
      exactly one of: published now, already at this version, skipped, or failed; a run in which no
      crate reached *published now* is not a successful release. This is distinct from **PUB-05**,
      which governs the missing-credential `dry_run=skip` branch: this requirement governs the case
      where authentication succeeded and nothing was published.

- [x] **PUBOPS-05**: **The stuck-halfway case has an operator runbook that states a yank policy, and
      the recovery path is exercised rather than only described.**
      The word `yank` currently appears nowhere in `docs/src/`, `.github/workflows/` or `scripts/`.
      The runbook sits beside the existing `docs/src/appendix/release-automation.md` and
      `release-checklist.md`, and answers concretely: how to establish which crates reached
      crates.io and which did not; whether to complete forward or roll back; that a published
      version is never deleted and never re-uploaded, so a bad publish is corrected by a new patch
      version plus `cargo yank` rather than a retry of the same version; who may yank; and what is
      recorded when one happens — an owner and a date, per the convention Phase 9/12 established for
      advisory suppressions. The procedure is then exercised against an induced partial failure on a
      throwaway version or an equivalent recorded exercise; `cargo publish --dry-run` cannot serve,
      because it never reaches the registry's publish endpoint and so can neither create the
      half-published state nor prove recovery from it. An unexercised runbook is labelled untested.

---

## Requirements — Release Artifacts (Phase 21)

*Added 2026-08-25. Forward work, not ingest-derived. `ARTIFACT-*` is the **twenty-second** prefix;
per Roadmap Extension Protocol item 3 no earlier prefix is recycled. Two neighbours are deliberately
**not** reused: `REL-*` is spent on Milestone 1, where it scoped whether the project could be
released at all, and `PUBOPS-*` (Phase 20) scopes what the pipeline **verifies before** it publishes
and how it **recovers** from a half-published run. `ARTIFACT-*` scopes what the release **hands to a
consumer** once publishing succeeds — the notes, the binaries, the image reference and the means to
verify them. As with `SAST-*`, `PUB-*` and `PUBOPS-*`, these IDs are minted at roadmap time rather
than during execution.*

### Release artifacts (ARTIFACT)

- [ ] **ARTIFACT-01**: **The GitHub release body is extracted from the curated `CHANGELOG.md`
      section for that version, and a missing section fails the run rather than falling back.**
      `release.yml`'s `create-release` job builds its body from
      `git log --pretty=format:"- %s" "$PREV_TAG"..HEAD`, discarding a curated Keep-a-Changelog
      corpus the project maintains on purpose: `docs/src/appendix/release-automation.md` records
      choosing a hand-authored `CHANGELOG.md` over Conventional-Commit generation, and `make release`
      finalizes `## [Unreleased]` into `## [VERSION] - <date>` precisely so the section exists at tag
      time. v0.8.0 spans **1,014 commits** (`be2ff05..48ac11a5`), so the current body would be a
      thousand-line dump of `chore:` and `docs(phase-16):` subjects. Extraction begins after the
      `## [X.Y.Z]` heading and stops at the next `## [` heading. **A silent fallback to `git log`
      does not satisfy this requirement** — it reproduces today's defect on exactly the release where
      someone forgot to finalize the changelog. Whether the ten per-crate changelogs also contribute
      is recorded either way.

- [x] **ARTIFACT-02**: **Every binary the release attaches is built with the features its target
      requires, and a leg that produces no executable fails.**
      This is a live defect, not a hardening measure. `[[bin]] paladin` declares
      `required-features = ["cli"]`; `cli` is absent from
      `default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`; `build-binaries` runs a bare
      `cargo build --release --target ${{ matrix.target }}`. Cargo **silently skips** a binary whose
      required features are unmet, so no `target/<triple>/release/paladin` is produced and the next
      step's `strip target/${{ matrix.target }}/release/paladin` is the first thing to notice — on
      all four matrix legs, which run `fail-fast: false`. The build names its feature set explicitly
      and the archive step asserts the expected executables exist before creating a tarball. The
      workspace declares three binaries — `paladin` and `paladin-cli` (`cli`), `paladin-server`
      (`web-server`) — and the release attaches none; which ship, under which features, is decided
      and written down.

- [x] **ARTIFACT-03**: **The release body references only artifacts the run actually produced.**
      The body hardcodes `docker pull ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest`, but the
      `latest` tag is generated by `type=raw,value=latest,enable={{is_default_branch}}` — false on a
      `refs/tags/v*` push. **This workflow has never pushed `latest` on a release**, so the pull
      command it prints has never worked. Image tags, asset names and the advertised platform list
      are all emitted from what the run produced, not from a static template.

- [x] **ARTIFACT-04**: **The published container image is bound to the release by immutable digest,
      and the image-size check stops reporting a problem as a passing run.**
      `build-docker` declares `needs: create-release`, pushes multi-arch images, and records nothing
      back to the release — no digest, no tag list, no link. The `sha256:` digest
      `docker/build-push-action` already returns appears in the release so a consumer can pin to the
      exact image this tag built rather than to a mutable tag. Separately, `Verify image size` emits
      `::warning::` above its 500 MB target and lets the run end green; it either fails or the body
      states the measured size as advisory. An artifact job that reports success while producing
      nothing usable is the Phase 12 defect in a new place.

- [x] **ARTIFACT-05**: **Attached artifacts are verifiable, and the release says how.**
      Per-asset `.sha256` files are produced today and explained nowhere in the release body or in
      `docs/src/appendix/`. The release carries checksums in a form a consumer can verify in one
      command, and identifies the CycloneDX SBOM that `sbom` attaches as covering the **root
      `paladin-ai` package** — `cargo cyclonedx --all` writes one document per crate and the job
      copies `paladin-ai.cdx.json` alone, so an unqualified "SBOM" overstates its scope. Whether
      artifacts are additionally signed or carry build provenance is **decided in this phase and
      recorded with its reasoning**; deferring is acceptable, leaving it unexamined is not.

- [x] **ARTIFACT-06**: **The artifact path runs on maintained actions and carries no branch for a
      target that is not built, and the whole path is exercised on a throwaway tag.**
      `actions/create-release@v1` and `actions/upload-release-asset@v1` have been archived upstream
      since 2021, and the `upload_url` plumbing exists only to serve them — `build-binaries` and
      `sbom` both consume `needs.create-release.outputs.upload_url`, so replacing them is
      coordinated with Phase 20's rewrite of `create-release` rather than duplicated. In the same
      pass, `build-binaries`'s `if: matrix.os != 'windows-latest'` guard on the `strip` step is
      removed: the matrix has no Windows leg, so the condition implies a target the release does not
      ship. **Evidence is a real run on a throwaway tag** whose assets download and verify, whose
      image pulls by the digest the release names, and whose body matches the `CHANGELOG.md` section
      for that version. Re-reading the workflow is not evidence — ARTIFACT-02's defect survived every
      previous reading of this file. If the rehearsal is not run, the path is recorded as unverified
      rather than presented as working.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SAST-01 | Phase 18 | Complete |
| SAST-02 | Phase 18 | Complete |
| SAST-03 | Phase 18 | Complete |
| SAST-04 | Phase 18 | Complete |
| PUB-01 | Phase 19 | Complete |
| PUB-02 | Phase 19 | Complete |
| PUB-03 | Phase 19 | Complete |
| PUB-04 | Phase 19 | Complete |
| PUB-05 | Phase 19 | Complete |
| PUBOPS-01 | Phase 20 | Complete |
| PUBOPS-02 | Phase 20 | Complete |
| PUBOPS-03 | Phase 20 | Complete |
| PUBOPS-04 | Phase 20 | Complete |
| PUBOPS-05 | Phase 20 | Complete |
| ARTIFACT-01 | Phase 21 | Pending |
| ARTIFACT-02 | Phase 21 | Complete |
| ARTIFACT-03 | Phase 21 | Complete |
| ARTIFACT-04 | Phase 21 | Complete |
| ARTIFACT-05 | Phase 21 | Complete |
| ARTIFACT-06 | Phase 21 | Complete |

**Coverage:**

- v0.9.0 requirements: **20 total** (4 Security Tooling, Phase 18; 5 Publishing Auth, Phase 19;
  5 Publish Operations, Phase 20; 6 Release Artifacts, Phase 21)

- Mapped to phases: 20
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓
- Phases carrying no requirement ID: 0

**Prefix register.** Twenty-two prefixes are spent and none may be recycled, per *Roadmap Extension
Protocol* item 3: `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` (Milestone 1); `VERIFY-*`, `CLOSE-*`
(Milestone 2-3); `ARCH-*`, `DEBT-*` (Milestone 4-6); `SEC-*`, `HARD-*`, `FACADE-*` (Milestone 7-8);
`SUPPLY-*`, `ORCH-*`, `WEB-*`, `PIPE-*`, `DEFER-*`, `DOCS-*` (Milestone 9-12 + Deferred-QA);
`PROV-*` (Provider Expansion); `SAST-*` (Security Tooling); `PUB-*` (Publishing Auth, added
2026-08-25); `PUBOPS-*` (Publish Operations, added 2026-08-25); `ARTIFACT-*` (Release Artifacts, added
2026-08-25). Ingested `REQ-*` IDs remain the stable merge keys.

**Phase 15.1 remains the one phase in this project's history with no requirement identifier**, by
its own recorded decision (plan `15.1-10`, D-00f, 2026-08-14) rather than by oversight. That record
lives in the v0.8.0 archive; it is noted here so the convention is not mistaken for an accident and
repeated by default. New phases mint their IDs at roadmap time.

---
*Requirements opened: 2026-08-24 at the v0.8.0 milestone close.*
