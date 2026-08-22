---
phase: quick-260822-gpf
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - .devcontainer/docker-compose.yml
  - .devcontainer/post-start.sh
  - .devcontainer/README.md
  - .devcontainer/FILES.md
autonomous: true
requirements:
  - "QUICK-260822-gpf: Claude Code session state survives devcontainer rebuilds"
user_setup:
  - service: host-filesystem
    why: "Docker creates a missing bind-mount source as a root-owned directory, which the vscode user then cannot write to (D-06)."
    dashboard_config:
      - task: "Run `mkdir -p ~/.claude-paladin && chmod 700 ~/.claude-paladin` on the HOST before rebuilding the container"
        location: "Host shell (not the container)"
      - task: "Rebuild the container (Dev Containers: Rebuild Container), then authenticate Claude Code once"
        location: "VS Code command palette"

must_haves:
  truths:
    - "After a devcontainer rebuild, the JSONL transcripts written before the rebuild are still present under /home/vscode/.claude/projects/-workspace/ (D-03)."
    - "After a rebuild, Claude Code starts already authenticated — no re-login — because .credentials.json lives inside the mounted dir (D-02, D-03)."
    - "Claude Code writes its global config to /home/vscode/.claude/.claude.json, INSIDE the bind mount, not to /home/vscode/.claude.json (D-04)."
    - "When the host source dir is absent or root-owned, post-start.sh prints an actionable host-side fix instead of failing silently (D-06)."
    - "/workspace/.claude/ (the project-local GSD install) is unmodified and still resolved by the GSD launcher (D-05)."
  artifacts:
    - ".devcontainer/docker-compose.yml — read-write bind mount + CLAUDE_CONFIG_DIR env var"
    - ".devcontainer/post-start.sh — mount presence/writability guard"
    - ".devcontainer/README.md — 'Claude Code session persistence' section"
    - ".devcontainer/FILES.md — updated docker-compose.yml and post-start.sh entries"
  key_links:
    - "CLAUDE_CONFIG_DIR and the bind-mount target MUST be the same path (/home/vscode/.claude). If they diverge, .claude.json lands outside the mount and persistence silently breaks."
    - "devcontainer.json `remoteUser: vscode` + `updateRemoteUserUID: true` remap the container user to the host user's UID — this is the only reason a host-owned read-write bind mount is writable."
    - "devcontainer.json `workspaceFolder: /workspace` derives the transcript key `projects/-workspace`. Changing workspaceFolder orphans every persisted session."
---

<objective>
Make Claude Code session state survive devcontainer rebuilds by relocating it onto a host
bind mount, following the pattern the repo already established for host-provided LLM API keys.

Purpose: `/home/vscode` is destroyed on every rebuild, so today every rebuild loses all session
transcripts, todos, shell snapshots, user settings and the auth token, forcing a re-login.
Output: a read-write host mount at `/home/vscode/.claude`, a `CLAUDE_CONFIG_DIR` env var that
pulls `.claude.json` inside it, a post-start guard for the root-owned-mount hazard, and updated
`.devcontainer/` docs.
</objective>

<execution_context>
@/workspace/.claude/gsd-core/workflows/execute-plan.md
@/workspace/.claude/gsd-core/templates/summary.md
</execution_context>

<context>
@.planning/STATE.md
@CLAUDE.md
@.devcontainer/docker-compose.yml
@.devcontainer/post-start.sh
@.devcontainer/paladin-env.sh
@.devcontainer/README.md
@.devcontainer/FILES.md
</context>

<verified_facts>
Measured inside THIS container on 2026-08-22. Do not re-derive these — they are load-bearing
for the design and re-deriving them costs context for no new information.

1. **On-disk layout.** `/home/vscode/.claude` exists, mode `drwx------`, owned `vscode:vscode`,
   and contains `.credentials.json` (the auth token), `.last-cleanup`, `backups/`, `ide/`,
   `projects/`, `session-env/`, `sessions/`, `shell-snapshots/`. Separately,
   `/home/vscode/.claude.json` exists at 40 KB, mode `-rw-------` — OUTSIDE the `.claude` dir,
   exactly as the problem statement describes.

