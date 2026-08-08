# FACADE-01 — D5 Rustdoc-Stdout Disposition Register

**Date:** 2026-08-08
**Subject:** FACADE-01 / M8 `deferred-items.md` D5 — "Residual `println!`/`eprintln!` in
services/infrastructure"
**Ledger key:** `REQ-m8-deferred-items-register` (`.planning/ledgers/milestone-07-08.md:345`)
**Status:** Closed by disposition — zero executable code changes (D-01, D-13)

## The finding, stated first

The register's *count* is exact and its *characterisation* is wrong. `deferred-items.md`'s D5
clause rates these 17 occurrences "low effort / low risk, the quick win" and recommends reviewing
the 6 files to "convert genuine diagnostics to `log::*`, keep intentional stdout output." Every one
of the 17 is a `///` or `//!` doc-comment line inside a fenced `rust` or `rust,ignore` code block —
rustdoc example output, not runtime library stdout. There is nothing to convert. See
`## Corpus-level finding` below.

## Re-measurement (D-00e evidence bar — re-run this session, 2026-08-08)

```
$ grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ | wc -l
17

$ grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ | grep -v '///' | grep -v '//!' | wc -l
0
```

Both figures match the register's own count (17) and `intel/code-verification.md`'s run-4 figure.
The filtered command proves every one of the 17 lines is a doc-comment line — none is executable
code. Per-file counts: `4+6+3+2+1+1` = **17**.

## Citation index — all 17 `file:line` occurrences

Full `file:line` form for every occurrence disposed of below, so a citation can be grepped directly
against this register without reading the per-file tables:

- `herald_registry.rs:165`, `herald_registry.rs:184`, `herald_registry.rs:197`,
  `herald_registry.rs:210` — `src/application/services/herald/herald_registry.rs`
- `circuit_breaker.rs:42`, `circuit_breaker.rs:44`, `circuit_breaker.rs:46`,
  `circuit_breaker.rs:305`, `circuit_breaker.rs:306`, `circuit_breaker.rs:307` —
  `src/infrastructure/resilience/circuit_breaker.rs`
- `paladin_execution_service.rs:43`, `paladin_execution_service.rs:44`,
  `paladin_execution_service.rs:466` —
  `src/application/services/paladin/paladin_execution_service.rs`
- `mcp_protocol.rs:26`, `mcp_protocol.rs:246` —
  `src/infrastructure/adapters/arsenal/mcp_protocol.rs`
- `tool_result_formatter.rs:22` — `src/infrastructure/adapters/arsenal/tool_result_formatter.rs`
- `tokio_cron_adapter.rs:32` — `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs`

## Per-occurrence dispositions

Every row's disposition resolves to the same verdict — **deliberate rustdoc-example stdout, no
conversion** — stated per row because the disposition is per-occurrence (D-01), not once per file.

### `src/application/services/herald/herald_registry.rs` (4 occurrences)

| Line | Snippet | Disposition |
|---|---|---|
| 165 | `/// println!("Available formatters: {:?}", available_formatters);` | Rustdoc example, inside the `rust,ignore` fence opened at `:163`. Deliberate stdout illustrating the example's output — not runtime library code. No conversion. |
| 184 | `///     println!("JSON formatter is available");` | Rustdoc example, inside the `rust,ignore` fence opened at `:182`. Deliberate stdout. No conversion. |
| 197 | `/// println!("Registry has {} formatters", registry.len());` | Rustdoc example, inside the `rust,ignore` fence opened at `:196`. Deliberate stdout. No conversion. |
| 210 | `///     println!("No formatters registered");` | Rustdoc example, inside the `rust,ignore` fence opened at `:208`. Deliberate stdout. No conversion. |

**Count: 4.**

### `src/infrastructure/resilience/circuit_breaker.rs` (6 occurrences)

