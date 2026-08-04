---
phase: 02
slug: functional-gap-closure
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-08-01
---

# Phase 02 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

Register origin: authored at plan time. All 11 PLAN files (`02-01` … `02-11`) carried a
`<threat_model>` block; this document consolidates them and records the verification verdict.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| shell → planning record | Command stdout transcribed into a durable evidence artifact four requirements close on | Test output, provenance probes (commit SHA, toolchain versions) |
| planning record → ledger | Verdicts applied to `milestone-01.md` by a later plan without re-running commands | Ledger row verdicts, forward owners |
| caller → `PaladinBuilder::validate` | Untrusted caller-supplied `temperature: f32` crosses into config sent to a paid external provider | Numeric config value |
| adapter → `ProviderCapabilities` | An adapter's self-declared capability consumed by callers who branch on it | Capability booleans across a published contract |
| caller → `Formation::validate` | Untrusted caller-supplied Paladin collection crosses into a domain invariant gating execution | Collection cardinality |
| in-memory type → persisted checkpoint | `BattalionState.config` serialized to disk by file-backed Citadel at `schema_version: "1.0.0"` | Serde field names, order, types |
| Paladin / LLM output → Herald rendering | Paladin names and LLM-produced text cross into JSON, Markdown and terminal-table output an operator reads | Untrusted-length, multi-byte text |
| operator config → formatter arithmetic | `herald.table.max_column_width` passes `HeraldConfig::validate` (rejects only `0`) into `TableHeraldConfig` unfiltered | Unbounded `usize` |
| execution service → `BattalionResult` | Per-node timing, token and failure data crosses into a serializable type exported to telemetry files | Provider error bodies, token counts |
| mock HTTP server → LLM adapter | Reactivated tests feed adapter code untrusted-shaped responses (401, 429, malformed, streaming) | HTTP response bodies |
| test process env → code under test | Reactivated suites mutate process-wide environment shared across parallel test threads | Env vars (`unsafe` mutation) |
| plan SUMMARYs → ledger rows | Verdicts from seven sibling plans transcribed into a record four later phases inherit | Verdicts, citations, dates |
| scoped edit → 4,038-line multi-ledger doc | A single write has reach to destroy two as-shipped ledgers and 78 forward requirements | Requirement completion state |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-02-01 | Repudiation | `02-test-baseline.md` evidence provenance | high | mitigate | Five provenance probes pasted verbatim — `02-test-baseline.md:8-51`; commit `7e55655` resolves via `git cat-file -t` | closed |
| T-02-02 | Tampering | Elided test output hiding a failure | high | mitigate | All 35 `test result:` lines pasted — `02-test-baseline.md:53-190`; independently re-summed to 2790 passed / 0 failed / 126 ignored | closed |
| T-02-03 | Information Disclosure | Env values in committed test stdout | medium | mitigate | Credential-pattern grep over `02-test-baseline.md` → 0 matches | closed |
| T-02-04 | Denial of Service | Hung or long workspace run blocking the phase | low | accept | ACR-01 — every `test result:` group ≤3.03s; corroborated by `02-VALIDATION.md:27` | closed |
| T-02-05 | Tampering | `PaladinBuilder::validate` temperature bound (ASVS V5) | high | mitigate | Provider-declared range, inclusive endpoints, no clamp, typed `ConfigurationError` — `src/application/services/paladin/paladin_builder.rs:1119-1132` | closed |
| T-02-06 | Spoofing | `supports_tool_calling` over-reporting | high | mitigate | All 3 adapters `false`; correspondence test `crates/paladin-llm/src/lib.rs:119-135` | closed |
| T-02-07 | Denial of Service | Panic in new validation code | medium | mitigate | No `unwrap`/`expect`/`panic!` in `validate()` — `paladin_builder.rs:1107-1174` | closed |
| T-02-08 | Tampering | Widening published ports type breaks constructors | medium | accept | ACR-02 — additive `temperature_range: Option<(f32,f32)>`, `Default` → `None` — `crates/paladin-ports/src/output/llm_port.rs:756-790` | closed |
| T-02-09 | Tampering | `Formation::validate` count bound (ASVS V5) | medium | mitigate | `formation.rs:108-117` + `test_formation_rejects_zero_paladins` at `formation.rs:182` | closed |
| T-02-10 | Tampering | Rename altering persisted checkpoint schema | high | mitigate | `git show 05fde82` — fields/order/types/derives byte-identical, no `#[serde(rename)]`, no version bump | closed |
| T-02-11 | Denial of Service | Panic in domain validation | medium | mitigate | Same `validate()` sites as T-02-07/09 — no panic sites introduced | closed |
| T-02-12 | Repudiation | Stale doc example teaching a removed type name | low | mitigate | `git show 05fde82` — `citadel_port.rs` example updated to `BattalionCheckpointConfig` | closed |
| T-02-13 | Spoofing | Herald rendering invented data | high | mitigate | `format_battalion_result` reads its argument — `json_herald.rs:179`, `markdown_herald.rs:216`, `table_herald.rs:168` | closed |
| T-02-14 | Tampering | Herald output built by string concatenation | medium | mitigate | `serde_json` value construction, no concatenation — `json_herald.rs:179-228` | closed |
| T-02-15 | Information Disclosure | Node error text carrying provider bodies into telemetry | medium | accept | ACR-03 — `error_aggregation.rs` untouched by any phase-02 commit (last touch `3621fd9`, pre-phase) | closed |
| T-02-16 | Denial of Service | Panic rendering empty/malformed result | medium | mitigate | Empty-result tests `table_herald.rs:474`, `json_herald.rs:450`; `markdown_herald.rs:248` iterates | closed |
| T-02-17 | Spoofing | Test proving only formatter self-consistency | high | mitigate | `battalion_herald_end_to_end_test.rs` — 0 `BattalionResult {` literals; real `FormationExecutionService::execute` at `:217` | closed |
| T-02-18 | Tampering | Assertions passing on any output | medium | mitigate | Positional `per_paladin[0..2]` asserts `:242-249`; total computed from mocks `:222-226` | closed |
| T-02-19 | Information Disclosure | Credential-shaped literal in test fixture | low | mitigate | `"Simulated failure for {}"` — synthetic, `battalion_herald_end_to_end_test.rs:137-140` | closed |
| T-02-20 | Denial of Service | Hanging integration test | low | mitigate | In-process mocks only; no network/filesystem imports in the test file | closed |
| T-02-21 | Spoofing | Task complete but tests never execute | high | mitigate | Live re-run `cargo test --test unit -- llm` → 41 passed, 0 failed, 0 ignored | closed |
| T-02-22 | Tampering | Silent test deletion hiding a gap | high | mitigate | `git show 465ecdb 7c3d4b7 7257f88 \| grep -c '^+.*#\[ignore'` → 0 | closed |
| T-02-23 | Tampering | Process-wide env mutation from parallel threads | medium | mitigate | Every `set_var`/`remove_var` in `unsafe{}` with `// SAFETY:`; `CleanProviderEnv` RAII guard — `tests/unit/llm/provider_factory_test.rs` | closed |
| T-02-24 | Information Disclosure | Provider test embedding a real API key | high | mitigate | `provider_switching_test.rs:82-83` `"test-key"` + local mockito URL; live re-run 2 passed; credential grep 0 | closed |
| T-02-25 | Denial of Service | Reactivated test hanging on mock server | medium | mitigate | Async `Server::new_async`; `test_deepseek_timeout` completes ~1.16s | closed |
| T-02-26 | Spoofing | Ledger items outstanding while 1,895 test lines uncompiled | high | mitigate | Live re-run `cargo test --features cli --test cli` → 99 passed, 0 failed, 0 ignored | closed |
| T-02-27 | Tampering | Silencing a reactivated suite to reach green | high | mitigate | `git show aa3f2f5 \| grep -c '^+.*#\[ignore'` → 0 | closed |
| T-02-28 | Tampering | Process-wide env mutation in CLI barrel | medium | mitigate | `tests/cli/mod.rs:38-43` `NO_COLOR` unsafe block has `SAFETY` comment; 5 reactivated suites add 0 new mutation sites | closed |
| T-02-29 | Information Disclosure | Reactivated CLI test embedding a real key | high | mitigate | Credential-pattern grep over `tests/cli/*.rs` → 0 matches | closed |
| T-02-30 | Elevation of Privilege | Duplicated mock diverging from shipped one | medium | mitigate | `tests/cli/helpers.rs` declares 0 mock types / port impls; `tests/helpers/` untouched by any phase-02 commit | closed |
| T-02-31 | Repudiation | Verdict recorded without traceable evidence | high | mitigate | Row S1.1 spot-checked against `garrison_port.rs:430` / `in_memory_garrison.rs:229`; `test_remember_and_recall` re-run → passed | closed |
| T-02-32 | Spoofing | Criterion upgraded on strength of code existing | high | mitigate | FR8.3 honestly downgraded to `present, unproven`; `grep -rn "\.validate()" src` confirms no Garrison call site | closed |
| T-02-33 | Tampering | Advisory coverage figure later read as a gate | medium | mitigate | `grep "%" 02-garrison-prd-review.md` → 0 matches | closed |
| T-02-34 | Information Disclosure | Quoted PRD/code content carrying a credential | low | mitigate | Credential grep over `02-garrison-prd-review.md` → 0 matches | closed |
| T-02-35 | Repudiation | Ledger row amended without traceable evidence | high | mitigate | `milestone-01.md:299,356,386,414,481` — each carries date `2026-08-01`, plan `02-09`, and a citation | closed |
| T-02-36 | Tampering | Wholesale edit destroying rows outside intent | high | mitigate | `git show 0dd3ae9 -- .planning/ROADMAP.md` → single hunk, confined to the Phase 2 block | closed |
| T-02-37 | Spoofing | Deferral recorded with no owner | high | mitigate | `milestone-01.md:357,431` name authority + owner; ADR-0007 names owner and prerequisite; cited grep re-run → 0 matches | closed |
| T-02-38 | Tampering | ADR conformance line reading as pending work | medium | mitigate | ADR-0007 § Code Conformance = "conforms"; parses under `adr-parser.cjs`, status `accepted` | closed |
| T-02-39 | Information Disclosure | Sweep output carrying environment content | low | mitigate | Credential/env grep over `02-test-wiring-sweep.md` → 0 matches | closed |
| T-02-40 | Denial of Service | `truncate_text` byte-index panic on Paladin name | high | mitigate | Char-count budget, total for every input — `crates/paladin-herald/src/table_herald.rs:110-123` | closed |
| T-02-41 | Denial of Service | `format_error` panic on `PaladinError` display string | high | mitigate | Routes through the same fixed `truncate_text` — `table_herald.rs:292-307` | closed |
| T-02-42 | Denial of Service | `usize` underflow for `max_column_width` of 1 or 2 | medium | mitigate | Explicit `max_column_width < 3` branch — `table_herald.rs:114-115` | closed |
| T-02-43 | Tampering | Truncation splitting a multi-byte sequence | low | mitigate | Tests assert absence of `U+FFFD` — `table_herald.rs:508,532,543,649,692` | closed |
| T-02-44 | Information Disclosure | `format_error` surfacing provider error bodies | low | accept | ACR-04 — `git show 617a0bb` confirms rendered text unchanged; only `truncate_text` internals fixed | closed |
| T-02-45 | Repudiation | Requirement marked complete with no evidence | high | mitigate | Dated provenance note citing `02-VERIFICATION.md`, `milestone-01.md`, commit `9e5ec04` — `.planning/REQUIREMENTS.md:236-254` | closed |
| T-02-46 | Tampering | Whole-file rewrite of `REQUIREMENTS.md` | high | mitigate | `git show c3fc822 --numstat` → 34 ins / 14 del; re-grepped checked 15, unchecked 71, Pending 71 — exact match | closed |
| T-02-47 | Spoofing | Citing a SUMMARY, row or commit that does not exist | high | mitigate | `git cat-file -t 9e5ec04 a5f8c27` both resolve; `02-VERIFICATION.md:99-107` matches every citation | closed |
| T-02-48 | Elevation of Privilege | GAP-03 complete while reverting defect still live | high | mitigate | Precondition `02-11-PLAN.md:92-97`, `depends_on: ["02-10"]`, wave 6; `cargo test -p paladin-herald` re-run → 70 passed, 0 failed | closed |
| T-02-49 | Information Disclosure | Secrets leaking into a planning document | low | accept | ACR-05 — `git show c3fc822` credential grep → 0; diff holds only IDs, paths, a date and 2 SHAs | closed |
| T-02-SC (02-01) | Tampering | npm/pip/cargo installs | high | mitigate | `git log ebb5d9d..f5e4aa2 -- Cargo.toml Cargo.lock '*/Cargo.toml'` → 0 across the phase; `02-RESEARCH.md:95` Package Legitimacy Audit | closed |
| T-02-SC (02-02) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-03) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-04) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-05) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-06) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0; `mockito` already-pinned version, no upgrade | closed |
| T-02-SC (02-07) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-08) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-09) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0 | closed |
| T-02-SC (02-10) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0; `std::str::chars` only | closed |
| T-02-SC (02-11) | Tampering | npm/pip/cargo installs | high | mitigate | Phase-wide dependency-diff check → 0; docs-only edit | closed |

