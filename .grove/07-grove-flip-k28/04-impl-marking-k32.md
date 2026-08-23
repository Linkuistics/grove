# marking-k32

## Goal

Move `leaf-retire` and `leaf-prune` onto the library's `rewrite`. The first
mutation grove performs through the library, and therefore the leaf that answers
**question 1**: the version-control-aware move is gone, and something has to be
true instead.

Both verbs are one thing algebraically — an entry keeps its ordinal, its key and
its species, and only the opaque remainder of its name moves. That is `rewrite`'s
definition, and it is why these two go first: the narrowest mutation that touches
every layer.

## Context

- `crates/ordinal-fs-tree/src/fs/mod.rs` — `fs::write`, `WriteGuard::rewrite`,
  and the fact that a mutating method **consumes** the guard.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, *Mutating* and *Refusals* —
  `RewriteSpeciesChange` in particular: parts implying a different species are
  refused, which is exactly what protects a node directory from being marked.
- `src/tree_lifecycle.rs` — `leaf_retire`, `leaf_prune`, `PruneResult`.
- `src/tree_rename.rs` — the whole module, and its header, which is the best
  statement of why it dispatches on trackedness (grove issue #3).
- `tests/leaf.rs`, which stands up a real git repo so the verb's `git mv` calls
  have tracked entries, and `tests/jj_tree_verbs.rs`, which asserts the rename is
  plain and git's index comes out untouched. Also `leaf_ops`,
  `reviewed_producer_lifecycle`.
- The node brief's *four inherited questions* table and the root brief's *What
  the flip inherits from dropping the version-control-aware move*.

## Done when

- `leaf-retire` and `leaf-prune` mark through `rewrite`, and `tree_rename` has no
  callers left from these two verbs.
- **Question 1 is answered and recorded.** The options the root brief names are:
  grove re-stages after a library rename, grove accepts the changed `git status`,
  or something else. Whichever it is, the record says what an operator's
  `git status` shows between a grove verb and the commit that folds it in — that
  is the observable the answer is about.
- The `git mv` assertions in `tests/leaf.rs` and `tests/jj_tree_verbs.rs` say
  what is now true, and the node brief's *Findings* records what they said before
  and why it changed. This test change is pre-authorised as question 1's
  consequence; it is not therefore unrecorded.
- `leaf-prune` on a **node** still does what its help promises — mark every live
  leaf in the subtree, leave `DONE` ones alone, refuse the grove root — or its
  contract changes deliberately and the change is written down. See below.
- The whole suite passes.

## Notes

**The atomicity problem is the real work here, and it is not obvious from the
verb.** `rewrite` consumes the write guard, so one guard is one operation, and
`leaf-prune` on a node subtree is *N* rewrites under *N* separate guards. Today
it is one critical section returning one `PruneResult`. Three ways out, and the
leaf owns the choice:

1. Accept it: pruning is HITL and rare, and a half-finished prune is visible in
   the tree because the marks are the state. Say so, and say what an operator
   does if it stops half way.
2. Re-plan it as one operation the library can express. It is not `append_many`;
   there is no `rewrite_many`.
3. Escalate: the library wants a batched rewrite. That is a change to a checked
   library — the model leads, so `operations.qnt` moves first — and it is a leaf
   of its own, cut here rather than done here.

Option 3 is not the default. It is listed because *discovering* the need for it
mid-session and doing it inline is the failure this note exists to prevent.

**jj is unaffected and that is not a reason to relax.** The root brief is
explicit: jj snapshots the working copy and grove already renames plainly there,
so nothing changes on this lane. The whole of question 1 is about the git lane,
and this working tree is jj — so the git behaviour has to be *tested*, not
observed. `tests/leaf.rs` already stands up a real git repo for exactly this.

**What is actually lost, precisely.** Both lanes commit byte-identical trees;
git infers renames at diff time by content similarity. What changes is the window
*before* the commit: a tracked entry renamed through the library leaves git's
index holding the old path, so `git status` shows a deletion plus an untracked
file rather than a rename, and a commit naming only the old path would record the
deletion alone. That last clause is the one with teeth — grove sessions commit,
and `references/commit.md` describes one focused commit per task.
