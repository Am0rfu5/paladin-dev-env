---
phase: 02-functional-gap-closure
reviewed: 2026-08-01T12:00:00Z
depth: standard
files_reviewed: 39
files_reviewed_list:
  - crates/paladin-battalion/src/formation_service.rs
  - crates/paladin-battalion/src/grove_service.rs
  - crates/paladin-core/src/platform/container/battalion/formation.rs
  - crates/paladin-core/src/platform/container/battalion/mod.rs
  - crates/paladin-core/src/platform/container/citadel.rs
  - crates/paladin-herald/src/json_herald.rs
  - crates/paladin-herald/src/markdown_herald.rs
  - crates/paladin-herald/src/table_herald.rs
  - crates/paladin-llm/src/anthropic/adapter.rs
  - crates/paladin-llm/src/deepseek/adapter.rs
  - crates/paladin-llm/src/lib.rs
  - crates/paladin-llm/src/mock.rs
  - crates/paladin-llm/src/openai/adapter.rs
  - crates/paladin-memory/src/citadel/file_citadel.rs
  - crates/paladin-ports/src/output/citadel_port.rs
  - crates/paladin-ports/src/output/llm_port.rs
  - src/application/services/paladin/paladin_builder.rs
  - src/application/services/paladin/paladin_execution_service.rs
  - src/application/services/paladin/planning_service.rs
  - src/application/services/paladin/prompt_generation_service.rs
  - src/application/services/paladin/temperature_service.rs
  - tests/cli/error_handling_test.rs
  - tests/cli/formation_execution_test.rs
  - tests/cli/helpers.rs
  - tests/cli/mod.rs
  - tests/cli/paladin_execution_test.rs
  - tests/cli/phalanx_execution_test.rs
  - tests/cli/tool_integration_test.rs
  - tests/helpers/mock_llm_adapter.rs
  - tests/integration/autonomous_planning_test.rs
  - tests/integration/battalion_herald_end_to_end_test.rs
  - tests/integration/citadel_integration_test.rs
  - tests/integration/mod.rs
  - tests/integration/provider_switching_test.rs
  - tests/lib.rs
  - tests/unit/battalion/formation_tests.rs
  - tests/unit/llm/anthropic_adapter_test.rs
  - tests/unit/llm/deepseek_adapter_test.rs
  - tests/unit/llm/mod.rs
  - tests/unit/llm/provider_factory_test.rs
  - tests/unit/mod.rs
findings:
  critical: 1
  warning: 7
  info: 2
  total: 10
status: issues_found
---

# Phase 02: Code Review Report

**Reviewed:** 2026-08-01T12:00:00Z
**Depth:** standard
**Files Reviewed:** 39
**Status:** issues_found

## Summary

This is a re-review of the same 39-file scope as the prior `02-REVIEW.md` (dated
2026-08-01T00:00:00Z), after gap-closure plan `02-10` landed. Per the workflow's
instructions, the prior review was read first; this report carries forward every prior
finding that is still open, marks the one finding that plan `02-10` demonstrably closed
as resolved, and adds one new finding surfaced by reading the Citadel port contract
alongside its file-backed adapter.

**Resolved since the prior review:**

