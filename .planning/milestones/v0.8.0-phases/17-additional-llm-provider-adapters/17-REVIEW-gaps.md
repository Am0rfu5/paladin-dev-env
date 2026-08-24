---
phase: 17-additional-llm-provider-adapters
reviewed: 2026-08-22T00:00:00Z
depth: standard
files_reviewed: 16
files_reviewed_list:
  - crates/paladin-llm/src/compat/engine.rs
  - crates/paladin-llm/src/compat/mod.rs
  - crates/paladin-llm/src/grok/adapter.rs
  - crates/paladin-llm/src/kimi/adapter.rs
  - crates/paladin-llm/src/qwen/adapter.rs
  - crates/paladin-llm/src/gemini/adapter.rs
  - crates/paladin-llm/src/ollama/adapter.rs
  - crates/paladin-llm/src/openai_compatible/adapter.rs
  - crates/paladin-llm/src/lib.rs
  - crates/paladin-llm/examples/live_vendor_smoke.rs
  - src/application/services/paladin/paladin_builder.rs
  - config.example.yml
  - .env.example
  - docs/src/getting-started/configuration.md
  - crates/paladin-llm/README.md
  - CHANGELOG.md
findings:
  critical: 1
  warning: 2
  info: 0
  total: 3
status: resolved
resolved_in: 9ce90b7
resolved_at: 2026-08-22
resolution: |
  All three findings independently verified against the source by the orchestrator,
  then fixed in commit 9ce90b7. CR-01 validated at the point of assignment (refuse per
  ADR-0004, never clamp) with two regression tests; WR-01 changed find('@') to rfind('@')
  per RFC 3986 with a multi-'@' regression test; WR-02 expanded Kimi's declaration to state
  all five fields explicitly, restoring the no-spread invariant. Full gate green after.
---

# Phase 17: Code Review Report (gap-closure run, since 81c4a68)

**Reviewed:** 2026-08-22
**Depth:** standard
**Files Reviewed:** 16
**Status:** issues_found

## Summary

This run's diff (plans 17-18 through 17-22, plus the two orchestrator doc commits) adds
`CompatRequestParameters` to the shared `CompatEngine`, adds `classify_fetch_failure` and
`base_url_without_userinfo` to `available_models()`'s diagnostic path, refreshes the
Grok/Kimi/Qwen/Gemini model and base-URL constants against live vendor measurements, and
narrows the ADR-0004 temperature gate in `PaladinBuilder` to fire only on an
explicitly-expressed temperature. Docs and `.env.example` were refreshed to match.

The mechanical parts hold up well: `classify_fetch_failure` is genuinely exhaustive over
`LlmError` (verified against the live enum — no `#[non_exhaustive]`, ten variants, ten
match arms, no wildcard), the D-13/D-14 fallback contract is unchanged and pinned by
tests, `build_request`'s five-parameter gating is data-driven with no provider-name or
base-URL branching in the shared engine, and every refreshed model ID / base URL in
`config.example.yml`, `.env.example`, the configuration guide and the README matches the
constant it mirrors.

Two problems were found. The more serious one is in `paladin_builder.rs`, not in the LLM
crate: the ADR-0004 temperature-validation narrowing this run introduces has a gap its own
doc comment claims does not exist — an LLM-auto-selected temperature can reach a
provider's wire request having never been range-checked at all. The second is a narrow
but real gap in the new credential-redaction helper `base_url_without_userinfo`: it strips
only up to the *first* `@` in the authority, not the last, so a malformed base URL whose
userinfo itself contains an unescaped `@` leaks part of the credential into the new
`warn!` log line instead of being fully redacted.

## Critical Issues

### CR-01: Auto-selected temperature bypasses ADR-0004 range validation, contradicting the method's own doc comment

**File:** `src/application/services/paladin/paladin_builder.rs:1303-1332` (auto-temperature
branch in `build()`), gated by `src/application/services/paladin/paladin_builder.rs:1137`
(the narrowed `validate()` gate)

