# creator-reference-k38 — brief

## Goal

Ship the methodology amendment that makes a finishing session name its run on
the reviews of each producer it finishes, or remove the line when it has no
run. Ship it in the same release as the dispatcher, with the conformance rows
and composition-guidance pins the creator-reference ADR says go with it. Then
prove the Grove creator lifecycle end to end with fake producers that follow
the documented convention.

## Done when

- The three methodology rules the ADR names are amended as it states, and only
  as far as it states: `body-carries-no-launch-metadata`,
  `retirement-is-filename-only` with the node-close steps and their
  `node-close-four-steps` row, and `diversity-is-the-configs`.
- The glossary, `TASK-FORMAT.md`, `docs/ARCHITECTURE.md` and `docs/USAGE.md`
  each say no code reads the relationship lines. Each such statement is scoped
  to Grove's own code.
- The conformance manifest rows and the composition-guidance pins cover the
  amended rules. The conformance runner, its suite and the pins pass, and each
  new pin is seen to fail against the unamended wording.
- The skills Grove provisions for Codex carry the same amendment as the
  plugin.
- Grove's launch-boundary suite passes these creator lifecycle cases:
  - retiring and reordering the producer between its launch and its review's
    launch leaves the review's creator unchanged;
  - a pre-cut review of a decomposed producer carries the run whose retirement
    closed it through a multi-level close, and selects although that run's task
    identity is the child's;
  - a dispatched producer attempt followed by a direct-harness finish leaves the
    pre-existing review with no `**Creator:**` line, and that review refuses with
    the declaration remedy.
- The usage documentation and configure-grove explain the `**Creator:**`
  conventions, the declaration remedy, and how a review attaches findings to
  the producer's run as an observation.

## Decomposition

1. `creator-methodology-k39`: the amendment, rescoped statements, conformance
   rows, pins and provisioned skills.
2. `creator-lifecycle-k40`: the Grove lifecycle cases, the creator
   documentation and the node review.

## Pointers

- ADR: `docs/adr/a-review-carries-its-creator-reference.md` carries the
  amendment text, including the node-close step. Once it ships, its "ship with
  the dispatch implementation" wording becomes current state.
- Spec sections: `#identity-and-creator`, `#grove-integration` and the Grove
  launch-boundary row of `#test-seams`.
- The glossary entries `Original creator` and `Creator reference` are already
  written in design terms. Make them current.
- Instruments: the conformance runner and its manifest under `plugins/grove/`,
  and the composition-guidance suite in `grove-llm`'s tests.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Retiring the node's last leaf closes it. That leaf cuts the review as the node's
sibling, directly after it and ahead of the next increment. Inside the node, a
review would keep the node open, so no session would finish the producer it
reviews (spec `#identity-and-creator`). The amendment changes rules
every future session follows. Conformance rows can prove that wording is
present on a loaded path, but not that the steps are coherent where a session
meets them, at retirement and at node close.

## Notes

The methodology reaches Claude Code sessions through the marketplace after the
release push, and reaches Codex through the binary. So this grove's own
sessions keep the cached, unamended skills. Verify the amendment by reading the
files and running the instruments. The next session's behavior is not
evidence.

`creator-methodology-k39` landed the amendment. The finishing session's step
is `references/retire.md`'s *Naming your run on what you finish*, and step 4
of the node close carries it. `TASK-FORMAT.md` states what the line is. The
step's conformance row is `finishing-session-names-its-run`, so a second
shipped skill file that restates its lead sentence fails the runner. A fake
producer follows that section. Owner documentation points to it, and the four
dispatch-side statements `creator-lifecycle-k40` owns are false until that
leaf makes them current.
