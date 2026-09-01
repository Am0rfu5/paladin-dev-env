# Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token - Context

**Gathered:** 2026-08-26
**Status:** Ready for planning

<domain>
## Phase Boundary

The ability to publish `paladin-*` crates to crates.io stops living in the long-lived
`CARGO_REGISTRY_TOKEN` repository secret (read at `.github/workflows/release.yml:385` and `:401`)
and moves to crates.io Trusted Publishing: a token minted per run from a GitHub OIDC identity via
`rust-lang/crates-io-auth-action`, expiring in ~30 minutes. The phase is finished when the old
secret **no longer exists** — revoked at crates.io and deleted from repository secrets, both
recorded — not when the OIDC path merely works. A repository holding both credentials has widened
its attack surface, not reduced it.

**In scope:** crate-set enumeration and reconciliation (PUB-01, including the `paladin-herald`
gap); the `publish-crates` job's authentication rewrite and GitHub Environment (PUB-02); the
proof-before-revoke event (PUB-03); revocation + secret deletion with recorded date/actor
(PUB-04); removal of the silent `dry_run=skip` green-exit and the per-crate trust-configuration
table (PUB-05).

**Out of scope:** idempotent re-runs, the pre-publish verification gate, the `create-release@v1`
replacement, index-wait/regex-detection fixes, and the yank runbook (all Phase 20, `PUBOPS-*`);
release notes and attached distributables (Phase 21, `ARTIFACT-*`); any change to `release.yml`'s
triggers (none is needed — it is already one of the two documented exceptions in
`scripts/check-workflow-triggers.sh`).

Requirements: `PUB-01`, `PUB-02`, `PUB-03`, `PUB-04`, `PUB-05`.

</domain>

<decisions>
## Implementation Decisions

*All decisions below were auto-selected under `--auto` (recommended option taken on each). Each
carries the reasoning that produced it so a human can overturn any single one without re-running
the discussion.*

### Crate set and the paladin-herald gap (PUB-01)

- **D-01:** The publishable set is enumerated from `Cargo.toml` manifests, and it is **eleven**
  crates: the ten in `release.yml`'s `CRATES` array plus `paladin-herald`.
  `crates/doc-examples` (`publish = false`) and `fixtures/codeql-probe` (workspace-excluded) are
  the only non-publishable members. **Verified live against crates.io on 2026-08-26:** all ten
  `CRATES` entries exist on the registry at `max_version 0.5.1`; `paladin-herald` returns 404 —
  it has never been published. Note the corollary: the tree is at 0.8.0 and tags v0.7.0/v0.7.1
  exist, so those releases were tagged but never reached crates.io — the last real publish was
  0.5.1 (2026-06-04).

- **D-02:** **Close the herald gap in this phase** rather than record-and-defer: add
  `paladin-herald` to the publish order (in dependency order, before `paladin-ai`, which depends
  on it as `version = "0.8.0", path = ...`). Rationale: a real `cargo publish -p paladin-ai` is
  broken without it, PUB-01 forbids carrying the gap forward unnoticed, and deferring merely
  hands the same broken order to Phase 20's pre-publish gate.
  — **Reversibility:** reversible until the first real publish lands; **one-way** afterward — a
  published crate version can never be deleted, so once `paladin-herald` exists on crates.io the
  decision is permanent (yank hides, it does not remove).

- **D-03:** `paladin-herald`'s first publish happens **during the proof event (PUB-03), while the
  old token is still valid** — the proof-before-revoke ordering makes this window legitimate — OR
  via the OIDC path directly if crates.io now supports Trusted Publishing for not-yet-existing
  crates (**researcher must verify current new-crate support**; historically a trust link could
  only be configured on an existing crate). Until `paladin-herald` exists on crates.io and its
  trust link is created, the trust table names it explicitly as *not covered* with its interim
  auth path stated (criterion 7); it is never described as covered.

### Proof event design (PUB-03)

- **D-04:** The proof is a **real prerelease publish through `release.yml`'s actual publish
  path** — e.g. a `v*.*.*-rc.1`-style prerelease tag or `workflow_dispatch` with `dry_run=false`
  — evidenced by the run URL plus the version visible on crates.io. Not a dry run (`--dry-run`
  needs no credential, so it proves nothing about the OIDC exchange), and not an out-of-band
  manual `cargo publish` from a laptop (that would prove a human's token, not the workflow's
  identity). The three-minor-version registry backlog (0.5.1 → 0.8.0) is **not** cleared as the
  proof vehicle: a full catch-up release drags in the half-published-recovery and pre-publish-gate
  risks that Phase 20 exists to fix, and this phase must not depend on machinery that doesn't
  exist yet. A prerelease minimizes blast radius because prerelease versions never win default
  dependency resolution.

