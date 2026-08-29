# keyed-launch-run-k11

## Goal

Move the launch half into `keyed-launch`: the out-of-band completion channel,
direct argv spawn with no shell, child supervision, and kill escalation.

## Context

`docs/specs/module-decomposition.md`, decision 7 — the `Channel`, `Token`,
`signal`, `Escalation`, `Launch`, `run`, `Ended`, `End` and `LaunchError` half of
the interface, stated verbatim.

`minimalism-k1` splits `src/driver_lease.rs` (1,737 lines) three ways: **signal
channel + epoch + nonce (~400) to the runner**; the one-driver-per-worktree lease
(~900) to the loop; and the same-device machinery (~200), already deleted by
`delete-finish-transaction-k8`. The spawn and escalation live in `src/launch.rs`
and `src/loop_driver.rs`.

## Done when

- `Channel::{allocate, path, read, discard}` and free `signal(path, token)` are
  in the crate: a fresh, collision-resistant path per launch, naming that launch
  alone. `Token` is **opaque to the runner** — its appearance ends the launch,
  its content is the caller's to interpret, which is why `as_str` and
  `into_string` exist.
- `run(Launch)` spawns the expanded argv **directly, with no shell**. The child's
  environment is the caller's, minus the scrubbed control values, plus the fresh
  channel path under the caller's chosen variable name.
- Escalation runs grace → SIGTERM → kill-grace → SIGKILL, because a child that
  returns to an interactive prompt is never reaped on its own. Preserve that
  reasoning in the code, not only the timings.
- The crate's tests drive a fake child end to end without grove — test seam 1.
- Grove's driver consumes the crate; `src/launch.rs` and the signal/epoch/nonce
  third of `src/driver_lease.rs` are gone from `src/`.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** Depends on `keyed-launch-templates-k10` only for the crate to
exist and for `Argv` to be the thing a launch takes — nothing else.

**`Argv` is built only by expansion.** That is a contract, not an implementation
detail: nothing reaches a spawn that a template did not author. If the run half
needs a constructor for its own tests, it is a test-only one.

**Keep grove's vocabulary out.** The same discipline as k10 — *key*, *template*,
*launch*, *child*, *signal*, *escalation*, and never *session*.
