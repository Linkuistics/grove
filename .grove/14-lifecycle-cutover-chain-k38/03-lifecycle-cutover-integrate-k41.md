# lifecycle-cutover-integrate-k41

**Kind:** integrate-review-impl
**Integrates:** lifecycle-cutover-review-k40

## Goal

Apply the verified findings from `lifecycle-cutover-review-k40` while preserving the reviewed artifact's contract.

## Context

- Verify every `lifecycle-cutover-review-k40` finding against the binding flow.
- Preserve the direct configured-command model; do not repair a finding by
  reconstructing harness identity or hidden defaults.

## Done when

- Every finding has a recorded disposition; verified issues are fixed through
  the bare-process/fake-command seam.
- Bare `grove` is the sole active lifecycle implementation for all non-finish
  states, even though obsolete compatibility surfaces await deletion.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Finish materialization and teardown belong to `finish-lifecycle-k43`.

## Finding dispositions

- **F1 — verified and fixed.** Restored the hostile `core.worktree` fixture and
  replaced its obsolete foreground guarantee with the approved split: driver
  lifecycle VCS children scrub foreign selectors and Git anchors the leased
  worktree, while the opaque configured command inherits the caller's exact Git
  context. A legacy bare-migration acceptance test first reproduced the
  unanchored commit being redirected to the foreign repository.
- **F2 — verified and fixed.** Split the environment seam into a narrow
  configured-session scrub for stale Grove controls and a combined internal
  child scrub for Grove controls plus repository selectors. Unit and bare
  process tests pin both sets and the fresh signal-path replacement.
- **F3 — verified coverage gap; covered.** A linked-Git-worktree bare-process
  case observes exact `${repo}`, `${worktree}`, and `${session_name}` values
  through literal `env` word zero. Its assignment contains an unevaluated shell
  command substitution, proving Grove passes argv without a shell.
- **F4 — verified coverage gap; covered.** Bare `grove` now migrates and
  launches a legacy tree in both native and colocated jj fixtures.
- **F5 — valid trade-off; recorded.** Failure to spawn remains a nonzero driver
  error. Once a configured child has spawned, no-signal termination is a
  successful loop stop even for a nonzero child status; status, elapsed time,
  kind, word zero, and config path remain the operator-facing distinction.
- **F6 — noise; no production fix.** Signal allocation chooses an absent random
  pathname but does not create it. With no spawned child there is no process
  that can materialize the channel, so the alleged artifact cannot leak. The
  spawn-failure acceptance case now asserts that no `signal-*` file exists.
