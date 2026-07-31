# Milestone 1 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section (D-17).
That section becomes a pointer to this file. Phases 5, 7, 10 and 13 each add a sibling ledger
(`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`, `milestone-09-12.md`) rather than
growing REQUIREMENTS.md further — REQUIREMENTS.md is already ~4,000 lines and five inline sets of
`file:line`-cited verdicts would make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — the ~40 outstanding Milestone-1 task
items are numbered positions inside `.project/` task-list files with no `REQ-*` key of their own,
so nesting them keeps this ledger joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing
new IDs (D-18).

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-19). This bar exists because "the
code exists" has already produced false-positive completions in this corpus: Milestone 4 Epic 3's
task list is fully checked while three CLI-only dependencies remain unconditional in library builds.

## Verdict legend

| Verdict | Meaning |
|---|---|
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |

## Divergences — shipped code superseded an ingested requirement

> **This divergence is a documented non-goal that shipped anyway.** Epic 9 explicitly declared "no
> REPL or interactive shell" a non-goal (NG-7). An interactive REPL now ships. This is the corpus's
> own evidence for why nothing in this planning record is treated as locked — even an explicit,
> written non-goal was superseded by later work with no recorded decision reversing it.

| Requirement | Ingested position | Shipped position | Verdict |
|---|---|---|---|
| `REQ-cli-interactive-mode` (Epic 9 non-goal NG-7) | "No REPL or interactive shell" — explicitly out of scope | An interactive REPL ships in the Armory CLI | shipped as **an interactive REPL**, not the declared non-goal of no REPL at all; **superseded by shipped code** |
| `REQ-mcp-sse-transport` | `.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md` FR-5 (lines 95-110) specifies `MCPSseAdapter` — "Uses Server-Sent Events (SSE) for receiving messages" | MCP ships on the official `rmcp` 2.1.0 SDK with a Streamable-HTTP transport: `pub struct MCPStreamableHttpAdapter` at `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs:76`; exercised by the passing test `streamable_http_round_trip_with_correct_bearer_token_succeeds` in `tests/integration/mcp_streamable_http_test.rs:176` | shipped as **Streamable-HTTP**, not the specified SSE transport; supersedes `REQ-mcp-sse-transport` (`REQUIREMENTS.md:2408` already records this as `Code diverges`); **superseded by shipped code** |
| `REQ-garrison-longterm-port`, `REQ-garrison-sqlite` | `.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md` specifies a `sqlite-vss` SQLite extension as Garrison's long-term/vector-search port | Semantic retrieval ships as **Sanctum** over **Qdrant** — `pub struct QdrantSanctumAdapter` at `crates/paladin-memory/src/sanctum/qdrant_adapter.rs:59` — plus an in-memory backend — `pub struct InMemorySanctum` at `crates/paladin-memory/src/sanctum/in_memory_adapter.rs:73`, exercised by the passing test `test_store_and_retrieve` in `tests/integration/in_memory_sanctum_tests.rs:38` (that file carries 0 `#[ignore]` attributes). The Qdrant-specific path is additionally covered by `test_store_and_retrieve` in `tests/integration/qdrant_sanctum_tests.rs:63`, but that test carries `#[ignore = "Requires Qdrant running on localhost:6334"]` at line 47 — **present, unproven** for the Qdrant-exerciser half specifically, not upgraded on the strength of the code existing. Missing coverage is supplied by run-2 requirements `REQ-sanctum-port`, `REQ-embedding-port`, `REQ-sanctum-domain-model` | shipped as **Sanctum**/**Qdrant**, not the specified `sqlite-vss` extension; supersedes `REQ-garrison-longterm-port` and `REQ-garrison-sqlite` (`REQUIREMENTS.md:2392,2394` already record these as `Code diverges`); **superseded by shipped code** |

All three rows carry a `file:line` citation plus a named test, example, or command that exercises
the shipped alternative, per the D-19 evidence bar, except where noted above (the Qdrant-specific
exerciser is `present, unproven` rather than upgraded on the strength of the code existing).

Plan 01-05 Task 1 resolved the RECON-08 Epic 10 Task 7.0 dispute below. The per-epic sections
further below are left as headings for plans 01-06 and 01-07 to fill with `REQ-*` rows; this plan
does not author any per-epic row.

## Epic 10 Task 7.0 — dispute resolution (RECON-08)

This section resolves the conflict recorded at `INGEST-CONFLICTS.md:125-127` ("Contradictory Epic 10
completion state"). Order matches that warning: task list first, validation report second.

**1. The task list's claim.**
`.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md` marks all of its own
checklist items complete across parent tasks 0.0 through 6.0 and contains **no Task 7.0 heading
anywhere** (`grep -n "Task 7" tasks-epic10-validation-documentation.md` returns zero matches). Its
own item total, counted deterministically (`grep -c '^\s*- \[x\]'`), is **103** — every one of them
checked, zero unchecked. `REQUIREMENTS.md:2519` already records this epic heading as "(103/103
items; Task 7.0 disputed)".

**2. The validation report's claim.**
`.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md` states, at line 440: "Epic 10
progress: 101 of 102 subtasks (99%)" and at line 441: "Only Task 7.0 (Final Documentation Review)
remains" — and at line 533, in its closing line: "**Next Task:** Task 7.0 - Final Documentation
Review (6 subtasks)". The report never itemizes what those six subtasks are anywhere in its own
body; "(6 subtasks)" is the only detail given, with no subtask-description text to corroborate
elsewhere.

**3. Where both documents make the same claim about the same parent task (Task 6.0), side by side.**
The task list marks `- [x] 6.0 Validation & Quality Assurance (All FRs)` complete, with all sixteen
of its own subtasks (6.1-6.16) individually checked. The validation report's own title and status
line assert the identical claim independently: "**Task:** Task 6.0 - Validation & Quality Assurance
Report … **Status:** ✅ **COMPLETE**" and "Task 6.0 - Validation & Quality Assurance has been
successfully completed with 16 of 16 subtasks (100%) validated." **The two documents agree on Task
6.0** — this is not the disputed claim. The dispute is entirely about whether a Task 7.0 exists
beyond it, which only the validation report asserts.

**4. The search record.** Commands run against this worktree and their results:

```
$ ls .project/Milestone_1-MVP/Epic_10/
epic10.md  prd-epic10-validation-documentation.md  task5.0-completion-summary.md
task6.0-validation-report.md  tasks-epic10-validation-documentation.md
# 5 files. No dedicated "Final Documentation Review" or "Task 7.0" artifact of any kind.

$ grep -rn "Final Documentation Review" .project/
task6.0-validation-report.md:441 and :533 — the only two hits in all 263 corpus documents.

$ grep -rn "Final Documentation Review" docs/
# 0 matches.

$ grep -rniE "documentation review|documentation sign-off|final review checklist" .project/
# Matches only in unrelated documents: this same Epic 10 PRD's own aspirational Phase-6 checklist
# ("Conduct documentation review with fresh eyes", prd-epic10-validation-documentation.md:655) and
# Acceptance Criteria ("Documentation review completed by technical writer (if available)", :694) —
# both pre-execution PRD checklist items, not evidence a Task 7.0 was ever executed or scoped as a
# distinct numbered task — plus incidental hits in Milestone 11 and Milestone 2 documents unrelated
# to Epic 10.

$ grep -rniE "documentation review|documentation sign-off|final review checklist" docs/
docs/src/appendix/contributing-legacy.md:324 — "Documentation Review: Check docs are clear", a
generic contributing-guide checklist item with no connection to Epic 10 or Task 7.0.

$ grep -rln "Task 7.0" .project/Milestone_1-MVP/
task6.0-validation-report.md — the only file in the whole milestone that mentions "Task 7.0".

$ grep -c "Final Documentation Review" .project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md
0

$ grep -c '^\s*- \[x\]' .project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md
103
```

