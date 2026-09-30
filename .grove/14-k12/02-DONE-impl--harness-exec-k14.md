# harness-exec-k14

## Goal

Make `harness-dispatch run` hand the selected joint choice to its harness. It
expands the candidate's argv from literals and whole-argument slots, resolves
the program, and replaces itself with the harness while preserving cwd,
descriptors and exit behavior. `inspect` shows the same expanded argv.

## Context

The contract is in the spec's `#command-interface`, `#policy-and-choice` (slots
and program resolution) and `#execution-contract` sections. Take only the plain
exec here. Handled signals, the linearization point and signal-state
transparency belong to `evaluation-boundary-k27`. The whole-selection deadline
is the next leaf's. The required handoff record, the run ID and the `runId`
slot belong to `handoff-records-k24`, later in this node, so `runId` is refused
as not yet supported. Until that leaf lands, this plain `run` is an internal
step of the increment, not a delivered form. The spec's notice does not list
`run` as delivered, and no increment boundary exposes it without its record.

## Done when

- `--prompt TEXT` or `--prompt-file PATH` is read once, preserving its bytes and
  trailing newlines. It must be valid UTF-8 with no NUL and at most 1 MiB. `run`
  requires exactly one of them. `inspect` renders a marked placeholder when
  neither is given. Terminal stdin is never read, and the prompt never reaches
  the worker.
- `--task-file` resolves against the original cwd. `--task-id` accepts an opaque
  UTF-8 identity of at most 1024 bytes. Both reach the policy request and their
  slots, and neither supplies kind or identity from anything else.
- Arguments are literals or slot objects: `prompt` (exactly once), `kind`,
  `taskFile`, `taskId`, `model` and `effort`. A slot fills one whole argument.
  An absent optional input cannot satisfy a used slot. There is no shell,
  splitting or interpolation.
- The program is an absolute path, a PATH name, or a relative path containing a
  separator, which resolves against the caller's cwd. Inspection reports the
  resolution. Only the selected program is checked. Not found exits 127 and
  unexecutable exits 126, and neither ever falls back to another candidate.
- `run` execs the harness in the caller's cwd with its descriptors and
  environment. Short choice diagnostics go to stderr, and stdout and stdin stay
  the harness's. An exec error reports errno with a remedy and exits nonzero.
- Command-seam tests show a fake harness receiving literal spaces, quotes, shell
  punctuation and newlines in the prompt and paths. It keeps its cwd and exits
  with its own code and signal.

## Decisions (running log)

**Inspection resolves the program exactly as `run` does, and refuses the same
way.** An unresolvable selected program makes `inspect` exit 127 (not found) or
126 (unexecutable), with the candidate, program and location, rather than
reporting exit 0 for a proposal `run` could not launch. The brief's acceptance
case lists an unavailable selected executable among the inputs that are
reported and launch nothing, and a caller scripting `inspect` as a preflight
gets the answer `run` would give. The command-seam sandbox therefore puts a
recording `fake-harness` on every test's PATH.

**PATH search is `execvp`'s, over the caller's own PATH, and resolves once.** A
program with no separator is looked up entry by entry in the front's PATH: an
empty entry means the cwd and a relative entry resolves against it, as the
shell and `execvp` do, so dispatch launches what the caller's shell would. The
first executable regular file wins; a same-named directory or non-executable
file is passed over, and only if nothing executable is found does it decide the
outcome (126 rather than 127). The resolved absolute path is what `run` execs,
so resolution and exec cannot disagree, and inspection reports which entry
matched.

**`argv[0]` is the configured program string**, as a shell passes the command
as typed; the resolved path is only the exec target. Both appear in inspection.

**An exec error after resolution exits 127 for `ENOENT` and 126 otherwise**,
with the stable code `exec_failed`, stage `exec`, the errno in the message and a
remedy chosen by errno (`E2BIG`, `ENOEXEC`, `EACCES`, `ENOENT`, `ETXTBSY`). A
file that vanished after resolution, or a script whose `#!` interpreter is
missing, is `ENOENT`.

**Slot rules are catalog-wide shape checks; slot satisfaction is per
invocation.** `prompt` exactly once and the refusal of `runId` (until
`handoff-records-k24`) are validated for every candidate, since the spec checks
all catalog shapes. A used `taskFile` or `taskId` slot with no matching input
refuses at stage `expansion` with `missing_input`, exit 3, naming the flag to
supply and the slot's location.

**The prompt is read before policy evaluation**, once, bounded at 1 MiB + 1
bytes, and never sent to the worker. Its failures are CLI input (exit 2):
`prompt_unreadable` for a file that cannot be opened or read, or that is a
terminal, and `prompt_invalid` for size, non-UTF-8 or NUL. A terminal is
refused so that `--prompt-file /dev/stdin` cannot turn selection into an
interactive read.

