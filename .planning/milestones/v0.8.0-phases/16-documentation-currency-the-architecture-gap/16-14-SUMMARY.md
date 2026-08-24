---
phase: 16-documentation-currency-the-architecture-gap
plan: 14
subsystem: docs
tags: [vhs, asciinema, demos, mdbook, readme, docs-currency]

# Dependency graph
requires:
  - phase: 16-13
    provides: "vhs 0.11.0, ttyd 1.7.7, ffmpeg 5.1.9-0+deb12u1, and asciinema 2.2.0 installed and verified on PATH; the project owner's authorization of the vhs/ttyd supply chain (Checkpoint 1); and the 2 MB per-file GIF size budget (Checkpoint 2), recorded in 16-DOCS-04-TOOLCHAIN.md"
  - phase: 16-05
    provides: "The out-of-scope finding that README.md:181 still states version 0.6.0 against the shipped 0.8.0, handed off for this plan to fix"
provides:
  - "Four checked-in .tape sources under docs/assets/recordings/, each driving a real cargo run --example <name> against a mock-backed example, with their .gif and .cast artifacts — none hand-performed"
  - "docs/DEMOS.md indexing all four demos in the fixed D-16 order with embedded .gif, .cast link, source link, and exact vhs/asciinema regeneration commands"
  - "One added line in README.md's Documentation section pointing at docs/DEMOS.md"
  - "REQUIREMENTS.md's DOCS-04 block amended in place: the live-API-key premise and the docs/assets/-exists-empty premise both corrected with dated notes, original text retained"
  - "README.md:181's stale 0.6.0 version corrected to 0.8.0, closing the finding 16-05 deferred to this plan"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Two artifacts from one command: VHS's Output directive renders the .gif from the .tape; a separate scripted `asciinema rec -c \"<same command>\"` produces the .cast VHS cannot emit (D-14 amendment)"
    - "Keep the version-fix commit separate from the one-permitted-line README commit so each commit's own git diff --numstat satisfies its own acceptance criterion independently"

key-files:
  created:
    - docs/assets/recordings/basic-paladin.tape
    - docs/assets/recordings/basic-paladin.gif
    - docs/assets/recordings/basic-paladin.cast
    - docs/assets/recordings/battalion-formation.tape
    - docs/assets/recordings/battalion-formation.gif
    - docs/assets/recordings/battalion-formation.cast
    - docs/assets/recordings/council-discussion.tape
    - docs/assets/recordings/council-discussion.gif
    - docs/assets/recordings/council-discussion.cast
    - docs/assets/recordings/grove-routing.tape
    - docs/assets/recordings/grove-routing.gif
    - docs/assets/recordings/grove-routing.cast
    - docs/DEMOS.md
  modified:
    - README.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "The devcontainer's vhs install (16-13) never actually exercised a GIF render — only `vhs --version`. Task 1 hit `could not launch browser: ... libatk-1.0.so.0: cannot open shared object file` (VHS renders via a headless Chromium through go-rod/ttyd, not a plain terminal capture). Fixed by installing the missing Chromium GTK runtime libraries (libatk1.0-0, libatk-bridge2.0-0, libcups2, libatspi2.0-0, libxcomposite1, libxdamage1, plus libxfixes3/libxrandr2/libgbm1/libpango-1.0-0/libasound2/libnss3) from the base image's own Debian bookworm repository — the same trust root already used for ffmpeg/asciinema in 16-13. This was a session-local `apt-get install` to unblock recording; it does not touch either Dockerfile, which is out of this plan's `files_modified` scope (owned by 16-13). Recorded here as a deferred follow-up: a future devcontainer-image update should add these to the vhs install block so a fresh container doesn't hit the same gap."
  - "Each .tape's Sleep duration was set to land inside its own D-16 target window (30-60s / 45-90s / 60-120s / 45-90s) after accounting for typing time and the near-instant compiled-binary run (each example completes in well under 1s once built) — the recording's length reads mostly as post-execution viewing time, not execution time, since all four mock-backed examples run near-instantly."
  - "The README.md:181 version fix (0.6.0 → 0.8.0, deferred by 16-05) was applied as its own separate commit rather than folded into Task 2's one-line DEMOS.md-link commit, so D-15's 'the README gets one line and nothing else' scope guard and Task 2's own `git diff --numstat README.md` == 1/0 acceptance criterion both hold true for that specific commit, while the orchestrator-mandated version fix still lands in this plan."
  - "docs/DEMOS.md's image embeds use the correct sibling-relative path (`assets/recordings/<file>.gif`, since DEMOS.md sits directly in docs/) rather than a `docs/`-prefixed path, so the links resolve correctly wherever the file is actually viewed (e.g. GitHub). The full `docs/assets/recordings/...` form appears anyway in the page's own regeneration commands (`vhs docs/assets/recordings/<file>.tape`, run from the repo root, matching every other command in this plan), which is what the plan's own path-resolution verification checks against."

