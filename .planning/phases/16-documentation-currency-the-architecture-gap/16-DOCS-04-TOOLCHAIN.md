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

### Checkpoint 1 resolution — APPROVED

**The project owner approved the vhs/ttyd provenance and the Charm APT repository addition to
both devcontainer images on 2026-08-24.**

Record exactly what happened and no more: the project owner reviewed the evidence gathered above
and replied "approved," authorising the `vhs` and `ttyd` installs and the addition of
`repo.charm.sh` to both `.devcontainer/Dockerfile.dev` and `.devcontainer/Dockerfile`.

**This executor has no information that an independent, out-of-band fingerprint comparison was
performed against a source other than `repo.charm.sh` and Charm's own GitHub-hosted README.** The
limitation recorded above under "Independent (non-`repo.charm.sh`) publication check" stands
unchanged: the `keys.openpgp.org` lookup for this fingerprint returned 404, and the
`api.github.com` code-search cross-check could not be completed (401, no auth session). The
approval above is recorded as-is — a project-owner authorization — **not** as confirmation that
the fingerprint was independently verified, confirmed against a third party, or validated
out-of-band. That gap remains open; the approval accepts it and proceeds regardless.

---

## Checkpoint 2 — Recordings size budget

**Decision: option-a — a per-file size budget, re-record if exceeded.**

**Selected by:** the orchestrator, under explicit delegation from the project owner ("whatever
you recommend to get our docs in the best possible shape"). Resolved 2026-08-24.

**Budget: 2 MB per committed `.gif`** — a ceiling of roughly 8 MB across the four demos (30-60s,
45-90s, 60-120s, 45-90s). `.cast` files are small JSON and are not separately budgeted, but should
stay reasonable.

**Remedy when exceeded:** re-record from the `.tape` at reduced width, lower frame rate, or
trimmed length. An over-budget GIF is never committed and fixed up afterward — that is precisely
the one-way outcome this checkpoint exists to prevent.

**Rationale:** committing a binary to git history is not reversible without a history rewrite
that disrupts every clone, so the bound is set before any binary exists. A README-linked terminal
demo should be short and tight regardless of any budget, and a `.tape` is cheap to re-record — so
the budget's cost is low and its protection is permanent. Option b (no budget) was rejected as
unbounded and unwalkable-back; option c (git-lfs) was rejected as repository-wide infrastructure
imposed on every contributor and every CI job, well outside a documentation-currency phase's
scope.

Plan 16-14 (which produces the actual `.tape` scripts and recordings) enforces this budget.

---

## Task 3 — Local tool versions and final Dockerfile lines

**Precondition re-check (2026-08-24, this environment, before install):**

| Host | Check | Result |
|---|---|---|
| `https://repo.charm.sh/apt/gpg.key` | `curl -sS -o /dev/null -w '%{http_code}'` | `200` |
| `https://github.com` | `curl -sS -o /dev/null -w '%{http_code}'` | `200` |

Both re-measured 200; proceeded with the pinned install path as planned (no substitution needed).

**Local install environment:** Debian 12 (bookworm), x86_64, user `vscode` with `sudo`. This
matches `.devcontainer/Dockerfile.dev`'s base image (`rust:1.97.1-slim-bookworm`) — the image
`docker-compose.yml` actually builds (`.devcontainer/docker-compose.yml:8`,
`dockerfile: .devcontainer/Dockerfile.dev`), consistent with 16-01's finding that
`.devcontainer/Dockerfile` (bullseye) is the file that silently diverges if not updated in
lockstep.

### ffmpeg / asciinema (base image's own Debian repository)

```
$ ffmpeg -version | head -1
ffmpeg version 5.1.9-0+deb12u1 Copyright (c) 2000-2026 the FFmpeg developers
$ asciinema --version
asciinema 2.2.0
```

### ttyd (GitHub release, checksum-verified)

Downloaded `https://github.com/tsl0922/ttyd/releases/download/1.7.7/ttyd.x86_64`, verified
against the recorded checksum before install:

```
$ echo "8a217c968aba172e0dbf3f34447218dc015bc4d5e59bf51db2f2cd12b7be4f55  ttyd.x86_64" | sha256sum -c -
ttyd.x86_64: OK
```

Installed to `/usr/local/bin/ttyd` (mode 0755):

```
$ ttyd --version
ttyd version 1.7.7-40e79c7
```

### vhs (Charm APT repository, keyring-scoped)

Key fetched from `https://repo.charm.sh/apt/gpg.key` — already ASCII-armored
(`-----BEGIN PGP PUBLIC KEY BLOCK-----`), so it was installed directly as an armored keyring file
at `/etc/apt/keyrings/charm.asc` (mode 0644, world-readable), the same pattern
`.devcontainer/Dockerfile.dev` already uses for the NodeSource key — apt supports armored
keyrings natively, avoiding a `gnupg` package install just to dearmor the key. No `gpg` binary
was invoked (none is present in this base image); this reuses the fingerprint already computed
in the Checkpoint 1 evidence via pure-Python RFC 4880 parsing, not a fresh `gpg --dearmor` re-run.

Repository source line added, scoped to that keyring only:
```
deb [signed-by=/etc/apt/keyrings/charm.asc] https://repo.charm.sh/apt/ * *
```

`apt-cache madison vhs` after `apt-get update` listed versions `0.1.0` through `0.11.0`
(newest); pinned to the explicit newest version, `0.11.0` — not a floating "latest":

```
$ apt-get install -y --no-install-recommends vhs=0.11.0
...
Setting up vhs (0.11.0) ...
$ vhs --version
vhs version v0.11.0 (c6af91a)
```

### Local verification — all four resolve on PATH

```
$ command -v vhs && command -v ttyd && command -v ffmpeg && command -v asciinema
/usr/bin/vhs
/usr/local/bin/ttyd
/usr/bin/ffmpeg
/usr/bin/asciinema
```

### Dockerfile changes

The identical install block (three `RUN` instructions — ttyd, vhs, ffmpeg+asciinema) was added to
both `.devcontainer/Dockerfile.dev` (after the `cargo-nextest` install, before the commented-out
"Optional CLI tools" line) and `.devcontainer/Dockerfile` (same anchor). No difference was forced
by the two Debian releases (bookworm vs. bullseye): the Charm APT repository's sources entry uses
a flat `* *` distribution/component that matches any Debian codename, `ttyd`'s binary download is
architecture- not distro-specific, and `ffmpeg`/`asciinema` are both present in each release's own
repository under the same package names. Each RUN block's comment names: what the tools are for
(the D-14 demo recordings), that `ttyd`/`ffmpeg` are vhs's own runtime dependencies rather than
independent choices, that `asciinema` produces the `.cast` vhs cannot emit, and that the vhs/ttyd
provenance was human-verified with a pointer to this file.

**Deviation from strict pinning, recorded per the plan's requirement:** `ffmpeg` and `asciinema`
are installed via plain `apt-get install` with no explicit version, pinned only to whatever the
base image tag's own Debian repository snapshot provides at build time. Reason: the distro
repository is already the image's trust root (same boundary as every other unpinned `apt-get
install` line in both files, e.g. `git`, `curl`, `jq`), and pinning a specific distro package
version across two different Debian releases (bookworm and bullseye) would produce an
unsatisfiable constraint in at least one of the two images the moment the two releases' package
versions diverge.

No recording was produced by this plan — `ls docs/assets/recordings/` finds nothing, confirmed
after Task 3's changes. Plan 16-14 owns the `.tape` scripts and the artifacts, under the 2 MB
per-file budget decided above.
