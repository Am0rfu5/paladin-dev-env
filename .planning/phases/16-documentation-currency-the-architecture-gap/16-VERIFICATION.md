---
phase: 16-documentation-currency-the-architecture-gap
verified: 2026-08-24T16:45:00Z
status: passed
score: 4/4 must-haves verified
behavior_unverified: 0
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 3/4
  gaps_closed:
    - "docs/src/deployment/cicd.md is settled by content — no invented CI configuration remains (Success Criterion 1 / DOCS-01)"
  gaps_remaining: []
  regressions: []
notes_carried_forward: # Judged acceptable in the prior pass; re-confirmed, not gaps
  - "DOCS-03: the 76-vs-79 D-05 entry-point delta and its 'stale FR-26.3 figure' attribution note — re-confirmed: 76/76 still holds (script re-run, exit 0)."
  - "DOCS-04: the Charm APT signing-key fingerprint's uncorroborated sourcing, disclosed with owner authorisation rather than hidden — not re-derived this pass, no new evidence bears on it."
  - "DOCS-04: the vhs Chromium-runtime-library fix is still unproven by a clean container rebuild (Docker unavailable in this verification environment, same as the prior pass). This is the same open human-verification item recorded previously; carried forward as a non-blocking note per this re-verification's explicit scope rather than re-routed to human_needed, since it does not concern the closed gap and was already accepted as an environment-limited residual in the initial pass."
---

# Phase 16: Documentation Currency & The Architecture Gap Verification Report

**Phase Goal:** The documentation describes the system that exists — the fourteen guides Milestone
11 left mid-update are current against the tree, and the architecture document either covers the
seven subsystems it omits or says out loud that it is archive material.

**Verified:** 2026-08-24
**Status:** passed
**Re-verification:** Yes — after gap closure (commit `6ba9ad37`)

## Re-Verification Summary

