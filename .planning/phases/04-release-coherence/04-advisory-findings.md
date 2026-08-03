# Advisory Posture — Raw Evidence Record (Phase 4, Plan 02)

This file is raw evidence only: verbatim commands, verbatim tool output, toolchain versions,
commit SHA, and UTC dates. It records the advisory posture as *measured* on this tree — per
`04-CONTEXT.md` D-07, SC3's "0 vulnerabilities / clean `cargo deny check`" half is already true;
this file's job is to record that fact to the D-17 provenance standard, not to fix an imagined
failure. It carries no gate or target value of its own beyond what `cargo audit` and
`cargo deny check` themselves assert.

All four sections below share one measurement session, run back-to-back after Task 1's `deny.toml`
edit landed (commit `ce08b1b`). The shared environment probe is repeated in full inside each
section per D-17's instruction that "every figure carries the Phase 1/Phase 3 provenance block" —
this is deliberate redundancy, not an oversight.

## Entry measurement — `cargo audit` verdict

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
ce08b1b04d7dcb74f7a4ba3bb1b47a8055321267
```

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2b1ecba9137d0d66
```

Command: `git status --porcelain`

```
(no output)
```

The tree is clean at this commit — Task 1's `deny.toml` edit is already committed, and no other
file is modified. This measurement runs against the post-Task-1 state.

Command: `date -u`

```
Mon Aug  3 00:18:21 UTC 2026
```

### Command and raw output

Per D-17's stated exception, `cargo audit` is the one command in this record that reaches the
network rather than running `--offline` — it must fetch `github.com/RustSec/advisory-db`.