- **CR-02 (`TableHerald::truncate_text` panic) is fixed.** `truncate_text` now counts and
  slices by `chars()` instead of byte indices, with an explicit `max_column_width < 3`
  branch to avoid the `usize` underflow that previously made widths 0–2 panic. The fix is
  backed by five new tests, including a table-driven sweep across 2-byte/3-byte/4-byte/mixed
  inputs and widths 0–24 plus 60, an overlong-multibyte-name test for
  `format_battalion_result`, and an overlong-multibyte-message test for `format_error`
  (which routes through the same helper via `PaladinError`'s `Display`). Verified by
  reading `crates/paladin-herald/src/table_herald.rs:93-123` and its test module.

**Still open (unchanged since the prior review):**

The prior review's remaining Critical finding — **CR-01, the OpenAI adapter dropping the
real user message** — is untouched: `crates/paladin-llm/src/openai/adapter.rs:223-227`
still reads `user_prompt.context` instead of `user_prompt.query` for `PromptType::User`,
identical to what was reported before, and still has no covering test (its own
`#[cfg(test)]` module and `tests/integration/provider_switching_test.rs` still only
exercise Mock/DeepSeek/Anthropic paths against `.query`, never OpenAI's
`convert_to_messages`). All six Warning/Info findings from the prior review (`WR-01`
through `WR-06`, `IN-01`, `IN-02`) were independently re-verified against their cited line
ranges in the current files and are byte-for-byte unchanged — none of the gap-closure work
in this phase touched them. They are reproduced below with their original numbering
preserved, plus one new Warning (`WR-07`) found during this pass.

## Critical Issues

### CR-01: OpenAI adapter sends empty content for `PromptType::User` prompts

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-llm/src/openai/adapter.rs:223-228`

**Issue:** `OpenAIAdapter::convert_to_messages` builds the outgoing `"user"` message body
from `user_prompt.context` instead of `user_prompt.query`:

```rust
PromptType::User(user_prompt) => {
    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content: user_prompt.context.clone().unwrap_or_default(),
    });
}
```

`UserPrompt` is `{ pub query: String, pub context: Option<String> }` — `query` is the
actual message text; `context` is optional supplementary context that is `None` in every
call site reviewed in this phase (`PaladinExecutionService::execute_with_retry_and_temperature`,
`PaladinExecutionService::execute_stream`, `PlanningService::create_plan`/`execute_subtask`,
`PromptGenerationService::generate_prompt`, `TemperatureService::detect_task_type_with_llm`,
`GroveExecutionService::route_by_llm` — all verified again in this pass, all still construct
`UserPrompt { query: <text>, context: None }`). When the configured `LlmPort` is
`OpenAIAdapter`, the resulting HTTP request to OpenAI's Chat Completions API carries an
**empty `"content"` field** for the user message — the actual prompt is silently dropped.
The Anthropic adapter (`crates/paladin-llm/src/anthropic/adapter.rs:164-168`, uses
`user_prompt.query.clone()`) and DeepSeek adapter
(`crates/paladin-llm/src/deepseek/adapter.rs:279-284`, same) do not have this bug — only
OpenAI is affected.

Re-verified in this pass: `crates/paladin-llm/src/openai/adapter.rs`'s own `#[cfg(test)]`
module (lines 660-734) still has no test that exercises `convert_to_messages` against a
`PromptType::User` prompt, and `tests/integration/provider_switching_test.rs` still only
switches between `MockLlmAdapter` and `DeepSeekAdapter` — both of which use `.query`
correctly — so this gap remains invisible to CI.

**Fix:**
```rust
PromptType::User(user_prompt) => {
    let mut content = user_prompt.query.clone();
    if let Some(context) = &user_prompt.context {
        content.push_str("\n\n");
        content.push_str(context);
    }
    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content,
    });
}
```
Add a unit test asserting `convert_to_messages` on a
`PromptType::User(UserPrompt { query: "…", context: None })` produces a message whose
`content` equals the query text.

## Warnings

### WR-01: `TableHerald` matches Paladin names to rows by a coincidence-prone (time, tokens) tuple

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-herald/src/table_herald.rs:203-222`

**Issue:** `format_battalion_result` recovers each row's Paladin name by building a pool of
`(name, execution_time_ms, token_count)` from `per_paladin_times`/`per_paladin_tokens` and
matching each `paladin_result` against the pool by **exact equality of
`(execution_time_ms, token_count)`**:
```rust
let name = name_pool
    .iter()
    .position(|(_, time, tokens)| {
        *time == paladin_result.execution_time_ms
            && *tokens == paladin_result.token_count
    })
    .map(|pos| name_pool.remove(pos).0)
    .unwrap_or_else(|| format!("Paladin {}", idx + 1));
