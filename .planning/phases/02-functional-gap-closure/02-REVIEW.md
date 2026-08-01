---
phase: 02-functional-gap-closure
reviewed: 2026-08-01T00:00:00Z
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
  critical: 2
  warning: 6
  info: 2
  total: 10
status: issues_found
---

# Phase 02: Code Review Report

**Reviewed:** 2026-08-01T00:00:00Z
**Depth:** standard
**Files Reviewed:** 39
**Status:** issues_found

## Summary

`diff_base` (`6dbcbf4c9d1cd9b1e6b0bd6e9c76ce1e3bcbf000`) does not resolve in this repository, so all 39 files listed in scope were reviewed as they currently stand rather than as a diff.

The bulk of the reviewed code (Formation/Grove battalion services, the JSON/Markdown Heralds, the Citadel state-persistence domain and file adapter, the LLM port contract, and the Planning/PromptGeneration/Temperature application services) is solid: consistent hexagonal boundaries, `Result`-based error handling, no `unsafe`, and strong test coverage including several deliberately adversarial "litmus test" assertions (e.g. `TableHerald`'s distinct-metrics fixtures, the Anthropic adapter's captured-response fixtures for thinking-block deserialization).

Two Critical issues were found, both effectively untested gaps in otherwise well-tested modules:

1. The **OpenAI adapter drops the actual user message** for `PromptType::User` prompts — it reads the wrong field (`context` instead of `query`), which is empty in every call site reviewed. Since `PromptType::User` is the prompt shape used by the core `PaladinExecutionService` loop, `PlanningService`, `PromptGenerationService`, `TemperatureService`, and `GroveExecutionService`'s LLM-based routing, this silently breaks the primary happy path whenever the OpenAI adapter is the configured `LlmPort`. The Anthropic and DeepSeek adapters do not have this bug.
2. **`TableHerald::truncate_text` can panic** on multi-byte UTF-8 strings longer than the configured column width, because it slices by raw byte index rather than a char-boundary-safe index — a real crash risk for any Paladin name or output containing non-ASCII text, and inconsistent with this project's "no panics in library code" rule (see `.github/instructions/rust.instructions.md`).

Several Warning-level correctness/maintainability issues were also found (a fragile name-matching heuristic in `TableHerald`, a doc/implementation mismatch in stop-word detection, dead duplicated retry logic, a misleading mock doc comment, silent metadata-read failures in `FileCitadel::list_saved`, and a misleading log message in `FormationExecutionService`).

## Critical Issues

### CR-01: OpenAI adapter sends empty content for `PromptType::User` prompts

**File:** `crates/paladin-llm/src/openai/adapter.rs:223-228`

**Issue:** `OpenAIAdapter::convert_to_messages` builds the outgoing `"user"` message body from `user_prompt.context` instead of `user_prompt.query`:

```rust
PromptType::User(user_prompt) => {
    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content: user_prompt.context.clone().unwrap_or_default(),
    });
}
```

`UserPrompt` (`crates/paladin-core/src/platform/container/prompt.rs:182-185`) is `{ pub query: String, pub context: Option<String> }` — `query` is the actual message text; `context` is optional supplementary context that is `None` in every call site reviewed in this phase:

- `PaladinExecutionService::execute_with_retry_and_temperature` (`src/application/services/paladin/paladin_execution_service.rs:1561-1564`) — the **core execution loop for every Paladin call**.
- `PaladinExecutionService::execute_stream` (same file, `:1859-1862`).
- `PlanningService::create_plan` / `execute_subtask` (`src/application/services/paladin/planning_service.rs:122-125`, `:531-534`).
- `PromptGenerationService::generate_prompt` (`src/application/services/paladin/prompt_generation_service.rs:129-132`).
- `TemperatureService::detect_task_type_with_llm` (`src/application/services/paladin/temperature_service.rs:134-137`).
- `GroveExecutionService::route_by_llm` (`crates/paladin-battalion/src/grove_service.rs:526-529`).

Every one of these constructs `UserPrompt { query: <the real text>, context: None }`. When the configured `LlmPort` is `OpenAIAdapter`, the resulting HTTP request to OpenAI's Chat Completions API carries an **empty `"content"` field** for the user message — the actual prompt is silently dropped. The Anthropic adapter (`crates/paladin-llm/src/anthropic/adapter.rs:164-168`, uses `user_prompt.query.clone()`) and DeepSeek adapter (`crates/paladin-llm/src/deepseek/adapter.rs:279-284`, same) do not have this bug — only OpenAI is affected, making it easy to miss when other providers are used in test/dev.

No test in `crates/paladin-llm/src/openai/adapter.rs`'s own `#[cfg(test)]` module, nor in `tests/integration/provider_switching_test.rs` (which specifically compares Mock vs. DeepSeek, both of which use `.query` correctly), exercises `convert_to_messages` against a `PromptType::User` prompt and asserts on the resulting message content — so this gap is currently invisible to CI.

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
Add a unit test asserting `convert_to_messages` on a `PromptType::User(UserPrompt { query: "…", context: None })` produces a message whose `content` equals the query text.

