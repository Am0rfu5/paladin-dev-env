# FACADE-03 — Removed Features Register: `paladin user …` CLI and TensorFlow ML Adapter

**Date:** 2026-08-08
**Subject:** FACADE-03 — the two features deliberately removed from the facade during Milestone 8,
their reintroduction conditions, and their recovery pointer.
**Ledger keys:** `REQ-deferred-cli-user-commands`, `REQ-deferred-tensorflow-ml-adapter-v3`
(`.planning/ledgers/milestone-07-08.md`)
**Status:** Closed by recorded deferral — conditions intact, neither feature reintroduced, no
crate created (D-09)

## Why this file exists

Until this session the only record of these two removals lived at
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md` — a DOC, second-from-
bottom in the D-00b precedence order and auto-overridable by the next document that mentions it.
Someone asking "why can I not run `paladin user register`?" should find the answer here, in
`.planning/`, without opening `.project/`. This file is that `.planning`-native home for both
records.

## Re-measurement (D-00e evidence bar — re-run this session, 2026-08-08)

```
$ git log -1 --format="%H %ai %s" 3d48768
3d487689a4f9099083003c1a7686a5fb9ae287ae 2026-06-04 19:46:05 +0000 chore(facade): remove half-built user CLI + tensorflow ML stub (M8)

$ git show --stat 3d48768
commit 3d487689a4f9099083003c1a7686a5fb9ae287ae
 Cargo.toml                                                       |    1 -
 .../deferred-features.md                                         |   72 ++
 src/application/cli/commands/mod.rs                              |    1 -
 src/application/cli/commands/user.rs                             | 1065 --------------------
 src/infrastructure/adapters/input/mod.rs                         |    2 -
 .../adapters/input/tensorflow_adapter.rs                         |  636 ------------
 6 files changed, 72 insertions(+), 1705 deletions(-)

$ ls src/application/cli/commands/
agent.rs  arsenal.rs  battalion.rs  council.rs  features.rs  maneuver.rs  mod.rs  muster.rs
onboarding.rs  setup_check.rs
$ ls src/application/cli/commands/ | wc -l
10
$ ls src/application/cli/commands/ | grep -c '^user.rs$'
0

$ test -d crates/paladin-ml; echo $?
1

$ find crates/paladin-ports -iname "*ml_port*"
crates/paladin-ports/src/input/ml_port.rs

$ git branch --list '*facade-cleanup-m8-finish*' | wc -l
0

$ git rev-parse --verify refs/remotes/origin/chore/facade-cleanup-m8-finish
4bf67454cccce792f9e61845668e23b29601ea33
(exit 0 — resolves)
```

**One commit removed both features.** `git show --stat 3d48768` confirms both
`src/application/cli/commands/user.rs` (1,065 LOC) and
`src/infrastructure/adapters/input/tensorflow_adapter.rs` (636 LOC) were deleted in the same
commit, on the same line of history — not two separate removals attributable to two different
kinds of pointer.

**The measured branch state, honestly recorded.** No *local* branch named
`chore/facade-cleanup-m8-finish` exists in this checkout (`git branch --list` returns zero
matches). A *remote-tracking* ref of that name does resolve —
`refs/remotes/origin/chore/facade-cleanup-m8-finish` → `4bf6745…`, and `git merge-base
--is-ancestor 3d48768 refs/remotes/origin/chore/facade-cleanup-m8-finish` succeeds, so commit
`3d48768` is reachable from that remote-tracking ref. This corrects the framing carried in
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md:72` and in
`11-CONTEXT.md`'s D-10, both of which assert the branch is "not present as a local **or remote**
ref" — that stronger claim is false today. The half that does hold, and the half the recovery
decision below actually rests on, is that no *local* branch of that name exists. The reason to
cite the SHA rather than any branch — local or remote-tracking — does not depend on the branch's
absence: a branch ref (local or remote-tracking) is mutable and deletable at any time by anyone
with push access, while a commit SHA is immutable once created. That property holds regardless of
whether `chore/facade-cleanup-m8-finish` happens to resolve on the day someone reads this file.

