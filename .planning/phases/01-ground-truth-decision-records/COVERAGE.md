# API Coverage — Phase 1: Ground Truth & Decision Records

No external API integration: this phase writes only Markdown records under `.planning/`
(six ADRs in `.planning/decisions/`, one cited status ledger in `.planning/ledgers/`, and
scoped edits to PROJECT.md / REQUIREMENTS.md / ROADMAP.md) and runs one local coverage
measurement — it adds no client, endpoint, SDK or transport, and modifies no Rust source.

## Why the detector fires anyway

The deterministic scan over the phase scope returns `detected: true` on two incidental prose
matches inside the plan bodies, neither of which is an integration:

- **`integration` + `api`**, in plan 01-07 Task 1 — the phrase "the live-provider-API integration
  test suite". That is a *ledger verdict* about an already-shipped Milestone 1 test suite (behind
  the `live-api-tests` feature). Phase 1 classifies it; it does not build it or call it.
- **`(surface)` + `api`**, in the plan threat models — the phrase "a plaintext OpenAI API key".
  That is a *security note* about a secret found in an ingested historical document which no
  Phase 1 artifact quotes. Not an integration.

The one external-surface touch anywhere in the phase is `cargo install cargo-llvm-cov` in plan
01-04, a build/CI tool already used by this project's own
`.github/workflows/integration-tests.yml:117`. It is a developer toolchain dependency, not a
capability surface the product integrates against, so it has no capability surface to subtract
from. Its legitimacy is recorded in `01-RESEARCH.md` § Package Legitimacy Audit (verdict OK).

The external APIs this framework really does integrate — the OpenAI, DeepSeek and Anthropic LLM
providers — were built in Milestone 1 Epic 6 and are shipped. Their coverage surface belongs to the
phase that changes them (Phase 2's GAP-07 for the `ProviderCapabilities` temperature range recorded
by ADR-0004, and Phase 14's WEB-03/WEB-04 for tool calling), not to a phase that only records what
they already are.

## Addendum — offline measurement path

The `cargo install cargo-llvm-cov` step named above as this phase's one external-surface touch was
**not used**. Plan 01-04 halted at that precondition — `cargo-llvm-cov` was absent and crates.io
returned HTTP 403 from that sandbox — before this addendum existed. Plan 01-09 carried information
01-04 did not have: the `llvm-tools` rustup component is already installed, so `rustc`'s own
source-based coverage instrumentation (`RUSTFLAGS="-C instrument-coverage"`, `llvm-profdata merge`,
`llvm-cov report`, resolved from `$(rustc --print sysroot)`) produces the same measurement fully
offline, with no crate installed and no registry touched. The registry trust boundary this
declaration's opening paragraph named is therefore eliminated: this phase's gap-closure set
installs nothing. See `01-coverage-measurement.md` for the full command record.