- **D-05:** Ordering is a ratchet, each step evidenced before the next: (1) trust links created
  on crates.io for the existing ten crates; (2) proof publish succeeds via OIDC (run URL +
  crates.io listing recorded); (3) token revoked at crates.io; (4) `CARGO_REGISTRY_TOKEN` deleted
  from repository secrets. Steps 3 and 4 never precede step 2. Revoking first and debugging the
  new path under pressure is the failure mode PUB-03 exists to prevent.
  — **Reversibility:** one-way at step 3 — a revoked crates.io token cannot be un-revoked; the
  fallback until step 2 completes is that the old path still works.

### Environment and permission shape (PUB-02)

- **D-06:** One GitHub Environment named **`crates-io`**, attached to the `publish-crates` job.
  The environment name is **pinned in every crate's crates.io trust configuration** (the
  environment field is optional there, but leaving it empty lets any branch/workflow with
  `id-token: write` mint a publish token — pinning it is the protection).
  — **Reversibility:** costly — the name is recorded in eleven per-crate trust configs on
  crates.io plus the trust table doc; renaming means editing all of them in step.

- **D-07:** `id-token: write` is granted **on the `publish-crates` job only**, added alongside its
  existing `contents: read`. `release.yml` declares `permissions:` per job today and that shape is
  preserved — no workflow-level block is introduced. Note: `docs.yml`'s grant is actually at
  *workflow* level (line 28-30); it proves the OIDC mechanism works in this repository, but its
  permission placement is **not** copied — PUB-02's text explicitly requires the job-level grant.

- **D-08:** Environment protection: deployment branch/tag policy restricted to `v*.*.*` tags (the
  only refs `release.yml` publishes from); **no required-reviewer gate initially**, so tag-push
  releases stay unattended. Tightening to require approval later is a repository-settings change
  plus a one-line doc update. The protection posture is recorded in the trust table doc so it is
  auditable.
  — **Reversibility:** reversible — both directions are settings changes.

### Failure-honesty rewrite of the skip branch (PUB-05)

- **D-09:** The token-presence check and the `dry_run=skip` branch (`release.yml:391-395`) are
  **deleted, not rewritten**: mode becomes exactly `dry_run=true|false` from the dispatch input
  (tag pushes are always `false`). In real mode the `rust-lang/crates-io-auth-action` step runs
  unconditionally and its failure **fails the job** — there is no secret whose absence could be
  "skipped over", and no `continue-on-error` anywhere on the publish path. This mirrors Phase 18's
  D-06 honesty posture: a green job must mean exactly what it says. A release that publishes
  nothing must not end green (the Phase 12 defect class).

- **D-10:** Dry-run mode **skips the OIDC mint entirely** — `cargo publish --dry-run` needs no
  credential, skipping the mint keeps dry runs working on forks and before trust links exist, and
  it preserves the honest claim boundary: a green dry run asserts packaging validity only, never
  anything about authentication. The docs state this boundary explicitly.

### Documentation and recording (PUB-04, PUB-05)

- **D-11:** The per-crate trust table lives in **`docs/src/appendix/release-automation.md`** —
  the doc that currently instructs operators to configure `CARGO_REGISTRY_TOKEN` (lines 99-101)
  and must be rewritten by this phase anyway, and the first place the next operator will look.
  Columns: crate name, source directory (names diverge: `crates/paladin-core` →
  `paladin-ai-core`, workspace root → `paladin-ai`), workflow filename, environment name, link
  date, status (`linked` / `not covered — interim path: …`). `docs/src/appendix/release-checklist.md`
  is updated in the same change wherever it references the token.

- **D-12:** The revocation record (PUB-04) follows the **Phase 9/12 convention — a named owner, a
  date, an actor — but not the register file**: `SECURITY-EXCEPTIONS.md` is scoped to RustSec
  advisory suppressions and is mechanically checked by `scripts/check-advisory-register.sh`, so
  shoehorning a credential event into it would break that script's contract. The revocation entry
  goes in a "Credential history" subsection of `release-automation.md` beside the trust table,
  plus a `CHANGELOG.md` entry.

- **D-13:** **Human-in-the-loop steps are explicit plan checkpoints.** Creating trust
  configurations and revoking the token happen in the crates.io UI under the crate-owner account
  (the user); no CI job or agent can perform them. GitHub Environment creation and repository
  secret deletion can be automated (`gh api`) with confirmation. Plans must sequence these as
  checkpoint/human-action tasks with exact instructions, not assume automation.

### Claude's Discretion

- Exact prerelease version string and whether the proof runs via prerelease tag or
  `workflow_dispatch` — pick whatever exercises the real publish path with least ceremony.
- Whether trust links for all ten existing crates are created before the proof or a single
  pilot crate is linked and proven first, then the rest — either satisfies D-05's ratchet.
- Wording and placement details inside `release-automation.md`, provided the table columns and
  credential-history record match D-11/D-12.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements and phase definition
- `.planning/ROADMAP.md` §Phase 19 — goal, seven success criteria, dependency notes
- `.planning/REQUIREMENTS.md` §Publish credential (PUB) — PUB-01 through PUB-05 binding text

