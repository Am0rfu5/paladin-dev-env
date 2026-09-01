# Phase 21: Release Artifacts — Artifact Path Rehearsal Evidence Log

Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
Requirements: ARTIFACT-01, ARTIFACT-02, ARTIFACT-03, ARTIFACT-04, ARTIFACT-05, ARTIFACT-06
Plan: 21-06

This document is the phase's rehearsal evidence log, following the shape of
`19-PUBLISH-EVIDENCE.md` / `20-RECOVERY-EVIDENCE.md`: measured facts, dated and sourced, not
summarized-away. Re-reading `.github/workflows/release.yml` is never cited as evidence anywhere
in this document — every claim below traces to a run URL, a command this executor ran itself, or
a measured figure.

**Status of this document: complete — the rehearsal was run (Task 1: option-a).**

## Task 1 — Rehearsal authorisation (decision record)

**Date:** authorised prior to this plan's Task 2/3 execution (recorded in the plan's
continuation state at hand-off to this executor).
**Selected option:** `option-a` — run the rehearsal on the next free release candidate.
**Version chosen:** `0.8.1-rc.5` — the successor to the highest `0.8.1-rc.N` heading in the root
`CHANGELOG.md` at the time (`0.8.1-rc.4`).

## Task 2 — the rehearsal tag and run

