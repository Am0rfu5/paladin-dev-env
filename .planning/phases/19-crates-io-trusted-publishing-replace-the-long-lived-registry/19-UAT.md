---
status: testing
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
source: [19-VERIFICATION.md]
started: 2026-08-27T19:21:34Z
updated: 2026-08-27T19:21:34Z
---

## Current Test

number: 1
name: Confirm the crates.io token revocation on the registry side
expected: |
  In crates.io → Account Settings → API Tokens: the token named "Paladin" is absent
  from the active token list, and no other publish-scoped token exists on the account.
  (crates.io exposes no API for this — the phase's PUB-04 claim rests on this
  human check; the evidence log records the corroborating fields as "not reported
  by operator" until this test supplies them.)
awaiting: user response

## Tests

### 1. Confirm the crates.io token revocation on the registry side
expected: Token "Paladin" absent from the active API-token list; no other publish-scoped token present on the account. Optionally report the last-used timestamp and revocation time so the Revocation Ledger's "not reported" fields can be filled.
result: [pending]

## Summary

total: 1
passed: 0
issues: 0
pending: 1
skipped: 0
blocked: 0

## Gaps
