---
phase: 04-release-coherence
verified: 2026-08-03T14:20:00Z
status: gaps_found
score: 9/10 must-haves verified; 1 truth FAILED (regression), 3 items deferred/human-needed and honestly recorded as such
behavior_unverified: 0
overrides_applied: 0
re_verification: no previous VERIFICATION.md existed for this phase
gaps:
  - truth: "CI on the release branch proves the full gate suite: ... workspace tests ... (ROADMAP SC5 / REL-05)"
    status: failed
    reason: "`cargo test --workspace --offline` FAILS at the reviewed HEAD (a9113a7) with 1 failed test: `paladin_web::openapi::tests::openapi_matches_committed_baseline`. This is a real, reproducible regression, not an environmental limitation. Plan 04-04 measured 'cargo test --workspace' green at commit `d2898a3` — correctly, at that time. Plan 04-05 (Wave 3, run afterward) then bumped every workspace crate's version to 0.7.0, including `crates/paladin-web/Cargo.toml`, which changes the generated OpenAPI spec's `info.version` field via `CARGO_PKG_VERSION`. Nobody regenerated the committed baseline `crates/paladin-web/openapi.json` (still reads `\"version\": \"0.6.0\"`), and nobody re-ran the full gate suite after the version bump to catch the drift. The milestone ledger's REL-05 row states '`cargo test --workspace --offline` — 2,924 passed / 0 failed / 122 ignored' as if this were the state of the final tree; it is not — that figure describes the pre-version-bump commit only, and no later plan (04-05, 04-06, 04-07) re-verified it."
    artifacts:
      - path: "crates/paladin-web/openapi.json"
        issue: "Committed OpenAPI baseline still declares `\"version\": \"0.6.0\"` after the workspace-wide bump to 0.7.0; the test that guards spec/baseline drift (`openapi::tests::openapi_matches_committed_baseline`, `crates/paladin-web/src/openapi.rs:141`) fails deterministically as a result."
    missing:
      - "Regenerate the baseline: `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline --offline`, then commit the updated `crates/paladin-web/openapi.json`."
      - "Re-run `cargo test --workspace --offline` in full (not just the one test) after the fix, and update `04-release-measurement.md` / the milestone ledger's REL-05 row with the re-verified count from the actual final commit, not the pre-bump commit."
deferred:
  - truth: "CI proves the multi-arch Docker build within its 500 MB / 300 s budget (ROADMAP SC5, literal wording)"
    addressed_in: "Phase 15 / PIPE (named owner in Phase 4's own ledger row; docker is absent from every sandbox that has worked this repo, including this verification session)"
    evidence: "04-ci-gate-deferrals.md rows 1 and 5; ledger row 'REL-05 — multi-arch Docker build within the 500 MB / 300 s budget | deferred with reason'"
  - truth: "CI proves the Kubernetes smoke test within its 30 s startup budget (ROADMAP SC5, literal wording)"
    addressed_in: "Phase 15 / PIPE (first execution, kind/kubectl absent here too) and Phase 14 / WEB (real readiness probes — k8s/deployment.yaml currently runs a placeholder sleep with all probes commented out)"
    evidence: "04-ci-gate-deferrals.md rows 2 and 3; ledger rows 'REL-05 — kind-based Kubernetes smoke test...' and 'REL-05 — real readiness-probe-based Kubernetes startup measurement'"
  - truth: "CI is observed actually running on a `release/**` push"
    addressed_in: "The human release gate (D-03) that owns the tag/branch push, explicitly out of this phase's scope"
    evidence: "04-ci-gate-deferrals.md row 4; ledger row 'REL-05 — CI actually running on a release/** push'"
---

# Phase 4: Release Coherence Verification Report

**Phase Goal:** A developer can clone the release tag, build it, trust its version and its
dependency posture, follow the quickstart to a working agent, and see CI prove all of it.
**Verified:** 2026-08-03T14:20:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification.

