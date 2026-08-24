---
phase: 09-release-security-gate-integrity
plan: 04
subsystem: infra
tags: [crates-io, ci, guard-script, python3-tomllib, cargo-deny, allow-list]

# Dependency graph
requires:
  - phase: 09-release-security-gate-integrity
    provides: "plan 09-01's proven failable-guard idiom (bash if/else, single exit 0, python3+tomllib heredoc) and its cargo-deny job placement pattern"
provides:
  - ".crate-names.txt — the committed allow-list of the eleven package names this project owns on crates.io"
  - "scripts/check-crate-names.sh — a bidirectional, demonstrably-failable set-equality guard over workspace [package] names (SEC-03/D-13)"
  - "Makefile check-crate-names target"
  - "A CI step ('Check crates.io package names') inside the cargo-deny job, required status-check context 'License & Dependency Policy'"
  - "ADR-0026 recording the offline-guard decision and its accepted residual cost"
affects: [09-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reused plan 09-01's failable-guard idiom verbatim: python3 heredoc computes and reports a status line plus detail, bash-level if/else performs the single exit 0 / exit 1."
    - "Bidirectional set-equality check (tree names vs allow-list, both directions) as the pattern for any future hand-maintained allow-list guard in this project — a one-directional check passes vacuously against the exact failure it exists to catch."

key-files:
  created:
    - .crate-names.txt
    - scripts/check-crate-names.sh
    - .planning/decisions/0026-crate-name-collision-guard.md
  modified:
    - Makefile
    - .github/workflows/ci.yml

key-decisions:
  - "Guard reads [package].name via tomllib, never grep — the root manifest carries [[bin]]/[[test]]/[[bench]] tables with their own name keys, and paladin-core's [lib] name (paladin_core) differs from its [package] name (paladin-ai-core)."
  - "Comparison is exact string equality: no case-folding, no hyphen/underscore normalisation. Proven by flipping one allow-list entry to a case-only variant (Paladin-Web) and observing both directions fail simultaneously."
  - "Exemption for paladin-doc-examples follows [package].publish = false, not the directory name — proven by temporarily flipping the field to true and observing the guard start naming paladin-doc-examples, then restoring."
  - "CI placement: appended immediately after plan 09-01's 'Check per-crate changelogs' step, inside the existing cargo-deny job, per the plan's explicit instruction not to create a new job or touch any dtolnay/actions-cache reference."
  - "ADR-0026 states the residual cost explicitly in both ## Decision and ## Downstream Consumers: the eleven existing names are already owned (zero collision risk); a genuinely novel name is still checked by a human against crates.io, not by CI."

patterns-established:
  - "Any future hand-maintained allow-list guard in this project should assert set equality in both directions and prove each direction's failure mode separately, matching this plan's and plan 09-01's negative-path evidence style."

requirements-completed: [SEC-03]

coverage:
  - id: D1
    description: ".crate-names.txt created — the committed allow-list of the eleven package names this project owns on crates.io"
    requirement: "SEC-03"
    verification:
      - kind: other
        ref: "grep -vc '^\\s*#' .crate-names.txt (=11); names match the tree's [package].name values exactly"
        status: pass
    human_judgment: false
  - id: D2
    description: "scripts/check-crate-names.sh — bidirectional set-equality guard, exact-comparison, publish=false exemption by field, demonstrably failable both directions"
    requirement: "SEC-03"
    verification:
      - kind: other
        ref: "./scripts/check-crate-names.sh (clean tree, exit 0) and the six negative-path invocations recorded below (all exit 1)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Makefile check-crate-names target wrapping the guard"
    verification:
      - kind: other
        ref: "make -n check-crate-names"
        status: pass
    human_judgment: false
  - id: D4
    description: "CI step 'Check crates.io package names' wired into the cargo-deny job (required status-check context 'License & Dependency Policy'), immediately after plan 09-01's changelog step"
    verification:
      - kind: other
        ref: "grep -c check-crate-names.sh .github/workflows/ci.yml (=1); python assertion confirms the step is inside the cargo-deny job block with exactly one 'Make scripts executable' step"
        status: pass
    human_judgment: true
    rationale: "The step's placement and invocation are verified structurally (grep + python block-boundary check + manual read); the step has not been executed by an actual GitHub Actions runner within this sandboxed worktree, so a human should confirm the next real CI run on this branch exercises it as expected."
  - id: D5
    description: "ADR-0026 recorded: offline-guard decision, rejected live-query and dry-run-only alternatives with concrete reasons (HTTP 403, main-branch/release-time-only detection), verbatim failing transcript, and the accepted residual cost"
    requirement: "SEC-03"
    verification:
      - kind: other
        ref: ".planning/decisions/0026-crate-name-collision-guard.md — seven-heading shape matches ADR-0022/0023, no frontmatter, 3 (rejected) tags, 1 '403' reference, 1 'residual cost' reference"
        status: pass
    human_judgment: false
  - id: D6
    description: "All six required guard failure modes demonstrated non-zero (unlisted name, stale entry, emptied allow-list, missing allow-list, case-only variant, publish-field exemption flip) plus the reordering pass-through, with exact commands and exit codes recorded verbatim; tree restored byte-identical after each"
    requirement: "SEC-03"
    verification:
      - kind: other
        ref: "See 'Negative-path evidence' section below for all transcripts"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 04: crates.io name-collision guard Summary

**Added an offline, bidirectionally-failable crates.io package-name allow-list guard (`.crate-names.txt` + `scripts/check-crate-names.sh`), wired it into a required CI context, and recorded ADR-0026 with the accepted residual cost.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-07T23:56:00Z (approx.)
- **Completed:** 2026-08-08T00:21:00Z
- **Tasks:** 3
- **Files modified:** 5 (3 created, 2 modified)

## Accomplishments

- Created `.crate-names.txt`, the committed, hand-edited allow-list of the eleven package names this project owns on crates.io (`paladin-ai`, `paladin-ai-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`), excluding `paladin-doc-examples` (`publish = false`).
- Wrote `scripts/check-crate-names.sh`, copying plan 09-01's proven failable-guard idiom: a `python3`+`tomllib` heredoc that reads `[package].name` from the root manifest and every `crates/*/Cargo.toml` (never grepping, since `[[bin]]`/`[[test]]`/`[[bench]]` tables carry their own `name` keys), excludes `publish = false` manifests, and asserts **set equality in both directions** against the allow-list — an unlisted tree name and a stale allow-list entry each produce a distinct failure message. Comparison is exact string equality; a zero-crates-discovered case is its own distinct failure. Closes with a single bash-level `exit 0`/`exit 1` conditional (`grep -c 'exit 0'` → `1`).
- Added a `Makefile` `check-crate-names` target immediately after `check-changelogs`, and a CI step `Check crates.io package names` inside the `cargo-deny` job (`License & Dependency Policy`, a required status-check context), immediately after plan 09-01's `Check per-crate changelogs` step — no other step, toolchain reference, or job in either file was touched.
- Drove the guard through six negative-path demonstrations plus one reordering pass-through, all reverting the tree to a byte-identical state after each, and recorded every transcript verbatim (below).
- Wrote `.planning/decisions/0026-crate-name-collision-guard.md` in ADR-0022/0023's exact seven-heading, no-frontmatter shape, recording the offline-guard decision, three rejected alternatives with concrete reasons, the verbatim failing transcript, and the accepted residual cost stated in both `## Decision` and `## Downstream Consumers`.

## Task Commits

1. **Task 1: Author the allow-list and the bidirectional name guard** - `264721a` (feat)
2. **Task 2: Wire the name guard into make and CI** - `2758a9d` (feat)
3. **Task 3: Write ADR-0026 — crates.io name-collision guard** - `5cde208` (docs)

**Plan metadata:** committed alongside this SUMMARY.

## Files Created/Modified

- `.crate-names.txt` - the eleven-name allow-list (SEC-03/D-13)
- `scripts/check-crate-names.sh` - the bidirectional name-collision guard
- `Makefile` - added `check-crate-names` target adjacent to `check-changelogs`
- `.github/workflows/ci.yml` - added "Check crates.io package names" step inside the `cargo-deny` job
- `.planning/decisions/0026-crate-name-collision-guard.md` - the ADR recording this decision

## Decisions Made

- Followed plan 09-01's exact failable-guard shape (single bash-level `exit 0`, `sys.exit(0)` from the python heredoc for every reporting path, never a python-level `sys.exit(1)`), keeping the "exactly one literal `exit 0`" acceptance criterion satisfied.
- Two of the plan's literal `git diff | grep -c ...` acceptance-criterion commands returned `1` instead of the specified `0`, for reasons that are diff-format artifacts rather than actual violations of intent — recorded in detail under "Diff-artifact note" below, with the refined commands proving zero real content changes to the guarded references.
- ADR-0026 embeds one representative negative-path transcript (unlisted-name mode) per D-19's evidence bar, and points to this SUMMARY for the full six-mode sweep, matching ADR-0023's citation style for evidence that lives primarily in a plan SUMMARY.

## Diff-artifact note (two acceptance-criterion commands read literally)

Task 2's acceptance criteria specify two exact `git diff | grep -c ...` commands expected to return `0`. Both returned `1` when run literally, for reasons that are properties of unified-diff formatting, not of the actual edits:

1. **`git diff -- .github/workflows/ci.yml | grep -c 'actions-rs\|dtolnay\|actions/cache'` → `1`.** The plan's own instruction places the new step "immediately after the `Check per-crate changelogs` step", which sits two lines above the pre-existing `uses: actions/cache@v4` line. Git's default 3-line diff context pulls that unchanged line into the hunk display, so it appears in the raw diff text even though it was never touched. Refined check, counting only actual `+`/`-` content lines:
   ```
   $ git diff -- .github/workflows/ci.yml | grep -E '^[+-]' | grep -v '^+++\|^---' | grep -c 'actions-rs\|dtolnay\|actions/cache'
   0
   ```
   Zero actual changed lines reference any of those strings — no deprecated-action reference was touched.

2. **`git diff -- Makefile | grep -cE '^\-'` → `1`.** The single match is the diff's own `--- a/Makefile` file-header line, which always starts with `-` regardless of content. Refined check, excluding the header:
   ```
   $ git diff -- Makefile | grep -E '^-' | grep -vc '^---'
   0
   ```
   Zero actual deleted content lines — the `Makefile` change is additive only, as required.

Both diffs are reproduced in full under "Task Commits" above (`2758a9d`); a human reviewing the commit will see the same three added lines in each file with no deletions.

## Negative-path evidence (Task 1, per D-19)

All fixtures were reverted after their run; `git status --porcelain` showed only the two intended new files (`.crate-names.txt`, `scripts/check-crate-names.sh`) throughout, confirmed empty of scratch leftovers before Task 1's commit.

### Baseline — clean tree

Command: `./scripts/check-crate-names.sh; echo "EXIT=$?"`

```
🔍 Checking crates.io package-name allow-list against the workspace ...
✅ 11 publishable crate(s) checked, all match the allow-list exactly.
EXIT=0
```

### Mode 1 — unlisted tree name (scratch crate directory)

Command:
```
mkdir -p crates/scratch-unlisted && cat > crates/scratch-unlisted/Cargo.toml <<'EOF'
[package]
name = "paladin-scratch-unlisted"
version = "0.1.0"
edition = "2024"
EOF
./scripts/check-crate-names.sh; echo "EXIT=$?"
```

```
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 1 tree package name(s) not on the allow-list:
  - paladin-scratch-unlisted

If this failure is unexpected:
  1. An unlisted tree name means a new crate name was added. Confirm it is
     available on crates.io, then add it to .crate-names.txt yourself --
     the list is hand-edited and never auto-generated from the tree.
  2. A stale allow-list entry means a crate was removed or renamed. Remove
     its old name from .crate-names.txt.
EXIT=1
```

Restored with `rm -rf crates/scratch-unlisted`; guard returned to `EXIT=0`.

### Mode 2 — stale allow-list entry

Command: `printf 'paladin-nonexistent\n' >> .crate-names.txt && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored from backup)

```
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 1 allow-list entry with no corresponding tree crate:
  - paladin-nonexistent
...
EXIT=1
```

### Mode 3 — emptied `.crate-names.txt`

Command: `: > .crate-names.txt && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored)

