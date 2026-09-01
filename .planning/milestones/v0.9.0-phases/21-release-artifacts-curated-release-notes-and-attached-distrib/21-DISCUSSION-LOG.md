# Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-31
**Phase:** 21-release-artifacts-curated-release-notes-and-attached-distrib
**Mode:** `--auto` — all gray areas auto-selected; recommended option taken on each question without user prompts. Every selection is logged here so a human can audit and overturn any single one.
**Areas discussed:** Release body composition & assembly order, Binary set & feature matrix, Asset upload mechanism & upload_url removal, Docker digest binding / latest / size check, Checksums / SBOM / signing, End-to-end rehearsal design

---

## Release body composition and assembly order

| Option | Description | Selected |
|--------|-------------|----------|
| Extract root section; missing section fails; artifact sections appended post-build by a finalize job from real outputs | Curated notes are the body; a terminal job appends digest/asset/verify sections only for legs that succeeded | ✓ |
| Extract root section into a fully static body at create-release time | Simpler, but cannot satisfy ARTIFACT-03 — body would advertise artifacts not yet (or never) produced | |
| Inline per-crate changelog sections too | Eleven fragments bury the notes; crate changelogs already ship in the crates | |

**Auto-selection:** finalize-job composition (D-01…D-04). Empty (heading-only) sections allowed — rc.4 is one today and rehearsal rc's will be too.
**Notes:** `[auto] Q: "Fallback when section missing?" → hard failure, no git-log fallback (ARTIFACT-01 binding text forbids it).`

## Binary set and feature matrix

| Option | Description | Selected |
|--------|-------------|----------|
| Ship all three declared binaries with explicit `--features cli,web-server` (defaults on top) | One build per target; archive asserts all expected executables exist | ✓ |
| Ship only `paladin` with `--features cli` | Smaller, but leaves two declared binaries unshipped without a recorded reason | |
| Separate archives per binary | More assets and checksum rows for no consumer benefit | |

**Auto-selection:** D-05/D-06, with researcher verification that `cli,web-server` cross-builds under `cross` for aarch64; per-target assert lists make any drop explicit.

## Asset upload mechanism and upload_url removal

| Option | Description | Selected |
|--------|-------------|----------|
| `gh release upload --clobber`; delete `upload_url` plumbing | Matches Phase 20 D-01 posture (no new marketplace actions); idempotent re-runs | ✓ |
| `softprops/action-gh-release` or similar | New third-party action surface in `contents: write` jobs | |

**Auto-selection:** D-07/D-08; dead Windows strip guard removed in the same pass.

## Docker digest binding, latest instruction, size check

| Option | Description | Selected |
|--------|-------------|----------|
| Bind by build-push digest output; delete `:latest` pull line; size stated in body as advisory | Honest reporting without a new unvalidated red gate | ✓ |
| Push `latest` on tags so the instruction becomes true | Changes tagging semantics beyond this phase's scope | |
| Make 500 MB a hard failure now | Baseline unmeasured; could block the first real release through the pipeline | |

**Auto-selection:** D-09/D-10. Hard-fail promotion recorded as a deferred idea pending the rehearsal's measured size.

## Checksums, SBOM labeling, signing

| Option | Description | Selected |
|--------|-------------|----------|
| Aggregated SHA256SUMS + per-asset files kept; one-command verify in body; SBOM labeled root-package; signing deferred with recorded reasoning | Criterion 5 permits deferral if examined and recorded | ✓ |
| Adopt cosign/attestations now | New action surface + identity management mid-cleanup phase, no demanding consumer | |

**Auto-selection:** D-11/D-12/D-13. Signing reasoning lands in `docs/src/appendix/release-automation.md`, not only planning files.

## End-to-end rehearsal design

| Option | Description | Selected |
|--------|-------------|----------|
| Real throwaway rc tag; evidence in `21-ARTIFACT-EVIDENCE.md`; unrun path recorded unverified | The Phase 18/19/20 honesty rule; re-reading the workflow is not evidence | ✓ |
| Reason from workflow text alone | Explicitly rejected by ARTIFACT-06 — this is how the binaries defect survived | |

**Auto-selection:** D-14/D-15. One-way rating recorded: the rehearsal tag publishes that rc to crates.io permanently.

## Claude's Discretion

- Extraction script placement (extend `create-or-reuse-release.sh` vs new script); exact script/job names
- Finalize mechanism (`gh release edit` vs API PATCH) and body-section layout
- SHA256SUMS aggregation mechanism
- Rehearsal rc version string and sequencing
- Whether the body links to per-crate changelogs

## Deferred Ideas

- Hard-fail image-size threshold (after rehearsal measures a baseline)
- Artifact signing / build provenance (`actions/attest-build-provenance` named as candidate)
- Windows or additional build targets
- Per-crate changelog content in release notes
- The real stable catch-up release (operator act)

## Todo Cross-Reference

- 1 match at score 0.6 ("Verify local make coverage reproduces CI's 82.39%") — **not folded**, deviating from the auto-mode ≥0.4 rule, carrying forward the Phase 19/20 determination (orthogonal scope, human-owned, forbids silent closure by a phase).
