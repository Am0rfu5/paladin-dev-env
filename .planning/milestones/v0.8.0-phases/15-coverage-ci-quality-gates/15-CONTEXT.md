# Phase 15: Coverage & CI Quality Gates - Context

**Gathered:** 2026-08-12
**Status:** Ready for planning
**Mode:** interactive — every decision below was selected by a human, except where marked
*Claude's Discretion*.

**Four gray areas were presented and all four selected:** coverage scope & enforcer · module gates
and the 0%-coverage binary · mock infrastructure (DEFER-01) · phase size and where the test-writing
stops.

**Eleven findings from this session's tree scout change the shape of the phase.** They are numbered
in `<specifics>`, each verified on 2026-08-12. Several requirement premises are false against the
live tree — `CONTRIBUTING.md` does not exist, the `run()` seam ADR-0006 names as a prerequisite is
already present, `tests/common/` cannot serve the requirements it exists to unblock, the
`MockNotificationService` the register names is not constructible as specified, and the 0.00%
figure for `paladin-server.rs` is stale. **Re-verify every line number and count in
`REQUIREMENTS.md` before acting on it** — Phase 13's hand-off already warned this for the eight
deprecated-action references, and it generalises.

<domain>
## Phase Boundary

Make the project measure its own quality on every push instead of asserting it. Eight requirements:
**PIPE-01 … PIPE-05** (the CI quality gates) and **DEFER-01 … DEFER-03** (the coverage register
those gates then measure). The two halves are **strictly sequential** — the register's own words are
that Epic 25 "establishes quality gates that validate all subsequent work" — and ROADMAP fixes that
order.

**Five deliverable classes:**

1. **CI gates that fail a build** (PIPE-01, PIPE-02) — a `cli-tests` job running the 86 CLI snapshot
   tests under `--features cli`, a `bench-check` job (`cargo bench --no-run`), and a `coverage` job
   that measures unit + integration together behind Redis and MinIO and fails on a threshold.
2. **One coverage number with one scope, wired into CI** (PIPE-02) — ADR-0006 amended in place with
   a figure measured under this phase's scope, the floor re-derived from it by the ADR's own
   truncate-toward-zero rule, and `.codecov.yml` landed for reporting.
3. **Local reproducibility and modernised workflows** (PIPE-03, PIPE-04) — the four Makefile targets,
   eight deprecated GitHub Action references replaced, `actionlint` clean.
4. **The coverage register closed** (DEFER-01, DEFER-02, DEFER-03) — shared `Send + Sync` test
   infrastructure, `user_service.rs` and `listener.rs` each to ≥ 80%, with the listener's stale
   57.83% baseline re-measured before its scope is stated.
5. **The record made true** (PIPE-05 and the corrections) — coverage documentation a contributor can
   reproduce the CI number from, and the rejected 80%/70% figure removed from the three instruction
   files that still assert it.

**Not in this phase:**

- **Closing Herald's ~14.5-point gap to its ≥ 95% module target.** D-05 records and re-measures it;
  it is handed forward as named work. Same for the autonomous ≥ 90% target, which already clears.
- **Enforcing any per-module gate in CI.** ADR-0006's own words: "the 84% floor remains the only
  binding gate". DEFER-02/03's ≥ 80% is a phase acceptance criterion, not a standing gate (D-12).
- **Widening the coverage denominator to non-default features beyond `integration-tests`.** `cli`,
  `web-server`, `qdrant`, `vision` and `s3-storage` stay out (D-01, D-06).
- **Benchmark regression *detection*** (`critcmp`, `github-action-benchmark`) — a Deferred-QA Epic 25
  explicit non-goal. Note the inversion: `benchmark-regression-signal` already ships at
  `ci.yml:812`; the compile-check prerequisite does not.
- **Splitting or relocating `user_service.rs`.** ADR-0034 **withdrew** the D2 split; the full
  relocation is owned by the run-3 v2 tech-debt item. This phase tests the file as it ships.
- **General documentation content currency.** The sweep in D-14 is scoped to *coverage-number claims
  only*. Milestone 11's fourteen content-currency files are DOCS-01, Phase 16.
- **The `api-surface` baseline path.** DEBT-01 owns `project/current-exports.txt`; PIPE-04 owns only
  the action versions. Recorded so neither is planned twice.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10, 11, 12, 13 and 14 — locked, not re-litigated

- **D-00a [informational]:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0042). **`PROMOTION.md:66` records 0043
  as next free** — verified this session. Update that line if this phase authors an ADR.
- **D-00b [informational]:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02.)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. *(Phase 5 D-08.)*
- **D-00d:** Ledgers and requirement texts are **amended in place**, dated, superseded text
  retained. Never a separate corrections file. *(Phase 2 D-02.)* **This phase amends several
  requirement premises at source** — see `<specifics>`.
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that produced
  it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10, 12, 13, 14.)*
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers.
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md.)* Applies where a domain noun is coined, not to CI/tooling terms of art.
- **D-00i [informational]:** Provenance of `--auto` decisions is carried forward rather than laundered. *(Phase 12
  hand-off item 6.)*
