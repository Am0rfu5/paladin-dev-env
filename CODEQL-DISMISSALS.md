# CodeQL Alert Dismissal Register

This file is the authoritative **governance** register for every CodeQL code-scanning alert this
Paladin workspace dismisses. It exists because a code-scanning dismissal in GitHub's UI is a
free-text reason string attached to the alert — it carries no named owner, no review date, no
scope and no compensating control, and none of it is queryable outside the platform itself. The
four fields a governed dismissal needs can only live somewhere a script can read them structurally.
This register gives those fields a structured home, modelled directly on `SECURITY-EXCEPTIONS.md`
(D-17), which already proved this register-plus-guard shape for advisory suppressions
(ADR-0036).

**The register is the governance surface; GitHub's alert store is the enforcement surface.** They
are reconciled by a named manual command, not by the guard below — every guard script in this
repository is offline by design, and this one is no exception. Run this to compare the register
against the live dismissed-alert set on GitHub:

```bash
gh api "/repos/DF3NDR/paladin-dev-env/code-scanning/alerts?state=dismissed&per_page=100" --paginate
```

Running that command is a human step. The guard below does not perform it, does not make any
network call, and does not claim to know what GitHub's live alert store currently holds — it
validates only that this file is internally consistent and not stale.

**Adoption context.** Per `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`'s
`## Verdict`, CodeQL (`2.26.3` / `rust-queries` `0.1.40`) is disqualified from promotion to a
required check — `.github/workflows/codeql.yml` runs advisory-only, on every push/PR/schedule,
never gating a merge. That status does not make a dismissed alert's governance optional: the one
class CodeQL's Rust query suite fires reliably on (`rust/hard-coded-cryptographic-value`) still
produces alerts a human has to triage, and an ungoverned dismissal of one of those alerts is the
same assurance-theatre failure mode this register exists to prevent regardless of whether the scan
that raised it can block a merge.

**Schema contract, stated plainly:** dismissing an alert on the platform without adding a row here
is a governance failure the guard cannot see, so it is stated as a rule and checked at
reconciliation. Adding a row here with any empty field, a past review date, or a duplicate alert
number fails CI.

## The 1 live dismissal

Declared dismissals: 1

<!-- BEGIN MACHINE-READABLE REGISTER -->
```toml
[[dismissal]]
alert_number = 28
rule_id = "rust/hard-coded-cryptographic-value"
path = "src/core/platform/manager/user_service.rs:1582"
why_present = "The rule's heuristic password-parameter sink fires on the literal \"any-password\" passed as the first argument to service.verify_password(\"any-password\", \"not-a-valid-phc-hash\") at user_service.rs:1582, inside #[tokio::test] async fn verify_password_against_a_malformed_hash_returns_a_hash_error(), itself inside the #[cfg(test)] mod tests block opening at line 491."
why_dismissed = "Direct source inspection confirms this is test-fixture data exercising the malformed-hash error path, not a leaked production credential. The literal carries no cryptographic sensitivity, is never logged, and is not derived from or usable against any real credential store -- it is an arbitrary string chosen only to be a syntactically-valid password argument."
dismissed_reason = "used in tests"
owner = "DF3NDR"
review_date = "2026-12-31"
scope = "test-only code path -- the #[cfg(test)] mod tests block in crates/paladin-core's user_service.rs; never compiled into a release binary"
compensating_control = "The flagged literal never leaves the #[cfg(test)] compilation unit: it is not read from configuration, not passed to any crypto API outside this test's own assertion, and not reachable from any code path a release build includes."
revisit_condition = "the rust-queries heuristic changes its password-parameter sink matching such that this test literal stops flagging on its own, or the test is refactored to use a non-literal placeholder value -- whichever comes first"
```
<!-- END MACHINE-READABLE REGISTER -->
