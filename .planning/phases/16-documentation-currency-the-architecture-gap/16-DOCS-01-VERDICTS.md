# DOCS-01 Per-File Currency Verdict Record (D-09)

**The single D-09 record.** One artifact, fourteen rows, appended to by plans 16-01 through
16-05 as each sweeps its assigned files. No other file carries a DOCS-01 verdict.

## Method (read before adding or trusting a row)

A verdict is settled by **content**, never by file existence or modification time
(D-00e, Success Criterion 1). "File exists" and "file was touched recently" prove nothing —
16-RESEARCH.md's Pitfall 5 found all fourteen files exist and twelve carry recent mtimes while
still containing fabricated content (`cicd.md`'s CI sample being the worked example this plan
settles). A row's Verdict is `current` only if every applicable signal class below was actually
run against that file and found to match the live tree; a row is `updated → commit` only after
the found discrepancies were fixed and the fix re-verified (`mdbook build docs/` green).

**The eight signal classes** (one producing command each, run per file `F`; every Findings cell
below names the actual command that produced its result, not a copy of this table):

| # | Signal class | Producing command | Compared against |
|---|---|---|---|
| 1 | version strings | `grep -nE 'v?[0-9]+\.[0-9]+\.[0-9]+' "$F"` | root `Cargo.toml:34` (`0.8.0`) |
| 2 | dependency pins | `grep -nE '^[a-z-]+ *= *\{? *version' "$F"` | that crate's `Cargo.toml` |
| 3 | crate names | `grep -noE 'paladin-[a-z-]+' "$F" \| sort -u` | `ls crates/` — classify vs k8s object names first |
| 4 | module/source paths | `grep -noE '(crates\|src)/[A-Za-z0-9_/.-]+\.rs' "$F"` | `test -f` each path |
| 5 | `make` targets | `grep -noE 'make [a-z-]+' "$F"` | `grep -oE '^[a-z-]+:' Makefile` |
| 6 | workflow/job names | `grep -noE '[a-z-]+\.yml' "$F"` + quoted `jobs:` ids | `ls .github/workflows/` + each file's `jobs:` block |
| 7 | error types | `grep -noE '[A-Z][A-Za-z]*Error(::[A-Za-z]+)?' "$F"` | `grep -rn` in `crates/` and `src/` |
| 8 | feature flags | `grep -noE -- '--features [a-z0-9,_-]+\|feature = "[a-z0-9_-]+"' "$F"` | the relevant `[features]` block |
| — | prose | read end to end | statements the 0.8.0 tree contradicts |

A `current` verdict with an empty Findings cell is invalid — the row must name every signal
class actually run, even when the answer for that class is "checked, matches, no finding."
`current` means *checked and found to match*, never *not looked at*.

## Row order

Fixed, declared here, never re-sorted: `docs/src/user-guides/` (alphabetical), then
`docs/src/deployment/` (alphabetical), then `docs/src/operations/` (alphabetical). This mirrors
the plan split (16-01 cicd worked example; 16-02/16-03 user-guides; 16-04 deployment;
16-05 operations) and lets a partial sweep be audited by scanning top-to-bottom without
cross-referencing a separate index.

## Concurrency note (DOCS-01 backstop truth)

If a sweep is interrupted part-way, this record stays internally consistent by construction:
every row marked `current`/`updated → commit` names a file that was actually checked in the
plan that touched it, and every not-yet-swept file is seeded below with the verdict text used
in the table's Verdict column (see below) rather than being omitted. A reader can never mistake
"row absent" for "row checked" — absence never happens; only the two defined states do.

## The fourteen rows