2. **`CLAUDE_CONFIG_DIR` is honoured AND it relocates `.claude.json`.** Verified empirically,
   not from docs. Running the Claude Code 2.1.239 binary under
   `env -i HOME=<scratch-home> CLAUDE_CONFIG_DIR=<scratch-cfg> claude mcp add --scope user ...`
   printed `File modified: <scratch-cfg>/.claude.json` and produced
   `<scratch-cfg>/.claude.json` plus `<scratch-cfg>/backups/.claude.json.backup.<ts>`, while
   `<scratch-home>` was left COMPLETELY EMPTY. So the D-04 fallback (copy `.claude.json` in and
   out on start) is NOT needed — do not implement it.

3. **Setting `CLAUDE_CONFIG_DIR=/home/vscode/.claude` is a no-op for everything except
   `.claude.json`.** The value is the same path Claude Code already uses by default for the
   user config root, so skills/plugins/settings resolution is unchanged; the only observable
   difference is that `.claude.json` and its backups move inside the mounted dir.

4. **Session key is stable.** `/home/vscode/.claude/projects/-workspace/` already exists and
   holds the current `<session-id>.jsonl` transcripts. `-workspace` is derived from the
   workspace path `/workspace`, which `devcontainer.json` pins via `"workspaceFolder":
   "/workspace"`. That path does not change across rebuilds, so transcripts rehydrate under
   the same key and `claude --continue` / `--resume` work after a rebuild.

5. **UID alignment holds.** `devcontainer.json` sets `"remoteUser": "vscode"` and
   `"updateRemoteUserUID": true`; `id` inside the container reports
   `uid=1000(vscode) gid=1000(vscode)`. That remap is what makes a host-owned read-write bind
   mount writable from inside.

6. **No compose variable override.** D-01 permits `${PALADIN_CLAUDE_STATE_DIR:-...}` ONLY if
   nested-default interpolation is verified. There is no `docker` CLI inside this container, so
   it could not be verified. Per D-01, use the plain `${HOME}/.claude-paladin` form — matching
   the existing credentials mount — and document how to change it by editing the compose file.

7. **GSD is unaffected.** The GSD launcher probes `<repo-root>/.claude/gsd-core/bin/gsd-tools.cjs`
   BEFORE its `${CLAUDE_CONFIG_DIR:-...}` fallback branch. `/workspace/.claude/gsd-core/bin/`
   exists and `/workspace/gsd-core` does not, so the repo-local install still wins once
   `CLAUDE_CONFIG_DIR` is set. `/workspace/.claude/` must not be touched (D-05).

8. **Gates that actually apply here.** No Rust code changes, so there is no `cargo test` for this
   work — do not invent one. The pre-commit suite runs `check-yaml`, `trailing-whitespace`
   (`--markdown-linebreak-ext=md`), `end-of-file-fixer`, `gitleaks`, and
   `shellcheck --severity=warning`. `shellcheck` 0.9.0, `pre-commit` 4.6.2 and PyYAML are all
   installed in this container.
</verified_facts>

<tasks>

<task type="tracer">
  <name>Task 1: Mount Claude Code state from the host and point CLAUDE_CONFIG_DIR at it</name>
  <files>.devcontainer/docker-compose.yml</files>
  <action>
This is the whole persistence mechanism end-to-end — the mount plus the env var. Everything
after it is a guard and documentation.

In the `paladin-dev` service `volumes:` list, append one entry after the existing
`${HOME}/.config/paladin:/home/vscode/.config/paladin:ro` line, with a comment block matching
the tone and depth of the credentials-mount comment directly above it. The entry is exactly
`- ${HOME}/.claude-paladin:/home/vscode/.claude` — note there is NO `:ro` suffix, because
Claude Code must write here (D-02). Use the literal `${HOME}` form, not a
`PALADIN_CLAUDE_STATE_DIR` override; per verified fact 6 the nested-default interpolation could
not be verified, and D-01 makes the plain form the required fallback.

