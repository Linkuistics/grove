# cleanup-driver-acceptance-k127

**Kind:** impl

## Goal

Run validated orphan cleanup under the driver lease without allowing cleanup
bytes to influence lifecycle classification.

## Context

- Consume the quarantine and auxiliary cleanup seams from
  `quarantine-cleanup-core-k125` and `auxiliary-cleanup-markers-k126`.
- Configuration validation, driver-lease acquisition, epoch invalidation, and
  the ordinary absent-root contract already exist; preserve their interfaces
  and ordering.

## Done when

- Bare lifecycle cleanup runs only after complete configuration validation and
  lease acquisition, then ordinary witness recovery/root classification proceeds
  independently of cleanup outcomes.
- Exact matching in-tree owners suppress cleanup; corrupt or ambiguous owners
  leave candidates untouched with actionable warnings; old attempts with a
  reused handle remain distinguishable.
- Persistent cleanup failures are reported and retried by later owners without
  blocking a valid fresh-root launch or borrowing authority from old signals.
- Driver/process tests cover orphan quarantine and auxiliary cleanup, partial
  and persistent failure, owner ambiguity, reused handles, abandoned signals,
  and fresh-root behavior across plain Git, native jj, and colocated jj where
  applicable.

## Notes

Do not broaden the driver lease or session-epoch interfaces, and do not treat a
cleanup marker as a finish receipt.
