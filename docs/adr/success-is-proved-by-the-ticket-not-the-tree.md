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
commit; a finish completes only after a transition of its own transaction has
taken the task root away, and every transition that takes it away does so only on
a proven result; and while the commit stands proven Grove never puts the pinned
task root back.

**Over Grove's own steps, and not over the repository either.** The removal
clause named the quarantine rename until `honest-classification-k85`, which is
the incumbent's artifact standing in for the role it plays
([`a-shared-safety-claim-names-the-role-not-the-artifact`](a-shared-safety-claim-names-the-role-not-the-artifact.md));
and the *proven commit* half was read off the repository at the moment the claim
was evaluated, which is the same mistake this record is about, one operand over.
The correlation ticket survives the destruction of Grove's own artifacts — that
is `FN-03` — but it does not survive the **operator**, who may drop the result at
any time, so a finish that had done everything right went red the moment they
did. The proof is recorded where Grove acts on it, at the step that takes the
root away.

`FN-20` — *a leftover artifact is garbage, never a receipt* — is this record's
converse, and the pair is complete only together: `FN-20` says no artifact the
transaction leaves behind is evidence, and this says no **absence** it leaves
behind is either. `FN-03` says the ticket survives the destruction of every
artifact the transaction owns; this adds that it must also survive the
**re-creation** of one, because a name is not an artifact.

**None of this makes the leftover invisible, and the distinction is easy to lose
— this record lost it once itself.** A standing quarantine is evidence that Grove
has **work outstanding at a name it reserves**: the contract's state table
classifies that disk `Reserved(Quarantined)`, ahead of `Absent`, precisely so the
next invocation does not read a free task-root name as a fresh grove. What it is
never evidence of is a **disposition** — neither that a finish happened nor that
one did not.

This paragraph first read *a standing quarantine is evidence that a finish is
unfinished*, which is this record's own error committed against itself: after
`FN-22`'s fourth revalidation point returns `Committed` unchanged the finish is
`Applied` with the quarantine still standing, and the shipped protocol returns
success there even when disposal fails. Under that reading one disk was
simultaneously a proven success and evidence of an unfinished transaction.
`finish-scope-k75` found it and `finish-scope-k76` repaired it, in the contract's
class sentence and here. **The rule is the one this record already states: the
tree answers questions about names, and the ticket answers questions about
outcomes.** Reading either onto the other is how one claim comes to forbid what
another requires.

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

## What enforces it

**The shipped retry path, in `tests/finish_lifecycle.rs`**, which decides both
directions this record forbids reading off the disk:
`rootless_finish_retry_refuses_when_no_teardown_result_exists` drives a
repository with no task tree at all and requires the refusal to name the
attempt — an absent `.grove/` is not a success, which is the false-success half;
`rootless_finish_retry_names_the_message_it_required_and_the_one_it_observed`
holds that the teardown commit's identity is its **message**, checked before any
structural test, so a repository whose shape happens to look right is still told
which commit it is missing; and
`rootless_finish_retry_refuses_a_teardown_result_from_another_finish_attempt`
runs two attempts and refuses the second on the first's result. That is the
correlation ticket doing the work the tree was never allowed to do.

**The model that found it has been retired.** `crates/grove-finish/models/finish.als`
— which falsified all three formulations of `FN-28`'s conjuncts named above, each
by exactly one trace — was deleted at `delete-finish-models-k30`, and the
catalogue those conjuncts were stated in, `docs/specs/semantic-contract.md`, at
`delete-formal-models-k29`. The three falsified formulations stay named in the
text because what they cost is the argument: each is what an implementer reaches
for, and each is wrong for the one reason this record states. The decision
survived the instrument that found it, which is the outcome that campaign was run
to test, and `docs/formalism-findings.md` keeps the record of how it was found.
