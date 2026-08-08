# session-epoch-review-k36

**Kind:** review-impl
**Reviews:** session-epoch-k35
**Producer launch:** {"producer":"session-epoch-k35","session":"session-epoch-k35","generation":"k35","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-epoch-k35` and record concrete findings for its integration step.

## Context

- Review `session-epoch-k35` as a concurrency protocol against both cited ADRs
  and the spec's explicit lock order.
- Attack self-deadlock, open/lock/path replacement races, lease-transfer versus
  shared admission, guard leakage across exec, orphaned descendants, signal
  reuse/cleanup timing, wrong-worktree aliases, and accidental authentication
  claims.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-epoch-integrate-k37` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity and a concrete interleaving/event
  trace or black-box reproducer, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded race and orphan-timeout evidence rather than re-running it.
- No production or test code is changed.

## Findings

Inspected: commit `session-epoch-k35: bind operations to live driver epochs`
(1268+/43-, 10 files); `src/driver_lease.rs` in full; `src/loop_driver.rs`
:129-360 and :1153-1290; `src/llm_cli.rs` :263-456 and :790-868;
`src/complete.rs` :78-90; `src/repo.rs` :33-196; `src/tree_access.rs` :120-180;
`.cargo/config.toml`; `docs/adr/one-live-driver-per-working-tree.md`;
`docs/specs/config-driven-sessions.md` §"Process ownership and session epochs"
(:339-512) and §"Test seams" (:785-875).

The producer recorded no verification prose — its commit subject is a single
line with no body — so its evidence is the committed suite itself: the
race/retry traces `epoch_acquisition_retries_open_lock_path_replacement_in_event_order`
(`src/driver_lease.rs:1115`), `lease_path_replacement_retries_until_the_locked_descriptor_is_current`
(:853) and `lease_path_replacement_fails_closed_after_eight_attempts` (:878);
the orphan bound `an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound`
(:1156); the handoff pair `an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`
(:1205) and `replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff`
(:1263); and the black-box `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`
(`tests/driver_lease.rs:750`), `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`
(:844) and `the_driver_activates_immediately_before_spawn_and_invalidates_after_reap`
(`tests/loop_driver.rs:96`). None of it was re-run; no production or test code
was changed by this review.

### F1 — medium — the liveness probe holds the lease exclusively across its validation, so a concurrent driver start is falsely refused

`probe_live_lease` (`src/driver_lease.rs:677-725`) takes `LOCK_EX|LOCK_NB` on
the lease at :688 and then, still holding it, does `fstat` (:690), `stat`
(:695), an identity comparison that may `continue` the retry loop (:699-708),
and a record comparison (:709-711) before reaching the `probe == 0` arm at :712.
There is no `LOCK_UN`; the lock is released only when `lease_file` drops at
function exit. The ADR requires the opposite — "A successful probe is closed and
released immediately" (`one-live-driver-per-working-tree.md`:52-53) — and the
repo already has the correct idiom in `tree_access::assert_guard_held`
(`src/tree_access.rs:166-170`), which probes and unconditionally `LOCK_UN`s
before doing anything else (test-only, but the same shape).

Interleaving:

- T0 — driver A owns the lease; session S is active; epoch is `state=active`
  with signal path `P_A`.
- T1 — A is SIGKILLed. The kernel releases the lease. The epoch record still
  reads active/`P_A`; S survives as an orphan.
- T2 — S runs `grove-llm pick`. `admit_ambient_session` takes the shared epoch
  guard, validates worktree + identity + `P_A`, and enters `probe_live_lease`.
  `flock(LOCK_EX|LOCK_NB)` at :688 **succeeds** — the orphan now holds the
  lease exclusively.
- T3 — the human runs `grove` in the same worktree. `acquire_lease_file_with_hook`
  (:505) calls `lock_exclusively_nonblocking` (:540), gets `EWOULDBLOCK`, and
  bails: *"another Grove driver already owns <worktree>; the existing Grove
  driver must stop before this one can start"* (:551). No driver exists; the
  remediation the message names is impossible to perform.
- T4 — S's probe finishes and its descriptor drops, releasing the lock. A retry
  by the human then succeeds.

The window is short (two `stat`s), so this is low-probability per attempt — but
it is up to eight acquire/release cycles when the identity check retries, it
recurs for every ambient command the orphan issues, and the failure mode is a
hard refusal carrying a factually wrong diagnostic in precisely the
crash-handoff scenario this slice exists to make safe. It also contradicts a
stated ADR sentence, so the doc and the code cannot both stand.

### F2 — medium — nothing exercises the loop's orphan `blocked` stop; the guarantee lives in three untested lines

`src/loop_driver.rs:204-215` is where "an orphan makes the loop stop instead of
parking or relaunching" is actually implemented: `launch_session`'s result is
held, `invalidate_session_epoch()` is `?`-propagated with the context
`"post-reap session epoch invalidation blocked; completion signal left
unconsumed"`, and only then is `ended?` applied. The `?` on the invalidation is
the whole guarantee — it is what stops `complete::read_signal` (:221) from
running and what prevents `continue` (:241) on a stale `Relaunch`.

`an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound`
(`src/driver_lease.rs:1156`) proves only that `acquire_epoch_file_with` bounds
and reports; it calls the primitive directly with an injected clock and never
touches `loop_driver`. Grepping `tests/` for `blocked`, `orphan`, `unconsumed`
and the contention string returns nothing relevant. The spec names this as a
required black-box outcome twice — "an orphaned tree command outliving its
SIGKILLed foreground parent causing a bounded stop rather than parking or
relaunching the loop" and "no tree/launch overlap, and a bounded stop"
(`config-driven-sessions.md`:836-847) — and the producer's own `Done when`
claims "tests cover … orphan timeout … and no operation/launch overlap".

Black-box reproducer the integration step should add: fake harness writes a
`Relaunch` signal, then spawns a detached child that holds a shared `flock` on
`<control>/session.epoch` and sleeps past the bound, then exits. With an
injected short bound the loop must exit non-zero naming the invalidation, must
not relaunch, and the signal file must still exist. Regression it catches:
downgrading the `?` at :212-214 to a warning — the loop would then consume the
stale signal and relaunch, and every existing test would still pass.

Also note the production contention message at `src/driver_lease.rs:410-415` is
reachable only through `acquire_epoch_file` — every unit test injects its own
`report_contention` closure — so that exact string, which the spec requires to
name mode and operation, is never asserted.

### F3 — low — an invalidation failure discards the launch error that caused it

At `src/loop_driver.rs:204-215` the launch result is bound but not inspected
until after `invalidate_session_epoch()?`. When both fail, the epoch error wins
and the launch error is dropped entirely.

Reproducer: configure a harness binary that does not exist, and have the
previous session leave an orphan holding a shared epoch guard. `cmd.spawn()`
fails with "launching the harness session"; `invalidate_session_epoch()` then
waits the full bound and fails; the operator is told the epoch handoff was
blocked and never learns the binary is missing. Related but smaller: when
`launch_session` fails *before* `activate_session_epoch` (:355-357) — e.g.
`append_codex_vcs_store_grant` at :323 errors — the driver still pays a full
epoch acquisition to write an `inactive` record over an already-`inactive` one.

### F4 — low — `complete` now hard-requires a resolvable working tree at the cwd

`admit_ambient_session` calls `repo::workspace_control(path)` (`src/driver_lease.rs:742`)
before it opens any control file, and `llm_cli::run` applies it to every verb
including `Complete` (`src/llm_cli.rs:417-418, 438`). With `GROVE_SIGNAL_FILE`
set — always true in-session — a cwd outside any git/jj marker fails with
`repo.rs:109`'s "not in a git or jj working tree (path: …)".

Reproducer: `cd /tmp && grove-llm complete --done` from inside a live session.
Previously `complete` read one env var and wrote one file, touching no
repository. Two consequences for the integration step to settle deliberately:
the diagnostic is neither of the two the spec names for refusal ("wrong working
tree" / "stale Grove session", `config-driven-sessions.md`:435-440), and the
grove skill still tells the agent the finish cycle's `complete --done` "writes
only the loop's signal file (in the temp dir) … so run it from any valid
directory" — a sentence already stale on the temp-dir half after
`session-signal-path-k57`, and now stale on the directory half too. Either
resolve the epoch path from `current_control.control_dir()` only when the cwd
resolves, or state the new requirement in the durable docs.

### F5 — low — an inactive epoch and a rotated epoch are indistinguishable in the diagnostic

`src/driver_lease.rs:773-777` reports "loop-control path does not match the
active epoch" whenever `epoch.signal_path != Some(signal_path)`. That branch is
taken both when the epoch is `state=inactive` (`read_epoch_record` maps inactive
to `None`, :659-666) and when it is active for a *different* launch.

Reproducer: `a_rotated_epoch_refuses_the_old_signal_path` (:1329) and the
between-sessions case both land on the same string, but they mean different
things — "the loop has rotated past you" versus "no session is running at all,
you are talking to a driver between launches". The spec distinguishes them
("missing, inactive, unlocked, malformed, or mismatched", :437-438); the
message asserts an active epoch that may not exist.

### Checked and clear

Recorded so `session-epoch-integrate-k37` does not re-derive them:

- **No self-deadlock.** Every exclusive epoch guard is a distinct lexical scope
  (`write_epoch_record` :259-269, `initialize_epoch_record` :271-291), and
  `acquire_epoch_file_with`'s retry loop drops its descriptor at each iteration
  end including on `continue` (:436-496). `DriverLease` opens the worktree root
  but never `flock`s it (:134-145), so the driver cannot collide with
  `tree_access`'s root lock.
- **Lock order holds.** `admit_ambient_session` runs before the match in
  `llm_cli::run` (:418), so the shared epoch guard is always taken before the
  Tree access lock; no process ever holds the tree lock while waiting on an
  epoch guard. The driver's only tree reads are the `grove-llm` subprocess at
  `loop_driver.rs:1167`, which holds no epoch guard at all.
- **The pre-flock lease read is not a torn read.** `probe_live_lease` reads the
  lease record before locking it (:686-687), but the only lease-record writer is
  `initialize_epoch_record` (:285), which runs under the *exclusive* epoch
  guard while every probe runs under a *shared* one. The two are mutually
  exclusive.
- **Crash mid-`initialize_epoch_record` is self-healing.** Epoch is written
  before the lease record (:278-290), so a crash between them leaves epoch nonce
  B / lease nonce A. Old sessions fail on the inactive epoch's absent signal
  path; the next driver rewrites both.
- **Guards do not cross exec.** `ensure_close_on_exec` covers the worktree
  descriptor (:140), lease (:513), epoch (:445) and probe (:685). The epoch
  guard is released before spawn in every path.
- **Cleanup ordering matches the ADR.** `cleanup_abandoned_signal_channels` runs
  after `initialize_epoch_record` (:163-168); an ambient `complete` writes its
  signal *while holding* the shared guard (`llm_cli.rs:438-455`), so a
  replacement driver cannot clean a channel out from under an admitted call.
- **Empty `GROVE_SIGNAL_FILE` is inert on both axes.** The only two readers
  filter it (`driver_lease.rs:736-741`, `complete.rs:81-85`); `CompleteArgs`
  carries no clap `env` attribute (`llm_cli.rs:292-296`) that would bypass the
  filter. `--version` is exempt structurally — clap short-circuits in
  `Cli::parse()` before `admit_ambient_session` — and
  `tests/driver_lease.rs:750` pins it.
- **Nested-grove leakage is closed.** `loop_driver.rs:1185` now scrubs
  loop-control env from the driver's own `grove-llm` peek; that is the only
  `grove_llm_bin()` spawn site, and `agent_grove_llm_version` (:1325) only runs
  the exempt `--version`.
- **No accidental authentication claim.** Diagnostics stay descriptive ("stale
  Grove session", "wrong working tree"); nothing in the module asserts identity
  beyond cooperative workflow consistency. The forged/stripped-context case is
  explicitly out of scope (`config-driven-sessions.md`:886-890) and the code
  does not pretend otherwise.

## Notes

The reviewer produces findings only; `session-epoch-integrate-k37` owns fixes.

F1 and F5 are code-or-doc choices, not pure bugs: F1 contradicts a written ADR
sentence, F5 contradicts a written spec distinction. Either side may move, but
they cannot both stay as they are. F2 is the one finding whose absence would
silently survive a future refactor.