```
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 11 tree package name(s) not on the allow-list:
  - paladin-ai
  - paladin-ai-core
  - paladin-battalion
  - paladin-content
  - paladin-herald
  - paladin-llm
  - paladin-memory
  - paladin-notifications
  - paladin-ports
  - paladin-storage
  - paladin-web
...
EXIT=1
```

All eleven tree crates named in one run.

### Mode 4 — removed `.crate-names.txt`

Command: `rm .crate-names.txt && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored)

```
⚠️  No allow-list found at /workspace/.claude/worktrees/agent-a7e915b2e3da9936a/.crate-names.txt
   Create it: one crates.io package name per line, hand-edited (see ADR-0026).
EXIT=1
```

Distinct missing-input message, following `check-api-surface.sh:15-19`'s shape; no python3 invocation attempted.

### Mode 5 — case-only-different allow-list entry

Command: `sed -i 's/^paladin-web$/Paladin-Web/' .crate-names.txt && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored)

```
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 1 tree package name(s) not on the allow-list:
  - paladin-web
FAIL: 1 allow-list entry with no corresponding tree crate:
  - Paladin-Web
...
EXIT=1
```

Both directions fail simultaneously — the comparison is exact, not case-insensitive.

### Mode 6 — reordered `.crate-names.txt` (must still pass)

