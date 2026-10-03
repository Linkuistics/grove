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
