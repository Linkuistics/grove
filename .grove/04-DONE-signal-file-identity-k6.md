# signal-file-identity-k6

**Kind:** work

## Goal

Key the loop's completion-signal file on the grove's *identity*, not just its
name, so one grove's signal can never end another grove's session.

## Context

Found by the adversarial review in review-k3 and **empirically reproduced**.
`signal_file_path` (`src/loop_driver.rs:71-83`) derives the path from the grove
**name only** — `$TMPDIR/grove-loop-<name>.signal`. `repo_path` is available at
the call site but is used solely for the session name, never for the path. Two
`grove do` runs in *different repos* whose worktree basenames collide — and
generic names like `bugs`, `plan`, `docs` are the norm — share one file.

This was benign before driver-side-kill-k2 and is not any more, which is why it
belongs to this grove: the file used to be *read* only after the harness had
already exited, so cross-talk could at worst produce a spurious relaunch
decision on a session that was over. The watcher now **polls it during the
session** and **kills on it**. Reproduced failure: a foreign write lands → the
innocent session is SIGTERM'd mid-work → `read_signal` returns `Relaunch` → the
loop relaunches → killed again → forever. The victim session never signalled
anything.

A related, lower-severity case folds into the same fix: on Linux `temp_dir()`
is the shared sticky `/tmp`, so another *user's* stale
`grove-loop-<name>.signal` cannot be removed by the ignored `remove_file` at
the top of each iteration. The watcher then sees it on the first poll of every
iteration and kills each session after `grace`, permanently. Not reachable on
macOS (per-user `$TMPDIR`), so it is invisible on the trial's own machine.

## Done when

Two `grove do` loops running concurrently in different repos with the same
grove name cannot interfere: neither kills nor relaunches the other, covered by
a regression test. The identity scheme is decided and recorded — a hash of the
canonicalised repo/worktree path folded into the filename, or moving the file
under the worktree — noting that the file is ephemeral loop IPC and must not
become durable grove state (constraint 1) nor land in git.

## Notes

Same test seam as driver-side-kill-k2: the fake-harness `GROVE_HARNESS_BIN`
script driving the real `run_loop`, with `GROVE_KILL_GRACE` /
`GROVE_KILL_GRACE_KILL` keeping it fast.
