# session-signal-path-review-k60

**Kind:** review-impl
**Reviews:** session-signal-path-k57
**Producer launch:** {"producer":"session-signal-path-k57","session":"session-signal-path-k57","generation":"k57","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-signal-path-k57` and record concrete findings for its integration step.

## Context

- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-signal-path-integrate-k61` owns every fix and
  all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The review relies on the producer's recorded verification evidence; no test,
  build, lint, or format command is run.
- No production or test code is changed.

## Findings

Reviewed: commit `session-signal-path-k57: add fresh loop-control channels`
(`src/driver_lease.rs`, `src/loop_driver.rs`, `tests/env_hygiene.rs`,
`tests/loop_driver.rs`, `content/SKILL.md`) against
`docs/adr/one-live-driver-per-working-tree.md` and
`docs/specs/config-driven-sessions.md` § *Process ownership and session epochs*.

### F1 — medium — best-effort channel housekeeping is fatal, and can permanently wedge `grove` in a working tree

`src/loop_driver.rs:136-138` propagates the cleanup error before anything else
the loop does:

```rust
driver_lease
    .cleanup_abandoned_signal_channels()
    .context("cleaning signal channels abandoned by a previous driver")?;
```

`cleanup_abandoned_signal_channels_with` (`src/driver_lease.rs:203-218`) fails
the whole call on the *first* unremovable entry, so one undeletable
`signal-<32 hex>` name — a directory of that name, an `EACCES` on the control
dir, an immutable flag — makes every subsequent `grove` invocation in that
working tree exit before selection, migration, or launch, with a diagnostic
about *cleanup* that names nothing the operator is trying to do. Nothing else
is attempted: the loop never reaches the version-skew guard, `pick`, or the
foreground launch.

Nothing in the design gives this operation a correctness role. The spec
(`docs/specs/config-driven-sessions.md:412-418`) and the ADR
(`docs/adr/one-live-driver-per-working-tree.md:33-40`) describe it purely as
removing files "abandoned by crashed launches"; the freshness guarantee the
loop actually depends on comes from the independent 128-bit draw, not from the
directory being empty. A stray channel costs a few bytes and is inert by
construction — the current driver watches only the path it allocated.

Note also that the abort is *partial*: entries after the failing one are left
in place, so the fatal path does not even achieve the cleanup it refuses to
proceed without.

*Suggested shape*: warn on stderr and continue, and let the loop keep going;
if the failure must stay visible, collect the per-path errors and report them
all rather than stopping at the first.

### F2 — medium — a failed channel removal overrides the session's real disposition, including a clean finish

`src/loop_driver.rs:219-235`:

```rust
let signal = complete::read_signal(signal_channel.path());
driver_lease
    .remove_signal_channel(signal_channel)
    .context("removing the interpreted foreground-session signal channel")?;

if ended == SessionEnd::Interrupted { … return Ok(LoopOutcome::Stopped); }

match signal { Some(Disposition::Done) => { … return Ok(LoopOutcome::Finished); } … }
```

The `?` sits *upstream* of both the interruption branch and the disposition
match, so a removal failure discards an already-computed, already-correct
outcome:

- a session that ran the Complete finish cycle and signalled `--done` returns
  `Err` instead of `LoopOutcome::Finished`, and the human never sees
  `grove: grove finished — loop complete.` — the grove *is* finished, and the
  loop reports a failure;
- a Ctrl-C returns `Err` instead of `Stopped`, swallowing the
  `Re-run \`grove do\` … to resume` guidance the interrupt path exists to print.

The predecessor treated every one of these removals as best-effort
(`let _ = std::fs::remove_file(&signal_file);` at each exit — see the removed
lines in the diff), and a leaked channel is self-healing: the next driver's
`cleanup_abandoned_signal_channels` removes it (`src/driver_lease.rs:130-150`).
Same root cause as F1 — a housekeeping failure escalated to a control-flow
failure — but a distinct call site with a different blast radius, so it needs
its own fix.

*Suggested shape*: keep the read-then-remove ordering (it is the improvement
this slice wanted), but demote the removal error to a warning and let `ended` /
`signal` decide the return value.

### F3 — low — the retry-exhaustion branch of the signal allocator has no test

`Done when` #3 asks the injected seams to "deterministically cover occupied
draws, retries, cleanup, and old-signal isolation".
`an_occupied_signal_draw_is_retried_without_touching_the_old_channel`
(`src/driver_lease.rs:429-467`) covers exactly one retry; nothing drives
`SIGNAL_DRAW_RETRY_LIMIT` to exhaustion, so the `bail!` at
`src/driver_lease.rs:196-200` — *"could not allocate a fresh signal path after 8
occupied random draws in …"* — is never executed. That message is the one an
operator meets when the control directory is genuinely wedged, and its wording
and reachability are exactly what a test should pin.

The asymmetry is visible in the same file: the structurally identical lease
retry has both a success-after-retries test and
`lease_path_replacement_fails_closed_after_eight_attempts`
(`src/driver_lease.rs:383-405`). The seam already accepts an injected
`draw_nonce`, so the test is a three-line addition (feed nine occupied draws,
assert the error text and that the closure was called exactly
`SIGNAL_DRAW_RETRY_LIMIT` times).

### F4 — low — `remove_signal_channel`'s parent-directory guard is unreachable

`src/driver_lease.rs:115-124` refuses a channel whose parent is not
`self.control_dir`. That branch cannot be taken: `SignalChannel` is
`pub(crate)` with a private `path` field, it is constructed only inside
`allocate_signal_channel_with` as `control_dir.join(…)`
(`src/driver_lease.rs:191-193`), and the sole production caller
`DriverLease::allocate_signal_channel` (`src/driver_lease.rs:103-113`) always
passes `self.control_dir`. There is one lease per process, so no
cross-lease misuse is expressible either.

The result is untestable defensive code guarding an invariant the type system
already holds. Either drop it, or — if the intent is to keep the invariant
explicit as the epoch slice starts moving these calls around — record it as a
`debug_assert!` so it does not read as a live failure mode.

### F5 — low — `tests/env_hygiene.rs`'s new assertion under-covers what its comment claims

`tests/env_hygiene.rs:105-120` replaced "not under the system temp dir" with a
prefix check against `CARGO_MANIFEST_DIR/.git/grove` and
`CARGO_MANIFEST_DIR/.jj/grove`, under the comment *"outside **either**
workspace-administration control directory where this checkout's driver can
allocate a random channel"*.

Those are not the only two. Per the ADR
(`docs/adr/one-live-driver-per-working-tree.md:5-13`) and the spec
(`docs/specs/config-driven-sessions.md:346-358`), a linked Git worktree of this
checkout resolves its control directory through the `gitdir:` pointer to
`<repo>/.git/worktrees/<name>/grove`, and a secondary jj workspace to that
workspace's own `.jj/grove` — neither of which `starts_with` either tested
path. This repo's own suite exercises linked worktrees
(`tests/loop_driver.rs` `init_worktree` under `.grove-worktrees/`), so the case
is not hypothetical.

Worth flagging in the same pass: the guard is now close to vacuous. The
override is `target/grove-test-signal/inert.signal`, repo-relative
(`.cargo/config.toml`), and real channels are confined to VCS-administration
directories, so the two cannot collide by construction — where the pre-change
scheme put real signals in `$TMPDIR` and the override genuinely had somewhere
to go wrong. If the test is kept as a tripwire on `.cargo/config.toml`, say
that in the comment rather than implying a live hazard; if the claim is meant
literally, derive the control directory through the same resolver the driver
uses instead of hand-assembling two paths.

### F6 — low — no recorded verification evidence for `Done when` #5

`session-signal-path-k57`'s `Done when` requires `cargo fmt --check` and
`cargo test --locked` to pass "with the existing completion protocol still
green". The commit message is a single subject line, and the producer leaf's
`## Notes` is empty, so nothing records that either command was run or what it
reported. This review is inspection-only and may not run them, so that bullet
is unverified rather than verified-failing.

This is the grove's standing commit convention (`driver-lease-k31`,
`session-kind-migration-integrate-k29`, and the rest are all single-line), so
it is a gap in the convention, not a lapse by this producer.
`session-signal-path-integrate-k61` runs the suite anyway; it should confirm
both commands and record the result, since F1–F3 touch code paths the suite
exercises.