### The workflow being changed and the proven pattern
- `.github/workflows/release.yml` — current token path (`:385`, `:401`), `dry_run` mode selection
  (`:387-395`), `CRATES` array (`:407-418`, ten entries), `publish-crates` job permissions
  (`:368-373`), tag/`workflow_dispatch` triggers
- `.github/workflows/docs.yml` — the in-repo OIDC precedent: `id-token: write` (line 30,
  workflow-level — do not copy the placement, per D-07) + `environment: github-pages` on the
  deploy job (`:85-87`)

### Constraints on workflow changes
- `scripts/check-workflow-triggers.sh` — `release.yml` is in `EXCEPTION_FILES` (line 118); no
  trigger change is planned, but any accidental trigger edit is gated by this script
- `docs/src/contributing/branching-model.md` — trigger-policy table row for `release.yml`
  (line 53); must stay literally consistent with the YAML if anything about triggers changes

### Documentation to update
- `docs/src/appendix/release-automation.md` — documents `CARGO_REGISTRY_TOKEN` (`:99-101`);
  receives the trust table and credential-history record (D-11, D-12)
- `docs/src/appendix/release-checklist.md` — release path checklist; token references updated in
  the same change

### Recording convention
- `SECURITY-EXCEPTIONS.md` — the Phase 9/12 owner+date governance convention the revocation
  record follows (convention only — the entry does NOT go in this file, per D-12)

### Crate manifests (the PUB-01 enumeration source)
- `Cargo.toml` (workspace root) — `members = [".", "crates/*"]`, root publishes as `paladin-ai`
  v0.8.0, depends on `paladin-herald = { version = "0.8.0", path = ... }` (line 29)
- `crates/paladin-herald/Cargo.toml` — no `publish = false`; the unpublished eleventh crate
- `crates/doc-examples/Cargo.toml` — `publish = false`; the one legitimate exclusion

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `docs.yml` deploy job: working GitHub OIDC + named-environment pattern in this repository —
  the mechanism is proven here, only the permission placement differs (D-07).
- `release.yml` `verify-tag-source` job: already enforces that release tags descend from `main`;
  composes with the environment tag-restriction in D-08 rather than being replaced.
- `gh api` is available in this devcontainer for environment creation and secret deletion
  (see memory: GH_TOKEN reaches non-interactive shells; a read-only token would fail these
  writes — verify scope before relying on it).

### Established Patterns
- Per-job `permissions:` blocks throughout `release.yml` — the least-privilege shape PUB-02
  requires preserved.
- Phase 12's "no job reports success while measuring nothing" and Phase 18 D-06's visible
  non-blocking posture — the honesty standard D-09/D-10 apply to the publish path.
- Phase 9/12 owner+date recording convention for security-relevant events.

### Integration Points
- `release.yml` `publish-crates` job (`:368` onward) — auth rewrite, environment attachment,
  `CRATES` array extension, skip-branch deletion all land here.
- crates.io per-crate settings (external, human-operated) — trust configuration and token
  revocation.
- Repository settings (automatable via `gh api`) — `crates-io` environment creation with tag
  policy; `CARGO_REGISTRY_TOKEN` secret deletion.

### Live registry facts (verified 2026-08-26 via crates.io API)
- All ten `CRATES`-listed crates exist on crates.io; `max_version` is **0.5.1** for every one
  (last publish 2026-06-04). `paladin-herald` does not exist (404). Tags v0.7.0/v0.7.1 were never
  published to the registry. The workspace tree is at 0.8.0.

</code_context>

<specifics>
## Specific Ideas

- The trust table must make the crate-name/directory divergence impossible to miss:
  `crates/paladin-core` publishes as `paladin-ai-core`, the workspace root as `paladin-ai`.
- PUB-03 evidence format: run URL, the minted-token publish visible in the run log, and the
  version listed on crates.io — the same "measured, not assumed" style Phase 18's evidence file
  used (`.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` is the
  reference shape, not a required template).

</specifics>

<deferred>
## Deferred Ideas

- **Full registry catch-up release (0.5.1 → current)** — publishing the backlog for real belongs
  after Phase 20's idempotent re-runs and pre-publish gate exist; this phase proves the credential
  path with a prerelease only.
- **Environment required-reviewer gate** — deliberately not enabled now (D-08); revisit once
  Phase 20's runbook defines who approves releases.
- **`create-release@v1` replacement, index-wait fix, yank policy** — Phase 20 (`PUBOPS-*`).

### Reviewed Todos (not folded)
- **"Verify local make coverage reproduces CI's 82.39% figure"**
  (`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`) — matched at score
  0.9 on keyword overlap only (docs/testing/github/workflows/yml). Not folded despite the
  auto-mode ≥0.4 rule: the todo concerns coverage-measurement reproduction, orthogonal to the
  publish credential; it is explicitly owned by the repo maintainer, requires a human on a
  Docker-capable machine, and its own text says it must not be silently closed by a phase.

</deferred>

---

*Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry*
*Context gathered: 2026-08-26*
