---
phase: 4
slug: release-coherence
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (high)
threats_open: 0
asvs_level: 1
created: 2026-08-03
---

# Phase 4 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

**Register origin:** `register_authored_at_plan_time: true` — all seven PLAN.md files carried a
parseable `<threat_model>` block. This audit **verifies the recorded mitigations exist**; it does
not scan for new threats.

**Scope note.** This phase ships no product code. Its security surface is **supply-chain and
release integrity** — advisory suppressions, dependency policy, what a pushed tag triggers, CI
configuration executing with runner privileges, and the honesty of the record later phases join
against. Threats are modelled accordingly rather than against web-input categories the phase does
not create.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| local repository → GitHub remote | A `git push` of a `v*.*.*` tag crosses from reversible local state into a triggered publish pipeline | Source tree, tags |
| pushed tag → crates.io | `release.yml` publishes ten crates in dependency order; crates.io permits yanking, never unpublishing | Published crate artifacts |
| `github.com/RustSec/advisory-db` → local audit verdict | An external versioned data source decides whether the security gate is green | Advisory metadata |
| `deny.toml` / `.cargo/audit.toml` → CI gate outcome | Two suppression files control what both gates are blind to | Policy configuration |
| GitHub Actions marketplace → CI runner | Third-party actions execute with runner privileges | Workflow execution |
| `cargo-release` / `cargo fix --edition` → workspace source | Automated rewriters with write access to twelve manifests and first-party source | Source and manifests |
| authored configuration → claimed verdict | Config text is not an executed gate — the boundary this phase was most likely to cross wrongly | Verification claims |
| a phase's verdict → the ledger Phases 5-16 read as ground truth | An overstated verdict propagates as fact without re-verification | Recorded truth |

---

## Threat Register

