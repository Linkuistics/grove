# jj is the only lane

Grove drives Jujutsu and refuses everything else. A working tree with no `.jj/`
directory at or above it is refused *before any mutation*, by one gate —
resolving a workspace through the version control seam, `crates/jj-workspace` —
with the command that fixes it:

```
not a Jujutsu working tree
  looked for a `.jj` directory at and above: /path/to/tree

Make the tree jj-enabled and rerun:
      jj git init --colocate     # an existing Git repository, history kept
      jj git init                # no repository here yet

Nothing was created or changed.
```

Nothing downstream branches on which version control owns the tree, because
nothing else can own it. A `.git` beside a `.jj` is a colocated repository and is
jj's business: Grove never reads it, never spawns `git`, and makes no promise
about the colocated index — which is why the index backup that used to guard it
is gone rather than kept for one lane.

## The trade-off

The safety principle this project is built on is that **the version control
system owns safety, history and transactionality** — Grove takes commits and
implements no transactions of its own. jj can honour that: it snapshots the
working copy before every command, and its operation log *is* the transaction
record, so a Grove operation that goes wrong is undone with `jj op restore`
rather than from machinery Grove wrote.

Plain Git cannot. It has no automatic snapshot and no operation log, so every
guarantee the principle assumes had to be hand-built for that lane and only that
lane: a durable record of the pre-operation state, a proven rollback, an index
image to restore unrelated staged bytes, an empty hooks directory so a user hook
could not mutate what no index image would restore. That machinery — roughly
four thousand lines across the finish commit, the index-backup family and its
auxiliary-replacement protocol — existed to give one lane what the other gets
from its own design.

Dropping the lane is what makes the principle true rather than aspirational, and
it is what lets the version-control seam be *fully* domain-free and state its own
precondition as a refusal instead of a dispatch.

The cost is real and is accepted: a plain-Git checkout cannot run Grove until
someone types `jj git init --colocate`. That command preserves the Git history
and leaves the repository usable by every Git tool, so the cost is one command
and no lost work — but it is a hard requirement, not a suggestion, and Grove will
not proceed without it.

## Considered options

- **Keep both lanes.** Rejected: it is the status quo whose price this record
  exists to state. Every guarantee has to be written twice, the second writing is
  the one nobody exercises, and the seam that ought to be the smallest of the
  five modules stays the largest.
- **Narrow the safety principle to *where the version control system can***, so
  Git keeps its hand-built transaction and jj uses its own. Rejected, and this is
  the alternative that had to be argued rather than dismissed: it is coherent,
  and it keeps every existing user. It fails on what it preserves — the finish
  transaction survives on one lane, so the module that was supposed to shrink to
  "take a commit" keeps its rollback machinery, its index images and its
  auxiliary protocol, and the *principle* becomes a statement about jj rather
  than about Grove. A principle with a lane-shaped exception is a dispatch
  wearing a principle's clothes.
- **Refuse plain Git, but repair it automatically** by running
  `jj git init --colocate` on the operator's behalf. Rejected under principle 2:
  an anomaly stops with a good message and is not repaired in code. Colocating a
  repository changes what every other tool in that checkout sees; it is the
  operator's call, and the refusal names the exact command so making it costs one
  line.
- **Refuse plain Git at the point of use** rather than at a precondition gate —
  let the tree verbs work and fail only at `finish-commit`. Rejected: it lets an
  operator build a whole grove in a tree that can never finish, and a refusal
  that arrives when the workstream is already complete is the most expensive one
  there is.

## Why this is hard to reverse

Not the gate, which is a dozen lines — the deletions behind it. Restoring the
Git lane means restoring the plain-Git finish commit and its proof, the
`GitIndexBackup` family, the auxiliary artifact-and-marker replacement protocol
with its ten checkpoints, the empty-hooks-path rule, the gitfile indirection in
workspace resolution and the three-variant control marker it fed, plus the
`git ls-files` trackedness probe and the `GIT_INDEX_FILE` scrubbing that probe
made load-bearing. Each was written against a shape that no longer exists: there
is no `Vcs` enum to dispatch through and no second control-directory derivation
to resolve to, so the restored code would be a new implementation of the same
idea rather than the one that was removed.
