# grove-task-slots-k11

## Goal

Give Grove lifecycle command templates three optional whole-argument slots,
`${kind}`, `${task_file}` and `${task_id}`. A wrapper can then receive the
selected task's open kind token, absolute task path and stable handle as native
argv data, without parsing the prompt or a filename. The increment is useful to
any wrapper; `harness-dispatch` does not exist yet and is not needed.

## Context

The contract is the spec's `#grove-integration` section. The existing slot rules
live in Grove's session configuration: whole-argument slots, a prompt that
occurs exactly once, NUL refused at expansion, and no splitting or
reinterpretation. The loop driver expands the command from its authoritative
selection. Populate the new slots from that same selection, the one that
composes the mandate. Do not add a second pick, and do not read the kind back
out of the path.

Standalone `grove run` has its own vocabulary. It must reject a template that
requests a lifecycle-only slot, with an actionable diagnostic. The common
configuration machinery stays consumer-vocabulary-driven: the runner learns no
Grove nouns.

## Done when

- The lifecycle vocabulary offers `kind`, `task_file` and `task_id` as optional
  slots. A template that uses them receives the selected leaf's kind, absolute
  path and `<slug>-k<key>` handle. Existing templates, the four existing slots
  and direct-harness commands behave byte-for-byte as before.
- Grove's existing launch-boundary integration test shows the three values
  arriving as whole native arguments at a fake harness. Spaces, quotes and shell
  punctuation in the task path and prompt must stay data. The prompt must arrive
  unchanged.
- A standalone `grove run` template that requests any of the three slots refuses
  and names the slot and the consumer. The existing standalone vocabulary is
  unchanged.
- `grove config show` renders the new slots symbolically when no task is
  selected. It fabricates no task and evaluates nothing.
- `docs/CONFIGURATION.md` documents the slots, which says "Grove's existing
  four slots" today. The configure-grove skill documents them too. The
  `grove-loop` book and any other touched book stay source-exact. `task check`
  passes.

## Notes

This leaf adds no dispatch-specific behavior, no environment variable, no
task-body metadata and no new invocation override. The spec's Grove-seam rows
that need a dispatcher belong to `grove-dispatch-k31`.
