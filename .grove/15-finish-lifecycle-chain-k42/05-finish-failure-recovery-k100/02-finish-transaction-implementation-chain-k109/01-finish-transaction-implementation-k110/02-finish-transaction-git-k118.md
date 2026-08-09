# finish-transaction-git-k118

**Kind:** impl

## Goal

Make plain-Git finish teardown use the fail-closed transaction from witness
preparation through scoped deletion commit, rollback, and forward quarantine.

## Context

- Build on `finish-transaction-preflight-k117` and keep the transaction's
  external interface unchanged.
- Preserve unrelated staged and working-tree bytes. Disable user hooks for the
  internal commit and classify ambiguous results through the exact immediate
  handle-and-attempt-named `.grove/`-only result.

## Done when

- Ready witness evacuation is restart-safe and every unrelated tree command
  refuses it.
- Proven uncommitted outcomes restore the exact start and prior Git index;
  divergent topology leaves actionable `Recovery pending` state.
- Proven committed outcomes never resurrect the tree and atomically quarantine
  the whole task root before no-follow disposal.
- Plain-Git process and transition tests cover commit failure, lost result,
  hooks suppression, index restoration, topology races, and cleanup failure.

## Notes

Do not extend the implementation to jj or driver startup in this leaf.
