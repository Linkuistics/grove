# moves-handoff-k187

## Goal

Give owner-ravel-skill-inventory-k38 and discovery-rollout-k12 the concrete
results of every move. Check the Done when of repository-moves-k5 against
the delivered state.

## Context

Each earlier leaf of this node adds its rows to
`docs/repository-moves-record.md`. The owner design's "Cutover contract"
makes these moves order 2. Its "Ravel sources" table names ten sources.

`skill-marketplace-decision-k201` replaced the separate marketplace with
Homebrew installation. `homebrew-skill-delivery-k215` delivers it before this
leaf, and the root brief's "Skill installation" states the decisions.

## Done when

- The record has one row for each of the ten Ravel sources. Each row names
  the source ID, the repository URL, the source directory and the roots. It
  also names the published commit for the pin, the tree ID of the source
  directory and the `resourceDigest` of a clean checkout. Each row names the
  formula that installs the source. The superpowers and quint rows read the
  upstream pins of the owner design again. They record the commits that
  their formulae pin.
- Each baseline entry of class owner pack has a row that names its new
  source and root, or a recorded change. Enumerate the baseline. Do not
  count rows. No owner-pack skill is missing.
- Each baseline entry outside the pack table has a stated route. That covers
  the third-party packs, the stale copies, the dangling entries, the
  tool-managed entries, the capability-only plugins and the platform skills.
- The record lists the preserved resources and the tool and hook
  requirements of each source. It also lists the native residue plugins, the
  keg and local marketplace file that hold each, and the registration step
  for the switch.
- The Grove line holds the `grove-core` candidate and is pushed to the
  bookmark `gh-issue-25`. The record names the candidate commit and the tree
  ID of `plugins/grove` at it.
- Grove's record of pack commits, the formulae and this record name the same
  commits. Otherwise this record names each difference and its reason.
- The record lists each reference that this grove cannot edit, such as the
  paths in paused task trees, for rollout's paused-grove text.
- Every pack passes alone. The aggregate of core with every pack passes.
  Grove's `bash scripts/check.sh` passes.

## Notes

If a check finds missing move work, add a leaf for it in the move's node.
Do not absorb a move into this leaf.
