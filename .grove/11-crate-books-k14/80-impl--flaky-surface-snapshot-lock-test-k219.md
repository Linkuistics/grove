# flaky-surface-snapshot-lock-test-k219

## Goal

Establish whether `grove-loop`'s
`task_grow::tests::surface_scans_one_snapshot_under_a_shared_lock` is
intermittently red, and make it deterministic if so — or record why the one
observed failure was not the test's fault.

## Context

Observed once at `jj-workspace-brief-early-use-drift-k215`, in a `bash
scripts/check.sh` run, as the only failure in the workspace:

    test task_grow::tests::surface_scans_one_snapshot_under_a_shared_lock ... FAILED
    test result: FAILED. 245 passed; 1 failed; 0 ignored; 0 measured

**The evidence is thin and its thinness is the point.** It did not reproduce in
any subsequent run: once alone (`cargo test -p grove-loop --lib
surface_scans_one_snapshot_under_a_shared_lock`), three times as
`cargo test -p grove-loop --lib` (246 passed each), and four times as
`cargo test --workspace`, plus a second green `check.sh`. So one failure in
roughly seven full-suite runs, never reproduced.

**The assertion message was not captured**, which is the first thing to fix if it
recurs — the observing session filtered `cargo test` output to result lines and
lost the panic. Do not assume the failure mode from the test's name.

**k215's own changes cannot plausibly explain it.** That session touched three
Markdown documents and two `.grove/` task files and no Rust, and the test passed
seven times afterwards over the same tree. The name points at concurrency — a
shared lock and a single snapshot — which is the usual source of an
order-dependent or timing-dependent red.

## Done when

- The test's behaviour is characterised: either a reproduction (with the panic
  message) and a fix that makes it deterministic, or a recorded argument that the
  observed failure came from outside the test, with the evidence for that.
- `bash scripts/check.sh` passes.

## Notes

**A single unreproduced failure is not yet a defect**, and this leaf is entitled to
conclude that nothing is wrong — but not by re-running it a few more times and
finding green, which is what the observing session already did seven times. Read
what the test actually shares with its neighbours: `mod tests` in
`crates/grove-loop/src/task_grow/tests.rs` is excluded from the book corpus as an
inline test module, so it is not frozen and may be edited freely.

**Prior art in this tree.** A suite hang here has previously been a grandchild
process holding a piped stderr and wedging `wait_with_output`; the remedy was
draining by process group and using files plus `Drop` in test code. A timing red
under a shared lock is a different failure, but the same neighbourhood.

**Watch the instrument.** If a reproduction attempt runs the test in a loop,
finish every edit before measuring and confirm the run reported the full test
count — a build failure prints no per-test lines and reads exactly like a clean
result.