**36 entries** (35 numbered + one phase-wide supply-chain entry). Block threshold: `high`.

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|---|---|---|---|---|---|---|
| T-04-20 | Elevation of privilege | pushing tag `v0.7.0` | critical | mitigate | Push/publish prohibited at plan level, gated on human by D-03 | **closed** — `git ls-remote --tags origin` returns 0 matches for `v0.7.0`, verified 2026-08-03 |
| T-04-02 | Repudiation | `04-release-measurement.md` | high | mitigate | D-17 provenance block, raw pasted stdout | closed — file present, 1856 lines, provenance sections verified |
| T-04-05 | Tampering | `deny.toml` `[advisories] ignore` | high | mitigate | Only the non-matching entry removed; count asserted 14 | closed — `cargo deny check` passes, zero `advisory-not-detected` warnings |
| T-04-07 | Repudiation | governance drift | high | mitigate | Owner/expiry fields prohibited; 4 advisories handed to named owners | closed — no owner/expiry field added to either config; SEC-01/SUPPLY-02 rows present |
| T-04-10 | Tampering | third-party GitHub Actions | high | mitigate | Only actions already running in sibling workflows | **closed — re-verified after the toolchain fix.** `git diff 68ba809..HEAD -- ci.yml` adds only `actions/cache@v4`, `actions/checkout@v5`, `docker/build-push-action@v6`, `docker/setup-qemu-action@v3`, `helm/kind-action@v1`, and changes `dtolnay/rust-toolchain@master` → `@stable` (same action, different ref). No new marketplace dependency |
| T-04-11 | Repudiation | reporting an unexecuted gate as green | high | mitigate | Plan-level prohibition; deferral register with named owners | closed — and subsequently **superseded by execution**: all three jobs ran for real (run `30842748080`) |
| T-04-15 | Spoofing | outward-facing GitHub actions | high | mitigate | `gh` read-only; push/dispatch/PR behind the D-03 human gate | **closed — control held, gate opened by the human.** See Accepted Risks / decision trail below |
| T-04-16 | Repudiation | gate sections of the measurement record | high | mitigate | Full D-17 probe block plus verbatim stdout per section | closed |
| T-04-17 | Tampering | gate command strings | high | mitigate | Narrowing `--workspace`, dropping `-D warnings` prohibited | closed — commands re-run independently by the orchestrator and the verifier |
| T-04-18 | Spoofing | bulk `cargo build --examples` exit code | high | mitigate | Binary-presence assertion for all 47 basenames, not exit code | closed — and proven in CI: `Example Muster` passed on a clean runner |
| T-04-21 | Elevation of privilege | `make release` | high | mitigate | Target prohibited outright | closed — zero `chore(release): version` commits; target never invoked |
| T-04-24 | Tampering | an untested commit carrying the release tag | high | mitigate | `depends_on: ["04-04"]` makes the gate suite a hard predecessor | **closed after being MATERIALLY REALIZED twice.** See Realized Threats below |
| T-04-26 | Repudiation | recorded QUICKSTART timing | high | mitigate | "measured under stated conditions" label; four blocking conditions enumerated | closed |
| T-04-27 | Tampering | the documented QUICKSTART steps | high | mitigate | Tuning/reordering to influence the measurement prohibited | closed |
| T-04-28 | Spoofing | a sample that looks correct and cannot compile | high | mitigate | Imports asserted byte-identical to `examples/basic_paladin.rs` | closed — sample compiled against the shipped tree, independently reproduced by the verifier |
| T-04-31 | Repudiation | the REL-05 ledger row | high | mitigate | REL-05 split into facet rows; blended `satisfied` prohibited | closed — facets present; Docker/K8s later flipped to `satisfied` **by measurement**, not by blending |
| T-04-33 | Elevation of privilege | recording an answer this phase does not own | high | mitigate | Each handed-off question appears only as a deferral row with an owner | closed |
| T-04-01 | Tampering | `cargo fix --edition --allow-dirty` | medium | mitigate | `-p <crate>` scoping; clean-tree precondition | closed |
| T-04-03 | Tampering | edition-migration diagnostics | medium | mitigate | Crate-level `#![allow(...)]` prohibited | closed |
| T-04-06 | Information disclosure | advisory-DB freshness | medium | mitigate | Advisory count and fetch date recorded | closed — DB `d91a8fc9`, 1186 advisories, fetched 2026-08-03 |
| T-04-08 | Tampering | `[licenses]` allow-list | medium | mitigate | Licence three-way prohibited from resolution by inference | closed — `[licenses]` block untouched; SEC-02 (Phase 9) still owns it |
| T-04-12 | Elevation of privilege | live `push:` trigger on `release/**` | medium | mitigate | Widens which branches run the existing workflow only; `push: false` retained on the docker job | closed — no job gained registry or crates.io credentials |
| T-04-13 | Denial of service | budget gates weakened to warnings | medium | mitigate | Softening a budget to `::warning::` prohibited | **PARTIALLY REALIZED → accepted.** Time assertion softened by explicit human decision; size and startup assertions remain hard. See Accepted Risks |
| T-04-19 | Tampering | in-place amendment of planning records | medium | mitigate | Dated parenthetical appended; original claim retained | closed — verified by the gsd-verifier reading the ledger diff directly |
| T-04-22 | Tampering | `cargo release version` rewriting external requirements | medium | mitigate | Both `tiktoken-rs` requirements asserted unchanged | closed — both still `version = "0.6.0"` |
| T-04-23 | Repudiation | the retroactive `[0.6.0]` date | medium | mitigate | Derived via `git log -S`, derivation recorded | closed — commit `67b6207`, 2026-06-10 |
| T-04-29 | Tampering | scratch project leaking into the tree | medium | mitigate | Built outside the repo; `git status --porcelain` asserted empty | closed |
| T-04-30 | Information disclosure | LLM API key handling | medium | mitigate | No key obtained; live path filed as a deferral | closed — no credential written to any scratch project or record |
| T-04-32 | Tampering | in-place amendment of prior claims | medium | mitigate | `CONCERNS.md` original wording retained | closed |
| T-04-34 | Spoofing | an ADR that parses as unstructured | medium | mitigate | Bulleted `## Code Locations` / `## Considered Options` | closed — ADR-0008 and ADR-0009 present |
| T-04-35 | Tampering | duplicate rather than amended ledger rows | medium | mitigate | Each forward-owner row asserted to appear exactly once | closed |
| T-04-04 | Spoofing | toolchain identity | low | accept | `rust-toolchain.toml` pins 1.97.1; `rustc -vV` recorded verbatim | closed (accepted) |
| T-04-09 | Spoofing | duplicate `Security Audit` CI job | low | accept | Measured non-blocking; handed to SUPPLY-01 (Phase 12) | closed (accepted) — both jobs passed in CI run `30842748080` |
| T-04-14 | Information disclosure | dummy secrets in the kind smoke job | low | accept | Literal dummy values in an ephemeral cluster | closed (accepted) — job executed; no real credential referenced |
| T-04-25 | Spoofing | lightweight vs annotated tag | low | mitigate | `git cat-file -t v0.7.0` must return `tag` | closed — returns `tag` |
| T-04-SC | Tampering | npm/pip/cargo installs | low | accept | Phase installs no packages | closed (accepted) — phase-wide; `04-RESEARCH.md` § "Package Legitimacy Audit" records "Not applicable" |