| File | Signals checked | Findings (command / file:line) | Verdict |
|---|---|---|---|
| docs/src/user-guides/maneuver-flow-dsl.md | — | — | pending — not yet checked |
| docs/src/user-guides/memory-management.md | — | — | pending — not yet checked |
| docs/src/user-guides/orchestration.md | — | — | pending — not yet checked |
| docs/src/user-guides/output-formatting.md | — | — | pending — not yet checked |
| docs/src/user-guides/paladin-configuration.md | — | — | pending — not yet checked |
| docs/src/user-guides/tool-integration.md | — | — | pending — not yet checked |
| docs/src/deployment/cicd.md | 1 (versions), 3 (crate names), 4 (source paths), 5 (make targets), 6 (workflow/job names), 7 (error types), 8 (feature flags), prose | 1: `grep -nE 'v?[0-9]+\.[0-9]+\.[0-9]+' docs/src/deployment/cicd.md` → 0 hits (no version-string finding). 3: `grep -noE 'paladin-[a-z-]+' docs/src/deployment/cicd.md \| sort -u` → 1 hit, `paladin-chart` (line 485, a Helm chart directory name, not a crate — classified per M-06, no action; `ls crates/` has no such entry and none is implied). 4/5/7: 0 hits each (no source paths, `make` targets, or typed errors quoted on this page). 8: `grep -noE -- '--features [a-z0-9,_-]+' docs/src/deployment/cicd.md` → 2 hits, `--features integration-tests` (lines 371, 382) — the flag and the crate feature it names are unchanged in the tree, no finding. 6 (the real findings — re-derived fresh this session, not copied from 16-RESEARCH.md): `grep -noE '[a-z-]+\.yml' docs/src/deployment/cicd.md \| sort -u` vs `ls .github/workflows/` (→ `benchmarks.yml ci.yml docs.yml feature-flags.yml pre-commit.yml release.yml`) showed the page's "Workflow Structure" diagram and its "Integration Testing" section both named `integration-tests.yml`, a file deleted by commit `2cf9919` (`git log --oneline -- .github/workflows/integration-tests.yml` shows the deletion; its job survives as `integration-tests:` inside `ci.yml:388`, confirmed by `grep -n '^  integration-tests:' .github/workflows/ci.yml`). Prose read: the page's top `ci.yml` sample quoted `on: push: branches: [ main, develop ]` / `pull_request: branches: [ main, develop ]` at lines 53/55 — the live trigger is `push: branches: [ '**' ]` / `pull_request: branches: [ main, 'release/**' ]` (`.github/workflows/ci.yml:13-24`; `develop` was pruned by plan 15.1-09's trunk cutover). The sample also named a `check`/`Check` job that does not exist; the live job is `lint`/`Code Quality` (`.github/workflows/ci.yml:41-42`). Fourth occurrence of the same stale trigger at lines 324/326 inside the Integration-Testing yaml sample, from when that job still had its own standalone `on:`/`schedule` block as a separate workflow file. Post-fix re-verification: `grep -cE 'main, ?develop' docs/src/deployment/cicd.md` → 0; `grep -c 'integration-tests.yml' docs/src/deployment/cicd.md` → 0; `grep -cF "branches: [ '**' ]" docs/src/deployment/cicd.md` → 1; `grep -c 'Code Quality' docs/src/deployment/cicd.md` → 1; `mdbook build docs/` → exit 0 (Run 3, `16-LINKCHECK-REPORT.md`). **Out-of-scope findings deliberately not fixed here** (not part of 16-RESEARCH.md's three pre-evidenced findings, no acceptance criterion covers them, logged to `deferred-items.md` per the executor's scope-boundary rule): the "Docker Build Pipeline" section names a `docker-publish.yml` workflow and the "Security Scanning" section names a `security.yml` workflow (including a `snyk` job) — neither file exists in `ls .github/workflows/`, and per `.github/instructions/security.instructions.md` Snyk was evaluated and removed from this project entirely, so that job also contradicts current governance, not just current file layout. | updated → commit |
| docs/src/deployment/docker.md | — | — | pending — not yet checked |
| docs/src/deployment/kubernetes.md | — | — | pending — not yet checked |
| docs/src/deployment/production.md | — | — | pending — not yet checked |
| docs/src/operations/logging.md | — | — | pending — not yet checked |
| docs/src/operations/monitoring.md | — | — | pending — not yet checked |
| docs/src/operations/performance-tuning.md | — | — | pending — not yet checked |
| docs/src/operations/troubleshooting.md | — | — | pending — not yet checked |

**Row count check:** 14 data rows (`grep -c '^| docs/src/' 16-DOCS-01-VERDICTS.md` → 14).
**Not-yet-checked count check:** thirteen of those fourteen rows still carry the seeded
not-yet-checked verdict text above after this plan; only the `cicd.md` row has moved to
`updated → commit`.
