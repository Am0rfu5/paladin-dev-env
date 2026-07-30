# PRD: Epic 5 — Publish and Finalize

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 5 — Publish and Finalize
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-06-03
**Author:** Paladin Framework Contributors

---

## 1. Introduction / Overview

This is the **terminal Epic** of Milestone 11. Epics 1–4 audited, restructured, rewrote, and
extended the documentation into a complete MDBook with compile-verified examples. Epic 5 finishes
the job: a final quality pass over the whole book, deployment to GitHub Pages, a rewritten root
`README.md` that serves as the public landing page, a `CHANGELOG.md` entry, a workspace version
bump to **0.5.0**, and the cut of the **v0.5.0** release.

The problem it solves: the documentation is finished but not yet *published* or *released*. The
root `README.md` is 1022 lines of stale, content-processing-era framing with an old crate list and
pre-workspace examples. The workspace is still at `0.4.3`. There is no published narrative site for
users to read, and no tagged `v0.5.0` release marking the milestone complete.

**The release is already automated** (Milestone 10). `make release VERSION=x.y.z` performs the
lockstep version bump, finalizes the changelog, commits, tags `vx.y.z`, and pushes; pushing the tag
triggers `.github/workflows/release.yml`, which verifies the tag is from `main`, runs the test
suite, **creates the GitHub Release, and publishes every crate to crates.io** in dependency order.
The canonical reference is [`docs/src/appendix/release-automation.md`](../../../docs/src/appendix/release-automation.md).
Epic 5's job is therefore to get the docs/README/CHANGELOG release-ready and then *drive* that
existing automation — not to re-implement it.

**Key decisions carried into this PRD** (resolved with the maintainer — see §9):

1. **End-to-end execution is authorized.** The implementer prepares all artifacts, merges to `main`,
   and runs `make release VERSION=0.5.0` from `main`. From there the existing GitHub Actions
   (`release.yml` + `docs.yml`) handle the tag verification, crates.io publish, GitHub Release, and
   Pages deploy automatically. The single human-driven irreversible action is `make release`, gated
   behind an all-green checkpoint (FR-13).
2. **Bump to 0.5.0 and synchronize all docs.** The Cargo.toml versions are bumped by `make release`
   (lockstep, via cargo-release — it also updates the internal `workspace.dependencies` pins). The
   *documentation* version strings (consumer-profile snippets, crate map, Getting Started) are not
   touched by cargo-release and must be updated by hand to `0.5.0` (FR-6). This supersedes Epic 4's
   interim OQ-3 decision to keep examples at 0.4.3.
3. **Badges: use everything that resolves — including crates.io and docs.rs.** The crates *are*
   published to crates.io via CI, so the version and docs.rs badges resolve. The README badge row is
   CI status, crates.io version, docs.rs, license, MSRV, plus a link to the Pages site.
4. **Concise landing-page README.** The 1022-line README is replaced with the concise structure from
   the Epic template; detailed material lives in the MDBook, which the README links to.

---

## 2. Goals

1. **Pass a complete final review** of the MDBook: clean build, resolving links, rendering
   diagrams/tables, accurate cross-references, content completeness.
2. **Publish the MDBook to GitHub Pages** at `https://df3ndr.github.io/paladin-dev-env/` via the
   existing `docs.yml` deploy job, and verify the live site.
3. **Bump the workspace to `0.5.0`** (via `make release`, lockstep) and reconcile every `0.4.3`
   reference in the documentation.
4. **Rewrite the root `README.md`** as a concise public landing page with a compile-verified quick
   example, resolving badges, and an accurate crate-ecosystem table.
5. **Prepare a complete `CHANGELOG.md` v0.5.0 entry** covering Milestones 8–11 (in `[Unreleased]`,
   which `make release` dates and promotes).
6. **Cut the v0.5.0 release** by driving the existing automation: `make release VERSION=0.5.0` →
   `release.yml` (tests, GitHub Release, crates.io publish) + `docs.yml` (Pages deploy).
7. **Point the `documentation` metadata** and repository "About" at the published Pages URL.

