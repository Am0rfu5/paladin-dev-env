---
phase: 16-documentation-currency-the-architecture-gap
reviewed: 2026-08-24T16:08:30Z
depth: standard
files_reviewed: 57
files_reviewed_list:
  - scripts/check-public-api-examples.sh
  - crates/paladin-web/src/agent_auth.rs
  - crates/paladin-web/src/agent_controller.rs
  - crates/paladin-web/src/agent_registry.rs
  - crates/paladin-web/src/app.rs
  - crates/paladin-web/src/delivery_controller.rs
  - crates/paladin-web/src/openapi.rs
  - crates/paladin-herald/src/lib.rs
  - crates/paladin-battalion/src/in_memory_registry.rs
  - crates/paladin-battalion/src/campaign_service.rs
  - crates/paladin-battalion/src/conclave_execution_service.rs
  - crates/paladin-battalion/src/council_service.rs
  - crates/paladin-battalion/src/formation_service.rs
  - crates/paladin-battalion/src/grove_service.rs
  - crates/paladin-battalion/src/maneuver/service.rs
  - crates/paladin-battalion/src/phalanx_service.rs
  - crates/paladin-core/src/base/service/collection_versioning_service.rs
  - crates/paladin-core/src/base/service/field_version_service.rs
  - crates/paladin-core/src/base/service/message_service.rs
  - crates/paladin-core/src/base/service/node_version_service.rs
  - crates/paladin-core/src/platform/container/battalion/council.rs
  - crates/paladin-core/src/platform/container/battalion/grove.rs
  - crates/paladin-core/src/platform/container/log.rs
  - crates/paladin-core/src/platform/container/paladin_config.rs
  - crates/paladin-core/src/platform/container/task.rs
  - crates/paladin-llm/src/llm_analysis_service.rs
  - crates/paladin-memory/src/services/memory_extraction_service.rs
  - crates/paladin-memory/src/services/rag_retrieval_service.rs
  - crates/paladin-ports/src/input/content_input_port.rs
  - crates/paladin-ports/src/input/document_port.rs
  - crates/paladin-ports/src/input/ml_port.rs
  - crates/paladin-ports/src/output/auth_port.rs
  - crates/paladin-ports/src/output/file_storage_port.rs
  - crates/paladin-ports/src/output/log_port.rs
  - crates/paladin-ports/src/output/orchestrator_port.rs
  - crates/paladin-ports/src/output/paladin_executor_port.rs
  - crates/paladin-ports/src/output/queue_port.rs
  - crates/paladin-ports/src/output/scheduler_port.rs
  - crates/paladin-ports/src/output/streaming_executor_port.rs
  - crates/paladin-ports/src/output/user_repository_port.rs
  - crates/paladin-ports/src/output/vision_port.rs
  - crates/paladin-ports/src/output/workflow_repository_port.rs
  - src/application/cli/formatters/progress.rs
  - src/application/cli/interactive/prompts.rs
  - src/application/services/arsenal/arsenal_execution_service.rs
  - src/application/services/arsenal/arsenal_registry_service.rs
  - src/application/services/content/content_ingestion_service.rs
  - src/application/services/paladin/handoff_service.rs
  - src/application/services/paladin/paladin_builder.rs
  - src/application/services/paladin/paladin_execution_service.rs
  - src/application/services/paladin/temperature_service.rs
  - src/core/platform/manager/content_service.rs
  - src/core/platform/manager/event_manager.rs
  - src/core/platform/manager/user_service.rs
  - src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs
  - src/infrastructure/security/encryption.rs
  - src/infrastructure/web/agent_host.rs
findings:
  critical: 1
  warning: 3
  info: 0
  total: 4
status: issues_found
---

# Phase 16: Code Review Report

**Reviewed:** 2026-08-24T16:08:30Z
**Depth:** standard
**Files Reviewed:** 57
**Status:** issues_found

## Summary

Phase 16's own claim — that every `.rs` change in this list is doc-comment-only (added `#
Examples` blocks, de-linked intra-doc links, singular→plural heading normalization) — holds
almost everywhere. I diffed all 57 files against `8b463c35e9dcf64e0391284507b7bead4a3b5ac8`,
isolated every added/removed line that was *not* inside a `///`/`//!` doc comment, and found
exactly one exception: `crates/paladin-herald/src/lib.rs` flips a lint attribute
(`#![allow(missing_docs)]` → `#![warn(missing_docs)]`), which is compiler-facing, not a doc
comment (see WR-03). Everything else — including the ~30 new rustdoc `# Examples` blocks across
the port traits and services, and `crates/paladin-web/openapi.json`'s regenerated snapshot of one
doc string — is genuinely doc-only.

