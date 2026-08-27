# Success is proved by the correlation ticket, never by the tree

A finish succeeds when the exact attempt-bound deletion commit is proven and
Grove itself has taken the task root away and not put it back. **Neither operand
may be read off the disk.** The only durable evidence that a finish succeeded is
the correlation ticket the deletion commit carries — `FN-03`'s subject — and no
artifact the transaction leaves behind, and no absence it leaves behind, is a
receipt for it.

## The trade-off it settles

The obvious success test is a `stat`: after a finish, `.grove/` is gone. It is
cheaper — no version-control read on the success path, no ticket lookup, no
lane-specific reasoning about what "the exact result" means on a working-copy-as-
commit lane — and it is what an implementer reaches for.

It is wrong in **both** directions, and the window is not narrow. `FN-19` settles
a proven commit by renaming the whole task root into the quarantine in one step,
after which the task-root **name is free and the world owns the namespace**:

- **A false failure.** Anything may be created at that name — including a new
  grove someone simply started using again. A finish that decided by the name
  would report failure over a completed deletion.
- **A false success.** The quarantine can be moved back over the task root's
  name, and on a lane where root identity is observable it can carry the
  quarantined root's own identity. A finish that decided by the *quarantine's*
  presence, or by the root's absence at the moment it looked, would report
  success over a tree that is still there.

`crates/grove-finish/models/finish.als` falsified three separate formulations of
`FN-28`'s conjuncts — `after finishSucceeded`, `implies finishSucceeded`, and
`Txn.pinned not in Root.rid'` — and each was falsified by exactly that trace and
by nothing else.

## What follows

`FN-28`'s operands are therefore stated over **Grove's own steps** rather than
over the disk: the step that completes a finish is reached only over a proven
commit; the only transition under a transaction that takes the task root away is
the quarantine rename, and it does so only on a proven result; and while the
commit stands proven Grove never puts the pinned task root back.

`FN-20` — *a leftover artifact is garbage, never a receipt* — is this record's
converse, and the pair is complete only together: `FN-20` says no artifact the
transaction leaves behind is evidence, and this says no **absence** it leaves
behind is either. `FN-03` says the ticket survives the destruction of every
artifact the transaction owns; this adds that it must also survive the
**re-creation** of one, because a name is not an artifact.

**None of this makes the leftover invisible, and the distinction is easy to lose.**
A standing quarantine is evidence that a finish is **unfinished** — the contract's
state table classifies that disk `Reserved(Quarantined)`, ahead of `Absent`,
precisely so the next invocation does not read a free task-root name as a fresh
grove. What it is never evidence of is that a finish **happened**. Those are
different questions with different answers, and reading either onto the other is
how one claim comes to forbid what another requires.

This is also why whole-root destruction cannot move behind a filesystem library's
interface
([`root-lifecycle-stays-with-its-receipt`](root-lifecycle-stays-with-its-receipt.md)):
a component whose whole world is `<root>` and `<root>/..` has nowhere to put a
receipt that is not the leftover.

**Reopen** if Grove gains a durable per-attempt record outside version control
that an operator and a recovery both consult — a receipt in a state directory
rather than in history. Then success has a second piece of evidence that a
re-created name cannot forge, and reading the tree becomes admissible as a
corroboration. It does not become admissible as the proof.
