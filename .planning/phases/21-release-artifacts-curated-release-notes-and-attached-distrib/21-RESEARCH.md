# Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables - Research

**Researched:** 2026-08-31
**Domain:** GitHub Actions release-workflow engineering (`gh` CLI, Docker Buildx metadata, Cargo
feature-gated binaries, shell checksum portability)
**Confidence:** MEDIUM-HIGH — the mechanics that could be tested locally (native `--features
cli,web-server` build, `gh` CLI flag semantics, dependency graph for the aarch64 cross leg) were
tool-verified in this session; the parts that require the live GitHub Actions/ghcr.io/crates.io
environment (digest output exact shape, aarch64 cross build, body size in practice) are CITED from
official docs/action metadata or corroborated by dependency analysis, and are explicitly flagged
for confirmation during the D-14 rehearsal rather than presented as proven.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Release notes for `vX.Y.Z` are extracted from the root `CHANGELOG.md` `## [X.Y.Z]`
  section by a repo script (logic-in-scripts pattern). Extraction begins after the `## [X.Y.Z]`
  heading, stops at the next `## [` heading. A tag whose version has no matching section fails
  `create-release` — no `git log` fallback in any form. The git-log changelog generation step is
  deleted, not bypassed.
- **D-02:** An empty section (heading only) is allowed — presence is the contract, content is the
  author's.
- **D-03:** The ten per-crate changelogs do not contribute to the release body — root section
  only.
- **D-04:** Assembly order inverts the current "advertise first, build later" shape.
  `create-release` creates/reuses the release with the curated changelog section as its body —
  nothing else. A new terminal job (`finalize-release-body`, `needs: [create-release, build-docker,
  build-binaries, sbom]`) appends artifact sections from real job outputs via `gh release
  edit`/`gh api` with the body passed as a file (CR-01 discipline). A leg that failed or was
  skipped gets its section omitted or stated as absent — never advertised. The finalize job
  recomputes the body idempotently from current outputs so both GitHub re-run shapes stay safe.
- **D-05:** All three declared binaries ship: `paladin` + `paladin-cli` (need `cli`) and
  `paladin-server` (needs `web-server`). One build per target with explicit `--features
  cli,web-server` (defaults remain on: openai + anthropic + deepseek). The aarch64 leg composes
  `vendored-openssl` on top, as today. **Researcher must verify** `cli,web-server` cross-compiles
  cleanly under `cross` for `aarch64-unknown-linux-gnu`; if a target provably cannot build a
  binary, that binary is dropped from that target's manifest explicitly (assert list per target)
  and recorded — not silently skipped.
- **D-06:** The archive step asserts every expected executable exists before creating the tarball
  — a missing binary fails the leg loudly. The tarball contains all three binaries; the existing
  `paladin-<os>-<arch>.tar.gz` asset naming is kept. `strip` runs on each binary that shipped.
- **D-07:** `actions/upload-release-asset@v1` is replaced with `gh release upload --clobber` in
  `build-binaries` and `sbom`. The `upload_url` output and all its plumbing are deleted from
  `create-release`, `build-binaries`, and `sbom`. The `version` output stays.
- **D-08:** The `if: matrix.os != 'windows-latest'` guard on the strip step is removed.
- **D-09:** `build-docker` captures the `sha256:` digest from `docker/build-push-action`'s
  `digest` output and the actually-pushed tag list from `metadata-action`, exposed as job outputs.
  The finalize job writes a pinnable `docker pull ghcr.io/<image>@sha256:…` line plus the real tag
  list. The `:latest` pull instruction is deleted.
- **D-10:** The image-size check takes the advisory branch: measured size in MB stated in the
  release body as advisory against the 500 MB target, replacing `::warning::`-then-green with
  honest reporting. Not converted to a hard failure now.
- **D-11:** A single aggregated `SHA256SUMS` file covering every uploaded binary archive is
  generated and attached alongside the existing per-asset `.sha256` files, and the release body
  carries the one-command verification (`sha256sum -c SHA256SUMS`, macOS `shasum -a 256 -c`
  variant noted). Generation must happen where all archives are visible — mechanism at planner
  discretion; the binding constraint is the sums file covers what was actually uploaded.
- **D-12:** The body identifies the attached CycloneDX SBOM as covering the root `paladin-ai`
  package, not the whole workspace.
- **D-13:** Signing and build provenance are deferred, with reasoning recorded in
  `docs/src/appendix/release-automation.md` (not only planning files).
- **D-14:** The path is proven on a real throwaway tag on the current prerelease line (next free
  rc). Evidence recorded in `21-ARTIFACT-EVIDENCE.md`. If the rehearsal is not run, the artifact
  path is recorded as unverified.
- **D-15:** `docs/src/appendix/release-automation.md` updated with body source, artifact
  inventory, verification instructions, D-13 signing decision; `release-checklist.md` picks up
  operator-visible changes. Trigger-policy table untouched.

### Claude's Discretion

- Whether changelog extraction extends `create-or-reuse-release.sh` or is its own script; exact
  script/job names.
- The finalize job's exact mechanism (`gh release edit` vs `gh api` PATCH) and body-section
  layout, provided failed legs are never advertised and re-runs are idempotent.
- How `SHA256SUMS` aggregation sees all archives (job outputs vs downloading released assets).
- The rehearsal rc version string and the order of rehearsal vs. any needed fix-up commits.
- Whether the body links to per-crate changelogs.

### Deferred Ideas (OUT OF SCOPE)

- Hard-fail image-size threshold (candidate follow-up after D-14 records a measured size).
- Artifact signing / build provenance (cosign or `actions/attest-build-provenance`) — examined,
  deferred with reasoning per D-13.
- Windows (or additional) build targets.
- Per-crate changelog content in release notes.
- The real stable catch-up release (operator act).

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ARTIFACT-01 | Release body extracted from curated `CHANGELOG.md` section; missing section fails the run, no `git log` fallback | See "Changelog Extraction Mechanics" and Code Examples; `check-release-consistency.sh` Clause 2's regex pattern reused/adapted for the extraction script's own heading match |
| ARTIFACT-02 | Every advertised binary is actually built under the features its target requires; a leg producing no executable fails | Native `--features cli,web-server` build VERIFIED in this session (all 3 binaries produced); aarch64 cross-compile risk assessed via dependency-graph analysis (MEDIUM confidence, not live-tested — Docker unavailable in research sandbox) |
| ARTIFACT-03 | Release body references only artifacts the run produced | See "Idempotent Body Composition" pattern (marker-based truncate-and-rebuild) |
| ARTIFACT-04 | Docker image bound by immutable digest; image-size check stops reporting a problem as a passing run | `docker/build-push-action` `digest` output CITED from `action.yml`; exact `sha256:`-prefixed format ASSUMED from established convention, flagged for D-14 confirmation |
| ARTIFACT-05 | Checksums verifiable in one command with instructions in release; SBOM scope identified; signing decided-or-deferred | `gh release download`/`gh release upload` mechanics VERIFIED via local `gh --help`; macOS `sha256sum` absence is a newly-surfaced, CITED, live pitfall affecting this exact requirement |
| ARTIFACT-06 | Archived actions and dead matrix branches removed; `upload_url` plumbing gone; proven end-to-end on a throwaway tag or recorded unverified | See Pitfalls and Validation Architecture; D-14 rehearsal is the actual proof, not this document |

