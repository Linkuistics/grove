# launch-witnesses-k25


## Goal

Have real driver launches prepare, publish and release the two witnesses through
one lease-owned value, using launch-events-k24. Mandatory admission continues
to work when observational setup fails.



## Context

Start at `DriverLease`, `drive`, `launch_configured_session`, the shared
snapshot selector and `TreeLifetime`. Source is in grove-loop's driver_lease,
loop_driver and observation modules; existing process fixtures are in
`crates/grove-loop/tests/driver_lease.rs`. The current Selection is copyable
value data and `picked` releases its tree guard before returning: capture the
selected root under the same validated selection guard, rather than reopening
an unrelated root from that value later. Preserve finish materialization too.

The spec's Mandate and lifetime and Witness publication and release sections
are normative. Reuse the nonce, descriptor identity and epoch-acquisition seams
without changing session-admission authority. The node brief records the Linux
route, though witnessed-observation-k26 owns the complete two-platform evidence.

## Done when

- Pin/check the selected task root under its tree read guard. Move that pin
  and private witness into one DriverLease-owned value before publication and
  recheck the root before publishing; replacement before publication refuses
  that stale launch. No descriptor becomes part of copyable Selection. Neither
  a tree guard nor an epoch guard spans spawn.
- After exclusive predecessor-epoch invalidation, and only then, acquire each
  witness exclusively and nonblocking: the directory lock is on the selected
  pin's open description; the private file is regular, independently named by
  an OS-random 128-bit suffix, exclusively created with bounded collision retry.
  Both descriptors are close-on-exec. Foreign shared holders or reported lock
  errors release partial setup and fail observation promptly, preserving launch.
- An optional recognized epoch extension binds key/handle/kind, task-root and
  witness identities, namespace-local basename, lease nonce and signal path.
  The handle key agrees with the explicit key. Publish the recognized version
  only with both witnesses prepared. Admission validates mandatory fields
  independently of absent, malformed or unsupported observation extensions.
- Started writes only the exact eight-byte `started\n` marker to the initially
  empty file. Reaped releases the private witness before the locked directory
  pin, before terminal recovery, epoch handoff or signal interpretation. Failed
  spawn releases prepared resources before invalidation and publishes no marker.
  Completion signal, retirement, helper return and an unconfirmed-reap error
  cannot release either lease-owned member early; lease drop/unwind releases
  them in the same order before driver ownership.
- Allocation/publication failures emit diagnostics without changing authority
  or successful launch outcome. Mandatory epoch-write failure still prevents
  spawn. Cleanup follows exclusive epoch invalidation, including replacement
  cleanup; cleanup failure preserves the outcome. Viewing performs no cleanup.
- Real configured launches and the internal event/lock seam prove preparation,
  Started, failed spawn, immediate exit, signal-before-reap, normal/unwind drop,
  confirmed and unconfirmed reap, foreign holders of either witness, random
  collisions and exhausted retry, write/lock failures and release order. Event
  barriers prove a replacement awaiting old epoch readers acquires neither
  witness, while tree mutation/deletion can acquire the containing-directory
  guard during a launch. Existing stale-admission, rotation, already-admitted
  operation and root-replacement controls continue to pass.
- Until k26 supplies the reader, production observation remains Unavailable
  for active epochs, including the new extension. Verify this conservative
  handoff through `try_observe`; do not infer RUNNING from prepared bytes.
- Update shipped protocol/API descriptions and all affected grove-loop and
  keyed-launch walkthrough fragments/indexes/manifests in this change. Usage
  and G6 retain the witnessed-view deferral. Run focused loop/runner/process
  tests and `bash scripts/check.sh` after all edits.

## Notes

This leaf owns the writer and resource lifetime, not the observer's liveness
inference. Use one production ownership seam in fault/process tests. k26 adds
native macOS/Linux kill/reap and exec-survivor evidence through the actual
observer, plus forced-reuse mutations; do not preclaim those results here.
If a helper returns an error with reap unconfirmed, retaining its value on a
helper stack is insufficient. The lease must still own it after that return.
