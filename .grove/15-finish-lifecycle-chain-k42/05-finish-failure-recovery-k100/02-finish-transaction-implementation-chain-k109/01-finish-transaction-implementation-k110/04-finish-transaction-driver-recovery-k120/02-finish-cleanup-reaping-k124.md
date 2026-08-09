# finish-cleanup-reaping-k124

**Kind:** impl

## Goal

Make lease-owned finish cleanup retryable without letting quarantine or
auxiliary bytes classify lifecycle state.

## Context

- Cleanup runs only after configuration validation and lease acquisition; an
  absent task root still follows the ordinary fresh-root contract.
- Quarantines and Git-index auxiliaries need attempt-keyed Grove cleanup
  markers and must never borrow authority from old signals or reused handles.

## Done when

- The driver reaps only quarantines and auxiliaries carrying a valid Grove
  cleanup marker and having no matching in-tree owner; ambiguous owners remain
  untouched with an actionable warning.
- Interruption or partial recursive deletion retains independently validated
  evidence so later lease owners can finish cleanup safely.
- Validation and deletion stay bound to the same no-follow filesystem object;
  cleanup failure never deletes a replacement path.
- Old-attempt cleanup and signals cannot authorize a replacement epoch or a
  reused handle, and cleanup bytes never act as a finish receipt.
- Driver/process tests cover orphan quarantine/auxiliary cleanup, partial and
  persistent cleanup failure, owner ambiguity, reused handles, and fresh-root
  behavior.

## Notes

Do not broaden the driver lease or session-epoch interfaces.
