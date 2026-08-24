---
phase: 16-documentation-currency-the-architecture-gap
plan: 13
subsystem: infra
tags: [devcontainer, vhs, ttyd, ffmpeg, asciinema, apt, supply-chain, d-14, d-11]

# Dependency graph
requires:
  - phase: 16-01
    provides: "The pinned-install / both-Dockerfiles pattern (D-11) this plan extends to the recorder toolchain"
provides:
  - "vhs 0.11.0, ttyd 1.7.7, ffmpeg 5.1.9, asciinema 2.2.0 resolving on PATH locally and in both devcontainer images"
  - "16-DOCS-04-TOOLCHAIN.md: recorded provenance, human-verify approval, and the recordings size-budget decision"
affects: ["16-14"]

# Tech tracking
tech-stack:
  added: ["vhs 0.11.0 (Charm APT repo)", "ttyd 1.7.7 (GitHub release binary)", "ffmpeg 5.1.9 (Debian repo)", "asciinema 2.2.0 (Debian repo)"]
  patterns:
    - "Third-party APT key installed to a dedicated ASCII-armored keyring file (signed-by=), never the global trusted keyring — reuses the NodeSource .asc pattern already in Dockerfile.dev instead of pulling in gnupg"
    - "GitHub release binary pinned to an explicit tag with its published SHA256 verified before install (fails closed on mismatch)"

key-files:
  created: []
  modified:
    - .devcontainer/Dockerfile.dev
    - .devcontainer/Dockerfile
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-04-TOOLCHAIN.md

key-decisions:
  - "Checkpoint 1 (human-verify, blocking-human): APPROVED 2026-08-24 by the project owner. Recorded without upgrading the claim — the prior executor's limitation (fingerprint could only be sourced from repo.charm.sh and Charm's own GitHub README, not a fully independent third party; keys.openpgp.org returned 404) is preserved verbatim, not erased by the approval."
  - "Checkpoint 2 (decision, blocking): option-a selected, 2 MB per committed .gif budget (~8 MB ceiling across the four demos). Re-record from .tape at reduced width/fps/length if exceeded; never commit over-budget and fix later."
  - "vhs pinned to 0.11.0 (newest available in the Charm APT repo at execute time, apt-cache madison showed 0.1.0-0.11.0)"
  - "ttyd pinned to release tag 1.7.7, ttyd.x86_64 asset, SHA256 8a217c96...4c5e59bf51db2f2cd12b7be4f55 verified before install"
  - "ffmpeg/asciinema left unpinned beyond the base image tag — pinning a distro package version across bookworm and bullseye would be an unsatisfiable constraint in one of the two images; documented in both Dockerfiles and in 16-DOCS-04-TOOLCHAIN.md"
  - "Charm's signing key installed as an ASCII-armored .asc file (not gpg --dearmor'd) since apt supports armored keyrings natively and no gpg binary exists in either base image — matches the existing NodeSource key pattern in Dockerfile.dev rather than introducing a new convention"

requirements-completed: [DOCS-04]

coverage:
  - id: D1
    description: "vhs, ttyd, ffmpeg, asciinema all resolve on PATH locally with verbatim --version output recorded"
    requirement: "DOCS-04"
    verification:
      - kind: other
        ref: "command -v vhs && command -v ttyd && command -v ffmpeg && command -v asciinema (all succeed); vhs --version / ttyd --version / ffmpeg -version / asciinema --version recorded in 16-DOCS-04-TOOLCHAIN.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "Both devcontainer images carry the identical recorder-toolchain install block, third-party repo keyring-scoped, no apt-key add, both non-distro tools explicitly pinned"
    requirement: "DOCS-04"
    verification:
      - kind: other
        ref: "grep -c for vhs/ttyd/ffmpeg/asciinema/signed-by= >=1 in both Dockerfiles; grep -c 'apt-key add' == 0 in both; vhs=0.11.0 and releases/download/1.7.7 present in both"
        status: pass
    human_judgment: false
  - id: D3
    description: "vhs and ttyd did not land in either Dockerfile until the human-verify checkpoint was approved; the size budget was decided before any recording binary exists"
    requirement: "DOCS-04"
    verification: []
    human_judgment: true
    rationale: "Checkpoint approval and the decision selection are human/orchestrator judgment calls captured in the checkpoint resolutions, not something a unit test can assert; the SUMMARY and 16-DOCS-04-TOOLCHAIN.md record what was approved/selected and by whom."

