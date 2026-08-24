# DOCS-03 gate evidence — `cargo doc` zero-warning bar

Verbatim before/after output of the CI gate at `.github/workflows/ci.yml:63`
(`lint` job), captured while executing plan `16-07`, per D-00e and D-08. Both
runs use the identical command, quoted character for character from the
workflow file:

```
cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
```

## Before — 2026-08-24, HEAD `41694e2c` (unmodified tree)

**Command:** `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`
**Exit status:** `1` (gate fails)

`grep -c "^warning: " /tmp/doc-output.txt` → `24` (20 individual warnings + 4
per-crate summary lines), split identically to ADR-0033's 2026-08-08
measurement:

```
warning: `paladin-battalion` (lib doc) generated 3 warnings
warning: `paladin-herald` (lib doc) generated 1 warning
warning: `paladin-web` (lib doc) generated 13 warnings
warning: `paladin-ai` (lib doc) generated 3 warnings
```

### The 20 warnings, re-derived fresh this session (not copied from ADR-0033)

```
paladin-battalion (3):
  crates/paladin-battalion/src/in_memory_registry.rs:9          unresolved link `paladin-core`
  crates/paladin-battalion/src/in_memory_registry.rs:9          unresolved link `paladin-ports`
  crates/paladin-battalion/src/in_memory_registry.rs:65:44       unclosed HTML tag `Paladin` (Arc<Paladin>)

paladin-web (13):
  crates/paladin-web/src/agent_auth.rs:8              unresolved link `AuthPort`
  crates/paladin-web/src/agent_registry.rs:5           unresolved link `Paladin`
  crates/paladin-web/src/agent_registry.rs:5           unresolved link `PaladinExecutorPort`
  crates/paladin-web/src/agent_registry.rs:10          unresolved link `PaladinExecutorPort`
  crates/paladin-web/src/agent_registry.rs:10          unresolved link `Paladin`
  crates/paladin-web/src/delivery_controller.rs:8      unresolved link `deliver_content`
  crates/paladin-web/src/delivery_controller.rs:9      unresolved link `get_delivery_status`
  crates/paladin-web/src/delivery_controller.rs:10     unresolved link `get_delivery_stats`
  crates/paladin-web/src/delivery_controller.rs:12     unresolved link `create_delivery_routes`
  crates/paladin-web/src/openapi.rs:5                  unresolved link `build_openapi`
  crates/paladin-web/src/openapi.rs:6                  unresolved link `docs_router`
  crates/paladin-web/src/agent_controller.rs:651:45    redundant explicit link `JobRecord`
  crates/paladin-web/src/app.rs:69:22                  redundant explicit link `agent_router`

paladin-herald (1):
  crates/paladin-herald/src/lib.rs:14:9                unresolved link `TableHerald`

paladin-ai / facade (3):
  src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs:18:16
      private-item link (enclosing item `mcp_streamable_http_adapter`, target `BearerToken::expose_secret`)
  src/infrastructure/web/agent_host.rs:216:56
      private-item link (enclosing item `validate_config`, target `build_agent`)
  src/infrastructure/web/agent_host.rs:268:7
      private-item link (enclosing item `build_agent_registry`, target `build_agent`)
```

**Re-derivation is visible, not assumed:** `crates/paladin-web/src/agent_auth.rs`'s
`[`AuthPort`]` link is at **line 8** in the tree today; ADR-0033's Code Locations
section cites `agent_auth.rs:7` for the same warning — a one-line drift, exactly
the example Pitfall 1 in `16-RESEARCH.md` predicted. All other citations in this
list matched ADR-0033's Code Locations section exactly (`agent_registry.rs:5,10`,
`delivery_controller.rs:8-12`, `openapi.rs:5-6`, `agent_controller.rs:651:45`,
`app.rs:69:22`, `in_memory_registry.rs:9,65:44`, `lib.rs:14:9`,
`mcp_streamable_http_adapter.rs:18:16`, `agent_host.rs:216:56,268:7`) — confirmed
by fresh `cargo doc --workspace --no-deps` output this session, not copied
verbatim from the ADR.

### Full rustdoc warning text (compilation noise elided; all 24 `warning:` lines retained)

