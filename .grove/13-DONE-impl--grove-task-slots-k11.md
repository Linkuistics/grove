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

## Decisions (running log)

**The expansion context carries the selection, not three values.** The lifecycle
`ExpansionContext` gains one field holding the driver's selected task
(`grove_loop::Selection`), and `expand` reads `${kind}`, `${task_file}` and
`${task_id}` from it: `Kind::label()`, `Selection::path` and the handle's
`<slug>-k<key>` rendering. The driver passes the same selection that composed
the mandate, so the three values and the prompt cannot come from two sources.
The path is absolute because the tree root is the lease's canonical worktree
root; nothing re-reads the kind from the path. The three slots are
`AtMostOnce`; `${prompt}` and the other three keep their rules.

**Standalone keeps the whole lifecycle vocabulary at load and refuses per key.**
`grove run` reads the same personal file, so validating it against a four-slot
vocabulary would let one lifecycle route using `${task_file}` break every
standalone kind. Instead it inspects the routed command's compiled words and
refuses any task slot before staging anything, naming each slot and `grove run`.
Expansion's value contract covers the whole vocabulary, so the three task slots
are offered empty values that the refusal has proved cannot reach argv.
`keyed-launch` is unchanged: it learns no Grove noun and no new API.

**`grove config show` needs no code change.** It already renders every slot word
as `slot <name>` from inspection, with no task selected; a test pins that for
the three new names in human and JSON output.

**Statements the slots made false are corrected where they stand.** The
methodology's `driver.md` said `${session_name}`, `${worktree}` and `${repo}`
were the only slots besides the prompt; it now names the task slots too. The
keyed-launch book keeps its four-slot carried example, since its source did not
change, and only its two claims that those four are Grove's whole vocabulary
are reframed. The grove-loop pin test is renamed to
`the_slots_are_the_vocabulary_and_prompt_is_the_required_one`, with its book and
structure-spec citations. The overview book gains a `standalone-slot-refusal`
fragment for the new function.