The comment above the entry must record four things: that this is Claude Code user state
(session transcripts, history, todos, shell snapshots, user settings, and the `.credentials.json`
auth token); that it is a DEDICATED host dir rather than the host's real `~/.claude`, so a
Claude Code session running on the host cannot race the container's (D-01); that it is
read-write unlike the credentials mount; and that the host directory must be created BEFORE
first launch with `mkdir -p ~/.claude-paladin` or the Docker daemon creates it root-owned (D-06).

In the same service's `environment:` list, insert `- CLAUDE_CONFIG_DIR=/home/vscode/.claude`
immediately after the `- MINIO_SECRET_KEY=minioadmin` line, so the contiguous block of bare
LLM-key passthrough names at the end of the list stays intact. Comment it with the reason from
D-04: `.claude.json` defaults to `$HOME/.claude.json`, outside the `.claude` dir, and Claude
Code rewrites it via atomic rename — so a single-file bind mount or a symlink would be replaced
by a plain file and persistence would break silently. Setting this variable keeps the file
inside the mounted directory instead. Record in the comment that this was verified empirically
against Claude Code 2.1.239 (verified fact 2), and that the value must stay identical to the
mount target above.

Do not add or change any other mount, service, volume, or variable. Do not touch
`/workspace/.claude` anywhere (D-05).
  </action>
  <verify>
    <automated>cd /workspace &amp;&amp; python3 -c 'import yaml;s=yaml.safe_load(open(".devcontainer/docker-compose.yml"))["services"]["paladin-dev"];assert "${HOME}/.claude-paladin:/home/vscode/.claude" in s["volumes"], s["volumes"];assert "CLAUDE_CONFIG_DIR=/home/vscode/.claude" in s["environment"], s["environment"];assert "${HOME}/.config/paladin:/home/vscode/.config/paladin:ro" in s["volumes"], "credentials mount was disturbed";print("compose OK")' &amp;&amp; pre-commit run check-yaml --files .devcontainer/docker-compose.yml</automated>
  </verify>
  <done>`docker-compose.yml` parses as YAML, declares the read-write mount `${HOME}/.claude-paladin:/home/vscode/.claude` (no `:ro`), sets `CLAUDE_CONFIG_DIR` to that same container path, and leaves the pre-existing read-only credentials mount untouched.</done>
  <reversibility rating="reversible">Two lines in a compose file; deleting them restores today's behaviour, and the host dir can simply be left in place or removed.</reversibility>
</task>

<task type="auto">
  <name>Task 2: Guard the mount in post-start.sh with an actionable failure message</name>
  <files>.devcontainer/post-start.sh</files>
  <action>
Docker silently creates a missing bind-mount source as a ROOT-OWNED directory, after which the
`vscode` user cannot write to it and Claude Code loses state without any visible error. Detect
that and say what to do about it (D-06).

Add `YELLOW='\033[1;33m'` and `RED='\033[0;31m'` to the existing colors block. Both MUST be
referenced by the code you add — shellcheck reports an unused variable at warning severity and
the pre-commit gate runs at `--severity=warning`, so an unused color declaration fails the build.

Insert the guard immediately after the existing `~/.config/paladin` credential-mount
`if`/`else` block and before the final `echo -e "${GREEN}✨ Ready to code!${NC}"`. Begin it by
resolving the path into a variable that defaults to the mount point but can be overridden, so
the three branches are testable without a rebuild: assign `CLAUDE_STATE_DIR` from
`${CLAUDE_STATE_DIR:-/home/vscode/.claude}`.

Three branches, matching the reporting style of the credential block above (two-space indent,
emoji status marker):

