# dispatched-launch-k32

## Goal

Prove at Grove's existing launch boundary that a lifecycle session routed
through `harness-dispatch` receives the right data and authority, and that
existing direct-harness routes are unaffected.

## Context

The slots come from `grove-task-slots-k11`. The run identity comes from
`handoff-records-k24` and the environment scrubbing from `ambient-authority-k30`.
Grove's launch-boundary integration suite builds temporary configurations with
fake harnesses. Extend it rather than creating a parallel harness. The suite
must obtain `harness-dispatch` and its worker by the same deterministic route
the skeleton chose, never by skipping.

## Done when

- A temporary personal Grove configuration routes a kind to `harness-dispatch
  run` with the `kind`, `task_file`, `task_id` and `prompt` slots. A temporary
  dispatch policy and state directory route it to a fake harness. A driven
  session shows the fake harness receiving the unchanged prompt, including
  spaces, quotes, shell punctuation and newlines, plus the authoritative kind,
  absolute task path and handle, each as one native argument.
- The fake harness receives `HARNESS_DISPATCH_RUN_ID`, matching a recorded run,
  and Grove's fresh completion channel, and can signal completion through it.
  A policy probe records its own environment, which lacks that channel.
- Task authoring (`grove-llm leaf-add`) succeeds under a valid wrapper whose
  dispatch policy lacks the kind. The next launch refuses with dispatch's
  incomplete-mapping diagnostic and the equivalent inspect invocation, and the
  leaf stays live for the next attempt.
- The existing direct-harness launch tests pass unchanged, and a direct and a
  dispatched kind coexist in one configuration.
- A literal `--choice` in the personal command definition reaches policy as the
  explicit choice.

## Decisions (running log)

**The cases join `crates/grove/tests/loop_driver.rs`, in a section of their
own.** That file is Grove's launch-boundary suite, and its driver, capture and
tree helpers are what the brief says to extend. The front comes from
`support::harness_dispatch()`, beside `grove_llm()` in `testing/support.rs`:
the workspace binary, found from the test's own profile directory. Its worker
comes by the skeleton's route, `task dispatch:worker` before `cargo test`, and a
missing or stale one refuses with exit 5 naming that task. No test skips.

**The fake harness ends its session through the real `grove-llm complete`.**
Writing the channel with `printf`, as the older fixtures do, would show the
path arrived but not that the harness can use it: `complete` also passes the
session-epoch admission against the channel it is given. A driver that reports
`loop complete` is the evidence.

**"Unchanged" is measured against a direct launch of the same leaf.** The
first case launches the leaf straight to the fake harness, then through
dispatch, and compares the two prompts byte for byte. A prompt fixed in the
test would prove only that dispatch passes on whatever it was given. The
working tree's name carries the spaces, quotes and shell punctuation, as in
`the_selected_task_arrives_as_native_arguments_beside_an_unchanged_prompt`, and
the prompt's own many lines carry the newlines.

**A refused launch is observed through the driver's own stderr.** The
foreground child inherits it, so dispatch's text refusal and its `inspect:`
line sit beside Grove's `configured session kind … failed`. The test runs that
line through `/bin/sh`, as its reader would, and sees the same refusal. That
the leaf "stays live" is shown by resumption, not by a filename: after the
owner routes the kind, the next driver run launches that same leaf.

**The authoring case authors from inside a dispatched session.** A session
launched through the wrapper runs `grove-llm leaf-add … --kind design`, which
is where authoring happens, and the test does not call the verb itself.

**Controls seen to fire, by hand.** Granting `--policy-env GROVE_SIGNAL_FILE`
to the main dispatched launch failed the "no Grove variable" assertion, and
the view showed the worker's whole environment. Removing Grove's `design` route
failed the authoring assertion with Grove's own "no launch template resolves"
refusal, so that case depends on the pre-authoring check it describes. The
grant control also stays in the test permanently. The choice case's control
is the same command without `--choice`, under which the policy sees no choice.

**Done-when instruments**, all in `crates/grove/tests/loop_driver.rs`:

- Unchanged prompt, and kind, absolute task path and handle as native
  arguments; `HARNESS_DISPATCH_RUN_ID` matching `record show`; the fresh channel
  that the live epoch names, used through `grove-llm complete`; the policy probe
  without it, and its control:
  `a_dispatched_session_receives_its_task_as_native_data_and_only_its_harness_the_channel`.
- Authoring succeeds, the launch refuses with `incomplete_mapping` and the
  reproducing `inspect`, and the leaf stays live:
  `a_leaf_authored_under_a_valid_wrapper_is_refused_at_launch_and_stays_live`.
- Direct-harness tests unchanged (the diff to that file only adds), and a
  direct and a dispatched kind in one configuration and one loop:
  `a_dispatched_and_a_direct_kind_coexist_in_one_configuration`.
- A literal `--choice` reaching policy, and its control:
  `a_literal_choice_in_the_command_definition_reaches_policy_as_the_explicit_choice`.

**No in-session reviewer.** These are tests of existing behavior, and each
claim has an assertion whose control was seen to fire. The node's integration
risks, cancellation and the controlling terminal, are
`dispatched-terminal-k33`'s.