---

## 3. User Stories

- **As a prospective user landing on the GitHub repo**, I want a concise README with a working quick
  example and a link to full docs, so I can evaluate Paladin in two minutes.
- **As a developer**, I want a published, navigable documentation site with working search, rendered
  diagrams, and syntax-highlighted examples, so I can learn the framework without cloning it.
- **As an existing user**, I want a CHANGELOG entry, a tagged v0.5.0 release, and the crates on
  crates.io, so I know what changed and can depend on a stable published version.
- **As a maintainer**, I want the version bumped consistently across all crates and docs, so the
  released artifact has no `0.4.3`/`0.5.0` mismatches.
- **As a contributor**, I want the README to link to the contributing guide and live docs, so I can
  find the current process.

---

## 4. Functional Requirements

### Task 5.1 — Final MDBook Build and Review

#### FR-1: Clean build
`mdbook build docs/` must complete with **zero errors** and **zero broken links** (linkcheck
`warning-policy = "error"`). All HTML output must be present in `docs/book/`. *(Note: linkcheck's
non-fatal "fragment resolution isn't implemented" notices for `#anchor` links and the
mdbook-mermaid version notice are tool limitations, not content errors, and do not fail the build —
the accurate interpretation of "zero warnings" carried over from Epic 4.)*

#### FR-2: Link and cross-reference integrity
- All internal MDBook links resolve; no broken relative references in `SUMMARY.md`.
- Spot-check external URLs (README badges, Pages URL, repo links) return success.
- All "see also" links between chapters point to the correct sections.
- `crate-map.md` matches the actual workspace `Cargo.toml` member list and per-crate feature flags.

#### FR-3: Code-example verification
- The Epic 4 compile gate (`./scripts/check-doc-examples.sh`, which compiles `paladin-doc-examples`)
  passes with zero failures.
- The config gate (`./scripts/check-doc-config.sh`) passes with zero failures.
- *(Note: the original Epic text references `cargo test --doc`; that is **not** the project's path —
  most crates set `doctest = false`. Use the `paladin-doc-examples` + `{{#include}}` mechanism, which
  is the authoritative and stronger gate. See Technical Considerations.)*

#### FR-4: Content completeness and rendering
- Every chapter in `SUMMARY.md` has real content (no placeholder-only user-facing pages).
- `introduction.md` gives a clear overview and navigation.
- Getting Started lets a new user install, configure, and run a basic Paladin.
- Mermaid diagrams render in the built HTML; code blocks are syntax-highlighted; tables render; no
  raw HTML artifacts or malformed Markdown.

#### FR-5: Release-automation doc currency
Verify [`docs/src/appendix/release-automation.md`](../../../docs/src/appendix/release-automation.md)
accurately describes the current `make release` + `release.yml` flow (lockstep bump, main-only tag,
crates.io publish on tag). Update it only if it has drifted.

### Documentation Version Sync (precedes the release)

#### FR-6: Documentation version synchronization
Update every `0.4.3` (and `"0.4"`-style) version reference in `docs/src/**` to `0.5.0`, including:
the consumer-profile snippets and crate table in `api-reference/crate-map.md`, Getting Started
(`installation.md`, `quickstart.md`), and any other guide that names the version. **Do not** hand-
edit `Cargo.toml` versions — those are bumped by `make release` (FR-12). After editing, re-run the
doc-examples compile gate to confirm consistency.

#### FR-7: `documentation` metadata
Set the root `Cargo.toml` `documentation` field to `https://df3ndr.github.io/paladin-dev-env/`
(the narrative MDBook). *(docs.rs continues to host the generated API docs; the README links both.)*

### Task 5.3 — Root README.md Rewrite

#### FR-8: Concise landing-page README
Replace the current 1022-line `README.md` with a concise landing page containing, in order:
1. Title + one-line tagline.
2. **Badge row** — CI status (`ci.yml`), **crates.io version**, **docs.rs**, license (MIT), and MSRV
   (Rust ≥ 1.85, from `edition = "2024"`; no `rust-toolchain.toml`), plus a link/badge to the Pages
   docs site. (Crates are published to crates.io, so all badges resolve — OQ-2.)