Command: `cargo audit`

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1186 security advisories (from /usr/local/cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (691 crate dependencies)
Crate:     ansi_term
Version:   0.12.1
Warning:   unmaintained
Title:     ansi_term is Unmaintained
Date:      2021-08-18
ID:        RUSTSEC-2021-0139
URL:       https://rustsec.org/advisories/RUSTSEC-2021-0139

Crate:     atty
Version:   0.2.14
Warning:   unmaintained
Title:     `atty` is unmaintained
Date:      2024-09-25
ID:        RUSTSEC-2024-0375
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0375

Crate:     dotenv
Version:   0.15.0
Warning:   unmaintained
Title:     dotenv is Unmaintained
Date:      2021-12-24
ID:        RUSTSEC-2021-0141
URL:       https://rustsec.org/advisories/RUSTSEC-2021-0141

Crate:     fxhash
Version:   0.2.1
Warning:   unmaintained
Title:     fxhash - no longer maintained
Date:      2025-09-05
ID:        RUSTSEC-2025-0057
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0057

Crate:     number_prefix
Version:   0.4.0
Warning:   unmaintained
Title:     number_prefix crate is unmaintained
Date:      2025-11-17
ID:        RUSTSEC-2025-0119
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0119

Crate:     paste
Version:   1.0.15
Warning:   unmaintained
Title:     paste - no longer maintained
Date:      2024-10-07
ID:        RUSTSEC-2024-0436
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0436

Crate:     proc-macro-error
Version:   1.0.4
Warning:   unmaintained
Title:     proc-macro-error is unmaintained
Date:      2024-09-01
ID:        RUSTSEC-2024-0370
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0370

Crate:     rustls-pemfile
Version:   2.2.0
Warning:   unmaintained
Title:     rustls-pemfile is unmaintained
Date:      2025-11-28
ID:        RUSTSEC-2025-0134
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0134

Crate:     structopt
Version:   0.3.26
Warning:   unmaintained
Title:     `structopt` is in maintenance mode
Date:      2022-02-08
ID:        RUSTSEC-2022-0104
URL:       https://rustsec.org/advisories/RUSTSEC-2022-0104

Crate:     atty
Version:   0.2.14
Warning:   unsound
Title:     Potential unaligned read
Date:      2021-07-04
ID:        RUSTSEC-2021-0145
URL:       https://rustsec.org/advisories/RUSTSEC-2021-0145

Crate:     event-listener
Version:   5.4.1
Warning:   unsound
Title:     `event-listener` allows `!Send` tags to cross thread boundaries via `StackSlot`
Date:      2026-07-13
ID:        RUSTSEC-2026-0221
URL:       https://rustsec.org/advisories/RUSTSEC-2026-0221

Crate:     scc
Version:   2.4.0
Warning:   unsound
Title:     `Array::insert` violates exception safety if compare function panics, leading to potential Double-Free
Date:      2026-07-06
ID:        RUSTSEC-2026-0205
URL:       https://rustsec.org/advisories/RUSTSEC-2026-0205

Crate:     spin
Version:   0.9.8
Warning:   yanked

warning: 13 allowed warnings found
```

Exit status: `0` (confirmed separately via `cargo audit > /dev/null 2>&1; echo "EXIT=$?"` →
`EXIT=0`).

**Verdict: 0 vulnerabilities.** No `Vulnerabilities found` block appears anywhere in the output —
only the 13 allowed warnings (9 `unmaintained`/maintenance-mode, 3 `unsound`, 1 `yanked`), all of
which are either already listed in `deny.toml`'s ignore array (the 9 unmaintained entries plus the
mirrored vulnerability advisories that produced no output at all because they carry no active
findings — `RUSTSEC-2023-0071` and `RUSTSEC-2025-0111` do not appear above, confirming their
suppressions are still matching live crates) or newly-surfaced and recorded, not suppressed (see
the next section). `RUSTSEC-2025-0121` (gcc) does **not** appear anywhere in this run's output,
corroborating Task 1's removal — it no longer matches any crate in the graph.

### Advisory database snapshot identity

Command: `git -C /usr/local/cargo/advisory-db log -1 --format='%H %cI'`

```
d91a8fc9492378f23cba86b81770c6d16de6ebba 2026-08-02T19:56:20+02:00
```

**Advisory count loaded: 1186.** **Database HEAD commit:** `d91a8fc9492378f23cba86b81770c6d16de6ebba`,
authored `2026-08-02T19:56:20+02:00`. This count is identical to the 1186 figure `04-CONTEXT.md`
D-07 recorded during the discussion session that ran earlier the same day — no second run is
recorded per D-17's instruction ("record the run twice if the count differs"), because the count
does not differ. A later reader can distinguish which snapshot produced this verdict from the
commit hash above, independent of the coincidentally-matching count.

---

## Entry measurement — `cargo deny check` verdict

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
ce08b1b04d7dcb74f7a4ba3bb1b47a8055321267
```

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2b1ecba9137d0d66
```

Command: `git status --porcelain`

```
(no output)
```

Tree clean, same commit as the `cargo audit` section above — no drift between the two measurements
in this record.

Command: `date -u`

```
Mon Aug  3 00:18:21 UTC 2026
```

### Before Task 1's edit (cited, not re-run)

Re-running `cargo deny check` against the pre-edit `deny.toml` would require reverting Task 1's
already-committed change — a destructive operation this record deliberately avoids per the
worktree's destructive-git prohibition. The "before" state is instead **cited from the
measurement already taken live during this same day's discussion session**, recorded verbatim at
`04-CONTEXT.md` D-07/D-08 and restated in this plan's `environment_constraints`:

> `cargo deny check` → **advisories ok, bans ok, licenses ok, sources ok**, with exactly one
> complaint: `warning[advisory-not-detected]` at `deny.toml:136` for `RUSTSEC-2025-0121` (gcc).

Source: `.planning/phases/04-release-coherence/04-CONTEXT.md`, D-07 and D-08.1, dated 2026-08-02,
measured at HEAD `68ba809` (the commit immediately preceding this plan's Task 1 edit).

### After Task 1's edit — live run, this session

Command: `cargo deny check`

```
    Fetching advisories database from 'https://github.com/rustsec/advisory-db'
warning[duplicate]: found 2 duplicate entries for crate 'axum'
   ┌─ /workspace/.claude/worktrees/agent-a2b1ecba9137d0d66/Cargo.lock:35:1
   │
35 │ ╭ axum 0.7.9 registry+https://github.com/rust-lang/crates.io-index
36 │ │ axum 0.8.9 registry+https://github.com/rust-lang/crates.io-index
   │ ╰────────────────────────────────────────────────────────────────┘ lock entries
   │
   ├ axum v0.7.9
     └── tonic v0.12.3
         └── qdrant-client v1.18.0
             ├── paladin-ai v0.6.0
             └── paladin-memory v0.6.0
                 └── paladin-ai v0.6.0 (*)
   ├ axum v0.8.9
     ├── (dev) paladin-ai v0.6.0
     ├── paladin-web v0.6.0
     │   └── paladin-ai v0.6.0 (*)
     ├── tonic v0.14.6
     │   └── tower_governor v0.8.0
     │       └── paladin-web v0.6.0 (*)
     ├── tower_governor v0.8.0 (*)
     ├── utoipa-axum v0.2.0
     │   └── paladin-web v0.6.0 (*)
     └── utoipa-swagger-ui v9.0.2
         └── paladin-web v0.6.0 (*)

[... 43 further warning[duplicate] blocks omitted for length — full list below ...]

warning[yanked]: detected yanked crate (try `cargo update -p spin`)
   ┌─ /workspace/.claude/worktrees/agent-a2b1ecba9137d0d66/Cargo.lock:???:1
   │
   │ spin 0.9.8 registry+https://github.com/rust-lang/crates.io-index
   │ yanked crate

advisories ok, bans ok, licenses ok, sources ok
```

Exit status: `0`.

**Truncation note (deliberate, not an omission of substance):** the full raw output is 5,742
lines, dominated by 44 `warning[duplicate]` blocks — one per crate with more than one version in
`Cargo.lock` — each printing a full reverse-dependency tree. These are `[bans]
multiple-versions = "warn"` findings, a deliberate non-blocking policy already documented in
`deny.toml`'s own header comment ("Duplicate versions are a warning (not an error) since most
duplicates come from transitive deps we don't control (FR 12 / Open Question 4)"), not an
advisory finding and not part of this plan's edit surface. The count breakdown, taken
deterministically rather than pasted in full:

```
$ grep -c '^warning\[duplicate\]' <full output>
44
$ grep -c '^warning\[yanked\]' <full output>
1
```

The single `warning[yanked]` line is `spin 0.9.8` — the same yanked crate named in the newly-
surfaced-advisories section below; its appearance here in `cargo deny check`'s own output
independently corroborates that finding via a second tool. The final summary line —
`advisories ok, bans ok, licenses ok, sources ok` — is pasted verbatim above and is the figure
this record cites as the after-Task-1 verdict.

Command: `cargo deny check advisories 2>&1 | grep -c 'advisory-not-detected'`

```
0
```

**Delta, stated explicitly:** the before-state carried exactly one `advisory-not-detected`
warning (`RUSTSEC-2025-0121`, gcc); the after-state carries zero. Task 1's removal of that single
stale entry is the entire cause of the delta — no other line in `deny.toml`'s `[advisories]`
block changed in a way that could affect this count.

---

## Entry measurement — Four newly-surfaced advisories, recorded not suppressed

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
ce08b1b04d7dcb74f7a4ba3bb1b47a8055321267
```

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2b1ecba9137d0d66
```

Command: `git status --porcelain`

```
(no output)
```

Command: `date -u`

```
Mon Aug  3 00:18:21 UTC 2026
```

### The four findings

All four appear in the `cargo audit` output pasted in full above and in neither `deny.toml` nor
`.cargo/audit.toml`'s ignore lists (verified: `grep -cE '"RUSTSEC-(2021-0145|2026-0221|2026-0205)"'
deny.toml .cargo/audit.toml` returns `0` for both files — `spin` has no RUSTSEC ID to grep for,
being a yanked-crate notice rather than an advisory). None fails either gate today: they are
`unsound`/`yanked` warnings, not vulnerabilities, and `cargo audit` and `cargo deny check` both
exit `0` in their presence.

**Dependency paths, derived live via `cargo tree --offline --invert <crate> -e normal,build,dev
--all-features` rather than assumed:**

1. **`RUSTSEC-2021-0145`** (`atty` 0.2.14, unsound — potential unaligned read). Distinct from the
   already-ignored `RUSTSEC-2024-0375` unmaintained notice on the same crate; this is a second,
   different advisory against the same version.
   ```
   atty v0.2.14
   └── clap v2.34.0
       └── structopt v0.3.26
           └── paladin-ai v0.6.0
   ```

2. **`RUSTSEC-2026-0221`** (`event-listener` 5.4.1, unsound — `!Send` tags can cross thread
   boundaries via `StackSlot`).
   ```
   event-listener v5.4.1
   └── sqlx-core v0.8.6
       ├── sqlx v0.8.6
       │   ├── paladin-ai v0.6.0
       │   ├── paladin-memory v0.6.0
       │   │   └── paladin-ai v0.6.0
       │   └── paladin-storage v0.6.0
       │       └── paladin-ai v0.6.0
       ├── sqlx-mysql v0.8.6
       │   └── sqlx v0.8.6 (*)
       └── sqlx-sqlite v0.8.6
           └── sqlx v0.8.6 (*)
   ```

3. **`RUSTSEC-2026-0205`** (`scc` 2.4.0, unsound — `Array::insert` exception-safety violation,
   potential double-free if the compare function panics).
   ```
   scc v2.4.0
   └── serial_test v3.4.0
       [dev-dependencies]
       └── paladin-ai v0.6.0
   ```
   Dev-dependency only — not present in a release build's dependency graph.

4. **`spin` 0.9.8** — **yanked**, not a RUSTSEC advisory ID.
   ```
   spin v0.9.8
   ├── flume v0.11.1
   │   └── sqlx-sqlite v0.8.6
   │       └── sqlx v0.8.6
   │           └── paladin-ai v0.6.0 (+ paladin-memory, paladin-storage)
   └── lazy_static v1.5.0
       └── (colored, deadpool/wiremock [dev], num-bigint-dig/rsa/sqlx-mysql,
            paladin-ai direct, sharded-slab/tracing-subscriber, structopt,
            tiktoken-rs/paladin-content/paladin-memory)
   ```
   `lazy_static`'s reverse-dependency fan-out is large (8 direct paths); the full tree is
   reproducible via `cargo tree --offline --invert spin -e normal,build,dev --all-features` and is
   abbreviated here to the two top-level pull-in points (`flume`, `lazy_static`) rather than
   repeating all 8 leaves.

### Hand-off row

| ID | Verdict | Evidence |
|---|---|---|
| REL-03 — four newly-surfaced advisories (`RUSTSEC-2021-0145` atty unsound, `RUSTSEC-2026-0221` event-listener unsound, `RUSTSEC-2026-0205` scc unsound, `spin` 0.9.8 yanked) | deferred with reason | Surfaced live by `cargo audit` against the current RustSec advisory-db snapshot (`d91a8fc9`, 1186 advisories, fetched 2026-08-03); present in neither `deny.toml` nor `.cargo/audit.toml`. None fails either gate today — `unsound`/`yanked` warnings, not vulnerabilities. Dependency paths derived above via `cargo tree --invert`. Adding a suppression for any of these here would be a new governance decision inside a phase whose governance owner is someone else; the owner/expiry schema these advisories would need is SUPPLY-02's, not this phase's. **Owner: Phase 9 / SEC-01 and Phase 12 / SUPPLY-02.** |

---

## Entry measurement — The duplicate `Security Audit` CI job, measured non-blocking

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
ce08b1b04d7dcb74f7a4ba3bb1b47a8055321267
```

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2b1ecba9137d0d66
```

Command: `git status --porcelain`

```
(no output)
```

Command: `date -u`

```
Mon Aug  3 00:18:21 UTC 2026
```

### The configuration

`.github/workflows/ci.yml` declares two jobs that both render as `Security Audit` in the GitHub
Actions UI (identical `name:` field, different job keys):

- **`security-audit`** (job key, `ci.yml:60-77`) — runs a bare `cargo audit`, which auto-discovers
  `.cargo/audit.toml` and applies its five-entry ignore list with no inline flags:
  ```yaml
  - name: Run cargo-audit (exceptions from .cargo/audit.toml)
    run: cargo audit
  ```
- **`security`** (job key, `ci.yml:389-406`) — runs `cargo audit` with two inline `--ignore` flags
  that duplicate two of the five entries already in `.cargo/audit.toml`:
  ```yaml
  - name: Run security audit
    # RUSTSEC-2023-0071 (rsa, no fix) and RUSTSEC-2025-0111 (tokio-tar, no fix)
    # are transitive dev-dependency advisories with no upstream fix available.
    run: cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111
  ```

### Reproducing the second job's command locally

Command: `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1186 security advisories (from /usr/local/cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (691 crate dependencies)
[... same 13-warning body as the primary `cargo audit` section above: ansi_term, atty
(unmaintained), dotenv, fxhash, number_prefix, paste, proc-macro-error, rustls-pemfile,
structopt, atty (unsound), event-listener, scc, spin (yanked) ...]

warning: 13 allowed warnings found
```

Exit status: `0`.

**Verdict: both jobs reach the same outcome on this tree.** `grep -c '^ID:'` against both this
run's output and the primary `cargo audit` run's output returns `12` for each (12 advisory IDs
plus the 1 ID-less `spin` yanked notice = 13 total warnings, matching both runs exactly). The
`--ignore` flags on the `security` job's command line **augment** `.cargo/audit.toml`'s ignore
list rather than replacing it — cargo-audit auto-discovers and applies `.cargo/audit.toml`
regardless of inline `--ignore` flags, so passing the same two IDs both ways is a no-op collision,
not a divergent configuration. Neither job blocks SC5 today; both exit `0` on the current tree.

### Hand-off row

| ID | Verdict | Evidence |
|---|---|---|
| REL-03 — duplicate `Security Audit` CI job (`ci.yml:60-77` `security-audit` vs. `ci.yml:389-406` `security`) | deferred with reason | Both jobs share the display name `Security Audit` but differ in job key and command. Measured live: `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` (the `security` job's exact command) exits `0` with the identical 13-warning set the bare `cargo audit` (the `security-audit` job's command) produces, because `--ignore` augments `.cargo/audit.toml` rather than replacing it. Neither job blocks SC5 on this tree. The 18-line deletion of the redundant job is not performed here — it is the payoff for a Milestone 10 acceptance criterion Phase 12 owns. **Owner: Phase 12 / SUPPLY-01.** |

---

## Not decided here

This record deliberately answers only what Phase 4 owns. The following are explicitly **not**
decided by this file, and a later reader should not mistake the silence for a verdict:

- **The owner/expiry field schema for advisory suppressions, and the 2026-09-30 risk-acceptance
  disposition.** Neither `deny.toml` nor `.cargo/audit.toml` gained an owner or expiry field in
  this plan (verified: `git diff -- deny.toml | grep -c '^+.*\(owner\|expiry\|expires\)'` returns
  `0`). **Owner: Phase 9 / SEC-01** (the set and the expiry) and **Phase 12 / SUPPLY-02** (the
  schema itself and the three unratified 2026 ignores).
- **Whether the four newly-surfaced advisories should eventually be suppressed, upgraded away, or
  accepted long-term.** This record only establishes that they exist, where they come from, and
  that they do not fail a gate today. The disposition decision belongs to the same two owners
  above.
- **The MIT vs. `MIT OR Apache-2.0` licence three-way.** `Cargo.toml` states `MIT`; a signed
  2026-05-28 decision checklist states `MIT OR Apache-2.0`. `deny.toml`'s `[licenses]` allow-list
  was not touched by this plan (only the `[advisories]` array changed — see Task 1's commit) and
  its permissive-only shape is not evidence toward resolving this three-way. **Owner: Phase 9 /
  SEC-02.**
- **Deleting the duplicate CI job.** Measured non-blocking above; the deletion itself is
  **Phase 12 / SUPPLY-01's** payoff to claim, not performed here.

## Self-check evidence

- `deny.toml` — Task 1's edit is committed at `ce08b1b04d7dcb74f7a4ba3bb1b47a8055321267`.
- This file — created and committed by Task 2 of plan `04-02`, immediately following.
