# DOCS-04 Recorder Toolchain — Provenance and Decisions

Plan: 16-13. Purpose: record the provenance of the `vhs`/`ttyd` supply chain for the blocking
human-verify checkpoint (Checkpoint 1), and the recordings size-budget decision (Checkpoint 2),
**before** either package lands in a devcontainer image. Gathered by the executor per the
checkpoint's `<what-built>` instruction — none of this data was installed, only fetched and
inspected read-only. Task 3 (after both checkpoints resolve) appends the local `--version`
output of all four tools and the final Dockerfile lines.

## Checkpoint 1 — vhs / ttyd supply-chain evidence (gathered, not verified)

**Egress reachability (measured 2026-08-24, this environment):**

| Host | Check | Result |
|---|---|---|
| `https://repo.charm.sh/apt/gpg.key` | `curl -sS -o /dev/null -w '%{http_code}'` | `200` |
| `https://github.com/tsl0922/ttyd/releases` | `curl -sS -o /dev/null -w '%{http_code}'` | `200` |

### vhs / Charm APT repository

- **Repository URL (from the official VHS README, `charmbracelet/vhs`, `main` branch, fetched
  2026-08-24):** `https://repo.charm.sh/apt/`
- **Key URL:** `https://repo.charm.sh/apt/gpg.key`
- **README's documented install pipeline (verbatim, `README.md:120-124`):**
  ```sh
  sudo mkdir -p /etc/apt/keyrings
  curl -fsSL https://repo.charm.sh/apt/gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/charm.gpg
  echo "deb [signed-by=/etc/apt/keyrings/charm.gpg] https://repo.charm.sh/apt/ * *" | sudo tee /etc/apt/sources.list.d/charm.list
  # Install ttyd from https://github.com/tsl0922/ttyd/releases
  sudo apt update && sudo apt install vhs ffmpeg
  ```
  This confirms the repository URL and key URL the plan names are the ones Charm itself
  documents, and confirms Charm's own docs tell the reader to get `ttyd` from GitHub releases
  rather than from `repo.charm.sh` — matching this plan's split provenance.
- **Key fingerprint, computed independently** (no `gpg` binary was available in this
  environment; the key was fetched with `curl`, dearmored, and its packets parsed in pure
  Python per RFC 4880 §5.5.2 — SHA-1 over `0x99 || 2-byte-length || public-key-packet-body` — to
  cross-check against a `gpg --dearmor` result without installing anything to a trusted keyring):
  ```
  pub  v4, RSA
  Fingerprint: ED92 7B38 BE98 1E53 CA09 153D 03BB F595 D4DF D35C
  Key ID (long, last 16 hex of fingerprint): 03BBF595D4DFD35C
  User ID: Charmbracelet Inc. (haters > /dev/null™) <vt100@charm.sh>
  ```
  Packet dump: 1 primary public key (v4, RSA) + 1 User ID + 1 self-signature + 1 subkey +
  1 subkey binding signature. No unexpected packets.
- **Independent (non-`repo.charm.sh`) publication check:** queried
  `https://keys.openpgp.org/vks/v1/by-fingerprint/ED927B38BE981E53CA09153D03BBF595D4DFD35C` —
  **404, not found.** The key is not mirrored on the SKS/Hagrid keyserver network under this
  fingerprint. An unauthenticated `api.github.com` code search for the key ID across
  `charmbracelet/*` repos returned `401 Requires authentication` (this environment has no `gh`
  auth session and no GitHub MCP tool exposed to this executor) and was not completed. **This
  means the only source this executor could reach for the fingerprint is `repo.charm.sh` itself
  and Charm's own GitHub-hosted README — the same publisher via two channels, not a fully
  independent third party.** The human verifier should treat this as a materially weaker
  out-of-band comparison than a keyserver or a security-page listing would provide, and should
  actively look for one before approving (Charm's website `charm.sh`, their Twitter/X or other
  Charm-controlled-but-differently-hosted channel, or a keyserver under a different fingerprint
  representation) rather than treating "the README says the same URL" as the out-of-band check.

### ttyd GitHub release

- **Repository:** `https://github.com/tsl0922/ttyd`
- **Latest non-prerelease, non-draft release (checked via `GET /repos/tsl0922/ttyd/releases`,
  first 10 entries, 2026-08-24):** `1.7.7`, published `2024-03-30T03:18:34Z`. Confirmed
  `draft: false`, `prerelease: false`. The nine releases before it in the same listing
  (1.7.6 down to 1.6.2) are also all non-prerelease, non-draft — 1.7.7 is not an isolated
  release-flagging anomaly.
- **⚠ Note for the human check:** the latest release is **~29 months old** as of this plan's
  execution date. That is a fact about the project's release cadence, not a defect in the
  release itself — but the checkpoint's step 3 ("confirm you are willing to add a third-party
  APT repository... that stays in the image's update path indefinitely") is partly about
  ongoing maintenance risk, and a ~2.5-year-stale upstream is relevant to that judgment.
- **Release page:** `https://github.com/tsl0922/ttyd/releases/tag/1.7.7`
- **Published checksums:** the release **does** publish a `SHA256SUMS` asset (fetched
  2026-08-24, following the GitHub release-asset redirect):
  ```
  8a217c968aba172e0dbf3f34447218dc015bc4d5e59bf51db2f2cd12b7be4f55  ttyd.x86_64
  b38acadd89d1d396a0f5649aa52c539edbad07f4bc7348b27b4f4b7219dd4165  ttyd.aarch64
  ```
  (Full eleven-line `SHA256SUMS` covers every platform asset in the release; only the two
  Linux-container-relevant architectures are reproduced here. `ttyd.x86_64` is the asset
  Task 3 will install into both devcontainer images, since neither `Dockerfile.dev`
  (`rust:1.97.1-slim-bookworm`) nor `Dockerfile` (`rust:1.97.1-slim-bullseye`) pins a
  non-default `--platform`, so both build to the host's default architecture — typically
  `amd64` in CI and on most dev machines. `ttyd.aarch64`'s checksum is recorded too in case a
  developer's devcontainer build targets `arm64` (e.g. Apple Silicon), so Task 3 does not have
  to re-fetch it.)
- **Asset filename Task 3 will download:** `ttyd.x86_64` (or `ttyd.aarch64` on an arm64 build),
  from `https://github.com/tsl0922/ttyd/releases/download/1.7.7/ttyd.x86_64`.

### Not part of this checkpoint

`ffmpeg` and `asciinema` — installed from the base image's own Debian repository in Task 3, the
same trust boundary as every other `apt-get install` line already in both Dockerfiles. No
separate provenance check applies.

---

## Checkpoint 2 — Recordings size budget

*Pending.* Recorded here once the human selects an option, before any recording binary exists.

---

## Task 3 — Local tool versions and final Dockerfile lines

*Pending checkpoint resolution.* Recorded here once both checkpoints are resolved and the
recorder toolchain is installed locally and in both devcontainer images.
