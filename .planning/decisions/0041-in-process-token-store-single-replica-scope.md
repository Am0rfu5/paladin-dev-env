# ADR-0041: In-process bearer-token store scoped to single-replica serving; shared store deferred

## Status

Accepted

**Date:** 2026-08-12

## Context

Five measured facts, re-verified against the tree this session, in the numbered style ADR-0035
uses for a contested position.

**(1) Milestone 9 Epic 5 §6.1 recorded this exact trade-off in its own words**
(`.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md:167-169`):
"tokens are validated against an in-process store, so a multi-process deployment would later
need a shared store. This is acceptable because validation is hidden behind `AuthPort`, so the
store can be swapped without touching the web layer." The same document's §6.2
(`:171-178`) states the port was designed for exactly this swap: `AuthPort` lives in
`paladin-ports`, the concrete adapter lives in the root crate, and `paladin-web` depends only on
`Arc<dyn AuthPort>` — a future shared-store adapter changes the implementation behind the seam,
not the seam itself.

**(2) The shipped `k8s/server/deployment.yaml` serves more than one replica.**
`k8s/server/deployment.yaml:14` reads `replicas: 2`, re-read this session, with live liveness
(`:74-79`) and readiness (`:80-85`) probes — the Milestone 12 Epic 7 artefact whose entire
purpose is multi-process serving.

**(3) The finding that changes the shape of the question: the shipped ConfigMap turns the
correctness edge off.** `k8s/server/configmap.yaml:44-45` sets `bearer_token: enabled: false`,
and the same file's `:32-38` authenticates instead with `${PALADIN_API_KEY_CI}` and
`${PALADIN_API_KEY_APP}` sourced from a Secret — static values that are byte-identical in every
pod. The in-process token store this record scopes only becomes reachable when an operator flips
`http.auth.bearer_token.enabled` to `true`.

**(4) The failure mode, stated precisely:** under more than one replica with the flag flipped on,
a token issued by one pod's `InMemoryTokenAuthAdapter` verifies only against that pod's own
in-memory `HashMap` — it does not verify on another pod. An authenticated request then fails or
succeeds non-deterministically depending on which pod it lands on. This is a correctness question,
not a scaling optimisation, exactly as `.planning/REQUIREMENTS.md`'s WEB-02 entry states.

**(5) WEB-02's own manifest citation is wrong, and this record corrects it (D-08).** WEB-02 names
`k8s/deployment.yaml` and `k8s/service.yaml`; the Milestone 12 Epic 7 artefacts it actually
describes are `k8s/server/deployment.yaml` and `k8s/server/service.yaml`. The root
`k8s/deployment.yaml` is a distinct, older Milestone-1-era placeholder, re-read this session:
`:65` `image: paladin:test`, `:66` `imagePullPolicy: Never`, `:68` `args: ["-c", "echo 'Paladin
started' && sleep 3600"]`, and its liveness (`:139-148`), readiness (`:150-161`) and startup
(`:163-174`) probes all commented out with "Disabled for testing." Nothing in that manifest can
issue or verify a token, so it carries no correctness question for this record to fix. Plan 14-07
applies the dated correction banner to WEB-02's own text; this record does not edit
`k8s/deployment.yaml` and confirms it is unmodified (`git diff --exit-code -- k8s/deployment.yaml`
is clean).

## Decision

**D-06: the shared-store requirement attaches to the `AuthPort` credential path, not to the
replica count, and the replicas stay as shipped.** WEB-02's own "done when" text offers exactly
two exits — pin `k8s/server/deployment.yaml`'s replica count to one with the reason recorded, or
build a shared-store `AuthPort` implementation and prove a token issued against one instance
verifies against another. **This record takes neither, and the deviation is the decision, stated
here rather than left for a reader to infer.**

The reasoning for each declined exit: pinning `replicas: 1` would degrade a working two-replica
deployment to guard a code path the shipped ConfigMap has already turned off
(`bearer_token.enabled: false`) — a real cost (halved capacity, no rolling-update headroom) paid
for a hazard that is not live in the shipped configuration. Building the shared-store `AuthPort`
now is not rejected outright; it is deferred, under D-09 below, with a named trigger rather than
built speculatively.

What is delivered instead is what ROADMAP criterion 2's second clause actually asks for — "the
deployment artefacts and documentation say it will not [scale past one replica while the store is
wired]" — realised as an unconditional startup warning (D-07) plus three artefacts landed by
plan 14-04: an inline comment above `bearer_token:` in `k8s/server/configmap.yaml`, a scaling
note and a `kubectl scale` qualification in `k8s/README.md`, and an "Authentication &
authorization" paragraph in `docs/src/deployment-topologies/http-service-host.md` contrasting the
two credential paths for a reader choosing between them.

