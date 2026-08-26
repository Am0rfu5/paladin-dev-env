# Phase 19: crates.io Trusted Publishing - Pattern Map

**Mapped:** 2026-08-26
**Files analyzed:** 4 (this phase is CI/workflow + docs, no new Rust source files)
**Analogs found:** 4 / 4

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|--------------------|------|-----------|-----------------|----------------|
| `.github/workflows/release.yml` (`publish-crates` job) | CI workflow job (config) | request-response (OIDC token exchange) + batch (sequential per-crate publish loop) | `.github/workflows/docs.yml` (`deploy` job) | role-match (same repo, same OIDC/Environment mechanism; different job/trigger shape) |
| `docs/src/appendix/release-automation.md` | documentation (reference/appendix) | transform (rewrite existing prose + add table) | itself, prior revision (`:99-101` token section) + `SECURITY-EXCEPTIONS.md` table convention | exact (same file, rewritten section) / role-match (record convention borrowed from SECURITY-EXCEPTIONS.md) |
| `docs/src/appendix/release-checklist.md` | documentation (checklist) | transform | `release-automation.md` (sibling appendix doc, same doc style) | role-match |
| `CHANGELOG.md` | documentation (changelog entry) | append-only event log | existing `## [Unreleased]` entries (Qwen adapter entries) | exact (same file, same section convention) |

No new Rust source files, no new tests — this phase has no `cargo test` surface (confirmed in RESEARCH.md "Validation Architecture").

## Pattern Assignments

### `.github/workflows/release.yml` — `publish-crates` job (CI workflow job, OIDC request-response + batch publish)

**Analog:** `.github/workflows/docs.yml` `deploy` job (OIDC + GitHub Environment, proven working in this repo) — structural pattern only. **Direct edit target:** `release.yml` itself, lines 368-441 (current job body).

**Current state to replace** (`.github/workflows/release.yml:368-441`):
```yaml
  publish-crates:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    needs: [test, create-release]
    permissions:
      contents: read
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Determine publish mode
        id: mode
        env:
          # Repository secret with crates.io publish scope. Empty if unset.
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: |
          DRY_RUN="${{ github.event.inputs.dry_run }}"
          if [ "$DRY_RUN" = "true" ]; then
            echo "dry_run=true" >> "$GITHUB_OUTPUT"
            echo "::notice::Dry-run mode — will run 'cargo publish --dry-run' and NOT publish."
          elif [ -n "$CARGO_REGISTRY_TOKEN" ]; then
            echo "dry_run=false" >> "$GITHUB_OUTPUT"
          else
            echo "dry_run=skip" >> "$GITHUB_OUTPUT"
            echo "::warning::CARGO_REGISTRY_TOKEN is not set — skipping crates.io publish."
          fi

      - name: Publish crates in dependency order
        if: steps.mode.outputs.dry_run != 'skip'
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
          DRY_RUN: ${{ steps.mode.outputs.dry_run }}
        run: |
          set -euo pipefail
          CRATES=(
            paladin-ai-core
            paladin-ports
            paladin-battalion
            paladin-llm
            paladin-memory
            paladin-web
            paladin-notifications
            paladin-content
            paladin-storage
            paladin-ai
          )
          publish_one() {
            local crate="$1"
            echo "::group::Publishing ${crate}"
            if [ "$DRY_RUN" = "true" ]; then
              cargo publish --dry-run -p "$crate"
            else
              if cargo publish -p "$crate" 2>&1 | tee /tmp/publish.log; then
                echo "${crate} published."
              elif grep -qiE "already (exists|uploaded)|is already uploaded|already published" /tmp/publish.log; then
                echo "::warning::${crate} version already published — continuing."
              else
                echo "::error::Failed to publish ${crate}."
                exit 1
              fi
              sleep 20
            fi
            echo "::endgroup::"
          }
          for c in "${CRATES[@]}"; do
            publish_one "$c"
          done
```