*Status: open · closed · open — below high threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above `workflow.security_block_on` (high) count toward `threats_open`*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| ACR-01 | T-02-04 | A hung or extremely long workspace run is recorded as evidence and escalated, not worked around. The full suite completed within the standard timeout on the baselined commit; every `test result:` group finished in ≤3.03s. | Plan 02-01 | 2026-08-01 |
| ACR-02 | T-02-08 | `#[non_exhaustive]` + `Default` was explicitly rejected per D-15 as a breaking change for downstream constructors, and deferred to the Phase 4 / REL-01 version decision. The additive field with a `None` default preserves current behaviour for every unmigrated site. | D-15 (Plan 02-02) | 2026-08-01 |
| ACR-03 | T-02-15 | Surfacing node error text in the Herald changes the display surface, not the data collected — `AggregatedError` and `export_metadata` already record it. Whether provider error bodies should be redacted before persistence is a separate concern with no requirement in this phase. | Plan 02-04 | 2026-08-01 |
| ACR-04 | T-02-44 | `format_error` already renders provider error-body text today; the Phase 2 change alters only whether the render completes. Redaction is a separate concern with no requirement in this phase; carried forward rather than silently widened. | Plan 02-10 | 2026-08-01 |
| ACR-05 | T-02-49 | The `REQUIREMENTS.md` task writes only requirement IDs, plan numbers, file paths, a date and two commit SHAs. No command output pasted, no environment dumped, no configuration values copied. | Plan 02-11 | 2026-08-01 |