Command: `sort -r .crate-names.txt > .crate-names.txt.new && mv .crate-names.txt.new .crate-names.txt && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored to canonical order)

```
🔍 Checking crates.io package-name allow-list against the workspace ...
✅ 11 publishable crate(s) checked, all match the allow-list exactly.
EXIT=0
```

Order-insensitive, as required — the guard is a set comparison.

### Mode 7 — `publish = false` exemption follows the manifest field, not the directory name

Command (untouched tree, allow-list restored): `python3 -c "import tomllib;d=tomllib.load(open('crates/doc-examples/Cargo.toml','rb'));assert d['package']['publish'] is False;print('exempt-by-field')"`

```
exempt-by-field
```

Command (manifest flipped): `sed -i 's/^publish = false$/publish = true/' crates/doc-examples/Cargo.toml && ./scripts/check-crate-names.sh; echo "EXIT=$?"` (then restored from a pre-edit copy)

```
🔍 Checking crates.io package-name allow-list against the workspace ...
❌ Crate-name allow-list check failed

FAIL: 1 tree package name(s) not on the allow-list:
  - paladin-doc-examples
...
EXIT=1
```

Flipping only the `publish` field (no directory rename, no allow-list change) flips the verdict and names `paladin-doc-examples` — the exemption is read from the manifest field. `git diff --stat -- crates/doc-examples/Cargo.toml` was empty after restoring.

### Post-fixture guard state

`bash -n scripts/check-crate-names.sh` → exit 0. `grep -c 'exit 0' scripts/check-crate-names.sh` → `1`. `./scripts/check-crate-names.sh` (final run before Task 1 commit) → exit 0. `git status --porcelain` showed only `.crate-names.txt` and `scripts/check-crate-names.sh` as new files — no scratch crate directory, no modified manifest.

## Issues Encountered

None requiring repair. The two literal `git diff | grep` acceptance-criterion mismatches (documented above under "Diff-artifact note") were investigated and confirmed to be diff-context/header artifacts, not defects in the change itself; no code or wiring was altered as a result.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `make check-crate-names` and the CI `cargo-deny` job both enforce the allow-list going forward, on every pull request rather than at main-branch dry-run or release time.
- ADR-0026 is ready for plan 09-07's close-out to add its row to `.planning/decisions/PROMOTION.md` (advancing to 0028, per D-18) and to `PROJECT.md`'s Key Decisions table; this plan deliberately did not touch `PROMOTION.md`.
- ADR-0026 also closes the residue left open in `.project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md` ("Keep CI/package guardrails that detect crates.io package-name collisions early") — Phase 10 / HARD-01's ledger row should record this decision as satisfying it.
- No blockers for the rest of Phase 9's wave 2 or wave 3 plans; this plan shares no file with 09-02, 09-03, 09-05, or 09-06.

## Self-Check: PASSED

- FOUND: `.crate-names.txt`
- FOUND: `scripts/check-crate-names.sh`
- FOUND: `.planning/decisions/0026-crate-name-collision-guard.md`
- FOUND commit: `264721a`
- FOUND commit: `2758a9d`
- FOUND commit: `5cde208`

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