## Section 1 — the `paladin user …` CLI surface

**What it was:** `src/application/cli/commands/user.rs`, 1,065 LOC, eight clap subcommands:
`register`, `login`, `get`, `update`, `list`, `activate`, `deactivate`, `verify`.

**Why it was removed:** Declared but never dispatched — no `UserCommands` arm existed in the CLI
binary's (`src/bin/paladin-cli.rs`) top-level command match, so the module compiled and did
nothing. It was dead surface, not a working feature that was cut.

**What survives:** The backend is intact and in active use elsewhere in the tree:
`core::platform::manager::user_service::{UserService, UserServiceTrait,
UserRegistrationRequest, UserLoginRequest, UserProfileUpdateRequest}` and
`core::platform::container::user::{User, UserProfile}`, with the existing `config/user_config.rs`
DI wiring available to reconnect to.

**What reintroduction costs:** Add a `User(UserCommands)` arm to the CLI's top-level command enum,
a dispatch handler that constructs `UserService` via the existing `config/user_config.rs` wiring
and calls the matching `UserServiceTrait` method, plus command tests under
`src/application/cli/tests/`. This is re-wiring against an intact backend, not new domain work.

**The recovery pointer:**

```
git show 3d48768^:src/application/cli/commands/user.rs
```

This recovers the module verbatim as it stood immediately before the removal commit, addressed by
the immutable SHA `3d48768` rather than by a branch name.

**A security note, one sentence.** The eight subcommands are registration, login and
account-lifecycle operations — not a purely mechanical re-wiring exercise from a security
standpoint — so a future reintroduction should carry a security review before it ships, not be
scheduled as a mechanical restore. That review itself belongs to `/gsd-secure-phase` and is not
performed here.

## Section 2 — the TensorFlow ML adapter

**What it was:** `src/infrastructure/adapters/input/tensorflow_adapter.rs`, 636 LOC, a
`#[doc(hidden)]` placeholder implementing `MlPort` with stub model loading and prediction, plus the
non-default `ml = []` feature flag in `Cargo.toml`.

**Why it was removed:** Both the adapter and the tensorflow_adapter's feature flag were deleted
outright by commit `3d48768`, rather than feature-gated as the Epic 3 disposition record's action
cell had described. It contained no real TensorFlow integration — model loading and prediction
were both stubs, with `#[allow(dead_code)]` on the unused fields — and nothing consumed it.

**What survives:** `paladin_ports::input::ml_port::MlPort` at
`crates/paladin-ports/src/input/ml_port.rs`, confirmed present this session
(`find crates/paladin-ports -iname "*ml_port*"` above). The integration point is stable.

**The reintroduction condition, reproduced verbatim:** any future TensorFlow adapter goes into a
dedicated `paladin-ml` **leaf crate** with the `ml` flag on that crate, **never back into the
facade**, and `paladin_ports::input::ml_port::MlPort` **stays in the workspace** so the
integration point does not move.

`.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` (ADR-0035) is the durable, promoted
home of that condition — this register carries it here for findability; the ADR carries it for
authority. Cite the ADR, do not restate it as this register's own authority.

**The asymmetric non-goal split, stated in both directions.** The M8 Epic 3 §5 non-goal — "No new
crates created. `paladin-herald`, `paladin-ml`, etc. are not in scope." — is **overridden for
`paladin-herald`**, which exists (created by reconciliation commit `66f6c4e`, confirmed present
this session), **and still holds for `paladin-ml`**, which is absent (`test -d crates/paladin-ml`
exits `1`, above). `paladin-herald`'s existence is **not** licence to create `paladin-ml`; the two
halves of the split are independent, and each stands on its own merits. `.planning/PROJECT.md`'s
`### Out of Scope` entry, which names `paladin-arsenal`, `paladin-sanctum` and `paladin-ml`, binds
this record.

## Disposition

FACADE-03 closes on **recorded deferral with conditions intact** for both features. Neither the
`paladin user …` CLI surface nor the TensorFlow ML adapter is reintroduced by this plan, no crate
is created, and promotion of either to active scope would be a phase of its own (D-09).
