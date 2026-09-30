# grove-dispatch-guidance-k34

## Goal

Document, where an owner configures Grove, how to route sessions through
`harness-dispatch` and which configuration owns what. Cover the two inspection
surfaces, the launch-time validation boundary, and the exact remedy when a
delegated mapping is incomplete.

## Context

The requirements put this guidance in usage documentation and in the
configure-grove skill. The skill lives in `plugins/grove/skills/configure-grove/`,
and Grove's usage and configuration references live under `docs/`. Grove's
user-guide coverage tests may pin wording. Update them with the prose, never
around it. The review policy and the creator-line remedy arrive with
`review-policy-k35` and `creator-reference-k38`. Leave room for them without
describing them early.

## Done when

- configure-grove explains activation. The owner points a personal command
  definition at `harness-dispatch run` with the new slots and the prompt, may
  add a literal `--choice`, and creates a personal policy that no install
  overwrites. It explains that Grove's configuration owns the wrapper while
  dispatch policy owns selection. It shows `grove config show` for the wrapper
  and `harness-dispatch inspect` for the selection. It states that Grove's
  pre-authoring check stops at the configured command, so delegated policy can
  refuse at launch after authoring succeeded. It gives the exact remedy for an
  incomplete mapping, and it warns never to grant `GROVE_SIGNAL_FILE` with
  `--policy-env`.
- The Grove usage and configuration references say the same thing for readers
  outside the skill, and link the dispatch usage documentation.
- `harness-dispatch --help` carries a Grove example that matches the
  documentation.
- The spec's notice states what is delivered. The node brief's `Done when`
  holds.

## Decisions (running log)

**Each surface carries its own share, and one command line joins them.**
Grove's configuration reference gets the whole account, in a section of its
own: activation, the division of ownership, both inspection surfaces, the
launch-time boundary, the incomplete-mapping remedy and the
`GROVE_SIGNAL_FILE` warning. The usage guide keeps to its register. It adds the
human workflow, which is inspecting both surfaces and recovering from a refused
launch, and links the reference for the configuration, because the coverage
inventory reserves kind routing to `CONFIGURATION.md`. configure-grove gets a
reference file of its own, `references/dispatch.md`, which `SKILL.md` routes to.
That leaves room for `review-policy-k35` and `creator-reference-k38` to add the
review policy and the creator-line remedy without reshaping the skill. The
dispatch README gains a short "Called from Grove" section, since the Grove
documents link it and an owner arriving from dispatch should find the same
line. Each of them quotes the tested command definition verbatim,
`harness-dispatch run --kind ${kind} --task-file ${task_file} --task-id
${task_id} --prompt ${prompt}`.

**"Matches the documentation" is a test, and it lives on Grove's side.** The
test sits beside `dispatch_template` in `crates/grove/tests/loop_driver.rs`.
It reads the Grove example from `harness-dispatch --help` and requires the
same words as the tested template. It requires each document to carry the
line verbatim, and has `grove config show` admit it with the three task slots
and the prompt as slots. Grove depends on dispatch, so reading the help from
Grove's suite crosses no package boundary an extraction would cut. The
dispatch package's own help test only gains the fact that the example is
there.

**The transcripts are real.** A scratch jj repository had a personal Grove
route for `design` through the documented command, a policy routing only
`impl`, and one live `design` leaf. `grove` printed the refusal, its
`inspect:` line and Grove's "configured session kind `design` failed" with
exit status 3, and left the leaf live. After `design` joined the routes, the
same `grove` launched that leaf. The usage guide carries both runs, with the
working tree and HOME rewritten.

**The usage guide says less, by its boundary.** It carries the ownership,
both inspection surfaces, the launch-time boundary and the refused-launch
remedy. For activation, `--choice` and the `GROVE_SIGNAL_FILE` warning it
links the configuration reference, because the coverage inventory reserves
kind routing to that document. The two Grove references together say what the
skill says. The inventory itself is unchanged, although `harness-dispatch` is
now a third binary the formula installs. Whether the guide owns its surface
is a boundary question for `dispatch-documentation-k41`, not this leaf.

**Controls seen to fire, by hand**, each restored and its digest checked
before the next. The documentation test failed on a wrong slot in the
configuration reference, on the README without the line, on the skill
wrapping it across two lines, on `--choice` after the prompt, and on
`--task-id` misspelt in the compiled help. The link check failed on a broken
anchor from the reference into the README, and on one back. Both suites were
green again after each restore.

**Done-when instruments:**

- configure-grove's activation, ownership, both surfaces, the launch-time
  boundary, the remedy and the warning:
  `plugins/grove/skills/configure-grove/references/dispatch.md`, routed from
  `SKILL.md` and `references/model-selection.md`.
- The Grove references: `docs/CONFIGURATION.md#harness-dispatch`, which
  links the dispatch README throughout, and
  `docs/USAGE.md#if-a-dispatched-launch-refuses` with the dispatch passage
  under *Inspecting configuration before launch*. Links resolve:
  `crates/grove/tests/reference_navigation.rs`.
- The help's Grove example matches the documentation and the launched command:
  `the_documented_command_definition_for_dispatch_is_the_one_launched_here`
  in `crates/grove/tests/loop_driver.rs`. It is present in both help texts:
  `help_carries_independent_use_grove_and_refusal_recovery_examples` in
  `crates/harness-dispatch/tests/refusals.rs`.
- The spec's notice states the guidance and the help example as delivered.

**No in-session reviewer.** The guidance restates behaviour that k32 and k33
tested. Where a document could drift from that behaviour, which is the command
it quotes, a test with controls now holds it. `dispatch-documentation-k41` ends
in the mandatory documentation-acceptance review.