</phase_requirements>

## Summary

This phase is CI/workflow engineering, not application-library integration — there is no new
runtime dependency to standardize on. The four technical surfaces are: (1) a bash/`awk`-or-`sed`
changelog-section extractor that must exactly mirror the heading-boundary semantics
`check-release-consistency.sh` Clause 2 already implements against the same files; (2) a Cargo
feature/target matrix fix that this session **directly verified** builds all three binaries
natively on x86_64 (`cargo build --bins --features cli,web-server` succeeded, producing `paladin`,
`paladin-cli`, `paladin-server`), with the `aarch64-unknown-linux-gnu` `cross` leg assessed as
low-risk by dependency-graph inspection but not live-verified (no Docker in this sandbox — D-14's
rehearsal is where this gets proven for real); (3) a `gh release edit`/`gh api` body-composition
step whose only safe idempotent shape is "always rebuild from a fixed marker," because `gh release
edit --notes-file` **replaces** the whole body rather than appending (verified locally via `gh
release edit --help`) — a naive "fetch body, append, write back" implementation would grow the
body by one artifact-section copy on every re-run, eventually risking the ~125,000-character GitHub
release body cap; and (4) checksum/asset handling where a real, previously-masked defect was found:
**`sha256sum` does not exist on GitHub's `macos-latest` runners** (multiple corroborating GitHub
issues, including `actions/runner-images#90`), so the two macOS matrix legs' existing `sha256sum
… .tar.gz > …sha256` step has never actually produced a checksum — it has always failed one step
earlier (in `strip`, on the ARTIFACT-02 defect) before reaching that line. Fixing ARTIFACT-02
without also fixing this exposes it for the first time on exactly the runners that need the D-11
aggregation to work.

**Primary recommendation:** build the changelog extractor and the finalize-body composer as
small, unit-tested repo scripts following the existing `tests/scripts/*_test.sh` fixture-harness
pattern (never inline YAML logic); use a fixed HTML-comment marker in the release body to make the
finalize job's read-modify-write idempotent by *truncate-and-rebuild* rather than *append*; make
every checksum-producing shell step OS-portable (`sha256sum` if present, else `shasum -a 256`)
rather than assuming GNU coreutils; and treat the aarch64 `cli,web-server` cross-build and the
Docker digest's exact wire format as open items the D-14 rehearsal settles, not as facts this
document can assert without a live CI run.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Changelog-section extraction | CI/Build (repo script) | — | Pure text processing over a tracked file; no runtime component, must be identical logic to `check-release-consistency.sh`'s Clause 2 heading match so the two never disagree |
| Release creation (body = curated section only) | CI/Build (`create-release` job) | — | Already Phase 20's `create-or-reuse-release.sh`; this phase only changes what `--body-file` contains |
| Binary build matrix | CI/Build (`build-binaries` job, 4-leg matrix) | — | Cargo feature/target resolution; no application-tier concern |
| Docker image build + digest capture | CI/Build (`build-docker` job) | Registry (ghcr.io) | The digest is registry-issued content-addressing; the workflow only captures and republishes it |
| SHA256SUMS aggregation | CI/Build (new step, in finalize job or a dedicated aggregation step) | — | Must see every archive across 4 fanned-out matrix legs — requires either GH Actions artifacts or `gh release download` from the already-uploaded assets |
| Release-body finalization (artifact sections) | CI/Build (`finalize-release-body` job) | GitHub Releases API | Writes to the GitHub-hosted release object; must be idempotent against GitHub's own re-run semantics |
| Documentation of the artifact path | Docs (`docs/src/appendix/`) | — | Consumer-facing; not code, but D-13/D-15 bind it to this phase |

## Package Legitimacy Audit

**Not applicable.** This phase adds no new Cargo dependencies, no new npm/pip packages, and (per
D-01/D-07) no new GitHub Marketplace Actions. The only "packages" it touches are Actions already
pinned in `release.yml` (`docker/build-push-action@v5`, `docker/metadata-action@v5`,
`dtolnay/rust-toolchain@<sha>`, `actions/checkout@v4`) — all already present and unchanged by this
phase's scope — and the `gh` CLI, which is GitHub's own first-party tool preinstalled on every
GitHub-hosted runner (no install/pin step needed). `cargo install cross --locked --version 0.2.5`
is existing, unchanged code (only the feature flags passed to `cross build` change under D-05).

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | — | No new packages introduced by this phase |

**Packages removed due to [SLOP] verdict:** none.
**Packages flagged as suspicious [SUS]:** none.

## Changelog Extraction Mechanics (ARTIFACT-01)

**Pattern to reuse, not reinvent:** `scripts/check-release-consistency.sh` Clause 2 already
implements the identical heading-match semantics this extraction needs, in Python embedded in the
script:

```python
heading_re = re.compile(r"^##\s*\[" + re.escape(tag_version) + r"\](\s|$)")
```

This anchors immediately after the bracketed version so `0.8.1` never matches `0.8.1-rc.2` and
`0.8.10` never matches `0.8.1`. **The extraction script for ARTIFACT-01 must use the same
anchoring discipline for its *start* boundary**, and additionally implement a *stop* boundary: the
next line matching `^##\s*\[` (any version) or end-of-file. D-01 requires the extraction begin
*after* the heading line (the heading itself is not included in the body — the curated content
starts extraction only) and stop *before* the next `## [` heading — this is a strict "everything
between two headings" slice, not "grep from heading to EOF."

**Version-string edge cases confirmed against `CHANGELOG.md` as it exists today** (verified by
direct read):
- `0.8.1-rc.4` — prerelease suffix with a literal hyphen and dot; the section is **heading-only
  (empty)** — `## [0.8.1-rc.4] - 2026-08-29` immediately followed by `## [0.8.1-rc.3]` with no
  intervening body text. This is the live D-02 case: extraction of this section correctly returns
  an empty (or whitespace-only) string, and D-02 requires this to be accepted, not treated as a
  missing-section failure. The **presence of the heading line** is the pass/fail signal, not the
  presence of body content after it.
- A dated suffix (`- 2026-08-29`) trails every non-`Unreleased` heading; the extraction regex must
  match on the bracketed version only and treat everything after `]` on the heading line as
  ignorable, exactly as `check-release-consistency.sh`'s `(\s|$)` lookahead already does.
- Tag `v` prefix: `check-release-consistency.sh` strips at most one leading `v` via `${TAG#v}`
  before comparison (`"v1.2.3"` → `"1.2.3"`, `"1.2.3"` unchanged). The extraction script must do
  the identical strip — `release.yml`'s `get_version` step already computes
  `${GITHUB_REF#refs/tags/}` (a full tag string like `v0.8.1-rc.5`), so the extraction script needs
  the same `${TAG#v}` normalization applied to the same input `check-release-consistency.sh`
  receives, not a second independent parsing of the ref.

**Missing-section failure message (from the phase's own `<specifics>`):** "no `## [X.Y.Z]` section
in CHANGELOG.md — run `make release VERSION=X.Y.Z` (finalizes changelogs) before tagging" — this
should be the literal `::error::`-prefixed message the extraction script emits, matching the
house style already used in `check-release-consistency.sh`'s failure reports (named failure code
+ actionable remedy, never a bare stack trace).

**Recommended implementation shape:** a small, `LIB_ONLY`-sourceable bash function
(`extract-changelog-section.sh`, matching every other script in `scripts/`'s sourcing-seam
convention) that takes `--changelog <path> --version <X.Y.Z>` and either prints the extracted
section to stdout (writing it to a `$RUNNER_TEMP` file for `--body-file`, exactly as the current
git-log-based changelog generation step already does — CR-01 discipline, never inline the section
text into a `run:` body) or exits non-zero with the missing-section message. Unit-test it the same
way `finalize-crate-changelogs_test.sh` and `check-release-consistency_test.sh` do: a fixture
directory with a scripted `CHANGELOG.md`, no network, no `cargo metadata` call needed (this script
touches only a text file).

## Standard Stack

### Core

| Tool | Version (as pinned in this repo) | Purpose | Why Standard |
|------|---------|---------|--------------|
| `gh` CLI | preinstalled on GH-hosted runners (2.97.0 confirmed in this session's devcontainer) | Release create/reuse/edit/upload/download, all HTTP to the Releases API | Already authenticated via `GITHUB_TOKEN`; Phase 19/20's established "no new marketplace action" posture (D-01, D-07) |
| `docker/build-push-action` | `@v5`, already pinned in `release.yml` | Multi-arch image build+push; exposes `digest` output | Already in use; only its output is newly consumed |
| `docker/metadata-action` | `@v5`, already pinned | Tag/label computation; `json` output already consumed by the existing `Verify image size` step | Already in use; the same `steps.meta.outputs.json` the size check already reads is the correct source for the pull-line's exact tag, per the workflow's own existing comment warning against hand-reconstructing the reference |
| `cross` | `0.2.5`, already pinned (`cargo install cross --locked --version 0.2.5`) | aarch64 cross-compilation container | Unchanged by this phase; only the `--features` flag passed to it changes |
| `cargo-cyclonedx` | already pinned (`cargo install cargo-cyclonedx --locked`) | SBOM generation | Unchanged; D-12 only relabels what the body says about its scope |

### Supporting

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `sha256sum` (GNU coreutils) | Checksum generation on Linux runners (`ubuntu-latest`) | Present by default on `ubuntu-latest` |
| `shasum -a 256` (Perl-based, BSD-adjacent) | Checksum generation on macOS runners | **Required** on `macos-latest` — `sha256sum` is absent there (see Pitfalls) |
| `jq` | JSON extraction from `steps.meta.outputs.json` / `gh api` responses | Already used in the existing `Verify image size` step; reuse the identical pattern for the digest/tag-list capture |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `gh release download` for SHA256SUMS aggregation | `actions/upload-artifact` + `actions/download-artifact` fan-in | Artifacts approach avoids a second network round-trip through the Releases API, but introduces a second, parallel asset-transport mechanism alongside the already-established `gh release upload` path, and both `actions/upload-artifact@v4`/`download-artifact@v4` must be added and pinned as new-to-this-workflow Actions. `gh release download` reuses the tool already authenticated and already the sole upload mechanism (D-07), at the cost of one extra `gh` round trip per archive in the finalize job. **Recommended: `gh release download`**, consistent with the repo's "no new marketplace action" posture. |
| `gh release edit --notes-file` (whole-body replace) for the finalize job | `gh api PATCH .../releases/{id}` with a hand-built JSON body via `jq -n` | Functionally equivalent — `gh release edit --notes-file` is a thin wrapper over the same PATCH endpoint. `gh release edit` is simpler and already the tool of choice elsewhere in this pipeline; `gh api` + `jq -n` is what `create-or-reuse-release.sh` uses today because it needs `-i`/status-code introspection that `gh release edit` does not expose. The finalize job does not need status-code introspection (a non-2xx exit from `gh release edit` is already a hard failure), so `gh release edit --notes-file` is the simpler, sufficient choice. |
| One aggregated `SHA256SUMS` via a dedicated finalize step | Have each matrix leg append to a shared file | **Not viable** — the four `build-binaries` matrix legs run on isolated runners with no shared filesystem; there is no way for leg N to see leg N-1's file without an external transport (Actions artifacts or the Releases API), which is exactly what D-11 says "mechanism at planner discretion" is deciding between. |

**Installation:** No new installation steps. All tools above are either GitHub-runner-preinstalled
(`gh`, `sha256sum`/`shasum`, `jq`) or already-pinned in `release.yml` (`docker/build-push-action`,
`docker/metadata-action`, `cross`, `cargo-cyclonedx`).

**Version verification (native x86_64 build, VERIFIED this session):**

```
$ rustc --version
rustc 1.97.1 (8bab26f4f 2026-07-14)
$ cargo build --bins --features cli,web-server
   Compiling paladin-ai-core v0.8.1-rc.4 ... (11 workspace crates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 46.73s
$ ls -la target/debug/{paladin,paladin-cli,paladin-server}
-rwxr-xr-x paladin        50130528 bytes
-rwxr-xr-x paladin-cli    84848448 bytes
-rwxr-xr-x paladin-server 61966568 bytes
```

All three `[[bin]]` targets built and produced real executables under `--features cli,web-server`
on the exact toolchain version `rust-toolchain.toml` pins (`1.97.1`), matching what `build-binaries`
reads via `dtolnay/rust-toolchain` + the `Read pinned Rust channel` step. This is direct evidence
for the x86_64 half of D-05's verification requirement — **debug profile**, not release (this
session did not run the `lto = true, codegen-units = 1` release profile to keep the check fast;
release-profile compilation succeeding is a fair inference from a successful debug build of the
same crate graph plus the fact that `release.yml`'s existing `cargo build --release` step already
compiles this crate graph successfully today for the library-only target — it just skips the
binaries. Nothing about `--release` changes feature resolution.)

## Aarch64 Cross-Compilation Risk Assessment (D-05)

**Could not be live-verified in this research session — Docker is unavailable in this sandbox**
(`docker info` failed; `cross` is not installed). This is stated plainly rather than inferred as
proven. What follows is a dependency-graph risk assessment, not a build proof.

**Dependency analysis (VERIFIED by direct `Cargo.toml` inspection):**
- The `cli` feature's new dependencies (`clap`, `dialoguer`, `indicatif`, `console`, `serde_yaml`,
  `colored`, `comfy-table`, plus `paladin-herald/table` and `paladin-herald/color`) are all
  pure-Rust crates with no `-sys`/C-toolchain dependency of their own.
- `crates/paladin-web/Cargo.toml` (the `web-server` feature's crate) depends on `axum`, `tower`,
  `tower-http`, `tower_governor`, `utoipa`, `utoipa-axum`, `utoipa-swagger-ui` — again all
  pure-Rust, no native/`-sys` crates.
- The **only** native/C dependency anywhere in this crate graph is `openssl-sys` (pulled
  transitively via `reqwest`'s TLS backend), and that dependency is **already exercised today** by
  the aarch64 `cross build --release --target aarch64-unknown-linux-gnu --features
  vendored-openssl` step — it is not new load this phase introduces. What *is* new is that the
  binary-target code paths (`src/main.rs`, `src/bin/paladin-cli.rs`, `src/bin/paladin-server.rs`)
  have never been compiled under `cross` for aarch64 at all, because today's bare `cargo build`
  (no `--features`) never triggers `required-features = ["cli"]`/`["web-server"]` on any leg — the
  binaries are silently skipped, so only the *library* target has ever been cross-compiled for
  aarch64 to date.

**Assessment: MEDIUM confidence, no known blocking incompatibility identified**, but genuinely
unverified. **Recommendation:** treat D-14's rehearsal as the actual gate for this decision, per
D-05's own instruction ("if a target provably cannot build a binary, that binary is dropped from
that target's manifest explicitly ... and recorded"). The plan should NOT assume the aarch64 leg
ships all three binaries as a given — it should build the assert-list-per-target mechanism (D-06)
generically enough that dropping one target's binary set is a one-line manifest change, not a
structural rewrite, precisely because this question is open until the rehearsal runs.

## Architecture Patterns

### System Architecture Diagram

```
 tag push (v*.*.*)
        │
        ▼
 verify-tag-source ──(ancestor-of-main check)──▶ [gate: fail if not on main]
        │
        ├──────────────┬───────────────┬─────────────────────┐
        ▼              ▼               ▼                     ▼
     test          create-release  check-release-      (parallel legs)
  (cargo test)     (D-01 extract    consistency
                    changelog       (Phase 20,
                    section →       unchanged)
                    body-only
                    release)
        │              │
        │   ┌──────────┼──────────────────┬───────────────────┐
        │   ▼          ▼                  ▼                   ▼
        │ build-docker build-binaries    sbom          (each needs:
        │ (digest +    (4-leg matrix,    (paladin-ai.  create-release
        │  tag-list    assert-then-      cdx.json)     only — WR-05
        │  outputs)     archive, gh                    asymmetry
        │               release upload                 preserved)
        │               --clobber)
        │   │          │                  │
        │   └──────────┴────────┬─────────┘
        │                       ▼
        │            finalize-release-body
        │            (needs: create-release,
        │             build-docker,
        │             build-binaries, sbom)
        │            — reads current release
        │              body, truncates at a
        │              fixed marker, appends
        │              freshly-composed
        │              artifact sections
        │              (digest+tags, asset
        │              list + SHA256SUMS
        │              instructions, SBOM
        │              scope, image-size
        │              advisory), gh release
        │              edit --notes-file
        │
        ▼
 publish-crates (needs: test, create-release,
                 check-release-consistency)
 — untouched by this phase
```

### Recommended Project Structure

No new source directories — this phase edits existing files:

```
.github/workflows/release.yml    # create-release, build-docker, build-binaries,
                                  # sbom rewritten; new finalize-release-body job
scripts/
├── extract-changelog-section.sh # NEW — ARTIFACT-01 (or extends create-or-reuse-release.sh,
│                                 # planner discretion per CONTEXT.md)
├── create-or-reuse-release.sh   # unchanged (Phase 20) — still takes --body-file
└── ...                          # existing gate/finalize scripts, unchanged
tests/scripts/
└── extract-changelog-section_test.sh  # NEW — fixture-based, mirrors existing pattern
docs/src/appendix/
├── release-automation.md        # D-13/D-15 updates
└── release-checklist.md         # D-15 updates
```

### Pattern 1: Idempotent Body Composition (Marker-Based Truncate-and-Rebuild)

**What:** `gh release edit --notes-file <path>` **replaces the entire release body** — confirmed
locally this session (`gh release edit --help`: "Update the release notes from the content of a
file"; there is no `--append` flag). A finalize job that runs more than once (either GitHub re-run
shape — Phase 20 D-03) must therefore never *append* to whatever body is currently on the release,
or the body grows by one duplicate artifact-section block per re-run, eventually risking GitHub's
~125,000-character release-body cap (CITED: multiple `cli/cli` and `changesets/action` GitHub
issues report a hard `422` at this limit).

**When to use:** Any time a workflow job composes a release body across more than one job in the
`needs` graph and must tolerate re-runs.

**Recommended shape:**

```bash
# In finalize-release-body, after computing $DIGEST_LINE, $ASSET_LIST, $SBOM_LINE, $SIZE_LINE:
MARKER='<!-- gsd:release-artifacts -->'
CURRENT_BODY=$(gh release view "$TAG" --json body -q .body)
# Truncate at the marker if it exists (a prior finalize run left it); if the marker is
# absent (first run), the whole current body is the curated changelog section and is kept
# in full.
CURATED_SECTION="${CURRENT_BODY%%"$MARKER"*}"
{
  printf '%s\n' "$CURATED_SECTION"
  printf '%s\n' "$MARKER"
  printf '\n---\n\n## Release Artifacts\n\n'
  printf '%s\n' "$DIGEST_LINE" "$ASSET_LIST" "$SBOM_LINE" "$SIZE_LINE"
} > "$RUNNER_TEMP/final-body.md"
gh release edit "$TAG" --notes-file "$RUNNER_TEMP/final-body.md"
```

This makes the finalize job idempotent by construction — running it N times reproduces byte-for-byte
the same body every time (given the same job outputs), which is exactly Phase 20 D-03's binding
constraint applied to this phase's new job. Bash's `${var%%pattern*}` (longest-match trim from the
end) is POSIX-portable and avoids a `sed`/`awk` dependency for this specific operation.

### Pattern 2: Existence-Assert Before Archive (D-06)

```bash
BINARIES=(paladin paladin-cli paladin-server)   # per-target list, may be shorter on a
                                                  # target where D-05's assert-list narrows it
MISSING=()
for b in "${BINARIES[@]}"; do
  if [ ! -f "target/${{ matrix.target }}/release/$b" ]; then
    MISSING+=("$b")
  fi
done
if [ "${#MISSING[@]}" -gt 0 ]; then
  echo "::error::expected binaries not built for ${{ matrix.target }}: ${MISSING[*]}"
  exit 1
fi
```

### Pattern 3: OS-Portable Checksum Generation (newly-surfaced pitfall, see below)

```bash
sha256_cmd() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$@"
  else
    shasum -a 256 "$@"
  fi
}
```

### Anti-Patterns to Avoid

- **Appending to the current release body without a marker:** grows unboundedly across re-runs;
  the exact failure class D-04's "recomputes the body idempotently" language exists to prevent.
- **Hand-reconstructing the ghcr.io image reference** instead of reading `steps.meta.outputs.json`
  — the existing `Verify image size` step's own comment already documents why (lowercasing,
  leading-`v` stripping) and this phase's digest line must follow the same discipline.
- **Assuming `sha256sum` exists everywhere** — see Pitfalls.
- **Treating a green `cargo build --release` (no `--features`) as evidence the binaries build** —
  this is the exact defect ARTIFACT-02 exists to fix; Cargo's silent skip of unmet
  `required-features` binaries produces no error and no warning.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Release create/reuse-by-tag idempotency | A custom `gh api` polling/retry wrapper | `scripts/create-or-reuse-release.sh` (Phase 20, already exists, already tested) | Already handles the 200/404/422-race decision table correctly; this phase only changes what it's given as `--body-file`, never its own logic |
| Asset upload idempotency | A custom delete-then-upload sequence | `gh release upload --clobber` | Already does delete-then-upload internally (confirmed via `gh release upload --help`); reimplementing it manually would just reproduce the same interrupted-upload data-loss window with more code |
| Changelog-section boundary detection | A second, slightly-different regex from `check-release-consistency.sh`'s | Match `check-release-consistency.sh` Clause 2's exact anchoring (`^##\s*\[<version>\](\s|$)`) | Two independently-maintained heading regexes for the same file is exactly the "four divergent surfaces" defect class this project's own history (SEC-01, the RustSec exception sprawl) has already paid for once; ARTIFACT-01's extractor and the consistency gate's Clause 2 must never be able to disagree about where a section starts |
| Docker image reference reconstruction | Manually lowercase `github.repository` and strip a leading `v` from the semver tag | `steps.meta.outputs.json` (`docker/metadata-action`'s own output) | The existing `Verify image size` step's comment already names this exact hazard and already established the correct pattern; reuse it rather than re-deriving |

**Key insight:** every "don't hand-roll" item above already has a working, tested implementation
somewhere in this same workflow or `scripts/` directory from Phases 19-20. This phase's actual net
scope is smaller than the roadmap prose suggests once the reusable pieces are identified — the new
work is the extraction script, the finalize job's composition logic, the feature-flag fix on the
build matrix, and the checksum-aggregation step.

## Common Pitfalls

### Pitfall 1: `sha256sum` does not exist on `macos-latest` GitHub-hosted runners

**What goes wrong:** The existing `Create archive` step runs `sha256sum
${{ matrix.artifact_name }}.tar.gz > ${{ matrix.artifact_name }}.tar.gz.sha256` unconditionally
across all four matrix legs, including the two `macos-latest` legs. GitHub's `macos-latest` runner
image does **not** ship GNU coreutils' `sha256sum` by default (CITED: `actions/runner-images#90`,
and multiple independent project issues reporting the identical failure — e.g.
`taiki-e/upload-rust-binary-action#30`). macOS ships `shasum -a 256` instead (part of Perl's
`Digest::SHA`, present by default).

**Why it happens:** This has been **masked until now** by ARTIFACT-02's own defect: today, `strip
target/${{ matrix.target }}/release/paladin` (the step immediately before `Create archive`) already
fails on every leg, because the `paladin` binary is never built (missing `required-features =
["cli"]`). The workflow has never reached the `sha256sum` line on a macOS runner in production.
**Fixing ARTIFACT-02 without also fixing this pitfall will surface a new, previously-invisible
failure on exactly the two macOS legs**, at exactly the point D-11's per-asset `.sha256` files and
the aggregated `SHA256SUMS` depend on.

**How to avoid:** Use the OS-portable `sha256_cmd()` pattern (Pattern 3 above) everywhere a
checksum is generated in `build-binaries` — both the per-asset `.sha256` file and, if the
aggregation step runs on a matrix leg rather than a dedicated `ubuntu-latest` finalize step, the
`SHA256SUMS` computation.

**Warning signs:** A macOS leg failing with `sha256sum: command not found` (exit 127) immediately
after a successful `Create archive` step — this is the specific, previously-impossible-to-observe
failure this fix must anticipate rather than discover live during the D-14 rehearsal.

### Pitfall 2: `gh release edit --notes-file` replaces, never appends

**What goes wrong:** A finalize-body implementation that does `gh release edit "$TAG" -n "$(cat
current_body.md)$NEW_SECTION"` naively, without truncating at a fixed marker first, will
duplicate the artifact section on every re-run (see Pattern 1 above for the fix). Not a
hypothetical — this is literally how `gh release edit`'s single-body-replace semantics interact
with the two-plus-times-idempotent requirement D-03/D-04 impose.

**How to avoid:** Marker-based truncate-and-rebuild (Pattern 1).

### Pitfall 3: `SHA256SUMS` generated from a matrix leg only covers that leg's own asset

**What goes wrong:** Because `build-binaries` fans out across 4 isolated runners with no shared
filesystem, a `SHA256SUMS` computed inside any single matrix leg can only ever contain that leg's
own archive — never all four. This is not a coding bug to catch in review; it's a structural
consequence of GitHub Actions matrix isolation that must be designed around from the start (D-11
explicitly flags this as the open mechanism decision).

**How to avoid:** Generate `SHA256SUMS` in a dedicated step that runs *after* all matrix legs and
`sbom` complete and can see every uploaded archive — either the `finalize-release-body` job itself
(if it runs on `ubuntu-latest`, `sha256sum` is available directly) via `gh release download "$TAG"
--pattern '*.tar.gz'` followed by `sha256sum *.tar.gz > SHA256SUMS`, or a small dedicated
aggregation job with the same `needs` edges. **Recommended: fold it into `finalize-release-body`**
— it already needs `build-binaries` in its `needs` graph for the asset-list section, so no new
`needs` edge is required, and it already runs on `ubuntu-latest` where `sha256sum` exists natively
(no portability fallback needed for this specific step, unlike the per-leg `.sha256` files in
Pitfall 1).

### Pitfall 4: Docker image reference must be lowercase; `github.repository` is not

**What goes wrong:** `IMAGE_NAME: ${{ github.repository }}` resolves to `DF3NDR/paladin-dev-env`
(mixed case, matching the actual GitHub repo casing). ghcr.io requires an all-lowercase reference.
The existing `Verify image size` step's own comment already documents this exact hazard and
already resolves it correctly by reading `steps.meta.outputs.json | jq -r '.tags[0]'` rather than
interpolating `${REGISTRY}/${IMAGE_NAME}` by hand. The digest-binding pull line this phase adds
must follow the identical pattern — pairing the digest output with a **hand-reconstructed** image
name would silently produce an invalid, never-tested `docker pull` instruction, reproducing
exactly the ARTIFACT-03 defect this phase exists to close, just in a new line.

**How to avoid:** `IMAGE_REF=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')` then
`echo "docker pull ${IMAGE_REF%%:*}@${{ steps.build.outputs.digest }}"` (strip the tag suffix,
append `@sha256:...`), reusing the exact JSON-extraction idiom already proven in this file.

### Pitfall 5: GitHub release body has a ~125,000-character cap

**What goes wrong:** Not a realistic risk for a single `CHANGELOG.md` version section plus a short
artifact-sections block under normal operation, but worth a defensive note given D-04's
idempotency requirement — a body-composition bug that fails to truncate at the marker (Pitfall 2)
combined with enough re-runs could theoretically approach it. CITED: multiple `cli/cli` /
`changesets/action` GitHub issues confirm `422` at 125,000 characters specifically for release
bodies (distinct from the 65,536-character limit that applies to PR/issue comments — do not
confuse the two when writing failure-handling code).

**How to avoid:** The marker-based truncate-and-rebuild pattern (Pattern 1) makes this a non-issue
by construction — the body length is bounded by the curated section (bounded by what a human
writes) plus one fixed-size artifact block, regardless of re-run count.

### Pitfall 6: `windows-latest` strip guard removal — verify it is truly dead

**What goes wrong:** D-08 requires removing `if: matrix.os != 'windows-latest'`. Before deleting
it, confirm (as CONTEXT.md's ground-truth section already states) that the matrix genuinely has no
Windows leg — verified directly by reading `release.yml`'s `build-binaries` matrix in this session:
four entries, `ubuntu-latest` ×2 and `macos-latest` ×2, no `windows-latest` anywhere. The guard is
provably dead code; removing it is a pure simplification with no behavior change.

## Code Examples

### Docker digest + tag-list job outputs (D-09)

```yaml
# In build-docker, after "Build and push":
      - name: Build and push
        id: build            # <-- add an id so its outputs are addressable
        uses: docker/build-push-action@v5
        with: # ...unchanged...

    outputs:
      digest: ${{ steps.build.outputs.digest }}
      tags_json: ${{ steps.meta.outputs.json }}
```
Source: `docker/build-push-action@v5`'s `action.yml` `outputs:` block (CITED — fetched this
session: `digest: description: 'Image digest'`). The exact wire format (whether it already carries
the `sha256:` prefix) is the field's well-established convention across the Docker/GHA ecosystem
but was not independently reproduced against a live push in this research session — **confirm
against the D-14 rehearsal's actual output** before hard-coding a `sha256:` prefix in the pull-line
composer; if the raw output already includes the prefix, prepending a second one produces
`sha256:sha256:...` and a broken pull command.

### SHA256SUMS aggregation via `gh release download` (D-11)

```bash
# In finalize-release-body, needs: [create-release, build-docker, build-binaries, sbom]
TAG="${{ needs.create-release.outputs.version }}"
mkdir -p "$RUNNER_TEMP/assets" && cd "$RUNNER_TEMP/assets"
gh release download "$TAG" --pattern '*.tar.gz'
sha256sum *.tar.gz > SHA256SUMS
gh release upload "$TAG" SHA256SUMS --clobber
```
`gh release download --pattern` VERIFIED via local `gh release download --help` this session.
Running on `ubuntu-latest` (the default for a job with no `runs-on: macos-*`) means `sha256sum` is
available natively here — no portability fallback needed for this specific aggregation step, unlike
the per-leg `.sha256` files inside `build-binaries` (Pitfall 1).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| `actions/create-release@v1` / `actions/upload-release-asset@v1` | `gh` CLI (`gh api`, `gh release upload`) | Both archived upstream since 2021; already partially migrated by Phase 20 (`create-release`) | `build-binaries`/`sbom` still use the archived upload action today — this phase finishes the migration these two jobs deferred |
| Body from `git log --pretty=format:"- %s"` | Body from curated `CHANGELOG.md` section | This phase (D-01) | Directly closes ARTIFACT-01 |
| `:latest` pull instruction | Pinned `@sha256:` digest pull instruction | This phase (D-09) | Directly closes ARTIFACT-03/04 |

**Deprecated/outdated:**
- `actions/create-release@v1`, `actions/upload-release-asset@v1` — archived upstream since 2021,
  no longer receiving updates; already documented as the target of removal by ARTIFACT-06 and
  Phase 20's own ground-truth notes.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `docker/build-push-action`'s `digest` output is already formatted with the `sha256:` prefix (so it can be concatenated directly as `image@${digest}`) | Code Examples, Pitfall 4 | If the raw output omits the prefix, the pull-line composer must add it; if it's already present, adding a second one breaks the command. Low blast radius (caught immediately by the D-14 rehearsal's own "image pulls by the digest the release names" acceptance check) but should be confirmed rather than assumed in the plan's task text. |
| A2 | The `cli,web-server` feature combination cross-compiles cleanly for `aarch64-unknown-linux-gnu` under `cross 0.2.5` with `vendored-openssl` | Aarch64 Cross-Compilation Risk Assessment | If wrong, D-05's own fallback (assert-list-per-target, drop the binary from that target's manifest) is the designed escape hatch — moderate risk, explicitly anticipated by the locked decision, not a plan-breaking surprise. |
| A3 | A release-profile (`opt-level=3, lto=true, codegen-units=1`) build with the same feature set succeeds given the debug-profile build succeeded | Standard Stack (Version verification) | Release-profile miscompilation from feature flags alone (as opposed to LTO/codegen settings) is rare; if wrong, it would surface immediately in `build-binaries`'s actual `cargo build --release` step during the rehearsal, not silently. |

**If this table is empty:** N/A — three items above need confirmation during plan execution /
the D-14 rehearsal, not before planning begins; none blocks planning.

## Open Questions

1. **Exact wire format of `docker/build-push-action`'s `digest` output**
   - What we know: it is named `digest`, described as "Image digest" in the action's own metadata
     (CITED, fetched this session).
   - What's unclear: whether it is pre-formatted as `sha256:<hex>` or bare `<hex>`.
   - Recommendation: the plan's task for D-09 should read the actual value with a debug `echo`
     step on the first CI run (or during the D-14 rehearsal) before hard-coding string
     concatenation logic, rather than assuming the format from this document.

2. **Whether the aarch64 leg ships all three binaries or a narrowed set**
   - What we know: no C-toolchain incompatibility identified by dependency analysis; the existing
     `vendored-openssl` cross-build path already works for the library target.
   - What's unclear: whether the binary-target code (not exercised under `cross` for aarch64 until
     now) compiles cleanly.
   - Recommendation: build the archive/assert step (D-06, Pattern 2) generically enough that
     narrowing the per-target binary list is a data change, not a structural one — this is already
     what D-05 asks for, and this research confirms it's the right posture given the genuine
     uncertainty.

3. **Whether `check-release-consistency.sh`'s Clause 2 regex should be factored into a shared
   library both scripts source, versus independently reimplemented with matching semantics**
   - What we know: the two must never disagree (Don't Hand-Roll).
   - What's unclear: whether the existing script exposes its regex in a sourceable form, or only
     embeds it inline in a heredoc-fed Python block (confirmed: it's inline in a `python3 -
     <<'PY'` heredoc, not factored out).
   - Recommendation: planner discretion, per CONTEXT.md — either duplicate the regex with a
     comment cross-referencing the other script's line (cheapest, matches this repo's existing
     convention of documented, not-DRY'd, parallel implementations — e.g.
     `finalize-crate-changelogs.sh` and `check-release-consistency.sh` already both independently
     enumerate "publishable packages" via the identical `cargo metadata` filter, documented as
     intentionally parallel so "the two scripts can never disagree about which files matter"), or
     extract a tiny shared `scripts/lib/changelog-heading.sh`. Given the codebase's established
     precedent for documented parallel implementation over premature shared libraries, duplication
     with a cross-reference comment is the lower-risk choice.

## Environment Availability

| Dependency | Required By | Available (this research sandbox) | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `gh` CLI | Extraction script testing, `create-or-reuse-release.sh`, all new `gh release`/`gh api` calls | ✓ | 2.97.0 | — |
| `cargo`/`rustc` | Native x86_64 binary-build verification | ✓ | 1.97.1 (matches `rust-toolchain.toml`) | — |
| Docker daemon | aarch64 cross-build verification, live digest-format confirmation | ✗ | — | None locally — this is exactly what the D-14 rehearsal on real GitHub Actions runners exists to prove; do not attempt to fully verify D-05/D-09's live behavior outside CI |
| `cross` (0.2.5) | aarch64 cross-build | ✗ (not installed; depends on Docker) | — | Same as above |
| `macos-latest`-equivalent runner (for `shasum` behavior) | Confirming Pitfall 1's exact failure/fix | ✗ (Linux devcontainer) | — | CITED from GitHub's own `runner-images` issue tracker instead of live-reproduced |

**Missing dependencies with no fallback:**
- Docker/`cross` for the aarch64 leg and the live Docker digest format — both require the actual
  GitHub Actions environment; D-14's rehearsal is the only place these get proven, not this
  research document and not local re-reading of the workflow (the phase's own honesty rule).

**Missing dependencies with fallback:**
- macOS `shasum` behavior — not reproducible in this Linux sandbox, but corroborated by multiple
  independent, dated GitHub issues rather than left as pure inference.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Custom bash fixture-harness (no third-party test framework — matches every existing `tests/scripts/*_test.sh`) |
| Config file | none — each `*_test.sh` is self-contained, sources the guard script under a `*_LIB_ONLY=1` env var to exercise its `_main` function without executing it |
| Quick run command | `./tests/scripts/<new-script>_test.sh` (direct invocation of the new test file only) |
| Full suite command | `make test-shell-guards` (loops over every `tests/scripts/*_test.sh`, fails loudly on an empty glob) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ARTIFACT-01 | Extraction returns exact section text; empty section (D-02) passes; missing section fails with the named error; version-string edge cases (`-rc.N`, dated suffix) handled | unit (bash fixture) | `./tests/scripts/extract-changelog-section_test.sh` | ❌ Wave 0 — new file |
| ARTIFACT-02 | `--features cli,web-server` produces all three binaries on x86_64 (already spot-checked live in this research session, debug profile) | integration (local `cargo build`) | `cargo build --release --features cli,web-server && ls target/release/{paladin,paladin-cli,paladin-server}` | ✅ — no new file, existing Cargo build; add this exact command as a Makefile/CI step so it's automated rather than ad hoc |
| ARTIFACT-02 (D-06) | Archive step fails loudly if any expected binary is missing | unit (bash fixture, or covered implicitly by CI once wired) | Depends on planner's choice of whether the assert logic lives in a testable script or inline `run:` — if inline, cover it with a CI dry-run against a scratch `target/` tree missing one binary | ❌ Wave 0 if factored into a script; N/A if kept inline (recommend factoring for testability, matching the repo's "logic in scripts" convention) |
| ARTIFACT-03/04 | Finalize-body composition is idempotent across re-runs (marker truncate-and-rebuild); references only real job outputs | unit (bash fixture with a scripted "already has marker" body fixture) | New `*_test.sh` for the finalize-body composer, following the `create-or-reuse-release_test.sh` pattern | ❌ Wave 0 — new file, if composition logic is factored into a script (recommended over inline YAML per this repo's convention) |
| ARTIFACT-05 | SHA256SUMS aggregation covers exactly what was uploaded; per-asset checksums use the OS-portable command | unit (bash fixture, `sha256_cmd()` helper tested independently of `gh`) | New `*_test.sh` or inline coverage within the archive-step test | ❌ Wave 0 — new file |
| ARTIFACT-06 | End-to-end: assets download, checksums verify, image pulls by digest, body matches CHANGELOG section | e2e (not automatable — real GitHub Actions run against real ghcr.io/crates.io) | manual: D-14 rehearsal, recorded in `21-ARTIFACT-EVIDENCE.md` | N/A — by design, per D-14/the phase's own honesty rule; no local substitute is acceptable evidence |

### Sampling Rate

- **Per task commit:** run the specific new `*_test.sh` for whatever script that task touched.
- **Per wave merge:** `make test-shell-guards` (full existing + new script-test suite) plus
  `make check-gates` (existing offline release-gate guards, unaffected by this phase but must stay
  green) plus a local `cargo build --release --features cli,web-server` to catch any feature-flag
  regression before it reaches CI.
- **Phase gate:** `make test-shell-guards` and `make check-gates` green, **plus the D-14 rehearsal
  on a real throwaway tag** — this phase's own success criterion 7 is explicit that "re-reading the
  workflow is not evidence."

### Wave 0 Gaps

- [ ] `tests/scripts/extract-changelog-section_test.sh` — covers ARTIFACT-01 (empty-section pass,
      missing-section fail, `-rc.N` boundary correctness, dated-suffix tolerance)
- [ ] A test file for the finalize-body composer, if factored into a standalone script (recommended)
      — covers ARTIFACT-03/04's idempotency requirement
- [ ] A test file (or fixture coverage folded into the above) for `sha256_cmd()`'s portability
      branch — covers ARTIFACT-05/Pitfall 1
- [ ] No framework install needed — the existing bash-fixture harness and `make test-shell-guards`
      loop already cover any new `tests/scripts/*_test.sh` file automatically (glob-based, no
      registration step)

## Security Domain

### Applicable ASVS Categories

This phase is CI/workflow engineering over already-public release artifacts (a GitHub release, a
public ghcr.io image, public crates.io packages) — most ASVS categories (authentication, session
management, access control over application data) do not apply. The relevant slice is input
handling of tainted values inside shell scripts and workflow YAML, which this repository already
governs via its own established `CR-01` convention.

| ASVS Category | Applies | Standard Control |
|---------------|---------|-------------------|
| V2 Authentication | No | N/A — `gh`/`GITHUB_TOKEN` auth is Phase 19/20's concern, unchanged here |
| V3 Session Management | No | N/A |
| V4 Access Control | No | N/A — release/package visibility is already public by design |
| V5 Input Validation | Yes | The existing `CR-01` pattern: every tainted value (tag input, commit-derived text, changelog content) reaches a `run:` body only via `env:` indirection, never inline-interpolated; every multi-line/untrusted body is passed to `gh`/`jq` via a file (`--body-file`, `--notes-file`) or `jq -n --arg`, never string-concatenated into a shell command. This phase's new extraction and finalize-body scripts must follow the identical discipline `create-or-reuse-release.sh` already establishes. |
| V6 Cryptography | No — this phase *reports* checksums/digests, it does not implement any cryptographic primitive | `sha256sum`/`shasum -a 256` and the registry-issued `sha256:` digest are both pre-existing, standard tools/mechanisms; nothing here hand-rolls hashing |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|----------------------|
| Commit-subject or changelog text injecting shell metacharacters into a `run:` block | Tampering | Already the concern `CR-01`, `create-or-reuse-release.sh`'s `jq -n --arg` payload construction, and the "write to `$RUNNER_TEMP` file, never interpolate" pattern for the changelog-generation step all exist to prevent — the new extraction script must preserve this, not reintroduce string interpolation of changelog content into a `run:` body |
| A credential-bearing HTTP request (e.g. `gh api` calls using `GITHUB_TOKEN`) following a redirect to an attacker-controlled host | Information Disclosure | Already governed by `security.instructions.md`'s "HTTP clients sending a credential header do not follow redirects" control; `check-release-consistency.sh`'s own comments already document "No `-L`/`--location` ... is ever passed to `curl` or `gh` here" — any new `gh api`/`curl` call this phase adds (e.g. for the digest/tag capture) must maintain the same no-redirect-follow discipline |
| A release body growing unboundedly across re-runs, eventually hitting the GitHub API's size cap and causing a hard, confusing `422` failure late in the pipeline | Denial of Service (of the release process itself) | Pattern 1 (marker-based truncate-and-rebuild) — see Pitfall 2/5 |

## Sources

### Primary (HIGH confidence — tool-verified this session)
- `gh release edit --help`, `gh release upload --help`, `gh release download --help` (local `gh`
  2.97.0) — exact flag semantics for the finalize-body and SHA256SUMS-aggregation mechanisms
- `cargo build --bins --features cli,web-server` (this repo, this session) — all three binaries
  produced natively on x86_64 under the toolchain `rust-toolchain.toml` pins
- Direct reads of `.github/workflows/release.yml`, `scripts/create-or-reuse-release.sh`,
  `scripts/check-release-consistency.sh`, `scripts/finalize-crate-changelogs.sh`, `Cargo.toml`,
  `CHANGELOG.md`, `Makefile`, `crates/paladin-web/Cargo.toml`, `tests/scripts/*_test.sh` — the
  existing code this phase edits/extends

### Secondary (MEDIUM confidence — CITED from official docs/action metadata)
- `docker/build-push-action@v5`'s `action.yml` (fetched this session) — `digest`/`imageid`/
  `metadata` output names and one-line descriptions
- `gh` CLI manual pages (`cli.github.com/manual`) — corroborating the local `--help` output
- GitHub Docs "About releases" and multiple `cli/cli`/`changesets/action` issues — the
  ~125,000-character release-body cap
- `actions/runner-images#90` and `taiki-e/upload-rust-binary-action#30` — `sha256sum` absence on
  `macos-latest` runners (Pitfall 1)

### Tertiary (LOW confidence — WebSearch summaries without a directly-fetched primary source)
- The exact wire format (`sha256:`-prefixed or not) of `docker/build-push-action`'s `digest`
  output — not independently confirmed against a live push in this session; flagged in Open
  Questions / Assumptions Log rather than asserted

## Metadata

**Confidence breakdown:**
- Changelog extraction mechanics: HIGH — directly derived from an existing, working, in-tree
  implementation (`check-release-consistency.sh` Clause 2) plus a direct read of the actual
  `CHANGELOG.md` edge case (`0.8.1-rc.4`'s empty section)
- Binary build matrix (x86_64): HIGH — live-verified in this session
- Binary build matrix (aarch64 cross): MEDIUM — dependency-graph analysis only, no live cross-build;
  explicitly flagged as an open item for D-14
- `gh` CLI mechanics: HIGH — live-verified via local `--help` output on the same `gh` version
  GitHub-hosted runners use
- Docker digest/metadata wire format: MEDIUM — action metadata confirms field existence, not exact
  format; flagged for rehearsal confirmation
- Checksum portability pitfall: HIGH — multiple independent, dated, corroborating sources plus a
  direct logical trace through the existing workflow explaining why it's never been observed
  before now

**Research date:** 2026-08-31
**Valid until:** 30 days for the `gh`/Docker-Actions mechanics (stable, slow-moving tooling); the
aarch64/digest-format open items are valid only until the D-14 rehearsal actually runs, at which
point this document's MEDIUM-confidence claims should be superseded by the rehearsal's measured
evidence in `21-ARTIFACT-EVIDENCE.md`.
