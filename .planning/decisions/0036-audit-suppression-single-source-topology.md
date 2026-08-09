# ADR-0036: The audit-suppression single-source topology invariant

## Status

Accepted

**Date:** 2026-08-09

## Context

The measured fact this ADR is written against, re-run in this execution rather than copied from an
earlier artefact: `./scripts/check-workflow-suppressions.sh` exits `0` against the current
`.github/workflows/` tree, reporting `6 workflow file(s) scanned, 109 run step(s) examined, 1 cargo
audit invocation(s) found; no inline advisory-ignore suppression detected.` The full transcript,
plus the ignore-family token census that confirms it, is recorded in `## Code Conformance` below.

The invariant being promoted has two framings in its own source document, and they are not
identical. `Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md`
FR-1 (`:71-73`) states the narrower form: "The CI `security-audit` job must invoke `cargo audit`
such that its ignore-list is sourced from the version-controlled `audit.toml` (single source of
truth) rather than inline `--ignore` flags." Read in isolation, FR-1 names one file. The same
document's §8 Success Metrics (`:187`) states the broader form: "`audit.toml` and `deny.toml` are
the only places policy/exceptions are defined; no inline advisory-ignore flags remain in CI." §8
names two files, because `deny.toml` carries its own `[advisories] ignore` array (nine
`unmaintained`-class entries `cargo audit` cannot consume at all — ADR-0024 `## Decision` item 1)
that FR-1's single-file framing does not account for. **This ADR adopts §8's two-file framing and
says so explicitly, rather than silently picking the narrower FR-1 reading**, because a topology
invariant that only watches `.cargo/audit.toml` would leave `deny.toml`'s `cargo deny` invocations
free to carry an inline suppression undetected — exactly the gap the FR-1-only reading would
reopen.

`PROMOTION.md` Part B candidate 7 recorded this invariant as "currently violated by the tree" at
the time of ingest run 5. **That is no longer true.** The guard transcript above is the measurement
that establishes it: zero inline advisory-ignore suppressions exist anywhere under
`.github/workflows/`, and the one surviving `cargo audit` invocation (`ci.yml:78`) is the bare,
config-driven form SUPPLY-01 left behind when Phase 9 deleted the duplicate `security:` job
(`ci.yml:465-482` in the pre-deletion tree, commit `cb75b2b`). The violation this ADR was written
to close was already closed by that deletion; what remained open was recording the invariant
somewhere that outranks the PRD text that still calls it a live violation.

ADR-0024 (`.planning/decisions/0024-rustsec-exception-governance.md`) and this ADR answer two
different questions about the same suppression system, and the division is meant to be quotable: **a
reader asking "who owns this advisory — which `RUSTSEC-*` ID, whose sign-off, reviewed when?" goes
to ADR-0024; a reader asking "may I add an `--ignore` flag to a workflow step?" goes here.** ADR-0024
owns suppression *contents* — its `## Decision` names `.cargo/audit.toml` as the authoritative
surface for `cargo audit`, `SECURITY-EXCEPTIONS.md` as the authoritative governance register, and
sets per-advisory owner and review-date fields for all ten live suppressions. This ADR owns
suppression *topology* — which mechanical surfaces (files, workflow steps) are legally allowed to
carry a suppression expression at all, independent of which advisories currently populate them. This
ADR never restates which advisories are suppressed, why, or who owns them; every one of those facts
stays in ADR-0024 and in `SECURITY-EXCEPTIONS.md`, cited here by number and by path only.

## Decision

Under this corpus's precedence order — ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox — an ADR that restates an invariant the tree
already satisfies ratifies a true state rather than instructing a code change, the same posture
ADR-0031 took for the extracted-crate dependency rule.

**The restated rule, as a single quotable sentence:** *an advisory suppression may be expressed
only in `.cargo/audit.toml` or `deny.toml`; no workflow file under `.github/workflows/` may carry
an inline advisory-ignore flag on a `cargo audit` or `cargo deny` invocation.*

Three sub-decisions, kept visually separate:

