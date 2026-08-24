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
| 16-01 T1 | 16-01 | 1 | DOCS-01 | T-16-01, T-16-SC | Every mdbook tool pinned `--locked --version` to `docs.yml:44-54`; no floating version in either image | integration (mdbook) | `mdbook build docs/ && git diff --exit-code docs/book.toml` | ✅ after W1 | ⬜ pending |
| 16-01 T2 | 16-01 | 1 | DOCS-01 | T-16-05, T-16-07 | Every verdict carries its producing command; a `current` row with an empty Findings cell is invalid | content-diff + integration | `test "$(grep -c '^\| docs/src/' 16-DOCS-01-VERDICTS.md)" = 14 && mdbook build docs/` | ✅ after W1 | ⬜ pending |
| 16-02 T1/T2 | 16-02 | 2 | DOCS-01 | T-16-05, T-16-08 | Cited source paths resolved before the row is written | content-diff + path resolution | `for p in $(grep -ohE '(crates\|src)/[A-Za-z0-9_/.-]+\.rs' <files> \| sort -u); do test -f "$p"; done && mdbook build docs/` | ✅ | ⬜ pending |
| 16-03 T1/T2 | 16-03 | 3 | DOCS-01 | T-16-05, T-16-07 | Every documented config key and MCP transport proven against the shipped tree | content-diff + path resolution | same battery, plus `test -z "$(git diff --name-only -- '*.rs')"` | ✅ | ⬜ pending |
| 16-04 T1/T2 | 16-04 | 4 | DOCS-01 | T-16-08, T-16-10 | `paladin-*` tokens classified crate-vs-Kubernetes **before** edit; removed scanner not left recommended | content-diff + make-target resolution | `test "$(grep -c 'v0\.4\.3' docs/src/deployment/docker.md)" = 0` + Makefile target loop | ✅ | ⬜ pending |
| 16-05 T1/T2/T3 | 16-05 | 5 | DOCS-01 | T-16-05, T-16-11, T-16-12 | Superseded ledger text retained beside the dated amendment | content-diff + integration | `test "$(grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md)" = 0 && mdbook build docs/` | ✅ | ⬜ pending |
| 16-06 T1/T2 | 16-06 | 2 | DOCS-02 | T-16-13, T-16-14, T-16-15, T-16-16 | ADR + PROMOTION advance in one commit; the 311 lines and the TOC entry survive archiving | manual (ADR) + measured metric | `grep -ric --include=*.md -w sentinel docs/src/architecture/` ≥ 1 and `test "$(wc -l < docs/src/appendix/design-and-architecture.md)" -gt 311` | ❌ W0 → ✅ after 16-06 | ⬜ pending |
| 16-07 T1/T2 | 16-07 | 2 | DOCS-03 | T-16-17, T-16-18, T-16-19, T-16-21 | No rustdoc lint suppression added; no visibility widened; workflow files untouched | smoke (compiler diagnostic) | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` | ✅ (`ci.yml:63`) | ⬜ pending |
| 16-07 T2 | 16-07 | 2 | DOCS-03 | T-16-20 | ADR-0033's stale Finding 1 claim retained beside its dated correction | smoke | `cargo doc -p paladin-herald --no-deps 2>&1 \| grep -c "warning:"` (expect 0, per M-07) | ✅ | ⬜ pending |
| 16-08 T1/T2 | 16-08 | 2 | DOCS-03 | T-16-22, T-16-23, T-16-24 | Gate fails loudly on a degenerate input; `--list` labels itself not-a-gate | unit (phase-authored script) | `shellcheck --severity=warning scripts/check-public-api-examples.sh && ! bash scripts/check-public-api-examples.sh` (failing baseline is the expected result here) | ❌ W0 → ✅ after 16-08 | ⬜ pending |
| 16-09 T1/T2 | 16-09 | 3 | DOCS-03 | T-16-17, T-16-18, T-16-26, T-16-27 | Examples executable and offline; no `ignore`/`text` fence added; only doc-comment lines in the diff | unit (doctest) | `cargo test --doc -p paladin-ports` | ✅ | ⬜ pending |
| 16-10 T1/T2 | 16-10 | 4 | DOCS-03 | T-16-17, T-16-18, T-16-26, T-16-28 | Feature-gated examples gated with their type; herald still warning-free after the 16-07 flip | unit (doctest) | `cargo test --doc -p paladin-core -p paladin-memory -p paladin-battalion -p paladin-herald` | ✅ | ⬜ pending |
| 16-11 T1/T2 | 16-11 | 5 | DOCS-03 | T-16-29, T-16-26, T-16-17, T-16-30 | No credential-shaped literal in any published example; offline implementors first | unit (doctest) + source assertion | `cargo test --doc -p paladin-llm -p paladin-storage -p paladin-web -p paladin-content -p paladin-notifications && cargo test --doc -p paladin-ai` | ✅ | ⬜ pending |
| 16-12 T1/T2 | 16-12 | 6 | DOCS-03 | T-16-31, T-16-17, T-16-24, T-16-32 | The gate script is byte-unchanged by the plan that must satisfy it | unit (script) + smoke + doctest | `bash scripts/check-public-api-examples.sh && cargo test --workspace --doc && git diff --exit-code scripts/check-public-api-examples.sh` | ✅ after 16-08 | ⬜ pending |
| 16-13 CP1/CP2/T3 | 16-13 | 2 | DOCS-04 | T-16-02, T-16-33, T-16-01, T-16-34, T-16-35, T-16-SC | `vhs`/`ttyd` gated behind a non-auto-approvable human fingerprint check; keyring-scoped `signed-by=`; size budget decided before any binary | manual checkpoint + smoke | `command -v vhs && command -v ttyd && command -v ffmpeg && command -v asciinema` plus a `signed-by=` / no-`apt-key add` grep over both Dockerfiles | ❌ W0 → ✅ after 16-13 | ⬜ pending |
| 16-14 T1 | 16-14 | 3 | DOCS-04 | T-16-03, T-16-36, T-16-35, T-16-16 | Only mock-backed offline examples recorded; artifacts verified by content before an irreversible commit | smoke (manual run + content check) | `cargo run --example basic_paladin` (exit 0, offline — M-08) plus GIF magic-byte and asciicast JSON-parse checks per artifact | ✅ | ⬜ pending |
| 16-14 T2/T3 | 16-14 | 3 | DOCS-04 | T-16-37, T-16-11 | README gains exactly one line; superseded requirement text retained | integration + source assertion | `test "$(git diff --numstat README.md \| awk '{print $1"/"$2}')" = "1/0" && mdbook build docs/` | ❌ W0 → ✅ after 16-14 | ⬜ pending |

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
