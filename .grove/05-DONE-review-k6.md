# review-k6

**Kind:** work

> **Discipline: `review`** — a fresh-context adversarial read, per the canonical
> five-kind taxonomy (ADR *task-kind-taxonomy*). The `**Kind:**` line above says
> `work` only because the *installed* CLI is v9.1.0, which still gates reads to the
> old `work`/`planning` pair and would error on `review` (`main` fixes this by
> degrading on read; it is simply unreleased). Drive this leaf as a review.

## Goal

Try to **break** the pruning change before it ships. Assume the author was
overconfident — they were the same context that designed it.

## Context

The change spans `prune-verb-k2` (CLI) and `methodology-k3` (`content/`,
`CONTEXT.md`, `docs/adr/`). Read the diff, not the intent. Do **not** read
`plan-k1`'s running log first — its reasoning is exactly the bias this leaf exists
to escape.

## Done when

Each of these is either confirmed sound *with evidence*, or reported as a finding:

- **The key counter is monotonic.** Construct the adversarial case: prune the
  highest-keyed leaf in a tree, then `leaf-add`. Does the new leaf collide with the
  pruned key? This is the defect the whole grove exists to close (issue #2) — prove
  it closed, don't assume it.
- **`pick` cannot be jammed.** A tree of only-`ABANDONED` leaves; a mixed
  `DONE`/`ABANDONED` node; an `ABANDONED` leaf inside a node whose siblings are
  live. Does the walk skip correctly and does the finish trigger fire exactly when
  it should — and never when live work remains?
- **`leaf-prune <node>` cannot destroy work.** It must never touch a `DONE` leaf,
  never touch a leaf outside the named subtree, and never accept the grove root.
- **No slug can impersonate a state.** `validate_slug` reserves `ABANDONED`.
- **The docs and the code agree.** `CONTEXT.md`, ADR *pruning*, ADR
  *task-tree-scheme* and `content/SKILL.md` describe the behaviour the tests
  actually assert. A doc that overstates the code is a finding.
- **The ADR set is still minimal and coherent.** No dangling citations; nothing
  restated in two places that can drift.

## Notes

Report findings; **fix nothing silently**. Each finding is one of: a real defect
(fix it), a real trade-off (accept it visibly), or noise raised for want of context
(note it and move on).

If a finding is big enough to be its own session, `leaf-insert` it ahead of
`release-k5` rather than absorbing it here.