```
warning: unresolved link to `paladin-core`
  |
  = note: the link appears in this line:

          Because this type depends only on [`paladin-core`] and [`paladin-ports`] it carries
                                             ^^^^^^^^^^^^^^
  = note: no item named `paladin-core` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
  = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default

warning: unresolved link to `paladin-ports`
  |
  = note: the link appears in this line:

          Because this type depends only on [`paladin-core`] and [`paladin-ports`] it carries
                                                                  ^^^^^^^^^^^^^^^
  = note: no item named `paladin-ports` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unclosed HTML tag `Paladin`
  --> crates/paladin-battalion/src/in_memory_registry.rs:65:44
   |
65 |     /// Internal storage: Paladin ID -> Arc<Paladin>
   |                                            ^^^^^^^^^
   |
   = note: `#[warn(rustdoc::invalid_html_tags)]` on by default
help: try marking as source code
   |
65 |     /// Internal storage: Paladin ID -> `Arc<Paladin>`
   |                                         +            +

warning: `paladin-battalion` (lib doc) generated 3 warnings

warning: unresolved link to `AuthPort`
  |
  = note: the link appears in this line:

            the injected [`AuthPort`] against the server's own in-process token store — not a
                          ^^^^^^^^^^
  = note: no item named `AuthPort` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
  = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default

warning: unresolved link to `Paladin`
  |
  = note: the link appears in this line:

          entry pairs a [`Paladin`] with its own [`PaladinExecutorPort`] implementation
                         ^^^^^^^^^
  = note: no item named `Paladin` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `PaladinExecutorPort`
  |
  = note: the link appears in this line:

          entry pairs a [`Paladin`] with its own [`PaladinExecutorPort`] implementation
                                                  ^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `PaladinExecutorPort` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `PaladinExecutorPort`
  |
  = note: the link appears in this line:

          [`PaladinExecutorPort`] *trait* (`paladin-ports`) and the [`Paladin`] entity
           ^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `PaladinExecutorPort` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `Paladin`
  |
  = note: the link appears in this line:

          [`PaladinExecutorPort`] *trait* (`paladin-ports`) and the [`Paladin`] entity
                                                                     ^^^^^^^^^
  = note: no item named `Paladin` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `deliver_content`
  |
  = note: the link appears in this line:

          | `POST /api/delivery/deliver` | [`deliver_content`] | Deliver a content payload now |
                                            ^^^^^^^^^^^^^^^^^
  = note: no item named `deliver_content` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `get_delivery_status`
  |
  = note: the link appears in this line:

          | `GET /api/delivery/status/{delivery_id}` | [`get_delivery_status`] | Look up a delivery by id |
                                                        ^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `get_delivery_status` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `get_delivery_stats`
  |
  = note: the link appears in this line:

          | `GET /api/delivery/stats` | [`get_delivery_stats`] | Aggregate delivery statistics |
                                         ^^^^^^^^^^^^^^^^^^^^
  = note: no item named `get_delivery_stats` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `create_delivery_routes`
  |
  = note: the link appears in this line:

          Build the router with [`create_delivery_routes`] and merge it into the application router.
                                 ^^^^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `create_delivery_routes` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `build_openapi`
  |
  = note: the link appears in this line:

          [`build_openapi`] assembles the `/v1` agent API document and decorates it with API info
           ^^^^^^^^^^^^^^^
  = note: no item named `build_openapi` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `docs_router`
  |
  = note: the link appears in this line:

          and the two security schemes (API key + opaque bearer token); [`docs_router`] serves it at
                                                                         ^^^^^^^^^^^^^
  = note: no item named `docs_router` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `TableHerald`
  --> crates/paladin-herald/src/lib.rs:14:9
   |
14 | //! - [`TableHerald`] — compact table output. Requires the `table` feature (gates the
   |         ^^^^^^^^^^^ no item named `TableHerald` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default

warning: redundant explicit link target
   --> crates/paladin-web/src/agent_controller.rs:651:45
    |
651 | /// Returns `200 OK` with the [`JobRecord`](crate::job_store::JobRecord), or `404` if no
    |                                -----------  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ explicit target is redundant
    |                                |
    |                                because label contains path that resolves to same destination
    |
    = note: when a link's destination is not specified,
            the label is used to resolve intra-doc links
    = note: `#[warn(rustdoc::redundant_explicit_links)]` on by default
help: remove explicit link target
    |
651 - /// Returns `200 OK` with the [`JobRecord`](crate::job_store::JobRecord), or `404` if no
651 + /// Returns `200 OK` with the [`JobRecord`], or `404` if no
    |

warning: redundant explicit link target
  --> crates/paladin-web/src/app.rs:69:22
   |
69 | /// [`agent_router`](crate::agent_controller::agent_router), merged in. It is the
   |      --------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ explicit target is redundant
   |      |
   |      because label contains path that resolves to same destination
   |
   = note: when a link's destination is not specified,
           the label is used to resolve intra-doc links
help: remove explicit link target
   |