Cross-checked against `.planning/intel/task-completion-state.md`'s deterministic Milestone_1-MVP
"Open items by list" breakdown: Epic 10's task list is **absent** from that list (only eight other
Epic/workstream task lists are named there with nonzero open items), which independently confirms
Epic 10's deterministic open-item count is **0** — corroborating the task list's own 103/103 claim
via a route that does not depend on reading the task list itself.

No trace of "Final Documentation Review" content, a six-subtask breakdown, or any artifact
resembling a documentation sign-off exists anywhere under `.project/Milestone_1-MVP/Epic_10/`,
the rest of `.project/Milestone_1-MVP/`, or `docs/`. The absence is the evidence.

**5. The verdict.** **The validation report is recorded as wrong.** The task list is the
corroborated document: it is internally complete (103/103, no Task 7.0), independently corroborated
by the deterministic checkbox count in `task-completion-state.md`, and no artifact anywhere in the
263-document corpus or the shipped tree supplies content for a Task 7.0 "Final Documentation Review"
of any kind. Epic 10's completion status is classified **`satisfied`** on this point — there is no
outstanding "Final Documentation Review" work item, named or otherwise, and no owner is assigned
because none is needed. Plans 01-06/01-07, when authoring the Epic 10 per-epic `REQ-*` section, use
this verdict rather than re-opening the dispute.

**6. The 102-vs-103 explanation.** The task list's own total is **103** (all of tasks 0.0-6.0,
verbatim from the deterministic checkbox count above). The validation report's total is **102**
("101 of 102 subtasks"), naming a Task 7.0 the task list never contained. The two totals cannot both
describe the same underlying set: if a real six-subtask Task 7.0 existed on top of the task list's
103 checked items, the combined total would be at least 109 (103 + 6), not 102; if the validation
report's 102 is meant to describe tasks 0.0-6.0 alone, that also does not match the task list's own
103. Under neither reading does 102 reconcile against anything the task list actually contains. The
102 figure is therefore not a re-derivation of the task list's total with six items subtracted or
added — it is an unreconciled number, consistent with the "Task 7.0" claim itself being fabricated
rather than a real, differently-counted view of the same work. This ledger uses **103** — the task
list's deterministic, independently-corroborated total — going forward.

## Ingest bookkeeping corrections (RECON-01)

### Battalion base module path

`INGEST-CONFLICTS.md:130-134` ("Contradictory Battalion base module path") records that
`.project/Milestone_1-MVP/Epic_4/epic4.md` names the Battalion base module `battalion/mod.rs`,
matching Appendix B of the project plan, while `.project/Milestone_1-MVP/Paladin Project Completion
Plan.md` names it `battalion/battalion.rs` in its own Epic 4 technical-design section —
contradicting its own Appendix B.

The code-observed answer, confirmed by listing the directory directly:

```
$ ls crates/paladin-core/src/platform/container/battalion/
campaign.rs  chain_of_command.rs  conclave.rs  council.rs  formation.rs  grove.rs  mod.rs  phalanx.rs
```

`crates/paladin-core/src/platform/container/battalion/mod.rs` **exists**;
`crates/paladin-core/src/platform/container/battalion/battalion.rs` **does not**. Two of the three
references (`epic4.md` and both instances of Appendix B) were already right. **The Epic 4 technical
design section of `Paladin Project Completion Plan.md` is the corrected document** — its
`battalion/battalion.rs` reference is wrong and its own Appendix B already disagreed with it.

### Requirement-count discrepancy

Counting the `REQ-*` rows in `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section
deterministically — a grep over the text between that heading (`REQUIREMENTS.md:2361`) and the next
`##` heading (`REQUIREMENTS.md:2542`, `## Milestone 2-3 as-shipped ledger`) — gives:

```
$ awk '/^## Milestone 1 as-shipped ledger/{flag=1; next} /^## /{if(flag){exit}} flag' REQUIREMENTS.md | grep -c '^| REQ-'
112
```

Every other total this corpus reports for the same nominal set:

- `REQUIREMENTS.md:31` (the file's own "How to read this file" summary table): "**Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope."
- `.planning/intel/SYNTHESIS.md:72`: "Requirements extracted: 348 cumulative (run 1: **115**, run 2: 118, run 3: 115)."
- A third, independent cross-check — counting the distinct `## REQ-*` headings in the run-1 section of `.planning/intel/requirements.md` (before the run-2 `MODE=merge` marker at line 1195) — also returns **115**.

**112 enumerated ledger rows, 115 reported IDs. The difference is exactly three, and it is explained
by how competing-variant pairs are recorded, not by any ID actually missing.** Three of the 115
ingested run-1 IDs — `REQ-herald-trait-v2`, `REQ-temperature-range-v2`, `REQ-test-coverage-target-v2`
— are each the "-v2" half of a competing-variant pair whose "-v1" half already occupies its own row
in the as-shipped ledger:

- `REQ-temperature-range-v1 / -v2` — `REQUIREMENTS.md:2469`
- `REQ-herald-trait-v1 / -v2` — `REQUIREMENTS.md:2490`
- `REQ-test-coverage-target-v1 / -v2` — `REQUIREMENTS.md:2537`

Both IDs of each pair are genuinely present in the file — the "-v2" half is fully described in
`## Competing variants (preserved unmerged)` (`REQUIREMENTS.md:1661,1677,1768`) — but the ledger
folds each pair into a single `| REQ-X-v1 / -v2 | Variant (group N) |` row rather than giving the
"-v2" ID a distinct grep-matchable row of its own. So a literal `grep -c '^| REQ-'` reads 112 while
"all 115 IDs are accounted for" is also true; the two figures measure different things (distinct
ledger rows vs. distinct requirement IDs), not competing counts of the same thing, and neither is
wrong. **This ledger uses 112 for "number of ledger rows in the Milestone 1 as-shipped ledger" and
115 for "number of distinct run-1 requirement IDs" going forward — the two labels are not
interchangeable, and a future reference to "the Milestone 1 requirement count" must say which one
it means.**

### Epic 1 — Paladin Domain Foundation

No open task items (182/182 complete per `intel/task-completion-state.md`) — every row below carries
no nested block.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-entity | satisfied | `PaladinData` struct at `crates/paladin-core/src/platform/container/paladin.rs:142`, `Paladin` type alias at `paladin.rs:229`; exercised by `test_paladin_data_default` (`paladin.rs:350`) |
| REQ-paladin-builder | satisfied | `PaladinBuilder` struct at `src/application/services/paladin/paladin_builder.rs:76`; exercised by `test_builder_validation_empty_prompt` (`paladin_builder.rs:1346`) |
| REQ-paladin-config | satisfied | `PaladinConfig` struct at `crates/paladin-core/src/platform/container/paladin_config.rs:44`; exercised by `test_paladin_config_defaults` (`paladin_config.rs:173`) |
| REQ-paladin-port | satisfied | `PaladinPort` trait (`execute`/`execute_stream`) at `crates/paladin-ports/src/output/paladin_port.rs:631,752`; exercised end-to-end through `IntegrationMockPaladinPort`'s trait impl (`tests/integration/commander_integration_tests.rs:78`) by `test_commander_executes_formation_end_to_end` (`commander_integration_tests.rs:150`) |
| REQ-paladin-execution-service | satisfied | `PaladinExecutionService` struct at `src/application/services/paladin/paladin_execution_service.rs:105`, `execute()` at `:470`; exercised by `test_paladin_without_garrison_single_turn` (`tests/integration/paladin_garrison_integration_test.rs:143`), which constructs the service directly and asserts `execute()` succeeds |
| REQ-paladin-error-handling | satisfied | `PaladinError` enum at `crates/paladin-core/src/platform/container/paladin_error.rs:19`; exercised by `test_is_retryable` (`paladin_error.rs:100`) and `test_garrison_error_conversion` (`:116`) |
| REQ-paladin-observability | present, unproven | Code uses `log`/`env_logger` (`use log::{debug, error, info, warn};` at `paladin_execution_service.rs:69`) alongside the workspace's `tracing-subscriber` dependency (`Cargo.toml:120`) — the same divergence `REQUIREMENTS.md:2541` already recorded (PRD specified `tracing`, code uses `log`). Logging calls are real and present at the citation, but no named test asserts log output content, so the exerciser half of the bar is unmet |
| REQ-paladin-testing-infra | satisfied | `MockLlmAdapter` at `crates/paladin-llm/src/mock.rs:73` (and the parallel `tests/helpers/mock_llm_adapter.rs:66`); exercised by `test_mock_returns_default_response` (`crates/paladin-llm/src/mock.rs:412`) and used throughout the integration suite (e.g. `paladin_garrison_integration_test.rs`) |

### Epic 2 — Garrison Memory System

4 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md`.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-garrison-entry | satisfied | `GarrisonEntry` struct at `crates/paladin-core/src/platform/container/garrison.rs:41`; exercised by `test_garrison_entry_creation` (`garrison.rs:422`) |
| REQ-garrison-windowing | satisfied | `ConversationHistory` windowing logic in `garrison.rs`; exercised by `test_conversation_history_windowing_by_count` (`garrison.rs:490`) |
| | | **Nested outstanding item:** `- [ ] 9.14 Write test: \`test_large_conversation_performance\` - benchmark with 1000 entries (future enhancement)` (`tasks-garrison-memory-system.md:222`) — **deferred with reason**. `REQUIREMENTS.md:2549` already records this exact test as "deferred → v2", and `STATE.md`'s Deferred Items table records it "Deferred — marked future enhancement" (Ingest run 1). No code change is expected here; the deferral is the task list's own words. |
| REQ-garrison-port | satisfied | `GarrisonPort` trait at `crates/paladin-ports/src/output/garrison_port.rs:380`; exercised via `InMemoryGarrison`'s trait impl by `test_remember_and_recall` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:229`) |
| REQ-garrison-longterm-port | superseded by shipped code | See the Divergences table above (`REQ-garrison-longterm-port`, `REQ-garrison-sqlite` row) — semantic retrieval ships as Sanctum/Qdrant, not a `sqlite-vss` extension of this port. Not re-decided here. |
| REQ-garrison-in-memory | satisfied | `InMemoryGarrison` struct at `crates/paladin-memory/src/garrison/in_memory_garrison.rs:58`; exercised by `test_remember_and_recall` (`:229`) and `test_importance_based_eviction` (`:354`) |
| REQ-garrison-sqlite | satisfied | The SQLite Garrison adapter itself shipped as specified: `SqliteGarrison` struct at `crates/paladin-memory/src/garrison/sqlite_garrison.rs:52`; exercised by `test_sqlite_remember_and_recall` (`sqlite_garrison.rs:521`) and `test_sqlite_persistence` (`:537`). Only the `sqlite-vss` **vector-search** extension diverged — that half is recorded in the Divergences table above (superseded by Sanctum/Qdrant), not repeated here as a contradiction. |
| REQ-garrison-paladin-integration | satisfied | Exercised by `test_paladin_multi_turn_conversation` (`tests/integration/paladin_garrison_integration_test.rs:169`) and `test_paladin_without_garrison_single_turn` (`:143`) |
| REQ-garrison-config | satisfied | `GarrisonSettings` struct at `crates/paladin-memory/src/config/garrison.rs:11`; exercised by `test_garrison_settings_validation_success` (`:100`) |
| REQ-garrison-errors | satisfied | `GarrisonError` enum at `crates/paladin-core/src/platform/container/garrison_error.rs:8`; exercised by `test_storage_error_display` (`:51`) |
| REQ-garrison-testing | present, unproven | The bulk of the Garrison testing infrastructure is real and passing (all rows above cite live tests; the task list's own annotation records "19 total: 12 paladin_garrison + 7 sqlite_garrison" integration tests). What remains unconfirmed is the closure claim itself — see the two nested items below, neither of which has a citable artifact in this tree. |
| | | **Nested outstanding item:** `- [ ] 11.0 Final Validation and Cleanup` (`tasks-garrison-memory-system.md:246`) — **present, unproven** (parent). Four of its six children (11.1-11.4: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build --release`) are marked done; the two below remain open and are the reason the parent stays unchecked — this is not the "stale parent over complete subtasks" shape seen elsewhere in this corpus. |
| | | **Nested outstanding item:** `- [ ] 11.5 Verify test coverage ≥ 80% using \`cargo llvm-cov\`` (`tasks-garrison-memory-system.md:251`) — **genuinely outstanding**. No coverage measurement for Garrison exists in this planning record as of this plan (`01-coverage-measurement.md`, produced by a sibling plan in this phase, does not exist yet). Forward owner: **QUAL-01**. |
| | | **Nested outstanding item:** `- [ ] 11.6 Review all acceptance criteria from PRD - ensure all met` (`tasks-garrison-memory-system.md:252`) — **genuinely outstanding**. No PRD-acceptance review artifact exists for Epic 2. Forward owner: **GAP-06** (matches `REQUIREMENTS.md:2557`'s existing "Partial → GAP-06, QUAL-01" note). |

### Epic 3 — Arsenal Tool System

3 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_3/tasks-arsenal-tool-system.md`.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-arsenal-domain-types | satisfied | `Armament`/`ArmamentCall`/`ArmamentResult` at `crates/paladin-core/src/platform/container/arsenal/core.rs:17,36,80`; exercised by `invoke_routes_to_the_registered_client_and_returns_real_output` (`src/application/services/arsenal/arsenal_execution_service.rs:303`) |
| REQ-arsenal-port | satisfied | `ArsenalPort` trait at `crates/paladin-ports/src/output/arsenal_port.rs:470`; exercised via `MockArsenalAdapter`'s trait impl by `test_mock_arsenal_invoke_success` (`tests/helpers/mock_arsenal_adapter.rs:246`). This upgrades the 2026-01 "services untested → QUAL-02" note (`REQUIREMENTS.md:2564`): `arsenal_execution_service.rs` now carries a full passing test module (re-verified 2026-07-31). |
| REQ-mcp-protocol | superseded by shipped code | `MCPClient` at `src/infrastructure/adapters/arsenal/mcp_protocol.rs:62` is, by its own doc comment (`mcp_protocol.rs:1,12-16`), "a thin facade over `rmcp::service::RunningService`" — the official `rmcp` 2.1.0 SDK performs the handshake, superseding the hand-rolled JSON-RPC client the Epic 3 PRD specified. Exercised by the passing test suite starting at `mcp_protocol.rs:370`. Same class of divergence as the `REQ-mcp-sse-transport` row already recorded in the Divergences table above, but this specific ID was not itself in that table — recorded here instead. |
| REQ-mcp-stdio-transport | satisfied | `MCPStdioAdapter` at `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs:34`, built on `rmcp::transport::TokioChildProcess`; exercised by `test_stdio_connect` (`tests/integration/mcp_stdio_test.rs:23`) and `test_stdio_invoke_tool_calculator` (`:110`) |
| REQ-mcp-sse-transport | superseded by shipped code | See the Divergences table above — shipped as Streamable-HTTP (`MCPStreamableHttpAdapter`), not SSE. Not re-decided here. |
| REQ-arsenal-builder-integration | satisfied | `PaladinBuilder::with_arsenal_registry` at `src/application/services/paladin/paladin_builder.rs:685`; exercised by `test_builder_auto_registers_handoff_tool_when_configured` (`paladin_builder.rs:2098`) |
| REQ-arsenal-resource-controls | satisfied | `TimeoutWrapper`/`ConcurrencyLimiter` at `src/infrastructure/adapters/arsenal/resource_controls.rs:53,160`; exercised by `test_concurrency_limit_enforced` (`resource_controls.rs:280`) |
| REQ-arsenal-resilience | satisfied | `ArsenalError::ToolNotFound` failure path exercised by `invoke_with_no_serving_client_returns_tool_not_found` (`arsenal_execution_service.rs:341`); `grep -c '#\[ignore' src/infrastructure/adapters/arsenal/*.rs src/application/services/arsenal/*.rs` returns 0. This upgrades the 2026-01 "Partial → QUAL-04 (failure paths untested)" note (`REQUIREMENTS.md:2570`) — the failure paths are tested and none is `#[ignore]`d, re-verified 2026-07-31. |
| REQ-arsenal-context-injection | satisfied | Exercised by `test_tool_invocation_and_injection` (`tests/integration/context_injection_test.rs:324`) and `test_paladin_continues_after_tool_failure` (`:399`) |
| | | **Nested outstanding item:** `- [ ] 9.30 Commit all changes with message: "feat: implement Arsenal Tool System (Epic 3)"` (`tasks-arsenal-tool-system.md:302`) — **superseded by shipped code**. A git-workflow step, not a functional requirement; no requirement-bearing row above is a closer semantic match, so it is recorded here against the epic's last row. The literal commit message this text describes has no discoverable trace as its own commit, but the deliverable it describes — the Arsenal Tool System — is fully present and compiles in the current `release/v0.7.0` tree (every citation above resolves against it), which is what "shipped" means for a housekeeping step whose only purpose was landing the code. |
| | | **Nested outstanding item:** `- [ ] 9.31 Push feature branch: \`git push -u origin feature/epic3-arsenal-tool-system\`` (`tasks-arsenal-tool-system.md:303`) — **superseded by shipped code**. Same reasoning as 9.30 above: the named feature branch has no discoverable trace, but the code it would have carried already ships on `release/v0.7.0`. |
| | | **Nested outstanding item:** `- [ ] 9.31 Push feature branch: \`git push -u origin feature/epic3-arsenal-tool-system\`` (`tasks-arsenal-tool-system.md:304` — the source file literally duplicates line 303 verbatim at this line, a defect in the source document itself, not a second distinct task) — **superseded by shipped code**, same reasoning. |

### Epic 4 — Battalion Orchestration

2 open task items per `intel/task-completion-state.md` (parent tasks 6.0 and 7.0), both under
`.project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md`. Rows whose subject is
`BattalionConfig`, `BattalionResult` or the Formation minimum Paladin count link to
[`ADR-0001`](../decisions/0001-battalion-config.md), [`ADR-0002`](../decisions/0002-battalion-result.md)
and [`ADR-0003`](../decisions/0003-formation-min-paladins.md) respectively rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-config-v1 | satisfied | See [ADR-0001](../decisions/0001-battalion-config.md). `BattalionConfig` struct at `crates/paladin-core/src/platform/container/battalion/mod.rs:37` is this exact field set (ADR-0001's Considered Options: "chosen"); exercised by `test_battalion_config_builder` (`battalion/mod.rs:886`). ADR-0001's "must change" conformance note applies only to the unrelated placeholder struct at `citadel.rs:280` (renamed to `BattalionCheckpointConfig` by GAP-07) — this row's struct is unaffected and already conforms. |
| REQ-battalion-config-v2 | superseded by shipped code | See [ADR-0001](../decisions/0001-battalion-config.md), which records this variant "rejected; not what shipped" — its `retry_attempts: u32` and `enable_checkpointing: bool` fields are absent from the tree, and the `description` field it proposed dropping was in fact kept. Not re-decided here. |
| REQ-battalion-error-strategy | satisfied | `AggregatedError` at `crates/paladin-battalion/src/error_aggregation.rs:13`; exercised by `test_add_error` (`error_aggregation.rs:186`) |
| REQ-battalion-retry-policy | satisfied | `RetryPolicy` struct at `battalion/mod.rs:189`, `calculate_retry_delay` at `crates/paladin-battalion/src/retry.rs:40`; exercised by `test_calculate_retry_delay_linear` (`retry.rs:115`) |
| REQ-formation-min-paladins-v1 | satisfied | See [ADR-0003](../decisions/0003-formation-min-paladins.md). `Formation::validate` at `crates/paladin-core/src/platform/container/battalion/formation.rs:109-111` currently rejects fewer than 2 Paladins, exactly this row's specification; exercised by `test_formation_validation_minimum_paladins` (`formation.rs:173`). ADR-0003 decides this bound **will relax to 1** — Formation's own rejection contradicts the Commander's passing `test_auto_selects_formation_for_single_paladin` (`crates/paladin-battalion/src/commander.rs:1912`), which routes a single Paladin to Formation. Per ADR-0003's "must change" conformance, this row records the code as it stands today; the change is owned by **GAP-07**, not asserted here as already done. |
| REQ-formation-construction | satisfied | Exercised by `test_formation_creation_valid` (`formation.rs:163`) |
| REQ-formation-execution | satisfied | Exercised by `test_sequential_execution_success` (`crates/paladin-battalion/src/formation_service.rs:472`) |
| REQ-formation-output | satisfied | Exercised by `test_output_passing_between_paladins` (`formation_service.rs:493`) |
| REQ-phalanx-construction | satisfied | Exercised by `test_phalanx_creation_valid` (`crates/paladin-core/src/platform/container/battalion/phalanx.rs:177`) |
| REQ-phalanx-concurrency | satisfied | `ConcurrencyLimiter`-backed `max_concurrency` exercised by `test_collect_all_with_concurrency_limit` (`crates/paladin-battalion/src/phalanx_service.rs:613`) and validated under real load by `test_load_phalanx_concurrent_execution` (`tests/integration/battalion/load_test.rs:192`) and `test_stress_high_concurrency_limit` (`load_test.rs:273`). This upgrades the 2026-01 "Partial → GAP-02 (concurrency claims unvalidated)" note (`REQUIREMENTS.md:2586`) — concurrency is now validated under load, re-verified 2026-07-31. |
| REQ-phalanx-aggregation | satisfied | Exercised by `test_collect_all_strategy_success` (`phalanx_service.rs:593`) |
| REQ-campaign-graph | satisfied | `EdgeCondition` enum and `CampaignEdge` struct at `crates/paladin-core/src/platform/container/battalion/campaign.rs:34,50` (petgraph-backed); exercised by `test_add_edge_success` (`campaign.rs:349`) |
| REQ-campaign-edge-conditions | satisfied | `EdgeCondition::Always` exercised end-to-end by `test_branching_campaign_fan_out` (`tests/integration/battalion_campaign_integration_test.rs:154`) |
| REQ-campaign-execution | satisfied | `CampaignExecutionService::execute` at `crates/paladin-battalion/src/campaign_service.rs:104`; exercised end-to-end by `test_linear_campaign_execution` (`tests/integration/battalion_campaign_integration_test.rs:121`). This upgrades the 2026-01 "Verify → QUAL-02 (`campaign_service.rs` at 4.26% coverage)" note (`REQUIREMENTS.md:2590`) — that figure measured only in-crate unit tests (`campaign_service.rs`'s own `#[cfg(test)]` module has just `test_service_creation`), but the integration suite exercises `execute()` directly and passes with 0 `#[ignore]`s, re-verified 2026-07-31. The low unit-file coverage figure itself is not re-measured here (that is Phase 1's coverage-measurement plan); this row only re-verifies that a real, passing, non-ignored exerciser exists. |
| REQ-chain-of-command-construction | satisfied | `ChainOfCommand` struct at `crates/paladin-core/src/platform/container/battalion/chain_of_command.rs:64`; exercised by `test_chain_of_command_new_with_valid_setup` (`tests/unit/battalion/chain_of_command_tests.rs:34`) |
| REQ-chain-of-command-execution | satisfied | `ChainOfCommandExecutionService::execute` at `crates/paladin-battalion/src/chain_of_command_service.rs:125`; exercised end-to-end by `test_commander_executes_chain_of_command_end_to_end` (`tests/integration/commander_integration_tests.rs:283`). This upgrades the 2026-01 "Verify → GAP-01" note (`REQUIREMENTS.md:2591`) — GAP-01's own description already stated shipped code contains `chain_of_command_service.rs`; a full run of `cargo test --test lib chain_of_command` on 2026-07-31 shows 54 passed, 0 failed, 0 ignored. |
| REQ-chain-of-command-aggregation | satisfied | Exercised by `test_broadcast_executes_all_specialists` (`tests/unit/battalion/chain_of_command_service_tests.rs:302`), which aggregates concurrent specialist results |
| REQ-battalion-status | satisfied | `BattalionStatus` enum at `battalion/mod.rs:471`; exercised by `test_sequential_execution_success` (`formation_service.rs:472`), which asserts `battalion_result.status == BattalionStatus::Completed` |
| REQ-battalion-logging | present, unproven | `log::info!`/`warn!` calls exist at `formation_service.rs:58,156,173` and the equivalent call sites in `phalanx_service.rs`, `campaign_service.rs` and `chain_of_command_service.rs`; no named test asserts log output content, so the exerciser half of the bar is unmet |
| REQ-battalion-cancellation | present, unproven | `CancellationToken`-based `execute_with_cancellation` at `phalanx_service.rs:151`, exercised by `test_cancellation_support` (`phalanx_service.rs:758`) — proven for **Phalanx only**. Formation, Campaign and ChainOfCommand expose no equivalent cancellation entry point (`grep -rn "execute_with_cancellation" crates/paladin-battalion/src/` returns only the Phalanx and Commander pass-through sites), so the requirement as a battalion-wide capability is citation-backed for one of four patterns and untested for the rest. Forward note: **GAP-02**. |
| | | **Nested outstanding item:** `- [ ] 6.0 Implement Chain of Command Pattern (Phase 2 - Hierarchical Delegation)` (`tasks-battalion-orchestration.md:258`) — **satisfied** (parent checkbox stale). All 42 of its own subtasks (6.1-6.42) are individually checked, and direct re-verification confirms the code: `ChainOfCommand` (`chain_of_command.rs:64`), `ChainOfCommandExecutionService` (`chain_of_command_service.rs`), and a full `cargo test --test lib chain_of_command` run on 2026-07-31 passing 54/54 with 0 ignored (see `REQ-chain-of-command-execution` above). This is the same "stale parent over complete subtasks" shape `REQUIREMENTS.md:2573` already flagged in its own heading ("tasks 6.0 and 7.0 open"), and matches the pattern Milestone 1 run 1 already found for this exact epic (Chain of Command wiring existed despite the January task list marking it incomplete). |
| | | **Nested outstanding item:** `- [ ] 7.0 Integration Testing, Performance Validation & Documentation` (`tasks-battalion-orchestration.md:302`) — **satisfied** (parent checkbox stale). All 22 of its own subtasks (7.1-7.22) are individually checked; re-verified: `tests/integration/battalion/load_test.rs` exists with 5 real, non-`#[ignore]`d load/stress tests (`test_load_formation_50_concurrent_battalions` at `:102`, `test_load_phalanx_concurrent_execution` at `:192`, `test_stress_high_concurrency_limit` at `:273`), `crates/paladin-battalion/benches/battalion_benchmarks.rs` exists, and `examples/chain_of_command_delegation.rs` exists and compiles. `docs/BATTALION.md` (7.9's literal path) does not exist at that path, but its content shipped as `docs/src/appendix/battalion-patterns-guide.md` — the same mdbook relocation pattern `PROJECT.md`'s ARCH-05 already records for other Milestone-1 docs deliverables (relocated, not missing). |

### Epic 5 — Commander Strategy Router

4 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_5/tasks-commander-strategy-router.md`. Rows whose subject is
`BattalionConfig`, `BattalionResult` or the Formation minimum Paladin count link to the same three
ADRs Epic 4 uses above, rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-config-v2 | superseded by shipped code | See [ADR-0001](../decisions/0001-battalion-config.md). Same row/reasoning as Epic 4's `REQ-battalion-config-v2` above; not re-decided per-epic. |
| REQ-battalion-result-v2 | superseded by shipped code | See [ADR-0002](../decisions/0002-battalion-result.md). The shipped `BattalionResult` (`battalion/mod.rs:549`) is a merged superset; this variant's `execution_time_ms` is displaced by `per_paladin_times`, and its `errors: Vec<PaladinError>` is displaced by `node_errors: Vec<NodeError>` (serialization reason recorded in the ADR). Not re-decided here. |
| REQ-formation-min-paladins-v2 | genuinely outstanding | See [ADR-0003](../decisions/0003-formation-min-paladins.md). The Commander-level half of this variant (construction validates only ≥1 Paladin, Auto routes a single Paladin to Formation) is real: `test_auto_selects_formation_for_single_paladin` (`crates/paladin-battalion/src/commander.rs:1912`) passes today. But the full behavioral claim this row makes — that a single-Paladin Battalion **executes** via Formation rather than failing — does not hold: `Formation::validate` (`crates/paladin-core/src/platform/container/battalion/formation.rs:109-111`) still rejects it, per ADR-0003's own contradiction record. No end-to-end test exists where a real single-Paladin Formation execution succeeds (the passing test only proves strategy *selection*, not successful execution). Forward owner: **GAP-07**, which lands ADR-0003's relaxed bound. |
| REQ-commander-construction | satisfied | `Commander` struct at `commander.rs:151`, `CommanderBuilder` at `:1272`; exercised by `test_commander_builder_success` (`commander.rs:1689`), `test_commander_builder_missing_paladins` (`:1728`) and `test_commander_builder_invalid_config` (`:1767`) |
| REQ-commander-strategy-types | satisfied | `BattalionStrategy` enum at `crates/paladin-core/src/platform/container/battalion/mod.rs:375`; exercised by `test_commander_all_strategies` (`commander.rs:1790`) |
| REQ-commander-auto-selection | satisfied | `analyze_and_select` exercised by 11 passing keyword-selection tests including `test_auto_selects_campaign_for_workflow_keywords`. This upgrades the 2026-01 "Partial → GAP-05 (one failing keyword test)" note (`REQUIREMENTS.md:2607`): the task list's own line 99 (`tasks-commander-strategy-router.md:99`) records `test_auto_selects_campaign_for_workflow_keywords` as "(FAILING - needs fix)", but `cargo test -p paladin-battalion --lib commander:: -- --test-threads=4` run on 2026-07-31 shows this test, and all 11 auto-selection tests, passing with 0 failures. See the nested item below. |
| | | **Nested outstanding item:** `- [ ] 3.11 Write unit test: test_auto_selects_campaign_for_workflow_keywords (FAILING - needs fix)` (`tasks-commander-strategy-router.md:99`) — **satisfied** (checkbox stale). Directly re-run 2026-07-31: `test commander::tests::test_auto_selects_campaign_for_workflow_keywords ... ok`. Whatever caused the January failure has since been fixed; no trace of the original bug remains in the tree. |
| REQ-commander-execute | satisfied | `Commander::execute` at `commander.rs:337`; exercised by `test_execute_resolves_auto_strategy` (`commander.rs:2063`), `test_execute_routes_to_campaign_service` (`:2006`) and `test_execute_routes_to_chain_service` (`:2035`) |
| REQ-commander-result-normalization | satisfied | `BattalionResult` metadata population exercised by `test_result_contains_telemetry_metadata` (`commander.rs:2155`). This upgrades the 2026-01 "Partial → GAP-04 (task 5.0 open)" note (`REQUIREMENTS.md:2609`) — see the nested items below, both of which re-verify as done. |
| | | **Nested outstanding item:** `- [ ] 5.0 Implement result normalization and telemetry metadata` (`tasks-commander-strategy-router.md:122`) — **satisfied** (parent checkbox stale). 13 of its 15 children are checked; the two unchecked children (5.10, 5.14, both below) are directly re-verified as implemented and tested, so nothing in this parent's scope is actually outstanding. |
| | | **Nested outstanding item:** `- [ ] 5.10 Implement metadata export to file if \`metadata_output_dir\` is configured (deferred - requires file I/O)` (`tasks-commander-strategy-router.md:132`) — **satisfied** (checkbox stale). `export_metadata` at `commander.rs:880` implements exactly this; exercised by `test_metadata_export_creates_file` (`commander.rs:2894`) and `test_metadata_export_correct_naming` (`:2932`), both passing. The "(deferred - requires file I/O)" annotation does not match the shipped tree. |
| | | **Nested outstanding item:** `- [ ] 5.14 Write unit test: test_metadata_export_to_file (deferred - requires file I/O setup)` (`tasks-commander-strategy-router.md:136`) — **satisfied** (checkbox stale). `test_metadata_export_json_structure` (`commander.rs:2980`) and `test_metadata_export_no_dir_configured` (`:3048`) are exactly this test, under different names than the task list anticipated; both pass. |
| REQ-commander-error-strategy | satisfied | Base capability proven by `test_error_handling_fail_fast` (`commander.rs:3072`), `test_error_handling_continue_on_error` (`:3108`) and `test_error_handling_retry_then_continue` (`:3141`), all passing. A residual caveat: 4 edge-case tests remain `#[ignore]`d with empty bodies — `test_fail_fast_stops_on_first_error` (`:2180`), `test_continue_on_error_collects_all_errors` (`:2188`), `test_retry_then_continue_retries_failed_paladins` (`:2196`), `test_partial_results_returned_with_errors` (`:2204`) — confirming the 2026-01 "4 remain in commander.rs" count (`REQUIREMENTS.md:2610`) still holds, re-verified 2026-07-31. Forward note: **QUAL-04**. |
| REQ-commander-config-passthrough | satisfied | Exercised by `test_config_passthrough_to_services` (`commander.rs:2212`) |
| REQ-commander-service-composition | satisfied | Exercised by `test_execute_routes_to_campaign_service` (`commander.rs:2006`) and `test_execute_routes_to_chain_service` (`:2035`) |
| REQ-commander-telemetry | satisfied | `export_metadata` at `commander.rs:880`; exercised by `test_metadata_export_creates_file` (`:2894`) and `test_metadata_export_json_structure` (`:2980`). This upgrades the 2026-01 "Partial → GAP-04" note (`REQUIREMENTS.md:2613`), whose own "Tree observation: export path exists at `crates/paladin-battalion/src/commander.rs:870`" is confirmed (the current line is `880`, a small drift consistent with intervening commits, not a contradiction). |
| REQ-commander-validation | satisfied | Exercised by `test_commander_builder_missing_paladins` (`commander.rs:1728`) and `test_commander_builder_invalid_config` (`:1767`) |

### Epic 6 — Provider Expansion

19 open task items per `intel/task-completion-state.md` — the single largest concentration in
Milestone 1, all under `.project/Milestone_1-MVP/Epic_6/tasks-provider-expansion.md` task 7.0 and
its 18 subtasks. The row whose subject is the DeepSeek temperature range links to
[ADR-0004](../decisions/0004-temperature-validation.md) rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-llm-port-interface | satisfied | `ProviderCapabilities` struct at `crates/paladin-ports/src/output/llm_port.rs:754`; `get_capabilities()` trait method at `llm_port.rs:1264`; exercised by `test_get_capabilities` (`crates/paladin-llm/src/openai/adapter.rs:698`) and `test_deepseek_provider_capabilities` (`crates/paladin-llm/src/deepseek/adapter.rs:618`) |
| REQ-deepseek-adapter | satisfied | `DeepSeekAdapter`/`DeepSeekConfig` at `crates/paladin-llm/src/deepseek/adapter.rs:212,27`; exercised by `test_deepseek_adapter_creation` (`:607`) and `test_deepseek_provider_capabilities` (`:618`), both passing on a fresh `cargo test -p paladin-llm --lib --all-features` run (2026-07-31, 67 passed / 0 failed). The task list's own "15.02% coverage" figure (`REQUIREMENTS.md:2621`) is carried forward from the prior measurement, not re-run here — no `cargo-llvm-cov` binary is available in this sandbox (same blocker plan 01-04/RECON-07 recorded); a fresh figure is that plan's output, not this one's |
| REQ-anthropic-adapter | satisfied | `AnthropicAdapter`/`AnthropicConfig` at `crates/paladin-llm/src/anthropic/adapter.rs:119,29`; exercised by `test_anthropic_adapter_creation` and `test_anthropic_provider_capabilities` (same file), both passing in the same 2026-07-31 run. The "28.19% coverage" figure (`REQUIREMENTS.md:2622`) is likewise carried forward, not re-measured |
| REQ-provider-configuration | satisfied | `LlmProviderFactory::create()` at `crates/paladin-llm/src/provider_factory.rs:62`, matching provider names to `openai`/`deepseek`/`anthropic`; `LlmConfig::default_provider` at `crates/paladin-llm/src/config/llm.rs:27`; exercised by `test_factory_creation` and `test_unknown_provider_returns_error` (`provider_factory.rs:163,169`) |
| REQ-provider-backward-compat | satisfied | `test_llm_config_default` (`crates/paladin-llm/src/config/llm.rs:123`) asserts `default_provider == Some("openai")`, i.e. OpenAI remains the default when no provider is configured, matching the requirement |
| REQ-provider-error-mapping | satisfied | Each adapter maps HTTP status codes to `LlmError` variants directly at the call site — e.g. `crates/paladin-llm/src/deepseek/adapter.rs:343-350` maps 401→`AuthenticationError`, 429→`RateLimitExceeded`, 404→`ModelNotAvailable`, 400→`InvalidPrompt`, else→`ProcessingError`. **Finding**: a separate `LlmProviderError` type with its own `From<LlmProviderError> for LlmError` impl exists at `crates/paladin-llm/src/error.rs:16,54`, but `grep -rn "LlmProviderError" crates/paladin-llm/src/` outside that one file returns zero matches — no adapter actually constructs it. The mapping the requirement asks for is real, but via direct `LlmError` construction at each site, not through the named `LlmProviderError` conversion path the file's own doc comment describes |
| REQ-provider-testing | present, unproven | Mocked-HTTP unit tests exist as source but are split across two locations with different fates. The tests actually wired into a runnable target are the 67 passing `cargo test -p paladin-llm --lib --all-features` tests above (config/adapter-creation/capabilities level, not HTTP-mock level). **Finding**: `tests/unit/llm/{deepseek_adapter_test,anthropic_adapter_test,provider_factory_test}.rs` — the richer mockito-based suite the task list's task 6.0 claims at line 200 ("27 new tests: 9 DeepSeek, 10 Anthropic, 8 Factory", including `test_deepseek_auth_failure_401`, `test_deepseek_rate_limit_429` at `deepseek_adapter_test.rs:119,148`) — is **dead code**: `tests/unit/mod.rs` (the `[[test]] name = "unit"` binary's entry point, `Cargo.toml:172-173`) does not declare `pub mod llm;` anywhere, so these three files are never compiled into any test binary and never run. Task 6.0 is checked `[x]` on the strength of tests that do not execute. The live-API integration suite (`tests/integration/llm_live_api_tests.rs`) does compile and run behind the `live-api-tests` feature flag (`Cargo.toml:265`), un-deferred by ingest run 2 per `STATE.md` §Deferred Items — but `require_api_key()` at `llm_live_api_tests.rs:65` panics on a missing key by design (confirmed by direct read, matching `STATE.md`'s "two contradictions are live in shipped code" note), reversing the graceful-skip semantics VERIFY-06 is scoped to resolve. Neither gap is a fabricated deferral — both are re-verified against the tree, not carried from the January note |
| REQ-provider-documentation | satisfied | `docs/src/appendix/provider-expansion.md` (521 lines) and `docs/src/contributing/contributing-providers.md` (458 lines) — the same mdbook-relocation pattern already recorded elsewhere in this ledger for other Milestone-1 docs deliverables (originally `docs/PROVIDER_EXPANSION.md` / `docs/CONTRIBUTING_PROVIDERS.md` per the task list, relocated not missing) |
| REQ-temperature-range-v1 / -v2 | superseded by shipped code | See [ADR-0004](../decisions/0004-temperature-validation.md), which records this as `must change`: `ProviderCapabilities` has no `temperature_range` field today, so neither the v1 global-clamp nor the v2 DeepSeek-specific position is currently reachable through the port boundary as the ADR's provider-aware design requires. The builder's shipped clamp is `[0.0, 1.0]` (`paladin_builder.rs:1112-1117`, matching v1 as today's fallback default); the DeepSeek `0.0-2.0` range v2 asks for remains unreachable until **GAP-07** lands the field. Not re-decided here |
| | | **Nested outstanding item:** `- [ ] 7.0 Write integration tests for live API validation (DEFERRED - unit tests with mocks provide sufficient coverage)` (`tasks-provider-expansion.md:225`) — **present, unproven** (parent). The task's own inline annotation calls this deferred, but per the second prohibition a deferral verdict needs a citation to an actual deferring document, not an inference from the checkbox text. `STATE.md` §Deferred Items records this row differently: "Un-deferred by run 2 — suite ships behind `live-api-tests`; only the skip-vs-fail semantics remain open (VERIFY-06)". The suite does ship (`tests/integration/llm_live_api_tests.rs` exists, gated by the real `live-api-tests` feature), so it is not `genuinely outstanding` either — it is real code with one unresolved semantic question, hence `present, unproven` rather than either extreme. Forward owner: **VERIFY-06** |
| | | **Nested outstanding item:** `- [ ] 7.1 Create \`tests/integration/llm/mod.rs\` for integration test organization` (`tasks-provider-expansion.md:226`) — **superseded by shipped code**. This exact reorganization never happened at this path, but the live-API tests it would have organized ship as a single flat file, `tests/integration/llm_live_api_tests.rs`, achieving the same functional purpose (a dedicated home for live-API tests) via a different structure |
| | | **Nested outstanding item:** `- [ ] 7.2 Create \`tests/integration/llm/deepseek_integration_test.rs\`` (`tasks-provider-expansion.md:227`) — **superseded by shipped code**. DeepSeek live-API coverage ships inside `llm_live_api_tests.rs` (`require_api_key("DEEPSEEK_API_KEY", ...)` at lines 282,313,363,390) rather than in a dedicated per-provider file |
| | | **Nested outstanding item:** `- [ ] 7.3 Write integration test: \`test_deepseek_live_completion()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:228`) — **present, unproven**. `llm_live_api_tests.rs` has DeepSeek completion tests gated by the `live-api-tests` feature rather than `#[ignore]`, functionally equivalent (neither runs by default), but `require_api_key`'s panic-on-missing-key means the test fails loudly rather than skipping if run without a key — the exact VERIFY-06 gap |
| | | **Nested outstanding item:** `- [ ] 7.4 Write integration test: \`test_deepseek_live_streaming()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:229`) — **present, unproven**, same reasoning as 7.3 |
| | | **Nested outstanding item:** `- [ ] 7.5 Write integration test: \`test_deepseek_model_validation()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:230`) — **present, unproven**, same reasoning as 7.3 |
| | | **Nested outstanding item:** `- [ ] 7.6 Create \`tests/integration/llm/anthropic_integration_test.rs\`` (`tasks-provider-expansion.md:231`) — **superseded by shipped code**, same reasoning as 7.2 — Anthropic live-API coverage ships inside the single flat file instead |
| | | **Nested outstanding item:** `- [ ] 7.7 Write integration test: \`test_anthropic_live_completion()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:232`) — **present, unproven**, same reasoning as 7.3 (Anthropic case, `llm_live_api_tests.rs:424,456,514,542`) |
| | | **Nested outstanding item:** `- [ ] 7.8 Write integration test: \`test_anthropic_live_streaming()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:233`) — **present, unproven**, same reasoning |
| | | **Nested outstanding item:** `- [ ] 7.9 Write integration test: \`test_anthropic_model_validation()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:234`) — **present, unproven**, same reasoning |
| | | **Nested outstanding item:** `- [ ] 7.10 Create \`tests/integration/llm/provider_switching_test.rs\`` (`tasks-provider-expansion.md:235`) — **genuinely outstanding**. No dedicated provider-switching integration test file exists anywhere in the tree under any name; `test_factory_creation`/`test_unknown_provider_returns_error` (`provider_factory.rs:163,169`) exercise the factory in isolation but not a runtime provider-switch scenario |
| | | **Nested outstanding item:** `- [ ] 7.11 Write integration test: \`test_switch_providers_via_config()\` with mocks` (`tasks-provider-expansion.md:236`) — **genuinely outstanding**, no matching test found anywhere in the tree |
| | | **Nested outstanding item:** `- [ ] 7.12 Write integration test: \`test_multiple_providers_simultaneously()\`` (`tasks-provider-expansion.md:237`) — **genuinely outstanding**, no matching test found |
| | | **Nested outstanding item:** `- [ ] 7.13 Write integration test: \`test_provider_capabilities_detection()\`` (`tasks-provider-expansion.md:238`) — **present, unproven** as a dedicated integration test by this name, though the underlying capability is unit-tested per-adapter (`test_get_capabilities`, `test_deepseek_provider_capabilities`, `test_anthropic_provider_capabilities` cited above) |
| | | **Nested outstanding item:** `- [ ] 7.14 Add CI configuration notes for optional live API tests (REQ-26)` (`tasks-provider-expansion.md:239`) — **genuinely outstanding**. `grep -n "live-api-tests" .github/workflows/*.yml` returns zero matches; the feature flag exists in `Cargo.toml:265` but no CI job references it |
| | | **Nested outstanding item:** `- [ ] 7.15 Run \`cargo test --test deepseek_integration_test\` (without --ignored) to verify non-live tests` (`tasks-provider-expansion.md:240`) — **superseded by shipped code**. No target named `deepseek_integration_test` exists (per 7.2/7.6 above); the equivalent command against the shipped structure is `cargo test --test llm_live_api_tests --features live-api-tests` |
| | | **Nested outstanding item:** `- [ ] 7.16 Optionally run \`cargo test --ignored\` with API keys set to test live APIs` (`tasks-provider-expansion.md:241`) — **present, unproven**. Running this command requires live provider API keys not available in this sandbox; not executed as part of this ledger entry |
| | | **Nested outstanding item:** `- [ ] 7.17 Fix any failing integration tests` (`tasks-provider-expansion.md:242`) — **present, unproven**, contingent on 7.16 actually being run |
| | | **Nested outstanding item:** `- [ ] 7.18 Document how to run integration tests in README or test files` (`tasks-provider-expansion.md:243`) — **satisfied**. `llm_live_api_tests.rs:10-27` carries a full "## Running Tests" doc comment with both `.env`-file and exported-variable invocation methods, provider-scoped run commands, and a cost warning — this documentation already exists, just inline in the test file rather than in the README |

### Epic 7 — Citadel State Persistence

No open task items (169/169 complete per `intel/task-completion-state.md` and confirmed by
`grep -cE '^\s*- \[ \]' tasks-citadel-state-persistence.md` returning 0) — every row below carries
no nested block.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-citadel-paladin-state-serialization | satisfied | `PaladinState` struct at `crates/paladin-core/src/platform/container/citadel.rs:128`; exercised by `test_paladin_state_serialization_roundtrip` (`citadel.rs:412`) |
| REQ-citadel-autosave | satisfied | `PaladinBuilder::enable_autosave()` at `src/application/services/paladin/paladin_builder.rs:1000`; `CitadelConfig::autosave_enabled` at `src/config/citadel.rs:14`; exercised by `test_builder_enable_autosave` (`paladin_builder.rs:1458`) and demonstrated end-to-end by `examples/citadel_autosave.rs` |
| REQ-citadel-paladin-restore | satisfied | `PaladinBuilder::restore_from()` at `paladin_builder.rs:1061`, returning `Result<Self, PaladinError>` (fallible, matching the PRD); exercised by `test_builder_restore_from_state_not_found` (`paladin_builder.rs:1685`) and demonstrated by `examples/citadel_restore.rs` |
| REQ-citadel-battalion-state-serialization | satisfied | `BattalionState` struct at `citadel.rs:227`; exercised by `test_battalion_state_serialization_roundtrip` (`citadel.rs:453`) |
| REQ-citadel-battalion-checkpoint-restore | satisfied | `CheckpointData` struct at `citadel.rs:304`, `mark_completed`/`mark_failed` at `:327,334`; exercised by `test_checkpoint_mark_completed`/`test_checkpoint_mark_failed` (`citadel.rs:539,553`) and demonstrated end-to-end by `examples/battalion_checkpoint_recovery.rs` |
| REQ-citadel-port | satisfied | `CitadelPort` trait at `crates/paladin-ports/src/output/citadel_port.rs:567`; exercised via a mock implementation by `test_mock_citadel_implements_trait` (`citadel_port.rs:624`) and object-safety-checked by `test_trait_is_object_safe` (`:658`) |
| REQ-citadel-errors | satisfied | `CitadelError` enum at `crates/paladin-core/src/platform/container/citadel_error.rs:25`; exercised by `test_state_not_found_error` (`:99`) and `test_incompatible_version_error` (`:116`) |
| REQ-citadel-builder-integration | satisfied | `PaladinBuilder::with_citadel()` at `paladin_builder.rs:977`; exercised by `test_builder_with_citadel` (`paladin_builder.rs:1448`) |
| REQ-citadel-state-directory | satisfied | `CitadelConfig::state_dir` at `src/config/citadel.rs:12`, default `"./paladin-states"` (`:25`); `FileCitadel` creates the directory on construction — exercised by `test_file_citadel_creates_directory` (`crates/paladin-memory/src/citadel/file_citadel.rs:379`) and `test_file_citadel_rejects_file_as_directory` (`:391`) |
| REQ-citadel-logging-docs | satisfied | `log::{info, warn}` calls in `file_citadel.rs:29` and call sites throughout; Citadel is documented across multiple mdbook pages rather than one dedicated file — `docs/src/architecture/domain-model.md`, `docs/src/getting-started/configuration.md`, `docs/src/architecture/overview.md`, `docs/src/api-reference/stable-api.md` all reference it, the same multi-page-relocation pattern already recorded elsewhere in this ledger for other Milestone-1 docs deliverables (no single `docs/CITADEL.md` exists, but the content is present, not missing) |

## Epic 7 — Citadel State Persistence

*(Filled by a later plan in this phase.)*

## Epic 8 — Herald Output Formatting

*(Filled by a later plan in this phase.)*

## Epic 9 — Armory CLI

*(Filled by a later plan in this phase.)*

## Epic 10 — Validation and Documentation

*(`REQ-*` rows filled by a later plan in this phase. The RECON-08 Task 7.0 dispute is already
resolved in the `## Epic 10 Task 7.0 — dispute resolution (RECON-08)` section above — verdict:
satisfied, the validation report is wrong — and the later plan uses that verdict rather than
re-opening it.)*