**Bottom line up front:** This phase's version/edition/advisory/docs work (REL-01 through REL-04)
is genuinely solid — every claim I independently re-derived checked out, including compiling the
repaired QUICKSTART sample myself from scratch (not trusting the SUMMARY's word for it). The
phase's own honesty discipline around Docker/Kubernetes/CI-execution (D-15/D-16) is also real: I
found no fabricated pass and no blended satisfied/deferred row anywhere in the ledger for those
three items. **But REL-05's "workspace tests pass" claim is false at the current HEAD.** I ran
`cargo test --workspace --offline` myself and it fails, deterministically, because plan 04-05's
version bump (0.6.0 → 0.7.0 in `crates/paladin-web/Cargo.toml`) changed the generated OpenAPI
spec's version field, and nobody regenerated the committed `openapi.json` baseline or re-ran the
gate suite afterward to catch it. The milestone ledger's REL-05 row cites a test count that was
measured *before* the version bump and presents it as the state of the final tree — this is exactly
the "task completion ≠ goal achievement" pattern this verification process exists to catch, and it
is a genuine, fixable regression, not an environmental constraint.

## The Regression (BLOCKER)

```
$ cargo test --workspace --offline
...
---- openapi::tests::openapi_matches_committed_baseline stdout ----
thread 'openapi::tests::openapi_matches_committed_baseline' panicked at crates/paladin-web/src/openapi.rs:141:9:
assertion `left == right` failed: OpenAPI spec drifted from /workspace/crates/paladin-web/openapi.json.
If the change is intentional, regenerate with: UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline
  left:  ... "version": "0.7.0" ...   (generated from the live crate, CARGO_PKG_VERSION = 0.7.0)
  right: ... "version": "0.6.0" ...   (crates/paladin-web/openapi.json, committed, never regenerated)

failures:
    openapi::tests::openapi_matches_committed_baseline
test result: FAILED. 116 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.52s
```

Confirmed deterministic by re-running the single named test twice more
(`cargo test -p paladin-web --lib openapi::tests::openapi_matches_committed_baseline --offline`).
Confirmed root cause: `crates/paladin-web/openapi.json:14` reads `"version": "0.6.0"`;
`git log --oneline -- crates/paladin-web/openapi.json` shows it was last regenerated at
`23b187b chore(release): regenerate OpenAPI baseline for 0.6.0 (M12 E7)` — i.e. for the *previous*
release, never for this one. Confirmed the timing: plan 04-04's `cargo test --workspace` measurement
(`04-release-measurement.md`, "Entry measurement — `cargo test --workspace`") was taken at commit
`d2898a3`, which precedes plan 04-05's version-bump commit (`c2e20a1`) — the 2,924/0/122 figure was
correct when measured and has not been re-verified since. Neither `04-05-PLAN.md`, `04-05-SUMMARY.md`,
`04-06-PLAN.md`/`SUMMARY.md`, nor `04-07-PLAN.md`/`SUMMARY.md` mentions `openapi.json` at all —
`crates/paladin-web/Cargo.toml` is in plan 04-05's own `files_modified` list, but the downstream
generated-artifact consequence of bumping it was never checked.

**This is not a Docker/Kubernetes-style "environment can't run it" gap.** It is fully reproducible
in this sandbox right now, with a one-line fix path the test's own panic message already names.

## Independent Re-Derivation (Goal-Backward) — everything else

I did not trust SUMMARY.md claims. Below is what I ran myself, in this session, against HEAD
`a9113a7` on `release/v0.7.0`.

### Version convergence (REL-01 / SC1) — ✓ VERIFIED

| Check | Command | My result | Match |
|---|---|---|---|
| All manifests agree on one version | `grep -h '^version' Cargo.toml crates/*/Cargo.toml \| sort \| uniq -c` | `12 version = "0.7.0"` | ✓ |
| Local tag exists, unpushed | `git tag --list`, `git rev-list -n1 v0.7.0`, `git ls-remote --tags origin \| grep v0.7.0` | tag `v0.7.0` present locally → `648e7a4`; absent from remote tags | ✓ |
| Nothing pushed this session | `git rev-list --count origin/release/v0.7.0..HEAD` | 249 commits ahead, unpushed | ✓ |
| CHANGELOG dated correctly | `grep -n '^## \[' CHANGELOG.md` | `## [0.7.0] - 2026-08-03`, `## [0.6.0] - 2026-06-10` | ✓ |
| `[0.6.0]` date not invented | `git log -S'## [0.6.0]' --oneline -- CHANGELOG.md` | `67b6207`, matches the record's cited commit/date exactly | ✓ |
| tiktoken-rs external pin untouched | `grep tiktoken-rs crates/paladin-memory/Cargo.toml crates/paladin-content/Cargo.toml` | both still `"0.6.0"` | ✓ |
| ADR exists, correct conformance field | `.planning/decisions/0008-workspace-version-0-7-0.md` | present, `must change`, cites REL-01 | ✓ |
| No push/publish executed | reflog + `git ls-remote --tags origin` | no push/publish evidence anywhere | ✓ |

### Rust edition (REL-02 / SC2) — ✓ VERIFIED

| Check | Command | My result | Match |
|---|---|---|---|
| One edition everywhere, 12 manifests | `grep -h '^edition' Cargo.toml crates/*/Cargo.toml \| sort -u` / `grep -c 2024` | one line `edition = "2024"`; count 12 | ✓ |
| Workspace builds | `cargo build --workspace --offline` | `Finished`, exit 0 | ✓ |
| ADR exists | `.planning/decisions/0009-workspace-rust-edition-2024.md` | present, `must change`, cites REL-02 | ✓ |
| Requirements amended in place | `REQUIREMENTS.md` ARCH-03(a) | amended text cites ADR-0009 | ✓ |
| CONCERNS.md corrected at source, not deleted | `codebase/CONCERNS.md:1-25` | 3 dated amendment blocks, original wrong claim preserved+corrected | ✓ |

### Advisory posture (REL-03 / SC3) — ✓ VERIFIED

| Check | Command | My result | Match |
|---|---|---|---|
| `cargo audit` | `cargo audit` | 0 vulnerabilities; 4 warnings (atty, event-listener, scc, spin-yanked) — exactly the "4 newly-surfaced" advisories the ledger names | ✓ |
| `cargo deny check` | `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` | ✓ |
| Stale suppression removed | `grep RUSTSEC-2025-0121 deny.toml` | absent | ✓ |
| 14 ignores, 12/14 carry migration/review note | read `deny.toml:112-148` | counted 14; 12 have explicit "revisit when X", 2 (`paste`, `lopdf`) have a reason without an explicit trigger — matches the "12 of 14" figure exactly | ✓ |
| Duplicate CI audit job present (scope fence, SUPPLY-01/Phase 12) | `grep -n "^  security:" ci.yml`, reproduced its exact command | present, untouched; `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` exits 0 locally | ✓ |

### Documentation / QUICKSTART (REL-04 / SC4) — ✓ VERIFIED

| Check | My result | Match |
|---|---|---|
| Target reconciled | `quickstart.md:3` "under 15 minutes"; `introduction.md:9` "15 minutes" — agree | ✓ |
| Feature-name fix real | `paladin-llm`'s `[features]` table defines `openai` (not `llm-openai`); quickstart.md uses `openai` | ✓ |
| **Sample actually compiles — I built it myself** | Scratch project outside the repo, path deps on `paladin-ai`/`paladin-ports`/`paladin-llm`, workspace `Cargo.lock` copied in (avoids re-solving into the yanked `spin 0.9.8`), quickstart.md's exact `src/main.rs` pasted verbatim: `cargo build --offline` → `Finished \`dev\` profile ... target(s) in 2m 32s`, exit 0 | ✓ — independent compile, not a re-read of the SUMMARY |
| No leaked scratch artifact | `git status --porcelain` after cleanup | clean | ✓ |

### Gate suite (REL-05 / SC5) — ⚠️ PARTIAL — one real regression, rest verified

| Check | My result | Match |
|---|---|---|
| `cargo fmt --all -- --check` | exit 0, clean | ✓ |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, zero warnings, all 12 crates | ✓ |
| `cargo test --workspace --offline` | **FAILS** — 1 failed test, `paladin-web`'s `openapi_matches_committed_baseline` | ✗ — see "The Regression" above |
| CI YAML parses; new jobs present | 16 jobs incl. `examples`, `docker`, `kubernetes-smoke` | ✓ |
| `push` trigger restored, covers `release/**` | `ci.yml:9` — `[ main, develop, 'feature/**', 'release/**' ]` | ✓ |
| Docker/K8s jobs explicitly marked unexecuted in-file | `AUTHORED AND STATICALLY VALIDATED ONLY ... has never been executed` comment blocks present at both job definitions | ✓ |
| Ledger rows SPLIT, not blended | one `satisfied` row for fmt/clippy/test/doctest/examples; separate `deferred with reason` rows for Docker/K8s/CI-push/readiness-probes/sibling-triggers, each with a named owner | ✓ structurally — but the `satisfied` row's test-count citation is stale (see regression above) |
| "22 examples" corrected at source, all 5 restatements | `ROADMAP.md`, `PROJECT.md` (×2), `REQUIREMENTS.md` all carry dated amendment blocks citing `04-release-measurement.md`, original text preserved | ✓ |
| Example feature-matrix (4 invocations, 47 targets) | matches stated rationale about cargo silently skipping feature-gated example targets | ✓ |

### Scope fences — confirmed respected (not touched by this phase)

| Fence | Result |
|---|---|
| Duplicate `Security Audit` job (`ci.yml:466`) | present, untouched — SUPPLY-01/Phase 12 |
| No owner/expiry fields added to `deny.toml`/`.cargo/audit.toml` | confirmed absent |
| Licence three-way untouched | unchanged allow-list/MPL exceptions |
| `api-surface` job untouched | present, unedited |
| `paladin-ports` `doctest = false` untouched | still present |
| No `cli-tests`/`bench-check`/`coverage` job, no `.codecov.yml` | none found; file absent |

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| REL-01 | ✓ SATISFIED | independently re-derived: 12 manifests at 0.7.0, tag local-unpushed, CHANGELOG dated correctly, ADR-0008, human confirmation recorded |
| REL-02 | ✓ SATISFIED | independently re-derived: 12 manifests at edition 2024, workspace builds, ADR-0009, CONCERNS.md corrected at source |
| REL-03 | ✓ SATISFIED | independently re-derived: audit/deny clean, stale suppression removed, 12/14 migration notes, 4 new advisories recorded+deferred with named owner |
| REL-04 | ✓ SATISFIED | independently re-derived: QUICKSTART fix proven by my own from-scratch compile; RECON-08 citation for doc-review clause; timing honestly labeled non-clean-machine |
| REL-05 | ✗ NOT FULLY SATISFIED | fmt/clippy/examples verified green; **`cargo test --workspace` fails at current HEAD** due to a stale OpenAPI baseline the version bump invalidated; Docker/K8s/CI-push honestly deferred with named owners (not a fabricated pass) |

REQUIREMENTS.md's traceability table (lines 3912-3916) currently marks all five `REL-01..REL-05` as
`Complete`. Given the reproducible test failure above, REL-05's `Complete` marking is premature
until `openapi.json` is regenerated and the full suite is re-verified green on the actual final
commit.

## Anti-Patterns Found

None beyond the regression documented above. No `TODO`/`FIXME`/`XXX`/`HACK`/`PLACEHOLDER` markers
introduced by this phase's edits. No stub returns, no hardcoded empty data, no console.log-only
handlers. The `AUTHORED AND STATICALLY VALIDATED ONLY` comment blocks in `ci.yml` are themselves a
deliberate anti-pattern *guard*.

## Human Verification / Deferred Items (secondary — do not block on these; the regression above does)

These three remain honestly recorded by the phase itself as `deferred with reason`, each with a
named owner, none fabricated as passing:

1. **Docker multi-arch build + 500 MB/300 s budgets** — `docker` absent from every sandbox
   including this one; authored and statically validated only. Owner: Phase 15/PIPE.
2. **Kubernetes kind smoke test + 30 s startup budget** — `kind`/`kubectl` absent here too;
   additionally `k8s/deployment.yaml:66-174` runs a placeholder with all readiness probes
   commented out, so even a first execution measures scheduling, not app readiness. Owner:
   Phase 15/PIPE (execution), Phase 14/WEB (real probes).
3. **CI actually observed firing on a `release/**` push** — the trigger is restored and correct,
   but observing it fire requires the human-gated tag/branch push (D-03), which this phase
   correctly stops short of.

## Gaps Summary

**One real, fixable blocker:** `cargo test --workspace --offline` fails at the reviewed HEAD because
plan 04-05's version bump changed `paladin-web`'s generated OpenAPI `info.version` to `0.7.0` while
the committed baseline (`crates/paladin-web/openapi.json`) was never regenerated and still reads
`0.6.0`. Fix: `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline --offline`,
commit the regenerated `openapi.json`, then re-run the full `cargo test --workspace --offline` suite
once and update `04-release-measurement.md` / the milestone ledger's REL-05 row with the count from
the actual final commit (not the pre-version-bump commit plan 04-04 measured).

Everything else in this phase — version convergence, edition unification, advisory posture, the
QUICKSTART repair (which I compiled myself, independently), the CI configuration repair, the two
ADRs, the "22 examples" correction at all five sites, and the honest, non-fabricated Docker/K8s/CI-
push deferrals — checked out exactly as claimed under independent re-derivation. This is a narrow,
one-file, mechanically-fixable gap, not a systemic problem with the phase's work.

---

_Verified: 2026-08-03T14:20:00Z_
_Verifier: Claude (gsd-verifier)_
