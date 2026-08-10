# atomic-colocated-index-rebind-k147 — brief

**Kind:** impl

## Goal

Make colocated-Jujutsu success-index marker rebinding crash-consistent, so an
interruption cannot strand an unmarked or stale-identity same-attempt artifact.

## Context

- Surfaced while reviewing `finish-witness-materialization-recovery-k146`.
- `GitIndexBackup::prepare_without_grove` publishes the success-index
  auxiliary, replaces its artifact while removing `.grove`, then calls
  `AuxiliaryCleanup::rebind_artifact_identity`.
- Rebinding currently removes the old marker before publishing the new marker.
  Process death in that interval can leave the deterministic auxiliary artifact
  without parseable ownership; failure before rebinding can leave a marker that
  names the artifact's previous identity.
- The preparing finish witness can preserve the handle and attempt, but normal
  recovery still needs an atomic or explicitly recoverable auxiliary state.
- Adversarial review showed that a boolean rebind intent is insufficient: it
  cannot distinguish Git's replacement artifact from an external file, an
  unconditional marker rename can overwrite a last-moment substitution, and
  cleanup cannot start while the index-filtering child may still publish.
- Deliver the transaction as four ordered, independently testable slices:
  quiesce the filtering child on every return path; make marker replacement
  explicitly recoverable; bind the intended replacement before adopting it;
  then wire and exercise the complete colocated-Jujutsu recovery matrix.

## Done when

- Every interruption boundary while changing a finish auxiliary artifact's
  identity has durable, parseable same-attempt ownership.
- Recovery can either validate and dispose the prepared success index or restore
  the exact backup without treating Grove-created intermediate state as foreign.
- Substituted artifacts and markers still fail closed without deleting external
  bytes.
- Colocated-Jujutsu process tests exercise the marker-rebind interruption
  boundaries and prove a same-attempt retry has no collision.

## Notes

Keep this concern in the auxiliary publication protocol; do not special-case an
unmarked deterministic filename in finish-witness recovery.
