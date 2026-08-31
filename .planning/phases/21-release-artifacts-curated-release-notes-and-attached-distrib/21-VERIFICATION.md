---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
verified: 2026-08-31T23:45:00Z
status: human_needed
score: 11/13 must-haves verified
behavior_unverified: 2
overrides_applied: 0
human_verification:
  - test: "From a machine with working `docker` (or a `packages:read`-scoped credential) outside CI, run `docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2` (or the digest of the next real release) and confirm the pull succeeds."
    expected: "The image pulls cleanly by the immutable digest the release body names — the literal ARTIFACT-06 acceptance clause ('whose image pulls by the digest the release names')."
    why_human: "21-ARTIFACT-EVIDENCE.md's own 'What this run does not prove' section states this executor could not do this: no local `docker` in the sandbox, an anonymous ghcr.io token was refused (401), and the operator's fine-grained PAT lacked the `packages:read` scope (403/404). The two corroborating readings cited in the evidence are both from *inside* the same CI run — the build step's self-reported digest, and a later `docker pull` in that job that pulled by *tag*, not by digest. Neither is the specific out-of-band pull-by-digest ARTIFACT-06 asks for. `COVERAGE.md` row 'Pull an image by immutable digest ... INTEGRATE ... 21-06 (rehearsal proof)' overstates this — it was not literally exercised."
  - test: "Run `./paladin-cli --help` (or equivalent) on the actual `ubuntu-latest`-built `x86_64-unknown-linux-gnu` archive from a host with glibc >= 2.39, or in the CI runner's own environment."
    expected: "The binary executes and produces real output (matching the clean runs already observed for `paladin` and `paladin-server` from the same archive)."
    why_human: "21-ARTIFACT-EVIDENCE.md D-14 Item 4 records `paladin-cli` failing with `GLIBC_2.38'/'GLIBC_2.39' not found` in this executor's Debian 12 sandbox. The binary was confirmed to be a well-formed, correctly linked ELF executable by static inspection (`readelf`) and its checksum verified, but no process was ever actually run from it in any verification pass to date."
gaps: []
---

# Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables — Verification Report

**Phase Goal:** A published release says what changed in the words this project already wrote
(release body = curated `CHANGELOG.md` section for the tag's version, missing section fails the
run with no git-log fallback) and hands a consumer something they can actually run and verify:
every advertised binary actually built under the features its target requires (a leg producing no
executable fails), the Docker image bound to the release by immutable digest, verifiable checksums
with instructions in the release, only actually-produced artifacts advertised, archived actions
(`create-release@v1`/`upload-release-asset@v1`) and `upload_url` plumbing and the dead Windows
strip guard removed, and the whole path proven end-to-end on a throwaway tag or recorded as
unverified.