Branch one, directory absent: a YELLOW warning that there is no Claude Code state mount at that
path and that sessions and login will NOT survive a rebuild, followed by the host-side fix
`mkdir -p ~/.claude-paladin && chmod 700 ~/.claude-paladin` and an instruction to run
Dev Containers: Rebuild Container.

Branch two, directory present but `[ -w ]` is false: a RED failure line naming the path and its
current owner from `stat -c '%U'`, an explanation that Docker creates a missing bind-mount source
root-owned, and the host-side fix `sudo chown -R "$(id -u):$(id -g)" ~/.claude-paladin`. That fix
line must be emitted from a SINGLE-quoted `echo` so the command substitutions are printed
literally rather than evaluated in the container.

Branch three, writable: a green-check line reporting the mount is active and how many session
transcripts exist for this workspace. Count them with `find "$CLAUDE_STATE_DIR/projects/-workspace"
-maxdepth 1 -name '*.jsonl' 2>/dev/null | wc -l` — use `find`, not `ls`, and keep the redirect so
a first-run absent directory yields zero rather than noise. Then, only when
`$CLAUDE_STATE_DIR/.claude.json` does not exist, print a follow-up line telling the user to
authenticate Claude Code once, after which the login persists across rebuilds; write it as a
`[ -f ... ] || echo ...` so it cannot trip the script's `set -e`.

The script runs under `set -e` — every added command must be inside an `if` condition, a `||`
fallback, or a pipeline whose last element cannot fail. Quote every variable expansion.
  </action>
  <verify>
    <automated>cd /workspace &amp;&amp; pre-commit run shellcheck --files .devcontainer/post-start.sh &amp;&amp; bash -n .devcontainer/post-start.sh &amp;&amp; CLAUDE_STATE_DIR=/nonexistent-probe-dir bash .devcontainer/post-start.sh 2>&amp;1 | grep -q 'mkdir -p ~/.claude-paladin' &amp;&amp; mkdir -p /tmp/probe-claude-ro &amp;&amp; chmod 500 /tmp/probe-claude-ro &amp;&amp; CLAUDE_STATE_DIR=/tmp/probe-claude-ro bash .devcontainer/post-start.sh 2>&amp;1 | grep -qi 'not writable' &amp;&amp; chmod 700 /tmp/probe-claude-ro &amp;&amp; rmdir /tmp/probe-claude-ro &amp;&amp; CLAUDE_STATE_DIR=/home/vscode/.claude bash .devcontainer/post-start.sh 2>&amp;1 | grep -q 'Claude Code state mount active' &amp;&amp; echo "guard OK"</automated>
  </verify>
  <done>`shellcheck --severity=warning` is clean, and all three branches are exercised: an absent dir prints the `mkdir` fix, a non-writable dir prints the ownership failure, and the real `/home/vscode/.claude` prints the active-mount line with a transcript count.</done>
</task>

<task type="auto">
  <name>Task 3: Document the mount in .devcontainer/README.md and FILES.md</name>
  <files>.devcontainer/README.md, .devcontainer/FILES.md</files>
  <action>
Mirror the structure and voice of the existing "LLM API credentials" section — a why, a
one-time host setup block, a precedence/mechanism explanation, and a checking step.

In `README.md`, add a `## Claude Code session persistence` section after the "LLM API
credentials" section and before `## Quick Start`. It must cover:

The problem, stated in the same terms the credentials section already uses: `/home/vscode` is
part of the container filesystem and is destroyed on every rebuild, and Claude Code keeps all of
its user state there — session transcripts under `.claude/projects/<escaped-cwd>/`, todos, shell
snapshots, user settings, and the `.credentials.json` auth token.

The one-time host setup, as a shell block: `mkdir -p ~/.claude-paladin` and
`chmod 700 ~/.claude-paladin`, run on the HOST before the container is (re)built. State plainly
that creating it first is required, because the Docker daemon otherwise creates it root-owned and
the container user cannot write to it; `post-start.sh` reports both failure modes with the fix.