**Job-level `permissions:` + `environment:` pattern to copy** (structure only — from `docs.yml:80-87`, adapted per D-06/D-07 job-level-not-workflow-level placement):
```yaml
# Source: .github/workflows/docs.yml:80-87 (deploy job) — environment + id-token mechanism proven
# in this repo. NOTE: docs.yml's id-token: write is at WORKFLOW level (lines 27-30). D-07 requires
# JOB level for publish-crates — do not copy that placement, only the environment/OIDC concept.
  deploy:
    name: Deploy to GitHub Pages
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

**Target shape for `publish-crates`** (per RESEARCH.md Pattern 1, D-06/D-07/D-09/D-10 applied):
```yaml
  publish-crates:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    needs: [test, create-release]
    environment: crates-io          # NEW — D-06, job-scoped GitHub Environment
    permissions:
      contents: read
      id-token: write               # NEW — D-07, job-scoped only, alongside existing contents: read
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Determine publish mode
        id: mode
        run: |
          # dry_run is exactly true|false from dispatch input; tag pushes are always false.
          # The old dry_run=skip branch (token-presence check) is DELETED, not rewritten — D-09.
          if [ "${{ github.event.inputs.dry_run }}" = "true" ]; then
            echo "dry_run=true" >> "$GITHUB_OUTPUT"
          else
            echo "dry_run=false" >> "$GITHUB_OUTPUT"
          fi

      - name: Authenticate with crates.io
        id: auth
        if: steps.mode.outputs.dry_run != 'true'   # D-10 — dry run skips the OIDC mint entirely
        uses: rust-lang/crates-io-auth-action@v1
        # No continue-on-error — a failed mint fails the job (D-09 honesty rule).

      - name: Publish crates in dependency order
        env:
          CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
          DRY_RUN: ${{ steps.mode.outputs.dry_run }}
        run: |
          set -euo pipefail
          # CRATES array gains paladin-herald, inserted after paladin-ai-core and before
          # paladin-ai (D-02) — it depends only on paladin-ai-core; only paladin-ai depends on it.
          CRATES=(
            paladin-ai-core
            paladin-herald
            paladin-ports
            paladin-battalion
            paladin-llm
            paladin-memory
            paladin-web
            paladin-notifications
            paladin-content
            paladin-storage
            paladin-ai
          )
          # ... publish_one() loop body unchanged from current file (already-published
          # tolerance is out of scope for this phase — Phase 20/PUBOPS-02 owns replacing the
          # regex-based already-published detection) ...
```

**Key deltas vs. current file, stated explicitly for the planner:**
1. Add `environment: crates-io` at job level.
2. Add `id-token: write` to the job's existing `permissions:` block (job-level, not workflow-level — do not touch top-of-file `permissions` if one exists elsewhere in this workflow).
3. Delete the `CARGO_REGISTRY_TOKEN`-presence check and the `dry_run=skip` branch/warning entirely (lines ~382-395 today) — replace with a plain `true|false` if/else.
4. Delete `if: steps.mode.outputs.dry_run != 'skip'` guard on the publish step (no longer a valid state).
5. Insert `rust-lang/crates-io-auth-action@v1` step (`id: auth`), gated `if: steps.mode.outputs.dry_run != 'true'`, with **no** `continue-on-error`.
6. Change the publish step's `CARGO_REGISTRY_TOKEN` source from `${{ secrets.CARGO_REGISTRY_TOKEN }}` to `${{ steps.auth.outputs.token }}`.
7. Insert `paladin-herald` into the `CRATES` array between `paladin-ai-core` and `paladin-ports` (or anywhere before `paladin-ai`, after `paladin-ai-core`).

---

### `docs/src/appendix/release-automation.md` (documentation — appendix reference)

**Analog:** itself, current "Required Secret" section (`:97-105`):
```markdown
### Required Secret

crates.io publishing requires a repository secret:

- `CARGO_REGISTRY_TOKEN` — a crates.io API token with publish scope.

