# finish-transaction-driver-recovery-k120 — brief

**Kind:** impl

## Goal

Recover finish witnesses and validated cleanup artifacts in the bare driver
without turning either into rootless lifecycle state.

## Context

- Recovery runs only after complete configuration validation, under the
  universal tree lock, and before normal format/liveness/missing-root handling.
- Pre-commit recovery exposes the restored finish leaf to a fresh HITL session;
  committed recovery completes handoff into the existing rootless/fresh-start
  contract.

## Done when

- Driver restart recovers or blocks every in-tree finish witness before
  selection and reports actionable topology diagnostics.
- Lease-owned cleanup reaps only artifacts with a valid Grove cleanup manifest
  and no matching in-tree owner; it never uses cleanup bytes as a finish
  receipt.
- Old-attempt cleanup and signals cannot authorize a replacement epoch or a
  reused handle.
- Driver/process tests cover pre-commit, committed, blocked, orphan cleanup,
  cleanup failure, and post-finish fresh-root behavior.

## Notes

Do not broaden the driver lease or session-epoch interfaces.