*Accepted risks do not resurface in future audit runs.*

**Forward owners for deferred concerns:** ACR-02 → Phase 4 / REL-01 version decision. ACR-03 and ACR-04 (provider error-body redaction before persistence and display) share one root concern with no owning requirement yet — recommend raising it against a future phase rather than letting it expire with this milestone.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-01 | 60 | 60 | 0 | gsd-security-auditor (`/gsd-secure-phase 2`) |

**Audit method:** ASVS L1. Verification went beyond presence-grep where cheap — live test re-runs (`cargo test --test unit -- llm` → 41/0/0; `cargo test --features cli --test cli` → 99/0/0; `cargo test -p paladin-herald` → 70/0), commit resolution via `git cat-file -t`, independent re-derivation of the `02-test-baseline.md` totals and the `REQUIREMENTS.md` bullet-count deltas, and ADR-0007 re-parsed under `adr-parser.cjs`.

### Auditor Observations (non-blocking)

1. **Process gap — no `## Threat Flags` section in any of the 11 SUMMARY files.** The convention was not populated, not even as "none". No undetected attack surface resulted: every file touched by Phase 2 traces to a registered threat, and a credential sweep across all phase artifacts and reactivated tests found only accounted-for synthetic placeholders (`test-key`, `sk-ant-test123`, `sk-mock-*-key`). Worth fixing as a convention for later phases.
2. **Pre-existing, out of Phase 2 scope —** `crates/paladin-herald/src/json_herald.rs:228` `format_error` has a fallback branch that raw-interpolates `error.to_string()` into a JSON string literal when `serde_json::to_string_pretty` fails. Confirmed via `git log` as predating Phase 2 and untouched by it. Recorded for awareness; not an open threat of this phase.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-08-01
