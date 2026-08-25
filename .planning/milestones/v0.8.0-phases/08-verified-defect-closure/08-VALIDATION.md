---
phase: 8
slug: verified-defect-closure
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
status: draft
nyquist_compliant: false
wave_0_complete: true
created: 2026-08-06
---

# Phase 8 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Derived from `08-RESEARCH.md` §"Validation Architecture". Every command below was either run in
> this checkout during research or is a direct variant of one that was.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (unit / integration / doc); shell scripts for CI gating; `cargo public-api` and `cargo tree` for public-surface proofs |
| **Config file** | `.github/workflows/ci.yml` (job definitions); `crates/paladin-ports/Cargo.toml` (`[lib] doctest`); root `Cargo.toml` (`[features]`, `[[bin]]`) |
| **Quick run command** | Per-item — see the Per-Criterion Verification Map below |
| **Full suite command** | `cargo test --workspace --offline` → `cargo fmt --check` → `cargo clippy --workspace -- -D warnings` (the CLAUDE.md workspace gate) |
| **Estimated runtime** | Workspace gate ~8–15 min cold, ~2–4 min warm (`target/` is pre-populated at ~14 GB) |

**Offline discipline:** the tree builds and tests offline. Prefer `--offline` on every `cargo`
invocation. The one exception is `rustup toolchain install nightly` (DEBT-01), which reaches
`static.rust-lang.org` and was proven to succeed during research even though `crates.io` returns
HTTP 403.

---

## Sampling Rate

- **After every task commit:** the specific command for that DEBT item from the map below.
- **After every plan wave:** `cargo test --workspace --offline`, `cargo fmt --check`,
  `cargo clippy --workspace -- -D warnings`.
- **Before `/gsd-verify-work`:** all five criterion commands run in sequence and green, plus the
  ADR-0006 coverage floor re-measured (**84% workspace line coverage, hard fail**) via the
  ADR-0006-recorded `cargo llvm-cov` pipeline — not `cargo tarpaulin`.
- **Max feedback latency:** ~240 s warm for the workspace gate; < 30 s for every per-criterion
  command except the doctest and coverage runs.

**Note on the pre-commit hook:** every commit runs `cargo fmt --check` and
`cargo clippy --workspace -D warnings`. Commits are slow but they do pass, and this hook is itself
a per-task sampling point.

---

## Per-Criterion Verification Map

Phase 8 is a defect-closure phase: its acceptance is defined by the ROADMAP's five success
criteria, not by new feature behaviour. Each row is the falsifiable observable for one criterion.

