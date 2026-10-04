# launch-directory-k16

## Goal

A per-launch directory becomes the launch's identity:

- **Allocation.** For each launch the driver creates a fresh owner-only
  directory, `launch-<128-bit hex>`, in its workspace control area under
  `.jj/grove/`. After scrubbing the loop-control variables it inherited, it
  publishes the path as `GROVE_LAUNCH_DIR`.
- **Admission.** The session epoch binds this path where it bound the signal
  path. The `grove-llm` tree verbs admit a session only while its launch
  directory is the current one: the same comparison as before, with the
  control directory found from the path's parent.
- **Removal.** The driver removes the directory once it has read the launch. A
  replacement driver removes abandoned ones unread, after taking the lease and
  invalidating the old epoch.
- **`grove-llm record-teardown`.** Admitted under the epoch like any tree verb.
  It refuses while `.grove/` still exists, naming `finish-commit`. Otherwise it
  creates `teardown` in the launch directory, and a record already there is
  success. Run without a launch directory, it records nothing and says so.
- **The loop finishes on a teardown record**, whatever else the launch
  shows.

This is the expand step, so the old contract keeps working. The completion
channel (which may now sit inside the launch directory) and `grove-llm
complete` still end a session, and the loop finishes on a teardown record or on
`complete --done`.

## Context

- The spec's *The launch directory* and *Ending a session* under *Grove
  integration*. Decision 9's `record_teardown` and `Recorded`. The session
  epoch in `docs/adr/one-live-driver-per-working-tree.md`, including the
  rejected finish tombstone, which this record is not: it never outlives its
  launch and never crosses drivers.
- `harness-wrapper-k2`'s W10 (the directory) and W11 (the verb).
- The epoch record and admission live in `crates/grove-loop/src/driver_lease.rs`
  and its `observation` and `witnesses` submodules, keyed on the completion
  signal path today. The verbs are in `crates/grove-loop/src/verbs.rs` and
  `complete.rs`, and the CLI in `crates/grove-llm/src/cli.rs`.
- Suites: `crates/grove-loop/tests/driver_lease.rs`,
  `crates/grove/tests/loop_driver.rs`, `crates/grove/tests/env_hygiene.rs`, and
  the `grove-llm` tests.

## Done when

- Seam 3, staleness: a stale session's tree verbs and `record-teardown` refuse
  after the epoch rotates. Each launch directory is removed after its launch,
  and an abandoned one by a replacement driver, which reads nothing in it.
- `record-teardown` refuses while `.grove/` exists. A teardown record followed
  by the session's ending finishes the loop, and so does a teardown record
  followed by the harness exiting on its own.
- `.cargo/config.toml` clears `GROVE_LAUNCH_DIR` and
  `HARNESS_DISPATCH_EXIT_FILE` beside `GROVE_SIGNAL_FILE`, and its comment says
  why each is there.
- The `grove-loop` and `grove-llm` books follow (P2), and `## Unreleased`
  records `record-teardown` and the launch directory.
- `bash scripts/check.sh` passes.

## Notes

- `GROVE_LAUNCH_DIR` joins the loop-control variables every spawn scrubs. The
  worker never receives it: dispatch's worker environment is built fresh, and
  the usage documents warn against granting it (`current-state-docs`).
- Keep the launch directory's reading in one place. `dispatch-ending` adds the
  ending file to it and `signal-contract` removes the token.

## Implementation plan

1. Exercise the real driver/front/worker with fake harnesses under a PTY: directory allocation, epoch identity, teardown refusal/idempotence, both legacy signal and own-exit finishes, rotated-epoch refusal, and unread abandoned cleanup.
2. Add a Grove-owned launch-directory module for allocation, reading and cleanup. Bind epoch admission and observation to its path, retaining legacy completion compatibility during this expand step.
3. Expose `record-teardown` through the loop verb surface and thin CLI. Scrub inherited launch authority before granting the new directory to the lifecycle session.
4. Update affected source-exact walkthrough fragments and their prose, record Unreleased notes, then run `task check` before retirement and the jj commit.

## Decisions (running log)

- L1: Keep the compatibility channel inside the fresh launch directory. Centralize the launch's reading in the directory module so the migrate step can add the dispatch ending file without a second interpreter.
- L2: The approved launch-cutover design and scheduled `launch-cutover-k19` review govern this implementation; no competing in-session review is added. Code discovery uses exact source because the graph CLI refuses to start against its active incompatible generation.
- L3: Compatibility-only callers carrying the old channel derive the launch identity from its canonical parent directory; old epoch records remain readable during expansion. The normal path uses `GROVE_LAUNCH_DIR` directly, and completion additionally checks the inherited channel and its directory before writing.
- L4: A teardown record takes precedence over the harness ending and the legacy completion token; a driver interrupt still wins. Legacy group-survival handling remains in place for launches without a teardown record.

## Verification

- The new teardown PTY regression first failed on the missing launch directory, then passed with the implementation. The PTY cases cover teardown refusal and idempotence, both legacy completion and own-exit finish (including a nonzero harness exit), fresh private directories, epoch rotation and stale tree/teardown refusals, removal after interpretation, and unread abandoned cleanup using a FIFO and an external symlink target.
- The lifecycle launch fixture now captures both launch and dispatch exit authority, asserting that inherited values are replaced and an own-exit directory is removed. Its 16 tests pass; the launch boundary suite's 31 tests and lease suite's 23 tests also pass.
- `cargo test --locked --workspace` passes after updating the lifecycle fixture's old channel-parent expectation. `record_teardown` tests also cover manual invocation outside a workspace, existing root shapes, record permissions/idempotence, and refusal of links/directories at the record path.
- Final source-exact book validation passes for grove-loop (16 roots, 14,326 lines) and grove-llm (4 roots, 975 lines), with no deferred ranges. The first full check passed all six books and its other checks, but found the old lifecycle fixture expectation; the final full check follows the fixture correction.
- Final `task check` (`bash scripts/check.sh`) passed all 12 principal checks, including the complete workspace test suite and all six books. Before/after SHA-256 digests matched item by item for all 1,926 tracked project files (manifests, scripts, fixtures, source, documentation and task notes) throughout that run.
