## What one commit contains

One task is **one focused commit**: the artifact, whatever the grow verbs wrote,
and the `DONE` rename that retires the leaf, together with anything the cascade
above promoted or added. Everything in that list is written by Retire, which is
why Retire comes first — the message cannot name a node you have not yet closed.

## Where the boundary falls, in git and in jj

That commit is also the boundary the *next* session starts from, and git and jj
reach it differently — the same asymmetry the tree verbs already carry (`git mv`
there, a plain rename under jj). In **git** the working tree is not history, so
one `git commit`, taken once the rename has landed, both records the task and
leaves the next session a clean tree. In **jj** the working copy *is* a commit:
this session's edits are already in `@`, so `jj describe -m` records the task but
leaves that change open, and the next session's first edit is snapshotted into
*this* task's commit. **Seal it** — `jj new` after describing, once the rename
has landed (`jj commit -m` is exactly those two) — so the next session opens on
its own empty change. Sealing is the last thing the boundary does, for the same
reason Retire precedes it: a `jj new` taken early puts every later edit —
the rename, a promoted ADR, a `leaf-add` — into the *next* task's change. An
unsealed change is expensive to unpick afterwards:
`jj split <fileset>` cannot separate a file both tasks touched, leaving the
operation log as the only way back. **The boundary above is the whole of what a
grove session needs, on either lane, and it binds on its own** — the
`linkuistics:using-jujutsu` skill deepens the jj lane rather than completing this
rule, so a checkout without the plugin commits correctly from this file alone.

Which lane you are on is not yours to re-derive: the driver states this working
tree's version control, and that statement is definitive.

## Why the handle, and not the position

Positions and paths move under renumber and reorder — a `leaf-insert` shifts
every later sibling, and a `leaf-decompose` turns a leaf's path into a
directory — but the `<slug>-k<key>` handle is permanent, assigned once and never
reused. So a commit message naming the work item by its handle stays meaningful
after restructures, and one naming `04-impl-extract` names a coordinate that may
already belong to something else (task-tree-scheme §5). Name each node the
cascade closed the same way, alongside the leaf's own.

**Do not compensate for the bare stem with a subject convention.** A `review:` or
`impl:` prefix would restate, unvalidated, the kind the leaf's own filename
carries — and that filename survives in the diff forever, because Retire-then-Commit
puts the `DONE` rename in the task's own commit and teardown removes `.grove/`
from the tip rather than from history. `git show --stat <commit>` names it
whether or not rename detection fires.