The mechanism: `docker-compose.yml` bind-mounts the host's `~/.claude-paladin` READ-WRITE at
`/home/vscode/.claude` (read-write, unlike the read-only credentials mount, because Claude Code
writes here), and sets `CLAUDE_CONFIG_DIR=/home/vscode/.claude` so that `.claude.json` — which
by default sits at `$HOME/.claude.json`, outside `.claude/`, and is rewritten by atomic rename —
is kept inside the mounted directory too. Note that a single-file mount or a symlink would not
work: the rename replaces it with a plain file and persistence breaks silently. Note that this
was verified against Claude Code 2.1.239.

Why a dedicated directory rather than the host's real `~/.claude`: so a Claude Code session
running on the host cannot race or conflict with the container's (D-01).

How to change the host path: edit the mount line in `.devcontainer/docker-compose.yml`. There is
deliberately no environment-variable override, matching the credentials mount.

First run: you must authenticate Claude Code once after the mount is in place; the login then
persists across rebuilds because `.credentials.json` lives inside the mount.

Session continuity: transcripts are filed under `projects/-workspace/`, a key derived from the
`workspaceFolder` `/workspace` pinned in `devcontainer.json`. That path is stable across
rebuilds, so `claude --continue` and `claude --resume` still find prior sessions afterwards.

An explicit scope statement, honest about the limits: this persists Claude Code session
transcripts, history, todos, shell snapshots, user settings and auth — it does NOT persist the
container filesystem generally, so anything else written under `/home/vscode` is still lost on
rebuild. Add a short note that `/workspace/.claude/` is a different thing entirely — the
project-local GSD install — already persisted by the workspace bind mount and untouched by this
change (D-05).

In `FILES.md`, make two scoped edits. Under the `### .devcontainer/docker-compose.yml` entry,
extend the feature list so it names BOTH host bind mounts — the read-only `~/.config/paladin`
credentials mount and the read-write `~/.claude-paladin` Claude Code state mount — and mentions
the `CLAUDE_CONFIG_DIR` variable. Under the `### .devcontainer/post-start.sh` entry, add the
Claude Code state mount check to its "Actions" list alongside the existing items. Leave the
stated file sizes alone unless they are already wrong; they are decorative.

Do not restructure or reflow sections you are not adding to.
  </action>
  <verify>
    <automated>cd /workspace &amp;&amp; grep -q 'Claude Code session persistence' .devcontainer/README.md &amp;&amp; grep -q 'claude-paladin' .devcontainer/README.md &amp;&amp; grep -q 'CLAUDE_CONFIG_DIR' .devcontainer/README.md &amp;&amp; grep -q 'workspaceFolder\|projects/-workspace' .devcontainer/README.md &amp;&amp; grep -q 'claude-paladin' .devcontainer/FILES.md &amp;&amp; grep -q 'CLAUDE_CONFIG_DIR' .devcontainer/FILES.md &amp;&amp; grep -q 'gemini_api_key' .devcontainer/README.md &amp;&amp; pre-commit run trailing-whitespace --files .devcontainer/README.md .devcontainer/FILES.md &amp;&amp; pre-commit run end-of-file-fixer --files .devcontainer/README.md .devcontainer/FILES.md &amp;&amp; echo "docs OK"</automated>
  </verify>
  <done>`README.md` has a "Claude Code session persistence" section covering the host setup command, the read-write mount, `CLAUDE_CONFIG_DIR`, the dedicated-directory rationale, the one-time authentication, the stable `projects/-workspace` key, and an explicit scope limit; `FILES.md` describes both bind mounts and the new post-start check; the pre-existing credentials documentation is intact; whitespace hooks pass.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| host filesystem → container | A read-write bind mount is the first writable host path exposed to this container; the existing credentials mount is read-only. |
