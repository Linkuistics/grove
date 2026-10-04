# launch-cutover-k19

**Reviews:** launch-cutover-k15
**Creator:** run e81ed5ab-1c47-42e6-9fe5-226f17dc4cc2

## Goal

Adversarially read the implementation that `launch-cutover-k15`'s three leaves
produced against the spec, the root brief's guarantees and the methodology it
now states, and report findings. Fix nothing.

## Context

- The artifact is the commits of `launch-directory-k16`, `dispatch-ending-k17`
  and `signal-contract-k18`. Each commit message names its handle.
- The contract: the spec's *Grove integration*, decisions 7 and 9 of
  `docs/specs/module-decomposition.md`, and
  `docs/adr/one-live-driver-per-working-tree.md`.
- Where the producer was least certain:
  - the stale-session guarantee now that admission keys on the launch
    directory, including a stale session's `record-teardown` and exit signal
    against a later launch;
  - the reading table's precedence: a teardown record finishes whatever the
    ending, and only `exit_signal` relaunches;
  - a replacement driver removing abandoned launch directories while reading
    nothing in them;
  - the terminal after dispatch's death, and a driver in the background;
  - the 10 s bound against dispatch's end;
  - whether every ending the prompt and the shipped skills now describe is one
    the driver actually reads that way, the finish's two commands above all.

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If any finding warrants action, an `integrate-review-impl` leaf with this
  stem is `leaf-insert`ed before the next root-level sibling with live work,
  so the documentation and the release describe an agreed cutover.

## Findings

Reviewed `launch-directory-k16` (`e905a282`), `dispatch-ending-k17` (`910ea232`)
and `signal-contract-k18` (`291364c7`) as the source stands after them, against
the spec's *Grove integration* and *Supervision*, decisions 7 and 9 of
`docs/specs/module-decomposition.md`, and
`docs/adr/one-live-driver-per-working-tree.md`. Nothing was built or run; the
evidence is the source, the tests' bodies, the three leaves' recorded
verification and `rg` sweeps over the tree. **No finding breaks the stale-session
guarantee, the reading table's precedence in code, abandoned-directory cleanup,
the terminal recovery or the 10 s bound** (see "What held"). F1 is the one
finding that can leave a session unable to end as the prompt tells it to; F2–F6
are untested precedence, a dead compatibility branch, and two statements that
overreach. F1 is **CONFIRMED** (source); F2–F6 are CONFIRMED against the source
too, and none needs a reproduction.

### F1 — P2 (CONFIRMED): a shipped skill still tells a session to end with the retired `grove-llm complete`

**Location:** `plugins/linkuistics/skills/doubt-driven-development/SKILL.md:145-146`
(the instruction wraps a line: `hand back with` `` `grove-llm `` / `` complete` ``);
`docs/specs/doubt-grove-review-mechanics.md:238` (*Producer handoff*, step 4 of the
spec that skill implements). The test that should have caught it is
`crates/grove-llm/tests/instructed_verbs.rs:30-31`, whose corpus is
`plugins/grove/skills` alone.

`k18`'s done-when was that no *shipped skill*, conformance row or Codex-provisioned
skill names `grove-llm complete`. Its sweep recorded its scope as "`crates/`,
`plugins/grove/`, `testing/` and `.cargo/`" — the path narrowing
`references/execute.md` names — and so never reached `plugins/linkuistics`, a plugin
this repository ships. `doubt-driven-development` is `harnesses: [any]`, so
`crates/grove/build.rs` also bundles it into what `grove` provisions for Codex. The
instruction is live on the path a picked leaf takes when it spends its in-session
reviewer allowance and escalates: *"finish only to a coherent reviewable boundary,
retire the producer, then commit … and hand back with `grove-llm complete`"*.

**Failure:** a session that escalates follows that skill, runs `grove-llm complete`
and gets clap's `unrecognized subcommand` (`tests/retired_completion.rs` pins that
exact refusal, with no hint). The prompt's own last sentence says
`harness-dispatch exit`; a session that obeys the skill's closing order instead never
sends the exit signal. An interactive harness then returns to its prompt and the loop
stalls, which the glossary's *Launch directory* entry states is the normal result of
a forgotten exit signal.