I compiled and ran every doctest in every touched crate
(`cargo test --doc -p paladin-ports -p paladin-ai-core -p paladin-battalion -p paladin-llm -p
paladin-memory -p paladin-web -p paladin-herald -p paladin-ai`): **330 doctests passed, 0
failed** (116+ compiled/run examples, the rest `ignored` where marked `ignore`/`no_run`
deliberately). No example uses `unwrap()`/`expect()`, no hardcoded credential-shaped literal
leaks into a doc block, and no example crosses the core→application→infrastructure dependency
direction (`agent_auth.rs`, `encryption.rs`, and `mcp_streamable_http_adapter.rs`'s
credential-handling doc text were checked line-by-line — no weakening of the
redact-before-truncate or no-secret-in-`Debug`/log rules from `security.instructions.md`).

The one real defect is in `scripts/check-public-api-examples.sh`, the new CI gate this phase
introduces. Its heading-detection regex is unanchored and matches the literal substring `#
Examples` anywhere in a doc comment's text — including inside prose that merely *mentions* the
phrase, not an actual Markdown heading. I reproduced this with a throwaway crate: a doc comment
that says `... see the "# Examples" text quoted from another module ...` and contains no heading
and no code example at all is classified `OK` by the script. Since this is exactly the
"gate that silently passes when it should fail" failure mode called out in the review brief,
it's filed as CR-01 / Critical.

## Critical Issues

### CR-01: `check-public-api-examples.sh` heading detector accepts prose that merely mentions "# Examples"

**File:** `scripts/check-public-api-examples.sh:92-104` (`heading_spelling()`)
**Issue:** `heading_spelling()` classifies a doc block by running `grep -qE '#{1,2}
Examples\b'` (and the singular variant) against the *entire* accumulated doc-comment text, with
no anchor requiring the match to be a `///`-prefixed heading line on its own. Any doc comment
whose prose happens to contain the four-character sequence `# Examples` — e.g. a sentence that
quotes or references the heading style used elsewhere — is classified `plural`/`OK`, identical
to a real `/// # Examples` heading followed by a working code block.

Reproduced directly against the script's own functions:
```
$ cat crates/faketest/src/lib.rs
/// A fake port for testing the gate script.
///
/// Note: this trait intentionally does not repeat itself; see the "# Examples" text
/// quoted from another module for style guidance, but no example is provided here.
pub trait FakePort: Send + Sync {
    fn noop(&self);
}

$ heading_spelling "$(own_doc_block crates/faketest/src/lib.rs 5)"
plural   # WRONG — there is no heading and no example in this file at all
```
A `pub trait *Port` with zero examples and zero `# Examples` heading passes the gate. This is
the exact "gate that silently passes when it should fail" failure mode the script exists to
prevent — for the very public-API surface (`*Builder`/`*Port`/`*Service`) D-05 targets.

**Fix:** Anchor the match to a doc-comment heading line specifically, not a text search:
```bash
heading_spelling() {
    local text="$1"
    if grep -qE '^[[:space:]]*///[[:space:]]*#{1,2}[[:space:]]+Examples[[:space:]]*$' <<< "${text}"; then
        printf 'plural'
    elif grep -qE '^[[:space:]]*///[[:space:]]*#{1,2}[[:space:]]+Example[[:space:]]*$' <<< "${text}"; then
        printf 'singular'
    else
        printf ''
    fi
}
```
(Adjust the `///` prefix requirement for `module_doc_block`'s `//!` case too — currently both
callers share this one function.) After the fix, re-run `--list` and confirm the total OK count
doesn't regress against real, correctly-headed items (verified locally: the workspace's real 76
entry points all still classify `OK` under the anchored pattern; only the synthetic prose-only
case above flips from `OK` to correctly `MISSING`).

## Warnings

### WR-01: `is_unpublished_crate()` never checks the workspace-root `Cargo.toml`, so exclusion is silently inert for the whole `src/**` tree