```
If two Paladins in the same Battalion happen to report identical `execution_time_ms` and
`token_count` (plausible with deterministic mocks, trivial/no-op tool calls, or simple
coincidence on fast executions), the wrong name is silently attached to a row — the table
renders with no error but incorrect data. The comment added alongside this code (lines
197-202) documents the mechanism but does not change the collision risk, and no test
exercises the collision case itself.

**Fix:** Thread the Paladin name through explicitly rather than reconstructing identity
from incidental metric values — e.g. have `BattalionResult`/`PaladinResult` carry the
originating Paladin name directly (Formation/Phalanx already have it available when
building `per_paladin_times`), or iterate `per_paladin_times`/`per_paladin_tokens`
themselves as the source of row identity instead of `paladin_results`.

### WR-02: `check_stop_words` doc says "exact word match" but implementation is substring `.contains()`

**Status:** Still open — unchanged since the prior review.

**File:** `src/application/services/paladin/paladin_execution_service.rs:1184-1205`

**Issue:** The doc comment states: "Performs case-insensitive exact word matching." The
implementation:
```rust
if output_lower.contains(&stop_word_lower) {
    return Some(stop_word.clone());
}
```
is a substring containment check, not word-boundary matching. A configured stop word such
as `"no"` would match inside `"know"`, `"not"`, `"nose"`, etc., causing
`PaladinExecutionService::execute_internal` to abort the loop early with
`PaladinError::StopWordDetected` on legitimate output that merely contains the stop word as
a substring of another word.

**Fix:** Either implement true word-boundary matching (split output into tokens and
compare, or use a regex with `\b` word boundaries), or update the doc comment to accurately
describe substring matching so callers configure stop words accordingly. Given the current
behavior can prematurely abort valid executions, the safer fix is the former.

### WR-03: Dead, duplicated retry/circuit-breaker logic

**Status:** Still open — unchanged since the prior review.

**File:** `src/application/services/paladin/paladin_execution_service.rs:1663-1771`

**Issue:** `execute_with_retry` (~110 lines, `#[allow(dead_code)]`) is a near-verbatim
duplicate of `execute_with_retry_and_temperature` (the one actually used by
`execute_internal`), differing only in that it always uses `paladin.node.temperature`
instead of a caller-supplied temperature. The comment says it is "retained pending
integration into the execution loop," but it is currently unreachable dead code that must
be kept in sync by hand with its live twin (retry backoff formula, circuit-breaker wiring,
error mapping) — a maintenance hazard where a bug fix applied to one copy is easily missed
in the other.

**Fix:** Delete `execute_with_retry` if it is genuinely superseded, or have
`execute_with_retry_and_temperature` delegate to a single shared implementation
parameterized by temperature, with `execute_with_retry` calling it with
`paladin.node.temperature`.

### WR-04: `MultiStepMockLlmPort` doc comment claims panic-on-overcall; implementation does not panic

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-llm/src/mock.rs:282-286, 327-331`

**Issue:** The doc comment reads: "this adapter returns each response exactly once and
then panics if called more times than there are responses." The actual `generate()`
implementation:
```rust
let content = self
    .responses
    .get(index)
    .cloned()
    .unwrap_or_else(|| format!("Mock step {} response", index));
```
falls back to a synthesized placeholder string instead of panicking when the queue is
exhausted. Tests that rely on the documented panic to catch an unexpected extra LLM call (a
common assertion pattern for "the code under test must not call the LLM more than N times")
will instead silently receive a nonsense response and may pass when they should fail.

**Fix:** Either make the implementation panic (or return an `Err`) on overcall to match the
documented contract, or correct the doc comment to describe the actual cycling/fallback
behavior.

### WR-05: `FileCitadel::list_saved` silently drops entries with unreadable metadata

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-memory/src/citadel/file_citadel.rs:322-335`

**Issue:**
```rust
if let Ok(metadata) = fs::metadata(&path).await
    && let Ok(created) = metadata.created()
    && let Ok(modified) = metadata.modified()
{
    summaries.push(StateSummary { .. });
}
```
If `fs::metadata`, `.created()`, or `.modified()` fails for a given state file (e.g. a
filesystem/platform without birthtime support, a permissions issue, or a race with
concurrent deletion), the entry is **silently omitted** from the returned list — no
`warn!`/`error!` log, no indication in the result that a file was skipped. An operator
listing saved states would have no way to know the count is incomplete.

**Fix:** Log a `warn!` with the path and error when metadata cannot be read, so silent
under-reporting is at least diagnosable.

