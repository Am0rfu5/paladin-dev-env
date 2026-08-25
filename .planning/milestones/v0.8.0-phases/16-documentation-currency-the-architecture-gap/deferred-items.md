# Phase 16 — Deferred Items (out-of-scope discoveries)

Items found during plan execution that were **not** fixed because they fall outside the
current task's declared scope and acceptance criteria (SCOPE BOUNDARY rule). Logged here
rather than fixed, per the executor's deviation-rule discipline.

## From 16-01, Task 2 (`docs/src/deployment/cicd.md` currency sweep)

The eight-signal battery (signal class 6, workflow/job names) turned up two further stale
workflow-file references in `cicd.md` beyond the three findings 16-RESEARCH.md had already
evidenced and that this plan's acceptance criteria cover:

- **"Docker Build Pipeline" section (`### docker-publish.yml`)** quotes a full `docker-publish.yml`
  GitHub Actions workflow. No such file exists in `ls .github/workflows/`
  (`benchmarks.yml ci.yml docs.yml feature-flags.yml pre-commit.yml release.yml`). Unlike
  `integration-tests.yml`, there is no evidence this ever existed and was deleted/absorbed — it
  reads as originally-aspirational content, same class of fabrication as the `cicd.md` findings
  this plan already fixed.
- **"Security Scanning" section (`### security.yml`)** quotes a `security.yml` workflow with an
  `audit`, `deny`, and **`snyk`** job. No such file exists in the live tree, and per
  `.github/instructions/security.instructions.md` ("Snyk was evaluated and removed
  (2026-08-18)"), Snyk has no Rust-language support and was deliberately removed from this
  project's security tooling — the `snyk` job here doesn't just name a missing file, it
  contradicts current governance. Reintroducing a Snyk-shaped example in documentation risks a
  future contributor copying it back into CI.

**Why not fixed now:** neither item is one of 16-RESEARCH.md's three pre-evidenced findings,
and no acceptance criterion in `16-01-PLAN.md` Task 2 covers them. Fixing them would have
exceeded this task's declared scope (`docs/src/deployment/cicd.md` is the plan's single target
file, but the *specific* findings to fix were pre-scoped, not "everything the battery turns
up"). Recorded here so the gap is visible rather than silently dropped.

**Suggested owner:** none of plans 16-02 through 16-05 re-visit `cicd.md` (each sweeps a
disjoint set of the other thirteen DOCS-01 files), so this is not automatically picked up
later in Phase 16. A future phase or a follow-up DOCS-01 amendment should either replace both
sections with accurate content (there is no live Docker-publish workflow and no live
security-scanning workflow to document — `ci.yml`'s own `security-audit`/`cargo-deny`/`osv-scanner`
jobs, `ci.yml:83,103,155`, are the closest real equivalents) or explicitly mark them as
illustrative/aspirational examples rather than descriptions of this repository's actual CI.

---

## RESOLVED 2026-08-24 — both `cicd.md` deferrals closed (phase-16 verification gap closure)

Plan 16-01 correctly logged these two findings as outside its own acceptance criteria rather than
fixing them silently. Phase verification (`16-VERIFICATION.md`) then flagged that deferring them
had left `docs/src/deployment/cicd.md` as the **only** swept file carrying unflagged fabricated
content — every other file settled with a known remaining gap (`monitoring.md`,
`troubleshooting.md`, `performance-tuning.md`, `output-formatting.md`) had an inline
reader-facing correction banner, and `cicd.md` had none. A reader consulting the live page saw
fabricated, governance-contradicting content presented as fact.

Both are now fixed in place, not merely flagged:

1. **"Docker Build Pipeline" / `docker-publish.yml`** — the workflow does not exist and never did.
   Replaced with the real mechanism: the `build-docker` job in `.github/workflows/release.yml:157`
   (ghcr.io registry at `release.yml:21`, multi-arch via `setup-qemu-action@v3` +
   `setup-buildx-action@v3`, `metadata-action@v5` tagging, `<version>` and `latest` tags at
   `release.yml:146-147`).

2. **"Security Scanning" / `security.yml` with a `snyk` job** — the workflow does not exist, and
   the Snyk step contradicted a recorded project decision (`security.instructions.md`: Snyk
   evaluated and **removed 2026-08-18**, no meaningful Rust coverage, a clean result means nothing
   was analysed). Replaced with the three real jobs — `security-audit` (`ci.yml:83`), `cargo-deny`
   (`ci.yml:103`), `osv-scanner` (`ci.yml:155`) — plus the local `make audit`/`deny`/`security`/
   `sbom` equivalents, and the project's own "no Rust SAST" gap stated plainly rather than papered
   over. The stale `SNYK_TOKEN` entry was removed from the required-secrets list.

Both replacements carry a dated correction banner naming what was fabricated, matching the
disclosure pattern the rest of the phase used. `mdbook build docs/` exits 0 with no broken links.