**Date:** 2026-08-31
**Tag:** `v0.8.1-rc.5`, annotated, pointing at commit `99329420a5e6356dde303ec7e6a45870e3c321ae`
(`chore(release): version 0.8.1-rc.5`).
**Trigger:** the tag push itself (`event: push`), not `workflow_dispatch` — consistent with every
prior rehearsal in this series.
**Run:** [33436573814](https://github.com/DF3NDR/paladin-dev-env/actions/runs/33436573814)
**Overall conclusion:** `success` — all twelve jobs green, first time this series has recorded a
run with no failing job at all (the four Build Binaries matrix legs and the Docker/SBOM/finalize
jobs that previously either failed or were absent are all green here):

| Job | Conclusion |
|---|---|
| Verify Tag From Main | success |
| Pre-Publish Consistency Gate | success |
| Test Suite | success |
| Create Release | success |
| Build and Push Docker Images | success |
| Generate SBOM | success |
| Build Binaries (ubuntu-latest, x86_64-unknown-linux-gnu, paladin-linux-amd64) | success |
| Build Binaries (macos-latest, x86_64-apple-darwin, paladin-macos-amd64) | success |
| Build Binaries (macos-latest, aarch64-apple-darwin, paladin-macos-arm64) | success |
| Build Binaries (ubuntu-latest, aarch64-unknown-linux-gnu, paladin-linux-arm64) | success |
| Publish to crates.io | success |
| Finalize Release Body | success |

### Deviation — release commit travelled via PR, not a direct push (precedented shape)

`make release VERSION=0.8.1-rc.5` completed every local step (lockstep version bump, changelog
finalization, OpenAPI baseline regeneration, local consistency check) and pushed the release
commit — but the direct push to `main` was rejected by the repository's PR-only ruleset, exactly
the shape `20-RECOVERY-EVIDENCE.md` Finding 2 recorded for `0.8.1-rc.4` (and `19-PUBLISH-EVIDENCE.md`
Deviation 1 for `0.8.1-rc.1`). The release commit `99329420` travelled to `main` via **PR #48**
(merge commit `caf83fbc`); the annotated tag `v0.8.1-rc.5` — still pointing at `99329420`, verified
below as an ancestor of `origin/main` — was pushed separately after the PR merged. This is not a
new failure mode; it is the same documented-procedure gap Phase 20 recorded and left as procedure
rather than tooling (Finding 2: "documented as procedure instead").

Verified locally by this executor, on branch `chore/21-close`:

```
$ git merge-base --is-ancestor v0.8.1-rc.5 origin/main && echo "ancestor-of-main: OK"
ancestor-of-main: OK
$ git rev-parse v0.8.1-rc.5^{commit}
99329420a5e6356dde303ec7e6a45870e3c321ae
```

### Publish to crates.io — all eleven crates, consistent with prior rehearsals

`Publish to crates.io` (job `99636059501`) succeeded, publishing all eleven crates at
`0.8.1-rc.5` in the committed dependency order (`paladin-ai-core`, `paladin-ports`,
`paladin-herald`, `paladin-battalion`, … through `paladin-ai`). This permanently occupies the
`0.8.1-rc.5` version on crates.io for all eleven crates — the accepted, bounded cost Phase 19 D-04
/ Phase 20 D-14 already established, and the same cost this plan's Task 1 checkpoint authorised
for this rehearsal specifically.

---

## D-14 Acceptance Item 1: assets download and `sha256sum -c` verifies

**Date:** 2026-08-31
**Actor:** this executor, in a scratch directory (`mktemp -d`), against the real published
release — not a re-read of the workflow.

```
$ gh release download v0.8.1-rc.5 --repo DF3NDR/paladin-dev-env
$ ls
SHA256SUMS
paladin-linux-amd64.tar.gz          paladin-linux-amd64.tar.gz.sha256
paladin-linux-arm64.tar.gz          paladin-linux-arm64.tar.gz.sha256
paladin-macos-amd64.tar.gz          paladin-macos-amd64.tar.gz.sha256
paladin-macos-arm64.tar.gz          paladin-macos-arm64.tar.gz.sha256
paladin-v0.8.1-rc.5.cdx.json
$ sha256sum -c SHA256SUMS
paladin-linux-amd64.tar.gz: OK
paladin-linux-arm64.tar.gz: OK
paladin-macos-amd64.tar.gz: OK
paladin-macos-arm64.tar.gz: OK
```

The exact one-command verification the release body itself instructs (`sha256sum -c SHA256SUMS`)
ran against the real downloaded archives and reported `OK` for all four. Ten assets total were
present as advertised: the aggregated `SHA256SUMS`, four archives, four per-asset `.sha256` files,
and the CycloneDX SBOM.

**Verdict: PASS.**

## D-14 Acceptance Item 2: image pulls by the digest the release names

**Date:** 2026-08-31

**What the release body names**, read directly from `gh release view v0.8.1-rc.5 --json body`:

```
docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2
```

**What this executor could measure directly.** Per this plan's Task 3 instructions, local `docker`
is unavailable in this sandbox (confirmed: `docker: command not found`) — a `docker pull` was not
attempted or faked. The instructed fallback was to verify the digest exists in ghcr via the
registry API. This executor attempted that fallback and hit a **credential-scope wall, recorded
here rather than silently worked around**:

```
$ curl -s "https://ghcr.io/token?scope=repository:df3ndr/paladin-dev-env:pull&service=ghcr.io"
{"errors":[{"code":"UNAUTHORIZED","message":"authentication required"}]}
HTTP 401
```

Anonymous token minting for this repository/scope was refused — this package is not configured
for anonymous/public pull access at the registry token-service level, independent of the parent
repository's own public visibility (`gh api repos/DF3NDR/paladin-dev-env --jq '{visibility,private}'`
→ `{"visibility":"public","private":false}`).

With the operator's authenticated fine-grained PAT (the only credential available to this
executor):

```
$ curl -s -u "Am0rfu5:${GH_TOKEN}" \
    "https://ghcr.io/token?scope=repository:df3ndr/paladin-dev-env:pull&service=ghcr.io"
{"token":"…"}                                                          HTTP 200

$ curl -s -H "Authorization: Bearer $TOKEN" \
    -H "Accept: application/vnd.oci.image.index.v1+json, application/vnd.docker.distribution.manifest.list.v2+json" \
    "https://ghcr.io/v2/df3ndr/paladin-dev-env/manifests/sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2"
{"errors":[{"code":"MANIFEST_UNKNOWN","message":"manifest unknown"}]}  HTTP 404

$ curl -sI … "https://ghcr.io/v2/df3ndr/paladin-dev-env/manifests/0.8.1-rc.5"
HTTP 403
```

