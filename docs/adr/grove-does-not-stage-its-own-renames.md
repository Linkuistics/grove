# Grove does not stage its own renames

Every entry Grove moves on disk moves with `rename(2)`, on every lane, and Grove
touches no version-control index afterwards. On the **Git** lane that changes
what an operator sees between a Grove verb and the commit that folds it in: a
tracked leaf renamed this way leaves Git's index holding the old path, so

```
 D .grove/04-impl-marking-k32.md
?? .grove/04-DONE-impl-marking-k32.md
```

is `git status`, where a `git mv` once showed `R  old -> new` already staged.
Nothing is lost at the commit — both lanes commit byte-identical trees and Git
infers renames at diff time by content similarity — provided the commit stages
the tree. A commit that stages only tracked paths (`git commit -a`, or a
pathspec naming the live name) records the deletion alone and never the mark's
arrival. `content/references/commit.md` says to stage the tree for that reason,
and `tests/leaf_ops.rs` asserts all three outcomes rather than describing them.

Jujutsu is unaffected: it has no index, snapshots the working copy, and Grove has
always renamed plainly there.

## The trade-off

`ordinal-fs-tree` performs the rename, and the architecture settled that it
detects no repository and requires no tool on `PATH`
([`docs/ordinal-fs-tree/ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md)).
So the version-control-aware move Grove's `src/tree_rename.rs` carried — `git mv`
for a tracked entry, `fs::rename` for an untracked one, plain on every jj
tree — had nowhere left to live inside the operation. The question was whether
Grove buys the old `git status` back from outside it.

It does not, and what that costs is exactly the pre-commit window. What it buys
is that Grove's tree layer stops knowing about repositories at all: no `git`
subprocess in the mutation path, no trackedness probe, no jj branch to keep in
step with the Git one, and no operation that behaves differently depending on
whether a tool is installed. Grove still runs Git and Jujutsu — for the migration
commit and the finish teardown, where a *commit* is the point — and those are
whole-transaction operations that name their own pathspecs. The distinction that
survives is between committing, which is version control, and renaming a file,
which is not.

## Considered options

- **Re-stage after a library rename** — `git rm --cached <old>` plus
  `git add <new>` where the entry was tracked. Rejected: it is the deleted
  primitive reassembled one layer up, with every part that made it costly. It
  needs the same trackedness probe issue #3 was about, since an untracked leaf
  has no index entry to move; it needs a jj branch of its own, because staging
  into a colocated repository's index is precisely what
  `tests/jj_tree_verbs.rs` forbids; and `git add` stages the file's *current
  content*, so a mark taken while the session is still editing the leaf would
  stage a draft the operator had not chosen to commit. `git mv` never did that,
  which makes the replacement not merely equivalent-but-elsewhere but worse.
  Reopen if a bulk mark ever has to be committed by a machine that cannot see
  the working tree.
- **Have Grove take the commit itself** for the marking verbs, the way the
  migration and the finish transaction do. Rejected because it inverts the
  methodology: `content/references/commit.md` puts one focused commit at the end
  of a task, carrying the artifact, the grow verbs' output and the `DONE` rename
  *together*. A verb that committed its own rename would split every task into
  two commits and take the boundary decision away from the session that owns it.
- **Keep `src/tree_rename.rs` alive for the marking verbs only**, calling it
  after the library's rewrite to fix the index up. Rejected as the worst of
  both: the library has already moved the file, so there is nothing for `git mv`
  to move, and the fix-up degenerates into the first option with an extra module.

## Why this is hard to reverse

Not the decision — the deletion, which has happened. `grove-flip-k28`'s contract
stage removed `src/tree_rename.rs` once its last caller had gone, and it was the
only place Grove had ever known how to move a tracked entry: the trackedness
probe, the `git mv` invocation, and the jj-first rule that a colocated tree
renames plainly all lived there and nowhere else. Restoring it means rebuilding
that dispatch
against a library that now performs the rename itself, so the reinstated code
would have to run *after* the move rather than instead of it — a different
operation with a different failure mode, not the one that was deleted. The
observable it protects, meanwhile, has already propagated: every session's commit
habit is formed against the status output this decision produces.
