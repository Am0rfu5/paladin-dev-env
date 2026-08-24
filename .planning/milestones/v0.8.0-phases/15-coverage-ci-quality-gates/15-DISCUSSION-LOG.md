# Phase 15: Coverage & CI Quality Gates - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-12
**Phase:** 15-coverage-ci-quality-gates
**Areas discussed:** Coverage scope & enforcer, Module gates & the 0% binary, Mock infrastructure
(DEFER-01), Phase size & where tests stop

**Area selection:** four gray areas presented, all four selected.

---

## Coverage scope & enforcer

### Q1 — What feature scope should the CI coverage job measure?

| Option | Description | Selected |
|--------|-------------|----------|
| integration-tests + Docker | `--workspace --features integration-tests` with Redis + MinIO — the extension ADR-0006 names as scheduled work. Measure once, re-derive the floor by the ADR's truncation rule, amend in place. One number, one scope. | ✓ |
| Default features, reproduce 84% exactly | CI reproduces ADR-0006's binding scope byte-for-byte; zero day-one-red risk. Leaves the ADR's own extension instruction unfulfilled. | |
| `--all-features`, as PIPE-02 literally says | Widest denominator. Qdrant needs a live service and embedding/vision suites need API keys, so that code enters the denominator with nothing able to exercise it. | |

**User's choice:** integration-tests + Docker (D-01)

### Q2 — What actually fails the build when coverage drops below the floor?

| Option | Description | Selected |
|--------|-------------|----------|
| In-workflow threshold | `cargo llvm-cov --fail-under-lines`. Gate lives in the repo, runs identically under `make coverage`, no third-party service or secret. `.codecov.yml` lands for reporting only. | ✓ |
| Codecov project status is the gate | Matches PIPE-02's literal design, free patch enforcement. Depends on an external service and a secret; a fork PR that fails to upload passes green. | |
| Both — belt and braces | Strongest signal; two places to update the number, and PR authors blocked by a service maintainers don't control. | |

**User's choice:** In-workflow threshold (D-02)
**Notes:** PIPE-02's own text warns Codecov uploads can fail silently without `CODECOV_TOKEN`.

### Q3 — Where does the combined coverage job live, and what happens to the existing integration-only step?

| Option | Description | Selected |
|--------|-------------|----------|
| New job in `ci.yml`, delete old step | Dedicated `coverage` job reusing `ci.yml`'s own Redis/MinIO block, on every push and PR. Deletes `integration-tests.yml`'s `continue-on-error` step and its `codecov-action@v3` — the parent PRD's own answer to Epic 25 OQ-3. | ✓ |
| Fold coverage into `ci.yml`'s existing integration-tests job | No duplicated services, no second instrumented build. Lengthens the required check's critical path; conflates test failure with coverage failure. | |
| New job, keep old step as informational | Preserves the integration-only historical series on Codecov. Two uploads for one tree — the duplicate-upload problem OQ-3 raised. | |

**User's choice:** New job in `ci.yml`, delete old step (D-03)

### Q4 — How does the phase get the wider-scope number, given Docker is unavailable locally?

| Option | Description | Selected |
|--------|-------------|----------|
| Two-step landing | Commit 1 lands the job in measure-only mode; CI produces the figure; it is transcribed byte-identical into the ADR-0006 amendment; commit 2 sets the floor and turns the gate on. Structurally guarantees no day-one red. | ✓ |
| Keep 84%, gate blocking from commit one | PIPE-02's mandate read literally; simplest. A red gate on landing day is live, since `minio.rs` and integration-only paths enter the denominator for the first time. | |
| Widen scope only after a local proxy measurement | Reproduce ADR-0006's offline `rustc` pipeline to bound the delta. Measures the denominator change without the numerator change — a guess dressed as a measurement. | |

**User's choice:** Two-step landing (D-04)
**Notes:** Verified this session — `docker info` fails; `cargo search` succeeds, so the HTTP 403 that
blocked `cargo install cargo-llvm-cov` through Phase 8 is gone.

---

## Module gates & the 0% binary

### Q1 — How should the two module-scoped gates (Herald ≥95%, autonomous ≥90%) be handled in CI?

| Option | Description | Selected |
|--------|-------------|----------|
| Record + re-measure, don't gate | The single workspace floor stays the only build-failing gate — ADR-0006's own words. Both figures re-measured and recorded with gaps stated; Herald's ~14.5-point climb handed forward with an owner. | ✓ |
| Enforce at re-measured values, no-regression ratchet | Coverage can't slide backwards; 95%/90% stay as targets. New machinery and two more numbers — the multi-number failure RECON-07 prevents, in smaller form. | |
| Enforce at the target numbers | Fully discharges the enforcement hand-off. Requires closing Herald's 14.5-point gap in this phase on top of a 35-45h register. | |