**`--task-file` is resolved, not read.** It is joined to the original cwd, kept
lexical (no symlink resolution) and must be UTF-8, because it travels in the
JSON request and as an argument. Nothing here reads it or requires it to exist;
the adapter and context loaders that read it arrive later.

**`run --json` writes JSON lines on stderr.** One handoff notice
(`{"schemaVersion":1,"handoff":{…},"diagnostics":{…}}`) precedes exec, and one
error object follows only if exec fails. Stdout and stdin stay the harness's.
Text mode prints the captured policy diagnostics and one `harness-dispatch:`
line instead.

**An unset or empty PATH is not replaced by `execvp`'s built-in default.** A
PATH name then refuses as not found, saying PATH is unset. Substituting a
search path the caller never supplied would be an invented default.

**Rust's `Command::exec` is `execvp`, which this leaf relies on.** Verified in
the installed std source (Rust 1.98.1,
`library/std/src/sys/process/unix/unix.rs:413`), not from memory: it calls
`execvp` with the program, inherits the signal mask, and resets SIGPIPE to
default. Passing the resolved absolute path means `execvp` searches nothing,
and an `ENOEXEC` file is retried through `/bin/sh` as a shell would. The
SIGPIPE reset is `evaluation-boundary-k27`'s to undo.

**Clap refusals now name their flag.** A malformed command line under `--json`
carries `input` taken from clap's own error context (`--task-file`, not
`--task-file <PATH>`). Clap refuses an empty path before this crate sees it, so
there is no second empty-path check to keep in step.

**`run` is visible in help.** It works and is documented as not yet delivered,
in the README and in the spec's notice, until `handoff-records-k24` adds the
required record. Hiding a working command would only make it harder to test.

**No in-session reviewer.** The node's scheduled `review-impl` already names
process semantics among its doubts, so this producer's review is scheduled.

**Done-when instruments.** The command-seam tests are in
`crates/harness-dispatch/tests/run.rs` unless another file is named.

- Prompt input: `run_requires_exactly_one_prompt_input`,
  `a_prompt_file_is_read_once_with_its_exact_bytes_from_the_callers_cwd`,
  `an_invalid_or_unreadable_prompt_is_refused_before_policy_runs` (size, UTF-8,
  NUL, unreadable, directory, beside a 1 MiB positive control and a sentinel
  showing no policy ran), `a_terminal_is_never_read_as_the_prompt`, and the
  placeholder in `inspection_reports_the_routed_candidate_and_its_evidence_in_json`
  (`inspect.rs`). That the prompt never reaches the worker is
  `the_request_carries_the_task_inputs_and_never_the_prompt` (`worker.rs`): a
  fake worker with the real identity records the evaluate frame byte for byte.
- Task file and identity: that same frame test,
  `task_identity_and_file_inputs_are_checked_as_caller_data`,
  `a_grove_shaped_task_file_supplies_neither_kind_nor_identity`, and
  `inspection_shows_the_unchanged_prompt_and_every_expanded_slot` (`inspect.rs`).
- Slots: the invalid-shape table's no-prompt, two-prompt and `runId` rows
  (`inspect.rs`), `an_absent_optional_input_satisfies_no_slot`, and
  `run_hands_the_exact_argv_to_the_harness_in_the_callers_own_process_and_cwd`.
- Program resolution:
  `the_program_resolves_as_a_path_name_an_absolute_path_or_a_cwd_relative_path`,
  `a_path_search_passes_over_a_non_executable_match_as_execvp_does`,
  `a_missing_program_exits_127_and_never_falls_back_to_another_candidate` (which
  also shows only the selected program is checked), and
  `an_unexecutable_program_exits_126`.
- The exec: the exact-argv test (same PID, physical cwd),
  `the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors`,
  `the_harness_exits_with_its_own_code_and_signal`,
  `run_json_writes_one_handoff_line_on_stderr_and_leaves_stdout_to_the_harness`
  and `an_exec_error_reports_errno_with_a_remedy` (`E2BIG` from a 1 MiB prompt,
  and `ENOENT` from a missing `#!` interpreter).
- Controls seen to fire, each by a mutation run and then reverted: sending the
  prompt in the request failed the frame test; spawning and waiting instead of
  exec failed the PID and signal assertions; stopping the PATH search at a
  non-executable match failed the `execvp` test; and dropping the terminal
  check failed the terminal test at its 20-second deadline, leaving no stray
  process.
- Checks: `cargo clippy -p harness-dispatch --all-targets -- -D warnings` is
  clean, and `task dispatch:typecheck` passes on the rebuilt worker.
  `scripts/check.sh` passed all 12 principal checks, with 1506 tests and none
  failing.