| Line | Snippet | Disposition |
|---|---|---|
| 42 | `//!     Ok(value) => println!("Success: {}", value),` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 44 | `//!         println!("Circuit breaker is open, failing fast");` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 46 | `//!     Err(e) => println!("Operation failed: {}", e),` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 305 | `///     CircuitState::Closed { .. } => println!("Circuit is closed"),` | Rustdoc example inside an item-level (`///`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 306 | `///     CircuitState::Open { .. } => println!("Circuit is open"),` | Rustdoc example inside an item-level (`///`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 307 | `///     CircuitState::HalfOpen { .. } => println!("Circuit is half-open"),` | Rustdoc example inside an item-level (`///`) doc comment's fenced example. Deliberate stdout. No conversion. |

**Count: 6.** Lines 42/44/46 sit inside one fenced block and 305/306/307 inside another; each keeps
its own row rather than being collapsed into one per block.

### `src/application/services/paladin/paladin_execution_service.rs` (3 occurrences)

| Line | Snippet | Disposition |
|---|---|---|
| 43 | `//! println!("Output: {}", result.output);` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 44 | `//! println!("Loops: {}, Tokens: {}", result.loop_count, result.token_count);` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example, same fence as line 43. Deliberate stdout. No conversion. |
| 466 | `/// println!("Result: {}", result.output);` | Rustdoc example inside an item-level (`///`) doc comment's fenced example. Deliberate stdout. No conversion. |

**Count: 3.** Lines 43/44 sit inside one fenced block and keep separate rows rather than being
collapsed.

### `src/infrastructure/adapters/arsenal/mcp_protocol.rs` (2 occurrences)

| Line | Snippet | Disposition |
|---|---|---|
| 26 | `//! println!("Available tools: {:?}", tools);` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |
| 246 | `///     println!("Found tool: {}", tool.name);` | Rustdoc example inside an item-level (`///`) doc comment's fenced example. Deliberate stdout. No conversion. |

**Count: 2.**

### `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` (1 occurrence)

| Line | Snippet | Disposition |
|---|---|---|
| 22 | `//! println!("{}", formatted);` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |

**Count: 1.** A single-occurrence file still gets its own section heading and its own one-row
table, not a bare prose line.

### `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` (1 occurrence)

| Line | Snippet | Disposition |
|---|---|---|
| 32 | `//!     println!("Scheduled job: {}", job_id);` | Rustdoc example inside the module-level (`//!`) doc comment's fenced example. Deliberate stdout. No conversion. |

**Count: 1.**

## Arithmetic

4 (`herald_registry.rs`) + 6 (`circuit_breaker.rs`) + 3 (`paladin_execution_service.rs`) +
2 (`mcp_protocol.rs`) + 1 (`tool_result_formatter.rs`) + 1 (`tokio_cron_adapter.rs`) = **17**,
matching both the re-run grep count above and the M8 register's original figure.

## Why the exception branch fires for all 17

The rule applied across this corpus is: default to `log::*`, annotate provable exceptions. Every
one of these 17 occurrences fires the exception branch, because `println!` is the idiomatic way a
rustdoc example demonstrates its output to a reader — that is what the fenced code block exists to
show. Converting any of them to `log::*` would not improve runtime hygiene (there is no runtime
call to improve; these lines do not execute as library code, they are `///`/`//!` doc-comment text
that `rustdoc` may compile as a doctest but never runs as part of the shipped binary's normal
control flow) — it would degrade the documentation and break the examples' illustrative purpose.

Proof the exception branch is total, not partial: the same 17-line grep, filtered to lines that are
*not* `///` or `//!` doc-comment lines, returns zero:

```
$ grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ | grep -v '///' | grep -v '//!'
(no output)
```

No occurrence falls outside the exception branch. All 17 are annotated here as deliberate rustdoc
stdout; none is converted.

## Hand-off to Phase 15 — the four `rust,ignore` fences

`src/application/services/herald/herald_registry.rs` carries four of its eleven fenced examples as
` ```rust,ignore ` rather than ` ```rust `, meaning those four doctests never compile and can
therefore drift from the real API silently without any test failure to catch it:

| Occurrence line | Enclosing `rust,ignore` fence line |
|---|---|
| 165 | 163 |
| 184 | 182 |
| 197 | 196 |
| 210 | 208 |

Re-confirmed this session:

```
$ grep -n '```rust,ignore' src/application/services/herald/herald_registry.rs
21:  //! ```rust,ignore
39:  //! ```rust,ignore
52:  //! ```rust,ignore
98:    /// ```rust,ignore
119:    /// ```rust,ignore
142:    /// ```rust,ignore
163:    /// ```rust,ignore
182:    /// ```rust,ignore
196:    /// ```rust,ignore
208:    /// ```rust,ignore
233:    /// ```rust,ignore
```

The four fences at `herald_registry.rs:163`, `herald_registry.rs:182`, `herald_registry.rs:196` and
`herald_registry.rs:208` are the ones immediately preceding the four D5 occurrence lines (165, 184,
197, 210 respectively) — each `println!` sits two lines after its fence opens. These four
`file:line` fence citations are recorded here **as enclosing-fence references only**; they are
never listed as occurrence lines anywhere in this register — the occurrence lines are 165, 184,
197 and 210 (see the Citation index above and the per-file table below).

**Phase 15 is named owner.** Phase 10 already routed this crate's doctest posture to Phase 15
(the seven-crate `doctest = false` posture, tracked under HARD-07). This plan does not un-ignore
any of the four fences — that decision, and any resulting compile-and-fix work, belongs to
whichever Phase 15 plan takes up doctest posture. This register only records the finding and the
four `file:line` citations so Phase 15 does not have to re-derive them.

## Corpus-level finding

`deferred-items.md` is described across this corpus as its highest-fidelity document — the
document other phases cite as the most reliable of the Milestone 8 registers. This is the first
measured case of it being misleading. Its D5 clause is exact about the *count* (17 occurrences
across 6 files, verified twice now — once at Milestone-8-era authorship and again this session)
and wrong about the *kind* (it frames them as runtime diagnostic residue needing case-by-case
conversion judgment, when in fact all 17 are rustdoc example lines with nothing to convert). This
is worth stating plainly as a fact about the corpus, not only about D5: even the corpus's most
carefully measured document can misclassify what it measured, and a register's count being right
is not the same as its recommendation being right.

## Cross-references

- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` — the D5 clause this
  register corrects, at `:95-105` and `:110`, dated correction banner added by this plan.
- `.planning/ROADMAP.md:726` — §Phase 11 criterion 1, amended in place by this plan.
- `.planning/ledgers/milestone-07-08.md:345` — `REQ-m8-deferred-items-register`, amended Evidence
  cell records the D5 disposition and cites this register.
