# Grove does not stage its own renames

Every entry Grove moves on disk moves with `rename(2)`, and Grove touches no
version-control state afterwards. Jujutsu snapshots the working copy on its next
command, so the mark is whole the moment the verb returns:

```
R .grove/{04-impl-marking-k32.md => 04-DONE-impl-marking-k32.md}
```

is `jj status`, and the commit that follows records exactly that.
`tests/leaf_ops.rs` asserts it rather than describing it.

## The trade-off

`ordinal-fs-tree` performs the rename, and the architecture settled that it
detects no repository and requires no tool on `PATH`
([`docs/ordinal-fs-tree/ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md)).
So the version-control-aware move Grove's `src/tree_rename.rs` carried had
nowhere left to live inside the operation. The question was whether Grove buys
something back from outside it.

It does not, and what that buys is that Grove's tree layer stops knowing about
repositories at all: no subprocess in the mutation path, no trackedness probe,
and no operation that behaves differently depending on whether a tool is
installed. Grove still runs Jujutsu — for the finish teardown, where a *commit*
is the point — and that is a whole-transaction operation naming its own fileset.
The distinction that survives is between committing, which is version control,
and renaming a file, which is not.

The store now creates and deletes tree **roots** as well as moving entries
within one, and that widening does not move this decision — the reasoning above
never turned on the effect being a rename. A `mkdir` and an `rmdir` are working-
copy changes exactly as `rename(2)` is: Jujutsu snapshots them on its next
command, so `jj status` shows an added or removed path the moment the verb
returns, and there is still nothing for Grove to stage afterwards. What the
decision rules out is a *repository-aware* mutation path, and a store that
creates a root without detecting a repository is one more operation that does not
have one.

Dropping the plain-Git lane ([*jj is the only lane*](jj-is-the-only-lane.md))
made the decision cheaper rather than changing it: there is no index left for an
unstaged rename to be half-recorded in, so the pre-commit window this used to
cost has closed on its own.

## Considered options

- **Have Grove take the commit itself** for the marking verbs, the way the finish
  transaction does. Rejected because it inverts the methodology:
  `content/references/commit.md` puts one focused commit at the end of a task,
  carrying the artifact, the grow verbs' output and the `DONE` rename
  *together*. A verb that committed its own rename would split every task into
  two commits and take the boundary decision away from the session that owns it.
- **Keep `src/tree_rename.rs` alive for the marking verbs only**, calling it
  after the library's rewrite to fix up whatever the repository holds. Rejected:
  the library has already moved the file, so the fix-up would be a second,
  differently-shaped operation bolted onto a completed one — and on the only lane
  Grove drives there is nothing left for it to fix.

## Why this is hard to reverse

Not the decision — the deletion, which has happened. `grove-flip-k28`'s contract
stage removed `src/tree_rename.rs` once its last caller had gone, and it was the
only place Grove had ever known how to move a tracked entry. Restoring it means
rebuilding that dispatch against a library that now performs the rename itself,
so the reinstated code would have to run *after* the move rather than instead of
it — a different operation with a different failure mode, not the one that was
deleted. The observable it protects, meanwhile, has already propagated: every
session's commit habit is formed against the working copy this decision produces.