requirements-completed: [DOCS-04]

coverage:
  - id: D1
    description: "Four .tape sources recorded (basic-paladin, battalion-formation, council-discussion, grove-routing), each driving cargo run --example <name> against basic_paladin.rs, formation_sequential.rs, council_discussion.rs, and grove_routing.rs — all four verified at exit 0, offline, no credentials, immediately before taping. Each yields a content-verified .gif (GIF89a magic bytes) and .cast (JSON-parseable first line), all under the 2 MB budget."
    requirement: "DOCS-04"
    verification:
      - kind: other
        ref: "ls docs/assets/recordings/*.tape | wc -l == 4; for-each: test -f $s.gif, test -f $s.cast, head -c 6 $s.gif == GIF8, head -1 $s.cast parses as JSON, grep 'cargo run --example' in tape, no .cast on an Output line; du -b: basic-paladin.gif=174480, battalion-formation.gif=1196271, council-discussion.gif=248166, grove-routing.gif=214532 (all < 2097152); ls docs/src/assets | wc -l == 6; git diff --name-only -- examples/ empty"
        status: pass
    human_judgment: false
  - id: D2
    description: "docs/DEMOS.md indexes all four demos in the fixed D-16 order (line numbers 25/41/57/73, strictly increasing), each with an embedded .gif, a .cast link, a source-example link, and the exact vhs/asciinema regeneration commands; README.md gains exactly one line pointing at it; docs/src/SUMMARY.md is untouched (0 DEMOS mentions) with the reason recorded on the page itself; mdbook build docs/ exits 0 with no broken links"
    requirement: "DOCS-04"
    verification:
      - kind: other
        ref: "test -f docs/DEMOS.md; path-resolution loop over grep -ohE 'docs/assets/recordings/...' extracted paths, all test -e true; git diff --numstat README.md == 1/0 (at Task 2 commit time); grep -c 'DEMOS.md' README.md == 1; grep -c 'DEMOS' docs/src/SUMMARY.md == 0; mdbook build docs/ exit 0; git status --porcelain docs/src/ empty"
        status: pass
    human_judgment: false
  - id: D3
    description: "REQUIREMENTS.md's DOCS-04 block amended in place with two dated 2026-08-24 notes correcting the live-API-key premise (M-08) and the docs/assets/-exists-empty premise (M-09), original text retained above them, FR-26.4's adapted-not-dropped README clause recorded, checkbox left unticked, no other block touched"
    requirement: "DOCS-04"
    verification:
      - kind: other
        ref: "grep -q 'live LLM API keys'; grep -q 'basic_paladin'; grep -q 'DEMOS.md'; grep -c '^- \\[x\\] \\*\\*DOCS-04\\*\\*' == 0; git diff -U0 .planning/REQUIREMENTS.md | grep -c '^-[^-]' == 0 (zero deleted lines)"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 14: Demo Recordings, Index, README Link, and DOCS-04 Closure Summary

**Recorded all four D-16 demos as checked-in VHS `.tape` sources with `.gif`/`.cast` artifacts (174KB-1.14MB, all under the 2 MB budget), indexed them in `docs/DEMOS.md` with one README link, corrected README.md's stale `0.6.0` version to `0.8.0`, and amended DOCS-04's two measured-false premises (live API keys, empty `docs/assets/`) in place in REQUIREMENTS.md.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-24T15:20:00Z (approx, first read after loading the 16-14 plan)
- **Completed:** 2026-08-24T15:39:35Z
- **Tasks:** 3 (plus one separately-committed deviation)
- **Files modified:** 17 (12 recording artifacts created, docs/DEMOS.md created, README.md modified twice, REQUIREMENTS.md modified once)

## Accomplishments