### CR-02: `TableHerald::truncate_text` can panic on multi-byte UTF-8 input

**File:** `crates/paladin-herald/src/table_herald.rs:93-100`

**Issue:**
```rust
fn truncate_text(&self, text: &str) -> String {
    if text.len() <= self.config.max_column_width {
        text.to_string()
    } else {
        format!("{}...", &text[..self.config.max_column_width - 3])
    }
}
```
`text.len()` is a **byte** length, and `&text[..N]` slices by **byte** index. For any `text` whose byte length exceeds `max_column_width` and that contains multi-byte UTF-8 characters (CJK text, emoji, accented characters, etc.), `max_column_width - 3` can land in the middle of a multi-byte character's encoding, which causes Rust to panic at runtime with `byte index N is not a char boundary`. `truncate_text` is called on both Paladin names and (in `format_paladin_result`) output text — user- and LLM-controlled data — so this is directly reachable from normal operation, not just adversarial input.

The existing multi-byte test, `test_table_herald_renders_multibyte_paladin_name` (same file, `:474-486`), uses `"斥候レビュアー"` (21 bytes), which is well under the default `max_column_width` of 60 and therefore never exercises the truncation branch — the panic path is untested.

Panicking on ordinary user-controlled text violates this project's "avoid panics in library code — return `Result`" convention (`.github/instructions/rust.instructions.md`), and a panic inside a `Herald` formatter would abort whatever CLI/service invocation triggered formatting of a Battalion or Paladin result.

