# finish-witness-materialization-recovery-k146

**Kind:** impl

## Goal

Make finish-witness creation crash-consistent so every interruption before
evacuation has an automatically recoverable disposition.

## Context

- Surfaced during `finish-task-root-identity-integrate-k144`: delaying witness
  creation until after the final task-root identity gate transferred repository
  preparation ownership across a fallible filesystem boundary.
- The integration leaf can abort repository preparation on synchronous witness
  creation failures and can remove a witness it just created when cleanup is
  proven safe. It cannot make a process death between `create_dir`, manifest
  publication, and `READY` automatically recoverable without a durable protocol.
- A partial `FINISHING-*` directory cannot currently be classified by
  `pending_manifest`, while its attempt-bound Git/Jujutsu auxiliaries may still
  be the only restoration evidence.
- Coordinate with `finish-task-root-descriptor-relative-k140`: recovery must not
  follow a substituted task-root or witness path.

## Done when

- Every interruption point during witness creation has a durable, parseable
  state that identifies the finish handle and attempt identity.
- Restart distinguishes a not-yet-evacuated witness from an evacuated witness,
  safely aborts repository preparation for the former, and preserves recovery
  evidence when exact ownership cannot be proven.
- Partial or substituted witness objects fail closed without creating a
  same-attempt auxiliary collision that has no normal recovery route.
- Plain Git and colocated Jujutsu interruption tests cover each published state.

## Notes

Prefer an explicit preparing/ready state transition over inferring state from
which files happen to exist. Publication ordering and descriptor-relative
cleanup are part of the protocol, not test-only timing fixes.