**User's choice:** Record + re-measure, don't gate (D-05)

### Q2 — What happens to the binaries in the gated denominator?

| Option | Description | Selected |
|--------|-------------|----------|
| Outside by construction | The three bins are feature-gated and don't compile under D-01's feature set; recorded explicitly like `minio.rs`, with `src/bin/**` in `.codecov.yml`'s ignore. ADR-0006's `run()`-seam prerequisite closed by observation; 0.00% corrected as stale. | ✓ |
| Add `web-server` so the server bin is measured | `paladin-server` is a shipped deployment artefact; `run()` gets real tests. Widens the scope past D-01 and pulls `paladin-web`/`axum` into the denominator. | |
| Add `web-server` and `cli` — measure everything shipped | Most complete picture. Drags the whole `src/application/cli/` tree in; PIPE-01 covers that surface more cheaply. | |

**User's choice:** Outside by construction (D-06)
**Notes:** `src/bin/paladin-server.rs:49` already holds the `run()` seam; `:256` holds a
`#[cfg(test)]` module Phase 14 added.

### Q3 — What shape should the CLI snapshot check take?

| Option | Description | Selected |
|--------|-------------|----------|
| Separate `cli-tests` job | `cargo test -p paladin-ai --features cli --test cli`, no `needs:`, parallel with `lint` and `test`. Makes a snapshot regression legible instead of a red `crate-isolation` leg. | ✓ |
| Add `--features cli` to crate-isolation's `paladin-ai` leg | One-line matrix change, no new job. Conflates isolation with feature testing; the diagnostic gets worse as coverage gets better. | |
| Step inside the existing `test` job | Reuses a warm cache. The `test` job is a stable-and-beta matrix, so the suite runs twice, and its feature set stops being "default". | |

**User's choice:** Separate `cli-tests` job (D-07)

---

## Mock infrastructure (DEFER-01)

### Q1 — Where does the shared mock/test infrastructure live?

| Option | Description | Selected |
|--------|-------------|----------|
| `src/` test-support module | `#[cfg(test)]`-gated, importable by the co-located test modules in `user_service.rs` and `listener.rs`. The only placement that works, since `tests/` is a separate crate. Keeps private paths reachable. | ✓ |
| `tests/helpers/` — extend the shipped convention | One mock location project-wide, matching the existing `Arc<Mutex<..>>` pattern. Forces DEFER-02/03 tests to become integration tests seeing only the public API. | |
| Both — `src/` for co-located, `tests/helpers/` for integration | Honest about the two-crate reality. Two mock homes is the fragmentation DEFER-01 exists to end. | |

**User's choice:** `src/` test-support module (D-08)

### Q2 — `mockall` or hand-written mocks? *(DEFER-01 Open Question 2)*

| Option | Description | Selected |
|--------|-------------|----------|
| Hand-written | Matches every mock in the workspace; `mockall` appears in none of the twelve manifests. No new dependency in a graph governed by ADR-0024/ADR-0036. Costs boilerplate. | ✓ |
| Adopt `mockall` | Derive-based mocks, less boilerplate, scales better. A new proc-macro dev-dependency and a second idiom alongside mocks nobody would rewrite. | |
| Hand-written now, revisit under a trigger | ADR-0035 precedent: record a reintroduction condition without building it. | |

**User's choice:** Hand-written (D-09)

### Q3 — Build all five named mocks, or only what the coverage work consumes?

| Option | Description | Selected |
|--------|-------------|----------|
| Demand-driven, with a verdict per name | Build what DEFER-02/03 consume; each of the five names gets built / replaced by X / unnecessary because Y. No mock written that no test calls, nothing silently dropped. | ✓ |
| All five, exactly as named | Literal discharge of the requirement text. `MockUserRepository` duplicates a working in-memory repo; `MockNotificationService` isn't constructible without a public signature change. | |
| All five in substance, reshaped to fit the code | Full conceptual coverage, no fictional constructs. A mapping table to maintain, and infrastructure built ahead of consumers. | |

**User's choice:** Demand-driven, with a verdict per name (D-10)
**Notes:** `UserService.notification_service` is a concrete `Arc<NotificationService>`, but
`register_channel_handler` is public — a `FailingChannelHandler` reaches the failure path with no
signature change. `register_user:228` already handles notification failure non-blockingly.

---

## Phase size & where tests stop

### Q1 — Does Phase 15 land the test-writing, or deliver the gates and hand the bulk forward?

