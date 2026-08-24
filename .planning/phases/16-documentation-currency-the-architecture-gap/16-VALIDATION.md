---
phase: 16
slug: documentation-currency-the-architecture-gap
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-24
---

# Phase 16 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded by `/gsd-plan-phase 16` from `16-RESEARCH.md` § Validation Architecture.
> **This is a documentation-content phase** — no unit-test framework applies. The
> feedback loop is the compiler's doc pass, the mdbook build, and two phase-authored
> scripts.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test --workspace --doc` (rustdoc doctests) + `mdbook build` (linkcheck) + phase-authored shell scripts |
| **Config file** | `docs/book.toml` (linkcheck, `warning-policy = "error"`); no dedicated test config for the doc-currency checks |
| **Quick run command** | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` |
| **Full suite command** | `cargo test --workspace --doc && mdbook build docs/ && ./scripts/check-public-api-examples.sh` |
| **Estimated runtime** | ~90–180 seconds (cold `cargo doc` dominates) |

**Toolchain note (M-10 + plan-phase measurement).** `mdbook`, `mdbook-linkcheck`,
`mdbook-mermaid`, `asciinema`, `vhs`, `ttyd`, `ffmpeg` and `go` are all absent locally.
Network egress *was* verified from the planning environment on 2026-08-24 — `index.crates.io`
`200`, a real `mdbook-0.4.40.crate` tarball `200`, `repo.charm.sh` `200`, `github.com` `200` —
so `cargo install --locked --version <pin>` is viable. Pin to CI's exact versions at
`.github/workflows/docs.yml:44-54`: **mdbook 0.4.40, mdbook-mermaid 0.13.0,
mdbook-linkcheck 0.7.7**.

---

## Sampling Rate

- **After every task commit:**
  - any `.rs` doc-comment edit → `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`
  - any `docs/src/**` edit → `mdbook build docs/`
- **After every plan wave:** `cargo test --workspace --doc && mdbook build docs/ && ./scripts/check-public-api-examples.sh`
- **Before `/gsd-verify-work`:** Full suite green, **plus** the D-09 per-file verdict record
  complete for all fourteen DOCS-01 files **and** the D-05 79-entry-point `# Examples` check passing.
- **Max feedback latency:** ~180 seconds

**`workflow.worktree_skip_hooks: true` (D-00o)** — commits do **not** cold-compile the workspace.
Sampling here is therefore not automatic on commit; each task must run its command explicitly.

---

## Per-Task Verification Map

*Seeded at the requirement level. `/gsd-planner` populates Task IDs / Plan / Wave columns as
plans are written; `/gsd-validate-phase` promotes this file to `status: validated`.*

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | DOCS-01 | — | N/A | content-diff (manual, evidence-recorded) | no single command — per-signal greps recorded in the D-09 verdict record | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | DOCS-01 | — | N/A | integration (mdbook) | `mdbook build docs/` | ✅ | ⬜ pending |
| TBD | TBD | TBD | DOCS-02 | — | N/A | manual (ADR review) | n/a — ADR-0047 authorship, not a test | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | DOCS-03 | — | N/A | smoke (compiler diagnostic) | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` | ✅ (`ci.yml:63`) | ⬜ pending |
| TBD | TBD | TBD | DOCS-03 | — | N/A | unit (phase-authored script) | `./scripts/check-public-api-examples.sh` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | DOCS-03 | — | N/A | smoke | `cargo doc -p paladin-herald --no-deps 2>&1 \| grep -c warning` (expect 0, per M-07) | ✅ | ⬜ pending |
| TBD | TBD | TBD | DOCS-04 | T-16-01 | Every new tool installed with an explicit pin (`--locked --version`); `vhs`/`ttyd` gated behind `checkpoint:human-verify` | smoke (manual run + recording) | `cargo run --example basic_paladin` (exit 0, offline — verified per M-08) | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `scripts/check-public-api-examples.sh` — phase-authored script asserting all 79 D-05 entry
      points carry a `# Examples` block. **No stable-Rust lint exists for this**:
      `rustdoc::missing_doc_code_examples` is nightly-only and feature-gated, and this project pins
      `dtolnay/rust-toolchain@stable`. A script is the honest fallback, not a workaround.
- [ ] The D-09 per-file verdict record artifact — a new `.md` under the phase directory and/or a
      ledger amendment (Claude's Discretion: which, or both).
- [ ] `docs/DEMOS.md` — does not exist (M-09).
- [ ] `docs/assets/recordings/` — does not exist; `docs/assets/` itself is absent (M-09).
      Not to be confused with `docs/src/assets/`, which exists and holds six architecture SVGs.
- [ ] `.tape` scripts for the four D-16 demos — none exist under any path.
- [ ] Doc toolchain install in both devcontainer images: mdbook 0.4.40, mdbook-mermaid 0.13.0,
      mdbook-linkcheck 0.7.7, plus `vhs`, `ttyd`, `ffmpeg` and **`asciinema`** (the last named by
      the 2026-08-24 D-14 amendment).

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Fourteen files are *current against the tree* | DOCS-01 | Currency is a content judgement, not a predicate. M-06 warns a naive `paladin-*` crate-name sweep is mostly false positives — most such tokens in `kubernetes.md` are Kubernetes object names, not crate names. Success criterion 1 states outright it is settled by content, never by file existence or mtime. | Per file, run the per-signal greps (version strings, `make` targets, workflow names, cited `crates/…/*.rs` paths, feature flags), read the surrounding prose, and record a **current** or **updated** verdict with the exact command or `file:line` that produced it (D-00e). |
| Architecture disposition is *clear to a reader* | DOCS-02 | Whether a developer looking for Sentinel finds it, or finds a clear pointer to where to look, is a comprehension outcome. | Read `docs/src/appendix/design-and-architecture.md` end-to-end after the change and confirm it either documents the subsystem or states plainly that it is historical, naming the live chapter. |
| Demo recordings are *watchable and correct* | DOCS-04 | A `.gif`/`.cast` renders correctly or it does not; no assertion covers legibility, pacing, or whether the terminal output shown is the real one. | Play each of the four recordings; confirm the output matches a live `cargo run --example <name>` and that no credential or key appears on screen. |
| `vhs` / `ttyd` supply-chain trust | DOCS-04 | Neither is a crates.io/npm/PyPI package, so the automated package-legitimacy gate cannot reach them. VHS's documented APT setup uses a `curl \| gpg --dearmor` key install. | `checkpoint:human-verify` before either lands in a devcontainer image: verify the Charm GPG key fingerprint out-of-band against Charm's published key. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 180s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