| container → host filesystem | Anything running as `vscode` in the container can now write into `~/.claude-paladin` on the host. |

## STRIDE Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation Plan |
|-----------|----------|-----------|----------|-------------|-----------------|
| T-gpf-01 | Information disclosure | `~/.claude-paladin/.credentials.json` on the host | medium | mitigate | The Claude Code OAuth token is now persisted on host disk. Documented `chmod 700 ~/.claude-paladin` in the one-time host setup (Task 3), matching the `chmod 700 ~/.config/paladin` precedent, so the directory is not group- or world-readable. |
| T-gpf-02 | Tampering | read-write host bind mount | medium | accept | A read-write mount is inherent to the requirement — Claude Code must write its state. Blast radius is bounded to one dedicated directory, which is precisely why D-01 forbids mounting the host's real `~/.claude`. |
| T-gpf-03 | Denial of service | root-owned bind-mount source | medium | mitigate | Task 2 guard detects an absent or non-writable mount at every container start and prints the exact host-side `mkdir`/`chown` fix, converting a silent state-loss failure into a visible, actionable one. |
| T-gpf-04 | Information disclosure | committed docs and config | low | mitigate | No secret values are introduced; the `gitleaks` pre-commit hook covers the changed files. Only paths and a variable name are committed. |
| T-gpf-05 | Elevation of privilege | `sudo chown` guidance in Task 2 | low | accept | The remediation text is printed for the user to run on the HOST against a path under their own `$HOME`; the container never executes it and no `sudo` is added to any script. |

No package-manager installs are introduced by this plan, so the supply-chain legitimacy gate
(`T-*-SC`) does not apply.
</threat_model>

<verification>
Automated, runnable inside the current container:

1. `cd /workspace && python3 -c 'import yaml;yaml.safe_load(open(".devcontainer/docker-compose.yml"))'` — compose still parses.
2. `make lint-shell` — shellcheck clean across all tracked `*.sh`, matching the pre-commit gate.
3. The three-branch guard exercise in Task 2's `<automated>` block.
4. `grep -c 'gemini_api_key' .devcontainer/README.md` returns non-zero — the pre-existing
   credentials documentation was not clobbered.
5. `test -d /workspace/.claude/gsd-core/bin` — the project-local GSD install is untouched (D-05).

Human-only (requires a rebuild, cannot be automated from inside the running container):

- On the HOST: `mkdir -p ~/.claude-paladin && chmod 700 ~/.claude-paladin`.
- Rebuild the container. Confirm `post-start.sh` prints the active-mount line.
- Authenticate Claude Code once. Confirm `/home/vscode/.claude/.claude.json` appears (proving
  `CLAUDE_CONFIG_DIR` took effect) and that `/home/vscode/.claude.json` does NOT reappear.
- Start a session, then rebuild again. Confirm no re-login is required and that
  `claude --resume` lists the pre-rebuild session, with its transcript still present under
  `/home/vscode/.claude/projects/-workspace/`.
- Confirm `ls -la ~/.claude-paladin` on the host shows the files owned by the host user.
</verification>

<success_criteria>
- `docker-compose.yml` declares a read-write `${HOME}/.claude-paladin:/home/vscode/.claude` mount and `CLAUDE_CONFIG_DIR=/home/vscode/.claude`, with the paths identical.
- The read-only `~/.config/paladin` credentials mount and every other service definition are unchanged.
- `post-start.sh` reports the mount as active, absent, or non-writable, with a host-side fix in the two failure branches, and passes `shellcheck --severity=warning`.
- `README.md` and `FILES.md` describe the mount, the one-time host setup, the one-time authentication, and the honest scope limit.
- `/workspace/.claude/` is not modified.
- No Rust code, no `Cargo.toml`, and no test file is touched.
</success_criteria>

<output>
Create `.planning/quick/260822-gpf-persist-claude-code-sessions-across-devc/260822-gpf-SUMMARY.md` when done.
</output>