3. One-paragraph description positioning Paladin as a **multi-agent AI orchestration framework**
   (not the old "content processing platform" framing).
4. **Quick Example** — a minimal end-to-end snippet that is **compile-verified** via the
   `paladin-doc-examples` gate (added as an anchored fn and `{{#include}}`-d, or otherwise routed
   through `check-doc-examples.sh`). No hand-written-only example.
5. **Key Features** — 5–8 headline bullets.
6. **Crate Ecosystem** — table of the 9 crates (name, purpose, key feature flags), consistent with
   `api-reference/crate-map.md`.
7. **Documentation** — link to the published MDBook (Pages) and to docs.rs.
8. **Getting Started** — prerequisites, add-to-`Cargo.toml`, minimal example or link to quickstart.
9. **Project Status** — current version (0.5.0), stability (link to `stable-api.md`), changelog link.
10. **Contributing** — short paragraph + links to `CONTRIBUTING.md` / `docs/src/contributing/`.
11. **License** — MIT statement.

#### FR-9: README quality
All badges resolve to real URLs; the crate table matches the actual workspace; all internal links
point to real files; the Quick Example compiles against the 0.5.0 workspace.

### Task 5.4 — CHANGELOG (prepared before the release cut)

#### FR-10: CHANGELOG `[Unreleased]` content
Populate the existing `## [Unreleased]` section of `CHANGELOG.md` with a complete summary of
Milestones 8–11 under `### Added`, `### Changed`, `### Fixed`, and `### Documentation`. The
Documentation subsection must call out: the MDBook published to GitHub Pages; the new orchestration,
content-processing, and bridge guides; the crate map & feature-flag reference; and that all examples
are compile-verified against the current API. **Do not** hand-date or add the `## [0.5.0]` heading —
`make release` moves `[Unreleased]` under `## [0.5.0] - <today>` automatically (FR-12).

### Task 5.2 + 5.4 — Release Cut and Publish (driving the existing automation)

#### FR-11: Go/no-go checkpoint before the release
Before running `make release`, confirm a single consolidated all-green checkpoint: doc-examples gate
green, config gate green, `mdbook build` clean, `cargo check --workspace` green, README and CHANGELOG
`[Unreleased]` ready, the branch merged to an up-to-date `main`. Only on all-green does the release
proceed.

#### FR-12: Cut the release with `make release`
From an up-to-date `main`, run `make release VERSION=0.5.0`. This (per the Makefile target):
- runs `release-check`;
- performs the **lockstep version bump** to `0.5.0` across all crates and updates the internal
  `workspace.dependencies` pins (via `cargo release version`);
- finalizes `CHANGELOG.md` (moves `## [Unreleased]` under `## [0.5.0] - <today>`);
- commits `chore(release): version 0.5.0`, tags `v0.5.0`, and pushes the commit and tag.

> Prerequisite: `cargo-release` installed; the target refuses to run off `main` or when behind
> `origin/main` (release-branch protection — see `docs/src/appendix/branch-protection.md`).

#### FR-13: Automated publish + deploy (verify outcomes)
Pushing the `v0.5.0` tag triggers `release.yml`, and the push to `main` triggers `docs.yml`. Verify
that the automation completes:
- `release.yml`: `verify-tag-source` (tag is in `main`) → test suite → **GitHub Release created** →
  **crates published to crates.io** in dependency order (also Docker images + binaries).
- `docs.yml`: MDBook built and **deployed to GitHub Pages**.

#### FR-14: Verify the published surfaces
- The MDBook is live at `https://df3ndr.github.io/paladin-dev-env/`: site loads, sidebar navigation
  works, all chapters reachable, code highlighting present, Mermaid diagrams render, search works.
- The crates appear on crates.io at `0.5.0`; docs.rs builds the API docs.
- The GitHub Release for `v0.5.0` exists with the changelog body.