## Verified without finding

Checked and confirmed correct, recorded so integration need not re-derive them:

- **Codex sandbox reachability of the new path.** Moving the signal file from
  `$TMPDIR` into the VCS administration area puts it behind codex's
  `workspace-write` carve-out of `.git`. `append_codex_vcs_store_grant`
  (`src/launch.rs:220-241`) already covers every shape the control-dir resolver
  produces: a plain checkout (`--add-dir <worktree>/.git` ⊇ `.git/grove`), a
  linked worktree or submodule (`--add-dir <git common dir>` ⊇
  `.git/worktrees/<name>/grove` and `.git/modules/<name>/grove`), and both jj
  shapes (`.jj` is not carved out and the workspace root is the sandbox cwd).
  No new grant is required.
- **`grove-llm complete` still runs from any directory.** The exported path is
  built from the canonicalised `control_dir` (`src/driver_lease.rs:66-67,
  103-113`), so it is absolute; `content/SKILL.md`'s updated claim holds.
- **Old-signal isolation within one driver.**
  `loop_relaunches_on_signal_and_stops_without_one` asserts three distinct
  paths and that all three are absent afterwards, and iteration 3 stops without
  signalling — which is the in-driver isolation property.
- **Documentation reconciliation.** No stale "temp dir" claim about the signal
  file survives in `content/`, `docs/USAGE.md`, `docs/CONFIGURATION.md`, or
  `docs/ARCHITECTURE.md`; the `1/2^128` bound is recorded in the ADR
  (`:36-40`), the spec (`:414-418`), and `CONTEXT.md` (*Loop control channel*).
- **Seam discipline.** Both injection points are function parameters
  (`allocate_signal_channel_with`, `cleanup_abandoned_signal_channels_with`);
  no new environment variable was introduced, satisfying "without becoming
  supported environment configuration".
- **Ordering against the lease.** Allocation happens after
  `driver_lease.revalidate()` (`src/loop_driver.rs:201-207`), so lease loss
  still refuses the launch before a channel is drawn —
  `lease_path_loss_after_tree_selection_refuses_the_foreground_launch`
  (`tests/driver_lease.rs:657`) remains meaningful.
- **Sequenceability for the epoch slice.** `cleanup_abandoned_signal_channels`
  is a standalone `pub(crate)` method rather than being folded into
  `DriverLease::acquire`, so `13-session-epoch-chain-k34` can move it after
  exclusive invalidation as the producer's `Done when` requires.

## Notes

No test, build, lint, or format command was run; no production or test code was
changed.
