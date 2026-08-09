# Deferred Features — removed from the facade, recorded for future implementation

> **RECORDS RELOCATED TO `.planning/` — 2026-08-08 (FACADE-03).**
> Both features' records now live at
> [`.planning/registers/facade-03-removed-features.md`](../../.planning/registers/facade-03-removed-features.md),
> so the answer to "why can I not run `paladin user register`?" is findable from `.planning/`
> without opening this document. The `paladin-ml` leaf-crate placement condition in §2 below is
> additionally promoted to
> [`.planning/decisions/0035-paladin-ml-leaf-crate-placement.md`](../../.planning/decisions/0035-paladin-ml-leaf-crate-placement.md)
> (ADR-0035), since it is a contested position rather than settled status.
>
> **Recovery-pointer correction.** The recoverable form is the immutable commit `3d48768`, given as
> the runnable command `git show 3d48768^:src/application/cli/commands/user.rs`. **One commit
> removed both features** — `3d48768` deletes both `src/application/cli/commands/user.rs` and
> `src/infrastructure/adapters/input/tensorflow_adapter.rs` — so attributing the CLI removal to a
> branch (below, at `:72`) and the ML removal to a commit splits a single event across two pointer
> kinds. Measured this session: no *local* branch named `chore/facade-cleanup-m8-finish` exists in
> this checkout, while a *remote-tracking* ref of that name does resolve
> (`refs/remotes/origin/chore/facade-cleanup-m8-finish`) and is an ancestor of the removal commit.
> A branch ref — local or remote-tracking — is mutable and deletable regardless of whether it
> happens to resolve today; that mutability, not its absence, is the durable reason to cite the
> immutable SHA instead.
>
> The original text below is retained unmodified.

**Date:** 2026-06-04
**Context:** Milestone 8 facade cleanup. Two half-built/placeholder features were removed from
`src/` to keep the facade lean. They were never wired into a runnable path. This document
preserves their intent so they can be reintroduced deliberately (with tests, TDD) when prioritized.

---

## 1. CLI user-management commands (`paladin user …`)

**Removed:** `src/application/cli/commands/user.rs` (1,065 LOC) and its `pub mod user;`
declaration in `src/application/cli/commands/mod.rs`.

**Status when removed:** The command module was declared but **never dispatched** from the CLI
binary (`src/bin/paladin-cli.rs`) — no `UserCommands` arm existed in the top-level command match,
so the subcommands were unreachable. It compiled but did nothing.

**Backend still exists:** The service layer it depended on is intact and in use elsewhere:
- `core::platform::manager::user_service::{UserService, UserServiceTrait, UserRegistrationRequest,
  UserLoginRequest, UserProfileUpdateRequest}`
- `core::platform::container::user::{User, UserProfile}`

Re-implementing the CLI surface is therefore mostly re-wiring, not new domain work.

**Intended command surface (clap subcommands):**

| Subcommand | Purpose | Key args |
|-----------|---------|----------|
| `register` | Register a new user | username, email, password, first/last name, bio, timezone, locale |
| `login` | Authenticate a user | email, password |
| `get` | Fetch user info | user id or email |
| `update` | Update a profile | user id, username, email, first/last name, bio, avatar URL |
| `list` | List users | filter by active, filter by verified, limit |
| `activate` | Activate a user | user id |
| `deactivate` | Deactivate a user | user id |
| `verify` | Mark a user verified | user id |

**To reintroduce:** add a `User(UserCommands)` arm to the CLI's top-level command enum and a
dispatch handler that constructs `UserService` (see `config/user_config.rs` for the existing DI
wiring) and calls the matching `UserServiceTrait` method. Add command tests under
`src/application/cli/tests/`. Recover the original module from git history (Milestone 8 Phase 3
removal commit) rather than rewriting from scratch.

---

## 2. TensorFlow ML inference adapter (`ml` feature)

**Removed:** `src/infrastructure/adapters/input/tensorflow_adapter.rs` (636 LOC), its
`#[cfg(feature = "ml")] pub mod tensorflow_adapter;` declaration in
`src/infrastructure/adapters/input/mod.rs`, and the now-unused `ml = []` feature flag in
`Cargo.toml`.

**Status when removed:** An explicit `#[doc(hidden)]` placeholder for a future `paladin-ml`
crate (Milestone 9+). It implemented `paladin_ports::input::ml_port::MlPort` but contained no real
TensorFlow integration — model loading/prediction were stubs, with `#[allow(dead_code)]` on
unused fields. Gated behind the non-default `ml` feature; nothing consumed it.

**Intended shape:** `TensorFlowAdapter` implementing `MlPort`
(`load_model` / `predict` / `model_info`), translating `MlPredictionRequest` →
TensorFlow ops → `MlPredictionResponse`, keyed by a `model_path` and a registry of loaded models.

**To reintroduce:** the port contract `paladin_ports::input::ml_port::MlPort` remains in the
workspace, so the integration point is stable. Implement the real adapter in a dedicated
`paladin-ml` leaf crate (consistent with the hexagonal layout — ML inference is an infrastructure
adapter, not facade code) rather than re-adding it to the facade. Re-add an `ml`/provider feature
flag on that crate at that time.

---

> Both modules are recoverable verbatim from git history at the Milestone 8 removal commit on
> branch `chore/facade-cleanup-m8-finish`.