A token was successfully minted (proving the PAT authenticates), but both the by-digest and
by-tag manifest fetches were refused. This executor traced the cause independently rather than
guessing: the GitHub Packages REST API for the same package returns the same class of denial —

```
$ curl -s -H "Authorization: Bearer $GH_TOKEN" -H "Accept: application/vnd.github+json" \
    "https://api.github.com/orgs/DF3NDR/packages/container/paladin-dev-env"
{"message":"Resource not accessible by personal access token", …}      HTTP 403
```

`"Resource not accessible by personal access token"` is the fine-grained-PAT missing-permission
message, distinct from a repository-visibility or digest-validity failure. **The credential
available to this executor does not carry the GitHub fine-grained PAT "Packages" permission** —
this is a property of the token, not of the image or the digest.

**What corroborating evidence exists, sourced from the CI run itself (not this executor's own
registry query).** The `Build and Push Docker Images` job's own `Verify image size` step ran a
*second*, independent `docker pull` — separate from the `docker/build-push-action` step that
originally captured the digest — against the tag reference, and the registry's own response
included:

```
Digest: sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2
Status: Downloaded newer image for ghcr.io/df3ndr/paladin-dev-env:0.8.1-rc.5
```

This digest is byte-identical to the `containerimage.digest` / `steps.build.outputs.digest` value
the build step captured earlier in the same job (`sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`),
and is what the finalize job wrote into the release body. Two independent readings inside the same
CI run (the build step's self-report and a later, separate pull-by-tag) agree — but both originate
from within the CI run's own credentials, not from this executor's independent, out-of-band check.

**Verdict: PARTIAL — the digest's existence and correctness is corroborated by two independent
readings inside the CI run itself, but this executor's own, out-of-band registry check (the
specific mechanism this plan's Task 3 instructed) was blocked by a credential-permission gap, not
by any defect in the release. Stated plainly in "What this does and does not prove" below — this
is not counted as a full pass.**

## D-14 Acceptance Item 3: the body matches the root `CHANGELOG.md` section for that version

**Date:** 2026-08-31

**The `CHANGELOG.md` section for `0.8.1-rc.5`, read directly from the tagged commit:**

```
$ git show 99329420:CHANGELOG.md | sed -n '1,12p'
# Changelog
…
## [Unreleased]

## [0.8.1-rc.5] - 2026-08-31

## [0.8.1-rc.4] - 2026-08-29
…
```

Heading-only — no body text between the `0.8.1-rc.5` heading and the next `## [` heading. This is
the live D-02 case again (as `0.8.1-rc.4` was): a quiet release candidate whose changelog section
is legitimately empty, and D-02 requires this to be accepted, not treated as a missing-section
failure.

**Running the actual in-tree extraction script** (`scripts/extract-changelog-section.sh`) against
the tagged `CHANGELOG.md`, to confirm the extraction this run performed is exactly reproducible
locally:

```
$ ./scripts/extract-changelog-section.sh --changelog CHANGELOG.md@99329420 \
    --version "v0.8.1-rc.5" --output /tmp/extracted-section.md
section_file=/tmp/extracted-section.md
$ wc -c /tmp/extracted-section.md
1 /tmp/extracted-section.md
```

One byte — a single trailing newline, i.e. empty content. Exit `0`.

**The release body's curated portion** (everything before the `<!-- paladin:release-artifacts -->`
marker), read directly via `gh release view v0.8.1-rc.5 --json body`:

```
$ awk '/<!-- paladin:release-artifacts -->/{exit} {print}' release-body.md | wc -c
0
```

Zero bytes — empty, matching the changelog section's own emptiness. The curated portion and the
`CHANGELOG.md` section agree: both are heading-only for this version.

**The `Create Release` job's own extraction step**, from the live run's log, confirms the same
script ran with no error:

```
##[group]Run ./scripts/extract-changelog-section.sh \
  --changelog CHANGELOG.md \
  --version "$RELEASE_VERSION" \
  --output "${RUNNER_TEMP}/release-body.md"
section_file=/home/runner/work/_temp/release-body.md
```

No `::error::` line, no non-zero exit — the extraction succeeded and produced the same empty
section this executor reproduced locally.

**Verdict: PASS.**

## D-14 Acceptance Item 4: the binaries in the tarball execute on at least the native runner

**Date:** 2026-08-31, in the same scratch directory as Item 1.

```
$ tar xzf paladin-linux-amd64.tar.gz -C extracted
$ ls -la extracted
paladin         6533360 bytes
paladin-cli    13716696 bytes
paladin-server 18659584 bytes
```

All three binaries the manifest for `x86_64-unknown-linux-gnu` expects
(`scripts/package-release-binaries.sh`'s `expected_binaries_for_target`) were present in the
archive and extracted successfully.

**`paladin`:**

```
$ ./paladin --version
error: unexpected argument '--version' found
Usage: paladin [OPTIONS]
For more information, try '--help'.
exit=2
$ ./paladin --help
Usage: paladin [OPTIONS]
Options:
  -c, --config <CONFIG>  [default: config.yml]
  -h, --help             Print help
exit=0
```

This binary has no `--version`/`-V` flag at all (its clap-derived option set is only
`-c/--config` and `-h/--help`) — recorded as a fact about this binary's CLI surface, not treated
as a failure to run. `--help`, the nearest equivalent this binary actually supports, ran and
exited `0`.

**`paladin-server`:**

```
$ ./paladin-server --version
[…] INFO  paladin_server] Loading configuration from '--version'
[…] ERROR paladin_server] paladin-server failed to start: configuration file "--version" not found
exit=1
```

`paladin-server`'s argument parser treats its sole positional argument as a config-file path
rather than recognising `--version`/`--help` as flags at all — the same behaviour repeats for
`--help`. The binary executed (it ran real code: loaded its logger, attempted to open a config
file, reported a real error) rather than crashing or refusing to load — the nearest equivalent to
"ran successfully" this binary's actual interface supports is that it started, logged, and failed
cleanly on a genuinely-missing file, which is a real, working binary, not a corrupt or
non-executable one.

**`paladin-cli`:**

```
$ ./paladin-cli --version
./paladin-cli: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.38' not found (required by ./paladin-cli)
./paladin-cli: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by ./paladin-cli)
exit=1
```

This executor's verification sandbox is Debian 12 "bookworm" (`glibc 2.36`); the release binary
was built on the `ubuntu-latest` (Ubuntu 24.04) GitHub-hosted runner, which ships a newer glibc
than this sandbox provides. This is a property of the **verification environment**, not of the
release: `readelf -h`/`readelf -d` on `paladin-cli` confirm it is a genuine, valid
`ELF64 x86-64` dynamically-linked `PIE` executable, requiring `libssl.so.3`, `libcrypto.so.3`,
`libgcc_s.so.1`, `libm.so.6`, `libc.so.6` — a real, well-formed binary that this specific sandbox
cannot execute. It could not be run here; it was not corrupted, empty, or a non-executable
placeholder.

**Verdict: PARTIAL — two of three binaries (`paladin`, `paladin-server`) executed directly in this
verification sandbox with real, sensible (if not literally `--version`) output; `paladin-cli` is
confirmed to be a valid, correctly-linked ELF executable by static inspection but could not be
run here due to a glibc-version mismatch between this sandbox and the `ubuntu-latest` runner that
built it. This gap is a verification-environment limitation, not a release defect, and is stated
plainly rather than silently passed.**

---

## Measurements and settled assumptions

### Assumption A1 — `docker/build-push-action`'s digest wire format: **SETTLED**

The `digest` output is **already prefixed with `sha256:`**, verified against the live run's own
job output, independently in two places within the same run:

- The `Build and push` step's own `containerimage.digest` field:
  `"containerimage.digest": "sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2"`
- The `Finalize Release Body` job's `DOCKER_OUTPUTS_JSON` env, which is the job output actually
  consumed downstream:
  `"digest": "sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2"`

Both carry the `sha256:` prefix verbatim. The release body's pull line
(`docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e…`) is therefore correct as composed —
concatenating a second `sha256:` prefix would have produced the broken `sha256:sha256:…` form
RESEARCH.md's Code Examples section warned against, and that did not happen.

### Assumption A2 — the aarch64 leg's actual binary set: **SETTLED, no narrowing needed**

`expected_binaries_for_target` (`scripts/package-release-binaries.sh`) declares the same
three-binary manifest (`paladin`, `paladin-cli`, `paladin-server`) for all four target triples,
including `aarch64-unknown-linux-gnu`. The `Package release binaries` step for the aarch64 leg
(job `99635035115`) ran to completion with no `::error::` and printed:

```
archive_path=target/aarch64-unknown-linux-gnu/release/paladin-linux-arm64.tar.gz
checksum_path=target/aarch64-unknown-linux-gnu/release/paladin-linux-arm64.tar.gz.sha256
```

Per the script's own logic (`package-release-binaries.sh` §2), this output is only reachable if
every binary in the expected list was present as a regular file — a missing binary is a hard,
named `::error::expected binaries not built for …` failure that prevents the archive step from
running at all. No such error appears in this leg's log. **The `cli,web-server` feature
combination cross-compiles cleanly for `aarch64-unknown-linux-gnu` under `cross 0.2.5` with
`vendored-openssl`** — RESEARCH.md's MEDIUM-confidence risk assessment is now a live-verified
fact. No narrowing of `expected_binaries_for_target` was needed or made.

### Measured image size

**86 MB**, read directly from the `Verify image size` step's output (`Image size: 86 MB`) and
confirmed in the release body ("Measured image size: 86 MB — within target (500 MB target,
advisory only)."). This is the figure the deferred hard-fail-threshold decision (D-10) now has to
work from — well under the 500 MB advisory target, with roughly 5.8x headroom at this
measurement.

### macOS legs and checksums — first time ever reached

Both macOS legs (`x86_64-apple-darwin`, `aarch64-apple-darwin`) produced a `checksum_path` output
from the `Package release binaries` step with no error — the OS-portable `sha256_cmd()` fallback
(`shasum -a 256`, since `sha256sum` does not exist on `macos-latest` runners — RESEARCH.md
Pitfall 1) fired successfully on both legs. This is the first time in this project's release
history that a macOS leg has reached a checksum line at all: previously, `strip` failed first on
every macOS leg because `paladin` was never built (the ARTIFACT-02 defect this phase fixes), so
the `sha256sum`-absence pitfall was masked until this run. Both `.tar.gz.sha256` per-asset files
and their entries in the aggregated `SHA256SUMS` verified correctly in D-14 Item 1 above.

### Task 2 deviation, restated for this section

The release commit travelled via **PR #48** (merge `caf83fbc`) rather than a direct push to
`main`, because `make release`'s `git push origin HEAD` is refused by the repository's PR-only
ruleset — the same shape `20-RECOVERY-EVIDENCE.md` Finding 2 recorded for `0.8.1-rc.4` and
`19-PUBLISH-EVIDENCE.md` Deviation 1 recorded for `0.8.1-rc.1`. The tag `v0.8.1-rc.5` was pushed
separately after the PR merged, and is verified above as an ancestor of `origin/main`, satisfying
`verify-tag-source`'s precondition. This is the third time this exact deviation has been recorded
across three separate phases' evidence logs; it remains documented procedure rather than fixed
tooling (Phase 20's own recorded choice), and is repeated here for the same reason those two
documents repeat it: so this record reads as a complete account rather than omitting a real,
recurring operational fact.

---

## What this does and does not prove

**What this run proves, with real, measured evidence:**

- The complete artifact path — curated changelog extraction, feature-correct binary builds on all
  four targets (including aarch64, previously never live-verified), digest-bound Docker image,
  aggregated and per-asset checksums, SBOM attachment, and idempotent finalize-body composition —
  ran end-to-end on a real, throwaway prerelease tag and produced a real, downloadable,
  independently-verifiable GitHub release.
- Every archive downloaded from the real release verifies against the real aggregated
  `SHA256SUMS`, using the exact one-command instruction the release body itself prints.
- Two of the three shipped binaries (`paladin`, `paladin-server`) executed directly in this
  verification sandbox and produced real, sensible output for the nearest equivalent to
  `--version` each binary's own CLI surface actually supports; the third (`paladin-cli`) is
  confirmed to be a valid, correctly-linked, non-corrupt ELF executable by static inspection, but
  could not be executed in this specific sandbox due to a glibc-version mismatch against the
  `ubuntu-latest` runner that built it.
- RESEARCH.md's Assumption A1 (digest wire format) and Assumption A2 (aarch64 binary set) are both
  settled by direct measurement rather than carried forward as open items.
- The release body's curated section is byte-for-byte empty, matching the root `CHANGELOG.md`'s
  equally heading-only `0.8.1-rc.5` section — the D-02 empty-section allowance is exercised live,
  a second time (after `0.8.1-rc.4`), confirming it is not a one-off.
- The measured image size (86 MB) gives the deferred hard-fail-threshold decision (D-10) its first
  real figure to work from.
- crates.io publishing of `0.8.1-rc.5` occurred for all eleven crates, consistent with every prior
  rehearsal in this series (Phase 19/20's evidence logs) — the artifact-path changes in this phase
  did not disturb the publish path, which remained out of this phase's scope throughout.

**What this run does not prove:**

- **This executor did not independently confirm, from outside the CI run, that the digest exists
  in ghcr.io.** A local `docker pull` was not possible (`docker` is not installed in this
  sandbox); the instructed registry-API fallback was attempted with both an anonymous token (401,
  the package does not permit anonymous pulls at the token-service level) and the operator's
  authenticated fine-grained PAT (403/404, traced to the PAT lacking the GitHub "Packages"
  permission — confirmed independently via the GitHub Packages REST API returning the same
  "Resource not accessible by personal access token" denial). What this document offers instead
  is two internally-consistent readings from inside the CI run itself: the `docker/build-push-action`
  step's own captured digest, and a separate `docker pull`-by-tag performed by the `Verify image
  size` step later in the same job, both agreeing on
  `sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`. This is corroborating,
  not independent, evidence — a future rehearsal with a credential carrying `packages:read` should
  close this gap with a genuinely out-of-band check.
- **The `paladin-cli` binary was not executed by this executor.** Its ELF structure and
  dynamic-linking requirements were verified statically (`readelf`), and its checksum verified
  against `SHA256SUMS`, but no process was ever run from it in this verification pass. The gap is
  attributable to this sandbox's older glibc, not to anything about the release.
- **`workflow_dispatch` eligibility for any job in this pipeline remains untested by this
  rehearsal** — the tag was pushed, exactly as every prior rehearsal in this series has done, to
  avoid resting any part of this phase's proof on that untested path.
- **No failure mode was exercised.** Unlike Phase 20's rehearsals, this run completed cleanly on
  every job on the first attempt — it demonstrates the happy path end-to-end, not recovery
  behaviour for this phase's new jobs (`build-docker`'s size/digest capture,
  `finalize-release-body`'s idempotent re-run). Phase 20's D-03 (every job safe to run twice)
  was not re-exercised here because no job failed and needed a re-run.
- **The macOS legs' binaries were not executed anywhere** — neither this rehearsal nor any prior
  one has run a macOS binary on a macOS host; only their checksums and archive structure were
  verified.
