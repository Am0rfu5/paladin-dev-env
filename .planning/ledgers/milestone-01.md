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

## Epic 4 — Battalion Orchestration

*(Filled by a later plan in this phase.)*

## Epic 5 — Commander Strategy Router

*(Filled by a later plan in this phase.)*

## Epic 6 — Provider Expansion

*(Filled by a later plan in this phase.)*

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