duration: ~25min (Task 3 only; excludes the prior executor's Checkpoint 1 evidence-gathering session)
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 13: Recorder Toolchain Provisioning Summary

**vhs 0.11.0, ttyd 1.7.7 (checksum-verified), ffmpeg 5.1.9, and asciinema 2.2.0 installed locally and in both devcontainer images behind an approved human-verify checkpoint on the vhs/ttyd supply chain, with a 2 MB/file recordings size budget decided before any binary exists.**

## Performance

- **Duration:** ~25 min (this continuation session, Task 3 through SUMMARY)
- **Started:** 2026-08-24 (continuation spawn)
- **Completed:** 2026-08-24
- **Tasks:** 3 (Checkpoint 1, Checkpoint 2, Task 3 — Checkpoints 1 and 2 resolved by human/orchestrator before this continuation)
- **Files modified:** 3 (`.devcontainer/Dockerfile.dev`, `.devcontainer/Dockerfile`, `16-DOCS-04-TOOLCHAIN.md`)

## Accomplishments

- Installed `ttyd` 1.7.7 from its GitHub release asset (`ttyd.x86_64`), verifying the published SHA256 checksum before installing to `/usr/local/bin/ttyd`
- Added the Charm APT repository for `vhs`, with its signing key installed to a dedicated ASCII-armored keyring file (`/etc/apt/keyrings/charm.asc`) referenced by `signed-by=` — never the globally-trusted keyring, no `apt-key add` — and pinned `vhs` to explicit version `0.11.0`
- Installed `ffmpeg` and `asciinema` from the base image's own Debian repository, with the deviation from explicit pinning recorded and reasoned in both Dockerfiles and the toolchain doc
- Added the identical three-`RUN` install block to both `.devcontainer/Dockerfile.dev` (bookworm) and `.devcontainer/Dockerfile` (bullseye), following D-11's pattern
- Recorded Checkpoint 1's approval and Checkpoint 2's decision in `16-DOCS-04-TOOLCHAIN.md`, preserving the prior executor's honestly-recorded fingerprint-sourcing limitation rather than implying independent verification occurred

## Task Commits

1. **(pre-checkpoint) Record vhs/ttyd supply-chain provenance** - `6e9f845a` (docs) — prior executor, before this continuation
2. **Task 3: Install the recorder toolchain locally and add it to both devcontainer images** - `55f9fb24` (feat)

**Plan metadata:** (this commit)

## Files Created/Modified

- `.devcontainer/Dockerfile.dev` - Added the ttyd/vhs/ffmpeg+asciinema install block after the `cargo-nextest` install
- `.devcontainer/Dockerfile` - Same install block, same anchor position (bullseye base)
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-04-TOOLCHAIN.md` - Extended with Checkpoint 1 approval, Checkpoint 2 decision, and Task 3's local verification evidence (verbatim tool versions, checksum verification transcript, Dockerfile-change description)

## Decisions Made

- Recorded Checkpoint 1's approval strictly as an authorization, not as evidence that an independent, third-party fingerprint comparison occurred — the earlier gap (only `repo.charm.sh` and Charm's own GitHub README were reachable; `keys.openpgp.org` returned 404 for this fingerprint) remains open and stated as such.
- Installed the Charm signing key as a plain ASCII-armored `.asc` file rather than `gpg --dearmor`-ing it, because no `gpg` binary exists in either base image and apt accepts armored keyrings natively via `signed-by=` — this matches the convention `Dockerfile.dev` already uses for the NodeSource key, rather than introducing a new one or adding a `gnupg` dependency just for this one key.
- Pinned `vhs` to `0.11.0` (the newest version `apt-cache madison vhs` listed against the Charm repo at execute time) rather than an older or floating version, since the plan requires an explicit pin and gives no target version to match.
- Left `ffmpeg`/`asciinema` unpinned beyond the base image tag, per the plan's own instruction, since pinning a specific distro package version across bookworm and bullseye risks an unsatisfiable constraint in one of the two images.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Local `asciinema --version` check blocked by a root-owned `.config` directory**
- **Found during:** Task 3, local install verification
- **Issue:** `asciinema --version` failed with `PermissionError: [Errno 13] Permission denied: '/home/vscode/.config/asciinema'` because `/home/vscode/.config` was owned by `root` in this sandbox (a pre-existing artifact of an earlier root-run process in this environment, unrelated to the Dockerfile install path itself — the Dockerfile's own user-creation sequence does not leave `.config` root-owned).
- **Fix:** `sudo chown vscode:vscode /home/vscode/.config`, then `asciinema --version` succeeded (`asciinema 2.2.0`). No Dockerfile change was needed — this was purely a local sandbox artifact, not a build-time defect, so no `HOME`/`.config` handling was added to either Dockerfile.
- **Files modified:** none (local filesystem permission only)
- **Verification:** `asciinema --version` returns `2.2.0` after the chown.
- **Committed in:** n/a (not a tracked-file change)

**2. [Rule 1 - Bug] Own-comment self-defeat: "no apt-key add" text tripped the negative-assertion grep**
- **Found during:** Task 3, running the plan's own `<verify>` block against the drafted Dockerfile comments
- **Issue:** The first drafted comment explaining the keyring choice read "...never the globally-trusted keyring (no `apt-key add`)." — which contains the literal substring `apt-key add`, so `grep -q 'apt-key add' "$f"` (the plan's own negative assertion that the deprecated path was not used) matched the comment text itself, not an actual `apt-key add` invocation, and would have reported the file as violating the very rule it was documenting compliance with.
- **Fix:** Reworded the comment to "the deprecated global-keyring-add path is never used" — same meaning, no literal substring match. Applied identically to both Dockerfiles.
- **Files modified:** `.devcontainer/Dockerfile.dev`, `.devcontainer/Dockerfile`
- **Verification:** `grep -c 'apt-key add' .devcontainer/Dockerfile.dev .devcontainer/Dockerfile` → `0` for both, after the fix.
- **Committed in:** `55f9fb24` (Task 3 commit — caught before commit, so the commit contains only the corrected wording)

---

**Total deviations:** 2 auto-fixed (1 Rule 3 blocking, 1 Rule 1 bug — both caught and resolved during Task 3, before commit)
**Impact on plan:** Neither affected the Dockerfile install logic itself, only local-verification friction (deviation 1) and comment wording (deviation 2). No scope creep; both fixes were necessary to complete Task 3's own acceptance criteria.

## Issues Encountered

None beyond the two auto-fixed deviations above. Both egress precondition checks (`repo.charm.sh`, `github.com`) re-measured `200` immediately before install, as required, so no substitution or halt was needed. The ttyd checksum matched on the first download attempt.

## User Setup Required

None - no external service configuration required. (The devcontainer rebuild that will pick up these Dockerfile changes happens on the next `.devcontainer` rebuild, which is a normal dev-environment refresh, not a manual setup step.)

## Next Phase Readiness

- All four recorder tools are provisioned, pinned (where a registry exists), and provenance-recorded — plan 16-14 can now write the `.tape` scripts and produce the actual demo recordings under the 2 MB-per-file budget decided in Checkpoint 2.
- `docs/assets/recordings/` remains empty, as required — this plan produces no recording.
- No blockers for 16-14. The one open item worth carrying forward: the Checkpoint 1 approval record explicitly notes the fingerprint-sourcing gap (only `repo.charm.sh` and Charm's own GitHub README were reachable) is accepted-but-open, not resolved — a future security-focused phase could still pursue a fully independent (e.g., keyserver-mirrored) fingerprint confirmation if that gap becomes material.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

All 4 claimed files verified present on disk (`.devcontainer/Dockerfile.dev`,
`.devcontainer/Dockerfile`, `16-DOCS-04-TOOLCHAIN.md`, this SUMMARY). Both task commit
hashes (`6e9f845a`, `55f9fb24`) verified present in `git log --oneline --all`.