**Issue:** This run's fix for G-17-4b narrows the ADR-0004 temperature-range check to fire
only `if self.manual_temperature_override` (line 1137), so a framework-fabricated default
temperature (`PaladinData::default()`'s `0.7`) that the caller never asked for is not
judged against a provider's narrow declared range (e.g. Kimi's measured `(1.0, 1.0)`).

The updated doc comment on `validate()` states the gate covers a temperature expressed
"via [`Self::temperature`], **or the auto-temperature branch in [`Self::build`]**"
(lines 1097-1099). That second clause is false as written. The auto-temperature branch
(`build()`, lines 1303-1332) calls `TemperatureService::calculate_optimal_temperature`
and, on success, does:

```rust
Ok(optimal_temp) => {
    log::info!(...);
    self.data.temperature = optimal_temp;   // line 1318 — no flag set
}
```

It never sets `self.manual_temperature_override = true`. `self.validate()` is called
later in the same function (line 1351), by which point `manual_temperature_override` is
still `false` for a Paladin built via `.auto_temperature(true)` with no explicit
`.temperature()` call — so the range check at line 1137 is skipped entirely for the
auto-selected value.

This is a real behaviour regression, not merely a doc error: before this run, `validate()`
unconditionally checked `self.data.temperature` (whatever value it held, auto-selected or
not) against the provider's declared range. `TemperatureService`'s built-in defaults
(`TemperatureConfig::default()`: 0.85 / 0.2 / 0.6) all fall within `[0.0, 1.0]`, but they
are not guaranteed to fall within an arbitrary provider's *narrower* declared range (Kimi's
own shipped `(1.0, 1.0)` is the concrete existing counter-example already in this codebase;
a custom `openai-compatible` deployment with `OPENAI_COMPATIBLE_TEMPERATURE_MIN/MAX` set
narrower still is another). An auto-selected value outside the provider's range now
reaches `CompatEngine::build_request` unchecked, producing an opaque wire-level `400` from
the vendor instead of the fast, named `ConfigurationError` ADR-0004 exists to provide.

No test in this diff exercises `auto_temperature_enabled` together with a narrow
provider-declared `temperature_range` — the two new tests added in this run
(`test_unexpressed_temperature_builds_against_a_degenerate_provider_range`,
`test_expressed_temperature_is_still_rejected_against_a_degenerate_provider_range`) both
use `manual_temperature_override` directly on hand-built `PaladinBuilder` structs; neither
drives the `auto_temperature(true)` code path end to end, so the gap was never exercised.

**Fix:** Set the override flag when the auto-temperature branch actually assigns a value,
so the value the service selected is treated as "expressed" for validation purposes —
consistent with what the doc comment already (incorrectly) claims:

```rust
Ok(optimal_temp) => {
    log::info!(
        "Auto-selected temperature {} for agent based on task type",
        optimal_temp
    );
    self.data.temperature = optimal_temp;
    self.manual_temperature_override = true; // this value must still be range-checked
}
```
Add a test that builds with `.auto_temperature(true)` against a mock LLM port declaring a
narrow `temperature_range` that does not contain any of `TemperatureConfig`'s three
defaults, and asserts `build()` returns `Err(ConfigurationError)` rather than `Ok`.

## Warnings

### WR-01: `base_url_without_userinfo` strips only the first `@`, not the last — a malformed URL leaks part of a credential into the new `warn!` log line

**File:** `crates/paladin-llm/src/compat/engine.rs:300-320`

**Issue:** `base_url_without_userinfo` is the redaction helper introduced in this run
specifically so a rejected-credential `warn!` in `available_models()` (line ~953) never
leaks a base URL's userinfo component. It locates the userinfo/host boundary with
`authority.find('@')` — the *first* `@` in the authority — and drops everything up to and
including that character.