#### FR-15: Publish the URL
Add the published Pages URL to the repository "About" section and confirm it is reflected in
`README.md` (FR-8) and the `Cargo.toml` `documentation` field (FR-7).

---

## 5. Non-Goals (Out of Scope)

- **Re-implementing release automation** — `make release` + `release.yml` already exist and are the
  authoritative path; Epic 5 *drives* them, it does not rebuild them.
- **Manual crate-by-crate `cargo publish`** — crates.io publishing is performed by `release.yml` on
  the pushed tag, in dependency order. Do not publish by hand.
- **New documentation content** — All content was written in Epics 3–4. Epic 5 only fixes issues
  found in final review (links, version strings, rendering) and writes the README/CHANGELOG.
- **Rewriting Epic 3 pages** — e.g. `battalion-patterns.md` still contains pre-compile-verification
  API names; bringing it under the doc-examples gate is a recommended follow-up, not part of Epic 5.
- **Source-code/behavioral changes** — Only version strings change (and those via `make release`);
  no API or logic changes.

---

## 6. Design Considerations

### Execution order (sequential — terminal Epic)
1. **FR-1–FR-5** final review on the feature branch (fix any issues found; verify release-automation doc).
2. **FR-6–FR-7** documentation version sync (markdown only) + `documentation` metadata field.
3. **FR-8–FR-9** README rewrite (with compile-verified Quick Example).
4. **FR-10** CHANGELOG `[Unreleased]` content.
5. Merge the branch to `main` via PR; ensure local `main` is up to date.
6. **FR-11** go/no-go checkpoint (all gates green).
7. **FR-12** `make release VERSION=0.5.0` from `main`.
8. **FR-13–FR-15** verify the automated publish/deploy outcomes and publish the URL.

### README Quick Example
Keep the README example honest (the recurring lesson from Epic 4): back it with the compile gate —
add it as an anchored fn in `crates/doc-examples/` and `{{#include}}` it, or otherwise route it
through `check-doc-examples.sh`. No hand-written-only example.

### Badges
Use shields.io with real targets: GitHub Actions badge for `ci.yml`; crates.io version badge for the
`paladin-ai` umbrella crate; docs.rs badge; static MIT license badge; static MSRV badge
(`Rust 1.85+`); and a docs link/badge to the Pages URL.

---

## 7. Technical Considerations

### Release flow (authoritative)
- `release.toml`: lockstep `shared-version = true`, `tag-name = "v{{version}}"`, local
  `publish = false` / `push = false` (publishing and pushing are delegated to CI / the Makefile).
- `make release VERSION=x.y.z`: validates semver; requires `cargo-release`; **enforces `main` and
  up-to-date-with-origin** (overridable only via `RELEASE_ALLOW_ANY_BRANCH=1` for hotfixes); bumps
  lockstep; finalizes changelog; commits/tags/pushes.
- `release.yml` (on `v*.*.*` tag push): `verify-tag-source` (tag commit must be an ancestor of
  `main`) → test suite → `create-release` (GitHub Release) → crates.io publish (dependency order) →
  Docker images + release binaries.
- `docs.yml` (on push to `main`, paths `docs/**` / `crates/doc-examples/**`): builds the MDBook and
  deploys to GitHub Pages (`actions/deploy-pages@v4`, `github-pages` environment — Pages source
  already configured to "GitHub Actions").

### Version surface
`make release` handles all `Cargo.toml` versions (root + 9 members + `doc-examples`) and the internal
`workspace.dependencies` pins. Only the **documentation** strings (markdown consumer profiles, crate
map, Getting Started) are outside cargo-release's reach and are updated manually in FR-6.

### Doc-test mechanism (FR-3)
Most crates set `doctest = false`, so `cargo test --doc` is **not** the verification path. The
authoritative mechanism is the `paladin-doc-examples` crate compiled by
`scripts/check-doc-examples.sh` (CI + pre-push).

### Key facts
- License **MIT**; edition **2024** → MSRV **Rust ≥ 1.85**; no `rust-toolchain.toml`.
- Repo `github.com/DF3NDR/paladin-dev-env`; Pages `https://df3ndr.github.io/paladin-dev-env/`.
- Crates (published names): `paladin-ai` (umbrella, lib `paladin`), `paladin-ai-core`,
  `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`,
  `paladin-content`, `paladin-notifications`, `paladin-web`.
