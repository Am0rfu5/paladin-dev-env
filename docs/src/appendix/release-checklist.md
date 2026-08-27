# Release Checklist

This checklist defines the required release path from code freeze through publish and announcement.

> **Automation:** Most of this checklist is automated by `make release VERSION=x.y.z` and the
> tag-triggered `.github/workflows/release.yml` pipeline. See
> [RELEASE_AUTOMATION.md](release-automation.md) for the tooling decision (`cargo-release`) and the
> operator guide. This checklist remains the authoritative description of the end-to-end process and
> the manual verification steps.

## 1. Code Freeze

- Confirm release branch and freeze window.
- Stop non-release feature merges.
- Confirm open blockers are triaged.

## 2. Changelog Finalization

- Ensure root changelog and per-crate changelogs are updated.
- Ensure notable breaking changes are explicitly called out.
- Verify release notes map to merged changes.

## 3. Version Bump

- Apply lockstep version update across public crates.
- Verify crate dependency versions remain aligned.
- Re-check Cargo.toml metadata completeness.

## 4. CI and Local Validation

Run and require success for:

- cargo test --workspace
- cargo fmt --all -- --check
- cargo clippy --workspace -- -D warnings
- cargo doc --workspace --no-deps
- cargo audit

## 5. Dry-Run Publish Validation

Run dependency-first dry-runs, in the real eleven-crate order:

1. paladin-ai-core
2. paladin-ports
3. paladin-herald
4. paladin-battalion, paladin-llm, paladin-memory, paladin-web, paladin-notifications,
   paladin-content, paladin-storage (leaf tier)
5. paladin-ai

Use:

- cargo publish --dry-run -p <crate>

If upstream crates are not yet on crates.io, execute dry-runs in publish order and expect dependent dry-runs to fail until prerequisites are available. This caveat is strictly more accurate now that paladin-herald is included: it depends on both paladin-ai-core and paladin-ports being present before its own dry-run can pass.

## 6. Publish

Publish in dependency-first order:

1. paladin-ai-core
2. paladin-ports
3. paladin-herald
4. paladin-battalion, paladin-llm, paladin-memory, paladin-web, paladin-notifications,
   paladin-content, paladin-storage (leaf tier)
5. paladin-ai

After each publish, verify crate availability on crates.io before continuing.

Publishing authenticates through crates.io Trusted Publishing, from the `publish-crates` job under
the `crates-io` GitHub Environment — there is no token to configure. See the per-crate trust table
and credential history in [Release Automation](release-automation.md#trusted-publishing).

## 7. Tag and Announcement

- Create and push release tag.
- Publish release notes.
- Announce release in project communication channels.
- Confirm docs.rs build status for published crates.

## 8. Post-Release Verification

- Re-run quick smoke tests on published versions.
- Verify dependency resolution for a downstream sample app.
- Log follow-up items for next release cycle.