**Fix:** truncate on a char boundary, e.g.:
```rust
fn truncate_text(&self, text: &str) -> String {
    if text.chars().count() * 1 <= self.config.max_column_width && text.len() <= self.config.max_column_width {
        // fast path unaffected
    }
    let budget = self.config.max_column_width.saturating_sub(3);
    if text.len() <= self.config.max_column_width {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(budget).collect();
        format!("{truncated}...")
    }
}
```
(See `crates/paladin-llm/src/anthropic/adapter.rs`'s own `bounded_excerpt` helper for a char-boundary-safe truncation already implemented elsewhere in this codebase — it can be reused/adapted here directly.) Add a test with a multi-byte string long enough to force truncation (e.g. 30+ repeated 3- or 4-byte characters) and assert it does not panic and does not emit `\u{FFFD}`.

## Warnings

### WR-01: `TableHerald` matches Paladin names to rows by a coincidence-prone (time, tokens) tuple

**File:** `crates/paladin-herald/src/table_herald.rs:174-199`

**Issue:** Because `PaladinResult` carries no name field, `format_battalion_result` recovers each row's Paladin name by building a pool of `(name, execution_time_ms, token_count)` from `per_paladin_times`/`per_paladin_tokens` and matching each `paladin_result` against the pool by an **exact equality of `(execution_time_ms, token_count)`**:
```rust
let name = name_pool
    .iter()
    .position(|(_, time, tokens)| {
        *time == paladin_result.execution_time_ms && *tokens == paladin_result.token_count
    })
    .map(|pos| name_pool.remove(pos).0)
    .unwrap_or_else(|| format!("Paladin {}", idx + 1));
```
If two Paladins in the same Battalion happen to report identical `execution_time_ms` and `token_count` (plausible with deterministic mocks, trivial/no-op tool calls, or simple coincidence on fast executions), the wrong name is silently attached to a row — the table renders with no error but incorrect data. The project's own tests are aware of the collision risk (`battalion_result_with_paladins` deliberately generates "distinct, non-round" values to dodge it) but no test exercises the collision case itself.

**Fix:** Thread the Paladin name through explicitly rather than reconstructing identity from incidental metric values — e.g. have `BattalionResult`/`PaladinResult` carry the originating Paladin name directly (Formation/Phalanx already have it available when building `per_paladin_times`), or iterate `per_paladin_times`/`per_paladin_tokens` themselves as the source of row identity instead of `paladin_results`.

### WR-02: `check_stop_words` doc says "exact word match" but implementation is substring `.contains()`

**File:** `src/application/services/paladin/paladin_execution_service.rs:1184-1205`

**Issue:** The doc comment states: "Performs case-insensitive exact word matching." The implementation:
```rust
if output_lower.contains(&stop_word_lower) {
    return Some(stop_word.clone());
}
```
is a substring containment check, not word-boundary matching. A configured stop word such as `"no"` would match inside `"know"`, `"not"`, `"nose"`, etc., causing `PaladinExecutionService::execute_internal` to abort the loop early with `PaladinError::StopWordDetected` on legitimate output that merely contains the stop word as a substring of another word.

**Fix:** Either implement true word-boundary matching (split output into tokens and compare, or use a regex with `\b` word boundaries), or update the doc comment to accurately describe substring matching so callers configure stop words accordingly. Given the current behavior can prematurely abort valid executions, the safer fix is the former.

### WR-03: Dead, duplicated retry/circuit-breaker logic

**File:** `src/application/services/paladin/paladin_execution_service.rs:1663-1771`

**Issue:** `execute_with_retry` (~110 lines, `#[allow(dead_code)]`) is a near-verbatim duplicate of `execute_with_retry_and_temperature` (the one actually used by `execute_internal`), differing only in that it always uses `paladin.node.temperature` instead of a caller-supplied temperature. The comment says it is "retained pending integration into the execution loop," but it is currently unreachable dead code that must be kept in sync by hand with its live twin (retry backoff formula, circuit-breaker wiring, error mapping) — a maintenance hazard where a bug fix applied to one copy is easily missed in the other.

**Fix:** Delete `execute_with_retry` if it is genuinely superseded, or have `execute_with_retry_and_temperature` delegate to a single shared implementation parameterized by temperature, with `execute_with_retry` calling it with `paladin.node.temperature`.

### WR-04: `MultiStepMockLlmPort` doc comment claims panic-on-overcall; implementation does not panic

**File:** `crates/paladin-llm/src/mock.rs:282-286, 327-331`

**Issue:** The doc comment reads: "this adapter returns each response exactly once and then panics if called more times than there are responses." The actual `generate()` implementation:
```rust
let content = self
    .responses
    .get(index)
    .cloned()
    .unwrap_or_else(|| format!("Mock step {} response", index));
```
falls back to a synthesized placeholder string instead of panicking when the queue is exhausted. Tests that rely on the documented panic to catch an unexpected extra LLM call (a common assertion pattern for "the code under test must not call the LLM more than N times") will instead silently receive a nonsense response and may pass when they should fail.

**Fix:** Either make the implementation panic (or return an `Err`) on overcall to match the documented contract, or correct the doc comment to describe the actual cycling/fallback behavior.

### WR-05: `FileCitadel::list_saved` silently drops entries with unreadable metadata

**File:** `crates/paladin-memory/src/citadel/file_citadel.rs:319-335`

**Issue:**
```rust
if let Ok(metadata) = fs::metadata(&path).await
    && let Ok(created) = metadata.created()
    && let Ok(modified) = metadata.modified()
{
    summaries.push(StateSummary { .. });
}
```
If `fs::metadata`, `.created()`, or `.modified()` fails for a given state file (e.g. a filesystem/platform without birthtime support, a permissions issue, or a race with concurrent deletion), the entry is **silently omitted** from the returned list — no `warn!`/`error!` log, no indication in the result that a file was skipped. An operator listing saved states would have no way to know the count is incomplete.

**Fix:** Log a `warn!` with the path and error when metadata cannot be read, so silent under-reporting is at least diagnosable.

### WR-06: Misleading "ContinueOnError" log message when actual strategy is RetryThenContinue

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
Both strategies share this arm, but the log message hardcodes `"ContinueOnError"` even when the actually-configured (and already-exhausted-its-retries) strategy is `RetryThenContinue`. Operators reading logs to diagnose a Formation failure will see an inaccurate strategy name.

**Fix:** Interpolate the actual `formation.config.error_strategy` (or a `{:?}`-formatted variant) into the message instead of a hardcoded string.

## Info

### IN-01: Stale doc comment on `GroveExecutionService::execute_agent`

**File:** `crates/paladin-battalion/src/grove_service.rs:693-706`

**Issue:** The doc comment describes `agent_id` as an argument "matches index in paladins vec, e.g., 'agent_0'" resolved against a `paladins: &[Paladin]` parameter, but the actual method signature takes a single already-resolved `paladin: &Paladin` (looked up via `PaladinRegistry` by the caller in `execute()`). The doc no longer matches the implementation and could mislead a future maintainer about how routing decisions map to execution.

**Fix:** Update the doc comment to describe the current signature and registry-based resolution flow.

### IN-02: Hardcoded `model: "gpt-4"` with unresolved TODO in Grove's LLM routing

**File:** `crates/paladin-battalion/src/grove_service.rs:537`

**Issue:** `route_by_llm` builds its `LlmRequest` with a hardcoded model string and an explicit `// TODO: Make configurable` left in shipped code:
```rust
model: "gpt-4".to_string(), // TODO: Make configurable
```
This silently ignores whatever model the caller/Grove configuration intends to use for routing decisions, and ties routing quality/cost to a magic string that can drift from what's actually available/desired.

**Fix:** Thread a configurable model identifier through `Grove`'s config (mirroring `min_confidence`/`routing_fallback`) instead of hardcoding it, or at minimum track this as a follow-up ticket rather than an inline TODO.

---

_Reviewed: 2026-08-01T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