**Open at or above `high`: 0.**

---

## Realized Threats

Recorded because a threat that actually fired teaches more than one that did not.

### T-04-24 — an untested commit carrying the release tag (high) — realized **twice**

The planned mitigation (`depends_on: ["04-04"]` making the gate suite a hard predecessor) was
correct but **insufficient**: it constrained where the tag was *created*, not where it *stayed*.

1. **First realization.** The tag was created after Wave 3's merge, then Waves 4-5 and the OpenAPI
   fix landed. The tag sat 17 commits behind HEAD, on a commit whose `cargo test --workspace`
   failed (stale `openapi.json`) and whose QUICKSTART was un-repaired. Caught by a goal-derived
   UAT pre-check. Fixed by re-pointing the tag.
2. **Second realization.** Four CI-fix iterations then changed `.github/workflows/ci.yml`
   (+58/-14). The tag again shipped a pre-fix pipeline — zero `dtolnay/rust-toolchain@stable`
   occurrences and no size-measurement load step. Caught by the exclude-`.planning` check written
   into the UAT record after the first realization. Fixed again.

**Structural correction, not another patch:** a release tag must be created **at seal time, after
the last shipped change** — not at the point in the plan where the version bump happens. Until a
phase seals, any tag it holds is provisional and must be re-checked with
`git diff --name-only <tag>..HEAD | grep -v '^\.planning/'` immediately before any push. A non-empty
result is the blocking condition; a `.planning/`-only result is benign.

**Current state (2026-08-03):** one commit past the tag, planning-record only, shipped tree
byte-identical (`git diff --stat v0.7.0..HEAD -- . ':(exclude).planning'` empty). Gate suite at that
tree: `cargo fmt --all -- --check` clean, `cargo test --workspace --offline` 0 failures.

### T-04-15 — outward-facing actions (high) — control held, gate opened by the human

The mitigation was "`gh` read-only; dispatching, pushing and PR creation are prohibited and stay
behind the D-03 human gate." The branch **was** pushed to origin — under an explicit user decision
recorded in `04-UAT.md` test 27 ("push the BRANCH only, not the tag"). The control functioned as
designed: the agent did not act unilaterally, and the human opened the gate deliberately.

The **tag** push, which is the step that reaches crates.io, remains unexecuted. Recorded here rather
than closed silently, because "the human authorized it" is only a valid closure if the
authorization is auditable.

---

## Accepted Risks

### AR-04-01 — Docker build wall-clock budget softened to a warning (from T-04-13, medium)

**What the threat prohibited:** softening a budget assertion to `::warning::`.