**Why the guard missed it:** the reader is sound — it scans the whole file and
accepts one line break inside the separator, and this mention is exactly that shape —
but it is not pointed at this file. This sweep's controls: `rg -nU
'grove-llm\s+complete'` over the tree finds both mentions. The positive control is
that the same pattern finds `grove-llm\s+record-teardown` in
`plugins/grove/skills/grove-finish/SKILL.md`; the cross-check is that a line-based
`rg 'grove-llm complete'` finds the spec's mention and misses the skill's wrapped one,
which is how a single-line sweep reads clean over a live instruction.

**Correction:** replace the instruction with the prompt's own ending (`harness-dispatch
exit`, which the skill should not restate as a contract it does not own — say "end the
session as the prompt directs"), make the same edit in the spec, and either widen
`instructed_verbs`'s corpus to every `plugins/*/skills` tree or add a second assertion
over `plugins/linkuistics` with the same reader, so a removed verb cannot survive in
a sibling plugin.

### F2 — P3 (CONFIRMED): the reading table's first two precedence rows, and teardown over a surviving group, have no test

**Location:** `crates/grove-loop/src/loop_driver.rs:217-244` (interrupt, then
teardown, then `Group::Present`, then `exit_signal`); the tests are
`crates/grove/tests/loop_driver.rs:931` (`a_sigtermed_driver_stops_and_reaps_its_child`),
`:3799` (`launch_directory_teardown_finishes_after_dispatch_signal_or_own_exit`) and
`:3963`.

The spec says *a teardown record finishes whatever the ending* and that the driver's
own TERM or HUP is the first row; the producer named the precedence as where it was
least certain. The code orders them as the table does. But no test produces two rows at
once: the TERM test never has a teardown record, and the teardown tests never have a TERM
or a surviving member of dispatch's group. Swapping the `Interrupted` and
`reading.teardown` blocks, or moving the `Group::Present` stop above the teardown test,
leaves every case green.

**Correction:** one PTY case where the fake session records its teardown and then holds
until the driver is TERMed (expect `Interrupted`, loop not finished), and one where
dispatch's group keeps a survivor after a teardown record (expect `Finished`).

### F3 — P3 (CONFIRMED): the driver's `Group::Present` stop is a row the *Grove integration* table does not have

**Location:** `crates/grove-loop/src/loop_driver.rs:229-240` against the table under
*Ending a session* in `docs/specs/harness-selection-and-execution.md`, and
`plugins/grove/skills/grove/references/driver.md` (*What ends the run and what
continues the loop*).

The table is stated as "first match wins" over four rows. The driver inserts a fifth
between teardown and `exit_signal`: a member of dispatch's own group that outlived the
runner's kills stops the loop even when the ending file says `exit_signal`. That is the
right behaviour, and `module-decomposition.md` decision 7 says the runner reports it so
that nothing acts beside a survivor; but a reader building the driver from the table the
spec calls authoritative would relaunch. The methodology's `driver.md` repeats the four
rows and inherits the gap.

**Correction:** add the row to the spec's table and to `driver.md`, naming that it is
dispatch's group (the policy worker's, not the harness's, which reaches Grove as exit 5).

### F4 — P4 (CONFIRMED): the abandoned-launch test's "would block" control cannot fail

**Location:** `crates/grove/tests/loop_driver.rs:3926-3930`.

The fixture makes a FIFO named `ending` inside the abandoned directory and comments that
reading it would block. The driver reads `ending.json` (`launch_directory.rs`), and does
so with `O_NONBLOCK | O_NOFOLLOW` and a regular-file check, so neither a read of
`ending.json` nor a read of `ending` could block it. The "reads nothing" claim is held by
the `teardown` file's text (a read of it would print `grove finished`), which does fail
when interpreted; the ending half is not pinned. A control that has not been seen to fail
is not a control.

**Correction:** name the FIFO `ending.json` and put a valid `exit_signal` document in a
second abandoned directory, expecting neither to relaunch or block.

### F5 — P4 (CONFIRMED): the contract step leaves the expand step's legacy epoch reading in place

**Location:** `crates/grove-loop/src/driver_lease.rs:769-783` (`parse_epoch_record`
accepts `signal-path-hex=` for an active record, with the comment "The expand step still
understands an installed v22 epoch record"); `crates/grove-tui/src/observation.rs:247,299-300`
(fixtures that use it).

After `k18` nothing in this repository writes `signal-path-hex`, the brief says the old
contract goes, and the ADR names only `launch-dir-hex`. The branch survives only for an
installed v22 driver's epoch record read by a v23 viewer or `grove-llm`, which the spec
nowhere promises, and the viewer's fixtures now exercise it by accident. Either the
compatibility is intended, in which case the ADR or `item-status.md` should say so and a
test should own it, or it should go with the fixtures.

### F6 — P4 (CONFIRMED): "no signal — the loop stops" overreaches for an interactive harness

**Location:** `plugins/grove/skills/grove-finish/SKILL.md:40` (the third row of the ending
table), the prompt's closing sentence in `crates/grove-loop/src/prompt.rs` (*"A session that
ends without signalling stops the loop instead"*), against `CONTEXT.md`'s *Launch directory*
entry: *"reading a **missing** exit signal as 'the loop stops'. That holds only for a harness
that exits on its own … the loop **stalls** instead."*

The driver reads nothing until dispatch has reaped the harness. A `finish` session that
declines, or runs with no human, and then simply ends its turn, leaves an interactive
harness at its prompt: the driver waits and the leaf stays live, but the loop has not
stopped. The skill and the prompt are the text a session acts on, and they are the two places
that do not say so; the pre-cutover wording had the same overreach, so this is not a
regression, but the glossary now disagrees with them.

**Correction:** say "no signal — the loop waits for you to end the harness, and then stops with
the leaf live", in the skill and in the prompt's sentence.

### Checked and held (what a later reader need not redo)

- **Stale sessions.** Admission compares the ambient `GROVE_LAUNCH_DIR` with the epoch's
  launch directory, found from the path's parent, under a shared epoch guard
  (`driver_lease.rs` `admit_session`). A stale `pick`, `leaf-add` and `record-teardown` are
  refused with *stale Grove session* (`loop_driver.rs:3865`), and a stale `harness-dispatch exit`
  fails because dispatch allocates each run's channel, by a 128-bit name, inside the launch
  directory that is already gone (`:4196`). The structural reason, rather than the test's, is
  that a launch directory is never reused, so a leftover one — a failed `remove_dir_all` — can
  hold a stale exit file that no live dispatch is watching.
- **Replacement cleanup.** `discard_abandoned` runs after `initialize_epoch_record` has taken the
  exclusive epoch guard and so after any admitted old verb has finished. It lists the control
  directory and removes only `launch-` plus 32 lowercase hex digits, never follows a symlink,
  and reads no file. An unremovable directory warns and does not stop the driver.
- **Reading.** `ending.json` is read with `O_NOFOLLOW | O_NONBLOCK`, regular files only, 1 MiB
  bounded, and relaunches only on `schemaVersion` 1, `source` `harness-dispatch`, `state`
  `observed` and `value` `exit_signal`. The post-reap invalidation precedes every read.
- **Terminal and background.** `dispatch_death_recovers_the_terminal_while_its_raw_harness_survives`
  and `a_driver_started_in_the_background_takes_no_foreground` exercise the real front under a PTY,
  with a control that the harness really did set raw mode.
- **The 10 s bound.** Dispatch's worst case from the driver's TERM is a 500 ms poll, the 5 s
  kill-grace, 10 ms, 20 ms, a 1 s confirmation and the 2 s store lock, about 8.53 s
  (`module-decomposition.md` decision 7), which `harness-dispatch`'s own constants
  (`run.rs`, `store.rs`, `keyed-launch/src/run.rs`) reproduce. The margin is real and the
  document says it is not a wall-clock guarantee.
- **Endings the prompt and skills describe.** Retire-and-exit relaunches; `record-teardown` then
  exit finishes, as does `record-teardown` then the harness exiting; exit without a record
  relaunches onto a tree-less grove only if the skill is unread (decision 9 states the residue);
  no signal stops once the harness is reaped (F6). `record-teardown` itself refuses while `.grove/`
  exists, resolves the working tree through the workspace root rather than the cwd, and is idempotent.
- **A launch directory is written by the session as well as by Grove.** A session can create
  `teardown` or `ending.json` in it directly, since `harness-dispatch exit` needs the directory
  writable. That is the ADR's stated cooperating-process limit and is the same trust the retired
  channel's `done` token had; it is not a finding.

### For the leaves after this one

Not findings against `launch-cutover-k15`, recorded so they are not rediscovered:
`docs/USAGE.md`, `docs/ARCHITECTURE.md`, `README.md`, the root `CLAUDE.md`,
`docs/specs/user-guide-coverage.md`, `grove-llm-book-structure.md` and
`keyed-launch-book-structure.md` still describe `complete`, the signal file or
`GROVE_SIGNAL_FILE` as current (`current-state-docs-k20`'s). `release.toml:68`,
`docs/RELEASING.md:492-496` and `scripts/release-publish.sh` still describe a session
that ends without `grove-llm complete` as the thing that breaks a loop across a
reinstall; under the new contract a v22 driver whose session meets the installed v23
`grove-llm` cannot send `complete` at all, so the live-grove cutover
(`major-release-k21`) has to decide how a running v22 session is ended.

### Limits of this review

No test, build, lint or format command ran. Linux was not exercised by the producer or by
this review. Codebase-graph access was unavailable, so no negative repo-wide claim rests on it;
the stale-text claims came from `rg` with the controls described under F1, and the claims about
which tests produce which combinations of endings came from reading the tests' bodies.
