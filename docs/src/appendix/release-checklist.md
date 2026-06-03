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

Run dependency-first dry-runs:

1. paladin-core
2. paladin-ports
3. leaf crates
4. paladin

Use:

- cargo publish --dry-run -p <crate>

If upstream crates are not yet on crates.io, execute dry-runs in publish order and expect dependent dry-runs to fail until prerequisites are available.

## 6. Publish

Publish in dependency-first order:

1. paladin-core
2. paladin-ports
3. leaf crates
4. paladin

After each publish, verify crate availability on crates.io before continuing.

## 7. Tag and Announcement

- Create and push release tag.
- Publish release notes.
- Announce release in project communication channels.
- Confirm docs.rs build status for published crates.

## 8. Post-Release Verification

- Re-run quick smoke tests on published versions.
- Verify dependency resolution for a downstream sample app.
- Log follow-up items for next release cycle.