**What was done, and why:** the multi-arch wall-clock assertion in `ci.yml`'s `docker` job was
changed from `::error::` + `exit 1` to `::warning::`, by explicit human decision on 2026-08-03,
after the first real measurement showed the budget was **mis-specified rather than unmet**. The
300 s figure derives from `PROJECT.md:767`'s single-arch "112 MB built in 5m31s" but SC5 applies it
to a multi-arch build. Measured multi-arch durations across this repository's history: 48m09s
(v0.4.2), 47m58s (v0.4.3), 41m47s (v0.5.0), 44m03s (v0.5.1), 49m43s (first CI execution) — the
budget has never once been met. `Dockerfile:33` builds natively per platform, so `linux/arm64`
compiles under QEMU emulation.

**What was NOT softened:** the image-size assertion (`::error::` + `exit 1` at 500 MB) and the
Kubernetes pod-startup assertion (`::error::` at 30 s) both remain hard gates. Verified in
`ci.yml` on 2026-08-03.

**Residual risk:** a genuine build-time regression would now warn rather than fail. Judged
acceptable because the figure measures the GitHub runner and its cache state, not Paladin — the same
build measured 2946 s cold and 44 s warm on consecutive runs.

**Compensating control:** SC5 and REL-05 were amended **at source** with dated provenance scoping
the figure to single-arch, so the criterion is truthful rather than aspirational. The measured
number is still printed on every run.

**Retirement condition:** replace QEMU with native `ubuntu-24.04-arm` runners (free for this public
repository), re-measure, and reinstate a hard, evidence-backed budget. **Owner: Phase 15 / PIPE.**

### AR-04-02 — Kubernetes smoke test measures scheduling, not readiness (from T-04-14 context, low)

The smoke job passes with a 6 s pod-startup figure, but `k8s/deployment.yaml:66-68` runs a
placeholder `sleep 3600` with all three probes commented out (`:137-174`). The budget therefore
proves kind/kubectl orchestration and manifest validity, **not** that a Paladin process serves
traffic. Recorded so the `satisfied` verdict is not read as more than it is.
**Owner: Phase 14 / WEB** (real readiness-probe wiring).

---

## Out of Scope — owned elsewhere

| Item | Owner |
|---|---|
| RustSec suppression owner/expiry schema; the 2026-09-30 risk acceptance | SEC-01 (Phase 9) / SUPPLY-02 (Phase 12) |
| Licence three-way (`Cargo.toml` MIT vs signed checklist `MIT OR Apache-2.0`) | SEC-02 (Phase 9) |
| Duplicate `Security Audit` job, `ci.yml:389-406` | SUPPLY-01 (Phase 12) |
| `API Surface Tracking` job — still red, pre-existing, not an SC5 gate | DEBT-01 (Phase 8) |
| Pinning `dtolnay/rust-toolchain` to a commit SHA to prevent recurrence of the `@master` drift | PIPE-04 (Phase 15) |
| `Dockerfile:10` pins `rust:1.93-slim-bookworm` while `rust-toolchain.toml` pins `1.97.1` | **un-owned — needs triage** |

---

## Audit Trail

### Security Audit 2026-08-03

| Metric | Count |
|--------|-------|
| Threats in register | 36 |
| Closed | 36 |
| Open (at/above `high`) | **0** |
| Realized and re-closed | 2 (T-04-24 ×2, T-04-15) |
| Accepted risks | 2 (AR-04-01, AR-04-02) |

**Method.** `register_authored_at_plan_time: true`, ASVS L1 — mitigations verified, no new-threat
scan. The critical and high entries were re-derived by command rather than read from the plans:
tag-absence from origin, annotated-tag type, `make release` non-invocation, `tiktoken-rs`
immutability, the marketplace-action diff against the phase base, and the gate suite at the tagged
tree. Two entries (T-04-10, T-04-24) required re-verification because circumstances changed after
their plans were written, and one (T-04-13) was found partially realized and converted to a
documented accepted risk rather than closed.