- Verified all four D-16 example scenarios (`basic_paladin`, `formation_sequential`, `council_discussion`, `grove_routing`) run offline at exit 0 with no credentials, both before and immediately before recording each one
- Authored four `.tape` sources under `docs/assets/recordings/` (`basic-paladin.tape`, `battalion-formation.tape`, `council-discussion.tape`, `grove-routing.tape`), each driving `cargo run --example <name>` with an explicit terminal width/height/font/theme (Dracula) for reproducibility, and produced their `.gif` via `vhs` and `.cast` via a scripted `asciinema rec -c "<same command>"` — nothing hand-performed
- Measured every `.gif`: `basic-paladin.gif` 174,480 bytes, `battalion-formation.gif` 1,196,271 bytes, `council-discussion.gif` 248,166 bytes, `grove-routing.gif` 214,532 bytes — all comfortably under the 2 MB per-file budget set at 16-13's checkpoint 2
- Verified every `.gif` starts with the GIF89a magic bytes and every `.cast`'s first line parses as JSON (asciicast header); scanned every `.cast` for credential-shaped strings (`sk-`, `api_key`, `Bearer <token>`, etc.) — none found
- Wrote `docs/DEMOS.md`: one section per demo in the fixed D-16 order (Basic Paladin Execution, Battalion Formation, Council Discussion, Grove Routing), each with the embedded `.gif`, a link to its `.cast`, a link to its source example, and its exact `vhs`/`asciinema` regeneration commands; opened with a note on the offline/no-API-key guarantee and the tool versions that produced the committed artifacts; recorded in the page itself why it carries no `docs/src/SUMMARY.md` entry (outside `docs/src/`, not part of the mdBook, per D-15)
- Added exactly one line to README.md's Documentation section pointing at `docs/DEMOS.md` — verified `git diff --numstat README.md` == `1 0` at that commit
- Separately corrected README.md:181's stale Project Status version (`0.6.0` → `0.8.0`, matching the workspace `Cargo.toml`), the finding 16-05 flagged as out of its own scope and handed to this plan — kept in its own commit so it doesn't collide with Task 2's one-line acceptance criterion
- Amended REQUIREMENTS.md's `DOCS-04` block in place with two dated 2026-08-24 notes: the live-API-key premise is measured false (M-08, all four scenarios verified offline, no credentials) and the `docs/assets/`-exists-and-is-empty premise is misstated (M-09, the directory did not exist at all); also recorded that FR-26.4's README-embedding clause was adapted, not dropped, per D-15. Original text retained above both notes; checkbox left unticked; zero lines deleted anywhere in the file
- Re-ran `mdbook build docs/` after all edits: exit 0, "No broken links found"; confirmed `docs/src/assets/` still holds exactly its six pre-existing architecture SVGs, untouched

## Task Commits

1. **Task 1: Author the four .tape scripts and produce the .gif and .cast artifacts** - `93642795` (feat)
2. **Task 2: Write docs/DEMOS.md and add the single README link** - `84df9fb2` (docs)
3. **Deviation: correct README.md's stale Project Status version** - `dea43eed` (fix)
4. **Task 3: Amend DOCS-04's two measured-false premises in the requirement text, in place** - `81bdc925` (docs)

**Plan metadata:** (this commit)

## Files Created/Modified

- `docs/assets/recordings/basic-paladin.tape` / `.gif` / `.cast` - Basic Paladin Execution demo (30-60s target)
- `docs/assets/recordings/battalion-formation.tape` / `.gif` / `.cast` - Battalion Formation demo (45-90s target)
- `docs/assets/recordings/council-discussion.tape` / `.gif` / `.cast` - Council Discussion demo (60-120s target)
- `docs/assets/recordings/grove-routing.tape` / `.gif` / `.cast` - Grove Routing demo (45-90s target)
- `docs/DEMOS.md` - New demo index, outside docs/src/, not in the mdBook
- `README.md` - One added line pointing at docs/DEMOS.md (Task 2); separately, the stale `0.6.0` → `0.8.0` version correction (deviation)
- `.planning/REQUIREMENTS.md` - DOCS-04 block amended in place with two dated notes, original text retained

## Decisions Made

