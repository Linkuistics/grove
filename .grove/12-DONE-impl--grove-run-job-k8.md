# grove-run-job-k8

## Goal

Move `grove run` to the shared function in its detached shape. This is the
first caller to migrate. It is the smaller one, and it needs no lease.

## Context

- The function of `ravel-job-k7`. Read its running log for the chosen names.
- `crates/grove/src/standalone.rs` is the caller. It uses
  `run_noninteractive_with_stdout`, `End`, `Escalation`, `Group`,
  `take_interrupt` and `regular_file_at` of `keyed-launch`.
- The root brief: *Behaviour that stays* (the `grove run` item), *Residues*
  and *Tests* (the standalone case under a pseudo-terminal).
- `docs/specs/standalone-invocations.md` and the section *Standalone
  interpretation* of the caller contract.
- The book `overview` has the whole of `crates/grove/src/**/*.rs` as its
  corpus.

## Done when

- `grove run` starts `ravel` through the shared function in the detached shape.
  It cancels on Ctrl-C, TERM or HUP. It publishes its outputs only after a
  successful Ravel status with `grove-continue`. Its transcript relay works as
  it does now.
- The survivor guard and the check of the group are gone. A failed run exits 1.
- The copy of the scrub list in `standalone.rs` is gone. `grove run` and the
  version probe of `dispatch/ravel.rs` use the list and the helper of the new
  module.
- `regular_file_at` is a private function of the standalone module. The
  observer is a no-op there.
- `crates/grove/Cargo.toml` no longer depends on `keyed-launch`.
- A new standalone case runs under a pseudo-terminal. A Ctrl-C typed at
  `grove run` cancels it and publishes nothing. The foreground stays with the
  group of `grove run`. The case is an ordinary test that the named activation
  tasks reach, with no `#[ignore]` outside what those tasks already run.
- Chapters 01, 05 and 06 of the `overview` book match the changed code and the
  changed `Cargo.toml`. The conversions of those chapters land first as their
  own changes.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `test:grove`, `ravel:activation` and `test:authority`.

## Notes

- The loop driver still uses `keyed-launch`. Two latches exist in the source
  tree until `lifecycle-job-k9`. They never run in one process, because
  `grove run` and `grove do` are separate invocations.
- Remove the conversion of `Argv` at the call site of `grove run` that
  `ravel-job-k7` left. That leaf already edited the quoting fragments of
  chapter 06. This leaf edits the same fragments again for the migration.
- Chapter 06 and its conversion may already exist from `ravel-floor-k6` and
  `ravel-job-k7`. Convert a chapter only if it is not yet converted.

## Decisions (running log)

- **Conversion.** Chapters 05 and 06 of `overview` already passed the checker.
  Chapter 01 had 52 hard findings. This leaf converted only the two sections
  that the migration edits, *A crate, not a `[[bin]]` target* and *Public
  library seams and binary-owned transfers*. It landed as its own change first.
  The other sections of chapter 01 stay unconverted.
- **Call site.** `standalone::execute` calls `ravel_job::launch` with the
  `Argv` of `session_command`, the work directory, the held stdout file, an
  observer that does nothing, and `Shape::Detached { transcript: log }`. The
  conversion to `keyed_launch::Argv` is gone. The survivor guard, the check of
  the group and the escalation bounds are gone.
- **Cancellation.** Two checks remain. `JobEnd::interrupted` finds a signal that
  the job caught. `ravel_job::take_interrupt` finds one that arrived after the
  reap, both before the status check and before each publish. A failed run exits
  1, as before.
- **Scrub list.** `standalone::CONTROL_ENV` is gone. The version probe in
  `dispatch/ravel.rs` calls `ravel_job::scrub`.
- **`regular_file_at`.** It is a private function of `standalone.rs`, copied
  from the crate with `openat`, `O_NOFOLLOW` and `O_NONBLOCK`. No observer
  matters there, so the observer is a no-op.
- **Dependency.** `crates/grove/Cargo.toml` no longer names `keyed-launch`, and
  `Cargo.lock` follows. The comment on the manifest says that main reaches only
  public loop and viewer interfaces.
- **New case.** `a_typed_ctrl_c_cancels_grove_run_and_the_foreground_stays_with_it`
  in `crates/grove/tests/standalone.rs` is an ordinary test, so `test:grove`
  reaches it. It starts `grove run` as a session leader on a pseudo-terminal of
  its own. It checks with `ps` that `grove run` leads its group and holds the
  foreground. It types Ctrl-C. Then it checks the exit status, the message
  `standalone invocation was cancelled`, the absent output and the gone harness.
  The case was seen to fail when `launch` stopped sending the latched signal to
  `ravel`. I did not see the foreground assertion fail. I found no cheap mutation
  that moves the foreground and keeps the transcript. The cancellation message
  is the second guard. A foreground held by `ravel` would send the
  interrupt past Grove. The run would then end as a failed Ravel and not as a
  cancellation.
- **Taskfile.** `ravel:activation` failed in `test:grove-llm`, because three
  `removed_surface` cases need `GROVE_TEST_RAVEL` and that task never set it.
  The failure predates this leaf. `test:grove-llm` now requires `RAVEL` and sets
  the variable, and `ravel:activation` passes it, as for `test:grove` and
  `test:grove-loop`.
- **Book.** Chapter 01 follows the manifest (59 lines, the dependency fragment
  at lines 28-38). Chapter 05 drops `keyed-launch` from the row of `grove` and
  names `ravel_job::launch` and `ravel_job::scrub` in the table of public items.
  Chapter 06 quotes the new `standalone.rs` (332 lines) and `ravel.rs` (330
  lines), and its supervision section describes the shared function. The
  fragment `standalone-artifact-checks` now covers `regular_file_at`
  (lines 222-296). The totals are 59, 786 and 1,454 in all. The source index,
  the manifest and the rollup lines of chapter 05 follow.
- **For the next leaves.** `book-specs-k14` records the new totals of `overview`
  in `overview-book-structure.md`. `user-records-k13` fixes `docs/USAGE.md`,
  which still names the survivor group of `grove run`. The loop driver still
  uses `keyed-launch`, so `ravel:activation` keeps `test:keyed-launch` until
  `keyed-launch-removal-k11`. `lifecycle-job-k9` already lists the prose of
  chapters 03 and 04 of `overview` that describes a surviving Ravel group, so
  this leaf leaves that prose alone.
- **Result.** `bash scripts/check.sh` passes all 20 checks. With the installed
  Ravel 1.1.0, `task ravel:activation` passes, and it includes `test:grove`,
  `test:grove-loop`, `ravel:job`, `test:grove-llm` and `test:authority`.
