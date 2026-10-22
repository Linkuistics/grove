# ravel-child-plan-k15

**Reviews:** ravel-child-plan-k4

## Goal

Read the planned subtree of `ravel-child-plan-k4` adversarially. Report
findings and make no fixes.

## Context

- The producer is `ravel-child-plan-k4`. Find its commit from the handle. The
  planned leaves are `ravel-floor-k6` to `book-specs-k14`.
- The root brief: *Design* and *Requirements*. Every item of *Design* must
  belong to exactly one leaf.
- The specific doubts:
  - Each leaf must keep `bash scripts/check.sh` green on its own. The
    `grove-loop` and `overview` books reproduce whole source files, so a code
    change that lands without its chapter edit breaks `book-check`. Check the
    order of the leaves against this rule, and check that no leaf holds a
    stale quote.
  - The expand step keeps `keyed-launch` alive while two callers move. Check
    that the conversions of `Argv` between the leaves compile at every step.
    Check that the two latches never meet in one process.
  - `lifecycle-job-k9` is the largest leaf. Check whether it fits one session,
    and whether its seam is right if it must decompose.
  - The records are split across three late leaves. Check that no record that
    states the reversed decision belongs to no leaf. A sweep of the tree for
    `keyed-launch`, `keyed_launch`, "survivor", "surviving" and "group" is
    the instrument. Follow the controls that `references/execute.md` requires.
  - Check that each leaf names the Ravel tasks that it must pass.

## Done when

- The findings are written in this file, each with a severity and a
  `path:line` coordinate or a leaf handle.
- Cut an integration leaf only if a finding warrants action. Use
  `leaf-insert` at the first live sibling, as `references/decompose.md` directs.

## Notes

- The producer finished without a provider receipt, so this leaf carries no
  `**Creator:**` line.