69 - /// [`agent_router`](crate::agent_controller::agent_router), merged in. It is the
69 + /// [`agent_router`], merged in. It is the
   |

warning: `paladin-herald` (lib doc) generated 1 warning
warning: `paladin-web` (lib doc) generated 13 warnings

warning: public documentation for `mcp_streamable_http_adapter` links to private item `BearerToken::expose_secret`
  --> src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs:18:16
   |
18 | //! only via [`BearerToken::expose_secret`], used exclusively inside
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^ this item is private
   |
   = note: this link will resolve properly if you pass `--document-private-items`
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default

warning: public documentation for `validate_config` links to private item `build_agent`
   --> src/infrastructure/web/agent_host.rs:216:56
    |
216 | /// checked when the provider is actually created in [`build_agent`].
    |                                                        ^^^^^^^^^^^ this item is private
    |
    = note: this link will resolve properly if you pass `--document-private-items`

warning: public documentation for `build_agent_registry` links to private item `build_agent`
   --> src/infrastructure/web/agent_host.rs:268:7
    |
268 | /// [`build_agent`]. A validation failure, an unresolvable provider, or a build failure
    |       ^^^^^^^^^^^ this item is private
    |
    = note: this link will resolve properly if you pass `--document-private-items`

warning: `paladin-ai` (lib doc) generated 3 warnings
```

## After (task 1) — 2026-08-24, task-1 fixes applied, `paladin-herald`'s attribute unchanged

**Command:** `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`
**Exit status:** `0` (gate passes)

`grep -c "warning:" /tmp/doc-output.txt` → `0`. All 20 warnings cleared by
class: 14 unresolved intra-doc links de-linked to plain code spans, 3
private-item links de-linked to plain code spans, 2 redundant explicit link
targets dropped (rustdoc's own suggested fix), 1 unclosed HTML tag wrapped in
a code span. Nothing suppressed — no `#[allow(rustdoc::...)]` added, no doc
example downgraded to a non-compiling fence, no doc comment deleted. Confirmed
by `git diff -U0 -- '*.rs' | grep -c '^+.*allow(rustdoc'` → `0` and
`git diff -U0 -- '*.rs' | grep -c '^+.*```rust,ignore'` → `0`.
`git diff --exit-code .github/workflows/` → exit `0` (no workflow file touched).

## After (task 2, final) — 2026-08-24, `paladin-herald`'s missing-docs bar flipped

**`cargo doc -p paladin-herald --no-deps` after flipping
`crates/paladin-herald/src/lib.rs:20` from `#![allow(missing_docs)]` to
`#![warn(missing_docs)]`:**

```
    Checking paladin-ai-core v0.8.0 (.../crates/paladin-core)
 Documenting paladin-herald v0.8.0 (.../crates/paladin-herald)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 16.73s
   Generated .../target/doc/paladin_herald/index.html
```

`grep -c "warning:" /tmp/doc-output.txt` → `0`. This matches M-07's
research-session measurement exactly: flipping the attribute produces zero
additional warnings.

**Final recorded state — full workspace gate, flip applied, this plan's last recorded run:**

**Command:** `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`
**Exit status:** `0` (gate passes)

```
    Checking paladin-herald v0.8.0 (.../crates/paladin-herald)
 Documenting paladin-herald v0.8.0 (.../crates/paladin-herald)
    Checking paladin-ai v0.8.0 (.../workspace/.claude/worktrees/agent-a58f2fdcc23350adc)
 Documenting paladin-ai v0.8.0 (.../workspace/.claude/worktrees/agent-a58f2fdcc23350adc)
 Documenting paladin-doc-examples v0.8.0 (.../crates/doc-examples)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.80s
   Generated .../target/doc/paladin/index.html and 12 other files
```

`grep -c "warning:" /tmp/doc-output.txt` → `0`. The exact `ci.yml:63` command
exits `0` with the flipped attribute in place, confirming the gate holds with
the missing-docs bar now uniform across all ten library crates and the
facade.

## Summary

| Run | Command exit | Warnings |
|---|---|---|
| Before (unmodified tree) | 1 | 20 (24 `warning:` lines incl. 4 per-crate summaries) |
| After task 1 (link/HTML-tag fixes; herald attribute unchanged) | 0 | 0 |
| After task 2 (herald `missing_docs` flipped to `warn`) | 0 | 0 |

DOCS-03's "adds the CI gate" clause was already satisfied by `ci.yml:63`
before this plan ran (D-00u) — this plan applies the already-ratified bar to
the tree and proves the gate green; it does not create the gate mechanism.
No `.github/workflows/` file was modified by this plan.
