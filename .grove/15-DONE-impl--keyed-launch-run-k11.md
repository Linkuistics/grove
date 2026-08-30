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

## Decisions (running log)

**The epoch stays in `src/driver_lease.rs`; only the channel and its nonce
leave.** `minimalism-k1` groups "signal channel + epoch + nonce" as the third of
`driver_lease.rs` that goes to the runner, but decision 7's interface — stated
verbatim as this leaf's context, and not to be redesigned — has no epoch in it,
and the machinery does not fit one: `admit_session` resolves a jj workspace,
compares the *working tree's* inode identity, and `flock`-probes the *lease*
file. Every one of those is one-driver-per-worktree, not one-launch-per-child. So
the channel's allocation, its `signal-<128-bit>` grammar, its abandoned-channel
cleanup and its nonce draw moved; the epoch record, its locking and admission
stayed, and travel to the loop crate at `loop-crate-driver-k22` with the lease
they belong to. `src/driver_lease.rs`'s own `random_nonce`/`hex_nonce` stay with
them, because the lease record's nonce is a different value from a channel name.

**Two fields decision 7's sketch does not have, both supplied rather than worked
around.** `Launch::cwd` — the driver spawns the session with the worktree root as
its cwd (`report_build_pairing`'s `resolve_in` comment depends on it), and
`Launch` had no way to say so; `Option<&Path>`, where `None` inherits the
launcher's. And `Channel::discard_abandoned(dir)` — a launcher that crashes
leaves channel files behind, and the only alternative to the crate recognising
them is a consumer open-coding `signal-<32 hex>`, which is a second spelling of a
rule one place should hold. Both are additions to what decision 7 enumerates, not
changes to it; `spec-to-current-state-k23` should carry them into the rewrite.

**SIGTERM/SIGHUP move into the crate, SIGINT does not.** `End::Interrupted` is a
promise decision 7 makes, so the handler and its latch have to be the crate's —
a caller cannot be relied on to have installed one, and two handlers on one
signal means whoever installed last wins. SIGINT stays grove's `ignore_interrupts`
because it is a *policy* about the human's Ctrl-C rather than a mechanism the
runner needs. The visible consequence is in the CHANGELOG: a SIGTERM arriving
between iterations now ends the driver instead of being latched onto the next
session.

**Not a cutover leaf, re-derived rather than inherited.** Nothing here is
tree-visible: `.grove/` is untouched, the filename grammar is untouched, and the
signal file's on-disk form is deliberately unchanged (`signal-<128-bit>` name,
`<token>\n` content), so the *installed* 19.3.0 driver meets exactly the tree and
the control directory it met before. No release, no reinstall, and this session
signals normally.

**`cargo clippy --all-targets` was never the gate the manifest thought it was.**
This root is also a package, so the documented command lints `grove` alone;
`crates/keyed-launch` had carried two `clippy::err_expect` errors since `k10`
under a workspace `deny` baseline nobody was running. Both fixed, and the
manifest now prescribes `--workspace`.

**The one in-session reviewer was spent on the supervision state machine**, and
found eleven things. Eight are fixed here: `End::Signalled` reported for a child
nothing touched; the interrupt latch swallowed between launches *and* spent on
the next child (the serious one — a fresh session SIGTERM'd on its first poll for
a signal that predated it); a `try_wait` failure returning with the child alive;
an interrupt restarting a kill grace already counting down; an empty channel file
read back as an empty token; the terminal restored only on the success path; the
`stty` spawn left unscrubbed; and a doc naming the wrong channel prefix. Two were
noise-adjacent contract restatements. Three — SIGINT's `SIG_IGN` inherited across
`exec`, the escalation signalling one pid rather than the process group, and a
SIGTERM'd driver exiting 0 — went to `child-signal-disposition-k31`, cut ahead of
`spec-to-current-state-k23`: none is a regression from this leaf, and each
changes what an interactive session experiences, which wants a terminal to verify
against rather than a unit test.

`crates/keyed-launch/tests/interrupt.rs` is the latch's proof and is its own
integration binary because the latch is process-global. It was confirmed to fail
without the fix (phase 3: *"a stale interrupt was charged to a child that was
never signalled"*) and pass with it.