- Relevant docs: `appendix/release-automation.md`, `appendix/branch-protection.md`,
  `appendix/release-checklist.md`.

---

## 8. Success Metrics

| Metric | Target |
|---|---|
| `mdbook build` errors / broken links | 0 / 0 |
| doc-examples compile gate failures | 0 |
| config (YAML) gate failures | 0 |
| `0.4.3` references remaining in `docs/src` after sync | 0 |
| README length | concise landing page (≈100–200 lines, down from 1022) |
| README Quick Example compiles | Yes (via doc-examples gate) |
| Broken README badges/links | 0 |
| `make release VERSION=0.5.0` succeeds | Yes |
| Workspace version after release | 0.5.0 (all `Cargo.toml`) |
| `release.yml` outcome | GitHub Release created + crates published to crates.io |
| `docs.yml` outcome | Site live at the Pages URL |
| CHANGELOG `## [0.5.0]` section (dated by `make release`) | Present |

---

## 9. Open Questions

| ID | Question | Resolution |
|---|---|---|
| OQ-1 | Is the repo's Pages source set to "GitHub Actions"? | **Yes.** The `docs.yml` deploy job is already working. (Maintainer.) |
| OQ-2 | Are `paladin-*` crates published to crates.io / docs.rs? | **Yes** — handled by `release.yml` on tag push (set up in a prior milestone). So crates.io/docs.rs badges resolve and are included (FR-8). The flow is documented in `appendix/release-automation.md`. (Maintainer.) |
| OQ-3 | Are all of these decisions green-lit, incl. end-to-end execution? | **Yes** — the maintainer green-lights the decisions and authorizes driving the release end-to-end via `make release` + CI, gated by FR-11. (Maintainer.) |
| OQ-4 | Version handling vs. Epic 4's 0.4.3 examples? | **Bump to 0.5.0 and sync all docs** (FR-6, FR-12). Supersedes Epic 4 OQ-3. (Maintainer.) |
| OQ-5 | README rewrite extent? | **Concise landing page** with MDBook links; details live in the book. (Maintainer.) |
| OQ-6 | Hand-roll the tag/release, or use the automation? | **Use the automation:** `make release VERSION=0.5.0` (lockstep bump + tag + push) → `release.yml` (GitHub Release + crates.io). No hand-rolled `git tag`/`gh release`. (Maintainer.) |
| OQ-7 | CHANGELOG release date? | **Today's date** (2026-06-03), applied automatically by `make release`. (Maintainer.) |

---

## Relevant Files

### Files Rewritten
- `README.md` — concise landing page (FR-8, FR-9)

### Files Updated (In-Place, by hand)
- `Cargo.toml` — `documentation` field → Pages URL (FR-7). *(Version is bumped by `make release`, not by hand.)*
- `crates/doc-examples/src/*` — add the README Quick Example anchor (FR-8)
- `docs/src/api-reference/crate-map.md`, `docs/src/getting-started/installation.md`, `quickstart.md`, and any other guide naming the version — `0.4.3` → `0.5.0` (FR-6)
- `CHANGELOG.md` — populate `[Unreleased]` with the v0.5.0 summary (FR-10)
- `docs/src/appendix/release-automation.md` — only if it has drifted (FR-5)

### Driven by automation (not hand-edited)
- All `Cargo.toml` versions + `workspace.dependencies` pins — bumped by `make release` (FR-12)
- `CHANGELOG.md` `## [0.5.0] - <date>` heading — finalized by `make release` (FR-12)
- `v0.5.0` tag, GitHub Release, crates.io publish — `make release` + `release.yml` (FR-12, FR-13)
- GitHub Pages deploy — `docs.yml` (FR-13)

### Repository settings (configured/verified, not files)
- "About" → documentation URL (FR-15); Pages source = "GitHub Actions" (already set, OQ-1)