| # | Requirement | Observable | Automated command | Expected | Sampling point |
|---|---|---|---|---|---|
| 1 | DEBT-01 | `check-api-surface.sh` exit code + stdout against the regenerated `.project/current-exports.txt` | `bash scripts/extract-public-api.sh /tmp/api-now.txt && bash scripts/check-api-surface.sh .project/current-exports.txt` | exit 0, "unchanged" | per-task commit (regeneration task) |
| 1b | DEBT-01 | The **negative** direction — an intentional public-API change must fail | add a throwaway `pub fn` to a library crate, re-run the command above, then revert | exit 1, "changed" | phase gate (both directions proven, diff reverted) |
| 1c | DEBT-01 | Zero stale path references remain | `grep -rn 'project/current-exports.txt' scripts/ .github/ \| grep -v '\.project/'` and the same over the five `.project/` requirement documents | no output | per-task commit |
| 1d | DEBT-01 | `check-deprecations.sh` is reachable **and can fail** | run the `api-surface` job's two steps in order; then feed the script a deliberately malformed `#[deprecated]` in a `crates/` file and confirm non-zero exit; revert | step 2 executes; malformed input exits non-zero | per-task commit |
| 2 | DEBT-02 | `#[deprecated]` count, cross-checked against three documents agreeing | `grep -rn '#\[deprecated' src crates \| wc -l` **plus** a read confirming `DEPRECATIONS.md`, `docs/src/api-reference/stable-api.md` and ADR-0022 state the same zero-and-why | `0`, three documents in agreement | phase gate (DEBT-02 close-out) — **`human_judgment: true`**, the three-way agreement is a reading, not a grep |
| 2b | DEBT-02 | The mdbook still builds and link-checks (M11 made linkcheck an error) | the repo's mdbook build/linkcheck command | exit 0, no broken links | per-task commit |
| 3 | DEBT-03 | `paladin-ports` doctests execute and pass | `cargo test --offline -p paladin-ports --doc` | **96 passed, 0 failed, 94 ignored** (the research-measured baseline; equal or better) | per-task commit |
| 3b | DEBT-03 | The workspace doctest run no longer excludes the crate | `grep -c 'exclude paladin-ports' .github/workflows/ci.yml` then `cargo test --offline --workspace --doc` | `0`; workspace count includes `paladin-ports`' 96 | phase gate |
| 4 | DEBT-04 | The library-only build **compiles** (guards against the Herald-consumer gap) | `cargo build --offline --lib --no-default-features` | exit 0 | per-task commit — run **before** criterion 4 proper |
| 4b | DEBT-04 | No CLI dependency in a library-only tree | `cargo tree --offline --no-default-features \| grep -E 'structopt\|colored\|comfy-table'` | no output | per-task commit (after D-13, again after D-14); phase gate, captured **verbatim** in the SUMMARY per D-16 |
| 4c | DEBT-04 | `structopt` is gone from the manifest entirely | `grep -c structopt Cargo.toml` and `grep -rln structopt src/ crates/` | `0` and no files | per-task commit |
| 4d | DEBT-04 | The `paladin` binary still builds **with** the feature | `cargo build --offline --bin paladin --features cli` | exit 0 | per-task commit |
| 4e | DEBT-04 | Nothing downstream assumed the binary builds by default | `Dockerfile:33`, `Dockerfile.chef:74`, `feature-flags.yml:144` ("Verify paladin binary builds without cli feature"), `docs/src/deployment/docker.md` each updated or confirmed unaffected | each site re-read and reconciled | phase gate — **`human_judgment: true`** |
| 5 | DEBT-05 | Exactly one `TokenUsage` definition | `grep -rn 'pub struct TokenUsage' crates src \| wc -l` | `1` | per-task commit |
| 5b | DEBT-05 | The re-exports broke none of the ~182 reference sites | `cargo test --offline --workspace --lib` | pass count ≥ pre-change baseline | per-task commit |
| — | all | Coverage floor not regressed | the ADR-0006 `cargo llvm-cov` pipeline | ≥ **84.00%** workspace line coverage | phase gate |
| — | all | Workspace gate | `cargo test --workspace --offline`, `cargo fmt --check`, `cargo clippy --workspace -- -D warnings` | all exit 0 | every wave merge |

---

## Wave 0 Requirements

**None.** Every command above runs against existing, committed test and CI infrastructure. No new
test file, fixture, or framework install is needed before this phase's tasks can execute.
`wave_0_complete: true` in the frontmatter reflects that.

---

## Manual-Only Verifications

| Behavior | Requirement | Why manual | Test instructions |
|----------|-------------|------------|-------------------|
| `DEPRECATIONS.md`, `stable-api.md` and ADR-0022 tell the same story about deprecation state | DEBT-02 | The "no third state" clause in the requirement is about *agreement between documents*. No grep can prove three prose documents agree; only a reading can. | Read all three. Confirm each states zero current deprecations, names the withdrawal, and cites ADR-0022. Confirm `stable-api.md` **keeps** its forward-looking deprecation *policy* while correcting any claim that deprecations exist today. |
| The `required-features` change is reflected everywhere the `paladin` binary is assumed to build | DEBT-04 / D-13 | A missed Dockerfile stage or CI leg does not fail any command in this phase — it fails the next release. | Re-read `Dockerfile:33`, `Dockerfile.chef:74`, `.github/workflows/feature-flags.yml:144`, `Makefile`, `k8s/`, `docs/src/deployment/docker.md`. Each either updated or explicitly confirmed unaffected, recorded in the SUMMARY. |
| `CHANGELOG.md` records both user-visible changes | DEBT-04 / D-13, D-14 | Judgement about what a downstream consumer needs told. | Confirm entries for (a) `cargo run` no longer building `paladin` without `--features cli`, and (b) `paladin-herald`'s formatters moving behind features. |
| Ledger rows amended in place, dated, superseded text retained | D-23 | Convention compliance (D-00d) is a reading, not a check. | Confirm `.planning/ledgers/milestone-04-06.md` rows 115, 116, 157, 160, 225 each carry the closing evidence and date, with prior text retained. |

---

## Validation Sign-Off

- [ ] All tasks have an `<automated>` verify command or an entry in Manual-Only above
- [ ] Sampling continuity: no 3 consecutive tasks without an automated verify
- [ ] Wave 0 covers all MISSING references — *n/a, no Wave 0 needed*
- [ ] No watch-mode flags
- [ ] Feedback latency < 240 s (warm)
- [ ] Criterion 1 proven in **both** directions (pass on unchanged tree, fail on intentional change)
- [ ] Criterion 4 preceded by a successful `--no-default-features` **build**, not just `cargo tree`
- [ ] ADR-0006's 84% floor re-measured and not regressed
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