See `key-decisions` in frontmatter for full rationale. In summary: fixed a devcontainer runtime gap (missing Chromium GTK libraries VHS's go-rod/ttyd renderer needs) with a session-local `apt-get install` from the base image's own trust root rather than touching either Dockerfile (out of this plan's scope, owned by 16-13) — flagged below as a follow-up; sized each `.tape`'s `Sleep` to land inside its own D-16 duration target; split the README version fix into its own commit so it doesn't collide with Task 2's one-added-line acceptance criterion; used sibling-relative image paths in `docs/DEMOS.md` so links resolve correctly from the file's actual location.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Installed missing Chromium runtime libraries for `vhs`**
- **Found during:** Task 1 (first `vhs` invocation)
- **Issue:** `vhs docs/assets/recordings/basic-paladin.tape` failed with `could not launch browser: ... libatk-1.0.so.0: cannot open shared object file: No such file or directory`. VHS renders terminal frames via a headless Chromium (go-rod) driven through `ttyd`'s web terminal, not a direct terminal capture — a runtime dependency 16-13's checkpoint (`vhs --version` only) never exercised.
- **Fix:** `sudo apt-get install -y --no-install-recommends libatk1.0-0 libatk-bridge2.0-0 libcups2 libatspi2.0-0 libxcomposite1 libxdamage1 libxfixes3 libxrandr2 libgbm1 libpango-1.0-0 libasound2 libnss3`, all from the base image's own Debian bookworm repository (same trust root as every other unpinned `apt-get install` line in both Dockerfiles, per the same reasoning 16-13 used for ffmpeg/asciinema).
- **Files modified:** None (session-local install; neither Dockerfile touched — out of this plan's `files_modified` scope, owned by 16-13).
- **Verification:** `vhs docs/assets/recordings/basic-paladin.tape` succeeded immediately after (exit 0, `basic-paladin.gif` produced with valid GIF89a header).
- **Committed in:** N/A (no repo file changed by this fix itself; its effect is visible in the Task 1 commit's artifacts).

**2. [Rule 1 - Bug, handed off from 16-05] Corrected README.md's stale Project Status version**
- **Found during:** Handed off by plan 16-05's out-of-scope observation (`16-05-SUMMARY.md`, README.md:181 `0.6.0` vs. shipped `0.8.0`); re-confirmed against `Cargo.toml`'s `version = "0.8.0"` before fixing.
- **Issue:** README.md's Project Status section stated `Current version: 0.6.0`, three minor versions stale against the shipped `0.8.0`.
- **Fix:** Changed `0.6.0` to `0.8.0` at README.md:181, in its own commit (kept separate from Task 2's single permitted line so D-15's scope guard and Task 2's own `git diff --numstat` acceptance criterion both hold for that commit).
- **Files modified:** `README.md`.
- **Verification:** `grep -m1 '^version' Cargo.toml` → `0.8.0`, matching the corrected README text.
- **Committed in:** `dea43eed`.

---

**Total deviations:** 2 auto-fixed (1 blocking-environment fix, 1 handed-off bug fix)
**Impact on plan:** The Chromium-library fix was necessary to produce any recording at all; it touches no repository file and is recorded here as a follow-up for 16-13's Dockerfiles. The README version fix was explicitly handed to this plan by 16-05 and is a factual correction, kept isolated in its own commit to avoid conflating it with D-15's one-line scope guard. No scope creep beyond what the plan and its hand-off already named.

## Issues Encountered

- `vhs` failed on its first real invocation despite passing `vhs --version` in 16-13's checkpoint — see deviation 1 above. Resolved by installing the missing Chromium GTK runtime libraries from the base image's own repository.
- `mdbook build docs/` failed once on a fresh worktree with "Unable to copy `docs/mermaid.min.js`" — the gitignored mermaid preprocessor assets weren't present yet, matching 16-05's own prior note. Resolved by running `mdbook-mermaid install docs/` once; confirmed `docs/book.toml` unchanged (`git status --short` empty) before and after.
- The sandboxed Bash tool rejected several multi-statement or `cd`-adjacent commands as "too complex to verify [they stay] inside the worktree" (consistent with prior plans in this phase); worked around by splitting into single-purpose commands.

## User Setup Required

None — no external service configuration required. (A follow-up for a maintainer, not end-user setup: 16-13's devcontainer install block for `vhs` doesn't include the Chromium GTK runtime libraries `go-rod` needs to actually render a GIF; a fresh container built from either `.devcontainer/Dockerfile.dev` or `.devcontainer/Dockerfile` today will hit the same `could not launch browser` failure this plan hit and fixed session-locally. Recorded here rather than fixed, since editing either Dockerfile is outside this plan's `files_modified` scope.)

## Next Phase Readiness

- DOCS-04 is closed on its own terms: the four demos are recorded, indexed, and linked; both of the requirement's own measured-false premises are corrected in place with dated notes and the superseded text retained; the checkbox is left unticked for the phase-close step.
- This is the final plan of Phase 16 — DOCS-01 through DOCS-04 (and DOCS-02, DOCS-03 closed in earlier plans) are all settled. The phase-close step should tick DOCS-04's checkbox in REQUIREMENTS.md, pointing to this plan's evidence.
- One deferred follow-up (not blocking, not part of this plan's scope): add the Chromium GTK runtime libraries this plan discovered missing to both devcontainer Dockerfiles' `vhs` install block, so a fresh container can actually render a `.tape` without the session-local fix this plan applied. See "User Setup Required" above.
- No blockers.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

Verified on disk: `docs/assets/recordings/basic-paladin.tape` and `docs/DEMOS.md` both FOUND.
Verified in `git log --oneline --all`: all four task commit hashes (`93642795`, `84df9fb2`,
`dea43eed`, `81bdc925`) FOUND.