**(i) The enforceable invariant.** A violation looks like exactly one shape: a `run:` step in any
`.github/workflows/*.yml`/`*.yaml` file whose logical command line matches both a `cargo audit` or
`cargo deny` invocation and an `--ignore`-family flag on the same line. This is what
`scripts/check-workflow-suppressions.sh`'s co-occurrence match (`CARGO_GATE_RE` and
`IGNORE_FLAG_RE`) tests structurally, per logical line, not by scraping comment prose. Two
already-present ignore-family tokens are explicitly not violations under this test, because neither
co-occurs with a `cargo audit`/`cargo deny` invocation on its line: `mc mb ... --ignore-existing`
(a MinIO client flag, `ci.yml:431-432`) and `cargo test ... -- --ignored` (Rust's own test-harness
opt-in flag, `ci.yml:466,469`, and elsewhere). The regex-boundary reasoning that keeps both silent
is recorded in the guard script's own header comment, not repeated here.

**(ii) Scope: topology only, never contents.** This ADR does not name which advisories are
suppressed, does not set or change any `owner` or `review_date` field, does not touch
`SECURITY-EXCEPTIONS.md`'s schema, and does not alter `.cargo/audit.toml` or `deny.toml`'s current
ignore arrays. Every one of those questions stays inside ADR-0024's `## Decision`, decisions 1-5.
This ADR answers only: *which files are the legal surface, and is a workflow file one of them.* The
answer to the second half is no.