If the secret is absent, the publish job is **skipped** (the rest of the release still runs), so the
pipeline can be exercised safely before the token is configured.
```
This entire section must be rewritten — it documents exactly the behavior D-09 deletes (silent skip on missing secret). Replace with: how Trusted Publishing works, the `crates-io` Environment, and the per-crate trust table (D-11).

**Table-with-governance-record convention to copy** (from `SECURITY-EXCEPTIONS.md`, the named-owner + date + revisit-condition shape referenced by D-12 as "convention only — not the register file itself"):
```toml
# Source: SECURITY-EXCEPTIONS.md:62-73 — shape to mirror in prose/table form, NOT as TOML
# (this doc is markdown, not a machine-checked register)
[[exception]]
id = "RUSTSEC-2023-0071"
...
owner = "DF3NDR"
review_date = "2026-12-31"
scope = "..."
compensating_control = "..."
revisit_condition = "..."
```
D-11's trust table (markdown table, not TOML) needs columns: crate name | source directory | workflow filename | environment name | link date | status (`linked` / `not covered — interim path: …`). D-12's "Credential history" subsection needs: event | date | actor (named owner) | evidence link — same named-owner-and-date discipline, prose/markdown table instead of TOML block.

**Existing divergent-name callout to preserve style from** — the doc already documents the workspace-root-vs-crate-name divergence pattern elsewhere in this file (root publishes as `paladin-ai`, `crates/paladin-core` as `paladin-ai-core` — confirmed present in `CRATES` array and Cargo.toml); the new trust table must repeat this divergence per-row rather than relying on a single footnote, per D-11 wording ("names diverge... impossible to miss").

---

### `docs/src/appendix/release-checklist.md` (documentation — checklist)

**Analog:** `release-automation.md` (sibling doc, same appendix directory/style). No token-specific excerpt found via grep in current checklist content (search returned no matches for "token"/"Token"/"CARGO_REGISTRY_TOKEN") — confirm during planning whether this file references the secret indirectly (e.g., "confirm secrets configured") before assuming line-level edits are needed. If a "Prerequisites" or "Secrets" section exists, mirror `release-automation.md`'s Required Secret → Trusted Publishing rewrite there too, per D-11 ("updated in the same change wherever it references the token").

---

### `CHANGELOG.md` (documentation — Unreleased entry)

**Analog:** existing `## [Unreleased]` → `### Changed` entries (`CHANGELOG.md:8-24`, the Qwen adapter entries):
```markdown
## [Unreleased]

### Changed

- **The Qwen (Alibaba DashScope) adapter's shipped default `base_url` is the Singapore
  (international) endpoint.** `QWEN_DEFAULT_BASE_URL` resolves to
  `https://dashscope-intl.aliyuncs.com/compatible-mode/v1`. This constant moved twice inside one
  week: ... See the "Region default" and "Reversal record" docs on `QWEN_DEFAULT_BASE_URL` in
  `crates/paladin-llm/src/qwen/adapter.rs` for the full endpoint table and why no single default
  here is ever the whole answer...
```
**Pattern to copy:** bold one-line summary as the entry's first sentence, followed by full context/rationale in the same bullet, with a pointer to the authoritative doc/source for full detail (here: `docs/src/appendix/release-automation.md`'s credential-history subsection) rather than duplicating detail inline. This phase's entry belongs under `### Security` or `### Changed` in `[Unreleased]` and should record the token revocation event per D-12 (date + link to the credential-history record), not restate the full ratchet.

---

## Shared Patterns

### GitHub OIDC + Environment (mechanism)
**Source:** `.github/workflows/docs.yml:27-30` (workflow-level `id-token: write` — placement NOT to copy) and `:80-87` (`environment:` block on the job — placement TO copy)
**Apply to:** `publish-crates` job only. This is the only OIDC precedent in the repo; it proves the mechanism works here but its permission-placement pattern must be adapted job-level per D-07.

### Named-owner + date governance record (convention, not literal reuse)
**Source:** `SECURITY-EXCEPTIONS.md` (owner/review_date/compensating_control/revisit_condition fields, TOML block style)
**Apply to:** `docs/src/appendix/release-automation.md`'s new "Credential history" subsection (D-12) and the trust table's per-row "link date"/status columns (D-11). Do **not** add rows to `SECURITY-EXCEPTIONS.md` itself — that file is scoped to RustSec advisories and mechanically checked by `scripts/check-advisory-register.sh`; a credential event there would break that script's contract.

### Changelog entry style
**Source:** `CHANGELOG.md:8-24` (`## [Unreleased]` → `### Changed`, bold-lead-sentence + pointer-to-source-of-truth pattern)
**Apply to:** the new PUB-04 revocation entry.

### Per-job `permissions:` blocks (existing repo-wide shape)
**Source:** `.github/workflows/release.yml` — every job in this file declares its own `permissions:` (e.g. `:161-162`, `:224`, `:332`, `:372-373`) rather than a single workflow-level block.
**Apply to:** `publish-crates` — preserve this shape; add `id-token: write` inside the job's own block, do not introduce a workflow-level `permissions:` for the whole file (D-07 explicit).

## No Analog Found

None — every file touched in this phase (workflow job, two appendix docs, changelog) has a direct in-repo analog, either itself (prior revision) or a structurally similar sibling file. The one genuinely new element — the `rust-lang/crates-io-auth-action@v1` step — has no in-repo analog by definition (first use of this action) but RESEARCH.md's Code Examples section already supplies the exact, citation-backed usage shape, so no further codebase search is needed.

## Metadata

**Analog search scope:** `.github/workflows/` (docs.yml, release.yml), `docs/src/appendix/` (release-automation.md, release-checklist.md), `SECURITY-EXCEPTIONS.md`, `CHANGELOG.md`
**Files scanned:** 6
**Pattern extraction date:** 2026-08-26