The prior pass (`gaps_found`, 3/4) found exactly one blocking gap: `docs/src/deployment/cicd.md`
carried two unflagged fabricated sections ("Docker Build Pipeline" documenting a nonexistent
`docker-publish.yml`, and "Security Scanning" documenting a nonexistent `security.yml` with a
`snyk` job that contradicted the project's recorded 2026-08-18 Snyk-removal decision), with no
reader-facing correction banner — unlike every other file this phase touched.

Commit `6ba9ad37` ("fix(16): close verification gap — replace cicd.md's two fabricated workflow
sections") rewrote both sections. This pass independently re-verified the fix rather than trusting
the commit message or `deferred-items.md`'s RESOLVED note.

### 1. Fabrications actually removed

```
grep -n "docker-publish.yml\|security\.yml" docs/src/deployment/cicd.md
  147: (inside the correction banner, explicitly named as "No such workflow exists")
  349: (inside the correction banner, explicitly named as "No such workflow exists")
```

Both filenames now appear **only** inside the correction banners that disclose they never
existed — neither is presented as real. `ls .github/workflows/` still returns exactly
`benchmarks.yml ci.yml docs.yml feature-flags.yml pre-commit.yml release.yml` — confirms both
fabricated names remain absent from the live tree, consistent with the correction text.

### 2. Replacement content accuracy — line-by-line ground truth check

This is the check the re-verification task called "the single most important check in this
pass." Every citation in the new "Docker Build Pipeline" and "Security Scanning" sections was
checked against the live workflow files, not assumed from the commit message:

| Claim in `cicd.md` | Cited location | Actual content at that location | Match |
|---|---|---|---|
| `build-docker` job builds/publishes images | `release.yml:157` | `  build-docker:` (job header, "Build and Push Docker Images") | ✓ exact |
| Registry is `ghcr.io` | `release.yml:21` | `  REGISTRY: ghcr.io` | ✓ exact |
| Multi-arch via QEMU + Buildx | `docker/setup-qemu-action@v3`, `docker/setup-buildx-action@v3` | `release.yml:169`, `release.yml:172` — both present verbatim | ✓ exact |
| Auth via `docker/login-action@v3` | `release.yml:175` | `        uses: docker/login-action@v3` | ✓ exact |
| Tagging via `docker/metadata-action@v5` | `release.yml:183` | `        uses: docker/metadata-action@v5` | ✓ exact |
| Published tags: `<version>` and `latest` | `release.yml:146-147` | `docker pull ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ steps.get_version.outputs.version }}` / `...:latest` | ✓ exact |
| `security-audit` job = `cargo audit` | `ci.yml:83` | `  security-audit:` (job header, "Security Audit") | ✓ exact |
| `cargo-deny` job = license/dependency policy | `ci.yml:103` | `  cargo-deny:` (job header, "License & Dependency Policy") | ✓ exact |
| `osv-scanner` job = OSV database scan | `ci.yml:155` | `  osv-scanner:` (job header, "OSV Scanner") | ✓ exact |

Zero discrepancies found. Every one of the nine checkable citations resolves to the exact job,
env var, or action the prose claims — the replacement is a correction, not a re-fabrication.

### 3. `SNYK_TOKEN` and remaining Snyk mentions

`### 2. Secrets Management` (line ~458) now lists only `GITHUB_TOKEN`, `OPENAI_API_KEY`,
`KUBE_CONFIG` — `SNYK_TOKEN` is gone. `grep -n -i snyk docs/src/deployment/cicd.md` returns only
lines 349–355, all inside the "Security Scanning" correction banner explaining Snyk's removal and
explicitly instructing "**Do not reintroduce a Snyk step.**" No remaining mention presents Snyk as
something to add.

### 4. Correction-banner pattern comparison

Compared directly against `monitoring.md:18` (`**Corrected 2026-08-24 (D-09/D-12 currency
sweep).** No \`/metrics\` endpoint exists...`), `troubleshooting.md:19` (`**Corrected
2026-08-24:** there is no \`/metrics\` endpoint...`), and `performance-tuning.md:30` (`**Dated,
unverified against the current tree (corrected 2026-08-24):**...`). `cicd.md`'s two new banners
(lines 146-150, 348-355) use the same bold `**Corrected 2026-08-24 (...)**` lead-in, state
plainly that the described artifact does not exist, and point to the real equivalent — delivered
as a blockquote (`>`) rather than a plain paragraph, a formatting variant but functionally the
same disclosure pattern (bold date-stamped lead, "no such X exists," real pointer). Consistent
with the rest of the phase.

### 5. `mdbook build docs/` and TOC anchors

Independently re-ran `mdbook build docs/`: exit 0, final log line
`[...] INFO mdbook_linkcheck] No broken links found` (the interspersed `WARN ... fragment
resolution isn't implemented` lines are the same pre-existing linkcheck limitation noted in the
original 4-run linkcheck report, not new breakage). The TOC's `#docker-build-pipeline` and
`#security-scanning` anchors both still resolve — headings `## Docker Build Pipeline` (line 144)
and `## Security Scanning` (line 346) are unchanged from before the edit; only the body content
beneath them was replaced.

### 6. `deferred-items.md` honesty check

`deferred-items.md` retains the original deferral entry (from plan 16-01, logged as
out-of-scope) unmodified, and appends a dated `RESOLVED 2026-08-24` section that accurately
describes both fixes (job names, line citations, the `SNYK_TOKEN` removal) — matching what is
actually in the diff (`git show --stat 6ba9ad37`: `deferred-items.md` +31, `cicd.md`
161 lines changed). This is an honest closure record, not a silent overwrite of the original
finding.

### Regression check on DOCS-03 (docs-tree-adjacent, per re-verification instructions)

Since `cicd.md`'s edit lives in the docs tree `mdbook build` covers, the three DOCS-03 gates were
re-run rather than assumed unaffected:

| Gate | Command | Result |
|---|---|---|
| `cargo doc` zero-warning bar | `cargo doc --workspace --no-deps 2>&1 \| tee ... && ! grep -q "warning:" ...` | exit 0, 0 warnings |
| Public API examples gate | `bash scripts/check-public-api-examples.sh` | exit 0, "All 76 D-05 public API entry points carry a plural '# Examples' heading." |
| mdBook linkcheck | `mdbook build docs/` | exit 0, "No broken links found" |

No regression. DOCS-02 (architecture disposition) and DOCS-04 (demos) were not touched by this
commit and were not re-verified beyond the notes carried forward above.

## Goal Achievement

### Observable Truths

| # | Truth (Success Criterion) | Status | Evidence |
|---|---|---|---|
| 1 | DOCS-01: each of the fourteen guide/deployment/ops pages is checked against the tree and settled by content, linkcheck reviewed | ✓ VERIFIED | 14/14 rows present in `16-DOCS-01-VERDICTS.md`, zero `pending`. The one prior residual (`cicd.md`'s two unflagged fabrications) is now closed: both sections replaced with content verified line-by-line against `release.yml`/`ci.yml` (see Re-Verification Summary §2), carrying the same disclosure-banner pattern used by the other 13 files. |
| 2 | DOCS-02: architecture doc either covers Commander/Council/Conclave/Grove/Maneuver/Sanctum/Sentinel or is explicitly marked archive material with a live pointer | ✓ VERIFIED | Unchanged from prior pass — archive banner present, all 7 subsystems present in the live chapter, ADR-0047 recorded. Not re-derived this pass (untouched by the gap-closure commit); prior evidence stands. |
| 3 | DOCS-03: one `cargo doc` bar, CI-enforced, every public entry point documented with a working example | ✓ VERIFIED | Re-run this pass: `cargo doc` gate exit 0/0 warnings; `check-public-api-examples.sh` exit 0, 76/76; CI gate at `ci.yml:63` unchanged. |
| 4 | DOCS-04: demos recorded and indexed, or withdrawn with reason; `docs/assets/` no longer implies work in flight | ✓ VERIFIED (one non-blocking note carried forward) | Unchanged from prior pass — recordings, index, README link all present. The Chromium clean-rebuild proof remains unverifiable in this environment (Docker absent); carried forward as an accepted, disclosed, non-blocking residual per the explicit scope of this re-verification, not re-derived. |

**Score:** 4/4 truths verified.

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `docs/src/deployment/cicd.md` | Settled by content, no unflagged fabrication | ✓ VERIFIED | Both prior fabrications replaced with content independently checked against `release.yml`/`ci.yml`; both carry dated correction banners |
| `.planning/phases/16-.../deferred-items.md` | Honest closure record | ✓ VERIFIED | Original deferral retained; RESOLVED section added, accurately matches the diff |
| `.planning/phases/16-.../16-DOCS-01-VERDICTS.md` | 14 evidence-bearing verdict rows, zero pending | ✓ VERIFIED (carried forward) | Unchanged — 14 rows, all "updated → commit" |
| `.planning/phases/16-.../16-LINKCHECK-REPORT.md` | Verbatim `mdbook build docs/` output | ✓ VERIFIED (re-confirmed) | Independently re-ran this pass — exit 0, "No broken links found" |
| `docs/src/appendix/design-and-architecture.md` | Archive banner or full rewrite | ✓ VERIFIED (carried forward) | Unchanged from prior pass |
| `scripts/check-public-api-examples.sh` | Examples gate script | ✓ VERIFIED (re-run) | exit 0, 76/76 |
| `.github/workflows/ci.yml:63` | `cargo doc` zero-warning gate | ✓ VERIFIED (carried forward) | Unchanged |
| `docs/assets/recordings/*.{tape,cast,gif}`, `docs/DEMOS.md` | 4 demo scenarios + index | ✓ VERIFIED (carried forward) | Unchanged |
| `.devcontainer/Dockerfile`, `.devcontainer/Dockerfile.dev` | Recorder + mdbook toolchain pinned | ✓ VERIFIED (pins, carried forward); non-blocking note (Chromium fix unrebuilt) | Unchanged |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| No fabricated CI workflow named `docker-publish.yml`/`security.yml` presented as real | `grep -n "docker-publish.yml\|security\.yml" docs/src/deployment/cicd.md` | Both hits are inside correction banners disclosing non-existence | ✓ PASS (was ✗ FAIL in prior pass) |
| Every citation in the replacement content matches ground truth | Direct read of `release.yml`/`ci.yml` at all 9 cited line numbers | 9/9 exact matches | ✓ PASS |
| `SNYK_TOKEN` removed from required secrets | `grep -n -i snyk docs/src/deployment/cicd.md` | Only correction-banner mentions remain | ✓ PASS |
| `cargo doc` zero-warning bar holds | `cargo doc --workspace --no-deps ... && ! grep -q "warning:" ...` | exit 0, 0 warnings | ✓ PASS |
| Public API examples gate holds | `bash scripts/check-public-api-examples.sh` | exit 0, 76/76 | ✓ PASS |
| mdBook linkcheck holds | `mdbook build docs/` | exit 0, "No broken links found" | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| DOCS-01 | 16-01, 16-02, 16-03, 16-04, 16-05 | Settle 14 M11-open docs by content | ✓ SATISFIED | 14/14 clean including the now-fixed `cicd.md` |
| DOCS-02 | 16-06 | Architecture doc disposition | ✓ SATISFIED | Carried forward, unchanged |
| DOCS-03 | 16-07 through 16-12 | One `cargo doc` bar + CI gate + examples | ✓ SATISFIED | Re-run this pass, all gates passing |
| DOCS-04 | 16-13, 16-14 | Demos decision, `docs/assets/` no longer implies work-in-flight | ✓ SATISFIED | Carried forward; one sub-item remains a disclosed, accepted, environment-limited non-blocking note |

No orphaned requirements.

### Anti-Patterns Found

None in `docs/src/deployment/cicd.md` after the fix — `grep -n -E "TBD|FIXME|XXX|TODO|HACK|PLACEHOLDER"` returns no matches. The two blocker-severity findings from the prior pass (fabricated `docker-publish.yml`/`security.yml` sections, one governance-contradicting) are resolved.

## Notes Carried Forward (non-blocking, re-confirmed acceptable)

These were judged acceptable in the initial verification pass and are carried forward rather than
re-litigated, per this re-verification's explicit scope:

1. **DOCS-03 — 76-vs-79 D-05 entry-point delta.** Re-confirmed this pass: `check-public-api-examples.sh` still reports 76/76 and the count is independently re-derivable via the documented grep commands. The "stale FR-26.3 figure" attribution stands.
2. **DOCS-04 — Charm APT signing-key fingerprint.** Uncorroborated by independent source, disclosed with owner authorisation rather than hidden. Not re-derived this pass; no new evidence bears on it.
3. **DOCS-04 — vhs Chromium-libraries fix unproven by clean rebuild.** Docker remains unavailable in this verification environment (`docker: command not found`, re-checked this pass). The Dockerfile fix is present and reasoned, but has not been proven against an actual `docker build` from a clean context. This is the same open item recorded in the prior pass's Human Verification section. It is unrelated to the closed cicd.md gap and was already accepted as an environment-limited residual — carried forward as a note rather than re-routed to `human_needed`, consistent with the explicit scope given for this re-verification pass. A future session with Docker access should still perform the rebuild-and-render check described in the original report.

## Gaps Summary

No open gaps. The single blocking gap from the initial pass — `docs/src/deployment/cicd.md`'s two
unflagged fabricated CI-workflow sections — is closed. The fix was independently verified, not
trusted from the commit message: both fabrications are gone, every factual claim in the
replacement content was checked line-by-line against `release.yml` and `ci.yml` and found exactly
accurate (9/9), the stale `SNYK_TOKEN` secret is removed, the correction banners match the
phase's established disclosure pattern, `mdbook build docs/` is clean, and `deferred-items.md`
records the closure honestly. All four DOCS-0X Success Criteria are now verified.

---

_Verified: 2026-08-24 (re-verification after gap closure, commit 6ba9ad37)_
_Verifier: Claude (gsd-verifier)_
