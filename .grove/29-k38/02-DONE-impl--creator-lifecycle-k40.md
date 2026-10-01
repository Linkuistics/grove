# creator-lifecycle-k40

## Goal

Prove the original-creator mechanism across Grove's lifecycle at the launch
boundary. Fake producers follow the documented `**Creator:**` convention, and
dispatched reviews resolve the right creator through retirement, reordering,
multi-level node close and a direct-harness finish. Document the conventions
and remedies for owners.

## Context

The spec's Grove launch-boundary row lists these cases. A fake producer is a
fake harness that behaves as the amended methodology tells a session to. It
reads `HARNESS_DISPATCH_RUN_ID`, writes or removes the review's line, and
retires or closes through `grove-llm`. A real session's compliance is the
conformance rows' concern, not this seam's.

`review-policy-k58` qualified four dispatch-side statements until the
methodology ships. Each says Grove's sessions do not yet write the line, so
the owner does. They are, in `crates/harness-dispatch`: the README's Grove
review section, in its form description and "When the creator line is
missing"; `worker/examples/grove-review.ts`'s header; and
`worker/grove/index.ts`'s header and `creator_line_missing` remedy. The
remedy's command-seam assertions are in `tests/grove.rs`. Make each current
with this leaf's documentation. Keep the owner's part for a producer with no
run, and the warning that a run reference is its writer's word.

## Done when

- **Retirement and reordering.** A dispatched producer writes its run on a
  pre-cut review. It is then retired, and the tree reordered before the
  review launches. The dispatched review selects from the named run's recorded
  provider, and a changed current mapping does not alter it.
- **Decomposed producer.** A review cut before its producer decomposed ends up
  carrying the run whose retirement closed the node through a multi-level
  close. It selects although that run's task identity is the child's handle,
  and inspection shows both.
- **Direct-harness finish.** A dispatched producer attempt followed by a
  direct-harness finish leaves the pre-existing review with no `**Creator:**`
  line. That review refuses with the declaration remedy. Adding
  `**Creator:** declared <provider>` then admits a different-origin reviewer.
- **Findings after teardown.** A review attaches an observation to the
  producer's run after `.grove/` is removed, and `record show` returns it.
- The usage documentation and configure-grove explain the `**Creator:**` forms,
  who writes or removes the line, the declaration remedy for artifacts with no
  run, and why a wrong but existing run is not detected at launch.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl dispatch-documentation-k41 creator-reference`, targeting the first root
  entry after this node (today `dispatch-documentation-k41`). Give it
  `**Reviews:** creator-reference-k38`, and write the node brief's review doubts
  into its body.

## Decisions (running log)

**The cases run the shipped example whole.** They are in
`crates/grove/tests/loop_driver.rs`, under a `Lifecycle` fixture over the
existing `Dispatch` one. The personal policy is `export { policy } from
"harness-dispatch/examples/grove-review"`, and the two wrapper programs its
catalog names are on the driver's `PATH`, each exec'ing the fake harness. A
catalog of the test's own would have tested a policy no owner is handed.

**A fake session follows `references/retire.md`, not the test's convenience.**
It reads its handle from the mandate sentence, resolves it with `grove-llm
resolve`, and retires, decomposes, inserts and signals through `grove-llm`.
`name_run` settles the line on every live review whose `**Reviews:**` line
names a handle. `finish` retires the leaf, then walks up while an ancestor node
has no live leaf, naming each such node's handle. Each case is one named test:
retirement and reordering, the decomposed producer, the direct-harness finish,
and findings after teardown.

**The dispatched attempt names its run, then dies before it retires.** An
attempt that wrote nothing would leave the direct finish nothing to remove, and
the case would pass with a fake that never removes. The owner then declares
`anthropic`, the direct harness's provider. The attempt's `openai` run would
have selected the `anthropic` reviewer, so the reviewer the case asserts is the
one a surviving line would not have given.

**The decomposed case has a sibling that retires without closing the node.**
`lexer-k3` retires while `grammar-k4` is live, and the review still has no
line. Only the retirement of `tokens-k5` closes both levels. Without the
sibling, exactly one session ever wrote, and the case could not tell *the run
that closed the node* from *any run under it*.

**Teardown is the removal of `.grove/`.** The review session reads the
producer's run from its own line while the tree exists. The observation is
imported after the directory is gone, against that run, and the case first
shows the run has no observation. Driving a real `finish` session would test
`finish-commit`, which this seam does not own.

**The driver fixture scrubs `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`.** The run identity is ambient, as the channel is.
Under an ambient run and without the scrub, the direct-finish case failed, and
so did two cases that predate this leaf, each on `run_id == "<unset>"`. They
would have failed the first time this repository's own sessions ran under
dispatch. With the scrub, all pass under the same ambient value.

**The teardown case is also the ordinary chain.** Its producer cuts its own
review with `leaf-add`, writes the body, then names its run on it. The other
three cases use a review cut beforehand, which the spec's row asks for, so
without this one the launch boundary never ran the step's first clause, *the
review leaf you cut*. The order matters: a producer that names its run before
it writes the cut review's `**Reviews:**` line leaves that review with no
creator, and the review refuses.

**Each control was seen to fail.** Eight mutations, each failing its case and
then reverted against a digest of the file: the inserted session also naming
the review; the owner's remap a no-op; `finish` not walking the cascade; the
non-closing retirement naming the open node; a session with no run leaving
the line; the owner declaring the attempt's provider; the scrub removed; and
the cut review's body written after the naming step. The reworded
`creator_line_missing` remedy's pin in `tests/grove.rs` failed against the
unamended adapter before the worker was rebuilt from the amended one.

**The owner's account has one home and three shorter forms.**
`docs/CONFIGURATION.md` gains *A review's creator line*: the two forms and
their writers, the declaration, why a wrong but existing run passes launch,
and attaching findings. The usage guide's review-composition section,
configure-grove's *The creator line* and the dispatch README's Grove review
section each carry a shorter form and point there or to `references/retire.md`.
None restates the conformance row's lead sentence, and the runner passes.

**A hand-written run line stays, as the second remedy.** The spec gives the
declaration for a producer finished with no run, and is silent on a dispatched
session that omitted its step. The documents and the adapter's remedy lead
with the declaration. They keep `**Creator:** run <run-id>`, with that
session's own run and not an earlier attempt's, for that one case. The
provider is then still the recorded one, and the association is the owner's
word instead of the session's. The node's review is asked to challenge this.

**The node's review is inserted ahead of `package-entry-resolution-k52`.** The
body above named `dispatch-documentation-k41` as today's first root entry
after the node. k52 has since been inserted there, and the rule is the first
entry after the node, so that the review precedes dependent work.

**No in-session reviewer was spent.** The node's `review-impl` is scheduled,
and this leaf cuts it. This session has no `HARNESS_DISPATCH_RUN_ID`, so the
review it cuts carries no `**Creator:**` line.
