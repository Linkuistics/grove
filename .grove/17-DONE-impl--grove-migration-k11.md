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

## Decisions (running log)

1. On 2026-09-13, both installed binaries report 21.0.0. Process inventory
   shows only this session's Grove driver; cutover decision 12 supplies the
   retained stop/migration authorization. Fresh discovery finds seven trees:
   APIAnyware.add-ocaml-target, InTheLoop, Modaliser, Writegood,
   grove.gh-issue-12, this workspace and the default Grove workspace.
2. Use an external temporary workflow with fresh per-entry mappings and
   recoverable archives, checking bytes, file types, keys and sibling positions
   before and after conversion. Preserve existing external workspace changes
   in their current jj changes, then create separate migration changes. Both
   Grove copies already use the installed grammar and need verification only;
   advance the clean default copy to this leaf's sealed result at the end.
3. All five external trees match the approved cutover snapshot file by file;
   no new tree or unexplained change was found. The two Grove copies differ
   through the recorded cutover and this running log. The temporary preflight
   preserves existing ordinal gaps in grove.gh-issue-12; migration does not
   renumber siblings. Scratch probes use canonical paths to account for macOS's
   `/tmp` alias. Neither workflow adjustment changed a live tree.
4. Fresh archives, exact mappings, per-entry SHA-256 inventories, installed
   reader outputs and jj boundaries are retained outside the repository at
   `/Users/antony/.local/share/grove-cutovers/grove-migration-k11-20260913/`.
   Every archive was compared with its source, every resulting entry with its
   mapped original, and sources were rechecked before mutation. Node keys,
   sibling positions, leaf names/outcomes and every retained file's bytes match.
   The workflow and input manifest digests stayed fixed during execution.
   Scratch controls verify partial-file-rename recovery, identity on a second
   conversion and refusal of competing root files without overwriting them.
5. Installed 21.0.0 acceptance from each actual workspace, with
   `GROVE_SIGNAL_FILE` cleared, passes `pick`, deepest-node handle resolution
   and a descendant's exact root-to-leaf node-file chain:

   | Tree | Next key | Deepest node checked | FORMAT | Migration commit |
   | --- | --- | --- | --- | --- |
   | APIAnyware.add-ocaml-target | k870 | l-k795 | obsolete witness removed | b4b83320 |
   | InTheLoop | k50 | adoption-lifecycle-k36 | absent | d716daef |
   | Modaliser | k110 | custom-native-storage-semantics-k94 | absent | b6d781f8 |
   | Writegood | k22 | inspection-pack-k43 | obsolete witness removed | 9bf13a77 |
   | grove.gh-issue-12 | k27 | persuasion-micro-test-k25 | obsolete witness removed | a86efa40 |
   | grove (default copy) | k11 | distinguished-names-k6 | absent | already converted |
   | this workspace | k11 | distinguished-names-k6 | absent | already converted |

   Each removed FORMAT contained exactly `session-kinds-v1` plus newline;
   originals remain in the archives. No unrelated FORMAT material was found.
   APIAnyware's longest actual byte path shrank from 1013 to 310, and
   Modaliser's from 783 to 251, with every descendant accounted for.
6. External migration changes contain only `.grove/` paths, each directly
   above its preserved pre-migration change, and are sealed with empty working
   changes above them. The other drivers remain stopped; operators may now
   restart these verified groves with 21.0.0. No harness was launched as a
   probe. No source, book, product parser or ADR change is needed. Retire this
   root-level leaf normally, seal its acceptance log and retirement together,
   then advance the clean default copy and verify both copies' terminal reads.
   The driver owns the subsequent finish session.
