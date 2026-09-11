# grove-migration-k11

## Goal

Migrate the remaining groves under `~/Development` and prove that every tree,
including this one, opens with the installed release while retaining its work.

## Context

- Depends on the completed installation, plugin refresh and this-tree cutover
  at `node-cutover-k10`. Start under the new driver; confirm installed versions
  before touching another tree. The interview authorized this workflow in
  `plan-k1` decision 7; nothing is added to published Grove tooling.
- Known trees (re-enumerate at execution time to include any added meanwhile):
  `~/Development/APIAnyware.add-ocaml-target/.grove`,
  `~/Development/InTheLoop/.grove`, `~/Development/Writegood/.grove`,
  `~/Development/grove.gh-issue-12/.grove`, and
  `~/Development/grove.dont-put-item-title-in-folder-names/.grove`.
- The last tree was converted at cutover and is checked again here. The
  `APIAnyware.add-ocaml-target` tree is the measured deep-path case. Root
  `FORMAT` files containing `session-kinds-v1` were observed in that tree,
  `Writegood` and `grove.gh-issue-12`; recheck contents before disposition.
  Release integration also materializes `~/Development/grove/.grove` in the
  default workspace. Cutover converts it by advancing to the final change;
  verify that copy again here, without treating it as an independent workstream.

## Done when

- The other drivers remain stopped while their trees are renamed. Reconcile
  their current inventory with cutover's preflight; preserve concurrent user
  work and do not reuse a stale mapping. For a missing or unexpectedly shaped
  tree, establish what happened rather than silently counting it migrated.
- A temporary, non-shipping workflow preflights all destinations and exact
  source entries before mutation, preserves a recoverable snapshot, renames
  node files and directories bottom-up, and handles partial completion without
  overwriting files or converting a tree twice. It never infers a slug from a
  body. Unexpected shapes are reported for specific recovery, not guessed at.
- Each resulting tree uses `_BRIEF.md` at the root and
  `NN-k<key>/_<slug>.md` for nodes. Compare leaf names, keys, outcomes, content
  digests, sibling order and node-file bytes item by item before and after.
  The deep-path case has shorter actual byte paths and no lost descendants.
- Inspect every root `FORMAT`, including in newly discovered trees. Remove it
  only if its contents establish that it is the obsolete Grove witness,
  retaining recovery evidence;
  if it contains unrelated material, preserve it as foreign and state why.
  It must not supply ownership evidence, routing or permission to overwrite.
- Run the installed new `grove-llm` directly from each workspace, with
  `GROVE_SIGNAL_FILE` cleared for probes into other groves. `pick` must open
  the whole tree and yield the expected next permanent key, or its valid
  terminal result; check a representative deepest node resolution and brief
  chain too. Include this tree in verification, accounting for this migration
  leaf still being live until its own retirement. Do not launch a harness as
  an acceptance probe.
- Record a concise per-tree acceptance result and the `FORMAT` disposition
  in this leaf's running log. Preserve unrelated edits in each workspace and
  make any migration-only jj boundary identifiable. Only verified groves may
  be restarted by their operators.
- No migration code, compatibility parser or persistent status artifact ships.
  No source/book change is expected; if one becomes necessary, return it to a
  precise code leaf with its book obligations. Acceptance is opening every
  actual tree with the installed binary, not a new unit-test seam.

## Notes

This grove may now retire its last ordinary leaf; the driver owns the later
finish session. Use the normal retirement, jj seal and `grove-llm complete`
ending here, under the new driver. The cutover leaf's no-signal exception does
not apply to migration. Retain snapshots until their comparisons succeed;
temporary workflow files are not a release deliverable.