### WR-06: Misleading "ContinueOnError" log message when actual strategy is RetryThenContinue

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-battalion/src/formation_service.rs:250-263`

**Issue:**
```rust
ErrorStrategy::ContinueOnError | ErrorStrategy::RetryThenContinue => {
    warn!(
        "ContinueOnError: Paladin {} failed, continuing with empty output",
        index + 1
    );
    ...
}
```
Both strategies share this arm, but the log message hardcodes `"ContinueOnError"` even when
the actually-configured (and already-exhausted-its-retries) strategy is
`RetryThenContinue`. Operators reading logs to diagnose a Formation failure will see an
inaccurate strategy name.

**Fix:** Interpolate the actual `formation.config.error_strategy` (or a `{:?}`-formatted
variant) into the message instead of a hardcoded string.

### WR-07: `FileCitadel` writes are not atomic, contradicting the `CitadelPort` contract it implements — new finding

**Status:** New.

**File:** `crates/paladin-memory/src/citadel/file_citadel.rs:160-181, 225-246`

**Issue:** `CitadelPort`'s own trait documentation
(`crates/paladin-ports/src/output/citadel_port.rs:299-306`) states as an **implementation
requirement**: "Use atomic operations for save (no partial writes)," and its
"Atomicity Guarantees" section (lines 494-508) spells out the expected pattern explicitly:

```rust
// File-based: Write to temp file, then atomic rename
let temp_file = format!("{}.tmp", state_file);
fs::write(&temp_file, json)?;
fs::rename(&temp_file, &state_file)?;  // Atomic on POSIX
```

`FileCitadel::save_paladin` and `FileCitadel::save_battalion` do not follow this pattern —
both write directly to the final path with a single `fs::write(&path, json).await`:

```rust
async fn save_paladin(&self, state: &PaladinState) -> Result<(), CitadelError> {
    let path = self.paladin_path(state.paladin.uuid);
    let json = serde_json::to_string_pretty(state)?;
    fs::write(&path, json).await.map_err(|e| { .. })?;
    ...
}
```

`FileCitadel`'s own module doc (line 12) even claims "Atomic writes using tokio::fs async
operations," which is inaccurate: `tokio::fs::write` is not atomic with respect to process
crashes or power loss mid-write — it truncates-then-writes the target file in place. If the
process is killed while a save is in flight (a real scenario for a system explicitly
designed to persist state "due to system failures, restarts, or intentional shutdown," per
this same module's own doc comment), the state file on disk is left truncated or
partially-written. The next `load_paladin`/`load_battalion` call on that file then fails
with `CitadelError::corrupted` (via the `serde_json::from_str` error path at
`file_citadel.rs:204-210`/`269-275`) — the exact "Corrupted State" failure mode the port's
own doc calls out as something implementations should guard against via atomicity.

**Fix:** Follow the pattern the port trait's own documentation prescribes: write to a
sibling `.tmp` file, then `tokio::fs::rename` it over the final path (atomic on POSIX
filesystems, and the standard fix for this class of bug):
```rust
let tmp_path = path.with_extension("json.tmp");
fs::write(&tmp_path, json).await.map_err(|e| { .. })?;
fs::rename(&tmp_path, &path).await.map_err(|e| { .. })?;
```
Add a test that simulates an interrupted write (e.g. write a truncated/partial file
directly, then assert `load_paladin` returns a typed `CitadelError` rather than corrupting
silently) to lock in the desired failure mode either way.

## Info

### IN-01: Stale doc comment on `GroveExecutionService::execute_agent`

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-battalion/src/grove_service.rs:693-705`

**Issue:** The doc comment describes `agent_id` as an argument "matches index in paladins
vec, e.g., 'agent_0'" resolved against a `paladins: &[Paladin]` parameter, but the actual
method signature takes a single already-resolved `paladin: &Paladin` (looked up via
`PaladinRegistry` by the caller in `execute()`). The doc no longer matches the
implementation and could mislead a future maintainer about how routing decisions map to
execution.

**Fix:** Update the doc comment to describe the current signature and registry-based
resolution flow.

### IN-02: Hardcoded `model: "gpt-4"` with unresolved TODO in Grove's LLM routing

**Status:** Still open — unchanged since the prior review.

**File:** `crates/paladin-battalion/src/grove_service.rs:537`

**Issue:** `route_by_llm` builds its `LlmRequest` with a hardcoded model string and an
explicit `// TODO: Make configurable` left in shipped code:
```rust
model: "gpt-4".to_string(), // TODO: Make configurable
```
This silently ignores whatever model the caller/Grove configuration intends to use for
routing decisions, and ties routing quality/cost to a magic string that can drift from
what's actually available/desired.

**Fix:** Thread a configurable model identifier through `Grove`'s config (mirroring
`min_confidence`/`routing_fallback`) instead of hardcoding it, or at minimum track this as
a follow-up ticket rather than an inline TODO.

---

_Reviewed: 2026-08-01T12:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
