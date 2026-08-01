//! Test-helper shim for the `cli` test target.
//!
//! The `cli` binary is its own `[[test]]` target rooted at `tests/cli/mod.rs`
//! (see `Cargo.toml`), so unlike the `unit` and `lib` targets it does not
//! automatically see the sibling `tests/helpers/` directory. This module
//! re-exports the existing mock barrel rather than duplicating it: no
//! `MockLlmAdapter`, `MockPaladinPort` or `MockArsenalPort` is redefined
//! here, only re-exported so `crate::helpers::…` resolves for the
//! reactivated suites in this target.

// The `cli` `[[test]]` target is its own crate, so pulling in the whole
// `tests/helpers/` tree here compiles items this crate's five reactivated
// suites do not all individually call (e.g. `MockArsenalPort::set_error_variant`,
// `MockLlmAdapter::last_invocation`, or the re-exported `Invocation`/
// `MockResponse`/`create_mock_with_mixed_responses`/`create_mock_with_tool_calls`/
// `create_test_paladin_with_mock` names this shim does not forward) even
// though every one of them is used by the `unit`/`lib` targets that already
// depend on this same barrel — hence the crate-local allows rather than
// trimming or forking `tests/helpers/mod.rs` itself, which must stay
// unmodified.
#[allow(dead_code, unused_imports)]
#[path = "../helpers/mod.rs"]
mod shared;

// This module is compiled twice: once as the `cli` `[[test]]` target's own
// crate root submodule (where the five reactivated suites resolve
// `crate::helpers::…` to these re-exports), and again as `crate::cli::helpers`
// when `tests/lib.rs`'s pre-existing `pub mod cli;` pulls the whole `tests/cli/`
// tree into the auto-discovered `lib` test binary — where sibling test files
// instead resolve `crate::helpers` to `tests/lib.rs`'s own top-level `helpers`
// module, leaving this re-export legitimately unused in that second context.
#[allow(unused_imports)]
pub use shared::{MockArsenalPort, MockLlmAdapter, MockPaladinPort, create_mock_with_responses};