**Verified:** 2026-08-31
**Status:** human_needed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | ARTIFACT-01: Release body = curated `CHANGELOG.md` section, missing section is a hard failure, no `git log` fallback | ✓ VERIFIED | `scripts/extract-changelog-section.sh` present and correct; `tests/scripts/extract-changelog-section_test.sh` 16/16 assertions pass locally; `grep -c 'git log --pretty' .github/workflows/release.yml` = 0; live rehearsal (`21-ARTIFACT-EVIDENCE.md` D-14 Item 3) confirms the body's curated portion matches the tagged `CHANGELOG.md` section byte-for-byte (both empty for the heading-only `0.8.1-rc.5` case) |
| 2 | ARTIFACT-02: Every advertised binary built under required features; a leg producing no executable fails | ✓ VERIFIED | `scripts/package-release-binaries.sh` asserts presence before archiving (`::error::expected binaries not built for <target>: <names>`); `--features cli,web-server`(`,vendored-openssl` on aarch64) explicit in `release.yml`; `tests/scripts/package-release-binaries_test.sh` 17/17 assertions pass; live rehearsal (D-14 Item 4 / "Measurements") confirms all four legs (incl. aarch64) produced all three binaries, settling RESEARCH Assumption A2 |
| 3 | ARTIFACT-03: Release body references only artifacts the run actually produced | ✓ VERIFIED | No `:latest` string in `release.yml`; `finalize-release-body.sh`'s downloads/verification/container-image/SBOM sections are populated only from real upstream job outputs and `gh release view --json assets`, never a static template; `tests/scripts/finalize-release-body_test.sh` covers omission-on-failure/skip cases (case 15, 25, etc.) |
| 4 | ARTIFACT-04: Image bound to release by immutable digest; size check no longer silently passes | ✓ VERIFIED (composition/normalization) — digest-correctness *pull* itself is UNCERTAIN, see below | `build-docker` exposes `digest`/`tags_json`/`image_size_mb` job outputs; digest normalized exactly once (`_frb_normalize_digest`); no `::warning::` annotation remains (`grep -c '::warning::'` = 0); live rehearsal settles RESEARCH Assumption A1 (digest already carries `sha256:` prefix) and measures image size (86 MB) |
| 5 | ARTIFACT-05: Attached artifacts verifiable in one command; SBOM scope stated correctly | ✓ VERIFIED | `aggregate_checksums` produces `SHA256SUMS`, uploaded via `--clobber`; body states both `sha256sum -c SHA256SUMS` and `shasum -a 256 -c SHA256SUMS`; SBOM section states "root `paladin-ai` package only"; live rehearsal D-14 Item 1: `sha256sum -c SHA256SUMS` against the real downloaded archives reported `OK` for all four — **PASS**, real command, real output |
| 6 | ARTIFACT-06: Archived actions/`upload_url`/dead Windows guard removed; whole path proven on a throwaway tag | ✓ VERIFIED (removal) / ⚠️ PARTIAL (proof), see below | `grep -c 'upload-release-asset@v1\|upload_url\|windows-latest'` all = 0; real rehearsal run 33436573814 on tag `v0.8.1-rc.5`, all 12 jobs green, evidence measured/dated/run-URL-sourced in `21-ARTIFACT-EVIDENCE.md` — not a re-read of the workflow |
| 7 | Prohibition (21-06 must_haves): "MUST NOT present the artifact path as working when the rehearsal was not run" | ✓ VERIFIED | The rehearsal WAS run (run 33436573814); every evidence-file claim traces to a run URL, a command this executor ran itself, or a measured figure — no claim cites re-reading `release.yml` as evidence |
| 8 | Code review findings closed: `finalize-release-body.sh` crashing on a zero-archive release (CR-01) | ✓ VERIFIED | Fix present at `scripts/finalize-release-body.sh` (`if ! "${gh}" release download ...; then :; fi`); the stub in `tests/scripts/finalize-release-body_test.sh` case 15 was itself corrected to model the real `gh` "no assets match" failure (exit 1, writes nothing) rather than always succeeding — a genuine behavioral regression test, not just a presence check; 84/84 assertions pass |
| 9 | Code review findings closed: `workflow_dispatch` `verify-tag-source` pathspec bug (CR-02) | ✓ VERIFIED | `git rev-list -n 1 -- "$RELEASE_TAG"` → `git rev-list -n 1 "$RELEASE_TAG"` confirmed in `.github/workflows/release.yml`; independently reproduced by this verifier (`git rev-list -n 1 "v0.4.1"` resolves correctly with no `--`) — this specific workflow_dispatch path was not exercised by the tag-push rehearsal, but the fix is a minimal, directly-verified one-liner |
| 10 | Code review WR-01/WR-04 closed (CR-01 convention violation; fragile positional jq lookup) | ✓ VERIFIED | `META_JSON` now routed through `env:`; `to_entries[2].value` replaced with `.image_size_mb` (named lookup) — confirmed by direct grep of `.github/workflows/release.yml` |
| 11 | Code review WR-02/WR-03 closed (stale docs contradicting current create-or-reuse and D-05 behavior) | ✓ VERIFIED | `docs/src/appendix/release-automation.md` no longer claims `actions/create-release@v1`-style upsert failure or "cause undiagnosed" for the binary-build defect this phase fixed |
| 12 | Docker image pulls by the exact digest the release names, from outside the CI run that produced it | ⚠️ PRESENT_BEHAVIOR_UNVERIFIED | See Human Verification item 1 below — not independently exercised |
| 13 | All three shipped binaries execute (ARTIFACT-02's "a consumer can actually run" intent) | ⚠️ PRESENT_BEHAVIOR_UNVERIFIED | `paladin`/`paladin-server` executed and produced real output; `paladin-cli` blocked by a sandbox glibc mismatch, confirmed only by static ELF inspection — see Human Verification item 2 |

**Score:** 11/13 truths verified (2 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `scripts/extract-changelog-section.sh` | Curated section extraction, hard-fail on missing version | ✓ VERIFIED | Exists, substantive, wired into `create-release`; 16/16 local assertions pass |
| `tests/scripts/extract-changelog-section_test.sh` | Regression harness | ✓ VERIFIED | 16 assertions, exit 0 |
| `scripts/package-release-binaries.sh` | Assert→strip→archive→checksum | ✓ VERIFIED | Exists, substantive, wired into `build-binaries`; 17/17 local assertions pass |
| `tests/scripts/package-release-binaries_test.sh` | Regression harness | ✓ VERIFIED | 17 assertions, exit 0 |
| `scripts/finalize-release-body.sh` | Marker-based truncate-and-rebuild + checksum aggregation | ✓ VERIFIED | Exists, substantive, wired into terminal `finalize-release-body` job; 84/84 local assertions pass (44 base + 40 aggregation) |
| `tests/scripts/finalize-release-body_test.sh` | Regression harness (incl. CR-01 zero-archive regression) | ✓ VERIFIED | 84 assertions, exit 0, includes the fixed stub that now models real `gh` failure |
| `.planning/phases/.../21-ARTIFACT-EVIDENCE.md` | Rehearsal evidence, measured/dated/run-URL-sourced | ✓ VERIFIED | Exists; covers all four D-14 acceptance items with two honestly-recorded PARTIAL verdicts, not silent passes |
| `docs/src/appendix/release-automation.md`, `release-checklist.md`, `release-recovery.md` | Consumer-facing docs describing the new pipeline | ✓ VERIFIED | All required content present (grep-confirmed); `make check-doc-config` exits 0; stale caveats (WR-02/WR-03) corrected |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `create-release` | `extract-changelog-section.sh` → `--body-file` | `create-or-reuse-release.sh` | ✓ WIRED | `grep -c 'extract-changelog-section.sh' .github/workflows/release.yml` = 1, invoked with `--output "${RUNNER_TEMP}/release-body.md"` |
| `Cargo.toml` `[[bin]]` `required-features` | `--features` flag | `build-binaries` `Build binary` step | ✓ WIRED | `cli,web-server`(`,vendored-openssl`) explicit on every leg, confirmed by grep and live rehearsal |
| `build-binaries`/`sbom` uploads | released asset | `gh release upload <tag> --clobber` | ✓ WIRED | 0 occurrences of `upload-release-asset@v1`/`upload_url`; 2 occurrences of `gh release upload` with `--clobber` |
| `build-docker` `steps.build.outputs.digest` + `steps.meta.outputs.json` | release body | `finalize-release-body` job → `finalize-release-body.sh` → `gh release edit --notes-file` | ✓ WIRED | Job outputs declared; script consumes via `env:`; live rehearsal shows the composed digest in the published body matching the build step's own capture |
| `build-binaries` matrix legs → uploaded assets | aggregated `SHA256SUMS` | `gh release download` → `aggregate_checksums` → `gh release upload --clobber` | ✓ WIRED | Confirmed by both the 84-assertion harness and D-14 Item 1's live `sha256sum -c` pass |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|--------------|--------|----------|
| ARTIFACT-01 | 21-01, 21-05, 21-06 | Curated changelog body, hard-fail on missing section | ✓ SATISFIED | See Truth #1 |
| ARTIFACT-02 | 21-02, 21-05, 21-06 | Feature-correct binaries, leg fails on missing executable | ✓ SATISFIED | See Truth #2, but see Truth #13 for a narrower "does it run" caveat |
| ARTIFACT-03 | 21-01, 21-03, 21-04 | Body advertises only actually-produced artifacts | ✓ SATISFIED | See Truth #3 |
| ARTIFACT-04 | 21-03 | Digest-bound image; size check no longer silently passes | ✓ SATISFIED (composition) — see Truth #12 for the narrower "does it pull" caveat | See Truth #4 |
| ARTIFACT-05 | 21-02, 21-04, 21-05 | One-command verifiable checksums; SBOM scope stated | ✓ SATISFIED | See Truth #5, live `sha256sum -c` PASS |
| ARTIFACT-06 | 21-02, 21-05, 21-06 | Maintained actions, dead guard removed, rehearsed end-to-end | ✓ SATISFIED (removal + rehearsal-run) — see Truth #12 for the one criterion not literally exercised | See Truth #6, #7 |

All six requirement IDs declared in Phase 21 plan frontmatter are also listed and marked "Complete" in `.planning/REQUIREMENTS.md`'s traceability table — no orphaned requirements.

### Anti-Patterns Found

None found in the phase's modified files. No `TBD`/`FIXME`/`XXX` debt markers (the one `XXXXXX` hit in `extract-changelog-section.sh` is a `mktemp` template, not a debt marker). No placeholder/stub returns in any of the three new scripts — all three implement full, tested logic paths.

### Code Review Follow-Through

`21-REVIEW.md` found 2 critical + 5 warning issues after the rehearsal. `21-REVIEW-FIX.md` claims 6 fixed, 1 skipped. This verifier independently confirmed, by reading the current tree (not trusting the claim):

- **CR-01** (zero-archive crash): fix present in `scripts/finalize-release-body.sh`; regression test (case 15) genuinely exercises the real `gh` failure mode via a corrected stub, not just a happy-path re-check. 84/84 assertions pass.
- **CR-02** (`workflow_dispatch` pathspec bug): fix present in `.github/workflows/release.yml`; independently reproduced by this verifier in an isolated git repo (`git rev-list -n 1 "$TAG"` resolves correctly without `--`).
- **WR-01, WR-04**: both fixes confirmed present by direct grep (`env: META_JSON`, `.image_size_mb` named lookup).
- **WR-02, WR-03**: stale doc claims corrected, confirmed by reading the current `release-automation.md`.
- **WR-05** (duplicated version-resolution logic): explicitly and reasonably skipped — non-functional (pure duplication risk, not a bug), and the stated reason (avoiding a job-graph change to the pipeline that was just rehearsed green, without a second rehearsal to prove the refactor) is sound. Not a phase-blocking gap.

None of the fix commits were re-run through the live workflow (the fixes post-date the rehearsal, as the task briefing flagged). The two behavior-relevant fixes (CR-01, CR-02) are each covered by a real local regression proof: CR-01 by a corrected stub in the 84-assertion harness that models the actual failure mode, and CR-02 by direct, independent reproduction of the git command in isolation. Neither is merely "code present," and neither required a second live GitHub Actions run to be credible.

### Human Verification Required

1. **Digest pull-by-digest from outside CI.** `21-ARTIFACT-EVIDENCE.md`'s own "What this run does not prove" section states plainly that this executor could not perform an out-of-band `docker pull <image>@sha256:...` (no local `docker`; anonymous ghcr.io token refused; operator's PAT lacked `packages:read`). The two corroborating readings cited are both internal to the same CI run, and one of them is a pull by *tag*, not by digest — neither is the literal ARTIFACT-06 clause ("whose image pulls by the digest the release names"). `COVERAGE.md`'s row claiming "21-06 (rehearsal proof)" for "Pull an image by immutable digest" overstates what was actually exercised; recommend either correcting that row or closing this gap with a credential that has `packages:read`.
2. **`paladin-cli` execution.** Confirmed as a valid, correctly linked ELF binary by static inspection and checksum, but never actually executed in any verification pass (blocked by a glibc mismatch in this sandbox vs. the `ubuntu-latest` runner that built it). Recommend running it once from a compatible host or CI step to close this out.

Both gaps are honestly disclosed in `21-ARTIFACT-EVIDENCE.md` itself (recorded as PARTIAL, not glossed over as PASS) — this verification confirms that disclosure is accurate and elevates both to explicit human-verification items rather than silently accepting the corroborating-but-not-identical evidence as a full pass.

### Gaps Summary

No BLOCKER-level gaps. All automated checks (117 total local test-harness assertions across three regression suites, `make check-gates`, `make check-doc-config`, `make test-shell-guards`, `./scripts/check-workflow-triggers.sh`, `shellcheck --severity=warning` on every new script) pass against the current tree, including the six post-rehearsal review fixes. All six ARTIFACT-01..06 requirements are structurally and (for five of six) behaviorally satisfied, cross-referenced in `.planning/REQUIREMENTS.md`, with no orphaned requirements.

The phase is held at `human_needed` rather than `passed` because two specific, narrow claims — literal pull-by-digest from outside the CI run, and `paladin-cli` actually running — were not independently proven, and the evidence file and this verification both say so plainly rather than letting corroborating-but-not-identical CI-internal readings stand in for the real check. Given the strength of everything else (a full, green, 12-job rehearsal on a real tag; 117 passing local assertions; every structural removal/addition confirmed by direct grep and file inspection; six of seven review findings fixed and independently re-verified in this pass), these are reasonably classified as follow-up confirmations rather than phase-blocking defects.

---

_Verified: 2026-08-31_
_Verifier: Claude (gsd-verifier)_