Standard URL parsing (and the WHATWG URL spec) locates the userinfo/host boundary at the
*last* `@` in the authority, precisely because a userinfo component (rare, but the doc
comment on this function itself notes it exists to guard the case of "the kind of
operator-supplied string that could carry a credential this way") can itself contain an
unencoded `@`. When it does, `find('@')` under-strips, and the tail of the userinfo
(commonly the tail of the password) survives into the "redacted" output. Verified
directly against the shipped function:

```rust
base_url_without_userinfo("https://alice:p@ssw0rd@example.com/v1")
// => "https://ssw0rd@example.com/v1"   -- "ssw0rd" (part of the password) is NOT redacted
```

The function's own test suite (`base_url_without_userinfo_strips_a_credential_from_the_authority`
and three siblings, `engine.rs:1788-1814`) only exercises a single well-formed `@`; none
covers a userinfo containing more than one `@`, so this gap shipped untested even though
the priority review area for this run explicitly calls out "malformed/edge-case URLs" for
this exact function.

This is lower severity than it looks because no shipped preset's documented `base_url`
format carries userinfo at all (per this function's own doc comment) — it is a defensive
measure against a hypothetical operator misconfiguration, not a path any shipped adapter
exercises today. But the entire purpose of the function is credential redaction in a log
line, and it does not fully achieve that purpose on the exact class of malformed input its
own docs describe defending against.

**Fix:** Locate the boundary at the *last* `@` in the authority, not the first:

```rust
let Some(at_idx) = authority.rfind('@') else {
    return base_url.to_string();
};
```
Add a test with a userinfo containing an embedded `@` (e.g.
`"https://alice:p@ssw0rd@example.com/v1"`) asserting the output contains no fragment of
the password.

### WR-02: Kimi's `CompatRequestParameters` declaration uses struct-update syntax, undermining the type's own documented "no silent inheritance" guarantee

**File:** `crates/paladin-llm/src/kimi/adapter.rs:233-237`; contrast with
`crates/paladin-llm/src/compat/engine.rs:98-103`

**Issue:** `CompatRequestParameters`'s doc comment (`engine.rs:98-103`) states the type is
"[d]eliberately has **no** `Default` impl, matching the same posture on
[`CompatEngineConfig`] itself (**no** `Default`, **no struct-update syntax at any
construction site**): a new preset must be a compile error until its author states a
position for every field, one at a time." The stated goal is that adding a sixth optional
sampling parameter to this struct in the future must force every preset author to
explicitly decide whether their vendor carries it.

Kimi's declaration, added in this run, does not meet that bar:

```rust
request_parameters: CompatRequestParameters {
    temperature: false,
    top_p: false,
    ..CompatRequestParameters::all()
},
```

This states a position for two of five fields and silently inherits the remaining three
(`max_tokens`, `frequency_penalty`, `presence_penalty`) via struct-update syntax against
`CompatRequestParameters::all()`. Every other preset in this diff (Grok, at
`grok/adapter.rs:211-217`) states all five fields by name with no `..`; Qwen, Ollama and
`openai_compatible` use `CompatRequestParameters::all()` as a complete value, not a partial
override. Kimi is the one call site that mixes the two: if a sixth field is added to
`CompatRequestParameters` later, Grok's fully-explicit literal fails to compile (exactly
the intended forcing function), while Kimi's `..CompatRequestParameters::all()` silently
absorbs the new field as `true` (carried) with no compile error and no review signal —
precisely the "silent inheritance of 'everything is supported'" failure mode the same doc
comment names as "exactly how the shipped Grok preset came to send a parameter xAI
rejects."

**Fix:** Spell out all five fields explicitly on `KimiAdapter`'s declaration, matching
Grok's pattern, so the type's documented invariant actually holds at every preset that
declares a partial override:

```rust
request_parameters: CompatRequestParameters {
    temperature: false,
    top_p: false,
    max_tokens: true,
    frequency_penalty: true,
    presence_penalty: true,
},
```

---

_Reviewed: 2026-08-22_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
