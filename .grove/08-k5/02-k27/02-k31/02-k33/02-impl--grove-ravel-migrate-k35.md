# grove-ravel-migrate-k35


## Goal
Complete the remaining atomic lifecycle and standalone migration to installed
Ravel's public start-session/end-session boundary.



## Context
Read the inherited k33/k31/k27/k5 contracts and their current handoffs. k34
expanded the authority guards only; both consumers still use the old locator,
argv, JSON readers and prompt. No runtime/ending contract was switched.

Consume k26's PATH/version resolver, k30's owned stdout runner entry points and
k32's SessionResult shared held-file reader. The frozen result is exactly
grove-continue. Interpret only after reap and lifecycle epoch invalidation;
preserve teardown precedence, cancellation and survivor checks.

Ravel 0.1.0 is installed at /opt/homebrew/bin/ravel. Its source CLI contract is
/Users/antony/Development/Ravel/docs/session-cli.md. Its actual task-file/task-id,
exit-dir/runtime-read flags are public; do not use a worker report protocol.

k34 leaves `task test:authority`, lifecycle/standalone real-policy refusal
probes and the expanded nine-name Cargo/helper/production scrub sets. Adapt
those fixtures to the active Ravel boundary without dropping dirty controls.
Build prerequisites require Bun 1.4.2; the existing verified binary is at
/Users/antony/Development/Ravel/target/tools/bun, while host PATH has 1.4.3.

## Done when
- Every remaining inherited switch condition holds: both actual consumers
  accept only the shared grove-continue result, reject missing/unsupported
  results, select/version-check/canonicalize Ravel once, and use that exact
  executable for launch, ending instructions and public runtime reads.
- Remove staged resolver dead-code expectations and the obsolete sibling
  locator. Migrate prompts/help/finish/Creator compatibility instructions,
  standalone/tree prose and all required authority seams together. k28 owns
  generic adapter/provenance assets; k29 owns the final complete surface audit.
- Real installed Ravel/worker fixtures exercise both actual readers/launches,
  separate versioned prefixes, stream isolation and the specified decoys.
  Existing continuation/completion/refusal/resume, cancellation, group/PTY,
  epoch and stale-context cases pass under the new consumer contract.
- Update every changed source book, manifests/indexes/rollups and required
  release-helper assertions. Preserve bundled delivery until k6. Use nextest
  with visible reporting, saved complete logs and matching input digests.
- Reconcile k33/k31/k27 Done when at retirement; close only what the actual
  migrated subtree delivers. Never install Grove or alter this session's
  original v22 harness-dispatch exit ending/admission/teardown contract.

## Notes