**Why the warning is unconditional on the store being wired, rather than conditional on an
observed replica count:** a running pod has no built-in way to learn `spec.replicas` without
calling out to the Kubernetes API, and adding a Kubernetes API client to make the warning smarter
was considered and rejected — it would add a runtime dependency and a failure mode (the API call
itself can fail) to make a warning marginally quieter in the one-replica case, where it is merely
redundant rather than wrong. A new opt-in refuse-to-start flag was also considered and rejected as
a knob whose only job is to be typed once by an operator who has already read the warning.

**D-09: the shared store is deferred with a named trigger, following the ADR-0035 precedent** — a
reintroduction condition promoted into a decision record without building the thing. **The
trigger is the first deployment that needs more than one replica serving `AuthPort`-issued
tokens.** Milestone 9 §6.2's own statement that the port was designed to permit exactly this swap
means the future work, when the trigger fires, is a new `AuthPort` adapter (SQLite via
`paladin-storage`, or Redis) — not a redesign of the seam. This is deliberately **not** recorded
as a permanent property of the topology, the treatment ADR-0039 gives a different question (the
absence of Garrison and Arsenal on HTTP-served agents, stated as never changing): the shared-store
capability is wanted, just not built yet, and nothing here forecloses it.

## Considered Options

- attach the requirement to the credential path, warn at startup, and document the limitation (chosen) — matches the shipped default (bearer-token verification off, static API keys on) and does not degrade a deployment to guard a disabled path
- pin the replica count to one (rejected — degrades a working two-replica deployment to guard a code path the shipped ConfigMap has already turned off)
- build the shared-store `AuthPort` now (deferred under D-09 with the named trigger, not rejected — the port was designed to permit exactly this swap, but nothing in the shipped configuration currently reaches the correctness edge it would fix)
- add an opt-in refuse-to-start flag when the store is wired at more than one replica (rejected — a knob whose only job is to be typed once by an operator who has already read the unconditional warning)
- make the startup warning conditional on an observed replica count (rejected — requires a Kubernetes API client a pod does not have by default, adding a dependency and a new failure mode to quiet a warning that is merely redundant, not wrong, in the one-replica case)

## Code Locations

- `src/bin/paladin-server.rs:148-151` — `IN_PROCESS_TOKEN_STORE_WARNING`, the unconditional constant naming the constraint and pointing at this ADR, re-verified this session
- `src/bin/paladin-server.rs:189-191` — the `if cfg.bearer_token.enabled` arm that fires the warning and constructs `InMemoryTokenAuthAdapter`, re-verified this session
- `src/bin/paladin-server.rs:321-343` — `build_auth_config_warns_when_in_process_token_store_is_wired`, the test proving the warning is actually emitted, re-verified this session
- `src/bin/paladin-server.rs:345-362` — `build_auth_config_fails_closed_when_enabled_with_no_credentials`, the test proving the fail-closed `Err` branch REQ-fail-closed-auth-posture describes, re-verified this session
- `k8s/server/configmap.yaml:39-45` — the inline comment above `bearer_token:` naming the constraint and this ADR, with `enabled: false` left unchanged, re-verified this session
- `k8s/server/deployment.yaml:14` — `replicas: 2`, unmodified by this record, re-verified this session
- `k8s/README.md:25-29` — the scaling note in the `paladin-server (HTTP API)` section pointing at this ADR, re-verified this session
- `docs/src/deployment-topologies/http-service-host.md:103-109` — the "Choosing a credential path for a multi-replica deployment" paragraph stating the limitation for a reader choosing a topology, re-verified this session
- `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — the sole `AuthPort` implementation whose scope this record fixes

## Code Conformance

conforms

The tree already satisfies this decision, landed by plan 14-04 with the exact test commands
recorded verbatim: `cargo test --bin paladin-server --features web-server
tests::build_auth_config_warns_when_in_process_token_store_is_wired` (pass — proves the warning
fires) and `cargo test --bin paladin-server --features web-server
tests::build_auth_config_fails_closed_when_enabled_with_no_credentials` (pass — proves the
fail-closed refusal branch). `cargo fmt --check` and `cargo clippy --all-targets --features
web-server -- -D warnings` both ran clean per `14-04-SUMMARY.md`. `git diff --exit-code --
k8s/server/deployment.yaml k8s/deployment.yaml` is clean, confirming neither manifest's replica
count was touched by landing this decision.

## Downstream Consumers

- **ADR-0040** — the token mechanism this record scopes to single-replica serving today.
- **The `REQ-k8s-manifests`, `REQ-opaque-bearer-token-adapter-v1` and
  `REQ-fail-closed-auth-posture` ledger rows** in `.planning/ledgers/milestone-09-12.md` — plan
  14-07 amends them against this record.
- **Any future phase that picks up the deferred shared store** — inherits the named trigger (the
  first deployment needing more than one replica serving `AuthPort`-issued tokens) and the
  ADR-0035-style promise that the future work is an adapter, not a redesign of the `AuthPort` seam.
- **Any operator enabling `http.auth.bearer_token.enabled`** — meets the unconditional startup
  warning and the same limitation stated in `k8s/server/configmap.yaml`, `k8s/README.md`, and
  `docs/src/deployment-topologies/http-service-host.md`.