| Option | Description | Selected |
|--------|-------------|----------|
| One phase, all of it | PIPE first (the register's own order), then DEFER, wave-decomposed. Meets ROADMAP criterion 6 as written, no roadmap amendment. The 35-45h figure is a stale upper bound. | ✓ |
| Gates + re-scoped register; tests follow on | Gets the gates live fastest. Leaves criterion 6 unmet, requiring a roadmap amendment and a new phase after 16. | |
| Everything except DEFER-03 | Defers the largest, least predictable block. Splits criterion 6 and defers the concurrency tests with the most defect-finding value. | |

**User's choice:** One phase, all of it (D-11)

### Q2 — What number must `user_service.rs` and `listener.rs` hit for criterion 6?

| Option | Description | Selected |
|--------|-------------|----------|
| ≥ 80% per module, as DEFER-02 names | A phase acceptance criterion verified by module-targeted `cargo llvm-cov`, not a standing CI gate — consistent with D-05. DEFER-03 inherits the same bar since its text names none. | ✓ |
| The workspace floor, applied per module | Reads "the gate" literally; one number governs everything. A higher bar than DEFER-02 asks, on two expensive modules. | |
| Re-measured entry figure plus a justified improvement | Most honest about stale baselines. Not falsifiable at a glance; criterion 6 becomes a judgement call. | |

**User's choice:** ≥ 80% per module, as DEFER-02 names (D-12)

### Q3 — Where does PIPE-05's Code Coverage documentation land?

| Option | Description | Selected |
|--------|-------------|----------|
| `docs/src/contributing/testing-guide.md` | The existing contributor chapter, inside mdbook linkcheck. PIPE-05's `CONTRIBUTING.md` premise corrected as relocated-by-outcome — the class PROJECT.md already records for `STABLE_API.md`. | ✓ |
| Root `CONTRIBUTING.md` stub + content in the book | GitHub surfaces a root file in the PR/issue UI. Reintroduces a root-path doc M11 relocated; the stub can drift. | |
| Create root `CONTRIBUTING.md` as PIPE-05 says | Zero interpretation. Reverses an M11 relocation, splits contributor docs, sits outside linkcheck. | |

**User's choice:** `docs/src/contributing/testing-guide.md` (D-13)
**Notes:** `CONTRIBUTING.md` does not exist anywhere in the tree, and the `cargo tarpaulin`
references PIPE-05 names are only in `.planning/codebase/TESTING.md:319-322`.

### Q4 — How far does the coverage-number correction sweep reach?

| Option | Description | Selected |
|--------|-------------|----------|
| All three instruction files | `CLAUDE.md`, `.github/copilot-instructions.md` and `.planning/codebase/TESTING.md` corrected to cite ADR-0006's number, the real CI mechanism and `cargo-llvm-cov`. Scope guard: coverage-number claims only. | ✓ |
| `TESTING.md` only | Exactly what ADR-0006 flags. Leaves the other two telling every future session a rejected number. | |
| All three plus a full tree-wide hunt | Nothing survives to contradict the gate. Overlaps hard with DOCS-01 in Phase 16. | |

**User's choice:** All three instruction files (D-14)

---

## Claude's Discretion

- PIPE-04's action-version mapping and `actionlint` scope (six workflows exist; the requirement names
  three; D-03 deletes one of the eight references rather than upgrading it).
- `bench-check` shape and caching (`cargo bench --no-run`).
- `.codecov.yml` contents beyond the specified keys, subject to D-02 and D-06.
- The `make coverage` / `make services-up` relationship.
- Naming of the `src/` test-support module, and whether `tests/helpers/` is eventually consolidated.
- Tokio time-control utilities (`pause()`/`advance()`) — std features needing no wrapper.
- Wave decomposition and plan boundaries, subject to D-11's ordering and D-04's two-commit sequence.
- ADR allocation — whether the CI-gate topology gets its own record or lives as an ADR-0006
  amendment. The amendment is mandatory either way.
- Whether the advisory Docker build-time budget at `ci.yml:539` ("Owner: Phase 15 / PIPE") is taken
  up here.
- Whether DEFERRED_COVERAGE's two remaining prerequisites get explicit closure records.

## Deferred Ideas

- Closing Herald's ~14.5-point gap to its ≥ 95% target — recorded and re-measured, not closed.
- Enforcing per-module coverage gates in CI via a report-parsing check script.
- Widening the denominator to `web-server` and `cli` so the shipped binaries are gated.
- Adopting `mockall`, with a named trigger.
- Consolidating `tests/helpers/` into the new `src/` test-support home.
- The native-arm64 CI rework replacing QEMU multi-arch emulation (`ci.yml:525-552`).
- Benchmark regression *detection* (`critcmp`, `github-action-benchmark`) — Epic 25 non-goal.
- A second, feature-scoped coverage measurement (ADR-0006 D-14b's `minio.rs` open question).
