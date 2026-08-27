# A witnessless root refuses what it cannot account for, and only Grove's own bytes prove the root is Grove's

Root initialisation makes the format witness visible last, so an interruption
leaves a present task root with no witness. The semantic contract had **one**
state for that condition — `PartialScaffold`, an exact closed subset of what a
fresh scaffold writes — and everything outside the subset fell through to
`Legacy`. A stray file beside Grove's own half-written scaffold therefore made
Grove read its own interrupted work as somebody else's legacy tree, one `crash`
and one `foreign-write` deep.

A witnessless root is now classified by an **ordered three-way test**, and the
order is what makes it fail closed:

1. **`PartialScaffold(Exact)`** — nothing but the fresh scaffold's own
   byte-exact entries. Completed, because every value the completion writes is
   fixed in advance.
2. **`PartialScaffold(Ambiguous)`** — otherwise, when the root carries at least
   one **root-init-exclusive** entry. Refused, mutating nothing.
3. **`Legacy`** — otherwise. Nothing proves this format's initialisation ran.

**Root-init-exclusive** means an entry only *this format's* root initialisation
writes: the reserved format temporary, and the first `requirements` leaf at
position 1 with key 1, canonically spelled, byte-equal to what a fresh scaffold
writes. **The root charter is deliberately excluded.** Its bytes derive from the
working-tree name and every earlier format wrote the same ones, so a charter is
evidence that *some* Grove was here and never evidence of *this* one.

The middle branch is `TT-24`'s fail-closed ownership rule applied at the **root**
grain rather than the entry grain, and it is the same split the contract already
draws one grain down: `WitnessPending` is an artifact at a reserved name Grove
**can** prove is its own, `ReservedNameOccupied` one it cannot classify at all.
Its refusal reason follows the same correspondence — `Reserved(class)` reports as
`WitnessPending(class)`, so `PartialScaffold(class)` reports as
`ScaffoldIncomplete(class)`, one parameterised member rather than two flat ones,
and the operator reads the class instead of guessing from the reason.

## The trade-off it settles

The competing repair is the one the Quint column proposed from its own
counterexample: define the state by the **presence** of the scaffold's own
entries and **ignore** entries outside the task grammar. It is simpler, it needs
no third branch, and it keeps the state table at eleven rows.

It is rejected because it establishes the wrong thing. The safety argument it
rests on — *every value a completion writes is fixed in advance, and a foreign
entry is not something completion writes* — proves the **bytes** a completion
writes are safe. It says nothing about whether **this root** is Grove's to write
into, and an entry Grove did not write is exactly the proof it lacks.
Completing under it is a mutation of a tree whose ownership the operation cannot
prove, which is the one thing `TT-24` forbids everywhere else. The counterexample
that motivated the proposal is real; only its second half is.

**The product had already answered, and answering the same way twice from
different evidence is what settled it.**
`recover_partial_root_init_unlocked` ([`src/tree_lifecycle.rs`](../../src/tree_lifecycle.rs))
runs exactly this test and refuses with an *ambiguous partial root scaffold*
diagnostic that the contract's state table had no member for. The charter
exclusion is not an inference from that code but a deliberate shipped test:
`an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` puts a byte-exact
charter beside a legacy-v2 leaf and **migrates**, because the alternative is
writing a format witness into somebody else's tree.

## What it costs, which is a window it does not close

`TT-20`'s prohibition on `Legacy` narrows to *once a root-init-exclusive entry
has landed*. Before that the root carries no evidence distinguishing it from a
legacy tree, so `Legacy` is honest — and the window is real and shipped: after
the charter, before the leaf, which `create_root_unlocked` and
`complete_scaffold` leave unguarded on purpose.

The cost is a **diagnostic** defect rather than a safety one, and it grows.
Nothing is silently completed in the window. But the approved breaking change
removes migration, after which `Legacy` fails closed and the operator is told to
migrate a tree that is not legacy, by a command that no longer exists, about a
directory Grove created and then failed to recognise.

## Alternatives rejected

- **Treat the charter as root-init-exclusive**, which closes the window
  entirely. Rejected on a fired control rather than on taste: it is what
  `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` exists to prevent,
  and its failure mode — completing somebody's legacy tree as a fresh current
  root — is strictly worse than the one it fixes. Reopen only if the charter
  gains bytes that distinguish the format that wrote it.
- **A guard across root initialisation's two phases.** Rejected because it buys
  nothing against the actor that produces the counterexample: `EN-06` grants
  only that *cooperating* processes are serialized, and the writer here is
  `EN-13`'s non-cooperating one. A guard would close a window against exactly
  the writers that were never in it. Reopen if the lock becomes mandatory.
- **A twelfth state beside `PartialScaffold` rather than a class of it.**
  Rejected because it asserts more than Grove knows in the wrong direction: the
  root *is* a partial scaffold — Grove can prove its own initialisation ran —
  and what it cannot prove is that the root's whole contents are its own. As a
  class, `TT-18` and `TT-20` stay stated over the scaffold **family** and are
  insensitive to a member being added or removed, which is the property
  `Reserved(class)` was given for the same reason.
- **Two flat refusal reasons instead of one parameterised member.** Rejected as
  unnecessary once the state carried the class. The operator distinction is real
  and must survive — telling an operator `ScaffoldIncomplete` about an ambiguous
  root would point them at a completion Grove has already declined — but
  `Reserved(class)` → `WitnessPending(class)` is the contract's own idiom for
  exactly that, and reusing it makes the vocabulary regular where a second member
  would make it lumpy. Reopen if a scaffold class ever needs a recovery the
  class parameter cannot name.
- **Reorder root initialisation so its first write is root-init-exclusive**,
  which closes the surviving window at the cost of one product change — and
  `tree_format::write_current_last` already validates and reuses a pre-existing
  temporary, so the code anticipates it. Not rejected on the merits: the formal
  phase alters no product behaviour, and the question is product-facing, so it
  sits with `handoff-audit-k66`'s other diagnostic questions. This record is
  where it comes back.

## What enforces it

`TT-17.b` states that the witnessless decision reads **bytes** and not only
names, so a name-only implementation fails a check rather than a review;
`TT-20`'s witnesses reach all three branches, including the surviving `Legacy`
window, so the narrowing is runnable rather than merely declared. Both families
answer both.

The prohibition's own conjunct is true by construction of each model's
classifier, which is the shape that hides a transcription, so it carries a
mutation: `crates/grove-task-tree/models/task-tree.qnt`'s
`SCAFFOLD_AMBIGUITY_CLASSED = false` restores the pre-decision definition and
`mutant_scaffold_absence_only` fails on it in 1.5 s. That control was itself
first written against an instance whose action menu never runs `root-init`, where
it reported no violation over 4000 traces — a control that cannot reach its
subject reads exactly like a control that found nothing, which is why the dial it
runs under is recorded beside it.