**File:** `scripts/check-public-api-examples.sh:39-53`
**Issue:** The walk-up loop `while [[ "${dir}" != "." && "${dir}" != "/" ]]` stops *before*
ever testing `dir == "."`, so for any file under `src/**` (the root `paladin-ai` crate, whose
manifest is exactly `./Cargo.toml`), the loop exits without checking the one Cargo.toml that
actually governs those files. Confirmed directly:
```
$ is_unpublished_crate "src/application/services/paladin/paladin_builder.rs"
loop exited without checking dir=.
result=1   # "published" by default — the root manifest was never inspected
```
Today this has zero live impact because the root `Cargo.toml` doesn't set `publish = false`.
But the D-05 "unpublished crate" exclusion the comment claims to implement is non-functional for
the entire `src/**` tree — if the root crate is ever marked `publish = false` (plausible for an
application binary crate whose only job is to assemble published library crates), every `src/**`
entry point would silently stop being excluded and the gate would start reporting spurious
violations, with no comment anywhere explaining why the exclusion "isn't working."
**Fix:** Check the boundary directory before giving up:
```bash
    while true; do
        if [[ -f "${dir}/Cargo.toml" ]]; then
            grep -qE '^[[:space:]]*publish[[:space:]]*=[[:space:]]*false' "${dir}/Cargo.toml" && return 0
            return 1
        fi
        [[ "${dir}" == "." || "${dir}" == "/" ]] && return 1
        dir="$(dirname "${dir}")"
    done
```

### WR-02: The new gate only checks for a heading, not a working example — several D-05 items pass via `` ```ignore `` blocks that are never compiled

**Files:** e.g. `crates/paladin-battalion/src/campaign_service.rs:52`,
`crates/paladin-battalion/src/conclave_execution_service.rs:26`,
`crates/paladin-battalion/src/council_service.rs:44`,
`crates/paladin-battalion/src/formation_service.rs:28`,
`crates/paladin-battalion/src/grove_service.rs:89`,
`crates/paladin-battalion/src/phalanx_service.rs:35`,
`crates/paladin-core/src/platform/container/battalion/council.rs:275`,
`crates/paladin-core/src/platform/container/battalion/grove.rs:298`
**Issue:** This phase's heading normalization (`# Example` → `# Examples`) touched these files,
but their fenced code blocks are marked `` ```ignore ``, meaning `cargo test --doc` never
compiles or runs them (confirmed: none of these appear in the "ok"/"compile ok" doctest output,
only the passing/ignored set as pre-existing `ignore`/`no_run` items). `--list`/gate mode reports
all of them `OK` purely because the heading text is present and correctly spelled — the script
gives no signal that the code beneath the heading is unverified and can silently rot (reference a
renamed type, a changed constructor signature, etc.) without ever failing CI. This isn't a
regression introduced by phase 16 (the `ignore` attribute predates it), but the new gate's
passing output creates a stronger impression of "documented and demonstrated" than is actually
true for these eight-plus items.
**Fix:** Not blocking, but worth tracking as follow-up: either convert these to compiled examples
(preferred, and the port-trait doctests added this phase show the pattern), or have `--list`
mode additionally flag entry points whose only code fence is `ignore`d so reviewers can
distinguish "verified" from "merely present."

### WR-03: `paladin-herald`'s `#![allow(missing_docs)]` → `#![warn(missing_docs)]` flip is a compiler-behavior change, not a doc comment, and falls outside this phase's stated scope

**File:** `crates/paladin-herald/src/lib.rs:14`
**Issue:** Every other change in this 57-file set is contained inside `///`/`//!` doc comments.
This one line is a crate-level lint attribute: it turns off suppression of the `missing_docs`
lint and turns the lint on as an active warning. That's the one line in the entire diff that
isn't a doc comment, and it silently changes future compiler behavior for this crate — any
future `pub` item added to `paladin-herald` without a doc comment will now warn, and (per this
project's `cargo clippy -- -D warnings` convention in CLAUDE.md) would fail CI once clippy is run
with `-D warnings` on this crate. Verified this doesn't currently break anything
(`cargo clippy -p paladin-herald --all-features -- -D warnings`, default features, and
`--no-default-features` all pass clean today), so there's no live breakage — but it's an
undisclosed scope expansion relative to the phase's own "no signature, method body, or
visibility touched" claim, and the next contributor who adds an undocumented public item to this
crate will hit a lint failure with no changelog entry explaining why.
**Fix:** No code change needed (current state is clean); either call this line out explicitly in
the phase's summary/changelog as an intentional, verified lint-tightening (not "doc-only"), or
revert it to `#![allow(missing_docs)]` and land it as its own tracked change if it wasn't meant
to ship silently inside a documentation-currency phase.

---

_Reviewed: 2026-08-24T16:08:30Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