**(iii) Enforced, not merely asserted.** Before this ADR, the invariant existed only as a comment
(`ci.yml:74-76`: "Exceptions are the single source of truth in `.cargo/audit.toml`... so no inline
`--ignore` flags are used here") — prose that SUPPLY-01's own root cause proves is not
self-enforcing: a second `cargo audit` job with two inline `--ignore` flags baked in shipped
alongside that exact comment for an unknown period before Phase 9 deleted it. The invariant is now a
gate: `scripts/check-workflow-suppressions.sh` is wired into `Makefile`'s `check-workflow-suppressions`
target (`Makefile:171-173`), that target is a `check-gates` prerequisite (`Makefile:176`), and
`ci.yml`'s `cargo-deny:` job runs it on every PR and push to the primary branches as the step named
"Check workflow files for inline advisory suppressions" (`ci.yml:103-104`).

An ADR — rather than a re-tagged `.project/` document — is the promotion mechanism here for the same
reason ADR-0016, ADR-0021, ADR-0024 and ADR-0025 already used it: `PROMOTION.md` §Part A closed the
`--manifest`/re-ingest path (`.planning/STATE.md`: "there is no run 6") and established that an ADR
in `.planning/decisions/` is its own document class, independent of the ingest manifest, sitting at
the top of the precedence order by construction. Promoting candidate 7 is the fifth instance of that
same six-step procedure, not a new mechanism invented for this ADR.

## Considered Options

- **Promote candidate 7 as a standalone ADR with a `conforms` verdict** (accepted) — the tree already satisfies the invariant (see `## Code Conformance`), matching ADR-0031's posture and closing `PROMOTION.md` Part B candidate 7 under Phase 12.
- **Decline and leave the invariant at PRD/DOC precedence** (rejected) — SUPPLY-03's own text concedes this leaves the invariant overridable by any future document, reopening the exact gap SUPPLY-01 closed once already.
- **Fold the invariant into ADR-0024 as an amendment** (rejected) — ADR-0024 already answers a different question (suppression contents, per-advisory ownership); merging topology into it would make one ADR answer two questions and blur the supersession mechanism.
- **Mark ADR-0024 superseded and reissue its contents under this ADR** (rejected) — false on its face: ADR-0024 is live and correct, and nothing in this ADR's topology question contradicts anything ADR-0024 decided about contents.

## Code Locations

- `.github/workflows/ci.yml:61-78` — the `security-audit:` job, display name `Security Audit`, the sole surviving `cargo audit` invocation.
- `.github/workflows/ci.yml:74-76` — the rationale comment declaring `.cargo/audit.toml` the single source of truth and stating no inline `--ignore` flags are used.
- `.github/workflows/ci.yml:78` — `run: cargo audit`, the bare, config-driven invocation this ADR's invariant requires.
- `.github/workflows/ci.yml:103-104` — the D-08 guard's own CI wiring: `- name: Check workflow files for inline advisory suppressions` / `run: ./scripts/check-workflow-suppressions.sh`, inside the `cargo-deny:` job.
- `.github/workflows/ci.yml:121` — `run: cargo deny check`, the second gated invocation class the guard's co-occurrence match covers.
- `scripts/check-workflow-suppressions.sh` — the D-08 regression guard: offline, PyYAML-structural, accumulate-all-failures, asserting both the co-occurrence clause and the exactly-one-`cargo-audit`-invocation clause.
- `Makefile:171-173` — the `check-workflow-suppressions` target wiring the guard into `make`.
- `Makefile:176` — the `check-gates` aggregate target, with `check-workflow-suppressions` appended to its prerequisite list.
- `.cargo/audit.toml:11,37` — the `[advisories]` `ignore` array, one of the two legal suppression surfaces this invariant names.
- `deny.toml:115-116` — the `[advisories]` `ignore` array, the second legal suppression surface, read by `cargo deny` and not by `cargo audit`.
- `SECURITY-EXCEPTIONS.md` — ADR-0024's governance register, cited here as the "why" a reader lands on after confirming a suppression sits in a legal surface.
- `.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md` FR-1 (`:71-73`, single-file framing) and §8 (`:187`, two-file framing this ADR adopts) — the source document being promoted.

## Code Conformance

conforms

Two measurements, both re-run in this execution rather than copied from `12-CONTEXT.md` or
`12-RESEARCH.md`:

**1. The guard's own exit-0 run against the real tree**, `./scripts/check-workflow-suppressions.sh`:

```
🔍 Checking workflow files for inline advisory-ignore suppressions on cargo audit/deny ...
✅ 6 workflow file(s) scanned, 109 run step(s) examined, 1 cargo audit invocation(s) found; no inline advisory-ignore suppression detected.
```

Exit code `0`.

**2. The ignore-family token census**, `grep -rn 'ignore' .github/workflows/*.yml` filtered to lines
that are not comments, cross-checked by hand against the guard's own co-occurrence logic:

```
ci.yml:431:          mc mb testminio/test-bucket --ignore-existing
ci.yml:432:          mc mb testminio/integration-tests --ignore-existing
ci.yml:466:          cargo test redis_queue_integration_tests --release -- --ignored --nocapture
ci.yml:469:          cargo test file_storage_integration_tests --release -- --ignored --nocapture
ci.yml:760:            cargo test file_storage_integration_tests --test lib -- --ignored --test-threads=1 --nocapture
```

Every ignore-family token found under `.github/workflows/` is one of exactly two kinds, and neither
is an advisory suppression: a MinIO client flag (`--ignore-existing`, on `mc mb` lines, which do not
match `CARGO_GATE_RE` at all) and Rust's own test-harness opt-in flag (`-- --ignored`, on `cargo
test` lines — `test`, not `audit` or `deny`, so `CARGO_GATE_RE`'s word boundary excludes them too).
No line anywhere in the scanned tree matches both halves of the co-occurrence test. The tree
satisfies the restated invariant today; this ADR instructs no code change.

## Downstream Consumers

- **Phase 13 / ORCH-01** — the Milestone 9-12 ground-truth ledger must record this invariant's
  verdict class as "100% complete with one false acceptance criterion — and, as of 2026-08-08, no
  longer false," citing this ADR and the guard transcript above rather than re-measuring from
  scratch.
- **Phase 15 / PIPE-01** — any new CI job that Phase 15 adds must not reintroduce an inline advisory
  suppression; `scripts/check-workflow-suppressions.sh`, already wired into `check-gates` and `ci.yml`,
  is what will catch it if one does, without Phase 15 needing to re-derive the invariant.
- **The milestone close-out** — inherits the unapplied-GitHub-rulesets finding plan 12-01 recorded
  (`.github/rulesets/` version-controlled but not applied to the live repository) as a related,
  separate open item; this ADR's own invariant is fully enforced regardless of that finding's
  disposition.