- **D-00l — ADR-0006 is the binding coverage record and is amended, never duplicated.** Its own
  ratchet clause specifies in-place amendment, D-00g makes it the house convention, and RECON-07
  exists precisely to eliminate the "choosing between two numbers" failure a second coverage ADR
  would recreate. Phases 5 and 8 both amended it in place; this phase does the same.
- **D-00m — ADR-0034 dissolved the `user_service.rs` collision.** The D2 split is **withdrawn**, not
  sequenced: the split is owned by nobody, the full relocation by the run-3 v2 tech-debt item, the
  tests by DEFER-02 / this phase. Phase 15 is explicitly free to size DEFER-02 against the unsplit
  file. Do not re-open the sequencing question REQUIREMENTS.md still describes as live.
- **D-00n [informational] — the project is at 0.8.0.** Phase 14's D-17 bumped all twelve manifests lockstep;
  `release.toml:17` sets `shared-version = true`. This phase ships under 0.8.0 and is not expected
  to be a breaking change.

---

### Coverage scope and what fails the build (PIPE-02, PIPE-03)

- **D-01: The CI coverage job measures `--workspace --features integration-tests` with Redis and MinIO running.**
  This is the extension ADR-0006 explicitly names as scheduled work ("PIPE-02 must
  either reproduce those three or record why its figure differs"; "must also extend the recorded
  scope to the Docker-backed `integration-tests.yml` suite this ADR could not reach"). The floor is
  **re-derived from the figure measured under this scope**, using ADR-0006's own arithmetic —
  truncate the measured percentage toward zero to a whole percent, comparison at-or-above — and
  ADR-0006 is amended in place with the new figure, command and date. Still **one number, one
  scope**: RECON-07's rule is intact. `--all-features` was rejected: `qdrant` needs a live Qdrant
  service and the embedding/vision suites need real API keys, so that code would enter the
  denominator with nothing in CI able to exercise it — depressing the number for no signal.
  Default-feature-only was rejected because it leaves ADR-0006's own extension instruction
  unfulfilled.
  — **Reversibility:** costly — the floor is a published gate number; moving the scope again means
  re-measuring and re-deriving, and every intervening comparison becomes non-comparable.

- **D-02: `cargo llvm-cov --fail-under-lines <floor>` in the workflow is the gate. Codecov reports, it does not gate.**
  `.codecov.yml` still lands per PIPE-02 (PR comments, diff view, dashboard) but
  carries no blocking status. Rationale is PIPE-02's own warning: without `CODECOV_TOKEN` an upload
  can fail **silently**, especially on fork PRs — a gate that silently does not run is worse than no
  gate. The in-workflow threshold also runs identically under `make coverage`, which is exactly
  PIPE-03's stated purpose.
  — **Reversibility:** reversible.

- **D-03: A dedicated `coverage` job in `ci.yml`, and `integration-tests.yml`'s coverage step is deleted.**
  The new job reuses the Redis/MinIO service block pattern already present in `ci.yml`'s
  own `integration-tests` job, and runs on every push and PR — the gate belongs on the workflow that
  gates merges, not on one that runs on PR plus a daily cron. Deleting
  `integration-tests.yml`'s `continue-on-error: true` coverage step and its `codecov-action@v3`
  upload **answers Epic 25's Open Question 3 with the parent PRD's own recommendation** (removal, to
  avoid duplicate uploads) and removes one of PIPE-04's eight deprecated-action references by
  deletion rather than upgrade.
  — **Reversibility:** reversible.

- **D-04: Two-step landing — measure first, gate second.** Commit 1 lands the `coverage` job in
  measure-only mode (no `--fail-under-lines`); CI produces the real figure under D-01's scope; that
  figure is transcribed **byte-identical** into the ADR-0006 amendment (D-00e); commit 2 sets the
  floor by the truncation rule and turns the gate on. This structurally guarantees ADR-0006's
  no-red-on-the-run-that-sets-it construction. **Forced by the environment:** Docker is absent here
  (verified 2026-08-12), so the `--features integration-tests` figure cannot be produced locally by
  any method. ADR-0006 rejected "record a CI-produced figure" at Phase 1 *because no CI gate existed
  to produce one* — this phase is what changes that premise, and the amendment must say so in those
  terms rather than appear to contradict the earlier rejection.
  — **Reversibility:** reversible.

### Module gates, binaries and the CLI surface (PIPE-01, PIPE-02)

- **D-05: The two module-scoped gates are re-measured and recorded, not enforced in CI.** The single
  workspace floor stays the only thing that fails a build — ADR-0006's own words, "the 84% floor
  remains the only binding gate", and module targets "never a replacement for it". Herald and the
  autonomous aggregate are re-measured under D-01's scope and recorded in the ADR amendment **with
  their gaps stated**, and Herald's ~14.5-point climb is handed forward as named work with an owner
  rather than silently dropped. A per-module no-regression ratchet was considered and rejected: it
  reintroduces the multi-number failure RECON-07 exists to prevent, in a smaller form.
  — **Reversibility:** reversible.

- **D-06: The three binaries are outside the gated denominator by construction, and this is recorded explicitly.**
  `paladin` and `paladin-cli` require `cli`; `paladin-server` requires `web-server`
  (`Cargo.toml:239-252`). Under D-01's feature set none of them compiles, so none enters the
  denominator — the identical treatment ADR-0006 already gives `minio.rs`, and it must be written
  down the same way rather than left as a silent absence. `.codecov.yml` gets `src/bin/**` in its
  `ignore` list so the report agrees with the gate. **Two ADR-0006 inheritances close here:** D-14a's
  `run()`-seam extraction is **already done** (`src/bin/paladin-server.rs:49`), and the **0.00%
  figure is stale** — Phase 14's D-15b landed a `#[cfg(test)]` module at `:256`. Both are corrected
  at source per D-00d.
  — **Reversibility:** reversible.

- **D-07: A separate `cli-tests` job, not a step.** `cargo test -p paladin-ai --features cli
  --test cli`, no `needs:`, in parallel with `lint` and `test` — it requires no external services.
  ROADMAP criterion 1 requires that a PR breaking a CLI snapshot **fails CI**; a distinctly named
  job makes that legible instead of surfacing as a red `crate-isolation (paladin-ai)` leg. **The
  missing ingredient is the feature flag, not a test surface** — `crate-isolation`'s `paladin-ai`
  leg already runs `cargo test -p paladin-ai` under default features, and the `cli` target is
  skipped there for exactly one reason: `required-features = ["cli"]` (`Cargo.toml:210-213`).
  — **Reversibility:** reversible.

### Mock and test infrastructure (DEFER-01)

- **D-08: The shared test infrastructure lives in `src/`, behind `#[cfg(test)]` — not `tests/common/`.**
  Both coverage targets test from *inside* `src/` via co-located
  `#[cfg(test)] mod tests` (`user_service.rs:467`, `listener.rs`), and `tests/` is a **separate
  crate** that `src/` cannot import from. `tests/common/` therefore cannot serve the two
  requirements DEFER-01 exists to unblock. A `src/test_support/` module (name at the planner's
  discretion) is the only placement that works, and it keeps tests able to exercise private paths —
  which matters for reaching ≥ 80%. DEFER-01's `tests/common/` premise is corrected at source as
  **stale-by-structure**, in the same class as its already-acknowledged stale module paths.
  — **Reversibility:** costly — moving mocks later means touching every test that imports them.

- **D-09: Hand-written mocks. `mockall` is not adopted.** This answers DEFER-01's **Open Question 2**
  explicitly rather than by default. `grep -rn "mockall"` returns nothing across all twelve
  manifests; every existing mock is hand-written with the `Arc<Mutex<..>>` recording pattern
  DEFER-01 itself specifies (`tests/helpers/mock_llm_adapter.rs`), so the new mocks copy a proven
  shape rather than introduce a second idiom nobody would retrofit. Adding a proc-macro
  dev-dependency is also a governed act under ADR-0024/ADR-0036's suppression register. Cost
  accepted: boilerplate.
  — **Reversibility:** reversible.

- **D-10: The mock set is demand-driven, and each of DEFER-01's five names gets a recorded verdict.**
  Build what DEFER-02 and DEFER-03 actually consume, verified against real signatures; then record
  per name: **built / replaced by X / unnecessary because Y**. Nothing is silently dropped and no
  mock is written that no test calls. On this session's evidence: `MockNotificationService` becomes
  a **`FailingChannelHandler`** (see `<specifics>` finding 6 — `UserService` holds a *concrete*
  `Arc<NotificationService>`, but `register_channel_handler` is public, so the failure path is
  reachable with no signature change), and `MockUserRepository` is **likely unnecessary** because
  `SqliteUserRepository::new("sqlite::memory:")` already fills that role in the shipped tests.
  — **Reversibility:** reversible.

### Phase size, acceptance bar and the record (DEFER-02, DEFER-03, PIPE-05)

- **D-11: One phase — PIPE-01 … PIPE-05 first, then DEFER-01 … DEFER-03, wave-decomposed.** Meets
  ROADMAP criterion 6 as written and needs no roadmap amendment; keeps the gate and the work it
  gates in one reviewable unit. **The register's 35-45h figure is a stale upper bound**: it dates
  from February 2026, D-05 removed the Herald climb, D-10 shrank the mock set, and both target
  modules have gained tests since it was struck. DEFER-03's real size comes from its own required
  re-measurement, not from the inherited estimate. The sequential PIPE→DEFER order is fixed by
  ROADMAP and by the register's own reasoning; it is not a planner choice.
  — **Reversibility:** reversible.

- **D-12: ≥ 80% per module, as a phase acceptance criterion — not a standing CI gate.** ROADMAP
  criterion 6's "covered to the gate" is ambiguous; it resolves to **DEFER-02's own figure** rather
  than an invented one, and DEFER-03 inherits the same ≥ 80% bar because its text names none and
  criterion 6 needs a number to be falsifiable. Verified by a module-targeted `cargo llvm-cov` run.
  Fully consistent with D-05: this is an acceptance bar for this phase, not a gate that lives on in
  CI. DEFER-02's rule that **intentionally untested paths carry a written justification** applies to
  both modules.
  — **Reversibility:** reversible.

- **D-13: PIPE-05's coverage documentation lands in `docs/src/contributing/testing-guide.md`.**
  **`CONTRIBUTING.md` does not exist** — contributor documentation was relocated into the mdbook by
  Milestone 11 (`docs/src/contributing/{testing-guide,development-setup,architecture-decisions,
  contributing-providers}.md`, plus `docs/src/appendix/contributing-legacy.md`). PIPE-05's premise is
  corrected at source as **relocated-by-outcome** — the same class PROJECT.md already records
  superseded for `STABLE_API.md` and the root `docs/*.md` deliverables. No root-path file that
  Milestone 11 deliberately removed is reintroduced. The page must satisfy PIPE-05's own "done when":
  a new contributor can reproduce the CI coverage number locally from the document alone.
  — **Reversibility:** reversible.

- **D-14: The rejected 80% / 70% figure is corrected in all three instruction files.** `CLAUDE.md`
  (`:59-60`), `.github/copilot-instructions.md` (`:155-156`) and `.planning/codebase/TESTING.md`
  (`:311-327`) all assert "unit ≥ 80% / integration ≥ 70%" — **position 1 in ADR-0006's Considered
  Options, explicitly rejected** — and TESTING.md adds two more falsehoods: `cargo tarpaulin` as the
  tool (ADR-0006's tool-of-record note already flags `:319-322`) and "Enforced in CI pipeline", which
  only becomes true because of this phase. These are the files every agent session and every
  contributor reads before writing a test, so the contradiction is self-perpetuating in a way a
  stale doc page is not. All three cite ADR-0006's single binding number, this phase's actual CI
  mechanism, and `cargo-llvm-cov` as tool-of-record.
  **Scope guard:** coverage-number claims **only**. General content currency is DOCS-01, Phase 16 —
  do not widen into it.
  — **Reversibility:** reversible.

### Claude's Discretion

- **PIPE-04's action-version mapping and `actionlint` scope.** The eight references and their
  replacements are specified; the line numbers in the requirement are stale (re-grep — see
  `<specifics>` finding 1). PIPE-04's text says "all three workflows" but **six exist** (`ci`,
  `docs`, `feature-flags`, `integration-tests`, `pre-commit`, `release`) — lint all six and record
  the correction per D-00d. Note D-03 deletes the `codecov-action@v3` reference rather than
  upgrading it, so PIPE-04's count of remaining work is seven, not eight.
- **`bench-check` shape and cost.** `cargo bench --no-run` is unambiguous; whether it is worth
  caching separately, and where it sits relative to `needs:`, is the planner's call.
- **`.codecov.yml` contents beyond the specified keys**, subject to D-02 (no blocking status) and
  D-06 (`src/bin/**` in `ignore`). PIPE-02 specifies the rest verbatim.
- **The `make coverage` / `make services-up` relationship.** The target must reproduce the CI number
  (PIPE-03's whole purpose), which means Redis and MinIO must be up; whether the target declares the
  dependency or fails loudly with a pointer is the planner's call.
- **Naming of the test-support module** under D-08, and whether `tests/helpers/` is eventually
  consolidated into it (out of scope here if it grows past a re-export).
- **Tokio time-control utilities.** DEFER-01 names `tokio::time::pause()`/`advance()`; these are std
  tokio features needing no wrapper unless the listener tests want a shared helper.
- **Wave decomposition and plan boundaries**, subject to D-11's PIPE→DEFER ordering and D-04's
  two-commit coverage-gate sequence.
- **ADR allocation.** Whether the CI-gate topology (D-01 … D-04) warrants its own record or lives
  entirely as an ADR-0006 amendment under D-00l. The amendment is mandatory either way.
- **Whether the advisory Docker build-time budget at `ci.yml:539` — which names "Owner: Phase 15 /
  PIPE" and proposes native `ubuntu-24.04-arm` runners instead of QEMU — is taken up here.** Raised
  and not decided; it is a CI-quality item with a named owner but no requirement ID.
- **DEFERRED_COVERAGE's two remaining prerequisites** — "document testing best practices" and
  "establish concurrency testing patterns" — are largely satisfied by D-13's page and DEFER-03's
  concurrency suite; whether they get explicit closure records is the planner's call.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` — Phase 15 entry: goal, the strictly-sequential PIPE→DEFER dependency, and
  the six success criteria. **Criterion 3's "78% hard gate versus a phased 70 → 74 → 78 ramp against
  a measured 76-77% baseline" is superseded** — ADR-0006 rejected both positions and the 76-77%
  figure; criterion 6's "covered to the gate" is resolved by D-12.
- `.planning/REQUIREMENTS.md` §"Deferred-QA pipeline and coverage" — **PIPE-01 … PIPE-05** and
  **DEFER-01 … DEFER-03** (lines ~2785-3004). The requirement text this phase closes and, per D-00d,
  amends at source. **Several premises are false against the tree — see `<specifics>`.**
- `.planning/REQUIREMENTS.md` §"Hand-off to Phase 15 / PIPE-01 … — dated 2026-08-10 (plan 13-13)"
  (line 2528) — five numbered items: the corrected 15-job `ci.yml` list, the `check-api-surface.sh`
  closure, the unresolved threshold variant (settled by ADR-0006), the stale action line numbers,
  and the mock prerequisite with ADR-0034's collision resolution.
- `.planning/PROJECT.md` §Active — this phase's place in the Milestone 9-12 + Deferred-QA close-out.

### Decisions this phase applies, amends, or must not re-open

- `.planning/decisions/0006-coverage-gate.md` — **the binding coverage record. Read it in full.**
  The measured figure and its verbatim pipeline, the floor arithmetic, the ratchet, the two
  module-scoped gates (Phase 5 amendment, D-13 table), the `paladin-server.rs` and `minio.rs`
  dispositions (D-14a/D-14b), the tool-of-record note, and the Phase 8 amendment's 85.85% figure.
  **This phase amends it in place (D-00l, D-01, D-05, D-06).**
- `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md` §"(ii) D2" — the
  `user_service.rs` split is **withdrawn**; Phase 15 sizes DEFER-02 against the unsplit file. Do not
  re-open (D-00m).
- `.planning/decisions/0024-rustsec-exception-governance.md` and
  `.planning/decisions/0036-audit-suppression-single-source-topology.md` — why adding a dependency
  (e.g. `mockall`) is a governed act, not a free choice (D-09).
- `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` — the precedent for recording a
  reintroduction condition without building the thing, if D-09's trigger shape is revisited.
- `.planning/decisions/PROMOTION.md:66` — **next free ADR number: 0043.** Update if this phase
  authors a record; the procedure is at `:224-233`.

### Evidence and ledger

- `.planning/ledgers/milestone-09-12.md` — rows `REQ-ci-cli-snapshot-job`, `REQ-ci-bench-check-job`,
  `REQ-ci-combined-coverage-job`, `REQ-codecov-config-thresholds`, `REQ-makefile-coverage-targets`,
  `REQ-modernize-github-actions`, `REQ-contributing-coverage-docs`, `REQ-mock-infrastructure`,
  `REQ-user-service-test-coverage`, `REQ-listener-service-test-coverage`. Amended in place per D-00d.
  Its head-note paragraph 2 carries the measured 15-job `ci.yml` list.
- `.planning/ledgers/milestone-02-03.md` — Epic 24 block verdict, cluster `8.0` row, and the
  `### Phase 6 CLOSE-02 scope` section: why `cli-tests`, `bench-check`, the `coverage` job and
  `.codecov.yml` were deferred here rather than built in Phase 6, with its two rejected alternatives.
- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` — the original
  measurement record and per-file rows ADR-0006 transcribes from.
- `.planning/phases/08-verified-defect-closure/08-09-SUMMARY.md` — the Phase 8 re-measurement
  (85.85%), and the `table_herald` feature-gating accounting that explains a symmetric denominator
  change. The nearest precedent for this phase's own measurement work.

### Source documents this phase corrects

- `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` — the parent PRD: FR-25.3
  item 10's 78% hard gate and Open Question 3. Both settled by ADR-0006; receives a dated banner per
  D-00c if the phase touches it.
- `.project/Deferred-QA-CICD-Completion/prd-cicd-pipeline-enhancement.md` — Epic 25: FR-25.1 … 25.10,
  Appendix B (the pre-M10 7-job pipeline table) and Appendix C (the phased ramp), plus **Open
  Question 3 on the integration-tests coverage step, answered by D-03**.
- `.project/Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md` — the register: Module 1
  (`user_service.rs`) and Module 2 (`listener_service.rs`), the three unchecked prerequisites, and
  the stale paths and baselines DEFER-02/DEFER-03 correct.

### Code sites — all verified 2026-08-12

**CI workflows (PIPE-01, PIPE-02, PIPE-04):**
- `.github/workflows/ci.yml` — **15 jobs**: `lint:21`, `security-audit:61`, `cargo-deny:81`,
  `osv-scanner:126`, `api-surface:155`, `test:206`, `examples:245`, `crate-isolation:319`,
  `integration-tests:374`, `docker:494`, `kubernetes-smoke:611`, `e2e-tests:718`, `benchmark:779`,
  `benchmark-regression-signal:812`, `publish-dry-run:898`.
- `.github/workflows/ci.yml:238,241` — the `test` job runs only `--lib --bins` and `--doc`.
- `.github/workflows/ci.yml:319-372` — `crate-isolation`, whose `paladin-ai` leg runs
  `cargo test -p paladin-ai` and **does** execute the root `tests/` tree under default features.
- `.github/workflows/ci.yml:374-400` — the existing Redis/MinIO service block to model D-03's job on
  (ports 6380/9010).
- `.github/workflows/ci.yml:163,408,788` — `actions-rs/toolchain@v1`.
- `.github/workflows/integration-tests.yml:71` (`actions-rs/toolchain@v1`), `:78,84,90`
  (`actions/cache@v3`), `:105` (the `--features integration-tests` run), `:113-127` (the coverage
  step and `codecov-action@v3` that D-03 deletes). Triggers: `pull_request` + daily cron only.
- `.github/workflows/ci.yml:539` — the advisory Docker build-time budget naming "Owner: Phase 15 /
  PIPE" and the native-arm64 rework.

**Test targets and features (PIPE-01, PIPE-03):**
- `Cargo.toml:210-213` — `[[test]] name = "cli"`, `path = "tests/cli/mod.rs"`,
  `required-features = ["cli"]`.
- `Cargo.toml:239-252` — the three `[[bin]]` targets and their `required-features` (D-06).
- `Cargo.toml` `[features]` — `default = ["llm-openai"]`, `integration-tests = []`, `cli = [...]`,
  `web-server = [...]`.
- `tests/cli/` — 15 files, **86 `.snap` files** under `tests/cli/snapshots/`, **97 `#[test]`
  functions** (not the 43 the requirement records).
- `Makefile:426-427` — `ci-test`, the target PIPE-03 extends. No `llvm-cov` reference anywhere.

**Coverage targets (DEFER-01, DEFER-02, DEFER-03):**
- `src/core/platform/manager/user_service.rs` — **583 lines**; `UserService` fields at `:28-34`
  (`Arc<dyn UserRepositoryPort>`, `Arc<dyn LogPort>`, **concrete** `Arc<NotificationService>`,
  `Argon2`, `Option<Arc<dyn AuthPort>>`); `send_welcome_notification:126`; `register_user:190` with
  the non-blocking notification handling at `:228`; existing `#[cfg(test)] mod tests:467` with five
  `#[tokio::test]`s built on real in-memory adapters.
- `src/application/services/orchestration/listener.rs` — **538 lines**, 3 tests. The module
  `DEFERRED_COVERAGE.md` still calls `src/core/platform/manager/listener_service.rs`.
- `src/application/services/notification_orchestrator/mod.rs:424` —
  `register_channel_handler(Arc<dyn NotificationChannelHandler>)`, public; `:432`
  `set_template_processor`. The seam D-10's `FailingChannelHandler` uses.
- `src/application/services/notification_orchestrator/types.rs:51-67` — the
  `NotificationChannelHandler` trait to implement.
- `tests/helpers/{mod,mock_llm_adapter,mock_arsenal_adapter,mock_paladin_port}.rs` — the existing
  hand-written mock convention and barrel D-09 follows.
- `src/bin/paladin-server.rs:34` (`main`), `:49` (**the `run()` seam, already present**), `:256`
  (`#[cfg(test)] mod tests`).
- `tests/paladin_server_smoke.rs` — `#![cfg(feature = "web-server")]`; re-builds the server wiring by
  hand rather than exercising the binary.

**Documentation surface (PIPE-05, D-14):**
- `docs/src/contributing/testing-guide.md` and `development-setup.md` — D-13's target.
- `CLAUDE.md:59-60` · `.github/copilot-instructions.md:155-156` ·
  `.planning/codebase/TESTING.md:311-327` — the three files asserting the rejected 80%/70% figure
  (D-14).

### Conventions

- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — TDD Red-Green-Refactor, no
  `unwrap()`/`expect()`/`panic!` in library code, `cargo test` → `cargo fmt --check` →
  `cargo clippy -- -D warnings` before commit, rustdoc on every public item. **Note `CLAUDE.md`'s
  own coverage numbers are what D-14 corrects.**
- `.github/instructions/snyk_rules.instructions.md` — Snyk scan on new/modified first-party code.
  **DEFER-02 touches the authentication and password-hashing surface's test coverage; the scan
  applies to any first-party code this phase adds there.**
- `.planning/codebase/TESTING.md` — the testing map: file organisation, the `Arc<Mutex<..>>` mock
  pattern, async/error/serial test idioms, and the dev-dependency inventory (`insta` 1.34,
  `serial_test` 3.2, `proptest` 1.4, `testcontainers` 0.24, `tokio-test`, `wiremock`). **Its
  §Coverage is what D-14 corrects.**
- `.planning/config.json` — `workflow.worktree_skip_hooks: true`. Surface this in executor prompts or
  every commit cold-compiles the workspace.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`ci.yml:374-400`** — a working Redis + MinIO `services:` block with health checks in the very
  file D-03's `coverage` job lands in. Copy it; nothing new is needed.
- **`ci.yml:319-372` (`crate-isolation`)** — an existing matrix job proving the root crate's `tests/`
  tree already runs in CI under default features. D-07 adds only the `cli` feature dimension.
- **`tests/helpers/mock_llm_adapter.rs`** — the canonical hand-written mock: `Arc<Mutex<VecDeque<..>>>`
  response queue, `Arc<Mutex<Vec<Invocation>>>` call recording, factory functions, a `mod.rs` barrel.
  D-09's new mocks are a copy of this shape, relocated per D-08.
- **`user_service.rs:477-491` (`build_service`)** — an existing test fixture assembling the service
  from **real in-memory adapters** (`SqliteUserRepository::new("sqlite::memory:")`,
  `SystemLogAdapter::new_for_test`, `InMemoryTokenAuthAdapter`). DEFER-02's suite extends this rather
  than replacing it, which is why D-10 expects `MockUserRepository` to prove unnecessary.
- **`src/bin/paladin-server.rs:49` (`run()`)** — the testable seam ADR-0006 D-14a names as a
  prerequisite. Already extracted; nothing to build.
- **`Makefile:426-427` (`ci-test`)** — the target PIPE-03 extends with `test-cli` and adds `ci-full`
  beside.

### Established Patterns

- **Feature-gated exclusion as a scope decision, not a coverage gap** — ADR-0006's `minio.rs`
  treatment and Phase 8's `table_herald` accounting both establish that a symmetric
  numerator+denominator removal is a scope exclusion and must be *recorded as one*. D-06 applies the
  same reasoning to the three binaries.
- **Co-located `#[cfg(test)] mod tests` for unit tests, `tests/` for integration** — TESTING.md's
  documented split, and the reason D-08 cannot use `tests/common/`.
- **Real in-memory adapters over mocks where one exists** — SQLite `:memory:`,
  `SystemLogAdapter::new_for_test`, `InMemoryTokenAuthAdapter`, `MockLlmAdapter` shipped in
  `paladin-llm/src/mock.rs`. Prefer these before writing a new double (D-10).
- **Dated in-place amendment with original text retained** (D-00c/D-00d) — every requirement,
  ledger and `.project/` correction in this phase.
- **Transcribe measurements byte-identical, never re-type or round** (D-00e) — ADR-0006's own rule
  for its TOTAL row, and what D-04's second step depends on.

### Integration Points

- **`ci.yml`'s `jobs:` map** is where PIPE-01 and PIPE-02 land: `cli-tests`, `bench-check` and
  `coverage`, taking the job count from 15 to 18.
- **`integration-tests.yml:113-127`** is the single seam D-03 deletes, and the one place
  `codecov-action` currently appears.
- **`Makefile`'s Testing section** is where `test-cli` and `bench-check` join, with a new Coverage
  section between Testing and Code Quality (PIPE-03).
- **`src/test_support/` (new, D-08)** becomes the seam every co-located test module in `src/` imports
  from — `user_service.rs`'s and `listener.rs`'s `mod tests` are its first two consumers.
- **`NotificationService::register_channel_handler`** is the seam that makes DEFER-02's required
  "notification failure must not block registration" case testable without a signature change.

</code_context>

<specifics>
## Specific Ideas

Eleven findings verified against the tree on 2026-08-12. Each changes what the phase has to do, and
several contradict requirement text that must be corrected at source per D-00d.

1. **`ci.yml` has 15 jobs and the action line numbers have moved again.** Phase 13's hand-off already
   corrected 14 → 15; this session confirms the list unchanged. The eight deprecated-action
   references are all still present but at `ci.yml:163,408,788` and
   `integration-tests.yml:71,78,84,90,123` — **none of the line numbers in PIPE-04's text is
   correct**. Re-grep before acting; do not trust cited line numbers anywhere in this requirement
   set.

2. **PIPE-01's `cargo test --test cli` compiles nothing as written.** `Cargo.toml:210-213` sets
   `required-features = ["cli"]`, so the target is silently skipped without the flag. The job must
   pass `--features cli` (D-07). This is also the entire reason the snapshots have never run in CI —
   `crate-isolation`'s `paladin-ai` leg already runs the rest of the root `tests/` tree.

3. **There are 86 snapshots and 97 CLI test functions, not 43.** `ls tests/cli/snapshots | wc -l` →
   86; `grep -c "#\[test\]" tests/cli/*.rs` sums to 97 across seven files (`environment_tests.rs` 44,
   `error_output_test.rs` 15, `help_output_test.rs` 12, `progress_output_test.rs` 8,
   `table_output_test.rs` 8, `error_handling_test.rs` 7, `integration_tests.rs` 3). The "43 total"
   figure in PIPE-01 and ROADMAP criterion 1 is stale and is corrected at source.

4. **The `run()` seam already exists and the 0.00% figure is stale.** ADR-0006's D-14a names
   "extracting a testable `run()` seam from `#[tokio::main] async fn main()`" as the concrete
   prerequisite Phase 15 inherits — `src/bin/paladin-server.rs:34` is `main`, `:49` is `run()`, and
   `:256` is a `#[cfg(test)] mod tests` that Phase 14's D-15b added. Both halves of the inheritance
   close by observation, not by work (D-06).

5. **All three binaries are feature-gated, so they were never in the default-feature denominator.**
   `Cargo.toml:239-252`. Under D-01's scope none compiles. Whether the 0.00% row in
   `01-coverage-measurement.md:426` reflects a wider feature set than ADR-0006 records is worth one
   line of research; either way D-06 settles the disposition going forward.

6. **`MockNotificationService` is not constructible as DEFER-01 specifies — and it isn't needed.**
   `UserService.notification_service` is `Arc<NotificationService>`, a **concrete struct**, so no
   trait double can be substituted without changing a public constructor. But
   `NotificationService::register_channel_handler(Arc<dyn NotificationChannelHandler>)` is public
   (`notification_orchestrator/mod.rs:424`), so a `FailingChannelHandler` forces the failure path
   with **no signature change**. And the behaviour DEFER-02 wants proven is already implemented:
   `register_user:228` uses `if let Err(e) = self.send_welcome_notification(..)`, not `?`. The test
   proves existing behaviour rather than driving new code.

7. **`tests/common/` cannot serve DEFER-02 or DEFER-03.** Both target modules test from inside `src/`
   via co-located `#[cfg(test)]` modules, and `tests/` is a separate crate. The PRD's placement is
   stale-by-structure, not merely stale-by-path (D-08).

8. **`user_service.rs` already has tests, built on real in-memory adapters.** `:467-583` — five
   `#[tokio::test]`s over delete/list/query paths using `SqliteUserRepository::new("sqlite::memory:")`,
   `SystemLogAdapter::new_for_test`, a real `NotificationService` and `InMemoryTokenAuthAdapter`. The
   recorded "488 LOC, ~4.23% coverage" profile is stale in both figures (the file is **583 lines**),
   and the mock set DEFER-01 sizes against it is larger than what the tests actually need (D-10).

9. **`CONTRIBUTING.md` does not exist**, and the `cargo tarpaulin` references PIPE-05 says to update
   are not in contributor documentation at all — the only ones in the tree are
   `.planning/codebase/TESTING.md:319-322`, which ADR-0006 already flags. Both premises corrected at
   source (D-13, D-14).

10. **Three instruction files assert a coverage position ADR-0006 rejected.** `CLAUDE.md:59-60`,
    `.github/copilot-instructions.md:155-156` and `.planning/codebase/TESTING.md:311-327` all state
    "unit ≥ 80% / integration ≥ 70%". `CLAUDE.md` is loaded into every session in this repository.
    This is the multi-number failure RECON-07 exists to close, sitting in the files most likely to
    reproduce it (D-14).

11. **The environment changed since Phase 8: crates.io is reachable, Docker is not.**
    `cargo search cargo-llvm-cov` returns `cargo-llvm-cov = "0.8.7"` — the HTTP 403 that forced
    ADR-0006's raw `rustc`/`llvm-profdata` pipeline through Phase 8 is gone, so `cargo llvm-cov` is
    now installable locally. `docker info` fails, so the Redis/MinIO suites still cannot run here.
    **This is exactly why D-04 exists**, and the ADR-0006 amendment must state both facts: the
    tool-of-record note's premise has changed, and the measurement under D-01's scope necessarily
    comes from CI.

</specifics>

<deferred>
## Deferred Ideas

- **Closing Herald's ~14.5-point gap to its ≥ 95% module target** (`herald.rs`, measured 80.49%).
  Recorded and re-measured by D-05, not closed here. Needs an owner and a phase; it is real
  test-writing work on a module whose `table_herald` half is feature-gated out of the default scope.
- **Enforcing per-module coverage gates in CI**, via a report-parsing check script with
  no-regression ratchets. Considered under D-05 and declined as reintroducing multiple binding
  numbers. Revisit if module coverage proves to slide in practice.
- **Widening the coverage denominator to `web-server` and `cli`** so the shipped binaries are gated.
  Offered under D-06 and declined — the CLI surface is covered by D-07's `cli-tests` job through a
  cheaper mechanism, and widening would move the floor unpredictably. A future phase's call once the
  gate has a track record.
- **Adopting `mockall`.** Declined by D-09, not lost: the trigger would be the hand-written mock set
  growing past the point where expectation assertions dominate test code.
- **Consolidating `tests/helpers/` into the new `src/` test-support home.** Out of scope here; the
  two coexist, with D-08's module serving `src/`-side co-located tests and `tests/helpers/` serving
  integration suites.
- **The native-arm64 CI rework** (`ci.yml:525-552`) — replacing QEMU multi-arch emulation with
  `ubuntu-24.04-arm` runners so the Docker build-time budget can be hard rather than advisory. The
  comment names "Owner: Phase 15 / PIPE" but no requirement ID covers it; left to the planner's
  discretion and otherwise a future infrastructure phase's work.
- **Benchmark regression *detection*** (`critcmp`, `github-action-benchmark`). Epic 25's explicit
  non-goal, and already partially shipped as `benchmark-regression-signal` from Milestone 7 Epic 3.
- **A second, feature-scoped coverage measurement** (ADR-0006 D-14b's open question on `minio.rs`).
  D-01 answers it by choosing one scope; a second measurement is not added. Revisit only if a
  feature-gated subsystem needs its own recorded number.

</deferred>

---

*Phase: 15-coverage-ci-quality-gates*
*Context gathered: 2026-08-12*
